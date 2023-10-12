use arbiter_core::bindings::arbiter_math;
use ethers::{types::I256, utils::format_units};

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
        info!("weight_x: {}", weight_x);
        let weight_y = I256::from_raw(self.weight_y().call().await?);
        info!("weight_y: {}", weight_y);
        let reserve_x = I256::from_raw(self.reserve_x().call().await?);
        info!("reserve_x: {}", reserve_x);
        let invariant = I256::from_raw(self.get_invariant().call().await?);
        info!("invariant: {}", invariant);

        let iwad = I256::from_raw(WAD);

        let ratio = weight_x * iwad / weight_y;
        info!("ratio: {}", ratio);

        let inside = ratio * iwad / I256::from_raw(target_price_wad);
        info!("inside: {}", inside);
        let delta_x = invariant * math.pow(inside, weight_y).call().await? / iwad - reserve_x;
        info!("delta_x: {}", delta_x);

        Ok(delta_x.into_raw())
    }

    async fn get_y_input(
        &self,
        target_price_wad: U256,
        math: &SD59x18Math<RevmMiddleware>,
    ) -> Result<U256> {
        let weight_x = I256::from_raw(self.weight_x().call().await?);
        info!("weight_x: {}", weight_x);
        let weight_y = I256::from_raw(self.weight_y().call().await?);
        info!("weight_y: {}", weight_y);
        let reserve_y = I256::from_raw(self.reserve_y().call().await?);
        info!("reserve_y: {}", reserve_y);
        let invariant = I256::from_raw(self.get_invariant().call().await?);
        info!("invariant: {}", invariant);

        let iwad = I256::from_raw(WAD);

        let ratio = weight_y * iwad / weight_x;
        info!("ratio: {}", ratio);

        let inside = ratio * I256::from_raw(target_price_wad) / iwad;
        info!("inside: {}", inside);
        let delta_y = invariant * math.pow(inside, weight_x).call().await? / iwad - reserve_y;
        info!("delta_y: {}", delta_y);
        Ok(delta_y.into_raw())
    }

    async fn get_spot_price(&self) -> Result<U256> {
        Ok(self.get_spot_price().call().await?)
    }

    async fn swap_fee(&self) -> Result<U256> {
        Ok(self.swap_fee().call().await?)
    }
}
