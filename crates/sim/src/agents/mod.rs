pub mod base_agents;
pub mod portfolio_management_agents;

use base_agents::{block_admin::*, price_changer::*, token_admin::*};
use portfolio_management_agents::{
    base::parameter_manager::*, g3m::g3m_liquidity_provider::*, lognormal::ln_liquidity_provider::*,
};

use super::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AgentParameters<P: Parameterized> {
    G3mLiquidityProvider(G3mLiquidityProviderParameters<P>),
    LogNormalLiquidityProvider(LogNormalLiquidityProviderParameters<P>),
    BlockAdmin(BlockAdminParameters),
    TokenAdmin(TokenAdminParameters),
    PriceChanger(PriceChangerParameters<P>),
    ParameterManager(ParameterManagerParameters<P>),
}

impl From<AgentParameters<Multiple>> for Vec<AgentParameters<Single>> {
    fn from(item: AgentParameters<Multiple>) -> Self {
        match item {
            AgentParameters::LogNormalLiquidityProvider(parameters) => {
                let parameters: Vec<LogNormalLiquidityProviderParameters<Single>> =
                    parameters.into();
                parameters
                    .into_iter()
                    .map(AgentParameters::LogNormalLiquidityProvider)
                    .collect()
            }
            AgentParameters::G3mLiquidityProvider(parameters) => {
                let parameters: Vec<G3mLiquidityProviderParameters<Single>> = parameters.into();
                parameters
                    .into_iter()
                    .map(AgentParameters::G3mLiquidityProvider)
                    .collect()
            }
            AgentParameters::ParameterManager(parameters) => {
                let parameters: Vec<ParameterManagerParameters<Single>> = parameters.into();
                parameters
                    .into_iter()
                    .map(AgentParameters::ParameterManager)
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
