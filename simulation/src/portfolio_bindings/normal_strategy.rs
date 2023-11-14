pub use normal_strategy::*;
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
pub mod normal_strategy {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("portfolio_"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("afterCreate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("afterCreate"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("strategyArgs"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("success"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("approximateReservesGivenPrice"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "approximateReservesGivenPrice",
                            ),
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
                                    name: ::std::borrow::ToOwned::to_owned("strategyArgs"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("beforeSwap"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("beforeSwap"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sellAsset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("swapper"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("configs"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("configs"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("strikePriceWad"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint128"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "volatilityBasisPoints",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("durationSeconds"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("creationTimestamp"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("isPerpetual"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getAmountOut"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getAmountOut"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sellAsset"),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("swapper"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("output"),
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
                    ::std::borrow::ToOwned::to_owned("getInvariant"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getInvariant"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("invariant"),
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
                    ::std::borrow::ToOwned::to_owned("getMaxOrder"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getMaxOrder"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sellAsset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("swapper"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct Order"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getSpotPrice"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getSpotPrice"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
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
                    ::std::borrow::ToOwned::to_owned("getStrategyData"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getStrategyData"),
                            inputs: ::std::vec![
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
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "volatilityBasisPoints",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("durationSeconds"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("isPerpetual"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
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
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("strategyData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("initialX"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("initialY"),
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
                    ::std::borrow::ToOwned::to_owned("portfolio"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("portfolio"),
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
                    ::std::borrow::ToOwned::to_owned("simulateSwap"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("simulateSwap"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("order"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct Order"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("timestamp"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("swapper"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("success"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("prevInvariant"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("postInvariant"),
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
                    ::std::borrow::ToOwned::to_owned("validatePool"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("validatePool"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
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
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("AfterCreate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("AfterCreate"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("portfolio"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("strikePriceWad"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "volatilityBasisPoints",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("durationSeconds"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("isPerpetual"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Genesis"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Genesis"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("portfolio"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
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
                    ::std::borrow::ToOwned::to_owned("NormalStrategyLib_ConfigExists"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "NormalStrategyLib_ConfigExists",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "NormalStrategyLib_InvalidDuration",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "NormalStrategyLib_InvalidDuration",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "NormalStrategyLib_InvalidStrategyArgs",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "NormalStrategyLib_InvalidStrategyArgs",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "NormalStrategyLib_InvalidStrikePrice",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "NormalStrategyLib_InvalidStrikePrice",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "NormalStrategyLib_InvalidVolatility",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "NormalStrategyLib_InvalidVolatility",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "NormalStrategyLib_NonExpiringPool",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "NormalStrategyLib_NonExpiringPool",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NormalStrategy_NotPortfolio"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "NormalStrategy_NotPortfolio",
                            ),
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
                (
                    ::std::borrow::ToOwned::to_owned("SwapLib_OutputExceedsReserves"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "SwapLib_OutputExceedsReserves",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SwapLib_ProtocolFeeTooHigh"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "SwapLib_ProtocolFeeTooHigh",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SwapLib_ZeroXAdjustment"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "SwapLib_ZeroXAdjustment",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SwapLib_ZeroYAdjustment"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "SwapLib_ZeroYAdjustment",
                            ),
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
    pub static NORMALSTRATEGY_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA0`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\0>48\x03\x80b\0>4\x839\x81\x01`@\x81\x90Rb\0\x004\x91b\0\0sV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\x80\x81\x90R`@Q\x7F;5\xCD\xE5\x85\"y\xBF\xD1\xCE\x80S\x8F\x94\xC2\xE4*S\x0F\x11\x1FVB\x9B\x91,\x85\xCF\xA6P\xD4\xD3\x90`\0\x90\xA2Pb\0\0\xA5V[`\0` \x82\x84\x03\x12\x15b\0\0\x86W`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\0\x9EW`\0\x80\xFD[\x93\x92PPPV[`\x80Qa=\x0Bb\0\x01)`\09`\0\x81\x81`\xD9\x01R\x81\x81a\x03\x7F\x01R\x81\x81a\x03\xF9\x01R\x81\x81a\x05\x7F\x01R\x81\x81a\x06i\x01R\x81\x81a\x08\xBF\x01R\x81\x81a\t@\x01R\x81\x81a\x0BI\x01R\x81\x81a\x0C\x13\x01R\x81\x81a\rV\x01R\x81\x81a\x0E\xD1\x01R\x81\x81a\x0F[\x01R\x81\x81a\x10\x98\x01R\x81\x81a\x10\xF7\x01R\x81\x81a\x12q\x01Ra\x12\xF3\x01Ra=\x0B`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xCFW`\x005`\xE0\x1C\x80c\x80\xAF\x9Dv\x11a\0\x8CW\x80c\xE31\xBA4\x11a\0fW\x80c\xE31\xBA4\x14a\x02\xABW\x80c\xE6\x04{\x19\x14a\x02\xBEW\x80c\xECshT\x14a\x02\xD2W\x80c\xF0{\x87\x9E\x14a\x02\xE5W`\0\x80\xFD[\x80c\x80\xAF\x9Dv\x14a\x02.W\x80c\xA4G\x89\x19\x14a\x02^W\x80c\xE0hx\x7F\x14a\x02\x88W`\0\x80\xFD[\x80c\x16\xED\xE0\x16\x14a\0\xD4W\x80c\x19\x05x\x07\x14a\x01\x18W\x80c4\xDB\xC7;\x14a\x019W\x80c9CMZ\x14a\x01\xD1W\x80cE-/\x18\x14a\x01\xE4W\x80cK\xF3F\xBF\x14a\x02\x06W[`\0\x80\xFD[a\0\xFB\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01+a\x01&6`\x04a4 V[a\x03TV[`@Q\x90\x81R` \x01a\x01\x0FV[a\x01\x91a\x01G6`\x04a4qV[`\0` \x81\x90R\x90\x81R`@\x90 T`\x01`\x01`\x80\x1B\x03\x81\x16\x90c\xFF\xFF\xFF\xFF`\x01`\x80\x1B\x82\x04\x81\x16\x91`\x01`\xA0\x1B\x81\x04\x82\x16\x91`\x01`\xC0\x1B\x82\x04\x16\x90`\xFF`\x01`\xE0\x1B\x90\x91\x04\x16\x85V[`@\x80Q`\x01`\x01`\x80\x1B\x03\x90\x96\x16\x86Rc\xFF\xFF\xFF\xFF\x94\x85\x16` \x87\x01R\x92\x84\x16\x92\x85\x01\x92\x90\x92R\x91\x90\x91\x16``\x83\x01R\x15\x15`\x80\x82\x01R`\xA0\x01a\x01\x0FV[a\x01+a\x01\xDF6`\x04a4qV[a\x06>V[a\x01\xF7a\x01\xF26`\x04a4\x8CV[a\x07bV[`@Qa\x01\x0F\x93\x92\x91\x90a4\xD5V[a\x02\x19a\x02\x146`\x04a5\x9FV[a\x08VV[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x01\x0FV[a\x02Aa\x02<6`\x04a6RV[a\x08\x8AV[`@\x80Q\x93\x15\x15\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x01\x0FV[a\x02qa\x02l6`\x04a6\xF8V[a\x0B\xE6V[`@\x80Q\x92\x15\x15\x83R` \x83\x01\x91\x90\x91R\x01a\x01\x0FV[a\x02\x9Ba\x02\x966`\x04a71V[a\rIV[`@Q\x90\x15\x15\x81R` \x01a\x01\x0FV[a\x01+a\x02\xB96`\x04a4qV[a\x0F0V[a\x02\x9Ba\x02\xCC6`\x04a4qV[P`\x01\x90V[a\x02qa\x02\xE06`\x04a7\xB3V[a\x10\x8AV[a\x02\xF8a\x02\xF36`\x04a7\xB3V[a\x12%V[`@Qa\x01\x0F\x91\x90`\0`\xA0\x82\x01\x90P`\x01`\x01`\x80\x1B\x03\x80\x84Q\x16\x83R\x80` \x85\x01Q\x16` \x84\x01RP`@\x83\x01Q\x15\x15`@\x83\x01R`\x01`\x01`@\x1B\x03``\x84\x01Q\x16``\x83\x01R`\x80\x83\x01Q\x15\x15`\x80\x83\x01R\x92\x91PPV[`@Qc\"i|!`\xE2\x1B\x81R`\x01`\x01`@\x1B\x03\x85\x16`\x04\x82\x01R`\0\x90\x81\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x89\xA5\xF0\x84\x90`$\x01a\x01\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\xC7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xEB\x91\x90a8-V[\x90P`\0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16c^Gf<` \x89\x90\x1Cc\xFF\xFF\xFF\xFF\x16`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x81Rb\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01R`$\x01`\x80`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04iW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\x8D\x91\x90a9\x07V[\x90Pa\x04\xB1\x86a\x04\xA1W\x81``\x01Qa\x04\xA7V[\x81` \x01Q[\x86\x90`\xFF\x16a\x16PV[`\x01`\x01`@\x1B\x03\x88\x16`\0\x90\x81R` \x81\x81R`@\x91\x82\x90 \x82Q`\xA0\x80\x82\x01\x85R\x91T`\x01`\x01`\x80\x1B\x03\x81\x16\x82R`\x01`\x80\x1B\x81\x04c\xFF\xFF\xFF\xFF\x90\x81\x16\x94\x83\x01\x94\x90\x94R`\x01`\xA0\x1B\x81\x04\x84\x16\x82\x86\x01R`\x01`\xC0\x1B\x81\x04\x90\x93\x16``\x82\x01R`\x01`\xE0\x1B\x90\x92\x04`\xFF\x16\x15\x15`\x80\x83\x01R\x82Q\x90\x81\x01\x90\x92R\x91\x96Pa\x06\n\x91\x90\x80a\x05@\x89a\x16gV[`\x01`\x01`\x80\x1B\x03\x16\x81R` \x01`\x01`\x01`\x01`\x80\x1B\x03\x16\x81R` \x01`\0\x15\x15\x81R` \x01\x8A`\x01`\x01`@\x1B\x03\x16\x81R` \x01\x89\x15\x15\x81RPB\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xB0\xE2\x1E\x8A`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\xDBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xFF\x91\x90a9\x87V[\x86\x93\x92\x91\x90\x89a\x16}V[\x92P`\0\x86a\x06\x1DW\x81` \x01Qa\x06#V[\x81``\x01Q[`\xFF\x16\x90Pa\x062\x84\x82a\x186V[\x98\x97PPPPPPPPV[`@Qc\"i|!`\xE2\x1B\x81R`\x01`\x01`@\x1B\x03\x82\x16`\x04\x82\x01R`\0\x90\x81\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x89\xA5\xF0\x84\x90`$\x01a\x01\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\xB1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xD5\x91\x90a8-V[`\x01`\x01`@\x1B\x03\x84\x16`\0\x90\x81R` \x81\x81R`@\x91\x82\x90 \x82Q`\xA0\x81\x01\x84R\x90T`\x01`\x01`\x80\x1B\x03\x81\x16\x82R`\x01`\x80\x1B\x81\x04c\xFF\xFF\xFF\xFF\x90\x81\x16\x93\x83\x01\x93\x90\x93R`\x01`\xA0\x1B\x81\x04\x83\x16\x93\x82\x01\x93\x90\x93R`\x01`\xC0\x1B\x83\x04\x82\x16``\x82\x01R`\x01`\xE0\x1B\x90\x92\x04`\xFF\x16\x15\x15`\x80\x83\x01R\x91\x92Pa\x07[\x91\x83\x91\x90a\x18L\x16V[\x93\x92PPPV[```\0\x80`\0`@Q\x80`\xA0\x01`@R\x80a\x07}\x8Ba\x16gV[`\x01`\x01`\x80\x1B\x03\x16\x81R` \x01a\x07\x94\x8Aa\x19-V[c\xFF\xFF\xFF\xFF\x16\x81R` \x01a\x07\xA8\x89a\x19-V[c\xFF\xFF\xFF\xFF\x16\x81R`\0` \x82\x01R\x87\x15\x15`@\x90\x91\x01R\x90Pa\x08/\x81`@\x80Q\x82Q`\x01`\x01`\x80\x1B\x03\x16` \x80\x83\x01\x91\x90\x91R\x83\x01Qc\xFF\xFF\xFF\xFF\x90\x81\x16\x82\x84\x01R\x91\x83\x01Q\x82\x16``\x82\x81\x01\x91\x90\x91R\x83\x81\x01Q\x90\x92\x16`\x80\x80\x83\x01\x91\x90\x91R\x83\x01Q\x15\x15`\xA0\x82\x01R`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x91\x90PV[\x93Pa\x08D\x85a\x08>\x83a\x19@V[\x90a\x19\xE6V[\x94\x9A\x90\x99P\x93\x97P\x92\x95PPPPPPV[`\0\x80`\0a\x08d\x84a\x1A\nV[\x90P`\0a\x08q\x82a\x19@V[\x90Pa\x08}\x81\x87a\x19\xE6V[\x93P\x93PPP\x92P\x92\x90PV[``\x83\x01Q`@Qc\"i|!`\xE2\x1B\x81R`\x01`\x01`@\x1B\x03\x90\x91\x16`\x04\x82\x01R`\0\x90\x81\x90\x81\x90\x81\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x89\xA5\xF0\x84\x90`$\x01a\x01\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\x07W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t+\x91\x90a8-V[``\x88\x01Q\x90\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c^Gf<\x90` \x1Cc\xFF\xFF\xFF\xFF\x16`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x81Rb\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01R`$\x01`\x80`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\xB0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\xD4\x91\x90a9\x07V[\x90Pa\t\xDEa3xV[\x88`\x80\x01Q\x15a\n&W` \x82\x01Q`\xFF\x90\x81\x16a\x01\0\x83\x01R``\x83\x01Q\x16a\x01 \x82\x01R\x81Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\xC0\x83\x01R`@\x83\x01Q\x16`\xE0\x82\x01Ra\n`V[``\x82\x01Q`\xFF\x90\x81\x16a\x01\0\x83\x01R` \x83\x01Q\x16a\x01 \x82\x01R`@\x82\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\xC0\x83\x01R\x82Q\x16`\xE0\x82\x01R[\x88Q`\x01`\x01`\x80\x1B\x03\x90\x81\x16`\x80\x83\x01R` \x8A\x01Q\x16`\xA0\x82\x01Ra\n\x86\x81a\x1AkV[\x90Pa\n\x95\x81`\x80\x01Qa\x16gV[`\x01`\x01`\x80\x1B\x03\x16\x89R`\xA0\x81\x01Qa\n\xAE\x90a\x16gV[`\x01`\x01`\x80\x1B\x03\x90\x81\x16` \x8B\x81\x01\x91\x90\x91R``\x80\x8C\x01Q`\x01`\x01`@\x1B\x03\x16`\0\x90\x81R\x80\x83R`@\x90\x81\x90 \x81Q`\xA0\x81\x01\x83R\x90T\x94\x85\x16\x81R`\x01`\x80\x1B\x85\x04c\xFF\xFF\xFF\xFF\x90\x81\x16\x82\x86\x01R`\x01`\xA0\x1B\x86\x04\x81\x16\x82\x84\x01R`\x01`\xC0\x1B\x86\x04\x16\x92\x81\x01\x92\x90\x92R`\x01`\xE0\x1B\x90\x93\x04`\xFF\x16\x15\x15`\x80\x82\x01R\x82QcXq\x0FE`\xE1\x1B\x81R\x92Qa\x0B\xC7\x93\x91\x92\x8D\x92\x8D\x92\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x92c\xB0\xE2\x1E\x8A\x92`\x04\x80\x82\x01\x93\x91\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x0B\x98W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\xBC\x91\x90a9\x87V[\x87\x93\x92\x91\x90\x8Ca\x1A\xE7V[\x90\x96P\x94Pa\x0B\xD8\x90P\x85\x85a\x1C?V[\x95PPPP\x93P\x93P\x93\x90PV[`@Qc\"i|!`\xE2\x1B\x81R`\x01`\x01`@\x1B\x03\x85\x16`\x04\x82\x01R`\0\x90\x81\x90\x81\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x89\xA5\xF0\x84\x90`$\x01a\x01\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C[W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\x7F\x91\x90a8-V[\x90Pa\x0C\x8A\x85a\x16gV[`\x01`\x01`\x80\x1B\x03\x16\x81Ra\x0C\x9E\x84a\x16gV[`\x01`\x01`\x80\x1B\x03\x90\x81\x16` \x83\x81\x01\x91\x90\x91R`\x01`\x01`@\x1B\x03\x89\x16`\0\x90\x81R\x80\x82R`@\x80\x82 \x81Q`\xA0\x81\x01\x83R\x90T\x94\x85\x16\x81R`\x01`\x80\x1B\x85\x04c\xFF\xFF\xFF\xFF\x90\x81\x16\x94\x82\x01\x94\x90\x94R`\x01`\xA0\x1B\x85\x04\x84\x16\x91\x81\x01\x91\x90\x91R`\x01`\xC0\x1B\x84\x04\x83\x16``\x82\x01R`\x01`\xE0\x1B\x90\x93\x04`\xFF\x16\x15\x15`\x80\x84\x01R\x91a\r,\x91\x84\x91\x90a\x18L\x16V[\x90P`\0a\r:\x88\x83a\x1C?V[\x99\x91\x98P\x90\x96PPPPPPPV[`\x003`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\r\x94W`@Qc:#%k`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\r\xD5\x84\x84\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x1A\n\x92PPPV[\x90Pa\x0E8\x81`\0\x01Q`\x01`\x01`\x80\x1B\x03\x16\x82` \x01Qc\xFF\xFF\xFF\xFF\x16\x83`@\x01Qc\xFF\xFF\xFF\xFF\x16\x84`\x80\x01Q`\0\x80\x8B`\x01`\x01`@\x1B\x03\x16`\x01`\x01`@\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 a\x1Ck\x90\x94\x93\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[P`\x01`\x01`@\x1B\x03\x84\x16`\0\x81\x81R` \x81\x81R`@\x91\x82\x90 \x82Q`\xA0\x81\x01\x84R\x90T`\x01`\x01`\x80\x1B\x03\x81\x16\x80\x83R`\x01`\x80\x1B\x82\x04c\xFF\xFF\xFF\xFF\x90\x81\x16\x84\x86\x01\x81\x90R`\x01`\xA0\x1B\x84\x04\x82\x16\x85\x88\x01\x81\x90R`\x01`\xC0\x1B\x85\x04\x90\x92\x16``\x80\x87\x01\x91\x90\x91R`\x01`\xE0\x1B\x90\x94\x04`\xFF\x16\x15\x15`\x80\x80\x87\x01\x82\x90R\x88Q\x94\x85R\x96\x84\x01\x91\x90\x91R\x95\x82\x01R\x90\x81\x01\x93\x90\x93R\x92\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x91\x7F\x8Di\xFAt\x9A\xF4\x15\xF9<\x96\x86>\xBF6\x92\x93&#\xC0\x10\xEB\xA8\x11\x8B\x02\xF0DXZ\xABe\x13\x91\x01`@Q\x80\x91\x03\x90\xA3P`\x01\x94\x93PPPPV[`@Qc\"i|!`\xE2\x1B\x81R`\x01`\x01`@\x1B\x03\x82\x16`\x04\x82\x01R`\0\x90\x81\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x89\xA5\xF0\x84\x90`$\x01a\x01\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\xA3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xC7\x91\x90a8-V[`\x01`\x01`@\x1B\x03\x84\x16`\0\x90\x81R` \x81\x81R`@\x80\x83 \x81Q`\xA0\x81\x01\x83R\x90T`\x01`\x01`\x80\x1B\x03\x81\x16\x82Rc\xFF\xFF\xFF\xFF`\x01`\x80\x1B\x82\x04\x81\x16\x94\x83\x01\x94\x90\x94R`\x01`\xA0\x1B\x81\x04\x84\x16\x92\x82\x01\x92\x90\x92R`\x01`\xC0\x1B\x82\x04\x90\x92\x16``\x83\x01R`\xFF`\x01`\xE0\x1B\x90\x91\x04\x16\x15\x15`\x80\x82\x01R\x91\x92Pa\x10H\x82a\x19@V[\x90Pa\x10T\x82Ba\x1E6V[`\x80\x82\x01R`@\x83\x01Q\x83Qa\x10\x81\x91a\x10z\x91`\x01`\x01`\x80\x1B\x03\x90\x81\x16\x91\x16a\x1ErV[\x82\x90a\x1E\x87V[\x95\x94PPPPPV[`\0\x803`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x10\xD6W`@Qc:#%k`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qc\"i|!`\xE2\x1B\x81R`\x01`\x01`@\x1B\x03\x86\x16`\x04\x82\x01R`\0\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\x89\xA5\xF0\x84\x90`$\x01a\x01\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11GW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11k\x91\x90a8-V[`\x01`\x01`@\x1B\x03\x87\x16`\0\x90\x81R` \x81\x81R`@\x80\x83 \x81Q`\xA0\x81\x01\x83R\x90T`\x01`\x01`\x80\x1B\x03\x81\x16\x82R`\x01`\x80\x1B\x81\x04c\xFF\xFF\xFF\xFF\x90\x81\x16\x94\x83\x01\x94\x90\x94R`\x01`\xA0\x1B\x81\x04\x84\x16\x92\x82\x01\x92\x90\x92R`\x01`\xC0\x1B\x82\x04\x83\x16``\x82\x01R`\x01`\xE0\x1B\x90\x91\x04`\xFF\x16\x15\x15`\x80\x82\x01R\x92\x93Pa\x11\xF5\x90\x84\x90\x84\x90\x8A\x90B\x90a\x1F\x94\x16V[\x90Pa\x12\x01\x83\x83a \xA3V[\x15a\x12\x14W`\0\x94P\x92Pa\x12\x1D\x91PPV[`\x01\x94P\x92PPP[\x93P\x93\x91PPV[`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x91\x90\x91R`@Qc\"i|!`\xE2\x1B\x81R`\x01`\x01`@\x1B\x03\x85\x16`\x04\x82\x01R`\0\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\x89\xA5\xF0\x84\x90`$\x01a\x01\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12\xC1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\xE5\x91\x90a8-V[\x90P`\0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16c^Gf<` \x88\x90\x1Cc\xFF\xFF\xFF\xFF\x16`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x81Rb\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01R`$\x01`\x80`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13cW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\x87\x91\x90a9\x07V[`\x01`\x01`@\x1B\x03\x87\x16`\0\x90\x81R` \x81\x81R`@\x80\x83 \x81Q`\xA0\x81\x01\x83R\x90T`\x01`\x01`\x80\x1B\x03\x81\x16\x82Rc\xFF\xFF\xFF\xFF`\x01`\x80\x1B\x82\x04\x81\x16\x94\x83\x01\x94\x90\x94R`\x01`\xA0\x1B\x81\x04\x84\x16\x92\x82\x01\x92\x90\x92R`\x01`\xC0\x1B\x82\x04\x90\x92\x16``\x83\x01R`\xFF`\x01`\xE0\x1B\x90\x91\x04\x16\x15\x15`\x80\x82\x01R\x91\x92Pa\x14\x08\x82a\x19@V[\x90P`\0\x80g\r\xE0\xB6\xB3\xA7d\0\0\x91P`\0\x80a\x14)\x85`@\x01Q\x90`\0\x90V[\x91P\x91P`\0`@Q\x80`\xA0\x01`@R\x80`\0`\x01`\x01`\x80\x1B\x03\x16\x81R` \x01`\0`\x01`\x01`\x80\x1B\x03\x16\x81R` \x01`\0\x15\x15\x81R` \x01\x8E`\x01`\x01`@\x1B\x03\x16\x81R` \x01\x8D\x15\x15\x81RP\x90P`\0\x80\x8D\x15a\x15fW`\x01\x8B`\0\x01Q`\x01`\x01`\x80\x1B\x03\x16a\x14\xB3\x8D`@\x01Q`\x01`\x01`\x80\x1B\x03\x16\x8Aa \xDB\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x14\xBD\x91\x90a9\xB6V[a\x14\xC7\x91\x90a9\xB6V[\x91P`\x01a\x14\xEB\x8C`@\x01Q`\x01`\x01`\x80\x1B\x03\x16\x86a \xDB\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x8C` \x01Q`\x01`\x01`\x80\x1B\x03\x16a\x15\x03\x91\x90a9\xB6V[a\x15\r\x91\x90a9\xB6V[\x90Pa\x151a\x15,\x8B` \x01Q`\xFF\x16\x84a\x186\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x16gV[`\x01`\x01`\x80\x1B\x03\x16\x83R``\x8A\x01Qa\x15S\x90a\x15,\x90\x83\x90`\xFF\x16a\x186V[`\x01`\x01`\x80\x1B\x03\x16` \x84\x01Ra\x16=V[`\x01\x8B` \x01Q`\x01`\x01`\x80\x1B\x03\x16a\x15\x96\x8D`@\x01Q`\x01`\x01`\x80\x1B\x03\x16\x88a \xDB\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x15\xA0\x91\x90a9\xB6V[a\x15\xAA\x91\x90a9\xB6V[\x91P`\x01a\x15\xCE\x8C`@\x01Q`\x01`\x01`\x80\x1B\x03\x16\x88a \xDB\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x8CQa\x15\xE3\x91\x90`\x01`\x01`\x80\x1B\x03\x16a9\xB6V[a\x15\xED\x91\x90a9\xB6V[\x90Pa\x16\x0Ca\x15,\x8B``\x01Q`\xFF\x16\x84a\x186\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[`\x01`\x01`\x80\x1B\x03\x16\x83R` \x8A\x01Qa\x16.\x90a\x15,\x90\x83\x90`\xFF\x16a\x186V[`\x01`\x01`\x80\x1B\x03\x16` \x84\x01R[P\x90\x9D\x9CPPPPPPPPPPPPPV[`\0\x80a\x16\\\x83a \xF0V[\x93\x90\x93\x02\x93\x92PPPV[`\0`\x01`\x80\x1B\x82\x10a\x16yW`\0\x80\xFD[P\x90V[`\0\x80`\0\x80a\x16\x91\x8A\x8A\x8A\x8A\x8A\x8Aa\x1A\xE7V[\x92P\x92P\x92P`\0a\x16\xA2\x8Aa\x19@V[`\xA0\x81\x01\x84\x90R\x90Pa\x16\xB5\x8A\x89a\x1E6V[`\x80\x80\x83\x01\x91\x90\x91R\x89\x01Q`\0\x90\x80\x15a\x16\xE4W\x85\x83Ra\x16\xD6\x83a!\x08V[` \x84\x01\x81\x90R\x91Pa\x16\xFAV[` \x83\x01\x86\x90Ra\x16\xF4\x83a!\xC9V[\x80\x84R\x91P[\x81`\0\x03a\x17*W\x80a\x17\x0EW\x8CQa\x17\x14V[\x8C` \x01Q[`\x01`\x01`\x80\x1B\x03\x16\x96PPPPPPPa\x18,V[`\0a\x179\x83`2`da\"\x8AV[\x90P`\0a\x17J\x84`\x96`da\"\xA9V[\x90Pa\x17\xCA\x85`@Q` \x01a\x17\x9F\x91\x90`\0`\xC0\x82\x01\x90P\x82Q\x82R` \x83\x01Q` \x83\x01R`@\x83\x01Q`@\x83\x01R``\x83\x01Q``\x83\x01R`\x80\x83\x01Q`\x80\x83\x01R`\xA0\x83\x01Q`\xA0\x83\x01R\x92\x91PPV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83\x83`\x01a\x01\0\x88a\x17\xC2Wa\"\xD7a#RV[a# a#RV[\x93P\x83a\x17\xD6\x81a9\xC9V[\x94PPa\x17\xF9\x8F`@\x01Q`\x01`\x01`\x80\x1B\x03\x16\x85a \xDB\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x93P\x83\x83a\x18\x08W\x8FQa\x18\x0EV[\x8F` \x01Q[`\x01`\x01`\x80\x1B\x03\x16a\x18!\x91\x90a9\xB6V[\x98PPPPPPPPP[\x96\x95PPPPPPV[`\0\x80a\x18B\x83a \xF0V[\x90\x93\x04\x93\x92PPPV[`\0\x80a\x18|\x84`@\x01Q`\x01`\x01`\x80\x1B\x03\x16\x85`\0\x01Q`\x01`\x01`\x80\x1B\x03\x16a\x1Er\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x18\xAD\x85`@\x01Q`\x01`\x01`\x80\x1B\x03\x16\x86` \x01Q`\x01`\x01`\x80\x1B\x03\x16a\x1Er\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0`@Q\x80`\xC0\x01`@R\x80\x84\x81R` \x01\x83\x81R` \x01\x86`\0\x01Q`\x01`\x01`\x80\x1B\x03\x16\x81R` \x01a\x18\xFD\x87` \x01Qc\xFF\xFF\xFF\xFF\x16a'\x10g\r\xE0\xB6\xB3\xA7d\0\0\x91\x90\x91\x02\x04\x90V[\x81R` \x01a\x19\x0C\x88\x88a$hV[\x81R` \x01`\0\x81RP\x90Pa\x19!\x81a$~V[\x93PPPP[\x92\x91PPV[`\0d\x01\0\0\0\0\x82\x10a\x16yW`\0\x80\xFD[a\x19y`@Q\x80`\xC0\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`@Q\x80`\xC0\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01\x83`\0\x01Q`\x01`\x01`\x80\x1B\x03\x16\x81R` \x01a\x19\xC7\x84` \x01Qc\xFF\xFF\xFF\xFF\x16a'\x10g\r\xE0\xB6\xB3\xA7d\0\0\x91\x90\x91\x02\x04\x90V[\x81R`@\x93\x84\x01Qc\xFF\xFF\xFF\xFF\x16` \x82\x01R`\0\x93\x01\x92\x90\x92RP\x90V[`\0\x80a\x19\xF3\x84\x84a%xV[\x80\x85R\x91Pa\x1A\x01\x84a!\x08V[\x90P\x92P\x92\x90PV[`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x91\x90\x91R`\xA0\x82Q\x14a\x1AWW`@Qc\x01YW\xD3`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x80` \x01\x90Q\x81\x01\x90a\x19'\x91\x90a9\xE2V[a\x1Asa3xV[a\x01\0\x82\x01Q`@\x83\x01Qa\x1A\x8A\x91`\xFF\x16a\x16PV[`@\x83\x01Ra\x01\0\x82\x01Q``\x83\x01Qa\x1A\xA6\x91`\xFF\x16a\x16PV[``\x83\x01Ra\x01\0\x82\x01Q`\x80\x83\x01Qa\x1A\xC2\x91`\xFF\x16a\x16PV[`\x80\x83\x01Ra\x01 \x82\x01Q`\xA0\x83\x01Qa\x1A\xDE\x91`\xFF\x16a\x16PV[`\xA0\x83\x01RP\x90V[`\0\x80`\0a\x1A\xFC\x89\x89\x89`\x80\x01Q\x89a\x1F\x94V[\x89Q` \x8B\x01Q`\xC0\x8C\x01Q\x92\x94P`\x01`\x01`\x80\x1B\x03\x91\x82\x16\x92\x91\x16\x90`\0\x90`\x01`\x01`\xA0\x1B\x03\x88\x81\x16\x91\x16\x14a\x1B9W\x8B`\x80\x01Qa\x1B?V[\x8B`\xA0\x01Q[a\xFF\xFF\x16\x90Pa\x1BR\x8A\x84\x84\x84\x8Ca&cV[\x90\x91\x92P\x90\x91P\x80\x93P\x81\x94PPP`\0`@Q\x80`\xC0\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01\x8D`\0\x01Q`\x01`\x01`\x80\x1B\x03\x16\x81R` \x01a\x1B\xB1\x8E` \x01Qc\xFF\xFF\xFF\xFF\x16a'\x10g\r\xE0\xB6\xB3\xA7d\0\0\x91\x90\x91\x02\x04\x90V[\x81R` \x01a\x1B\xC0\x8E\x8Da\x1E6V[\x81R` \x01`\0\x81RP\x90Pa\x1B\xEC\x8D`@\x01Q`\x01`\x01`\x80\x1B\x03\x16\x85a\x1Er\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x81R`@\x8D\x01Qa\x1C\x07\x90\x84\x90`\x01`\x01`\x80\x1B\x03\x16a\x1ErV[` \x82\x01Ra\x1C\x15\x81a$~V[\x94P\x8A`\x80\x01Qa\x1C*W\x80` \x01Qa\x1C-V[\x80Q[\x96PPPPP\x96P\x96P\x96\x93PPPPV[`\0\x80a\x1CL\x84\x84a:VV[\x90P`\x01\x81\x12\x15a\x1CaW`\0\x91PPa\x19'V[P`\x01\x93\x92PPPV[\x84T`\x01`\xC0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x15a\x1C\x98W`@Qc\x1E\x13\x89\xA1`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x15a\x1C\xE5W\x84T`\xFF`\xE0\x1B\x19\x16`\x01`\xE0\x1B\x82\x15\x15\x02\x17\x85Ua\x1C\xC0c\x01\xE1\x85Ya\x19-V[\x85Tc\xFF\xFF\xFF\xFF\x91\x90\x91\x16`\x01`\xA0\x1B\x02c\xFF\xFF\xFF\xFF`\xA0\x1B\x19\x90\x91\x16\x17\x85Ua\x1DWV[a\x1D\x10b\x01Q\x80a\x1C\xFBc\x01\xE1\x85Y`\x03a:vV[\x80\x85\x10\x90\x85\x14\x17\x81\x85\x11\x91\x85\x14\x91\x90\x91\x17\x16\x90V[a\x1D-W`@Qc\xB5\x97\x03\x0F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1D6\x82a\x19-V[\x85Tc\xFF\xFF\xFF\xFF\x91\x90\x91\x16`\x01`\xA0\x1B\x02c\xFF\xFF\xFF\xFF`\xA0\x1B\x19\x90\x91\x16\x17\x85U[aa\xA8\x80\x84\x10\x90\x84\x14\x17`\x01\x80\x85\x11\x90\x85\x14\x17\x16a\x1D\x88W`@Qc9]8\x19`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1D\x91\x83a\x19-V[\x85Tc\xFF\xFF\xFF\xFF`\x80\x1B\x19\x16`\x01`\x80\x1Bc\xFF\xFF\xFF\xFF\x92\x83\x16\x02\x17\x86Ua\x1D\xC6\x90\x85\x90`\x01\x90`\x01`\x01`\x80\x1B\x03\x90a&\x97\x16V[a\x1D\xE3W`@Qc\xB2B\xE3A`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1D\xEC\x84a\x16gV[\x85T`\x01`\x01`\x80\x1B\x03\x91\x90\x91\x16\x7F\xFF\xFF\xFF\xFF\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x17`\x01`\xC0\x1BBc\xFF\xFF\xFF\xFF\x16\x02\x17\x90\x94UPPPPV[`\0\x82`\x80\x01Q\x15a\x1EMWPc\x01\xE1\x85Ya\x19'V[`\0a\x1EX\x84a&\xADV[c\xFF\xFF\xFF\xFF\x16\x80\x84\x03\x90\x84\x10\x02\x83\x03\x92\x90\x92\x03\x93\x92PPPV[`\0a\x07[\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\"\x8AV[`\0g\r\xE0\xB6\xB3\xA7d\0\0\x81\x81\x84\x10a\x1E\xA5W`\0\x92PPPa\x19'V[\x80\x84\x11a\x1E\xBEWP`\x01`\x01`\x80\x1B\x03\x91Pa\x19'\x90PV[`\x80\x85\x01Q`\0\x90a\x1E\xD4\x90c\x01\xE1\x85Ya\x1ErV[\x90P`\0a\x1E\xE1\x87a&\xE2V[\x90P`\0a\x1E\xFFa\x1E\xFA\x88g\r\xE0\xB6\xB3\xA7d\0\0a9\xB6V[a'7V[\x90P`\0a\x1F\r\x83\x83a:\x8DV[\x90P`\0g\x1B\xC1mgN\xC8\0\0\x85\x8B``\x01Q\x8C``\x01Qa\x1F/\x91\x90a:vV[a\x1F9\x91\x90a:vV[a\x1FC\x91\x90a:\xD3V[\x90P`\0a\x1Flg\r\xE0\xB6\xB3\xA7d\0\0a\x1F]\x84\x86a:VV[a\x1Fg\x91\x90a:\xE7V[a'\xD4V[\x90Pa\x1F\x85\x8B`@\x01Q\x82a \xDB\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x9B\x9APPPPPPPPPPPV[\x83Q` \x85\x01Q`\0\x91`\x01`\x01`\x80\x1B\x03\x90\x81\x16\x91\x16\x84\x15a\x1F\xF3W`@\x87\x01Qa\x1F\xCA\x90\x83\x90`\x01`\x01`\x80\x1B\x03\x16a\x1ErV[\x91Pa\x1F\xEC\x87`@\x01Q`\x01`\x01`\x80\x1B\x03\x16\x82a)}\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa 1V[`@\x87\x01Qa \x0C\x90\x83\x90`\x01`\x01`\x80\x1B\x03\x16a)}V[\x91Pa .\x87`@\x01Q`\x01`\x01`\x80\x1B\x03\x16\x82a\x1Er\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P[`\0`@Q\x80`\xC0\x01`@R\x80\x84\x81R` \x01\x83\x81R` \x01\x88`\0\x01Q`\x01`\x01`\x80\x1B\x03\x16\x81R` \x01a \x7F\x89` \x01Qc\xFF\xFF\xFF\xFF\x16a'\x10g\r\xE0\xB6\xB3\xA7d\0\0\x91\x90\x91\x02\x04\x90V[\x81R` \x01a \x8E\x89\x88a\x1E6V[\x81R` \x01`\0\x81RP\x90Pa\x062\x81a$~V[`\0\x81`\x80\x01Q\x15a \xB7WP`\0a\x19'V[a \xC0\x82a&\xADV[c\xFF\xFF\xFF\xFF\x16\x83``\x01Qc\xFF\xFF\xFF\xFF\x16\x10\x15\x90P\x92\x91PPV[`\0a\x07[\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\"\x8AV[`\0a \xFD\x82`\x12a9\xB6V[a\x19'\x90`\na;\xF9V[`\0\x80\x80g\r\xE0\xB6\xB3\xA7d\0\0\x91P`\0\x80a!(\x86`@\x01Q\x90`\0\x90V[\x91P\x91P\x83\x86`\0\x01Q\x10a!@W\x95\x94PPPPPV[\x85Q\x83\x10a!QWP\x94\x93PPPPV[`\0a!\\\x87a&\xE2V[\x87Q\x90\x91P`\0\x90a!v\x90g\r\xE0\xB6\xB3\xA7d\0\0a9\xB6V[\x90P`\0a!\x83\x82a'7V[\x90P`\0\x89`\xA0\x01Q\x84\x83a!\x98\x91\x90a:VV[a!\xA2\x91\x90a<\x05V[\x90Pa!\xBB\x8A`@\x01Qa!\xB5\x83a)\x92V[\x90a \xDBV[\x9A\x99PPPPPPPPPPV[`\0\x80\x80g\r\xE0\xB6\xB3\xA7d\0\0\x91P`\0\x80a!\xE9\x86`@\x01Q\x90`\0\x90V[\x91P\x91P\x81\x86` \x01Q\x10a\"\x02WP\x90\x94\x93PPPPV[\x80\x86` \x01Q\x11a\"\x17WP\x91\x94\x93PPPPV[`\0a\"\"\x87a&\xE2V[\x90P`\0a\"A\x88`@\x01Q\x89` \x01Qa)}\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\"N\x82a'7V[\x90P`\0\x89`\xA0\x01Q\x84\x83a\"c\x91\x90a<\x05V[a\"m\x91\x90a:VV[\x90Pa\"x\x81a)\x92V[a!\xBB\x90g\r\xE0\xB6\xB3\xA7d\0\0a9\xB6V[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\"\xA2W`\0\x80\xFD[\x04\x92\x91PPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\"\xC1W`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[`\0\x80\x83\x80` \x01\x90Q\x81\x01\x90a\"\xEE\x91\x90a<-V[\x83\x81R`\xA0\x81\x01Q\x90\x91Pa#\x05\x90`\x01\x90a<\x05V[a#\x0E\x82a$~V[a#\x18\x91\x90a:VV[\x94\x93PPPPV[`\0\x80\x83\x80` \x01\x90Q\x81\x01\x90a#7\x91\x90a<-V[` \x81\x01\x84\x90R`\xA0\x81\x01Q\x90\x91Pa#\x05\x90`\x01\x90a<\x05V[`\0\x84\x86\x11\x15a#\x84W`@Qc0\x82\xDF\xDB`\xE1\x1B\x81R`\x04\x81\x01\x87\x90R`$\x81\x01\x86\x90R`D\x01[`@Q\x80\x91\x03\x90\xFD[`\0a#\x94\x88\x88\x85c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a#\xA6\x89\x88\x86c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a#\xB4\x82\x84a:\x8DV[\x13\x15a#\xDDW`@Qc\x06\xF1\xBE]`\xE2\x1B\x81R`\x04\x81\x01\x83\x90R`$\x81\x01\x82\x90R`D\x01a#{V[`\0a#\xE9\x89\x89a9\xB6V[\x90P`\0[`\x02a#\xFA\x8A\x8Ca<\xA6V[a$\x04\x91\x90a:\xD3V[\x94P`\0a$\x16\x8C\x87\x89c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a$$\x86\x83a:\x8DV[\x13a$1W\x85\x99Pa$8V[\x85\x9AP\x80\x94P[a$B\x8B\x8Ba9\xB6V[\x92PP`\x01\x01\x87\x82\x11\x80\x15a$VWP\x86\x81\x10[a#\xEEWPPPP\x96\x95PPPPPPV[`\0a\x07[\x82\x84``\x01Qc\xFF\xFF\xFF\xFF\x16a\x1E6V[`\0\x80a$\x8A\x83a&\xE2V[\x90P`\0\x80g\r\xE0\xB6\xB3\xA7d\0\0\x91P`\0\x82\x86`\0\x01Q\x10a$\xAFWP`\x01a$\xE8V[\x85Q\x82\x10a$\xD1Wa$\xCA`\x01g\r\xE0\xB6\xB3\xA7d\0\0a9\xB6V[\x90Pa$\xE8V[\x85Qa$\xE5\x90g\r\xE0\xB6\xB3\xA7d\0\0a9\xB6V[\x90P[`\0a%\x05\x87`@\x01Q\x88` \x01Qa\x1Er\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0\x81\x10a%0Wa%)`\x01g\r\xE0\xB6\xB3\xA7d\0\0a9\xB6V[\x90Pa%<V[`\0\x81\x11a%<WP`\x01[`\0a%G\x83a'7V[\x90P`\0a%T\x83a'7V[\x90P\x86a%a\x83\x83a:VV[a%k\x91\x90a<\x05V[\x99\x98PPPPPPPPPV[`\0\x80a%\x92\x84`@\x01Q\x84a\x1Er\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x80\x15a&\\W`\0a%\xA5\x82a)\xFBV[\x90P`\0a%\xC4c\x01\xE1\x85Y\x87`\x80\x01Qa\x1Er\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[``\x87\x01Q\x90\x91P`\0\x90g\x1B\xC1mgN\xC8\0\0\x90a%\xE3\x90\x80a:vV[a%\xED\x91\x90a:\xD3V[\x90P`\0a%\xFA\x88a&\xE2V[\x90P`\0a&\x10g\r\xE0\xB6\xB3\xA7d\0\0\x86a:\x8DV[\x90Pa&\x1C\x84\x84a:\x8DV[a&&\x90\x82a<\x05V[\x90Pa&2\x82\x82a:\xE7V[\x90P`\0a&?\x82a)\x92V[\x90Pa&S\x81g\r\xE0\xB6\xB3\xA7d\0\0a:VV[\x97PPPPPPP[P\x92\x91PPV[`\0\x80\x80\x80a&s\x89\x87\x87a+\xD6V[\x90\x94P\x92Pa&\x85\x89\x89\x89\x87\x87a,\x1BV[\x94\x9A\x93\x99P\x97P\x92\x95P\x90\x93PPPPV[\x81\x83\x14\x91\x83\x11\x91\x90\x91\x17\x82\x82\x14\x91\x90\x92\x10\x17\x16\x90V[`\0\x81`\x80\x01Q\x15a&\xD2W`@Qc\xB0\x19\x84\x97`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P`@\x81\x01Q``\x90\x91\x01Q\x01\x90V[`\0\x80a'\0c\x01\xE1\x85Y\x84`\x80\x01Qa\x1Er\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0c;\x9A\xCA\0a'\x12\x83a-SV[a'\x1C\x91\x90a:vV[\x90P`\0a\x10\x81\x82\x86``\x01Qa \xDB\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[`\0g\x06\xF0[Y\xD3\xB2\0\0\x82\x03a'PWP`\0\x91\x90PV[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x12a'xW`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a'\x99W`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a'\xA6\x83`\x02a:\x8DV[\x90P`\0a'\xB3\x82a-\xF7V[\x90P`\0a'\xC9g\x13\xA0K\xBD\xFD\xC9\xBE\x88\x83a0uV[\x90Pa\x10\x81\x81a<\xB9V[`\0h\x02H\xCE6\xA7\x0C\xB2k>\x19\x82\x13a'\xEFWP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a(6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x01a#{V[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05`\x01`_\x1B\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1Dm\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1Dn\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[`\0a\x07[\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\"\xA9V[`\0\x80g\x13\xA0K\xBD\xFD\xC9\xBE\x88a)\xB0g\r\xE0\xB6\xB3\xA7d\0\0\x85a:\x8DV[a)\xBA\x91\x90a:\xE7V[\x90P`\0a)\xC7\x82a<\xB9V[\x90P`\0a)\xD4\x82a0\x8AV[\x90Pg\x1B\xC1mgN\xC8\0\0a)\xF1g\r\xE0\xB6\xB3\xA7d\0\0\x83a:\x8DV[a\x10\x81\x91\x90a:\xE7V[`\0\x80\x82\x13a*8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a#{V[`\0``a*E\x84a2nV[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[\x82Q`\0\x90\x81\x90a+\xF3\x90`\x01`\x01`\x80\x1B\x03\x16\x85a'\x10a\"\xA9V[\x91P\x82\x15a\x12\x1DWa,\x05\x83\x83a:\xD3V[\x90Pa,\x11\x81\x83a9\xB6V[\x91P\x93P\x93\x91PPV[`\0\x80`\0\x87`\x80\x01Qa,/W\x85a,1V[\x86[\x90P`\0\x88`\x80\x01Qa,DW\x87a,FV[\x86[\x89Q\x90\x91Pa,^\x90`\x01`\x01`\x80\x1B\x03\x16\x83a<\xA6V[\x91Pa,j\x86\x83a9\xB6V[\x91P\x81\x85\x11\x15a,\x8DW`@QcvG\x0F\xE7`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a,\x97\x85\x83a9\xB6V[\x91P\x80\x89` \x01Q`\x01`\x01`\x80\x1B\x03\x16\x11\x15a,\xC7W`@Qc\x86j\x03+`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x89\x01Qa,\xDF\x90`\x01`\x01`\x80\x1B\x03\x16\x82a9\xB6V[\x90P\x88`\x80\x01Qa,\xF0W\x80a,\xF2V[\x81[\x93P\x88`\x80\x01Qa-\x03W\x81a-\x05V[\x80[\x92P\x83\x88\x03a-'W`@Qc9;xE`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82\x87\x03a-GW`@Qc\x1F\xB0\xB7\xDD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PP\x95P\x95\x93PPPPV[`\xB5\x81`\x01`\x88\x1B\x81\x10a-lW`@\x91\x90\x91\x1B\x90`\x80\x1C[i\x01\0\0\0\0\0\0\0\0\0\x81\x10a-\x88W` \x91\x90\x91\x1B\x90`@\x1C[e\x01\0\0\0\0\0\x81\x10a-\xA0W`\x10\x91\x90\x91\x1B\x90` \x1C[c\x01\0\0\0\x81\x10a-\xB6W`\x08\x91\x90\x91\x1B\x90`\x10\x1C[b\x01\0\0\x01\x02`\x12\x1C\x80\x82\x04\x01`\x01\x90\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x90\x81\x90\x04\x81\x11\x90\x03\x90V[`\0\x80\x82\x12\x80a.\x0EWPg\x1B\xC1mgN\xC8\0\0\x82\x13[\x15a.,W`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a.MW`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81g\x1B\xC1mgN\xC8\0\0\x03a.uW`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x15a.\x80W\x91\x90PV[`\0g\r\xE0\xB6\xB3\xA7d\0\0\x83\x12a.\xA8Wa.\xA3\x83g\x1B\xC1mgN\xC8\0\0a:VV[a.\xAAV[\x82[\x90P`\0a.\xC0\x82g\x1B\xC1mgN\xC8\0\0a3\x0CV[\x90P\x80`\0\x03a.\xE3W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a.\xEE\x82a)\xFBV[\x90P`\0c;\x9A\xCA\0a/\x19a/\x14a/\x0Eg\x1B\xC1mgN\xC8\0\0a<\xB9V[\x85a0uV[a-SV[a/#\x91\x90a:\x8DV[\x90P`\0\x80a/:\x83g\x03\xC1f\\z\xAB \0a0uV[a/L\x90g \x05\xFEO&\x8E\xA0\0a<\x05V[\x90P`\0a/|\x84a/e\x86f\x9F2u$b\xA0\0a0uV[a/w\x90g\r\xC5R\x7Fd, \0a<\x05V[a0uV[a/\x8E\x90g\r\xE0\xB6\xB3\xA7d\0\0a<\x05V[\x90Pa/\xB2g\t\xD0(\xCCo _\xFF\x19\x85a/\xA8\x85\x85a3\x0CV[a/w\x91\x90a:VV[\x92PPP`\0[`\x02\x81\x10\x15a0MW`\0\x86a/\xCE\x84a0\x8AV[a/\xD8\x91\x90a:VV[\x90P`\0a/\xE6\x84\x85a0uV[a/\xEF\x90a<\xB9V[\x90P`\0a/\xFC\x82a'\xD4V[\x90P`\0a0\n\x86\x85a0uV[a0\x1Cg\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x84a0uV[a0&\x91\x90a:VV[\x90Pa02\x84\x82a3\x0CV[a0<\x90\x87a<\x05V[\x95P\x84`\x01\x01\x94PPPPPa/\xB9V[g\r\xE0\xB6\xB3\xA7d\0\0\x88\x12a0jWa0e\x82a<\xB9V[a\x062V[P\x96\x95PPPPPPV[`\0a\x07[\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a3\x1DV[`\0\x81`\0\x03a0\xA3WPg\r\xE0\xB6\xB3\xA7d\0\0\x91\x90PV[gV\x98\xEE\xF0fp\0\0\x82\x12a0\xBAWP`\0\x91\x90PV[a0\xCBgV\x98\xEE\xF0fp\0\0a<\xB9V[\x82\x13a0\xE0WPg\x1B\xC1mgN\xC8\0\0\x91\x90PV[`\0a0\xEB\x83a3<V[\x90P`\0a1$g\r\xE0\xB6\xB3\xA7d\0\0a1\r\x84g\x1B\xC1mgN\xC8\0\0a\x1ErV[a1\x1F\x90g\r\xE0\xB6\xB3\xA7d\0\0a<\x05V[a3\x0CV[\x90P`\0\x80\x82a1\x80\x81a1m\x81a1[\x81a1H\x81g\x02_\x0F\xE1\x05\xA3\x14\0a0uV[a/w\x90g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19a<\x05V[a/w\x90g\x14\xA8EL\x19\xE1\xAC\0a<\x05V[a/w\x90g\x0F\xC1\x0E\x01W\x82w\xFF\x19a<\x05V[a1\x92\x90g\x03\xDE\xBD\x08;\x8C|\0a<\x05V[\x91P\x83\x90Pa1\xFA\x81a1\xE8\x81a1\xD6\x81a1\xC4\x81a1\xB1\x81\x8Ba0uV[a/w\x90g\x02\x95\xD4\0\xEA2W\xFF\x19a<\x05V[a/w\x90g\x01W\xD8\xB2\xEC\xC7\x08\0a<\x05V[a/w\x90g\x051\n\xA7\xD5!0\0a<\x05V[a/w\x90g\r\xE0\xCC=\x15a\0\0a<\x05V[\x91P\x81g\x11\x90\0\xAB\x10\x0F\xFC\0a2\x10\x87\x88a0uV[a2\x1C\x90`\0\x19a:\x8DV[a2&\x91\x90a:VV[a20\x91\x90a<\x05V[\x92PP`\0a2>\x83a'\xD4V[\x90P`\0a2L\x85\x83a0uV[\x90P`\0\x88\x12a2\\W\x80a\x062V[a\x062\x81g\x1B\xC1mgN\xC8\0\0a:VV[`\0\x80\x82\x11a2\xABW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a#{V[P`\x01`\x01`\x01`\x80\x1B\x03\x82\x11`\x07\x1B\x82\x81\x1C`\x01`\x01`@\x1B\x03\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[`\0a\x07[\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x05\x85\x14\x17\x16a35W`\0\x80\xFD[\x05\x92\x91PPV[`\0`\x01`\xFF\x1B\x82\x03a3bW`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x12\x15a\x16yWP\x19`\x01\x01\x90V[\x91\x90PV[`@Q\x80a\x01@\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\0`\xFF\x16\x81R` \x01`\0`\xFF\x16\x81RP\x90V[\x805`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a3sW`\0\x80\xFD[\x80\x15\x15\x81\x14a4\x08W`\0\x80\xFD[PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a4\x08W`\0\x80\xFD[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a46W`\0\x80\xFD[a4?\x85a3\xE3V[\x93P` \x85\x015a4O\x81a3\xFAV[\x92P`@\x85\x015\x91P``\x85\x015a4f\x81a4\x0BV[\x93\x96\x92\x95P\x90\x93PPV[`\0` \x82\x84\x03\x12\x15a4\x83W`\0\x80\xFD[a\x07[\x82a3\xE3V[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a4\xA4W`\0\x80\xFD[\x855\x94P` \x86\x015\x93P`@\x86\x015\x92P``\x86\x015a4\xC4\x81a3\xFAV[\x94\x97\x93\x96P\x91\x94`\x80\x015\x92\x91PPV[``\x81R`\0\x84Q\x80``\x84\x01R`\0[\x81\x81\x10\x15a5\x03W` \x81\x88\x01\x81\x01Q`\x80\x86\x84\x01\x01R\x01a4\xE6V[P`\0`\x80\x82\x85\x01\x01R`\x80`\x1F\x19`\x1F\x83\x01\x16\x84\x01\x01\x91PP\x83` \x83\x01R\x82`@\x83\x01R\x94\x93PPPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\xA0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a5iWa5ia51V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a5\x97Wa5\x97a51V[`@R\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a5\xB2W`\0\x80\xFD[\x825\x91P` \x80\x84\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a5\xD1W`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a5\xE5W`\0\x80\xFD[\x815\x81\x81\x11\x15a5\xF7Wa5\xF7a51V[a6\t`\x1F\x82\x01`\x1F\x19\x16\x85\x01a5oV[\x91P\x80\x82R\x87\x84\x82\x85\x01\x01\x11\x15a6\x1FW`\0\x80\xFD[\x80\x84\x84\x01\x85\x84\x017`\0\x84\x82\x84\x01\x01RP\x80\x93PPPP\x92P\x92\x90PV[`\x01`\x01`\x80\x1B\x03\x81\x16\x81\x14a4\x08W`\0\x80\xFD[`\0\x80`\0\x83\x85\x03`\xE0\x81\x12\x15a6hW`\0\x80\xFD[`\xA0\x81\x12\x15a6vW`\0\x80\xFD[Pa6\x7Fa5GV[\x845a6\x8A\x81a6=V[\x81R` \x85\x015a6\x9A\x81a6=V[` \x82\x01R`@\x85\x015a6\xAD\x81a3\xFAV[`@\x82\x01Ra6\xBE``\x86\x01a3\xE3V[``\x82\x01R`\x80\x85\x015a6\xD1\x81a3\xFAV[`\x80\x82\x01R\x92P`\xA0\x84\x015\x91P`\xC0\x84\x015a6\xED\x81a4\x0BV[\x80\x91PP\x92P\x92P\x92V[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a7\x0EW`\0\x80\xFD[a7\x17\x85a3\xE3V[\x96` \x86\x015\x96P`@\x86\x015\x95``\x015\x94P\x92PPPV[`\0\x80`\0`@\x84\x86\x03\x12\x15a7FW`\0\x80\xFD[a7O\x84a3\xE3V[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a7kW`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a7\x7FW`\0\x80\xFD[\x815\x81\x81\x11\x15a7\x8EW`\0\x80\xFD[\x87` \x82\x85\x01\x01\x11\x15a7\xA0W`\0\x80\xFD[` \x83\x01\x94P\x80\x93PPPP\x92P\x92P\x92V[`\0\x80`\0``\x84\x86\x03\x12\x15a7\xC8W`\0\x80\xFD[a7\xD1\x84a3\xE3V[\x92P` \x84\x015a7\xE1\x81a3\xFAV[\x91P`@\x84\x015a6\xED\x81a4\x0BV[\x80Qa3s\x81a6=V[\x80Qc\xFF\xFF\xFF\xFF\x81\x16\x81\x14a3sW`\0\x80\xFD[\x80Qa\xFF\xFF\x81\x16\x81\x14a3sW`\0\x80\xFD[\x80Qa3s\x81a4\x0BV[`\0a\x01\0\x80\x83\x85\x03\x12\x15a8AW`\0\x80\xFD[`@Q\x90\x81\x01\x90`\x01`\x01`@\x1B\x03\x82\x11\x81\x83\x10\x17\x15a8cWa8ca51V[\x81`@R\x83Q\x91Pa8t\x82a6=V[\x81\x81Ra8\x83` \x85\x01a7\xF1V[` \x82\x01Ra8\x94`@\x85\x01a7\xF1V[`@\x82\x01Ra8\xA5``\x85\x01a7\xFCV[``\x82\x01Ra8\xB6`\x80\x85\x01a8\x10V[`\x80\x82\x01Ra8\xC7`\xA0\x85\x01a8\x10V[`\xA0\x82\x01Ra8\xD8`\xC0\x85\x01a8\"V[`\xC0\x82\x01Ra8\xE9`\xE0\x85\x01a8\"V[`\xE0\x82\x01R\x94\x93PPPPV[\x80Q`\xFF\x81\x16\x81\x14a3sW`\0\x80\xFD[`\0`\x80\x82\x84\x03\x12\x15a9\x19W`\0\x80\xFD[`@Q`\x80\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a9;Wa9;a51V[`@R\x82Qa9I\x81a4\x0BV[\x81Ra9W` \x84\x01a8\xF6V[` \x82\x01R`@\x83\x01Qa9j\x81a4\x0BV[`@\x82\x01Ra9{``\x84\x01a8\xF6V[``\x82\x01R\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a9\x99W`\0\x80\xFD[PQ\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x19'Wa\x19'a9\xA0V[`\0`\x01\x82\x01a9\xDBWa9\xDBa9\xA0V[P`\x01\x01\x90V[`\0`\xA0\x82\x84\x03\x12\x15a9\xF4W`\0\x80\xFD[a9\xFCa5GV[\x82Qa:\x07\x81a6=V[\x81Ra:\x15` \x84\x01a7\xFCV[` \x82\x01Ra:&`@\x84\x01a7\xFCV[`@\x82\x01Ra:7``\x84\x01a7\xFCV[``\x82\x01R`\x80\x83\x01Qa:J\x81a3\xFAV[`\x80\x82\x01R\x93\x92PPPV[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a&\\Wa&\\a9\xA0V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x19'Wa\x19'a9\xA0V[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15a:\xA9Wa:\xA9a9\xA0V[\x81\x81\x05\x83\x14\x82\x15\x17a\x19'Wa\x19'a9\xA0V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a:\xE2Wa:\xE2a:\xBDV[P\x04\x90V[`\0\x82a:\xF6Wa:\xF6a:\xBDV[`\x01`\xFF\x1B\x82\x14`\0\x19\x84\x14\x16\x15a;\x10Wa;\x10a9\xA0V[P\x05\x90V[`\x01\x81\x81[\x80\x85\x11\x15a;PW\x81`\0\x19\x04\x82\x11\x15a;6Wa;6a9\xA0V[\x80\x85\x16\x15a;CW\x91\x81\x02\x91[\x93\x84\x1C\x93\x90\x80\x02\x90a;\x1AV[P\x92P\x92\x90PV[`\0\x82a;gWP`\x01a\x19'V[\x81a;tWP`\0a\x19'V[\x81`\x01\x81\x14a;\x8AW`\x02\x81\x14a;\x94Wa;\xB0V[`\x01\x91PPa\x19'V[`\xFF\x84\x11\x15a;\xA5Wa;\xA5a9\xA0V[PP`\x01\x82\x1Ba\x19'V[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a;\xD3WP\x81\x81\na\x19'V[a;\xDD\x83\x83a;\x15V[\x80`\0\x19\x04\x82\x11\x15a;\xF1Wa;\xF1a9\xA0V[\x02\x93\x92PPPV[`\0a\x07[\x83\x83a;XV[\x80\x82\x01\x82\x81\x12`\0\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a<%Wa<%a9\xA0V[PP\x92\x91PPV[`\0`\xC0\x82\x84\x03\x12\x15a<?W`\0\x80\xFD[`@Q`\xC0\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a<aWa<aa51V[\x80`@RP\x82Q\x81R` \x83\x01Q` \x82\x01R`@\x83\x01Q`@\x82\x01R``\x83\x01Q``\x82\x01R`\x80\x83\x01Q`\x80\x82\x01R`\xA0\x83\x01Q`\xA0\x82\x01R\x80\x91PP\x92\x91PPV[\x80\x82\x01\x80\x82\x11\x15a\x19'Wa\x19'a9\xA0V[`\0`\x01`\xFF\x1B\x82\x01a<\xCEWa<\xCEa9\xA0V[P`\0\x03\x90V\xFE\xA2dipfsX\"\x12 \x8F\xC4j!\x94\x95\x99E\x9A\x0F\xD1\xE8\xEB9\xB2\xA9\xCA\xD1\x1BJ\xDE\xBDK\x16\x9A,\x86\x88\xAF$\x0B-dsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static NORMALSTRATEGY_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xCFW`\x005`\xE0\x1C\x80c\x80\xAF\x9Dv\x11a\0\x8CW\x80c\xE31\xBA4\x11a\0fW\x80c\xE31\xBA4\x14a\x02\xABW\x80c\xE6\x04{\x19\x14a\x02\xBEW\x80c\xECshT\x14a\x02\xD2W\x80c\xF0{\x87\x9E\x14a\x02\xE5W`\0\x80\xFD[\x80c\x80\xAF\x9Dv\x14a\x02.W\x80c\xA4G\x89\x19\x14a\x02^W\x80c\xE0hx\x7F\x14a\x02\x88W`\0\x80\xFD[\x80c\x16\xED\xE0\x16\x14a\0\xD4W\x80c\x19\x05x\x07\x14a\x01\x18W\x80c4\xDB\xC7;\x14a\x019W\x80c9CMZ\x14a\x01\xD1W\x80cE-/\x18\x14a\x01\xE4W\x80cK\xF3F\xBF\x14a\x02\x06W[`\0\x80\xFD[a\0\xFB\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01+a\x01&6`\x04a4 V[a\x03TV[`@Q\x90\x81R` \x01a\x01\x0FV[a\x01\x91a\x01G6`\x04a4qV[`\0` \x81\x90R\x90\x81R`@\x90 T`\x01`\x01`\x80\x1B\x03\x81\x16\x90c\xFF\xFF\xFF\xFF`\x01`\x80\x1B\x82\x04\x81\x16\x91`\x01`\xA0\x1B\x81\x04\x82\x16\x91`\x01`\xC0\x1B\x82\x04\x16\x90`\xFF`\x01`\xE0\x1B\x90\x91\x04\x16\x85V[`@\x80Q`\x01`\x01`\x80\x1B\x03\x90\x96\x16\x86Rc\xFF\xFF\xFF\xFF\x94\x85\x16` \x87\x01R\x92\x84\x16\x92\x85\x01\x92\x90\x92R\x91\x90\x91\x16``\x83\x01R\x15\x15`\x80\x82\x01R`\xA0\x01a\x01\x0FV[a\x01+a\x01\xDF6`\x04a4qV[a\x06>V[a\x01\xF7a\x01\xF26`\x04a4\x8CV[a\x07bV[`@Qa\x01\x0F\x93\x92\x91\x90a4\xD5V[a\x02\x19a\x02\x146`\x04a5\x9FV[a\x08VV[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x01\x0FV[a\x02Aa\x02<6`\x04a6RV[a\x08\x8AV[`@\x80Q\x93\x15\x15\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x01\x0FV[a\x02qa\x02l6`\x04a6\xF8V[a\x0B\xE6V[`@\x80Q\x92\x15\x15\x83R` \x83\x01\x91\x90\x91R\x01a\x01\x0FV[a\x02\x9Ba\x02\x966`\x04a71V[a\rIV[`@Q\x90\x15\x15\x81R` \x01a\x01\x0FV[a\x01+a\x02\xB96`\x04a4qV[a\x0F0V[a\x02\x9Ba\x02\xCC6`\x04a4qV[P`\x01\x90V[a\x02qa\x02\xE06`\x04a7\xB3V[a\x10\x8AV[a\x02\xF8a\x02\xF36`\x04a7\xB3V[a\x12%V[`@Qa\x01\x0F\x91\x90`\0`\xA0\x82\x01\x90P`\x01`\x01`\x80\x1B\x03\x80\x84Q\x16\x83R\x80` \x85\x01Q\x16` \x84\x01RP`@\x83\x01Q\x15\x15`@\x83\x01R`\x01`\x01`@\x1B\x03``\x84\x01Q\x16``\x83\x01R`\x80\x83\x01Q\x15\x15`\x80\x83\x01R\x92\x91PPV[`@Qc\"i|!`\xE2\x1B\x81R`\x01`\x01`@\x1B\x03\x85\x16`\x04\x82\x01R`\0\x90\x81\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x89\xA5\xF0\x84\x90`$\x01a\x01\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\xC7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xEB\x91\x90a8-V[\x90P`\0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16c^Gf<` \x89\x90\x1Cc\xFF\xFF\xFF\xFF\x16`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x81Rb\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01R`$\x01`\x80`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04iW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\x8D\x91\x90a9\x07V[\x90Pa\x04\xB1\x86a\x04\xA1W\x81``\x01Qa\x04\xA7V[\x81` \x01Q[\x86\x90`\xFF\x16a\x16PV[`\x01`\x01`@\x1B\x03\x88\x16`\0\x90\x81R` \x81\x81R`@\x91\x82\x90 \x82Q`\xA0\x80\x82\x01\x85R\x91T`\x01`\x01`\x80\x1B\x03\x81\x16\x82R`\x01`\x80\x1B\x81\x04c\xFF\xFF\xFF\xFF\x90\x81\x16\x94\x83\x01\x94\x90\x94R`\x01`\xA0\x1B\x81\x04\x84\x16\x82\x86\x01R`\x01`\xC0\x1B\x81\x04\x90\x93\x16``\x82\x01R`\x01`\xE0\x1B\x90\x92\x04`\xFF\x16\x15\x15`\x80\x83\x01R\x82Q\x90\x81\x01\x90\x92R\x91\x96Pa\x06\n\x91\x90\x80a\x05@\x89a\x16gV[`\x01`\x01`\x80\x1B\x03\x16\x81R` \x01`\x01`\x01`\x01`\x80\x1B\x03\x16\x81R` \x01`\0\x15\x15\x81R` \x01\x8A`\x01`\x01`@\x1B\x03\x16\x81R` \x01\x89\x15\x15\x81RPB\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xB0\xE2\x1E\x8A`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\xDBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xFF\x91\x90a9\x87V[\x86\x93\x92\x91\x90\x89a\x16}V[\x92P`\0\x86a\x06\x1DW\x81` \x01Qa\x06#V[\x81``\x01Q[`\xFF\x16\x90Pa\x062\x84\x82a\x186V[\x98\x97PPPPPPPPV[`@Qc\"i|!`\xE2\x1B\x81R`\x01`\x01`@\x1B\x03\x82\x16`\x04\x82\x01R`\0\x90\x81\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x89\xA5\xF0\x84\x90`$\x01a\x01\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\xB1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xD5\x91\x90a8-V[`\x01`\x01`@\x1B\x03\x84\x16`\0\x90\x81R` \x81\x81R`@\x91\x82\x90 \x82Q`\xA0\x81\x01\x84R\x90T`\x01`\x01`\x80\x1B\x03\x81\x16\x82R`\x01`\x80\x1B\x81\x04c\xFF\xFF\xFF\xFF\x90\x81\x16\x93\x83\x01\x93\x90\x93R`\x01`\xA0\x1B\x81\x04\x83\x16\x93\x82\x01\x93\x90\x93R`\x01`\xC0\x1B\x83\x04\x82\x16``\x82\x01R`\x01`\xE0\x1B\x90\x92\x04`\xFF\x16\x15\x15`\x80\x83\x01R\x91\x92Pa\x07[\x91\x83\x91\x90a\x18L\x16V[\x93\x92PPPV[```\0\x80`\0`@Q\x80`\xA0\x01`@R\x80a\x07}\x8Ba\x16gV[`\x01`\x01`\x80\x1B\x03\x16\x81R` \x01a\x07\x94\x8Aa\x19-V[c\xFF\xFF\xFF\xFF\x16\x81R` \x01a\x07\xA8\x89a\x19-V[c\xFF\xFF\xFF\xFF\x16\x81R`\0` \x82\x01R\x87\x15\x15`@\x90\x91\x01R\x90Pa\x08/\x81`@\x80Q\x82Q`\x01`\x01`\x80\x1B\x03\x16` \x80\x83\x01\x91\x90\x91R\x83\x01Qc\xFF\xFF\xFF\xFF\x90\x81\x16\x82\x84\x01R\x91\x83\x01Q\x82\x16``\x82\x81\x01\x91\x90\x91R\x83\x81\x01Q\x90\x92\x16`\x80\x80\x83\x01\x91\x90\x91R\x83\x01Q\x15\x15`\xA0\x82\x01R`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x91\x90PV[\x93Pa\x08D\x85a\x08>\x83a\x19@V[\x90a\x19\xE6V[\x94\x9A\x90\x99P\x93\x97P\x92\x95PPPPPPV[`\0\x80`\0a\x08d\x84a\x1A\nV[\x90P`\0a\x08q\x82a\x19@V[\x90Pa\x08}\x81\x87a\x19\xE6V[\x93P\x93PPP\x92P\x92\x90PV[``\x83\x01Q`@Qc\"i|!`\xE2\x1B\x81R`\x01`\x01`@\x1B\x03\x90\x91\x16`\x04\x82\x01R`\0\x90\x81\x90\x81\x90\x81\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x89\xA5\xF0\x84\x90`$\x01a\x01\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\x07W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t+\x91\x90a8-V[``\x88\x01Q\x90\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c^Gf<\x90` \x1Cc\xFF\xFF\xFF\xFF\x16`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x81Rb\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01R`$\x01`\x80`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\xB0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\xD4\x91\x90a9\x07V[\x90Pa\t\xDEa3xV[\x88`\x80\x01Q\x15a\n&W` \x82\x01Q`\xFF\x90\x81\x16a\x01\0\x83\x01R``\x83\x01Q\x16a\x01 \x82\x01R\x81Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\xC0\x83\x01R`@\x83\x01Q\x16`\xE0\x82\x01Ra\n`V[``\x82\x01Q`\xFF\x90\x81\x16a\x01\0\x83\x01R` \x83\x01Q\x16a\x01 \x82\x01R`@\x82\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\xC0\x83\x01R\x82Q\x16`\xE0\x82\x01R[\x88Q`\x01`\x01`\x80\x1B\x03\x90\x81\x16`\x80\x83\x01R` \x8A\x01Q\x16`\xA0\x82\x01Ra\n\x86\x81a\x1AkV[\x90Pa\n\x95\x81`\x80\x01Qa\x16gV[`\x01`\x01`\x80\x1B\x03\x16\x89R`\xA0\x81\x01Qa\n\xAE\x90a\x16gV[`\x01`\x01`\x80\x1B\x03\x90\x81\x16` \x8B\x81\x01\x91\x90\x91R``\x80\x8C\x01Q`\x01`\x01`@\x1B\x03\x16`\0\x90\x81R\x80\x83R`@\x90\x81\x90 \x81Q`\xA0\x81\x01\x83R\x90T\x94\x85\x16\x81R`\x01`\x80\x1B\x85\x04c\xFF\xFF\xFF\xFF\x90\x81\x16\x82\x86\x01R`\x01`\xA0\x1B\x86\x04\x81\x16\x82\x84\x01R`\x01`\xC0\x1B\x86\x04\x16\x92\x81\x01\x92\x90\x92R`\x01`\xE0\x1B\x90\x93\x04`\xFF\x16\x15\x15`\x80\x82\x01R\x82QcXq\x0FE`\xE1\x1B\x81R\x92Qa\x0B\xC7\x93\x91\x92\x8D\x92\x8D\x92\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x92c\xB0\xE2\x1E\x8A\x92`\x04\x80\x82\x01\x93\x91\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x0B\x98W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\xBC\x91\x90a9\x87V[\x87\x93\x92\x91\x90\x8Ca\x1A\xE7V[\x90\x96P\x94Pa\x0B\xD8\x90P\x85\x85a\x1C?V[\x95PPPP\x93P\x93P\x93\x90PV[`@Qc\"i|!`\xE2\x1B\x81R`\x01`\x01`@\x1B\x03\x85\x16`\x04\x82\x01R`\0\x90\x81\x90\x81\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x89\xA5\xF0\x84\x90`$\x01a\x01\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C[W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\x7F\x91\x90a8-V[\x90Pa\x0C\x8A\x85a\x16gV[`\x01`\x01`\x80\x1B\x03\x16\x81Ra\x0C\x9E\x84a\x16gV[`\x01`\x01`\x80\x1B\x03\x90\x81\x16` \x83\x81\x01\x91\x90\x91R`\x01`\x01`@\x1B\x03\x89\x16`\0\x90\x81R\x80\x82R`@\x80\x82 \x81Q`\xA0\x81\x01\x83R\x90T\x94\x85\x16\x81R`\x01`\x80\x1B\x85\x04c\xFF\xFF\xFF\xFF\x90\x81\x16\x94\x82\x01\x94\x90\x94R`\x01`\xA0\x1B\x85\x04\x84\x16\x91\x81\x01\x91\x90\x91R`\x01`\xC0\x1B\x84\x04\x83\x16``\x82\x01R`\x01`\xE0\x1B\x90\x93\x04`\xFF\x16\x15\x15`\x80\x84\x01R\x91a\r,\x91\x84\x91\x90a\x18L\x16V[\x90P`\0a\r:\x88\x83a\x1C?V[\x99\x91\x98P\x90\x96PPPPPPPV[`\x003`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\r\x94W`@Qc:#%k`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\r\xD5\x84\x84\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x1A\n\x92PPPV[\x90Pa\x0E8\x81`\0\x01Q`\x01`\x01`\x80\x1B\x03\x16\x82` \x01Qc\xFF\xFF\xFF\xFF\x16\x83`@\x01Qc\xFF\xFF\xFF\xFF\x16\x84`\x80\x01Q`\0\x80\x8B`\x01`\x01`@\x1B\x03\x16`\x01`\x01`@\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 a\x1Ck\x90\x94\x93\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[P`\x01`\x01`@\x1B\x03\x84\x16`\0\x81\x81R` \x81\x81R`@\x91\x82\x90 \x82Q`\xA0\x81\x01\x84R\x90T`\x01`\x01`\x80\x1B\x03\x81\x16\x80\x83R`\x01`\x80\x1B\x82\x04c\xFF\xFF\xFF\xFF\x90\x81\x16\x84\x86\x01\x81\x90R`\x01`\xA0\x1B\x84\x04\x82\x16\x85\x88\x01\x81\x90R`\x01`\xC0\x1B\x85\x04\x90\x92\x16``\x80\x87\x01\x91\x90\x91R`\x01`\xE0\x1B\x90\x94\x04`\xFF\x16\x15\x15`\x80\x80\x87\x01\x82\x90R\x88Q\x94\x85R\x96\x84\x01\x91\x90\x91R\x95\x82\x01R\x90\x81\x01\x93\x90\x93R\x92\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x91\x7F\x8Di\xFAt\x9A\xF4\x15\xF9<\x96\x86>\xBF6\x92\x93&#\xC0\x10\xEB\xA8\x11\x8B\x02\xF0DXZ\xABe\x13\x91\x01`@Q\x80\x91\x03\x90\xA3P`\x01\x94\x93PPPPV[`@Qc\"i|!`\xE2\x1B\x81R`\x01`\x01`@\x1B\x03\x82\x16`\x04\x82\x01R`\0\x90\x81\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x89\xA5\xF0\x84\x90`$\x01a\x01\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\xA3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xC7\x91\x90a8-V[`\x01`\x01`@\x1B\x03\x84\x16`\0\x90\x81R` \x81\x81R`@\x80\x83 \x81Q`\xA0\x81\x01\x83R\x90T`\x01`\x01`\x80\x1B\x03\x81\x16\x82Rc\xFF\xFF\xFF\xFF`\x01`\x80\x1B\x82\x04\x81\x16\x94\x83\x01\x94\x90\x94R`\x01`\xA0\x1B\x81\x04\x84\x16\x92\x82\x01\x92\x90\x92R`\x01`\xC0\x1B\x82\x04\x90\x92\x16``\x83\x01R`\xFF`\x01`\xE0\x1B\x90\x91\x04\x16\x15\x15`\x80\x82\x01R\x91\x92Pa\x10H\x82a\x19@V[\x90Pa\x10T\x82Ba\x1E6V[`\x80\x82\x01R`@\x83\x01Q\x83Qa\x10\x81\x91a\x10z\x91`\x01`\x01`\x80\x1B\x03\x90\x81\x16\x91\x16a\x1ErV[\x82\x90a\x1E\x87V[\x95\x94PPPPPV[`\0\x803`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x10\xD6W`@Qc:#%k`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qc\"i|!`\xE2\x1B\x81R`\x01`\x01`@\x1B\x03\x86\x16`\x04\x82\x01R`\0\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\x89\xA5\xF0\x84\x90`$\x01a\x01\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11GW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11k\x91\x90a8-V[`\x01`\x01`@\x1B\x03\x87\x16`\0\x90\x81R` \x81\x81R`@\x80\x83 \x81Q`\xA0\x81\x01\x83R\x90T`\x01`\x01`\x80\x1B\x03\x81\x16\x82R`\x01`\x80\x1B\x81\x04c\xFF\xFF\xFF\xFF\x90\x81\x16\x94\x83\x01\x94\x90\x94R`\x01`\xA0\x1B\x81\x04\x84\x16\x92\x82\x01\x92\x90\x92R`\x01`\xC0\x1B\x82\x04\x83\x16``\x82\x01R`\x01`\xE0\x1B\x90\x91\x04`\xFF\x16\x15\x15`\x80\x82\x01R\x92\x93Pa\x11\xF5\x90\x84\x90\x84\x90\x8A\x90B\x90a\x1F\x94\x16V[\x90Pa\x12\x01\x83\x83a \xA3V[\x15a\x12\x14W`\0\x94P\x92Pa\x12\x1D\x91PPV[`\x01\x94P\x92PPP[\x93P\x93\x91PPV[`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x91\x90\x91R`@Qc\"i|!`\xE2\x1B\x81R`\x01`\x01`@\x1B\x03\x85\x16`\x04\x82\x01R`\0\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\x89\xA5\xF0\x84\x90`$\x01a\x01\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12\xC1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\xE5\x91\x90a8-V[\x90P`\0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16c^Gf<` \x88\x90\x1Cc\xFF\xFF\xFF\xFF\x16`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x81Rb\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01R`$\x01`\x80`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13cW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\x87\x91\x90a9\x07V[`\x01`\x01`@\x1B\x03\x87\x16`\0\x90\x81R` \x81\x81R`@\x80\x83 \x81Q`\xA0\x81\x01\x83R\x90T`\x01`\x01`\x80\x1B\x03\x81\x16\x82Rc\xFF\xFF\xFF\xFF`\x01`\x80\x1B\x82\x04\x81\x16\x94\x83\x01\x94\x90\x94R`\x01`\xA0\x1B\x81\x04\x84\x16\x92\x82\x01\x92\x90\x92R`\x01`\xC0\x1B\x82\x04\x90\x92\x16``\x83\x01R`\xFF`\x01`\xE0\x1B\x90\x91\x04\x16\x15\x15`\x80\x82\x01R\x91\x92Pa\x14\x08\x82a\x19@V[\x90P`\0\x80g\r\xE0\xB6\xB3\xA7d\0\0\x91P`\0\x80a\x14)\x85`@\x01Q\x90`\0\x90V[\x91P\x91P`\0`@Q\x80`\xA0\x01`@R\x80`\0`\x01`\x01`\x80\x1B\x03\x16\x81R` \x01`\0`\x01`\x01`\x80\x1B\x03\x16\x81R` \x01`\0\x15\x15\x81R` \x01\x8E`\x01`\x01`@\x1B\x03\x16\x81R` \x01\x8D\x15\x15\x81RP\x90P`\0\x80\x8D\x15a\x15fW`\x01\x8B`\0\x01Q`\x01`\x01`\x80\x1B\x03\x16a\x14\xB3\x8D`@\x01Q`\x01`\x01`\x80\x1B\x03\x16\x8Aa \xDB\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x14\xBD\x91\x90a9\xB6V[a\x14\xC7\x91\x90a9\xB6V[\x91P`\x01a\x14\xEB\x8C`@\x01Q`\x01`\x01`\x80\x1B\x03\x16\x86a \xDB\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x8C` \x01Q`\x01`\x01`\x80\x1B\x03\x16a\x15\x03\x91\x90a9\xB6V[a\x15\r\x91\x90a9\xB6V[\x90Pa\x151a\x15,\x8B` \x01Q`\xFF\x16\x84a\x186\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x16gV[`\x01`\x01`\x80\x1B\x03\x16\x83R``\x8A\x01Qa\x15S\x90a\x15,\x90\x83\x90`\xFF\x16a\x186V[`\x01`\x01`\x80\x1B\x03\x16` \x84\x01Ra\x16=V[`\x01\x8B` \x01Q`\x01`\x01`\x80\x1B\x03\x16a\x15\x96\x8D`@\x01Q`\x01`\x01`\x80\x1B\x03\x16\x88a \xDB\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x15\xA0\x91\x90a9\xB6V[a\x15\xAA\x91\x90a9\xB6V[\x91P`\x01a\x15\xCE\x8C`@\x01Q`\x01`\x01`\x80\x1B\x03\x16\x88a \xDB\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x8CQa\x15\xE3\x91\x90`\x01`\x01`\x80\x1B\x03\x16a9\xB6V[a\x15\xED\x91\x90a9\xB6V[\x90Pa\x16\x0Ca\x15,\x8B``\x01Q`\xFF\x16\x84a\x186\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[`\x01`\x01`\x80\x1B\x03\x16\x83R` \x8A\x01Qa\x16.\x90a\x15,\x90\x83\x90`\xFF\x16a\x186V[`\x01`\x01`\x80\x1B\x03\x16` \x84\x01R[P\x90\x9D\x9CPPPPPPPPPPPPPV[`\0\x80a\x16\\\x83a \xF0V[\x93\x90\x93\x02\x93\x92PPPV[`\0`\x01`\x80\x1B\x82\x10a\x16yW`\0\x80\xFD[P\x90V[`\0\x80`\0\x80a\x16\x91\x8A\x8A\x8A\x8A\x8A\x8Aa\x1A\xE7V[\x92P\x92P\x92P`\0a\x16\xA2\x8Aa\x19@V[`\xA0\x81\x01\x84\x90R\x90Pa\x16\xB5\x8A\x89a\x1E6V[`\x80\x80\x83\x01\x91\x90\x91R\x89\x01Q`\0\x90\x80\x15a\x16\xE4W\x85\x83Ra\x16\xD6\x83a!\x08V[` \x84\x01\x81\x90R\x91Pa\x16\xFAV[` \x83\x01\x86\x90Ra\x16\xF4\x83a!\xC9V[\x80\x84R\x91P[\x81`\0\x03a\x17*W\x80a\x17\x0EW\x8CQa\x17\x14V[\x8C` \x01Q[`\x01`\x01`\x80\x1B\x03\x16\x96PPPPPPPa\x18,V[`\0a\x179\x83`2`da\"\x8AV[\x90P`\0a\x17J\x84`\x96`da\"\xA9V[\x90Pa\x17\xCA\x85`@Q` \x01a\x17\x9F\x91\x90`\0`\xC0\x82\x01\x90P\x82Q\x82R` \x83\x01Q` \x83\x01R`@\x83\x01Q`@\x83\x01R``\x83\x01Q``\x83\x01R`\x80\x83\x01Q`\x80\x83\x01R`\xA0\x83\x01Q`\xA0\x83\x01R\x92\x91PPV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83\x83`\x01a\x01\0\x88a\x17\xC2Wa\"\xD7a#RV[a# a#RV[\x93P\x83a\x17\xD6\x81a9\xC9V[\x94PPa\x17\xF9\x8F`@\x01Q`\x01`\x01`\x80\x1B\x03\x16\x85a \xDB\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x93P\x83\x83a\x18\x08W\x8FQa\x18\x0EV[\x8F` \x01Q[`\x01`\x01`\x80\x1B\x03\x16a\x18!\x91\x90a9\xB6V[\x98PPPPPPPPP[\x96\x95PPPPPPV[`\0\x80a\x18B\x83a \xF0V[\x90\x93\x04\x93\x92PPPV[`\0\x80a\x18|\x84`@\x01Q`\x01`\x01`\x80\x1B\x03\x16\x85`\0\x01Q`\x01`\x01`\x80\x1B\x03\x16a\x1Er\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x18\xAD\x85`@\x01Q`\x01`\x01`\x80\x1B\x03\x16\x86` \x01Q`\x01`\x01`\x80\x1B\x03\x16a\x1Er\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0`@Q\x80`\xC0\x01`@R\x80\x84\x81R` \x01\x83\x81R` \x01\x86`\0\x01Q`\x01`\x01`\x80\x1B\x03\x16\x81R` \x01a\x18\xFD\x87` \x01Qc\xFF\xFF\xFF\xFF\x16a'\x10g\r\xE0\xB6\xB3\xA7d\0\0\x91\x90\x91\x02\x04\x90V[\x81R` \x01a\x19\x0C\x88\x88a$hV[\x81R` \x01`\0\x81RP\x90Pa\x19!\x81a$~V[\x93PPPP[\x92\x91PPV[`\0d\x01\0\0\0\0\x82\x10a\x16yW`\0\x80\xFD[a\x19y`@Q\x80`\xC0\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`@Q\x80`\xC0\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01\x83`\0\x01Q`\x01`\x01`\x80\x1B\x03\x16\x81R` \x01a\x19\xC7\x84` \x01Qc\xFF\xFF\xFF\xFF\x16a'\x10g\r\xE0\xB6\xB3\xA7d\0\0\x91\x90\x91\x02\x04\x90V[\x81R`@\x93\x84\x01Qc\xFF\xFF\xFF\xFF\x16` \x82\x01R`\0\x93\x01\x92\x90\x92RP\x90V[`\0\x80a\x19\xF3\x84\x84a%xV[\x80\x85R\x91Pa\x1A\x01\x84a!\x08V[\x90P\x92P\x92\x90PV[`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x91\x90\x91R`\xA0\x82Q\x14a\x1AWW`@Qc\x01YW\xD3`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x80` \x01\x90Q\x81\x01\x90a\x19'\x91\x90a9\xE2V[a\x1Asa3xV[a\x01\0\x82\x01Q`@\x83\x01Qa\x1A\x8A\x91`\xFF\x16a\x16PV[`@\x83\x01Ra\x01\0\x82\x01Q``\x83\x01Qa\x1A\xA6\x91`\xFF\x16a\x16PV[``\x83\x01Ra\x01\0\x82\x01Q`\x80\x83\x01Qa\x1A\xC2\x91`\xFF\x16a\x16PV[`\x80\x83\x01Ra\x01 \x82\x01Q`\xA0\x83\x01Qa\x1A\xDE\x91`\xFF\x16a\x16PV[`\xA0\x83\x01RP\x90V[`\0\x80`\0a\x1A\xFC\x89\x89\x89`\x80\x01Q\x89a\x1F\x94V[\x89Q` \x8B\x01Q`\xC0\x8C\x01Q\x92\x94P`\x01`\x01`\x80\x1B\x03\x91\x82\x16\x92\x91\x16\x90`\0\x90`\x01`\x01`\xA0\x1B\x03\x88\x81\x16\x91\x16\x14a\x1B9W\x8B`\x80\x01Qa\x1B?V[\x8B`\xA0\x01Q[a\xFF\xFF\x16\x90Pa\x1BR\x8A\x84\x84\x84\x8Ca&cV[\x90\x91\x92P\x90\x91P\x80\x93P\x81\x94PPP`\0`@Q\x80`\xC0\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01\x8D`\0\x01Q`\x01`\x01`\x80\x1B\x03\x16\x81R` \x01a\x1B\xB1\x8E` \x01Qc\xFF\xFF\xFF\xFF\x16a'\x10g\r\xE0\xB6\xB3\xA7d\0\0\x91\x90\x91\x02\x04\x90V[\x81R` \x01a\x1B\xC0\x8E\x8Da\x1E6V[\x81R` \x01`\0\x81RP\x90Pa\x1B\xEC\x8D`@\x01Q`\x01`\x01`\x80\x1B\x03\x16\x85a\x1Er\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x81R`@\x8D\x01Qa\x1C\x07\x90\x84\x90`\x01`\x01`\x80\x1B\x03\x16a\x1ErV[` \x82\x01Ra\x1C\x15\x81a$~V[\x94P\x8A`\x80\x01Qa\x1C*W\x80` \x01Qa\x1C-V[\x80Q[\x96PPPPP\x96P\x96P\x96\x93PPPPV[`\0\x80a\x1CL\x84\x84a:VV[\x90P`\x01\x81\x12\x15a\x1CaW`\0\x91PPa\x19'V[P`\x01\x93\x92PPPV[\x84T`\x01`\xC0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x15a\x1C\x98W`@Qc\x1E\x13\x89\xA1`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x15a\x1C\xE5W\x84T`\xFF`\xE0\x1B\x19\x16`\x01`\xE0\x1B\x82\x15\x15\x02\x17\x85Ua\x1C\xC0c\x01\xE1\x85Ya\x19-V[\x85Tc\xFF\xFF\xFF\xFF\x91\x90\x91\x16`\x01`\xA0\x1B\x02c\xFF\xFF\xFF\xFF`\xA0\x1B\x19\x90\x91\x16\x17\x85Ua\x1DWV[a\x1D\x10b\x01Q\x80a\x1C\xFBc\x01\xE1\x85Y`\x03a:vV[\x80\x85\x10\x90\x85\x14\x17\x81\x85\x11\x91\x85\x14\x91\x90\x91\x17\x16\x90V[a\x1D-W`@Qc\xB5\x97\x03\x0F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1D6\x82a\x19-V[\x85Tc\xFF\xFF\xFF\xFF\x91\x90\x91\x16`\x01`\xA0\x1B\x02c\xFF\xFF\xFF\xFF`\xA0\x1B\x19\x90\x91\x16\x17\x85U[aa\xA8\x80\x84\x10\x90\x84\x14\x17`\x01\x80\x85\x11\x90\x85\x14\x17\x16a\x1D\x88W`@Qc9]8\x19`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1D\x91\x83a\x19-V[\x85Tc\xFF\xFF\xFF\xFF`\x80\x1B\x19\x16`\x01`\x80\x1Bc\xFF\xFF\xFF\xFF\x92\x83\x16\x02\x17\x86Ua\x1D\xC6\x90\x85\x90`\x01\x90`\x01`\x01`\x80\x1B\x03\x90a&\x97\x16V[a\x1D\xE3W`@Qc\xB2B\xE3A`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1D\xEC\x84a\x16gV[\x85T`\x01`\x01`\x80\x1B\x03\x91\x90\x91\x16\x7F\xFF\xFF\xFF\xFF\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x17`\x01`\xC0\x1BBc\xFF\xFF\xFF\xFF\x16\x02\x17\x90\x94UPPPPV[`\0\x82`\x80\x01Q\x15a\x1EMWPc\x01\xE1\x85Ya\x19'V[`\0a\x1EX\x84a&\xADV[c\xFF\xFF\xFF\xFF\x16\x80\x84\x03\x90\x84\x10\x02\x83\x03\x92\x90\x92\x03\x93\x92PPPV[`\0a\x07[\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\"\x8AV[`\0g\r\xE0\xB6\xB3\xA7d\0\0\x81\x81\x84\x10a\x1E\xA5W`\0\x92PPPa\x19'V[\x80\x84\x11a\x1E\xBEWP`\x01`\x01`\x80\x1B\x03\x91Pa\x19'\x90PV[`\x80\x85\x01Q`\0\x90a\x1E\xD4\x90c\x01\xE1\x85Ya\x1ErV[\x90P`\0a\x1E\xE1\x87a&\xE2V[\x90P`\0a\x1E\xFFa\x1E\xFA\x88g\r\xE0\xB6\xB3\xA7d\0\0a9\xB6V[a'7V[\x90P`\0a\x1F\r\x83\x83a:\x8DV[\x90P`\0g\x1B\xC1mgN\xC8\0\0\x85\x8B``\x01Q\x8C``\x01Qa\x1F/\x91\x90a:vV[a\x1F9\x91\x90a:vV[a\x1FC\x91\x90a:\xD3V[\x90P`\0a\x1Flg\r\xE0\xB6\xB3\xA7d\0\0a\x1F]\x84\x86a:VV[a\x1Fg\x91\x90a:\xE7V[a'\xD4V[\x90Pa\x1F\x85\x8B`@\x01Q\x82a \xDB\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x9B\x9APPPPPPPPPPPV[\x83Q` \x85\x01Q`\0\x91`\x01`\x01`\x80\x1B\x03\x90\x81\x16\x91\x16\x84\x15a\x1F\xF3W`@\x87\x01Qa\x1F\xCA\x90\x83\x90`\x01`\x01`\x80\x1B\x03\x16a\x1ErV[\x91Pa\x1F\xEC\x87`@\x01Q`\x01`\x01`\x80\x1B\x03\x16\x82a)}\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa 1V[`@\x87\x01Qa \x0C\x90\x83\x90`\x01`\x01`\x80\x1B\x03\x16a)}V[\x91Pa .\x87`@\x01Q`\x01`\x01`\x80\x1B\x03\x16\x82a\x1Er\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P[`\0`@Q\x80`\xC0\x01`@R\x80\x84\x81R` \x01\x83\x81R` \x01\x88`\0\x01Q`\x01`\x01`\x80\x1B\x03\x16\x81R` \x01a \x7F\x89` \x01Qc\xFF\xFF\xFF\xFF\x16a'\x10g\r\xE0\xB6\xB3\xA7d\0\0\x91\x90\x91\x02\x04\x90V[\x81R` \x01a \x8E\x89\x88a\x1E6V[\x81R` \x01`\0\x81RP\x90Pa\x062\x81a$~V[`\0\x81`\x80\x01Q\x15a \xB7WP`\0a\x19'V[a \xC0\x82a&\xADV[c\xFF\xFF\xFF\xFF\x16\x83``\x01Qc\xFF\xFF\xFF\xFF\x16\x10\x15\x90P\x92\x91PPV[`\0a\x07[\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\"\x8AV[`\0a \xFD\x82`\x12a9\xB6V[a\x19'\x90`\na;\xF9V[`\0\x80\x80g\r\xE0\xB6\xB3\xA7d\0\0\x91P`\0\x80a!(\x86`@\x01Q\x90`\0\x90V[\x91P\x91P\x83\x86`\0\x01Q\x10a!@W\x95\x94PPPPPV[\x85Q\x83\x10a!QWP\x94\x93PPPPV[`\0a!\\\x87a&\xE2V[\x87Q\x90\x91P`\0\x90a!v\x90g\r\xE0\xB6\xB3\xA7d\0\0a9\xB6V[\x90P`\0a!\x83\x82a'7V[\x90P`\0\x89`\xA0\x01Q\x84\x83a!\x98\x91\x90a:VV[a!\xA2\x91\x90a<\x05V[\x90Pa!\xBB\x8A`@\x01Qa!\xB5\x83a)\x92V[\x90a \xDBV[\x9A\x99PPPPPPPPPPV[`\0\x80\x80g\r\xE0\xB6\xB3\xA7d\0\0\x91P`\0\x80a!\xE9\x86`@\x01Q\x90`\0\x90V[\x91P\x91P\x81\x86` \x01Q\x10a\"\x02WP\x90\x94\x93PPPPV[\x80\x86` \x01Q\x11a\"\x17WP\x91\x94\x93PPPPV[`\0a\"\"\x87a&\xE2V[\x90P`\0a\"A\x88`@\x01Q\x89` \x01Qa)}\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\"N\x82a'7V[\x90P`\0\x89`\xA0\x01Q\x84\x83a\"c\x91\x90a<\x05V[a\"m\x91\x90a:VV[\x90Pa\"x\x81a)\x92V[a!\xBB\x90g\r\xE0\xB6\xB3\xA7d\0\0a9\xB6V[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\"\xA2W`\0\x80\xFD[\x04\x92\x91PPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\"\xC1W`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[`\0\x80\x83\x80` \x01\x90Q\x81\x01\x90a\"\xEE\x91\x90a<-V[\x83\x81R`\xA0\x81\x01Q\x90\x91Pa#\x05\x90`\x01\x90a<\x05V[a#\x0E\x82a$~V[a#\x18\x91\x90a:VV[\x94\x93PPPPV[`\0\x80\x83\x80` \x01\x90Q\x81\x01\x90a#7\x91\x90a<-V[` \x81\x01\x84\x90R`\xA0\x81\x01Q\x90\x91Pa#\x05\x90`\x01\x90a<\x05V[`\0\x84\x86\x11\x15a#\x84W`@Qc0\x82\xDF\xDB`\xE1\x1B\x81R`\x04\x81\x01\x87\x90R`$\x81\x01\x86\x90R`D\x01[`@Q\x80\x91\x03\x90\xFD[`\0a#\x94\x88\x88\x85c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a#\xA6\x89\x88\x86c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a#\xB4\x82\x84a:\x8DV[\x13\x15a#\xDDW`@Qc\x06\xF1\xBE]`\xE2\x1B\x81R`\x04\x81\x01\x83\x90R`$\x81\x01\x82\x90R`D\x01a#{V[`\0a#\xE9\x89\x89a9\xB6V[\x90P`\0[`\x02a#\xFA\x8A\x8Ca<\xA6V[a$\x04\x91\x90a:\xD3V[\x94P`\0a$\x16\x8C\x87\x89c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a$$\x86\x83a:\x8DV[\x13a$1W\x85\x99Pa$8V[\x85\x9AP\x80\x94P[a$B\x8B\x8Ba9\xB6V[\x92PP`\x01\x01\x87\x82\x11\x80\x15a$VWP\x86\x81\x10[a#\xEEWPPPP\x96\x95PPPPPPV[`\0a\x07[\x82\x84``\x01Qc\xFF\xFF\xFF\xFF\x16a\x1E6V[`\0\x80a$\x8A\x83a&\xE2V[\x90P`\0\x80g\r\xE0\xB6\xB3\xA7d\0\0\x91P`\0\x82\x86`\0\x01Q\x10a$\xAFWP`\x01a$\xE8V[\x85Q\x82\x10a$\xD1Wa$\xCA`\x01g\r\xE0\xB6\xB3\xA7d\0\0a9\xB6V[\x90Pa$\xE8V[\x85Qa$\xE5\x90g\r\xE0\xB6\xB3\xA7d\0\0a9\xB6V[\x90P[`\0a%\x05\x87`@\x01Q\x88` \x01Qa\x1Er\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0\x81\x10a%0Wa%)`\x01g\r\xE0\xB6\xB3\xA7d\0\0a9\xB6V[\x90Pa%<V[`\0\x81\x11a%<WP`\x01[`\0a%G\x83a'7V[\x90P`\0a%T\x83a'7V[\x90P\x86a%a\x83\x83a:VV[a%k\x91\x90a<\x05V[\x99\x98PPPPPPPPPV[`\0\x80a%\x92\x84`@\x01Q\x84a\x1Er\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x80\x15a&\\W`\0a%\xA5\x82a)\xFBV[\x90P`\0a%\xC4c\x01\xE1\x85Y\x87`\x80\x01Qa\x1Er\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[``\x87\x01Q\x90\x91P`\0\x90g\x1B\xC1mgN\xC8\0\0\x90a%\xE3\x90\x80a:vV[a%\xED\x91\x90a:\xD3V[\x90P`\0a%\xFA\x88a&\xE2V[\x90P`\0a&\x10g\r\xE0\xB6\xB3\xA7d\0\0\x86a:\x8DV[\x90Pa&\x1C\x84\x84a:\x8DV[a&&\x90\x82a<\x05V[\x90Pa&2\x82\x82a:\xE7V[\x90P`\0a&?\x82a)\x92V[\x90Pa&S\x81g\r\xE0\xB6\xB3\xA7d\0\0a:VV[\x97PPPPPPP[P\x92\x91PPV[`\0\x80\x80\x80a&s\x89\x87\x87a+\xD6V[\x90\x94P\x92Pa&\x85\x89\x89\x89\x87\x87a,\x1BV[\x94\x9A\x93\x99P\x97P\x92\x95P\x90\x93PPPPV[\x81\x83\x14\x91\x83\x11\x91\x90\x91\x17\x82\x82\x14\x91\x90\x92\x10\x17\x16\x90V[`\0\x81`\x80\x01Q\x15a&\xD2W`@Qc\xB0\x19\x84\x97`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P`@\x81\x01Q``\x90\x91\x01Q\x01\x90V[`\0\x80a'\0c\x01\xE1\x85Y\x84`\x80\x01Qa\x1Er\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0c;\x9A\xCA\0a'\x12\x83a-SV[a'\x1C\x91\x90a:vV[\x90P`\0a\x10\x81\x82\x86``\x01Qa \xDB\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[`\0g\x06\xF0[Y\xD3\xB2\0\0\x82\x03a'PWP`\0\x91\x90PV[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x12a'xW`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a'\x99W`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a'\xA6\x83`\x02a:\x8DV[\x90P`\0a'\xB3\x82a-\xF7V[\x90P`\0a'\xC9g\x13\xA0K\xBD\xFD\xC9\xBE\x88\x83a0uV[\x90Pa\x10\x81\x81a<\xB9V[`\0h\x02H\xCE6\xA7\x0C\xB2k>\x19\x82\x13a'\xEFWP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a(6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x01a#{V[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05`\x01`_\x1B\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1Dm\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1Dn\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[`\0a\x07[\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\"\xA9V[`\0\x80g\x13\xA0K\xBD\xFD\xC9\xBE\x88a)\xB0g\r\xE0\xB6\xB3\xA7d\0\0\x85a:\x8DV[a)\xBA\x91\x90a:\xE7V[\x90P`\0a)\xC7\x82a<\xB9V[\x90P`\0a)\xD4\x82a0\x8AV[\x90Pg\x1B\xC1mgN\xC8\0\0a)\xF1g\r\xE0\xB6\xB3\xA7d\0\0\x83a:\x8DV[a\x10\x81\x91\x90a:\xE7V[`\0\x80\x82\x13a*8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a#{V[`\0``a*E\x84a2nV[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[\x82Q`\0\x90\x81\x90a+\xF3\x90`\x01`\x01`\x80\x1B\x03\x16\x85a'\x10a\"\xA9V[\x91P\x82\x15a\x12\x1DWa,\x05\x83\x83a:\xD3V[\x90Pa,\x11\x81\x83a9\xB6V[\x91P\x93P\x93\x91PPV[`\0\x80`\0\x87`\x80\x01Qa,/W\x85a,1V[\x86[\x90P`\0\x88`\x80\x01Qa,DW\x87a,FV[\x86[\x89Q\x90\x91Pa,^\x90`\x01`\x01`\x80\x1B\x03\x16\x83a<\xA6V[\x91Pa,j\x86\x83a9\xB6V[\x91P\x81\x85\x11\x15a,\x8DW`@QcvG\x0F\xE7`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a,\x97\x85\x83a9\xB6V[\x91P\x80\x89` \x01Q`\x01`\x01`\x80\x1B\x03\x16\x11\x15a,\xC7W`@Qc\x86j\x03+`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x89\x01Qa,\xDF\x90`\x01`\x01`\x80\x1B\x03\x16\x82a9\xB6V[\x90P\x88`\x80\x01Qa,\xF0W\x80a,\xF2V[\x81[\x93P\x88`\x80\x01Qa-\x03W\x81a-\x05V[\x80[\x92P\x83\x88\x03a-'W`@Qc9;xE`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82\x87\x03a-GW`@Qc\x1F\xB0\xB7\xDD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PP\x95P\x95\x93PPPPV[`\xB5\x81`\x01`\x88\x1B\x81\x10a-lW`@\x91\x90\x91\x1B\x90`\x80\x1C[i\x01\0\0\0\0\0\0\0\0\0\x81\x10a-\x88W` \x91\x90\x91\x1B\x90`@\x1C[e\x01\0\0\0\0\0\x81\x10a-\xA0W`\x10\x91\x90\x91\x1B\x90` \x1C[c\x01\0\0\0\x81\x10a-\xB6W`\x08\x91\x90\x91\x1B\x90`\x10\x1C[b\x01\0\0\x01\x02`\x12\x1C\x80\x82\x04\x01`\x01\x90\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x90\x81\x90\x04\x81\x11\x90\x03\x90V[`\0\x80\x82\x12\x80a.\x0EWPg\x1B\xC1mgN\xC8\0\0\x82\x13[\x15a.,W`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a.MW`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81g\x1B\xC1mgN\xC8\0\0\x03a.uW`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x15a.\x80W\x91\x90PV[`\0g\r\xE0\xB6\xB3\xA7d\0\0\x83\x12a.\xA8Wa.\xA3\x83g\x1B\xC1mgN\xC8\0\0a:VV[a.\xAAV[\x82[\x90P`\0a.\xC0\x82g\x1B\xC1mgN\xC8\0\0a3\x0CV[\x90P\x80`\0\x03a.\xE3W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a.\xEE\x82a)\xFBV[\x90P`\0c;\x9A\xCA\0a/\x19a/\x14a/\x0Eg\x1B\xC1mgN\xC8\0\0a<\xB9V[\x85a0uV[a-SV[a/#\x91\x90a:\x8DV[\x90P`\0\x80a/:\x83g\x03\xC1f\\z\xAB \0a0uV[a/L\x90g \x05\xFEO&\x8E\xA0\0a<\x05V[\x90P`\0a/|\x84a/e\x86f\x9F2u$b\xA0\0a0uV[a/w\x90g\r\xC5R\x7Fd, \0a<\x05V[a0uV[a/\x8E\x90g\r\xE0\xB6\xB3\xA7d\0\0a<\x05V[\x90Pa/\xB2g\t\xD0(\xCCo _\xFF\x19\x85a/\xA8\x85\x85a3\x0CV[a/w\x91\x90a:VV[\x92PPP`\0[`\x02\x81\x10\x15a0MW`\0\x86a/\xCE\x84a0\x8AV[a/\xD8\x91\x90a:VV[\x90P`\0a/\xE6\x84\x85a0uV[a/\xEF\x90a<\xB9V[\x90P`\0a/\xFC\x82a'\xD4V[\x90P`\0a0\n\x86\x85a0uV[a0\x1Cg\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x84a0uV[a0&\x91\x90a:VV[\x90Pa02\x84\x82a3\x0CV[a0<\x90\x87a<\x05V[\x95P\x84`\x01\x01\x94PPPPPa/\xB9V[g\r\xE0\xB6\xB3\xA7d\0\0\x88\x12a0jWa0e\x82a<\xB9V[a\x062V[P\x96\x95PPPPPPV[`\0a\x07[\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a3\x1DV[`\0\x81`\0\x03a0\xA3WPg\r\xE0\xB6\xB3\xA7d\0\0\x91\x90PV[gV\x98\xEE\xF0fp\0\0\x82\x12a0\xBAWP`\0\x91\x90PV[a0\xCBgV\x98\xEE\xF0fp\0\0a<\xB9V[\x82\x13a0\xE0WPg\x1B\xC1mgN\xC8\0\0\x91\x90PV[`\0a0\xEB\x83a3<V[\x90P`\0a1$g\r\xE0\xB6\xB3\xA7d\0\0a1\r\x84g\x1B\xC1mgN\xC8\0\0a\x1ErV[a1\x1F\x90g\r\xE0\xB6\xB3\xA7d\0\0a<\x05V[a3\x0CV[\x90P`\0\x80\x82a1\x80\x81a1m\x81a1[\x81a1H\x81g\x02_\x0F\xE1\x05\xA3\x14\0a0uV[a/w\x90g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19a<\x05V[a/w\x90g\x14\xA8EL\x19\xE1\xAC\0a<\x05V[a/w\x90g\x0F\xC1\x0E\x01W\x82w\xFF\x19a<\x05V[a1\x92\x90g\x03\xDE\xBD\x08;\x8C|\0a<\x05V[\x91P\x83\x90Pa1\xFA\x81a1\xE8\x81a1\xD6\x81a1\xC4\x81a1\xB1\x81\x8Ba0uV[a/w\x90g\x02\x95\xD4\0\xEA2W\xFF\x19a<\x05V[a/w\x90g\x01W\xD8\xB2\xEC\xC7\x08\0a<\x05V[a/w\x90g\x051\n\xA7\xD5!0\0a<\x05V[a/w\x90g\r\xE0\xCC=\x15a\0\0a<\x05V[\x91P\x81g\x11\x90\0\xAB\x10\x0F\xFC\0a2\x10\x87\x88a0uV[a2\x1C\x90`\0\x19a:\x8DV[a2&\x91\x90a:VV[a20\x91\x90a<\x05V[\x92PP`\0a2>\x83a'\xD4V[\x90P`\0a2L\x85\x83a0uV[\x90P`\0\x88\x12a2\\W\x80a\x062V[a\x062\x81g\x1B\xC1mgN\xC8\0\0a:VV[`\0\x80\x82\x11a2\xABW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a#{V[P`\x01`\x01`\x01`\x80\x1B\x03\x82\x11`\x07\x1B\x82\x81\x1C`\x01`\x01`@\x1B\x03\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[`\0a\x07[\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x05\x85\x14\x17\x16a35W`\0\x80\xFD[\x05\x92\x91PPV[`\0`\x01`\xFF\x1B\x82\x03a3bW`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x12\x15a\x16yWP\x19`\x01\x01\x90V[\x91\x90PV[`@Q\x80a\x01@\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\0`\xFF\x16\x81R` \x01`\0`\xFF\x16\x81RP\x90V[\x805`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a3sW`\0\x80\xFD[\x80\x15\x15\x81\x14a4\x08W`\0\x80\xFD[PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a4\x08W`\0\x80\xFD[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a46W`\0\x80\xFD[a4?\x85a3\xE3V[\x93P` \x85\x015a4O\x81a3\xFAV[\x92P`@\x85\x015\x91P``\x85\x015a4f\x81a4\x0BV[\x93\x96\x92\x95P\x90\x93PPV[`\0` \x82\x84\x03\x12\x15a4\x83W`\0\x80\xFD[a\x07[\x82a3\xE3V[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a4\xA4W`\0\x80\xFD[\x855\x94P` \x86\x015\x93P`@\x86\x015\x92P``\x86\x015a4\xC4\x81a3\xFAV[\x94\x97\x93\x96P\x91\x94`\x80\x015\x92\x91PPV[``\x81R`\0\x84Q\x80``\x84\x01R`\0[\x81\x81\x10\x15a5\x03W` \x81\x88\x01\x81\x01Q`\x80\x86\x84\x01\x01R\x01a4\xE6V[P`\0`\x80\x82\x85\x01\x01R`\x80`\x1F\x19`\x1F\x83\x01\x16\x84\x01\x01\x91PP\x83` \x83\x01R\x82`@\x83\x01R\x94\x93PPPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\xA0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a5iWa5ia51V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a5\x97Wa5\x97a51V[`@R\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a5\xB2W`\0\x80\xFD[\x825\x91P` \x80\x84\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a5\xD1W`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a5\xE5W`\0\x80\xFD[\x815\x81\x81\x11\x15a5\xF7Wa5\xF7a51V[a6\t`\x1F\x82\x01`\x1F\x19\x16\x85\x01a5oV[\x91P\x80\x82R\x87\x84\x82\x85\x01\x01\x11\x15a6\x1FW`\0\x80\xFD[\x80\x84\x84\x01\x85\x84\x017`\0\x84\x82\x84\x01\x01RP\x80\x93PPPP\x92P\x92\x90PV[`\x01`\x01`\x80\x1B\x03\x81\x16\x81\x14a4\x08W`\0\x80\xFD[`\0\x80`\0\x83\x85\x03`\xE0\x81\x12\x15a6hW`\0\x80\xFD[`\xA0\x81\x12\x15a6vW`\0\x80\xFD[Pa6\x7Fa5GV[\x845a6\x8A\x81a6=V[\x81R` \x85\x015a6\x9A\x81a6=V[` \x82\x01R`@\x85\x015a6\xAD\x81a3\xFAV[`@\x82\x01Ra6\xBE``\x86\x01a3\xE3V[``\x82\x01R`\x80\x85\x015a6\xD1\x81a3\xFAV[`\x80\x82\x01R\x92P`\xA0\x84\x015\x91P`\xC0\x84\x015a6\xED\x81a4\x0BV[\x80\x91PP\x92P\x92P\x92V[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a7\x0EW`\0\x80\xFD[a7\x17\x85a3\xE3V[\x96` \x86\x015\x96P`@\x86\x015\x95``\x015\x94P\x92PPPV[`\0\x80`\0`@\x84\x86\x03\x12\x15a7FW`\0\x80\xFD[a7O\x84a3\xE3V[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a7kW`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a7\x7FW`\0\x80\xFD[\x815\x81\x81\x11\x15a7\x8EW`\0\x80\xFD[\x87` \x82\x85\x01\x01\x11\x15a7\xA0W`\0\x80\xFD[` \x83\x01\x94P\x80\x93PPPP\x92P\x92P\x92V[`\0\x80`\0``\x84\x86\x03\x12\x15a7\xC8W`\0\x80\xFD[a7\xD1\x84a3\xE3V[\x92P` \x84\x015a7\xE1\x81a3\xFAV[\x91P`@\x84\x015a6\xED\x81a4\x0BV[\x80Qa3s\x81a6=V[\x80Qc\xFF\xFF\xFF\xFF\x81\x16\x81\x14a3sW`\0\x80\xFD[\x80Qa\xFF\xFF\x81\x16\x81\x14a3sW`\0\x80\xFD[\x80Qa3s\x81a4\x0BV[`\0a\x01\0\x80\x83\x85\x03\x12\x15a8AW`\0\x80\xFD[`@Q\x90\x81\x01\x90`\x01`\x01`@\x1B\x03\x82\x11\x81\x83\x10\x17\x15a8cWa8ca51V[\x81`@R\x83Q\x91Pa8t\x82a6=V[\x81\x81Ra8\x83` \x85\x01a7\xF1V[` \x82\x01Ra8\x94`@\x85\x01a7\xF1V[`@\x82\x01Ra8\xA5``\x85\x01a7\xFCV[``\x82\x01Ra8\xB6`\x80\x85\x01a8\x10V[`\x80\x82\x01Ra8\xC7`\xA0\x85\x01a8\x10V[`\xA0\x82\x01Ra8\xD8`\xC0\x85\x01a8\"V[`\xC0\x82\x01Ra8\xE9`\xE0\x85\x01a8\"V[`\xE0\x82\x01R\x94\x93PPPPV[\x80Q`\xFF\x81\x16\x81\x14a3sW`\0\x80\xFD[`\0`\x80\x82\x84\x03\x12\x15a9\x19W`\0\x80\xFD[`@Q`\x80\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a9;Wa9;a51V[`@R\x82Qa9I\x81a4\x0BV[\x81Ra9W` \x84\x01a8\xF6V[` \x82\x01R`@\x83\x01Qa9j\x81a4\x0BV[`@\x82\x01Ra9{``\x84\x01a8\xF6V[``\x82\x01R\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a9\x99W`\0\x80\xFD[PQ\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x19'Wa\x19'a9\xA0V[`\0`\x01\x82\x01a9\xDBWa9\xDBa9\xA0V[P`\x01\x01\x90V[`\0`\xA0\x82\x84\x03\x12\x15a9\xF4W`\0\x80\xFD[a9\xFCa5GV[\x82Qa:\x07\x81a6=V[\x81Ra:\x15` \x84\x01a7\xFCV[` \x82\x01Ra:&`@\x84\x01a7\xFCV[`@\x82\x01Ra:7``\x84\x01a7\xFCV[``\x82\x01R`\x80\x83\x01Qa:J\x81a3\xFAV[`\x80\x82\x01R\x93\x92PPPV[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a&\\Wa&\\a9\xA0V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x19'Wa\x19'a9\xA0V[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15a:\xA9Wa:\xA9a9\xA0V[\x81\x81\x05\x83\x14\x82\x15\x17a\x19'Wa\x19'a9\xA0V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a:\xE2Wa:\xE2a:\xBDV[P\x04\x90V[`\0\x82a:\xF6Wa:\xF6a:\xBDV[`\x01`\xFF\x1B\x82\x14`\0\x19\x84\x14\x16\x15a;\x10Wa;\x10a9\xA0V[P\x05\x90V[`\x01\x81\x81[\x80\x85\x11\x15a;PW\x81`\0\x19\x04\x82\x11\x15a;6Wa;6a9\xA0V[\x80\x85\x16\x15a;CW\x91\x81\x02\x91[\x93\x84\x1C\x93\x90\x80\x02\x90a;\x1AV[P\x92P\x92\x90PV[`\0\x82a;gWP`\x01a\x19'V[\x81a;tWP`\0a\x19'V[\x81`\x01\x81\x14a;\x8AW`\x02\x81\x14a;\x94Wa;\xB0V[`\x01\x91PPa\x19'V[`\xFF\x84\x11\x15a;\xA5Wa;\xA5a9\xA0V[PP`\x01\x82\x1Ba\x19'V[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a;\xD3WP\x81\x81\na\x19'V[a;\xDD\x83\x83a;\x15V[\x80`\0\x19\x04\x82\x11\x15a;\xF1Wa;\xF1a9\xA0V[\x02\x93\x92PPPV[`\0a\x07[\x83\x83a;XV[\x80\x82\x01\x82\x81\x12`\0\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a<%Wa<%a9\xA0V[PP\x92\x91PPV[`\0`\xC0\x82\x84\x03\x12\x15a<?W`\0\x80\xFD[`@Q`\xC0\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a<aWa<aa51V[\x80`@RP\x82Q\x81R` \x83\x01Q` \x82\x01R`@\x83\x01Q`@\x82\x01R``\x83\x01Q``\x82\x01R`\x80\x83\x01Q`\x80\x82\x01R`\xA0\x83\x01Q`\xA0\x82\x01R\x80\x91PP\x92\x91PPV[\x80\x82\x01\x80\x82\x11\x15a\x19'Wa\x19'a9\xA0V[`\0`\x01`\xFF\x1B\x82\x01a<\xCEWa<\xCEa9\xA0V[P`\0\x03\x90V\xFE\xA2dipfsX\"\x12 \x8F\xC4j!\x94\x95\x99E\x9A\x0F\xD1\xE8\xEB9\xB2\xA9\xCA\xD1\x1BJ\xDE\xBDK\x16\x9A,\x86\x88\xAF$\x0B-dsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static NORMALSTRATEGY_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct NormalStrategy<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for NormalStrategy<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for NormalStrategy<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for NormalStrategy<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for NormalStrategy<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(NormalStrategy))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> NormalStrategy<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    NORMALSTRATEGY_ABI.clone(),
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
                NORMALSTRATEGY_ABI.clone(),
                NORMALSTRATEGY_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `afterCreate` (0xe068787f) function
        pub fn after_create(
            &self,
            pool_id: u64,
            strategy_args: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([224, 104, 120, 127], (pool_id, strategy_args))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `approximateReservesGivenPrice` (0x4bf346bf) function
        pub fn approximate_reserves_given_price(
            &self,
            price_wad: ::ethers::core::types::U256,
            strategy_args: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([75, 243, 70, 191], (price_wad, strategy_args))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `beforeSwap` (0xec736854) function
        pub fn before_swap(
            &self,
            pool_id: u64,
            sell_asset: bool,
            swapper: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (bool, ::ethers::core::types::I256),
        > {
            self.0
                .method_hash([236, 115, 104, 84], (pool_id, sell_asset, swapper))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `configs` (0x34dbc73b) function
        pub fn configs(
            &self,
            pool_id: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, (u128, u32, u32, u32, bool)> {
            self.0
                .method_hash([52, 219, 199, 59], pool_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getAmountOut` (0x19057807) function
        pub fn get_amount_out(
            &self,
            pool_id: u64,
            sell_asset: bool,
            amount_in: ::ethers::core::types::U256,
            swapper: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([25, 5, 120, 7], (pool_id, sell_asset, amount_in, swapper))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getInvariant` (0x39434d5a) function
        pub fn get_invariant(
            &self,
            pool_id: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([57, 67, 77, 90], pool_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getMaxOrder` (0xf07b879e) function
        pub fn get_max_order(
            &self,
            pool_id: u64,
            sell_asset: bool,
            swapper: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, Order> {
            self.0
                .method_hash([240, 123, 135, 158], (pool_id, sell_asset, swapper))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getSpotPrice` (0xe331ba34) function
        pub fn get_spot_price(
            &self,
            pool_id: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([227, 49, 186, 52], pool_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getStrategyData` (0x452d2f18) function
        pub fn get_strategy_data(
            &self,
            strike_price_wad: ::ethers::core::types::U256,
            volatility_basis_points: ::ethers::core::types::U256,
            duration_seconds: ::ethers::core::types::U256,
            is_perpetual: bool,
            price_wad: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::Bytes,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash(
                    [69, 45, 47, 24],
                    (
                        strike_price_wad,
                        volatility_basis_points,
                        duration_seconds,
                        is_perpetual,
                        price_wad,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `portfolio` (0x16ede016) function
        pub fn portfolio(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([22, 237, 224, 22], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `simulateSwap` (0x80af9d76) function
        pub fn simulate_swap(
            &self,
            order: Order,
            timestamp: ::ethers::core::types::U256,
            swapper: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (bool, ::ethers::core::types::I256, ::ethers::core::types::I256),
        > {
            self.0
                .method_hash([128, 175, 157, 118], (order, timestamp, swapper))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `validatePool` (0xe6047b19) function
        pub fn validate_pool(
            &self,
            pool_id: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([230, 4, 123, 25], pool_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `validateSwap` (0xa4478919) function
        pub fn validate_swap(
            &self,
            pool_id: u64,
            invariant: ::ethers::core::types::I256,
            reserve_x: ::ethers::core::types::U256,
            reserve_y: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (bool, ::ethers::core::types::I256),
        > {
            self.0
                .method_hash(
                    [164, 71, 137, 25],
                    (pool_id, invariant, reserve_x, reserve_y),
                )
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `AfterCreate` event
        pub fn after_create_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AfterCreateFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Genesis` event
        pub fn genesis_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, GenesisFilter> {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            NormalStrategyEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for NormalStrategy<M> {
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
    ///Custom Error type `NormalStrategyLib_ConfigExists` with signature `NormalStrategyLib_ConfigExists()` and selector `0x784e2684`
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
        name = "NormalStrategyLib_ConfigExists",
        abi = "NormalStrategyLib_ConfigExists()"
    )]
    pub struct NormalStrategyLib_ConfigExists;
    ///Custom Error type `NormalStrategyLib_InvalidDuration` with signature `NormalStrategyLib_InvalidDuration()` and selector `0xb597030f`
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
        name = "NormalStrategyLib_InvalidDuration",
        abi = "NormalStrategyLib_InvalidDuration()"
    )]
    pub struct NormalStrategyLib_InvalidDuration;
    ///Custom Error type `NormalStrategyLib_InvalidStrategyArgs` with signature `NormalStrategyLib_InvalidStrategyArgs()` and selector `0x05655f4c`
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
        name = "NormalStrategyLib_InvalidStrategyArgs",
        abi = "NormalStrategyLib_InvalidStrategyArgs()"
    )]
    pub struct NormalStrategyLib_InvalidStrategyArgs;
    ///Custom Error type `NormalStrategyLib_InvalidStrikePrice` with signature `NormalStrategyLib_InvalidStrikePrice()` and selector `0xb242e341`
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
        name = "NormalStrategyLib_InvalidStrikePrice",
        abi = "NormalStrategyLib_InvalidStrikePrice()"
    )]
    pub struct NormalStrategyLib_InvalidStrikePrice;
    ///Custom Error type `NormalStrategyLib_InvalidVolatility` with signature `NormalStrategyLib_InvalidVolatility()` and selector `0x395d3819`
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
        name = "NormalStrategyLib_InvalidVolatility",
        abi = "NormalStrategyLib_InvalidVolatility()"
    )]
    pub struct NormalStrategyLib_InvalidVolatility;
    ///Custom Error type `NormalStrategyLib_NonExpiringPool` with signature `NormalStrategyLib_NonExpiringPool()` and selector `0xb0198497`
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
        name = "NormalStrategyLib_NonExpiringPool",
        abi = "NormalStrategyLib_NonExpiringPool()"
    )]
    pub struct NormalStrategyLib_NonExpiringPool;
    ///Custom Error type `NormalStrategy_NotPortfolio` with signature `NormalStrategy_NotPortfolio()` and selector `0xe88c95ac`
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
        name = "NormalStrategy_NotPortfolio",
        abi = "NormalStrategy_NotPortfolio()"
    )]
    pub struct NormalStrategy_NotPortfolio;
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
    ///Custom Error type `SwapLib_OutputExceedsReserves` with signature `SwapLib_OutputExceedsReserves()` and selector `0x866a032b`
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
        name = "SwapLib_OutputExceedsReserves",
        abi = "SwapLib_OutputExceedsReserves()"
    )]
    pub struct SwapLib_OutputExceedsReserves;
    ///Custom Error type `SwapLib_ProtocolFeeTooHigh` with signature `SwapLib_ProtocolFeeTooHigh()` and selector `0xec8e1fce`
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
        name = "SwapLib_ProtocolFeeTooHigh",
        abi = "SwapLib_ProtocolFeeTooHigh()"
    )]
    pub struct SwapLib_ProtocolFeeTooHigh;
    ///Custom Error type `SwapLib_ZeroXAdjustment` with signature `SwapLib_ZeroXAdjustment()` and selector `0x7276f08a`
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
    #[etherror(name = "SwapLib_ZeroXAdjustment", abi = "SwapLib_ZeroXAdjustment()")]
    pub struct SwapLib_ZeroXAdjustment;
    ///Custom Error type `SwapLib_ZeroYAdjustment` with signature `SwapLib_ZeroYAdjustment()` and selector `0x1fb0b7dd`
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
    #[etherror(name = "SwapLib_ZeroYAdjustment", abi = "SwapLib_ZeroYAdjustment()")]
    pub struct SwapLib_ZeroYAdjustment;
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
    pub enum NormalStrategyErrors {
        BisectionLib_InvalidBounds(BisectionLib_InvalidBounds),
        BisectionLib_RootOutsideBounds(BisectionLib_RootOutsideBounds),
        Infinity(Infinity),
        Min(Min),
        NegativeInfinity(NegativeInfinity),
        NormalStrategyLib_ConfigExists(NormalStrategyLib_ConfigExists),
        NormalStrategyLib_InvalidDuration(NormalStrategyLib_InvalidDuration),
        NormalStrategyLib_InvalidStrategyArgs(NormalStrategyLib_InvalidStrategyArgs),
        NormalStrategyLib_InvalidStrikePrice(NormalStrategyLib_InvalidStrikePrice),
        NormalStrategyLib_InvalidVolatility(NormalStrategyLib_InvalidVolatility),
        NormalStrategyLib_NonExpiringPool(NormalStrategyLib_NonExpiringPool),
        NormalStrategy_NotPortfolio(NormalStrategy_NotPortfolio),
        OutOfBounds(OutOfBounds),
        SwapLib_OutputExceedsReserves(SwapLib_OutputExceedsReserves),
        SwapLib_ProtocolFeeTooHigh(SwapLib_ProtocolFeeTooHigh),
        SwapLib_ZeroXAdjustment(SwapLib_ZeroXAdjustment),
        SwapLib_ZeroYAdjustment(SwapLib_ZeroYAdjustment),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for NormalStrategyErrors {
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
            if let Ok(decoded) = <NormalStrategyLib_ConfigExists as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NormalStrategyLib_ConfigExists(decoded));
            }
            if let Ok(decoded) = <NormalStrategyLib_InvalidDuration as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NormalStrategyLib_InvalidDuration(decoded));
            }
            if let Ok(decoded) = <NormalStrategyLib_InvalidStrategyArgs as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NormalStrategyLib_InvalidStrategyArgs(decoded));
            }
            if let Ok(decoded) = <NormalStrategyLib_InvalidStrikePrice as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NormalStrategyLib_InvalidStrikePrice(decoded));
            }
            if let Ok(decoded) = <NormalStrategyLib_InvalidVolatility as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NormalStrategyLib_InvalidVolatility(decoded));
            }
            if let Ok(decoded) = <NormalStrategyLib_NonExpiringPool as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NormalStrategyLib_NonExpiringPool(decoded));
            }
            if let Ok(decoded) = <NormalStrategy_NotPortfolio as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NormalStrategy_NotPortfolio(decoded));
            }
            if let Ok(decoded) = <OutOfBounds as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OutOfBounds(decoded));
            }
            if let Ok(decoded) = <SwapLib_OutputExceedsReserves as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SwapLib_OutputExceedsReserves(decoded));
            }
            if let Ok(decoded) = <SwapLib_ProtocolFeeTooHigh as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SwapLib_ProtocolFeeTooHigh(decoded));
            }
            if let Ok(decoded) = <SwapLib_ZeroXAdjustment as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SwapLib_ZeroXAdjustment(decoded));
            }
            if let Ok(decoded) = <SwapLib_ZeroYAdjustment as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SwapLib_ZeroYAdjustment(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for NormalStrategyErrors {
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
                Self::NormalStrategyLib_ConfigExists(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NormalStrategyLib_InvalidDuration(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NormalStrategyLib_InvalidStrategyArgs(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NormalStrategyLib_InvalidStrikePrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NormalStrategyLib_InvalidVolatility(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NormalStrategyLib_NonExpiringPool(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NormalStrategy_NotPortfolio(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OutOfBounds(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SwapLib_OutputExceedsReserves(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SwapLib_ProtocolFeeTooHigh(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SwapLib_ZeroXAdjustment(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SwapLib_ZeroYAdjustment(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for NormalStrategyErrors {
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
                    == <NormalStrategyLib_ConfigExists as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NormalStrategyLib_InvalidDuration as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NormalStrategyLib_InvalidStrategyArgs as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NormalStrategyLib_InvalidStrikePrice as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NormalStrategyLib_InvalidVolatility as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NormalStrategyLib_NonExpiringPool as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NormalStrategy_NotPortfolio as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OutOfBounds as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <SwapLib_OutputExceedsReserves as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <SwapLib_ProtocolFeeTooHigh as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <SwapLib_ZeroXAdjustment as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <SwapLib_ZeroYAdjustment as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for NormalStrategyErrors {
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
                Self::NormalStrategyLib_ConfigExists(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NormalStrategyLib_InvalidDuration(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NormalStrategyLib_InvalidStrategyArgs(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NormalStrategyLib_InvalidStrikePrice(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NormalStrategyLib_InvalidVolatility(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NormalStrategyLib_NonExpiringPool(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NormalStrategy_NotPortfolio(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OutOfBounds(element) => ::core::fmt::Display::fmt(element, f),
                Self::SwapLib_OutputExceedsReserves(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SwapLib_ProtocolFeeTooHigh(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SwapLib_ZeroXAdjustment(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SwapLib_ZeroYAdjustment(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for NormalStrategyErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<BisectionLib_InvalidBounds> for NormalStrategyErrors {
        fn from(value: BisectionLib_InvalidBounds) -> Self {
            Self::BisectionLib_InvalidBounds(value)
        }
    }
    impl ::core::convert::From<BisectionLib_RootOutsideBounds> for NormalStrategyErrors {
        fn from(value: BisectionLib_RootOutsideBounds) -> Self {
            Self::BisectionLib_RootOutsideBounds(value)
        }
    }
    impl ::core::convert::From<Infinity> for NormalStrategyErrors {
        fn from(value: Infinity) -> Self {
            Self::Infinity(value)
        }
    }
    impl ::core::convert::From<Min> for NormalStrategyErrors {
        fn from(value: Min) -> Self {
            Self::Min(value)
        }
    }
    impl ::core::convert::From<NegativeInfinity> for NormalStrategyErrors {
        fn from(value: NegativeInfinity) -> Self {
            Self::NegativeInfinity(value)
        }
    }
    impl ::core::convert::From<NormalStrategyLib_ConfigExists> for NormalStrategyErrors {
        fn from(value: NormalStrategyLib_ConfigExists) -> Self {
            Self::NormalStrategyLib_ConfigExists(value)
        }
    }
    impl ::core::convert::From<NormalStrategyLib_InvalidDuration>
    for NormalStrategyErrors {
        fn from(value: NormalStrategyLib_InvalidDuration) -> Self {
            Self::NormalStrategyLib_InvalidDuration(value)
        }
    }
    impl ::core::convert::From<NormalStrategyLib_InvalidStrategyArgs>
    for NormalStrategyErrors {
        fn from(value: NormalStrategyLib_InvalidStrategyArgs) -> Self {
            Self::NormalStrategyLib_InvalidStrategyArgs(value)
        }
    }
    impl ::core::convert::From<NormalStrategyLib_InvalidStrikePrice>
    for NormalStrategyErrors {
        fn from(value: NormalStrategyLib_InvalidStrikePrice) -> Self {
            Self::NormalStrategyLib_InvalidStrikePrice(value)
        }
    }
    impl ::core::convert::From<NormalStrategyLib_InvalidVolatility>
    for NormalStrategyErrors {
        fn from(value: NormalStrategyLib_InvalidVolatility) -> Self {
            Self::NormalStrategyLib_InvalidVolatility(value)
        }
    }
    impl ::core::convert::From<NormalStrategyLib_NonExpiringPool>
    for NormalStrategyErrors {
        fn from(value: NormalStrategyLib_NonExpiringPool) -> Self {
            Self::NormalStrategyLib_NonExpiringPool(value)
        }
    }
    impl ::core::convert::From<NormalStrategy_NotPortfolio> for NormalStrategyErrors {
        fn from(value: NormalStrategy_NotPortfolio) -> Self {
            Self::NormalStrategy_NotPortfolio(value)
        }
    }
    impl ::core::convert::From<OutOfBounds> for NormalStrategyErrors {
        fn from(value: OutOfBounds) -> Self {
            Self::OutOfBounds(value)
        }
    }
    impl ::core::convert::From<SwapLib_OutputExceedsReserves> for NormalStrategyErrors {
        fn from(value: SwapLib_OutputExceedsReserves) -> Self {
            Self::SwapLib_OutputExceedsReserves(value)
        }
    }
    impl ::core::convert::From<SwapLib_ProtocolFeeTooHigh> for NormalStrategyErrors {
        fn from(value: SwapLib_ProtocolFeeTooHigh) -> Self {
            Self::SwapLib_ProtocolFeeTooHigh(value)
        }
    }
    impl ::core::convert::From<SwapLib_ZeroXAdjustment> for NormalStrategyErrors {
        fn from(value: SwapLib_ZeroXAdjustment) -> Self {
            Self::SwapLib_ZeroXAdjustment(value)
        }
    }
    impl ::core::convert::From<SwapLib_ZeroYAdjustment> for NormalStrategyErrors {
        fn from(value: SwapLib_ZeroYAdjustment) -> Self {
            Self::SwapLib_ZeroYAdjustment(value)
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
        name = "AfterCreate",
        abi = "AfterCreate(address,uint64,uint256,uint256,uint256,bool)"
    )]
    pub struct AfterCreateFilter {
        #[ethevent(indexed)]
        pub portfolio: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub pool_id: u64,
        pub strike_price_wad: ::ethers::core::types::U256,
        pub volatility_basis_points: ::ethers::core::types::U256,
        pub duration_seconds: ::ethers::core::types::U256,
        pub is_perpetual: bool,
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
    #[ethevent(name = "Genesis", abi = "Genesis(address)")]
    pub struct GenesisFilter {
        #[ethevent(indexed)]
        pub portfolio: ::ethers::core::types::Address,
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
    pub enum NormalStrategyEvents {
        AfterCreateFilter(AfterCreateFilter),
        GenesisFilter(GenesisFilter),
    }
    impl ::ethers::contract::EthLogDecode for NormalStrategyEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AfterCreateFilter::decode_log(log) {
                return Ok(NormalStrategyEvents::AfterCreateFilter(decoded));
            }
            if let Ok(decoded) = GenesisFilter::decode_log(log) {
                return Ok(NormalStrategyEvents::GenesisFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for NormalStrategyEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AfterCreateFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::GenesisFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AfterCreateFilter> for NormalStrategyEvents {
        fn from(value: AfterCreateFilter) -> Self {
            Self::AfterCreateFilter(value)
        }
    }
    impl ::core::convert::From<GenesisFilter> for NormalStrategyEvents {
        fn from(value: GenesisFilter) -> Self {
            Self::GenesisFilter(value)
        }
    }
    ///Container type for all input parameters for the `afterCreate` function with signature `afterCreate(uint64,bytes)` and selector `0xe068787f`
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
    #[ethcall(name = "afterCreate", abi = "afterCreate(uint64,bytes)")]
    pub struct AfterCreateCall {
        pub pool_id: u64,
        pub strategy_args: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `approximateReservesGivenPrice` function with signature `approximateReservesGivenPrice(uint256,bytes)` and selector `0x4bf346bf`
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
        name = "approximateReservesGivenPrice",
        abi = "approximateReservesGivenPrice(uint256,bytes)"
    )]
    pub struct ApproximateReservesGivenPriceCall {
        pub price_wad: ::ethers::core::types::U256,
        pub strategy_args: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `beforeSwap` function with signature `beforeSwap(uint64,bool,address)` and selector `0xec736854`
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
    #[ethcall(name = "beforeSwap", abi = "beforeSwap(uint64,bool,address)")]
    pub struct BeforeSwapCall {
        pub pool_id: u64,
        pub sell_asset: bool,
        pub swapper: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `configs` function with signature `configs(uint64)` and selector `0x34dbc73b`
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
    #[ethcall(name = "configs", abi = "configs(uint64)")]
    pub struct ConfigsCall {
        pub pool_id: u64,
    }
    ///Container type for all input parameters for the `getAmountOut` function with signature `getAmountOut(uint64,bool,uint256,address)` and selector `0x19057807`
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
    #[ethcall(name = "getAmountOut", abi = "getAmountOut(uint64,bool,uint256,address)")]
    pub struct GetAmountOutCall {
        pub pool_id: u64,
        pub sell_asset: bool,
        pub amount_in: ::ethers::core::types::U256,
        pub swapper: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getInvariant` function with signature `getInvariant(uint64)` and selector `0x39434d5a`
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
    #[ethcall(name = "getInvariant", abi = "getInvariant(uint64)")]
    pub struct GetInvariantCall {
        pub pool_id: u64,
    }
    ///Container type for all input parameters for the `getMaxOrder` function with signature `getMaxOrder(uint64,bool,address)` and selector `0xf07b879e`
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
    #[ethcall(name = "getMaxOrder", abi = "getMaxOrder(uint64,bool,address)")]
    pub struct GetMaxOrderCall {
        pub pool_id: u64,
        pub sell_asset: bool,
        pub swapper: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getSpotPrice` function with signature `getSpotPrice(uint64)` and selector `0xe331ba34`
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
    #[ethcall(name = "getSpotPrice", abi = "getSpotPrice(uint64)")]
    pub struct GetSpotPriceCall {
        pub pool_id: u64,
    }
    ///Container type for all input parameters for the `getStrategyData` function with signature `getStrategyData(uint256,uint256,uint256,bool,uint256)` and selector `0x452d2f18`
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
        name = "getStrategyData",
        abi = "getStrategyData(uint256,uint256,uint256,bool,uint256)"
    )]
    pub struct GetStrategyDataCall {
        pub strike_price_wad: ::ethers::core::types::U256,
        pub volatility_basis_points: ::ethers::core::types::U256,
        pub duration_seconds: ::ethers::core::types::U256,
        pub is_perpetual: bool,
        pub price_wad: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `portfolio` function with signature `portfolio()` and selector `0x16ede016`
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
    #[ethcall(name = "portfolio", abi = "portfolio()")]
    pub struct PortfolioCall;
    ///Container type for all input parameters for the `simulateSwap` function with signature `simulateSwap((uint128,uint128,bool,uint64,bool),uint256,address)` and selector `0x80af9d76`
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
        name = "simulateSwap",
        abi = "simulateSwap((uint128,uint128,bool,uint64,bool),uint256,address)"
    )]
    pub struct SimulateSwapCall {
        pub order: Order,
        pub timestamp: ::ethers::core::types::U256,
        pub swapper: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `validatePool` function with signature `validatePool(uint64)` and selector `0xe6047b19`
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
    #[ethcall(name = "validatePool", abi = "validatePool(uint64)")]
    pub struct ValidatePoolCall {
        pub pool_id: u64,
    }
    ///Container type for all input parameters for the `validateSwap` function with signature `validateSwap(uint64,int256,uint256,uint256)` and selector `0xa4478919`
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
        name = "validateSwap",
        abi = "validateSwap(uint64,int256,uint256,uint256)"
    )]
    pub struct ValidateSwapCall {
        pub pool_id: u64,
        pub invariant: ::ethers::core::types::I256,
        pub reserve_x: ::ethers::core::types::U256,
        pub reserve_y: ::ethers::core::types::U256,
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
    pub enum NormalStrategyCalls {
        AfterCreate(AfterCreateCall),
        ApproximateReservesGivenPrice(ApproximateReservesGivenPriceCall),
        BeforeSwap(BeforeSwapCall),
        Configs(ConfigsCall),
        GetAmountOut(GetAmountOutCall),
        GetInvariant(GetInvariantCall),
        GetMaxOrder(GetMaxOrderCall),
        GetSpotPrice(GetSpotPriceCall),
        GetStrategyData(GetStrategyDataCall),
        Portfolio(PortfolioCall),
        SimulateSwap(SimulateSwapCall),
        ValidatePool(ValidatePoolCall),
        ValidateSwap(ValidateSwapCall),
    }
    impl ::ethers::core::abi::AbiDecode for NormalStrategyCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AfterCreateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AfterCreate(decoded));
            }
            if let Ok(decoded) = <ApproximateReservesGivenPriceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ApproximateReservesGivenPrice(decoded));
            }
            if let Ok(decoded) = <BeforeSwapCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BeforeSwap(decoded));
            }
            if let Ok(decoded) = <ConfigsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Configs(decoded));
            }
            if let Ok(decoded) = <GetAmountOutCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetAmountOut(decoded));
            }
            if let Ok(decoded) = <GetInvariantCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetInvariant(decoded));
            }
            if let Ok(decoded) = <GetMaxOrderCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetMaxOrder(decoded));
            }
            if let Ok(decoded) = <GetSpotPriceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetSpotPrice(decoded));
            }
            if let Ok(decoded) = <GetStrategyDataCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetStrategyData(decoded));
            }
            if let Ok(decoded) = <PortfolioCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Portfolio(decoded));
            }
            if let Ok(decoded) = <SimulateSwapCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SimulateSwap(decoded));
            }
            if let Ok(decoded) = <ValidatePoolCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ValidatePool(decoded));
            }
            if let Ok(decoded) = <ValidateSwapCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ValidateSwap(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for NormalStrategyCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AfterCreate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ApproximateReservesGivenPrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BeforeSwap(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Configs(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetAmountOut(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetInvariant(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetMaxOrder(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetSpotPrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetStrategyData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Portfolio(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SimulateSwap(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ValidatePool(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ValidateSwap(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for NormalStrategyCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AfterCreate(element) => ::core::fmt::Display::fmt(element, f),
                Self::ApproximateReservesGivenPrice(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BeforeSwap(element) => ::core::fmt::Display::fmt(element, f),
                Self::Configs(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetAmountOut(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetInvariant(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetMaxOrder(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSpotPrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetStrategyData(element) => ::core::fmt::Display::fmt(element, f),
                Self::Portfolio(element) => ::core::fmt::Display::fmt(element, f),
                Self::SimulateSwap(element) => ::core::fmt::Display::fmt(element, f),
                Self::ValidatePool(element) => ::core::fmt::Display::fmt(element, f),
                Self::ValidateSwap(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AfterCreateCall> for NormalStrategyCalls {
        fn from(value: AfterCreateCall) -> Self {
            Self::AfterCreate(value)
        }
    }
    impl ::core::convert::From<ApproximateReservesGivenPriceCall>
    for NormalStrategyCalls {
        fn from(value: ApproximateReservesGivenPriceCall) -> Self {
            Self::ApproximateReservesGivenPrice(value)
        }
    }
    impl ::core::convert::From<BeforeSwapCall> for NormalStrategyCalls {
        fn from(value: BeforeSwapCall) -> Self {
            Self::BeforeSwap(value)
        }
    }
    impl ::core::convert::From<ConfigsCall> for NormalStrategyCalls {
        fn from(value: ConfigsCall) -> Self {
            Self::Configs(value)
        }
    }
    impl ::core::convert::From<GetAmountOutCall> for NormalStrategyCalls {
        fn from(value: GetAmountOutCall) -> Self {
            Self::GetAmountOut(value)
        }
    }
    impl ::core::convert::From<GetInvariantCall> for NormalStrategyCalls {
        fn from(value: GetInvariantCall) -> Self {
            Self::GetInvariant(value)
        }
    }
    impl ::core::convert::From<GetMaxOrderCall> for NormalStrategyCalls {
        fn from(value: GetMaxOrderCall) -> Self {
            Self::GetMaxOrder(value)
        }
    }
    impl ::core::convert::From<GetSpotPriceCall> for NormalStrategyCalls {
        fn from(value: GetSpotPriceCall) -> Self {
            Self::GetSpotPrice(value)
        }
    }
    impl ::core::convert::From<GetStrategyDataCall> for NormalStrategyCalls {
        fn from(value: GetStrategyDataCall) -> Self {
            Self::GetStrategyData(value)
        }
    }
    impl ::core::convert::From<PortfolioCall> for NormalStrategyCalls {
        fn from(value: PortfolioCall) -> Self {
            Self::Portfolio(value)
        }
    }
    impl ::core::convert::From<SimulateSwapCall> for NormalStrategyCalls {
        fn from(value: SimulateSwapCall) -> Self {
            Self::SimulateSwap(value)
        }
    }
    impl ::core::convert::From<ValidatePoolCall> for NormalStrategyCalls {
        fn from(value: ValidatePoolCall) -> Self {
            Self::ValidatePool(value)
        }
    }
    impl ::core::convert::From<ValidateSwapCall> for NormalStrategyCalls {
        fn from(value: ValidateSwapCall) -> Self {
            Self::ValidateSwap(value)
        }
    }
    ///Container type for all return fields from the `afterCreate` function with signature `afterCreate(uint64,bytes)` and selector `0xe068787f`
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
    pub struct AfterCreateReturn {
        pub success: bool,
    }
    ///Container type for all return fields from the `approximateReservesGivenPrice` function with signature `approximateReservesGivenPrice(uint256,bytes)` and selector `0x4bf346bf`
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
    pub struct ApproximateReservesGivenPriceReturn(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `beforeSwap` function with signature `beforeSwap(uint64,bool,address)` and selector `0xec736854`
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
    pub struct BeforeSwapReturn(pub bool, pub ::ethers::core::types::I256);
    ///Container type for all return fields from the `configs` function with signature `configs(uint64)` and selector `0x34dbc73b`
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
    pub struct ConfigsReturn {
        pub strike_price_wad: u128,
        pub volatility_basis_points: u32,
        pub duration_seconds: u32,
        pub creation_timestamp: u32,
        pub is_perpetual: bool,
    }
    ///Container type for all return fields from the `getAmountOut` function with signature `getAmountOut(uint64,bool,uint256,address)` and selector `0x19057807`
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
    pub struct GetAmountOutReturn {
        pub output: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `getInvariant` function with signature `getInvariant(uint64)` and selector `0x39434d5a`
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
    pub struct GetInvariantReturn {
        pub invariant: ::ethers::core::types::I256,
    }
    ///Container type for all return fields from the `getMaxOrder` function with signature `getMaxOrder(uint64,bool,address)` and selector `0xf07b879e`
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
    pub struct GetMaxOrderReturn(pub Order);
    ///Container type for all return fields from the `getSpotPrice` function with signature `getSpotPrice(uint64)` and selector `0xe331ba34`
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
    pub struct GetSpotPriceReturn {
        pub price: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `getStrategyData` function with signature `getStrategyData(uint256,uint256,uint256,bool,uint256)` and selector `0x452d2f18`
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
    pub struct GetStrategyDataReturn {
        pub strategy_data: ::ethers::core::types::Bytes,
        pub initial_x: ::ethers::core::types::U256,
        pub initial_y: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `portfolio` function with signature `portfolio()` and selector `0x16ede016`
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
    pub struct PortfolioReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `simulateSwap` function with signature `simulateSwap((uint128,uint128,bool,uint64,bool),uint256,address)` and selector `0x80af9d76`
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
    pub struct SimulateSwapReturn {
        pub success: bool,
        pub prev_invariant: ::ethers::core::types::I256,
        pub post_invariant: ::ethers::core::types::I256,
    }
    ///Container type for all return fields from the `validatePool` function with signature `validatePool(uint64)` and selector `0xe6047b19`
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
    pub struct ValidatePoolReturn(pub bool);
    ///Container type for all return fields from the `validateSwap` function with signature `validateSwap(uint64,int256,uint256,uint256)` and selector `0xa4478919`
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
    pub struct ValidateSwapReturn(pub bool, pub ::ethers::core::types::I256);
}
