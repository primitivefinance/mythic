use arbiter_core::environment::builder::BlockSettings;

use crate::simulations::{dynamic_weights, Simulation};

use super::*;
mod swapper_integration;

async fn startup_static() -> Simulation {
    let config_path = "src/tests/configs/static.toml";
    let config = simulations::import(config_path).unwrap();
    let configs: Vec<SimulationConfig<Single>> = config.into();
    let config = configs[0].clone();
    let environment = EnvironmentBuilder::new()
        .block_settings(BlockSettings::UserControlled)
        .label(config.output_file_name.clone().unwrap())
        .build();
    dynamic_weights::setup(environment, config).await.unwrap()
}