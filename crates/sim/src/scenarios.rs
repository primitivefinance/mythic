use arbiter_core::data_collection::EventLogger;
use revm::db::{CacheDB, EmptyDB};

use super::*;
use crate::{
    agent::Agents,
    agents::{
        base::{block_admin::BlockAdmin, price_changer::PriceChanger, token_admin::TokenAdmin},
        pm::{
            arbitrageur::Arbitrageur, g3m_arbitrageur::G3mArbitrageur,
            liquidity_provider::LiquidityProvider, submitter::VolatilityTargetingSubmitter,
        },
    },
};

/// Implements the scenario by adding the chosen agents to the simulation's
/// environment.
#[async_trait::async_trait]
pub trait Scenario: Clone + Sync + Send + Sized + Any {
    async fn setup(
        &self,
        db: Option<CacheDB<EmptyDB>>,
        environment: Environment,
        config: SimulationConfig<Single>,
    ) -> Result<(Agents, usize, Environment), SimulationError>;
}

#[derive(Debug, Clone)]
pub struct BasicScenario;

#[async_trait::async_trait]
impl Scenario for BasicScenario {
    async fn setup(
        &self,
        db: Option<CacheDB<EmptyDB>>,
        environment: Environment,
        config: SimulationConfig<Single>,
    ) -> Result<(Agents, usize, Environment), SimulationError> {
        let steps = 10;
        let mut agents = Agents::new();

        let block_admin = BlockAdmin::new(db, &environment, &config, "block_admin").await?;
        agents.add(block_admin);

        Ok((agents, steps, environment))
    }
}

#[derive(Debug, Clone)]
pub struct DFMMScenario;

#[async_trait::async_trait]
impl Scenario for DFMMScenario {
    async fn setup(
        &self,
        db: Option<CacheDB<EmptyDB>>,
        environment: Environment,
        config: SimulationConfig<Single>,
    ) -> Result<(Agents, usize, Environment), SimulationError> {
        let mut agents = Agents::new();

        let block_admin = BlockAdmin::new(db, &environment, &config, "block_admin").await?;
        agents.add(block_admin);

        let token_admin = TokenAdmin::new(&environment, &config, "token_admin").await?;
        agents.add(token_admin.clone());

        // ----- Scenario Specific ----- //

        // 1. Price changer deploys a Liquid Exchange.
        let price_changer =
            PriceChanger::new(&environment, &config, "price_changer", &token_admin).await?;
        let steps = price_changer.trajectory.paths[0].len() - 1;

        let lex = from_ethers_address(price_changer.liquid_exchange.address());
        let lex_events = price_changer.liquid_exchange.events();
        agents.add(price_changer);

        // 2. Portfolio manager deploys a Dynamic Function MM & updates its parameters.
        let pm = VolatilityTargetingSubmitter::new(
            &environment,
            &config,
            "portfolio_manager",
            lex,
            token_admin.arbx.address(),
            token_admin.arby.address(),
        )
        .await?;

        let protocol_client = pm.protocol_client.clone();
        let market_events = pm.protocol_client.protocol.events();
        agents.add(pm);

        // 3. Liquidity provider initializes the DFMM.
        let lp = LiquidityProvider::new(
            &environment,
            &config,
            "lp",
            &token_admin,
            protocol_client.clone(),
        )
        .await?;
        agents.add(lp);

        // 4. Arbitrageur arbitrages between the DFMM and the Liquid Exchange.
        let arbitrageur =
            Arbitrageur::new(&environment, &token_admin, lex, protocol_client.clone()).await?;
        agents.add(arbitrageur.clone());

        let g3m_arbitrageur =
            G3mArbitrageur::new(&environment, &token_admin, lex, protocol_client.clone()).await?;
        agents.add(g3m_arbitrageur.clone());

        EventLogger::builder()
            .directory(config.output_directory.clone())
            .file_name(config.output_file_name.clone().unwrap())
            .add(lex_events, "lex")
            .add(market_events, "dfmm")
            .add(token_admin.arbx.events(), "arbx")
            .add(token_admin.arby.events(), "arby")
            .add(arbitrageur.atomic_arbitrage.events(), "ln_atomic_arbitrage")
            .add(
                g3m_arbitrageur.atomic_arbitrage.events(),
                "g3m_atomic_arbitrage",
            )
            .run()
            .map_err(|e| SimulationError::GenericError(e.to_string()))?;

        Ok((agents, steps, environment))
    }
}
