use std::sync::Arc;

use alloy_primitives::{
    utils::{format_ether, format_units, parse_ether},
    Address, U256,
};
use arbiter_bindings::bindings::{arbiter_token::ArbiterToken, liquid_exchange::LiquidExchange};
use arbiter_core::middleware::errors::RevmMiddlewareError;
use clients::protocol::ProtocolClient;

use super::{
    agents::base::token_admin::TokenAdmin,
    bindings::{arb_math::ArbMath, atomic_v2::AtomicV2},
    Environment, Result, RevmMiddleware, *,
};

pub const WAD: U256 = U256::from_limbs([1_000_000_000_000_000_000, 0, 0, 0]);

#[derive(Debug, Clone)]
pub struct G3mArbitrageur {
    pub client: Arc<RevmMiddleware>,
    /// Connects the Arbitrageur agent to the DFMM protocol.
    pub protocol_client: ProtocolClient<RevmMiddleware>,
    /// The arbitrageur's client connection to the liquid exchange.
    pub liquid_exchange: LiquidExchange<RevmMiddleware>,
    /// Arbitrage vehicle for atomically swapping between exchanges.
    pub atomic_arbitrage: AtomicV2<RevmMiddleware>,
    // Arbiter math to do math!
    pub arb_math: ArbMath<RevmMiddleware>,
}

pub struct ArbInputs {
    pub i_wad: I256,
    pub target_price_wad: I256,
    pub wx: I256,
    pub wy: I256,
    pub gamma: I256,
    pub rx: I256,
    pub ry: I256,
    pub liq: I256,
}

// gradient ascent until you find the max arb profit
// implement the arb math
// start with doing root finding for the gradient
// from there add sophistication incrementally
// 1. we have arb profit function we can call f(x)
// 2. define fn g(x) = (f(x + 1) - f(x - 1)) / 2
impl G3mArbitrageur {
    pub async fn new(
        environment: &Environment,
        token_admin: &TokenAdmin,
        liquid_exchange_address: Address,
        protocol_client: ProtocolClient<RevmMiddleware>,
    ) -> Result<Self> {
        // Create a client for the arbitrageur.
        let client = RevmMiddleware::new(environment, "g3m_arbitrageur".into())?;
        let protocol_client = protocol_client.bind(client.clone()).await?;

        // Get the exchanges and arb contract connected to the arbitrageur client.
        let liquid_exchange =
            LiquidExchange::new(to_ethers_address(liquid_exchange_address), client.clone());

        // Deploy the arbitrageur's atomic contract to atomically swap between
        // exchanges.
        let atomic_arbitrage = AtomicV2::deploy(
            client.clone(),
            (
                protocol_client.g_solver.address(),
                protocol_client.protocol.address(),
                liquid_exchange.address(),
                token_admin.arbx.address(),
                token_admin.arby.address(),
            ),
        )?
        .send()
        .await?;

        let arb_math = ArbMath::deploy(client.clone(), ())?.send().await?;

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

        trace!("arbx_allowance: {:?}", arbx_allowance);
        trace!("arby_allowance: {:?}", arby_allowance);

        Ok(Self {
            client,
            protocol_client,
            liquid_exchange,
            atomic_arbitrage,
            arb_math,
        })
    }

    /// Detects if there is an arbitrage opportunity.
    /// Returns the direction of the swap `XtoY` or `YtoX` if there is an
    /// arbitrage opportunity. Returns `None` if there is no arbitrage
    /// opportunity.
    #[tracing::instrument(skip(self), level = "trace", ret)]
    async fn detect_arbitrage(&self) -> Result<Swap> {
        // Update the prices the for the arbitrageur.
        let liquid_exchange_price_wad = self.liquid_exchange.price().call().await?;
        let liquid_exchange_price_wad = from_ethers_u256(liquid_exchange_price_wad);

        let target_exchange_price_wad = self
            .protocol_client
            .g_solver
            .internal_price(ethers::types::U256::from(1))
            .call()
            .await?;
        let target_exchange_price_wad = from_ethers_u256(target_exchange_price_wad);
        debug!("=== Start Loop ===");
        // info!("Price[LEX]: {:?}", format_ether(liquid_exchange_price_wad));
        // info!("Price[G3M]: {:?}", format_ether(target_exchange_price_wad));

        match liquid_exchange_price_wad {
            _ if liquid_exchange_price_wad > target_exchange_price_wad => {
                // Raise the portfolio price by selling asset for quote
                Ok(Swap::RaiseExchangePrice(liquid_exchange_price_wad))
            }
            _ if liquid_exchange_price_wad < target_exchange_price_wad => {
                // Lower the exchange price by selling asset for quote
                Ok(Swap::LowerExchangePrice(liquid_exchange_price_wad))
            }
            _ => {
                // Prices are within the no-arbitrage bounds, so we don't have an arbitrage.
                Ok(Swap::None)
            }
        }
    }

