use std::{any::Any, fmt::Debug, sync::Arc};

use arbiter_core::{errors::ArbiterCoreError, middleware::ArbiterMiddleware};
use linked_hash_map::LinkedHashMap;

use super::*;

/// Universal agent methods for interacting with the simulation environment or
/// loop.3
#[async_trait::async_trait]
pub trait Agent: Sync + Send + Any + Debug {
    /// Executed before the main simulation loop starts.
    async fn init(&mut self) -> Result<()> {
        Ok(())
    }

    /// Executed by each agent inside the main simulation loop.
    /// Ordering is determined by placement in the simulation loop.
    async fn step(&mut self) -> Result<(), ArbiterCoreError> {
        Ok(())
    }

    /// Executed after the main simulation loop ends.
    async fn exit(&mut self) -> Result<()> {
        Ok(())
    }

    /// All agents exist as an individual EOA with client.
    fn client(&self) -> Arc<ArbiterMiddleware>;

    /// Typecasting to Any so that agents can be converted to other types.
    fn as_any(&self) -> &dyn Any;
}

#[derive(Debug, Default)]
pub struct Agents(pub LinkedHashMap<String, Box<dyn Agent>>);

impl Agents {
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&String, &mut Box<dyn Agent>)> {
        self.0.iter_mut()
    }

    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self(LinkedHashMap::new())
    }

    pub fn add(&mut self, agent: impl Agent + 'static) {
        self.0.insert(
            agent.client().label.as_ref().unwrap().clone(),
            Box::new(agent),
        );
    }
}

#[async_trait::async_trait]
impl Agent for Agents {
    fn client(&self) -> Arc<ArbiterMiddleware> {
        unimplemented!()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
