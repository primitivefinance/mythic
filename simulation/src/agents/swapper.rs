use ethers::utils::{parse_units, ParseUnits};

use super::{block_admin::BlockAdmin, price_changer::PriceChanger, token_admin::TokenAdmin, *};
use crate::settings::parameters::LinspaceParameters;

#[derive(Clone, Debug)]
pub struct Swapper {
    pub client: Arc<RevmMiddleware>,
    pub liquid_exchange: LiquidExchange<RevmMiddleware>,
    pub input_token: Address,
    pub swap_times: Vec<U256>,
    pub amount_in: U256,
    pub swap_index: usize,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct SwapperParameters<P: Parameterized> {
    pub num_swaps: P,
    pub initial_balance: f64,
    /// True swaps X for Y
    pub swap_direction: bool,
    pub end_timestamp: f64,
}

impl From<SwapperParameters<Multiple>> for Vec<SwapperParameters<Single>> {
    fn from(item: SwapperParameters<Multiple>) -> Self {
        item.num_swaps
            .parameters()
            .into_iter()
            .map(|num_swaps| SwapperParameters {
                num_swaps: Single(num_swaps),
                initial_balance: item.initial_balance,
                swap_direction: item.swap_direction,
                end_timestamp: item.end_timestamp,
            })
            .collect()
    }
}

// TODO: Mint the amounts here.
impl Swapper {
    pub async fn new(
        environment: &Environment,
        config: &SimulationConfig<Single>,
        label: &str,
        price_changer: &PriceChanger,
    ) -> Result<Self> {
        let client = RevmMiddleware::new(environment, Some(label))?;
        let liquid_exchange =
            LiquidExchange::new(price_changer.liquid_exchange.address(), client.clone());

        if let Some(AgentParameters::Swapper(params)) = config.agent_parameters.get(label).cloned()
        {
            let initial_balance = parse_ether(params.initial_balance)?;
            debug!("Initial balance for swapper is: {}", initial_balance);
            let input_token = if params.swap_direction {
                liquid_exchange.arbiter_token_x().call().await?
            } else {
                liquid_exchange.arbiter_token_y().call().await?
            };
            let token = ArbiterToken::new(input_token, client.clone());
            token
                .approve(liquid_exchange.address(), initial_balance)
                .send()
                .await?
                .await?;

            let swap_times = linspace!(0_f64, params.end_timestamp, params.num_swaps.0 as usize);
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

            Ok(Self {
                client,
                liquid_exchange,
                input_token,
                swap_times,
                amount_in,
                swap_index: 0,
            })
        } else {
            Err(anyhow::anyhow!("No agent parameters found for {}", label))
        }
    }
}

#[async_trait::async_trait]
impl Agent for Swapper {
    async fn step(&mut self) -> Result<()> {
        debug!("Entering swapper step");
        if self.client.get_block_timestamp().await? >= self.swap_times[self.swap_index] {
            trace!("Swapper is swapping: {:?}", self.amount_in);
            self.liquid_exchange
                .swap(self.input_token, self.amount_in)
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
}
