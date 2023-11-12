use ethers::types::TransactionReceipt;

use super::*;
use crate::bindings::i_strategy::IStrategy;

/// Type of data that is specific to the G3M strategy.
/// Each G3M pool has weights for each side of the pool.
/// These weights are used in the G3M invariant and its derived functions.
pub struct G3MStrategyData {
    weight_x: U256,
    weight_y: U256,
}

/// Implements the G3M strategy invariant and its derived functions.
/// Calls the G3M contract that implements the IStrategy interface.
#[async_trait::async_trait]
impl Strategy for IStrategy<RevmMiddleware> {
    type StrategyData = G3MStrategyData;

    async fn decode_strategy_data(&self) -> Result<Self::StrategyData> {
        let data = self.get_strategy_data().call().await?;
        // decode the bytes data into the weight x and weight y U256 types:
        let weight_x = U256::from_big_endian(&data[0..32]);
        let weight_y = U256::from_big_endian(&data[32..64]);
        Ok(G3MStrategyData { weight_x, weight_y })
    }

    fn new(strategy_address: Address, client: Arc<RevmMiddleware>) -> Self {
        Self::new(strategy_address, client)
    }

    async fn get_spot_price(&self) -> Result<U256> {
        Ok(self.get_spot_price().call().await?)
    }

    async fn get_swap_fee(&self) -> Result<U256> {
        Ok(self.get_swap_fee().call().await?)
    }
}

#[async_trait::async_trait]
impl LiquidityStrategy for IStrategy<RevmMiddleware> {
    async fn initialize_pool(
        &self,
        initial_x_wad: U256,
        initial_price_wad: U256,
    ) -> Result<Option<TransactionReceipt>> {
        Ok(self
            .init_exact_x(initial_x_wad, initial_price_wad)
            .send()
            .await?
            .await?)
    }

    async fn get_reserve_x(&self) -> Result<U256> {
        Ok(self.get_reserve_x().call().await?)
    }

    async fn get_reserve_y(&self) -> Result<U256> {
        Ok(self.get_reserve_y().call().await?)
    }

    async fn get_invariant(&self) -> Result<U256> {
        Ok(self
            .get_invariant()
            .call()
            .await?
            .checked_abs()
            .and_then(|x| Some(x.twos_complement()))
            .unwrap_or_default())
    }

    async fn get_portfolio_value(&self) -> Result<U256> {
        Ok(self.get_portfolio_value().call().await?)
    }
}

/// Uses algebraic methods based on the G3M invariant math to compute the amount
/// of tokens to swap.
#[async_trait::async_trait]
impl ArbitrageStrategy for IStrategy<RevmMiddleware> {
    async fn get_x_input(
        &self,
        target_price_wad: U256,
        math: &SD59x18Math<RevmMiddleware>,
    ) -> Result<U256> {
        let strategy_data = self.decode_strategy_data().await?;
        let weight_x = I256::from_raw(strategy_data.weight_x);
        trace!("weight_x: {}", weight_x);
        let weight_y = I256::from_raw(strategy_data.weight_y);
        trace!("weight_y: {}", weight_y);
        let reserve_x = I256::from_raw(self.get_reserve_x().call().await?);
        trace!("reserve_x: {}", reserve_x);
        let reserve_y = I256::from_raw(self.get_reserve_y().call().await?);
        trace!("reserve_y: {}", reserve_y);
        let invariant = self.get_invariant().call().await?;
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
        let strategy_data = self.decode_strategy_data().await?;
        let weight_x = I256::from_raw(strategy_data.weight_x);
        trace!("weight_x: {}", weight_x);
        let weight_y = I256::from_raw(strategy_data.weight_y);
        trace!("weight_y: {}", weight_y);
        let reserve_x = I256::from_raw(self.get_reserve_x().call().await?);
        trace!("reserve_x: {}", reserve_x);
        let reserve_y = I256::from_raw(self.get_reserve_y().call().await?);
        trace!("reserve_y: {}", reserve_y);
        let invariant = self.get_invariant().call().await?;
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
}
