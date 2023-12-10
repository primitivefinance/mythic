//! In the Model-view-controller architecture the model is ["The central
//! component of the pattern. It is the application's dynamic data structure,
//! independent of the user interface.[14] It directly manages the data, logic
//! and rules of the application."](https://en.wikipedia.org/wiki/Model%E2%80%93view%E2%80%93controller)
//!
//! For the Portfolio dashboard, there is a lot of convoluted data that needs to
//! be managed. The portfolio dashboard is the most complex screen in the
//! application. It is responsible for:
//! - Fetching protocol state
//! - Fetching user address information
//! - Maintaining data freshness
//! - Deriving data from underlying data
//! - Updating and manipulating data
//! - Computing plots off the data to be displayed in charts
//! - Subscribing to data feeds from asynchronous sources

use alloy_sol_types::{sol, SolCall, SolInterface};
use anyhow::{Error, Result};
use ethers::types::transaction::eip2718::TypedTransaction;
use sim::{from_ethers_u256, to_ethers_address};

use super::*;

type EthersAddress = ethers::types::Address;
type EthersU256 = ethers::types::U256;
type AlloyAddress = alloy_primitives::Address;
type AlloyU256 = alloy_primitives::U256;

type Client = Arc<Provider<Ws>>;

/// The model!
#[derive(Debug, Clone, Default)]
pub struct RawDataModel<A, V> {
    pub raw_protocol_address: Option<A>,
    pub raw_strategy_address: Option<A>,
    pub raw_internal_spot_price: Option<V>,
    pub raw_external_spot_price: Option<V>,
    pub raw_asset_token: Option<A>,
    pub raw_quote_token: Option<A>,
    pub raw_user_asset_balance: Option<V>,
    pub raw_user_quote_balance: Option<V>,
    pub raw_asset_reserve: Option<V>,
    pub raw_quote_reserve: Option<V>,
    pub raw_user_asset_reserve: Option<V>,
    pub raw_user_quote_reserve: Option<V>,
    pub raw_total_liquidity: Option<V>,
    pub raw_user_total_liquidity: Option<V>,
    pub raw_strike_price_wad: Option<V>,
    pub raw_time_remaining_wad: Option<V>,
    pub raw_volatility_wad: Option<V>,
    pub raw_portfolio_values_series: Option<Vec<(u64, V)>>,
    pub raw_last_chain_data_sync_timestamp: Option<DateTime<Utc>>,
    pub raw_last_chain_data_sync_block: Option<u64>,
}

sol! {
    interface IERC20 {
        function balanceOf(address account) external view returns(uint balance);
    }
}

impl RawDataModel<AlloyAddress, AlloyU256> {
    pub type Address = AlloyAddress;
    pub type Value = AlloyU256;

    pub fn new() -> Self {
        Self::default()
    }

    pub async fn fetch_balance(client: Client, address: Self::Address) -> Result<Self::Value> {
        let converted_address = to_ethers_address(address);
        let balance = client.get_balance(converted_address, None).await?;
        let converted_balance = from_ethers_u256(balance);
        Ok(converted_balance)
    }

    #[tracing::instrument(skip(client), level = "trace", ret, err)]
    pub async fn fetch_balance_of(
        &self,
        client: Client,
        address: Self::Address,
        token_address: Self::Address,
    ) -> Result<Self::Value> {
        let converted_token_address = to_ethers_address(token_address);

        let payload = IERC20::balanceOfCall { account: address };
        let payload = ethers::types::Bytes::from(payload.abi_encode());

        let mut tx = TypedTransaction::default();
        tx.set_to(converted_token_address).set_data(payload);

        // Send the call to the token contract.
        let balance = client.call(&tx, None).await?;
        let decoded: <IERC20::balanceOfCall as SolCall>::Return =
            IERC20::balanceOfCall::abi_decode_returns(&balance, true)?;
        let decoded_balance: Self::Value = decoded.balance.into();

        Ok(decoded_balance)
    }
}

#[cfg(test)]
mod tests {
    use bindings::mock_erc20::MockERC20;
    use ethers::{prelude::*, utils::Anvil};
    use sim::from_ethers_address;

    use super::{AlloyAddress, AlloyU256, *};

    sol! {
        #[derive(Debug)]
        interface IERC20 {
            function balanceOf(address account) external view returns(uint);
        }
    }

    #[test]
    fn test_sol_ierc20() {
        println!(
            "IERC20: {:?}",
            IERC20::balanceOfCall {
                account: AlloyAddress::ZERO,
            }
        );

        println!("IERC20 selector: {:?}", IERC20::balanceOfCall::SELECTOR);
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_fetch_balance_of() -> anyhow::Result<()> {
        let anvil = Anvil::default()
            .arg("--gas-limit")
            .arg("20000000")
            .chain_id(31337_u64)
            .spawn();

        let url = anvil.ws_endpoint().to_string();
        println!("URL: {}", url);
        let wallet: LocalWallet = anvil.keys().get(0).unwrap().clone().into();
        let wallet = wallet.with_chain_id(anvil.chain_id());

        println!("Wallet address: {}", wallet.address());
        let provider = Provider::<Ws>::connect(&url).await?;

        let client = Arc::new(SignerMiddleware::new(provider, wallet));

        // Need to deploy a token and mint some to wallet!
        let token =
            MockERC20::deploy(client.clone(), ("Test".to_string(), "T".to_string(), 18_u8))?
                .send()
                .await?;

        println!("Deployed token: {:?}", token.address());

        let sender = client.address();

        token.mint(sender, 100.into()).send().await?;

        // Now we can fetch the balance of the wallet.
        let model = RawDataModel::<AlloyAddress, AlloyU256>::new();

        let converted_address = from_ethers_address(sender);
        let converted_token_address = from_ethers_address(token.address());

        let balance = model
            .fetch_balance_of(
                client.provider().clone().into(),
                converted_address,
                converted_token_address,
            )
            .await?;

        println!("Balance: {}", balance);

        Ok(())
    }
}
