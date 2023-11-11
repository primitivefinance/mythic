use std::{
    collections::{HashMap, VecDeque},
    sync::mpsc::Receiver,
    time::Duration,
};

use anyhow::anyhow;
use iced::{futures::io::Empty, time};
use simulation::{
    agents::{
        counter::CounterAgent,
        liquidity_provider::{LiquidityProvider, LiquidityProviderWrapper},
        price_changer::PriceChanger,
    },
    settings::{parameters::Single, SimulationConfig},
    strategy::LiquidityStrategy,
};
use tracing::{Instrument, Span};

use super::{app::Message, view::terminal_view, *};

/// Implement this trait to make a new screen for the app.
pub trait State
where
    Self: Sync + Send,
{
    fn view<'a>(&'a self) -> Element<'a, view::Message>;
    fn update(&mut self, _message: Message) -> Command<Message> {
        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::none()
    }
}

/// Wraps anything that implements the State trait into an easier to use struct.
pub struct Screen(pub Box<dyn State>);

impl Screen {
    pub fn new(state: Box<dyn State>) -> Self {
        Self(state)
    }

    pub fn view<'a>(&'a self) -> Element<'a, view::Message> {
        self.0.view()
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        self.0.update(message)
    }

    pub fn subscription(&self) -> Subscription<Message> {
        self.0.subscription()
    }
}

const USER_ACTION_FILTER: &str = "user.";

pub struct Terminal {
    logs: VecDeque<String>,
    firehoses: Vec<VecDeque<String>>,
    receiver: Arc<Mutex<Receiver<String>>>,
    world_manager: Arc<tokio::sync::Mutex<WorldManager>>,
    status: WorldManagerState,
    realtime: bool,
    // storage slots to keep track of for giving feedback to user.
    watching_state: HashMap<String, String>,
    // render the firehose logs or not
    hide_logs: bool,
}

pub async fn spawn() -> anyhow::Result<Arc<tokio::sync::Mutex<WorldManager>>, anyhow::Error> {
    // Override the world manager with a new one that has spawned worlds.
    Ok(Arc::new(tokio::sync::Mutex::new(
        WorldManager::default().spawn(1).await?,
    )))
}

impl Terminal {
    pub fn new(receiver: Arc<Mutex<Receiver<String>>>) -> Self {
        // Need to fill this log to render the main terminal!
        let mut welcome = VecDeque::new();
        welcome.push_back(format!("Welcome to Excalibur"));

        let mut firehoses = vec![welcome.clone()];
        // Add a firehose for the user
        firehoses.push(VecDeque::new());

        Self {
            logs: welcome.clone(),
            firehoses,
            receiver,
            world_manager: Arc::new(tokio::sync::Mutex::new(WorldManager::default())),
            status: WorldManagerState::Stopped,
            realtime: true,
            watching_state: HashMap::new(),
            hide_logs: false,
        }
    }

    // todo: this iterates over the flattened logs storage, it should only effect
    // storage of each existing firehose storage...
    // this way we can keep logs for a firehose even if we aren't storing them in
    // the main log storage.
    pub fn filter_logs_into_firehoses(&self) -> Vec<VecDeque<String>> {
        // Need to fill this log to render the main terminal!
        let mut welcome = VecDeque::new();
        welcome.push_back(format!("Welcome to Excalibur"));

        let mut firehoses = vec![welcome];

        const MAX_LOGS: usize = 100;

        let mut world_to_firehose_index: HashMap<usize, usize> = HashMap::new();
        world_to_firehose_index.insert(0, 0); // Main thread

        // Add a firehose for the user
        firehoses.push(VecDeque::new());
        world_to_firehose_index.insert(1, 1); // User

        for log in &self.logs {
            let parts: Vec<&str> = log.split('.').collect();
            if parts[0] == "world" {
                let world_id: usize = parts[1]
                    .split('.')
                    .next()
                    .unwrap()
                    .parse()
                    .unwrap_or_else(|_| panic!("Failed to parse world_id"));
                if !world_to_firehose_index.contains_key(&world_id) {
                    firehoses.push(VecDeque::new());
                    world_to_firehose_index.insert(world_id, firehoses.len() - 1);
                }
                let firehose_index = *world_to_firehose_index.get(&world_id).unwrap();
                firehoses[firehose_index].push_back(log.clone());
                if firehoses[firehose_index].len() > MAX_LOGS {
                    firehoses[firehose_index].pop_front();
                }
            } else if parts[0] == "user" {
                firehoses[1].push_back(log.clone());
                if firehoses[1].len() > MAX_LOGS {
                    firehoses[1].pop_front();
                }
            } else {
                firehoses[0].push_back(log.clone());
                if firehoses[0].len() > MAX_LOGS {
                    firehoses[0].pop_front();
                }
            }
        }

        firehoses
    }

