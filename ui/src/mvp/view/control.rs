//! Renders the main simulation controls.

use iced::Color;
use iced_aw::graphics::icons::{self, icon_to_char, ICON_FONT};

use super::*;
use crate::mvp::components::button::CustomButtonStyle;

/// Messages emitted from user interaction with agents.
#[derive(Debug, Clone)]
pub enum AgentOperations {
    Add,
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

/// Renders a gray text label in lowercase.
pub fn label_item<'a>(t: String) -> Element<'a, Message> {
    let content = t.to_lowercase();

    text(content)
        .size(16)
        .style(Color::from_rgb(0.5, 0.5, 0.5))
        .into()
}

/// Renders a column with a label and an element.
pub fn labeled<'a, T: Into<Element<'a, Message>>>(
    label: String,
    element: T,
) -> Element<'a, Message> {
    let mut content = Column::new().push(label_item(label)).push(element.into());
    content = content.spacing(8);
    content.into()
}

/// Renders a row of labeled controls, where each control has a label.
pub fn labeled_controls<'a, T: Into<Element<'a, Message>>>(
    controls: Vec<(String, T)>,
) -> Element<'a, Message> {
    let mut content = Row::new();
    for (label, control) in controls {
        content = content.push(labeled(label, control));
    }
    content.spacing(4).into()
}

/// Renders a nice blue button.
pub fn action_button<'a>(label: String) -> iced::widget::Button<'a, Message> {
    let content = text(label)
        .size(16)
        .horizontal_alignment(iced::alignment::Horizontal::Center)
        .vertical_alignment(iced::alignment::Vertical::Center)
        .style(Color::WHITE);
    let action_button_style = CustomButtonStyle::new()
        .border_radius(5.0.into())
        .background_color(Color::from_rgb8(35, 88, 226))
        .hovered()
        .border_radius(5.0.into())
        .background_color(Color::from_rgb8(88, 135, 255))
        .pressed()
        .border_radius(5.0.into())
        .background_color(Color::from_rgb8(11, 63, 197));
    button(content).style(action_button_style.as_custom())
}

/// Container that groups actions or settings with a label and a row of
/// controls.
pub fn controls_container<'a, T: Into<Element<'a, Message>>>(
    label: String,
    actions: Vec<T>,
) -> Element<'a, Message> {
    let mut content = Column::new().push(label_item(label));
    let mut row = Row::new().spacing(4);
    for action in actions {
        row = row.push(action.into());
    }
    content = content.push(row);
    content.spacing(8).into()
}

/// Renders a column with a label and a piece of data with the DAGGERSQUARE
/// font.
pub fn labeled_data<'a>(label: String, data: String) -> Element<'a, Message> {
    let mut content = Column::new()
        .push(label_item(label))
        .push(text(data).font(FONT_DAGGERSQUARE).size(20));
    content = content.spacing(8);
    content.into()
}

/// Renders a row of labeled data elements using labeled_data. Specify the
/// maximum amount of elements in the row, if the total amount of elements
/// exceeds the value, it will push a new row to the column.
pub fn labeled_data_row<'a>(
    label_data: Vec<(String, String)>,
    max_elements: usize,
) -> Element<'a, Message> {
    let mut content = Column::new();
    let mut row = Row::new().spacing(4);
    let mut i = 0;
    for (label, data) in label_data {
        row = row.push(labeled_data(label, data));
        i += 1;
        if i == max_elements {
            content = content.push(row);
            row = Row::new().spacing(4);
            i = 0;
        }
    }
    content = content.push(row);
    content.spacing(8).into()
}
