use std::{
    collections::{HashMap, VecDeque},
    sync::mpsc::Receiver,
    time::Duration,
};

use anyhow::anyhow;
use iced::time;
use simulation::{
    agents::{counter::CounterAgent, price_changer::PriceChanger, SubscribedData},
    settings::{parameters::Single, SimulationConfig},
};
use tracing::{Instrument, Span};

use super::{
    app::Message,
    tracer::{AppEventLayer, AppEventLog},
    State, *,
};

const USER_ACTION_FILTER: &str = "user.";

pub type StateSubscriptionStore = HashMap<u64, HashMap<String, StateSubscription>>;

pub struct Terminal {
    data_feed: VecDeque<AppEventLog>,
    receiver: Arc<Mutex<Receiver<tracer::AppEventLog>>>,
    world_manager: Arc<tokio::sync::Mutex<WorldManager>>,
    status: WorldManagerState,
    // State subscription from the chain, indexed by world.
    state_data: StateSubscriptionStore,
    realtime: bool,
    hide_firehoses: bool,
}

#[tracing::instrument]
pub async fn spawn() -> anyhow::Result<Arc<tokio::sync::Mutex<WorldManager>>, anyhow::Error> {
    // Override the world manager with a new one that has spawned worlds.
    Ok(Arc::new(tokio::sync::Mutex::new(
        WorldManager::default().spawn(1).await?,
    )))
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

#[tracing::instrument(level = "trace", skip(m), fields(layer = %"system", action = %"step"))]
pub async fn step_simulation(
    m: Arc<tokio::sync::Mutex<WorldManager>>,
) -> anyhow::Result<(), anyhow::Error> {
    let mut m_locked = m.lock().await;
    if let Some(ref mut tx) = m_locked.tx {
        let msg = tx.lock().await.send(1);
        tracing::trace!("Sent message {:?}", msg);
        match msg {
            Ok(_) => {}
            Err(e) => {
                tracing::error!("Simulation world failed to step: {:?}", e);
                return Err(anyhow!("Simulation world failed to step: {:?} ", e));
            }
        }
    }

    Ok(())
}

#[tracing::instrument(level = "trace", skip(m), fields(layer = %"system", action = %"step"))]
pub async fn handle_state_subscriptions(
    m: Arc<tokio::sync::Mutex<WorldManager>>,
) -> anyhow::Result<StateSubscriptionStore, anyhow::Error> {
    let locked = m.lock().await;
    let mut state_data = HashMap::new();

    for world in locked.worlds.iter() {
        let world = world.lock().await;
        let world_id = world.seed;
        let mut agents = world.agents.lock().await;

        // truncate the world id to just leave the first three characters
        let formatted_world_id = world_id.clone().to_string();
        let formatted_world_id = &formatted_world_id[0..3];

        for agent in agents.0.iter_mut() {
            let subscribed = agent.1.get_subscribed().await?;

            // Skip empty subscriptions to avoid populating the state data with empty
            // subscriptions.
            if subscribed.len() == 0 {
                continue;
            }

            if agent.0.to_lowercase().contains("monitor") {
                state_data
                    .entry(world_id.clone())
                    .or_insert(HashMap::new())
                    .insert(
                        agent.0.to_string(),
                        StateSubscription {
                            logs: subscribed,
                            label: format!("{} {}", formatted_world_id, agent.0),
                            category: AppEventLayer::System,
                            id: world_id,
                        },
                    );
            } else {
                // Add the subscribed data as a state subscription inside the hashm ap with keys
                // world id -> agent name
                state_data
                    .entry(world_id.clone())
                    .or_insert(HashMap::new())
                    .insert(
                        agent.0.to_string(),
                        StateSubscription {
                            logs: subscribed,
                            label: format!("{} {}", formatted_world_id, agent.0),
                            category: AppEventLayer::Agent,
                            id: world_id,
                        },
                    );
            }
        }
    }

    Ok(state_data)
}

#[tracing::instrument(level = "trace", skip(m), fields(layer = %"system", action = %"step"))]
pub async fn handle_price_path(
    m: Arc<tokio::sync::Mutex<WorldManager>>,
) -> anyhow::Result<bool, anyhow::Error> {
    let locked = m.lock().await;

    for world in locked.worlds.iter() {
        let world = world.lock().await;
        let mut agents = world.agents.lock().await;

        tracing::debug!("Getting subscribed data");
        for agent in agents.0.iter_mut() {
            let price_changer = agent.1.as_any().downcast_ref::<PriceChanger>();

            match price_changer {
                Some(price_changer) => {
                    let price_path = price_changer.trajectory.paths[0].clone();
                    let current_index = price_changer.index;
                    let last_index = price_path.len();
                    if current_index == last_index {
                        tracing::debug!("Price path is empty, stopping simulation");
                        return Ok(true);
                    }
                }
                _ => {}
            }
        }
    }

    Ok(false)
}

impl Terminal {
    pub fn new(receiver: Arc<Mutex<Receiver<tracer::AppEventLog>>>) -> Self {
        Self {
            data_feed: VecDeque::new(),
            receiver,
            world_manager: Arc::new(tokio::sync::Mutex::new(WorldManager::default())),
            status: WorldManagerState::Stopped,
            state_data: HashMap::new(),
            realtime: false,
            hide_firehoses: false,
        }
    }

    pub fn purge_non_main_logs(&mut self) {
        // Clears all the logs that are not keyed with "system".
        let mut new_data_feed = VecDeque::new();

        for log in self.data_feed.iter() {
            if let Some(metadata) = log.data.get(&AppEventLayer::System) {
                // Take the last element from the data vector and push it to system_logs
                if let Some(_) = metadata.data.last() {
                    new_data_feed.push_back(log.clone());
                }
            }
        }

        self.data_feed = new_data_feed;
    }

    /// Executes the logic when a spawned message is received.
    #[tracing::instrument(
        level = "trace",
        skip(self, world_manager),
        fields(layer = %"system", id = %"spawned", action = %"startup")
    )]
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
    #[tracing::instrument(level = "trace", skip(self), fields(layer = %"user", action = "%continue"))]
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

    #[tracing::instrument(level = "trace", skip(self), fields(layer = %"user", action = %"spawn"))]
    pub fn handle_spawn(&mut self) -> Command<Message> {
        if self.status != WorldManagerState::Stopped {
            tracing::warn!("Simulation already spawned! Stop before spawning a new one.");
            return Command::none();
        }

        tracing::trace!("Spawn simulation message received!");
        self.purge_non_main_logs();

        // Triggers a step, which will start the run loop.
        Command::perform(async { spawn().instrument(terminal_span()).await }, |res| {
            app::Message::WindowsMessage(app::WindowsMessage::Simulation(app::Simulation::Spawned(
                res,
            )))
        })
    }

    // todo: this is triggered on View::Step, but it should be a root level message.
    #[tracing::instrument(level = "trace", skip(self), fields(layer = %"user", action = %"step"))]
    pub fn handle_step(&mut self) -> Command<Message> {
        if self.status == WorldManagerState::Stopped {
            tracing::trace!("Simulation is stopped, cannot step.");
            return Command::none();
        }

        // If paused, we can still step.
        tracing::trace!("Step simulation message received!");

        let m = self.world_manager.clone();

        // Message emitted after step completes.
        let mut exit_msg = Message::Empty;
        if self.status == WorldManagerState::Running && !self.realtime {
            exit_msg = Message::View(view::Message::Simulation(view::control::Operation::Step));
        }

        return Command::perform(
            async move {
                step_simulation(m.clone()).await?;

                let subscription_data = handle_state_subscriptions(m.clone()).await?;
                let finished_path = handle_price_path(m.clone()).await?;

                Ok((subscription_data, finished_path))
            },
            |result: Result<(StateSubscriptionStore, bool), anyhow::Error>| {
                match result {
                    Ok((subscription_data, finished_path)) => {
                        tracing::trace!("Simulation world stepped!");

                        // todo: this short-circuits the last update watched
                        // value!
                        if finished_path {
                            tracing::trace!("Simulation world finished!");
                            return app::Message::WindowsMessage(app::WindowsMessage::Simulation(
                                app::Simulation::Completed,
                            ));
                        }

                        return Message::View(view::Message::Data(view::Data::UpdateWatchedValue(
                            subscription_data,
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

    #[tracing::instrument(level = "trace", skip(self), fields(layer = %"user", action = %"stop"))]
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

    #[tracing::instrument(level = "trace", skip(self), fields(layer = %"user", action = %"pause"))]
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

    #[tracing::instrument(level = "trace", skip(self), fields(layer = %"user", action = %"add_agent"))]
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
                        let name = agent.0;

                        if name.contains("liquidity_provider") {
                            tracing::trace!("world.{}.: Found LP agent", world_id.clone());
                            lp_address = agent.1.get_client().unwrap().address();
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

pub fn terminal_span() -> Span {
    tracing::info_span!("terminal")
}

#[derive(Debug, Clone)]
pub struct StateSubscription {
    pub logs: Vec<SubscribedData>,
    pub label: String,
    pub category: AppEventLayer,
    pub id: u64,
}

impl State for Terminal {
    fn view<'a>(&'a self) -> Element<'a, view::Message> {
        let state_data = self.state_data.clone();
        let mut data = self.data_feed.clone();
        if self.hide_firehoses {
            data = VecDeque::new();
        }

        view::app_layout(
            &view::Page::Terminal,
            view::terminal_layout(self.realtime, state_data.clone()),
        )
        .into()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::WindowsMessage(app::WindowsMessage::Simulation(msg)) => match msg {
                app::Simulation::Spawned(world_manager) => {
                    match world_manager {
                        Ok(world_manager) => {
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
                        self.hide_firehoses = !self.hide_firehoses;
                        Command::none()
                    }
                },

                view::Message::Data(msg) => match msg {
                    view::Data::LogTrace => {
                        trigger_debug_trace();
                        println!("Logs: {:?}", self.data_feed.clone());
                        Command::none()
                    }
                    view::Data::AppEvent => {
                        // Cannot  use trace here.
                        // get the app logs from the main app
                        Command::none()
                    }
                    view::Data::UpdateWatchedValue(value) => {
                        // Update the current watched values with the new ones, if any.
                        self.state_data = value;

                        if self.status == WorldManagerState::Running && !self.realtime {
                            return Command::perform(async { Ok::<(), ()>(()) }, |_| {
                                Message::View(view::Message::Simulation(
                                    view::control::Operation::Step,
                                ))
                            });
                        }

                        Command::none()
                    }
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
            .map(|_| Message::StreamsMessage(app::StreamsMessage::Data(app::Data::ProcessTracer)))
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

#[tracing::instrument(fields(layer = %"user"))]
fn trigger_debug_trace() {
    tracing::info!("LogTrace message received!");
}
