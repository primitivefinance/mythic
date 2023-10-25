use arbiter_core::environment::builder::BlockSettings;

use super::{errors::SimulationError, *};
use crate::{
    agents::{
        arbitrageur::Arbitrageur, block_admin::BlockAdmin, liquidity_provider::LiquidityProvider,
        price_changer::PriceChanger, token_admin::TokenAdmin, momentum_strategist::MomentumStrategist, Agent,
        Agents,
    },
    bindings::i_strategy::IStrategy,
    settings::SimulationConfig,
};

pub async fn setup(config: SimulationConfig<Fixed>) -> Result<Simulation, SimulationError> {
    let environment = EnvironmentBuilder::new()
        .block_settings(BlockSettings::UserControlled)
        .build();
    let mut block_admin = BlockAdmin::new(&environment, &config).await?;

    let token_admin = TokenAdmin::new(&environment).await?;
    let mut price_changer = PriceChanger::new(&environment, &token_admin, &config).await?;
    let mut weight_changer = MomentumStrategist::new(
        &environment,
        &config,
        price_changer.liquid_exchange.address(),
        token_admin.arbx.address(),
        token_admin.arby.address(),
    )
    .await?;

    let mut lp = LiquidityProvider::<IStrategy<RevmMiddleware>>::new(
        &environment,
        &token_admin,
        weight_changer.g3m.address(),
        &config,
    )
    .await?;
    let mut arbitrageur = Arbitrageur::<IStrategy<RevmMiddleware>>::new(
        &environment,
        &token_admin,
        weight_changer.lex.address(),
        weight_changer.g3m.address(),
    )
    .await?;

    EventLogger::builder()
        .path(config.output_directory)
        .add(price_changer.liquid_exchange.events(), "lex")
        .add(weight_changer.g3m.events(), "g3m")
        .run()?;

    Ok(Simulation {
        agents: Agents::new()
            .add(price_changer)
            .add(arbitrageur)
            .add(block_admin)
            .add(weight_changer)
            .add(lp),
        steps: config.trajectory.num_steps,
        environment,
    })
}
