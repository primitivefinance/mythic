use std::sync::Arc;

use arbiter_core::errors::ArbiterCoreError;
use clients::protocol::{pool::PoolKind, PoolParams, ProtocolClient};
use ethers::{
    types::{Bytes, I256, U256},
    utils::format_ether,
};
use tracing::info;

use super::{
    agent::*,
    agents::{base::token_admin::TokenAdmin, portfolio_management::base::arbitrageur::Arbitrageur},
    ArbiterMiddleware, Environment, Result, *,
};

#[derive(Debug, Clone)]
pub struct G3mArbitrageur(pub Arbitrageur);

pub struct G3mArbInputs {
    pub i_wad: I256,
    pub target_price_wad: I256,
    pub wx: I256,
    pub wy: I256,
    pub gamma: I256,
    pub rx: I256,
    pub ry: I256,
    pub liq: I256,
}

impl G3mArbitrageur {
    pub async fn new(
        environment: &Environment,
        token_admin: &TokenAdmin,
        liquid_exchange_address: Address,
        protocol_client: ProtocolClient<ArbiterMiddleware>,
        pool_id: U256,
    ) -> Result<Self> {
        let arbitrageur = Arbitrageur::new(
            "g3m_arbitrageur",
            environment,
            token_admin,
            liquid_exchange_address,
            protocol_client,
            PoolKind::GeometricMean,
            pool_id,
        )
        .await?;
        Ok(Self(arbitrageur))
    }

    pub async fn get_arb_inputs(&self) -> Result<G3mArbInputs> {
        let i_wad = I256::from_raw(WAD);
        let target_price_wad = I256::from_raw(self.0.liquid_exchange.price().call().await?);
        let pool_params = self.0.protocol_client.get_params(self.0.pool_id).await?;

        let (wx, wy, swap_fee) = match pool_params {
            PoolParams::GeometricMean(g3m_params) => {
                (g3m_params.w_x, g3m_params.w_y, g3m_params.swap_fee)
            }
            PoolParams::LogNormal(_) => bail!("Failed to parse G3M params, received LogNormal"),
        };

        let (wx, wy, swap_fee) = (
            I256::from_raw(wx),
            I256::from_raw(wy),
            I256::from_raw(swap_fee),
        );
        let gamma = i_wad - swap_fee;
        let (rx, ry, liq) = self
            .0
            .protocol_client
            .get_reserves_and_liquidity(self.0.pool_id)
            .await?;
        let (rx, ry, liq) = (I256::from_raw(rx), I256::from_raw(ry), I256::from_raw(liq));

        Ok(G3mArbInputs {
            i_wad,
            target_price_wad,
            wx,
            wy,
            gamma,
            rx,
            ry,
            liq,
        })
    }

    pub async fn get_dx(&self) -> Result<I256> {
        let G3mArbInputs {
            i_wad,
            target_price_wad,
            wx,
            wy,
            gamma,
            rx,
            ry: _,
            liq,
        } = self.get_arb_inputs().await?;

        let ratio = wx * i_wad / wy;

        let inside = ratio * i_wad / target_price_wad;
        let delta_x =
            (liq * self.0.arb_math.pow(inside, wy).call().await? / i_wad - rx) * (i_wad / gamma);

        Ok(delta_x)
    }

    pub async fn get_dy(&self) -> Result<I256> {
        let G3mArbInputs {
            i_wad,
            target_price_wad,
            wx,
            wy,
            gamma,
            rx: _,
            ry,
            liq,
        } = self.get_arb_inputs().await?;
        let ratio = wy * i_wad / wx;

        let inside = ratio * target_price_wad / i_wad;
        let delta_y =
            (liq * self.0.arb_math.pow(inside, wx).call().await? / i_wad - ry) * (i_wad / gamma);

        Ok(delta_y)
    }
}

#[async_trait::async_trait]
impl Agent for G3mArbitrageur {
    #[allow(unused)]
    async fn step(&mut self) -> Result<(), ArbiterCoreError> {
        match self.0.detect_arbitrage().await.unwrap() {
            Swap::RaiseExchangePrice(target_price) => {
                info!(
                    "[G3M]: Signal[RAISE PRICE]: {:?}",
                    format_ether(target_price)
                );
                let x_in = false;
                let mut input = self.get_dy().await.unwrap();
                if (input < I256::from(0)) {
                    input = I256::from(0);
                    info!("Encountered negative Y input for G3m swap")
                }

                let optimal_dy = self
                    .0
                    .protocol_client
                    .g_solver
                    .compute_optimal_arb_raise_price(self.0.pool_id, target_price, input.into_raw())
                    .call()
                    .await
                    .unwrap();

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
                        info!("Price Post Swap [LEX]: {:?}", format_ether(target_price));
                        info!("Price Post Swap [G3M]: {:?}", format_ether(internal_price));
                        output.await?;
                    }
                    Err(e) => {
                        if let ArbiterCoreError::ExecutionRevert { gas_used, output } =
                            e.as_middleware_error().unwrap()
                        {
                            info!("[G3M]: Swap failed");
                            debug!(
                                "Execution revert: {:?} Gas Used: {:?}",
                                Bytes::from(output.clone()),
                                gas_used
                            );
                        }
                    }
                }
            }
            Swap::LowerExchangePrice(target_price) => {
                info!(
                    "[G3M]: Signal[LOWER PRICE] {:?}",
                    format_ether(target_price)
                );
                let x_in = true;
                let liquid_exchange_price = self.0.liquid_exchange.price().call().await.unwrap();
                let mut input = self.get_dx().await.unwrap();
                if (input < I256::from(0)) {
                    info!("Encountered negative X input for G3m swap");
                    input = I256::from(0);
                }
                let optimal_dx = self
                    .0
                    .protocol_client
                    .g_solver
                    .compute_optimal_arb_lower_price(self.0.pool_id, target_price, input.into_raw())
                    .call()
                    .await
                    .unwrap();
                let input = optimal_dx * liquid_exchange_price / WAD;

                let tx = self
                    .0
                    .atomic_arbitrage
                    .lower_exchange_price(self.0.pool_id, input);

                let output = tx.send().await;

                match output {
                    Ok(output) => {
                        let internal_price = self
                            .0
                            .protocol_client
                            .get_internal_price(self.0.pool_id)
                            .await?;
                        info!("Price Post Swap [LEX]: {:?}", format_ether(target_price));
                        info!("Price Post Swap [G3M]: {:?}", format_ether(internal_price));
                        output.await?;
                    }
                    Err(e) => {
                        if let ArbiterCoreError::ExecutionRevert { gas_used, output } =
                            e.as_middleware_error().unwrap()
                        {
                            info!("[G3M]: Swap failed");
                            debug!(
                                "Execution revert: {:?} Gas Used: {:?}",
                                Bytes::from(output.clone()),
                                gas_used
                            );
                        }
                    }
                }
                trace!("Sent arbitrage.");

                debug!("[G3M]: === End Loop ===");
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
