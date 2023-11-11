use self::{
    dollar_cost_averaging::{DollarCostAveragingParameters, DollarCostAveragingStategist},
    momentum::{MomentumParameters, MomentumStrategist},
    volatility_targeting::{VolatilityTargetingParameters, VolatilityTargetingStrategist},
};
use super::*;

pub mod dollar_cost_averaging;
pub mod momentum;
pub mod volatility_targeting;

#[async_trait::async_trait]
pub trait G3mPortfolioManager: Agent {
    async fn execute_smooth_rebalance(&mut self) -> Result<()>;
    fn g3m(&self) -> &G3M<RevmMiddleware>;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct G3mPortfolioManagerParameters<P: Parameterized> {
    pub initial_weight_x: P,
    pub fee: P,
    pub specialty: G3mPortfolioManagerSpecialty<P>,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum G3mPortfolioManagerSpecialty<P: Parameterized> {
    Momentum(MomentumParameters<P>),
    VolatilityTargeting(VolatilityTargetingParameters<P>),
    DollarCostAveraging(DollarCostAveragingParameters),
}

use itertools::iproduct;

impl From<G3mPortfolioManagerParameters<Multiple>> for Vec<G3mPortfolioManagerParameters<Single>> {
    fn from(item: G3mPortfolioManagerParameters<Multiple>) -> Self {
        let specialties: Vec<G3mPortfolioManagerSpecialty<Single>> = item.specialty.into();
        iproduct!(
            item.initial_weight_x.parameters(),
            item.fee.parameters(),
            specialties
        )
        .map(|(iwx, f, specialty)| G3mPortfolioManagerParameters {
            initial_weight_x: Single(iwx),
            fee: Single(f),
            specialty,
        })
        .collect()
    }
}

impl From<G3mPortfolioManagerSpecialty<Multiple>> for Vec<G3mPortfolioManagerSpecialty<Single>> {
    fn from(item: G3mPortfolioManagerSpecialty<Multiple>) -> Self {
        match item {
            G3mPortfolioManagerSpecialty::Momentum(parameters) => {
                let parameters: Vec<MomentumParameters<Single>> = parameters.into();
                parameters
                    .into_iter()
                    .map(G3mPortfolioManagerSpecialty::Momentum)
                    .collect()
            }
            G3mPortfolioManagerSpecialty::VolatilityTargeting(parameters) => {
                let parameters: Vec<VolatilityTargetingParameters<Single>> = parameters.into();
                parameters
                    .into_iter()
                    .map(G3mPortfolioManagerSpecialty::VolatilityTargeting)
                    .collect()
            }
            G3mPortfolioManagerSpecialty::DollarCostAveraging(parameters) => {
                vec![G3mPortfolioManagerSpecialty::DollarCostAveraging(
                    parameters,
                )]
            }
        }
    }
}

#[derive(Debug)]
pub struct G3mPortfolioManagerType(pub Box<dyn G3mPortfolioManager>);

impl std::fmt::Debug for G3mPortfolioManagerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("G3mPortfolioManagerType").finish()
    }
}

impl G3mPortfolioManagerType {
    pub async fn new(
        environment: &Environment,
        config: &SimulationConfig<Single>,
        label: impl Into<String>,
        liquid_exchange_address: Address,
    ) -> Result<Self> {
        let label: String = label.into();
        let client = RevmMiddleware::new(environment, Some(&label))?;
        let lex = LiquidExchange::new(liquid_exchange_address, client.clone());

        if let Some(AgentParameters::G3mPortfolioManager(params)) =
            config.agent_parameters.get(&label)
        {
            let g3m_args = (
                lex.arbiter_token_x().call().await?,
                lex.arbiter_token_y().call().await?,
                parse_ether(params.fee.0)? / 10000,
            );
            let g3m = G3M::deploy(client.clone(), g3m_args)?.send().await?;

            debug!("Deployed G3M at address: {:?}", g3m.address(),);

            match params.specialty {
                G3mPortfolioManagerSpecialty::Momentum(parameters) => {
                    let strategist = MomentumStrategist {
                        client,
                        lex,
                        g3m,
                        update_frequency: parameters.update_frequency.0 as u64,
                        next_update_timestamp: parameters.update_frequency.0 as u64,
                        portfolio_prices: Vec::new(),
                        asset_prices: Vec::new(),
                        portfolio_returns: Vec::new(),
                        asset_returns: Vec::new(),
                    };
                    Ok(Self(Box::new(strategist)))
                }
                G3mPortfolioManagerSpecialty::VolatilityTargeting(parameters) => {
                    let strategist = VolatilityTargetingStrategist {
                        client,
                        lex,
                        g3m,
                        update_frequency: parameters.update_frequency.0 as u64,
                        next_update_timestamp: parameters.update_frequency.0 as u64,
                        target_volatility: parameters.target_volatility.0,
                        sensitivity: parameters.sensitivity.0,
                        max_weight_change: parameters.max_weight_change.0,
                        portfolio_prices: Vec::new(),
                        asset_prices: Vec::new(),
                        portfolio_rv: Vec::new(),
                        asset_rv: Vec::new(),
                    };
                    Ok(Self(Box::new(strategist)))
                }
                G3mPortfolioManagerSpecialty::DollarCostAveraging(parameters) => {
                    let strategist = DollarCostAveragingStategist {
                        client,
                        g3m,
                        end_weight: parameters.end_weight,
                        end_timestamp: parameters.end_timestamp,
                    };
                    Ok(Self(Box::new(strategist)))
                }
            }
        } else {
            Err(anyhow::anyhow!("No parameters found for weight changer"))
        }
    }
}

#[async_trait::async_trait]
impl Agent for G3mPortfolioManagerType {
    fn as_any(&self) -> &dyn Any {
        self
    }

    async fn step(&mut self) -> Result<()> {
        self.0.step().await
    }

    async fn startup(&mut self) -> Result<()> {
        self.0.startup().await
    }
    fn client(&self) -> Arc<RevmMiddleware> {
        self.0.client()
    }
}

#[async_trait::async_trait]
impl G3mPortfolioManager for G3mPortfolioManagerType {
    async fn execute_smooth_rebalance(&mut self) -> Result<()> {
        self.0.execute_smooth_rebalance().await
    }

    fn g3m(&self) -> &G3M<RevmMiddleware> {
        self.0.g3m()
    }
}
