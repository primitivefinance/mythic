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
        AgentParameters, Agents,
    },
    settings::{parameters::Single, SimulationConfig},
    simulations::errors::SimulationError,
    strategy::g3m::G3mStrategy,
};
use tokio::{
    runtime::Builder,
    sync::{broadcast, Mutex, Semaphore},
};
use tracing_futures::Instrument;

use self::config::ConfigBuilder;
use super::world::{self, World, WorldManager};

pub type SimulatedWorld = anyhow::Result<Arc<Mutex<World>>, Arc<anyhow::Error>>;

pub type InstanceManager = Arc<Mutex<WorldManager>>;

#[tracing::instrument]
pub async fn spawn(builders: Vec<MiniWorldBuilder>) -> SimulatedWorld {
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
pub async fn manager(builders: Vec<MiniWorldBuilder>) -> anyhow::Result<World, anyhow::Error> {
    // just get the first builder
    let builder = builders[0].clone();
    let slice = spawn_world(builder).await?;

    tracing::info!("Worlds spawned, exiting spawn fn.");
    let world = slice.join().unwrap().unwrap();
    Ok(world)
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
            current_step: 0,
        })
    }

    async fn build_arc(self) -> anyhow::Result<Arc<Mutex<World>>> {
        let world = self.build().await?;
        Ok(Arc::new(Mutex::new(world)))
    }
}

#[tracing::instrument(skip(builder))]
pub async fn spawn_world(
    builder: MiniWorldBuilder,
) -> anyhow::Result<std::thread::JoinHandle<Result<World, anyhow::Error>>, anyhow::Error> {
    let world = builder.build().await.unwrap();

    tracing::info!("Worlds spawned, running tasks in separate thread.");
    let slice = std::thread::spawn(move || {
        let rt = Builder::new_multi_thread().build()?;
        let semaphore = Arc::new(Semaphore::new(1));
        let errors = Arc::new(tokio::sync::Mutex::new(vec![] as Vec<anyhow::Error>));

        let res = rt.block_on(spawn_tasks(world, semaphore, errors));
        res
    });

    Ok(slice)
}

#[tracing::instrument(skip(world, semaphore, errors))]
pub async fn spawn_tasks(
    world: World,
    semaphore: Arc<Semaphore>,
    errors: Arc<tokio::sync::Mutex<Vec<anyhow::Error>>>,
) -> anyhow::Result<World, anyhow::Error> {
    let current_span = tracing::Span::current();
    let new_span = current_span.clone();
    let handle = create_task(world, semaphore.clone(), errors.clone()).instrument(new_span);
    let result = handle.await.unwrap();
    result
}

#[tracing::instrument(skip(world, semaphore, errors))]
fn create_task(
    world: World,
    semaphore: Arc<Semaphore>,
    errors: Arc<tokio::sync::Mutex<Vec<anyhow::Error>>>,
) -> tokio::task::JoinHandle<anyhow::Result<World, anyhow::Error>> {
    let errors_clone = errors.clone();
    let semaphore_clone = semaphore.clone();
    let handle = tokio::spawn(async move {
        tracing::debug!("Running environment; Full config: {:#?}", world.config);

        // Should add a check that the agents have already been started up.
        // Running simulation.
        let mut world = world;
        world.run().await.unwrap();
        world.startup().await.unwrap();

        let permit = semaphore_clone.acquire().await.unwrap();

        let price_changer = world
            .config
            .agent_parameters
            .iter()
            .find(|p| matches!(p.1, AgentParameters::PriceChanger(_)))
            .unwrap();

        let price_changer = match price_changer {
            (_, AgentParameters::PriceChanger(p)) => p,
            _ => unreachable!(),
        };

        let num_steps = price_changer.num_steps;

        for _ in 0..num_steps {
            let result: anyhow::Result<(), anyhow::Error> = world.update().await;
            match result {
                Err(e) => {
                    tracing::error!("Got step error: {:?}", e);
                    let mut errors_clone_lock = errors_clone.lock().await;
                    errors_clone_lock.push(e);
                    // Drop the permit when the simulation is done.
                    drop(permit);
                    break;
                }
                Ok(_) => {
                    // Continue running the simulation.
                }
            }
        }

        Ok(world)
    });

    handle
}
