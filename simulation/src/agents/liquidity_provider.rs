use super::{strategy::Strategy, token_admin::TokenAdmin, *};

#[derive(Clone)]
pub struct LiquidityProvider<S: Strategy> {
    pub client: Arc<RevmMiddleware>,
    pub strategy: S,
    pub initial_balances: (U256, U256),
}

impl<S: Strategy> LiquidityProvider<S> {
    pub async fn new(
        environment: &Environment,
        token_admin: &TokenAdmin,
        strategy_address: Address,
        config: &SimulationConfig,
    ) -> Result<Self> {
        let client = RevmMiddleware::new(environment, "liquidity_provider".into())?;
        let strategy: S = Strategy::new(strategy_address, client.clone());

        let arbx = ArbiterToken::new(token_admin.arbx.address(), client.clone());
        let arby = ArbiterToken::new(token_admin.arby.address(), client.clone());

        let initial_balances = strategy.get_lp_amounts(config).await;

        token_admin
            .mint(
                client.address(),
                parse_ether(initial_balances.0).unwrap(),
                parse_ether(initial_balances.1).unwrap(),
            )
            .await?;

        arbx.approve(strategy_address, U256::MAX).send().await?;
        arby.approve(strategy_address, U256::MAX).send().await?;

        Ok(Self {
            client,
            strategy,
            initial_balances,
        })
    }

    // TODO: This can be consolidated if we have a generalized way to deposit
    pub async fn add_liquidity(self) -> Result<()> {
        // Call init pool to setup the portfolio
        // Needs an amount of both tokens, the amounts can be anything but note that
        // they affect the spot price.
        self.strategy
            .init_pool(self.initial_balances.0, self.initial_balances.1)
            .await?;
        Ok(())
    }
}
