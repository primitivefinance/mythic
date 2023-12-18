//! Controller for managing
//! 1. Dashboard controller
//! 2. Create portfolio controller

pub mod create;
pub mod dashboard;
pub mod monolithic;

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
    Monolithic(monolithic::Message),
    SyncModel,
}

#[derive(Debug, Clone, Default)]
pub enum Page {
    Empty,
    Create,
    Dashboard,
    #[default]
    Monolithic,
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
    pub monolithic: monolithic::Monolithic,
    pub client: Option<Arc<ExcaliburMiddleware<Ws, LocalWallet>>>,
}

impl PortfolioRoot {
    pub fn new(client: Option<Arc<ExcaliburMiddleware<Ws, LocalWallet>>>, model: Model) -> Self {
        Self {
            page: Page::default(),
            create: create::CreatePortfolio::new(model.user.clone()),
            dashboard: dashboard::Dashboard::new(None, client.clone(), model.clone()),
            monolithic: monolithic::Monolithic::new(client.clone(), model.clone()),
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
                Message::SyncModel => Command::perform(async {}, |_| {
                    Self::ViewMessage::Root(view::RootMessage::ModelSyncRequest)
                })
                .map(Self::AppMessage::View),
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
                            view::Message::Portfolio(Message::SyncModel)
                        })
                        .map(Self::AppMessage::View),
                    );
                    commands.push(
                        self.dashboard
                            .update(dashboard::Message::Refetch)
                            .map(|x| Message::Dashboard(x).into()),
                    );

                    Command::batch(commands)
                }
                Message::Monolithic(monolithic::Message::SyncModel(_block)) => {
                    Command::perform(async {}, |_| view::Message::Portfolio(Message::SyncModel))
                        .map(Self::AppMessage::View)
                }
                Message::Dashboard(message) => self
                    .dashboard
                    .update(message)
                    .map(|x| Message::Dashboard(x).into()),
                Message::Monolithic(message) => self
                    .monolithic
                    .update(message)
                    .map(|x| Message::Monolithic(x).into()),
            },

            // Lazy update, todo: this is kind of complicated, can we make it easier?
            // This will "catch" the root update model message and propagate it down to the
            // dashboard. The result of this is that when model updates happen in the
            // root controller, they will also sync the dashboard's model.
            Self::AppMessage::ModelSyncResult(model) => {
                Command::batch(vec![
                    self.dashboard
                        .update(dashboard::Message::UpdateDataModel(model.clone()))
                        .map(|x| Message::Dashboard(x).into()),
                    self.monolithic
                        .update(monolithic::Message::UpdateDataModel(model.clone()))
                        .map(|x| Message::Monolithic(x).into()),
                ])
            }
            _ => Command::none(),
        }
    }

    fn view(&self) -> Element<'_, Self::ViewMessage> {
        let content = match self.page.clone() {
            Page::Empty => Column::new().push(label("Select a page").build()).into(),
            Page::Create => self.create.view().map(|x| Message::Create(x).into()),
            Page::Dashboard => self.dashboard.view().map(|x| Message::Dashboard(x).into()),
            Page::Monolithic => self
                .monolithic
                .view()
                .map(|x| Message::Monolithic(x).into()),
        };

        Container::new(content)
            .center_x()
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn subscription(&self) -> Subscription<Self::AppMessage> {
        // todo: fix the subscriptions!
        // need subscriptions to fetch new blocks, new price path, etc.
        Subscription::batch(vec![
            self.dashboard
                .subscription()
                .map(|x| Message::Dashboard(x).into()),
            match self.page {
                Page::Empty => Subscription::none(),
                Page::Create => self
                    .create
                    .subscription()
                    .map(|x| Message::Create(x).into()),
                Page::Dashboard => self
                    .dashboard
                    .subscription()
                    .map(|x| Message::Dashboard(x).into()),
                Page::Monolithic => self
                    .monolithic
                    .subscription()
                    .map(|x| Message::Monolithic(x).into()),
            },
        ])
    }
}
