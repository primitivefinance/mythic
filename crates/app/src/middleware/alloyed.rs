use std::fmt::Debug;

use alloy_networks::{Network, Receipt};
use alloy_primitives::{Address, Bloom, Bytes, B256, U128, U256, U64, U8};

use alloy_pubsub::PubSubFrontend;
use alloy_rlp::{RlpDecodable, RlpEncodable};
use alloy_rpc_client::{ClientBuilder, RpcClient};
use alloy_transport_http::Http;
use alloy_transport_ws::WsConnect;
use anyhow::anyhow;
use reqwest::Client;

pub struct ExcNet;

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ExcReceipt {
    /// Transaction Hash.
    pub transaction_hash: Option<B256>,
    /// Index within the block.
    pub transaction_index: U64,
    /// Hash of the block this transaction was included within.
    pub block_hash: Option<B256>,
    /// Number of the block this transaction was included within.
    pub block_number: Option<U256>,
    /// Cumulative gas used within the block after this was executed.
    pub cumulative_gas_used: U256,
    /// Gas used by this transaction alone.
    pub gas_used: Option<U256>,
    /// The price paid post-execution by the transaction (i.e. base fee +
    /// priority fee). Both fields in 1559-style transactions are maximums
    /// (max fee + max priority fee), the amount that's actually paid by
    /// users can only be determined post-execution
    pub effective_gas_price: U128,
    /// Blob gas used by the eip-4844 transaction
    ///
    /// This is None for non eip-4844 transactions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blob_gas_used: Option<U128>,
    /// The price paid by the eip-4844 transaction per blob gas.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blob_gas_price: Option<U128>,
    /// Address of the sender
    pub from: Address,
    /// Address of the receiver. null when its a contract creation transaction.
    pub to: Option<Address>,
    /// Contract address created, or None if not a deployment.
    pub contract_address: Option<Address>,
    /// Logs emitted by this transaction.
    pub logs: Vec<alloy_primitives::Log>,
    /// Logs bloom
    pub logs_bloom: Bloom,
    /// The post-transaction stateroot (pre Byzantium)
    ///
    /// EIP98 makes this optional field, if it's missing then skip serializing
    /// it
    #[serde(skip_serializing_if = "Option::is_none", rename = "root")]
    pub state_root: Option<B256>,
    /// Status: either 1 (success) or 0 (failure). Only present after activation
    /// of EIP-658
    #[serde(skip_serializing_if = "Option::is_none", rename = "status")]
    pub status_code: Option<U64>,
    /// EIP-2718 Transaction type, Some(1) for AccessList transaction, None for
    /// Legacy
    #[serde(rename = "type")]
    pub transaction_type: U8,
}

impl Receipt for ExcReceipt {}

#[derive(Clone, Debug, RlpEncodable, RlpDecodable, serde::Serialize, serde::Deserialize)]
#[rlp(trailing)]
pub struct ExcTransaction {
    /// from address
    pub from: Option<Address>,
    /// to address
    pub to: Option<Address>,
    /// legacy, gas Price
    #[serde(default)]
    pub gas_price: Option<U128>,
    /// max base fee per gas sender is willing to pay
    #[serde(default)]
    pub max_fee_per_gas: Option<U128>,
    /// miner tip
    #[serde(default)]
    pub max_priority_fee_per_gas: Option<U128>,
    /// gas
    pub gas: Option<U256>,
    /// value of th tx in wei
    pub value: Option<U256>,
    /// Any additional data sent
    #[serde(alias = "input")]
    pub data: Option<Bytes>,
    /// Transaction nonce
    pub nonce: Option<U64>,
    /// EIP-2718 type
    #[serde(rename = "type")]
    pub transaction_type: Option<U8>,
}

impl alloy_networks::Transaction for ExcTransaction {
    fn set_gas(&mut self, gas: alloy_primitives::U256) {
        self.gas = Some(gas);
    }
}

#[derive(Clone, Debug, RlpDecodable, RlpEncodable, serde::Serialize, serde::Deserialize)]
pub struct MockError(pub String);

#[derive(Clone, Debug, RlpDecodable, RlpEncodable, serde::Serialize, serde::Deserialize)]
#[rlp(trailing)]
pub struct MockResponse {
    pub result: String,
    pub error: Option<MockError>,
}

