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
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qb\0%l8\x03\x80b\0%l\x839\x81\x01`@\x81\x90Ra\x001\x91a\09V[`\0Ua\0RV[`\0` \x82\x84\x03\x12\x15a\0KW`\0\x80\xFD[PQ\x91\x90PV[a%\n\x80b\0\0b`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\xA9W`\x005`\xE0\x1C\x80c\x85\xAAEN\x11a\0\xF9W\x80c\xC1nP\xEF\x11a\0\x97W\x80c\xDB\xAF\x11B\x11a\0qW\x80c\xDB\xAF\x11B\x14a\x04\x08W\x80c\xE4\x93t(\x14a\x04\x1BW\x80c\xF9\xC2\x82\x11\x14a\x04.W\x80c\xFF\xB3bf\x14a\x047W`\0\x80\xFD[\x80c\xC1nP\xEF\x14a\x03\xA0W\x80c\xCDd\xAE\xA2\x14a\x03\xE2W\x80c\xD7\xF9{\xEF\x14a\x03\xF5W`\0\x80\xFD[\x80c\x9D\x81\x9E\x83\x11a\0\xD3W\x80c\x9D\x81\x9E\x83\x14a\x03)W\x80c\xA1\x9C\xD3\xD1\x14a\x03<W\x80c\xA6\xD34\x98\x14a\x03OW\x80c\xBDB-(\x14a\x03bW`\0\x80\xFD[\x80c\x85\xAAEN\x14a\x02\xF4W\x80c\x88;m\xC5\x14a\x03\x03W\x80c\x97\x16\xAE\xBB\x14a\x03\x16W`\0\x80\xFD[\x80c@\xB41i\x11a\x01fW\x80cX\xFAc\xCA\x11a\x01@W\x80cX\xFAc\xCA\x14a\x02\xC2W\x80cd\x17\xD4\xB5\x14a\x02\xCAW\x80cj\x14`$\x14a\x02\xDDW\x80cme\"\x99\x14a\x02\xECW`\0\x80\xFD[\x80c@\xB41i\x14a\x02iW\x80cM\xDFG\xD4\x14a\x02rW\x80cV\x08\xBE\xA1\x14a\x02\xAFW`\0\x80\xFD[\x80c\n\xC304\x14a\x01\xAEW\x80c\x0F\n\xA3\x95\x14a\x01\xD9W\x80c\x18M0\xBA\x14a\x01\xF9W\x80c\x1A\x88\xBCf\x14a\x02\x1AW\x80c\x1D\x9C\xF7\"\x14a\x02GW\x80c']g\xC8\x14a\x02ZW[`\0\x80\xFD[a\x01\xC1a\x01\xBC6`\x04a\x1F\x06V[a\x04JV[`@Qa\x01\xD0\x93\x92\x91\x90a\x1F}V[`@Q\x80\x91\x03\x90\xF3[a\x01\xECa\x01\xE76`\x04a \x17V[a\x06\xF0V[`@Qa\x01\xD0\x91\x90a WV[a\x02\x0Ca\x02\x076`\x04a jV[a\x07\"V[`@Q\x90\x81R` \x01a\x01\xD0V[`\x01T`\x02T`\x03Ta\x02,\x92\x91\x90\x83V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x01\xD0V[a\x02\x0Ca\x02U6`\x04a \xA0V[a\x07tV[a\x02\x0Cg\x1B\xC1mgN\xC8\0\0\x81V[a\x02\x0C`\0T\x81V[a\x02\x85a\x02\x806`\x04a \xB9V[a\x07\xC2V[`@\x80Q\x95\x15\x15\x86R` \x86\x01\x94\x90\x94R\x92\x84\x01\x91\x90\x91R``\x83\x01R`\x80\x82\x01R`\xA0\x01a\x01\xD0V[a\x02\x0Ca\x02\xBD6`\x04a jV[a\x08^V[a\x02\x0C`\0\x81V[a\x02\x0Ca\x02\xD86`\x04a \x17V[a\x08\x9FV[a\x02\x0Cg\r\xE0\xB6\xB3\xA7d\0\0\x81V[a\x02\x0C`\x01\x81V[a\x02\x0Cg\x06\xF0[Y\xD3\xB2\0\0\x81V[a\x02\x0Ca\x03\x116`\x04a \x17V[a\x08\xFAV[a\x02\x0Ca\x03$6`\x04a!+V[a\tSV[a\x02\x0Ca\x0376`\x04a jV[a\t\xF0V[a\x02\x0Ca\x03J6`\x04a!XV[a\n\x85V[a\x02\x0Ca\x03]6`\x04a \xA0V[a\n\xE0V[a\x01\xECa\x03p6`\x04a\"\tV[`@\x80Q` \x81\x01\x94\x90\x94R\x83\x81\x01\x92\x90\x92R``\x80\x84\x01\x91\x90\x91R\x81Q\x80\x84\x03\x90\x91\x01\x81R`\x80\x90\x92\x01\x90R\x90V[a\x03\xB3a\x03\xAE6`\x04a!XV[a\x0B\tV[`@\x80Q\x96\x15\x15\x87R` \x87\x01\x95\x90\x95R\x93\x85\x01\x92\x90\x92R``\x84\x01R`\x80\x83\x01R`\xA0\x82\x01R`\xC0\x01a\x01\xD0V[a\x02\x0Ca\x03\xF06`\x04a \x17V[a\x0E9V[a\x02\x0Ca\x04\x036`\x04a!+V[a\x0E\x8AV[a\x02\x0Ca\x04\x166`\x04a jV[a\x0E\xB2V[a\x02\x0Ca\x04)6`\x04a\"\tV[a\x0E\xF4V[a\x02\x0Ca\x01\0\x81V[a\x02\x0Ca\x04E6`\x04a\"\tV[a\x0FrV[`\0\x80``a\x04p`@Q\x80`\x80\x01`@R\x80`G\x81R` \x01a$\x9E`G\x919a\x0F\xF9V[`\0\x80`\x003`\x01`\x01`\xA0\x1B\x03\x16c\xEB\xAD\xEF\x01`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01```@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xB3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xD7\x91\x90a\"5V[`@\x80Q` \x81\x01\x85\x90R\x90\x81\x01\x83\x90R``\x81\x01\x82\x90R\x92\x95P\x90\x93P\x91P`\0\x90a\x05\x15\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@Ra\n\x85V[\x90P`\0\x89\x15a\x06\x02W`\0\x80Ta\x05.\x90\x8B\x90a\x10?V[\x90P`\0a\x05F\x87a\x05@\x84\x88a\x10?V[\x90a\x10ZV[\x90Pa\x05S`\x01\x82a\"yV[\x90Pa\x05_\x8B\x88a\"yV[\x96Pa\x05k\x81\x86a\"yV[`@\x80Q``\x81\x01\x82R`\x01T\x81R`\x02T` \x82\x01R`\x03T\x91\x81\x01\x91\x90\x91R\x90\x95P\x86\x90a\x05\xA0\x90\x89\x90\x88\x90\x88\x90a\x08\x9FV[\x96Pa\x05\xAD`\x01\x88a\"yV[\x96Pa\x05\xB9\x87\x82a\"\x8CV[\x93Pa\x05\xFA`@Q\x80`@\x01`@R\x80`\x1C\x81R` \x01\x7FEsimated Y reserve to submit\0\0\0\0\x81RP\x88a\x10oV[PPPa\x06\x9CV[`\0\x80Ta\x06\x11\x90\x8B\x90a\x10?V[\x90P`\0a\x06#\x86a\x05@\x84\x88a\x10?V[\x90Pa\x060`\x01\x82a\"yV[\x90Pa\x06<\x8B\x87a\"yV[\x95Pa\x06H\x81\x86a\"yV[`@\x80Q``\x81\x01\x82R`\x01T\x81R`\x02T` \x82\x01R`\x03T\x91\x81\x01\x91\x90\x91R\x90\x95P\x87\x90a\x06}\x90\x88\x90\x88\x90\x88\x90a\x0E9V[\x97Pa\x06\x8A`\x01\x89a\"yV[\x97Pa\x06\x96\x88\x82a\"\x8CV[\x93PPPP[`@\x80Q` \x81\x01\x87\x90R\x90\x81\x01\x85\x90R``\x81\x01\x84\x90R`\0\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`\0a\x06\xD6\x82a\x0B\tV[P\x93\x9DP\x95\x9BP\x93\x99PPPPPPPPPP\x92P\x92P\x92V[``\x84\x84\x84\x84`@Q` \x01a\x07\t\x94\x93\x92\x91\x90a\"\x9FV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x94\x93PPPPV[`\0\x80a\x07/\x84\x84a\tSV[\x90P`\0a\x07<\x82a\x10\xB8V[\x90P`\0a\x07I\x82a\x07tV[\x90Pa\x07ga\x07`\x82g\r\xE0\xB6\xB3\xA7d\0\0a\"\x8CV[\x88\x90a\x10?V[\x93PPPP[\x93\x92PPPV[`\0\x80\x82\x12\x15a\x07\xBEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01RotoUint: negative`\x80\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[P\x90V[`\0\x80`\0\x80`\0a\x07\xEB`@Q\x80`\x80\x01`@R\x80`G\x81R` \x01a$\x9E`G\x919a\x0F\xF9V[a\x07\xF7\x86\x88\x01\x88a \x17V[\x80Q`\x01\x81\x90U` \x80\x83\x01Q`\x02\x81\x90U`@\x93\x84\x01Q`\x03\x81\x90U\x84Q``\x81\x01\x86R\x93\x84R\x91\x83\x01R\x91\x81\x01\x91\x90\x91R\x92\x95P\x90\x93P\x91Pa\x08A\x90\x84\x90\x84\x90\x84\x90a\x11!V[\x93Pa\x08O`\0`\x03a\"\xD3V[\x84\x12\x15\x94P\x92\x95P\x92\x95\x90\x93PV[`\0\x80a\x08k\x84\x84a\x0E\x8AV[\x90P`\0a\x08x\x82a\x10\xB8V[\x90P`\0a\x08\x85\x82a\x07tV[\x85Q\x90\x91Pa\x07g\x90a\x08\x98\x90\x83a\x11\xD1V[\x88\x90a\x11\xE6V[\x80Q`\0\x90`d\x90\x82\x90a\x08\xB5\x90`\x01\x90a\"\x8CV[\x90Pa\x08\xEF\x87\x87\x87\x87`@Q` \x01a\x08\xD1\x94\x93\x92\x91\x90a\"\x9FV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83\x83`\x01a\x01\0a\x11\xFBa\x126V[\x97\x96PPPPPPPV[`\0\x80a\t\x08\x86`\x01a\"yV[\x90P`\0k\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x90Pa\x08\xEF\x87\x87\x87\x87`@Q` \x01a\t5\x94\x93\x92\x91\x90a\"\x9FV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83\x83`\x01a\x01\0a\x13Ga\x126V[`\0\x80a\th\x83` \x01Q\x84`@\x01Qa\x13xV[\x90P`\0a\ty\x84` \x01Qa\n\xE0V[\x90P`\0a\t\x9Ca\t\x97\x86`\0\x01Q\x88a\x10Z\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x13\xA2V[\x90P\x80a\t\xA8\x81a\"\xFBV[\x91PP`\0a\t\xC4\x86`@\x01Q\x84a\x11\xD1\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\t\xCE\x90\x83a\"\xD3V[\x90P\x83a\t\xE4\x82g\r\xE0\xB6\xB3\xA7d\0\0\x83a\x0FrV[\x98\x97PPPPPPPPV[`\0\x80a\t\xFD\x84\x84a\tSV[\x90P`\0a\n\n\x82a\x10\xB8V[\x90P`\0a\n\x17\x82a\x07tV[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0\x81\x10a\npW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7Flx: denominator is zero\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x07\xB5V[a\x07ga\x08\x98\x82g\r\xE0\xB6\xB3\xA7d\0\0a\"\x8CV[`\0\x80`\0\x80\x84\x80` \x01\x90Q\x81\x01\x90a\n\x9F\x91\x90a\"5V[`@\x80Q``\x81\x01\x82R`\x01T\x81R`\x02T` \x82\x01R`\x03T\x91\x81\x01\x91\x90\x91R\x92\x95P\x90\x93P\x91Pa\n\xD7\x90\x84\x90\x84\x90\x84\x90a\x11!V[\x95\x94PPPPPV[`\0\x80a\n\xF5\x83g\x1B\xC1mgN\xC8\0\0a\x15}V[\x90Pa\x07mg\x06\xF0[Y\xD3\xB2\0\0\x82a\x11\xD1V[`\0\x80`\0\x80`\0\x80a\x0B3`@Q\x80`\x80\x01`@R\x80`G\x81R` \x01a$\x9E`G\x919a\x0F\xF9V[`\0\x80`\x003`\x01`\x01`\xA0\x1B\x03\x16c\xEB\xAD\xEF\x01`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01```@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0BvW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\x9A\x91\x90a\"5V[\x92P\x92P\x92P\x89\x80` \x01\x90Q\x81\x01\x90a\x0B\xB4\x91\x90a\"5V[\x91\x97P\x95P\x93P`\x01\x83\x87\x11\x15a\x0C\rW`\0a\x0B\xD1\x85\x89a\"\x8CV[\x90P`\0a\x0B\xEA`\0T\x83a\x10?\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa\x0B\xFA\x86a\x05@\x83\x87a\x10?V[a\x0C\x04\x90\x84a\"yV[\x92PPPa\x0C\xABV[\x82\x86\x11\x15a\x0CJW`\0a\x0C!\x84\x88a\"\x8CV[\x90P`\0a\x0C:`\0T\x83a\x10?\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa\x0B\xFA\x85a\x05@\x83\x87a\x10?V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`0`$\x82\x01R\x7Finvalid swap: inputs x and y hav`D\x82\x01Roe the same sign!`\x80\x1B`d\x82\x01R`\x84\x01a\x07\xB5V[a\x0C\xB5\x82\x86a#\x1AV[`@\x80Q``\x81\x01\x82R`\x01T\x81R`\x02T` \x82\x01R`\x03T\x91\x81\x01\x91\x90\x91R\x90\x98Pa\x0C\xE8\x90\x85\x90\x85\x90\x85\x90a\x11!V[`@\x80Q``\x81\x01\x82R`\x01T\x81R`\x02T` \x82\x01R`\x03T\x91\x81\x01\x91\x90\x91Ra\r\x18\x90\x89\x90\x89\x90\x89\x90a\x11!V[a\r\"\x91\x90a#\x1AV[\x98Pa\rY`@Q\x80`@\x01`@R\x80`\x14\x81R` \x01s\nn\xEC.\x04\x0Cm\xED\xCEn\x8C-\xCE\x84\x0C\xEEM\xEE\xEE\x8D`c\x1B\x81RPa\x0F\xF9V[a\rb\x89a\x15\xAEV[a\r\xA0`@Q\x80`@\x01`@R\x80`\x19\x81R` \x01\x7FSubmitted Liquidity delta\0\0\0\0\0\0\0\x81RPa\x0F\xF9V[a\r\xA9\x88a\x15\xAEV[a\r\xDD`@Q\x80`@\x01`@R\x80`\x13\x81R` \x01rMin liquidity delta`h\x1B\x81RPa\x0F\xF9V[a\r\xE6\x81a\x15\xAEV[a\x0E\x07`@Q\x80``\x01`@R\x80`%\x81R` \x01a$\xE5`%\x919a\x0F\xF9V[a\x0E\x19a\x0E\x14\x82\x8Aa#\x1AV[a\x15\xAEV[`\0\x89\x12\x15\x80\x15a\x0E*WP\x80\x88\x12\x15[\x99PPPPP\x91\x93\x95P\x91\x93\x95V[`\0`\x02\x81a\x0EP\x82g\r\xE0\xB6\xB3\xA7d\0\0a\"\x8CV[\x90Pa\x08\xEF\x87\x87\x87\x87`@Q` \x01a\x0El\x94\x93\x92\x91\x90a\"\x9FV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83\x83`\x01a\x01\0a\x15\xF3a\x126V[`\0a\x0E\x9E\x82` \x01Q\x83`@\x01Qa\x13xV[a\x0E\xA8\x84\x84a\tSV[a\x07m\x91\x90a#\x1AV[`\0\x80a\x0E\xBF\x84\x84a\x0E\x8AV[\x90P`\0a\x0E\xCC\x82a\x10\xB8V[\x90P`\0a\x0E\xD9\x82a\x07tV[\x85Q\x90\x91Pa\x07g\x90\x82\x90a\x0E\xEE\x90\x8Aa\x10?V[\x90a\x10?V[\x82\x82\x02\x81\x15\x80\x15\x90a\x0F\x1CWP\x83\x15\x80a\x0F\x1CWP\x82\x84\x82\x81a\x0F\x19Wa\x0F\x19a#AV[\x05\x14[a\x0FZW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rn\x1B][\x1AY\x1A]\x88\x1A[\x9D\x98[\x1AY`\x8A\x1B`D\x82\x01R`d\x01a\x07\xB5V[\x81\x81\x81a\x0FiWa\x0Fia#AV[\x05\x94\x93PPPPV[`\0a\x0F\x7F\x84\x84\x84a\x0E\xF4V[\x90P\x81a\x0F\x8C\x84\x86a#WV[a\x0F\x96\x91\x90a#\x87V[\x15a\x07mW`\x01`\x01`\xFF\x1B\x03\x81\x12a\x0F\xE6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01RqmulidivUp overflow`p\x1B`D\x82\x01R`d\x01a\x07\xB5V[a\x0F\xF1`\x01\x82a\"\xD3V[\x94\x93PPPPV[a\x10<\x81`@Q`$\x01a\x10\r\x91\x90a WV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\x10L\x13\xEB`\xE2\x1B\x17\x90Ra\x16$V[PV[`\0a\x07m\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x16EV[\x92\x91PPV[`\0a\x07m\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x16EV[a\x10\xB4\x82\x82`@Q`$\x01a\x10\x85\x92\x91\x90a#\x9BV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c-\x83\x9C\xB3`\xE2\x1B\x17\x90Ra\x16$V[PPV[`\0\x80g\x13\xA0K\xBD\xFD\xC9\xBE\x88a\x10\xD6g\r\xE0\xB6\xB3\xA7d\0\0\x85a#WV[a\x10\xE0\x91\x90a#\xBDV[\x90P`\0a\x10\xED\x82a#\xEBV[\x90P`\0a\x10\xFA\x82a\x16sV[\x90Pg\x1B\xC1mgN\xC8\0\0a\x11\x17g\r\xE0\xB6\xB3\xA7d\0\0\x83a#WV[a\n\xD7\x91\x90a#\xBDV[`\0\x82\x85\x10a\x11rW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FtradingFunction: invalid x\0\0\0\0\0\0`D\x82\x01R`d\x01a\x07\xB5V[`\0a\x11\x86a\x11\x81\x87\x86a\x11\xE6V[a\x18\\V[\x90P`\0a\x11\xA4a\x11\x81\x87a\x11\x9F\x87`\0\x01Q\x89a\x11\xD1V[a\x11\xE6V[\x90P`\0a\x11\xBA\x85` \x01Q\x86`@\x01Qa\x13xV[\x90P\x80a\x11\xC7\x83\x85a\"\xD3V[a\t\xE4\x91\x90a\"\xD3V[`\0a\x07m\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x18\xF9V[`\0a\x07m\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x18\xF9V[`\0\x80`\0\x80`\0\x86\x80` \x01\x90Q\x81\x01\x90a\x12\x17\x91\x90a$\x07V[\x93P\x93P\x93P\x93P\x81a\x12,\x85\x88\x86\x85a\x11!V[a\x08\xEF\x91\x90a#\x1AV[`\0\x84\x86\x11\x15a\x12cW`@Qc0\x82\xDF\xDB`\xE1\x1B\x81R`\x04\x81\x01\x87\x90R`$\x81\x01\x86\x90R`D\x01a\x07\xB5V[`\0a\x12s\x88\x88\x85c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x12\x85\x89\x88\x86c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x12\x93\x82\x84a#WV[\x13\x15a\x12\xBCW`@Qc\x06\xF1\xBE]`\xE2\x1B\x81R`\x04\x81\x01\x83\x90R`$\x81\x01\x82\x90R`D\x01a\x07\xB5V[`\0a\x12\xC8\x89\x89a\"\x8CV[\x90P`\0[`\x02a\x12\xD9\x8A\x8Ca\"yV[a\x12\xE3\x91\x90a$rV[\x94P`\0a\x12\xF5\x8C\x87\x89c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x13\x03\x86\x83a#WV[\x13a\x13\x10W\x85\x99Pa\x13\x17V[\x85\x9AP\x80\x94P[a\x13!\x8B\x8Ba\"\x8CV[\x92PP`\x01\x01\x87\x82\x11\x80\x15a\x135WP\x86\x81\x10[a\x12\xCDWPPPP\x96\x95PPPPPPV[`\0\x80`\0\x80`\0\x86\x80` \x01\x90Q\x81\x01\x90a\x13c\x91\x90a$\x07V[\x93P\x93P\x93P\x93P\x81a\x12,\x85\x85\x89\x85a\x11!V[`\0\x80a\x13\x84\x83a\x19\x18V[\x90P`\0a\x13\x96c;\x9A\xCA\0\x83a$\x86V[\x90Pa\n\xD7\x85\x82a\x11\xD1V[`\0\x80\x82\x13a\x13\xDFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x07\xB5V[`\0``a\x13\xEC\x84a\x19\xBCV[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[`\0a\x07mg\r\xE0\xB6\xB3\xA7d\0\0\x83a\x15\x95\x86a\x13\xA2V[a\x15\x9F\x91\x90a#WV[a\x15\xA9\x91\x90a#\xBDV[a\x1AdV[a\x10<\x81`@Q`$\x01a\x15\xC4\x91\x81R` \x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c-[l\xB9`\xE0\x1B\x17\x90Ra\x16$V[`\0\x80`\0\x80`\0\x86\x80` \x01\x90Q\x81\x01\x90a\x16\x0F\x91\x90a$\x07V[\x93P\x93P\x93P\x93P\x81a\x12,\x87\x86\x86\x85a\x11!V[\x80Qjconsole.log` \x83\x01`\0\x80\x84\x83\x85Z\xFAPPPPPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x16]W`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[`\0\x81`\0\x03a\x16\x8CWPg\r\xE0\xB6\xB3\xA7d\0\0\x91\x90PV[gV\x98\xEE\xF0fp\0\0\x82\x12a\x16\xA3WP`\0\x91\x90PV[a\x16\xB4gV\x98\xEE\xF0fp\0\0a#\xEBV[\x82\x13a\x16\xC9WPg\x1B\xC1mgN\xC8\0\0\x91\x90PV[`\0a\x16\xD4\x83a\x1C\rV[\x90P`\0a\x17\rg\r\xE0\xB6\xB3\xA7d\0\0a\x16\xF6\x84g\x1B\xC1mgN\xC8\0\0a\x11\xE6V[a\x17\x08\x90g\r\xE0\xB6\xB3\xA7d\0\0a\"\xD3V[a\x1CDV[\x90P`\0\x80\x82a\x17n\x81a\x17[\x81a\x17I\x81a\x171\x81g\x02_\x0F\xE1\x05\xA3\x14\0a\x1CYV[a\x17D\x90g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19a\"\xD3V[a\x1CYV[a\x17D\x90g\x14\xA8EL\x19\xE1\xAC\0a\"\xD3V[a\x17D\x90g\x0F\xC1\x0E\x01W\x82w\xFF\x19a\"\xD3V[a\x17\x80\x90g\x03\xDE\xBD\x08;\x8C|\0a\"\xD3V[\x91P\x83\x90Pa\x17\xE8\x81a\x17\xD6\x81a\x17\xC4\x81a\x17\xB2\x81a\x17\x9F\x81\x8Ba\x1CYV[a\x17D\x90g\x02\x95\xD4\0\xEA2W\xFF\x19a\"\xD3V[a\x17D\x90g\x01W\xD8\xB2\xEC\xC7\x08\0a\"\xD3V[a\x17D\x90g\x051\n\xA7\xD5!0\0a\"\xD3V[a\x17D\x90g\r\xE0\xCC=\x15a\0\0a\"\xD3V[\x91P\x81g\x11\x90\0\xAB\x10\x0F\xFC\0a\x17\xFE\x87\x88a\x1CYV[a\x18\n\x90`\0\x19a#WV[a\x18\x14\x91\x90a#\x1AV[a\x18\x1E\x91\x90a\"\xD3V[\x92PP`\0a\x18,\x83a\x1AdV[\x90P`\0a\x18:\x85\x83a\x1CYV[\x90P`\0\x88\x12a\x18JW\x80a\t\xE4V[a\t\xE4\x81g\x1B\xC1mgN\xC8\0\0a#\x1AV[`\0g\x06\xF0[Y\xD3\xB2\0\0\x82\x03a\x18uWP`\0\x91\x90PV[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x12a\x18\x9DW`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\x18\xBEW`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x18\xCB\x83`\x02a#WV[\x90P`\0a\x18\xD8\x82a\x1CnV[\x90P`\0a\x18\xEEg\x13\xA0K\xBD\xFD\xC9\xBE\x88\x83a\x1CYV[\x90Pa\n\xD7\x81a#\xEBV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x19\x11W`\0\x80\xFD[\x04\x92\x91PPV[`\xB5\x81`\x01`\x88\x1B\x81\x10a\x191W`@\x91\x90\x91\x1B\x90`\x80\x1C[i\x01\0\0\0\0\0\0\0\0\0\x81\x10a\x19MW` \x91\x90\x91\x1B\x90`@\x1C[e\x01\0\0\0\0\0\x81\x10a\x19eW`\x10\x91\x90\x91\x1B\x90` \x1C[c\x01\0\0\0\x81\x10a\x19{W`\x08\x91\x90\x91\x1B\x90`\x10\x1C[b\x01\0\0\x01\x02`\x12\x1C\x80\x82\x04\x01`\x01\x90\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x90\x81\x90\x04\x81\x11\x90\x03\x90V[`\0\x80\x82\x11a\x19\xF9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x07\xB5V[P`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x82\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[`\0h\x02H\xCE6\xA7\x0C\xB2k>\x19\x82\x13a\x1A\x7FWP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a\x1A\xC6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x01a\x07\xB5V[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05`\x01`_\x1B\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1Dm\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1Dn\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[`\0`\x01`\xFF\x1B\x82\x03a\x1C3W`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x12\x15a\x07\xBEWP\x19`\x01\x01\x90V[`\0a\x07m\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x1E\xE7V[`\0a\x07m\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x1E\xE7V[`\0\x80\x82\x12\x80a\x1C\x85WPg\x1B\xC1mgN\xC8\0\0\x82\x13[\x15a\x1C\xA3W`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\x1C\xC4W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81g\x1B\xC1mgN\xC8\0\0\x03a\x1C\xECW`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x15a\x1C\xF7W\x91\x90PV[`\0g\r\xE0\xB6\xB3\xA7d\0\0\x83\x12a\x1D\x1FWa\x1D\x1A\x83g\x1B\xC1mgN\xC8\0\0a#\x1AV[a\x1D!V[\x82[\x90P`\0a\x1D7\x82g\x1B\xC1mgN\xC8\0\0a\x1CDV[\x90P\x80`\0\x03a\x1DZW`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x1De\x82a\x13\xA2V[\x90P`\0c;\x9A\xCA\0a\x1D\x90a\x1D\x8Ba\x1D\x85g\x1B\xC1mgN\xC8\0\0a#\xEBV[\x85a\x1CYV[a\x19\x18V[a\x1D\x9A\x91\x90a#WV[\x90P`\0\x80a\x1D\xB1\x83g\x03\xC1f\\z\xAB \0a\x1CYV[a\x1D\xC3\x90g \x05\xFEO&\x8E\xA0\0a\"\xD3V[\x90P`\0a\x1D\xEE\x84a\x1D\xDC\x86f\x9F2u$b\xA0\0a\x1CYV[a\x17D\x90g\r\xC5R\x7Fd, \0a\"\xD3V[a\x1E\0\x90g\r\xE0\xB6\xB3\xA7d\0\0a\"\xD3V[\x90Pa\x1E$g\t\xD0(\xCCo _\xFF\x19\x85a\x1E\x1A\x85\x85a\x1CDV[a\x17D\x91\x90a#\x1AV[\x92PPP`\0[`\x02\x81\x10\x15a\x1E\xBFW`\0\x86a\x1E@\x84a\x16sV[a\x1EJ\x91\x90a#\x1AV[\x90P`\0a\x1EX\x84\x85a\x1CYV[a\x1Ea\x90a#\xEBV[\x90P`\0a\x1En\x82a\x1AdV[\x90P`\0a\x1E|\x86\x85a\x1CYV[a\x1E\x8Eg\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x84a\x1CYV[a\x1E\x98\x91\x90a#\x1AV[\x90Pa\x1E\xA4\x84\x82a\x1CDV[a\x1E\xAE\x90\x87a\"\xD3V[\x95P\x84`\x01\x01\x94PPPPPa\x1E+V[g\r\xE0\xB6\xB3\xA7d\0\0\x88\x12a\x1E\xDCWa\x1E\xD7\x82a#\xEBV[a\t\xE4V[P\x96\x95PPPPPPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x05\x85\x14\x17\x16a\x1E\xFFW`\0\x80\xFD[\x05\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a\x1F\x19W`\0\x80\xFD[\x825\x80\x15\x15\x81\x14a\x1F)W`\0\x80\xFD[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a\x1F]W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\x1FAV[P`\0` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[\x83\x15\x15\x81R\x82` \x82\x01R```@\x82\x01R`\0a\n\xD7``\x83\x01\x84a\x1F7V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x1F\xD7Wa\x1F\xD7a\x1F\x9EV[`@R\x90V[`\0``\x82\x84\x03\x12\x15a\x1F\xEFW`\0\x80\xFD[a\x1F\xF7a\x1F\xB4V[\x90P\x815\x81R` \x82\x015` \x82\x01R`@\x82\x015`@\x82\x01R\x92\x91PPV[`\0\x80`\0\x80`\xC0\x85\x87\x03\x12\x15a -W`\0\x80\xFD[\x845\x93P` \x85\x015\x92P`@\x85\x015\x91Pa L\x86``\x87\x01a\x1F\xDDV[\x90P\x92\x95\x91\x94P\x92PV[` \x81R`\0a\x07m` \x83\x01\x84a\x1F7V[`\0\x80`\0`\xA0\x84\x86\x03\x12\x15a \x7FW`\0\x80\xFD[\x835\x92P` \x84\x015\x91Pa \x97\x85`@\x86\x01a\x1F\xDDV[\x90P\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a \xB2W`\0\x80\xFD[P5\x91\x90PV[`\0\x80` \x83\x85\x03\x12\x15a \xCCW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a \xE4W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a \xF8W`\0\x80\xFD[\x815\x81\x81\x11\x15a!\x07W`\0\x80\xFD[\x86` \x82\x85\x01\x01\x11\x15a!\x19W`\0\x80\xFD[` \x92\x90\x92\x01\x96\x91\x95P\x90\x93PPPPV[`\0\x80`\x80\x83\x85\x03\x12\x15a!>W`\0\x80\xFD[\x825\x91Pa!O\x84` \x85\x01a\x1F\xDDV[\x90P\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a!jW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a!\x82W`\0\x80\xFD[\x81\x84\x01\x91P\x84`\x1F\x83\x01\x12a!\x96W`\0\x80\xFD[\x815\x81\x81\x11\x15a!\xA8Wa!\xA8a\x1F\x9EV[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a!\xD0Wa!\xD0a\x1F\x9EV[\x81`@R\x82\x81R\x87` \x84\x87\x01\x01\x11\x15a!\xE9W`\0\x80\xFD[\x82` \x86\x01` \x83\x017`\0\x92\x81\x01` \x01\x92\x90\x92RP\x95\x94PPPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\"\x1EW`\0\x80\xFD[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\"JW`\0\x80\xFD[\x83Q\x92P` \x84\x01Q\x91P`@\x84\x01Q\x90P\x92P\x92P\x92V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x10TWa\x10Ta\"cV[\x81\x81\x03\x81\x81\x11\x15a\x10TWa\x10Ta\"cV[\x84\x81R` \x80\x82\x01\x85\x90R`@\x80\x83\x01\x85\x90R\x83Q``\x84\x01R\x90\x83\x01Q`\x80\x83\x01R\x82\x01Q`\xA0\x82\x01R`\xC0\x81\x01a\n\xD7V[\x80\x82\x01\x82\x81\x12`\0\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a\"\xF3Wa\"\xF3a\"cV[PP\x92\x91PPV[`\0`\x01`\x01`\xFF\x1B\x01\x82\x01a#\x13Wa#\x13a\"cV[P`\x01\x01\x90V[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a#:Wa#:a\"cV[P\x92\x91PPV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15a#sWa#sa\"cV[\x81\x81\x05\x83\x14\x82\x15\x17a\x10TWa\x10Ta\"cV[`\0\x82a#\x96Wa#\x96a#AV[P\x07\x90V[`@\x81R`\0a#\xAE`@\x83\x01\x85a\x1F7V[\x90P\x82` \x83\x01R\x93\x92PPPV[`\0\x82a#\xCCWa#\xCCa#AV[`\x01`\xFF\x1B\x82\x14`\0\x19\x84\x14\x16\x15a#\xE6Wa#\xE6a\"cV[P\x05\x90V[`\0`\x01`\xFF\x1B\x82\x01a$\0Wa$\0a\"cV[P`\0\x03\x90V[`\0\x80`\0\x80\x84\x86\x03`\xC0\x81\x12\x15a$\x1EW`\0\x80\xFD[\x85Q\x94P` \x86\x01Q\x93P`@\x86\x01Q\x92P```_\x19\x82\x01\x12\x15a$BW`\0\x80\xFD[Pa$Ka\x1F\xB4V[``\x86\x01Q\x81R`\x80\x86\x01Q` \x82\x01R`\xA0\x90\x95\x01Q`@\x86\x01RP\x91\x94\x90\x93P\x90\x91\x90V[`\0\x82a$\x81Wa$\x81a#AV[P\x04\x90V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x10TWa\x10Ta\"cV\xFEMake sure the calling contract of this console.log is the Core contractliquidity delta - min liquidity delta";
    /// The bytecode of the contract.
    pub static LOGNORMAL_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\xA9W`\x005`\xE0\x1C\x80c\x85\xAAEN\x11a\0\xF9W\x80c\xC1nP\xEF\x11a\0\x97W\x80c\xDB\xAF\x11B\x11a\0qW\x80c\xDB\xAF\x11B\x14a\x04\x08W\x80c\xE4\x93t(\x14a\x04\x1BW\x80c\xF9\xC2\x82\x11\x14a\x04.W\x80c\xFF\xB3bf\x14a\x047W`\0\x80\xFD[\x80c\xC1nP\xEF\x14a\x03\xA0W\x80c\xCDd\xAE\xA2\x14a\x03\xE2W\x80c\xD7\xF9{\xEF\x14a\x03\xF5W`\0\x80\xFD[\x80c\x9D\x81\x9E\x83\x11a\0\xD3W\x80c\x9D\x81\x9E\x83\x14a\x03)W\x80c\xA1\x9C\xD3\xD1\x14a\x03<W\x80c\xA6\xD34\x98\x14a\x03OW\x80c\xBDB-(\x14a\x03bW`\0\x80\xFD[\x80c\x85\xAAEN\x14a\x02\xF4W\x80c\x88;m\xC5\x14a\x03\x03W\x80c\x97\x16\xAE\xBB\x14a\x03\x16W`\0\x80\xFD[\x80c@\xB41i\x11a\x01fW\x80cX\xFAc\xCA\x11a\x01@W\x80cX\xFAc\xCA\x14a\x02\xC2W\x80cd\x17\xD4\xB5\x14a\x02\xCAW\x80cj\x14`$\x14a\x02\xDDW\x80cme\"\x99\x14a\x02\xECW`\0\x80\xFD[\x80c@\xB41i\x14a\x02iW\x80cM\xDFG\xD4\x14a\x02rW\x80cV\x08\xBE\xA1\x14a\x02\xAFW`\0\x80\xFD[\x80c\n\xC304\x14a\x01\xAEW\x80c\x0F\n\xA3\x95\x14a\x01\xD9W\x80c\x18M0\xBA\x14a\x01\xF9W\x80c\x1A\x88\xBCf\x14a\x02\x1AW\x80c\x1D\x9C\xF7\"\x14a\x02GW\x80c']g\xC8\x14a\x02ZW[`\0\x80\xFD[a\x01\xC1a\x01\xBC6`\x04a\x1F\x06V[a\x04JV[`@Qa\x01\xD0\x93\x92\x91\x90a\x1F}V[`@Q\x80\x91\x03\x90\xF3[a\x01\xECa\x01\xE76`\x04a \x17V[a\x06\xF0V[`@Qa\x01\xD0\x91\x90a WV[a\x02\x0Ca\x02\x076`\x04a jV[a\x07\"V[`@Q\x90\x81R` \x01a\x01\xD0V[`\x01T`\x02T`\x03Ta\x02,\x92\x91\x90\x83V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x01\xD0V[a\x02\x0Ca\x02U6`\x04a \xA0V[a\x07tV[a\x02\x0Cg\x1B\xC1mgN\xC8\0\0\x81V[a\x02\x0C`\0T\x81V[a\x02\x85a\x02\x806`\x04a \xB9V[a\x07\xC2V[`@\x80Q\x95\x15\x15\x86R` \x86\x01\x94\x90\x94R\x92\x84\x01\x91\x90\x91R``\x83\x01R`\x80\x82\x01R`\xA0\x01a\x01\xD0V[a\x02\x0Ca\x02\xBD6`\x04a jV[a\x08^V[a\x02\x0C`\0\x81V[a\x02\x0Ca\x02\xD86`\x04a \x17V[a\x08\x9FV[a\x02\x0Cg\r\xE0\xB6\xB3\xA7d\0\0\x81V[a\x02\x0C`\x01\x81V[a\x02\x0Cg\x06\xF0[Y\xD3\xB2\0\0\x81V[a\x02\x0Ca\x03\x116`\x04a \x17V[a\x08\xFAV[a\x02\x0Ca\x03$6`\x04a!+V[a\tSV[a\x02\x0Ca\x0376`\x04a jV[a\t\xF0V[a\x02\x0Ca\x03J6`\x04a!XV[a\n\x85V[a\x02\x0Ca\x03]6`\x04a \xA0V[a\n\xE0V[a\x01\xECa\x03p6`\x04a\"\tV[`@\x80Q` \x81\x01\x94\x90\x94R\x83\x81\x01\x92\x90\x92R``\x80\x84\x01\x91\x90\x91R\x81Q\x80\x84\x03\x90\x91\x01\x81R`\x80\x90\x92\x01\x90R\x90V[a\x03\xB3a\x03\xAE6`\x04a!XV[a\x0B\tV[`@\x80Q\x96\x15\x15\x87R` \x87\x01\x95\x90\x95R\x93\x85\x01\x92\x90\x92R``\x84\x01R`\x80\x83\x01R`\xA0\x82\x01R`\xC0\x01a\x01\xD0V[a\x02\x0Ca\x03\xF06`\x04a \x17V[a\x0E9V[a\x02\x0Ca\x04\x036`\x04a!+V[a\x0E\x8AV[a\x02\x0Ca\x04\x166`\x04a jV[a\x0E\xB2V[a\x02\x0Ca\x04)6`\x04a\"\tV[a\x0E\xF4V[a\x02\x0Ca\x01\0\x81V[a\x02\x0Ca\x04E6`\x04a\"\tV[a\x0FrV[`\0\x80``a\x04p`@Q\x80`\x80\x01`@R\x80`G\x81R` \x01a$\x9E`G\x919a\x0F\xF9V[`\0\x80`\x003`\x01`\x01`\xA0\x1B\x03\x16c\xEB\xAD\xEF\x01`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01```@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xB3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xD7\x91\x90a\"5V[`@\x80Q` \x81\x01\x85\x90R\x90\x81\x01\x83\x90R``\x81\x01\x82\x90R\x92\x95P\x90\x93P\x91P`\0\x90a\x05\x15\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@Ra\n\x85V[\x90P`\0\x89\x15a\x06\x02W`\0\x80Ta\x05.\x90\x8B\x90a\x10?V[\x90P`\0a\x05F\x87a\x05@\x84\x88a\x10?V[\x90a\x10ZV[\x90Pa\x05S`\x01\x82a\"yV[\x90Pa\x05_\x8B\x88a\"yV[\x96Pa\x05k\x81\x86a\"yV[`@\x80Q``\x81\x01\x82R`\x01T\x81R`\x02T` \x82\x01R`\x03T\x91\x81\x01\x91\x90\x91R\x90\x95P\x86\x90a\x05\xA0\x90\x89\x90\x88\x90\x88\x90a\x08\x9FV[\x96Pa\x05\xAD`\x01\x88a\"yV[\x96Pa\x05\xB9\x87\x82a\"\x8CV[\x93Pa\x05\xFA`@Q\x80`@\x01`@R\x80`\x1C\x81R` \x01\x7FEsimated Y reserve to submit\0\0\0\0\x81RP\x88a\x10oV[PPPa\x06\x9CV[`\0\x80Ta\x06\x11\x90\x8B\x90a\x10?V[\x90P`\0a\x06#\x86a\x05@\x84\x88a\x10?V[\x90Pa\x060`\x01\x82a\"yV[\x90Pa\x06<\x8B\x87a\"yV[\x95Pa\x06H\x81\x86a\"yV[`@\x80Q``\x81\x01\x82R`\x01T\x81R`\x02T` \x82\x01R`\x03T\x91\x81\x01\x91\x90\x91R\x90\x95P\x87\x90a\x06}\x90\x88\x90\x88\x90\x88\x90a\x0E9V[\x97Pa\x06\x8A`\x01\x89a\"yV[\x97Pa\x06\x96\x88\x82a\"\x8CV[\x93PPPP[`@\x80Q` \x81\x01\x87\x90R\x90\x81\x01\x85\x90R``\x81\x01\x84\x90R`\0\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`\0a\x06\xD6\x82a\x0B\tV[P\x93\x9DP\x95\x9BP\x93\x99PPPPPPPPPP\x92P\x92P\x92V[``\x84\x84\x84\x84`@Q` \x01a\x07\t\x94\x93\x92\x91\x90a\"\x9FV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x94\x93PPPPV[`\0\x80a\x07/\x84\x84a\tSV[\x90P`\0a\x07<\x82a\x10\xB8V[\x90P`\0a\x07I\x82a\x07tV[\x90Pa\x07ga\x07`\x82g\r\xE0\xB6\xB3\xA7d\0\0a\"\x8CV[\x88\x90a\x10?V[\x93PPPP[\x93\x92PPPV[`\0\x80\x82\x12\x15a\x07\xBEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01RotoUint: negative`\x80\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[P\x90V[`\0\x80`\0\x80`\0a\x07\xEB`@Q\x80`\x80\x01`@R\x80`G\x81R` \x01a$\x9E`G\x919a\x0F\xF9V[a\x07\xF7\x86\x88\x01\x88a \x17V[\x80Q`\x01\x81\x90U` \x80\x83\x01Q`\x02\x81\x90U`@\x93\x84\x01Q`\x03\x81\x90U\x84Q``\x81\x01\x86R\x93\x84R\x91\x83\x01R\x91\x81\x01\x91\x90\x91R\x92\x95P\x90\x93P\x91Pa\x08A\x90\x84\x90\x84\x90\x84\x90a\x11!V[\x93Pa\x08O`\0`\x03a\"\xD3V[\x84\x12\x15\x94P\x92\x95P\x92\x95\x90\x93PV[`\0\x80a\x08k\x84\x84a\x0E\x8AV[\x90P`\0a\x08x\x82a\x10\xB8V[\x90P`\0a\x08\x85\x82a\x07tV[\x85Q\x90\x91Pa\x07g\x90a\x08\x98\x90\x83a\x11\xD1V[\x88\x90a\x11\xE6V[\x80Q`\0\x90`d\x90\x82\x90a\x08\xB5\x90`\x01\x90a\"\x8CV[\x90Pa\x08\xEF\x87\x87\x87\x87`@Q` \x01a\x08\xD1\x94\x93\x92\x91\x90a\"\x9FV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83\x83`\x01a\x01\0a\x11\xFBa\x126V[\x97\x96PPPPPPPV[`\0\x80a\t\x08\x86`\x01a\"yV[\x90P`\0k\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x90Pa\x08\xEF\x87\x87\x87\x87`@Q` \x01a\t5\x94\x93\x92\x91\x90a\"\x9FV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83\x83`\x01a\x01\0a\x13Ga\x126V[`\0\x80a\th\x83` \x01Q\x84`@\x01Qa\x13xV[\x90P`\0a\ty\x84` \x01Qa\n\xE0V[\x90P`\0a\t\x9Ca\t\x97\x86`\0\x01Q\x88a\x10Z\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x13\xA2V[\x90P\x80a\t\xA8\x81a\"\xFBV[\x91PP`\0a\t\xC4\x86`@\x01Q\x84a\x11\xD1\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\t\xCE\x90\x83a\"\xD3V[\x90P\x83a\t\xE4\x82g\r\xE0\xB6\xB3\xA7d\0\0\x83a\x0FrV[\x98\x97PPPPPPPPV[`\0\x80a\t\xFD\x84\x84a\tSV[\x90P`\0a\n\n\x82a\x10\xB8V[\x90P`\0a\n\x17\x82a\x07tV[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0\x81\x10a\npW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7Flx: denominator is zero\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x07\xB5V[a\x07ga\x08\x98\x82g\r\xE0\xB6\xB3\xA7d\0\0a\"\x8CV[`\0\x80`\0\x80\x84\x80` \x01\x90Q\x81\x01\x90a\n\x9F\x91\x90a\"5V[`@\x80Q``\x81\x01\x82R`\x01T\x81R`\x02T` \x82\x01R`\x03T\x91\x81\x01\x91\x90\x91R\x92\x95P\x90\x93P\x91Pa\n\xD7\x90\x84\x90\x84\x90\x84\x90a\x11!V[\x95\x94PPPPPV[`\0\x80a\n\xF5\x83g\x1B\xC1mgN\xC8\0\0a\x15}V[\x90Pa\x07mg\x06\xF0[Y\xD3\xB2\0\0\x82a\x11\xD1V[`\0\x80`\0\x80`\0\x80a\x0B3`@Q\x80`\x80\x01`@R\x80`G\x81R` \x01a$\x9E`G\x919a\x0F\xF9V[`\0\x80`\x003`\x01`\x01`\xA0\x1B\x03\x16c\xEB\xAD\xEF\x01`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01```@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0BvW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\x9A\x91\x90a\"5V[\x92P\x92P\x92P\x89\x80` \x01\x90Q\x81\x01\x90a\x0B\xB4\x91\x90a\"5V[\x91\x97P\x95P\x93P`\x01\x83\x87\x11\x15a\x0C\rW`\0a\x0B\xD1\x85\x89a\"\x8CV[\x90P`\0a\x0B\xEA`\0T\x83a\x10?\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa\x0B\xFA\x86a\x05@\x83\x87a\x10?V[a\x0C\x04\x90\x84a\"yV[\x92PPPa\x0C\xABV[\x82\x86\x11\x15a\x0CJW`\0a\x0C!\x84\x88a\"\x8CV[\x90P`\0a\x0C:`\0T\x83a\x10?\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa\x0B\xFA\x85a\x05@\x83\x87a\x10?V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`0`$\x82\x01R\x7Finvalid swap: inputs x and y hav`D\x82\x01Roe the same sign!`\x80\x1B`d\x82\x01R`\x84\x01a\x07\xB5V[a\x0C\xB5\x82\x86a#\x1AV[`@\x80Q``\x81\x01\x82R`\x01T\x81R`\x02T` \x82\x01R`\x03T\x91\x81\x01\x91\x90\x91R\x90\x98Pa\x0C\xE8\x90\x85\x90\x85\x90\x85\x90a\x11!V[`@\x80Q``\x81\x01\x82R`\x01T\x81R`\x02T` \x82\x01R`\x03T\x91\x81\x01\x91\x90\x91Ra\r\x18\x90\x89\x90\x89\x90\x89\x90a\x11!V[a\r\"\x91\x90a#\x1AV[\x98Pa\rY`@Q\x80`@\x01`@R\x80`\x14\x81R` \x01s\nn\xEC.\x04\x0Cm\xED\xCEn\x8C-\xCE\x84\x0C\xEEM\xEE\xEE\x8D`c\x1B\x81RPa\x0F\xF9V[a\rb\x89a\x15\xAEV[a\r\xA0`@Q\x80`@\x01`@R\x80`\x19\x81R` \x01\x7FSubmitted Liquidity delta\0\0\0\0\0\0\0\x81RPa\x0F\xF9V[a\r\xA9\x88a\x15\xAEV[a\r\xDD`@Q\x80`@\x01`@R\x80`\x13\x81R` \x01rMin liquidity delta`h\x1B\x81RPa\x0F\xF9V[a\r\xE6\x81a\x15\xAEV[a\x0E\x07`@Q\x80``\x01`@R\x80`%\x81R` \x01a$\xE5`%\x919a\x0F\xF9V[a\x0E\x19a\x0E\x14\x82\x8Aa#\x1AV[a\x15\xAEV[`\0\x89\x12\x15\x80\x15a\x0E*WP\x80\x88\x12\x15[\x99PPPPP\x91\x93\x95P\x91\x93\x95V[`\0`\x02\x81a\x0EP\x82g\r\xE0\xB6\xB3\xA7d\0\0a\"\x8CV[\x90Pa\x08\xEF\x87\x87\x87\x87`@Q` \x01a\x0El\x94\x93\x92\x91\x90a\"\x9FV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83\x83`\x01a\x01\0a\x15\xF3a\x126V[`\0a\x0E\x9E\x82` \x01Q\x83`@\x01Qa\x13xV[a\x0E\xA8\x84\x84a\tSV[a\x07m\x91\x90a#\x1AV[`\0\x80a\x0E\xBF\x84\x84a\x0E\x8AV[\x90P`\0a\x0E\xCC\x82a\x10\xB8V[\x90P`\0a\x0E\xD9\x82a\x07tV[\x85Q\x90\x91Pa\x07g\x90\x82\x90a\x0E\xEE\x90\x8Aa\x10?V[\x90a\x10?V[\x82\x82\x02\x81\x15\x80\x15\x90a\x0F\x1CWP\x83\x15\x80a\x0F\x1CWP\x82\x84\x82\x81a\x0F\x19Wa\x0F\x19a#AV[\x05\x14[a\x0FZW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rn\x1B][\x1AY\x1A]\x88\x1A[\x9D\x98[\x1AY`\x8A\x1B`D\x82\x01R`d\x01a\x07\xB5V[\x81\x81\x81a\x0FiWa\x0Fia#AV[\x05\x94\x93PPPPV[`\0a\x0F\x7F\x84\x84\x84a\x0E\xF4V[\x90P\x81a\x0F\x8C\x84\x86a#WV[a\x0F\x96\x91\x90a#\x87V[\x15a\x07mW`\x01`\x01`\xFF\x1B\x03\x81\x12a\x0F\xE6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01RqmulidivUp overflow`p\x1B`D\x82\x01R`d\x01a\x07\xB5V[a\x0F\xF1`\x01\x82a\"\xD3V[\x94\x93PPPPV[a\x10<\x81`@Q`$\x01a\x10\r\x91\x90a WV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\x10L\x13\xEB`\xE2\x1B\x17\x90Ra\x16$V[PV[`\0a\x07m\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x16EV[\x92\x91PPV[`\0a\x07m\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x16EV[a\x10\xB4\x82\x82`@Q`$\x01a\x10\x85\x92\x91\x90a#\x9BV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c-\x83\x9C\xB3`\xE2\x1B\x17\x90Ra\x16$V[PPV[`\0\x80g\x13\xA0K\xBD\xFD\xC9\xBE\x88a\x10\xD6g\r\xE0\xB6\xB3\xA7d\0\0\x85a#WV[a\x10\xE0\x91\x90a#\xBDV[\x90P`\0a\x10\xED\x82a#\xEBV[\x90P`\0a\x10\xFA\x82a\x16sV[\x90Pg\x1B\xC1mgN\xC8\0\0a\x11\x17g\r\xE0\xB6\xB3\xA7d\0\0\x83a#WV[a\n\xD7\x91\x90a#\xBDV[`\0\x82\x85\x10a\x11rW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FtradingFunction: invalid x\0\0\0\0\0\0`D\x82\x01R`d\x01a\x07\xB5V[`\0a\x11\x86a\x11\x81\x87\x86a\x11\xE6V[a\x18\\V[\x90P`\0a\x11\xA4a\x11\x81\x87a\x11\x9F\x87`\0\x01Q\x89a\x11\xD1V[a\x11\xE6V[\x90P`\0a\x11\xBA\x85` \x01Q\x86`@\x01Qa\x13xV[\x90P\x80a\x11\xC7\x83\x85a\"\xD3V[a\t\xE4\x91\x90a\"\xD3V[`\0a\x07m\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x18\xF9V[`\0a\x07m\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x18\xF9V[`\0\x80`\0\x80`\0\x86\x80` \x01\x90Q\x81\x01\x90a\x12\x17\x91\x90a$\x07V[\x93P\x93P\x93P\x93P\x81a\x12,\x85\x88\x86\x85a\x11!V[a\x08\xEF\x91\x90a#\x1AV[`\0\x84\x86\x11\x15a\x12cW`@Qc0\x82\xDF\xDB`\xE1\x1B\x81R`\x04\x81\x01\x87\x90R`$\x81\x01\x86\x90R`D\x01a\x07\xB5V[`\0a\x12s\x88\x88\x85c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x12\x85\x89\x88\x86c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x12\x93\x82\x84a#WV[\x13\x15a\x12\xBCW`@Qc\x06\xF1\xBE]`\xE2\x1B\x81R`\x04\x81\x01\x83\x90R`$\x81\x01\x82\x90R`D\x01a\x07\xB5V[`\0a\x12\xC8\x89\x89a\"\x8CV[\x90P`\0[`\x02a\x12\xD9\x8A\x8Ca\"yV[a\x12\xE3\x91\x90a$rV[\x94P`\0a\x12\xF5\x8C\x87\x89c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x13\x03\x86\x83a#WV[\x13a\x13\x10W\x85\x99Pa\x13\x17V[\x85\x9AP\x80\x94P[a\x13!\x8B\x8Ba\"\x8CV[\x92PP`\x01\x01\x87\x82\x11\x80\x15a\x135WP\x86\x81\x10[a\x12\xCDWPPPP\x96\x95PPPPPPV[`\0\x80`\0\x80`\0\x86\x80` \x01\x90Q\x81\x01\x90a\x13c\x91\x90a$\x07V[\x93P\x93P\x93P\x93P\x81a\x12,\x85\x85\x89\x85a\x11!V[`\0\x80a\x13\x84\x83a\x19\x18V[\x90P`\0a\x13\x96c;\x9A\xCA\0\x83a$\x86V[\x90Pa\n\xD7\x85\x82a\x11\xD1V[`\0\x80\x82\x13a\x13\xDFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x07\xB5V[`\0``a\x13\xEC\x84a\x19\xBCV[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[`\0a\x07mg\r\xE0\xB6\xB3\xA7d\0\0\x83a\x15\x95\x86a\x13\xA2V[a\x15\x9F\x91\x90a#WV[a\x15\xA9\x91\x90a#\xBDV[a\x1AdV[a\x10<\x81`@Q`$\x01a\x15\xC4\x91\x81R` \x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c-[l\xB9`\xE0\x1B\x17\x90Ra\x16$V[`\0\x80`\0\x80`\0\x86\x80` \x01\x90Q\x81\x01\x90a\x16\x0F\x91\x90a$\x07V[\x93P\x93P\x93P\x93P\x81a\x12,\x87\x86\x86\x85a\x11!V[\x80Qjconsole.log` \x83\x01`\0\x80\x84\x83\x85Z\xFAPPPPPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x16]W`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[`\0\x81`\0\x03a\x16\x8CWPg\r\xE0\xB6\xB3\xA7d\0\0\x91\x90PV[gV\x98\xEE\xF0fp\0\0\x82\x12a\x16\xA3WP`\0\x91\x90PV[a\x16\xB4gV\x98\xEE\xF0fp\0\0a#\xEBV[\x82\x13a\x16\xC9WPg\x1B\xC1mgN\xC8\0\0\x91\x90PV[`\0a\x16\xD4\x83a\x1C\rV[\x90P`\0a\x17\rg\r\xE0\xB6\xB3\xA7d\0\0a\x16\xF6\x84g\x1B\xC1mgN\xC8\0\0a\x11\xE6V[a\x17\x08\x90g\r\xE0\xB6\xB3\xA7d\0\0a\"\xD3V[a\x1CDV[\x90P`\0\x80\x82a\x17n\x81a\x17[\x81a\x17I\x81a\x171\x81g\x02_\x0F\xE1\x05\xA3\x14\0a\x1CYV[a\x17D\x90g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19a\"\xD3V[a\x1CYV[a\x17D\x90g\x14\xA8EL\x19\xE1\xAC\0a\"\xD3V[a\x17D\x90g\x0F\xC1\x0E\x01W\x82w\xFF\x19a\"\xD3V[a\x17\x80\x90g\x03\xDE\xBD\x08;\x8C|\0a\"\xD3V[\x91P\x83\x90Pa\x17\xE8\x81a\x17\xD6\x81a\x17\xC4\x81a\x17\xB2\x81a\x17\x9F\x81\x8Ba\x1CYV[a\x17D\x90g\x02\x95\xD4\0\xEA2W\xFF\x19a\"\xD3V[a\x17D\x90g\x01W\xD8\xB2\xEC\xC7\x08\0a\"\xD3V[a\x17D\x90g\x051\n\xA7\xD5!0\0a\"\xD3V[a\x17D\x90g\r\xE0\xCC=\x15a\0\0a\"\xD3V[\x91P\x81g\x11\x90\0\xAB\x10\x0F\xFC\0a\x17\xFE\x87\x88a\x1CYV[a\x18\n\x90`\0\x19a#WV[a\x18\x14\x91\x90a#\x1AV[a\x18\x1E\x91\x90a\"\xD3V[\x92PP`\0a\x18,\x83a\x1AdV[\x90P`\0a\x18:\x85\x83a\x1CYV[\x90P`\0\x88\x12a\x18JW\x80a\t\xE4V[a\t\xE4\x81g\x1B\xC1mgN\xC8\0\0a#\x1AV[`\0g\x06\xF0[Y\xD3\xB2\0\0\x82\x03a\x18uWP`\0\x91\x90PV[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x12a\x18\x9DW`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\x18\xBEW`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x18\xCB\x83`\x02a#WV[\x90P`\0a\x18\xD8\x82a\x1CnV[\x90P`\0a\x18\xEEg\x13\xA0K\xBD\xFD\xC9\xBE\x88\x83a\x1CYV[\x90Pa\n\xD7\x81a#\xEBV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x19\x11W`\0\x80\xFD[\x04\x92\x91PPV[`\xB5\x81`\x01`\x88\x1B\x81\x10a\x191W`@\x91\x90\x91\x1B\x90`\x80\x1C[i\x01\0\0\0\0\0\0\0\0\0\x81\x10a\x19MW` \x91\x90\x91\x1B\x90`@\x1C[e\x01\0\0\0\0\0\x81\x10a\x19eW`\x10\x91\x90\x91\x1B\x90` \x1C[c\x01\0\0\0\x81\x10a\x19{W`\x08\x91\x90\x91\x1B\x90`\x10\x1C[b\x01\0\0\x01\x02`\x12\x1C\x80\x82\x04\x01`\x01\x90\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x90\x81\x90\x04\x81\x11\x90\x03\x90V[`\0\x80\x82\x11a\x19\xF9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x07\xB5V[P`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x82\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[`\0h\x02H\xCE6\xA7\x0C\xB2k>\x19\x82\x13a\x1A\x7FWP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a\x1A\xC6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x01a\x07\xB5V[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05`\x01`_\x1B\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1Dm\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1Dn\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[`\0`\x01`\xFF\x1B\x82\x03a\x1C3W`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x12\x15a\x07\xBEWP\x19`\x01\x01\x90V[`\0a\x07m\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x1E\xE7V[`\0a\x07m\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x1E\xE7V[`\0\x80\x82\x12\x80a\x1C\x85WPg\x1B\xC1mgN\xC8\0\0\x82\x13[\x15a\x1C\xA3W`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\x1C\xC4W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81g\x1B\xC1mgN\xC8\0\0\x03a\x1C\xECW`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x15a\x1C\xF7W\x91\x90PV[`\0g\r\xE0\xB6\xB3\xA7d\0\0\x83\x12a\x1D\x1FWa\x1D\x1A\x83g\x1B\xC1mgN\xC8\0\0a#\x1AV[a\x1D!V[\x82[\x90P`\0a\x1D7\x82g\x1B\xC1mgN\xC8\0\0a\x1CDV[\x90P\x80`\0\x03a\x1DZW`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x1De\x82a\x13\xA2V[\x90P`\0c;\x9A\xCA\0a\x1D\x90a\x1D\x8Ba\x1D\x85g\x1B\xC1mgN\xC8\0\0a#\xEBV[\x85a\x1CYV[a\x19\x18V[a\x1D\x9A\x91\x90a#WV[\x90P`\0\x80a\x1D\xB1\x83g\x03\xC1f\\z\xAB \0a\x1CYV[a\x1D\xC3\x90g \x05\xFEO&\x8E\xA0\0a\"\xD3V[\x90P`\0a\x1D\xEE\x84a\x1D\xDC\x86f\x9F2u$b\xA0\0a\x1CYV[a\x17D\x90g\r\xC5R\x7Fd, \0a\"\xD3V[a\x1E\0\x90g\r\xE0\xB6\xB3\xA7d\0\0a\"\xD3V[\x90Pa\x1E$g\t\xD0(\xCCo _\xFF\x19\x85a\x1E\x1A\x85\x85a\x1CDV[a\x17D\x91\x90a#\x1AV[\x92PPP`\0[`\x02\x81\x10\x15a\x1E\xBFW`\0\x86a\x1E@\x84a\x16sV[a\x1EJ\x91\x90a#\x1AV[\x90P`\0a\x1EX\x84\x85a\x1CYV[a\x1Ea\x90a#\xEBV[\x90P`\0a\x1En\x82a\x1AdV[\x90P`\0a\x1E|\x86\x85a\x1CYV[a\x1E\x8Eg\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x84a\x1CYV[a\x1E\x98\x91\x90a#\x1AV[\x90Pa\x1E\xA4\x84\x82a\x1CDV[a\x1E\xAE\x90\x87a\"\xD3V[\x95P\x84`\x01\x01\x94PPPPPa\x1E+V[g\r\xE0\xB6\xB3\xA7d\0\0\x88\x12a\x1E\xDCWa\x1E\xD7\x82a#\xEBV[a\t\xE4V[P\x96\x95PPPPPPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x05\x85\x14\x17\x16a\x1E\xFFW`\0\x80\xFD[\x05\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a\x1F\x19W`\0\x80\xFD[\x825\x80\x15\x15\x81\x14a\x1F)W`\0\x80\xFD[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a\x1F]W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\x1FAV[P`\0` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[\x83\x15\x15\x81R\x82` \x82\x01R```@\x82\x01R`\0a\n\xD7``\x83\x01\x84a\x1F7V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x1F\xD7Wa\x1F\xD7a\x1F\x9EV[`@R\x90V[`\0``\x82\x84\x03\x12\x15a\x1F\xEFW`\0\x80\xFD[a\x1F\xF7a\x1F\xB4V[\x90P\x815\x81R` \x82\x015` \x82\x01R`@\x82\x015`@\x82\x01R\x92\x91PPV[`\0\x80`\0\x80`\xC0\x85\x87\x03\x12\x15a -W`\0\x80\xFD[\x845\x93P` \x85\x015\x92P`@\x85\x015\x91Pa L\x86``\x87\x01a\x1F\xDDV[\x90P\x92\x95\x91\x94P\x92PV[` \x81R`\0a\x07m` \x83\x01\x84a\x1F7V[`\0\x80`\0`\xA0\x84\x86\x03\x12\x15a \x7FW`\0\x80\xFD[\x835\x92P` \x84\x015\x91Pa \x97\x85`@\x86\x01a\x1F\xDDV[\x90P\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a \xB2W`\0\x80\xFD[P5\x91\x90PV[`\0\x80` \x83\x85\x03\x12\x15a \xCCW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a \xE4W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a \xF8W`\0\x80\xFD[\x815\x81\x81\x11\x15a!\x07W`\0\x80\xFD[\x86` \x82\x85\x01\x01\x11\x15a!\x19W`\0\x80\xFD[` \x92\x90\x92\x01\x96\x91\x95P\x90\x93PPPPV[`\0\x80`\x80\x83\x85\x03\x12\x15a!>W`\0\x80\xFD[\x825\x91Pa!O\x84` \x85\x01a\x1F\xDDV[\x90P\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a!jW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a!\x82W`\0\x80\xFD[\x81\x84\x01\x91P\x84`\x1F\x83\x01\x12a!\x96W`\0\x80\xFD[\x815\x81\x81\x11\x15a!\xA8Wa!\xA8a\x1F\x9EV[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a!\xD0Wa!\xD0a\x1F\x9EV[\x81`@R\x82\x81R\x87` \x84\x87\x01\x01\x11\x15a!\xE9W`\0\x80\xFD[\x82` \x86\x01` \x83\x017`\0\x92\x81\x01` \x01\x92\x90\x92RP\x95\x94PPPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\"\x1EW`\0\x80\xFD[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\"JW`\0\x80\xFD[\x83Q\x92P` \x84\x01Q\x91P`@\x84\x01Q\x90P\x92P\x92P\x92V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x10TWa\x10Ta\"cV[\x81\x81\x03\x81\x81\x11\x15a\x10TWa\x10Ta\"cV[\x84\x81R` \x80\x82\x01\x85\x90R`@\x80\x83\x01\x85\x90R\x83Q``\x84\x01R\x90\x83\x01Q`\x80\x83\x01R\x82\x01Q`\xA0\x82\x01R`\xC0\x81\x01a\n\xD7V[\x80\x82\x01\x82\x81\x12`\0\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a\"\xF3Wa\"\xF3a\"cV[PP\x92\x91PPV[`\0`\x01`\x01`\xFF\x1B\x01\x82\x01a#\x13Wa#\x13a\"cV[P`\x01\x01\x90V[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a#:Wa#:a\"cV[P\x92\x91PPV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15a#sWa#sa\"cV[\x81\x81\x05\x83\x14\x82\x15\x17a\x10TWa\x10Ta\"cV[`\0\x82a#\x96Wa#\x96a#AV[P\x07\x90V[`@\x81R`\0a#\xAE`@\x83\x01\x85a\x1F7V[\x90P\x82` \x83\x01R\x93\x92PPPV[`\0\x82a#\xCCWa#\xCCa#AV[`\x01`\xFF\x1B\x82\x14`\0\x19\x84\x14\x16\x15a#\xE6Wa#\xE6a\"cV[P\x05\x90V[`\0`\x01`\xFF\x1B\x82\x01a$\0Wa$\0a\"cV[P`\0\x03\x90V[`\0\x80`\0\x80\x84\x86\x03`\xC0\x81\x12\x15a$\x1EW`\0\x80\xFD[\x85Q\x94P` \x86\x01Q\x93P`@\x86\x01Q\x92P```_\x19\x82\x01\x12\x15a$BW`\0\x80\xFD[Pa$Ka\x1F\xB4V[``\x86\x01Q\x81R`\x80\x86\x01Q` \x82\x01R`\xA0\x90\x95\x01Q`@\x86\x01RP\x91\x94\x90\x93P\x90\x91\x90V[`\0\x82a$\x81Wa$\x81a#AV[P\x04\x90V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x10TWa\x10Ta\"cV\xFEMake sure the calling contract of this console.log is the Core contractliquidity delta - min liquidity delta";
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
            (bool, ::ethers::core::types::U256, ::ethers::core::types::Bytes),
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
        BisectionEpsilon(BisectionEpsilonCall),
        HalfWad(HalfWadCall),
        MaxBisectionIters(MaxBisectionItersCall),
        TwoWad(TwoWadCall),
        Wad(WadCall),
        Zero(ZeroCall),
        ComputeD1(ComputeD1Call),
        ComputeD2(ComputeD2Call),
        ComputeHalfSigmaSquared(ComputeHalfSigmaSquaredCall),
        ComputeSwapConstant(ComputeSwapConstantCall),
        EncodeInitData(EncodeInitDataCall),
        EncodeValidateData(EncodeValidateDataCall),
        FindLiquidity(FindLiquidityCall),
        FindX(FindXCall),
        FindY(FindYCall),
        Init(InitCall),
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
                Self::BisectionEpsilon(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HalfWad(element) => ::ethers::core::abi::AbiEncode::encode(element),
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
                Self::BisectionEpsilon(element) => ::core::fmt::Display::fmt(element, f),
                Self::HalfWad(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaxBisectionIters(element) => ::core::fmt::Display::fmt(element, f),
                Self::TwoWad(element) => ::core::fmt::Display::fmt(element, f),
                Self::Wad(element) => ::core::fmt::Display::fmt(element, f),
                Self::Zero(element) => ::core::fmt::Display::fmt(element, f),
                Self::ComputeD1(element) => ::core::fmt::Display::fmt(element, f),
                Self::ComputeD2(element) => ::core::fmt::Display::fmt(element, f),
                Self::ComputeHalfSigmaSquared(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
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
