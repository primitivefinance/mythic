/// `GhostType(address,address,uint64)`
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
    Hash,
)]
pub struct GhostType {
    pub actor: ::ethers::core::types::Address,
    pub subject: ::ethers::core::types::Address,
    pub pool_id: u64,
}
/// `Order(uint128,uint128,bool,uint64,bool)`
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
    Hash,
)]
pub struct Order {
    pub input: u128,
    pub output: u128,
    pub use_max: bool,
    pub pool_id: u64,
    pub sell_asset: bool,
}
/// `FuzzSelector(address,bytes4[])`
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
    Hash,
)]
pub struct FuzzSelector {
    pub addr: ::ethers::core::types::Address,
    pub selectors: ::std::vec::Vec<[u8; 4]>,
}
/// `SubjectsType(address,address,address,address,address,address)`
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
    Hash,
)]
pub struct SubjectsType {
    pub deployer: ::ethers::core::types::Address,
    pub registry: ::ethers::core::types::Address,
    pub weth: ::ethers::core::types::Address,
    pub portfolio: ::ethers::core::types::Address,
    pub position_renderer: ::ethers::core::types::Address,
    pub normal_strategy: ::ethers::core::types::Address,
}
