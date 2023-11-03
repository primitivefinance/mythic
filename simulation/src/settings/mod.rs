pub mod parameters;
use std::{collections::BTreeMap, env, path::Path};

use parameters::*;
use unit_conversions::time::*;

use super::*;
use crate::{
    agents::{price_changer::PriceChangerParameters, swapper::SwapperParameters, AgentParameters},
    simulations::SimulationType,
};

pub use self::parameters::Parameterized;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SimulationConfig<P: Parameterized> {
    pub simulation: SimulationType,
    pub max_parallel: Option<usize>,
    pub output_directory: String,
    pub output_file_name: Option<String>,
    pub agent_parameters: BTreeMap<String, AgentParameters<P>>,
}

impl SimulationConfig<Multiple> {
    pub fn new(config_path: &str) -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(config::File::with_name(config_path))
            .build()?;
        s.try_deserialize()
    }
}

impl From<SimulationConfig<Multiple>> for Vec<SimulationConfig<Single>> {
    fn from(item: SimulationConfig<Multiple>) -> Self {
        let agent_parameters_multiple = item.agent_parameters;
        let keys: Vec<String> = agent_parameters_multiple.keys().cloned().collect();
        let values_multiple: Vec<AgentParameters<Multiple>> =
            agent_parameters_multiple.values().cloned().collect();

        let values_single: Vec<Vec<AgentParameters<Single>>> =
            values_multiple.into_iter().map(|v| v.into()).collect();

        let mut configs: Vec<SimulationConfig<Single>> = Vec::new();

        for i in 0..values_single[0].len() {
            let mut agent_parameters_single: BTreeMap<String, AgentParameters<Single>> =
                BTreeMap::new();
            for (key, values) in keys.iter().zip(values_single.iter()) {
                agent_parameters_single.insert(key.clone(), values[i].clone());
            }
            let config_single = SimulationConfig {
                simulation: item.simulation,
                max_parallel: item.max_parallel,
                output_directory: item.output_directory.clone(),
                output_file_name: item.output_file_name.clone(),
                agent_parameters: agent_parameters_single,
            };
            configs.push(config_single);
        }

        configs
    }
}

#[cfg(test)]
mod tests {
    // TODO: Generate the static and sweep configs here.
    use super::*;

    #[test]
    fn read_in_static() {
        let config = SimulationConfig::new("src/tests/configs/static.toml").unwrap();
        let configs: Vec<SimulationConfig<Single>> = config.into();
        println!("{:?}", configs);
        assert_eq!(configs.len(), 1);
        assert_eq!(configs[0].simulation, SimulationType::DynamicWeights);
        assert_eq!(
            configs[0].output_directory,
            "src/tests/configs/test_static_output".to_string(),
        );
        let agent_parameters = configs[0].agent_parameters.clone();
        assert_eq!(agent_parameters.len(), 4);
    }

    // assert_eq!(configs[0].trajectory.process, "gbm");
    // assert_eq!(configs[0].trajectory.initial_price, 1.0);
    // assert_eq!(configs[0].trajectory.t_0, 0.0);
    // assert_eq!(configs[0].trajectory.t_n, 1.0);
    // assert_eq!(configs[0].trajectory.num_steps, 100);
    // assert_eq!(configs[0].trajectory.seed, 2);
    // assert_eq!(configs[0].gbm.unwrap().drift, 0.1);
    // assert_eq!(configs[0].gbm.unwrap().volatility, 0.3);
    // assert_eq!(configs[0].pool.fee_basis_points, 30);
    // assert_eq!(configs[0].pool.weight_x, 0.5);
    // assert_eq!(configs[0].pool.target_volatility, 0.15);
    // assert_eq!(configs[0].lp.x_liquidity, 1.0);
    // assert_eq!(configs[0].block.timestep_size, 15);
    // assert_eq!(
    //     configs[0]
    //         .weight_changer
    //         .volatility_targeting
    //         .unwrap()
    //         .target_volatility,
    //     0.15
    // );
    // assert_eq!(
    //     configs[0]
    //         .weight_changer
    //         .volatility_targeting
    //         .unwrap()
    //         .update_frequency,
    //     150
    // );
    #[test]
    fn read_in_sweep() {
        todo!()
        // let config = SimulationConfig::new("configs/test/sweep.toml").unwrap();
        // let configs = config.generate();
        // assert_eq!(configs.len(), 8);
        // assert_eq!(configs[0].gbm.unwrap().drift, -1.0);
        // assert_eq!(configs[1].gbm.unwrap().drift, -1.0);
        // assert_eq!(configs[2].gbm.unwrap().drift, 1.0);
        // assert_eq!(configs[3].gbm.unwrap().drift, 1.0);
        // assert_eq!(configs[4].gbm.unwrap().drift, -1.0);
        // assert_eq!(configs[5].gbm.unwrap().drift, -1.0);
        // assert_eq!(configs[6].gbm.unwrap().drift, 1.0);
        // assert_eq!(configs[7].gbm.unwrap().drift, 1.0);
        // assert_eq!(configs[0].gbm.unwrap().volatility, 0.0);
        // assert_eq!(configs[1].gbm.unwrap().volatility, 1.0);
        // assert_eq!(configs[2].gbm.unwrap().volatility, 0.0);
        // assert_eq!(configs[3].gbm.unwrap().volatility, 1.0);
        // assert_eq!(configs[4].gbm.unwrap().volatility, 0.0);
        // assert_eq!(configs[5].gbm.unwrap().volatility, 1.0);
        // assert_eq!(configs[6].gbm.unwrap().volatility, 0.0);
        // assert_eq!(configs[7].gbm.unwrap().volatility, 1.0);
    }
}
