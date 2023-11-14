use super::*;

#[async_trait::async_trait]
impl Strategy for RMM<RevmMiddleware> {
    type StrategyData = ();

    async fn decode_strategy_data(&self) -> Result<Self::StrategyData> {
        todo!()
    }

    fn new(strategy_address: Address, client: Arc<RevmMiddleware>) -> Self {
        Self::new(strategy_address, client)
    }

    async fn get_spot_price(&self) -> Result<U256> {
        todo!();
    }

    async fn get_swap_fee(&self) -> Result<U256> {
        todo!()
    }
    async fn get_strategy_logs(&self) {
        self.log_data().send().await.unwrap().await.unwrap();
    }
}
