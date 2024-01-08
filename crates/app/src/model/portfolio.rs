//! In the Model-view-controller architecture the model is ["The central
//! component of the pattern. It is the application's dynamic data structure,
//! independent of the user interface.[14] It directly manages the data, logic
//! and rules of the application."](https://en.wikipedia.org/wiki/Model%E2%80%93view%E2%80%93controller)
//!
//! For the Portfolio dashboard, there is a lot of convoluted data that needs to
//! be managed. The portfolio dashboard is the most complex screen in the
//! application. It is responsible for:
//! - Fetching protocol state
//! - Fetching user address information
//! - Maintaining data freshness
//! - Deriving data from underlying data
//! - Updating and manipulating data
//! - Computing plots off the data to be displayed in charts
//! - Subscribing to data feeds from asynchronous sources
//!
//! # Glossary
//! - "get" - Returns values stored in the model. Cheap.
//! - "fetch" - Async call to acquire data from an external source. Expensive.
//! - "compute" - Computes a result based on inputs. Can be expensive.
//! - "derive" - Computes a result derived from model data input. Expensive.

use std::{cmp::Ordering, collections::BTreeMap};

// use alloy_rpc_types::raw_log;
use alloy_sol_types::{sol, SolCall};
use anyhow::{anyhow, Error, Result};
use bindings::{
    dfmm::{InitFilter, DFMM},
    log_normal::LogNormal,
    log_normal_solver::LogNormalSolver,
};
use cfmm_math::trading_functions::rmm::{
    compute_l_given_x_rust, compute_price_given_x_rust, compute_x_given_l_rust,
    compute_x_given_price, compute_y_given_l_rust, compute_y_given_x_rust, liq_distribution,
};
use chrono::{DateTime, Utc};
use ethers::types::transaction::eip2718::TypedTransaction;
use serde::{Deserialize, Serialize};
use sim::{from_ethers_u256, to_ethers_address};

use super::*;
use crate::components::chart::{
    coords_to_line_series, CartesianRanges, ChartLineSeries, ChartPoint,
};

pub type EthersAddress = ethers::types::Address;
pub type EthersU256 = ethers::types::U256;
pub type AlloyAddress = alloy_primitives::Address;
pub type AlloyU256 = alloy_primitives::U256;

pub const ALLOY_WAD: AlloyU256 = AlloyU256::from_limbs([1_000_000_000_000_000_000, 0, 0, 0]);

pub enum DataModelError {
    CheckedMul,
    CheckedDiv,
}

