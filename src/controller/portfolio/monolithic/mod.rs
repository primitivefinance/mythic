pub mod tx_history;
mod view;

use ethers::types::Address;
use iced::{futures::TryFutureExt, subscription, Padding};
use RustQuant::{
    models::GeometricBrownianMotion,
    stochastics::{StochasticProcess, Trajectories},
};

use self::tx_history::TxHistory;
use super::*;
use crate::{
    components::system::{ExcaliburChart, ExcaliburContainer},
    model::portfolio::{format_and_parse, WAD},
    view::portfolio_view::PortfolioPresenter,
};

#[derive(Debug, Clone, Default)]
pub enum Message {
    #[default]
    Empty,

    // todo: do we need these on all pages?? maybe just reference the  model.
    // I think we should use this pattern called the translator pattern:
    // https://medium.com/@alex.lew/the-translator-pattern-a-model-for-child-to-parent-communication-in-elm-f4bfaa1d3f98
    UpdateDataModel(Result<Model, Arc<anyhow::Error>>),
    // Trigger a re-sync
    SyncModel(Block<ethers::types::H256>),

    // placeholder
    Refresh,

    // Updates the price process, temp, todo: replace with real price.
    UpdatePriceProcess,
}

#[derive(Debug, Clone, Default)]
pub enum FormMessage {
    #[default]
    Empty,
}

impl MessageWrapper for Message {
    type ParentMessage = super::Message;
}

impl MessageWrapperView for Message {
    type ParentMessage = super::Message;
}

impl From<Message> for <Message as MessageWrapper>::ParentMessage {
    fn from(msg: Message) -> Self {
        super::Message::Monolithic(msg)
    }
}

#[derive(Debug, Clone, Default)]
pub struct Monolithic {
    client: Option<Arc<ExcaliburMiddleware<Ws, LocalWallet>>>,
    model: Model,
    presenter: view::MonolithicPresenter,
    chart_presenter: PortfolioPresenter,
    price_process: Option<PriceProcess>,
}

impl Monolithic {
    pub fn new(client: Option<Arc<ExcaliburMiddleware<Ws, LocalWallet>>>, model: Model) -> Self {
        let presenter = view::MonolithicPresenter::new(model.clone());
        let chart_presenter = PortfolioPresenter::default();

        // todo: integrate a live price process instead of this one!
        let process = Some(PriceProcess {
            trajectories: GeometricBrownianMotion::new(0.05, 0.9)
                .euler_maruyama(1.0, 0.0, 1.0, 1000, 1, false),
            step: 0,
            max_steps: 1000,
        });

        Self {
            client,
            model,
            presenter,
            chart_presenter,
            price_process: process,
        }
    }

    pub fn handle_updated_model(&mut self, updated_model: Model) -> Command<Message> {
        // Update the model
        self.model = updated_model.clone();

        // Update presenter
        self.presenter.update(updated_model.clone());

        // Update charts
        self.chart_presenter.update(updated_model.clone());

        // Re-cache historical txs.
        let txs = self.presenter.get_historical_txs();
        self.presenter.cache_historical_txs(txs);

        Command::none()
    }
}

impl State for Monolithic {
    type AppMessage = Message;
    type ViewMessage = Message;

    fn load(&self) -> Command<Self::AppMessage> {
        let model = self.model.clone();

        let update_data_model = Command::perform(async {}, move |_| {
            Self::AppMessage::UpdateDataModel(Ok(model.clone()))
        });

        Command::batch(vec![update_data_model])
    }

