use iced::{
    widget::{Column, Text},
    Command, Element, Subscription,
};

use super::{app::Message, state::State, view};

pub struct Execution;

impl Execution {
    pub fn new() -> Self {
        Self {}
    }
}

impl State for Execution {
    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Empty => Command::none(),
            Message::View(_) => Command::none(),
            Message::Simulation(_) => Command::none(),
            Message::Data(_) => Command::none(),
        }
    }

    fn view<'a>(&'a self) -> Element<'a, view::Message> {
        view::app_layout(&view::Page::Execute, view::execution_view()).into()
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::none()
    }

    fn load(&self) -> Command<Message> {
        Command::none()
    }
}
