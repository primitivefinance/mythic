//! View component for rendering events in different styled displays.

use iced::Color;

use super::*;

type UserId = u64;
type AgentId = u64;
type WorldId = u64;

#[derive(Debug)]
pub enum EventType {
    Info,
    Warning,
    Error,
}

#[derive(Debug)]
pub struct Event {
    pub event_type: EventType,
    pub timestamp: std::time::SystemTime,
}

#[derive(Debug)]
pub enum EventGroup {
    System(Vec<Event>),
    User(UserId, Vec<Event>),
    Agent(AgentId, Vec<Event>),
    World(WorldId, Vec<Event>),
}

#[derive(Debug)]
pub struct EventFeed {
    pub events: Vec<EventGroup>,
}

impl EventFeed {
    pub fn new() -> Self {
        Self { events: vec![] }
    }

    pub fn view<'a>(&self) -> Element<'a, Message> {
        let mut content = Column::new();
        for event in &self.events {
            match event {
                EventGroup::System(events) => {
                    for event in events {
                        content = content.push(event_view(format!("{:?}", event.event_type)));
                    }
                }
                EventGroup::User(user_id, events) => {
                    for event in events {
                        content = content.push(event_view(format!(
                            "User {}: {:?}",
                            user_id, event.event_type
                        )));
                    }
                }
                EventGroup::Agent(agent_id, events) => {
                    for event in events {
                        content = content.push(event_view(format!(
                            "Agent {}: {:?}",
                            agent_id, event.event_type
                        )));
                    }
                }
                EventGroup::World(world_id, events) => {
                    for event in events {
                        content = content.push(event_view(format!(
                            "World {}: {:?}",
                            world_id, event.event_type
                        )));
                    }
                }
            }
        }
        content.into()
    }

    pub fn add_event(&mut self, event: EventGroup) {
        self.events.push(event);
    }

    pub fn clear(&mut self) {
        self.events.clear();
    }
}

pub fn event_view<'a>(t: String) -> Element<'a, Message> {
    let mut content = Column::new().push(event_item(t));
    content = content.spacing(8);
    content.into()
}

/// Renders a line of an event in a table.
pub fn event_item<'a>(t: String) -> Text<'a> {
    text(t).size(16).style(Color::from_rgb(0.5, 0.5, 0.5))
}

pub fn mock_event_groups() -> Vec<EventGroup> {
    vec![
        EventGroup::System(vec![
            Event {
                event_type: EventType::Info,
                timestamp: std::time::SystemTime::now(),
            },
            Event {
                event_type: EventType::Warning,
                timestamp: std::time::SystemTime::now(),
            },
            Event {
                event_type: EventType::Error,
                timestamp: std::time::SystemTime::now(),
            },
        ]),
        EventGroup::User(
            1,
            vec![
                Event {
                    event_type: EventType::Info,
                    timestamp: std::time::SystemTime::now(),
                },
                Event {
                    event_type: EventType::Warning,
                    timestamp: std::time::SystemTime::now(),
                },
                Event {
                    event_type: EventType::Error,
                    timestamp: std::time::SystemTime::now(),
                },
            ],
        ),
        EventGroup::Agent(
            1,
            vec![
                Event {
                    event_type: EventType::Info,
                    timestamp: std::time::SystemTime::now(),
                },
                Event {
                    event_type: EventType::Warning,
                    timestamp: std::time::SystemTime::now(),
                },
                Event {
                    event_type: EventType::Error,
                    timestamp: std::time::SystemTime::now(),
                },
            ],
        ),
        EventGroup::World(
            1,
            vec![
                Event {
                    event_type: EventType::Info,
                    timestamp: std::time::SystemTime::now(),
                },
                Event {
                    event_type: EventType::Warning,
                    timestamp: std::time::SystemTime::now(),
                },
                Event {
                    event_type: EventType::Error,
                    timestamp: std::time::SystemTime::now(),
                },
            ],
        ),
    ]
}
