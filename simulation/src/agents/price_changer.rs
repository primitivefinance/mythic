use rand::random;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use arbiter_core::math::GeometricBrownianMotion;

use super::*;

/// The `PriceChanger` holds the data and has methods that allow it to update
/// the price of the `LiquidExchange`.
pub struct PriceChanger {
    /// The path the price process takes.
    pub trajectory: Trajectories,

    /// The `LiquidExchange` contract with the admin `Client`.
    pub liquid_exchange: LiquidExchange<RevmMiddleware>,

    /// The index of the current price in the trajectory.
    pub index: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PriceChangerParameters<P: Parameterized> {
    /// The initial price of the asset.
    pub initial_price: P,
    /// The start time of the process.
    pub t_0: P,
    /// The end time of the process.
    pub t_n: P,
    /// The number of steps in the process.
    pub num_steps: usize,
    pub num_paths: usize,
    pub seed: Option<u64>,
    pub process: PriceProcess<P>,
}

impl PriceChanger {
    /// Create a new `PriceChanger` with the given `LiquidExchange` contract
    /// bound to the admin `Client`. The `PriceChanger` will use the
    /// `OrnsteinUhlenbeck` process to generate a price trajectory with the
    /// constants defined in `config.rs`.
    /// Ornstein-Uhlenbeck processes are useful for modeling the price of stable
    /// tokens.
    pub async fn new(
        environment: &Environment,
        config: &SimulationConfig<Single>,
        label: impl Into<String>,
        token_admin: &token_admin::TokenAdmin,
    ) -> Result<Self> {
        let label: String = label.into();
        let client = RevmMiddleware::new(environment, Some(&label))?;

        if let Some(AgentParameters::PriceChanger(parameters)) = config.agent_parameters.get(&label)
        {
            let liquid_exchange = LiquidExchange::deploy(
                client,
                (
                    token_admin.arbx.address(),
                    token_admin.arby.address(),
                    ethers::utils::parse_ether(parameters.initial_price.0)?,
                ),
            )?
            .send()
            .await?;

            token_admin
                .mint(
                    liquid_exchange.address(),
                    parse_ether(100_000_000_000_u64).unwrap(),
                    parse_ether(100_000_000_000_u64).unwrap(),
                )
                .await?;

            let trajectory = if let Some(seed) = parameters.seed {
                let initial_price = parameters.initial_price;
                let t_0 = parameters.t_0;
                let t_n = parameters.t_n;
                let n_steps = parameters.num_steps;
                if let Some(seed) = parameters.seed {
                    parameters.process.seedable_euler_maruyama(
                        initial_price.0,
                        t_0.0,
                        t_n.0,
                        n_steps,
                        1,
                        false,
                        seed,
                    )
                } else {
                    parameters.process.euler_maruyama(
                        initial_price.0,
                        t_0.0,
                        t_n.0,
                        n_steps,
                        1,
                        false,
                    )
                }
            } else {
                return Err(anyhow::anyhow!("No parameters found for price changer"));
            };

            Ok(Self {
                trajectory,
                liquid_exchange,
                index: 1, /* start after the initial price since it is already set on contract
                           * deployment */
            })
        } else {
            Err(anyhow::anyhow!("No parameters found for price changer"))
        }
    }

