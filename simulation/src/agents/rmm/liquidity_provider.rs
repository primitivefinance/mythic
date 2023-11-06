use super::{strategy::Strategy, token_admin::TokenAdmin, *};
use crate::strategy::LiquidityStrategy;

#[derive(Clone)]
pub struct LiquidityProvider<S: LiquidityStrategy> {
    pub client: Arc<RevmMiddleware>,
    pub low_vol_strategy: S,
    pub high_vol_strategy: S,
    initial_x: U256,
    initial_price: U256,
}

impl<S: LiquidityStrategy> LiquidityProvider<S> {
    pub async fn new(
        environment: &Environment,
        token_admin: &TokenAdmin,
        low_vol_strategy_contract: Address,
        high_vol_strategy_contract: Address,
        config: &SimulationConfig<Fixed>,
    ) -> Result<Self> {
        let client = RevmMiddleware::new(environment, "liquidity_provider".into())?;
        let low_vol_strategy: S = S::new(low_vol_strategy_contract, client.clone());
        let high_vol_strategy: S = S::new(high_vol_strategy_contract, client.clone());

        let arbx = ArbiterToken::new(token_admin.arbx.address(), client.clone());
        let arby = ArbiterToken::new(token_admin.arby.address(), client.clone());

        token_admin
            .mint(client.address(), U256::MAX / 2, U256::MAX / 2)
            .await?;

        arbx.approve(low_vol_strategy_contract, U256::MAX)
            .send()
            .await?;
        arby.approve(high_vol_strategy_contract, U256::MAX)
            .send()
            .await?;

        Ok(Self {
            client,
            low_vol_strategy,
            high_vol_strategy,
            initial_x: float_to_wad(config.lp.x_liquidity),
            initial_price: float_to_wad(config.trajectory.initial_price.0),
        })
    }
}

#[async_trait::async_trait]
impl<S: LiquidityStrategy + std::marker::Sync + std::marker::Send> Agent for LiquidityProvider<S> {
    async fn startup(&mut self) -> Result<()> {
        info!("LiquidityProvider.startup: starting up");
        // Initializes the liquidity of a pool with a target price given an initial
        // amount of x tokens.
        let tx = self
            .low_vol_strategy
            .instantiate(self.initial_x, self.initial_price)
            .await?;

        let tx = self
            .high_vol_strategy
            .instantiate(self.initial_x, self.initial_price)
            .await?;

        info!(
            "LiquidityProvider.startup: instantiated pool at price {:?} wei",
            self.high_vol_strategy.get_spot_price().await?
        );
        info!(
            "LiquidityProvider.startup: instantiated pool at price {:?} wei",
            self.low_vol_strategy.get_spot_price().await?
        );
        Ok(())
    }
}
