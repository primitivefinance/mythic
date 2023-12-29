pub mod create;
mod inventory;
mod metrics;
pub mod tx_history;
mod view;

use alloy_primitives::utils::format_ether;
use datatypes::portfolio::coin::Coin;
use iced::{futures::TryFutureExt, subscription, Padding};

use self::{
    create::{FormView, LiquidityTypes},
    metrics::Metrics,
    tx_history::TxHistory,
    view::{MonolithicPresenter, MonolithicView},
};
use super::{dashboard::stages::review::Times, *};
use crate::{
    components::system::{ExcaliburChart, ExcaliburContainer},
    middleware::Protocol,
    model::portfolio::AlloyAddress,
    view::portfolio_view::PortfolioPresenter,
};

#[derive(Debug, Clone, Default)]
pub enum Message {
    #[default]
    Empty,
    StartAllocate,
    Form(FormMessage),
    SelectPosition(AlloyAddress),
    AllocateResult(anyhow::Result<Option<TransactionReceipt>, Arc<anyhow::Error>>),

    // todo: do we need these on all pages?? maybe just reference the  model.
    // I think we should use this pattern called the translator pattern:
    // https://medium.com/@alex.lew/the-translator-pattern-a-model-for-child-to-parent-communication-in-elm-f4bfaa1d3f98
    UpdateDataModel(Result<Model, Arc<anyhow::Error>>),
    // Trigger a re-sync
    SyncModel(Block<ethers::types::H256>),

    // placeholder
    Refresh,
}

#[derive(Debug, Clone, Default)]
pub enum FormMessage {
    #[default]
    Empty,
    Close,
    Amount(Option<String>),
    Asset(Coin),
    Quote(Coin),
    Duration(Times),
    EndPrice(Option<String>),
    Liquidity(LiquidityTypes),
    Submit,
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
    presenter: MonolithicPresenter,
    chart_presenter: PortfolioPresenter,
    create: create::Form,
    allocate: bool,
    view_position: Option<AlloyAddress>,
    create_status: create::SubmitState,
}

impl Monolithic {
    pub fn new(client: Option<Arc<ExcaliburMiddleware<Ws, LocalWallet>>>, model: Model) -> Self {
        let presenter = MonolithicPresenter::new(model.clone());
        let chart_presenter = PortfolioPresenter::default();
        Self {
            client,
            model,
            presenter,
            chart_presenter,
            create: create::Form::new(),
            allocate: false,
            view_position: None,
            create_status: create::SubmitState::Empty,
        }
    }

