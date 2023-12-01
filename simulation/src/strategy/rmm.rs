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

    #[tracing::instrument(skip(self), ret, level = "trace")]
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

    async fn get_reserve_x(&self) -> Result<U256> {
        self.0.get_reserve_x().call().await.map_err(Into::into)
    }

    async fn get_reserve_y(&self) -> Result<U256> {
        self.0.get_reserve_y().call().await.map_err(Into::into)
    }

    async fn get_invariant(&self) -> Result<U256> {
        Ok(self
            .0
            .get_invariant()
            .call()
            .await?
            .checked_abs()
            .map(|x| x.twos_complement())
            .unwrap_or_default())
    }

    async fn get_portfolio_value(&self) -> Result<U256> {
        Ok(self.0.get_portfolio_value().call().await?)
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
    ) -> Result<(U256, U256)> {
        let (sigma, strike_price, tau) = get_strategy_args(self).await?;
        let (sigma, strike_price, _tau) = (
            I256::from_raw(sigma),
            I256::from_raw(strike_price),
            I256::from_raw(tau),
        );

        let gamma = I256::from_raw(self.get_swap_fee().await?);

        let (reserve_x, reserve_y) = get_reserves(self).await?;
        let (reserve_x, _reserve_y) = (I256::from_raw(reserve_x), I256::from_raw(reserve_y));

        let i_wad = I256::from_raw(WAD);

        let next_liquidity = I256::from_raw(get_next_liquidity(self).await?);
        let target_price_wad = I256::from_raw(target_price_wad);

        let dx = get_dx(
            target_price_wad,
            gamma,
            sigma,
            strike_price,
            reserve_x,
            next_liquidity,
            i_wad,
            rmm_math,
        )
        .await?;
        debug!("dx: {}", dx);
        if dx < 0.into() {
            return Ok((0.into(), 0.into()));
        }
        Ok((dx.into_raw(), next_liquidity.into_raw()))
    }

    #[tracing::instrument(ret, skip(self, _g3m_math, rmm_math), level = "trace")]
    async fn get_y_input(
        &self,
        target_price_wad: U256,
        _g3m_math: &SD59x18Math<RevmMiddleware>,
        rmm_math: &ArbiterMath<RevmMiddleware>,
    ) -> Result<(U256, U256)> {
        let (sigma, strike_price, tau) = get_strategy_args(self).await?;
        let (sigma, strike_price, _tau) = (
            I256::from_raw(sigma),
            I256::from_raw(strike_price),
            I256::from_raw(tau),
        );

        let gamma = I256::from_raw(self.get_swap_fee().await?);

        let (reserve_x, reserve_y) = get_reserves(self).await?;
        let (_reserve_x, reserve_y) = (I256::from_raw(reserve_x), I256::from_raw(reserve_y));

        let i_wad = I256::from_raw(WAD);

        let next_liquidity = I256::from_raw(get_next_liquidity(self).await?);
        let target_price_wad = I256::from_raw(target_price_wad);

        let dy = get_dy(
            target_price_wad,
            gamma,
            sigma,
            strike_price,
            reserve_y,
            next_liquidity,
            i_wad,
            rmm_math,
        )
        .await?;
        debug!("dy: {}", dy);
        if dy <= 0.into() {
            return Ok((0.into(), 0.into()));
        }
        Ok((dy.into_raw(), next_liquidity.into_raw()))
    }

    #[tracing::instrument(ret, skip(self))]
    async fn get_reserves_and_liquidity(&self) -> Result<(U256, U256, U256)> {
        let reserve_x = self.0.get_reserve_x().call().await?;
        let reserve_y = self.0.get_reserve_y().call().await?;
        let liquidity = self.0.get_liquidity().call().await?;
        Ok((reserve_x, reserve_y, liquidity))
    }
}

#[tracing::instrument(skip(strategy), ret, level = "trace")]
pub async fn get_reserves(strategy: &RmmStrategy) -> Result<(U256, U256)> {
    let reserve_x = strategy.0.get_reserve_x().call().await?;
    let reserve_y = strategy.0.get_reserve_y().call().await?;
    Ok((reserve_x, reserve_y))
}

#[tracing::instrument(skip(strategy), ret, level = "trace")]
pub async fn get_next_liquidity(strategy: &RmmStrategy) -> Result<U256> {
    let next_liquidity = strategy.0.get_next_liquidity().call().await?;
    Ok(next_liquidity)
}

