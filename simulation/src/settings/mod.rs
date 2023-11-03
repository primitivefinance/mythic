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
use itertools::{Itertools, MultiProduct};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SimulationConfig<P: Parameterized> {
    pub simulation: SimulationType,
    pub max_parallel: Option<usize>,
    pub output_directory: String,
    pub output_file_name: Option<String>,
    #[serde(rename = "agent")]
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
        println!("{:?}\n", item);
        let mut index = 0;
        let mut configs = Vec::new();
        let mut map_vector: BTreeMap<String, Vec<AgentParameters<Single>>> = BTreeMap::new();
        for (label, parameters) in &item.agent_parameters {
            let parameters_multiple = parameters.clone();
            let parameters: Vec<AgentParameters<Single>> = parameters_multiple.into();
            map_vector.insert(label.clone(), parameters);
        }
        let combinations = map_vector.values().multi_cartesian_product();
        for combination in combinations {
            let mut config = SimulationConfig {
                simulation: item.simulation,
                max_parallel: item.max_parallel,
                output_directory: item.output_directory.clone(),
                output_file_name: Some(index.to_string()),
                agent_parameters: BTreeMap::new(),
            };
            for (label, parameters) in map_vector.keys().zip(combination) {
                config
                    .agent_parameters
                    .insert(label.clone(), parameters.clone()); // Clone the parameters as they are owned by the combination
            }
            configs.push(config);
            index += 1;
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
        let configs = SimulationConfig::new("src/tests/configs/static.toml").unwrap();
        let configs: Vec<SimulationConfig<Single>> = configs.into();
        let config = configs[0].clone();
        println!("{:?}\n", configs);
        assert_eq!(configs.len(), 1);
        assert_eq!(config.simulation, SimulationType::DynamicWeights);
        assert_eq!(
            config.output_directory,
            "src/tests/configs/test_static_output".to_string(),
        );
        let agent_parameters = config.agent_parameters.clone();
        assert_eq!(agent_parameters.len(), 4);

        assert!(agent_parameters.get("block_admin").is_some());
        assert!(agent_parameters.get("token_admin").is_some());
        assert!(agent_parameters.get("price_changer").is_some());
        assert!(agent_parameters.get("weight_changer").is_some());
    }

    #[test]
    fn read_in_sweep() {
        let config = SimulationConfig::new("src/tests/configs/sweep.toml").unwrap();
        let configs: Vec<SimulationConfig<Single>> = config.into();
        println!("{:?}\n", configs);
        assert_eq!(configs.len(), 512);
    }
}
