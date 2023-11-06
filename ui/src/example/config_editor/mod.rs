//! Renders a UI for editing a config.
//!
//! ## ConfigEditor
//! Handles the logic for editing the config's intermediary storage, rendering
//! the UI components, and saving the intermediary storage to the config.
//!
//! ## ConfigInput
//! Renders a single field of a config and emits [`EditorEvent::FieldChanged`]
//! when the field is changed.
//!
//! ## The Flow
//! - Config is instantiated by having a config type that implements the Config
//!   trait converted into a ConfigStore.
//! - Updating the config will edit the store
//! - Saving the config will call back to the app with the store, and the app
//!   will convert the store back into a config before saving it.
//! -
//! ## The Bugs
//! - Fields get switched after the first update.
//! - Fields don't give feedback if they have invalid inputs.
//! - Converting the store back into a config is a little jank because of data
//!   validation.

#[macro_use]
pub mod config_macros;
pub mod config;
pub mod config_simulation;
pub mod config_store;

use config::*;
use config_store::*;
use iced::{
    widget::{button, text, Column},
    Element, Renderer,
};
use tracing::info;

use crate::example::components::create_input_component;

#[derive(Debug, Clone)]
pub enum EditorToAppMessage {
    SaveConfig(ConfigStore),
}

#[derive(Debug, Clone)]
pub enum EditorEvent {
    FieldChanged(String, String),
    NestedFieldChanged(String, String, String),
    SaveButtonPressed,
    DebugConfig,
}

/// A dedicated component for editing a config that implements the Config trait.
/// The ConfigEditor makes use of a "store", which is an important intermediary
/// storage for the config values.
/// Without storing as Strings, they would get stored in the native config
/// types. This can lead to the text inputs being very clunky to use because
/// they only accept the native type.
/// - store is a recursive ConfigStore type that can hold individual values or
///   more Stores.
#[derive(Debug, Clone)]
pub struct ConfigEditor {
    store: ConfigStore,
}

impl ConfigEditor {
    pub fn new<C: Config>(config: C) -> Self
    where
        ConfigStore: From<C>,
    {
        Self {
            store: config.into(),
        }
    }

    pub fn update(&mut self, event: EditorEvent) -> Option<EditorToAppMessage> {
        match event {
            EditorEvent::FieldChanged(field_name, value) => {
                self.set_field(field_name, value);
            }
            EditorEvent::NestedFieldChanged(nested_name, field_name, value) => {
                self.set_nested_field(nested_name, field_name, value);
            }
            EditorEvent::SaveButtonPressed => {
                return Some(EditorToAppMessage::SaveConfig(self.store.clone()));
            }
            EditorEvent::DebugConfig => {
                info!("ConfigStore: {:?}", self.store.0.clone());
            }
        }

        None
    }

    pub fn view<'a>(&self) -> Element<'a, EditorEvent> {
        let mut column = Column::new();

        // Iterate through the store and render each field.
        for (field_name, field_value) in self.store.0.iter() {
            match field_value {
                StoreField::Value(s) => {
                    column = column.push(create_field_input(field_name.clone(), s.clone()));
                }
                StoreField::Nested(nested_store) => {
                    column = column.push(text(field_name.as_str()));
                    for (nested_field_name, nested_field_value) in nested_store.0.iter() {
                        match nested_field_value {
                            StoreField::Value(s) => {
                                column = column.push(create_nested_field_input(
                                    field_name.clone(),
                                    nested_field_name.clone(),
                                    s.clone(),
                                ))
                            }
                            // Nested fields with depth 2 are unsupported.
                            StoreField::Nested(_) => {}
                        }
                    }
                }
            }
        }

        column = column.push(button(text("Save")).on_press(EditorEvent::SaveButtonPressed));
        column = column.push(button(text("Debug")).on_press(EditorEvent::DebugConfig));

        column.into()
    }

    pub fn set_field(&mut self, field_name: String, value: String) {
        let current_value = self.store.0.get(&field_name).unwrap();
        let current_value = match current_value {
            StoreField::Value(s) => s.clone(),
            StoreField::Nested(_) => String::new(),
        };

        info!(
            "Changing field: {} from {} to {}",
            field_name, current_value, value
        );
        // Edit the store's field value.
        self.store
            .0
            .insert(field_name.clone(), StoreField::Value(value.clone()));
    }

    pub fn set_nested_field(&mut self, nested_name: String, field_name: String, value: String) {
        let current_value = self.store.0.get(&nested_name).unwrap();
        let current_value = match current_value {
            StoreField::Value(s) => s.clone(),
            StoreField::Nested(_) => String::new(),
        };

        info!(
            "Changing nested field: {} from {} to {}",
            field_name, current_value, value
        );

        // Edit the store's nested field value.
        let nested_store = self.store.0.get_mut(&nested_name).unwrap();
        let nested_store = match nested_store {
            StoreField::Value(_) => return,
            StoreField::Nested(s) => s,
        };

        nested_store
            .0
            .insert(field_name.clone(), StoreField::Value(value.clone()));
    }
}

/// Renders a single field of a config and emits [`EditorEvent::FieldChanged`]
/// when the field is changed.
pub fn create_field_input<'a>(
    field_name: String,
    field_value: String,
) -> Element<'a, EditorEvent, Renderer> {
    let mut column = Column::new();

    column = column.push(text(field_name.as_str()));
    column = column.push(create_input_component(Some(field_value), move |x| {
        EditorEvent::FieldChanged(field_name.clone(), x.unwrap_or_default())
    }));

    column.into()
}

/// Renders a field with a depth > 1 and emits
/// [`EditorEvent::NestedFieldChanged`] when the field is changed.
pub fn create_nested_field_input<'a>(
    nested_field_label: String,
    field_label: String,
    field_value: String,
) -> Element<'a, EditorEvent, Renderer> {
    let mut column = Column::new();

    column = column.push(text(field_label.as_str()));
    column = column.push(create_input_component(Some(field_value), move |x| {
        EditorEvent::NestedFieldChanged(
            nested_field_label.clone(),
            field_label.clone(),
            x.unwrap_or_default(),
        )
    }));

    column.into()
}
