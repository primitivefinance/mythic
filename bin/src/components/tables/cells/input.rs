//! An editable table cell that can accept any text input from the user.

use super::*;

pub fn cell_input<'a, Message>(
    value: Option<String>,
    on_change: impl Fn(Option<String>) -> Message + 'static,
    on_submit: impl Fn(Option<String>) -> Message + 'static,
    placeholder: Option<String>,
    internal_padding: Option<Sizes>,
) -> CustomInput<Message>
where
    Message: 'a + Default,
{
    let mut input = custom_input(value, on_change, on_submit);

    if let Some(placeholder) = placeholder {
        input = input.placeholder(placeholder);
    }

    if let Some(padding) = internal_padding {
        input = input.padding(padding);
    }

    input
}

// todo: move this to input and replace current input.
pub fn custom_input<Message>(
    value: Option<String>,
    on_change: impl Fn(Option<String>) -> Message + 'static,
    on_submit: impl Fn(Option<String>) -> Message + 'static,
) -> CustomInput<Message>
where
    Message: Default,
{
    CustomInput::new(value, on_change, on_submit)
}

// todo: make this just our general input component
pub struct CustomInput<Message>
where
    Message: Default,
{
    /// Current value in the input.
    value: Option<String>,
    /// Callback for when the input changes.
    on_change: Box<dyn Fn(Option<String>) -> Message>,
    /// Callback for when the input is submitted.
    on_submit: Box<dyn Fn(Option<String>) -> Message>,
    /// Placeholder text
    placeholder: Option<String>,
    /// Padding
    padding: Sizes,
}

#[derive(Debug, Clone, Default)]
pub enum Event {
    OnChange(String),
    OnSubmit,
    Debug(String),
    #[default]
    Empty,
}

impl<Message> Default for CustomInput<Message>
where
    Message: Default,
{
    fn default() -> Self {
        Self {
            value: None,
            on_change: Box::new(|_| Message::default()),
            on_submit: Box::new(|_| Message::default()),
            placeholder: None,
            padding: Sizes::Zero,
        }
    }
}

impl<Message> CustomInput<Message>
where
    Message: Default,
{
    pub fn new(
        value: Option<String>,
        on_change: impl Fn(Option<String>) -> Message + 'static,
        on_submit: impl Fn(Option<String>) -> Message + 'static,
    ) -> Self {
        Self {
            value,
            on_change: Box::new(on_change),
            on_submit: Box::new(on_submit),
            ..Default::default()
        }
    }

    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    pub fn padding(mut self, padding: Sizes) -> Self {
        self.padding = padding;
        self
    }
}

impl<Message> Component<Message, Renderer> for CustomInput<Message>
where
    Message: Default,
{
    type State = ();
    type Event = Event;

    fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<Message> {
        match event {
            Self::Event::Empty => None,
            Self::Event::OnChange(value) => {
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
            Self::Event::OnSubmit => {
                let value = self.value.clone();
                let value = match value {
                    Some(value) => value,
                    None => {
                        tracing::warn!("No value to submit.");
                        return None;
                    }
                };

                if value.is_empty() {
                    Some((self.on_submit)(None))
                } else {
                    tracing::trace!("Input submitted: {}", value);
                    let parsed_value = value.parse();
                    match parsed_value {
                        Ok(parsed_value) => Some((self.on_submit)(Some(parsed_value))),
                        Err(e) => {
                            tracing::warn!("Error parsing input: {:?}", e);
                            None
                        }
                    }
                }
            }
            Self::Event::Debug(value) => {
                tracing::debug!("Debug: {}", value);
                None
            }
        }
    }

    fn view(&self, _state: &Self::State) -> Element<Self::Event, Renderer> {
        let input = text_input(
            self.placeholder.as_deref().unwrap_or("Type a value..."),
            self.value
                .as_ref()
                .map(String::to_string)
                .as_deref()
                .unwrap_or_default(),
        )
        .on_input(Event::OnChange)
        .on_submit(Event::OnSubmit)
        .padding(self.padding as u16);

        input.into()
    }
}

impl<'a, Event> From<CustomInput<Event>> for Element<'a, Event, Renderer>
where
    Event: 'a + Default,
{
    fn from(v: CustomInput<Event>) -> Self {
        component(v)
    }
}
