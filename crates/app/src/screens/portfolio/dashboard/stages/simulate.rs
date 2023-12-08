use std::{collections::BTreeMap, convert::Infallible};

use ethers::{abi::Token, utils::format_ether};
use logging::tracer::AppEventLayer;
use sim::engine::ArbiterInstance;
use uuid::Uuid;

use super::*;
use crate::components::system::label;

type World = Arc<ArbiterInstance>;
type InstanceManager = Arc<tokio::sync::Mutex<ArbiterInstanceManager>>;

#[derive(Debug, Clone, Default)]
pub enum Message {
    #[default]
    Empty,
    /// Trigger preparation of simulation.
    Arm,
    /// Simulation is ready to be executed.
    Armed(ArbiterInstanceManager),
    /// Simulation was called to be run.
    Simulate,
    /// Simulation has completed.
    Complete,
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

/// Single instance of a world with its last block of subscription data.
#[derive(Debug, Clone, Default)]
pub struct WorldOutcome {
    pub world: Option<Arc<tokio::sync::Mutex<World>>>,
}

/// A cache of the worlds that have been simulated, mapped by their world key.
#[derive(Debug, Clone, Default)]
pub struct WorldCache(pub BTreeMap<Uuid, WorldOutcome>);

#[derive(Debug, Clone, Default)]
pub struct Simulate {
    /// Simulation settings form.
    form: Form,
    /// Constructs the simulation instances and exposes configuration options.
    builders: Vec<ArbiterInstanceManager>,
    /// The worlds that have been simulated.
    cache: WorldCache,
    /// Whether the simulation has been armed.
    pub armed: bool,
}

impl Simulate {
    pub type ViewMessage = Message;

    pub fn new(builders: Vec<ArbiterInstanceManager>) -> Self {
        Self {
            builders,
            ..Default::default()
        }
    }

    /// Renders the `store` results.
    pub fn render_simulation_outcome(&self) -> Element<'_, Self::ViewMessage> {
        let action = match self.armed {
            true => Message::Simulate,
            false => Message::Arm,
        };

        let action_cta = match self.armed {
            true => "Simulate".to_string(),
            false => "Arm Simulation".to_string(),
        };

        let mut content = Column::new()
            .spacing(Sizes::Xl)
            .push(label(&"Simulation Results").title1().build())
            .push(action_button(action_cta.to_string()).on_press(action));

        content.into()
    }

    /// Returns an instructions element to guide the user.
    pub fn guide(&self, on_submit: Option<super::Message>) -> Container<'static, super::Message> {
        instructions(
            vec![instruction_text(
                "Review the simulated results and make any desired modifications before continuing to execution.".to_string(),
            )],
            Some("Review Adjustment Transaction".to_string()),
            None,
            on_submit,
        )
    }
}

#[tracing::instrument(level = "trace", skip(m))]
pub async fn sim_startup(m: InstanceManager) -> anyhow::Result<(), anyhow::Error> {
    let locked = m.lock().await;

    // for world in locked.worlds.iter() {
    // let mut world = world.lock().await;
    // let world_id = world.seed;
    // tracing::debug!("world.{}.: Running startup", world_id.clone());
    // world.startup().await?;
    // }

    Ok(())
}

// #[tracing::instrument(level = "trace", skip(m), fields(layer = %"system",
// action = %"step"))] pub async fn handle_state_subscriptions(
// id: Uuid,
// m: Arc<tokio::sync::Mutex<World>>,
// ) -> anyhow::Result<(Uuid, StateSubscriptionStore), Arc<anyhow::Error>> {
// let mut state_data = HashMap::new();
//
// let world = m.lock().await;
// let world_id = world.seed;
// let mut agents = world.agents.lock().await;
//
// truncate the world id to just leave the first three characters
// let formatted_world_id = world_id.clone().to_string();
//
// for agent in agents.0.iter_mut() {
// let subscribed = agent.1.get_subscribed().await.map_err(Arc::new)?;
//
// Skip empty subscriptions to avoid populating the state data with empty
// subscriptions.
// if subscribed.is_empty() {
// continue;
// }
//
// if agent.0.to_lowercase().contains("monitor") {
// state_data.entry(world_id).or_insert(HashMap::new()).insert(
// agent.0.to_string(),
// StateSubscription {
// logs: subscribed,
// label: format!("{} {}", formatted_world_id, agent.0),
// category: AppEventLayer::System,
// id: world_id,
// },
// );
// } else {
// Add the subscribed data as a state subscription inside the hashm ap with keys
// world id -> agent name
// state_data.entry(world_id).or_insert(HashMap::new()).insert(
// agent.0.to_string(),
// StateSubscription {
// logs: subscribed,
// label: format!("{} {}", formatted_world_id, agent.0),
// category: AppEventLayer::Agent,
// id: world_id,
// },
// );
// }
// }
//
// Ok((id, state_data))
// }

impl State for Simulate {
    type AppMessage = Message;
    type ViewMessage = Message;

    fn update(&mut self, message: Self::AppMessage) -> Command<Self::AppMessage> {
        match message {
            Message::Empty => {}
            Message::Arm => {
                tracing::debug!("Arming simulation");
                self.armed = true;
            }
            Message::Armed(builder) => {
                self.builders.push(builder);
            }
            Message::Simulate => {
                return Command::perform(async {}, |_| Message::Complete);
            }
            Message::Complete => {
                /* // Set the cached world.
                let world = match result {
                    Ok(world) => world,
                    Err(error) => {
                        tracing::error!("Error did not get simulation world returned: {:?}", error);
                        return Command::none();
                    }
                };

                let key = Uuid::new_v4();
                self.cache.0.insert(
                    key,
                    WorldOutcome {
                        world: Some(world.clone()),
                        state_data: HashMap::new(),
                    },
                );

                // When the simulation completes, we query the final state of the agent
                // subscriptions in the simulation.
                return Command::perform(
                    handle_state_subscriptions(key, world),
                    Message::FetchResults,
                ); */
            }

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
