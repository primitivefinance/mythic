use std::collections::hash_map::DefaultHasher;

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
    pub process: PriceProcess,
}

pub enum PriceProcess {
    GBM(GeometricBrownianMotion),
    OU(OrnsteinUhlenbeck),
}

impl StochasticProcess for PriceProcess {}

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
        let client = RevmMiddleware::new(environment, label.into())?;
        let liquid_exchange = LiquidExchange::deploy(
            client,
            (
                token_admin.arbx.address(),
                token_admin.arby.address(),
                float_to_wad(config.trajectory.initial_price.0),
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

        let trajectory = if let AgentParameters::PriceChanger(parameters) =
            config.agent_parameters.get(label.into())
        {
            let initial_price = parameters.initial_price;
            let t_0 = parameters.t_0;
            let t_n = parameters.t_n;
            let n_steps = parameters.num_steps;
            if let Some(seed) = parameters.seed {
                parameters.process.seedable_euler_maruyama(
                    initial_price,
                    t_0,
                    t_n,
                    n_steps,
                    1,
                    false,
                    seed,
                )
            } else {
                parameters
                    .process
                    .euler_maruyama(initial_price, t_0, t_n, n_steps, 1, false)
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
    }

    /// Update the price of the `LiquidExchange` contract to the next price in
    /// the trajectory and increment the index.
    pub async fn update_price(&mut self) -> Result<()> {
        let price = self.trajectory.paths[0][self.index];
        info!("Updating price of liquid_exchange to: {}", price);
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
        self.update_price().await?;
        Ok(())
    }
}

impl Into<PriceChangerParameters<Single>> for PriceChangerParameters<Multiple> {
    fn into(&self) -> Vec<PriceChangerParameters<Single>> {
        let initial_prices = self.initial_price.into();
        let t_0 = self.t_0.into();
        let t_n = self.t_n.into();
        let mut result = vec![];
        let mut hasher = DefaultHasher::new();

        if let Some(seed) = self.seed {
            for intial_price in initial_prices {
                for t0 in t_0.clone() {
                    for tn in t_n.clone() {
                        result.push(PriceChangerParameters {
                            process: self.process.clone(),
                            initial_price: self.initial_price,
                            t_0,
                            t_n,
                            num_steps: self.num_steps,
                            num_paths: 1,
                            seed: Some(seed),
                        });
                    }
                }
            }
        }

        result
    }
}
