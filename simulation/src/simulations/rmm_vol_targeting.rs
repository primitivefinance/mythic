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
    let mut block_admin = BlockAdmin::new(&environment, &config, "block_admin").await?;
    let token_admin = TokenAdmin::new(&environment, &config, "token_admin").await?;
    let mut price_changer =
        PriceChanger::new(&environment, &config, "price_changer", &token_admin).await?;
    let rmm_portfolio_manager = PortfolioManagerType::new(
        &environment,
        &config,
        "portfolio_manager",
        price_changer.liquid_exchange.address(),
    )
    .await?;
    let mut lp = RmmLiquidityProvider::<RmmStrategy>::new(
        &environment,
        &config,
        "lp",
        &token_admin,
        rmm_portfolio_manager.0.low_vol_pool().address(),
        rmm_portfolio_manager.0.high_vol_pool().address(),
    )
    .await?;
    let mut arbitrageur = RmmArbitrageur::<RmmStrategy>::new(
        &environment,
        &token_admin,
        price_changer.liquid_exchange.address(),
        rmm_portfolio_manager.0.low_vol_pool().address(),
        rmm_portfolio_manager.0.high_vol_pool().address(),
    )
    .await?;

    println!(
        "low vol pool: {}",
        rmm_portfolio_manager.0.low_vol_pool().address()
    );
    println!(
        "high vol pool: {}",
        rmm_portfolio_manager.0.high_vol_pool().address()
    );

    EventLogger::builder()
        .add(price_changer.liquid_exchange.events(), "lex")
        .add(
            rmm_portfolio_manager.0.low_vol_pool().events(),
            "low_vol_pool",
        )
        .add(
            rmm_portfolio_manager.0.high_vol_pool().events(),
            "low_vol_pool",
        )
        .run()
        .map_err(|e| SimulationError::GenericError(e.to_string()))?;

    let steps = price_changer.trajectory.paths[0].len() - 1;

    Ok(Simulation {
        agents: Agents::new()
            .add(price_changer)
            .add(arbitrageur)
            .add(block_admin)
            .add(rmm_portfolio_manager)
            .add(lp),
        steps,
        environment,
    })
}
