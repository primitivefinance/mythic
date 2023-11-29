use arbiter_core::environment::builder::BlockSettings;
use tracing::event;

use super::{errors::SimulationError, *};
use crate::{
    agents::{
        block_admin::BlockAdmin,
        g3m::{
            arbitrageur::Arbitrageur,
            g3m_portfolio_manager::{
                momentum::MomentumStrategist, volatility_targeting::VolatilityTargetingStrategist,
                G3mPortfolioManager, G3mPortfolioManagerType,
            },
            liquidity_provider::LiquidityProvider,
            swapper::Swapper,
        },
        price_changer::{PriceChanger, PriceChangerParameters},
        token_admin::TokenAdmin,
        Agent, Agents,
    },
    bindings::i_strategy::IStrategy,
    settings::SimulationConfig,
    strategy::g3m::G3mStrategy,
};

pub async fn setup(
    environment: Environment,
    config: SimulationConfig<Single>,
) -> Result<Simulation, SimulationError> {
    let mut agents = Agents::new();
    let mut event_logger = EventLogger::builder()
        .directory(config.output_directory.clone())
        .file_name(config.output_file_name.clone().unwrap());

    let mut block_admin = BlockAdmin::new(&environment, &config, "block_admin").await?;
    agents.add(block_admin);

    let token_admin = TokenAdmin::new(&environment, &config, "token_admin").await?;
    agents.add(token_admin.clone());

    let mut price_changer =
        PriceChanger::new(&environment, &config, "price_changer", &token_admin).await?;
    agents.add(price_changer.clone());
    event_logger = event_logger.add(price_changer.liquid_exchange.events(), "lex");

    let portfolio_manager = G3mPortfolioManagerType::new(
        &environment,
        &config,
        "portfolio_manager",
        price_changer.liquid_exchange.address(),
    )
    .await?;
    let strategy_address = portfolio_manager.g3m().address();
    event_logger = event_logger.add(portfolio_manager.g3m().events(), "g3m");
    agents.add(portfolio_manager);

    let mut lp = LiquidityProvider::<G3mStrategy>::new(
        &environment,
        &config,
        "lp",
        &token_admin,
        strategy_address,
    )
    .await?;
    agents.add(lp);

    let mut arbitrageur = Arbitrageur::<G3mStrategy>::new(
        &environment,
        &token_admin,
        price_changer.liquid_exchange.address(),
        strategy_address,
    )
    .await?;
    agents.add(arbitrageur);

    match Swapper::new(
        &environment,
        &config,
        "swapper",
        &price_changer,
        &token_admin,
    )
    .await
    {
        Ok(swapper) => {
            agents.add(swapper.clone());
            event_logger =
                event_logger.add(swapper.portfolio_tracker.events(), "portfolio_tracker");
        }
        Err(e) => {
            warn!("Swapper not initialized: {}", e);
        }
    };

    event_logger
        .metadata(config)
        .map_err(|e| SimulationError::GenericError(e.to_string()))?
        .run()
        .map_err(|e| SimulationError::GenericError(e.to_string()))?;
    let steps = price_changer.trajectory.paths[0].len() - 1;

    Ok(Simulation {
        agents,
        steps,
        environment,
    })
}
