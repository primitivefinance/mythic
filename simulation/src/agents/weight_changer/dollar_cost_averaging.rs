use super::*;

#[derive(Clone)]
pub struct DollarCostAveragingStategist {
    pub client: Arc<RevmMiddleware>,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct DollarCostAveragingParameters<P: Parameterized> {
    pub start: P,
    pub end: P,
    pub which_token: bool,
}

impl From<DollarCostAveragingParameters<Multiple>> for Vec<DollarCostAveragingParameters<Single>> {
    fn from(item: DollarCostAveragingParameters<Multiple>) -> Self {
        item.start
            .parameters()
            .into_iter()
            .zip(item.end.parameters())
            .map(|(s, e)| DollarCostAveragingParameters {
                start: Single(s),
                end: Single(e),
                which_token: item.which_token,
            })
            .collect()
    }
}

impl DollarCostAveragingStategist {
    pub async fn new(
        environment: &Environment,
        config: &SimulationConfig<Single>,
        liquid_exchange_address: Address,
    ) -> Result<Self> {
        let client = RevmMiddleware::new(environment, "weight_changer".into())?;
        todo!()
    }
}

#[async_trait::async_trait]
impl WeightChanger for DollarCostAveragingStategist {
    async fn execute_smooth_rebalance(&mut self) -> Result<()> {
        todo!()
    }

    fn lex(&self) -> &LiquidExchange<RevmMiddleware> {
        todo!()
    }

    fn g3m(&self) -> &G3M<RevmMiddleware> {
        todo!()
    }
}

#[async_trait::async_trait]
impl Agent for DollarCostAveragingStategist {}
