//! Renders a view of the portfolio's positions and strategies.

mod portfolio_model;
mod portfolio_view;
pub mod stages;
pub mod table;

use std::{collections::HashMap, time::Instant};

use alloy_primitives::{utils::parse_ether, Address, U256};
use arbiter_bindings::bindings::liquid_exchange::LiquidExchange;
use cfmm_math::trading_functions::rmm::{
    compute_l_given_x_rust, compute_x_given_l_rust, compute_y_given_l_rust, compute_y_given_x_rust,
};
use chrono::{DateTime, Utc};
use datatypes::portfolio::{
    coin::Coin,
    position::{Position, Positions},
    weight::Weight,
    Portfolio,
};
use iced::Color;
use serde::{Deserialize, Serialize, Serializer};
use sim::{from_ethers_address, from_ethers_u256, to_ethers_address, to_ethers_u256};
use stages::Stages;
use uuid::Uuid;

use self::{
    portfolio_model::{AlloyAddress, AlloyU256, RawDataModel, ALLOY_WAD},
    portfolio_view::DataView,
    stages::DashboardState,
    table::PortfolioTable,
};
use super::*;
use crate::{
    components::{
        chart::CartesianChart,
        system::{
            Card, ExcaliburButton, ExcaliburChart, ExcaliburColor, ExcaliburContainer,
            ExcaliburTable, ExcaliburText,
        },
        tables::{
            builder::TableBuilder, cells::CellBuilder, columns::ColumnBuilder, key_value_table,
            rows::RowBuilder,
        },
    },
    middleware::{Cortex, Watcher},
};

/// Executed on `load` for the Dashboard screen.
#[tracing::instrument(ret)]
async fn load_portfolio(name: Option<String>) -> anyhow::Result<Portfolio, Arc<anyhow::Error>> {
    let path = name.clone().map(Portfolio::file_path_with_name);
    let portfolio = Portfolio::load(path);
    let portfolio = match portfolio {
        Ok(portfolio) => portfolio,
        Err(_) => {
            // If dev mode, load the dev profile.
            if std::env::var("DEV_MODE").is_ok() {
                // Else create it
                tracing::debug!("Creating the dev portfolio.");
                Portfolio::create_new(Some("dev".to_string()))?
            } else {
                // Else load the default profile.
                tracing::debug!("Loading the default portfolio.");
                Portfolio::load(Some(Portfolio::file_path_with_name("default".to_string())))?
            }
        }
    };

    // If dev mode, load up the basic two coin portfolio.
    if std::env::var("DEV_MODE").is_ok() {
        // todo: use this
    }

    Ok(portfolio)
}

#[tracing::instrument(skip(client), ret)]
async fn fetch_balance(
    client: Arc<Provider<Ws>>,
    address: ethers::types::Address,
) -> anyhow::Result<ethers::types::U256, Arc<anyhow::Error>> {
    client
        .get_balance(address, None)
        .await
        .map_err(|e| Arc::new(anyhow::Error::from(e)))
}

async fn fetch_portfolio(
    portfolio: Portfolio,
    dev_client: DevClient<SignerMiddleware<Provider<Ws>, LocalWallet>>,
) -> anyhow::Result<Portfolio, Arc<anyhow::Error>> {
    let mut portfolio = portfolio.clone();
    let coin_x: Coin =
        serde_json::from_str(super::dev::COIN_X).map_err(|e| Arc::new(anyhow::Error::from(e)))?;
    let coin_y: Coin =
        serde_json::from_str(super::dev::COIN_Y).map_err(|e| Arc::new(anyhow::Error::from(e)))?;
    let balance_x = dev_client
        .balance_of_x(dev_client.client().address())
        .await?;
    let balance_y = dev_client
        .balance_of_y(dev_client.client().address())
        .await?;
    let initial_price = super::dev::INITIAL_X_PRICE;
    let balance_x = ethers::utils::format_ether(balance_x)
        .parse::<f64>()
        .unwrap();
    let balance_y = ethers::utils::format_ether(balance_y)
        .parse::<f64>()
        .unwrap();

    // Based on the price of x and the balances, compute the weights of both.
    let total_value = balance_x * initial_price + balance_y;
    let position_x_weight = balance_x * initial_price / total_value;
    let position_y_weight = balance_y / total_value;
    let position_x_weight = Weight {
        id: Uuid::new_v4(),
        value: position_x_weight,
    };
    let position_y_weight = Weight {
        id: Uuid::new_v4(),
        value: position_y_weight,
    };

    let position_x = Position::new(
        coin_x,
        Some(super::dev::INITIAL_X_PRICE),
        Some(balance_x),
        Some(position_x_weight),
        None,
    );

    let position_y = Position::new(
        coin_y,
        Some(super::dev::INITIAL_Y_PRICE),
        Some(balance_y),
        Some(position_y_weight),
        None,
    );

    // Add the positions to the portfolio.
    // note: ran into a problem of adding positions sequentially, as they would be
    // added then the validate() would get called, and if that weight is not 1 then
    // it won't sum to 1.
    // Will need to work on this.
    // Workaround is to override the positions directly.
    let positions = Positions::new(vec![position_x, position_y]);
    portfolio.positions = positions;

    Ok(portfolio)
}

