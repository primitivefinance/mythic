//! Implements the `iced::widget::Component` trait to render the config editor.
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
//! - Config is instantiated by having a config type that implements the Config trait converted into a Store.
//! - Updating the config will edit the store
//! - Saving the config will call back to the app with the store, and the app will convert the store back into a config before saving it.
//! -
//! ## The Bugs
//! - Fields get switched after the first update.
//! - Fields don't give feedback if they have invalid inputs.
//! - Converting the store back into a config is a little jank because of data validation.

use std::collections::HashMap;

use iced::{
    widget::{button, text, Column},
    Element, Renderer,
};
use tracing::info;

use super::config::*;
use crate::example::components::create_input_component;

/// A config can have a field that is a value, or a nested group of more fields.
#[derive(Debug, Clone)]
pub enum StoreField {
    Value(String),
    Nested(Store),
}

/// A recursive data type to hold fields and nested fields.
#[derive(Debug, Clone)]
pub struct Store(HashMap<String, StoreField>);

impl Store {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
    pub fn inner(&self) -> &HashMap<String, StoreField> {
        &self.0
    }

    pub fn inner_mut(&mut self) -> &mut HashMap<String, StoreField> {
        &mut self.0
    }

    pub fn get(&self, field_name: &str) -> Option<&StoreField> {
        self.0.get(field_name)
    }

    pub fn get_mut(&mut self, field_name: &str) -> Option<&mut StoreField> {
        self.0.get_mut(field_name)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &StoreField)> {
        self.0.iter()
    }
}

#[derive(Debug, Clone)]
pub enum EditorToAppMessage {
    SaveConfig(Store),
}

/// A dedicated component for editing a config that implements the Config trait.
/// The ConfigEditor makes use of a "store", which is an important intermediary
/// storage for the config values.
/// Without storing as Strings, they would get stored in the native config
/// types. This can lead to the text inputs being very clunky to use because
/// they only accept the native type.
/// - store is a recursive Store type that can hold individual values or more
///   Stores.
#[derive(Debug, Clone)]
pub struct ConfigEditor {
    store: Store,
}

impl ConfigEditor {
    pub fn new<C: Config>(config: C) -> Self
    where
        Store: From<C>,
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
                info!("Config: {:?}", self.store.0.clone());
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

#[derive(Debug, Clone)]
pub enum EditorEvent {
    FieldChanged(String, String),
    NestedFieldChanged(String, String, String),
    SaveButtonPressed,
    DebugConfig,
}

/// Converts a Config into a Store.
impl From<Box<dyn Config>> for Store {
    fn from(c: Box<dyn Config>) -> Self {
        let mut store = Store(HashMap::new());

        // Populate the store with the current values of the config.
        for field in c.fields().iter() {
            let value = field.get_value().clone();
            store.0.insert(field.label.clone(), value.clone().into());
        }

        // Populate the store with all the nested fields of the config.
        for nested in c.nested_fields().iter() {
            let mut nested_field_store = Store(HashMap::new());
            for field in nested.field.fields().iter() {
                let value = field.get_value().clone();
                nested_field_store
                    .0
                    .insert(field.label.clone(), value.clone().into());
            }

            store
                .0
                .insert(nested.name.clone(), nested_field_store.into());
        }

        store
    }
}

impl From<String> for StoreField {
    fn from(s: String) -> Self {
        StoreField::Value(s)
    }
}

impl From<StoreField> for String {
    fn from(s: StoreField) -> Self {
        match s {
            StoreField::Value(s) => s,
            StoreField::Nested(_) => String::new(),
        }
    }
}

impl From<Store> for StoreField {
    fn from(s: Store) -> Self {
        StoreField::Nested(s)
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
