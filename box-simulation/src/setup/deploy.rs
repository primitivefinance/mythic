use anyhow::Result;
use arbiter_core::bindings::arbiter_token::ArbiterToken;
use arbiter_core::bindings::liquid_exchange::LiquidExchange;
use arbiter_core::data_collection::EventLogger;
use arbiter_core::environment::Environment;
use arbiter_core::middleware::RevmMiddleware;
use bindings::g3m::G3M;
use ethers::types::U256;

pub struct ContractInstances {
    pub arbx: ArbiterToken<RevmMiddleware>,
    pub arby: ArbiterToken<RevmMiddleware>,
    pub g3m: G3M<RevmMiddleware>,
    pub lex: LiquidExchange<RevmMiddleware>,
}

pub async fn deploy_contracts(env: &Environment) -> Result<ContractInstances, anyhow::Error> {
    let deployer = RevmMiddleware::new(&env, "deployer".into())?;

    let arbx = ArbiterToken::deploy(
        deployer.clone(),
        ("Arbiter Token X".to_string(), "arbx".to_string(), 18),
    )?
    .send()
    .await?;
    let arby = ArbiterToken::deploy(
        deployer.clone(),
        ("Arbiter Token Y".to_string(), "arby".to_string(), 18),
    )?
    .send()
    .await?;

    let g3m = G3M::deploy(
        deployer.clone(),
        (
            arbx.address(),
            arby.address(),
            U256::from(10u64.pow(18) / 2),
        ),
    )?
    .send()
    .await?;
    let lex = LiquidExchange::deploy(
        deployer.clone(),
        (arbx.address(), arby.address(), g3m.address()),
    )?
    .send()
    .await?;

    EventLogger::builder()
        .add(arbx.events(), "arbx")
        .add(arby.events(), "arby")
        .run()
        .await?;
    // let g3m_listener = EventLogger::builder().add(g3m.events(), "g3m").run().await?;

    Ok(ContractInstances {
        arbx,
        arby,
        g3m,
        lex,
    })
}
