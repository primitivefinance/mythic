//! Abstraction layer for managing and communicating with the simulation
//! environment.

use std::{path::Path, sync::Arc};

use arbiter_core::{
    data_collection::EventLogger,
    environment::{builder::EnvironmentBuilder, Environment},
};
use ethers::prelude::rand::{self, Rng};
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
        strategy_monitor::StrategyMonitorAgent,
        token_admin::TokenAdmin,
        Agents,
    },
    settings::{
        parameters::{Multiple, Single},
        SimulationConfig,
    },
    simulations::errors::SimulationError,
    strategy::g3m::G3mStrategy,
};
use tokio::{
    runtime::Builder,
    sync::{broadcast, Mutex, Semaphore},
    task::JoinHandle,
};
use tracing_futures::Instrument;

#[derive(Debug, PartialEq)]
pub enum Status {
    Running,
    Paused,
    Stopped,
}

/// State of the world, past, present, and future.
#[derive(Debug)]
pub struct State {
    pub current_step: usize,

    /// Simulations are a runtime that can be started, paused, and stopped.
    pub status: Status,
}

impl State {
    pub fn new() -> Self {
        Self {
            current_step: 0,
            status: Status::Stopped,
        }
    }
}

#[derive(Debug)]
pub struct World {
    // The simulation instance.
    pub arbiter: Environment,
    // The agents in the simulation.
    pub agents: Arc<Mutex<Agents>>,
    // The state of the simulation.
    pub state: State,
    // Global simulation settings
    pub config: SimulationConfig<Multiple>,
    // Rough rng for world.
    pub seed: u64,
}

const WORLD_TRACE_IDENTIFIER: &str = "world";

impl World {
    pub fn new(arbiter: Environment, agents: Arc<Mutex<Agents>>, seed: u64) -> Self {
        Self {
            arbiter,
            agents,
            state: State::new(),
            config: SimulationConfig::default(),
            seed,
        }
    }

    /// Cycles the core simulation loop.
    /// Exits early if not running.
    #[tracing::instrument(skip(self), fields(layer = %"world", id = self.seed, action = %"update"))]
    pub async fn update(&mut self) -> anyhow::Result<(), anyhow::Error> {
        tracing::trace!("{}.{}.: Updating.", WORLD_TRACE_IDENTIFIER, self.seed);

        // Call the step function.
        self.step().await?;

        tracing::debug!(
            "{}.{}.: Simulation step complete. Step: {}",
            WORLD_TRACE_IDENTIFIER,
            self.seed,
            self.state.current_step
        );

        Ok(())
    }

    /// Handles running the simulation.
    #[tracing::instrument(skip(self), fields(layer = %"world", id = self.seed, action = %"run"))]
    pub async fn run(&mut self) -> anyhow::Result<(), anyhow::Error> {
        // Exit if simulation is already running.
        if self.state.status == Status::Running {
            return Ok(());
        }

        tracing::trace!(
            "{}.{}.: Running simulation.",
            WORLD_TRACE_IDENTIFIER,
            self.seed
        );

        self.state.status = Status::Running;

        Ok(())
    }

    /// Handles pausing the simulation.
    /// todo: does not need to be async?
    #[tracing::instrument(skip(self), fields(layer = %"world", id = self.seed, action = %"pause"))]
    pub async fn pause(&mut self) -> anyhow::Result<(), anyhow::Error> {
        // Exit if simulation is already paused.
        if self.state.status == Status::Paused {
            return Ok(());
        }

        tracing::trace!(
            "{}.{}.: Pausing simulation.",
            WORLD_TRACE_IDENTIFIER,
            self.seed
        );

        self.state.status = Status::Paused;

        Ok(())
    }

    /// Handles stopping the simulation.
    #[tracing::instrument(skip(self), fields(layer = %"world", id = self.seed, action = %"stop"))]
    pub fn stop(&mut self) -> anyhow::Result<(), anyhow::Error> {
        // Exit if simulation is already stopped.
        if self.state.status == Status::Stopped {
            return Ok(());
        }

        tracing::trace!(
            "{}.{}.: Stopping simulation.",
            WORLD_TRACE_IDENTIFIER,
            self.seed
        );

        self.state.status = Status::Stopped;

        Ok(())
    }

