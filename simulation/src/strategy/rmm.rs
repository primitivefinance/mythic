use super::*;

#[async_trait::async_trait]
impl Strategy for RMM<RevmMiddleware> {
    fn new(strategy_address: Address, client: Arc<RevmMiddleware>) -> Self {
        Self::new(strategy_address, client)
    }
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
        todo!();
    }

    async fn swap_fee(&self) -> Result<U256> {
        todo!()
    }
}
