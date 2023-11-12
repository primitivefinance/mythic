use ethers::abi::Tokenizable;

use super::{strategy::Strategy, token_admin::TokenAdmin, *};
use crate::{bindings::ig3m::IG3M, strategy::LiquidityStrategy};

#[derive(Clone, Debug)]
pub struct LiquidityProvider<S: LiquidityStrategy>
where
    Self: Sized,
{
    pub client: Arc<RevmMiddleware>,
    pub strategy: S,
    initial_x: U256,
    initial_price: U256,
    pub token_admin: TokenAdmin,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct LiquidityProviderParameters<P: Parameterized> {
    pub x_liquidity: P,
    pub initial_price: P,
}

impl From<LiquidityProviderParameters<Multiple>> for Vec<LiquidityProviderParameters<Single>> {
    fn from(params: LiquidityProviderParameters<Multiple>) -> Self {
        itertools::iproduct!(
            params.x_liquidity.parameters(),
            params.initial_price.parameters()
        )
        .map(|(xl, ip)| LiquidityProviderParameters {
            x_liquidity: Single(xl),
            initial_price: Single(ip),
        })
        .collect()
    }
}

impl<S: LiquidityStrategy + Sized> LiquidityProvider<S> {
    pub async fn new(
        environment: &Environment,
        config: &SimulationConfig<Single>,
        label: impl Into<String>,
        token_admin: &TokenAdmin,
        strategy_address: Address,
    ) -> Result<Self> {
        let label = label.into();
        let client = RevmMiddleware::new(environment, Some(&label))?;
        let strategy: S = S::new(strategy_address, client.clone());

        let arbx = ArbiterToken::new(token_admin.arbx.address(), client.clone());
        let arby = ArbiterToken::new(token_admin.arby.address(), client.clone());

        token_admin
            .mint(
                client.address(),
                parse_ether(100_000_u64).unwrap(),
                parse_ether(100_000_u64).unwrap(),
            )
            .await?;

        arbx.approve(strategy_address, U256::MAX).send().await?;
        arby.approve(strategy_address, U256::MAX).send().await?;

        if let Some(AgentParameters::LiquidityProvider(params)) =
            config.agent_parameters.get(&label).cloned()
        {
            Ok(Self {
                client,
                strategy,
                initial_x: ethers::utils::parse_ether(params.x_liquidity.0)?,
                initial_price: ethers::utils::parse_ether(params.initial_price.0)?,
                token_admin: token_admin.clone(),
            })
        } else {
            Err(anyhow::anyhow!(
                "No parameters found for `LiquidityProvider`"
            ))
        }
    }

    pub async fn get(&self) -> Result<U256> {
        let res = self.strategy.get_portfolio_value().await;
        match res {
            Ok(res) => Ok(res),
            Err(err) => {
                tracing::warn!("Error getting pvf: {}", err);
                Err(anyhow::anyhow!("Error getting pvf: {}", err))
            }
        }
    }
}

pub trait LiquidityProviderWrapper: Sized {
    fn as_liquidity_provider(&self) -> &dyn Any;
}

impl<S: LiquidityStrategy + 'static + Debug> LiquidityProviderWrapper for LiquidityProvider<S> {
    fn as_liquidity_provider(&self) -> &dyn Any {
        self
    }
}

#[async_trait::async_trait]
impl<S: LiquidityStrategy + 'static + Debug> Agent for LiquidityProvider<S> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_client(&self) -> Result<Arc<RevmMiddleware>> {
        Ok(self.client.clone())
    }

    fn get_name(&self) -> String {
        format!("liquidity_provider")
    }

    async fn get_subscribed(&self) -> Result<Vec<SubscribedData>> {
        // get spot price, reserves, invariant, portfolio value.

        let spot_price = self.strategy.get_spot_price().await?;
        let reserve_x = self.strategy.get_reserve_x().await?;
        let reserve_y = self.strategy.get_reserve_y().await?;
        let invariant = self.strategy.get_invariant().await?;
        let portfolio_value = self.strategy.get_portfolio_value().await?;
        let this = self.client.address();
        let x_balance = self.token_admin.arbx.balance_of(this).call().await?;
        let y_balance = self.token_admin.arby.balance_of(this).call().await?;

        let subbed = vec![
            SubscribedData {
                name: "spot_price".to_string(),
                data: spot_price.into_token(),
            },
            SubscribedData {
                name: "x_balance".to_string(),
                data: x_balance.into_token(),
            },
            SubscribedData {
                name: "y_balance".to_string(),
                data: y_balance.into_token(),
            },
            SubscribedData {
                name: "reserve_x".to_string(),
                data: reserve_x.into_token(),
            },
            SubscribedData {
                name: "reserve_y".to_string(),
                data: reserve_y.into_token(),
            },
            SubscribedData {
                name: "invariant".to_string(),
                data: invariant.into_token(),
            },
            SubscribedData {
                name: "portfolio_value".to_string(),
                data: portfolio_value.into_token(),
            },
        ];

        Ok(subbed)
    }

    async fn startup(&mut self) -> Result<()> {
        debug!("Entering `LiquidityProvider` startup");
        debug!(
            "Getting pvf state {}",
            self.strategy.get_portfolio_value().await?.to_string()
        );
        // Initializes the liquidity of a pool with a target price given an initial
        // amount of x tokens.

        trace!(
            "Initializing pool with {} x tokens and target price of {} wei",
            self.initial_x,
            self.initial_price
        );
        self.strategy
            .initialize_pool(self.initial_x, self.initial_price)
            .await?;

        debug!(
            "Exited `LiquidityProvider` startup, instantiated pool at price {:?} wei",
            self.strategy.get_spot_price().await?
        );
        Ok(())
    }
    fn client(&self) -> Arc<RevmMiddleware> {
        self.client.clone()
    }
    async fn step(&mut self) -> Result<()> {
        self.strategy.get_strategy_logs().await;
        Ok(())

    async fn get_state(&self) -> Result<U256> {
        let res = self.strategy.get_portfolio_value().await;
        match res {
            Ok(res) => Ok(res),
            Err(err) => {
                tracing::warn!("Error getting pvf: {}", err);
                Err(anyhow::anyhow!("Error getting pvf: {}", err))
            }
        }
    }
}
