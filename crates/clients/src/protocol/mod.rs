//! Dynamic Function Market Making Protocol Client
//!
//! Middleware layer for agents to communicate with the DFMM protocol.
pub mod pool;

use std::{sync::Arc, time::Duration};

use anyhow::Result;
use arbiter_core::errors::ArbiterCoreError;
use bindings::{
    dfmm::{Pool as PoolStruct, DFMM},
    g3m::G3M,
    g3m_solver::{G3MSolver, G3Mparams},
    log_normal::LogNormal,
    log_normal_solver::{LogNormalParams, LogNormalSolver},
    shared_types::InitParams,
};
use ethers::utils::parse_ether;
use pool::{Pool, PoolKind};

use super::*;

#[derive(Debug, Clone)]
pub struct G3mF64 {
    pub wx: f64,
    pub swap_fee: f64,
    pub controller: Address,
}

#[derive(Debug, Clone)]
pub struct LogNormalF64 {
    pub sigma: f64,
    pub strike: f64,
    pub tau: f64,
    pub swap_fee: f64,
    pub controller: Address,
}

#[derive(Debug, Clone)]
pub enum PoolInitParamsF64 {
    G3M(G3mF64),
    LogNormal(LogNormalF64),
}

#[derive(Debug, Clone)]
pub enum PoolParams {
    G3M(G3Mparams),
    LogNormal(LogNormalParams),
}

#[derive(Debug)]
pub struct ProtocolClient<C> {
    pub client: Arc<C>,
    pub protocol: DFMM<C>,
    pub ln_solver: LogNormalSolver<C>,
    pub ln_strategy: LogNormal<C>,
    pub g_solver: G3MSolver<C>,
    pub g_strategy: G3M<C>,
}

impl<C> Clone for ProtocolClient<C> {
    fn clone(&self) -> Self {
        Self {
            client: self.client.clone(),
            protocol: self.protocol.clone(),
            ln_solver: self.ln_solver.clone(),
            ln_strategy: self.ln_strategy.clone(),
            g_solver: self.g_solver.clone(),
            g_strategy: self.g_strategy.clone(),
        }
    }
}

