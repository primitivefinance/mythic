use arbiter_core::environment::EnvironmentParameters;
use config::{Config, ConfigError};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct PriceProcessParameters {
    /// The initial price of the asset.
    pub initial_price: f64,
    /// The mean (price) of the process.
    pub mean: f64,
    /// The standard deviation of the process.
    pub std_dev: f64,
    /// The theta parameter of the process.
    /// This describes how strongly the process will revert to the mean.
    pub theta: f64,
    /// The start time of the process.
    pub t_0: f64,
    /// The end time of the process.
    pub t_n: f64,
    /// The number of steps in the process.
    pub num_steps: usize,
    pub seed: Option<u64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TokenParameters {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
}

/// expected settings for g3m pool
/// TODO: adjust settings based on g3m pool configurations
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PortfolioPoolParameters {
    /// fee in bips
    pub fee_basis_points: u16,
    pub weight_token_0: u16,
    pub weight_token_1: u16,

    pub liquidity_mantissa: u64,
    pub liquidity_exponent: u32,

    /// The initial price of the Portfolio pool.
    pub initial_price: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SimulationParameters {
    pub number_of_paths: u16,
    pub sweep_parameters: Option<SweepParameters>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SweepParameters {
    pub sweeps: BTreeMap<String, LinspaceParameters>,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct LinspaceParameters {
    pub start: f64,
    pub end: f64,
    pub steps: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SimulationConfig {
    /// This struct contains some basic settings for the environment such its
    /// label, average number of transactions per block, and a seed for the
    /// transactions per block randomness.
    pub environment_parameters: EnvironmentParameters,

    /// Contains all the necessary data for the Orstein-Uhlenbeck process used
    /// in this simulation.]
    pub price_process_parameters: PriceProcessParameters,

    pub asset_token_parameters: TokenParameters,

    pub quote_token_parameters: TokenParameters,

    pub portfolio_pool_parameters: PortfolioPoolParameters,

    pub simulation_parameters: SimulationParameters,
}

impl SimulationConfig {
    pub fn new() -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(config::File::with_name("Config.toml"))
            .build()?;
        s.try_deserialize()
    }
}
