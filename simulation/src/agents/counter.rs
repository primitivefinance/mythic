//! The most basic agent... it just increments a counter.

use super::*;
use bindings::counter::Counter;

#[derive(Clone)]
pub struct CounterAgent {
    pub client: Arc<RevmMiddleware>,
    pub counter: Counter<RevmMiddleware>,
}

impl CounterAgent {
    pub async fn new(
        environment: &Environment,
        config: &SimulationConfig<Single>,
        label: impl Into<String>,
    ) -> Result<Self> {
        let label: String = label.into();
        let client = RevmMiddleware::new(environment, Some(&label))?;

        let counter = Counter::deploy(client.clone(), ())?.send().await?;

        tracing::trace!("Made a new counter agent with label {}", label);
        Ok(Self { client, counter })
    }

    pub async fn get(&self) -> Result<U256> {
        let res = self.counter.number().call().await;
        match res {
            Ok(res) => Ok(res),
            Err(err) => Err(anyhow::anyhow!("Error getting counter: {}", err)),
        }
    }

    pub async fn increment(&self) -> Result<()> {
        self.counter.increment().send().await?.await?;
        Ok(())
    }
}

#[async_trait::async_trait]
impl Agent for CounterAgent {
    async fn step(&mut self) -> Result<()> {
        tracing::trace!("Incrementing counter");
        // very simple!
        self.increment().await?;
        Ok(())
    }
}
