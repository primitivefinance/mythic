use arbiter_core::environment::builder::BlockSettings;

use super::{errors::SimulationError, *};
use crate::{
    agents::{
        arbitrageur::Arbitrageur, block_admin::BlockAdmin, liquidity_provider::LiquidityProvider,
        momentum_strategist::MomentumStrategist, price_changer::PriceChanger,
        token_admin::TokenAdmin, Agent, Agents,
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
    let mut momentum_strategist = MomentumStrategist::new(
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
        momentum_strategist.g3m.address(),
        &config,
    )
    .await?;
    let mut arbitrageur = Arbitrageur::<IStrategy<RevmMiddleware>>::new(
        &environment,
        &token_admin,
        momentum_strategist.lex.address(),
        momentum_strategist.g3m.address(),
    )
    .await?;

    EventLogger::builder()
        .add(price_changer.liquid_exchange.events(), "lex")
        .add(momentum_strategist.g3m.events(), "g3m")
        .run()?;

    Ok(Simulation {
        agents: Agents::new()
            .add(price_changer)
            .add(arbitrageur)
            .add(block_admin)
            .add(momentum_strategist)
            .add(lp),
        steps: config.trajectory.num_steps,
        environment,
    })
}
