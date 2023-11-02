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

impl Into<Vec<SimulationConfig<Single>>> for SimulationConfig<Multiple> {
    fn into(self) -> Vec<SimulationConfig<Single>> {
        let mut path = Path::new(env::current_dir().unwrap().to_str().unwrap())
            .join(self.output_directory.as_str());

        let mut agent_parameters = BTreeMap::new();
        for (key, value) in self.agent_parameters {
            let mut output_directory = path.clone();
            output_directory.push(key.as_str());
            agent_parameters.insert(key, value.into());
        }

        SimulationConfig::<Single> {
            simulation: self.simulation,
            max_parallel: None,
            output_directory: path.to_str().unwrap().to_string(),
            agent_parameters,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn read_in_static() {
        let config = SimulationConfig::new("configs/test/static.toml").unwrap();
        let configs = config.generate();
        assert_eq!(configs.len(), 1);
        assert_eq!(configs[0].simulation, SimulationType::DynamicWeights);
        assert_eq!(configs[0].trajectory.process, "gbm");
        assert_eq!(configs[0].trajectory.initial_price, 1.0);
        assert_eq!(configs[0].trajectory.t_0, 0.0);
        assert_eq!(configs[0].trajectory.t_n, 1.0);
        assert_eq!(configs[0].trajectory.num_steps, 100);
        assert_eq!(configs[0].trajectory.seed, 2);
        assert_eq!(configs[0].gbm.unwrap().drift, 0.1);
        assert_eq!(configs[0].gbm.unwrap().volatility, 0.3);
        assert_eq!(configs[0].pool.fee_basis_points, 30);
        assert_eq!(configs[0].pool.weight_x, 0.5);
        assert_eq!(configs[0].pool.target_volatility, 0.15);
        assert_eq!(configs[0].lp.x_liquidity, 1.0);
        assert_eq!(configs[0].block.timestep_size, 15);
        assert_eq!(
            configs[0]
                .weight_changer
                .volatility_targeting
                .unwrap()
                .target_volatility,
            0.15
        );
        assert_eq!(
            configs[0]
                .weight_changer
                .volatility_targeting
                .unwrap()
                .update_frequency,
            150
        );
    }

    #[test]
    fn read_in_sweep() {
        let config = SimulationConfig::new("configs/test/sweep.toml").unwrap();
        let configs = config.generate();
        assert_eq!(configs.len(), 8);
        assert_eq!(configs[0].gbm.unwrap().drift, -1.0);
        assert_eq!(configs[1].gbm.unwrap().drift, -1.0);
        assert_eq!(configs[2].gbm.unwrap().drift, 1.0);
        assert_eq!(configs[3].gbm.unwrap().drift, 1.0);
        assert_eq!(configs[4].gbm.unwrap().drift, -1.0);
        assert_eq!(configs[5].gbm.unwrap().drift, -1.0);
        assert_eq!(configs[6].gbm.unwrap().drift, 1.0);
        assert_eq!(configs[7].gbm.unwrap().drift, 1.0);
        assert_eq!(configs[0].gbm.unwrap().volatility, 0.0);
        assert_eq!(configs[1].gbm.unwrap().volatility, 1.0);
        assert_eq!(configs[2].gbm.unwrap().volatility, 0.0);
        assert_eq!(configs[3].gbm.unwrap().volatility, 1.0);
        assert_eq!(configs[4].gbm.unwrap().volatility, 0.0);
        assert_eq!(configs[5].gbm.unwrap().volatility, 1.0);
        assert_eq!(configs[6].gbm.unwrap().volatility, 0.0);
        assert_eq!(configs[7].gbm.unwrap().volatility, 1.0);
    }
}
