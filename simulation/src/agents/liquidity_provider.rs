use super::{strategy::Strategy, token_admin::TokenAdmin, *};

#[derive(Clone)]
pub struct LiquidityProvider<S: Strategy> {
    pub client: Arc<RevmMiddleware>,
    pub strategy: S,
    initial_x: U256,
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

        token_admin
            .mint(client.address(), U256::MAX / 2, U256::MAX / 2)
            .await?;

        arbx.approve(strategy_address, U256::MAX).send().await?;
        arby.approve(strategy_address, U256::MAX).send().await?;

        Ok(Self {
            client,
            strategy,
            initial_x: float_to_wad(config.lp.x_liquidity),
        })
    }

    // TODO: This can be consolidated if we have a generalized way to deposit
    async fn add_liquidity(&self) -> Result<()> {
        println!("here");
        self.strategy.init_pool_with_x(self.initial_x).await?;
        println!("after the call");
        Ok(())
    }
}

#[async_trait::async_trait]
impl<S: Strategy + std::marker::Sync + std::marker::Send> Agent for LiquidityProvider<S> {
    async fn startup(&mut self) -> Result<()> {
        self.add_liquidity().await?;
        Ok(())
    }
}
