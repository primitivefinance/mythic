use ethers::prelude::*;
use iced::{Element, Subscription, Task};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use tracing::Span;

use crate::blockchain::{self, AlloyClient};
use crate::data::{
    contacts::{self, ContactValue},
    rpcs::RPCValue,
    user::Saveable,
    Model,
};
use crate::pages::{
    dashboard::Dashboard,
    empty::EmptyPage,
    exit::ExitPage,
    settings::{self, SettingsPage},
    Lifecycle, MessageWrapper, Page,
};
use crate::view::{self, connect::Connect, header::Header, navigation::NavEvent};

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

    State(StateEvent),
}

#[derive(Debug, Clone, Default)]
pub enum StateEvent {
    #[default]
    Empty,
    Update(String, Value),
}

impl From<StateEvent> for AppMessage {
    fn from(event: StateEvent) -> Self {
        AppMessage::State(event)
    }
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
    pub connect: Connect,
}

impl Default for Windows {
    fn default() -> Self {
        Self {
            screen: EmptyPage::new().into(),
            header: Header::new(),
            connect: Connect::default(),
        }
    }
}

impl Windows {
    pub fn new(screen: Page, header: Header, connect: Connect) -> Self {
        Self {
            screen,
            header,
            connect,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct AppState {
    data: HashMap<String, Value>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn insert<T: Serialize>(&mut self, key: String, value: T) {
        self.data.insert(key, serde_json::to_value(value).unwrap());
    }

    pub fn get<T: DeserializeOwned>(&self, key: &str) -> Option<T> {
        self.data
            .get(key)
            .and_then(|v| serde_json::from_value(v.clone()).ok())
    }

    pub fn subscription(client: Arc<AlloyClient>) -> Subscription<AppMessage> {
        let client = client.clone();
        Subscription::run_with_id("block_number", blockchain::listen_for_blocks(client))
    }
}

pub type SharedState = Arc<RwLock<AppState>>;

pub struct App {
    pub client: Arc<AlloyClient>,
    pub model: Model,
    pub windows: Windows,

    pub state: SharedState,
}

impl App {
    pub fn new(model: Model, client: Arc<AlloyClient>) -> (Self, Task<AppMessage>) {
        let state = Arc::new(RwLock::new(AppState::new()));
        let dashboard = Dashboard::new(client.clone(), state.clone()).into();
        let mut header = Header::new();
        header.current_nav = view::navigation::Navigation::Dashboard;
        let connect = Connect::new(client.clone());
        (
            Self {
                client,
                model,
                windows: Windows::new(dashboard, header, connect),
                state,
            },
            Task::perform(async {}, |_| AppMessage::Load),
        )
    }

    pub fn load(&mut self) -> Task<AppMessage> {
        let cmds = vec![
            self.windows.header.load().map(|x| x.into()),
            self.windows.screen.load().map(|x| x),
        ];
        Task::batch(cmds)
    }

    pub fn update(&mut self, message: AppMessage) -> Task<AppMessage> {
        app_span().in_scope(|| match message {
            AppMessage::Load => self.load(),
            AppMessage::QuitReady => Task::none(),
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
                Task::none()
            }
            AppMessage::UpdateUser(msg) => self.update_user(msg),
            AppMessage::View(view::ViewMessage::Root(msg)) => match msg {
                view::RootMessage::ModelSyncRequest => self.sync_model(),
                view::RootMessage::Route(route) => self.switch_window(&route),
                view::RootMessage::CopyToClipboard(contents) => iced::clipboard::write(contents),
                view::RootMessage::SaveAndExit => self.exit(),
                view::RootMessage::Empty => Task::none(),
                view::RootMessage::ConfirmExit => {
                    tracing::debug!("Confirming exit");
                    self.windows
                        .screen
                        .update(AppMessage::View(view::ViewMessage::Root(msg)))
                        .map(|x| x)
                }
                view::RootMessage::Connect(msg) => match msg {
                    view::connect::Message::UpdateClient(ref client) => {
                        let client = client.clone();
                        tracing::debug!("Updated client: {:?}", client.provider.is_some());

                        self.client = Arc::new(client);

                        // Client is overwritten so the original Arc<..> is stale.
                        // todo: fix connection switching to prevent stale issue or make this propagation better.
                        Task::batch(vec![
                            self.windows.screen.update(
                                view::ViewMessage::Dashboard(
                                    crate::pages::dashboard::Message::UpdateClient(
                                        self.client.clone(),
                                    ),
                                )
                                .into(),
                            ),
                            self.windows.connect.update(msg),
                        ])
                    }
                    _ => self.windows.connect.update(msg),
                },
            },
            AppMessage::State(event) => {
                match event {
                    StateEvent::Update(key, value) => {
                        if let Ok(mut state) = self.state.write() {
                            tracing::debug!("Successfully updated state: {} = {:?}", key, value);
                            state.insert(key, value);
                        } else {
                            tracing::error!("Failed to update state");
                        }
                    }
                    _ => {}
                }
                Task::none()
            }
            _ => self.windows.screen.update(message),
        })
    }

    pub fn view(&self) -> Element<AppMessage> {
        view::document::body(
            &self.windows.connect,
            &self.windows.header,
            self.windows.screen.view(),
        )
        .map(AppMessage::View)
    }

    pub fn subscription(&self) -> Subscription<AppMessage> {
        let mut subscriptions = vec![self.windows.screen.subscription()];

        if self.client.provider.is_some() {
            subscriptions.push(AppState::subscription(self.client.clone()));
        }

        Subscription::batch(subscriptions)
    }

    pub fn exit(&mut self) -> Task<AppMessage> {
        let result = self.model.save();
        match result {
            Ok(_) => tracing::info!("Saved profile to disk"),
            Err(e) => tracing::error!("Failed to save profile to disk: {:?}", e),
        }

        let mut tasks = Vec::new();
        let cmd = self.windows.screen.exit();
        tasks.push(cmd);

        if self.client.provider.is_some() {
            let cmd = Task::perform(
                save_snapshot(self.client.clone()),
                UserProfileMessage::SaveAnvilSnapshot,
            )
            .map(AppMessage::UpdateUser);
            tasks.push(cmd);
        }

        Task::batch(tasks)
    }

    fn sync_model(&mut self) -> Task<AppMessage> {
        let model = self.model.clone();
        //let provider = self.client.get_client();
        Task::perform(
            async move {
                let mut model = model;
                //model.update(provider).await?;
                Ok(model)
            },
            AppMessage::ModelSyncResult,
        )
    }

    fn update_user(&mut self, message: UserProfileMessage) -> Task<AppMessage> {
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

                return Task::perform(async {}, |_| AppMessage::QuitReady);
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
        Task::perform(async {}, move |_| {
            view::ViewMessage::Settings(settings::Message::Rpc(settings::rpc::Message::Sync(
                rpcs.clone(),
            )))
        })
        .map(|x| x.into())
    }

    #[allow(unreachable_patterns)]
    fn switch_window(&mut self, navigate_to: &view::navigation::NavEvent) -> Task<AppMessage> {
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
                        Dashboard::new(self.client.clone(), self.state.clone()).into()
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

        Task::batch(cmds)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnvilSave {
    pub snapshot: String,
    pub block_number: u64,
}

#[tracing::instrument(skip(client))]
async fn save_snapshot(client: Arc<AlloyClient>) -> anyhow::Result<AnvilSave> {
    tracing::debug!("Attempting to save anvil snapshot");
    client.snapshot().await
}
