///`G3Mparameters(uint256,uint256)`
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
pub struct G3Mparameters {
    pub wx: ::ethers::core::types::U256,
    pub wy: ::ethers::core::types::U256,
}
///`LogNormParameters(uint256,uint256,uint256)`
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
pub struct LogNormParameters {
    pub strike: ::ethers::core::types::U256,
    pub sigma: ::ethers::core::types::U256,
    pub tau: ::ethers::core::types::U256,
}
