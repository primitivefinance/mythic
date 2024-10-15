//! Alloy is a library for connecting to EVM chains.

use alloy::{
    providers::{Provider, ProviderBuilder, RootProvider, WsConnect},
    pubsub::PubSubFrontend,
    rpc::client::NoParams,
};
use anyhow::Result;
use futures_util::StreamExt;
use iced::futures;
use std::sync::Arc;

use crate::app;

#[derive(Clone)]
pub struct AlloyClient {
    pub provider: Option<Arc<RootProvider<PubSubFrontend>>>,
}

impl Default for AlloyClient {
    fn default() -> Self {
        Self { provider: None }
    }
}

impl AlloyClient {
    pub async fn connect(self, rpc_url: &str) -> Result<Self> {
        let ws = WsConnect::new(rpc_url);
        let provider = ProviderBuilder::new().on_ws(ws).await?;

        // Polling begins after the first transaction and continues to monitor for all future pending transactions until the provider is dropped.
        provider
            .client()
            .set_poll_interval(std::time::Duration::from_millis(10_000));

        Ok(Self {
            provider: Some(Arc::new(provider)),
        })
    }

    pub async fn snapshot(&self) -> Result<app::AnvilSave> {
        let snapshot = self
            .provider
            .as_ref()
            .unwrap()
            .raw_request("anvil_dumpState".into(), NoParams::default())
            .await?;

        Ok(app::AnvilSave {
            snapshot,
            block_number: 0,
        })
    }
}

impl std::fmt::Debug for AlloyClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AlloyClient {{ provider: {:?} }}", self.provider)
    }
}

#[tracing::instrument(skip(client))]
pub fn listen_for_blocks(
    client: Arc<AlloyClient>,
) -> impl futures::stream::Stream<Item = app::AppMessage> {
    iced::stream::channel(0, |mut output| async move {
        let client = client.clone();
        let provider = match client.provider.as_ref() {
            Some(provider) => provider,
            None => {
                tracing::error!("No provider found for client");
                return;
            }
        };

        let sub = match provider.subscribe_blocks().await {
            Ok(sub) => sub,
            Err(e) => {
                tracing::error!("Failed to subscribe to blocks: {}", e);
                return;
            }
        };
        let mut stream = sub.into_stream();
        loop {
            while let Some(block) = stream.next().await {
                tracing::debug!("Received block: {:?}", block);

                if let Err(e) = output.try_send(
                    app::StateEvent::Update("block_number".to_string(), block.header.number.into())
                        .into(),
                ) {
                    tracing::error!("Failed to send block number to app: {}", e);
                }
            }
        }
    })
}
