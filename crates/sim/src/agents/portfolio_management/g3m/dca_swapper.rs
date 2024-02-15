use std::sync::Arc;

use arbiter_bindings::bindings::{arbiter_token::ArbiterToken, liquid_exchange::LiquidExchange};
use ethers::utils::{parse_ether, parse_units, ParseUnits};

use super::{
    agent::*, agents::base::token_admin::TokenAdmin, bindings::portfolio_tracker::PortfolioTracker,
    Environment, Result, RevmMiddleware, *,
};
use crate::settings::parameters::LinspaceParameters;

#[derive(Clone, Debug)]
pub struct DcaSwapper {
    pub client: Arc<RevmMiddleware>,
    pub liquid_exchange: LiquidExchange<RevmMiddleware>,
    pub portfolio_tracker: PortfolioTracker<RevmMiddleware>,
    pub input_token_address: Address,
    pub output_token_address: Address,
    pub swap_times: Vec<U256>,
    pub amount_in: U256,
    pub swap_index: usize,
    pub swap_direction: bool,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct DcaSwapperParameters<P: Parameterized> {
    pub num_swaps: P,
    pub initial_balance: f64,
    /// True swaps X for Y
    pub swap_direction: bool,
    pub end_timestamp: f64,
    pub start_timestamp: f64,
}

impl From<DcaSwapperParameters<Multiple>> for Vec<DcaSwapperParameters<Single>> {
    fn from(item: DcaSwapperParameters<Multiple>) -> Self {
        item.num_swaps
            .parameters()
            .into_iter()
            .map(|num_swaps| DcaSwapperParameters {
                num_swaps: Single(num_swaps),
                initial_balance: item.initial_balance,
                swap_direction: item.swap_direction,
                end_timestamp: item.end_timestamp,
                start_timestamp: item.start_timestamp,
            })
            .collect()
    }
}

// TODO: Mint the amounts here.
impl DcaSwapper {
    pub async fn new(
        environment: &Environment,
        config: &SimulationConfig<Single>,
        label: &str,
        liquid_exchange_address: Address,
        token_admin: &TokenAdmin,
    ) -> Result<Self> {
        let client = RevmMiddleware::new(environment, Some(label))?;

        let liquid_exchange = LiquidExchange::new(liquid_exchange_address, client.clone());

        if let Some(AgentParameters::DcaSwapper(params)) =
            config.agent_parameters.get(label).cloned()
        {
            let initial_balance = parse_ether(params.initial_balance)?;
            debug!("Initial balance for swapper is: {}", initial_balance);
            let (input_token_address, output_token_address) = if params.swap_direction {
                token_admin
                    .arbx
                    .mint(client.clone().address(), initial_balance)
                    .send()
                    .await?
                    .await?;
                (
                    liquid_exchange.arbiter_token_x().call().await?,
                    liquid_exchange.arbiter_token_y().call().await?,
                )
            } else {
                token_admin
                    .arby
                    .mint(client.clone().address(), initial_balance)
                    .send()
                    .await?
                    .await?;
                (
                    liquid_exchange.arbiter_token_y().call().await?,
                    liquid_exchange.arbiter_token_x().call().await?,
                )
            };

            let input_token = ArbiterToken::new(input_token_address, client.clone());
            input_token
                .approve(liquid_exchange.address(), MAX)
                .send()
                .await?
                .await?;

            let swap_times = linspace!(
                params.start_timestamp,
                params.end_timestamp,
                params.num_swaps.0 as usize
            );
            trace!(
                "Swapper will be tendering token with address: {:?}",
                input_token
            );

            let swap_times = swap_times
                .iter()
                .map(|time| parse_units(time.to_string(), "wei").unwrap())
                .map(|unit| {
                    if let ParseUnits::U256(amount) = unit {
                        amount
                    } else {
                        unreachable!()
                    }
                })
                .collect();
            trace!("Swap times for the swapper are: {:?}", swap_times);

            let amount_in = initial_balance / params.num_swaps.0 as u128;
            trace!(
                "Amount in for each of the swapper's swaps is: {}",
                amount_in
            );
            let portfolio_tracker = PortfolioTracker::deploy(client.clone(), ())?.send().await?;
            Ok(Self {
                client,
                liquid_exchange,
                input_token_address,
                output_token_address,
                swap_times,
                amount_in,
                swap_index: 0,
                portfolio_tracker,
                swap_direction: params.swap_direction,
            })
        } else {
            Err(anyhow::anyhow!("No agent parameters found for {}", label))
        }
    }
}

#[async_trait::async_trait]
impl Agent for DcaSwapper {
    async fn step(&mut self) -> Result<()> {
        debug!("Entering swapper step");
        if self.swap_direction {
            self.portfolio_tracker
                .log_portfolio(self.input_token_address, self.output_token_address)
                .send()
                .await?
                .await?;
        } else {
            self.portfolio_tracker
                .log_portfolio(self.output_token_address, self.input_token_address)
                .send()
                .await?
                .await?;
        }
        if self.swap_index >= self.swap_times.len() - 1
            && self.client.get_block_timestamp().await? >= self.swap_times[self.swap_index]
        {
            // Make sure we can't go past the last time
            return Ok(());
        }
        if self.client.get_block_timestamp().await? >= self.swap_times[self.swap_index] {
            trace!("Swapper is swapping: {:?}", self.amount_in);

            self.liquid_exchange
                .swap(self.input_token_address, self.amount_in)
                .send()
                .await?
                .await?;
            trace!("Swapper swapped and is incrementing index");
            self.swap_index += 1;
        }
        debug!("Exiting swapper step");
        Ok(())
    }
    fn client(&self) -> Arc<RevmMiddleware> {
        self.client.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
