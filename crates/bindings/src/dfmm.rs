pub use dfmm::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod dfmm {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("allocate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("allocate"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("balanceOf"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("balanceOf"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("balance"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("deallocate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("deallocate"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getPool"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getPool"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct IDFMM.Pool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getReservesAndLiquidity"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getReservesAndLiquidity",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("init"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("init"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("params"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct IDFMM.InitParams"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("lastFeeGrowthOf"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("lastFeeGrowthOf"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("lastFeeGrowth"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("nonce"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("nonce"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("pools"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("pools"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("inited"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("controller"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("strategy"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenX"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenY"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("reserveX"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("reserveY"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("totalLiquidity"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("feeGrowth"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("swap"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("swap"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("update"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("update"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("Allocate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Allocate"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("deltaX"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("deltaY"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("deltaL"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Deallocate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Deallocate"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("deltaX"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("deltaY"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("deltaL"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Init"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Init"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("strategy"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("reserveX"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("reserveY"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("totalLiquidity"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("LogPoolStats"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("LogPoolStats"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("rx"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("ry"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("L"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("invariant"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sigma"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("strike"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("tau"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("timestamp"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Swap"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Swap"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("isSwapXForY"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("inputAmount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("outputAmount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("Invalid"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("Invalid"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("negative"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "swapConstantGrowth",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidSwapInputTransfer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InvalidSwapInputTransfer",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidSwapOutputTransfer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InvalidSwapOutputTransfer",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidTokens"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidTokens"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Locked"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("Locked"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Min"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("Min"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NotController"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("NotController"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NotInitialized"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("NotInitialized"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static DFMM_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R`\x01\x80U4\x80\x15a\0aW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[Pa%\xC3\x80a\0q`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\0\xF5W`\x005`\xE0\x1C\x80c_\0\xE1\xF3\x11a\0\xBEW\x80c_\0\xE1\xF3\x14a\x02.W\x80c\x9D\x94/\x9A\x14a\x02YW\x80c\xACJ\xFA8\x14a\x02lW\x80c\xAF\xFE\xD0\xE0\x14a\x02\xD7W\x80c\xBD\x06%\xAB\x14a\x02\xDFW\x80c\xCE\x15;\xF4\x14a\x03\x07Wa\0\xF5V[\x80b\xFD\xD5\x8E\x14a\x01ZW\x80c\x02\x16\xB88\x14a\x01\x98W\x80c\x06\x8B\xCD\x8D\x14a\x01\xADW\x80c\x14U\xF1\xFC\x14a\x01\xCDW\x80c.\xC3\x81\x88\x14a\x02\0W[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x01\x85a\x01h6`\x04a\x1F\xF7V[`\x02` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\xABa\x01\xA66`\x04a }V[a\x03\x1AV[\0[a\x01\xC0a\x01\xBB6`\x04a!\xAAV[a\x04\xACV[`@Qa\x01\x8F\x91\x90a!\xC6V[a\x01\xE0a\x01\xDB6`\x04a\"fV[a\x05\xA0V[`@\x80Q\x94\x85R` \x85\x01\x93\x90\x93R\x91\x83\x01R``\x82\x01R`\x80\x01a\x01\x8FV[a\x02\x13a\x02\x0E6`\x04a }V[a\n\x95V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x01\x8FV[a\x01\x85a\x02<6`\x04a\x1F\xF7V[`\x03` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[a\x02\x13a\x02g6`\x04a }V[a\x0C\x8DV[a\x02\x7Fa\x02z6`\x04a!\xAAV[a\x107V[`@\x80Q\x99\x15\x15\x8AR`\x01`\x01`\xA0\x1B\x03\x98\x89\x16` \x8B\x01R\x96\x88\x16\x96\x89\x01\x96\x90\x96R\x93\x86\x16``\x88\x01R\x94\x90\x91\x16`\x80\x86\x01R`\xA0\x85\x01R`\xC0\x84\x01\x92\x90\x92R`\xE0\x83\x01\x91\x90\x91Ra\x01\0\x82\x01Ra\x01 \x01a\x01\x8FV[`\0Ta\x01\x85V[a\x02\xF2a\x02\xED6`\x04a }V[a\x10\xA7V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x01\x8FV[a\x02\x13a\x03\x156`\x04a!\xAAV[a\x12\xB6V[`\x01T`\x02\x03a\x03=W`@Qc\x03\xCB\x96\xDB`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x01\x81\x90UP\x82`\0\x81\x81T\x81\x10a\x03YWa\x03Ya\"\xFEV[`\0\x91\x82R` \x90\x91 `\x08\x90\x91\x02\x01T`\xFF\x16a\x03\x8AW`@Qc!\xC4\xE3W`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x84\x81T\x81\x10a\x03\x9DWa\x03\x9Da\"\xFEV[`\0\x91\x82R` \x90\x91 `\x08\x90\x91\x02\x01Ta\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x163\x14a\x03\xDBW`@Qc#\x01\x9Eg`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x84\x81T\x81\x10a\x03\xEEWa\x03\xEEa\"\xFEV[`\0\x91\x82R` \x90\x91 `\x01`\x08\x90\x92\x02\x01\x01T`@QbB\xD7\x07`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x02\x16\xB88\x90a\x042\x90\x87\x90\x87\x90\x87\x90`\x04\x01a#\x14V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x04\x8AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a%\xA3\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x04\x9EW=`\0\x80>=`\0\xFD[PP`\x01\x80UPPPPPPV[`@\x80Qa\x01 \x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R`\xC0\x81\x01\x82\x90R`\xE0\x81\x01\x82\x90Ra\x01\0\x81\x01\x91\x90\x91R`\0\x82\x81T\x81\x10a\x05\x08Wa\x05\x08a\"\xFEV[`\0\x91\x82R` \x91\x82\x90 `@\x80Qa\x01 \x81\x01\x82R`\x08\x90\x93\x02\x90\x91\x01\x80T`\xFF\x81\x16\x15\x15\x84R`\x01`\x01`\xA0\x1B\x03a\x01\0\x91\x82\x90\x04\x81\x16\x95\x85\x01\x95\x90\x95R`\x01\x82\x01T\x85\x16\x92\x84\x01\x92\x90\x92R`\x02\x81\x01T\x84\x16``\x84\x01R`\x03\x81\x01T\x90\x93\x16`\x80\x83\x01R`\x04\x83\x01T`\xA0\x83\x01R`\x05\x83\x01T`\xC0\x83\x01R`\x06\x83\x01T`\xE0\x83\x01R`\x07\x90\x92\x01T\x91\x81\x01\x91\x90\x91R\x92\x91PPV[`\0\x80`\0\x80`\x01T`\x02\x03a\x05\xC9W`@Qc\x03\xCB\x96\xDB`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x01Ua\x05\xDE``\x86\x01`@\x87\x01a#JV[`\x01`\x01`\xA0\x1B\x03\x16a\x05\xF7`@\x87\x01` \x88\x01a#JV[`\x01`\x01`\xA0\x1B\x03\x16\x03a\x06\x1EW`@Qc3\x91\n\xEF`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80\x80\x80\x80a\x061` \x8B\x01\x8Ba#JV[`\0T`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90c\x9F\x83\x13{\x90a\x06T``\x8E\x01\x8Ea#\xADV[`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x06r\x93\x92\x91\x90a#\x14V[`\xA0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x06\xCAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a%\xA3\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x06\xDEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x02\x91\x90a$\x8EV[\x94P\x94P\x94P\x94P\x94P\x84a\x07EW`\0\x84\x12a\x07\x1E\x85a\x136V[`@Qcw`m)`\xE1\x1B\x81R\x91\x15\x15`\x04\x83\x01R`$\x82\x01R`D\x01[`@Q\x80\x91\x03\x90\xFD[`@\x80Qa\x01 \x81\x01\x82R`\x01\x81R3` \x80\x83\x01\x91\x90\x91R`\0\x92\x82\x01\x90a\x07p\x90\x8E\x01\x8Ea#JV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8C` \x01` \x81\x01\x90a\x07\x91\x91\x90a#JV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01a\x07\xAF``\x8E\x01`@\x8F\x01a#JV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R` \x80\x83\x01\x88\x90R`@\x80\x84\x01\x88\x90R``\x80\x85\x01\x88\x90Rg\r\xE0\xB6\xB3\xA7d\0\0`\x80\x95\x86\x01R`\0\x80T`\x01\x80\x82\x01\x83U\x82\x80R\x88Q\x7F)\r\xEC\xD9T\x8Bb\xA8\xD6\x03E\xA9\x888o\xC8K\xA6\xBC\x95H@\x08\xF66/\x93\x16\x0E\xF3\xE5c`\x08\x90\x93\x02\x92\x83\x01\x80T\x97\x8B\x01Q`\x01`\x01`\xA8\x1B\x03\x19\x90\x98\x16\x91\x15\x15a\x01\0`\x01`\xA8\x1B\x03\x19\x16\x91\x90\x91\x17a\x01\0\x97\x89\x16\x88\x02\x17\x90U\x93\x88\x01Q\x7F)\r\xEC\xD9T\x8Bb\xA8\xD6\x03E\xA9\x888o\xC8K\xA6\xBC\x95H@\x08\xF66/\x93\x16\x0E\xF3\xE5d\x82\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16\x92\x89\x16\x92\x90\x92\x17\x90U\x92\x88\x01Q\x7F)\r\xEC\xD9T\x8Bb\xA8\xD6\x03E\xA9\x888o\xC8K\xA6\xBC\x95H@\x08\xF66/\x93\x16\x0E\xF3\xE5e\x82\x01\x80T\x85\x16\x91\x88\x16\x91\x90\x91\x17\x90U\x95\x87\x01Q\x7F)\r\xEC\xD9T\x8Bb\xA8\xD6\x03E\xA9\x888o\xC8K\xA6\xBC\x95H@\x08\xF66/\x93\x16\x0E\xF3\xE5f\x87\x01\x80T\x90\x93\x16\x95\x16\x94\x90\x94\x17\x90U`\xA0\x85\x01Q\x7F)\r\xEC\xD9T\x8Bb\xA8\xD6\x03E\xA9\x888o\xC8K\xA6\xBC\x95H@\x08\xF66/\x93\x16\x0E\xF3\xE5g\x85\x01U`\xC0\x85\x01Q\x7F)\r\xEC\xD9T\x8Bb\xA8\xD6\x03E\xA9\x888o\xC8K\xA6\xBC\x95H@\x08\xF66/\x93\x16\x0E\xF3\xE5h\x85\x01U`\xE0\x85\x01Q\x7F)\r\xEC\xD9T\x8Bb\xA8\xD6\x03E\xA9\x888o\xC8K\xA6\xBC\x95H@\x08\xF66/\x93\x16\x0E\xF3\xE5i\x85\x01U\x90\x84\x01Q\x7F)\r\xEC\xD9T\x8Bb\xA8\xD6\x03E\xA9\x888o\xC8K\xA6\xBC\x95H@\x08\xF66/\x93\x16\x0E\xF3\xE5j\x90\x93\x01\x92\x90\x92U\x80T\x92\x93P\x91a\t\xA8\x91\x90a$\xEEV[3`\0\x81\x81R`\x02` \x90\x81R`@\x80\x83 \x85\x84R\x82R\x80\x83 \x88\x90U\x92\x82R`\x03\x81R\x82\x82 \x84\x83R\x81R\x90\x82\x90 g\r\xE0\xB6\xB3\xA7d\0\0\x90U\x91\x92Pa\n\0\x91a\t\xF8\x91\x8F\x01\x90\x8F\x01a#JV[30\x88a\x13uV[a\n\x1Ba\n\x13``\x8E\x01`@\x8F\x01a#JV[30\x87a\x13uV[a\n(` \x8D\x01\x8Da#JV[`@\x80Q\x83\x81R` \x81\x01\x88\x90R\x90\x81\x01\x86\x90R``\x81\x01\x85\x90R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x903\x90\x7F\x9C\xF5\x96\x1A\x85\xC7\xF8\xE7\xA38\xE8\xA6_[(\xEC\x17\x98_\xA7q\xAB\x0C{\xB9>\xB8kv\xCF\xECX\x90`\x80\x01`@Q\x80\x91\x03\x90\xA3`\x01\x80U\x9B\x93\x9AP\x91\x98P\x96P\x90\x94PPPPPV[`\0\x80`\0`\x01T`\x02\x03a\n\xBDW`@Qc\x03\xCB\x96\xDB`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x01\x81\x90UP\x85`\0\x81\x81T\x81\x10a\n\xD9Wa\n\xD9a\"\xFEV[`\0\x91\x82R` \x90\x91 `\x08\x90\x91\x02\x01T`\xFF\x16a\x0B\nW`@Qc!\xC4\xE3W`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0B\x13\x87a\x14\x03V[`\0\x80`\0a\x0B%\x8A`\x01\x8B\x8Ba\x15\rV[\x92P\x92P\x92P\x80`\x02`\x003`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x8C\x81R` \x01\x90\x81R` \x01`\0 `\0\x82\x82Ta\x0Bq\x91\x90a%\x01V[\x90\x91UPP`\0\x80T\x8B\x90\x81\x10a\x0B\x8AWa\x0B\x8Aa\"\xFEV[`\0\x91\x82R` \x80\x83 `\x07`\x08\x90\x93\x02\x01\x91\x90\x91\x01T3\x83R`\x03\x82R`@\x80\x84 \x8E\x85R\x90\x92R\x90\x82 U\x80Ta\x0B\xF3\x91\x90\x8C\x90\x81\x10a\x0B\xCEWa\x0B\xCEa\"\xFEV[`\0\x91\x82R` \x90\x91 `\x02`\x08\x90\x92\x02\x01\x01T`\x01`\x01`\xA0\x1B\x03\x1630\x86a\x13uV[a\x0C.`\0\x8B\x81T\x81\x10a\x0C\tWa\x0C\ta\"\xFEV[`\0\x91\x82R` \x90\x91 `\x03`\x08\x90\x92\x02\x01\x01T`\x01`\x01`\xA0\x1B\x03\x1630\x85a\x13uV[`@\x80Q\x8B\x81R` \x81\x01\x85\x90R\x90\x81\x01\x83\x90R``\x81\x01\x82\x90R3\x90\x7F\x95\x97W\x7F3\x93 w^c\xD3\xFE\xD7\xD5\xDD\xE66[\xAD\xCC\x9F\xCC\xDBf\xB3H\x94c\x0C\xA9\x8Bo\x90`\x80\x01[`@Q\x80\x91\x03\x90\xA2`\x01\x80U\x91\x99\x90\x98P\x90\x96P\x94PPPPPV[`\0\x80`\0`\x01T`\x02\x03a\x0C\xB5W`@Qc\x03\xCB\x96\xDB`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x01\x81\x90UP\x85`\0\x81\x81T\x81\x10a\x0C\xD1Wa\x0C\xD1a\"\xFEV[`\0\x91\x82R` \x90\x91 `\x08\x90\x91\x02\x01T`\xFF\x16a\r\x02W`@Qc!\xC4\xE3W`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\r\x0B\x87a\x14\x03V[`\0\x80`\0a\r\x1D\x8A`\0\x8B\x8Ba\x15\rV[\x92P\x92P\x92P\x80`\x02`\x003`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x8C\x81R` \x01\x90\x81R` \x01`\0 `\0\x82\x82Ta\ri\x91\x90a$\xEEV[\x90\x91UPP`\0\x80T\x8B\x90\x81\x10a\r\x82Wa\r\x82a\"\xFEV[`\0\x91\x82R` \x80\x83 `\x07`\x08\x90\x93\x02\x01\x91\x90\x91\x01T3\x83R`\x03\x82R`@\x80\x84 \x8E\x85R\x90\x92R\x90\x82 U\x80T\x8B\x90\x81\x10a\r\xC1Wa\r\xC1a\"\xFEV[`\0\x91\x82R` \x90\x91 `\x08\x90\x91\x02\x01`\x02\x01T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x85\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0E\\W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a%\xA3\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x0EpW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\x94\x91\x90a%\x14V[P`\0\x8A\x81T\x81\x10a\x0E\xA8Wa\x0E\xA8a\"\xFEV[`\0\x91\x82R` \x90\x91 `\x08\x90\x91\x02\x01`\x03\x01T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0FCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a%\xA3\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x0FWW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F{\x91\x90a%\x14V[Pa\x0F\xB6`\0\x8B\x81T\x81\x10a\x0F\x92Wa\x0F\x92a\"\xFEV[`\0\x91\x82R` \x90\x91 `\x02`\x08\x90\x92\x02\x01\x01T`\x01`\x01`\xA0\x1B\x03\x163\x85a\x17\xE8V[a\x0F\xF0`\0\x8B\x81T\x81\x10a\x0F\xCCWa\x0F\xCCa\"\xFEV[`\0\x91\x82R` \x90\x91 `\x03`\x08\x90\x92\x02\x01\x01T`\x01`\x01`\xA0\x1B\x03\x163\x84a\x17\xE8V[`@\x80Q\x8B\x81R` \x81\x01\x85\x90R\x90\x81\x01\x83\x90R``\x81\x01\x82\x90R3\x90\x7F\xAC\xBE\x12~\x93\xA8\xA0\xB2x\xD8\xE0n' [=\xF9\xD1\xF3\x81$\x14\xBC\x89\x17\xC7t\xA87\x101n\x90`\x80\x01a\x0CqV[`\0\x81\x81T\x81\x10a\x10GW`\0\x80\xFD[`\0\x91\x82R` \x90\x91 `\x08\x90\x91\x02\x01\x80T`\x01\x82\x01T`\x02\x83\x01T`\x03\x84\x01T`\x04\x85\x01T`\x05\x86\x01T`\x06\x87\x01T`\x07\x90\x97\x01T`\xFF\x87\x16\x98Pa\x01\0\x90\x96\x04`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x97\x95\x81\x16\x96\x94\x81\x16\x95\x93\x16\x93\x91\x92\x90\x91\x89V[`\0\x80`\x01T`\x02\x03a\x10\xCDW`@Qc\x03\xCB\x96\xDB`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x01\x81\x90UP\x84`\0\x81\x81T\x81\x10a\x10\xE9Wa\x10\xE9a\"\xFEV[`\0\x91\x82R` \x90\x91 `\x08\x90\x91\x02\x01T`\xFF\x16a\x11\x1AW`@Qc!\xC4\xE3W`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80`\0\x80`\0\x80\x8B\x81T\x81\x10a\x114Wa\x114a\"\xFEV[`\0\x91\x82R` \x90\x91 `\x01`\x08\x90\x92\x02\x01\x01T`@Qc2\x14\x89\x0F`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c2\x14\x89\x0F\x90a\x11y\x90\x8E\x90\x8E\x90\x8E\x90`\x04\x01a#\x14V[`\xC0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x11\xCFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a%\xA3\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x11\xE3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\x07\x91\x90a%2V[\x95P\x95P\x95PP\x94P\x94P\x84a\x12$W`\0\x84\x12a\x07\x1E\x85a\x136V[a\x12.\x8B\x82a\x18lV[`\0\x80`\0a\x12>\x8E\x87\x87a\x19=V[\x94P\x94PPP\x92P\x8D3`\x01`\x01`\xA0\x1B\x03\x16\x7FL}\xEF\x84\xE4++\xC0\xA5\xAA\xB2\"\x86\x8D\xD7\xA0\x92\xB53w\xA4\xB57\xAB\xCD\x944Zz\x85'\xED\x85\x85\x85`@Qa\x12\x97\x93\x92\x91\x90\x92\x15\x15\x83R` \x83\x01\x91\x90\x91R`@\x82\x01R``\x01\x90V[`@Q\x80\x91\x03\x90\xA3`\x01\x80U\x90\x9D\x90\x9CP\x9APPPPPPPPPPPV[`\0\x80`\0\x80\x84\x81T\x81\x10a\x12\xCDWa\x12\xCDa\"\xFEV[\x90`\0R` `\0 \x90`\x08\x02\x01`\x04\x01T`\0\x85\x81T\x81\x10a\x12\xF2Wa\x12\xF2a\"\xFEV[\x90`\0R` `\0 \x90`\x08\x02\x01`\x05\x01T`\0\x86\x81T\x81\x10a\x13\x17Wa\x13\x17a\"\xFEV[\x90`\0R` `\0 \x90`\x08\x02\x01`\x06\x01T\x92P\x92P\x92P\x91\x93\x90\x92PV[`\0`\x01`\xFF\x1B\x82\x03a\x13\\W`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x12\x15a\x13mWP\x19`\x01\x01\x90V[P\x80[\x91\x90PV[`\0`@Qc#\xB8r\xDD`\xE0\x1B`\0R\x84`\x04R\x83`$R\x82`DR` `\0`d`\0\x80\x8AZ\xF1=\x15`\x1F=\x11`\x01`\0Q\x14\x16\x17\x16\x91P`\0``R\x80`@RP\x80a\x13\xFCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rs\x15\x14\x90S\x94\xD1\x91T\x97\xD1\x94\x93\xD3W\xD1\x90RS\x11Q`b\x1B`D\x82\x01R`d\x01a\x07<V[PPPPPV[3`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 \x84\x84R\x90\x91R\x90 T\x15\x80\x15\x90a\x14lWP3`\0\x90\x81R`\x03` \x90\x81R`@\x80\x83 \x84\x84R\x90\x91R\x81 T\x81T\x90\x91\x90\x83\x90\x81\x10a\x14WWa\x14Wa\"\xFEV[\x90`\0R` `\0 \x90`\x08\x02\x01`\x07\x01T\x14\x15[\x15a\x15\nW3`\0\x90\x81R`\x03` \x90\x81R`@\x80\x83 \x84\x84R\x90\x91R\x81 T\x81Ta\x14\xC5\x91\x90\x83\x90\x85\x90\x81\x10a\x14\xA5Wa\x14\xA5a\"\xFEV[\x90`\0R` `\0 \x90`\x08\x02\x01`\x07\x01Ta\x1E\xF2\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[3`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 \x86\x84R\x90\x91R\x90 T\x90\x91Pa\x14\xED\x90\x82a\x1E\xF2V[3`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 \x86\x84R\x90\x91R\x90 UP[PV[`\0\x80`\0\x80`\0\x80`\0\x80`\0\x8C\x81T\x81\x10a\x15,Wa\x15,a\"\xFEV[`\0\x91\x82R` \x90\x91 `\x01`\x08\x90\x92\x02\x01\x01T`@Qb#\x8Bu`\xEA\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x8E-\xD4\0\x90a\x15p\x90\x8F\x90\x8E\x90\x8E\x90`\x04\x01a#\x14V[`\xA0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x15\xC6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a%\xA3\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x15\xDAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\xFE\x91\x90a$\x8EV[\x94P\x94P\x94P\x94P\x94P\x84a\x16\x1AW`\0\x84\x12a\x07\x1E\x85a\x136V[\x8Aa\x16TW\x82`\0\x8D\x81T\x81\x10a\x163Wa\x163a\"\xFEV[\x90`\0R` `\0 \x90`\x08\x02\x01`\x04\x01Ta\x16O\x91\x90a$\xEEV[a\x16\x84V[`\0\x8C\x81T\x81\x10a\x16gWa\x16ga\"\xFEV[\x90`\0R` `\0 \x90`\x08\x02\x01`\x04\x01T\x83a\x16\x84\x91\x90a$\xEEV[\x97P\x8Aa\x16\xC0W\x81`\0\x8D\x81T\x81\x10a\x16\x9FWa\x16\x9Fa\"\xFEV[\x90`\0R` `\0 \x90`\x08\x02\x01`\x05\x01Ta\x16\xBB\x91\x90a$\xEEV[a\x16\xF0V[`\0\x8C\x81T\x81\x10a\x16\xD3Wa\x16\xD3a\"\xFEV[\x90`\0R` `\0 \x90`\x08\x02\x01`\x05\x01T\x82a\x16\xF0\x91\x90a$\xEEV[\x96P\x8Aa\x17,W\x80`\0\x8D\x81T\x81\x10a\x17\x0BWa\x17\x0Ba\"\xFEV[\x90`\0R` `\0 \x90`\x08\x02\x01`\x06\x01Ta\x17'\x91\x90a$\xEEV[a\x17\\V[`\0\x8C\x81T\x81\x10a\x17?Wa\x17?a\"\xFEV[\x90`\0R` `\0 \x90`\x08\x02\x01`\x06\x01T\x81a\x17\\\x91\x90a$\xEEV[\x95P\x82`\0\x8D\x81T\x81\x10a\x17rWa\x17ra\"\xFEV[\x90`\0R` `\0 \x90`\x08\x02\x01`\x04\x01\x81\x90UP\x81`\0\x8D\x81T\x81\x10a\x17\x9BWa\x17\x9Ba\"\xFEV[\x90`\0R` `\0 \x90`\x08\x02\x01`\x05\x01\x81\x90UP\x80`\0\x8D\x81T\x81\x10a\x17\xC4Wa\x17\xC4a\"\xFEV[\x90`\0R` `\0 \x90`\x08\x02\x01`\x06\x01\x81\x90UPPPPPP\x94P\x94P\x94\x91PPV[`\0`@Qc\xA9\x05\x9C\xBB`\xE0\x1B`\0R\x83`\x04R\x82`$R` `\0`D`\0\x80\x89Z\xF1=\x15`\x1F=\x11`\x01`\0Q\x14\x16\x17\x16\x91P`\0``R\x80`@RP\x80a\x18fW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rn\x15\x14\x90S\x94\xD1\x91T\x97\xD1\x90RS\x11Q`\x8A\x1B`D\x82\x01R`d\x01a\x07<V[PPPPV[`\0\x80\x83\x81T\x81\x10a\x18\x80Wa\x18\x80a\"\xFEV[\x90`\0R` `\0 \x90`\x08\x02\x01`\x06\x01T\x90P\x81`\0\x84\x81T\x81\x10a\x18\xA8Wa\x18\xA8a\"\xFEV[\x90`\0R` `\0 \x90`\x08\x02\x01`\x06\x01\x81\x90UP`\0a\x18\xF6\x82`\0\x86\x81T\x81\x10a\x18\xD6Wa\x18\xD6a\"\xFEV[\x90`\0R` `\0 \x90`\x08\x02\x01`\x06\x01Ta\x1F\x10\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa\x19\x0F\x81`\0\x86\x81T\x81\x10a\x14\xA5Wa\x14\xA5a\"\xFEV[`\0\x85\x81T\x81\x10a\x19\"Wa\x19\"a\"\xFEV[\x90`\0R` `\0 \x90`\x08\x02\x01`\x07\x01\x81\x90UPPPPPV[`\0\x80`\0\x80`\0\x80`\0\x89\x81T\x81\x10a\x19YWa\x19Ya\"\xFEV[\x90`\0R` `\0 \x90`\x08\x02\x01`\x04\x01T\x90P`\0\x80\x8A\x81T\x81\x10a\x19\x81Wa\x19\x81a\"\xFEV[\x90`\0R` `\0 \x90`\x08\x02\x01`\x05\x01T\x90P\x81\x89\x11\x96P\x86a\x19\xD4W`\0\x8A\x81T\x81\x10a\x19\xB2Wa\x19\xB2a\"\xFEV[`\0\x91\x82R` \x90\x91 `\x03`\x08\x90\x92\x02\x01\x01T`\x01`\x01`\xA0\x1B\x03\x16a\x1A\x05V[`\0\x8A\x81T\x81\x10a\x19\xE7Wa\x19\xE7a\"\xFEV[`\0\x91\x82R` \x90\x91 `\x02`\x08\x90\x92\x02\x01\x01T`\x01`\x01`\xA0\x1B\x03\x16[\x95P\x86a\x1AAW`\0\x8A\x81T\x81\x10a\x1A\x1FWa\x1A\x1Fa\"\xFEV[`\0\x91\x82R` \x90\x91 `\x02`\x08\x90\x92\x02\x01\x01T`\x01`\x01`\xA0\x1B\x03\x16a\x1ArV[`\0\x8A\x81T\x81\x10a\x1ATWa\x1ATa\"\xFEV[`\0\x91\x82R` \x90\x91 `\x03`\x08\x90\x92\x02\x01\x01T`\x01`\x01`\xA0\x1B\x03\x16[\x94P\x86a\x1A\x88Wa\x1A\x83\x81\x89a$\xEEV[a\x1A\x92V[a\x1A\x92\x82\x8Aa$\xEEV[\x93P\x86a\x1A\xA8Wa\x1A\xA3\x89\x83a$\xEEV[a\x1A\xB2V[a\x1A\xB2\x88\x82a$\xEEV[\x92P\x86\x15a\x1A\xFDW\x87\x81\x11a\x1A\xF8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01Rk\x06\x96\xE7f\x16\xC6\x96B\x077v\x17`\xA4\x1B`D\x82\x01R`d\x01a\x07<V[a\x1B;V[\x88\x82\x11a\x1B;W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01Rk\x06\x96\xE7f\x16\xC6\x96B\x077v\x17`\xA4\x1B`D\x82\x01R`d\x01a\x07<V[\x88`\0\x8B\x81T\x81\x10a\x1BOWa\x1BOa\"\xFEV[\x90`\0R` `\0 \x90`\x08\x02\x01`\x04\x01\x81\x90UP\x87`\0\x8B\x81T\x81\x10a\x1BxWa\x1Bxa\"\xFEV[`\0\x91\x82R` \x82 `\x08\x91\x90\x91\x02\x01`\x05\x01\x91\x90\x91U`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x88\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x1C\x0CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a%\xA3\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x1C W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1CD\x91\x90a%\x86V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x88\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x1C\xC7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a%\xA3\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x1C\xDBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C\xFF\x91\x90a%\x86V[\x90Pa\x1D\r\x8830\x89a\x13uV[a\x1D\x18\x873\x87a\x17\xE8V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x90`\x01`\x01`\xA0\x1B\x03\x8A\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x1D\x98W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a%\xA3\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x1D\xACW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1D\xD0\x91\x90a%\x86V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x8A\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x1ESW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a%\xA3\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x1EgW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E\x8B\x91\x90a%\x86V[\x90Pa\x1E\x97\x88\x85a%\x01V[\x82\x10\x15a\x1E\xB7W`@Qc =\x90\x1D`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1E\xC1\x87\x84a$\xEEV[\x81\x10\x15a\x1E\xE1W`@Qc\xF3\xCB\xBC\x87`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPP\x93\x97\x92\x96P\x93P\x93PV[`\0a\x1F\x07\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x1F!V[\x90P[\x92\x91PPV[`\0a\x1F\x07\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x1F9W`\0\x80\xFD[\x04\x92\x91PPV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x13pW`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a \rWa \ra\x1F@V[a \x16\x83a\x1F\xE0V[\x94` \x93\x90\x93\x015\x93PPPV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rjrray stride`\xA8\x1B`d\x82\x01R`\x84\x81\xFD[`\0\x80`\0`@\x84\x86\x03\x12\x15a \x95Wa \x95a\x1F@V[\x835\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a \xB7Wa \xB7a\x1F\x90V[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a!\x1FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x81\xFD[\x815\x81\x81\x11\x15a!\x82W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x0ENL/$\r\x8C\xAD\xCC\xEE\x8D`\xAB\x1B`d\x82\x01R`\x84\x81\xFD[\x87` \x82\x85\x01\x01\x11\x15a!\x97Wa!\x97a $V[` \x83\x01\x94P\x80\x93PPPP\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a!\xBFWa!\xBFa\x1F@V[P5\x91\x90PV[\x81Q\x15\x15\x81R` \x80\x83\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x90\x82\x01R`@\x80\x83\x01Qa\x01 \x83\x01\x91a!\xFF\x90\x84\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P``\x83\x01Qa\"\x1A``\x84\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P`\x80\x83\x01Qa\"5`\x80\x84\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P`\xA0\x83\x01Q`\xA0\x83\x01R`\xC0\x83\x01Q`\xC0\x83\x01R`\xE0\x83\x01Q`\xE0\x83\x01Ra\x01\0\x80\x84\x01Q\x81\x84\x01RP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\"{Wa\"{a\x1F@V[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\"\x95Wa\"\x95a\x1F\x90V[\x82\x01`\x80\x81\x85\x03\x12\x15a\"\xF7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FABI decoding: struct calldata to`D\x82\x01Rf\x1B\xC8\x1C\xDA\x1B\xDC\x9D`\xCA\x1B`d\x82\x01R`\x84\x81\xFD[\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[\x83\x81R`@` \x82\x01R\x81`@\x82\x01R\x81\x83``\x83\x017`\0\x81\x83\x01``\x90\x81\x01\x91\x90\x91R`\x1F\x90\x92\x01`\x1F\x19\x16\x01\x01\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a#_Wa#_a\x1F@V[a\x1F\x07\x82a\x1F\xE0V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FCalldata tail too short\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x81\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a$\x04W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FInvalid calldata tail offset\0\0\0\0`D\x82\x01R`d\x81\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a$_W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FInvalid calldata tail length\0\0\0\0`D\x82\x01R`d\x81\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a$wWa$wa#hV[\x92P\x92\x90PV[\x80Q\x80\x15\x15\x81\x14a\x13pW`\0\x80\xFD[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a$\xA9Wa$\xA9a\x1F@V[a$\xB2\x86a$~V[` \x87\x01Q`@\x88\x01Q``\x89\x01Q`\x80\x90\x99\x01Q\x92\x9A\x91\x99P\x97\x96P\x90\x94P\x92PPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x1F\nWa\x1F\na$\xD8V[\x80\x82\x01\x80\x82\x11\x15a\x1F\nWa\x1F\na$\xD8V[`\0` \x82\x84\x03\x12\x15a%)Wa%)a\x1F@V[a\x1F\x07\x82a$~V[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a%NWa%Na\x1F@V[a%W\x87a$~V[\x95P` \x87\x01Q\x94P`@\x87\x01Q\x93P``\x87\x01Q\x92P`\x80\x87\x01Q\x91P`\xA0\x87\x01Q\x90P\x92\x95P\x92\x95P\x92\x95V[`\0` \x82\x84\x03\x12\x15a%\x9BWa%\x9Ba\x1F@V[PQ\x91\x90PV\xFETarget contract does not contain";
    /// The bytecode of the contract.
    pub static DFMM_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\0\xF5W`\x005`\xE0\x1C\x80c_\0\xE1\xF3\x11a\0\xBEW\x80c_\0\xE1\xF3\x14a\x02.W\x80c\x9D\x94/\x9A\x14a\x02YW\x80c\xACJ\xFA8\x14a\x02lW\x80c\xAF\xFE\xD0\xE0\x14a\x02\xD7W\x80c\xBD\x06%\xAB\x14a\x02\xDFW\x80c\xCE\x15;\xF4\x14a\x03\x07Wa\0\xF5V[\x80b\xFD\xD5\x8E\x14a\x01ZW\x80c\x02\x16\xB88\x14a\x01\x98W\x80c\x06\x8B\xCD\x8D\x14a\x01\xADW\x80c\x14U\xF1\xFC\x14a\x01\xCDW\x80c.\xC3\x81\x88\x14a\x02\0W[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x01\x85a\x01h6`\x04a\x1F\xF7V[`\x02` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\xABa\x01\xA66`\x04a }V[a\x03\x1AV[\0[a\x01\xC0a\x01\xBB6`\x04a!\xAAV[a\x04\xACV[`@Qa\x01\x8F\x91\x90a!\xC6V[a\x01\xE0a\x01\xDB6`\x04a\"fV[a\x05\xA0V[`@\x80Q\x94\x85R` \x85\x01\x93\x90\x93R\x91\x83\x01R``\x82\x01R`\x80\x01a\x01\x8FV[a\x02\x13a\x02\x0E6`\x04a }V[a\n\x95V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x01\x8FV[a\x01\x85a\x02<6`\x04a\x1F\xF7V[`\x03` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[a\x02\x13a\x02g6`\x04a }V[a\x0C\x8DV[a\x02\x7Fa\x02z6`\x04a!\xAAV[a\x107V[`@\x80Q\x99\x15\x15\x8AR`\x01`\x01`\xA0\x1B\x03\x98\x89\x16` \x8B\x01R\x96\x88\x16\x96\x89\x01\x96\x90\x96R\x93\x86\x16``\x88\x01R\x94\x90\x91\x16`\x80\x86\x01R`\xA0\x85\x01R`\xC0\x84\x01\x92\x90\x92R`\xE0\x83\x01\x91\x90\x91Ra\x01\0\x82\x01Ra\x01 \x01a\x01\x8FV[`\0Ta\x01\x85V[a\x02\xF2a\x02\xED6`\x04a }V[a\x10\xA7V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x01\x8FV[a\x02\x13a\x03\x156`\x04a!\xAAV[a\x12\xB6V[`\x01T`\x02\x03a\x03=W`@Qc\x03\xCB\x96\xDB`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x01\x81\x90UP\x82`\0\x81\x81T\x81\x10a\x03YWa\x03Ya\"\xFEV[`\0\x91\x82R` \x90\x91 `\x08\x90\x91\x02\x01T`\xFF\x16a\x03\x8AW`@Qc!\xC4\xE3W`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x84\x81T\x81\x10a\x03\x9DWa\x03\x9Da\"\xFEV[`\0\x91\x82R` \x90\x91 `\x08\x90\x91\x02\x01Ta\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x163\x14a\x03\xDBW`@Qc#\x01\x9Eg`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x84\x81T\x81\x10a\x03\xEEWa\x03\xEEa\"\xFEV[`\0\x91\x82R` \x90\x91 `\x01`\x08\x90\x92\x02\x01\x01T`@QbB\xD7\x07`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x02\x16\xB88\x90a\x042\x90\x87\x90\x87\x90\x87\x90`\x04\x01a#\x14V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x04\x8AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a%\xA3\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x04\x9EW=`\0\x80>=`\0\xFD[PP`\x01\x80UPPPPPPV[`@\x80Qa\x01 \x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R`\xC0\x81\x01\x82\x90R`\xE0\x81\x01\x82\x90Ra\x01\0\x81\x01\x91\x90\x91R`\0\x82\x81T\x81\x10a\x05\x08Wa\x05\x08a\"\xFEV[`\0\x91\x82R` \x91\x82\x90 `@\x80Qa\x01 \x81\x01\x82R`\x08\x90\x93\x02\x90\x91\x01\x80T`\xFF\x81\x16\x15\x15\x84R`\x01`\x01`\xA0\x1B\x03a\x01\0\x91\x82\x90\x04\x81\x16\x95\x85\x01\x95\x90\x95R`\x01\x82\x01T\x85\x16\x92\x84\x01\x92\x90\x92R`\x02\x81\x01T\x84\x16``\x84\x01R`\x03\x81\x01T\x90\x93\x16`\x80\x83\x01R`\x04\x83\x01T`\xA0\x83\x01R`\x05\x83\x01T`\xC0\x83\x01R`\x06\x83\x01T`\xE0\x83\x01R`\x07\x90\x92\x01T\x91\x81\x01\x91\x90\x91R\x92\x91PPV[`\0\x80`\0\x80`\x01T`\x02\x03a\x05\xC9W`@Qc\x03\xCB\x96\xDB`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x01Ua\x05\xDE``\x86\x01`@\x87\x01a#JV[`\x01`\x01`\xA0\x1B\x03\x16a\x05\xF7`@\x87\x01` \x88\x01a#JV[`\x01`\x01`\xA0\x1B\x03\x16\x03a\x06\x1EW`@Qc3\x91\n\xEF`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80\x80\x80\x80a\x061` \x8B\x01\x8Ba#JV[`\0T`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90c\x9F\x83\x13{\x90a\x06T``\x8E\x01\x8Ea#\xADV[`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x06r\x93\x92\x91\x90a#\x14V[`\xA0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x06\xCAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a%\xA3\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x06\xDEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x02\x91\x90a$\x8EV[\x94P\x94P\x94P\x94P\x94P\x84a\x07EW`\0\x84\x12a\x07\x1E\x85a\x136V[`@Qcw`m)`\xE1\x1B\x81R\x91\x15\x15`\x04\x83\x01R`$\x82\x01R`D\x01[`@Q\x80\x91\x03\x90\xFD[`@\x80Qa\x01 \x81\x01\x82R`\x01\x81R3` \x80\x83\x01\x91\x90\x91R`\0\x92\x82\x01\x90a\x07p\x90\x8E\x01\x8Ea#JV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8C` \x01` \x81\x01\x90a\x07\x91\x91\x90a#JV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01a\x07\xAF``\x8E\x01`@\x8F\x01a#JV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R` \x80\x83\x01\x88\x90R`@\x80\x84\x01\x88\x90R``\x80\x85\x01\x88\x90Rg\r\xE0\xB6\xB3\xA7d\0\0`\x80\x95\x86\x01R`\0\x80T`\x01\x80\x82\x01\x83U\x82\x80R\x88Q\x7F)\r\xEC\xD9T\x8Bb\xA8\xD6\x03E\xA9\x888o\xC8K\xA6\xBC\x95H@\x08\xF66/\x93\x16\x0E\xF3\xE5c`\x08\x90\x93\x02\x92\x83\x01\x80T\x97\x8B\x01Q`\x01`\x01`\xA8\x1B\x03\x19\x90\x98\x16\x91\x15\x15a\x01\0`\x01`\xA8\x1B\x03\x19\x16\x91\x90\x91\x17a\x01\0\x97\x89\x16\x88\x02\x17\x90U\x93\x88\x01Q\x7F)\r\xEC\xD9T\x8Bb\xA8\xD6\x03E\xA9\x888o\xC8K\xA6\xBC\x95H@\x08\xF66/\x93\x16\x0E\xF3\xE5d\x82\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16\x92\x89\x16\x92\x90\x92\x17\x90U\x92\x88\x01Q\x7F)\r\xEC\xD9T\x8Bb\xA8\xD6\x03E\xA9\x888o\xC8K\xA6\xBC\x95H@\x08\xF66/\x93\x16\x0E\xF3\xE5e\x82\x01\x80T\x85\x16\x91\x88\x16\x91\x90\x91\x17\x90U\x95\x87\x01Q\x7F)\r\xEC\xD9T\x8Bb\xA8\xD6\x03E\xA9\x888o\xC8K\xA6\xBC\x95H@\x08\xF66/\x93\x16\x0E\xF3\xE5f\x87\x01\x80T\x90\x93\x16\x95\x16\x94\x90\x94\x17\x90U`\xA0\x85\x01Q\x7F)\r\xEC\xD9T\x8Bb\xA8\xD6\x03E\xA9\x888o\xC8K\xA6\xBC\x95H@\x08\xF66/\x93\x16\x0E\xF3\xE5g\x85\x01U`\xC0\x85\x01Q\x7F)\r\xEC\xD9T\x8Bb\xA8\xD6\x03E\xA9\x888o\xC8K\xA6\xBC\x95H@\x08\xF66/\x93\x16\x0E\xF3\xE5h\x85\x01U`\xE0\x85\x01Q\x7F)\r\xEC\xD9T\x8Bb\xA8\xD6\x03E\xA9\x888o\xC8K\xA6\xBC\x95H@\x08\xF66/\x93\x16\x0E\xF3\xE5i\x85\x01U\x90\x84\x01Q\x7F)\r\xEC\xD9T\x8Bb\xA8\xD6\x03E\xA9\x888o\xC8K\xA6\xBC\x95H@\x08\xF66/\x93\x16\x0E\xF3\xE5j\x90\x93\x01\x92\x90\x92U\x80T\x92\x93P\x91a\t\xA8\x91\x90a$\xEEV[3`\0\x81\x81R`\x02` \x90\x81R`@\x80\x83 \x85\x84R\x82R\x80\x83 \x88\x90U\x92\x82R`\x03\x81R\x82\x82 \x84\x83R\x81R\x90\x82\x90 g\r\xE0\xB6\xB3\xA7d\0\0\x90U\x91\x92Pa\n\0\x91a\t\xF8\x91\x8F\x01\x90\x8F\x01a#JV[30\x88a\x13uV[a\n\x1Ba\n\x13``\x8E\x01`@\x8F\x01a#JV[30\x87a\x13uV[a\n(` \x8D\x01\x8Da#JV[`@\x80Q\x83\x81R` \x81\x01\x88\x90R\x90\x81\x01\x86\x90R``\x81\x01\x85\x90R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x903\x90\x7F\x9C\xF5\x96\x1A\x85\xC7\xF8\xE7\xA38\xE8\xA6_[(\xEC\x17\x98_\xA7q\xAB\x0C{\xB9>\xB8kv\xCF\xECX\x90`\x80\x01`@Q\x80\x91\x03\x90\xA3`\x01\x80U\x9B\x93\x9AP\x91\x98P\x96P\x90\x94PPPPPV[`\0\x80`\0`\x01T`\x02\x03a\n\xBDW`@Qc\x03\xCB\x96\xDB`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x01\x81\x90UP\x85`\0\x81\x81T\x81\x10a\n\xD9Wa\n\xD9a\"\xFEV[`\0\x91\x82R` \x90\x91 `\x08\x90\x91\x02\x01T`\xFF\x16a\x0B\nW`@Qc!\xC4\xE3W`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0B\x13\x87a\x14\x03V[`\0\x80`\0a\x0B%\x8A`\x01\x8B\x8Ba\x15\rV[\x92P\x92P\x92P\x80`\x02`\x003`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x8C\x81R` \x01\x90\x81R` \x01`\0 `\0\x82\x82Ta\x0Bq\x91\x90a%\x01V[\x90\x91UPP`\0\x80T\x8B\x90\x81\x10a\x0B\x8AWa\x0B\x8Aa\"\xFEV[`\0\x91\x82R` \x80\x83 `\x07`\x08\x90\x93\x02\x01\x91\x90\x91\x01T3\x83R`\x03\x82R`@\x80\x84 \x8E\x85R\x90\x92R\x90\x82 U\x80Ta\x0B\xF3\x91\x90\x8C\x90\x81\x10a\x0B\xCEWa\x0B\xCEa\"\xFEV[`\0\x91\x82R` \x90\x91 `\x02`\x08\x90\x92\x02\x01\x01T`\x01`\x01`\xA0\x1B\x03\x1630\x86a\x13uV[a\x0C.`\0\x8B\x81T\x81\x10a\x0C\tWa\x0C\ta\"\xFEV[`\0\x91\x82R` \x90\x91 `\x03`\x08\x90\x92\x02\x01\x01T`\x01`\x01`\xA0\x1B\x03\x1630\x85a\x13uV[`@\x80Q\x8B\x81R` \x81\x01\x85\x90R\x90\x81\x01\x83\x90R``\x81\x01\x82\x90R3\x90\x7F\x95\x97W\x7F3\x93 w^c\xD3\xFE\xD7\xD5\xDD\xE66[\xAD\xCC\x9F\xCC\xDBf\xB3H\x94c\x0C\xA9\x8Bo\x90`\x80\x01[`@Q\x80\x91\x03\x90\xA2`\x01\x80U\x91\x99\x90\x98P\x90\x96P\x94PPPPPV[`\0\x80`\0`\x01T`\x02\x03a\x0C\xB5W`@Qc\x03\xCB\x96\xDB`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x01\x81\x90UP\x85`\0\x81\x81T\x81\x10a\x0C\xD1Wa\x0C\xD1a\"\xFEV[`\0\x91\x82R` \x90\x91 `\x08\x90\x91\x02\x01T`\xFF\x16a\r\x02W`@Qc!\xC4\xE3W`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\r\x0B\x87a\x14\x03V[`\0\x80`\0a\r\x1D\x8A`\0\x8B\x8Ba\x15\rV[\x92P\x92P\x92P\x80`\x02`\x003`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x8C\x81R` \x01\x90\x81R` \x01`\0 `\0\x82\x82Ta\ri\x91\x90a$\xEEV[\x90\x91UPP`\0\x80T\x8B\x90\x81\x10a\r\x82Wa\r\x82a\"\xFEV[`\0\x91\x82R` \x80\x83 `\x07`\x08\x90\x93\x02\x01\x91\x90\x91\x01T3\x83R`\x03\x82R`@\x80\x84 \x8E\x85R\x90\x92R\x90\x82 U\x80T\x8B\x90\x81\x10a\r\xC1Wa\r\xC1a\"\xFEV[`\0\x91\x82R` \x90\x91 `\x08\x90\x91\x02\x01`\x02\x01T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x85\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0E\\W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a%\xA3\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x0EpW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\x94\x91\x90a%\x14V[P`\0\x8A\x81T\x81\x10a\x0E\xA8Wa\x0E\xA8a\"\xFEV[`\0\x91\x82R` \x90\x91 `\x08\x90\x91\x02\x01`\x03\x01T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0FCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a%\xA3\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x0FWW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F{\x91\x90a%\x14V[Pa\x0F\xB6`\0\x8B\x81T\x81\x10a\x0F\x92Wa\x0F\x92a\"\xFEV[`\0\x91\x82R` \x90\x91 `\x02`\x08\x90\x92\x02\x01\x01T`\x01`\x01`\xA0\x1B\x03\x163\x85a\x17\xE8V[a\x0F\xF0`\0\x8B\x81T\x81\x10a\x0F\xCCWa\x0F\xCCa\"\xFEV[`\0\x91\x82R` \x90\x91 `\x03`\x08\x90\x92\x02\x01\x01T`\x01`\x01`\xA0\x1B\x03\x163\x84a\x17\xE8V[`@\x80Q\x8B\x81R` \x81\x01\x85\x90R\x90\x81\x01\x83\x90R``\x81\x01\x82\x90R3\x90\x7F\xAC\xBE\x12~\x93\xA8\xA0\xB2x\xD8\xE0n' [=\xF9\xD1\xF3\x81$\x14\xBC\x89\x17\xC7t\xA87\x101n\x90`\x80\x01a\x0CqV[`\0\x81\x81T\x81\x10a\x10GW`\0\x80\xFD[`\0\x91\x82R` \x90\x91 `\x08\x90\x91\x02\x01\x80T`\x01\x82\x01T`\x02\x83\x01T`\x03\x84\x01T`\x04\x85\x01T`\x05\x86\x01T`\x06\x87\x01T`\x07\x90\x97\x01T`\xFF\x87\x16\x98Pa\x01\0\x90\x96\x04`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x97\x95\x81\x16\x96\x94\x81\x16\x95\x93\x16\x93\x91\x92\x90\x91\x89V[`\0\x80`\x01T`\x02\x03a\x10\xCDW`@Qc\x03\xCB\x96\xDB`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x01\x81\x90UP\x84`\0\x81\x81T\x81\x10a\x10\xE9Wa\x10\xE9a\"\xFEV[`\0\x91\x82R` \x90\x91 `\x08\x90\x91\x02\x01T`\xFF\x16a\x11\x1AW`@Qc!\xC4\xE3W`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80`\0\x80`\0\x80\x8B\x81T\x81\x10a\x114Wa\x114a\"\xFEV[`\0\x91\x82R` \x90\x91 `\x01`\x08\x90\x92\x02\x01\x01T`@Qc2\x14\x89\x0F`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c2\x14\x89\x0F\x90a\x11y\x90\x8E\x90\x8E\x90\x8E\x90`\x04\x01a#\x14V[`\xC0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x11\xCFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a%\xA3\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x11\xE3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\x07\x91\x90a%2V[\x95P\x95P\x95PP\x94P\x94P\x84a\x12$W`\0\x84\x12a\x07\x1E\x85a\x136V[a\x12.\x8B\x82a\x18lV[`\0\x80`\0a\x12>\x8E\x87\x87a\x19=V[\x94P\x94PPP\x92P\x8D3`\x01`\x01`\xA0\x1B\x03\x16\x7FL}\xEF\x84\xE4++\xC0\xA5\xAA\xB2\"\x86\x8D\xD7\xA0\x92\xB53w\xA4\xB57\xAB\xCD\x944Zz\x85'\xED\x85\x85\x85`@Qa\x12\x97\x93\x92\x91\x90\x92\x15\x15\x83R` \x83\x01\x91\x90\x91R`@\x82\x01R``\x01\x90V[`@Q\x80\x91\x03\x90\xA3`\x01\x80U\x90\x9D\x90\x9CP\x9APPPPPPPPPPPV[`\0\x80`\0\x80\x84\x81T\x81\x10a\x12\xCDWa\x12\xCDa\"\xFEV[\x90`\0R` `\0 \x90`\x08\x02\x01`\x04\x01T`\0\x85\x81T\x81\x10a\x12\xF2Wa\x12\xF2a\"\xFEV[\x90`\0R` `\0 \x90`\x08\x02\x01`\x05\x01T`\0\x86\x81T\x81\x10a\x13\x17Wa\x13\x17a\"\xFEV[\x90`\0R` `\0 \x90`\x08\x02\x01`\x06\x01T\x92P\x92P\x92P\x91\x93\x90\x92PV[`\0`\x01`\xFF\x1B\x82\x03a\x13\\W`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x12\x15a\x13mWP\x19`\x01\x01\x90V[P\x80[\x91\x90PV[`\0`@Qc#\xB8r\xDD`\xE0\x1B`\0R\x84`\x04R\x83`$R\x82`DR` `\0`d`\0\x80\x8AZ\xF1=\x15`\x1F=\x11`\x01`\0Q\x14\x16\x17\x16\x91P`\0``R\x80`@RP\x80a\x13\xFCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rs\x15\x14\x90S\x94\xD1\x91T\x97\xD1\x94\x93\xD3W\xD1\x90RS\x11Q`b\x1B`D\x82\x01R`d\x01a\x07<V[PPPPPV[3`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 \x84\x84R\x90\x91R\x90 T\x15\x80\x15\x90a\x14lWP3`\0\x90\x81R`\x03` \x90\x81R`@\x80\x83 \x84\x84R\x90\x91R\x81 T\x81T\x90\x91\x90\x83\x90\x81\x10a\x14WWa\x14Wa\"\xFEV[\x90`\0R` `\0 \x90`\x08\x02\x01`\x07\x01T\x14\x15[\x15a\x15\nW3`\0\x90\x81R`\x03` \x90\x81R`@\x80\x83 \x84\x84R\x90\x91R\x81 T\x81Ta\x14\xC5\x91\x90\x83\x90\x85\x90\x81\x10a\x14\xA5Wa\x14\xA5a\"\xFEV[\x90`\0R` `\0 \x90`\x08\x02\x01`\x07\x01Ta\x1E\xF2\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[3`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 \x86\x84R\x90\x91R\x90 T\x90\x91Pa\x14\xED\x90\x82a\x1E\xF2V[3`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 \x86\x84R\x90\x91R\x90 UP[PV[`\0\x80`\0\x80`\0\x80`\0\x80`\0\x8C\x81T\x81\x10a\x15,Wa\x15,a\"\xFEV[`\0\x91\x82R` \x90\x91 `\x01`\x08\x90\x92\x02\x01\x01T`@Qb#\x8Bu`\xEA\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x8E-\xD4\0\x90a\x15p\x90\x8F\x90\x8E\x90\x8E\x90`\x04\x01a#\x14V[`\xA0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x15\xC6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a%\xA3\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x15\xDAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\xFE\x91\x90a$\x8EV[\x94P\x94P\x94P\x94P\x94P\x84a\x16\x1AW`\0\x84\x12a\x07\x1E\x85a\x136V[\x8Aa\x16TW\x82`\0\x8D\x81T\x81\x10a\x163Wa\x163a\"\xFEV[\x90`\0R` `\0 \x90`\x08\x02\x01`\x04\x01Ta\x16O\x91\x90a$\xEEV[a\x16\x84V[`\0\x8C\x81T\x81\x10a\x16gWa\x16ga\"\xFEV[\x90`\0R` `\0 \x90`\x08\x02\x01`\x04\x01T\x83a\x16\x84\x91\x90a$\xEEV[\x97P\x8Aa\x16\xC0W\x81`\0\x8D\x81T\x81\x10a\x16\x9FWa\x16\x9Fa\"\xFEV[\x90`\0R` `\0 \x90`\x08\x02\x01`\x05\x01Ta\x16\xBB\x91\x90a$\xEEV[a\x16\xF0V[`\0\x8C\x81T\x81\x10a\x16\xD3Wa\x16\xD3a\"\xFEV[\x90`\0R` `\0 \x90`\x08\x02\x01`\x05\x01T\x82a\x16\xF0\x91\x90a$\xEEV[\x96P\x8Aa\x17,W\x80`\0\x8D\x81T\x81\x10a\x17\x0BWa\x17\x0Ba\"\xFEV[\x90`\0R` `\0 \x90`\x08\x02\x01`\x06\x01Ta\x17'\x91\x90a$\xEEV[a\x17\\V[`\0\x8C\x81T\x81\x10a\x17?Wa\x17?a\"\xFEV[\x90`\0R` `\0 \x90`\x08\x02\x01`\x06\x01T\x81a\x17\\\x91\x90a$\xEEV[\x95P\x82`\0\x8D\x81T\x81\x10a\x17rWa\x17ra\"\xFEV[\x90`\0R` `\0 \x90`\x08\x02\x01`\x04\x01\x81\x90UP\x81`\0\x8D\x81T\x81\x10a\x17\x9BWa\x17\x9Ba\"\xFEV[\x90`\0R` `\0 \x90`\x08\x02\x01`\x05\x01\x81\x90UP\x80`\0\x8D\x81T\x81\x10a\x17\xC4Wa\x17\xC4a\"\xFEV[\x90`\0R` `\0 \x90`\x08\x02\x01`\x06\x01\x81\x90UPPPPPP\x94P\x94P\x94\x91PPV[`\0`@Qc\xA9\x05\x9C\xBB`\xE0\x1B`\0R\x83`\x04R\x82`$R` `\0`D`\0\x80\x89Z\xF1=\x15`\x1F=\x11`\x01`\0Q\x14\x16\x17\x16\x91P`\0``R\x80`@RP\x80a\x18fW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rn\x15\x14\x90S\x94\xD1\x91T\x97\xD1\x90RS\x11Q`\x8A\x1B`D\x82\x01R`d\x01a\x07<V[PPPPV[`\0\x80\x83\x81T\x81\x10a\x18\x80Wa\x18\x80a\"\xFEV[\x90`\0R` `\0 \x90`\x08\x02\x01`\x06\x01T\x90P\x81`\0\x84\x81T\x81\x10a\x18\xA8Wa\x18\xA8a\"\xFEV[\x90`\0R` `\0 \x90`\x08\x02\x01`\x06\x01\x81\x90UP`\0a\x18\xF6\x82`\0\x86\x81T\x81\x10a\x18\xD6Wa\x18\xD6a\"\xFEV[\x90`\0R` `\0 \x90`\x08\x02\x01`\x06\x01Ta\x1F\x10\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa\x19\x0F\x81`\0\x86\x81T\x81\x10a\x14\xA5Wa\x14\xA5a\"\xFEV[`\0\x85\x81T\x81\x10a\x19\"Wa\x19\"a\"\xFEV[\x90`\0R` `\0 \x90`\x08\x02\x01`\x07\x01\x81\x90UPPPPPV[`\0\x80`\0\x80`\0\x80`\0\x89\x81T\x81\x10a\x19YWa\x19Ya\"\xFEV[\x90`\0R` `\0 \x90`\x08\x02\x01`\x04\x01T\x90P`\0\x80\x8A\x81T\x81\x10a\x19\x81Wa\x19\x81a\"\xFEV[\x90`\0R` `\0 \x90`\x08\x02\x01`\x05\x01T\x90P\x81\x89\x11\x96P\x86a\x19\xD4W`\0\x8A\x81T\x81\x10a\x19\xB2Wa\x19\xB2a\"\xFEV[`\0\x91\x82R` \x90\x91 `\x03`\x08\x90\x92\x02\x01\x01T`\x01`\x01`\xA0\x1B\x03\x16a\x1A\x05V[`\0\x8A\x81T\x81\x10a\x19\xE7Wa\x19\xE7a\"\xFEV[`\0\x91\x82R` \x90\x91 `\x02`\x08\x90\x92\x02\x01\x01T`\x01`\x01`\xA0\x1B\x03\x16[\x95P\x86a\x1AAW`\0\x8A\x81T\x81\x10a\x1A\x1FWa\x1A\x1Fa\"\xFEV[`\0\x91\x82R` \x90\x91 `\x02`\x08\x90\x92\x02\x01\x01T`\x01`\x01`\xA0\x1B\x03\x16a\x1ArV[`\0\x8A\x81T\x81\x10a\x1ATWa\x1ATa\"\xFEV[`\0\x91\x82R` \x90\x91 `\x03`\x08\x90\x92\x02\x01\x01T`\x01`\x01`\xA0\x1B\x03\x16[\x94P\x86a\x1A\x88Wa\x1A\x83\x81\x89a$\xEEV[a\x1A\x92V[a\x1A\x92\x82\x8Aa$\xEEV[\x93P\x86a\x1A\xA8Wa\x1A\xA3\x89\x83a$\xEEV[a\x1A\xB2V[a\x1A\xB2\x88\x82a$\xEEV[\x92P\x86\x15a\x1A\xFDW\x87\x81\x11a\x1A\xF8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01Rk\x06\x96\xE7f\x16\xC6\x96B\x077v\x17`\xA4\x1B`D\x82\x01R`d\x01a\x07<V[a\x1B;V[\x88\x82\x11a\x1B;W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01Rk\x06\x96\xE7f\x16\xC6\x96B\x077v\x17`\xA4\x1B`D\x82\x01R`d\x01a\x07<V[\x88`\0\x8B\x81T\x81\x10a\x1BOWa\x1BOa\"\xFEV[\x90`\0R` `\0 \x90`\x08\x02\x01`\x04\x01\x81\x90UP\x87`\0\x8B\x81T\x81\x10a\x1BxWa\x1Bxa\"\xFEV[`\0\x91\x82R` \x82 `\x08\x91\x90\x91\x02\x01`\x05\x01\x91\x90\x91U`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x88\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x1C\x0CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a%\xA3\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x1C W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1CD\x91\x90a%\x86V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x88\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x1C\xC7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a%\xA3\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x1C\xDBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C\xFF\x91\x90a%\x86V[\x90Pa\x1D\r\x8830\x89a\x13uV[a\x1D\x18\x873\x87a\x17\xE8V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x90`\x01`\x01`\xA0\x1B\x03\x8A\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x1D\x98W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a%\xA3\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x1D\xACW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1D\xD0\x91\x90a%\x86V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x8A\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x1ESW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a%\xA3\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x1EgW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E\x8B\x91\x90a%\x86V[\x90Pa\x1E\x97\x88\x85a%\x01V[\x82\x10\x15a\x1E\xB7W`@Qc =\x90\x1D`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1E\xC1\x87\x84a$\xEEV[\x81\x10\x15a\x1E\xE1W`@Qc\xF3\xCB\xBC\x87`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPP\x93\x97\x92\x96P\x93P\x93PV[`\0a\x1F\x07\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x1F!V[\x90P[\x92\x91PPV[`\0a\x1F\x07\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x1F9W`\0\x80\xFD[\x04\x92\x91PPV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x13pW`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a \rWa \ra\x1F@V[a \x16\x83a\x1F\xE0V[\x94` \x93\x90\x93\x015\x93PPPV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rjrray stride`\xA8\x1B`d\x82\x01R`\x84\x81\xFD[`\0\x80`\0`@\x84\x86\x03\x12\x15a \x95Wa \x95a\x1F@V[\x835\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a \xB7Wa \xB7a\x1F\x90V[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a!\x1FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x81\xFD[\x815\x81\x81\x11\x15a!\x82W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x0ENL/$\r\x8C\xAD\xCC\xEE\x8D`\xAB\x1B`d\x82\x01R`\x84\x81\xFD[\x87` \x82\x85\x01\x01\x11\x15a!\x97Wa!\x97a $V[` \x83\x01\x94P\x80\x93PPPP\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a!\xBFWa!\xBFa\x1F@V[P5\x91\x90PV[\x81Q\x15\x15\x81R` \x80\x83\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x90\x82\x01R`@\x80\x83\x01Qa\x01 \x83\x01\x91a!\xFF\x90\x84\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P``\x83\x01Qa\"\x1A``\x84\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P`\x80\x83\x01Qa\"5`\x80\x84\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P`\xA0\x83\x01Q`\xA0\x83\x01R`\xC0\x83\x01Q`\xC0\x83\x01R`\xE0\x83\x01Q`\xE0\x83\x01Ra\x01\0\x80\x84\x01Q\x81\x84\x01RP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\"{Wa\"{a\x1F@V[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\"\x95Wa\"\x95a\x1F\x90V[\x82\x01`\x80\x81\x85\x03\x12\x15a\"\xF7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FABI decoding: struct calldata to`D\x82\x01Rf\x1B\xC8\x1C\xDA\x1B\xDC\x9D`\xCA\x1B`d\x82\x01R`\x84\x81\xFD[\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[\x83\x81R`@` \x82\x01R\x81`@\x82\x01R\x81\x83``\x83\x017`\0\x81\x83\x01``\x90\x81\x01\x91\x90\x91R`\x1F\x90\x92\x01`\x1F\x19\x16\x01\x01\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a#_Wa#_a\x1F@V[a\x1F\x07\x82a\x1F\xE0V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FCalldata tail too short\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x81\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a$\x04W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FInvalid calldata tail offset\0\0\0\0`D\x82\x01R`d\x81\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a$_W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FInvalid calldata tail length\0\0\0\0`D\x82\x01R`d\x81\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a$wWa$wa#hV[\x92P\x92\x90PV[\x80Q\x80\x15\x15\x81\x14a\x13pW`\0\x80\xFD[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a$\xA9Wa$\xA9a\x1F@V[a$\xB2\x86a$~V[` \x87\x01Q`@\x88\x01Q``\x89\x01Q`\x80\x90\x99\x01Q\x92\x9A\x91\x99P\x97\x96P\x90\x94P\x92PPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x1F\nWa\x1F\na$\xD8V[\x80\x82\x01\x80\x82\x11\x15a\x1F\nWa\x1F\na$\xD8V[`\0` \x82\x84\x03\x12\x15a%)Wa%)a\x1F@V[a\x1F\x07\x82a$~V[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a%NWa%Na\x1F@V[a%W\x87a$~V[\x95P` \x87\x01Q\x94P`@\x87\x01Q\x93P``\x87\x01Q\x92P`\x80\x87\x01Q\x91P`\xA0\x87\x01Q\x90P\x92\x95P\x92\x95P\x92\x95V[`\0` \x82\x84\x03\x12\x15a%\x9BWa%\x9Ba\x1F@V[PQ\x91\x90PV\xFETarget contract does not contain";
    /// The deployed bytecode of the contract.
    pub static DFMM_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct DFMM<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for DFMM<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for DFMM<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for DFMM<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for DFMM<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(DFMM)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> DFMM<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    DFMM_ABI.clone(),
                    client,
                ),
            )
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                DFMM_ABI.clone(),
                DFMM_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `allocate` (0x2ec38188) function
        pub fn allocate(
            &self,
            pool_id: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([46, 195, 129, 136], (pool_id, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `balanceOf` (0x00fdd58e) function
        pub fn balance_of(
            &self,
            account: ::ethers::core::types::Address,
            pool_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([0, 253, 213, 142], (account, pool_id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deallocate` (0x9d942f9a) function
        pub fn deallocate(
            &self,
            pool_id: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([157, 148, 47, 154], (pool_id, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPool` (0x068bcd8d) function
        pub fn get_pool(
            &self,
            pool_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, Pool> {
            self.0
                .method_hash([6, 139, 205, 141], pool_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getReservesAndLiquidity` (0xce153bf4) function
        pub fn get_reserves_and_liquidity(
            &self,
            pool_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([206, 21, 59, 244], pool_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `init` (0x1455f1fc) function
        pub fn init(
            &self,
            params: InitParams,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([20, 85, 241, 252], (params,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lastFeeGrowthOf` (0x5f00e1f3) function
        pub fn last_fee_growth_of(
            &self,
            account: ::ethers::core::types::Address,
            pool_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([95, 0, 225, 243], (account, pool_id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `nonce` (0xaffed0e0) function
        pub fn nonce(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([175, 254, 208, 224], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pools` (0xac4afa38) function
        pub fn pools(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                bool,
                ::ethers::core::types::Address,
                ::ethers::core::types::Address,
                ::ethers::core::types::Address,
                ::ethers::core::types::Address,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([172, 74, 250, 56], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `swap` (0xbd0625ab) function
        pub fn swap(
            &self,
            pool_id: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([189, 6, 37, 171], (pool_id, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `update` (0x0216b838) function
        pub fn update(
            &self,
            pool_id: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([2, 22, 184, 56], (pool_id, data))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `Allocate` event
        pub fn allocate_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AllocateFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Deallocate` event
        pub fn deallocate_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DeallocateFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Init` event
        pub fn init_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, InitFilter> {
            self.0.event()
        }
        ///Gets the contract's `LogPoolStats` event
        pub fn log_pool_stats_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogPoolStatsFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Swap` event
        pub fn swap_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, SwapFilter> {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, DFMMEvents> {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for DFMM<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `Invalid` with signature `Invalid(bool,uint256)` and selector `0xeec0da52`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "Invalid", abi = "Invalid(bool,uint256)")]
    pub struct Invalid {
        pub negative: bool,
        pub swap_constant_growth: ::ethers::core::types::U256,
    }
    ///Custom Error type `InvalidSwapInputTransfer` with signature `InvalidSwapInputTransfer()` and selector `0x80f64074`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "InvalidSwapInputTransfer", abi = "InvalidSwapInputTransfer()")]
    pub struct InvalidSwapInputTransfer;
    ///Custom Error type `InvalidSwapOutputTransfer` with signature `InvalidSwapOutputTransfer()` and selector `0xf3cbbc87`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "InvalidSwapOutputTransfer", abi = "InvalidSwapOutputTransfer()")]
    pub struct InvalidSwapOutputTransfer;
    ///Custom Error type `InvalidTokens` with signature `InvalidTokens()` and selector `0x672215de`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "InvalidTokens", abi = "InvalidTokens()")]
    pub struct InvalidTokens;
    ///Custom Error type `Locked` with signature `Locked()` and selector `0x0f2e5b6c`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "Locked", abi = "Locked()")]
    pub struct Locked;
    ///Custom Error type `Min` with signature `Min()` and selector `0x4d2d75b1`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "Min", abi = "Min()")]
    pub struct Min;
    ///Custom Error type `NotController` with signature `NotController()` and selector `0x23019e67`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "NotController", abi = "NotController()")]
    pub struct NotController;
    ///Custom Error type `NotInitialized` with signature `NotInitialized()` and selector `0x87138d5c`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "NotInitialized", abi = "NotInitialized()")]
    pub struct NotInitialized;
    ///Container type for all of the contract's custom errors
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub enum DFMMErrors {
        Invalid(Invalid),
        InvalidSwapInputTransfer(InvalidSwapInputTransfer),
        InvalidSwapOutputTransfer(InvalidSwapOutputTransfer),
        InvalidTokens(InvalidTokens),
        Locked(Locked),
        Min(Min),
        NotController(NotController),
        NotInitialized(NotInitialized),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for DFMMErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <Invalid as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Invalid(decoded));
            }
            if let Ok(decoded) = <InvalidSwapInputTransfer as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidSwapInputTransfer(decoded));
            }
            if let Ok(decoded) = <InvalidSwapOutputTransfer as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidSwapOutputTransfer(decoded));
            }
            if let Ok(decoded) = <InvalidTokens as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidTokens(decoded));
            }
            if let Ok(decoded) = <Locked as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Locked(decoded));
            }
            if let Ok(decoded) = <Min as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Min(decoded));
            }
            if let Ok(decoded) = <NotController as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NotController(decoded));
            }
            if let Ok(decoded) = <NotInitialized as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NotInitialized(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for DFMMErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::Invalid(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InvalidSwapInputTransfer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidSwapOutputTransfer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidTokens(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Locked(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Min(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NotController(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NotInitialized(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for DFMMErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <Invalid as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <InvalidSwapInputTransfer as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidSwapOutputTransfer as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidTokens as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector == <Locked as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector == <Min as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NotController as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NotInitialized as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for DFMMErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Invalid(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidSwapInputTransfer(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidSwapOutputTransfer(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidTokens(element) => ::core::fmt::Display::fmt(element, f),
                Self::Locked(element) => ::core::fmt::Display::fmt(element, f),
                Self::Min(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotController(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotInitialized(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for DFMMErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<Invalid> for DFMMErrors {
        fn from(value: Invalid) -> Self {
            Self::Invalid(value)
        }
    }
    impl ::core::convert::From<InvalidSwapInputTransfer> for DFMMErrors {
        fn from(value: InvalidSwapInputTransfer) -> Self {
            Self::InvalidSwapInputTransfer(value)
        }
    }
    impl ::core::convert::From<InvalidSwapOutputTransfer> for DFMMErrors {
        fn from(value: InvalidSwapOutputTransfer) -> Self {
            Self::InvalidSwapOutputTransfer(value)
        }
    }
    impl ::core::convert::From<InvalidTokens> for DFMMErrors {
        fn from(value: InvalidTokens) -> Self {
            Self::InvalidTokens(value)
        }
    }
    impl ::core::convert::From<Locked> for DFMMErrors {
        fn from(value: Locked) -> Self {
            Self::Locked(value)
        }
    }
    impl ::core::convert::From<Min> for DFMMErrors {
        fn from(value: Min) -> Self {
            Self::Min(value)
        }
    }
    impl ::core::convert::From<NotController> for DFMMErrors {
        fn from(value: NotController) -> Self {
            Self::NotController(value)
        }
    }
    impl ::core::convert::From<NotInitialized> for DFMMErrors {
        fn from(value: NotInitialized) -> Self {
            Self::NotInitialized(value)
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "Allocate",
        abi = "Allocate(address,uint256,uint256,uint256,uint256)"
    )]
    pub struct AllocateFilter {
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        pub pool_id: ::ethers::core::types::U256,
        pub delta_x: ::ethers::core::types::U256,
        pub delta_y: ::ethers::core::types::U256,
        pub delta_l: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "Deallocate",
        abi = "Deallocate(address,uint256,uint256,uint256,uint256)"
    )]
    pub struct DeallocateFilter {
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        pub pool_id: ::ethers::core::types::U256,
        pub delta_x: ::ethers::core::types::U256,
        pub delta_y: ::ethers::core::types::U256,
        pub delta_l: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "Init",
        abi = "Init(address,address,uint256,uint256,uint256,uint256)"
    )]
    pub struct InitFilter {
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub strategy: ::ethers::core::types::Address,
        pub pool_id: ::ethers::core::types::U256,
        pub reserve_x: ::ethers::core::types::U256,
        pub reserve_y: ::ethers::core::types::U256,
        pub total_liquidity: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "LogPoolStats",
        abi = "LogPoolStats(uint256,uint256,uint256,int256,uint256,uint256,uint256,uint256)"
    )]
    pub struct LogPoolStatsFilter {
        pub rx: ::ethers::core::types::U256,
        pub ry: ::ethers::core::types::U256,
        pub l: ::ethers::core::types::U256,
        pub invariant: ::ethers::core::types::I256,
        pub sigma: ::ethers::core::types::U256,
        pub strike: ::ethers::core::types::U256,
        pub tau: ::ethers::core::types::U256,
        pub timestamp: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "Swap", abi = "Swap(address,uint256,bool,uint256,uint256)")]
    pub struct SwapFilter {
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub pool_id: ::ethers::core::types::U256,
        pub is_swap_x_for_y: bool,
        pub input_amount: ::ethers::core::types::U256,
        pub output_amount: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's events
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub enum DFMMEvents {
        AllocateFilter(AllocateFilter),
        DeallocateFilter(DeallocateFilter),
        InitFilter(InitFilter),
        LogPoolStatsFilter(LogPoolStatsFilter),
        SwapFilter(SwapFilter),
    }
    impl ::ethers::contract::EthLogDecode for DFMMEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AllocateFilter::decode_log(log) {
                return Ok(DFMMEvents::AllocateFilter(decoded));
            }
            if let Ok(decoded) = DeallocateFilter::decode_log(log) {
                return Ok(DFMMEvents::DeallocateFilter(decoded));
            }
            if let Ok(decoded) = InitFilter::decode_log(log) {
                return Ok(DFMMEvents::InitFilter(decoded));
            }
            if let Ok(decoded) = LogPoolStatsFilter::decode_log(log) {
                return Ok(DFMMEvents::LogPoolStatsFilter(decoded));
            }
            if let Ok(decoded) = SwapFilter::decode_log(log) {
                return Ok(DFMMEvents::SwapFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for DFMMEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AllocateFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeallocateFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::InitFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogPoolStatsFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SwapFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AllocateFilter> for DFMMEvents {
        fn from(value: AllocateFilter) -> Self {
            Self::AllocateFilter(value)
        }
    }
    impl ::core::convert::From<DeallocateFilter> for DFMMEvents {
        fn from(value: DeallocateFilter) -> Self {
            Self::DeallocateFilter(value)
        }
    }
    impl ::core::convert::From<InitFilter> for DFMMEvents {
        fn from(value: InitFilter) -> Self {
            Self::InitFilter(value)
        }
    }
    impl ::core::convert::From<LogPoolStatsFilter> for DFMMEvents {
        fn from(value: LogPoolStatsFilter) -> Self {
            Self::LogPoolStatsFilter(value)
        }
    }
    impl ::core::convert::From<SwapFilter> for DFMMEvents {
        fn from(value: SwapFilter) -> Self {
            Self::SwapFilter(value)
        }
    }
    ///Container type for all input parameters for the `allocate` function with signature `allocate(uint256,bytes)` and selector `0x2ec38188`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "allocate", abi = "allocate(uint256,bytes)")]
    pub struct AllocateCall {
        pub pool_id: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `balanceOf` function with signature `balanceOf(address,uint256)` and selector `0x00fdd58e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "balanceOf", abi = "balanceOf(address,uint256)")]
    pub struct BalanceOfCall {
        pub account: ::ethers::core::types::Address,
        pub pool_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `deallocate` function with signature `deallocate(uint256,bytes)` and selector `0x9d942f9a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "deallocate", abi = "deallocate(uint256,bytes)")]
    pub struct DeallocateCall {
        pub pool_id: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `getPool` function with signature `getPool(uint256)` and selector `0x068bcd8d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getPool", abi = "getPool(uint256)")]
    pub struct GetPoolCall {
        pub pool_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getReservesAndLiquidity` function with signature `getReservesAndLiquidity(uint256)` and selector `0xce153bf4`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "getReservesAndLiquidity",
        abi = "getReservesAndLiquidity(uint256)"
    )]
    pub struct GetReservesAndLiquidityCall {
        pub pool_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `init` function with signature `init((address,address,address,bytes))` and selector `0x1455f1fc`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "init", abi = "init((address,address,address,bytes))")]
    pub struct InitCall {
        pub params: InitParams,
    }
    ///Container type for all input parameters for the `lastFeeGrowthOf` function with signature `lastFeeGrowthOf(address,uint256)` and selector `0x5f00e1f3`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "lastFeeGrowthOf", abi = "lastFeeGrowthOf(address,uint256)")]
    pub struct LastFeeGrowthOfCall {
        pub account: ::ethers::core::types::Address,
        pub pool_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `nonce` function with signature `nonce()` and selector `0xaffed0e0`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "nonce", abi = "nonce()")]
    pub struct NonceCall;
    ///Container type for all input parameters for the `pools` function with signature `pools(uint256)` and selector `0xac4afa38`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "pools", abi = "pools(uint256)")]
    pub struct PoolsCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `swap` function with signature `swap(uint256,bytes)` and selector `0xbd0625ab`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "swap", abi = "swap(uint256,bytes)")]
    pub struct SwapCall {
        pub pool_id: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `update` function with signature `update(uint256,bytes)` and selector `0x0216b838`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "update", abi = "update(uint256,bytes)")]
    pub struct UpdateCall {
        pub pool_id: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all of the contract's call
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub enum DFMMCalls {
        Allocate(AllocateCall),
        BalanceOf(BalanceOfCall),
        Deallocate(DeallocateCall),
        GetPool(GetPoolCall),
        GetReservesAndLiquidity(GetReservesAndLiquidityCall),
        Init(InitCall),
        LastFeeGrowthOf(LastFeeGrowthOfCall),
        Nonce(NonceCall),
        Pools(PoolsCall),
        Swap(SwapCall),
        Update(UpdateCall),
    }
    impl ::ethers::core::abi::AbiDecode for DFMMCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AllocateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Allocate(decoded));
            }
            if let Ok(decoded) = <BalanceOfCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BalanceOf(decoded));
            }
            if let Ok(decoded) = <DeallocateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Deallocate(decoded));
            }
            if let Ok(decoded) = <GetPoolCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetPool(decoded));
            }
            if let Ok(decoded) = <GetReservesAndLiquidityCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetReservesAndLiquidity(decoded));
            }
            if let Ok(decoded) = <InitCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Init(decoded));
            }
            if let Ok(decoded) = <LastFeeGrowthOfCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LastFeeGrowthOf(decoded));
            }
            if let Ok(decoded) = <NonceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Nonce(decoded));
            }
            if let Ok(decoded) = <PoolsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Pools(decoded));
            }
            if let Ok(decoded) = <SwapCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Swap(decoded));
            }
            if let Ok(decoded) = <UpdateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Update(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for DFMMCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Allocate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BalanceOf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Deallocate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetPool(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetReservesAndLiquidity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Init(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LastFeeGrowthOf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Nonce(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Pools(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Swap(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Update(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for DFMMCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Allocate(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::Deallocate(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPool(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetReservesAndLiquidity(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Init(element) => ::core::fmt::Display::fmt(element, f),
                Self::LastFeeGrowthOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::Nonce(element) => ::core::fmt::Display::fmt(element, f),
                Self::Pools(element) => ::core::fmt::Display::fmt(element, f),
                Self::Swap(element) => ::core::fmt::Display::fmt(element, f),
                Self::Update(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AllocateCall> for DFMMCalls {
        fn from(value: AllocateCall) -> Self {
            Self::Allocate(value)
        }
    }
    impl ::core::convert::From<BalanceOfCall> for DFMMCalls {
        fn from(value: BalanceOfCall) -> Self {
            Self::BalanceOf(value)
        }
    }
    impl ::core::convert::From<DeallocateCall> for DFMMCalls {
        fn from(value: DeallocateCall) -> Self {
            Self::Deallocate(value)
        }
    }
    impl ::core::convert::From<GetPoolCall> for DFMMCalls {
        fn from(value: GetPoolCall) -> Self {
            Self::GetPool(value)
        }
    }
    impl ::core::convert::From<GetReservesAndLiquidityCall> for DFMMCalls {
        fn from(value: GetReservesAndLiquidityCall) -> Self {
            Self::GetReservesAndLiquidity(value)
        }
    }
    impl ::core::convert::From<InitCall> for DFMMCalls {
        fn from(value: InitCall) -> Self {
            Self::Init(value)
        }
    }
    impl ::core::convert::From<LastFeeGrowthOfCall> for DFMMCalls {
        fn from(value: LastFeeGrowthOfCall) -> Self {
            Self::LastFeeGrowthOf(value)
        }
    }
    impl ::core::convert::From<NonceCall> for DFMMCalls {
        fn from(value: NonceCall) -> Self {
            Self::Nonce(value)
        }
    }
    impl ::core::convert::From<PoolsCall> for DFMMCalls {
        fn from(value: PoolsCall) -> Self {
            Self::Pools(value)
        }
    }
    impl ::core::convert::From<SwapCall> for DFMMCalls {
        fn from(value: SwapCall) -> Self {
            Self::Swap(value)
        }
    }
    impl ::core::convert::From<UpdateCall> for DFMMCalls {
        fn from(value: UpdateCall) -> Self {
            Self::Update(value)
        }
    }
    ///Container type for all return fields from the `allocate` function with signature `allocate(uint256,bytes)` and selector `0x2ec38188`
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
    pub struct AllocateReturn(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `balanceOf` function with signature `balanceOf(address,uint256)` and selector `0x00fdd58e`
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
    pub struct BalanceOfReturn {
        pub balance: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `deallocate` function with signature `deallocate(uint256,bytes)` and selector `0x9d942f9a`
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
    pub struct DeallocateReturn(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `getPool` function with signature `getPool(uint256)` and selector `0x068bcd8d`
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
    pub struct GetPoolReturn(pub Pool);
    ///Container type for all return fields from the `getReservesAndLiquidity` function with signature `getReservesAndLiquidity(uint256)` and selector `0xce153bf4`
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
    pub struct GetReservesAndLiquidityReturn(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `init` function with signature `init((address,address,address,bytes))` and selector `0x1455f1fc`
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
    pub struct InitReturn(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `lastFeeGrowthOf` function with signature `lastFeeGrowthOf(address,uint256)` and selector `0x5f00e1f3`
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
    pub struct LastFeeGrowthOfReturn {
        pub last_fee_growth: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `nonce` function with signature `nonce()` and selector `0xaffed0e0`
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
    pub struct NonceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `pools` function with signature `pools(uint256)` and selector `0xac4afa38`
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
    pub struct PoolsReturn {
        pub inited: bool,
        pub controller: ::ethers::core::types::Address,
        pub strategy: ::ethers::core::types::Address,
        pub token_x: ::ethers::core::types::Address,
        pub token_y: ::ethers::core::types::Address,
        pub reserve_x: ::ethers::core::types::U256,
        pub reserve_y: ::ethers::core::types::U256,
        pub total_liquidity: ::ethers::core::types::U256,
        pub fee_growth: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `swap` function with signature `swap(uint256,bytes)` and selector `0xbd0625ab`
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
    pub struct SwapReturn(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    ///`Pool(bool,address,address,address,address,uint256,uint256,uint256,uint256)`
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
    pub struct Pool {
        pub inited: bool,
        pub controller: ::ethers::core::types::Address,
        pub strategy: ::ethers::core::types::Address,
        pub token_x: ::ethers::core::types::Address,
        pub token_y: ::ethers::core::types::Address,
        pub reserve_x: ::ethers::core::types::U256,
        pub reserve_y: ::ethers::core::types::U256,
        pub total_liquidity: ::ethers::core::types::U256,
        pub fee_growth: ::ethers::core::types::U256,
    }
}
