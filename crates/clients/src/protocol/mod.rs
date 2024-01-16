//! Dynamic Function Market Making Protocol Client
//!
//! Middleware layer for agents to communicate with the DFMM protocol.
use std::{sync::Arc, time::Duration};

use anyhow::Result;
use bindings::{
    dfmm::DFMM,
    g3m::G3M,
    g3m_solver::{G3MSolver, PublicParams as G3Mparameters},
    log_normal::LogNormal,
    log_normal_solver::{LogNormalSolver, PublicParams as LogNormParameters},
    shared_types::InitParams,
};
use tracing::debug;

use super::*;

#[derive(Debug)]
pub struct ProtocolClient<C> {
    pub client: Arc<C>,
    pub protocol: DFMM<C>,
    pub ln_strategy: LogNormal<C>,
    pub ln_solver: LogNormalSolver<C>,
    pub g_strategy: G3M<C>,
    pub g_solver: G3MSolver<C>,
    pub token_x: Address,
    pub token_y: Address,
}

pub struct ProtocolClientBuilder<C> {
    pub client: Arc<C>,
    pub protocol: Option<DFMM<C>>,
    pub ln_strategy: Option<LogNormal<C>>,
    pub ln_solver: Option<LogNormalSolver<C>>,
    pub g_strategy: Option<G3M<C>>,
    pub g_solver: Option<G3MSolver<C>>,
    pub token_x: Option<Address>,
    pub token_y: Option<Address>,
}

impl<C: Middleware + 'static> ProtocolClientBuilder<C> {
    pub fn protocol(mut self, protocol: Address) -> Self {
        self.protocol = Some(DFMM::new(protocol, self.client.clone()));
        self
    }

    pub fn ln_strategy(mut self, ln_strategy: Address) -> Self {
        self.ln_strategy = Some(LogNormal::new(ln_strategy, self.client.clone()));
        self
    }

    pub fn ln_solver(mut self, ln_solver: Address) -> Self {
        self.ln_solver = Some(LogNormalSolver::new(ln_solver, self.client.clone()));
        self
    }

    pub fn g_strategy(mut self, g_strategy: Address) -> Self {
        self.g_strategy = Some(G3M::new(g_strategy, self.client.clone()));
        self
    }

    pub fn g_solver(mut self, g_solver: Address) -> Self {
        self.g_solver = Some(G3MSolver::new(g_solver, self.client.clone()));
        self
    }

    pub fn build(self) -> Result<ProtocolClient<C>> {
        let protocol = self.protocol.unwrap();
        let ln_strategy = self.ln_strategy.unwrap();
        let ln_solver = self.ln_solver.unwrap();
        let g_strategy = self.g_strategy.unwrap();
        let g_solver = self.g_solver.unwrap();
        let token_x = self.token_x.unwrap();
        let token_y = self.token_y.unwrap();

        Ok(ProtocolClient {
            client: self.client,
            protocol,
            ln_strategy,
            ln_solver,
            g_strategy,
            g_solver,
            token_x,
            token_y,
        })
    }
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
            token_x: self.token_x.clone(),
            token_y: self.token_y.clone(),
        }
    }
}

type F64Wad = f64;

impl<C: Middleware + 'static> ProtocolClient<C> {
    pub fn builder(client: Arc<C>) -> ProtocolClientBuilder<C> {
        ProtocolClientBuilder {
            client,
            protocol: None,
            ln_strategy: None,
            ln_solver: None,
            g_strategy: None,
            g_solver: None,
            token_x: None,
            token_y: None,
        }
    }

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
            ln_strategy,
            ln_solver,
            g_strategy,
            g_solver,
            token_x,
            token_y,
        })
    }

    pub async fn bind(self, client: Arc<C>) -> anyhow::Result<Self> {
        let protocol = DFMM::new(self.protocol.address(), client.clone());
        let ln_strategy = LogNormal::new(self.ln_strategy.address(), client.clone());
        let g_strategy = G3M::new(self.g_strategy.address(), client.clone());
        let ln_solver = LogNormalSolver::new(self.ln_solver.address(), client.clone());
        let g_solver = G3MSolver::new(self.g_solver.address(), client.clone());

        Ok(Self {
            client,
            protocol,
            ln_strategy,
            ln_solver,
            g_strategy,
            g_solver,
            token_x: self.token_x,
            token_y: self.token_y,
        })
    }

    pub fn get_tokens(&self) -> Result<(Address, Address)> {
        Ok((self.token_x, self.token_y))
    }

    #[tracing::instrument(skip(self), level = "trace", ret)]
    pub async fn get_swap_fee(&self, pool_id: U256) -> Result<U256> {
        let params = self.ln_solver.get_pool_params(pool_id).call().await?;
        Ok(params.swap_fee)
    }

    #[tracing::instrument(skip(self), level = "trace", ret)]
    pub async fn get_ln_init_payload(
        &self,
        init_reserve_x_wad: U256,
        init_price_wad: U256,
        init_params: LogNormParameters,
    ) -> Result<InitParams> {
        let init_data = self
            .ln_solver
            .get_initial_pool_data(init_reserve_x_wad, init_price_wad, init_params)
            .call()
            .await?;
        let init_params: InitParams = InitParams {
            strategy: self.ln_strategy.address(),
            token_x: self.token_x,
            token_y: self.token_y,
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
        let init_data = self
            .g_solver
            .get_initial_pool_data(init_reserve_x_wad, init_price_wad, init_params)
            .call()
            .await?;
        let init_params: InitParams = InitParams {
            strategy: self.g_strategy.address(),
            token_x: self.token_x,
            token_y: self.token_y,
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
        swap_fee_percent_wad: F64Wad,
    ) -> Result<Option<TransactionReceipt>> {
        debug!("Initializing DFMM from protocol client.");

        // Format the parameters for the log-normal strategy.
        let params: LogNormParameters = LogNormParameters {
            strike: to_wad(strike_price_wad),
            sigma: to_wad(sigma_percent_wad),
            tau: to_wad(tau_years_wad),
            swap_fee: to_wad(swap_fee_percent_wad),
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

    #[tracing::instrument(skip(self), level = "debug")]
    pub async fn initialize_g_pool(
        &self,
        init_reserve_x_wad: F64Wad,
        init_price_wad: F64Wad,
        init_weight_x: F64Wad,
        swap_fee_percent_wad: F64Wad,
    ) -> Result<Option<TransactionReceipt>> {
        debug!("Initializing DFMM from protocol client.");

        // Format the parameters for the log-normal strategy.
        let params: G3Mparameters = G3Mparameters {
            w_x: to_wad(init_weight_x),
            w_y: to_wad(1.0 - init_weight_x),
            swap_fee: to_wad(swap_fee_percent_wad),
        };
        println!("reserve x wad: {}", init_reserve_x_wad);

        let init_reserve_x_wad = to_wad(init_reserve_x_wad);
        let init_price_wad = to_wad(init_price_wad);

        // Encode the data together to send it to the DFMM protocol.
        let payload = self
            .get_g_init_payload(init_reserve_x_wad, init_price_wad, params.clone())
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
        let pool_data = self.ln_solver.get_pool_params(pool_id).call().await?;
        Ok(pool_data.strike)
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

    pub async fn set_weight_x(
        &self,
        pool_id: U256,
        target_wx: F64Wad,
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

fn to_wad(value: F64Wad) -> U256 {
    ethers::utils::parse_ether(value).unwrap()
}
