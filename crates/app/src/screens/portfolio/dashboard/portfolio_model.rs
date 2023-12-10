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
// todo: remove this in favor of alloy types when possible.
use bindings::{dfmm::DFMM, log_normal::LogNormal};
use ethers::types::transaction::eip2718::TypedTransaction;
use sim::{from_ethers_u256, to_ethers_address};

use super::*;

type EthersAddress = ethers::types::Address;
type EthersU256 = ethers::types::U256;
type AlloyAddress = alloy_primitives::Address;
type AlloyU256 = alloy_primitives::U256;

type Client = Provider<Ws>;

/// The model!
#[derive(Debug, Clone, Default)]
pub struct RawDataModel<A, V> {
    // Must set these addresses.
    pub user_address: Option<A>,
    pub raw_protocol_address: Option<A>,
    pub raw_strategy_address: Option<A>,
    pub raw_asset_token: Option<A>,
    pub raw_quote_token: Option<A>,

    // Balances of tokens.
    pub raw_user_asset_balance: Option<V>,
    pub raw_user_quote_balance: Option<V>,
    pub raw_protocol_asset_balance: Option<V>,
    pub raw_protocol_quote_balance: Option<V>,

    // Protocol state
    pub raw_asset_reserve: Option<V>,
    pub raw_quote_reserve: Option<V>,
    pub raw_total_liquidity: Option<V>,
    pub raw_user_total_liquidity: Option<V>,
    pub raw_internal_spot_price: Option<V>,
    pub raw_external_spot_price: Option<V>,

    // Strategy state
    pub raw_strike_price_wad: Option<V>,
    pub raw_time_remaining_wad: Option<V>,
    pub raw_volatility_wad: Option<V>,
    pub raw_portfolio_values_series: Option<Vec<(u64, V)>>,

    // Info
    pub raw_last_chain_data_sync_timestamp: Option<DateTime<Utc>>,
    pub raw_last_chain_data_sync_block: Option<u64>,
}

sol! {
    #[derive(Debug)]
    interface IERC20 {
        function balanceOf(address account) external view returns(uint balance);
    }
}

// // todo: use this in the future.
// sol! {
// interface Strategy {
// function getParams() external view returns(uint strikePrice, uint volatility,
// uint timeRemaining); }
// }
//
// todo: use in the future.
// sol! {
// interface Protocol {
// function getReservesAndLiquidity() external view returns(uint reserveX, uint
// reserveY, uint liquidity); function getInternalPrice() external view
// returns(uint internalPrice); function balanceOf(address account) external
// view returns(uint liquidity); }
// }

impl RawDataModel<AlloyAddress, AlloyU256> {
    pub type Address = AlloyAddress;
    pub type Value = AlloyU256;

    pub fn new() -> Self {
        Self::default()
    }

    pub async fn protocol(&self, client: Arc<Client>) -> Result<DFMM<Client>> {
        let protocol_address = self
            .raw_protocol_address
            .ok_or(Error::msg("Protocol address not set"))?;
        let converted_address = to_ethers_address(protocol_address);
        let protocol = DFMM::new(converted_address, client.clone());
        Ok(protocol)
    }

    pub async fn strategy(&self, client: Arc<Client>) -> Result<LogNormal<Client>> {
        let strategy_address = self
            .raw_strategy_address
            .ok_or(Error::msg("Strategy address not set"))?;
        let converted_address = to_ethers_address(strategy_address);
        let strategy = LogNormal::new(converted_address, client.clone());
        Ok(strategy)
    }

    pub async fn fetch_balance(client: Arc<Client>, address: Self::Address) -> Result<Self::Value> {
        let converted_address = to_ethers_address(address);
        let balance = client.get_balance(converted_address, None).await?;
        let converted_balance = from_ethers_u256(balance);
        Ok(converted_balance)
    }

    #[tracing::instrument(skip(client), level = "trace", ret, err)]
    pub async fn fetch_balance_of(
        &self,
        client: Arc<Client>,
        token_address: Self::Address,
        address: Self::Address,
    ) -> Result<Self::Value> {
        let converted_token_address = to_ethers_address(token_address);

        let payload = IERC20::balanceOfCall { account: address };
        let payload = ethers::types::Bytes::from(payload.abi_encode());

        let mut tx = TypedTransaction::default();
        tx.set_to(converted_token_address).set_data(payload);

        // Send the call to the token contract.
        println!("Calling token contract: {:?}", tx);
        let balance = client.call(&tx, None).await?;
        println!("Balance: {:?}", balance);
        let decoded: <IERC20::balanceOfCall as SolCall>::Return =
            IERC20::balanceOfCall::abi_decode_returns(&balance, false)?;
        println!("Decoded call: {:?}", decoded);
        let decoded_balance: Self::Value = decoded.balance.into();

        Ok(decoded_balance)
    }

