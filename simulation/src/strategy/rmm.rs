use super::*;

#[async_trait::async_trait]
impl Strategy for RMM<RevmMiddleware> {
    async fn get_x_input(
        &self,
        target_price_wad: U256,
        math: &SD59x18Math<RevmMiddleware>,
    ) -> Result<U256> {
        todo!()
    }

    async fn get_y_input(
        &self,
        target_price_wad: U256,
        math: &SD59x18Math<RevmMiddleware>,
    ) -> Result<U256> {
        todo!()
    }

    async fn get_spot_price(&self) -> Result<U256> {
        Ok(self.get_spot_price().call().await?)
    }

    async fn swap_fee(&self) -> Result<U256> {
        todo!()
    }

    // TODO: This can be removed if we have the right deposit flow on the contract
    async fn get_lp_amounts(&self, config: &SimulationConfig) -> (U256, U256) {
        todo!()
    }

    async fn init_pool(
        &self,
        amount_x: U256,
        amount_y: U256,
    ) -> Result<Option<TransactionReceipt>> {
        todo!()
    }
}
