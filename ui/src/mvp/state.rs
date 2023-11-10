use std::{
    collections::{HashMap, VecDeque},
    sync::mpsc::Receiver,
    time::Duration,
};

use anyhow::anyhow;
use iced::{futures::io::Empty, time};
use simulation::{
    agents::counter::CounterAgent,
    settings::{parameters::Single, SimulationConfig},
};

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
    watching_state: Vec<String>,
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
            watching_state: vec!["counter: 0".to_string()],
        }
    }

    pub fn spawn(&mut self) {
        if self.status != WorldManagerState::Stopped {
            tracing::warn!("Simulation world already running!");
            return;
        }

        // Override the world manager with a new one that has spawned worlds.
        self.world_manager = Arc::new(tokio::sync::Mutex::new(
            WorldManager::default().spawn(3).unwrap(),
        ));
    }

    pub fn continue_simulation(&mut self) {
        if self.status == WorldManagerState::Running {
            tracing::warn!("Simulation world already running!");
            return;
        }

        self.status = WorldManagerState::Running;
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
}

impl State for Terminal {
    fn view<'a>(&'a self) -> Element<'a, view::Message> {
        let filtered_logs = self.firehoses.clone();
        view::app_layout(view::terminal_view_multiple_firehose(
            filtered_logs,
            self.realtime,
            self.watching_state.clone(),
        ))
        .into()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ProcessTracer => {
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
            Message::View(msg) => match msg {
                view::Message::UpdateWatchedValue(value) => {
                    self.watching_state[0] = value;
                    if self.status == WorldManagerState::Running && !self.realtime {
                        return Command::perform(async { Ok::<(), ()>(()) }, |_| {
                            Message::View(view::Message::Simulation(view::SimulationMessage::Step))
                        });
                    }

                    Command::none()
                }
                view::Message::AddAgent => {
                    tracing::warn!(
                        "{}.{}: Add agent message received!",
                        USER_ACTION_FILTER,
                        "add_agent"
                    );
                    let m = self.world_manager.clone();
                    return Command::perform(
                        async move {
                            let locked = m.lock().await;
                            let world = locked.worlds[0].lock().await;

                            tracing::info!("Adding a counter agent");

                            let direct_configs: Vec<SimulationConfig<Single>> =
                                world.config.clone().into();

                            // Create a new agent
                            let counter_agent = CounterAgent::new(
                                &world.arbiter,
                                &direct_configs[0],
                                "counter".to_string(),
                            )
                            .await
                            .unwrap();

                            {
                                // Change the agents in the world then drops the lock.
                                let mut agents = world.agents.lock().await;

                                tracing::trace!("Current agents: {}", agents.0.len());
                                agents.add(counter_agent);
                                tracing::trace!("New agents: {}", agents.0.len());
                            }
                        },
                        |_| Message::Empty,
                    );
                }
                view::Message::ToggleRealtime => {
                    self.realtime = !self.realtime;
                    Command::none()
                }
                view::Message::Simulation(msg) => match msg {
                    view::SimulationMessage::Spawn => {
                        if self.status != WorldManagerState::Stopped {
                            tracing::warn!(
                                "{}.{}: Simulation already spawned! Stop before spawning a new one.", USER_ACTION_FILTER, "spawn"
                            );
                            return Command::none();
                        }

                        tracing::trace!("Spawn simulation message received!");
                        self.spawn();
                        self.purge_non_main_logs();

                        // Triggers a step, which will start the run loop.
                        return Command::perform(async { Ok::<(), ()>(()) }, |_| {
                            Message::View(view::Message::Simulation(
                                view::SimulationMessage::Continue,
                            ))
                        });
                    }
                    view::SimulationMessage::Continue => {
                        // Return early if paused.

                        tracing::trace!(
                            "{}.{}: Start simulation message received!",
                            USER_ACTION_FILTER,
                            "start"
                        );
                        self.continue_simulation();

                        // Triggers a step, which will start the run loop.
                        return Command::perform(async { Ok::<(), ()>(()) }, |_| {
                            Message::View(view::Message::Simulation(view::SimulationMessage::Step))
                        });
                    }
                    view::SimulationMessage::Stop => {
                        tracing::trace!(
                            "{}.{}: Stop simulation world message received!",
                            USER_ACTION_FILTER,
                            "stop"
                        );
                        self.status = WorldManagerState::Stopped;

                        // If world manager is set, then we need to stop it.
                        let m = self.world_manager.clone();
                        return Command::perform(
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
                                        tracing::error!(
                                            "Simulation world failed to stop:
                         {:?}",
                                            e
                                        );
                                    }
                                }
                                Message::Empty
                            },
                        );
                    }
                    view::SimulationMessage::Pause => {
                        tracing::trace!(
                            "{}.{}: Pause simulation message received!",
                            USER_ACTION_FILTER,
                            "pause"
                        );
                        self.status = WorldManagerState::Paused;

                        let mut m = self.world_manager.clone();
                        return Command::perform(
                            async move {
                                let locked = m.lock().await;

                                if locked.status() == WorldManagerState::Running {
                                    return locked.pause().await;
                                }

                                Ok(())
                            },
                            |_| Message::Empty,
                        );
                    }
                    // todo: I don't like this logic in this place.
                    view::SimulationMessage::Step => {
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
                            exit_msg = Message::View(view::Message::Simulation(
                                view::SimulationMessage::Step,
                            ));
                        }

                        return Command::perform(
                            async move {
                                let mut m_locked = m.lock().await;
                                if let Some(ref mut tx) = m_locked.tx {
                                    let msg = tx.lock().await.send(1);
                                    tracing::trace!("{} Sent message {:?}", trace_id, msg);
                                    match msg {
                                        Ok(_) => {
                                            tracing::trace!(
                                                "{} Simulation world stepped!",
                                                trace_id
                                            );
                                        }
                                        Err(e) => {
                                            tracing::error!(
                                                "{}  Simulation world failed to step:
                            {:?}",
                                                trace_id,
                                                e
                                            );

                                            return Err(anyhow!(
                                                "{} Simulation world failed to step:
                                            ",
                                                trace_id
                                            ));
                                        }
                                    }
                                }

                                // get the agent counter, if it exists, and save its current count.
                                let mut agent_counter: u128 = 0;
                                let world = m_locked.worlds[0].lock().await;
                                let mut agents = world.agents.lock().await;
                                for agent in agents.0.iter_mut() {
                                    let counter_agent =
                                        agent.as_any().downcast_ref::<CounterAgent>();

                                    match counter_agent {
                                        Some(counter_agent) => {
                                            agent_counter =
                                                counter_agent.get().await.unwrap().as_u128();
                                        }
                                        None => {}
                                    }
                                }

                                Ok(agent_counter)
                            },
                            |result| {
                                match result {
                                    Ok(count) => {
                                        if count > 0 {
                                            tracing::trace!(
                                                "{}.{}: Agent counter: {}",
                                                USER_ACTION_FILTER,
                                                "user-added-agent",
                                                count
                                            );

                                            return Message::View(
                                                view::Message::UpdateWatchedValue(format!(
                                                    "counter: {}",
                                                    count
                                                )),
                                            );
                                        }
                                        tracing::trace!("Simulation world stepped!");
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
                },
                view::Message::LogTrace => {
                    tracing::info!("LogTrace message received!");
                    Command::none()
                }
                _ => Command::none(),
            },
            _ => Command::none(),
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        let process_tracer_subscription = time::every(Duration::from_millis(100))
            .map(|_| Message::ProcessTracer)
            .into();
        let mut subs = vec![process_tracer_subscription];

        if self.status != WorldManagerState::Running || !self.realtime {
            return Subscription::batch(subs);
        }

        // Runs on a 1s timer.
        let step_sim_subscription = time::every(Duration::from_millis(1000))
            .map(|_| Message::View(view::Message::Simulation(view::SimulationMessage::Step)))
            .into();

        subs.push(step_sim_subscription);
        Subscription::batch(subs)
    }
}