impl From<DataModelError> for Error {
    fn from(error: DataModelError) -> Self {
        match error {
            DataModelError::CheckedMul => Error::msg("Checked mul error"),
            DataModelError::CheckedDiv => Error::msg("Checked div error"),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Cached {
    pub asset_token_info: Option<TokenInfo>,
    pub quote_token_info: Option<TokenInfo>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TokenInfo {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
}

/// The model!
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataModel<A, V> {
    pub chain_id: Option<u64>,

    // Cached data is updated only once.
    pub cached: Cached,

    // Must set these addresses.
    pub user_address: Option<A>,
    pub lex_address: Option<A>,
    pub protocol_address: Option<A>,
    pub strategy_address: Option<A>,
    pub solver_address: Option<A>,
    pub asset_token: Option<A>,
    pub quote_token: Option<A>,

    // Balances of tokens.
    pub user_asset_balance: Option<V>,
    pub user_quote_balance: Option<V>,
    pub protocol_asset_balance: Option<V>,
    pub protocol_quote_balance: Option<V>,

    // Values of tokens.
    pub user_asset_value_series: Option<Vec<(u64, V)>>,
    pub user_quote_value_series: Option<Vec<(u64, V)>>,
    pub unallocated_portfolio_value_series: Option<Vec<(u64, V)>>,
    pub protocol_asset_value_series: Option<Vec<(u64, V)>>,
    pub protocol_quote_value_series: Option<Vec<(u64, V)>>,

    // Prices
    pub external_spot_price_series: Option<Vec<(u64, V)>>,
    // seems redundant to have both a series and a single value of the series.
    // What do you think about maybe just having two series one for each market
    pub external_spot_price: Option<V>,
    pub external_quote_price: Option<V>,

    // Protocol state
    pub asset_reserve: Option<V>,
    pub quote_reserve: Option<V>,
    pub total_liquidity: Option<V>,
    pub user_total_liquidity: Option<V>,
    pub internal_spot_price: Option<V>,
    pub internal_spot_price_series: Option<Vec<(u64, V)>>,

    // Strategy state
    pub strike_price_wad: Option<V>,
    pub time_remaining_wad: Option<V>,
    pub volatility_wad: Option<V>,
    pub portfolio_values_series: Option<Vec<(u64, V)>>,

    // Info
    pub latest_timestamp: DateTime<Utc>,
    pub latest_block: u64,

    // User historical transactions
    pub user_historical_transactions: Option<Vec<HistoricalTx>>,
    pub last_historical_transaction_sync_block: u64,
}

sol! {
    #[derive(Debug)]
    interface IERC20 {
        function balanceOf(address account) external view returns(uint balance);
        function name() external view returns(string name);
        function symbol() external view returns(string symbol);
        function decimals() external view returns(uint8 decimals);
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub enum ProtocolActions {
    #[default]
    Empty,
    CreatePosition,
    Swap,
}

impl std::fmt::Display for ProtocolActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            ProtocolActions::Empty => "Empty".to_string(),
            ProtocolActions::CreatePosition => "Create position".to_string(),
            ProtocolActions::Swap => "Swap".to_string(),
        };
        write!(f, "{}", string)
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HistoricalTx {
    pub tx_hash: TxHash,
    pub block_number: u64,
    pub timestamp: DateTime<Utc>,
    pub action: ProtocolActions,
    pub position_name: String,
    pub market_value: f64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StrategyPosition {
    pub balance_x: f64,
    pub balance_y: f64,
    pub liquidity: f64,
    pub external_price: f64,
    pub internal_price: f64,
    pub quote_price: f64,
}

impl StrategyPosition {
    pub fn compute_value(&self) -> Result<f64> {
        Ok(self.balance_x * self.external_price + self.balance_y * self.quote_price)
    }
}

impl DataModel<AlloyAddress, AlloyU256> {
    /// Creates a completely fresh model with no values set.
    pub fn new(chain_id: u64) -> Self {
        Self {
            chain_id: Some(chain_id),
            ..Default::default()
        }
    }

    /// Sets up the model with the required addresses needed to fetch all the
    /// data in the model.
    #[allow(clippy::too_many_arguments)]
    pub fn setup(
        &mut self,
        user_address: AlloyAddress,
        lex_address: AlloyAddress,
        protocol_address: AlloyAddress,
        strategy_address: AlloyAddress,
        solver_address: AlloyAddress,
        asset_token: AlloyAddress,
        quote_token: AlloyAddress,
    ) {
        self.user_address = Some(user_address);
        self.lex_address = Some(lex_address);
        self.protocol_address = Some(protocol_address);
        self.strategy_address = Some(strategy_address);
        self.solver_address = Some(solver_address);
        self.asset_token = Some(asset_token);
        self.quote_token = Some(quote_token);
    }

    /// Updates the ENTIRE model! Wow!
    pub async fn update<M: Middleware + 'static>(&mut self, client: Arc<M>) -> Result<()>
    where
        <M as ethers::providers::Middleware>::Error: 'static,
    {

        // Update sync block + timestamp first, since the other update methods need it.
        // These updates must be successful.
        self.update_last_sync_block(client.clone()).await?;

        // Update state first.
        self.update_token_balances(client.clone()).await?;
        self.update_reserves_and_liquidity(client.clone()).await?;
        self.update_strategy_state(client.clone()).await?;

        // todo: figure out how to handle the calls that should not fail and the ones
        // that can be handled gracefully.
        // Try to update the internal price, which only works if a position exists.
        if let Err(error) = self.update_internal_price(client.clone()).await {
            tracing::warn!("Internal price update failed: {:?}", error);
        }

        // Update prices.
        self.update_external_prices(client.clone()).await?;

        // Update historical tx
        // todo: this is dependent on the external price series!
        if let Err(error) = self.update_historical_txs(client.clone()).await {
            tracing::warn!("Historical tx update failed: {:?}", error);
        }

        // Finally update cached data, which will only update if conditions are met.
        self.update_cached(client.clone()).await?;

        // Update series data.
        self.update_series()?;
        Ok(())
    }

    pub fn update_series(&mut self) -> Result<()> {
        if let Err(error) = self.update_internal_price_series() {
            tracing::warn!("Internal price series update failed: {:?}", error);
        }
        self.update_portfolio_value_series()?;
        self.update_external_price_series()?;
        self.update_user_asset_value_series()?;
        self.update_user_quote_value_series()?;
        self.update_unallocated_portfolio_value_series()?;
        self.update_protocol_asset_value_series()?;
        self.update_protocol_quote_value_series()?;
        Ok(())
    }

    pub async fn fetch_user_historical_tx<M: Middleware + 'static>(
        &self,
        client: Arc<M>,
    ) -> Result<Vec<HistoricalTx>>
    where
        <M as ethers::providers::Middleware>::Error: 'static,
    {
        let current_block = self.fetch_block_number(client.clone()).await?;
        let user_address = match self.user_address {
            Some(address) => to_ethers_address(address),
            None => {
                tracing::error!("User address is not set.");
                return Err(anyhow::anyhow!("User address is not set."));
            }
        };

        let protocol_address = match self.protocol_address {
            Some(address) => to_ethers_address(address),
            None => {
                tracing::error!("Protocol address is not set.");
                return Err(anyhow::anyhow!("Protocol address is not set."));
            }
        };

        let protocol = DFMM::new(protocol_address, client.clone());
        tracing::debug!("Fetching historical tx!");

        let create_pos_filter = protocol
            .init_filter()
            .filter
            .from_block(self.last_historical_transaction_sync_block)
            .to_block(current_block);

        let logs = client.get_logs(&create_pos_filter).await?;
        let filtered_logs = logs
            .into_iter()
            .filter(|log| {
                log.topics.get(1).map(|topic| EthersAddress::from(*topic)) == Some(user_address)
            })
            .collect::<Vec<_>>();

        let mut historical_tx = vec![];

        for log in filtered_logs {
            let block_number = log.block_number.unwrap().as_u64();
            let block_id: BlockId = block_number.into();
            let timestamp = client
                .get_block(block_id)
                .await?
                .unwrap()
                .timestamp
                .as_u64();
            let timestamp = timestamp as i64; // convert u64 to i64
            let naive_datetime = chrono::NaiveDateTime::from_timestamp_opt(timestamp, 0);
            let datetime: DateTime<Utc> =
                DateTime::from_naive_utc_and_offset(naive_datetime.unwrap(), Utc);
            let tx_hash = log.transaction_hash.unwrap();
            let action = ProtocolActions::CreatePosition;

            let parsed_log = protocol.decode_event::<InitFilter>(
                "Init",
                log.topics.clone(),
                log.data.clone(),
            )?;

            let amount_x = EthersU256::from(parsed_log.xxxxxxx);
            let amount_y = EthersU256::from(parsed_log.yyyyyy);

            // Try getting the prices from the series
            let external_x_price = self
                .external_spot_price_series
                .as_ref()
                .and_then(|series| {
                    series
                        .iter()
                        .find(|(block, _)| *block == block_number)
                        .map(|(_, price)| *price)
                })
                .ok_or(Error::msg(format!(
                    "Missing external price for historical tx at block {}",
                    block_number
                )))?;

            // only support usd numeraire for now
            let external_y_price = ALLOY_WAD;

            let amount_x = from_ethers_u256(amount_x);
            let amount_y = from_ethers_u256(amount_y);

            let amount_x = amount_x
                .checked_mul(external_x_price)
                .ok_or(anyhow!(DataModelError::CheckedMul))?
                .checked_div(ALLOY_WAD)
                .ok_or(anyhow!(DataModelError::CheckedDiv))?;

            let amount_y = amount_y
                .checked_mul(external_y_price)
                .ok_or(anyhow!(DataModelError::CheckedMul))?
                .checked_div(ALLOY_WAD)
                .ok_or(anyhow!(DataModelError::CheckedDiv))?;

            let amount_x = Self::format_and_parse(amount_x)?;
            let amount_y = Self::format_and_parse(amount_y)?;

            let market_value = amount_x + amount_y;

            let token_x_symbol = self
                .cached
                .asset_token_info
                .as_ref()
                .ok_or(Error::msg("Asset token info not set"))?
                .symbol
                .clone();

            let token_y_symbol = self
                .cached
                .quote_token_info
                .as_ref()
                .ok_or(Error::msg("Quote token info not set"))?
                .symbol
                .clone();

            let position_name = format!("{} / {}", token_x_symbol, token_y_symbol);

            historical_tx.push(HistoricalTx {
                tx_hash,
                block_number,
                timestamp: datetime,
                action,
                position_name,
                market_value,
            });
        }

        Ok(historical_tx)
    }

    pub async fn update_cached<M: Middleware + 'static>(&mut self, client: Arc<M>) -> Result<()>
    where
        <M as ethers::providers::Middleware>::Error: 'static,
    {
        // Only update token info if cache is not set.
        if self.cached.asset_token_info.is_none() || self.cached.quote_token_info.is_none() {
            self.update_token_info(client.clone()).await?;
        }
        Ok(())
    }

    pub async fn update_historical_txs<M: Middleware + 'static>(
        &mut self,
        client: Arc<M>,
    ) -> Result<()>
    where
        <M as ethers::providers::Middleware>::Error: 'static,
    {
        let historical_txs = self.fetch_user_historical_tx(client.clone()).await?;
        self.user_historical_transactions = Some(historical_txs);
        self.last_historical_transaction_sync_block = self.latest_block;
        Ok(())
    }

    pub async fn update_token_info<M: Middleware + 'static>(&mut self, client: Arc<M>) -> Result<()>
    where
        <M as ethers::providers::Middleware>::Error: 'static,
    {
        let asset_token_info = self.cached.asset_token_info.clone();
        let quote_token_info = self.cached.quote_token_info.clone();

        if asset_token_info.is_none() {
            if let Some(asset_token) = self.asset_token {
                let asset_token_info = self.fetch_token_info(client.clone(), asset_token).await?;
                self.cached.asset_token_info = Some(asset_token_info);
            } else {
                return Err(anyhow::anyhow!("Asset token address is not set."));
            }
        }

        if quote_token_info.is_none() {
            if let Some(quote_token) = self.quote_token {
                let quote_token_info = self.fetch_token_info(client.clone(), quote_token).await?;
                self.cached.quote_token_info = Some(quote_token_info);
            } else {
                return Err(anyhow::anyhow!("Quote token address is not set."));
            }
        }

        Ok(())
    }

    pub async fn fetch_token_info<M: Middleware + 'static>(
        &self,
        client: Arc<M>,
        token_address: AlloyAddress,
    ) -> Result<TokenInfo>
    where
        <M as ethers::providers::Middleware>::Error: 'static,
    {
        let converted_token_address = to_ethers_address(token_address);

        let payload = IERC20::nameCall {};
        let payload = ethers::types::Bytes::from(payload.abi_encode());

        let mut tx = TypedTransaction::default();
        tx.set_to(converted_token_address).set_data(payload);

        // Send the call to the token contract.
        let name = client.call(&tx, None).await?;
        let decoded: <IERC20::nameCall as SolCall>::Return =
            IERC20::nameCall::abi_decode_returns(&name, false)?;
        let name = decoded.name;

        let payload = IERC20::symbolCall {};
        let payload = ethers::types::Bytes::from(payload.abi_encode());

        let mut tx = TypedTransaction::default();
        tx.set_to(converted_token_address).set_data(payload);

        // Send the call to the token contract.
        let symbol = client.call(&tx, None).await?;
        let decoded: <IERC20::symbolCall as SolCall>::Return =
            IERC20::symbolCall::abi_decode_returns(&symbol, false)?;
        let symbol = decoded.symbol;

        let payload = IERC20::decimalsCall {};
        let payload = ethers::types::Bytes::from(payload.abi_encode());

        let mut tx = TypedTransaction::default();
        tx.set_to(converted_token_address).set_data(payload);

        // Send the call to the token contract.
        let decimals = client.call(&tx, None).await?;
        let decoded: <IERC20::decimalsCall as SolCall>::Return =
            IERC20::decimalsCall::abi_decode_returns(&decimals, false)?;
        let decimals = decoded.decimals;

        Ok(TokenInfo {
            name,
            symbol,
            decimals,
        })
    }

    pub fn update_last_sync_timestamp(&mut self) -> Result<()> {
        self.latest_timestamp = Utc::now();
        Ok(())
    }

    /// Gets the protocol contract instance given the model's protocol address.
    pub async fn dfmm(&self, client: Arc<Client>) -> Result<DFMM<Client>> {
        let protocol_address = self
            .protocol_address
            .ok_or(anyhow!("Protocol address is not set"))?;
        let converted_address = to_ethers_address(protocol_address);
        let dfmm = DFMM::new(converted_address, client.clone());
        Ok(dfmm)
    }

    /// Gets the strategy contract instance given the model's strategy address.
    pub async fn strategy<M: Middleware + 'static>(&self, client: Arc<M>) -> Result<LogNormal<M>> {
        let strategy_address = self
            .strategy_address
            .ok_or(Error::msg("Strategy address not set"))?;
        let converted_address = to_ethers_address(strategy_address);
        let strategy = LogNormal::new(converted_address, client.clone());
        Ok(strategy)
    }

    /// Gets the strategy contract instance given the model's strategy address.
    pub async fn solver<M: Middleware + 'static>(
        &self,
        client: Arc<M>,
    ) -> Result<LogNormalSolver<M>> {
        let solver_address = self
            .solver_address
            .ok_or(Error::msg("Solver address not set"))?;
        let converted_address = to_ethers_address(solver_address);
        let solver = LogNormalSolver::new(converted_address, client.clone());
        Ok(solver)
    }

    /// Gets the "unallocated position" balances.
    pub fn format_and_parse(value: AlloyU256) -> Result<f64> {
        let formatted = alloy_primitives::utils::format_ether(value);
        formatted.parse::<f64>().map_err(|e| e.into())
    }

    pub fn get_unallocated_positions_info(&self) -> Result<StrategyPosition> {
        let quote_price =
            self.fetch_and_format(self.external_quote_price, "external_quote_price is None")?;
        let external_price =
            self.fetch_and_format(self.external_spot_price, "external_spot_price is None")?;
        let balance_x =
            self.fetch_and_format(self.user_asset_balance, "user_asset_balance is None")?;
        let balance_y =
            self.fetch_and_format(self.user_quote_balance, "user_quote_balance is None")?;

        Ok(StrategyPosition {
            balance_x,
            balance_y,
            liquidity: 0.0,
            external_price,
            internal_price: 0.0,
            quote_price,
        })
    }

    fn fetch_and_format(&self, value: Option<AlloyU256>, error_msg: &str) -> Result<f64> {
        let value = value.ok_or(anyhow!(error_msg.to_owned()))?;
        Self::format_and_parse(value)
    }
    /// Gets the balances and prices of the asset and quote tokens and formats
    /// them into floats.
    pub fn get_position_info(&self) -> Result<StrategyPosition> {
        let external_price =
            self.fetch_and_format(self.external_spot_price, "external_spot_price is None")?;
        let balance_x =
            self.fetch_and_format(self.user_asset_balance, "user_asset_balance is None")?;
        let balance_y =
            self.fetch_and_format(self.user_quote_balance, "user_quote_balance is None")?;
        let liquidity =
            self.fetch_and_format(self.user_total_liquidity, "user_total_liquidity is None")?;
        let internal_price =
            self.fetch_and_format(self.internal_spot_price, "internal_spot_price is None")?;
        let quote_price =
            self.fetch_and_format(self.external_quote_price, "external_quote_price is None")?;

        Ok(StrategyPosition {
            balance_x,
            balance_y,
            liquidity,
            external_price,
            internal_price,
            quote_price,
        })
    }

    // Provider

    pub async fn fetch_block_number<M: Middleware + 'static>(&self, client: Arc<M>) -> Result<u64>
    where
        <M as ethers::providers::Middleware>::Error: 'static,
    {
        let block_number = client.get_block_number().await?;
        Ok(block_number.as_u64())
    }

    pub async fn update_last_sync_block<M: Middleware + 'static>(
        &mut self,
        client: Arc<M>,
    ) -> Result<()> {
        self.latest_block = self.fetch_block_number(client.clone()).await?;
        self.update_last_sync_timestamp()?;
        Ok(())
    }

