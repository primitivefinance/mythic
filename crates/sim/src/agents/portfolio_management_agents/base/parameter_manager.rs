use std::sync::Arc;

use arbiter_bindings::bindings::liquid_exchange::LiquidExchange;
use clients::protocol::{PoolParams, ProtocolClient};
use ethers::types::Address;
use itertools::iproduct;

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
pub struct ParameterManager {
    pub client: Arc<RevmMiddleware>,
    pub lex: LiquidExchange<RevmMiddleware>,
    pub protocol_client: ProtocolClient<RevmMiddleware>,
    pub next_update_time: u64,
    pub update_frequency: u64,
    pub target_volatility: f64,
    pub max_change: f64,
    pub sensitivity: f64,
    pub data: PositionData,
    pub pool_id: U256,
}

#[async_trait::async_trait]
impl Agent for ParameterManager {
    async fn step(&mut self) -> Result<()> {
        let time = self.client.get_block_timestamp().await?.as_u64();
        let asset_price = self.get_asset_price().await?;
        let portfolio_price = self.get_portfolio_price().await?;
        if time >= self.next_update_time {
            self.next_update_time = time + self.update_frequency;
            self.update_position_data(portfolio_price, asset_price, time);
            self.calculate_rv()?;
            self.execute_smooth_rebalance().await?;
        }
        Ok(())
    }

