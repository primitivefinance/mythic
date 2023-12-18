//! Empty is a default screen if no app is selected.

use iced::widget::Container;

use super::*;
use crate::components::system::label;

pub struct EmptyScreen;

impl EmptyScreen {
    pub fn new() -> Self {
        Self
    }
}

impl From<EmptyScreen> for Screen {
    fn from(screen: EmptyScreen) -> Self {
        Screen::new(Box::new(screen))
    }
}

impl State for EmptyScreen {
    type AppMessage = app::Message;
    type ViewMessage = view::Message;

    fn load(&self) -> Command<Self::AppMessage> {
        Command::none()
    }

    fn update(&mut self, _message: Self::AppMessage) -> Command<Self::AppMessage> {
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::ViewMessage> {
        Container::new(Column::new().push(label("Select an app to get started.").title2().build()))
            .center_x()
            .center_y()
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
