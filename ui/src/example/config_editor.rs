use iced::{
    widget::{button, component, text, text_input, Column, Component},
    Element, Renderer,
};
use simulation::{
    settings::{self, parameters::Parameterized},
    simulations::SimulationType,
};

/// Generic enum type for use as config fields.
pub trait ConfigEnum: std::fmt::Debug {}

pub enum ConfigFieldType {
    StringType(String),
    ValueType(f64),
    EnumType(Box<dyn ConfigEnum>),
    ConfigType(Box<dyn Config>),
}

impl ConfigFieldType {
    pub fn raw(&self) -> String {
        match self {
            ConfigFieldType::StringType(s) => s.clone(),
            ConfigFieldType::ValueType(s) => s.to_string(),
            ConfigFieldType::EnumType(e) => format!("{:?}", e),
            _ => String::new(),
        }
    }
}

pub struct ConfigField {
    label: String,
    value: ConfigFieldType,
}

macro_rules! config_fields {
    ($self:expr, $($field:ident),* $(,)?) => {
        vec![
            $(
                ConfigField {
                    label: stringify!($field).to_string(),
                    value: ConfigFieldType::from($self.$field.clone()),
                },
            )*
        ]
    };
}

impl From<u16> for ConfigFieldType {
    fn from(s: u16) -> Self {
        ConfigFieldType::ValueType(s as f64)
    }
}

impl From<u64> for ConfigFieldType {
    fn from(s: u64) -> Self {
        ConfigFieldType::ValueType(s as f64)
    }
}

impl From<String> for ConfigFieldType {
    fn from(s: String) -> Self {
        ConfigFieldType::StringType(s)
    }
}

impl From<f64> for ConfigFieldType {
    fn from(s: f64) -> Self {
        ConfigFieldType::ValueType(s)
    }
}

impl From<Box<dyn Config>> for ConfigFieldType {
    fn from(s: Box<dyn Config>) -> Self {
        ConfigFieldType::ConfigType(s)
    }
}

impl ConfigEnum for SimulationType {}
impl From<SimulationType> for ConfigFieldType {
    fn from(e: SimulationType) -> Self {
        ConfigFieldType::EnumType(Box::new(e))
    }
}

impl From<Option<usize>> for ConfigFieldType {
    fn from(e: Option<usize>) -> Self {
        ConfigFieldType::ValueType(e.unwrap_or_default() as f64)
    }
}

impl From<Option<String>> for ConfigFieldType {
    fn from(e: Option<String>) -> Self {
        ConfigFieldType::StringType(e.unwrap_or_default())
    }
}

impl From<Option<u16>> for ConfigFieldType {
    fn from(e: Option<u16>) -> Self {
        ConfigFieldType::ValueType(e.unwrap_or_default() as f64)
    }
}

pub trait Config {
    fn fields(&self) -> Vec<ConfigField>;
    fn set_field(&mut self, field_name: String, value: String);
    fn set_nested_field(&mut self, nested_name: String, field_name: String, value: String) {}
    fn nested_fields(&self) -> Vec<NestedConfigField> {
        vec![]
    }
}

pub struct ConfigEditor<C: Config> {
    config: C,
    field_values: Vec<String>,
    save_button: button::State,
}

impl<C: Config> ConfigEditor<C> {
    pub fn new(config: C) -> Self {
        let field_values = vec![String::new(); config.fields().len()];
        Self {
            config,
            field_values,
            save_button: button::State::new(),
        }
    }
}

impl<Message, C: Config> Component<Message, Renderer> for ConfigEditor<C> {
    type State = ();
    type Event = Event;

    fn update(&mut self, _state: &mut Self::State, event: Event) -> Option<Message> {
        match event {
            Event::FieldChanged(field_name, value) => {
                tracing::info!("Field changed: {} to {}", field_name, value);
                self.config.set_field(field_name, value);
                None
            }
            Event::NestedFieldChanged(nested_name, field_name, value) => {
                tracing::info!("Nested field changed: {} to {}", field_name, value);
                self.config.set_nested_field(nested_name, field_name, value);
                None
            }
            Event::SaveButtonPressed => {
                tracing::info!("Save button pressed");
                None
            }
        }
    }

