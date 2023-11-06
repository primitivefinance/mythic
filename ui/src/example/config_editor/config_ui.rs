//! Implements the `iced::widget::Component` trait to render the config editor.
//!
//! ## ConfigEditor
//! Handles the logic for editing the config's intermediary storage, rendering the UI components, and saving the intermediary storage to the config.
//!
//! ## ConfigInput
//! Renders a single field of a config and emits [`Event::FieldChanged`] when the field is changed.
//!
//! ## The Flow
//! - The ConfigEditor is created with a config using `create_config_editor()`, which can be pushed to an iced element.
//! - The ConfigEditor renders a [`ConfigInput`] for each field in the config.
//! - The [`ConfigInput`] emits [`Event::FieldChanged`] when the field is changed.
//! - The ConfigEditor handles the [`Event::FieldChanged`] and updates the intermediary storage `store`.
//! - The ConfigEditor renders a save button that emits [`Event::SaveButtonPressed`] when pressed.
//! - The ConfigEditor saves the `store` to the config when the save button is pressed.
//! - The ConfigEditor renders a debug button that emits [`Event::DebugConfig`] when pressed.
//! - The ConfigEditor prints the config to the console when the debug button is pressed.
//! - Input validation and saving to the config is handled by the config itself, in `config.rs`.
//!
//! ## The Bugs
//! - Fields get switched after the first update.
//! - Fields don't give feedback if they have invalid inputs.

use iced::{
    widget::{button, component, row, text, text_input, Column, Component},
    Element, Renderer,
};
use std::collections::HashMap;
use tracing::info;

use super::config::*;

/// A dedicated component for editing a config that implements the Config trait.
/// The ConfigEditor makes use of a "store", which is an important intermediary storage
/// for the config values.
/// Without storing as Strings, they would get stored in the native config types.
/// This can lead to the text inputs being very clunky to use because they only accept the native type.
/// - store is a HashMap of field names and values.
/// - nested_store is a HashMap of nested field names and values.
pub struct ConfigEditor<C: Config> {
    config: C,
    store: HashMap<String, String>,
    nested_store: HashMap<String, HashMap<String, String>>,
}

impl<C: Config> ConfigEditor<C> {
    pub fn new(config: C) -> Self {
        let mut store = HashMap::new();

        // Populate the store with the current values of the config.
        for field in config.fields().iter() {
            let value = field.get_value().clone();
            store.insert(field.label.clone(), value);
        }

        // Populate the store with all the nested fields of the config.
        let mut nested_store = HashMap::new();
        for nested in config.nested_fields().iter() {
            let mut nested_field_store = HashMap::new();
            for field in nested.field.fields().iter() {
                let value = field.get_value().clone();
                nested_field_store.insert(field.label.clone(), value);
            }
            nested_store.insert(nested.name.clone(), nested_field_store);
        }

        Self {
            config,
            store,
            nested_store,
        }
    }

    pub fn set_field(&mut self, field_name: String, value: String) {
        let current_value = self.store.get(&field_name).unwrap();
        info!(
            "Field changed: {} from {} to {}",
            field_name, current_value, value
        );
        // Edit the store's field value.
        self.store.insert(field_name.clone(), value.clone());
    }

    pub fn set_nested_field(&mut self, nested_name: String, field_name: String, value: String) {
        let current_value = self
            .nested_store
            .get(&nested_name)
            .unwrap()
            .get(&field_name)
            .unwrap();
        info!(
            "Nested field changed: {} from {} to {}",
            field_name, current_value, value
        );
        // Edit the store's nested field values.
        self.nested_store
            .get_mut(&nested_name)
            .unwrap()
            .insert(field_name.clone(), value.clone());
    }

    pub fn save_config(&mut self) {
        // Writes the store's values to the config.
        info!("Saving config...");

        // For each field in the store, set the config's field to the store's value.
        for (field_name, value) in self.store.iter() {
            let res = self.config.set_field(field_name.clone(), value.clone());
            if let Err(e) = res {
                info!(
                    "Error setting field '{}' to value '{}': {}",
                    field_name, value, e
                )
            }
        }

        // For each nested field in the store, set the config's nested field to the store's value.
        for (nested_name, nested_field_store) in self.nested_store.iter() {
            for (field_name, value) in nested_field_store.iter() {
                let res = self.config.set_nested_field(
                    nested_name.clone(),
                    field_name.clone(),
                    value.clone(),
                );

                if let Err(e) = res {
                    info!(
                        "Error setting nested field '{}.{}' to value '{}': {}",
                        nested_name, field_name, value, e
                    )
                }
            }
        }

        info!("Config saving complete");
    }
}

#[derive(Debug, Clone)]
pub enum Event {
    FieldChanged(String, String),
    NestedFieldChanged(String, String, String),
    SaveButtonPressed,
    DebugConfig,
}

