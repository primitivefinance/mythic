//! Dynamic Function Market Making Protocol Client
//!
//! Middleware layer for agents to communicate with the DFMM protocol.
use std::{sync::Arc, time::Duration};

use anyhow::Result;
use bindings::{
    dfmm::DFMM,
    log_normal::LogNormal,
    log_normal_solver::{LogNormParameters as PoolParams, LogNormalSolver},
};
use tracing::debug;

use super::*;

#[derive(Debug)]
pub struct ProtocolClient<C> {
    pub client: Arc<C>,
    pub dfmm: DFMM<C>,
    pub solver: LogNormalSolver<C>,
}

impl<C> Clone for ProtocolClient<C> {
    fn clone(&self) -> Self {
        Self {
            client: self.client.clone(),
            dfmm: self.dfmm.clone(),
            solver: self.solver.clone(),
        }
    }
}

type F64Wad = f64;

impl<C: Middleware + 'static> ProtocolClient<C> {
    pub fn new(client: Arc<C>, dfmm_address: Address, solver_address: Address) -> Self {
        let dfmm = DFMM::new(dfmm_address, client.clone());
        let solver = LogNormalSolver::new(solver_address, client.clone());
        Self {
            client,
            dfmm,
            solver,
        }
    }

    pub async fn get_tokens(&self) -> Result<(Address, Address)> {
        let tokens = (
            self.dfmm.token_x().call().await?,
            self.dfmm.token_y().call().await?,
        );
        Ok(tokens)
    }

    #[tracing::instrument(skip(client), level = "trace", ret)]
    pub async fn deploy_protocol(
        client: Arc<C>,
        token_x: Address,
        token_y: Address,
        swap_fee_percent_wad: f64,
    ) -> anyhow::Result<Self> {
        let swap_fee_percent_wad = ethers::utils::parse_ether(swap_fee_percent_wad).unwrap();
        let args = (true, token_x, token_y, swap_fee_percent_wad);
        let dfmm = DFMM::deploy(client.clone(), args)?.send().await?;
        tracing::info!("Deployed DFMM protocol at address: {}", dfmm.address());
        let strategy = dfmm.strategy().call().await?;
        tracing::info!("Deployed DFMM strategy at address: {}", strategy);

        let solver = LogNormalSolver::deploy(client.clone(), strategy)?
            .send()
            .await.unwrap();
        Ok(Self {
            client,
            dfmm,
            solver,
        })
    }

    #[tracing::instrument(skip(self), level = "trace")]
    pub async fn get_strategy(&self) -> Result<LogNormal<C>> {
        let strategy = LogNormal::new(self.dfmm.strategy().call().await?, self.client.clone());
        Ok(strategy)
    }

    #[tracing::instrument(skip(self), level = "trace", ret)]
    pub async fn get_swap_fee(&self) -> Result<U256> {
        let strategy = self.get_strategy().await?;
        let swap_fee = strategy.swap_fee().call().await?;
        Ok(swap_fee)
    }

    #[tracing::instrument(skip(self), level = "trace", ret)]
    pub async fn get_init_payload(
        &self,
        init_reserve_x_wad: U256,
        init_price_wad: U256,
        init_params: PoolParams,
    ) -> Result<ethers::types::Bytes> {
        let init_data = self
            .solver
            .get_initial_pool_data(init_reserve_x_wad, init_price_wad, init_params)
            .call()
            .await?;
        Ok(init_data)
    }

    pub async fn get_internal_price(&self) -> Result<U256> {
        Ok(self.solver.internal_price().call().await?)
    }

    #[tracing::instrument(skip(self), level = "debug")]
    pub async fn initialize_pool(
        &self,
        init_reserve_x_wad: F64Wad,
        init_price_wad: F64Wad,
        strike_price_wad: F64Wad,
        sigma_percent_wad: F64Wad,
        tau_years_wad: F64Wad,
    ) -> Result<Option<TransactionReceipt>> {
        debug!("Initializing DFMM from protocol client.");

        // Format the parameters for the log-normal strategy.
        let params: PoolParams = PoolParams {
            strike: to_wad(strike_price_wad),
            sigma: to_wad(sigma_percent_wad),
            tau: to_wad(tau_years_wad),
        };
        println!("reserve x wad: {}", init_reserve_x_wad);

        let init_reserve_x_wad = to_wad(init_reserve_x_wad);
        let init_price_wad = to_wad(init_price_wad);

        // Encode the data together to send it to the DFMM protocol.
        let payload = self
            .get_init_payload(init_reserve_x_wad, init_price_wad, params.clone())
            .await?;

        let tx = self
            .dfmm
            .init(payload)
            .send()
            .await?
            .confirmations(0)
            .interval(Duration::from_millis(100))
            .await?;

        Ok(tx)
    }

    #[tracing::instrument(skip(self), level = "trace", ret)]
    pub async fn get_strike_price(&self) -> Result<U256> {
        let strategy = self.get_strategy().await?;
        let strike_price = strategy.strike_price().call().await?;
        Ok(strike_price)
    }

    #[tracing::instrument(skip(self), level = "trace", ret)]
    pub async fn set_strike_price(
        &self,
        target_strike_price_wad: F64Wad,
        next_timestamp: u64,
    ) -> Result<Option<TransactionReceipt>> {
        let strategy = self.get_strategy().await?;
        let target_strike_price_wad = to_wad(target_strike_price_wad);
        let tx = strategy
            .set_strike_price(
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
