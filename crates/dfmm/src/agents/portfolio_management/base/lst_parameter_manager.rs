use std::sync::Arc;

use arbiter_bindings::bindings::liquid_exchange::LiquidExchange;
use arbiter_core::errors::ArbiterCoreError;
use clients::protocol::ProtocolClient;
use ethers::types::Address;
use itertools::iproduct;
use settings::parameters::LinspaceParameters;

use super::{agent::*, *};

#[derive(Debug, Clone)]
pub struct PositionData {
    pub portfolio_prices: Vec<(f64, u64)>,
    pub asset_prices: Vec<(f64, u64)>,
    pub portfolio_rv: Vec<(f64, u64)>,
    pub asset_rv: Vec<(f64, u64)>,
}

impl PositionData {
    pub fn new() -> Result<Self> {
        Ok(Self {
            portfolio_prices: Vec::new(),
            asset_prices: Vec::new(),
            portfolio_rv: Vec::new(),
            asset_rv: Vec::new(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct LstParameterManager {
    pub client: Arc<ArbiterMiddleware>,
    pub lex: LiquidExchange<ArbiterMiddleware>,
    pub protocol_client: ProtocolClient<ArbiterMiddleware>,
    pub pool_id: U256,
    pub strike_values: Vec<f64>,
    pub timestep_size: u64,
    pub index: u64,
}

#[async_trait::async_trait]
impl Agent for LstParameterManager {
    async fn step(&mut self) -> Result<(), ArbiterCoreError> {
        let time = self.client.get_block_timestamp().await?.as_u64();
        self.index += 1;
        let next_strike = self.strike_values[self.index as usize];
        self.protocol_client
            .set_strike_price(self.pool_id, next_strike, time + self.timestep_size)
            .await
            .unwrap();

        Ok(())
    }

    fn client(&self) -> Arc<ArbiterMiddleware> {
        self.client.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl LstParameterManager {
    pub async fn new(
        environment: &Environment,
        config: &SimulationConfig<Single>,
        protocol_client: ProtocolClient<ArbiterMiddleware>,
        label: impl Into<String>,
        liquid_exchange_address: Address,
        pool_id: U256,
    ) -> Result<Self> {
        let label: String = label.into();
        let client = ArbiterMiddleware::new(environment, Some(&label))?;
        let protocol_client = protocol_client.connect(client.clone())?;
        let lex = LiquidExchange::new(liquid_exchange_address, client.clone());

        if let Some(AgentParameters::LstParameterManager(params)) =
            config.agent_parameters.get(&label).cloned()
        {
            let times = linspace!(params.t0.0, params.tn.0, params.timestep_size.0 as usize);
            let strike_values = times
                .iter()
                .map(|t| {
                    let i = 1.0 + params.rate.0;
                    i.powf(*t)
                })
                .collect();
            Ok(Self {
                client,
                lex,
                protocol_client,
                strike_values,
                timestep_size: params.timestep_size.0 as u64,
                pool_id,
                index: 0,
            })
        } else {
            Err(anyhow::anyhow!(
                "No parameters found for `LstParameterManager` with label: {}",
                label
            ))
        }
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct LstParameterManagerParams<P: Parameterized> {
    pub rate: P,
    pub timestep_size: P,
    pub t0: P,
    pub tn: P,
}

impl From<LstParameterManagerParams<Multiple>> for Vec<LstParameterManagerParams<Single>> {
    fn from(params: LstParameterManagerParams<Multiple>) -> Self {
        iproduct!(
            params.rate.parameters(),
            params.timestep_size.parameters(),
            params.t0.parameters(),
            params.tn.parameters()
        )
        .map(|(r, ts, t0, tn)| LstParameterManagerParams {
            rate: Single(r),
            timestep_size: Single(ts),
            t0: Single(t0),
            tn: Single(tn),
        })
        .collect()
    }
}
