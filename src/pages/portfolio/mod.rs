pub mod monolithic;

use iced::widget::Container;

use super::*;
use crate::{app::RootMessage, components::system::label, middleware::ExcaliburMiddleware};

#[derive(Debug, Clone, Default)]
pub enum Message {
    #[default]
    Empty,
    Monolithic(monolithic::Message),
    SyncModel,
}

#[derive(Debug, Clone, Default)]
pub enum PortfolioState {
    #[allow(dead_code)]
    Empty,
    #[default]
    Monolithic,
}

impl From<Message> for RootMessage {
    fn from(message: Message) -> Self {
        Self::View(view::ViewMessage::Portfolio(message))
    }
}

impl From<Message> for view::ViewMessage {
    fn from(message: Message) -> Self {
        Self::Portfolio(message)
    }
}

pub struct PortfolioRootPage {
    pub state: PortfolioState,
    pub monolithic: monolithic::Monolithic,
    pub client: Option<Arc<ExcaliburMiddleware<Ws, LocalWallet>>>,
}

impl PortfolioRootPage {
    pub fn new(client: Option<Arc<ExcaliburMiddleware<Ws, LocalWallet>>>, model: Model) -> Self {
        Self {
            state: PortfolioState::default(),
            monolithic: monolithic::Monolithic::new(client.clone(), model.clone()),
            client,
        }
    }
}

impl From<PortfolioRootPage> for Page {
    fn from(screen: PortfolioRootPage) -> Self {
        Page::new(Box::new(screen))
    }
}

impl Lifecycle for PortfolioRootPage {
    type AppMessage = app::AppMessage;
    type ViewMessage = view::ViewMessage;

    fn load(&self) -> Command<Self::AppMessage> {
        self.monolithic
            .load()
            .map(|x| Message::Monolithic(x).into())
    }

    fn update(&mut self, message: Self::AppMessage) -> Command<Self::AppMessage> {
        match message {
            Self::AppMessage::View(view::ViewMessage::Portfolio(message)) => match message {
                Message::SyncModel => Command::perform(async {}, |_| {
                    Self::ViewMessage::Root(view::RootMessage::ModelSyncRequest)
                })
                .map(Self::AppMessage::View),
                Message::Empty => Command::none(),
                Message::Monolithic(monolithic::Message::SyncModel(_block)) => {
                    Command::perform(async {}, |_| {
                        view::ViewMessage::Portfolio(Message::SyncModel)
                    })
                    .map(Self::AppMessage::View)
                }
                Message::Monolithic(message) => self
                    .monolithic
                    .update(message)
                    .map(|x| Message::Monolithic(x).into()),
            },
            // Lazy update, todo: this is kind of complicated, can we make it easier?
            // This will "catch" the root update model message and propagate it down to the
            // dashboard. The result of this is that when model updates happen in the
            // root controller, they will also sync the dashboard's model.
            Self::AppMessage::ModelSyncResult(model) => Command::batch(vec![self
                .monolithic
                .update(monolithic::Message::UpdateDataModel(model.clone()))
                .map(|x| Message::Monolithic(x).into())]),
            _ => Command::none(),
        }
    }

    fn view(&self) -> Element<'_, Self::ViewMessage> {
        let content = match self.state.clone() {
            PortfolioState::Empty => Column::new().push(label("Select a state").build()).into(),
            PortfolioState::Monolithic => self
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
        // Need to understand how they are broken
        // need subscriptions to fetch new blocks, new price path, etc.
        Subscription::batch(vec![match self.state {
            PortfolioState::Monolithic => self
                .monolithic
                .subscription()
                .map(|x| Message::Monolithic(x).into()),
            _ => Subscription::none(),
        }])
    }
}
