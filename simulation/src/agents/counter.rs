//! The most basic agent... it just increments a counter.

use bindings::{counter::Counter, g3m::G3M};

use super::*;

#[derive(Clone, Debug)]
pub struct CounterAgent {
    pub client: Arc<RevmMiddleware>,
    pub counter: Counter<RevmMiddleware>,
    pub lp_address: Address,
}

impl CounterAgent {
    pub async fn new(
        environment: &Environment,
        config: &SimulationConfig<Single>,
        label: impl Into<String>,
        lp_address: Address,
    ) -> Result<Self> {
        let label: String = label.into();
        let client = RevmMiddleware::new(environment, Some(&label))?;

        let counter = Counter::deploy(client.clone(), ())?.send().await?;

        tracing::trace!("Made a new counter agent with label {}", label);
        Ok(Self {
            client,
            counter,
            lp_address,
        })
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

        // we are also going to call increment on the g3m.
        // we setup this increment so that it affects each g3m differently.
        if (self.lp_address.clone() != Address::zero()) {
            let g3m = G3M::new(self.lp_address, self.client.clone());
            g3m.increment().send().await?.await?;
        }
        Ok(())
    }
}

#[async_trait::async_trait]
impl Agent for CounterAgent {
    fn as_any(&self) -> &dyn Any {
        self
    }

    async fn step(&mut self) -> Result<()> {
        tracing::trace!("Incrementing counter");
        // very simple!
        self.increment().await?;
        Ok(())
    }
}
