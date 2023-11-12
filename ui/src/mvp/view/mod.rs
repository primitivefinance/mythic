use std::collections::{HashMap, VecDeque};

use iced::widget::{checkbox, Column, Row};

use self::{
    control::control_panel,
    event::{mock_event_groups, EventFeed},
    feed::Feed,
    monitor::{labeled_data_card, labeled_data_cards},
};
use super::{
    column,
    components::{containers::*, *},
    tracer::AppEventLog,
    *,
};

pub mod agent;
pub mod control;
pub mod event;
pub mod feed;
pub mod monitor;

/// Messages emitted from user interaction with the settings.
#[derive(Debug, Clone)]
pub enum Settings {
    ToggleRealtime,
    ToggleFirehoseVisibility,
}

/// Messages emitted from user interaction with data components.
#[derive(Debug, Clone)]
pub enum Data {
    // for debugging...
    LogTrace,
    // todo: this needs a refactor
    UpdateWatchedValue(HashMap<String, String>),
}

/// Root message for the Terminal component.
#[derive(Debug, Clone)]
pub enum Message {
    Empty,
    Simulation(control::Operation),
    Settings(Settings),
    Data(Data),
}

pub fn app_layout<'a, T: Into<Element<'a, Message>>>(content: T) -> Element<'a, Message> {
    container(row![
        Space::with_width(Length::FillPortion(1)),
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

pub fn terminal_view_multiple_firehose<'a>(
    data: VecDeque<AppEventLog>,
    realtime: bool,
    state_vars: Vec<String>,
    firehose_visible: bool,
) -> Element<'a, Message> {
    let mut labeled_data = vec![];
    for state_var in state_vars.clone() {
        let mut split = state_var.split(":");
        let label = split.next().unwrap().to_string();
        let data = split.next().unwrap().to_string();
        labeled_data.push((label, data));
    }

    let mut actions = control_panel(labeled_data.clone(), realtime, firehose_visible);
    let agents = mock_agent_card();
    let monitored = labeled_data_card("monitored".to_string(), "data".to_string(), 200);
    let monitor_group = mock_monitor_group();

    let eventing = EventFeed {
        events: mock_event_groups(),
    }
    .view();

    let mut feed = Feed::new(20);
    feed.vec_to_bucketed_logs(data);

    let feed_view: Element<'a, Message> = feed.view("default").into();

    let content_row = Row::new()
        .push(
            Column::new()
                .push(agents)
                .push(monitored)
                .push(monitor_group)
                .width(Length::FillPortion(2)),
        )
        .push(
            Column::new()
                .push(feed_view)
                .push(eventing)
                .width(Length::FillPortion(2)),
        )
        .width(Length::Fill);

    Column::new()
        .push(
            container(actions)
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

fn mock_agent_card() -> Element<'static, Message> {
    agent::agent_card(vec![
        ("name".to_string(), "agent".to_string()),
        ("name".to_string(), "agent".to_string()),
        ("name".to_string(), "agent".to_string()),
        ("name".to_string(), "agent".to_string()),
    ])
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
