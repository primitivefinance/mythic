use anyhow::Result;
use arbiter_core::bindings::arbiter_token::ArbiterToken;
use arbiter_core::bindings::liquid_exchange::LiquidExchange;
use arbiter_core::data_collection::EventLogger;
use arbiter_core::environment::Environment;
use arbiter_core::middleware::RevmMiddleware;
use arbiter_derive::Deploy;
use bindings::g3m::G3M;
use ethers::prelude::*;
use ethers::types::U256;
use ethers::utils::parse_ether;
use std::sync::Arc;

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
    pub lex:
        ContractDeploymentTx<Arc<RevmMiddleware>, RevmMiddleware, LiquidExchange<RevmMiddleware>>,
}

pub struct Contracts {
    pub tokens: TokensDeployed,
    pub exchanges: ExchangesDeployed,
}

pub async fn deploy_contracts(env: &Environment) -> Result<Contracts, anyhow::Error> {
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

    let g3m_args = (
        tokens.arbx.address(),
        tokens.arby.address(),
        U256::from(10u64.pow(18) / 2),
    );

    let lex_args = (
        tokens.arbx.address(),
        tokens.arby.address(),
        parse_ether(1).unwrap(),
    );

    let exchanges = Exchanges {
        g3m: G3M::deploy(deployer.clone(), g3m_args)?,
        lex: LiquidExchange::deploy(deployer.clone(), lex_args)?,
    }
    .deploy()
    .await?;

    let contracts = Contracts { tokens, exchanges };

    // let g3m_listener = EventLogger::builder().add(g3m.events(), "g3m").run().await?;

    Ok(contracts)
}
