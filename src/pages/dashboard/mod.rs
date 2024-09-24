use iced::widget::{Column, Container};

use super::*;
use crate::components::system::label;

#[derive(Debug, Clone, Default)]
pub enum Message {
    #[default]
    Empty,
}

pub struct Dashboard;

impl Dashboard {
    pub fn new() -> Self {
        Self
    }
}

impl From<Dashboard> for Page {
    fn from(screen: Dashboard) -> Self {
        Page::new(Box::new(screen))
    }
}

impl Lifecycle for Dashboard {
    type AppMessage = app::AppMessage;
    type ViewMessage = view::ViewMessage;

    fn load(&self) -> Command<Self::AppMessage> {
        Command::none()
    }

    fn update(&mut self, message: Self::AppMessage) -> Command<Self::AppMessage> {
        match message {
            Self::AppMessage::View(view::ViewMessage::Dashboard(message)) => match message {
                Message::Empty => Command::none(),
            },
            _ => Command::none(),
        }
    }

    fn view(&self) -> Element<'_, Self::ViewMessage> {
        Container::new(Column::new().push(label("Dashboard").build()))
            .center_x()
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn subscription(&self) -> Subscription<Self::AppMessage> {
        Subscription::none()
    }
}

impl From<Message> for app::RootMessage {
    fn from(message: Message) -> Self {
        Self::View(view::ViewMessage::Dashboard(message))
    }
}

impl From<Message> for view::ViewMessage {
    fn from(message: Message) -> Self {
        Self::Dashboard(message)
    }
}
