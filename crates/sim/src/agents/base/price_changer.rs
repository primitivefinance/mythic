use std::{collections::hash_map::DefaultHasher, hash::Hasher, sync::Arc};

use bindings::lex::Lex;
use ethers::utils::parse_ether;
use itertools::iproduct;
use RustQuant::stochastics::{
    GeometricBrownianMotion, OrnsteinUhlenbeck, StochasticProcess, Trajectories,
};

use super::{agent::*, *};

/// The `PriceChanger` holds the data and has methods that allow it to update
/// the price of the `LiquidExchange`.
pub struct PriceChanger {
    /// The client for the `PriceChanger`
    pub client: Arc<ArbiterMiddleware>,
    /// The path the price process takes.
    pub trajectory: Trajectories,

    /// The `LiquidExchange` contract with the admin `Client`.
    pub liquid_exchange: Lex<ArbiterMiddleware>,

    /// The index of the current price in the trajectory.
    pub index: usize,
}

impl Clone for PriceChanger {
    fn clone(&self) -> Self {
        let trajectory = Trajectories {
            times: self.trajectory.times.clone(),
            paths: self.trajectory.paths.clone(),
        };
        Self {
            client: self.client.clone(),
            trajectory,
            liquid_exchange: self.liquid_exchange.clone(),
            index: self.index,
        }
    }
}

impl std::fmt::Debug for PriceChanger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PriceChanger")
            .field("liquid_exchange", &self.liquid_exchange)
            .field("index", &self.index)
            .finish()
    }
}

#[async_trait::async_trait]
impl Agent for PriceChanger {
    fn as_any(&self) -> &dyn Any {
        self
    }

    async fn step(&mut self) -> Result<()> {
        let current_price = self.liquid_exchange.price().call().await?;
        debug!("Updating lex price from {:?}", current_price);
        self.update_price().await?;
        debug!(
            "Updated lex price to {:?}",
            self.liquid_exchange.price().call().await?
        );
        Ok(())
    }
    fn client(&self) -> Arc<ArbiterMiddleware> {
        self.client.clone()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PriceChangerParameters<P: Parameterized> {
    /// To backtest (true) or not (false)
    pub backtest: bool,
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
        let client = ArbiterMiddleware::new(environment, Some(&label))?;
        if let Some(AgentParameters::PriceChanger(parameters)) = config.agent_parameters.get(&label)
        {
            let liquid_exchange = Lex::deploy(
                client.clone(),
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
                    parse_ether(1_000_000)?,
                    parse_ether(1_000_000)?,
                )
                .await?;
            if parameters.backtest {
                debug!("Backtesting price changer");
                // TODO Cache this somewhere
                let mut prices = get_historical_daily_prices(parameters.num_steps).await?;
                prices.reverse();
                let mut times = vec![];
                let mut time = 0.0;
                for _ in &prices {
                    time += 1.0;
                    times.push(time);
                }
                let paths = vec![prices.clone()];
                let trajectory = Trajectories { times, paths };
                Ok(Self {
                    client,
                    trajectory,
                    liquid_exchange,
                    index: 1,
                })
            } else {
                let trajectory = if let Some(_seed) = parameters.seed {
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
                    client,
                    trajectory,
                    liquid_exchange,
                    index: 1, /* start after the initial price since it is already set on
                               * contract deployment */
                })
            }
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
            .set_price(ethers::utils::parse_ether(price)?)
            .send()
            .await?
            .await?;
        self.index += 1;
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
                                backtest: item.backtest,
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
    #[serde(rename = "GBM")]
    Gbm(GBMParameters<P>),
    #[serde(rename = "OU")]
    Ou(OUParameters<P>),
}

impl StochasticProcess for PriceProcess<Single> {
    fn drift(&self, x: f64, t: f64) -> f64 {
        match self {
            PriceProcess::Gbm(parameters) => {
                GeometricBrownianMotion::new(parameters.drift.0, parameters.volatility.0)
                    .drift(x, t)
            }
            PriceProcess::Ou(parameters) => OrnsteinUhlenbeck::new(
                parameters.mean.0,
                parameters.volatility.0,
                parameters.theta.0,
            )
            .drift(x, t),
        }
    }

    fn diffusion(&self, x: f64, t: f64) -> f64 {
        match self {
            PriceProcess::Gbm(parameters) => {
                GeometricBrownianMotion::new(parameters.drift.0, parameters.volatility.0)
                    .diffusion(x, t)
            }
            PriceProcess::Ou(parameters) => OrnsteinUhlenbeck::new(
                parameters.mean.0,
                parameters.volatility.0,
                parameters.theta.0,
            )
            .diffusion(x, t),
        }
    }

    fn jump(&self, x: f64, t: f64) -> Option<f64> {
        match self {
            PriceProcess::Gbm(parameters) => {
                GeometricBrownianMotion::new(parameters.drift.0, parameters.volatility.0).jump(x, t)
            }
            PriceProcess::Ou(parameters) => OrnsteinUhlenbeck::new(
                parameters.mean.0,
                parameters.volatility.0,
                parameters.theta.0,
            )
            .jump(x, t),
        }
    }
}

impl From<PriceProcess<Multiple>> for Vec<PriceProcess<Single>> {
    fn from(item: PriceProcess<Multiple>) -> Self {
        match item {
            PriceProcess::Gbm(parameters) => {
                let parameters: Vec<GBMParameters<Single>> = parameters.into();
                parameters.into_iter().map(PriceProcess::Gbm).collect()
            }
            PriceProcess::Ou(parameters) => {
                let parameters: Vec<OUParameters<Single>> = parameters.into();
                parameters.into_iter().map(PriceProcess::Ou).collect()
            }
        }
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct GBMParameters<P: Parameterized> {
    pub drift: P,
    pub volatility: P,
}

impl From<GBMParameters<Multiple>> for Vec<GBMParameters<Single>> {
    fn from(item: GBMParameters<Multiple>) -> Self {
        let drift = item.drift.parameters();
        let volatility = item.volatility.parameters();

        iproduct!(drift, volatility.clone())
            .map(|(d, v)| GBMParameters {
                drift: Single(d),
                volatility: Single(v),
            })
            .collect()
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct OUParameters<P: Parameterized> {
    pub theta: P,
    pub mean: P,
    pub volatility: P,
}

impl From<OUParameters<Multiple>> for Vec<OUParameters<Single>> {
    fn from(item: OUParameters<Multiple>) -> Self {
        let theta = item.theta.parameters();
        let mean = item.mean.parameters();
        let volatility = item.volatility.parameters();

        iproduct!(theta, mean.clone(), volatility.clone())
            .map(|(t, m, v)| OUParameters {
                theta: Single(t),
                mean: Single(m),
                volatility: Single(v),
            })
            .collect()
    }
}
