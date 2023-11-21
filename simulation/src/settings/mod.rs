pub mod parameters;
use std::{collections::BTreeMap, env, hash::Hasher, path::Path};

use itertools::{Itertools, MultiProduct};
use parameters::*;
use unit_conversions::time::*;

pub use self::parameters::Parameterized;
use super::*;
use crate::{
    agents::{
        block_admin::BlockAdminParameters, g3m::swapper::SwapperParameters,
        price_changer::PriceChangerParameters, AgentParameters,
    },
    simulations::SimulationType,
};

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

impl<P> Default for SimulationConfig<P>
where
    P: Parameterized,
{
    fn default() -> Self {
        Self {
            simulation: SimulationType::DynamicWeights,
            max_parallel: None,
            output_directory: "output".to_string(),
            output_file_name: None,
            agent_parameters: BTreeMap::new(),
        }
    }
}

#[derive(Default)]
pub struct SimulationConfigBuilder {
    simulation: Option<SimulationType>,
    max_parallel: Option<usize>,
    output_directory: Option<String>,
    output_file_name: Option<String>,
    agent_parameters: Option<BTreeMap<String, AgentParameters<Single>>>,
}

impl SimulationConfigBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn simulation(&mut self, simulation: SimulationType) -> &mut SimulationConfigBuilder {
        self.simulation = Some(simulation);
        self
    }

    pub fn max_parallel(&mut self, max_parallel: usize) -> &mut SimulationConfigBuilder {
        self.max_parallel = Some(max_parallel);
        self
    }

    pub fn output_directory(&mut self, output_directory: String) -> &mut SimulationConfigBuilder {
        self.output_directory = Some(output_directory);
        self
    }

    pub fn output_file_name(&mut self, output_file_name: String) -> &mut SimulationConfigBuilder {
        self.output_file_name = Some(output_file_name);
        self
    }

    pub fn agent_parameters(
        &mut self,
        agent_parameters: BTreeMap<String, AgentParameters<Single>>,
    ) -> &mut SimulationConfigBuilder {
        self.agent_parameters = Some(agent_parameters);
        self
    }

    pub fn build(&mut self) -> Result<SimulationConfig<Single>, &'static str> {
        if self.simulation.is_none() {
            return Err("Simulation not set");
        }
        // Similar checks for other fields...

        Ok(SimulationConfig {
            simulation: self.simulation.take().unwrap(),
            max_parallel: self.max_parallel.take(),
            output_directory: self.output_directory.take().unwrap(),
            output_file_name: self.output_file_name.take(),
            agent_parameters: self.agent_parameters.take().unwrap(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_in_static() {
        let configs = SimulationConfig::new("src/tests/configs/static.toml").unwrap();
        let configs: Vec<SimulationConfig<Single>> = configs.into();
        let config = configs[0].clone();
        assert_eq!(configs.len(), 1);
        assert_eq!(config.simulation, SimulationType::DynamicWeights);
        assert_eq!(
            config.output_directory,
            "src/tests/output/static".to_string(),
        );
        let agent_parameters = config.agent_parameters.clone();
        assert_eq!(agent_parameters.len(), 5);

        assert!(agent_parameters.get("block_admin").is_some());
        assert!(agent_parameters.get("token_admin").is_some());
        assert!(agent_parameters.get("price_changer").is_some());
        assert!(agent_parameters.get("weight_changer").is_some());
        assert!(agent_parameters.get("lp").is_some())
    }

    #[test]
    fn read_in_sweep() {
        let config = SimulationConfig::new("src/tests/configs/sweep.toml").unwrap();
        let configs: Vec<SimulationConfig<Single>> = config.into();
        assert_eq!(configs.len(), 128);
    }
}
