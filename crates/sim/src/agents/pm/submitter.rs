use std::sync::Arc;

use alloy_primitives::Address;
use arbiter_bindings::bindings::liquid_exchange::LiquidExchange;
use clients::protocol::ProtocolClient;
use ethers::utils::format_ether;
use itertools::iproduct;
use tracing::{debug, info};

use super::*;

#[derive(Debug, Clone)]
pub struct PositionData {
    pub portfolio_prices: Vec<(f64, u64)>,
    pub asset_prices: Vec<(f64, u64)>,
    pub portfolio_rv: Vec<(f64, u64)>,
    pub asset_rv: Vec<(f64, u64)>,
}

#[derive(Debug, Clone)]
pub struct VolatilityTargetingSubmitter {
    pub client: Arc<RevmMiddleware>,
    pub lex: LiquidExchange<RevmMiddleware>,
    pub protocol_client: ProtocolClient<RevmMiddleware>,
    pub next_update_timestamp: u64,
    pub update_frequency: u64,
    pub target_volatility: f64,
    pub sensitivity: f64,
    pub max_strike_change: f64,
    pub ln_data: PositionData,
    pub g_data: PositionData,
}

#[async_trait::async_trait]
impl Agent for VolatilityTargetingSubmitter {
    async fn step(&mut self) -> Result<()> {
        let timestamp = self.client.get_block_timestamp().await?.as_u64();
        let asset_price =
            ethers::utils::format_ether(self.lex.price().call().await?).parse::<f64>()?;
        let ln_pool = self
            .protocol_client
            .protocol
            .get_pool(ethers::types::U256::from(0))
            .await?;
        let g_pool = self
            .protocol_client
            .protocol
            .get_pool(ethers::types::U256::from(1))
            .await?;
        let ln_rx = ethers::utils::format_ether(ln_pool.reserve_x_wad).parse::<f64>()?;
        let ln_ry = ethers::utils::format_ether(ln_pool.reserve_y_wad).parse::<f64>()?;

        let g_rx = ethers::utils::format_ether(g_pool.reserve_x_wad).parse::<f64>()?;
        let g_ry = ethers::utils::format_ether(g_pool.reserve_y_wad).parse::<f64>()?;

        let ln_portfolio_price = ln_rx * asset_price + ln_ry;
        let g_portfolio_price = g_rx * asset_price + g_ry;

        if self.ln_data.portfolio_prices.is_empty() {
            info!("portfolio_price: {}", ln_portfolio_price);
            self.ln_data.portfolio_prices.push((ln_portfolio_price, 0));
            self.ln_data.asset_prices.push((asset_price, 0));
        }

        if self.g_data.portfolio_prices.is_empty() {
            info!("portfolio_price: {}", g_portfolio_price);
            self.g_data.portfolio_prices.push((g_portfolio_price, 0));
            self.g_data.asset_prices.push((asset_price, 0));
        }

        if timestamp >= self.next_update_timestamp {
            self.next_update_timestamp = timestamp + self.update_frequency;
            debug!("ln portfolio_price: {}", ln_portfolio_price);
            debug!("g3m portfolio_price: {}", g_portfolio_price);
            self.ln_data.asset_prices.push((asset_price, timestamp));
            self.ln_data
                .portfolio_prices
                .push((ln_portfolio_price, timestamp));
            self.g_data.asset_prices.push((asset_price, timestamp));
            self.g_data
                .portfolio_prices
                .push((g_portfolio_price, timestamp));
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

impl VolatilityTargetingSubmitter {
    pub async fn new(
        environment: &Environment,
        config: &SimulationConfig<Single>,
        label: impl Into<String>,
        liquid_exchange_address: Address,
        token_x: H160,
        token_y: H160,
    ) -> Result<Self> {
        let label: String = label.into();
        let client = RevmMiddleware::new(environment, Some(&label))?;
        let lex = LiquidExchange::new(to_ethers_address(liquid_exchange_address), client.clone());

        if let Some(AgentParameters::VolatilityTargetingSubmitter(params)) =
            config.agent_parameters.get(&label)
        {
            match params.specialty {
                Specialty::VolatilityTargeting(parameters) => {
                    let protocol_client =
                        ProtocolClient::new(client.clone(), token_x, token_y, 0.003)
                            .await
                            .unwrap();
                    let strategist = VolatilityTargetingSubmitter {
                        client,
                        lex,
                        protocol_client,
                        update_frequency: parameters.update_frequency.0 as u64,
                        next_update_timestamp: parameters.update_frequency.0 as u64,
                        target_volatility: parameters.target_volatility.0,
                        sensitivity: parameters.sensitivity.0,
                        max_strike_change: parameters.max_strike_change.0,
                        ln_data: PositionData {
                            portfolio_prices: Vec::new(),
                            asset_prices: Vec::new(),
                            portfolio_rv: Vec::new(),
                            asset_rv: Vec::new(),
                        },
                        g_data: PositionData {
                            portfolio_prices: Vec::new(),
                            asset_prices: Vec::new(),
                            portfolio_rv: Vec::new(),
                            asset_rv: Vec::new(),
                        },
                    };
                    Ok(strategist)
                }
            }
        } else {
            Err(anyhow::anyhow!(
                "No parameters found for volatility targeting portfolio manager"
            ))
        }
    }

    async fn execute_smooth_rebalance(&mut self) -> Result<()> {
        if self.ln_data.portfolio_rv.len() < 2 {
            return Ok(());
        }
        if self.g_data.portfolio_rv.len() < 2 {
            return Ok(());
        }
        let ln_portfolio_rv = self.ln_data.portfolio_rv.last().unwrap().0;
        let g_portfolio_rv = self.g_data.portfolio_rv.last().unwrap().0;
        info!("ln portfolio_rv: {}", ln_portfolio_rv);
        info!("g3m portfolio_rv: {}", g_portfolio_rv);
        let current_strike = self.protocol_client.get_strike_price(U256::from(0)).await?;

        let current_strike_float = ethers::utils::format_ether(current_strike)
            .parse::<f64>()
            .unwrap();
        let current_wx = self
            .protocol_client
            .g_strategy
            .weight_x(U256::from(1))
            .await?;
        let wx_float = format_ether(current_wx).parse::<f64>().unwrap();
        info!("current strike float: {}", current_strike_float);
        let mut new_strike = current_strike_float;
        let vol_diff = (ln_portfolio_rv - self.target_volatility).abs();
        let mut scaling_factor = vol_diff * self.sensitivity / self.target_volatility;
        if scaling_factor > self.max_strike_change {
            scaling_factor = self.max_strike_change;
        }
        if ln_portfolio_rv > self.target_volatility {
            new_strike -= scaling_factor;
        } else {
            new_strike += scaling_factor;
        }
        if g_portfolio_rv < self.target_volatility {
            let mut new_weight = wx_float + 0.0025;
            debug!("new weight: {}", new_weight);
            if new_weight >= 0.99 {
                new_weight = 0.99;
            }
            self.protocol_client
                .set_weight_x(U256::from(1), new_weight, self.next_update_timestamp)
                .await?;
        } else {
            let mut new_weight = wx_float - 0.0025;
            if new_weight <= 0.01 {
                new_weight = 0.01;
            }
            debug!("new weight: {}", new_weight);
            self.protocol_client
                .set_weight_x(U256::from(1), new_weight, self.next_update_timestamp)
                .await?;
        }
        info!("new strike float: {}", new_strike);
        self.protocol_client
            .set_strike_price(U256::from(0), new_strike, self.next_update_timestamp)
            .await?;
        Ok(())
    }

    fn calculate_rv(&mut self) -> Result<()> {
        // if self.asset_prices.len() > 15 then only calculate for the last 15 elements
        if self.ln_data.asset_prices.len() > 15 {
            let asset_rv = compute_realized_volatility(
                self.ln_data
                    .asset_prices
                    .iter()
                    .skip(self.ln_data.asset_prices.len() - 15)
                    .map(|(price, _)| *price)
                    .collect::<Vec<f64>>(),
            );
            self.ln_data
                .asset_rv
                .push((asset_rv, self.next_update_timestamp));
        }
        if self.ln_data.portfolio_prices.len() > 15 {
            let portfolio_rv = compute_realized_volatility(
                self.ln_data
                    .portfolio_prices
                    .iter()
                    .skip(self.ln_data.portfolio_prices.len() - 15)
                    .map(|(price, _)| *price)
                    .collect::<Vec<f64>>(),
            );

            self.ln_data
                .portfolio_rv
                .push((portfolio_rv, self.next_update_timestamp));
        }
        info!(
            "Return[ASSET]: {}",
            (self.ln_data.asset_prices.last().unwrap().0
                - self.ln_data.asset_prices.first().unwrap().0)
                / self.ln_data.asset_prices.first().unwrap().0
        );
        info!(
            "Return[LN_PORTFOLIO]: {}",
            (self.ln_data.portfolio_prices.last().unwrap().0
                - self.ln_data.portfolio_prices.first().unwrap().0)
                / self.ln_data.portfolio_prices.first().unwrap().0
        );
        debug!(
            "initial portfolio price: {}",
            self.ln_data.portfolio_prices.first().unwrap().0
        );
        debug!(
            "current portfolio price: {}",
            self.ln_data.portfolio_prices.last().unwrap().0
        );

        // if self.asset_prices.len() > 15 then only calculate for the last 15 elements
        if self.g_data.asset_prices.len() > 15 {
            let asset_rv = compute_realized_volatility(
                self.g_data
                    .asset_prices
                    .iter()
                    .skip(self.g_data.asset_prices.len() - 15)
                    .map(|(price, _)| *price)
                    .collect::<Vec<f64>>(),
            );
            self.g_data
                .asset_rv
                .push((asset_rv, self.next_update_timestamp));
        }
        if self.g_data.portfolio_prices.len() > 15 {
            let portfolio_rv = compute_realized_volatility(
                self.g_data
                    .portfolio_prices
                    .iter()
                    .skip(self.g_data.portfolio_prices.len() - 15)
                    .map(|(price, _)| *price)
                    .collect::<Vec<f64>>(),
            );

            self.g_data
                .portfolio_rv
                .push((portfolio_rv, self.next_update_timestamp));
        }
        info!(
            "Return[ASSET]: {}",
            (self.g_data.asset_prices.last().unwrap().0
                - self.g_data.asset_prices.first().unwrap().0)
                / self.g_data.asset_prices.first().unwrap().0
        );
        info!(
            "Return[G3M_PORTFOLIO]: {}",
            (self.g_data.portfolio_prices.last().unwrap().0
                - self.g_data.portfolio_prices.first().unwrap().0)
                / self.g_data.portfolio_prices.first().unwrap().0
        );
        debug!(
            "initial portfolio price: {}",
            self.g_data.portfolio_prices.first().unwrap().0
        );
        debug!(
            "current portfolio price: {}",
            self.g_data.portfolio_prices.last().unwrap().0
        );

        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SubmitterParameters<P: Parameterized> {
    sigma: P,
    tau: P,
    strike_price: P,
    pub fee: P,
    pub specialty: Specialty<P>,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum Specialty<P: Parameterized> {
    VolatilityTargeting(DynamicVolatilityTargetingParameters<P>),
}

impl From<SubmitterParameters<Multiple>> for Vec<SubmitterParameters<Single>> {
    fn from(item: SubmitterParameters<Multiple>) -> Self {
        let specialties: Vec<Specialty<Single>> = item.specialty.into();
        iproduct!(
            item.sigma.parameters(),
            item.tau.parameters(),
            item.strike_price.parameters(),
            item.fee.parameters(),
            specialties
        )
        .map(|(s, tau, sp, f, specialty)| SubmitterParameters {
            sigma: Single(s),
            tau: Single(tau),
            strike_price: Single(sp),
            fee: Single(f),
            specialty,
        })
        .collect()
    }
}

impl From<Specialty<Multiple>> for Vec<Specialty<Single>> {
    fn from(item: Specialty<Multiple>) -> Self {
        match item {
            Specialty::VolatilityTargeting(parameters) => {
                let parameters: Vec<DynamicVolatilityTargetingParameters<Single>> =
                    parameters.into();
                parameters
                    .into_iter()
                    .map(Specialty::VolatilityTargeting)
                    .collect()
            }
        }
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct DynamicVolatilityTargetingParameters<P: Parameterized> {
    pub target_volatility: P,
    pub update_frequency: P,
    pub sensitivity: P,
    pub max_strike_change: P,
}

impl From<DynamicVolatilityTargetingParameters<Multiple>>
    for Vec<DynamicVolatilityTargetingParameters<Single>>
{
    fn from(item: DynamicVolatilityTargetingParameters<Multiple>) -> Self {
        iproduct!(
            item.target_volatility.parameters(),
            item.update_frequency.parameters(),
            item.sensitivity.parameters(),
            item.max_strike_change.parameters()
        )
        .map(|(tv, uf, s, msc)| DynamicVolatilityTargetingParameters {
            target_volatility: Single(tv),
            update_frequency: Single(uf),
            sensitivity: Single(s),
            max_strike_change: Single(msc),
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