#[derive(Debug, Clone, Default)]
pub enum Message {
    #[default]
    Empty,
    /// Triggered after `load_portfolio` completes.
    Load(anyhow::Result<Portfolio, Arc<anyhow::Error>>),
    /// Triggered on any form changes or button clicks within the staging area.
    UpdateStaging(stages::Message),
    /// Triggered when the user inputs a delta in the table.
    UpdateTable(table::Message),
    /// Triggered when new data is needs to be fetched.
    UpdateDataModel(Result<RawDataModel<AlloyAddress, AlloyU256>, Arc<anyhow::Error>>),
    /// Triggered when the view model needs to be synced to the data model.
    UpdateDataView,
    /// A 1s subscription.
    Tick,
    /// todo: remove
    Refetch,
}

impl MessageWrapperView for Message {
    type ParentMessage = super::Message;
}

impl MessageWrapper for Message {
    type ParentMessage = super::Message;
}

impl From<Message> for <Message as MessageWrapper>::ParentMessage {
    fn from(msg: Message) -> Self {
        super::Message::Dashboard(msg)
    }
}

pub struct TestCortex<P, S>
where
    P: JsonRpcClient + 'static,
    S: Signer + 'static,
{
    pub provider: Arc<Provider<P>>,
    pub signer: S,
    pub client: Arc<SignerMiddleware<Provider<P>, S>>,
    pub watcher: Option<Watcher>,
}

#[async_trait::async_trait]
impl Cortex for TestCortex<Ws, LocalWallet> {
    type Provider = Ws;
    type Signer = LocalWallet;
    type Middleware = SignerMiddleware<Provider<Ws>, LocalWallet>;

    fn client(&self) -> Arc<SignerMiddleware<Provider<Ws>, LocalWallet>> {
        self.client.clone()
    }

    fn provider(&self) -> Arc<Provider<Ws>> {
        self.provider.clone()
    }

    fn signer(&self) -> Option<LocalWallet> {
        Some(self.signer.clone())
    }

    fn protocol_address(&self) -> ethers::types::Address {
        ethers::types::Address::zero()
    }

    fn strategy_address(&self) -> ethers::types::Address {
        ethers::types::Address::zero()
    }
}

#[derive(Debug, Clone, Default)]
pub struct Dashboard {
    /// Connection to the network!
    pub cortex: Option<
        Arc<
            dyn Cortex<
                Middleware = SignerMiddleware<Provider<Ws>, LocalWallet>,
                Provider = Ws,
                Signer = LocalWallet,
            >,
        >,
    >,

    /// The portfolio that is loaded, synced, and saved on close.
    pub portfolio: Option<Portfolio>,

    /// The table state.
    pub table: PortfolioTable,

    /// The current action that the user is taking.
    pub stage: Stages,

    /// The underlying data model of the dashboard.
    pub data_model: RawDataModel<AlloyAddress, AlloyU256>,

    /// The view of the data model to render the dashboard components.
    pub data_view: DataView,
}

impl Dashboard {
    pub type AppMessage = Message;
    pub type ViewMessage = Message;

