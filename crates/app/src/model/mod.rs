//! Aggregated model for the application's entire data system.

pub mod contacts;
pub mod portfolio;
pub mod rpcs;
pub mod user;

use std::{collections::HashMap, fs::File};

use alloy_primitives::ChainId;
use datatypes::portfolio::{
    coin::Coin,
    position::{Position, PositionLayer, Positions},
    weight::Weight,
};
use uuid::Uuid;

use self::{
    portfolio::{AlloyAddress, AlloyU256, RawDataModel},
    user::{Saveable, UserProfile},
};
use super::*;
use crate::model::portfolio::StrategyPosition;

pub const COIN_X: &str = r#"{
    "symbol": "X",
    "name": "X",
    "decimals": 18,
    "tags": [],
    "chain_id": 31337,
    "address": "0x5fbdb2315678afecb367f032d93f642f64180aa3",
    "logo_uri": ""
}"#;

pub const COIN_Y: &str = r#"{
    "symbol": "Y",
    "name": "Y",
    "decimals": 18,
    "tags": [],
    "chain_id": 31337,
    "address": "0x5fbdb2315678afecb367f032d93f642f64180aa4",
    "logo_uri": ""
}"#;

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct Model {
    pub networks: HashMap<ChainId, RawDataModel<AlloyAddress, AlloyU256>>,
    pub user: UserProfile,
    pub current: Option<ChainId>,
}

impl Model {
    pub fn new(user: UserProfile) -> Self {
        Self {
            user,
            networks: HashMap::new(),
            current: None,
        }
    }

    /// Gets the model of the currently connected network, connected via
    /// `connect_to_network`.
    pub fn get_current(&self) -> Option<&RawDataModel<AlloyAddress, AlloyU256>> {
        self.current
            .and_then(|chain_id| self.networks.get(&chain_id))
    }

    /// Gets the mutable model of the currently connected network, connected via
    /// `connect_to_network`.
    pub fn get_current_mut(&mut self) -> Option<&mut RawDataModel<AlloyAddress, AlloyU256>> {
        self.current
            .and_then(move |chain_id| self.networks.get_mut(&chain_id))
    }

