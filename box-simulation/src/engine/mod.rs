use crate::settings::{self, params::SimulationConfig};
use crate::setup::{self, deploy::Contracts};

use self::price_path::PriceChanger;
use arbiter_core::environment::Environment;

pub mod builder;
pub mod price_path;

/// Runs the main simulation loop.
pub async fn run(
    contracts: &Contracts,
    env: &Environment,
    config: &SimulationConfig,
) -> Result<PriceChanger, anyhow::Error> {
    // Get the initial price path from the configuration.
    let mut price_changer = price_path::PriceChanger::new(
        contracts.exchanges.lex.clone(),
        config.price_process_parameters.clone(),
    );

    for _ in 0..(price_changer.trajectory.paths[0].len() - 1) {
        price_changer.update_price().await?;
    }

    Ok(price_changer)
}

#[cfg(test)]
mod tests {
    use super::*;
    use arbiter_core::{environment::builder::EnvironmentBuilder, math::wad_to_float};

    async fn setup_test_environment(
    ) -> Result<(Contracts, Environment, SimulationConfig), anyhow::Error> {
        let config = settings::params::SimulationConfig::new()?;

        let env = EnvironmentBuilder::new().build();
        let contracts = setup::deploy::deploy_contracts(&env, &config).await?;

        Ok((contracts, env, config))
    }

    #[tokio::test]
    async fn test_price_path_update_in_loop() -> Result<(), anyhow::Error> {
        let (contracts, env, config) = setup_test_environment().await?;

        let mut price_changer = run(&contracts, &env, &config).await?;

        let price_index = price_changer.index;

        let new_price_lex = contracts.exchanges.lex.price().call().await?;

        // Assert the price of the lex is the same as the last price in the trajectory
        assert_eq!(
            price_changer.trajectory.paths[0][price_index - 1],
            wad_to_float(new_price_lex)
        );

        // Assert we reached the end of the trajectory
        assert_eq!(price_changer.index, price_changer.trajectory.paths[0].len());

        // Print the last price in the trajectory
        println!(
            "last price in trajectory: {}",
            price_changer.trajectory.paths[0][price_index - 1]
        );

        // Print price of lex
        println!("price of lex: {}", new_price_lex);

        Ok(())
    }
}
