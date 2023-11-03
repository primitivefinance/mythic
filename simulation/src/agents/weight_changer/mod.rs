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
pub trait WeightChanger: Agent {
    async fn execute_smooth_rebalance(&mut self) -> Result<()>;
    fn g3m(&self) -> &G3M<RevmMiddleware>;
    fn lex(&self) -> &LiquidExchange<RevmMiddleware>;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WeightChangerParameters<P: Parameterized> {
    pub initial_weight_x: P,
    pub fee: P,
    pub specialty: WeightChangerSpecialty<P>,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum WeightChangerSpecialty<P: Parameterized> {
    Momentum(MomentumParameters<P>),
    VolatilityTargeting(VolatilityTargetingParameters<P>),
    DollarCostAveraging(DollarCostAveragingParameters<P>),
}

use itertools::iproduct;

impl From<WeightChangerParameters<Multiple>> for Vec<WeightChangerParameters<Single>> {
    fn from(item: WeightChangerParameters<Multiple>) -> Self {
        let specialties: Vec<WeightChangerSpecialty<Single>> = item.specialty.into();
        iproduct!(
            item.initial_weight_x.parameters(),
            item.fee.parameters(),
            specialties
        )
        .map(|(iwx, f, specialty)| WeightChangerParameters {
            initial_weight_x: Single(iwx),
            fee: Single(f),
            specialty,
        })
        .collect()
    }
}

impl From<WeightChangerSpecialty<Multiple>> for Vec<WeightChangerSpecialty<Single>> {
    fn from(item: WeightChangerSpecialty<Multiple>) -> Self {
        match item {
            WeightChangerSpecialty::Momentum(parameters) => {
                let parameters: Vec<MomentumParameters<Single>> = parameters.into();
                parameters
                    .into_iter()
                    .map(WeightChangerSpecialty::Momentum)
                    .collect()
            }
            WeightChangerSpecialty::VolatilityTargeting(parameters) => {
                let parameters: Vec<VolatilityTargetingParameters<Single>> = parameters.into();
                parameters
                    .into_iter()
                    .map(WeightChangerSpecialty::VolatilityTargeting)
                    .collect()
            }
            WeightChangerSpecialty::DollarCostAveraging(parameters) => {
                let parameters: Vec<DollarCostAveragingParameters<Single>> = parameters.into();
                parameters
                    .into_iter()
                    .map(WeightChangerSpecialty::DollarCostAveraging)
                    .collect()
            }
        }
    }
}

pub struct WeightChangerType(pub Box<dyn WeightChanger>);

impl WeightChangerType {
    pub async fn new(
        environment: &Environment,
        config: &SimulationConfig<Single>,
        label: impl Into<String>,
        liquid_exchange_address: Address,
    ) -> Result<Self> {
        let label: String = label.into();
        let client = RevmMiddleware::new(environment, Some(&label))?;
        let lex = LiquidExchange::new(liquid_exchange_address, client.clone());

        if let Some(AgentParameters::WeightChanger(params)) = config.agent_parameters.get(&label) {
            let g3m_args = (
                lex.arbiter_token_x().call().await?,
                lex.arbiter_token_y().call().await?,
                ethers::utils::parse_ether(params.initial_weight_x.0)?,
                ethers::utils::parse_ether(params.fee.0)? / 10000,
            );
            let g3m = G3M::deploy(client.clone(), g3m_args)?.send().await?;

            debug!("Deployed G3M at address: {:?}", g3m.address(),);

            match params.specialty {
                WeightChangerSpecialty::Momentum(parameters) => {
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
                WeightChangerSpecialty::VolatilityTargeting(parameters) => {
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
                WeightChangerSpecialty::DollarCostAveraging(parameters) => {
                    todo!()
                }
            }
        } else {
            Err(anyhow::anyhow!("No parameters found for weight changer"))
        }
    }
}

#[async_trait::async_trait]
impl Agent for WeightChangerType {
    async fn step(&mut self) -> Result<()> {
        self.0.step().await
    }

    async fn startup(&mut self) -> Result<()> {
        self.0.startup().await
    }
}

#[async_trait::async_trait]
impl WeightChanger for WeightChangerType {
    async fn execute_smooth_rebalance(&mut self) -> Result<()> {
        self.0.execute_smooth_rebalance().await
    }

    fn g3m(&self) -> &G3M<RevmMiddleware> {
        self.0.g3m()
    }

    fn lex(&self) -> &LiquidExchange<RevmMiddleware> {
        self.0.lex()
    }
}
