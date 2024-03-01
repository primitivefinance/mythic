use std::sync::Arc;

use arbiter_core::errors::ArbiterCoreError;
use bindings::{
    atomic_v2::AtomicV2Errors,
    log_normal_solver::{LogNormalSolver, LogNormalSolverErrors},
};
use clients::protocol::{pool::PoolKind, PoolParams, ProtocolClient};
use ethers::{
    types::{Address, U256},
    utils::format_ether,
};
use tracing::info;

use self::agents::portfolio_management::base::arbitrageur::Arbitrageur;
use super::{
    agent::*, agents::base::token_admin::TokenAdmin, ArbiterMiddleware, Environment, Result, *,
};

#[derive(Debug, Clone)]
pub struct LnArbitrageur(pub Arbitrageur);

pub struct ArbInputs {
    pub i_wad: I256,
    pub target_price_wad: I256,
    pub strike: I256,
    pub sigma: I256,
    pub tau: I256,
    pub gamma: I256,
    pub rx: I256,
    pub ry: I256,
    pub liq: I256,
}

impl LnArbitrageur {
    pub async fn new(
        environment: &Environment,
        token_admin: &TokenAdmin,
        liquid_exchange_address: Address,
        protocol_client: ProtocolClient<ArbiterMiddleware>,
        pool_id: U256,
    ) -> Result<Self> {
        let arbitrageur = Arbitrageur::new(
            "ln_arbitrageur",
            environment,
            token_admin,
            liquid_exchange_address,
            protocol_client,
            PoolKind::LogNormal,
            pool_id,
        )
        .await?;
        Ok(Self(arbitrageur))
    }

    pub async fn get_arb_inputs(&self) -> Result<ArbInputs> {
        let i_wad = I256::from_raw(WAD);
        let target_price_wad = I256::from_raw(self.0.liquid_exchange.price().call().await?);
        let pool_params = self.0.protocol_client.get_params(self.0.pool_id).await?;

        let (strike, sigma, tau, swap_fee) = match pool_params {
            PoolParams::LogNormal(ln_params) => (
                ln_params.strike,
                ln_params.sigma,
                ln_params.tau,
                ln_params.swap_fee,
            ),
            PoolParams::GeometricMean(_) => bail!("Failed to parse LogNormal params, received G3M"),
        };

        info!("strike: {:?}", format_ether(strike));
        info!("lex price: {:?}", format_ether(target_price_wad));

        let (strike, sigma, tau, swap_fee) = (
            I256::from_raw(strike),
            I256::from_raw(sigma),
            I256::from_raw(tau),
            I256::from_raw(swap_fee),
        );
        let gamma = i_wad - swap_fee;
        let (rx, ry, liq) = self
            .0
            .protocol_client
            .get_reserves_and_liquidity(self.0.pool_id)
            .await?;
        info!("rx: {:?}", format_ether(rx));
        info!("ry: {:?}", format_ether(ry));
        info!("liq: {:?}", format_ether(liq));
        let (rx, ry, liq) = (I256::from_raw(rx), I256::from_raw(ry), I256::from_raw(liq));
        Ok(ArbInputs {
            i_wad,
            target_price_wad,
            strike,
            sigma,
            tau,
            gamma,
            rx,
            ry,
            liq,
        })
    }

    pub fn assert_valid_dx(
        &self,
        dx: I256,
        rx: I256,
        liq: I256,
        gamma: I256,
        i_wad: I256,
    ) -> Result<I256> {
        let two = I256::from_raw(WAD * U256::from(2));
        let a = (two * (dx + rx)) / i_wad;
        let b = liq + dx - (dx * gamma) / i_wad;
        let res = (a * i_wad) / (b);

        if res > two {
            let next_dx = (liq - rx) / gamma;
            Ok(next_dx - 1_000)
        } else {
            Ok(dx)
        }
    }

    pub async fn get_dx(&self) -> Result<I256> {
        let ArbInputs {
            i_wad,
            target_price_wad,
            strike,
            sigma,
            tau: _,
            gamma,
            rx,
            ry: _,
            liq,
        } = self.get_arb_inputs().await?;

        let log_p = self
            .0
            .atomic_arbitrage
            .log(target_price_wad * i_wad / strike)
            .call()
            .await?;
        let inner_p = log_p * i_wad / sigma + (sigma / 2);
        let cdf_p = self.0.atomic_arbitrage.cdf(inner_p).call().await?;
        let delta = liq * (i_wad - cdf_p) / i_wad;
        let num = (delta - rx) * i_wad;
        let den_a = ((gamma - i_wad) * (i_wad - cdf_p)) / i_wad;
        let den_b = (rx * i_wad / liq) + i_wad;
        let dx = num / ((den_a * i_wad / den_b) + i_wad);

        let valid_dx = self.assert_valid_dx(dx, rx, liq, gamma, i_wad)?;

        Ok(valid_dx)
    }

