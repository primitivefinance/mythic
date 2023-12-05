use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

// todo: remove this dep
use crate::{
    agents::AgentParameters,
    settings::{parameters::Multiple, SimulationConfig},
};

pub type AgentConfigurations<P> = BTreeMap<String, AgentParameters<P>>;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ConfigBuilder {
    pub config: SimulationConfig<Multiple>,
}

impl ConfigBuilder {
    pub fn new() -> Self {
        Self {
            config: SimulationConfig::default(),
        }
    }

    /// Adds all configuration parameters from a Configurable type.
    pub fn add_configurable(&mut self, configurable: &dyn Configurable) {
        self.config
            .agent_parameters
            .extend(configurable.configure().into_iter());
    }

    pub fn get(&self) -> SimulationConfig<Multiple> {
        self.config.clone()
    }
}

/// Implement this trait to add a Configurable type to the config builder.
pub trait Configurable {
    fn configure(&self) -> BTreeMap<String, AgentParameters<Multiple>>;
}
