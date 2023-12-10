use std::fmt::Debug;

use ethers::{core::k256::ecdsa::SigningKey, utils::AnvilInstance};
use tokio_util::sync::CancellationToken;
use tracing::info;

use super::*;

/// Connects a middleware client to the underlying protocol and exposes all
/// the necessary methods to interact with the connected protocol.
#[async_trait::async_trait]
pub trait Cortex: Send + Sync {
    type Middleware: Middleware + 'static;
    type Provider: JsonRpcClient + 'static + PubsubClient + Send + Sync;
    type Signer: Signer + 'static;

    /// Gets the middleware client to send requests to the network.
    fn client(&self) -> Arc<Self::Middleware>;

    /// Gets the underlying ws or http provider.
    fn provider(&self) -> Arc<Provider<Self::Provider>>;

    /// Gets the underlying signer.
    fn signer(&self) -> Option<Self::Signer>;

    /// Gets the protocol address.
    fn protocol_address(&self) -> ethers::types::Address;

    /// Gets the strategy contract address.
    fn strategy_address(&self) -> ethers::types::Address;

    // /// Subscribes to an event log and returns a [`Watcher`] handle to allow
    // the caller to cancel the subscription.
    // todo: does this even work?
    // async fn subscribe<D: EthEvent + Debug + 'static>(
    // &self,
    // block: impl Into<BlockNumber> + Send,
    // address: ethers::types::Address,
    // ) -> anyhow::Result<Watcher> {
    // let client = self.provider();
    // Watcher::new::<D, Self::Provider>(client, block, address).await
    // }
}

impl std::fmt::Debug
    for dyn Cortex<
        Middleware = SignerMiddleware<Provider<Ws>, LocalWallet>,
        Provider = Ws,
        Signer = LocalWallet,
    >
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Cortex")
            .field("protocol_address", &self.protocol_address())
            .field("strategy_address", &self.strategy_address())
            .finish()
    }
}

/// Subscribes to an event and returns a handle to allow the caller to cancel
/// the subscription.
#[derive(Debug)]
pub struct Watcher {
    pub handle: CancellationToken,
}

impl Watcher {
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
                    return;
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

pub async fn connect_call_client(url: String) -> anyhow::Result<Provider<Http>> {
    let client = Provider::<Http>::try_from(&url).unwrap();
    Ok(client)
}

pub async fn connect_sub_client(url: String) -> anyhow::Result<Provider<Ws>> {
    let client = Provider::<Ws>::connect(&url).await?;
    Ok(client)
}

pub async fn from_anvil(
    anvil: &Arc<AnvilInstance>,
) -> anyhow::Result<(Vec<Provider<Ws>>, Vec<Wallet<SigningKey>>)> {
    let mut clients = Vec::new();
    let mut wallets = Vec::new();

    let wallet: LocalWallet = anvil
        .keys()
        .first()
        .expect("no keys in anvil")
        .clone()
        .into();

    let wallet = wallet.with_chain_id(anvil.chain_id());
    let url = anvil.endpoint();
    let url = url.replace("http", "ws");

    let provider = connect_sub_client(url)
        .await
        .expect("failed to connect to anvil");

    clients.push(provider);
    wallets.push(wallet);

    Ok((clients, wallets))
}

pub fn s_curve(x: f32) -> f32 {
    let sigmoid_x = 1.0 / (1.0 + (-x).exp());
    (sigmoid_x - 0.5) * 2.0
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_s_curve() {
        let mut t = 0.0;
        while t < 1.0 {
            let s_curve = super::s_curve(t);
            println!("s_curve: {} {}", t, s_curve);
            assert!(s_curve >= 0.0);
            assert!(s_curve <= 1.0);
            t += 0.01;
        }
    }
}
