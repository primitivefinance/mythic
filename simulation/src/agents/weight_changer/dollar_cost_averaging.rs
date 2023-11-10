use super::*;

#[derive(Clone)]
pub struct DollarCostAveragingStategist {
    pub client: Arc<RevmMiddleware>,
    pub g3m: G3M<RevmMiddleware>,
    pub end_weight: f64,
    pub end_timestamp: u64,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct DollarCostAveragingParameters {
    pub end_weight: f64,
    pub end_timestamp: u64,
}

#[async_trait::async_trait]
impl WeightChanger for DollarCostAveragingStategist {
    async fn execute_smooth_rebalance(&mut self) -> Result<()> {
        todo!()
    }

    fn g3m(&self) -> &G3M<RevmMiddleware> {
        &self.g3m
    }
}

#[async_trait::async_trait]
impl Agent for DollarCostAveragingStategist {
    fn as_any(&self) -> &dyn Any {
        self
    }

    async fn step(&mut self) -> Result<()> {
        Ok(())
    }

    async fn startup(&mut self) -> Result<()> {
        debug!("Entered startup for `DollarCostAveragingStategist`");
        self.g3m()
            .set_weight_x(
                parse_ether(self.end_weight)?,
                U256::from(self.end_timestamp),
            )
            .send()
            .await?
            .await?;
        debug!("Finished startup for `DollarCostAveragingStategist`");
        Ok(())
    }
}