    pub async fn get_arb_inputs_as_i256(&self) -> Result<ArbInputs> {
        let i_wad = I256::from_raw(ethers::utils::parse_ether("1")?);
        let target_price_wad = I256::from_raw(self.liquid_exchange.price().call().await?);
        let (wx, wy) = self
            .protocol_client
            .g_strategy
            .get_params(ethers::types::U256::from(1))
            .call()
            .await?;
        let (wx, wy) = (I256::from_raw(wx), I256::from_raw(wy));
        let gamma = I256::from_raw(ethers::utils::parse_ether("1")?)
            - I256::from_raw(self.protocol_client.g_strategy.swap_fee().call().await?);
        let (rx, ry, liq) = self
            .protocol_client
            .protocol
            .get_reserves_and_liquidity(ethers::types::U256::from(1))
            .call()
            .await?;
        let (rx, ry, liq) = (I256::from_raw(rx), I256::from_raw(ry), I256::from_raw(liq));
        Ok(ArbInputs {
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
        let ArbInputs {
            i_wad,
            target_price_wad,
            wx,
            wy,
            gamma,
            rx,
            ry: _,
            liq,
        } = self.get_arb_inputs_as_i256().await?;

        let ratio = wx * i_wad / wy;

        let inside = ratio * i_wad / target_price_wad;
        let delta_x =
            (liq * self.arb_math.pow(inside, wy).call().await? / i_wad - rx) * (i_wad / gamma);

        Ok(delta_x)
    }

    // todo (matt): figure out why this returns u256::max sometimes
    pub async fn get_dy(&self) -> Result<I256> {
        let ArbInputs {
            i_wad,
            target_price_wad,
            wx,
            wy,
            gamma,
            rx: _,
            ry,
            liq,
        } = self.get_arb_inputs_as_i256().await?;
        let ratio = wy * i_wad / wx;

        let inside = ratio * target_price_wad / i_wad;
        let delta_y =
            (liq * self.arb_math.pow(inside, wx).call().await? / i_wad - ry) * (i_wad / gamma);

        Ok(delta_y)
    }
}
// TODO: make sure we're swapping on low and high vol strategies
#[async_trait::async_trait]
impl Agent for G3mArbitrageur {
    #[allow(unused)]
    async fn step(&mut self) -> Result<()> {
        // Detect if there is an arbitrage opportunity.
        let arbx = self.atomic_arbitrage.asset().call().await?;
        let arby = self.atomic_arbitrage.quote().call().await?;
        let arbx = ArbiterToken::new(arbx, self.client.clone());
        let arby = ArbiterToken::new(arby, self.client.clone());
        let arbx_balance = arbx.balance_of(self.client.address()).call().await?;
        let arby_balance = arby.balance_of(self.client.address()).call().await?;

        match self.detect_arbitrage().await? {
            Swap::RaiseExchangePrice(target_price) => {
                // info!(
                //     "Signal[RAISE PRICE]: {:?}",
                //     format_units(target_price, "ether")?
                // );
                let x_in = false;
                let mut input = self.get_dy().await?;
                if (input < ethers::types::I256::from(0)) {
                    input = ethers::types::I256::from(0);
                }
                let input = input.into_raw();

                debug!("Optimal y input: {:?}", input);

                let tx = self
                    .atomic_arbitrage
                    .raise_exchange_price(ethers::types::U256::from(1), input);

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
                            // info!("Execution revert: {:?} Gas Used: {:?}",
                            // output, gas_used);
                        }
                    }
                }

                let internal_price = self
                    .protocol_client
                    .get_g_internal_price(ethers::types::U256::from(1))
                    .await?;
                let internal_price = from_ethers_u256(internal_price);
                // info!("Price[LEX]: {:?}", format_ether(target_price));
                // info!("Price[G3M]: {:?}", format_ether(internal_price));
                debug!("=== End Loop ===");
            }
            Swap::LowerExchangePrice(target_price) => {
                // info!(
                //     "Signal[LOWER PRICE] {:?}",
                //     format_units(target_price, "ether")?
                // );

                let x_in = true;
                let liquid_exchange_price = self.liquid_exchange.price().call().await?;
                let mut input = self.get_dx().await?;
                if (input < ethers::types::I256::from(0)) {
                    input = ethers::types::I256::from(0);
                }
                let input = input.into_raw();
                let input = input * liquid_exchange_price / ethers::utils::parse_ether("1")?;
                debug!("Optimal x input: {:?}", input);

                let tx = self
                    .atomic_arbitrage
                    .lower_exchange_price(ethers::types::U256::from(1), input);
                let output = tx.send().await;

                let arbx_balance = arbx.balance_of(self.client.address()).call().await?;
                let arby_balance = arby.balance_of(self.client.address()).call().await?;
                let (reserve_x, reserve_y, liquidity) = self
                    .protocol_client
                    .protocol
                    .get_reserves_and_liquidity(ethers::types::U256::from(1))
                    .call()
                    .await?;
                let (wx, wy) = self
                    .protocol_client
                    .g_strategy
                    .get_params(ethers::types::U256::from(1))
                    .call()
                    .await?;
                debug!("====Params[POST-SWAP] wx: {:?} wy: {:?}", wx, wy);
                debug!(
                    "====Reserves[POST-SWAP] reserve_x: {:?} reserve_y: {:?} liquidity: {:?}=====",
                    reserve_x, reserve_y, liquidity
                );

                match output {
                    Ok(output) => {
                        output.await?;
                    }
                    Err(e) => {
                        if let RevmMiddlewareError::ExecutionRevert { gas_used, output } =
                            e.as_middleware_error().unwrap()
                        {
                            // info!("Execution revert: {:?} Gas Used: {:?}",
                            // output, gas_used);
                        }
                    }
                }
                trace!("Sent arbitrage.");

                let internal_price = self
                    .protocol_client
                    .get_g_internal_price(ethers::types::U256::from(1))
                    .await?;
                let internal_price = from_ethers_u256(internal_price);
                debug!("Price[LEX]: {:?}", format_ether(target_price));
                debug!("Price[G3M]: {:?}", format_ether(internal_price));
                debug!("=== End Loop ===");
            }
            Swap::None => {
                trace!("No arbitrage opportunity");
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

#[derive(Debug, Clone)]
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
