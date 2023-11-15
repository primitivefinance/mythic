use std::collections::{BTreeMap, HashMap, VecDeque};

use ethers::utils::format_ether;
use iced::widget::{checkbox, text_input, Column, Container, Row};
use simulation::agents::SubscribedData;

use self::{
    control::control_panel,
    event::{mock_event_groups, EventFeed},
    feed::Feed,
    monitor::{labeled_data_card, labeled_data_cards},
};
use super::{
    column,
    components::{containers::*, *},
    state::{StateSubscription, StateSubscriptionStore},
    tracer::{AppEventLayer, AppEventLog},
    *,
};

pub mod agent;
pub mod control;
pub mod event;
pub mod execute;
pub mod feed;
pub mod monitor;

/// Messages emitted from user interaction with the settings.
#[derive(Debug, Clone)]
pub enum Settings {
    ToggleRealtime,
    ToggleFirehoseVisibility,
}

/// Messages emitted from user interaction with event_data components.
#[derive(Debug, Clone)]
pub enum Data {
    // for debugging...
    LogTrace,
    // todo: this needs a refactor
    UpdateWatchedValue(StateSubscriptionStore),
}

/// Root message for the Terminal component.
#[derive(Debug, Clone)]
pub enum Message {
    Empty,
    Simulation(control::Operation),
    Settings(Settings),
    Data(Data),
    Page(Page),
    Execution(Execution),
}

#[derive(Debug, Clone)]
pub enum Execution {
    Next,
    Previous,
}

pub fn app_layout<'a, T: Into<Element<'a, Message>>>(
    menu: &Page,
    content: T,
) -> Element<'a, Message> {
    container(row![
        Column::new()
            .push(page_menu(menu))
            .width(Length::FillPortion(1)),
        column![container(
            column![content.into()]
                .width(Length::Fill)
                .height(Length::Fill)
        )
        .center_x()
        .width(Length::Fill)
        .height(Length::Fill)]
        .width(Length::FillPortion(8))
    ])
    .style(BackgroundContainerTheme::theme())
    .width(Length::Fill)
    .height(Length::Fill)
    .center_x()
    .center_y()
    .into()
}

#[derive(Debug, Clone)]
pub enum Page {
    Terminal,
    Execute,
}

pub fn page_menu<'a>(menu: &Page) -> Container<'a, Message> {
    let terminal_button = button(text("terminal")).on_press(Message::Page(Page::Terminal));
    let transact_button = button(text("execute")).on_press(Message::Page(Page::Execute));

    Container::new(
        Column::new().push(
            Column::new()
                .push(terminal_button)
                .push(transact_button)
                .height(Length::Fill)
                .spacing(8),
        ),
    )
    .style(MenuContainerTheme::theme())
    .padding(16)
}

pub fn input_row<'a>() -> Element<'a, Message> {
    let input = text_input("0x", "value").padding(8).width(Length::Fill);

    let button = button(text("send")).padding(8);

    Row::new().push(input).push(button).into()
}

pub fn execution_view<'a>(step: execution::TransactionSteps) -> Element<'a, Message> {
    let mut content = Column::new().spacing(16).padding(32).width(Length::Fill);

    let title = data_item("execution".to_string()).size(36);
    let input = input_row();
    let input2 = input_row();

    let action_column = Column::new()
        .width(Length::FillPortion(2))
        .spacing(32)
        .push(label_item("action".to_string()).size(28))
        .push(input)
        .push(label_item("action".to_string()).size(28))
        .push(input2)
        .push(
            Row::new()
                .push(label_item("transaction cost".to_string()))
                .push(text("$20.00")),
        );

    let summary = Column::new()
        .spacing(4)
        .push(label_item("summary".to_string()).size(16))
        .push(text("Transaction will succeed"))
        .push("Transaction has warnings");

    let review_button = Column::new()
        .height(Length::Fill)
        .width(Length::Shrink)
        .push(
            Row::new()
                .height(Length::Fill)
                .push(
                    Column::new().align_items(alignment::Alignment::End).push(
                        button(text("review"))
                            .padding(8)
                            .on_press(Message::Execution(Execution::Next)),
                    ),
                )
                .align_items(alignment::Alignment::End),
        );

    let info_column = Column::new()
        .height(Length::Fill)
        .width(Length::FillPortion(2))
        .spacing(16)
        .push(summary)
        .push(review_button);

    let content_row = Row::new()
        .width(Length::Fill)
        .spacing(8)
        .align_items(alignment::Alignment::Center)
        .push(action_column)
        .push(info_column);

    content = content.push(title).push(content_row);
    let center_container = Container::new(content)
        .width(Length::Fixed(800.0))
        .height(Length::Fixed(800.0))
        .style(MenuContainerTheme::theme());

    Container::new(center_container)
        .center_x()
        .center_y()
        .align_x(alignment::Horizontal::Center)
        .align_y(alignment::Vertical::Center)
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(36)
        .into()
}