    pub fn assert_valid_dy(
        &self,
        dy: I256,
        ry: I256,
        liq: I256,
        strike: I256,
        gamma: I256,
        i_wad: I256,
    ) -> Result<I256> {
        let two = I256::from_raw(WAD * U256::from(2));
        let a = (two * (dy + ry)) / i_wad;
        let b = (strike * liq) / i_wad + dy - (dy * gamma) / i_wad;
        let res = (a * i_wad) / b;

        if res > two {
            let next_dy = (((strike * liq) / i_wad) - ry) / gamma;
            Ok(next_dy - 1_000)
        } else {
            Ok(dy)
        }
    }

    // todo (matt): figure out why this returns u256::max sometimes
    pub async fn get_dy(&self) -> Result<I256> {
        let ArbInputs {
            i_wad,
            target_price_wad,
            strike,
            sigma,
            tau: _,
            gamma,
            rx: _,
            ry,
            liq,
        } = self.get_arb_inputs().await?;

        let log_p = self
            .0
            .atomic_arbitrage
            .log(target_price_wad * i_wad / strike)
            .call()
            .await?;
        let inner_p = log_p * i_wad / sigma - (sigma / 2);
        let cdf_p = self.0.atomic_arbitrage.cdf(inner_p).call().await?;
        let delta = (liq * strike) / i_wad * (cdf_p) / i_wad;
        let num = (delta - ry) * i_wad;
        let den_a = ((gamma - i_wad) * cdf_p) / i_wad;
        //        println!("den_a: {:?}", den_a);
        let den_b = ry * i_wad / ((strike * liq) / i_wad);
        //       println!("den_b: {:?}", den_b);
        let dy = num / ((den_a * i_wad / den_b) + i_wad);
        //      println!("dy: {:?}", dy);

        let valid_dy = self.assert_valid_dy(dy, ry, liq, strike, gamma, i_wad)?;
        return Ok(valid_dy);
    }
}

