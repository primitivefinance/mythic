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
                        name: ::std::borrow::ToOwned::to_owned("dfmm_"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("dfmm"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("dfmm"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getPoolParams"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getPoolParams"),
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
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("internalParams"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("internalParams"),
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
                                    name: ::std::borrow::ToOwned::to_owned("sigma"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct DynamicParam"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tau"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct DynamicParam"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("strike"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct DynamicParam"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("swapFee"),
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
                                    name: ::std::borrow::ToOwned::to_owned("target"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("end"),
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
                                    name: ::std::borrow::ToOwned::to_owned("target"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("end"),
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
                                    name: ::std::borrow::ToOwned::to_owned("target"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("end"),
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
            events: ::std::collections::BTreeMap::new(),
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
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15b\0\0^W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`@Qb\0%\xE98\x03\x80b\0%\xE9\x839\x81\x01`@\x81\x90Rb\0\0\x81\x91b\0\0\xA7V[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90Ub\0\x01$V[`\0` \x82\x84\x03\x12\x15b\0\x01\x05W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x01\x1DW`\0\x80\xFD[\x93\x92PPPV[a$\xB5\x80b\0\x014`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\0\xF5W`\x005`\xE0\x1C\x80crQ0\x8C\x11a\0\xBEW\x80crQ0\x8C\x14a\x02\xCBW\x80c\x8E-\xD4\0\x14a\x02\xDEW\x80c\x9F\x83\x13{\x14a\x03\x1BW\x80c\xAF\xBA\x13\xC4\x14a\x03.W\x80c\xD7\x10\x955\x14a\x03YW\x80c\xDC\x17\x83U\x14a\x03lWa\0\xF5V[\x80b.RK\x14a\x01ZW\x80c\x02\x16\xB88\x14a\x01\x80W\x80c\x1E\xDBq\xE5\x14a\x01\x95W\x80c2\x14\x89\x0F\x14a\x02vW\x80c?C\xBA\xAC\x14a\x02\xB8W[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x01ma\x01h6`\x04a\x1E\x1AV[a\x03\x8CV[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\x93a\x01\x8E6`\x04a\x1F\x12V[a\x03\xDFV[\0[a\x02fa\x01\xA36`\x04a AV[`\x01` \x81\x81R`\0\x92\x83R`@\x92\x83\x90 \x83Q`\xA0\x80\x82\x01\x86R\x82T\x82R\x93\x82\x01T\x81\x84\x01R`\x02\x82\x01T\x81\x86\x01R`\x03\x82\x01T``\x80\x83\x01\x91\x90\x91R`\x04\x83\x01T`\x80\x80\x84\x01\x91\x90\x91R\x86Q\x80\x87\x01\x88R`\x05\x85\x01T\x81R`\x06\x85\x01T\x81\x87\x01R`\x07\x85\x01T\x81\x89\x01R`\x08\x85\x01T\x81\x84\x01R`\t\x85\x01T\x81\x83\x01R\x87Q\x96\x87\x01\x88R`\n\x85\x01T\x87R`\x0B\x85\x01T\x95\x87\x01\x95\x90\x95R`\x0C\x84\x01T\x96\x86\x01\x96\x90\x96R`\r\x83\x01T\x90\x85\x01R`\x0E\x82\x01T\x94\x84\x01\x94\x90\x94R`\x0F\x01T\x90\x91\x90\x84V[`@Qa\x01w\x94\x93\x92\x91\x90a ]V[a\x02\x89a\x02\x846`\x04a\x1E\x1AV[a\x04\x9CV[`@\x80Q\x96\x15\x15\x87R` \x87\x01\x95\x90\x95R\x93\x85\x01\x92\x90\x92R``\x84\x01R`\x80\x83\x01R`\xA0\x82\x01R`\xC0\x01a\x01wV[a\x01\x93a\x02\xC66`\x04a!\x0CV[a\x07\x08V[a\x01\x93a\x02\xD96`\x04a!\x0CV[a\x08\x9CV[a\x02\xF1a\x02\xEC6`\x04a\x1F\x12V[a\n4V[`@\x80Q\x95\x15\x15\x86R` \x86\x01\x94\x90\x94R\x92\x84\x01\x91\x90\x91R``\x83\x01R`\x80\x82\x01R`\xA0\x01a\x01wV[a\x02\xF1a\x03)6`\x04a\x1F\x12V[a\n\x83V[`\0Ta\x03A\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01wV[a\x01\x93a\x03g6`\x04a!\x0CV[a\n\xABV[a\x03\x7Fa\x03z6`\x04a AV[a\x0CAV[`@Qa\x01w\x91\x90a!;V[`\0\x80`\0\x80\x84\x80` \x01\x90Q\x81\x01\x90a\x03\xA6\x91\x90a!\x89V[\x92P\x92P\x92Pa\x03\xD3\x83\x83\x83a\x03\xBB\x8Aa\x0CAV[\x80` \x01\x90Q\x81\x01\x90a\x03\xCE\x91\x90a\"\x0BV[a\r\xDAV[\x93PPPP[\x92\x91PPV[`\0a\x03\xED\x82\x84\x01\x84a\"\xC7V[`\0\x94\x85R`\x01` \x81\x81R`@\x96\x87\x90 \x83Q\x80Q\x82U\x80\x83\x01Q\x93\x82\x01\x93\x90\x93U\x82\x88\x01Q`\x02\x82\x01U``\x80\x84\x01Q`\x03\x83\x01U`\x80\x93\x84\x01Q`\x04\x83\x01U\x82\x85\x01Q\x80Q`\x05\x84\x01U\x80\x84\x01Q`\x06\x84\x01U\x80\x8A\x01Q`\x07\x84\x01U\x80\x82\x01Q`\x08\x84\x01U\x84\x01Q`\t\x83\x01U\x88\x85\x01Q\x80Q`\n\x84\x01U\x92\x83\x01Q`\x0B\x83\x01U\x97\x82\x01Q`\x0C\x82\x01U\x81\x88\x01Q`\r\x82\x01U\x91\x01Q`\x0E\x82\x01U\x94\x01Q`\x0F\x90\x94\x01\x93\x90\x93UPPPV[`\0\x80`\0\x80`\0\x80`\0a\x04\xB0\x89a\x0CAV[\x80` \x01\x90Q\x81\x01\x90a\x04\xC3\x91\x90a\"\x0BV[`\0\x80T`@Qc3\x85N\xFD`\xE2\x1B\x81R`\x04\x81\x01\x8D\x90R\x92\x93P\x90\x91\x82\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xCE\x15;\xF4\x90`$\x01```@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x05aW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x05uW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\x99\x91\x90a!\x89V[\x92P\x92P\x92P\x8A\x80` \x01\x90Q\x81\x01\x90a\x05\xB3\x91\x90a!\x89V[\x91\x98P\x96P\x94P`\0\x80\x80\x85\x8A\x11\x15a\x06\x10Wa\x05\xD0\x86\x8Ba#?V[\x91Pa\x05\xE9\x87``\x01Q\x83a\x0E\xEC\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa\x05\xFF\x86a\x05\xF9\x83\x87a\x0E\xECV[\x90a\x0F\x08V[a\x06\t\x90\x84a#RV[\x92Pa\x06\xB1V[\x84\x89\x11\x15a\x06KWa\x06\"\x85\x8Aa#?V[\x91Pa\x06;\x87``\x01Q\x83a\x0E\xEC\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa\x05\xFF\x85a\x05\xF9\x83\x87a\x0E\xECV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`0`$\x82\x01R\x7Finvalid swap: inputs x and y hav`D\x82\x01Roe the same sign!`\x80\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[a\x06\xBB\x84\x89a#eV[\x9APa\x06\xC9\x8A\x8A\x8A\x8Aa\r\xDAV[\x9BP`\0\x8Ca\x06\xD8`\x1Ea#\x8CV[\x12\x80\x15a\x06\xE5WP`\x1E\x8D\x12[\x90P\x80\x80\x15a\x06\xF4WP\x83\x8C\x12\x15[\x9DPPPPPPPPP\x92\x95P\x92\x95P\x92\x95V[`\0\x83\x81R`\x01` \x81\x81R`@\x92\x83\x90 \x83Qa\x01 \x81\x01\x85R\x81T`\x80\x80\x83\x01\x91\x82R\x94\x83\x01T`\xA0\x80\x84\x01\x91\x90\x91R`\x02\x84\x01T`\xC0\x84\x01R`\x03\x84\x01T`\xE0\x84\x01R`\x04\x84\x01Ta\x01\0\x84\x01R\x90\x82R\x85Q\x80\x82\x01\x87R`\x05\x84\x01T\x81R`\x06\x84\x01T\x81\x86\x01R`\x07\x84\x01T\x81\x88\x01R`\x08\x84\x01T``\x80\x83\x01\x91\x90\x91R`\t\x85\x01T\x82\x88\x01R\x83\x86\x01\x91\x82R\x87Q\x92\x83\x01\x88R`\n\x85\x01T\x83R`\x0B\x85\x01T\x95\x83\x01\x95\x90\x95R`\x0C\x84\x01T\x82\x88\x01R`\r\x84\x01T\x82\x86\x01R`\x0E\x84\x01T\x95\x82\x01\x95\x90\x95R\x94\x81\x01\x94\x90\x94R`\x0F\x01T\x90\x83\x01RQa\x07\xEC\x90\x84\x84a\x0F\x1DV[` \x80\x83\x01\x91\x82R`\0\x95\x86R`\x01\x80\x82R`@\x96\x87\x90 \x84Q\x80Q\x82U\x80\x84\x01Q\x92\x82\x01\x92\x90\x92U\x81\x88\x01Q`\x02\x82\x01U``\x80\x83\x01Q`\x03\x83\x01U`\x80\x92\x83\x01Q`\x04\x83\x01U\x93Q\x80Q`\x05\x83\x01U\x80\x84\x01Q`\x06\x83\x01U\x80\x89\x01Q`\x07\x83\x01U\x80\x85\x01Q`\x08\x83\x01U\x82\x01Q`\t\x82\x01U\x87\x85\x01Q\x80Q`\n\x83\x01U\x92\x83\x01Q`\x0B\x82\x01U\x96\x82\x01Q`\x0C\x88\x01U\x81\x83\x01Q`\r\x88\x01U\x01Q`\x0E\x86\x01U\x01Q`\x0F\x90\x93\x01\x92\x90\x92UPPV[`\0\x83\x81R`\x01` \x81\x81R`@\x92\x83\x90 \x83Qa\x01 \x81\x01\x85R\x81T`\x80\x80\x83\x01\x91\x82R\x94\x83\x01T`\xA0\x80\x84\x01\x91\x90\x91R`\x02\x84\x01T`\xC0\x84\x01R`\x03\x84\x01T`\xE0\x84\x01R`\x04\x84\x01Ta\x01\0\x84\x01R\x90\x82R\x85Q\x80\x82\x01\x87R`\x05\x84\x01T\x81R`\x06\x84\x01T\x81\x86\x01R`\x07\x84\x01T\x81\x88\x01R`\x08\x84\x01T``\x80\x83\x01\x91\x90\x91R`\t\x85\x01T\x82\x88\x01R\x83\x86\x01\x91\x90\x91R\x86Q\x91\x82\x01\x87R`\n\x84\x01T\x82R`\x0B\x84\x01T\x94\x82\x01\x94\x90\x94R`\x0C\x83\x01T\x81\x87\x01R`\r\x83\x01T\x81\x85\x01R`\x0E\x83\x01T\x94\x81\x01\x94\x90\x94R\x93\x84\x01\x92\x90\x92R`\x0F\x90\x91\x01T\x90\x82\x01R\x80Qa\t\x84\x90\x84\x84a\x0F\x1DV[\x81R`\0\x93\x84R`\x01` \x81\x81R`@\x95\x86\x90 \x83Q\x80Q\x82U\x80\x83\x01Q\x93\x82\x01\x93\x90\x93U\x82\x87\x01Q`\x02\x82\x01U``\x80\x84\x01Q`\x03\x83\x01U`\x80\x93\x84\x01Q`\x04\x83\x01U\x82\x85\x01Q\x80Q`\x05\x84\x01U\x80\x84\x01Q`\x06\x84\x01U\x80\x89\x01Q`\x07\x84\x01U\x80\x82\x01Q`\x08\x84\x01U\x84\x01Q`\t\x83\x01U\x87\x85\x01Q\x80Q`\n\x84\x01U\x92\x83\x01Q`\x0B\x83\x01U\x96\x82\x01Q`\x0C\x82\x01U\x81\x87\x01Q`\r\x82\x01U\x91\x01Q`\x0E\x82\x01U\x93\x01Q`\x0F\x90\x93\x01\x92\x90\x92UPPV[`\0\x80\x80\x80\x80a\nF\x86\x88\x01\x88a!\x0CV[\x91\x94P\x92P\x90Pa\n\\\x83\x83\x83a\x03\xBB\x8Ca\x0CAV[\x93P\x83a\ni`\x1Ea#\x8CV[\x12\x80\x15a\nvWP`\x1E\x84\x12[\x94P\x93\x97\x92\x96P\x93P\x93PV[`\0\x80`\0\x80`\0a\n\x96\x88\x88\x88a\x10\x1FV[P\x93\x9C\x92\x9BP\x90\x99P\x97P\x90\x95P\x93PPPPV[`\0\x83\x81R`\x01` \x81\x81R`@\x92\x83\x90 \x83Qa\x01 \x81\x01\x85R\x81T`\x80\x80\x83\x01\x91\x82R\x94\x83\x01T`\xA0\x80\x84\x01\x91\x90\x91R`\x02\x84\x01T`\xC0\x84\x01R`\x03\x84\x01T`\xE0\x84\x01R`\x04\x84\x01Ta\x01\0\x84\x01R\x90\x82R\x85Q\x80\x82\x01\x87R`\x05\x84\x01T\x81R`\x06\x84\x01T\x81\x86\x01R`\x07\x84\x01T\x81\x88\x01R`\x08\x84\x01T``\x80\x83\x01\x91\x90\x91R`\t\x85\x01T\x82\x88\x01R\x83\x86\x01\x91\x90\x91R\x86Q\x91\x82\x01\x87R`\n\x84\x01T\x82R`\x0B\x84\x01T\x94\x82\x01\x94\x90\x94R`\x0C\x83\x01T\x81\x87\x01R`\r\x83\x01T\x81\x85\x01R`\x0E\x83\x01T\x94\x81\x01\x94\x90\x94R\x93\x84\x01\x83\x90R`\x0F\x01T\x90\x83\x01Ra\x0B\x8E\x90\x84\x84a\x0F\x1DV[`@\x80\x83\x01\x91\x82R`\0\x95\x86R`\x01` \x81\x81R\x96\x82\x90 \x84Q\x80Q\x82U\x80\x89\x01Q\x92\x82\x01\x92\x90\x92U\x81\x83\x01Q`\x02\x82\x01U``\x80\x83\x01Q`\x03\x83\x01U`\x80\x92\x83\x01Q`\x04\x83\x01U\x88\x86\x01Q\x80Q`\x05\x84\x01U\x80\x8A\x01Q`\x06\x84\x01U\x80\x85\x01Q`\x07\x84\x01U\x80\x82\x01Q`\x08\x84\x01U\x83\x01Q`\t\x83\x01U\x93Q\x80Q`\n\x83\x01U\x97\x88\x01Q`\x0B\x82\x01U\x91\x87\x01Q`\x0C\x83\x01U\x86\x83\x01Q`\r\x83\x01U\x95\x90\x95\x01Q`\x0E\x86\x01U\x01Q`\x0F\x90\x93\x01\x92\x90\x92UPPV[``a\x0Cn`@Q\x80`\x80\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[a\x0C\xC9`\x01`\0\x85\x81R` \x01\x90\x81R` \x01`\0 `\0\x01`@Q\x80`\xA0\x01`@R\x90\x81`\0\x82\x01T\x81R` \x01`\x01\x82\x01T\x81R` \x01`\x02\x82\x01T\x81R` \x01`\x03\x82\x01T\x81R` \x01`\x04\x82\x01T\x81RPPa\x10\xD7V[` \x80\x83\x01\x91\x90\x91R`\0\x84\x81R`\x01\x82R`@\x90\x81\x90 \x81Q`\xA0\x81\x01\x83R`\n\x82\x01T\x81R`\x0B\x82\x01T\x93\x81\x01\x93\x90\x93R`\x0C\x81\x01T\x91\x83\x01\x91\x90\x91R`\r\x81\x01T``\x83\x01R`\x0E\x01T`\x80\x82\x01Ra\r$\x90a\x10\xD7V[\x81R`\0\x83\x81R`\x01` \x90\x81R`@\x91\x82\x90 \x82Q`\xA0\x81\x01\x84R`\x05\x82\x01T\x81R`\x06\x82\x01T\x92\x81\x01\x92\x90\x92R`\x07\x81\x01T\x92\x82\x01\x92\x90\x92R`\x08\x82\x01T``\x82\x01R`\t\x90\x91\x01T`\x80\x82\x01Ra\r}\x90a\x10\xD7V[`@\x82\x81\x01\x91\x82R`\0\x94\x85R`\x01` \x90\x81R\x94\x81\x90 `\x0F\x01T``\x80\x85\x01\x91\x82R\x82Q\x85Q\x81\x89\x01R\x96\x90\x94\x01Q\x86\x83\x01R\x91Q\x92\x85\x01\x92\x90\x92RQ`\x80\x80\x85\x01\x91\x90\x91R\x81Q\x80\x85\x03\x90\x91\x01\x81R`\xA0\x90\x93\x01\x90RP\x90V[`\0\x82\x85\x10a\x0E+W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FtradingFunction: invalid x\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06\xA8V[`\0\x80g\r\xE0\xB6\xB3\xA7d\0\0a\x0EA\x88\x87a\x11nV[\x10a\x0EUW`\x01`\x01`\xFF\x1B\x03\x91Pa\x0EjV[a\x0Ega\x0Eb\x88\x87a\x11nV[a\x11\x83V[\x91P[g\r\xE0\xB6\xB3\xA7d\0\0a\x0E\x8A\x87a\x0E\x85\x87`\0\x01Q\x89a\x12)V[a\x11nV[\x10a\x0E\x9DWP`\x01`\x01`\xFF\x1B\x03a\x0E\xB5V[a\x0E\xB2a\x0Eb\x87a\x0E\x85\x87`\0\x01Q\x89a\x12)V[\x90P[`\0a\x0E\xC9\x85` \x01Q\x86`@\x01Qa\x12>V[\x90P\x80a\x0E\xD6\x83\x85a#\xA8V[a\x0E\xE0\x91\x90a#\xA8V[\x98\x97PPPPPPPPV[`\0a\x0F\x01\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x12dV[\x93\x92PPPV[`\0a\x0F\x01\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x12dV[a\x0FO`@Q\x80`\xA0\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[B\x82\x11a\x0F\xB7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`0`$\x82\x01R\x7FEnd must be greater than current`D\x82\x01Ro\x02\x06&\xC6\xF66\xB2\xE7F\x96\xD6W7F\x16\xD7`\x84\x1B`d\x82\x01R`\x84\x01a\x06\xA8V[`\0a\x0F\xC2\x85a\x12\x92V[\x90P`\0\x84\x82` \x01Q\x11a\x0F\xE5W` \x82\x01Qa\x0F\xE0\x90\x86a#?V[a\x0F\xF5V[\x84\x82` \x01Qa\x0F\xF5\x91\x90a#?V[\x90Pa\x10\x01B\x85a#?V[a\x10\x0B\x90\x82a#\xD0V[``\x83\x01RP\x92\x83RP`@\x82\x01R\x91\x90PV[`\0\x80`\0\x80`\0a\x10R`@Q\x80`\x80\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[a\x10^\x87\x89\x01\x89a#\xF2V[` \x80\x82\x01Q`\0\x8F\x81R`\x01\x92\x83\x90R`@\x90\x81\x90 \x92\x83\x01\x91\x90\x91U\x82\x01Q`\x06\x82\x01U\x81Q`\x0B\x82\x01U``\x82\x01Q`\x0F\x90\x91\x01U\x92\x96P\x90\x94P\x92P\x90Pa\x10\xAF\x84\x84\x84a\x03\xBB\x8Da\x0CAV[\x94P\x84a\x10\xBC`\x1Ea#\x8CV[\x12\x80\x15a\x10\xC9WP`\x1E\x85\x12[\x95P\x93\x97P\x93\x97\x91\x95P\x93PV[`\0\x81`@\x01Q\x82`\x80\x01Q\x03a\x10\xF0WP` \x01Q\x90V[`\0\x82`@\x01QB\x11a\x11\x03WBa\x11\tV[\x82`@\x01Q[\x90P`\0\x83`\x80\x01Q\x82a\x11\x1D\x91\x90a#?V[``\x85\x01Q\x90\x91P\x15a\x11PW``\x84\x01Qa\x119\x90\x82a$nV[\x84` \x01Qa\x11H\x91\x90a#RV[\x94\x93PPPPV[``\x84\x01Qa\x11_\x90\x82a$nV[\x84` \x01Qa\x11H\x91\x90a#?V[`\0a\x0F\x01\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x12\xDCV[`\0g\x06\xF0[Y\xD3\xB2\0\0\x82\x03a\x11\x9CWP`\0\x91\x90PV[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x12a\x11\xC4W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\x11\xE5W`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x11\xF2\x83`\x02a$\x85V[\x90P`\0a\x11\xFF\x82a\x12\xFBV[\x90P`\0a\x12\x15g\x13\xA0K\xBD\xFD\xC9\xBE\x88\x83a\x15yV[\x90Pa\x12 \x81a#\x8CV[\x95\x94PPPPPV[`\0a\x0F\x01\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x12\xDCV[`\0\x80a\x12J\x83a\x15\x8EV[a\x12X\x90c;\x9A\xCA\0a$nV[\x90Pa\x11H\x84\x82a\x12)V[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x12|W`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[a\x12\xC4`@Q\x80`\xA0\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[a\x12\xCD\x82a\x10\xD7V[` \x83\x01RPB`\x80\x82\x01R\x90V[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x12\xF4W`\0\x80\xFD[\x04\x92\x91PPV[`\0\x80\x82\x12\x80a\x13\x12WPg\x1B\xC1mgN\xC8\0\0\x82\x13[\x15a\x130W`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\x13QW`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81g\x1B\xC1mgN\xC8\0\0\x03a\x13yW`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x15a\x13\x84W\x91\x90PV[`\0g\r\xE0\xB6\xB3\xA7d\0\0\x83\x12a\x13\xACWa\x13\xA7\x83g\x1B\xC1mgN\xC8\0\0a#eV[a\x13\xAEV[\x82[\x90P`\0a\x13\xC4\x82g\x1B\xC1mgN\xC8\0\0a\x162V[\x90P\x80`\0\x03a\x13\xE7W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x13\xF2\x82a\x16GV[\x90P`\0c;\x9A\xCA\0a\x14\x1Da\x14\x18a\x14\x12g\x1B\xC1mgN\xC8\0\0a#\x8CV[\x85a\x15yV[a\x15\x8EV[a\x14'\x91\x90a$\x85V[\x90P`\0\x80a\x14>\x83g\x03\xC1f\\z\xAB \0a\x15yV[a\x14P\x90g \x05\xFEO&\x8E\xA0\0a#\xA8V[\x90P`\0a\x14\x80\x84a\x14i\x86f\x9F2u$b\xA0\0a\x15yV[a\x14{\x90g\r\xC5R\x7Fd, \0a#\xA8V[a\x15yV[a\x14\x92\x90g\r\xE0\xB6\xB3\xA7d\0\0a#\xA8V[\x90Pa\x14\xB6g\t\xD0(\xCCo _\xFF\x19\x85a\x14\xAC\x85\x85a\x162V[a\x14{\x91\x90a#eV[\x92PPP`\0[`\x02\x81\x10\x15a\x15QW`\0\x86a\x14\xD2\x84a\x18\"V[a\x14\xDC\x91\x90a#eV[\x90P`\0a\x14\xEA\x84\x85a\x15yV[a\x14\xF3\x90a#\x8CV[\x90P`\0a\x15\0\x82a\x1A\x06V[\x90P`\0a\x15\x0E\x86\x85a\x15yV[a\x15 g\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x84a\x15yV[a\x15*\x91\x90a#eV[\x90Pa\x156\x84\x82a\x162V[a\x15@\x90\x87a#\xA8V[\x95P\x84`\x01\x01\x94PPPPPa\x14\xBDV[g\r\xE0\xB6\xB3\xA7d\0\0\x88\x12a\x15nWa\x15i\x82a#\x8CV[a\x0E\xE0V[P\x96\x95PPPPPPV[`\0a\x0F\x01\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x1B\xAFV[`\xB5\x81`\x01`\x88\x1B\x81\x10a\x15\xA7W`@\x91\x90\x91\x1B\x90`\x80\x1C[i\x01\0\0\0\0\0\0\0\0\0\x81\x10a\x15\xC3W` \x91\x90\x91\x1B\x90`@\x1C[e\x01\0\0\0\0\0\x81\x10a\x15\xDBW`\x10\x91\x90\x91\x1B\x90` \x1C[c\x01\0\0\0\x81\x10a\x15\xF1W`\x08\x91\x90\x91\x1B\x90`\x10\x1C[b\x01\0\0\x01\x02`\x12\x1C\x80\x82\x04\x01`\x01\x90\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x90\x81\x90\x04\x81\x11\x90\x03\x90V[`\0a\x0F\x01\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x1B\xAFV[`\0\x80\x82\x13a\x16\x84W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x06\xA8V[`\0``a\x16\x91\x84a\x1B\xCEV[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[`\0\x81`\0\x03a\x18;WPg\r\xE0\xB6\xB3\xA7d\0\0\x91\x90PV[gV\x98\xEE\xF0fp\0\0\x82\x12a\x18RWP`\0\x91\x90PV[a\x18cgV\x98\xEE\xF0fp\0\0a#\x8CV[\x82\x13a\x18xWPg\x1B\xC1mgN\xC8\0\0\x91\x90PV[`\0a\x18\x83\x83a\x1CvV[\x90P`\0a\x18\xBCg\r\xE0\xB6\xB3\xA7d\0\0a\x18\xA5\x84g\x1B\xC1mgN\xC8\0\0a\x11nV[a\x18\xB7\x90g\r\xE0\xB6\xB3\xA7d\0\0a#\xA8V[a\x162V[\x90P`\0\x80\x82a\x19\x18\x81a\x19\x05\x81a\x18\xF3\x81a\x18\xE0\x81g\x02_\x0F\xE1\x05\xA3\x14\0a\x15yV[a\x14{\x90g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19a#\xA8V[a\x14{\x90g\x14\xA8EL\x19\xE1\xAC\0a#\xA8V[a\x14{\x90g\x0F\xC1\x0E\x01W\x82w\xFF\x19a#\xA8V[a\x19*\x90g\x03\xDE\xBD\x08;\x8C|\0a#\xA8V[\x91P\x83\x90Pa\x19\x92\x81a\x19\x80\x81a\x19n\x81a\x19\\\x81a\x19I\x81\x8Ba\x15yV[a\x14{\x90g\x02\x95\xD4\0\xEA2W\xFF\x19a#\xA8V[a\x14{\x90g\x01W\xD8\xB2\xEC\xC7\x08\0a#\xA8V[a\x14{\x90g\x051\n\xA7\xD5!0\0a#\xA8V[a\x14{\x90g\r\xE0\xCC=\x15a\0\0a#\xA8V[\x91P\x81g\x11\x90\0\xAB\x10\x0F\xFC\0a\x19\xA8\x87\x88a\x15yV[a\x19\xB4\x90`\0\x19a$\x85V[a\x19\xBE\x91\x90a#eV[a\x19\xC8\x91\x90a#\xA8V[\x92PP`\0a\x19\xD6\x83a\x1A\x06V[\x90P`\0a\x19\xE4\x85\x83a\x15yV[\x90P`\0\x88\x12a\x19\xF4W\x80a\x0E\xE0V[a\x0E\xE0\x81g\x1B\xC1mgN\xC8\0\0a#eV[`\0h\x02H\xCE6\xA7\x0C\xB2k>\x19\x82\x13a\x1A!WP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a\x1AhW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x01a\x06\xA8V[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05`\x01`_\x1B\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1Dm\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1Dn\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x05\x85\x14\x17\x16a\x1B\xC7W`\0\x80\xFD[\x05\x92\x91PPV[`\0\x80\x82\x11a\x1C\x0BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x06\xA8V[P`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x82\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[`\0`\x01`\xFF\x1B\x82\x03a\x1C\x9CW`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x12\x15a\x1C\xADWP\x19`\x01\x01\x90V[P\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x81\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x80\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x1D\xE3Wa\x1D\xE3a\x1D\xAAV[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x1E\x12Wa\x1E\x12a\x1D\xAAV[`@R\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x1E0Wa\x1E0a\x1C\xB1V[\x825\x91P` \x80\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x1ESWa\x1ESa\x1D\x01V[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x1EjWa\x1Eja\x1DQV[\x815\x81\x81\x11\x15a\x1E|Wa\x1E|a\x1D\xAAV[a\x1E\x8E`\x1F\x82\x01`\x1F\x19\x16\x85\x01a\x1D\xE9V[\x91P\x80\x82R\x87\x84\x82\x85\x01\x01\x11\x15a\x1E\xF4W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x85\x90R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R`\x84\x81\xFD[\x80\x84\x84\x01\x85\x84\x017`\0\x84\x82\x84\x01\x01RP\x80\x93PPPP\x92P\x92\x90PV[`\0\x80`\0`@\x84\x86\x03\x12\x15a\x1F*Wa\x1F*a\x1C\xB1V[\x835\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x1FLWa\x1FLa\x1D\x01V[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x1FcWa\x1Fca\x1DQV[\x815\x81\x81\x11\x15a\x1F\xC6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x0ENL/$\r\x8C\xAD\xCC\xEE\x8D`\xAB\x1B`d\x82\x01R`\x84\x81\xFD[\x87` \x82\x85\x01\x01\x11\x15a .W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rjrray stride`\xA8\x1B`d\x82\x01R\x91P`\x84\x82\xFD[` \x83\x01\x94P\x80\x93PPPP\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a VWa Va\x1C\xB1V[P5\x91\x90PV[a\x02\0\x81\x01a \x97\x82\x87\x80Q\x82R` \x81\x01Q` \x83\x01R`@\x81\x01Q`@\x83\x01R``\x81\x01Q``\x83\x01R`\x80\x81\x01Q`\x80\x83\x01RPPV[\x84Q`\xA0\x83\x01R` \x85\x01Q`\xC0\x83\x01R`@\x85\x01Q`\xE0\x83\x01R``\x85\x01Qa\x01\0\x83\x01R`\x80\x85\x01Qa\x01 \x83\x01R\x83Qa\x01@\x83\x01R` \x84\x01Qa\x01`\x83\x01R`@\x84\x01Qa\x01\x80\x83\x01R``\x84\x01Qa\x01\xA0\x83\x01R`\x80\x84\x01Qa\x01\xC0\x83\x01R\x82a\x01\xE0\x83\x01R\x95\x94PPPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a!$Wa!$a\x1C\xB1V[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a!hW\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a!LV[P`\0`@\x82\x86\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x92PPP\x92\x91PPV[`\0\x80`\0``\x84\x86\x03\x12\x15a!\xA1Wa!\xA1a\x1C\xB1V[\x83Q\x92P` \x84\x01Q\x91P`@\x84\x01Q\x90P\x92P\x92P\x92V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FABI decoding: struct data too sh`D\x82\x01Rb\x1B\xDC\x9D`\xEA\x1B`d\x82\x01R`\x84\x81\xFD[`\0`\x80\x82\x84\x03\x12\x15a\" Wa\" a\x1C\xB1V[a\"(a\x1D\xC0V[\x82Q\x81R` \x83\x01Q` \x82\x01R`@\x83\x01Q`@\x82\x01R``\x83\x01Q``\x82\x01R\x80\x91PP\x92\x91PPV[`\0`\xA0\x82\x84\x03\x12\x15a\"iWa\"ia!\xBAV[`@Q`\xA0\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\"\x8CWa\"\x8Ca\x1D\xAAV[\x80`@RP\x80\x91P\x825\x81R` \x83\x015` \x82\x01R`@\x83\x015`@\x82\x01R``\x83\x015``\x82\x01R`\x80\x83\x015`\x80\x82\x01RP\x92\x91PPV[`\0a\x02\0\x82\x84\x03\x12\x15a\"\xDDWa\"\xDDa\x1C\xB1V[a\"\xE5a\x1D\xC0V[a\"\xEF\x84\x84a\"TV[\x81Ra\"\xFE\x84`\xA0\x85\x01a\"TV[` \x82\x01Ra#\x11\x84a\x01@\x85\x01a\"TV[`@\x82\x01Ra\x01\xE0\x92\x90\x92\x015``\x83\x01RP\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x03\xD9Wa\x03\xD9a#)V[\x80\x82\x01\x80\x82\x11\x15a\x03\xD9Wa\x03\xD9a#)V[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a#\x85Wa#\x85a#)V[P\x92\x91PPV[`\0`\x01`\xFF\x1B\x82\x01a#\xA1Wa#\xA1a#)V[P`\0\x03\x90V[\x80\x82\x01\x82\x81\x12`\0\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a#\xC8Wa#\xC8a#)V[PP\x92\x91PPV[`\0\x82a#\xEDWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[`\0\x80`\0\x80\x84\x86\x03`\xE0\x81\x12\x15a$\x0CWa$\x0Ca\x1C\xB1V[\x855\x94P` \x86\x015\x93P`@\x86\x015\x92P`\x80`_\x19\x82\x01\x12\x15a$3Wa$3a!\xBAV[Pa$<a\x1D\xC0V[``\x86\x015\x81R`\x80\x86\x015` \x82\x01R`\xA0\x86\x015`@\x82\x01R`\xC0\x86\x015``\x82\x01R\x80\x91PP\x92\x95\x91\x94P\x92PV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x03\xD9Wa\x03\xD9a#)V[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15a$\xA1Wa$\xA1a#)V[\x81\x81\x05\x83\x14\x82\x15\x17a\x03\xD9Wa\x03\xD9a#)V";
    /// The bytecode of the contract.
    pub static LOGNORMAL_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\0\xF5W`\x005`\xE0\x1C\x80crQ0\x8C\x11a\0\xBEW\x80crQ0\x8C\x14a\x02\xCBW\x80c\x8E-\xD4\0\x14a\x02\xDEW\x80c\x9F\x83\x13{\x14a\x03\x1BW\x80c\xAF\xBA\x13\xC4\x14a\x03.W\x80c\xD7\x10\x955\x14a\x03YW\x80c\xDC\x17\x83U\x14a\x03lWa\0\xF5V[\x80b.RK\x14a\x01ZW\x80c\x02\x16\xB88\x14a\x01\x80W\x80c\x1E\xDBq\xE5\x14a\x01\x95W\x80c2\x14\x89\x0F\x14a\x02vW\x80c?C\xBA\xAC\x14a\x02\xB8W[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x01ma\x01h6`\x04a\x1E\x1AV[a\x03\x8CV[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\x93a\x01\x8E6`\x04a\x1F\x12V[a\x03\xDFV[\0[a\x02fa\x01\xA36`\x04a AV[`\x01` \x81\x81R`\0\x92\x83R`@\x92\x83\x90 \x83Q`\xA0\x80\x82\x01\x86R\x82T\x82R\x93\x82\x01T\x81\x84\x01R`\x02\x82\x01T\x81\x86\x01R`\x03\x82\x01T``\x80\x83\x01\x91\x90\x91R`\x04\x83\x01T`\x80\x80\x84\x01\x91\x90\x91R\x86Q\x80\x87\x01\x88R`\x05\x85\x01T\x81R`\x06\x85\x01T\x81\x87\x01R`\x07\x85\x01T\x81\x89\x01R`\x08\x85\x01T\x81\x84\x01R`\t\x85\x01T\x81\x83\x01R\x87Q\x96\x87\x01\x88R`\n\x85\x01T\x87R`\x0B\x85\x01T\x95\x87\x01\x95\x90\x95R`\x0C\x84\x01T\x96\x86\x01\x96\x90\x96R`\r\x83\x01T\x90\x85\x01R`\x0E\x82\x01T\x94\x84\x01\x94\x90\x94R`\x0F\x01T\x90\x91\x90\x84V[`@Qa\x01w\x94\x93\x92\x91\x90a ]V[a\x02\x89a\x02\x846`\x04a\x1E\x1AV[a\x04\x9CV[`@\x80Q\x96\x15\x15\x87R` \x87\x01\x95\x90\x95R\x93\x85\x01\x92\x90\x92R``\x84\x01R`\x80\x83\x01R`\xA0\x82\x01R`\xC0\x01a\x01wV[a\x01\x93a\x02\xC66`\x04a!\x0CV[a\x07\x08V[a\x01\x93a\x02\xD96`\x04a!\x0CV[a\x08\x9CV[a\x02\xF1a\x02\xEC6`\x04a\x1F\x12V[a\n4V[`@\x80Q\x95\x15\x15\x86R` \x86\x01\x94\x90\x94R\x92\x84\x01\x91\x90\x91R``\x83\x01R`\x80\x82\x01R`\xA0\x01a\x01wV[a\x02\xF1a\x03)6`\x04a\x1F\x12V[a\n\x83V[`\0Ta\x03A\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01wV[a\x01\x93a\x03g6`\x04a!\x0CV[a\n\xABV[a\x03\x7Fa\x03z6`\x04a AV[a\x0CAV[`@Qa\x01w\x91\x90a!;V[`\0\x80`\0\x80\x84\x80` \x01\x90Q\x81\x01\x90a\x03\xA6\x91\x90a!\x89V[\x92P\x92P\x92Pa\x03\xD3\x83\x83\x83a\x03\xBB\x8Aa\x0CAV[\x80` \x01\x90Q\x81\x01\x90a\x03\xCE\x91\x90a\"\x0BV[a\r\xDAV[\x93PPPP[\x92\x91PPV[`\0a\x03\xED\x82\x84\x01\x84a\"\xC7V[`\0\x94\x85R`\x01` \x81\x81R`@\x96\x87\x90 \x83Q\x80Q\x82U\x80\x83\x01Q\x93\x82\x01\x93\x90\x93U\x82\x88\x01Q`\x02\x82\x01U``\x80\x84\x01Q`\x03\x83\x01U`\x80\x93\x84\x01Q`\x04\x83\x01U\x82\x85\x01Q\x80Q`\x05\x84\x01U\x80\x84\x01Q`\x06\x84\x01U\x80\x8A\x01Q`\x07\x84\x01U\x80\x82\x01Q`\x08\x84\x01U\x84\x01Q`\t\x83\x01U\x88\x85\x01Q\x80Q`\n\x84\x01U\x92\x83\x01Q`\x0B\x83\x01U\x97\x82\x01Q`\x0C\x82\x01U\x81\x88\x01Q`\r\x82\x01U\x91\x01Q`\x0E\x82\x01U\x94\x01Q`\x0F\x90\x94\x01\x93\x90\x93UPPPV[`\0\x80`\0\x80`\0\x80`\0a\x04\xB0\x89a\x0CAV[\x80` \x01\x90Q\x81\x01\x90a\x04\xC3\x91\x90a\"\x0BV[`\0\x80T`@Qc3\x85N\xFD`\xE2\x1B\x81R`\x04\x81\x01\x8D\x90R\x92\x93P\x90\x91\x82\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xCE\x15;\xF4\x90`$\x01```@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x05aW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x05uW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\x99\x91\x90a!\x89V[\x92P\x92P\x92P\x8A\x80` \x01\x90Q\x81\x01\x90a\x05\xB3\x91\x90a!\x89V[\x91\x98P\x96P\x94P`\0\x80\x80\x85\x8A\x11\x15a\x06\x10Wa\x05\xD0\x86\x8Ba#?V[\x91Pa\x05\xE9\x87``\x01Q\x83a\x0E\xEC\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa\x05\xFF\x86a\x05\xF9\x83\x87a\x0E\xECV[\x90a\x0F\x08V[a\x06\t\x90\x84a#RV[\x92Pa\x06\xB1V[\x84\x89\x11\x15a\x06KWa\x06\"\x85\x8Aa#?V[\x91Pa\x06;\x87``\x01Q\x83a\x0E\xEC\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa\x05\xFF\x85a\x05\xF9\x83\x87a\x0E\xECV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`0`$\x82\x01R\x7Finvalid swap: inputs x and y hav`D\x82\x01Roe the same sign!`\x80\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[a\x06\xBB\x84\x89a#eV[\x9APa\x06\xC9\x8A\x8A\x8A\x8Aa\r\xDAV[\x9BP`\0\x8Ca\x06\xD8`\x1Ea#\x8CV[\x12\x80\x15a\x06\xE5WP`\x1E\x8D\x12[\x90P\x80\x80\x15a\x06\xF4WP\x83\x8C\x12\x15[\x9DPPPPPPPPP\x92\x95P\x92\x95P\x92\x95V[`\0\x83\x81R`\x01` \x81\x81R`@\x92\x83\x90 \x83Qa\x01 \x81\x01\x85R\x81T`\x80\x80\x83\x01\x91\x82R\x94\x83\x01T`\xA0\x80\x84\x01\x91\x90\x91R`\x02\x84\x01T`\xC0\x84\x01R`\x03\x84\x01T`\xE0\x84\x01R`\x04\x84\x01Ta\x01\0\x84\x01R\x90\x82R\x85Q\x80\x82\x01\x87R`\x05\x84\x01T\x81R`\x06\x84\x01T\x81\x86\x01R`\x07\x84\x01T\x81\x88\x01R`\x08\x84\x01T``\x80\x83\x01\x91\x90\x91R`\t\x85\x01T\x82\x88\x01R\x83\x86\x01\x91\x82R\x87Q\x92\x83\x01\x88R`\n\x85\x01T\x83R`\x0B\x85\x01T\x95\x83\x01\x95\x90\x95R`\x0C\x84\x01T\x82\x88\x01R`\r\x84\x01T\x82\x86\x01R`\x0E\x84\x01T\x95\x82\x01\x95\x90\x95R\x94\x81\x01\x94\x90\x94R`\x0F\x01T\x90\x83\x01RQa\x07\xEC\x90\x84\x84a\x0F\x1DV[` \x80\x83\x01\x91\x82R`\0\x95\x86R`\x01\x80\x82R`@\x96\x87\x90 \x84Q\x80Q\x82U\x80\x84\x01Q\x92\x82\x01\x92\x90\x92U\x81\x88\x01Q`\x02\x82\x01U``\x80\x83\x01Q`\x03\x83\x01U`\x80\x92\x83\x01Q`\x04\x83\x01U\x93Q\x80Q`\x05\x83\x01U\x80\x84\x01Q`\x06\x83\x01U\x80\x89\x01Q`\x07\x83\x01U\x80\x85\x01Q`\x08\x83\x01U\x82\x01Q`\t\x82\x01U\x87\x85\x01Q\x80Q`\n\x83\x01U\x92\x83\x01Q`\x0B\x82\x01U\x96\x82\x01Q`\x0C\x88\x01U\x81\x83\x01Q`\r\x88\x01U\x01Q`\x0E\x86\x01U\x01Q`\x0F\x90\x93\x01\x92\x90\x92UPPV[`\0\x83\x81R`\x01` \x81\x81R`@\x92\x83\x90 \x83Qa\x01 \x81\x01\x85R\x81T`\x80\x80\x83\x01\x91\x82R\x94\x83\x01T`\xA0\x80\x84\x01\x91\x90\x91R`\x02\x84\x01T`\xC0\x84\x01R`\x03\x84\x01T`\xE0\x84\x01R`\x04\x84\x01Ta\x01\0\x84\x01R\x90\x82R\x85Q\x80\x82\x01\x87R`\x05\x84\x01T\x81R`\x06\x84\x01T\x81\x86\x01R`\x07\x84\x01T\x81\x88\x01R`\x08\x84\x01T``\x80\x83\x01\x91\x90\x91R`\t\x85\x01T\x82\x88\x01R\x83\x86\x01\x91\x90\x91R\x86Q\x91\x82\x01\x87R`\n\x84\x01T\x82R`\x0B\x84\x01T\x94\x82\x01\x94\x90\x94R`\x0C\x83\x01T\x81\x87\x01R`\r\x83\x01T\x81\x85\x01R`\x0E\x83\x01T\x94\x81\x01\x94\x90\x94R\x93\x84\x01\x92\x90\x92R`\x0F\x90\x91\x01T\x90\x82\x01R\x80Qa\t\x84\x90\x84\x84a\x0F\x1DV[\x81R`\0\x93\x84R`\x01` \x81\x81R`@\x95\x86\x90 \x83Q\x80Q\x82U\x80\x83\x01Q\x93\x82\x01\x93\x90\x93U\x82\x87\x01Q`\x02\x82\x01U``\x80\x84\x01Q`\x03\x83\x01U`\x80\x93\x84\x01Q`\x04\x83\x01U\x82\x85\x01Q\x80Q`\x05\x84\x01U\x80\x84\x01Q`\x06\x84\x01U\x80\x89\x01Q`\x07\x84\x01U\x80\x82\x01Q`\x08\x84\x01U\x84\x01Q`\t\x83\x01U\x87\x85\x01Q\x80Q`\n\x84\x01U\x92\x83\x01Q`\x0B\x83\x01U\x96\x82\x01Q`\x0C\x82\x01U\x81\x87\x01Q`\r\x82\x01U\x91\x01Q`\x0E\x82\x01U\x93\x01Q`\x0F\x90\x93\x01\x92\x90\x92UPPV[`\0\x80\x80\x80\x80a\nF\x86\x88\x01\x88a!\x0CV[\x91\x94P\x92P\x90Pa\n\\\x83\x83\x83a\x03\xBB\x8Ca\x0CAV[\x93P\x83a\ni`\x1Ea#\x8CV[\x12\x80\x15a\nvWP`\x1E\x84\x12[\x94P\x93\x97\x92\x96P\x93P\x93PV[`\0\x80`\0\x80`\0a\n\x96\x88\x88\x88a\x10\x1FV[P\x93\x9C\x92\x9BP\x90\x99P\x97P\x90\x95P\x93PPPPV[`\0\x83\x81R`\x01` \x81\x81R`@\x92\x83\x90 \x83Qa\x01 \x81\x01\x85R\x81T`\x80\x80\x83\x01\x91\x82R\x94\x83\x01T`\xA0\x80\x84\x01\x91\x90\x91R`\x02\x84\x01T`\xC0\x84\x01R`\x03\x84\x01T`\xE0\x84\x01R`\x04\x84\x01Ta\x01\0\x84\x01R\x90\x82R\x85Q\x80\x82\x01\x87R`\x05\x84\x01T\x81R`\x06\x84\x01T\x81\x86\x01R`\x07\x84\x01T\x81\x88\x01R`\x08\x84\x01T``\x80\x83\x01\x91\x90\x91R`\t\x85\x01T\x82\x88\x01R\x83\x86\x01\x91\x90\x91R\x86Q\x91\x82\x01\x87R`\n\x84\x01T\x82R`\x0B\x84\x01T\x94\x82\x01\x94\x90\x94R`\x0C\x83\x01T\x81\x87\x01R`\r\x83\x01T\x81\x85\x01R`\x0E\x83\x01T\x94\x81\x01\x94\x90\x94R\x93\x84\x01\x83\x90R`\x0F\x01T\x90\x83\x01Ra\x0B\x8E\x90\x84\x84a\x0F\x1DV[`@\x80\x83\x01\x91\x82R`\0\x95\x86R`\x01` \x81\x81R\x96\x82\x90 \x84Q\x80Q\x82U\x80\x89\x01Q\x92\x82\x01\x92\x90\x92U\x81\x83\x01Q`\x02\x82\x01U``\x80\x83\x01Q`\x03\x83\x01U`\x80\x92\x83\x01Q`\x04\x83\x01U\x88\x86\x01Q\x80Q`\x05\x84\x01U\x80\x8A\x01Q`\x06\x84\x01U\x80\x85\x01Q`\x07\x84\x01U\x80\x82\x01Q`\x08\x84\x01U\x83\x01Q`\t\x83\x01U\x93Q\x80Q`\n\x83\x01U\x97\x88\x01Q`\x0B\x82\x01U\x91\x87\x01Q`\x0C\x83\x01U\x86\x83\x01Q`\r\x83\x01U\x95\x90\x95\x01Q`\x0E\x86\x01U\x01Q`\x0F\x90\x93\x01\x92\x90\x92UPPV[``a\x0Cn`@Q\x80`\x80\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[a\x0C\xC9`\x01`\0\x85\x81R` \x01\x90\x81R` \x01`\0 `\0\x01`@Q\x80`\xA0\x01`@R\x90\x81`\0\x82\x01T\x81R` \x01`\x01\x82\x01T\x81R` \x01`\x02\x82\x01T\x81R` \x01`\x03\x82\x01T\x81R` \x01`\x04\x82\x01T\x81RPPa\x10\xD7V[` \x80\x83\x01\x91\x90\x91R`\0\x84\x81R`\x01\x82R`@\x90\x81\x90 \x81Q`\xA0\x81\x01\x83R`\n\x82\x01T\x81R`\x0B\x82\x01T\x93\x81\x01\x93\x90\x93R`\x0C\x81\x01T\x91\x83\x01\x91\x90\x91R`\r\x81\x01T``\x83\x01R`\x0E\x01T`\x80\x82\x01Ra\r$\x90a\x10\xD7V[\x81R`\0\x83\x81R`\x01` \x90\x81R`@\x91\x82\x90 \x82Q`\xA0\x81\x01\x84R`\x05\x82\x01T\x81R`\x06\x82\x01T\x92\x81\x01\x92\x90\x92R`\x07\x81\x01T\x92\x82\x01\x92\x90\x92R`\x08\x82\x01T``\x82\x01R`\t\x90\x91\x01T`\x80\x82\x01Ra\r}\x90a\x10\xD7V[`@\x82\x81\x01\x91\x82R`\0\x94\x85R`\x01` \x90\x81R\x94\x81\x90 `\x0F\x01T``\x80\x85\x01\x91\x82R\x82Q\x85Q\x81\x89\x01R\x96\x90\x94\x01Q\x86\x83\x01R\x91Q\x92\x85\x01\x92\x90\x92RQ`\x80\x80\x85\x01\x91\x90\x91R\x81Q\x80\x85\x03\x90\x91\x01\x81R`\xA0\x90\x93\x01\x90RP\x90V[`\0\x82\x85\x10a\x0E+W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FtradingFunction: invalid x\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06\xA8V[`\0\x80g\r\xE0\xB6\xB3\xA7d\0\0a\x0EA\x88\x87a\x11nV[\x10a\x0EUW`\x01`\x01`\xFF\x1B\x03\x91Pa\x0EjV[a\x0Ega\x0Eb\x88\x87a\x11nV[a\x11\x83V[\x91P[g\r\xE0\xB6\xB3\xA7d\0\0a\x0E\x8A\x87a\x0E\x85\x87`\0\x01Q\x89a\x12)V[a\x11nV[\x10a\x0E\x9DWP`\x01`\x01`\xFF\x1B\x03a\x0E\xB5V[a\x0E\xB2a\x0Eb\x87a\x0E\x85\x87`\0\x01Q\x89a\x12)V[\x90P[`\0a\x0E\xC9\x85` \x01Q\x86`@\x01Qa\x12>V[\x90P\x80a\x0E\xD6\x83\x85a#\xA8V[a\x0E\xE0\x91\x90a#\xA8V[\x98\x97PPPPPPPPV[`\0a\x0F\x01\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x12dV[\x93\x92PPPV[`\0a\x0F\x01\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x12dV[a\x0FO`@Q\x80`\xA0\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[B\x82\x11a\x0F\xB7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`0`$\x82\x01R\x7FEnd must be greater than current`D\x82\x01Ro\x02\x06&\xC6\xF66\xB2\xE7F\x96\xD6W7F\x16\xD7`\x84\x1B`d\x82\x01R`\x84\x01a\x06\xA8V[`\0a\x0F\xC2\x85a\x12\x92V[\x90P`\0\x84\x82` \x01Q\x11a\x0F\xE5W` \x82\x01Qa\x0F\xE0\x90\x86a#?V[a\x0F\xF5V[\x84\x82` \x01Qa\x0F\xF5\x91\x90a#?V[\x90Pa\x10\x01B\x85a#?V[a\x10\x0B\x90\x82a#\xD0V[``\x83\x01RP\x92\x83RP`@\x82\x01R\x91\x90PV[`\0\x80`\0\x80`\0a\x10R`@Q\x80`\x80\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[a\x10^\x87\x89\x01\x89a#\xF2V[` \x80\x82\x01Q`\0\x8F\x81R`\x01\x92\x83\x90R`@\x90\x81\x90 \x92\x83\x01\x91\x90\x91U\x82\x01Q`\x06\x82\x01U\x81Q`\x0B\x82\x01U``\x82\x01Q`\x0F\x90\x91\x01U\x92\x96P\x90\x94P\x92P\x90Pa\x10\xAF\x84\x84\x84a\x03\xBB\x8Da\x0CAV[\x94P\x84a\x10\xBC`\x1Ea#\x8CV[\x12\x80\x15a\x10\xC9WP`\x1E\x85\x12[\x95P\x93\x97P\x93\x97\x91\x95P\x93PV[`\0\x81`@\x01Q\x82`\x80\x01Q\x03a\x10\xF0WP` \x01Q\x90V[`\0\x82`@\x01QB\x11a\x11\x03WBa\x11\tV[\x82`@\x01Q[\x90P`\0\x83`\x80\x01Q\x82a\x11\x1D\x91\x90a#?V[``\x85\x01Q\x90\x91P\x15a\x11PW``\x84\x01Qa\x119\x90\x82a$nV[\x84` \x01Qa\x11H\x91\x90a#RV[\x94\x93PPPPV[``\x84\x01Qa\x11_\x90\x82a$nV[\x84` \x01Qa\x11H\x91\x90a#?V[`\0a\x0F\x01\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x12\xDCV[`\0g\x06\xF0[Y\xD3\xB2\0\0\x82\x03a\x11\x9CWP`\0\x91\x90PV[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x12a\x11\xC4W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\x11\xE5W`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x11\xF2\x83`\x02a$\x85V[\x90P`\0a\x11\xFF\x82a\x12\xFBV[\x90P`\0a\x12\x15g\x13\xA0K\xBD\xFD\xC9\xBE\x88\x83a\x15yV[\x90Pa\x12 \x81a#\x8CV[\x95\x94PPPPPV[`\0a\x0F\x01\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x12\xDCV[`\0\x80a\x12J\x83a\x15\x8EV[a\x12X\x90c;\x9A\xCA\0a$nV[\x90Pa\x11H\x84\x82a\x12)V[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x12|W`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[a\x12\xC4`@Q\x80`\xA0\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[a\x12\xCD\x82a\x10\xD7V[` \x83\x01RPB`\x80\x82\x01R\x90V[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x12\xF4W`\0\x80\xFD[\x04\x92\x91PPV[`\0\x80\x82\x12\x80a\x13\x12WPg\x1B\xC1mgN\xC8\0\0\x82\x13[\x15a\x130W`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\x13QW`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81g\x1B\xC1mgN\xC8\0\0\x03a\x13yW`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x15a\x13\x84W\x91\x90PV[`\0g\r\xE0\xB6\xB3\xA7d\0\0\x83\x12a\x13\xACWa\x13\xA7\x83g\x1B\xC1mgN\xC8\0\0a#eV[a\x13\xAEV[\x82[\x90P`\0a\x13\xC4\x82g\x1B\xC1mgN\xC8\0\0a\x162V[\x90P\x80`\0\x03a\x13\xE7W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x13\xF2\x82a\x16GV[\x90P`\0c;\x9A\xCA\0a\x14\x1Da\x14\x18a\x14\x12g\x1B\xC1mgN\xC8\0\0a#\x8CV[\x85a\x15yV[a\x15\x8EV[a\x14'\x91\x90a$\x85V[\x90P`\0\x80a\x14>\x83g\x03\xC1f\\z\xAB \0a\x15yV[a\x14P\x90g \x05\xFEO&\x8E\xA0\0a#\xA8V[\x90P`\0a\x14\x80\x84a\x14i\x86f\x9F2u$b\xA0\0a\x15yV[a\x14{\x90g\r\xC5R\x7Fd, \0a#\xA8V[a\x15yV[a\x14\x92\x90g\r\xE0\xB6\xB3\xA7d\0\0a#\xA8V[\x90Pa\x14\xB6g\t\xD0(\xCCo _\xFF\x19\x85a\x14\xAC\x85\x85a\x162V[a\x14{\x91\x90a#eV[\x92PPP`\0[`\x02\x81\x10\x15a\x15QW`\0\x86a\x14\xD2\x84a\x18\"V[a\x14\xDC\x91\x90a#eV[\x90P`\0a\x14\xEA\x84\x85a\x15yV[a\x14\xF3\x90a#\x8CV[\x90P`\0a\x15\0\x82a\x1A\x06V[\x90P`\0a\x15\x0E\x86\x85a\x15yV[a\x15 g\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x84a\x15yV[a\x15*\x91\x90a#eV[\x90Pa\x156\x84\x82a\x162V[a\x15@\x90\x87a#\xA8V[\x95P\x84`\x01\x01\x94PPPPPa\x14\xBDV[g\r\xE0\xB6\xB3\xA7d\0\0\x88\x12a\x15nWa\x15i\x82a#\x8CV[a\x0E\xE0V[P\x96\x95PPPPPPV[`\0a\x0F\x01\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x1B\xAFV[`\xB5\x81`\x01`\x88\x1B\x81\x10a\x15\xA7W`@\x91\x90\x91\x1B\x90`\x80\x1C[i\x01\0\0\0\0\0\0\0\0\0\x81\x10a\x15\xC3W` \x91\x90\x91\x1B\x90`@\x1C[e\x01\0\0\0\0\0\x81\x10a\x15\xDBW`\x10\x91\x90\x91\x1B\x90` \x1C[c\x01\0\0\0\x81\x10a\x15\xF1W`\x08\x91\x90\x91\x1B\x90`\x10\x1C[b\x01\0\0\x01\x02`\x12\x1C\x80\x82\x04\x01`\x01\x90\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x90\x81\x90\x04\x81\x11\x90\x03\x90V[`\0a\x0F\x01\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x1B\xAFV[`\0\x80\x82\x13a\x16\x84W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x06\xA8V[`\0``a\x16\x91\x84a\x1B\xCEV[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[`\0\x81`\0\x03a\x18;WPg\r\xE0\xB6\xB3\xA7d\0\0\x91\x90PV[gV\x98\xEE\xF0fp\0\0\x82\x12a\x18RWP`\0\x91\x90PV[a\x18cgV\x98\xEE\xF0fp\0\0a#\x8CV[\x82\x13a\x18xWPg\x1B\xC1mgN\xC8\0\0\x91\x90PV[`\0a\x18\x83\x83a\x1CvV[\x90P`\0a\x18\xBCg\r\xE0\xB6\xB3\xA7d\0\0a\x18\xA5\x84g\x1B\xC1mgN\xC8\0\0a\x11nV[a\x18\xB7\x90g\r\xE0\xB6\xB3\xA7d\0\0a#\xA8V[a\x162V[\x90P`\0\x80\x82a\x19\x18\x81a\x19\x05\x81a\x18\xF3\x81a\x18\xE0\x81g\x02_\x0F\xE1\x05\xA3\x14\0a\x15yV[a\x14{\x90g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19a#\xA8V[a\x14{\x90g\x14\xA8EL\x19\xE1\xAC\0a#\xA8V[a\x14{\x90g\x0F\xC1\x0E\x01W\x82w\xFF\x19a#\xA8V[a\x19*\x90g\x03\xDE\xBD\x08;\x8C|\0a#\xA8V[\x91P\x83\x90Pa\x19\x92\x81a\x19\x80\x81a\x19n\x81a\x19\\\x81a\x19I\x81\x8Ba\x15yV[a\x14{\x90g\x02\x95\xD4\0\xEA2W\xFF\x19a#\xA8V[a\x14{\x90g\x01W\xD8\xB2\xEC\xC7\x08\0a#\xA8V[a\x14{\x90g\x051\n\xA7\xD5!0\0a#\xA8V[a\x14{\x90g\r\xE0\xCC=\x15a\0\0a#\xA8V[\x91P\x81g\x11\x90\0\xAB\x10\x0F\xFC\0a\x19\xA8\x87\x88a\x15yV[a\x19\xB4\x90`\0\x19a$\x85V[a\x19\xBE\x91\x90a#eV[a\x19\xC8\x91\x90a#\xA8V[\x92PP`\0a\x19\xD6\x83a\x1A\x06V[\x90P`\0a\x19\xE4\x85\x83a\x15yV[\x90P`\0\x88\x12a\x19\xF4W\x80a\x0E\xE0V[a\x0E\xE0\x81g\x1B\xC1mgN\xC8\0\0a#eV[`\0h\x02H\xCE6\xA7\x0C\xB2k>\x19\x82\x13a\x1A!WP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a\x1AhW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x01a\x06\xA8V[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05`\x01`_\x1B\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1Dm\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1Dn\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x05\x85\x14\x17\x16a\x1B\xC7W`\0\x80\xFD[\x05\x92\x91PPV[`\0\x80\x82\x11a\x1C\x0BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x06\xA8V[P`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x82\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[`\0`\x01`\xFF\x1B\x82\x03a\x1C\x9CW`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x12\x15a\x1C\xADWP\x19`\x01\x01\x90V[P\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x81\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x80\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x1D\xE3Wa\x1D\xE3a\x1D\xAAV[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x1E\x12Wa\x1E\x12a\x1D\xAAV[`@R\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x1E0Wa\x1E0a\x1C\xB1V[\x825\x91P` \x80\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x1ESWa\x1ESa\x1D\x01V[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x1EjWa\x1Eja\x1DQV[\x815\x81\x81\x11\x15a\x1E|Wa\x1E|a\x1D\xAAV[a\x1E\x8E`\x1F\x82\x01`\x1F\x19\x16\x85\x01a\x1D\xE9V[\x91P\x80\x82R\x87\x84\x82\x85\x01\x01\x11\x15a\x1E\xF4W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x85\x90R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R`\x84\x81\xFD[\x80\x84\x84\x01\x85\x84\x017`\0\x84\x82\x84\x01\x01RP\x80\x93PPPP\x92P\x92\x90PV[`\0\x80`\0`@\x84\x86\x03\x12\x15a\x1F*Wa\x1F*a\x1C\xB1V[\x835\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x1FLWa\x1FLa\x1D\x01V[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x1FcWa\x1Fca\x1DQV[\x815\x81\x81\x11\x15a\x1F\xC6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x0ENL/$\r\x8C\xAD\xCC\xEE\x8D`\xAB\x1B`d\x82\x01R`\x84\x81\xFD[\x87` \x82\x85\x01\x01\x11\x15a .W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rjrray stride`\xA8\x1B`d\x82\x01R\x91P`\x84\x82\xFD[` \x83\x01\x94P\x80\x93PPPP\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a VWa Va\x1C\xB1V[P5\x91\x90PV[a\x02\0\x81\x01a \x97\x82\x87\x80Q\x82R` \x81\x01Q` \x83\x01R`@\x81\x01Q`@\x83\x01R``\x81\x01Q``\x83\x01R`\x80\x81\x01Q`\x80\x83\x01RPPV[\x84Q`\xA0\x83\x01R` \x85\x01Q`\xC0\x83\x01R`@\x85\x01Q`\xE0\x83\x01R``\x85\x01Qa\x01\0\x83\x01R`\x80\x85\x01Qa\x01 \x83\x01R\x83Qa\x01@\x83\x01R` \x84\x01Qa\x01`\x83\x01R`@\x84\x01Qa\x01\x80\x83\x01R``\x84\x01Qa\x01\xA0\x83\x01R`\x80\x84\x01Qa\x01\xC0\x83\x01R\x82a\x01\xE0\x83\x01R\x95\x94PPPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a!$Wa!$a\x1C\xB1V[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a!hW\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a!LV[P`\0`@\x82\x86\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x92PPP\x92\x91PPV[`\0\x80`\0``\x84\x86\x03\x12\x15a!\xA1Wa!\xA1a\x1C\xB1V[\x83Q\x92P` \x84\x01Q\x91P`@\x84\x01Q\x90P\x92P\x92P\x92V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FABI decoding: struct data too sh`D\x82\x01Rb\x1B\xDC\x9D`\xEA\x1B`d\x82\x01R`\x84\x81\xFD[`\0`\x80\x82\x84\x03\x12\x15a\" Wa\" a\x1C\xB1V[a\"(a\x1D\xC0V[\x82Q\x81R` \x83\x01Q` \x82\x01R`@\x83\x01Q`@\x82\x01R``\x83\x01Q``\x82\x01R\x80\x91PP\x92\x91PPV[`\0`\xA0\x82\x84\x03\x12\x15a\"iWa\"ia!\xBAV[`@Q`\xA0\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\"\x8CWa\"\x8Ca\x1D\xAAV[\x80`@RP\x80\x91P\x825\x81R` \x83\x015` \x82\x01R`@\x83\x015`@\x82\x01R``\x83\x015``\x82\x01R`\x80\x83\x015`\x80\x82\x01RP\x92\x91PPV[`\0a\x02\0\x82\x84\x03\x12\x15a\"\xDDWa\"\xDDa\x1C\xB1V[a\"\xE5a\x1D\xC0V[a\"\xEF\x84\x84a\"TV[\x81Ra\"\xFE\x84`\xA0\x85\x01a\"TV[` \x82\x01Ra#\x11\x84a\x01@\x85\x01a\"TV[`@\x82\x01Ra\x01\xE0\x92\x90\x92\x015``\x83\x01RP\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x03\xD9Wa\x03\xD9a#)V[\x80\x82\x01\x80\x82\x11\x15a\x03\xD9Wa\x03\xD9a#)V[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a#\x85Wa#\x85a#)V[P\x92\x91PPV[`\0`\x01`\xFF\x1B\x82\x01a#\xA1Wa#\xA1a#)V[P`\0\x03\x90V[\x80\x82\x01\x82\x81\x12`\0\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a#\xC8Wa#\xC8a#)V[PP\x92\x91PPV[`\0\x82a#\xEDWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[`\0\x80`\0\x80\x84\x86\x03`\xE0\x81\x12\x15a$\x0CWa$\x0Ca\x1C\xB1V[\x855\x94P` \x86\x015\x93P`@\x86\x015\x92P`\x80`_\x19\x82\x01\x12\x15a$3Wa$3a!\xBAV[Pa$<a\x1D\xC0V[``\x86\x015\x81R`\x80\x86\x015` \x82\x01R`\xA0\x86\x015`@\x82\x01R`\xC0\x86\x015``\x82\x01R\x80\x91PP\x92\x95\x91\x94P\x92PV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x03\xD9Wa\x03\xD9a#)V[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15a$\xA1Wa$\xA1a#)V[\x81\x81\x05\x83\x14\x82\x15\x17a\x03\xD9Wa\x03\xD9a#)V";
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
        ///Calls the contract's `dfmm` (0xafba13c4) function
        pub fn dfmm(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([175, 186, 19, 196], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPoolParams` (0xdc178355) function
        pub fn get_pool_params(
            &self,
            pool_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([220, 23, 131, 85], pool_id)
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
        ///Calls the contract's `internalParams` (0x1edb71e5) function
        pub fn internal_params(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (DynamicParam, DynamicParam, DynamicParam, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([30, 219, 113, 229], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setSigma` (0x7251308c) function
        pub fn set_sigma(
            &self,
            pool_id: ::ethers::core::types::U256,
            target: ::ethers::core::types::U256,
            end: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([114, 81, 48, 140], (pool_id, target, end))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setStrikePrice` (0xd7109535) function
        pub fn set_strike_price(
            &self,
            pool_id: ::ethers::core::types::U256,
            target: ::ethers::core::types::U256,
            end: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([215, 16, 149, 53], (pool_id, target, end))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setTau` (0x3f43baac) function
        pub fn set_tau(
            &self,
            pool_id: ::ethers::core::types::U256,
            target: ::ethers::core::types::U256,
            end: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([63, 67, 186, 172], (pool_id, target, end))
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
    ///Container type for all input parameters for the `dfmm` function with signature `dfmm()` and selector `0xafba13c4`
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
    #[ethcall(name = "dfmm", abi = "dfmm()")]
    pub struct DfmmCall;
    ///Container type for all input parameters for the `getPoolParams` function with signature `getPoolParams(uint256)` and selector `0xdc178355`
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
    #[ethcall(name = "getPoolParams", abi = "getPoolParams(uint256)")]
    pub struct GetPoolParamsCall {
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
    ///Container type for all input parameters for the `internalParams` function with signature `internalParams(uint256)` and selector `0x1edb71e5`
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
    #[ethcall(name = "internalParams", abi = "internalParams(uint256)")]
    pub struct InternalParamsCall(pub ::ethers::core::types::U256);
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
        pub target: ::ethers::core::types::U256,
        pub end: ::ethers::core::types::U256,
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
        pub target: ::ethers::core::types::U256,
        pub end: ::ethers::core::types::U256,
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
        pub target: ::ethers::core::types::U256,
        pub end: ::ethers::core::types::U256,
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
        Dfmm(DfmmCall),
        GetPoolParams(GetPoolParamsCall),
        Init(InitCall),
        InternalParams(InternalParamsCall),
        SetSigma(SetSigmaCall),
        SetStrikePrice(SetStrikePriceCall),
        SetTau(SetTauCall),
        Update(UpdateCall),
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
            if let Ok(decoded) = <DfmmCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Dfmm(decoded));
            }
            if let Ok(decoded) = <GetPoolParamsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetPoolParams(decoded));
            }
            if let Ok(decoded) = <InitCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Init(decoded));
            }
            if let Ok(decoded) = <InternalParamsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InternalParams(decoded));
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
            if let Ok(decoded) = <UpdateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Update(decoded));
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
                Self::Dfmm(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetPoolParams(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Init(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InternalParams(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetSigma(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetStrikePrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetTau(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Update(element) => ::ethers::core::abi::AbiEncode::encode(element),
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
                Self::Dfmm(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPoolParams(element) => ::core::fmt::Display::fmt(element, f),
                Self::Init(element) => ::core::fmt::Display::fmt(element, f),
                Self::InternalParams(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetSigma(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetStrikePrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetTau(element) => ::core::fmt::Display::fmt(element, f),
                Self::Update(element) => ::core::fmt::Display::fmt(element, f),
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
    impl ::core::convert::From<DfmmCall> for LogNormalCalls {
        fn from(value: DfmmCall) -> Self {
            Self::Dfmm(value)
        }
    }
    impl ::core::convert::From<GetPoolParamsCall> for LogNormalCalls {
        fn from(value: GetPoolParamsCall) -> Self {
            Self::GetPoolParams(value)
        }
    }
    impl ::core::convert::From<InitCall> for LogNormalCalls {
        fn from(value: InitCall) -> Self {
            Self::Init(value)
        }
    }
    impl ::core::convert::From<InternalParamsCall> for LogNormalCalls {
        fn from(value: InternalParamsCall) -> Self {
            Self::InternalParams(value)
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
    impl ::core::convert::From<UpdateCall> for LogNormalCalls {
        fn from(value: UpdateCall) -> Self {
            Self::Update(value)
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
    ///Container type for all return fields from the `dfmm` function with signature `dfmm()` and selector `0xafba13c4`
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
    pub struct DfmmReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getPoolParams` function with signature `getPoolParams(uint256)` and selector `0xdc178355`
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
    pub struct GetPoolParamsReturn(pub ::ethers::core::types::Bytes);
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
        pub reserve_x: ::ethers::core::types::U256,
        pub reserve_y: ::ethers::core::types::U256,
        pub total_liquidity: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `internalParams` function with signature `internalParams(uint256)` and selector `0x1edb71e5`
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
    pub struct InternalParamsReturn {
        pub sigma: DynamicParam,
        pub tau: DynamicParam,
        pub strike: DynamicParam,
        pub swap_fee: ::ethers::core::types::U256,
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
        pub reserve_x: ::ethers::core::types::U256,
        pub reserve_y: ::ethers::core::types::U256,
        pub total_liquidity: ::ethers::core::types::U256,
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