    /// Handles agent startup functions.
    /// Startup should be called before the simulation is started.
    #[tracing::instrument(skip(self), fields(layer = %"world", id = self.seed, action = %"startup"))]
    pub async fn startup(&mut self) -> anyhow::Result<(), anyhow::Error> {
        // Exit if simulation is already running.
        // if self.state.status == Status::Running {
        // return Ok(());
        // }

        tracing::trace!(
            "{}.{}.: Starting up agents.",
            WORLD_TRACE_IDENTIFIER,
            self.seed
        );

        for agent in self.agents.lock().await.iter_mut() {
            let layer = "agent";
            let id = agent.0;
            let action = "startup";
            let agent_span =
                tracing::debug_span!("agent", layer = %layer, id = %id, action = %action);

            agent.1.startup().instrument(agent_span).await?;
        }

        Ok(())
    }

    /// Moves the simulation forward one step.
    /// Handles the execution of agent steps.
    #[tracing::instrument(skip(self), fields(layer = %"world", id = self.seed, action = %"step"))]
    pub async fn step(&mut self) -> anyhow::Result<(), anyhow::Error> {
        // Exit if the simulation is not running.
        if self.state.status != Status::Running {
            return Ok(());
        }

        tracing::trace!(
            "{}.{}.: Stepping agents.",
            WORLD_TRACE_IDENTIFIER,
            self.seed
        );

        self.state.current_step += 1;

        for agent in self.agents.lock().await.iter_mut() {
            agent.1.priority_step().await?;
        }

        for agent in self.agents.lock().await.iter_mut() {
            let layer = "agent";
            let id = agent.0;
            let action = "step";
            let agent_span =
                tracing::debug_span!("agent", layer = %layer, id = %id, action = %action);
            agent.1.step().instrument(agent_span).await?;
        }

        Ok(())
    }
}

pub struct WorldBuilder {
    arbiter: Option<Environment>,
    agents: Option<Agents>,
    config: Option<SimulationConfig<Multiple>>,
    seed: u64, // Add a field for the seed
}

impl WorldBuilder {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let seed = rng.gen::<u64>(); // Generate a random seed

        Self {
            arbiter: None,
            agents: None,
            config: None,
            seed, // Store the seed
        }
    }

    pub fn arbiter(mut self, arbiter: Environment) -> Self {
        self.arbiter = Some(arbiter);
        self
    }

    pub fn agents(mut self, agents: Agents) -> Self {
        self.agents = Some(agents);
        self
    }

    pub fn config(mut self, config: SimulationConfig<Multiple>) -> Self {
        self.config = Some(config);
        self
    }

    // AGENT-SETUP
    pub async fn setup(
        environment: &Environment,
        config: SimulationConfig<Single>,
    ) -> anyhow::Result<Agents, anyhow::Error> {
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

        tracing::info!("Generic agents created.");

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

        tracing::info!("Portfolio manager created.");

        let lp = LiquidityProvider::<G3mStrategy>::new(
            &environment,
            &config,
            "lp",
            &token_admin,
            strategy_address,
        )
        .await?;
        agents.add(lp);

        tracing::info!("Liquidity provider created.");

        let arbitrageur = Arbitrageur::<G3mStrategy>::new(
            &environment,
            &token_admin,
            price_changer.liquid_exchange.address(),
            strategy_address,
        )
        .await?;
        agents.add(arbitrageur);

        tracing::info!("Arbitrageur created.");

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

        let strategy_monitor = StrategyMonitorAgent::<G3mStrategy>::new(
            &environment,
            &config,
            "strategy_monitor",
            strategy_address,
            &token_admin,
        )
        .await?;
        agents.add(strategy_monitor);

        event_logger
            .metadata(config)
            .map_err(|e| SimulationError::GenericError(e.to_string()))?
            .run()
            .map_err(|e| SimulationError::GenericError(e.to_string()))?;

        tracing::info!("Agents setup.");
        Ok(agents)
    }

    #[tracing::instrument(skip(self))]
    pub async fn build(self) -> anyhow::Result<World> {
        let arbiter = self
            .arbiter
            .ok_or_else(|| anyhow::anyhow!("Arbiter not set"))?;
        let mut agents = self
            .agents
            .ok_or_else(|| anyhow::anyhow!("Agents not set"))?;

        // todo: lots to fix here
        let config = self.config.unwrap();
        let seed = self.seed;

        let direct_configs: Vec<SimulationConfig<Single>> = config.clone().into();

        let mut count = 0;
        for config in direct_configs {
            if count > 0 {
                break;
            }
            tracing::info!("Setting up agents for config");
            agents = Self::setup(&arbiter, config).await?;
            count += 1;
        }

        tracing::info!("World built.");

        Ok(World {
            arbiter,
            agents: Arc::new(Mutex::new(agents)),
            state: State::new(),
            config,
            seed,
        })
    }

    /// Wrap it with arc and mutex
    #[tracing::instrument(skip(self))]
    pub async fn build_arc(self) -> anyhow::Result<Arc<Mutex<World>>> {
        let world = self.build().await?;
        Ok(Arc::new(Mutex::new(world)))
    }
}