impl alloy_networks::Transaction for MockResponse {
    fn set_gas(&mut self, _gas: alloy_primitives::U256) {}
}

impl Network for ExcNet {
    type Receipt = ExcReceipt;
    type TransactionRequest = ExcTransaction;
    type TransactionResponse = MockResponse;
}

pub type ExcProvider = alloy_providers::provider::Provider<Http<Client>>;

pub struct ExcaliburMiddleware;

impl ExcaliburMiddleware {
    pub async fn build(url: &str) -> anyhow::Result<RpcClient<PubSubFrontend>> {
        let ws = WsConnect {
            url: url.to_string(),
            auth: None,
        };
        ClientBuilder::default()
            .ws(ws)
            .await
            .map_err(|e| anyhow!("failed to build client {}", e))
    }

    // pub fn provider(url: &str) -> anyhow::Result<ExcProvider> {
    //     Ok(ExcProvider::new(url).map_err(|e| anyhow!("failed to build provider {}", e))?)
    // }
    pub fn provider(url: &str) -> anyhow::Result<ExcProvider> {
        let url = url::Url::parse(url).map_err(|e| anyhow!("failed to parse url: {}", e))?;
        let http = Http::new(url);
        Ok(ExcProvider::new(http))
    }
}

#[cfg(test)]
mod test {
    use alloy_primitives::{Bytes, U64};
    use alloy_providers::{provider::ClientError, NetworkRpcClient};
    use alloy_providers::{provider::TempProvider, Provider};
    use alloy_signer::LocalWallet;
    use alloy_signer::{Signer, SignerSync};
    use ethers::utils::AnvilInstance;
    use ethers::{
        middleware::{Middleware, MiddlewareBuilder},
        signers::{LocalWallet as EthersWallet, Signer as EthersSigner},
        types::{transaction::eip2718::TypedTransaction, Bytes as EthersBytes},
        utils::Anvil,
    };
    use sim::to_ethers_address;
    use std::time::Duration;

    use super::*;

    async fn setup() -> anyhow::Result<AnvilInstance> {
        let anvil = Anvil::default()
            .arg("--gas-limit")
            .arg("20000000")
            .chain_id(31337_u64)
            .spawn();

        Ok(anvil)
    }

