pub mod create;
mod inventory;
mod metrics;
pub mod tx_history;
mod view;

use alloy_primitives::utils::format_ether;
use arbiter_bindings::bindings::liquid_exchange::LiquidExchange;
use cfmm_math::trading_functions::rmm::{
    compute_value_function, compute_x_given_l_rust, compute_y_given_x_rust,
};
use clients::protocol::{LogNormalF64, PoolInitParamsF64};
use datatypes::portfolio::coin::Coin;
use iced::{futures::TryFutureExt, subscription, Padding};
use sim::{from_ethers_u256, to_ethers_address, to_ethers_u256};
use RustQuant::stochastics::{GeometricBrownianMotion, StochasticProcess, Trajectories};

use self::{
    create::{FormView, LiquidityTypes, Times},
    metrics::Metrics,
    tx_history::TxHistory,
    view::{MonolithicPresenter, MonolithicView},
};
use super::*;
use crate::{
    components::system::{ExcaliburChart, ExcaliburContainer},
    model::portfolio::{format_and_parse, AlloyAddress, ALLOY_WAD},
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

    // Updates the price process, temp, todo: replace with real price.
    UpdatePriceProcess,
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
    price_process: Option<PriceProcess>,
}

impl Monolithic {
    pub fn new(client: Option<Arc<ExcaliburMiddleware<Ws, LocalWallet>>>, model: Model) -> Self {
        let presenter = MonolithicPresenter::new(model.clone());
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
            create: create::Form::new(),
            allocate: false,
            view_position: None,
            create_status: create::SubmitState::Empty,
            price_process: process,
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
            if let (Some(signer), Some(_)) = (client.signer.as_ref(), client.dfmm_client.as_ref()) {
                let _submitter = signer.address();
                let pool_id = 0; // todo: get pool id from the model.

                // Get the tokens from the user's data token list.
                let token_list = self.model.user.coins.clone();

                // Find the asset token, which has a tag of "ether".
                let asset_token = token_list
                    .tokens
                    .clone()
                    .into_iter()
                    .find(|token| token.tags.contains(&"ether".to_string()))
                    .map(|token| token.address);
                let asset_token = match asset_token {
                    Some(x) => x,
                    None => return Err(anyhow::anyhow!("No asset token")),
                };

                // Find the quote token, which has a tag of "stablecoin".
                let quote_token = token_list
                    .tokens
                    .clone()
                    .into_iter()
                    .find(|token| token.tags.contains(&"stablecoin".to_string()))
                    .map(|token| token.address);
                let quote_token = match quote_token {
                    Some(x) => x,
                    None => return Err(anyhow::anyhow!("No quote token")),
                };

                let deposit_amount_dollars = self.create.amount.clone();
                let deposit_amount_dollars = match deposit_amount_dollars {
                    Some(x) => x.parse::<f64>().unwrap(),
                    None => return Err(anyhow::anyhow!("No deposit amount")),
                };

                if self.model.get_current().is_none() {
                    return Err(anyhow::anyhow!(
                        "Data model is not connected to any network."
                    ));
                }

                // Does not panic because it's caught in the above if statement.
                let asset_price = self
                    .model
                    .get_current()
                    .unwrap()
                    .price_of_token(asset_token)?;
                let asset_price = format_and_parse(asset_price)?;

                let parameters = self.create.liquidity;
                let parameters: LiquidityTypes = match parameters {
                    Some(x) => x,
                    None => return Err(anyhow::anyhow!("No liquidity parameters")),
                };
                let parameters = parameters.to_parameters(asset_price);

                let (amount_x, _amount_y, _total_liquidity) = get_deposits_given_price(
                    asset_price,
                    deposit_amount_dollars,
                    parameters.strike_price_wad,
                    parameters.sigma_percent_wad,
                    parameters.time_remaining_years_wad,
                );

                let payload_params = PoolInitParamsF64::LogNormal(LogNormalF64 {
                    sigma: parameters.sigma_percent_wad,
                    strike: parameters.strike_price_wad,
                    tau: parameters.time_remaining_years_wad,
                    swap_fee: 0.003,
                });

                let init_price_wad =
                    alloy_primitives::utils::parse_ether(&format!("{}", asset_price))?;
                let init_price_wad = to_ethers_u256(init_price_wad);

                let init_reserve_x_wad = to_ethers_u256(alloy_primitives::utils::parse_ether(
                    &format!("{}", amount_x),
                )?);

                let client = client.clone();
                tracing::info!(
                    "Sending transaction to address: {:?}",
                    client
                        .dfmm_client
                        .clone()
                        .unwrap()
                        .protocol
                        .address()
                        .clone()
                );
                return Ok(Command::perform(
                    async move {
                        let dfmm = client
                            .dfmm_client
                            .as_ref()
                            .unwrap_or_else(|| panic!("No DFMM client in ExcaliburMiddleware"));

                        let payload = dfmm
                            .get_init_payload(
                                to_ethers_address(asset_token),
                                to_ethers_address(quote_token),
                                init_reserve_x_wad,
                                init_price_wad,
                                payload_params,
                            )
                            .await?;

                        // todo: handle mutable update to the pools array in the protocol client
                        // separately.
                        dfmm.initialize_pool(payload).map_err(Arc::new).await
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

                if let Some(connected_model) = self.model.get_current() {
                    // todo: placeholder tactic until we have proper asset selection in the flow.
                    // Get the token from the token list that has "ether" tag.
                    let token_list = self.model.user.coins.clone();
                    let asset_token = token_list
                        .tokens
                        .into_iter()
                        .find(|token| token.tags.contains(&"ether".to_string()))
                        .map(|token| token.address);

                    tracing::info!("Asset token: {:?}", asset_token);
                    let asset_token = match asset_token {
                        Some(x) => x,
                        None => return Command::none(),
                    };

                    let external_price = connected_model.price_of_token(asset_token);
                    tracing::info!("External price: {:?}", external_price);
                    let external_price = match external_price {
                        Ok(x) => format_and_parse(x).unwrap(),
                        Err(_) => return Command::none(),
                    };

                    let parameters = liquidity.to_parameters(external_price);
                    self.presenter.sync_strategy_preview(
                        external_price,
                        parameters.strike_price_wad,
                        parameters.sigma_percent_wad,
                        parameters.time_remaining_years_wad,
                    );

                    Command::perform(async {}, |_| Message::Refresh)
                } else {
                    Command::none()
                }
            }
        }
    }
}

impl State for Monolithic {
    type AppMessage = Message;
    type ViewMessage = Message;

    fn load(&self) -> Command<Self::AppMessage> {
        let model = self.model.clone();
        let coins = model.user.coins.clone();
        let base_asset = coins.tokens.first().unwrap().clone();
        let quote_asset = coins.tokens.last().unwrap().clone();

        let update_base_asset = Command::perform(async {}, move |_| {
            Self::AppMessage::Form(FormMessage::Asset(base_asset.clone()))
        });
        let update_quote_asset = Command::perform(async {}, move |_| {
            Self::AppMessage::Form(FormMessage::Quote(quote_asset.clone()))
        });
        let update_data_model = Command::perform(async {}, move |_| {
            Self::AppMessage::UpdateDataModel(Ok(model.clone()))
        });

        Command::batch(vec![
            update_base_asset,
            update_quote_asset,
            update_data_model,
        ])
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
    exchange: AlloyAddress,
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
            let lex = LiquidExchange::new(to_ethers_address(exchange), client);

            let current_price = lex.price().await?;
            // make the new price a random price +/- 1% of current price.
            let random = 1.0 + (rand::random::<f64>() - 0.5) * 0.01;
            let random =
                alloy_primitives::utils::parse_ether(format!("{}", random).as_str()).unwrap();
            let mut new_price = from_ethers_u256(current_price)
                .checked_mul(random)
                .unwrap()
                .checked_div(ALLOY_WAD)
                .unwrap();

            if next_price > 0.0 {
                new_price = alloy_primitives::utils::parse_ether(&format!("{}", next_price))?;
            }

            let result = lex.set_price(to_ethers_u256(new_price)).send().await?.await;

            match result {
                Ok(_tx) => {
                    tracing::info!(
                        "Updated external price process to: {:?}",
                        format_and_parse(new_price)?
                    );
                    Ok(())
                }
                Err(e) => {
                    tracing::error!("Failed to set price: {:?}", e);
                    Err(anyhow::Error::from(e))
                }
            }
        },
        |_| Message::Empty,
    )
}

/// L = Deposit $ / V(c)
/// x = X(L)
/// y = Y(x, L)
pub fn get_deposits_given_price(
    price: f64,
    amount_dollars: f64,
    strike_price_wad: f64,
    sigma_percent_wad: f64,
    tau_years_wad: f64,
) -> (f64, f64, f64) {
    let value_per =
        compute_value_function(price, strike_price_wad, sigma_percent_wad, tau_years_wad);

    let total_liquidity = amount_dollars / value_per;

    let amount_x = compute_x_given_l_rust(
        total_liquidity,
        price,
        strike_price_wad,
        sigma_percent_wad,
        tau_years_wad,
    );

    let amount_y = compute_y_given_x_rust(
        amount_x,
        total_liquidity,
        strike_price_wad,
        sigma_percent_wad,
        tau_years_wad,
    );

    (amount_x, amount_y, total_liquidity)
}
