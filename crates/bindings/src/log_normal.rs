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
                    ::std::borrow::ToOwned::to_owned("__slot__"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("__slot__"),
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
                                        ::std::borrow::ToOwned::to_owned("contract Core"),
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
                            inputs: ::std::vec![],
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
                                        ::std::borrow::ToOwned::to_owned("struct Parameters"),
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
                    ::std::borrow::ToOwned::to_owned("setSigma"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setSigma"),
                            inputs: ::std::vec![
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
                    ::std::borrow::ToOwned::to_owned("staticSlot"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("staticSlot"),
                            inputs: ::std::vec![],
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
                                        ::std::borrow::ToOwned::to_owned("struct Parameters"),
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
                    ::std::borrow::ToOwned::to_owned("targetSigma"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("targetSigma"),
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
                    ::std::borrow::ToOwned::to_owned("targetStrike"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("targetStrike"),
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
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`@Qa\x1E\xE18\x03\x80a\x1E\xE1\x839\x81\x01`@\x81\x90Ra\0|\x91a\x01\x04V[g\r\xE0\xB6\xB3\xA7d\0\0\x81\x10a\0\xEAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7Fswap fee percentage must be less`D\x82\x01Ri than 100%`\xB0\x1B`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[`\x01U`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x163\x17\x90Ua\x01hV[`\0` \x82\x84\x03\x12\x15a\x01aW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[PQ\x91\x90PV[a\x1Dj\x80a\x01w`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x01cW`\x005`\xE0\x1C\x80c\xA1\x9C\xD3\xD1\x11a\0\xEFW\x80c\xC1\xE0\x04;\x11a\0\xBEW\x80c\xC1\xE0\x04;\x14a\x03*W\x80c\xC5)\x87\xCF\x14a\x032W\x80c\xCF\xC4\xAFU\x14a\x03:W\x80c\xEB\xAD\xEF\x01\x14a\x03BW\x80c\xF2\xF4\xEB&\x14a\x03JWa\x01cV[\x80c\xA1\x9C\xD3\xD1\x14a\x02\xC4W\x80c\xAF\xDF1\xCD\x14a\x02\xD7W\x80c\xC1e&\x12\x14a\x02\xDFW\x80c\xC1nP\xEF\x14a\x02\xE8Wa\x01cV[\x80c^aZk\x11a\x016W\x80c^aZk\x14a\x02TW\x80cf\x8F\x90U\x14a\x02wW\x80c\x7F\x0EL\x8C\x14a\x02\x89W\x80c\x8DR\xA1\xFC\x14a\x02\x9EW\x80c\x99\x87\xFEd\x14a\x02\xB1Wa\x01cV[\x80c.-yi\x14a\x01\xC8W\x80c>\x1E3\x92\x14a\x01\xF7W\x80c@\xB41i\x14a\x02\x0EW\x80cM\xDFG\xD4\x14a\x02\x17W[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x01\xD0a\x03uV[`@\x80Q\x82Q\x81R` \x80\x84\x01Q\x90\x82\x01R\x91\x81\x01Q\x90\x82\x01R``\x01[`@Q\x80\x91\x03\x90\xF3[a\x02\0`\x0BT\x81V[`@Q\x90\x81R` \x01a\x01\xEEV[a\x02\0`\x01T\x81V[a\x02*a\x02%6`\x04a\x18\x9CV[a\x03\xBEV[`@\x80Q\x95\x15\x15\x86R` \x86\x01\x94\x90\x94R\x92\x84\x01\x91\x90\x91R``\x83\x01R`\x80\x82\x01R`\xA0\x01a\x01\xEEV[a\x02\\a\x04,V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x01\xEEV[`\x02T`\x03T`\x04Ta\x02\\\x92\x91\x90\x83V[a\x02\x9Ca\x02\x976`\x04a\x19\xC1V[a\x04TV[\0[a\x02\x9Ca\x02\xAC6`\x04a\x19\xC1V[a\x05NV[a\x02\x9Ca\x02\xBF6`\x04a\x19\xC1V[a\x06\x12V[a\x02\0a\x02\xD26`\x04a\x1AVV[a\x06\xD6V[a\x02\0a\x07\rV[a\x02\0`\x06T\x81V[a\x02\xFBa\x02\xF66`\x04a\x1AVV[a\x07}V[`@\x80Q\x96\x15\x15\x87R` \x87\x01\x95\x90\x95R\x93\x85\x01\x92\x90\x92R``\x84\x01R`\x80\x83\x01R`\xA0\x82\x01R`\xC0\x01a\x01\xEEV[a\x01\xD0a\x08\xE9V[a\x02\0a\t4V[a\x02\0a\t\x9FV[a\x02\\a\n\nV[`\0Ta\x03]\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\xEEV[a\x03\x99`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[P`@\x80Q``\x81\x01\x82R`\x02T\x81R`\x03T` \x82\x01R`\x04T\x91\x81\x01\x91\x90\x91R\x90V[`\0\x80\x80\x80\x80a\x03\xD0\x86\x88\x01\x88a\x1BDV[\x80Q`\x02U` \x81\x01Q`\x03U`@\x01Q`\x04U\x91\x94P\x92P\x90Pa\x03\xF3a\n\xE4V[a\x04\x06\x83\x83\x83a\x04\x01a\x08\xE9V[a\x0B3V[\x93P\x83a\x04\x13`\na\x1C\x14V[\x12\x80\x15a\x04 WP`\n\x84\x12[\x94P\x92\x95P\x92\x95\x90\x93PV[`\0\x80`\0a\x049a\t4V[a\x04Aa\x07\rV[a\x04Ia\t\x9FV[\x92P\x92P\x92P\x90\x91\x92V[B\x81\x11a\x04|W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04s\x90a\x1C0V[`@Q\x80\x91\x03\x90\xFD[a\x04\x84a\x0CEV[`\0\x82`\x0FT\x11a\x04\xA1W`\x0FTa\x04\x9C\x90\x84a\x1C[V[a\x04\xAFV[\x82`\x0FTa\x04\xAF\x91\x90a\x1C[V[\x90Pa\x04\xBBB\x83a\x1C[V[a\x04\xC5\x90\x82a\x1CnV[`\x12U`\x10\x83\x90U`\x13\x82\x90U`\x0FT\x7F\x1EK Gg&%\xF4\xDF!\xD4|\xA7\xA0\x0C \x12\xCC\xF7x\xF2w\xBF2(\xDD\xF4Y=o\x84\xAA\x90\x84\x90\x84\x81\x83\x11a\x05\x16W`\x10T`\x0FTa\x05\x11\x91\x90a\x1C[V[a\x05&V[`\x0FT`\x10Ta\x05&\x91\x90a\x1C[V[`@\x80Q\x94\x85R` \x85\x01\x93\x90\x93R\x91\x83\x01R``\x82\x01R`\x80\x01`@Q\x80\x91\x03\x90\xA1PPPV[B\x81\x11a\x05mW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04s\x90a\x1C0V[a\x05ua\x0CVV[`\0\x82`\nT\x11a\x05\x92W`\nTa\x05\x8D\x90\x84a\x1C[V[a\x05\xA0V[\x82`\nTa\x05\xA0\x91\x90a\x1C[V[\x90Pa\x05\xACB\x83a\x1C[V[a\x05\xB6\x90\x82a\x1CnV[`\rU`\x0B\x83\x90U`\x0E\x82\x90U`\nT\x7F$\xBBEgU#\x9Cf@Y\xB5\x19\xE4c\xB4\xE5\xEA\xEF\x05\x08~\x87\xB0\xDF\xED\xF6\x06\x11iR\xAC\xEE\x90\x84\x90\x84\x81\x83\x11a\x06\x02W`\x0BT`\nTa\x05\x11\x91\x90a\x1C[V[`\nT`\x0BTa\x05&\x91\x90a\x1C[V[B\x81\x11a\x061W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04s\x90a\x1C0V[a\x069a\x0CgV[`\0\x82`\x05T\x11a\x06VW`\x05Ta\x06Q\x90\x84a\x1C[V[a\x06dV[\x82`\x05Ta\x06d\x91\x90a\x1C[V[\x90Pa\x06pB\x83a\x1C[V[a\x06z\x90\x82a\x1CnV[`\x08U`\x06\x83\x90U`\t\x82\x90U`\x05T\x7F\xB9hO\xE86\x87_\xAF\x05m\xB2]\xE9\x85S\x859`\xB3&\x08\x8A\x10!\xAF\xA7[\xA1`\x12d\xF5\x90\x84\x90\x84\x81\x83\x11a\x06\xC6W`\x06T`\x05Ta\x05\x11\x91\x90a\x1C[V[`\x05T`\x06Ta\x05&\x91\x90a\x1C[V[`\0\x80`\0\x80\x84\x80` \x01\x90Q\x81\x01\x90a\x06\xF0\x91\x90a\x1C\x90V[\x92P\x92P\x92Pa\x07\x04\x83\x83\x83a\x04\x01a\x08\xE9V[\x95\x94PPPPPV[`\0`\tTB\x10a\x07\x1FWP`\x06T\x90V[`\x06T`\x05T\x11a\x07VW`\x08T`\x07Ta\x07:\x90Ba\x1C[V[a\x07D\x91\x90a\x1C\xC1V[`\x05Ta\x07Q\x91\x90a\x1C\xD8V[\x90P\x90V[`\x08T`\x07Ta\x07f\x90Ba\x1C[V[a\x07p\x91\x90a\x1C\xC1V[`\x05Ta\x07Q\x91\x90a\x1C[V[`\0\x80`\0\x80`\0\x80`\0\x80`\0a\x07\x93a\n\nV[\x92P\x92P\x92P\x89\x80` \x01\x90Q\x81\x01\x90a\x07\xAD\x91\x90a\x1C\x90V[\x91\x97P\x95P\x93P`\0\x80\x80\x85\x89\x11\x15a\x08\x08Wa\x07\xCA\x86\x8Aa\x1C[V[\x91Pa\x07\xE1`\x01T\x83a\x0Cx\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa\x07\xF7\x86a\x07\xF1\x83\x87a\x0CxV[\x90a\x0C\x96V[a\x08\x01\x90\x84a\x1C\xD8V[\x92Pa\x08\xA2V[\x84\x88\x11\x15a\x08AWa\x08\x1A\x85\x89a\x1C[V[\x91Pa\x081`\x01T\x83a\x0Cx\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa\x07\xF7\x85a\x07\xF1\x83\x87a\x0CxV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`0`$\x82\x01R\x7Finvalid swap: inputs x and y hav`D\x82\x01Roe the same sign!`\x80\x1B`d\x82\x01R`\x84\x01a\x04sV[a\x08\xAC\x84\x88a\x1C\xEBV[\x99P`\0\x8Ba\x08\xBB`\na\x1C\x14V[\x12\x80\x15a\x08\xC8WP`\n\x8C\x12[\x90P\x80\x80\x15a\x08\xD7WP\x83\x8B\x12\x15[\x9CPPPPPPPP\x91\x93\x95P\x91\x93\x95V[a\t\r`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[a\t\x15a\t4V[a\t\x1Da\x07\rV[a\t%a\t\x9FV[`@\x84\x01R` \x83\x01R\x81R\x90V[`\0`\x0ETB\x10a\tFWP`\x0BT\x90V[`\x0BT`\nT\x11a\txW`\rT`\x0CTa\ta\x90Ba\x1C[V[a\tk\x91\x90a\x1C\xC1V[`\nTa\x07Q\x91\x90a\x1C\xD8V[`\rT`\x0CTa\t\x88\x90Ba\x1C[V[a\t\x92\x91\x90a\x1C\xC1V[`\nTa\x07Q\x91\x90a\x1C[V[`\0`\x13TB\x10a\t\xB1WP`\x10T\x90V[`\x10T`\x0FT\x11a\t\xE3W`\x12T`\x11Ta\t\xCC\x90Ba\x1C[V[a\t\xD6\x91\x90a\x1C\xC1V[`\x0FTa\x07Q\x91\x90a\x1C\xD8V[`\x12T`\x11Ta\t\xF3\x90Ba\x1C[V[a\t\xFD\x91\x90a\x1C\xC1V[`\x0FTa\x07Q\x91\x90a\x1C[V[`\0\x80`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEB\xAD\xEF\x01`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01```@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\n\xACW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\n\xC0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04I\x91\x90a\x1C\x90V[`\0a\n\xEEa\x03uV[` \x81\x01Q`\x06\x81\x90U`\x05UB`\t\x81\x90U`\x07\x81\x90U\x81Q`\x0B\x81\x90U`\nU`\x0E\x81\x90U`\x0C\x81\x90U`@\x90\x91\x01Q`\x10\x81\x90U`\x0FU`\x13\x81\x90U`\x11UPV[`\0\x82\x85\x10a\x0B\x84W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FtradingFunction: invalid x\0\0\0\0\0\0`D\x82\x01R`d\x01a\x04sV[`\0\x80g\r\xE0\xB6\xB3\xA7d\0\0a\x0B\x9A\x88\x87a\x0C\xABV[\x10a\x0B\xAEW`\x01`\x01`\xFF\x1B\x03\x91Pa\x0B\xC3V[a\x0B\xC0a\x0B\xBB\x88\x87a\x0C\xABV[a\x0C\xC0V[\x91P[g\r\xE0\xB6\xB3\xA7d\0\0a\x0B\xE3\x87a\x0B\xDE\x87`\0\x01Q\x89a\r]V[a\x0C\xABV[\x10a\x0B\xF6WP`\x01`\x01`\xFF\x1B\x03a\x0C\x0EV[a\x0C\x0Ba\x0B\xBB\x87a\x0B\xDE\x87`\0\x01Q\x89a\r]V[\x90P[`\0a\x0C\"\x85` \x01Q\x86`@\x01Qa\rrV[\x90P\x80a\x0C/\x83\x85a\x1D\x12V[a\x0C9\x91\x90a\x1D\x12V[\x98\x97PPPPPPPPV[a\x0CMa\t\x9FV[`\x0FUB`\x11UV[a\x0C^a\t4V[`\nUB`\x0CUV[a\x0Coa\x07\rV[`\x05UB`\x07UV[`\0a\x0C\x8D\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\r\xA0V[\x90P[\x92\x91PPV[`\0a\x0C\x8D\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\r\xA0V[`\0a\x0C\x8D\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\r\xCEV[`\0g\x06\xF0[Y\xD3\xB2\0\0\x82\x03a\x0C\xD9WP`\0\x91\x90PV[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x12a\r\x01W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\r\"W`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\r/\x83`\x02a\x1D:V[\x90P`\0a\r<\x82a\r\xEDV[\x90P`\0a\rRg\x13\xA0K\xBD\xFD\xC9\xBE\x88\x83a\x10kV[\x90Pa\x07\x04\x81a\x1C\x14V[`\0a\x0C\x8D\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\r\xCEV[`\0\x80a\r~\x83a\x10\x80V[a\r\x8C\x90c;\x9A\xCA\0a\x1C\xC1V[\x90Pa\r\x98\x84\x82a\r]V[\x94\x93PPPPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\r\xB8W`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\r\xE6W`\0\x80\xFD[\x04\x92\x91PPV[`\0\x80\x82\x12\x80a\x0E\x04WPg\x1B\xC1mgN\xC8\0\0\x82\x13[\x15a\x0E\"W`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\x0ECW`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81g\x1B\xC1mgN\xC8\0\0\x03a\x0EkW`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x15a\x0EvW\x91\x90PV[`\0g\r\xE0\xB6\xB3\xA7d\0\0\x83\x12a\x0E\x9EWa\x0E\x99\x83g\x1B\xC1mgN\xC8\0\0a\x1C\xEBV[a\x0E\xA0V[\x82[\x90P`\0a\x0E\xB6\x82g\x1B\xC1mgN\xC8\0\0a\x11$V[\x90P\x80`\0\x03a\x0E\xD9W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x0E\xE4\x82a\x119V[\x90P`\0c;\x9A\xCA\0a\x0F\x0Fa\x0F\na\x0F\x04g\x1B\xC1mgN\xC8\0\0a\x1C\x14V[\x85a\x10kV[a\x10\x80V[a\x0F\x19\x91\x90a\x1D:V[\x90P`\0\x80a\x0F0\x83g\x03\xC1f\\z\xAB \0a\x10kV[a\x0FB\x90g \x05\xFEO&\x8E\xA0\0a\x1D\x12V[\x90P`\0a\x0Fr\x84a\x0F[\x86f\x9F2u$b\xA0\0a\x10kV[a\x0Fm\x90g\r\xC5R\x7Fd, \0a\x1D\x12V[a\x10kV[a\x0F\x84\x90g\r\xE0\xB6\xB3\xA7d\0\0a\x1D\x12V[\x90Pa\x0F\xA8g\t\xD0(\xCCo _\xFF\x19\x85a\x0F\x9E\x85\x85a\x11$V[a\x0Fm\x91\x90a\x1C\xEBV[\x92PPP`\0[`\x02\x81\x10\x15a\x10CW`\0\x86a\x0F\xC4\x84a\x13\x14V[a\x0F\xCE\x91\x90a\x1C\xEBV[\x90P`\0a\x0F\xDC\x84\x85a\x10kV[a\x0F\xE5\x90a\x1C\x14V[\x90P`\0a\x0F\xF2\x82a\x14\xF8V[\x90P`\0a\x10\0\x86\x85a\x10kV[a\x10\x12g\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x84a\x10kV[a\x10\x1C\x91\x90a\x1C\xEBV[\x90Pa\x10(\x84\x82a\x11$V[a\x102\x90\x87a\x1D\x12V[\x95P\x84`\x01\x01\x94PPPPPa\x0F\xAFV[g\r\xE0\xB6\xB3\xA7d\0\0\x88\x12a\x10`Wa\x10[\x82a\x1C\x14V[a\x0C9V[P\x96\x95PPPPPPV[`\0a\x0C\x8D\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x16\xA1V[`\xB5\x81`\x01`\x88\x1B\x81\x10a\x10\x99W`@\x91\x90\x91\x1B\x90`\x80\x1C[i\x01\0\0\0\0\0\0\0\0\0\x81\x10a\x10\xB5W` \x91\x90\x91\x1B\x90`@\x1C[e\x01\0\0\0\0\0\x81\x10a\x10\xCDW`\x10\x91\x90\x91\x1B\x90` \x1C[c\x01\0\0\0\x81\x10a\x10\xE3W`\x08\x91\x90\x91\x1B\x90`\x10\x1C[b\x01\0\0\x01\x02`\x12\x1C\x80\x82\x04\x01`\x01\x90\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x90\x81\x90\x04\x81\x11\x90\x03\x90V[`\0a\x0C\x8D\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x16\xA1V[`\0\x80\x82\x13a\x11vW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x04sV[`\0``a\x11\x83\x84a\x16\xC0V[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[`\0\x81`\0\x03a\x13-WPg\r\xE0\xB6\xB3\xA7d\0\0\x91\x90PV[gV\x98\xEE\xF0fp\0\0\x82\x12a\x13DWP`\0\x91\x90PV[a\x13UgV\x98\xEE\xF0fp\0\0a\x1C\x14V[\x82\x13a\x13jWPg\x1B\xC1mgN\xC8\0\0\x91\x90PV[`\0a\x13u\x83a\x17hV[\x90P`\0a\x13\xAEg\r\xE0\xB6\xB3\xA7d\0\0a\x13\x97\x84g\x1B\xC1mgN\xC8\0\0a\x0C\xABV[a\x13\xA9\x90g\r\xE0\xB6\xB3\xA7d\0\0a\x1D\x12V[a\x11$V[\x90P`\0\x80\x82a\x14\n\x81a\x13\xF7\x81a\x13\xE5\x81a\x13\xD2\x81g\x02_\x0F\xE1\x05\xA3\x14\0a\x10kV[a\x0Fm\x90g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19a\x1D\x12V[a\x0Fm\x90g\x14\xA8EL\x19\xE1\xAC\0a\x1D\x12V[a\x0Fm\x90g\x0F\xC1\x0E\x01W\x82w\xFF\x19a\x1D\x12V[a\x14\x1C\x90g\x03\xDE\xBD\x08;\x8C|\0a\x1D\x12V[\x91P\x83\x90Pa\x14\x84\x81a\x14r\x81a\x14`\x81a\x14N\x81a\x14;\x81\x8Ba\x10kV[a\x0Fm\x90g\x02\x95\xD4\0\xEA2W\xFF\x19a\x1D\x12V[a\x0Fm\x90g\x01W\xD8\xB2\xEC\xC7\x08\0a\x1D\x12V[a\x0Fm\x90g\x051\n\xA7\xD5!0\0a\x1D\x12V[a\x0Fm\x90g\r\xE0\xCC=\x15a\0\0a\x1D\x12V[\x91P\x81g\x11\x90\0\xAB\x10\x0F\xFC\0a\x14\x9A\x87\x88a\x10kV[a\x14\xA6\x90`\0\x19a\x1D:V[a\x14\xB0\x91\x90a\x1C\xEBV[a\x14\xBA\x91\x90a\x1D\x12V[\x92PP`\0a\x14\xC8\x83a\x14\xF8V[\x90P`\0a\x14\xD6\x85\x83a\x10kV[\x90P`\0\x88\x12a\x14\xE6W\x80a\x0C9V[a\x0C9\x81g\x1B\xC1mgN\xC8\0\0a\x1C\xEBV[`\0h\x02H\xCE6\xA7\x0C\xB2k>\x19\x82\x13a\x15\x13WP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a\x15ZW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x01a\x04sV[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05`\x01`_\x1B\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1Dm\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1Dn\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x05\x85\x14\x17\x16a\x16\xB9W`\0\x80\xFD[\x05\x92\x91PPV[`\0\x80\x82\x11a\x16\xFDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x04sV[P`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x82\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[`\0`\x01`\xFF\x1B\x82\x03a\x17\x8EW`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x12\x15a\x17\x9FWP\x19`\x01\x01\x90V[P\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x81\xFD[`\0\x80` \x83\x85\x03\x12\x15a\x18\xB2Wa\x18\xB2a\x17\xA3V[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x18\xCDWa\x18\xCDa\x17\xF3V[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x18\xE4Wa\x18\xE4a\x18CV[\x815\x81\x81\x11\x15a\x19GW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x0ENL/$\r\x8C\xAD\xCC\xEE\x8D`\xAB\x1B`d\x82\x01R`\x84\x81\xFD[\x86` \x82\x85\x01\x01\x11\x15a\x19\xAFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rjrray stride`\xA8\x1B`d\x82\x01R\x91P`\x84\x82\xFD[` \x92\x90\x92\x01\x96\x91\x95P\x90\x93PPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x19\xD7Wa\x19\xD7a\x17\xA3V[PP\x805\x92` \x90\x91\x015\x91PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x1A\x1FWa\x1A\x1Fa\x19\xE6V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x1ANWa\x1ANa\x19\xE6V[`@R\x91\x90PV[`\0` \x80\x83\x85\x03\x12\x15a\x1AlWa\x1Ala\x17\xA3V[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x1A\x87Wa\x1A\x87a\x17\xF3V[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x1A\x9EWa\x1A\x9Ea\x18CV[\x815\x81\x81\x11\x15a\x1A\xB0Wa\x1A\xB0a\x19\xE6V[a\x1A\xC2`\x1F\x82\x01`\x1F\x19\x16\x85\x01a\x1A%V[\x91P\x80\x82R\x86\x84\x82\x85\x01\x01\x11\x15a\x1B(W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x85\x90R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R`\x84\x81\xFD[\x80\x84\x84\x01\x85\x84\x017`\0\x90\x82\x01\x90\x93\x01\x92\x90\x92RP\x93\x92PPPV[`\0\x80`\0\x80\x84\x86\x03`\xC0\x81\x12\x15a\x1B^Wa\x1B^a\x17\xA3V[\x855\x94P` \x86\x015\x93P`@\x86\x015\x92P```_\x19\x82\x01\x12\x15a\x1B\xCEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FABI decoding: struct data too sh`D\x82\x01Rb\x1B\xDC\x9D`\xEA\x1B`d\x82\x01R`\x84\x81\xFD[Pa\x1B\xD7a\x19\xFCV[``\x86\x015\x81R`\x80\x86\x015` \x82\x01R`\xA0\x90\x95\x015`@\x86\x01RP\x91\x94\x90\x93P\x90\x91\x90V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01`\xFF\x1B\x82\x01a\x1C)Wa\x1C)a\x1B\xFEV[P`\0\x03\x90V[` \x80\x82R`\x11\x90\x82\x01Rp\x15\\\x19\x18]\x19H\x19[\x99\x08\x1C\x18\\\xDC\xD9Y`z\x1B`@\x82\x01R``\x01\x90V[\x81\x81\x03\x81\x81\x11\x15a\x0C\x90Wa\x0C\x90a\x1B\xFEV[`\0\x82a\x1C\x8BWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[`\0\x80`\0``\x84\x86\x03\x12\x15a\x1C\xA8Wa\x1C\xA8a\x17\xA3V[\x83Q\x92P` \x84\x01Q\x91P`@\x84\x01Q\x90P\x92P\x92P\x92V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x0C\x90Wa\x0C\x90a\x1B\xFEV[\x80\x82\x01\x80\x82\x11\x15a\x0C\x90Wa\x0C\x90a\x1B\xFEV[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a\x1D\x0BWa\x1D\x0Ba\x1B\xFEV[P\x92\x91PPV[\x80\x82\x01\x82\x81\x12`\0\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a\x1D2Wa\x1D2a\x1B\xFEV[PP\x92\x91PPV[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15a\x1DVWa\x1DVa\x1B\xFEV[\x81\x81\x05\x83\x14\x82\x15\x17a\x0C\x90Wa\x0C\x90a\x1B\xFEV";
    /// The bytecode of the contract.
    pub static LOGNORMAL_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x01cW`\x005`\xE0\x1C\x80c\xA1\x9C\xD3\xD1\x11a\0\xEFW\x80c\xC1\xE0\x04;\x11a\0\xBEW\x80c\xC1\xE0\x04;\x14a\x03*W\x80c\xC5)\x87\xCF\x14a\x032W\x80c\xCF\xC4\xAFU\x14a\x03:W\x80c\xEB\xAD\xEF\x01\x14a\x03BW\x80c\xF2\xF4\xEB&\x14a\x03JWa\x01cV[\x80c\xA1\x9C\xD3\xD1\x14a\x02\xC4W\x80c\xAF\xDF1\xCD\x14a\x02\xD7W\x80c\xC1e&\x12\x14a\x02\xDFW\x80c\xC1nP\xEF\x14a\x02\xE8Wa\x01cV[\x80c^aZk\x11a\x016W\x80c^aZk\x14a\x02TW\x80cf\x8F\x90U\x14a\x02wW\x80c\x7F\x0EL\x8C\x14a\x02\x89W\x80c\x8DR\xA1\xFC\x14a\x02\x9EW\x80c\x99\x87\xFEd\x14a\x02\xB1Wa\x01cV[\x80c.-yi\x14a\x01\xC8W\x80c>\x1E3\x92\x14a\x01\xF7W\x80c@\xB41i\x14a\x02\x0EW\x80cM\xDFG\xD4\x14a\x02\x17W[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x01\xD0a\x03uV[`@\x80Q\x82Q\x81R` \x80\x84\x01Q\x90\x82\x01R\x91\x81\x01Q\x90\x82\x01R``\x01[`@Q\x80\x91\x03\x90\xF3[a\x02\0`\x0BT\x81V[`@Q\x90\x81R` \x01a\x01\xEEV[a\x02\0`\x01T\x81V[a\x02*a\x02%6`\x04a\x18\x9CV[a\x03\xBEV[`@\x80Q\x95\x15\x15\x86R` \x86\x01\x94\x90\x94R\x92\x84\x01\x91\x90\x91R``\x83\x01R`\x80\x82\x01R`\xA0\x01a\x01\xEEV[a\x02\\a\x04,V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x01\xEEV[`\x02T`\x03T`\x04Ta\x02\\\x92\x91\x90\x83V[a\x02\x9Ca\x02\x976`\x04a\x19\xC1V[a\x04TV[\0[a\x02\x9Ca\x02\xAC6`\x04a\x19\xC1V[a\x05NV[a\x02\x9Ca\x02\xBF6`\x04a\x19\xC1V[a\x06\x12V[a\x02\0a\x02\xD26`\x04a\x1AVV[a\x06\xD6V[a\x02\0a\x07\rV[a\x02\0`\x06T\x81V[a\x02\xFBa\x02\xF66`\x04a\x1AVV[a\x07}V[`@\x80Q\x96\x15\x15\x87R` \x87\x01\x95\x90\x95R\x93\x85\x01\x92\x90\x92R``\x84\x01R`\x80\x83\x01R`\xA0\x82\x01R`\xC0\x01a\x01\xEEV[a\x01\xD0a\x08\xE9V[a\x02\0a\t4V[a\x02\0a\t\x9FV[a\x02\\a\n\nV[`\0Ta\x03]\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\xEEV[a\x03\x99`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[P`@\x80Q``\x81\x01\x82R`\x02T\x81R`\x03T` \x82\x01R`\x04T\x91\x81\x01\x91\x90\x91R\x90V[`\0\x80\x80\x80\x80a\x03\xD0\x86\x88\x01\x88a\x1BDV[\x80Q`\x02U` \x81\x01Q`\x03U`@\x01Q`\x04U\x91\x94P\x92P\x90Pa\x03\xF3a\n\xE4V[a\x04\x06\x83\x83\x83a\x04\x01a\x08\xE9V[a\x0B3V[\x93P\x83a\x04\x13`\na\x1C\x14V[\x12\x80\x15a\x04 WP`\n\x84\x12[\x94P\x92\x95P\x92\x95\x90\x93PV[`\0\x80`\0a\x049a\t4V[a\x04Aa\x07\rV[a\x04Ia\t\x9FV[\x92P\x92P\x92P\x90\x91\x92V[B\x81\x11a\x04|W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04s\x90a\x1C0V[`@Q\x80\x91\x03\x90\xFD[a\x04\x84a\x0CEV[`\0\x82`\x0FT\x11a\x04\xA1W`\x0FTa\x04\x9C\x90\x84a\x1C[V[a\x04\xAFV[\x82`\x0FTa\x04\xAF\x91\x90a\x1C[V[\x90Pa\x04\xBBB\x83a\x1C[V[a\x04\xC5\x90\x82a\x1CnV[`\x12U`\x10\x83\x90U`\x13\x82\x90U`\x0FT\x7F\x1EK Gg&%\xF4\xDF!\xD4|\xA7\xA0\x0C \x12\xCC\xF7x\xF2w\xBF2(\xDD\xF4Y=o\x84\xAA\x90\x84\x90\x84\x81\x83\x11a\x05\x16W`\x10T`\x0FTa\x05\x11\x91\x90a\x1C[V[a\x05&V[`\x0FT`\x10Ta\x05&\x91\x90a\x1C[V[`@\x80Q\x94\x85R` \x85\x01\x93\x90\x93R\x91\x83\x01R``\x82\x01R`\x80\x01`@Q\x80\x91\x03\x90\xA1PPPV[B\x81\x11a\x05mW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04s\x90a\x1C0V[a\x05ua\x0CVV[`\0\x82`\nT\x11a\x05\x92W`\nTa\x05\x8D\x90\x84a\x1C[V[a\x05\xA0V[\x82`\nTa\x05\xA0\x91\x90a\x1C[V[\x90Pa\x05\xACB\x83a\x1C[V[a\x05\xB6\x90\x82a\x1CnV[`\rU`\x0B\x83\x90U`\x0E\x82\x90U`\nT\x7F$\xBBEgU#\x9Cf@Y\xB5\x19\xE4c\xB4\xE5\xEA\xEF\x05\x08~\x87\xB0\xDF\xED\xF6\x06\x11iR\xAC\xEE\x90\x84\x90\x84\x81\x83\x11a\x06\x02W`\x0BT`\nTa\x05\x11\x91\x90a\x1C[V[`\nT`\x0BTa\x05&\x91\x90a\x1C[V[B\x81\x11a\x061W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04s\x90a\x1C0V[a\x069a\x0CgV[`\0\x82`\x05T\x11a\x06VW`\x05Ta\x06Q\x90\x84a\x1C[V[a\x06dV[\x82`\x05Ta\x06d\x91\x90a\x1C[V[\x90Pa\x06pB\x83a\x1C[V[a\x06z\x90\x82a\x1CnV[`\x08U`\x06\x83\x90U`\t\x82\x90U`\x05T\x7F\xB9hO\xE86\x87_\xAF\x05m\xB2]\xE9\x85S\x859`\xB3&\x08\x8A\x10!\xAF\xA7[\xA1`\x12d\xF5\x90\x84\x90\x84\x81\x83\x11a\x06\xC6W`\x06T`\x05Ta\x05\x11\x91\x90a\x1C[V[`\x05T`\x06Ta\x05&\x91\x90a\x1C[V[`\0\x80`\0\x80\x84\x80` \x01\x90Q\x81\x01\x90a\x06\xF0\x91\x90a\x1C\x90V[\x92P\x92P\x92Pa\x07\x04\x83\x83\x83a\x04\x01a\x08\xE9V[\x95\x94PPPPPV[`\0`\tTB\x10a\x07\x1FWP`\x06T\x90V[`\x06T`\x05T\x11a\x07VW`\x08T`\x07Ta\x07:\x90Ba\x1C[V[a\x07D\x91\x90a\x1C\xC1V[`\x05Ta\x07Q\x91\x90a\x1C\xD8V[\x90P\x90V[`\x08T`\x07Ta\x07f\x90Ba\x1C[V[a\x07p\x91\x90a\x1C\xC1V[`\x05Ta\x07Q\x91\x90a\x1C[V[`\0\x80`\0\x80`\0\x80`\0\x80`\0a\x07\x93a\n\nV[\x92P\x92P\x92P\x89\x80` \x01\x90Q\x81\x01\x90a\x07\xAD\x91\x90a\x1C\x90V[\x91\x97P\x95P\x93P`\0\x80\x80\x85\x89\x11\x15a\x08\x08Wa\x07\xCA\x86\x8Aa\x1C[V[\x91Pa\x07\xE1`\x01T\x83a\x0Cx\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa\x07\xF7\x86a\x07\xF1\x83\x87a\x0CxV[\x90a\x0C\x96V[a\x08\x01\x90\x84a\x1C\xD8V[\x92Pa\x08\xA2V[\x84\x88\x11\x15a\x08AWa\x08\x1A\x85\x89a\x1C[V[\x91Pa\x081`\x01T\x83a\x0Cx\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa\x07\xF7\x85a\x07\xF1\x83\x87a\x0CxV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`0`$\x82\x01R\x7Finvalid swap: inputs x and y hav`D\x82\x01Roe the same sign!`\x80\x1B`d\x82\x01R`\x84\x01a\x04sV[a\x08\xAC\x84\x88a\x1C\xEBV[\x99P`\0\x8Ba\x08\xBB`\na\x1C\x14V[\x12\x80\x15a\x08\xC8WP`\n\x8C\x12[\x90P\x80\x80\x15a\x08\xD7WP\x83\x8B\x12\x15[\x9CPPPPPPPP\x91\x93\x95P\x91\x93\x95V[a\t\r`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[a\t\x15a\t4V[a\t\x1Da\x07\rV[a\t%a\t\x9FV[`@\x84\x01R` \x83\x01R\x81R\x90V[`\0`\x0ETB\x10a\tFWP`\x0BT\x90V[`\x0BT`\nT\x11a\txW`\rT`\x0CTa\ta\x90Ba\x1C[V[a\tk\x91\x90a\x1C\xC1V[`\nTa\x07Q\x91\x90a\x1C\xD8V[`\rT`\x0CTa\t\x88\x90Ba\x1C[V[a\t\x92\x91\x90a\x1C\xC1V[`\nTa\x07Q\x91\x90a\x1C[V[`\0`\x13TB\x10a\t\xB1WP`\x10T\x90V[`\x10T`\x0FT\x11a\t\xE3W`\x12T`\x11Ta\t\xCC\x90Ba\x1C[V[a\t\xD6\x91\x90a\x1C\xC1V[`\x0FTa\x07Q\x91\x90a\x1C\xD8V[`\x12T`\x11Ta\t\xF3\x90Ba\x1C[V[a\t\xFD\x91\x90a\x1C\xC1V[`\x0FTa\x07Q\x91\x90a\x1C[V[`\0\x80`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEB\xAD\xEF\x01`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01```@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\n\xACW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\n\xC0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04I\x91\x90a\x1C\x90V[`\0a\n\xEEa\x03uV[` \x81\x01Q`\x06\x81\x90U`\x05UB`\t\x81\x90U`\x07\x81\x90U\x81Q`\x0B\x81\x90U`\nU`\x0E\x81\x90U`\x0C\x81\x90U`@\x90\x91\x01Q`\x10\x81\x90U`\x0FU`\x13\x81\x90U`\x11UPV[`\0\x82\x85\x10a\x0B\x84W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FtradingFunction: invalid x\0\0\0\0\0\0`D\x82\x01R`d\x01a\x04sV[`\0\x80g\r\xE0\xB6\xB3\xA7d\0\0a\x0B\x9A\x88\x87a\x0C\xABV[\x10a\x0B\xAEW`\x01`\x01`\xFF\x1B\x03\x91Pa\x0B\xC3V[a\x0B\xC0a\x0B\xBB\x88\x87a\x0C\xABV[a\x0C\xC0V[\x91P[g\r\xE0\xB6\xB3\xA7d\0\0a\x0B\xE3\x87a\x0B\xDE\x87`\0\x01Q\x89a\r]V[a\x0C\xABV[\x10a\x0B\xF6WP`\x01`\x01`\xFF\x1B\x03a\x0C\x0EV[a\x0C\x0Ba\x0B\xBB\x87a\x0B\xDE\x87`\0\x01Q\x89a\r]V[\x90P[`\0a\x0C\"\x85` \x01Q\x86`@\x01Qa\rrV[\x90P\x80a\x0C/\x83\x85a\x1D\x12V[a\x0C9\x91\x90a\x1D\x12V[\x98\x97PPPPPPPPV[a\x0CMa\t\x9FV[`\x0FUB`\x11UV[a\x0C^a\t4V[`\nUB`\x0CUV[a\x0Coa\x07\rV[`\x05UB`\x07UV[`\0a\x0C\x8D\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\r\xA0V[\x90P[\x92\x91PPV[`\0a\x0C\x8D\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\r\xA0V[`\0a\x0C\x8D\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\r\xCEV[`\0g\x06\xF0[Y\xD3\xB2\0\0\x82\x03a\x0C\xD9WP`\0\x91\x90PV[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x12a\r\x01W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\r\"W`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\r/\x83`\x02a\x1D:V[\x90P`\0a\r<\x82a\r\xEDV[\x90P`\0a\rRg\x13\xA0K\xBD\xFD\xC9\xBE\x88\x83a\x10kV[\x90Pa\x07\x04\x81a\x1C\x14V[`\0a\x0C\x8D\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\r\xCEV[`\0\x80a\r~\x83a\x10\x80V[a\r\x8C\x90c;\x9A\xCA\0a\x1C\xC1V[\x90Pa\r\x98\x84\x82a\r]V[\x94\x93PPPPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\r\xB8W`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\r\xE6W`\0\x80\xFD[\x04\x92\x91PPV[`\0\x80\x82\x12\x80a\x0E\x04WPg\x1B\xC1mgN\xC8\0\0\x82\x13[\x15a\x0E\"W`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\x0ECW`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81g\x1B\xC1mgN\xC8\0\0\x03a\x0EkW`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x15a\x0EvW\x91\x90PV[`\0g\r\xE0\xB6\xB3\xA7d\0\0\x83\x12a\x0E\x9EWa\x0E\x99\x83g\x1B\xC1mgN\xC8\0\0a\x1C\xEBV[a\x0E\xA0V[\x82[\x90P`\0a\x0E\xB6\x82g\x1B\xC1mgN\xC8\0\0a\x11$V[\x90P\x80`\0\x03a\x0E\xD9W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x0E\xE4\x82a\x119V[\x90P`\0c;\x9A\xCA\0a\x0F\x0Fa\x0F\na\x0F\x04g\x1B\xC1mgN\xC8\0\0a\x1C\x14V[\x85a\x10kV[a\x10\x80V[a\x0F\x19\x91\x90a\x1D:V[\x90P`\0\x80a\x0F0\x83g\x03\xC1f\\z\xAB \0a\x10kV[a\x0FB\x90g \x05\xFEO&\x8E\xA0\0a\x1D\x12V[\x90P`\0a\x0Fr\x84a\x0F[\x86f\x9F2u$b\xA0\0a\x10kV[a\x0Fm\x90g\r\xC5R\x7Fd, \0a\x1D\x12V[a\x10kV[a\x0F\x84\x90g\r\xE0\xB6\xB3\xA7d\0\0a\x1D\x12V[\x90Pa\x0F\xA8g\t\xD0(\xCCo _\xFF\x19\x85a\x0F\x9E\x85\x85a\x11$V[a\x0Fm\x91\x90a\x1C\xEBV[\x92PPP`\0[`\x02\x81\x10\x15a\x10CW`\0\x86a\x0F\xC4\x84a\x13\x14V[a\x0F\xCE\x91\x90a\x1C\xEBV[\x90P`\0a\x0F\xDC\x84\x85a\x10kV[a\x0F\xE5\x90a\x1C\x14V[\x90P`\0a\x0F\xF2\x82a\x14\xF8V[\x90P`\0a\x10\0\x86\x85a\x10kV[a\x10\x12g\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x84a\x10kV[a\x10\x1C\x91\x90a\x1C\xEBV[\x90Pa\x10(\x84\x82a\x11$V[a\x102\x90\x87a\x1D\x12V[\x95P\x84`\x01\x01\x94PPPPPa\x0F\xAFV[g\r\xE0\xB6\xB3\xA7d\0\0\x88\x12a\x10`Wa\x10[\x82a\x1C\x14V[a\x0C9V[P\x96\x95PPPPPPV[`\0a\x0C\x8D\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x16\xA1V[`\xB5\x81`\x01`\x88\x1B\x81\x10a\x10\x99W`@\x91\x90\x91\x1B\x90`\x80\x1C[i\x01\0\0\0\0\0\0\0\0\0\x81\x10a\x10\xB5W` \x91\x90\x91\x1B\x90`@\x1C[e\x01\0\0\0\0\0\x81\x10a\x10\xCDW`\x10\x91\x90\x91\x1B\x90` \x1C[c\x01\0\0\0\x81\x10a\x10\xE3W`\x08\x91\x90\x91\x1B\x90`\x10\x1C[b\x01\0\0\x01\x02`\x12\x1C\x80\x82\x04\x01`\x01\x90\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x90\x81\x90\x04\x81\x11\x90\x03\x90V[`\0a\x0C\x8D\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x16\xA1V[`\0\x80\x82\x13a\x11vW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x04sV[`\0``a\x11\x83\x84a\x16\xC0V[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[`\0\x81`\0\x03a\x13-WPg\r\xE0\xB6\xB3\xA7d\0\0\x91\x90PV[gV\x98\xEE\xF0fp\0\0\x82\x12a\x13DWP`\0\x91\x90PV[a\x13UgV\x98\xEE\xF0fp\0\0a\x1C\x14V[\x82\x13a\x13jWPg\x1B\xC1mgN\xC8\0\0\x91\x90PV[`\0a\x13u\x83a\x17hV[\x90P`\0a\x13\xAEg\r\xE0\xB6\xB3\xA7d\0\0a\x13\x97\x84g\x1B\xC1mgN\xC8\0\0a\x0C\xABV[a\x13\xA9\x90g\r\xE0\xB6\xB3\xA7d\0\0a\x1D\x12V[a\x11$V[\x90P`\0\x80\x82a\x14\n\x81a\x13\xF7\x81a\x13\xE5\x81a\x13\xD2\x81g\x02_\x0F\xE1\x05\xA3\x14\0a\x10kV[a\x0Fm\x90g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19a\x1D\x12V[a\x0Fm\x90g\x14\xA8EL\x19\xE1\xAC\0a\x1D\x12V[a\x0Fm\x90g\x0F\xC1\x0E\x01W\x82w\xFF\x19a\x1D\x12V[a\x14\x1C\x90g\x03\xDE\xBD\x08;\x8C|\0a\x1D\x12V[\x91P\x83\x90Pa\x14\x84\x81a\x14r\x81a\x14`\x81a\x14N\x81a\x14;\x81\x8Ba\x10kV[a\x0Fm\x90g\x02\x95\xD4\0\xEA2W\xFF\x19a\x1D\x12V[a\x0Fm\x90g\x01W\xD8\xB2\xEC\xC7\x08\0a\x1D\x12V[a\x0Fm\x90g\x051\n\xA7\xD5!0\0a\x1D\x12V[a\x0Fm\x90g\r\xE0\xCC=\x15a\0\0a\x1D\x12V[\x91P\x81g\x11\x90\0\xAB\x10\x0F\xFC\0a\x14\x9A\x87\x88a\x10kV[a\x14\xA6\x90`\0\x19a\x1D:V[a\x14\xB0\x91\x90a\x1C\xEBV[a\x14\xBA\x91\x90a\x1D\x12V[\x92PP`\0a\x14\xC8\x83a\x14\xF8V[\x90P`\0a\x14\xD6\x85\x83a\x10kV[\x90P`\0\x88\x12a\x14\xE6W\x80a\x0C9V[a\x0C9\x81g\x1B\xC1mgN\xC8\0\0a\x1C\xEBV[`\0h\x02H\xCE6\xA7\x0C\xB2k>\x19\x82\x13a\x15\x13WP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a\x15ZW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x01a\x04sV[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05`\x01`_\x1B\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1Dm\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1Dn\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x05\x85\x14\x17\x16a\x16\xB9W`\0\x80\xFD[\x05\x92\x91PPV[`\0\x80\x82\x11a\x16\xFDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x04sV[P`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x82\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[`\0`\x01`\xFF\x1B\x82\x03a\x17\x8EW`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x12\x15a\x17\x9FWP\x19`\x01\x01\x90V[P\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x81\xFD[`\0\x80` \x83\x85\x03\x12\x15a\x18\xB2Wa\x18\xB2a\x17\xA3V[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x18\xCDWa\x18\xCDa\x17\xF3V[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x18\xE4Wa\x18\xE4a\x18CV[\x815\x81\x81\x11\x15a\x19GW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x0ENL/$\r\x8C\xAD\xCC\xEE\x8D`\xAB\x1B`d\x82\x01R`\x84\x81\xFD[\x86` \x82\x85\x01\x01\x11\x15a\x19\xAFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rjrray stride`\xA8\x1B`d\x82\x01R\x91P`\x84\x82\xFD[` \x92\x90\x92\x01\x96\x91\x95P\x90\x93PPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x19\xD7Wa\x19\xD7a\x17\xA3V[PP\x805\x92` \x90\x91\x015\x91PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x1A\x1FWa\x1A\x1Fa\x19\xE6V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x1ANWa\x1ANa\x19\xE6V[`@R\x91\x90PV[`\0` \x80\x83\x85\x03\x12\x15a\x1AlWa\x1Ala\x17\xA3V[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x1A\x87Wa\x1A\x87a\x17\xF3V[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x1A\x9EWa\x1A\x9Ea\x18CV[\x815\x81\x81\x11\x15a\x1A\xB0Wa\x1A\xB0a\x19\xE6V[a\x1A\xC2`\x1F\x82\x01`\x1F\x19\x16\x85\x01a\x1A%V[\x91P\x80\x82R\x86\x84\x82\x85\x01\x01\x11\x15a\x1B(W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x85\x90R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R`\x84\x81\xFD[\x80\x84\x84\x01\x85\x84\x017`\0\x90\x82\x01\x90\x93\x01\x92\x90\x92RP\x93\x92PPPV[`\0\x80`\0\x80\x84\x86\x03`\xC0\x81\x12\x15a\x1B^Wa\x1B^a\x17\xA3V[\x855\x94P` \x86\x015\x93P`@\x86\x015\x92P```_\x19\x82\x01\x12\x15a\x1B\xCEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FABI decoding: struct data too sh`D\x82\x01Rb\x1B\xDC\x9D`\xEA\x1B`d\x82\x01R`\x84\x81\xFD[Pa\x1B\xD7a\x19\xFCV[``\x86\x015\x81R`\x80\x86\x015` \x82\x01R`\xA0\x90\x95\x015`@\x86\x01RP\x91\x94\x90\x93P\x90\x91\x90V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01`\xFF\x1B\x82\x01a\x1C)Wa\x1C)a\x1B\xFEV[P`\0\x03\x90V[` \x80\x82R`\x11\x90\x82\x01Rp\x15\\\x19\x18]\x19H\x19[\x99\x08\x1C\x18\\\xDC\xD9Y`z\x1B`@\x82\x01R``\x01\x90V[\x81\x81\x03\x81\x81\x11\x15a\x0C\x90Wa\x0C\x90a\x1B\xFEV[`\0\x82a\x1C\x8BWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[`\0\x80`\0``\x84\x86\x03\x12\x15a\x1C\xA8Wa\x1C\xA8a\x17\xA3V[\x83Q\x92P` \x84\x01Q\x91P`@\x84\x01Q\x90P\x92P\x92P\x92V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x0C\x90Wa\x0C\x90a\x1B\xFEV[\x80\x82\x01\x80\x82\x11\x15a\x0C\x90Wa\x0C\x90a\x1B\xFEV[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a\x1D\x0BWa\x1D\x0Ba\x1B\xFEV[P\x92\x91PPV[\x80\x82\x01\x82\x81\x12`\0\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a\x1D2Wa\x1D2a\x1B\xFEV[PP\x92\x91PPV[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15a\x1DVWa\x1DVa\x1B\xFEV[\x81\x81\x05\x83\x14\x82\x15\x17a\x0C\x90Wa\x0C\x90a\x1B\xFEV";
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
        ///Calls the contract's `__slot__` (0x668f9055) function
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
                .method_hash([102, 143, 144, 85], ())
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
        ///Calls the contract's `dynamicSlot` (0xc1e0043b) function
        pub fn dynamic_slot(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, Parameters> {
            self.0
                .method_hash([193, 224, 4, 59], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getParams` (0x5e615a6b) function
        pub fn get_params(
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
                .method_hash([94, 97, 90, 107], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getReservesAndLiquidity` (0xebadef01) function
        pub fn get_reserves_and_liquidity(
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
                .method_hash([235, 173, 239, 1], ())
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
        ///Calls the contract's `setSigma` (0x9987fe64) function
        pub fn set_sigma(
            &self,
            new_target_sigma: ::ethers::core::types::U256,
            new_sigma_update_end: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [153, 135, 254, 100],
                    (new_target_sigma, new_sigma_update_end),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setStrikePrice` (0x8d52a1fc) function
        pub fn set_strike_price(
            &self,
            new_target_strike: ::ethers::core::types::U256,
            new_strike_update_end: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [141, 82, 161, 252],
                    (new_target_strike, new_strike_update_end),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setTau` (0x7f0e4c8c) function
        pub fn set_tau(
            &self,
            new_target_tau: ::ethers::core::types::U256,
            new_tau_update_end: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([127, 14, 76, 140], (new_target_tau, new_tau_update_end))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sigma` (0xafdf31cd) function
        pub fn sigma(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([175, 223, 49, 205], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `staticSlot` (0x2e2d7969) function
        pub fn static_slot(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, Parameters> {
            self.0
                .method_hash([46, 45, 121, 105], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `strikePrice` (0xc52987cf) function
        pub fn strike_price(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([197, 41, 135, 207], ())
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
        ///Calls the contract's `targetSigma` (0xc1652612) function
        pub fn target_sigma(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([193, 101, 38, 18], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `targetStrike` (0x3e1e3392) function
        pub fn target_strike(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([62, 30, 51, 146], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `tau` (0xcfc4af55) function
        pub fn tau(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([207, 196, 175, 85], ())
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
    ///Container type for all input parameters for the `__slot__` function with signature `__slot__()` and selector `0x668f9055`
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
    #[ethcall(name = "__slot__", abi = "__slot__()")]
    pub struct SlotCall;
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
    ///Container type for all input parameters for the `dynamicSlot` function with signature `dynamicSlot()` and selector `0xc1e0043b`
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
    #[ethcall(name = "dynamicSlot", abi = "dynamicSlot()")]
    pub struct DynamicSlotCall;
    ///Container type for all input parameters for the `getParams` function with signature `getParams()` and selector `0x5e615a6b`
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
    #[ethcall(name = "getParams", abi = "getParams()")]
    pub struct GetParamsCall;
    ///Container type for all input parameters for the `getReservesAndLiquidity` function with signature `getReservesAndLiquidity()` and selector `0xebadef01`
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
    #[ethcall(name = "getReservesAndLiquidity", abi = "getReservesAndLiquidity()")]
    pub struct GetReservesAndLiquidityCall;
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
    ///Container type for all input parameters for the `setSigma` function with signature `setSigma(uint256,uint256)` and selector `0x9987fe64`
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
    #[ethcall(name = "setSigma", abi = "setSigma(uint256,uint256)")]
    pub struct SetSigmaCall {
        pub new_target_sigma: ::ethers::core::types::U256,
        pub new_sigma_update_end: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setStrikePrice` function with signature `setStrikePrice(uint256,uint256)` and selector `0x8d52a1fc`
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
    #[ethcall(name = "setStrikePrice", abi = "setStrikePrice(uint256,uint256)")]
    pub struct SetStrikePriceCall {
        pub new_target_strike: ::ethers::core::types::U256,
        pub new_strike_update_end: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setTau` function with signature `setTau(uint256,uint256)` and selector `0x7f0e4c8c`
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
    #[ethcall(name = "setTau", abi = "setTau(uint256,uint256)")]
    pub struct SetTauCall {
        pub new_target_tau: ::ethers::core::types::U256,
        pub new_tau_update_end: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `sigma` function with signature `sigma()` and selector `0xafdf31cd`
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
    #[ethcall(name = "sigma", abi = "sigma()")]
    pub struct SigmaCall;
    ///Container type for all input parameters for the `staticSlot` function with signature `staticSlot()` and selector `0x2e2d7969`
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
    #[ethcall(name = "staticSlot", abi = "staticSlot()")]
    pub struct StaticSlotCall;
    ///Container type for all input parameters for the `strikePrice` function with signature `strikePrice()` and selector `0xc52987cf`
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
    #[ethcall(name = "strikePrice", abi = "strikePrice()")]
    pub struct StrikePriceCall;
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
    ///Container type for all input parameters for the `targetSigma` function with signature `targetSigma()` and selector `0xc1652612`
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
    #[ethcall(name = "targetSigma", abi = "targetSigma()")]
    pub struct TargetSigmaCall;
    ///Container type for all input parameters for the `targetStrike` function with signature `targetStrike()` and selector `0x3e1e3392`
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
    #[ethcall(name = "targetStrike", abi = "targetStrike()")]
    pub struct TargetStrikeCall;
    ///Container type for all input parameters for the `tau` function with signature `tau()` and selector `0xcfc4af55`
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
    #[ethcall(name = "tau", abi = "tau()")]
    pub struct TauCall;
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
        Slot(SlotCall),
        ComputeSwapConstant(ComputeSwapConstantCall),
        Core(CoreCall),
        DynamicSlot(DynamicSlotCall),
        GetParams(GetParamsCall),
        GetReservesAndLiquidity(GetReservesAndLiquidityCall),
        Init(InitCall),
        SetSigma(SetSigmaCall),
        SetStrikePrice(SetStrikePriceCall),
        SetTau(SetTauCall),
        Sigma(SigmaCall),
        StaticSlot(StaticSlotCall),
        StrikePrice(StrikePriceCall),
        SwapFeePercentageWad(SwapFeePercentageWadCall),
        TargetSigma(TargetSigmaCall),
        TargetStrike(TargetStrikeCall),
        Tau(TauCall),
        Validate(ValidateCall),
    }
    impl ::ethers::core::abi::AbiDecode for LogNormalCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <SlotCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Slot(decoded));
            }
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
            if let Ok(decoded) = <SwapFeePercentageWadCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SwapFeePercentageWad(decoded));
            }
            if let Ok(decoded) = <TargetSigmaCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TargetSigma(decoded));
            }
            if let Ok(decoded) = <TargetStrikeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TargetStrike(decoded));
            }
            if let Ok(decoded) = <TauCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Tau(decoded));
            }
            if let Ok(decoded) = <ValidateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Validate(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for LogNormalCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Slot(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ComputeSwapConstant(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Core(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DynamicSlot(element) => {
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
                Self::StaticSlot(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StrikePrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SwapFeePercentageWad(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TargetSigma(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TargetStrike(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Tau(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Validate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for LogNormalCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Slot(element) => ::core::fmt::Display::fmt(element, f),
                Self::ComputeSwapConstant(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Core(element) => ::core::fmt::Display::fmt(element, f),
                Self::DynamicSlot(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetParams(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetReservesAndLiquidity(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Init(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetSigma(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetStrikePrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetTau(element) => ::core::fmt::Display::fmt(element, f),
                Self::Sigma(element) => ::core::fmt::Display::fmt(element, f),
                Self::StaticSlot(element) => ::core::fmt::Display::fmt(element, f),
                Self::StrikePrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::SwapFeePercentageWad(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TargetSigma(element) => ::core::fmt::Display::fmt(element, f),
                Self::TargetStrike(element) => ::core::fmt::Display::fmt(element, f),
                Self::Tau(element) => ::core::fmt::Display::fmt(element, f),
                Self::Validate(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<SlotCall> for LogNormalCalls {
        fn from(value: SlotCall) -> Self {
            Self::Slot(value)
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
    impl ::core::convert::From<SwapFeePercentageWadCall> for LogNormalCalls {
        fn from(value: SwapFeePercentageWadCall) -> Self {
            Self::SwapFeePercentageWad(value)
        }
    }
    impl ::core::convert::From<TargetSigmaCall> for LogNormalCalls {
        fn from(value: TargetSigmaCall) -> Self {
            Self::TargetSigma(value)
        }
    }
    impl ::core::convert::From<TargetStrikeCall> for LogNormalCalls {
        fn from(value: TargetStrikeCall) -> Self {
            Self::TargetStrike(value)
        }
    }
    impl ::core::convert::From<TauCall> for LogNormalCalls {
        fn from(value: TauCall) -> Self {
            Self::Tau(value)
        }
    }
    impl ::core::convert::From<ValidateCall> for LogNormalCalls {
        fn from(value: ValidateCall) -> Self {
            Self::Validate(value)
        }
    }
    ///Container type for all return fields from the `__slot__` function with signature `__slot__()` and selector `0x668f9055`
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
    ///Container type for all return fields from the `dynamicSlot` function with signature `dynamicSlot()` and selector `0xc1e0043b`
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
        pub params: Parameters,
    }
    ///Container type for all return fields from the `getParams` function with signature `getParams()` and selector `0x5e615a6b`
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
    ///Container type for all return fields from the `getReservesAndLiquidity` function with signature `getReservesAndLiquidity()` and selector `0xebadef01`
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
    ///Container type for all return fields from the `sigma` function with signature `sigma()` and selector `0xafdf31cd`
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
    ///Container type for all return fields from the `staticSlot` function with signature `staticSlot()` and selector `0x2e2d7969`
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
    pub struct StaticSlotReturn(pub Parameters);
    ///Container type for all return fields from the `strikePrice` function with signature `strikePrice()` and selector `0xc52987cf`
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
    ///Container type for all return fields from the `targetSigma` function with signature `targetSigma()` and selector `0xc1652612`
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
    pub struct TargetSigmaReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `targetStrike` function with signature `targetStrike()` and selector `0x3e1e3392`
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
    pub struct TargetStrikeReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `tau` function with signature `tau()` and selector `0xcfc4af55`
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
}
