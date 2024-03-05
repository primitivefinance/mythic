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

    pub async fn assert_valid_dx(&self, dx: I256) -> Result<U256> {
        let i_wad = I256::from_raw(WAD);
        let params = self
            .0
            .protocol_client
            .ln_solver
            .fetch_pool_params(self.0.pool_id)
            .call()
            .await
            .unwrap();

        let gamma = I256::from_raw(WAD - params.swap_fee);
        let (rx, ry, liq) = self
            .0
            .protocol_client
            .ln_solver
            .get_reserves_and_liquidity(self.0.pool_id)
            .call()
            .await
            .unwrap();

        let two = I256::from_raw(WAD * U256::from(2));
        let a = (two * (dx + I256::from_raw(rx))) / i_wad;
        let b = I256::from_raw(liq) + dx - (dx * gamma) / i_wad;
        let res = (a * i_wad) / (b);

        if res > two {
            let next_dx = (I256::from_raw(liq) - I256::from_raw(rx)) / gamma;
            let (_, dx) = next_dx.into_sign_and_abs();

            Ok(dx)
        } else {
            Ok(dx.into_raw())
        }
    }

    pub async fn assert_valid_dy(&self, dy: I256) -> Result<U256> {
        let i_wad = I256::from_raw(WAD);
        let params = self
            .0
            .protocol_client
            .ln_solver
            .fetch_pool_params(self.0.pool_id)
            .call()
            .await
            .unwrap();

        let gamma = I256::from_raw(WAD - params.swap_fee);
        let strike = I256::from_raw(params.strike);
        let (rx, ry, liq) = self
            .0
            .protocol_client
            .ln_solver
            .get_reserves_and_liquidity(self.0.pool_id)
            .call()
            .await
            .unwrap();

        let a = dy + I256::from_raw(ry);
        let b = (strike * I256::from_raw(liq)) / i_wad + (i_wad - gamma) * dy / i_wad;
        let res = (a * i_wad) / b;

        if res > i_wad {
            let next_dy = ((strike * I256::from_raw(liq)) - I256::from_raw(ry) * i_wad) / gamma;
            let (_, dy) = next_dy.into_sign_and_abs();
            Ok(dy)
        } else {
            Ok(dy.into_raw())
        }
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
                let input = self
                    .0
                    .protocol_client
                    .ln_solver
                    .get_dy_given_s(self.0.pool_id, target_price)
                    .call()
                    .await
                    .unwrap();

                let input = self.assert_valid_dy(input).await.unwrap();

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

                if input > U256::from(0) {
                    let optimal_dy = match self
                        .0
                        .protocol_client
                        .ln_solver
                        .compute_optimal_arb_raise_price(self.0.pool_id, target_price, input - 1000)
                        .call()
                        .await
                    {
                        Ok(optimal_dy) => optimal_dy,
                        Err(e) => {
                            let bytes = match e.as_middleware_error().unwrap() {
                                ArbiterCoreError::ExecutionRevert { output, gas_used } => {
                                    info!(
                                        "Execution revert: {:?} Gas Used: {:?}",
                                        output, gas_used
                                    );
                                    output
                                }
                                _ => {
                                    error!(
                                        "Error computing optimal_dy: {:?}, with params: {:?}, with
                                reserves: {:?}, target_price: {:?}, current_price: {:?}",
                                        e, ln_params, reserves, target_price, current_price
                                    );
                                    panic!();
                                }
                            };
                            let err = LogNormalSolverErrors::decode_with_selector(bytes).unwrap();
                            error!(
                                "Error computing optimal_dy: {:?}, with params: {:?}, with
                reserves: {:?}, with upper bound: {:?}, target_price: {:?},
                current_price: {:?}",
                                err.clone(),
                                ln_params,
                                reserves,
                                input,
                                target_price,
                                current_price
                            );
                            U256::from(1)
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

                                error!(
                                    "Execution revert: {:?} Gas Used: {:?}",
                                    ethers::utils::hex::encode(output),
                                    gas_used
                                );
                            }
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

                let input = self
                    .0
                    .protocol_client
                    .ln_solver
                    .get_dx_given_s(self.0.pool_id, target_price)
                    .call()
                    .await
                    .unwrap();
                let input = self.assert_valid_dx(input).await.unwrap();

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

                if input > U256::from(0) {
                    let optimal_dx = match self
                        .0
                        .protocol_client
                        .ln_solver
                        .compute_optimal_arb_lower_price(self.0.pool_id, target_price, input - 1000)
                        .call()
                        .await
                    {
                        Ok(optimal_dx) => optimal_dx,
                        Err(e) => {
                            let bytes = match e.as_middleware_error().unwrap() {
                                ArbiterCoreError::ExecutionRevert { output, gas_used } => {
                                    info!(
                                        "Execution revert: {:?} Gas Used: {:?}",
                                        output, gas_used
                                    );
                                    output
                                }
                                _ => {
                                    error!(
                                        "Error getting optimal_dx: {:?}, with params: {:?}, with
                                reserves: {:?}, target_price: {:?}, current_price: {:?}",
                                        e, ln_params, reserves, target_price, current_price
                                    );
                                    panic!();
                                }
                            };
                            // let err = AtomicV2Errors::decode_with_selector(bytes).unwrap();
                            // error!("Error computing optimal_dx: {:?}", err);
                            let err = LogNormalSolverErrors::decode_with_selector(bytes).unwrap();
                            error!(
                                "Error computing optimal_dx: {:?}, with params: {:?}, with
                reserves: {:?}, with upper bound: {:?}, target_price: {:?},
                current_price: {:?}",
                                err.clone(),
                                ln_params,
                                reserves,
                                input,
                                target_price,
                                current_price
                            );
                            U256::from(1)
                        }
                    };

                    info!("optimal_dx: {:?}", optimal_dx);

                    let optimal_dx = optimal_dx * target_price / WAD;

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
                                let hex = ethers::utils::hex::encode(output);
                                error!("Execution revert: {:?} Gas Used: {:?}", hex, gas_used);
                            }
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
