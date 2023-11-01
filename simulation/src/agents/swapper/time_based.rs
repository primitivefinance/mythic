use super::*;

pub struct TimeBasedSwapper {
    pub client: Arc<RevmMiddleware>,
    pub liquid_exchange: LiquidExchange<RevmMiddleware>,
    pub arbx_address: Address,
    pub arby_address: Address,
    pub arbx_balance: U256,
    pub arby_balance: U256,
}

impl TimeBasedSwapper {
    fn new(
        environment: Environment,
        liquid_exchange_address: Address,
        arbx_address: Address,
        arby_address: Address,
    ) -> Self {
        let client = RevmMiddleware::new(&environment, "time_based_swapper".into()).unwrap();
        let liquid_exchange = LiquidExchange::new(liquid_exchange_address, client.clone());
        Self {
            client,
            liquid_exchange,
            arbx_address,
            arby_address,
        }
    }
}

impl Swapper for TimeBasedSwapper {
    fn swap(&self, amount: f64) -> f64 {
        amount
    }
}
