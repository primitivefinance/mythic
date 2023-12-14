//! Renders a view of the portfolio's positions and strategies.

pub mod stages;
pub mod table;

use std::collections::HashMap;

use alloy_primitives::{utils::parse_ether, U256};
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
use sim::{from_ethers_address, from_ethers_u256, to_ethers_address, to_ethers_u256};
use stages::Stages;
use uuid::Uuid;
use RustQuant::stochastics::{GeometricBrownianMotion, StochasticProcess, Trajectories};

use self::{stages::DashboardState, table::PortfolioTable};
use super::*;
use crate::{
    components::{
        system::{
            Card, ExcaliburButton, ExcaliburChart, ExcaliburColor, ExcaliburContainer,
            ExcaliburTable,
        },
        tables::{
            builder::TableBuilder, cells::CellBuilder, columns::ColumnBuilder, key_value_table,
            rows::RowBuilder,
        },
    },
    model::portfolio::{AlloyAddress, AlloyU256, RawDataModel, ALLOY_WAD},
    view::portfolio_view::DataView,
};

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
    client: DevClient<SignerMiddleware<Provider<Ws>, LocalWallet>>,
) -> anyhow::Result<Portfolio, Arc<anyhow::Error>> {
    let mut portfolio = portfolio.clone();
    let coin_x: Coin =
        serde_json::from_str(super::dev::COIN_X).map_err(|e| Arc::new(anyhow::Error::from(e)))?;
    let coin_y: Coin =
        serde_json::from_str(super::dev::COIN_Y).map_err(|e| Arc::new(anyhow::Error::from(e)))?;
    let balance_x = client.balance_of_x(client.client().address()).await?;
    let balance_y = client.balance_of_y(client.client().address()).await?;
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
    Load(Option<Portfolio>),
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

#[derive(Debug, Clone, Default)]
pub struct Dashboard {
    /// Connection to the network!
    pub client: Option<Arc<ExcaliburMiddleware<Ws, LocalWallet>>>,

    /// Root application model.
    pub model: Model,

    /// The portfolio that is loaded, synced, and saved on close.
    pub portfolio: Option<Portfolio>,

    /// The table state.
    pub table: PortfolioTable,

    /// The current action that the user is taking.
    pub stage: Stages,

    /// The view of the data model to render the dashboard components.
    pub data_view: view::portfolio_view::DataView,

    /// A test price process
    pub test_price_process: Option<PriceProcess>,
}

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

impl Dashboard {
    pub type AppMessage = Message;
    pub type ViewMessage = Message;

    /// Try loading the portfolio from the name.
    pub fn new(
        name: Option<String>,
        client: Option<Arc<ExcaliburMiddleware<Ws, LocalWallet>>>,
        model: Model,
    ) -> Self {
        let mut model = model.clone();
        let mut data_model = model.portfolio.clone();

        // Get the addresses from the dev client and setup the data_model with them.
        // todo: get a better place to do this.
        if let Some(client) = client.clone() {
            let user_address = client
                .address()
                .map_or(AlloyAddress::ZERO, from_ethers_address);
            tracing::info!("User address: {:?}", user_address);
            let (protocol, strategy, asset, quote, lex): (
                AlloyAddress,
                AlloyAddress,
                AlloyAddress,
                AlloyAddress,
                AlloyAddress,
            ) = (
                client
                    .contracts
                    .get("protocol")
                    .cloned()
                    .map_or(AlloyAddress::ZERO, from_ethers_address),
                client
                    .contracts
                    .get("strategy")
                    .cloned()
                    .map_or(AlloyAddress::ZERO, from_ethers_address),
                client
                    .contracts
                    .get("token_x")
                    .cloned()
                    .map_or(AlloyAddress::ZERO, from_ethers_address),
                client
                    .contracts
                    .get("token_y")
                    .cloned()
                    .map_or(AlloyAddress::ZERO, from_ethers_address),
                client
                    .contracts
                    .get("lex")
                    .cloned()
                    .map_or(AlloyAddress::ZERO, from_ethers_address),
            );

            data_model.setup(user_address, lex, protocol, strategy, asset, quote);
        }

        tracing::info!("Set user address: {:?}", data_model.user_address);

        // Create the data view based on this current model.
        let mut data_view = DataView::new(
            data_model.clone(),
            ExcaliburChart::new(),
            ExcaliburChart::new(),
        );
        data_view.update_model(data_model.clone());

        let process = Some(PriceProcess {
            trajectories: GeometricBrownianMotion::new(0.05, 0.9)
                .euler_maruyama(1.0, 0.0, 1.0, 1000, 1, false),
            step: 0,
            max_steps: 1000,
        });

        let portfolio = model.user.portfolio.clone();
        model.portfolio = data_model.clone();

        Self {
            client: client.clone(),
            model,
            portfolio: Some(portfolio),
            table: PortfolioTable::new(),
            stage: Stages::new(client),
            data_view,
            test_price_process: process,
        }
    }

    #[tracing::instrument(skip(self), level = "debug")]
    pub fn update_data(&self) -> Command<Message> {
        let mut commands = vec![];

        // Get the provider.
        if let Some(client) = &self.client {
            let client = client.client().cloned().unwrap();
            let data_model = self.model.portfolio.clone();
            let provider = Arc::new(client.provider().clone());
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
                    data.update(provider.clone()).await?;
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

    pub fn positions_table(&self) -> Element<'_, Self::AppMessage> {
        let portfolio = self.portfolio.clone();
        let position_x = portfolio
            .clone()
            .unwrap_or_default()
            .positions
            .0
            .iter()
            .find(|x| x.asset.symbol == "X")
            .cloned()
            .unwrap_or_default();

        let position_y = portfolio
            .clone()
            .unwrap_or_default()
            .positions
            .0
            .iter()
            .find(|x| x.asset.symbol == "Y")
            .cloned()
            .unwrap_or_default();

        let mut cell_data: Vec<Vec<CellBuilder<Self::ViewMessage>>> = vec![];

        if let (Some(x_cost), Some(x_balance), Some(x_weight)) =
            (position_x.cost, position_x.balance, position_x.weight)
        {
            let x_cost = format!("{}", x_cost);
            let x_balance = format!("{}", x_balance);
            let x_weight = format!("{}", x_weight);

            cell_data.push(vec![
                CellBuilder::new().child(label("ETH").body().build()),
                CellBuilder::new().child(label(&x_cost).quantitative().build()),
                CellBuilder::new().child(label(&x_balance).quantitative().build()),
                CellBuilder::new().child(label(&x_weight).percentage().build()),
            ]);
        }

        if let (Some(y_cost), Some(y_balance), Some(y_weight)) =
            (position_y.cost, position_y.balance, position_y.weight)
        {
            let y_cost = format!("{}", y_cost);
            let y_balance = format!("{}", y_balance);
            let y_weight = format!("{}", y_weight);

            cell_data.push(vec![
                CellBuilder::new().child(label("USDC").body().build()),
                CellBuilder::new().child(label(&y_cost).quantitative().build()),
                CellBuilder::new().child(label(&y_balance).quantitative().build()),
                CellBuilder::new().child(label(&y_weight).percentage().build()),
            ]);
        }

        // If no positions add an empty cell with "no data" label.
        if cell_data.is_empty() {
            cell_data.push(vec![
                CellBuilder::new().child(label("No data").secondary().build())
            ]);
        }

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
        let data_quadrant = Row::new()
            .spacing(Sizes::Sm)
            .push(Container::new(self.data_view.external_price()).width(Length::FillPortion(1)))
            .push(Container::new(self.data_view.tvl()).width(Length::FillPortion(1)))
            .push(
                Container::new(self.data_view.internal_portfolio_value())
                    .width(Length::FillPortion(1)),
            )
            .push(Container::new(self.data_view.portfolio_health()).width(Length::FillPortion(1)));

        Column::new()
            .spacing(Sizes::Lg)
            .push(data_quadrant)
            .push(
                Column::new()
                    .spacing(Sizes::Md)
                    .push(label(&"Liquidity curve").headline().highlight().build())
                    .push(
                        self.data_view
                            .portfolio_strategy_plot()
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
                    .push(label(&"Good morning, Alex.").title3().highlight().build())
                    .push(
                        label(&"Your portfolio has maintained replication health since inception. Consider reviewing your portfolio liquidity distribution to maximize liquidity provision.")
                            .body()
                            .build(),
                    ).push(
                        label(&format!("Date: {}", Utc::now().format("%Y-%m-%d"))).caption().tertiary().build(),
                    ),
            )
            .push(Column::new()
            .spacing(Sizes::Md)
            .push(label(&"Portfolio value / time").highlight().headline().build())
            .push(
                self.data_view
                    .portfolio_value_series()
                    .map(|x| Message::Empty),
            )
            .push(self.data_view.last_sync_timestamp()))
            .into()
    }

    pub fn quadrant_iii(&self) -> Element<'_, Self::AppMessage> {
        Column::new()
            .spacing(Sizes::Lg)
            .push(
                Row::new()
                    .spacing(Sizes::Md)
                    .push(label(&"Positions").highlight().build())
                    .push(
                        ExcaliburButton::new()
                            .transparent()
                            .build(label(&"Refetch").caption().secondary().build())
                            .on_press(Message::Refetch),
                    ),
            )
            .push(
                Column::new()
                    .spacing(Sizes::Sm)
                    .push(self.positions_table())
                    .push(self.data_view.last_sync_block()),
            )
            .into()
    }

    pub fn quadrant_iv(&self) -> Element<'_, Self::AppMessage> {
        self.render_staging_area()
    }
}

impl State for Dashboard {
    type AppMessage = Message;
    type ViewMessage = Message;

    /// todo: how to handle different portfolio loads.
    fn load(&self) -> Command<Self::AppMessage> {
        let mut commands = vec![];

        commands.push(self.update_data());

        // todo: does this even work for the children components?
        // Loads the staging area, which enters the first stage.
        commands.push(self.stage.load().map(|x| x.into()));

        Command::batch(commands)
    }

    fn update(&mut self, message: Message) -> Command<Self::AppMessage> {
        match message {
            Message::Load(portfolio) => {
                let mut commands: Vec<Command<Self::AppMessage>> = vec![];
                let portfolio = portfolio.unwrap_or_default();

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

                self.portfolio = Some(portfolio.clone());

                return Command::batch(commands);
            }
            Message::UpdateDataModel(Ok(data_model)) => {
                // Update the data model.
                self.model.portfolio = data_model.clone();
                // Sync the view model.
                self.data_view.update_model(data_model.clone());

                tracing::debug!(
                    "Synced data model to block: {:?}",
                    self.model
                        .portfolio
                        .raw_last_chain_data_sync_block
                        .unwrap_or_default()
                );

                let pos_info = self.model.portfolio.get_position_info();

                tracing::debug!("Attempting to fetch portfolio positions");
                // Update the portfolio with the new position info.
                if let (Some(ref portfolio), Ok(pos_info)) = (&self.portfolio, pos_info) {
                    tracing::debug!("Updating portfolio positions");
                    let mut portfolio = portfolio.clone();

                    // Based on the price of x and the balances, compute the weights of both.
                    let total_value =
                        pos_info.balance_x * pos_info.external_price + pos_info.balance_y;
                    let position_x_weight =
                        pos_info.balance_x * pos_info.external_price / total_value;
                    let position_y_weight = pos_info.balance_y / total_value;
                    let position_x_weight = Weight {
                        id: Uuid::new_v4(),
                        value: position_x_weight,
                    };
                    let position_y_weight = Weight {
                        id: Uuid::new_v4(),
                        value: position_y_weight,
                    };

                    let coin_x: Coin = serde_json::from_str(super::dev::COIN_X)
                        .map_err(|e| Arc::new(anyhow::Error::from(e)))
                        .expect("No x token");
                    let coin_y: Coin = serde_json::from_str(super::dev::COIN_Y)
                        .map_err(|e| Arc::new(anyhow::Error::from(e)))
                        .expect("No y token");

                    let position_x = Position::new(
                        coin_x,
                        Some(pos_info.external_price),
                        Some(pos_info.balance_x),
                        Some(position_x_weight),
                        None,
                    );

                    let position_y = Position::new(
                        coin_y,
                        Some(pos_info.quote_price),
                        Some(pos_info.balance_y),
                        Some(position_y_weight),
                        None,
                    );

                    // Workaround is to override the positions directly.
                    let positions = Positions::new(vec![position_x, position_y]);
                    portfolio.positions = positions;

                    // Update the table with the new position info.
                    tracing::debug!(
                        "Updating table with new position info. {:?}",
                        portfolio.positions
                    );
                    return Command::perform(async {}, move |_| {
                        Message::Load(Some(portfolio.clone()))
                    });
                }
            }
            Message::UpdateDataModel(Err(e)) => {
                tracing::error!("Failed to update data model: {:?}", e);
            }
            Message::Refetch => {
                return self.update_data();
            }
            Message::Tick => {
                let mut commands = vec![];

                let mut next_price = None;
                if let Some(mut process) = self.test_price_process.as_mut() {
                    process.step += 1;
                    if process.step < process.max_steps {
                        let price = process.trajectories.paths[0].get(process.step).cloned();
                        if let Some(price) = price {
                            next_price = Some(price);
                        }
                    }
                };

                if let (Some(external_exchange), Some(client)) = (
                    &self.model.portfolio.raw_external_exchange_address,
                    &self.client,
                ) {
                    tracing::info!("Tick, updating price.");
                    let client = client.client().cloned().unwrap();
                    // for testing live price chart.
                    let external_exchange = external_exchange.clone();

                    commands.push(Command::perform(
                        async move {
                            let next_price = next_price.unwrap_or_default();
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
                            let mut new_price = from_ethers_u256(current_price)
                                .checked_mul(random)
                                .unwrap()
                                .checked_div(ALLOY_WAD)
                                .unwrap();

                            if next_price > 0.0 {
                                new_price = parse_ether(&format!("{}", next_price))?;
                            }

                            let result =
                                lex.set_price(to_ethers_u256(new_price)).send().await?.await;

                            match result {
                                Ok(tx) => {
                                    tracing::info!("Updated price");
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
            .build(Column::new().push(self.quadrant_i()))
            .padding(Sizes::Md);
        let quadrant_2 = ExcaliburContainer::default()
            .build((self.quadrant_ii()))
            .padding(Sizes::Md);
        let quadrant_3 = ExcaliburContainer::default()
            .build((self.quadrant_iii()))
            .padding(Sizes::Md);
        let quadrant_4 = Container::new(self.quadrant_iv());

        Container::new(
            Column::new()
                .spacing(Sizes::Xl)
                .push(
                    Row::new()
                        .spacing(Sizes::Sm)
                        .push(quadrant_2.width(Length::FillPortion(2)))
                        .push(quadrant_1.width(Length::FillPortion(2)))
                        .height(Length::FillPortion(3))
                        .align_items(alignment::Alignment::Start),
                )
                .push(
                    Row::new()
                        .spacing(Sizes::Sm)
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