#[tracing::instrument(skip(strategy), ret, level = "trace")]
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
#[tracing::instrument(skip(rmm_math), ret, level = "trace")]
pub async fn get_dy(
    target_price_wad: I256,
    gamma: I256,
    sigma: I256,
    strike_price: I256,
    reserve_y: I256,
    liquidity: I256,
    i_wad: I256,
    rmm_math: &ArbiterMath<RevmMiddleware>,
) -> Result<I256> {
    let log_p = rmm_math
        .log(target_price_wad * i_wad / strike_price)
        .call()
        .await?;
    let inner_p = log_p * i_wad / sigma - (sigma / 2);
    let cdf_p = rmm_math.cdf(inner_p).call().await?;
    let delta = (liquidity * strike_price) / i_wad * (cdf_p) / i_wad;
    let dy = (delta - reserve_y) * i_wad * i_wad
        / (((gamma - i_wad) * cdf_p) / (reserve_y * i_wad * i_wad / (strike_price * liquidity))
            + i_wad);

    Ok(dy / i_wad)
}

// \boxed{\Delta x =
// L\cdot\left(1-\Phi\left(\frac{\ln\frac{S'}{K}+\frac{1}{2}\sigma^2}{\sigma}\
// right)\right) - x}
pub async fn get_dx(
    target_price_wad: I256,
    gamma: I256,
    sigma: I256,
    strike_price: I256,
    reserve_x: I256,
    liquidity: I256,
    i_wad: I256,
    rmm_math: &ArbiterMath<RevmMiddleware>,
) -> Result<I256> {
    let log_p = rmm_math
        .log(target_price_wad * i_wad / strike_price)
        .call()
        .await?;
    let inner_p = log_p * i_wad / sigma + (sigma / 2);
    let cdf_p = rmm_math.cdf(inner_p).call().await?;
    let delta = liquidity * (i_wad - cdf_p) / i_wad;
    let dx = (delta - reserve_x) * i_wad * i_wad
        / (((gamma - i_wad) * (i_wad - cdf_p)) / (reserve_x * i_wad / liquidity) + i_wad);
    Ok(dx / i_wad)
}

#[cfg(test)]
mod tests {
    use super::*;
    use arbiter_core::environment::builder::EnvironmentBuilder;

    #[tokio::test]
    async fn test_get_dx() {
        let environment = EnvironmentBuilder::new().label("test").build();
        let client = RevmMiddleware::new(&environment, "arbitrageur".into()).unwrap();
        let rmm_math = ArbiterMath::deploy(client.clone(), ())
            .unwrap()
            .send()
            .await
            .unwrap();
        let i_wad = I256::from_raw(parse_ether("1").unwrap());

        let target_price_wad = I256::from_raw(parse_ether("0.9").unwrap());
        let gamma = I256::from_raw(parse_ether("0.997").unwrap());
        let sigma = I256::from_raw(parse_ether("1").unwrap());
        let strike_price = I256::from_raw(parse_ether("1").unwrap());
        let reserve_x = I256::from_raw(parse_ether("1").unwrap());
        let liquidity = I256::from_raw(parse_ether("3.241096705").unwrap());

        let dx = get_dx(
            target_price_wad,
            gamma,
            sigma,
            strike_price,
            reserve_x,
            liquidity,
            i_wad,
            &rmm_math,
        )
        .await
        .unwrap();
        assert_eq!(
            dx,
            I256::from_raw(parse_ether("0.000000000000000001").unwrap())
        );
    }

    #[tokio::test]
    async fn test_get_dy() {
        let environment = EnvironmentBuilder::new().label("test").build();
        let client = RevmMiddleware::new(&environment, "arbitrageur".into()).unwrap();
        let rmm_math = ArbiterMath::deploy(client.clone(), ())
            .unwrap()
            .send()
            .await
            .unwrap();
        let i_wad = I256::from_raw(parse_ether("1").unwrap());

        let target_price_wad = I256::from_raw(parse_ether("1.1").unwrap());
        let gamma = I256::from_raw(parse_ether("0.997").unwrap());
        let sigma = I256::from_raw(parse_ether("1").unwrap());
        let strike_price = I256::from_raw(parse_ether("1").unwrap());
        let reserve_y = I256::from_raw(parse_ether("1").unwrap());
        let liquidity = I256::from_raw(parse_ether("3.241096705").unwrap());

        let dy = get_dy(
            target_price_wad,
            gamma,
            sigma,
            strike_price,
            reserve_y,
            liquidity,
            i_wad,
            &rmm_math,
        )
        .await
        .unwrap();
        assert_eq!(
            dy,
            I256::from_raw(parse_ether("0.000000000000000001").unwrap())
        );
    }
}