    // Token balances

    async fn fetch_user_asset_balance(
        &self,
        client: Arc<Client>,
        address: Self::Address,
    ) -> Result<Self::Value> {
        let asset_token = self
            .raw_asset_token
            .ok_or(Error::msg("Asset token not set"))?;
        self.fetch_balance_of(client, asset_token, address).await
    }

    async fn fetch_user_quote_balance(
        &self,
        client: Arc<Client>,
        address: Self::Address,
    ) -> Result<Self::Value> {
        let quote_token = self
            .raw_quote_token
            .ok_or(Error::msg("Quote token not set"))?;
        self.fetch_balance_of(client, quote_token, address).await
    }

    async fn fetch_protocol_asset_balance(&self, client: Arc<Client>) -> Result<Self::Value> {
        let asset_token = self
            .raw_asset_token
            .ok_or(Error::msg("Asset token not set"))?;
        let protocol = self
            .raw_protocol_address
            .ok_or(Error::msg("Protocol address not set"))?;
        self.fetch_balance_of(client, asset_token, protocol).await
    }

    async fn fetch_protocol_quote_balance(&self, client: Arc<Client>) -> Result<Self::Value> {
        let quote_token = self
            .raw_quote_token
            .ok_or(Error::msg("Quote token not set"))?;
        let protocol = self
            .raw_protocol_address
            .ok_or(Error::msg("Protocol address not set"))?;
        self.fetch_balance_of(client, quote_token, protocol).await
    }

    // Protocol state

    async fn fetch_reserves_and_liquidity(
        &self,
        client: Arc<Client>,
    ) -> Result<(Self::Value, Self::Value, Self::Value)> {
        let protocol = self
            .raw_protocol_address
            .ok_or(Error::msg("Protocol address not set"))?;
        let protocol = self.protocol(client.clone()).await?;
        let (reserve_x, reserve_y, liquidity) = protocol.get_reserves_and_liquidity().await?;
        let reserve_x = from_ethers_u256(reserve_x);
        let reserve_y = from_ethers_u256(reserve_y);
        let liquidity = from_ethers_u256(liquidity);
        Ok((reserve_x, reserve_y, liquidity))
    }

    async fn fetch_internal_price(&self, client: Arc<Client>) -> Result<Self::Value> {
        let protocol = self
            .raw_protocol_address
            .ok_or(Error::msg("Protocol address not set"))?;
        let protocol = self.protocol(client.clone()).await?;
        let internal_price = protocol.internal_price().await?;
        let internal_price = from_ethers_u256(internal_price);
        Ok(internal_price)
    }

    async fn fetch_strategy_params(
        &self,
        client: Arc<Client>,
    ) -> Result<(Self::Value, Self::Value, Self::Value)> {
        let strategy = self
            .raw_strategy_address
            .ok_or(Error::msg("Strategy address not set"))?;
        let strategy = self.strategy(client.clone()).await?;
        let (strike_price, volatility, time_remaining) = strategy.get_params().await?;
        let strike_price = from_ethers_u256(strike_price);
        let volatility = from_ethers_u256(volatility);
        let time_remaining = from_ethers_u256(time_remaining);
        Ok((strike_price, volatility, time_remaining))
    }

    async fn update_protocol_state(&mut self, client: Arc<Client>) -> Result<()> {
        let (reserve_x, reserve_y, liquidity) =
            self.fetch_reserves_and_liquidity(client.clone()).await?;
        let internal_price = self.fetch_internal_price(client.clone()).await?;

        self.raw_asset_reserve = Some(reserve_x);
        self.raw_quote_reserve = Some(reserve_y);
        self.raw_total_liquidity = Some(liquidity);
        self.raw_internal_spot_price = Some(internal_price);

        Ok(())
    }

    async fn update_token_balances(&mut self, client: Arc<Client>) -> Result<()> {
        let user_address = self
            .user_address
            .ok_or(Error::msg("User address not set"))?;
        let user_asset_balance = self
            .fetch_user_asset_balance(client.clone(), user_address)
            .await?;
        let user_quote_balance = self
            .fetch_user_quote_balance(client.clone(), user_address)
            .await?;
        let protocol_asset_balance = self.fetch_protocol_asset_balance(client.clone()).await?;
        let protocol_quote_balance = self.fetch_protocol_quote_balance(client.clone()).await?;

        self.raw_user_asset_balance = Some(user_asset_balance);
        self.raw_user_quote_balance = Some(user_quote_balance);
        self.raw_protocol_asset_balance = Some(protocol_asset_balance);
        self.raw_protocol_quote_balance = Some(protocol_quote_balance);

        Ok(())
    }

