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
    pub static RMM_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15b\0\0^W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`@Qb\0;h8\x03\x80b\0;h\x839\x81\x01`@\x81\x90Rb\0\0\x81\x91b\0\x01hV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x80\x89\x16`\x01`\x01`\xA0\x1B\x03\x19\x92\x83\x16\x17\x90\x92U`\x01\x80T\x92\x88\x16\x92\x90\x91\x16\x91\x90\x91\x17\x90U`\x08\x84\x90U`\x07\x84\x90UB`\x0B\x81\x90U`\t\x81\x90U`\r\x84\x90U`\x0C\x84\x90U`\x10\x81\x90U`\x0E\x81\x90U`\x12\x83\x90U`\x11\x83\x90U`\x15\x81\x90U`\x13Ug\r\xE0\xB6\xB3\xA7d\0\0\x81\x10b\0\x01<W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01Rp\nn\xEC.\x04\x0C\xCC\xAC\xA4\x0E\x8D\xED\xE4\r\r,\xED`{\x1B`D\x82\x01R`d\x01`@Q\x80\x91\x03\x90\xFD[`\x02UPb\0\x02\x10\x93PPPPV[\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x01cW`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15b\0\x01\xCDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[b\0\x01\xD8\x87b\0\x01KV[\x95Pb\0\x01\xE8` \x88\x01b\0\x01KV[\x94P`@\x87\x01Q\x93P``\x87\x01Q\x92P`\x80\x87\x01Q\x91P`\xA0\x87\x01Q\x90P\x92\x95P\x92\x95P\x92\x95V[a9H\x80b\0\x02 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x02\x7FW`\x005`\xE0\x1C\x80c\x8DR\xA1\xFC\x11a\x01}W\x80c\xC1e&\x12\x11a\x01\x05W\x80c\xDCv\xFA\xBC\x11a\0\xC9W\x80c\xDCv\xFA\xBC\x14a\x05#W\x80c\xF3\xA8\xEF\xE3\x14a\x05+W\x80c\xF9\xA1\xC8Z\x14a\x053W\x80c\xFA\xDF\xA6[\x14a\x05FW\x80c\xFE\xD3\xDF\xDA\x14a\x05OWa\x02\x7FV[\x80c\xC1e&\x12\x14a\x04\xEFW\x80c\xC5)\x87\xCF\x14a\x04\xF8W\x80c\xCC\xD1\xE4\xBE\x14a\x05\0W\x80c\xCF\xC4\xAFU\x14a\x05\x13W\x80c\xD4\xCA\xDFh\x14a\x05\x1BWa\x02\x7FV[\x80c\xAF\xDF1\xCD\x11a\x01LW\x80c\xAF\xDF1\xCD\x14a\x04\xBCW\x80c\xB7\xD1\x9F\xC4\x14a\x04\xC4W\x80c\xBB\x04\x98\xDE\x14a\x04\xD7W\x80c\xBC\xC1}\xC7\x14a\x04\xDFW\x80c\xC0\xFF\x1A\x15\x14a\x04\xE7Wa\x02\x7FV[\x80c\x8DR\xA1\xFC\x14a\x04pW\x80c\x99\x87\xFEd\x14a\x04\x83W\x80c\x9C\x9D\xA9\xEA\x14a\x04\x96W\x80c\xA5\x9C\x18o\x14a\x04\xA9Wa\x02\x7FV[\x80cAf\xD3H\x11a\x02\x0BW\x80cvp\x166\x11a\x01\xCFW\x80cvp\x166\x14a\x04\x0FW\x80c}iS~\x14a\x04\"W\x80c\x7F\x0EL\x8C\x14a\x045W\x80c\x82\xBE\xAD\xED\x14a\x04HW\x80c\x87kU\xF1\x14a\x04[Wa\x02\x7FV[\x80cAf\xD3H\x14a\x03\xC3W\x80cT\xCF*\xEB\x14a\x03\xD6W\x80cU\x9D\x16\x02\x14a\x03\xDFW\x80c^aZk\x14a\x03\xE7W\x80cp\xA0\x821\x14a\x03\xEFWa\x02\x7FV[\x80c\x15w\x0F\x92\x11a\x02RW\x80c\x15w\x0F\x92\x14a\x03^W\x80c\x16\xDC\x16[\x14a\x03gW\x80c4\xE1\x99\x07\x14a\x03\x92W\x80c69\xAA2\x14a\x03\xA7W\x80c>\x1E3\x92\x14a\x03\xBAWa\x02\x7FV[\x80c\x02\xC2\xE5]\x14a\x02\xE4W\x80c\x04\xAF\xA8\"\x14a\x03\x11W\x80c\x08\xEA\xBD\xDA\x14a\x03?W\x80c\t\x10\xA5\x10\x14a\x03VW[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x02\xF7a\x02\xF26`\x04a5IV[a\x05WV[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01[`@Q\x80\x91\x03\x90\xF3[a\x03$a\x03\x1F6`\x04a5vV[a\x07\xFAV[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x03\x08V[a\x03H`\x03T\x81V[`@Q\x90\x81R` \x01a\x03\x08V[`\x05Ta\x03HV[a\x03H`\x05T\x81V[`\0Ta\x03z\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x03\x08V[a\x03\xA5a\x03\xA06`\x04a5IV[a\nkV[\0[a\x03$a\x03\xB56`\x04a5\xAEV[a\n\xBFV[a\x03H`\rT\x81V[a\x03Ha\x03\xD16`\x04a5IV[a\n\xDDV[a\x03H`\x02T\x81V[`\x03Ta\x03HV[a\x03$a\x0B\x0FV[a\x03Ha\x03\xFD6`\x04a5\xD3V[`\x06` R`\0\x90\x81R`@\x90 T\x81V[a\x03$a\x04\x1D6`\x04a6\x06V[a\x0B7V[a\x02\xF7a\x0406`\x04a5IV[a\x0E\x9CV[a\x03\xA5a\x04C6`\x04a5\xAEV[a\x11\x0EV[a\x03Ha\x04V6`\x04a5vV[a\x11\xD3V[a\x04ca\x11\xE8V[`@Qa\x03\x08\x91\x90a65V[a\x03\xA5a\x04~6`\x04a5\xAEV[a\x120V[a\x03\xA5a\x04\x916`\x04a5\xAEV[a\x12\xBCV[a\x02\xF7a\x04\xA46`\x04a5IV[a\x13HV[a\x02\xF7a\x04\xB76`\x04a5\xAEV[a\x15\xCFV[a\x03Ha\x17\xABV[`\x01Ta\x03z\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x03Ha\x18\x1BV[a\x03\xA5a\x18RV[a\x03Ha\x18\x9FV[a\x03H`\x08T\x81V[a\x03Ha\x18\xBAV[a\x02\xF7a\x05\x0E6`\x04a5IV[a\x19%V[a\x03Ha\x1B\x97V[`\x02Ta\x03HV[a\x03Ha\x1C\x02V[a\x03Ha\x1C*V[a\x02\xF7a\x05A6`\x04a5\xAEV[a\x1CZV[a\x03H`\x04T\x81V[`\x04Ta\x03HV[`\0\x80`\0\x80`\0a\x05ga\x0B\x0FV[\x92P\x92P\x92P`\0a\x05\x80`\x03T`\x05T\x86\x86\x86a\x1D\x97V[\x90P`\0a\x05\x9E\x88`\x03Ta\x05\x95\x91\x90a6\x99V[\x83\x87\x87\x87a\x1E\x1BV[\x90P`\0a\x05\xAF\x82\x84\x88\x88\x88a\x1E[V[\x90P`\0`\x04T\x82a\x05\xC1\x91\x90a6\xACV[\x90P`\0`\x05T\x84a\x05\xD3\x91\x90a6\xACV[\x90P\x83`\x05\x81\x90UP\x8A`\x03`\0\x82\x82Ta\x05\xEE\x91\x90a6\x99V[\x92PP\x81\x90UP\x81`\x04`\0\x82\x82Ta\x06\x07\x91\x90a6\x99V[\x92PP\x81\x90UP`\0\x80T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c#\xB8r\xDD30\x8E`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x06Q\x93\x92\x91\x90a6\xBFV[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x06\xA9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a9\x08\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x06\xBDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xE1\x91\x90a6\xE3V[P`\x01T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\x07\x16\x903\x900\x90\x87\x90`\x04\x01a6\xBFV[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x07nW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a9\x08\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x07\x82W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\xA6\x91\x90a6\xE3V[P`@\x80Q\x85\x81R` \x81\x01\x8D\x90R\x90\x81\x01\x83\x90R3\x90\x7F\xBE\xB3\x88W\x86\xD67\xA4t\xCB\xC2\x87\xC0\xA4E\x87#\x163\xA0w\xF0\xBD05MZK\x18\x99o\xCE\x90``\x01[`@Q\x80\x91\x03\x90\xA2\x9A\x90\x99P\x97PPPPPPPPV[`\0\x80`\0`\x05T`\0\x14a\x08VW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FPool already initialized\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x80`\0a\x08ca\x0B\x0FV[\x92P\x92P\x92P\x88\x15a\x08\x95W\x87\x95Pa\x08\x7F\x86\x88\x85\x85\x85a\x1E\x1BV[\x93Pa\x08\x8E\x84\x88\x85\x85\x85a\x1E[V[\x94Pa\x08\xB7V[\x87\x94Pa\x08\xA5\x85\x88\x85\x85\x85a\x1E}V[\x93Pa\x08\xB4\x84\x88\x85\x85\x85a\x1E\x93V[\x95P[`\x05\x84\x90U`\x03\x86\x90U`\x04\x85\x81U3`\0\x81\x81R`\x06` R`@\x80\x82 \x88\x90U\x90T\x90Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x92c#\xB8r\xDD\x92a\t\t\x92\x90\x910\x91\x8C\x91\x01a6\xBFV[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\taW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a9\x08\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\tuW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\x99\x91\x90a6\xE3V[P`\x01T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\t\xCE\x903\x900\x90\x8A\x90`\x04\x01a6\xBFV[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\n&W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a9\x08\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\n:W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n^\x91\x90a6\xE3V[PPPP\x93P\x93P\x93\x90PV[g\r\xE0\xB6\xB3\xA7d\0\0\x81\x10a\n\xBAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rt\t\xCC\xAE\xE4\x0En\xEC.\x04\x0C\xCC\xAC\xA4\x0E\x8D\xED\xE4\r\r,\xED`[\x1B`D\x82\x01R`d\x01a\x08MV[`\x02UV[`\0\x80`\0a\n\xD0`\x01\x86\x86a\x07\xFAV[\x92P\x92P\x92P\x92P\x92P\x92V[`\0\x80`\0\x80a\n\xEBa\x0B\x0FV[\x92P\x92P\x92Pa\x0B\x06`\x03T`\x04T`\x05T\x86\x86\x86\x8Ba\x1E\xBDV[\x95\x94PPPPPV[`\0\x80`\0a\x0B\x1Ca\x18\xBAV[a\x0B$a\x17\xABV[a\x0B,a\x1B\x97V[\x92P\x92P\x92P\x90\x91\x92V[`\0\x80`\0\x80`\x05T\x11a\x0B\x84W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rs\x14\x1B\xDB\xDB\x08\x1B\x9B\xDD\x08\x1A[\x9A]\x1AX[\x1A^\x99Y`b\x1B`D\x82\x01R`d\x01a\x08MV[`\0\x80`\0a\x0B\x91a\x0B\x0FV[\x92P\x92P\x92P`\0a\x0B\xAA`\x03T`\x05T\x86\x86\x86a\x1D\x97V[\x90P\x88\x15a\x0C\x02W\x87\x96P`\0a\x0B\xC8\x88`\x03Ta\x05\x95\x91\x90a6\x99V[\x90P`\0a\x0B\xD9\x82\x84\x88\x88\x88a\x1E[V[\x90P`\x04T\x81a\x0B\xE9\x91\x90a6\xACV[\x97P`\x05T\x82a\x0B\xF9\x91\x90a6\xACV[\x96PPPa\x0CWV[\x87\x95P`\0a\x0C!\x87`\x04Ta\x0C\x18\x91\x90a6\x99V[\x83\x87\x87\x87a\x1E}V[\x90P`\0a\x0C2\x82\x84\x88\x88\x88a\x1E\x93V[\x90P`\x03T\x81a\x0CB\x91\x90a6\xACV[\x98P`\x05T\x82a\x0CR\x91\x90a6\xACV[\x96PPP[\x84`\x05`\0\x82\x82Ta\x0Ci\x91\x90a6\x99V[\x92PP\x81\x90UP\x86`\x03`\0\x82\x82Ta\x0C\x82\x91\x90a6\x99V[\x92PP\x81\x90UP\x85`\x04`\0\x82\x82Ta\x0C\x9B\x91\x90a6\x99V[\x90\x91UPP3`\0\x90\x81R`\x06` R`@\x81 \x80T\x87\x92\x90a\x0C\xBF\x90\x84\x90a6\x99V[\x90\x91UPP`\0T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\x0C\xF8\x903\x900\x90\x8C\x90`\x04\x01a6\xBFV[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\rPW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a9\x08\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\rdW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\x88\x91\x90a6\xE3V[P`\x01T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\r\xBD\x903\x900\x90\x8B\x90`\x04\x01a6\xBFV[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0E\x15W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a9\x08\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x0E)W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0EM\x91\x90a6\xE3V[P`@\x80Q\x86\x81R` \x81\x01\x89\x90R\x90\x81\x01\x87\x90R3\x90\x7F\xBE\xB3\x88W\x86\xD67\xA4t\xCB\xC2\x87\xC0\xA4E\x87#\x163\xA0w\xF0\xBD05MZK\x18\x99o\xCE\x90``\x01`@Q\x80\x91\x03\x90\xA2PPPP\x92P\x92P\x92V[`\0\x80`\0\x80`\0a\x0E\xACa\x0B\x0FV[\x92P\x92P\x92P`\0a\x0E\xC5`\x03T`\x05T\x86\x86\x86a\x1D\x97V[\x90P`\0a\x0E\xDA\x88`\x03Ta\x0C\x18\x91\x90a6\xACV[\x90P`\0a\x0E\xEB\x82\x84\x88\x88\x88a\x1E\x93V[\x90P`\0\x81`\x03Ta\x0E\xFD\x91\x90a6\xACV[\x90P`\0\x83`\x05Ta\x0F\x0F\x91\x90a6\xACV[\x90P\x83`\x05\x81\x90UP\x81`\x03`\0\x82\x82Ta\x0F*\x91\x90a6\xACV[\x92PP\x81\x90UP\x8A`\x04`\0\x82\x82Ta\x0FC\x91\x90a6\xACV[\x90\x91UPP`\0T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0F\xD2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a9\x08\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x0F\xE6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\n\x91\x90a6\xE3V[P`\x01T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x8D\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x10\x95W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a9\x08\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x10\xA9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\xCD\x91\x90a6\xE3V[P`@\x80Q\x85\x81R` \x81\x01\x84\x90R\x90\x81\x01\x8C\x90R3\x90\x7FY\xC3\xA0\xB6\x0Cj\xB7\xDE\xB6.\x14@\xC9\xE7$A\xDBm\xB7\xDF\xE5\x14\xDB\xA8\xCB\x18\xE6\x0C\r\x89n\xFA\x90``\x01a\x07\xE3V[B\x81\x11a\x11-W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08M\x90a7\x03V[a\x115a\x1E\xF9V[`\0\x82`\x11T\x11a\x11RW`\x11Ta\x11M\x90\x84a6\xACV[a\x11`V[\x82`\x11Ta\x11`\x91\x90a6\xACV[\x90Pa\x11lB\x83a6\xACV[a\x11v\x90\x82a7DV[`\x14U`\x12\x83\x90U`\x15\x82\x90U`\0\x80Q` a9(\x839\x81Q\x91Ra\x11\x9Aa\x17\xABV[a\x11\xA2a\x18\xBAV[a\x11\xAAa\x1B\x97V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01RB``\x82\x01R`\x80\x01`@Q\x80\x91\x03\x90\xA1PPPV[`\0a\x11\xE0\x84\x84\x84a\x1F\nV[\x94\x93PPPPV[``a\x11\xF2a\x17\xABV[a\x11\xFAa\x18\xBAV[a\x12\x02a\x1B\x97V[`@\x80Q` \x81\x01\x94\x90\x94R\x83\x01\x91\x90\x91R``\x82\x01R`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[B\x81\x11a\x12OW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08M\x90a7\x03V[a\x12Wa$\x8DV[`\0\x82`\x0CT\x11a\x12tW`\x0CTa\x12o\x90\x84a6\xACV[a\x12\x82V[\x82`\x0CTa\x12\x82\x91\x90a6\xACV[\x90Pa\x12\x8EB\x83a6\xACV[a\x12\x98\x90\x82a7DV[`\x0FU`\r\x83\x90U`\x10\x82\x90U`\0\x80Q` a9(\x839\x81Q\x91Ra\x11\x9Aa\x17\xABV[B\x81\x11a\x12\xDBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08M\x90a7\x03V[a\x12\xE3a$\x9EV[`\0\x82`\x07T\x11a\x13\0W`\x07Ta\x12\xFB\x90\x84a6\xACV[a\x13\x0EV[\x82`\x07Ta\x13\x0E\x91\x90a6\xACV[\x90Pa\x13\x1AB\x83a6\xACV[a\x13$\x90\x82a7DV[`\nU`\x08\x83\x90U`\x0B\x82\x90U`\0\x80Q` a9(\x839\x81Q\x91Ra\x11\x9Aa\x17\xABV[`\0\x80`\0\x80`\0a\x13Xa\x0B\x0FV[\x92P\x92P\x92P`\0a\x13q`\x03T`\x05T\x86\x86\x86a\x1D\x97V[\x90P`\0a\x13\x86\x88`\x04Ta\x0C\x18\x91\x90a6\x99V[\x90P`\0a\x13\x97\x82\x84\x88\x88\x88a\x1E\x93V[\x90P`\0`\x03T\x82a\x13\xA9\x91\x90a6\xACV[\x90P`\0`\x05T\x84a\x13\xBB\x91\x90a6\xACV[\x90P\x83`\x05\x81\x90UP\x81`\x03`\0\x82\x82Ta\x13\xD6\x91\x90a6\x99V[\x92PP\x81\x90UP\x8A`\x04`\0\x82\x82Ta\x13\xEF\x91\x90a6\x99V[\x90\x91UPP`\0T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\x14(\x903\x900\x90\x87\x90`\x04\x01a6\xBFV[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x14\x80W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a9\x08\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x14\x94W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14\xB8\x91\x90a6\xE3V[P`\x01`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c#\xB8r\xDD30\x8E`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x14\xFE\x93\x92\x91\x90a6\xBFV[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x15VW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a9\x08\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x15jW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\x8E\x91\x90a6\xE3V[P`@\x80Q\x85\x81R` \x81\x01\x84\x90R\x90\x81\x01\x8C\x90R3\x90\x7F\xBE\xB3\x88W\x86\xD67\xA4t\xCB\xC2\x87\xC0\xA4E\x87#\x163\xA0w\xF0\xBD05MZK\x18\x99o\xCE\x90``\x01a\x07\xE3V[`\0\x80`\0\x80`\0a\x15\xDFa\x0B\x0FV[\x92P\x92P\x92P`\0a\x15\xF4\x88\x88\x86\x86\x86a\x1E}V[\x90P`\0a\x16\x05\x82\x89\x87\x87\x87a\x1E\x93V[`\x05\x83\x90U`\x03\x81\x90U`\x04\x8A\x81U`\0T`@Qc#\xB8r\xDD`\xE0\x1B\x81R\x92\x93P`\x01`\x01`\xA0\x1B\x03\x16\x91c#\xB8r\xDD\x91a\x16G\x913\x910\x91\x87\x91\x01a6\xBFV[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x16\x9FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a9\x08\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x16\xB3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\xD7\x91\x90a6\xE3V[P`\x01T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\x17\x0C\x903\x900\x90\x8E\x90`\x04\x01a6\xBFV[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x17dW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a9\x08\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x17xW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17\x9C\x91\x90a6\xE3V[P\x90\x98\x90\x97P\x95PPPPPPV[`\0`\x0BTB\x10a\x17\xBDWP`\x08T\x90V[`\x08T`\x07T\x11a\x17\xF4W`\nT`\tTa\x17\xD8\x90Ba6\xACV[a\x17\xE2\x91\x90a7XV[`\x07Ta\x17\xEF\x91\x90a6\x99V[\x90P\x90V[`\nT`\tTa\x18\x04\x90Ba6\xACV[a\x18\x0E\x91\x90a7XV[`\x07Ta\x17\xEF\x91\x90a6\xACV[`\0`\x04Tg\r\xE0\xB6\xB3\xA7d\0\0a\x181a\x1C\x02V[`\x03Ta\x18>\x91\x90a7XV[a\x18H\x91\x90a7DV[a\x17\xEF\x91\x90a6\x99V[`\0\x80Q` a9(\x839\x81Q\x91Ra\x18ia\x17\xABV[a\x18qa\x18\xBAV[a\x18ya\x1B\x97V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01RB``\x82\x01R`\x80\x01`@Q\x80\x91\x03\x90\xA1V[`\0a\x17\xEF`\x03T`\x05T`\x04Ta\x18\xB5a\x18\xBAV[a$\xAFV[`\0`\x10TB\x10a\x18\xCCWP`\rT\x90V[`\rT`\x0CT\x11a\x18\xFEW`\x0FT`\x0ETa\x18\xE7\x90Ba6\xACV[a\x18\xF1\x91\x90a7XV[`\x0CTa\x17\xEF\x91\x90a6\x99V[`\x0FT`\x0ETa\x19\x0E\x90Ba6\xACV[a\x19\x18\x91\x90a7XV[`\x0CTa\x17\xEF\x91\x90a6\xACV[`\0\x80`\0\x80`\0a\x195a\x0B\x0FV[\x92P\x92P\x92P`\0a\x19N`\x03T`\x05T\x86\x86\x86a\x1D\x97V[\x90P`\0a\x19c\x88`\x03Ta\x05\x95\x91\x90a6\xACV[\x90P`\0a\x19t\x82\x84\x88\x88\x88a\x1E[V[\x90P`\0\x81`\x04Ta\x19\x86\x91\x90a6\xACV[\x90P`\0\x83`\x05Ta\x19\x98\x91\x90a6\xACV[\x90P\x83`\x05\x81\x90UP\x8A`\x03`\0\x82\x82Ta\x19\xB3\x91\x90a6\xACV[\x92PP\x81\x90UP\x81`\x04`\0\x82\x82Ta\x19\xCC\x91\x90a6\xACV[\x90\x91UPP`\0T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x8D\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1A[W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a9\x08\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x1AoW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A\x93\x91\x90a6\xE3V[P`\x01T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1B\x1EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a9\x08\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x1B2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1BV\x91\x90a6\xE3V[P`@\x80Q\x85\x81R` \x81\x01\x8D\x90R\x90\x81\x01\x83\x90R3\x90\x7FY\xC3\xA0\xB6\x0Cj\xB7\xDE\xB6.\x14@\xC9\xE7$A\xDBm\xB7\xDF\xE5\x14\xDB\xA8\xCB\x18\xE6\x0C\r\x89n\xFA\x90``\x01a\x07\xE3V[`\0`\x15TB\x10a\x1B\xA9WP`\x12T\x90V[`\x12T`\x11T\x11a\x1B\xDBW`\x14T`\x13Ta\x1B\xC4\x90Ba6\xACV[a\x1B\xCE\x91\x90a7XV[`\x11Ta\x17\xEF\x91\x90a6\x99V[`\x14T`\x13Ta\x1B\xEB\x90Ba6\xACV[a\x1B\xF5\x91\x90a7XV[`\x11Ta\x17\xEF\x91\x90a6\xACV[`\0a\x17\xEF`\x03T`\x05Ta\x1C\x15a\x18\xBAV[a\x1C\x1Da\x17\xABV[a\x1C%a\x1B\x97V[a\x1D\x97V[`\0\x80`\0\x80a\x1C8a\x0B\x0FV[\x92P\x92P\x92Pa\x1CR`\x03T`\x04T`\x05T\x86\x86\x86a$\xDFV[\x93PPPP\x90V[`\0\x80`\0\x80`\0a\x1Cja\x0B\x0FV[\x92P\x92P\x92P`\0a\x1C\x7F\x88\x88\x86\x86\x86a\x1E\x1BV[\x90P`\0a\x1C\x90\x82\x89\x87\x87\x87a\x1E[V[`\x05\x83\x90U`\x03\x8A\x90U`\x04\x81\x81U`\0T`@Qc#\xB8r\xDD`\xE0\x1B\x81R\x92\x93P`\x01`\x01`\xA0\x1B\x03\x16\x91c#\xB8r\xDD\x91a\x1C\xD2\x913\x910\x91\x8F\x91\x01a6\xBFV[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1D*W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a9\x08\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x1D>W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1Db\x91\x90a6\xE3V[P`\x01T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\x17\x0C\x903\x900\x90\x86\x90`\x04\x01a6\xBFV[`\0\x80a\x1D\xA4\x84\x84a%\xB0V[\x90P`\0a\x1D\xB2\x85\x85a%\xD6V[\x90P`\0a\x1D\xC0\x89\x89a&\x08V[\x90Pa\x1E\x0E\x87a\x1E\t\x84g\r\xE0\xB6\xB3\xA7d\0\0\x87a\x1D\xE6a\x1D\xE1\x88\x84a6\xACV[a&&V[a\x1D\xF0\x91\x90a7oV[a\x1D\xFA\x91\x90a7\x9FV[a\x1E\x04\x91\x90a7\xCDV[a&\xC3V[a(lV[\x99\x98PPPPPPPPPV[`\0\x80a\x1E2a\x1E-\x87\x87\x87\x87a(\x81V[a(\xDFV[a\x1ED\x90g\r\xE0\xB6\xB3\xA7d\0\0a7\xCDV[\x90Pa\x1EP\x87\x82a)HV[\x97\x96PPPPPPPV[`\0\x80a\x1Ema\x1E-\x87\x87\x87\x87a)]V[\x90Pa\x1EP\x85a\x1E\t\x89\x84a(lV[`\0\x80a\x1ED\x85a\x1E\ta\x1E-\x89\x89\x89\x89a)]V[`\0\x80a\x1E\xA5a\x1E-\x87\x87\x87\x87a(\x81V[\x90Pa\x1EP\x87a\x1E\t\x83g\r\xE0\xB6\xB3\xA7d\0\0a7\xCDV[`\0\x80a\x1E\xCE\x89\x89\x89\x89\x89\x89a)\x93V[\x90Pa\x1E\x0E\x81`@Q` \x01a\x1E\xE4\x91\x90a7\xF4V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x84a)\xF1V[a\x1F\x01a\x1B\x97V[`\x11UB`\x13UV[`\0\x80`\0\x80a\x1F\x18a\x0B\x0FV[\x92P\x92P\x92P`\0a\x1F5`\x03T`\x04T`\x05T\x87\x87\x87\x8Da\x1E\xBDV[\x90P\x80`c\x19\x12\x80\x15a\x1FHWP`d\x81\x12[a\x1F\x94W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FSwap constant out of range\0\0\0\0\0\0`D\x82\x01R`d\x01a\x08MV[`\0a\x1F\xA5`\x03T\x89\x87\x87\x87a\x1D\x97V[\x90P\x88\x15a!\xE7W`\0g\r\xE0\xB6\xB3\xA7d\0\0`\x02Tg\r\xE0\xB6\xB3\xA7d\0\0a\x1F\xCE\x91\x90a6\xACV[a\x1F\xD8\x90\x8Aa7XV[a\x1F\xE2\x91\x90a7DV[\x90P`\0a\x1F\xF3\x82\x84\x89\x89\x89a\x1E\x1BV[\x90P`\x01a \x0B`\x03T`\x04T\x8C\x8E\x86\x8D\x8D\x8Da*dV[a \x15\x91\x90a7\xCDV[\x19\x97Pa \"\x81\x8Ba6\x99V[`\x05\x81\x90UP\x88`\x03`\0\x82\x82Ta :\x91\x90a6\x99V[\x92PP\x81\x90UP\x87`\x04`\0\x82\x82Ta S\x91\x90a6\xACV[\x90\x91UPP`\0T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a \x8C\x903\x900\x90\x8E\x90`\x04\x01a6\xBFV[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a \xE4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a9\x08\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a \xF8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a!\x1C\x91\x90a6\xE3V[P`\x01T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x8A\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a!\xA7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a9\x08\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a!\xBBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a!\xDF\x91\x90a6\xE3V[PPPa$\x1DV[`\0g\r\xE0\xB6\xB3\xA7d\0\0`\x02Tg\r\xE0\xB6\xB3\xA7d\0\0a\"\x08\x91\x90a6\xACV[a\"\x12\x90\x8Aa7XV[a\"\x1C\x91\x90a7DV[\x90P`\0a\"-\x82\x84\x89\x89\x89a\x1E}V[\x90P`\x01a\"E`\x03T`\x04T\x8C\x8E\x86\x8D\x8D\x8Da*\xE5V[a\"O\x91\x90a7\xCDV[\x19\x97Pa\"\\\x81\x8Ba6\x99V[`\x05\x81\x90UP\x88`\x04`\0\x82\x82Ta\"t\x91\x90a6\x99V[\x92PP\x81\x90UP\x87`\x03`\0\x82\x82Ta\"\x8D\x91\x90a6\xACV[\x90\x91UPP`\x01T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\"\xC6\x903\x900\x90\x8E\x90`\x04\x01a6\xBFV[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a#\x1EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a9\x08\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a#2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a#V\x91\x90a6\xE3V[P`\0T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x8A\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a#\xE1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a9\x08\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a#\xF5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a$\x19\x91\x90a6\xE3V[PPP[3`\x01`\x01`\xA0\x1B\x03\x16\x7F\x17fH\xF1\xF1\x1C\xDA(L\x12D\x90\x08k\xE4*\x92m\xDF\n\xE8\x87\xEB\xE7\xB1\xD6\xB37\xD8\x94'V\x8A\x89\x89a$\\`\x03T`\x05T\x8C\x8C\x8Ca\x1D\x97V[`@\x80Q\x94\x15\x15\x85R` \x85\x01\x93\x90\x93R\x91\x83\x01R``\x82\x01R`\x80\x01`@Q\x80\x91\x03\x90\xA2PPPPP\x93\x92PPPV[a$\x95a\x18\xBAV[`\x0CUB`\x0EUV[a$\xA6a\x17\xABV[`\x07UB`\tUV[`\0a$\xC8a$\xBE\x85\x84a+3V[a\x1D\xE1\x90\x85a7DV[a$\xD5a\x1D\xE1\x86\x88a7DV[a\x0B\x06\x91\x90a88V[`\0\x80`\0\x80a$\xF3\x8A\x8A\x8A\x8A\x8A\x8Aa)\x93V[\x90P`\0a%$\x82`@Q` \x01a%\x0B\x91\x90a7\xF4V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83`@\x01Qa)\xF1V[\x90P\x80`\0\x03a%:W\x88\x94PPPPPa%\xA6V[`\0\x81\x12\x15a%ZW\x88\x92Pa%S\x83`b`da+DV[\x93Pa%mV[\x88\x93Pa%j\x84`f`da+cV[\x92P[a%\x9F\x82`@Q` \x01a%\x81\x91\x90a7\xF4V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x85\x85`\x01a\x01\0a)\xF1a+\x91V[\x94PPPPP[\x96\x95PPPPPPV[`\0\x80a%\xBC\x83a,\xA2V[a%\xCA\x90c;\x9A\xCA\0a7XV[\x90Pa\x11\xE0\x84\x82a+3V[`\0\x80a%\xF4a%\xEE\x85g\x1B\xC1mgN\xC8\0\0a-FV[\x84a+3V[\x90Pa\x11\xE0g\x06\xF0[Y\xD3\xB2\0\0\x82a+3V[`\0a&\x1D\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a+DV[\x90P[\x92\x91PPV[`\0g\x06\xF0[Y\xD3\xB2\0\0\x82\x03a&?WP`\0\x91\x90PV[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x12a&gW`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a&\x88W`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a&\x95\x83`\x02a7oV[\x90P`\0a&\xA2\x82a-rV[\x90P`\0a&\xB8g\x13\xA0K\xBD\xFD\xC9\xBE\x88\x83a/\xF0V[\x90Pa\x0B\x06\x81a8`V[`\0h\x02H\xCE6\xA7\x0C\xB2k>\x19\x82\x13a&\xDEWP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a'%W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x01a\x08MV[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05`\x01`_\x1B\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1Dm\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1Dn\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[`\0a&\x1D\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a+cV[`\0\x80a(\x8E\x84\x84a%\xB0V[\x90P`\0a(\x9C\x87\x87a0\x05V[\x90P`\0a(\xAA\x86\x86a%\xD6V[\x90P\x82a(\xB7\x82\x84a88V[a(\xC9\x90g\r\xE0\xB6\xB3\xA7d\0\0a7oV[a(\xD3\x91\x90a7\x9FV[\x98\x97PPPPPPPPV[`\0\x80g\x13\xA0K\xBD\xFD\xC9\xBE\x88a(\xFDg\r\xE0\xB6\xB3\xA7d\0\0\x85a7oV[a)\x07\x91\x90a7\x9FV[\x90P`\0a)\x14\x82a8`V[\x90P`\0a)!\x82a0\x19V[\x90Pg\x1B\xC1mgN\xC8\0\0a)>g\r\xE0\xB6\xB3\xA7d\0\0\x83a7oV[a\x0B\x06\x91\x90a7\x9FV[`\0a&\x1D\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a+cV[`\0\x80a)j\x84\x84a%\xB0V[\x90P`\0a)x\x87\x87a0\x05V[\x90P`\0a)\x86\x86\x86a%\xD6V[\x90P\x82a(\xB7\x82\x84a7\xCDV[a)\xCC`@Q\x80`\xC0\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[\x95\x86R` \x86\x01\x94\x90\x94R`@\x85\x01\x92\x90\x92R``\x84\x01R`\x80\x83\x01R`\xA0\x82\x01R\x90V[`\0\x80\x83\x80` \x01\x90Q\x81\x01\x90a*\x08\x91\x90a8|V[\x90Pa*\x1C\x81`\x80\x01Q\x82`\xA0\x01Qa%\xB0V[a*?a*:\x83` \x01Qa*5\x87\x86``\x01Qa+3V[a&\x08V[a1\xFDV[a*Pa*:\x84`\0\x01Q\x87a&\x08V[a*Z\x91\x90a88V[a\x11\xE0\x91\x90a88V[`\0\x80a*q\x84\x84a%\xB0V[\x90P`\0a*\x88\x86a*\x83\x89\x8Ba6\x99V[a+3V[\x90P`\0a*\xBEa*\xABa\x1D\xE1\x8C\x8Fa*\xA1\x91\x90a6\x99V[a*5\x8C\x8Ea6\x99V[a*\xB4\x85a8`V[a\x1E-\x91\x90a7\xCDV[\x90P\x8Aa*\xCB\x83\x83a+3V[a*\xD5\x91\x90a7\xCDV[\x9C\x9BPPPPPPPPPPPPV[`\0\x80a*\xF2\x84\x84a%\xB0V[\x90P`\0a+\x04\x86a*\x83\x89\x8Ba6\x99V[\x90P`\0a+!a*\xABa\x1D\xE1a+\x1B\x8D\x8Fa6\x99V[\x85a&\x08V[\x90P\x8Ba*\xCBa+1\x8A\x8Ca6\x99V[\x83[`\0a&\x1D\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a+\\W`\0\x80\xFD[\x04\x92\x91PPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a+{W`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[`\0\x84\x86\x11\x15a+\xBEW`@Qc0\x82\xDF\xDB`\xE1\x1B\x81R`\x04\x81\x01\x87\x90R`$\x81\x01\x86\x90R`D\x01a\x08MV[`\0a+\xCE\x88\x88\x85c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a+\xE0\x89\x88\x86c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a+\xEE\x82\x84a7oV[\x13\x15a,\x17W`@Qc\x06\xF1\xBE]`\xE2\x1B\x81R`\x04\x81\x01\x83\x90R`$\x81\x01\x82\x90R`D\x01a\x08MV[`\0a,#\x89\x89a6\xACV[\x90P`\0[`\x02a,4\x8A\x8Ca6\x99V[a,>\x91\x90a7DV[\x94P`\0a,P\x8C\x87\x89c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a,^\x86\x83a7oV[\x13a,kW\x85\x99Pa,rV[\x85\x9AP\x80\x94P[a,|\x8B\x8Ba6\xACV[\x92PP`\x01\x01\x87\x82\x11\x80\x15a,\x90WP\x86\x81\x10[a,(WPPPP\x96\x95PPPPPPV[`\xB5\x81`\x01`\x88\x1B\x81\x10a,\xBBW`@\x91\x90\x91\x1B\x90`\x80\x1C[i\x01\0\0\0\0\0\0\0\0\0\x81\x10a,\xD7W` \x91\x90\x91\x1B\x90`@\x1C[e\x01\0\0\0\0\0\x81\x10a,\xEFW`\x10\x91\x90\x91\x1B\x90` \x1C[c\x01\0\0\0\x81\x10a-\x05W`\x08\x91\x90\x91\x1B\x90`\x10\x1C[b\x01\0\0\x01\x02`\x12\x1C\x80\x82\x04\x01`\x01\x90\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x90\x81\x90\x04\x81\x11\x90\x03\x90V[`\0a&\x1Dg\r\xE0\xB6\xB3\xA7d\0\0\x83a-^\x86a2\x0BV[a-h\x91\x90a7oV[a\x1E\x04\x91\x90a7\x9FV[`\0\x80\x82\x12\x80a-\x89WPg\x1B\xC1mgN\xC8\0\0\x82\x13[\x15a-\xA7W`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a-\xC8W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81g\x1B\xC1mgN\xC8\0\0\x03a-\xF0W`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x15a-\xFBW\x91\x90PV[`\0g\r\xE0\xB6\xB3\xA7d\0\0\x83\x12a.#Wa.\x1E\x83g\x1B\xC1mgN\xC8\0\0a7\xCDV[a.%V[\x82[\x90P`\0a.;\x82g\x1B\xC1mgN\xC8\0\0a3\xE6V[\x90P\x80`\0\x03a.^W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a.i\x82a2\x0BV[\x90P`\0c;\x9A\xCA\0a.\x94a.\x8Fa.\x89g\x1B\xC1mgN\xC8\0\0a8`V[\x85a/\xF0V[a,\xA2V[a.\x9E\x91\x90a7oV[\x90P`\0\x80a.\xB5\x83g\x03\xC1f\\z\xAB \0a/\xF0V[a.\xC7\x90g \x05\xFEO&\x8E\xA0\0a88V[\x90P`\0a.\xF7\x84a.\xE0\x86f\x9F2u$b\xA0\0a/\xF0V[a.\xF2\x90g\r\xC5R\x7Fd, \0a88V[a/\xF0V[a/\t\x90g\r\xE0\xB6\xB3\xA7d\0\0a88V[\x90Pa/-g\t\xD0(\xCCo _\xFF\x19\x85a/#\x85\x85a3\xE6V[a.\xF2\x91\x90a7\xCDV[\x92PPP`\0[`\x02\x81\x10\x15a/\xC8W`\0\x86a/I\x84a0\x19V[a/S\x91\x90a7\xCDV[\x90P`\0a/a\x84\x85a/\xF0V[a/j\x90a8`V[\x90P`\0a/w\x82a&\xC3V[\x90P`\0a/\x85\x86\x85a/\xF0V[a/\x97g\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x84a/\xF0V[a/\xA1\x91\x90a7\xCDV[\x90Pa/\xAD\x84\x82a3\xE6V[a/\xB7\x90\x87a88V[\x95P\x84`\x01\x01\x94PPPPPa/4V[g\r\xE0\xB6\xB3\xA7d\0\0\x88\x12a/\xE5Wa/\xE0\x82a8`V[a(\xD3V[P\x96\x95PPPPPPV[`\0a&\x1D\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a3\xF7V[`\0a&\x1Da0\x14\x84\x84a)HV[a2\x0BV[`\0\x81`\0\x03a02WPg\r\xE0\xB6\xB3\xA7d\0\0\x91\x90PV[gV\x98\xEE\xF0fp\0\0\x82\x12a0IWP`\0\x91\x90PV[a0ZgV\x98\xEE\xF0fp\0\0a8`V[\x82\x13a0oWPg\x1B\xC1mgN\xC8\0\0\x91\x90PV[`\0a0z\x83a4\x16V[\x90P`\0a0\xB3g\r\xE0\xB6\xB3\xA7d\0\0a0\x9C\x84g\x1B\xC1mgN\xC8\0\0a&\x08V[a0\xAE\x90g\r\xE0\xB6\xB3\xA7d\0\0a88V[a3\xE6V[\x90P`\0\x80\x82a1\x0F\x81a0\xFC\x81a0\xEA\x81a0\xD7\x81g\x02_\x0F\xE1\x05\xA3\x14\0a/\xF0V[a.\xF2\x90g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19a88V[a.\xF2\x90g\x14\xA8EL\x19\xE1\xAC\0a88V[a.\xF2\x90g\x0F\xC1\x0E\x01W\x82w\xFF\x19a88V[a1!\x90g\x03\xDE\xBD\x08;\x8C|\0a88V[\x91P\x83\x90Pa1\x89\x81a1w\x81a1e\x81a1S\x81a1@\x81\x8Ba/\xF0V[a.\xF2\x90g\x02\x95\xD4\0\xEA2W\xFF\x19a88V[a.\xF2\x90g\x01W\xD8\xB2\xEC\xC7\x08\0a88V[a.\xF2\x90g\x051\n\xA7\xD5!0\0a88V[a.\xF2\x90g\r\xE0\xCC=\x15a\0\0a88V[\x91P\x81g\x11\x90\0\xAB\x10\x0F\xFC\0a1\x9F\x87\x88a/\xF0V[a1\xAB\x90`\0\x19a7oV[a1\xB5\x91\x90a7\xCDV[a1\xBF\x91\x90a88V[\x92PP`\0a1\xCD\x83a&\xC3V[\x90P`\0a1\xDB\x85\x83a/\xF0V[\x90P`\0\x88\x12a1\xEBW\x80a(\xD3V[a(\xD3\x81g\x1B\xC1mgN\xC8\0\0a7\xCDV[`\0\x80a&\x95\x83`\x02a7oV[`\0\x80\x82\x13a2HW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x08MV[`\0``a2U\x84a4QV[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[`\0a&\x1D\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x05\x85\x14\x17\x16a4\x0FW`\0\x80\xFD[\x05\x92\x91PPV[`\0`\x01`\xFF\x1B\x82\x03a4<W`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x12\x15a4MWP\x19`\x01\x01\x90V[P\x90V[`\0\x80\x82\x11a4\x8EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x08MV[P`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x82\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`\0` \x82\x84\x03\x12\x15a5^Wa5^a4\xF9V[P5\x91\x90PV[\x80\x15\x15\x81\x14a5sW`\0\x80\xFD[PV[`\0\x80`\0``\x84\x86\x03\x12\x15a5\x8EWa5\x8Ea4\xF9V[\x835a5\x99\x81a5eV[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a5\xC4Wa5\xC4a4\xF9V[PP\x805\x92` \x90\x91\x015\x91PV[`\0` \x82\x84\x03\x12\x15a5\xE8Wa5\xE8a4\xF9V[\x815`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a5\xFFW`\0\x80\xFD[\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a6\x1CWa6\x1Ca4\xF9V[\x825a6'\x81a5eV[\x94` \x93\x90\x93\x015\x93PPPV[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a6bW\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a6FV[P`\0`@\x82\x86\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x92PPP\x92\x91PPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a& Wa& a6\x83V[\x81\x81\x03\x81\x81\x11\x15a& Wa& a6\x83V[`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x81R\x91\x90\x92\x16` \x82\x01R`@\x81\x01\x91\x90\x91R``\x01\x90V[`\0` \x82\x84\x03\x12\x15a6\xF8Wa6\xF8a4\xF9V[\x81Qa5\xFF\x81a5eV[` \x80\x82R`\x11\x90\x82\x01Rp\x15\\\x19\x18]\x19H\x19[\x99\x08\x1C\x18\\\xDC\xD9Y`z\x1B`@\x82\x01R``\x01\x90V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a7SWa7Sa7.V[P\x04\x90V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a& Wa& a6\x83V[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15a7\x8BWa7\x8Ba6\x83V[\x81\x81\x05\x83\x14\x82\x15\x17a& Wa& a6\x83V[`\0\x82a7\xAEWa7\xAEa7.V[`\x01`\xFF\x1B\x82\x14`\0\x19\x84\x14\x16\x15a7\xC8Wa7\xC8a6\x83V[P\x05\x90V[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a7\xEDWa7\xEDa6\x83V[P\x92\x91PPV[`\0`\xC0\x82\x01\x90P\x82Q\x82R` \x83\x01Q` \x83\x01R`@\x83\x01Q`@\x83\x01R``\x83\x01Q``\x83\x01R`\x80\x83\x01Q`\x80\x83\x01R`\xA0\x83\x01Q`\xA0\x83\x01R\x92\x91PPV[\x80\x82\x01\x82\x81\x12`\0\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a8XWa8Xa6\x83V[PP\x92\x91PPV[`\0`\x01`\xFF\x1B\x82\x01a8uWa8ua6\x83V[P`\0\x03\x90V[`\0`\xC0\x82\x84\x03\x12\x15a8\x91Wa8\x91a4\xF9V[`@Q`\xC0\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a8\xC2WcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x80`@RP\x82Q\x81R` \x83\x01Q` \x82\x01R`@\x83\x01Q`@\x82\x01R``\x83\x01Q``\x82\x01R`\x80\x83\x01Q`\x80\x82\x01R`\xA0\x83\x01Q`\xA0\x82\x01R\x80\x91PP\x92\x91PPV\xFETarget contract does not contains\xEBa\xB7\x83\x9B\xB3\xC27\xA0\xB3\xAE\xAB\x18iQ\0\xBCQ.q9\xF0\xE3@e]\xD7\x02\xDA*\xA1";
    /// The bytecode of the contract.
    pub static RMM_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x02\x7FW`\x005`\xE0\x1C\x80c\x8DR\xA1\xFC\x11a\x01}W\x80c\xC1e&\x12\x11a\x01\x05W\x80c\xDCv\xFA\xBC\x11a\0\xC9W\x80c\xDCv\xFA\xBC\x14a\x05#W\x80c\xF3\xA8\xEF\xE3\x14a\x05+W\x80c\xF9\xA1\xC8Z\x14a\x053W\x80c\xFA\xDF\xA6[\x14a\x05FW\x80c\xFE\xD3\xDF\xDA\x14a\x05OWa\x02\x7FV[\x80c\xC1e&\x12\x14a\x04\xEFW\x80c\xC5)\x87\xCF\x14a\x04\xF8W\x80c\xCC\xD1\xE4\xBE\x14a\x05\0W\x80c\xCF\xC4\xAFU\x14a\x05\x13W\x80c\xD4\xCA\xDFh\x14a\x05\x1BWa\x02\x7FV[\x80c\xAF\xDF1\xCD\x11a\x01LW\x80c\xAF\xDF1\xCD\x14a\x04\xBCW\x80c\xB7\xD1\x9F\xC4\x14a\x04\xC4W\x80c\xBB\x04\x98\xDE\x14a\x04\xD7W\x80c\xBC\xC1}\xC7\x14a\x04\xDFW\x80c\xC0\xFF\x1A\x15\x14a\x04\xE7Wa\x02\x7FV[\x80c\x8DR\xA1\xFC\x14a\x04pW\x80c\x99\x87\xFEd\x14a\x04\x83W\x80c\x9C\x9D\xA9\xEA\x14a\x04\x96W\x80c\xA5\x9C\x18o\x14a\x04\xA9Wa\x02\x7FV[\x80cAf\xD3H\x11a\x02\x0BW\x80cvp\x166\x11a\x01\xCFW\x80cvp\x166\x14a\x04\x0FW\x80c}iS~\x14a\x04\"W\x80c\x7F\x0EL\x8C\x14a\x045W\x80c\x82\xBE\xAD\xED\x14a\x04HW\x80c\x87kU\xF1\x14a\x04[Wa\x02\x7FV[\x80cAf\xD3H\x14a\x03\xC3W\x80cT\xCF*\xEB\x14a\x03\xD6W\x80cU\x9D\x16\x02\x14a\x03\xDFW\x80c^aZk\x14a\x03\xE7W\x80cp\xA0\x821\x14a\x03\xEFWa\x02\x7FV[\x80c\x15w\x0F\x92\x11a\x02RW\x80c\x15w\x0F\x92\x14a\x03^W\x80c\x16\xDC\x16[\x14a\x03gW\x80c4\xE1\x99\x07\x14a\x03\x92W\x80c69\xAA2\x14a\x03\xA7W\x80c>\x1E3\x92\x14a\x03\xBAWa\x02\x7FV[\x80c\x02\xC2\xE5]\x14a\x02\xE4W\x80c\x04\xAF\xA8\"\x14a\x03\x11W\x80c\x08\xEA\xBD\xDA\x14a\x03?W\x80c\t\x10\xA5\x10\x14a\x03VW[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x02\xF7a\x02\xF26`\x04a5IV[a\x05WV[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01[`@Q\x80\x91\x03\x90\xF3[a\x03$a\x03\x1F6`\x04a5vV[a\x07\xFAV[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x03\x08V[a\x03H`\x03T\x81V[`@Q\x90\x81R` \x01a\x03\x08V[`\x05Ta\x03HV[a\x03H`\x05T\x81V[`\0Ta\x03z\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x03\x08V[a\x03\xA5a\x03\xA06`\x04a5IV[a\nkV[\0[a\x03$a\x03\xB56`\x04a5\xAEV[a\n\xBFV[a\x03H`\rT\x81V[a\x03Ha\x03\xD16`\x04a5IV[a\n\xDDV[a\x03H`\x02T\x81V[`\x03Ta\x03HV[a\x03$a\x0B\x0FV[a\x03Ha\x03\xFD6`\x04a5\xD3V[`\x06` R`\0\x90\x81R`@\x90 T\x81V[a\x03$a\x04\x1D6`\x04a6\x06V[a\x0B7V[a\x02\xF7a\x0406`\x04a5IV[a\x0E\x9CV[a\x03\xA5a\x04C6`\x04a5\xAEV[a\x11\x0EV[a\x03Ha\x04V6`\x04a5vV[a\x11\xD3V[a\x04ca\x11\xE8V[`@Qa\x03\x08\x91\x90a65V[a\x03\xA5a\x04~6`\x04a5\xAEV[a\x120V[a\x03\xA5a\x04\x916`\x04a5\xAEV[a\x12\xBCV[a\x02\xF7a\x04\xA46`\x04a5IV[a\x13HV[a\x02\xF7a\x04\xB76`\x04a5\xAEV[a\x15\xCFV[a\x03Ha\x17\xABV[`\x01Ta\x03z\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x03Ha\x18\x1BV[a\x03\xA5a\x18RV[a\x03Ha\x18\x9FV[a\x03H`\x08T\x81V[a\x03Ha\x18\xBAV[a\x02\xF7a\x05\x0E6`\x04a5IV[a\x19%V[a\x03Ha\x1B\x97V[`\x02Ta\x03HV[a\x03Ha\x1C\x02V[a\x03Ha\x1C*V[a\x02\xF7a\x05A6`\x04a5\xAEV[a\x1CZV[a\x03H`\x04T\x81V[`\x04Ta\x03HV[`\0\x80`\0\x80`\0a\x05ga\x0B\x0FV[\x92P\x92P\x92P`\0a\x05\x80`\x03T`\x05T\x86\x86\x86a\x1D\x97V[\x90P`\0a\x05\x9E\x88`\x03Ta\x05\x95\x91\x90a6\x99V[\x83\x87\x87\x87a\x1E\x1BV[\x90P`\0a\x05\xAF\x82\x84\x88\x88\x88a\x1E[V[\x90P`\0`\x04T\x82a\x05\xC1\x91\x90a6\xACV[\x90P`\0`\x05T\x84a\x05\xD3\x91\x90a6\xACV[\x90P\x83`\x05\x81\x90UP\x8A`\x03`\0\x82\x82Ta\x05\xEE\x91\x90a6\x99V[\x92PP\x81\x90UP\x81`\x04`\0\x82\x82Ta\x06\x07\x91\x90a6\x99V[\x92PP\x81\x90UP`\0\x80T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c#\xB8r\xDD30\x8E`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x06Q\x93\x92\x91\x90a6\xBFV[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x06\xA9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a9\x08\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x06\xBDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xE1\x91\x90a6\xE3V[P`\x01T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\x07\x16\x903\x900\x90\x87\x90`\x04\x01a6\xBFV[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x07nW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a9\x08\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x07\x82W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\xA6\x91\x90a6\xE3V[P`@\x80Q\x85\x81R` \x81\x01\x8D\x90R\x90\x81\x01\x83\x90R3\x90\x7F\xBE\xB3\x88W\x86\xD67\xA4t\xCB\xC2\x87\xC0\xA4E\x87#\x163\xA0w\xF0\xBD05MZK\x18\x99o\xCE\x90``\x01[`@Q\x80\x91\x03\x90\xA2\x9A\x90\x99P\x97PPPPPPPPV[`\0\x80`\0`\x05T`\0\x14a\x08VW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FPool already initialized\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x80`\0a\x08ca\x0B\x0FV[\x92P\x92P\x92P\x88\x15a\x08\x95W\x87\x95Pa\x08\x7F\x86\x88\x85\x85\x85a\x1E\x1BV[\x93Pa\x08\x8E\x84\x88\x85\x85\x85a\x1E[V[\x94Pa\x08\xB7V[\x87\x94Pa\x08\xA5\x85\x88\x85\x85\x85a\x1E}V[\x93Pa\x08\xB4\x84\x88\x85\x85\x85a\x1E\x93V[\x95P[`\x05\x84\x90U`\x03\x86\x90U`\x04\x85\x81U3`\0\x81\x81R`\x06` R`@\x80\x82 \x88\x90U\x90T\x90Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x92c#\xB8r\xDD\x92a\t\t\x92\x90\x910\x91\x8C\x91\x01a6\xBFV[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\taW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a9\x08\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\tuW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\x99\x91\x90a6\xE3V[P`\x01T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\t\xCE\x903\x900\x90\x8A\x90`\x04\x01a6\xBFV[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\n&W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a9\x08\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\n:W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n^\x91\x90a6\xE3V[PPPP\x93P\x93P\x93\x90PV[g\r\xE0\xB6\xB3\xA7d\0\0\x81\x10a\n\xBAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rt\t\xCC\xAE\xE4\x0En\xEC.\x04\x0C\xCC\xAC\xA4\x0E\x8D\xED\xE4\r\r,\xED`[\x1B`D\x82\x01R`d\x01a\x08MV[`\x02UV[`\0\x80`\0a\n\xD0`\x01\x86\x86a\x07\xFAV[\x92P\x92P\x92P\x92P\x92P\x92V[`\0\x80`\0\x80a\n\xEBa\x0B\x0FV[\x92P\x92P\x92Pa\x0B\x06`\x03T`\x04T`\x05T\x86\x86\x86\x8Ba\x1E\xBDV[\x95\x94PPPPPV[`\0\x80`\0a\x0B\x1Ca\x18\xBAV[a\x0B$a\x17\xABV[a\x0B,a\x1B\x97V[\x92P\x92P\x92P\x90\x91\x92V[`\0\x80`\0\x80`\x05T\x11a\x0B\x84W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rs\x14\x1B\xDB\xDB\x08\x1B\x9B\xDD\x08\x1A[\x9A]\x1AX[\x1A^\x99Y`b\x1B`D\x82\x01R`d\x01a\x08MV[`\0\x80`\0a\x0B\x91a\x0B\x0FV[\x92P\x92P\x92P`\0a\x0B\xAA`\x03T`\x05T\x86\x86\x86a\x1D\x97V[\x90P\x88\x15a\x0C\x02W\x87\x96P`\0a\x0B\xC8\x88`\x03Ta\x05\x95\x91\x90a6\x99V[\x90P`\0a\x0B\xD9\x82\x84\x88\x88\x88a\x1E[V[\x90P`\x04T\x81a\x0B\xE9\x91\x90a6\xACV[\x97P`\x05T\x82a\x0B\xF9\x91\x90a6\xACV[\x96PPPa\x0CWV[\x87\x95P`\0a\x0C!\x87`\x04Ta\x0C\x18\x91\x90a6\x99V[\x83\x87\x87\x87a\x1E}V[\x90P`\0a\x0C2\x82\x84\x88\x88\x88a\x1E\x93V[\x90P`\x03T\x81a\x0CB\x91\x90a6\xACV[\x98P`\x05T\x82a\x0CR\x91\x90a6\xACV[\x96PPP[\x84`\x05`\0\x82\x82Ta\x0Ci\x91\x90a6\x99V[\x92PP\x81\x90UP\x86`\x03`\0\x82\x82Ta\x0C\x82\x91\x90a6\x99V[\x92PP\x81\x90UP\x85`\x04`\0\x82\x82Ta\x0C\x9B\x91\x90a6\x99V[\x90\x91UPP3`\0\x90\x81R`\x06` R`@\x81 \x80T\x87\x92\x90a\x0C\xBF\x90\x84\x90a6\x99V[\x90\x91UPP`\0T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\x0C\xF8\x903\x900\x90\x8C\x90`\x04\x01a6\xBFV[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\rPW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a9\x08\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\rdW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\x88\x91\x90a6\xE3V[P`\x01T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\r\xBD\x903\x900\x90\x8B\x90`\x04\x01a6\xBFV[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0E\x15W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a9\x08\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x0E)W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0EM\x91\x90a6\xE3V[P`@\x80Q\x86\x81R` \x81\x01\x89\x90R\x90\x81\x01\x87\x90R3\x90\x7F\xBE\xB3\x88W\x86\xD67\xA4t\xCB\xC2\x87\xC0\xA4E\x87#\x163\xA0w\xF0\xBD05MZK\x18\x99o\xCE\x90``\x01`@Q\x80\x91\x03\x90\xA2PPPP\x92P\x92P\x92V[`\0\x80`\0\x80`\0a\x0E\xACa\x0B\x0FV[\x92P\x92P\x92P`\0a\x0E\xC5`\x03T`\x05T\x86\x86\x86a\x1D\x97V[\x90P`\0a\x0E\xDA\x88`\x03Ta\x0C\x18\x91\x90a6\xACV[\x90P`\0a\x0E\xEB\x82\x84\x88\x88\x88a\x1E\x93V[\x90P`\0\x81`\x03Ta\x0E\xFD\x91\x90a6\xACV[\x90P`\0\x83`\x05Ta\x0F\x0F\x91\x90a6\xACV[\x90P\x83`\x05\x81\x90UP\x81`\x03`\0\x82\x82Ta\x0F*\x91\x90a6\xACV[\x92PP\x81\x90UP\x8A`\x04`\0\x82\x82Ta\x0FC\x91\x90a6\xACV[\x90\x91UPP`\0T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0F\xD2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a9\x08\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x0F\xE6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\n\x91\x90a6\xE3V[P`\x01T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x8D\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x10\x95W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a9\x08\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x10\xA9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\xCD\x91\x90a6\xE3V[P`@\x80Q\x85\x81R` \x81\x01\x84\x90R\x90\x81\x01\x8C\x90R3\x90\x7FY\xC3\xA0\xB6\x0Cj\xB7\xDE\xB6.\x14@\xC9\xE7$A\xDBm\xB7\xDF\xE5\x14\xDB\xA8\xCB\x18\xE6\x0C\r\x89n\xFA\x90``\x01a\x07\xE3V[B\x81\x11a\x11-W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08M\x90a7\x03V[a\x115a\x1E\xF9V[`\0\x82`\x11T\x11a\x11RW`\x11Ta\x11M\x90\x84a6\xACV[a\x11`V[\x82`\x11Ta\x11`\x91\x90a6\xACV[\x90Pa\x11lB\x83a6\xACV[a\x11v\x90\x82a7DV[`\x14U`\x12\x83\x90U`\x15\x82\x90U`\0\x80Q` a9(\x839\x81Q\x91Ra\x11\x9Aa\x17\xABV[a\x11\xA2a\x18\xBAV[a\x11\xAAa\x1B\x97V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01RB``\x82\x01R`\x80\x01`@Q\x80\x91\x03\x90\xA1PPPV[`\0a\x11\xE0\x84\x84\x84a\x1F\nV[\x94\x93PPPPV[``a\x11\xF2a\x17\xABV[a\x11\xFAa\x18\xBAV[a\x12\x02a\x1B\x97V[`@\x80Q` \x81\x01\x94\x90\x94R\x83\x01\x91\x90\x91R``\x82\x01R`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[B\x81\x11a\x12OW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08M\x90a7\x03V[a\x12Wa$\x8DV[`\0\x82`\x0CT\x11a\x12tW`\x0CTa\x12o\x90\x84a6\xACV[a\x12\x82V[\x82`\x0CTa\x12\x82\x91\x90a6\xACV[\x90Pa\x12\x8EB\x83a6\xACV[a\x12\x98\x90\x82a7DV[`\x0FU`\r\x83\x90U`\x10\x82\x90U`\0\x80Q` a9(\x839\x81Q\x91Ra\x11\x9Aa\x17\xABV[B\x81\x11a\x12\xDBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08M\x90a7\x03V[a\x12\xE3a$\x9EV[`\0\x82`\x07T\x11a\x13\0W`\x07Ta\x12\xFB\x90\x84a6\xACV[a\x13\x0EV[\x82`\x07Ta\x13\x0E\x91\x90a6\xACV[\x90Pa\x13\x1AB\x83a6\xACV[a\x13$\x90\x82a7DV[`\nU`\x08\x83\x90U`\x0B\x82\x90U`\0\x80Q` a9(\x839\x81Q\x91Ra\x11\x9Aa\x17\xABV[`\0\x80`\0\x80`\0a\x13Xa\x0B\x0FV[\x92P\x92P\x92P`\0a\x13q`\x03T`\x05T\x86\x86\x86a\x1D\x97V[\x90P`\0a\x13\x86\x88`\x04Ta\x0C\x18\x91\x90a6\x99V[\x90P`\0a\x13\x97\x82\x84\x88\x88\x88a\x1E\x93V[\x90P`\0`\x03T\x82a\x13\xA9\x91\x90a6\xACV[\x90P`\0`\x05T\x84a\x13\xBB\x91\x90a6\xACV[\x90P\x83`\x05\x81\x90UP\x81`\x03`\0\x82\x82Ta\x13\xD6\x91\x90a6\x99V[\x92PP\x81\x90UP\x8A`\x04`\0\x82\x82Ta\x13\xEF\x91\x90a6\x99V[\x90\x91UPP`\0T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\x14(\x903\x900\x90\x87\x90`\x04\x01a6\xBFV[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x14\x80W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a9\x08\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x14\x94W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14\xB8\x91\x90a6\xE3V[P`\x01`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c#\xB8r\xDD30\x8E`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x14\xFE\x93\x92\x91\x90a6\xBFV[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x15VW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a9\x08\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x15jW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\x8E\x91\x90a6\xE3V[P`@\x80Q\x85\x81R` \x81\x01\x84\x90R\x90\x81\x01\x8C\x90R3\x90\x7F\xBE\xB3\x88W\x86\xD67\xA4t\xCB\xC2\x87\xC0\xA4E\x87#\x163\xA0w\xF0\xBD05MZK\x18\x99o\xCE\x90``\x01a\x07\xE3V[`\0\x80`\0\x80`\0a\x15\xDFa\x0B\x0FV[\x92P\x92P\x92P`\0a\x15\xF4\x88\x88\x86\x86\x86a\x1E}V[\x90P`\0a\x16\x05\x82\x89\x87\x87\x87a\x1E\x93V[`\x05\x83\x90U`\x03\x81\x90U`\x04\x8A\x81U`\0T`@Qc#\xB8r\xDD`\xE0\x1B\x81R\x92\x93P`\x01`\x01`\xA0\x1B\x03\x16\x91c#\xB8r\xDD\x91a\x16G\x913\x910\x91\x87\x91\x01a6\xBFV[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x16\x9FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a9\x08\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x16\xB3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\xD7\x91\x90a6\xE3V[P`\x01T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\x17\x0C\x903\x900\x90\x8E\x90`\x04\x01a6\xBFV[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x17dW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a9\x08\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x17xW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17\x9C\x91\x90a6\xE3V[P\x90\x98\x90\x97P\x95PPPPPPV[`\0`\x0BTB\x10a\x17\xBDWP`\x08T\x90V[`\x08T`\x07T\x11a\x17\xF4W`\nT`\tTa\x17\xD8\x90Ba6\xACV[a\x17\xE2\x91\x90a7XV[`\x07Ta\x17\xEF\x91\x90a6\x99V[\x90P\x90V[`\nT`\tTa\x18\x04\x90Ba6\xACV[a\x18\x0E\x91\x90a7XV[`\x07Ta\x17\xEF\x91\x90a6\xACV[`\0`\x04Tg\r\xE0\xB6\xB3\xA7d\0\0a\x181a\x1C\x02V[`\x03Ta\x18>\x91\x90a7XV[a\x18H\x91\x90a7DV[a\x17\xEF\x91\x90a6\x99V[`\0\x80Q` a9(\x839\x81Q\x91Ra\x18ia\x17\xABV[a\x18qa\x18\xBAV[a\x18ya\x1B\x97V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01RB``\x82\x01R`\x80\x01`@Q\x80\x91\x03\x90\xA1V[`\0a\x17\xEF`\x03T`\x05T`\x04Ta\x18\xB5a\x18\xBAV[a$\xAFV[`\0`\x10TB\x10a\x18\xCCWP`\rT\x90V[`\rT`\x0CT\x11a\x18\xFEW`\x0FT`\x0ETa\x18\xE7\x90Ba6\xACV[a\x18\xF1\x91\x90a7XV[`\x0CTa\x17\xEF\x91\x90a6\x99V[`\x0FT`\x0ETa\x19\x0E\x90Ba6\xACV[a\x19\x18\x91\x90a7XV[`\x0CTa\x17\xEF\x91\x90a6\xACV[`\0\x80`\0\x80`\0a\x195a\x0B\x0FV[\x92P\x92P\x92P`\0a\x19N`\x03T`\x05T\x86\x86\x86a\x1D\x97V[\x90P`\0a\x19c\x88`\x03Ta\x05\x95\x91\x90a6\xACV[\x90P`\0a\x19t\x82\x84\x88\x88\x88a\x1E[V[\x90P`\0\x81`\x04Ta\x19\x86\x91\x90a6\xACV[\x90P`\0\x83`\x05Ta\x19\x98\x91\x90a6\xACV[\x90P\x83`\x05\x81\x90UP\x8A`\x03`\0\x82\x82Ta\x19\xB3\x91\x90a6\xACV[\x92PP\x81\x90UP\x81`\x04`\0\x82\x82Ta\x19\xCC\x91\x90a6\xACV[\x90\x91UPP`\0T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x8D\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1A[W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a9\x08\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x1AoW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A\x93\x91\x90a6\xE3V[P`\x01T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1B\x1EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a9\x08\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x1B2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1BV\x91\x90a6\xE3V[P`@\x80Q\x85\x81R` \x81\x01\x8D\x90R\x90\x81\x01\x83\x90R3\x90\x7FY\xC3\xA0\xB6\x0Cj\xB7\xDE\xB6.\x14@\xC9\xE7$A\xDBm\xB7\xDF\xE5\x14\xDB\xA8\xCB\x18\xE6\x0C\r\x89n\xFA\x90``\x01a\x07\xE3V[`\0`\x15TB\x10a\x1B\xA9WP`\x12T\x90V[`\x12T`\x11T\x11a\x1B\xDBW`\x14T`\x13Ta\x1B\xC4\x90Ba6\xACV[a\x1B\xCE\x91\x90a7XV[`\x11Ta\x17\xEF\x91\x90a6\x99V[`\x14T`\x13Ta\x1B\xEB\x90Ba6\xACV[a\x1B\xF5\x91\x90a7XV[`\x11Ta\x17\xEF\x91\x90a6\xACV[`\0a\x17\xEF`\x03T`\x05Ta\x1C\x15a\x18\xBAV[a\x1C\x1Da\x17\xABV[a\x1C%a\x1B\x97V[a\x1D\x97V[`\0\x80`\0\x80a\x1C8a\x0B\x0FV[\x92P\x92P\x92Pa\x1CR`\x03T`\x04T`\x05T\x86\x86\x86a$\xDFV[\x93PPPP\x90V[`\0\x80`\0\x80`\0a\x1Cja\x0B\x0FV[\x92P\x92P\x92P`\0a\x1C\x7F\x88\x88\x86\x86\x86a\x1E\x1BV[\x90P`\0a\x1C\x90\x82\x89\x87\x87\x87a\x1E[V[`\x05\x83\x90U`\x03\x8A\x90U`\x04\x81\x81U`\0T`@Qc#\xB8r\xDD`\xE0\x1B\x81R\x92\x93P`\x01`\x01`\xA0\x1B\x03\x16\x91c#\xB8r\xDD\x91a\x1C\xD2\x913\x910\x91\x8F\x91\x01a6\xBFV[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1D*W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a9\x08\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x1D>W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1Db\x91\x90a6\xE3V[P`\x01T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\x17\x0C\x903\x900\x90\x86\x90`\x04\x01a6\xBFV[`\0\x80a\x1D\xA4\x84\x84a%\xB0V[\x90P`\0a\x1D\xB2\x85\x85a%\xD6V[\x90P`\0a\x1D\xC0\x89\x89a&\x08V[\x90Pa\x1E\x0E\x87a\x1E\t\x84g\r\xE0\xB6\xB3\xA7d\0\0\x87a\x1D\xE6a\x1D\xE1\x88\x84a6\xACV[a&&V[a\x1D\xF0\x91\x90a7oV[a\x1D\xFA\x91\x90a7\x9FV[a\x1E\x04\x91\x90a7\xCDV[a&\xC3V[a(lV[\x99\x98PPPPPPPPPV[`\0\x80a\x1E2a\x1E-\x87\x87\x87\x87a(\x81V[a(\xDFV[a\x1ED\x90g\r\xE0\xB6\xB3\xA7d\0\0a7\xCDV[\x90Pa\x1EP\x87\x82a)HV[\x97\x96PPPPPPPV[`\0\x80a\x1Ema\x1E-\x87\x87\x87\x87a)]V[\x90Pa\x1EP\x85a\x1E\t\x89\x84a(lV[`\0\x80a\x1ED\x85a\x1E\ta\x1E-\x89\x89\x89\x89a)]V[`\0\x80a\x1E\xA5a\x1E-\x87\x87\x87\x87a(\x81V[\x90Pa\x1EP\x87a\x1E\t\x83g\r\xE0\xB6\xB3\xA7d\0\0a7\xCDV[`\0\x80a\x1E\xCE\x89\x89\x89\x89\x89\x89a)\x93V[\x90Pa\x1E\x0E\x81`@Q` \x01a\x1E\xE4\x91\x90a7\xF4V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x84a)\xF1V[a\x1F\x01a\x1B\x97V[`\x11UB`\x13UV[`\0\x80`\0\x80a\x1F\x18a\x0B\x0FV[\x92P\x92P\x92P`\0a\x1F5`\x03T`\x04T`\x05T\x87\x87\x87\x8Da\x1E\xBDV[\x90P\x80`c\x19\x12\x80\x15a\x1FHWP`d\x81\x12[a\x1F\x94W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FSwap constant out of range\0\0\0\0\0\0`D\x82\x01R`d\x01a\x08MV[`\0a\x1F\xA5`\x03T\x89\x87\x87\x87a\x1D\x97V[\x90P\x88\x15a!\xE7W`\0g\r\xE0\xB6\xB3\xA7d\0\0`\x02Tg\r\xE0\xB6\xB3\xA7d\0\0a\x1F\xCE\x91\x90a6\xACV[a\x1F\xD8\x90\x8Aa7XV[a\x1F\xE2\x91\x90a7DV[\x90P`\0a\x1F\xF3\x82\x84\x89\x89\x89a\x1E\x1BV[\x90P`\x01a \x0B`\x03T`\x04T\x8C\x8E\x86\x8D\x8D\x8Da*dV[a \x15\x91\x90a7\xCDV[\x19\x97Pa \"\x81\x8Ba6\x99V[`\x05\x81\x90UP\x88`\x03`\0\x82\x82Ta :\x91\x90a6\x99V[\x92PP\x81\x90UP\x87`\x04`\0\x82\x82Ta S\x91\x90a6\xACV[\x90\x91UPP`\0T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a \x8C\x903\x900\x90\x8E\x90`\x04\x01a6\xBFV[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a \xE4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a9\x08\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a \xF8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a!\x1C\x91\x90a6\xE3V[P`\x01T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x8A\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a!\xA7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a9\x08\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a!\xBBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a!\xDF\x91\x90a6\xE3V[PPPa$\x1DV[`\0g\r\xE0\xB6\xB3\xA7d\0\0`\x02Tg\r\xE0\xB6\xB3\xA7d\0\0a\"\x08\x91\x90a6\xACV[a\"\x12\x90\x8Aa7XV[a\"\x1C\x91\x90a7DV[\x90P`\0a\"-\x82\x84\x89\x89\x89a\x1E}V[\x90P`\x01a\"E`\x03T`\x04T\x8C\x8E\x86\x8D\x8D\x8Da*\xE5V[a\"O\x91\x90a7\xCDV[\x19\x97Pa\"\\\x81\x8Ba6\x99V[`\x05\x81\x90UP\x88`\x04`\0\x82\x82Ta\"t\x91\x90a6\x99V[\x92PP\x81\x90UP\x87`\x03`\0\x82\x82Ta\"\x8D\x91\x90a6\xACV[\x90\x91UPP`\x01T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\"\xC6\x903\x900\x90\x8E\x90`\x04\x01a6\xBFV[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a#\x1EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a9\x08\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a#2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a#V\x91\x90a6\xE3V[P`\0T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x8A\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a#\xE1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a9\x08\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a#\xF5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a$\x19\x91\x90a6\xE3V[PPP[3`\x01`\x01`\xA0\x1B\x03\x16\x7F\x17fH\xF1\xF1\x1C\xDA(L\x12D\x90\x08k\xE4*\x92m\xDF\n\xE8\x87\xEB\xE7\xB1\xD6\xB37\xD8\x94'V\x8A\x89\x89a$\\`\x03T`\x05T\x8C\x8C\x8Ca\x1D\x97V[`@\x80Q\x94\x15\x15\x85R` \x85\x01\x93\x90\x93R\x91\x83\x01R``\x82\x01R`\x80\x01`@Q\x80\x91\x03\x90\xA2PPPPP\x93\x92PPPV[a$\x95a\x18\xBAV[`\x0CUB`\x0EUV[a$\xA6a\x17\xABV[`\x07UB`\tUV[`\0a$\xC8a$\xBE\x85\x84a+3V[a\x1D\xE1\x90\x85a7DV[a$\xD5a\x1D\xE1\x86\x88a7DV[a\x0B\x06\x91\x90a88V[`\0\x80`\0\x80a$\xF3\x8A\x8A\x8A\x8A\x8A\x8Aa)\x93V[\x90P`\0a%$\x82`@Q` \x01a%\x0B\x91\x90a7\xF4V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83`@\x01Qa)\xF1V[\x90P\x80`\0\x03a%:W\x88\x94PPPPPa%\xA6V[`\0\x81\x12\x15a%ZW\x88\x92Pa%S\x83`b`da+DV[\x93Pa%mV[\x88\x93Pa%j\x84`f`da+cV[\x92P[a%\x9F\x82`@Q` \x01a%\x81\x91\x90a7\xF4V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x85\x85`\x01a\x01\0a)\xF1a+\x91V[\x94PPPPP[\x96\x95PPPPPPV[`\0\x80a%\xBC\x83a,\xA2V[a%\xCA\x90c;\x9A\xCA\0a7XV[\x90Pa\x11\xE0\x84\x82a+3V[`\0\x80a%\xF4a%\xEE\x85g\x1B\xC1mgN\xC8\0\0a-FV[\x84a+3V[\x90Pa\x11\xE0g\x06\xF0[Y\xD3\xB2\0\0\x82a+3V[`\0a&\x1D\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a+DV[\x90P[\x92\x91PPV[`\0g\x06\xF0[Y\xD3\xB2\0\0\x82\x03a&?WP`\0\x91\x90PV[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x12a&gW`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a&\x88W`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a&\x95\x83`\x02a7oV[\x90P`\0a&\xA2\x82a-rV[\x90P`\0a&\xB8g\x13\xA0K\xBD\xFD\xC9\xBE\x88\x83a/\xF0V[\x90Pa\x0B\x06\x81a8`V[`\0h\x02H\xCE6\xA7\x0C\xB2k>\x19\x82\x13a&\xDEWP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a'%W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x01a\x08MV[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05`\x01`_\x1B\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1Dm\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1Dn\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[`\0a&\x1D\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a+cV[`\0\x80a(\x8E\x84\x84a%\xB0V[\x90P`\0a(\x9C\x87\x87a0\x05V[\x90P`\0a(\xAA\x86\x86a%\xD6V[\x90P\x82a(\xB7\x82\x84a88V[a(\xC9\x90g\r\xE0\xB6\xB3\xA7d\0\0a7oV[a(\xD3\x91\x90a7\x9FV[\x98\x97PPPPPPPPV[`\0\x80g\x13\xA0K\xBD\xFD\xC9\xBE\x88a(\xFDg\r\xE0\xB6\xB3\xA7d\0\0\x85a7oV[a)\x07\x91\x90a7\x9FV[\x90P`\0a)\x14\x82a8`V[\x90P`\0a)!\x82a0\x19V[\x90Pg\x1B\xC1mgN\xC8\0\0a)>g\r\xE0\xB6\xB3\xA7d\0\0\x83a7oV[a\x0B\x06\x91\x90a7\x9FV[`\0a&\x1D\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a+cV[`\0\x80a)j\x84\x84a%\xB0V[\x90P`\0a)x\x87\x87a0\x05V[\x90P`\0a)\x86\x86\x86a%\xD6V[\x90P\x82a(\xB7\x82\x84a7\xCDV[a)\xCC`@Q\x80`\xC0\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[\x95\x86R` \x86\x01\x94\x90\x94R`@\x85\x01\x92\x90\x92R``\x84\x01R`\x80\x83\x01R`\xA0\x82\x01R\x90V[`\0\x80\x83\x80` \x01\x90Q\x81\x01\x90a*\x08\x91\x90a8|V[\x90Pa*\x1C\x81`\x80\x01Q\x82`\xA0\x01Qa%\xB0V[a*?a*:\x83` \x01Qa*5\x87\x86``\x01Qa+3V[a&\x08V[a1\xFDV[a*Pa*:\x84`\0\x01Q\x87a&\x08V[a*Z\x91\x90a88V[a\x11\xE0\x91\x90a88V[`\0\x80a*q\x84\x84a%\xB0V[\x90P`\0a*\x88\x86a*\x83\x89\x8Ba6\x99V[a+3V[\x90P`\0a*\xBEa*\xABa\x1D\xE1\x8C\x8Fa*\xA1\x91\x90a6\x99V[a*5\x8C\x8Ea6\x99V[a*\xB4\x85a8`V[a\x1E-\x91\x90a7\xCDV[\x90P\x8Aa*\xCB\x83\x83a+3V[a*\xD5\x91\x90a7\xCDV[\x9C\x9BPPPPPPPPPPPPV[`\0\x80a*\xF2\x84\x84a%\xB0V[\x90P`\0a+\x04\x86a*\x83\x89\x8Ba6\x99V[\x90P`\0a+!a*\xABa\x1D\xE1a+\x1B\x8D\x8Fa6\x99V[\x85a&\x08V[\x90P\x8Ba*\xCBa+1\x8A\x8Ca6\x99V[\x83[`\0a&\x1D\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a+\\W`\0\x80\xFD[\x04\x92\x91PPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a+{W`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[`\0\x84\x86\x11\x15a+\xBEW`@Qc0\x82\xDF\xDB`\xE1\x1B\x81R`\x04\x81\x01\x87\x90R`$\x81\x01\x86\x90R`D\x01a\x08MV[`\0a+\xCE\x88\x88\x85c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a+\xE0\x89\x88\x86c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a+\xEE\x82\x84a7oV[\x13\x15a,\x17W`@Qc\x06\xF1\xBE]`\xE2\x1B\x81R`\x04\x81\x01\x83\x90R`$\x81\x01\x82\x90R`D\x01a\x08MV[`\0a,#\x89\x89a6\xACV[\x90P`\0[`\x02a,4\x8A\x8Ca6\x99V[a,>\x91\x90a7DV[\x94P`\0a,P\x8C\x87\x89c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a,^\x86\x83a7oV[\x13a,kW\x85\x99Pa,rV[\x85\x9AP\x80\x94P[a,|\x8B\x8Ba6\xACV[\x92PP`\x01\x01\x87\x82\x11\x80\x15a,\x90WP\x86\x81\x10[a,(WPPPP\x96\x95PPPPPPV[`\xB5\x81`\x01`\x88\x1B\x81\x10a,\xBBW`@\x91\x90\x91\x1B\x90`\x80\x1C[i\x01\0\0\0\0\0\0\0\0\0\x81\x10a,\xD7W` \x91\x90\x91\x1B\x90`@\x1C[e\x01\0\0\0\0\0\x81\x10a,\xEFW`\x10\x91\x90\x91\x1B\x90` \x1C[c\x01\0\0\0\x81\x10a-\x05W`\x08\x91\x90\x91\x1B\x90`\x10\x1C[b\x01\0\0\x01\x02`\x12\x1C\x80\x82\x04\x01`\x01\x90\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x90\x81\x90\x04\x81\x11\x90\x03\x90V[`\0a&\x1Dg\r\xE0\xB6\xB3\xA7d\0\0\x83a-^\x86a2\x0BV[a-h\x91\x90a7oV[a\x1E\x04\x91\x90a7\x9FV[`\0\x80\x82\x12\x80a-\x89WPg\x1B\xC1mgN\xC8\0\0\x82\x13[\x15a-\xA7W`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a-\xC8W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81g\x1B\xC1mgN\xC8\0\0\x03a-\xF0W`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x15a-\xFBW\x91\x90PV[`\0g\r\xE0\xB6\xB3\xA7d\0\0\x83\x12a.#Wa.\x1E\x83g\x1B\xC1mgN\xC8\0\0a7\xCDV[a.%V[\x82[\x90P`\0a.;\x82g\x1B\xC1mgN\xC8\0\0a3\xE6V[\x90P\x80`\0\x03a.^W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a.i\x82a2\x0BV[\x90P`\0c;\x9A\xCA\0a.\x94a.\x8Fa.\x89g\x1B\xC1mgN\xC8\0\0a8`V[\x85a/\xF0V[a,\xA2V[a.\x9E\x91\x90a7oV[\x90P`\0\x80a.\xB5\x83g\x03\xC1f\\z\xAB \0a/\xF0V[a.\xC7\x90g \x05\xFEO&\x8E\xA0\0a88V[\x90P`\0a.\xF7\x84a.\xE0\x86f\x9F2u$b\xA0\0a/\xF0V[a.\xF2\x90g\r\xC5R\x7Fd, \0a88V[a/\xF0V[a/\t\x90g\r\xE0\xB6\xB3\xA7d\0\0a88V[\x90Pa/-g\t\xD0(\xCCo _\xFF\x19\x85a/#\x85\x85a3\xE6V[a.\xF2\x91\x90a7\xCDV[\x92PPP`\0[`\x02\x81\x10\x15a/\xC8W`\0\x86a/I\x84a0\x19V[a/S\x91\x90a7\xCDV[\x90P`\0a/a\x84\x85a/\xF0V[a/j\x90a8`V[\x90P`\0a/w\x82a&\xC3V[\x90P`\0a/\x85\x86\x85a/\xF0V[a/\x97g\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x84a/\xF0V[a/\xA1\x91\x90a7\xCDV[\x90Pa/\xAD\x84\x82a3\xE6V[a/\xB7\x90\x87a88V[\x95P\x84`\x01\x01\x94PPPPPa/4V[g\r\xE0\xB6\xB3\xA7d\0\0\x88\x12a/\xE5Wa/\xE0\x82a8`V[a(\xD3V[P\x96\x95PPPPPPV[`\0a&\x1D\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a3\xF7V[`\0a&\x1Da0\x14\x84\x84a)HV[a2\x0BV[`\0\x81`\0\x03a02WPg\r\xE0\xB6\xB3\xA7d\0\0\x91\x90PV[gV\x98\xEE\xF0fp\0\0\x82\x12a0IWP`\0\x91\x90PV[a0ZgV\x98\xEE\xF0fp\0\0a8`V[\x82\x13a0oWPg\x1B\xC1mgN\xC8\0\0\x91\x90PV[`\0a0z\x83a4\x16V[\x90P`\0a0\xB3g\r\xE0\xB6\xB3\xA7d\0\0a0\x9C\x84g\x1B\xC1mgN\xC8\0\0a&\x08V[a0\xAE\x90g\r\xE0\xB6\xB3\xA7d\0\0a88V[a3\xE6V[\x90P`\0\x80\x82a1\x0F\x81a0\xFC\x81a0\xEA\x81a0\xD7\x81g\x02_\x0F\xE1\x05\xA3\x14\0a/\xF0V[a.\xF2\x90g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19a88V[a.\xF2\x90g\x14\xA8EL\x19\xE1\xAC\0a88V[a.\xF2\x90g\x0F\xC1\x0E\x01W\x82w\xFF\x19a88V[a1!\x90g\x03\xDE\xBD\x08;\x8C|\0a88V[\x91P\x83\x90Pa1\x89\x81a1w\x81a1e\x81a1S\x81a1@\x81\x8Ba/\xF0V[a.\xF2\x90g\x02\x95\xD4\0\xEA2W\xFF\x19a88V[a.\xF2\x90g\x01W\xD8\xB2\xEC\xC7\x08\0a88V[a.\xF2\x90g\x051\n\xA7\xD5!0\0a88V[a.\xF2\x90g\r\xE0\xCC=\x15a\0\0a88V[\x91P\x81g\x11\x90\0\xAB\x10\x0F\xFC\0a1\x9F\x87\x88a/\xF0V[a1\xAB\x90`\0\x19a7oV[a1\xB5\x91\x90a7\xCDV[a1\xBF\x91\x90a88V[\x92PP`\0a1\xCD\x83a&\xC3V[\x90P`\0a1\xDB\x85\x83a/\xF0V[\x90P`\0\x88\x12a1\xEBW\x80a(\xD3V[a(\xD3\x81g\x1B\xC1mgN\xC8\0\0a7\xCDV[`\0\x80a&\x95\x83`\x02a7oV[`\0\x80\x82\x13a2HW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x08MV[`\0``a2U\x84a4QV[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[`\0a&\x1D\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x05\x85\x14\x17\x16a4\x0FW`\0\x80\xFD[\x05\x92\x91PPV[`\0`\x01`\xFF\x1B\x82\x03a4<W`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x12\x15a4MWP\x19`\x01\x01\x90V[P\x90V[`\0\x80\x82\x11a4\x8EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x08MV[P`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x82\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`\0` \x82\x84\x03\x12\x15a5^Wa5^a4\xF9V[P5\x91\x90PV[\x80\x15\x15\x81\x14a5sW`\0\x80\xFD[PV[`\0\x80`\0``\x84\x86\x03\x12\x15a5\x8EWa5\x8Ea4\xF9V[\x835a5\x99\x81a5eV[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a5\xC4Wa5\xC4a4\xF9V[PP\x805\x92` \x90\x91\x015\x91PV[`\0` \x82\x84\x03\x12\x15a5\xE8Wa5\xE8a4\xF9V[\x815`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a5\xFFW`\0\x80\xFD[\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a6\x1CWa6\x1Ca4\xF9V[\x825a6'\x81a5eV[\x94` \x93\x90\x93\x015\x93PPPV[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a6bW\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a6FV[P`\0`@\x82\x86\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x92PPP\x92\x91PPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a& Wa& a6\x83V[\x81\x81\x03\x81\x81\x11\x15a& Wa& a6\x83V[`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x81R\x91\x90\x92\x16` \x82\x01R`@\x81\x01\x91\x90\x91R``\x01\x90V[`\0` \x82\x84\x03\x12\x15a6\xF8Wa6\xF8a4\xF9V[\x81Qa5\xFF\x81a5eV[` \x80\x82R`\x11\x90\x82\x01Rp\x15\\\x19\x18]\x19H\x19[\x99\x08\x1C\x18\\\xDC\xD9Y`z\x1B`@\x82\x01R``\x01\x90V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a7SWa7Sa7.V[P\x04\x90V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a& Wa& a6\x83V[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15a7\x8BWa7\x8Ba6\x83V[\x81\x81\x05\x83\x14\x82\x15\x17a& Wa& a6\x83V[`\0\x82a7\xAEWa7\xAEa7.V[`\x01`\xFF\x1B\x82\x14`\0\x19\x84\x14\x16\x15a7\xC8Wa7\xC8a6\x83V[P\x05\x90V[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a7\xEDWa7\xEDa6\x83V[P\x92\x91PPV[`\0`\xC0\x82\x01\x90P\x82Q\x82R` \x83\x01Q` \x83\x01R`@\x83\x01Q`@\x83\x01R``\x83\x01Q``\x83\x01R`\x80\x83\x01Q`\x80\x83\x01R`\xA0\x83\x01Q`\xA0\x83\x01R\x92\x91PPV[\x80\x82\x01\x82\x81\x12`\0\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a8XWa8Xa6\x83V[PP\x92\x91PPV[`\0`\x01`\xFF\x1B\x82\x01a8uWa8ua6\x83V[P`\0\x03\x90V[`\0`\xC0\x82\x84\x03\x12\x15a8\x91Wa8\x91a4\xF9V[`@Q`\xC0\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a8\xC2WcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x80`@RP\x82Q\x81R` \x83\x01Q` \x82\x01R`@\x83\x01Q`@\x82\x01R``\x83\x01Q``\x82\x01R`\x80\x83\x01Q`\x80\x82\x01R`\xA0\x83\x01Q`\xA0\x82\x01R\x80\x91PP\x92\x91PPV\xFETarget contract does not contains\xEBa\xB7\x83\x9B\xB3\xC27\xA0\xB3\xAE\xAB\x18iQ\0\xBCQ.q9\xF0\xE3@e]\xD7\x02\xDA*\xA1";
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
        ///Calls the contract's `swapAmountIn` (0x82beaded) function
        pub fn swap_amount_in(
            &self,
            swap_direction: bool,
            next_liquidity: ::ethers::core::types::U256,
            amount_in: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [130, 190, 173, 237],
                    (swap_direction, next_liquidity, amount_in),
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
    pub enum RMMErrors {
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
    impl ::ethers::core::abi::AbiEncode for RMMErrors {
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
    impl ::ethers::contract::ContractRevert for RMMErrors {
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
    impl ::core::fmt::Display for RMMErrors {
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
    impl ::core::convert::From<::std::string::String> for RMMErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
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
    impl ::core::convert::From<OutOfBounds> for RMMErrors {
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
    ///Container type for all input parameters for the `swapAmountIn` function with signature `swapAmountIn(bool,uint256,uint256)` and selector `0x82beaded`
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
    #[ethcall(name = "swapAmountIn", abi = "swapAmountIn(bool,uint256,uint256)")]
    pub struct SwapAmountInCall {
        pub swap_direction: bool,
        pub next_liquidity: ::ethers::core::types::U256,
        pub amount_in: ::ethers::core::types::U256,
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
        GetInvariant(GetInvariantCall),
        GetLiquidity(GetLiquidityCall),
        GetNextLiquidity(GetNextLiquidityCall),
        GetParams(GetParamsCall),
        GetPortfolioValue(GetPortfolioValueCall),
        GetReserveX(GetReserveXCall),
        GetReserveY(GetReserveYCall),
        GetSpotPrice(GetSpotPriceCall),
        GetStrategyData(GetStrategyDataCall),
        GetSwapConstantGivenLiquidity(GetSwapConstantGivenLiquidityCall),
        GetSwapFee(GetSwapFeeCall),
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
                Self::GetStrategyData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetSwapConstantGivenLiquidity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetSwapFee(element) => {
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
                Self::GetInvariant(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetLiquidity(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetNextLiquidity(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetParams(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPortfolioValue(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetReserveX(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetReserveY(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSpotPrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetStrategyData(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSwapConstantGivenLiquidity(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetSwapFee(element) => ::core::fmt::Display::fmt(element, f),
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
    ///Container type for all return fields from the `swapAmountIn` function with signature `swapAmountIn(bool,uint256,uint256)` and selector `0x82beaded`
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
