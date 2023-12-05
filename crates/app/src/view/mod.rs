use std::collections::{BTreeMap, HashMap};

use ethers::{abi::Token, utils::format_ether};
use iced::widget::{Column, Container, Row};

use self::sidebar::Page;
use super::{
    components::{containers::*, *},
    tracer::AppEventLayer,
    *,
};
use crate::screens::State;

pub mod sidebar;

/// Root message for the Terminal component.
#[derive(Debug, Clone, Default)]
pub enum Message {
    #[default]
    Empty,
    // Exit application
    Exit,
    // Confirm exit application
    ConfirmExit,
    // Route to a new page.
    Route(sidebar::Route),
    // Copy to clipboard.
    CopyToClipboard(String),
    // Specific window messages.
    CreatePortfolio(portfolio::create::Message),
    Dashboard(portfolio::dashboard::Message),
}

impl MessageWrapperView for Message {
    type ParentMessage = Message;
}

impl MessageWrapper for Message {
    type ParentMessage = app::Message;
}

impl From<Message> for app::Message {
    fn from(message: Message) -> Self {
        app::Message::View(message)
    }
}

pub fn app_layout<'a, T: Into<Element<'a, Message>>>(
    menu: &'a sidebar::Sidebar,
    content: T,
) -> Element<'a, Message> {
    Container::new(
        Row::new()
            .push(
                Column::new()
                    .push(menu.view())
                    .width(Length::FillPortion(1)),
            )
            .push(
                Column::new()
                    .push(screen_layout(&menu.page, content))
                    .width(Length::FillPortion(5)),
            ),
    )
    .style(BackgroundContainerTheme::theme())
    .width(Length::Fill)
    .height(Length::Fill)
    .center_x()
    .center_y()
    .into()
}

/// For rendering content inside a screen that implements [`State`].
pub fn screen_layout<'a, T: Into<Element<'a, Message>>>(
    window: &'a Page,
    content: T,
) -> Element<'a, Message> {
    Container::new(screen_window(window, content))
        .center_x()
        .center_y()
        .align_x(alignment::Horizontal::Center)
        .align_y(alignment::Vertical::Center)
        .width(Length::Shrink)
        .height(Length::Shrink)
        .padding(Sizes::Xl as u16)
        .into()
}

#[derive(Debug, Clone)]
pub struct SubscribedData {
    pub name: String,
    pub data: Token,
}

impl SubscribedData {
    pub fn new(name: String, data: Token) -> Self {
        Self { name, data }
    }
}

type SubscriptionViewWrapper = BTreeMap<u64, BTreeMap<String, Vec<(String, String)>>>;

#[derive(Debug, Clone)]
pub struct StateSubscription {
    pub logs: Vec<SubscribedData>,
    pub label: String,
    pub category: AppEventLayer,
    pub id: u64,
}

pub type StateSubscriptionStore = HashMap<u64, HashMap<String, StateSubscription>>;

