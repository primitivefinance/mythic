use ethers::abi::Param;

use super::*;
pub trait Parameter {
    fn generate(&self) -> Vec<f64>;
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Direct(pub f64);
impl Parameter for Direct {
    fn generate(&self) -> Vec<f64> {
        vec![self.0]
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Meta(LinspaceParameters);
impl Parameter for Meta {
    fn generate(&self) -> Vec<f64> {
        self.0.generate()
    }
}

pub trait GenerateDirect<T> {
    fn generate_direct(&self) -> Vec<T>;
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct LinspaceParameters {
    pub start: f64,
    pub end: f64,
    pub steps: usize,
}

impl LinspaceParameters {
    fn generate(&self) -> Vec<f64> {
        let step_size = (self.end - self.start) / (self.steps as f64 - 1.0);
        (0..self.steps)
            .map(|i| self.start + step_size * i as f64)
            .collect()
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct BlockParameters {
    pub timestep_size: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TrajectoryParameters<P: Parameter> {
    pub process: String,
    /// The initial price of the asset.
    pub initial_price: P,
    /// The start time of the process.
    pub t_0: P,
    /// The end time of the process.
    pub t_n: P,
    /// The number of steps in the process.
    pub num_steps: usize,
    pub seed: u64,
}

impl GenerateDirect<TrajectoryParameters<Direct>> for TrajectoryParameters<Meta> {
    fn generate_direct(&self) -> Vec<TrajectoryParameters<Direct>> {
        let initial_price = self.initial_price.generate();
        let t_0 = self.t_0.generate();
        let t_n = self.t_n.generate();
        let mut result = vec![];
        for p in initial_price {
            for t0 in t_0.clone() {
                for tn in t_n.clone() {
                    result.push(TrajectoryParameters {
                        process: self.process.clone(),
                        initial_price: Direct(p),
                        t_0: Direct(t0),
                        t_n: Direct(tn),
                        num_steps: self.num_steps,
                        seed: self.seed,
                    });
                }
            }
        }
        result
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct GBMParameters<P: Parameter> {
    // The drift of the process.
    pub drift: P,
    // The volatility of the process.
    pub volatility: P,
}

impl GenerateDirect<GBMParameters<Direct>> for GBMParameters<Meta> {
    fn generate_direct(&self) -> Vec<GBMParameters<Direct>> {
        let drift = self.drift.generate();
        let volatility = self.volatility.generate();
        let mut result = vec![];
        for d in drift {
            for v in volatility.clone() {
                result.push(GBMParameters {
                    drift: Direct(d),
                    volatility: Direct(v),
                });
            }
        }
        result
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct OUParameters<P: Parameter> {
    /// The mean (price) of the process.
    pub mean: P,
    /// The standard deviation of the process.
    pub std_dev: P,
    /// The theta parameter of the process.
    /// This describes how strongly the process will revert to the mean.
    pub theta: P,
}

impl GenerateDirect<OUParameters<Direct>> for OUParameters<Meta> {
    fn generate_direct(&self) -> Vec<OUParameters<Direct>> {
        let mean = self.mean.generate();
        let std_dev = self.std_dev.generate();
        let theta = self.theta.generate();
        let mut result = vec![];
        for m in mean {
            for s in std_dev.clone() {
                for t in theta.clone() {
                    result.push(OUParameters {
                        mean: Direct(m),
                        std_dev: Direct(s),
                        theta: Direct(t),
                    });
                }
            }
        }
        result
    }
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
    pub x_liquidity: f64,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct WeightChangerParameters {
    pub target_volatility: f64,
    pub update_frequency: u64,
}
