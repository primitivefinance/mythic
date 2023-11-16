use ethers::types::TransactionReceipt;

use super::*;
use crate::bindings::i_strategy::IStrategy;

/// Type of data that is specific to the G3M strategy.
/// Each G3M pool has weights for each side of the pool.
/// These weights are used in the G3M invariant and its derived functions.
#[derive(Debug, Clone, Copy)]
pub struct RmmStrategyData {
    pub sigma: U256,
    pub strike_price: U256,
    pub tau: U256,
}

#[derive(Debug, Clone)]
pub struct RmmStrategy(IStrategy<RevmMiddleware>);

/// Implements the G3M strategy invariant and its derived functions.
/// Calls the G3M contract that implements the IStrategy interface.
#[async_trait::async_trait]
impl Strategy for RmmStrategy {
    type StrategyData = RmmStrategyData;

    fn get_address(&self) -> Address {
        self.0.address()
    }

    #[tracing::instrument(skip(self), ret)]
    async fn decode_strategy_data(&self) -> Result<Self::StrategyData> {
        let data = self.0.get_strategy_data().call().await?;
        // decode the bytes data into the weight x and weight y U256 types:
        let sigma = U256::from_big_endian(&data[0..32]);
        let strike_price = U256::from_big_endian(&data[32..64]);
        let tau = U256::from_big_endian(&data[64..96]);
        Ok(RmmStrategyData {
            sigma,
            strike_price,
            tau,
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

    async fn get_strategy_logs(&self) {
        self.0.log_data().send().await.unwrap().await.unwrap();
    }
}

/// G3M pools must be initialized with a starting amount of x tokens and an
/// initial price.
#[async_trait::async_trait]
impl LiquidityStrategy for RmmStrategy {
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
impl ArbitrageStrategy for RmmStrategy {
    async fn get_x_input(
        &self,
        target_price_wad: U256,
        _g3m_math: &SD59x18Math<RevmMiddleware>,
        rmm_math: &ArbiterMath<RevmMiddleware>,
    ) -> Result<U256> {
        let strategy_data = self.decode_strategy_data().await?;
        let sigma = I256::from_raw(strategy_data.sigma);
        debug!("sigma: {}", sigma);
        let tau = I256::from_raw(strategy_data.tau);
        debug!("tau: {}", tau);
        let strike_price = I256::from_raw(strategy_data.strike_price);
        debug!("strike: {}", strike_price);
        let reserve_x = I256::from_raw(self.0.get_reserve_x().call().await?);
        debug!("reserve_x: {}", reserve_x);
        let reserve_y = I256::from_raw(self.0.get_reserve_y().call().await?);
        debug!("reserve_y: {}", reserve_y);
        let liquidity = I256::from_raw(self.0.get_liquidity().call().await?);
        debug!("liquidity: {}", liquidity);
        let i_wad = I256::from_raw(WAD);
        // \boxed{\Delta x =
        // L\cdot\left(1-\Phi\left(\frac{\ln\frac{S'}{K}+\frac{1}{2}\sigma^2}{\sigma}\
        // right)\right) - x}
        let dx = liquidity
            * (i_wad
                - rmm_math
                    .cdf(
                        rmm_math
                            .log(I256::from_raw(target_price_wad) * i_wad / strike_price)
                            .call()
                            .await?
                            * i_wad
                            / sigma
                            + sigma / 2,
                    )
                    .call()
                    .await?)
            / i_wad
            - reserve_x;

        debug!("dx: {}", dx);
        if dx < 0.into() {
            return Ok(0.into());
        }
        Ok(dx.into_raw())
    }

    #[tracing::instrument(ret, skip(self, _g3m_math, rmm_math))]
    async fn get_y_input(
        &self,
        target_price_wad: U256,
        _g3m_math: &SD59x18Math<RevmMiddleware>,
        rmm_math: &ArbiterMath<RevmMiddleware>,
    ) -> Result<U256> {
        let (sigma, strike_price, tau) = get_strategy_args(self).await?;
        let (sigma, strike_price, tau) = (
            I256::from_raw(sigma),
            I256::from_raw(strike_price),
            I256::from_raw(tau),
        );

        let (_reserve_x, reserve_y) = get_reserves(self).await?;
        let (_reserve_x, reserve_y) = (I256::from_raw(_reserve_x), I256::from_raw(reserve_y));

        let liquidity = I256::from_raw(self.0.get_liquidity().call().await?);
        debug!("liquidity: {}", liquidity);

        let i_wad = I256::from_raw(WAD);

        let dy = get_dy(
            target_price_wad,
            sigma,
            strike_price,
            reserve_y,
            liquidity,
            i_wad,
            rmm_math,
        )
        .await?;
        Ok(dy.into_raw())
    }

    #[tracing::instrument(ret, skip(self))]
    async fn get_reserves_and_liquidity(&self) -> Result<(U256, U256, U256)> {
        let reserve_x = self.0.get_reserve_x().call().await?;
        let reserve_y = self.0.get_reserve_y().call().await?;
        let liquidity = self.0.get_liquidity().call().await?;
        Ok((reserve_x, reserve_y, liquidity))
    }
}

#[tracing::instrument(skip(strategy), ret)]
pub async fn get_reserves(strategy: &RmmStrategy) -> Result<(U256, U256)> {
    let reserve_x = strategy.0.get_reserve_x().call().await?;
    let reserve_y = strategy.0.get_reserve_y().call().await?;
    Ok((reserve_x, reserve_y))
}

#[tracing::instrument(skip(strategy), ret)]
pub async fn get_strategy_args(strategy: &RmmStrategy) -> Result<(U256, U256, U256)> {
    let strategy_data = strategy.decode_strategy_data().await?;
    Ok((
        strategy_data.sigma,
        strategy_data.strike_price,
        strategy_data.tau,
    ))
}

/// \boxed{\Delta_y = y'-y = K\cdot L \cdot
/// \Phi\left(\frac{\ln\frac{S'}{K}-\frac{1}{2}\sigma^2}{\sigma}\right)-y}
#[tracing::instrument(skip(rmm_math), ret)]
pub async fn get_dy(
    target_price_wad: U256,
    sigma: I256,
    strike_price: I256,
    reserve_y: I256,
    liquidity: I256,
    i_wad: I256,
    rmm_math: &ArbiterMath<RevmMiddleware>,
) -> Result<I256> {
    let dy = liquidity
        * rmm_math
            .cdf(
                rmm_math
                    .log(I256::from_raw(target_price_wad) * i_wad / strike_price)
                    .call()
                    .await?
                    * i_wad
                    / sigma
                    - sigma / 2,
            )
            .call()
            .await?
        / i_wad
        - reserve_y;

    Ok(dy)
}