impl<Message, C: Config> Component<Message, Renderer> for ConfigEditor<C> {
    type State = ();
    type Event = Event;

    fn update(&mut self, _state: &mut Self::State, event: Event) -> Option<Message> {
        match event {
            Event::FieldChanged(field_name, value) => {
                self.set_field(field_name, value);
            }
            Event::NestedFieldChanged(nested_name, field_name, value) => {
                self.set_nested_field(nested_name, field_name, value);
            }
            Event::SaveButtonPressed => {
                self.save_config();
            }
            Event::DebugConfig => {
                info!("Config: {:?}", self.config);
            }
        }

        None
    }

    fn view(&self, _state: &Self::State) -> iced::Element<Event, Renderer> {
        let mut column = Column::new();

        // Iterate through each field in the store and render it by passing it to the config_input component.
        for (field_name, value) in self.store.iter() {
            column = column.push(create_field_input(field_name.clone(), value.clone()));
        }

        // Iterate through each nested field and render it by passing it to the config_input component.
        for (nested_name, nested_field_store) in self.nested_store.iter() {
            let name = nested_name.clone();
            column = column.push(text(name.as_str()));

            for (field_name, value) in nested_field_store.iter() {
                column = column.push(create_nested_field_input(
                    name.clone(),
                    field_name.clone(),
                    value.clone(),
                ));
            }
        }

        column = column.push(button(text("Save")).on_press(Event::SaveButtonPressed));
        column = column.push(button(text("Debug")).on_press(Event::DebugConfig));

        column.into()
    }
}

pub fn create_config_editor<'a, Message, C: Config + 'a>(
    config: C,
) -> Element<'a, Message, Renderer>
where
    Message: 'a,
{
    ConfigEditor::new(config).into()
}

/// Renders a single field of a config and emits [`Event::FieldChanged`] when
/// the field is changed.
pub fn create_field_input<'a>(
    field_name: String,
    field_value: String,
) -> Element<'a, Event, Renderer> {
    let mut column = Column::new();

    column = column.push(text(field_name.as_str()));
    column = column.push(create_config_input(Some(field_value), move |x| {
        Event::FieldChanged(field_name.clone(), x.unwrap_or_default())
    }));

    column.into()
}

/// Renders a field with a depth > 1 and emits [`Event::NestedFieldChanged`]
/// when the field is changed.
pub fn create_nested_field_input<'a>(
    nested_field_label: String,
    field_label: String,
    field_value: String,
) -> Element<'a, Event, Renderer> {
    let mut column = Column::new();

    column = column.push(text(field_label.as_str()));
    column = column.push(create_config_input(Some(field_value), move |x| {
        Event::NestedFieldChanged(
            nested_field_label.clone(),
            field_label.clone(),
            x.unwrap_or_default(),
        )
    }));

    column.into()
}

impl<'a, Message, C: Config + 'a> From<ConfigEditor<C>> for Element<'a, Message, Renderer>
where
    Message: 'a,
{
    fn from(counter: ConfigEditor<C>) -> Self {
        component(counter).into()
    }
}

/// Individual component for managing the input
pub struct ConfigInput<Message> {
    value: Option<String>,
    on_change: Box<dyn Fn(Option<String>) -> Message>,
}

pub fn create_config_input<Message>(
    value: Option<String>,
    on_change: impl Fn(Option<String>) -> Message + 'static,
) -> ConfigInput<Message> {
    ConfigInput::new(value, on_change)
}

#[derive(Debug, Clone)]
pub enum ConfigInputEvent {
    InputChanged(String),
}

impl<Message> ConfigInput<Message> {
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

impl<Message> Component<Message, Renderer> for ConfigInput<Message> {
    type State = ();
    type Event = ConfigInputEvent;

    fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<Message> {
        match event {
            Self::Event::InputChanged(value) => {
                self.value = Some(value.clone());

                if value.is_empty() {
                    Some((self.on_change)(None))
                } else {
                    info!("Input changed: {}", value);
                    value.parse().ok().map(Some).map(self.on_change.as_ref())
                }
            }
        }
    }

    fn view(&self, _state: &Self::State) -> iced::Element<Self::Event, Renderer> {
        row![text_input(
            "Type a value...",
            self.value
                .as_ref()
                .map(String::to_string)
                .as_deref()
                .unwrap_or(""),
        )
        .on_input(ConfigInputEvent::InputChanged)
        .padding(10)]
        .spacing(10)
        .into()
    }
}

impl<'a, Event> From<ConfigInput<Event>> for Element<'a, Event, Renderer>
where
    Event: 'a,
{
    fn from(config_input: ConfigInput<Event>) -> Self {
        component(config_input).into()
    }
}