// todo: lot of logic to handle in here, maybe we parse it further up in the
// app?
pub fn state_render<'a>(state_data: StateSubscriptionStore) -> Element<'a, Message> {
    let mut agent_data: SubscriptionViewWrapper = BTreeMap::new();
    let mut monitored_data: SubscriptionViewWrapper = BTreeMap::new();

    let empty_map = BTreeMap::new();

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

            match category {
                AppEventLayer::Agent => {
                    // Each agent should have one tuple element of (name, label)
                    agent_data
                        .entry(world_id)
                        .or_insert(empty_map.clone())
                        .entry(agent_name.clone())
                        .or_default()
                        .push(("name".to_string(), label.clone()));
                    for log in logs {
                        let name = log.name.clone();
                        let value = log.data.clone();

                        let mut signed = false;
                        let value_int = value.clone().into_int();
                        let value_uint = match value_int {
                            Some(value) => {
                                signed = true;
                                I256::from_raw(value)
                                    .checked_abs()
                                    .map(|x| x.twos_complement())
                                    .unwrap_or_default()
                            }
                            None => value.into_uint().unwrap_or_default(),
                        };

                        // todo: this is hardcoded parsing... it could become wrong easily.
                        let formatted = format_ether(value_uint).parse::<f64>().unwrap_or_default();
                        let sign = if signed { "-" } else { "" };
                        // truncated
                        let truncated = format!("{}{:.2}", sign, formatted);

                        agent_data
                            .entry(world_id)
                            .or_insert(empty_map.clone())
                            .entry(agent_name.clone())
                            .or_default()
                            .push((name, truncated));
                    }
                }
                _ => {
                    for log in logs {
                        let name = log.name.to_string();
                        let value = log.data.clone();
                        let mut signed = false;
                        let value_int = value.clone().into_int();
                        let value_uint = match value_int {
                            Some(value) => {
                                signed = true;
                                I256::from_raw(value)
                                    .checked_abs()
                                    .map(|x| x.twos_complement())
                                    .unwrap_or_default()
                            }
                            None => value.into_uint().unwrap_or_default(),
                        };
                        // todo: this is hardcoded parsing... it could become wrong easily.
                        let formatted = format_ether(value_uint).parse::<f64>().unwrap_or_default();
                        let sign = if signed { "-" } else { "" };
                        // truncated
                        let truncated = format!("{}{:.2}", sign, formatted);

                        monitored_data
                            .entry(world_id)
                            .or_insert(empty_map.clone())
                            .entry(agent_name.clone())
                            .or_default()
                            .push((name, truncated));
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

    let agent_groups_title = data_item("Agents".to_string()).size(36);

    scrollable(
        Column::new()
            .push(data_item("Protocol".to_string()).size(36))
            .push(monitored_groups)
            .push(agent_groups_title)
            .push(agent_groups)
            .spacing(Sizes::Md as u16)
            .width(Length::Fill),
    )
    .into()
}

/// Renders a grid of agents cards, with a maximum amount of cards per row.
pub fn agent_card_grid<'a, Message>(
    data: Vec<Vec<(String, String)>>,
    max: usize,
) -> Element<'a, Message>
where
    Message: 'a,
{
    let mut content = Column::new();
    let mut row = Row::new().spacing(Sizes::Lg as u16);
    let mut i = 0;
    for card in data.into_iter() {
        row = row.push(labeled_data_cards(card, 4));
        i += 1;
        if i == max {
            content = content.push(row);
            row = Row::new().spacing(Sizes::Lg as u16);
            i = 0;
        }
    }
    content = content.push(row);
    content.spacing(Sizes::Lg as u16).into()
}

/// Renders a single piece of labeled data in a container with a panel
/// background and padding.
pub fn labeled_data_card<'a, Message>(
    label: String,
    data: String,
    _max_width: u16,
) -> Element<'a, Message>
where
    Message: 'a,
{
    let mut content = Column::new()
        .push(labeled_data(label, data))
        .width(Length::Fixed(100.0));
    content = content.spacing(Sizes::Sm as u16);
    Card::new(container(content))
        .padding(Sizes::Md as u16)
        .into()
}

/// Renders a group of labeled data cards in a row with a maximum amount of
/// elements per row. If the total amount of elements exceeds the maximum, it
/// will push a new row inside the column. There is a group label rendered above
/// the rows.
pub fn labeled_data_cards<'a, Message>(
    data: Vec<(String, String)>,
    max_elements: usize,
) -> Element<'a, Message>
where
    Message: 'a,
{
    let mut content = Column::new();

    let mut row = Row::new().spacing(Sizes::Sm as u16);
    let mut i = 0;
    for (label, data) in data {
        row = row.push(labeled_data_card(label, data, 200));
        i += 1;
        if i == max_elements {
            content = content.push(row);
            row = Row::new().spacing(Sizes::Sm as u16);
            i = 0;
        }
    }
    content = content.push(row);
    content.spacing(16).into()
}
