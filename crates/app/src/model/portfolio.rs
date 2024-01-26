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

use std::collections::BTreeMap;

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
use datatypes::portfolio::coin_list::{self, CoinList};
use ethers::types::transaction::eip2718::TypedTransaction;
use serde::{Deserialize, Serialize};
use sim::{from_ethers_address, from_ethers_u256, to_ethers_address, to_ethers_u256};

use super::*;
use crate::components::chart::{
    coords_to_line_series, CartesianRanges, ChartLineSeries, ChartPoint,
};

pub type EthersAddress = ethers::types::Address;
pub type EthersU256 = ethers::types::U256;
pub type AlloyAddress = alloy_primitives::Address;
pub type AlloyU256 = alloy_primitives::U256;

pub const ALLOY_WAD: AlloyU256 = AlloyU256::from_limbs([1_000_000_000_000_000_000, 0, 0, 0]);

pub enum RawDataModelError {
    CheckedMul,
    CheckedDiv,
    CheckedAdd,
    BlockNumberMismatch(String),
}

impl From<RawDataModelError> for Error {
    fn from(error: RawDataModelError) -> Self {
        match error {
            RawDataModelError::CheckedMul => Error::msg("Checked mul error"),
            RawDataModelError::CheckedDiv => Error::msg("Checked div error"),
            RawDataModelError::CheckedAdd => Error::msg("Checked add error"),
            RawDataModelError::BlockNumberMismatch(error) => {
                Error::msg(format!("Block number mismatch error: {}", error))
            }
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Cached {
    pub raw_asset_token_info: Option<TokenInfo>,
    pub raw_quote_token_info: Option<TokenInfo>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TokenInfo {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
}

/// Tracks global pool state and the individual token balances of the respective
/// pool's tokens for the connected user.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PoolState<A, V> {
    pub id: Option<V>,
    pub controller: Option<A>,
    pub strategy: Option<A>,
    pub asset_token: Option<A>,
    pub quote_token: Option<A>,
    pub liquidity_token: Option<A>,

    // Tracks the internal price of the pool as reported by the solver contract.
    pub internal_price: Option<Vec<(u64, V)>>,
    // Tracks the total virtual liquidity of the pool. Not the same as liquidity token.
    pub total_liquidity: Option<Vec<(u64, V)>>,
    // Tracks the pool's token reserves.
    pub asset_reserve: Option<Vec<(u64, V)>>,
    pub quote_reserve: Option<Vec<(u64, V)>>,
    // Tracks the total supply of the liquidity token to compute the user's
    // percentage of the pool.
    pub liquidity_token_total_supply: Option<Vec<(u64, V)>>,
    // Tracks the strategy specific state. Only one can be set per pool.
    pub log_normal_strategy: Option<LogNormalStrategyState<V>>,
    pub g3m_strategy: Option<G3MStrategyState<V>>,
    // Swap fee is specific to strategy, but all strategies have a swap fee.
    pub swap_fee_wad: Option<V>,
}

/// todo: support/integrate the dynamic params and time series data of the
/// dynamic params.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LogNormalStrategyState<V> {
    pub strike_price: V,
    pub volatility: V,
    pub time_remaining: V,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct G3MStrategyState<V> {
    // todo!
    pub todo: V,
}

/// The model!
/// - user_address must be set to a valid address via `setup` before calling
///   `update`.
/// - external_exchange_address must be set to a valid address via `setup`
///   before calling `update`.
/// - dfmm_address must be set to a valid address via `setup` before calling
///   `update`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RawDataModel<A: Ord, V> {
    // Network id for the chain that this model is connected to / fetching data from.
    pub chain_id: Option<u64>,
    // Signer's public address.
    pub user_address: Option<A>,
    // An external exchange address that can be used to fetch pricing info.
    pub external_exchange_address: Option<A>,
    // The address of the DFMM smart contract.
    pub dfmm_address: Option<A>,
    // The address of the solver for the log normal strategy.
    pub log_normal_solver_address: Option<A>,
    // The address of the solver for the g3m strategy.
    pub g3m_solver_address: Option<A>,
    // Timestamp of the most recent model update.
    pub last_sync: Option<DateTime<Utc>>,
    // Block number of the most recent model update.
    pub last_sync_block: Option<u64>,
    // Tracks all the user's transactions with the DFMM protocol.
    pub user_history: Option<Vec<HistoricalTx>>,
    // Block number of the most recent model update that fetched user history.
    pub last_user_history_sync_block: Option<u64>,
    // Balances of tokens held directly by the connected user.
    #[serde(default)]
    pub user_token_balances: BTreeMap<A, Vec<(u64, V)>>,
    // Tracks the global state of pools.
    pub pool_state: Option<BTreeMap<u64, PoolState<A, V>>>,
    // Tracks the prices of the tokens in the user_token_balances
    // mapping reported by the external exchange
    pub external_prices: Option<BTreeMap<A, Vec<(u64, V)>>>,
    // Used to filter out the liquidity tokens from the user_token_balances mapping.
    pub liquidity_token_addresses: Option<Vec<A>>,
    // Stores the metadata of the tokens being fetched once.
    pub token_metadata: Option<BTreeMap<A, TokenInfo>>,
}

sol! {
    #[derive(Debug)]
    interface IERC20 {
        function balanceOf(address account) external view returns(uint balance);
        function name() external view returns(string name);
        function symbol() external view returns(string symbol);
        function decimals() external view returns(uint8 decimals);
        function totalSupply() external view returns(uint);
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Eq, PartialEq, Hash)]
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
    pub pool_id: u64,
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

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AllocatedPosition {
    pub token_x_address: AlloyAddress,
    pub token_y_address: AlloyAddress,
    pub token_l_address: AlloyAddress,
    pub claimable_balance_x: f64,
    pub claimable_balance_y: f64,
    pub liquidity_balance: f64,
    pub liquidity_value: f64,
    pub external_price: f64,
    pub internal_price: f64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UnallocatedPosition {
    pub token_address: AlloyAddress,
    pub balance: f64,
    pub external_price: f64,
}

impl StrategyPosition {
    pub fn compute_value(&self) -> Result<f64> {
        Ok(self.balance_x * self.external_price + self.balance_y * self.quote_price)
    }
}

impl RawDataModel<AlloyAddress, AlloyU256> {
    // ----- Init model ----- //

    /// Creates a completely fresh model with no values set.
    pub fn new(chain_id: u64) -> Self {
        Self {
            chain_id: Some(chain_id),
            ..Default::default()
        }
    }

    /// Sets up the model with the required addresses needed to fetch all the
    /// data in the model.
    /// - user_address is the public address of the signer used in the
    ///   application to send transactions.
    /// - external_exchange_address is the address of the external exchange used
    ///   to fetch prices.
    /// - dfmm_address is the address of the DFMM protocol.
    #[allow(clippy::too_many_arguments)]
    pub fn setup(
        &mut self,
        user_address: AlloyAddress,
        external_exchange_address: AlloyAddress,
        dfmm_address: AlloyAddress,
        log_normal_solver_address: AlloyAddress,
        g3m_solver_address: AlloyAddress,
    ) {
        self.user_address = Some(user_address);
        self.external_exchange_address = Some(external_exchange_address);
        self.dfmm_address = Some(dfmm_address);
        self.log_normal_solver_address = Some(log_normal_solver_address);
        self.g3m_solver_address = Some(g3m_solver_address);
    }

    /// Adds a token to the token balance mapping, which is the universal token
    /// list that will be tracked and updated by the model.
    pub fn add_token(&mut self, token_address: AlloyAddress) -> Result<()> {
        self.user_token_balances
            .entry(token_address)
            .or_insert(Vec::new());
        Ok(())
    }

    /// Adds a pool_id to the pool state mapping, which will be tracked in
    /// future model updates.
    pub fn add_pool(&mut self, pool_id: u64) -> Result<()> {
        self.pool_state
            .get_or_insert_with(BTreeMap::new)
            .entry(pool_id)
            .or_insert_with(PoolState::default);
        Ok(())
    }

    // ----- Model updates ----- //

    /// Updates the ENTIRE model! Wow!
    pub async fn update<M: Middleware + 'static>(&mut self, client: Arc<M>) -> Result<()>
    where
        <M as ethers::providers::Middleware>::Error: 'static,
    {
        // Update sync block + timestamp first, since the other update methods need it.
        // These updates must be successful.
        self.update_last_sync_timestamp()?;
        self.update_last_sync_block(client.clone()).await?;
        self.last_user_history_sync_block = Some(0);

        // Updates the model state.

        if let Err(err) = self.update_token_prices_mapping(client.clone()).await {
            tracing::warn!("Token price mapping update failed: {:?}", err);
        }

        if let Err(err) = self.update_historical_txs(client.clone()).await {
            tracing::warn!("User history update failed: {:?}", err);
        }

        if let Err(err) = self.update_all_pools(client.clone()).await {
            tracing::warn!("Pool update failed: {:?}", err);
        }

        if let Err(err) = self.sync_liquidity_tokens_to_balance_mapping() {
            tracing::warn!("Liquidity token sync failed: {:?}", err);
        }

        if let Err(err) = self.update_token_balance_mapping(client.clone()).await {
            tracing::warn!("Token balance mapping update failed: {:?}", err);
        }

        if let Err(err) = self.update_token_info_mapping(client.clone()).await {
            tracing::warn!("Token info mapping update failed: {:?}", err);
        }

        Ok(())
    }

    // Updates all the token balance series' for the user for the current block.
    async fn update_token_balance_mapping<M: Middleware + 'static>(
        &mut self,
        client: Arc<M>,
    ) -> Result<()> {
        let user_address = self
            .user_address
            .ok_or(Error::msg("User address not set"))?;

        // For each token tracked in the token_balances mapping, fetch the balances and
        // update the model.
        let current_block = self.fetch_block_number(client.clone()).await?;
        let token_addresses: Vec<_> = self.user_token_balances.keys().cloned().collect();
        for token_address in token_addresses {
            let new_balance = self
                .fetch_balance_of(client.clone(), token_address, user_address)
                .await?;
            self.user_token_balances
                .get_mut(&token_address)
                .unwrap()
                .push((current_block, new_balance));
        }

        Ok(())
    }

    /// todo: update to work with prices correctly, since current exchange
    /// assumes price of asset token.
    async fn update_token_prices_mapping<M: Middleware + 'static>(
        &mut self,
        client: Arc<M>,
    ) -> Result<()> {
        let current_block = self.fetch_block_number(client.clone()).await?;
        let token_addresses: Vec<_> = self.user_token_balances.keys().cloned().collect();
        for token_address in token_addresses {
            let new_price = self
                .fetch_external_price_of_token(client.clone(), token_address)
                .await?;
            let external_prices = self.external_prices.get_or_insert_with(BTreeMap::new);
            let price_series = external_prices
                .entry(token_address)
                .or_insert_with(Vec::new);
            price_series.push((current_block, new_price));
        }

        Ok(())
    }

    /// Ensures all liquidity token addresses are keys in the balance mapping.
    pub fn sync_liquidity_tokens_to_balance_mapping(&mut self) -> Result<()> {
        let liquidity_token_addresses = self
            .liquidity_token_addresses
            .as_ref()
            .ok_or(Error::msg("Liquidity token addresses not set"))?;

        for liquidity_token_address in liquidity_token_addresses {
            self.user_token_balances
                .entry(liquidity_token_address.clone())
                .or_insert_with(Vec::new);
        }

        Ok(())
    }

    pub async fn update_pool<M: Middleware + 'static>(
        &mut self,
        client: Arc<M>,
        pool_id: u64,
    ) -> Result<()>
    where
        <M as ethers::providers::Middleware>::Error: 'static,
    {
        let new_pool_state = self.fetch_pool_state(client.clone(), pool_id).await?;
        let liquidity_token_address = new_pool_state.liquidity_token.clone().unwrap();

        let pool_state_map = self.pool_state.get_or_insert_with(BTreeMap::new);

        if let Some(existing_pool_state) = pool_state_map.get_mut(&pool_id) {
            // Append new series data to existing pool state
            if let Some(new_internal_price) = new_pool_state.internal_price {
                existing_pool_state
                    .internal_price
                    .get_or_insert_with(Vec::new)
                    .extend(new_internal_price);
            }
            if let Some(new_total_liquidity) = new_pool_state.total_liquidity {
                existing_pool_state
                    .total_liquidity
                    .get_or_insert_with(Vec::new)
                    .extend(new_total_liquidity);
            }
            if let Some(new_asset_reserve) = new_pool_state.asset_reserve {
                existing_pool_state
                    .asset_reserve
                    .get_or_insert_with(Vec::new)
                    .extend(new_asset_reserve);
            }
            if let Some(new_quote_reserve) = new_pool_state.quote_reserve {
                existing_pool_state
                    .quote_reserve
                    .get_or_insert_with(Vec::new)
                    .extend(new_quote_reserve);
            }
            if let Some(new_liquidity_token_total_supply) =
                new_pool_state.liquidity_token_total_supply
            {
                existing_pool_state
                    .liquidity_token_total_supply
                    .get_or_insert_with(Vec::new)
                    .extend(new_liquidity_token_total_supply);
            }
            // Add other fields as needed
        } else {
            // Insert new pool state as is
            pool_state_map.insert(pool_id, new_pool_state);
        }

        // Update the liquidity_token_addresses mapping if it missing the new pool's
        // liquidity token address.
        let liquidity_token_addresses = self.liquidity_token_addresses.get_or_insert_with(Vec::new);
        if !liquidity_token_addresses.contains(&liquidity_token_address) {
            liquidity_token_addresses.push(liquidity_token_address);
        }

        // Update the liquidity token's metadata if it is missing from the
        // token_metadata mapping.
        if self
            .token_metadata
            .get_or_insert_with(BTreeMap::new)
            .get(&liquidity_token_address)
            .is_none()
        {
            let token_info = self
                .fetch_token_info(client.clone(), liquidity_token_address)
                .await?;
            self.token_metadata
                .as_mut()
                .unwrap()
                .insert(liquidity_token_address, token_info);
        }

        Ok(())
    }

