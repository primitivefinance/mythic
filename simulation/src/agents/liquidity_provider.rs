use crate::strategy::LiquidityStrategy;

use super::{strategy::Strategy, token_admin::TokenAdmin, *};

#[derive(Clone)]
pub struct LiquidityProvider<S: LiquidityStrategy> {
    pub client: Arc<RevmMiddleware>,
    pub strategy: S,
    initial_x: U256,
    initial_price: U256,
}

impl<S: LiquidityStrategy> LiquidityProvider<S> {
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
            initial_price: float_to_wad(config.trajectory.initial_price),
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
impl<S: LiquidityStrategy + std::marker::Sync + std::marker::Send> Agent for LiquidityProvider<S> {
    async fn startup(&mut self) -> Result<()> {
        // Initializes the liquidity of a pool with a target price given an initial amount of x tokens.
        self.strategy
            .instantiate(self.initial_x, self.initial_price)
            .await?;

        info!(
            "LiquidityProvider.startup: instantiated pool at price {:?} wei",
            self.strategy.get_spot_price().await?
        );
        Ok(())
    }
}
