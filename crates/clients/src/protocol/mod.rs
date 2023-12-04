//! Dynamic Function Market Making Protocol Client
//!
//! Middleware layer for agents to communicate with the DFMM protocol.
pub mod dfmm;
pub mod log_normal;

use std::sync::Arc;

use anyhow::Result;
use arbiter_core::middleware::RevmMiddleware;
use tracing::debug;

use super::*;

#[derive(Debug, Clone)]
pub struct ProtocolClient {
    pub client: Arc<RevmMiddleware>,
    pub protocol: dfmm::DFMM<RevmMiddleware>,
}

type F64Wad = f64;

impl ProtocolClient {
    pub fn new(client: Arc<RevmMiddleware>, dfmm_address: Address) -> Self {
        let protocol = dfmm::DFMM::new(dfmm_address, client.clone());
        Self { client, protocol }
    }

    #[tracing::instrument(skip(self), level = "trace")]
    pub async fn get_strategy(&self) -> Result<log_normal::LogNormal<RevmMiddleware>> {
        let strategy =
            log_normal::LogNormal::new(self.protocol.source().call().await?, self.client.clone());
        Ok(strategy)
    }

    #[tracing::instrument(skip(self), level = "trace", ret)]
    pub async fn get_init_payload(
        &self,
        init_reserve_x_wad: U256,
        init_reserve_y_wad: U256,
        init_liquidity_wad: U256,
        init_params: log_normal::Parameters,
    ) -> Result<ethers::types::Bytes> {
        let strategy = self.get_strategy().await?;
        let init_data = strategy
            .encode_init_data(
                init_reserve_x_wad,
                init_reserve_y_wad,
                init_liquidity_wad,
                init_params,
            )
            .call()
            .await?;
        Ok(init_data)
    }

    #[tracing::instrument(skip(self), level = "trace", ret)]
    pub async fn get_liquidity_given_x(
        &self,
        reserve_x_wad: U256,
        price_wad: U256,
        params: log_normal::Parameters,
    ) -> Result<U256> {
        let strategy = self.get_strategy().await?;
        let liquidity = strategy.lx(reserve_x_wad, price_wad, params).call().await?;
        Ok(liquidity)
    }

    #[tracing::instrument(skip(self), level = "trace", ret)]
    pub async fn get_y_given_liquidity(
        &self,
        liquidity_wad: U256,
        price_wad: U256,
        params: log_normal::Parameters,
    ) -> Result<U256> {
        let strategy = self.get_strategy().await?;
        let y = strategy.yl(liquidity_wad, price_wad, params).call().await?;
        Ok(y)
    }

    #[tracing::instrument(skip(self), level = "trace", ret)]
    pub async fn find_liquidity(
        &self,
        reserve_x_wad: U256,
        reserve_y_wad: U256,
        swap_constant: I256,
        params: log_normal::Parameters,
    ) -> Result<U256> {
        let strategy = self.get_strategy().await?;
        let liquidity = strategy
            .find_liquidity(reserve_x_wad, reserve_y_wad, swap_constant, params)
            .call()
            .await?;
        Ok(liquidity)
    }

    #[tracing::instrument(skip(self), level = "debug")]
    pub async fn initialize(
        &self,
        init_price_wad: F64Wad,
        init_reserve_x_wad: F64Wad,
        strike_price_wad: F64Wad,
        sigma_percent_wad: F64Wad,
        tau_years_wad: F64Wad,
    ) -> Result<Option<TransactionReceipt>> {
        debug!("Initializing DFMM from protocol client.");

        // Format the parameters for the log-normal strategy.
        let params: log_normal::Parameters = log_normal::Parameters {
            strike_price_wad: to_wad(strike_price_wad),
            sigma_percent_wad: to_wad(sigma_percent_wad),
            tau_years_wad: to_wad(tau_years_wad),
        };

        let init_reserve_x_wad = to_wad(init_reserve_x_wad);
        let init_price_wad = to_wad(init_price_wad);

        let init_liquidity_wad = self
            .get_liquidity_given_x(init_reserve_x_wad, init_price_wad, params.clone())
            .await?;

        let init_reserve_y_wad = self
            .get_y_given_liquidity(init_liquidity_wad, init_price_wad, params.clone())
            .await?;

        let init_swap_constant = I256::zero();

        let liquidity_root = self
            .find_liquidity(
                init_reserve_x_wad,
                init_reserve_y_wad,
                init_swap_constant,
                params.clone(),
            )
            .await?;

        // Encode the data together to send it to the DFMM protocol.
        let payload = self
            .get_init_payload(
                init_reserve_x_wad,
                init_reserve_y_wad,
                liquidity_root,
                params.clone(),
            )
            .await?;

        debug!(
            "Initializing DFMM with: [x: {:?}], [y: {:?}], [L: {:?}], [p: {:?}]",
            init_reserve_x_wad, init_reserve_y_wad, liquidity_root, init_price_wad
        );
        let tx = self.protocol.init(payload).send().await?.await?;

        Ok(tx)
    }
}

fn to_wad(value: F64Wad) -> U256 {
    ethers::utils::parse_ether(value).unwrap()
}
