pub mod base;
pub mod portfolio_management;

use base::{block_admin::*, price_changer::*, token_admin::*};
use portfolio_management::{base::parameter_manager::*, g3m::dca_g3m_liquidity_provider::*};

use self::portfolio_management::{
    base::parameter_manager::ParameterManagerParameters,
    g3m::{
        dca_g3m_liquidity_provider::DcaG3mLiquidityProviderParameters,
        dca_swapper::DcaSwapperParameters, g3m_liquidity_provider::G3mLiquidityProviderParameters,
    },
    lognormal::ln_liquidity_provider::LogNormalLiquidityProviderParameters,
};
use super::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AgentParameters<P: Parameterized> {
    G3mLiquidityProvider(G3mLiquidityProviderParameters<P>),
    DcaG3mLiquidityProvider(DcaG3mLiquidityProviderParameters<P>),
    LogNormalLiquidityProvider(LogNormalLiquidityProviderParameters<P>),
    BlockAdmin(BlockAdminParameters),
    TokenAdmin(TokenAdminParameters),
    PriceChanger(PriceChangerParameters<P>),
    ParameterManager(ParameterManagerParameters<P>),
    DcaSwapper(DcaSwapperParameters<P>),
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
            AgentParameters::DcaG3mLiquidityProvider(parameters) => {
                let parameters: Vec<DcaG3mLiquidityProviderParameters<Single>> = parameters.into();
                parameters
                    .into_iter()
                    .map(AgentParameters::DcaG3mLiquidityProvider)
                    .collect()
            }
            AgentParameters::DcaSwapper(parameters) => {
                let parameters: Vec<DcaSwapperParameters<Single>> = parameters.into();
                parameters
                    .into_iter()
                    .map(AgentParameters::DcaSwapper)
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
