//! Widget for deploying a smart contract.

use crate::sdk::vault::*;
use ethers::prelude::*;
use iced::{
    alignment::{self},
    widget::{button, column, text},
    Element, Length,
};
use thiserror::Error;
use tracing::info;

#[derive(Debug, Clone)]
pub enum DeployerToAppMessage {
    TriggerDeploy,
    Deployed(Result<Vault, DeployerError>),
}

#[derive(Debug, Error, Clone)]
pub enum DeployerError {
    #[error("API Error")]
    ProviderError(#[from] &'static ethers::providers::ProviderError),
}

#[derive(Debug, Clone)]
pub enum AppToDeployerMessage {
    Deploy,
    DeploySuccess(Result<Vault, DeployerError>),
}

#[derive(Debug, Clone)]
pub struct DeployerComponent {
    pub address: Address,
}

impl DeployerComponent {
    pub fn new() -> Self {
        Self {
            address: Address::zero(),
        }
    }

    pub fn update(&mut self, message: AppToDeployerMessage) -> Option<DeployerToAppMessage> {
        match message {
            AppToDeployerMessage::Deploy => Some(DeployerToAppMessage::TriggerDeploy),
            AppToDeployerMessage::DeploySuccess(result) => {
                info!("Deployed: {:?}", result);

                self.address = match result {
                    Ok(vault) => vault.clone().address,
                    Err(error) => {
                        return Some(DeployerToAppMessage::Deployed(Err(error)));
                    }
                };

                None
            }
        }
    }

    pub fn view<'a>(&self) -> Element<'a, AppToDeployerMessage> {
        let mut content = column![];

        let button = |label, on_press| {
            button(
                text(label)
                    .width(Length::Fill)
                    .horizontal_alignment(alignment::Horizontal::Center),
            )
            .on_press(on_press)
        };

        content = content.push(button("Deploy", AppToDeployerMessage::Deploy));

        // Push the address to content if `self.address` is not the zero address (Address::zero()).
        if self.address != Address::zero() {
            content = content.push(text(format!("Address: {}", self.address)));
        }

        content.into()
    }
}
