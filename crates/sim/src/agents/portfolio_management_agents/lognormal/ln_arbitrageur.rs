use std::sync::Arc;

use arbiter_core::middleware::errors::RevmMiddlewareError;
use clients::protocol::{pool::PoolKind, PoolParams, ProtocolClient};
use ethers::{
    types::{Address, U256},
    utils::format_ether,
};
use tracing::info;

use self::agents::portfolio_management_agents::base::arbitrageur::Arbitrageur;
use super::{
    agent::*, agents::base::token_admin::TokenAdmin, Environment, Result, RevmMiddleware, *,
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
        protocol_client: ProtocolClient<RevmMiddleware>,
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
            PoolParams::G3M(_) => bail!("Failed to parse LogNormal params, received G3M"),
        };

        info!("strike: {:?}", format_ether(strike));
        info!("sigma: {:?}", format_ether(sigma));
        info!("tau: {:?}", format_ether(tau));

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
        let dx = (delta - rx) * i_wad * i_wad
            / (((gamma - i_wad) * (i_wad - cdf_p)) / (rx * i_wad / liq) + i_wad);
        info!("dx: {:?}", dx / i_wad);
        Ok(dx / i_wad)
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
        let dy = (delta - ry) * i_wad * i_wad
            / (((gamma - i_wad) * cdf_p) / (ry * i_wad * i_wad / (strike * liq)) + i_wad);
        info!("dy: {:?}", dy / i_wad);

        Ok(dy / i_wad)
    }
}

#[async_trait::async_trait]
impl Agent for LnArbitrageur {
    #[allow(unused)]
    async fn step(&mut self) -> Result<()> {
        match self.0.detect_arbitrage().await? {
            Swap::RaiseExchangePrice(target_price) => {
                info!("Signal[RAISE PRICE]: {:?}", format_ether(target_price));
                let x_in = false;
                let input = self.get_dy().await?.into_raw();

                let tx = self
                    .0
                    .atomic_arbitrage
                    .raise_exchange_price(self.0.pool_id, input);

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
                        output.await?;
                    }
                    Err(e) => {
                        if let RevmMiddlewareError::ExecutionRevert { gas_used, output } =
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
                let liquid_exchange_price = self.0.liquid_exchange.price().call().await?;
                let input = self.get_dx().await?.into_raw();

                let input = input * liquid_exchange_price / ethers::utils::parse_ether("1")?;

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
                        info!(
                            "Price Post Swap [LOGNORM]: {:?}",
                            format_ether(internal_price)
                        );
                        output.await?;
                    }
                    Err(e) => {
                        if let RevmMiddlewareError::ExecutionRevert { gas_used, output } =
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

    fn client(&self) -> Arc<RevmMiddleware> {
        self.0.client.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