    pub fn purge_non_main_logs(&mut self) {
        let mut new_logs = VecDeque::new();
        for log in &self.logs {
            let parts: Vec<&str> = log.split('.').collect();
            if parts[0] == "world" {
                continue;
            }
            new_logs.push_back(log.clone());
        }
        self.logs = new_logs;
    }

    /// Executes the logic when a spawned message is received.
    #[tracing::instrument(level = "trace", skip(self, world_manager))]
    pub fn handle_startup(
        &mut self,
        world_manager: Arc<tokio::sync::Mutex<WorldManager>>,
    ) -> Command<Message> {
        tracing::trace!("Setting world manager.");
        self.world_manager = world_manager.clone();

        let m = world_manager.clone();
        let f = sim_startup(m);
        Command::perform(f.instrument(terminal_span()), |_| {
            Message::View(view::Message::Simulation(
                view::control::Operation::Continue,
            ))
        })
    }

    /// Executes the logic when a continue message is received.
    #[tracing::instrument(level = "trace", skip(self), fields(user_action = "continue"))]
    pub fn handle_continue(&mut self) -> Command<Message> {
        if self.status == WorldManagerState::Completed {
            tracing::warn!("Simulation world already completed!");
            return Command::none();
        }

        if self.status == WorldManagerState::Running {
            tracing::warn!("Simulation world already running!");
            return Command::none();
        }

        self.status = WorldManagerState::Running;

        // Triggers a step, which will start the run loop.
        let m = self.world_manager.clone();
        Command::perform(
            async move {
                let locked = m.lock().await;

                if locked.status() == WorldManagerState::Running {
                    return locked.run().await;
                }

                Ok(())
            }
            .instrument(terminal_span()),
            |_| Message::View(view::Message::Simulation(view::control::Operation::Step)),
        )
    }

    #[tracing::instrument(level = "trace", skip(self), fields(user_action = "spawn"))]
    pub fn handle_spawn(&mut self) -> Command<Message> {
        if self.status != WorldManagerState::Stopped {
            tracing::warn!("Simulation already spawned! Stop before spawning a new one.");
            return Command::none();
        }

        tracing::trace!("Spawn simulation message received!");
        self.purge_non_main_logs();

        // Triggers a step, which will start the run loop.
        Command::perform(async { spawn().instrument(terminal_span()).await }, |res| {
            Message::Simulation(app::Simulation::Spawned(res))
        })
    }

