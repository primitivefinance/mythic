pub mod parameters;
use std::{env, path::Path};

use parameters::*;

use super::*;
use crate::simulations::SimulationType;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SimulationConfig<P: Parameterized<f64>> {
    pub simulation: SimulationType,
    pub max_parallel: Option<usize>,
    pub output_directory: String,
    pub output_file_name: Option<String>,
    pub trajectory: TrajectoryParameters<P>,
    pub gbm: Option<GBMParameters<P>>,
    pub ou: Option<OUParameters<P>>,
    pub pool: PoolParameters,
    pub lp: LPParameters,
    pub block: BlockParameters,
    pub weight_changer: WeightChangerParameters,
}

impl SimulationConfig<Meta> {
    pub fn new(config_path: &str) -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(config::File::with_name(config_path))
            .build()?;
        s.try_deserialize()
    }
}

impl Parameterized<SimulationConfig<Fixed>> for SimulationConfig<Meta> {
    fn generate(&self) -> Vec<SimulationConfig<Fixed>> {
        let mut result = vec![];
        let trajectories = self.trajectory.generate();

        let gbms = self
            .gbm
            .as_ref()
            .map(|gbm| gbm.generate())
            .unwrap_or_default();

        let ous = self.ou.as_ref().map(|ou| ou.generate()).unwrap_or_default();

        if gbms.is_empty() && ous.is_empty() {
            panic!("You must supply either a gbm or an ou configuration.");
        }

        if !gbms.is_empty() && !ous.is_empty() {
            panic!("You can only supply either a gbm or an ou configuration, not both.");
        }

        let mut path = Path::new(env::current_dir().unwrap().to_str().unwrap())
            .join(self.output_directory.as_str());

        for trajectory in &trajectories {
            for gbm in &gbms {
                let output_directory = self.output_directory.clone()
                    + "/gbm_drift="
                    + &gbm.drift.0.to_string()
                    + "_vol="
                    + &gbm.volatility.0.to_string();
                let output_file_name =
                    format!("trajectory={}", trajectory.output_tag.clone().unwrap());
                result.push(SimulationConfig {
                    simulation: self.simulation,
                    max_parallel: None,
                    output_directory,
                    output_file_name: Some(output_file_name),
                    trajectory: trajectory.clone(),
                    gbm: Some(*gbm),
                    ou: None,
                    pool: self.pool,
                    lp: self.lp,
                    block: self.block,
                    weight_changer: self.weight_changer,
                });
            }

            for ou in &ous {
                let output_directory = self.output_directory.clone()
                    + "/ou_mean="
                    + &ou.mean.0.to_string()
                    + "_std="
                    + &ou.std_dev.0.to_string()
                    + "_theta="
                    + &ou.theta.0.to_string();
                let output_file_name =
                    format!("trajectory={}", trajectory.output_tag.clone().unwrap());

                result.push(SimulationConfig {
                    simulation: self.simulation,
                    max_parallel: None,
                    output_directory,
                    output_file_name: Some(output_file_name),
                    trajectory: trajectory.clone(),
                    gbm: None,
                    ou: Some(*ou),
                    pool: self.pool,
                    lp: self.lp,
                    block: self.block,
                    weight_changer: self.weight_changer,
                });
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use parameters::Parameterized;

    use super::*;

    #[test]
    fn read_in_static() {
        let config = SimulationConfig::new("configs/test/static.toml").unwrap();
        let configs = config.generate();
        assert_eq!(configs.len(), 1);
        assert_eq!(configs[0].simulation, SimulationType::DynamicWeights);
        assert_eq!(configs[0].trajectory.process, "gbm");
        assert_eq!(configs[0].trajectory.initial_price, Fixed(1.0));
        assert_eq!(configs[0].trajectory.t_0, Fixed(0.0));
        assert_eq!(configs[0].trajectory.t_n, Fixed(1.0));
        assert_eq!(configs[0].trajectory.num_steps, 100);
        assert_eq!(configs[0].trajectory.seed, 2);
        assert_eq!(configs[0].gbm.unwrap().drift, Fixed(0.1));
        assert_eq!(configs[0].gbm.unwrap().volatility, Fixed(0.35));
        assert_eq!(configs[0].pool.fee_basis_points, 30);
        assert_eq!(configs[0].pool.weight_x, 0.5);
        assert_eq!(configs[0].pool.target_volatility, 0.15);
        assert_eq!(configs[0].lp.x_liquidity, 1.0);
        assert_eq!(configs[0].block.timestep_size, 15);
        assert_eq!(configs[0].weight_changer.target_volatility, 0.15);
        assert_eq!(configs[0].weight_changer.update_frequency, 150);
    }

    #[test]
    fn read_in_sweep() {
        let config = SimulationConfig::new("configs/test/sweep.toml").unwrap();
        let configs = config.generate();
        assert_eq!(configs.len(), 8);
        assert_eq!(configs[0].gbm.unwrap().drift, Fixed(-1.0));
        assert_eq!(configs[1].gbm.unwrap().drift, Fixed(-1.0));
        assert_eq!(configs[2].gbm.unwrap().drift, Fixed(1.0));
        assert_eq!(configs[3].gbm.unwrap().drift, Fixed(1.0));
        assert_eq!(configs[4].gbm.unwrap().drift, Fixed(-1.0));
        assert_eq!(configs[5].gbm.unwrap().drift, Fixed(-1.0));
        assert_eq!(configs[6].gbm.unwrap().drift, Fixed(1.0));
        assert_eq!(configs[7].gbm.unwrap().drift, Fixed(1.0));
        assert_eq!(configs[0].gbm.unwrap().volatility, Fixed(0.0));
        assert_eq!(configs[1].gbm.unwrap().volatility, Fixed(1.0));
        assert_eq!(configs[2].gbm.unwrap().volatility, Fixed(0.0));
        assert_eq!(configs[3].gbm.unwrap().volatility, Fixed(1.0));
        assert_eq!(configs[4].gbm.unwrap().volatility, Fixed(0.0));
        assert_eq!(configs[5].gbm.unwrap().volatility, Fixed(1.0));
        assert_eq!(configs[6].gbm.unwrap().volatility, Fixed(0.0));
        assert_eq!(configs[7].gbm.unwrap().volatility, Fixed(1.0));
    }
}
