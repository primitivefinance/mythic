use arbiter_core::data_collection::EventLogger;
use clients::protocol::ProtocolClient;
use revm::db::{CacheDB, EmptyDB};

use self::agents::portfolio_management_agents::{g3m::g3m_setup, lognormal::ln_setup};

use super::*;
use crate::{
    agent::Agents,
    agents::base_agents::{
        block_admin::BlockAdmin, price_changer::PriceChanger, token_admin::TokenAdmin,
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

        let price_changer =
            PriceChanger::new(&environment, &config, "price_changer", &token_admin).await?;
        let steps = price_changer.trajectory.paths[0].len() - 1;

        let lex = price_changer.liquid_exchange.address();
        let lex_events = price_changer.liquid_exchange.events();
        agents.add(price_changer);

        let base_client = RevmMiddleware::new(&environment, "base".into());
        let base_protocol_client = ProtocolClient::new(
            base_client.clone(),
            token_admin.arbx.address(),
            token_admin.arby.address(),
            0.003,
        )
        .await?;

        let g3m_pool_id = base_protocol_client.get_next_pool_id().await?;

        let (g3m_lp, g3m_arb, g3m_manager) = g3m_setup(
            &environment,
            &config,
            base_protocol_client.clone(),
            lex.address(),
            &token_admin,
            g3m_pool_id,
        );
        agents.add(g3m_lp);
        agents.add(g3m_arb);
        agents.add(g3m_manager);

        let ln_pool_id = base_protocol_client.get_next_pool_id().await?;

        let (ln_lp, ln_arb, ln_manager) = ln_setup(
            &environment,
            &config,
            base_protocol_client.clone(),
            lex.address(),
            &token_admin,
            ln_pool_id,
        );
        agents.add(ln_lp);
        agents.add(ln_arb);
        agents.add(ln_manager);

        EventLogger::builder()
            .directory(config.output_directory.clone())
            .file_name(config.output_file_name.clone().unwrap())
            .add(lex_events, "lex")
            .add(base_protocol_client.protocol.address(), "dfmm")
            .add(token_admin.arbx.events(), "arbx")
            .add(token_admin.arby.events(), "arby")
            .add(g3m_arb.atomic_arbitrage.events(), "ln_atomic_arbitrage")
            .add(ln_arb.atomic_arbitrage.events(), "g3m_atomic_arbitrage")
            .run()
            .map_err(|e| SimulationError::GenericError(e.to_string()))?;

        Ok((agents, steps, environment))
    }
}
