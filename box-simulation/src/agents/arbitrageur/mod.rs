use std::{f32::consts::E, ops::Div, sync::Arc};

use arbiter_core::{
    bindings::arbiter_token::ArbiterToken, middleware::errors::RevmMiddlewareError,
};
use bindings::{atomic_arbitrage::NotProfitable, g3m::G3MErrors, sd5_9x_18_math::SD59x18Math};
use ethers::{abi::AbiDecode, utils::format_units};
use tracing::info;

use super::*;

#[allow(clippy::all)]
pub mod atomic_arbitrage;
pub mod g3m;

#[derive(Clone)]
pub struct Arbitrageur<S: Strategy> {
    pub client: Arc<RevmMiddleware>,
    /// The arbitrageur's client connection to the liquid exchange.
    pub liquid_exchange: LiquidExchange<RevmMiddleware>,

    /// The strategy used by the exchange.
    pub strategy: S,

    /// The atomic arbitrage contract.
    pub atomic_arbitrage: AtomicArbitrage<RevmMiddleware>,

    pub math: SD59x18Math<RevmMiddleware>,
}

#[async_trait::async_trait]
pub trait Strategy {
    fn new(strategy_address: Address, client: Arc<RevmMiddleware>) -> Self;
    async fn get_x_input(
        &self,
        target_price_wad: U256,
        math: &SD59x18Math<RevmMiddleware>,
    ) -> Result<U256>;
    async fn get_y_input(
        &self,
        target_price_wad: U256,
        math: &SD59x18Math<RevmMiddleware>,
    ) -> Result<U256>;
    async fn get_spot_price(&self) -> Result<U256>;
    async fn swap_fee(&self) -> Result<U256>;
}

impl<S: Strategy> Arbitrageur<S> {
    pub async fn new(
        label: &str,
        environment: &Environment,
        liquid_exchange_address: Address,
        strategy_address: Address,
    ) -> Result<Self> {
        // Create a client for the arbitrageur.
        let client = RevmMiddleware::new(environment, Some(label))?;

        // Get the exchanges and arb contract connected to the arbitrageur client.
        let liquid_exchange = LiquidExchange::new(liquid_exchange_address, client.clone());
        let strategy = Strategy::new(strategy_address, client.clone());
        let arbx = liquid_exchange.arbiter_token_x().call().await?;
        let arby = liquid_exchange.arbiter_token_y().call().await?;
        let atomic_arbitrage = AtomicArbitrage::deploy(
            client.clone(),
            (strategy_address, liquid_exchange_address, arbx, arby),
        )?
        .send()
        .await?;

        let arbx = ArbiterToken::new(arbx, client.clone());
        let arby = ArbiterToken::new(arby, client.clone());
        arbx.approve(atomic_arbitrage.address(), U256::MAX)
            .send()
            .await?
            .await?;
        arby.approve(atomic_arbitrage.address(), U256::MAX)
            .send()
            .await?
            .await?;

        let math = SD59x18Math::deploy(client.clone(), ())?.send().await?;

        Ok(Self {
            client,
            liquid_exchange,
            atomic_arbitrage,
            strategy,
            math,
        })
    }