    fn update(&mut self, message: Self::AppMessage) -> Command<Self::AppMessage> {
        match message {
            Self::AppMessage::Refresh => Command::none(),
            Self::AppMessage::SyncModel(_block) => Command::none(),
            Self::AppMessage::UpdateDataModel(result) => match result {
                Ok(updated_model) => self.handle_updated_model(updated_model),
                Err(err) => {
                    tracing::error!("Error when updating data model: {:?}", err);
                    Command::none()
                }
            },
            Self::AppMessage::Empty => Command::none(),
            Self::AppMessage::UpdatePriceProcess => {
                if let (Some(_), Some(exchange)) = (
                    self.price_process.clone(),
                    self.model
                        .get_current()
                        .map(|x| x.external_exchange_address)
                        .unwrap_or_else(|| None),
                ) {
                    // Step the price process.
                    self.price_process.as_mut().unwrap().step += 1;

                    // Update the price of the exchange based on the new step.
                    return price_process_update_after_step(
                        self.price_process.clone().unwrap(),
                        exchange,
                        self.client.clone().unwrap(),
                    );
                }

                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Self::ViewMessage> {
        let mut content = Column::new().spacing(Sizes::Xl);
        content = content.push(view::MonolithicView::layout());
        content = content.push(TxHistory::layout(
            "Transaction History",
            "Portfolio",
            TxHistory::table(&self.presenter.historical_txs),
        ));

        scrollable(
            Container::new(content)
                .center_x()
                .padding(Padding {
                    top: Sizes::Xl.into(),
                    bottom: Sizes::Xl.into(),
                    left: Sizes::Xl2.into(),
                    right: Sizes::Xl2.into(),
                })
                .max_height(5000.0),
        )
        .into()
    }

    fn subscription(&self) -> Subscription<Self::AppMessage> {
        if let Some(client) = self.client.clone() {
            let provider = client.get_client();
            let mut subscriptions: Vec<Subscription<Message>> = vec![];

            // Fetches the most recent block and updates the model.
            subscriptions.push(listen_to_blocks(provider));

            // Steps the price process forward.
            // todo: remove this in favor of a live price feed.
            if self.price_process.clone().is_some() {
                let s1 = iced::time::every(std::time::Duration::from_secs(5))
                    .map(|_| Self::AppMessage::UpdatePriceProcess);
                subscriptions.push(s1);
            }

            return Subscription::batch(subscriptions);
        }

        Subscription::none()
    }
}

/// Fetches the most recent block and updates the model with the state in the
/// new block.
pub fn listen_to_blocks(
    provider: Arc<SignerMiddleware<Provider<Ws>, LocalWallet>>,
) -> Subscription<Message> {
    struct Blocks;

    subscription::channel(
        std::any::TypeId::of::<Blocks>(),
        0,
        |mut output| async move {
            let mut subscription = provider.subscribe_blocks().await.unwrap();
            loop {
                while let Some(block) = subscription.next().await {
                    output.try_send(Message::SyncModel(block)).unwrap();
                }
            }
        },
    )
}

/// For testing the UI with a "live" price.
pub struct PriceProcess {
    pub trajectories: Trajectories,
    pub step: usize,
    pub max_steps: usize,
}

impl Clone for PriceProcess {
    fn clone(&self) -> Self {
        let times: Vec<f64> = self.trajectories.times.clone();
        let paths: Vec<Vec<f64>> = self.trajectories.paths.clone();
        Self {
            trajectories: Trajectories { times, paths },
            step: self.step,
            max_steps: self.max_steps,
        }
    }
}

impl std::fmt::Debug for PriceProcess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PriceProcess")
            .field("trajectories", &self.trajectories.paths)
            .field("step", &self.step)
            .field("max_steps", &self.max_steps)
            .finish()
    }
}

/// This function will step the price process forward and update the price of
/// the liquid exchange in the development environment. We should eventually
/// replace this with a live price feed.
///
/// note: expects price process step to be incremented before calling this.
fn price_process_update_after_step(
    process: PriceProcess,
    exchange: Address,
    client: Arc<ExcaliburMiddleware<Ws, LocalWallet>>,
) -> Command<Message> {
    let mut next_price = None;

    if process.step < process.max_steps {
        let price = process.trajectories.paths[0].get(process.step).cloned();
        if let Some(price) = price {
            next_price = Some(price);
        }
    }

    let client = client.get_client();

    Command::perform(
        async move {
            let next_price = next_price.unwrap_or_default();
            let client = client.clone();

            match next_price {
                next_price => Ok(()),
                _ => Err(anyhow::anyhow!("No next price")),
            }
        },
        |_| Message::Empty,
    )
}
