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

pub fn control_panel<'a>(_realtime: bool) -> Element<'a, Message> {
    let mut content = Row::new().spacing(Sizes::Xl as u16);

    content = content.push(
        Column::new()
            .push(labeled_controls(vec![
                (
                    "Actions".to_string(),
                    action_button("Spawn".to_string().to_lowercase())
                        .padding([12, 10])
                        .on_press(Message::Simulation(control::Operation::Spawn))
                        .into(),
                ),
                ("Play".to_string(), control::play()),
                ("Pause".to_string(), control::pause()),
                ("Step".to_string(), control::step()),
                ("Stop".to_string(), control::stop()),
            ]))
            .align_items(alignment::Alignment::End),
    );

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
        .background_color(PRIMARY_COLOR.into())
        .border_radius(5.0.into());
    button(content).style(control_button_style.as_custom())
}

pub fn custom_icon_button<'a>(
    icon: icons::Icon,
    font_size: u16,
) -> iced::widget::Button<'a, Message> {
    let content = text(icon_to_char(icon))
        .font(ICON_FONT)
        .size(font_size)
        .style(Color::WHITE);
    let control_button_style = CustomButtonStyle::new()
        .background_color(Color::TRANSPARENT)
        .hovered()
        .background_color(PRIMARY_COLOR.into())
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
