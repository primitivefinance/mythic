use super::*;
use crate::agents::Agent;
use crate::{
    agents::{
        arbitrageur::Arbitrageur, block_admin::BlockAdmin, liquidity_provider::LiquidityProvider,
        price_changer::PriceChanger, token_admin::TokenAdmin, weight_changer::WeightChanger,
    },
    settings::SimulationConfig,
};
use arbiter_core::environment::builder::BlockSettings;

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

    let mut lp = LiquidityProvider::<G3M<RevmMiddleware>>::new(
        &env,
        &token_admin,
        weight_changer.g3m.address(),
        &config,
    )
    .await?;
    let mut arbitrageur = Arbitrageur::<G3M<RevmMiddleware>>::new(
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

    let mut steppers: Vec<Box<dyn Agent>> = vec![];
    steppers.push(Box::new(price_changer));
    steppers.push(Box::new(arbitrageur));
    steppers.push(Box::new(block_admin));
    steppers.push(Box::new(weight_changer));
    steppers.push(Box::new(lp));

    info!("Entering startup loop for agents.");
    for stepper in steppers.iter_mut() {
        stepper.startup().await?;
    }

    let total_steps = 1; //config.trajectory.num_steps; // todo: for testing we are just running one loop right now, just replace with the trajectory steps.
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
