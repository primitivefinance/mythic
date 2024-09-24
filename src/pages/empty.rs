use iced::widget::Container;

use super::*;
use crate::components::system::label;

pub struct EmptyPage;

impl Default for EmptyPage {
    fn default() -> Self {
        Self::new()
    }
}

impl EmptyPage {
    pub fn new() -> Self {
        Self
    }
}

impl From<EmptyPage> for Page {
    fn from(screen: EmptyPage) -> Self {
        Page::new(Box::new(screen))
    }
}

impl Lifecycle for EmptyPage {
    type AppMessage = app::AppMessage;
    type ViewMessage = view::ViewMessage;

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
