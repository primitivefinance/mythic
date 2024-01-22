//! Dynamic Function Market Making Protocol Client
//!
//! Middleware layer for agents to communicate with the DFMM protocol.
mod pool;

use std::{collections::BTreeMap, sync::Arc, time::Duration};

use anyhow::Result;
use bindings::{
    dfmm::DFMM,
    g3m::G3M,
    g3m_solver::{G3MSolver, PublicParams as G3mParameters},
    log_normal::LogNormal,
    log_normal_solver::{LogNormalSolver, PublicParams as LogNormalParameters},
    shared_types::InitParams,
};
use datatypes::TokenData;
use pool::{Pool, PoolKind};

use super::*;

#[derive(Debug, Clone)]
pub struct G3mF64 {
    pub wx: f64,
    pub swap_fee: f64,
}

#[derive(Debug, Clone)]
pub struct LogNormalF64 {
    pub sigma: f64,
    pub strike: f64,
    pub tau: f64,
    pub swap_fee: f64,
}

#[derive(Debug, Clone)]
pub enum PoolInitParamsF64 {
    G3M(G3mF64),
    LogNormal(LogNormalF64),
}

#[derive(Debug, Clone)]
pub enum PoolParams {
    G3M(G3mParameters),
    LogNormal(LogNormalParameters),
}

