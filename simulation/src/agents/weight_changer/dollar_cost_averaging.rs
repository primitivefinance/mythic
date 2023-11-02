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
    pub initial_weight_x: P,
    pub fee_basis_points: P,
}

impl Into<Vec<DollarCostAveragingParameters<Single>>> for DollarCostAveragingParameters<Multiple> {
    fn into(self) -> Vec<DollarCostAveragingParameters<Single>> {
        self.start
            .parameters()
            .into_iter()
            .zip(self.end.parameters().into_iter())
            .zip(self.initial_weight_x.parameters().into_iter())
            .zip(self.fee_basis_points.parameters().into_iter())
            .map(|(((s, e), iwx), fbps)| DollarCostAveragingParameters {
                start: Single(s),
                end: Single(e),
                initial_weight_x: Single(iwx),
                fee_basis_points: Single(fbps),
                which_token: self.which_token,
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