    /// Fetches the ether balance of an address.
    pub async fn fetch_balance<M: Middleware + 'static>(
        &self,
        client: Arc<M>,
        address: AlloyAddress,
    ) -> Result<AlloyU256>
    where
        <M as ethers::providers::Middleware>::Error: 'static,
    {
        let converted_address = to_ethers_address(address);
        let balance = client.get_balance(converted_address, None).await?;
        let converted_balance = from_ethers_u256(balance);
        Ok(converted_balance)
    }

    // Tokens

    /// Fetches the balance of tokens of a given address for a given token.
    #[tracing::instrument(skip(client), level = "trace")]
    pub async fn fetch_balance_of<M: Middleware + 'static>(
        &self,
        client: Arc<M>,
        token_address: AlloyAddress,
        address: AlloyAddress,
    ) -> Result<AlloyU256>
    where
        <M as ethers::providers::Middleware>::Error: 'static,
    {
        let converted_token_address = to_ethers_address(token_address);

        let token = arbiter_bindings::bindings::arbiter_token::ArbiterToken::new(
            converted_token_address,
            client.clone(),
        );
        // Send the call to the token contract.
        let balance = token.balance_of(address).await?;
        let decoded_balance: AlloyU256 = balance;
        Ok(decoded_balance)
    }

    // Token balances

    async fn fetch_user_asset_balance<M: Middleware + 'static>(
        &self,
        client: Arc<M>,
        address: AlloyAddress,
    ) -> Result<AlloyU256>
    where
        <M as ethers::providers::Middleware>::Error: 'static,
    {
        let asset_token = self
            .asset_token
            .ok_or(Error::msg("Asset token not set"))?;
        self.fetch_balance_of(client, asset_token, address).await
    }

    async fn fetch_user_quote_balance<M: Middleware + 'static>(
        &self,
        client: Arc<M>,
        address: AlloyAddress,
    ) -> Result<AlloyU256>
    where
        <M as ethers::providers::Middleware>::Error: 'static,
    {
        let quote_token = self
            .quote_token
            .ok_or(Error::msg("Quote token not set"))?;
        self.fetch_balance_of(client, quote_token, address).await
    }

    async fn fetch_protocol_asset_balance<M: Middleware + 'static>(
        &self,
        client: Arc<M>,
    ) -> Result<AlloyU256>
    where
        <M as ethers::providers::Middleware>::Error: 'static,
    {
        let asset_token = self
            .asset_token
            .ok_or(Error::msg("Asset token not set"))?;
        let protocol = self
            .protocol_address
            .ok_or(Error::msg("Protocol address not set"))?;
        self.fetch_balance_of(client, asset_token, protocol).await
    }

    async fn fetch_protocol_quote_balance<M: Middleware + 'static>(
        &self,
        client: Arc<M>,
    ) -> Result<AlloyU256>
    where
        <M as ethers::providers::Middleware>::Error: 'static,
    {
        let quote_token = self
            .quote_token
            .ok_or(Error::msg("Quote token not set"))?;
        let protocol = self
            .protocol_address
            .ok_or(Error::msg("Protocol address not set"))?;
        self.fetch_balance_of(client, quote_token, protocol).await;
        let token =
            bindings::arbiter_token::ArbiterToken::new(converted_token_address, client.clone());
        let balance = token.balance_of(converted_token_address).await?;
        let alloy = from_ethers_u256(balance);
        Ok(alloy)
    }

    async fn fetch_external_price<M: Middleware + 'static>(
        &self,
        client: Arc<M>,
    ) -> Result<AlloyU256> {
        let lex = self
            .lex_address
            .ok_or(Error::msg("External exchange address not set"))?;

        let lex = arbiter_bindings::bindings::liquid_exchange::LiquidExchange::new(
            to_ethers_address(lex_address),
            client.clone(),
        );
        let price = lex.price().await;
        let price = match price {
            Ok(price) => price,
            Err(error) => {
                tracing::warn!("Error fetching external price: {:?}", error);
                return Err(anyhow!("Error fetching external price"));
            }
        };
        let price = from_ethers_u256(price);

        Ok(price)
    }

    // Dfmm state
    async fn fetch_reserves_and_liquidity<M: Middleware + 'static>(
        &self,
        client: Arc<M>,
    ) -> Result<(AlloyU256, AlloyU256, AlloyU256)> {
        let dfmm = self.dfmm(client.clone()).await?;
        let result = dfmm.get_reserves_and_liquidity().await;
        let (reserve_x, reserve_y, liquidity) = match result {
            Ok(result) => result,
            Err(error) => {
                tracing::warn!("Error fetching reserves and liquidity: {:?}", error);
                return Err(anyhow!("Error fetching reserves and liquidity"));
            }
        };
        let reserve_x = from_ethers_u256(reserve_x);
        let reserve_y = from_ethers_u256(reserve_y);
        let liquidity = from_ethers_u256(liquidity);
        Ok((reserve_x, reserve_y, liquidity))
    }

    async fn fetch_internal_price<M: Middleware + 'static>(
        &self,
        client: Arc<M>,
    ) -> Result<AlloyU256> {
        let solver = self.solver(client.clone()).await?;
        let internal_price = solver.internal_price().await;
        let internal_price = match internal_price {
            Ok(internal_price) => internal_price,
            Err(error) => {
                tracing::warn!("Error fetching internal price: {:?}", error);
                return Err(anyhow!("Error fetching internal price"));
            }
        };

        let internal_price = from_ethers_u256(internal_price);
        Ok(internal_price)
    }

    async fn fetch_strategy_params<M: Middleware + 'static>(
        &self,
        client: Arc<M>,
    ) -> Result<(AlloyU256, AlloyU256, AlloyU256)> {
        let strategy = self.strategy(client.clone()).await?;
        let (strike_price, volatility, time_remaining) = strategy.get_params().await?;
        let strike_price = from_ethers_u256(strike_price);
        let volatility = from_ethers_u256(volatility);
        let time_remaining = from_ethers_u256(time_remaining);
        Ok((strike_price, volatility, time_remaining))
    }

    async fn update_reserves_and_liquidity<M: Middleware + 'static>(
        &mut self,
        client: Arc<M>,
    ) -> Result<()> {
        let (reserve_x, reserve_y, liquidity) =
            self.fetch_reserves_and_liquidity(client.clone()).await?;

        self.asset_reserve = Some(reserve_x);
        self.quote_reserve = Some(reserve_y);
        self.total_liquidity = Some(liquidity);

        Ok(())
    }

    async fn update_internal_price<M: Middleware + 'static>(
        &mut self,
        client: Arc<M>,
    ) -> Result<()> {
        let internal_price = self.fetch_internal_price(client.clone()).await?;

        self.internal_spot_price = Some(internal_price);

        Ok(())
    }

    async fn update_external_prices<M: Middleware + 'static>(
        &mut self,
        client: Arc<M>,
    ) -> Result<()> {
        // TODO: fix this, essentially the price function in the lex just gives in terms
        // of 1 but we should have it get both. we would haave to do this in the
        // `fetch_external_price` function
        let asset_price = self.fetch_external_price(client.clone()).await?;
        // only support usd numerier for now
        let quote_price = ALLOY_WAD;
        self.external_spot_price = Some(asset_price);
        self.external_quote_price = Some(quote_price);

        Ok(())
    }

    /// Checks the current block number and updates the portfolio value series
    /// if the current block number is greater than the last block number.
    fn update_portfolio_value_series(&mut self) -> Result<()> {
        // Only update the series if the last element in the series is behind the
        // current block number.
        let default = (0u64, self.derive_external_portfolio_value());
        let last_element = self
            .portfolio_values_series
            .as_ref()
            .map_or(&default, |v| v.last().unwrap_or(&default));
        if last_element.0 >= self.latest_block {
            return Ok(());
        }
        let portfolio_value = self.derive_external_portfolio_value();
        self.portfolio_values_series
            .get_or_insert(Vec::new())
            .push((self.latest_block, portfolio_value));
        Ok(())
    }

    fn update_unallocated_portfolio_value_series(&mut self) -> Result<()> {
        let default = (0u64, self.derive_unallocated_position_value()?);
        let last_element = &self
            .unallocated_portfolio_value_series
            .as_ref()
            .map_or(&default, |v| v.last().unwrap_or(&default));
        if last_element.0 >= self.latest_block {
            return Ok(());
        }
        let portfolio_value = self.derive_unallocated_position_value()?;
        self.unallocated_portfolio_value_series
            .get_or_insert(Vec::new())
            .push((self.latest_block, portfolio_value));
        Ok(())
    }

    fn update_external_price_series(
        &mut self,
    ) -> Result<()>
    {
        if let Some(series) = &self.external_spot_price_series {
            let last_element = series.last().ok_or(Error::msg("Last element not set"))?;
            if last_element.0 > self.latest_block {
                return Ok(());
            }
        }
        if let Some(external_spot_price) = self.external_spot_price {
            self.external_spot_price_series
                .get_or_insert(Vec::new())
                .push((self.latest_block, external_spot_price));
            tracing::debug!(
                "Added external price at block: {:?} {:?}",
                self.latest_block,
                external_spot_price
            );
        }
        Ok(())
    }

    fn update_user_asset_value_series(
        &mut self,
    ) -> Result<()>

    {
        if let Some(series) = &self.user_asset_value_series {
            let last_element = series.last().ok_or(Error::msg("Last element not set"))?;
            if last_element.0 >= self.latest_block {
                return Ok(());
            }
        }
        let user_asset_balance = self
            .user_asset_balance
            .ok_or(anyhow!("User asset balance is None"))?;
        let asset_value = user_asset_balance
            .checked_mul(
                self.external_spot_price
                    .ok_or(anyhow!("External spot price is None"))?,
            )
            .ok_or(anyhow!(DataModelError::CheckedMul))?
            .checked_div(ALLOY_WAD)
            .ok_or(anyhow!(DataModelError::CheckedDiv))?;
        self.user_asset_value_series
            .get_or_insert(Vec::new())
            .push((self.latest_block, asset_value));

        Ok(())
    }

    fn update_user_quote_value_series(
        &mut self,
    ) -> Result<()>
    {
        if let Some(series) = &self.user_quote_value_series {
            let last_element = series.last().ok_or(Error::msg("Last element not set"))?;
            if last_element.0 >= self.latest_block {
                return Ok(());
            }
        }

        let user_quote_balance = self
            .user_quote_balance
            .ok_or(anyhow!("User quote balance is None"))?;
        let quote_value = user_quote_balance
            .checked_mul(
                self.external_quote_price
                    .ok_or(anyhow!("External quote price is None"))?,
            )
            .ok_or(anyhow!(DataModelError::CheckedMul))?
            .checked_div(ALLOY_WAD)
            .ok_or(anyhow!(DataModelError::CheckedDiv))?;

        self.user_quote_value_series
            .get_or_insert(Vec::new())
            .push((self.latest_block, quote_value));
        Ok(())
    }

    fn update_protocol_asset_value_series(
        &mut self) -> Result<()>
    {
        if let Some(series) = &self.protocol_asset_value_series {
            let last_element = series.last().ok_or(Error::msg("Last element not set"))?;
            if last_element.0 >= self.latest_block {
                return Ok(());
            }
        }
        let protocol_asset_balance = self
            .protocol_asset_balance
            .ok_or(anyhow!("Protocol asset balance is None"))?;
        let asset_value = protocol_asset_balance
            .checked_mul(
                self.external_spot_price
                    .ok_or(anyhow!("External spot price is None"))?,
            )
            .ok_or(anyhow!(DataModelError::CheckedMul))?
            .checked_div(ALLOY_WAD)
            .ok_or(anyhow!(DataModelError::CheckedDiv))?;
        self.protocol_asset_value_series
            .get_or_insert(Vec::new())
            .push((self.latest_block, asset_value));

        Ok(())
    }

    fn update_protocol_quote_value_series(
        &mut self) -> Result<()> {
        if let Some(series) = &self.protocol_quote_value_series {
            let last_element = series.last().ok_or(Error::msg("Last element not set"))?;
            if last_element.0 >= self.latest_block {
                return Ok(());
            }
        }
        let protocol_quote_balance = self
            .protocol_quote_balance
            .ok_or(anyhow!("Protocol quote balance is None"))?;
        let quote_value = protocol_quote_balance
            .checked_mul(
                self.external_quote_price
                    .ok_or(anyhow!("External quote price is None"))?,
            )
            .ok_or(anyhow!(DataModelError::CheckedMul))?
            .checked_div(ALLOY_WAD)
            .ok_or(anyhow!(DataModelError::CheckedDiv))?;

        self.protocol_quote_value_series
            .get_or_insert(Vec::new())
            .push((self.latest_block, quote_value));
        Ok(())
    }

    fn update_internal_price_series(&mut self) -> Result<()> {
        if let Some(internal_spot_price) = self.internal_spot_price {
            let default = (0u64, internal_spot_price);
            let last_element = self
                .internal_spot_price_series
                .as_ref()
                .map_or(&default, |v| v.last().unwrap_or(&default));
            if last_element.0 < self.latest_block {
                if let Some(internal_spot_price_series) = &mut self.internal_spot_price_series {
                    internal_spot_price_series.push((self.latest_block, internal_spot_price));
                }
            }
            Ok(())
        } else {
            Err(anyhow!("Internal spot price is None"))
        }
    }
    async fn update_token_balances(&mut self, client: Arc<Client>) -> Result<()> {
        let default = (
            0u64,
            self.user_asset_balance
                .ok_or(anyhow!("User asset balance is None"))?,
        );
        let last_element = self
            .user_asset_value_series
            .as_ref()
            .map_or(&default, |v| v.last().unwrap_or(&default));
        if last_element.0 >= self.latest_block {
            return Ok(());
        }
        let user_asset_balance = self
            .fetch_balance_of(
                client.clone(),
                self.asset_token.ok_or(anyhow!("Asset token is None"))?,
                self.user_address.ok_or(anyhow!("User address is None"))?,
            )
            .await?;
        let user_quote_balance = self
            .fetch_balance_of(
                client.clone(),
                self.quote_token.ok_or(anyhow!("Quote token is None"))?,
                self.user_address.ok_or(anyhow!("User address is None"))?,
            )
            .await?;
        let protocol_asset_balance = self
            .fetch_balance_of(
                client.clone(),
                self.asset_token.ok_or(anyhow!("Asset token is None"))?,
                self.protocol_address
                    .ok_or(anyhow!("Protocol address is None"))?,
            )
            .await?;
        let protocol_quote_balance = self
            .fetch_balance_of(
                client.clone(),
                self.quote_token.ok_or(anyhow!("Quote token is None"))?,
                self.protocol_address
                    .ok_or(anyhow!("Protocol address is None"))?,
            )
            .await?;

        self.user_asset_balance = Some(user_asset_balance);
        self.user_quote_balance = Some(user_quote_balance);
        self.protocol_asset_balance = Some(protocol_asset_balance);
        self.protocol_quote_balance = Some(protocol_quote_balance);

        Ok(())
    }

    async fn update_strategy_state<M: Middleware + 'static>(
        &mut self,
        client: Arc<M>,
    ) -> Result<()> {
        let (strike_price, volatility, time_remaining) =
            self.fetch_strategy_params(client.clone()).await?;

        self.strike_price_wad = Some(strike_price);
        self.volatility_wad = Some(volatility);
        self.time_remaining_wad = Some(time_remaining);

        Ok(())
    }

    /// Computes the sum of a dual asset portfolio given their balances and
    /// prices. Prices are assumed to be denominated in USD-pegged assets, and
    /// in WAD units.
    #[tracing::instrument(level = "trace")]
    pub fn compute_portfolio_value_real(
        asset_price_wad: AlloyU256,
        quote_price_wad: AlloyU256,
        quote_balance_wad: AlloyU256,
        asset_balance_wad: AlloyU256,
    ) -> Result<AlloyU256> {
        let asset_value = asset_balance_wad
            .checked_mul(asset_price_wad)
            .ok_or(anyhow!(DataModelError::CheckedMul))?
            .checked_div(ALLOY_WAD)
            .ok_or(anyhow!(DataModelError::CheckedDiv))?;

        let quote_value = quote_balance_wad
            .checked_mul(quote_price_wad)
            .ok_or(anyhow!(DataModelError::CheckedMul))?
            .checked_div(ALLOY_WAD)
            .ok_or(anyhow!(DataModelError::CheckedDiv))?;

        let portfolio_value = asset_value.checked_add(quote_value).ok_or(anyhow!(
            "Failed to add asset value and quote value: {} + {}",
            asset_value,
            quote_value
        ))?;

        Ok(portfolio_value)
    }

    /// Computes the theoretical portfolio value of a given strategy.
    pub fn compute_portfolio_value_theoretical(
        asset_price_wad: AlloyU256,
        quote_price_wad: AlloyU256,
        total_liquidity_wad: AlloyU256,
        strike_price_wad: AlloyU256,
        volatility_percent_wad: AlloyU256,
        time_remaining_years_wad: AlloyU256,
    ) -> Result<AlloyU256> {
        let quote_price = Self::format_and_parse(quote_price_wad)?;
        let price = Self::format_and_parse(asset_price_wad)?;
        let liquidity = Self::format_and_parse(total_liquidity_wad)?;
        let strike_price = Self::format_and_parse(strike_price_wad)?;
        let volatility = Self::format_and_parse(volatility_percent_wad)?;
        let time_remaining = Self::format_and_parse(time_remaining_years_wad)?;

        // Get x given price, then l given x, then y given l.
        let x = compute_x_given_l_rust(liquidity, price, strike_price, volatility, time_remaining);
        let l = compute_l_given_x_rust(x, price, strike_price, volatility, time_remaining);
        let y = compute_y_given_l_rust(l, price, strike_price, volatility, time_remaining);

        let portfolio_value = y * quote_price + x * price;
        let portfolio_value = format!("{}", portfolio_value);
        let portfolio_value = alloy_primitives::utils::parse_ether(&portfolio_value)?;

        Ok(portfolio_value)
    }

    /// Sum of external portfolio value (allocated positions) and unallocated
    /// positions' value.
    pub fn derive_total_aum(&self) -> Result<AlloyU256> {
        // todo: this naming is confusing but the external portfolio value is the
        // unallocated value.
        let external_portfolio_value = self.derive_external_portfolio_value();
        let unallocated_position_value = self.derive_unallocated_position_value()?;

        let total_aum = external_portfolio_value
            .checked_add(unallocated_position_value)
            .ok_or(anyhow!(
                "Failed to add external portfolio value and unallocated position value: {} + {}",
                external_portfolio_value,
                unallocated_position_value
            ))?;

        Ok(total_aum)
    }

    /// Computes the value of the unallocated positions.
    pub fn derive_unallocated_position_value(&self) -> Result<AlloyU256> {
        let unallocated_position = self.get_unallocated_positions_info()?;
        let unallocated_position_value = unallocated_position.compute_value()?;
        let unallocated_position_value =
            alloy_primitives::utils::parse_ether(&format!("{}", unallocated_position_value))?;
        Ok(unallocated_position_value)
    }

    /// Computes the portfolio value of the user's strategy deposits according
    /// to an external price.
    pub fn derive_external_portfolio_value(&self) -> AlloyU256 {
        let asset_price_wad = self.external_spot_price.unwrap_or_default();
        let quote_price_wad = self.external_quote_price.unwrap_or_default();
        let quote_reserve_wad = self.quote_reserve.unwrap_or_default();
        let asset_reserve_wad = self.asset_reserve.unwrap_or_default();

        Self::compute_portfolio_value_real(
            asset_price_wad,
            quote_price_wad,
            quote_reserve_wad,
            asset_reserve_wad,
        )
        .unwrap()
    }

    /// Computes the portfolio value of the user's deposits in a strategy
    /// according to the internal price.
    pub fn derive_internal_portfolio_value(&self) -> Result<AlloyU256> {
        let asset_price_wad = self.internal_spot_price.unwrap_or_default();
        let quote_price_wad = self.external_quote_price.unwrap_or_default();

        // todo: using the global reserves of the market for now.
        let quote_balance_wad = self.quote_reserve.unwrap_or_default();
        let asset_balance_wad = self.asset_reserve.unwrap_or_default();

        Self::compute_portfolio_value_real(
            asset_price_wad,
            quote_price_wad,
            quote_balance_wad,
            asset_balance_wad,
        )
    }

    /// Computes the theoretical portfolio value given the strategy parameters,
    /// external market price, and amount of liquidity.
    pub fn derive_theoretical_portfolio_value(&self) -> Result<AlloyU256> {
        Self::compute_portfolio_value_theoretical(
            self.external_spot_price.unwrap_or_default(),
            self.external_quote_price.unwrap_or_default(),
            self.total_liquidity.unwrap_or_default(),
            self.strike_price_wad.unwrap_or_default(),
            self.volatility_wad.unwrap_or_default(),
            self.time_remaining_wad.unwrap_or_default(),
        )
    }

    /// Computes the replication health of the deposited portfolio.
    /// Health = (Internal Portfolio Value) / (Theoretical Portfolio Value)
    pub fn compute_health(
        internal_portfolio_value_wad: AlloyU256,
        theoretical_value_wad: AlloyU256,
    ) -> Result<AlloyU256> {
        let health = internal_portfolio_value_wad
            .checked_mul(ALLOY_WAD)
            .ok_or(anyhow!(DataModelError::CheckedMul))?
            .checked_div(theoretical_value_wad)
            .ok_or(anyhow!(DataModelError::CheckedDiv))?;

        Ok(health)
    }

    /// Computes the health of the user's portfolio.
    pub fn derive_portfolio_health(&self) -> Result<AlloyU256> {
        let internal_portfolio_value_wad = self.derive_internal_portfolio_value()?;
        let theoretical_value_wad = self.derive_theoretical_portfolio_value()?;

        Self::compute_health(internal_portfolio_value_wad, theoretical_value_wad)
    }

    /// Transforms series data in native types to chart types.
    pub fn transform_series_over_block_number(
        series: &[(u64, AlloyU256)],
    ) -> Result<(CartesianRanges, ChartLineSeries)> {
        let mut transformed = Vec::new();

        if series.is_empty() {
            return Ok((
                CartesianRanges {
                    x_range: (0.0, 1.0),
                    y_range: (0.0, 1.0),
                },
                ChartLineSeries::default(),
            ));
        }

        for (block, value) in series {
            let block = *block as f32;
            let value = Self::format_and_parse(*value)?;
            transformed.push((block, value));
        }
        // Get the ranges, which should be within 20% of the last value.
        let first_block = transformed.first().unwrap().0;
        let last_value = transformed.last().unwrap().1;

        let min_y = (last_value - last_value * 0.2) as f32;
        let max_y = (last_value + last_value * 0.2) as f32;

        let max_y = if max_y <= f32::EPSILON { 1.0 } else { max_y };

        let ranges = CartesianRanges {
            x_range: (first_block, first_block + 10.0),
            y_range: (min_y, max_y),
        };
        let transformed: Vec<(_, f32)> = transformed
            .into_iter()
            .map(|(x, y)| (x, y as f32))
            .collect();

        Ok((ranges, coords_to_line_series(transformed)))
    }

    /// Transforms x,y data in native types to chart types.
    pub fn transform_plot(data: &[(AlloyU256, AlloyU256)]) -> Result<ChartLineSeries> {
        let mut transformed = Vec::new();

        for (x, y) in data {
            let x = Self::format_and_parse(*x)? as f32;
            let y = Self::format_and_parse(*y)? as f32;
            transformed.push((x, y));
        }

        Ok(coords_to_line_series(transformed))
    }

    /// Transforms the portfolio value series into a chart series that can be
    /// plotted by the view logic.
    pub fn derive_portfolio_value_series(&self) -> Result<(CartesianRanges, ChartLineSeries)> {
        let portfolio_values_series = self
            .portfolio_values_series
            .clone()
            .unwrap_or_default();
        let mut result = Self::transform_series_over_block_number(&portfolio_values_series)?;

        result.1.legend = "Portfolio Value".to_string();
        result.1.color = plotters::style::full_palette::DEEPPURPLE_400;
        result.1.time_series = true;

        Ok(result)
    }

    /// Gets the time series data for the unallocated portfolio value.
    pub fn derive_unallocated_portfolio_value_series(
        &self,
    ) -> Result<(CartesianRanges, ChartLineSeries)> {
        let unallocated_portfolio_value_series = self
            .unallocated_portfolio_value_series
            .clone()
            .unwrap_or_default();
        let mut result =
            Self::transform_series_over_block_number(&unallocated_portfolio_value_series)?;

        result.1.legend = "Unallocated".to_string();
        result.1.color = plotters::style::full_palette::LIGHTBLUE_A400;
        result.1.time_series = true;

        Ok(result)
    }

    /// Gets the time series data for the protocol asset value series.
    pub fn derive_protocol_asset_value_series(&self) -> Result<(CartesianRanges, ChartLineSeries)> {
        let protocol_asset_value_series = self
            .protocol_asset_value_series
            .clone()
            .unwrap_or_default();
        let mut result = Self::transform_series_over_block_number(&protocol_asset_value_series)?;

        result.1.legend = "Protocol Asset".to_string();
        result.1.color = plotters::style::full_palette::PURPLE_A700;
        result.1.time_series = true;

        Ok(result)
    }

    /// Gets the time series data for the protocol quote value series.
    pub fn derive_protocol_quote_value_series(&self) -> Result<(CartesianRanges, ChartLineSeries)> {
        let protocol_quote_value_series = self
            .protocol_quote_value_series
            .clone()
            .unwrap_or_default();
        let mut result = Self::transform_series_over_block_number(&protocol_quote_value_series)?;

        result.1.legend = "Protocol Quote".to_string();
        result.1.color = plotters::style::full_palette::PURPLE_A400;
        result.1.time_series = true;

        Ok(result)
    }

    /// Gets the points of interest on the strategy plot.
    pub fn derive_portfolio_strategy_points(&self) -> Result<Vec<ChartPoint>> {
        let asset_reserve =
            Self::format_and_parse(self.asset_reserve.ok_or(anyhow!("Asset reserve is None"))?)?;
        let quote_reserve =
            Self::format_and_parse(self.quote_reserve.ok_or(anyhow!("Quote reserve is None"))?)?;
        let total_liquidity = Self::format_and_parse(
            self.total_liquidity
                .ok_or(anyhow!("Total liquidity is None"))?,
        )?;

        // todo: handle better!
        if total_liquidity == 0.0 {
            return Err(anyhow!("Total liquidity is 0"));
        }

        let internal_reserves = (
            (asset_reserve / total_liquidity) as f32,
            (quote_reserve / total_liquidity) as f32,
        );

        // Compute the theoretical reserves by using the price to find the x and y.
        let spot_price_float = Self::format_and_parse(
            self.external_spot_price
                .ok_or(anyhow!("Spot price is None"))?,
        )?;
        let strike_price_wad_float = Self::format_and_parse(
            self.strike_price_wad
                .ok_or(anyhow!("Strike price is None"))?,
        )?;

        let sigma_percent_wad_float =
            Self::format_and_parse(self.volatility_wad.ok_or(anyhow!("Volatility is None"))?)?;
        let time_to_expiry_years_wad_float = Self::format_and_parse(
            self.time_remaining_wad
                .ok_or(anyhow!("Time remaining is None"))?,
        )?;

        let mut points: Vec<ChartPoint> = vec![];

        let theoretical_asset_reserve = compute_x_given_price(
            spot_price_float,
            total_liquidity,
            strike_price_wad_float,
            sigma_percent_wad_float,
            time_to_expiry_years_wad_float,
        );

        let theoretical_quote_reserve = compute_y_given_x_rust(
            theoretical_asset_reserve,
            total_liquidity,
            strike_price_wad_float,
            sigma_percent_wad_float,
            time_to_expiry_years_wad_float,
        );

        let theoretical_reserves = (
            (theoretical_asset_reserve / total_liquidity) as f32,
            (theoretical_quote_reserve / total_liquidity) as f32,
        );

        let theoretical_reserves = ChartPoint {
            x: theoretical_reserves.0,
            y: theoretical_reserves.1,
            color: plotters::style::full_palette::DEEPORANGE_A400,
            ..Default::default()
        };
        points.push(theoretical_reserves);

        let theoretical_price = compute_price_given_x_rust(
            theoretical_reserves.x as f64,
            1.0,
            strike_price_wad_float,
            sigma_percent_wad_float,
            time_to_expiry_years_wad_float,
        );

        let theoretical_price = ChartPoint {
            x: theoretical_reserves.x,
            y: theoretical_price as f32,
            color: plotters::style::full_palette::DEEPORANGE_A400,
            ..Default::default()
        };
        points.push(theoretical_price);

        let theoretical_liq_dist = liq_distribution(
            theoretical_reserves.x as f64,
            1.0,
            strike_price_wad_float,
            sigma_percent_wad_float,
            time_to_expiry_years_wad_float,
        ) as f32;

        let theoretical_liq_dist = ChartPoint {
            x: theoretical_reserves.x,
            y: theoretical_liq_dist,
            color: plotters::style::full_palette::DEEPORANGE_A400,
            ..Default::default()
        };
        points.push(theoretical_liq_dist);

        if internal_reserves.0 > 0.0 {
            let internal_reserves = ChartPoint {
                x: internal_reserves.0,
                y: internal_reserves.1,
                color: plotters::style::full_palette::DEEPPURPLE_A400,
                ..Default::default()
            };
            points.push(internal_reserves);

            let internal_liq_dist = liq_distribution(
                internal_reserves.x as f64,
                1.0,
                strike_price_wad_float,
                sigma_percent_wad_float,
                time_to_expiry_years_wad_float,
            ) as f32;

            let internal_liq_dist = ChartPoint {
                x: internal_reserves.x,
                y: internal_liq_dist,
                color: plotters::style::full_palette::DEEPPURPLE_A400,
                ..Default::default()
            };
            points.push(internal_liq_dist);

            // Hardcodes 1.0 liquidity to keep the scale in line.
            // todo: fix these to scale better.
            let internal_price = compute_price_given_x_rust(
                internal_reserves.x as f64,
                1.0,
                strike_price_wad_float,
                sigma_percent_wad_float,
                time_to_expiry_years_wad_float,
            );

            let internal_price = ChartPoint {
                x: internal_reserves.x,
                y: internal_price as f32,
                color: plotters::style::full_palette::DEEPPURPLE_A400,
                ..Default::default()
            };
            points.push(internal_price);
        }

        Ok(points)
    }

    /// Computes the plot of the trading function given strike price,
    /// volatility, and time remaining.
    /// Plots:
    /// - Trading function
    /// - Liquidity distribution
    /// - Price curve
    #[allow(clippy::type_complexity)]
    pub fn compute_strategy_plot(
        &self,
        strike_price: f64,
        volatility: f64,
        time_remaining: f64,
        // TODO: make a type allias for this
    ) -> (Vec<(f64, f64)>, Vec<(f64, f64)>, Vec<(f64, f64)>) {
        let mut curve_points = vec![];
        let mut liq_dist_points = vec![];
        let mut price_curve_points = vec![];

        // Initial x != 0!!! be careful.
        let mut x = f64::EPSILON;
        let samples = 100.0;
        let max = 1.0;
        while x < max {
            let y = compute_y_given_x_rust(x, 1.0, strike_price, volatility, time_remaining);
            curve_points.push((x, y));

            // This really impacts performance!! Like freezes the app.
            let liq_dist = liq_distribution(x, 1.0, strike_price, volatility, time_remaining);
            liq_dist_points.push((x, liq_dist));

            let price =
                compute_price_given_x_rust(x, 1.0, strike_price, volatility, time_remaining);
            price_curve_points.push((x, price));

            x += max / samples;
        }

        (curve_points, liq_dist_points, price_curve_points)
    }

    pub fn derive_computed_strategy_plot(
        &self,
        strike_price: f64,
        volatility: f64,
        time_remaining: f64,
    ) -> Result<Vec<(CartesianRanges, ChartLineSeries)>> {
        let (curve_points, liq_dist_points, price_curve_points) =
            self.compute_strategy_plot(strike_price, volatility, time_remaining);

        let max_x = 1.0; // total_liquidity;
        let max_y = strike_price; // strike_price * total_liquidity;

        // Min y and min x are both 0, so set their margin to a slightly negative
        // proportion of the total range.
        let min_x = -max_x * 0.1; // 10%
        let min_y = -max_y * 0.1; // 10%

        // to f32
        let converted_curve_points = curve_points
            .iter()
            .map(|(x, y)| (*x as f32, *y as f32))
            .collect();
        let mut curve_series = coords_to_line_series(converted_curve_points);
        curve_series.legend = "Log Normal".to_string();

        let curve_ranges = CartesianRanges {
            x_range: (min_x as f32, max_x as f32),
            y_range: (min_y as f32, max_y as f32),
        };

        // to f32
        let converted_liq_dist_points = liq_dist_points
            .iter()
            .map(|(x, y)| (*x as f32, *y as f32))
            .collect();

        let mut liq_dist_series = coords_to_line_series(converted_liq_dist_points);
        liq_dist_series.legend = "Liq. Dist.".to_string();
        liq_dist_series.color = plotters::style::full_palette::PURPLE_A400;

        let max_y_dist = liq_dist_points
            .iter()
            .max_by(|(_, y1), (_, y2)| y1.partial_cmp(y2).unwrap_or(Ordering::Equal))
            .ok_or(Error::msg("Failed to get max y"))?
            .1;

        let liq_dist_ranges = CartesianRanges {
            x_range: (min_x as f32, max_x as f32),
            y_range: (min_y as f32, max_y_dist as f32),
        };

        let converted_price_points = price_curve_points
            .iter()
            .map(|(x, y)| (*x as f32, *y as f32))
            .collect();
        let mut price_curve_series = coords_to_line_series(converted_price_points);
        price_curve_series.legend = "Price".to_string();
        price_curve_series.color = plotters::style::full_palette::GREEN_A400;

        // Set the ranges.
        let price_curve_ranges = CartesianRanges {
            x_range: (min_x as f32, max_x as f32),
            y_range: (min_y as f32, max_y as f32),
        };

        // Return it all!
        Ok(vec![
            (curve_ranges, curve_series),
            (liq_dist_ranges, liq_dist_series),
            (price_curve_ranges, price_curve_series),
        ])
    }

    /// Transforms the portfolio strategy into a plotted curve with the current
    /// portfolio composition as a point of interest.
    pub fn derive_portfolio_strategy_plot(
        &self,
    ) -> Result<(CartesianRanges, Vec<ChartLineSeries>)> {
        let strike_price = Self::format_and_parse(
            self.strike_price_wad
                .ok_or(anyhow!("Strike price is None"))?,
        )?;
        let volatility =
            Self::format_and_parse(self.volatility_wad.ok_or(anyhow!("Volatility is None"))?)?;
        let time_remaining = Self::format_and_parse(
            self.time_remaining_wad
                .ok_or(anyhow!("Time remaining is None"))?,
        )?;
        let total_liquidity = Self::format_and_parse(
            self.total_liquidity
                .ok_or(anyhow!("Total liquidity is None"))?,
        )?;
        if total_liquidity == 0.0 {
            return Err(anyhow!("Total liquidity is 0"));
        }

        // Choose the maximum bounds for x and y. These use log normal curve
        // assumptions.
        // todo: fix bounds based on scaling, scale liquidity on/off
        let max_x = 1.0; // total_liquidity;
        let max_y = strike_price; // strike_price * total_liquidity;

        // Min y and min x are both 0, so set their margin to a slightly negative
        // proportion of the total range.
        let min_x = -max_x * 0.1; // 10%
        let min_y = -max_y * 0.1; // 10%

        // Compute the x and y values for the curve.
        let mut curve_points = vec![];

        let mut liq_dist_points = vec![];

        let mut price_curve_points = vec![];

        // Initial x != 0!!! be careful.
        let mut x = f64::EPSILON;
        let samples = 100.0;
        let max = 1.0;
        while x < max {
            let y = compute_y_given_x_rust(x, 1.0, strike_price, volatility, time_remaining);
            curve_points.push((x, y));

            // This really impacts performance!! Like freezes the app.
            let liq_dist = liq_distribution(x, 1.0, strike_price, volatility, time_remaining);
            liq_dist_points.push((x, liq_dist));

            let price =
                compute_price_given_x_rust(x, 1.0, strike_price, volatility, time_remaining);
            price_curve_points.push((x, price));

            x += max / samples;
        }

        // Convert the x and y values to curve_points that can be converted to a line
        // series, which uses f32 types.
        let converted_curve_points = curve_points
            .iter()
            .map(|(x, y)| (*x as f32, *y as f32))
            .collect();
        let mut curve_series = coords_to_line_series(converted_curve_points);
        curve_series.legend = "Log Normal".to_string();

        let converted_liq_dist_points = liq_dist_points
            .iter()
            .map(|(x, y)| (*x as f32, *y as f32))
            .collect();
        let mut liq_dist_series = coords_to_line_series(converted_liq_dist_points);
        liq_dist_series.legend = "Liq. Dist.".to_string();
        liq_dist_series.color = plotters::style::full_palette::PURPLE_A400;

        let converted_price_points = price_curve_points
            .iter()
            .map(|(x, y)| (*x as f32, *y as f32))
            .collect();
        let mut price_curve_series = coords_to_line_series(converted_price_points);
        price_curve_series.legend = "Price".to_string();
        price_curve_series.color = plotters::style::full_palette::GREEN_A400;

        // Set the ranges.
        let ranges = CartesianRanges {
            x_range: (min_x as f32, max_x as f32),
            y_range: (min_y as f32, max_y as f32),
        };

        // Return it all!
        Ok((
            ranges,
            vec![curve_series, liq_dist_series, price_curve_series],
        ))
    }

    pub fn derive_asset_value_series(&self) -> Result<(CartesianRanges, ChartLineSeries)> {
        let user_asset_value_series = self
            .user_asset_value_series
            .clone()
            .ok_or(anyhow!("User asset value series is None"))?;
        let mut result = Self::transform_series_over_block_number(&user_asset_value_series)?;

        let asset_name = if let Some(asset_token_info) = &self.cached.asset_token_info {
            &asset_token_info.symbol
        } else {
            "Asset"
        };

        result.1.legend = format!("$ {}", asset_name);
        result.1.color = plotters::style::full_palette::BLUE_A400;
        result.1.time_series = true;

        Ok(result)
    }

    pub fn derive_quote_value_series(&self) -> Result<(CartesianRanges, ChartLineSeries)> {
        let user_quote_value_series = self
            .user_quote_value_series
            .clone()
            .ok_or(anyhow!("User quote value series is None"))?;
        let mut result = Self::transform_series_over_block_number(&user_quote_value_series)?;

        let quote_name = if let Some(quote_token_info) = &self.cached.quote_token_info {
            &quote_token_info.symbol
        } else {
            "Quote"
        };

        result.1.legend = format!("$ {}", quote_name);
        result.1.color = plotters::style::full_palette::PINK_A400;
        result.1.time_series = true;

        Ok(result)
    }

    /// Computes liquidity distribution and returns histogram formatted data.
    pub fn derive_liquidity_histogram(
        &self,
        strike_price: f64,
        volatility: f64,
        time_remaining: f64,
    ) -> Result<HistogramData> {
        let current_price = Self::format_and_parse(
            self.external_spot_price
                .ok_or(anyhow!("External spot price is None"))?,
        )?;

        let min_price = f64::EPSILON;
        let max_price = current_price * 2.0;

        // Scale the x-axis "prices" to the histogram "bins".
        let scalar = 100.0; // Define the scalar
        let min_bin = (min_price * scalar).round() as u32;
        let max_bin = (max_price * scalar).round() as u32;

        // Define the number of bins to group the data by.
        let num_bins = 25u32;
        let bin_size = (max_bin - min_bin) / num_bins;

        let mut max_count = 0;
        let mut data: BTreeMap<u32, u32> = BTreeMap::new();
        let mut notable_bars: BTreeMap<u32, u32> = BTreeMap::new();

        for bin_index in 0..num_bins {
            let bin_start = min_bin + bin_index * bin_size;
            let bin_end = bin_start + bin_size;

            // Calculate price for this bin
            let price = ((bin_start + bin_end) as f64 / scalar) / 2.0;
            let price_key = (price * scalar).round() as u32;

            // Compute the x and liquidity distribution values at this given price.
            let x = compute_x_given_price(price, 1.0, strike_price, volatility, time_remaining);
            let liq_dist = liq_distribution(x, 1.0, strike_price, volatility, time_remaining);

            // Collect the data into the histogram.
            let count = data.entry(price_key).or_insert(0);
            *count += (liq_dist * scalar).round() as u32;

            // Keep a track of the highest count to bound the y-axis in the chart.
            if *count > max_count {
                max_count = *count;
            }
        }

        // Find the closest bin to the current price, and use its value as a notable
        // bar.
        let mut closest_bin = 0;
        let mut closest_bin_distance = f64::MAX;
        for bin in data.keys() {
            let distance = (*bin as f64 / scalar - current_price).abs();
            if distance < closest_bin_distance {
                closest_bin_distance = distance;
                closest_bin = *bin;
            }
        }

        // Add the closest bin to the notable bars.
        notable_bars.insert(closest_bin, *data.get(&closest_bin).unwrap());

        Ok(HistogramData {
            min_bin,
            max_bin,
            max_count,
            bin_size,
            data,
            notable_bars,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct HistogramData {
    pub min_bin: u32,
    pub max_bin: u32,
    pub max_count: u32,
    pub bin_size: u32,
    pub data: BTreeMap<u32, u32>,
    pub notable_bars: BTreeMap<u32, u32>,
}

#[cfg(test)]
mod tests {
    use arbiter_bindings::bindings::arbiter_token::ArbiterToken;
    use ethers::{
        prelude::*,
        utils::{Anvil, AnvilInstance},
    };
    use sim::from_ethers_address;

    use super::{AlloyAddress, AlloyU256, *};

    async fn setup() -> anyhow::Result<(
        AnvilInstance,
        Arc<SignerMiddleware<Provider<Ws>, LocalWallet>>,
    )> {
        let anvil = Anvil::default()
            .arg("--gas-limit")
            .arg("20000000")
            .chain_id(31337_u64)
            .spawn();

        let url = anvil.ws_endpoint().to_string();
        let wallet: LocalWallet = anvil.keys().get(0).unwrap().clone().into();
        let wallet = wallet.with_chain_id(anvil.chain_id());
        println!("Connected to URL: {}", url);

        println!("Wallet address: {}", wallet.address());
        let provider = Provider::<Ws>::connect(&url).await?;
        let client = Arc::new(SignerMiddleware::new(provider, wallet));

        Ok((anvil, client))
    }

    #[test]
    fn test_sol_ierc20() {
        println!(
            "IERC20: {:?}",
            IERC20::balanceOfCall {
                account: AlloyAddress::ZERO,
            }
        );

        println!("IERC20 selector: {:?}", IERC20::balanceOfCall::SELECTOR);
    }

    #[test]
    fn test_decode_returns_bug() {
        let data = alloy_primitives::Bytes::from(vec![0_u8]);
        let decoded = IERC20::balanceOfCall::abi_decode_returns(&data, false);
        println!("Decoded: {:?}", decoded);
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_fetch_balance_of() -> anyhow::Result<()> {
        let (anvil, client) = setup().await?;

        // Need to deploy a token and mint some to wallet!
        let token =
            ArbiterToken::deploy(client.clone(), ("Test".to_string(), "T".to_string(), 18_u8))?
                .send()
                .await?;

        println!("Deployed token: {:?}", token.address());

        let sender = client.address();

        token.mint(sender, 100.into()).send().await?;
        println!("Minted tokens");

        let chain_id = anvil.chain_id();

        // Now we can fetch the balance of the wallet.
        let model = DataModel::<AlloyAddress, AlloyU256>::new(chain_id);

        let converted_address = from_ethers_address(sender);
        let converted_token_address = from_ethers_address(token.address());

        println!("Fetching balance of: {}", converted_address);
        let balance = model
            .fetch_balance_of(
                client.provider().clone().into(),
                converted_token_address,
                converted_address,
            )
            .await?;

        println!("Balance: {}", balance);

        Ok(())
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_update_token_balances() -> anyhow::Result<()> {
        let (anvil, client) = setup().await?;

        // Need to deploy a token and mint some to wallet!
        let token =
            ArbiterToken::deploy(client.clone(), ("Test".to_string(), "T".to_string(), 18_u8))?
                .send()
                .await?;

        println!("Deployed token: {:?}", token.address());

        let sender = client.address();

        token.mint(sender, 100.into()).send().await?;
        println!("Minted tokens");

        let chain_id = anvil.chain_id();

        // Now we can fetch the balance of the wallet.
        let mut model = DataModel::<AlloyAddress, AlloyU256>::new(chain_id);

        let converted_address = from_ethers_address(sender);
        let converted_token_address = from_ethers_address(token.address());

        println!("Fetching balance of: {}", converted_address);
        let balance = model
            .fetch_balance_of(
                client.provider().clone().into(),
                converted_token_address,
                converted_address,
            )
            .await?;

        println!("Balance: {}", balance);

        // Update the model.
        let converted_user_address = from_ethers_address(sender);
        let converted_token_address = from_ethers_address(token.address());
        model.user_address = Some(converted_user_address);
        model.asset_token = Some(converted_token_address);
        model.quote_token = Some(converted_token_address);
        model.protocol_address = Some(AlloyAddress::ZERO);
        model.strategy_address = Some(AlloyAddress::ZERO);

        model
            .update_token_balances(client.provider().clone().into())
            .await?;

        // Log the new balance.
        println!("User asset balance: {:?}", model.user_asset_balance);

        assert_eq!(model.user_asset_balance, Some(balance));

        Ok(())
    }
}
