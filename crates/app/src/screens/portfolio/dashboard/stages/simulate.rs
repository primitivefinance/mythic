use clients::arbiter::portfolio_adjustment::{InstanceManager, SpawnedManager};
use logging::tracer::AppEventLayer;
use simulation::agents::{price_changer::PriceChanger, SubscribedData};

use super::*;
use crate::{
    screens::terminal::{StateSubscription, StateSubscriptionStore},
    view::state_render,
};

#[derive(Debug, Clone, Default)]
pub enum Message {
    #[default]
    Empty,
    Submit,
    /// Prepares the simulation.
    Ready(SpawnedManager),
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
    manager: InstanceManager,
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

    pub fn render_simulation_outcome<'a>(&'a self) -> Element<'a, Self::ViewMessage> {
        let mut content = Column::new().push(Text::new("Simulation Results").size(40));
        content = content.push(state_render(self.store.clone()).map(|_| Message::Empty));
        content.into()
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
                tokio::time::sleep(tokio::time::Duration::from_millis(5)).await;
            }
            Err(error) => {
                tracing::error!("Error sending message to manager: {:?}", error);

                // If we need to try again, wait a bit and try again.
                if retries < max_retries {
                    retries += 1;
                    tokio::time::sleep(tokio::time::Duration::from_millis(5 * (retries as u64)))
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
    m: InstanceManager,
) -> anyhow::Result<StateSubscriptionStore, Arc<anyhow::Error>> {
    let locked = m.lock().await;
    let mut state_data = HashMap::new();

    for world in locked.worlds.iter() {
        let world = world.lock().await;
        let world_id = world.seed;
        let mut agents = world.agents.lock().await;

        // truncate the world id to just leave the first three characters
        let formatted_world_id = world_id.clone().to_string();

        for agent in agents.0.iter_mut() {
            let subscribed = agent.1.get_subscribed().await.map_err(|e| Arc::new(e))?;

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

impl State for Simulate {
    type AppMessage = Message;
    type ViewMessage = Message;

    fn update(&mut self, message: Self::AppMessage) -> Command<Self::AppMessage> {
        match message {
            Message::Ready(manager) => match manager {
                Ok(manager) => {
                    tracing::info!("Manager spawned!");
                    self.manager = manager;
                    let m = self.manager.clone();
                    return Command::perform(sim_startup(m), |_| Message::Empty);
                }
                Err(error) => {
                    tracing::error!("Error spawning manager: {:?}", error);
                }
            },
            Message::Simulate => {
                return Command::perform(run_simulation(self.manager.clone()), |_| {
                    Message::Complete
                });
            }
            Message::Empty => {}
            Message::Submit => {}
            Message::Complete => {
                // When the simulation completes, we want to get the ending
                // reserves.
                let m = self.manager.clone();
                return Command::perform(handle_state_subscriptions(m), Message::Outcome);
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

    fn view<'a>(&'a self) -> Element<'a, Self::ViewMessage> {
        let mut content = Column::new();

        content = content.push(button("Simulate!").on_press(Message::Simulate));

        // content = content.push(
        // Column::new()
        // .spacing(Sizes::Md)
        // .push(
        // Row::with_children(
        // self.metrics
        // .iter()
        // .map(|x| Card::new(label_item(x.to_string())).into())
        // .collect::<Vec<Element<'a, Message>>>(),
        // )
        // .spacing(Sizes::Md),
        // )
        // .push(
        // Row::new()
        // .push(
        // Column::new()
        // .width(Length::FillPortion(3))
        // .push(label_item("Simulation Results".to_string()))
        // .push(
        // Row::with_children(
        // vec![
        // Card::new(label_item("results 1".to_string())).into(),
        // Card::new(label_item("results 2".to_string())).into(),
        // ]
        // .into_iter()
        // .collect::<Vec<Element<'a, Message>>>(),
        // )
        // .spacing(Sizes::Md),
        // ),
        // )
        // .push(
        // instructions(
        // vec![label_item("Execute the Adjustment".to_string())],
        // None,
        // None,
        // Some(Message::Submit),
        // )
        // .width(Length::FillPortion(1)),
        // ),
        // ),
        // );

        content = content.push(self.render_simulation_outcome());

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