impl<C: Middleware + 'static> ProtocolClient<C> {
    #[tracing::instrument(level = "trace", ret)]
    pub async fn new(
        client: Arc<C>,
        token_x: Address,
        token_y: Address,
        swap_fee_percent_wad: f64,
    ) -> Result<Self> {
        let protocol = DFMM::deploy(client.clone(), ())?.send().await?;
        let ln_strategy = LogNormal::deploy(client.clone(), protocol.address())?
            .send()
            .await?;

        let g_strategy = G3M::deploy(client.clone(), protocol.address())?
            .send()
            .await?;

        let ln_solver = LogNormalSolver::deploy(client.clone(), ln_strategy.address())?
            .send()
            .await?;

        let g_solver = G3MSolver::deploy(client.clone(), g_strategy.address())?
            .send()
            .await?;

        Ok(Self {
            client,
            protocol,
            ln_solver,
            ln_strategy,
            g_solver,
            g_strategy,
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub fn from_deployed(
        client: Arc<C>,
        protocol_addr: Address,
        ln_strategy_addr: Address,
        ln_solver_addr: Address,
        g_strategy_addr: Address,
        g_solver_addr: Address,
    ) -> Result<Self> {
        Ok(Self {
            client: client.clone(),
            protocol: DFMM::new(protocol_addr, client.clone()),
            ln_strategy: LogNormal::new(ln_strategy_addr, client.clone()),
            ln_solver: LogNormalSolver::new(ln_solver_addr, client.clone()),
            g_strategy: G3M::new(g_strategy_addr, client.clone()),
            g_solver: G3MSolver::new(g_solver_addr, client.clone()),
        })
    }

    pub fn connect(&self, client: Arc<C>) -> Result<Self> {
        Ok(Self {
            client: client.clone(),
            protocol: self.protocol.connect(client.clone()).into(),
            ln_strategy: self.ln_strategy.connect(client.clone()).into(),
            ln_solver: self.ln_solver.connect(client.clone()).into(),
            g_strategy: self.g_strategy.connect(client.clone()).into(),
            g_solver: self.g_solver.connect(client.clone()).into(),
        })
    }

    pub async fn get_pool(&self, pool_id: U256) -> Result<Pool, ArbiterCoreError> {
        let pool_data: PoolStruct = self.protocol.get_pool(pool_id).call().await.unwrap();

        let token_x = pool_data.token_x;
        let token_y = pool_data.token_y;
        let strategy = pool_data.strategy;
        let ln_address = self.ln_strategy.address();
        let g_address = self.g_strategy.address();

        let kind = match strategy {
            _ if strategy == ln_address => PoolKind::LogNormal,
            _ if strategy == g_address => PoolKind::G3M,
            _ => return Err(ArbiterCoreError::MissingDataError),
        };

        let pool = Pool {
            kind,
            token_x,
            token_y,
            pool_id,
        };

        Ok(pool)
    }
    // todo(matt): fix this
    // pub async fn update_controller(&self, pool_id: U256, new_controller: Address)
    // -> Result<()> { self.protocol
    // .update_controller(pool_id, new_controller)
    // .send()
    // .await?;
    // Ok(())
    // }

    #[tracing::instrument(skip(self), level = "trace", ret)]
    pub async fn init_pool(
        &self,
        token_x: Address,
        token_y: Address,
        init_reserve_x_wad: U256,
        init_price_wad: U256,
        init_params: PoolInitParamsF64,
    ) -> Result<TransactionReceipt> {
        let pool_params = to_init_params_wad(init_params)?;

        match pool_params {
            PoolParams::G3M(g3m_params) => {
                let init_data = self
                    .g_solver
                    .get_initial_pool_data(init_reserve_x_wad, init_price_wad, g3m_params)
                    .call()
                    .await?;
                let init_params: InitParams = InitParams {
                    strategy: self.g_strategy.address(),
                    token_x,
                    token_y,
                    data: init_data,
                };

                let tx = self.initialize_pool(init_params).await?.unwrap();

                Ok(tx)
            }
            PoolParams::LogNormal(log_normal_params) => {
                let init_data = self
                    .ln_solver
                    .get_initial_pool_data(init_reserve_x_wad, init_price_wad, log_normal_params)
                    .call()
                    .await?;
                let init_params: InitParams = InitParams {
                    strategy: self.ln_strategy.address(),
                    token_x,
                    token_y,
                    data: init_data,
                };

                let tx = self.initialize_pool(init_params).await?.unwrap();

                Ok(tx)
            }
        }
    }

    pub async fn get_next_pool_id(&self) -> Result<U256> {
        Ok(self.protocol.nonce().call().await?)
    }

    /// Gets the data to send to the `initialize_pool` function.
    pub async fn get_init_payload(
        &self,
        token_x: Address,
        token_y: Address,
        init_reserve_x_wad: U256,
        init_price_wad: U256,
        init_params: PoolInitParamsF64,
    ) -> Result<InitParams> {
        let pool_params = to_init_params_wad(init_params)?;
        match pool_params {
            PoolParams::G3M(g3m_params) => {
                let init_data = self
                    .g_solver
                    .get_initial_pool_data(init_reserve_x_wad, init_price_wad, g3m_params)
                    .call()
                    .await?;
                let init_params: InitParams = InitParams {
                    strategy: self.g_strategy.address(),
                    token_x,
                    token_y,
                    data: init_data,
                };
                Ok(init_params)
            }
            PoolParams::LogNormal(log_normal_params) => {
                tracing::info!("log normal solver address: {:?}", self.ln_solver.address());
                let init_data = self
                    .ln_solver
                    .get_initial_pool_data(init_reserve_x_wad, init_price_wad, log_normal_params)
                    .call()
                    .await?;
                let init_params: InitParams = InitParams {
                    strategy: self.ln_strategy.address(),
                    token_x,
                    token_y,
                    data: init_data,
                };
                Ok(init_params)
            }
        }
    }

    pub async fn initialize_pool(&self, payload: InitParams) -> Result<Option<TransactionReceipt>> {
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

    /// alex: testing out some blocking in the application
    pub async fn create_position(
        &self,
        token_x: Address,
        token_y: Address,
        init_reserve_x_wad: U256,
        init_price_wad: U256,
        init_params: PoolInitParamsF64,
    ) -> Result<Option<TransactionReceipt>> {
        let payload = self
            .get_init_payload(
                token_x,
                token_y,
                init_reserve_x_wad,
                init_price_wad,
                init_params,
            )
            .await?;

        self.initialize_pool(payload).await
    }

    #[tracing::instrument(skip(self), level = "trace", ret)]
    pub async fn get_internal_price(&self, pool_id: U256) -> Result<U256, ArbiterCoreError> {
        let pool = self.get_pool(pool_id).await?;

        match pool.kind {
            PoolKind::G3M => Ok(self.g_solver.internal_price(pool_id).call().await.unwrap()),
            PoolKind::LogNormal => Ok(self.ln_solver.internal_price(pool_id).call().await.unwrap()),
            _ => return Err(ArbiterCoreError::MissingDataError),
        }
    }

    pub async fn get_reserves_and_liquidity(&self, pool_id: U256) -> Result<(U256, U256, U256)> {
        Ok(self
            .protocol
            .get_reserves_and_liquidity(pool_id)
            .call()
            .await?)
    }

    #[tracing::instrument(skip(self), level = "trace", ret)]
    pub async fn get_params(&self, pool_id: U256) -> Result<PoolParams> {
        let pool = self.get_pool(pool_id).await?;

        match pool.kind {
            PoolKind::G3M => Ok(PoolParams::G3M(
                self.g_solver.fetch_pool_params(pool_id).call().await?,
            )),
            PoolKind::LogNormal => Ok(PoolParams::LogNormal(
                self.ln_solver.fetch_pool_params(pool_id).call().await?,
            )),
        }
    }

    #[tracing::instrument(skip(self), level = "trace", ret)]
    pub async fn set_strike_price(
        &self,
        pool_id: U256,
        target_strike_price: f64,
        next_timestamp: u64,
    ) -> Result<Option<TransactionReceipt>> {
        let target_strike_wad = to_wad(target_strike_price);
        let timestamp_wad = ethers::types::U256::from(next_timestamp);
        let update_data = self
            .ln_solver
            .prepare_strike_update(target_strike_wad, timestamp_wad)
            .call()
            .await?;
        let tx = self
            .protocol
            .update(pool_id, update_data)
            .send()
            .await?
            .await?;
        Ok(tx)
    }

    pub async fn set_weight_x(
        &self,
        pool_id: U256,
        target_wx: f64,
        next_timestamp: u64,
    ) -> Result<Option<TransactionReceipt>> {
        let target_wx_wad = to_wad(target_wx);
        let timestamp_wad = U256::from(next_timestamp);
        let update_data = self
            .g_solver
            .prepare_weight_x_update(target_wx_wad, timestamp_wad)
            .call()
            .await?;
        let tx = self
            .protocol
            .update(pool_id, update_data)
            .send()
            .await?
            .await?;
        Ok(tx)
    }
}

fn to_wad(value: f64) -> U256 {
    parse_ether(value).unwrap()
}

fn to_init_params_wad(init_params: PoolInitParamsF64) -> Result<PoolParams> {
    match init_params {
        PoolInitParamsF64::G3M(g3m_params) => Ok(PoolParams::G3M(G3Mparams {
            w_x: to_wad(g3m_params.wx),
            w_y: to_wad(1.0) - to_wad(g3m_params.wx),
            swap_fee: to_wad(g3m_params.swap_fee),
            controller: g3m_params.controller,
        })),
        PoolInitParamsF64::LogNormal(ln_params) => Ok(PoolParams::LogNormal(LogNormalParams {
            sigma: to_wad(ln_params.sigma),
            strike: to_wad(ln_params.strike),
            tau: to_wad(ln_params.tau),
            swap_fee: to_wad(ln_params.swap_fee),
            controller: ln_params.controller,
        })),
    }
}
