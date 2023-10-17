use super::*;

pub mod g3m;
pub mod rmm;

#[async_trait::async_trait]
pub trait Strategy {
    fn new(strategy_address: Address, client: Arc<RevmMiddleware>) -> Self;
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
    // TODO: This can be removed if we have the right deposit flow on the contract
    async fn get_lp_amounts(&self, config: &SimulationConfig) -> (U256, U256);
    async fn init_pool(&self, amount_x: U256, amount_y: U256)
        -> Result<Option<TransactionReceipt>>;
}
