use ethers::types::TransactionReceipt;

use super::*;

#[async_trait::async_trait]
impl Strategy for G3M<RevmMiddleware> {
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

    async fn get_lp_amounts(&self, config: &SimulationConfig) -> (U256, U256) {
        // Initial weight is set in the simulation config, but it can be overridden with
        // setWeightX() function.
        let weight_x = parse_ether(config.pool.weight_x).unwrap();
        let weight_y = parse_ether(1).unwrap().checked_sub(weight_x).unwrap();
        // Using the initial weight, initial price, and initial reserve x, we can
        // compute reserve y.
        let initial_price = config.trajectory.initial_price;
        let initial_reserve_x = float_to_wad(config.lp.x_liquidity);
        trace!("initial_reserve_x: {}", initial_reserve_x);

        // p = (x / w_x) / (y / w_y)
        // y / w_y = (x / w_x) / p
        // y = (x / w_x) / p * w_y
        let one_ether = parse_ether(1).unwrap();
        let initial_reserve_y = initial_reserve_x
            .checked_mul(one_ether)
            .unwrap()
            .checked_div(weight_x)
            .unwrap()
            .checked_mul(one_ether)
            .unwrap()
            .checked_div(parse_ether(initial_price).unwrap())
            .unwrap()
            .checked_mul(weight_y)
            .unwrap()
            .checked_div(one_ether)
            .unwrap();
        trace!("initial_reserve_y: {}", initial_reserve_y);
        (initial_reserve_x, initial_reserve_y)
    }

    async fn init_pool(
        &self,
        amount_x: U256,
        amount_y: U256,
    ) -> Result<Option<TransactionReceipt>> {
        Ok(self.init_pool(amount_x, amount_y).send().await?.await?)
    }
}