    /// Sets the current network to the given chain id and instantiates a new
    /// data model for it.
    pub async fn connect_to_network<M: Middleware + 'static>(
        &mut self,
        client: Arc<M>,
    ) -> anyhow::Result<()> {
        let chain_id = client.get_chainid().await?;
        self.networks
            .entry(chain_id.as_u64())
            .or_insert_with(|| RawDataModel::new(chain_id.as_u64()));
        self.current = Some(chain_id.as_u64());
        Ok(())
    }

    pub async fn update<M: Middleware + 'static>(&mut self, client: Arc<M>) -> anyhow::Result<()> {
        // 1. Fetches and updates the data model stored in `self.portfolio`.
        // 2. Fetches the now updated position info from the data model.
        // 3. Using the position info, derives the weights of the positions.
        // 4. Propagates updated position info to the user's saved portfolio data.
        self.update_data_model(client).await.map(|_| {
            if let Err(error) = self.update_portfolio_positions() {
                tracing::warn!("Failed to update portfolio positions: {:?}", error);
            }
        })
    }

    /// Fetches the balances and values of the user's tokens.
    /// - Uses the model's `token_balance_mapping` to get token balances, which
    ///   includes unallocated "raw" tokens held by the user, and allocated
    ///   positions in the form of liquidity tokens.
    /// - Uses the model's `external_prices` mapping to get the external prices
    ///   of raw tokens.
    /// - Uses the model's `derive_user_allocated_balance_usd` to get the value
    ///   of liquidity tokens.
    /// - Combines these values into a `Portfolio` of `Position`s with the
    ///   computed weights.
    ///
    /// Returns early with an `Ok` result if the model is not connected to a
    /// network.
    pub fn update_portfolio_positions(&mut self) -> anyhow::Result<()> {
        if self.current.is_none() {
            return Ok(());
        }

        let chain_id = self.current.unwrap();
        let model = self.networks.get_mut(&chain_id).unwrap();
        let coin_list = self.user.coins.clone();
        let unallocated_positions = model.get_unallocated_positions(coin_list.clone())?;
        let allocated_positions = model.get_allocated_positions()?;

        // Clones the user's current portfolio to mutate it.
        let mut portfolio = self.user.portfolio.clone();

        // Construct the portfolio by combining the value of all positions to derive the
        // weights of each individual position.
        // todo: ignore the token x and token y values of a liquidity position, and just
        // use the liquidity token as its own position.

        // Build a list of all individual positions, which are tokens, from the
        // unallocated and allocated positions.
        let position_tokens = unallocated_positions
            .iter()
            .map(|position| position.token_address.clone())
            .chain(
                allocated_positions
                    .iter()
                    .map(|position| position.token_l_address.clone()),
            )
            .collect::<Vec<AlloyAddress>>();

        // Find the total value of all of these positions by looping over each position,
        // and multiplying its balance by price.
        let total_value = unallocated_positions
            .iter()
            .map(|pos| {
                let balance = pos.balance.clone();
                let price = pos.external_price.clone();
                balance * price
            })
            .sum::<f64>()
            + allocated_positions
                .iter()
                .map(|pos| {
                    let balance = pos.liquidity_balance.clone();
                    let price = pos.liquidity_value.clone();
                    balance * price
                })
                .sum::<f64>();

        // Create the data type `Position` to be entered into the portfolio using the
        // total value to compute its weight, for each position token.
        let mut positions = vec![];
        for token in position_tokens {
            let mut is_allocated = false;
            let (balance, price) = if let Some(position) = unallocated_positions
                .iter()
                .find(|pos| pos.token_address == token)
            {
                (position.balance.clone(), position.external_price.clone())
            } else if let Some(position) = allocated_positions
                .iter()
                .find(|pos| pos.token_l_address == token)
            {
                is_allocated = true;
                (
                    position.liquidity_balance.clone(),
                    position.liquidity_value.clone() / position.liquidity_balance.clone(),
                )
            } else {
                continue;
            };

            let weight = balance * price / total_value;

            let coin = coin_list
                .tokens
                .iter()
                .find(|coin| coin.address == token)
                .cloned();

            // If coin is None, then we need to insert the coin into the coinlist using its
            // token metadata in the portfolio model.
            let coin_metadata = model
                .clone()
                .token_metadata
                .unwrap()
                .get(&token)
                .ok_or_else(|| {
                    anyhow::anyhow!("Failed to get token metadata for token: {:?}", token)
                })?
                .clone();

            // Add the coin to the coinlist.
            let coin = match coin {
                Some(coin) => coin,
                None => {
                    // If no coin is available, create a new one using its existing metadata.

                    let mut coin = Coin::new(
                        coin_metadata.symbol,
                        coin_metadata.name,
                        coin_metadata.decimals,
                    );
                    coin.address = token;
                    coin.chain_id = chain_id;
                    coin.tags = vec!["lp".to_string()];

                    // Add the coin to the coinlist of the user.
                    // todo: this is kind of hidden for a big operation and should have an
                    // intermediary function to manage the coinlist updates.
                    self.user.coins.tokens.push(coin.clone());

                    coin
                }
            };

            let layer = match is_allocated {
                true => PositionLayer::Liquidity,
                false => PositionLayer::RawBalance,
            };

            let position = Position::new(
                coin,
                Some(price),
                Some(balance),
                Some(Weight {
                    id: Uuid::new_v4(),
                    value: weight,
                }),
                None,
            )
            .layer(layer);

            positions.push(position);
        }

        // Workaround to NaN errors in portfolio position changes is to override the
        // positions directly.
        portfolio.positions = Positions::new(positions);

        self.user.update_portfolio(&portfolio)
    }

    /// Updates the currently connected model with the latest data from the
    /// connected network.
    #[tracing::instrument(skip(self, client), level = "debug")]
    pub async fn update_data_model<M: Middleware + 'static>(
        &mut self,
        client: Arc<M>,
    ) -> anyhow::Result<()> {
        tracing::info!(
            "Updating model at block: {}",
            client.get_block_number().await?
        );

        let chain_id = client.get_chainid().await?;

        // Try updating the token list.
        self.update_tracked_tokens()?;

        self.networks
            .get_mut(&chain_id.as_u64())
            .unwrap()
            .update(client)
            .await?;

        Ok(())
    }

    /// Syncs the user's token list to the data model. By adding it to the data
    /// model, the token's balance will be tracked.
    pub fn update_tracked_tokens(&mut self) -> anyhow::Result<()> {
        // Exit early if no network is actively connected.
        if self.current.is_none() {
            return Ok(());
        }

        let coin_list = self.user.coins.clone();

        // For each coin, add it to the token balance mapping.
        for coin in coin_list.tokens.clone() {
            let coin = coin.clone();
            let chain_id = self.current.unwrap();
            let network = self.networks.get_mut(&chain_id).unwrap();
            network.add_token(coin.address)?;
        }

        Ok(())
    }
}

pub const MODEL_EXTENSION: &str = "json";
pub const MODEL_SUFFIX: &str = "user_data";

impl Saveable for Model {
    const EXTENSION: &'static str = MODEL_EXTENSION;
    const SUFFIX: &'static str = MODEL_SUFFIX;

    fn prefix(&self) -> Option<String> {
        self.user.name.clone()
    }

    /// Creates a new user save.
    fn create_new(name: Option<String>) -> anyhow::Result<Self, anyhow::Error> {
        // Check the org directory exists, if not, create it.
        if !Self::org_dir().exists() {
            println!("Creating org directory: {:?}", Self::org_dir());
            std::fs::create_dir(Self::org_dir()).expect("Failed to create org directory.");
        }

        // Check if the app directory exists, if not, create it.
        if !Self::app_dir().exists() {
            println!("Creating app directory: {:?}", Self::app_dir());
            std::fs::create_dir(Self::app_dir()).expect("Failed to create app directory.");
        }

        let user_data_file = match name.clone() {
            Some(name) => Self::file_path_with_name(name),
            None => Self::path(),
        };
        // Don't overwrite existing profiles.
        if user_data_file.exists() {
            return Self::load(Some(user_data_file));
        }

        let mut formatted_path = Self::file_name_ending();
        if let Some(name) = name.clone() {
            formatted_path = format!("{}.{}", name, formatted_path);
        }

        let profile_path = Self::dir().join(formatted_path);
        let file = File::create(profile_path)?;

        let value = Model {
            user: UserProfile::default(),
            networks: HashMap::new(),
            current: None,
        };

        serde_json::to_writer_pretty(file, &value)?;

        Ok(value)
    }
}
