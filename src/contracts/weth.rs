use alloy::primitives::utils::parse_ether;
use alloy::primitives::{Address, U256};
use alloy::providers::RootProvider;
use alloy::pubsub::PubSubFrontend;
use alloy::rpc::types::TransactionReceipt;
use alloy::sol;
use anyhow::Result;
use iced::Task;
use std::collections::HashMap;
use std::sync::Arc;

use crate::blockchain::AlloyClient;
use crate::components::panes::handler::FormTransactionHandler;
use crate::components::panes::Message;

sol! {
    #[allow(missing_docs)]
    #[sol(rpc)]
    IWETH9,
    "src/contracts/abi/IWETH9.json"
}

pub async fn wrap(
    provider: Arc<RootProvider<PubSubFrontend>>,
    address: Address,
    amount: U256,
) -> Result<TransactionReceipt> {
    IWETH9::new(address, provider)
        .deposit()
        .value(amount)
        .send()
        .await?
        .with_required_confirmations(0)
        .with_timeout(Some(std::time::Duration::from_secs(2)))
        .get_receipt()
        .await
        .map_err(|e| anyhow::anyhow!(e))
}

pub struct WethWrapHandler;

impl FormTransactionHandler<Message> for WethWrapHandler {
    fn handle_transaction(
        client: Arc<AlloyClient>,
        data: &HashMap<String, String>,
    ) -> Task<Message> {
        let address: Address = data.get("weth_address").unwrap().parse().unwrap();
        let amount = parse_ether(data.get("weth_amount").unwrap()).unwrap();

        if let Some(client) = client.provider.clone() {
            let client = client.clone();

            tracing::info!("Wrapping WETH");
            Task::perform(async move { wrap(client, address, amount).await }, |r| {
                Message::TransactionResult(
                    r.map_err(|e| crate::panes::PaneError::Tx(e.to_string())),
                )
            })
        } else {
            tracing::error!("Failed to submit WETH transaction, client provider is None");
            Task::none()
        }
    }
}