#[derive(Debug)]
pub struct ProtocolClient<C> {
    pub client: Arc<C>,
    pub protocol: DFMM<C>,
    pub ln_solver: LogNormalSolver<C>,
    pub ln_strategy: LogNormal<C>,
    pub g_solver: G3MSolver<C>,
    pub g_strategy: G3M<C>,
    pub pools: BTreeMap<U256, Pool>,
    pub tokens: BTreeMap<Address, TokenData>,
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
            pools: self.pools.clone(),
            tokens: self.tokens.clone(),
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
    ) -> anyhow::Result<Self> {
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
            pools: BTreeMap::new(),
            tokens: BTreeMap::new(),
        })
    }

    pub fn from(
        client: Arc<C>,
        protocol_address: Address,
        ln_solver_address: Address,
        ln_strategy_address: Address,
        g_solver_address: Address,
        g_strategy_address: Address,
    ) -> Result<Self> {
        let protocol = DFMM::new(protocol_address, client.clone());
        let ln_strategy = LogNormal::new(ln_strategy_address, client.clone());
        let g_strategy = G3M::new(g_strategy_address, client.clone());
        let ln_solver = LogNormalSolver::new(ln_solver_address, client.clone());
        let g_solver = G3MSolver::new(g_solver_address, client.clone());

        // todo: get protocol nonce then loop through pools to generate token list and
        // pool list
        Ok(Self {
            client,
            protocol,
            ln_solver,
            ln_strategy,
            g_solver,
            g_strategy,
            pools: BTreeMap::new(),
            tokens: BTreeMap::new(),
        })
    }

    pub fn bind(self, client: Arc<C>) -> anyhow::Result<Self> {
        let protocol = DFMM::new(self.protocol.address(), client.clone());
        let ln_strategy = LogNormal::new(self.ln_strategy.address(), client.clone());
        let g_strategy = G3M::new(self.g_strategy.address(), client.clone());
        let ln_solver = LogNormalSolver::new(self.ln_solver.address(), client.clone());
        let g_solver = G3MSolver::new(self.g_solver.address(), client.clone());
        let pools = self.pools.clone();
        let mut tokens = BTreeMap::new();
        for (key, value) in self.tokens.iter() {
            let token = value.clone();
            tokens.insert(*key, token);
        }

        Ok(Self {
            client,
            protocol,
            ln_strategy,
            ln_solver,
            g_strategy,
            g_solver,
            pools,
            tokens,
        })
    }

    pub fn add_token(
        &mut self,
        address: Address,
        name: String,
        symbol: String,
        decimals: u8,
    ) -> Result<()> {
        let token_data = TokenData {
            name,
            symbol,
            decimals,
        };
        self.tokens.insert(address, token_data).unwrap();
        Ok(())
    }

    pub fn get_token(&self, address: Address) -> Result<&TokenData> {
        Ok(self.tokens.get(&address).unwrap())
    }

    pub fn get_token_by_symbol(&self, symbol: String) -> Result<&TokenData> {
        let (_address, token) = self
            .tokens
            .iter()
            .find(|(_, v)| v.symbol == symbol)
            .unwrap();
        Ok(token)
    }

    pub fn get_token_by_name(&self, name: String) -> Result<&TokenData> {
        let (_address, token) = self.tokens.iter().find(|(_k, v)| v.name == name).unwrap();
        Ok(token)
    }

    pub fn get_pool_tokens(&self, pool: Pool) -> Result<(&TokenData, &TokenData)> {
        Ok((self.get_token(pool.token_x)?, self.get_token(pool.token_y)?))
    }

    #[tracing::instrument(skip(self), level = "trace", ret)]
    pub async fn init_pool(
        &mut self,
        token_x: Address,
        token_y: Address,
        init_reserve_x_wad: U256,
        init_price_wad: U256,
        init_params: PoolInitParamsF64,
    ) -> Result<TransactionReceipt> {
        let pool_params = to_init_params_wad(init_params)?;
        let pool_id = self.get_next_pool_id().await?;

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

                let pool = Pool {
                    kind: PoolKind::G3M,
                    token_x,
                    token_y,
                    pool_id,
                };
                self.pools.insert(pool_id, pool).unwrap();

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

                let pool = Pool {
                    kind: PoolKind::LogNormal,
                    token_x,
                    token_y,
                    pool_id,
                };
                self.pools.insert(pool_id, pool).unwrap();

                Ok(tx)
            }
        }
    }

    async fn get_next_pool_id(&self) -> Result<U256> {
        let pool_id = self.protocol.nonce().call().await?;
        Ok(pool_id)
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

    #[tracing::instrument(skip(self), level = "trace", ret)]
    pub async fn get_internal_price(&self, pool_id: U256) -> Result<U256> {
        let kind = self.pools.get(&pool_id).unwrap().kind;
        match kind {
            PoolKind::G3M => Ok(self.g_solver.internal_price(pool_id).call().await?),
            PoolKind::LogNormal => Ok(self.ln_solver.internal_price(pool_id).call().await?),
        }
    }

    #[tracing::instrument(skip(self), level = "trace", ret)]
    pub async fn get_params(&self, pool_id: U256) -> Result<PoolParams> {
        let pool = self.pools.get(&pool_id).unwrap();

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
        target_strike_price_wad: f64,
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

    pub async fn set_weight_x(
        &self,
        pool_id: U256,
        target_wx: f64,
        next_timestamp: u64,
    ) -> Result<Option<TransactionReceipt>> {
        let target_wx_wad = to_wad(target_wx);
        let tx = self
            .g_strategy
            .set_weight_x(
                pool_id,
                target_wx_wad,
                ethers::types::U256::from(next_timestamp),
            )
            .send()
            .await?
            .await?;
        Ok(tx)
    }
}

fn to_wad(value: f64) -> U256 {
    ethers::utils::parse_ether(value).unwrap()
}

fn to_init_params_wad(init_params: PoolInitParamsF64) -> Result<PoolParams> {
    match init_params {
        PoolInitParamsF64::G3M(g3m_params) => Ok(PoolParams::G3M(G3mParameters {
            w_x: to_wad(g3m_params.wx),
            w_y: to_wad(1.0) - to_wad(g3m_params.wx),
            swap_fee: to_wad(g3m_params.swap_fee),
        })),
        PoolInitParamsF64::LogNormal(ln_params) => Ok(PoolParams::LogNormal(LogNormalParameters {
            sigma: to_wad(ln_params.sigma),
            strike: to_wad(ln_params.strike),
            tau: to_wad(ln_params.tau),
            swap_fee: to_wad(ln_params.swap_fee),
        })),
    }
}
