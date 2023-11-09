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
pub struct App;

impl App {
    pub fn new() -> (Self, Command<Message>) {
        (Self {}, Command::none())
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
