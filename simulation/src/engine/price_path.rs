//! ALl utilities for building and using a price path.

use arbiter_core::{
    math::{ornstein_uhlenbeck::OrnsteinUhlenbeck, StochasticProcess, Trajectories},
    middleware::RevmMiddleware,
};
use tracing::info;

use crate::settings::params::PriceProcessParameters;
use arbiter_core::bindings::liquid_exchange::LiquidExchange;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// PriceChanger
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// The `PriceChanger` holds the data and has methods that allow it to update
/// the price of the `LiquidExchange`.
pub struct PriceChanger {
    /// The path the price process takes.
    pub trajectory: Trajectories,

    /// The `LiquidExchange` contract with the admin `Client`.
    pub liquid_exchange: LiquidExchange<RevmMiddleware>,

    /// The index of the current price in the trajectory.
    pub index: usize,
}

impl PriceChanger {
    /// Create a new `PriceChanger` with the given `LiquidExchange` contract
    /// bound to the admin `Client`. The `PriceChanger` will use the
    /// `OrnsteinUhlenbeck` process to generate a price trajectory with the
    /// constants defined in `config.rs`.
    /// Ornstein-Uhlenbeck processes are useful for modeling the price of stable
    /// tokens.
    pub fn new(
        liquid_exchange: LiquidExchange<RevmMiddleware>,
        price_process_params: PriceProcessParameters,
    ) -> Self {
        let PriceProcessParameters {
            initial_price,
            mean,
            std_dev,
            theta,
            t_0,
            t_n,
            num_steps,
            seed,
        } = price_process_params;
        let process = OrnsteinUhlenbeck::new(mean, std_dev, theta);

        let trajectory = match seed {
            Some(seed) => {
                process.seedable_euler_maruyama(initial_price, t_0, t_n, num_steps, 1, false, seed)
            }
            None => process.euler_maruyama(initial_price, t_0, t_n, num_steps, 1, false),
        };

        Self {
            trajectory,
            liquid_exchange,
            index: 1, /* start after the initial price since it is already set on contract
                       * deployment */
        }
    }

    /// Update the price of the `LiquidExchange` contract to the next price in
    /// the trajectory and increment the index.
    /// !!IMPORTANT!! The update price function will have a real integer value that is smaller than the float value.
    pub async fn update_price(&mut self) -> Result<(), anyhow::Error> {
        let price = self.trajectory.paths[0][self.index];
        info!("Updating price of liquid_exchange to: {}", price);
        self.liquid_exchange
            .set_price(arbiter_core::math::float_to_wad(price))
            .send()
            .await?
            .await?;
        self.index += 1;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::settings::{self, params::SimulationConfig};
    use crate::setup::{self, deploy::Contracts};
    use arbiter_core::environment::Environment;
    use arbiter_core::{environment::builder::EnvironmentBuilder, math::wad_to_float};

    async fn setup_test_environment(
    ) -> Result<(Contracts, Environment, SimulationConfig), anyhow::Error> {
        let config = settings::params::SimulationConfig::new()?;

        let env = EnvironmentBuilder::new().build();
        let contracts = setup::deploy::deploy_contracts(&env, &config).await?;

        Ok((contracts, env, config))
    }

    #[tokio::test]
    async fn test_price_path_update() -> Result<(), anyhow::Error> {
        let (contracts, env, config) = setup_test_environment().await?;

        let mut price_changer = PriceChanger::new(
            contracts.exchanges.lex.clone(),
            config.price_process_parameters.clone(),
        );

        let initial_price = price_changer.trajectory.paths[0][0];
        let initial_index = price_changer.index;

        let _ = price_changer.update_price().await?;

        let new_price = price_changer.trajectory.paths[0][1];
        let new_index = price_changer.index;
        let new_price_lex = contracts.exchanges.lex.price().call().await?;

        assert_ne!(initial_price, new_price);
        assert_eq!(initial_index + 1, new_index);
        assert_eq!(new_price, wad_to_float(new_price_lex));

        // print these prices in the assertions
        println!("initial_price: {}", initial_price);
        println!("new_price: {}", new_price);
        println!("new_price_lex: {}", new_price_lex);

        Ok(())
    }
}