    /// Try loading the portfolio from the name.
    pub fn new(name: Option<String>, dev_client: Option<DevClient<DefaultMiddleware>>) -> Self {
        // Converts dev client to cortex, todo: just have cortex.
        let cortex = if let Some(dev) = dev_client.clone() {
            let client: DevClient<DefaultMiddleware> = dev.clone();
            let cortex: Arc<
                dyn Cortex<
                    Middleware = SignerMiddleware<Provider<Ws>, LocalWallet>,
                    Provider = Ws,
                    Signer = LocalWallet,
                >,
            > = Arc::new(TestCortex {
                provider: client.client().provider().clone().into(),
                signer: client.client().signer().clone(),
                client: client.client().clone(),
                watcher: None,
            });

            Some(cortex)
        } else {
            None
        };

        let mut data_model = RawDataModel::<AlloyAddress, AlloyU256>::default();

        // Get the addresses from the dev client and setup the data_model with them.
        // todo: get a better place to do this.
        if let Some(dev_client) = dev_client.clone() {
            let address = dev_client.client().address();
            let user_address = from_ethers_address(address);
            let raw_protocol_address = from_ethers_address(dev_client.protocol.protocol.address());
            let raw_strategy_address = from_ethers_address(dev_client.strategy.address());
            let raw_asset_token = from_ethers_address(dev_client.token_x.address());
            let raw_quote_token = from_ethers_address(dev_client.token_y.address());
            let lex = from_ethers_address(dev_client.liquid_exchange.address());

            data_model.setup(
                user_address,
                lex,
                raw_protocol_address,
                raw_strategy_address,
                raw_asset_token,
                raw_quote_token,
            );
        }

        // Create the data view based on this current model.
        let mut data_view = DataView::new(
            RawDataModel::<AlloyAddress, AlloyU256>::default(),
            ExcaliburChart::new(),
            ExcaliburChart::new(),
        );
        data_view.update_model(data_model.clone());

        Self {
            cortex,
            portfolio: None,
            table: PortfolioTable::new(),
            stage: Stages::new(dev_client),
            data_model,
            data_view,
        }
    }

    #[tracing::instrument(skip(self), level = "debug")]
    pub fn update_data(&self) -> Command<Message> {
        let mut commands = vec![];

        // Get the provider.
        if let Some(cortex) = &self.cortex {
            let client = cortex.provider().clone();
            let data_model = self.data_model.clone();
            commands.push(Command::perform(
                async move {
                    tracing::info!(
                        "Updating model at block: {}",
                        client
                            .get_block_number()
                            .await
                            .map_err(|e| { Arc::new(anyhow::Error::from(e)) })?
                    );
                    // Get the data
                    let mut data = data_model;
                    // Update the data
                    data.update(client).await?;
                    // Return the data
                    Ok(data)
                },
                Message::UpdateDataModel,
            ));
        }

        Command::batch(commands)
    }

    pub fn loaded(&self) -> bool {
        self.portfolio.is_some()
    }

    #[tracing::instrument(skip(self), level = "debug")]
    pub fn adjusted_portfolio_from_table(&self) -> Option<Portfolio> {
        let mut portfolio = self.portfolio.clone()?;
        let mut positions = portfolio.positions.0.clone();

        // Loop over each position and see if there is an existing __weight__ delta.
        let mut adjusted = false;
        for (i, position) in positions.iter_mut().enumerate() {
            let delta = self.table.form.weight.get(&i).cloned();

            // Skip if delta == "-"
            if delta.is_none() || delta.clone().unwrap() == "-" {
                continue;
            }

            let delta = delta.unwrap().parse::<f64>();
            let delta = match delta {
                Ok(delta) => delta,
                Err(e) => {
                    tracing::error!("Failed to parse delta: {:?}", e);
                    continue;
                }
            };

            //  Adjust the portfolio with using the weight delta.
            let weight = position.weight.unwrap_or_default().clone();
            portfolio.adjust(weight.id, delta).unwrap();
            adjusted = true;
        }

        if !adjusted {
            return None;
        }

        Some(portfolio)
    }

