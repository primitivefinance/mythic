use parameters::*;

use super::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SimulationConfig {
    pub trajectory: TrajectoryParameters,
    pub gbm: Option<GBMParameters>,
    pub ou: Option<OUParameters>,
    pub pool: PoolParameters,
    pub lp: LPParameters,
    pub block: BlockParameters,
    pub weight_changer: WeightChangerParameters,
}

impl SimulationConfig {
    pub fn new(config_path: &str) -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(config::File::with_name(config_path))
            .build()?;
        s.try_deserialize()
    }
}

pub mod parameters {
    use super::*;

    #[derive(Copy, Clone, Debug, Serialize, Deserialize)]
    pub struct BlockParameters {
        pub timestep_size: u64,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct TrajectoryParameters {
        pub process: String,
        /// The initial price of the asset.
        pub initial_price: f64,
        /// The start time of the process.
        pub t_0: f64,
        /// The end time of the process.
        pub t_n: f64,
        /// The number of steps in the process.
        pub num_steps: usize,
        pub seed: u64,
    }

    #[derive(Copy, Clone, Debug, Serialize, Deserialize)]
    pub struct GBMParameters {
        // The drift of the process.
        pub drift: f64,
        // The volatility of the process.
        pub volatility: f64,
    }

    #[derive(Copy, Clone, Debug, Serialize, Deserialize)]
    pub struct OUParameters {
        /// The mean (price) of the process.
        pub mean: f64,
        /// The standard deviation of the process.
        pub std_dev: f64,
        /// The theta parameter of the process.
        /// This describes how strongly the process will revert to the mean.
        pub theta: f64,
    }

    #[derive(Copy, Clone, Debug, Serialize, Deserialize)]
    pub struct PoolParameters {
        /// fee in bips
        pub fee_basis_points: u16,

        /// Weight for `token_x` in the pool.
        /// Weight for `token_y` will be `1-weight_x`
        pub weight_x: f64,

        /// The target volatility of the pool.
        pub target_volatility: f64,
    }

    #[derive(Copy, Clone, Debug, Serialize, Deserialize)]
    pub struct LPParameters {
        pub liquidity_mantissa: u64,
        pub liquidity_exponent: u32,
    }

    #[derive(Copy, Clone, Debug, Serialize, Deserialize)]
    pub struct WeightChangerParameters {
        pub target_volatility: f64,
        pub update_frequency: u64,
    }
}
