pub use multi_dfmm::*;
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
pub mod multi_dfmm {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![],
            }),
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
                    ::std::borrow::ToOwned::to_owned("feeGrowthLast"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("feeGrowthLast"),
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
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct Pool"),
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
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IParams.InitParams",
                                        ),
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
                    ::std::borrow::ToOwned::to_owned("locked"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("locked"),
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
                                    name: ::std::borrow::ToOwned::to_owned("reserveXWad"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("reserveYWad"),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "swapFeePercentageWad",
                                    ),
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
                                    name: ::std::borrow::ToOwned::to_owned("x"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("y"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("l"),
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
                                    name: ::std::borrow::ToOwned::to_owned("x"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("y"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("l"),
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
                                    name: ::std::borrow::ToOwned::to_owned("swapper"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("strategy"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("XXXXXXX"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("YYYYYY"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("LLLLLL"),
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
                                    name: ::std::borrow::ToOwned::to_owned("swapper"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("source"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("tokenIn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("tokenOut"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amountIn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amountOut"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("liquidityDelta"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
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
                    ::std::borrow::ToOwned::to_owned("InvalidSwap"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidSwap"),
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
    pub static MULTIDFMM_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R`\x01`\0U4\x80\x15a\0bW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[Pa%\xAB\x80a\0r`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\0\xF5W`\x005`\xE0\x1C\x80c\xACJ\xFA8\x11a\0\xBEW\x80c\xACJ\xFA8\x14a\x02$W\x80c\xAF\xFE\xD0\xE0\x14a\x02\x95W\x80c\xBD\x06%\xAB\x14a\x02\x9DW\x80c\xCE\x15;\xF4\x14a\x02\xB2W\x80c\xCF0\x90\x12\x14a\x02\xC5W\x80c\xE6v\xC3\xAC\x14a\x02\xCEWa\0\xF5V[\x80b\xFD\xD5\x8E\x14a\x01ZW\x80c\x06\x8B\xCD\x8D\x14a\x01\x98W\x80c\x14\xAE4/\x14a\x01\xB8W\x80c.\xC3\x81\x88\x14a\x01\xE3W\x80c\x9D\x94/\x9A\x14a\x02\x11W[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x01\x85a\x01h6`\x04a\x1F\x88V[`\x02` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\xABa\x01\xA66`\x04a\x1F\xB5V[a\x02\xE1V[`@Qa\x01\x8F\x91\x90a\x1F\xD1V[a\x01\x85a\x01\xC66`\x04a\x1F\x88V[`\x03` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[a\x01\xF6a\x01\xF16`\x04a \xDDV[a\x03\xE5V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x01\x8FV[a\x01\xF6a\x02\x1F6`\x04a \xDDV[a\x06\xD3V[a\x027a\x0226`\x04a\x1F\xB5V[a\t\x9CV[`@\x80Q\x9A\x15\x15\x8BR`\x01`\x01`\xA0\x1B\x03\x99\x8A\x16` \x8C\x01R\x97\x89\x16\x97\x8A\x01\x97\x90\x97R\x94\x87\x16``\x89\x01R\x95\x90\x92\x16`\x80\x87\x01R`\xA0\x86\x01R`\xC0\x85\x01R`\xE0\x84\x01\x92\x90\x92Ra\x01\0\x83\x01\x91\x90\x91Ra\x01 \x82\x01Ra\x01@\x01a\x01\x8FV[`\x01Ta\x01\x85V[a\x02\xB0a\x02\xAB6`\x04a \xDDV[a\n\x14V[\0[a\x01\xF6a\x02\xC06`\x04a\x1F\xB5V[a\x0C\xBCV[a\x01\x85`\0T\x81V[a\x01\xF6a\x02\xDC6`\x04a\"\nV[a\r=V[`@\x80Qa\x01@\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R`\xC0\x81\x01\x82\x90R`\xE0\x81\x01\x82\x90Ra\x01\0\x81\x01\x82\x90Ra\x01 \x81\x01\x91\x90\x91R`\x01\x82\x81T\x81\x10a\x03EWa\x03Ea\"\xA2V[`\0\x91\x82R` \x91\x82\x90 `@\x80Qa\x01@\x81\x01\x82R`\t\x90\x93\x02\x90\x91\x01\x80T`\xFF\x81\x16\x15\x15\x84R`\x01`\x01`\xA0\x1B\x03a\x01\0\x91\x82\x90\x04\x81\x16\x95\x85\x01\x95\x90\x95R`\x01\x82\x01T\x85\x16\x92\x84\x01\x92\x90\x92R`\x02\x81\x01T\x84\x16``\x84\x01R`\x03\x81\x01T\x90\x93\x16`\x80\x83\x01R`\x04\x83\x01T`\xA0\x83\x01R`\x05\x83\x01T`\xC0\x83\x01R`\x06\x83\x01T`\xE0\x83\x01R`\x07\x83\x01T\x90\x82\x01R`\x08\x90\x91\x01Ta\x01 \x82\x01R\x92\x91PPV[`\0\x80`\0\x80T`\x01\x14a\x04\x14W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04\x0B\x90a\"\xB8V[`@Q\x80\x91\x03\x90\xFD[`\0\x80Ua\x04!\x86a\x12\xEBV[`\0\x80`\0a\x043\x89`\x01\x8A\x8Aa\x13\xD5V[3`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 \x8F\x84R\x90\x91R\x81 \x80T\x94\x97P\x92\x95P\x90\x93P\x83\x92a\x04e\x90\x84\x90a\"\xEEV[\x90\x91UPP`\x01\x80T\x8A\x90\x81\x10a\x04~Wa\x04~a\"\xA2V[`\0\x91\x82R` \x80\x83 `\x07`\t\x90\x93\x02\x01\x91\x90\x91\x01T3\x83R`\x03\x82R`@\x80\x84 \x8D\x85R\x90\x92R\x91 U`\x01\x80T\x8A\x90\x81\x10a\x04\xBEWa\x04\xBEa\"\xA2V[`\0\x91\x82R` \x90\x91 `\x02`\t\x90\x92\x02\x01\x01T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\x05\x03\x903\x900\x90\x88\x90`\x04\x01a#\x01V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x05[W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a%\x8B\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x05oW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\x93\x91\x90a#5V[P`\x01\x89\x81T\x81\x10a\x05\xA7Wa\x05\xA7a\"\xA2V[`\0\x91\x82R` \x90\x91 `\x03`\t\x90\x92\x02\x01\x01T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\x05\xEC\x903\x900\x90\x87\x90`\x04\x01a#\x01V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x06DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a%\x8B\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x06XW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06|\x91\x90a#5V[P`@\x80Q\x84\x81R` \x81\x01\x84\x90R\x90\x81\x01\x82\x90R\x7F\x92\x8F-\xEA\x06G\xB6\xB9A\xA5K\xBC\x03\xCE\x9B\xCB\x0E\xC1\xE0\x8E\xAAl\x12\x98.{\xF4H\xD9?\xCCt\x90``\x01[`@Q\x80\x91\x03\x90\xA1`\x01`\0U\x91\x98\x90\x97P\x90\x95P\x93PPPPV[`\0\x80`\0\x80T`\x01\x14a\x06\xF9W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04\x0B\x90a\"\xB8V[`\0\x80Ua\x07\x06\x86a\x12\xEBV[`\0\x80`\0a\x07\x18\x89`\0\x8A\x8Aa\x13\xD5V[3`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 \x8F\x84R\x90\x91R\x81 \x80T\x94\x97P\x92\x95P\x90\x93P\x83\x92a\x07J\x90\x84\x90a#SV[\x90\x91UPP`\x01\x80T\x8A\x90\x81\x10a\x07cWa\x07ca\"\xA2V[`\0\x91\x82R` \x80\x83 `\x07`\t\x90\x93\x02\x01\x91\x90\x91\x01T3\x83R`\x03\x82R`@\x80\x84 \x8D\x85R\x90\x92R\x91 U`\x01\x80T\x8A\x90\x81\x10a\x07\xA3Wa\x07\xA3a\"\xA2V[`\0\x91\x82R` \x90\x91 `\t\x90\x91\x02\x01`\x02\x01T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x85\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x08>W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a%\x8B\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x08RW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08v\x91\x90a#5V[P`\x01\x89\x81T\x81\x10a\x08\x8AWa\x08\x8Aa\"\xA2V[`\0\x91\x82R` \x90\x91 `\t\x90\x91\x02\x01`\x03\x01T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\t%W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a%\x8B\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\t9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t]\x91\x90a#5V[P`@\x80Q\x84\x81R` \x81\x01\x84\x90R\x90\x81\x01\x82\x90R\x7FDO\xFE\xDB%A\xFF,*\xC9\xA6\x16\xD97|\xE4\x0Ev*\x8F\x86\xC2\xF9-AJ\xB8\x8E\x14\xE7Z\x85\x90``\x01a\x06\xB7V[`\x01\x81\x81T\x81\x10a\t\xACW`\0\x80\xFD[`\0\x91\x82R` \x90\x91 `\t\x90\x91\x02\x01\x80T`\x01\x82\x01T`\x02\x83\x01T`\x03\x84\x01T`\x04\x85\x01T`\x05\x86\x01T`\x06\x87\x01T`\x07\x88\x01T`\x08\x90\x98\x01T`\xFF\x88\x16\x99Pa\x01\0\x90\x97\x04`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x98\x96\x81\x16\x97\x95\x81\x16\x96\x94\x16\x94\x92\x93\x91\x92\x90\x91\x90\x8AV[`\0T`\x01\x14a\n6W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04\x0B\x90a\"\xB8V[`\0\x80\x81\x90UP\x82`\x01\x81\x81T\x81\x10a\nQWa\nQa\"\xA2V[`\0\x91\x82R` \x90\x91 `\t\x90\x91\x02\x01T`\xFF\x16a\n\x82W`@Qc!\xC4\xE3W`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80`\0\x80`\0`\x01\x89\x81T\x81\x10a\n\x9DWa\n\x9Da\"\xA2V[`\0\x91\x82R` \x90\x91 `\x01`\t\x90\x92\x02\x01\x01T`@Qc2\x14\x89\x0F`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c2\x14\x89\x0F\x90a\n\xE2\x90\x8C\x90\x8C\x90\x8C\x90`\x04\x01a#fV[`\xC0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0B8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a%\x8B\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0BLW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Bp\x91\x90a#\x9CV[\x95P\x95P\x95PP\x94P\x94P\x84a\x0B\xAFW`\0\x84\x12a\x0B\x8D\x85a\x16\xB0V[`@Qcw`m)`\xE1\x1B\x81R\x91\x15\x15`\x04\x83\x01R`$\x82\x01R`D\x01a\x04\x0BV[`\0`\x01\x8A\x81T\x81\x10a\x0B\xC4Wa\x0B\xC4a\"\xA2V[\x90`\0R` `\0 \x90`\t\x02\x01`\x06\x01T\x90P\x81`\x01\x8B\x81T\x81\x10a\x0B\xECWa\x0B\xECa\"\xA2V[\x90`\0R` `\0 \x90`\t\x02\x01`\x06\x01\x81\x90UP`\0a\x0C:\x82`\x01\x8D\x81T\x81\x10a\x0C\x1AWa\x0C\x1Aa\"\xA2V[\x90`\0R` `\0 \x90`\t\x02\x01`\x06\x01Ta\x16\xEF\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa\x0Cs\x81`\x01\x8D\x81T\x81\x10a\x0CSWa\x0CSa\"\xA2V[\x90`\0R` `\0 \x90`\t\x02\x01`\x07\x01Ta\x17\r\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[`\x01\x8C\x81T\x81\x10a\x0C\x86Wa\x0C\x86a\"\xA2V[\x90`\0R` `\0 \x90`\t\x02\x01`\x07\x01\x81\x90UPa\x0C\xA6\x8B\x86\x86a\x17\"V[PP`\x01`\0UPPPPPPPPPPPPPV[`\0\x80`\0`\x01\x84\x81T\x81\x10a\x0C\xD4Wa\x0C\xD4a\"\xA2V[\x90`\0R` `\0 \x90`\t\x02\x01`\x04\x01T`\x01\x85\x81T\x81\x10a\x0C\xF9Wa\x0C\xF9a\"\xA2V[\x90`\0R` `\0 \x90`\t\x02\x01`\x05\x01T`\x01\x86\x81T\x81\x10a\r\x1EWa\r\x1Ea\"\xA2V[\x90`\0R` `\0 \x90`\t\x02\x01`\x06\x01T\x92P\x92P\x92P\x91\x93\x90\x92PV[`\0\x80`\0\x80T`\x01\x14a\rcW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04\x0B\x90a\"\xB8V[`\0\x80\x80U\x80\x80\x80\x80a\r|`@\x8A\x01` \x8B\x01a#\xF0V[`\x01`\x01`\xA0\x1B\x03\x16c\x9F\x83\x13{a\r\x93`\x01T\x90V[a\r\xA0`\xA0\x8D\x01\x8Da$SV[`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\r\xBE\x93\x92\x91\x90a#fV[`\xA0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0E\x16W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a%\x8B\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x0E*W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0EN\x91\x90a%$V[\x94P\x94P\x94P\x94P\x94P\x84a\x0EjW`\0\x84\x12a\x0B\x8D\x85a\x16\xB0V[`\0`@Q\x80a\x01@\x01`@R\x80`\x01\x15\x15\x81R` \x013`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8B` \x01` \x81\x01\x90a\x0E\xA3\x91\x90a#\xF0V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01a\x0E\xC1``\x8D\x01`@\x8E\x01a#\xF0V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01a\x0E\xDF`\x80\x8D\x01``\x8E\x01a#\xF0V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x85\x81R` \x01\x84\x81R` \x01\x83\x81R` \x01g\r\xE0\xB6\xB3\xA7d\0\0\x81R` \x01\x8B`\x80\x015\x81RP\x90P`\x01\x81\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90`\0R` `\0 \x90`\t\x02\x01`\0\x90\x91\x90\x91\x90\x91P`\0\x82\x01Q\x81`\0\x01`\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP` \x82\x01Q\x81`\0\x01`\x01a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UP`@\x82\x01Q\x81`\x01\x01`\0a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UP``\x82\x01Q\x81`\x02\x01`\0a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UP`\x80\x82\x01Q\x81`\x03\x01`\0a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UP`\xA0\x82\x01Q\x81`\x04\x01U`\xC0\x82\x01Q\x81`\x05\x01U`\xE0\x82\x01Q\x81`\x06\x01Ua\x01\0\x82\x01Q\x81`\x07\x01Ua\x01 \x82\x01Q\x81`\x08\x01UPP\x81`\x02`\x003`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 `\0`\x01\x80\x80T\x90Pa\x10\x86\x91\x90a#SV[\x81R` \x80\x82\x01\x92\x90\x92R`@\x90\x81\x01`\0\x90\x81 \x93\x90\x93U3\x83R`\x03\x90\x91R\x81 `\x01\x80Tg\r\xE0\xB6\xB3\xA7d\0\0\x93\x91a\x10\xC1\x91a#SV[\x81R` \x01\x90\x81R` \x01`\0 \x81\x90UP\x89`@\x01` \x81\x01\x90a\x10\xE6\x91\x90a#\xF0V[`\x01`\x01`\xA0\x1B\x03\x16c#\xB8r\xDD30\x87`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x11\x15\x93\x92\x91\x90a#\x01V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x11mW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a%\x8B\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x11\x81W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\xA5\x91\x90a#5V[Pa\x11\xB6`\x80\x8B\x01``\x8C\x01a#\xF0V[`\x01`\x01`\xA0\x1B\x03\x16c#\xB8r\xDD30\x86`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x11\xE5\x93\x92\x91\x90a#\x01V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x12=W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a%\x8B\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x12QW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12u\x91\x90a#5V[Pa\x12\x86`@\x8B\x01` \x8C\x01a#\xF0V[`@\x80Q\x86\x81R` \x81\x01\x86\x90R\x90\x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x903\x90\x7F?\x1Fc\xA0\x1B[6Ff\x17!O4 E\xD7SS\x025<\xE7\t\xC2\x9E\xD3\xECL\x02H\xBD5\x90``\x01`@Q\x80\x91\x03\x90\xA3P`\x01`\0U\x91\x98\x90\x97P\x90\x95P\x93PPPPV[3`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 \x84\x84R\x90\x91R\x90 T\x15\x80\x15\x90a\x13SWP3`\0\x90\x81R`\x03` \x90\x81R`@\x80\x83 \x84\x84R\x90\x91R\x90 T`\x01\x80T\x83\x90\x81\x10a\x13>Wa\x13>a\"\xA2V[\x90`\0R` `\0 \x90`\t\x02\x01`\x07\x01T\x14\x15[\x15a\x13\xD2W3`\0\x90\x81R`\x03` \x90\x81R`@\x80\x83 \x84\x84R\x90\x91R\x81 T`\x01\x80Ta\x13\x8D\x92\x91\x90\x85\x90\x81\x10a\x0CSWa\x0CSa\"\xA2V[3`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 \x86\x84R\x90\x91R\x90 T\x90\x91Pa\x13\xB5\x90\x82a\x17\rV[3`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 \x86\x84R\x90\x91R\x90 UP[PV[`\0\x80`\0\x80`\0\x80`\0\x80`\x01\x8C\x81T\x81\x10a\x13\xF4Wa\x13\xF4a\"\xA2V[`\0\x91\x82R` \x90\x91 `\x01`\t\x90\x92\x02\x01\x01T`@Qb#\x8Bu`\xEA\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x8E-\xD4\0\x90a\x148\x90\x8F\x90\x8E\x90\x8E\x90`\x04\x01a#fV[`\xA0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x14\x8EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a%\x8B\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x14\xA2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14\xC6\x91\x90a%$V[\x94P\x94P\x94P\x94P\x94P\x84a\x14\xE2W`\0\x84\x12a\x0B\x8D\x85a\x16\xB0V[\x8Aa\x15\x1CW\x82`\x01\x8D\x81T\x81\x10a\x14\xFBWa\x14\xFBa\"\xA2V[\x90`\0R` `\0 \x90`\t\x02\x01`\x04\x01Ta\x15\x17\x91\x90a#SV[a\x15LV[`\x01\x8C\x81T\x81\x10a\x15/Wa\x15/a\"\xA2V[\x90`\0R` `\0 \x90`\t\x02\x01`\x04\x01T\x83a\x15L\x91\x90a#SV[\x97P\x8Aa\x15\x88W\x81`\x01\x8D\x81T\x81\x10a\x15gWa\x15ga\"\xA2V[\x90`\0R` `\0 \x90`\t\x02\x01`\x05\x01Ta\x15\x83\x91\x90a#SV[a\x15\xB8V[`\x01\x8C\x81T\x81\x10a\x15\x9BWa\x15\x9Ba\"\xA2V[\x90`\0R` `\0 \x90`\t\x02\x01`\x05\x01T\x82a\x15\xB8\x91\x90a#SV[\x96P\x8Aa\x15\xF4W\x80`\x01\x8D\x81T\x81\x10a\x15\xD3Wa\x15\xD3a\"\xA2V[\x90`\0R` `\0 \x90`\t\x02\x01`\x06\x01Ta\x15\xEF\x91\x90a#SV[a\x16$V[`\x01\x8C\x81T\x81\x10a\x16\x07Wa\x16\x07a\"\xA2V[\x90`\0R` `\0 \x90`\t\x02\x01`\x06\x01T\x81a\x16$\x91\x90a#SV[\x95P\x82`\x01\x8D\x81T\x81\x10a\x16:Wa\x16:a\"\xA2V[\x90`\0R` `\0 \x90`\t\x02\x01`\x04\x01\x81\x90UP\x81`\x01\x8D\x81T\x81\x10a\x16cWa\x16ca\"\xA2V[\x90`\0R` `\0 \x90`\t\x02\x01`\x05\x01\x81\x90UP\x80`\x01\x8D\x81T\x81\x10a\x16\x8CWa\x16\x8Ca\"\xA2V[\x90`\0R` `\0 \x90`\t\x02\x01`\x06\x01\x81\x90UPPPPPP\x94P\x94P\x94\x91PPV[`\0`\x01`\xFF\x1B\x82\x03a\x16\xD6W`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x12\x15a\x16\xE7WP\x19`\x01\x01\x90V[P\x80[\x91\x90PV[`\0a\x17\x04\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x1E\xB2V[\x90P[\x92\x91PPV[`\0a\x17\x04\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x1E\xB2V[`\0\x80`\0\x80`\0\x80`\x01\x89\x81T\x81\x10a\x17>Wa\x17>a\"\xA2V[\x90`\0R` `\0 \x90`\t\x02\x01`\x04\x01T`\x01\x8A\x81T\x81\x10a\x17cWa\x17ca\"\xA2V[\x90`\0R` `\0 \x90`\t\x02\x01`\x05\x01T\x91P\x91P\x81\x88\x11\x15a\x18YW`\x01\x89\x81T\x81\x10a\x17\x94Wa\x17\x94a\"\xA2V[\x90`\0R` `\0 \x90`\t\x02\x01`\x02\x01`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x95P`\x01\x89\x81T\x81\x10a\x17\xCEWa\x17\xCEa\"\xA2V[`\0\x91\x82R` \x90\x91 `\x03`\t\x90\x92\x02\x01\x01T`\x01`\x01`\xA0\x1B\x03\x16\x94Pa\x17\xF7\x82\x89a#SV[\x93P\x86\x81\x11a\x18HW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7Finvalid swap: inputs x and y\0\0\0\0`D\x82\x01R`d\x01a\x04\x0BV[a\x18R\x87\x82a#SV[\x92Pa\x19-V[`\x01\x89\x81T\x81\x10a\x18lWa\x18la\"\xA2V[\x90`\0R` `\0 \x90`\t\x02\x01`\x03\x01`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x95P`\x01\x89\x81T\x81\x10a\x18\xA6Wa\x18\xA6a\"\xA2V[`\0\x91\x82R` \x90\x91 `\x02`\t\x90\x92\x02\x01\x01T`\x01`\x01`\xA0\x1B\x03\x16\x94Pa\x18\xCF\x81\x88a#SV[\x93P\x87\x82\x11a\x19 W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7Finvalid swap: inputs x and y\0\0\0\0`D\x82\x01R`d\x01a\x04\x0BV[a\x19*\x88\x83a#SV[\x92P[\x87`\x01\x8A\x81T\x81\x10a\x19AWa\x19Aa\"\xA2V[\x90`\0R` `\0 \x90`\t\x02\x01`\x04\x01\x81\x90UP\x86`\x01\x8A\x81T\x81\x10a\x19jWa\x19ja\"\xA2V[`\0\x91\x82R` \x82 `\t\x91\x90\x91\x02\x01`\x05\x01\x91\x90\x91U`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x88\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x19\xFEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a%\x8B\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x1A\x12W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A6\x91\x90a%nV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x88\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x1A\xB9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a%\x8B\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x1A\xCDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A\xF1\x91\x90a%nV[`@Qc#\xB8r\xDD`\xE0\x1B\x81R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x89\x16\x90c#\xB8r\xDD\x90a\x1B$\x903\x900\x90\x8B\x90`\x04\x01a#\x01V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1B|W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a%\x8B\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x1B\x90W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B\xB4\x91\x90a#5V[P`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x86\x90R`\x01`\x01`\xA0\x1B\x03\x88\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1C;W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a%\x8B\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x1COW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1Cs\x91\x90a#5V[Pa\x1C~\x86\x83a\"\xEEV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x8A\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x1C\xFBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a%\x8B\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x1D\x0FW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1D3\x91\x90a%nV[\x10\x15a\x1D\x8CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: input token transf`D\x82\x01Ra2\xB9`\xF1\x1B`d\x82\x01R`\x84\x01a\x04\x0BV[a\x1D\x96\x85\x82a#SV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x89\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x1E\x13W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a%\x8B\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x1E'W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1EK\x91\x90a%nV[\x10\x15a\x1E\xA5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7Finvalid swap: output token trans`D\x82\x01Rb32\xB9`\xE9\x1B`d\x82\x01R`\x84\x01a\x04\x0BV[PPPP\x93P\x93P\x93P\x93V[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x1E\xCAW`\0\x80\xFD[\x04\x92\x91PPV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x16\xEAW`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x1F\x9EWa\x1F\x9Ea\x1E\xD1V[a\x1F\xA7\x83a\x1FqV[\x94` \x93\x90\x93\x015\x93PPPV[`\0` \x82\x84\x03\x12\x15a\x1F\xCAWa\x1F\xCAa\x1E\xD1V[P5\x91\x90PV[\x81Q\x15\x15\x81Ra\x01@\x81\x01` \x83\x01Qa\x1F\xF6` \x84\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P`@\x83\x01Qa \x11`@\x84\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P``\x83\x01Qa ,``\x84\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P`\x80\x83\x01Qa G`\x80\x84\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P`\xA0\x83\x01Q`\xA0\x83\x01R`\xC0\x83\x01Q`\xC0\x83\x01R`\xE0\x83\x01Q`\xE0\x83\x01Ra\x01\0\x80\x84\x01Q\x81\x84\x01RPa\x01 \x80\x84\x01Q\x81\x84\x01RP\x92\x91PPV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rjrray stride`\xA8\x1B`d\x82\x01R`\x84\x81\xFD[`\0\x80`\0`@\x84\x86\x03\x12\x15a \xF5Wa \xF5a\x1E\xD1V[\x835\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a!\x17Wa!\x17a\x1F!V[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a!\x7FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x81\xFD[\x815\x81\x81\x11\x15a!\xE2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x0ENL/$\r\x8C\xAD\xCC\xEE\x8D`\xAB\x1B`d\x82\x01R`\x84\x81\xFD[\x87` \x82\x85\x01\x01\x11\x15a!\xF7Wa!\xF7a \x84V[` \x83\x01\x94P\x80\x93PPPP\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a\"\x1FWa\"\x1Fa\x1E\xD1V[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\"9Wa\"9a\x1F!V[\x82\x01`\xC0\x81\x85\x03\x12\x15a\"\x9BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FABI decoding: struct calldata to`D\x82\x01Rf\x1B\xC8\x1C\xDA\x1B\xDC\x9D`\xCA\x1B`d\x82\x01R`\x84\x81\xFD[\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[` \x80\x82R`\x06\x90\x82\x01Re\x1B\x1B\xD8\xDA\xD9Y`\xD2\x1B`@\x82\x01R``\x01\x90V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x17\x07Wa\x17\x07a\"\xD8V[`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x81R\x91\x90\x92\x16` \x82\x01R`@\x81\x01\x91\x90\x91R``\x01\x90V[\x80Q\x80\x15\x15\x81\x14a\x16\xEAW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a#JWa#Ja\x1E\xD1V[a\x17\x04\x82a#%V[\x81\x81\x03\x81\x81\x11\x15a\x17\x07Wa\x17\x07a\"\xD8V[\x83\x81R`@` \x82\x01R\x81`@\x82\x01R\x81\x83``\x83\x017`\0\x81\x83\x01``\x90\x81\x01\x91\x90\x91R`\x1F\x90\x92\x01`\x1F\x19\x16\x01\x01\x92\x91PPV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a#\xB8Wa#\xB8a\x1E\xD1V[a#\xC1\x87a#%V[\x95P` \x87\x01Q\x94P`@\x87\x01Q\x93P``\x87\x01Q\x92P`\x80\x87\x01Q\x91P`\xA0\x87\x01Q\x90P\x92\x95P\x92\x95P\x92\x95V[`\0` \x82\x84\x03\x12\x15a$\x05Wa$\x05a\x1E\xD1V[a\x17\x04\x82a\x1FqV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FCalldata tail too short\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x81\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a$\xAAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FInvalid calldata tail offset\0\0\0\0`D\x82\x01R`d\x81\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a%\x05W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FInvalid calldata tail length\0\0\0\0`D\x82\x01R`d\x81\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a%\x1DWa%\x1Da$\x0EV[\x92P\x92\x90PV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a%?Wa%?a\x1E\xD1V[a%H\x86a#%V[` \x87\x01Q`@\x88\x01Q``\x89\x01Q`\x80\x90\x99\x01Q\x92\x9A\x91\x99P\x97\x96P\x90\x94P\x92PPPV[`\0` \x82\x84\x03\x12\x15a%\x83Wa%\x83a\x1E\xD1V[PQ\x91\x90PV\xFETarget contract does not contain";
    /// The bytecode of the contract.
    pub static MULTIDFMM_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\0\xF5W`\x005`\xE0\x1C\x80c\xACJ\xFA8\x11a\0\xBEW\x80c\xACJ\xFA8\x14a\x02$W\x80c\xAF\xFE\xD0\xE0\x14a\x02\x95W\x80c\xBD\x06%\xAB\x14a\x02\x9DW\x80c\xCE\x15;\xF4\x14a\x02\xB2W\x80c\xCF0\x90\x12\x14a\x02\xC5W\x80c\xE6v\xC3\xAC\x14a\x02\xCEWa\0\xF5V[\x80b\xFD\xD5\x8E\x14a\x01ZW\x80c\x06\x8B\xCD\x8D\x14a\x01\x98W\x80c\x14\xAE4/\x14a\x01\xB8W\x80c.\xC3\x81\x88\x14a\x01\xE3W\x80c\x9D\x94/\x9A\x14a\x02\x11W[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x01\x85a\x01h6`\x04a\x1F\x88V[`\x02` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\xABa\x01\xA66`\x04a\x1F\xB5V[a\x02\xE1V[`@Qa\x01\x8F\x91\x90a\x1F\xD1V[a\x01\x85a\x01\xC66`\x04a\x1F\x88V[`\x03` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[a\x01\xF6a\x01\xF16`\x04a \xDDV[a\x03\xE5V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x01\x8FV[a\x01\xF6a\x02\x1F6`\x04a \xDDV[a\x06\xD3V[a\x027a\x0226`\x04a\x1F\xB5V[a\t\x9CV[`@\x80Q\x9A\x15\x15\x8BR`\x01`\x01`\xA0\x1B\x03\x99\x8A\x16` \x8C\x01R\x97\x89\x16\x97\x8A\x01\x97\x90\x97R\x94\x87\x16``\x89\x01R\x95\x90\x92\x16`\x80\x87\x01R`\xA0\x86\x01R`\xC0\x85\x01R`\xE0\x84\x01\x92\x90\x92Ra\x01\0\x83\x01\x91\x90\x91Ra\x01 \x82\x01Ra\x01@\x01a\x01\x8FV[`\x01Ta\x01\x85V[a\x02\xB0a\x02\xAB6`\x04a \xDDV[a\n\x14V[\0[a\x01\xF6a\x02\xC06`\x04a\x1F\xB5V[a\x0C\xBCV[a\x01\x85`\0T\x81V[a\x01\xF6a\x02\xDC6`\x04a\"\nV[a\r=V[`@\x80Qa\x01@\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R`\xC0\x81\x01\x82\x90R`\xE0\x81\x01\x82\x90Ra\x01\0\x81\x01\x82\x90Ra\x01 \x81\x01\x91\x90\x91R`\x01\x82\x81T\x81\x10a\x03EWa\x03Ea\"\xA2V[`\0\x91\x82R` \x91\x82\x90 `@\x80Qa\x01@\x81\x01\x82R`\t\x90\x93\x02\x90\x91\x01\x80T`\xFF\x81\x16\x15\x15\x84R`\x01`\x01`\xA0\x1B\x03a\x01\0\x91\x82\x90\x04\x81\x16\x95\x85\x01\x95\x90\x95R`\x01\x82\x01T\x85\x16\x92\x84\x01\x92\x90\x92R`\x02\x81\x01T\x84\x16``\x84\x01R`\x03\x81\x01T\x90\x93\x16`\x80\x83\x01R`\x04\x83\x01T`\xA0\x83\x01R`\x05\x83\x01T`\xC0\x83\x01R`\x06\x83\x01T`\xE0\x83\x01R`\x07\x83\x01T\x90\x82\x01R`\x08\x90\x91\x01Ta\x01 \x82\x01R\x92\x91PPV[`\0\x80`\0\x80T`\x01\x14a\x04\x14W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04\x0B\x90a\"\xB8V[`@Q\x80\x91\x03\x90\xFD[`\0\x80Ua\x04!\x86a\x12\xEBV[`\0\x80`\0a\x043\x89`\x01\x8A\x8Aa\x13\xD5V[3`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 \x8F\x84R\x90\x91R\x81 \x80T\x94\x97P\x92\x95P\x90\x93P\x83\x92a\x04e\x90\x84\x90a\"\xEEV[\x90\x91UPP`\x01\x80T\x8A\x90\x81\x10a\x04~Wa\x04~a\"\xA2V[`\0\x91\x82R` \x80\x83 `\x07`\t\x90\x93\x02\x01\x91\x90\x91\x01T3\x83R`\x03\x82R`@\x80\x84 \x8D\x85R\x90\x92R\x91 U`\x01\x80T\x8A\x90\x81\x10a\x04\xBEWa\x04\xBEa\"\xA2V[`\0\x91\x82R` \x90\x91 `\x02`\t\x90\x92\x02\x01\x01T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\x05\x03\x903\x900\x90\x88\x90`\x04\x01a#\x01V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x05[W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a%\x8B\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x05oW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\x93\x91\x90a#5V[P`\x01\x89\x81T\x81\x10a\x05\xA7Wa\x05\xA7a\"\xA2V[`\0\x91\x82R` \x90\x91 `\x03`\t\x90\x92\x02\x01\x01T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\x05\xEC\x903\x900\x90\x87\x90`\x04\x01a#\x01V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x06DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a%\x8B\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x06XW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06|\x91\x90a#5V[P`@\x80Q\x84\x81R` \x81\x01\x84\x90R\x90\x81\x01\x82\x90R\x7F\x92\x8F-\xEA\x06G\xB6\xB9A\xA5K\xBC\x03\xCE\x9B\xCB\x0E\xC1\xE0\x8E\xAAl\x12\x98.{\xF4H\xD9?\xCCt\x90``\x01[`@Q\x80\x91\x03\x90\xA1`\x01`\0U\x91\x98\x90\x97P\x90\x95P\x93PPPPV[`\0\x80`\0\x80T`\x01\x14a\x06\xF9W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04\x0B\x90a\"\xB8V[`\0\x80Ua\x07\x06\x86a\x12\xEBV[`\0\x80`\0a\x07\x18\x89`\0\x8A\x8Aa\x13\xD5V[3`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 \x8F\x84R\x90\x91R\x81 \x80T\x94\x97P\x92\x95P\x90\x93P\x83\x92a\x07J\x90\x84\x90a#SV[\x90\x91UPP`\x01\x80T\x8A\x90\x81\x10a\x07cWa\x07ca\"\xA2V[`\0\x91\x82R` \x80\x83 `\x07`\t\x90\x93\x02\x01\x91\x90\x91\x01T3\x83R`\x03\x82R`@\x80\x84 \x8D\x85R\x90\x92R\x91 U`\x01\x80T\x8A\x90\x81\x10a\x07\xA3Wa\x07\xA3a\"\xA2V[`\0\x91\x82R` \x90\x91 `\t\x90\x91\x02\x01`\x02\x01T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x85\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x08>W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a%\x8B\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x08RW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08v\x91\x90a#5V[P`\x01\x89\x81T\x81\x10a\x08\x8AWa\x08\x8Aa\"\xA2V[`\0\x91\x82R` \x90\x91 `\t\x90\x91\x02\x01`\x03\x01T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\t%W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a%\x8B\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\t9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t]\x91\x90a#5V[P`@\x80Q\x84\x81R` \x81\x01\x84\x90R\x90\x81\x01\x82\x90R\x7FDO\xFE\xDB%A\xFF,*\xC9\xA6\x16\xD97|\xE4\x0Ev*\x8F\x86\xC2\xF9-AJ\xB8\x8E\x14\xE7Z\x85\x90``\x01a\x06\xB7V[`\x01\x81\x81T\x81\x10a\t\xACW`\0\x80\xFD[`\0\x91\x82R` \x90\x91 `\t\x90\x91\x02\x01\x80T`\x01\x82\x01T`\x02\x83\x01T`\x03\x84\x01T`\x04\x85\x01T`\x05\x86\x01T`\x06\x87\x01T`\x07\x88\x01T`\x08\x90\x98\x01T`\xFF\x88\x16\x99Pa\x01\0\x90\x97\x04`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x98\x96\x81\x16\x97\x95\x81\x16\x96\x94\x16\x94\x92\x93\x91\x92\x90\x91\x90\x8AV[`\0T`\x01\x14a\n6W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04\x0B\x90a\"\xB8V[`\0\x80\x81\x90UP\x82`\x01\x81\x81T\x81\x10a\nQWa\nQa\"\xA2V[`\0\x91\x82R` \x90\x91 `\t\x90\x91\x02\x01T`\xFF\x16a\n\x82W`@Qc!\xC4\xE3W`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80`\0\x80`\0`\x01\x89\x81T\x81\x10a\n\x9DWa\n\x9Da\"\xA2V[`\0\x91\x82R` \x90\x91 `\x01`\t\x90\x92\x02\x01\x01T`@Qc2\x14\x89\x0F`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c2\x14\x89\x0F\x90a\n\xE2\x90\x8C\x90\x8C\x90\x8C\x90`\x04\x01a#fV[`\xC0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0B8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a%\x8B\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0BLW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Bp\x91\x90a#\x9CV[\x95P\x95P\x95PP\x94P\x94P\x84a\x0B\xAFW`\0\x84\x12a\x0B\x8D\x85a\x16\xB0V[`@Qcw`m)`\xE1\x1B\x81R\x91\x15\x15`\x04\x83\x01R`$\x82\x01R`D\x01a\x04\x0BV[`\0`\x01\x8A\x81T\x81\x10a\x0B\xC4Wa\x0B\xC4a\"\xA2V[\x90`\0R` `\0 \x90`\t\x02\x01`\x06\x01T\x90P\x81`\x01\x8B\x81T\x81\x10a\x0B\xECWa\x0B\xECa\"\xA2V[\x90`\0R` `\0 \x90`\t\x02\x01`\x06\x01\x81\x90UP`\0a\x0C:\x82`\x01\x8D\x81T\x81\x10a\x0C\x1AWa\x0C\x1Aa\"\xA2V[\x90`\0R` `\0 \x90`\t\x02\x01`\x06\x01Ta\x16\xEF\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa\x0Cs\x81`\x01\x8D\x81T\x81\x10a\x0CSWa\x0CSa\"\xA2V[\x90`\0R` `\0 \x90`\t\x02\x01`\x07\x01Ta\x17\r\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[`\x01\x8C\x81T\x81\x10a\x0C\x86Wa\x0C\x86a\"\xA2V[\x90`\0R` `\0 \x90`\t\x02\x01`\x07\x01\x81\x90UPa\x0C\xA6\x8B\x86\x86a\x17\"V[PP`\x01`\0UPPPPPPPPPPPPPV[`\0\x80`\0`\x01\x84\x81T\x81\x10a\x0C\xD4Wa\x0C\xD4a\"\xA2V[\x90`\0R` `\0 \x90`\t\x02\x01`\x04\x01T`\x01\x85\x81T\x81\x10a\x0C\xF9Wa\x0C\xF9a\"\xA2V[\x90`\0R` `\0 \x90`\t\x02\x01`\x05\x01T`\x01\x86\x81T\x81\x10a\r\x1EWa\r\x1Ea\"\xA2V[\x90`\0R` `\0 \x90`\t\x02\x01`\x06\x01T\x92P\x92P\x92P\x91\x93\x90\x92PV[`\0\x80`\0\x80T`\x01\x14a\rcW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04\x0B\x90a\"\xB8V[`\0\x80\x80U\x80\x80\x80\x80a\r|`@\x8A\x01` \x8B\x01a#\xF0V[`\x01`\x01`\xA0\x1B\x03\x16c\x9F\x83\x13{a\r\x93`\x01T\x90V[a\r\xA0`\xA0\x8D\x01\x8Da$SV[`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\r\xBE\x93\x92\x91\x90a#fV[`\xA0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0E\x16W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a%\x8B\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x0E*W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0EN\x91\x90a%$V[\x94P\x94P\x94P\x94P\x94P\x84a\x0EjW`\0\x84\x12a\x0B\x8D\x85a\x16\xB0V[`\0`@Q\x80a\x01@\x01`@R\x80`\x01\x15\x15\x81R` \x013`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8B` \x01` \x81\x01\x90a\x0E\xA3\x91\x90a#\xF0V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01a\x0E\xC1``\x8D\x01`@\x8E\x01a#\xF0V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01a\x0E\xDF`\x80\x8D\x01``\x8E\x01a#\xF0V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x85\x81R` \x01\x84\x81R` \x01\x83\x81R` \x01g\r\xE0\xB6\xB3\xA7d\0\0\x81R` \x01\x8B`\x80\x015\x81RP\x90P`\x01\x81\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90`\0R` `\0 \x90`\t\x02\x01`\0\x90\x91\x90\x91\x90\x91P`\0\x82\x01Q\x81`\0\x01`\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP` \x82\x01Q\x81`\0\x01`\x01a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UP`@\x82\x01Q\x81`\x01\x01`\0a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UP``\x82\x01Q\x81`\x02\x01`\0a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UP`\x80\x82\x01Q\x81`\x03\x01`\0a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UP`\xA0\x82\x01Q\x81`\x04\x01U`\xC0\x82\x01Q\x81`\x05\x01U`\xE0\x82\x01Q\x81`\x06\x01Ua\x01\0\x82\x01Q\x81`\x07\x01Ua\x01 \x82\x01Q\x81`\x08\x01UPP\x81`\x02`\x003`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 `\0`\x01\x80\x80T\x90Pa\x10\x86\x91\x90a#SV[\x81R` \x80\x82\x01\x92\x90\x92R`@\x90\x81\x01`\0\x90\x81 \x93\x90\x93U3\x83R`\x03\x90\x91R\x81 `\x01\x80Tg\r\xE0\xB6\xB3\xA7d\0\0\x93\x91a\x10\xC1\x91a#SV[\x81R` \x01\x90\x81R` \x01`\0 \x81\x90UP\x89`@\x01` \x81\x01\x90a\x10\xE6\x91\x90a#\xF0V[`\x01`\x01`\xA0\x1B\x03\x16c#\xB8r\xDD30\x87`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x11\x15\x93\x92\x91\x90a#\x01V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x11mW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a%\x8B\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x11\x81W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\xA5\x91\x90a#5V[Pa\x11\xB6`\x80\x8B\x01``\x8C\x01a#\xF0V[`\x01`\x01`\xA0\x1B\x03\x16c#\xB8r\xDD30\x86`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x11\xE5\x93\x92\x91\x90a#\x01V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x12=W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a%\x8B\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x12QW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12u\x91\x90a#5V[Pa\x12\x86`@\x8B\x01` \x8C\x01a#\xF0V[`@\x80Q\x86\x81R` \x81\x01\x86\x90R\x90\x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x903\x90\x7F?\x1Fc\xA0\x1B[6Ff\x17!O4 E\xD7SS\x025<\xE7\t\xC2\x9E\xD3\xECL\x02H\xBD5\x90``\x01`@Q\x80\x91\x03\x90\xA3P`\x01`\0U\x91\x98\x90\x97P\x90\x95P\x93PPPPV[3`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 \x84\x84R\x90\x91R\x90 T\x15\x80\x15\x90a\x13SWP3`\0\x90\x81R`\x03` \x90\x81R`@\x80\x83 \x84\x84R\x90\x91R\x90 T`\x01\x80T\x83\x90\x81\x10a\x13>Wa\x13>a\"\xA2V[\x90`\0R` `\0 \x90`\t\x02\x01`\x07\x01T\x14\x15[\x15a\x13\xD2W3`\0\x90\x81R`\x03` \x90\x81R`@\x80\x83 \x84\x84R\x90\x91R\x81 T`\x01\x80Ta\x13\x8D\x92\x91\x90\x85\x90\x81\x10a\x0CSWa\x0CSa\"\xA2V[3`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 \x86\x84R\x90\x91R\x90 T\x90\x91Pa\x13\xB5\x90\x82a\x17\rV[3`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 \x86\x84R\x90\x91R\x90 UP[PV[`\0\x80`\0\x80`\0\x80`\0\x80`\x01\x8C\x81T\x81\x10a\x13\xF4Wa\x13\xF4a\"\xA2V[`\0\x91\x82R` \x90\x91 `\x01`\t\x90\x92\x02\x01\x01T`@Qb#\x8Bu`\xEA\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x8E-\xD4\0\x90a\x148\x90\x8F\x90\x8E\x90\x8E\x90`\x04\x01a#fV[`\xA0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x14\x8EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a%\x8B\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x14\xA2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14\xC6\x91\x90a%$V[\x94P\x94P\x94P\x94P\x94P\x84a\x14\xE2W`\0\x84\x12a\x0B\x8D\x85a\x16\xB0V[\x8Aa\x15\x1CW\x82`\x01\x8D\x81T\x81\x10a\x14\xFBWa\x14\xFBa\"\xA2V[\x90`\0R` `\0 \x90`\t\x02\x01`\x04\x01Ta\x15\x17\x91\x90a#SV[a\x15LV[`\x01\x8C\x81T\x81\x10a\x15/Wa\x15/a\"\xA2V[\x90`\0R` `\0 \x90`\t\x02\x01`\x04\x01T\x83a\x15L\x91\x90a#SV[\x97P\x8Aa\x15\x88W\x81`\x01\x8D\x81T\x81\x10a\x15gWa\x15ga\"\xA2V[\x90`\0R` `\0 \x90`\t\x02\x01`\x05\x01Ta\x15\x83\x91\x90a#SV[a\x15\xB8V[`\x01\x8C\x81T\x81\x10a\x15\x9BWa\x15\x9Ba\"\xA2V[\x90`\0R` `\0 \x90`\t\x02\x01`\x05\x01T\x82a\x15\xB8\x91\x90a#SV[\x96P\x8Aa\x15\xF4W\x80`\x01\x8D\x81T\x81\x10a\x15\xD3Wa\x15\xD3a\"\xA2V[\x90`\0R` `\0 \x90`\t\x02\x01`\x06\x01Ta\x15\xEF\x91\x90a#SV[a\x16$V[`\x01\x8C\x81T\x81\x10a\x16\x07Wa\x16\x07a\"\xA2V[\x90`\0R` `\0 \x90`\t\x02\x01`\x06\x01T\x81a\x16$\x91\x90a#SV[\x95P\x82`\x01\x8D\x81T\x81\x10a\x16:Wa\x16:a\"\xA2V[\x90`\0R` `\0 \x90`\t\x02\x01`\x04\x01\x81\x90UP\x81`\x01\x8D\x81T\x81\x10a\x16cWa\x16ca\"\xA2V[\x90`\0R` `\0 \x90`\t\x02\x01`\x05\x01\x81\x90UP\x80`\x01\x8D\x81T\x81\x10a\x16\x8CWa\x16\x8Ca\"\xA2V[\x90`\0R` `\0 \x90`\t\x02\x01`\x06\x01\x81\x90UPPPPPP\x94P\x94P\x94\x91PPV[`\0`\x01`\xFF\x1B\x82\x03a\x16\xD6W`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x12\x15a\x16\xE7WP\x19`\x01\x01\x90V[P\x80[\x91\x90PV[`\0a\x17\x04\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x1E\xB2V[\x90P[\x92\x91PPV[`\0a\x17\x04\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x1E\xB2V[`\0\x80`\0\x80`\0\x80`\x01\x89\x81T\x81\x10a\x17>Wa\x17>a\"\xA2V[\x90`\0R` `\0 \x90`\t\x02\x01`\x04\x01T`\x01\x8A\x81T\x81\x10a\x17cWa\x17ca\"\xA2V[\x90`\0R` `\0 \x90`\t\x02\x01`\x05\x01T\x91P\x91P\x81\x88\x11\x15a\x18YW`\x01\x89\x81T\x81\x10a\x17\x94Wa\x17\x94a\"\xA2V[\x90`\0R` `\0 \x90`\t\x02\x01`\x02\x01`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x95P`\x01\x89\x81T\x81\x10a\x17\xCEWa\x17\xCEa\"\xA2V[`\0\x91\x82R` \x90\x91 `\x03`\t\x90\x92\x02\x01\x01T`\x01`\x01`\xA0\x1B\x03\x16\x94Pa\x17\xF7\x82\x89a#SV[\x93P\x86\x81\x11a\x18HW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7Finvalid swap: inputs x and y\0\0\0\0`D\x82\x01R`d\x01a\x04\x0BV[a\x18R\x87\x82a#SV[\x92Pa\x19-V[`\x01\x89\x81T\x81\x10a\x18lWa\x18la\"\xA2V[\x90`\0R` `\0 \x90`\t\x02\x01`\x03\x01`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x95P`\x01\x89\x81T\x81\x10a\x18\xA6Wa\x18\xA6a\"\xA2V[`\0\x91\x82R` \x90\x91 `\x02`\t\x90\x92\x02\x01\x01T`\x01`\x01`\xA0\x1B\x03\x16\x94Pa\x18\xCF\x81\x88a#SV[\x93P\x87\x82\x11a\x19 W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7Finvalid swap: inputs x and y\0\0\0\0`D\x82\x01R`d\x01a\x04\x0BV[a\x19*\x88\x83a#SV[\x92P[\x87`\x01\x8A\x81T\x81\x10a\x19AWa\x19Aa\"\xA2V[\x90`\0R` `\0 \x90`\t\x02\x01`\x04\x01\x81\x90UP\x86`\x01\x8A\x81T\x81\x10a\x19jWa\x19ja\"\xA2V[`\0\x91\x82R` \x82 `\t\x91\x90\x91\x02\x01`\x05\x01\x91\x90\x91U`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x88\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x19\xFEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a%\x8B\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x1A\x12W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A6\x91\x90a%nV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x88\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x1A\xB9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a%\x8B\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x1A\xCDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A\xF1\x91\x90a%nV[`@Qc#\xB8r\xDD`\xE0\x1B\x81R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x89\x16\x90c#\xB8r\xDD\x90a\x1B$\x903\x900\x90\x8B\x90`\x04\x01a#\x01V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1B|W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a%\x8B\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x1B\x90W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B\xB4\x91\x90a#5V[P`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x86\x90R`\x01`\x01`\xA0\x1B\x03\x88\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1C;W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a%\x8B\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x1COW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1Cs\x91\x90a#5V[Pa\x1C~\x86\x83a\"\xEEV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x8A\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x1C\xFBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a%\x8B\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x1D\x0FW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1D3\x91\x90a%nV[\x10\x15a\x1D\x8CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: input token transf`D\x82\x01Ra2\xB9`\xF1\x1B`d\x82\x01R`\x84\x01a\x04\x0BV[a\x1D\x96\x85\x82a#SV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x89\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x1E\x13W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a%\x8B\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x1E'W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1EK\x91\x90a%nV[\x10\x15a\x1E\xA5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7Finvalid swap: output token trans`D\x82\x01Rb32\xB9`\xE9\x1B`d\x82\x01R`\x84\x01a\x04\x0BV[PPPP\x93P\x93P\x93P\x93V[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x1E\xCAW`\0\x80\xFD[\x04\x92\x91PPV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x16\xEAW`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x1F\x9EWa\x1F\x9Ea\x1E\xD1V[a\x1F\xA7\x83a\x1FqV[\x94` \x93\x90\x93\x015\x93PPPV[`\0` \x82\x84\x03\x12\x15a\x1F\xCAWa\x1F\xCAa\x1E\xD1V[P5\x91\x90PV[\x81Q\x15\x15\x81Ra\x01@\x81\x01` \x83\x01Qa\x1F\xF6` \x84\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P`@\x83\x01Qa \x11`@\x84\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P``\x83\x01Qa ,``\x84\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P`\x80\x83\x01Qa G`\x80\x84\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P`\xA0\x83\x01Q`\xA0\x83\x01R`\xC0\x83\x01Q`\xC0\x83\x01R`\xE0\x83\x01Q`\xE0\x83\x01Ra\x01\0\x80\x84\x01Q\x81\x84\x01RPa\x01 \x80\x84\x01Q\x81\x84\x01RP\x92\x91PPV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rjrray stride`\xA8\x1B`d\x82\x01R`\x84\x81\xFD[`\0\x80`\0`@\x84\x86\x03\x12\x15a \xF5Wa \xF5a\x1E\xD1V[\x835\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a!\x17Wa!\x17a\x1F!V[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a!\x7FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x81\xFD[\x815\x81\x81\x11\x15a!\xE2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x0ENL/$\r\x8C\xAD\xCC\xEE\x8D`\xAB\x1B`d\x82\x01R`\x84\x81\xFD[\x87` \x82\x85\x01\x01\x11\x15a!\xF7Wa!\xF7a \x84V[` \x83\x01\x94P\x80\x93PPPP\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a\"\x1FWa\"\x1Fa\x1E\xD1V[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\"9Wa\"9a\x1F!V[\x82\x01`\xC0\x81\x85\x03\x12\x15a\"\x9BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FABI decoding: struct calldata to`D\x82\x01Rf\x1B\xC8\x1C\xDA\x1B\xDC\x9D`\xCA\x1B`d\x82\x01R`\x84\x81\xFD[\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[` \x80\x82R`\x06\x90\x82\x01Re\x1B\x1B\xD8\xDA\xD9Y`\xD2\x1B`@\x82\x01R``\x01\x90V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x17\x07Wa\x17\x07a\"\xD8V[`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x81R\x91\x90\x92\x16` \x82\x01R`@\x81\x01\x91\x90\x91R``\x01\x90V[\x80Q\x80\x15\x15\x81\x14a\x16\xEAW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a#JWa#Ja\x1E\xD1V[a\x17\x04\x82a#%V[\x81\x81\x03\x81\x81\x11\x15a\x17\x07Wa\x17\x07a\"\xD8V[\x83\x81R`@` \x82\x01R\x81`@\x82\x01R\x81\x83``\x83\x017`\0\x81\x83\x01``\x90\x81\x01\x91\x90\x91R`\x1F\x90\x92\x01`\x1F\x19\x16\x01\x01\x92\x91PPV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a#\xB8Wa#\xB8a\x1E\xD1V[a#\xC1\x87a#%V[\x95P` \x87\x01Q\x94P`@\x87\x01Q\x93P``\x87\x01Q\x92P`\x80\x87\x01Q\x91P`\xA0\x87\x01Q\x90P\x92\x95P\x92\x95P\x92\x95V[`\0` \x82\x84\x03\x12\x15a$\x05Wa$\x05a\x1E\xD1V[a\x17\x04\x82a\x1FqV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FCalldata tail too short\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x81\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a$\xAAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FInvalid calldata tail offset\0\0\0\0`D\x82\x01R`d\x81\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a%\x05W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FInvalid calldata tail length\0\0\0\0`D\x82\x01R`d\x81\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a%\x1DWa%\x1Da$\x0EV[\x92P\x92\x90PV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a%?Wa%?a\x1E\xD1V[a%H\x86a#%V[` \x87\x01Q`@\x88\x01Q``\x89\x01Q`\x80\x90\x99\x01Q\x92\x9A\x91\x99P\x97\x96P\x90\x94P\x92PPPV[`\0` \x82\x84\x03\x12\x15a%\x83Wa%\x83a\x1E\xD1V[PQ\x91\x90PV\xFETarget contract does not contain";
    /// The deployed bytecode of the contract.
    pub static MULTIDFMM_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct MultiDFMM<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MultiDFMM<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MultiDFMM<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MultiDFMM<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MultiDFMM<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MultiDFMM)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MultiDFMM<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    MULTIDFMM_ABI.clone(),
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
                MULTIDFMM_ABI.clone(),
                MULTIDFMM_BYTECODE.clone().into(),
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
        ///Calls the contract's `feeGrowthLast` (0x14ae342f) function
        pub fn fee_growth_last(
            &self,
            account: ::ethers::core::types::Address,
            pool_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([20, 174, 52, 47], (account, pool_id))
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
        ///Calls the contract's `init` (0xe676c3ac) function
        pub fn init(
            &self,
            params: InitParams,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([230, 118, 195, 172], (params,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `locked` (0xcf309012) function
        pub fn locked(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([207, 48, 144, 18], ())
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
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([189, 6, 37, 171], (pool_id, data))
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
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            MultiDFMMEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for MultiDFMM<M> {
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
    ///Custom Error type `InvalidSwap` with signature `InvalidSwap()` and selector `0x11157667`
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
    #[etherror(name = "InvalidSwap", abi = "InvalidSwap()")]
    pub struct InvalidSwap;
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
    pub enum MultiDFMMErrors {
        Invalid(Invalid),
        InvalidSwap(InvalidSwap),
        Min(Min),
        NotInitialized(NotInitialized),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for MultiDFMMErrors {
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
            if let Ok(decoded) = <InvalidSwap as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidSwap(decoded));
            }
            if let Ok(decoded) = <Min as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Min(decoded));
            }
            if let Ok(decoded) = <NotInitialized as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NotInitialized(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MultiDFMMErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::Invalid(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InvalidSwap(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Min(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NotInitialized(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for MultiDFMMErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <Invalid as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <InvalidSwap as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <Min as ::ethers::contract::EthError>::selector() => {
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
    impl ::core::fmt::Display for MultiDFMMErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Invalid(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidSwap(element) => ::core::fmt::Display::fmt(element, f),
                Self::Min(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotInitialized(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for MultiDFMMErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<Invalid> for MultiDFMMErrors {
        fn from(value: Invalid) -> Self {
            Self::Invalid(value)
        }
    }
    impl ::core::convert::From<InvalidSwap> for MultiDFMMErrors {
        fn from(value: InvalidSwap) -> Self {
            Self::InvalidSwap(value)
        }
    }
    impl ::core::convert::From<Min> for MultiDFMMErrors {
        fn from(value: Min) -> Self {
            Self::Min(value)
        }
    }
    impl ::core::convert::From<NotInitialized> for MultiDFMMErrors {
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
    #[ethevent(name = "Allocate", abi = "Allocate(uint256,uint256,uint256)")]
    pub struct AllocateFilter {
        pub x: ::ethers::core::types::U256,
        pub y: ::ethers::core::types::U256,
        pub l: ::ethers::core::types::U256,
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
    #[ethevent(name = "Deallocate", abi = "Deallocate(uint256,uint256,uint256)")]
    pub struct DeallocateFilter {
        pub x: ::ethers::core::types::U256,
        pub y: ::ethers::core::types::U256,
        pub l: ::ethers::core::types::U256,
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
    #[ethevent(name = "Init", abi = "Init(address,address,uint256,uint256,uint256)")]
    pub struct InitFilter {
        #[ethevent(indexed)]
        pub swapper: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub strategy: ::ethers::core::types::Address,
        pub xxxxxxx: ::ethers::core::types::U256,
        pub yyyyyy: ::ethers::core::types::U256,
        pub llllll: ::ethers::core::types::U256,
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
    #[ethevent(
        name = "Swap",
        abi = "Swap(address,address,address,address,uint256,uint256,int256)"
    )]
    pub struct SwapFilter {
        #[ethevent(indexed)]
        pub swapper: ::ethers::core::types::Address,
        pub source: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub token_in: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub token_out: ::ethers::core::types::Address,
        pub amount_in: ::ethers::core::types::U256,
        pub amount_out: ::ethers::core::types::U256,
        pub liquidity_delta: ::ethers::core::types::I256,
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
    pub enum MultiDFMMEvents {
        AllocateFilter(AllocateFilter),
        DeallocateFilter(DeallocateFilter),
        InitFilter(InitFilter),
        LogPoolStatsFilter(LogPoolStatsFilter),
        SwapFilter(SwapFilter),
    }
    impl ::ethers::contract::EthLogDecode for MultiDFMMEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AllocateFilter::decode_log(log) {
                return Ok(MultiDFMMEvents::AllocateFilter(decoded));
            }
            if let Ok(decoded) = DeallocateFilter::decode_log(log) {
                return Ok(MultiDFMMEvents::DeallocateFilter(decoded));
            }
            if let Ok(decoded) = InitFilter::decode_log(log) {
                return Ok(MultiDFMMEvents::InitFilter(decoded));
            }
            if let Ok(decoded) = LogPoolStatsFilter::decode_log(log) {
                return Ok(MultiDFMMEvents::LogPoolStatsFilter(decoded));
            }
            if let Ok(decoded) = SwapFilter::decode_log(log) {
                return Ok(MultiDFMMEvents::SwapFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for MultiDFMMEvents {
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
    impl ::core::convert::From<AllocateFilter> for MultiDFMMEvents {
        fn from(value: AllocateFilter) -> Self {
            Self::AllocateFilter(value)
        }
    }
    impl ::core::convert::From<DeallocateFilter> for MultiDFMMEvents {
        fn from(value: DeallocateFilter) -> Self {
            Self::DeallocateFilter(value)
        }
    }
    impl ::core::convert::From<InitFilter> for MultiDFMMEvents {
        fn from(value: InitFilter) -> Self {
            Self::InitFilter(value)
        }
    }
    impl ::core::convert::From<LogPoolStatsFilter> for MultiDFMMEvents {
        fn from(value: LogPoolStatsFilter) -> Self {
            Self::LogPoolStatsFilter(value)
        }
    }
    impl ::core::convert::From<SwapFilter> for MultiDFMMEvents {
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
    ///Container type for all input parameters for the `feeGrowthLast` function with signature `feeGrowthLast(address,uint256)` and selector `0x14ae342f`
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
    #[ethcall(name = "feeGrowthLast", abi = "feeGrowthLast(address,uint256)")]
    pub struct FeeGrowthLastCall {
        pub account: ::ethers::core::types::Address,
        pub pool_id: ::ethers::core::types::U256,
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
    ///Container type for all input parameters for the `init` function with signature `init((uint256,address,address,address,uint256,bytes))` and selector `0xe676c3ac`
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
        name = "init",
        abi = "init((uint256,address,address,address,uint256,bytes))"
    )]
    pub struct InitCall {
        pub params: InitParams,
    }
    ///Container type for all input parameters for the `locked` function with signature `locked()` and selector `0xcf309012`
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
    #[ethcall(name = "locked", abi = "locked()")]
    pub struct LockedCall;
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
    pub enum MultiDFMMCalls {
        Allocate(AllocateCall),
        BalanceOf(BalanceOfCall),
        Deallocate(DeallocateCall),
        FeeGrowthLast(FeeGrowthLastCall),
        GetPool(GetPoolCall),
        GetReservesAndLiquidity(GetReservesAndLiquidityCall),
        Init(InitCall),
        Locked(LockedCall),
        Nonce(NonceCall),
        Pools(PoolsCall),
        Swap(SwapCall),
    }
    impl ::ethers::core::abi::AbiDecode for MultiDFMMCalls {
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
            if let Ok(decoded) = <FeeGrowthLastCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FeeGrowthLast(decoded));
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
            if let Ok(decoded) = <LockedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Locked(decoded));
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
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MultiDFMMCalls {
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
                Self::FeeGrowthLast(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetPool(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetReservesAndLiquidity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Init(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Locked(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Nonce(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Pools(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Swap(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for MultiDFMMCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Allocate(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::Deallocate(element) => ::core::fmt::Display::fmt(element, f),
                Self::FeeGrowthLast(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPool(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetReservesAndLiquidity(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Init(element) => ::core::fmt::Display::fmt(element, f),
                Self::Locked(element) => ::core::fmt::Display::fmt(element, f),
                Self::Nonce(element) => ::core::fmt::Display::fmt(element, f),
                Self::Pools(element) => ::core::fmt::Display::fmt(element, f),
                Self::Swap(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AllocateCall> for MultiDFMMCalls {
        fn from(value: AllocateCall) -> Self {
            Self::Allocate(value)
        }
    }
    impl ::core::convert::From<BalanceOfCall> for MultiDFMMCalls {
        fn from(value: BalanceOfCall) -> Self {
            Self::BalanceOf(value)
        }
    }
    impl ::core::convert::From<DeallocateCall> for MultiDFMMCalls {
        fn from(value: DeallocateCall) -> Self {
            Self::Deallocate(value)
        }
    }
    impl ::core::convert::From<FeeGrowthLastCall> for MultiDFMMCalls {
        fn from(value: FeeGrowthLastCall) -> Self {
            Self::FeeGrowthLast(value)
        }
    }
    impl ::core::convert::From<GetPoolCall> for MultiDFMMCalls {
        fn from(value: GetPoolCall) -> Self {
            Self::GetPool(value)
        }
    }
    impl ::core::convert::From<GetReservesAndLiquidityCall> for MultiDFMMCalls {
        fn from(value: GetReservesAndLiquidityCall) -> Self {
            Self::GetReservesAndLiquidity(value)
        }
    }
    impl ::core::convert::From<InitCall> for MultiDFMMCalls {
        fn from(value: InitCall) -> Self {
            Self::Init(value)
        }
    }
    impl ::core::convert::From<LockedCall> for MultiDFMMCalls {
        fn from(value: LockedCall) -> Self {
            Self::Locked(value)
        }
    }
    impl ::core::convert::From<NonceCall> for MultiDFMMCalls {
        fn from(value: NonceCall) -> Self {
            Self::Nonce(value)
        }
    }
    impl ::core::convert::From<PoolsCall> for MultiDFMMCalls {
        fn from(value: PoolsCall) -> Self {
            Self::Pools(value)
        }
    }
    impl ::core::convert::From<SwapCall> for MultiDFMMCalls {
        fn from(value: SwapCall) -> Self {
            Self::Swap(value)
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
    ///Container type for all return fields from the `feeGrowthLast` function with signature `feeGrowthLast(address,uint256)` and selector `0x14ae342f`
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
    pub struct FeeGrowthLastReturn {
        pub balance: ::ethers::core::types::U256,
    }
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
    ///Container type for all return fields from the `init` function with signature `init((uint256,address,address,address,uint256,bytes))` and selector `0xe676c3ac`
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
    );
    ///Container type for all return fields from the `locked` function with signature `locked()` and selector `0xcf309012`
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
    pub struct LockedReturn(pub ::ethers::core::types::U256);
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
        pub reserve_x_wad: ::ethers::core::types::U256,
        pub reserve_y_wad: ::ethers::core::types::U256,
        pub total_liquidity: ::ethers::core::types::U256,
        pub fee_growth: ::ethers::core::types::U256,
        pub swap_fee_percentage_wad: ::ethers::core::types::U256,
    }
    ///`Pool(bool,address,address,address,address,uint256,uint256,uint256,uint256,uint256)`
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
        pub reserve_x_wad: ::ethers::core::types::U256,
        pub reserve_y_wad: ::ethers::core::types::U256,
        pub total_liquidity: ::ethers::core::types::U256,
        pub fee_growth: ::ethers::core::types::U256,
        pub swap_fee_percentage_wad: ::ethers::core::types::U256,
    }
}