    pub fn sample_table(&self) -> Element<'_, Self::AppMessage> {
        let cell_data: Vec<Vec<CellBuilder<Self::ViewMessage>>> = vec![
            vec![
                CellBuilder::new().child(label("BTC").secondary().build()),
                CellBuilder::new().child(label("1000000.00").quantitative().build()),
                CellBuilder::new().child(label("0.00000000").quantitative().build()),
                CellBuilder::new().child(label("0.09").percentage().build()),
            ],
            vec![
                CellBuilder::new().child(label("ETH").secondary().build()),
                CellBuilder::new().child(label("5000").quantitative().build()),
                CellBuilder::new().child(label("0.00000000").quantitative().build()),
                CellBuilder::new().child(label("0.90").percentage().build()),
            ],
            vec![
                CellBuilder::new().child(label("USDC").secondary().build()),
                CellBuilder::new().child(label("1").quantitative().build()),
                CellBuilder::new().child(label("0.00000000").quantitative().build()),
                CellBuilder::new().child(label("0.01").percentage().build()),
            ],
        ];

        let exp_table = ExcaliburTable::new()
            .header("Asset")
            .header("Price")
            .header("Balance")
            .header("Weight")
            .build_custom(cell_data);

        exp_table.into()
    }

    pub fn render_staging_area(&self) -> Element<'_, Self::AppMessage> {
        match self.stage.current {
            DashboardState::Empty => {
                let instruct: Element<'_, Self::AppMessage> = instructions(
                    vec![instruction_text(
                        "Change the position deltas in the table to start the portfolio adjustment process.".to_string(),
                    )],
                    Some("Continue".to_string()),
                    None,
                    None,
                )
                .into();

                Row::new()
                    .spacing(Sizes::Lg)
                    .push(
                        Column::new()
                            .align_items(alignment::Alignment::Start)
                            .push(
                                Card::new(
                                    label(&"Make adjustments to view the estimated results")
                                        .title3()
                                        .build(),
                                )
                                .center_x()
                                .center_y()
                                .padding(Sizes::Lg)
                                .width(Length::Fill),
                            )
                            .width(Length::FillPortion(3)),
                    )
                    .push(
                        Column::new()
                            .align_items(alignment::Alignment::End)
                            .push(instruct)
                            .width(Length::FillPortion(1)),
                    )
                    .into()
            }
            _ => self.stage.view().map(|x| x.into()),
        }
    }

    pub fn quadrant_i(&self) -> Element<'_, Self::AppMessage> {
        Column::new()
            .spacing(Sizes::Lg)
            .push(space_between(
                space_between(self.data_view.external_price(), self.data_view.tvl()).into(),
                space_between(
                    self.data_view.internal_portfolio_value(),
                    self.data_view.portfolio_health(),
                )
                .into(),
            ))
            .push(
                Column::new()
                    .spacing(Sizes::Md)
                    .push(label(&"Portfolio value / time").highlight().build())
                    .push(
                        self.data_view
                            .portfolio_value_series()
                            .map(|x| Message::Empty),
                    )
                    .push(self.data_view.last_sync_timestamp()),
            )
            .into()
    }

    pub fn quadrant_ii(&self) -> Element<'_, Self::AppMessage> {
        Column::new()
            .spacing(Sizes::Lg)
            .push(
                Column::new()
                    .spacing(Sizes::Sm)
                    .push(label(&"Good morning, X938A.").title3().highlight().build())
                    .push(
                        label(&"Your portfolio has maintained replication health since inception. Consider reviewing your portfolio liquidity distribution to maximize liquidity provision.")
                            .title3()
                            .highlight()
                            .build(),
                    ),
            )
            .push(Column::new()
            .spacing(Sizes::Xs)
            .push(self.data_view.portfolio_strategy_plot().map(|x| Message::Empty))
            .push(self.data_view.last_sync_timestamp()))
            .into()
    }

    pub fn quadrant_iii(&self) -> Element<'_, Self::AppMessage> {
        Column::new()
            .spacing(Sizes::Lg)
            .push(self.sample_table())
            .push(
                ExcaliburButton::new()
                    .primary()
                    .build(label(&"Refetch").build())
                    .on_press(Message::Refetch),
            )
            .into()
    }

    pub fn quadrant_iv(&self) -> Element<'_, Self::AppMessage> {
        Column::new()
            .spacing(Sizes::Lg)
            .push(self.render_staging_area())
            .into()
    }
}

