use std::time::Duration;

use serde::{Deserialize, Serialize};
use tracing::Span;

use super::{
    pages::{empty::EmptyPage, exit::ExitPage, Page},
    *,
};
use crate::{
    components::system::{label, ExcaliburColor},
    data::{
        contacts::{self, ContactValue},
        rpcs::RPCValue,
        user::Saveable,
    },
    middleware::ExcaliburMiddleware,
    pages::{portfolio::PortfolioRootPage, settings::SettingsPage},
    view::header::Header,
    view::navigation::NavEvent,
};

pub fn app_span() -> Span {
    tracing::debug_span!("App")
}

#[derive(Debug, Default)]
pub enum AppMessage {
    #[default]
    Empty,
    Load,
    QuitReady,
    View(view::ViewMessage),
    UpdateUser(UserProfileMessage),
    SwitchWindow(NavEvent),
    ModelSyncResult(Result<Model, Arc<anyhow::Error>>),
}

#[derive(Debug)]
pub enum UserProfileMessage {
    SaveAnvilSnapshot(anyhow::Result<AnvilSave>),
    AddAddress(String, Address, contacts::Category),
    RemoveAddress(String, contacts::Category),
    ClearAddresses(contacts::Category),
    AddRPC(RPCValue),
    RemoveRPC(String),
    ClearRPCs,
}

pub type RootMessage = AppMessage;
pub type RootViewMessage = view::ViewMessage;

impl MessageWrapper for AppMessage {
    type ParentMessage = AppMessage;
}

impl From<UserProfileMessage> for AppMessage {
    fn from(message: UserProfileMessage) -> Self {
        Self::UpdateUser(message)
    }
}

pub struct Windows {
    pub screen: Page,
    pub header: Header,
}

impl Default for Windows {
    fn default() -> Self {
        Self {
            screen: EmptyPage::new().into(),
            header: Header::new(),
        }
    }
}

impl Windows {
    pub fn new(screen: Page, header: Header) -> Self {
        Self { screen, header }
    }
}

pub struct App {
    pub client: Arc<ExcaliburMiddleware<Ws, LocalWallet>>,
    pub model: Model,
    pub windows: Windows,
}

impl App {
    pub fn new(
        model: Model,
        client: Arc<ExcaliburMiddleware<Ws, LocalWallet>>,
    ) -> (Self, Command<AppMessage>) {
        let dashboard = PortfolioRootPage::new(Some(client.clone()), model.clone()).into();
        let mut header = Header::new();
        header.current_nav = view::navigation::Navigation::Dashboard;
        (
            Self {
                client,
                model,
                windows: Windows::new(dashboard, header),
            },
            Command::perform(async {}, |_| AppMessage::Load),
        )
    }

    pub fn load(&mut self) -> Command<AppMessage> {
        let cmds = vec![
            self.windows.header.load().map(|x| x.into()),
            self.windows.screen.load().map(|x| x),
        ];
        Command::batch(cmds)
    }

    pub fn update(&mut self, message: AppMessage) -> Command<AppMessage> {
        app_span().in_scope(|| match message {
            AppMessage::Load => self.load(),
            AppMessage::QuitReady => Command::none(),
            AppMessage::ModelSyncResult(Ok(model)) => {
                self.model = model.clone();

                self.windows
                    .screen
                    .update(AppMessage::ModelSyncResult(Ok(model)))
            }
            AppMessage::ModelSyncResult(Err(e)) => {
                tracing::error!(
                    "Critical failure - sync model threw an error while updating: {:?}",
                    e
                );
                Command::none()
            }
            AppMessage::UpdateUser(msg) => self.update_user(msg),
            AppMessage::View(view::ViewMessage::Root(msg)) => match msg {
                view::RootMessage::ModelSyncRequest => self.sync_model(),
                view::RootMessage::Route(route) => self.switch_window(&route),
                view::RootMessage::CopyToClipboard(contents) => iced::clipboard::write(contents),
                view::RootMessage::SaveAndExit => self.exit(),
                view::RootMessage::Empty => Command::none(),
                view::RootMessage::ConfirmExit => {
                    tracing::debug!("Confirming exit");
                    self.windows
                        .screen
                        .update(AppMessage::View(view::ViewMessage::Root(msg)))
                        .map(|x| x)
                }
            },

            _ => self.windows.screen.update(message),
        })
    }

    pub fn view(&self) -> Element<AppMessage> {
        view::document::body(&self.windows.header, self.windows.screen.view()).map(AppMessage::View)
    }

    pub fn subscription(&self) -> Subscription<AppMessage> {
        self.windows.screen.subscription()
    }

