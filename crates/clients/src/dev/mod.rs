//! A client for setting a local development environment.

use std::sync::Arc;

use anyhow::Result;
use bindings::{log_normal, mock_erc20::MockERC20};

use super::{protocol::ProtocolClient, *};

#[derive(Debug, Clone)]
pub struct DevClient<C> {
    pub protocol: ProtocolClient<C>,
    pub token_x: MockERC20<C>,
    pub token_y: MockERC20<C>,
}

impl<C: Middleware + 'static> DevClient<C> {
    pub fn client(&self) -> Arc<C> {
        self.protocol.client.clone()
    }

    #[tracing::instrument(skip(client), level = "trace")]
    pub async fn deploy(client: Arc<C>) -> Result<Self> {
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

        let swap_fee_percent_wad = 0.003;

        tracing::trace!("Deploying protocol");
        let protocol = ProtocolClient::deploy_protocol(
            client,
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

        Ok(Self {
            protocol,
            token_x,
            token_y,
        })
    }

    pub async fn get_strategy(&self) -> Result<log_normal::LogNormal<C>> {
        self.protocol.get_strategy().await
    }

    #[tracing::instrument(skip(self), level = "trace", ret)]
    pub async fn create_position(
        &self,
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

        self.token_x
            .mint(self.protocol.protocol.address(), amount_x_wad)
            .send()
            .await?;
        self.token_y
            .mint(self.protocol.protocol.address(), amount_y_wad)
            .send()
            .await?;

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
}
