use self::{
    momentum_strategist::MomentumStrategist,
    volatility_targeting_strategist::VolatilityTargetingStrategist,
};

use super::*;

pub mod momentum_strategist;
pub mod volatility_targeting_strategist;

#[async_trait::async_trait]
pub trait WeightChanger: Agent {
    async fn execute_smooth_rebalance(&mut self) -> Result<()>;
    fn g3m(&self) -> &G3M<RevmMiddleware>;
    fn lex(&self) -> &LiquidExchange<RevmMiddleware>;
}

pub enum WeightChangerType {
    Momentum(MomentumStrategist),
    VolatilityTargeting(VolatilityTargetingStrategist),
}

impl WeightChangerType {
    pub async fn new(
        environment: &Environment,
        config: &SimulationConfig<Fixed>,
        liquid_exchange_address: Address,
        arbx_address: Address,
        arby_address: Address,
    ) -> Result<Self> {
        if let Some(settings) = &config.weight_changer.momentum {
            let strategist = MomentumStrategist::new(
                environment,
                config,
                liquid_exchange_address,
                arbx_address,
                arby_address,
            )
            .await?;
            Ok(WeightChangerType::Momentum(strategist))
        } else {
            let strategist = VolatilityTargetingStrategist::new(
                environment,
                config,
                liquid_exchange_address,
                arbx_address,
                arby_address,
            )
            .await?;
            Ok(WeightChangerType::VolatilityTargeting(strategist))
        }
    }
}

#[async_trait::async_trait]
impl Agent for WeightChangerType {
    async fn step(&mut self) -> Result<()> {
        match self {
            Self::Momentum(strategist) => strategist.step().await,
            Self::VolatilityTargeting(strategist) => strategist.step().await,
        };
        Ok(())
    }

    async fn startup(&mut self) -> Result<()> {
        match self {
            Self::Momentum(strategist) => strategist.startup().await,
            Self::VolatilityTargeting(strategist) => strategist.startup().await,
        };
        Ok(())
    }
}

#[async_trait::async_trait]
impl WeightChanger for WeightChangerType {
    async fn execute_smooth_rebalance(&mut self) -> Result<()> {
        match self {
            Self::Momentum(strategist) => strategist.execute_smooth_rebalance().await,
            Self::VolatilityTargeting(strategist) => strategist.execute_smooth_rebalance().await,
        }
    }

    fn g3m(&self) -> &G3M<RevmMiddleware> {
        match self {
            Self::Momentum(strategist) => strategist.g3m(),
            Self::VolatilityTargeting(strategist) => strategist.g3m(),
        }
    }

    fn lex(&self) -> &LiquidExchange<RevmMiddleware> {
        match self {
            Self::Momentum(strategist) => strategist.lex(),
            Self::VolatilityTargeting(strategist) => strategist.lex(),
        }
    }
}
