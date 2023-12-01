use std::{str::FromStr, sync::atomic};

use arbiter_core::bindings::arbiter_math::ArbiterMath;
use bindings::rmm_atomic_arbitrage::RMMAtomicArbitrage;
use ethers::abi::AbiEncode;

use super::*;
use crate::{
    agents::Agent,
    bindings::rmm_math_like::RMMMathLike,
    strategy::{
        rmm::{RmmStrategy, RmmStrategyData},
        ArbitrageStrategy,
    },
};

#[derive(Debug, Clone)]
pub struct RmmArbitrageur<S: ArbitrageStrategy> {
    pub client: Arc<RevmMiddleware>,
    /// The arbitrageur's client connection to the liquid exchange.
    pub liquid_exchange: LiquidExchange<RevmMiddleware>,
    /// The rmm strategy used by the exchange.
    pub rmm_strategy: S,
    /// The atomic arbitrage contract.
    pub atomic_arbitrage: RMMAtomicArbitrage<RevmMiddleware>,
    pub g3m_math: SD59x18Math<RevmMiddleware>,
    pub rmm_math: ArbiterMath<RevmMiddleware>,
}

impl<S: ArbitrageStrategy> RmmArbitrageur<S> {
    pub async fn new(
        environment: &Environment,
        token_admin: &token_admin::TokenAdmin,
        liquid_exchange_address: Address,
        rmm_address: Address,
    ) -> Result<Self> {
        // Create a client for the arbitrageur.
        let client = RevmMiddleware::new(environment, "arbitrageur".into())?;

        // Get the exchanges and arb contract connected to the arbitrageur client.
        let liquid_exchange = LiquidExchange::new(liquid_exchange_address, client.clone());
        let rmm_strategy = S::new(rmm_address, client.clone());
        let atomic_arbitrage = RMMAtomicArbitrage::deploy(
            client.clone(),
            (
                rmm_address,
                liquid_exchange_address,
                token_admin.arbx.address(),
                token_admin.arby.address(),
            ),
        )?
        .send()
        .await?;

        let arbx = ArbiterToken::new(token_admin.arbx.address(), client.clone());
        let arby = ArbiterToken::new(token_admin.arby.address(), client.clone());

        token_admin
            .mint(
                client.address(),
                parse_ether(10).unwrap(),
                parse_ether(10).unwrap(),
            )
            .await?;

        arbx.approve(atomic_arbitrage.address(), U256::MAX)
            .send()
            .await?
            .await?;
        arby.approve(atomic_arbitrage.address(), U256::MAX)
            .send()
            .await?
            .await?;

        let arby_allowance = arby
            .allowance(client.address(), atomic_arbitrage.address())
            .call()
            .await?;
        let arbx_allowance = arbx
            .allowance(client.address(), atomic_arbitrage.address())
            .call()
            .await?;

        println!("arby_allowance: {:?}", arby_allowance);

        let g3m_math = SD59x18Math::deploy(client.clone(), ())?.send().await?;
        let rmm_math = ArbiterMath::deploy(client.clone(), ())?.send().await?;

        Ok(Self {
            client,
            liquid_exchange,
            atomic_arbitrage,
            rmm_strategy,
            g3m_math,
            rmm_math,
        })
    }