    // todo: this is triggered on View::Step, but it should be a root level message.
    #[tracing::instrument(level = "trace", skip(self), fields(user_action = "step"))]
    pub fn handle_step(&mut self) -> Command<Message> {
        if self.status == WorldManagerState::Stopped {
            tracing::trace!("Simulation is stopped, cannot step.");
            return Command::none();
        }

        // If paused, we can still step.
        tracing::trace!("Step simulation message received!");

        let m = self.world_manager.clone();
        let trace_id = "process.step";

        // Message emitted after step completes.
        let mut exit_msg = Message::Empty;
        if self.status == WorldManagerState::Running && !self.realtime {
            exit_msg = Message::View(view::Message::Simulation(view::control::Operation::Step));
        }

        return Command::perform(
            async move {
                let mut m_locked = m.lock().await;
                if let Some(ref mut tx) = m_locked.tx {
                    let msg = tx.lock().await.send(1);
                    tracing::trace!("{} Sent message {:?}", trace_id, msg);
                    match msg {
                        Ok(_) => {
                            // tracing::trace!(
                            // "{} Simulation world
                            // stepped!",
                            // trace_id
                            // );
                        }
                        Err(e) => {
                            tracing::error!(
                                "{}  Simulation world failed to step: {:?}",
                                trace_id,
                                e
                            );
                            return Err(anyhow!("{} Simulation world failed to step:", trace_id));
                        }
                    }
                }

                // for each world get the watched vars
                let mut finished_path = false;
                let mut watched_vars: HashMap<String, String> = HashMap::new();
                for world in m_locked.worlds.iter() {
                    let world = world.lock().await;
                    let world_id = world.seed;
                    let mut agents = world.agents.lock().await;

                    // tracing::debug!("agents: {}", agents.0.len());
                    // We also need to get the PriceChanger agent and check if
                    // its reached the end of its price path.
                    // If so, we need to stop the simulation.
                    for agent in agents.0.iter_mut() {
                        let counter_agent = agent.as_any().downcast_ref::<CounterAgent>();

                        match counter_agent {
                            Some(counter_agent) => {
                                let counter = counter_agent.get().await;
                                match counter {
                                    Ok(counter) => {
                                        tracing::debug!(
                                            "world.{}.: Got counter agent state {}",
                                            world_id.clone(),
                                            counter.as_u128()
                                        );
                                        watched_vars.insert(
                                            format!("world.{}.counter", world_id.clone(),),
                                            format!("{}", counter.as_u128()),
                                        );
                                        // watched_vars.
                                        // push(format!(
                                        // "world.{}.{}",
                                        // world_id.clone(),
                                        // counter
                                        // ));
                                    }
                                    Err(e) => {
                                        tracing::error!(
                                            "world.{}.: Failed to get counter agent state: {:?}",
                                            world_id.clone(),
                                            e
                                        );
                                    }
                                }
                            }
                            None => {
                                // else we got another agent...
                                let price_changer = agent.as_any().downcast_ref::<PriceChanger>();

                                match price_changer {
                                    Some(price_changer) => {
                                        // todo: fix this index, its hardcoded
                                        // to just one trajectory for now.
                                        let price_path = price_changer.trajectory.paths[0].clone();

                                        let current_index = price_changer.index;

                                        let last_index = price_path.len();

                                        if current_index == last_index {
                                            tracing::debug!(
                                                "world.{}.: Price path is empty, stopping simulation",
                                                world_id.clone()
                                            );
                                            finished_path = true;
                                        }
                                    }
                                    None => {
                                        // else its another agent...
                                        let temp = agent.get_state().await;

                                        match temp {
                                            Ok(temp) => {
                                                tracing::debug!(
                                                    "world.{}.: Got LP agent state {}",
                                                    world_id.clone(),
                                                    temp,
                                                );

                                                if temp.as_u128() != 0 {
                                                    tracing::debug!(
                                                        "world.{}.: Got LP agent pvf {}",
                                                        world_id.clone(),
                                                        temp
                                                    );
                                                    watched_vars.insert(
                                                        format!("world.{}.pvf", world_id.clone(),),
                                                        format!("{}", temp.as_u128()),
                                                    );
                                                }
                                            }
                                            Err(e) => {
                                                tracing::error!(
                                                    "world.{}.: Failed to get LP agent state: {:?}",
                                                    world_id.clone(),
                                                    e
                                                );
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                Ok((watched_vars, finished_path))
            },
            |result| {
                match result {
                    Ok((watched_vars, finished_path)) => {
                        tracing::trace!("Simulation world stepped!");

                        // todo: this short-circuits the last update watched
                        // value!
                        if finished_path {
                            tracing::trace!("Simulation world finished!");
                            return Message::Simulation(app::Simulation::Completed);
                        }

                        return Message::View(view::Message::Data(view::Data::UpdateWatchedValue(
                            watched_vars,
                        )));
                    }
                    Err(e) => {
                        tracing::error!(
                            "Simulation world failed to step:
     {:?}",
                            e
                        );
                    }
                }

                exit_msg
            },
        );
    }

    #[tracing::instrument(level = "trace", skip(self), fields(user_action = "stop"))]
    pub fn handle_stop(&mut self) -> Command<Message> {
        tracing::trace!("Stop simulation message received!");
        self.status = WorldManagerState::Stopped;

        // If world manager is set, then we need to stop it.
        let m = self.world_manager.clone();
        Command::perform(
            async move {
                let locked = m.lock().await;

                if locked.status() == WorldManagerState::Running {
                    return locked.stop().await;
                }

                Ok(())
            },
            |result| {
                match result {
                    Ok(_) => {
                        tracing::trace!("Simulation world stopped!");
                    }
                    Err(e) => {
                        tracing::error!("Simulation world failed to stop: {:?}", e);
                    }
                }
                Message::Empty
            },
        )
    }

    #[tracing::instrument(level = "trace", skip(self), fields(user_action = "pause"))]
    pub fn handle_pause(&mut self) -> Command<Message> {
        tracing::trace!("Pause simulation message received!");
        self.status = WorldManagerState::Paused;

        let m = self.world_manager.clone();
        Command::perform(
            async move {
                let locked = m.lock().await;

                if locked.status() == WorldManagerState::Running {
                    return locked.pause().await;
                }

                Ok(())
            },
            |_| Message::Empty,
        )
    }

    #[tracing::instrument(level = "trace", skip(self), fields(user_action = "add_agent"))]
    pub fn handle_add_agent(&mut self) -> Command<Message> {
        tracing::warn!(
            "{}.{}: Add agent message received!",
            USER_ACTION_FILTER,
            "add_agent"
        );
        let m = self.world_manager.clone();
        return Command::perform(
            async move {
                let locked = m.lock().await;

                for world in locked.worlds.iter() {
                    let world = world.lock().await;
                    let world_id = world.seed;
                    let mut agents = world.agents.lock().await;

                    tracing::info!("world.{}.: Adding agent", world_id.clone());
                    let direct_configs: Vec<SimulationConfig<Single>> = world.config.clone().into();

                    // loop through the agents, and find the LP agent by checking the
                    // label on the agent's client.
                    let mut lp_address = Address::zero();

                    for agent in agents.0.iter_mut() {
                        let name = agent.get_name();

                        if name.contains("liquidity_provider") {
                            tracing::trace!("world.{}.: Found LP agent", world_id.clone());
                            lp_address = agent.get_client().unwrap().address();
                        }
                    }

                    // Create a new agent
                    let counter_agent = CounterAgent::new(
                        &world.arbiter,
                        &direct_configs[0],
                        "counter".to_string(),
                        lp_address.clone(),
                    )
                    .await
                    .unwrap();

                    tracing::trace!(
                        "world.{}.: Current agents: {}",
                        world_id.clone(),
                        agents.0.len()
                    );
                    agents.add(counter_agent);
                    tracing::trace!(
                        "world.{}.: New agents: {}",
                        world_id.clone(),
                        agents.0.len()
                    );
                }
            },
            |_| Message::View(view::Message::Simulation(view::control::Operation::Step)),
        );
    }
}

#[tracing::instrument(level = "trace", skip(m))]
pub async fn sim_startup(
    m: Arc<tokio::sync::Mutex<WorldManager>>,
) -> anyhow::Result<(), anyhow::Error> {
    let locked = m.lock().await;

    for world in locked.worlds.iter() {
        let mut world = world.lock().await;
        let world_id = world.seed;
        tracing::debug!("world.{}.: Running startup", world_id.clone());
        world.startup().await?;
    }

    Ok(())
}

pub fn terminal_span() -> Span {
    tracing::info_span!("terminal")
}

impl State for Terminal {
    fn view<'a>(&'a self) -> Element<'a, view::Message> {
        let watching_state_vec = self
            .watching_state
            .iter()
            .map(|(k, v)| format!("{}: {}", k, v))
            .collect::<Vec<String>>();
        let mut filtered_logs = self.firehoses.clone();
        if self.hide_logs {
            filtered_logs = vec![VecDeque::new()];
        }

        view::app_layout(view::terminal_view_multiple_firehose(
            filtered_logs,
            self.realtime,
            watching_state_vec.clone(),
            self.hide_logs,
        ))
        .into()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Data(msg) => match msg {
                app::Data::ProcessTracer => {
                    while let Ok(log) = self.receiver.lock().unwrap().try_recv() {
                        // Define the maximum number of logs
                        const MAX_LOGS: usize = 100;

                        // Push the new log
                        self.logs.push_back(log);

                        // If the number of logs exceeds the maximum, remove the oldest one
                        if self.logs.len() > MAX_LOGS {
                            self.logs.pop_front();
                        }

                        // Process the logs.
                        self.firehoses = self.filter_logs_into_firehoses();
                    }

                    Command::none()
                }
            },

            Message::Simulation(msg) => match msg {
                app::Simulation::Spawned(world_manager) => {
                    match world_manager {
                        Ok(world_manager) => {
                            tracing::info!("Simulation world spawned!");
                            return self.handle_startup(world_manager);
                        }
                        Err(e) => {
                            tracing::error!("Simulation world failed to spawn: {:?}", e);
                        }
                    }

                    Command::none()
                }
                app::Simulation::Completed => {
                    tracing::info!("Simulation world completed!");
                    self.status = WorldManagerState::Completed;
                    Command::none()
                }
            },

            Message::View(msg) => match msg {
                view::Message::Settings(msg) => match msg {
                    view::Settings::ToggleRealtime => {
                        self.realtime = !self.realtime;
                        Command::none()
                    }
                    view::Settings::ToggleFirehoseVisibility => {
                        self.hide_logs = !self.hide_logs;
                        Command::none()
                    }
                },

                view::Message::Data(msg) => match msg {
                    view::Data::LogTrace => {
                        tracing::info!("LogTrace message received!");
                        Command::none()
                    }
                    view::Data::UpdateWatchedValue(value) => {
                        // Update the current watched values with the new ones, if any.
                        for (k, v) in value {
                            self.watching_state.insert(k, v);
                        }

                        if self.status == WorldManagerState::Running && !self.realtime {
                            return Command::perform(async { Ok::<(), ()>(()) }, |_| {
                                Message::View(view::Message::Simulation(
                                    view::control::Operation::Step,
                                ))
                            });
                        }

                        Command::none()
                    }

                    _ => Command::none(),
                },

                view::Message::Simulation(msg) => {
                    match msg {
                        view::control::Operation::Spawn => self.handle_spawn(),
                        view::control::Operation::Continue => self.handle_continue(),
                        view::control::Operation::Stop => self.handle_stop(),
                        view::control::Operation::Pause => self.handle_pause(),
                        // todo: I don't like this logic in this place.
                        view::control::Operation::Step => self.handle_step(),
                        view::control::Operation::Agent(msg) => match msg {
                            view::control::AgentOperations::Add => self.handle_add_agent(),
                            _ => Command::none(),
                        },
                    }
                }

                _ => Command::none(),
            },
            _ => Command::none(),
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        let process_tracer_subscription = time::every(Duration::from_millis(100))
            .map(|_| Message::Data(app::Data::ProcessTracer))
            .into();
        let mut subs = vec![process_tracer_subscription];

        if self.status != WorldManagerState::Running || !self.realtime {
            return Subscription::batch(subs);
        }

        // Runs on a 1s timer.
        let step_sim_subscription = time::every(Duration::from_millis(1000))
            .map(|_| Message::View(view::Message::Simulation(view::control::Operation::Step)))
            .into();

        subs.push(step_sim_subscription);
        Subscription::batch(subs)
    }
}
