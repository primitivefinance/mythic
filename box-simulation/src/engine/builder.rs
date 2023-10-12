//! Builds the core simulation logic into the engine so it's run in the main loop.
//! The main part of the "simulation loop" is to update the reference price of the liquid exchange.
//! All simulation logic occurs around this main event, so this builder is used to easily
//! add before and after hooks to this main event.
use arbiter_core::environment::Environment;

use super::price_path::PriceChanger;
use crate::settings::params::SimulationConfig;
use crate::setup::deploy::Contracts;

use std::future::Future;
use std::pin::Pin;

// Step 1: Define our return type for async functions
type AsyncResult = Result<(), anyhow::Error>;

// Step 2: Define the basic Future type that our async functions will return
type OurFuture<'a> = dyn Future<Output = AsyncResult> + Send + 'a;

// Step 3: Pin that Future because async functions often return Pin<Box<FutureType>>
type PinnedFuture<'a> = Pin<Box<OurFuture<'a>>>;

// Step 4: Define the type for functions that take a &Contracts and return our PinnedFuture
type FunctionType<'a> = dyn Fn(&'a Contracts) -> PinnedFuture<'a> + Send;

// Step 5: Box the function type so we can store it easily in a Vec
// The lifetime 'a is the lifetime of the Contracts reference, because we need to make sure its still alive!
pub type Action<'a> = Box<FunctionType<'a>>;

/// A builder for creating the simulation loop by specifying the actions to run inside the main sim loop.
/// before_hooks - Async functions that that are run inside the sim loop before the core logic.
/// after_hooks - Async functions that that are run inside the sim loop after the core logic.
pub struct EngineBuilder<'a> {
    pub before_hooks: Vec<Action<'a>>,
    pub after_hooks: Vec<Action<'a>>,
}

/// Initializes the simulation engine with two vectors of actions to run before and after the core simulation logic.
/// Calling the `run` method on the engine will run the before hooks, then the core simulation logic, then the after hooks within the sim loop.
impl<'a> EngineBuilder<'a> {
    /// Creates a new engine builder with empty hook vectors.
    pub fn new() -> Self {
        Self {
            before_hooks: Vec::new(),
            after_hooks: Vec::new(),
        }
    }

    /// Adds a before hook to the engine.
    pub fn before(mut self, hook: Action<'a>) -> Self {
        self.before_hooks.push(hook);
        self
    }

    /// Adds an after hook to the engine.
    pub fn after(mut self, hook: Action<'a>) -> Self {
        self.after_hooks.push(hook);
        self
    }

    /// Creates a "PriceChanger", which has a vector of a price trajectory.
    /// Loops over that trajectory and runs the before hooks, updates the price, then the after hooks.
    pub async fn run(
        &self,
        contracts: &'a Contracts,
        env: &Environment,
        config: &SimulationConfig,
    ) -> Result<(), anyhow::Error> {
        // Run the main simulation loop
        let mut price_changer = PriceChanger::new(
            // contracts.exchanges.lex.clone(),
            config.price_process_parameters.clone(),
        );

        for _ in 0..(price_changer.trajectory.paths[0].len() - 1) {
            // Run the before hooks
            for func in &self.before_hooks {
                func(contracts).await?;
            }

            // Run the core simulation logic, which is updating the price.
            price_changer.update_price().await?;

            // Run the after hooks
            for func in &self.after_hooks {
                func(contracts).await?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::settings::{self, params::SimulationConfig};
    use crate::setup::{self, deploy::Contracts};
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

        let _ = EngineBuilder::new().run(&contracts, &env, &config).await?;
        let initial_price_float = config.price_process_parameters.initial_price;
        let lex_price_float = wad_to_float(contracts.exchanges.lex.price().call().await?);

        // Assert the price of the liquid exchange != initial config price to ensure the sim's core logic was run.
        assert_ne!(
            initial_price_float, lex_price_float,
            "The price of the liquid exchange should have been updated by the simulation's core logic."
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_engine_builder_add_before_hook() -> Result<(), anyhow::Error> {
        let (contracts, env, config) = setup_test_environment().await?;

        // Run a before hook to increment the swap fee of the g3m exchange by 1.
        let _ = EngineBuilder::new()
            .before(Box::new(|contracts| {
                Box::pin(async move {
                    let current_swap_fee = contracts.exchanges.g3m.swap_fee().call().await?;
                    contracts
                        .exchanges
                        .g3m
                        .set_swap_fee((current_swap_fee + 1).into())
                        .send()
                        .await?
                        .await?;

                    Ok(())
                })
            }))
            .run(&contracts, &env, &config)
            .await?;

        // Assert the swap_fee is equal to the nonce of the price changer trajectory.
        // We incremented the swap fee by 1 on each step,
        // so the actual swap fee should be equal to initial swap fee + the number of steps in the trajectory.
        let actual_swap_fee = contracts.exchanges.g3m.swap_fee().call().await?;
        let expected_swap_fee = config.portfolio_pool_parameters.fee_basis_points as u128
            + config.price_process_parameters.num_steps as u128;

        assert_eq!(
            actual_swap_fee.as_u128(), expected_swap_fee,
            "The swap fee should have been incremented by the number of steps in the price changer trajectory."
        );

        Ok(())
    }
}
