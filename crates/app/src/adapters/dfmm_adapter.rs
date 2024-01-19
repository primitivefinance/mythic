//! Adapter for the DFMM protocol.
//! Exposes an API for interacting with the protocol that can be used by the
//! app.
use anyhow::Result;
use arbiter_bindings::bindings::arbiter_token::ArbiterToken;
use cfmm_math::trading_functions::rmm::{
    compute_value_function, compute_x_given_l_rust, compute_y_given_x_rust,
};
use clients::{
    dev::ProtocolPosition,
    protocol::{LogNormalF64, PoolInitParamsF64, ProtocolClient},
};

use self::middleware::{ExcaliburMiddleware, NetworkClient};
use super::*;

#[async_trait::async_trait]
pub trait DFMMProtocol {
    fn protocol(&self) -> Result<ProtocolClient<NetworkClient<Ws, LocalWallet>>>;

    async fn create_position(
        &self,
        sender: Address,
        amount_dollars: f64,
        price: f64,
        strike_price_wad: f64,
        sigma_percent_wad: f64,
        tau_years_wad: f64,
    ) -> anyhow::Result<Option<TransactionReceipt>>;

    async fn get_position(&self) -> anyhow::Result<ProtocolPosition>;
}

/// L = Deposit $ / V(c)
/// x = X(L)
/// y = Y(x, L)
pub fn get_deposits_given_price(
    price: f64,
    amount_dollars: f64,
    strike_price_wad: f64,
    sigma_percent_wad: f64,
    tau_years_wad: f64,
) -> (f64, f64, f64) {
    let value_per =
        compute_value_function(price, strike_price_wad, sigma_percent_wad, tau_years_wad);

    let total_liquidity = amount_dollars / value_per;

    let amount_x = compute_x_given_l_rust(
        total_liquidity,
        price,
        strike_price_wad,
        sigma_percent_wad,
        tau_years_wad,
    );

    let amount_y = compute_y_given_x_rust(
        amount_x,
        total_liquidity,
        strike_price_wad,
        sigma_percent_wad,
        tau_years_wad,
    );

    (amount_x, amount_y, total_liquidity)
}

#[async_trait::async_trait]
impl DFMMProtocol for ExcaliburMiddleware<Ws, LocalWallet> {
    fn protocol(&self) -> Result<ProtocolClient<NetworkClient<Ws, LocalWallet>>> {
        let client = self.get_client();
        let address = self.contracts.get("protocol").cloned();
        let solver = self.contracts.get("solver").cloned();
        let address = address.ok_or_else(|| anyhow::anyhow!("No protocol address"))?;
        let solver = solver.ok_or_else(|| anyhow::anyhow!("No solver address"))?;
        let protocol_client =
            ProtocolClient::from(client, address, address, address, address, address)?;
        Ok(protocol_client)
    }

    async fn create_position(
        &self,
        sender: Address,
        amount_dollars: f64,
        price: f64,
        strike_price_wad: f64,
        sigma_percent_wad: f64,
        tau_years_wad: f64,
    ) -> anyhow::Result<Option<TransactionReceipt>> {
        let client = self.get_client();
        let mut protocol = self.protocol()?;

        let (amount_x, amount_y, _total_liquidity) = get_deposits_given_price(
            price,
            amount_dollars,
            strike_price_wad,
            sigma_percent_wad,
            tau_years_wad,
        );

        let amount_x_wad = ethers::utils::parse_ether(amount_x).unwrap();
        let amount_y_wad = ethers::utils::parse_ether(amount_y).unwrap();
        let price_wad = ethers::utils::parse_ether(price).unwrap();

        // let (token_x, token_y) = protocol.get_tokens()?;
        let (token_x, token_y) = (
            ArbiterToken::new(protocol.client.address(), client.clone()),
            ArbiterToken::new(protocol.client.address(), client.clone()),
        );

        token_x.mint(sender, amount_x_wad).send().await?;
        token_y.mint(sender, amount_y_wad).send().await?;
        let init_params = PoolInitParamsF64::LogNormal(LogNormalF64 {
            sigma: sigma_percent_wad,
            strike: strike_price_wad,
            tau: tau_years_wad,
            swap_fee: 0.003,
        });

        let tx = protocol
            .init_pool(
                token_x.address(),
                token_y.address(),
                amount_x_wad,
                price_wad,
                init_params,
            )
            .await?;

        Ok(Some(tx))
    }

    async fn get_position(&self) -> anyhow::Result<ProtocolPosition> {
        let protocol = self.protocol()?;
        let (balance_x, balance_y, liquidity) = protocol
            .protocol
            .get_reserves_and_liquidity(ethers::types::U256::from(0))
            .await?;
        let internal_price = protocol
            .get_internal_price(ethers::types::U256::from(0))
            .await?;
        let balance_x = ethers::utils::format_ether(balance_x);
        let balance_y = ethers::utils::format_ether(balance_y);
        let liquidity = ethers::utils::format_ether(liquidity);
        let internal_price = ethers::utils::format_ether(internal_price);

        let balance_x = format!("{:.2}", balance_x.parse::<f64>().unwrap());
        let balance_y = format!("{:.2}", balance_y.parse::<f64>().unwrap());
        let liquidity = format!("{:.2}", liquidity.parse::<f64>().unwrap());
        let internal_price = format!("{:.2}", internal_price.parse::<f64>().unwrap());

        Ok(ProtocolPosition {
            balance_x: Some(balance_x),
            balance_y: Some(balance_y),
            liquidity: Some(liquidity),
            internal_price: Some(internal_price),
        })
    }
}
