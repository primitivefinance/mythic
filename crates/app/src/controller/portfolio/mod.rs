//! Controller for managing
//! 1. Dashboard controller
//! 2. Create portfolio controller

pub mod create;
pub mod dashboard;
pub mod dev;

use iced::widget::Container;

use super::*;
use crate::{
    app::RootMessage, components::system::label, middleware::ExcaliburMiddleware,
    model::user::UserProfile,
};

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

/// Intermediary controller that manages the controllers related to portfolio
/// management. Responsible for catching requests to update the root model and
/// pushing them to the root controller.
pub struct PortfolioRoot {
    pub page: Page,
    pub create: create::CreatePortfolio,
    pub dashboard: dashboard::Dashboard,
    pub client: Option<Arc<ExcaliburMiddleware<Ws, LocalWallet>>>,
}

impl PortfolioRoot {
    pub fn new(client: Option<Arc<ExcaliburMiddleware<Ws, LocalWallet>>>, model: Model) -> Self {
        Self {
            page: Page::default(),
            create: create::CreatePortfolio::new(model.user.clone()),
            dashboard: dashboard::Dashboard::new(None, client.clone(), model.clone()),
            client,
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
                Message::Dashboard(dashboard::Message::Refetch) => {
                    let mut commands = vec![];

                    // todo: very clunky way to push the sync model upstream...
                    commands.push(
                        Command::perform(async {}, |_| {
                            Self::ViewMessage::Root(view::RootMessage::ModelSyncRequest)
                        })
                        .map(|x| Self::AppMessage::View(x)),
                    );
                    commands.push(
                        self.dashboard
                            .update(dashboard::Message::Refetch)
                            .map(|x| Message::Dashboard(x).into()),
                    );

                    Command::batch(commands)
                }
                Message::Dashboard(message) => self
                    .dashboard
                    .update(message)
                    .map(|x| Message::Dashboard(x).into()),
            },

            // Lazy update, todo: this is kind of complicated, can we make it easier?
            // This will "catch" the root update model message and propagate it down to the
            // dashboard. The result of this is that when model updates happen in the
            // root controller, they will also sync the dashboard's model.
            Self::AppMessage::ModelSyncResult(model) => {
                return self
                    .dashboard
                    .update(dashboard::Message::UpdateDataModel(model))
                    .map(|x| Message::Dashboard(x).into())
            }
            _ => Command::none(),
        }
    }

    fn view(&self) -> Element<'_, Self::ViewMessage> {
        let content = match self.page.clone() {
            Page::Empty => Column::new().push(label(&"Select a page").build()).into(),
            Page::Create => self.create.view().map(|x| Message::Create(x).into()),
            Page::Dashboard => self.dashboard.view().map(|x| Message::Dashboard(x).into()),
        };

        Container::new(content)
            .center_x()
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn subscription(&self) -> Subscription<Self::AppMessage> {
        self.dashboard
            .subscription()
            .map(|x| Message::Dashboard(x).into())
    }
}
