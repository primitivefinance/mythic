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
        let reserve_y = self.reserve_y().call().await?;
        let invariant = self.get_invariant().call().await?;

        let pow = math
            .div(I256::from(1), I256::from_raw(weight_y))
            .call()
            .await?;
        info!("pow: {:#?}", format_units(pow, "wei"));
        let invariant_to_pow = math.pow(I256::from_raw(invariant), pow).call().await?;
        info!(
            "invariant_to_pow: {:#?}",
            format_units(invariant_to_pow, "wei")
        );

        Ok(weight_y
            * U256::from(1)
                .div(target_price_wad * invariant.pow(U256::from(1).div(weight_x)))
                .pow(U256::from(1) + weight_y.div(weight_x))
            - reserve_y)
    }

    async fn get_y_input(
        &self,
        target_price_wad: U256,
        math: &SD59x18Math<RevmMiddleware>,
    ) -> Result<U256> {
        let weight_x = self.weight_x().call().await?;
        let weight_y = self.weight_y().call().await?;
        let reserve_y = self.reserve_y().call().await?;
        let invariant = self.get_invariant().call().await?;

        let pow = math
            .div(I256::from(1), I256::from_raw(weight_x))
            .call()
            .await?;
        info!("pow: {:#?}", format_units(pow, "wei"));
        let invariant_to_pow = math.pow(I256::from_raw(invariant), pow).call().await?;
        info!(
            "invariant_to_pow: {:#?}",
            format_units(invariant_to_pow, "wei")
        );

        let quantity = I256::from_raw(target_price_wad) * invariant_to_pow / I256::from_raw(WAD);
        info!("quantity: {:#?}", format_units(quantity, "wei"));
        let power_for_quantity = math
            .div(
                I256::from_raw(WAD),
                I256::from_raw(WAD)
                    + math
                        .div(I256::from_raw(weight_y), I256::from_raw(weight_x))
                        .call()
                        .await?,
            )
            .call()
            .await?;
        info!(
            "power_for_quantity: {:#?}",
            format_units(power_for_quantity, "wei")
        );
        let quantity_to_pow = math.pow(quantity, power_for_quantity).call().await?;
        info!(
            "quantity_to_pow: {:#?}",
            format_units(quantity_to_pow, "wei")
        );
        let delta_y = I256::from_raw(weight_y) * quantity_to_pow / I256::from_raw(WAD)
            - I256::from_raw(reserve_y);
        info!("delta_y: {:#?}", format_units(delta_y, "wei"));

        Ok(delta_y.into_raw())
    }

    async fn get_spot_price(&self) -> Result<U256> {
        Ok(self.get_spot_price().call().await?)
    }

    async fn swap_fee(&self) -> Result<U256> {
        Ok(self.swap_fee().call().await?)
    }
}
