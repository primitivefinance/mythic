use std::collections::BTreeMap;

use clients::arbiter::{
    portfolio_adjustment::{InstanceManager, SimulatedWorld},
    world::World,
};
use ethers::{abi::Token, utils::format_ether};
use logging::tracer::AppEventLayer;
use simulation::agents::SubscribedData;

use super::*;
use crate::{
    screens::terminal::{StateSubscription, StateSubscriptionStore},
    view::{agent_card_grid, monitor::labeled_data_cards},
};

#[derive(Debug, Clone, Default)]
pub enum Message {
    #[default]
    Empty,
    Submit,
    /// Prepares the simulation.
    Ready(SimulatedWorld),
    /// Starts the simulation run.
    Simulate,
    /// Simulation run complete.
    Complete,
    /// Fetched the final state of the simulation.
    Outcome(anyhow::Result<StateSubscriptionStore, Arc<anyhow::Error>>),
}

impl MessageWrapperView for Message {
    type ParentMessage = super::Message;
}

impl MessageWrapper for Message {
    type ParentMessage = super::Message;
}

impl From<Message> for <Message as MessageWrapper>::ParentMessage {
    fn from(message: Message) -> Self {
        Self::Simulate(message)
    }
}

#[derive(Debug, Clone, Default)]
pub struct Form {}

#[derive(Debug, Clone, Default)]
pub struct Simulate {
    builders: Vec<MiniWorldBuilder>,
    form: Form,
    metrics: Vec<String>,
    world: Option<Arc<tokio::sync::Mutex<World>>>,
    store: StateSubscriptionStore,
}

impl Simulate {
    pub type ViewMessage = Message;

    pub fn new(builders: Vec<MiniWorldBuilder>) -> Self {
        Self {
            builders,
            store: HashMap::new(),
            ..Default::default()
        }
    }

    pub fn render_simulation_outcome(&self) -> Element<'_, Self::ViewMessage> {
        let mut content = Column::new()
            .spacing(Sizes::Xl)
            .push(h1("Simulation Results".to_string()));
        let results = render_simulation_results(self.store.clone());
        content = content.push(results);
        content.into()
    }

    /// Returns an instructions element to guide the user.
    pub fn guide(&self, on_submit: Option<super::Message>) -> Container<'static, super::Message> {
        instructions(
            vec![instruction_text(
                "Review the simulated results and make any desired modifications before continuing to execution.".to_string(),
            )],
            Some("Execute Adjustment".to_string()),
            None,
            on_submit,
        )
    }
}

#[tracing::instrument(skip(m), level = "debug")]
pub async fn run_simulation(m: InstanceManager) -> anyhow::Result<(), anyhow::Error> {
    // Acquire the lock on the manager.
    let mut locked = m.lock().await;

    let max_retries: usize = 10;
    let mut retries: usize = 0;

    // Get a reference to the broadcast channel.
    // todo does this work??
    'outer: while let Some(ref mut tx) = locked.tx {
        // First check if we reached the end of the price path, a condition to break the
        // loop.
        // todo: difficult to get this working.

        // Broadcasting a message triggers the loop that is waiting to receive a
        // message, which actually does the `step` call.
        let msg = tx.lock().await.send(1);
        tracing::debug!("Sent message: {:?}", msg);

        match msg {
            Ok(_) => {
                tracing::info!("Sent message to manager!");
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            }
            Err(error) => {
                tracing::error!("Error sending message to manager: {:?}", error);

                // If we need to try again, wait a bit and try again.
                if retries < max_retries {
                    retries += 1;
                    tokio::time::sleep(tokio::time::Duration::from_millis(1000 * (retries as u64)))
                        .await;
                    continue 'outer;
                }

                // Break on the max retries.
                if retries >= max_retries {
                    break;
                }
            }
        }
    }

    Ok(())
}