    #[allow(unused)]
    pub async fn step(&mut self) -> Result<()> {
        // Detect if there is an arbitrage opportunity.
        match self.detect_arbitrage().await? {
            Swap::RaiseExchangePrice(target_price) => {
                info!(
                    "Detected the need to raise price to {:?}",
                    format_units(target_price, "ether")?
                );
                let input = self.strategy.get_y_input(target_price, &self.math).await?;
                info!("Got input: {:?}", input);
                let tx = self.atomic_arbitrage.raise_exchange_price(input);
                let output = tx.send().await;
                match output {
                    Ok(output) => {
                        output.await?;
                    }
                    Err(e) => {
                        if let RevmMiddlewareError::ExecutionRevert { gas_used, output } =
                            e.as_middleware_error().unwrap()
                        {
                            info!("Execution revert: {:?}", output);
                            let NotProfitable {
                                first_swap_output,
                                second_swap_output,
                            } = NotProfitable::decode(output)?;
                            println!(
                                "first_swap_output: {:?}",
                                format_units(first_swap_output, "ether")?
                            );
                            println!(
                                "second_swap_output: {:?}",
                                format_units(second_swap_output, "ether")?
                            );
                        }
                    }
                }

                info!("Sent arbitrage.");
            }
            Swap::LowerExchangePrice(target_price) => {
                info!(
                    "Detected the need to lower price to {:?}",
                    format_units(target_price, "ether")?
                );
                let input = self.strategy.get_x_input(target_price, &self.math).await?;
                info!("Got input: {:?}", input);
                let tx = self.atomic_arbitrage.lower_exchange_price(input);
                let output = tx.send().await;
                match output {
                    Ok(output) => {
                        output.await?;
                    }
                    Err(e) => {
                        if let RevmMiddlewareError::ExecutionRevert { gas_used, output } =
                            e.as_middleware_error().unwrap()
                        {
                            info!("Execution revert: {:?}", output);
                            let NotProfitable {
                                first_swap_output,
                                second_swap_output,
                            } = NotProfitable::decode(output)?;
                            println!(
                                "first_swap_output: {:?}",
                                format_units(first_swap_output, "ether")?
                            );
                            println!(
                                "second_swap_output: {:?}",
                                format_units(second_swap_output, "ether")?
                            );
                            //     match G3MErrors::decode(output)? {
                            //         G3MErrors::InvalidSwap(message) => {
                            //             info!("Invalid swap: {:?}", message);
                            //         }
                            //         _ => {
                            //             info!("Unknown error: {:?}", output);
                            //         }
                            //     }
                        }
                    }
                }
                // println!("output: {:?}", output);

                // let logs = output.logs;
                // if let G3MErrors::InvalidSwap(message) = G3MErrors::decode(logs[0].clone().data)? {
                //     info!("Invalid swap: {:?}", message);
                // } else {
                //     println!("logs: {:?}", logs)
                // }
                info!("Sent arbitrage.");
            }
            Swap::None => {
                info!("No arbitrage opportunity");
            }
        }
        Ok(())
    }

    /// Detects if there is an arbitrage opportunity.
    /// Returns the direction of the swap `XtoY` or `YtoX` if there is an
    /// arbitrage opportunity. Returns `None` if there is no arbitrage
    /// opportunity.
    async fn detect_arbitrage(&mut self) -> Result<Swap> {
        // Update the prices the for the arbitrageur.
        let liquid_exchange_price_wad = self.liquid_exchange.price().call().await?;
        info!("liquid_exchange_price_wad: {:?}", liquid_exchange_price_wad);
        let g3m_price_wad = self.strategy.get_spot_price().await?;
        info!("g3m_price_wad: {:?}", g3m_price_wad);

        let gamma_wad = WAD - (self.strategy.swap_fee().await?) * U256::from(10u128.pow(14));
        info!("gamma_wad: {:?}", gamma_wad);

        // Compute the no-arbitrage bounds.
        let upper_arb_bound = WAD * g3m_price_wad / gamma_wad;
        info!("upper_arb_bound: {:?}", upper_arb_bound);
        let lower_arb_bound = g3m_price_wad * gamma_wad / WAD;
        info!("lower_arb_bound: {:?}", lower_arb_bound);

        // Check if we have an arbitrage opportunity by comparing against the bounds and
        // current price.
        // If these conditions are not satisfied, there cannot be a profitable
        // arbitrage. See: [An Analysis of Uniswap Markets](https://arxiv.org/pdf/1911.03380.pdf) Eq. 3, for example.
        if liquid_exchange_price_wad > upper_arb_bound && liquid_exchange_price_wad > g3m_price_wad
        {
            // Raise the portfolio price by selling asset for quote
            Ok(Swap::RaiseExchangePrice(liquid_exchange_price_wad))
        } else if liquid_exchange_price_wad < lower_arb_bound
            && liquid_exchange_price_wad < g3m_price_wad
        {
            // Lower the exchange price by selling asset for quote
            Ok(Swap::LowerExchangePrice(liquid_exchange_price_wad))
        } else {
            // Prices are within the no-arbitrage bounds, so we don't have an arbitrage.
            Ok(Swap::None)
        }
    }
}

enum Swap {
    RaiseExchangePrice(U256),
    LowerExchangePrice(U256),
    None,
}
