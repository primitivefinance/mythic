///`DynamicParam(uint256,uint256,uint256,int256)`
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
pub struct DynamicParam {
    pub last_computed_value: ::ethers::core::types::U256,
    pub update_end: ::ethers::core::types::U256,
    pub last_update_at: ::ethers::core::types::U256,
    pub update_per_second: ::ethers::core::types::I256,
}
///`InitParams(address,address,address,bytes)`
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
    pub strategy: ::ethers::core::types::Address,
    pub token_x: ::ethers::core::types::Address,
    pub token_y: ::ethers::core::types::Address,
    pub data: ::ethers::core::types::Bytes,
}
///`FuzzInterface(address,string[])`
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
pub struct FuzzInterface {
    pub addr: ::ethers::core::types::Address,
    pub artifacts: ::std::vec::Vec<::std::string::String>,
}
///`FuzzSelector(address,bytes4[])`
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
pub struct FuzzSelector {
    pub addr: ::ethers::core::types::Address,
    pub selectors: ::std::vec::Vec<[u8; 4]>,
}