impl Default for WorldBuilder {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        let seed = rng.gen::<u64>(); // Generate a random seed

        // todo: this breaks tests because the test calls this from within ui/ crate...
        let config_path = Path::new("configs").join("dca").join("static.toml");

        let config = simulation::simulations::import(&config_path.to_str().unwrap()).unwrap();
        Self {
            arbiter: Some(EnvironmentBuilder::new().build()),
            agents: Some(Agents::new()),
            config: Some(config),
            seed,
        }
    }
}

#[tracing::instrument(skip(worlds, tx, semaphore, errors), fields(worlds = worlds.len()))]
async fn spawn_tasks(
    worlds: Vec<Arc<Mutex<World>>>,
    tx: broadcast::Sender<usize>,
    semaphore: Arc<Semaphore>,
    errors: Arc<tokio::sync::Mutex<Vec<anyhow::Error>>>,
) -> anyhow::Result<Vec<Arc<Mutex<World>>>, anyhow::Error> {
    let mut handles = vec![];

    for world in &worlds {
        let world_clone = world.clone();
        let rx = tx.subscribe();
        let rx = Arc::new(Mutex::new(rx));
        let current_span = tracing::Span::current();
        let new_span = current_span.clone();
        let handle =
            create_task(world_clone, rx, semaphore.clone(), errors.clone()).instrument(new_span);
        handles.push(handle);
    }

    for handle in handles {
        handle.await?;
    }

    Ok(worlds)
}

#[tracing::instrument(skip(world, rx, semaphore, errors))]
fn create_task(
    world: Arc<Mutex<World>>,
    rx: Arc<Mutex<broadcast::Receiver<usize>>>,
    semaphore: Arc<Semaphore>,
    errors: Arc<tokio::sync::Mutex<Vec<anyhow::Error>>>,
) -> JoinHandle<()> {
    let errors_clone = errors.clone();
    let semaphore_clone = semaphore.clone();
    let rx_clone = rx.clone();
    let world_clone = world.clone();
    tokio::spawn(async move {
        {
            // Drops the lock when done calling run().
            let mut world = world_clone.lock().await;
            tracing::debug!("Running environment; Full config: {:#?}", world.config);

            // Startup agents.
            // todo: figure out why this reverts, probably something with agents?
            // world.startup().await.unwrap();

            // Running simulation.
            world.run().await.unwrap();
        }

        while let Ok(_) = rx_clone.lock().await.recv().await {
            let mut world = world_clone.lock().await;
            tracing::trace!("Received message");
            let permit = semaphore_clone.acquire().await.unwrap();
            let result: anyhow::Result<(), anyhow::Error> = world.update().await;
            match result {
                Err(e) => {
                    tracing::error!("Got step error: {:?}", e);
                    let mut errors_clone_lock = errors_clone.lock().await;
                    errors_clone_lock.push(e);
                    // Drop the world when done.
                    std::mem::drop(world);
                    // Drop the permit when the simulation is done.
                    drop(permit);
                }
                Ok(_) => {
                    // Drop the world when done.
                    tracing::trace!("Got step result.");
                    std::mem::drop(world);
                    drop(permit);
                }
            }
        }
    })
}