impl State for Dashboard {
    type AppMessage = Message;
    type ViewMessage = Message;

    /// todo: how to handle different portfolio loads.
    fn load(&self) -> Command<Self::AppMessage> {
        let mut commands = vec![];

        commands.push(Command::perform(load_portfolio(None), Message::Load));

        commands.push(self.update_data());

        // todo: does this even work for the children components?
        // Loads the staging area, which enters the first stage.
        commands.push(self.stage.load().map(|x| x.into()));

        Command::batch(commands)
    }

    fn update(&mut self, message: Message) -> Command<Self::AppMessage> {
        match message {
            Message::Load(Ok(portfolio)) => {
                // self.data_model.raw
                let mut commands: Vec<Command<Self::AppMessage>> = vec![];

                // Store the portfolio in the staging area to reference it.
                commands.push(
                    self.stage
                        .update(stages::Message::Load(portfolio.clone()))
                        .map(|x| x.into()),
                );

                // Store the portfolio in the table to reference it.
                commands.push(
                    self.table
                        .update(table::Message::Load(portfolio.clone()))
                        .map(|x| x.into()),
                );

                return Command::batch(commands);
            }
            Message::Load(Err(e)) => {
                tracing::error!("Failed to load portfolio: {:?}", e);
            }
            Message::UpdateDataModel(Ok(data_model)) => {
                // Update the data model.
                self.data_model = data_model.clone();
                // Sync the view model.
                self.data_view.update_model(data_model.clone());

                tracing::info!("Updated to data model: {:?}", self.data_model);
            }
            Message::UpdateDataModel(Err(e)) => {
                tracing::error!("Failed to update data model: {:?}", e);
            }
            Message::Refetch => {
                return self.update_data();

                // Get the provider.
                /* if let Some(cortex) = &self.cortex {
                    let provider = cortex.provider().clone();

                    let address = cortex.client().address();
                    tracing::info!("Fetching balance for address: {:?}", address);

                    // Fetch the balance of the caller.
                    return Command::perform(fetch_balance(provider, address), move |result| {
                        match result {
                            Ok(balance) => {
                                tracing::info!("Fetched balance: {:?}", balance);
                                Message::Empty
                            }
                            Err(e) => {
                                tracing::error!("Failed to fetch balance: {:?}", e);
                                Message::Empty
                            }
                        }
                    });
                } */
            }
            Message::Tick => {
                let mut commands = vec![];

                if let (Some(external_exchange), Some(cortex)) =
                    (&self.data_model.raw_external_exchange_address, &self.cortex)
                {
                    tracing::info!("Tick, updating price.");
                    let client = cortex.clone().client().clone();
                    // for testing live price chart.
                    let external_exchange = external_exchange.clone();
                    commands.push(Command::perform(
                        async move {
                            let client = client.clone();
                            let external_exchange = external_exchange.clone();
                            // Call the set_price function on the external exchange.
                            let lex = LiquidExchange::new(
                                to_ethers_address(external_exchange.clone()),
                                client,
                            );

                            let current_price = lex.price().await?;
                            // make the new price a random price +/- 1% of current price.
                            let random = (1.0 + (rand::random::<f64>() - 0.5) * 0.01);
                            let random = parse_ether(format!("{}", random).as_str()).unwrap();
                            let new_price = from_ethers_u256(current_price)
                                .checked_mul(random)
                                .unwrap()
                                .checked_div(ALLOY_WAD)
                                .unwrap();

                            let result =
                                lex.set_price(to_ethers_u256(new_price)).send().await?.await;

                            match result {
                                Ok(tx) => {
                                    tracing::info!("Updated price. {:?}", tx);
                                    Ok(())
                                }
                                Err(e) => {
                                    tracing::error!("Failed to set price: {:?}", e);
                                    Err(anyhow::Error::from(e))
                                }
                            }
                        },
                        |_| Message::Empty,
                    ));
                }

                commands.push(self.update_data());

                return Command::batch(commands);
            }
            // todo: this might be a little slow, since it gets the adjusted portfolio.
            Message::UpdateTable(message) => {
                // Catch the WeightUpdated message and try to update the staging area with the
                // adjusted portfolio.
                if let table::Message::DeltaForm(table::form::DeltaFormMessage::WeightUpdated) =
                    message
                {
                    tracing::debug!("Weight updated, updating staging area.");

                    let adjusted = self.adjusted_portfolio_from_table();
                    if adjusted.is_none() {
                        tracing::debug!("Weight updated but adjusted portfolio is None.");
                    }

                    let mut commands = vec![];

                    // todo: clean up this logic?
                    if adjusted.is_some() {
                        commands = adjusted
                            .clone()
                            .unwrap()
                            .positions
                            .0
                            .iter()
                            .enumerate()
                            .map(|(pos_index, position)| {
                                let balance =
                                    position.balance.clone().unwrap_or_default().to_string();
                                let market_value = position.market_value().clone().to_string();

                                let balance_command = self
                                    .table
                                    .update(table::Message::DeltaForm(
                                        table::form::DeltaFormMessage::Balance(
                                            pos_index,
                                            Some(balance),
                                        ),
                                    ))
                                    .map(|x| x.into());

                                let market_value_command = self
                                    .table
                                    .update(table::Message::DeltaForm(
                                        table::form::DeltaFormMessage::MarketValue(
                                            pos_index,
                                            Some(market_value.clone()),
                                        ),
                                    ))
                                    .map(|x| x.into());

                                vec![balance_command, market_value_command]
                            })
                            .flatten()
                            .collect::<Vec<Command<Self::AppMessage>>>();
                    }

                    commands.push(
                        self.stage
                            .update(stages::Message::SetAdjusted(adjusted))
                            .map(|x| x.into()),
                    );

                    return Command::batch(commands);
                }

                return self.table.update(message.clone()).map(|x| x.into());
            }
            Message::UpdateStaging(stage) => {
                let mut commands = vec![];
                // If its an Execute::NewStrategyPosition message, then update the deposited
                // portfolio.
                if let stages::Message::Execute(stages::execute::Message::NewStrategyPosition(
                    portfolio,
                )) = stage.clone()
                {
                    // todo: update current position table with new strategy
                    // position.
                }

                // Catch the FetchPositionResult and call update_data.
                if let stages::Message::Execute(stages::execute::Message::FetchPositionResult(_)) =
                    stage.clone()
                {
                    tracing::info!("Caught fetch position, updating data model.");
                    commands.push(self.update_data());
                }

                commands.push(self.stage.update(stage).map(|x| x.into()));

                return Command::batch(commands);
            }
            _ => {}
        }

        Command::none()
    }

