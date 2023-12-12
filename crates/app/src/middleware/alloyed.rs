use std::fmt::Debug;

use alloy_networks::{Network, Receipt};
use alloy_primitives::Address;
use alloy_providers::{Provider, ProviderBuilder};
use alloy_pubsub::PubSubFrontend;
use alloy_rlp::{Decodable, Encodable, RlpDecodable, RlpEncodable};
use alloy_rpc_client::{ClientBuilder, RpcClient};
use alloy_transport_ws::WsConnect;
use anyhow::anyhow;
use arbiter_core::middleware::RevmMiddleware;
use ethers::{core::k256::ecdsa::SigningKey, utils::AnvilInstance};
use tokio_util::sync::CancellationToken;
use tracing::info;

pub struct ExcNet;

#[derive(Clone, Debug, RlpEncodable, RlpDecodable, serde::Serialize, serde::Deserialize)]
pub struct ExcReceipt(pub alloy_rpc_types::TransactionReceipt);
impl Receipt for ExcReceipt {}

#[rlp(trailing)]
#[derive(Clone, Debug, RlpEncodable, RlpDecodable, serde::Serialize, serde::Deserialize)]
pub struct ExcTransaction {
    from: Address,
    #[serde(skip_serializing_if = "Option::is_none")]
    to: Option<Address>,
}

impl alloy_networks::Transaction for ExcTransaction {
    fn set_gas(&mut self, gas: alloy_primitives::U256) {
        self.0.gas = Some(gas);
    }
}

impl Network for ExcNet {
    type Receipt = ExcReceipt;
    type TransactionRequest = ExcTransaction;
    type TransactionResponse = ExcTransaction;
}

pub struct ExcaliburMiddleware;

impl ExcaliburMiddleware {
    pub async fn build() -> anyhow::Result<RpcClient<PubSubFrontend>> {
        let ws = WsConnect {
            url: "ws://localhost:8545".to_string(),
            auth: None,
        };
        ClientBuilder::default()
            .ws(ws)
            .await
            .map_err(|e| anyhow!("failed to build client {}", e))
    }
}