    /// Update the price of the `LiquidExchange` contract to the next price in
    /// the trajectory and increment the index.
    pub async fn update_price(&mut self) -> Result<()> {
        let price = self.trajectory.paths[0][self.index];
        trace!("Updating price of liquid_exchange to: {}", price);
        self.liquid_exchange
            .set_price(arbiter_core::math::float_to_wad(price))
            .send()
            .await?
            .await?;
        self.index += 1;
        Ok(())
    }
}

#[async_trait::async_trait]
impl Agent for PriceChanger {
    async fn step(&mut self) -> Result<()> {
        debug!("Updating price on lex");
        self.update_price().await?;
        debug!("Price updated on lex");
        Ok(())
    }
}

impl From<PriceChangerParameters<Multiple>> for Vec<PriceChangerParameters<Single>> {
    fn from(item: PriceChangerParameters<Multiple>) -> Self {
        let num_paths = item.num_paths;
        let initial_prices = item.initial_price.parameters();
        let t_0 = item.t_0.parameters();
        let t_n = item.t_n.parameters();
        let process: Vec<PriceProcess<Single>> = item.process.into();
        let mut result: Vec<PriceChangerParameters<Single>> = vec![];

        let mut hasher = DefaultHasher::new();
        let mut seed = match item.seed {
            Some(val) => val,
            None => rand::random::<u64>(),
        };
        for _ in 0..num_paths {
            for initial_price in initial_prices.clone() {
                for t0 in t_0.clone() {
                    for tn in t_n.clone() {
                        for process in process.clone() {
                            result.push(PriceChangerParameters {
                                process,
                                initial_price: Single(initial_price),
                                t_0: Single(t0),
                                t_n: Single(tn),
                                num_steps: item.num_steps,
                                num_paths: 1,
                                seed: Some(seed),
                            });
                            hasher.write_u64(seed);
                            seed = hasher.finish();
                        }
                    }
                }
            }
        }

        result
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum PriceProcess<P: Parameterized> {
    GBM(GBMParameters<P>),
    OU(OUParameters<P>),
}

impl StochasticProcess for PriceProcess<Single> {
    fn drift(&self, x: f64, t: f64) -> f64 {
        match self {
            PriceProcess::GBM(parameters) => {
                GeometricBrownianMotion::new(parameters.drift.0, parameters.volatility.0)
                    .drift(x, t)
            }
            PriceProcess::OU(parameters) => OrnsteinUhlenbeck::new(
                parameters.theta.0,
                parameters.mean.0,
                parameters.volatility.0,
            )
            .drift(x, t),
        }
    }

    fn diffusion(&self, x: f64, t: f64) -> f64 {
        match self {
            PriceProcess::GBM(parameters) => {
                GeometricBrownianMotion::new(parameters.drift.0, parameters.volatility.0)
                    .diffusion(x, t)
            }
            PriceProcess::OU(parameters) => OrnsteinUhlenbeck::new(
                parameters.theta.0,
                parameters.mean.0,
                parameters.volatility.0,
            )
            .diffusion(x, t),
        }
    }

    fn jump(&self, x: f64, t: f64) -> Option<f64> {
        match self {
            PriceProcess::GBM(parameters) => {
                GeometricBrownianMotion::new(parameters.drift.0, parameters.volatility.0).jump(x, t)
            }
            PriceProcess::OU(parameters) => OrnsteinUhlenbeck::new(
                parameters.theta.0,
                parameters.mean.0,
                parameters.volatility.0,
            )
            .jump(x, t),
        }
    }
}

impl Into<Vec<PriceProcess<Single>>> for PriceProcess<Multiple> {
    fn into(self) -> Vec<PriceProcess<Single>> {
        match self {
            PriceProcess::GBM(parameters) => {
                let parameters: Vec<GBMParameters<Single>> = parameters.into();
                parameters.into_iter().map(PriceProcess::GBM).collect()
            }
            PriceProcess::OU(parameters) => {
                let parameters: Vec<OUParameters<Single>> = parameters.into();
                parameters.into_iter().map(PriceProcess::OU).collect()
            }
        }
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct GBMParameters<P: Parameterized> {
    pub drift: P,
    pub volatility: P,
}

impl Into<Vec<GBMParameters<Single>>> for GBMParameters<Multiple> {
    fn into(self) -> Vec<GBMParameters<Single>> {
        let drift = self.drift.parameters();
        let volatility = self.volatility.parameters();
        let mut result = vec![];
        for d in drift {
            for v in volatility.clone() {
                result.push(GBMParameters {
                    drift: Single(d),
                    volatility: Single(v),
                });
            }
        }
        result
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct OUParameters<P: Parameterized> {
    pub theta: P,
    pub mean: P,
    pub volatility: P,
}

impl Into<Vec<OUParameters<Single>>> for OUParameters<Multiple> {
    fn into(self) -> Vec<OUParameters<Single>> {
        let theta = self.theta.parameters();
        let mean = self.mean.parameters();
        let volatility = self.volatility.parameters();
        let mut result = vec![];
        for t in theta {
            for m in mean.clone() {
                for v in volatility.clone() {
                    result.push(OUParameters {
                        theta: Single(t),
                        mean: Single(m),
                        volatility: Single(v),
                    });
                }
            }
        }
        result
    }
}
