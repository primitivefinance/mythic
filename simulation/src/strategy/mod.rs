use super::*;

pub mod g3m;
pub mod rmm;

#[async_trait::async_trait]
pub trait Strategy: Sized {
    fn new(strategy_address: Address, client: Arc<RevmMiddleware>) -> Self;

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

    async fn get_spot_price(&self) -> Result<U256>;

    async fn swap_fee(&self) -> Result<U256>;

    async fn init_pool_with_x(&self, amount_x: U256) -> Result<Option<TransactionReceipt>>;
}

#[async_trait::async_trait]
pub trait LiquidityStrategy: Strategy {
    /// Instantiate's the pool with a target amount of reserves and liquidity, at a price.
    async fn instantiate(&self, initial_x_wad: U256, initial_price_wad: U256) -> Result<()>;
}

#[async_trait::async_trait]
pub trait RebalanceStrategy: Strategy {
    async fn rebalance(&self) -> Result<Option<TransactionReceipt>>;
}
