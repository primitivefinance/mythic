use clients::protocol::ProtocolClient;
use ethers::abi::Address;

use self::{g3m_arbitrageur::G3mArbitrageur, g3m_liquidity_provider::G3mLiquidityProvider};
use super::{ParameterManager, *};
pub mod dca_g3m_liquidity_provider;
pub mod g3m_arbitrageur;
pub mod g3m_liquidity_provider;

pub async fn g3m_setup(
    environment: &Environment,
    config: &SimulationConfig<Single>,
    protocol_client: ProtocolClient<RevmMiddleware>,
    liquid_exchange_address: Address,
    token_admin: &TokenAdmin,
    pool_id: U256,
) -> Result<(G3mLiquidityProvider, G3mArbitrageur, ParameterManager)> {
    let manager = ParameterManager::new(
        environment,
        config,
        protocol_client.clone(),
        "g3m_manager",
        liquid_exchange_address,
        pool_id,
    )
    .await?;
    let arbitrageur = G3mArbitrageur::new(
        environment,
        token_admin,
        liquid_exchange_address,
        protocol_client.clone(),
        pool_id,
    )
    .await?;
    let lp = G3mLiquidityProvider::new(
        environment,
        config,
        "g3m_lp",
        token_admin,
        protocol_client.clone(),
        manager.client.address(),
    )
    .await?;
    Ok((lp, arbitrageur, manager))
}

pub async fn dca_g3m_setup(
    environment: &Environment,
    config: &SimulationConfig<Single>,
    protocol_client: ProtocolClient<RevmMiddleware>,
    liquid_exchange_address: Address,
    token_admin: &TokenAdmin,
    pool_id: U256,
) -> Result<(DcaG3mLiquidityProvider, G3mArbitrageur)> {
    let arbitrageur = G3mArbitrageur::new(
        environment,
        token_admin,
        liquid_exchange_address,
        protocol_client.clone(),
        pool_id,
    )
    .await?;
    let lp = DcaG3mLiquidityProvider::new(
        environment,
        config,
        "g3m_lp",
        token_admin,
        protocol_client.clone(),
    )
    .await?;
    Ok((lp, arbitrageur))
}
