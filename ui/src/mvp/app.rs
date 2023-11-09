use arbiter_core::environment::{builder::EnvironmentBuilder, Environment};
use iced::{
    alignment,
    widget::{button, column, container, text},
    Length,
};

use super::{
    state::{Screen, Terminal},
    *,
};

#[derive(Debug)]
pub enum Message {
    Empty,
    View(view::Message),
}

/// Storage for the entire application.
/// This should hold the most important pieces of data that many children components will need.
pub struct App {
    arbiter: Environment,
    local: Local<Ws>,
    screen: Screen,
}

impl App {
    pub fn new(arbiter: Environment, local: Local<Ws>) -> (Self, Command<Message>) {
        let screen = Screen::new(Box::new(Terminal::new()));
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
        Command::none()
    }

    pub fn view(&self) -> Element<Message> {
        self.screen.view().map(Message::View)
    }
}
