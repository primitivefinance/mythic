use super::*;
use crate::settings::parameters::LinspaceParameters;

#[derive(Clone, Debug)]
pub struct Swapper {
    pub client: Arc<RevmMiddleware>,
    pub liquid_exchange: LiquidExchange<RevmMiddleware>,
    pub input_token: Address,
    pub swap_times: Vec<U256>,
    pub amount_in: U256,
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
        liquid_exchange_address: Address,
    ) -> Result<Self> {
        todo!()
        // let parameters = config.swapper.unwrap();
        // let client = RevmMiddleware::new(&environment,
        // "time_based_swapper".into()).unwrap(); let liquid_exchange =
        // LiquidExchange::new(liquid_exchange_address, client.clone());
        // let input_token = if parameters.swap_direction {
        //     liquid_exchange.arbiter_token_x().call().await?
        // } else {
        //     liquid_exchange.arbiter_token_x().call().await?
        // };
        // let swap_times = linspace!(
        //     config.trajectory.t_0.0,
        //     config.trajectory.t_n.0,
        //     config.trajectory.num_steps
        // );
        // let amount_in = parameters.initial_balance / parameters.num_swaps;
        // Ok(Self {
        //     client,
        //     liquid_exchange,
        //     input_token,
        //     swap_times,
        //     amount_in,
        // })
    }
}

#[async_trait::async_trait]
impl Agent for Swapper {
    fn as_any(&self) -> &dyn Any {
        self
    }
    // async fn step(&mut self) -> Result<()> {
    //     if self.client.get_block_timestamp().await? == self.next_swap_time {
    //         self.liquid_exchange.swap(self.input_token, self.amount_in)
    //     }
    //     Ok(())
    // }

    // async fn startup(&mut self) -> Result<()> {
    //     self.0.startup().await
    // }
}
