use std::sync::Arc;

use alloy_primitives::{
    utils::{format_ether, format_units, parse_ether},
    Address, U256,
};
use arbiter_bindings::bindings::{arbiter_token::ArbiterToken, liquid_exchange::LiquidExchange};
use arbiter_core::middleware::errors::RevmMiddlewareError;
use tracing::{debug, info, trace};

use super::{
    agents::base::token_admin::TokenAdmin,
    bindings::{atomic_arbitrage::AtomicArbitrage, dfmm::DFMM},
    Environment, Result, RevmMiddleware, *,
};

pub const WAD: U256 = U256::from_limbs([1_000_000_000_000_000_000, 0, 0, 0]);

#[derive(Debug, Clone)]
pub struct Arbitrageur {
    pub client: Arc<RevmMiddleware>,
    /// The arbitrageur's client connection to the liquid exchange.
    pub liquid_exchange: LiquidExchange<RevmMiddleware>,
    /// The target contract being arbitraged.
    pub target_exchange: DFMM<RevmMiddleware>,
    /// Arbitrage vehicle for atomically swapping between exchanges.
    pub atomic_arbitrage: AtomicArbitrage<RevmMiddleware>,
}

impl Arbitrageur {
    pub async fn new(
        environment: &Environment,
        token_admin: &TokenAdmin,
        liquid_exchange_address: Address,
        dfmm_address: Address,
    ) -> Result<Self> {
        // Create a client for the arbitrageur.
        let client = RevmMiddleware::new(environment, "arbitrageur".into())?;

        // Get the exchanges and arb contract connected to the arbitrageur client.
        let liquid_exchange =
            LiquidExchange::new(to_ethers_address(liquid_exchange_address), client.clone());

        let target_exchange = DFMM::deploy(
            client.clone(),
            (token_admin.arbx.address(), token_admin.arby.address()),
        )?
        .send()
        .await?;

        let atomic_arbitrage = AtomicArbitrage::deploy(
            client.clone(),
            (
                target_exchange.address(),
                liquid_exchange.address(),
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
                from_ethers_address(client.address()),
                parse_ether(100_000_000.to_string().as_str()).unwrap(),
                parse_ether(100_000_000.to_string().as_str()).unwrap(),
            )
            .await?;

        arbx.approve(atomic_arbitrage.address(), ethers::types::U256::MAX)
            .send()
            .await?
            .await?;
        arby.approve(atomic_arbitrage.address(), ethers::types::U256::MAX)
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

        println!("arbx_allowance: {:?}", arbx_allowance);
        println!("arby_allowance: {:?}", arby_allowance);

        Ok(Self {
            client,
            liquid_exchange,
            target_exchange,
            atomic_arbitrage,
        })
    }

    /// Detects if there is an arbitrage opportunity.
    /// Returns the direction of the swap `XtoY` or `YtoX` if there is an
    /// arbitrage opportunity. Returns `None` if there is no arbitrage
    /// opportunity.
    async fn detect_arbitrage(&self) -> Result<Swap> {
        // Update the prices the for the arbitrageur.
        let liquid_exchange_price_wad = self.liquid_exchange.price().call().await?;
        let liquid_exchange_price_wad = from_ethers_u256(liquid_exchange_price_wad);
        let price = U256::ZERO;
        debug!("liquid_exchange_price_wad: {:?}", liquid_exchange_price_wad);
        debug!("rmm_price_wad: {:?}", price);

        let gamma_wad = WAD;
        let upper_arb_bound = WAD * price / (WAD - gamma_wad);
        let lower_arb_bound = price * (WAD - gamma_wad) / WAD;

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
impl Agent for Arbitrageur {
    #[allow(unused)]
    async fn step(&mut self) -> Result<()> {
        // Detect if there is an arbitrage opportunity.
        let arbx = self.atomic_arbitrage.asset().call().await?;
        let arby = self.atomic_arbitrage.quote().call().await?;
        let arbx = ArbiterToken::new(arbx, self.client.clone());
        let arby = ArbiterToken::new(arby, self.client.clone());
        let arbx_balance = arbx.balance_of(self.client.address()).call().await?;
        let arby_balance = arby.balance_of(self.client.address()).call().await?;
        debug!("arbx_balance: {:?}", arbx_balance);
        debug!("arby_balance: {:?}", arby_balance);

        match self.detect_arbitrage().await? {
            Swap::RaiseExchangePrice(target_price) => {
                info!(
                    "Detected the need to raise price to {:?}",
                    format_units(target_price, "ether")?
                );
                let input = U256::ZERO;

                let tx = self
                    .atomic_arbitrage
                    .raise_exchange_price(to_ethers_u256(input));
                let output = tx.send().await;
                let arbx_balance = arbx.balance_of(self.client.address()).call().await?;
                let arby_balance = arby.balance_of(self.client.address()).call().await?;
                debug!("arbx_balance after: {:?}", arbx_balance);
                debug!("arby_balance after: {:?}", arby_balance);
                match output {
                    Ok(output) => {
                        output.await?;
                    }
                    Err(e) => {
                        if let RevmMiddlewareError::ExecutionRevert { gas_used, output } =
                            e.as_middleware_error().unwrap()
                        {
                            info!("Execution revert: {:?} Gas Used: {:?}", output, gas_used);
                        }
                    }
                }
            }
            Swap::LowerExchangePrice(target_price) => {
                info!(
                    "Detected the need to lower price to {:?}",
                    format_units(target_price, "ether")?
                );
                let input = U256::ZERO;
                info!("Got input: {:?}", input);
                if input.is_zero() {
                    return Ok(());
                }
                let tx = self
                    .atomic_arbitrage
                    .lower_exchange_price(to_ethers_u256(input));
                let output = tx.send().await;
                let arbx_balance = arbx.balance_of(self.client.address()).call().await?;
                let arby_balance = arby.balance_of(self.client.address()).call().await?;
                trace!("arbx_balance after: {:?}", arbx_balance);
                trace!("arby_balance after: {:?}", arby_balance);
                match output {
                    Ok(output) => {
                        output.await?;
                    }
                    Err(e) => {
                        if let RevmMiddlewareError::ExecutionRevert { gas_used, output } =
                            e.as_middleware_error().unwrap()
                        {
                            info!("Execution revert: {:?}", output);
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
mod tests {}

pub fn to_float(value: U256) -> f64 {
    format_ether(value).parse::<f64>().unwrap()
}
