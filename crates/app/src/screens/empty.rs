//! Empty is a default screen if no app is selected.

use iced::widget::Container;

use super::*;

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
        Container::new(Column::new().push(h2("Select an app to get started.".to_string())))
            .center_x()
            .center_y()
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
