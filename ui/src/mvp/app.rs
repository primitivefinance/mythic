use std::sync::mpsc::Receiver;

use arbiter_core::environment::Environment;

use super::{
    state::{Screen, Terminal},
    *,
};

#[derive(Debug)]
pub enum Message {
    Empty,
    View(view::Message),
    ProcessTracer,
}

/// Storage for the entire application.
/// This should hold the most important pieces of data that many children
/// components will need.
pub struct App {
    arbiter: Environment,
    local: Local<Ws>,
    screen: Screen,
}

impl App {
    pub fn new(
        arbiter: Environment,
        local: Local<Ws>,
        receiver: Arc<Mutex<Receiver<String>>>,
    ) -> (Self, Command<Message>) {
        let screen = Screen::new(Box::new(Terminal::new(receiver.clone())));
        (
            Self {
                arbiter,
                local,
                screen,
            },
            Command::none(),
        )
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            _ => self.screen.update(message),
        }
    }

    pub fn view(&self) -> Element<Message> {
        self.screen.view().map(Message::View)
    }

    pub fn subscription(&self) -> Subscription<Message> {
        self.screen.subscription()
    }
}
