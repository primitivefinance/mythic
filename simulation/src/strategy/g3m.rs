use ethers::types::TransactionReceipt;

use super::*;

#[async_trait::async_trait]
impl Strategy for G3M<RevmMiddleware> {
    fn new(strategy_address: Address, client: Arc<RevmMiddleware>) -> Self {
        Self::new(strategy_address, client)
    }
    async fn get_x_input(
        &self,
        target_price_wad: U256,
        math: &SD59x18Math<RevmMiddleware>,
    ) -> Result<U256> {
        let weight_x = I256::from_raw(self.weight_x().call().await?);
        trace!("weight_x: {}", weight_x);
        let weight_y = I256::from_raw(self.weight_y().call().await?);
        trace!("weight_y: {}", weight_y);
        let reserve_x = I256::from_raw(self.reserve_x_without_precision().call().await?);
        trace!("reserve_x: {}", reserve_x);
        let reserve_y = I256::from_raw(self.reserve_y_without_precision().call().await?);
        trace!("reserve_y: {}", reserve_y);
        let invariant = I256::from_raw(self.get_invariant().call().await?);
        trace!("invariant: {}", invariant);

        let iwad = I256::from_raw(WAD);

        let ratio = weight_x * iwad / weight_y;
        trace!("ratio: {}", ratio);

        let inside = ratio * iwad / I256::from_raw(target_price_wad);
        trace!("inside: {}", inside);
        let delta_x = invariant * math.pow(inside, weight_y).call().await? / iwad - reserve_x;
        trace!("delta_x: {}", delta_x);

        Ok(delta_x.into_raw())
    }

    async fn get_y_input(
        &self,
        target_price_wad: U256,
        math: &SD59x18Math<RevmMiddleware>,
    ) -> Result<U256> {
        let weight_x = I256::from_raw(self.weight_x().call().await?);
        trace!("weight_x: {}", weight_x);
        let weight_y = I256::from_raw(self.weight_y().call().await?);
        trace!("weight_y: {}", weight_y);
        let reserve_x = I256::from_raw(self.reserve_x_without_precision().call().await?);
        trace!("reserve_x: {}", reserve_x);
        let reserve_y = I256::from_raw(self.reserve_y_without_precision().call().await?);
        trace!("reserve_y: {}", reserve_y);
        let invariant = I256::from_raw(self.get_invariant().call().await?);
        trace!("invariant: {}", invariant);

        let iwad = I256::from_raw(WAD);

        let ratio = weight_y * iwad / weight_x;
        trace!("ratio: {}", ratio);

        let inside = ratio * I256::from_raw(target_price_wad) / iwad;
        trace!("inside: {}", inside);
        let delta_y = invariant * math.pow(inside, weight_x).call().await? / iwad - reserve_y;
        trace!("delta_y: {}", delta_y);

        Ok(delta_y.into_raw())
    }

    async fn get_spot_price(&self) -> Result<U256> {
        Ok(self.get_spot_price().call().await?)
    }

    async fn swap_fee(&self) -> Result<U256> {
        Ok(self.swap_fee().call().await?)
    }

    async fn init_pool_with_x(&self, amount_x: U256) -> Result<Option<TransactionReceipt>> {
        Ok(self
            .add_liquidity_with_exact_x(true, amount_x)
            .send()
            .await?
            .await?)
    }
}
