//! The most basic agent... it just increments a counter.

use bindings::{counter::Counter, g3m::G3M};
use ethers::abi::Tokenizable;

use super::{token_admin::TokenAdmin, *};
use crate::strategy::LiquidityStrategy;

#[derive(Clone, Debug)]
pub struct StrategyMonitorAgent<S: LiquidityStrategy>
where
    Self: Sized,
{
    pub client: Arc<RevmMiddleware>,
    pub strategy: S,
    pub strategy_address: Address,
    pub token_admin: TokenAdmin,
}

impl<S: LiquidityStrategy + Sized> StrategyMonitorAgent<S> {
    pub async fn new(
        environment: &Environment,
        config: &SimulationConfig<Single>,
        label: impl Into<String>,
        strategy_address: Address,
        token_admin: &TokenAdmin,
    ) -> Result<Self> {
        let label: String = label.into();
        let client = RevmMiddleware::new(environment, Some(&label))?;
        let strategy: S = S::new(strategy_address, client.clone());

        tracing::trace!("Made a new counter agent with label {}", label);
        Ok(Self {
            client,
            strategy,
            strategy_address,
            token_admin: token_admin.clone(),
        })
    }
}

#[async_trait::async_trait]
impl<S: LiquidityStrategy + 'static + Debug> Agent for StrategyMonitorAgent<S> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_name(&self) -> String {
        "strategy_monitor".to_string()
    }

    async fn get_subscribed(&self) -> Result<Vec<SubscribedData>> {
        let spot_price = self.strategy.get_spot_price().await?;
        let reserve_x = self.strategy.get_reserve_x().await?;
        let reserve_y = self.strategy.get_reserve_y().await?;
        let invariant = self.strategy.get_invariant().await?;
        let portfolio_value = self.strategy.get_portfolio_value().await?;
        let strategy_address = self.strategy_address;
        let x_balance = self
            .token_admin
            .arbx
            .balance_of(strategy_address)
            .call()
            .await?;
        let y_balance = self
            .token_admin
            .arby
            .balance_of(strategy_address)
            .call()
            .await?;

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
}
