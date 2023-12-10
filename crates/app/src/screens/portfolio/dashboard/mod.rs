//! Renders a view of the portfolio's positions and strategies.

pub mod stages;
pub mod table;

use std::{collections::HashMap, time::Instant};

use alloy_primitives::{utils::parse_ether, Address, U256};
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
use stages::Stages;
use uuid::Uuid;

use self::{stages::DashboardState, table::PortfolioTable};
use super::*;
use crate::components::{
    chart::CartesianChart,
    system::{
        Card, ExcaliburChart, ExcaliburColor, ExcaliburContainer, ExcaliburTable, ExcaliburText,
    },
    tables::{
        builder::TableBuilder, cells::CellBuilder, columns::ColumnBuilder, key_value_table,
        rows::RowBuilder,
    },
};

/// Executed on `load` for the Dashboard screen.
#[tracing::instrument(skip(dev_client), ret)]
async fn load_portfolio(
    dev_client: Option<DevClient<DefaultMiddleware>>,
    name: Option<String>,
) -> anyhow::Result<Portfolio, Arc<anyhow::Error>> {
    let path = name.clone().map(Portfolio::file_path_with_name);
    let portfolio = Portfolio::load(path);
    let mut portfolio = match portfolio {
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
    if std::env::var("DEV_MODE").is_ok() && dev_client.is_some() {
        let dev_client: DevClient<SignerMiddleware<Provider<Ws>, LocalWallet>> =
            dev_client.clone().unwrap();
        portfolio = fetch_portfolio(portfolio, dev_client).await?;
    }

    Ok(portfolio)
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
    /// Triggered every 3s.
    FetchPortfolioState,
    Tick,
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

/// The ENTIRE data model of the dashboard, stored in native types.
#[derive(Debug, Clone, Default)]
pub struct DataModel {
    /// AMM reported spot price.
    pub raw_internal_spot_price: Option<U256>,
    /// Spot price from an external source.
    pub raw_external_spot_price: Option<U256>,
    pub raw_asset_token: Option<Address>,
    pub raw_quote_token: Option<Address>,
    pub raw_user_asset_balance: Option<U256>,
    pub raw_user_quote_balance: Option<U256>,
    pub raw_asset_reserve: Option<U256>,
    pub raw_quote_reserve: Option<U256>,
    pub raw_user_asset_reserve: Option<U256>,
    pub raw_user_quote_reserve: Option<U256>,
    pub raw_total_liquidity: Option<U256>,
    pub raw_user_total_liquidity: Option<U256>,
    pub raw_strike_price_wad: Option<U256>,
    pub raw_time_remaining_wad: Option<U256>,
    pub raw_volatility_wad: Option<U256>,
    pub raw_portfolio_values_series: Option<Vec<(u64, U256)>>,
    pub raw_last_chain_data_sync_timestamp: Option<DateTime<Utc>>,
    pub raw_last_chain_data_sync_block: Option<u64>,
}

pub fn u256_to_label(value: Option<U256>) -> ExcaliburText {
    if let Some(value) = value {
        let value = alloy_primitives::utils::format_ether(value);
        match value.parse::<f64>() {
            Ok(_) => label(&value).quantitative().title1(),
            Err(_) => label(&"Failed to parse U256 as float.")
                .caption()
                .tertiary(),
        }
    } else {
        label(&"N/A").caption().tertiary()
    }
}

/// Computes the portfolio value as Sum(quote balance, asset balance * spot
/// price).
pub fn derive_portfolio_value(
    price: Option<U256>,
    quote_balance: Option<U256>,
    asset_balance: Option<U256>,
) -> Option<U256> {
    if let (Some(price), Some(quote_balance), Some(asset_balance)) =
        (price, quote_balance, asset_balance)
    {
        let price = alloy_primitives::utils::format_ether(price);
        let price = price.parse::<f64>().unwrap();
        let quote_balance = alloy_primitives::utils::format_ether(quote_balance);
        let quote_balance = quote_balance.parse::<f64>().unwrap();
        let asset_balance = alloy_primitives::utils::format_ether(asset_balance);
        let asset_balance = asset_balance.parse::<f64>().unwrap();

        let portfolio_value = quote_balance + asset_balance * price;
        let portfolio_value = format!("{}", portfolio_value);
        let portfolio_value = alloy_primitives::utils::parse_ether(&portfolio_value).unwrap();

        Some(portfolio_value)
    } else {
        None
    }
}

/// Compute the theoretical portfolio value given price, x, K, v, t.
pub fn derive_theoretical_portfolio_value(
    price: Option<U256>,
    liquidity: Option<U256>,
    strike_price: Option<U256>,
    volatility: Option<U256>,
    time_remaining: Option<U256>,
) -> Option<U256> {
    if let (
        Some(liquidity),
        Some(price),
        Some(strike_price),
        Some(volatility),
        Some(time_remaining),
    ) = (liquidity, price, strike_price, volatility, time_remaining)
    {
        let price = alloy_primitives::utils::format_ether(price);
        let price = price.parse::<f64>().unwrap();
        let liquidity = alloy_primitives::utils::format_ether(liquidity);
        let liquidity = liquidity.parse::<f64>().unwrap();
        let strike_price = alloy_primitives::utils::format_ether(strike_price);
        let strike_price = strike_price.parse::<f64>().unwrap();
        let volatility = alloy_primitives::utils::format_ether(volatility);
        let volatility = volatility.parse::<f64>().unwrap();
        let time_remaining = alloy_primitives::utils::format_ether(time_remaining);
        let time_remaining = time_remaining.parse::<f64>().unwrap();

        // Get x given price, then l given x, then y given l.
        let x = compute_x_given_l_rust(liquidity, price, strike_price, volatility, time_remaining);
        let l = compute_l_given_x_rust(x, price, strike_price, volatility, time_remaining);
        let y = compute_y_given_l_rust(l, price, strike_price, volatility, time_remaining);

        let portfolio_value = y + x * price;
        let portfolio_value = format!("{}", portfolio_value);
        let portfolio_value = alloy_primitives::utils::parse_ether(&portfolio_value).unwrap();

        Some(portfolio_value)
    } else {
        None
    }
}

/// Get the current portfolio value divided by the theoretically computed
/// portfolio value.
pub fn derive_portfolio_value_ratio(
    portfolio_value: Option<U256>,
    theoretical_portfolio_value: Option<U256>,
) -> Option<f64> {
    if let (Some(portfolio_value), Some(theoretical_portfolio_value)) =
        (portfolio_value, theoretical_portfolio_value)
    {
        let portfolio_value = alloy_primitives::utils::format_ether(portfolio_value);
        let portfolio_value = portfolio_value.parse::<f64>().unwrap();
        let theoretical_portfolio_value =
            alloy_primitives::utils::format_ether(theoretical_portfolio_value);
        let theoretical_portfolio_value = theoretical_portfolio_value.parse::<f64>().unwrap();

        Some(portfolio_value / theoretical_portfolio_value)
    } else {
        None
    }
}

/// The implementation defines how the data is synced and rendered.
impl DataModel {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn internal_price<'a, Message>(&self) -> Element<'a, Message>
    where
        Message: 'a,
    {
        double_labeled_data(
            u256_to_label(self.raw_internal_spot_price.clone()).build(),
            label(&"Internal Price").highlight().build(),
            label(&"ETH/USD").secondary().caption().build(),
        )
        .into()
    }

    pub fn portfolio_value<'a, Message>(&self) -> Element<'a, Message>
    where
        Message: 'a,
    {
        let pfv = derive_portfolio_value(
            self.raw_internal_spot_price.clone(),
            self.raw_user_quote_balance.clone(),
            self.raw_user_asset_balance.clone(),
        );

        double_labeled_data(
            u256_to_label(pfv).build(),
            label(&"Portfolio Value").highlight().build(),
            label(&"USD").secondary().caption().build(),
        )
        .into()
    }

    pub fn replication_health<'a, Message>(&self) -> Element<'a, Message>
    where
        Message: 'a,
    {
        let pfv = derive_portfolio_value(
            self.raw_internal_spot_price.clone(),
            self.raw_user_quote_balance.clone(),
            self.raw_user_asset_balance.clone(),
        );

        let theoretical_pfv = derive_theoretical_portfolio_value(
            self.raw_internal_spot_price.clone(),
            self.raw_total_liquidity.clone(),
            self.raw_strike_price_wad.clone(),
            self.raw_volatility_wad.clone(),
            self.raw_time_remaining_wad.clone(),
        );

        let ratio = derive_portfolio_value_ratio(pfv, theoretical_pfv);

        let ratio = match ratio {
            Some(ratio) => label(&format!("{}", ratio)).title1().percentage().build(),
            None => label(&"N/A").caption().tertiary().build(),
        };

        double_labeled_data(
            ratio,
            label(&"Replication Health").highlight().build(),
            label(&"PFV / tPFV").secondary().caption().build(),
        )
        .into()
    }

    pub fn tvl<'a, Message>(&self) -> Element<'a, Message>
    where
        Message: 'a,
    {
        let tvl = derive_portfolio_value(
            self.raw_internal_spot_price.clone(),
            self.raw_asset_reserve.clone(),
            self.raw_quote_reserve.clone(),
        );

        double_labeled_data(
            u256_to_label(tvl).build(),
            label(&"Total Value Locked").highlight().build(),
            label(&"USD").secondary().caption().build(),
        )
        .into()
    }
}

#[derive(Debug, Clone, Default)]
pub struct Dashboard {
    /// Underlying data structure.
    portfolio: Option<Portfolio>,
    /// Table to render the underlying data.
    table: PortfolioTable,
    /// Stages of the dashboard for interacting with the underlying data.
    stage: Stages,
    /// Original name of the loaded portfolio.
    loaded_from: Option<String>,
    /// Dev client for loading the portfolio.
    dev_client: Option<DevClient<DefaultMiddleware>>,
    /// Portfolio that is deposited into an active strategy.
    pub deposited_portfolio: Option<Portfolio>,
    /// Table for the deposited portfolio.
    pub deposited_table: PortfolioTable,

    pub data_model: DataModel,
    pub portfolio_values_plot: ExcaliburChart,
    pub trading_function_plot: ExcaliburChart,
}

impl Dashboard {
    pub type AppMessage = Message;
    pub type ViewMessage = Message;

    /// Try loading the portfolio from the name.
    pub fn new(name: Option<String>, dev_client: Option<DevClient<DefaultMiddleware>>) -> Self {
        Self {
            portfolio: None,
            table: PortfolioTable::new(),
            stage: Stages::new(dev_client.clone()),
            loaded_from: name,
            dev_client,
            deposited_portfolio: None,
            deposited_table: PortfolioTable::new(),
            data_model: DataModel::new(),
            portfolio_values_plot: ExcaliburChart::new().rmm_trading_fn(),
            trading_function_plot: ExcaliburChart::new().rmm_trading_fn(),
        }
    }

    pub fn loaded(&self) -> bool {
        self.portfolio.is_some()
    }

    pub fn sample_data(&mut self) {
        // Initial params: K = 1000, t = 1, v = 1.
        let strike_price = 1000.0;
        let strike_price_wad = parse_ether(&format!("{}", strike_price)).unwrap();
        let tau = 1.0;
        let tau_wad = parse_ether(&format!("{}", tau)).unwrap();
        let sigma = 1.0;
        let sigma_wad = parse_ether(&format!("{}", sigma)).unwrap();
        self.data_model.raw_strike_price_wad = Some(strike_price_wad);
        self.data_model.raw_time_remaining_wad = Some(tau_wad);
        self.data_model.raw_volatility_wad = Some(sigma_wad);

        // Initial deposit amounts and price.
        let initial_price = 1000.0;
        let initial_price_wad = parse_ether(&format!("{}", initial_price)).unwrap();
        self.data_model.raw_external_spot_price = Some(initial_price_wad);
        self.data_model.raw_internal_spot_price = Some(initial_price_wad);

        let initial_x = 1000.0;
        let initial_x_wad = parse_ether(&format!("{}", initial_x)).unwrap();
        self.data_model.raw_user_asset_balance = Some(initial_x_wad);
        self.data_model.raw_asset_reserve = Some(initial_x_wad);

        let init_liquidity =
            compute_l_given_x_rust(initial_x, initial_price, strike_price, sigma, tau);
        let init_liquidity_wad = parse_ether(&format!("{}", init_liquidity)).unwrap();
        self.data_model.raw_user_total_liquidity = Some(init_liquidity_wad);
        self.data_model.raw_total_liquidity = Some(init_liquidity_wad);

        let initial_y =
            compute_y_given_l_rust(init_liquidity, initial_price, strike_price, sigma, tau);
        let initial_y_wad = parse_ether(&format!("{}", initial_y)).unwrap();
        self.data_model.raw_user_quote_balance = Some(initial_y_wad);
        self.data_model.raw_quote_reserve = Some(initial_y_wad);
    }

    fn update_position(&mut self, price: f64) {
        // New price, need to adjust reserves and liquidity.
        let price_wad = parse_ether(&format!("{}", price)).unwrap();
        self.data_model.raw_internal_spot_price = Some(price_wad);
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

    pub fn render_header(&self) -> Element<'_, Self::AppMessage> {
        Column::new()
            .spacing(Sizes::Md)
            .push(label(&"Dashboard").title1().build())
            .into()
    }

    pub fn render_table(&self) -> Element<'_, Self::AppMessage> {
        Column::new()
            .spacing(Sizes::Lg)
            .push(label(&"Positions").title2().build())
            .push(self.table.view().map(|x| x.into()))
            .into()
    }

    pub fn render_deposited_table(&self) -> Element<'_, Self::AppMessage> {
        // If the table is not loaded, return an empty element.
        if self.deposited_portfolio.is_none() {
            return Container::new(label(&"No deposited positions.").build())
                .center_x()
                .center_y()
                .into();
        }

        Column::new()
            .spacing(Sizes::Lg)
            .push(label(&"Active Strategies").title2().build())
            .push(self.deposited_table.view().map(|x| x.into()))
            .into()
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
                space_between(self.data_model.internal_price(), self.data_model.tvl()).into(),
                space_between(
                    self.data_model.portfolio_value(),
                    self.data_model.replication_health(),
                )
                .into(),
            ))
            .push(
                Column::new()
                    .spacing(Sizes::Md)
                    .push(label(&"Portfolio value / time").highlight().build())
                    .push(self.portfolio_values_plot.build().map(|x| Message::Empty))
                    .push(label(&"Last sync: 1m ago").caption().tertiary().build()),
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
            .push(self.portfolio_values_plot.build().map(|x| Message::Empty))
            .push(label(&"Last sync: 1m ago").caption().tertiary().build()))
            .into()
    }

    pub fn quadrant_iii(&self) -> Element<'_, Self::AppMessage> {
        Column::new()
            .spacing(Sizes::Lg)
            .push(self.sample_table())
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
        let name = match self.loaded_from.clone() {
            Some(name) => Some(name),
            None => None,
        };

        let mut commands = vec![];

        // Loads the initial portfolio from the dev client
        let dev_client = self.dev_client.clone();
        commands.push(Command::perform(
            load_portfolio(dev_client, None),
            Message::Load,
        ));

        // todo: does this even work for the children components?
        // Loads the staging area, which enters the first stage.
        commands.push(self.stage.load().map(|x| x.into()));

        Command::batch(commands)
    }

    fn update(&mut self, message: Message) -> Command<Self::AppMessage> {
        match message {
            Message::Load(Ok(portfolio)) => {
                // Set some initial values in data model temporarily.
                self.sample_data();

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
            Message::Tick => {
                let random_val = rand::random::<f64>();
                let random_sign = rand::random::<bool>();
                let random_val = if random_sign { random_val } else { -random_val };
                let val = 1000.0 * (1.0 + (random_val as f64 / 2.0));

                self.update_position(val);
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
                    self.deposited_portfolio = Some(portfolio.clone());
                    commands.push(
                        self.deposited_table
                            .update(table::Message::Load(portfolio.clone()))
                            .map(|x| x.into()),
                    );
                }

                commands.push(self.stage.update(stage).map(|x| x.into()));

                return Command::batch(commands);
            }
            Message::FetchPortfolioState => {
                // Refetch the origin and deposited portfolios via dev client, if dev client is
                // available.
                if let (Some(dev_client), Some(origin)) =
                    (&self.dev_client, &self.portfolio).clone()
                {
                    let dev_client = dev_client.clone();
                    let origin = self.portfolio.clone();
                    let deposited = self.deposited_portfolio.clone();

                    let mut commands = vec![];

                    commands.push(Command::perform(
                        async move {
                            let origin = match origin {
                                Some(origin) => {
                                    Some(fetch_portfolio(origin, dev_client.clone()).await?)
                                }
                                None => None,
                            };
                            let deposited = match deposited {
                                Some(deposited) => {
                                    Some(fetch_portfolio(deposited, dev_client.clone()).await?)
                                }
                                None => None,
                            };
                            Ok::<(Option<Portfolio>, Option<Portfolio>), Arc<anyhow::Error>>((
                                origin, deposited,
                            ))
                        },
                        move |result| match result {
                            Ok((origin, _deposited)) => {
                                tracing::debug!(
                                    "Fetched portfolio state in FETCH message. {:?}",
                                    origin.clone()
                                );
                                if let Some(origin) = origin {
                                    Message::Load(Ok(origin))
                                } else {
                                    Message::Empty
                                }
                            }
                            Err(e) => {
                                tracing::error!("Failed to fetch portfolio state: {:?}", e);
                                Message::Empty
                            }
                        },
                    ));

                    // Fetches the deposited position, which emits an event that we capture in here
                    // anyway.
                    commands.push(Command::perform(async {}, |_| {
                        Message::UpdateStaging(stages::Message::Execute(
                            stages::execute::Message::FetchPosition,
                        ))
                    }));
                    return Command::batch(commands);
                }
            }
            _ => {}
        }

        Command::none()
    }

    // Layout is a 2x2 quadrant grid
    fn view(&self) -> Element<'_, Self::ViewMessage> {
        let mut quadrant_1 = ExcaliburContainer::default()
            .light_border()
            .build(Column::new().push(self.quadrant_i()))
            .padding(Sizes::Md);
        let mut quadrant_2 = ExcaliburContainer::default()
            .light_border()
            .build((self.quadrant_ii()))
            .padding(Sizes::Md);
        let mut quadrant_3 = ExcaliburContainer::default()
            .light_border()
            .build((self.quadrant_iii()))
            .padding(Sizes::Md);
        let mut quadrant_4 = ExcaliburContainer::default()
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
        let s1 = iced::time::every(std::time::Duration::from_secs(1)).map(|_| Message::Tick);
        // every 5s fetch portfolio state
        let s2 = iced::time::every(std::time::Duration::from_secs(5))
            .map(|_| Message::FetchPortfolioState);

        Subscription::batch(vec![s1])
    }
}
