use self::{
    block_admin::BlockAdminParameters,
    liquidity_provider::LiquidityProviderParameters,
    price_changer::PriceChangerParameters,
    swapper::SwapperParameters,
    token_admin::TokenAdminParameters,
    weight_changer::{WeightChangerParameters, WeightChangerSpecialty},
};
use super::*;
use crate::settings::{
    parameters::{LinspaceParameters, Multiple, Single},
    Parameterized,
};

pub mod arbitrageur;
pub mod block_admin;
pub mod liquidity_provider;
pub mod price_changer;
pub mod swapper;
#[cfg(test)]
pub mod tests;
pub mod token_admin;
pub mod weight_changer;

use std::marker::{Send, Sync};

use linked_hash_map::LinkedHashMap;

#[derive(Debug)]
pub struct Agents(pub LinkedHashMap<String, Box<dyn Agent>>);

impl Agents {
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&String, &mut Box<dyn Agent>)> {
        self.0.iter_mut()
    }

    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self(LinkedHashMap::new())
    }

    pub fn add(&mut self, agent: impl Agent + 'static) {
        self.0.insert(
            agent.client().label.as_ref().unwrap().clone(),
            Box::new(agent),
        );
    }
}

/// Universal agent methods for interacting with the simulation environment or
/// loop.3
#[async_trait::async_trait]
pub trait Agent: Sync + Send + std::fmt::Debug {
    /// Executed outside the main simulation loop.
    async fn startup(&mut self) -> Result<()> {
        Ok(())
    }

    /// Executed by each agent inside the main simulation loop.
    /// Ordering is determined by placement in the simulation loop.
    async fn step(&mut self) -> Result<()> {
        Ok(())
    }

    /// Executed by each agent in a separate loop before the main loop.
    async fn priority_step(&mut self) -> Result<()> {
        Ok(())
    }

    /// In order to be able to track agents by their label, each agent must
    /// implement a label method.
    fn client(&self) -> Arc<RevmMiddleware>;
}

#[derive(Clone, Debug, Serialize, Deserialize, Hash)]
pub enum AgentParameters<P: Parameterized> {
    WeightChanger(WeightChangerParameters<P>),
    Swapper(SwapperParameters<P>),
    LiquidityProvider(LiquidityProviderParameters<P>),
    BlockAdmin(BlockAdminParameters),
    TokenAdmin(TokenAdminParameters),
    PriceChanger(PriceChangerParameters<P>),
}

impl From<AgentParameters<Multiple>> for Vec<AgentParameters<Single>> {
    fn from(item: AgentParameters<Multiple>) -> Self {
        match item {
            AgentParameters::WeightChanger(parameters) => {
                let parameters: Vec<WeightChangerParameters<Single>> = parameters.into();
                parameters
                    .into_iter()
                    .map(AgentParameters::WeightChanger)
                    .collect()
            }
            AgentParameters::Swapper(parameters) => {
                let parameters: Vec<SwapperParameters<Single>> = parameters.into();
                parameters
                    .into_iter()
                    .map(AgentParameters::Swapper)
                    .collect()
            }
            AgentParameters::LiquidityProvider(parameters) => {
                let parameters: Vec<LiquidityProviderParameters<Single>> = parameters.into();
                parameters
                    .into_iter()
                    .map(AgentParameters::LiquidityProvider)
                    .collect()
            }
            AgentParameters::PriceChanger(parameters) => {
                let parameters: Vec<PriceChangerParameters<Single>> = parameters.into();
                parameters
                    .into_iter()
                    .map(AgentParameters::PriceChanger)
                    .collect()
            }
            AgentParameters::BlockAdmin(parameters) => {
                vec![AgentParameters::BlockAdmin(parameters)]
            }
            AgentParameters::TokenAdmin(parameters) => {
                vec![AgentParameters::TokenAdmin(parameters)]
            }
        }
    }
}
