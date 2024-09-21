pub mod contacts;
pub mod portfolio;
pub mod rpcs;
pub mod user;

use std::{collections::HashMap, fs::File};
use uuid::Uuid;

use self::{
    portfolio::DataModel,
    user::{Saveable, UserProfile},
};
use super::*;

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct Model {
    pub networks: HashMap<u64, DataModel>,
    pub user: UserProfile,
    pub current: Option<u64>,
}

impl Model {
    pub fn new(user: UserProfile) -> Self {
        Self {
            user,
            networks: HashMap::new(),
            current: None,
        }
    }

    pub fn get_current(&self) -> Option<&DataModel> {
        self.current
            .and_then(|chain_id| self.networks.get(&chain_id))
    }

    pub fn get_current_mut(&mut self) -> Option<&mut DataModel> {
        self.current
            .and_then(move |chain_id| self.networks.get_mut(&chain_id))
    }

    pub async fn connect_to_network<M: Middleware + 'static>(
        &mut self,
        client: Arc<M>,
    ) -> anyhow::Result<()> {
        let chain_id = client.get_chainid().await?;
        self.networks
            .entry(chain_id.as_u64())
            .or_insert_with(|| DataModel::new(chain_id.as_u64()));
        self.current = Some(chain_id.as_u64());
        Ok(())
    }

    pub async fn update<M: Middleware + 'static>(&mut self, client: Arc<M>) -> anyhow::Result<()> {
        self.update_data_model(client).await.map(|_| {
            if let Err(error) = self.update_portfolio_positions() {
                tracing::warn!("Failed to update portfolio positions: {:?}", error);
            }
        })
    }

    pub fn update_portfolio_positions(&mut self) -> anyhow::Result<()> {
        if self.current.is_none() {
            return Ok(());
        }

        let chain_id = self.current.unwrap();
        let model = self.networks.get_mut(&chain_id).unwrap();

        Ok(())
    }

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

        self.update_tracked_tokens()?;

        self.networks
            .get_mut(&chain_id.as_u64())
            .unwrap()
            .update(client)
            .await?;

        Ok(())
    }

    pub fn update_tracked_tokens(&mut self) -> anyhow::Result<()> {
        if self.current.is_none() {
            return Ok(());
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

    fn create_new(name: Option<String>) -> anyhow::Result<Self, anyhow::Error> {
        if !Self::org_dir().exists() {
            println!("Creating org directory: {:?}", Self::org_dir());
            std::fs::create_dir(Self::org_dir()).expect("Failed to create org directory.");
        }

        if !Self::app_dir().exists() {
            println!("Creating app directory: {:?}", Self::app_dir());
            std::fs::create_dir(Self::app_dir()).expect("Failed to create app directory.");
        }

        let user_data_file = match name.clone() {
            Some(name) => Self::file_path_with_name(name),
            None => Self::path(),
        };
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
