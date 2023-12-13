use tracing::Span;
use user::{
    contacts::{self, ContactValue},
    UserProfile,
};

use super::{
    screens::{empty::EmptyScreen, exit::ExitScreen, Screen},
    *,
};
use crate::{
    middleware::ExcaliburMiddleware,
    screens::{
        dev::experimental::ExperimentalScreen,
        portfolio::{
            dashboard::portfolio_model::{AlloyAddress, AlloyU256, RawDataModel},
            PortfolioRoot,
        },
        settings::SettingsScreen,
        State,
    },
    user::networks::RPCValue,
    view::sidebar::Sidebar,
};

pub fn app_span() -> Span {
    tracing::debug_span!("App")
}

/// Root message for the Application.
#[derive(Debug, Default)]
pub enum Message {
    /// An empty message used as a default.
    #[default]
    Empty,
    /// Emitted on the initial load of the App.
    Load,
    /// All interface level messages are wrapped in this View message.
    View(view::Message),
    /// Modifications to the persistent user profile.
    UpdateUser(UserProfileMessage),
    /// Switches the active "app" to the target Route.
    SwitchWindow(view::sidebar::Route),
    /// Exits the application immediately, without saving. Save should be called
    /// before this event is triggered.
    Exit,
}

/// All messages for making modifications to the persistent user profile.
#[derive(Debug)]
pub enum UserProfileMessage {
    /// Stores a stringified snapshot of an Anvil instance.
    SaveAnvilSnapshot(anyhow::Result<String>),
    /// Adds an address to the contacts list.
    AddAddress(String, Address, contacts::Category),
    /// Removes an address from the contacts list.
    RemoveAddress(String, contacts::Category),
    /// warning! Deletes all addresses from a category in the list.
    ClearAddresses(contacts::Category),
    /// Adds an RPC to the RPC list.
    AddRPC(RPCValue),
    /// Removes an RPC from the RPC list.
    RemoveRPC(String),
    /// warning! Deletes all RPCs from the list.
    ClearRPCs,
}

pub type RootMessage = Message;
pub type RootViewMessage = view::Message;

impl MessageWrapper for Message {
    type ParentMessage = Message;
}

impl From<UserProfileMessage> for Message {
    fn from(message: UserProfileMessage) -> Self {
        Self::UpdateUser(message)
    }
}

/// State for all temporarily cached state. This is cleared on exiting the
/// application.
#[derive(Default)]
pub struct Cache {
    pub data: RawDataModel<AlloyAddress, AlloyU256>,
}

impl Cache {
    pub fn new() -> Self {
        Self {
            data: RawDataModel::default(),
        }
    }
}

/// State for specific windows that are open.
pub struct Windows {
    pub screen: Screen,
    pub sidebar: Sidebar,
}

impl Default for Windows {
    fn default() -> Self {
        Self {
            screen: ExperimentalScreen::new().into(),
            sidebar: Sidebar::new(),
        }
    }
}

impl Windows {
    pub fn new(screen: Screen, sidebar: Sidebar) -> Self {
        Self { screen, sidebar }
    }
}

/// Storage for the entire application.
/// This should hold the most important pieces of data that many children
/// components will need.
pub struct App {
    /// Connection to networks.
    pub client: Arc<ExcaliburMiddleware<Ws, LocalWallet>>,
    /// Transient state for the application.
    pub cache: Cache,
    /// Persistent state for the application stored as a user profile.
    pub user: UserProfile,
    /// State of the active window and sidebar the user is viewing.
    pub windows: Windows,
}

impl App {
    pub fn new(
        user: UserProfile,
        client: Arc<ExcaliburMiddleware<Ws, LocalWallet>>,
    ) -> (Self, Command<Message>) {
        let dashboard = PortfolioRoot::new(Some(client.clone()), user.clone()).into();
        (
            Self {
                client,
                user,
                cache: Cache::new(),
                windows: Windows::new(dashboard, Sidebar::new()),
            },
            Command::perform(async {}, |_| Message::Load),
        )
    }

    /// Loads the sidebar and the default screen. Called after new().
    pub fn load(&mut self) -> Command<Message> {
        let mut cmds = Vec::new();

        // Load the sidebar.
        cmds.push(self.windows.sidebar.load().map(|x| x.into()));

        // Load the current window.
        cmds.push(self.windows.screen.load().map(|x| x.into()));

        Command::batch(cmds)
    }

    /// All view updates are forwarded to the Screen's update function.
    pub fn update(&mut self, message: Message) -> Command<Message> {
        app_span().in_scope(|| match message {
            Message::Exit => iced::window::close(),
            Message::Load => self.load(),
            Message::UpdateUser(msg) => self.update_user(msg),
            Message::SwitchWindow(route) => self.switch_window(&route),
            Message::View(view::Message::Route(route)) => self.switch_window(&route),
            Message::View(view::Message::Exit) => self.exit(),
            Message::View(view::Message::CopyToClipboard(contents)) => {
                iced::clipboard::write(contents)
            }
            Message::Empty => Command::none(),
            // All the view messages are forwarded to the screen.
            _ => self.windows.screen.update(message),
        })
    }