#[tracing::instrument(level = "trace", skip(m))]
pub async fn sim_startup(m: InstanceManager) -> anyhow::Result<(), anyhow::Error> {
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
pub async fn handle_state_subscriptions(
    m: Arc<tokio::sync::Mutex<World>>,
) -> anyhow::Result<StateSubscriptionStore, Arc<anyhow::Error>> {
    let mut state_data = HashMap::new();

    let world = m.lock().await;
    let world_id = world.seed;
    let mut agents = world.agents.lock().await;

    // truncate the world id to just leave the first three characters
    let formatted_world_id = world_id.clone().to_string();

    for agent in agents.0.iter_mut() {
        let subscribed = agent.1.get_subscribed().await.map_err(Arc::new)?;

        // Skip empty subscriptions to avoid populating the state data with empty
        // subscriptions.
        if subscribed.is_empty() {
            continue;
        }

        if agent.0.to_lowercase().contains("monitor") {
            state_data.entry(world_id).or_insert(HashMap::new()).insert(
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
            state_data.entry(world_id).or_insert(HashMap::new()).insert(
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

    Ok(state_data)
}

impl State for Simulate {
    type AppMessage = Message;
    type ViewMessage = Message;

    fn update(&mut self, message: Self::AppMessage) -> Command<Self::AppMessage> {
        match message {
            Message::Ready(world) => match world {
                Ok(world) => {
                    tracing::info!("Simulation finished!");
                    self.world = Some(world);
                    let m = self.world.clone().unwrap();
                    return Command::perform(handle_state_subscriptions(m), Message::Outcome);
                }
                Err(error) => {
                    tracing::error!("Error spawning manager: {:?}", error);
                }
            },
            Message::Simulate => {
                return Command::perform(async {}, |_| Message::Complete);
            }
            Message::Empty => {}
            Message::Submit => {}
            Message::Complete => {
                // When the simulation completes, we want to get the ending
                // reserves.
                if let Some(world) = self.world.clone() {
                    let m = world.clone();
                    return Command::perform(handle_state_subscriptions(m), Message::Outcome);
                }
            }
            // Load the state data into the store so it can be rendered.
            Message::Outcome(state_data) => match state_data {
                Ok(state_data) => {
                    tracing::info!("Successfully fetched state data: {:?}", state_data);
                    self.store = state_data;
                }
                Err(error) => {
                    tracing::error!("Error getting state data: {:?}", error);
                }
            },

            _ => {}
        }

        Command::none()
    }

    fn view(&self) -> Element<'_, Self::ViewMessage> {
        Container::new(self.render_simulation_outcome())
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .into()
    }
}

type SubscriptionViewWrapper = BTreeMap<u64, BTreeMap<String, Vec<(String, String)>>>;

pub fn render_simulation_results<'a, Message>(
    state_data: StateSubscriptionStore,
) -> Element<'a, Message>
where
    Message: 'a,
{
    let empty_map = BTreeMap::new();
    let mut agent_data: SubscriptionViewWrapper = BTreeMap::new();
    let mut monitored_data: SubscriptionViewWrapper = BTreeMap::new();

    let cloned: StateSubscriptionStore = state_data.clone();
    for (world_id, world_data) in cloned.into_iter() {
        // todo: handle rendering for multiple worlds, should probably be grouped.

        let cloned_world: HashMap<String, StateSubscription> = world_data.clone();

        // Exit early if world data is empty
        if cloned_world.is_empty() {
            continue;
        }

        for (agent_name, agent) in cloned_world.into_iter() {
            let label = agent.label.clone();
            let category: AppEventLayer = agent.category.clone();
            let logs: Vec<SubscribedData> = agent.logs.clone();

            // Insert the agent label if it has non empty state subscriptions
            if agent.logs.is_empty() {
                continue;
            }

            for log in logs {
                let name = log.name.clone();
                let value = log.data.clone();
                let (signed, value_uint) = parse_token_int(value);
                let formatted = format_int_wad(signed, value_uint);

                match category {
                    AppEventLayer::Agent => {
                        // Each agent should have one tuple element of (name, label)
                        agent_data
                            .entry(world_id)
                            .or_insert(empty_map.clone())
                            .entry(agent_name.clone())
                            .or_default()
                            .push(("name".to_string(), label.clone()));

                        agent_data
                            .entry(world_id)
                            .or_insert(empty_map.clone())
                            .entry(agent_name.clone())
                            .or_default()
                            .push((name, formatted));
                    }
                    _ => {
                        monitored_data
                            .entry(world_id)
                            .or_insert(empty_map.clone())
                            .entry(agent_name.clone())
                            .or_default()
                            .push((name, formatted));
                    }
                }
            }
        }
    }

    let mut agent_groups = Column::new().spacing(16);

    for (_world_id, world_data) in agent_data.into_iter() {
        let mut agent_cards = Vec::new();
        for (_agent_name, agent) in world_data.into_iter() {
            agent_cards.push(agent);
        }

        agent_groups = agent_groups.push(agent_card_grid(agent_cards, 4));
    }

    let mut monitored_groups = Column::new().spacing(Sizes::Md as u16);

    for (_world_id, world_data) in monitored_data.into_iter() {
        let mut monitored_cards = Vec::new();
        for (_agent_name, agent) in world_data.into_iter() {
            monitored_cards.push(agent);
        }

        let first_elemn = monitored_cards.clone()[0].clone();
        monitored_groups = monitored_groups.push(labeled_data_cards(first_elemn, 6));
    }

    Column::new()
        .push(monitored_groups)
        .push(agent_groups)
        .spacing(Sizes::Lg as u16)
        .width(Length::Fill)
        .into()
}

pub fn parse_token_int(value: Token) -> (bool, U256) {
    let mut signed = false;
    let value_int = value.clone().into_int();
    let value_uint = match value_int {
        Some(value) => {
            let raw = I256::from_raw(value);
            signed = raw.sign() == Sign::Negative;
            I256::from_raw(value)
                .checked_abs()
                .map(|x| x.twos_complement())
                .unwrap_or_default()
        }
        None => value.into_uint().unwrap_or_default(),
    };

    (signed, value_uint)
}

pub fn format_int_wad(signed: bool, value: U256) -> String {
    let formatted = format_ether(value).parse::<f64>().unwrap_or_default();
    let sign = if signed { "-" } else { "" };
    // truncated
    format!("{}{:.2}", sign, formatted)
}
