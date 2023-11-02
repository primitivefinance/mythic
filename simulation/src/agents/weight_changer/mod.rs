use self::{
    dollar_cost_averaging::DollarCostAveragingParameters,
    momentum::{MomentumParameters, MomentumStrategist},
    volatility_targeting::{VolatilityTargetingParameters, VolatilityTargetingStrategist},
};

use super::*;

pub mod dollar_cost_averaging;
pub mod momentum;
pub mod volatility_targeting;

#[async_trait::async_trait]
pub trait WeightChanger: Agent {
    async fn execute_smooth_rebalance(&mut self) -> Result<()>;
    fn g3m(&self) -> &G3M<RevmMiddleware>;
    fn lex(&self) -> &LiquidExchange<RevmMiddleware>;
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum WeightChangerParameters<P: Parameterized> {
    Momentum(MomentumParameters<P>),
    VolatilityTargeting(VolatilityTargetingParameters<P>),
    DollarCostAveraging(DollarCostAveragingParameters<P>),
}

pub struct WeightChangerType(pub Box<dyn WeightChanger>);

impl WeightChangerType {
    pub async fn new(
        environment: &Environment,
        config: &SimulationConfig<Single>,
        liquid_exchange_address: Address,
        arbx_address: Address,
        arby_address: Address,
    ) -> Result<Self> {
        if let Some(settings) = &config.weight_changer.momentum {
            let momentum =
                MomentumStrategist::new(environment, config, liquid_exchange_address).await?;
            Ok(WeightChangerType(Box::new(momentum)))
        } else {
            let volatility =
                MomentumStrategist::new(environment, config, liquid_exchange_address).await?;
            Ok(WeightChangerType(Box::new(volatility)))
        }
    }
}

#[async_trait::async_trait]
impl Agent for WeightChangerType {
    async fn step(&mut self) -> Result<()> {
        self.0.step().await
    }

    async fn startup(&mut self) -> Result<()> {
        self.0.startup().await
    }
}

#[async_trait::async_trait]
impl WeightChanger for WeightChangerType {
    async fn execute_smooth_rebalance(&mut self) -> Result<()> {
        self.0.execute_smooth_rebalance().await
    }

    fn g3m(&self) -> &G3M<RevmMiddleware> {
        self.0.g3m()
    }

    fn lex(&self) -> &LiquidExchange<RevmMiddleware> {
        self.0.lex()
    }
}