    pub fn exit(&mut self) -> Command<AppMessage> {
        let result = self.model.save();
        match result {
            Ok(_) => tracing::info!("Saved profile to disk"),
            Err(e) => tracing::error!("Failed to save profile to disk: {:?}", e),
        }

        let mut commands = Vec::new();
        let cmd = self.windows.screen.exit();
        commands.push(cmd);

        if self.client.anvil.is_some() {
            let cmd = Command::perform(
                save_snapshot(self.client.clone()),
                UserProfileMessage::SaveAnvilSnapshot,
            )
            .map(AppMessage::UpdateUser);
            commands.push(cmd);
        }

        Command::batch(commands)
    }

    fn sync_model(&mut self) -> Command<AppMessage> {
        let model = self.model.clone();
        let provider = self.client.get_client();
        Command::perform(
            async move {
                let mut model = model;
                model.update(provider).await?;
                Ok(model)
            },
            AppMessage::ModelSyncResult,
        )
    }

    fn update_user(&mut self, message: UserProfileMessage) -> Command<AppMessage> {
        let model = &mut self.model;
        match message {
            UserProfileMessage::SaveAnvilSnapshot(snapshot) => {
                tracing::debug!("Saving anvil snapshot to profile");
                match snapshot {
                    Ok(snapshot) => {
                        self.model.user.anvil_snapshot = Some(snapshot);
                        if let Err(e) = self.model.save() {
                            tracing::error!("Failed to save anvil snapshot to file: {:?}", e);
                        } else {
                            tracing::debug!("Saved anvil snapshot to file");
                        }
                    }
                    Err(e) => tracing::error!("Failed to save anvil snapshot: {:?}", e),
                }

                return Command::perform(async {}, |_| AppMessage::QuitReady);
            }
            UserProfileMessage::AddAddress(name, address, category) => {
                model.user.contacts.add(
                    address,
                    ContactValue {
                        label: name,
                        ..Default::default()
                    },
                    category,
                );
            }
            UserProfileMessage::RemoveAddress(name, category) => {
                let address = name.parse::<Address>().unwrap();
                model.user.contacts.remove(&address, category);
            }
            UserProfileMessage::ClearAddresses(category) => {
                model.user.contacts.clear(category);
            }
            UserProfileMessage::AddRPC(chain) => {
                model.user.rpcs.add(chain);
            }
            UserProfileMessage::RemoveRPC(name) => {
                tracing::debug!("Removing RPC from storage: {}", name);
                model.user.rpcs.remove(&name);
            }

            UserProfileMessage::ClearRPCs => {
                model.user.rpcs.clear();
            }
        }

        let result = model.save();
        match result {
            Ok(_) => tracing::info!("Saved profile to disk"),
            Err(e) => tracing::error!("Failed to save profile to disk: {:?}", e),
        }

        let rpcs = model.user.rpcs.clone();
        Command::perform(async {}, move |_| {
            view::ViewMessage::Settings(settings::Message::Rpc(settings::rpc::Message::Sync(rpcs)))
        })
        .map(|x| x.into())
    }

    #[allow(unreachable_patterns)]
    fn switch_window(&mut self, navigate_to: &view::navigation::NavEvent) -> Command<AppMessage> {
        let mut cmds = Vec::new();

        let exit_cmd = self.windows.screen.exit();
        cmds.push(exit_cmd);

        self.windows.screen = match navigate_to {
            view::navigation::NavEvent::Navigate(page) => {
                cmds.push(
                    self.windows
                        .header
                        .update(view::navigation::NavEvent::Navigate(*page))
                        .map(|x| x.into()),
                );

                match page {
                    view::navigation::Navigation::Empty => EmptyPage::new().into(),
                    view::navigation::Navigation::Dashboard => {
                        PortfolioRootPage::new(Some(self.client.clone()), self.model.clone()).into()
                    }
                    view::navigation::Navigation::Settings => {
                        SettingsPage::new(self.model.user.clone()).into()
                    }
                    view::navigation::Navigation::Exit => ExitPage::new(true).into(),
                }
            }
            _ => EmptyPage::new().into(),
        };

        let load_cmd = self.windows.screen.load();
        cmds.push(load_cmd);

        Command::batch(cmds)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnvilSave {
    pub snapshot: String,
    pub block_number: u64,
}

#[tracing::instrument(skip(client))]
async fn save_snapshot(
    client: Arc<ExcaliburMiddleware<Ws, LocalWallet>>,
) -> anyhow::Result<AnvilSave> {
    tracing::debug!("Attempting to save anvil snapshot");
    client.snapshot().await
}