    fn client(&self) -> Arc<RevmMiddleware> {
        self.client.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl ParameterManager {
    pub async fn new(
        environment: &Environment,
        config: &SimulationConfig<Single>,
        protocol_client: ProtocolClient<RevmMiddleware>,
        label: impl Into<String>,
        liquid_exchange_address: Address,
        pool_id: U256,
    ) -> Result<Self> {
        let label: String = label.into();
        let client = RevmMiddleware::new(environment, Some(&label))?;
        let protocol_client = protocol_client.bind(client.clone())?;
        let lex = LiquidExchange::new(liquid_exchange_address, client.clone());

        if let Some(AgentParameters::ParameterManager(params)) = config.agent_parameters.get(&label)
        {
            match params.specialty {
                Specialty::VolatilityTargeting(parameters) => Ok(Self {
                    client,
                    lex,
                    protocol_client,
                    update_frequency: parameters.update_frequency.0 as u64,
                    next_update_time: parameters.update_frequency.0 as u64,
                    target_volatility: parameters.target_volatility.0,
                    sensitivity: parameters.sensitivity.0,
                    max_change: parameters.max_change.0,
                    data: PositionData::new()?,
                    pool_id,
                }),
            }
        } else {
            Err(anyhow::anyhow!(
                "No parameters found for volatility targeting portfolio manager"
            ))
        }
    }

    async fn get_asset_price(&self) -> Result<f64> {
        let price = self.lex.price().call().await?;
        parse_ether_to_f64(price)
    }

    async fn get_portfolio_price(&self) -> Result<f64> {
        let (rx, ry, _liq) = self
            .protocol_client
            .get_reserves_and_liquidity(self.pool_id)
            .await?;
        let rx = parse_ether_to_f64(rx)?;
        let ry = parse_ether_to_f64(ry)?;
        Ok(rx * self.get_asset_price().await? + ry)
    }

    fn update_position_data(&mut self, portfolio_price: f64, asset_price: f64, timestamp: u64) {
        if self.data.portfolio_prices.is_empty() {
            self.data.portfolio_prices.push((portfolio_price, 0));
            self.data.asset_prices.push((asset_price, 0));
        }
        self.data.asset_prices.push((asset_price, timestamp));
        self.data
            .portfolio_prices
            .push((portfolio_price, timestamp));
    }

    async fn execute_smooth_rebalance(&mut self) -> Result<()> {
        if self.data.portfolio_rv.len() < 2 {
            return Ok(());
        }
        let portfolio_rv = self.data.portfolio_rv.last().unwrap().0;
        let pool_params = self.protocol_client.get_params(self.pool_id).await?;
        let vol_diff = (portfolio_rv - self.target_volatility).abs();
        match pool_params {
            PoolParams::G3M(g3m_params) => {
                let wx = parse_ether_to_f64(g3m_params.w_x)?;
                let mut new_wx;
                if portfolio_rv < self.target_volatility {
                    new_wx = wx + 0.0025;
                    if new_wx >= 0.99 {
                        new_wx = 0.99;
                    }
                    self.protocol_client
                        .set_weight_x(self.pool_id, new_wx, self.next_update_time)
                        .await?;
                } else {
                    new_wx = wx - 0.0025;
                    if new_wx <= 0.01 {
                        new_wx = 0.01;
                    }
                    self.protocol_client
                        .set_weight_x(self.pool_id, new_wx, self.next_update_time)
                        .await?;
                }
                Ok(())
            }
            PoolParams::LogNormal(log_normal_params) => {
                let current_strike = log_normal_params.strike;
                let current_strike_float = parse_ether_to_f64(current_strike)?;
                tracing::info!("current_strike: {:?}", current_strike_float);
                let mut new_strike = current_strike_float;
                let mut scaling_factor = vol_diff * self.sensitivity / self.target_volatility;
                if scaling_factor > self.max_change {
                    scaling_factor = self.max_change;
                }
                if portfolio_rv > self.target_volatility {
                    new_strike -= scaling_factor;
                } else {
                    new_strike += scaling_factor;
                }
                tracing::info!("new_strike: {:?}", new_strike);
                self.protocol_client
                    .set_strike_price(self.pool_id, new_strike, self.next_update_time)
                    .await?;
                Ok(())
            }
        }
    }

    fn calculate_rv(&mut self) -> Result<()> {
        // if self.asset_prices.len() > 15 then only calculate for the last 15 elements
        if self.data.asset_prices.len() > 15 {
            let asset_rv = compute_realized_volatility(
                self.data
                    .asset_prices
                    .iter()
                    .skip(self.data.asset_prices.len() - 15)
                    .map(|(price, _)| *price)
                    .collect::<Vec<f64>>(),
            );
            self.data.asset_rv.push((asset_rv, self.next_update_time));
        }
        if self.data.portfolio_prices.len() > 15 {
            let portfolio_rv = compute_realized_volatility(
                self.data
                    .portfolio_prices
                    .iter()
                    .skip(self.data.portfolio_prices.len() - 15)
                    .map(|(price, _)| *price)
                    .collect::<Vec<f64>>(),
            );

            self.data
                .portfolio_rv
                .push((portfolio_rv, self.next_update_time));
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ParameterManagerParameters<P: Parameterized> {
    pub specialty: Specialty<P>,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum Specialty<P: Parameterized> {
    VolatilityTargeting(VolatilityTargetingParameters<P>),
}

impl From<ParameterManagerParameters<Multiple>> for Vec<ParameterManagerParameters<Single>> {
    fn from(item: ParameterManagerParameters<Multiple>) -> Self {
        let specialties: Vec<Specialty<Single>> = item.specialty.into();
        iproduct!(specialties)
            .map(|specialty| ParameterManagerParameters { specialty })
            .collect()
    }
}

impl From<Specialty<Multiple>> for Vec<Specialty<Single>> {
    fn from(item: Specialty<Multiple>) -> Self {
        match item {
            Specialty::VolatilityTargeting(parameters) => {
                let parameters: Vec<VolatilityTargetingParameters<Single>> = parameters.into();
                parameters
                    .into_iter()
                    .map(Specialty::VolatilityTargeting)
                    .collect()
            }
        }
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct VolatilityTargetingParameters<P: Parameterized> {
    pub target_volatility: P,
    pub update_frequency: P,
    pub sensitivity: P,
    pub max_change: P,
}

impl From<VolatilityTargetingParameters<Multiple>> for Vec<VolatilityTargetingParameters<Single>> {
    fn from(item: VolatilityTargetingParameters<Multiple>) -> Self {
        iproduct!(
            item.target_volatility.parameters(),
            item.update_frequency.parameters(),
            item.sensitivity.parameters(),
            item.max_change.parameters()
        )
        .map(|(tv, uf, s, mc)| VolatilityTargetingParameters {
            target_volatility: Single(tv),
            update_frequency: Single(uf),
            sensitivity: Single(s),
            max_change: Single(mc),
        })
        .collect()
    }
}

/// Math functions for portfolio optimization and management.
/// Compute the returns of a series of values.
/// Which is defined as the ratio of the current value to the previous value
/// minus 1.
pub fn compute_returns(values: impl IntoIterator<Item = f64>) -> Vec<f64> {
    let values = values.into_iter().collect::<Vec<f64>>();
    let mut returns = Vec::new();
    for i in 1..values.len() {
        returns.push(values[i] / values[i - 1] - 1.0);
    }
    returns
}

pub fn compute_log_returns(values: impl IntoIterator<Item = f64>) -> Vec<f64> {
    let mut previous_value = 0.0_f64;
    let mut returns = Vec::new();
    for value in values {
        if previous_value != 0.0 {
            returns.push((value / previous_value).ln());
        }
        previous_value = value;
    }
    returns
}

pub fn compute_simple_returns(values: impl IntoIterator<Item = f64>) -> Vec<f64> {
    let mut previous_value = 0.0_f64;
    let mut returns = Vec::new();
    for value in values {
        if previous_value != 0.0 {
            returns.push(value / previous_value - 1.0);
        }
        previous_value = value;
    }
    returns
}

pub fn compute_net_returns(values: impl IntoIterator<Item = f64>) -> f64 {
    let values = values.into_iter().collect::<Vec<f64>>();
    let net_return = values.last().unwrap_or(&0.0) / values.first().unwrap_or(&1.0) - 1.0;
    net_return
}

pub fn compute_variance(values: impl IntoIterator<Item = f64>) -> f64 {
    let values = values.into_iter().collect::<Vec<f64>>();
    let mean = values.iter().sum::<f64>() / values.len() as f64;
    let variance = values
        .iter()
        .map(|&return_| (return_ - mean).powi(2))
        .sum::<f64>()
        / values.len() as f64;
    variance
}

pub fn compute_std_deviation(values: impl IntoIterator<Item = f64>) -> f64 {
    let variance = compute_variance(values);
    variance.sqrt()
}

pub fn compute_realized_volatility(values: impl IntoIterator<Item = f64>) -> f64 {
    let returns = compute_log_returns(values);
    let len = returns.len() + 1;
    compute_std_deviation(returns) / (len as f64 / 365.0)
}