    #[tokio::test]
    async fn test_excalibur_middleware() {
        let anvil = setup().await.unwrap();
        let http_endpoint = anvil.endpoint();

        let client = ExcaliburMiddleware::provider(&http_endpoint).unwrap();
        let block = client.get_block_number().await.unwrap();
        println!("block number: {}", block);

        let chain_id = client.get_chain_id().await.unwrap();
        println!("chain id: {}", chain_id);
        assert_eq!(chain_id, U64::from(31337));
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn excalibur_middleware_sign_tx() -> anyhow::Result<()> {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::TRACE)
            .init();
        let anvil = setup().await?;
        let http_endpoint = anvil.endpoint();
        let chain_id = anvil.chain_id();

        let bob_key = anvil.keys().first().expect("no keys in anvil").clone();
        let alice_key = anvil.keys().last().expect("no keys in anvil").clone();

        let alice_wallet = LocalWallet::from(alice_key.clone()).with_chain_id(chain_id);
        println!("Alice {}", alice_wallet.address());
        let wallet = LocalWallet::from(bob_key.clone()).with_chain_id(chain_id);
        println!("Bob {}", wallet.address());
        let ethers_wallet: EthersWallet =
            EthersWallet::from(bob_key.clone()).with_chain_id(chain_id);
        println!("Ethers Wallet Bob {}", wallet.address());

        let provider = ExcaliburMiddleware::provider(&http_endpoint)?;
        let bob_balance = provider.get_balance(wallet.address(), None).await?;
        println!("Bob balance: {}", bob_balance);
        let alice_balance = provider.get_balance(alice_wallet.address(), None).await?;
        println!("Alice balance: {}", alice_balance);

        println!("Building send transaction.");
        let mut tx: TypedTransaction = ethers::types::TransactionRequest::new()
            .nonce(0)
            .from(to_ethers_address(wallet.address()))
            .to(to_ethers_address(alice_wallet.address()))
            .value(200_u64)
            .gas_price(20000000000u64)
            .gas(50_000)
            .chain_id(chain_id)
            .into();

        let chain_id = ethers_wallet.chain_id();
        match tx.chain_id() {
            Some(id) if id.as_u64() != chain_id => {
                return Err(anyhow!(
                    "chain id mismatch: tx: {}, wallet: {}",
                    id.as_u64(),
                    chain_id
                ))
            }
            None => {
                tx.set_chain_id(chain_id);
            }
            _ => {}
        }

        println!("tx: {:?}", tx);

        // Sign the tx payload.
        let signature = ethers_wallet.sign_transaction(&tx).await?;
        println!("signature: {:?}", signature);

        // RLP encode the tx with the signature.
        let ethers_tx_payload: EthersBytes = tx.rlp_signed(&signature);
        println!("ethers_tx_payload: {:?}", ethers_tx_payload);

        // Try sending over ethers provider.
        let ethers_provider =
            ethers::providers::Provider::<ethers::providers::Ws>::connect(anvil.ws_endpoint())
                .await
                .unwrap()
                .interval(Duration::from_millis(50u64));
        let ethers_provider = ethers_provider.with_signer(ethers_wallet.clone());

        println!("Sending tx over ethers provider.");
        let ethers_tx = ethers_provider
            .send_raw_transaction(ethers_tx_payload)
            .await?
            .await?
            .unwrap();
        println!("ethers_tx: {:?}", ethers_tx);
        let bob_balance = provider.get_balance(wallet.address(), None).await?;
        println!("Bob balance: {}", bob_balance);
        let alice_balance = provider.get_balance(alice_wallet.address(), None).await?;
        println!("Alice balance: {}", alice_balance);

        // Increment the nonce, so we can do the tx again.
        tx.set_nonce(ethers::types::U256::from(1));

        // Resign everything.

        // Sign the tx payload.
        let signature = ethers_wallet.sign_transaction(&tx).await?;
        println!("signature: {:?}", signature);

        // Send the tx payload over the alloy provider.
        // Need to rlp sign it.
        println!("alloy tx: {:?}", tx);
        let ethers_tx_payload: EthersBytes = tx.rlp_signed(&signature);
        println!("ethers_tx_payload: {:?}", ethers_tx_payload);

        let tx_payload: Bytes = Bytes(ethers_tx_payload.0);
        println!("tx_payload: {:?}", tx_payload);

        let url = anvil
            .endpoint()
            .parse()
            .map_err(|_e| ClientError::ParseError)?;
        let inner = ClientBuilder::default().reqwest_http(url);

        let nc: NetworkRpcClient<ExcNet, Http<reqwest::Client>> = NetworkRpcClient::from(inner);

        let tx_request = ExcTransaction {
            from: Some(wallet.address()),
            to: Some(alice_wallet.address()),
            gas: None,
            gas_price: None,
            max_fee_per_gas: None,
            max_priority_fee_per_gas: None,
            value: Some(U256::from(200_u64)),
            data: None,
            nonce: Some(U64::from(0_u64)),
            transaction_type: None,
        };

        println!("ATTEMPTING SEND TRANSACTION");
        let tx_1 = nc.send_transaction(&tx_request).await?;
        println!("tx_1: {:?}", tx_1);

        // This is where defeat greeted us :(
        let tx_hash = provider.send_raw_transaction(tx_payload).await?;
        println!("tx_hash: {:?}", tx_hash);

        // Wait for the tx to be mined.
        let receipt = provider.get_transaction_receipt(tx_hash).await?.unwrap();
        println!("receipt: {:?}", receipt);

        let bob_balance = provider.get_balance(wallet.address(), None).await?;
        println!("Bob balance: {}", bob_balance);
        let alice_balance = provider.get_balance(alice_wallet.address(), None).await?;
        println!("Alice balance: {}", alice_balance);

        Ok(())
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn excalibur_middleware_sign_msg() -> anyhow::Result<()> {
        let anvil = setup().await?;

        let anvil_key = anvil.keys().first().expect("no keys in anvil").clone();

        let wallet = LocalWallet::from(anvil_key);
        println!("Address {}", wallet.address());

        let message = "Some data";
        // Sign the message
        let signature = wallet.sign_message_sync(message.as_bytes())?;
        println!("signature: {:?}", signature);

        // Recover the signer from the message
        let recovered = signature.recover_address_from_msg(message)?;
        println!("recovered: {}", recovered);

        assert_eq!(recovered, wallet.address());

        Ok(())
    }
}
