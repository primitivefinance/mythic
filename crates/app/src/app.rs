use std::{collections::VecDeque, sync::mpsc::Receiver};

use clients::{client::AnvilClient, dev::DevClient, ledger::LedgerClient};
use ethers::core::k256::ecdsa::SigningKey;
use profile::Profile;
use tracer::AppEventLog;
use tracing::Span;
use user::contacts::{self, ContactValue};

use super::{
    screens::{empty::EmptyScreen, exit::ExitScreen, Screen},
    *,
};
use crate::{
    loader::DefaultMiddleware,
    screens::{
        dev::experimental::ExperimentalScreen, portfolio::PortfolioRoot, settings::SettingsScreen,
        State,
    },
    user::networks::ChainPacket,
    view::sidebar::Sidebar,
};

pub fn app_span() -> Span {
    tracing::debug_span!("App")
}

/// Root message for the Application.
#[derive(Debug, Default)]
pub enum Message {
    #[default]
    Empty,
    Load,
    View(view::Message),
    ChainsMessage(ChainMessage),
    StreamsMessage(StreamsMessage),
    CacheMessage(CacheMessage),
    StorageMessage(StorageMessage),
    WindowsMessage(WindowsMessage),
    Exit,
}

pub type RootMessage = Message;
pub type RootViewMessage = view::Message;

impl MessageWrapper for Message {
    type ParentMessage = Message;
}

/// State for all chain related data.
/// Idea: maybe make chains over generic over signers?
#[derive(Debug, Clone)]
pub struct Chains {
    pub sub_clients: Vec<Provider<Ws>>,
    pub call_clients: Vec<Provider<Http>>,
    pub sign_clients: Vec<Wallet<SigningKey>>,
    pub anvil_client: AnvilClient,
}

impl Chains {
    #[tracing::instrument(skip(self))]
    pub fn get_signer(
        &self,
        sub_client: usize,
        sign_client: usize,
    ) -> anyhow::Result<Arc<SignerMiddleware<Provider<Ws>, Wallet<SigningKey>>>> {
        let sub_client = self
            .sub_clients
            .get(sub_client)
            .ok_or(anyhow::anyhow!("Sub client not found"))?;
        let sign_client = self
            .sign_clients
            .get(sign_client)
            .ok_or(anyhow::anyhow!("Sign client not found"))?;

        tracing::debug!("Creating signer middleware");
        let signer = SignerMiddleware::new(sub_client.clone(), sign_client.clone());
        Ok(Arc::new(signer))
    }
}

#[derive(Debug)]
pub enum ChainMessage {}

/// State for all channel senders and receivers.
pub struct Streams {
    pub app_event_receiver: Arc<Mutex<Receiver<AppEventLog>>>,
}

#[derive(Debug)]
pub enum StreamsMessage {
    Data(Data),
}

/// Emitted when data is involved.
#[derive(Debug)]
pub enum Data {
    ProcessTracer,
}

/// State for all temporarily cached state.
#[derive(Default)]
pub struct Cache {
    pub app_events: VecDeque<AppEventLog>,
}

impl Cache {
    pub fn new() -> Self {
        Self {
            app_events: VecDeque::new(),
        }
    }
}

#[derive(Debug)]
pub enum CacheMessage {
    AppEvent(AppEventLog),
}

/// State for all permanent state that is loaded from disk or api.
#[derive(Debug, Clone, Default)]
pub struct Storage {
    pub profile: Profile,
}

#[derive(Debug)]
pub enum StorageMessage {
    AddressBook(AddressBookMessage),
    RPCStorage(RPCStorageMessage),
    AnvilSnapshot(anyhow::Result<String>),
}

#[derive(Debug)]
pub enum AddressBookMessage {
    Add(String, Address, contacts::Category),
    Remove(String, contacts::Category),
    Get(String, contacts::Category),
    List(contacts::Category),
    Clear(contacts::Category),
}

#[derive(Debug)]
pub enum RPCStorageMessage {
    Add(ChainPacket),
    Remove(String),
    Get(String),
    List,
    Clear,
}

impl From<RPCStorageMessage> for StorageMessage {
    fn from(msg: RPCStorageMessage) -> Self {
        Self::RPCStorage(msg)
    }
}

impl From<StorageMessage> for Message {
    fn from(msg: StorageMessage) -> Self {
        Self::StorageMessage(msg)
    }
}

impl From<RPCStorageMessage> for Message {
    fn from(msg: RPCStorageMessage) -> Self {
        Self::StorageMessage(msg.into())
    }
}

