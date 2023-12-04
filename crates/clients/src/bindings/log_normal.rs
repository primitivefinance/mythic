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
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("swapFeePercentageWad_"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("APPROXIMATED_MINIMUM_X_INPUT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "APPROXIMATED_MINIMUM_X_INPUT",
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
                    ::std::borrow::ToOwned::to_owned("HALF_WAD"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("HALF_WAD"),
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
                    ::std::borrow::ToOwned::to_owned("INFINITY_IS_NOT_REAL"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "INFINITY_IS_NOT_REAL",
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
                    ::std::borrow::ToOwned::to_owned("TWO_WAD"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("TWO_WAD"),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("WAD"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("WAD"),
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
                    ::std::borrow::ToOwned::to_owned("ZERO"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("ZERO"),
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
                    ::std::borrow::ToOwned::to_owned("computeD1"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("computeD1"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("priceWad"),
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
                                    name: ::std::borrow::ToOwned::to_owned("d1"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("computeD2"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("computeD2"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("priceWad"),
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
                                    name: ::std::borrow::ToOwned::to_owned("d2"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("computeHalfSigmaSquared"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "computeHalfSigmaSquared",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sigmaPercentWad"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("computePrice"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("computePrice"),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("strikePriceWad"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sigmaPercentWad"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tauYearsWad"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("computeSwapConstant"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "computeSwapConstant",
                            ),
                            inputs: ::std::vec![
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
                                    name: ::std::borrow::ToOwned::to_owned("reseveYWad"),
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
                    ::std::borrow::ToOwned::to_owned("encodeValidateData"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("encodeValidateData"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "adjustedReserveXWad",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "adjustedReserveYWad",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("adjustedLiquidity"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("findLiquidity"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("findLiquidity"),
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
                    ::std::borrow::ToOwned::to_owned("findX"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("findX"),
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
                    ::std::borrow::ToOwned::to_owned("findY"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("findY"),
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
                    ::std::borrow::ToOwned::to_owned("init"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("init"),
                            inputs: ::std::vec![
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
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "swapConstantGrowth",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
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
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
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
                    ::std::borrow::ToOwned::to_owned("lx"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("lx"),
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
                                    name: ::std::borrow::ToOwned::to_owned("priceWad"),
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
                    ::std::borrow::ToOwned::to_owned("ly"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("ly"),
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
                                    name: ::std::borrow::ToOwned::to_owned("priceWad"),
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
                    ::std::borrow::ToOwned::to_owned("mulidiv"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("mulidiv"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("x"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("y"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("denominator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("z"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("mulidivUp"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("mulidivUp"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("x"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("y"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("denominator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("z"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
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
                    ::std::borrow::ToOwned::to_owned("slot"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("slot"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("strikePriceWad"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sigmaPercentWad"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tauYearsWad"),
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
                    ::std::borrow::ToOwned::to_owned("swapFeePercentageWad"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "swapFeePercentageWad",
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
                    ::std::borrow::ToOwned::to_owned("toUint"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("toUint"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("x"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("validate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("validate"),
                            inputs: ::std::vec![
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
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "swapConstantGrowth",
                                    ),
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
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "adjustedReserveXWad",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "adjustedReserveYWad",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("adjustedLiquidity"),
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
                    ::std::borrow::ToOwned::to_owned("xl"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("xl"),
                            inputs: ::std::vec![
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
                                    name: ::std::borrow::ToOwned::to_owned("priceWad"),
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
                    ::std::borrow::ToOwned::to_owned("yl"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("yl"),
                            inputs: ::std::vec![
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
                                    name: ::std::borrow::ToOwned::to_owned("priceWad"),
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
    pub static LOGNORMAL_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15b\0\0^W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`@Qb\0-\x038\x03\x80b\0-\x03\x839\x81\x01`@\x81\x90Rb\0\0\x81\x91b\0\0\x8AV[`\0Ub\0\0\xEFV[`\0` \x82\x84\x03\x12\x15b\0\0\xE8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[PQ\x91\x90PV[a,\x04\x80b\0\0\xFF`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x022W`\x005`\xE0\x1C\x80c\x85\xAAEN\x11a\x01\\W\x80c\xC1nP\xEF\x11a\0\xEFW\x80c\xE3\xD0\xDC\xA5\x11a\0\xBEW\x80c\xE3\xD0\xDC\xA5\x14a\x05/W\x80c\xE4\x93t(\x14a\x058W\x80c\xF9\xC2\x82\x11\x14a\x05KW\x80c\xFF\xB3bf\x14a\x05TWa\x022V[\x80c\xC1nP\xEF\x14a\x04\xB4W\x80c\xCDd\xAE\xA2\x14a\x04\xF6W\x80c\xD7\xF9{\xEF\x14a\x05\tW\x80c\xDB\xAF\x11B\x14a\x05\x1CWa\x022V[\x80c\xA1\x9C\xD3\xD1\x11a\x01+W\x80c\xA1\x9C\xD3\xD1\x14a\x04=W\x80c\xA6\xD34\x98\x14a\x04PW\x80c\xB8\xF0\x0Br\x14a\x04cW\x80c\xBDB-(\x14a\x04vWa\x022V[\x80c\x85\xAAEN\x14a\x03\xF5W\x80c\x88;m\xC5\x14a\x04\x04W\x80c\x97\x16\xAE\xBB\x14a\x04\x17W\x80c\x9D\x81\x9E\x83\x14a\x04*Wa\x022V[\x80c@\xB41i\x11a\x01\xD4W\x80cd\x17\xD4\xB5\x11a\x01\xA3W\x80cd\x17\xD4\xB5\x14a\x03\xC3W\x80cj\x14`$\x14a\x03\xD6W\x80cme\"\x99\x14a\x03\xE5W\x80c\x84\xC8*&\x14a\x03\xEDWa\x022V[\x80c@\xB41i\x14a\x03bW\x80cM\xDFG\xD4\x14a\x03kW\x80cV\x08\xBE\xA1\x14a\x03\xA8W\x80cX\xFAc\xCA\x14a\x03\xBBWa\x022V[\x80c\x18M0\xBA\x11a\x02\x10W\x80c\x18M0\xBA\x14a\x03\0W\x80c\x1A\x88\xBCf\x14a\x03\x13W\x80c\x1D\x9C\xF7\"\x14a\x03@W\x80c']g\xC8\x14a\x03SWa\x022V[\x80c\x06%\xA6#\x14a\x02\x97W\x80c\n\xC304\x14a\x02\xBDW\x80c\x0F\n\xA3\x95\x14a\x02\xE0W[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x02\xAAa\x02\xA56`\x04a#\xA4V[a\x05gV[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x02\xD0a\x02\xCB6`\x04a#\xE2V[a\x06SV[`@Qa\x02\xB4\x94\x93\x92\x91\x90a$\\V[a\x02\xF3a\x02\xEE6`\x04a%\x8BV[a\n\x8DV[`@Qa\x02\xB4\x91\x90a%\xCEV[a\x02\xAAa\x03\x0E6`\x04a%\xE1V[a\n\xBFV[`\x01T`\x02T`\x03Ta\x03%\x92\x91\x90\x83V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x02\xB4V[a\x02\xAAa\x03N6`\x04a&\x1AV[a\x0B\x11V[a\x02\xAAg\x1B\xC1mgN\xC8\0\0\x81V[a\x02\xAA`\0T\x81V[a\x03~a\x03y6`\x04a&\x8FV[a\x0BZV[`@\x80Q\x95\x15\x15\x86R` \x86\x01\x94\x90\x94R\x92\x84\x01\x91\x90\x91R``\x83\x01R`\x80\x82\x01R`\xA0\x01a\x02\xB4V[a\x02\xAAa\x03\xB66`\x04a%\xE1V[a\x0B\xF6V[a\x02\xAA`\0\x81V[a\x02\xAAa\x03\xD16`\x04a%\x8BV[a\x0C7V[a\x02\xAAg\r\xE0\xB6\xB3\xA7d\0\0\x81V[a\x02\xAA`\x01\x81V[a\x02\xAA`\n\x81V[a\x02\xAAg\x06\xF0[Y\xD3\xB2\0\0\x81V[a\x02\xAAa\x04\x126`\x04a%\x8BV[a\x0C\x91V[a\x02\xAAa\x04%6`\x04a'\xB4V[a\x0C\xEAV[a\x02\xAAa\x0486`\x04a%\xE1V[a\r\x87V[a\x02\xAAa\x04K6`\x04a'\xE4V[a\x0E\x1CV[a\x02\xAAa\x04^6`\x04a&\x1AV[a\x0EnV[a\x02\xAAa\x04q6`\x04a(\xD2V[a\x0E\x97V[a\x02\xF3a\x04\x846`\x04a(\xF7V[`@\x80Q` \x81\x01\x94\x90\x94R\x83\x81\x01\x92\x90\x92R``\x80\x84\x01\x91\x90\x91R\x81Q\x80\x84\x03\x90\x91\x01\x81R`\x80\x90\x92\x01\x90R\x90V[a\x04\xC7a\x04\xC26`\x04a'\xE4V[a\x0E\xB4V[`@\x80Q\x96\x15\x15\x87R` \x87\x01\x95\x90\x95R\x93\x85\x01\x92\x90\x92R``\x84\x01R`\x80\x83\x01R`\xA0\x82\x01R`\xC0\x01a\x02\xB4V[a\x02\xAAa\x05\x046`\x04a%\x8BV[a\x12CV[a\x02\xAAa\x05\x176`\x04a'\xB4V[a\x12\x8CV[a\x02\xAAa\x05*6`\x04a%\xE1V[a\x12\xB4V[a\x02\xAA`\0\x19\x81V[a\x02\xAAa\x05F6`\x04a(\xF7V[a\x12\xF6V[a\x02\xAAa\x01\0\x81V[a\x02\xAAa\x05b6`\x04a(\xF7V[a\x13tV[`\0\x80a\x05t\x84\x84a\x13\xFBV[\x90P`\0a\x05\x81\x85a\x0EnV[\x90P`\0a\x05\x8F\x82\x86a\x140V[\x90P`\0a\x05\x9D\x8A\x8Aa\x14EV[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0\x81\x12a\x05\xBBW`\0\x94PPPPPa\x06JV[`\0\x81\x13a\x05\xD1W`\0\x19\x94PPPPPa\x06JV[`\0a\x05\xEDa\x05\xE8\x83g\r\xE0\xB6\xB3\xA7d\0\0a)<V[a\x14ZV[\x90P`\0\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x06\x05\x88\x85a)cV[a\x06\x0F\x91\x90a)\xA9V[a\x06\x19\x91\x90a)<V[\x90P`\0a\x06&\x82a\x14\xF7V[\x90P`\0a\x063\x82a\x0B\x11V[\x90Pa\x06?\x8C\x82a\x16\xA0V[\x98PPPPPPPPP[\x95\x94PPPPPV[`\0\x80`\0``a\x06{`@Q\x80`\x80\x01`@R\x80`G\x81R` \x01a+\x98`G\x919a\x16\xB5V[`\0\x80`\x003`\x01`\x01`\xA0\x1B\x03\x16c\xEB\xAD\xEF\x01`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01```@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x07\tW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x07\x1DW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07A\x91\x90a)\xD7V[`@\x80Q` \x81\x01\x85\x90R\x90\x81\x01\x83\x90R``\x81\x01\x82\x90R\x92\x95P\x90\x93P\x91P`\0\x90a\x07\x7F\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@Ra\x0E\x1CV[\x90P`\0\x8A\x15a\x08\xCBW`\0\x80Ta\x07\x98\x90\x8C\x90a\x16\xA0V[\x90P`\0a\x07\xB0\x87a\x07\xAA\x84\x88a\x16\xA0V[\x90a\x16\xFBV[\x90Pa\x07\xBD`\x01\x82a*\x08V[\x90Pa\x07\xC9\x8C\x88a*\x08V[\x96Pa\x07\xD5\x81\x86a*\x08V[`@\x80Q``\x81\x01\x82R`\x01T\x81R`\x02T` \x82\x01R`\x03T\x91\x81\x01\x91\x90\x91R\x90\x95P\x86\x90a\x08\n\x90\x89\x90\x88\x90\x88\x90a\x0C7V[\x96Pa\x08\x17`\x01\x88a*\x08V[\x96P\x80\x87\x10a\x08xW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: y reserve increase`D\x82\x01Rad!`\xF0\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[a\x08\x82\x87\x82a*\x1BV[\x93Pa\x08\xC3`@Q\x80`@\x01`@R\x80`\x1C\x81R` \x01\x7FEsimated Y reserve to submit\0\0\0\0\x81RP\x88a\x17\x10V[PPPa\n\x17V[`\0\x80Ta\x08\xDA\x90\x8C\x90a\x16\xA0V[\x90P`\0a\x08\xEC\x86a\x07\xAA\x84\x88a\x16\xA0V[\x90Pa\x08\xF9`\x01\x82a*\x08V[\x90Pa\t\x05\x8C\x87a*\x08V[\x95Pa\t\x11\x81\x86a*\x08V[\x94P`\0\x87\x90Pa\tC`@Q\x80`@\x01`@R\x80`\t\x81R` \x01hadjustedY`\xB8\x1B\x81RP\x88a\x17\x10V[a\tn`@Q\x80`@\x01`@R\x80`\t\x81R` \x01h\x18Y\x1A\x9D\\\xDD\x19Y\x13`\xBA\x1B\x81RP\x87a\x17\x10V[`@\x80Q``\x81\x01\x82R`\x01T\x81R`\x02T` \x82\x01R`\x03T\x91\x81\x01\x91\x90\x91Ra\t\x9E\x90\x88\x90\x88\x90\x88\x90a\x12CV[\x97Pa\t\xAB`\x01\x89a*\x08V[\x97P\x80\x88\x10a\n\x07W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: x reserve increase`D\x82\x01Rad!`\xF0\x1B`d\x82\x01R`\x84\x01a\x08oV[a\n\x11\x88\x82a*\x1BV[\x93PPPP[`@\x80Q` \x81\x01\x87\x90R\x90\x81\x01\x85\x90R``\x81\x01\x84\x90R`\0\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`\0a\nQ\x82a\x0E\xB4V[PPPPP\x90P\x80\x83a\nu\x89\x88`\x01`\0\x01T`\x01\x80\x01T`\x01`\x02\x01Ta\x05gV[\x91\x9F\x90\x9EP\x90\x9CP\x91\x9AP\x90\x98PPPPPPPPPV[``\x84\x84\x84\x84`@Q` \x01a\n\xA6\x94\x93\x92\x91\x90a*.V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x94\x93PPPPV[`\0\x80a\n\xCC\x84\x84a\x0C\xEAV[\x90P`\0a\n\xD9\x82a\x17YV[\x90P`\0a\n\xE6\x82a\x0B\x11V[\x90Pa\x0B\x04a\n\xFD\x82g\r\xE0\xB6\xB3\xA7d\0\0a*\x1BV[\x88\x90a\x16\xA0V[\x93PPPP[\x93\x92PPPV[`\0\x80\x82\x12\x15a\x0BVW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01RotoUint: negative`\x80\x1B`D\x82\x01R`d\x01a\x08oV[P\x90V[`\0\x80`\0\x80`\0a\x0B\x83`@Q\x80`\x80\x01`@R\x80`G\x81R` \x01a+\x98`G\x919a\x16\xB5V[a\x0B\x8F\x86\x88\x01\x88a%\x8BV[\x80Q`\x01\x81\x90U` \x80\x83\x01Q`\x02\x81\x90U`@\x93\x84\x01Q`\x03\x81\x90U\x84Q``\x81\x01\x86R\x93\x84R\x91\x83\x01R\x91\x81\x01\x91\x90\x91R\x92\x95P\x90\x93P\x91Pa\x0B\xD9\x90\x84\x90\x84\x90\x84\x90a\x17\xC2V[\x93Pa\x0B\xE7`\0`\x03a*bV[\x84\x12\x15\x94P\x92\x95P\x92\x95\x90\x93PV[`\0\x80a\x0C\x03\x84\x84a\x12\x8CV[\x90P`\0a\x0C\x10\x82a\x17YV[\x90P`\0a\x0C\x1D\x82a\x0B\x11V[\x85Q\x90\x91Pa\x0B\x04\x90a\x0C0\x90\x83a\x140V[\x88\x90a\x14EV[\x80Q`\0\x90`\n\x90\x82\x90a\x0CL\x90\x83\x90a*\x1BV[\x90Pa\x0C\x86\x87\x87\x87\x87`@Q` \x01a\x0Ch\x94\x93\x92\x91\x90a*.V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83\x83`\x01a\x01\0a\x18ma\x18\xA8V[\x97\x96PPPPPPPV[`\0\x80a\x0C\x9F\x86`\x01a*\x08V[\x90P`\0k\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x90Pa\x0C\x86\x87\x87\x87\x87`@Q` \x01a\x0C\xCC\x94\x93\x92\x91\x90a*.V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83\x83`\x01a\x01\0a\x19\xB9a\x18\xA8V[`\0\x80a\x0C\xFF\x83` \x01Q\x84`@\x01Qa\x13\xFBV[\x90P`\0a\r\x10\x84` \x01Qa\x0EnV[\x90P`\0a\r3a\r.\x86`\0\x01Q\x88a\x16\xFB\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x19\xEAV[\x90P\x80a\r?\x81a*\x8AV[\x91PP`\0a\r[\x86`@\x01Q\x84a\x140\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\re\x90\x83a*bV[\x90P\x83a\r{\x82g\r\xE0\xB6\xB3\xA7d\0\0\x83a\x13tV[\x98\x97PPPPPPPPV[`\0\x80a\r\x94\x84\x84a\x0C\xEAV[\x90P`\0a\r\xA1\x82a\x17YV[\x90P`\0a\r\xAE\x82a\x0B\x11V[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0\x81\x10a\x0E\x07W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7Flx: denominator is zero\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x08oV[a\x0B\x04a\x0C0\x82g\r\xE0\xB6\xB3\xA7d\0\0a*\x1BV[`\0\x80`\0\x80\x84\x80` \x01\x90Q\x81\x01\x90a\x0E6\x91\x90a)\xD7V[`@\x80Q``\x81\x01\x82R`\x01T\x81R`\x02T` \x82\x01R`\x03T\x91\x81\x01\x91\x90\x91R\x92\x95P\x90\x93P\x91Pa\x06J\x90\x84\x90\x84\x90\x84\x90a\x17\xC2V[`\0\x80a\x0E\x83\x83g\x1B\xC1mgN\xC8\0\0a\x1B\xC5V[\x90Pa\x0B\ng\x06\xF0[Y\xD3\xB2\0\0\x82a\x140V[`\0a\x0B\n\x83\x83`\x01`\0\x01T`\x01\x80\x01T`\x01`\x02\x01Ta\x05gV[`\0\x80`\0\x80`\0\x80a\x0E\xDE`@Q\x80`\x80\x01`@R\x80`G\x81R` \x01a+\x98`G\x919a\x16\xB5V[`\0\x80`\x003`\x01`\x01`\xA0\x1B\x03\x16c\xEB\xAD\xEF\x01`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01```@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0FlW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0F\x80W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xA4\x91\x90a)\xD7V[\x92P\x92P\x92P\x89\x80` \x01\x90Q\x81\x01\x90a\x0F\xBE\x91\x90a)\xD7V[\x91\x97P\x95P\x93P`\x01\x83\x87\x11\x15a\x10\x17W`\0a\x0F\xDB\x85\x89a*\x1BV[\x90P`\0a\x0F\xF4`\0T\x83a\x16\xA0\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa\x10\x04\x86a\x07\xAA\x83\x87a\x16\xA0V[a\x10\x0E\x90\x84a*\x08V[\x92PPPa\x10\xB5V[\x82\x86\x11\x15a\x10TW`\0a\x10+\x84\x88a*\x1BV[\x90P`\0a\x10D`\0T\x83a\x16\xA0\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa\x10\x04\x85a\x07\xAA\x83\x87a\x16\xA0V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`0`$\x82\x01R\x7Finvalid swap: inputs x and y hav`D\x82\x01Roe the same sign!`\x80\x1B`d\x82\x01R`\x84\x01a\x08oV[a\x10\xBF\x82\x86a)<V[`@\x80Q``\x81\x01\x82R`\x01T\x81R`\x02T` \x82\x01R`\x03T\x91\x81\x01\x91\x90\x91R\x90\x98Pa\x10\xF2\x90\x85\x90\x85\x90\x85\x90a\x17\xC2V[`@\x80Q``\x81\x01\x82R`\x01T\x81R`\x02T` \x82\x01R`\x03T\x91\x81\x01\x91\x90\x91Ra\x11\"\x90\x89\x90\x89\x90\x89\x90a\x17\xC2V[a\x11,\x91\x90a)<V[\x98Pa\x11c`@Q\x80`@\x01`@R\x80`\x14\x81R` \x01s\nn\xEC.\x04\x0Cm\xED\xCEn\x8C-\xCE\x84\x0C\xEEM\xEE\xEE\x8D`c\x1B\x81RPa\x16\xB5V[a\x11l\x89a\x1B\xF6V[a\x11\xAA`@Q\x80`@\x01`@R\x80`\x19\x81R` \x01\x7FSubmitted Liquidity delta\0\0\0\0\0\0\0\x81RPa\x16\xB5V[a\x11\xB3\x88a\x1B\xF6V[a\x11\xE7`@Q\x80`@\x01`@R\x80`\x13\x81R` \x01rMin liquidity delta`h\x1B\x81RPa\x16\xB5V[a\x11\xF0\x81a\x1B\xF6V[a\x12\x11`@Q\x80``\x01`@R\x80`%\x81R` \x01a+\xDF`%\x919a\x16\xB5V[a\x12#a\x12\x1E\x82\x8Aa)<V[a\x1B\xF6V[`\0\x89\x12\x15\x80\x15a\x124WP\x80\x88\x12\x15[\x99PPPPP\x91\x93\x95P\x91\x93\x95V[`\0`\n\x81a\x12R\x82\x87a*\x1BV[\x90Pa\x0C\x86\x87\x87\x87\x87`@Q` \x01a\x12n\x94\x93\x92\x91\x90a*.V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83\x83`\x01a\x01\0a\x1C;a\x18\xA8V[`\0a\x12\xA0\x82` \x01Q\x83`@\x01Qa\x13\xFBV[a\x12\xAA\x84\x84a\x0C\xEAV[a\x0B\n\x91\x90a)<V[`\0\x80a\x12\xC1\x84\x84a\x12\x8CV[\x90P`\0a\x12\xCE\x82a\x17YV[\x90P`\0a\x12\xDB\x82a\x0B\x11V[\x85Q\x90\x91Pa\x0B\x04\x90\x82\x90a\x12\xF0\x90\x8Aa\x16\xA0V[\x90a\x16\xA0V[\x82\x82\x02\x81\x15\x80\x15\x90a\x13\x1EWP\x83\x15\x80a\x13\x1EWP\x82\x84\x82\x81a\x13\x1BWa\x13\x1Ba)\x93V[\x05\x14[a\x13\\W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rn\x1B][\x1AY\x1A]\x88\x1A[\x9D\x98[\x1AY`\x8A\x1B`D\x82\x01R`d\x01a\x08oV[\x81\x81\x81a\x13kWa\x13ka)\x93V[\x05\x94\x93PPPPV[`\0a\x13\x81\x84\x84\x84a\x12\xF6V[\x90P\x81a\x13\x8E\x84\x86a)cV[a\x13\x98\x91\x90a*\xA9V[\x15a\x0B\nW`\x01`\x01`\xFF\x1B\x03\x81\x12a\x13\xE8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01RqmulidivUp overflow`p\x1B`D\x82\x01R`d\x01a\x08oV[a\x13\xF3`\x01\x82a*bV[\x94\x93PPPPV[`\0\x80a\x14\x07\x83a\x1ClV[\x90P`\0a\x14\x19c;\x9A\xCA\0\x83a*\xBDV[\x90Pa\x14%\x85\x82a\x140V[\x92PPP[\x92\x91PPV[`\0a\x0B\n\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x1D\x10V[`\0a\x0B\n\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x1D\x10V[`\0g\x06\xF0[Y\xD3\xB2\0\0\x82\x03a\x14sWP`\0\x91\x90PV[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x12a\x14\x9BW`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\x14\xBCW`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x14\xC9\x83`\x02a)cV[\x90P`\0a\x14\xD6\x82a\x1D/V[\x90P`\0a\x14\xECg\x13\xA0K\xBD\xFD\xC9\xBE\x88\x83a\x1F\xADV[\x90Pa\x06J\x81a*\xD4V[`\0h\x02H\xCE6\xA7\x0C\xB2k>\x19\x82\x13a\x15\x12WP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a\x15YW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x01a\x08oV[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05`\x01`_\x1B\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1Dm\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1Dn\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[`\0a\x0B\n\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x1F\xC2V[a\x16\xF8\x81`@Q`$\x01a\x16\xC9\x91\x90a%\xCEV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\x10L\x13\xEB`\xE2\x1B\x17\x90Ra\x1F\xF0V[PV[`\0a\x0B\n\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x1F\xC2V[a\x17U\x82\x82`@Q`$\x01a\x17&\x92\x91\x90a*\xF0V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c-\x83\x9C\xB3`\xE2\x1B\x17\x90Ra\x1F\xF0V[PPV[`\0\x80g\x13\xA0K\xBD\xFD\xC9\xBE\x88a\x17wg\r\xE0\xB6\xB3\xA7d\0\0\x85a)cV[a\x17\x81\x91\x90a)\xA9V[\x90P`\0a\x17\x8E\x82a*\xD4V[\x90P`\0a\x17\x9B\x82a \x11V[\x90Pg\x1B\xC1mgN\xC8\0\0a\x17\xB8g\r\xE0\xB6\xB3\xA7d\0\0\x83a)cV[a\x06J\x91\x90a)\xA9V[`\0\x82\x85\x10a\x18\x13W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FtradingFunction: invalid x\0\0\0\0\0\0`D\x82\x01R`d\x01a\x08oV[`\0a\x18\"a\x05\xE8\x87\x86a\x14EV[\x90P`\0a\x18@a\x05\xE8\x87a\x18;\x87`\0\x01Q\x89a\x140V[a\x14EV[\x90P`\0a\x18V\x85` \x01Q\x86`@\x01Qa\x13\xFBV[\x90P\x80a\x18c\x83\x85a*bV[a\r{\x91\x90a*bV[`\0\x80`\0\x80`\0\x86\x80` \x01\x90Q\x81\x01\x90a\x18\x89\x91\x90a+\x12V[\x93P\x93P\x93P\x93P\x81a\x18\x9E\x85\x88\x86\x85a\x17\xC2V[a\x0C\x86\x91\x90a)<V[`\0\x84\x86\x11\x15a\x18\xD5W`@Qc0\x82\xDF\xDB`\xE1\x1B\x81R`\x04\x81\x01\x87\x90R`$\x81\x01\x86\x90R`D\x01a\x08oV[`\0a\x18\xE5\x88\x88\x85c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x18\xF7\x89\x88\x86c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x19\x05\x82\x84a)cV[\x13\x15a\x19.W`@Qc\x06\xF1\xBE]`\xE2\x1B\x81R`\x04\x81\x01\x83\x90R`$\x81\x01\x82\x90R`D\x01a\x08oV[`\0a\x19:\x89\x89a*\x1BV[\x90P`\0[`\x02a\x19K\x8A\x8Ca*\x08V[a\x19U\x91\x90a+\x83V[\x94P`\0a\x19g\x8C\x87\x89c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x19u\x86\x83a)cV[\x13a\x19\x82W\x85\x99Pa\x19\x89V[\x85\x9AP\x80\x94P[a\x19\x93\x8B\x8Ba*\x1BV[\x92PP`\x01\x01\x87\x82\x11\x80\x15a\x19\xA7WP\x86\x81\x10[a\x19?WPPPP\x96\x95PPPPPPV[`\0\x80`\0\x80`\0\x86\x80` \x01\x90Q\x81\x01\x90a\x19\xD5\x91\x90a+\x12V[\x93P\x93P\x93P\x93P\x81a\x18\x9E\x85\x85\x89\x85a\x17\xC2V[`\0\x80\x82\x13a\x1A'W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x08oV[`\0``a\x1A4\x84a!\xF5V[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[`\0a\x0B\ng\r\xE0\xB6\xB3\xA7d\0\0\x83a\x1B\xDD\x86a\x19\xEAV[a\x1B\xE7\x91\x90a)cV[a\x1B\xF1\x91\x90a)\xA9V[a\x14\xF7V[a\x16\xF8\x81`@Q`$\x01a\x1C\x0C\x91\x81R` \x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c-[l\xB9`\xE0\x1B\x17\x90Ra\x1F\xF0V[`\0\x80`\0\x80`\0\x86\x80` \x01\x90Q\x81\x01\x90a\x1CW\x91\x90a+\x12V[\x93P\x93P\x93P\x93P\x81a\x18\x9E\x87\x86\x86\x85a\x17\xC2V[`\xB5\x81`\x01`\x88\x1B\x81\x10a\x1C\x85W`@\x91\x90\x91\x1B\x90`\x80\x1C[i\x01\0\0\0\0\0\0\0\0\0\x81\x10a\x1C\xA1W` \x91\x90\x91\x1B\x90`@\x1C[e\x01\0\0\0\0\0\x81\x10a\x1C\xB9W`\x10\x91\x90\x91\x1B\x90` \x1C[c\x01\0\0\0\x81\x10a\x1C\xCFW`\x08\x91\x90\x91\x1B\x90`\x10\x1C[b\x01\0\0\x01\x02`\x12\x1C\x80\x82\x04\x01`\x01\x90\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x90\x81\x90\x04\x81\x11\x90\x03\x90V[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x1D(W`\0\x80\xFD[\x04\x92\x91PPV[`\0\x80\x82\x12\x80a\x1DFWPg\x1B\xC1mgN\xC8\0\0\x82\x13[\x15a\x1DdW`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\x1D\x85W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81g\x1B\xC1mgN\xC8\0\0\x03a\x1D\xADW`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x15a\x1D\xB8W\x91\x90PV[`\0g\r\xE0\xB6\xB3\xA7d\0\0\x83\x12a\x1D\xE0Wa\x1D\xDB\x83g\x1B\xC1mgN\xC8\0\0a)<V[a\x1D\xE2V[\x82[\x90P`\0a\x1D\xF8\x82g\x1B\xC1mgN\xC8\0\0a\"\x9DV[\x90P\x80`\0\x03a\x1E\x1BW`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x1E&\x82a\x19\xEAV[\x90P`\0c;\x9A\xCA\0a\x1EQa\x1ELa\x1EFg\x1B\xC1mgN\xC8\0\0a*\xD4V[\x85a\x1F\xADV[a\x1ClV[a\x1E[\x91\x90a)cV[\x90P`\0\x80a\x1Er\x83g\x03\xC1f\\z\xAB \0a\x1F\xADV[a\x1E\x84\x90g \x05\xFEO&\x8E\xA0\0a*bV[\x90P`\0a\x1E\xB4\x84a\x1E\x9D\x86f\x9F2u$b\xA0\0a\x1F\xADV[a\x1E\xAF\x90g\r\xC5R\x7Fd, \0a*bV[a\x1F\xADV[a\x1E\xC6\x90g\r\xE0\xB6\xB3\xA7d\0\0a*bV[\x90Pa\x1E\xEAg\t\xD0(\xCCo _\xFF\x19\x85a\x1E\xE0\x85\x85a\"\x9DV[a\x1E\xAF\x91\x90a)<V[\x92PPP`\0[`\x02\x81\x10\x15a\x1F\x85W`\0\x86a\x1F\x06\x84a \x11V[a\x1F\x10\x91\x90a)<V[\x90P`\0a\x1F\x1E\x84\x85a\x1F\xADV[a\x1F'\x90a*\xD4V[\x90P`\0a\x1F4\x82a\x14\xF7V[\x90P`\0a\x1FB\x86\x85a\x1F\xADV[a\x1FTg\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x84a\x1F\xADV[a\x1F^\x91\x90a)<V[\x90Pa\x1Fj\x84\x82a\"\x9DV[a\x1Ft\x90\x87a*bV[\x95P\x84`\x01\x01\x94PPPPPa\x1E\xF1V[g\r\xE0\xB6\xB3\xA7d\0\0\x88\x12a\x1F\xA2Wa\x1F\x9D\x82a*\xD4V[a\r{V[P\x96\x95PPPPPPV[`\0a\x0B\n\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\"\xAEV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x1F\xDAW`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[\x80Qjconsole.log` \x83\x01`\0\x80\x84\x83\x85Z\xFAPPPPPV[`\0\x81`\0\x03a *WPg\r\xE0\xB6\xB3\xA7d\0\0\x91\x90PV[gV\x98\xEE\xF0fp\0\0\x82\x12a AWP`\0\x91\x90PV[a RgV\x98\xEE\xF0fp\0\0a*\xD4V[\x82\x13a gWPg\x1B\xC1mgN\xC8\0\0\x91\x90PV[`\0a r\x83a\"\xCDV[\x90P`\0a \xABg\r\xE0\xB6\xB3\xA7d\0\0a \x94\x84g\x1B\xC1mgN\xC8\0\0a\x14EV[a \xA6\x90g\r\xE0\xB6\xB3\xA7d\0\0a*bV[a\"\x9DV[\x90P`\0\x80\x82a!\x07\x81a \xF4\x81a \xE2\x81a \xCF\x81g\x02_\x0F\xE1\x05\xA3\x14\0a\x1F\xADV[a\x1E\xAF\x90g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19a*bV[a\x1E\xAF\x90g\x14\xA8EL\x19\xE1\xAC\0a*bV[a\x1E\xAF\x90g\x0F\xC1\x0E\x01W\x82w\xFF\x19a*bV[a!\x19\x90g\x03\xDE\xBD\x08;\x8C|\0a*bV[\x91P\x83\x90Pa!\x81\x81a!o\x81a!]\x81a!K\x81a!8\x81\x8Ba\x1F\xADV[a\x1E\xAF\x90g\x02\x95\xD4\0\xEA2W\xFF\x19a*bV[a\x1E\xAF\x90g\x01W\xD8\xB2\xEC\xC7\x08\0a*bV[a\x1E\xAF\x90g\x051\n\xA7\xD5!0\0a*bV[a\x1E\xAF\x90g\r\xE0\xCC=\x15a\0\0a*bV[\x91P\x81g\x11\x90\0\xAB\x10\x0F\xFC\0a!\x97\x87\x88a\x1F\xADV[a!\xA3\x90`\0\x19a)cV[a!\xAD\x91\x90a)<V[a!\xB7\x91\x90a*bV[\x92PP`\0a!\xC5\x83a\x14\xF7V[\x90P`\0a!\xD3\x85\x83a\x1F\xADV[\x90P`\0\x88\x12a!\xE3W\x80a\r{V[a\r{\x81g\x1B\xC1mgN\xC8\0\0a)<V[`\0\x80\x82\x11a\"2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x08oV[P`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x82\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[`\0a\x0B\n\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x05\x85\x14\x17\x16a\"\xC6W`\0\x80\xFD[\x05\x92\x91PPV[`\0`\x01`\xFF\x1B\x82\x03a\"\xF3W`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x12\x15a\x0BVWP\x19`\x01\x01\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a#\xBFWa#\xBFa#\x04V[PP\x835\x95` \x85\x015\x95P`@\x85\x015\x94``\x81\x015\x94P`\x80\x015\x92P\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a#\xF8Wa#\xF8a#\x04V[\x825\x80\x15\x15\x81\x14a$\x08W`\0\x80\xFD[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a$<W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a$ V[P`\0` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[\x84\x15\x15\x81R\x83` \x82\x01R\x82`@\x82\x01R`\x80``\x82\x01R`\0a$\x83`\x80\x83\x01\x84a$\x16V[\x96\x95PPPPPPV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FABI decoding: struct data too sh`D\x82\x01Rb\x1B\xDC\x9D`\xEA\x1B`d\x82\x01R`\x84\x81\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a%\x17Wa%\x17a$\xDEV[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a%FWa%Fa$\xDEV[`@R\x91\x90PV[`\0``\x82\x84\x03\x12\x15a%cWa%ca$\x8DV[a%ka$\xF4V[\x90P\x815\x81R` \x82\x015` \x82\x01R`@\x82\x015`@\x82\x01R\x92\x91PPV[`\0\x80`\0\x80`\xC0\x85\x87\x03\x12\x15a%\xA4Wa%\xA4a#\x04V[\x845\x93P` \x85\x015\x92P`@\x85\x015\x91Pa%\xC3\x86``\x87\x01a%NV[\x90P\x92\x95\x91\x94P\x92PV[` \x81R`\0a\x0B\n` \x83\x01\x84a$\x16V[`\0\x80`\0`\xA0\x84\x86\x03\x12\x15a%\xF9Wa%\xF9a#\x04V[\x835\x92P` \x84\x015\x91Pa&\x11\x85`@\x86\x01a%NV[\x90P\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a&/Wa&/a#\x04V[P5\x91\x90PV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x81\xFD[`\0\x80` \x83\x85\x03\x12\x15a&\xA5Wa&\xA5a#\x04V[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a&\xC0Wa&\xC0a#TV[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a&\xD7Wa&\xD7a&6V[\x815\x81\x81\x11\x15a':W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x0ENL/$\r\x8C\xAD\xCC\xEE\x8D`\xAB\x1B`d\x82\x01R`\x84\x81\xFD[\x86` \x82\x85\x01\x01\x11\x15a'\xA2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rjrray stride`\xA8\x1B`d\x82\x01R\x91P`\x84\x82\xFD[` \x92\x90\x92\x01\x96\x91\x95P\x90\x93PPPPV[`\0\x80`\x80\x83\x85\x03\x12\x15a'\xCAWa'\xCAa#\x04V[\x825\x91Pa'\xDB\x84` \x85\x01a%NV[\x90P\x92P\x92\x90PV[`\0` \x80\x83\x85\x03\x12\x15a'\xFAWa'\xFAa#\x04V[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a(\x15Wa(\x15a#TV[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a(,Wa(,a&6V[\x815\x81\x81\x11\x15a(>Wa(>a$\xDEV[a(P`\x1F\x82\x01`\x1F\x19\x16\x85\x01a%\x1DV[\x91P\x80\x82R\x86\x84\x82\x85\x01\x01\x11\x15a(\xB6W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x85\x90R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R`\x84\x81\xFD[\x80\x84\x84\x01\x85\x84\x017`\0\x90\x82\x01\x90\x93\x01\x92\x90\x92RP\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a(\xE8Wa(\xE8a#\x04V[PP\x805\x92` \x90\x91\x015\x91PV[`\0\x80`\0``\x84\x86\x03\x12\x15a)\x0FWa)\x0Fa#\x04V[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a)\\Wa)\\a)&V[P\x92\x91PPV[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15a)\x7FWa)\x7Fa)&V[\x81\x81\x05\x83\x14\x82\x15\x17a\x14*Wa\x14*a)&V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a)\xB8Wa)\xB8a)\x93V[`\x01`\xFF\x1B\x82\x14`\0\x19\x84\x14\x16\x15a)\xD2Wa)\xD2a)&V[P\x05\x90V[`\0\x80`\0``\x84\x86\x03\x12\x15a)\xEFWa)\xEFa#\x04V[\x83Q\x92P` \x84\x01Q\x91P`@\x84\x01Q\x90P\x92P\x92P\x92V[\x80\x82\x01\x80\x82\x11\x15a\x14*Wa\x14*a)&V[\x81\x81\x03\x81\x81\x11\x15a\x14*Wa\x14*a)&V[\x84\x81R` \x80\x82\x01\x85\x90R`@\x80\x83\x01\x85\x90R\x83Q``\x84\x01R\x90\x83\x01Q`\x80\x83\x01R\x82\x01Q`\xA0\x82\x01R`\xC0\x81\x01a\x06JV[\x80\x82\x01\x82\x81\x12`\0\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a*\x82Wa*\x82a)&V[PP\x92\x91PPV[`\0`\x01`\x01`\xFF\x1B\x01\x82\x01a*\xA2Wa*\xA2a)&V[P`\x01\x01\x90V[`\0\x82a*\xB8Wa*\xB8a)\x93V[P\x07\x90V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x14*Wa\x14*a)&V[`\0`\x01`\xFF\x1B\x82\x01a*\xE9Wa*\xE9a)&V[P`\0\x03\x90V[`@\x81R`\0a+\x03`@\x83\x01\x85a$\x16V[\x90P\x82` \x83\x01R\x93\x92PPPV[`\0\x80`\0\x80\x84\x86\x03`\xC0\x81\x12\x15a+,Wa+,a#\x04V[\x85Q\x94P` \x86\x01Q\x93P`@\x86\x01Q\x92P```_\x19\x82\x01\x12\x15a+SWa+Sa$\x8DV[Pa+\\a$\xF4V[``\x86\x01Q\x81R`\x80\x86\x01Q` \x82\x01R`\xA0\x90\x95\x01Q`@\x86\x01RP\x91\x94\x90\x93P\x90\x91\x90V[`\0\x82a+\x92Wa+\x92a)\x93V[P\x04\x90V\xFEMake sure the calling contract of this console.log is the Core contractliquidity delta - min liquidity delta";
    /// The bytecode of the contract.
    pub static LOGNORMAL_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x022W`\x005`\xE0\x1C\x80c\x85\xAAEN\x11a\x01\\W\x80c\xC1nP\xEF\x11a\0\xEFW\x80c\xE3\xD0\xDC\xA5\x11a\0\xBEW\x80c\xE3\xD0\xDC\xA5\x14a\x05/W\x80c\xE4\x93t(\x14a\x058W\x80c\xF9\xC2\x82\x11\x14a\x05KW\x80c\xFF\xB3bf\x14a\x05TWa\x022V[\x80c\xC1nP\xEF\x14a\x04\xB4W\x80c\xCDd\xAE\xA2\x14a\x04\xF6W\x80c\xD7\xF9{\xEF\x14a\x05\tW\x80c\xDB\xAF\x11B\x14a\x05\x1CWa\x022V[\x80c\xA1\x9C\xD3\xD1\x11a\x01+W\x80c\xA1\x9C\xD3\xD1\x14a\x04=W\x80c\xA6\xD34\x98\x14a\x04PW\x80c\xB8\xF0\x0Br\x14a\x04cW\x80c\xBDB-(\x14a\x04vWa\x022V[\x80c\x85\xAAEN\x14a\x03\xF5W\x80c\x88;m\xC5\x14a\x04\x04W\x80c\x97\x16\xAE\xBB\x14a\x04\x17W\x80c\x9D\x81\x9E\x83\x14a\x04*Wa\x022V[\x80c@\xB41i\x11a\x01\xD4W\x80cd\x17\xD4\xB5\x11a\x01\xA3W\x80cd\x17\xD4\xB5\x14a\x03\xC3W\x80cj\x14`$\x14a\x03\xD6W\x80cme\"\x99\x14a\x03\xE5W\x80c\x84\xC8*&\x14a\x03\xEDWa\x022V[\x80c@\xB41i\x14a\x03bW\x80cM\xDFG\xD4\x14a\x03kW\x80cV\x08\xBE\xA1\x14a\x03\xA8W\x80cX\xFAc\xCA\x14a\x03\xBBWa\x022V[\x80c\x18M0\xBA\x11a\x02\x10W\x80c\x18M0\xBA\x14a\x03\0W\x80c\x1A\x88\xBCf\x14a\x03\x13W\x80c\x1D\x9C\xF7\"\x14a\x03@W\x80c']g\xC8\x14a\x03SWa\x022V[\x80c\x06%\xA6#\x14a\x02\x97W\x80c\n\xC304\x14a\x02\xBDW\x80c\x0F\n\xA3\x95\x14a\x02\xE0W[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x02\xAAa\x02\xA56`\x04a#\xA4V[a\x05gV[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x02\xD0a\x02\xCB6`\x04a#\xE2V[a\x06SV[`@Qa\x02\xB4\x94\x93\x92\x91\x90a$\\V[a\x02\xF3a\x02\xEE6`\x04a%\x8BV[a\n\x8DV[`@Qa\x02\xB4\x91\x90a%\xCEV[a\x02\xAAa\x03\x0E6`\x04a%\xE1V[a\n\xBFV[`\x01T`\x02T`\x03Ta\x03%\x92\x91\x90\x83V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x02\xB4V[a\x02\xAAa\x03N6`\x04a&\x1AV[a\x0B\x11V[a\x02\xAAg\x1B\xC1mgN\xC8\0\0\x81V[a\x02\xAA`\0T\x81V[a\x03~a\x03y6`\x04a&\x8FV[a\x0BZV[`@\x80Q\x95\x15\x15\x86R` \x86\x01\x94\x90\x94R\x92\x84\x01\x91\x90\x91R``\x83\x01R`\x80\x82\x01R`\xA0\x01a\x02\xB4V[a\x02\xAAa\x03\xB66`\x04a%\xE1V[a\x0B\xF6V[a\x02\xAA`\0\x81V[a\x02\xAAa\x03\xD16`\x04a%\x8BV[a\x0C7V[a\x02\xAAg\r\xE0\xB6\xB3\xA7d\0\0\x81V[a\x02\xAA`\x01\x81V[a\x02\xAA`\n\x81V[a\x02\xAAg\x06\xF0[Y\xD3\xB2\0\0\x81V[a\x02\xAAa\x04\x126`\x04a%\x8BV[a\x0C\x91V[a\x02\xAAa\x04%6`\x04a'\xB4V[a\x0C\xEAV[a\x02\xAAa\x0486`\x04a%\xE1V[a\r\x87V[a\x02\xAAa\x04K6`\x04a'\xE4V[a\x0E\x1CV[a\x02\xAAa\x04^6`\x04a&\x1AV[a\x0EnV[a\x02\xAAa\x04q6`\x04a(\xD2V[a\x0E\x97V[a\x02\xF3a\x04\x846`\x04a(\xF7V[`@\x80Q` \x81\x01\x94\x90\x94R\x83\x81\x01\x92\x90\x92R``\x80\x84\x01\x91\x90\x91R\x81Q\x80\x84\x03\x90\x91\x01\x81R`\x80\x90\x92\x01\x90R\x90V[a\x04\xC7a\x04\xC26`\x04a'\xE4V[a\x0E\xB4V[`@\x80Q\x96\x15\x15\x87R` \x87\x01\x95\x90\x95R\x93\x85\x01\x92\x90\x92R``\x84\x01R`\x80\x83\x01R`\xA0\x82\x01R`\xC0\x01a\x02\xB4V[a\x02\xAAa\x05\x046`\x04a%\x8BV[a\x12CV[a\x02\xAAa\x05\x176`\x04a'\xB4V[a\x12\x8CV[a\x02\xAAa\x05*6`\x04a%\xE1V[a\x12\xB4V[a\x02\xAA`\0\x19\x81V[a\x02\xAAa\x05F6`\x04a(\xF7V[a\x12\xF6V[a\x02\xAAa\x01\0\x81V[a\x02\xAAa\x05b6`\x04a(\xF7V[a\x13tV[`\0\x80a\x05t\x84\x84a\x13\xFBV[\x90P`\0a\x05\x81\x85a\x0EnV[\x90P`\0a\x05\x8F\x82\x86a\x140V[\x90P`\0a\x05\x9D\x8A\x8Aa\x14EV[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0\x81\x12a\x05\xBBW`\0\x94PPPPPa\x06JV[`\0\x81\x13a\x05\xD1W`\0\x19\x94PPPPPa\x06JV[`\0a\x05\xEDa\x05\xE8\x83g\r\xE0\xB6\xB3\xA7d\0\0a)<V[a\x14ZV[\x90P`\0\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x06\x05\x88\x85a)cV[a\x06\x0F\x91\x90a)\xA9V[a\x06\x19\x91\x90a)<V[\x90P`\0a\x06&\x82a\x14\xF7V[\x90P`\0a\x063\x82a\x0B\x11V[\x90Pa\x06?\x8C\x82a\x16\xA0V[\x98PPPPPPPPP[\x95\x94PPPPPV[`\0\x80`\0``a\x06{`@Q\x80`\x80\x01`@R\x80`G\x81R` \x01a+\x98`G\x919a\x16\xB5V[`\0\x80`\x003`\x01`\x01`\xA0\x1B\x03\x16c\xEB\xAD\xEF\x01`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01```@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x07\tW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x07\x1DW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07A\x91\x90a)\xD7V[`@\x80Q` \x81\x01\x85\x90R\x90\x81\x01\x83\x90R``\x81\x01\x82\x90R\x92\x95P\x90\x93P\x91P`\0\x90a\x07\x7F\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@Ra\x0E\x1CV[\x90P`\0\x8A\x15a\x08\xCBW`\0\x80Ta\x07\x98\x90\x8C\x90a\x16\xA0V[\x90P`\0a\x07\xB0\x87a\x07\xAA\x84\x88a\x16\xA0V[\x90a\x16\xFBV[\x90Pa\x07\xBD`\x01\x82a*\x08V[\x90Pa\x07\xC9\x8C\x88a*\x08V[\x96Pa\x07\xD5\x81\x86a*\x08V[`@\x80Q``\x81\x01\x82R`\x01T\x81R`\x02T` \x82\x01R`\x03T\x91\x81\x01\x91\x90\x91R\x90\x95P\x86\x90a\x08\n\x90\x89\x90\x88\x90\x88\x90a\x0C7V[\x96Pa\x08\x17`\x01\x88a*\x08V[\x96P\x80\x87\x10a\x08xW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: y reserve increase`D\x82\x01Rad!`\xF0\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[a\x08\x82\x87\x82a*\x1BV[\x93Pa\x08\xC3`@Q\x80`@\x01`@R\x80`\x1C\x81R` \x01\x7FEsimated Y reserve to submit\0\0\0\0\x81RP\x88a\x17\x10V[PPPa\n\x17V[`\0\x80Ta\x08\xDA\x90\x8C\x90a\x16\xA0V[\x90P`\0a\x08\xEC\x86a\x07\xAA\x84\x88a\x16\xA0V[\x90Pa\x08\xF9`\x01\x82a*\x08V[\x90Pa\t\x05\x8C\x87a*\x08V[\x95Pa\t\x11\x81\x86a*\x08V[\x94P`\0\x87\x90Pa\tC`@Q\x80`@\x01`@R\x80`\t\x81R` \x01hadjustedY`\xB8\x1B\x81RP\x88a\x17\x10V[a\tn`@Q\x80`@\x01`@R\x80`\t\x81R` \x01h\x18Y\x1A\x9D\\\xDD\x19Y\x13`\xBA\x1B\x81RP\x87a\x17\x10V[`@\x80Q``\x81\x01\x82R`\x01T\x81R`\x02T` \x82\x01R`\x03T\x91\x81\x01\x91\x90\x91Ra\t\x9E\x90\x88\x90\x88\x90\x88\x90a\x12CV[\x97Pa\t\xAB`\x01\x89a*\x08V[\x97P\x80\x88\x10a\n\x07W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: x reserve increase`D\x82\x01Rad!`\xF0\x1B`d\x82\x01R`\x84\x01a\x08oV[a\n\x11\x88\x82a*\x1BV[\x93PPPP[`@\x80Q` \x81\x01\x87\x90R\x90\x81\x01\x85\x90R``\x81\x01\x84\x90R`\0\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`\0a\nQ\x82a\x0E\xB4V[PPPPP\x90P\x80\x83a\nu\x89\x88`\x01`\0\x01T`\x01\x80\x01T`\x01`\x02\x01Ta\x05gV[\x91\x9F\x90\x9EP\x90\x9CP\x91\x9AP\x90\x98PPPPPPPPPV[``\x84\x84\x84\x84`@Q` \x01a\n\xA6\x94\x93\x92\x91\x90a*.V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x94\x93PPPPV[`\0\x80a\n\xCC\x84\x84a\x0C\xEAV[\x90P`\0a\n\xD9\x82a\x17YV[\x90P`\0a\n\xE6\x82a\x0B\x11V[\x90Pa\x0B\x04a\n\xFD\x82g\r\xE0\xB6\xB3\xA7d\0\0a*\x1BV[\x88\x90a\x16\xA0V[\x93PPPP[\x93\x92PPPV[`\0\x80\x82\x12\x15a\x0BVW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01RotoUint: negative`\x80\x1B`D\x82\x01R`d\x01a\x08oV[P\x90V[`\0\x80`\0\x80`\0a\x0B\x83`@Q\x80`\x80\x01`@R\x80`G\x81R` \x01a+\x98`G\x919a\x16\xB5V[a\x0B\x8F\x86\x88\x01\x88a%\x8BV[\x80Q`\x01\x81\x90U` \x80\x83\x01Q`\x02\x81\x90U`@\x93\x84\x01Q`\x03\x81\x90U\x84Q``\x81\x01\x86R\x93\x84R\x91\x83\x01R\x91\x81\x01\x91\x90\x91R\x92\x95P\x90\x93P\x91Pa\x0B\xD9\x90\x84\x90\x84\x90\x84\x90a\x17\xC2V[\x93Pa\x0B\xE7`\0`\x03a*bV[\x84\x12\x15\x94P\x92\x95P\x92\x95\x90\x93PV[`\0\x80a\x0C\x03\x84\x84a\x12\x8CV[\x90P`\0a\x0C\x10\x82a\x17YV[\x90P`\0a\x0C\x1D\x82a\x0B\x11V[\x85Q\x90\x91Pa\x0B\x04\x90a\x0C0\x90\x83a\x140V[\x88\x90a\x14EV[\x80Q`\0\x90`\n\x90\x82\x90a\x0CL\x90\x83\x90a*\x1BV[\x90Pa\x0C\x86\x87\x87\x87\x87`@Q` \x01a\x0Ch\x94\x93\x92\x91\x90a*.V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83\x83`\x01a\x01\0a\x18ma\x18\xA8V[\x97\x96PPPPPPPV[`\0\x80a\x0C\x9F\x86`\x01a*\x08V[\x90P`\0k\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x90Pa\x0C\x86\x87\x87\x87\x87`@Q` \x01a\x0C\xCC\x94\x93\x92\x91\x90a*.V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83\x83`\x01a\x01\0a\x19\xB9a\x18\xA8V[`\0\x80a\x0C\xFF\x83` \x01Q\x84`@\x01Qa\x13\xFBV[\x90P`\0a\r\x10\x84` \x01Qa\x0EnV[\x90P`\0a\r3a\r.\x86`\0\x01Q\x88a\x16\xFB\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x19\xEAV[\x90P\x80a\r?\x81a*\x8AV[\x91PP`\0a\r[\x86`@\x01Q\x84a\x140\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\re\x90\x83a*bV[\x90P\x83a\r{\x82g\r\xE0\xB6\xB3\xA7d\0\0\x83a\x13tV[\x98\x97PPPPPPPPV[`\0\x80a\r\x94\x84\x84a\x0C\xEAV[\x90P`\0a\r\xA1\x82a\x17YV[\x90P`\0a\r\xAE\x82a\x0B\x11V[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0\x81\x10a\x0E\x07W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7Flx: denominator is zero\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x08oV[a\x0B\x04a\x0C0\x82g\r\xE0\xB6\xB3\xA7d\0\0a*\x1BV[`\0\x80`\0\x80\x84\x80` \x01\x90Q\x81\x01\x90a\x0E6\x91\x90a)\xD7V[`@\x80Q``\x81\x01\x82R`\x01T\x81R`\x02T` \x82\x01R`\x03T\x91\x81\x01\x91\x90\x91R\x92\x95P\x90\x93P\x91Pa\x06J\x90\x84\x90\x84\x90\x84\x90a\x17\xC2V[`\0\x80a\x0E\x83\x83g\x1B\xC1mgN\xC8\0\0a\x1B\xC5V[\x90Pa\x0B\ng\x06\xF0[Y\xD3\xB2\0\0\x82a\x140V[`\0a\x0B\n\x83\x83`\x01`\0\x01T`\x01\x80\x01T`\x01`\x02\x01Ta\x05gV[`\0\x80`\0\x80`\0\x80a\x0E\xDE`@Q\x80`\x80\x01`@R\x80`G\x81R` \x01a+\x98`G\x919a\x16\xB5V[`\0\x80`\x003`\x01`\x01`\xA0\x1B\x03\x16c\xEB\xAD\xEF\x01`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01```@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0FlW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0F\x80W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xA4\x91\x90a)\xD7V[\x92P\x92P\x92P\x89\x80` \x01\x90Q\x81\x01\x90a\x0F\xBE\x91\x90a)\xD7V[\x91\x97P\x95P\x93P`\x01\x83\x87\x11\x15a\x10\x17W`\0a\x0F\xDB\x85\x89a*\x1BV[\x90P`\0a\x0F\xF4`\0T\x83a\x16\xA0\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa\x10\x04\x86a\x07\xAA\x83\x87a\x16\xA0V[a\x10\x0E\x90\x84a*\x08V[\x92PPPa\x10\xB5V[\x82\x86\x11\x15a\x10TW`\0a\x10+\x84\x88a*\x1BV[\x90P`\0a\x10D`\0T\x83a\x16\xA0\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa\x10\x04\x85a\x07\xAA\x83\x87a\x16\xA0V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`0`$\x82\x01R\x7Finvalid swap: inputs x and y hav`D\x82\x01Roe the same sign!`\x80\x1B`d\x82\x01R`\x84\x01a\x08oV[a\x10\xBF\x82\x86a)<V[`@\x80Q``\x81\x01\x82R`\x01T\x81R`\x02T` \x82\x01R`\x03T\x91\x81\x01\x91\x90\x91R\x90\x98Pa\x10\xF2\x90\x85\x90\x85\x90\x85\x90a\x17\xC2V[`@\x80Q``\x81\x01\x82R`\x01T\x81R`\x02T` \x82\x01R`\x03T\x91\x81\x01\x91\x90\x91Ra\x11\"\x90\x89\x90\x89\x90\x89\x90a\x17\xC2V[a\x11,\x91\x90a)<V[\x98Pa\x11c`@Q\x80`@\x01`@R\x80`\x14\x81R` \x01s\nn\xEC.\x04\x0Cm\xED\xCEn\x8C-\xCE\x84\x0C\xEEM\xEE\xEE\x8D`c\x1B\x81RPa\x16\xB5V[a\x11l\x89a\x1B\xF6V[a\x11\xAA`@Q\x80`@\x01`@R\x80`\x19\x81R` \x01\x7FSubmitted Liquidity delta\0\0\0\0\0\0\0\x81RPa\x16\xB5V[a\x11\xB3\x88a\x1B\xF6V[a\x11\xE7`@Q\x80`@\x01`@R\x80`\x13\x81R` \x01rMin liquidity delta`h\x1B\x81RPa\x16\xB5V[a\x11\xF0\x81a\x1B\xF6V[a\x12\x11`@Q\x80``\x01`@R\x80`%\x81R` \x01a+\xDF`%\x919a\x16\xB5V[a\x12#a\x12\x1E\x82\x8Aa)<V[a\x1B\xF6V[`\0\x89\x12\x15\x80\x15a\x124WP\x80\x88\x12\x15[\x99PPPPP\x91\x93\x95P\x91\x93\x95V[`\0`\n\x81a\x12R\x82\x87a*\x1BV[\x90Pa\x0C\x86\x87\x87\x87\x87`@Q` \x01a\x12n\x94\x93\x92\x91\x90a*.V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83\x83`\x01a\x01\0a\x1C;a\x18\xA8V[`\0a\x12\xA0\x82` \x01Q\x83`@\x01Qa\x13\xFBV[a\x12\xAA\x84\x84a\x0C\xEAV[a\x0B\n\x91\x90a)<V[`\0\x80a\x12\xC1\x84\x84a\x12\x8CV[\x90P`\0a\x12\xCE\x82a\x17YV[\x90P`\0a\x12\xDB\x82a\x0B\x11V[\x85Q\x90\x91Pa\x0B\x04\x90\x82\x90a\x12\xF0\x90\x8Aa\x16\xA0V[\x90a\x16\xA0V[\x82\x82\x02\x81\x15\x80\x15\x90a\x13\x1EWP\x83\x15\x80a\x13\x1EWP\x82\x84\x82\x81a\x13\x1BWa\x13\x1Ba)\x93V[\x05\x14[a\x13\\W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rn\x1B][\x1AY\x1A]\x88\x1A[\x9D\x98[\x1AY`\x8A\x1B`D\x82\x01R`d\x01a\x08oV[\x81\x81\x81a\x13kWa\x13ka)\x93V[\x05\x94\x93PPPPV[`\0a\x13\x81\x84\x84\x84a\x12\xF6V[\x90P\x81a\x13\x8E\x84\x86a)cV[a\x13\x98\x91\x90a*\xA9V[\x15a\x0B\nW`\x01`\x01`\xFF\x1B\x03\x81\x12a\x13\xE8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01RqmulidivUp overflow`p\x1B`D\x82\x01R`d\x01a\x08oV[a\x13\xF3`\x01\x82a*bV[\x94\x93PPPPV[`\0\x80a\x14\x07\x83a\x1ClV[\x90P`\0a\x14\x19c;\x9A\xCA\0\x83a*\xBDV[\x90Pa\x14%\x85\x82a\x140V[\x92PPP[\x92\x91PPV[`\0a\x0B\n\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x1D\x10V[`\0a\x0B\n\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x1D\x10V[`\0g\x06\xF0[Y\xD3\xB2\0\0\x82\x03a\x14sWP`\0\x91\x90PV[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x12a\x14\x9BW`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\x14\xBCW`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x14\xC9\x83`\x02a)cV[\x90P`\0a\x14\xD6\x82a\x1D/V[\x90P`\0a\x14\xECg\x13\xA0K\xBD\xFD\xC9\xBE\x88\x83a\x1F\xADV[\x90Pa\x06J\x81a*\xD4V[`\0h\x02H\xCE6\xA7\x0C\xB2k>\x19\x82\x13a\x15\x12WP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a\x15YW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x01a\x08oV[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05`\x01`_\x1B\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1Dm\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1Dn\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[`\0a\x0B\n\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x1F\xC2V[a\x16\xF8\x81`@Q`$\x01a\x16\xC9\x91\x90a%\xCEV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\x10L\x13\xEB`\xE2\x1B\x17\x90Ra\x1F\xF0V[PV[`\0a\x0B\n\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x1F\xC2V[a\x17U\x82\x82`@Q`$\x01a\x17&\x92\x91\x90a*\xF0V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c-\x83\x9C\xB3`\xE2\x1B\x17\x90Ra\x1F\xF0V[PPV[`\0\x80g\x13\xA0K\xBD\xFD\xC9\xBE\x88a\x17wg\r\xE0\xB6\xB3\xA7d\0\0\x85a)cV[a\x17\x81\x91\x90a)\xA9V[\x90P`\0a\x17\x8E\x82a*\xD4V[\x90P`\0a\x17\x9B\x82a \x11V[\x90Pg\x1B\xC1mgN\xC8\0\0a\x17\xB8g\r\xE0\xB6\xB3\xA7d\0\0\x83a)cV[a\x06J\x91\x90a)\xA9V[`\0\x82\x85\x10a\x18\x13W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FtradingFunction: invalid x\0\0\0\0\0\0`D\x82\x01R`d\x01a\x08oV[`\0a\x18\"a\x05\xE8\x87\x86a\x14EV[\x90P`\0a\x18@a\x05\xE8\x87a\x18;\x87`\0\x01Q\x89a\x140V[a\x14EV[\x90P`\0a\x18V\x85` \x01Q\x86`@\x01Qa\x13\xFBV[\x90P\x80a\x18c\x83\x85a*bV[a\r{\x91\x90a*bV[`\0\x80`\0\x80`\0\x86\x80` \x01\x90Q\x81\x01\x90a\x18\x89\x91\x90a+\x12V[\x93P\x93P\x93P\x93P\x81a\x18\x9E\x85\x88\x86\x85a\x17\xC2V[a\x0C\x86\x91\x90a)<V[`\0\x84\x86\x11\x15a\x18\xD5W`@Qc0\x82\xDF\xDB`\xE1\x1B\x81R`\x04\x81\x01\x87\x90R`$\x81\x01\x86\x90R`D\x01a\x08oV[`\0a\x18\xE5\x88\x88\x85c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x18\xF7\x89\x88\x86c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x19\x05\x82\x84a)cV[\x13\x15a\x19.W`@Qc\x06\xF1\xBE]`\xE2\x1B\x81R`\x04\x81\x01\x83\x90R`$\x81\x01\x82\x90R`D\x01a\x08oV[`\0a\x19:\x89\x89a*\x1BV[\x90P`\0[`\x02a\x19K\x8A\x8Ca*\x08V[a\x19U\x91\x90a+\x83V[\x94P`\0a\x19g\x8C\x87\x89c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x19u\x86\x83a)cV[\x13a\x19\x82W\x85\x99Pa\x19\x89V[\x85\x9AP\x80\x94P[a\x19\x93\x8B\x8Ba*\x1BV[\x92PP`\x01\x01\x87\x82\x11\x80\x15a\x19\xA7WP\x86\x81\x10[a\x19?WPPPP\x96\x95PPPPPPV[`\0\x80`\0\x80`\0\x86\x80` \x01\x90Q\x81\x01\x90a\x19\xD5\x91\x90a+\x12V[\x93P\x93P\x93P\x93P\x81a\x18\x9E\x85\x85\x89\x85a\x17\xC2V[`\0\x80\x82\x13a\x1A'W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x08oV[`\0``a\x1A4\x84a!\xF5V[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[`\0a\x0B\ng\r\xE0\xB6\xB3\xA7d\0\0\x83a\x1B\xDD\x86a\x19\xEAV[a\x1B\xE7\x91\x90a)cV[a\x1B\xF1\x91\x90a)\xA9V[a\x14\xF7V[a\x16\xF8\x81`@Q`$\x01a\x1C\x0C\x91\x81R` \x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c-[l\xB9`\xE0\x1B\x17\x90Ra\x1F\xF0V[`\0\x80`\0\x80`\0\x86\x80` \x01\x90Q\x81\x01\x90a\x1CW\x91\x90a+\x12V[\x93P\x93P\x93P\x93P\x81a\x18\x9E\x87\x86\x86\x85a\x17\xC2V[`\xB5\x81`\x01`\x88\x1B\x81\x10a\x1C\x85W`@\x91\x90\x91\x1B\x90`\x80\x1C[i\x01\0\0\0\0\0\0\0\0\0\x81\x10a\x1C\xA1W` \x91\x90\x91\x1B\x90`@\x1C[e\x01\0\0\0\0\0\x81\x10a\x1C\xB9W`\x10\x91\x90\x91\x1B\x90` \x1C[c\x01\0\0\0\x81\x10a\x1C\xCFW`\x08\x91\x90\x91\x1B\x90`\x10\x1C[b\x01\0\0\x01\x02`\x12\x1C\x80\x82\x04\x01`\x01\x90\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x90\x81\x90\x04\x81\x11\x90\x03\x90V[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x1D(W`\0\x80\xFD[\x04\x92\x91PPV[`\0\x80\x82\x12\x80a\x1DFWPg\x1B\xC1mgN\xC8\0\0\x82\x13[\x15a\x1DdW`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\x1D\x85W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81g\x1B\xC1mgN\xC8\0\0\x03a\x1D\xADW`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x15a\x1D\xB8W\x91\x90PV[`\0g\r\xE0\xB6\xB3\xA7d\0\0\x83\x12a\x1D\xE0Wa\x1D\xDB\x83g\x1B\xC1mgN\xC8\0\0a)<V[a\x1D\xE2V[\x82[\x90P`\0a\x1D\xF8\x82g\x1B\xC1mgN\xC8\0\0a\"\x9DV[\x90P\x80`\0\x03a\x1E\x1BW`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x1E&\x82a\x19\xEAV[\x90P`\0c;\x9A\xCA\0a\x1EQa\x1ELa\x1EFg\x1B\xC1mgN\xC8\0\0a*\xD4V[\x85a\x1F\xADV[a\x1ClV[a\x1E[\x91\x90a)cV[\x90P`\0\x80a\x1Er\x83g\x03\xC1f\\z\xAB \0a\x1F\xADV[a\x1E\x84\x90g \x05\xFEO&\x8E\xA0\0a*bV[\x90P`\0a\x1E\xB4\x84a\x1E\x9D\x86f\x9F2u$b\xA0\0a\x1F\xADV[a\x1E\xAF\x90g\r\xC5R\x7Fd, \0a*bV[a\x1F\xADV[a\x1E\xC6\x90g\r\xE0\xB6\xB3\xA7d\0\0a*bV[\x90Pa\x1E\xEAg\t\xD0(\xCCo _\xFF\x19\x85a\x1E\xE0\x85\x85a\"\x9DV[a\x1E\xAF\x91\x90a)<V[\x92PPP`\0[`\x02\x81\x10\x15a\x1F\x85W`\0\x86a\x1F\x06\x84a \x11V[a\x1F\x10\x91\x90a)<V[\x90P`\0a\x1F\x1E\x84\x85a\x1F\xADV[a\x1F'\x90a*\xD4V[\x90P`\0a\x1F4\x82a\x14\xF7V[\x90P`\0a\x1FB\x86\x85a\x1F\xADV[a\x1FTg\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x84a\x1F\xADV[a\x1F^\x91\x90a)<V[\x90Pa\x1Fj\x84\x82a\"\x9DV[a\x1Ft\x90\x87a*bV[\x95P\x84`\x01\x01\x94PPPPPa\x1E\xF1V[g\r\xE0\xB6\xB3\xA7d\0\0\x88\x12a\x1F\xA2Wa\x1F\x9D\x82a*\xD4V[a\r{V[P\x96\x95PPPPPPV[`\0a\x0B\n\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\"\xAEV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x1F\xDAW`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[\x80Qjconsole.log` \x83\x01`\0\x80\x84\x83\x85Z\xFAPPPPPV[`\0\x81`\0\x03a *WPg\r\xE0\xB6\xB3\xA7d\0\0\x91\x90PV[gV\x98\xEE\xF0fp\0\0\x82\x12a AWP`\0\x91\x90PV[a RgV\x98\xEE\xF0fp\0\0a*\xD4V[\x82\x13a gWPg\x1B\xC1mgN\xC8\0\0\x91\x90PV[`\0a r\x83a\"\xCDV[\x90P`\0a \xABg\r\xE0\xB6\xB3\xA7d\0\0a \x94\x84g\x1B\xC1mgN\xC8\0\0a\x14EV[a \xA6\x90g\r\xE0\xB6\xB3\xA7d\0\0a*bV[a\"\x9DV[\x90P`\0\x80\x82a!\x07\x81a \xF4\x81a \xE2\x81a \xCF\x81g\x02_\x0F\xE1\x05\xA3\x14\0a\x1F\xADV[a\x1E\xAF\x90g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19a*bV[a\x1E\xAF\x90g\x14\xA8EL\x19\xE1\xAC\0a*bV[a\x1E\xAF\x90g\x0F\xC1\x0E\x01W\x82w\xFF\x19a*bV[a!\x19\x90g\x03\xDE\xBD\x08;\x8C|\0a*bV[\x91P\x83\x90Pa!\x81\x81a!o\x81a!]\x81a!K\x81a!8\x81\x8Ba\x1F\xADV[a\x1E\xAF\x90g\x02\x95\xD4\0\xEA2W\xFF\x19a*bV[a\x1E\xAF\x90g\x01W\xD8\xB2\xEC\xC7\x08\0a*bV[a\x1E\xAF\x90g\x051\n\xA7\xD5!0\0a*bV[a\x1E\xAF\x90g\r\xE0\xCC=\x15a\0\0a*bV[\x91P\x81g\x11\x90\0\xAB\x10\x0F\xFC\0a!\x97\x87\x88a\x1F\xADV[a!\xA3\x90`\0\x19a)cV[a!\xAD\x91\x90a)<V[a!\xB7\x91\x90a*bV[\x92PP`\0a!\xC5\x83a\x14\xF7V[\x90P`\0a!\xD3\x85\x83a\x1F\xADV[\x90P`\0\x88\x12a!\xE3W\x80a\r{V[a\r{\x81g\x1B\xC1mgN\xC8\0\0a)<V[`\0\x80\x82\x11a\"2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x08oV[P`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x82\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[`\0a\x0B\n\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x05\x85\x14\x17\x16a\"\xC6W`\0\x80\xFD[\x05\x92\x91PPV[`\0`\x01`\xFF\x1B\x82\x03a\"\xF3W`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x12\x15a\x0BVWP\x19`\x01\x01\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a#\xBFWa#\xBFa#\x04V[PP\x835\x95` \x85\x015\x95P`@\x85\x015\x94``\x81\x015\x94P`\x80\x015\x92P\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a#\xF8Wa#\xF8a#\x04V[\x825\x80\x15\x15\x81\x14a$\x08W`\0\x80\xFD[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a$<W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a$ V[P`\0` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[\x84\x15\x15\x81R\x83` \x82\x01R\x82`@\x82\x01R`\x80``\x82\x01R`\0a$\x83`\x80\x83\x01\x84a$\x16V[\x96\x95PPPPPPV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FABI decoding: struct data too sh`D\x82\x01Rb\x1B\xDC\x9D`\xEA\x1B`d\x82\x01R`\x84\x81\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a%\x17Wa%\x17a$\xDEV[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a%FWa%Fa$\xDEV[`@R\x91\x90PV[`\0``\x82\x84\x03\x12\x15a%cWa%ca$\x8DV[a%ka$\xF4V[\x90P\x815\x81R` \x82\x015` \x82\x01R`@\x82\x015`@\x82\x01R\x92\x91PPV[`\0\x80`\0\x80`\xC0\x85\x87\x03\x12\x15a%\xA4Wa%\xA4a#\x04V[\x845\x93P` \x85\x015\x92P`@\x85\x015\x91Pa%\xC3\x86``\x87\x01a%NV[\x90P\x92\x95\x91\x94P\x92PV[` \x81R`\0a\x0B\n` \x83\x01\x84a$\x16V[`\0\x80`\0`\xA0\x84\x86\x03\x12\x15a%\xF9Wa%\xF9a#\x04V[\x835\x92P` \x84\x015\x91Pa&\x11\x85`@\x86\x01a%NV[\x90P\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a&/Wa&/a#\x04V[P5\x91\x90PV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x81\xFD[`\0\x80` \x83\x85\x03\x12\x15a&\xA5Wa&\xA5a#\x04V[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a&\xC0Wa&\xC0a#TV[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a&\xD7Wa&\xD7a&6V[\x815\x81\x81\x11\x15a':W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x0ENL/$\r\x8C\xAD\xCC\xEE\x8D`\xAB\x1B`d\x82\x01R`\x84\x81\xFD[\x86` \x82\x85\x01\x01\x11\x15a'\xA2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rjrray stride`\xA8\x1B`d\x82\x01R\x91P`\x84\x82\xFD[` \x92\x90\x92\x01\x96\x91\x95P\x90\x93PPPPV[`\0\x80`\x80\x83\x85\x03\x12\x15a'\xCAWa'\xCAa#\x04V[\x825\x91Pa'\xDB\x84` \x85\x01a%NV[\x90P\x92P\x92\x90PV[`\0` \x80\x83\x85\x03\x12\x15a'\xFAWa'\xFAa#\x04V[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a(\x15Wa(\x15a#TV[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a(,Wa(,a&6V[\x815\x81\x81\x11\x15a(>Wa(>a$\xDEV[a(P`\x1F\x82\x01`\x1F\x19\x16\x85\x01a%\x1DV[\x91P\x80\x82R\x86\x84\x82\x85\x01\x01\x11\x15a(\xB6W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x85\x90R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R`\x84\x81\xFD[\x80\x84\x84\x01\x85\x84\x017`\0\x90\x82\x01\x90\x93\x01\x92\x90\x92RP\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a(\xE8Wa(\xE8a#\x04V[PP\x805\x92` \x90\x91\x015\x91PV[`\0\x80`\0``\x84\x86\x03\x12\x15a)\x0FWa)\x0Fa#\x04V[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a)\\Wa)\\a)&V[P\x92\x91PPV[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15a)\x7FWa)\x7Fa)&V[\x81\x81\x05\x83\x14\x82\x15\x17a\x14*Wa\x14*a)&V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a)\xB8Wa)\xB8a)\x93V[`\x01`\xFF\x1B\x82\x14`\0\x19\x84\x14\x16\x15a)\xD2Wa)\xD2a)&V[P\x05\x90V[`\0\x80`\0``\x84\x86\x03\x12\x15a)\xEFWa)\xEFa#\x04V[\x83Q\x92P` \x84\x01Q\x91P`@\x84\x01Q\x90P\x92P\x92P\x92V[\x80\x82\x01\x80\x82\x11\x15a\x14*Wa\x14*a)&V[\x81\x81\x03\x81\x81\x11\x15a\x14*Wa\x14*a)&V[\x84\x81R` \x80\x82\x01\x85\x90R`@\x80\x83\x01\x85\x90R\x83Q``\x84\x01R\x90\x83\x01Q`\x80\x83\x01R\x82\x01Q`\xA0\x82\x01R`\xC0\x81\x01a\x06JV[\x80\x82\x01\x82\x81\x12`\0\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a*\x82Wa*\x82a)&V[PP\x92\x91PPV[`\0`\x01`\x01`\xFF\x1B\x01\x82\x01a*\xA2Wa*\xA2a)&V[P`\x01\x01\x90V[`\0\x82a*\xB8Wa*\xB8a)\x93V[P\x07\x90V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x14*Wa\x14*a)&V[`\0`\x01`\xFF\x1B\x82\x01a*\xE9Wa*\xE9a)&V[P`\0\x03\x90V[`@\x81R`\0a+\x03`@\x83\x01\x85a$\x16V[\x90P\x82` \x83\x01R\x93\x92PPPV[`\0\x80`\0\x80\x84\x86\x03`\xC0\x81\x12\x15a+,Wa+,a#\x04V[\x85Q\x94P` \x86\x01Q\x93P`@\x86\x01Q\x92P```_\x19\x82\x01\x12\x15a+SWa+Sa$\x8DV[Pa+\\a$\xF4V[``\x86\x01Q\x81R`\x80\x86\x01Q` \x82\x01R`\xA0\x90\x95\x01Q`@\x86\x01RP\x91\x94\x90\x93P\x90\x91\x90V[`\0\x82a+\x92Wa+\x92a)\x93V[P\x04\x90V\xFEMake sure the calling contract of this console.log is the Core contractliquidity delta - min liquidity delta";
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
        ///Calls the contract's `APPROXIMATED_MINIMUM_X_INPUT` (0x84c82a26) function
        pub fn approximated_minimum_x_input(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([132, 200, 42, 38], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `BISECTION_EPSILON` (0x6d652299) function
        pub fn bisection_epsilon(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([109, 101, 34, 153], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `HALF_WAD` (0x85aa454e) function
        pub fn half_wad(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([133, 170, 69, 78], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `INFINITY_IS_NOT_REAL` (0xe3d0dca5) function
        pub fn infinity_is_not_real(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([227, 208, 220, 165], ())
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
        ///Calls the contract's `TWO_WAD` (0x275d67c8) function
        pub fn two_wad(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([39, 93, 103, 200], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `WAD` (0x6a146024) function
        pub fn wad(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([106, 20, 96, 36], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ZERO` (0x58fa63ca) function
        pub fn zero(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([88, 250, 99, 202], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `computeD1` (0x9716aebb) function
        pub fn compute_d1(
            &self,
            price_wad: ::ethers::core::types::U256,
            params: Parameters,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([151, 22, 174, 187], (price_wad, params))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `computeD2` (0xd7f97bef) function
        pub fn compute_d2(
            &self,
            price_wad: ::ethers::core::types::U256,
            params: Parameters,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([215, 249, 123, 239], (price_wad, params))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `computeHalfSigmaSquared` (0xa6d33498) function
        pub fn compute_half_sigma_squared(
            &self,
            sigma_percent_wad: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([166, 211, 52, 152], sigma_percent_wad)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `computePrice` (0x0625a623) function
        pub fn compute_price(
            &self,
            reserve_x_wad: ::ethers::core::types::U256,
            total_liquidity: ::ethers::core::types::U256,
            strike_price_wad: ::ethers::core::types::U256,
            sigma_percent_wad: ::ethers::core::types::U256,
            tau_years_wad: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [6, 37, 166, 35],
                    (
                        reserve_x_wad,
                        total_liquidity,
                        strike_price_wad,
                        sigma_percent_wad,
                        tau_years_wad,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `computeSwapConstant` (0xa19cd3d1) function
        pub fn compute_swap_constant(
            &self,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([161, 156, 211, 209], data)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `encodeInitData` (0x0f0aa395) function
        pub fn encode_init_data(
            &self,
            reserve_x_wad: ::ethers::core::types::U256,
            reseve_y_wad: ::ethers::core::types::U256,
            total_liquidity: ::ethers::core::types::U256,
            params: Parameters,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash(
                    [15, 10, 163, 149],
                    (reserve_x_wad, reseve_y_wad, total_liquidity, params),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `encodeValidateData` (0xbd422d28) function
        pub fn encode_validate_data(
            &self,
            adjusted_reserve_x_wad: ::ethers::core::types::U256,
            adjusted_reserve_y_wad: ::ethers::core::types::U256,
            adjusted_liquidity: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash(
                    [189, 66, 45, 40],
                    (adjusted_reserve_x_wad, adjusted_reserve_y_wad, adjusted_liquidity),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `findLiquidity` (0x883b6dc5) function
        pub fn find_liquidity(
            &self,
            reserve_x_wad: ::ethers::core::types::U256,
            reserve_y_wad: ::ethers::core::types::U256,
            swap_constant: ::ethers::core::types::I256,
            params: Parameters,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [136, 59, 109, 197],
                    (reserve_x_wad, reserve_y_wad, swap_constant, params),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `findX` (0xcd64aea2) function
        pub fn find_x(
            &self,
            reserve_y_wad: ::ethers::core::types::U256,
            liquidity: ::ethers::core::types::U256,
            swap_constant: ::ethers::core::types::I256,
            params: Parameters,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [205, 100, 174, 162],
                    (reserve_y_wad, liquidity, swap_constant, params),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `findY` (0x6417d4b5) function
        pub fn find_y(
            &self,
            reserve_x_wad: ::ethers::core::types::U256,
            liquidity: ::ethers::core::types::U256,
            swap_constant: ::ethers::core::types::I256,
            params: Parameters,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [100, 23, 212, 181],
                    (reserve_x_wad, liquidity, swap_constant, params),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `init` (0x4ddf47d4) function
        pub fn init(
            &self,
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
                .method_hash([77, 223, 71, 212], data)
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
        ///Calls the contract's `lx` (0x9d819e83) function
        pub fn lx(
            &self,
            reserve_x_wad: ::ethers::core::types::U256,
            price_wad: ::ethers::core::types::U256,
            params: Parameters,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([157, 129, 158, 131], (reserve_x_wad, price_wad, params))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ly` (0x5608bea1) function
        pub fn ly(
            &self,
            reserve_y_wad: ::ethers::core::types::U256,
            price_wad: ::ethers::core::types::U256,
            params: Parameters,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([86, 8, 190, 161], (reserve_y_wad, price_wad, params))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mulidiv` (0xe4937428) function
        pub fn mulidiv(
            &self,
            x: ::ethers::core::types::I256,
            y: ::ethers::core::types::I256,
            denominator: ::ethers::core::types::I256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([228, 147, 116, 40], (x, y, denominator))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mulidivUp` (0xffb36266) function
        pub fn mulidiv_up(
            &self,
            x: ::ethers::core::types::I256,
            y: ::ethers::core::types::I256,
            denominator: ::ethers::core::types::I256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([255, 179, 98, 102], (x, y, denominator))
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
        ///Calls the contract's `slot` (0x1a88bc66) function
        pub fn slot(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([26, 136, 188, 102], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `swapFeePercentageWad` (0x40b43169) function
        pub fn swap_fee_percentage_wad(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([64, 180, 49, 105], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `toUint` (0x1d9cf722) function
        pub fn to_uint(
            &self,
            x: ::ethers::core::types::I256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([29, 156, 247, 34], x)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `validate` (0xc16e50ef) function
        pub fn validate(
            &self,
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
                .method_hash([193, 110, 80, 239], data)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `xl` (0x184d30ba) function
        pub fn xl(
            &self,
            total_liquidity: ::ethers::core::types::U256,
            price_wad: ::ethers::core::types::U256,
            params: Parameters,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([24, 77, 48, 186], (total_liquidity, price_wad, params))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `yl` (0xdbaf1142) function
        pub fn yl(
            &self,
            total_liquidity: ::ethers::core::types::U256,
            price_wad: ::ethers::core::types::U256,
            params: Parameters,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([219, 175, 17, 66], (total_liquidity, price_wad, params))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for LogNormal<M> {
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
    pub enum LogNormalErrors {
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
    impl ::ethers::core::abi::AbiEncode for LogNormalErrors {
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
    impl ::ethers::contract::ContractRevert for LogNormalErrors {
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
    impl ::core::fmt::Display for LogNormalErrors {
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
    impl ::core::convert::From<::std::string::String> for LogNormalErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<BisectionLib_InvalidBounds> for LogNormalErrors {
        fn from(value: BisectionLib_InvalidBounds) -> Self {
            Self::BisectionLib_InvalidBounds(value)
        }
    }
    impl ::core::convert::From<BisectionLib_RootOutsideBounds> for LogNormalErrors {
        fn from(value: BisectionLib_RootOutsideBounds) -> Self {
            Self::BisectionLib_RootOutsideBounds(value)
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
    ///Container type for all input parameters for the `APPROXIMATED_MINIMUM_X_INPUT` function with signature `APPROXIMATED_MINIMUM_X_INPUT()` and selector `0x84c82a26`
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
        name = "APPROXIMATED_MINIMUM_X_INPUT",
        abi = "APPROXIMATED_MINIMUM_X_INPUT()"
    )]
    pub struct ApproximatedMinimumXInputCall;
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
    ///Container type for all input parameters for the `HALF_WAD` function with signature `HALF_WAD()` and selector `0x85aa454e`
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
    #[ethcall(name = "HALF_WAD", abi = "HALF_WAD()")]
    pub struct HalfWadCall;
    ///Container type for all input parameters for the `INFINITY_IS_NOT_REAL` function with signature `INFINITY_IS_NOT_REAL()` and selector `0xe3d0dca5`
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
    #[ethcall(name = "INFINITY_IS_NOT_REAL", abi = "INFINITY_IS_NOT_REAL()")]
    pub struct InfinityIsNotRealCall;
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
    ///Container type for all input parameters for the `TWO_WAD` function with signature `TWO_WAD()` and selector `0x275d67c8`
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
    #[ethcall(name = "TWO_WAD", abi = "TWO_WAD()")]
    pub struct TwoWadCall;
    ///Container type for all input parameters for the `WAD` function with signature `WAD()` and selector `0x6a146024`
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
    #[ethcall(name = "WAD", abi = "WAD()")]
    pub struct WadCall;
    ///Container type for all input parameters for the `ZERO` function with signature `ZERO()` and selector `0x58fa63ca`
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
    #[ethcall(name = "ZERO", abi = "ZERO()")]
    pub struct ZeroCall;
    ///Container type for all input parameters for the `computeD1` function with signature `computeD1(uint256,(uint256,uint256,uint256))` and selector `0x9716aebb`
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
    #[ethcall(name = "computeD1", abi = "computeD1(uint256,(uint256,uint256,uint256))")]
    pub struct ComputeD1Call {
        pub price_wad: ::ethers::core::types::U256,
        pub params: Parameters,
    }
    ///Container type for all input parameters for the `computeD2` function with signature `computeD2(uint256,(uint256,uint256,uint256))` and selector `0xd7f97bef`
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
    #[ethcall(name = "computeD2", abi = "computeD2(uint256,(uint256,uint256,uint256))")]
    pub struct ComputeD2Call {
        pub price_wad: ::ethers::core::types::U256,
        pub params: Parameters,
    }
    ///Container type for all input parameters for the `computeHalfSigmaSquared` function with signature `computeHalfSigmaSquared(uint256)` and selector `0xa6d33498`
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
        name = "computeHalfSigmaSquared",
        abi = "computeHalfSigmaSquared(uint256)"
    )]
    pub struct ComputeHalfSigmaSquaredCall {
        pub sigma_percent_wad: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `computePrice` function with signature `computePrice(uint256,uint256,uint256,uint256,uint256)` and selector `0x0625a623`
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
        name = "computePrice",
        abi = "computePrice(uint256,uint256,uint256,uint256,uint256)"
    )]
    pub struct ComputePriceCall {
        pub reserve_x_wad: ::ethers::core::types::U256,
        pub total_liquidity: ::ethers::core::types::U256,
        pub strike_price_wad: ::ethers::core::types::U256,
        pub sigma_percent_wad: ::ethers::core::types::U256,
        pub tau_years_wad: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `computeSwapConstant` function with signature `computeSwapConstant(bytes)` and selector `0xa19cd3d1`
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
    #[ethcall(name = "computeSwapConstant", abi = "computeSwapConstant(bytes)")]
    pub struct ComputeSwapConstantCall {
        pub data: ::ethers::core::types::Bytes,
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
        pub reseve_y_wad: ::ethers::core::types::U256,
        pub total_liquidity: ::ethers::core::types::U256,
        pub params: Parameters,
    }
    ///Container type for all input parameters for the `encodeValidateData` function with signature `encodeValidateData(uint256,uint256,uint256)` and selector `0xbd422d28`
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
        name = "encodeValidateData",
        abi = "encodeValidateData(uint256,uint256,uint256)"
    )]
    pub struct EncodeValidateDataCall {
        pub adjusted_reserve_x_wad: ::ethers::core::types::U256,
        pub adjusted_reserve_y_wad: ::ethers::core::types::U256,
        pub adjusted_liquidity: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `findLiquidity` function with signature `findLiquidity(uint256,uint256,int256,(uint256,uint256,uint256))` and selector `0x883b6dc5`
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
        name = "findLiquidity",
        abi = "findLiquidity(uint256,uint256,int256,(uint256,uint256,uint256))"
    )]
    pub struct FindLiquidityCall {
        pub reserve_x_wad: ::ethers::core::types::U256,
        pub reserve_y_wad: ::ethers::core::types::U256,
        pub swap_constant: ::ethers::core::types::I256,
        pub params: Parameters,
    }
    ///Container type for all input parameters for the `findX` function with signature `findX(uint256,uint256,int256,(uint256,uint256,uint256))` and selector `0xcd64aea2`
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
        name = "findX",
        abi = "findX(uint256,uint256,int256,(uint256,uint256,uint256))"
    )]
    pub struct FindXCall {
        pub reserve_y_wad: ::ethers::core::types::U256,
        pub liquidity: ::ethers::core::types::U256,
        pub swap_constant: ::ethers::core::types::I256,
        pub params: Parameters,
    }
    ///Container type for all input parameters for the `findY` function with signature `findY(uint256,uint256,int256,(uint256,uint256,uint256))` and selector `0x6417d4b5`
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
        name = "findY",
        abi = "findY(uint256,uint256,int256,(uint256,uint256,uint256))"
    )]
    pub struct FindYCall {
        pub reserve_x_wad: ::ethers::core::types::U256,
        pub liquidity: ::ethers::core::types::U256,
        pub swap_constant: ::ethers::core::types::I256,
        pub params: Parameters,
    }
    ///Container type for all input parameters for the `init` function with signature `init(bytes)` and selector `0x4ddf47d4`
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
    #[ethcall(name = "init", abi = "init(bytes)")]
    pub struct InitCall {
        pub data: ::ethers::core::types::Bytes,
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
    ///Container type for all input parameters for the `lx` function with signature `lx(uint256,uint256,(uint256,uint256,uint256))` and selector `0x9d819e83`
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
    #[ethcall(name = "lx", abi = "lx(uint256,uint256,(uint256,uint256,uint256))")]
    pub struct LxCall {
        pub reserve_x_wad: ::ethers::core::types::U256,
        pub price_wad: ::ethers::core::types::U256,
        pub params: Parameters,
    }
    ///Container type for all input parameters for the `ly` function with signature `ly(uint256,uint256,(uint256,uint256,uint256))` and selector `0x5608bea1`
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
    #[ethcall(name = "ly", abi = "ly(uint256,uint256,(uint256,uint256,uint256))")]
    pub struct LyCall {
        pub reserve_y_wad: ::ethers::core::types::U256,
        pub price_wad: ::ethers::core::types::U256,
        pub params: Parameters,
    }
    ///Container type for all input parameters for the `mulidiv` function with signature `mulidiv(int256,int256,int256)` and selector `0xe4937428`
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
    #[ethcall(name = "mulidiv", abi = "mulidiv(int256,int256,int256)")]
    pub struct MulidivCall {
        pub x: ::ethers::core::types::I256,
        pub y: ::ethers::core::types::I256,
        pub denominator: ::ethers::core::types::I256,
    }
    ///Container type for all input parameters for the `mulidivUp` function with signature `mulidivUp(int256,int256,int256)` and selector `0xffb36266`
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
    #[ethcall(name = "mulidivUp", abi = "mulidivUp(int256,int256,int256)")]
    pub struct MulidivUpCall {
        pub x: ::ethers::core::types::I256,
        pub y: ::ethers::core::types::I256,
        pub denominator: ::ethers::core::types::I256,
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
    ///Container type for all input parameters for the `slot` function with signature `slot()` and selector `0x1a88bc66`
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
    #[ethcall(name = "slot", abi = "slot()")]
    pub struct SlotCall;
    ///Container type for all input parameters for the `swapFeePercentageWad` function with signature `swapFeePercentageWad()` and selector `0x40b43169`
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
    #[ethcall(name = "swapFeePercentageWad", abi = "swapFeePercentageWad()")]
    pub struct SwapFeePercentageWadCall;
    ///Container type for all input parameters for the `toUint` function with signature `toUint(int256)` and selector `0x1d9cf722`
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
    #[ethcall(name = "toUint", abi = "toUint(int256)")]
    pub struct ToUintCall {
        pub x: ::ethers::core::types::I256,
    }
    ///Container type for all input parameters for the `validate` function with signature `validate(bytes)` and selector `0xc16e50ef`
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
    #[ethcall(name = "validate", abi = "validate(bytes)")]
    pub struct ValidateCall {
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `xl` function with signature `xl(uint256,uint256,(uint256,uint256,uint256))` and selector `0x184d30ba`
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
    #[ethcall(name = "xl", abi = "xl(uint256,uint256,(uint256,uint256,uint256))")]
    pub struct XlCall {
        pub total_liquidity: ::ethers::core::types::U256,
        pub price_wad: ::ethers::core::types::U256,
        pub params: Parameters,
    }
    ///Container type for all input parameters for the `yl` function with signature `yl(uint256,uint256,(uint256,uint256,uint256))` and selector `0xdbaf1142`
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
    #[ethcall(name = "yl", abi = "yl(uint256,uint256,(uint256,uint256,uint256))")]
    pub struct YlCall {
        pub total_liquidity: ::ethers::core::types::U256,
        pub price_wad: ::ethers::core::types::U256,
        pub params: Parameters,
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
        ApproximatedMinimumXInput(ApproximatedMinimumXInputCall),
        BisectionEpsilon(BisectionEpsilonCall),
        HalfWad(HalfWadCall),
        InfinityIsNotReal(InfinityIsNotRealCall),
        MaxBisectionIters(MaxBisectionItersCall),
        TwoWad(TwoWadCall),
        Wad(WadCall),
        Zero(ZeroCall),
        ComputeD1(ComputeD1Call),
        ComputeD2(ComputeD2Call),
        ComputeHalfSigmaSquared(ComputeHalfSigmaSquaredCall),
        ComputePrice(ComputePriceCall),
        ComputeSwapConstant(ComputeSwapConstantCall),
        EncodeInitData(EncodeInitDataCall),
        EncodeValidateData(EncodeValidateDataCall),
        FindLiquidity(FindLiquidityCall),
        FindX(FindXCall),
        FindY(FindYCall),
        Init(InitCall),
        InternalPrice(InternalPriceCall),
        Lx(LxCall),
        Ly(LyCall),
        Mulidiv(MulidivCall),
        MulidivUp(MulidivUpCall),
        SimulateSwap(SimulateSwapCall),
        Slot(SlotCall),
        SwapFeePercentageWad(SwapFeePercentageWadCall),
        ToUint(ToUintCall),
        Validate(ValidateCall),
        Xl(XlCall),
        Yl(YlCall),
    }
    impl ::ethers::core::abi::AbiDecode for LogNormalCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <ApproximatedMinimumXInputCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ApproximatedMinimumXInput(decoded));
            }
            if let Ok(decoded) = <BisectionEpsilonCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BisectionEpsilon(decoded));
            }
            if let Ok(decoded) = <HalfWadCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::HalfWad(decoded));
            }
            if let Ok(decoded) = <InfinityIsNotRealCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InfinityIsNotReal(decoded));
            }
            if let Ok(decoded) = <MaxBisectionItersCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MaxBisectionIters(decoded));
            }
            if let Ok(decoded) = <TwoWadCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TwoWad(decoded));
            }
            if let Ok(decoded) = <WadCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Wad(decoded));
            }
            if let Ok(decoded) = <ZeroCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Zero(decoded));
            }
            if let Ok(decoded) = <ComputeD1Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ComputeD1(decoded));
            }
            if let Ok(decoded) = <ComputeD2Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ComputeD2(decoded));
            }
            if let Ok(decoded) = <ComputeHalfSigmaSquaredCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ComputeHalfSigmaSquared(decoded));
            }
            if let Ok(decoded) = <ComputePriceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ComputePrice(decoded));
            }
            if let Ok(decoded) = <ComputeSwapConstantCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ComputeSwapConstant(decoded));
            }
            if let Ok(decoded) = <EncodeInitDataCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EncodeInitData(decoded));
            }
            if let Ok(decoded) = <EncodeValidateDataCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EncodeValidateData(decoded));
            }
            if let Ok(decoded) = <FindLiquidityCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FindLiquidity(decoded));
            }
            if let Ok(decoded) = <FindXCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FindX(decoded));
            }
            if let Ok(decoded) = <FindYCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FindY(decoded));
            }
            if let Ok(decoded) = <InitCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Init(decoded));
            }
            if let Ok(decoded) = <InternalPriceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InternalPrice(decoded));
            }
            if let Ok(decoded) = <LxCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Lx(decoded));
            }
            if let Ok(decoded) = <LyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Ly(decoded));
            }
            if let Ok(decoded) = <MulidivCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Mulidiv(decoded));
            }
            if let Ok(decoded) = <MulidivUpCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MulidivUp(decoded));
            }
            if let Ok(decoded) = <SimulateSwapCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SimulateSwap(decoded));
            }
            if let Ok(decoded) = <SlotCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Slot(decoded));
            }
            if let Ok(decoded) = <SwapFeePercentageWadCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SwapFeePercentageWad(decoded));
            }
            if let Ok(decoded) = <ToUintCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ToUint(decoded));
            }
            if let Ok(decoded) = <ValidateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Validate(decoded));
            }
            if let Ok(decoded) = <XlCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Xl(decoded));
            }
            if let Ok(decoded) = <YlCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Yl(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for LogNormalCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::ApproximatedMinimumXInput(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BisectionEpsilon(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HalfWad(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InfinityIsNotReal(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MaxBisectionIters(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TwoWad(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Wad(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Zero(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ComputeD1(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ComputeD2(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ComputeHalfSigmaSquared(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ComputePrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ComputeSwapConstant(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EncodeInitData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EncodeValidateData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FindLiquidity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FindX(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::FindY(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Init(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InternalPrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Lx(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Ly(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Mulidiv(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MulidivUp(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SimulateSwap(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Slot(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SwapFeePercentageWad(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ToUint(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Validate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Xl(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Yl(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for LogNormalCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ApproximatedMinimumXInput(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BisectionEpsilon(element) => ::core::fmt::Display::fmt(element, f),
                Self::HalfWad(element) => ::core::fmt::Display::fmt(element, f),
                Self::InfinityIsNotReal(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaxBisectionIters(element) => ::core::fmt::Display::fmt(element, f),
                Self::TwoWad(element) => ::core::fmt::Display::fmt(element, f),
                Self::Wad(element) => ::core::fmt::Display::fmt(element, f),
                Self::Zero(element) => ::core::fmt::Display::fmt(element, f),
                Self::ComputeD1(element) => ::core::fmt::Display::fmt(element, f),
                Self::ComputeD2(element) => ::core::fmt::Display::fmt(element, f),
                Self::ComputeHalfSigmaSquared(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ComputePrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::ComputeSwapConstant(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EncodeInitData(element) => ::core::fmt::Display::fmt(element, f),
                Self::EncodeValidateData(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FindLiquidity(element) => ::core::fmt::Display::fmt(element, f),
                Self::FindX(element) => ::core::fmt::Display::fmt(element, f),
                Self::FindY(element) => ::core::fmt::Display::fmt(element, f),
                Self::Init(element) => ::core::fmt::Display::fmt(element, f),
                Self::InternalPrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::Lx(element) => ::core::fmt::Display::fmt(element, f),
                Self::Ly(element) => ::core::fmt::Display::fmt(element, f),
                Self::Mulidiv(element) => ::core::fmt::Display::fmt(element, f),
                Self::MulidivUp(element) => ::core::fmt::Display::fmt(element, f),
                Self::SimulateSwap(element) => ::core::fmt::Display::fmt(element, f),
                Self::Slot(element) => ::core::fmt::Display::fmt(element, f),
                Self::SwapFeePercentageWad(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ToUint(element) => ::core::fmt::Display::fmt(element, f),
                Self::Validate(element) => ::core::fmt::Display::fmt(element, f),
                Self::Xl(element) => ::core::fmt::Display::fmt(element, f),
                Self::Yl(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ApproximatedMinimumXInputCall> for LogNormalCalls {
        fn from(value: ApproximatedMinimumXInputCall) -> Self {
            Self::ApproximatedMinimumXInput(value)
        }
    }
    impl ::core::convert::From<BisectionEpsilonCall> for LogNormalCalls {
        fn from(value: BisectionEpsilonCall) -> Self {
            Self::BisectionEpsilon(value)
        }
    }
    impl ::core::convert::From<HalfWadCall> for LogNormalCalls {
        fn from(value: HalfWadCall) -> Self {
            Self::HalfWad(value)
        }
    }
    impl ::core::convert::From<InfinityIsNotRealCall> for LogNormalCalls {
        fn from(value: InfinityIsNotRealCall) -> Self {
            Self::InfinityIsNotReal(value)
        }
    }
    impl ::core::convert::From<MaxBisectionItersCall> for LogNormalCalls {
        fn from(value: MaxBisectionItersCall) -> Self {
            Self::MaxBisectionIters(value)
        }
    }
    impl ::core::convert::From<TwoWadCall> for LogNormalCalls {
        fn from(value: TwoWadCall) -> Self {
            Self::TwoWad(value)
        }
    }
    impl ::core::convert::From<WadCall> for LogNormalCalls {
        fn from(value: WadCall) -> Self {
            Self::Wad(value)
        }
    }
    impl ::core::convert::From<ZeroCall> for LogNormalCalls {
        fn from(value: ZeroCall) -> Self {
            Self::Zero(value)
        }
    }
    impl ::core::convert::From<ComputeD1Call> for LogNormalCalls {
        fn from(value: ComputeD1Call) -> Self {
            Self::ComputeD1(value)
        }
    }
    impl ::core::convert::From<ComputeD2Call> for LogNormalCalls {
        fn from(value: ComputeD2Call) -> Self {
            Self::ComputeD2(value)
        }
    }
    impl ::core::convert::From<ComputeHalfSigmaSquaredCall> for LogNormalCalls {
        fn from(value: ComputeHalfSigmaSquaredCall) -> Self {
            Self::ComputeHalfSigmaSquared(value)
        }
    }
    impl ::core::convert::From<ComputePriceCall> for LogNormalCalls {
        fn from(value: ComputePriceCall) -> Self {
            Self::ComputePrice(value)
        }
    }
    impl ::core::convert::From<ComputeSwapConstantCall> for LogNormalCalls {
        fn from(value: ComputeSwapConstantCall) -> Self {
            Self::ComputeSwapConstant(value)
        }
    }
    impl ::core::convert::From<EncodeInitDataCall> for LogNormalCalls {
        fn from(value: EncodeInitDataCall) -> Self {
            Self::EncodeInitData(value)
        }
    }
    impl ::core::convert::From<EncodeValidateDataCall> for LogNormalCalls {
        fn from(value: EncodeValidateDataCall) -> Self {
            Self::EncodeValidateData(value)
        }
    }
    impl ::core::convert::From<FindLiquidityCall> for LogNormalCalls {
        fn from(value: FindLiquidityCall) -> Self {
            Self::FindLiquidity(value)
        }
    }
    impl ::core::convert::From<FindXCall> for LogNormalCalls {
        fn from(value: FindXCall) -> Self {
            Self::FindX(value)
        }
    }
    impl ::core::convert::From<FindYCall> for LogNormalCalls {
        fn from(value: FindYCall) -> Self {
            Self::FindY(value)
        }
    }
    impl ::core::convert::From<InitCall> for LogNormalCalls {
        fn from(value: InitCall) -> Self {
            Self::Init(value)
        }
    }
    impl ::core::convert::From<InternalPriceCall> for LogNormalCalls {
        fn from(value: InternalPriceCall) -> Self {
            Self::InternalPrice(value)
        }
    }
    impl ::core::convert::From<LxCall> for LogNormalCalls {
        fn from(value: LxCall) -> Self {
            Self::Lx(value)
        }
    }
    impl ::core::convert::From<LyCall> for LogNormalCalls {
        fn from(value: LyCall) -> Self {
            Self::Ly(value)
        }
    }
    impl ::core::convert::From<MulidivCall> for LogNormalCalls {
        fn from(value: MulidivCall) -> Self {
            Self::Mulidiv(value)
        }
    }
    impl ::core::convert::From<MulidivUpCall> for LogNormalCalls {
        fn from(value: MulidivUpCall) -> Self {
            Self::MulidivUp(value)
        }
    }
    impl ::core::convert::From<SimulateSwapCall> for LogNormalCalls {
        fn from(value: SimulateSwapCall) -> Self {
            Self::SimulateSwap(value)
        }
    }
    impl ::core::convert::From<SlotCall> for LogNormalCalls {
        fn from(value: SlotCall) -> Self {
            Self::Slot(value)
        }
    }
    impl ::core::convert::From<SwapFeePercentageWadCall> for LogNormalCalls {
        fn from(value: SwapFeePercentageWadCall) -> Self {
            Self::SwapFeePercentageWad(value)
        }
    }
    impl ::core::convert::From<ToUintCall> for LogNormalCalls {
        fn from(value: ToUintCall) -> Self {
            Self::ToUint(value)
        }
    }
    impl ::core::convert::From<ValidateCall> for LogNormalCalls {
        fn from(value: ValidateCall) -> Self {
            Self::Validate(value)
        }
    }
    impl ::core::convert::From<XlCall> for LogNormalCalls {
        fn from(value: XlCall) -> Self {
            Self::Xl(value)
        }
    }
    impl ::core::convert::From<YlCall> for LogNormalCalls {
        fn from(value: YlCall) -> Self {
            Self::Yl(value)
        }
    }
    ///Container type for all return fields from the `APPROXIMATED_MINIMUM_X_INPUT` function with signature `APPROXIMATED_MINIMUM_X_INPUT()` and selector `0x84c82a26`
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
    pub struct ApproximatedMinimumXInputReturn(pub ::ethers::core::types::U256);
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
    ///Container type for all return fields from the `HALF_WAD` function with signature `HALF_WAD()` and selector `0x85aa454e`
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
    pub struct HalfWadReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `INFINITY_IS_NOT_REAL` function with signature `INFINITY_IS_NOT_REAL()` and selector `0xe3d0dca5`
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
    pub struct InfinityIsNotRealReturn(pub ::ethers::core::types::U256);
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
    ///Container type for all return fields from the `TWO_WAD` function with signature `TWO_WAD()` and selector `0x275d67c8`
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
    pub struct TwoWadReturn(pub ::ethers::core::types::I256);
    ///Container type for all return fields from the `WAD` function with signature `WAD()` and selector `0x6a146024`
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
    pub struct WadReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `ZERO` function with signature `ZERO()` and selector `0x58fa63ca`
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
    pub struct ZeroReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `computeD1` function with signature `computeD1(uint256,(uint256,uint256,uint256))` and selector `0x9716aebb`
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
    pub struct ComputeD1Return {
        pub d_1: ::ethers::core::types::I256,
    }
    ///Container type for all return fields from the `computeD2` function with signature `computeD2(uint256,(uint256,uint256,uint256))` and selector `0xd7f97bef`
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
    pub struct ComputeD2Return {
        pub d_2: ::ethers::core::types::I256,
    }
    ///Container type for all return fields from the `computeHalfSigmaSquared` function with signature `computeHalfSigmaSquared(uint256)` and selector `0xa6d33498`
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
    pub struct ComputeHalfSigmaSquaredReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `computePrice` function with signature `computePrice(uint256,uint256,uint256,uint256,uint256)` and selector `0x0625a623`
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
    pub struct ComputePriceReturn {
        pub price: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `computeSwapConstant` function with signature `computeSwapConstant(bytes)` and selector `0xa19cd3d1`
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
    ///Container type for all return fields from the `encodeValidateData` function with signature `encodeValidateData(uint256,uint256,uint256)` and selector `0xbd422d28`
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
    pub struct EncodeValidateDataReturn(pub ::ethers::core::types::Bytes);
    ///Container type for all return fields from the `findLiquidity` function with signature `findLiquidity(uint256,uint256,int256,(uint256,uint256,uint256))` and selector `0x883b6dc5`
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
    pub struct FindLiquidityReturn {
        pub liquidity: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `findX` function with signature `findX(uint256,uint256,int256,(uint256,uint256,uint256))` and selector `0xcd64aea2`
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
    pub struct FindXReturn {
        pub reserve_y: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `findY` function with signature `findY(uint256,uint256,int256,(uint256,uint256,uint256))` and selector `0x6417d4b5`
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
    pub struct FindYReturn {
        pub reserve_y: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `init` function with signature `init(bytes)` and selector `0x4ddf47d4`
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
        pub swap_constant_growth: ::ethers::core::types::I256,
        pub reserve_x_wad: ::ethers::core::types::U256,
        pub reserve_y_wad: ::ethers::core::types::U256,
        pub total_liquidity: ::ethers::core::types::U256,
    }
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
    ///Container type for all return fields from the `lx` function with signature `lx(uint256,uint256,(uint256,uint256,uint256))` and selector `0x9d819e83`
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
    pub struct LxReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `ly` function with signature `ly(uint256,uint256,(uint256,uint256,uint256))` and selector `0x5608bea1`
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
    pub struct LyReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `mulidiv` function with signature `mulidiv(int256,int256,int256)` and selector `0xe4937428`
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
    pub struct MulidivReturn {
        pub z: ::ethers::core::types::I256,
    }
    ///Container type for all return fields from the `mulidivUp` function with signature `mulidivUp(int256,int256,int256)` and selector `0xffb36266`
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
    pub struct MulidivUpReturn {
        pub z: ::ethers::core::types::I256,
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
    ///Container type for all return fields from the `slot` function with signature `slot()` and selector `0x1a88bc66`
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
    pub struct SlotReturn {
        pub strike_price_wad: ::ethers::core::types::U256,
        pub sigma_percent_wad: ::ethers::core::types::U256,
        pub tau_years_wad: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `swapFeePercentageWad` function with signature `swapFeePercentageWad()` and selector `0x40b43169`
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
    pub struct SwapFeePercentageWadReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `toUint` function with signature `toUint(int256)` and selector `0x1d9cf722`
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
    pub struct ToUintReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `validate` function with signature `validate(bytes)` and selector `0xc16e50ef`
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
    pub struct ValidateReturn {
        pub valid: bool,
        pub swap_constant_growth: ::ethers::core::types::I256,
        pub liquidity_delta: ::ethers::core::types::I256,
        pub adjusted_reserve_x_wad: ::ethers::core::types::U256,
        pub adjusted_reserve_y_wad: ::ethers::core::types::U256,
        pub adjusted_liquidity: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `xl` function with signature `xl(uint256,uint256,(uint256,uint256,uint256))` and selector `0x184d30ba`
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
    pub struct XlReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `yl` function with signature `yl(uint256,uint256,(uint256,uint256,uint256))` and selector `0xdbaf1142`
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
    pub struct YlReturn(pub ::ethers::core::types::U256);
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
}
