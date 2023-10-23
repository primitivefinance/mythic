use arbiter_core::environment::builder::BlockSettings;

use super::*;
use crate::{
    agents::{
        arbitrageur::Arbitrageur, block_admin::BlockAdmin, liquidity_provider::LiquidityProvider,
        price_changer::PriceChanger, token_admin::TokenAdmin, weight_changer::WeightChanger, Agent,
    },
    bindings::i_strategy::IStrategy,
    settings::SimulationConfig,
};

pub async fn run(config_path: &str) -> Result<()> {
    let config = SimulationConfig::new(config_path)?;

    let env = EnvironmentBuilder::new()
        .block_settings(BlockSettings::UserControlled)
        .build();
    let mut block_admin = BlockAdmin::new(&env, &config).await?;

    let token_admin = TokenAdmin::new(&env).await?;
    let mut price_changer = PriceChanger::new(&env, &token_admin, &config).await?;
    let mut weight_changer = WeightChanger::new(
        &env,
        &config,
        price_changer.liquid_exchange.address(),
        token_admin.arbx.address(),
        token_admin.arby.address(),
    )
    .await?;

    let mut lp = LiquidityProvider::<IStrategy<RevmMiddleware>>::new(
        &env,
        &token_admin,
        weight_changer.g3m.address(),
        &config,
    )
    .await?;
    let mut arbitrageur = Arbitrageur::<IStrategy<RevmMiddleware>>::new(
        &env,
        &token_admin,
        weight_changer.lex.address(),
        weight_changer.g3m.address(),
    )
    .await?;

    EventLogger::builder()
        .path("./analysis/test_data")
        .add(price_changer.liquid_exchange.events(), "lex")
        .add(weight_changer.g3m.events(), "g3m")
        .run()?;

    let mut steppers: Vec<Box<dyn Agent>> = vec![
        Box::new(price_changer),
        Box::new(arbitrageur),
        Box::new(block_admin),
        Box::new(weight_changer),
        Box::new(lp),
    ];

    info!("Entering startup loop for agents.");
    for stepper in steppers.iter_mut() {
        stepper.startup().await?;
    }

    let total_steps = config.trajectory.num_steps;
    for index in 0..total_steps {
        info!("Entering priority loop for index: {}", index);
        for stepper in steppers.iter_mut() {
            stepper.priority_step().await?;
        }

        info!("Entering core loop for index: {}", index);
        for stepper in steppers.iter_mut() {
            stepper.step().await?;
        }
    }

    Ok(())
}
