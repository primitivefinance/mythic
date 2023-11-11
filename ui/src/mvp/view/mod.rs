use std::collections::{HashMap, VecDeque};

use iced::widget::checkbox;
use tracing::Span;

use super::{column, *};

pub mod control;

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

pub fn view_span() -> Span {
    tracing::info_span!("View")
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

pub fn terminal_view<'a>(logs: Vec<String>) -> Element<'a, Message> {
    let mut content = Column::new().push(Text::new("Terminal").size(28));
    content = content.push(firehose_view(logs.clone(), "main".to_string()));
    content.spacing(16).into()
}

pub fn terminal_view_multiple_firehose<'a>(
    log_containers: Vec<VecDeque<String>>,
    realtime: bool,
    state_vars: Vec<String>,
    firehose_visible: bool,
) -> Element<'a, Message> {
    let mut content = Column::new();
    let mut actions = row![].width(Length::Fill).height(Length::Shrink).spacing(8);

    actions = actions
        .push(control::play())
        .push(control::pause())
        .push(control::stop())
        .push(control::step());
    actions = actions
        .push(button(text("Spawn Worlds")).on_press(Message::Simulation(control::Operation::Spawn)))
        .push(
            button(text("Spawn Agent")).on_press(Message::Simulation(control::Operation::Agent(
                control::AgentOperations::Add,
            ))),
        )
        .push(button(text("Log debug trace")).on_press(Message::Data(Data::LogTrace)));

    actions = actions
        .push(checkbox("realtime", realtime, |_| {
            Message::Settings(Settings::ToggleRealtime)
        }))
        .push(checkbox("firehose visible", !firehose_visible, |_| {
            Message::Settings(Settings::ToggleFirehoseVisibility)
        }));

    let mut watched = column![text("watching:").size(16)]
        .width(Length::Fill)
        .spacing(4)
        .align_items(iced::Alignment::End);
    for state_var in state_vars {
        watched = watched.push(Text::new(state_var).size(18));
    }
    actions = actions.push(watched);

    content = content
        .push(
            container(actions)
                .padding(8)
                .style(MenuContainerTheme::theme())
                .width(Length::Fill),
        )
        .push(multiple_firehose(log_containers.clone()));
    content
        .spacing(16)
        .padding(16)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

pub fn firehose_view<'a>(logs: Vec<String>, label: String) -> Element<'a, Message> {
    let mut content = Column::new().push(Text::new(label).size(24));

    let firehose = logs.iter().rev().fold(column![].spacing(2), |column, log| {
        column.push(firehose_log(log.clone()))
    });
    let firehose_content = container(scrollable(firehose))
        .style(FirehoseContainer::theme())
        .height(Length::Fixed(400.0))
        .width(Length::Fixed(300.0))
        .padding(4);

    content = content.push(firehose_content);
    content.spacing(16).padding(16).into()
}

/// warning! Adding logging in here can lead to infinite loops.
pub fn multiple_firehose<'a>(log_containers: Vec<VecDeque<String>>) -> Element<'a, Message> {
    let mut firehose_column = column![];
    let mut firehose_row = row![].spacing(4).width(Length::Fill);
    let mut count = 0;

    for firehose_logs in log_containers {
        let mut label = format!("firehose_{}", count);
        if count == 0 {
            label = format!("main");
        }
        if count == 1 {
            label = format!("user (you)");
        }

        firehose_row = firehose_row.push(firehose_view(firehose_logs.clone().into(), label));
        count += 1;

        // todo: this spacing should be based on current window length or something...
        if count % 4 == 0 {
            firehose_column = firehose_column.push(firehose_row);
            firehose_row = row![].spacing(4).width(Length::Fill);
        }
    }

    // Push the last row if it has any firehoses
    if count % 2 != 0 {
        firehose_column = firehose_column.push(firehose_row);
    }

    firehose_column
        .spacing(4)
        .height(Length::Fill)
        .width(Length::Fill)
        .into()
}

pub fn firehose_log<'a>(log: String) -> Element<'a, Message> {
    let firehose_element = text(log)
        .size(12)
        .vertical_alignment(alignment::Vertical::Center)
        .horizontal_alignment(alignment::Horizontal::Left);

    container(firehose_element)
        .style(FirehoseTrace::theme())
        .width(Length::Fill)
        .padding(4)
        .into()
}

pub struct MenuContainerTheme;

impl container::StyleSheet for MenuContainerTheme {
    type Style = iced::Theme;

    fn appearance(&self, _: &<Self as container::StyleSheet>::Style) -> container::Appearance {
        container::Appearance {
            background: Some(iced::Background::Color(PANEL)),
            border_radius: 2.0.into(),
            border_width: 1.0,
            border_color: BORDER_COLOR,
            ..Default::default()
        }
    }
}

impl MenuContainerTheme {
    pub fn theme() -> iced::theme::Container {
        iced::theme::Container::Custom(Box::from(MenuContainerTheme))
    }
}

pub struct BackgroundContainerTheme;

impl container::StyleSheet for BackgroundContainerTheme {
    type Style = iced::Theme;

    fn appearance(&self, _: &<Self as container::StyleSheet>::Style) -> container::Appearance {
        container::Appearance {
            background: Some(iced::Background::Color(BACKGROUND)),
            ..Default::default()
        }
    }
}

impl BackgroundContainerTheme {
    pub fn theme() -> iced::theme::Container {
        iced::theme::Container::Custom(Box::from(BackgroundContainerTheme))
    }
}

pub struct FirehoseContainer;

impl container::StyleSheet for FirehoseContainer {
    type Style = iced::Theme;

    fn appearance(&self, _: &<Self as container::StyleSheet>::Style) -> container::Appearance {
        container::Appearance {
            background: Some(iced::Background::Color(PANEL)),
            border_radius: 2.0.into(),
            border_width: 1.0,
            border_color: BORDER_COLOR,
            ..Default::default()
        }
    }
}

impl FirehoseContainer {
    pub fn theme() -> iced::theme::Container {
        iced::theme::Container::Custom(Box::from(FirehoseContainer))
    }
}

pub struct FirehoseTrace;

impl container::StyleSheet for FirehoseTrace {
    type Style = iced::Theme;

    fn appearance(&self, _: &<Self as container::StyleSheet>::Style) -> container::Appearance {
        container::Appearance {
            background: Some(iced::Background::Color(MODAL)),
            border_radius: 4.0.into(),
            border_width: 1.0,
            border_color: BORDER_COLOR,
            ..Default::default()
        }
    }
}

impl FirehoseTrace {
    pub fn theme() -> iced::theme::Container {
        iced::theme::Container::Custom(Box::from(FirehoseTrace))
    }
}
