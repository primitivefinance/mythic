use super::{errors::SimulationError, *};
use crate::{
    agents::{
        arbitrageur::Arbitrageur, liquidity_provider::LiquidityProvider,
        price_changer::PriceChanger, token_admin::TokenAdmin,
        volatility_targeting_strategist::VolatilityTargetingStrategist,
    },
    settings::SimulationConfig,
};

pub async fn setup(config: SimulationConfig<Fixed>) -> Result<Simulation, SimulationError> {
    todo!();
}
