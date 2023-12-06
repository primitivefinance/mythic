pub mod create;
pub mod dashboard;
pub mod dev;

use clients::dev::DevClient;
use iced::widget::Container;

use super::*;
use crate::{app::RootMessage, loader::DefaultMiddleware};

#[derive(Debug, Clone, Default)]
pub enum Message {
    #[default]
    Empty,
    Create(create::Message),
    Dashboard(dashboard::Message),
}

#[derive(Debug, Clone, Default)]
pub enum Page {
    Empty,
    Create,
    #[default]
    Dashboard,
}

impl From<Message> for RootMessage {
    fn from(message: Message) -> Self {
        Self::View(view::Message::Portfolio(message))
    }
}

impl From<Message> for view::Message {
    fn from(message: Message) -> Self {
        Self::Portfolio(message)
    }
}

pub struct PortfolioRoot {
    pub page: Page,
    pub create: create::CreatePortfolio,
    pub dashboard: dashboard::Dashboard,
    pub dev_client: Option<DevClient<DefaultMiddleware>>,
}

impl PortfolioRoot {
    pub fn new(dev_client: Option<DevClient<DefaultMiddleware>>) -> Self {
        Self {
            page: Page::default(),
            create: create::CreatePortfolio::new(),
            dashboard: dashboard::Dashboard::new(None, dev_client.clone()),
            dev_client,
        }
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
        self.dashboard.load().map(|x| Message::Dashboard(x).into())
    }

    fn update(&mut self, message: Self::AppMessage) -> Command<Self::AppMessage> {
        match message {
            Self::AppMessage::View(view::Message::Portfolio(message)) => match message {
                Message::Empty => Command::none(),
                Message::Create(message) => self
                    .create
                    .update(message)
                    .map(|x| Message::Create(x).into()),
                Message::Dashboard(message) => self
                    .dashboard
                    .update(message)
                    .map(|x| Message::Dashboard(x).into()),
            },
            _ => Command::none(),
        }
    }

    fn view(&self) -> Element<'_, Self::ViewMessage> {
        let content = match self.page.clone() {
            Page::Empty => Column::new().push(h2("Select a page".to_string())).into(),
            Page::Create => self.create.view().map(|x| Message::Create(x).into()),
            Page::Dashboard => self.dashboard.view().map(|x| Message::Dashboard(x).into()),
        };

        Container::new(content)
            .center_x()
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
