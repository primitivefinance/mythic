//! Renders the main simulation controls.

use iced::Color;
use iced_aw::graphics::icons::{self, icon_to_char, ICON_FONT};

use super::{components::button::*, *};

/// Messages emitted from user interaction with agents.
#[derive(Debug, Clone)]
pub enum AgentOperations {
    Add,
    Settings(String),
}

/// Messages emitted from user interaction with the simulation.
#[derive(Debug, Clone)]
pub enum Operation {
    Spawn,
    Continue,
    Stop,
    Pause,
    Step,
    Agent(AgentOperations),
}

pub fn control_panel<'a>(
    data: Vec<(String, String)>,
    realtime: bool,
    firehose_visible: bool,
) -> Element<'a, Message> {
    let mut content = Row::new().spacing(16).height(Length::Shrink);
    content = content.push(labeled_controls(vec![
        ("play".to_string(), control::play()),
        ("pause".to_string(), control::pause()),
        ("step".to_string(), control::step()),
        ("stop".to_string(), control::stop()),
    ]));

    content = content.push(controls_container(
        "settings".to_string(),
        vec![
            checkbox("realtime", realtime, |_| {
                Message::Settings(Settings::ToggleRealtime)
            }),
            checkbox("firehose visible", !firehose_visible, |_| {
                Message::Settings(Settings::ToggleFirehoseVisibility)
            }),
        ],
    ));

    content = content.push(controls_container(
        "actions".to_string(),
        vec![
            action_button("Spawn".to_string().to_lowercase())
                .on_press(Message::Simulation(control::Operation::Spawn)),
            action_button("Spawn Agent".to_string().to_lowercase()).on_press(Message::Simulation(
                control::Operation::Agent(control::AgentOperations::Add),
            )),
            action_button("Log debug trace".to_string().to_lowercase())
                .on_press(Message::Data(Data::LogTrace)),
        ],
    ));

    content = content.push(labeled_data_container("watched".to_string(), data, 3));

    content.into()
}

pub fn control_button<'a>(icon: icons::Icon) -> iced::widget::Button<'a, Message> {
    let content = text(icon_to_char(icon))
        .font(ICON_FONT)
        .size(28)
        .style(Color::WHITE);
    let control_button_style = CustomButtonStyle::new()
        .background_color(Color::TRANSPARENT)
        .hovered()
        .background_color(Color::from_rgba8(40, 40, 40, 0.5))
        .border_radius(5.0.into());
    button(content).style(control_button_style.as_custom())
}

pub fn play<'a>() -> Element<'a, Message> {
    control_button(icons::Icon::PlayFill)
        .on_press(Message::Simulation(Operation::Continue))
        .into()
}

pub fn stop<'a>() -> Element<'a, Message> {
    control_button(icons::Icon::StopFill)
        .on_press(Message::Simulation(Operation::Stop))
        .into()
}

pub fn pause<'a>() -> Element<'a, Message> {
    control_button(icons::Icon::PauseFill)
        .on_press(Message::Simulation(Operation::Pause))
        .into()
}

pub fn step<'a>() -> Element<'a, Message> {
    control_button(icons::Icon::ForwardFill)
        .on_press(Message::Simulation(Operation::Step))
        .into()
}
