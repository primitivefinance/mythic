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
                            ::std::borrow::ToOwned::to_owned("contract ERC20"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("tokenY_"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract ERC20"),
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
                                    name: ::std::borrow::ToOwned::to_owned("amountY"),
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
    const __BYTECODE: &[u8] = b"`\xE0`@R4\x80\x15b\0\0^W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`@Qb\x007\xE08\x03\x80b\x007\xE0\x839\x81\x01`@\x81\x90Rb\0\0\x81\x91b\0\x01<V[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x80\x89\x16`\x01`\x01`\xA0\x1B\x03\x19\x92\x83\x16\x17\x90\x92U`\x01\x80T\x92\x88\x16\x92\x90\x91\x16\x91\x90\x91\x17\x90U`\x80\x84\x90R`\xA0\x83\x90R`\xC0\x82\x90Rg\r\xE0\xB6\xB3\xA7d\0\0\x81\x10b\0\x01\x10W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01Rp\nn\xEC.\x04\x0C\xCC\xAC\xA4\x0E\x8D\xED\xE4\r\r,\xED`{\x1B`D\x82\x01R`d\x01`@Q\x80\x91\x03\x90\xFD[`\x02UPb\0\x01\xE4\x93PPPPV[\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x017W`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15b\0\x01\xA1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[b\0\x01\xAC\x87b\0\x01\x1FV[\x95Pb\0\x01\xBC` \x88\x01b\0\x01\x1FV[\x94P`@\x87\x01Q\x93P``\x87\x01Q\x92P`\x80\x87\x01Q\x91P`\xA0\x87\x01Q\x90P\x92\x95P\x92\x95P\x92\x95V[`\x80Q`\xA0Q`\xC0Qa4\x13b\0\x03\xCD`\09`\0\x81\x81a\x04>\x01R\x81\x81a\x04\xE6\x01R\x81\x81a\x0C5\x01R\x81\x81a\x10W\x01R\x81\x81a\x13\x98\x01R\x81\x81a\x16\xF7\x01R\x81\x81a\x17\x9C\x01R\x81\x81a\x1DR\x01Ra M\x01R`\0\x81\x81a\x04\x04\x01R\x81\x81a\x04\xA4\x01R\x81\x81a\x05\"\x01R\x81\x81a\x05r\x01R\x81\x81a\x08Z\x01R\x81\x81a\x08\xA8\x01R\x81\x81a\x08\xFE\x01R\x81\x81a\tL\x01R\x81\x81a\x0B\xF3\x01R\x81\x81a\x0C\x82\x01R\x81\x81a\r\t\x01R\x81\x81a\rY\x01R\x81\x81a\x10\x15\x01R\x81\x81a\x10\x9B\x01R\x81\x81a\x13V\x01R\x81\x81a\x13\xC7\x01R\x81\x81a\x14\x17\x01R\x81\x81a\x14q\x01R\x81\x81a\x16\xB5\x01R\x81\x81a\x17Z\x01R\x81\x81a\x17\xE0\x01R\x81\x81a\x1AX\x01R\x81\x81a\x1A\xA8\x01R\x81\x81a\x1C\xDC\x01R\x81\x81a\x1D\x10\x01R\x81\x81a\x1D\x96\x01R\x81\x81a \x0B\x01R\x81\x81a }\x01Ra \xCD\x01R`\0\x81\x81a\x03\xBA\x01R\x81\x81a\x04\xC5\x01R\x81\x81a\x05C\x01R\x81\x81a\x05\x93\x01R\x81\x81a\x08{\x01R\x81\x81a\x08\xC9\x01R\x81\x81a\t\x1F\x01R\x81\x81a\tm\x01R\x81\x81a\x0C\x14\x01R\x81\x81a\x0C\xA3\x01R\x81\x81a\r*\x01R\x81\x81a\rz\x01R\x81\x81a\x106\x01R\x81\x81a\x10\xBC\x01R\x81\x81a\x13w\x01R\x81\x81a\x13\xE8\x01R\x81\x81a\x148\x01R\x81\x81a\x14\x92\x01R\x81\x81a\x16\xD6\x01R\x81\x81a\x17{\x01R\x81\x81a\x18\x01\x01R\x81\x81a\x1Ay\x01R\x81\x81a\x1A\xC9\x01R\x81\x81a\x1D1\x01R\x81\x81a\x1D\xB7\x01R\x81\x81a ,\x01R\x81\x81a \x9E\x01Ra \xEE\x01Ra4\x13`\0\xF3\xFE`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x01\xF6W`\x005`\xE0\x1C\x80c\x9C\x9D\xA9\xEA\x11a\x01FW\x80c\xCC\xD1\xE4\xBE\x11a\0\xE4W\x80c\xDCv\xFA\xBC\x11a\0\xBEW\x80c\xDCv\xFA\xBC\x14a\x04hW\x80c\xF9\xA1\xC8Z\x14a\x04pW\x80c\xFA\xDF\xA6[\x14a\x04\x83W\x80c\xFE\xD3\xDF\xDA\x14a\x04\x8CWa\x01\xF6V[\x80c\xCC\xD1\xE4\xBE\x14a\x04&W\x80c\xCF\xC4\xAFU\x14a\x049W\x80c\xD4\xCA\xDFh\x14a\x04`Wa\x01\xF6V[\x80c\xB7\xD1\x9F\xC4\x11a\x01 W\x80c\xB7\xD1\x9F\xC4\x14a\x03\xDCW\x80c\xBB\x04\x98\xDE\x14a\x03\xEFW\x80c\xC0\xFF\x1A\x15\x14a\x03\xF7W\x80c\xC5)\x87\xCF\x14a\x03\xFFWa\x01\xF6V[\x80c\x9C\x9D\xA9\xEA\x14a\x03\x8FW\x80c\xA5\x9C\x18o\x14a\x03\xA2W\x80c\xAF\xDF1\xCD\x14a\x03\xB5Wa\x01\xF6V[\x80cT\xCF*\xEB\x11a\x01\xB3W\x80cvp\x166\x11a\x01\x8DW\x80cvp\x166\x14a\x03GW\x80c}iS~\x14a\x03ZW\x80c\x87kU\xF1\x14a\x03mW\x80c\x94\xB9\x18\xDE\x14a\x03|Wa\x01\xF6V[\x80cT\xCF*\xEB\x14a\x03\x16W\x80cU\x9D\x16\x02\x14a\x03\x1FW\x80cp\xA0\x821\x14a\x03'Wa\x01\xF6V[\x80c\x02\xC2\xE5]\x14a\x02[W\x80c\x04\xAF\xA8\"\x14a\x02\x88W\x80c\x08\xEA\xBD\xDA\x14a\x02\xB6W\x80c\x15w\x0F\x92\x14a\x02\xCDW\x80c\x16\xDC\x16[\x14a\x02\xD6W\x80c4\xE1\x99\x07\x14a\x03\x01W[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x02na\x02i6`\x04a1.V[a\x04\x94V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01[`@Q\x80\x91\x03\x90\xF3[a\x02\x9Ba\x02\x966`\x04a1[V[a\x07\xEEV[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x02\x7FV[a\x02\xBF`\x03T\x81V[`@Q\x90\x81R` \x01a\x02\x7FV[a\x02\xBF`\x05T\x81V[`\0Ta\x02\xE9\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\x7FV[a\x03\x14a\x03\x0F6`\x04a1.V[a\x0BEV[\0[a\x02\xBF`\x02T\x81V[`\x03Ta\x02\xBFV[a\x02\xBFa\x0356`\x04a1\x93V[`\x06` R`\0\x90\x81R`@\x90 T\x81V[a\x02\x9Ba\x03U6`\x04a1\xC6V[a\x0B\x99V[a\x02na\x03h6`\x04a1.V[a\x10\x05V[```@Qa\x02\x7F\x91\x90a1\xF5V[a\x02\xBFa\x03\x8A6`\x04a1.V[a\x13\x03V[a\x02na\x03\x9D6`\x04a1.V[a\x17JV[a\x02na\x03\xB06`\x04a2CV[a\x1ALV[a\x02\xBF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\x01Ta\x02\xE9\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x02\xBFa\x1C\x90V[a\x02\xBFa\x1C\xCCV[a\x02\xBF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02na\x0446`\x04a1.V[a\x1D\0V[a\x02\xBF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\x02Ta\x02\xBFV[a\x02\xBFa\x1F\xFEV[a\x02na\x04~6`\x04a2CV[a qV[a\x02\xBF`\x04T\x81V[`\x04Ta\x02\xBFV[`\0\x80`\0a\x05\n`\x03T`\x05T\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\"\x19V[\x90P`\0a\x05g\x85`\x03Ta\x05\x1F\x91\x90a2~V[\x83\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\"\xC3V[\x90P`\0a\x05\xB7\x82\x84\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a#OV[\x90P`\0`\x04T\x82a\x05\xC9\x91\x90a2\x91V[\x90P`\0`\x05T\x84a\x05\xDB\x91\x90a2\x91V[\x90P\x83`\x05\x81\x90UP\x87`\x03`\0\x82\x82Ta\x05\xF6\x91\x90a2~V[\x92PP\x81\x90UP\x81`\x04`\0\x82\x82Ta\x06\x0F\x91\x90a2~V[\x90\x91UPP`\0T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\x06H\x903\x900\x90\x8D\x90`\x04\x01a2\xA4V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x06\xA0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a3\xF3\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x06\xB4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xD8\x91\x90a2\xC8V[P`\x01T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\x07\r\x903\x900\x90\x87\x90`\x04\x01a2\xA4V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x07eW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a3\xF3\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x07yW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x9D\x91\x90a2\xC8V[P`@\x80Q\x85\x81R` \x81\x01\x8A\x90R\x90\x81\x01\x83\x90R3\x90\x7F\xBE\xB3\x88W\x86\xD67\xA4t\xCB\xC2\x87\xC0\xA4E\x87#\x163\xA0w\xF0\xBD05MZK\x18\x99o\xCE\x90``\x01[`@Q\x80\x91\x03\x90\xA2\x97\x90\x96P\x94PPPPPV[`\0\x80`\0`\x05T`\0\x14a\x08JW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FPool already initialized\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[\x85\x15a\x08\xF4W\x84\x92Pa\x08\x9F\x83\x85\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\"\xC3V[\x90Pa\x08\xED\x81\x85\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a#OV[\x91Pa\t\x94V[\x84\x91Pa\tC\x82\x85\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a#\xDBV[\x90Pa\t\x91\x81\x85\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a$@V[\x92P[`\x05\x81\x90U`\x03\x83\x90U`\x04\x82\x81U3`\0\x81\x81R`\x06` R`@\x80\x82 \x85\x90U\x90T\x90Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x92c#\xB8r\xDD\x92a\t\xE6\x92\x90\x910\x91\x89\x91\x01a2\xA4V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\n>W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a3\xF3\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\nRW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\nv\x91\x90a2\xC8V[P`\x01T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\n\xAB\x903\x900\x90\x87\x90`\x04\x01a2\xA4V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0B\x03W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a3\xF3\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x0B\x17W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B;\x91\x90a2\xC8V[P\x93P\x93P\x93\x90PV[g\r\xE0\xB6\xB3\xA7d\0\0\x81\x10a\x0B\x94W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rt\t\xCC\xAE\xE4\x0En\xEC.\x04\x0C\xCC\xAC\xA4\x0E\x8D\xED\xE4\r\r,\xED`[\x1B`D\x82\x01R`d\x01a\x08AV[`\x02UV[`\0\x80`\0\x80`\x05T\x11a\x0B\xE6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rs\x14\x1B\xDB\xDB\x08\x1B\x9B\xDD\x08\x1A[\x9A]\x1AX[\x1A^\x99Y`b\x1B`D\x82\x01R`d\x01a\x08AV[`\0a\x0CY`\x03T`\x05T\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\"\x19V[\x90P\x85\x15a\x0C\xF0W\x84\x93P`\0a\x0Cw\x85`\x03Ta\x05\x1F\x91\x90a2~V[\x90P`\0a\x0C\xC7\x82\x84\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a#OV[\x90P`\x04T\x81a\x0C\xD7\x91\x90a2\x91V[\x94P`\x05T\x82a\x0C\xE7\x91\x90a2\x91V[\x93PPPa\r\xC3V[\x84\x92P`\0a\rN\x84`\x04Ta\r\x06\x91\x90a2~V[\x83\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a#\xDBV[\x90P`\0a\r\x9E\x82\x84\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a$@V[\x90P`\x03T\x81a\r\xAE\x91\x90a2\x91V[\x95P`\x05T\x82a\r\xBE\x91\x90a2\x91V[\x93PPP[\x81`\x05`\0\x82\x82Ta\r\xD5\x91\x90a2~V[\x92PP\x81\x90UP\x83`\x03`\0\x82\x82Ta\r\xEE\x91\x90a2~V[\x92PP\x81\x90UP\x82`\x04`\0\x82\x82Ta\x0E\x07\x91\x90a2~V[\x90\x91UPP3`\0\x90\x81R`\x06` R`@\x81 \x80T\x84\x92\x90a\x0E+\x90\x84\x90a2~V[\x90\x91UPP`\0T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\x0Ed\x903\x900\x90\x89\x90`\x04\x01a2\xA4V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0E\xBCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a3\xF3\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x0E\xD0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\xF4\x91\x90a2\xC8V[P`\x01T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\x0F)\x903\x900\x90\x88\x90`\x04\x01a2\xA4V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0F\x81W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a3\xF3\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x0F\x95W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xB9\x91\x90a2\xC8V[P`@\x80Q\x83\x81R` \x81\x01\x86\x90R\x90\x81\x01\x84\x90R3\x90\x7F\xBE\xB3\x88W\x86\xD67\xA4t\xCB\xC2\x87\xC0\xA4E\x87#\x163\xA0w\xF0\xBD05MZK\x18\x99o\xCE\x90``\x01`@Q\x80\x91\x03\x90\xA2P\x92P\x92P\x92V[`\0\x80`\0a\x10{`\x03T`\x05T\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\"\x19V[\x90P`\0a\x10\x90\x85`\x03Ta\r\x06\x91\x90a2\x91V[\x90P`\0a\x10\xE0\x82\x84\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a$@V[\x90P`\0\x81`\x03Ta\x10\xF2\x91\x90a2\x91V[\x90P`\0\x83`\x05Ta\x11\x04\x91\x90a2\x91V[\x90P\x83`\x05\x81\x90UP\x81`\x03`\0\x82\x82Ta\x11\x1F\x91\x90a2\x91V[\x92PP\x81\x90UP\x87`\x04`\0\x82\x82Ta\x118\x91\x90a2\x91V[\x90\x91UPP`\0T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x11\xC7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a3\xF3\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x11\xDBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\xFF\x91\x90a2\xC8V[P`\x01T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x8A\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x12\x8AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a3\xF3\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x12\x9EW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\xC2\x91\x90a2\xC8V[P`@\x80Q\x85\x81R` \x81\x01\x84\x90R\x90\x81\x01\x89\x90R3\x90\x7FY\xC3\xA0\xB6\x0Cj\xB7\xDE\xB6.\x14@\xC9\xE7$A\xDBm\xB7\xDF\xE5\x14\xDB\xA8\xCB\x18\xE6\x0C\r\x89n\xFA\x90``\x01a\x07\xDAV[`\0\x80g\r\xE0\xB6\xB3\xA7d\0\0`\x02Tg\r\xE0\xB6\xB3\xA7d\0\0a\x13%\x91\x90a2\x91V[a\x13/\x90\x85a2\xE8V[a\x139\x91\x90a3\x15V[\x90P`\0a\x13G\x82\x85a2\x91V[\x90P`\0a\x13\xBC`\x03T`\x05T\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\"\x19V[\x90P`\0a\x14\x0C\x83\x83\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\"\xC3V[\x90P`\0a\x14\\\x82\x84\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a#OV[\x90P`\x01a\x14\xB6`\x03T\x89`\x04T\x85`\x05T\x88\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a$\x9CV[a\x14\xC0\x91\x90a3)V[\x19\x95P\x86`\x03`\0\x82\x82Ta\x14\xD5\x91\x90a2~V[\x92PP\x81\x90UP\x85`\x04`\0\x82\x82Ta\x14\xEE\x91\x90a2\x91V[\x90\x91UPP`\0T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\x15'\x903\x900\x90\x8C\x90`\x04\x01a2\xA4V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x15\x7FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a3\xF3\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x15\x93W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\xB7\x91\x90a2\xC8V[P`\x01T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x88\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x16BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a3\xF3\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x16VW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16z\x91\x90a2\xC8V[P3`\x01`\x01`\xA0\x1B\x03\x16\x7F\x17fH\xF1\xF1\x1C\xDA(L\x12D\x90\x08k\xE4*\x92m\xDF\n\xE8\x87\xEB\xE7\xB1\xD6\xB37\xD8\x94'V`\x01\x89\x89a\x17\x1B`\x03T`\x05T\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\"\x19V[`@\x80Q\x94\x15\x15\x85R` \x85\x01\x93\x90\x93R\x91\x83\x01R``\x82\x01R`\x80\x01`@Q\x80\x91\x03\x90\xA2PPPPP\x91\x90PV[`\0\x80`\0a\x17\xC0`\x03T`\x05T\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\"\x19V[\x90P`\0a\x17\xD5\x85`\x04Ta\r\x06\x91\x90a2~V[\x90P`\0a\x18%\x82\x84\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a$@V[\x90P`\0`\x03T\x82a\x187\x91\x90a2\x91V[\x90P`\0`\x05T\x84a\x18I\x91\x90a2\x91V[\x90P\x83`\x05\x81\x90UP\x81`\x03`\0\x82\x82Ta\x18d\x91\x90a2~V[\x92PP\x81\x90UP\x87`\x04`\0\x82\x82Ta\x18}\x91\x90a2~V[\x90\x91UPP`\0T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\x18\xB6\x903\x900\x90\x87\x90`\x04\x01a2\xA4V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x19\x0EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a3\xF3\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x19\"W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19F\x91\x90a2\xC8V[P`\x01T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\x19{\x903\x900\x90\x8D\x90`\x04\x01a2\xA4V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x19\xD3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a3\xF3\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x19\xE7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A\x0B\x91\x90a2\xC8V[P`@\x80Q\x85\x81R` \x81\x01\x84\x90R\x90\x81\x01\x89\x90R3\x90\x7F\xBE\xB3\x88W\x86\xD67\xA4t\xCB\xC2\x87\xC0\xA4E\x87#\x163\xA0w\xF0\xBD05MZK\x18\x99o\xCE\x90``\x01a\x07\xDAV[`\0\x80`\0a\x1A\x9D\x85\x85\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a#\xDBV[\x90P`\0a\x1A\xED\x82\x86\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a$@V[`\x05\x83\x90U`\x03\x81\x90U`\x04\x87\x81U`\0T`@Qc#\xB8r\xDD`\xE0\x1B\x81R\x92\x93P`\x01`\x01`\xA0\x1B\x03\x16\x91c#\xB8r\xDD\x91a\x1B/\x913\x910\x91\x87\x91\x01a2\xA4V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1B\x87W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a3\xF3\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x1B\x9BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B\xBF\x91\x90a2\xC8V[P`\x01T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\x1B\xF4\x903\x900\x90\x8B\x90`\x04\x01a2\xA4V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1CLW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a3\xF3\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x1C`W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C\x84\x91\x90a2\xC8V[P\x90\x95\x90\x94P\x92PPPV[`\0`\x04Tg\r\xE0\xB6\xB3\xA7d\0\0a\x1C\xA6a\x1F\xFEV[`\x03Ta\x1C\xB3\x91\x90a2\xE8V[a\x1C\xBD\x91\x90a3\x15V[a\x1C\xC7\x91\x90a2~V[\x90P\x90V[`\0a\x1C\xC7`\x03T`\x05T`\x04T\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a%\x17V[`\0\x80`\0a\x1Dv`\x03T`\x05T\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\"\x19V[\x90P`\0a\x1D\x8B\x85`\x03Ta\x05\x1F\x91\x90a2\x91V[\x90P`\0a\x1D\xDB\x82\x84\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a#OV[\x90P`\0\x81`\x04Ta\x1D\xED\x91\x90a2\x91V[\x90P`\0\x83`\x05Ta\x1D\xFF\x91\x90a2\x91V[\x90P\x83`\x05\x81\x90UP\x87`\x03`\0\x82\x82Ta\x1E\x1A\x91\x90a2\x91V[\x92PP\x81\x90UP\x81`\x04`\0\x82\x82Ta\x1E3\x91\x90a2\x91V[\x90\x91UPP`\0T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x8A\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1E\xC2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a3\xF3\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x1E\xD6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E\xFA\x91\x90a2\xC8V[P`\x01T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1F\x85W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a3\xF3\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x1F\x99W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1F\xBD\x91\x90a2\xC8V[P`@\x80Q\x85\x81R` \x81\x01\x8A\x90R\x90\x81\x01\x83\x90R3\x90\x7FY\xC3\xA0\xB6\x0Cj\xB7\xDE\xB6.\x14@\xC9\xE7$A\xDBm\xB7\xDF\xE5\x14\xDB\xA8\xCB\x18\xE6\x0C\r\x89n\xFA\x90``\x01a\x07\xDAV[`\0a\x1C\xC7`\x03T`\x05T\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\"\x19V[`\0\x80`\0a \xC2\x85\x85\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\"\xC3V[\x90P`\0a!\x12\x82\x86\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a#OV[`\x05\x83\x90U`\x03\x87\x90U`\x04\x81\x81U`\0T`@Qc#\xB8r\xDD`\xE0\x1B\x81R\x92\x93P`\x01`\x01`\xA0\x1B\x03\x16\x91c#\xB8r\xDD\x91a!T\x913\x910\x91\x8C\x91\x01a2\xA4V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a!\xACW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a3\xF3\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a!\xC0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a!\xE4\x91\x90a2\xC8V[P`\x01T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\x1B\xF4\x903\x900\x90\x86\x90`\x04\x01a2\xA4V[`\0\x80a\"Hg\x06\xF0[Y\xD3\xB2\0\0a\"Ca\"=\x87g\x1B\xC1mgN\xC8\0\0a%PV[\x86a%\x85V[a%\x85V[\x90P`\0a\"Y\x85a\"C\x86a%\x9AV[a\"g\x90c;\x9A\xCA\0a2\xE8V[\x90P`\0a\"u\x89\x89a&>V[\x90Pa\"\xB6\x87a\"\xB1\x85a\"\xA2a\"\x9Ca\"\x97\x87g\r\xE0\xB6\xB3\xA7d\0\0a2\x91V[a&SV[\x87a%\x85V[a\"\xAC\x91\x90a2\x91V[a&\xF0V[a(\x99V[\x99\x98PPPPPPPPPV[`\0\x80a\"\xD8a\"\xD3\x86\x86a(\xAEV[a(\xC3V[\x90P`\0a\"\xFAg\x06\xF0[Y\xD3\xB2\0\0a\"\xB1\x86g\x1B\xC1mgN\xC8\0\0a%PV[\x90P`\0a#-\x85a#\x0C\x84\x86a3PV[a#\x1E\x90g\r\xE0\xB6\xB3\xA7d\0\0a3xV[a#(\x91\x90a3\xA8V[a*\x9EV[\x90P`\0a#C\x82g\r\xE0\xB6\xB3\xA7d\0\0a3)V[\x90Pa\"\xB6\x89\x82a(\xAEV[`\0\x80a#_a\"\xD3\x86\x86a(\xAEV[\x90P`\0a#\x81g\x06\xF0[Y\xD3\xB2\0\0a\"\xB1\x86g\x1B\xC1mgN\xC8\0\0a%PV[\x90P`\0a#\x8F\x82\x84a3)V[\x90P`\0\x85a#\xA6\x83g\r\xE0\xB6\xB3\xA7d\0\0a3xV[a#\xB0\x91\x90a3\xA8V[\x90P`\0a#\xBD\x82a*\x9EV[\x90Pa#\xCD\x88a\"\xB1\x8C\x84a(\x99V[\x9A\x99PPPPPPPPPPV[`\0\x80a#\xEBa\"\xD3\x86\x86a(\xAEV[\x90P`\0a$\rg\x06\xF0[Y\xD3\xB2\0\0a\"\xB1\x86g\x1B\xC1mgN\xC8\0\0a%PV[\x90P`\0a$\x1F\x85a#\x0C\x84\x86a3)V[\x90Pa$4\x88a$/\x88\x84a(\x99V[a(\xAEV[\x98\x97PPPPPPPPV[`\0\x80a$Pa\"\xD3\x86\x86a(\xAEV[\x90P`\0a$rg\x06\xF0[Y\xD3\xB2\0\0a\"\xB1\x86g\x1B\xC1mgN\xC8\0\0a%PV[\x90P`\0a$\x84\x85a#\x0C\x84\x86a3PV[\x90Pa$4\x88a\"\xB1\x83g\r\xE0\xB6\xB3\xA7d\0\0a3)V[`\0\x80a$\xAD\x84a\"C\x87\x89a2~V[\x90P`\0a$\xE6a$\xD3a\"\x97a$\xC4\x8D\x8Fa2~V[a$\xCE\x8A\x8Ca2~V[a&>V[a$\xDC\x86a3\xD6V[a#(\x91\x90a3)V[\x90P\x87\x89a$\xF4\x84\x84a%\x85V[a$\xFE\x91\x90a3)V[a%\x08\x91\x90a3)V[\x9B\x9APPPPPPPPPPPV[`\0a%0a%&\x85\x84a%\x85V[a\"\x97\x90\x85a3\x15V[a%=a\"\x97\x86\x88a3\x15V[a%G\x91\x90a3PV[\x95\x94PPPPPV[`\0a%|g\r\xE0\xB6\xB3\xA7d\0\0\x83a%h\x86a(\xC3V[a%r\x91\x90a3xV[a\"\xAC\x91\x90a3\xA8V[\x90P[\x92\x91PPV[`\0a%|\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a+\x07V[`\xB5\x81`\x01`\x88\x1B\x81\x10a%\xB3W`@\x91\x90\x91\x1B\x90`\x80\x1C[i\x01\0\0\0\0\0\0\0\0\0\x81\x10a%\xCFW` \x91\x90\x91\x1B\x90`@\x1C[e\x01\0\0\0\0\0\x81\x10a%\xE7W`\x10\x91\x90\x91\x1B\x90` \x1C[c\x01\0\0\0\x81\x10a%\xFDW`\x08\x91\x90\x91\x1B\x90`\x10\x1C[b\x01\0\0\x01\x02`\x12\x1C\x80\x82\x04\x01`\x01\x90\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x90\x81\x90\x04\x81\x11\x90\x03\x90V[`\0a%|\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a+\x07V[`\0g\x06\xF0[Y\xD3\xB2\0\0\x82\x03a&lWP`\0\x91\x90PV[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x12a&\x94W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a&\xB5W`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a&\xC2\x83`\x02a3xV[\x90P`\0a&\xCF\x82a+&V[\x90P`\0a&\xE5g\x13\xA0K\xBD\xFD\xC9\xBE\x88\x83a-\xA4V[\x90Pa%G\x81a3\xD6V[`\0h\x02H\xCE6\xA7\x0C\xB2k>\x19\x82\x13a'\x0BWP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a'RW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x01a\x08AV[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05`\x01`_\x1B\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1Dm\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1Dn\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[`\0a%|\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a-\xB9V[`\0a%|\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a-\xB9V[`\0\x80\x82\x13a)\0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x08AV[`\0``a)\r\x84a-\xE7V[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[`\0\x80g\x13\xA0K\xBD\xFD\xC9\xBE\x88a*\xBCg\r\xE0\xB6\xB3\xA7d\0\0\x85a3xV[a*\xC6\x91\x90a3\xA8V[\x90P`\0a*\xD3\x82a3\xD6V[\x90P`\0a*\xE0\x82a.\x8FV[\x90Pg\x1B\xC1mgN\xC8\0\0a*\xFDg\r\xE0\xB6\xB3\xA7d\0\0\x83a3xV[a%G\x91\x90a3\xA8V[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a+\x1FW`\0\x80\xFD[\x04\x92\x91PPV[`\0\x80\x82\x12\x80a+=WPg\x1B\xC1mgN\xC8\0\0\x82\x13[\x15a+[W`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a+|W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81g\x1B\xC1mgN\xC8\0\0\x03a+\xA4W`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x15a+\xAFW\x91\x90PV[`\0g\r\xE0\xB6\xB3\xA7d\0\0\x83\x12a+\xD7Wa+\xD2\x83g\x1B\xC1mgN\xC8\0\0a3)V[a+\xD9V[\x82[\x90P`\0a+\xEF\x82g\x1B\xC1mgN\xC8\0\0a0sV[\x90P\x80`\0\x03a,\x12W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a,\x1D\x82a(\xC3V[\x90P`\0c;\x9A\xCA\0a,Ha,Ca,=g\x1B\xC1mgN\xC8\0\0a3\xD6V[\x85a-\xA4V[a%\x9AV[a,R\x91\x90a3xV[\x90P`\0\x80a,i\x83g\x03\xC1f\\z\xAB \0a-\xA4V[a,{\x90g \x05\xFEO&\x8E\xA0\0a3PV[\x90P`\0a,\xAB\x84a,\x94\x86f\x9F2u$b\xA0\0a-\xA4V[a,\xA6\x90g\r\xC5R\x7Fd, \0a3PV[a-\xA4V[a,\xBD\x90g\r\xE0\xB6\xB3\xA7d\0\0a3PV[\x90Pa,\xE1g\t\xD0(\xCCo _\xFF\x19\x85a,\xD7\x85\x85a0sV[a,\xA6\x91\x90a3)V[\x92PPP`\0[`\x02\x81\x10\x15a-|W`\0\x86a,\xFD\x84a.\x8FV[a-\x07\x91\x90a3)V[\x90P`\0a-\x15\x84\x85a-\xA4V[a-\x1E\x90a3\xD6V[\x90P`\0a-+\x82a&\xF0V[\x90P`\0a-9\x86\x85a-\xA4V[a-Kg\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x84a-\xA4V[a-U\x91\x90a3)V[\x90Pa-a\x84\x82a0sV[a-k\x90\x87a3PV[\x95P\x84`\x01\x01\x94PPPPPa,\xE8V[g\r\xE0\xB6\xB3\xA7d\0\0\x88\x12a-\x99Wa-\x94\x82a3\xD6V[a$4V[P\x96\x95PPPPPPV[`\0a%|\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a0\x84V[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a-\xD1W`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[`\0\x80\x82\x11a.$W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x08AV[P`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x82\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[`\0\x81`\0\x03a.\xA8WPg\r\xE0\xB6\xB3\xA7d\0\0\x91\x90PV[gV\x98\xEE\xF0fp\0\0\x82\x12a.\xBFWP`\0\x91\x90PV[a.\xD0gV\x98\xEE\xF0fp\0\0a3\xD6V[\x82\x13a.\xE5WPg\x1B\xC1mgN\xC8\0\0\x91\x90PV[`\0a.\xF0\x83a0\xA3V[\x90P`\0a/)g\r\xE0\xB6\xB3\xA7d\0\0a/\x12\x84g\x1B\xC1mgN\xC8\0\0a&>V[a/$\x90g\r\xE0\xB6\xB3\xA7d\0\0a3PV[a0sV[\x90P`\0\x80\x82a/\x85\x81a/r\x81a/`\x81a/M\x81g\x02_\x0F\xE1\x05\xA3\x14\0a-\xA4V[a,\xA6\x90g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19a3PV[a,\xA6\x90g\x14\xA8EL\x19\xE1\xAC\0a3PV[a,\xA6\x90g\x0F\xC1\x0E\x01W\x82w\xFF\x19a3PV[a/\x97\x90g\x03\xDE\xBD\x08;\x8C|\0a3PV[\x91P\x83\x90Pa/\xFF\x81a/\xED\x81a/\xDB\x81a/\xC9\x81a/\xB6\x81\x8Ba-\xA4V[a,\xA6\x90g\x02\x95\xD4\0\xEA2W\xFF\x19a3PV[a,\xA6\x90g\x01W\xD8\xB2\xEC\xC7\x08\0a3PV[a,\xA6\x90g\x051\n\xA7\xD5!0\0a3PV[a,\xA6\x90g\r\xE0\xCC=\x15a\0\0a3PV[\x91P\x81g\x11\x90\0\xAB\x10\x0F\xFC\0a0\x15\x87\x88a-\xA4V[a0!\x90`\0\x19a3xV[a0+\x91\x90a3)V[a05\x91\x90a3PV[\x92PP`\0a0C\x83a&\xF0V[\x90P`\0a0Q\x85\x83a-\xA4V[\x90P`\0\x88\x12a0aW\x80a$4V[a$4\x81g\x1B\xC1mgN\xC8\0\0a3)V[`\0a%|\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x05\x85\x14\x17\x16a0\x9CW`\0\x80\xFD[\x05\x92\x91PPV[`\0`\x01`\xFF\x1B\x82\x03a0\xC9W`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x12\x15a0\xDAWP\x19`\x01\x01\x90V[P\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`\0` \x82\x84\x03\x12\x15a1CWa1Ca0\xDEV[P5\x91\x90PV[\x80\x15\x15\x81\x14a1XW`\0\x80\xFD[PV[`\0\x80`\0``\x84\x86\x03\x12\x15a1sWa1sa0\xDEV[\x835a1~\x81a1JV[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a1\xA8Wa1\xA8a0\xDEV[\x815`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a1\xBFW`\0\x80\xFD[\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a1\xDCWa1\xDCa0\xDEV[\x825a1\xE7\x81a1JV[\x94` \x93\x90\x93\x015\x93PPPV[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a2\"W\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a2\x06V[P`\0`@\x82\x86\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x92PPP\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a2YWa2Ya0\xDEV[PP\x805\x92` \x90\x91\x015\x91PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a%\x7FWa%\x7Fa2hV[\x81\x81\x03\x81\x81\x11\x15a%\x7FWa%\x7Fa2hV[`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x81R\x91\x90\x92\x16` \x82\x01R`@\x81\x01\x91\x90\x91R``\x01\x90V[`\0` \x82\x84\x03\x12\x15a2\xDDWa2\xDDa0\xDEV[\x81Qa1\xBF\x81a1JV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a%\x7FWa%\x7Fa2hV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a3$Wa3$a2\xFFV[P\x04\x90V[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a3IWa3Ia2hV[P\x92\x91PPV[\x80\x82\x01\x82\x81\x12`\0\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a3pWa3pa2hV[PP\x92\x91PPV[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15a3\x94Wa3\x94a2hV[\x81\x81\x05\x83\x14\x82\x15\x17a%\x7FWa%\x7Fa2hV[`\0\x82a3\xB7Wa3\xB7a2\xFFV[`\x01`\xFF\x1B\x82\x14`\0\x19\x84\x14\x16\x15a3\xD1Wa3\xD1a2hV[P\x05\x90V[`\0`\x01`\xFF\x1B\x82\x01a3\xEBWa3\xEBa2hV[P`\0\x03\x90V\xFETarget contract does not contain";
    /// The bytecode of the contract.
    pub static RMM_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x01\xF6W`\x005`\xE0\x1C\x80c\x9C\x9D\xA9\xEA\x11a\x01FW\x80c\xCC\xD1\xE4\xBE\x11a\0\xE4W\x80c\xDCv\xFA\xBC\x11a\0\xBEW\x80c\xDCv\xFA\xBC\x14a\x04hW\x80c\xF9\xA1\xC8Z\x14a\x04pW\x80c\xFA\xDF\xA6[\x14a\x04\x83W\x80c\xFE\xD3\xDF\xDA\x14a\x04\x8CWa\x01\xF6V[\x80c\xCC\xD1\xE4\xBE\x14a\x04&W\x80c\xCF\xC4\xAFU\x14a\x049W\x80c\xD4\xCA\xDFh\x14a\x04`Wa\x01\xF6V[\x80c\xB7\xD1\x9F\xC4\x11a\x01 W\x80c\xB7\xD1\x9F\xC4\x14a\x03\xDCW\x80c\xBB\x04\x98\xDE\x14a\x03\xEFW\x80c\xC0\xFF\x1A\x15\x14a\x03\xF7W\x80c\xC5)\x87\xCF\x14a\x03\xFFWa\x01\xF6V[\x80c\x9C\x9D\xA9\xEA\x14a\x03\x8FW\x80c\xA5\x9C\x18o\x14a\x03\xA2W\x80c\xAF\xDF1\xCD\x14a\x03\xB5Wa\x01\xF6V[\x80cT\xCF*\xEB\x11a\x01\xB3W\x80cvp\x166\x11a\x01\x8DW\x80cvp\x166\x14a\x03GW\x80c}iS~\x14a\x03ZW\x80c\x87kU\xF1\x14a\x03mW\x80c\x94\xB9\x18\xDE\x14a\x03|Wa\x01\xF6V[\x80cT\xCF*\xEB\x14a\x03\x16W\x80cU\x9D\x16\x02\x14a\x03\x1FW\x80cp\xA0\x821\x14a\x03'Wa\x01\xF6V[\x80c\x02\xC2\xE5]\x14a\x02[W\x80c\x04\xAF\xA8\"\x14a\x02\x88W\x80c\x08\xEA\xBD\xDA\x14a\x02\xB6W\x80c\x15w\x0F\x92\x14a\x02\xCDW\x80c\x16\xDC\x16[\x14a\x02\xD6W\x80c4\xE1\x99\x07\x14a\x03\x01W[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x02na\x02i6`\x04a1.V[a\x04\x94V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01[`@Q\x80\x91\x03\x90\xF3[a\x02\x9Ba\x02\x966`\x04a1[V[a\x07\xEEV[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x02\x7FV[a\x02\xBF`\x03T\x81V[`@Q\x90\x81R` \x01a\x02\x7FV[a\x02\xBF`\x05T\x81V[`\0Ta\x02\xE9\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\x7FV[a\x03\x14a\x03\x0F6`\x04a1.V[a\x0BEV[\0[a\x02\xBF`\x02T\x81V[`\x03Ta\x02\xBFV[a\x02\xBFa\x0356`\x04a1\x93V[`\x06` R`\0\x90\x81R`@\x90 T\x81V[a\x02\x9Ba\x03U6`\x04a1\xC6V[a\x0B\x99V[a\x02na\x03h6`\x04a1.V[a\x10\x05V[```@Qa\x02\x7F\x91\x90a1\xF5V[a\x02\xBFa\x03\x8A6`\x04a1.V[a\x13\x03V[a\x02na\x03\x9D6`\x04a1.V[a\x17JV[a\x02na\x03\xB06`\x04a2CV[a\x1ALV[a\x02\xBF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\x01Ta\x02\xE9\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x02\xBFa\x1C\x90V[a\x02\xBFa\x1C\xCCV[a\x02\xBF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02na\x0446`\x04a1.V[a\x1D\0V[a\x02\xBF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\x02Ta\x02\xBFV[a\x02\xBFa\x1F\xFEV[a\x02na\x04~6`\x04a2CV[a qV[a\x02\xBF`\x04T\x81V[`\x04Ta\x02\xBFV[`\0\x80`\0a\x05\n`\x03T`\x05T\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\"\x19V[\x90P`\0a\x05g\x85`\x03Ta\x05\x1F\x91\x90a2~V[\x83\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\"\xC3V[\x90P`\0a\x05\xB7\x82\x84\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a#OV[\x90P`\0`\x04T\x82a\x05\xC9\x91\x90a2\x91V[\x90P`\0`\x05T\x84a\x05\xDB\x91\x90a2\x91V[\x90P\x83`\x05\x81\x90UP\x87`\x03`\0\x82\x82Ta\x05\xF6\x91\x90a2~V[\x92PP\x81\x90UP\x81`\x04`\0\x82\x82Ta\x06\x0F\x91\x90a2~V[\x90\x91UPP`\0T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\x06H\x903\x900\x90\x8D\x90`\x04\x01a2\xA4V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x06\xA0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a3\xF3\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x06\xB4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xD8\x91\x90a2\xC8V[P`\x01T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\x07\r\x903\x900\x90\x87\x90`\x04\x01a2\xA4V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x07eW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a3\xF3\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x07yW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x9D\x91\x90a2\xC8V[P`@\x80Q\x85\x81R` \x81\x01\x8A\x90R\x90\x81\x01\x83\x90R3\x90\x7F\xBE\xB3\x88W\x86\xD67\xA4t\xCB\xC2\x87\xC0\xA4E\x87#\x163\xA0w\xF0\xBD05MZK\x18\x99o\xCE\x90``\x01[`@Q\x80\x91\x03\x90\xA2\x97\x90\x96P\x94PPPPPV[`\0\x80`\0`\x05T`\0\x14a\x08JW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FPool already initialized\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[\x85\x15a\x08\xF4W\x84\x92Pa\x08\x9F\x83\x85\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\"\xC3V[\x90Pa\x08\xED\x81\x85\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a#OV[\x91Pa\t\x94V[\x84\x91Pa\tC\x82\x85\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a#\xDBV[\x90Pa\t\x91\x81\x85\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a$@V[\x92P[`\x05\x81\x90U`\x03\x83\x90U`\x04\x82\x81U3`\0\x81\x81R`\x06` R`@\x80\x82 \x85\x90U\x90T\x90Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x92c#\xB8r\xDD\x92a\t\xE6\x92\x90\x910\x91\x89\x91\x01a2\xA4V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\n>W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a3\xF3\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\nRW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\nv\x91\x90a2\xC8V[P`\x01T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\n\xAB\x903\x900\x90\x87\x90`\x04\x01a2\xA4V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0B\x03W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a3\xF3\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x0B\x17W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B;\x91\x90a2\xC8V[P\x93P\x93P\x93\x90PV[g\r\xE0\xB6\xB3\xA7d\0\0\x81\x10a\x0B\x94W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rt\t\xCC\xAE\xE4\x0En\xEC.\x04\x0C\xCC\xAC\xA4\x0E\x8D\xED\xE4\r\r,\xED`[\x1B`D\x82\x01R`d\x01a\x08AV[`\x02UV[`\0\x80`\0\x80`\x05T\x11a\x0B\xE6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rs\x14\x1B\xDB\xDB\x08\x1B\x9B\xDD\x08\x1A[\x9A]\x1AX[\x1A^\x99Y`b\x1B`D\x82\x01R`d\x01a\x08AV[`\0a\x0CY`\x03T`\x05T\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\"\x19V[\x90P\x85\x15a\x0C\xF0W\x84\x93P`\0a\x0Cw\x85`\x03Ta\x05\x1F\x91\x90a2~V[\x90P`\0a\x0C\xC7\x82\x84\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a#OV[\x90P`\x04T\x81a\x0C\xD7\x91\x90a2\x91V[\x94P`\x05T\x82a\x0C\xE7\x91\x90a2\x91V[\x93PPPa\r\xC3V[\x84\x92P`\0a\rN\x84`\x04Ta\r\x06\x91\x90a2~V[\x83\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a#\xDBV[\x90P`\0a\r\x9E\x82\x84\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a$@V[\x90P`\x03T\x81a\r\xAE\x91\x90a2\x91V[\x95P`\x05T\x82a\r\xBE\x91\x90a2\x91V[\x93PPP[\x81`\x05`\0\x82\x82Ta\r\xD5\x91\x90a2~V[\x92PP\x81\x90UP\x83`\x03`\0\x82\x82Ta\r\xEE\x91\x90a2~V[\x92PP\x81\x90UP\x82`\x04`\0\x82\x82Ta\x0E\x07\x91\x90a2~V[\x90\x91UPP3`\0\x90\x81R`\x06` R`@\x81 \x80T\x84\x92\x90a\x0E+\x90\x84\x90a2~V[\x90\x91UPP`\0T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\x0Ed\x903\x900\x90\x89\x90`\x04\x01a2\xA4V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0E\xBCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a3\xF3\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x0E\xD0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\xF4\x91\x90a2\xC8V[P`\x01T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\x0F)\x903\x900\x90\x88\x90`\x04\x01a2\xA4V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0F\x81W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a3\xF3\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x0F\x95W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xB9\x91\x90a2\xC8V[P`@\x80Q\x83\x81R` \x81\x01\x86\x90R\x90\x81\x01\x84\x90R3\x90\x7F\xBE\xB3\x88W\x86\xD67\xA4t\xCB\xC2\x87\xC0\xA4E\x87#\x163\xA0w\xF0\xBD05MZK\x18\x99o\xCE\x90``\x01`@Q\x80\x91\x03\x90\xA2P\x92P\x92P\x92V[`\0\x80`\0a\x10{`\x03T`\x05T\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\"\x19V[\x90P`\0a\x10\x90\x85`\x03Ta\r\x06\x91\x90a2\x91V[\x90P`\0a\x10\xE0\x82\x84\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a$@V[\x90P`\0\x81`\x03Ta\x10\xF2\x91\x90a2\x91V[\x90P`\0\x83`\x05Ta\x11\x04\x91\x90a2\x91V[\x90P\x83`\x05\x81\x90UP\x81`\x03`\0\x82\x82Ta\x11\x1F\x91\x90a2\x91V[\x92PP\x81\x90UP\x87`\x04`\0\x82\x82Ta\x118\x91\x90a2\x91V[\x90\x91UPP`\0T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x11\xC7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a3\xF3\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x11\xDBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\xFF\x91\x90a2\xC8V[P`\x01T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x8A\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x12\x8AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a3\xF3\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x12\x9EW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\xC2\x91\x90a2\xC8V[P`@\x80Q\x85\x81R` \x81\x01\x84\x90R\x90\x81\x01\x89\x90R3\x90\x7FY\xC3\xA0\xB6\x0Cj\xB7\xDE\xB6.\x14@\xC9\xE7$A\xDBm\xB7\xDF\xE5\x14\xDB\xA8\xCB\x18\xE6\x0C\r\x89n\xFA\x90``\x01a\x07\xDAV[`\0\x80g\r\xE0\xB6\xB3\xA7d\0\0`\x02Tg\r\xE0\xB6\xB3\xA7d\0\0a\x13%\x91\x90a2\x91V[a\x13/\x90\x85a2\xE8V[a\x139\x91\x90a3\x15V[\x90P`\0a\x13G\x82\x85a2\x91V[\x90P`\0a\x13\xBC`\x03T`\x05T\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\"\x19V[\x90P`\0a\x14\x0C\x83\x83\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\"\xC3V[\x90P`\0a\x14\\\x82\x84\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a#OV[\x90P`\x01a\x14\xB6`\x03T\x89`\x04T\x85`\x05T\x88\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a$\x9CV[a\x14\xC0\x91\x90a3)V[\x19\x95P\x86`\x03`\0\x82\x82Ta\x14\xD5\x91\x90a2~V[\x92PP\x81\x90UP\x85`\x04`\0\x82\x82Ta\x14\xEE\x91\x90a2\x91V[\x90\x91UPP`\0T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\x15'\x903\x900\x90\x8C\x90`\x04\x01a2\xA4V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x15\x7FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a3\xF3\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x15\x93W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\xB7\x91\x90a2\xC8V[P`\x01T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x88\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x16BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a3\xF3\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x16VW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16z\x91\x90a2\xC8V[P3`\x01`\x01`\xA0\x1B\x03\x16\x7F\x17fH\xF1\xF1\x1C\xDA(L\x12D\x90\x08k\xE4*\x92m\xDF\n\xE8\x87\xEB\xE7\xB1\xD6\xB37\xD8\x94'V`\x01\x89\x89a\x17\x1B`\x03T`\x05T\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\"\x19V[`@\x80Q\x94\x15\x15\x85R` \x85\x01\x93\x90\x93R\x91\x83\x01R``\x82\x01R`\x80\x01`@Q\x80\x91\x03\x90\xA2PPPPP\x91\x90PV[`\0\x80`\0a\x17\xC0`\x03T`\x05T\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\"\x19V[\x90P`\0a\x17\xD5\x85`\x04Ta\r\x06\x91\x90a2~V[\x90P`\0a\x18%\x82\x84\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a$@V[\x90P`\0`\x03T\x82a\x187\x91\x90a2\x91V[\x90P`\0`\x05T\x84a\x18I\x91\x90a2\x91V[\x90P\x83`\x05\x81\x90UP\x81`\x03`\0\x82\x82Ta\x18d\x91\x90a2~V[\x92PP\x81\x90UP\x87`\x04`\0\x82\x82Ta\x18}\x91\x90a2~V[\x90\x91UPP`\0T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\x18\xB6\x903\x900\x90\x87\x90`\x04\x01a2\xA4V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x19\x0EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a3\xF3\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x19\"W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19F\x91\x90a2\xC8V[P`\x01T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\x19{\x903\x900\x90\x8D\x90`\x04\x01a2\xA4V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x19\xD3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a3\xF3\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x19\xE7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A\x0B\x91\x90a2\xC8V[P`@\x80Q\x85\x81R` \x81\x01\x84\x90R\x90\x81\x01\x89\x90R3\x90\x7F\xBE\xB3\x88W\x86\xD67\xA4t\xCB\xC2\x87\xC0\xA4E\x87#\x163\xA0w\xF0\xBD05MZK\x18\x99o\xCE\x90``\x01a\x07\xDAV[`\0\x80`\0a\x1A\x9D\x85\x85\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a#\xDBV[\x90P`\0a\x1A\xED\x82\x86\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a$@V[`\x05\x83\x90U`\x03\x81\x90U`\x04\x87\x81U`\0T`@Qc#\xB8r\xDD`\xE0\x1B\x81R\x92\x93P`\x01`\x01`\xA0\x1B\x03\x16\x91c#\xB8r\xDD\x91a\x1B/\x913\x910\x91\x87\x91\x01a2\xA4V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1B\x87W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a3\xF3\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x1B\x9BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B\xBF\x91\x90a2\xC8V[P`\x01T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\x1B\xF4\x903\x900\x90\x8B\x90`\x04\x01a2\xA4V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1CLW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a3\xF3\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x1C`W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C\x84\x91\x90a2\xC8V[P\x90\x95\x90\x94P\x92PPPV[`\0`\x04Tg\r\xE0\xB6\xB3\xA7d\0\0a\x1C\xA6a\x1F\xFEV[`\x03Ta\x1C\xB3\x91\x90a2\xE8V[a\x1C\xBD\x91\x90a3\x15V[a\x1C\xC7\x91\x90a2~V[\x90P\x90V[`\0a\x1C\xC7`\x03T`\x05T`\x04T\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a%\x17V[`\0\x80`\0a\x1Dv`\x03T`\x05T\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\"\x19V[\x90P`\0a\x1D\x8B\x85`\x03Ta\x05\x1F\x91\x90a2\x91V[\x90P`\0a\x1D\xDB\x82\x84\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a#OV[\x90P`\0\x81`\x04Ta\x1D\xED\x91\x90a2\x91V[\x90P`\0\x83`\x05Ta\x1D\xFF\x91\x90a2\x91V[\x90P\x83`\x05\x81\x90UP\x87`\x03`\0\x82\x82Ta\x1E\x1A\x91\x90a2\x91V[\x92PP\x81\x90UP\x81`\x04`\0\x82\x82Ta\x1E3\x91\x90a2\x91V[\x90\x91UPP`\0T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x8A\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1E\xC2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a3\xF3\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x1E\xD6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E\xFA\x91\x90a2\xC8V[P`\x01T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1F\x85W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a3\xF3\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x1F\x99W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1F\xBD\x91\x90a2\xC8V[P`@\x80Q\x85\x81R` \x81\x01\x8A\x90R\x90\x81\x01\x83\x90R3\x90\x7FY\xC3\xA0\xB6\x0Cj\xB7\xDE\xB6.\x14@\xC9\xE7$A\xDBm\xB7\xDF\xE5\x14\xDB\xA8\xCB\x18\xE6\x0C\r\x89n\xFA\x90``\x01a\x07\xDAV[`\0a\x1C\xC7`\x03T`\x05T\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\"\x19V[`\0\x80`\0a \xC2\x85\x85\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\"\xC3V[\x90P`\0a!\x12\x82\x86\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a#OV[`\x05\x83\x90U`\x03\x87\x90U`\x04\x81\x81U`\0T`@Qc#\xB8r\xDD`\xE0\x1B\x81R\x92\x93P`\x01`\x01`\xA0\x1B\x03\x16\x91c#\xB8r\xDD\x91a!T\x913\x910\x91\x8C\x91\x01a2\xA4V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a!\xACW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a3\xF3\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a!\xC0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a!\xE4\x91\x90a2\xC8V[P`\x01T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\x1B\xF4\x903\x900\x90\x86\x90`\x04\x01a2\xA4V[`\0\x80a\"Hg\x06\xF0[Y\xD3\xB2\0\0a\"Ca\"=\x87g\x1B\xC1mgN\xC8\0\0a%PV[\x86a%\x85V[a%\x85V[\x90P`\0a\"Y\x85a\"C\x86a%\x9AV[a\"g\x90c;\x9A\xCA\0a2\xE8V[\x90P`\0a\"u\x89\x89a&>V[\x90Pa\"\xB6\x87a\"\xB1\x85a\"\xA2a\"\x9Ca\"\x97\x87g\r\xE0\xB6\xB3\xA7d\0\0a2\x91V[a&SV[\x87a%\x85V[a\"\xAC\x91\x90a2\x91V[a&\xF0V[a(\x99V[\x99\x98PPPPPPPPPV[`\0\x80a\"\xD8a\"\xD3\x86\x86a(\xAEV[a(\xC3V[\x90P`\0a\"\xFAg\x06\xF0[Y\xD3\xB2\0\0a\"\xB1\x86g\x1B\xC1mgN\xC8\0\0a%PV[\x90P`\0a#-\x85a#\x0C\x84\x86a3PV[a#\x1E\x90g\r\xE0\xB6\xB3\xA7d\0\0a3xV[a#(\x91\x90a3\xA8V[a*\x9EV[\x90P`\0a#C\x82g\r\xE0\xB6\xB3\xA7d\0\0a3)V[\x90Pa\"\xB6\x89\x82a(\xAEV[`\0\x80a#_a\"\xD3\x86\x86a(\xAEV[\x90P`\0a#\x81g\x06\xF0[Y\xD3\xB2\0\0a\"\xB1\x86g\x1B\xC1mgN\xC8\0\0a%PV[\x90P`\0a#\x8F\x82\x84a3)V[\x90P`\0\x85a#\xA6\x83g\r\xE0\xB6\xB3\xA7d\0\0a3xV[a#\xB0\x91\x90a3\xA8V[\x90P`\0a#\xBD\x82a*\x9EV[\x90Pa#\xCD\x88a\"\xB1\x8C\x84a(\x99V[\x9A\x99PPPPPPPPPPV[`\0\x80a#\xEBa\"\xD3\x86\x86a(\xAEV[\x90P`\0a$\rg\x06\xF0[Y\xD3\xB2\0\0a\"\xB1\x86g\x1B\xC1mgN\xC8\0\0a%PV[\x90P`\0a$\x1F\x85a#\x0C\x84\x86a3)V[\x90Pa$4\x88a$/\x88\x84a(\x99V[a(\xAEV[\x98\x97PPPPPPPPV[`\0\x80a$Pa\"\xD3\x86\x86a(\xAEV[\x90P`\0a$rg\x06\xF0[Y\xD3\xB2\0\0a\"\xB1\x86g\x1B\xC1mgN\xC8\0\0a%PV[\x90P`\0a$\x84\x85a#\x0C\x84\x86a3PV[\x90Pa$4\x88a\"\xB1\x83g\r\xE0\xB6\xB3\xA7d\0\0a3)V[`\0\x80a$\xAD\x84a\"C\x87\x89a2~V[\x90P`\0a$\xE6a$\xD3a\"\x97a$\xC4\x8D\x8Fa2~V[a$\xCE\x8A\x8Ca2~V[a&>V[a$\xDC\x86a3\xD6V[a#(\x91\x90a3)V[\x90P\x87\x89a$\xF4\x84\x84a%\x85V[a$\xFE\x91\x90a3)V[a%\x08\x91\x90a3)V[\x9B\x9APPPPPPPPPPPV[`\0a%0a%&\x85\x84a%\x85V[a\"\x97\x90\x85a3\x15V[a%=a\"\x97\x86\x88a3\x15V[a%G\x91\x90a3PV[\x95\x94PPPPPV[`\0a%|g\r\xE0\xB6\xB3\xA7d\0\0\x83a%h\x86a(\xC3V[a%r\x91\x90a3xV[a\"\xAC\x91\x90a3\xA8V[\x90P[\x92\x91PPV[`\0a%|\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a+\x07V[`\xB5\x81`\x01`\x88\x1B\x81\x10a%\xB3W`@\x91\x90\x91\x1B\x90`\x80\x1C[i\x01\0\0\0\0\0\0\0\0\0\x81\x10a%\xCFW` \x91\x90\x91\x1B\x90`@\x1C[e\x01\0\0\0\0\0\x81\x10a%\xE7W`\x10\x91\x90\x91\x1B\x90` \x1C[c\x01\0\0\0\x81\x10a%\xFDW`\x08\x91\x90\x91\x1B\x90`\x10\x1C[b\x01\0\0\x01\x02`\x12\x1C\x80\x82\x04\x01`\x01\x90\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x90\x81\x90\x04\x81\x11\x90\x03\x90V[`\0a%|\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a+\x07V[`\0g\x06\xF0[Y\xD3\xB2\0\0\x82\x03a&lWP`\0\x91\x90PV[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x12a&\x94W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a&\xB5W`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a&\xC2\x83`\x02a3xV[\x90P`\0a&\xCF\x82a+&V[\x90P`\0a&\xE5g\x13\xA0K\xBD\xFD\xC9\xBE\x88\x83a-\xA4V[\x90Pa%G\x81a3\xD6V[`\0h\x02H\xCE6\xA7\x0C\xB2k>\x19\x82\x13a'\x0BWP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a'RW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x01a\x08AV[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05`\x01`_\x1B\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1Dm\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1Dn\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[`\0a%|\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a-\xB9V[`\0a%|\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a-\xB9V[`\0\x80\x82\x13a)\0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x08AV[`\0``a)\r\x84a-\xE7V[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[`\0\x80g\x13\xA0K\xBD\xFD\xC9\xBE\x88a*\xBCg\r\xE0\xB6\xB3\xA7d\0\0\x85a3xV[a*\xC6\x91\x90a3\xA8V[\x90P`\0a*\xD3\x82a3\xD6V[\x90P`\0a*\xE0\x82a.\x8FV[\x90Pg\x1B\xC1mgN\xC8\0\0a*\xFDg\r\xE0\xB6\xB3\xA7d\0\0\x83a3xV[a%G\x91\x90a3\xA8V[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a+\x1FW`\0\x80\xFD[\x04\x92\x91PPV[`\0\x80\x82\x12\x80a+=WPg\x1B\xC1mgN\xC8\0\0\x82\x13[\x15a+[W`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a+|W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81g\x1B\xC1mgN\xC8\0\0\x03a+\xA4W`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x15a+\xAFW\x91\x90PV[`\0g\r\xE0\xB6\xB3\xA7d\0\0\x83\x12a+\xD7Wa+\xD2\x83g\x1B\xC1mgN\xC8\0\0a3)V[a+\xD9V[\x82[\x90P`\0a+\xEF\x82g\x1B\xC1mgN\xC8\0\0a0sV[\x90P\x80`\0\x03a,\x12W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a,\x1D\x82a(\xC3V[\x90P`\0c;\x9A\xCA\0a,Ha,Ca,=g\x1B\xC1mgN\xC8\0\0a3\xD6V[\x85a-\xA4V[a%\x9AV[a,R\x91\x90a3xV[\x90P`\0\x80a,i\x83g\x03\xC1f\\z\xAB \0a-\xA4V[a,{\x90g \x05\xFEO&\x8E\xA0\0a3PV[\x90P`\0a,\xAB\x84a,\x94\x86f\x9F2u$b\xA0\0a-\xA4V[a,\xA6\x90g\r\xC5R\x7Fd, \0a3PV[a-\xA4V[a,\xBD\x90g\r\xE0\xB6\xB3\xA7d\0\0a3PV[\x90Pa,\xE1g\t\xD0(\xCCo _\xFF\x19\x85a,\xD7\x85\x85a0sV[a,\xA6\x91\x90a3)V[\x92PPP`\0[`\x02\x81\x10\x15a-|W`\0\x86a,\xFD\x84a.\x8FV[a-\x07\x91\x90a3)V[\x90P`\0a-\x15\x84\x85a-\xA4V[a-\x1E\x90a3\xD6V[\x90P`\0a-+\x82a&\xF0V[\x90P`\0a-9\x86\x85a-\xA4V[a-Kg\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x84a-\xA4V[a-U\x91\x90a3)V[\x90Pa-a\x84\x82a0sV[a-k\x90\x87a3PV[\x95P\x84`\x01\x01\x94PPPPPa,\xE8V[g\r\xE0\xB6\xB3\xA7d\0\0\x88\x12a-\x99Wa-\x94\x82a3\xD6V[a$4V[P\x96\x95PPPPPPV[`\0a%|\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a0\x84V[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a-\xD1W`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[`\0\x80\x82\x11a.$W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x08AV[P`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x82\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[`\0\x81`\0\x03a.\xA8WPg\r\xE0\xB6\xB3\xA7d\0\0\x91\x90PV[gV\x98\xEE\xF0fp\0\0\x82\x12a.\xBFWP`\0\x91\x90PV[a.\xD0gV\x98\xEE\xF0fp\0\0a3\xD6V[\x82\x13a.\xE5WPg\x1B\xC1mgN\xC8\0\0\x91\x90PV[`\0a.\xF0\x83a0\xA3V[\x90P`\0a/)g\r\xE0\xB6\xB3\xA7d\0\0a/\x12\x84g\x1B\xC1mgN\xC8\0\0a&>V[a/$\x90g\r\xE0\xB6\xB3\xA7d\0\0a3PV[a0sV[\x90P`\0\x80\x82a/\x85\x81a/r\x81a/`\x81a/M\x81g\x02_\x0F\xE1\x05\xA3\x14\0a-\xA4V[a,\xA6\x90g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19a3PV[a,\xA6\x90g\x14\xA8EL\x19\xE1\xAC\0a3PV[a,\xA6\x90g\x0F\xC1\x0E\x01W\x82w\xFF\x19a3PV[a/\x97\x90g\x03\xDE\xBD\x08;\x8C|\0a3PV[\x91P\x83\x90Pa/\xFF\x81a/\xED\x81a/\xDB\x81a/\xC9\x81a/\xB6\x81\x8Ba-\xA4V[a,\xA6\x90g\x02\x95\xD4\0\xEA2W\xFF\x19a3PV[a,\xA6\x90g\x01W\xD8\xB2\xEC\xC7\x08\0a3PV[a,\xA6\x90g\x051\n\xA7\xD5!0\0a3PV[a,\xA6\x90g\r\xE0\xCC=\x15a\0\0a3PV[\x91P\x81g\x11\x90\0\xAB\x10\x0F\xFC\0a0\x15\x87\x88a-\xA4V[a0!\x90`\0\x19a3xV[a0+\x91\x90a3)V[a05\x91\x90a3PV[\x92PP`\0a0C\x83a&\xF0V[\x90P`\0a0Q\x85\x83a-\xA4V[\x90P`\0\x88\x12a0aW\x80a$4V[a$4\x81g\x1B\xC1mgN\xC8\0\0a3)V[`\0a%|\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x05\x85\x14\x17\x16a0\x9CW`\0\x80\xFD[\x05\x92\x91PPV[`\0`\x01`\xFF\x1B\x82\x03a0\xC9W`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x12\x15a0\xDAWP\x19`\x01\x01\x90V[P\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`\0` \x82\x84\x03\x12\x15a1CWa1Ca0\xDEV[P5\x91\x90PV[\x80\x15\x15\x81\x14a1XW`\0\x80\xFD[PV[`\0\x80`\0``\x84\x86\x03\x12\x15a1sWa1sa0\xDEV[\x835a1~\x81a1JV[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a1\xA8Wa1\xA8a0\xDEV[\x815`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a1\xBFW`\0\x80\xFD[\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a1\xDCWa1\xDCa0\xDEV[\x825a1\xE7\x81a1JV[\x94` \x93\x90\x93\x015\x93PPPV[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a2\"W\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a2\x06V[P`\0`@\x82\x86\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x92PPP\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a2YWa2Ya0\xDEV[PP\x805\x92` \x90\x91\x015\x91PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a%\x7FWa%\x7Fa2hV[\x81\x81\x03\x81\x81\x11\x15a%\x7FWa%\x7Fa2hV[`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x81R\x91\x90\x92\x16` \x82\x01R`@\x81\x01\x91\x90\x91R``\x01\x90V[`\0` \x82\x84\x03\x12\x15a2\xDDWa2\xDDa0\xDEV[\x81Qa1\xBF\x81a1JV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a%\x7FWa%\x7Fa2hV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a3$Wa3$a2\xFFV[P\x04\x90V[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a3IWa3Ia2hV[P\x92\x91PPV[\x80\x82\x01\x82\x81\x12`\0\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a3pWa3pa2hV[PP\x92\x91PPV[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15a3\x94Wa3\x94a2hV[\x81\x81\x05\x83\x14\x82\x15\x17a%\x7FWa%\x7Fa2hV[`\0\x82a3\xB7Wa3\xB7a2\xFFV[`\x01`\xFF\x1B\x82\x14`\0\x19\x84\x14\x16\x15a3\xD1Wa3\xD1a2hV[P\x05\x90V[`\0`\x01`\xFF\x1B\x82\x01a3\xEBWa3\xEBa2hV[P`\0\x03\x90V\xFETarget contract does not contain";
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
        ///Calls the contract's `setSwapFee` (0x34e19907) function
        pub fn set_swap_fee(
            &self,
            new_swap_fee: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([52, 225, 153, 7], new_swap_fee)
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
        ///Calls the contract's `swap` (0x94b918de) function
        pub fn swap(
            &self,
            amount_x: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([148, 185, 24, 222], amount_x)
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
    ///Container type for all input parameters for the `swap` function with signature `swap(uint256)` and selector `0x94b918de`
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
    #[ethcall(name = "swap", abi = "swap(uint256)")]
    pub struct SwapCall {
        pub amount_x: ::ethers::core::types::U256,
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
        GetPortfolioValue(GetPortfolioValueCall),
        GetReserveX(GetReserveXCall),
        GetReserveY(GetReserveYCall),
        GetSpotPrice(GetSpotPriceCall),
        GetStrategyData(GetStrategyDataCall),
        GetSwapFee(GetSwapFeeCall),
        InitExactX(InitExactXCall),
        InitExactY(InitExactYCall),
        InitPool(InitPoolCall),
        RemoveLiquidityExactX(RemoveLiquidityExactXCall),
        RemoveLiquidityExactY(RemoveLiquidityExactYCall),
        ReserveX(ReserveXCall),
        ReserveY(ReserveYCall),
        SetSwapFee(SetSwapFeeCall),
        Sigma(SigmaCall),
        StrikePrice(StrikePriceCall),
        Swap(SwapCall),
        SwapFee(SwapFeeCall),
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
            if let Ok(decoded) = <SetSwapFeeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetSwapFee(decoded));
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
                Self::InitExactX(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InitExactY(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InitPool(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
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
                Self::SetSwapFee(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Sigma(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::StrikePrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Swap(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SwapFee(element) => ::ethers::core::abi::AbiEncode::encode(element),
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
                Self::GetPortfolioValue(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetReserveX(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetReserveY(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSpotPrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetStrategyData(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSwapFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::InitExactX(element) => ::core::fmt::Display::fmt(element, f),
                Self::InitExactY(element) => ::core::fmt::Display::fmt(element, f),
                Self::InitPool(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveLiquidityExactX(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RemoveLiquidityExactY(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ReserveX(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReserveY(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetSwapFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::Sigma(element) => ::core::fmt::Display::fmt(element, f),
                Self::StrikePrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::Swap(element) => ::core::fmt::Display::fmt(element, f),
                Self::SwapFee(element) => ::core::fmt::Display::fmt(element, f),
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
    impl ::core::convert::From<SetSwapFeeCall> for RMMCalls {
        fn from(value: SetSwapFeeCall) -> Self {
            Self::SetSwapFee(value)
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
    impl ::core::convert::From<SwapFeeCall> for RMMCalls {
        fn from(value: SwapFeeCall) -> Self {
            Self::SwapFee(value)
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
    ///Container type for all return fields from the `swap` function with signature `swap(uint256)` and selector `0x94b918de`
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
    pub struct SwapReturn {
        pub amount_y: ::ethers::core::types::U256,
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
