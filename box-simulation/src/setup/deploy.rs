use anyhow::Result;
use arbiter_core::bindings::{arbiter_token::ArbiterToken, liquid_exchange::LiquidExchange};
use arbiter_core::data_collection::EventLogger;
use arbiter_core::environment::Environment;
use arbiter_core::middleware::RevmMiddleware;
use arbiter_derive::Deploy;
use bindings::g3m::G3M;
use ethers::prelude::*;
use ethers::types::U256;
use ethers::utils::parse_ether;
use std::sync::Arc;

use crate::settings::params::SimulationConfig;

#[derive(Deploy)]
pub struct Tokens {
    pub arbx:
        ContractDeploymentTx<Arc<RevmMiddleware>, RevmMiddleware, ArbiterToken<RevmMiddleware>>,
    pub arby:
        ContractDeploymentTx<Arc<RevmMiddleware>, RevmMiddleware, ArbiterToken<RevmMiddleware>>,
}

#[derive(Deploy)]
pub struct Exchanges {
    pub g3m: ContractDeploymentTx<Arc<RevmMiddleware>, RevmMiddleware, G3M<RevmMiddleware>>,
    // pub lex:
    //     ContractDeploymentTx<Arc<RevmMiddleware>, RevmMiddleware, LiquidExchange<RevmMiddleware>>,
}

pub struct Contracts {
    pub deployer: Arc<RevmMiddleware>,
    pub tokens: TokensDeployed,
    pub exchanges: ExchangesDeployed,
}

pub async fn deploy_contracts(
    env: &Environment,
    config: &SimulationConfig,
) -> Result<Contracts, anyhow::Error> {
    let deployer = RevmMiddleware::new(&env, "deployer".into())?;
    let decimals = u8::from(18);
    let arbx_args = ("Arbiter Token X".to_string(), "arbx".to_string(), decimals);
    let arby_args = ("Arbiter Token Y".to_string(), "arby".to_string(), decimals);

    let tokens = Tokens {
        arbx: ArbiterToken::deploy(deployer.clone(), arbx_args)?,
        arby: ArbiterToken::deploy(deployer.clone(), arby_args)?,
    }
    .deploy()
    .await?;

    let initial_weight_float = config.portfolio_pool_parameters.weight_token_0;
    println!("Initial weight float: {}", initial_weight_float);
    let initial_weight =
        ethers::utils::parse_ether(config.portfolio_pool_parameters.weight_token_0)?;
    let initial_fee_bps = U256::from(config.portfolio_pool_parameters.fee_basis_points);

    let g3m_args = (
        tokens.arbx.address(), //tokena
        tokens.arby.address(), //tokenb
        initial_weight,        //weight
        initial_fee_bps,
    );

    println!(
        "G3M args: {:?}",
        (
            tokens.arbx.address(),
            tokens.arby.address(),
            initial_weight,
            initial_fee_bps
        )
    );

    let lex_args = (
        tokens.arbx.address(),
        tokens.arby.address(),
        parse_ether(1)?,
    );

    let exchanges = Exchanges {
        g3m: G3M::deploy(deployer.clone(), g3m_args)?,
        // lex: LiquidExchange::deploy(deployer.clone(), lex_args)?,
    }
    .deploy()
    .await?;

    let contracts = Contracts {
        deployer,
        tokens,
        exchanges,
    };

    // agents
    // 1. arbitraguer :check:
    // 2. rebalancer

    Ok(contracts)
}
