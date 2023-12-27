use std::sync::Arc;

use alloy_primitives::{
    utils::{format_ether, format_units, parse_ether},
    Address, U256,
};
use arbiter_bindings::bindings::{arbiter_token::ArbiterToken, liquid_exchange::LiquidExchange};
use arbiter_core::middleware::errors::RevmMiddlewareError;
use clients::protocol::ProtocolClient;

use super::{
    agents::base::token_admin::TokenAdmin, bindings::atomic_v2::AtomicV2, Environment, Result,
    RevmMiddleware, *,
};

pub const WAD: U256 = U256::from_limbs([1_000_000_000_000_000_000, 0, 0, 0]);

#[derive(Debug, Clone)]
pub struct Arbitrageur {
    pub client: Arc<RevmMiddleware>,
    /// Connects the Arbitrageur agent to the DFMM protocol.
    pub protocol_client: ProtocolClient<RevmMiddleware>,
    /// The arbitrageur's client connection to the liquid exchange.
    pub liquid_exchange: LiquidExchange<RevmMiddleware>,
    /// Arbitrage vehicle for atomically swapping between exchanges.
    pub atomic_arbitrage: AtomicV2<RevmMiddleware>,
}

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

// gradient ascent until you find the max arb profit
// implement the arb math
// start with doing root finding for the gradient
// from there add sophistication incrementally
// 1. we have arb profit function we can call f(x)
// 2. define fn g(x) = (f(x + 1) - f(x - 1)) / 2
impl Arbitrageur {
    pub async fn new(
        environment: &Environment,
        token_admin: &TokenAdmin,
        liquid_exchange_address: Address,
        dfmm_address: Address,
        solver_address: Address,
    ) -> Result<Self> {
        // Create a client for the arbitrageur.
        let client = RevmMiddleware::new(environment, "arbitrageur".into())?;

        // Create the protocol client.
        let protocol_client = ProtocolClient::new(
            client.clone(),
            to_ethers_address(dfmm_address),
            to_ethers_address(solver_address),
        );

        // Get the exchanges and arb contract connected to the arbitrageur client.
        let liquid_exchange =
            LiquidExchange::new(to_ethers_address(liquid_exchange_address), client.clone());

        // Deploy the arbitrageur's atomic contract to atomically swap between
        // exchanges.
        let atomic_arbitrage = AtomicV2::deploy(
            client.clone(),
            (
                protocol_client.solver.address(),
                protocol_client.protocol.address(),
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

        trace!("arbx_allowance: {:?}", arbx_allowance);
        trace!("arby_allowance: {:?}", arby_allowance);

        Ok(Self {
            client,
            protocol_client,
            liquid_exchange,
            atomic_arbitrage,
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

        let target_exchange_price_wad = self.protocol_client.get_internal_price().await?;
        let target_exchange_price_wad = from_ethers_u256(target_exchange_price_wad);
        debug!("=== Start Loop ===");
        info!("Price[LEX]: {:?}", format_ether(liquid_exchange_price_wad));
        info!("Price[DEX]: {:?}", format_ether(target_exchange_price_wad));

        if liquid_exchange_price_wad > target_exchange_price_wad {
            // Raise the portfolio price by selling asset for quote
            Ok(Swap::RaiseExchangePrice(liquid_exchange_price_wad))
        } else if liquid_exchange_price_wad < target_exchange_price_wad {
            // Lower the exchange price by selling asset for quote
            Ok(Swap::LowerExchangePrice(liquid_exchange_price_wad))
        } else {
            // Prices are within the no-arbitrage bounds, so we don't have an arbitrage.
            Ok(Swap::None)
        }
    }

    pub async fn get_arb_inputs_as_i256(&self) -> Result<ArbInputs> {
        let log_normal = self.protocol_client.get_strategy().await?;
        let i_wad = I256::from_raw(ethers::utils::parse_ether("1")?);
        let target_price_wad = I256::from_raw(self.liquid_exchange.price().call().await?);
        let (strike, sigma, tau) = log_normal.get_params().call().await?;
        let (strike, sigma, tau) = (
            I256::from_raw(strike),
            I256::from_raw(sigma),
            I256::from_raw(tau),
        );
        let gamma = I256::from_raw(ethers::utils::parse_ether("1")?)
            - I256::from_raw(log_normal.swap_fee().call().await?);
        let (rx, ry, liq) = self
            .protocol_client
            .protocol
            .get_reserves_and_liquidity()
            .call()
            .await?;
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
        } = self.get_arb_inputs_as_i256().await?;

        let log_p = self
            .atomic_arbitrage
            .log(target_price_wad * i_wad / strike)
            .call()
            .await?;
        let inner_p = log_p * i_wad / sigma + (sigma / 2);
        let cdf_p = self.atomic_arbitrage.cdf(inner_p).call().await?;
        let delta = liq * (i_wad - cdf_p) / i_wad;
        let dx = (delta - rx) * i_wad * i_wad
            / (((gamma - i_wad) * (i_wad - cdf_p)) / (rx * i_wad / liq) + i_wad);
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
        } = self.get_arb_inputs_as_i256().await?;

        let log_p = self
            .atomic_arbitrage
            .log(target_price_wad * i_wad / strike)
            .call()
            .await?;
        let inner_p = log_p * i_wad / sigma - (sigma / 2);
        let cdf_p = self.atomic_arbitrage.cdf(inner_p).call().await?;
        let delta = (liq * strike) / i_wad * (cdf_p) / i_wad;
        let dy = (delta - ry) * i_wad * i_wad
            / (((gamma - i_wad) * cdf_p) / (ry * i_wad * i_wad / (strike * liq)) + i_wad);

        Ok(dy / i_wad)
    }

    pub async fn compute_max_profit_trade(
        &self,
        x_in: bool,
        initial_guess_in: ethers::types::U256,
    ) -> Result<ethers::types::U256, Error> {
        let (computed_amount_in, computed_profit, _expected_price) = self
            .atomic_arbitrage
            .search_max_arb_profit(initial_guess_in, x_in)
            .call()
            .await?;
        debug!("computed amount in: {:?}", computed_amount_in);
        debug!("computed profit: {:?}", computed_profit);
        Ok(computed_amount_in)
    }

    pub async fn compute_g_x(
        &self,
        x_in: bool,
        amount_in: ethers::types::U256,
        lex_price: ethers::types::U256,
    ) -> Result<I256> {
        let i_wad = I256::from_raw(ethers::utils::parse_ether("1")?);
        let amount_in_up = amount_in + ethers::types::U256::from(1000);
        let amount_in_down = amount_in - ethers::types::U256::from(1000);
        let profit_up = self
            .compute_swap_profit(x_in, amount_in_up, lex_price)
            .await?;
        let profit_down = self
            .compute_swap_profit(x_in, amount_in_down, lex_price)
            .await?;
        debug!("amount in: {amount_in}");
        debug!("profit up: {profit_up}");
        debug!("profit down: {profit_down}");
        debug!("profit up minus profit down: {:?}", profit_up - profit_down);

        Ok((profit_up - profit_down) * i_wad / I256::from_raw(ethers::types::U256::from(200)))
    }

    pub async fn compute_swap_profit(
        &self,
        x_in: bool,
        amount_in: ethers::types::U256,
        lex_price: ethers::types::U256,
    ) -> Result<I256> {
        let (valid, amount_out, _price, _payload) = self
            .atomic_arbitrage
            .simulate_swap(x_in, amount_in)
            .call()
            .await?;
        if !valid {
            error!("=====valid swap: {valid}=====");
        }
        self.calculate_profit(x_in, amount_in, amount_out, lex_price)
    }

    pub fn calculate_profit(
        &self,
        x_in: bool,
        amount_in: ethers::types::U256,
        amount_out: ethers::types::U256,
        lex_price: ethers::types::U256,
    ) -> Result<I256> {
        let u_wad = ethers::utils::parse_ether("1")?;
        let out_value: I256;
        let in_value: I256;
        if x_in {
            out_value = I256::from_raw(amount_out);
            in_value = I256::from_raw(amount_in * lex_price / u_wad);
            Ok(out_value - in_value)
        } else {
            out_value = I256::from_raw(amount_out * lex_price / u_wad);
            in_value = I256::from_raw(amount_in);
            Ok(out_value - in_value)
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
        info!("arby_balance before: {:?}", arby_balance);

        match self.detect_arbitrage().await? {
            Swap::RaiseExchangePrice(target_price) => {
                info!(
                    "Signal[RAISE PRICE]: {:?}",
                    format_units(target_price, "ether")?
                );
                let x_in = false;
                let input = self.get_dy().await?.into_raw();

                debug!("Optimal y input: {:?}", input);

                let tx = self.atomic_arbitrage.raise_exchange_price(input);
                let output = tx.send().await;
                let arbx_balance = arbx.balance_of(self.client.address()).call().await?;
                let arby_balance = arby.balance_of(self.client.address()).call().await?;
                debug!("arby_balance after: {:?}", arby_balance);
                let (reserve_x, reserve_y, liquidity) = self
                    .protocol_client
                    .protocol
                    .get_reserves_and_liquidity()
                    .call()
                    .await?;
                debug!(
                    "reserve_x: {:?} reserve_y: {:?} liquidity: {:?}",
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
                            info!("Execution revert: {:?} Gas Used: {:?}", output, gas_used);
                        }
                    }
                }

                let internal_price = self.protocol_client.get_internal_price().await?;
                let internal_price = from_ethers_u256(internal_price);
                info!("Price[LEX]: {:?}", format_ether(target_price));
                info!("Price[DEX]: {:?}", format_ether(internal_price));
                debug!("=== End Loop ===");
            }
            Swap::LowerExchangePrice(target_price) => {
                info!(
                    "Signal[LOWER PRICE] {:?}",
                    format_units(target_price, "ether")?
                );

                let x_in = true;
                let liquid_exchange_price = self.liquid_exchange.price().call().await?;
                let input = self.get_dx().await?.into_raw();
                debug!("Initial Guess x in: {:?}", input);

                // let input = self
                //     .compute_max_profit_trade(x_in, initial_guess_in)
                //     .await?;
                let input = input * liquid_exchange_price / ethers::utils::parse_ether("1")?;

                debug!("Optimal x input: {:?}", input);

                let tx = self.atomic_arbitrage.lower_exchange_price(input);
                let output = tx.send().await;

                let arbx_balance = arbx.balance_of(self.client.address()).call().await?;
                let arby_balance = arby.balance_of(self.client.address()).call().await?;
                let (reserve_x, reserve_y, liquidity) = self
                    .protocol_client
                    .protocol
                    .get_reserves_and_liquidity()
                    .call()
                    .await?;
                trace!("arby_balance after: {:?}", arby_balance);
                trace!(
                    "reserve_x: {:?} reserve_y: {:?} liquidity: {:?}",
                    reserve_x,
                    reserve_y,
                    liquidity
                );

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
                trace!("Sent arbitrage.");

                let internal_price = self.protocol_client.get_internal_price().await?;
                let internal_price = from_ethers_u256(internal_price);
                debug!("Price[LEX]: {:?}", format_ether(target_price));
                debug!("Price[DEX]: {:?}", format_ether(internal_price));
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
