//! # Simulation -> Middleware -> Smart Contracts
//!
//! Smart contracts have a very specific api that is always forced to be the most secure.
//! This makes the API sometimes very clunky or cumbersome to use, it also requires many downstream changes when its altered.
//! Middleware is a layer that sits between the simulation and the smart contracts, so the API
//! can be more developer friendly and changed easily without sacrificing security in the smart contracts.
//!
//! For example, the smart contracts sometimes require off-chain data to be passed to the function.
//! This can handled by the middleware, so the developer calling the smart contract's function
//! does not need to figure out how to get that data and pass it through.
//! E.g. slippage tolerance, minimum output amounts for trades, etc.
//!
//! # Full Example
//!
//! SmartContract.sol
//! ```solidity
//! function rebalance(bool swapDirection, uint amountIn, uint minAmountOut, address recipient) external returns(uint amountOut);
//! ```
//!
//! Config.toml
//! ```toml
//! [middleware]
//! slippage_tolerance = 0.01
//! ```
//!
//! Middleware.rs
//! ```rust,ignore
//! let slippage_tolerance = config.middleware.slippage_tolerance;
//!
//! fn rebalance(swap_direction: bool, amount: u64) -> u64 {
//!     let min_amount_out = amount * (1 - slippage_tolerance);
//!     let recipient = "0x1234";
//!     let amount_out = smart_contract.rebalance(swap_direction, amount_in, min_amount_out, recipient);
//!     amount_out
//! }
//! ```
//!
//! Simulation.rs
//! ```rust,ignore
//! let amount_in = 100;
//! let amount_out = middleware.rebalance(true, amount_in);
//! ```
//!
//! As you can see, the simulation code is very clean and easy to read.
//! And the internal details are handled by the middleware, which can be configured in the config file.
//! It also makes it easier to change the smart contract's API without having to change the simulation code,
//! since it only needs to be updated in the middleware.
//!
//! Just don't go overboard with middleware & abstractions and you are good to go.

use async_trait::async_trait;
use auto_impl::auto_impl;
use ethers::prelude::*;
use std::fmt::Debug;

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[auto_impl(&, Box, Arc)]
pub trait Membrane: Sync + Send + Debug {
    /// Middleware client for the respective network/chain/arbiter.
    type Client: Middleware;

    /// Action for rebalancing the portfolio.
    /// - swap_direction: Whether to swap from the first asset to the second asset, or vice versa.
    /// - amount: Amount of the first asset to swap.
    /// - returns: Amount of the second asset received.
    /// - errors: If the swap fails, an error is returned.
    /// - notes: The middleware should handle the slippage tolerance and minimum output amount.
    async fn rebalance(
        &self,
        swap_direction: bool,
        amount: f64,
    ) -> anyhow::Result<f64, anyhow::Error>;

    /// Action for adding liquidity to the portfolio
    /// - asset_1: First asset to add liquidity for.
    /// - asset_2: Second asset to add liquidity for.
    /// - amount_1: Amount of first asset to add liquidity for.
    /// - amount_2: Amount of second asset to add liquidity for.
    /// - returns: Amount of liquidity tokens received.
    async fn add_liquidity(
        &self,
        asset_1: &str,
        asset_2: &str,
        amount_1: f64,
        amount_2: f64,
    ) -> anyhow::Result<f64, anyhow::Error>;

    /// Action for removing liquidity from the portfolio.
    /// - asset_1: First asset to remove liquidity for.
    /// - asset_2: Second asset to remove liquidity for.
    /// - amount: Amount of liquidity tokens to remove.
    /// - returns: Amount of first asset received.
    /// - returns: Amount of second asset received.
    async fn remove_liquidity(
        &self,
        asset_1: &str,
        asset_2: &str,
        amount: f64,
    ) -> anyhow::Result<(f64, f64), anyhow::Error>;

    /// Action for updating the portfolio's weight composition.
    /// Setting the weight for asset will implicitly set the weight for asset_2,
    /// because the sum of the weights must equal 1.
    /// - asset: Asset to update weight of in portfolio.
    /// - weight: Weight of asset in portfolio.
    /// - returns: Both asset identifiers and their respective weights
    async fn update_weights(
        &self,
        asset: &str,
        weight: f64,
    ) -> anyhow::Result<(&str, f64, &str, f64), anyhow::Error>;

    /// Fetches the current price of the primary asset.
    async fn get_spot_price(&self) -> anyhow::Result<f64, anyhow::Error>;

    /// Fetches the weights of the assets in the portfolio.
    /// - returns: Primary (base) asset weight and secondary (quote) asset weight.
    async fn get_weights(&self) -> anyhow::Result<(f64, f64), anyhow::Error>;
}

// #[cfg(test)]
// mod tests {

//     use super::*;
//     use anyhow::anyhow;
//     use bindings::counter::{Counter, COUNTER_ABI, COUNTER_BYTECODE};
//     use ethers::{prelude::*, types::transaction::eip2718::TypedTransaction};
//     use std::sync::Arc;

