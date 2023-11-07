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

use std::collections::BTreeMap;

use iced::{
    widget::{button, text, Column},
    Element, Renderer,
};
use serde_json::Value;
use simulation::settings::{Parameterized, SimulationConfig};
use tracing::info;

use crate::example::components::create_input_component;

#[derive(Debug, Clone)]
pub enum EditorToAppMessage {
    SaveConfig(ConfigStore),
}

#[derive(Debug, Clone)]
pub enum EditorEvent {
    FieldChanged(String, Value),
    SaveButtonPressed,
    DebugConfig,
}

type ConfigStore = BTreeMap<String, Value>;

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

pub trait Config: serde::Serialize {
    fn to_store(&self) -> ConfigStore;
    fn from_store(store: ConfigStore) -> Self;
}

impl<P> Config for SimulationConfig<P>
where
    P: Parameterized
        + Default
        + std::fmt::Debug
        + Clone
        + 'static
        + serde::de::DeserializeOwned
        + serde::Serialize,
{
    fn to_store(&self) -> ConfigStore {
        let store: BTreeMap<String, serde_json::Value> = serde_json::to_value(&self)
            .unwrap()
            .as_object()
            .unwrap()
            .clone()
            .into_iter()
            .collect();
        store
    }

    fn from_store(store: ConfigStore) -> Self {
        let map: serde_json::Map<String, serde_json::Value> = store.into_iter().collect();
        serde_json::from_value(serde_json::Value::Object(map)).unwrap()
    }
}

impl ConfigEditor {
    pub fn new<C: Config>(config: C) -> Self {
        let store = config.to_store();
        info!("Loading config into ConfigEditor: {:?}", store);
        Self { store }
    }

    pub fn update(&mut self, event: EditorEvent) -> Option<EditorToAppMessage> {
        match event {
            EditorEvent::FieldChanged(field_name, value) => {
                self.set_field(field_name, value);
            }
            EditorEvent::SaveButtonPressed => {
                return Some(EditorToAppMessage::SaveConfig(self.store.clone()));
            }
            EditorEvent::DebugConfig => {
                info!(
                    "Debug ConfigStore in ConfigEditor: {:?}",
                    self.store.clone()
                );
            }
        }

        None
    }

    pub fn view<'a>(&self) -> Element<'a, EditorEvent> {
        let mut column = Column::new().max_width(512).spacing(10).padding(10);

        for (field_name, field_value) in self.store.iter() {
            column = self.render_field(column, field_name, field_value);
        }

        column = column.push(button(text("Save")).on_press(EditorEvent::SaveButtonPressed));
        column = column.push(button(text("Debug")).on_press(EditorEvent::DebugConfig));

        column.into()
    }

    /// Recursively renders each field of a config.
    fn render_field<'a>(
        &self,
        mut column: Column<'a, EditorEvent>,
        field_name: &String,
        field_value: &Value,
    ) -> Column<'a, EditorEvent> {
        match field_value {
            Value::Object(obj) => {
                for (nested_field_name, nested_field_value) in obj {
                    column = self.render_field(
                        column,
                        &format!("{}.{}", field_name, nested_field_name),
                        nested_field_value,
                    );
                }
            }
            _ => {
                column = column.push(create_field_input(field_name.clone(), field_value.clone()));
            }
        }

        column
    }

    pub fn set_field(&mut self, field_name: String, value: Value) {
        let parts: Vec<&str> = field_name.split('.').collect();
        let mut current_value = self.store.get_mut(parts[0]).unwrap();

        for part in parts.iter().skip(1) {
            current_value = current_value.get_mut(part).unwrap();
        }

        info!("Changing field {} to {}", field_name, value);
        *current_value = value;
    }
}

/// Renders a single field of a config and emits [`EditorEvent::FieldChanged`]
/// when the field is changed.
pub fn create_field_input<'a>(
    field_name: String,
    field_value: Value,
) -> Element<'a, EditorEvent, Renderer> {
    let mut column = Column::new();

    column = column.push(text(field_name.as_str()));
    column = column.push(create_input_component(
        Some(field_value.to_string()),
        move |x| EditorEvent::FieldChanged(field_name.clone(), x.unwrap_or_default()),
    ));

    column.into()
}
