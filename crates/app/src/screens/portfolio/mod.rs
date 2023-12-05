pub mod create;
pub mod dashboard;
pub mod dev;

use iced::widget::Container;

use super::*;

#[derive(Debug, Clone, Default)]
pub enum Message {
    #[default]
    Empty,
    Create(create::Message),
    Dashboard(dashboard::Message),
}

pub struct PortfolioRoot;

impl PortfolioRoot {
    pub fn new() -> Self {
        Self
    }
}

impl From<PortfolioRoot> for Screen {
    fn from(screen: PortfolioRoot) -> Self {
        Screen::new(Box::new(screen))
    }
}

impl State for PortfolioRoot {
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
