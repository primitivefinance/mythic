use super::{strategy::Strategy, token_admin::TokenAdmin, *};
use crate::strategy::LiquidityStrategy;

#[derive(Clone)]
pub struct RmmLiquidityProvider<S: LiquidityStrategy> {
    pub client: Arc<RevmMiddleware>,
    pub low_vol_strategy: S,
    pub high_vol_strategy: S,
    initial_x: U256,
    initial_price: U256,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct RmmLiquidityProviderParameters<P: Parameterized> {
    pub x_liquidity: P,
    pub initial_price: P,
}

impl From<RmmLiquidityProviderParameters<Multiple>>
    for Vec<RmmLiquidityProviderParameters<Single>>
{
    fn from(params: RmmLiquidityProviderParameters<Multiple>) -> Self {
        itertools::iproduct!(
            params.x_liquidity.parameters(),
            params.initial_price.parameters()
        )
        .map(|(xl, ip)| RmmLiquidityProviderParameters {
            x_liquidity: Single(xl),
            initial_price: Single(ip),
        })
        .collect()
    }
}

impl<S: LiquidityStrategy> RmmLiquidityProvider<S> {
    pub async fn new(
        environment: &Environment,
        config: &SimulationConfig<Single>,
        label: impl Into<String>,
        token_admin: &TokenAdmin,
        low_vol_strategy_contract: Address,
        high_vol_strategy_contract: Address,
    ) -> Result<Self> {
        let label = label.into();
        let client = RevmMiddleware::new(environment, Some(&label))?;
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
        arbx.approve(high_vol_strategy_contract, U256::MAX)
            .send()
            .await?;
        arby.approve(low_vol_strategy_contract, U256::MAX)
            .send()
            .await?;
        arby.approve(high_vol_strategy_contract, U256::MAX)
            .send()
            .await?;
        if let Some(AgentParameters::RmmLiquidityProvider(params)) =
            config.agent_parameters.get(&label).cloned()
        {
            Ok(Self {
                client,
                low_vol_strategy,
                high_vol_strategy,
                initial_x: ethers::utils::parse_ether(params.x_liquidity.0)?,
                initial_price: ethers::utils::parse_ether(params.initial_price.0)?,
            })
        } else {
            Err(anyhow::anyhow!(
                "No parameters found for `LiquidityProvider`"
            ))
        }
    }
}

#[async_trait::async_trait]
impl<S: LiquidityStrategy + std::marker::Sync + std::marker::Send> Agent
    for RmmLiquidityProvider<S>
{
    async fn startup(&mut self) -> Result<()> {
        info!("LiquidityProvider.startup: starting up");
        // Initializes the liquidity of a pool with a target price given an initial
        // amount of x tokens.
        let tx = self
            .low_vol_strategy
            .initialize_pool(self.initial_x, self.initial_price)
            .await?;

        let tx = self
            .high_vol_strategy
            .initialize_pool(self.initial_x, self.initial_price)
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
