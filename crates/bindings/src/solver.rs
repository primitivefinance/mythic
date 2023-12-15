pub use solver::*;
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
pub mod solver {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_strategy"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("BISECTION_EPSILON"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("BISECTION_EPSILON"),
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
                    ::std::borrow::ToOwned::to_owned("MAX_BISECTION_ITERS"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "MAX_BISECTION_ITERS",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("computeNextLiquidity"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "computeNextLiquidity",
                            ),
                            inputs: ::std::vec![
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
                                    name: ::std::borrow::ToOwned::to_owned("swapConstant"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("currentLiquidity"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
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
                                        ::std::borrow::ToOwned::to_owned("struct Parameters"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("liquidity"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("computeNextReserveX"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "computeNextReserveX",
                            ),
                            inputs: ::std::vec![
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
                                    name: ::std::borrow::ToOwned::to_owned("liquidity"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("swapConstant"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
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
                                        ::std::borrow::ToOwned::to_owned("struct Parameters"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("reserveY"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("computeNextReserveY"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "computeNextReserveY",
                            ),
                            inputs: ::std::vec![
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
                                    name: ::std::borrow::ToOwned::to_owned("liquidity"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("swapConstant"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
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
                                        ::std::borrow::ToOwned::to_owned("struct Parameters"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("reserveY"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("encodeInitData"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("encodeInitData"),
                            inputs: ::std::vec![
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
                                    name: ::std::borrow::ToOwned::to_owned("params"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct Parameters"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getNextLiquidity"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getNextLiquidity"),
                            inputs: ::std::vec![
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
                    ::std::borrow::ToOwned::to_owned("getNextReserveX"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getNextReserveX"),
                            inputs: ::std::vec![
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
                    ::std::borrow::ToOwned::to_owned("getNextReserveY"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getNextReserveY"),
                            inputs: ::std::vec![
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
                                    name: ::std::borrow::ToOwned::to_owned("totalLiquidity"),
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
                    ::std::borrow::ToOwned::to_owned("internalPrice"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("internalPrice"),
                            inputs: ::std::vec![
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
                                    name: ::std::borrow::ToOwned::to_owned("totalLiquidity"),
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
                                    name: ::std::borrow::ToOwned::to_owned("price"),
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
                    ::std::borrow::ToOwned::to_owned("simulateSwap"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("simulateSwap"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("swapXIn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountIn"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
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
                    ::std::borrow::ToOwned::to_owned("strategy"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("strategy"),
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
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("BisectionLib_InvalidBounds"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "BisectionLib_InvalidBounds",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("lower"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("upper"),
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
                    ::std::borrow::ToOwned::to_owned("BisectionLib_RootOutsideBounds"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "BisectionLib_RootOutsideBounds",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("lowerResult"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("upperResult"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
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
    pub static SOLVER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15b\0\0^W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`@Qb\0&\x808\x03\x80b\0&\x80\x839\x81\x01`@\x81\x90Rb\0\0\x81\x91b\0\0\xA7V[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90Ub\0\x01$V[`\0` \x82\x84\x03\x12\x15b\0\x01\x05W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x01\x1DW`\0\x80\xFD[\x93\x92PPPV[a%L\x80b\0\x014`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x01\x01W`\x005`\xE0\x1C\x80cme\"\x99\x11a\0\xBEW\x80cme\"\x99\x14a\x02\x0CW\x80c\xA8\xC6.v\x14a\x02\x14W\x80c\xB8\xF0\x0Br\x14a\x02?W\x80c\xEC)\xD8\xE6\x14a\x02RW\x80c\xF4+\xFF|\x14a\x02eW\x80c\xF9\xC2\x82\x11\x14a\x02xWa\x01\x01V[\x80c\n\xC304\x14a\x01fW\x80c\n\xE1\xC5\xA5\x14a\x01\x92W\x80c\x0F\n\xA3\x95\x14a\x01\xB3W\x80c\x1F\xCA8\xDB\x14a\x01\xD3W\x80cH\xA5\xF1\x9D\x14a\x01\xE6W\x80ci\xA4\xE4\x88\x14a\x01\xF9W[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x01ya\x01t6`\x04a \"V[a\x02\x80V[`@Qa\x01\x89\x94\x93\x92\x91\x90a \x97V[`@Q\x80\x91\x03\x90\xF3[a\x01\xA5a\x01\xA06`\x04a \xBEV[a\x08zV[`@Q\x90\x81R` \x01a\x01\x89V[a\x01\xC6a\x01\xC16`\x04a!\xA8V[a\n,V[`@Qa\x01\x89\x91\x90a!\xEBV[a\x01\xA5a\x01\xE16`\x04a \xBEV[a\n^V[a\x01\xA5a\x01\xF46`\x04a!\xA8V[a\x0C\x06V[a\x01\xA5a\x02\x076`\x04a!\xFEV[a\x0CYV[a\x01\xA5`\x01\x81V[`\0Ta\x02'\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\x89V[a\x01\xA5a\x02M6`\x04a \xBEV[a\rtV[a\x01\xA5a\x02`6`\x04a\"KV[a\x0EJV[a\x01\xA5a\x02s6`\x04a!\xA8V[a\x0F\xFCV[a\x01\xA5`Z\x81V[`\0\x80`\0``a\x02\xAB`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[a\x02\xCF`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`\0\x80T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEB\xAD\xEF\x01`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01```@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x03YW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a%,\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x03mW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\x91\x91\x90a\"zV[`@\x80\x86\x01\x91\x90\x91R` \x85\x01\x91\x90\x91R\x90\x83R`\0\x80T\x82Qc\xC1\xE0\x04;`\xE0\x1B\x81R\x92Q\x91\x92`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91c\xC1\xE0\x04;\x91`\x04\x80\x82\x01\x92``\x92\x90\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\x04)W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a%,\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x04=W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04a\x91\x90a\"\xE8V[\x90P`\0\x80`\0\x80T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c@\xB41i`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x04\xF0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a%,\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x05\x04W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05(\x91\x90a#\x07V[\x90P`\0a\x05C\x86`\0\x01Q\x87` \x01Q\x88`@\x01Qa\x0EJV[\x90P\x8B\x15a\x06PW`\0a\x05W\x8C\x84a\x10VV[\x87Q\x90\x91P`\0\x90a\x05s\x90a\x05m\x84\x86a\x10VV[\x90a\x10rV[\x90Pa\x05\x80`\x01\x82a#9V[\x88Q\x90\x91Pa\x05\x90\x90\x8E\x90a#9V[\x87R`@\x88\x01Qa\x05\xA2\x90\x82\x90a#9V[`@\x88\x01\x81\x90R\x87Qa\x05\xB4\x91a\x08zV[` \x88\x01\x81\x81R`\x01\x91a\x05\xC9\x90\x83\x90a#9V[\x90RP` \x80\x89\x01Q\x90\x88\x01Q\x10a\x063W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: y reserve increase`D\x82\x01Rad!`\xF0\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[\x86` \x01Q\x88` \x01Qa\x06G\x91\x90a#LV[\x94PPPa\x07>V[`\0a\x06\\\x8C\x84a\x10VV[\x87Q\x90\x91P`\0\x90a\x06r\x90a\x05m\x84\x86a\x10VV[\x90Pa\x06\x7F`\x01\x82a#9V[\x88Q\x90\x91Pa\x06\x8F\x90\x8E\x90a#9V[` \x88\x01R`@\x88\x01Qa\x06\xA4\x90\x82\x90a#9V[`@\x88\x01\x81\x90R` \x88\x01Qa\x06\xB9\x91a\n^V[\x80\x88R`\x01\x90\x88\x90a\x06\xCC\x90\x83\x90a#9V[\x90RP\x87Q\x87Q\x10a\x07+W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: x reserve increase`D\x82\x01Rad!`\xF0\x1B`d\x82\x01R`\x84\x01a\x06*V[\x86Q\x88Qa\x079\x91\x90a#LV[\x94PPP[PP\x82Q` \x80\x85\x01Q`@\x80\x87\x01Q\x81Q\x93\x84\x01\x94\x90\x94R\x82\x01R``\x81\x01\x91\x90\x91R`\0\x90`\x80\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R`\0\x80Tc\xC1nP\xEF`\xE0\x1B\x84R\x91\x93P\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xC1nP\xEF\x90a\x07\xAB\x90\x85\x90`\x04\x01a!\xEBV[`\xC0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x08\x01W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a%,\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x08\x15W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x089\x91\x90a#_V[PPPPP\x90P\x80\x83a\x08c\x87`\0\x01Q\x88`@\x01Q\x88`\0\x01Q\x89` \x01Q\x8A`@\x01Qa\x10\x87V[\x91\x9E\x90\x9DP\x90\x9BP\x91\x99P\x90\x97PPPPPPPPV[`\0\x80T`@\x80Q` \x81\x01\x86\x90R\x80\x82\x01\x85\x90R\x81Q\x80\x82\x03\x83\x01\x81R``\x82\x01\x92\x83\x90Rc\xA1\x9C\xD3\xD1`\xE0\x1B\x90\x92R\x83\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\xA1\x9C\xD3\xD1\x91a\x08\xCA\x91\x90`d\x01a!\xEBV[` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\t W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a%,\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\t4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\tX\x91\x90a#\x07V[\x90Pa\n\"\x84\x84\x83`\0\x80T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xC1\xE0\x04;`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01```@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\t\xEAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a%,\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\t\xFEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02s\x91\x90a\"\xE8V[\x91PP[\x92\x91PPV[``\x84\x84\x84\x84`@Q` \x01a\nE\x94\x93\x92\x91\x90a#\xB5V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x94\x93PPPPV[`\0\x80T`@\x80Q` \x81\x01\x86\x90R\x80\x82\x01\x85\x90R\x81Q\x80\x82\x03\x83\x01\x81R``\x82\x01\x92\x83\x90Rc\xA1\x9C\xD3\xD1`\xE0\x1B\x90\x92R\x83\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\xA1\x9C\xD3\xD1\x91a\n\xAE\x91\x90`d\x01a!\xEBV[` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0B\x04W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a%,\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0B\x18W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B<\x91\x90a#\x07V[\x90Pa\n\"\x84\x84\x83`\0\x80T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xC1\xE0\x04;`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01```@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0B\xCEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a%,\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0B\xE2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xF4\x91\x90a\"\xE8V[`\0`\n\x81a\x0C\x15\x82\x87a#LV[\x90Pa\x0CN\x87\x87\x87\x87`@Q` \x01a\x0C1\x94\x93\x92\x91\x90a#\xB5V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83\x83`\x01`Za\x11qa\x11\xACV[\x97\x96PPPPPPPV[`\0\x80`\0\x80`\0a\x0Cx\x86`\0\x01Q\x8Aa\x12\xBD\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x86\x15a\x0C\xF6W`\x14\x88\x12\x80\x15a\x0C\x91WP`\x13\x19\x88\x13[\x15a\x0C\xA2W\x86\x94PPPPPa\rkV[`\0\x88\x12\x15a\x0C\xDBW\x86\x92P\x80\x8A\x11a\x0C\xC5Wa\x0C\xC0\x81`\x01a#9V[a\x0C\xD0V[a\x0C\xD0\x8A`\x01a#9V[\x93P`\x96\x91Pa\r.V[k\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x92P\x86\x93P`\x96\x91Pa\r.V[k\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x92P\x80\x8A\x11a\r\x1CWa\r\x17\x81`\x01a#9V[a\r'V[a\r'\x8A`\x01a#9V[\x93P`\x96\x91P[a\rd\x8A\x8A\x8A\x89`@Q` \x01a\rH\x94\x93\x92\x91\x90a#\xB5V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x85\x85`\x01\x86a\x12\xD2a\x11\xACV[\x94PPPPP[\x95\x94PPPPPV[`\0\x80T`@\x80Qc\xC1\xE0\x04;`\xE0\x1B\x81R\x90Q\x83\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\xC1\xE0\x04;\x91`\x04\x80\x83\x01\x92``\x92\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\r\xF7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a%,\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0E\x0BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E/\x91\x90a\"\xE8V[\x90Pa\n\"\x84\x84\x83`\0\x01Q\x84` \x01Q\x85`@\x01Qa\x10\x87V[`\0\x80T`@\x80Q` \x81\x01\x87\x90R\x80\x82\x01\x86\x90R``\x80\x82\x01\x86\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x80\x82\x01\x92\x83\x90Rc\xA1\x9C\xD3\xD1`\xE0\x1B\x90\x92R\x83\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\xA1\x9C\xD3\xD1\x91a\x0E\xA3\x91\x90`\x84\x01a!\xEBV[` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0E\xF9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a%,\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0F\rW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F1\x91\x90a#\x07V[\x90Pa\rk\x85\x85\x83\x86`\0\x80T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xC1\xE0\x04;`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01```@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0F\xC4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a%,\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0F\xD8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\x07\x91\x90a\"\xE8V[\x80Q`\0\x90`\n\x90\x82\x90\x82\x90a\x10\x13\x90\x88\x90a\x10VV[a\x10\x1D\x91\x90a#LV[\x90Pa\x0CN\x87\x87\x87\x87`@Q` \x01a\x109\x94\x93\x92\x91\x90a#\xB5V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83\x83`\x01`Za\x13\ta\x11\xACV[`\0a\x10k\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x13:V[\x93\x92PPPV[`\0a\x10k\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x13:V[`\0\x80a\x10\x94\x84\x84a\x13hV[\x90P`\0a\x10\xA1\x85a\x13\x92V[\x90P`\0a\x10\xAF\x82\x86a\x13\xB7V[\x90P`\0a\x10\xBD\x8A\x8Aa\x12\xBDV[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0\x81\x12a\x10\xDBW`\0\x94PPPPPa\rkV[`\0\x81\x13a\x10\xF1W`\0\x19\x94PPPPPa\rkV[`\0a\x11\ra\x11\x08\x83g\r\xE0\xB6\xB3\xA7d\0\0a#\xE9V[a\x13\xCCV[\x90P`\0\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x11%\x88\x85a$\x10V[a\x11/\x91\x90a$VV[a\x119\x91\x90a#\xE9V[\x90P`\0a\x11F\x82a\x14iV[\x90P`\0a\x11S\x82a\x16\x12V[\x90Pa\x11_\x8C\x82a\x10VV[\x9E\x9DPPPPPPPPPPPPPPV[`\0\x80`\0\x80`\0\x86\x80` \x01\x90Q\x81\x01\x90a\x11\x8D\x91\x90a$\x84V[\x93P\x93P\x93P\x93P\x81a\x11\xA2\x87\x86\x86\x85a\x16[V[a\x0CN\x91\x90a#\xE9V[`\0\x84\x86\x11\x15a\x11\xD9W`@Qc0\x82\xDF\xDB`\xE1\x1B\x81R`\x04\x81\x01\x87\x90R`$\x81\x01\x86\x90R`D\x01a\x06*V[`\0a\x11\xE9\x88\x88\x85c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x11\xFB\x89\x88\x86c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x12\t\x82\x84a$\x10V[\x13\x15a\x122W`@Qc\x06\xF1\xBE]`\xE2\x1B\x81R`\x04\x81\x01\x83\x90R`$\x81\x01\x82\x90R`D\x01a\x06*V[`\0a\x12>\x89\x89a#LV[\x90P`\0[`\x02a\x12O\x8A\x8Ca#9V[a\x12Y\x91\x90a$\xBCV[\x94P`\0a\x12k\x8C\x87\x89c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x12y\x86\x83a$\x10V[\x13a\x12\x86W\x85\x99Pa\x12\x8DV[\x85\x9AP\x80\x94P[a\x12\x97\x8B\x8Ba#LV[\x92PP`\x01\x01\x87\x82\x11\x80\x15a\x12\xABWP\x86\x81\x10[a\x12CWPPPP\x96\x95PPPPPPV[`\0a\x10k\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x17hV[`\0\x80`\0\x80\x85\x80` \x01\x90Q\x81\x01\x90a\x12\xEC\x91\x90a$\x84V[\x93PP\x92P\x92Pa\x12\xFF\x83\x83\x87\x84a\x16[V[\x96\x95PPPPPPV[`\0\x80`\0\x80`\0\x86\x80` \x01\x90Q\x81\x01\x90a\x13%\x91\x90a$\x84V[\x93P\x93P\x93P\x93P\x81a\x11\xA2\x85\x88\x86\x85a\x16[V[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x13RW`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[`\0\x80a\x13t\x83a\x17\x87V[\x90P`\0a\x13\x86c;\x9A\xCA\0\x83a$\xD0V[\x90Pa\rk\x85\x82a\x13\xB7V[`\0\x80a\x13\xA7\x83g\x1B\xC1mgN\xC8\0\0a\x18+V[\x90Pa\x10kg\x06\xF0[Y\xD3\xB2\0\0\x82[`\0a\x10k\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x17hV[`\0g\x06\xF0[Y\xD3\xB2\0\0\x82\x03a\x13\xE5WP`\0\x91\x90PV[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x12a\x14\rW`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\x14.W`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x14;\x83`\x02a$\x10V[\x90P`\0a\x14H\x82a\x18\\V[\x90P`\0a\x14^g\x13\xA0K\xBD\xFD\xC9\xBE\x88\x83a\x1A\xDAV[\x90Pa\rk\x81a$\xE7V[`\0h\x02H\xCE6\xA7\x0C\xB2k>\x19\x82\x13a\x14\x84WP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a\x14\xCBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x01a\x06*V[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05`\x01`_\x1B\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1Dm\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1Dn\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[`\0\x80\x82\x12\x15a\x16WW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01RotoUint: negative`\x80\x1B`D\x82\x01R`d\x01a\x06*V[P\x90V[`\0\x82\x85\x10a\x16\xACW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FtradingFunction: invalid x\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06*V[`\0\x80g\r\xE0\xB6\xB3\xA7d\0\0a\x16\xC2\x88\x87a\x12\xBDV[\x10a\x16\xD6W`\x01`\x01`\xFF\x1B\x03\x91Pa\x16\xE6V[a\x16\xE3a\x11\x08\x88\x87a\x12\xBDV[\x91P[g\r\xE0\xB6\xB3\xA7d\0\0a\x17\x06\x87a\x17\x01\x87`\0\x01Q\x89a\x13\xB7V[a\x12\xBDV[\x10a\x17\x19WP`\x01`\x01`\xFF\x1B\x03a\x171V[a\x17.a\x11\x08\x87a\x17\x01\x87`\0\x01Q\x89a\x13\xB7V[\x90P[`\0a\x17E\x85` \x01Q\x86`@\x01Qa\x13hV[\x90P\x80a\x17R\x83\x85a%\x03V[a\x17\\\x91\x90a%\x03V[\x98\x97PPPPPPPPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x17\x80W`\0\x80\xFD[\x04\x92\x91PPV[`\xB5\x81`\x01`\x88\x1B\x81\x10a\x17\xA0W`@\x91\x90\x91\x1B\x90`\x80\x1C[i\x01\0\0\0\0\0\0\0\0\0\x81\x10a\x17\xBCW` \x91\x90\x91\x1B\x90`@\x1C[e\x01\0\0\0\0\0\x81\x10a\x17\xD4W`\x10\x91\x90\x91\x1B\x90` \x1C[c\x01\0\0\0\x81\x10a\x17\xEAW`\x08\x91\x90\x91\x1B\x90`\x10\x1C[b\x01\0\0\x01\x02`\x12\x1C\x80\x82\x04\x01`\x01\x90\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x90\x81\x90\x04\x81\x11\x90\x03\x90V[`\0a\x10kg\r\xE0\xB6\xB3\xA7d\0\0\x83a\x18C\x86a\x1A\xEFV[a\x18M\x91\x90a$\x10V[a\x18W\x91\x90a$VV[a\x14iV[`\0\x80\x82\x12\x80a\x18sWPg\x1B\xC1mgN\xC8\0\0\x82\x13[\x15a\x18\x91W`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\x18\xB2W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81g\x1B\xC1mgN\xC8\0\0\x03a\x18\xDAW`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x15a\x18\xE5W\x91\x90PV[`\0g\r\xE0\xB6\xB3\xA7d\0\0\x83\x12a\x19\rWa\x19\x08\x83g\x1B\xC1mgN\xC8\0\0a#\xE9V[a\x19\x0FV[\x82[\x90P`\0a\x19%\x82g\x1B\xC1mgN\xC8\0\0a\x1C\xCAV[\x90P\x80`\0\x03a\x19HW`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x19S\x82a\x1A\xEFV[\x90P`\0c;\x9A\xCA\0a\x19~a\x19ya\x19sg\x1B\xC1mgN\xC8\0\0a$\xE7V[\x85a\x1A\xDAV[a\x17\x87V[a\x19\x88\x91\x90a$\x10V[\x90P`\0\x80a\x19\x9F\x83g\x03\xC1f\\z\xAB \0a\x1A\xDAV[a\x19\xB1\x90g \x05\xFEO&\x8E\xA0\0a%\x03V[\x90P`\0a\x19\xE1\x84a\x19\xCA\x86f\x9F2u$b\xA0\0a\x1A\xDAV[a\x19\xDC\x90g\r\xC5R\x7Fd, \0a%\x03V[a\x1A\xDAV[a\x19\xF3\x90g\r\xE0\xB6\xB3\xA7d\0\0a%\x03V[\x90Pa\x1A\x17g\t\xD0(\xCCo _\xFF\x19\x85a\x1A\r\x85\x85a\x1C\xCAV[a\x19\xDC\x91\x90a#\xE9V[\x92PPP`\0[`\x02\x81\x10\x15a\x1A\xB2W`\0\x86a\x1A3\x84a\x1C\xDFV[a\x1A=\x91\x90a#\xE9V[\x90P`\0a\x1AK\x84\x85a\x1A\xDAV[a\x1AT\x90a$\xE7V[\x90P`\0a\x1Aa\x82a\x14iV[\x90P`\0a\x1Ao\x86\x85a\x1A\xDAV[a\x1A\x81g\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x84a\x1A\xDAV[a\x1A\x8B\x91\x90a#\xE9V[\x90Pa\x1A\x97\x84\x82a\x1C\xCAV[a\x1A\xA1\x90\x87a%\x03V[\x95P\x84`\x01\x01\x94PPPPPa\x1A\x1EV[g\r\xE0\xB6\xB3\xA7d\0\0\x88\x12a\x1A\xCFWa\x1A\xCA\x82a$\xE7V[a\x17\\V[P\x96\x95PPPPPPV[`\0a\x10k\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x1E\xC3V[`\0\x80\x82\x13a\x1B,W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x06*V[`\0``a\x1B9\x84a\x1E\xE2V[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[`\0a\x10k\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x1E\xC3V[`\0\x81`\0\x03a\x1C\xF8WPg\r\xE0\xB6\xB3\xA7d\0\0\x91\x90PV[gV\x98\xEE\xF0fp\0\0\x82\x12a\x1D\x0FWP`\0\x91\x90PV[a\x1D gV\x98\xEE\xF0fp\0\0a$\xE7V[\x82\x13a\x1D5WPg\x1B\xC1mgN\xC8\0\0\x91\x90PV[`\0a\x1D@\x83a\x1F\x8AV[\x90P`\0a\x1Dyg\r\xE0\xB6\xB3\xA7d\0\0a\x1Db\x84g\x1B\xC1mgN\xC8\0\0a\x12\xBDV[a\x1Dt\x90g\r\xE0\xB6\xB3\xA7d\0\0a%\x03V[a\x1C\xCAV[\x90P`\0\x80\x82a\x1D\xD5\x81a\x1D\xC2\x81a\x1D\xB0\x81a\x1D\x9D\x81g\x02_\x0F\xE1\x05\xA3\x14\0a\x1A\xDAV[a\x19\xDC\x90g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19a%\x03V[a\x19\xDC\x90g\x14\xA8EL\x19\xE1\xAC\0a%\x03V[a\x19\xDC\x90g\x0F\xC1\x0E\x01W\x82w\xFF\x19a%\x03V[a\x1D\xE7\x90g\x03\xDE\xBD\x08;\x8C|\0a%\x03V[\x91P\x83\x90Pa\x1EO\x81a\x1E=\x81a\x1E+\x81a\x1E\x19\x81a\x1E\x06\x81\x8Ba\x1A\xDAV[a\x19\xDC\x90g\x02\x95\xD4\0\xEA2W\xFF\x19a%\x03V[a\x19\xDC\x90g\x01W\xD8\xB2\xEC\xC7\x08\0a%\x03V[a\x19\xDC\x90g\x051\n\xA7\xD5!0\0a%\x03V[a\x19\xDC\x90g\r\xE0\xCC=\x15a\0\0a%\x03V[\x91P\x81g\x11\x90\0\xAB\x10\x0F\xFC\0a\x1Ee\x87\x88a\x1A\xDAV[a\x1Eq\x90`\0\x19a$\x10V[a\x1E{\x91\x90a#\xE9V[a\x1E\x85\x91\x90a%\x03V[\x92PP`\0a\x1E\x93\x83a\x14iV[\x90P`\0a\x1E\xA1\x85\x83a\x1A\xDAV[\x90P`\0\x88\x12a\x1E\xB1W\x80a\x17\\V[a\x17\\\x81g\x1B\xC1mgN\xC8\0\0a#\xE9V[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x05\x85\x14\x17\x16a\x1E\xDBW`\0\x80\xFD[\x05\x92\x91PPV[`\0\x80\x82\x11a\x1F\x1FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x06*V[P`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x82\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[`\0`\x01`\xFF\x1B\x82\x03a\x1F\xB0W`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x12\x15a\x16WWP\x19`\x01\x01\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[\x80\x15\x15\x81\x14a \x1FW`\0\x80\xFD[PV[`\0\x80`@\x83\x85\x03\x12\x15a 8Wa 8a\x1F\xC1V[\x825a C\x81a \x11V[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a wW` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a [V[P`\0` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[\x84\x15\x15\x81R\x83` \x82\x01R\x82`@\x82\x01R`\x80``\x82\x01R`\0a\x12\xFF`\x80\x83\x01\x84a QV[`\0\x80`@\x83\x85\x03\x12\x15a \xD4Wa \xD4a\x1F\xC1V[PP\x805\x92` \x90\x91\x015\x91PV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FABI decoding: struct data too sh`D\x82\x01Rb\x1B\xDC\x9D`\xEA\x1B`d\x82\x01R`\x84\x81\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a!eWcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@R\x90V[`\0``\x82\x84\x03\x12\x15a!\x80Wa!\x80a \xE3V[a!\x88a!4V[\x90P\x815\x81R` \x82\x015` \x82\x01R`@\x82\x015`@\x82\x01R\x92\x91PPV[`\0\x80`\0\x80`\xC0\x85\x87\x03\x12\x15a!\xC1Wa!\xC1a\x1F\xC1V[\x845\x93P` \x85\x015\x92P`@\x85\x015\x91Pa!\xE0\x86``\x87\x01a!kV[\x90P\x92\x95\x91\x94P\x92PV[` \x81R`\0a\x10k` \x83\x01\x84a QV[`\0\x80`\0\x80`\0`\xE0\x86\x88\x03\x12\x15a\"\x19Wa\"\x19a\x1F\xC1V[\x855\x94P` \x86\x015\x93P`@\x86\x015\x92P``\x86\x015\x91Pa\"?\x87`\x80\x88\x01a!kV[\x90P\x92\x95P\x92\x95\x90\x93PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\"cWa\"ca\x1F\xC1V[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\"\x92Wa\"\x92a\x1F\xC1V[\x83Q\x92P` \x84\x01Q\x91P`@\x84\x01Q\x90P\x92P\x92P\x92V[`\0``\x82\x84\x03\x12\x15a\"\xC0Wa\"\xC0a \xE3V[a\"\xC8a!4V[\x90P\x81Q\x81R` \x82\x01Q` \x82\x01R`@\x82\x01Q`@\x82\x01R\x92\x91PPV[`\0``\x82\x84\x03\x12\x15a\"\xFDWa\"\xFDa\x1F\xC1V[a\x10k\x83\x83a\"\xABV[`\0` \x82\x84\x03\x12\x15a#\x1CWa#\x1Ca\x1F\xC1V[PQ\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\n&Wa\n&a##V[\x81\x81\x03\x81\x81\x11\x15a\n&Wa\n&a##V[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a#{Wa#{a\x1F\xC1V[\x86Qa#\x86\x81a \x11V[` \x88\x01Q`@\x89\x01Q``\x8A\x01Q`\x80\x8B\x01Q`\xA0\x90\x9B\x01Q\x93\x9C\x92\x9BP\x90\x99\x90\x98P\x96P\x90\x94P\x92PPPV[\x84\x81R` \x80\x82\x01\x85\x90R`@\x80\x83\x01\x85\x90R\x83Q``\x84\x01R\x90\x83\x01Q`\x80\x83\x01R\x82\x01Q`\xA0\x82\x01R`\xC0\x81\x01a\rkV[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a$\tWa$\ta##V[P\x92\x91PPV[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15a$,Wa$,a##V[\x81\x81\x05\x83\x14\x82\x15\x17a\n&Wa\n&a##V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a$eWa$ea$@V[`\x01`\xFF\x1B\x82\x14`\0\x19\x84\x14\x16\x15a$\x7FWa$\x7Fa##V[P\x05\x90V[`\0\x80`\0\x80`\xC0\x85\x87\x03\x12\x15a$\x9DWa$\x9Da\x1F\xC1V[\x84Q\x93P` \x85\x01Q\x92P`@\x85\x01Q\x91Pa!\xE0\x86``\x87\x01a\"\xABV[`\0\x82a$\xCBWa$\xCBa$@V[P\x04\x90V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\n&Wa\n&a##V[`\0`\x01`\xFF\x1B\x82\x01a$\xFCWa$\xFCa##V[P`\0\x03\x90V[\x80\x82\x01\x82\x81\x12`\0\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a%#Wa%#a##V[PP\x92\x91PPV\xFETarget contract does not contain";
    /// The bytecode of the contract.
    pub static SOLVER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x01\x01W`\x005`\xE0\x1C\x80cme\"\x99\x11a\0\xBEW\x80cme\"\x99\x14a\x02\x0CW\x80c\xA8\xC6.v\x14a\x02\x14W\x80c\xB8\xF0\x0Br\x14a\x02?W\x80c\xEC)\xD8\xE6\x14a\x02RW\x80c\xF4+\xFF|\x14a\x02eW\x80c\xF9\xC2\x82\x11\x14a\x02xWa\x01\x01V[\x80c\n\xC304\x14a\x01fW\x80c\n\xE1\xC5\xA5\x14a\x01\x92W\x80c\x0F\n\xA3\x95\x14a\x01\xB3W\x80c\x1F\xCA8\xDB\x14a\x01\xD3W\x80cH\xA5\xF1\x9D\x14a\x01\xE6W\x80ci\xA4\xE4\x88\x14a\x01\xF9W[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x01ya\x01t6`\x04a \"V[a\x02\x80V[`@Qa\x01\x89\x94\x93\x92\x91\x90a \x97V[`@Q\x80\x91\x03\x90\xF3[a\x01\xA5a\x01\xA06`\x04a \xBEV[a\x08zV[`@Q\x90\x81R` \x01a\x01\x89V[a\x01\xC6a\x01\xC16`\x04a!\xA8V[a\n,V[`@Qa\x01\x89\x91\x90a!\xEBV[a\x01\xA5a\x01\xE16`\x04a \xBEV[a\n^V[a\x01\xA5a\x01\xF46`\x04a!\xA8V[a\x0C\x06V[a\x01\xA5a\x02\x076`\x04a!\xFEV[a\x0CYV[a\x01\xA5`\x01\x81V[`\0Ta\x02'\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\x89V[a\x01\xA5a\x02M6`\x04a \xBEV[a\rtV[a\x01\xA5a\x02`6`\x04a\"KV[a\x0EJV[a\x01\xA5a\x02s6`\x04a!\xA8V[a\x0F\xFCV[a\x01\xA5`Z\x81V[`\0\x80`\0``a\x02\xAB`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[a\x02\xCF`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`\0\x80T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEB\xAD\xEF\x01`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01```@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x03YW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a%,\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x03mW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\x91\x91\x90a\"zV[`@\x80\x86\x01\x91\x90\x91R` \x85\x01\x91\x90\x91R\x90\x83R`\0\x80T\x82Qc\xC1\xE0\x04;`\xE0\x1B\x81R\x92Q\x91\x92`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91c\xC1\xE0\x04;\x91`\x04\x80\x82\x01\x92``\x92\x90\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\x04)W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a%,\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x04=W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04a\x91\x90a\"\xE8V[\x90P`\0\x80`\0\x80T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c@\xB41i`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x04\xF0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a%,\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x05\x04W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05(\x91\x90a#\x07V[\x90P`\0a\x05C\x86`\0\x01Q\x87` \x01Q\x88`@\x01Qa\x0EJV[\x90P\x8B\x15a\x06PW`\0a\x05W\x8C\x84a\x10VV[\x87Q\x90\x91P`\0\x90a\x05s\x90a\x05m\x84\x86a\x10VV[\x90a\x10rV[\x90Pa\x05\x80`\x01\x82a#9V[\x88Q\x90\x91Pa\x05\x90\x90\x8E\x90a#9V[\x87R`@\x88\x01Qa\x05\xA2\x90\x82\x90a#9V[`@\x88\x01\x81\x90R\x87Qa\x05\xB4\x91a\x08zV[` \x88\x01\x81\x81R`\x01\x91a\x05\xC9\x90\x83\x90a#9V[\x90RP` \x80\x89\x01Q\x90\x88\x01Q\x10a\x063W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: y reserve increase`D\x82\x01Rad!`\xF0\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[\x86` \x01Q\x88` \x01Qa\x06G\x91\x90a#LV[\x94PPPa\x07>V[`\0a\x06\\\x8C\x84a\x10VV[\x87Q\x90\x91P`\0\x90a\x06r\x90a\x05m\x84\x86a\x10VV[\x90Pa\x06\x7F`\x01\x82a#9V[\x88Q\x90\x91Pa\x06\x8F\x90\x8E\x90a#9V[` \x88\x01R`@\x88\x01Qa\x06\xA4\x90\x82\x90a#9V[`@\x88\x01\x81\x90R` \x88\x01Qa\x06\xB9\x91a\n^V[\x80\x88R`\x01\x90\x88\x90a\x06\xCC\x90\x83\x90a#9V[\x90RP\x87Q\x87Q\x10a\x07+W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: x reserve increase`D\x82\x01Rad!`\xF0\x1B`d\x82\x01R`\x84\x01a\x06*V[\x86Q\x88Qa\x079\x91\x90a#LV[\x94PPP[PP\x82Q` \x80\x85\x01Q`@\x80\x87\x01Q\x81Q\x93\x84\x01\x94\x90\x94R\x82\x01R``\x81\x01\x91\x90\x91R`\0\x90`\x80\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R`\0\x80Tc\xC1nP\xEF`\xE0\x1B\x84R\x91\x93P\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xC1nP\xEF\x90a\x07\xAB\x90\x85\x90`\x04\x01a!\xEBV[`\xC0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x08\x01W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a%,\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x08\x15W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x089\x91\x90a#_V[PPPPP\x90P\x80\x83a\x08c\x87`\0\x01Q\x88`@\x01Q\x88`\0\x01Q\x89` \x01Q\x8A`@\x01Qa\x10\x87V[\x91\x9E\x90\x9DP\x90\x9BP\x91\x99P\x90\x97PPPPPPPPV[`\0\x80T`@\x80Q` \x81\x01\x86\x90R\x80\x82\x01\x85\x90R\x81Q\x80\x82\x03\x83\x01\x81R``\x82\x01\x92\x83\x90Rc\xA1\x9C\xD3\xD1`\xE0\x1B\x90\x92R\x83\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\xA1\x9C\xD3\xD1\x91a\x08\xCA\x91\x90`d\x01a!\xEBV[` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\t W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a%,\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\t4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\tX\x91\x90a#\x07V[\x90Pa\n\"\x84\x84\x83`\0\x80T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xC1\xE0\x04;`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01```@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\t\xEAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a%,\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\t\xFEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02s\x91\x90a\"\xE8V[\x91PP[\x92\x91PPV[``\x84\x84\x84\x84`@Q` \x01a\nE\x94\x93\x92\x91\x90a#\xB5V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x94\x93PPPPV[`\0\x80T`@\x80Q` \x81\x01\x86\x90R\x80\x82\x01\x85\x90R\x81Q\x80\x82\x03\x83\x01\x81R``\x82\x01\x92\x83\x90Rc\xA1\x9C\xD3\xD1`\xE0\x1B\x90\x92R\x83\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\xA1\x9C\xD3\xD1\x91a\n\xAE\x91\x90`d\x01a!\xEBV[` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0B\x04W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a%,\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0B\x18W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B<\x91\x90a#\x07V[\x90Pa\n\"\x84\x84\x83`\0\x80T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xC1\xE0\x04;`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01```@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0B\xCEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a%,\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0B\xE2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xF4\x91\x90a\"\xE8V[`\0`\n\x81a\x0C\x15\x82\x87a#LV[\x90Pa\x0CN\x87\x87\x87\x87`@Q` \x01a\x0C1\x94\x93\x92\x91\x90a#\xB5V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83\x83`\x01`Za\x11qa\x11\xACV[\x97\x96PPPPPPPV[`\0\x80`\0\x80`\0a\x0Cx\x86`\0\x01Q\x8Aa\x12\xBD\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x86\x15a\x0C\xF6W`\x14\x88\x12\x80\x15a\x0C\x91WP`\x13\x19\x88\x13[\x15a\x0C\xA2W\x86\x94PPPPPa\rkV[`\0\x88\x12\x15a\x0C\xDBW\x86\x92P\x80\x8A\x11a\x0C\xC5Wa\x0C\xC0\x81`\x01a#9V[a\x0C\xD0V[a\x0C\xD0\x8A`\x01a#9V[\x93P`\x96\x91Pa\r.V[k\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x92P\x86\x93P`\x96\x91Pa\r.V[k\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x92P\x80\x8A\x11a\r\x1CWa\r\x17\x81`\x01a#9V[a\r'V[a\r'\x8A`\x01a#9V[\x93P`\x96\x91P[a\rd\x8A\x8A\x8A\x89`@Q` \x01a\rH\x94\x93\x92\x91\x90a#\xB5V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x85\x85`\x01\x86a\x12\xD2a\x11\xACV[\x94PPPPP[\x95\x94PPPPPV[`\0\x80T`@\x80Qc\xC1\xE0\x04;`\xE0\x1B\x81R\x90Q\x83\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\xC1\xE0\x04;\x91`\x04\x80\x83\x01\x92``\x92\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\r\xF7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a%,\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0E\x0BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E/\x91\x90a\"\xE8V[\x90Pa\n\"\x84\x84\x83`\0\x01Q\x84` \x01Q\x85`@\x01Qa\x10\x87V[`\0\x80T`@\x80Q` \x81\x01\x87\x90R\x80\x82\x01\x86\x90R``\x80\x82\x01\x86\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x80\x82\x01\x92\x83\x90Rc\xA1\x9C\xD3\xD1`\xE0\x1B\x90\x92R\x83\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\xA1\x9C\xD3\xD1\x91a\x0E\xA3\x91\x90`\x84\x01a!\xEBV[` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0E\xF9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a%,\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0F\rW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F1\x91\x90a#\x07V[\x90Pa\rk\x85\x85\x83\x86`\0\x80T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xC1\xE0\x04;`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01```@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0F\xC4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a%,\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0F\xD8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\x07\x91\x90a\"\xE8V[\x80Q`\0\x90`\n\x90\x82\x90\x82\x90a\x10\x13\x90\x88\x90a\x10VV[a\x10\x1D\x91\x90a#LV[\x90Pa\x0CN\x87\x87\x87\x87`@Q` \x01a\x109\x94\x93\x92\x91\x90a#\xB5V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83\x83`\x01`Za\x13\ta\x11\xACV[`\0a\x10k\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x13:V[\x93\x92PPPV[`\0a\x10k\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x13:V[`\0\x80a\x10\x94\x84\x84a\x13hV[\x90P`\0a\x10\xA1\x85a\x13\x92V[\x90P`\0a\x10\xAF\x82\x86a\x13\xB7V[\x90P`\0a\x10\xBD\x8A\x8Aa\x12\xBDV[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0\x81\x12a\x10\xDBW`\0\x94PPPPPa\rkV[`\0\x81\x13a\x10\xF1W`\0\x19\x94PPPPPa\rkV[`\0a\x11\ra\x11\x08\x83g\r\xE0\xB6\xB3\xA7d\0\0a#\xE9V[a\x13\xCCV[\x90P`\0\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x11%\x88\x85a$\x10V[a\x11/\x91\x90a$VV[a\x119\x91\x90a#\xE9V[\x90P`\0a\x11F\x82a\x14iV[\x90P`\0a\x11S\x82a\x16\x12V[\x90Pa\x11_\x8C\x82a\x10VV[\x9E\x9DPPPPPPPPPPPPPPV[`\0\x80`\0\x80`\0\x86\x80` \x01\x90Q\x81\x01\x90a\x11\x8D\x91\x90a$\x84V[\x93P\x93P\x93P\x93P\x81a\x11\xA2\x87\x86\x86\x85a\x16[V[a\x0CN\x91\x90a#\xE9V[`\0\x84\x86\x11\x15a\x11\xD9W`@Qc0\x82\xDF\xDB`\xE1\x1B\x81R`\x04\x81\x01\x87\x90R`$\x81\x01\x86\x90R`D\x01a\x06*V[`\0a\x11\xE9\x88\x88\x85c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x11\xFB\x89\x88\x86c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x12\t\x82\x84a$\x10V[\x13\x15a\x122W`@Qc\x06\xF1\xBE]`\xE2\x1B\x81R`\x04\x81\x01\x83\x90R`$\x81\x01\x82\x90R`D\x01a\x06*V[`\0a\x12>\x89\x89a#LV[\x90P`\0[`\x02a\x12O\x8A\x8Ca#9V[a\x12Y\x91\x90a$\xBCV[\x94P`\0a\x12k\x8C\x87\x89c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x12y\x86\x83a$\x10V[\x13a\x12\x86W\x85\x99Pa\x12\x8DV[\x85\x9AP\x80\x94P[a\x12\x97\x8B\x8Ba#LV[\x92PP`\x01\x01\x87\x82\x11\x80\x15a\x12\xABWP\x86\x81\x10[a\x12CWPPPP\x96\x95PPPPPPV[`\0a\x10k\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x17hV[`\0\x80`\0\x80\x85\x80` \x01\x90Q\x81\x01\x90a\x12\xEC\x91\x90a$\x84V[\x93PP\x92P\x92Pa\x12\xFF\x83\x83\x87\x84a\x16[V[\x96\x95PPPPPPV[`\0\x80`\0\x80`\0\x86\x80` \x01\x90Q\x81\x01\x90a\x13%\x91\x90a$\x84V[\x93P\x93P\x93P\x93P\x81a\x11\xA2\x85\x88\x86\x85a\x16[V[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x13RW`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[`\0\x80a\x13t\x83a\x17\x87V[\x90P`\0a\x13\x86c;\x9A\xCA\0\x83a$\xD0V[\x90Pa\rk\x85\x82a\x13\xB7V[`\0\x80a\x13\xA7\x83g\x1B\xC1mgN\xC8\0\0a\x18+V[\x90Pa\x10kg\x06\xF0[Y\xD3\xB2\0\0\x82[`\0a\x10k\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x17hV[`\0g\x06\xF0[Y\xD3\xB2\0\0\x82\x03a\x13\xE5WP`\0\x91\x90PV[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x12a\x14\rW`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\x14.W`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x14;\x83`\x02a$\x10V[\x90P`\0a\x14H\x82a\x18\\V[\x90P`\0a\x14^g\x13\xA0K\xBD\xFD\xC9\xBE\x88\x83a\x1A\xDAV[\x90Pa\rk\x81a$\xE7V[`\0h\x02H\xCE6\xA7\x0C\xB2k>\x19\x82\x13a\x14\x84WP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a\x14\xCBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x01a\x06*V[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05`\x01`_\x1B\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1Dm\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1Dn\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[`\0\x80\x82\x12\x15a\x16WW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01RotoUint: negative`\x80\x1B`D\x82\x01R`d\x01a\x06*V[P\x90V[`\0\x82\x85\x10a\x16\xACW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FtradingFunction: invalid x\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06*V[`\0\x80g\r\xE0\xB6\xB3\xA7d\0\0a\x16\xC2\x88\x87a\x12\xBDV[\x10a\x16\xD6W`\x01`\x01`\xFF\x1B\x03\x91Pa\x16\xE6V[a\x16\xE3a\x11\x08\x88\x87a\x12\xBDV[\x91P[g\r\xE0\xB6\xB3\xA7d\0\0a\x17\x06\x87a\x17\x01\x87`\0\x01Q\x89a\x13\xB7V[a\x12\xBDV[\x10a\x17\x19WP`\x01`\x01`\xFF\x1B\x03a\x171V[a\x17.a\x11\x08\x87a\x17\x01\x87`\0\x01Q\x89a\x13\xB7V[\x90P[`\0a\x17E\x85` \x01Q\x86`@\x01Qa\x13hV[\x90P\x80a\x17R\x83\x85a%\x03V[a\x17\\\x91\x90a%\x03V[\x98\x97PPPPPPPPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x17\x80W`\0\x80\xFD[\x04\x92\x91PPV[`\xB5\x81`\x01`\x88\x1B\x81\x10a\x17\xA0W`@\x91\x90\x91\x1B\x90`\x80\x1C[i\x01\0\0\0\0\0\0\0\0\0\x81\x10a\x17\xBCW` \x91\x90\x91\x1B\x90`@\x1C[e\x01\0\0\0\0\0\x81\x10a\x17\xD4W`\x10\x91\x90\x91\x1B\x90` \x1C[c\x01\0\0\0\x81\x10a\x17\xEAW`\x08\x91\x90\x91\x1B\x90`\x10\x1C[b\x01\0\0\x01\x02`\x12\x1C\x80\x82\x04\x01`\x01\x90\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x90\x81\x90\x04\x81\x11\x90\x03\x90V[`\0a\x10kg\r\xE0\xB6\xB3\xA7d\0\0\x83a\x18C\x86a\x1A\xEFV[a\x18M\x91\x90a$\x10V[a\x18W\x91\x90a$VV[a\x14iV[`\0\x80\x82\x12\x80a\x18sWPg\x1B\xC1mgN\xC8\0\0\x82\x13[\x15a\x18\x91W`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\x18\xB2W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81g\x1B\xC1mgN\xC8\0\0\x03a\x18\xDAW`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x15a\x18\xE5W\x91\x90PV[`\0g\r\xE0\xB6\xB3\xA7d\0\0\x83\x12a\x19\rWa\x19\x08\x83g\x1B\xC1mgN\xC8\0\0a#\xE9V[a\x19\x0FV[\x82[\x90P`\0a\x19%\x82g\x1B\xC1mgN\xC8\0\0a\x1C\xCAV[\x90P\x80`\0\x03a\x19HW`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x19S\x82a\x1A\xEFV[\x90P`\0c;\x9A\xCA\0a\x19~a\x19ya\x19sg\x1B\xC1mgN\xC8\0\0a$\xE7V[\x85a\x1A\xDAV[a\x17\x87V[a\x19\x88\x91\x90a$\x10V[\x90P`\0\x80a\x19\x9F\x83g\x03\xC1f\\z\xAB \0a\x1A\xDAV[a\x19\xB1\x90g \x05\xFEO&\x8E\xA0\0a%\x03V[\x90P`\0a\x19\xE1\x84a\x19\xCA\x86f\x9F2u$b\xA0\0a\x1A\xDAV[a\x19\xDC\x90g\r\xC5R\x7Fd, \0a%\x03V[a\x1A\xDAV[a\x19\xF3\x90g\r\xE0\xB6\xB3\xA7d\0\0a%\x03V[\x90Pa\x1A\x17g\t\xD0(\xCCo _\xFF\x19\x85a\x1A\r\x85\x85a\x1C\xCAV[a\x19\xDC\x91\x90a#\xE9V[\x92PPP`\0[`\x02\x81\x10\x15a\x1A\xB2W`\0\x86a\x1A3\x84a\x1C\xDFV[a\x1A=\x91\x90a#\xE9V[\x90P`\0a\x1AK\x84\x85a\x1A\xDAV[a\x1AT\x90a$\xE7V[\x90P`\0a\x1Aa\x82a\x14iV[\x90P`\0a\x1Ao\x86\x85a\x1A\xDAV[a\x1A\x81g\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x84a\x1A\xDAV[a\x1A\x8B\x91\x90a#\xE9V[\x90Pa\x1A\x97\x84\x82a\x1C\xCAV[a\x1A\xA1\x90\x87a%\x03V[\x95P\x84`\x01\x01\x94PPPPPa\x1A\x1EV[g\r\xE0\xB6\xB3\xA7d\0\0\x88\x12a\x1A\xCFWa\x1A\xCA\x82a$\xE7V[a\x17\\V[P\x96\x95PPPPPPV[`\0a\x10k\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x1E\xC3V[`\0\x80\x82\x13a\x1B,W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x06*V[`\0``a\x1B9\x84a\x1E\xE2V[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[`\0a\x10k\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x1E\xC3V[`\0\x81`\0\x03a\x1C\xF8WPg\r\xE0\xB6\xB3\xA7d\0\0\x91\x90PV[gV\x98\xEE\xF0fp\0\0\x82\x12a\x1D\x0FWP`\0\x91\x90PV[a\x1D gV\x98\xEE\xF0fp\0\0a$\xE7V[\x82\x13a\x1D5WPg\x1B\xC1mgN\xC8\0\0\x91\x90PV[`\0a\x1D@\x83a\x1F\x8AV[\x90P`\0a\x1Dyg\r\xE0\xB6\xB3\xA7d\0\0a\x1Db\x84g\x1B\xC1mgN\xC8\0\0a\x12\xBDV[a\x1Dt\x90g\r\xE0\xB6\xB3\xA7d\0\0a%\x03V[a\x1C\xCAV[\x90P`\0\x80\x82a\x1D\xD5\x81a\x1D\xC2\x81a\x1D\xB0\x81a\x1D\x9D\x81g\x02_\x0F\xE1\x05\xA3\x14\0a\x1A\xDAV[a\x19\xDC\x90g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19a%\x03V[a\x19\xDC\x90g\x14\xA8EL\x19\xE1\xAC\0a%\x03V[a\x19\xDC\x90g\x0F\xC1\x0E\x01W\x82w\xFF\x19a%\x03V[a\x1D\xE7\x90g\x03\xDE\xBD\x08;\x8C|\0a%\x03V[\x91P\x83\x90Pa\x1EO\x81a\x1E=\x81a\x1E+\x81a\x1E\x19\x81a\x1E\x06\x81\x8Ba\x1A\xDAV[a\x19\xDC\x90g\x02\x95\xD4\0\xEA2W\xFF\x19a%\x03V[a\x19\xDC\x90g\x01W\xD8\xB2\xEC\xC7\x08\0a%\x03V[a\x19\xDC\x90g\x051\n\xA7\xD5!0\0a%\x03V[a\x19\xDC\x90g\r\xE0\xCC=\x15a\0\0a%\x03V[\x91P\x81g\x11\x90\0\xAB\x10\x0F\xFC\0a\x1Ee\x87\x88a\x1A\xDAV[a\x1Eq\x90`\0\x19a$\x10V[a\x1E{\x91\x90a#\xE9V[a\x1E\x85\x91\x90a%\x03V[\x92PP`\0a\x1E\x93\x83a\x14iV[\x90P`\0a\x1E\xA1\x85\x83a\x1A\xDAV[\x90P`\0\x88\x12a\x1E\xB1W\x80a\x17\\V[a\x17\\\x81g\x1B\xC1mgN\xC8\0\0a#\xE9V[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x05\x85\x14\x17\x16a\x1E\xDBW`\0\x80\xFD[\x05\x92\x91PPV[`\0\x80\x82\x11a\x1F\x1FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x06*V[P`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x82\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[`\0`\x01`\xFF\x1B\x82\x03a\x1F\xB0W`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x12\x15a\x16WWP\x19`\x01\x01\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[\x80\x15\x15\x81\x14a \x1FW`\0\x80\xFD[PV[`\0\x80`@\x83\x85\x03\x12\x15a 8Wa 8a\x1F\xC1V[\x825a C\x81a \x11V[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a wW` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a [V[P`\0` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[\x84\x15\x15\x81R\x83` \x82\x01R\x82`@\x82\x01R`\x80``\x82\x01R`\0a\x12\xFF`\x80\x83\x01\x84a QV[`\0\x80`@\x83\x85\x03\x12\x15a \xD4Wa \xD4a\x1F\xC1V[PP\x805\x92` \x90\x91\x015\x91PV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FABI decoding: struct data too sh`D\x82\x01Rb\x1B\xDC\x9D`\xEA\x1B`d\x82\x01R`\x84\x81\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a!eWcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@R\x90V[`\0``\x82\x84\x03\x12\x15a!\x80Wa!\x80a \xE3V[a!\x88a!4V[\x90P\x815\x81R` \x82\x015` \x82\x01R`@\x82\x015`@\x82\x01R\x92\x91PPV[`\0\x80`\0\x80`\xC0\x85\x87\x03\x12\x15a!\xC1Wa!\xC1a\x1F\xC1V[\x845\x93P` \x85\x015\x92P`@\x85\x015\x91Pa!\xE0\x86``\x87\x01a!kV[\x90P\x92\x95\x91\x94P\x92PV[` \x81R`\0a\x10k` \x83\x01\x84a QV[`\0\x80`\0\x80`\0`\xE0\x86\x88\x03\x12\x15a\"\x19Wa\"\x19a\x1F\xC1V[\x855\x94P` \x86\x015\x93P`@\x86\x015\x92P``\x86\x015\x91Pa\"?\x87`\x80\x88\x01a!kV[\x90P\x92\x95P\x92\x95\x90\x93PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\"cWa\"ca\x1F\xC1V[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\"\x92Wa\"\x92a\x1F\xC1V[\x83Q\x92P` \x84\x01Q\x91P`@\x84\x01Q\x90P\x92P\x92P\x92V[`\0``\x82\x84\x03\x12\x15a\"\xC0Wa\"\xC0a \xE3V[a\"\xC8a!4V[\x90P\x81Q\x81R` \x82\x01Q` \x82\x01R`@\x82\x01Q`@\x82\x01R\x92\x91PPV[`\0``\x82\x84\x03\x12\x15a\"\xFDWa\"\xFDa\x1F\xC1V[a\x10k\x83\x83a\"\xABV[`\0` \x82\x84\x03\x12\x15a#\x1CWa#\x1Ca\x1F\xC1V[PQ\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\n&Wa\n&a##V[\x81\x81\x03\x81\x81\x11\x15a\n&Wa\n&a##V[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a#{Wa#{a\x1F\xC1V[\x86Qa#\x86\x81a \x11V[` \x88\x01Q`@\x89\x01Q``\x8A\x01Q`\x80\x8B\x01Q`\xA0\x90\x9B\x01Q\x93\x9C\x92\x9BP\x90\x99\x90\x98P\x96P\x90\x94P\x92PPPV[\x84\x81R` \x80\x82\x01\x85\x90R`@\x80\x83\x01\x85\x90R\x83Q``\x84\x01R\x90\x83\x01Q`\x80\x83\x01R\x82\x01Q`\xA0\x82\x01R`\xC0\x81\x01a\rkV[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a$\tWa$\ta##V[P\x92\x91PPV[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15a$,Wa$,a##V[\x81\x81\x05\x83\x14\x82\x15\x17a\n&Wa\n&a##V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a$eWa$ea$@V[`\x01`\xFF\x1B\x82\x14`\0\x19\x84\x14\x16\x15a$\x7FWa$\x7Fa##V[P\x05\x90V[`\0\x80`\0\x80`\xC0\x85\x87\x03\x12\x15a$\x9DWa$\x9Da\x1F\xC1V[\x84Q\x93P` \x85\x01Q\x92P`@\x85\x01Q\x91Pa!\xE0\x86``\x87\x01a\"\xABV[`\0\x82a$\xCBWa$\xCBa$@V[P\x04\x90V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\n&Wa\n&a##V[`\0`\x01`\xFF\x1B\x82\x01a$\xFCWa$\xFCa##V[P`\0\x03\x90V[\x80\x82\x01\x82\x81\x12`\0\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a%#Wa%#a##V[PP\x92\x91PPV\xFETarget contract does not contain";
    /// The deployed bytecode of the contract.
    pub static SOLVER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct Solver<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Solver<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Solver<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Solver<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Solver<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Solver)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Solver<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    SOLVER_ABI.clone(),
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
                SOLVER_ABI.clone(),
                SOLVER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `BISECTION_EPSILON` (0x6d652299) function
        pub fn bisection_epsilon(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([109, 101, 34, 153], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `MAX_BISECTION_ITERS` (0xf9c28211) function
        pub fn max_bisection_iters(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([249, 194, 130, 17], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `computeNextLiquidity` (0x69a4e488) function
        pub fn compute_next_liquidity(
            &self,
            reserve_x_wad: ::ethers::core::types::U256,
            reserve_y_wad: ::ethers::core::types::U256,
            swap_constant: ::ethers::core::types::I256,
            current_liquidity: ::ethers::core::types::U256,
            params: Parameters,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [105, 164, 228, 136],
                    (
                        reserve_x_wad,
                        reserve_y_wad,
                        swap_constant,
                        current_liquidity,
                        params,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `computeNextReserveX` (0x48a5f19d) function
        pub fn compute_next_reserve_x(
            &self,
            reserve_y_wad: ::ethers::core::types::U256,
            liquidity: ::ethers::core::types::U256,
            swap_constant: ::ethers::core::types::I256,
            params: Parameters,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [72, 165, 241, 157],
                    (reserve_y_wad, liquidity, swap_constant, params),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `computeNextReserveY` (0xf42bff7c) function
        pub fn compute_next_reserve_y(
            &self,
            reserve_x_wad: ::ethers::core::types::U256,
            liquidity: ::ethers::core::types::U256,
            swap_constant: ::ethers::core::types::I256,
            params: Parameters,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [244, 43, 255, 124],
                    (reserve_x_wad, liquidity, swap_constant, params),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `encodeInitData` (0x0f0aa395) function
        pub fn encode_init_data(
            &self,
            reserve_x_wad: ::ethers::core::types::U256,
            reserve_y_wad: ::ethers::core::types::U256,
            total_liquidity: ::ethers::core::types::U256,
            params: Parameters,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash(
                    [15, 10, 163, 149],
                    (reserve_x_wad, reserve_y_wad, total_liquidity, params),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getNextLiquidity` (0xec29d8e6) function
        pub fn get_next_liquidity(
            &self,
            reserve_x_wad: ::ethers::core::types::U256,
            reserve_y_wad: ::ethers::core::types::U256,
            total_liquidity: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [236, 41, 216, 230],
                    (reserve_x_wad, reserve_y_wad, total_liquidity),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getNextReserveX` (0x1fca38db) function
        pub fn get_next_reserve_x(
            &self,
            reserve_y_wad: ::ethers::core::types::U256,
            total_liquidity: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([31, 202, 56, 219], (reserve_y_wad, total_liquidity))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getNextReserveY` (0x0ae1c5a5) function
        pub fn get_next_reserve_y(
            &self,
            reserve_x_wad: ::ethers::core::types::U256,
            total_liquidity: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([10, 225, 197, 165], (reserve_x_wad, total_liquidity))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `internalPrice` (0xb8f00b72) function
        pub fn internal_price(
            &self,
            reserve_x_wad: ::ethers::core::types::U256,
            total_liquidity: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([184, 240, 11, 114], (reserve_x_wad, total_liquidity))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `simulateSwap` (0x0ac33034) function
        pub fn simulate_swap(
            &self,
            swap_x_in: bool,
            amount_in: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                bool,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::Bytes,
            ),
        > {
            self.0
                .method_hash([10, 195, 48, 52], (swap_x_in, amount_in))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `strategy` (0xa8c62e76) function
        pub fn strategy(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([168, 198, 46, 118], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Solver<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `BisectionLib_InvalidBounds` with signature `BisectionLib_InvalidBounds(uint256,uint256)` and selector `0x6105bfb6`
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
    #[etherror(
        name = "BisectionLib_InvalidBounds",
        abi = "BisectionLib_InvalidBounds(uint256,uint256)"
    )]
    pub struct BisectionLib_InvalidBounds {
        pub lower: ::ethers::core::types::U256,
        pub upper: ::ethers::core::types::U256,
    }
    ///Custom Error type `BisectionLib_RootOutsideBounds` with signature `BisectionLib_RootOutsideBounds(int256,int256)` and selector `0x1bc6f974`
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
    #[etherror(
        name = "BisectionLib_RootOutsideBounds",
        abi = "BisectionLib_RootOutsideBounds(int256,int256)"
    )]
    pub struct BisectionLib_RootOutsideBounds {
        pub lower_result: ::ethers::core::types::I256,
        pub upper_result: ::ethers::core::types::I256,
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
    pub enum SolverErrors {
        BisectionLib_InvalidBounds(BisectionLib_InvalidBounds),
        BisectionLib_RootOutsideBounds(BisectionLib_RootOutsideBounds),
        Infinity(Infinity),
        Min(Min),
        NegativeInfinity(NegativeInfinity),
        OutOfBounds(OutOfBounds),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for SolverErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <BisectionLib_InvalidBounds as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BisectionLib_InvalidBounds(decoded));
            }
            if let Ok(decoded) = <BisectionLib_RootOutsideBounds as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BisectionLib_RootOutsideBounds(decoded));
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
    impl ::ethers::core::abi::AbiEncode for SolverErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::BisectionLib_InvalidBounds(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BisectionLib_RootOutsideBounds(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
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
    impl ::ethers::contract::ContractRevert for SolverErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <BisectionLib_InvalidBounds as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <BisectionLib_RootOutsideBounds as ::ethers::contract::EthError>::selector() => {
                    true
                }
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
    impl ::core::fmt::Display for SolverErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::BisectionLib_InvalidBounds(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BisectionLib_RootOutsideBounds(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Infinity(element) => ::core::fmt::Display::fmt(element, f),
                Self::Min(element) => ::core::fmt::Display::fmt(element, f),
                Self::NegativeInfinity(element) => ::core::fmt::Display::fmt(element, f),
                Self::OutOfBounds(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for SolverErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<BisectionLib_InvalidBounds> for SolverErrors {
        fn from(value: BisectionLib_InvalidBounds) -> Self {
            Self::BisectionLib_InvalidBounds(value)
        }
    }
    impl ::core::convert::From<BisectionLib_RootOutsideBounds> for SolverErrors {
        fn from(value: BisectionLib_RootOutsideBounds) -> Self {
            Self::BisectionLib_RootOutsideBounds(value)
        }
    }
    impl ::core::convert::From<Infinity> for SolverErrors {
        fn from(value: Infinity) -> Self {
            Self::Infinity(value)
        }
    }
    impl ::core::convert::From<Min> for SolverErrors {
        fn from(value: Min) -> Self {
            Self::Min(value)
        }
    }
    impl ::core::convert::From<NegativeInfinity> for SolverErrors {
        fn from(value: NegativeInfinity) -> Self {
            Self::NegativeInfinity(value)
        }
    }
    impl ::core::convert::From<OutOfBounds> for SolverErrors {
        fn from(value: OutOfBounds) -> Self {
            Self::OutOfBounds(value)
        }
    }
    ///Container type for all input parameters for the `BISECTION_EPSILON` function with signature `BISECTION_EPSILON()` and selector `0x6d652299`
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
    #[ethcall(name = "BISECTION_EPSILON", abi = "BISECTION_EPSILON()")]
    pub struct BisectionEpsilonCall;
    ///Container type for all input parameters for the `MAX_BISECTION_ITERS` function with signature `MAX_BISECTION_ITERS()` and selector `0xf9c28211`
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
    #[ethcall(name = "MAX_BISECTION_ITERS", abi = "MAX_BISECTION_ITERS()")]
    pub struct MaxBisectionItersCall;
    ///Container type for all input parameters for the `computeNextLiquidity` function with signature `computeNextLiquidity(uint256,uint256,int256,uint256,(uint256,uint256,uint256))` and selector `0x69a4e488`
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
        name = "computeNextLiquidity",
        abi = "computeNextLiquidity(uint256,uint256,int256,uint256,(uint256,uint256,uint256))"
    )]
    pub struct ComputeNextLiquidityCall {
        pub reserve_x_wad: ::ethers::core::types::U256,
        pub reserve_y_wad: ::ethers::core::types::U256,
        pub swap_constant: ::ethers::core::types::I256,
        pub current_liquidity: ::ethers::core::types::U256,
        pub params: Parameters,
    }
    ///Container type for all input parameters for the `computeNextReserveX` function with signature `computeNextReserveX(uint256,uint256,int256,(uint256,uint256,uint256))` and selector `0x48a5f19d`
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
        name = "computeNextReserveX",
        abi = "computeNextReserveX(uint256,uint256,int256,(uint256,uint256,uint256))"
    )]
    pub struct ComputeNextReserveXCall {
        pub reserve_y_wad: ::ethers::core::types::U256,
        pub liquidity: ::ethers::core::types::U256,
        pub swap_constant: ::ethers::core::types::I256,
        pub params: Parameters,
    }
    ///Container type for all input parameters for the `computeNextReserveY` function with signature `computeNextReserveY(uint256,uint256,int256,(uint256,uint256,uint256))` and selector `0xf42bff7c`
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
        name = "computeNextReserveY",
        abi = "computeNextReserveY(uint256,uint256,int256,(uint256,uint256,uint256))"
    )]
    pub struct ComputeNextReserveYCall {
        pub reserve_x_wad: ::ethers::core::types::U256,
        pub liquidity: ::ethers::core::types::U256,
        pub swap_constant: ::ethers::core::types::I256,
        pub params: Parameters,
    }
    ///Container type for all input parameters for the `encodeInitData` function with signature `encodeInitData(uint256,uint256,uint256,(uint256,uint256,uint256))` and selector `0x0f0aa395`
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
        name = "encodeInitData",
        abi = "encodeInitData(uint256,uint256,uint256,(uint256,uint256,uint256))"
    )]
    pub struct EncodeInitDataCall {
        pub reserve_x_wad: ::ethers::core::types::U256,
        pub reserve_y_wad: ::ethers::core::types::U256,
        pub total_liquidity: ::ethers::core::types::U256,
        pub params: Parameters,
    }
    ///Container type for all input parameters for the `getNextLiquidity` function with signature `getNextLiquidity(uint256,uint256,uint256)` and selector `0xec29d8e6`
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
        name = "getNextLiquidity",
        abi = "getNextLiquidity(uint256,uint256,uint256)"
    )]
    pub struct GetNextLiquidityCall {
        pub reserve_x_wad: ::ethers::core::types::U256,
        pub reserve_y_wad: ::ethers::core::types::U256,
        pub total_liquidity: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getNextReserveX` function with signature `getNextReserveX(uint256,uint256)` and selector `0x1fca38db`
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
    #[ethcall(name = "getNextReserveX", abi = "getNextReserveX(uint256,uint256)")]
    pub struct GetNextReserveXCall {
        pub reserve_y_wad: ::ethers::core::types::U256,
        pub total_liquidity: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getNextReserveY` function with signature `getNextReserveY(uint256,uint256)` and selector `0x0ae1c5a5`
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
    #[ethcall(name = "getNextReserveY", abi = "getNextReserveY(uint256,uint256)")]
    pub struct GetNextReserveYCall {
        pub reserve_x_wad: ::ethers::core::types::U256,
        pub total_liquidity: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `internalPrice` function with signature `internalPrice(uint256,uint256)` and selector `0xb8f00b72`
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
    #[ethcall(name = "internalPrice", abi = "internalPrice(uint256,uint256)")]
    pub struct InternalPriceCall {
        pub reserve_x_wad: ::ethers::core::types::U256,
        pub total_liquidity: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `simulateSwap` function with signature `simulateSwap(bool,uint256)` and selector `0x0ac33034`
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
    #[ethcall(name = "simulateSwap", abi = "simulateSwap(bool,uint256)")]
    pub struct SimulateSwapCall {
        pub swap_x_in: bool,
        pub amount_in: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `strategy` function with signature `strategy()` and selector `0xa8c62e76`
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
    #[ethcall(name = "strategy", abi = "strategy()")]
    pub struct StrategyCall;
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
    pub enum SolverCalls {
        BisectionEpsilon(BisectionEpsilonCall),
        MaxBisectionIters(MaxBisectionItersCall),
        ComputeNextLiquidity(ComputeNextLiquidityCall),
        ComputeNextReserveX(ComputeNextReserveXCall),
        ComputeNextReserveY(ComputeNextReserveYCall),
        EncodeInitData(EncodeInitDataCall),
        GetNextLiquidity(GetNextLiquidityCall),
        GetNextReserveX(GetNextReserveXCall),
        GetNextReserveY(GetNextReserveYCall),
        InternalPrice(InternalPriceCall),
        SimulateSwap(SimulateSwapCall),
        Strategy(StrategyCall),
    }
    impl ::ethers::core::abi::AbiDecode for SolverCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <BisectionEpsilonCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BisectionEpsilon(decoded));
            }
            if let Ok(decoded) = <MaxBisectionItersCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MaxBisectionIters(decoded));
            }
            if let Ok(decoded) = <ComputeNextLiquidityCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ComputeNextLiquidity(decoded));
            }
            if let Ok(decoded) = <ComputeNextReserveXCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ComputeNextReserveX(decoded));
            }
            if let Ok(decoded) = <ComputeNextReserveYCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ComputeNextReserveY(decoded));
            }
            if let Ok(decoded) = <EncodeInitDataCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EncodeInitData(decoded));
            }
            if let Ok(decoded) = <GetNextLiquidityCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetNextLiquidity(decoded));
            }
            if let Ok(decoded) = <GetNextReserveXCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetNextReserveX(decoded));
            }
            if let Ok(decoded) = <GetNextReserveYCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetNextReserveY(decoded));
            }
            if let Ok(decoded) = <InternalPriceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InternalPrice(decoded));
            }
            if let Ok(decoded) = <SimulateSwapCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SimulateSwap(decoded));
            }
            if let Ok(decoded) = <StrategyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Strategy(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for SolverCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::BisectionEpsilon(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MaxBisectionIters(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ComputeNextLiquidity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ComputeNextReserveX(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ComputeNextReserveY(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EncodeInitData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetNextLiquidity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetNextReserveX(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetNextReserveY(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InternalPrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SimulateSwap(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Strategy(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for SolverCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::BisectionEpsilon(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaxBisectionIters(element) => ::core::fmt::Display::fmt(element, f),
                Self::ComputeNextLiquidity(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ComputeNextReserveX(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ComputeNextReserveY(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EncodeInitData(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetNextLiquidity(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetNextReserveX(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetNextReserveY(element) => ::core::fmt::Display::fmt(element, f),
                Self::InternalPrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::SimulateSwap(element) => ::core::fmt::Display::fmt(element, f),
                Self::Strategy(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<BisectionEpsilonCall> for SolverCalls {
        fn from(value: BisectionEpsilonCall) -> Self {
            Self::BisectionEpsilon(value)
        }
    }
    impl ::core::convert::From<MaxBisectionItersCall> for SolverCalls {
        fn from(value: MaxBisectionItersCall) -> Self {
            Self::MaxBisectionIters(value)
        }
    }
    impl ::core::convert::From<ComputeNextLiquidityCall> for SolverCalls {
        fn from(value: ComputeNextLiquidityCall) -> Self {
            Self::ComputeNextLiquidity(value)
        }
    }
    impl ::core::convert::From<ComputeNextReserveXCall> for SolverCalls {
        fn from(value: ComputeNextReserveXCall) -> Self {
            Self::ComputeNextReserveX(value)
        }
    }
    impl ::core::convert::From<ComputeNextReserveYCall> for SolverCalls {
        fn from(value: ComputeNextReserveYCall) -> Self {
            Self::ComputeNextReserveY(value)
        }
    }
    impl ::core::convert::From<EncodeInitDataCall> for SolverCalls {
        fn from(value: EncodeInitDataCall) -> Self {
            Self::EncodeInitData(value)
        }
    }
    impl ::core::convert::From<GetNextLiquidityCall> for SolverCalls {
        fn from(value: GetNextLiquidityCall) -> Self {
            Self::GetNextLiquidity(value)
        }
    }
    impl ::core::convert::From<GetNextReserveXCall> for SolverCalls {
        fn from(value: GetNextReserveXCall) -> Self {
            Self::GetNextReserveX(value)
        }
    }
    impl ::core::convert::From<GetNextReserveYCall> for SolverCalls {
        fn from(value: GetNextReserveYCall) -> Self {
            Self::GetNextReserveY(value)
        }
    }
    impl ::core::convert::From<InternalPriceCall> for SolverCalls {
        fn from(value: InternalPriceCall) -> Self {
            Self::InternalPrice(value)
        }
    }
    impl ::core::convert::From<SimulateSwapCall> for SolverCalls {
        fn from(value: SimulateSwapCall) -> Self {
            Self::SimulateSwap(value)
        }
    }
    impl ::core::convert::From<StrategyCall> for SolverCalls {
        fn from(value: StrategyCall) -> Self {
            Self::Strategy(value)
        }
    }
    ///Container type for all return fields from the `BISECTION_EPSILON` function with signature `BISECTION_EPSILON()` and selector `0x6d652299`
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
    pub struct BisectionEpsilonReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `MAX_BISECTION_ITERS` function with signature `MAX_BISECTION_ITERS()` and selector `0xf9c28211`
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
    pub struct MaxBisectionItersReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `computeNextLiquidity` function with signature `computeNextLiquidity(uint256,uint256,int256,uint256,(uint256,uint256,uint256))` and selector `0x69a4e488`
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
    pub struct ComputeNextLiquidityReturn {
        pub liquidity: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `computeNextReserveX` function with signature `computeNextReserveX(uint256,uint256,int256,(uint256,uint256,uint256))` and selector `0x48a5f19d`
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
    pub struct ComputeNextReserveXReturn {
        pub reserve_y: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `computeNextReserveY` function with signature `computeNextReserveY(uint256,uint256,int256,(uint256,uint256,uint256))` and selector `0xf42bff7c`
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
    pub struct ComputeNextReserveYReturn {
        pub reserve_y: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `encodeInitData` function with signature `encodeInitData(uint256,uint256,uint256,(uint256,uint256,uint256))` and selector `0x0f0aa395`
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
    pub struct EncodeInitDataReturn(pub ::ethers::core::types::Bytes);
    ///Container type for all return fields from the `getNextLiquidity` function with signature `getNextLiquidity(uint256,uint256,uint256)` and selector `0xec29d8e6`
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
    pub struct GetNextLiquidityReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getNextReserveX` function with signature `getNextReserveX(uint256,uint256)` and selector `0x1fca38db`
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
    pub struct GetNextReserveXReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getNextReserveY` function with signature `getNextReserveY(uint256,uint256)` and selector `0x0ae1c5a5`
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
    pub struct GetNextReserveYReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `internalPrice` function with signature `internalPrice(uint256,uint256)` and selector `0xb8f00b72`
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
    pub struct InternalPriceReturn {
        pub price: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `simulateSwap` function with signature `simulateSwap(bool,uint256)` and selector `0x0ac33034`
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
    pub struct SimulateSwapReturn(
        pub bool,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::Bytes,
    );
    ///Container type for all return fields from the `strategy` function with signature `strategy()` and selector `0xa8c62e76`
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
    pub struct StrategyReturn(pub ::ethers::core::types::Address);
}
