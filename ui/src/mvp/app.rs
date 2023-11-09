use arbiter_core::environment::{builder::EnvironmentBuilder, Environment};
use iced::{
    alignment,
    widget::{button, column, container, text},
    Length,
};

use super::*;

#[derive(Debug)]
pub enum Message {
    Empty,
}

/// Storage for the entire application.
/// This should hold the most important pieces of data that many children components will need.
pub struct App {
    arbiter: Environment,
    local: Local<Ws>,
}

impl App {
    pub fn new(arbiter: Environment, local: Local<Ws>) -> (Self, Command<Message>) {
        (Self { arbiter, local }, Command::none())
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        Command::none()
    }

    pub fn view(&self) -> Element<Message> {
        container(column![
            text("running app!").style(iced::theme::Text::Color(iced::Color::BLACK))
        ])
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()
    }
}
