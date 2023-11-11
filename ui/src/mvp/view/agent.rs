//! View components for Agents.
use iced::Color;
use iced_aw::{
    graphics::icons::{self, icon_to_char},
    ICON_FONT,
};

use super::{control::control_button, *};

pub fn agent_card<'a>(agent_data: Vec<(String, String)>) -> Element<'a, Message> {
    let mut content = Column::new().spacing(16);

    // Agent name and settings button
    let agent_name = agent_data
        .iter()
        .find(|(k, _)| k == "name")
        .unwrap()
        .1
        .clone();

    content = content
        .push(agent_header(agent_name.clone()))
        .push(agent_content(agent_data.clone()))
        .push(agent_actions());

    container(content)
        .style(MenuContainerTheme::theme())
        .padding(16)
        .into()
}

/// Renders the agent icon, name, and a settings gear icon button in a max width
/// column.
pub fn agent_header<'a>(agent_name: String) -> Element<'a, Message> {
    let settings_button = control_button(icons::Icon::Gear).on_press(Message::Simulation(
        control::Operation::Agent(control::AgentOperations::Settings(agent_name.clone())),
    ));

    let agent_icon = text(icon_to_char(icons::Icon::Cpu))
        .font(ICON_FONT)
        .size(32)
        .style(Color::WHITE);

    let header_elements: Vec<Element<'_, Message>> = vec![
        agent_icon.clone().into(),
        text(agent_name).font(FONT_DAGGERSQUARE).size(24).into(),
        settings_button.into(),
    ];

    let header = space_between_row(header_elements);

    Column::new().push(header).max_width(350.0).into()
}

/// Renders the agent's labeled data items.
pub fn agent_content<'a>(agent_data: Vec<(String, String)>) -> Element<'a, Message> {
    labeled_data_container("agent_data".to_string(), agent_data, 3)
}

/// Renders the agent's action buttons.
pub fn agent_actions<'a>() -> Element<'a, Message> {
    let control_buttons = vec![
        components::action_button("Action 1".to_string().to_lowercase()).on_press(
            Message::Simulation(control::Operation::Agent(control::AgentOperations::Add)),
        ),
        components::action_button("Action 2".to_string().to_lowercase()).on_press(
            Message::Simulation(control::Operation::Agent(control::AgentOperations::Add)),
        ),
    ];
    controls_container("agent_controls".to_string(), control_buttons)
}
