use super::*;

pub mod g3m;
pub mod rmm;

/// Strategies are a layer between an agent and the smart contracts responsible for managing a strategy's parameters and assets.
/// Sub-traits extend this core Strategy trait to provide additional functionality that is specific to different agents.
#[async_trait::async_trait]
pub trait Strategy: Sized {
    /// Strategy stored is fetched from the strategy smart contract as bytes.
    /// This type defines how those bytes are decoded into a strategy data type.
    type StrategyData;

    /// Creates a new strategy instance.
    fn new(strategy_address: Address, client: Arc<RevmMiddleware>) -> Self;

    /// Returns the spot price of the strategy's pool.
    async fn get_spot_price(&self) -> Result<U256>;

    /// Returns the fee charged to the arbitrageur on rebalances.
    async fn get_swap_fee(&self) -> Result<U256>;

    /// Responsible for decoding the strategy data into the strategy data type defined above.
    async fn decode_strategy_data(&self) -> Result<Self::StrategyData>;
}

/// A sub-trait for liquidity providers to implement their specific logic for providing/setting up a pool.
#[async_trait::async_trait]
pub trait LiquidityStrategy: Strategy {
    /// Provides the pool with an initial amount of reserves and liquidity, at a price.
    async fn instantiate(
        &self,
        initial_x_wad: U256,
        initial_price_wad: U256,
    ) -> Result<Option<TransactionReceipt>>;
}

/// A sub-trait for arbitrageurs to implement their logic for computing how many tokens to swap.
/// For example, a generalized arbitrageur can use a root-finding method, while a specialized arbitrageur (arb G3M pools) can use algebra.
#[async_trait::async_trait]
pub trait ArbitrageStrategy: Strategy {
    // For arbitrage
    async fn get_x_input(
        &self,
        target_price_wad: U256,
        math: &SD59x18Math<RevmMiddleware>,
    ) -> Result<U256>;
    async fn get_y_input(
        &self,
        target_price_wad: U256,
        math: &SD59x18Math<RevmMiddleware>,
    ) -> Result<U256>;
}
