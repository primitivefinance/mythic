use clients::protocol::ProtocolClient;
use ethers::abi::Address;

use self::{ln_arbitrageur::LnArbitrageur, ln_liquidity_provider::LogNormalLiquidityProvider};
use super::{ParameterManager, *};

pub mod ln_arbitrageur;
pub mod ln_liquidity_provider;

pub async fn ln_setup(
    environment: &Environment,
    config: &SimulationConfig<Single>,
    protocol_client: ProtocolClient<RevmMiddleware>,
    liquid_exchange_address: Address,
    token_admin: &TokenAdmin,
    pool_id: U256,
) -> Result<(LogNormalLiquidityProvider, LnArbitrageur, ParameterManager)> {
    let manager = ParameterManager::new(
        environment,
        config,
        protocol_client.clone(),
        "ln_manager",
        liquid_exchange_address,
        token_admin,
        pool_id,
    )
    .await?;
    let arbitrageur = LnArbitrageur::new(
        environment,
        token_admin,
        liquid_exchange_address,
        protocol_client.clone(),
        pool_id,
    )
    .await?;
    let lp = LogNormalLiquidityProvider::new(
        environment,
        config,
        "ln_lp",
        token_admin,
        protocol_client.clone(),
    )
    .await?;
    Ok((lp, arbitrageur, manager))
}
