//! Aggregated model for the application's entire data system.

pub mod contacts;
pub mod portfolio;
pub mod rpcs;
pub mod user;

use std::fs::File;

use datatypes::portfolio::{
    coin::Coin,
    position::{Position, Positions},
    weight::Weight,
};
use uuid::Uuid;

use self::{
    portfolio::{AlloyAddress, AlloyU256},
    user::{Saveable, UserProfile},
};
use super::*;

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
    pub portfolio: portfolio::RawDataModel<AlloyAddress, AlloyU256>,
    pub user: UserProfile,
}

impl Model {
    pub fn new(user: UserProfile) -> Self {
        Self {
            user,
            portfolio: portfolio::RawDataModel::new(),
        }
    }

    pub async fn update(&mut self, client: Arc<Provider<Ws>>) -> anyhow::Result<()> {
        // 1. Fetches and updates the data model stored in `self.portfolio`.
        // 2. Fetches the now updated position info from the data model.
        // 3. Using the position info, derives the weights of the positions.
        // 4. Propagates updated position info to the user's saved portfolio data.
        self.update_data_model(client).await.and_then(|_| {
            self.update_portfolio_positions()?;
            Ok(())
        })
    }

    pub fn update_portfolio_positions(&mut self) -> anyhow::Result<()> {
        // Gets the current position info. This should be updated prior to calling this
        // function.
        let pos_info = self.portfolio.get_position_info()?;

        // Clones the user's current portfolio to mutate it.
        let mut portfolio = self.user.portfolio.clone();

        // Based on the price of x and the balances, compute the weights of both.
        // todo: this code should be somewhere else, right?
        let total_value = pos_info.balance_x * pos_info.external_price
            + pos_info.balance_y * pos_info.quote_price;
        let position_x_weight = pos_info.balance_x * pos_info.external_price / total_value;
        let position_y_weight = pos_info.balance_y / total_value;
        let position_x_weight = Weight {
            id: Uuid::new_v4(),
            value: position_x_weight,
        };
        let position_y_weight = Weight {
            id: Uuid::new_v4(),
            value: position_y_weight,
        };

        let coin_x: Coin = serde_json::from_str(COIN_X).expect("No x token");
        let coin_y: Coin = serde_json::from_str(COIN_Y).expect("No y token");

        let position_x = Position::new(
            coin_x,
            Some(pos_info.external_price),
            Some(pos_info.balance_x),
            Some(position_x_weight),
            None,
        );

        let position_y = Position::new(
            coin_y,
            Some(pos_info.quote_price),
            Some(pos_info.balance_y),
            Some(position_y_weight),
            None,
        );

        // Workaround is to override the positions directly.
        let positions = Positions::new(vec![position_x, position_y]);
        portfolio.positions = positions;

        self.user.update_portfolio(&portfolio)
    }

    #[tracing::instrument(skip(self, client), level = "debug")]
    pub async fn update_data_model(&mut self, client: Arc<Provider<Ws>>) -> anyhow::Result<()> {
        tracing::info!(
            "Updating model at block: {}",
            client.get_block_number().await?
        );
        self.portfolio.update(client).await
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
            return Ok(Self::load(Some(user_data_file))?);
        }

        let mut formatted_path = Self::file_name_ending();
        if let Some(name) = name.clone() {
            formatted_path = format!("{}.{}", name, formatted_path);
        }

        let profile_path = Self::dir().join(formatted_path);
        let file = File::create(profile_path)?;

        let value = Model {
            user: UserProfile::default(),
            portfolio: portfolio::RawDataModel::new(),
        };

        serde_json::to_writer_pretty(file, &value)?;

        Ok(value)
    }
}