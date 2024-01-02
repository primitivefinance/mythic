use std::fmt::Debug;

use tokio_util::sync::CancellationToken;
use tracing::info;

use super::*;

/// Subscribes to an event and returns a handle to allow the caller to cancel
/// the subscription.
#[derive(Debug)]
pub struct Watcher {
    pub handle: CancellationToken,
}

impl Watcher {
    #[allow(dead_code)]
    pub async fn new<D: EthEvent + Debug + 'static, P: PubsubClient + 'static>(
        client: Arc<Provider<P>>,
        block: impl Into<BlockNumber>,
        address: ethers::types::Address,
    ) -> anyhow::Result<Self> {
        let cancel_token = CancellationToken::new();
        let cloned_token = cancel_token.clone();

        let event = Contract::event_of_type::<D>(client.clone())
            .from_block(block)
            .address(ValueOrArray::Array(vec![address]));

        info!("Starting event stream watcher");

        // Note that `log` has type AnswerUpdatedFilter
        // In this code, the tokio::select! block will either wait for the cancellation
        // token to be triggered or for the event stream to complete.
        // If the cancellation token is triggered,
        // it will log a message and then return,
        // effectively stopping the task.
        // If the event stream completes,
        // it will log a message and continue running.
        let _handle = tokio::spawn(async move {
            tokio::select! {
                _ = cloned_token.cancelled() => {
                    info!("Cancellation token triggered");
                }
                _ = async {
                    let mut set = tokio::task::JoinSet::new();
                    set.spawn(async move {
                        let mut stream = event.subscribe_with_meta().await.unwrap();
                        while let Some(Ok((log, meta))) = stream.next().await {
                            info!("{log:?}");
                            info!("{meta:?}")
                        }
                    });

                    while let Some(res) = set.join_next().await {
                        info!("stream watcher completed: {:?}", res);
                    }
                } => {}
            }
        });

        info!("Exited event stream watcher");

        Ok(Self {
            handle: cancel_token.clone(),
        })
    }
}
