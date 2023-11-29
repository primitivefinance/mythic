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
                    ::std::borrow::ToOwned::to_owned("checkSwapConstant"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("checkSwapConstant"),
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
                    ::std::borrow::ToOwned::to_owned("getNewLFromParameters"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getNewLFromParameters",
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
                    ::std::borrow::ToOwned::to_owned("getSwapUpperLower"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getSwapUpperLower"),
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
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15b\0\0^W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`@Qb\0;\xA18\x03\x80b\0;\xA1\x839\x81\x01`@\x81\x90Rb\0\0\x81\x91b\0\x01hV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x80\x89\x16`\x01`\x01`\xA0\x1B\x03\x19\x92\x83\x16\x17\x90\x92U`\x01\x80T\x92\x88\x16\x92\x90\x91\x16\x91\x90\x91\x17\x90U`\x08\x84\x90U`\x07\x84\x90UB`\x0B\x81\x90U`\t\x81\x90U`\r\x84\x90U`\x0C\x84\x90U`\x10\x81\x90U`\x0E\x81\x90U`\x12\x83\x90U`\x11\x83\x90U`\x15\x81\x90U`\x13Ug\r\xE0\xB6\xB3\xA7d\0\0\x81\x10b\0\x01<W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01Rp\nn\xEC.\x04\x0C\xCC\xAC\xA4\x0E\x8D\xED\xE4\r\r,\xED`{\x1B`D\x82\x01R`d\x01`@Q\x80\x91\x03\x90\xFD[`\x02UPb\0\x02\x10\x93PPPPV[\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x01cW`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15b\0\x01\xCDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[b\0\x01\xD8\x87b\0\x01KV[\x95Pb\0\x01\xE8` \x88\x01b\0\x01KV[\x94P`@\x87\x01Q\x93P``\x87\x01Q\x92P`\x80\x87\x01Q\x91P`\xA0\x87\x01Q\x90P\x92\x95P\x92\x95P\x92\x95V[a9\x81\x80b\0\x02 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x02\x7FW`\x005`\xE0\x1C\x80c\x8DR\xA1\xFC\x11a\x01}W\x80c\xC0\xFF\x1A\x15\x11a\x01\x05W\x80c\xDCv\xFA\xBC\x11a\0\xC9W\x80c\xDCv\xFA\xBC\x14a\x05\"W\x80c\xF2\xBFMT\x14a\x05*W\x80c\xF9\xA1\xC8Z\x14a\x052W\x80c\xFA\xDF\xA6[\x14a\x05EW\x80c\xFE\xD3\xDF\xDA\x14a\x05NWa\x02\x7FV[\x80c\xC0\xFF\x1A\x15\x14a\x04\xEFW\x80c\xC5)\x87\xCF\x14a\x04\xF7W\x80c\xCC\xD1\xE4\xBE\x14a\x04\xFFW\x80c\xCF\xC4\xAFU\x14a\x05\x12W\x80c\xD4\xCA\xDFh\x14a\x05\x1AWa\x02\x7FV[\x80c\xAF\xDF1\xCD\x11a\x01LW\x80c\xAF\xDF1\xCD\x14a\x04\xB1W\x80c\xB7\xD1\x9F\xC4\x14a\x04\xB9W\x80c\xB9\xD9Z\xCA\x14a\x04\xCCW\x80c\xBB\x04\x98\xDE\x14a\x04\xDFW\x80c\xBC\xC1}\xC7\x14a\x04\xE7Wa\x02\x7FV[\x80c\x8DR\xA1\xFC\x14a\x04eW\x80c\x99\x87\xFEd\x14a\x04xW\x80c\x9C\x9D\xA9\xEA\x14a\x04\x8BW\x80c\xA5\x9C\x18o\x14a\x04\x9EWa\x02\x7FV[\x80c69\xAA2\x11a\x02\x0BW\x80cp\xA0\x821\x11a\x01\xCFW\x80cp\xA0\x821\x14a\x03\xF7W\x80cvp\x166\x14a\x04\x17W\x80c}iS~\x14a\x04*W\x80c\x7F\x0EL\x8C\x14a\x04=W\x80c\x87kU\xF1\x14a\x04PWa\x02\x7FV[\x80c69\xAA2\x14a\x03\xC2W\x80c>\x1E3\x92\x14a\x03\xD5W\x80cT\xCF*\xEB\x14a\x03\xDEW\x80cU\x9D\x16\x02\x14a\x03\xE7W\x80c^aZk\x14a\x03\xEFWa\x02\x7FV[\x80c\t\x10\xA5\x10\x11a\x02RW\x80c\t\x10\xA5\x10\x14a\x03^W\x80c\x15w\x0F\x92\x14a\x03fW\x80c\x16\xDC\x16[\x14a\x03oW\x80c\x1F\xDA\xBC'\x14a\x03\x9AW\x80c4\xE1\x99\x07\x14a\x03\xADWa\x02\x7FV[\x80c\x02\xC2\xE5]\x14a\x02\xE4W\x80c\x04\xAF\xA8\"\x14a\x03\x11W\x80c\x06\x81\x83\x1A\x14a\x03?W\x80c\x08\xEA\xBD\xDA\x14a\x03GW[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x02\xF7a\x02\xF26`\x04a5\x82V[a\x05VV[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01[`@Q\x80\x91\x03\x90\xF3[a\x03$a\x03\x1F6`\x04a5\xAFV[a\x07\xF9V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x03\x08V[a\x02\xF7a\njV[a\x03P`\x03T\x81V[`@Q\x90\x81R` \x01a\x03\x08V[`\x05Ta\x03PV[a\x03P`\x05T\x81V[`\0Ta\x03\x82\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x03\x08V[a\x03Pa\x03\xA86`\x04a5\xE7V[a\n\x9FV[a\x03\xC0a\x03\xBB6`\x04a5\x82V[a\n\xB4V[\0[a\x03$a\x03\xD06`\x04a6\x16V[a\x0B\x08V[a\x03P`\rT\x81V[a\x03P`\x02T\x81V[`\x03Ta\x03PV[a\x03$a\x0B&V[a\x03Pa\x04\x056`\x04a6;V[`\x06` R`\0\x90\x81R`@\x90 T\x81V[a\x03$a\x04%6`\x04a5\xE7V[a\x0BNV[a\x02\xF7a\x0486`\x04a5\x82V[a\x0E\xB3V[a\x03\xC0a\x04K6`\x04a6\x16V[a\x11%V[a\x04Xa\x11\xEAV[`@Qa\x03\x08\x91\x90a6nV[a\x03\xC0a\x04s6`\x04a6\x16V[a\x122V[a\x03\xC0a\x04\x866`\x04a6\x16V[a\x12\xBEV[a\x02\xF7a\x04\x996`\x04a5\x82V[a\x13JV[a\x02\xF7a\x04\xAC6`\x04a6\x16V[a\x15\xD1V[a\x03Pa\x17\xADV[`\x01Ta\x03\x82\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x03Pa\x04\xDA6`\x04a5\x82V[a\x18\x1DV[a\x03Pa\x18vV[a\x03\xC0a\x18\xADV[a\x03Pa\x18\xFAV[a\x03Pa\x19\x15V[a\x02\xF7a\x05\r6`\x04a5\x82V[a\x19\x80V[a\x03Pa\x1B\xF2V[`\x02Ta\x03PV[a\x03Pa\x1C]V[a\x03Pa\x1C\x85V[a\x02\xF7a\x05@6`\x04a6\x16V[a\x1C\xB5V[a\x03P`\x04T\x81V[`\x04Ta\x03PV[`\0\x80`\0\x80`\0a\x05fa\x0B&V[\x92P\x92P\x92P`\0a\x05\x7F`\x03T`\x05T\x86\x86\x86a\x1D\xF2V[\x90P`\0a\x05\x9D\x88`\x03Ta\x05\x94\x91\x90a6\xD2V[\x83\x87\x87\x87a\x1EvV[\x90P`\0a\x05\xAE\x82\x84\x88\x88\x88a\x1E\xB6V[\x90P`\0`\x04T\x82a\x05\xC0\x91\x90a6\xE5V[\x90P`\0`\x05T\x84a\x05\xD2\x91\x90a6\xE5V[\x90P\x83`\x05\x81\x90UP\x8A`\x03`\0\x82\x82Ta\x05\xED\x91\x90a6\xD2V[\x92PP\x81\x90UP\x81`\x04`\0\x82\x82Ta\x06\x06\x91\x90a6\xD2V[\x92PP\x81\x90UP`\0\x80T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c#\xB8r\xDD30\x8E`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x06P\x93\x92\x91\x90a6\xF8V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x06\xA8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a9A\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x06\xBCW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xE0\x91\x90a7\x1CV[P`\x01T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\x07\x15\x903\x900\x90\x87\x90`\x04\x01a6\xF8V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x07mW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a9A\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x07\x81W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\xA5\x91\x90a7\x1CV[P`@\x80Q\x85\x81R` \x81\x01\x8D\x90R\x90\x81\x01\x83\x90R3\x90\x7F\xBE\xB3\x88W\x86\xD67\xA4t\xCB\xC2\x87\xC0\xA4E\x87#\x163\xA0w\xF0\xBD05MZK\x18\x99o\xCE\x90``\x01[`@Q\x80\x91\x03\x90\xA2\x9A\x90\x99P\x97PPPPPPPPV[`\0\x80`\0`\x05T`\0\x14a\x08UW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FPool already initialized\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x80`\0a\x08ba\x0B&V[\x92P\x92P\x92P\x88\x15a\x08\x94W\x87\x95Pa\x08~\x86\x88\x85\x85\x85a\x1EvV[\x93Pa\x08\x8D\x84\x88\x85\x85\x85a\x1E\xB6V[\x94Pa\x08\xB6V[\x87\x94Pa\x08\xA4\x85\x88\x85\x85\x85a\x1E\xD8V[\x93Pa\x08\xB3\x84\x88\x85\x85\x85a\x1E\xEEV[\x95P[`\x05\x84\x90U`\x03\x86\x90U`\x04\x85\x81U3`\0\x81\x81R`\x06` R`@\x80\x82 \x88\x90U\x90T\x90Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x92c#\xB8r\xDD\x92a\t\x08\x92\x90\x910\x91\x8C\x91\x01a6\xF8V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\t`W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a9A\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\ttW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\x98\x91\x90a7\x1CV[P`\x01T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\t\xCD\x903\x900\x90\x8A\x90`\x04\x01a6\xF8V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\n%W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a9A\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\n9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n]\x91\x90a7\x1CV[PPPP\x93P\x93P\x93\x90PV[`\0\x80`\0\x80`\0a\nza\x0B&V[\x92P\x92P\x92Pa\n\x94`\x03T`\x04T`\x05T\x86\x86\x86a\x1F\x18V[\x94P\x94PPPP\x90\x91V[`\0a\n\xAB\x83\x83a\x1F\xA8V[\x90P[\x92\x91PPV[g\r\xE0\xB6\xB3\xA7d\0\0\x81\x10a\x0B\x03W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rt\t\xCC\xAE\xE4\x0En\xEC.\x04\x0C\xCC\xAC\xA4\x0E\x8D\xED\xE4\r\r,\xED`[\x1B`D\x82\x01R`d\x01a\x08LV[`\x02UV[`\0\x80`\0a\x0B\x19`\x01\x86\x86a\x07\xF9V[\x92P\x92P\x92P\x92P\x92P\x92V[`\0\x80`\0a\x0B3a\x19\x15V[a\x0B;a\x17\xADV[a\x0BCa\x1B\xF2V[\x92P\x92P\x92P\x90\x91\x92V[`\0\x80`\0\x80`\x05T\x11a\x0B\x9BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rs\x14\x1B\xDB\xDB\x08\x1B\x9B\xDD\x08\x1A[\x9A]\x1AX[\x1A^\x99Y`b\x1B`D\x82\x01R`d\x01a\x08LV[`\0\x80`\0a\x0B\xA8a\x0B&V[\x92P\x92P\x92P`\0a\x0B\xC1`\x03T`\x05T\x86\x86\x86a\x1D\xF2V[\x90P\x88\x15a\x0C\x19W\x87\x96P`\0a\x0B\xDF\x88`\x03Ta\x05\x94\x91\x90a6\xD2V[\x90P`\0a\x0B\xF0\x82\x84\x88\x88\x88a\x1E\xB6V[\x90P`\x04T\x81a\x0C\0\x91\x90a6\xE5V[\x97P`\x05T\x82a\x0C\x10\x91\x90a6\xE5V[\x96PPPa\x0CnV[\x87\x95P`\0a\x0C8\x87`\x04Ta\x0C/\x91\x90a6\xD2V[\x83\x87\x87\x87a\x1E\xD8V[\x90P`\0a\x0CI\x82\x84\x88\x88\x88a\x1E\xEEV[\x90P`\x03T\x81a\x0CY\x91\x90a6\xE5V[\x98P`\x05T\x82a\x0Ci\x91\x90a6\xE5V[\x96PPP[\x84`\x05`\0\x82\x82Ta\x0C\x80\x91\x90a6\xD2V[\x92PP\x81\x90UP\x86`\x03`\0\x82\x82Ta\x0C\x99\x91\x90a6\xD2V[\x92PP\x81\x90UP\x85`\x04`\0\x82\x82Ta\x0C\xB2\x91\x90a6\xD2V[\x90\x91UPP3`\0\x90\x81R`\x06` R`@\x81 \x80T\x87\x92\x90a\x0C\xD6\x90\x84\x90a6\xD2V[\x90\x91UPP`\0T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\r\x0F\x903\x900\x90\x8C\x90`\x04\x01a6\xF8V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\rgW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a9A\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\r{W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\x9F\x91\x90a7\x1CV[P`\x01T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\r\xD4\x903\x900\x90\x8B\x90`\x04\x01a6\xF8V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0E,W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a9A\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x0E@W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Ed\x91\x90a7\x1CV[P`@\x80Q\x86\x81R` \x81\x01\x89\x90R\x90\x81\x01\x87\x90R3\x90\x7F\xBE\xB3\x88W\x86\xD67\xA4t\xCB\xC2\x87\xC0\xA4E\x87#\x163\xA0w\xF0\xBD05MZK\x18\x99o\xCE\x90``\x01`@Q\x80\x91\x03\x90\xA2PPPP\x92P\x92P\x92V[`\0\x80`\0\x80`\0a\x0E\xC3a\x0B&V[\x92P\x92P\x92P`\0a\x0E\xDC`\x03T`\x05T\x86\x86\x86a\x1D\xF2V[\x90P`\0a\x0E\xF1\x88`\x03Ta\x0C/\x91\x90a6\xE5V[\x90P`\0a\x0F\x02\x82\x84\x88\x88\x88a\x1E\xEEV[\x90P`\0\x81`\x03Ta\x0F\x14\x91\x90a6\xE5V[\x90P`\0\x83`\x05Ta\x0F&\x91\x90a6\xE5V[\x90P\x83`\x05\x81\x90UP\x81`\x03`\0\x82\x82Ta\x0FA\x91\x90a6\xE5V[\x92PP\x81\x90UP\x8A`\x04`\0\x82\x82Ta\x0FZ\x91\x90a6\xE5V[\x90\x91UPP`\0T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0F\xE9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a9A\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x0F\xFDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10!\x91\x90a7\x1CV[P`\x01T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x8D\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x10\xACW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a9A\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x10\xC0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\xE4\x91\x90a7\x1CV[P`@\x80Q\x85\x81R` \x81\x01\x84\x90R\x90\x81\x01\x8C\x90R3\x90\x7FY\xC3\xA0\xB6\x0Cj\xB7\xDE\xB6.\x14@\xC9\xE7$A\xDBm\xB7\xDF\xE5\x14\xDB\xA8\xCB\x18\xE6\x0C\r\x89n\xFA\x90``\x01a\x07\xE2V[B\x81\x11a\x11DW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08L\x90a7<V[a\x11La$\xCBV[`\0\x82`\x11T\x11a\x11iW`\x11Ta\x11d\x90\x84a6\xE5V[a\x11wV[\x82`\x11Ta\x11w\x91\x90a6\xE5V[\x90Pa\x11\x83B\x83a6\xE5V[a\x11\x8D\x90\x82a7}V[`\x14U`\x12\x83\x90U`\x15\x82\x90U`\0\x80Q` a9a\x839\x81Q\x91Ra\x11\xB1a\x17\xADV[a\x11\xB9a\x19\x15V[a\x11\xC1a\x1B\xF2V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01RB``\x82\x01R`\x80\x01`@Q\x80\x91\x03\x90\xA1PPPV[``a\x11\xF4a\x17\xADV[a\x11\xFCa\x19\x15V[a\x12\x04a\x1B\xF2V[`@\x80Q` \x81\x01\x94\x90\x94R\x83\x01\x91\x90\x91R``\x82\x01R`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[B\x81\x11a\x12QW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08L\x90a7<V[a\x12Ya$\xDCV[`\0\x82`\x0CT\x11a\x12vW`\x0CTa\x12q\x90\x84a6\xE5V[a\x12\x84V[\x82`\x0CTa\x12\x84\x91\x90a6\xE5V[\x90Pa\x12\x90B\x83a6\xE5V[a\x12\x9A\x90\x82a7}V[`\x0FU`\r\x83\x90U`\x10\x82\x90U`\0\x80Q` a9a\x839\x81Q\x91Ra\x11\xB1a\x17\xADV[B\x81\x11a\x12\xDDW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08L\x90a7<V[a\x12\xE5a$\xEDV[`\0\x82`\x07T\x11a\x13\x02W`\x07Ta\x12\xFD\x90\x84a6\xE5V[a\x13\x10V[\x82`\x07Ta\x13\x10\x91\x90a6\xE5V[\x90Pa\x13\x1CB\x83a6\xE5V[a\x13&\x90\x82a7}V[`\nU`\x08\x83\x90U`\x0B\x82\x90U`\0\x80Q` a9a\x839\x81Q\x91Ra\x11\xB1a\x17\xADV[`\0\x80`\0\x80`\0a\x13Za\x0B&V[\x92P\x92P\x92P`\0a\x13s`\x03T`\x05T\x86\x86\x86a\x1D\xF2V[\x90P`\0a\x13\x88\x88`\x04Ta\x0C/\x91\x90a6\xD2V[\x90P`\0a\x13\x99\x82\x84\x88\x88\x88a\x1E\xEEV[\x90P`\0`\x03T\x82a\x13\xAB\x91\x90a6\xE5V[\x90P`\0`\x05T\x84a\x13\xBD\x91\x90a6\xE5V[\x90P\x83`\x05\x81\x90UP\x81`\x03`\0\x82\x82Ta\x13\xD8\x91\x90a6\xD2V[\x92PP\x81\x90UP\x8A`\x04`\0\x82\x82Ta\x13\xF1\x91\x90a6\xD2V[\x90\x91UPP`\0T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\x14*\x903\x900\x90\x87\x90`\x04\x01a6\xF8V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x14\x82W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a9A\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x14\x96W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14\xBA\x91\x90a7\x1CV[P`\x01`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c#\xB8r\xDD30\x8E`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x15\0\x93\x92\x91\x90a6\xF8V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x15XW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a9A\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x15lW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\x90\x91\x90a7\x1CV[P`@\x80Q\x85\x81R` \x81\x01\x84\x90R\x90\x81\x01\x8C\x90R3\x90\x7F\xBE\xB3\x88W\x86\xD67\xA4t\xCB\xC2\x87\xC0\xA4E\x87#\x163\xA0w\xF0\xBD05MZK\x18\x99o\xCE\x90``\x01a\x07\xE2V[`\0\x80`\0\x80`\0a\x15\xE1a\x0B&V[\x92P\x92P\x92P`\0a\x15\xF6\x88\x88\x86\x86\x86a\x1E\xD8V[\x90P`\0a\x16\x07\x82\x89\x87\x87\x87a\x1E\xEEV[`\x05\x83\x90U`\x03\x81\x90U`\x04\x8A\x81U`\0T`@Qc#\xB8r\xDD`\xE0\x1B\x81R\x92\x93P`\x01`\x01`\xA0\x1B\x03\x16\x91c#\xB8r\xDD\x91a\x16I\x913\x910\x91\x87\x91\x01a6\xF8V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x16\xA1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a9A\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x16\xB5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\xD9\x91\x90a7\x1CV[P`\x01T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\x17\x0E\x903\x900\x90\x8E\x90`\x04\x01a6\xF8V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x17fW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a9A\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x17zW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17\x9E\x91\x90a7\x1CV[P\x90\x98\x90\x97P\x95PPPPPPV[`\0`\x0BTB\x10a\x17\xBFWP`\x08T\x90V[`\x08T`\x07T\x11a\x17\xF6W`\nT`\tTa\x17\xDA\x90Ba6\xE5V[a\x17\xE4\x91\x90a7\x91V[`\x07Ta\x17\xF1\x91\x90a6\xD2V[\x90P\x90V[`\nT`\tTa\x18\x06\x90Ba6\xE5V[a\x18\x10\x91\x90a7\x91V[`\x07Ta\x17\xF1\x91\x90a6\xE5V[`\0\x80`\0\x80a\x18+a\x0B&V[\x92P\x92P\x92Pa\x18ma\x18H`\x03T`\x04T`\x05T\x87\x87\x87a$\xFEV[`@Q` \x01a\x18X\x91\x90a7\xA8V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x86a%\\V[\x95\x94PPPPPV[`\0`\x04Tg\r\xE0\xB6\xB3\xA7d\0\0a\x18\x8Ca\x1C]V[`\x03Ta\x18\x99\x91\x90a7\x91V[a\x18\xA3\x91\x90a7}V[a\x17\xF1\x91\x90a6\xD2V[`\0\x80Q` a9a\x839\x81Q\x91Ra\x18\xC4a\x17\xADV[a\x18\xCCa\x19\x15V[a\x18\xD4a\x1B\xF2V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01RB``\x82\x01R`\x80\x01`@Q\x80\x91\x03\x90\xA1V[`\0a\x17\xF1`\x03T`\x05T`\x04Ta\x19\x10a\x19\x15V[a%\xD7V[`\0`\x10TB\x10a\x19'WP`\rT\x90V[`\rT`\x0CT\x11a\x19YW`\x0FT`\x0ETa\x19B\x90Ba6\xE5V[a\x19L\x91\x90a7\x91V[`\x0CTa\x17\xF1\x91\x90a6\xD2V[`\x0FT`\x0ETa\x19i\x90Ba6\xE5V[a\x19s\x91\x90a7\x91V[`\x0CTa\x17\xF1\x91\x90a6\xE5V[`\0\x80`\0\x80`\0a\x19\x90a\x0B&V[\x92P\x92P\x92P`\0a\x19\xA9`\x03T`\x05T\x86\x86\x86a\x1D\xF2V[\x90P`\0a\x19\xBE\x88`\x03Ta\x05\x94\x91\x90a6\xE5V[\x90P`\0a\x19\xCF\x82\x84\x88\x88\x88a\x1E\xB6V[\x90P`\0\x81`\x04Ta\x19\xE1\x91\x90a6\xE5V[\x90P`\0\x83`\x05Ta\x19\xF3\x91\x90a6\xE5V[\x90P\x83`\x05\x81\x90UP\x8A`\x03`\0\x82\x82Ta\x1A\x0E\x91\x90a6\xE5V[\x92PP\x81\x90UP\x81`\x04`\0\x82\x82Ta\x1A'\x91\x90a6\xE5V[\x90\x91UPP`\0T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x8D\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1A\xB6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a9A\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x1A\xCAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A\xEE\x91\x90a7\x1CV[P`\x01T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1ByW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a9A\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x1B\x8DW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B\xB1\x91\x90a7\x1CV[P`@\x80Q\x85\x81R` \x81\x01\x8D\x90R\x90\x81\x01\x83\x90R3\x90\x7FY\xC3\xA0\xB6\x0Cj\xB7\xDE\xB6.\x14@\xC9\xE7$A\xDBm\xB7\xDF\xE5\x14\xDB\xA8\xCB\x18\xE6\x0C\r\x89n\xFA\x90``\x01a\x07\xE2V[`\0`\x15TB\x10a\x1C\x04WP`\x12T\x90V[`\x12T`\x11T\x11a\x1C6W`\x14T`\x13Ta\x1C\x1F\x90Ba6\xE5V[a\x1C)\x91\x90a7\x91V[`\x11Ta\x17\xF1\x91\x90a6\xD2V[`\x14T`\x13Ta\x1CF\x90Ba6\xE5V[a\x1CP\x91\x90a7\x91V[`\x11Ta\x17\xF1\x91\x90a6\xE5V[`\0a\x17\xF1`\x03T`\x05Ta\x1Cpa\x19\x15V[a\x1Cxa\x17\xADV[a\x1C\x80a\x1B\xF2V[a\x1D\xF2V[`\0\x80`\0\x80a\x1C\x93a\x0B&V[\x92P\x92P\x92Pa\x1C\xAD`\x03T`\x04T`\x05T\x86\x86\x86a&\x07V[\x93PPPP\x90V[`\0\x80`\0\x80`\0a\x1C\xC5a\x0B&V[\x92P\x92P\x92P`\0a\x1C\xDA\x88\x88\x86\x86\x86a\x1EvV[\x90P`\0a\x1C\xEB\x82\x89\x87\x87\x87a\x1E\xB6V[`\x05\x83\x90U`\x03\x8A\x90U`\x04\x81\x81U`\0T`@Qc#\xB8r\xDD`\xE0\x1B\x81R\x92\x93P`\x01`\x01`\xA0\x1B\x03\x16\x91c#\xB8r\xDD\x91a\x1D-\x913\x910\x91\x8F\x91\x01a6\xF8V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1D\x85W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a9A\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x1D\x99W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1D\xBD\x91\x90a7\x1CV[P`\x01T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\x17\x0E\x903\x900\x90\x86\x90`\x04\x01a6\xF8V[`\0\x80a\x1D\xFF\x84\x84a&\xBFV[\x90P`\0a\x1E\r\x85\x85a&\xE5V[\x90P`\0a\x1E\x1B\x89\x89a'\x17V[\x90Pa\x1Ei\x87a\x1Ed\x84g\r\xE0\xB6\xB3\xA7d\0\0\x87a\x1EAa\x1E<\x88\x84a6\xE5V[a',V[a\x1EK\x91\x90a7\xECV[a\x1EU\x91\x90a8\x1CV[a\x1E_\x91\x90a8JV[a'\xC9V[a)rV[\x99\x98PPPPPPPPPV[`\0\x80a\x1E\x8Da\x1E\x88\x87\x87\x87\x87a)\x87V[a)\xE5V[a\x1E\x9F\x90g\r\xE0\xB6\xB3\xA7d\0\0a8JV[\x90Pa\x1E\xAB\x87\x82a*NV[\x97\x96PPPPPPPV[`\0\x80a\x1E\xC8a\x1E\x88\x87\x87\x87\x87a*cV[\x90Pa\x1E\xAB\x85a\x1Ed\x89\x84a)rV[`\0\x80a\x1E\x9F\x85a\x1Eda\x1E\x88\x89\x89\x89\x89a*cV[`\0\x80a\x1F\0a\x1E\x88\x87\x87\x87\x87a)\x87V[\x90Pa\x1E\xAB\x87a\x1Ed\x83g\r\xE0\xB6\xB3\xA7d\0\0a8JV[`\0\x80`\0\x80`\0a\x1F.\x8B\x8B\x8B\x8B\x8B\x8Ba$\xFEV[\x90P`\0a\x1F_\x82`@Q` \x01a\x1FF\x91\x90a7\xA8V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83`@\x01Qa%\\V[\x90P`\0\x81\x12\x15a\x1F\x83W\x89\x92Pa\x1F|\x83a'\x11a'\x10a*\x99V[\x93Pa\x1F\x96V[\x89\x93Pa\x1F\x93\x84`2`da*\xC7V[\x92P[P\x90\x9A\x91\x99P\x90\x97PPPPPPPPV[`\0\x80`\0\x80a\x1F\xB6a\x0B&V[\x92P\x92P\x92P`\0a\x1F\xCF`\x03T`\x05T\x86\x86\x86a\x1D\xF2V[\x90P\x86\x15a\"\x1CW`\0g\r\xE0\xB6\xB3\xA7d\0\0`\x02Tg\r\xE0\xB6\xB3\xA7d\0\0a\x1F\xF8\x91\x90a6\xE5V[a \x02\x90\x89a7\x91V[a \x0C\x91\x90a7}V[\x90P`\0a \x1D\x82\x84\x88\x88\x88a\x1EvV[\x90P`\x01a 7`\x03T`\x04T\x8B`\x05T\x86\x8C\x8C\x8Ca*\xE6V[a A\x91\x90a8JV[\x19\x96P\x80`\x05`\0\x82\x82Ta V\x91\x90a6\xD2V[\x92PP\x81\x90UP\x87`\x03`\0\x82\x82Ta o\x91\x90a6\xD2V[\x92PP\x81\x90UP\x86`\x04`\0\x82\x82Ta \x88\x91\x90a6\xE5V[\x90\x91UPP`\0T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a \xC1\x903\x900\x90\x8D\x90`\x04\x01a6\xF8V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a!\x19W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a9A\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a!-W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a!Q\x91\x90a7\x1CV[P`\x01T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x89\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a!\xDCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a9A\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a!\xF0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\"\x14\x91\x90a7\x1CV[PPPa$]V[`\0g\r\xE0\xB6\xB3\xA7d\0\0`\x02Tg\r\xE0\xB6\xB3\xA7d\0\0a\"=\x91\x90a6\xE5V[a\"G\x90\x89a7\x91V[a\"Q\x91\x90a7}V[\x90P`\0a\"b\x82\x84\x88\x88\x88a\x1E\xD8V[\x90P`\x01a\"|`\x03T`\x04T\x8B`\x05T\x86\x8C\x8C\x8Ca+gV[a\"\x86\x91\x90a8JV[\x19\x96P\x80`\x05`\0\x82\x82Ta\"\x9B\x91\x90a6\xD2V[\x92PP\x81\x90UP\x87`\x04`\0\x82\x82Ta\"\xB4\x91\x90a6\xD2V[\x92PP\x81\x90UP\x86`\x03`\0\x82\x82Ta\"\xCD\x91\x90a6\xE5V[\x90\x91UPP`\x01T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a#\x06\x903\x900\x90\x8D\x90`\x04\x01a6\xF8V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a#^W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a9A\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a#rW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a#\x96\x91\x90a7\x1CV[P`\0T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x89\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a$!W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a9A\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a$5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a$Y\x91\x90a7\x1CV[PPP[3`\x01`\x01`\xA0\x1B\x03\x16\x7F\x17fH\xF1\xF1\x1C\xDA(L\x12D\x90\x08k\xE4*\x92m\xDF\n\xE8\x87\xEB\xE7\xB1\xD6\xB37\xD8\x94'V\x88\x88\x88a$\x9C`\x03T`\x05T\x8B\x8B\x8Ba\x1D\xF2V[`@\x80Q\x94\x15\x15\x85R` \x85\x01\x93\x90\x93R\x91\x83\x01R``\x82\x01R`\x80\x01`@Q\x80\x91\x03\x90\xA2PPPP\x92\x91PPV[a$\xD3a\x1B\xF2V[`\x11UB`\x13UV[a$\xE4a\x19\x15V[`\x0CUB`\x0EUV[a$\xF5a\x17\xADV[`\x07UB`\tUV[a%7`@Q\x80`\xC0\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[\x95\x86R` \x86\x01\x94\x90\x94R`@\x85\x01\x92\x90\x92R``\x84\x01R`\x80\x83\x01R`\xA0\x82\x01R\x90V[`\0\x80\x83\x80` \x01\x90Q\x81\x01\x90a%s\x91\x90a8qV[\x90Pa%\x87\x81`\x80\x01Q\x82`\xA0\x01Qa&\xBFV[a%\xAAa%\xA5\x83` \x01Qa%\xA0\x87\x86``\x01Qa+\xB5V[a'\x17V[a+\xCAV[a%\xBBa%\xA5\x84`\0\x01Q\x87a'\x17V[a%\xC5\x91\x90a8\xFCV[a%\xCF\x91\x90a8\xFCV[\x94\x93PPPPV[`\0a%\xF0a%\xE6\x85\x84a+\xB5V[a\x1E<\x90\x85a7}V[a%\xFDa\x1E<\x86\x88a7}V[a\x18m\x91\x90a8\xFCV[`\0\x80`\0\x80a&\x1B\x8A\x8A\x8A\x8A\x8A\x8Aa$\xFEV[\x90P`\0a&3\x82`@Q` \x01a\x1FF\x91\x90a7\xA8V[\x90P\x80`\0\x03a&IW\x88\x94PPPPPa&\xB5V[`\0\x81\x12\x15a&iW\x88\x92Pa&b\x83`c`da*\xC7V[\x93Pa&|V[\x88\x93Pa&y\x84`e`da*\x99V[\x92P[a&\xAE\x82`@Q` \x01a&\x90\x91\x90a7\xA8V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x85\x85`\x01a\x01\0a%\\a+\xD8V[\x94PPPPP[\x96\x95PPPPPPV[`\0\x80a&\xCB\x83a,\xE9V[a&\xD9\x90c;\x9A\xCA\0a7\x91V[\x90Pa%\xCF\x84\x82a+\xB5V[`\0\x80a'\x03a&\xFD\x85g\x1B\xC1mgN\xC8\0\0a-\x8DV[\x84a+\xB5V[\x90Pa%\xCFg\x06\xF0[Y\xD3\xB2\0\0\x82a+\xB5V[`\0a\n\xAB\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a*\xC7V[`\0g\x06\xF0[Y\xD3\xB2\0\0\x82\x03a'EWP`\0\x91\x90PV[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x12a'mW`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a'\x8EW`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a'\x9B\x83`\x02a7\xECV[\x90P`\0a'\xA8\x82a-\xB9V[\x90P`\0a'\xBEg\x13\xA0K\xBD\xFD\xC9\xBE\x88\x83a07V[\x90Pa\x18m\x81a9$V[`\0h\x02H\xCE6\xA7\x0C\xB2k>\x19\x82\x13a'\xE4WP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a(+W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x01a\x08LV[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05`\x01`_\x1B\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1Dm\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1Dn\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[`\0a\n\xAB\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a*\x99V[`\0\x80a)\x94\x84\x84a&\xBFV[\x90P`\0a)\xA2\x87\x87a0LV[\x90P`\0a)\xB0\x86\x86a&\xE5V[\x90P\x82a)\xBD\x82\x84a8\xFCV[a)\xCF\x90g\r\xE0\xB6\xB3\xA7d\0\0a7\xECV[a)\xD9\x91\x90a8\x1CV[\x98\x97PPPPPPPPV[`\0\x80g\x13\xA0K\xBD\xFD\xC9\xBE\x88a*\x03g\r\xE0\xB6\xB3\xA7d\0\0\x85a7\xECV[a*\r\x91\x90a8\x1CV[\x90P`\0a*\x1A\x82a9$V[\x90P`\0a*'\x82a0`V[\x90Pg\x1B\xC1mgN\xC8\0\0a*Dg\r\xE0\xB6\xB3\xA7d\0\0\x83a7\xECV[a\x18m\x91\x90a8\x1CV[`\0a\n\xAB\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a*\x99V[`\0\x80a*p\x84\x84a&\xBFV[\x90P`\0a*~\x87\x87a0LV[\x90P`\0a*\x8C\x86\x86a&\xE5V[\x90P\x82a)\xBD\x82\x84a8JV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a*\xB1W`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a*\xDFW`\0\x80\xFD[\x04\x92\x91PPV[`\0\x80a*\xF3\x84\x84a&\xBFV[\x90P`\0a+\n\x86a+\x05\x89\x8Ba6\xD2V[a+\xB5V[\x90P`\0a+@a+-a\x1E<\x8C\x8Fa+#\x91\x90a6\xD2V[a%\xA0\x8C\x8Ea6\xD2V[a+6\x85a9$V[a\x1E\x88\x91\x90a8JV[\x90P\x8Aa+M\x83\x83a+\xB5V[a+W\x91\x90a8JV[\x9C\x9BPPPPPPPPPPPPV[`\0\x80a+t\x84\x84a&\xBFV[\x90P`\0a+\x86\x86a+\x05\x89\x8Ba6\xD2V[\x90P`\0a+\xA3a+-a\x1E<a+\x9D\x8D\x8Fa6\xD2V[\x85a'\x17V[\x90P\x8Ba+Ma+\xB3\x8A\x8Ca6\xD2V[\x83[`\0a\n\xAB\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a*\xC7V[`\0\x80a'\x9B\x83`\x02a7\xECV[`\0\x84\x86\x11\x15a,\x05W`@Qc0\x82\xDF\xDB`\xE1\x1B\x81R`\x04\x81\x01\x87\x90R`$\x81\x01\x86\x90R`D\x01a\x08LV[`\0a,\x15\x88\x88\x85c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a,'\x89\x88\x86c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a,5\x82\x84a7\xECV[\x13\x15a,^W`@Qc\x06\xF1\xBE]`\xE2\x1B\x81R`\x04\x81\x01\x83\x90R`$\x81\x01\x82\x90R`D\x01a\x08LV[`\0a,j\x89\x89a6\xE5V[\x90P`\0[`\x02a,{\x8A\x8Ca6\xD2V[a,\x85\x91\x90a7}V[\x94P`\0a,\x97\x8C\x87\x89c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a,\xA5\x86\x83a7\xECV[\x13a,\xB2W\x85\x99Pa,\xB9V[\x85\x9AP\x80\x94P[a,\xC3\x8B\x8Ba6\xE5V[\x92PP`\x01\x01\x87\x82\x11\x80\x15a,\xD7WP\x86\x81\x10[a,oWPPPP\x96\x95PPPPPPV[`\xB5\x81`\x01`\x88\x1B\x81\x10a-\x02W`@\x91\x90\x91\x1B\x90`\x80\x1C[i\x01\0\0\0\0\0\0\0\0\0\x81\x10a-\x1EW` \x91\x90\x91\x1B\x90`@\x1C[e\x01\0\0\0\0\0\x81\x10a-6W`\x10\x91\x90\x91\x1B\x90` \x1C[c\x01\0\0\0\x81\x10a-LW`\x08\x91\x90\x91\x1B\x90`\x10\x1C[b\x01\0\0\x01\x02`\x12\x1C\x80\x82\x04\x01`\x01\x90\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x90\x81\x90\x04\x81\x11\x90\x03\x90V[`\0a\n\xABg\r\xE0\xB6\xB3\xA7d\0\0\x83a-\xA5\x86a2DV[a-\xAF\x91\x90a7\xECV[a\x1E_\x91\x90a8\x1CV[`\0\x80\x82\x12\x80a-\xD0WPg\x1B\xC1mgN\xC8\0\0\x82\x13[\x15a-\xEEW`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a.\x0FW`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81g\x1B\xC1mgN\xC8\0\0\x03a.7W`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x15a.BW\x91\x90PV[`\0g\r\xE0\xB6\xB3\xA7d\0\0\x83\x12a.jWa.e\x83g\x1B\xC1mgN\xC8\0\0a8JV[a.lV[\x82[\x90P`\0a.\x82\x82g\x1B\xC1mgN\xC8\0\0a4\x1FV[\x90P\x80`\0\x03a.\xA5W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a.\xB0\x82a2DV[\x90P`\0c;\x9A\xCA\0a.\xDBa.\xD6a.\xD0g\x1B\xC1mgN\xC8\0\0a9$V[\x85a07V[a,\xE9V[a.\xE5\x91\x90a7\xECV[\x90P`\0\x80a.\xFC\x83g\x03\xC1f\\z\xAB \0a07V[a/\x0E\x90g \x05\xFEO&\x8E\xA0\0a8\xFCV[\x90P`\0a/>\x84a/'\x86f\x9F2u$b\xA0\0a07V[a/9\x90g\r\xC5R\x7Fd, \0a8\xFCV[a07V[a/P\x90g\r\xE0\xB6\xB3\xA7d\0\0a8\xFCV[\x90Pa/tg\t\xD0(\xCCo _\xFF\x19\x85a/j\x85\x85a4\x1FV[a/9\x91\x90a8JV[\x92PPP`\0[`\x02\x81\x10\x15a0\x0FW`\0\x86a/\x90\x84a0`V[a/\x9A\x91\x90a8JV[\x90P`\0a/\xA8\x84\x85a07V[a/\xB1\x90a9$V[\x90P`\0a/\xBE\x82a'\xC9V[\x90P`\0a/\xCC\x86\x85a07V[a/\xDEg\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x84a07V[a/\xE8\x91\x90a8JV[\x90Pa/\xF4\x84\x82a4\x1FV[a/\xFE\x90\x87a8\xFCV[\x95P\x84`\x01\x01\x94PPPPPa/{V[g\r\xE0\xB6\xB3\xA7d\0\0\x88\x12a0,Wa0'\x82a9$V[a)\xD9V[P\x96\x95PPPPPPV[`\0a\n\xAB\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a40V[`\0a\n\xABa0[\x84\x84a*NV[a2DV[`\0\x81`\0\x03a0yWPg\r\xE0\xB6\xB3\xA7d\0\0\x91\x90PV[gV\x98\xEE\xF0fp\0\0\x82\x12a0\x90WP`\0\x91\x90PV[a0\xA1gV\x98\xEE\xF0fp\0\0a9$V[\x82\x13a0\xB6WPg\x1B\xC1mgN\xC8\0\0\x91\x90PV[`\0a0\xC1\x83a4OV[\x90P`\0a0\xFAg\r\xE0\xB6\xB3\xA7d\0\0a0\xE3\x84g\x1B\xC1mgN\xC8\0\0a'\x17V[a0\xF5\x90g\r\xE0\xB6\xB3\xA7d\0\0a8\xFCV[a4\x1FV[\x90P`\0\x80\x82a1V\x81a1C\x81a11\x81a1\x1E\x81g\x02_\x0F\xE1\x05\xA3\x14\0a07V[a/9\x90g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19a8\xFCV[a/9\x90g\x14\xA8EL\x19\xE1\xAC\0a8\xFCV[a/9\x90g\x0F\xC1\x0E\x01W\x82w\xFF\x19a8\xFCV[a1h\x90g\x03\xDE\xBD\x08;\x8C|\0a8\xFCV[\x91P\x83\x90Pa1\xD0\x81a1\xBE\x81a1\xAC\x81a1\x9A\x81a1\x87\x81\x8Ba07V[a/9\x90g\x02\x95\xD4\0\xEA2W\xFF\x19a8\xFCV[a/9\x90g\x01W\xD8\xB2\xEC\xC7\x08\0a8\xFCV[a/9\x90g\x051\n\xA7\xD5!0\0a8\xFCV[a/9\x90g\r\xE0\xCC=\x15a\0\0a8\xFCV[\x91P\x81g\x11\x90\0\xAB\x10\x0F\xFC\0a1\xE6\x87\x88a07V[a1\xF2\x90`\0\x19a7\xECV[a1\xFC\x91\x90a8JV[a2\x06\x91\x90a8\xFCV[\x92PP`\0a2\x14\x83a'\xC9V[\x90P`\0a2\"\x85\x83a07V[\x90P`\0\x88\x12a22W\x80a)\xD9V[a)\xD9\x81g\x1B\xC1mgN\xC8\0\0a8JV[`\0\x80\x82\x13a2\x81W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x08LV[`\0``a2\x8E\x84a4\x8AV[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[`\0a\n\xAB\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x05\x85\x14\x17\x16a4HW`\0\x80\xFD[\x05\x92\x91PPV[`\0`\x01`\xFF\x1B\x82\x03a4uW`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x12\x15a4\x86WP\x19`\x01\x01\x90V[P\x90V[`\0\x80\x82\x11a4\xC7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x08LV[P`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x82\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`\0` \x82\x84\x03\x12\x15a5\x97Wa5\x97a52V[P5\x91\x90PV[\x80\x15\x15\x81\x14a5\xACW`\0\x80\xFD[PV[`\0\x80`\0``\x84\x86\x03\x12\x15a5\xC7Wa5\xC7a52V[\x835a5\xD2\x81a5\x9EV[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a5\xFDWa5\xFDa52V[\x825a6\x08\x81a5\x9EV[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`@\x83\x85\x03\x12\x15a6,Wa6,a52V[PP\x805\x92` \x90\x91\x015\x91PV[`\0` \x82\x84\x03\x12\x15a6PWa6Pa52V[\x815`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a6gW`\0\x80\xFD[\x93\x92PPPV[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a6\x9BW\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a6\x7FV[P`\0`@\x82\x86\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x92PPP\x92\x91PPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\n\xAEWa\n\xAEa6\xBCV[\x81\x81\x03\x81\x81\x11\x15a\n\xAEWa\n\xAEa6\xBCV[`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x81R\x91\x90\x92\x16` \x82\x01R`@\x81\x01\x91\x90\x91R``\x01\x90V[`\0` \x82\x84\x03\x12\x15a71Wa71a52V[\x81Qa6g\x81a5\x9EV[` \x80\x82R`\x11\x90\x82\x01Rp\x15\\\x19\x18]\x19H\x19[\x99\x08\x1C\x18\\\xDC\xD9Y`z\x1B`@\x82\x01R``\x01\x90V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a7\x8CWa7\x8Ca7gV[P\x04\x90V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\n\xAEWa\n\xAEa6\xBCV[`\0`\xC0\x82\x01\x90P\x82Q\x82R` \x83\x01Q` \x83\x01R`@\x83\x01Q`@\x83\x01R``\x83\x01Q``\x83\x01R`\x80\x83\x01Q`\x80\x83\x01R`\xA0\x83\x01Q`\xA0\x83\x01R\x92\x91PPV[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15a8\x08Wa8\x08a6\xBCV[\x81\x81\x05\x83\x14\x82\x15\x17a\n\xAEWa\n\xAEa6\xBCV[`\0\x82a8+Wa8+a7gV[`\x01`\xFF\x1B\x82\x14`\0\x19\x84\x14\x16\x15a8EWa8Ea6\xBCV[P\x05\x90V[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a8jWa8ja6\xBCV[P\x92\x91PPV[`\0`\xC0\x82\x84\x03\x12\x15a8\x86Wa8\x86a52V[`@Q`\xC0\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a8\xB7WcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x80`@RP\x82Q\x81R` \x83\x01Q` \x82\x01R`@\x83\x01Q`@\x82\x01R``\x83\x01Q``\x82\x01R`\x80\x83\x01Q`\x80\x82\x01R`\xA0\x83\x01Q`\xA0\x82\x01R\x80\x91PP\x92\x91PPV[\x80\x82\x01\x82\x81\x12`\0\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a9\x1CWa9\x1Ca6\xBCV[PP\x92\x91PPV[`\0`\x01`\xFF\x1B\x82\x01a99Wa99a6\xBCV[P`\0\x03\x90V\xFETarget contract does not contains\xEBa\xB7\x83\x9B\xB3\xC27\xA0\xB3\xAE\xAB\x18iQ\0\xBCQ.q9\xF0\xE3@e]\xD7\x02\xDA*\xA1";
    /// The bytecode of the contract.
    pub static RMM_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x02\x7FW`\x005`\xE0\x1C\x80c\x8DR\xA1\xFC\x11a\x01}W\x80c\xC0\xFF\x1A\x15\x11a\x01\x05W\x80c\xDCv\xFA\xBC\x11a\0\xC9W\x80c\xDCv\xFA\xBC\x14a\x05\"W\x80c\xF2\xBFMT\x14a\x05*W\x80c\xF9\xA1\xC8Z\x14a\x052W\x80c\xFA\xDF\xA6[\x14a\x05EW\x80c\xFE\xD3\xDF\xDA\x14a\x05NWa\x02\x7FV[\x80c\xC0\xFF\x1A\x15\x14a\x04\xEFW\x80c\xC5)\x87\xCF\x14a\x04\xF7W\x80c\xCC\xD1\xE4\xBE\x14a\x04\xFFW\x80c\xCF\xC4\xAFU\x14a\x05\x12W\x80c\xD4\xCA\xDFh\x14a\x05\x1AWa\x02\x7FV[\x80c\xAF\xDF1\xCD\x11a\x01LW\x80c\xAF\xDF1\xCD\x14a\x04\xB1W\x80c\xB7\xD1\x9F\xC4\x14a\x04\xB9W\x80c\xB9\xD9Z\xCA\x14a\x04\xCCW\x80c\xBB\x04\x98\xDE\x14a\x04\xDFW\x80c\xBC\xC1}\xC7\x14a\x04\xE7Wa\x02\x7FV[\x80c\x8DR\xA1\xFC\x14a\x04eW\x80c\x99\x87\xFEd\x14a\x04xW\x80c\x9C\x9D\xA9\xEA\x14a\x04\x8BW\x80c\xA5\x9C\x18o\x14a\x04\x9EWa\x02\x7FV[\x80c69\xAA2\x11a\x02\x0BW\x80cp\xA0\x821\x11a\x01\xCFW\x80cp\xA0\x821\x14a\x03\xF7W\x80cvp\x166\x14a\x04\x17W\x80c}iS~\x14a\x04*W\x80c\x7F\x0EL\x8C\x14a\x04=W\x80c\x87kU\xF1\x14a\x04PWa\x02\x7FV[\x80c69\xAA2\x14a\x03\xC2W\x80c>\x1E3\x92\x14a\x03\xD5W\x80cT\xCF*\xEB\x14a\x03\xDEW\x80cU\x9D\x16\x02\x14a\x03\xE7W\x80c^aZk\x14a\x03\xEFWa\x02\x7FV[\x80c\t\x10\xA5\x10\x11a\x02RW\x80c\t\x10\xA5\x10\x14a\x03^W\x80c\x15w\x0F\x92\x14a\x03fW\x80c\x16\xDC\x16[\x14a\x03oW\x80c\x1F\xDA\xBC'\x14a\x03\x9AW\x80c4\xE1\x99\x07\x14a\x03\xADWa\x02\x7FV[\x80c\x02\xC2\xE5]\x14a\x02\xE4W\x80c\x04\xAF\xA8\"\x14a\x03\x11W\x80c\x06\x81\x83\x1A\x14a\x03?W\x80c\x08\xEA\xBD\xDA\x14a\x03GW[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x02\xF7a\x02\xF26`\x04a5\x82V[a\x05VV[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01[`@Q\x80\x91\x03\x90\xF3[a\x03$a\x03\x1F6`\x04a5\xAFV[a\x07\xF9V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x03\x08V[a\x02\xF7a\njV[a\x03P`\x03T\x81V[`@Q\x90\x81R` \x01a\x03\x08V[`\x05Ta\x03PV[a\x03P`\x05T\x81V[`\0Ta\x03\x82\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x03\x08V[a\x03Pa\x03\xA86`\x04a5\xE7V[a\n\x9FV[a\x03\xC0a\x03\xBB6`\x04a5\x82V[a\n\xB4V[\0[a\x03$a\x03\xD06`\x04a6\x16V[a\x0B\x08V[a\x03P`\rT\x81V[a\x03P`\x02T\x81V[`\x03Ta\x03PV[a\x03$a\x0B&V[a\x03Pa\x04\x056`\x04a6;V[`\x06` R`\0\x90\x81R`@\x90 T\x81V[a\x03$a\x04%6`\x04a5\xE7V[a\x0BNV[a\x02\xF7a\x0486`\x04a5\x82V[a\x0E\xB3V[a\x03\xC0a\x04K6`\x04a6\x16V[a\x11%V[a\x04Xa\x11\xEAV[`@Qa\x03\x08\x91\x90a6nV[a\x03\xC0a\x04s6`\x04a6\x16V[a\x122V[a\x03\xC0a\x04\x866`\x04a6\x16V[a\x12\xBEV[a\x02\xF7a\x04\x996`\x04a5\x82V[a\x13JV[a\x02\xF7a\x04\xAC6`\x04a6\x16V[a\x15\xD1V[a\x03Pa\x17\xADV[`\x01Ta\x03\x82\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x03Pa\x04\xDA6`\x04a5\x82V[a\x18\x1DV[a\x03Pa\x18vV[a\x03\xC0a\x18\xADV[a\x03Pa\x18\xFAV[a\x03Pa\x19\x15V[a\x02\xF7a\x05\r6`\x04a5\x82V[a\x19\x80V[a\x03Pa\x1B\xF2V[`\x02Ta\x03PV[a\x03Pa\x1C]V[a\x03Pa\x1C\x85V[a\x02\xF7a\x05@6`\x04a6\x16V[a\x1C\xB5V[a\x03P`\x04T\x81V[`\x04Ta\x03PV[`\0\x80`\0\x80`\0a\x05fa\x0B&V[\x92P\x92P\x92P`\0a\x05\x7F`\x03T`\x05T\x86\x86\x86a\x1D\xF2V[\x90P`\0a\x05\x9D\x88`\x03Ta\x05\x94\x91\x90a6\xD2V[\x83\x87\x87\x87a\x1EvV[\x90P`\0a\x05\xAE\x82\x84\x88\x88\x88a\x1E\xB6V[\x90P`\0`\x04T\x82a\x05\xC0\x91\x90a6\xE5V[\x90P`\0`\x05T\x84a\x05\xD2\x91\x90a6\xE5V[\x90P\x83`\x05\x81\x90UP\x8A`\x03`\0\x82\x82Ta\x05\xED\x91\x90a6\xD2V[\x92PP\x81\x90UP\x81`\x04`\0\x82\x82Ta\x06\x06\x91\x90a6\xD2V[\x92PP\x81\x90UP`\0\x80T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c#\xB8r\xDD30\x8E`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x06P\x93\x92\x91\x90a6\xF8V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x06\xA8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a9A\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x06\xBCW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xE0\x91\x90a7\x1CV[P`\x01T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\x07\x15\x903\x900\x90\x87\x90`\x04\x01a6\xF8V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x07mW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a9A\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x07\x81W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\xA5\x91\x90a7\x1CV[P`@\x80Q\x85\x81R` \x81\x01\x8D\x90R\x90\x81\x01\x83\x90R3\x90\x7F\xBE\xB3\x88W\x86\xD67\xA4t\xCB\xC2\x87\xC0\xA4E\x87#\x163\xA0w\xF0\xBD05MZK\x18\x99o\xCE\x90``\x01[`@Q\x80\x91\x03\x90\xA2\x9A\x90\x99P\x97PPPPPPPPV[`\0\x80`\0`\x05T`\0\x14a\x08UW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FPool already initialized\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x80`\0a\x08ba\x0B&V[\x92P\x92P\x92P\x88\x15a\x08\x94W\x87\x95Pa\x08~\x86\x88\x85\x85\x85a\x1EvV[\x93Pa\x08\x8D\x84\x88\x85\x85\x85a\x1E\xB6V[\x94Pa\x08\xB6V[\x87\x94Pa\x08\xA4\x85\x88\x85\x85\x85a\x1E\xD8V[\x93Pa\x08\xB3\x84\x88\x85\x85\x85a\x1E\xEEV[\x95P[`\x05\x84\x90U`\x03\x86\x90U`\x04\x85\x81U3`\0\x81\x81R`\x06` R`@\x80\x82 \x88\x90U\x90T\x90Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x92c#\xB8r\xDD\x92a\t\x08\x92\x90\x910\x91\x8C\x91\x01a6\xF8V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\t`W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a9A\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\ttW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\x98\x91\x90a7\x1CV[P`\x01T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\t\xCD\x903\x900\x90\x8A\x90`\x04\x01a6\xF8V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\n%W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a9A\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\n9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n]\x91\x90a7\x1CV[PPPP\x93P\x93P\x93\x90PV[`\0\x80`\0\x80`\0a\nza\x0B&V[\x92P\x92P\x92Pa\n\x94`\x03T`\x04T`\x05T\x86\x86\x86a\x1F\x18V[\x94P\x94PPPP\x90\x91V[`\0a\n\xAB\x83\x83a\x1F\xA8V[\x90P[\x92\x91PPV[g\r\xE0\xB6\xB3\xA7d\0\0\x81\x10a\x0B\x03W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rt\t\xCC\xAE\xE4\x0En\xEC.\x04\x0C\xCC\xAC\xA4\x0E\x8D\xED\xE4\r\r,\xED`[\x1B`D\x82\x01R`d\x01a\x08LV[`\x02UV[`\0\x80`\0a\x0B\x19`\x01\x86\x86a\x07\xF9V[\x92P\x92P\x92P\x92P\x92P\x92V[`\0\x80`\0a\x0B3a\x19\x15V[a\x0B;a\x17\xADV[a\x0BCa\x1B\xF2V[\x92P\x92P\x92P\x90\x91\x92V[`\0\x80`\0\x80`\x05T\x11a\x0B\x9BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rs\x14\x1B\xDB\xDB\x08\x1B\x9B\xDD\x08\x1A[\x9A]\x1AX[\x1A^\x99Y`b\x1B`D\x82\x01R`d\x01a\x08LV[`\0\x80`\0a\x0B\xA8a\x0B&V[\x92P\x92P\x92P`\0a\x0B\xC1`\x03T`\x05T\x86\x86\x86a\x1D\xF2V[\x90P\x88\x15a\x0C\x19W\x87\x96P`\0a\x0B\xDF\x88`\x03Ta\x05\x94\x91\x90a6\xD2V[\x90P`\0a\x0B\xF0\x82\x84\x88\x88\x88a\x1E\xB6V[\x90P`\x04T\x81a\x0C\0\x91\x90a6\xE5V[\x97P`\x05T\x82a\x0C\x10\x91\x90a6\xE5V[\x96PPPa\x0CnV[\x87\x95P`\0a\x0C8\x87`\x04Ta\x0C/\x91\x90a6\xD2V[\x83\x87\x87\x87a\x1E\xD8V[\x90P`\0a\x0CI\x82\x84\x88\x88\x88a\x1E\xEEV[\x90P`\x03T\x81a\x0CY\x91\x90a6\xE5V[\x98P`\x05T\x82a\x0Ci\x91\x90a6\xE5V[\x96PPP[\x84`\x05`\0\x82\x82Ta\x0C\x80\x91\x90a6\xD2V[\x92PP\x81\x90UP\x86`\x03`\0\x82\x82Ta\x0C\x99\x91\x90a6\xD2V[\x92PP\x81\x90UP\x85`\x04`\0\x82\x82Ta\x0C\xB2\x91\x90a6\xD2V[\x90\x91UPP3`\0\x90\x81R`\x06` R`@\x81 \x80T\x87\x92\x90a\x0C\xD6\x90\x84\x90a6\xD2V[\x90\x91UPP`\0T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\r\x0F\x903\x900\x90\x8C\x90`\x04\x01a6\xF8V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\rgW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a9A\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\r{W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\x9F\x91\x90a7\x1CV[P`\x01T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\r\xD4\x903\x900\x90\x8B\x90`\x04\x01a6\xF8V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0E,W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a9A\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x0E@W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Ed\x91\x90a7\x1CV[P`@\x80Q\x86\x81R` \x81\x01\x89\x90R\x90\x81\x01\x87\x90R3\x90\x7F\xBE\xB3\x88W\x86\xD67\xA4t\xCB\xC2\x87\xC0\xA4E\x87#\x163\xA0w\xF0\xBD05MZK\x18\x99o\xCE\x90``\x01`@Q\x80\x91\x03\x90\xA2PPPP\x92P\x92P\x92V[`\0\x80`\0\x80`\0a\x0E\xC3a\x0B&V[\x92P\x92P\x92P`\0a\x0E\xDC`\x03T`\x05T\x86\x86\x86a\x1D\xF2V[\x90P`\0a\x0E\xF1\x88`\x03Ta\x0C/\x91\x90a6\xE5V[\x90P`\0a\x0F\x02\x82\x84\x88\x88\x88a\x1E\xEEV[\x90P`\0\x81`\x03Ta\x0F\x14\x91\x90a6\xE5V[\x90P`\0\x83`\x05Ta\x0F&\x91\x90a6\xE5V[\x90P\x83`\x05\x81\x90UP\x81`\x03`\0\x82\x82Ta\x0FA\x91\x90a6\xE5V[\x92PP\x81\x90UP\x8A`\x04`\0\x82\x82Ta\x0FZ\x91\x90a6\xE5V[\x90\x91UPP`\0T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0F\xE9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a9A\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x0F\xFDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10!\x91\x90a7\x1CV[P`\x01T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x8D\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x10\xACW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a9A\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x10\xC0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\xE4\x91\x90a7\x1CV[P`@\x80Q\x85\x81R` \x81\x01\x84\x90R\x90\x81\x01\x8C\x90R3\x90\x7FY\xC3\xA0\xB6\x0Cj\xB7\xDE\xB6.\x14@\xC9\xE7$A\xDBm\xB7\xDF\xE5\x14\xDB\xA8\xCB\x18\xE6\x0C\r\x89n\xFA\x90``\x01a\x07\xE2V[B\x81\x11a\x11DW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08L\x90a7<V[a\x11La$\xCBV[`\0\x82`\x11T\x11a\x11iW`\x11Ta\x11d\x90\x84a6\xE5V[a\x11wV[\x82`\x11Ta\x11w\x91\x90a6\xE5V[\x90Pa\x11\x83B\x83a6\xE5V[a\x11\x8D\x90\x82a7}V[`\x14U`\x12\x83\x90U`\x15\x82\x90U`\0\x80Q` a9a\x839\x81Q\x91Ra\x11\xB1a\x17\xADV[a\x11\xB9a\x19\x15V[a\x11\xC1a\x1B\xF2V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01RB``\x82\x01R`\x80\x01`@Q\x80\x91\x03\x90\xA1PPPV[``a\x11\xF4a\x17\xADV[a\x11\xFCa\x19\x15V[a\x12\x04a\x1B\xF2V[`@\x80Q` \x81\x01\x94\x90\x94R\x83\x01\x91\x90\x91R``\x82\x01R`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[B\x81\x11a\x12QW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08L\x90a7<V[a\x12Ya$\xDCV[`\0\x82`\x0CT\x11a\x12vW`\x0CTa\x12q\x90\x84a6\xE5V[a\x12\x84V[\x82`\x0CTa\x12\x84\x91\x90a6\xE5V[\x90Pa\x12\x90B\x83a6\xE5V[a\x12\x9A\x90\x82a7}V[`\x0FU`\r\x83\x90U`\x10\x82\x90U`\0\x80Q` a9a\x839\x81Q\x91Ra\x11\xB1a\x17\xADV[B\x81\x11a\x12\xDDW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08L\x90a7<V[a\x12\xE5a$\xEDV[`\0\x82`\x07T\x11a\x13\x02W`\x07Ta\x12\xFD\x90\x84a6\xE5V[a\x13\x10V[\x82`\x07Ta\x13\x10\x91\x90a6\xE5V[\x90Pa\x13\x1CB\x83a6\xE5V[a\x13&\x90\x82a7}V[`\nU`\x08\x83\x90U`\x0B\x82\x90U`\0\x80Q` a9a\x839\x81Q\x91Ra\x11\xB1a\x17\xADV[`\0\x80`\0\x80`\0a\x13Za\x0B&V[\x92P\x92P\x92P`\0a\x13s`\x03T`\x05T\x86\x86\x86a\x1D\xF2V[\x90P`\0a\x13\x88\x88`\x04Ta\x0C/\x91\x90a6\xD2V[\x90P`\0a\x13\x99\x82\x84\x88\x88\x88a\x1E\xEEV[\x90P`\0`\x03T\x82a\x13\xAB\x91\x90a6\xE5V[\x90P`\0`\x05T\x84a\x13\xBD\x91\x90a6\xE5V[\x90P\x83`\x05\x81\x90UP\x81`\x03`\0\x82\x82Ta\x13\xD8\x91\x90a6\xD2V[\x92PP\x81\x90UP\x8A`\x04`\0\x82\x82Ta\x13\xF1\x91\x90a6\xD2V[\x90\x91UPP`\0T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\x14*\x903\x900\x90\x87\x90`\x04\x01a6\xF8V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x14\x82W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a9A\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x14\x96W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14\xBA\x91\x90a7\x1CV[P`\x01`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c#\xB8r\xDD30\x8E`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x15\0\x93\x92\x91\x90a6\xF8V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x15XW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a9A\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x15lW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\x90\x91\x90a7\x1CV[P`@\x80Q\x85\x81R` \x81\x01\x84\x90R\x90\x81\x01\x8C\x90R3\x90\x7F\xBE\xB3\x88W\x86\xD67\xA4t\xCB\xC2\x87\xC0\xA4E\x87#\x163\xA0w\xF0\xBD05MZK\x18\x99o\xCE\x90``\x01a\x07\xE2V[`\0\x80`\0\x80`\0a\x15\xE1a\x0B&V[\x92P\x92P\x92P`\0a\x15\xF6\x88\x88\x86\x86\x86a\x1E\xD8V[\x90P`\0a\x16\x07\x82\x89\x87\x87\x87a\x1E\xEEV[`\x05\x83\x90U`\x03\x81\x90U`\x04\x8A\x81U`\0T`@Qc#\xB8r\xDD`\xE0\x1B\x81R\x92\x93P`\x01`\x01`\xA0\x1B\x03\x16\x91c#\xB8r\xDD\x91a\x16I\x913\x910\x91\x87\x91\x01a6\xF8V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x16\xA1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a9A\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x16\xB5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\xD9\x91\x90a7\x1CV[P`\x01T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\x17\x0E\x903\x900\x90\x8E\x90`\x04\x01a6\xF8V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x17fW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a9A\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x17zW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17\x9E\x91\x90a7\x1CV[P\x90\x98\x90\x97P\x95PPPPPPV[`\0`\x0BTB\x10a\x17\xBFWP`\x08T\x90V[`\x08T`\x07T\x11a\x17\xF6W`\nT`\tTa\x17\xDA\x90Ba6\xE5V[a\x17\xE4\x91\x90a7\x91V[`\x07Ta\x17\xF1\x91\x90a6\xD2V[\x90P\x90V[`\nT`\tTa\x18\x06\x90Ba6\xE5V[a\x18\x10\x91\x90a7\x91V[`\x07Ta\x17\xF1\x91\x90a6\xE5V[`\0\x80`\0\x80a\x18+a\x0B&V[\x92P\x92P\x92Pa\x18ma\x18H`\x03T`\x04T`\x05T\x87\x87\x87a$\xFEV[`@Q` \x01a\x18X\x91\x90a7\xA8V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x86a%\\V[\x95\x94PPPPPV[`\0`\x04Tg\r\xE0\xB6\xB3\xA7d\0\0a\x18\x8Ca\x1C]V[`\x03Ta\x18\x99\x91\x90a7\x91V[a\x18\xA3\x91\x90a7}V[a\x17\xF1\x91\x90a6\xD2V[`\0\x80Q` a9a\x839\x81Q\x91Ra\x18\xC4a\x17\xADV[a\x18\xCCa\x19\x15V[a\x18\xD4a\x1B\xF2V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01RB``\x82\x01R`\x80\x01`@Q\x80\x91\x03\x90\xA1V[`\0a\x17\xF1`\x03T`\x05T`\x04Ta\x19\x10a\x19\x15V[a%\xD7V[`\0`\x10TB\x10a\x19'WP`\rT\x90V[`\rT`\x0CT\x11a\x19YW`\x0FT`\x0ETa\x19B\x90Ba6\xE5V[a\x19L\x91\x90a7\x91V[`\x0CTa\x17\xF1\x91\x90a6\xD2V[`\x0FT`\x0ETa\x19i\x90Ba6\xE5V[a\x19s\x91\x90a7\x91V[`\x0CTa\x17\xF1\x91\x90a6\xE5V[`\0\x80`\0\x80`\0a\x19\x90a\x0B&V[\x92P\x92P\x92P`\0a\x19\xA9`\x03T`\x05T\x86\x86\x86a\x1D\xF2V[\x90P`\0a\x19\xBE\x88`\x03Ta\x05\x94\x91\x90a6\xE5V[\x90P`\0a\x19\xCF\x82\x84\x88\x88\x88a\x1E\xB6V[\x90P`\0\x81`\x04Ta\x19\xE1\x91\x90a6\xE5V[\x90P`\0\x83`\x05Ta\x19\xF3\x91\x90a6\xE5V[\x90P\x83`\x05\x81\x90UP\x8A`\x03`\0\x82\x82Ta\x1A\x0E\x91\x90a6\xE5V[\x92PP\x81\x90UP\x81`\x04`\0\x82\x82Ta\x1A'\x91\x90a6\xE5V[\x90\x91UPP`\0T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x8D\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1A\xB6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a9A\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x1A\xCAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A\xEE\x91\x90a7\x1CV[P`\x01T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1ByW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a9A\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x1B\x8DW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B\xB1\x91\x90a7\x1CV[P`@\x80Q\x85\x81R` \x81\x01\x8D\x90R\x90\x81\x01\x83\x90R3\x90\x7FY\xC3\xA0\xB6\x0Cj\xB7\xDE\xB6.\x14@\xC9\xE7$A\xDBm\xB7\xDF\xE5\x14\xDB\xA8\xCB\x18\xE6\x0C\r\x89n\xFA\x90``\x01a\x07\xE2V[`\0`\x15TB\x10a\x1C\x04WP`\x12T\x90V[`\x12T`\x11T\x11a\x1C6W`\x14T`\x13Ta\x1C\x1F\x90Ba6\xE5V[a\x1C)\x91\x90a7\x91V[`\x11Ta\x17\xF1\x91\x90a6\xD2V[`\x14T`\x13Ta\x1CF\x90Ba6\xE5V[a\x1CP\x91\x90a7\x91V[`\x11Ta\x17\xF1\x91\x90a6\xE5V[`\0a\x17\xF1`\x03T`\x05Ta\x1Cpa\x19\x15V[a\x1Cxa\x17\xADV[a\x1C\x80a\x1B\xF2V[a\x1D\xF2V[`\0\x80`\0\x80a\x1C\x93a\x0B&V[\x92P\x92P\x92Pa\x1C\xAD`\x03T`\x04T`\x05T\x86\x86\x86a&\x07V[\x93PPPP\x90V[`\0\x80`\0\x80`\0a\x1C\xC5a\x0B&V[\x92P\x92P\x92P`\0a\x1C\xDA\x88\x88\x86\x86\x86a\x1EvV[\x90P`\0a\x1C\xEB\x82\x89\x87\x87\x87a\x1E\xB6V[`\x05\x83\x90U`\x03\x8A\x90U`\x04\x81\x81U`\0T`@Qc#\xB8r\xDD`\xE0\x1B\x81R\x92\x93P`\x01`\x01`\xA0\x1B\x03\x16\x91c#\xB8r\xDD\x91a\x1D-\x913\x910\x91\x8F\x91\x01a6\xF8V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1D\x85W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a9A\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x1D\x99W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1D\xBD\x91\x90a7\x1CV[P`\x01T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\x17\x0E\x903\x900\x90\x86\x90`\x04\x01a6\xF8V[`\0\x80a\x1D\xFF\x84\x84a&\xBFV[\x90P`\0a\x1E\r\x85\x85a&\xE5V[\x90P`\0a\x1E\x1B\x89\x89a'\x17V[\x90Pa\x1Ei\x87a\x1Ed\x84g\r\xE0\xB6\xB3\xA7d\0\0\x87a\x1EAa\x1E<\x88\x84a6\xE5V[a',V[a\x1EK\x91\x90a7\xECV[a\x1EU\x91\x90a8\x1CV[a\x1E_\x91\x90a8JV[a'\xC9V[a)rV[\x99\x98PPPPPPPPPV[`\0\x80a\x1E\x8Da\x1E\x88\x87\x87\x87\x87a)\x87V[a)\xE5V[a\x1E\x9F\x90g\r\xE0\xB6\xB3\xA7d\0\0a8JV[\x90Pa\x1E\xAB\x87\x82a*NV[\x97\x96PPPPPPPV[`\0\x80a\x1E\xC8a\x1E\x88\x87\x87\x87\x87a*cV[\x90Pa\x1E\xAB\x85a\x1Ed\x89\x84a)rV[`\0\x80a\x1E\x9F\x85a\x1Eda\x1E\x88\x89\x89\x89\x89a*cV[`\0\x80a\x1F\0a\x1E\x88\x87\x87\x87\x87a)\x87V[\x90Pa\x1E\xAB\x87a\x1Ed\x83g\r\xE0\xB6\xB3\xA7d\0\0a8JV[`\0\x80`\0\x80`\0a\x1F.\x8B\x8B\x8B\x8B\x8B\x8Ba$\xFEV[\x90P`\0a\x1F_\x82`@Q` \x01a\x1FF\x91\x90a7\xA8V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83`@\x01Qa%\\V[\x90P`\0\x81\x12\x15a\x1F\x83W\x89\x92Pa\x1F|\x83a'\x11a'\x10a*\x99V[\x93Pa\x1F\x96V[\x89\x93Pa\x1F\x93\x84`2`da*\xC7V[\x92P[P\x90\x9A\x91\x99P\x90\x97PPPPPPPPV[`\0\x80`\0\x80a\x1F\xB6a\x0B&V[\x92P\x92P\x92P`\0a\x1F\xCF`\x03T`\x05T\x86\x86\x86a\x1D\xF2V[\x90P\x86\x15a\"\x1CW`\0g\r\xE0\xB6\xB3\xA7d\0\0`\x02Tg\r\xE0\xB6\xB3\xA7d\0\0a\x1F\xF8\x91\x90a6\xE5V[a \x02\x90\x89a7\x91V[a \x0C\x91\x90a7}V[\x90P`\0a \x1D\x82\x84\x88\x88\x88a\x1EvV[\x90P`\x01a 7`\x03T`\x04T\x8B`\x05T\x86\x8C\x8C\x8Ca*\xE6V[a A\x91\x90a8JV[\x19\x96P\x80`\x05`\0\x82\x82Ta V\x91\x90a6\xD2V[\x92PP\x81\x90UP\x87`\x03`\0\x82\x82Ta o\x91\x90a6\xD2V[\x92PP\x81\x90UP\x86`\x04`\0\x82\x82Ta \x88\x91\x90a6\xE5V[\x90\x91UPP`\0T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a \xC1\x903\x900\x90\x8D\x90`\x04\x01a6\xF8V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a!\x19W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a9A\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a!-W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a!Q\x91\x90a7\x1CV[P`\x01T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x89\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a!\xDCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a9A\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a!\xF0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\"\x14\x91\x90a7\x1CV[PPPa$]V[`\0g\r\xE0\xB6\xB3\xA7d\0\0`\x02Tg\r\xE0\xB6\xB3\xA7d\0\0a\"=\x91\x90a6\xE5V[a\"G\x90\x89a7\x91V[a\"Q\x91\x90a7}V[\x90P`\0a\"b\x82\x84\x88\x88\x88a\x1E\xD8V[\x90P`\x01a\"|`\x03T`\x04T\x8B`\x05T\x86\x8C\x8C\x8Ca+gV[a\"\x86\x91\x90a8JV[\x19\x96P\x80`\x05`\0\x82\x82Ta\"\x9B\x91\x90a6\xD2V[\x92PP\x81\x90UP\x87`\x04`\0\x82\x82Ta\"\xB4\x91\x90a6\xD2V[\x92PP\x81\x90UP\x86`\x03`\0\x82\x82Ta\"\xCD\x91\x90a6\xE5V[\x90\x91UPP`\x01T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a#\x06\x903\x900\x90\x8D\x90`\x04\x01a6\xF8V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a#^W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a9A\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a#rW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a#\x96\x91\x90a7\x1CV[P`\0T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x89\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a$!W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a9A\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a$5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a$Y\x91\x90a7\x1CV[PPP[3`\x01`\x01`\xA0\x1B\x03\x16\x7F\x17fH\xF1\xF1\x1C\xDA(L\x12D\x90\x08k\xE4*\x92m\xDF\n\xE8\x87\xEB\xE7\xB1\xD6\xB37\xD8\x94'V\x88\x88\x88a$\x9C`\x03T`\x05T\x8B\x8B\x8Ba\x1D\xF2V[`@\x80Q\x94\x15\x15\x85R` \x85\x01\x93\x90\x93R\x91\x83\x01R``\x82\x01R`\x80\x01`@Q\x80\x91\x03\x90\xA2PPPP\x92\x91PPV[a$\xD3a\x1B\xF2V[`\x11UB`\x13UV[a$\xE4a\x19\x15V[`\x0CUB`\x0EUV[a$\xF5a\x17\xADV[`\x07UB`\tUV[a%7`@Q\x80`\xC0\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[\x95\x86R` \x86\x01\x94\x90\x94R`@\x85\x01\x92\x90\x92R``\x84\x01R`\x80\x83\x01R`\xA0\x82\x01R\x90V[`\0\x80\x83\x80` \x01\x90Q\x81\x01\x90a%s\x91\x90a8qV[\x90Pa%\x87\x81`\x80\x01Q\x82`\xA0\x01Qa&\xBFV[a%\xAAa%\xA5\x83` \x01Qa%\xA0\x87\x86``\x01Qa+\xB5V[a'\x17V[a+\xCAV[a%\xBBa%\xA5\x84`\0\x01Q\x87a'\x17V[a%\xC5\x91\x90a8\xFCV[a%\xCF\x91\x90a8\xFCV[\x94\x93PPPPV[`\0a%\xF0a%\xE6\x85\x84a+\xB5V[a\x1E<\x90\x85a7}V[a%\xFDa\x1E<\x86\x88a7}V[a\x18m\x91\x90a8\xFCV[`\0\x80`\0\x80a&\x1B\x8A\x8A\x8A\x8A\x8A\x8Aa$\xFEV[\x90P`\0a&3\x82`@Q` \x01a\x1FF\x91\x90a7\xA8V[\x90P\x80`\0\x03a&IW\x88\x94PPPPPa&\xB5V[`\0\x81\x12\x15a&iW\x88\x92Pa&b\x83`c`da*\xC7V[\x93Pa&|V[\x88\x93Pa&y\x84`e`da*\x99V[\x92P[a&\xAE\x82`@Q` \x01a&\x90\x91\x90a7\xA8V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x85\x85`\x01a\x01\0a%\\a+\xD8V[\x94PPPPP[\x96\x95PPPPPPV[`\0\x80a&\xCB\x83a,\xE9V[a&\xD9\x90c;\x9A\xCA\0a7\x91V[\x90Pa%\xCF\x84\x82a+\xB5V[`\0\x80a'\x03a&\xFD\x85g\x1B\xC1mgN\xC8\0\0a-\x8DV[\x84a+\xB5V[\x90Pa%\xCFg\x06\xF0[Y\xD3\xB2\0\0\x82a+\xB5V[`\0a\n\xAB\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a*\xC7V[`\0g\x06\xF0[Y\xD3\xB2\0\0\x82\x03a'EWP`\0\x91\x90PV[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x12a'mW`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a'\x8EW`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a'\x9B\x83`\x02a7\xECV[\x90P`\0a'\xA8\x82a-\xB9V[\x90P`\0a'\xBEg\x13\xA0K\xBD\xFD\xC9\xBE\x88\x83a07V[\x90Pa\x18m\x81a9$V[`\0h\x02H\xCE6\xA7\x0C\xB2k>\x19\x82\x13a'\xE4WP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a(+W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x01a\x08LV[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05`\x01`_\x1B\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1Dm\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1Dn\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[`\0a\n\xAB\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a*\x99V[`\0\x80a)\x94\x84\x84a&\xBFV[\x90P`\0a)\xA2\x87\x87a0LV[\x90P`\0a)\xB0\x86\x86a&\xE5V[\x90P\x82a)\xBD\x82\x84a8\xFCV[a)\xCF\x90g\r\xE0\xB6\xB3\xA7d\0\0a7\xECV[a)\xD9\x91\x90a8\x1CV[\x98\x97PPPPPPPPV[`\0\x80g\x13\xA0K\xBD\xFD\xC9\xBE\x88a*\x03g\r\xE0\xB6\xB3\xA7d\0\0\x85a7\xECV[a*\r\x91\x90a8\x1CV[\x90P`\0a*\x1A\x82a9$V[\x90P`\0a*'\x82a0`V[\x90Pg\x1B\xC1mgN\xC8\0\0a*Dg\r\xE0\xB6\xB3\xA7d\0\0\x83a7\xECV[a\x18m\x91\x90a8\x1CV[`\0a\n\xAB\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a*\x99V[`\0\x80a*p\x84\x84a&\xBFV[\x90P`\0a*~\x87\x87a0LV[\x90P`\0a*\x8C\x86\x86a&\xE5V[\x90P\x82a)\xBD\x82\x84a8JV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a*\xB1W`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a*\xDFW`\0\x80\xFD[\x04\x92\x91PPV[`\0\x80a*\xF3\x84\x84a&\xBFV[\x90P`\0a+\n\x86a+\x05\x89\x8Ba6\xD2V[a+\xB5V[\x90P`\0a+@a+-a\x1E<\x8C\x8Fa+#\x91\x90a6\xD2V[a%\xA0\x8C\x8Ea6\xD2V[a+6\x85a9$V[a\x1E\x88\x91\x90a8JV[\x90P\x8Aa+M\x83\x83a+\xB5V[a+W\x91\x90a8JV[\x9C\x9BPPPPPPPPPPPPV[`\0\x80a+t\x84\x84a&\xBFV[\x90P`\0a+\x86\x86a+\x05\x89\x8Ba6\xD2V[\x90P`\0a+\xA3a+-a\x1E<a+\x9D\x8D\x8Fa6\xD2V[\x85a'\x17V[\x90P\x8Ba+Ma+\xB3\x8A\x8Ca6\xD2V[\x83[`\0a\n\xAB\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a*\xC7V[`\0\x80a'\x9B\x83`\x02a7\xECV[`\0\x84\x86\x11\x15a,\x05W`@Qc0\x82\xDF\xDB`\xE1\x1B\x81R`\x04\x81\x01\x87\x90R`$\x81\x01\x86\x90R`D\x01a\x08LV[`\0a,\x15\x88\x88\x85c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a,'\x89\x88\x86c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a,5\x82\x84a7\xECV[\x13\x15a,^W`@Qc\x06\xF1\xBE]`\xE2\x1B\x81R`\x04\x81\x01\x83\x90R`$\x81\x01\x82\x90R`D\x01a\x08LV[`\0a,j\x89\x89a6\xE5V[\x90P`\0[`\x02a,{\x8A\x8Ca6\xD2V[a,\x85\x91\x90a7}V[\x94P`\0a,\x97\x8C\x87\x89c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a,\xA5\x86\x83a7\xECV[\x13a,\xB2W\x85\x99Pa,\xB9V[\x85\x9AP\x80\x94P[a,\xC3\x8B\x8Ba6\xE5V[\x92PP`\x01\x01\x87\x82\x11\x80\x15a,\xD7WP\x86\x81\x10[a,oWPPPP\x96\x95PPPPPPV[`\xB5\x81`\x01`\x88\x1B\x81\x10a-\x02W`@\x91\x90\x91\x1B\x90`\x80\x1C[i\x01\0\0\0\0\0\0\0\0\0\x81\x10a-\x1EW` \x91\x90\x91\x1B\x90`@\x1C[e\x01\0\0\0\0\0\x81\x10a-6W`\x10\x91\x90\x91\x1B\x90` \x1C[c\x01\0\0\0\x81\x10a-LW`\x08\x91\x90\x91\x1B\x90`\x10\x1C[b\x01\0\0\x01\x02`\x12\x1C\x80\x82\x04\x01`\x01\x90\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x90\x81\x90\x04\x81\x11\x90\x03\x90V[`\0a\n\xABg\r\xE0\xB6\xB3\xA7d\0\0\x83a-\xA5\x86a2DV[a-\xAF\x91\x90a7\xECV[a\x1E_\x91\x90a8\x1CV[`\0\x80\x82\x12\x80a-\xD0WPg\x1B\xC1mgN\xC8\0\0\x82\x13[\x15a-\xEEW`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a.\x0FW`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81g\x1B\xC1mgN\xC8\0\0\x03a.7W`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x15a.BW\x91\x90PV[`\0g\r\xE0\xB6\xB3\xA7d\0\0\x83\x12a.jWa.e\x83g\x1B\xC1mgN\xC8\0\0a8JV[a.lV[\x82[\x90P`\0a.\x82\x82g\x1B\xC1mgN\xC8\0\0a4\x1FV[\x90P\x80`\0\x03a.\xA5W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a.\xB0\x82a2DV[\x90P`\0c;\x9A\xCA\0a.\xDBa.\xD6a.\xD0g\x1B\xC1mgN\xC8\0\0a9$V[\x85a07V[a,\xE9V[a.\xE5\x91\x90a7\xECV[\x90P`\0\x80a.\xFC\x83g\x03\xC1f\\z\xAB \0a07V[a/\x0E\x90g \x05\xFEO&\x8E\xA0\0a8\xFCV[\x90P`\0a/>\x84a/'\x86f\x9F2u$b\xA0\0a07V[a/9\x90g\r\xC5R\x7Fd, \0a8\xFCV[a07V[a/P\x90g\r\xE0\xB6\xB3\xA7d\0\0a8\xFCV[\x90Pa/tg\t\xD0(\xCCo _\xFF\x19\x85a/j\x85\x85a4\x1FV[a/9\x91\x90a8JV[\x92PPP`\0[`\x02\x81\x10\x15a0\x0FW`\0\x86a/\x90\x84a0`V[a/\x9A\x91\x90a8JV[\x90P`\0a/\xA8\x84\x85a07V[a/\xB1\x90a9$V[\x90P`\0a/\xBE\x82a'\xC9V[\x90P`\0a/\xCC\x86\x85a07V[a/\xDEg\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x84a07V[a/\xE8\x91\x90a8JV[\x90Pa/\xF4\x84\x82a4\x1FV[a/\xFE\x90\x87a8\xFCV[\x95P\x84`\x01\x01\x94PPPPPa/{V[g\r\xE0\xB6\xB3\xA7d\0\0\x88\x12a0,Wa0'\x82a9$V[a)\xD9V[P\x96\x95PPPPPPV[`\0a\n\xAB\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a40V[`\0a\n\xABa0[\x84\x84a*NV[a2DV[`\0\x81`\0\x03a0yWPg\r\xE0\xB6\xB3\xA7d\0\0\x91\x90PV[gV\x98\xEE\xF0fp\0\0\x82\x12a0\x90WP`\0\x91\x90PV[a0\xA1gV\x98\xEE\xF0fp\0\0a9$V[\x82\x13a0\xB6WPg\x1B\xC1mgN\xC8\0\0\x91\x90PV[`\0a0\xC1\x83a4OV[\x90P`\0a0\xFAg\r\xE0\xB6\xB3\xA7d\0\0a0\xE3\x84g\x1B\xC1mgN\xC8\0\0a'\x17V[a0\xF5\x90g\r\xE0\xB6\xB3\xA7d\0\0a8\xFCV[a4\x1FV[\x90P`\0\x80\x82a1V\x81a1C\x81a11\x81a1\x1E\x81g\x02_\x0F\xE1\x05\xA3\x14\0a07V[a/9\x90g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19a8\xFCV[a/9\x90g\x14\xA8EL\x19\xE1\xAC\0a8\xFCV[a/9\x90g\x0F\xC1\x0E\x01W\x82w\xFF\x19a8\xFCV[a1h\x90g\x03\xDE\xBD\x08;\x8C|\0a8\xFCV[\x91P\x83\x90Pa1\xD0\x81a1\xBE\x81a1\xAC\x81a1\x9A\x81a1\x87\x81\x8Ba07V[a/9\x90g\x02\x95\xD4\0\xEA2W\xFF\x19a8\xFCV[a/9\x90g\x01W\xD8\xB2\xEC\xC7\x08\0a8\xFCV[a/9\x90g\x051\n\xA7\xD5!0\0a8\xFCV[a/9\x90g\r\xE0\xCC=\x15a\0\0a8\xFCV[\x91P\x81g\x11\x90\0\xAB\x10\x0F\xFC\0a1\xE6\x87\x88a07V[a1\xF2\x90`\0\x19a7\xECV[a1\xFC\x91\x90a8JV[a2\x06\x91\x90a8\xFCV[\x92PP`\0a2\x14\x83a'\xC9V[\x90P`\0a2\"\x85\x83a07V[\x90P`\0\x88\x12a22W\x80a)\xD9V[a)\xD9\x81g\x1B\xC1mgN\xC8\0\0a8JV[`\0\x80\x82\x13a2\x81W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x08LV[`\0``a2\x8E\x84a4\x8AV[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[`\0a\n\xAB\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x05\x85\x14\x17\x16a4HW`\0\x80\xFD[\x05\x92\x91PPV[`\0`\x01`\xFF\x1B\x82\x03a4uW`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x12\x15a4\x86WP\x19`\x01\x01\x90V[P\x90V[`\0\x80\x82\x11a4\xC7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x08LV[P`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x82\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`\0` \x82\x84\x03\x12\x15a5\x97Wa5\x97a52V[P5\x91\x90PV[\x80\x15\x15\x81\x14a5\xACW`\0\x80\xFD[PV[`\0\x80`\0``\x84\x86\x03\x12\x15a5\xC7Wa5\xC7a52V[\x835a5\xD2\x81a5\x9EV[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a5\xFDWa5\xFDa52V[\x825a6\x08\x81a5\x9EV[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`@\x83\x85\x03\x12\x15a6,Wa6,a52V[PP\x805\x92` \x90\x91\x015\x91PV[`\0` \x82\x84\x03\x12\x15a6PWa6Pa52V[\x815`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a6gW`\0\x80\xFD[\x93\x92PPPV[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a6\x9BW\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a6\x7FV[P`\0`@\x82\x86\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x92PPP\x92\x91PPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\n\xAEWa\n\xAEa6\xBCV[\x81\x81\x03\x81\x81\x11\x15a\n\xAEWa\n\xAEa6\xBCV[`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x81R\x91\x90\x92\x16` \x82\x01R`@\x81\x01\x91\x90\x91R``\x01\x90V[`\0` \x82\x84\x03\x12\x15a71Wa71a52V[\x81Qa6g\x81a5\x9EV[` \x80\x82R`\x11\x90\x82\x01Rp\x15\\\x19\x18]\x19H\x19[\x99\x08\x1C\x18\\\xDC\xD9Y`z\x1B`@\x82\x01R``\x01\x90V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a7\x8CWa7\x8Ca7gV[P\x04\x90V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\n\xAEWa\n\xAEa6\xBCV[`\0`\xC0\x82\x01\x90P\x82Q\x82R` \x83\x01Q` \x83\x01R`@\x83\x01Q`@\x83\x01R``\x83\x01Q``\x83\x01R`\x80\x83\x01Q`\x80\x83\x01R`\xA0\x83\x01Q`\xA0\x83\x01R\x92\x91PPV[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15a8\x08Wa8\x08a6\xBCV[\x81\x81\x05\x83\x14\x82\x15\x17a\n\xAEWa\n\xAEa6\xBCV[`\0\x82a8+Wa8+a7gV[`\x01`\xFF\x1B\x82\x14`\0\x19\x84\x14\x16\x15a8EWa8Ea6\xBCV[P\x05\x90V[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a8jWa8ja6\xBCV[P\x92\x91PPV[`\0`\xC0\x82\x84\x03\x12\x15a8\x86Wa8\x86a52V[`@Q`\xC0\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a8\xB7WcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x80`@RP\x82Q\x81R` \x83\x01Q` \x82\x01R`@\x83\x01Q`@\x82\x01R``\x83\x01Q``\x82\x01R`\x80\x83\x01Q`\x80\x82\x01R`\xA0\x83\x01Q`\xA0\x82\x01R\x80\x91PP\x92\x91PPV[\x80\x82\x01\x82\x81\x12`\0\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a9\x1CWa9\x1Ca6\xBCV[PP\x92\x91PPV[`\0`\x01`\xFF\x1B\x82\x01a99Wa99a6\xBCV[P`\0\x03\x90V\xFETarget contract does not contains\xEBa\xB7\x83\x9B\xB3\xC27\xA0\xB3\xAE\xAB\x18iQ\0\xBCQ.q9\xF0\xE3@e]\xD7\x02\xDA*\xA1";
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
        ///Calls the contract's `checkSwapConstant` (0xb9d95aca) function
        pub fn check_swap_constant(
            &self,
            new_liquidity: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([185, 217, 90, 202], new_liquidity)
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
        ///Calls the contract's `getNewLFromParameters` (0xf2bf4d54) function
        pub fn get_new_l_from_parameters(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([242, 191, 77, 84], ())
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
        ///Calls the contract's `getSwapFee` (0xd4cadf68) function
        pub fn get_swap_fee(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([212, 202, 223, 104], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getSwapUpperLower` (0x0681831a) function
        pub fn get_swap_upper_lower(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([6, 129, 131, 26], ())
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
        ///Calls the contract's `swapAmountIn` (0x1fdabc27) function
        pub fn swap_amount_in(
            &self,
            swap_direction: bool,
            amount_in: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([31, 218, 188, 39], (swap_direction, amount_in))
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
    ///Container type for all input parameters for the `checkSwapConstant` function with signature `checkSwapConstant(uint256)` and selector `0xb9d95aca`
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
    #[ethcall(name = "checkSwapConstant", abi = "checkSwapConstant(uint256)")]
    pub struct CheckSwapConstantCall {
        pub new_liquidity: ::ethers::core::types::U256,
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
    ///Container type for all input parameters for the `getNewLFromParameters` function with signature `getNewLFromParameters()` and selector `0xf2bf4d54`
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
    #[ethcall(name = "getNewLFromParameters", abi = "getNewLFromParameters()")]
    pub struct GetNewLFromParametersCall;
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
    ///Container type for all input parameters for the `getSwapUpperLower` function with signature `getSwapUpperLower()` and selector `0x0681831a`
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
    #[ethcall(name = "getSwapUpperLower", abi = "getSwapUpperLower()")]
    pub struct GetSwapUpperLowerCall;
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
    ///Container type for all input parameters for the `swapAmountIn` function with signature `swapAmountIn(bool,uint256)` and selector `0x1fdabc27`
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
    #[ethcall(name = "swapAmountIn", abi = "swapAmountIn(bool,uint256)")]
    pub struct SwapAmountInCall {
        pub swap_direction: bool,
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
        CheckSwapConstant(CheckSwapConstantCall),
        GetInvariant(GetInvariantCall),
        GetLiquidity(GetLiquidityCall),
        GetNewLFromParameters(GetNewLFromParametersCall),
        GetParams(GetParamsCall),
        GetPortfolioValue(GetPortfolioValueCall),
        GetReserveX(GetReserveXCall),
        GetReserveY(GetReserveYCall),
        GetSpotPrice(GetSpotPriceCall),
        GetStrategyData(GetStrategyDataCall),
        GetSwapFee(GetSwapFeeCall),
        GetSwapUpperLower(GetSwapUpperLowerCall),
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
            if let Ok(decoded) = <CheckSwapConstantCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CheckSwapConstant(decoded));
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
            if let Ok(decoded) = <GetNewLFromParametersCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetNewLFromParameters(decoded));
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
            if let Ok(decoded) = <GetSwapFeeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetSwapFee(decoded));
            }
            if let Ok(decoded) = <GetSwapUpperLowerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetSwapUpperLower(decoded));
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
                Self::CheckSwapConstant(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetInvariant(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetLiquidity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetNewLFromParameters(element) => {
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
                Self::GetSwapFee(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetSwapUpperLower(element) => {
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
                Self::CheckSwapConstant(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetInvariant(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetLiquidity(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetNewLFromParameters(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetParams(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPortfolioValue(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetReserveX(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetReserveY(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSpotPrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetStrategyData(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSwapFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSwapUpperLower(element) => ::core::fmt::Display::fmt(element, f),
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
    impl ::core::convert::From<CheckSwapConstantCall> for RMMCalls {
        fn from(value: CheckSwapConstantCall) -> Self {
            Self::CheckSwapConstant(value)
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
    impl ::core::convert::From<GetNewLFromParametersCall> for RMMCalls {
        fn from(value: GetNewLFromParametersCall) -> Self {
            Self::GetNewLFromParameters(value)
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
    impl ::core::convert::From<GetSwapFeeCall> for RMMCalls {
        fn from(value: GetSwapFeeCall) -> Self {
            Self::GetSwapFee(value)
        }
    }
    impl ::core::convert::From<GetSwapUpperLowerCall> for RMMCalls {
        fn from(value: GetSwapUpperLowerCall) -> Self {
            Self::GetSwapUpperLower(value)
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
    ///Container type for all return fields from the `checkSwapConstant` function with signature `checkSwapConstant(uint256)` and selector `0xb9d95aca`
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
    pub struct CheckSwapConstantReturn(pub ::ethers::core::types::I256);
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
    ///Container type for all return fields from the `getNewLFromParameters` function with signature `getNewLFromParameters()` and selector `0xf2bf4d54`
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
    pub struct GetNewLFromParametersReturn(pub ::ethers::core::types::U256);
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
    ///Container type for all return fields from the `getSwapUpperLower` function with signature `getSwapUpperLower()` and selector `0x0681831a`
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
    pub struct GetSwapUpperLowerReturn(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
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
    ///Container type for all return fields from the `swapAmountIn` function with signature `swapAmountIn(bool,uint256)` and selector `0x1fdabc27`
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
