use std::sync::mpsc::Receiver;

use arbiter_core::environment::Environment;
use tracing::Span;

use super::{
    screens::{terminal::Terminal, Screen},
    tracer::AppEventLog,
    view::Page,
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

/// Root message for the Application.
#[derive(Debug)]
pub enum Message {
    Empty,
    View(view::Message),
    Simulation(Simulation),
    Data(Data),
}

/// Storage for the entire application.
/// This should hold the most important pieces of data that many children
/// components will need.
pub struct App {
    #[allow(dead_code)]
    arbiter: Environment,
    #[allow(dead_code)]
    local: Local<Ws>,
    screen: Screen,
    receiver: Arc<Mutex<Receiver<AppEventLog>>>,
}

impl App {
    pub fn new(
        arbiter: Environment,
        local: Local<Ws>,
        receiver: Arc<Mutex<Receiver<AppEventLog>>>,
    ) -> (Self, Command<Message>) {
        let screen = Screen::new(Box::new(Terminal::new(receiver.clone())));
        (
            Self {
                arbiter,
                local,
                screen,
                receiver,
            },
            Command::none(),
        )
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        let msg = app_span().in_scope(|| match message {
            Message::View(view::Message::Page(navigate_to)) => self.switch_page(&navigate_to),
            _ => self.screen.update(message),
        });

        msg
    }

    pub fn view(&self) -> Element<Message> {
        self.screen.view().map(Message::View)
    }

    pub fn subscription(&self) -> Subscription<Message> {
        self.screen.subscription()
    }

    fn switch_page(&mut self, navigate_to: &Page) -> Command<Message> {
        self.screen = match navigate_to {
            view::Page::Execute => Screen::new(Box::new(execution::Execution::new())),
            _ => Screen::new(Box::new(Terminal::new(self.receiver.clone()))),
        };

        self.screen.load()
    }
}
