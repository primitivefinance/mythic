pub mod config;

use std::sync::Arc;

use arbiter_core::{
    data_collection::EventLogger,
    environment::{builder::EnvironmentBuilder, Environment},
};
use simulation::{
    agents::{
        block_admin::BlockAdmin,
        g3m::{
            arbitrageur::Arbitrageur,
            g3m_portfolio_manager::{G3mPortfolioManager, G3mPortfolioManagerType},
            liquidity_provider::LiquidityProvider,
            swapper::Swapper,
        },
        price_changer::PriceChanger,
        token_admin::TokenAdmin,
        Agents,
    },
    settings::{parameters::Single, SimulationConfig},
    simulations::errors::SimulationError,
    strategy::g3m::G3mStrategy,
};
use tokio::{
    runtime::Builder,
    sync::{broadcast, Mutex, Semaphore},
};

use self::config::ConfigBuilder;
use super::world::{self, spawn_tasks, World, WorldManager};

pub type SpawnedManager = anyhow::Result<Arc<Mutex<WorldManager>>, Arc<anyhow::Error>>;
pub type InstanceManager = Arc<Mutex<WorldManager>>;

#[tracing::instrument]
pub async fn spawn(builders: Vec<MiniWorldBuilder>) -> SpawnedManager {
    // Override the world manager with a new one that has spawned worlds.
    Ok(Arc::new(Mutex::new(
        manager(builders).await.map_err(|e| Arc::new(e))?,
    )))
}

/// Need to get the world manager into the UI, with arc and mutex.
/// Need a custom World to be created.
/// We can create a new world manager with its own worlds, tx, slice, so just
/// need to make a spawn function to do that.
/// To use this manager, make your world builders and pass them into it.
/// Making a world builder lets you customize the configuration beforehand.
#[tracing::instrument]
pub async fn manager(
    builders: Vec<MiniWorldBuilder>,
) -> anyhow::Result<WorldManager, anyhow::Error> {
    let (tx, worlds, slice) = spawn_worlds(builders).await?;

    tracing::info!("Worlds spawned, exiting spawn fn.");
    Ok(WorldManager::new(
        worlds,
        Some(Arc::new(Mutex::new(tx))),
        Some(Arc::new(Mutex::new(slice))),
    ))
}

/// Constructs a World instance from a config builder and environment builder.
#[derive(Debug, Clone)]
pub struct MiniWorldBuilder {
    pub config_builder: ConfigBuilder,
    pub environment_builder: EnvironmentBuilder,
    pub seed: u64, // Add a field for the seed
}

