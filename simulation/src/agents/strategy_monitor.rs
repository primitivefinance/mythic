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
    pub portfolio_value_start: U256,
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
            portfolio_value_start: U256::zero(),
        })
    }
}

#[async_trait::async_trait]
impl<S: LiquidityStrategy + 'static + Debug> Agent for StrategyMonitorAgent<S> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn client(&self) -> Arc<RevmMiddleware> {
        self.client.clone()
    }

    fn get_name(&self) -> String {
        "Strategy Monitor".to_string()
    }

    async fn startup(&mut self) -> Result<()> {
        self.portfolio_value_start = self.strategy.get_portfolio_value().await?;
        Ok(())
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

        let start_value: I256 = I256::from_raw(self.portfolio_value_start);
        let end_value: I256 = I256::from_raw(portfolio_value);
        let net_profit = end_value.checked_sub(start_value).unwrap();

        debug!("Start value: {}", start_value);
        debug!("End value: {}", end_value);
        debug!("Net profit: {}", net_profit);

        let subbed = vec![
            SubscribedData {
                name: "X Price".to_string(),
                data: spot_price.into_token(),
            },
            SubscribedData {
                name: "Your X Balance".to_string(),
                data: reserve_x.into_token(),
            },
            SubscribedData {
                name: "Your Y Balance".to_string(),
                data: reserve_y.into_token(),
            },
            SubscribedData {
                name: "Your Portfolio Value".to_string(),
                data: portfolio_value.into_token(),
            },
            SubscribedData {
                name: "Your Net Profit".to_string(),
                data: net_profit.into_token(),
            },
            SubscribedData {
                name: "Invariant".to_string(),
                data: invariant.into_token(),
            },
        ];

        Ok(subbed)
    }
}
