use ethers::types::TransactionReceipt;

use super::*;
use crate::bindings::i_strategy::IStrategy;

pub struct G3mStrategy(IStrategy<RevmMiddleware>);

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
impl Strategy for G3mStrategy {
    type StrategyData = G3MStrategyData;

    fn get_address(&self) -> Address {
        self.0.address()
    }

    async fn decode_strategy_data(&self) -> Result<Self::StrategyData> {
        let data = self.0.get_strategy_data().call().await?;
        // decode the bytes data into the weight x and weight y U256 types:
        let weight_x = U256::from_big_endian(&data[0..32]);
        let weight_y = U256::from_big_endian(&data[32..64]);
        Ok(G3MStrategyData { weight_x, weight_y })
    }

    fn new(strategy_address: Address, client: Arc<RevmMiddleware>) -> Self {
        G3mStrategy(IStrategy::new(strategy_address, client))
    }

    async fn get_spot_price(&self) -> Result<U256> {
        Ok(self.0.get_spot_price().call().await?)
    }

    async fn get_swap_fee(&self) -> Result<U256> {
        Ok(self.0.get_swap_fee().call().await?)
    }
}

#[async_trait::async_trait]
impl LiquidityStrategy for G3mStrategy {
    async fn initialize_pool(
        &self,
        initial_x_wad: U256,
        initial_price_wad: U256,
    ) -> Result<Option<TransactionReceipt>> {
        Ok(self
            .0
            .init_exact_x(initial_x_wad, initial_price_wad)
            .send()
            .await?
            .await?)
    }
}

/// Uses algebraic methods based on the G3M invariant math to compute the amount
/// of tokens to swap.
#[async_trait::async_trait]
impl ArbitrageStrategy for G3mStrategy {
    async fn get_x_input(
        &self,
        target_price_wad: U256,
        math: &SD59x18Math<RevmMiddleware>,
        rmm_math: &ArbiterMath<RevmMiddleware>,
    ) -> Result<U256> {
        let strategy_data = self.decode_strategy_data().await?;
        let weight_x = I256::from_raw(strategy_data.weight_x);
        trace!("weight_x: {}", weight_x);
        let weight_y = I256::from_raw(strategy_data.weight_y);
        trace!("weight_y: {}", weight_y);
        let reserve_x = I256::from_raw(self.0.get_reserve_x().call().await?);
        trace!("reserve_x: {}", reserve_x);
        let reserve_y = I256::from_raw(self.0.get_reserve_y().call().await?);
        trace!("reserve_y: {}", reserve_y);
        let invariant = self.0.get_invariant().call().await?;
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
        rmm_math: &ArbiterMath<RevmMiddleware>,
    ) -> Result<U256> {
        let strategy_data = self.decode_strategy_data().await?;
        let weight_x = I256::from_raw(strategy_data.weight_x);
        trace!("weight_x: {}", weight_x);
        let weight_y = I256::from_raw(strategy_data.weight_y);
        trace!("weight_y: {}", weight_y);
        let reserve_x = I256::from_raw(self.0.get_reserve_x().call().await?);
        trace!("reserve_x: {}", reserve_x);
        let reserve_y = I256::from_raw(self.0.get_reserve_y().call().await?);
        trace!("reserve_y: {}", reserve_y);
        let invariant = self.0.get_invariant().call().await?;
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

    #[tracing::instrument(ret, skip(self))]
    async fn get_reserves_and_liquidity(&self) -> Result<(U256, U256, U256)> {
        let reserve_x = self.0.get_reserve_x().call().await?;
        let reserve_y = self.0.get_reserve_y().call().await?;
        let liquidity = self.0.get_liquidity().call().await?;
        Ok((reserve_x, reserve_y, liquidity))
    }
}
