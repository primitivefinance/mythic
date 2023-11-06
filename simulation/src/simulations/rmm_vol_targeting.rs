use arbiter_core::environment::builder::BlockSettings;

use super::{errors::SimulationError, *};
use crate::{
    agents::rmm::{
        arbitrageur::Arbitrageur, liquidity_provider::LiquidityProvider,
        rmm_strategist::VolatilityTargetingStrategist,
    },
    agents::{
        block_admin::BlockAdmin, momentum_strategist::MomentumStrategist,
        price_changer::PriceChanger, rmm::rmm_strategist, token_admin::TokenAdmin, Agent, Agents,
    },
    bindings::i_strategy::IStrategy,
    settings::SimulationConfig,
    strategy::rmm::RmmStrategy,
};

pub async fn setup(config: SimulationConfig<Fixed>) -> Result<Simulation, SimulationError> {
    let environment = EnvironmentBuilder::new()
        .block_settings(BlockSettings::UserControlled)
        .build();
    let mut block_admin = BlockAdmin::new(&environment, &config).await?;

    let token_admin = TokenAdmin::new(&environment).await?;
    let mut price_changer = PriceChanger::new(&environment, &token_admin, &config).await?;
    let vol_strategist = VolatilityTargetingStrategist::new(
        &environment,
        &config,
        price_changer.liquid_exchange.address(),
        token_admin.arbx.address(),
        token_admin.arby.address(),
    )
    .await?;

    let mut lp = LiquidityProvider::<RmmStrategy>::new(
        &environment,
        &token_admin,
        vol_strategist.low_vol_pool.address(),
        vol_strategist.high_vol_pool.address(),
        &config,
    )
    .await?;
    let mut arbitrageur = Arbitrageur::<RmmStrategy>::new(
        &environment,
        &token_admin,
        price_changer.liquid_exchange.address(),
        vol_strategist.low_vol_pool.address(),
        vol_strategist.high_vol_pool.address(),
    )
    .await?;

    println!("low vol pool: {}", vol_strategist.low_vol_pool.address());
    println!("low vol pool: {}", vol_strategist.high_vol_pool.address());

    EventLogger::builder()
        .add(price_changer.liquid_exchange.events(), "lex")
        .add(vol_strategist.low_vol_pool.events(), "low_vol_pool")
        .add(vol_strategist.high_vol_pool.events(), "low_vol_pool")
        .run()
        .map_err(|e| SimulationError::GenericError(e.to_string()))?;

    Ok(Simulation {
        agents: Agents::new()
            .add(price_changer)
            .add(arbitrageur)
            .add(block_admin)
            .add(vol_strategist)
            .add(lp),
        steps: config.trajectory.num_steps,
        environment,
    })
}