    pub fn view(&self) -> Element<Message> {
        view::app_layout(&self.windows.sidebar, self.windows.screen.view()).map(Message::View)
    }

    pub fn subscription(&self) -> Subscription<Message> {
        self.windows.screen.subscription()
    }

    pub fn exit(&mut self) -> Command<Message> {
        // Save the profile to disk.
        let result = self.user.save();
        match result {
            Ok(_) => tracing::info!("Saved profile to disk"),
            Err(e) => tracing::error!("Failed to save profile to disk: {:?}", e),
        }

        // Call exit on the opened window.
        let mut commands = Vec::new();
        let cmd = self.windows.screen.exit();
        commands.push(cmd);

        // If the dev client is Some, call the anvil client using `anvil_dumpState`, and
        // set the profile's anvil snapshot to the result.
        if let Some(_) = self.client.anvil {
            let cmd = Command::perform(save_snapshot(self.client.clone()), |result| {
                UserProfileMessage::SaveAnvilSnapshot(result)
            })
            .map(|x| Message::UpdateUser(x));
            commands.push(cmd);
        }

        Command::batch(commands)
    }

    #[allow(unused_assignments)]
    fn update_user(&mut self, message: UserProfileMessage) -> Command<Message> {
        let profile = &mut self.user;

        let mut cmd = Command::none();
        match message {
            UserProfileMessage::SaveAnvilSnapshot(snapshot) => {
                tracing::debug!("Saving anvil snapshot to profile");
                match snapshot {
                    Ok(snapshot) => {
                        self.user.anvil_snapshot = Some(snapshot);
                        tracing::debug!("Saved anvil snapshot to profile");
                    }
                    Err(e) => tracing::error!("Failed to save anvil snapshot: {:?}", e),
                }

                // Exits the application after saving the anvil snapshot.
                return Command::perform(async {}, |_| Message::Exit);
            }
            UserProfileMessage::AddAddress(name, address, category) => {
                profile.contacts.add(
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
                profile.contacts.remove(&address, category);
            }
            UserProfileMessage::ClearAddresses(category) => {
                profile.contacts.clear(category);
            }
            UserProfileMessage::AddRPC(chain) => {
                profile.rpcs.add(chain);
            }
            UserProfileMessage::RemoveRPC(name) => {
                tracing::debug!("Removing RPC from storage: {}", name);
                profile.rpcs.remove(&name);
            }

            UserProfileMessage::ClearRPCs => {
                profile.rpcs.clear();
            }
        }

        let result = profile.save();
        match result {
            Ok(_) => tracing::info!("Saved profile to disk"),
            Err(e) => tracing::error!("Failed to save profile to disk: {:?}", e),
        }

        let rpcs = profile.rpcs.clone();
        cmd = Command::perform(async {}, move |_| {
            view::Message::Settings(settings::Message::Rpc(settings::rpc::Message::Sync(rpcs)))
        })
        .map(|x| x.into());
        cmd
    }

    #[allow(unreachable_patterns)]
    fn switch_window(&mut self, navigate_to: &view::sidebar::Route) -> Command<Message> {
        let mut cmds = Vec::new();

        let exit_cmd = self.windows.screen.exit();
        cmds.push(exit_cmd);

        self.windows.screen = match navigate_to {
            view::sidebar::Route::Page(page) => {
                // Updates the current page in the sidebar.
                cmds.push(
                    self.windows
                        .sidebar
                        .update(view::sidebar::Route::Page(*page))
                        .map(|x| x.into()),
                );

                match page {
                    view::sidebar::Page::Empty => EmptyScreen::new().into(),
                    view::sidebar::Page::Portfolio => {
                        PortfolioRoot::new(Some(self.client.clone()), self.user.clone()).into()
                    }
                    view::sidebar::Page::Settings => SettingsScreen::new(self.user.clone()).into(),
                    view::sidebar::Page::Exit => ExitScreen::new(true).into(),
                }
            }
            _ => EmptyScreen::new().into(),
        };

        let load_cmd = self.windows.screen.load();
        cmds.push(load_cmd);

        Command::batch(cmds)
    }
}

async fn save_snapshot(
    client: Arc<ExcaliburMiddleware<Ws, LocalWallet>>,
) -> anyhow::Result<String> {
    client.snapshot().await
}

#[cfg(test)]
mod tests {

    use criterion::{black_box, criterion_group, criterion_main, Criterion};

    use super::*;

    fn cache_update_bench(c: &mut Criterion) {
        // let storage = Storage::default();
        // let chains = Chains::default();
        // let mut app = App::new(storage, chains, Streams::default(), None).0;
        // c.bench_function("cache_update", |b| {
        // b.iter(|| {
        // app.cache_update(CacheMessage::AppEvent(AppEventLog::new(
        // "test".to_string(),
        // "test".to_string(),
        // )))
        // })
        // });
    }
}