//     /// Mock membrane has two fields:
//     /// - client - The middleware client.
//     /// - contract - The Counter smart contract instance. Mutable so it can be overridden after deployed.
//     /// M type must be restricted to the middleware client type.
//     #[derive(Debug, Clone)]
//     pub struct MockMembrane<M>
//     where
//         M: Middleware + Clone + Send + Sync + 'static,
//     {
//         client: Arc<M>,
//         contract: Counter<M>,
//     }

//     impl<M> MockMembrane<M>
//     where
//         M: Middleware + Clone + Send + Sync + 'static,
//     {
//         /// Creates a new mock membrane.
//         pub fn new(client: Arc<M>) -> Self {
//             let contract = Counter::new(ethers::types::Address::zero(), client.clone());
//             Self { client, contract }
//         }

//         /// Deploys the smart contract to the network.
//         /// - returns: The address of the deployed smart contract.
//         /// - errors: If the deployment fails, an error is returned.
//         /// - notes: This is only used for testing.
//         pub async fn deploy(mut self) -> Self {
//             let contract = Counter::deploy(self.client.clone(), ())
//                 .unwrap()
//                 .send()
//                 .await
//                 .unwrap();

//             // Override contract to use the address.
//             self.contract = contract;

//             self
//         }
//     }

//     /// Implements the Membrane trait for the MockMembrane.
//     /// This is where the middleware client is used to call the smart contract.
//     /// The middleware methods act as the standardized api.
//     ///
//     /// Two methods are implemented that call it's Mock "contract" (Counter smart contract):
//     /// - rebalance() - Calls "increment()": Increments the counter by 1.
//     /// - get_spot_price() - Calls "number()": Fetches the current count.
//     #[async_trait]
//     impl Membrane for MockMembrane<SignerMiddleware<Provider<Http>, LocalWallet>> {
//         type Client = Arc<SignerMiddleware<Provider<Http>, LocalWallet>>;
//         async fn rebalance(
//             &self,
//             _swap_direction: bool,
//             _amount: f64,
//         ) -> anyhow::Result<f64, anyhow::Error> {
//             // Increments the counter by 1.
//             let res = self.contract.increment().send().await?.await?;

//             // Returns "1.0" if success, errors otherwise.
//             let outcome = match res {
//                 Some(_) => Ok(1.0),
//                 None => Err(anyhow!("No result returned from contract call.")),
//             }?;

//             Ok(outcome)
//         }

//         async fn add_liquidity(
//             &self,
//             asset_1: &str,
//             asset_2: &str,
//             amount_1: f64,
//             amount_2: f64,
//         ) -> anyhow::Result<f64, anyhow::Error> {
//             Ok(0.0)
//         }

//         async fn remove_liquidity(
//             &self,
//             asset_1: &str,
//             asset_2: &str,
//             amount: f64,
//         ) -> anyhow::Result<(f64, f64), anyhow::Error> {
//             Ok((0.0, 0.0))
//         }

//         async fn update_weights(
//             &self,
//             asset: &str,
//             weight: f64,
//         ) -> anyhow::Result<(&str, f64, &str, f64), anyhow::Error> {
//             Ok(("", 0.0, "", 0.0))
//         }

//         async fn get_spot_price(&self) -> anyhow::Result<f64, anyhow::Error> {
//             let count = self.contract.clone().number().await.unwrap();
//             let count = count.as_u64() as f64;

//             Ok(count)
//         }

//         async fn get_weights(&self) -> anyhow::Result<(f64, f64), anyhow::Error> {
//             Ok((0.0, 0.0))
//         }
//     }

//     #[tokio::test]
//     async fn test_counter() {
//         // 1. Create an anvil instance.
//         let anvil = ethers::core::utils::Anvil::new().spawn();

//         // 2. Get a wallet using the anvil instance developer wallets.
//         let wallet: LocalWallet = anvil.keys()[0].clone().into();

//         // 3. Connect to the network
//         let provider = Provider::<Http>::try_from(anvil.endpoint())
//             .unwrap()
//             .interval(std::time::Duration::from_millis(10u64));

//         // 4. Instantiate the client with the wallet
//         let client = SignerMiddleware::new(provider, wallet.with_chain_id(anvil.chain_id()));
//         let client = Arc::new(client);

//         // 5. Instantiate the membrane implementation with the client.
//         let mut membrane = MockMembrane::new(client.clone());

//         // 6. Deploy the smart contract of the membrane implementation using the client.
//         println!(
//             "membrane contract address: {:?}",
//             membrane.contract.clone().address()
//         );
//         membrane = membrane.deploy().await;
//         println!(
//             "membrane contract address: {:?}",
//             membrane.contract.clone().address()
//         );

//         // 7. Check the initial count is zero.
//         let count = membrane.get_spot_price().await.unwrap();
//         assert_eq!(count, 0.0);
//         println!("count: {}", count);

//         // 8. Calls rebalance(), which implements the increment() call.
//         let is_success = membrane.rebalance(true, 0.0).await.unwrap();
//         assert_eq!(is_success, 1.0); // Returns 1.0 if success, just to make things easy for us.

//         // 9. Check the count is now one.
//         let count = membrane.get_spot_price().await.unwrap();
//         assert_eq!(count, 1.0);
//         println!("count: {}", count);
//     }

//     async fn test_counter_arbiter() {
//         todo!("Test the arbiter membrane implementation.")
//     }
// }
