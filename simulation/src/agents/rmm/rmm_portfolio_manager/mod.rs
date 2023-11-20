use self::rmm_volatility_targeting::{
    RmmVolatilityTargetingParameters, RmmVolatilityTargetingStrategist,
};
use super::*;

pub mod rmm_volatility_targeting;

#[async_trait::async_trait]
pub trait RmmPortfolioManager: Agent {
    async fn execute_smooth_rebalance(&mut self) -> Result<()>;
    fn rmm(&self) -> &RMM<RevmMiddleware>;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RmmPortfolioManagerParameters<P: Parameterized> {
    sigma: P,
    tau: P,
    strike_price: P,
    pub fee: P,
    pub specialty: RmmPortfolioManagerSpecialty<P>,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum RmmPortfolioManagerSpecialty<P: Parameterized> {
    VolatilityTargeting(RmmVolatilityTargetingParameters<P>),
}

use itertools::iproduct;

impl From<RmmPortfolioManagerParameters<Multiple>> for Vec<RmmPortfolioManagerParameters<Single>> {
    fn from(item: RmmPortfolioManagerParameters<Multiple>) -> Self {
        let specialties: Vec<RmmPortfolioManagerSpecialty<Single>> = item.specialty.into();
        iproduct!(
            item.sigma.parameters(),
            item.tau.parameters(),
            item.strike_price.parameters(),
            item.fee.parameters(),
            specialties
        )
        .map(|(s, tau, sp, f, specialty)| RmmPortfolioManagerParameters {
            sigma: Single(s),
            tau: Single(tau),
            strike_price: Single(sp),
            fee: Single(f),
            specialty,
        })
        .collect()
    }
}

impl From<RmmPortfolioManagerSpecialty<Multiple>> for Vec<RmmPortfolioManagerSpecialty<Single>> {
    fn from(item: RmmPortfolioManagerSpecialty<Multiple>) -> Self {
        match item {
            RmmPortfolioManagerSpecialty::VolatilityTargeting(parameters) => {
                let parameters: Vec<RmmVolatilityTargetingParameters<Single>> = parameters.into();
                parameters
                    .into_iter()
                    .map(RmmPortfolioManagerSpecialty::VolatilityTargeting)
                    .collect()
            }
        }
    }
}
#[derive(Debug)]
pub struct RmmPortfolioManagerType(pub Box<dyn RmmPortfolioManager>);

impl RmmPortfolioManagerType {
    pub async fn new(
        environment: &Environment,
        config: &SimulationConfig<Single>,
        label: impl Into<String>,
        liquid_exchange_address: Address,
    ) -> Result<Self> {
        let label: String = label.into();
        let client = RevmMiddleware::new(environment, Some(&label))?;
        let lex = LiquidExchange::new(liquid_exchange_address, client.clone());

        if let Some(AgentParameters::RmmPortfolioManager(params)) =
            config.agent_parameters.get(&label)
        {
            let rmm_args = (
                lex.arbiter_token_x().call().await?,
                lex.arbiter_token_y().call().await?,
                parse_ether(2.5)?,
                parse_ether(1.0)?,
                parse_ether(1.0)?,
                parse_ether(0.997)?,
            );
            match params.specialty {
                RmmPortfolioManagerSpecialty::VolatilityTargeting(parameters) => {
                    let rmm = RMM::deploy(client.clone(), rmm_args)?.send().await?;
                    trace!("Deployed rmm at address: {:?}", rmm.address());
                    let strategist = RmmVolatilityTargetingStrategist {
                        client,
                        lex,
                        rmm,
                        update_frequency: parameters.update_frequency.0 as u64,
                        next_update_timestamp: parameters.update_frequency.0 as u64,
                        target_volatility: parameters.target_volatility.0,
                        portfolio_prices: Vec::new(),
                        asset_prices: Vec::new(),
                        portfolio_rv: Vec::new(),
                        asset_rv: Vec::new(),
                    };
                    Ok(Self(Box::new(strategist)))
                }
            }
        } else {
            Err(anyhow::anyhow!(
                "No parameters found for volatility targeting portfolio manager"
            ))
        }
    }
}

#[async_trait::async_trait]
impl Agent for RmmPortfolioManagerType {
    async fn step(&mut self) -> Result<()> {
        self.0.step().await
    }

    async fn startup(&mut self) -> Result<()> {
        self.0.startup().await
    }

    fn client(&self) -> Arc<RevmMiddleware> {
        self.0.client()
    }

    fn as_any(&self) -> &dyn Any {
        self.0.as_any()
    }
}

#[async_trait::async_trait]
impl RmmPortfolioManager for RmmPortfolioManagerType {
    async fn execute_smooth_rebalance(&mut self) -> Result<()> {
        self.0.execute_smooth_rebalance().await
    }

    fn rmm(&self) -> &RMM<RevmMiddleware> {
        self.0.rmm()
    }
}
