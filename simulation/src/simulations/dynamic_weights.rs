use arbiter_core::environment::builder::BlockSettings;

use super::{errors::SimulationError, *};
use crate::{
    agents::{
        arbitrageur::Arbitrageur,
        block_admin::BlockAdmin,
        liquidity_provider::LiquidityProvider,
        price_changer::{PriceChanger, PriceChangerParameters},
        token_admin::TokenAdmin,
        weight_changer::{
            momentum::MomentumStrategist, volatility_targeting::VolatilityTargetingStrategist,
            WeightChanger, WeightChangerType,
        },
        Agent, Agents,
    },
    bindings::i_strategy::IStrategy,
    settings::SimulationConfig,
};

pub async fn setup(config: SimulationConfig<Single>) -> Result<Simulation, SimulationError> {
    let environment = EnvironmentBuilder::new()
        .block_settings(BlockSettings::UserControlled)
        .build();

    let mut block_admin = BlockAdmin::new(&environment, &config, "block_admin").await?;
    let token_admin = TokenAdmin::new(&environment, &config, "token_admin").await?;
    let mut price_changer =
        PriceChanger::new(&environment, &config, "price_changer", &token_admin).await?;

    let weight_changer = WeightChangerType::new(
        &environment,
        &config,
        "weight_changer",
        price_changer.liquid_exchange.address(),
    )
    .await?;

    let mut lp = LiquidityProvider::<IStrategy<RevmMiddleware>>::new(
        &environment,
        &config,
        &token_admin,
        weight_changer.g3m().address(),
    )
    .await?;

    let mut arbitrageur = Arbitrageur::<IStrategy<RevmMiddleware>>::new(
        &environment,
        &token_admin,
        weight_changer.lex().address(),
        weight_changer.g3m().address(),
    )
    .await?;

    EventLogger::builder()
        .directory(config.output_directory)
        .file_name(config.output_file_name.unwrap())
        .add(price_changer.liquid_exchange.events(), "lex")
        .add(weight_changer.g3m().events(), "g3m")
        .run()
        .map_err(|e| SimulationError::GenericError(e.to_string()))?;
    let steps = price_changer.trajectory.paths[0].len();
    Ok(Simulation {
        agents: Agents::new()
            .add(price_changer)
            .add(arbitrageur)
            .add(block_admin)
            .add(weight_changer)
            .add(lp),
        steps,
        environment,
    })
}
