pub mod parameters;
use parameters::*;

use crate::simulations::SimulationType;

use super::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SimulationConfig<P: Parameter> {
    pub simulation_type: SimulationType,
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

    pub fn to_direct_configs(self) -> Vec<SimulationConfig<Direct>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    fn read_in_static() {
        todo!()
    }

    fn read_in_sweep() {
        todo!()
    }
}
