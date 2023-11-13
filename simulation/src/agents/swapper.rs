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
    pub initial_balance: U256,
    /// True swaps X for Y
    pub swap_direction: bool,
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
            })
            .collect()
    }
}

// TODO: Mint the amounts here.
impl Swapper {
    async fn new(
        environment: &Environment,
        config: &SimulationConfig<Single>,
        label: &str,
        price_changer: &PriceChanger,
        token_admin: &TokenAdmin,
        liquid_exchange_address: Address,
    ) -> Result<Self> {
        let client = RevmMiddleware::new(environment, Some(label))?;
        let liquid_exchange = LiquidExchange::new(liquid_exchange_address, client.clone());

        if let Some(AgentParameters::Swapper(params)) = config.agent_parameters.get(label).cloned()
        {
            let input_token = if params.swap_direction {
                token_admin
                    .arbx
                    .mint(client.address(), params.initial_balance)
                    .send()
                    .await?
                    .await?;
                token_admin
                    .arbx
                    .approve(client.address(), params.initial_balance)
                    .send()
                    .await?
                    .await?;
                liquid_exchange.arbiter_token_x().call().await?
            } else {
                token_admin
                    .arby
                    .mint(client.address(), params.initial_balance)
                    .send()
                    .await?
                    .await?;
                token_admin
                    .arby
                    .approve(client.address(), params.initial_balance)
                    .send()
                    .await?
                    .await?;
                liquid_exchange.arbiter_token_y().call().await?
            };

            let swap_times = linspace!(
                price_changer.trajectory.times[0],
                price_changer.trajectory.times.last().unwrap().clone(),
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

            let amount_in = params.initial_balance / params.num_swaps.0 as u128;
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
        if self.client.get_block_timestamp().await? >= self.swap_times[self.swap_index] {
            self.liquid_exchange
                .swap(self.input_token, self.amount_in)
                .send()
                .await?
                .await?;
            self.swap_index += 1;
        }
        Ok(())
    }
}
