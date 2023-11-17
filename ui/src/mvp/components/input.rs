use iced::widget::{component, text_input, Component};

use super::*;

pub fn create_input_component<Message>(
    value: Option<String>,
    on_change: impl Fn(Option<String>) -> Message + 'static,
) -> InputComponent<Message> {
    InputComponent::new(value, on_change)
}

/// Individual component for managing inputs with string values.
/// todo: better error handling and tracing
pub struct InputComponent<Message> {
    /// Current value in the input.
    value: Option<String>,
    /// Callback for when the input changes.
    on_change: Box<dyn Fn(Option<String>) -> Message>,
}

#[derive(Debug, Clone)]
pub enum StringInputComponentEvent {
    InputChanged(String),
}

impl<Message> InputComponent<Message> {
    pub fn new(
        value: Option<String>,
        on_change: impl Fn(Option<String>) -> Message + 'static,
    ) -> Self {
        Self {
            value,
            on_change: Box::new(on_change),
        }
    }
}

impl<Message> Component<Message, Renderer> for InputComponent<Message> {
    type State = ();
    type Event = StringInputComponentEvent;

    fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<Message> {
        match event {
            Self::Event::InputChanged(value) => {
                self.value = Some(value.clone());

                if value.is_empty() {
                    Some((self.on_change)(None))
                } else {
                    tracing::trace!("Input changed: {}", value);
                    let parsed_value = value.parse();
                    match parsed_value {
                        Ok(parsed_value) => Some((self.on_change)(Some(parsed_value))),
                        Err(e) => {
                            tracing::warn!("Error parsing input: {:?}", e);
                            None
                        }
                    }
                }
            }
        }
    }

    fn view(&self, _state: &Self::State) -> Element<Self::Event, Renderer> {
        let input = text_input(
            "Type a value...",
            self.value
                .as_ref()
                .map(String::to_string)
                .as_deref()
                .unwrap_or(""),
        )
        .on_input(StringInputComponentEvent::InputChanged);

        input.into()
    }
}

impl<'a, Event> From<InputComponent<Event>> for Element<'a, Event, Renderer>
where
    Event: 'a,
{
    fn from(config_input: InputComponent<Event>) -> Self {
        component(config_input).into()
    }
}
