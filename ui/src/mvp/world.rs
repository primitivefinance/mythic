//! Abstraction layer for managing and communicating with the simulation environment.

use arbiter_core::environment::{builder::EnvironmentBuilder, Environment};
use simulation::{
    agents::{Agent, Agents},
    settings::{parameters::Single, SimulationConfig},
};
use tokio::{
    runtime::Builder,
    sync::{mpsc, Mutex, Semaphore},
    time::Instant,
};

use super::*;

#[derive(PartialEq)]
pub enum Status {
    Running,
    Paused,
    Stopped,
}

/// State of the world, past, present, and future.
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

pub struct World {
    // The simulation instance.
    pub arbiter: Environment,
    // The agents in the simulation.
    pub agents: Agents,
    // The state of the simulation.
    pub state: State,
    // Global simulation settings
    pub config: SimulationConfig<Single>,
}

impl World {
    pub fn new(arbiter: Environment, agents: Agents) -> Self {
        Self {
            arbiter,
            agents,
            state: State::new(),
            config: SimulationConfig::default(),
        }
    }

    /// Cycles the core simulation loop.
    /// Exits early if not running.
    pub async fn update(&mut self) -> anyhow::Result<(), anyhow::Error> {
        tracing::trace!("Updating simulation.");

        // Call the step function.
        self.step().await?;

        tracing::warn!(
            "Simulation step complete. Step: {}",
            self.state.current_step
        );

        Ok(())
    }

    /// Handles running the simulation.
    pub async fn run(&mut self) -> anyhow::Result<(), anyhow::Error> {
        // Exit if simulation is already running.
        if self.state.status == Status::Running {
            return Ok(());
        }

        tracing::trace!("Running simulation.");

        self.state.status = Status::Running;

        Ok(())
    }

    /// Handles pausing the simulation.
    pub async fn pause(&mut self) -> anyhow::Result<(), anyhow::Error> {
        // Exit if simulation is already paused.
        if self.state.status == Status::Paused {
            return Ok(());
        }

        tracing::trace!("Pausing simulation.");

        self.state.status = Status::Paused;

        Ok(())
    }

    /// Handles stopping the simulation.
    pub async fn stop(&mut self) -> anyhow::Result<(), anyhow::Error> {
        // Exit if simulation is already stopped.
        if self.state.status == Status::Stopped {
            return Ok(());
        }

        tracing::trace!("Stopping simulation.");

        self.state.status = Status::Stopped;

        Ok(())
    }

    /// Handles agent startup functions.
    /// Startup should be called before the simulation is started.
    pub async fn startup(&mut self) -> anyhow::Result<(), anyhow::Error> {
        // Exit if simulation is already running.
        if self.state.status == Status::Running {
            return Ok(());
        }

        tracing::trace!("Starting agents.");

        for agent in self.agents.iter_mut() {
            agent.startup().await?;
        }

        Ok(())
    }

    /// Moves the simulation forward one step.
    /// Handles the execution of agent steps.
    pub async fn step(&mut self) -> anyhow::Result<(), anyhow::Error> {
        // Exit if the simulation is not running.
        if self.state.status != Status::Running {
            return Ok(());
        }

        tracing::trace!("Stepping simulation.");

        self.state.current_step += 1;

        for agent in self.agents.iter_mut() {
            agent.priority_step().await?;
        }

        for agent in self.agents.iter_mut() {
            agent.step().await?;
        }

        Ok(())
    }
}

fn example_world() -> anyhow::Result<(mpsc::Sender<usize>, Arc<Mutex<World>>)> {
    let arbiter = EnvironmentBuilder::new().build();
    let agents = Agents::new();
    let mut world = Arc::new(Mutex::new(World::new(arbiter, agents)));

    // Create a channel for sending update signals
    let (tx, mut rx) = mpsc::channel::<usize>(100);

    let tx_clone = tx.clone();

    let world_thread = world.clone();

    std::thread::spawn(move || {
        let start = Instant::now();

        // Create a multi-threaded runtime
        let rt = Builder::new_multi_thread().build()?;

        // Create a semaphore with a given number of permits
        let semaphore = Arc::new(Semaphore::new(1));

        let rx = Arc::new(Mutex::new(rx));

        let res: anyhow::Result<()> = rt.block_on(async {
            let mut handles = vec![];
            let errors = Arc::new(tokio::sync::Mutex::new(vec![]));

            for i in 0..1 {
                let errors_clone = errors.clone();
                let semaphore_clone = semaphore.clone();
                let rx_clone = rx.clone();
                let world_clone = world_thread.clone();
                handles.push(tokio::spawn(async move {
                    // Acquire a world lock.
                    let mut world = world_clone.lock().await;

                    tracing::warn!("Running environment; Full config: {:#?}", world.config);
                    world.run().await.unwrap(); // Running simulation.

                    while let Some(_) = rx_clone.lock().await.recv().await {
                        tracing::warn!("Received message");
                        // Acquire a permit outside the spawned task
                        let permit = semaphore_clone.acquire().await.unwrap();
                        let result: anyhow::Result<(), anyhow::Error> = world.update().await;
                        match result {
                            Err(e) => {
                                tracing::error!("Got step error: {:?}", e);
                                let mut errors_clone_lock = errors_clone.lock().await;
                                errors_clone_lock.push(e);
                                // Drop the permit when the simulation is done.
                                drop(permit);
                            }
                            Ok(_) => {
                                tracing::warn!("Got step result.");
                                drop(permit);
                                return;
                            }
                        }
                    }
                }));
            }

            for handle in handles {
                handle.await?;
                tracing::warn!("Simulation complete");
            }

            Ok(())
        });

        let duration = start.elapsed();
        tracing::info!("Total duration of simulations: {:?}", duration);

        res
    });

    Ok((tx_clone, world.clone()))
}

pub enum Error {
    Empty,
}

#[cfg(test)]
mod tests {
    use tracing::Level;

    use super::*;
    use tracing_subscriber::prelude::*;

    #[tokio::test]
    async fn test_example_world() {
        // start tracer
        let subscriber = tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .finish();
        tracing::subscriber::set_global_default(subscriber).expect("Failed to set global default");

        let (tx, world) = example_world().unwrap();

        // Send a message
        tx.send(1).await.unwrap();

        // Add a delay here so it has time to process.
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

        // Lock world and check its state
        let world_lock = world.lock().await;
        assert_eq!(world_lock.state.current_step, 1);
    }
}