impl Default for MiniWorldBuilder {
    fn default() -> Self {
        Self {
            config_builder: ConfigBuilder::default(),
            environment_builder: EnvironmentBuilder::new(),
            seed: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AgentBuilder;

impl AgentBuilder {
    pub async fn build(
        environment: &Environment,
        config: &SimulationConfig<Single>,
    ) -> anyhow::Result<Agents> {
        let mut agents = Agents::new();
        let mut event_logger = EventLogger::builder()
            .directory(config.output_directory.clone())
            .file_name(config.output_file_name.clone().unwrap());

        let block_admin = BlockAdmin::new(&environment, &config, "block_admin").await?;
        agents.add(block_admin);

        let token_admin = TokenAdmin::new(&environment, &config, "token_admin").await?;
        agents.add(token_admin.clone());

        let price_changer =
            PriceChanger::new(&environment, &config, "price_changer", &token_admin).await?;
        agents.add(price_changer.clone());
        event_logger = event_logger.add(price_changer.liquid_exchange.events(), "lex");

        let portfolio_manager = G3mPortfolioManagerType::new(
            &environment,
            &config,
            "portfolio_manager",
            price_changer.liquid_exchange.address(),
        )
        .await?;
        let strategy_address = portfolio_manager.g3m().address();
        event_logger = event_logger.add(portfolio_manager.g3m().events(), "g3m");
        agents.add(portfolio_manager);

        let lp = LiquidityProvider::<G3mStrategy>::new(
            &environment,
            &config,
            "lp",
            &token_admin,
            strategy_address,
        )
        .await?;
        agents.add(lp);

        let arbitrageur = Arbitrageur::<G3mStrategy>::new(
            &environment,
            &token_admin,
            price_changer.liquid_exchange.address(),
            strategy_address,
        )
        .await?;
        agents.add(arbitrageur);

        match Swapper::new(
            &environment,
            &config,
            "swapper",
            &price_changer,
            &token_admin,
        )
        .await
        {
            Ok(swapper) => {
                agents.add(swapper.clone());
                event_logger =
                    event_logger.add(swapper.portfolio_tracker.events(), "portfolio_tracker");
            }
            Err(e) => {
                tracing::warn!("Swapper not initialized: {}", e);
            }
        };

        event_logger
            .metadata(config)
            .map_err(|e| SimulationError::GenericError(e.to_string()))?
            .run()
            .map_err(|e| SimulationError::GenericError(e.to_string()))?;
        let steps = price_changer.trajectory.paths[0].len() - 1;

        Ok(agents)
    }
}

impl MiniWorldBuilder {
    pub async fn setup(
        environment: &Environment,
        config: &SimulationConfig<Single>,
    ) -> anyhow::Result<Agents> {
        let mut agents = Agents::new();
        let mut event_logger = EventLogger::builder()
            .directory(config.output_directory.clone())
            .file_name(config.output_file_name.clone().unwrap());

        let block_admin = BlockAdmin::new(&environment, &config, "block_admin").await?;
        agents.add(block_admin);

        let token_admin = TokenAdmin::new(&environment, &config, "token_admin").await?;
        agents.add(token_admin.clone());

        let price_changer =
            PriceChanger::new(&environment, &config, "price_changer", &token_admin).await?;
        agents.add(price_changer.clone());
        event_logger = event_logger.add(price_changer.liquid_exchange.events(), "lex");

        let portfolio_manager = G3mPortfolioManagerType::new(
            &environment,
            &config,
            "portfolio_manager",
            price_changer.liquid_exchange.address(),
        )
        .await?;
        let strategy_address = portfolio_manager.g3m().address();
        event_logger = event_logger.add(portfolio_manager.g3m().events(), "g3m");
        agents.add(portfolio_manager);

        let lp = LiquidityProvider::<G3mStrategy>::new(
            &environment,
            &config,
            "lp",
            &token_admin,
            strategy_address,
        )
        .await?;
        agents.add(lp);

        let arbitrageur = Arbitrageur::<G3mStrategy>::new(
            &environment,
            &token_admin,
            price_changer.liquid_exchange.address(),
            strategy_address,
        )
        .await?;
        agents.add(arbitrageur);

        match Swapper::new(
            &environment,
            &config,
            "swapper",
            &price_changer,
            &token_admin,
        )
        .await
        {
            Ok(swapper) => {
                agents.add(swapper.clone());
                event_logger =
                    event_logger.add(swapper.portfolio_tracker.events(), "portfolio_tracker");
            }
            Err(e) => {
                tracing::warn!("Swapper not initialized: {}", e);
            }
        };

        event_logger
            .metadata(config)
            .map_err(|e| SimulationError::GenericError(e.to_string()))?
            .run()
            .map_err(|e| SimulationError::GenericError(e.to_string()))?;
        let steps = price_changer.trajectory.paths[0].len() - 1;

        Ok(agents)
    }

    pub async fn build(self) -> anyhow::Result<World> {
        let seed = self.seed;
        let environment = self.environment_builder.build();
        let config = self.config_builder.build();

        // todo: this is a hack to get the first config
        let direct_configs: Vec<SimulationConfig<Single>> = config.clone().into();
        let mut agents = Agents::new();
        let mut count = 0;
        for config in direct_configs {
            if count > 0 {
                break;
            }
            tracing::info!("Setting up agents for config");
            agents = AgentBuilder::build(&environment, &config).await?;
            count += 1;
        }

        tracing::info!("World built.");

        Ok(World {
            arbiter: environment,
            agents: Arc::new(Mutex::new(agents)),
            state: world::State::new(),
            config,
            seed,
        })
    }

    async fn build_arc(self) -> anyhow::Result<Arc<Mutex<World>>> {
        let world = self.build().await?;
        Ok(Arc::new(Mutex::new(world)))
    }
}

#[tracing::instrument(skip(builders))]
pub async fn spawn_worlds(
    builders: Vec<MiniWorldBuilder>,
) -> anyhow::Result<
    (
        broadcast::Sender<usize>,
        Vec<Arc<Mutex<World>>>,
        std::thread::JoinHandle<Result<Vec<Arc<Mutex<World>>>, anyhow::Error>>,
    ),
    anyhow::Error,
> {
    // Create a broadcast channel instead of a standard channel
    let (tx, _) = broadcast::channel::<usize>(100);

    let tx_clone = tx.clone();

    let mut worlds = vec![];

    for (i, builder) in builders.into_iter().enumerate() {
        tracing::info!("Spawning world: {}", i);
        let world = builder.build_arc().await?;
        worlds.push(world);
    }

    let worlds_clone = worlds.clone();

    tracing::info!("Worlds spawned, running tasks in separate thread.");
    let slice = std::thread::spawn(move || {
        let rt = Builder::new_multi_thread().build()?;
        let semaphore = Arc::new(Semaphore::new(worlds_clone.len()));
        let errors = Arc::new(tokio::sync::Mutex::new(vec![] as Vec<anyhow::Error>));

        let res = rt.block_on(spawn_tasks(worlds_clone, tx, semaphore, errors));
        res
    });

    Ok((tx_clone, worlds, slice))
}
