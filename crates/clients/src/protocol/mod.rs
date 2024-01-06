//! Dynamic Function Market Making Protocol Client
//!
//! Middleware layer for agents to communicate with the DFMM protocol.
use std::{sync::Arc, time::Duration};

use anyhow::Result;
use bindings::{
    g3m::G3M,
    g3m_solver::G3MSolver,
    log_normal::LogNormal,
    log_normal_solver::{LogNormParameters, LogNormalSolver},
    multi_dfmm::MultiDFMM,
    shared_types::{G3Mparameters, InitParams},
};
use tracing::debug;

use super::*;

#[derive(Debug)]
pub struct ProtocolClient<C> {
    pub client: Arc<C>,
    pub protocol: MultiDFMM<C>,
    pub ln_strategy: LogNormal<C>,
    pub ln_solver: LogNormalSolver<C>,
    pub g_strategy: G3M<C>,
    pub g_solver: G3MSolver<C>,
    pub token_x: Address,
    pub token_y: Address,
}

impl<C> Clone for ProtocolClient<C> {
    fn clone(&self) -> Self {
        Self {
            client: self.client.clone(),
            protocol: self.protocol.clone(),
            ln_strategy: self.ln_strategy.clone(),
            ln_solver: self.ln_solver.clone(),
            g_strategy: self.g_strategy.clone(),
            g_solver: self.g_solver.clone(),
            token_x: self.token_x,
            token_y: self.token_y,
        }
    }
}

type F64Wad = f64;

impl<C: Middleware + 'static> ProtocolClient<C> {
    #[tracing::instrument(skip(client), level = "trace", ret)]
    pub async fn new(
        client: Arc<C>,
        token_x: Address,
        token_y: Address,
        swap_fee_percent_wad: f64,
    ) -> anyhow::Result<Self> {
        let swap_fee_percent_wad = ethers::utils::parse_ether(swap_fee_percent_wad).unwrap();
        let protocol = MultiDFMM::deploy(client.clone(), ())?.send().await?;
        let ln_strategy =
            LogNormal::deploy(client.clone(), (protocol.address(), swap_fee_percent_wad))?
                .send()
                .await?;
        let g_strategy = G3M::deploy(client.clone(), (protocol.address(), swap_fee_percent_wad))?
            .send()
            .await?;
        let ln_solver = LogNormalSolver::deploy(client.clone(), (ln_strategy.address()))?
            .send()
            .await?;
        let g_solver = G3MSolver::deploy(client.clone(), (g_strategy.address()))?
            .send()
            .await?;
        Ok(Self {
            client,
            protocol,
            ln_strategy,
            ln_solver,
            g_strategy,
            g_solver,
            token_x,
            token_y,
        })
    }

    pub async fn get_tokens(&self) -> Result<(Address, Address)> {
        Ok((self.token_x, self.token_y))
    }

    #[tracing::instrument(skip(self), level = "trace", ret)]
    pub async fn get_swap_fee(&self) -> Result<U256> {
        let swap_fee = self.ln_strategy.swap_fee().call().await?;
        Ok(swap_fee)
    }

    #[tracing::instrument(skip(self), level = "trace", ret)]
    pub async fn get_ln_init_payload(
        &self,
        init_reserve_x_wad: U256,
        init_price_wad: U256,
        init_params: LogNormParameters,
    ) -> Result<InitParams> {
        let nonce = self.protocol.nonce().call().await?;
        let init_data = self
            .ln_solver
            .get_initial_pool_data(init_reserve_x_wad, init_price_wad, init_params)
            .call()
            .await?;
        let init_params: InitParams = InitParams {
            pool_id: nonce,
            strategy: self.ln_strategy.address(),
            token_x: self.token_x,
            token_y: self.token_y,
            swap_fee_percentage_wad: ethers::utils::parse_ether(0.003).unwrap(),
            data: init_data,
        };
        Ok(init_params)
    }

    #[tracing::instrument(skip(self), level = "trace", ret)]
    pub async fn get_g_init_payload(
        &self,
        init_reserve_x_wad: U256,
        init_price_wad: U256,
        init_params: G3Mparameters,
    ) -> Result<InitParams> {
        let nonce = self.protocol.nonce().call().await?;
        let init_data = self
            .g_solver
            .get_initial_pool_data(init_reserve_x_wad, init_price_wad, init_params)
            .call()
            .await?;
        let init_params: InitParams = InitParams {
            pool_id: nonce,
            strategy: self.g_strategy.address(),
            token_x: self.token_x,
            token_y: self.token_y,
            swap_fee_percentage_wad: ethers::utils::parse_ether(0.003).unwrap(),
            data: init_data,
        };
        Ok(init_params)
    }

    pub async fn get_ln_internal_price(&self, pool_id: U256) -> Result<U256> {
        Ok(self.ln_solver.internal_price(pool_id).call().await?)
    }

    pub async fn get_g_internal_price(&self, pool_id: U256) -> Result<U256> {
        Ok(self.g_solver.internal_price(pool_id).call().await?)
    }

    #[tracing::instrument(skip(self), level = "debug")]
    pub async fn initialize_ln_pool(
        &self,
        init_reserve_x_wad: F64Wad,
        init_price_wad: F64Wad,
        strike_price_wad: F64Wad,
        sigma_percent_wad: F64Wad,
        tau_years_wad: F64Wad,
    ) -> Result<Option<TransactionReceipt>> {
        debug!("Initializing DFMM from protocol client.");

        // Format the parameters for the log-normal strategy.
        let params: LogNormParameters = LogNormParameters {
            strike: to_wad(strike_price_wad),
            sigma: to_wad(sigma_percent_wad),
            tau: to_wad(tau_years_wad),
        };
        println!("reserve x wad: {}", init_reserve_x_wad);

        let init_reserve_x_wad = to_wad(init_reserve_x_wad);
        let init_price_wad = to_wad(init_price_wad);

        // Encode the data together to send it to the DFMM protocol.
        let payload = self
            .get_ln_init_payload(init_reserve_x_wad, init_price_wad, params.clone())
            .await?;

        let tx = self
            .protocol
            .init(payload)
            .send()
            .await?
            .confirmations(0)
            .interval(Duration::from_millis(100))
            .await?;

        Ok(tx)
    }

    #[tracing::instrument(skip(self), level = "trace", ret)]
    pub async fn get_strike_price(&self, pool_id: U256) -> Result<U256> {
        let strike_price = self.ln_strategy.strike_price(pool_id).call().await?;
        Ok(strike_price)
    }

    #[tracing::instrument(skip(self), level = "trace", ret)]
    pub async fn set_strike_price(
        &self,
        pool_id: U256,
        target_strike_price_wad: F64Wad,
        next_timestamp: u64,
    ) -> Result<Option<TransactionReceipt>> {
        let target_strike_price_wad = to_wad(target_strike_price_wad);
        let tx = self
            .ln_strategy
            .set_strike_price(
                pool_id,
                target_strike_price_wad,
                ethers::types::U256::from(next_timestamp),
            )
            .send()
            .await?
            .await?;
        Ok(tx)
    }
}

fn to_wad(value: F64Wad) -> U256 {
    ethers::utils::parse_ether(value).unwrap()
}