#[tracing::instrument()]
pub async fn spawn_worlds(
    num_worlds: usize,
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
    for i in 0..num_worlds {
        tracing::info!("Spawning world: {}", i);
        let world = WorldBuilder::default().build_arc().await?;
        worlds.push(world);
    }

    let worlds_clone = worlds.clone();

    tracing::info!("Worlds spawned, running tasks in separate thread.");
    let slice = std::thread::spawn(move || {
        let rt = Builder::new_multi_thread().build()?;
        let semaphore = Arc::new(Semaphore::new(num_worlds));
        let errors = Arc::new(tokio::sync::Mutex::new(vec![] as Vec<anyhow::Error>));

        let res = rt.block_on(spawn_tasks(worlds_clone, tx, semaphore, errors));
        res
    });

    Ok((tx_clone, worlds, slice))
}

/// Manages the worlds and the thread that runs them.
#[derive(Debug)]
pub struct WorldManager {
    pub worlds: Vec<Arc<Mutex<World>>>,
    pub tx: Option<Arc<Mutex<broadcast::Sender<usize>>>>,
    pub slice:
        Option<Arc<Mutex<std::thread::JoinHandle<Result<Vec<Arc<Mutex<World>>>, anyhow::Error>>>>>,
}

#[derive(Debug, PartialEq)]
pub enum WorldManagerState {
    Running,
    Paused,
    Stopped,
    Completed,
}

impl WorldManager {
    pub fn status(&self) -> WorldManagerState {
        match self.tx {
            Some(_) => WorldManagerState::Running,
            None => WorldManagerState::Stopped,
        }
    }

    /// Consumes the world manager and spawns the worlds.
    #[tracing::instrument(skip(self))]
    pub async fn spawn(mut self, num_worlds: usize) -> anyhow::Result<Self, anyhow::Error> {
        let (tx, worlds, slice) = spawn_worlds(num_worlds).await?;
        self.tx = Some(Arc::new(Mutex::new(tx)));
        self.worlds = worlds;
        self.slice = Some(Arc::new(Mutex::new(slice)));

        tracing::info!("Worlds spawned, exiting spawn fn.");
        Ok(self)
    }

    pub fn new(
        worlds: Vec<Arc<Mutex<World>>>,
        tx: Option<Arc<Mutex<broadcast::Sender<usize>>>>,
        slice: Option<
            Arc<Mutex<std::thread::JoinHandle<Result<Vec<Arc<Mutex<World>>>, anyhow::Error>>>>,
        >,
    ) -> Self {
        Self { worlds, tx, slice }
    }

    pub fn add_world(&mut self, world: Arc<Mutex<World>>) {
        self.worlds.push(world);
    }

    #[tracing::instrument(skip(self))]
    pub async fn run(&self) -> anyhow::Result<(), anyhow::Error> {
        // for each world, call run.
        for world in &self.worlds {
            {
                let mut world_lock = world.lock().await;
                world_lock.run().await?;
            }
        }

        Ok(())
    }

    #[tracing::instrument(skip(self))]
    pub async fn stop(&self) -> anyhow::Result<(), anyhow::Error> {
        // for each world, call stop.
        for world in &self.worlds {
            {
                let mut world_lock = world.lock().await;
                world_lock.stop()?;
            }
        }

        Ok(())
    }

    #[tracing::instrument(skip(self))]
    pub async fn pause(&self) -> anyhow::Result<(), anyhow::Error> {
        // for each world, call pause.
        for world in &self.worlds {
            {
                let mut world_lock = world.lock().await;
                world_lock.pause().await?;
            }
        }

        Ok(())
    }
}

impl Default for WorldManager {
    fn default() -> Self {
        Self {
            worlds: vec![],
            tx: None,
            slice: None,
        }
    }
}

pub fn world_span(world: &World) -> tracing::Span {
    tracing::span!(
        tracing::Level::TRACE,
        "world",
        seed = world.seed,
        step = world.state.current_step,
        status = ?world.state.status
    )
}

pub fn world_manager_span(world_manager: &WorldManager) -> tracing::Span {
    tracing::span!(
        tracing::Level::TRACE,
        "world_manager",
        worlds = world_manager.worlds.len(),
        status = ?world_manager.status()
    )
}

#[cfg(test)]
mod tests {
    use ethers::types::{Address, U256};
    use simulation::agents::counter::CounterAgent;

    use super::*;
    use crate::mvp::api::tests::TEST_SUBSCRIBER;