#[async_trait::async_trait]
impl Agent for LnArbitrageur {
    #[allow(unused)]
    async fn step(&mut self) -> Result<(), ArbiterCoreError> {
        self.0
            .atomic_arbitrage
            .log_data(self.0.pool_id)
            .send()
            .await
            .unwrap();
        match self.0.detect_arbitrage().await.unwrap() {
            Swap::RaiseExchangePrice(target_price) => {
                info!("Signal[RAISE PRICE]: {:?}", format_ether(target_price));
                let x_in = false;
                let input = self.get_dy().await.unwrap().into_raw();

                let ln_params = self
                    .0
                    .protocol_client
                    .ln_solver
                    .fetch_pool_params(self.0.pool_id)
                    .call()
                    .await
                    .unwrap();

                let reserves = self
                    .0
                    .protocol_client
                    .ln_solver
                    .get_reserves_and_liquidity(self.0.pool_id)
                    .call()
                    .await
                    .unwrap();

                let current_price = self
                    .0
                    .protocol_client
                    .get_internal_price(self.0.pool_id)
                    .await
                    .unwrap();

                let optimal_dy = match self
                    .0
                    .protocol_client
                    .ln_solver
                    .compute_optimal_arb_raise_price(self.0.pool_id, target_price, input)
                    .call()
                    .await
                {
                    Ok(optimal_dy) => optimal_dy,
                    Err(e) => {
                        let bytes = match e.as_middleware_error().unwrap() {
                            ArbiterCoreError::ExecutionRevert { output, gas_used } => {
                                info!("Execution revert: {:?} Gas Used: {:?}", output, gas_used);
                                output
                            }
                            _ => {
                                error!(
                                    "Error computing optimal_dy: {:?}, with params: {:?}, with
                                reserves: {:?}, target_price: {:?}, current_price: {:?}",
                                    e, ln_params, reserves, target_price, current_price
                                );
                                return Ok(());
                            }
                        };
                        let err = LogNormalSolverErrors::decode_with_selector(bytes).unwrap();
                        // let err = AtomicV2Errors::decode_with_selector(bytes).unwrap();
                        // error!("Error computing optimal_dy: {:?}", err);
                        error!(
                            "Error computing optimal_dy: {:?}, with params: {:?}, with
                        reserves: {:?}, with upper bound: {:?}, target_price: {:?},
                        current_price: {:?}",
                            err, ln_params, reserves, input, target_price, current_price
                        );
                        return Ok(());
                    }
                };

                info!("optimal_dy: {:?}", optimal_dy);

                let tx = self
                    .0
                    .atomic_arbitrage
                    .raise_exchange_price(self.0.pool_id, optimal_dy);

                let output = tx.send().await;

                match output {
                    Ok(output) => {
                        let internal_price = self
                            .0
                            .protocol_client
                            .get_internal_price(self.0.pool_id)
                            .await?;
                        info!("Price Post Swap[LEX]: {:?}", format_ether(target_price));
                        info!(
                            "Price Post Swap[LOGNORM]: {:?}",
                            format_ether(internal_price)
                        );
                        self.0.num_arbs += 1;
                        info!("Arb Count: {:?}", self.0.num_arbs);
                        output.await?;
                    }
                    Err(e) => {
                        if let ArbiterCoreError::ExecutionRevert { gas_used, output } =
                            e.as_middleware_error().unwrap()
                        {
                            info!("[LOGNORM]: Swap failed");
                            debug!("Execution revert: {:?} Gas Used: {:?}", output, gas_used);
                        }
                    }
                }
            }
            Swap::LowerExchangePrice(target_price) => {
                info!(
                    "[LogNorm]: Signal[LOWER PRICE] {:?}",
                    format_ether(target_price)
                );

                let x_in = true;
                let liquid_exchange_price = self.0.liquid_exchange.price().call().await.unwrap();
                let input = self.get_dx().await.unwrap().into_raw();

                let ln_params = self
                    .0
                    .protocol_client
                    .ln_solver
                    .fetch_pool_params(self.0.pool_id)
                    .call()
                    .await
                    .unwrap();

                let reserves = self
                    .0
                    .protocol_client
                    .ln_solver
                    .get_reserves_and_liquidity(self.0.pool_id)
                    .call()
                    .await
                    .unwrap();

                let current_price = self
                    .0
                    .protocol_client
                    .get_internal_price(self.0.pool_id)
                    .await
                    .unwrap();

                let optimal_dx = match self
                    .0
                    .protocol_client
                    .ln_solver
                    .compute_optimal_arb_lower_price(self.0.pool_id, target_price, input)
                    .call()
                    .await
                {
                    Ok(optimal_dx) => optimal_dx,
                    Err(e) => {
                        let bytes = match e.as_middleware_error().unwrap() {
                            ArbiterCoreError::ExecutionRevert { output, gas_used } => {
                                info!("Execution revert: {:?} Gas Used: {:?}", output, gas_used);
                                output
                            }
                            _ => {
                                error!(
                                    "Error getting optimal_dx: {:?}, with params: {:?}, with
                                reserves: {:?}, target_price: {:?}, current_price: {:?}",
                                    e, ln_params, reserves, target_price, current_price
                                );
                                return Ok(());
                            }
                        };
                        // let err = AtomicV2Errors::decode_with_selector(bytes).unwrap();
                        // error!("Error computing optimal_dx: {:?}", err);
                        let err = LogNormalSolverErrors::decode_with_selector(bytes).unwrap();
                        error!(
                            "Error computing optimal_dx: {:?}, with params: {:?}, with
                        reserves: {:?}, with upper bound: {:?}, target_price: {:?},
                        current_price: {:?}",
                            err, ln_params, reserves, input, target_price, current_price
                        );
                        return Ok(());
                    }
                };

                info!("optimal_dx: {:?}", optimal_dx);

                let optimal_dx =
                    optimal_dx * liquid_exchange_price / ethers::utils::parse_ether("1").unwrap();

                let tx = self
                    .0
                    .atomic_arbitrage
                    .lower_exchange_price(self.0.pool_id, optimal_dx);

                let output = tx.send().await;

                match output {
                    Ok(output) => {
                        let internal_price = self
                            .0
                            .protocol_client
                            .get_internal_price(self.0.pool_id)
                            .await?;
                        info!("Price Post Swap [LEX]: {:?}", format_ether(target_price));
                        info!(
                            "Price Post Swap [LOGNORM]: {:?}",
                            format_ether(internal_price)
                        );
                        self.0.num_arbs += 1;
                        info!("Arb Count: {:?}", self.0.num_arbs);
                        output.await?;
                    }
                    Err(e) => {
                        if let ArbiterCoreError::ExecutionRevert { gas_used, output } =
                            e.as_middleware_error().unwrap()
                        {
                            info!("[LOGNORM]: Swap failed");
                            debug!("Execution revert: {:?} Gas Used: {:?}", output, gas_used);
                        }
                    }
                }
            }
            _ => {
                trace!("No arbitrage opportunity");
            }
        }
        Ok(())
    }

    fn client(&self) -> Arc<ArbiterMiddleware> {
        self.0.client.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