pub fn terminal_view_multiple_firehose<'a>(
    event_data: VecDeque<AppEventLog>,
    realtime: bool,
    state_data: StateSubscriptionStore,
    firehose_visible: bool,
) -> Element<'a, Message> {
    let cloned = state_data.clone();

    let control_view = control_panel(vec![], realtime, firehose_visible);

    let event_view = EventFeed {
        events: mock_event_groups(),
    }
    .view();

    let mut feed = Feed::new(20);
    feed.vec_to_bucketed_logs(event_data);

    let state_view = state_render(cloned);

    let mut feed_column = Column::new();
    let mut feed_row = Column::new().spacing(8);
    let mut feed_names: Vec<_> = feed.buckets.keys().cloned().collect();
    feed_names.sort();
    feed_names.reverse();
    for feed_name in feed_names {
        feed_row = feed_row.push(feed.view(feed_name.as_str()));
    }
    feed_column = feed_column.push(scrollable(feed_row));

    let content_row = Row::new()
        .push(Column::new().push(state_view).width(Length::FillPortion(3)))
        .push(feed_column)
        .width(Length::Fill);

    Column::new()
        .push(
            container(control_view)
                .padding(8)
                .style(MenuContainerTheme::theme())
                .width(Length::Fill),
        )
        .push(content_row)
        .spacing(16)
        .padding(16)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

type SubscriptionViewWrapper = BTreeMap<u64, BTreeMap<String, Vec<(String, String)>>>;

// todo: lot of logic to handle in here, maybe we parse it further up in the
// app?
fn state_render<'a>(state_data: StateSubscriptionStore) -> Element<'a, Message> {
    let mut agent_data: SubscriptionViewWrapper = BTreeMap::new();
    let mut monitored_data: SubscriptionViewWrapper = BTreeMap::new();

    let empty_map = BTreeMap::new();

    let cloned: StateSubscriptionStore = state_data.clone();

    for (i, (world_id, world_data)) in cloned.into_iter().enumerate() {
        // todo: handle rendering for multiple worlds, should probably be grouped.
        // if i > 0 {
        // continue;
        // }

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
                        .or_insert(vec![])
                        .push(("name".to_string(), label.clone()));
                    for log in logs {
                        let name = log.name.clone();
                        let value = log.data.clone();

                        // todo: this can easily be 0...
                        let value_uint = value.into_uint().unwrap_or_default();
                        // todo: this is hardcoded parsing... it could become wrong easily.
                        let formatted = format_ether(value_uint).parse::<f64>().unwrap_or_default();
                        // truncated
                        let truncated = format!("{:.2}", formatted);

                        agent_data
                            .entry(world_id)
                            .or_insert(empty_map.clone())
                            .entry(agent_name.clone())
                            .or_insert(vec![])
                            .push((name, truncated));
                    }
                }
                _ => {
                    for log in logs {
                        let name = format!("{}: {}", label.clone(), log.name);
                        let value = log.data.clone();
                        // todo: this can easily be 0...
                        let value_uint = value.into_uint().unwrap_or_default();
                        // todo: this is hardcoded parsing... it could become wrong easily.
                        let formatted = format_ether(value_uint).parse::<f64>().unwrap_or_default();
                        // truncated
                        let truncated = format!("{:.2}", formatted);

                        monitored_data
                            .entry(world_id)
                            .or_insert(empty_map.clone())
                            .entry(agent_name.clone())
                            .or_insert(vec![])
                            .push((name, truncated));
                    }
                }
            }
        }
    }

    let mut agent_groups = Column::new().spacing(16);

    for (world_id, world_data) in agent_data.into_iter() {
        let mut agent_cards = Vec::new();
        for (agent_name, agent) in world_data.into_iter() {
            agent_cards.push(agent);
        }

        agent_groups = agent_groups.push(agent_card_grid(agent_cards, 2));
    }

    let mut monitored_groups = Column::new().spacing(16);

    for (world_id, world_data) in monitored_data.into_iter() {
        let mut monitored_cards = Vec::new();
        for (agent_name, agent) in world_data.into_iter() {
            monitored_cards.push(agent);
        }

        let first_elemn = monitored_cards.clone()[0].clone();

        monitored_groups = monitored_groups.push(labeled_data_cards(
            format!("world {}", world_id),
            first_elemn,
            4,
        ));
    }

    let agent_groups_title = data_item("Agents".to_string()).size(36);

    scrollable(
        Column::new()
            .push(agent_groups_title)
            .push(agent_groups)
            .push(data_item("Protocol".to_string()).size(36))
            .push(monitored_groups)
            .spacing(16)
            .width(Length::Fill),
    )
    .into()
}

/// Renders a grid of agents cards, with a maximum amount of cards per row.
fn agent_card_grid<'a>(data: Vec<Vec<(String, String)>>, max: usize) -> Element<'a, Message> {
    let mut content = Column::new().spacing(16);
    let mut row = Row::new().spacing(16);
    let mut i = 0;
    for card in data.into_iter() {
        row = row.push(agent::agent_card(card, false));
        i += 1;
        if i == max {
            content = content.push(row);
            row = Row::new().spacing(16);
            i = 0;
        }
    }
    content = content.push(row);
    content.spacing(8).into()
}

fn mock_agent_card() -> Element<'static, Message> {
    agent::agent_card(
        vec![
            ("name".to_string(), "agent".to_string()),
            ("name".to_string(), "agent".to_string()),
            ("name".to_string(), "agent".to_string()),
            ("name".to_string(), "agent".to_string()),
        ],
        false,
    )
}

fn mock_monitor_group() -> Element<'static, Message> {
    labeled_data_cards(
        "protocol".to_string(),
        vec![
            ("name".to_string(), "agent".to_string()),
            ("name".to_string(), "agent".to_string()),
            ("name".to_string(), "agent".to_string()),
            ("name".to_string(), "agent".to_string()),
        ],
        3,
    )
}
