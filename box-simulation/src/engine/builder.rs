//! Builds the core simulation logic into the engine so it's run in the main loop.
use arbiter_core::environment::Environment;

use crate::engine::PriceChanger;
use crate::settings::{self, params::SimulationConfig};
use crate::setup::{self, deploy::Contracts};

/// A specific type for the async action function that uses contracts and environment
/// to run the core simulation logic.
/// Implements the Sized trait so it can be used as a type.
pub type Action = Box<dyn Fn(&Contracts) -> Result<(), anyhow::Error> + Send + Sync>;

/// A builder for creating the simulation loop by specifying the actions to run inside the main sim loop.
/// before_hooks - Async functions that that are run inside the sim loop before the core logic.
/// after_hooks - Async functions that that are run inside the sim loop after the core logic.
pub struct EngineBuilder {
    pub before_hooks: Vec<Action>,
    pub after_hooks: Vec<Action>,
}

/// Initializes the simulation engine with two vectors of actions to run before and after the core simulation logic.
/// Calling the `run` method on the engine will run the before hooks, then the core simulation logic, then the after hooks within the sim loop.
impl EngineBuilder {
    pub fn new() -> Self {
        Self {
            before_hooks: Vec::new(),
            after_hooks: Vec::new(),
        }
    }

    /// Adds a before hook to the engine.
    pub fn before(mut self, hook: Action) -> Self {
        self.before_hooks.push(hook);
        self
    }

    pub async fn run(
        &self,
        contracts: &Contracts,
        env: &Environment,
        config: &SimulationConfig,
    ) -> Result<(), anyhow::Error> {
        // Run the main simulation loop
        let mut price_changer = PriceChanger::new(
            contracts.exchanges.lex.clone(),
            config.price_process_parameters.clone(),
        );

        for _ in 0..(price_changer.trajectory.paths[0].len() - 1) {
            // Run the before hooks
            for hook in self.before_hooks.iter() {
                hook(contracts)?;
            }

            // Run the core simulation logic, which is updating the price.
            price_changer.update_price().await?;

            // Run the after hooks
            for hook in self.after_hooks.iter() {
                hook(contracts)?;
            }
        }

        Ok(())
    }
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
    async fn test_engine_builder_vanilla() -> Result<(), anyhow::Error> {
        let (contracts, env, config) = setup_test_environment().await?;

        let engine_builder = EngineBuilder::new().run(&contracts, &env, &config).await?;

        let initial_price_float = config.price_process_parameters.initial_price;
        let lex_price_float = wad_to_float(contracts.exchanges.lex.price().call().await?);

        // Assert the price of the liquid exchange != initial config price to ensure the sim's core logic was run.
        assert_ne!(
            initial_price_float, lex_price_float,
            "The price of the liquid exchange should have been updated by the simulation's core logic."
        );

        println!("initial_price_float: {}", initial_price_float);
        println!("lex_price_float: {}", lex_price_float);

        Ok(())
    }

    #[tokio::test]
    async fn test_engine_builder_add_before_hook() -> Result<(), anyhow::Error> {
        let (contracts, env, config) = setup_test_environment().await?;

        let engine_builder = EngineBuilder::new()
            .before(Box::new(|contracts| {
                println!("before hook");
                Ok(())
            }))
            .run(&contracts, &env, &config)
            .await?;

        Ok(())
    }
}
