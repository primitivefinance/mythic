use alloy_primitives::Address;
use arbiter_core::data_collection::EventLogger;
use revm::db::{CacheDB, EmptyDB};

use super::*;
use crate::{
    agent::Agents,
    agents::{
        base::{block_admin::BlockAdmin, price_changer::PriceChanger, token_admin::TokenAdmin},
        pm::{arbitrageur::Arbitrageur, liquidity_provider::LiquidityProvider},
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

        let lp = LiquidityProvider::new(&environment, &config, "lp", &token_admin, Address::ZERO)
            .await?;
        agents.add(lp);

        let arbitrageur = Arbitrageur::new(
            &environment,
            &token_admin,
            from_ethers_address(price_changer.liquid_exchange.address()),
            Address::ZERO,
        )
        .await?;
        agents.add(arbitrageur.clone());

        //.add(rmm_events, "rmm")

        EventLogger::builder()
            .directory(config.output_directory.clone())
            .file_name(config.output_file_name.clone().unwrap())
            .add(price_changer.liquid_exchange.events(), "lex")
            .add(token_admin.arbx.events(), "arbx")
            .add(token_admin.arby.events(), "arby")
            .add(arbitrageur.atomic_arbitrage.events(), "atomic_arbitrage")
            .run()
            .map_err(|e| SimulationError::GenericError(e.to_string()))?;

        Ok((agents, steps, environment))
    }
}