    pub fn submit_ready(&self) -> Option<FormMessage> {
        match self.create.state {
            create::SubmitState::Pending => None,
            create::SubmitState::Confirmed => Some(FormMessage::Close),
            create::SubmitState::Failed => Some(FormMessage::Close),
            _ => {
                if self.create.amount.is_some() && self.create.liquidity.is_some() {
                    Some(FormMessage::Submit)
                } else {
                    None
                }
            }
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

    pub fn handle_submit_allocate(&mut self) -> anyhow::Result<Command<Message>> {
        if let Some(client) = self.client.clone() {
            if let Some(signer) = client.signer() {
                let submitter = signer.address();

                let deposit_amount = self.create.amount.clone();
                let deposit_amount = match deposit_amount {
                    Some(x) => x.parse::<f64>().unwrap(),
                    None => return Err(anyhow::anyhow!("No deposit amount")),
                };

                let asset_price = self.model.portfolio.external_spot_price;
                let asset_price = match asset_price {
                    Some(x) => format_ether(x).parse::<f64>(),
                    None => return Err(anyhow::anyhow!("No asset price")),
                };
                let asset_price = match asset_price {
                    Ok(x) => x,
                    Err(_) => return Err(anyhow::anyhow!("Failed to parse")),
                };

                let parameters = self.create.liquidity;
                let parameters: LiquidityTypes = match parameters {
                    Some(x) => x,
                    None => return Err(anyhow::anyhow!("No liquidity parameters")),
                };
                let parameters = parameters.to_parameters(asset_price);

                let client = client.clone();
                return Ok(Command::perform(
                    async move {
                        client
                            .create_position(
                                submitter,
                                deposit_amount,
                                asset_price,
                                parameters.strike_price_wad,
                                parameters.sigma_percent_wad,
                                parameters.time_remaining_years_wad,
                            )
                            .map_err(Arc::new)
                            .await
                    },
                    Message::AllocateResult,
                ));
            }

            return Err(anyhow::anyhow!("No signer"));
        }

        Err(anyhow::anyhow!("No client"))
    }

    pub fn handle_form_message(&mut self, message: FormMessage) -> Command<Message> {
        match message {
            FormMessage::Empty => Command::none(),
            FormMessage::Close => {
                self.allocate = false;
                self.create.reset();
                Command::none()
            }
            FormMessage::Submit => {
                self.create.pending();
                self.create_status = create::SubmitState::Pending;

                match self.handle_submit_allocate() {
                    Ok(command) => command,
                    Err(err) => {
                        tracing::error!("Error when submitting allocate transaction: {:?}", err);
                        Command::none()
                    }
                }
            }
            FormMessage::Amount(amount) => {
                self.create.amount = amount;
                Command::none()
            }
            FormMessage::Asset(asset) => {
                self.create.chosen_asset = Some(asset);
                Command::none()
            }
            FormMessage::Quote(quote) => {
                self.create.chosen_quote = Some(quote);
                Command::none()
            }
            FormMessage::Duration(duration) => {
                self.create.duration = Some(duration);
                Command::none()
            }
            FormMessage::EndPrice(end_price) => {
                self.create.end_price = end_price;
                Command::none()
            }
            FormMessage::Liquidity(liquidity) => {
                self.create.liquidity = Some(liquidity);

                let external_price = self.model.portfolio.external_spot_price;
                let external_price = match external_price {
                    Some(x) => format_ether(x).parse::<f64>().unwrap(),
                    None => return Command::none(),
                };

                // Sync the strategy preview chart.
                let parameters = liquidity.to_parameters(external_price);
                self.presenter.sync_strategy_preview(
                    parameters.strike_price_wad,
                    parameters.sigma_percent_wad,
                    parameters.time_remaining_years_wad,
                );

                Command::perform(async {}, |_| Message::Refresh)
            }
        }
    }
}

impl State for Monolithic {
    type AppMessage = Message;
    type ViewMessage = Message;

    fn load(&self) -> Command<Self::AppMessage> {
        let model = self.model.clone();
        Command::perform(async {}, |_| Self::AppMessage::UpdateDataModel(Ok(model)))
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
            Self::AppMessage::SelectPosition(address) => {
                self.view_position = Some(address);
                Command::none()
            }
            Self::AppMessage::StartAllocate => {
                self.allocate = true;
                Command::none()
            }
            Self::AppMessage::Form(form_message) => self.handle_form_message(form_message),
            Self::AppMessage::AllocateResult(result) => match result {
                Ok(receipt) => {
                    tracing::info!("Receipt: {:?}", receipt);
                    self.create.confirmed();
                    self.create_status = create::SubmitState::Confirmed;
                    Command::none()
                }
                Err(err) => {
                    tracing::error!("Error: {:?}", err);
                    self.create_status = create::SubmitState::Failed;
                    self.create.failed();
                    Command::none()
                }
            },
        }
    }

    fn view(&self) -> Element<Self::ViewMessage> {
        let (allocated_positions, logos) = self.presenter.get_positions();
        let (unallocated_positions, _) = self.presenter.get_unallocated_positions();
        let mut content = Column::new().spacing(Sizes::Xl);
        content = content.push(MonolithicView::layout(
            self.presenter.get_aum(),
            unallocated_positions,
            allocated_positions,
            logos,
            Some(Message::StartAllocate),
            Message::SelectPosition,
        ));

        if let Some(address) = self.view_position {
            let (title, external_price, aum, health) = self.presenter.get_metrics(address);

            content = content.push(Metrics::layout(
                title,
                external_price,
                label("USD"),
                aum,
                health,
            ));

            content = content.push(ExcaliburContainer::default().build(FormView::chart_layout(
                &self.chart_presenter.portfolio_value_series,
                label("Portfolio Value").title2(),
                self.presenter.get_last_sync_timestamp(),
            )));

            content = content.push(ExcaliburContainer::default().build(FormView::chart_layout(
                &self.chart_presenter.portfolio_strategy_plot,
                label("Strategy").title2(),
                self.presenter.get_last_sync_timestamp(),
            )));
        }

        if self.allocate {
            content = content.push(
                self.create
                    .view::<FormMessage>(
                        &self.presenter.cached_strategy_histogram,
                        &self.create_status,
                        Some(FormMessage::Close),
                        self.submit_ready(),
                        FormMessage::Amount,
                        FormMessage::Asset,
                        FormMessage::Quote,
                        FormMessage::Duration,
                        FormMessage::EndPrice,
                        FormMessage::Liquidity,
                    )
                    .map(Message::Form),
            );
        }

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
            let provider = client.client().unwrap().clone();
            return listen_to_blocks(provider);
        }

        Subscription::none()
    }
}

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