/// State for specific windows that are open.
pub struct Windows {
    pub screen: Screen,
}

impl Default for Windows {
    fn default() -> Self {
        Self {
            screen: ExperimentalScreen::new().into(),
        }
    }
}

#[derive(Debug)]
pub enum WindowsMessage {
    Switch(view::sidebar::Route),
}

impl From<WindowsMessage> for Message {
    fn from(msg: WindowsMessage) -> Self {
        Message::WindowsMessage(msg)
    }
}

/// Storage for the entire application.
/// This should hold the most important pieces of data that many children
/// components will need.
pub struct App {
    pub cache: Cache,
    pub storage: Storage,
    pub streams: Streams,
    pub windows: Windows,
    pub chains: Chains,
    pub sidebar: Sidebar,
    // this is a handle that has a lock on the ledger device
    // we have to talk to it async
    pub ledger: Option<LedgerClient>,
    // dev client for testing
    pub dev_client: Option<DevClient<DefaultMiddleware>>,
}

impl App {
    pub fn new(
        storage: Storage,
        chains: Chains,
        streams: Streams,
        ledger: Option<LedgerClient>,
        dev_client: Option<DevClient<DefaultMiddleware>>,
    ) -> (Self, Command<Message>) {
        (
            Self {
                storage,
                streams,
                chains,
                cache: Cache::new(),
                windows: Windows::default(),
                sidebar: Sidebar::new(),
                ledger,
                dev_client,
            },
            Command::perform(async {}, |_| Message::Load),
        )
    }

    // Loads the sidebar and the default screen.
    pub fn load(&mut self) -> Command<Message> {
        let mut cmds = Vec::new();

        // Load the sidebar.
        cmds.push(self.sidebar.load().map(|x| x.into()));

        // Load the current window.
        cmds.push(self.windows.screen.load().map(|x| x.into()));

        Command::batch(cmds)
    }

