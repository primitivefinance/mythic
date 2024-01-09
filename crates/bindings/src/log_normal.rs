pub use log_normal::*;
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
pub mod log_normal {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_core"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_swapFee"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("computeSwapConstant"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "computeSwapConstant",
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("core"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("core"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IMultiCore"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("dynamicSlot"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("dynamicSlot"),
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
                                    name: ::std::borrow::ToOwned::to_owned("params"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("dynamicSlotInternal"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "dynamicSlotInternal",
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
                                    name: ::std::borrow::ToOwned::to_owned("params"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct LogNormParameters"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getParams"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getParams"),
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
                                    name: ::std::borrow::ToOwned::to_owned("valid"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("invariant"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("rx"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("ry"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("L"),
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
                    ::std::borrow::ToOwned::to_owned("setSigma"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setSigma"),
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
                                    name: ::std::borrow::ToOwned::to_owned("newTargetSigma"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newSigmaUpdateEnd"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setStrikePrice"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setStrikePrice"),
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
                                    name: ::std::borrow::ToOwned::to_owned("newTargetStrike"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newStrikeUpdateEnd",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setTau"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setTau"),
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
                                    name: ::std::borrow::ToOwned::to_owned("newTargetTau"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newTauUpdateEnd"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("sigma"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("sigma"),
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
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("sigmas"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("sigmas"),
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
                                    name: ::std::borrow::ToOwned::to_owned("target"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("last"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("updateEnd"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("updatePerSecond"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("lastSync"),
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
                    ::std::borrow::ToOwned::to_owned("slots"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("slots"),
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
                                    name: ::std::borrow::ToOwned::to_owned("strike"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sigma"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tau"),
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
                    ::std::borrow::ToOwned::to_owned("staticSlot"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("staticSlot"),
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
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct LogNormParameters"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("strikePrice"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("strikePrice"),
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
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("strikes"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("strikes"),
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
                                    name: ::std::borrow::ToOwned::to_owned("target"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("last"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("updateEnd"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("updatePerSecond"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("lastSync"),
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
                    ::std::borrow::ToOwned::to_owned("swapFee"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("swapFee"),
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
                    ::std::borrow::ToOwned::to_owned("tau"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("tau"),
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
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("taus"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("taus"),
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
                                    name: ::std::borrow::ToOwned::to_owned("target"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("last"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("updateEnd"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("updatePerSecond"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("lastSync"),
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
                    ::std::borrow::ToOwned::to_owned("validateAllocateOrDeallocate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "validateAllocateOrDeallocate",
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
                                    name: ::std::borrow::ToOwned::to_owned("valid"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("invariant"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("rx"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("ry"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("L"),
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
                    ::std::borrow::ToOwned::to_owned("validateSwap"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("validateSwap"),
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
                                    name: ::std::borrow::ToOwned::to_owned("valid"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("invariant"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("liquidityDelta"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("nextRx"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("nextRy"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("nextL"),
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
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("SetSigma"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("SetSigma"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("targetSigma"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("lastSigma"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sigmaUpdateEnd"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("delta"),
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
                    ::std::borrow::ToOwned::to_owned("SetStrikePrice"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("SetStrikePrice"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("targetStrike"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("lastStrike"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("strikeUpdateEnd"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("delta"),
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
                    ::std::borrow::ToOwned::to_owned("SetTau"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("SetTau"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("targetTau"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("lastTau"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("tauUpdateEnd"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("delta"),
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
                    ::std::borrow::ToOwned::to_owned("Infinity"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("Infinity"),
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
                    ::std::borrow::ToOwned::to_owned("NegativeInfinity"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("NegativeInfinity"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OutOfBounds"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("OutOfBounds"),
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
    pub static LOGNORMAL_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15b\0\0^W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`@Qb\0'\x038\x03\x80b\0'\x03\x839\x81\x01`@\x81\x90Rb\0\0\x81\x91b\0\x01\x19V[g\r\xE0\xB6\xB3\xA7d\0\0\x81\x10b\0\0\xF0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7Fswap fee percentage must be less`D\x82\x01Ri than 100%`\xB0\x1B`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[`\x01U`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90Ub\0\x01\xA0V[`\0\x80`@\x83\x85\x03\x12\x15b\0\x01xW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[\x82Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x01\x90W`\0\x80\xFD[` \x93\x90\x93\x01Q\x92\x94\x92\x93PPPV[a%S\x80b\0\x01\xB0`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x01\x83W`\x005`\xE0\x1C\x80c\x8C3\x88\xDC\x11a\x01\x05W\x80c\xCE\x15;\xF4\x11a\0\xC9W\x80c\xCE\x15;\xF4\x14a\x04TW\x80c\xD7\x10\x955\x14a\x04gW\x80c\xE8\0\xBDD\x14a\x04zW\x80c\xECM\xC1[\x14a\x04\x8DW\x80c\xF2\xF4\xEB&\x14a\x04\xC2W\x80c\xFC\x13\x05\xE5\x14a\x04\xEDWa\x01\x83V[\x80c\x8C3\x88\xDC\x14a\x03\x93W\x80c\x8C\xC1\xC1\xAD\x14a\x03\xD1W\x80c\x8E-\xD4\0\x14a\x03\xE4W\x80c\x9F\x83\x13{\x14a\x04!W\x80c\xA4\xD4z^\x14a\x044Wa\x01\x83V[\x80c8}\xD9\xE9\x11a\x01LW\x80c8}\xD9\xE9\x14a\x02\xF6W\x80c?C\xBA\xAC\x14a\x03$W\x80cT\xCF*\xEB\x14a\x039W\x80crQ0\x8C\x14a\x03BW\x80cy$\x17\xB1\x14a\x03UWa\x01\x83V[\x80b.RK\x14a\x01\xE8W\x80c\x08\xA4\xF0r\x14a\x02\x0EW\x80c\x1336\xAA\x14a\x02<W\x80c\x16\x98\xAAI\x14a\x02OW\x80c2\x14\x89\x0F\x14a\x02\xB4W[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x01\xFBa\x01\xF66`\x04a mV[a\x05\0V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x02!a\x02\x1C6`\x04a!eV[a\x05@V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x02\x05V[a\x01\xFBa\x02J6`\x04a!eV[a\x05mV[a\x02\x8Ca\x02]6`\x04a!eV[`\x05` R`\0\x90\x81R`@\x90 \x80T`\x01\x82\x01T`\x02\x83\x01T`\x03\x84\x01T`\x04\x90\x94\x01T\x92\x93\x91\x92\x90\x91\x90\x85V[`@\x80Q\x95\x86R` \x86\x01\x94\x90\x94R\x92\x84\x01\x91\x90\x91R``\x83\x01R`\x80\x82\x01R`\xA0\x01a\x02\x05V[a\x02\xC7a\x02\xC26`\x04a mV[a\x065V[`@\x80Q\x96\x15\x15\x87R` \x87\x01\x95\x90\x95R\x93\x85\x01\x92\x90\x92R``\x84\x01R`\x80\x83\x01R`\xA0\x82\x01R`\xC0\x01a\x02\x05V[a\x02!a\x03\x046`\x04a!eV[`\x02` \x81\x90R`\0\x91\x82R`@\x90\x91 \x80T`\x01\x82\x01T\x91\x90\x92\x01T\x83V[a\x037a\x0326`\x04a!\x81V[a\x07\xC4V[\0[a\x01\xFB`\x01T\x81V[a\x037a\x03P6`\x04a!\x81V[a\t;V[a\x02\x8Ca\x03c6`\x04a!eV[`\x04` \x81\x90R`\0\x91\x82R`@\x90\x91 \x80T`\x01\x82\x01T`\x02\x83\x01T`\x03\x84\x01T\x93\x90\x94\x01T\x91\x93\x90\x92\x90\x91\x85V[a\x02\x8Ca\x03\xA16`\x04a!eV[`\x03` \x81\x90R`\0\x91\x82R`@\x90\x91 \x80T`\x01\x82\x01T`\x02\x83\x01T\x93\x83\x01T`\x04\x90\x93\x01T\x91\x93\x90\x92\x90\x91\x85V[a\x01\xFBa\x03\xDF6`\x04a!eV[a\nwV[a\x03\xF7a\x03\xF26`\x04a!\xB0V[a\n\xCBV[`@\x80Q\x95\x15\x15\x86R` \x86\x01\x94\x90\x94R\x92\x84\x01\x91\x90\x91R``\x83\x01R`\x80\x82\x01R`\xA0\x01a\x02\x05V[a\x03\xF7a\x04/6`\x04a!\xB0V[a\x0B\x1AV[a\x04Ga\x04B6`\x04a!eV[a\x0BrV[`@Qa\x02\x05\x91\x90a\"\xDFV[a\x02!a\x04b6`\x04a!eV[a\x0B\xBFV[a\x037a\x04u6`\x04a!\x81V[a\x0C\x90V[a\x01\xFBa\x04\x886`\x04a!eV[a\r\xC6V[a\x04\xA0a\x04\x9B6`\x04a!eV[a\x0E\x1BV[`@\x80Q\x82Q\x81R` \x80\x84\x01Q\x90\x82\x01R\x91\x81\x01Q\x90\x82\x01R``\x01a\x02\x05V[`\0Ta\x04\xD5\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\x05V[a\x04\xA0a\x04\xFB6`\x04a!eV[a\x0EkV[`\0\x80`\0\x80\x84\x80` \x01\x90Q\x81\x01\x90a\x05\x1A\x91\x90a#-V[\x92P\x92P\x92Pa\x054\x83\x83\x83a\x05/\x8Aa\x0E\x1BV[a\x0E\xC8V[\x93PPPP[\x92\x91PPV[`\0\x80`\0a\x05N\x84a\r\xC6V[a\x05W\x85a\x05mV[a\x05`\x86a\nwV[\x92P\x92P\x92P\x91\x93\x90\x92PV[`\0\x81\x81R`\x03` \x81\x81R`@\x80\x84 \x81Q`\xA0\x81\x01\x83R\x81T\x81R`\x01\x82\x01T\x93\x81\x01\x93\x90\x93R`\x02\x81\x01T\x91\x83\x01\x82\x90R\x92\x83\x01T``\x83\x01R`\x04\x90\x92\x01T`\x80\x82\x01R\x90B\x10a\x05\xC3WQ\x92\x91PPV[\x80Q` \x82\x01Q\x11a\x06\x01W``\x81\x01Q`\x80\x82\x01Qa\x05\xE3\x90Ba#tV[a\x05\xED\x91\x90a#\x87V[\x81` \x01Qa\x05\xFC\x91\x90a#\x9EV[a\x06.V[``\x81\x01Q`\x80\x82\x01Qa\x06\x15\x90Ba#tV[a\x06\x1F\x91\x90a#\x87V[\x81` \x01Qa\x06.\x91\x90a#tV[\x93\x92PPPV[`\0\x80`\0\x80`\0\x80`\0\x80`\0a\x06L\x8Ba\x0B\xBFV[\x92P\x92P\x92P\x89\x80` \x01\x90Q\x81\x01\x90a\x06f\x91\x90a#-V[\x91\x97P\x95P\x93P`\0\x80\x80\x85\x89\x11\x15a\x06\xC1Wa\x06\x83\x86\x8Aa#tV[\x91Pa\x06\x9A`\x01T\x83a\x0F\xDA\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa\x06\xB0\x86a\x06\xAA\x83\x87a\x0F\xDAV[\x90a\x0F\xEFV[a\x06\xBA\x90\x84a#\x9EV[\x92Pa\x07`V[\x84\x88\x11\x15a\x06\xFAWa\x06\xD3\x85\x89a#tV[\x91Pa\x06\xEA`\x01T\x83a\x0F\xDA\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa\x06\xB0\x85a\x06\xAA\x83\x87a\x0F\xDAV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`0`$\x82\x01R\x7Finvalid swap: inputs x and y hav`D\x82\x01Roe the same sign!`\x80\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[a\x07j\x84\x88a#\xB1V[\x99P`\0a\x07w\x8Fa\x0E\x1BV[\x90Pa\x07\x85\x8A\x8A\x8A\x84a\x0E\xC8V[\x9BP`\0\x8Ca\x07\x94`\x1Ea#\xD8V[\x12\x80\x15a\x07\xA1WP`\x1E\x8D\x12[\x90P\x80\x80\x15a\x07\xB0WP\x84\x8C\x12\x15[\x9DPPPPPPPPP\x92\x95P\x92\x95P\x92\x95V[B\x81\x11a\x07\xE3W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07W\x90a#\xF4V[a\x07\xEC\x83a\x10\x04V[`\0\x83\x81R`\x05` \x90\x81R`@\x80\x83 \x81Q`\xA0\x81\x01\x83R\x81T\x81R`\x01\x82\x01T\x93\x81\x01\x84\x90R`\x02\x82\x01T\x92\x81\x01\x92\x90\x92R`\x03\x81\x01T``\x83\x01R`\x04\x01T`\x80\x82\x01R\x91\x90\x84\x10a\x08OW` \x82\x01Qa\x08J\x90\x85a#tV[a\x08_V[\x83\x82` \x01Qa\x08_\x91\x90a#tV[\x90Pa\x08kB\x84a#tV[a\x08u\x90\x82a$\x1FV[``\x83\x01\x90\x81R\x84\x83R`@\x80\x84\x01\x85\x81R`\0\x88\x81R`\x05` \x90\x81R\x92\x90 \x85Q\x80\x82U\x92\x86\x01Q`\x01\x82\x01\x81\x90U\x91Q`\x02\x82\x01\x81\x90U\x93Q`\x03\x82\x01U`\x80\x86\x01Q`\x04\x90\x91\x01U\x7F\x1EK Gg&%\xF4\xDF!\xD4|\xA7\xA0\x0C \x12\xCC\xF7x\xF2w\xBF2(\xDD\xF4Y=o\x84\xAA\x92\x81\x83\x11a\t\0W\x85Q` \x87\x01Qa\x08\xFB\x91\x90a#tV[a\t\x11V[` \x86\x01Q\x86Qa\t\x11\x91\x90a#tV[`@\x80Q\x94\x85R` \x85\x01\x93\x90\x93R\x91\x83\x01R``\x82\x01R`\x80\x01`@Q\x80\x91\x03\x90\xA1PPPPPV[B\x81\x11a\tZW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07W\x90a#\xF4V[a\tc\x83a\x10\xA1V[`\0\x83\x81R`\x03` \x81\x81R`@\x80\x84 \x81Q`\xA0\x81\x01\x83R\x81T\x81R`\x01\x82\x01T\x93\x81\x01\x84\x90R`\x02\x82\x01T\x92\x81\x01\x92\x90\x92R\x92\x83\x01T``\x82\x01R`\x04\x90\x92\x01T`\x80\x83\x01R\x90\x91\x90\x84\x10a\t\xC8W` \x82\x01Qa\t\xC3\x90\x85a#tV[a\t\xD8V[\x83\x82` \x01Qa\t\xD8\x91\x90a#tV[\x90Pa\t\xE4B\x84a#tV[a\t\xEE\x90\x82a$\x1FV[``\x83\x01\x90\x81R\x84\x83R`@\x80\x84\x01\x85\x81R`\0\x88\x81R`\x03` \x81\x81R\x93\x90\x91 \x86Q\x80\x82U\x93\x87\x01Q`\x01\x82\x01\x81\x90U\x92Q`\x02\x82\x01\x81\x90U\x94Q\x91\x81\x01\x91\x90\x91U`\x80\x86\x01Q`\x04\x90\x91\x01U\x7F\xB9hO\xE86\x87_\xAF\x05m\xB2]\xE9\x85S\x859`\xB3&\x08\x8A\x10!\xAF\xA7[\xA1`\x12d\xF5\x92\x81\x83\x11a\t\0W\x85Q` \x87\x01Qa\x08\xFB\x91\x90a#tV[`\0\x81\x81R`\x05` \x90\x81R`@\x80\x83 \x81Q`\xA0\x81\x01\x83R\x81T\x81R`\x01\x82\x01T\x93\x81\x01\x93\x90\x93R`\x02\x81\x01T\x91\x83\x01\x82\x90R`\x03\x81\x01T``\x84\x01R`\x04\x01T`\x80\x83\x01RB\x10a\x05\xC3WQ\x92\x91PPV[`\0\x80\x80\x80\x80a\n\xDD\x86\x88\x01\x88a!\x81V[\x91\x94P\x92P\x90Pa\n\xF3\x83\x83\x83a\x05/\x8Ca\x0E\x1BV[\x93P\x83a\x0B\0`\x1Ea#\xD8V[\x12\x80\x15a\x0B\rWP`\x1E\x84\x12[\x94P\x93\x97\x92\x96P\x93P\x93PV[`\0\x80\x80\x80\x80a\x0B,\x86\x88\x01\x88a$AV[`\0\x8C\x81R`\x02` \x81\x81R`@\x92\x83\x90 \x84Q\x81U\x90\x84\x01Q`\x01\x82\x01U\x92\x90\x91\x01Q\x91\x01U\x91\x94P\x92P\x90Pa\x0Bc\x88a\x11@V[a\n\xF3\x83\x83\x83a\x05/\x8Ca\x0E\x1BV[``a\x0B}\x82a\r\xC6V[a\x0B\x86\x83a\x05mV[a\x0B\x8F\x84a\nwV[`@\x80Q` \x81\x01\x94\x90\x94R\x83\x01\x91\x90\x91R``\x82\x01R`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x91\x90PV[`\0\x80T`@Qc3\x85N\xFD`\xE2\x1B\x81R`\x04\x81\x01\x84\x90R\x82\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xCE\x15;\xF4\x90`$\x01```@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0CXW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0ClW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05`\x91\x90a#-V[B\x81\x11a\x0C\xAFW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07W\x90a#\xF4V[a\x0C\xB8\x83a\x13kV[`\0\x83\x81R`\x04` \x81\x81R`@\x80\x84 \x81Q`\xA0\x81\x01\x83R\x81T\x81R`\x01\x82\x01T\x93\x81\x01\x84\x90R`\x02\x82\x01T\x92\x81\x01\x92\x90\x92R`\x03\x81\x01T``\x83\x01R\x90\x92\x01T`\x80\x83\x01R\x90\x91\x90\x84\x10a\r\x1CW` \x82\x01Qa\r\x17\x90\x85a#tV[a\r,V[\x83\x82` \x01Qa\r,\x91\x90a#tV[\x90Pa\r8B\x84a#tV[a\rB\x90\x82a$\x1FV[``\x83\x01\x90\x81R\x84\x83R`@\x80\x84\x01\x85\x81R`\0\x88\x81R`\x04` \x81\x81R\x93\x90\x91 \x86Q\x80\x82U\x93\x87\x01Q`\x01\x82\x01\x81\x90U\x92Q`\x02\x82\x01\x81\x90U\x94Q`\x03\x82\x01U`\x80\x87\x01Q\x91\x01U\x7F$\xBBEgU#\x9Cf@Y\xB5\x19\xE4c\xB4\xE5\xEA\xEF\x05\x08~\x87\xB0\xDF\xED\xF6\x06\x11iR\xAC\xEE\x92\x81\x83\x11a\t\0W\x85Q` \x87\x01Qa\x08\xFB\x91\x90a#tV[`\0\x81\x81R`\x04` \x81\x81R`@\x80\x84 \x81Q`\xA0\x81\x01\x83R\x81T\x81R`\x01\x82\x01T\x93\x81\x01\x93\x90\x93R`\x02\x81\x01T\x91\x83\x01\x82\x90R`\x03\x81\x01T``\x84\x01R\x90\x92\x01T`\x80\x82\x01R\x90B\x10a\x05\xC3WQ\x92\x91PPV[a\x0E?`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[a\x0EH\x82a\r\xC6V[a\x0EQ\x83a\x05mV[a\x0EZ\x84a\nwV[`@\x84\x01R` \x83\x01R\x81R\x91\x90PV[a\x0E\x8F`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[P`\0\x90\x81R`\x02` \x81\x81R`@\x92\x83\x90 \x83Q``\x81\x01\x85R\x81T\x81R`\x01\x82\x01T\x92\x81\x01\x92\x90\x92R\x90\x91\x01T\x91\x81\x01\x91\x90\x91R\x90V[`\0\x82\x85\x10a\x0F\x19W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FtradingFunction: invalid x\0\0\0\0\0\0`D\x82\x01R`d\x01a\x07WV[`\0\x80g\r\xE0\xB6\xB3\xA7d\0\0a\x0F/\x88\x87a\x14\x03V[\x10a\x0FCW`\x01`\x01`\xFF\x1B\x03\x91Pa\x0FXV[a\x0FUa\x0FP\x88\x87a\x14\x03V[a\x14\x18V[\x91P[g\r\xE0\xB6\xB3\xA7d\0\0a\x0Fx\x87a\x0Fs\x87`\0\x01Q\x89a\x14\xBEV[a\x14\x03V[\x10a\x0F\x8BWP`\x01`\x01`\xFF\x1B\x03a\x0F\xA3V[a\x0F\xA0a\x0FP\x87a\x0Fs\x87`\0\x01Q\x89a\x14\xBEV[\x90P[`\0a\x0F\xB7\x85` \x01Q\x86`@\x01Qa\x14\xD3V[\x90P\x80a\x0F\xC4\x83\x85a$\xFBV[a\x0F\xCE\x91\x90a$\xFBV[\x98\x97PPPPPPPPV[`\0a\x06.\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x15\x01V[`\0a\x06.\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x15\x01V[`\0\x81\x81R`\x05` \x90\x81R`@\x91\x82\x90 \x82Q`\xA0\x81\x01\x84R\x81T\x81R`\x01\x82\x01T\x92\x81\x01\x92\x90\x92R`\x02\x81\x01T\x92\x82\x01\x92\x90\x92R`\x03\x82\x01T``\x82\x01R`\x04\x90\x91\x01T`\x80\x82\x01Ra\x10X\x82a\nwV[` \x80\x83\x01\x91\x82RB`\x80\x84\x01\x90\x81R`\0\x94\x85R`\x05\x90\x91R`@\x93\x84\x90 \x83Q\x81U\x91Q`\x01\x83\x01U\x92\x82\x01Q`\x02\x82\x01U``\x90\x91\x01Q`\x03\x82\x01U\x90Q`\x04\x90\x91\x01UV[`\0\x81\x81R`\x03` \x81\x81R`@\x92\x83\x90 \x83Q`\xA0\x81\x01\x85R\x81T\x81R`\x01\x82\x01T\x92\x81\x01\x92\x90\x92R`\x02\x81\x01T\x93\x82\x01\x93\x90\x93R\x90\x82\x01T``\x82\x01R`\x04\x90\x91\x01T`\x80\x82\x01Ra\x10\xF4\x82a\x05mV[` \x80\x83\x01\x91\x82RB`\x80\x84\x01\x90\x81R`\0\x94\x85R`\x03\x91\x82\x90R`@\x94\x85\x90 \x84Q\x81U\x92Q`\x01\x84\x01U\x93\x83\x01Q`\x02\x83\x01U``\x90\x92\x01Q\x91\x81\x01\x91\x90\x91U\x90Q`\x04\x90\x91\x01UV[`\0\x81\x81R`\x02` \x81\x81R`@\x80\x84 \x81Q``\x80\x82\x01\x84R\x82T\x82R`\x01\x83\x01T\x82\x86\x01R\x91\x90\x94\x01T\x84\x83\x01R\x81Q`\xA0\x81\x01\x83R\x85\x81R\x92\x83\x01\x85\x90R\x90\x82\x01\x84\x90R\x81\x01\x83\x90R`\x80\x81\x01\x92\x90\x92R\x90a\x11\xC7`@Q\x80`\xA0\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[a\x11\xF9`@Q\x80`\xA0\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[\x83` \x01Q\x83`\0\x01\x81\x81RPP\x83` \x01Q\x83` \x01\x81\x81RPPB\x83`@\x01\x81\x81RPPB\x83`\x80\x01\x81\x81RPP\x83`\0\x01Q\x82`\0\x01\x81\x81RPP\x83`\0\x01Q\x82` \x01\x81\x81RPPB\x82`@\x01\x81\x81RPPB\x82`\x80\x01\x81\x81RPP\x83`@\x01Q\x81`\0\x01\x81\x81RPP\x83`@\x01Q\x81` \x01\x81\x81RPPB\x81`@\x01\x81\x81RPPB\x81`\x80\x01\x81\x81RPP\x82`\x03`\0\x87\x81R` \x01\x90\x81R` \x01`\0 `\0\x82\x01Q\x81`\0\x01U` \x82\x01Q\x81`\x01\x01U`@\x82\x01Q\x81`\x02\x01U``\x82\x01Q\x81`\x03\x01U`\x80\x82\x01Q\x81`\x04\x01U\x90PP\x81`\x04`\0\x87\x81R` \x01\x90\x81R` \x01`\0 `\0\x82\x01Q\x81`\0\x01U` \x82\x01Q\x81`\x01\x01U`@\x82\x01Q\x81`\x02\x01U``\x82\x01Q\x81`\x03\x01U`\x80\x82\x01Q\x81`\x04\x01U\x90PP\x80`\x05`\0\x87\x81R` \x01\x90\x81R` \x01`\0 `\0\x82\x01Q\x81`\0\x01U` \x82\x01Q\x81`\x01\x01U`@\x82\x01Q\x81`\x02\x01U``\x82\x01Q\x81`\x03\x01U`\x80\x82\x01Q\x81`\x04\x01U\x90PPPPPPPV[`\0\x81\x81R`\x04` \x81\x81R`@\x92\x83\x90 \x83Q`\xA0\x81\x01\x85R\x81T\x81R`\x01\x82\x01T\x92\x81\x01\x92\x90\x92R`\x02\x81\x01T\x93\x82\x01\x93\x90\x93R`\x03\x83\x01T``\x82\x01R\x91\x01T`\x80\x82\x01Ra\x13\xBC\x82a\r\xC6V[` \x80\x83\x01\x91\x82RB`\x80\x84\x01\x90\x81R`\0\x94\x85R`\x04\x91\x82\x90R`@\x94\x85\x90 \x84Q\x81U\x92Q`\x01\x84\x01U\x93\x83\x01Q`\x02\x83\x01U``\x90\x92\x01Q`\x03\x82\x01U\x91Q\x91\x01UV[`\0a\x06.\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x15/V[`\0g\x06\xF0[Y\xD3\xB2\0\0\x82\x03a\x141WP`\0\x91\x90PV[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x12a\x14YW`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\x14zW`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x14\x87\x83`\x02a%#V[\x90P`\0a\x14\x94\x82a\x15NV[\x90P`\0a\x14\xAAg\x13\xA0K\xBD\xFD\xC9\xBE\x88\x83a\x17\xCCV[\x90Pa\x14\xB5\x81a#\xD8V[\x95\x94PPPPPV[`\0a\x06.\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x15/V[`\0\x80a\x14\xDF\x83a\x17\xE1V[a\x14\xED\x90c;\x9A\xCA\0a#\x87V[\x90Pa\x14\xF9\x84\x82a\x14\xBEV[\x94\x93PPPPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x15\x19W`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x15GW`\0\x80\xFD[\x04\x92\x91PPV[`\0\x80\x82\x12\x80a\x15eWPg\x1B\xC1mgN\xC8\0\0\x82\x13[\x15a\x15\x83W`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\x15\xA4W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81g\x1B\xC1mgN\xC8\0\0\x03a\x15\xCCW`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x15a\x15\xD7W\x91\x90PV[`\0g\r\xE0\xB6\xB3\xA7d\0\0\x83\x12a\x15\xFFWa\x15\xFA\x83g\x1B\xC1mgN\xC8\0\0a#\xB1V[a\x16\x01V[\x82[\x90P`\0a\x16\x17\x82g\x1B\xC1mgN\xC8\0\0a\x18\x85V[\x90P\x80`\0\x03a\x16:W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x16E\x82a\x18\x9AV[\x90P`\0c;\x9A\xCA\0a\x16pa\x16ka\x16eg\x1B\xC1mgN\xC8\0\0a#\xD8V[\x85a\x17\xCCV[a\x17\xE1V[a\x16z\x91\x90a%#V[\x90P`\0\x80a\x16\x91\x83g\x03\xC1f\\z\xAB \0a\x17\xCCV[a\x16\xA3\x90g \x05\xFEO&\x8E\xA0\0a$\xFBV[\x90P`\0a\x16\xD3\x84a\x16\xBC\x86f\x9F2u$b\xA0\0a\x17\xCCV[a\x16\xCE\x90g\r\xC5R\x7Fd, \0a$\xFBV[a\x17\xCCV[a\x16\xE5\x90g\r\xE0\xB6\xB3\xA7d\0\0a$\xFBV[\x90Pa\x17\tg\t\xD0(\xCCo _\xFF\x19\x85a\x16\xFF\x85\x85a\x18\x85V[a\x16\xCE\x91\x90a#\xB1V[\x92PPP`\0[`\x02\x81\x10\x15a\x17\xA4W`\0\x86a\x17%\x84a\x1AuV[a\x17/\x91\x90a#\xB1V[\x90P`\0a\x17=\x84\x85a\x17\xCCV[a\x17F\x90a#\xD8V[\x90P`\0a\x17S\x82a\x1CYV[\x90P`\0a\x17a\x86\x85a\x17\xCCV[a\x17sg\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x84a\x17\xCCV[a\x17}\x91\x90a#\xB1V[\x90Pa\x17\x89\x84\x82a\x18\x85V[a\x17\x93\x90\x87a$\xFBV[\x95P\x84`\x01\x01\x94PPPPPa\x17\x10V[g\r\xE0\xB6\xB3\xA7d\0\0\x88\x12a\x17\xC1Wa\x17\xBC\x82a#\xD8V[a\x0F\xCEV[P\x96\x95PPPPPPV[`\0a\x06.\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x1E\x02V[`\xB5\x81`\x01`\x88\x1B\x81\x10a\x17\xFAW`@\x91\x90\x91\x1B\x90`\x80\x1C[i\x01\0\0\0\0\0\0\0\0\0\x81\x10a\x18\x16W` \x91\x90\x91\x1B\x90`@\x1C[e\x01\0\0\0\0\0\x81\x10a\x18.W`\x10\x91\x90\x91\x1B\x90` \x1C[c\x01\0\0\0\x81\x10a\x18DW`\x08\x91\x90\x91\x1B\x90`\x10\x1C[b\x01\0\0\x01\x02`\x12\x1C\x80\x82\x04\x01`\x01\x90\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x90\x81\x90\x04\x81\x11\x90\x03\x90V[`\0a\x06.\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x1E\x02V[`\0\x80\x82\x13a\x18\xD7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x07WV[`\0``a\x18\xE4\x84a\x1E!V[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[`\0\x81`\0\x03a\x1A\x8EWPg\r\xE0\xB6\xB3\xA7d\0\0\x91\x90PV[gV\x98\xEE\xF0fp\0\0\x82\x12a\x1A\xA5WP`\0\x91\x90PV[a\x1A\xB6gV\x98\xEE\xF0fp\0\0a#\xD8V[\x82\x13a\x1A\xCBWPg\x1B\xC1mgN\xC8\0\0\x91\x90PV[`\0a\x1A\xD6\x83a\x1E\xC9V[\x90P`\0a\x1B\x0Fg\r\xE0\xB6\xB3\xA7d\0\0a\x1A\xF8\x84g\x1B\xC1mgN\xC8\0\0a\x14\x03V[a\x1B\n\x90g\r\xE0\xB6\xB3\xA7d\0\0a$\xFBV[a\x18\x85V[\x90P`\0\x80\x82a\x1Bk\x81a\x1BX\x81a\x1BF\x81a\x1B3\x81g\x02_\x0F\xE1\x05\xA3\x14\0a\x17\xCCV[a\x16\xCE\x90g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19a$\xFBV[a\x16\xCE\x90g\x14\xA8EL\x19\xE1\xAC\0a$\xFBV[a\x16\xCE\x90g\x0F\xC1\x0E\x01W\x82w\xFF\x19a$\xFBV[a\x1B}\x90g\x03\xDE\xBD\x08;\x8C|\0a$\xFBV[\x91P\x83\x90Pa\x1B\xE5\x81a\x1B\xD3\x81a\x1B\xC1\x81a\x1B\xAF\x81a\x1B\x9C\x81\x8Ba\x17\xCCV[a\x16\xCE\x90g\x02\x95\xD4\0\xEA2W\xFF\x19a$\xFBV[a\x16\xCE\x90g\x01W\xD8\xB2\xEC\xC7\x08\0a$\xFBV[a\x16\xCE\x90g\x051\n\xA7\xD5!0\0a$\xFBV[a\x16\xCE\x90g\r\xE0\xCC=\x15a\0\0a$\xFBV[\x91P\x81g\x11\x90\0\xAB\x10\x0F\xFC\0a\x1B\xFB\x87\x88a\x17\xCCV[a\x1C\x07\x90`\0\x19a%#V[a\x1C\x11\x91\x90a#\xB1V[a\x1C\x1B\x91\x90a$\xFBV[\x92PP`\0a\x1C)\x83a\x1CYV[\x90P`\0a\x1C7\x85\x83a\x17\xCCV[\x90P`\0\x88\x12a\x1CGW\x80a\x0F\xCEV[a\x0F\xCE\x81g\x1B\xC1mgN\xC8\0\0a#\xB1V[`\0h\x02H\xCE6\xA7\x0C\xB2k>\x19\x82\x13a\x1CtWP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a\x1C\xBBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x01a\x07WV[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05`\x01`_\x1B\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1Dm\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1Dn\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x05\x85\x14\x17\x16a\x1E\x1AW`\0\x80\xFD[\x05\x92\x91PPV[`\0\x80\x82\x11a\x1E^W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x07WV[P`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x82\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[`\0`\x01`\xFF\x1B\x82\x03a\x1E\xEFW`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x12\x15a\x1F\0WP\x19`\x01\x01\x90V[P\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x81\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a 6Wa 6a\x1F\xFDV[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a eWa ea\x1F\xFDV[`@R\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a \x83Wa \x83a\x1F\x04V[\x825\x91P` \x80\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a \xA6Wa \xA6a\x1FTV[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a \xBDWa \xBDa\x1F\xA4V[\x815\x81\x81\x11\x15a \xCFWa \xCFa\x1F\xFDV[a \xE1`\x1F\x82\x01`\x1F\x19\x16\x85\x01a <V[\x91P\x80\x82R\x87\x84\x82\x85\x01\x01\x11\x15a!GW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x85\x90R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R`\x84\x81\xFD[\x80\x84\x84\x01\x85\x84\x017`\0\x84\x82\x84\x01\x01RP\x80\x93PPPP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a!zWa!za\x1F\x04V[P5\x91\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a!\x99Wa!\x99a\x1F\x04V[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[`\0\x80`\0`@\x84\x86\x03\x12\x15a!\xC8Wa!\xC8a\x1F\x04V[\x835\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a!\xEAWa!\xEAa\x1FTV[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\"\x01Wa\"\x01a\x1F\xA4V[\x815\x81\x81\x11\x15a\"dW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x0ENL/$\r\x8C\xAD\xCC\xEE\x8D`\xAB\x1B`d\x82\x01R`\x84\x81\xFD[\x87` \x82\x85\x01\x01\x11\x15a\"\xCCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rjrray stride`\xA8\x1B`d\x82\x01R\x91P`\x84\x82\xFD[` \x83\x01\x94P\x80\x93PPPP\x92P\x92P\x92V[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a#\x0CW\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a\"\xF0V[P`\0`@\x82\x86\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x92PPP\x92\x91PPV[`\0\x80`\0``\x84\x86\x03\x12\x15a#EWa#Ea\x1F\x04V[\x83Q\x92P` \x84\x01Q\x91P`@\x84\x01Q\x90P\x92P\x92P\x92V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x05:Wa\x05:a#^V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x05:Wa\x05:a#^V[\x80\x82\x01\x80\x82\x11\x15a\x05:Wa\x05:a#^V[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a#\xD1Wa#\xD1a#^V[P\x92\x91PPV[`\0`\x01`\xFF\x1B\x82\x01a#\xEDWa#\xEDa#^V[P`\0\x03\x90V[` \x80\x82R`\x11\x90\x82\x01Rp\x15\\\x19\x18]\x19H\x19[\x99\x08\x1C\x18\\\xDC\xD9Y`z\x1B`@\x82\x01R``\x01\x90V[`\0\x82a$<WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[`\0\x80`\0\x80\x84\x86\x03`\xC0\x81\x12\x15a$[Wa$[a\x1F\x04V[\x855\x94P` \x86\x015\x93P`@\x86\x015\x92P```_\x19\x82\x01\x12\x15a$\xCBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FABI decoding: struct data too sh`D\x82\x01Rb\x1B\xDC\x9D`\xEA\x1B`d\x82\x01R`\x84\x81\xFD[Pa$\xD4a \x13V[``\x86\x015\x81R`\x80\x86\x015` \x82\x01R`\xA0\x90\x95\x015`@\x86\x01RP\x91\x94\x90\x93P\x90\x91\x90V[\x80\x82\x01\x82\x81\x12`\0\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a%\x1BWa%\x1Ba#^V[PP\x92\x91PPV[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15a%?Wa%?a#^V[\x81\x81\x05\x83\x14\x82\x15\x17a\x05:Wa\x05:a#^V";
    /// The bytecode of the contract.
    pub static LOGNORMAL_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x01\x83W`\x005`\xE0\x1C\x80c\x8C3\x88\xDC\x11a\x01\x05W\x80c\xCE\x15;\xF4\x11a\0\xC9W\x80c\xCE\x15;\xF4\x14a\x04TW\x80c\xD7\x10\x955\x14a\x04gW\x80c\xE8\0\xBDD\x14a\x04zW\x80c\xECM\xC1[\x14a\x04\x8DW\x80c\xF2\xF4\xEB&\x14a\x04\xC2W\x80c\xFC\x13\x05\xE5\x14a\x04\xEDWa\x01\x83V[\x80c\x8C3\x88\xDC\x14a\x03\x93W\x80c\x8C\xC1\xC1\xAD\x14a\x03\xD1W\x80c\x8E-\xD4\0\x14a\x03\xE4W\x80c\x9F\x83\x13{\x14a\x04!W\x80c\xA4\xD4z^\x14a\x044Wa\x01\x83V[\x80c8}\xD9\xE9\x11a\x01LW\x80c8}\xD9\xE9\x14a\x02\xF6W\x80c?C\xBA\xAC\x14a\x03$W\x80cT\xCF*\xEB\x14a\x039W\x80crQ0\x8C\x14a\x03BW\x80cy$\x17\xB1\x14a\x03UWa\x01\x83V[\x80b.RK\x14a\x01\xE8W\x80c\x08\xA4\xF0r\x14a\x02\x0EW\x80c\x1336\xAA\x14a\x02<W\x80c\x16\x98\xAAI\x14a\x02OW\x80c2\x14\x89\x0F\x14a\x02\xB4W[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x01\xFBa\x01\xF66`\x04a mV[a\x05\0V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x02!a\x02\x1C6`\x04a!eV[a\x05@V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x02\x05V[a\x01\xFBa\x02J6`\x04a!eV[a\x05mV[a\x02\x8Ca\x02]6`\x04a!eV[`\x05` R`\0\x90\x81R`@\x90 \x80T`\x01\x82\x01T`\x02\x83\x01T`\x03\x84\x01T`\x04\x90\x94\x01T\x92\x93\x91\x92\x90\x91\x90\x85V[`@\x80Q\x95\x86R` \x86\x01\x94\x90\x94R\x92\x84\x01\x91\x90\x91R``\x83\x01R`\x80\x82\x01R`\xA0\x01a\x02\x05V[a\x02\xC7a\x02\xC26`\x04a mV[a\x065V[`@\x80Q\x96\x15\x15\x87R` \x87\x01\x95\x90\x95R\x93\x85\x01\x92\x90\x92R``\x84\x01R`\x80\x83\x01R`\xA0\x82\x01R`\xC0\x01a\x02\x05V[a\x02!a\x03\x046`\x04a!eV[`\x02` \x81\x90R`\0\x91\x82R`@\x90\x91 \x80T`\x01\x82\x01T\x91\x90\x92\x01T\x83V[a\x037a\x0326`\x04a!\x81V[a\x07\xC4V[\0[a\x01\xFB`\x01T\x81V[a\x037a\x03P6`\x04a!\x81V[a\t;V[a\x02\x8Ca\x03c6`\x04a!eV[`\x04` \x81\x90R`\0\x91\x82R`@\x90\x91 \x80T`\x01\x82\x01T`\x02\x83\x01T`\x03\x84\x01T\x93\x90\x94\x01T\x91\x93\x90\x92\x90\x91\x85V[a\x02\x8Ca\x03\xA16`\x04a!eV[`\x03` \x81\x90R`\0\x91\x82R`@\x90\x91 \x80T`\x01\x82\x01T`\x02\x83\x01T\x93\x83\x01T`\x04\x90\x93\x01T\x91\x93\x90\x92\x90\x91\x85V[a\x01\xFBa\x03\xDF6`\x04a!eV[a\nwV[a\x03\xF7a\x03\xF26`\x04a!\xB0V[a\n\xCBV[`@\x80Q\x95\x15\x15\x86R` \x86\x01\x94\x90\x94R\x92\x84\x01\x91\x90\x91R``\x83\x01R`\x80\x82\x01R`\xA0\x01a\x02\x05V[a\x03\xF7a\x04/6`\x04a!\xB0V[a\x0B\x1AV[a\x04Ga\x04B6`\x04a!eV[a\x0BrV[`@Qa\x02\x05\x91\x90a\"\xDFV[a\x02!a\x04b6`\x04a!eV[a\x0B\xBFV[a\x037a\x04u6`\x04a!\x81V[a\x0C\x90V[a\x01\xFBa\x04\x886`\x04a!eV[a\r\xC6V[a\x04\xA0a\x04\x9B6`\x04a!eV[a\x0E\x1BV[`@\x80Q\x82Q\x81R` \x80\x84\x01Q\x90\x82\x01R\x91\x81\x01Q\x90\x82\x01R``\x01a\x02\x05V[`\0Ta\x04\xD5\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\x05V[a\x04\xA0a\x04\xFB6`\x04a!eV[a\x0EkV[`\0\x80`\0\x80\x84\x80` \x01\x90Q\x81\x01\x90a\x05\x1A\x91\x90a#-V[\x92P\x92P\x92Pa\x054\x83\x83\x83a\x05/\x8Aa\x0E\x1BV[a\x0E\xC8V[\x93PPPP[\x92\x91PPV[`\0\x80`\0a\x05N\x84a\r\xC6V[a\x05W\x85a\x05mV[a\x05`\x86a\nwV[\x92P\x92P\x92P\x91\x93\x90\x92PV[`\0\x81\x81R`\x03` \x81\x81R`@\x80\x84 \x81Q`\xA0\x81\x01\x83R\x81T\x81R`\x01\x82\x01T\x93\x81\x01\x93\x90\x93R`\x02\x81\x01T\x91\x83\x01\x82\x90R\x92\x83\x01T``\x83\x01R`\x04\x90\x92\x01T`\x80\x82\x01R\x90B\x10a\x05\xC3WQ\x92\x91PPV[\x80Q` \x82\x01Q\x11a\x06\x01W``\x81\x01Q`\x80\x82\x01Qa\x05\xE3\x90Ba#tV[a\x05\xED\x91\x90a#\x87V[\x81` \x01Qa\x05\xFC\x91\x90a#\x9EV[a\x06.V[``\x81\x01Q`\x80\x82\x01Qa\x06\x15\x90Ba#tV[a\x06\x1F\x91\x90a#\x87V[\x81` \x01Qa\x06.\x91\x90a#tV[\x93\x92PPPV[`\0\x80`\0\x80`\0\x80`\0\x80`\0a\x06L\x8Ba\x0B\xBFV[\x92P\x92P\x92P\x89\x80` \x01\x90Q\x81\x01\x90a\x06f\x91\x90a#-V[\x91\x97P\x95P\x93P`\0\x80\x80\x85\x89\x11\x15a\x06\xC1Wa\x06\x83\x86\x8Aa#tV[\x91Pa\x06\x9A`\x01T\x83a\x0F\xDA\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa\x06\xB0\x86a\x06\xAA\x83\x87a\x0F\xDAV[\x90a\x0F\xEFV[a\x06\xBA\x90\x84a#\x9EV[\x92Pa\x07`V[\x84\x88\x11\x15a\x06\xFAWa\x06\xD3\x85\x89a#tV[\x91Pa\x06\xEA`\x01T\x83a\x0F\xDA\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa\x06\xB0\x85a\x06\xAA\x83\x87a\x0F\xDAV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`0`$\x82\x01R\x7Finvalid swap: inputs x and y hav`D\x82\x01Roe the same sign!`\x80\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[a\x07j\x84\x88a#\xB1V[\x99P`\0a\x07w\x8Fa\x0E\x1BV[\x90Pa\x07\x85\x8A\x8A\x8A\x84a\x0E\xC8V[\x9BP`\0\x8Ca\x07\x94`\x1Ea#\xD8V[\x12\x80\x15a\x07\xA1WP`\x1E\x8D\x12[\x90P\x80\x80\x15a\x07\xB0WP\x84\x8C\x12\x15[\x9DPPPPPPPPP\x92\x95P\x92\x95P\x92\x95V[B\x81\x11a\x07\xE3W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07W\x90a#\xF4V[a\x07\xEC\x83a\x10\x04V[`\0\x83\x81R`\x05` \x90\x81R`@\x80\x83 \x81Q`\xA0\x81\x01\x83R\x81T\x81R`\x01\x82\x01T\x93\x81\x01\x84\x90R`\x02\x82\x01T\x92\x81\x01\x92\x90\x92R`\x03\x81\x01T``\x83\x01R`\x04\x01T`\x80\x82\x01R\x91\x90\x84\x10a\x08OW` \x82\x01Qa\x08J\x90\x85a#tV[a\x08_V[\x83\x82` \x01Qa\x08_\x91\x90a#tV[\x90Pa\x08kB\x84a#tV[a\x08u\x90\x82a$\x1FV[``\x83\x01\x90\x81R\x84\x83R`@\x80\x84\x01\x85\x81R`\0\x88\x81R`\x05` \x90\x81R\x92\x90 \x85Q\x80\x82U\x92\x86\x01Q`\x01\x82\x01\x81\x90U\x91Q`\x02\x82\x01\x81\x90U\x93Q`\x03\x82\x01U`\x80\x86\x01Q`\x04\x90\x91\x01U\x7F\x1EK Gg&%\xF4\xDF!\xD4|\xA7\xA0\x0C \x12\xCC\xF7x\xF2w\xBF2(\xDD\xF4Y=o\x84\xAA\x92\x81\x83\x11a\t\0W\x85Q` \x87\x01Qa\x08\xFB\x91\x90a#tV[a\t\x11V[` \x86\x01Q\x86Qa\t\x11\x91\x90a#tV[`@\x80Q\x94\x85R` \x85\x01\x93\x90\x93R\x91\x83\x01R``\x82\x01R`\x80\x01`@Q\x80\x91\x03\x90\xA1PPPPPV[B\x81\x11a\tZW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07W\x90a#\xF4V[a\tc\x83a\x10\xA1V[`\0\x83\x81R`\x03` \x81\x81R`@\x80\x84 \x81Q`\xA0\x81\x01\x83R\x81T\x81R`\x01\x82\x01T\x93\x81\x01\x84\x90R`\x02\x82\x01T\x92\x81\x01\x92\x90\x92R\x92\x83\x01T``\x82\x01R`\x04\x90\x92\x01T`\x80\x83\x01R\x90\x91\x90\x84\x10a\t\xC8W` \x82\x01Qa\t\xC3\x90\x85a#tV[a\t\xD8V[\x83\x82` \x01Qa\t\xD8\x91\x90a#tV[\x90Pa\t\xE4B\x84a#tV[a\t\xEE\x90\x82a$\x1FV[``\x83\x01\x90\x81R\x84\x83R`@\x80\x84\x01\x85\x81R`\0\x88\x81R`\x03` \x81\x81R\x93\x90\x91 \x86Q\x80\x82U\x93\x87\x01Q`\x01\x82\x01\x81\x90U\x92Q`\x02\x82\x01\x81\x90U\x94Q\x91\x81\x01\x91\x90\x91U`\x80\x86\x01Q`\x04\x90\x91\x01U\x7F\xB9hO\xE86\x87_\xAF\x05m\xB2]\xE9\x85S\x859`\xB3&\x08\x8A\x10!\xAF\xA7[\xA1`\x12d\xF5\x92\x81\x83\x11a\t\0W\x85Q` \x87\x01Qa\x08\xFB\x91\x90a#tV[`\0\x81\x81R`\x05` \x90\x81R`@\x80\x83 \x81Q`\xA0\x81\x01\x83R\x81T\x81R`\x01\x82\x01T\x93\x81\x01\x93\x90\x93R`\x02\x81\x01T\x91\x83\x01\x82\x90R`\x03\x81\x01T``\x84\x01R`\x04\x01T`\x80\x83\x01RB\x10a\x05\xC3WQ\x92\x91PPV[`\0\x80\x80\x80\x80a\n\xDD\x86\x88\x01\x88a!\x81V[\x91\x94P\x92P\x90Pa\n\xF3\x83\x83\x83a\x05/\x8Ca\x0E\x1BV[\x93P\x83a\x0B\0`\x1Ea#\xD8V[\x12\x80\x15a\x0B\rWP`\x1E\x84\x12[\x94P\x93\x97\x92\x96P\x93P\x93PV[`\0\x80\x80\x80\x80a\x0B,\x86\x88\x01\x88a$AV[`\0\x8C\x81R`\x02` \x81\x81R`@\x92\x83\x90 \x84Q\x81U\x90\x84\x01Q`\x01\x82\x01U\x92\x90\x91\x01Q\x91\x01U\x91\x94P\x92P\x90Pa\x0Bc\x88a\x11@V[a\n\xF3\x83\x83\x83a\x05/\x8Ca\x0E\x1BV[``a\x0B}\x82a\r\xC6V[a\x0B\x86\x83a\x05mV[a\x0B\x8F\x84a\nwV[`@\x80Q` \x81\x01\x94\x90\x94R\x83\x01\x91\x90\x91R``\x82\x01R`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x91\x90PV[`\0\x80T`@Qc3\x85N\xFD`\xE2\x1B\x81R`\x04\x81\x01\x84\x90R\x82\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xCE\x15;\xF4\x90`$\x01```@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0CXW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0ClW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05`\x91\x90a#-V[B\x81\x11a\x0C\xAFW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07W\x90a#\xF4V[a\x0C\xB8\x83a\x13kV[`\0\x83\x81R`\x04` \x81\x81R`@\x80\x84 \x81Q`\xA0\x81\x01\x83R\x81T\x81R`\x01\x82\x01T\x93\x81\x01\x84\x90R`\x02\x82\x01T\x92\x81\x01\x92\x90\x92R`\x03\x81\x01T``\x83\x01R\x90\x92\x01T`\x80\x83\x01R\x90\x91\x90\x84\x10a\r\x1CW` \x82\x01Qa\r\x17\x90\x85a#tV[a\r,V[\x83\x82` \x01Qa\r,\x91\x90a#tV[\x90Pa\r8B\x84a#tV[a\rB\x90\x82a$\x1FV[``\x83\x01\x90\x81R\x84\x83R`@\x80\x84\x01\x85\x81R`\0\x88\x81R`\x04` \x81\x81R\x93\x90\x91 \x86Q\x80\x82U\x93\x87\x01Q`\x01\x82\x01\x81\x90U\x92Q`\x02\x82\x01\x81\x90U\x94Q`\x03\x82\x01U`\x80\x87\x01Q\x91\x01U\x7F$\xBBEgU#\x9Cf@Y\xB5\x19\xE4c\xB4\xE5\xEA\xEF\x05\x08~\x87\xB0\xDF\xED\xF6\x06\x11iR\xAC\xEE\x92\x81\x83\x11a\t\0W\x85Q` \x87\x01Qa\x08\xFB\x91\x90a#tV[`\0\x81\x81R`\x04` \x81\x81R`@\x80\x84 \x81Q`\xA0\x81\x01\x83R\x81T\x81R`\x01\x82\x01T\x93\x81\x01\x93\x90\x93R`\x02\x81\x01T\x91\x83\x01\x82\x90R`\x03\x81\x01T``\x84\x01R\x90\x92\x01T`\x80\x82\x01R\x90B\x10a\x05\xC3WQ\x92\x91PPV[a\x0E?`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[a\x0EH\x82a\r\xC6V[a\x0EQ\x83a\x05mV[a\x0EZ\x84a\nwV[`@\x84\x01R` \x83\x01R\x81R\x91\x90PV[a\x0E\x8F`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[P`\0\x90\x81R`\x02` \x81\x81R`@\x92\x83\x90 \x83Q``\x81\x01\x85R\x81T\x81R`\x01\x82\x01T\x92\x81\x01\x92\x90\x92R\x90\x91\x01T\x91\x81\x01\x91\x90\x91R\x90V[`\0\x82\x85\x10a\x0F\x19W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FtradingFunction: invalid x\0\0\0\0\0\0`D\x82\x01R`d\x01a\x07WV[`\0\x80g\r\xE0\xB6\xB3\xA7d\0\0a\x0F/\x88\x87a\x14\x03V[\x10a\x0FCW`\x01`\x01`\xFF\x1B\x03\x91Pa\x0FXV[a\x0FUa\x0FP\x88\x87a\x14\x03V[a\x14\x18V[\x91P[g\r\xE0\xB6\xB3\xA7d\0\0a\x0Fx\x87a\x0Fs\x87`\0\x01Q\x89a\x14\xBEV[a\x14\x03V[\x10a\x0F\x8BWP`\x01`\x01`\xFF\x1B\x03a\x0F\xA3V[a\x0F\xA0a\x0FP\x87a\x0Fs\x87`\0\x01Q\x89a\x14\xBEV[\x90P[`\0a\x0F\xB7\x85` \x01Q\x86`@\x01Qa\x14\xD3V[\x90P\x80a\x0F\xC4\x83\x85a$\xFBV[a\x0F\xCE\x91\x90a$\xFBV[\x98\x97PPPPPPPPV[`\0a\x06.\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x15\x01V[`\0a\x06.\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x15\x01V[`\0\x81\x81R`\x05` \x90\x81R`@\x91\x82\x90 \x82Q`\xA0\x81\x01\x84R\x81T\x81R`\x01\x82\x01T\x92\x81\x01\x92\x90\x92R`\x02\x81\x01T\x92\x82\x01\x92\x90\x92R`\x03\x82\x01T``\x82\x01R`\x04\x90\x91\x01T`\x80\x82\x01Ra\x10X\x82a\nwV[` \x80\x83\x01\x91\x82RB`\x80\x84\x01\x90\x81R`\0\x94\x85R`\x05\x90\x91R`@\x93\x84\x90 \x83Q\x81U\x91Q`\x01\x83\x01U\x92\x82\x01Q`\x02\x82\x01U``\x90\x91\x01Q`\x03\x82\x01U\x90Q`\x04\x90\x91\x01UV[`\0\x81\x81R`\x03` \x81\x81R`@\x92\x83\x90 \x83Q`\xA0\x81\x01\x85R\x81T\x81R`\x01\x82\x01T\x92\x81\x01\x92\x90\x92R`\x02\x81\x01T\x93\x82\x01\x93\x90\x93R\x90\x82\x01T``\x82\x01R`\x04\x90\x91\x01T`\x80\x82\x01Ra\x10\xF4\x82a\x05mV[` \x80\x83\x01\x91\x82RB`\x80\x84\x01\x90\x81R`\0\x94\x85R`\x03\x91\x82\x90R`@\x94\x85\x90 \x84Q\x81U\x92Q`\x01\x84\x01U\x93\x83\x01Q`\x02\x83\x01U``\x90\x92\x01Q\x91\x81\x01\x91\x90\x91U\x90Q`\x04\x90\x91\x01UV[`\0\x81\x81R`\x02` \x81\x81R`@\x80\x84 \x81Q``\x80\x82\x01\x84R\x82T\x82R`\x01\x83\x01T\x82\x86\x01R\x91\x90\x94\x01T\x84\x83\x01R\x81Q`\xA0\x81\x01\x83R\x85\x81R\x92\x83\x01\x85\x90R\x90\x82\x01\x84\x90R\x81\x01\x83\x90R`\x80\x81\x01\x92\x90\x92R\x90a\x11\xC7`@Q\x80`\xA0\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[a\x11\xF9`@Q\x80`\xA0\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[\x83` \x01Q\x83`\0\x01\x81\x81RPP\x83` \x01Q\x83` \x01\x81\x81RPPB\x83`@\x01\x81\x81RPPB\x83`\x80\x01\x81\x81RPP\x83`\0\x01Q\x82`\0\x01\x81\x81RPP\x83`\0\x01Q\x82` \x01\x81\x81RPPB\x82`@\x01\x81\x81RPPB\x82`\x80\x01\x81\x81RPP\x83`@\x01Q\x81`\0\x01\x81\x81RPP\x83`@\x01Q\x81` \x01\x81\x81RPPB\x81`@\x01\x81\x81RPPB\x81`\x80\x01\x81\x81RPP\x82`\x03`\0\x87\x81R` \x01\x90\x81R` \x01`\0 `\0\x82\x01Q\x81`\0\x01U` \x82\x01Q\x81`\x01\x01U`@\x82\x01Q\x81`\x02\x01U``\x82\x01Q\x81`\x03\x01U`\x80\x82\x01Q\x81`\x04\x01U\x90PP\x81`\x04`\0\x87\x81R` \x01\x90\x81R` \x01`\0 `\0\x82\x01Q\x81`\0\x01U` \x82\x01Q\x81`\x01\x01U`@\x82\x01Q\x81`\x02\x01U``\x82\x01Q\x81`\x03\x01U`\x80\x82\x01Q\x81`\x04\x01U\x90PP\x80`\x05`\0\x87\x81R` \x01\x90\x81R` \x01`\0 `\0\x82\x01Q\x81`\0\x01U` \x82\x01Q\x81`\x01\x01U`@\x82\x01Q\x81`\x02\x01U``\x82\x01Q\x81`\x03\x01U`\x80\x82\x01Q\x81`\x04\x01U\x90PPPPPPPV[`\0\x81\x81R`\x04` \x81\x81R`@\x92\x83\x90 \x83Q`\xA0\x81\x01\x85R\x81T\x81R`\x01\x82\x01T\x92\x81\x01\x92\x90\x92R`\x02\x81\x01T\x93\x82\x01\x93\x90\x93R`\x03\x83\x01T``\x82\x01R\x91\x01T`\x80\x82\x01Ra\x13\xBC\x82a\r\xC6V[` \x80\x83\x01\x91\x82RB`\x80\x84\x01\x90\x81R`\0\x94\x85R`\x04\x91\x82\x90R`@\x94\x85\x90 \x84Q\x81U\x92Q`\x01\x84\x01U\x93\x83\x01Q`\x02\x83\x01U``\x90\x92\x01Q`\x03\x82\x01U\x91Q\x91\x01UV[`\0a\x06.\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x15/V[`\0g\x06\xF0[Y\xD3\xB2\0\0\x82\x03a\x141WP`\0\x91\x90PV[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x12a\x14YW`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\x14zW`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x14\x87\x83`\x02a%#V[\x90P`\0a\x14\x94\x82a\x15NV[\x90P`\0a\x14\xAAg\x13\xA0K\xBD\xFD\xC9\xBE\x88\x83a\x17\xCCV[\x90Pa\x14\xB5\x81a#\xD8V[\x95\x94PPPPPV[`\0a\x06.\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x15/V[`\0\x80a\x14\xDF\x83a\x17\xE1V[a\x14\xED\x90c;\x9A\xCA\0a#\x87V[\x90Pa\x14\xF9\x84\x82a\x14\xBEV[\x94\x93PPPPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x15\x19W`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x15GW`\0\x80\xFD[\x04\x92\x91PPV[`\0\x80\x82\x12\x80a\x15eWPg\x1B\xC1mgN\xC8\0\0\x82\x13[\x15a\x15\x83W`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\x15\xA4W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81g\x1B\xC1mgN\xC8\0\0\x03a\x15\xCCW`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x15a\x15\xD7W\x91\x90PV[`\0g\r\xE0\xB6\xB3\xA7d\0\0\x83\x12a\x15\xFFWa\x15\xFA\x83g\x1B\xC1mgN\xC8\0\0a#\xB1V[a\x16\x01V[\x82[\x90P`\0a\x16\x17\x82g\x1B\xC1mgN\xC8\0\0a\x18\x85V[\x90P\x80`\0\x03a\x16:W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x16E\x82a\x18\x9AV[\x90P`\0c;\x9A\xCA\0a\x16pa\x16ka\x16eg\x1B\xC1mgN\xC8\0\0a#\xD8V[\x85a\x17\xCCV[a\x17\xE1V[a\x16z\x91\x90a%#V[\x90P`\0\x80a\x16\x91\x83g\x03\xC1f\\z\xAB \0a\x17\xCCV[a\x16\xA3\x90g \x05\xFEO&\x8E\xA0\0a$\xFBV[\x90P`\0a\x16\xD3\x84a\x16\xBC\x86f\x9F2u$b\xA0\0a\x17\xCCV[a\x16\xCE\x90g\r\xC5R\x7Fd, \0a$\xFBV[a\x17\xCCV[a\x16\xE5\x90g\r\xE0\xB6\xB3\xA7d\0\0a$\xFBV[\x90Pa\x17\tg\t\xD0(\xCCo _\xFF\x19\x85a\x16\xFF\x85\x85a\x18\x85V[a\x16\xCE\x91\x90a#\xB1V[\x92PPP`\0[`\x02\x81\x10\x15a\x17\xA4W`\0\x86a\x17%\x84a\x1AuV[a\x17/\x91\x90a#\xB1V[\x90P`\0a\x17=\x84\x85a\x17\xCCV[a\x17F\x90a#\xD8V[\x90P`\0a\x17S\x82a\x1CYV[\x90P`\0a\x17a\x86\x85a\x17\xCCV[a\x17sg\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x84a\x17\xCCV[a\x17}\x91\x90a#\xB1V[\x90Pa\x17\x89\x84\x82a\x18\x85V[a\x17\x93\x90\x87a$\xFBV[\x95P\x84`\x01\x01\x94PPPPPa\x17\x10V[g\r\xE0\xB6\xB3\xA7d\0\0\x88\x12a\x17\xC1Wa\x17\xBC\x82a#\xD8V[a\x0F\xCEV[P\x96\x95PPPPPPV[`\0a\x06.\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x1E\x02V[`\xB5\x81`\x01`\x88\x1B\x81\x10a\x17\xFAW`@\x91\x90\x91\x1B\x90`\x80\x1C[i\x01\0\0\0\0\0\0\0\0\0\x81\x10a\x18\x16W` \x91\x90\x91\x1B\x90`@\x1C[e\x01\0\0\0\0\0\x81\x10a\x18.W`\x10\x91\x90\x91\x1B\x90` \x1C[c\x01\0\0\0\x81\x10a\x18DW`\x08\x91\x90\x91\x1B\x90`\x10\x1C[b\x01\0\0\x01\x02`\x12\x1C\x80\x82\x04\x01`\x01\x90\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x90\x81\x90\x04\x81\x11\x90\x03\x90V[`\0a\x06.\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x1E\x02V[`\0\x80\x82\x13a\x18\xD7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x07WV[`\0``a\x18\xE4\x84a\x1E!V[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[`\0\x81`\0\x03a\x1A\x8EWPg\r\xE0\xB6\xB3\xA7d\0\0\x91\x90PV[gV\x98\xEE\xF0fp\0\0\x82\x12a\x1A\xA5WP`\0\x91\x90PV[a\x1A\xB6gV\x98\xEE\xF0fp\0\0a#\xD8V[\x82\x13a\x1A\xCBWPg\x1B\xC1mgN\xC8\0\0\x91\x90PV[`\0a\x1A\xD6\x83a\x1E\xC9V[\x90P`\0a\x1B\x0Fg\r\xE0\xB6\xB3\xA7d\0\0a\x1A\xF8\x84g\x1B\xC1mgN\xC8\0\0a\x14\x03V[a\x1B\n\x90g\r\xE0\xB6\xB3\xA7d\0\0a$\xFBV[a\x18\x85V[\x90P`\0\x80\x82a\x1Bk\x81a\x1BX\x81a\x1BF\x81a\x1B3\x81g\x02_\x0F\xE1\x05\xA3\x14\0a\x17\xCCV[a\x16\xCE\x90g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19a$\xFBV[a\x16\xCE\x90g\x14\xA8EL\x19\xE1\xAC\0a$\xFBV[a\x16\xCE\x90g\x0F\xC1\x0E\x01W\x82w\xFF\x19a$\xFBV[a\x1B}\x90g\x03\xDE\xBD\x08;\x8C|\0a$\xFBV[\x91P\x83\x90Pa\x1B\xE5\x81a\x1B\xD3\x81a\x1B\xC1\x81a\x1B\xAF\x81a\x1B\x9C\x81\x8Ba\x17\xCCV[a\x16\xCE\x90g\x02\x95\xD4\0\xEA2W\xFF\x19a$\xFBV[a\x16\xCE\x90g\x01W\xD8\xB2\xEC\xC7\x08\0a$\xFBV[a\x16\xCE\x90g\x051\n\xA7\xD5!0\0a$\xFBV[a\x16\xCE\x90g\r\xE0\xCC=\x15a\0\0a$\xFBV[\x91P\x81g\x11\x90\0\xAB\x10\x0F\xFC\0a\x1B\xFB\x87\x88a\x17\xCCV[a\x1C\x07\x90`\0\x19a%#V[a\x1C\x11\x91\x90a#\xB1V[a\x1C\x1B\x91\x90a$\xFBV[\x92PP`\0a\x1C)\x83a\x1CYV[\x90P`\0a\x1C7\x85\x83a\x17\xCCV[\x90P`\0\x88\x12a\x1CGW\x80a\x0F\xCEV[a\x0F\xCE\x81g\x1B\xC1mgN\xC8\0\0a#\xB1V[`\0h\x02H\xCE6\xA7\x0C\xB2k>\x19\x82\x13a\x1CtWP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a\x1C\xBBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x01a\x07WV[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05`\x01`_\x1B\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1Dm\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1Dn\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x05\x85\x14\x17\x16a\x1E\x1AW`\0\x80\xFD[\x05\x92\x91PPV[`\0\x80\x82\x11a\x1E^W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x07WV[P`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x82\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[`\0`\x01`\xFF\x1B\x82\x03a\x1E\xEFW`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x12\x15a\x1F\0WP\x19`\x01\x01\x90V[P\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x81\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a 6Wa 6a\x1F\xFDV[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a eWa ea\x1F\xFDV[`@R\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a \x83Wa \x83a\x1F\x04V[\x825\x91P` \x80\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a \xA6Wa \xA6a\x1FTV[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a \xBDWa \xBDa\x1F\xA4V[\x815\x81\x81\x11\x15a \xCFWa \xCFa\x1F\xFDV[a \xE1`\x1F\x82\x01`\x1F\x19\x16\x85\x01a <V[\x91P\x80\x82R\x87\x84\x82\x85\x01\x01\x11\x15a!GW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x85\x90R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R`\x84\x81\xFD[\x80\x84\x84\x01\x85\x84\x017`\0\x84\x82\x84\x01\x01RP\x80\x93PPPP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a!zWa!za\x1F\x04V[P5\x91\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a!\x99Wa!\x99a\x1F\x04V[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[`\0\x80`\0`@\x84\x86\x03\x12\x15a!\xC8Wa!\xC8a\x1F\x04V[\x835\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a!\xEAWa!\xEAa\x1FTV[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\"\x01Wa\"\x01a\x1F\xA4V[\x815\x81\x81\x11\x15a\"dW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x0ENL/$\r\x8C\xAD\xCC\xEE\x8D`\xAB\x1B`d\x82\x01R`\x84\x81\xFD[\x87` \x82\x85\x01\x01\x11\x15a\"\xCCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rjrray stride`\xA8\x1B`d\x82\x01R\x91P`\x84\x82\xFD[` \x83\x01\x94P\x80\x93PPPP\x92P\x92P\x92V[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a#\x0CW\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a\"\xF0V[P`\0`@\x82\x86\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x92PPP\x92\x91PPV[`\0\x80`\0``\x84\x86\x03\x12\x15a#EWa#Ea\x1F\x04V[\x83Q\x92P` \x84\x01Q\x91P`@\x84\x01Q\x90P\x92P\x92P\x92V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x05:Wa\x05:a#^V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x05:Wa\x05:a#^V[\x80\x82\x01\x80\x82\x11\x15a\x05:Wa\x05:a#^V[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a#\xD1Wa#\xD1a#^V[P\x92\x91PPV[`\0`\x01`\xFF\x1B\x82\x01a#\xEDWa#\xEDa#^V[P`\0\x03\x90V[` \x80\x82R`\x11\x90\x82\x01Rp\x15\\\x19\x18]\x19H\x19[\x99\x08\x1C\x18\\\xDC\xD9Y`z\x1B`@\x82\x01R``\x01\x90V[`\0\x82a$<WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[`\0\x80`\0\x80\x84\x86\x03`\xC0\x81\x12\x15a$[Wa$[a\x1F\x04V[\x855\x94P` \x86\x015\x93P`@\x86\x015\x92P```_\x19\x82\x01\x12\x15a$\xCBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FABI decoding: struct data too sh`D\x82\x01Rb\x1B\xDC\x9D`\xEA\x1B`d\x82\x01R`\x84\x81\xFD[Pa$\xD4a \x13V[``\x86\x015\x81R`\x80\x86\x015` \x82\x01R`\xA0\x90\x95\x015`@\x86\x01RP\x91\x94\x90\x93P\x90\x91\x90V[\x80\x82\x01\x82\x81\x12`\0\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a%\x1BWa%\x1Ba#^V[PP\x92\x91PPV[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15a%?Wa%?a#^V[\x81\x81\x05\x83\x14\x82\x15\x17a\x05:Wa\x05:a#^V";
    /// The deployed bytecode of the contract.
    pub static LOGNORMAL_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct LogNormal<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for LogNormal<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for LogNormal<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for LogNormal<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for LogNormal<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(LogNormal)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> LogNormal<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    LOGNORMAL_ABI.clone(),
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
                LOGNORMAL_ABI.clone(),
                LOGNORMAL_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `computeSwapConstant` (0x002e524b) function
        pub fn compute_swap_constant(
            &self,
            pool_id: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([0, 46, 82, 75], (pool_id, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `core` (0xf2f4eb26) function
        pub fn core(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([242, 244, 235, 38], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `dynamicSlot` (0xa4d47a5e) function
        pub fn dynamic_slot(
            &self,
            pool_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([164, 212, 122, 94], pool_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `dynamicSlotInternal` (0xec4dc15b) function
        pub fn dynamic_slot_internal(
            &self,
            pool_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, LogNormParameters> {
            self.0
                .method_hash([236, 77, 193, 91], pool_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getParams` (0x08a4f072) function
        pub fn get_params(
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
                .method_hash([8, 164, 240, 114], pool_id)
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
        ///Calls the contract's `init` (0x9f83137b) function
        pub fn init(
            &self,
            pool_id: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                bool,
                ::ethers::core::types::I256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([159, 131, 19, 123], (pool_id, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setSigma` (0x7251308c) function
        pub fn set_sigma(
            &self,
            pool_id: ::ethers::core::types::U256,
            new_target_sigma: ::ethers::core::types::U256,
            new_sigma_update_end: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [114, 81, 48, 140],
                    (pool_id, new_target_sigma, new_sigma_update_end),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setStrikePrice` (0xd7109535) function
        pub fn set_strike_price(
            &self,
            pool_id: ::ethers::core::types::U256,
            new_target_strike: ::ethers::core::types::U256,
            new_strike_update_end: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [215, 16, 149, 53],
                    (pool_id, new_target_strike, new_strike_update_end),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setTau` (0x3f43baac) function
        pub fn set_tau(
            &self,
            pool_id: ::ethers::core::types::U256,
            new_target_tau: ::ethers::core::types::U256,
            new_tau_update_end: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [63, 67, 186, 172],
                    (pool_id, new_target_tau, new_tau_update_end),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sigma` (0x133336aa) function
        pub fn sigma(
            &self,
            pool_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([19, 51, 54, 170], pool_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sigmas` (0x8c3388dc) function
        pub fn sigmas(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([140, 51, 136, 220], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `slots` (0x387dd9e9) function
        pub fn slots(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([56, 125, 217, 233], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `staticSlot` (0xfc1305e5) function
        pub fn static_slot(
            &self,
            pool_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, LogNormParameters> {
            self.0
                .method_hash([252, 19, 5, 229], pool_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `strikePrice` (0xe800bd44) function
        pub fn strike_price(
            &self,
            pool_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([232, 0, 189, 68], pool_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `strikes` (0x792417b1) function
        pub fn strikes(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([121, 36, 23, 177], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `swapFee` (0x54cf2aeb) function
        pub fn swap_fee(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([84, 207, 42, 235], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `tau` (0x8cc1c1ad) function
        pub fn tau(
            &self,
            pool_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([140, 193, 193, 173], pool_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `taus` (0x1698aa49) function
        pub fn taus(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([22, 152, 170, 73], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `validateAllocateOrDeallocate` (0x8e2dd400) function
        pub fn validate_allocate_or_deallocate(
            &self,
            pool_id: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                bool,
                ::ethers::core::types::I256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([142, 45, 212, 0], (pool_id, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `validateSwap` (0x3214890f) function
        pub fn validate_swap(
            &self,
            pool_id: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                bool,
                ::ethers::core::types::I256,
                ::ethers::core::types::I256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([50, 20, 137, 15], (pool_id, data))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `SetSigma` event
        pub fn set_sigma_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SetSigmaFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `SetStrikePrice` event
        pub fn set_strike_price_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SetStrikePriceFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `SetTau` event
        pub fn set_tau_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, SetTauFilter> {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogNormalEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for LogNormal<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `Infinity` with signature `Infinity()` and selector `0x07a02127`
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
    #[etherror(name = "Infinity", abi = "Infinity()")]
    pub struct Infinity;
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
    ///Custom Error type `NegativeInfinity` with signature `NegativeInfinity()` and selector `0x8bb56614`
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
    #[etherror(name = "NegativeInfinity", abi = "NegativeInfinity()")]
    pub struct NegativeInfinity;
    ///Custom Error type `OutOfBounds` with signature `OutOfBounds()` and selector `0xb4120f14`
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
    #[etherror(name = "OutOfBounds", abi = "OutOfBounds()")]
    pub struct OutOfBounds;
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
    pub enum LogNormalErrors {
        Infinity(Infinity),
        Min(Min),
        NegativeInfinity(NegativeInfinity),
        OutOfBounds(OutOfBounds),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for LogNormalErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <Infinity as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Infinity(decoded));
            }
            if let Ok(decoded) = <Min as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Min(decoded));
            }
            if let Ok(decoded) = <NegativeInfinity as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NegativeInfinity(decoded));
            }
            if let Ok(decoded) = <OutOfBounds as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OutOfBounds(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for LogNormalErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::Infinity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Min(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NegativeInfinity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OutOfBounds(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for LogNormalErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <Infinity as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <Min as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NegativeInfinity as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OutOfBounds as ::ethers::contract::EthError>::selector() => true,
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for LogNormalErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Infinity(element) => ::core::fmt::Display::fmt(element, f),
                Self::Min(element) => ::core::fmt::Display::fmt(element, f),
                Self::NegativeInfinity(element) => ::core::fmt::Display::fmt(element, f),
                Self::OutOfBounds(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for LogNormalErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<Infinity> for LogNormalErrors {
        fn from(value: Infinity) -> Self {
            Self::Infinity(value)
        }
    }
    impl ::core::convert::From<Min> for LogNormalErrors {
        fn from(value: Min) -> Self {
            Self::Min(value)
        }
    }
    impl ::core::convert::From<NegativeInfinity> for LogNormalErrors {
        fn from(value: NegativeInfinity) -> Self {
            Self::NegativeInfinity(value)
        }
    }
    impl ::core::convert::From<OutOfBounds> for LogNormalErrors {
        fn from(value: OutOfBounds) -> Self {
            Self::OutOfBounds(value)
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
    #[ethevent(name = "SetSigma", abi = "SetSigma(uint256,uint256,uint256,uint256)")]
    pub struct SetSigmaFilter {
        pub target_sigma: ::ethers::core::types::U256,
        pub last_sigma: ::ethers::core::types::U256,
        pub sigma_update_end: ::ethers::core::types::U256,
        pub delta: ::ethers::core::types::U256,
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
        name = "SetStrikePrice",
        abi = "SetStrikePrice(uint256,uint256,uint256,uint256)"
    )]
    pub struct SetStrikePriceFilter {
        pub target_strike: ::ethers::core::types::U256,
        pub last_strike: ::ethers::core::types::U256,
        pub strike_update_end: ::ethers::core::types::U256,
        pub delta: ::ethers::core::types::U256,
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
    #[ethevent(name = "SetTau", abi = "SetTau(uint256,uint256,uint256,uint256)")]
    pub struct SetTauFilter {
        pub target_tau: ::ethers::core::types::U256,
        pub last_tau: ::ethers::core::types::U256,
        pub tau_update_end: ::ethers::core::types::U256,
        pub delta: ::ethers::core::types::U256,
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
    pub enum LogNormalEvents {
        SetSigmaFilter(SetSigmaFilter),
        SetStrikePriceFilter(SetStrikePriceFilter),
        SetTauFilter(SetTauFilter),
    }
    impl ::ethers::contract::EthLogDecode for LogNormalEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = SetSigmaFilter::decode_log(log) {
                return Ok(LogNormalEvents::SetSigmaFilter(decoded));
            }
            if let Ok(decoded) = SetStrikePriceFilter::decode_log(log) {
                return Ok(LogNormalEvents::SetStrikePriceFilter(decoded));
            }
            if let Ok(decoded) = SetTauFilter::decode_log(log) {
                return Ok(LogNormalEvents::SetTauFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for LogNormalEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::SetSigmaFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetStrikePriceFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetTauFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<SetSigmaFilter> for LogNormalEvents {
        fn from(value: SetSigmaFilter) -> Self {
            Self::SetSigmaFilter(value)
        }
    }
    impl ::core::convert::From<SetStrikePriceFilter> for LogNormalEvents {
        fn from(value: SetStrikePriceFilter) -> Self {
            Self::SetStrikePriceFilter(value)
        }
    }
    impl ::core::convert::From<SetTauFilter> for LogNormalEvents {
        fn from(value: SetTauFilter) -> Self {
            Self::SetTauFilter(value)
        }
    }
    ///Container type for all input parameters for the `computeSwapConstant` function with signature `computeSwapConstant(uint256,bytes)` and selector `0x002e524b`
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
    #[ethcall(name = "computeSwapConstant", abi = "computeSwapConstant(uint256,bytes)")]
    pub struct ComputeSwapConstantCall {
        pub pool_id: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `core` function with signature `core()` and selector `0xf2f4eb26`
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
    #[ethcall(name = "core", abi = "core()")]
    pub struct CoreCall;
    ///Container type for all input parameters for the `dynamicSlot` function with signature `dynamicSlot(uint256)` and selector `0xa4d47a5e`
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
    #[ethcall(name = "dynamicSlot", abi = "dynamicSlot(uint256)")]
    pub struct DynamicSlotCall {
        pub pool_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `dynamicSlotInternal` function with signature `dynamicSlotInternal(uint256)` and selector `0xec4dc15b`
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
    #[ethcall(name = "dynamicSlotInternal", abi = "dynamicSlotInternal(uint256)")]
    pub struct DynamicSlotInternalCall {
        pub pool_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getParams` function with signature `getParams(uint256)` and selector `0x08a4f072`
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
    #[ethcall(name = "getParams", abi = "getParams(uint256)")]
    pub struct GetParamsCall {
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
    ///Container type for all input parameters for the `init` function with signature `init(uint256,bytes)` and selector `0x9f83137b`
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
    #[ethcall(name = "init", abi = "init(uint256,bytes)")]
    pub struct InitCall {
        pub pool_id: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `setSigma` function with signature `setSigma(uint256,uint256,uint256)` and selector `0x7251308c`
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
    #[ethcall(name = "setSigma", abi = "setSigma(uint256,uint256,uint256)")]
    pub struct SetSigmaCall {
        pub pool_id: ::ethers::core::types::U256,
        pub new_target_sigma: ::ethers::core::types::U256,
        pub new_sigma_update_end: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setStrikePrice` function with signature `setStrikePrice(uint256,uint256,uint256)` and selector `0xd7109535`
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
    #[ethcall(name = "setStrikePrice", abi = "setStrikePrice(uint256,uint256,uint256)")]
    pub struct SetStrikePriceCall {
        pub pool_id: ::ethers::core::types::U256,
        pub new_target_strike: ::ethers::core::types::U256,
        pub new_strike_update_end: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setTau` function with signature `setTau(uint256,uint256,uint256)` and selector `0x3f43baac`
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
    #[ethcall(name = "setTau", abi = "setTau(uint256,uint256,uint256)")]
    pub struct SetTauCall {
        pub pool_id: ::ethers::core::types::U256,
        pub new_target_tau: ::ethers::core::types::U256,
        pub new_tau_update_end: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `sigma` function with signature `sigma(uint256)` and selector `0x133336aa`
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
    #[ethcall(name = "sigma", abi = "sigma(uint256)")]
    pub struct SigmaCall {
        pub pool_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `sigmas` function with signature `sigmas(uint256)` and selector `0x8c3388dc`
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
    #[ethcall(name = "sigmas", abi = "sigmas(uint256)")]
    pub struct SigmasCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `slots` function with signature `slots(uint256)` and selector `0x387dd9e9`
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
    #[ethcall(name = "slots", abi = "slots(uint256)")]
    pub struct SlotsCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `staticSlot` function with signature `staticSlot(uint256)` and selector `0xfc1305e5`
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
    #[ethcall(name = "staticSlot", abi = "staticSlot(uint256)")]
    pub struct StaticSlotCall {
        pub pool_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `strikePrice` function with signature `strikePrice(uint256)` and selector `0xe800bd44`
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
    #[ethcall(name = "strikePrice", abi = "strikePrice(uint256)")]
    pub struct StrikePriceCall {
        pub pool_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `strikes` function with signature `strikes(uint256)` and selector `0x792417b1`
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
    #[ethcall(name = "strikes", abi = "strikes(uint256)")]
    pub struct StrikesCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `swapFee` function with signature `swapFee()` and selector `0x54cf2aeb`
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
    #[ethcall(name = "swapFee", abi = "swapFee()")]
    pub struct SwapFeeCall;
    ///Container type for all input parameters for the `tau` function with signature `tau(uint256)` and selector `0x8cc1c1ad`
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
    #[ethcall(name = "tau", abi = "tau(uint256)")]
    pub struct TauCall {
        pub pool_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `taus` function with signature `taus(uint256)` and selector `0x1698aa49`
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
    #[ethcall(name = "taus", abi = "taus(uint256)")]
    pub struct TausCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `validateAllocateOrDeallocate` function with signature `validateAllocateOrDeallocate(uint256,bytes)` and selector `0x8e2dd400`
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
        name = "validateAllocateOrDeallocate",
        abi = "validateAllocateOrDeallocate(uint256,bytes)"
    )]
    pub struct ValidateAllocateOrDeallocateCall {
        pub pool_id: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `validateSwap` function with signature `validateSwap(uint256,bytes)` and selector `0x3214890f`
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
    #[ethcall(name = "validateSwap", abi = "validateSwap(uint256,bytes)")]
    pub struct ValidateSwapCall {
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
    pub enum LogNormalCalls {
        ComputeSwapConstant(ComputeSwapConstantCall),
        Core(CoreCall),
        DynamicSlot(DynamicSlotCall),
        DynamicSlotInternal(DynamicSlotInternalCall),
        GetParams(GetParamsCall),
        GetReservesAndLiquidity(GetReservesAndLiquidityCall),
        Init(InitCall),
        SetSigma(SetSigmaCall),
        SetStrikePrice(SetStrikePriceCall),
        SetTau(SetTauCall),
        Sigma(SigmaCall),
        Sigmas(SigmasCall),
        Slots(SlotsCall),
        StaticSlot(StaticSlotCall),
        StrikePrice(StrikePriceCall),
        Strikes(StrikesCall),
        SwapFee(SwapFeeCall),
        Tau(TauCall),
        Taus(TausCall),
        ValidateAllocateOrDeallocate(ValidateAllocateOrDeallocateCall),
        ValidateSwap(ValidateSwapCall),
    }
    impl ::ethers::core::abi::AbiDecode for LogNormalCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <ComputeSwapConstantCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ComputeSwapConstant(decoded));
            }
            if let Ok(decoded) = <CoreCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Core(decoded));
            }
            if let Ok(decoded) = <DynamicSlotCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DynamicSlot(decoded));
            }
            if let Ok(decoded) = <DynamicSlotInternalCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DynamicSlotInternal(decoded));
            }
            if let Ok(decoded) = <GetParamsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetParams(decoded));
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
            if let Ok(decoded) = <SetSigmaCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetSigma(decoded));
            }
            if let Ok(decoded) = <SetStrikePriceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetStrikePrice(decoded));
            }
            if let Ok(decoded) = <SetTauCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetTau(decoded));
            }
            if let Ok(decoded) = <SigmaCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Sigma(decoded));
            }
            if let Ok(decoded) = <SigmasCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Sigmas(decoded));
            }
            if let Ok(decoded) = <SlotsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Slots(decoded));
            }
            if let Ok(decoded) = <StaticSlotCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::StaticSlot(decoded));
            }
            if let Ok(decoded) = <StrikePriceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::StrikePrice(decoded));
            }
            if let Ok(decoded) = <StrikesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Strikes(decoded));
            }
            if let Ok(decoded) = <SwapFeeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SwapFee(decoded));
            }
            if let Ok(decoded) = <TauCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Tau(decoded));
            }
            if let Ok(decoded) = <TausCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Taus(decoded));
            }
            if let Ok(decoded) = <ValidateAllocateOrDeallocateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ValidateAllocateOrDeallocate(decoded));
            }
            if let Ok(decoded) = <ValidateSwapCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ValidateSwap(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for LogNormalCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::ComputeSwapConstant(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Core(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DynamicSlot(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DynamicSlotInternal(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetParams(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetReservesAndLiquidity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Init(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetSigma(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetStrikePrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetTau(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Sigma(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Sigmas(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Slots(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::StaticSlot(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StrikePrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Strikes(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SwapFee(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Tau(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Taus(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ValidateAllocateOrDeallocate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ValidateSwap(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for LogNormalCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ComputeSwapConstant(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Core(element) => ::core::fmt::Display::fmt(element, f),
                Self::DynamicSlot(element) => ::core::fmt::Display::fmt(element, f),
                Self::DynamicSlotInternal(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetParams(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetReservesAndLiquidity(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Init(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetSigma(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetStrikePrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetTau(element) => ::core::fmt::Display::fmt(element, f),
                Self::Sigma(element) => ::core::fmt::Display::fmt(element, f),
                Self::Sigmas(element) => ::core::fmt::Display::fmt(element, f),
                Self::Slots(element) => ::core::fmt::Display::fmt(element, f),
                Self::StaticSlot(element) => ::core::fmt::Display::fmt(element, f),
                Self::StrikePrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::Strikes(element) => ::core::fmt::Display::fmt(element, f),
                Self::SwapFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::Tau(element) => ::core::fmt::Display::fmt(element, f),
                Self::Taus(element) => ::core::fmt::Display::fmt(element, f),
                Self::ValidateAllocateOrDeallocate(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ValidateSwap(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ComputeSwapConstantCall> for LogNormalCalls {
        fn from(value: ComputeSwapConstantCall) -> Self {
            Self::ComputeSwapConstant(value)
        }
    }
    impl ::core::convert::From<CoreCall> for LogNormalCalls {
        fn from(value: CoreCall) -> Self {
            Self::Core(value)
        }
    }
    impl ::core::convert::From<DynamicSlotCall> for LogNormalCalls {
        fn from(value: DynamicSlotCall) -> Self {
            Self::DynamicSlot(value)
        }
    }
    impl ::core::convert::From<DynamicSlotInternalCall> for LogNormalCalls {
        fn from(value: DynamicSlotInternalCall) -> Self {
            Self::DynamicSlotInternal(value)
        }
    }
    impl ::core::convert::From<GetParamsCall> for LogNormalCalls {
        fn from(value: GetParamsCall) -> Self {
            Self::GetParams(value)
        }
    }
    impl ::core::convert::From<GetReservesAndLiquidityCall> for LogNormalCalls {
        fn from(value: GetReservesAndLiquidityCall) -> Self {
            Self::GetReservesAndLiquidity(value)
        }
    }
    impl ::core::convert::From<InitCall> for LogNormalCalls {
        fn from(value: InitCall) -> Self {
            Self::Init(value)
        }
    }
    impl ::core::convert::From<SetSigmaCall> for LogNormalCalls {
        fn from(value: SetSigmaCall) -> Self {
            Self::SetSigma(value)
        }
    }
    impl ::core::convert::From<SetStrikePriceCall> for LogNormalCalls {
        fn from(value: SetStrikePriceCall) -> Self {
            Self::SetStrikePrice(value)
        }
    }
    impl ::core::convert::From<SetTauCall> for LogNormalCalls {
        fn from(value: SetTauCall) -> Self {
            Self::SetTau(value)
        }
    }
    impl ::core::convert::From<SigmaCall> for LogNormalCalls {
        fn from(value: SigmaCall) -> Self {
            Self::Sigma(value)
        }
    }
    impl ::core::convert::From<SigmasCall> for LogNormalCalls {
        fn from(value: SigmasCall) -> Self {
            Self::Sigmas(value)
        }
    }
    impl ::core::convert::From<SlotsCall> for LogNormalCalls {
        fn from(value: SlotsCall) -> Self {
            Self::Slots(value)
        }
    }
    impl ::core::convert::From<StaticSlotCall> for LogNormalCalls {
        fn from(value: StaticSlotCall) -> Self {
            Self::StaticSlot(value)
        }
    }
    impl ::core::convert::From<StrikePriceCall> for LogNormalCalls {
        fn from(value: StrikePriceCall) -> Self {
            Self::StrikePrice(value)
        }
    }
    impl ::core::convert::From<StrikesCall> for LogNormalCalls {
        fn from(value: StrikesCall) -> Self {
            Self::Strikes(value)
        }
    }
    impl ::core::convert::From<SwapFeeCall> for LogNormalCalls {
        fn from(value: SwapFeeCall) -> Self {
            Self::SwapFee(value)
        }
    }
    impl ::core::convert::From<TauCall> for LogNormalCalls {
        fn from(value: TauCall) -> Self {
            Self::Tau(value)
        }
    }
    impl ::core::convert::From<TausCall> for LogNormalCalls {
        fn from(value: TausCall) -> Self {
            Self::Taus(value)
        }
    }
    impl ::core::convert::From<ValidateAllocateOrDeallocateCall> for LogNormalCalls {
        fn from(value: ValidateAllocateOrDeallocateCall) -> Self {
            Self::ValidateAllocateOrDeallocate(value)
        }
    }
    impl ::core::convert::From<ValidateSwapCall> for LogNormalCalls {
        fn from(value: ValidateSwapCall) -> Self {
            Self::ValidateSwap(value)
        }
    }
    ///Container type for all return fields from the `computeSwapConstant` function with signature `computeSwapConstant(uint256,bytes)` and selector `0x002e524b`
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
    pub struct ComputeSwapConstantReturn(pub ::ethers::core::types::I256);
    ///Container type for all return fields from the `core` function with signature `core()` and selector `0xf2f4eb26`
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
    pub struct CoreReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `dynamicSlot` function with signature `dynamicSlot(uint256)` and selector `0xa4d47a5e`
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
    pub struct DynamicSlotReturn {
        pub params: ::ethers::core::types::Bytes,
    }
    ///Container type for all return fields from the `dynamicSlotInternal` function with signature `dynamicSlotInternal(uint256)` and selector `0xec4dc15b`
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
    pub struct DynamicSlotInternalReturn {
        pub params: LogNormParameters,
    }
    ///Container type for all return fields from the `getParams` function with signature `getParams(uint256)` and selector `0x08a4f072`
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
    pub struct GetParamsReturn(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
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
    ///Container type for all return fields from the `init` function with signature `init(uint256,bytes)` and selector `0x9f83137b`
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
    pub struct InitReturn {
        pub valid: bool,
        pub invariant: ::ethers::core::types::I256,
        pub rx: ::ethers::core::types::U256,
        pub ry: ::ethers::core::types::U256,
        pub l: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `sigma` function with signature `sigma(uint256)` and selector `0x133336aa`
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
    pub struct SigmaReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `sigmas` function with signature `sigmas(uint256)` and selector `0x8c3388dc`
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
    pub struct SigmasReturn {
        pub target: ::ethers::core::types::U256,
        pub last: ::ethers::core::types::U256,
        pub update_end: ::ethers::core::types::U256,
        pub update_per_second: ::ethers::core::types::U256,
        pub last_sync: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `slots` function with signature `slots(uint256)` and selector `0x387dd9e9`
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
    pub struct SlotsReturn {
        pub strike: ::ethers::core::types::U256,
        pub sigma: ::ethers::core::types::U256,
        pub tau: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `staticSlot` function with signature `staticSlot(uint256)` and selector `0xfc1305e5`
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
    pub struct StaticSlotReturn(pub LogNormParameters);
    ///Container type for all return fields from the `strikePrice` function with signature `strikePrice(uint256)` and selector `0xe800bd44`
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
    pub struct StrikePriceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `strikes` function with signature `strikes(uint256)` and selector `0x792417b1`
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
    pub struct StrikesReturn {
        pub target: ::ethers::core::types::U256,
        pub last: ::ethers::core::types::U256,
        pub update_end: ::ethers::core::types::U256,
        pub update_per_second: ::ethers::core::types::U256,
        pub last_sync: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `swapFee` function with signature `swapFee()` and selector `0x54cf2aeb`
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
    pub struct SwapFeeReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `tau` function with signature `tau(uint256)` and selector `0x8cc1c1ad`
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
    pub struct TauReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `taus` function with signature `taus(uint256)` and selector `0x1698aa49`
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
    pub struct TausReturn {
        pub target: ::ethers::core::types::U256,
        pub last: ::ethers::core::types::U256,
        pub update_end: ::ethers::core::types::U256,
        pub update_per_second: ::ethers::core::types::U256,
        pub last_sync: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `validateAllocateOrDeallocate` function with signature `validateAllocateOrDeallocate(uint256,bytes)` and selector `0x8e2dd400`
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
    pub struct ValidateAllocateOrDeallocateReturn {
        pub valid: bool,
        pub invariant: ::ethers::core::types::I256,
        pub rx: ::ethers::core::types::U256,
        pub ry: ::ethers::core::types::U256,
        pub l: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `validateSwap` function with signature `validateSwap(uint256,bytes)` and selector `0x3214890f`
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
    pub struct ValidateSwapReturn {
        pub valid: bool,
        pub invariant: ::ethers::core::types::I256,
        pub liquidity_delta: ::ethers::core::types::I256,
        pub next_rx: ::ethers::core::types::U256,
        pub next_ry: ::ethers::core::types::U256,
        pub next_l: ::ethers::core::types::U256,
    }
}
