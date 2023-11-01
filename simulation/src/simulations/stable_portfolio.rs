use super::{errors::SimulationError, *};
use crate::{
    agents::{
        arbitrageur::Arbitrageur, liquidity_provider::LiquidityProvider,
        price_changer::PriceChanger, token_admin::TokenAdmin,
    },
    settings::SimulationConfig,
};

pub async fn setup(config: SimulationConfig<Fixed>) -> Result<Simulation, SimulationError> {
    todo!();
}
