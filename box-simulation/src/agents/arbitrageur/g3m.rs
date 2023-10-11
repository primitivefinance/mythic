use super::*;

#[async_trait::async_trait]
impl Strategy for G3M<RevmMiddleware> {
    fn new(strategy_address: Address, client: Arc<RevmMiddleware>) -> Self {
        Self::new(strategy_address, client)
    }
    async fn get_x_input(&self, target_price_wad: U256) -> Result<U256> {
        let weight_x = self.weight_x().call().await?;
        let weight_y = self.weight_y().call().await?;
        let reserve_y = self.reserve_y().call().await?;
        let invariant = self.get_invariant().call().await?;

        // Ok(weight_y
        //     * U256::from(1)
        //         .div(target_price_wad * invariant.pow(U256::from(1).div(weight_x)))
        //         .pow(U256::from(1) + weight_y.div(weight_x))
        //     - reserve_y)
        Ok(U256::from(1_000_000_000))
    }

    async fn get_y_input(&self, target_price_wad: U256) -> Result<U256> {
        let weight_x = self.weight_x().call().await?;
        let weight_y = self.weight_y().call().await?;
        let reserve_x = self.reserve_x().call().await?;
        let invariant = self.get_invariant().call().await?;

        // Ok(weight_x
        //     * target_price_wad
        //         .div(invariant.pow(U256::from(1).div(weight_y)))
        //         .pow(U256::from(1) + weight_x.div(weight_y))
        //     - reserve_x)
        Ok(U256::from(1_000_000_000))
    }

    async fn get_spot_price(&self) -> Result<U256> {
        Ok(self.get_spot_price().call().await?)
    }

    async fn swap_fee(&self) -> Result<U256> {
        Ok(self.swap_fee().call().await?)
    }
}
