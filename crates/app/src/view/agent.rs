//! View components for Agents.

use super::*;

pub fn agent_card<'a>(agent_data: Vec<(String, String)>, actions: bool) -> Element<'a, Message> {
    let mut content = Column::new().spacing(16);

    // Try finding the agent name, else use a default value
    let agent_name = agent_data
        .iter()
        .find(|(name, _)| name == "name")
        .map(|(_, value)| value.clone())
        .unwrap_or("default".to_string());

    // Remove the name from the agent data so its not rendered in the content
    let filtered_data = agent_data
        .clone()
        .into_iter()
        .filter(|(name, _)| name != "name")
        .collect::<Vec<(String, String)>>();

    // Uppercase first letter of agent name and replace "_" with a space.
    let agent_name = agent_name
        .chars()
        .enumerate()
        .map(|(i, c)| match i {
            0 => c.to_uppercase().to_string(),
            _ => c.to_string(),
        })
        .collect::<String>()
        .replace("_", " ");

    content = content
        .push(agent_header(agent_name.clone()))
        .push(agent_content(filtered_data.clone()));

    if actions {
        content = content.push(agent_actions());
    }

    Card::new(
        container(content)
            .padding(Sizes::Md as u16)
            .max_height(300.0),
    )
    .into()
}

/// Renders the agent icon, name, and a settings gear icon button in a max width
/// column.
pub fn agent_header<'a>(agent_name: String) -> Element<'a, Message> {
    Column::new().push(h2(agent_name)).into()
}

/// Renders the agent's labeled data items.
pub fn agent_content<'a>(agent_data: Vec<(String, String)>) -> Element<'a, Message> {
    labeled_data_container("Data".to_string(), agent_data, 2)
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