    /// Detects if there is an arbitrage opportunity.
    /// Returns the direction of the swap `XtoY` or `YtoX` if there is an
    /// arbitrage opportunity. Returns `None` if there is no arbitrage
    /// opportunity.
    async fn detect_arbitrage(&self, pool: &S) -> Result<Swap> {
        // Update the prices the for the arbitrageur.
        let liquid_exchange_price_wad = self.liquid_exchange.price().call().await?;
        let price = pool.get_spot_price().await?;
        debug!("liquid_exchange_price_wad: {:?}", liquid_exchange_price_wad);
        debug!("rmm_price_wad: {:?}", price);

        let gamma_wad = WAD - pool.get_swap_fee().await?;

        let upper_arb_bound = WAD * price / (WAD - gamma_wad);
        let lower_arb_bound = price * (WAD - gamma_wad) / WAD;
        info!(
            "upper_arb_bound: {:?}, lower_arb_bound: {:?}",
            format_units(upper_arb_bound, "ether")?,
            format_units(lower_arb_bound, "ether")?
        );

        // Check if we have an arbitrage opportunity by comparing against the bounds and
        // current price.
        // If these conditions are not satisfied, there cannot be a profitable
        // arbitrage. See: [An Analysis of Uniswap Markets](https://arxiv.org/pdf/1911.03380.pdf) Eq. 3, for example.
        if liquid_exchange_price_wad > upper_arb_bound && liquid_exchange_price_wad > price {
            // Raise the portfolio price by selling asset for quote
            Ok(Swap::RaiseExchangePrice(liquid_exchange_price_wad))
        } else if liquid_exchange_price_wad < lower_arb_bound && liquid_exchange_price_wad < price {
            // Lower the exchange price by selling asset for quote
            Ok(Swap::LowerExchangePrice(liquid_exchange_price_wad))
        } else {
            // Prices are within the no-arbitrage bounds, so we don't have an arbitrage.
            Ok(Swap::None)
        }
    }
}
// TODO: make sure we're swapping on low and high vol strategies
#[async_trait::async_trait]
impl<S: ArbitrageStrategy + std::marker::Sync + std::marker::Send + 'static> Agent
    for RmmArbitrageur<S>
{
    #[allow(unused)]
    async fn step(&mut self) -> Result<()> {
        // Detect if there is an arbitrage opportunity.
        let arbx = self.atomic_arbitrage.asset().call().await?;
        let arby = self.atomic_arbitrage.quote().call().await?;
        let arbx = ArbiterToken::new(arbx, self.client.clone());
        let arby = ArbiterToken::new(arby, self.client.clone());
        let arbx_balance = arbx.balance_of(self.client.address()).call().await?;
        let arby_balance = arby.balance_of(self.client.address()).call().await?;
        debug!("arby_balance: {:?}", arby_balance);

        match self.detect_arbitrage(&self.rmm_strategy).await? {
            Swap::RaiseExchangePrice(target_price) => {
                info!(
                    "Detected the need to raise price to {:?}",
                    format_units(target_price, "ether")?
                );
                let (input, next_liquidity) = self
                    .rmm_strategy
                    .get_y_input(target_price, &self.g3m_math, &self.rmm_math)
                    .await?;

                info!("got input: {:?}", input);
                if input <= 0.into() {
                    return Ok(());
                }

                let tx = self
                    .atomic_arbitrage
                    .raise_exchange_price(input, next_liquidity);

                let output = tx.send().await;
                let arbx_balance = arbx.balance_of(self.client.address()).call().await?;
                let arby_balance = arby.balance_of(self.client.address()).call().await?;
                debug!("arby_balance after: {:?}", arby_balance);
                match output {
                    Ok(output) => {
                        output.await?;
                    }
                    Err(e) => {
                        if let RevmMiddlewareError::ExecutionRevert { gas_used, output } =
                            e.as_middleware_error().unwrap()
                        {
                            let decoded_output =
                                bindings::rmm_atomic_arbitrage::NotProfitable::decode(&output)?;
                            info!(
                                "Execution revert: {:?} Gas Used: {:?}",
                                decoded_output, gas_used
                            );
                        }
                    }
                }
            }
            Swap::LowerExchangePrice(target_price) => {
                info!(
                    "Detected the need to lower price to {:?}",
                    format_units(target_price, "ether")?
                );
                let (input, next_liquidity) = self
                    .rmm_strategy
                    .get_x_input(target_price, &self.g3m_math, &self.rmm_math)
                    .await?;
                info!("Got input: {:?}", input);
                if input <= 0.into() {
                    return Ok(());
                }
                let tx = self
                    .atomic_arbitrage
                    .lower_exchange_price(input, next_liquidity);
                let output = tx.send().await;
                let arbx_balance = arbx.balance_of(self.client.address()).call().await?;
                let arby_balance = arby.balance_of(self.client.address()).call().await?;
                trace!("arby_balance after: {:?}", arby_balance);
                match output {
                    Ok(output) => {
                        output.await?;
                    }
                    Err(e) => {
                        if let RevmMiddlewareError::ExecutionRevert { gas_used, output } =
                            e.as_middleware_error().unwrap()
                        {
                            let decoded_output =
                                bindings::rmm_atomic_arbitrage::NotProfitable::decode(&output)?;
                            info!(
                                "Execution revert: {:?} Gas Used: {:?}",
                                decoded_output, gas_used
                            );
                        }
                    }
                }
                info!("Sent arbitrage.");
            }
            Swap::None => {
                info!("No arbitrage opportunity");
            }
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

enum Swap {
    RaiseExchangePrice(U256),
    LowerExchangePrice(U256),
    None,
}

#[cfg(test)]
mod tests {
    use statrs::distribution::Normal;
    use tracing_subscriber::prelude::*;
    use RustQuant::assert_approx_equal;

    use super::*;
    use crate::{
        agents::{
            price_changer::PriceChanger,
            rmm::{
                liquidity_provider::RmmLiquidityProvider,
                rmm_portfolio_manager::RmmPortfolioManagerType,
            },
            token_admin::TokenAdmin,
        },
        bindings::rmm_math_like::RMMMathLike,
        simulations::import,
        strategy::rmm::{get_strategy_args, RmmStrategy},
    };

    async fn setup(
        environment: &Environment,
    ) -> anyhow::Result<(RmmArbitrageur<RmmStrategy>, RMMMathLike<RevmMiddleware>), anyhow::Error>
    {
        let cwd = std::env::current_dir().unwrap();
        let path = std::path::Path::new(cwd.to_str().unwrap());
        let config_path = path
            .parent()
            .unwrap()
            .join("configs")
            .join("rmm_vol_targeting")
            .join("static.toml");
        let configuration = import(config_path.to_str().unwrap())?;
        let direct_configs: Vec<SimulationConfig<Single>> = configuration.clone().into();
        let config = &direct_configs.clone()[0];

        let token_admin = TokenAdmin::new(environment, config, "token_admin").await?;
        let mut price_changer =
            PriceChanger::new(environment, config, "price_changer", &token_admin).await?;
        let rmm_portfolio_manager = RmmPortfolioManagerType::new(
            environment,
            config,
            "portfolio_manager",
            price_changer.liquid_exchange.address(),
        )
        .await?;

        let client = RevmMiddleware::new(environment, Some(&"rmm_math_like"))?;
        let rmm_math = RMMMathLike::deploy(client.clone(), ())?.send().await?;

        let mut lp = RmmLiquidityProvider::<RmmStrategy>::new(
            environment,
            config,
            "lp",
            &token_admin,
            rmm_portfolio_manager.0.rmm().address(),
        )
        .await?;

        lp.startup().await?;

        let arbitrageur = RmmArbitrageur::<RmmStrategy>::new(
            environment,
            &token_admin,
            price_changer.liquid_exchange.address(),
            rmm_portfolio_manager.0.rmm().address(),
        )
        .await?;

        Ok((arbitrageur, rmm_math))
    }
}

pub fn to_float(value: U256) -> f64 {
    format_ether(value).parse::<f64>().unwrap()
}

/// L_x(x, S) = x / (1 - cdf(ln(S/K) + sigma^2/2) / sigma)
#[tracing::instrument(ret, skip(instance), level = "trace")]
pub async fn compute_l_given_x_solidity(
    instance: &RMMMathLike<RevmMiddleware>,
    reserve_x: U256,
    spot_price: U256,
    strike_price: U256,
    sigma: U256,
    tau: U256,
) -> Result<(U256)> {
    let l = instance
        .compute_l_given_x(reserve_x, spot_price, strike_price, sigma, tau)
        .call()
        .await?;
    Ok(l)
}

#[allow(clippy::too_many_arguments)]
pub async fn compute_output_x_given_y_solidity(
    instance: &RMMMathLike<RevmMiddleware>,
    reserve_x: U256,
    reserve_y: U256,
    delta_y: U256,
    liquidity: U256,
    delta_l: U256,
    strike_price: U256,
    sigma: U256,
    tau: U256,
) -> Result<(I256)> {
    let x = instance
        .compute_output_x_given_y(
            reserve_x,
            reserve_y,
            delta_y,
            liquidity,
            delta_l,
            strike_price,
            sigma,
            tau,
        )
        .await?;
    Ok(x)
}

#[allow(clippy::too_many_arguments)]
pub async fn compute_output_y_given_x_solidity(
    instance: &RMMMathLike<RevmMiddleware>,
    reserve_x: U256,
    reserve_y: U256,
    delta_x: U256,
    liquidity: U256,
    delta_l: U256,
    strike_price: U256,
    sigma: U256,
    tau: U256,
) -> Result<(I256)> {
    let x = instance
        .compute_output_x_given_y(
            reserve_x,
            reserve_y,
            delta_x,
            liquidity,
            delta_l,
            strike_price,
            sigma,
            tau,
        )
        .await?;
    Ok(x)
}