    /// Updates the state of all the pools.
    pub async fn update_all_pools<M: Middleware + 'static>(&mut self, client: Arc<M>) -> Result<()>
    where
        <M as ethers::providers::Middleware>::Error: 'static,
    {
        let historical_txs = self
            .user_history
            .as_ref()
            .ok_or(Error::msg("User history not set"))?;

        let historical_creates: Vec<_> = historical_txs
            .iter()
            .filter(|tx| tx.action == ProtocolActions::CreatePosition)
            .collect();

        let historical_create_pool_ids: Vec<_> =
            historical_creates.iter().map(|tx| tx.pool_id).collect();

        for pool_id in historical_create_pool_ids {
            self.update_pool(client.clone(), pool_id).await?;
        }

        Ok(())
    }

    /// Cross-references the token balances mapping with the token metadata
    /// mapping and fetches token info if it is missing.
    pub async fn update_token_info_mapping<M: Middleware + 'static>(
        &mut self,
        client: Arc<M>,
    ) -> Result<()>
    where
        <M as ethers::providers::Middleware>::Error: 'static,
    {
        let token_addresses: Vec<_> = self.user_token_balances.keys().cloned().collect();
        for token_address in token_addresses {
            if self
                .token_metadata
                .get_or_insert_with(BTreeMap::new)
                .get(&token_address)
                .is_none()
            {
                let token_info = self.fetch_token_info(client.clone(), token_address).await?;
                self.token_metadata
                    .as_mut()
                    .unwrap()
                    .insert(token_address, token_info);
            }
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
        self.user_history = Some(historical_txs);
        self.last_user_history_sync_block = self.last_sync_block;
        Ok(())
    }

    pub fn update_last_sync_timestamp(&mut self) -> Result<()> {
        let timestamp = Utc::now();
        self.last_sync = Some(timestamp);

        Ok(())
    }

    pub async fn update_last_sync_block<M: Middleware + 'static>(
        &mut self,
        client: Arc<M>,
    ) -> Result<()> {
        let block_number = self.fetch_block_number(client.clone()).await?;
        self.last_sync_block = Some(block_number);
        Ok(())
    }

    // ----- Fetchers ----- //

    pub async fn fetch_block_number<M: Middleware + 'static>(&self, client: Arc<M>) -> Result<u64>
    where
        <M as ethers::providers::Middleware>::Error: 'static,
    {
        let block_number = client.get_block_number().await?;
        Ok(block_number.as_u64())
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

        let payload = IERC20::balanceOfCall { account: address };
        let payload = ethers::types::Bytes::from(payload.abi_encode());

        let mut tx = TypedTransaction::default();
        tx.set_to(converted_token_address).set_data(payload);

        // Send the call to the token contract.
        let balance = client.call(&tx, None).await?;
        let decoded: <IERC20::balanceOfCall as SolCall>::Return =
            IERC20::balanceOfCall::abi_decode_returns(&balance, false)?;
        let decoded_balance: AlloyU256 = decoded.balance;

        Ok(decoded_balance)
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

    /// Indexes the "Init" events emitted by the protocol when the user creates
    /// a new position.
    pub async fn fetch_user_historical_tx<M: Middleware + 'static>(
        &self,
        client: Arc<M>,
    ) -> Result<Vec<HistoricalTx>>
    where
        <M as ethers::providers::Middleware>::Error: 'static,
    {
        let current_block = self.fetch_block_number(client.clone()).await?;
        let last_block = self
            .last_user_history_sync_block
            .ok_or(Error::msg("Last historical chain data sync block not set"))?;

        let user_address = self
            .user_address
            .ok_or(Error::msg("User address not set"))?;
        let user_address = to_ethers_address(user_address);

        let protocol_address = self
            .dfmm_address
            .ok_or(Error::msg("Protocol address not set"))?;
        let protocol_address = to_ethers_address(protocol_address);

        let protocol = DFMM::new(protocol_address, client.clone());
        tracing::debug!("Fetching historical tx!");

        let create_pos_filter = protocol
            .init_filter()
            .filter
            .from_block(last_block)
            .to_block(current_block);

        let raw_logs = client.get_logs(&create_pos_filter).await?;
        let raw_logs = raw_logs
            .into_iter()
            .filter(|log| {
                log.topics.get(1).map(|topic| EthersAddress::from(*topic)) == Some(user_address)
            })
            .collect::<Vec<_>>();

        let mut historical_tx = vec![];

        for raw_log in raw_logs {
            let block_number = raw_log.block_number.unwrap().as_u64();
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
            let tx_hash = raw_log.transaction_hash.unwrap();
            let action = ProtocolActions::CreatePosition;

            let parsed_log = protocol.decode_event::<InitFilter>(
                "Init",
                raw_log.topics.clone(),
                raw_log.data.clone(),
            )?;

            let pool_id = parsed_log.pool_id.as_u64();

            // Get the pool state for the given pool id.
            let pool_state = self.fetch_pool_state(client.clone(), pool_id).await?;
            // Get the token metadata for the pool's assets.
            let token_x_metadata = self
                .fetch_token_info(client.clone(), pool_state.asset_token.unwrap())
                .await?;
            let token_y_metadata = self
                .fetch_token_info(client.clone(), pool_state.quote_token.unwrap())
                .await?;

            let amount_x = EthersU256::from(parsed_log.reserve_x);
            let amount_y = EthersU256::from(parsed_log.reserve_y);

            // Try getting the prices from the series
            let x_token_address = pool_state
                .asset_token
                .ok_or(Error::msg("Asset token address not set for pool state"))?;
            let y_token_address = pool_state
                .quote_token
                .ok_or(Error::msg("Quote token address not set for pool state"))?;

            let external_price_series = self.external_prices.as_ref().ok_or(Error::msg(
                "External prices mapping not set for user token balances",
            ))?;

            let external_x_price = external_price_series[&x_token_address]
                .iter()
                .find(|(block, _)| *block == block_number)
                .map(|(_, price)| *price)
                .ok_or(Error::msg(format!(
                    "Missing external price for historical tx at block {}",
                    block_number
                )))?;

            // todo: need to add an external quote price series.
            let external_y_price = ALLOY_WAD;

            let amount_x = from_ethers_u256(amount_x);
            let amount_y = from_ethers_u256(amount_y);

            let amount_x = amount_x
                .checked_mul(external_x_price)
                .ok_or(anyhow!(RawDataModelError::CheckedMul))?
                .checked_div(ALLOY_WAD)
                .ok_or(anyhow!(RawDataModelError::CheckedDiv))?;

            let amount_y = amount_y
                .checked_mul(external_y_price)
                .ok_or(anyhow!(RawDataModelError::CheckedMul))?
                .checked_div(ALLOY_WAD)
                .ok_or(anyhow!(RawDataModelError::CheckedDiv))?;

            let amount_x = alloy_primitives::utils::format_ether(amount_x);
            let amount_y = alloy_primitives::utils::format_ether(amount_y);

            let amount_x = amount_x.parse::<f64>()?;
            let amount_y = amount_y.parse::<f64>()?;

            let market_value = amount_x + amount_y;

            let position_name =
                format!("{} / {}", token_x_metadata.symbol, token_y_metadata.symbol);

            historical_tx.push(HistoricalTx {
                tx_hash,
                block_number,
                timestamp: datetime,
                action,
                position_name,
                market_value,
                pool_id,
            });
        }

        Ok(historical_tx)
    }

    /// Fetches the raw pool state from the protocol. Should only be used to
    /// initialize the immutable state of a pool upon pool initialization.
    pub async fn fetch_pool_state<M: Middleware + 'static>(
        &self,
        client: Arc<M>,
        pool_id: u64,
    ) -> Result<PoolState<AlloyAddress, AlloyU256>>
    where
        <M as ethers::providers::Middleware>::Error: 'static,
    {
        let parsed_pool_id = EthersU256::from(pool_id);

        let dfmm = self.protocol(client.clone()).await?;
        let current_block = self.fetch_block_number(client.clone()).await?;

        let (
            controller,
            strategy,
            asset_token,
            quote_token,
            asset_reserve,
            quote_reserve,
            total_liquidity,
            liquidity_token,
        ) = dfmm.pools(parsed_pool_id).call().await?;

        let asset_reserve: Option<Vec<(u64, AlloyU256)>> = if asset_reserve.is_zero() {
            None
        } else {
            Some(vec![(current_block, from_ethers_u256(asset_reserve))])
        };

        let quote_reserve: Option<Vec<(u64, AlloyU256)>> = if quote_reserve.is_zero() {
            None
        } else {
            Some(vec![(current_block, from_ethers_u256(quote_reserve))])
        };

        let total_liquidity: Option<Vec<(u64, AlloyU256)>> = if total_liquidity.is_zero() {
            None
        } else {
            Some(vec![(current_block, from_ethers_u256(total_liquidity))])
        };

        let payload = IERC20::totalSupplyCall {};
        let payload = ethers::types::Bytes::from(payload.abi_encode());

        let mut tx = TypedTransaction::default();
        tx.set_to(liquidity_token).set_data(payload);

        // Send the call to the token contract.
        let total_supply = client.call(&tx, None).await?;
        let decoded: <IERC20::totalSupplyCall as SolCall>::Return =
            IERC20::totalSupplyCall::abi_decode_returns(&total_supply, false)?;
        let total_supply = decoded._0;

        let liquidity_token_total_supply: Option<Vec<(u64, AlloyU256)>> = if total_supply.is_zero()
        {
            None
        } else {
            Some(vec![(current_block, total_supply)])
        };

        // todo: add g3m strategy check/integration
        let strategy_instance =
            self.log_normal_strategy(client.clone(), from_ethers_address(strategy))?;

        let (strike_price, volatility, time_remaining, swap_fee_wad) = strategy_instance
            .internal_params(parsed_pool_id)
            .call()
            .await
            .map_err(|error| anyhow!(error))?;

        let strike_price = strike_price.last_computed_value;
        let volatility = volatility.last_computed_value;
        let time_remaining = time_remaining.last_computed_value;

        let log_normal_strategy = Some(LogNormalStrategyState {
            strike_price: from_ethers_u256(strike_price),
            volatility: from_ethers_u256(volatility),
            time_remaining: from_ethers_u256(time_remaining),
        });

        let solver_address = self
            .log_normal_solver_address
            .ok_or(Error::msg("Solver address not set"))?;
        let solver = self.get_solver(client.clone(), solver_address).await?;
        let internal_price = solver.internal_price(parsed_pool_id).call().await?;

        let internal_price = if internal_price.is_zero() {
            None
        } else {
            Some(vec![(current_block, from_ethers_u256(internal_price))])
        };

        Ok(PoolState {
            id: Some(from_ethers_u256(parsed_pool_id)),
            controller: Some(from_ethers_address(controller)),
            strategy: Some(from_ethers_address(strategy)),
            asset_token: Some(from_ethers_address(asset_token)),
            quote_token: Some(from_ethers_address(quote_token)),
            liquidity_token: Some(from_ethers_address(liquidity_token)),
            internal_price,
            total_liquidity,
            asset_reserve,
            quote_reserve,
            liquidity_token_total_supply,
            log_normal_strategy,
            g3m_strategy: None,
            swap_fee_wad: Some(from_ethers_u256(swap_fee_wad)),
        })
    }

    async fn fetch_external_price<M: Middleware + 'static>(
        &self,
        client: Arc<M>,
    ) -> Result<AlloyU256> {
        let external_exchange = self
            .external_exchange_address
            .ok_or(Error::msg("External exchange address not set"))?;

        // todo: replace

        let lex = arbiter_bindings::bindings::liquid_exchange::LiquidExchange::new(
            to_ethers_address(external_exchange),
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

    /// todo: external exchange contract needs updates for this
    async fn fetch_external_price_of_token<M: Middleware + 'static>(
        &self,
        client: Arc<M>,
        _token_address: AlloyAddress,
    ) -> Result<AlloyU256> {
        let external_exchange = self
            .external_exchange_address
            .ok_or(Error::msg("External exchange address not set"))?;

        // todo: replace

        let lex = arbiter_bindings::bindings::liquid_exchange::LiquidExchange::new(
            to_ethers_address(external_exchange),
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

    // Protocol state

    async fn fetch_reserves_and_liquidity<M: Middleware + 'static>(
        &self,
        client: Arc<M>,
    ) -> Result<(AlloyU256, AlloyU256, AlloyU256)> {
        let protocol = self.protocol(client.clone()).await?;

        let total_pools = protocol.nonce().call().await?;

        // Returns early with 0 values if there are no pools.
        // todo: handle this case a little better
        if total_pools.is_zero() {
            return Ok((AlloyU256::ZERO, AlloyU256::ZERO, AlloyU256::ZERO));
        }

        let result = protocol.get_reserves_and_liquidity(U256::from(0)).await;
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
        let solver = self.get_log_normal_solver(client.clone())?;
        let internal_price = solver
            .internal_price(ethers::types::U256::from(0))
            .call()
            .await;
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
        let solver = self.get_log_normal_solver(client.clone())?;
        let pool_params = solver
            .fetch_pool_params(ethers::types::U256::from(0))
            .call()
            .await?;
        let strike_price = from_ethers_u256(pool_params.strike);
        let volatility = from_ethers_u256(pool_params.sigma);
        let time_remaining = from_ethers_u256(pool_params.tau);
        Ok((strike_price, volatility, time_remaining))
    }

    // ----- Getters ----- //

    pub fn get_pool_state(&self, pool_id: u64) -> Result<PoolState<AlloyAddress, AlloyU256>> {
        Ok(self
            .pool_state
            .as_ref()
            .ok_or(Error::msg("Pool state not set"))?
            .get(&pool_id)
            .cloned()
            .ok_or(Error::msg(format!("Pool with id {} not found", pool_id)))?)
    }

    pub fn price_of_token(&self, token_address: AlloyAddress) -> Result<AlloyU256> {
        let external_prices = self
            .external_prices
            .as_ref()
            .ok_or(Error::msg("External prices not set"))?;
        let price_series = external_prices
            .get(&token_address)
            .ok_or(Error::msg(format!(
                "Missing external price series for token address {}",
                token_address
            )))?;
        let price = price_series
            .last()
            .ok_or(Error::msg(format!(
                "Missing external price for token address {}",
                token_address
            )))?
            .1;
        Ok(price)
    }

    pub fn get_external_price_of_pool_asset(&self, pool_id: u64) -> Result<AlloyU256> {
        let pool_state = self.get_pool_state(pool_id)?;
        let asset_token = pool_state
            .asset_token
            .ok_or(Error::msg("Asset token address not set for pool state"))?;
        let external_price = self
            .external_prices
            .as_ref()
            .unwrap()
            .get(&asset_token)
            .ok_or(Error::msg(format!(
                "Missing external price series for asset token address {}",
                asset_token
            )))?
            .last()
            .ok_or(Error::msg(format!(
                "Missing external price for asset token address {}",
                asset_token
            )))?
            .1;
        Ok(external_price)
    }

    pub fn get_internal_price_of_pool_asset(&self, pool_id: u64) -> Result<AlloyU256> {
        let pool_state = self.get_pool_state(pool_id)?;
        let internal_price = pool_state
            .internal_price
            .as_ref()
            .ok_or(Error::msg("Internal price series not set for pool state"))?
            .last()
            .ok_or(Error::msg("Missing internal price for pool state"))?
            .1;
        Ok(internal_price)
    }

    // todo: figure out how to get the price of the token we care about/focused on.
    pub fn get_current_price(&self) -> Result<AlloyU256> {
        // Get the external price of the ETH token.
        let token_address = self
            .token_metadata
            .as_ref()
            .ok_or(Error::msg("Token metadata not set"))?
            .keys()
            .next()
            .unwrap();

        let external_price = self
            .external_prices
            .as_ref()
            .ok_or(Error::msg("External prices not set"))?
            .get(token_address)
            .unwrap()
            .last()
            .unwrap()
            .1;

        Ok(external_price)
    }

    /// Gets the series of USD values for each token the user has with a
    /// non-zero balance.
    pub fn get_user_balances_usd(&self) -> Result<Vec<(AlloyAddress, Vec<(u64, AlloyU256)>)>> {
        // Filter the liquidity token addresses from the user_token_balances
        // keys, then separate them into two different vectors of addresses.
        let liquidity_token_addresses = self
            .liquidity_token_addresses
            .as_ref()
            .cloned()
            .unwrap_or(Vec::new());

        let non_liquidity_token_addresses: Vec<_> = self
            .user_token_balances
            .keys()
            .cloned()
            .filter(|address| !liquidity_token_addresses.contains(address))
            .collect();

        // For each non-liquidity token address, then for each element in the series,
        // build a series of balance * price.
        let mut all_token_balances_usd = vec![];
        for token_address in non_liquidity_token_addresses {
            let balance_series = self.user_token_balances.get(&token_address).unwrap();
            let price_series = self
                .external_prices
                .as_ref()
                .unwrap()
                .get(&token_address)
                .unwrap();

            let balance_usd_series = balance_series
                .iter()
                .zip(price_series.iter())
                .map(|((block, balance), (_, price))| {
                    let balance_usd = balance
                        .checked_mul(*price)
                        .ok_or(anyhow!(RawDataModelError::CheckedMul))?
                        .checked_div(ALLOY_WAD)
                        .ok_or(anyhow!(RawDataModelError::CheckedDiv))?;

                    // temp override until lex is updated.
                    // check if the token is a stable token and if so, set the price to 1.
                    let token_info = self
                        .token_metadata
                        .as_ref()
                        .unwrap()
                        .get(&token_address)
                        .ok_or(Error::msg(format!(
                            "Token info not set for token address {}",
                            token_address
                        )))?;
                    let is_stable = token_info.symbol == "Y"
                        || token_info.symbol == "TKNY"
                        || token_info.symbol == "USDC";
                    // use the token balance as the value since 1 token = 1 usd
                    let balance_usd = if is_stable { *balance } else { balance_usd };

                    Ok((*block, balance_usd))
                })
                .collect::<Result<Vec<_>>>()?;

            all_token_balances_usd.push((token_address, balance_usd_series));
        }

        Ok(all_token_balances_usd.clone())
    }

    /// Computes the amount of reserves claimable in total given all the
    /// liquidity.
    fn get_reserves(&self, pool_id: u64) -> Result<(Vec<(u64, AlloyU256)>, Vec<(u64, AlloyU256)>)> {
        // Get the pool state of the given pool id.
        let pool_state = self.get_pool_state(pool_id)?;

        // Get the reserves of the pool.
        let asset_reserve_series = pool_state
            .asset_reserve
            .ok_or(Error::msg("Asset reserve series not set for pool state"))?;

        let quote_reserve_series = pool_state
            .quote_reserve
            .ok_or(Error::msg("Quote reserve series not set for pool state"))?;

        // Return the new series of the user's claimable token balance in USD, which can
        // be summed to find the allocated position's value over time.
        Ok((asset_reserve_series, quote_reserve_series))
    }

    /// Gets the protocol contract instance given the model's protocol address.
    pub async fn protocol<M: Middleware + 'static>(&self, client: Arc<M>) -> Result<DFMM<M>> {
        let protocol_address = self
            .dfmm_address
            .ok_or(Error::msg("Protocol address not set"))?;
        let converted_address = to_ethers_address(protocol_address);
        let protocol = DFMM::new(converted_address, client.clone());
        Ok(protocol)
    }

    /// Gets the strategy contract instance given a pool's strategy address.
    pub fn log_normal_strategy<M: Middleware + 'static>(
        &self,
        client: Arc<M>,
        address: AlloyAddress,
    ) -> Result<LogNormal<M>> {
        let strategy = LogNormal::new(to_ethers_address(address), client.clone());
        Ok(strategy)
    }

    /// Gets a pool's strategy's solver contract instance given an address.
    pub async fn get_solver<M: Middleware + 'static>(
        &self,
        client: Arc<M>,
        address: AlloyAddress,
    ) -> Result<LogNormalSolver<M>> {
        let solver = LogNormalSolver::new(to_ethers_address(address), client.clone());
        Ok(solver)
    }

    /// For each token in the coin_list, gets the user's balance and the
    /// external price of the token using the model's saved data in the
    /// respective mappings.
    pub fn get_unallocated_positions(
        &self,
        coin_list: CoinList,
    ) -> Result<Vec<UnallocatedPosition>> {
        let mut coins = coin_list.tokens;
        if coins.is_empty() {
            return Ok(Vec::new());
        }

        // Get the liquidity tokens and filter them from the coins list, so that we
        // don't mistakenly make the liquidity tokens an unallocated position.
        let liquidity_token_addresses = self
            .liquidity_token_addresses
            .as_ref()
            .cloned()
            .unwrap_or(Vec::new());

        let coins = coins
            .drain(..)
            .filter(|coin| !liquidity_token_addresses.contains(&coin.address))
            .collect::<Vec<_>>();

        let mut unallocated_positions = vec![];
        for coin in coins {
            let token_address = coin.address;
            let balance = self.user_token_balances[&token_address]
                .last()
                .map(|(_, balance)| balance)
                .unwrap_or(&AlloyU256::ZERO);

            let external_price = self.external_prices.as_ref().ok_or(Error::msg(
                "get_unallocated_positions: External price series not set",
            ))?[&token_address]
                .last()
                .map(|(_, price)| price)
                .unwrap_or(&AlloyU256::ZERO);

            // todo: temp override until lex is fixed.
            let external_price = match coin.tags.contains(&"stablecoin".to_string()) {
                true => ALLOY_WAD,
                false => *external_price,
            };

            unallocated_positions.push(UnallocatedPosition {
                token_address,
                balance: format_and_parse(*balance)?,
                external_price: format_and_parse(external_price)?,
            });
        }

        Ok(unallocated_positions)
    }

    /// Gets the pool id of a given liquidity token.
    pub fn get_pool_id_of_liquidity_token(
        &self,
        liquidity_token_address: AlloyAddress,
    ) -> Result<u64> {
        let pool_state_map = self.pool_state.as_ref().ok_or(Error::msg(
            "Pool state is not set when attempting to find pool of liquidity token.",
        ))?;
        let pool_id = pool_state_map
            .iter()
            .find(|(_, pool_state)| {
                pool_state
                    .liquidity_token
                    .map(|address| address.eq(&liquidity_token_address))
                    .unwrap_or(false)
            })
            .map(|(pool_id, _)| *pool_id)
            .ok_or(Error::msg(format!(
                "Missing pool id for liquidity token address {}",
                liquidity_token_address
            )))?;

        Ok(pool_id)
    }

    /// Filters all the tokens in `token_balance_mapping` for the liquidity
    /// tokens, then gets the user's balance of the liquidity tokens and
    /// derives their value using `derive_user_allocated_position_usd`.
    /// Returns a vector of the token's balance and value for each liquidity
    /// token.
    pub fn get_allocated_positions(&self) -> Result<Vec<AllocatedPosition>> {
        let filtered_tokens = self
            .liquidity_token_addresses
            .as_ref()
            .cloned()
            .unwrap_or(Vec::new());

        if filtered_tokens.is_empty() {
            return Ok(Vec::new());
        }

        let mut allocated_positions = vec![];
        for liquidity_token in filtered_tokens {
            let balance = self.user_token_balances[&liquidity_token]
                .last()
                .map(|(_, balance)| balance)
                .unwrap_or(&AlloyU256::ZERO);
            let pool_id = self.get_pool_id_of_liquidity_token(liquidity_token)?;
            let pool_state = self.get_pool_state(pool_id)?.clone();
            let (token_x_address, token_y_address) = (
                pool_state
                    .asset_token
                    .ok_or(Error::msg("Asset token address not set for pool state"))?,
                pool_state
                    .quote_token
                    .ok_or(Error::msg("Quote token address not set for pool state"))?,
            );
            let internal_asset_price = self.get_internal_price_of_pool_asset(pool_id)?;
            let (claimable_assets, claimable_quotes) =
                self.derive_user_allocated_balances(pool_id)?;
            let claimable_assets = claimable_assets
                .last()
                .map(|(_, balance)| balance)
                .unwrap_or(&AlloyU256::ZERO);
            let claimable_quotes = claimable_quotes
                .last()
                .map(|(_, balance)| balance)
                .unwrap_or(&AlloyU256::ZERO);
            let external_asset_price = self.external_prices.as_ref().ok_or(Error::msg(
                "get_allocated_positions: External price series not set",
            ))?[&token_x_address]
                .last()
                .map(|(_, price)| price)
                .unwrap_or(&AlloyU256::ZERO);
            let external_quote_price = ALLOY_WAD; // todo: fix price of other tokens in lex.
            let asset_value = claimable_assets
                .checked_mul(*external_asset_price)
                .ok_or(anyhow!(RawDataModelError::CheckedMul))?
                .checked_div(ALLOY_WAD)
                .ok_or(anyhow!(RawDataModelError::CheckedDiv))?;
            let quote_value = claimable_quotes
                .checked_mul(external_quote_price)
                .ok_or(anyhow!(RawDataModelError::CheckedMul))?
                .checked_div(ALLOY_WAD)
                .ok_or(anyhow!(RawDataModelError::CheckedDiv))?;
            let value = asset_value
                .checked_add(quote_value)
                .ok_or(anyhow!(RawDataModelError::CheckedAdd))?;
            let allocated_position = AllocatedPosition {
                token_x_address,
                token_y_address,
                token_l_address: liquidity_token,
                claimable_balance_x: format_and_parse(*claimable_assets)?,
                claimable_balance_y: format_and_parse(*claimable_quotes)?,
                liquidity_balance: format_and_parse(*balance)?,
                liquidity_value: format_and_parse(value)?,
                external_price: format_and_parse(*external_asset_price)?,
                internal_price: format_and_parse(internal_asset_price)?,
            };
            allocated_positions.push(allocated_position);
        }

        Ok(allocated_positions)
    }

    /// Gets the balances and prices of the asset and quote tokens and formats
    /// them into floats.
    pub fn get_position_info(&self, pool_id: u64) -> Result<StrategyPosition> {
        let pool_state = self.get_pool_state(pool_id)?.clone();

        let (claimable_assets, claimable_quotes) = self.derive_user_allocated_balances(pool_id)?;
        let balance_x = claimable_assets
            .last()
            .map(|(_, balance)| balance)
            .unwrap_or(&AlloyU256::ZERO);
        let balance_y = claimable_quotes
            .last()
            .map(|(_, balance)| balance)
            .unwrap_or(&AlloyU256::ZERO);

        let internal_price = pool_state
            .internal_price
            .as_ref()
            .ok_or(Error::msg(
                "get_position_info: Internal price series not set for pool state",
            ))?
            .last()
            .map(|(_, price)| price)
            .unwrap_or(&AlloyU256::ZERO);

        let liquidity = pool_state
            .total_liquidity
            .as_ref()
            .map(|series| {
                series
                    .last()
                    .map(|(_, liquidity)| liquidity)
                    .unwrap_or(&AlloyU256::ZERO)
            })
            .unwrap_or(&AlloyU256::ZERO);

        let quote_price = ALLOY_WAD; // todo: fix price of other tokens in lex.
        let asset_token = pool_state.asset_token.ok_or(Error::msg(
            "get_position_info: Asset token address not set for pool state",
        ))?;
        let external_price = self.external_prices.as_ref().ok_or(Error::msg(
            "get_position_info: External price series not set",
        ))?[&asset_token]
            .last()
            .map(|(_, price)| price)
            .unwrap_or(&AlloyU256::ZERO);

        let external_price = format_and_parse(*external_price)?;
        let balance_x = format_and_parse(*balance_x)?;
        let balance_y = format_and_parse(*balance_y)?;
        let internal_price = format_and_parse(*internal_price)?;
        let liquidity = format_and_parse(*liquidity)?;
        let quote_price = format_and_parse(quote_price)?;

        Ok(StrategyPosition {
            balance_x,
            balance_y,
            liquidity,
            external_price,
            internal_price,
            quote_price,
        })
    }

    // ----- Derivations ----- //

    /// Computes the value of an allocated position of the given pool id.
    /// Sums the value of the position's claimable token balance and returns the
    /// series.
    fn derive_position_value_series(
        &self,
        pool_id: u64,
    ) -> Result<(Vec<(u64, AlloyU256)>, Vec<(u64, AlloyU256)>)> {
        // Get the pool state of the given pool id.
        let pool_state = self.get_pool_state(pool_id)?;

        // Get the quantity of tokens claimable via the liquidity tokens owned by the
        // user.
        let (claimable_asset_balance_series, claimable_quote_balance_series) =
            self.derive_user_allocated_balances(pool_id)?;

        // Get the external price series for the asset token.
        let asset_token_address = pool_state
            .asset_token
            .ok_or(Error::msg("Asset token address not set for pool state"))?;
        let asset_price_series = self
            .external_prices
            .as_ref()
            .unwrap()
            .get(&asset_token_address)
            .ok_or(Error::msg(format!(
                "External price series not set for asset token address {}",
                asset_token_address
            )))?;

        // todo: assumes the current price of the quote token is 1, lex needs to be
        // updated to fix this. Make a copy of the quote series but for prices
        // of ALLOY_WAD
        let quote_price_series = claimable_quote_balance_series
            .iter()
            .map(|(block, _)| (*block, ALLOY_WAD))
            .collect::<Vec<_>>();

        // Make new series' which have the computed usd values using the price series'.
        let claimable_asset_balance_usd_series = claimable_asset_balance_series
            .iter()
            .zip(asset_price_series.iter())
            .map(
                |((block_balance, claimable_asset_balance), (block_price, asset_price))| {
                    // Throw an error if the block numbers don't match.
                    if block_balance != block_price {
                        return Err(anyhow!(RawDataModelError::BlockNumberMismatch(
                            "derive_position_value_series: asset".to_string(),
                        )));
                    }

                    let claimable_asset_balance_usd = claimable_asset_balance
                        .checked_mul(*asset_price)
                        .ok_or(anyhow!(RawDataModelError::CheckedMul))?
                        .checked_div(ALLOY_WAD)
                        .ok_or(anyhow!(RawDataModelError::CheckedDiv))?;
                    Ok((*block_balance, claimable_asset_balance_usd))
                },
            )
            .collect::<Result<Vec<_>>>()?;

        let claimable_quote_balance_usd_series = claimable_quote_balance_series
            .iter()
            .zip(quote_price_series.iter())
            .map(
                |((block_balance, claimable_quote_balance), (block_price, quote_price))| {
                    // Throw an error if the block numbers don't match.
                    if block_balance != block_price {
                        return Err(anyhow!(RawDataModelError::BlockNumberMismatch(
                            "derive_position_value_series: quote".to_string(),
                        )));
                    }

                    let claimable_quote_balance_usd = claimable_quote_balance
                        .checked_mul(*quote_price)
                        .ok_or(anyhow!(RawDataModelError::CheckedMul))?
                        .checked_div(ALLOY_WAD)
                        .ok_or(anyhow!(RawDataModelError::CheckedDiv))?;

                    Ok((*block_balance, claimable_quote_balance_usd))
                },
            )
            .collect::<Result<Vec<_>>>()?;

        // Return the new series of the user's claimable token balance in USD, which can
        // be summed to find the allocated position's value over time.
        Ok((
            claimable_asset_balance_usd_series,
            claimable_quote_balance_usd_series,
        ))
    }

    /// Computes the amount of reserves claimable given the user's liquidity.
    fn derive_user_allocated_balances(
        &self,
        pool_id: u64,
    ) -> Result<(Vec<(u64, AlloyU256)>, Vec<(u64, AlloyU256)>)> {
        // Get the pool state of the given pool id.
        let pool_state = self
            .pool_state
            .as_ref()
            .unwrap()
            .get(&pool_id)
            .ok_or(Error::msg(format!(
                "Pool state not set for pool id {}",
                pool_id
            )))?
            .clone();

        // Get the liquidity token balance series for the given pool id of the user.
        let liquidity_token_address = pool_state
            .liquidity_token
            .ok_or(Error::msg("Liquidity token address not set for pool state"))?;

        let liquidity_token_balance_series = self
            .user_token_balances
            .get(&liquidity_token_address)
            .ok_or(Error::msg(format!(
                "Liquidity token balance series not set for pool id {}",
                pool_id
            )))?;

        // Get the total liquidity token supply series for the given pool id.
        let liquidity_token_total_supply_series = pool_state.liquidity_token_total_supply.ok_or(
            Error::msg("Liquidity token total supply series not set for pool state"),
        )?;

        // Get the reserves of the pool.
        let asset_reserve_series = pool_state
            .asset_reserve
            .ok_or(Error::msg("Asset reserve series not set for pool state"))?;

        let quote_reserve_series = pool_state
            .quote_reserve
            .ok_or(Error::msg("Quote reserve series not set for pool state"))?;

        // Compute new series of the user's claimable token balance by multiplying
        // the user's liquidity token balance by the respective reserves, then dividing
        // by the total liquidity token supply.
        let claimable_asset_balance_series = liquidity_token_balance_series
            .iter()
            .zip(asset_reserve_series.iter())
            .zip(liquidity_token_total_supply_series.iter())
            .map(
                |(((block, liquidity_token_balance), (_, asset_reserve)), (_, total_supply))| {
                    let claimable_asset_balance = liquidity_token_balance
                        .checked_mul(*asset_reserve)
                        .ok_or(anyhow!(RawDataModelError::CheckedMul))?
                        .checked_div(*total_supply)
                        .ok_or(anyhow!(RawDataModelError::CheckedDiv))?;
                    Ok((*block, claimable_asset_balance))
                },
            )
            .collect::<Result<Vec<_>>>()?;

        let claimable_quote_balance_series = liquidity_token_balance_series
            .iter()
            .zip(quote_reserve_series.iter())
            .zip(liquidity_token_total_supply_series.iter())
            .map(
                |(((block, liquidity_token_balance), (_, quote_reserve)), (_, total_supply))| {
                    let claimable_quote_balance = liquidity_token_balance
                        .checked_mul(*quote_reserve)
                        .ok_or(anyhow!(RawDataModelError::CheckedMul))?
                        .checked_div(*total_supply)
                        .ok_or(anyhow!(RawDataModelError::CheckedDiv))?;
                    Ok((*block, claimable_quote_balance))
                },
            )
            .collect::<Result<Vec<_>>>()?;

        // Return the new series of the user's claimable token balance in USD, which can
        // be summed to find the allocated position's value over time.
        Ok((
            claimable_asset_balance_series,
            claimable_quote_balance_series,
        ))
    }

    /// Computes the price of each lp token in USD using the external prices of
    /// the tokens stored in the external prices mapping.
    pub fn derive_lp_token_price_series(&self, pool_id: u64) -> Result<Vec<(u64, AlloyU256)>> {
        let pool_state = self.get_pool_state(pool_id)?.clone();
        let (claimable_assets, claimable_quotes) = self.get_reserves(pool_id)?;
        let claimable_assets_series = claimable_assets
            .into_iter()
            .map(|(block, balance)| (block, balance))
            .collect::<Vec<_>>();
        let claimable_quotes_series = claimable_quotes
            .into_iter()
            .map(|(block, balance)| (block, balance))
            .collect::<Vec<_>>();
        let external_asset_price_series = self.external_prices.as_ref().ok_or(Error::msg(
            "derive_lp_token_price_series: External price series not set",
        ))?[&pool_state.asset_token.unwrap()]
            .clone();
        let external_quote_price = ALLOY_WAD; // todo: fix price of other tokens in lex.
        let total_supply_series = pool_state
            .liquidity_token_total_supply
            .ok_or(Error::msg(
                "Liquidity token total supply series not set for pool state",
            ))?
            .clone();

        // Align the series to match properly
        // this is very important! external asset price series has a longer history most
        // likely, so if the series are not aligned it will mismatch the values.
        let min_block_number = claimable_assets_series
            .first()
            .map(|(block, _)| *block)
            .unwrap_or(0);
        let external_asset_price_series = external_asset_price_series
            .into_iter()
            .filter(|(block, _)| *block >= min_block_number)
            .collect::<Vec<_>>();
        let lp_token_price_series = claimable_assets_series
            .iter()
            .zip(claimable_quotes_series.iter())
            .zip(external_asset_price_series.iter())
            .zip(total_supply_series.iter())
            .map(
                |(
                    (
                        ((block_asset, claimable_assets), (block_quote, claimable_quotes)),
                        (block_asset_price, external_asset_price),
                    ),
                    (block_total_supply, total_supply),
                )| {
                    // Make sure the blocks match.
                    if block_asset != block_quote
                        || block_asset != block_asset_price
                        || block_asset != block_total_supply
                    {
                        return Err(anyhow!(RawDataModelError::BlockNumberMismatch(format!(
                            "lp_token_price_series: asset {}, quote {}, asset price {}, supply {}",
                            block_asset, block_quote, block_asset_price, block_total_supply
                        ))));
                    }

                    let asset_value = claimable_assets
                        .checked_mul(*external_asset_price)
                        .ok_or(anyhow!(RawDataModelError::CheckedMul))?
                        .checked_div(ALLOY_WAD)
                        .ok_or(anyhow!(RawDataModelError::CheckedDiv))?;
                    let quote_value = claimable_quotes
                        .checked_mul(external_quote_price)
                        .ok_or(anyhow!(RawDataModelError::CheckedMul))?
                        .checked_div(ALLOY_WAD)
                        .ok_or(anyhow!(RawDataModelError::CheckedDiv))?;
                    let total_value = asset_value
                        .checked_add(quote_value)
                        .ok_or(anyhow!(RawDataModelError::CheckedAdd))?;
                    let price_per_lp = total_value
                        .checked_mul(ALLOY_WAD)
                        .ok_or(anyhow!(RawDataModelError::CheckedMul))?
                        .checked_div(*total_supply)
                        .ok_or(anyhow!(RawDataModelError::CheckedDiv))?;

                    Ok((*block_asset, price_per_lp))
                },
            )
            .collect::<Result<Vec<_>>>()?;

        Ok(lp_token_price_series)
    }

    /// Gets the "unallocated position" balances.
    /// todo: work on this to make it more accurate. Probably need to refactor
    /// position types a bit.
    pub fn get_unallocated_positions_info(&self, coin_list: CoinList) -> Result<StrategyPosition> {
        let coins = coin_list.tokens;
        if coins.len() != 2 {
            return Err(Error::msg(
                "get_position_info: Coin list must have 2 tokens",
            ));
        }
        let asset_token = coins[0].address;
        let quote_token = coins[1].address;

        let balance_x = self.user_token_balances[&asset_token]
            .last()
            .map(|(_, balance)| balance)
            .unwrap_or(&AlloyU256::ZERO);

        let balance_y = self.user_token_balances[&quote_token]
            .last()
            .map(|(_, balance)| balance)
            .unwrap_or(&AlloyU256::ZERO);
        let external_price = self.external_prices.as_ref().ok_or(Error::msg(
            "get_position_info: External price series not set",
        ))?[&asset_token]
            .last()
            .map(|(_, price)| price)
            .unwrap_or(&AlloyU256::ZERO);
        let quote_price = ALLOY_WAD; // todo: fix price of other tokens in lex.

        let external_price = format_and_parse(*external_price)?;
        let balance_x = format_and_parse(*balance_x)?;
        let balance_y = format_and_parse(*balance_y)?;
        let quote_price = format_and_parse(quote_price)?;

        Ok(StrategyPosition {
            balance_x,
            balance_y,
            liquidity: 0.0,
            external_price,
            internal_price: 0.0,
            quote_price,
        })
    }

    fn get_log_normal_solver<M: Middleware + 'static>(
        &self,
        client: Arc<M>,
    ) -> Result<LogNormalSolver<M>> {
        let solver_address = self
            .log_normal_solver_address
            .ok_or(Error::msg("Solver address not set"))?;
        let solver = LogNormalSolver::new(to_ethers_address(solver_address), client.clone());
        Ok(solver)
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
            .ok_or(anyhow!(RawDataModelError::CheckedMul))?
            .checked_div(ALLOY_WAD)
            .ok_or(anyhow!(RawDataModelError::CheckedDiv))?;

        let quote_value = quote_balance_wad
            .checked_mul(quote_price_wad)
            .ok_or(anyhow!(RawDataModelError::CheckedMul))?
            .checked_div(ALLOY_WAD)
            .ok_or(anyhow!(RawDataModelError::CheckedDiv))?;

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
        let quote_price = alloy_primitives::utils::format_ether(quote_price_wad);
        let quote_price = quote_price.parse::<f64>()?;
        let price = alloy_primitives::utils::format_ether(asset_price_wad);
        let price = price.parse::<f64>()?;
        let liquidity = alloy_primitives::utils::format_ether(total_liquidity_wad);
        let liquidity = liquidity.parse::<f64>()?;
        let strike_price = alloy_primitives::utils::format_ether(strike_price_wad);
        let strike_price = strike_price.parse::<f64>()?;
        let volatility = alloy_primitives::utils::format_ether(volatility_percent_wad);
        let volatility = volatility.parse::<f64>()?;
        let time_remaining = alloy_primitives::utils::format_ether(time_remaining_years_wad);
        let time_remaining = time_remaining.parse::<f64>()?;

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
    pub fn derive_total_aum(&self, pool_id: u64) -> Result<AlloyU256> {
        // todo: this naming is confusing but the external portfolio value is the
        // unallocated value.
        // todo: maybe handle this more explicitly?
        let external_portfolio_value = self
            .derive_external_portfolio_value(pool_id)
            .unwrap_or_default();
        // todo: handle the scenario that the user has the ability to create an
        // allocated position with unallocated tokens, track that?
        // let unallocated_position_value = self.derive_unallocated_position_value()?;

        let unallocated_values_series = self.get_user_balances_usd();
        let unallocated_values_series = match unallocated_values_series {
            Ok(unallocated_values_series) => unallocated_values_series,
            Err(error) => {
                tracing::warn!("Error fetching unallocated position value: {:?}", error);
                // set value to 0
                vec![]
            }
        };
        // Make a new series of all the last values of the series in this vec
        let unallocated_values_series = unallocated_values_series
            .iter()
            .map(|series| series.1.last().unwrap().1)
            .collect::<Vec<_>>();
        // Sum the series
        let unallocated_position_value =
            unallocated_values_series
                .iter()
                .try_fold(AlloyU256::ZERO, |acc, value| {
                    acc.checked_add(*value)
                        .ok_or_else(|| anyhow!("Failed to add unallocated values"))
                })?;

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
    pub fn derive_unallocated_position_value(&self, coin_list: CoinList) -> Result<AlloyU256> {
        let unallocated_position = self.get_unallocated_positions_info(coin_list)?;
        let unallocated_position_value = unallocated_position.compute_value()?;
        let unallocated_position_value =
            alloy_primitives::utils::parse_ether(&format!("{}", unallocated_position_value))?;
        Ok(unallocated_position_value)
    }

    /// Computes the portfolio value of the user's strategy deposits according
    /// to an external price.
    pub fn derive_external_portfolio_value(&self, pool_id: u64) -> Result<AlloyU256> {
        let pool_state = self.get_pool_state(pool_id)?;
        let (asset_token, quote_token) = (
            pool_state.asset_token.ok_or(Error::msg(
                "derive_external_portfolio_value: Asset token not set",
            ))?,
            pool_state.quote_token.ok_or(Error::msg(
                "derive_external_portfolio_value: Quote token not set",
            ))?,
        );
        let asset_price_wad = self
            .external_prices
            .as_ref()
            .ok_or(Error::msg("External prices not set"))?[&asset_token]
            .last()
            .unwrap()
            .1;
        // todo: hardcoded stablecoin price
        let quote_price_wad = ALLOY_WAD;
        let (claimable_assets, claimable_quotes) = self.derive_user_allocated_balances(pool_id)?;
        let (asset_reserve_wad, quote_reserve_wad) = (
            claimable_assets
                .last()
                .ok_or(Error::msg(
                    "derive_external_portfolio_value: Asset reserve not set",
                ))?
                .1,
            claimable_quotes
                .last()
                .ok_or(Error::msg(
                    "derive_external_portfolio_value: Quote reserve not set",
                ))?
                .1,
        );

        Self::compute_portfolio_value_real(
            asset_price_wad,
            quote_price_wad,
            quote_reserve_wad,
            asset_reserve_wad,
        )
    }

    /// Computes the portfolio value of the user's deposits in a strategy
    /// according to the internal price.
    pub fn derive_internal_portfolio_value(&self, pool_id: u64) -> Result<AlloyU256> {
        let pool_state = self.get_pool_state(pool_id)?;
        let internal_price = pool_state
            .internal_price
            .ok_or(Error::msg(
                "derive_internal_portfolio_value: Internal price not set",
            ))?
            .last()
            .unwrap()
            .1;

        let quote_token = pool_state.quote_token.ok_or(Error::msg(
            "derive_internal_portfolio_value: Quote token not set",
        ))?;

        let quote_price = ALLOY_WAD; // todo: fix quote price

        // todo: using the global reserves of the market for now.
        let (claimable_assets, claimable_quotes) = self.derive_user_allocated_balances(pool_id)?;
        let asset_balance_wad = claimable_assets
            .last()
            .ok_or(Error::msg(
                "derive_internal_portfolio_value: Asset balance not set",
            ))?
            .1;
        let quote_balance_wad = claimable_quotes
            .last()
            .ok_or(Error::msg(
                "derive_internal_portfolio_value: Quote balance not set",
            ))?
            .1;

        Self::compute_portfolio_value_real(
            internal_price,
            quote_price,
            quote_balance_wad,
            asset_balance_wad,
        )
    }

    /// Computes the theoretical portfolio value given the strategy parameters,
    /// external market price, and amount of liquidity.
    pub fn derive_theoretical_portfolio_value(&self, pool_id: u64) -> Result<AlloyU256> {
        let pool_state = self.get_pool_state(pool_id)?;

        let params = pool_state.log_normal_strategy.ok_or(Error::msg(
            "derive_theoretical_portfolio_value: Log normal strategy not set",
        ))?;

        let (strike_price_wad, volatility_wad, time_remaining_wad) = (
            params.strike_price,
            params.volatility,
            params.time_remaining,
        );

        let asset_token = pool_state.asset_token.ok_or(Error::msg(
            "derive_theoretical_portfolio_value: Asset token not set",
        ))?;

        let quote_token = pool_state.quote_token.ok_or(Error::msg(
            "derive_theoretical_portfolio_value: Quote token not set",
        ))?;

        let asset_price_wad = self.external_prices.as_ref().ok_or(Error::msg(
            "derive_theoretical_portfolio_value: External prices not set",
        ))?[&asset_token]
            .last()
            .unwrap()
            .1;

        let quote_price_wad = self.external_prices.as_ref().ok_or(Error::msg(
            "derive_theoretical_portfolio_value: External prices not set",
        ))?[&quote_token]
            .last()
            .unwrap()
            .1;

        let total_liquidity_wad = pool_state
            .total_liquidity
            .ok_or(Error::msg(
                "derive_theoretical_portfolio_value: Total liquidity not set",
            ))?
            .last()
            .unwrap()
            .1;

        Self::compute_portfolio_value_theoretical(
            asset_price_wad,
            quote_price_wad,
            total_liquidity_wad,
            strike_price_wad,
            volatility_wad,
            time_remaining_wad,
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
            .ok_or(anyhow!(RawDataModelError::CheckedMul))?
            .checked_div(theoretical_value_wad)
            .ok_or(anyhow!(RawDataModelError::CheckedDiv))?;

        Ok(health)
    }

    /// Computes the health of the user's portfolio.
    pub fn derive_portfolio_health(&self) -> Result<AlloyU256> {
        let pool_id = 0; // todo: fix pool_id
        let internal_portfolio_value_wad = self.derive_internal_portfolio_value(pool_id)?;
        let theoretical_value_wad = self.derive_theoretical_portfolio_value(pool_id)?;

        Self::compute_health(internal_portfolio_value_wad, theoretical_value_wad)
    }

    /// Transforms series data in native types to chart types.
    pub fn transform_series_over_block_number(
        series: Vec<(u64, AlloyU256)>,
    ) -> Result<(CartesianRanges, ChartLineSeries)> {
        if series.is_empty() {
            return Err(Error::msg("Series is empty"));
        }

        let mut transformed = Vec::new();

        for (block, value) in series {
            let block = block as f32;
            let value = alloy_primitives::utils::format_ether(value);
            let value = value.parse::<f32>()?;
            transformed.push((block, value));
        }

        // Get the ranges, which should be within 20% of the last value.
        let first_block = transformed.first().unwrap().0;
        let last_value = transformed.last().unwrap().1;

        let min_y = last_value - last_value * 0.2;
        let max_y = last_value + last_value * 0.2;

        let max_y = if max_y <= f32::EPSILON { 1.0 } else { max_y };

        let ranges = CartesianRanges {
            x_range: (first_block, first_block + 10.0),
            y_range: (min_y, max_y),
        };

        Ok((ranges, coords_to_line_series(transformed)))
    }

    /// Transforms x,y data in native types to chart types.
    pub fn transform_plot(data: &[(AlloyU256, AlloyU256)]) -> Result<ChartLineSeries> {
        let mut transformed = Vec::new();

        for (x, y) in data {
            let x = alloy_primitives::utils::format_ether(*x);
            let x = x.parse::<f32>()?;
            let y = alloy_primitives::utils::format_ether(*y);
            let y = y.parse::<f32>()?;
            transformed.push((x, y));
        }

        Ok(coords_to_line_series(transformed))
    }

    /// Transforms the portfolio value series into a chart series that can be
    /// plotted by the view logic.
    pub fn derive_portfolio_value_series(
        &self,
        pool_id: u64,
    ) -> Result<(CartesianRanges, ChartLineSeries)> {
        let lp_token_price_series = self.derive_lp_token_price_series(pool_id)?;
        let pool_state = self.get_pool_state(pool_id)?;
        let liquidity_token = pool_state
            .liquidity_token
            .ok_or(Error::msg("Liquidity token not set"))?;
        let lp_token_balances =
            self.user_token_balances
                .get(&liquidity_token)
                .ok_or(Error::msg(
                    "Liquidity token balances not set for user token balances",
                ))?;

        // Multiply the values into a single series.
        let series = lp_token_price_series
            .iter()
            .zip(lp_token_balances.iter())
            .map(|((block, lp_token_price), (_, lp_token_balance))| {
                let portfolio_value = lp_token_price
                    .checked_mul(*lp_token_balance)
                    .ok_or(anyhow!(
                        "Failed to multiply lp token price and lp token balance: {} * {}",
                        lp_token_price,
                        lp_token_balance
                    ))
                    .unwrap_or(AlloyU256::ZERO)
                    .checked_div(ALLOY_WAD)
                    .ok_or(anyhow!(
                        "Failed to divide lp token price and ALLOY_WAD: {} / {}",
                        lp_token_price,
                        ALLOY_WAD
                    ))
                    .unwrap_or(AlloyU256::ZERO);
                (*block, portfolio_value)
            })
            .collect::<Vec<(u64, AlloyU256)>>();

        let mut result = Self::transform_series_over_block_number(series)?;

        result.1.legend = "Portfolio Value".to_string();
        result.1.color = plotters::style::full_palette::DEEPPURPLE_400;
        result.1.time_series = true;

        Ok(result)
    }

    /// Gets the time series data for the unallocated portfolio value.
    pub fn derive_unallocated_portfolio_value_series(
        &self,
    ) -> Result<(CartesianRanges, ChartLineSeries)> {
        let all_usd_balances = self.get_user_balances_usd()?;

        // Combine the vector of (token, balance series) into a single series of (block,
        // balance sum).
        let mut unallocated_series: Vec<(u64, AlloyU256)> = Vec::new();
        for (_, balance_series) in all_usd_balances {
            for (block, balance) in balance_series {
                let mut found = false;
                for (i, (existing_block, existing_balance)) in
                    unallocated_series.iter_mut().enumerate()
                {
                    if *existing_block == block {
                        *existing_balance += balance;
                        found = true;
                        break;
                    }
                }
                if !found {
                    unallocated_series.push((block, balance));
                }
            }
        }

        let mut result = Self::transform_series_over_block_number(unallocated_series)?;

        result.1.legend = "Unallocated".to_string();
        result.1.color = plotters::style::full_palette::LIGHTBLUE_A400;
        result.1.time_series = true;

        Ok(result)
    }

    /// Gets the time series data for the protocol asset value series.
    pub fn derive_protocol_asset_value_series(
        &self,
        pool_id: u64,
    ) -> Result<(CartesianRanges, ChartLineSeries)> {
        let series = self.derive_position_value_series(pool_id)?.0;
        let mut result = Self::transform_series_over_block_number(series)?;

        result.1.legend = "Protocol Asset".to_string();
        result.1.color = plotters::style::full_palette::PURPLE_A700;
        result.1.time_series = true;

        Ok(result)
    }

    /// Gets the time series data for the protocol quote value series.
    pub fn derive_protocol_quote_value_series(
        &self,
        pool_id: u64,
    ) -> Result<(CartesianRanges, ChartLineSeries)> {
        let series = self.derive_position_value_series(pool_id)?.1;
        let mut result = Self::transform_series_over_block_number(series)?;

        result.1.legend = "Protocol Quote".to_string();
        result.1.color = plotters::style::full_palette::PURPLE_A400;
        result.1.time_series = true;

        Ok(result)
    }

    /// Gets the points of interest on the strategy plot.
    pub fn derive_portfolio_strategy_points(&self, pool_id: u64) -> Result<Vec<ChartPoint>> {
        let pool_state = self
            .pool_state
            .as_ref()
            .ok_or(Error::msg("Pool state not set"))?
            .get(&pool_id)
            .ok_or(Error::msg("Pool state not found"))?;

        let asset_token = pool_state
            .asset_token
            .clone()
            .ok_or(Error::msg("Asset token not set"))?;

        let last_asset_reserve = pool_state
            .asset_reserve
            .as_ref()
            .ok_or(Error::msg("Last asset reserve not set for pool state"))?
            .last()
            .unwrap();

        let last_quote_reserve = pool_state
            .quote_reserve
            .as_ref()
            .ok_or(Error::msg("Last quote reserve not set for pool state"))?
            .last()
            .unwrap();

        let last_total_liquidity = pool_state
            .total_liquidity
            .as_ref()
            .ok_or(Error::msg("Last total liquidity not set for pool state"))?
            .last()
            .unwrap();

        let asset_reserve = format_and_parse(last_asset_reserve.1)?;
        let quote_reserve = format_and_parse(last_quote_reserve.1)?;
        let total_liquidity = format_and_parse(last_total_liquidity.1)?;

        // todo: handle better!
        if total_liquidity == 0.0 {
            return Err(anyhow!("Total liquidity is 0"));
        }

        let internal_reserves = (
            (asset_reserve / total_liquidity) as f32,
            (quote_reserve / total_liquidity) as f32,
        );

        let external_asset_price = self
            .external_prices
            .clone()
            .ok_or(Error::msg(
                "External prices not set, cannot compute theoretical reserves",
            ))?
            .get(&asset_token)
            .ok_or(Error::msg(
                "External asset price not set, cannot compute theoretical reserves",
            ))?
            .last()
            .unwrap()
            .1;

        let spot_price_float = format_and_parse(external_asset_price)?;

        let params = pool_state.log_normal_strategy.clone().ok_or(Error::msg(
            "Log normal strategy not set, cannot compute theoretical reserves",
        ))?;

        let (strike_price_wad_float, sigma_percent_wad_float, time_to_expiry_years_wad_float) = (
            format_and_parse(params.strike_price)?,
            format_and_parse(params.volatility)?,
            format_and_parse(params.time_remaining)?,
        );

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

        let converted_liq_dist_points = liq_dist_points
            .iter()
            .map(|(x, y)| (*x as f32, *y as f32))
            .collect();
        let mut liq_dist_series = coords_to_line_series(converted_liq_dist_points);
        liq_dist_series.legend = "Liq. Dist.".to_string();
        liq_dist_series.color = plotters::style::full_palette::PURPLE_A400;

        let max_y_dist = liq_dist_points
            .iter()
            .max_by(|(_, y1), (_, y2)| y1.partial_cmp(y2).unwrap())
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
        pool_id: u64,
    ) -> Result<(CartesianRanges, Vec<ChartLineSeries>)> {
        let pool_state = self
            .pool_state
            .as_ref()
            .ok_or(Error::msg("Pool state not set"))?
            .get(&pool_id)
            .ok_or(Error::msg("Pool state not found"))?;

        let params = pool_state
            .log_normal_strategy
            .clone()
            .ok_or(Error::msg("Log normal strategy not set for pool state"))?;

        // Get the current strategy parameters, reserves, and liquidity.
        let (strike_price_wad, volatility_wad, time_remaining_wad) = (
            params.strike_price,
            params.volatility,
            params.time_remaining,
        );

        let total_liquidity_wad = pool_state
            .total_liquidity
            .clone()
            .ok_or(Error::msg("Total liquidity not set for pool state"))?
            .last()
            .unwrap()
            .1;

        // Convert these to float types.
        let strike_price = format_and_parse(strike_price_wad)?;
        let volatility = format_and_parse(volatility_wad)?;
        let time_remaining = format_and_parse(time_remaining_wad)?;
        let total_liquidity = format_and_parse(total_liquidity_wad)?;

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

    pub fn get_token_value_and_info(
        &self,
        token_address: AlloyAddress,
    ) -> Result<(Vec<(u64, AlloyU256)>, TokenInfo)> {
        let usd_balances = self.get_user_balances_usd()?.clone();

        let token_data = usd_balances
            .first()
            .ok_or(Error::msg(
                "Failed to get quote balance from user balances in USD",
            ))?
            .clone();

        let token_address = token_data.0;
        let token_value_series = usd_balances
            .first()
            .ok_or(Error::msg(
                "Failed to get quote balance from user balances in USD",
            ))?
            .1
            .clone();

        let token_info = self
            .token_metadata
            .as_ref()
            .unwrap()
            .get(&token_address)
            .ok_or(Error::msg(
                "Failed to get token info for quote token in user balances in USD",
            ))?;

        Ok((token_value_series, token_info.clone()))
    }

    pub fn derive_asset_value_series(
        &self,
        pool_id: u64,
    ) -> Result<(CartesianRanges, ChartLineSeries)> {
        let pool_state = self
            .pool_state
            .as_ref()
            .ok_or(Error::msg("Pool state not set"))?
            .get(&pool_id)
            .ok_or(Error::msg("Pool id missing from pool state data."))?;
        let (series, metadata) = self.get_token_value_and_info(pool_state.asset_token.ok_or(
            Error::msg("Asset token address missing from pool state data."),
        )?)?;

        let mut result = Self::transform_series_over_block_number(series)?;

        result.1.legend = format!("$ {}", metadata.symbol);
        result.1.color = plotters::style::full_palette::BLUE_A400;
        result.1.time_series = true;

        Ok(result)
    }

    // todo: fix the name to be more clear this is the usd balance of the user's
    // quote tokens.
    pub fn derive_quote_value_series(
        &self,
        pool_id: u64,
    ) -> Result<(CartesianRanges, ChartLineSeries)> {
        let pool_state = self
            .pool_state
            .as_ref()
            .ok_or(Error::msg("Pool state not set"))?
            .get(&pool_id)
            .ok_or(Error::msg("Pool id missing from pool state data."))?;

        let token_address = pool_state.quote_token.ok_or(Error::msg(
            "Quote token address missing from pool state data.",
        ))?;

        let (series, metadata) = self.get_token_value_and_info(token_address)?;

        // todo: fix with quote address?
        let mut result = Self::transform_series_over_block_number(series)?;

        result.1.legend = format!("$ {}", metadata.symbol);
        result.1.color = plotters::style::full_palette::PINK_A400;
        result.1.time_series = true;

        Ok(result)
    }

    /// Computes liquidity distribution and returns histogram formatted data.
    pub fn derive_liquidity_histogram(
        &self,
        current_price: f64,
        strike_price: f64,
        volatility: f64,
        time_remaining: f64,
    ) -> Result<HistogramData> {
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

/// Formats WAD values into stringified floating point decimals, then parses
/// them into f64.
/// todo: take an optional decimal argument for using format_units instead.
pub fn format_and_parse(value: AlloyU256) -> Result<f64> {
    let formatted = alloy_primitives::utils::format_ether(value);
    formatted.parse::<f64>().map_err(|e| e.into())
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
        // TODO: unused anvil instance
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
        let model = RawDataModel::<AlloyAddress, AlloyU256>::new(chain_id);

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
        // TODO: unused anvil instance
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
        let mut model = RawDataModel::<AlloyAddress, AlloyU256>::new(chain_id);

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

        model
            .update_token_balance_mapping(client.provider().clone().into())
            .await?;

        let saved_balance = model
            .user_token_balances
            .get(&converted_token_address)
            .unwrap()
            .last()
            .unwrap()
            .1;

        // Log the new balance.
        println!("User balance: {:?}", saved_balance);

        assert_eq!(saved_balance, balance);

        Ok(())
    }
}
