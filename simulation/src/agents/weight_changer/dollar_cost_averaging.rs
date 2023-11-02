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

impl DollarCostAveragingStategist {
    pub async fn new(
        environment: &Environment,
        config: &SimulationConfig<Single>,
        liquid_exchange_address: Address,
        arbx: Address,
        arby: Address,
    ) -> Result<Self> {
        let client = RevmMiddleware::new(environment, "weight_changer".into())?;
        todo!()
    }
}
