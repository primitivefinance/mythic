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
///`InitParams(uint256,address,address,address,uint256,bytes)`
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
pub struct InitParams {
    pub pool_id: ::ethers::core::types::U256,
    pub strategy: ::ethers::core::types::Address,
    pub token_x: ::ethers::core::types::Address,
    pub token_y: ::ethers::core::types::Address,
    pub swap_fee_percentage_wad: ::ethers::core::types::U256,
    pub data: ::ethers::core::types::Bytes,
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
