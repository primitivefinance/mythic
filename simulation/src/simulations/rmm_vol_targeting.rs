use arbiter_core::bindings::arbiter_math::ArbiterMath;
use arbiter_core::environment::builder::BlockSettings;

use super::{errors::SimulationError, *};
use crate::{
    agents::{
        block_admin::BlockAdmin,
        price_changer::PriceChanger,
        rmm::{
            arbitrageur::RmmArbitrageur, liquidity_provider::RmmLiquidityProvider,
            portfolio_manager::PortfolioManagerType,
        },
        token_admin::TokenAdmin,
        Agent, Agents,
    },
    bindings::i_strategy::IStrategy,
    settings::SimulationConfig,
    strategy::rmm::RmmStrategy,
};

pub async fn setup(
    environment: Environment,
    config: SimulationConfig<Single>,
) -> Result<Simulation, SimulationError> {
    let mut agents = Agents::new();

    let mut block_admin = BlockAdmin::new(&environment, &config, "block_admin").await?;
    agents.add(block_admin);

    let token_admin = TokenAdmin::new(&environment, &config, "token_admin").await?;
    agents.add(token_admin.clone());

    let mut price_changer =
        PriceChanger::new(&environment, &config, "price_changer", &token_admin).await?;
    agents.add(price_changer.clone());

    let rmm_portfolio_manager = PortfolioManagerType::new(
        &environment,
        &config,
        "portfolio_manager",
        price_changer.liquid_exchange.address(),
    )
    .await?;
    agents.add(rmm_portfolio_manager.clone());

    let mut lp = RmmLiquidityProvider::<RmmStrategy>::new(
        &environment,
        &config,
        "lp",
        &token_admin,
        rmm_portfolio_manager.0.rmm().address(),
    )
    .await?;
    agents.add(lp);

    let mut arbitrageur = RmmArbitrageur::<RmmStrategy>::new(
        &environment,
        &token_admin,
        price_changer.liquid_exchange.address(),
        rmm_portfolio_manager.0.rmm().address(),
    )
    .await?;
    agents.add(arbitrageur.clone());

    EventLogger::builder()
        .directory(config.output_directory.clone())
        .file_name(config.output_file_name.clone().unwrap())
        .add(price_changer.liquid_exchange.events(), "lex")
        .add(rmm_portfolio_manager.0.rmm().events(), "rmm")
        .add(token_admin.arbx.events(), "arbx")
        .add(token_admin.arby.events(), "arby")
        .add(arbitrageur.atomic_arbitrage.events(), "atomic_arbitrage")
        .run()
        .map_err(|e| SimulationError::GenericError(e.to_string()))?;

    let steps = price_changer.trajectory.paths[0].len() - 1;

    Ok(Simulation {
        agents,
        steps,
        environment,
    })
}
