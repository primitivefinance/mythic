//! A client for setting a local development environment.

use std::sync::Arc;

use anyhow::Result;
use arbiter_bindings::bindings::liquid_exchange::LiquidExchange;
use bindings::{log_normal, mock_erc20::MockERC20};

use super::{protocol::ProtocolClient, *};

pub const INITIAL_X_BALANCE: f64 = 100.0;
pub const INITIAL_Y_BALANCE: f64 = 100.0;
pub const INITIAL_PRICE: f64 = 1.0;

#[derive(Debug, Clone)]
pub struct DevClient<C> {
    pub protocol: ProtocolClient<C>,
    pub strategy: log_normal::LogNormal<C>,
    pub liquid_exchange: LiquidExchange<C>,
    pub token_x: MockERC20<C>,
    pub token_y: MockERC20<C>,
}

impl<C: Middleware + 'static> DevClient<C> {
    pub fn client(&self) -> Arc<C> {
        self.protocol.client.clone()
    }

    pub async fn balance_of_x(&self, address: Address) -> Result<U256> {
        Ok(self.token_x.balance_of(address).call().await?)
    }

    pub async fn balance_of_y(&self, address: Address) -> Result<U256> {
        Ok(self.token_y.balance_of(address).call().await?)
    }

    #[tracing::instrument(skip(client), level = "trace")]
    pub async fn deploy(client: Arc<C>, sender: Address) -> Result<Self> {
        tracing::trace!("Deploying token x");
        let token_x_args = ("Token X".to_string(), "X".to_string(), 18_u8);
        let token_x = MockERC20::deploy(client.clone(), token_x_args)?
            .send()
            .await?;

        tracing::trace!("Deploying token y");
        let token_y_args = ("Token Y".to_string(), "Y".to_string(), 18_u8);
        let token_y = MockERC20::deploy(client.clone(), token_y_args)?
            .send()
            .await?;

        // Mint an initial portfolio of 50/50.
        let initial_portfolio = 0.5;
        let initial_portfolio_wad = ethers::utils::parse_ether(initial_portfolio).unwrap();

        token_x
            .mint(sender, ethers::utils::parse_ether(INITIAL_X_BALANCE)?)
            .send()
            .await?;
        token_y
            .mint(sender, ethers::utils::parse_ether(INITIAL_Y_BALANCE)?)
            .send()
            .await?;

        let swap_fee_percent_wad = 0.003;

        tracing::trace!("Deploying protocol");
        let protocol = ProtocolClient::deploy_protocol(
            client.clone(),
            token_x.address(),
            token_y.address(),
            swap_fee_percent_wad,
        )
        .await?;

        tracing::trace!("Approving tokens");
        token_x
            .approve(protocol.protocol.address(), ethers::types::U256::MAX)
            .send()
            .await?;
        token_y
            .approve(protocol.protocol.address(), ethers::types::U256::MAX)
            .send()
            .await?;

        let lex_args = (
            token_x.address(),
            token_y.address(),
            ethers::utils::parse_ether(INITIAL_PRICE)?,
        );

        let liquid_exchange = LiquidExchange::deploy(client.clone(), lex_args)?
            .send()
            .await?;

        let strategy = log_normal::LogNormal::new(protocol.protocol.source().call().await?, client);

        // Make sure to set the token y price to 1.0.

        Ok(Self {
            protocol,
            token_x,
            token_y,
            strategy,
            liquid_exchange,
        })
    }

    pub async fn get_strategy(&self) -> Result<log_normal::LogNormal<C>> {
        self.protocol.get_strategy().await
    }

    #[tracing::instrument(skip(self), level = "trace", ret)]
    pub async fn create_position(
        &self,
        sender: Address,
        amount_dollars: f64,
        price: f64,
        strike_price_wad: f64,
        sigma_percent_wad: f64,
        tau_years_wad: f64,
    ) -> Result<Option<TransactionReceipt>> {
        let amount_x = amount_dollars / price;
        let amount_y = amount_x * price;
        let amount_x_wad = ethers::utils::parse_ether(amount_x).unwrap();
        let amount_y_wad = ethers::utils::parse_ether(amount_y).unwrap();

        self.token_x.mint(sender, amount_x_wad).send().await?;
        self.token_y.mint(sender, amount_y_wad).send().await?;

        Ok(self
            .protocol
            .initialize(
                price,
                amount_x,
                strike_price_wad,
                sigma_percent_wad,
                tau_years_wad,
            )
            .await?)
    }

    pub async fn get_position(&self) -> Result<ProtocolPosition> {
        let (balance_x, balance_y, liquidity) = self.protocol.get_reserves_and_liquidity().await?;
        let internal_price = self.protocol.get_internal_price().await?;
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ProtocolPosition {
    pub balance_x: Option<String>,
    pub balance_y: Option<String>,
    pub liquidity: Option<String>,
    pub internal_price: Option<String>,
}
