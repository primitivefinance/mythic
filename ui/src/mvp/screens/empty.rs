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
    fn load(&self) -> Command<Message> {
        Command::none()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        Command::none()
    }

    fn view<'a>(&'a self) -> Element<'a, view::Message> {
        view::app_layout(
            &view::Page::Empty,
            Container::new(Column::new().push(h2("Select an app to get started.".to_string())))
                .center_x()
                .center_y()
                .width(Length::Fill)
                .height(Length::Fill),
        )
        .into()
    }
}
