use std::collections::BTreeMap;

use anyhow::{anyhow, Error, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::*;
use mythic_charts::{coords_to_line_series, CartesianRanges, ChartLineSeries, ChartPoint};

pub const WAD: U256 = U256([1_000_000_000_000_000_000, 0, 0, 0]);

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

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PoolState {
    pub id: Option<U256>,
    pub controller: Option<Address>,
    pub strategy: Option<Address>,
    pub asset_token: Option<Address>,
    pub quote_token: Option<Address>,
    pub liquidity_token: Option<Address>,

    pub internal_price: Option<Vec<(u64, U256)>>,
    pub total_liquidity: Option<Vec<(u64, U256)>>,
    pub asset_reserve: Option<Vec<(u64, U256)>>,
    pub quote_reserve: Option<Vec<(u64, U256)>>,
    pub liquidity_token_total_supply: Option<Vec<(u64, U256)>>,
    pub log_normal_strategy: Option<LogNormalStrategyState<U256>>,
    pub swap_fee_wad: Option<U256>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LogNormalStrategyState<V> {
    pub strike_price: V,
    pub volatility: V,
    pub time_remaining: V,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataModel {
    pub chain_id: Option<u64>,
    pub user_address: Option<Address>,
    pub external_exchange_address: Option<Address>,
    pub dfmm_address: Option<Address>,
    pub log_normal_solver_address: Option<Address>,
    pub g3m_solver_address: Option<Address>,
    pub last_sync: Option<DateTime<Utc>>,
    pub last_sync_block: Option<u64>,
    pub user_history: Option<Vec<HistoricalTx>>,
    pub last_user_history_sync_block: Option<u64>,
    #[serde(default)]
    pub user_token_balances: BTreeMap<Address, Vec<(u64, U256)>>,
    pub pool_state: Option<BTreeMap<u64, PoolState>>,
    pub external_prices: Option<BTreeMap<Address, Vec<(u64, U256)>>>,
    pub liquidity_token_addresses: Option<Vec<Address>>,
    pub token_metadata: Option<BTreeMap<Address, TokenInfo>>,
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
    pub token_x_address: Address,
    pub token_y_address: Address,
    pub token_l_address: Address,
    pub claimable_balance_x: f64,
    pub claimable_balance_y: f64,
    pub liquidity_balance: f64,
    pub liquidity_value: f64,
    pub external_price: f64,
    pub internal_price: f64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UnallocatedPosition {
    pub token_address: Address,
    pub balance: f64,
    pub external_price: f64,
}

impl StrategyPosition {
    pub fn compute_value(&self) -> Result<f64> {
        Ok(self.balance_x * self.external_price + self.balance_y * self.quote_price)
    }
}

impl DataModel {
    pub fn new(chain_id: u64) -> Self {
        Self {
            chain_id: Some(chain_id),
            ..Default::default()
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn setup(
        &mut self,
        user_address: Address,
        external_exchange_address: Address,
        dfmm_address: Address,
        log_normal_solver_address: Address,
        g3m_solver_address: Address,
    ) {
        self.user_address = Some(user_address);
        self.external_exchange_address = Some(external_exchange_address);
        self.dfmm_address = Some(dfmm_address);
        self.log_normal_solver_address = Some(log_normal_solver_address);
        self.g3m_solver_address = Some(g3m_solver_address);
    }

    pub fn add_token(&mut self, token_address: Address) -> Result<()> {
        self.user_token_balances.entry(token_address).or_default();
        Ok(())
    }

    pub fn add_pool(&mut self, pool_id: u64) -> Result<()> {
        self.pool_state
            .get_or_insert_with(BTreeMap::new)
            .entry(pool_id)
            .or_default();
        Ok(())
    }

    pub async fn update<M: Middleware + 'static>(&mut self, client: Arc<M>) -> Result<()>
    where
        <M as ethers::providers::Middleware>::Error: 'static,
    {
        self.update_last_sync_timestamp()?;
        self.update_last_sync_block(client.clone()).await?;
        self.last_user_history_sync_block = Some(0);

        if let Err(err) = self.sync_liquidity_tokens_to_balance_mapping() {
            tracing::warn!("Liquidity token sync failed: {:?}", err);
        }

        if let Err(err) = self.update_token_info_mapping(client.clone()).await {
            tracing::warn!("Token info mapping update failed: {:?}", err);
        }

        Ok(())
    }

    pub fn sync_liquidity_tokens_to_balance_mapping(&mut self) -> Result<()> {
        let liquidity_token_addresses = self
            .liquidity_token_addresses
            .as_ref()
            .ok_or(Error::msg("Liquidity token addresses not set"))?;

        for liquidity_token_address in liquidity_token_addresses {
            self.user_token_balances
                .entry(*liquidity_token_address)
                .or_default();
        }

        Ok(())
    }

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
                let token_info = fetch_token_info(client.clone(), token_address).await?;
                self.token_metadata
                    .as_mut()
                    .unwrap()
                    .insert(token_address, token_info);
            }
        }
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

    pub async fn fetch_block_number<M: Middleware + 'static>(&self, client: Arc<M>) -> Result<u64>
    where
        <M as ethers::providers::Middleware>::Error: 'static,
    {
        let block_number = client.get_block_number().await?;
        Ok(block_number.as_u64())
    }
}

async fn fetch_token_info<M: Middleware + 'static>(
    _client: Arc<M>,
    _token_address: Address,
) -> Result<TokenInfo> {
    let name = "temp".to_string();
    let symbol = "temp".to_string();
    let decimals = 18u8;

    Ok(TokenInfo {
        name,
        symbol,
        decimals,
    })
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

pub fn format_and_parse(value: U256) -> Result<f64> {
    let formatted = ethers::utils::format_ether(value);
    formatted.parse::<f64>().map_err(|e| e.into())
}

#[cfg(test)]
mod tests {
    use ethers::{
        prelude::*,
        utils::{Anvil, AnvilInstance},
    };

    use super::*;

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
        let wallet: LocalWallet = anvil.keys().first().unwrap().clone().into();
        let wallet = wallet.with_chain_id(anvil.chain_id());
        println!("Connected to URL: {}", url);

        println!("Wallet address: {}", wallet.address());
        let provider = Provider::<Ws>::connect(&url).await?;
        let client = Arc::new(SignerMiddleware::new(provider, wallet));

        Ok((anvil, client))
    }
}
