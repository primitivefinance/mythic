//! Implements the `iced::widget::Component` trait to render the config editor.

use iced::{
    widget::{button, component, text, text_input, Column, Component},
    Element, Renderer,
};
use tracing::info;

use super::config::*;

/// A dedicated component for editing a config that implements the Config trait.
pub struct ConfigEditor<C: Config> {
    config: C,
}

impl<C: Config> ConfigEditor<C> {
    pub fn new(config: C) -> Self {
        Self { config }
    }
}

impl<Message, C: Config> Component<Message, Renderer> for ConfigEditor<C> {
    type State = ();
    type Event = Event;

    fn update(&mut self, _state: &mut Self::State, event: Event) -> Option<Message> {
        match event {
            Event::FieldChanged(field_name, value) => {
                info!("Field changed: {} to {}", field_name, value);
                self.config.set_field(field_name, value);
                None
            }
            Event::NestedFieldChanged(nested_name, field_name, value) => {
                info!("Nested field changed: {} to {}", field_name, value);
                self.config.set_nested_field(nested_name, field_name, value);
                None
            }
            Event::SaveButtonPressed => {
                info!("todo: Save button pressed");
                None
            }
        }
    }

    fn view(&self, _state: &Self::State) -> iced::Element<Event, Renderer> {
        let mut column = Column::new();

        for field in self.config.fields().iter() {
            column = column.push(create_field_input(field));
        }
        for nested in self.config.nested_fields().iter() {
            column = column.push(text(nested.name.as_str()));
            for field in nested.field.fields().iter() {
                column = column.push(create_nested_field_input(nested.name.as_str(), field));
            }
        }

        column = column.push(button(text("Save")).on_press(Event::SaveButtonPressed));

        column.into()
    }
}

/// Renders a single field of a config and emits [`Event::FieldChanged`] when
/// the field is changed.
pub fn create_field_input<'a>(field: &ConfigField) -> Element<'a, Event, Renderer> {
    let mut column = Column::new();
    let label = field.label.clone();
    let value = field.value.raw().clone();

    column = column.push(text(label.as_str()));
    column = column.push(
        text_input("Enter a value...", value.as_str())
            .on_input(move |x| Event::FieldChanged(label.clone(), x)),
    );

    column.into()
}

/// Renders a field with a depth > 1 and emits [`Event::NestedFieldChanged`]
/// when the field is changed.
pub fn create_nested_field_input<'a>(
    nested_name: &str,
    field: &ConfigField,
) -> Element<'a, Event, Renderer> {
    let mut column = Column::new();
    let name = nested_name.to_string();
    let label = field.label.clone();
    let value = field.value.raw().clone();
    column = column.push(text(label.as_str()));
    column = column.push(
        text_input("Enter a value...", value.as_str())
            .on_input(move |x| Event::NestedFieldChanged(name.clone(), label.clone(), x)),
    );

    column.into()
}

#[derive(Debug, Clone)]
pub enum Event {
    FieldChanged(String, String),
    NestedFieldChanged(String, String, String),
    SaveButtonPressed,
}

impl<'a, Message, C: Config + 'a> From<ConfigEditor<C>> for Element<'a, Message, Renderer>
where
    Message: 'a,
{
    fn from(counter: ConfigEditor<C>) -> Self {
        component(counter).into()
    }
}
