pub use rmm::*;
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
pub mod rmm {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("tokenX_"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("tokenY_"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("sigma_"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("strikePrice_"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("tau_"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("swapFee_"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("addLiquidity"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("addLiquidity"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("exactX"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
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
                                    name: ::std::borrow::ToOwned::to_owned("amountX"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountY"),
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
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("addLiquidityExactX"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("addLiquidityExactX"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountX"),
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
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("addLiquidityExactY"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("addLiquidityExactY"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountY"),
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
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("computeInitialPoolState"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "computeInitialPoolState",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountX"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("initialPrice"),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("initialLiquidity"),
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
                    ::std::borrow::ToOwned::to_owned("getAmountOut"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getAmountOut"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("swapDirection"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("nextLiquidity"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
                    ::std::borrow::ToOwned::to_owned("getDeltaL"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getDeltaL"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("swapDirection"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("nextLiquidity"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
                    ::std::borrow::ToOwned::to_owned("getLiquidity"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getLiquidity"),
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
                    ::std::borrow::ToOwned::to_owned("getNextLiquidity"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getNextLiquidity"),
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
                    ::std::borrow::ToOwned::to_owned("getPortfolioValue"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getPortfolioValue"),
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
                    ::std::borrow::ToOwned::to_owned("getReserveX"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getReserveX"),
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
                    ::std::borrow::ToOwned::to_owned("getReserveY"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getReserveY"),
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
                    ::std::borrow::ToOwned::to_owned("getSpotPrice"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getSpotPrice"),
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
                    ::std::borrow::ToOwned::to_owned("getSpotPriceFromY"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getSpotPriceFromY"),
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
                    ::std::borrow::ToOwned::to_owned("getStrategyData"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getStrategyData"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
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
                    ::std::borrow::ToOwned::to_owned("getSwapConstantGivenLiquidity"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getSwapConstantGivenLiquidity",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newLiquidity"),
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
                    ::std::borrow::ToOwned::to_owned("getSwapFee"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getSwapFee"),
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
                    ::std::borrow::ToOwned::to_owned("initExactTokensAndLiquidity"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "initExactTokensAndLiquidity",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountX"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountY"),
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
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("initExactX"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("initExactX"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountX"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
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
                    ::std::borrow::ToOwned::to_owned("initExactY"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("initExactY"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountY"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
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
                    ::std::borrow::ToOwned::to_owned("initPool"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("initPool"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("exactX"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountX"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountY"),
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
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("instantiate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("instantiate"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountX"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountY"),
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
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("logData"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("logData"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("removeLiquidityExactX"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "removeLiquidityExactX",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountX"),
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
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("removeLiquidityExactY"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "removeLiquidityExactY",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountY"),
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
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("reserveX"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("reserveX"),
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
                    ::std::borrow::ToOwned::to_owned("reserveY"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("reserveY"),
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
                    ::std::borrow::ToOwned::to_owned("setSwapFee"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setSwapFee"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newSwapFee"),
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
                    ::std::borrow::ToOwned::to_owned("swap"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("swap"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("swapDirection"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("nextLiquidity"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
                                    name: ::std::borrow::ToOwned::to_owned("amountOut"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("swapAmountIn"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("swapAmountIn"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("swapDirection"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("nextLiquidity"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
                                    name: ::std::borrow::ToOwned::to_owned("amountOut"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
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
                    ::std::borrow::ToOwned::to_owned("tokenX"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("tokenX"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract ERC20"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("tokenY"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("tokenY"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract ERC20"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("totalLiquidity"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("totalLiquidity"),
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
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("AddLiquidity"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("AddLiquidity"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("liquidity"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amountX"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amountY"),
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
                    ::std::borrow::ToOwned::to_owned("LogParameters"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("LogParameters"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sigma"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("strikePrice"),
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
                                    name: ::std::borrow::ToOwned::to_owned("blockTimestamp"),
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
                    ::std::borrow::ToOwned::to_owned("LogReserves"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("LogReserves"),
                            inputs: ::std::vec![
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
                                    name: ::std::borrow::ToOwned::to_owned("blockTimestamp"),
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
                    ::std::borrow::ToOwned::to_owned("RemoveLiquidity"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("RemoveLiquidity"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("liquidity"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amountX"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amountY"),
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
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("swapDirection"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("input"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("output"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newPrice"),
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
                    ::std::borrow::ToOwned::to_owned("AmountOutConstantOutOfRange"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "AmountOutConstantOutOfRange",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("swapDirection"),
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
                                    name: ::std::borrow::ToOwned::to_owned("amountOut"),
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
                            ],
                        },
                    ],
                ),
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
                    ::std::borrow::ToOwned::to_owned("LiquidityOutOfRange"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "LiquidityOutOfRange",
                            ),
                            inputs: ::std::vec![
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
                            ],
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
                    ::std::borrow::ToOwned::to_owned("NextLiquidityConstantOutOfRange"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "NextLiquidityConstantOutOfRange",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("swapDirection"),
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
                                    name: ::std::borrow::ToOwned::to_owned("amountOut"),
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
                            ],
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
                    ::std::borrow::ToOwned::to_owned("XReserveOutOfRange"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("XReserveOutOfRange"),
                            inputs: ::std::vec![
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
                                    name: ::std::borrow::ToOwned::to_owned("swapConstant"),
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
                    ::std::borrow::ToOwned::to_owned("YReserveOutOfRange"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("YReserveOutOfRange"),
                            inputs: ::std::vec![
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
                                    name: ::std::borrow::ToOwned::to_owned("swapConstant"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static RMM_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15b\0\0^W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`@Qb\0F\xCB8\x03\x80b\0F\xCB\x839\x81\x01`@\x81\x90Rb\0\0\x81\x91b\0\x01hV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x80\x89\x16`\x01`\x01`\xA0\x1B\x03\x19\x92\x83\x16\x17\x90\x92U`\x01\x80T\x92\x88\x16\x92\x90\x91\x16\x91\x90\x91\x17\x90U`\x08\x84\x90U`\x07\x84\x90UB`\x0B\x81\x90U`\t\x81\x90U`\r\x84\x90U`\x0C\x84\x90U`\x10\x81\x90U`\x0E\x81\x90U`\x12\x83\x90U`\x11\x83\x90U`\x15\x81\x90U`\x13Ug\r\xE0\xB6\xB3\xA7d\0\0\x81\x10b\0\x01<W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01Rp\nn\xEC.\x04\x0C\xCC\xAC\xA4\x0E\x8D\xED\xE4\r\r,\xED`{\x1B`D\x82\x01R`d\x01`@Q\x80\x91\x03\x90\xFD[`\x02UPb\0\x02\x10\x93PPPPV[\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x01cW`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15b\0\x01\xCDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[b\0\x01\xD8\x87b\0\x01KV[\x95Pb\0\x01\xE8` \x88\x01b\0\x01KV[\x94P`@\x87\x01Q\x93P``\x87\x01Q\x92P`\x80\x87\x01Q\x91P`\xA0\x87\x01Q\x90P\x92\x95P\x92\x95P\x92\x95V[aD\xAB\x80b\0\x02 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x02\xC1W`\x005`\xE0\x1C\x80c\x8DR\xA1\xFC\x11a\x01\x9EW\x80c\xC5)\x87\xCF\x11a\x01\x10W\x80c\xDCv\xFA\xBC\x11a\0\xD4W\x80c\xDCv\xFA\xBC\x14a\x05\xB9W\x80c\xF2\xD1\x92/\x14a\x04<W\x80c\xF3\xA8\xEF\xE3\x14a\x05\xC1W\x80c\xF9\xA1\xC8Z\x14a\x05\xC9W\x80c\xFA\xDF\xA6[\x14a\x05\xDCW\x80c\xFE\xD3\xDF\xDA\x14a\x05\xE5Wa\x02\xC1V[\x80c\xC5)\x87\xCF\x14a\x05{W\x80c\xCC\xD1\xE4\xBE\x14a\x05\x83W\x80c\xCFx\xE4\x8F\x14a\x05\x96W\x80c\xCF\xC4\xAFU\x14a\x05\xA9W\x80c\xD4\xCA\xDFh\x14a\x05\xB1Wa\x02\xC1V[\x80c\xAF\xDF1\xCD\x11a\x01bW\x80c\xAF\xDF1\xCD\x14a\x05?W\x80c\xB7\xD1\x9F\xC4\x14a\x05GW\x80c\xBB\x04\x98\xDE\x14a\x05ZW\x80c\xBC\xC1}\xC7\x14a\x05bW\x80c\xC0\xFF\x1A\x15\x14a\x05jW\x80c\xC1e&\x12\x14a\x05rWa\x02\xC1V[\x80c\x8DR\xA1\xFC\x14a\x04\xE0W\x80c\x99\x87\xFEd\x14a\x04\xF3W\x80c\x9C\x9D\xA9\xEA\x14a\x05\x06W\x80c\x9E\x192\xD8\x14a\x05\x19W\x80c\xA5\x9C\x18o\x14a\x05,Wa\x02\xC1V[\x80cT\xCF*\xEB\x11a\x027W\x80cp\xA0\x821\x11a\x01\xFBW\x80cp\xA0\x821\x14a\x04jW\x80cvp\x166\x14a\x04\x8AW\x80c}iS~\x14a\x04\x9DW\x80c\x7F\x0EL\x8C\x14a\x04\xB0W\x80c\x87kU\xF1\x14a\x04\xC3W\x80c\x89\xE2\xC2Y\x14a\x04\xD8Wa\x02\xC1V[\x80cT\xCF*\xEB\x14a\x04+W\x80cU\x9D\x16\x02\x14a\x044W\x80cYT,\xA9\x14a\x04<W\x80c]\x84\n\xE5\x14a\x04OW\x80c^aZk\x14a\x04bWa\x02\xC1V[\x80c\x16\xDC\x16[\x11a\x02\x89W\x80c\x16\xDC\x16[\x14a\x03\xA9W\x80c4\xE1\x99\x07\x14a\x03\xD4W\x80c69\xAA2\x14a\x03\xE9W\x80c7\xAAs\x81\x14a\x03\xFCW\x80c>\x1E3\x92\x14a\x04\x0FW\x80cAf\xD3H\x14a\x04\x18Wa\x02\xC1V[\x80c\x02\xC2\xE5]\x14a\x03&W\x80c\x04\xAF\xA8\"\x14a\x03SW\x80c\x08\xEA\xBD\xDA\x14a\x03\x81W\x80c\t\x10\xA5\x10\x14a\x03\x98W\x80c\x15w\x0F\x92\x14a\x03\xA0W[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x039a\x0346`\x04a@\x1CV[a\x05\xEDV[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01[`@Q\x80\x91\x03\x90\xF3[a\x03fa\x03a6`\x04a@FV[a\x08\x90V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x03JV[a\x03\x8A`\x03T\x81V[`@Q\x90\x81R` \x01a\x03JV[`\x05Ta\x03\x8AV[a\x03\x8A`\x05T\x81V[`\0Ta\x03\xBC\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x03JV[a\x03\xE7a\x03\xE26`\x04a@\x1CV[a\n\xFCV[\0[a\x03fa\x03\xF76`\x04a@~V[a\x0BPV[a\x03fa\x04\n6`\x04a@~V[a\x0BnV[a\x03\x8A`\rT\x81V[a\x03\x8Aa\x04&6`\x04a@\x1CV[a\x0COV[a\x03\x8A`\x02T\x81V[`\x03Ta\x03\x8AV[a\x03\x8Aa\x04J6`\x04a@\xA3V[a\x0C\x81V[a\x03\x8Aa\x04]6`\x04a@FV[a\x0C\x8FV[a\x03fa\r\xF0V[a\x03\x8Aa\x04x6`\x04a@\xE1V[`\x06` R`\0\x90\x81R`@\x90 T\x81V[a\x03fa\x04\x986`\x04aA\rV[a\x0E\x18V[a\x039a\x04\xAB6`\x04a@\x1CV[a\x11}V[a\x03\xE7a\x04\xBE6`\x04a@~V[a\x13\xEFV[a\x04\xCBa\x14\xB4V[`@Qa\x03J\x91\x90aA\x82V[a\x03\x8Aa\x14\xFCV[a\x03\xE7a\x04\xEE6`\x04a@~V[a\x15!V[a\x03\xE7a\x05\x016`\x04a@~V[a\x15\xADV[a\x039a\x05\x146`\x04a@\x1CV[a\x169V[a\x03\xE7a\x05'6`\x04aA\x95V[a\x18\xC0V[a\x039a\x05:6`\x04a@~V[a\x1B\xDBV[a\x03\x8Aa\x1D\xB7V[`\x01Ta\x03\xBC\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x03\x8Aa\x1E\"V[a\x03\xE7a\x1EYV[a\x03\x8Aa\x1E\xA6V[a\x03\x8A`\x08T\x81V[a\x03\x8Aa\x1E\xC1V[a\x039a\x05\x916`\x04a@\x1CV[a\x1F,V[a\x03\x8Aa\x05\xA46`\x04a@FV[a!\x9EV[a\x03\x8Aa\"lV[`\x02Ta\x03\x8AV[a\x03\x8Aa\"\xD7V[a\x03\x8Aa\"\xFFV[a\x039a\x05\xD76`\x04a@~V[a#/V[a\x03\x8A`\x04T\x81V[`\x04Ta\x03\x8AV[`\0\x80`\0\x80`\0a\x05\xFDa\r\xF0V[\x92P\x92P\x92P`\0a\x06\x16`\x03T`\x05T\x86\x86\x86a$lV[\x90P`\0a\x064\x88`\x03Ta\x06+\x91\x90aA\xDAV[\x83\x87\x87\x87a$\xF0V[\x90P`\0a\x06E\x82\x84\x88\x88\x88a%0V[\x90P`\0`\x04T\x82a\x06W\x91\x90aA\xEDV[\x90P`\0`\x05T\x84a\x06i\x91\x90aA\xEDV[\x90P\x83`\x05\x81\x90UP\x8A`\x03`\0\x82\x82Ta\x06\x84\x91\x90aA\xDAV[\x92PP\x81\x90UP\x81`\x04`\0\x82\x82Ta\x06\x9D\x91\x90aA\xDAV[\x92PP\x81\x90UP`\0\x80T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c#\xB8r\xDD30\x8E`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x06\xE7\x93\x92\x91\x90aB\0V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x07?W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` aDk\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x07SW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07w\x91\x90aB$V[P`\x01T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\x07\xAC\x903\x900\x90\x87\x90`\x04\x01aB\0V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x08\x04W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` aDk\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x08\x18W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08<\x91\x90aB$V[P`@\x80Q\x85\x81R` \x81\x01\x8D\x90R\x90\x81\x01\x83\x90R3\x90\x7F\xBE\xB3\x88W\x86\xD67\xA4t\xCB\xC2\x87\xC0\xA4E\x87#\x163\xA0w\xF0\xBD05MZK\x18\x99o\xCE\x90``\x01[`@Q\x80\x91\x03\x90\xA2\x9A\x90\x99P\x97PPPPPPPPV[`\0\x80`\0`\x05T`\0\x14a\x08\xE7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01Rw\x14\x1B\xDB\xDB\x08\x18[\x1C\x99XY\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`B\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x80`\0a\x08\xF4a\r\xF0V[\x92P\x92P\x92P\x88\x15a\t&W\x87\x95Pa\t\x10\x86\x88\x85\x85\x85a$\xF0V[\x93Pa\t\x1F\x84\x88\x85\x85\x85a%0V[\x94Pa\tHV[\x87\x94Pa\t6\x85\x88\x85\x85\x85a%RV[\x93Pa\tE\x84\x88\x85\x85\x85a%hV[\x95P[`\x05\x84\x90U`\x03\x86\x90U`\x04\x85\x81U3`\0\x81\x81R`\x06` R`@\x80\x82 \x88\x90U\x90T\x90Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x92c#\xB8r\xDD\x92a\t\x9A\x92\x90\x910\x91\x8C\x91\x01aB\0V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\t\xF2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` aDk\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\n\x06W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n*\x91\x90aB$V[P`\x01T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\n_\x903\x900\x90\x8A\x90`\x04\x01aB\0V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\n\xB7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` aDk\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\n\xCBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\xEF\x91\x90aB$V[PPPP\x93P\x93P\x93\x90PV[g\r\xE0\xB6\xB3\xA7d\0\0\x81\x10a\x0BKW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rt\t\xCC\xAE\xE4\x0En\xEC.\x04\x0C\xCC\xAC\xA4\x0E\x8D\xED\xE4\r\r,\xED`[\x1B`D\x82\x01R`d\x01a\x08\xDEV[`\x02UV[`\0\x80`\0a\x0Ba`\x01\x86\x86a\x08\x90V[\x92P\x92P\x92P\x92P\x92P\x92V[`\0\x80`\0\x80`\0\x80a\x0B\x7Fa\r\xF0V[\x92P\x92P\x92P`\0a\x0B\x94\x89\x89\x86\x86\x86a$\xF0V[\x90Pa\x0B\xC6`@Q\x80`@\x01`@R\x80`\x0E\x81R` \x01mbase liquidity`\x90\x1B\x81RP\x82a%\x92V[a\x0B\xD3\x81\x89\x86\x86\x86a%0V[\x95Pa\x0B\xFF`@Q\x80`@\x01`@R\x80`\x08\x81R` \x01ginitialY`\xC0\x1B\x81RP\x87a%\x92V[a\x0C\r\x89\x87\x83\x87\x87\x87a%\xDBV[\x94Pa\x0CA`@Q\x80`@\x01`@R\x80`\x10\x81R` \x01oinitialLiquidity`\x80\x1B\x81RP\x86a%\x92V[\x88\x96PPPPP\x92P\x92P\x92V[`\0\x80`\0\x80a\x0C]a\r\xF0V[\x92P\x92P\x92Pa\x0Cx`\x03T`\x04T`\x05T\x86\x86\x86\x8Ba&\xABV[\x95\x94PPPPPV[`\0a\x0Cx\x85\x85\x85\x85a&\xE7V[`\0a\x0C\xB5`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[a\x0C\xBDa\r\xF0V[`@\x84\x01\x81\x90R` \x84\x01\x82\x90R\x82\x84R`\x03T`\0\x93a\x0C\xE2\x93\x91\x92\x89\x92\x90a$lV[\x90P`\0g\r\xE0\xB6\xB3\xA7d\0\0`\x02Tg\r\xE0\xB6\xB3\xA7d\0\0a\r\x05\x91\x90aA\xEDV[a\r\x0F\x90\x87aBDV[a\r\x19\x91\x90aBqV[\x90P\x86\x15a\r\x8BW`\0a\r<\x82\x84\x86`\0\x01Q\x87` \x01Q\x88`@\x01Qa$\xF0V[\x90P`\0a\ro\x87`\x03Ta\rQ\x91\x90aA\xDAV[`\x04Ta\r^\x85\x8CaA\xDAV[\x88Q` \x8A\x01Q`@\x8B\x01Qa-\xD7V[\x90P\x80`\x04Ta\r\x7F\x91\x90aA\xEDV[\x95PPPPPPa\r\xE9V[`\0a\r\xA6\x82\x84\x86`\0\x01Q\x87` \x01Q\x88`@\x01Qa%RV[\x90P`\0a\r\xD9`\x03T\x88`\x04Ta\r\xBE\x91\x90aA\xDAV[a\r\xC8\x85\x8CaA\xDAV[\x88Q` \x8A\x01Q`@\x8B\x01Qa.\x81V[\x90P\x80`\x03Ta\r\x7F\x91\x90aA\xEDV[\x93\x92PPPV[`\0\x80`\0a\r\xFDa\x1E\xC1V[a\x0E\x05a\x1D\xB7V[a\x0E\ra\"lV[\x92P\x92P\x92P\x90\x91\x92V[`\0\x80`\0\x80`\x05T\x11a\x0EeW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rs\x14\x1B\xDB\xDB\x08\x1B\x9B\xDD\x08\x1A[\x9A]\x1AX[\x1A^\x99Y`b\x1B`D\x82\x01R`d\x01a\x08\xDEV[`\0\x80`\0a\x0Era\r\xF0V[\x92P\x92P\x92P`\0a\x0E\x8B`\x03T`\x05T\x86\x86\x86a$lV[\x90P\x88\x15a\x0E\xE3W\x87\x96P`\0a\x0E\xA9\x88`\x03Ta\x06+\x91\x90aA\xDAV[\x90P`\0a\x0E\xBA\x82\x84\x88\x88\x88a%0V[\x90P`\x04T\x81a\x0E\xCA\x91\x90aA\xEDV[\x97P`\x05T\x82a\x0E\xDA\x91\x90aA\xEDV[\x96PPPa\x0F8V[\x87\x95P`\0a\x0F\x02\x87`\x04Ta\x0E\xF9\x91\x90aA\xDAV[\x83\x87\x87\x87a%RV[\x90P`\0a\x0F\x13\x82\x84\x88\x88\x88a%hV[\x90P`\x03T\x81a\x0F#\x91\x90aA\xEDV[\x98P`\x05T\x82a\x0F3\x91\x90aA\xEDV[\x96PPP[\x84`\x05`\0\x82\x82Ta\x0FJ\x91\x90aA\xDAV[\x92PP\x81\x90UP\x86`\x03`\0\x82\x82Ta\x0Fc\x91\x90aA\xDAV[\x92PP\x81\x90UP\x85`\x04`\0\x82\x82Ta\x0F|\x91\x90aA\xDAV[\x90\x91UPP3`\0\x90\x81R`\x06` R`@\x81 \x80T\x87\x92\x90a\x0F\xA0\x90\x84\x90aA\xDAV[\x90\x91UPP`\0T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\x0F\xD9\x903\x900\x90\x8C\x90`\x04\x01aB\0V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x101W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` aDk\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x10EW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10i\x91\x90aB$V[P`\x01T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\x10\x9E\x903\x900\x90\x8B\x90`\x04\x01aB\0V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x10\xF6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` aDk\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x11\nW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11.\x91\x90aB$V[P`@\x80Q\x86\x81R` \x81\x01\x89\x90R\x90\x81\x01\x87\x90R3\x90\x7F\xBE\xB3\x88W\x86\xD67\xA4t\xCB\xC2\x87\xC0\xA4E\x87#\x163\xA0w\xF0\xBD05MZK\x18\x99o\xCE\x90``\x01`@Q\x80\x91\x03\x90\xA2PPPP\x92P\x92P\x92V[`\0\x80`\0\x80`\0a\x11\x8Da\r\xF0V[\x92P\x92P\x92P`\0a\x11\xA6`\x03T`\x05T\x86\x86\x86a$lV[\x90P`\0a\x11\xBB\x88`\x03Ta\x0E\xF9\x91\x90aA\xEDV[\x90P`\0a\x11\xCC\x82\x84\x88\x88\x88a%hV[\x90P`\0\x81`\x03Ta\x11\xDE\x91\x90aA\xEDV[\x90P`\0\x83`\x05Ta\x11\xF0\x91\x90aA\xEDV[\x90P\x83`\x05\x81\x90UP\x81`\x03`\0\x82\x82Ta\x12\x0B\x91\x90aA\xEDV[\x92PP\x81\x90UP\x8A`\x04`\0\x82\x82Ta\x12$\x91\x90aA\xEDV[\x90\x91UPP`\0T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x12\xB3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` aDk\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x12\xC7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\xEB\x91\x90aB$V[P`\x01T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x8D\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x13vW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` aDk\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x13\x8AW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\xAE\x91\x90aB$V[P`@\x80Q\x85\x81R` \x81\x01\x84\x90R\x90\x81\x01\x8C\x90R3\x90\x7FY\xC3\xA0\xB6\x0Cj\xB7\xDE\xB6.\x14@\xC9\xE7$A\xDBm\xB7\xDF\xE5\x14\xDB\xA8\xCB\x18\xE6\x0C\r\x89n\xFA\x90``\x01a\x08yV[B\x81\x11a\x14\x0EW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xDE\x90aB\x85V[a\x14\x16a/!V[`\0\x82`\x11T\x11a\x143W`\x11Ta\x14.\x90\x84aA\xEDV[a\x14AV[\x82`\x11Ta\x14A\x91\x90aA\xEDV[\x90Pa\x14MB\x83aA\xEDV[a\x14W\x90\x82aBqV[`\x14U`\x12\x83\x90U`\x15\x82\x90U`\0\x80Q` aD\x8B\x839\x81Q\x91Ra\x14{a\x1D\xB7V[a\x14\x83a\x1E\xC1V[a\x14\x8Ba\"lV[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01RB``\x82\x01R`\x80\x01`@Q\x80\x91\x03\x90\xA1PPPV[``a\x14\xBEa\x1D\xB7V[a\x14\xC6a\x1E\xC1V[a\x14\xCEa\"lV[`@\x80Q` \x81\x01\x94\x90\x94R\x83\x01\x91\x90\x91R``\x82\x01R`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[`\0a\x15\x1C`\x04T`\x05Ta\x15\x0Fa\x1E\xC1V[a\x15\x17a\x1D\xB7V[a/2V[\x90P\x90V[B\x81\x11a\x15@W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xDE\x90aB\x85V[a\x15Ha/\xA2V[`\0\x82`\x0CT\x11a\x15eW`\x0CTa\x15`\x90\x84aA\xEDV[a\x15sV[\x82`\x0CTa\x15s\x91\x90aA\xEDV[\x90Pa\x15\x7FB\x83aA\xEDV[a\x15\x89\x90\x82aBqV[`\x0FU`\r\x83\x90U`\x10\x82\x90U`\0\x80Q` aD\x8B\x839\x81Q\x91Ra\x14{a\x1D\xB7V[B\x81\x11a\x15\xCCW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xDE\x90aB\x85V[a\x15\xD4a/\xB3V[`\0\x82`\x07T\x11a\x15\xF1W`\x07Ta\x15\xEC\x90\x84aA\xEDV[a\x15\xFFV[\x82`\x07Ta\x15\xFF\x91\x90aA\xEDV[\x90Pa\x16\x0BB\x83aA\xEDV[a\x16\x15\x90\x82aBqV[`\nU`\x08\x83\x90U`\x0B\x82\x90U`\0\x80Q` aD\x8B\x839\x81Q\x91Ra\x14{a\x1D\xB7V[`\0\x80`\0\x80`\0a\x16Ia\r\xF0V[\x92P\x92P\x92P`\0a\x16b`\x03T`\x05T\x86\x86\x86a$lV[\x90P`\0a\x16w\x88`\x04Ta\x0E\xF9\x91\x90aA\xDAV[\x90P`\0a\x16\x88\x82\x84\x88\x88\x88a%hV[\x90P`\0`\x03T\x82a\x16\x9A\x91\x90aA\xEDV[\x90P`\0`\x05T\x84a\x16\xAC\x91\x90aA\xEDV[\x90P\x83`\x05\x81\x90UP\x81`\x03`\0\x82\x82Ta\x16\xC7\x91\x90aA\xDAV[\x92PP\x81\x90UP\x8A`\x04`\0\x82\x82Ta\x16\xE0\x91\x90aA\xDAV[\x90\x91UPP`\0T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\x17\x19\x903\x900\x90\x87\x90`\x04\x01aB\0V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x17qW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` aDk\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x17\x85W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17\xA9\x91\x90aB$V[P`\x01`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c#\xB8r\xDD30\x8E`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x17\xEF\x93\x92\x91\x90aB\0V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x18GW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` aDk\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x18[W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18\x7F\x91\x90aB$V[P`@\x80Q\x85\x81R` \x81\x01\x84\x90R\x90\x81\x01\x8C\x90R3\x90\x7F\xBE\xB3\x88W\x86\xD67\xA4t\xCB\xC2\x87\xC0\xA4E\x87#\x163\xA0w\xF0\xBD05MZK\x18\x99o\xCE\x90``\x01a\x08yV[`\x05T\x15a\x19\x0BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01Rw\x14\x1B\xDB\xDB\x08\x18[\x1C\x99XY\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`B\x1B`D\x82\x01R`d\x01a\x08\xDEV[`\0\x80`\0a\x19\x18a\r\xF0V[\x92P\x92P\x92P`\0a\x19/\x87\x87\x87\x87\x87\x87\x8Ba&\xABV[\x90P`\0a\x19B\x88\x88\x88\x88\x88\x88\x8Ea/\xC4V[\x90P`\0a\x19U\x89\x89\x89\x89\x89\x89\x8Ea0\0V[\x90P\x82`\x18\x19\x13\x80a\x19gWP`\x19\x83\x13[\x15a\x19\x8FW`@Qc\x08\xD4\xF3\xC1`\xE3\x1B\x81R`\x04\x81\x01\x88\x90R`$\x81\x01\x84\x90R`D\x01a\x08\xDEV[\x81`\x18\x19\x13\x80a\x19\x9FWP`\x19\x82\x13[\x15a\x19\xC7W`@Qc\xC7KR\x13`\xE0\x1B\x81R`\x04\x81\x01\x8A\x90R`$\x81\x01\x83\x90R`D\x01a\x08\xDEV[\x80`\x18\x19\x13\x80a\x19\xD7WP`\x19\x81\x13[\x15a\x19\xFFW`@Qc:M\x97\x7F`\xE2\x1B\x81R`\x04\x81\x01\x89\x90R`$\x81\x01\x82\x90R`D\x01a\x08\xDEV[\x86`\x05\x81\x90UP\x88`\x03`\0\x82\x82Ta\x1A\x18\x91\x90aA\xDAV[\x92PP\x81\x90UP\x87`\x04`\0\x82\x82Ta\x1A1\x91\x90aA\xDAV[\x90\x91UPP3`\0\x81\x81R`\x06` R`@\x80\x82 \x8A\x90U\x90T\x90Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91c#\xB8r\xDD\x91a\x1Az\x91\x900\x90\x8E\x90`\x04\x01aB\0V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1A\xD2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` aDk\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x1A\xE6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B\n\x91\x90aB$V[P`\x01T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\x1B?\x903\x900\x90\x8D\x90`\x04\x01aB\0V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1B\x97W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` aDk\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x1B\xABW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B\xCF\x91\x90aB$V[PPPPPPPPPPV[`\0\x80`\0\x80`\0a\x1B\xEBa\r\xF0V[\x92P\x92P\x92P`\0a\x1C\0\x88\x88\x86\x86\x86a%RV[\x90P`\0a\x1C\x11\x82\x89\x87\x87\x87a%hV[`\x05\x83\x90U`\x03\x81\x90U`\x04\x8A\x81U`\0T`@Qc#\xB8r\xDD`\xE0\x1B\x81R\x92\x93P`\x01`\x01`\xA0\x1B\x03\x16\x91c#\xB8r\xDD\x91a\x1CS\x913\x910\x91\x87\x91\x01aB\0V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1C\xABW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` aDk\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x1C\xBFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C\xE3\x91\x90aB$V[P`\x01T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\x1D\x18\x903\x900\x90\x8E\x90`\x04\x01aB\0V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1DpW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` aDk\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x1D\x84W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1D\xA8\x91\x90aB$V[P\x90\x98\x90\x97P\x95PPPPPPV[`\0`\x0BTB\x10a\x1D\xC9WP`\x08T\x90V[`\x08T`\x07T\x11a\x1D\xFBW`\nT`\tTa\x1D\xE4\x90BaA\xEDV[a\x1D\xEE\x91\x90aBDV[`\x07Ta\x15\x1C\x91\x90aA\xDAV[`\nT`\tTa\x1E\x0B\x90BaA\xEDV[a\x1E\x15\x91\x90aBDV[`\x07Ta\x15\x1C\x91\x90aA\xEDV[`\0`\x04Tg\r\xE0\xB6\xB3\xA7d\0\0a\x1E8a\"\xD7V[`\x03Ta\x1EE\x91\x90aBDV[a\x1EO\x91\x90aBqV[a\x15\x1C\x91\x90aA\xDAV[`\0\x80Q` aD\x8B\x839\x81Q\x91Ra\x1Epa\x1D\xB7V[a\x1Exa\x1E\xC1V[a\x1E\x80a\"lV[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01RB``\x82\x01R`\x80\x01`@Q\x80\x91\x03\x90\xA1V[`\0a\x15\x1C`\x03T`\x05T`\x04Ta\x1E\xBCa\x1E\xC1V[a0<V[`\0`\x10TB\x10a\x1E\xD3WP`\rT\x90V[`\rT`\x0CT\x11a\x1F\x05W`\x0FT`\x0ETa\x1E\xEE\x90BaA\xEDV[a\x1E\xF8\x91\x90aBDV[`\x0CTa\x15\x1C\x91\x90aA\xDAV[`\x0FT`\x0ETa\x1F\x15\x90BaA\xEDV[a\x1F\x1F\x91\x90aBDV[`\x0CTa\x15\x1C\x91\x90aA\xEDV[`\0\x80`\0\x80`\0a\x1F<a\r\xF0V[\x92P\x92P\x92P`\0a\x1FU`\x03T`\x05T\x86\x86\x86a$lV[\x90P`\0a\x1Fj\x88`\x03Ta\x06+\x91\x90aA\xEDV[\x90P`\0a\x1F{\x82\x84\x88\x88\x88a%0V[\x90P`\0\x81`\x04Ta\x1F\x8D\x91\x90aA\xEDV[\x90P`\0\x83`\x05Ta\x1F\x9F\x91\x90aA\xEDV[\x90P\x83`\x05\x81\x90UP\x8A`\x03`\0\x82\x82Ta\x1F\xBA\x91\x90aA\xEDV[\x92PP\x81\x90UP\x81`\x04`\0\x82\x82Ta\x1F\xD3\x91\x90aA\xEDV[\x90\x91UPP`\0T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x8D\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a bW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` aDk\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a vW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a \x9A\x91\x90aB$V[P`\x01T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a!%W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` aDk\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a!9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a!]\x91\x90aB$V[P`@\x80Q\x85\x81R` \x81\x01\x8D\x90R\x90\x81\x01\x83\x90R3\x90\x7FY\xC3\xA0\xB6\x0Cj\xB7\xDE\xB6.\x14@\xC9\xE7$A\xDBm\xB7\xDF\xE5\x14\xDB\xA8\xCB\x18\xE6\x0C\r\x89n\xFA\x90``\x01a\x08yV[`\0a!\xC4`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[a!\xCCa\r\xF0V[`@\x84\x01\x81\x90R` \x84\x01\x82\x90R\x82\x84R`\x03T`\0\x93a!\xF1\x93\x91\x92\x89\x92\x90a$lV[\x90P`\0g\r\xE0\xB6\xB3\xA7d\0\0`\x02Tg\r\xE0\xB6\xB3\xA7d\0\0a\"\x14\x91\x90aA\xEDV[a\"\x1E\x90\x87aBDV[a\"(\x91\x90aBqV[\x90P\x86\x15a\"SWa\"I\x81\x83\x85`\0\x01Q\x86` \x01Q\x87`@\x01Qa$\xF0V[\x93PPPPa\r\xE9V[a\"I\x81\x83\x85`\0\x01Q\x86` \x01Q\x87`@\x01Qa%RV[`\0`\x15TB\x10a\"~WP`\x12T\x90V[`\x12T`\x11T\x11a\"\xB0W`\x14T`\x13Ta\"\x99\x90BaA\xEDV[a\"\xA3\x91\x90aBDV[`\x11Ta\x15\x1C\x91\x90aA\xDAV[`\x14T`\x13Ta\"\xC0\x90BaA\xEDV[a\"\xCA\x91\x90aBDV[`\x11Ta\x15\x1C\x91\x90aA\xEDV[`\0a\x15\x1C`\x03T`\x05Ta\"\xEAa\x1E\xC1V[a\"\xF2a\x1D\xB7V[a\"\xFAa\"lV[a$lV[`\0\x80`\0\x80a#\ra\r\xF0V[\x92P\x92P\x92Pa#'`\x03T`\x04T`\x05T\x86\x86\x86a%\xDBV[\x93PPPP\x90V[`\0\x80`\0\x80`\0a#?a\r\xF0V[\x92P\x92P\x92P`\0a#T\x88\x88\x86\x86\x86a$\xF0V[\x90P`\0a#e\x82\x89\x87\x87\x87a%0V[`\x05\x83\x90U`\x03\x8A\x90U`\x04\x81\x81U`\0T`@Qc#\xB8r\xDD`\xE0\x1B\x81R\x92\x93P`\x01`\x01`\xA0\x1B\x03\x16\x91c#\xB8r\xDD\x91a#\xA7\x913\x910\x91\x8F\x91\x01aB\0V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a#\xFFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` aDk\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a$\x13W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a$7\x91\x90aB$V[P`\x01T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\x1D\x18\x903\x900\x90\x86\x90`\x04\x01aB\0V[`\0\x80a$y\x84\x84a0lV[\x90P`\0a$\x87\x85\x85a0\x9AV[\x90P`\0a$\x95\x89\x89a0\xCCV[\x90Pa$\xE3\x87a$\xDE\x84g\r\xE0\xB6\xB3\xA7d\0\0\x87a$\xBBa$\xB6\x88\x84aA\xEDV[a0\xEAV[a$\xC5\x91\x90aB\xB0V[a$\xCF\x91\x90aB\xE0V[a$\xD9\x91\x90aC\x0EV[a1\x87V[a30V[\x99\x98PPPPPPPPPV[`\0\x80a%\x07a%\x02\x87\x87\x87\x87a3EV[a3\xA3V[a%\x19\x90g\r\xE0\xB6\xB3\xA7d\0\0aC\x0EV[\x90Pa%%\x87\x82a4\x0CV[\x97\x96PPPPPPPV[`\0\x80a%Ba%\x02\x87\x87\x87\x87a4!V[\x90Pa%%\x85a$\xDE\x89\x84a30V[`\0\x80a%\x19\x85a$\xDEa%\x02\x89\x89\x89\x89a4!V[`\0\x80a%za%\x02\x87\x87\x87\x87a3EV[\x90Pa%%\x87a$\xDE\x83g\r\xE0\xB6\xB3\xA7d\0\0aC\x0EV[a%\xD7\x82\x82`@Q`$\x01a%\xA8\x92\x91\x90aC5V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c-\x83\x9C\xB3`\xE2\x1B\x17\x90Ra4WV[PPV[`\0\x80`\0\x80a%\xEF\x8A\x8A\x8A\x8A\x8A\x8Aa4cV[\x90P`\0a& \x82`@Q` \x01a&\x07\x91\x90aCWV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83`@\x01Qa4\xC1V[\x90P\x80`\0\x03a&6W\x88\x94PPPPPa&\xA1V[`\0\x81\x12\x15a&VW\x88\x92Pa&O\x83`2`da5/V[\x93Pa&iV[\x88\x93Pa&f\x84`\x96`da5NV[\x92P[a&\x9A\x82`@Q` \x01a&}\x91\x90aCWV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x85\x85`\n`\x80a4\xC1a5|V[\x94PPPPP[\x96\x95PPPPPPV[`\0\x80a&\xBC\x89\x89\x89\x89\x89\x89a4cV[\x90Pa$\xE3\x81`@Q` \x01a&\xD2\x91\x90aCWV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x84a4\xC1V[`\0a'\r`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[a'\x15a\r\xF0V[`@\x84\x01\x81\x90R` \x84\x01\x82\x90R\x82\x84R`\x03T`\x04T`\x05T`\0\x95a'E\x95\x93\x94\x92\x93\x91\x92\x90\x91\x90\x8Ca&\xABV[\x90P\x80`\x18\x19\x13\x80a'WWP`\x19\x81\x13[\x15a'\x8EW`@Qc$J^\x17`\xE1\x1B\x81R\x87\x15\x15`\x04\x82\x01R`$\x81\x01\x86\x90R`D\x81\x01\x85\x90R`d\x81\x01\x82\x90R`\x84\x01a\x08\xDEV[`\0a'\xAB`\x03T\x88\x85`\0\x01Q\x86` \x01Q\x87`@\x01Qa$lV[\x90P\x87\x15a*\xB0W`\0g\r\xE0\xB6\xB3\xA7d\0\0`\x02Tg\r\xE0\xB6\xB3\xA7d\0\0a'\xD4\x91\x90aA\xEDV[a'\xDE\x90\x89aBDV[a'\xE8\x91\x90aBqV[\x90Pa(\x10`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01cfees`\xE0\x1B\x81RP\x82a%\x92V[`\0a(+\x82\x84\x87`\0\x01Q\x88` \x01Q\x89`@\x01Qa$\xF0V[\x90Pa(U`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01e\x19\x19[\x1D\x18S`\xD2\x1B\x81RP\x82a%\x92V[`\0a(\x97\x89`\x03Ta(h\x91\x90aA\xDAV[`\x04Ta(u\x85\x8EaA\xDAV[\x89`\0\x01Q\x8A` \x01Q\x8B`@\x01Q\x8E`\x04Ta(\x92\x91\x90aA\xEDV[a0\0V[\x90P\x80`1\x19\x13\x80a(\xA9WP`2\x81\x13[\x15a(\xE0W`@QcBCc\xB7`\xE1\x1B\x81R\x8B\x15\x15`\x04\x82\x01R`$\x81\x01\x8A\x90R`D\x81\x01\x89\x90R`d\x81\x01\x82\x90R`\x84\x01a\x08\xDEV[a(\xEA\x82\x8BaA\xDAV[`\x05\x81\x90UP\x88`\x03`\0\x82\x82Ta)\x02\x91\x90aA\xDAV[\x92PP\x81\x90UP\x87`\x04`\0\x82\x82Ta)\x1B\x91\x90aA\xEDV[\x90\x91UPP`\0T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a)T\x903\x900\x90\x8E\x90`\x04\x01aB\0V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a)\xACW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` aDk\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a)\xC0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a)\xE4\x91\x90aB$V[P`\x01T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x8A\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a*oW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` aDk\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a*\x83W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a*\xA7\x91\x90aB$V[PPPPa-[V[`\0g\r\xE0\xB6\xB3\xA7d\0\0`\x02Tg\r\xE0\xB6\xB3\xA7d\0\0a*\xD1\x91\x90aA\xEDV[a*\xDB\x90\x89aBDV[a*\xE5\x91\x90aBqV[\x90P`\0a+\x02\x82\x84\x87`\0\x01Q\x88` \x01Q\x89`@\x01Qa%RV[\x90P`\0a+F`\x03T\x8A`\x04Ta+\x1A\x91\x90aA\xDAV[a+$\x85\x8EaA\xDAV[\x89`\0\x01Q\x8A` \x01Q\x8B`@\x01Q\x8E`\x03Ta+A\x91\x90aA\xEDV[a/\xC4V[\x90P\x80`\x18\x19\x13\x80a+XWP`\x19\x81\x13[\x15a+\x8FW`@QcBCc\xB7`\xE1\x1B\x81R\x8B\x15\x15`\x04\x82\x01R`$\x81\x01\x8A\x90R`D\x81\x01\x89\x90R`d\x81\x01\x82\x90R`\x84\x01a\x08\xDEV[a+\x99\x82\x8BaA\xDAV[`\x05\x81\x90UP\x88`\x04`\0\x82\x82Ta+\xB1\x91\x90aA\xDAV[\x92PP\x81\x90UP\x87`\x03`\0\x82\x82Ta+\xCA\x91\x90aA\xEDV[\x90\x91UPP`\x01T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a,\x03\x903\x900\x90\x8E\x90`\x04\x01aB\0V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a,[W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` aDk\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a,oW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a,\x93\x91\x90aB$V[P`\0T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x8A\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a-\x1EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` aDk\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a-2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a-V\x91\x90aB$V[PPPP[3`\x01`\x01`\xA0\x1B\x03\x16\x7F\x17fH\xF1\xF1\x1C\xDA(L\x12D\x90\x08k\xE4*\x92m\xDF\n\xE8\x87\xEB\xE7\xB1\xD6\xB37\xD8\x94'V\x89\x88\x88a-\xA6`\x03T`\x05T\x8A`\0\x01Q\x8B` \x01Q\x8C`@\x01Qa$lV[`@\x80Q\x94\x15\x15\x85R` \x85\x01\x93\x90\x93R\x91\x83\x01R``\x82\x01R`\x80\x01`@Q\x80\x91\x03\x90\xA2P\x92\x96\x95PPPPPPV[`\0\x80`\0\x80a-\xEB\x8A\x8A\x8A\x8A\x8A\x8Aa4cV[\x90P`\0a.\x1C\x82`@Q` \x01a.\x03\x91\x90aCWV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83` \x01Qa6\x8DV[\x90P\x80`\0\x03a.5WP` \x01Q\x92Pa&\xA1\x91PPV[` \x82\x01Qa.G\x90`2`da5/V[\x93P\x81` \x01Q\x92Pa&\x9A\x82`@Q` \x01a.d\x91\x90aCWV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x85\x85`\n`\x80a6\x8Da5|V[`\0\x80`\0\x80a.\x95\x8A\x8A\x8A\x8A\x8A\x8Aa4cV[\x90P`\0a.\xC4\x82`@Q` \x01a.\xAD\x91\x90aCWV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x83Qa6\xE6V[\x90P\x80`\0\x03a.\xDAWPQ\x92Pa&\xA1\x91PPV[\x81Qa.\xE9\x90`2`da5/V[\x82Q\x90\x94P\x92Pa&\x9A\x82`@Q` \x01a/\x04\x91\x90aCWV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x85\x85`\n`\x80a6\xE6a5|V[a/)a\"lV[`\x11UB`\x13UV[`\0\x80a/G\x83g\r\xE0\xB6\xB3\xA7d\0\0a7?V[\x90P`\0a/^\x87a/Y\x87\x89a7kV[a0\xCCV[\x90Pa%%\x85a$\xDEa/r`\x02\x86aB\xE0V[g\r\xE0\xB6\xB3\xA7d\0\0a/\x84\x86a0\xEAV[a/\x8E\x90\x8AaB\xB0V[a/\x98\x91\x90aB\xE0V[a$\xD9\x91\x90aC\x9BV[a/\xAAa\x1E\xC1V[`\x0CUB`\x0EUV[a/\xBBa\x1D\xB7V[`\x07UB`\tUV[`\0\x80a/\xD5\x89\x89\x89\x89\x89\x89a4cV[\x90Pa$\xE3\x81`@Q` \x01a/\xEB\x91\x90aCWV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x84a6\xE6V[`\0\x80a0\x11\x89\x89\x89\x89\x89\x89a4cV[\x90Pa$\xE3\x81`@Q` \x01a0'\x91\x90aCWV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x84a6\x8DV[`\0a0Ua0K\x85\x84a7kV[a$\xB6\x90\x85aBqV[a0ba$\xB6\x86\x88aBqV[a\x0Cx\x91\x90aC\x9BV[`\0\x80a0x\x83a7\x80V[a0\x86\x90c;\x9A\xCA\0aBDV[\x90Pa0\x92\x84\x82a7kV[\x94\x93PPPPV[`\0\x80a0\xB8a0\xB2\x85g\x1B\xC1mgN\xC8\0\0a7?V[\x84a7kV[\x90Pa0\x92g\x06\xF0[Y\xD3\xB2\0\0\x82a7kV[`\0a0\xE1\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a5/V[\x90P[\x92\x91PPV[`\0g\x06\xF0[Y\xD3\xB2\0\0\x82\x03a1\x03WP`\0\x91\x90PV[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x12a1+W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a1LW`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a1Y\x83`\x02aB\xB0V[\x90P`\0a1f\x82a8$V[\x90P`\0a1|g\x13\xA0K\xBD\xFD\xC9\xBE\x88\x83a:\xA2V[\x90Pa\x0Cx\x81aC\xC3V[`\0h\x02H\xCE6\xA7\x0C\xB2k>\x19\x82\x13a1\xA2WP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a1\xE9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x01a\x08\xDEV[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05`\x01`_\x1B\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1Dm\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1Dn\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[`\0a0\xE1\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a5NV[`\0\x80a3R\x84\x84a0lV[\x90P`\0a3`\x87\x87a:\xB7V[\x90P`\0a3n\x86\x86a0\x9AV[\x90P\x82a3{\x82\x84aC\x9BV[a3\x8D\x90g\r\xE0\xB6\xB3\xA7d\0\0aB\xB0V[a3\x97\x91\x90aB\xE0V[\x98\x97PPPPPPPPV[`\0\x80g\x13\xA0K\xBD\xFD\xC9\xBE\x88a3\xC1g\r\xE0\xB6\xB3\xA7d\0\0\x85aB\xB0V[a3\xCB\x91\x90aB\xE0V[\x90P`\0a3\xD8\x82aC\xC3V[\x90P`\0a3\xE5\x82a:\xCBV[\x90Pg\x1B\xC1mgN\xC8\0\0a4\x02g\r\xE0\xB6\xB3\xA7d\0\0\x83aB\xB0V[a\x0Cx\x91\x90aB\xE0V[`\0a0\xE1\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a5NV[`\0\x80a4.\x84\x84a0lV[\x90P`\0a4<\x87\x87a:\xB7V[\x90P`\0a4J\x86\x86a0\x9AV[\x90P\x82a3{\x82\x84aC\x0EV[a4`\x81a<\xAFV[PV[a4\x9C`@Q\x80`\xC0\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[\x95\x86R` \x86\x01\x94\x90\x94R`@\x85\x01\x92\x90\x92R``\x84\x01R`\x80\x83\x01R`\xA0\x82\x01R\x90V[`\0\x80\x83\x80` \x01\x90Q\x81\x01\x90a4\xD8\x91\x90aC\xDFV[\x90Pa4\xEC\x81`\x80\x01Q\x82`\xA0\x01Qa0lV[a5\na5\x05\x83` \x01Qa/Y\x87\x86``\x01Qa7kV[a<\xD0V[a5\x1Ba5\x05\x84`\0\x01Q\x87a0\xCCV[a5%\x91\x90aC\x9BV[a0\x92\x91\x90aC\x9BV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a5GW`\0\x80\xFD[\x04\x92\x91PPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a5fW`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[`\0\x84\x86\x11\x15a5\xA9W`@Qc0\x82\xDF\xDB`\xE1\x1B\x81R`\x04\x81\x01\x87\x90R`$\x81\x01\x86\x90R`D\x01a\x08\xDEV[`\0a5\xB9\x88\x88\x85c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a5\xCB\x89\x88\x86c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a5\xD9\x82\x84aB\xB0V[\x13\x15a6\x02W`@Qc\x06\xF1\xBE]`\xE2\x1B\x81R`\x04\x81\x01\x83\x90R`$\x81\x01\x82\x90R`D\x01a\x08\xDEV[`\0a6\x0E\x89\x89aA\xEDV[\x90P`\0[`\x02a6\x1F\x8A\x8CaA\xDAV[a6)\x91\x90aBqV[\x94P`\0a6;\x8C\x87\x89c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a6I\x86\x83aB\xB0V[\x13a6VW\x85\x99Pa6]V[\x85\x9AP\x80\x94P[a6g\x8B\x8BaA\xEDV[\x92PP`\x01\x01\x87\x82\x11\x80\x15a6{WP\x86\x81\x10[a6\x13WPPPP\x96\x95PPPPPPV[`\0\x80\x83\x80` \x01\x90Q\x81\x01\x90a6\xA4\x91\x90aC\xDFV[\x90Pa6\xB8\x81`\x80\x01Q\x82`\xA0\x01Qa0lV[a6\xD1a5\x05\x85a/Y\x85`@\x01Q\x86``\x01Qa7kV[a5\x1Ba5\x05\x84`\0\x01Q\x85`@\x01Qa0\xCCV[`\0\x80\x83\x80` \x01\x90Q\x81\x01\x90a6\xFD\x91\x90aC\xDFV[\x90Pa7\x11\x81`\x80\x01Q\x82`\xA0\x01Qa0lV[a7.a5\x05\x83` \x01Qa/Y\x85`@\x01Q\x86``\x01Qa7kV[a5\x1Ba5\x05\x86\x85`@\x01Qa0\xCCV[`\0a0\xE1g\r\xE0\xB6\xB3\xA7d\0\0\x83a7W\x86a<\xDEV[a7a\x91\x90aB\xB0V[a$\xD9\x91\x90aB\xE0V[`\0a0\xE1\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a5/V[`\xB5\x81`\x01`\x88\x1B\x81\x10a7\x99W`@\x91\x90\x91\x1B\x90`\x80\x1C[i\x01\0\0\0\0\0\0\0\0\0\x81\x10a7\xB5W` \x91\x90\x91\x1B\x90`@\x1C[e\x01\0\0\0\0\0\x81\x10a7\xCDW`\x10\x91\x90\x91\x1B\x90` \x1C[c\x01\0\0\0\x81\x10a7\xE3W`\x08\x91\x90\x91\x1B\x90`\x10\x1C[b\x01\0\0\x01\x02`\x12\x1C\x80\x82\x04\x01`\x01\x90\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x90\x81\x90\x04\x81\x11\x90\x03\x90V[`\0\x80\x82\x12\x80a8;WPg\x1B\xC1mgN\xC8\0\0\x82\x13[\x15a8YW`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a8zW`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81g\x1B\xC1mgN\xC8\0\0\x03a8\xA2W`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x15a8\xADW\x91\x90PV[`\0g\r\xE0\xB6\xB3\xA7d\0\0\x83\x12a8\xD5Wa8\xD0\x83g\x1B\xC1mgN\xC8\0\0aC\x0EV[a8\xD7V[\x82[\x90P`\0a8\xED\x82g\x1B\xC1mgN\xC8\0\0a>\xB9V[\x90P\x80`\0\x03a9\x10W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a9\x1B\x82a<\xDEV[\x90P`\0c;\x9A\xCA\0a9Fa9Aa9;g\x1B\xC1mgN\xC8\0\0aC\xC3V[\x85a:\xA2V[a7\x80V[a9P\x91\x90aB\xB0V[\x90P`\0\x80a9g\x83g\x03\xC1f\\z\xAB \0a:\xA2V[a9y\x90g \x05\xFEO&\x8E\xA0\0aC\x9BV[\x90P`\0a9\xA9\x84a9\x92\x86f\x9F2u$b\xA0\0a:\xA2V[a9\xA4\x90g\r\xC5R\x7Fd, \0aC\x9BV[a:\xA2V[a9\xBB\x90g\r\xE0\xB6\xB3\xA7d\0\0aC\x9BV[\x90Pa9\xDFg\t\xD0(\xCCo _\xFF\x19\x85a9\xD5\x85\x85a>\xB9V[a9\xA4\x91\x90aC\x0EV[\x92PPP`\0[`\x02\x81\x10\x15a:zW`\0\x86a9\xFB\x84a:\xCBV[a:\x05\x91\x90aC\x0EV[\x90P`\0a:\x13\x84\x85a:\xA2V[a:\x1C\x90aC\xC3V[\x90P`\0a:)\x82a1\x87V[\x90P`\0a:7\x86\x85a:\xA2V[a:Ig\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x84a:\xA2V[a:S\x91\x90aC\x0EV[\x90Pa:_\x84\x82a>\xB9V[a:i\x90\x87aC\x9BV[\x95P\x84`\x01\x01\x94PPPPPa9\xE6V[g\r\xE0\xB6\xB3\xA7d\0\0\x88\x12a:\x97Wa:\x92\x82aC\xC3V[a3\x97V[P\x96\x95PPPPPPV[`\0a0\xE1\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a>\xCAV[`\0a0\xE1a:\xC6\x84\x84a4\x0CV[a<\xDEV[`\0\x81`\0\x03a:\xE4WPg\r\xE0\xB6\xB3\xA7d\0\0\x91\x90PV[gV\x98\xEE\xF0fp\0\0\x82\x12a:\xFBWP`\0\x91\x90PV[a;\x0CgV\x98\xEE\xF0fp\0\0aC\xC3V[\x82\x13a;!WPg\x1B\xC1mgN\xC8\0\0\x91\x90PV[`\0a;,\x83a>\xE9V[\x90P`\0a;eg\r\xE0\xB6\xB3\xA7d\0\0a;N\x84g\x1B\xC1mgN\xC8\0\0a0\xCCV[a;`\x90g\r\xE0\xB6\xB3\xA7d\0\0aC\x9BV[a>\xB9V[\x90P`\0\x80\x82a;\xC1\x81a;\xAE\x81a;\x9C\x81a;\x89\x81g\x02_\x0F\xE1\x05\xA3\x14\0a:\xA2V[a9\xA4\x90g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19aC\x9BV[a9\xA4\x90g\x14\xA8EL\x19\xE1\xAC\0aC\x9BV[a9\xA4\x90g\x0F\xC1\x0E\x01W\x82w\xFF\x19aC\x9BV[a;\xD3\x90g\x03\xDE\xBD\x08;\x8C|\0aC\x9BV[\x91P\x83\x90Pa<;\x81a<)\x81a<\x17\x81a<\x05\x81a;\xF2\x81\x8Ba:\xA2V[a9\xA4\x90g\x02\x95\xD4\0\xEA2W\xFF\x19aC\x9BV[a9\xA4\x90g\x01W\xD8\xB2\xEC\xC7\x08\0aC\x9BV[a9\xA4\x90g\x051\n\xA7\xD5!0\0aC\x9BV[a9\xA4\x90g\r\xE0\xCC=\x15a\0\0aC\x9BV[\x91P\x81g\x11\x90\0\xAB\x10\x0F\xFC\0a<Q\x87\x88a:\xA2V[a<]\x90`\0\x19aB\xB0V[a<g\x91\x90aC\x0EV[a<q\x91\x90aC\x9BV[\x92PP`\0a<\x7F\x83a1\x87V[\x90P`\0a<\x8D\x85\x83a:\xA2V[\x90P`\0\x88\x12a<\x9DW\x80a3\x97V[a3\x97\x81g\x1B\xC1mgN\xC8\0\0aC\x0EV[\x80Qjconsole.log` \x83\x01`\0\x80\x84\x83\x85Z\xFAPPPPPV[`\0\x80a1Y\x83`\x02aB\xB0V[`\0\x80\x82\x13a=\x1BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x08\xDEV[`\0``a=(\x84a?$V[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[`\0a0\xE1\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x05\x85\x14\x17\x16a>\xE2W`\0\x80\xFD[\x05\x92\x91PPV[`\0`\x01`\xFF\x1B\x82\x03a?\x0FW`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x12\x15a? WP\x19`\x01\x01\x90V[P\x90V[`\0\x80\x82\x11a?aW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x08\xDEV[P`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x82\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`\0` \x82\x84\x03\x12\x15a@1Wa@1a?\xCCV[P5\x91\x90PV[\x80\x15\x15\x81\x14a4`W`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15a@^Wa@^a?\xCCV[\x835a@i\x81a@8V[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a@\x94Wa@\x94a?\xCCV[PP\x805\x92` \x90\x91\x015\x91PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a@\xBCWa@\xBCa?\xCCV[\x845a@\xC7\x81a@8V[\x96` \x86\x015\x96P`@\x86\x015\x95``\x015\x94P\x92PPPV[`\0` \x82\x84\x03\x12\x15a@\xF6Wa@\xF6a?\xCCV[\x815`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\r\xE9W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15aA#WaA#a?\xCCV[\x825aA.\x81a@8V[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15aAbW` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01aAFV[P`\0` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R`\0a0\xE1` \x83\x01\x84aA<V[`\0\x80`\0``\x84\x86\x03\x12\x15aA\xADWaA\xADa?\xCCV[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a0\xE4Wa0\xE4aA\xC4V[\x81\x81\x03\x81\x81\x11\x15a0\xE4Wa0\xE4aA\xC4V[`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x81R\x91\x90\x92\x16` \x82\x01R`@\x81\x01\x91\x90\x91R``\x01\x90V[`\0` \x82\x84\x03\x12\x15aB9WaB9a?\xCCV[\x81Qa\r\xE9\x81a@8V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a0\xE4Wa0\xE4aA\xC4V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82aB\x80WaB\x80aB[V[P\x04\x90V[` \x80\x82R`\x11\x90\x82\x01Rp\x15\\\x19\x18]\x19H\x19[\x99\x08\x1C\x18\\\xDC\xD9Y`z\x1B`@\x82\x01R``\x01\x90V[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15aB\xCCWaB\xCCaA\xC4V[\x81\x81\x05\x83\x14\x82\x15\x17a0\xE4Wa0\xE4aA\xC4V[`\0\x82aB\xEFWaB\xEFaB[V[`\x01`\xFF\x1B\x82\x14`\0\x19\x84\x14\x16\x15aC\tWaC\taA\xC4V[P\x05\x90V[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15aC.WaC.aA\xC4V[P\x92\x91PPV[`@\x81R`\0aCH`@\x83\x01\x85aA<V[\x90P\x82` \x83\x01R\x93\x92PPPV[`\0`\xC0\x82\x01\x90P\x82Q\x82R` \x83\x01Q` \x83\x01R`@\x83\x01Q`@\x83\x01R``\x83\x01Q``\x83\x01R`\x80\x83\x01Q`\x80\x83\x01R`\xA0\x83\x01Q`\xA0\x83\x01R\x92\x91PPV[\x80\x82\x01\x82\x81\x12`\0\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15aC\xBBWaC\xBBaA\xC4V[PP\x92\x91PPV[`\0`\x01`\xFF\x1B\x82\x01aC\xD8WaC\xD8aA\xC4V[P`\0\x03\x90V[`\0`\xC0\x82\x84\x03\x12\x15aC\xF4WaC\xF4a?\xCCV[`@Q`\xC0\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15aD%WcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x80`@RP\x82Q\x81R` \x83\x01Q` \x82\x01R`@\x83\x01Q`@\x82\x01R``\x83\x01Q``\x82\x01R`\x80\x83\x01Q`\x80\x82\x01R`\xA0\x83\x01Q`\xA0\x82\x01R\x80\x91PP\x92\x91PPV\xFETarget contract does not contains\xEBa\xB7\x83\x9B\xB3\xC27\xA0\xB3\xAE\xAB\x18iQ\0\xBCQ.q9\xF0\xE3@e]\xD7\x02\xDA*\xA1";
    /// The bytecode of the contract.
    pub static RMM_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x02\xC1W`\x005`\xE0\x1C\x80c\x8DR\xA1\xFC\x11a\x01\x9EW\x80c\xC5)\x87\xCF\x11a\x01\x10W\x80c\xDCv\xFA\xBC\x11a\0\xD4W\x80c\xDCv\xFA\xBC\x14a\x05\xB9W\x80c\xF2\xD1\x92/\x14a\x04<W\x80c\xF3\xA8\xEF\xE3\x14a\x05\xC1W\x80c\xF9\xA1\xC8Z\x14a\x05\xC9W\x80c\xFA\xDF\xA6[\x14a\x05\xDCW\x80c\xFE\xD3\xDF\xDA\x14a\x05\xE5Wa\x02\xC1V[\x80c\xC5)\x87\xCF\x14a\x05{W\x80c\xCC\xD1\xE4\xBE\x14a\x05\x83W\x80c\xCFx\xE4\x8F\x14a\x05\x96W\x80c\xCF\xC4\xAFU\x14a\x05\xA9W\x80c\xD4\xCA\xDFh\x14a\x05\xB1Wa\x02\xC1V[\x80c\xAF\xDF1\xCD\x11a\x01bW\x80c\xAF\xDF1\xCD\x14a\x05?W\x80c\xB7\xD1\x9F\xC4\x14a\x05GW\x80c\xBB\x04\x98\xDE\x14a\x05ZW\x80c\xBC\xC1}\xC7\x14a\x05bW\x80c\xC0\xFF\x1A\x15\x14a\x05jW\x80c\xC1e&\x12\x14a\x05rWa\x02\xC1V[\x80c\x8DR\xA1\xFC\x14a\x04\xE0W\x80c\x99\x87\xFEd\x14a\x04\xF3W\x80c\x9C\x9D\xA9\xEA\x14a\x05\x06W\x80c\x9E\x192\xD8\x14a\x05\x19W\x80c\xA5\x9C\x18o\x14a\x05,Wa\x02\xC1V[\x80cT\xCF*\xEB\x11a\x027W\x80cp\xA0\x821\x11a\x01\xFBW\x80cp\xA0\x821\x14a\x04jW\x80cvp\x166\x14a\x04\x8AW\x80c}iS~\x14a\x04\x9DW\x80c\x7F\x0EL\x8C\x14a\x04\xB0W\x80c\x87kU\xF1\x14a\x04\xC3W\x80c\x89\xE2\xC2Y\x14a\x04\xD8Wa\x02\xC1V[\x80cT\xCF*\xEB\x14a\x04+W\x80cU\x9D\x16\x02\x14a\x044W\x80cYT,\xA9\x14a\x04<W\x80c]\x84\n\xE5\x14a\x04OW\x80c^aZk\x14a\x04bWa\x02\xC1V[\x80c\x16\xDC\x16[\x11a\x02\x89W\x80c\x16\xDC\x16[\x14a\x03\xA9W\x80c4\xE1\x99\x07\x14a\x03\xD4W\x80c69\xAA2\x14a\x03\xE9W\x80c7\xAAs\x81\x14a\x03\xFCW\x80c>\x1E3\x92\x14a\x04\x0FW\x80cAf\xD3H\x14a\x04\x18Wa\x02\xC1V[\x80c\x02\xC2\xE5]\x14a\x03&W\x80c\x04\xAF\xA8\"\x14a\x03SW\x80c\x08\xEA\xBD\xDA\x14a\x03\x81W\x80c\t\x10\xA5\x10\x14a\x03\x98W\x80c\x15w\x0F\x92\x14a\x03\xA0W[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x039a\x0346`\x04a@\x1CV[a\x05\xEDV[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01[`@Q\x80\x91\x03\x90\xF3[a\x03fa\x03a6`\x04a@FV[a\x08\x90V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x03JV[a\x03\x8A`\x03T\x81V[`@Q\x90\x81R` \x01a\x03JV[`\x05Ta\x03\x8AV[a\x03\x8A`\x05T\x81V[`\0Ta\x03\xBC\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x03JV[a\x03\xE7a\x03\xE26`\x04a@\x1CV[a\n\xFCV[\0[a\x03fa\x03\xF76`\x04a@~V[a\x0BPV[a\x03fa\x04\n6`\x04a@~V[a\x0BnV[a\x03\x8A`\rT\x81V[a\x03\x8Aa\x04&6`\x04a@\x1CV[a\x0COV[a\x03\x8A`\x02T\x81V[`\x03Ta\x03\x8AV[a\x03\x8Aa\x04J6`\x04a@\xA3V[a\x0C\x81V[a\x03\x8Aa\x04]6`\x04a@FV[a\x0C\x8FV[a\x03fa\r\xF0V[a\x03\x8Aa\x04x6`\x04a@\xE1V[`\x06` R`\0\x90\x81R`@\x90 T\x81V[a\x03fa\x04\x986`\x04aA\rV[a\x0E\x18V[a\x039a\x04\xAB6`\x04a@\x1CV[a\x11}V[a\x03\xE7a\x04\xBE6`\x04a@~V[a\x13\xEFV[a\x04\xCBa\x14\xB4V[`@Qa\x03J\x91\x90aA\x82V[a\x03\x8Aa\x14\xFCV[a\x03\xE7a\x04\xEE6`\x04a@~V[a\x15!V[a\x03\xE7a\x05\x016`\x04a@~V[a\x15\xADV[a\x039a\x05\x146`\x04a@\x1CV[a\x169V[a\x03\xE7a\x05'6`\x04aA\x95V[a\x18\xC0V[a\x039a\x05:6`\x04a@~V[a\x1B\xDBV[a\x03\x8Aa\x1D\xB7V[`\x01Ta\x03\xBC\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x03\x8Aa\x1E\"V[a\x03\xE7a\x1EYV[a\x03\x8Aa\x1E\xA6V[a\x03\x8A`\x08T\x81V[a\x03\x8Aa\x1E\xC1V[a\x039a\x05\x916`\x04a@\x1CV[a\x1F,V[a\x03\x8Aa\x05\xA46`\x04a@FV[a!\x9EV[a\x03\x8Aa\"lV[`\x02Ta\x03\x8AV[a\x03\x8Aa\"\xD7V[a\x03\x8Aa\"\xFFV[a\x039a\x05\xD76`\x04a@~V[a#/V[a\x03\x8A`\x04T\x81V[`\x04Ta\x03\x8AV[`\0\x80`\0\x80`\0a\x05\xFDa\r\xF0V[\x92P\x92P\x92P`\0a\x06\x16`\x03T`\x05T\x86\x86\x86a$lV[\x90P`\0a\x064\x88`\x03Ta\x06+\x91\x90aA\xDAV[\x83\x87\x87\x87a$\xF0V[\x90P`\0a\x06E\x82\x84\x88\x88\x88a%0V[\x90P`\0`\x04T\x82a\x06W\x91\x90aA\xEDV[\x90P`\0`\x05T\x84a\x06i\x91\x90aA\xEDV[\x90P\x83`\x05\x81\x90UP\x8A`\x03`\0\x82\x82Ta\x06\x84\x91\x90aA\xDAV[\x92PP\x81\x90UP\x81`\x04`\0\x82\x82Ta\x06\x9D\x91\x90aA\xDAV[\x92PP\x81\x90UP`\0\x80T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c#\xB8r\xDD30\x8E`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x06\xE7\x93\x92\x91\x90aB\0V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x07?W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` aDk\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x07SW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07w\x91\x90aB$V[P`\x01T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\x07\xAC\x903\x900\x90\x87\x90`\x04\x01aB\0V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x08\x04W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` aDk\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x08\x18W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08<\x91\x90aB$V[P`@\x80Q\x85\x81R` \x81\x01\x8D\x90R\x90\x81\x01\x83\x90R3\x90\x7F\xBE\xB3\x88W\x86\xD67\xA4t\xCB\xC2\x87\xC0\xA4E\x87#\x163\xA0w\xF0\xBD05MZK\x18\x99o\xCE\x90``\x01[`@Q\x80\x91\x03\x90\xA2\x9A\x90\x99P\x97PPPPPPPPV[`\0\x80`\0`\x05T`\0\x14a\x08\xE7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01Rw\x14\x1B\xDB\xDB\x08\x18[\x1C\x99XY\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`B\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x80`\0a\x08\xF4a\r\xF0V[\x92P\x92P\x92P\x88\x15a\t&W\x87\x95Pa\t\x10\x86\x88\x85\x85\x85a$\xF0V[\x93Pa\t\x1F\x84\x88\x85\x85\x85a%0V[\x94Pa\tHV[\x87\x94Pa\t6\x85\x88\x85\x85\x85a%RV[\x93Pa\tE\x84\x88\x85\x85\x85a%hV[\x95P[`\x05\x84\x90U`\x03\x86\x90U`\x04\x85\x81U3`\0\x81\x81R`\x06` R`@\x80\x82 \x88\x90U\x90T\x90Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x92c#\xB8r\xDD\x92a\t\x9A\x92\x90\x910\x91\x8C\x91\x01aB\0V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\t\xF2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` aDk\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\n\x06W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n*\x91\x90aB$V[P`\x01T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\n_\x903\x900\x90\x8A\x90`\x04\x01aB\0V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\n\xB7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` aDk\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\n\xCBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\xEF\x91\x90aB$V[PPPP\x93P\x93P\x93\x90PV[g\r\xE0\xB6\xB3\xA7d\0\0\x81\x10a\x0BKW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rt\t\xCC\xAE\xE4\x0En\xEC.\x04\x0C\xCC\xAC\xA4\x0E\x8D\xED\xE4\r\r,\xED`[\x1B`D\x82\x01R`d\x01a\x08\xDEV[`\x02UV[`\0\x80`\0a\x0Ba`\x01\x86\x86a\x08\x90V[\x92P\x92P\x92P\x92P\x92P\x92V[`\0\x80`\0\x80`\0\x80a\x0B\x7Fa\r\xF0V[\x92P\x92P\x92P`\0a\x0B\x94\x89\x89\x86\x86\x86a$\xF0V[\x90Pa\x0B\xC6`@Q\x80`@\x01`@R\x80`\x0E\x81R` \x01mbase liquidity`\x90\x1B\x81RP\x82a%\x92V[a\x0B\xD3\x81\x89\x86\x86\x86a%0V[\x95Pa\x0B\xFF`@Q\x80`@\x01`@R\x80`\x08\x81R` \x01ginitialY`\xC0\x1B\x81RP\x87a%\x92V[a\x0C\r\x89\x87\x83\x87\x87\x87a%\xDBV[\x94Pa\x0CA`@Q\x80`@\x01`@R\x80`\x10\x81R` \x01oinitialLiquidity`\x80\x1B\x81RP\x86a%\x92V[\x88\x96PPPPP\x92P\x92P\x92V[`\0\x80`\0\x80a\x0C]a\r\xF0V[\x92P\x92P\x92Pa\x0Cx`\x03T`\x04T`\x05T\x86\x86\x86\x8Ba&\xABV[\x95\x94PPPPPV[`\0a\x0Cx\x85\x85\x85\x85a&\xE7V[`\0a\x0C\xB5`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[a\x0C\xBDa\r\xF0V[`@\x84\x01\x81\x90R` \x84\x01\x82\x90R\x82\x84R`\x03T`\0\x93a\x0C\xE2\x93\x91\x92\x89\x92\x90a$lV[\x90P`\0g\r\xE0\xB6\xB3\xA7d\0\0`\x02Tg\r\xE0\xB6\xB3\xA7d\0\0a\r\x05\x91\x90aA\xEDV[a\r\x0F\x90\x87aBDV[a\r\x19\x91\x90aBqV[\x90P\x86\x15a\r\x8BW`\0a\r<\x82\x84\x86`\0\x01Q\x87` \x01Q\x88`@\x01Qa$\xF0V[\x90P`\0a\ro\x87`\x03Ta\rQ\x91\x90aA\xDAV[`\x04Ta\r^\x85\x8CaA\xDAV[\x88Q` \x8A\x01Q`@\x8B\x01Qa-\xD7V[\x90P\x80`\x04Ta\r\x7F\x91\x90aA\xEDV[\x95PPPPPPa\r\xE9V[`\0a\r\xA6\x82\x84\x86`\0\x01Q\x87` \x01Q\x88`@\x01Qa%RV[\x90P`\0a\r\xD9`\x03T\x88`\x04Ta\r\xBE\x91\x90aA\xDAV[a\r\xC8\x85\x8CaA\xDAV[\x88Q` \x8A\x01Q`@\x8B\x01Qa.\x81V[\x90P\x80`\x03Ta\r\x7F\x91\x90aA\xEDV[\x93\x92PPPV[`\0\x80`\0a\r\xFDa\x1E\xC1V[a\x0E\x05a\x1D\xB7V[a\x0E\ra\"lV[\x92P\x92P\x92P\x90\x91\x92V[`\0\x80`\0\x80`\x05T\x11a\x0EeW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rs\x14\x1B\xDB\xDB\x08\x1B\x9B\xDD\x08\x1A[\x9A]\x1AX[\x1A^\x99Y`b\x1B`D\x82\x01R`d\x01a\x08\xDEV[`\0\x80`\0a\x0Era\r\xF0V[\x92P\x92P\x92P`\0a\x0E\x8B`\x03T`\x05T\x86\x86\x86a$lV[\x90P\x88\x15a\x0E\xE3W\x87\x96P`\0a\x0E\xA9\x88`\x03Ta\x06+\x91\x90aA\xDAV[\x90P`\0a\x0E\xBA\x82\x84\x88\x88\x88a%0V[\x90P`\x04T\x81a\x0E\xCA\x91\x90aA\xEDV[\x97P`\x05T\x82a\x0E\xDA\x91\x90aA\xEDV[\x96PPPa\x0F8V[\x87\x95P`\0a\x0F\x02\x87`\x04Ta\x0E\xF9\x91\x90aA\xDAV[\x83\x87\x87\x87a%RV[\x90P`\0a\x0F\x13\x82\x84\x88\x88\x88a%hV[\x90P`\x03T\x81a\x0F#\x91\x90aA\xEDV[\x98P`\x05T\x82a\x0F3\x91\x90aA\xEDV[\x96PPP[\x84`\x05`\0\x82\x82Ta\x0FJ\x91\x90aA\xDAV[\x92PP\x81\x90UP\x86`\x03`\0\x82\x82Ta\x0Fc\x91\x90aA\xDAV[\x92PP\x81\x90UP\x85`\x04`\0\x82\x82Ta\x0F|\x91\x90aA\xDAV[\x90\x91UPP3`\0\x90\x81R`\x06` R`@\x81 \x80T\x87\x92\x90a\x0F\xA0\x90\x84\x90aA\xDAV[\x90\x91UPP`\0T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\x0F\xD9\x903\x900\x90\x8C\x90`\x04\x01aB\0V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x101W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` aDk\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x10EW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10i\x91\x90aB$V[P`\x01T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\x10\x9E\x903\x900\x90\x8B\x90`\x04\x01aB\0V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x10\xF6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` aDk\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x11\nW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11.\x91\x90aB$V[P`@\x80Q\x86\x81R` \x81\x01\x89\x90R\x90\x81\x01\x87\x90R3\x90\x7F\xBE\xB3\x88W\x86\xD67\xA4t\xCB\xC2\x87\xC0\xA4E\x87#\x163\xA0w\xF0\xBD05MZK\x18\x99o\xCE\x90``\x01`@Q\x80\x91\x03\x90\xA2PPPP\x92P\x92P\x92V[`\0\x80`\0\x80`\0a\x11\x8Da\r\xF0V[\x92P\x92P\x92P`\0a\x11\xA6`\x03T`\x05T\x86\x86\x86a$lV[\x90P`\0a\x11\xBB\x88`\x03Ta\x0E\xF9\x91\x90aA\xEDV[\x90P`\0a\x11\xCC\x82\x84\x88\x88\x88a%hV[\x90P`\0\x81`\x03Ta\x11\xDE\x91\x90aA\xEDV[\x90P`\0\x83`\x05Ta\x11\xF0\x91\x90aA\xEDV[\x90P\x83`\x05\x81\x90UP\x81`\x03`\0\x82\x82Ta\x12\x0B\x91\x90aA\xEDV[\x92PP\x81\x90UP\x8A`\x04`\0\x82\x82Ta\x12$\x91\x90aA\xEDV[\x90\x91UPP`\0T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x12\xB3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` aDk\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x12\xC7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\xEB\x91\x90aB$V[P`\x01T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x8D\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x13vW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` aDk\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x13\x8AW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\xAE\x91\x90aB$V[P`@\x80Q\x85\x81R` \x81\x01\x84\x90R\x90\x81\x01\x8C\x90R3\x90\x7FY\xC3\xA0\xB6\x0Cj\xB7\xDE\xB6.\x14@\xC9\xE7$A\xDBm\xB7\xDF\xE5\x14\xDB\xA8\xCB\x18\xE6\x0C\r\x89n\xFA\x90``\x01a\x08yV[B\x81\x11a\x14\x0EW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xDE\x90aB\x85V[a\x14\x16a/!V[`\0\x82`\x11T\x11a\x143W`\x11Ta\x14.\x90\x84aA\xEDV[a\x14AV[\x82`\x11Ta\x14A\x91\x90aA\xEDV[\x90Pa\x14MB\x83aA\xEDV[a\x14W\x90\x82aBqV[`\x14U`\x12\x83\x90U`\x15\x82\x90U`\0\x80Q` aD\x8B\x839\x81Q\x91Ra\x14{a\x1D\xB7V[a\x14\x83a\x1E\xC1V[a\x14\x8Ba\"lV[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01RB``\x82\x01R`\x80\x01`@Q\x80\x91\x03\x90\xA1PPPV[``a\x14\xBEa\x1D\xB7V[a\x14\xC6a\x1E\xC1V[a\x14\xCEa\"lV[`@\x80Q` \x81\x01\x94\x90\x94R\x83\x01\x91\x90\x91R``\x82\x01R`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[`\0a\x15\x1C`\x04T`\x05Ta\x15\x0Fa\x1E\xC1V[a\x15\x17a\x1D\xB7V[a/2V[\x90P\x90V[B\x81\x11a\x15@W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xDE\x90aB\x85V[a\x15Ha/\xA2V[`\0\x82`\x0CT\x11a\x15eW`\x0CTa\x15`\x90\x84aA\xEDV[a\x15sV[\x82`\x0CTa\x15s\x91\x90aA\xEDV[\x90Pa\x15\x7FB\x83aA\xEDV[a\x15\x89\x90\x82aBqV[`\x0FU`\r\x83\x90U`\x10\x82\x90U`\0\x80Q` aD\x8B\x839\x81Q\x91Ra\x14{a\x1D\xB7V[B\x81\x11a\x15\xCCW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xDE\x90aB\x85V[a\x15\xD4a/\xB3V[`\0\x82`\x07T\x11a\x15\xF1W`\x07Ta\x15\xEC\x90\x84aA\xEDV[a\x15\xFFV[\x82`\x07Ta\x15\xFF\x91\x90aA\xEDV[\x90Pa\x16\x0BB\x83aA\xEDV[a\x16\x15\x90\x82aBqV[`\nU`\x08\x83\x90U`\x0B\x82\x90U`\0\x80Q` aD\x8B\x839\x81Q\x91Ra\x14{a\x1D\xB7V[`\0\x80`\0\x80`\0a\x16Ia\r\xF0V[\x92P\x92P\x92P`\0a\x16b`\x03T`\x05T\x86\x86\x86a$lV[\x90P`\0a\x16w\x88`\x04Ta\x0E\xF9\x91\x90aA\xDAV[\x90P`\0a\x16\x88\x82\x84\x88\x88\x88a%hV[\x90P`\0`\x03T\x82a\x16\x9A\x91\x90aA\xEDV[\x90P`\0`\x05T\x84a\x16\xAC\x91\x90aA\xEDV[\x90P\x83`\x05\x81\x90UP\x81`\x03`\0\x82\x82Ta\x16\xC7\x91\x90aA\xDAV[\x92PP\x81\x90UP\x8A`\x04`\0\x82\x82Ta\x16\xE0\x91\x90aA\xDAV[\x90\x91UPP`\0T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\x17\x19\x903\x900\x90\x87\x90`\x04\x01aB\0V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x17qW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` aDk\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x17\x85W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17\xA9\x91\x90aB$V[P`\x01`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c#\xB8r\xDD30\x8E`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x17\xEF\x93\x92\x91\x90aB\0V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x18GW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` aDk\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x18[W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18\x7F\x91\x90aB$V[P`@\x80Q\x85\x81R` \x81\x01\x84\x90R\x90\x81\x01\x8C\x90R3\x90\x7F\xBE\xB3\x88W\x86\xD67\xA4t\xCB\xC2\x87\xC0\xA4E\x87#\x163\xA0w\xF0\xBD05MZK\x18\x99o\xCE\x90``\x01a\x08yV[`\x05T\x15a\x19\x0BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01Rw\x14\x1B\xDB\xDB\x08\x18[\x1C\x99XY\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`B\x1B`D\x82\x01R`d\x01a\x08\xDEV[`\0\x80`\0a\x19\x18a\r\xF0V[\x92P\x92P\x92P`\0a\x19/\x87\x87\x87\x87\x87\x87\x8Ba&\xABV[\x90P`\0a\x19B\x88\x88\x88\x88\x88\x88\x8Ea/\xC4V[\x90P`\0a\x19U\x89\x89\x89\x89\x89\x89\x8Ea0\0V[\x90P\x82`\x18\x19\x13\x80a\x19gWP`\x19\x83\x13[\x15a\x19\x8FW`@Qc\x08\xD4\xF3\xC1`\xE3\x1B\x81R`\x04\x81\x01\x88\x90R`$\x81\x01\x84\x90R`D\x01a\x08\xDEV[\x81`\x18\x19\x13\x80a\x19\x9FWP`\x19\x82\x13[\x15a\x19\xC7W`@Qc\xC7KR\x13`\xE0\x1B\x81R`\x04\x81\x01\x8A\x90R`$\x81\x01\x83\x90R`D\x01a\x08\xDEV[\x80`\x18\x19\x13\x80a\x19\xD7WP`\x19\x81\x13[\x15a\x19\xFFW`@Qc:M\x97\x7F`\xE2\x1B\x81R`\x04\x81\x01\x89\x90R`$\x81\x01\x82\x90R`D\x01a\x08\xDEV[\x86`\x05\x81\x90UP\x88`\x03`\0\x82\x82Ta\x1A\x18\x91\x90aA\xDAV[\x92PP\x81\x90UP\x87`\x04`\0\x82\x82Ta\x1A1\x91\x90aA\xDAV[\x90\x91UPP3`\0\x81\x81R`\x06` R`@\x80\x82 \x8A\x90U\x90T\x90Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91c#\xB8r\xDD\x91a\x1Az\x91\x900\x90\x8E\x90`\x04\x01aB\0V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1A\xD2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` aDk\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x1A\xE6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B\n\x91\x90aB$V[P`\x01T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\x1B?\x903\x900\x90\x8D\x90`\x04\x01aB\0V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1B\x97W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` aDk\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x1B\xABW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B\xCF\x91\x90aB$V[PPPPPPPPPPV[`\0\x80`\0\x80`\0a\x1B\xEBa\r\xF0V[\x92P\x92P\x92P`\0a\x1C\0\x88\x88\x86\x86\x86a%RV[\x90P`\0a\x1C\x11\x82\x89\x87\x87\x87a%hV[`\x05\x83\x90U`\x03\x81\x90U`\x04\x8A\x81U`\0T`@Qc#\xB8r\xDD`\xE0\x1B\x81R\x92\x93P`\x01`\x01`\xA0\x1B\x03\x16\x91c#\xB8r\xDD\x91a\x1CS\x913\x910\x91\x87\x91\x01aB\0V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1C\xABW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` aDk\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x1C\xBFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C\xE3\x91\x90aB$V[P`\x01T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\x1D\x18\x903\x900\x90\x8E\x90`\x04\x01aB\0V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1DpW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` aDk\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x1D\x84W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1D\xA8\x91\x90aB$V[P\x90\x98\x90\x97P\x95PPPPPPV[`\0`\x0BTB\x10a\x1D\xC9WP`\x08T\x90V[`\x08T`\x07T\x11a\x1D\xFBW`\nT`\tTa\x1D\xE4\x90BaA\xEDV[a\x1D\xEE\x91\x90aBDV[`\x07Ta\x15\x1C\x91\x90aA\xDAV[`\nT`\tTa\x1E\x0B\x90BaA\xEDV[a\x1E\x15\x91\x90aBDV[`\x07Ta\x15\x1C\x91\x90aA\xEDV[`\0`\x04Tg\r\xE0\xB6\xB3\xA7d\0\0a\x1E8a\"\xD7V[`\x03Ta\x1EE\x91\x90aBDV[a\x1EO\x91\x90aBqV[a\x15\x1C\x91\x90aA\xDAV[`\0\x80Q` aD\x8B\x839\x81Q\x91Ra\x1Epa\x1D\xB7V[a\x1Exa\x1E\xC1V[a\x1E\x80a\"lV[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01RB``\x82\x01R`\x80\x01`@Q\x80\x91\x03\x90\xA1V[`\0a\x15\x1C`\x03T`\x05T`\x04Ta\x1E\xBCa\x1E\xC1V[a0<V[`\0`\x10TB\x10a\x1E\xD3WP`\rT\x90V[`\rT`\x0CT\x11a\x1F\x05W`\x0FT`\x0ETa\x1E\xEE\x90BaA\xEDV[a\x1E\xF8\x91\x90aBDV[`\x0CTa\x15\x1C\x91\x90aA\xDAV[`\x0FT`\x0ETa\x1F\x15\x90BaA\xEDV[a\x1F\x1F\x91\x90aBDV[`\x0CTa\x15\x1C\x91\x90aA\xEDV[`\0\x80`\0\x80`\0a\x1F<a\r\xF0V[\x92P\x92P\x92P`\0a\x1FU`\x03T`\x05T\x86\x86\x86a$lV[\x90P`\0a\x1Fj\x88`\x03Ta\x06+\x91\x90aA\xEDV[\x90P`\0a\x1F{\x82\x84\x88\x88\x88a%0V[\x90P`\0\x81`\x04Ta\x1F\x8D\x91\x90aA\xEDV[\x90P`\0\x83`\x05Ta\x1F\x9F\x91\x90aA\xEDV[\x90P\x83`\x05\x81\x90UP\x8A`\x03`\0\x82\x82Ta\x1F\xBA\x91\x90aA\xEDV[\x92PP\x81\x90UP\x81`\x04`\0\x82\x82Ta\x1F\xD3\x91\x90aA\xEDV[\x90\x91UPP`\0T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x8D\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a bW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` aDk\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a vW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a \x9A\x91\x90aB$V[P`\x01T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a!%W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` aDk\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a!9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a!]\x91\x90aB$V[P`@\x80Q\x85\x81R` \x81\x01\x8D\x90R\x90\x81\x01\x83\x90R3\x90\x7FY\xC3\xA0\xB6\x0Cj\xB7\xDE\xB6.\x14@\xC9\xE7$A\xDBm\xB7\xDF\xE5\x14\xDB\xA8\xCB\x18\xE6\x0C\r\x89n\xFA\x90``\x01a\x08yV[`\0a!\xC4`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[a!\xCCa\r\xF0V[`@\x84\x01\x81\x90R` \x84\x01\x82\x90R\x82\x84R`\x03T`\0\x93a!\xF1\x93\x91\x92\x89\x92\x90a$lV[\x90P`\0g\r\xE0\xB6\xB3\xA7d\0\0`\x02Tg\r\xE0\xB6\xB3\xA7d\0\0a\"\x14\x91\x90aA\xEDV[a\"\x1E\x90\x87aBDV[a\"(\x91\x90aBqV[\x90P\x86\x15a\"SWa\"I\x81\x83\x85`\0\x01Q\x86` \x01Q\x87`@\x01Qa$\xF0V[\x93PPPPa\r\xE9V[a\"I\x81\x83\x85`\0\x01Q\x86` \x01Q\x87`@\x01Qa%RV[`\0`\x15TB\x10a\"~WP`\x12T\x90V[`\x12T`\x11T\x11a\"\xB0W`\x14T`\x13Ta\"\x99\x90BaA\xEDV[a\"\xA3\x91\x90aBDV[`\x11Ta\x15\x1C\x91\x90aA\xDAV[`\x14T`\x13Ta\"\xC0\x90BaA\xEDV[a\"\xCA\x91\x90aBDV[`\x11Ta\x15\x1C\x91\x90aA\xEDV[`\0a\x15\x1C`\x03T`\x05Ta\"\xEAa\x1E\xC1V[a\"\xF2a\x1D\xB7V[a\"\xFAa\"lV[a$lV[`\0\x80`\0\x80a#\ra\r\xF0V[\x92P\x92P\x92Pa#'`\x03T`\x04T`\x05T\x86\x86\x86a%\xDBV[\x93PPPP\x90V[`\0\x80`\0\x80`\0a#?a\r\xF0V[\x92P\x92P\x92P`\0a#T\x88\x88\x86\x86\x86a$\xF0V[\x90P`\0a#e\x82\x89\x87\x87\x87a%0V[`\x05\x83\x90U`\x03\x8A\x90U`\x04\x81\x81U`\0T`@Qc#\xB8r\xDD`\xE0\x1B\x81R\x92\x93P`\x01`\x01`\xA0\x1B\x03\x16\x91c#\xB8r\xDD\x91a#\xA7\x913\x910\x91\x8F\x91\x01aB\0V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a#\xFFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` aDk\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a$\x13W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a$7\x91\x90aB$V[P`\x01T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\x1D\x18\x903\x900\x90\x86\x90`\x04\x01aB\0V[`\0\x80a$y\x84\x84a0lV[\x90P`\0a$\x87\x85\x85a0\x9AV[\x90P`\0a$\x95\x89\x89a0\xCCV[\x90Pa$\xE3\x87a$\xDE\x84g\r\xE0\xB6\xB3\xA7d\0\0\x87a$\xBBa$\xB6\x88\x84aA\xEDV[a0\xEAV[a$\xC5\x91\x90aB\xB0V[a$\xCF\x91\x90aB\xE0V[a$\xD9\x91\x90aC\x0EV[a1\x87V[a30V[\x99\x98PPPPPPPPPV[`\0\x80a%\x07a%\x02\x87\x87\x87\x87a3EV[a3\xA3V[a%\x19\x90g\r\xE0\xB6\xB3\xA7d\0\0aC\x0EV[\x90Pa%%\x87\x82a4\x0CV[\x97\x96PPPPPPPV[`\0\x80a%Ba%\x02\x87\x87\x87\x87a4!V[\x90Pa%%\x85a$\xDE\x89\x84a30V[`\0\x80a%\x19\x85a$\xDEa%\x02\x89\x89\x89\x89a4!V[`\0\x80a%za%\x02\x87\x87\x87\x87a3EV[\x90Pa%%\x87a$\xDE\x83g\r\xE0\xB6\xB3\xA7d\0\0aC\x0EV[a%\xD7\x82\x82`@Q`$\x01a%\xA8\x92\x91\x90aC5V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c-\x83\x9C\xB3`\xE2\x1B\x17\x90Ra4WV[PPV[`\0\x80`\0\x80a%\xEF\x8A\x8A\x8A\x8A\x8A\x8Aa4cV[\x90P`\0a& \x82`@Q` \x01a&\x07\x91\x90aCWV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83`@\x01Qa4\xC1V[\x90P\x80`\0\x03a&6W\x88\x94PPPPPa&\xA1V[`\0\x81\x12\x15a&VW\x88\x92Pa&O\x83`2`da5/V[\x93Pa&iV[\x88\x93Pa&f\x84`\x96`da5NV[\x92P[a&\x9A\x82`@Q` \x01a&}\x91\x90aCWV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x85\x85`\n`\x80a4\xC1a5|V[\x94PPPPP[\x96\x95PPPPPPV[`\0\x80a&\xBC\x89\x89\x89\x89\x89\x89a4cV[\x90Pa$\xE3\x81`@Q` \x01a&\xD2\x91\x90aCWV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x84a4\xC1V[`\0a'\r`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[a'\x15a\r\xF0V[`@\x84\x01\x81\x90R` \x84\x01\x82\x90R\x82\x84R`\x03T`\x04T`\x05T`\0\x95a'E\x95\x93\x94\x92\x93\x91\x92\x90\x91\x90\x8Ca&\xABV[\x90P\x80`\x18\x19\x13\x80a'WWP`\x19\x81\x13[\x15a'\x8EW`@Qc$J^\x17`\xE1\x1B\x81R\x87\x15\x15`\x04\x82\x01R`$\x81\x01\x86\x90R`D\x81\x01\x85\x90R`d\x81\x01\x82\x90R`\x84\x01a\x08\xDEV[`\0a'\xAB`\x03T\x88\x85`\0\x01Q\x86` \x01Q\x87`@\x01Qa$lV[\x90P\x87\x15a*\xB0W`\0g\r\xE0\xB6\xB3\xA7d\0\0`\x02Tg\r\xE0\xB6\xB3\xA7d\0\0a'\xD4\x91\x90aA\xEDV[a'\xDE\x90\x89aBDV[a'\xE8\x91\x90aBqV[\x90Pa(\x10`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01cfees`\xE0\x1B\x81RP\x82a%\x92V[`\0a(+\x82\x84\x87`\0\x01Q\x88` \x01Q\x89`@\x01Qa$\xF0V[\x90Pa(U`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01e\x19\x19[\x1D\x18S`\xD2\x1B\x81RP\x82a%\x92V[`\0a(\x97\x89`\x03Ta(h\x91\x90aA\xDAV[`\x04Ta(u\x85\x8EaA\xDAV[\x89`\0\x01Q\x8A` \x01Q\x8B`@\x01Q\x8E`\x04Ta(\x92\x91\x90aA\xEDV[a0\0V[\x90P\x80`1\x19\x13\x80a(\xA9WP`2\x81\x13[\x15a(\xE0W`@QcBCc\xB7`\xE1\x1B\x81R\x8B\x15\x15`\x04\x82\x01R`$\x81\x01\x8A\x90R`D\x81\x01\x89\x90R`d\x81\x01\x82\x90R`\x84\x01a\x08\xDEV[a(\xEA\x82\x8BaA\xDAV[`\x05\x81\x90UP\x88`\x03`\0\x82\x82Ta)\x02\x91\x90aA\xDAV[\x92PP\x81\x90UP\x87`\x04`\0\x82\x82Ta)\x1B\x91\x90aA\xEDV[\x90\x91UPP`\0T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a)T\x903\x900\x90\x8E\x90`\x04\x01aB\0V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a)\xACW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` aDk\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a)\xC0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a)\xE4\x91\x90aB$V[P`\x01T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x8A\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a*oW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` aDk\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a*\x83W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a*\xA7\x91\x90aB$V[PPPPa-[V[`\0g\r\xE0\xB6\xB3\xA7d\0\0`\x02Tg\r\xE0\xB6\xB3\xA7d\0\0a*\xD1\x91\x90aA\xEDV[a*\xDB\x90\x89aBDV[a*\xE5\x91\x90aBqV[\x90P`\0a+\x02\x82\x84\x87`\0\x01Q\x88` \x01Q\x89`@\x01Qa%RV[\x90P`\0a+F`\x03T\x8A`\x04Ta+\x1A\x91\x90aA\xDAV[a+$\x85\x8EaA\xDAV[\x89`\0\x01Q\x8A` \x01Q\x8B`@\x01Q\x8E`\x03Ta+A\x91\x90aA\xEDV[a/\xC4V[\x90P\x80`\x18\x19\x13\x80a+XWP`\x19\x81\x13[\x15a+\x8FW`@QcBCc\xB7`\xE1\x1B\x81R\x8B\x15\x15`\x04\x82\x01R`$\x81\x01\x8A\x90R`D\x81\x01\x89\x90R`d\x81\x01\x82\x90R`\x84\x01a\x08\xDEV[a+\x99\x82\x8BaA\xDAV[`\x05\x81\x90UP\x88`\x04`\0\x82\x82Ta+\xB1\x91\x90aA\xDAV[\x92PP\x81\x90UP\x87`\x03`\0\x82\x82Ta+\xCA\x91\x90aA\xEDV[\x90\x91UPP`\x01T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a,\x03\x903\x900\x90\x8E\x90`\x04\x01aB\0V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a,[W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` aDk\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a,oW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a,\x93\x91\x90aB$V[P`\0T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x8A\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a-\x1EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` aDk\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a-2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a-V\x91\x90aB$V[PPPP[3`\x01`\x01`\xA0\x1B\x03\x16\x7F\x17fH\xF1\xF1\x1C\xDA(L\x12D\x90\x08k\xE4*\x92m\xDF\n\xE8\x87\xEB\xE7\xB1\xD6\xB37\xD8\x94'V\x89\x88\x88a-\xA6`\x03T`\x05T\x8A`\0\x01Q\x8B` \x01Q\x8C`@\x01Qa$lV[`@\x80Q\x94\x15\x15\x85R` \x85\x01\x93\x90\x93R\x91\x83\x01R``\x82\x01R`\x80\x01`@Q\x80\x91\x03\x90\xA2P\x92\x96\x95PPPPPPV[`\0\x80`\0\x80a-\xEB\x8A\x8A\x8A\x8A\x8A\x8Aa4cV[\x90P`\0a.\x1C\x82`@Q` \x01a.\x03\x91\x90aCWV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83` \x01Qa6\x8DV[\x90P\x80`\0\x03a.5WP` \x01Q\x92Pa&\xA1\x91PPV[` \x82\x01Qa.G\x90`2`da5/V[\x93P\x81` \x01Q\x92Pa&\x9A\x82`@Q` \x01a.d\x91\x90aCWV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x85\x85`\n`\x80a6\x8Da5|V[`\0\x80`\0\x80a.\x95\x8A\x8A\x8A\x8A\x8A\x8Aa4cV[\x90P`\0a.\xC4\x82`@Q` \x01a.\xAD\x91\x90aCWV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x83Qa6\xE6V[\x90P\x80`\0\x03a.\xDAWPQ\x92Pa&\xA1\x91PPV[\x81Qa.\xE9\x90`2`da5/V[\x82Q\x90\x94P\x92Pa&\x9A\x82`@Q` \x01a/\x04\x91\x90aCWV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x85\x85`\n`\x80a6\xE6a5|V[a/)a\"lV[`\x11UB`\x13UV[`\0\x80a/G\x83g\r\xE0\xB6\xB3\xA7d\0\0a7?V[\x90P`\0a/^\x87a/Y\x87\x89a7kV[a0\xCCV[\x90Pa%%\x85a$\xDEa/r`\x02\x86aB\xE0V[g\r\xE0\xB6\xB3\xA7d\0\0a/\x84\x86a0\xEAV[a/\x8E\x90\x8AaB\xB0V[a/\x98\x91\x90aB\xE0V[a$\xD9\x91\x90aC\x9BV[a/\xAAa\x1E\xC1V[`\x0CUB`\x0EUV[a/\xBBa\x1D\xB7V[`\x07UB`\tUV[`\0\x80a/\xD5\x89\x89\x89\x89\x89\x89a4cV[\x90Pa$\xE3\x81`@Q` \x01a/\xEB\x91\x90aCWV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x84a6\xE6V[`\0\x80a0\x11\x89\x89\x89\x89\x89\x89a4cV[\x90Pa$\xE3\x81`@Q` \x01a0'\x91\x90aCWV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x84a6\x8DV[`\0a0Ua0K\x85\x84a7kV[a$\xB6\x90\x85aBqV[a0ba$\xB6\x86\x88aBqV[a\x0Cx\x91\x90aC\x9BV[`\0\x80a0x\x83a7\x80V[a0\x86\x90c;\x9A\xCA\0aBDV[\x90Pa0\x92\x84\x82a7kV[\x94\x93PPPPV[`\0\x80a0\xB8a0\xB2\x85g\x1B\xC1mgN\xC8\0\0a7?V[\x84a7kV[\x90Pa0\x92g\x06\xF0[Y\xD3\xB2\0\0\x82a7kV[`\0a0\xE1\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a5/V[\x90P[\x92\x91PPV[`\0g\x06\xF0[Y\xD3\xB2\0\0\x82\x03a1\x03WP`\0\x91\x90PV[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x12a1+W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a1LW`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a1Y\x83`\x02aB\xB0V[\x90P`\0a1f\x82a8$V[\x90P`\0a1|g\x13\xA0K\xBD\xFD\xC9\xBE\x88\x83a:\xA2V[\x90Pa\x0Cx\x81aC\xC3V[`\0h\x02H\xCE6\xA7\x0C\xB2k>\x19\x82\x13a1\xA2WP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a1\xE9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x01a\x08\xDEV[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05`\x01`_\x1B\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1Dm\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1Dn\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[`\0a0\xE1\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a5NV[`\0\x80a3R\x84\x84a0lV[\x90P`\0a3`\x87\x87a:\xB7V[\x90P`\0a3n\x86\x86a0\x9AV[\x90P\x82a3{\x82\x84aC\x9BV[a3\x8D\x90g\r\xE0\xB6\xB3\xA7d\0\0aB\xB0V[a3\x97\x91\x90aB\xE0V[\x98\x97PPPPPPPPV[`\0\x80g\x13\xA0K\xBD\xFD\xC9\xBE\x88a3\xC1g\r\xE0\xB6\xB3\xA7d\0\0\x85aB\xB0V[a3\xCB\x91\x90aB\xE0V[\x90P`\0a3\xD8\x82aC\xC3V[\x90P`\0a3\xE5\x82a:\xCBV[\x90Pg\x1B\xC1mgN\xC8\0\0a4\x02g\r\xE0\xB6\xB3\xA7d\0\0\x83aB\xB0V[a\x0Cx\x91\x90aB\xE0V[`\0a0\xE1\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a5NV[`\0\x80a4.\x84\x84a0lV[\x90P`\0a4<\x87\x87a:\xB7V[\x90P`\0a4J\x86\x86a0\x9AV[\x90P\x82a3{\x82\x84aC\x0EV[a4`\x81a<\xAFV[PV[a4\x9C`@Q\x80`\xC0\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[\x95\x86R` \x86\x01\x94\x90\x94R`@\x85\x01\x92\x90\x92R``\x84\x01R`\x80\x83\x01R`\xA0\x82\x01R\x90V[`\0\x80\x83\x80` \x01\x90Q\x81\x01\x90a4\xD8\x91\x90aC\xDFV[\x90Pa4\xEC\x81`\x80\x01Q\x82`\xA0\x01Qa0lV[a5\na5\x05\x83` \x01Qa/Y\x87\x86``\x01Qa7kV[a<\xD0V[a5\x1Ba5\x05\x84`\0\x01Q\x87a0\xCCV[a5%\x91\x90aC\x9BV[a0\x92\x91\x90aC\x9BV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a5GW`\0\x80\xFD[\x04\x92\x91PPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a5fW`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[`\0\x84\x86\x11\x15a5\xA9W`@Qc0\x82\xDF\xDB`\xE1\x1B\x81R`\x04\x81\x01\x87\x90R`$\x81\x01\x86\x90R`D\x01a\x08\xDEV[`\0a5\xB9\x88\x88\x85c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a5\xCB\x89\x88\x86c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a5\xD9\x82\x84aB\xB0V[\x13\x15a6\x02W`@Qc\x06\xF1\xBE]`\xE2\x1B\x81R`\x04\x81\x01\x83\x90R`$\x81\x01\x82\x90R`D\x01a\x08\xDEV[`\0a6\x0E\x89\x89aA\xEDV[\x90P`\0[`\x02a6\x1F\x8A\x8CaA\xDAV[a6)\x91\x90aBqV[\x94P`\0a6;\x8C\x87\x89c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a6I\x86\x83aB\xB0V[\x13a6VW\x85\x99Pa6]V[\x85\x9AP\x80\x94P[a6g\x8B\x8BaA\xEDV[\x92PP`\x01\x01\x87\x82\x11\x80\x15a6{WP\x86\x81\x10[a6\x13WPPPP\x96\x95PPPPPPV[`\0\x80\x83\x80` \x01\x90Q\x81\x01\x90a6\xA4\x91\x90aC\xDFV[\x90Pa6\xB8\x81`\x80\x01Q\x82`\xA0\x01Qa0lV[a6\xD1a5\x05\x85a/Y\x85`@\x01Q\x86``\x01Qa7kV[a5\x1Ba5\x05\x84`\0\x01Q\x85`@\x01Qa0\xCCV[`\0\x80\x83\x80` \x01\x90Q\x81\x01\x90a6\xFD\x91\x90aC\xDFV[\x90Pa7\x11\x81`\x80\x01Q\x82`\xA0\x01Qa0lV[a7.a5\x05\x83` \x01Qa/Y\x85`@\x01Q\x86``\x01Qa7kV[a5\x1Ba5\x05\x86\x85`@\x01Qa0\xCCV[`\0a0\xE1g\r\xE0\xB6\xB3\xA7d\0\0\x83a7W\x86a<\xDEV[a7a\x91\x90aB\xB0V[a$\xD9\x91\x90aB\xE0V[`\0a0\xE1\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a5/V[`\xB5\x81`\x01`\x88\x1B\x81\x10a7\x99W`@\x91\x90\x91\x1B\x90`\x80\x1C[i\x01\0\0\0\0\0\0\0\0\0\x81\x10a7\xB5W` \x91\x90\x91\x1B\x90`@\x1C[e\x01\0\0\0\0\0\x81\x10a7\xCDW`\x10\x91\x90\x91\x1B\x90` \x1C[c\x01\0\0\0\x81\x10a7\xE3W`\x08\x91\x90\x91\x1B\x90`\x10\x1C[b\x01\0\0\x01\x02`\x12\x1C\x80\x82\x04\x01`\x01\x90\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x90\x81\x90\x04\x81\x11\x90\x03\x90V[`\0\x80\x82\x12\x80a8;WPg\x1B\xC1mgN\xC8\0\0\x82\x13[\x15a8YW`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a8zW`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81g\x1B\xC1mgN\xC8\0\0\x03a8\xA2W`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x15a8\xADW\x91\x90PV[`\0g\r\xE0\xB6\xB3\xA7d\0\0\x83\x12a8\xD5Wa8\xD0\x83g\x1B\xC1mgN\xC8\0\0aC\x0EV[a8\xD7V[\x82[\x90P`\0a8\xED\x82g\x1B\xC1mgN\xC8\0\0a>\xB9V[\x90P\x80`\0\x03a9\x10W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a9\x1B\x82a<\xDEV[\x90P`\0c;\x9A\xCA\0a9Fa9Aa9;g\x1B\xC1mgN\xC8\0\0aC\xC3V[\x85a:\xA2V[a7\x80V[a9P\x91\x90aB\xB0V[\x90P`\0\x80a9g\x83g\x03\xC1f\\z\xAB \0a:\xA2V[a9y\x90g \x05\xFEO&\x8E\xA0\0aC\x9BV[\x90P`\0a9\xA9\x84a9\x92\x86f\x9F2u$b\xA0\0a:\xA2V[a9\xA4\x90g\r\xC5R\x7Fd, \0aC\x9BV[a:\xA2V[a9\xBB\x90g\r\xE0\xB6\xB3\xA7d\0\0aC\x9BV[\x90Pa9\xDFg\t\xD0(\xCCo _\xFF\x19\x85a9\xD5\x85\x85a>\xB9V[a9\xA4\x91\x90aC\x0EV[\x92PPP`\0[`\x02\x81\x10\x15a:zW`\0\x86a9\xFB\x84a:\xCBV[a:\x05\x91\x90aC\x0EV[\x90P`\0a:\x13\x84\x85a:\xA2V[a:\x1C\x90aC\xC3V[\x90P`\0a:)\x82a1\x87V[\x90P`\0a:7\x86\x85a:\xA2V[a:Ig\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x84a:\xA2V[a:S\x91\x90aC\x0EV[\x90Pa:_\x84\x82a>\xB9V[a:i\x90\x87aC\x9BV[\x95P\x84`\x01\x01\x94PPPPPa9\xE6V[g\r\xE0\xB6\xB3\xA7d\0\0\x88\x12a:\x97Wa:\x92\x82aC\xC3V[a3\x97V[P\x96\x95PPPPPPV[`\0a0\xE1\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a>\xCAV[`\0a0\xE1a:\xC6\x84\x84a4\x0CV[a<\xDEV[`\0\x81`\0\x03a:\xE4WPg\r\xE0\xB6\xB3\xA7d\0\0\x91\x90PV[gV\x98\xEE\xF0fp\0\0\x82\x12a:\xFBWP`\0\x91\x90PV[a;\x0CgV\x98\xEE\xF0fp\0\0aC\xC3V[\x82\x13a;!WPg\x1B\xC1mgN\xC8\0\0\x91\x90PV[`\0a;,\x83a>\xE9V[\x90P`\0a;eg\r\xE0\xB6\xB3\xA7d\0\0a;N\x84g\x1B\xC1mgN\xC8\0\0a0\xCCV[a;`\x90g\r\xE0\xB6\xB3\xA7d\0\0aC\x9BV[a>\xB9V[\x90P`\0\x80\x82a;\xC1\x81a;\xAE\x81a;\x9C\x81a;\x89\x81g\x02_\x0F\xE1\x05\xA3\x14\0a:\xA2V[a9\xA4\x90g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19aC\x9BV[a9\xA4\x90g\x14\xA8EL\x19\xE1\xAC\0aC\x9BV[a9\xA4\x90g\x0F\xC1\x0E\x01W\x82w\xFF\x19aC\x9BV[a;\xD3\x90g\x03\xDE\xBD\x08;\x8C|\0aC\x9BV[\x91P\x83\x90Pa<;\x81a<)\x81a<\x17\x81a<\x05\x81a;\xF2\x81\x8Ba:\xA2V[a9\xA4\x90g\x02\x95\xD4\0\xEA2W\xFF\x19aC\x9BV[a9\xA4\x90g\x01W\xD8\xB2\xEC\xC7\x08\0aC\x9BV[a9\xA4\x90g\x051\n\xA7\xD5!0\0aC\x9BV[a9\xA4\x90g\r\xE0\xCC=\x15a\0\0aC\x9BV[\x91P\x81g\x11\x90\0\xAB\x10\x0F\xFC\0a<Q\x87\x88a:\xA2V[a<]\x90`\0\x19aB\xB0V[a<g\x91\x90aC\x0EV[a<q\x91\x90aC\x9BV[\x92PP`\0a<\x7F\x83a1\x87V[\x90P`\0a<\x8D\x85\x83a:\xA2V[\x90P`\0\x88\x12a<\x9DW\x80a3\x97V[a3\x97\x81g\x1B\xC1mgN\xC8\0\0aC\x0EV[\x80Qjconsole.log` \x83\x01`\0\x80\x84\x83\x85Z\xFAPPPPPV[`\0\x80a1Y\x83`\x02aB\xB0V[`\0\x80\x82\x13a=\x1BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x08\xDEV[`\0``a=(\x84a?$V[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[`\0a0\xE1\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x05\x85\x14\x17\x16a>\xE2W`\0\x80\xFD[\x05\x92\x91PPV[`\0`\x01`\xFF\x1B\x82\x03a?\x0FW`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x12\x15a? WP\x19`\x01\x01\x90V[P\x90V[`\0\x80\x82\x11a?aW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x08\xDEV[P`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x82\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`\0` \x82\x84\x03\x12\x15a@1Wa@1a?\xCCV[P5\x91\x90PV[\x80\x15\x15\x81\x14a4`W`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15a@^Wa@^a?\xCCV[\x835a@i\x81a@8V[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a@\x94Wa@\x94a?\xCCV[PP\x805\x92` \x90\x91\x015\x91PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a@\xBCWa@\xBCa?\xCCV[\x845a@\xC7\x81a@8V[\x96` \x86\x015\x96P`@\x86\x015\x95``\x015\x94P\x92PPPV[`\0` \x82\x84\x03\x12\x15a@\xF6Wa@\xF6a?\xCCV[\x815`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\r\xE9W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15aA#WaA#a?\xCCV[\x825aA.\x81a@8V[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15aAbW` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01aAFV[P`\0` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R`\0a0\xE1` \x83\x01\x84aA<V[`\0\x80`\0``\x84\x86\x03\x12\x15aA\xADWaA\xADa?\xCCV[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a0\xE4Wa0\xE4aA\xC4V[\x81\x81\x03\x81\x81\x11\x15a0\xE4Wa0\xE4aA\xC4V[`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x81R\x91\x90\x92\x16` \x82\x01R`@\x81\x01\x91\x90\x91R``\x01\x90V[`\0` \x82\x84\x03\x12\x15aB9WaB9a?\xCCV[\x81Qa\r\xE9\x81a@8V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a0\xE4Wa0\xE4aA\xC4V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82aB\x80WaB\x80aB[V[P\x04\x90V[` \x80\x82R`\x11\x90\x82\x01Rp\x15\\\x19\x18]\x19H\x19[\x99\x08\x1C\x18\\\xDC\xD9Y`z\x1B`@\x82\x01R``\x01\x90V[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15aB\xCCWaB\xCCaA\xC4V[\x81\x81\x05\x83\x14\x82\x15\x17a0\xE4Wa0\xE4aA\xC4V[`\0\x82aB\xEFWaB\xEFaB[V[`\x01`\xFF\x1B\x82\x14`\0\x19\x84\x14\x16\x15aC\tWaC\taA\xC4V[P\x05\x90V[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15aC.WaC.aA\xC4V[P\x92\x91PPV[`@\x81R`\0aCH`@\x83\x01\x85aA<V[\x90P\x82` \x83\x01R\x93\x92PPPV[`\0`\xC0\x82\x01\x90P\x82Q\x82R` \x83\x01Q` \x83\x01R`@\x83\x01Q`@\x83\x01R``\x83\x01Q``\x83\x01R`\x80\x83\x01Q`\x80\x83\x01R`\xA0\x83\x01Q`\xA0\x83\x01R\x92\x91PPV[\x80\x82\x01\x82\x81\x12`\0\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15aC\xBBWaC\xBBaA\xC4V[PP\x92\x91PPV[`\0`\x01`\xFF\x1B\x82\x01aC\xD8WaC\xD8aA\xC4V[P`\0\x03\x90V[`\0`\xC0\x82\x84\x03\x12\x15aC\xF4WaC\xF4a?\xCCV[`@Q`\xC0\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15aD%WcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x80`@RP\x82Q\x81R` \x83\x01Q` \x82\x01R`@\x83\x01Q`@\x82\x01R``\x83\x01Q``\x82\x01R`\x80\x83\x01Q`\x80\x82\x01R`\xA0\x83\x01Q`\xA0\x82\x01R\x80\x91PP\x92\x91PPV\xFETarget contract does not contains\xEBa\xB7\x83\x9B\xB3\xC27\xA0\xB3\xAE\xAB\x18iQ\0\xBCQ.q9\xF0\xE3@e]\xD7\x02\xDA*\xA1";
    /// The deployed bytecode of the contract.
    pub static RMM_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct RMM<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for RMM<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for RMM<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for RMM<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for RMM<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(RMM)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> RMM<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    RMM_ABI.clone(),
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
                RMM_ABI.clone(),
                RMM_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `addLiquidity` (0x76701636) function
        pub fn add_liquidity(
            &self,
            exact_x: bool,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([118, 112, 22, 54], (exact_x, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `addLiquidityExactX` (0x02c2e55d) function
        pub fn add_liquidity_exact_x(
            &self,
            amount_x: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([2, 194, 229, 93], amount_x)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `addLiquidityExactY` (0x9c9da9ea) function
        pub fn add_liquidity_exact_y(
            &self,
            amount_y: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([156, 157, 169, 234], amount_y)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `balanceOf` (0x70a08231) function
        pub fn balance_of(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `computeInitialPoolState` (0x37aa7381) function
        pub fn compute_initial_pool_state(
            &self,
            amount_x: ::ethers::core::types::U256,
            initial_price: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([55, 170, 115, 129], (amount_x, initial_price))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getAmountOut` (0x5d840ae5) function
        pub fn get_amount_out(
            &self,
            swap_direction: bool,
            next_liquidity: ::ethers::core::types::U256,
            amount_in: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [93, 132, 10, 229],
                    (swap_direction, next_liquidity, amount_in),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getDeltaL` (0xcf78e48f) function
        pub fn get_delta_l(
            &self,
            swap_direction: bool,
            next_liquidity: ::ethers::core::types::U256,
            amount_in: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [207, 120, 228, 143],
                    (swap_direction, next_liquidity, amount_in),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getInvariant` (0xc0ff1a15) function
        pub fn get_invariant(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([192, 255, 26, 21], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getLiquidity` (0x0910a510) function
        pub fn get_liquidity(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([9, 16, 165, 16], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getNextLiquidity` (0xf3a8efe3) function
        pub fn get_next_liquidity(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([243, 168, 239, 227], ())
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
        ///Calls the contract's `getPortfolioValue` (0xbb0498de) function
        pub fn get_portfolio_value(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([187, 4, 152, 222], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getReserveX` (0x559d1602) function
        pub fn get_reserve_x(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([85, 157, 22, 2], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getReserveY` (0xfed3dfda) function
        pub fn get_reserve_y(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([254, 211, 223, 218], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getSpotPrice` (0xdc76fabc) function
        pub fn get_spot_price(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([220, 118, 250, 188], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getSpotPriceFromY` (0x89e2c259) function
        pub fn get_spot_price_from_y(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([137, 226, 194, 89], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getStrategyData` (0x876b55f1) function
        pub fn get_strategy_data(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([135, 107, 85, 241], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getSwapConstantGivenLiquidity` (0x4166d348) function
        pub fn get_swap_constant_given_liquidity(
            &self,
            new_liquidity: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([65, 102, 211, 72], new_liquidity)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getSwapFee` (0xd4cadf68) function
        pub fn get_swap_fee(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([212, 202, 223, 104], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initExactTokensAndLiquidity` (0x9e1932d8) function
        pub fn init_exact_tokens_and_liquidity(
            &self,
            amount_x: ::ethers::core::types::U256,
            amount_y: ::ethers::core::types::U256,
            liquidity: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([158, 25, 50, 216], (amount_x, amount_y, liquidity))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initExactX` (0xf9a1c85a) function
        pub fn init_exact_x(
            &self,
            amount_x: ::ethers::core::types::U256,
            price: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([249, 161, 200, 90], (amount_x, price))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initExactY` (0xa59c186f) function
        pub fn init_exact_y(
            &self,
            amount_y: ::ethers::core::types::U256,
            price: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([165, 156, 24, 111], (amount_y, price))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initPool` (0x04afa822) function
        pub fn init_pool(
            &self,
            exact_x: bool,
            amount: ::ethers::core::types::U256,
            price: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([4, 175, 168, 34], (exact_x, amount, price))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `instantiate` (0x3639aa32) function
        pub fn instantiate(
            &self,
            amount: ::ethers::core::types::U256,
            price: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([54, 57, 170, 50], (amount, price))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `logData` (0xbcc17dc7) function
        pub fn log_data(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([188, 193, 125, 199], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeLiquidityExactX` (0xccd1e4be) function
        pub fn remove_liquidity_exact_x(
            &self,
            amount_x: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([204, 209, 228, 190], amount_x)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeLiquidityExactY` (0x7d69537e) function
        pub fn remove_liquidity_exact_y(
            &self,
            amount_y: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([125, 105, 83, 126], amount_y)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `reserveX` (0x08eabdda) function
        pub fn reserve_x(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([8, 234, 189, 218], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `reserveY` (0xfadfa65b) function
        pub fn reserve_y(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([250, 223, 166, 91], ())
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
        ///Calls the contract's `setSwapFee` (0x34e19907) function
        pub fn set_swap_fee(
            &self,
            new_swap_fee: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([52, 225, 153, 7], new_swap_fee)
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
        ///Calls the contract's `strikePrice` (0xc52987cf) function
        pub fn strike_price(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([197, 41, 135, 207], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `swap` (0x59542ca9) function
        pub fn swap(
            &self,
            swap_direction: bool,
            next_liquidity: ::ethers::core::types::U256,
            amount_in: ::ethers::core::types::U256,
            amount_out: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [89, 84, 44, 169],
                    (swap_direction, next_liquidity, amount_in, amount_out),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `swapAmountIn` (0xf2d1922f) function
        pub fn swap_amount_in(
            &self,
            swap_direction: bool,
            next_liquidity: ::ethers::core::types::U256,
            amount_in: ::ethers::core::types::U256,
            amount_out: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [242, 209, 146, 47],
                    (swap_direction, next_liquidity, amount_in, amount_out),
                )
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
        ///Calls the contract's `tokenX` (0x16dc165b) function
        pub fn token_x(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([22, 220, 22, 91], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `tokenY` (0xb7d19fc4) function
        pub fn token_y(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([183, 209, 159, 196], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `totalLiquidity` (0x15770f92) function
        pub fn total_liquidity(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([21, 119, 15, 146], ())
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `AddLiquidity` event
        pub fn add_liquidity_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AddLiquidityFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `LogParameters` event
        pub fn log_parameters_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogParametersFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `LogReserves` event
        pub fn log_reserves_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogReservesFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RemoveLiquidity` event
        pub fn remove_liquidity_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RemoveLiquidityFilter,
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
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, RMMEvents> {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for RMM<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `AmountOutConstantOutOfRange` with signature `AmountOutConstantOutOfRange(bool,uint256,uint256,int256)` and selector `0x8486c76e`
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
        name = "AmountOutConstantOutOfRange",
        abi = "AmountOutConstantOutOfRange(bool,uint256,uint256,int256)"
    )]
    pub struct AmountOutConstantOutOfRange {
        pub swap_direction: bool,
        pub amount_in: ::ethers::core::types::U256,
        pub amount_out: ::ethers::core::types::U256,
        pub swap_constant: ::ethers::core::types::I256,
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
    ///Custom Error type `LiquidityOutOfRange` with signature `LiquidityOutOfRange(uint256,int256)` and selector `0x46a79e08`
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
        name = "LiquidityOutOfRange",
        abi = "LiquidityOutOfRange(uint256,int256)"
    )]
    pub struct LiquidityOutOfRange {
        pub liquidity: ::ethers::core::types::U256,
        pub swap_constant: ::ethers::core::types::I256,
    }
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
    ///Custom Error type `NextLiquidityConstantOutOfRange` with signature `NextLiquidityConstantOutOfRange(bool,uint256,uint256,int256)` and selector `0x4894bc2e`
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
        name = "NextLiquidityConstantOutOfRange",
        abi = "NextLiquidityConstantOutOfRange(bool,uint256,uint256,int256)"
    )]
    pub struct NextLiquidityConstantOutOfRange {
        pub swap_direction: bool,
        pub amount_in: ::ethers::core::types::U256,
        pub amount_out: ::ethers::core::types::U256,
        pub swap_constant: ::ethers::core::types::I256,
    }
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
    ///Custom Error type `XReserveOutOfRange` with signature `XReserveOutOfRange(uint256,int256)` and selector `0xc74b5213`
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
    #[etherror(name = "XReserveOutOfRange", abi = "XReserveOutOfRange(uint256,int256)")]
    pub struct XReserveOutOfRange {
        pub reserve_x: ::ethers::core::types::U256,
        pub swap_constant: ::ethers::core::types::I256,
    }
    ///Custom Error type `YReserveOutOfRange` with signature `YReserveOutOfRange(uint256,int256)` and selector `0xe9365dfc`
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
    #[etherror(name = "YReserveOutOfRange", abi = "YReserveOutOfRange(uint256,int256)")]
    pub struct YReserveOutOfRange {
        pub reserve_y: ::ethers::core::types::U256,
        pub swap_constant: ::ethers::core::types::I256,
    }
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
    pub enum RMMErrors {
        AmountOutConstantOutOfRange(AmountOutConstantOutOfRange),
        BisectionLib_InvalidBounds(BisectionLib_InvalidBounds),
        BisectionLib_RootOutsideBounds(BisectionLib_RootOutsideBounds),
        Infinity(Infinity),
        LiquidityOutOfRange(LiquidityOutOfRange),
        Min(Min),
        NegativeInfinity(NegativeInfinity),
        NextLiquidityConstantOutOfRange(NextLiquidityConstantOutOfRange),
        OutOfBounds(OutOfBounds),
        XReserveOutOfRange(XReserveOutOfRange),
        YReserveOutOfRange(YReserveOutOfRange),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for RMMErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <AmountOutConstantOutOfRange as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AmountOutConstantOutOfRange(decoded));
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
            if let Ok(decoded) = <LiquidityOutOfRange as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LiquidityOutOfRange(decoded));
            }
            if let Ok(decoded) = <Min as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Min(decoded));
            }
            if let Ok(decoded) = <NegativeInfinity as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NegativeInfinity(decoded));
            }
            if let Ok(decoded) = <NextLiquidityConstantOutOfRange as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NextLiquidityConstantOutOfRange(decoded));
            }
            if let Ok(decoded) = <OutOfBounds as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OutOfBounds(decoded));
            }
            if let Ok(decoded) = <XReserveOutOfRange as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::XReserveOutOfRange(decoded));
            }
            if let Ok(decoded) = <YReserveOutOfRange as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::YReserveOutOfRange(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for RMMErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::AmountOutConstantOutOfRange(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BisectionLib_InvalidBounds(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BisectionLib_RootOutsideBounds(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Infinity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LiquidityOutOfRange(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Min(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NegativeInfinity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NextLiquidityConstantOutOfRange(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OutOfBounds(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::XReserveOutOfRange(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::YReserveOutOfRange(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for RMMErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <AmountOutConstantOutOfRange as ::ethers::contract::EthError>::selector() => {
                    true
                }
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
                _ if selector
                    == <LiquidityOutOfRange as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector == <Min as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NegativeInfinity as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NextLiquidityConstantOutOfRange as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OutOfBounds as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <XReserveOutOfRange as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <YReserveOutOfRange as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for RMMErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AmountOutConstantOutOfRange(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BisectionLib_InvalidBounds(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BisectionLib_RootOutsideBounds(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Infinity(element) => ::core::fmt::Display::fmt(element, f),
                Self::LiquidityOutOfRange(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Min(element) => ::core::fmt::Display::fmt(element, f),
                Self::NegativeInfinity(element) => ::core::fmt::Display::fmt(element, f),
                Self::NextLiquidityConstantOutOfRange(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OutOfBounds(element) => ::core::fmt::Display::fmt(element, f),
                Self::XReserveOutOfRange(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::YReserveOutOfRange(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for RMMErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<AmountOutConstantOutOfRange> for RMMErrors {
        fn from(value: AmountOutConstantOutOfRange) -> Self {
            Self::AmountOutConstantOutOfRange(value)
        }
    }
    impl ::core::convert::From<BisectionLib_InvalidBounds> for RMMErrors {
        fn from(value: BisectionLib_InvalidBounds) -> Self {
            Self::BisectionLib_InvalidBounds(value)
        }
    }
    impl ::core::convert::From<BisectionLib_RootOutsideBounds> for RMMErrors {
        fn from(value: BisectionLib_RootOutsideBounds) -> Self {
            Self::BisectionLib_RootOutsideBounds(value)
        }
    }
    impl ::core::convert::From<Infinity> for RMMErrors {
        fn from(value: Infinity) -> Self {
            Self::Infinity(value)
        }
    }
    impl ::core::convert::From<LiquidityOutOfRange> for RMMErrors {
        fn from(value: LiquidityOutOfRange) -> Self {
            Self::LiquidityOutOfRange(value)
        }
    }
    impl ::core::convert::From<Min> for RMMErrors {
        fn from(value: Min) -> Self {
            Self::Min(value)
        }
    }
    impl ::core::convert::From<NegativeInfinity> for RMMErrors {
        fn from(value: NegativeInfinity) -> Self {
            Self::NegativeInfinity(value)
        }
    }
    impl ::core::convert::From<NextLiquidityConstantOutOfRange> for RMMErrors {
        fn from(value: NextLiquidityConstantOutOfRange) -> Self {
            Self::NextLiquidityConstantOutOfRange(value)
        }
    }
    impl ::core::convert::From<OutOfBounds> for RMMErrors {
        fn from(value: OutOfBounds) -> Self {
            Self::OutOfBounds(value)
        }
    }
    impl ::core::convert::From<XReserveOutOfRange> for RMMErrors {
        fn from(value: XReserveOutOfRange) -> Self {
            Self::XReserveOutOfRange(value)
        }
    }
    impl ::core::convert::From<YReserveOutOfRange> for RMMErrors {
        fn from(value: YReserveOutOfRange) -> Self {
            Self::YReserveOutOfRange(value)
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
        name = "AddLiquidity",
        abi = "AddLiquidity(address,uint256,uint256,uint256)"
    )]
    pub struct AddLiquidityFilter {
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
        pub liquidity: ::ethers::core::types::U256,
        pub amount_x: ::ethers::core::types::U256,
        pub amount_y: ::ethers::core::types::U256,
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
        name = "LogParameters",
        abi = "LogParameters(uint256,uint256,uint256,uint256)"
    )]
    pub struct LogParametersFilter {
        pub sigma: ::ethers::core::types::U256,
        pub strike_price: ::ethers::core::types::U256,
        pub tau: ::ethers::core::types::U256,
        pub block_timestamp: ::ethers::core::types::U256,
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
    #[ethevent(name = "LogReserves", abi = "LogReserves(uint256,uint256,uint256)")]
    pub struct LogReservesFilter {
        pub reserve_x: ::ethers::core::types::U256,
        pub reserve_y: ::ethers::core::types::U256,
        pub block_timestamp: ::ethers::core::types::U256,
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
        name = "RemoveLiquidity",
        abi = "RemoveLiquidity(address,uint256,uint256,uint256)"
    )]
    pub struct RemoveLiquidityFilter {
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
        pub liquidity: ::ethers::core::types::U256,
        pub amount_x: ::ethers::core::types::U256,
        pub amount_y: ::ethers::core::types::U256,
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
    #[ethevent(name = "Swap", abi = "Swap(address,bool,uint256,uint256,uint256)")]
    pub struct SwapFilter {
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
        pub swap_direction: bool,
        pub input: ::ethers::core::types::U256,
        pub output: ::ethers::core::types::U256,
        pub new_price: ::ethers::core::types::U256,
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
    pub enum RMMEvents {
        AddLiquidityFilter(AddLiquidityFilter),
        LogParametersFilter(LogParametersFilter),
        LogReservesFilter(LogReservesFilter),
        RemoveLiquidityFilter(RemoveLiquidityFilter),
        SwapFilter(SwapFilter),
    }
    impl ::ethers::contract::EthLogDecode for RMMEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AddLiquidityFilter::decode_log(log) {
                return Ok(RMMEvents::AddLiquidityFilter(decoded));
            }
            if let Ok(decoded) = LogParametersFilter::decode_log(log) {
                return Ok(RMMEvents::LogParametersFilter(decoded));
            }
            if let Ok(decoded) = LogReservesFilter::decode_log(log) {
                return Ok(RMMEvents::LogReservesFilter(decoded));
            }
            if let Ok(decoded) = RemoveLiquidityFilter::decode_log(log) {
                return Ok(RMMEvents::RemoveLiquidityFilter(decoded));
            }
            if let Ok(decoded) = SwapFilter::decode_log(log) {
                return Ok(RMMEvents::SwapFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for RMMEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddLiquidityFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LogParametersFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LogReservesFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveLiquidityFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SwapFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AddLiquidityFilter> for RMMEvents {
        fn from(value: AddLiquidityFilter) -> Self {
            Self::AddLiquidityFilter(value)
        }
    }
    impl ::core::convert::From<LogParametersFilter> for RMMEvents {
        fn from(value: LogParametersFilter) -> Self {
            Self::LogParametersFilter(value)
        }
    }
    impl ::core::convert::From<LogReservesFilter> for RMMEvents {
        fn from(value: LogReservesFilter) -> Self {
            Self::LogReservesFilter(value)
        }
    }
    impl ::core::convert::From<RemoveLiquidityFilter> for RMMEvents {
        fn from(value: RemoveLiquidityFilter) -> Self {
            Self::RemoveLiquidityFilter(value)
        }
    }
    impl ::core::convert::From<SwapFilter> for RMMEvents {
        fn from(value: SwapFilter) -> Self {
            Self::SwapFilter(value)
        }
    }
    ///Container type for all input parameters for the `addLiquidity` function with signature `addLiquidity(bool,uint256)` and selector `0x76701636`
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
    #[ethcall(name = "addLiquidity", abi = "addLiquidity(bool,uint256)")]
    pub struct AddLiquidityCall {
        pub exact_x: bool,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `addLiquidityExactX` function with signature `addLiquidityExactX(uint256)` and selector `0x02c2e55d`
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
    #[ethcall(name = "addLiquidityExactX", abi = "addLiquidityExactX(uint256)")]
    pub struct AddLiquidityExactXCall {
        pub amount_x: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `addLiquidityExactY` function with signature `addLiquidityExactY(uint256)` and selector `0x9c9da9ea`
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
    #[ethcall(name = "addLiquidityExactY", abi = "addLiquidityExactY(uint256)")]
    pub struct AddLiquidityExactYCall {
        pub amount_y: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `balanceOf` function with signature `balanceOf(address)` and selector `0x70a08231`
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
    #[ethcall(name = "balanceOf", abi = "balanceOf(address)")]
    pub struct BalanceOfCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `computeInitialPoolState` function with signature `computeInitialPoolState(uint256,uint256)` and selector `0x37aa7381`
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
        name = "computeInitialPoolState",
        abi = "computeInitialPoolState(uint256,uint256)"
    )]
    pub struct ComputeInitialPoolStateCall {
        pub amount_x: ::ethers::core::types::U256,
        pub initial_price: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getAmountOut` function with signature `getAmountOut(bool,uint256,uint256)` and selector `0x5d840ae5`
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
    #[ethcall(name = "getAmountOut", abi = "getAmountOut(bool,uint256,uint256)")]
    pub struct GetAmountOutCall {
        pub swap_direction: bool,
        pub next_liquidity: ::ethers::core::types::U256,
        pub amount_in: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getDeltaL` function with signature `getDeltaL(bool,uint256,uint256)` and selector `0xcf78e48f`
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
    #[ethcall(name = "getDeltaL", abi = "getDeltaL(bool,uint256,uint256)")]
    pub struct GetDeltaLCall {
        pub swap_direction: bool,
        pub next_liquidity: ::ethers::core::types::U256,
        pub amount_in: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getInvariant` function with signature `getInvariant()` and selector `0xc0ff1a15`
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
    #[ethcall(name = "getInvariant", abi = "getInvariant()")]
    pub struct GetInvariantCall;
    ///Container type for all input parameters for the `getLiquidity` function with signature `getLiquidity()` and selector `0x0910a510`
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
    #[ethcall(name = "getLiquidity", abi = "getLiquidity()")]
    pub struct GetLiquidityCall;
    ///Container type for all input parameters for the `getNextLiquidity` function with signature `getNextLiquidity()` and selector `0xf3a8efe3`
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
    #[ethcall(name = "getNextLiquidity", abi = "getNextLiquidity()")]
    pub struct GetNextLiquidityCall;
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
    ///Container type for all input parameters for the `getPortfolioValue` function with signature `getPortfolioValue()` and selector `0xbb0498de`
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
    #[ethcall(name = "getPortfolioValue", abi = "getPortfolioValue()")]
    pub struct GetPortfolioValueCall;
    ///Container type for all input parameters for the `getReserveX` function with signature `getReserveX()` and selector `0x559d1602`
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
    #[ethcall(name = "getReserveX", abi = "getReserveX()")]
    pub struct GetReserveXCall;
    ///Container type for all input parameters for the `getReserveY` function with signature `getReserveY()` and selector `0xfed3dfda`
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
    #[ethcall(name = "getReserveY", abi = "getReserveY()")]
    pub struct GetReserveYCall;
    ///Container type for all input parameters for the `getSpotPrice` function with signature `getSpotPrice()` and selector `0xdc76fabc`
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
    #[ethcall(name = "getSpotPrice", abi = "getSpotPrice()")]
    pub struct GetSpotPriceCall;
    ///Container type for all input parameters for the `getSpotPriceFromY` function with signature `getSpotPriceFromY()` and selector `0x89e2c259`
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
    #[ethcall(name = "getSpotPriceFromY", abi = "getSpotPriceFromY()")]
    pub struct GetSpotPriceFromYCall;
    ///Container type for all input parameters for the `getStrategyData` function with signature `getStrategyData()` and selector `0x876b55f1`
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
    #[ethcall(name = "getStrategyData", abi = "getStrategyData()")]
    pub struct GetStrategyDataCall;
    ///Container type for all input parameters for the `getSwapConstantGivenLiquidity` function with signature `getSwapConstantGivenLiquidity(uint256)` and selector `0x4166d348`
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
        name = "getSwapConstantGivenLiquidity",
        abi = "getSwapConstantGivenLiquidity(uint256)"
    )]
    pub struct GetSwapConstantGivenLiquidityCall {
        pub new_liquidity: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getSwapFee` function with signature `getSwapFee()` and selector `0xd4cadf68`
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
    #[ethcall(name = "getSwapFee", abi = "getSwapFee()")]
    pub struct GetSwapFeeCall;
    ///Container type for all input parameters for the `initExactTokensAndLiquidity` function with signature `initExactTokensAndLiquidity(uint256,uint256,uint256)` and selector `0x9e1932d8`
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
        name = "initExactTokensAndLiquidity",
        abi = "initExactTokensAndLiquidity(uint256,uint256,uint256)"
    )]
    pub struct InitExactTokensAndLiquidityCall {
        pub amount_x: ::ethers::core::types::U256,
        pub amount_y: ::ethers::core::types::U256,
        pub liquidity: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `initExactX` function with signature `initExactX(uint256,uint256)` and selector `0xf9a1c85a`
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
    #[ethcall(name = "initExactX", abi = "initExactX(uint256,uint256)")]
    pub struct InitExactXCall {
        pub amount_x: ::ethers::core::types::U256,
        pub price: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `initExactY` function with signature `initExactY(uint256,uint256)` and selector `0xa59c186f`
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
    #[ethcall(name = "initExactY", abi = "initExactY(uint256,uint256)")]
    pub struct InitExactYCall {
        pub amount_y: ::ethers::core::types::U256,
        pub price: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `initPool` function with signature `initPool(bool,uint256,uint256)` and selector `0x04afa822`
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
    #[ethcall(name = "initPool", abi = "initPool(bool,uint256,uint256)")]
    pub struct InitPoolCall {
        pub exact_x: bool,
        pub amount: ::ethers::core::types::U256,
        pub price: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `instantiate` function with signature `instantiate(uint256,uint256)` and selector `0x3639aa32`
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
    #[ethcall(name = "instantiate", abi = "instantiate(uint256,uint256)")]
    pub struct InstantiateCall {
        pub amount: ::ethers::core::types::U256,
        pub price: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `logData` function with signature `logData()` and selector `0xbcc17dc7`
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
    #[ethcall(name = "logData", abi = "logData()")]
    pub struct LogDataCall;
    ///Container type for all input parameters for the `removeLiquidityExactX` function with signature `removeLiquidityExactX(uint256)` and selector `0xccd1e4be`
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
    #[ethcall(name = "removeLiquidityExactX", abi = "removeLiquidityExactX(uint256)")]
    pub struct RemoveLiquidityExactXCall {
        pub amount_x: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `removeLiquidityExactY` function with signature `removeLiquidityExactY(uint256)` and selector `0x7d69537e`
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
    #[ethcall(name = "removeLiquidityExactY", abi = "removeLiquidityExactY(uint256)")]
    pub struct RemoveLiquidityExactYCall {
        pub amount_y: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `reserveX` function with signature `reserveX()` and selector `0x08eabdda`
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
    #[ethcall(name = "reserveX", abi = "reserveX()")]
    pub struct ReserveXCall;
    ///Container type for all input parameters for the `reserveY` function with signature `reserveY()` and selector `0xfadfa65b`
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
    #[ethcall(name = "reserveY", abi = "reserveY()")]
    pub struct ReserveYCall;
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
    ///Container type for all input parameters for the `setSwapFee` function with signature `setSwapFee(uint256)` and selector `0x34e19907`
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
    #[ethcall(name = "setSwapFee", abi = "setSwapFee(uint256)")]
    pub struct SetSwapFeeCall {
        pub new_swap_fee: ::ethers::core::types::U256,
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
    ///Container type for all input parameters for the `swap` function with signature `swap(bool,uint256,uint256,uint256)` and selector `0x59542ca9`
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
    #[ethcall(name = "swap", abi = "swap(bool,uint256,uint256,uint256)")]
    pub struct SwapCall {
        pub swap_direction: bool,
        pub next_liquidity: ::ethers::core::types::U256,
        pub amount_in: ::ethers::core::types::U256,
        pub amount_out: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `swapAmountIn` function with signature `swapAmountIn(bool,uint256,uint256,uint256)` and selector `0xf2d1922f`
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
    #[ethcall(name = "swapAmountIn", abi = "swapAmountIn(bool,uint256,uint256,uint256)")]
    pub struct SwapAmountInCall {
        pub swap_direction: bool,
        pub next_liquidity: ::ethers::core::types::U256,
        pub amount_in: ::ethers::core::types::U256,
        pub amount_out: ::ethers::core::types::U256,
    }
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
    ///Container type for all input parameters for the `tokenX` function with signature `tokenX()` and selector `0x16dc165b`
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
    #[ethcall(name = "tokenX", abi = "tokenX()")]
    pub struct TokenXCall;
    ///Container type for all input parameters for the `tokenY` function with signature `tokenY()` and selector `0xb7d19fc4`
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
    #[ethcall(name = "tokenY", abi = "tokenY()")]
    pub struct TokenYCall;
    ///Container type for all input parameters for the `totalLiquidity` function with signature `totalLiquidity()` and selector `0x15770f92`
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
    #[ethcall(name = "totalLiquidity", abi = "totalLiquidity()")]
    pub struct TotalLiquidityCall;
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
    pub enum RMMCalls {
        AddLiquidity(AddLiquidityCall),
        AddLiquidityExactX(AddLiquidityExactXCall),
        AddLiquidityExactY(AddLiquidityExactYCall),
        BalanceOf(BalanceOfCall),
        ComputeInitialPoolState(ComputeInitialPoolStateCall),
        GetAmountOut(GetAmountOutCall),
        GetDeltaL(GetDeltaLCall),
        GetInvariant(GetInvariantCall),
        GetLiquidity(GetLiquidityCall),
        GetNextLiquidity(GetNextLiquidityCall),
        GetParams(GetParamsCall),
        GetPortfolioValue(GetPortfolioValueCall),
        GetReserveX(GetReserveXCall),
        GetReserveY(GetReserveYCall),
        GetSpotPrice(GetSpotPriceCall),
        GetSpotPriceFromY(GetSpotPriceFromYCall),
        GetStrategyData(GetStrategyDataCall),
        GetSwapConstantGivenLiquidity(GetSwapConstantGivenLiquidityCall),
        GetSwapFee(GetSwapFeeCall),
        InitExactTokensAndLiquidity(InitExactTokensAndLiquidityCall),
        InitExactX(InitExactXCall),
        InitExactY(InitExactYCall),
        InitPool(InitPoolCall),
        Instantiate(InstantiateCall),
        LogData(LogDataCall),
        RemoveLiquidityExactX(RemoveLiquidityExactXCall),
        RemoveLiquidityExactY(RemoveLiquidityExactYCall),
        ReserveX(ReserveXCall),
        ReserveY(ReserveYCall),
        SetSigma(SetSigmaCall),
        SetStrikePrice(SetStrikePriceCall),
        SetSwapFee(SetSwapFeeCall),
        SetTau(SetTauCall),
        Sigma(SigmaCall),
        StrikePrice(StrikePriceCall),
        Swap(SwapCall),
        SwapAmountIn(SwapAmountInCall),
        SwapFee(SwapFeeCall),
        TargetSigma(TargetSigmaCall),
        TargetStrike(TargetStrikeCall),
        Tau(TauCall),
        TokenX(TokenXCall),
        TokenY(TokenYCall),
        TotalLiquidity(TotalLiquidityCall),
    }
    impl ::ethers::core::abi::AbiDecode for RMMCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AddLiquidityCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AddLiquidity(decoded));
            }
            if let Ok(decoded) = <AddLiquidityExactXCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AddLiquidityExactX(decoded));
            }
            if let Ok(decoded) = <AddLiquidityExactYCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AddLiquidityExactY(decoded));
            }
            if let Ok(decoded) = <BalanceOfCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BalanceOf(decoded));
            }
            if let Ok(decoded) = <ComputeInitialPoolStateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ComputeInitialPoolState(decoded));
            }
            if let Ok(decoded) = <GetAmountOutCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetAmountOut(decoded));
            }
            if let Ok(decoded) = <GetDeltaLCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetDeltaL(decoded));
            }
            if let Ok(decoded) = <GetInvariantCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetInvariant(decoded));
            }
            if let Ok(decoded) = <GetLiquidityCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetLiquidity(decoded));
            }
            if let Ok(decoded) = <GetNextLiquidityCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetNextLiquidity(decoded));
            }
            if let Ok(decoded) = <GetParamsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetParams(decoded));
            }
            if let Ok(decoded) = <GetPortfolioValueCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetPortfolioValue(decoded));
            }
            if let Ok(decoded) = <GetReserveXCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetReserveX(decoded));
            }
            if let Ok(decoded) = <GetReserveYCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetReserveY(decoded));
            }
            if let Ok(decoded) = <GetSpotPriceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetSpotPrice(decoded));
            }
            if let Ok(decoded) = <GetSpotPriceFromYCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetSpotPriceFromY(decoded));
            }
            if let Ok(decoded) = <GetStrategyDataCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetStrategyData(decoded));
            }
            if let Ok(decoded) = <GetSwapConstantGivenLiquidityCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetSwapConstantGivenLiquidity(decoded));
            }
            if let Ok(decoded) = <GetSwapFeeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetSwapFee(decoded));
            }
            if let Ok(decoded) = <InitExactTokensAndLiquidityCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InitExactTokensAndLiquidity(decoded));
            }
            if let Ok(decoded) = <InitExactXCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InitExactX(decoded));
            }
            if let Ok(decoded) = <InitExactYCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InitExactY(decoded));
            }
            if let Ok(decoded) = <InitPoolCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InitPool(decoded));
            }
            if let Ok(decoded) = <InstantiateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Instantiate(decoded));
            }
            if let Ok(decoded) = <LogDataCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LogData(decoded));
            }
            if let Ok(decoded) = <RemoveLiquidityExactXCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RemoveLiquidityExactX(decoded));
            }
            if let Ok(decoded) = <RemoveLiquidityExactYCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RemoveLiquidityExactY(decoded));
            }
            if let Ok(decoded) = <ReserveXCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ReserveX(decoded));
            }
            if let Ok(decoded) = <ReserveYCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ReserveY(decoded));
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
            if let Ok(decoded) = <SetSwapFeeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetSwapFee(decoded));
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
            if let Ok(decoded) = <StrikePriceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::StrikePrice(decoded));
            }
            if let Ok(decoded) = <SwapCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Swap(decoded));
            }
            if let Ok(decoded) = <SwapAmountInCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SwapAmountIn(decoded));
            }
            if let Ok(decoded) = <SwapFeeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SwapFee(decoded));
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
            if let Ok(decoded) = <TokenXCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TokenX(decoded));
            }
            if let Ok(decoded) = <TokenYCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TokenY(decoded));
            }
            if let Ok(decoded) = <TotalLiquidityCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TotalLiquidity(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for RMMCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AddLiquidity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddLiquidityExactX(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddLiquidityExactY(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BalanceOf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ComputeInitialPoolState(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetAmountOut(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetDeltaL(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetInvariant(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetLiquidity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetNextLiquidity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetParams(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetPortfolioValue(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetReserveX(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetReserveY(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetSpotPrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetSpotPriceFromY(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetStrategyData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetSwapConstantGivenLiquidity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetSwapFee(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InitExactTokensAndLiquidity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InitExactX(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InitExactY(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InitPool(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Instantiate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LogData(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RemoveLiquidityExactX(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemoveLiquidityExactY(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ReserveX(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ReserveY(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetSigma(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetStrikePrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetSwapFee(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetTau(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Sigma(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::StrikePrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Swap(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SwapAmountIn(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SwapFee(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TargetSigma(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TargetStrike(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Tau(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TokenX(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TokenY(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TotalLiquidity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for RMMCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddLiquidity(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddLiquidityExactX(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AddLiquidityExactY(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BalanceOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::ComputeInitialPoolState(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetAmountOut(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetDeltaL(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetInvariant(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetLiquidity(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetNextLiquidity(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetParams(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPortfolioValue(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetReserveX(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetReserveY(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSpotPrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSpotPriceFromY(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetStrategyData(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSwapConstantGivenLiquidity(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetSwapFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::InitExactTokensAndLiquidity(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InitExactX(element) => ::core::fmt::Display::fmt(element, f),
                Self::InitExactY(element) => ::core::fmt::Display::fmt(element, f),
                Self::InitPool(element) => ::core::fmt::Display::fmt(element, f),
                Self::Instantiate(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogData(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveLiquidityExactX(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RemoveLiquidityExactY(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ReserveX(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReserveY(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetSigma(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetStrikePrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetSwapFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetTau(element) => ::core::fmt::Display::fmt(element, f),
                Self::Sigma(element) => ::core::fmt::Display::fmt(element, f),
                Self::StrikePrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::Swap(element) => ::core::fmt::Display::fmt(element, f),
                Self::SwapAmountIn(element) => ::core::fmt::Display::fmt(element, f),
                Self::SwapFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::TargetSigma(element) => ::core::fmt::Display::fmt(element, f),
                Self::TargetStrike(element) => ::core::fmt::Display::fmt(element, f),
                Self::Tau(element) => ::core::fmt::Display::fmt(element, f),
                Self::TokenX(element) => ::core::fmt::Display::fmt(element, f),
                Self::TokenY(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalLiquidity(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AddLiquidityCall> for RMMCalls {
        fn from(value: AddLiquidityCall) -> Self {
            Self::AddLiquidity(value)
        }
    }
    impl ::core::convert::From<AddLiquidityExactXCall> for RMMCalls {
        fn from(value: AddLiquidityExactXCall) -> Self {
            Self::AddLiquidityExactX(value)
        }
    }
    impl ::core::convert::From<AddLiquidityExactYCall> for RMMCalls {
        fn from(value: AddLiquidityExactYCall) -> Self {
            Self::AddLiquidityExactY(value)
        }
    }
    impl ::core::convert::From<BalanceOfCall> for RMMCalls {
        fn from(value: BalanceOfCall) -> Self {
            Self::BalanceOf(value)
        }
    }
    impl ::core::convert::From<ComputeInitialPoolStateCall> for RMMCalls {
        fn from(value: ComputeInitialPoolStateCall) -> Self {
            Self::ComputeInitialPoolState(value)
        }
    }
    impl ::core::convert::From<GetAmountOutCall> for RMMCalls {
        fn from(value: GetAmountOutCall) -> Self {
            Self::GetAmountOut(value)
        }
    }
    impl ::core::convert::From<GetDeltaLCall> for RMMCalls {
        fn from(value: GetDeltaLCall) -> Self {
            Self::GetDeltaL(value)
        }
    }
    impl ::core::convert::From<GetInvariantCall> for RMMCalls {
        fn from(value: GetInvariantCall) -> Self {
            Self::GetInvariant(value)
        }
    }
    impl ::core::convert::From<GetLiquidityCall> for RMMCalls {
        fn from(value: GetLiquidityCall) -> Self {
            Self::GetLiquidity(value)
        }
    }
    impl ::core::convert::From<GetNextLiquidityCall> for RMMCalls {
        fn from(value: GetNextLiquidityCall) -> Self {
            Self::GetNextLiquidity(value)
        }
    }
    impl ::core::convert::From<GetParamsCall> for RMMCalls {
        fn from(value: GetParamsCall) -> Self {
            Self::GetParams(value)
        }
    }
    impl ::core::convert::From<GetPortfolioValueCall> for RMMCalls {
        fn from(value: GetPortfolioValueCall) -> Self {
            Self::GetPortfolioValue(value)
        }
    }
    impl ::core::convert::From<GetReserveXCall> for RMMCalls {
        fn from(value: GetReserveXCall) -> Self {
            Self::GetReserveX(value)
        }
    }
    impl ::core::convert::From<GetReserveYCall> for RMMCalls {
        fn from(value: GetReserveYCall) -> Self {
            Self::GetReserveY(value)
        }
    }
    impl ::core::convert::From<GetSpotPriceCall> for RMMCalls {
        fn from(value: GetSpotPriceCall) -> Self {
            Self::GetSpotPrice(value)
        }
    }
    impl ::core::convert::From<GetSpotPriceFromYCall> for RMMCalls {
        fn from(value: GetSpotPriceFromYCall) -> Self {
            Self::GetSpotPriceFromY(value)
        }
    }
    impl ::core::convert::From<GetStrategyDataCall> for RMMCalls {
        fn from(value: GetStrategyDataCall) -> Self {
            Self::GetStrategyData(value)
        }
    }
    impl ::core::convert::From<GetSwapConstantGivenLiquidityCall> for RMMCalls {
        fn from(value: GetSwapConstantGivenLiquidityCall) -> Self {
            Self::GetSwapConstantGivenLiquidity(value)
        }
    }
    impl ::core::convert::From<GetSwapFeeCall> for RMMCalls {
        fn from(value: GetSwapFeeCall) -> Self {
            Self::GetSwapFee(value)
        }
    }
    impl ::core::convert::From<InitExactTokensAndLiquidityCall> for RMMCalls {
        fn from(value: InitExactTokensAndLiquidityCall) -> Self {
            Self::InitExactTokensAndLiquidity(value)
        }
    }
    impl ::core::convert::From<InitExactXCall> for RMMCalls {
        fn from(value: InitExactXCall) -> Self {
            Self::InitExactX(value)
        }
    }
    impl ::core::convert::From<InitExactYCall> for RMMCalls {
        fn from(value: InitExactYCall) -> Self {
            Self::InitExactY(value)
        }
    }
    impl ::core::convert::From<InitPoolCall> for RMMCalls {
        fn from(value: InitPoolCall) -> Self {
            Self::InitPool(value)
        }
    }
    impl ::core::convert::From<InstantiateCall> for RMMCalls {
        fn from(value: InstantiateCall) -> Self {
            Self::Instantiate(value)
        }
    }
    impl ::core::convert::From<LogDataCall> for RMMCalls {
        fn from(value: LogDataCall) -> Self {
            Self::LogData(value)
        }
    }
    impl ::core::convert::From<RemoveLiquidityExactXCall> for RMMCalls {
        fn from(value: RemoveLiquidityExactXCall) -> Self {
            Self::RemoveLiquidityExactX(value)
        }
    }
    impl ::core::convert::From<RemoveLiquidityExactYCall> for RMMCalls {
        fn from(value: RemoveLiquidityExactYCall) -> Self {
            Self::RemoveLiquidityExactY(value)
        }
    }
    impl ::core::convert::From<ReserveXCall> for RMMCalls {
        fn from(value: ReserveXCall) -> Self {
            Self::ReserveX(value)
        }
    }
    impl ::core::convert::From<ReserveYCall> for RMMCalls {
        fn from(value: ReserveYCall) -> Self {
            Self::ReserveY(value)
        }
    }
    impl ::core::convert::From<SetSigmaCall> for RMMCalls {
        fn from(value: SetSigmaCall) -> Self {
            Self::SetSigma(value)
        }
    }
    impl ::core::convert::From<SetStrikePriceCall> for RMMCalls {
        fn from(value: SetStrikePriceCall) -> Self {
            Self::SetStrikePrice(value)
        }
    }
    impl ::core::convert::From<SetSwapFeeCall> for RMMCalls {
        fn from(value: SetSwapFeeCall) -> Self {
            Self::SetSwapFee(value)
        }
    }
    impl ::core::convert::From<SetTauCall> for RMMCalls {
        fn from(value: SetTauCall) -> Self {
            Self::SetTau(value)
        }
    }
    impl ::core::convert::From<SigmaCall> for RMMCalls {
        fn from(value: SigmaCall) -> Self {
            Self::Sigma(value)
        }
    }
    impl ::core::convert::From<StrikePriceCall> for RMMCalls {
        fn from(value: StrikePriceCall) -> Self {
            Self::StrikePrice(value)
        }
    }
    impl ::core::convert::From<SwapCall> for RMMCalls {
        fn from(value: SwapCall) -> Self {
            Self::Swap(value)
        }
    }
    impl ::core::convert::From<SwapAmountInCall> for RMMCalls {
        fn from(value: SwapAmountInCall) -> Self {
            Self::SwapAmountIn(value)
        }
    }
    impl ::core::convert::From<SwapFeeCall> for RMMCalls {
        fn from(value: SwapFeeCall) -> Self {
            Self::SwapFee(value)
        }
    }
    impl ::core::convert::From<TargetSigmaCall> for RMMCalls {
        fn from(value: TargetSigmaCall) -> Self {
            Self::TargetSigma(value)
        }
    }
    impl ::core::convert::From<TargetStrikeCall> for RMMCalls {
        fn from(value: TargetStrikeCall) -> Self {
            Self::TargetStrike(value)
        }
    }
    impl ::core::convert::From<TauCall> for RMMCalls {
        fn from(value: TauCall) -> Self {
            Self::Tau(value)
        }
    }
    impl ::core::convert::From<TokenXCall> for RMMCalls {
        fn from(value: TokenXCall) -> Self {
            Self::TokenX(value)
        }
    }
    impl ::core::convert::From<TokenYCall> for RMMCalls {
        fn from(value: TokenYCall) -> Self {
            Self::TokenY(value)
        }
    }
    impl ::core::convert::From<TotalLiquidityCall> for RMMCalls {
        fn from(value: TotalLiquidityCall) -> Self {
            Self::TotalLiquidity(value)
        }
    }
    ///Container type for all return fields from the `addLiquidity` function with signature `addLiquidity(bool,uint256)` and selector `0x76701636`
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
    pub struct AddLiquidityReturn {
        pub amount_x: ::ethers::core::types::U256,
        pub amount_y: ::ethers::core::types::U256,
        pub liquidity: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `addLiquidityExactX` function with signature `addLiquidityExactX(uint256)` and selector `0x02c2e55d`
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
    pub struct AddLiquidityExactXReturn(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `addLiquidityExactY` function with signature `addLiquidityExactY(uint256)` and selector `0x9c9da9ea`
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
    pub struct AddLiquidityExactYReturn(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `balanceOf` function with signature `balanceOf(address)` and selector `0x70a08231`
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
    pub struct BalanceOfReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `computeInitialPoolState` function with signature `computeInitialPoolState(uint256,uint256)` and selector `0x37aa7381`
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
    pub struct ComputeInitialPoolStateReturn {
        pub initial_x: ::ethers::core::types::U256,
        pub initial_y: ::ethers::core::types::U256,
        pub initial_liquidity: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `getAmountOut` function with signature `getAmountOut(bool,uint256,uint256)` and selector `0x5d840ae5`
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
    pub struct GetAmountOutReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getDeltaL` function with signature `getDeltaL(bool,uint256,uint256)` and selector `0xcf78e48f`
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
    pub struct GetDeltaLReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getInvariant` function with signature `getInvariant()` and selector `0xc0ff1a15`
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
    pub struct GetInvariantReturn(pub ::ethers::core::types::I256);
    ///Container type for all return fields from the `getLiquidity` function with signature `getLiquidity()` and selector `0x0910a510`
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
    pub struct GetLiquidityReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getNextLiquidity` function with signature `getNextLiquidity()` and selector `0xf3a8efe3`
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
    ///Container type for all return fields from the `getPortfolioValue` function with signature `getPortfolioValue()` and selector `0xbb0498de`
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
    pub struct GetPortfolioValueReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getReserveX` function with signature `getReserveX()` and selector `0x559d1602`
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
    pub struct GetReserveXReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getReserveY` function with signature `getReserveY()` and selector `0xfed3dfda`
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
    pub struct GetReserveYReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getSpotPrice` function with signature `getSpotPrice()` and selector `0xdc76fabc`
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
    pub struct GetSpotPriceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getSpotPriceFromY` function with signature `getSpotPriceFromY()` and selector `0x89e2c259`
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
    pub struct GetSpotPriceFromYReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getStrategyData` function with signature `getStrategyData()` and selector `0x876b55f1`
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
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all return fields from the `getSwapConstantGivenLiquidity` function with signature `getSwapConstantGivenLiquidity(uint256)` and selector `0x4166d348`
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
    pub struct GetSwapConstantGivenLiquidityReturn(pub ::ethers::core::types::I256);
    ///Container type for all return fields from the `getSwapFee` function with signature `getSwapFee()` and selector `0xd4cadf68`
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
    pub struct GetSwapFeeReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `initExactX` function with signature `initExactX(uint256,uint256)` and selector `0xf9a1c85a`
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
    pub struct InitExactXReturn(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `initExactY` function with signature `initExactY(uint256,uint256)` and selector `0xa59c186f`
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
    pub struct InitExactYReturn(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `initPool` function with signature `initPool(bool,uint256,uint256)` and selector `0x04afa822`
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
    pub struct InitPoolReturn {
        pub amount_x: ::ethers::core::types::U256,
        pub amount_y: ::ethers::core::types::U256,
        pub liquidity: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `instantiate` function with signature `instantiate(uint256,uint256)` and selector `0x3639aa32`
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
    pub struct InstantiateReturn {
        pub amount_x: ::ethers::core::types::U256,
        pub amount_y: ::ethers::core::types::U256,
        pub liquidity: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `removeLiquidityExactX` function with signature `removeLiquidityExactX(uint256)` and selector `0xccd1e4be`
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
    pub struct RemoveLiquidityExactXReturn(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `removeLiquidityExactY` function with signature `removeLiquidityExactY(uint256)` and selector `0x7d69537e`
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
    pub struct RemoveLiquidityExactYReturn(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `reserveX` function with signature `reserveX()` and selector `0x08eabdda`
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
    pub struct ReserveXReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `reserveY` function with signature `reserveY()` and selector `0xfadfa65b`
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
    pub struct ReserveYReturn(pub ::ethers::core::types::U256);
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
    ///Container type for all return fields from the `swap` function with signature `swap(bool,uint256,uint256,uint256)` and selector `0x59542ca9`
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
    pub struct SwapReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `swapAmountIn` function with signature `swapAmountIn(bool,uint256,uint256,uint256)` and selector `0xf2d1922f`
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
    pub struct SwapAmountInReturn(pub ::ethers::core::types::U256);
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
    ///Container type for all return fields from the `tokenX` function with signature `tokenX()` and selector `0x16dc165b`
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
    pub struct TokenXReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `tokenY` function with signature `tokenY()` and selector `0xb7d19fc4`
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
    pub struct TokenYReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `totalLiquidity` function with signature `totalLiquidity()` and selector `0x15770f92`
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
    pub struct TotalLiquidityReturn(pub ::ethers::core::types::U256);
}