    // All view updates are forwarded to the Screen's update function.
    pub fn update(&mut self, message: Message) -> Command<Message> {
        app_span().in_scope(|| match message {
            Message::Exit => iced::window::close(),
            Message::Load => self.load(),
            Message::StorageMessage(msg) => self.storage_update(msg),
            Message::CacheMessage(msg) => self.cache_update(msg),
            Message::StreamsMessage(msg) => self.streams_update(msg),
            Message::ChainsMessage(msg) => self.chains_update(msg),
            Message::WindowsMessage(msg) => self.windows_update(msg),
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
        view::app_layout(&self.sidebar, self.windows.screen.view()).map(Message::View)
    }

    pub fn subscription(&self) -> Subscription<Message> {
        self.windows.screen.subscription()
    }

    pub fn exit(&mut self) -> Command<Message> {
        // Save the profile to disk.
        let result = self.storage.profile.save();
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
        let anvil_client = self.chains.anvil_client.clone();
        match self.dev_client.clone() {
            Some(dev_client) => {
                let cmd = Command::perform(save_snapshot(anvil_client), |snapshot| {
                    Message::StorageMessage(StorageMessage::AnvilSnapshot(snapshot))
                });
                commands.push(cmd);
            }
            None => {}
        };

        Command::batch(commands)
    }

    fn streams_update(&mut self, message: StreamsMessage) -> Command<Message> {
        let cmd = Command::none();
        match message {
            StreamsMessage::Data(Data::ProcessTracer) => {
                let cloned = self.streams.app_event_receiver.clone();
                let locked = cloned.lock().unwrap();

                let mut logs = Vec::new();

                // todo: does this work? could it block if nothing?
                while let Ok(log) = locked.try_recv() {
                    logs.push(log);
                }

                // Warning! Cannot use any tracing in the following code branch.
                if let Some(log) = logs.last() {
                    return self.cache_update(CacheMessage::AppEvent(log.clone()));
                }
            }
        }

        cmd
    }

    #[allow(unused_assignments)]
    fn cache_update(&mut self, message: CacheMessage) -> Command<Message> {
        let mut cmd = Command::none();
        match message {
            // Cannot use tracing here.
            CacheMessage::AppEvent(log) => {
                // Define the maximum number of logs
                const MAX_LOGS: usize = 100;

                // Push the new log
                self.cache.app_events.push_back(log);

                // If the number of data_feed exceeds the maximum, remove the oldest one
                if self.cache.app_events.len() > MAX_LOGS {
                    self.cache.app_events.pop_front();
                }

                // todo: figure out how to best pipe updated app state to windows...
                // todo: update, old.
                cmd = Command::perform(async {}, |_| Message::View(view::Message::Empty));
            }
        }
        cmd
    }

    fn contacts_update(&mut self, message: AddressBookMessage) -> Command<Message> {
        let cmd = Command::none();
        let contacts = &mut self.storage.profile.contacts;
        match message {
            // todo: update these messages
            AddressBookMessage::Add(name, address, category) => {
                contacts.add(
                    address,
                    ContactValue {
                        label: name,
                        ..Default::default()
                    },
                    category,
                );
            }
            AddressBookMessage::Remove(name, category) => {
                let address = name.parse::<Address>().unwrap();
                contacts.remove(&address, category);
            }
            AddressBookMessage::Get(name, category) => {
                let address = name.parse::<Address>().unwrap();
                contacts.get(&address, category);
            }
            AddressBookMessage::List(category) => {
                contacts.list(category);
            }
            AddressBookMessage::Clear(category) => {
                contacts.clear(category);
            }
        }
        cmd
    }

    fn rpcs_update(&mut self, message: RPCStorageMessage) -> Command<Message> {
        let profile = &mut self.storage.profile;
        match message {
            RPCStorageMessage::Add(chain) => {
                profile.rpcs.add(chain);
            }
            RPCStorageMessage::Remove(name) => {
                tracing::debug!("Removing RPC from storage: {}", name);
                profile.rpcs.remove(&name);
            }
            RPCStorageMessage::Get(name) => {
                profile.rpcs.get(&name);
            }
            RPCStorageMessage::List => {
                profile.rpcs.list();
            }
            RPCStorageMessage::Clear => {
                profile.rpcs.clear();
            }
        }

        // Make sure to save the new storage.
        let result = profile.save();
        match result {
            Ok(_) => tracing::info!("Saved profile to disk"),
            Err(e) => tracing::error!("Failed to save profile to disk: {:?}", e),
        }

        // todo: this can probably be removed.
        // we can just update the storage in the rpc settings manually.
        let rpcs = profile.rpcs.clone();
        tracing::info!("Syncing RPCs from app: {:?}", rpcs);
        Command::perform(async {}, move |_| {
            view::Message::Settings(settings::Message::Rpc(settings::rpc::Message::Sync(rpcs)))
        })
        .map(|x| x.into())
    }

    #[allow(unused_assignments)]
    fn storage_update(&mut self, message: StorageMessage) -> Command<Message> {
        let mut cmd = Command::none();
        match message {
            StorageMessage::AddressBook(msg) => {
                cmd = self.contacts_update(msg);
            }
            StorageMessage::RPCStorage(msg) => {
                cmd = self.rpcs_update(msg);
            }
            StorageMessage::AnvilSnapshot(snapshot) => {
                tracing::debug!("Saving anvil snapshot to profile");
                match snapshot {
                    Ok(snapshot) => {
                        self.storage.profile.anvil_snapshot = Some(snapshot);
                        tracing::debug!("Saved anvil snapshot to profile");
                    }
                    Err(e) => tracing::error!("Failed to save anvil snapshot: {:?}", e),
                }

                // Exits the application after saving the anvil snapshot.
                return Command::perform(async {}, |_| Message::Exit);
            }
        }
        cmd
    }

    fn chains_update(&mut self, _message: ChainMessage) -> Command<Message> {
        Command::none()
    }

    // Forwards window messages to the screen.
    #[allow(unused_assignments)]
    fn windows_update(&mut self, message: WindowsMessage) -> Command<Message> {
        let mut cmd = Command::none();
        match message {
            WindowsMessage::Switch(route) => {
                cmd = self.switch_window(&route);
            }
            _ => cmd = self.windows.screen.update(Message::WindowsMessage(message)),
        }
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
                    self.sidebar
                        .update(view::sidebar::Route::Page(*page))
                        .map(|x| x.into()),
                );

                match page {
                    view::sidebar::Page::Empty => EmptyScreen::new().into(),
                    view::sidebar::Page::Portfolio => {
                        PortfolioRoot::new(self.dev_client.clone()).into()
                    }
                    view::sidebar::Page::Settings => {
                        SettingsScreen::new(self.storage.clone()).into()
                    }
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

async fn save_snapshot(anvil: AnvilClient) -> anyhow::Result<String> {
    Ok(anvil.snapshot().await)
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
