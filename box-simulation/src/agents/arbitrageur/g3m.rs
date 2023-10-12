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
        let weight_x = self.weight_x().call().await?;
        let weight_y = self.weight_y().call().await?;
        let reserve_x = self.reserve_x().call().await?;
        let invariant = self.get_invariant().call().await?;

        Ok(U256::from(1))
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

        let inside = (ratio * I256::from_raw(target_price_wad) / iwad);
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