    // Layout is a 2x2 quadrant grid
    fn view(&self) -> Element<'_, Self::ViewMessage> {
        let quadrant_1 = ExcaliburContainer::default()
            .light_border()
            .build(Column::new().push(self.quadrant_i()))
            .padding(Sizes::Md);
        let quadrant_2 = ExcaliburContainer::default()
            .light_border()
            .build((self.quadrant_ii()))
            .padding(Sizes::Md);
        let quadrant_3 = ExcaliburContainer::default()
            .light_border()
            .build((self.quadrant_iii()))
            .padding(Sizes::Md);
        let quadrant_4 = ExcaliburContainer::default()
            .light_border()
            .build((self.quadrant_iv()))
            .padding(Sizes::Md);

        Container::new(
            Column::new()
                .spacing(Sizes::Lg)
                .push(
                    Row::new()
                        .spacing(Sizes::Lg)
                        .push(quadrant_2.width(Length::FillPortion(2)))
                        .push(quadrant_1.width(Length::FillPortion(2)))
                        .height(Length::FillPortion(2)),
                )
                .push(
                    Row::new()
                        .spacing(Sizes::Lg)
                        .push(quadrant_3.width(Length::FillPortion(2)))
                        .push(quadrant_4.width(Length::FillPortion(2)))
                        .height(Length::FillPortion(2)),
                )
                .width(Length::Fill),
        )
        .into()
    }

    fn subscription(&self) -> Subscription<Self::AppMessage> {
        let s1 = iced::time::every(std::time::Duration::from_secs(5)).map(|_| Message::Tick);
        Subscription::batch(vec![s1])
    }
}