    fn view(&self, _state: &Self::State) -> iced::Element<Event, Renderer> {
        let mut column = Column::new();
        for (i, field) in self.config.fields().iter().enumerate() {
            let label = field.label.clone();
            let value = field.value.raw().clone();
            column = column.push(text(label.as_str()));
            column = column.push(
                text_input("Enter a value...", value.as_str())
                    .on_input(move |x| Event::FieldChanged(label.clone(), x)),
            );
        }
        for (j, nested) in self.config.nested_fields().iter().enumerate() {
            for (i, field) in nested.field.fields().iter().enumerate() {
                let name = nested.name.clone();
                let label = field.label.clone();
                let value = field.value.raw().clone();
                column = column.push(text(label.as_str()));
                column =
                    column.push(text_input("Enter a value...", value.as_str()).on_input(
                        move |x| Event::NestedFieldChanged(name.clone(), label.clone(), x),
                    ));
            }
        }
        column = column.push(button(text("Save")).on_press(Event::SaveButtonPressed));
        column.into()
    }
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

pub struct NestedConfigField {
    name: String,
    field: Box<dyn Config>,
}

macro_rules! nested_configs {
    ($($field:expr),* $(,)?) => {
        vec![
            $(
                NestedConfigField {
                    name: format!("{}", stringify!($field).split('.').last().unwrap_or("").to_string()),
                    field: Box::new($field.clone()),
                },
            )*
        ]
    };
}

impl<P: Parameterized<f64> + Default> Config for settings::SimulationConfig<P> {
    fn fields(&self) -> Vec<ConfigField> {
        config_fields!(
            self,
            simulation,
            max_parallel,
            output_directory,
            output_file_name
        )
    }

    fn set_field(&mut self, field_name: String, value: String) {
        match field_name.as_str() {
            "max_parallel" => self.max_parallel = value.parse().ok(),
            "output_directory" => self.output_directory = value,
            "output_file_name" => self.output_file_name = value.parse().ok(),
            _ => tracing::info!(
                "Attempting field name: {} with value: {}",
                field_name,
                value
            ),
        }
    }

    fn nested_fields(&self) -> Vec<NestedConfigField> {
        nested_configs!(self.pool)
    }

    fn set_nested_field(&mut self, nested_name: String, field_name: String, value: String) {
        if nested_name == "pool" {
            match field_name.as_str() {
                "fee_basis_points" => {
                    self.pool.fee_basis_points = value.parse().ok().unwrap_or_default()
                }
                "weight_x" => self.pool.weight_x = value.parse().ok().unwrap_or_default(),
                "target_volatility" => {
                    self.pool.target_volatility = value.parse().ok().unwrap_or_default()
                }
                _ => tracing::info!(
                    "Attempting nested field name: {} with value: {}",
                    field_name,
                    value
                ),
            }
        } else {
            tracing::info!(
                "Attempting nested field name: {} with value: {}",
                nested_name,
                value
            )
        }
    }
}

impl Config for settings::parameters::PoolParameters {
    fn fields(&self) -> Vec<ConfigField> {
        config_fields!(self, fee_basis_points, weight_x, target_volatility)
    }

    fn set_field(&mut self, field_name: String, value: String) {
        match field_name.as_str() {
            "fee_basis_points" => self.fee_basis_points = value.parse().ok().unwrap_or_default(),
            "weight_x" => self.weight_x = value.parse().ok().unwrap_or_default(),
            "target_volatility" => self.target_volatility = value.parse().ok().unwrap_or_default(),
            _ => tracing::info!(
                "Attempting field name: {} with value: {}",
                field_name,
                value
            ),
        }
    }
}
