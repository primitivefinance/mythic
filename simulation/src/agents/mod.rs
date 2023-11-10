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
pub mod counter;
pub mod liquidity_provider;
pub mod price_changer;
pub mod swapper;
pub mod token_admin;
pub mod weight_changer;

use std::marker::{Send, Sync};

pub struct Agents(pub Vec<Box<dyn Agent>>);

impl Agents {
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Box<dyn Agent>> {
        self.0.iter_mut()
    }

    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self(vec![])
    }

    /// Modifies the agents in place.
    /// dev: We can't consume this object and return Self, because we might want to mutate it later.
    /// so for now we just push and don't return anything.
    pub fn add(&mut self, agent: impl Agent + 'static) {
        self.0.push(Box::new(agent));
    }
}

/// Universal agent methods for interacting with the simulation environment or
/// loop.
#[async_trait::async_trait]
pub trait Agent: Sync + Send {
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
}

#[async_trait::async_trait]
impl Agent for Agents {
    async fn step(&mut self) -> Result<()> {
        Ok(())
    }

    async fn priority_step(&mut self) -> Result<()> {
        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
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
