//! # Example component
//! This is a "component" that interacts with the Counter.sol smart contract.
//! A component is just a siloed piece of the UI that has its own state.
//!
//! Adding this component to the UI is as simple as pushing it to the container that is rendered in the app's view function.

use iced::{
    alignment::{self, Alignment},
    widget::{button, component, row, text, Component},
    Element, Length, Renderer,
};

use std::sync::Arc;

/// Type alias for the on_change function that can be passed to the counter component.
/// This enables the application to react to changes in the counter's state.
type HandlerFn<Msg> = Arc<Box<dyn Fn(Option<u32>) -> Msg + Send + Sync + 'static>>;

/// This is the "model" for the counter component.
/// It holds the state of the component and a function handler for updating the model.
#[derive(Clone)]
pub struct Counter<Msg> {
    value: Option<u32>,
    on_change: HandlerFn<Msg>,
}

/// - Msg is a generic type for the application Message that is transmitted from the on_change function.
impl<Msg> Counter<Msg> {
    pub fn new(
        value: Option<u32>,
        on_change: impl Fn(Option<u32>) -> Msg + Send + Sync + 'static,
    ) -> Self {
        Self {
            value,
            on_change: Arc::new(Box::new(on_change)),
        }
    }
}

/// Events that occur in the component.
#[derive(Debug, Clone)]
pub enum Event {
    Increment,
    Decrement,
    InputChanged(String),
}

/// Implementation of the actual component for the application.
/// update - Handles the model updates.
/// view - Handles the model view.
impl<Msg> Component<Msg, Renderer> for Counter<Msg> {
    type State = ();
    type Event = Event;

    fn update(&mut self, _state: &mut Self::State, event: Event) -> Option<Msg> {
        match event {
            Event::Increment => Some((self.on_change)(Some(
                self.value.unwrap_or_default().saturating_add(1),
            ))),
            Event::Decrement => Some((self.on_change)(Some(
                self.value.unwrap_or_default().saturating_sub(1),
            ))),
            Event::InputChanged(value) => {
                if value.is_empty() {
                    Some((self.on_change)(None))
                } else {
                    value.parse().ok().map(Some).map(self.on_change.as_ref())
                }
            }
        }
    }

    fn view(&self, _state: &Self::State) -> Element<Event, Renderer> {
        let button = |label, on_press| {
            button(
                text(label)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .vertical_alignment(alignment::Vertical::Center),
            )
            .width(40)
            .height(40)
            .on_press(on_press)
        };

        row![
            button("-", Event::Decrement),
            button("+", Event::Increment),
            button("refresh", Event::InputChanged("1".to_string()))
        ]
        .align_items(Alignment::Center)
        .spacing(10)
        .into()
    }
}

/// Converts the component into an iced Element, which can be pushed to a content container in the UI.
impl<'a, Msg> From<Counter<Msg>> for Element<'a, Msg, Renderer>
where
    Msg: 'a,
{
    fn from(counter: Counter<Msg>) -> Self {
        component(counter).into()
    }
}