    async fn update_strategy_state(&mut self, client: Arc<Client>) -> Result<()> {
        let (strike_price, volatility, time_remaining) =
            self.fetch_strategy_params(client.clone()).await?;

        self.raw_strike_price_wad = Some(strike_price);
        self.raw_volatility_wad = Some(volatility);
        self.raw_time_remaining_wad = Some(time_remaining);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use bindings::mock_erc20::MockERC20;
    use ethers::{
        prelude::*,
        utils::{Anvil, AnvilInstance},
    };
    use sim::from_ethers_address;

    use super::{AlloyAddress, AlloyU256, *};

    async fn setup() -> anyhow::Result<(
        AnvilInstance,
        Arc<SignerMiddleware<Provider<Ws>, LocalWallet>>,
    )> {
        let anvil = Anvil::default()
            .arg("--gas-limit")
            .arg("20000000")
            .chain_id(31337_u64)
            .spawn();

        let url = anvil.ws_endpoint().to_string();
        let wallet: LocalWallet = anvil.keys().get(0).unwrap().clone().into();
        let wallet = wallet.with_chain_id(anvil.chain_id());
        println!("Connected to URL: {}", url);

        println!("Wallet address: {}", wallet.address());
        let provider = Provider::<Ws>::connect(&url).await?;
        let client = Arc::new(SignerMiddleware::new(provider, wallet));

        Ok((anvil, client))
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

    #[test]
    fn test_decode_returns_bug() {
        let data = alloy_primitives::Bytes::from(vec![0_u8]);
        let decoded = IERC20::balanceOfCall::abi_decode_returns(&data, false);
        println!("Decoded: {:?}", decoded);
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_fetch_balance_of() -> anyhow::Result<()> {
        let (anvil, client) = setup().await?;

        // Need to deploy a token and mint some to wallet!
        let token =
            MockERC20::deploy(client.clone(), ("Test".to_string(), "T".to_string(), 18_u8))?
                .send()
                .await?;

        println!("Deployed token: {:?}", token.address());

        let sender = client.address();

        token.mint(sender, 100.into()).send().await?;
        println!("Minted tokens");

        // Now we can fetch the balance of the wallet.
        let mut model = RawDataModel::<AlloyAddress, AlloyU256>::new();

        let converted_address = from_ethers_address(sender);
        let converted_token_address = from_ethers_address(token.address());

        println!("Fetching balance of: {}", converted_address);
        let balance = model
            .fetch_balance_of(
                client.provider().clone().into(),
                converted_token_address,
                converted_address,
            )
            .await?;

        println!("Balance: {}", balance);

        Ok(())
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_update_token_balances() -> anyhow::Result<()> {
        let (anvil, client) = setup().await?;

        // Need to deploy a token and mint some to wallet!
        let token =
            MockERC20::deploy(client.clone(), ("Test".to_string(), "T".to_string(), 18_u8))?
                .send()
                .await?;

        println!("Deployed token: {:?}", token.address());

        let sender = client.address();

        token.mint(sender, 100.into()).send().await?;
        println!("Minted tokens");

        // Now we can fetch the balance of the wallet.
        let mut model = RawDataModel::<AlloyAddress, AlloyU256>::new();

        let converted_address = from_ethers_address(sender);
        let converted_token_address = from_ethers_address(token.address());

        println!("Fetching balance of: {}", converted_address);
        let balance = model
            .fetch_balance_of(
                client.provider().clone().into(),
                converted_token_address,
                converted_address,
            )
            .await?;

        println!("Balance: {}", balance);

        // Update the model.
        let converted_user_address = from_ethers_address(sender);
        let converted_token_address = from_ethers_address(token.address());
        model.user_address = Some(converted_user_address);
        model.raw_asset_token = Some(converted_token_address);
        model.raw_quote_token = Some(converted_token_address);
        model.raw_protocol_address = Some(AlloyAddress::ZERO);
        model.raw_strategy_address = Some(AlloyAddress::ZERO);

        model
            .update_token_balances(client.provider().clone().into())
            .await?;

        // Log the new balance.
        println!("User asset balance: {:?}", model.raw_user_asset_balance);

        assert_eq!(model.raw_user_asset_balance.unwrap(), balance);

        Ok(())
    }
}
