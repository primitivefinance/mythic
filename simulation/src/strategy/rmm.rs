use super::*;
use crate::bindings::i_strategy::IStrategy;
use ethers::types::TransactionReceipt;

/// Type of data that is specific to the G3M strategy.
/// Each G3M pool has weights for each side of the pool.
/// These weights are used in the G3M invariant and its derived functions.
pub struct RmmStrategyData {
    sigma: U256,
    tau: U256,
    strike_price: U256,
}

pub struct RmmStrategy(IStrategy<RevmMiddleware>);

/// Implements the G3M strategy invariant and its derived functions.
/// Calls the G3M contract that implements the IStrategy interface.
#[async_trait::async_trait]
impl Strategy for RmmStrategy {
    type StrategyData = RmmStrategyData;

    async fn decode_strategy_data(&self) -> Result<Self::StrategyData> {
        let data = self.0.get_strategy_data().call().await?;
        // decode the bytes data into the weight x and weight y U256 types:
        let sigma = U256::from_big_endian(&data[0..32]);
        let tau = U256::from_big_endian(&data[32..64]);
        let strike_price = U256::from_big_endian(&data[64..96]);
        Ok(RmmStrategyData {
            sigma,
            tau,
            strike_price,
        })
    }

    fn new(strategy_address: Address, client: Arc<RevmMiddleware>) -> Self {
        RmmStrategy(IStrategy::new(strategy_address, client))
    }

    async fn get_spot_price(&self) -> Result<U256> {
        Ok(self.0.get_spot_price().call().await?)
    }

    async fn get_swap_fee(&self) -> Result<U256> {
        Ok(self.0.get_swap_fee().call().await?)
    }
}

/// G3M pools must be initialized with a starting amount of x tokens and an
/// initial price.
#[async_trait::async_trait]
impl LiquidityStrategy for RmmStrategy {
    async fn instantiate(
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
impl ArbitrageStrategy for RmmStrategy {
    async fn get_x_input(
        &self,
        target_price_wad: U256,
        math: &SD59x18Math<RevmMiddleware>,
    ) -> Result<U256> {
        let strategy_data = self.decode_strategy_data().await?;
        let sigma = I256::from_raw(strategy_data.sigma);
        trace!("sigma: {}", sigma);
        let tau = I256::from_raw(strategy_data.tau);
        trace!("tau: {}", tau);
        let strike_price = I256::from_raw(strategy_data.strike_price);
        trace!("strike: {}", strike_price);
        let reserve_x = I256::from_raw(self.0.get_reserve_x().call().await?);
        trace!("reserve_x: {}", reserve_x);
        let reserve_y = I256::from_raw(self.0.get_reserve_y().call().await?);
        trace!("reserve_y: {}", reserve_y);
        let invariant = self.0.get_invariant().call().await?;
        trace!("invariant: {}", invariant);

        // let iwad = I256::from_raw(WAD);

        // let ratio = weight_x * iwad / weight_y;
        // trace!("ratio: {}", ratio);

        // let inside = ratio * iwad / I256::from_raw(target_price_wad);
        // trace!("inside: {}", inside);
        // let delta_x = invariant * math.pow(inside, weight_y).call().await? / iwad - reserve_x;
        // trace!("delta_x: {}", delta_x);

        Ok(U256::zero())
    }

    async fn get_y_input(
        &self,
        target_price_wad: U256,
        math: &SD59x18Math<RevmMiddleware>,
    ) -> Result<U256> {
        let strategy_data = self.decode_strategy_data().await?;
        let sigma = I256::from_raw(strategy_data.sigma);
        trace!("sigma: {}", sigma);
        let tau = I256::from_raw(strategy_data.tau);
        trace!("tau: {}", tau);
        let strike_price = I256::from_raw(strategy_data.strike_price);
        trace!("strike: {}", strike_price);
        let reserve_x = I256::from_raw(self.0.get_reserve_x().call().await?);
        trace!("reserve_x: {}", reserve_x);
        let reserve_y = I256::from_raw(self.0.get_reserve_y().call().await?);
        trace!("reserve_y: {}", reserve_y);
        let invariant = self.0.get_invariant().call().await?;
        trace!("invariant: {}", invariant);

        // let iwad = I256::from_raw(WAD);

        // let ratio = weight_y * iwad / weight_x;
        // trace!("ratio: {}", ratio);

        // let inside = ratio * I256::from_raw(target_price_wad) / iwad;
        // trace!("inside: {}", inside);
        // let delta_y = invariant * math.pow(inside, weight_x).call().await? / iwad - reserve_y;
        // trace!("delta_y: {}", delta_y);

        Ok(U256::zero())
    }
}
