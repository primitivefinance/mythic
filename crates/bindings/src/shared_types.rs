///`Parameters(uint256,uint256,uint256)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct Parameters {
    pub strike_price_wad: ::ethers::core::types::U256,
    pub sigma_percent_wad: ::ethers::core::types::U256,
    pub tau_years_wad: ::ethers::core::types::U256,
}
