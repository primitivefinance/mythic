use std::{collections::VecDeque, sync::mpsc::Receiver};

use api::contacts::{self, ContactValue};
use arbiter_core::environment::Environment;
use clients::{arbiter::world::WorldManager, rpc::local::Local, scroll::Scroll};
use profile::Profile;
use tracer::AppEventLog;
use tracing::Span;

use super::{
    screens::{
        address_book::AddressBookScreen, developer::DeveloperScreen, empty::EmptyScreen,
        exit::ExitScreen, experimental::ExperimentalScreen, terminal::Terminal, Screen,
    },
    view::sidebar::Page,
    *,
};

pub fn app_span() -> Span {
    tracing::info_span!("App")
}

pub type SpawnResult = anyhow::Result<Arc<tokio::sync::Mutex<WorldManager>>, anyhow::Error>;

/// Emitted on simulation events.
#[derive(Debug)]
pub enum Simulation {
    Spawned(SpawnResult),
    Completed,
}

/// Emitted when data is involved.
#[derive(Debug)]
pub enum Data {
    ProcessTracer,
}

#[derive(Debug)]
pub enum Execution {
    Form(execution::form::FormMessage),
    Simulated(anyhow::Result<Scroll, anyhow::Error>),
    Executed(anyhow::Result<Scroll, anyhow::Error>),
    // Triggered after Execution::Executed is completed.
    Confirmed,
}

impl From<Execution> for WindowsMessage {
    fn from(msg: Execution) -> Self {
        WindowsMessage::Execution(msg)
    }
}

impl From<Execution> for Message {
    fn from(msg: Execution) -> Self {
        Message::WindowsMessage(msg.into()).into()
    }
}

/// Root message for the Application.
#[derive(Debug, Default)]
pub enum Message {
    #[default]
    Empty,
    View(view::Message),
    ChainsMessage(ChainMessage),
    StreamsMessage(StreamsMessage),
    CacheMessage(CacheMessage),
    StorageMessage(StorageMessage),
    WindowsMessage(WindowsMessage),
}

impl MessageWrapper for Message {
    type ParentMessage = Message;
}

#[derive(Debug)]
pub enum AddressBookMessage {
    Add(String, Address, contacts::Category),
    Remove(String, contacts::Category),
    Get(String, contacts::Category),
    List(contacts::Category),
    Clear(contacts::Category),
}

/// State for all chain related data.
#[derive(Clone, Debug)]
pub struct Chains {
    pub arbiter: Arc<Mutex<Environment>>,
    pub local: Local<Ws>,
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

/// State for all temporarily cached state.
#[derive(Default)]
pub struct Cache {
    pub app_events: VecDeque<AppEventLog>,
    pub current_page: Page,
}

impl Cache {
    pub fn new() -> Self {
        Self {
            app_events: VecDeque::new(),
            current_page: Page::Empty,
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
}

/// State for specific windows that are open.
pub struct Windows {
    pub screen: Screen,
}

impl Default for Windows {
    fn default() -> Self {
        Self {
            screen: EmptyScreen::new().into(),
        }
    }
}

#[derive(Debug)]
pub enum WindowsMessage {
    Switch(Page),
    Execution(Execution),
    Simulation(Simulation),
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
}

impl App {
    pub fn new(storage: Storage, chains: Chains, streams: Streams) -> (Self, Command<Message>) {
        (
            Self {
                storage,
                streams,
                chains,
                cache: Cache::new(),
                windows: Windows::default(),
            },
            Command::none(),
        )
    }

    // All view updates are forwarded to the Screen's update function.
    pub fn update(&mut self, message: Message) -> Command<Message> {
        let msg = app_span().in_scope(|| match message {
            Message::StorageMessage(msg) => self.storage_update(msg),
            Message::CacheMessage(msg) => self.cache_update(msg),
            Message::StreamsMessage(msg) => self.streams_update(msg),
            Message::ChainsMessage(msg) => self.chains_update(msg),
            Message::WindowsMessage(msg) => self.windows_update(msg),
            Message::View(view::Message::Page(page)) => self.switch_window(&page),
            Message::View(view::Message::Exit) => self.exit(),
            Message::View(view::Message::CopyToClipboard(contents)) => {
                iced::clipboard::write(contents)
            }
            Message::Empty => Command::none(),
            _ => self.windows.screen.update(message),
        });

        msg
    }

    pub fn view(&self) -> Element<Message> {
        view::app_layout(&self.cache.current_page, self.windows.screen.view()).map(Message::View)
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
        let cmd = self.windows.screen.exit();

        Command::batch(vec![cmd, iced::window::close()])
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
                cmd = Command::perform(async {}, |_| {
                    Message::View(view::Message::Data(view::Data::AppEvent))
                });
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

    #[allow(unused_assignments)]
    fn storage_update(&mut self, message: StorageMessage) -> Command<Message> {
        let mut cmd = Command::none();
        match message {
            StorageMessage::AddressBook(msg) => {
                cmd = self.contacts_update(msg);
            }
        }
        cmd
    }

    fn chains_update(&mut self, _message: ChainMessage) -> Command<Message> {
        let cmd = Command::none();
        // todo: implement
        cmd
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
    fn switch_window(&mut self, navigate_to: &Page) -> Command<Message> {
        // Update the current page.
        self.cache.current_page = navigate_to.clone();

        let mut cmds = Vec::new();
        let exit_cmd = self.windows.screen.exit();
        cmds.push(exit_cmd);

        self.windows.screen = match navigate_to {
            view::sidebar::Page::Execute => Screen::new(Box::new(execution::Execution::new(
                self.chains.clone(),
                self.storage.clone(),
            ))),
            view::sidebar::Page::AddressBook => Screen::new(Box::new(AddressBookScreen::new(
                self.storage.profile.contacts.clone(),
            ))),
            view::sidebar::Page::Empty => EmptyScreen::new().into(),
            view::sidebar::Page::Exit => ExitScreen::new(true).into(),
            view::sidebar::Page::Terminal => Screen::new(Box::new(Terminal::new(
                self.streams.app_event_receiver.clone(),
            ))),
            view::sidebar::Page::Experimental => Screen::new(Box::new(ExperimentalScreen::new())),
            view::sidebar::Page::Developer => DeveloperScreen::new().into(),
            _ => EmptyScreen::new().into(),
        };

        let load_cmd = self.windows.screen.load();
        cmds.push(load_cmd);

        Command::batch(cmds)
    }
}

#[cfg(test)]
mod tests {

    use criterion::{black_box, criterion_group, criterion_main, Criterion};

    use super::*;

    fn cache_update_bench(c: &mut Criterion) {
        // let mut app = App::new();
        c.bench_function("cache_update", |b| {
            b.iter(|| {
                // app.cache_update(CacheMessage::AppEvent(AppEventLog::new(
                // "test".to_string(),
                // "test".to_string(),
                // )))
            })
        });
    }
}