    #[tokio::test]
    async fn test_world_manager() {
        let _ = *TEST_SUBSCRIBER;
        let world_manager = WorldManager::default().spawn(5).await.unwrap();
        assert_eq!(world_manager.worlds.len(), 5);
    }

    #[tokio::test]
    async fn test_world_builder() {
        let mut world = WorldBuilder::default().build().await.unwrap();
        assert_eq!(world.state.current_step, 0);

        // try running the simulation
        world.run().await.unwrap();

        // try updating
        world.update().await.unwrap();

        // assert step changed
        assert_eq!(world.state.current_step, 1);
    }

    #[tokio::test]
    async fn test_concurrent_worlds() {
        let _ = *TEST_SUBSCRIBER;

        let (tx, worlds, _slice) = spawn_worlds(5).await.unwrap();

        // Add a delay here so it has time to process.
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

        // Send a message
        let msg_result = tx.send(1);
        match msg_result {
            Ok(_) => tracing::warn!("Message sent"),
            Err(e) => tracing::error!("Failed to send message: {:?}", e),
        }

        // Add a delay here so it has time to process.
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

        // for each world, lock it, and check its state step changed.
        for world in worlds {
            let world_lock = world.lock().await;
            assert_eq!(world_lock.state.current_step, 1);
        }
    }

    #[tokio::test]
    async fn test_single_world() {
        let _ = *TEST_SUBSCRIBER;

        let (tx, worlds, slice) = spawn_worlds(1).await.unwrap();

        // Add a delay here so it has time to process.
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

        // Send a message
        tx.send(1).unwrap();

        // Add a delay here so it has time to process.
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

        // Lock world and check its state
        let world_lock = worlds[0].lock().await;
        assert_eq!(world_lock.state.current_step, 1);

        drop(slice);
    }

    #[tokio::test]
    async fn test_changing_agents_in_worlds() {
        let _ = *TEST_SUBSCRIBER;

        let (tx, worlds, slice) = spawn_worlds(1).await.unwrap();

        // Add a delay here so it has time to process.
        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

        // Send a message
        let mut msg = tx.send(1);
        match msg {
            Ok(_) => tracing::warn!("Message sent"),
            Err(e) => tracing::error!("Failed to send message: {:?}", e),
        }

        // Add a delay here so it has time to process.
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

        tracing::info!("Getting world lock");

        // Lock world and check its state, then add an agent, then drop the lock.
        {
            let world_lock = worlds[0].lock().await;
            assert_eq!(world_lock.state.current_step, 1);

            tracing::info!("Adding a counter agent");

            let direct_configs: Vec<SimulationConfig<Single>> = world_lock.config.clone().into();

            // Create a new agent
            // todo: get lp address
            let counter_agent = CounterAgent::new(
                &world_lock.arbiter,
                &direct_configs[0],
                "counter".to_string(),
                Address::zero(),
            )
            .await
            .unwrap();

            {
                // Change the agents in the world then drops the lock.
                let mut agents = world_lock.agents.lock().await;

                tracing::info!("Current agents: {}", agents.0.len());
                agents.add(counter_agent);
                tracing::info!("New agents: {}", agents.0.len());
            }
        }
        // Add a delay here so it has time to process.
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

        tracing::info!("Sending another message after adding agent to agents.");

        // Send a message
        let msg = tx.send(1);
        match msg {
            Ok(_) => tracing::warn!("Message sent"),
            Err(e) => tracing::error!("Failed to send message: {:?}", e),
        }

        // Add a delay here so it has time to process.
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

        // Lock world and check its state then drop the lock.
        {
            let world_lock = worlds[0].lock().await;
            assert_eq!(world_lock.state.current_step, 2);
        }

        {
            let world_lock = worlds[0].lock().await;
            let agents_lock = world_lock.agents.lock().await;
            // Check the counter is the current step - 1, because we added the agent in the
            // middle.
            let counter_agent = agents_lock
                .0
                .get("counter")
                .unwrap()
                .as_any()
                .downcast_ref::<CounterAgent>()
                .unwrap();
            assert_eq!(counter_agent.get().await.unwrap(), U256::from(1));
        }

        // stop the sim by dropping the tx channel
        drop(tx);
    }
}
