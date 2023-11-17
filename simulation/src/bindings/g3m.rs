pub use g3m::*;
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
pub mod g3m {
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
                        name: ::std::borrow::ToOwned::to_owned("weightX_"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("UD60x18"),
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
                    ::std::borrow::ToOwned::to_owned("_initPool"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("_initPool"),
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
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("UD60x18"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("addLiquidity"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("addLiquidity"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("liquidity"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("UD60x18"),
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
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
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
                                        ::std::borrow::ToOwned::to_owned("UD60x18"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("admin"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("admin"),
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
                                        ::std::borrow::ToOwned::to_owned("UD60x18"),
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
                    ::std::borrow::ToOwned::to_owned("initPool"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("initPool"),
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
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("UD60x18"),
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
                                    name: ::std::borrow::ToOwned::to_owned("initial_x"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("initial_price"),
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
                    ::std::borrow::ToOwned::to_owned("liquidityWithoutPrecision"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "liquidityWithoutPrecision",
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
                    ::std::borrow::ToOwned::to_owned("removeLiquidity"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("removeLiquidity"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("liquidity"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("UD60x18"),
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
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("removeLiquidity"),
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
                                        ::std::borrow::ToOwned::to_owned("UD60x18"),
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
                                        ::std::borrow::ToOwned::to_owned("UD60x18"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("reserveXWithoutPrecision"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "reserveXWithoutPrecision",
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
                                        ::std::borrow::ToOwned::to_owned("UD60x18"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("reserveYWithoutPrecision"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "reserveYWithoutPrecision",
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
                    ::std::borrow::ToOwned::to_owned("setWeightX"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setWeightX"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newTargetWeightX"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("UD60x18"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newWeightXUpdateEnd",
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
                    ::std::borrow::ToOwned::to_owned("swapAmountOut"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("swapAmountOut"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("swapDirection"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
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
                                        ::std::borrow::ToOwned::to_owned("UD60x18"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("weightX"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("weightX"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("UD60x18"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("weightY"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("weightY"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("UD60x18"),
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
                    ::std::borrow::ToOwned::to_owned("LogPrices"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("LogPrices"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("spotPrice"),
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
                    ::std::borrow::ToOwned::to_owned("LogSyncingWeight"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("LogSyncingWeight"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("weightX"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("weightY"),
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
                    ::std::borrow::ToOwned::to_owned("LogWeights"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("LogWeights"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("blockTimestamp"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("weightX"),
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
                    ::std::borrow::ToOwned::to_owned("SetTargetWeightX"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("SetTargetWeightX"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newTargetWeightX"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newWeightXUpdateEnd",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newWeightXUpdatePerSecond",
                                    ),
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
                    ::std::borrow::ToOwned::to_owned("SetWeightX"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("SetWeightX"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("oldWeightX"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newWeightX"),
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
                    ::std::borrow::ToOwned::to_owned("InvalidSwap"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidSwap"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("invariantBefore"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("UD60x18"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("invariantAfter"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("UD60x18"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PRBMath_MulDiv18_Overflow"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "PRBMath_MulDiv18_Overflow",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("x"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("y"),
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
                    ::std::borrow::ToOwned::to_owned("PRBMath_MulDiv_Overflow"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "PRBMath_MulDiv_Overflow",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("x"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("y"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("denominator"),
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
                    ::std::borrow::ToOwned::to_owned("PRBMath_UD60x18_Convert_Overflow"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "PRBMath_UD60x18_Convert_Overflow",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("x"),
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
                    ::std::borrow::ToOwned::to_owned("PRBMath_UD60x18_Exp2_InputTooBig"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "PRBMath_UD60x18_Exp2_InputTooBig",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("x"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("UD60x18"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "PRBMath_UD60x18_Log_InputTooSmall",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "PRBMath_UD60x18_Log_InputTooSmall",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("x"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("UD60x18"),
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
    pub static G3M_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@\x90\x80\x82R4b\0\x029WP\x80Q`\x1Fb\0.\xC88\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x91`\x01`\x01`@\x1B\x03\x83\x11\x84\x84\x10\x17b\0\x02#W\x80\x84\x92`\x80\x94\x87R\x839\x81\x01\x03\x12b\0\x01\xD4Wb\0\0U\x81b\0\x02\x86V[\x90b\0\0d` \x82\x01b\0\x02\x86V[\x83\x82\x01Q``\x90\x92\x01Q\x92`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x91\x16\x80\x82\x14b\0\x01\x9FW`\x01\x80`\xA0\x1B\x03\x19\x91\x82`\x01T\x16\x17`\x01U\x81`\x02T\x16\x17`\x02U3\x90`\0T\x16\x17`\0Uf#\x86\xF2o\xC1\0\0\x81\x10b\0\x01hWg\r\xBD/\xC17\xA3\0\0\x81\x11b\0\x010W\x80`\nUB`\x0BU`\x08UB`\tUg\r\xE0\xB6\xB3\xA7d\0\0\x81\x11b\0\0\xF8W`\x03UQa,'\x90\x81b\0\x02\xA1\x829\xF3[\x81QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01Rp\nn\xEC.\x04\x0C\xCC\xAC\xA4\x0E\x8D\xED\xE4\r\r,\xED`{\x1B`D\x82\x01R`d\x90\xFD[\x82QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01Rp\n\xEC\xAD,\xED\x0E\x84\x0B\x04\x0E\x8D\xED\xE4\r\r,\xED`{\x1B`D\x82\x01R`d\x90\xFD[\x82QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01RoWeight X too low`\x80\x1B`D\x82\x01R`d\x90\xFD[\x84QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01RmInvalid tokens`\x90\x1B`D\x82\x01R`d\x90\xFD[\x81QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[bF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01Ra7\xB7`\xF1\x1B`d\x82\x01R`\x84\x90\xFD[Q\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03b\0\x02\x9BWV[`\0\x80\xFD\xFE`\x80`@R`\x046\x10\x15a\0\x92W[`\x84`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01R\x7Fnor receive functions\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R\xFD[`\x005`\xE0\x1C\x80c\x08\xEA\xBD\xDA\x14a\x18~W\x80c\x15w\x0F\x92\x14a\x18`W\x80c\x16\xDC\x16[\x14a\x18,W\x80c\x194\xEB%\x14a\x01\xF8W\x80c\x1F\xDA\xBC'\x14a\x15,W\x80c4\xE1\x99\x07\x14a\x14\x83W\x80c69\xAA2\x14a\x14FW\x80cQ\xC6Y\n\x14a\x12fW\x80cT\xCF*\xEB\x14a\x05^W\x80cU\x9D\x16\x02\x14a\x03\tW\x80cp\xA0\x821\x14a\x12\x17W\x80cu\xAE\xE1\xDA\x14a\x11\xEFW\x80cvp\x166\x14a\x10\x12W\x80c\x84\x89PO\x14a\t4W\x80c\x87kU\xF1\x14a\x0F+W\x80c\x8AYS\xC7\x14a\x0F\rW\x80c\x9C\x8F\x9F#\x14a\x0C\xB6W\x80c\x9C\xE32\xD4\x14a\t9W\x80c\x9E\x1B\0E\x14a\t4W\x80c\xA0\xDBj\x82\x14a\x06\xF7W\x80c\xAD\xB5\x1D\xEE\x14a\x06\xDCW\x80c\xB7\xD1\x9F\xC4\x14a\x06\xA8W\x80c\xBC\xC1}\xC7\x14a\x05\xD6W\x80c\xC0\xFF\x1A\x15\x14a\x05|W\x80c\xD4\xCA\xDFh\x14a\x05^W\x80c\xDBy\x10C\x14a\x03TW\x80c\xDCv\xFA\xBC\x14a\x031W\x80c\xE3\x11\xCE\xC8\x14a\x03\tW\x80c\xF8Q\xA4@\x14a\x02\xD5W\x80c\xF9\xA1\xC8Z\x14a\x02HW\x80c\xFA\xDF\xA6[\x14a\x02*Wc\xFE\xD3\xDF\xDA\x03a\0\x0EW[4a\x02%W`\0`\x03\x196\x01\x12a\x02 W` g\r\xE0\xB6\xB3\xA7d\0\0`\x05T\x04`@Q\x90\x81R\xF3[a\x19 V[a\x18\x9CV[4a\x02%W`\0`\x03\x196\x01\x12a\x02 W` `\x05T`@Q\x90\x81R\xF3[4a\x02%Wa\x02V6a\x19\xC3V[\x90`\x08Tg\r\xE0\xB6\xB3\xA7d\0\0\x90\x80\x82\x03\x90\x82\x82\x11a\x02\xA6Wa\x02\x88\x84a\x02\x83a\x02\x8D\x94a\x02\x94\x98a\x1AoV[a\x1AoV[a\x1A\x82V[\x04\x90a\x1DEV[P`@\x80Q`\0\x81R`\0` \x82\x01R\xF3[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[4a\x02%W`\0`\x03\x196\x01\x12a\x02 W` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\0T\x16`@Q\x90\x81R\xF3[4a\x02%W`\0`\x03\x196\x01\x12a\x02 W` g\r\xE0\xB6\xB3\xA7d\0\0`\x04T\x04`@Q\x90\x81R\xF3[4a\x02%W`\0`\x03\x196\x01\x12a\x02 W` a\x03La!MV[`@Q\x90\x81R\xF3[4a\x02%W`@`\x03\x196\x01\x12a\x02 W`\x045`$5\x90a\x03\x8Fs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\0T\x163\x14a\x1A\xFCV[f#\x86\xF2o\xC1\0\0\x81\x10a\x05\0Wg\r\xBD/\xC17\xA3\0\0\x81\x11a\x04\xA2WB\x82\x11\x15a\x04DWa\x03\xF3a\x03\xBFa \xFDV[\x80`\x08UB`\tU\x82\x80\x82\x11`\0\x14a\x045Wa\x03\xDB\x91a\x1AbV[a\x03\xEDa\x03\xE8B\x86a\x1AbV[a\x1BaV[\x90a\x1B\xBFV[`\x0CU`\nU`\x0BU\x7F\x91\t\xD5\xCC\xAE\x12\x89A\xC9q\x94\xA8t\xB03\xDE'l\x81#\xF7!\xAC\x0F\x18\x17M\x11r\xBEj\xC2`@a\x04'a \xFDV[\x81Q\x90B\x82R` \x82\x01R\xA1\0[\x90a\x04?\x91a\x1AbV[a\x03\xDBV[`d`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x11`$\x82\x01R\x7FUpdate end pasted\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R\xFD[`d`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x11`$\x82\x01R\x7FWeight X too high\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R\xFD[`d`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x10`$\x82\x01R\x7FWeight X too low\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R\xFD[4a\x02%W`\0`\x03\x196\x01\x12a\x02 W` `\x03T`@Q\x90\x81R\xF3[4a\x02%W`\0`\x03\x196\x01\x12a\x02 W` g\r\xE0\xB6\xB3\xA7d\0\0a\x05\xCD`\x04Ta\x05\xC7a\x05\xA9a \xFDV[\x91a\x05\xC1`\x05T\x93a\x05\xC1a\x05\xBCa \xFDV[a\x19\xF2V[\x92a!oV[\x90a \x13V[\x04`@Q\x90\x81R\xF3[4a\x02%W`\0`\x03\x196\x01\x12a\x02 W\x7F\x90\x84\x9A\xC6O\xD5x\x0E\xD6\xE7\xD5\x9E\xBC\xAD\xA1\xFA\xED!\xB8y\x1A\xE1z\x1C\xA8\xF555\x8E\xF5%\xD2`@a\x06\x12a!MV[\x81Q\x90\x81RB` \x82\x01R\xA1`\x04T`\x05T`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91RB\x90\x82\x01R\x7F\x94a\x90\xF3N\x99(\xE7m\x01\xD1,\xBFP7\xB1~\xDAr=\x983\xCE\xF5R$\x97Do\x7F\x81\xD5\x90``\x90\xA1\x7F+\x8E\xE9\x04\xF2\xA9_()\x9E\x1D\x8Et\xB4\xEF \xD1;\xFC\rHxL(\xE5\x0F\xEC\xC1\xD5X\xDE\xF0``a\x06\x89a \xFDV[a\x06\x94a\x05\xBCa \xFDV[`@Q\x91\x82R` \x82\x01RB`@\x82\x01R\xA1\0[4a\x02%W`\0`\x03\x196\x01\x12a\x02 W` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x02T\x16`@Q\x90\x81R\xF3[4a\x02%W`\0`\x03\x196\x01\x12a\x02 W` a\x03La \xFDV[4a\x02%Wa\x07\x056a\x19\xA4V[\x90\x15a\t\x1FWa\x07\x1B\x81`\x04T`\x05T\x90a+\xE2V[\x90[a\x07na\x07ia\x078`\x04Ta\x072\x85a\x1BaV[\x90a\x1AbV[\x80`\x04Ua\x05\xC7a\x07N`\x05Ta\x072\x88a\x1BaV[\x91\x82`\x05Ua\x05\xC1a\x07^a \xFDV[a\x05\xC1a\x05\xBCa \xFDV[a\x1F>V[a\x07z\x81`\x06Ta\x1AbV[\x90`\x06U3`\0R` \x90`\x07\x82Ra\x07\x98\x81`@`\0 Ta\x1AbV[3`\0R`\x07\x83R`@`\0 U`@Q\x7FY\xC3\xA0\xB6\x0Cj\xB7\xDE\xB6.\x14@\xC9\xE7$A\xDBm\xB7\xDF\xE5\x14\xDB\xA8\xCB\x18\xE6\x0C\r\x89n\xFA3\x91\x80a\x07\xEA\x88\x88\x87\x84`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[\x03\x90\xA2s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x82`\x01T\x16\x92\x83;\x15a\x08\xFDW`@Q\x7F\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80\x82R3`\x04\x83\x01R`$\x82\x01\x87\x90R\x94\x83\x90\x82\x90`D\x90\x82\x90`\0\x90Z\xF1\x80\x15a\x08\xF1Wa\t\x02W[P`\x02T\x16\x92\x83;\x15a\x08\xFDW`@Q\x90\x81R3`\x04\x82\x01R`$\x81\x01\x86\x90R\x92\x81\x90\x84\x90\x81`\0\x81`D\x81\x01[\x03\x92Z\xF1\x92\x83\x15a\x08\xF1Wa\x08\xBF\x93a\x08\xC3W[PP`@Q\x93\x84\x93\x84`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[\x03\x90\xF3[\x81a\x08\xE2\x92\x90=\x10a\x08\xEAW[a\x08\xDA\x81\x83a\x1A\xBBV[\x81\x01\x90a\x1D-V[P\x84\x80a\x08\xA0V[P=a\x08\xD0V[`@Q=`\0\x82>=\x90\xFD[a\x1C\xA9V[a\t\x18\x90\x83=\x85\x11a\x08\xEAWa\x08\xDA\x81\x83a\x1A\xBBV[P\x86a\x08^V[a\t/\x81`\x04T`\x05T\x90a,\x1AV[a\x07\x1DV[a\x19\xD9V[4a\x02%Wa\tG6a\x19\xA4V[a\tOa \xFDV[\x91a\t[a\x05\xBCa \xFDV[\x91`\x04T\x92`\x05Ta\tza\tp\x87\x87a!oV[a\x05\xC7\x84\x84a!oV[\x90\x84\x15a\x0C\xB0W\x85[\x85\x15a\x0C\xA9W\x87\x90[\x86\x15a\x0C\x9FWa\t\xDB\x83\x91[\x88\x15a\x0C\x8FWa\x05\xC7a\t\xD6a\t\xBC\x89\x95[a\x03\xEDa\t\xB6\x8Da\x1BaV[\x82a\x1AbV[a\t\xD0g\r\xE0\xB6\xB3\xA7d\0\0\x97\x88\x97a\x1B\xBFV[\x90a!oV[a\x1A5V[\x04\x90\x80\x82\x02\x91\x80\x83\x04\x82\x14\x90\x15\x17\x15a\x02\xA6W`\x03T\x81\x03\x90\x81\x11a\x02\xA6Wa\n\x03\x91a\x1A\x82V[\x95\x85\x15a\x0CeWPP`\x04Ta\n\x18\x86a\x1BaV[\x81\x01\x80\x91\x11a\x02\xA6W`\x04Ua\n3`\x05Ta\x072\x85a\x1BaV[`\x05U[a\nN`\x04Ta\x05\xC7\x84a\x05\xC1\x8A`\x05T\x94a!oV[\x80\x82\x11a\x0C.WPP\x82\x15a\x0C\x0FWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80`\x01T\x16[\x16\x80;\x15a\x08\xFDW`@Q\x7F#\xB8r\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R3`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x86\x90R\x90` \x90\x82\x90`d\x90\x82\x90`\0\x90Z\xF1\x80\x15a\x08\xF1Wa\x0B\xF0W[P\x82\x15a\x0B\xD1Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80`\x02T\x16[\x16\x90\x81;\x15a\x08\xFDW`@Q\x7F\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R3`\x04\x82\x01R`$\x81\x01\x84\x90R\x91` \x90\x83\x90`D\x90\x82\x90`\0\x90Z\xF1\x80\x15a\x08\xF1W` \x96a\x0Bn\x93a\x03\xED\x92a\x0B\xB4W[Pa\x0Bh`\x04T\x93`\x05Ta\x1B\xBFV[\x92a\x1B\xBFV[\x90`@Q\x92\x15\x15\x83R\x83\x85\x84\x01R`@\x83\x01R``\x82\x01R\x7F\x17fH\xF1\xF1\x1C\xDA(L\x12D\x90\x08k\xE4*\x92m\xDF\n\xE8\x87\xEB\xE7\xB1\xD6\xB37\xD8\x94'V`\x803\x92\xA2`@Q\x90\x81R\xF3[a\x0B\xCA\x90\x89=\x8B\x11a\x08\xEAWa\x08\xDA\x81\x83a\x1A\xBBV[P\x88a\x0BXV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80`\x01T\x16a\n\xF6V[a\x0C\x08\x90` =` \x11a\x08\xEAWa\x08\xDA\x81\x83a\x1A\xBBV[P\x85a\n\xD4V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80`\x02T\x16a\nxV[`D\x92P`@Q\x91\x7F\xB2!\xD0\xE4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04\x83\x01R`$\x82\x01R\xFD[a\x0Cr\x90a\x072\x86a\x1BaV[`\x04Ua\x0C~\x86a\x1BaV[\x81\x01\x80\x91\x11a\x02\xA6W`\x05Ua\n7V[a\x05\xC7a\t\xD6a\t\xBC\x8D\x95a\t\xAAV[a\t\xDB\x88\x91a\t\x98V[\x83\x90a\t\x8CV[\x80a\t\x83V[4a\x02%W` \x80`\x03\x196\x01\x12a\x02 W`\x045\x903`\0R`\x07\x81R\x81`@`\0 T\x10a\x0E\xAFW`\x06T\x90g\r\xE0\xB6\xB3\xA7d\0\0a\r(\x81a\r\x12`\x04Ta\r\ra\x05\xBC\x88a\r\x08\x8B\x82a\x1AbV[a\x1B\xBFV[a \x13V[\x04\x93a\r\ra\x05\xBC`\x05T\x92a\r\x08\x89\x82a\x1AbV[\x04\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93\x84`\x01T\x16\x94\x85;\x15a\x08\xFDW`@Q\x7F\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80\x82R3`\x04\x83\x01R`$\x82\x01\x87\x90R\x96\x84\x90\x82\x90`D\x90\x82\x90`\0\x90Z\xF1\x80\x15a\x08\xF1Wa\x0E\x92W[P`\x02T\x16\x94\x85;\x15a\x08\xFDW`@Q\x90\x81R3`\x04\x82\x01R`$\x81\x01\x84\x90R\x94\x82\x90\x86\x90`D\x90\x82\x90`\0\x90Z\xF1\x94\x85\x15a\x08\xF1W`@\x95a\x0EuW[P3`\0R`\x07\x82Ra\r\xF0\x81\x86`\0 Ta\x1AbV[3`\0R`\x07\x83R\x85`\0 Ua\x0E\t\x81`\x06Ta\x1AbV[`\x06Ua\x0E\x1B`\x04Ta\x072\x86a\x1BaV[`\x04Ua\x0E-`\x05Ta\x072\x85a\x1BaV[`\x05U\x84Q\x90\x81R` \x81\x01\x84\x90R`@\x81\x01\x83\x90R3\x90\x7FY\xC3\xA0\xB6\x0Cj\xB7\xDE\xB6.\x14@\xC9\xE7$A\xDBm\xB7\xDF\xE5\x14\xDB\xA8\xCB\x18\xE6\x0C\r\x89n\xFA\x90``\x90\xA2\x83Q\x92\x83R\x82\x01R\xF3[a\x0E\x8B\x90\x83=\x85\x11a\x08\xEAWa\x08\xDA\x81\x83a\x1A\xBBV[P\x85a\r\xD9V[a\x0E\xA8\x90\x84=\x86\x11a\x08\xEAWa\x08\xDA\x81\x83a\x1A\xBBV[P\x86a\r\x9BV[`d\x90`@Q\x90\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R`\x16`$\x82\x01R\x7FInsufficient liquidity\0\0\0\0\0\0\0\0\0\0`D\x82\x01R\xFD[4a\x02%W`\0`\x03\x196\x01\x12a\x02 W` a\x03La\x05\xBCa \xFDV[4a\x02%W`\0`\x03\x196\x01\x12a\x02 Wa\x0FDa \xFDV[a\x0FOa\x05\xBCa \xFDV[`@Q\x90` \x92\x83\x83\x01R`@\x82\x01R`@\x81R``\x81\x01\x90\x80\x82\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x17a\x0F\xE3W\x81`@R\x82\x82R\x80Q\x92\x83`\x80\x83\x01R`\0[\x84\x81\x10a\x0F\xCFW\x83`@\x84\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x89`\0`\xA0\x82\x86\x01\x01R\x01\x16\x81\x01\x03\x01\x90\xF3[\x80\x82\x91\x84\x01`\xA0\x83\x82\x01Q\x91\x01R\x01a\x0F\x8EV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[4a\x02%Wa\x10 6a\x19\xA4V[`\x06T\x91a\x10/\x83\x15\x15a \x98V[\x15a\x11\xD8Wa\x10D\x81`\x04T`\x05T\x90a+\xE2V[\x91[`\x04Ta\x10R\x83a\x1BaV[\x81\x01\x80\x91\x11a\x02\xA6W\x80`\x04U`\x05Ta\x10k\x85a\x1BaV[\x81\x01\x80\x91\x11a\x02\xA6Wa\x07i\x81a\x05\xC7a\x10\x98\x94a\x10\x91\x94`\x05Ua\x05\xC1a\x07^a \xFDV[\x91\x82a\x1AbV[\x90`\x06U3`\0R` \x90`\x07\x82R`@`\0 T\x81\x81\x01\x80\x91\x11a\x02\xA6W3`\0R`\x07\x83R`@`\0 U`@Q\x7F\xBE\xB3\x88W\x86\xD67\xA4t\xCB\xC2\x87\xC0\xA4E\x87#\x163\xA0w\xF0\xBD05MZK\x18\x99o\xCE3\x91\x80a\x11\t\x88\x88\x87\x84`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[\x03\x90\xA2s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x82`\x01T\x16\x92\x83;\x15a\x08\xFDW`@Q\x7F#\xB8r\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80\x82R3`\x04\x83\x01R0`$\x83\x01R`D\x82\x01\x87\x90R\x94\x83\x90\x82\x90`d\x90\x82\x90`\0\x90Z\xF1\x80\x15a\x08\xF1Wa\x11\xBBW[P`\x02T\x16\x92\x83;\x15a\x08\xFDW`@Q\x90\x81R3`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x86\x90R\x92\x81\x90\x84\x90\x81`\0\x81`d\x81\x01a\x08\x8CV[a\x11\xD1\x90\x83=\x85\x11a\x08\xEAWa\x08\xDA\x81\x83a\x1A\xBBV[P\x86a\x11\x83V[\x90a\x11\xE9\x82`\x04T`\x05T\x90a,\x1AV[\x90a\x10FV[4a\x02%W`\0`\x03\x196\x01\x12a\x02 W` g\r\xE0\xB6\xB3\xA7d\0\0`\x06T\x04`@Q\x90\x81R\xF3[4a\x02%W` `\x03\x196\x01\x12a\x02 W`\x045s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x80\x91\x03a\x12aW`\0R`\x07` R` `@`\0 T`@Q\x90\x81R\xF3[`\0\x80\xFD[4a\x02%W` \x80`\x03\x196\x01\x12a\x02 W`\x06T`\x045\x91a\x12\x8A\x82\x15\x15a \x98V[a\x12\xA5a\x12\x9A`\x04T\x85\x85a+\xB9V[\x92\x84`\x05T\x91a+\xB9V[\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80`\x01T\x16\x90\x81;\x15a\x08\xFDW`@Q\x7F#\xB8r\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80\x82R3`\x04\x83\x01R0`$\x83\x01R`D\x82\x01\x87\x90R\x92\x84\x90\x82\x90`d\x90\x82\x90`\0\x90Z\xF1\x80\x15a\x08\xF1Wa\x14)W[P`\x02T\x16\x90\x81;\x15a\x08\xFDW`@Q\x90\x81R3`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x84\x90R\x90\x82\x90\x82\x90`d\x90\x82\x90`\0\x90Z\xF1\x80\x15a\x08\xF1Wa\x14\x0CW[P`@\x80Q\x85\x81R` \x81\x01\x85\x90R\x90\x81\x01\x83\x90R3\x90\x7F\xBE\xB3\x88W\x86\xD67\xA4t\xCB\xC2\x87\xC0\xA4E\x87#\x163\xA0w\xF0\xBD05MZK\x18\x99o\xCE\x90``\x90\xA2`\x04Ta\x13\xA5\x84a\x1BaV[\x81\x01\x80\x91\x11a\x02\xA6W`\x04U`\x05Ta\x13\xBD\x83a\x1BaV[\x81\x01\x80\x91\x11a\x02\xA6W`\x05U3`\0R`\x07\x81R`@`\0 T\x84\x81\x01\x80\x91\x11a\x02\xA6W3`\0R`\x07\x82R`@`\0 U`\x06T\x93\x84\x01\x80\x94\x11a\x02\xA6W`@\x93`\x06U\x83Q\x92\x83R\x82\x01R\xF3[a\x14\"\x90\x82=\x84\x11a\x08\xEAWa\x08\xDA\x81\x83a\x1A\xBBV[P\x84a\x13\\V[a\x14?\x90\x84=\x86\x11a\x08\xEAWa\x08\xDA\x81\x83a\x1A\xBBV[P\x86a\x13\x1CV[4a\x02%Wa\x14T6a\x19\xC3V[\x90`\x08Tg\r\xE0\xB6\xB3\xA7d\0\0\x90\x80\x82\x03\x90\x82\x82\x11a\x02\xA6Wa\x02\x88\x84a\x02\x83a\x02\x8D\x94a\x14\x81\x98a\x1AoV[\0[4a\x02%W` `\x03\x196\x01\x12a\x02 W`\x045a\x14\xBAs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\0T\x163\x14a\x1A\xFCV[g\r\xE0\xB6\xB3\xA7d\0\0\x81\x11a\x14\xCEW`\x03U\0[`d`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x11`$\x82\x01R\x7FSwap fee too high\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R\xFD[4a\x02%Wa\x15:6a\x19\xA4V[a\x15Ba \xFDV[\x91a\x15Na\x05\xBCa \xFDV[\x91`\x04T\x92`\x05\x80Ta\x15na\x15d\x88\x88a!oV[a\x05\xC7\x85\x84a!oV[\x90g\r\xE0\xB6\xB3\xA7d\0\0a\x15\x8F\x81a\x15\x88`\x03T\x89a\x1AoV[\x04\x87a\x1AbV[\x87\x15a\x18&W\x88[\x88\x15a\x18\x1FW\x8A\x91[\x89\x15a\x18\x18W\x84\x91[\x8A\x15a\x18\x0EWa\x15\xB9\x89\x92a\x1BaV[\x81\x01\x93\x84\x82\x11a\x02\xA6Wa\t\xD0a\x05\xC7\x93a\x0Bha\x15\xDA\x97a\x05\xBC\x95a\x1B\xBFV[\x04\x96\x86\x15a\x17\xE5WPP`\x04Ta\x15\xF0\x85a\x1BaV[\x81\x01\x80\x91\x11a\x02\xA6W`\x04Ua\x16\n\x82Ta\x072\x88a\x1BaV[\x82U[a\x16#`\x04Ta\x05\xC7\x85a\x05\xC1\x8B\x87T\x94a!oV[\x80\x82\x11a\x0C.WPP\x83\x15a\x17\xC6Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x01T\x16\x95[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x97\x16\x96\x87;\x15a\x08\xFDW`@Q\x7F#\xB8r\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R3`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x86\x90R` \x98\x89\x90\x82\x90`d\x90\x82\x90`\0\x90Z\xF1\x80\x15a\x08\xF1Wa\x17\xA9W[P\x85\x15a\x17\x9FW\x80`\x02T\x16[\x16\x90\x81;\x15a\x08\xFDW`@Q\x7F\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R3`\x04\x82\x01R`$\x81\x01\x88\x90R\x91\x88\x90\x83\x90`D\x90\x82\x90`\0\x90Z\xF1\x80\x15a\x08\xF1Wa\x17<\x94a\x03\xED\x93a\x0Bh\x92a\x17\x82W[P`\x04T\x94Ta\x1B\xBFV[\x90`@Q\x92\x15\x15\x83R\x84\x83\x01R\x82`@\x83\x01R``\x82\x01R\x7F\x17fH\xF1\xF1\x1C\xDA(L\x12D\x90\x08k\xE4*\x92m\xDF\n\xE8\x87\xEB\xE7\xB1\xD6\xB37\xD8\x94'V`\x803\x92\xA2`@Q\x90\x81R\xF3[a\x17\x98\x90\x8B=\x8D\x11a\x08\xEAWa\x08\xDA\x81\x83a\x1A\xBBV[P\x8Aa\x171V[\x80`\x01T\x16a\x16\xCFV[a\x17\xBF\x90\x89=\x8B\x11a\x08\xEAWa\x08\xDA\x81\x83a\x1A\xBBV[P\x88a\x16\xC2V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x02T\x16\x95a\x16MV[a\x17\xF2\x90a\x072\x89a\x1BaV[`\x04Ua\x17\xFE\x85a\x1BaV[\x81\x01\x80\x91\x11a\x02\xA6W\x82Ua\x16\rV[a\x15\xB9\x8D\x92a\x1BaV[\x8A\x91a\x15\xA9V[\x86\x91a\x15\xA0V[\x82a\x15\x97V[4a\x02%W`\0`\x03\x196\x01\x12a\x02 W` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x01T\x16`@Q\x90\x81R\xF3[4a\x02%W`\0`\x03\x196\x01\x12a\x02 W` `\x06T`@Q\x90\x81R\xF3[4a\x18\x9CW`\0`\x03\x196\x01\x12a\x02 W` `\x04T`@Q\x90\x81R\xF3[`\x84`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R\xFD[`\x84`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01R\x7Frt\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R\xFD[`\x03\x19`@\x91\x01\x12a\x02 W`\x045\x80\x15\x15\x81\x03a\x12aW\x90`$5\x90V[`\x03\x19`@\x91\x01\x12a\x02 W`\x045\x90`$5\x90V[4a\x02%W` a\x03La\x19\xEC6a\x19\xC3V[\x90a\x1DEV[\x90g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x03\x91\x82\x11a\x02\xA6WV[\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC\x18\x82\x01\x91\x82\x11a\x02\xA6WV[\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xF2\x1FILX\x9C\0\0\x82\x01\x91\x82\x11a\x02\xA6WV[\x91\x90\x82\x03\x91\x82\x11a\x02\xA6WV[\x81\x81\x02\x92\x91\x81\x15\x91\x84\x04\x14\x17\x15a\x02\xA6WV[\x81\x15a\x1A\x8CW\x04\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[\x90`\x1F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x0F\xE3W`@RV[\x15a\x1B\x03WV[`d`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\t`$\x82\x01R\x7FNot admin\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R\xFD[x\x12r]\xD1\xD2C\xAB\xA0\xE7_\xE6E\xCCHs\xF9\xE6Z\xFEh\x8C\x92\x8E\x1F!\x81\x11a\x1B\x8EWg\r\xE0\xB6\xB3\xA7d\0\0\x02\x90V[`$\x90`@Q\x90\x7F\x1C\xD9Q\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\xFD[g\r\xE0\xB6\xB3\xA7d\0\0\x91\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x83\t\x92\x80\x83\x02\x92\x83\x80\x86\x10\x95\x03\x94\x80\x86\x03\x95\x14a\x1C\x99W\x82\x85\x10\x15a\x1C]W\x90\x82\x91\t`\x01\x82\x19\x01\x82\x16\x80\x92\x04`\x02\x80\x82`\x03\x02\x18\x80\x83\x02\x82\x03\x02\x80\x83\x02\x82\x03\x02\x80\x83\x02\x82\x03\x02\x80\x83\x02\x82\x03\x02\x80\x83\x02\x82\x03\x02\x80\x92\x02\x90\x03\x02\x93`\x01\x83\x80`\0\x03\x04\x01\x90\x84\x83\x11\x90\x03\x02\x92\x03\x04\x17\x02\x90V[\x82`d\x92`@Q\x92\x7Fc\xA0Wx\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01R`$\x83\x01R`D\x82\x01R\xFD[PP\x90a\x1C\xA6\x92Pa\x1A\x82V[\x90V[`\x84`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R\xFD[\x90\x81` \x91\x03\x12a\x02 WQ\x80\x15\x15\x81\x03a\x12aW\x90V[\x91\x90`\x06T\x90\x81a\x1E\xE0Wa\x1DY\x84a\x1BaV[\x90a\x1Dc\x81a\x1BaV[\x94a\x1D\x97a\x07ia\x1Ds\x83a\x1BaV[a\x05\xC7a\x1D~a \xFDV[\x91a\x05\xC1a\x1D\x8B\x88a\x1BaV[\x93a\x05\xC1a\x05\xBCa \xFDV[\x93\x84\x81\x01\x80\x91\x11a\x02\xA6W`\x06Ua\x1D\xAE\x84a\x1A\x08V[\x90`\0\x963\x88R` \x94`\x07\x86R`@\x93\x84\x8A U\x88\x80Ra\x03\xE8\x84\x8A U`\x04U`\x05Us\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80`\x01T\x16\x80;\x15a\x08\xFDW\x83Q\x7F#\xB8r\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80\x82R3`\x04\x83\x01R0`$\x83\x01R`D\x82\x01\x94\x90\x94R\x90\x86\x90\x82\x90`d\x90\x82\x90\x8D\x90Z\xF1\x80\x15a\x1E\xD6Wa\x1E\xB9W[P`\x02T\x16\x92\x83;\x15a\x08\xFDW\x82Q\x91\x82R3`\x04\x83\x01R0`$\x83\x01R`D\x82\x01R\x91\x83\x90\x83\x90`d\x90\x82\x90\x8A\x90Z\xF1\x90\x81\x15a\x1E\xB0WPa\x1C\xA6\x94\x95Pa\x1E\x92W[PPa\x1A\x08V[\x81a\x1E\xA8\x92\x90=\x10a\x08\xEAWa\x08\xDA\x81\x83a\x1A\xBBV[P8\x80a\x1E\x8BV[Q=\x87\x82>=\x90\xFD[a\x1E\xCF\x90\x86=\x88\x11a\x08\xEAWa\x08\xDA\x81\x83a\x1A\xBBV[P8a\x1EGV[\x84Q=\x8B\x82>=\x90\xFD[`d`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FPool already initialized\0\0\0\0\0\0\0\0`D\x82\x01R\xFD[\x90g\x1B\xC1mgN\xC8\0\0\x91\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x82\t\x83\x82\x02\x91\x82\x80\x83\x10\x92\x03\x91\x80\x83\x03\x92\x14a \x02Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x83\x10\x15a\x1F\xCBW\x94\x7F\xAC\xCB\x18\x16[\xD6\xFE1\xAE\x1C\xF3\x18\xDC[Q\xEE\xE0\xE1\xBAV\x9B\x88\xCDt\xC1w;\x91\xFA\xC1\x06i\x94\x95\t\x90\x82\x82\x11\x90\x03`\xEE\x1B\x91\x03`\x12\x1C\x17\x02\x90V[`D\x90\x86`@Q\x91\x7FQsd\x8D\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04\x83\x01R`$\x82\x01R\xFD[PPg\r\xE0\xB6\xB3\xA7d\0\0\x90\x04\x91PV[\x91\x90\x91\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x82\t\x83\x82\x02\x91\x82\x80\x83\x10\x92\x03\x91\x80\x83\x03\x92\x14a \x02Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x83\x10\x15a\x1F\xCBW\x94\x7F\xAC\xCB\x18\x16[\xD6\xFE1\xAE\x1C\xF3\x18\xDC[Q\xEE\xE0\xE1\xBAV\x9B\x88\xCDt\xC1w;\x91\xFA\xC1\x06i\x94\x95\t\x90\x82\x82\x11\x90\x03`\xEE\x1B\x91\x03`\x12\x1C\x17\x02\x90V[\x15a \x9FWV[`d`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x14`$\x82\x01R\x7FPool not initialized\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R\xFD[`\x0BTB\x10\x15a!GWa!\"a!\x19a\x03\xE8`\tTBa\x1AbV[`\x0CT\x90a \x13V[`\x08T\x90`\nT\x82\x11`\0\x14a!;Wa\x1C\xA6\x91a\x1AbV[\x81\x01\x80\x91\x11a\x02\xA6W\x90V[`\nT\x90V[a\x1C\xA6`\x04Ta\x03\xEDa!^a \xFDV[a\x0Bh`\x05Ta\x03\xEDa\x05\xBCa \xFDV[\x90\x81\x15\x80\x15a!\x8EWP`\0\x91Pa\x1C\xA6WPg\r\xE0\xB6\xB3\xA7d\0\0\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x80\x84\x14a\"\x1EWP\x81a!\xB3WPPPg\r\xE0\xB6\xB3\xA7d\0\0\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x14a\"\x19Wg\r\xE0\xB6\xB3\xA7d\0\0\x83\x11\x15a!\xE8WPa!\xE3\x90a\r\ra\x1C\xA6\x93a*\xAAV[a\"%V[a\x1A\x8CWa!\xE3a\"\x0E\x91a\r\rn\xC0\x97\xCE{\xC9\x07\x15\xB3K\x9F\x10\0\0\0\0\x94\x85\x04a*\xAAV[\x90\x81\x15a\x1A\x8CW\x04\x90V[PP\x90V[\x92PPP\x90V[h\nh\x89\x06\xBD\x8A\xFF\xFF\xFF\x81\x11a*yWg\r\xE0\xB6\xB3\xA7d\0\0\x80`@\x92\x83\x1B\x04\x90w\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\0\0\0\0\0\0\0\x83\x16a)\\W[f\xFF\0\0\0\0\0\0\x83\x16a(TW[e\xFF\0\0\0\0\0\x83\x16a'TW[d\xFF\0\0\0\0\x83\x16a&\\W[c\xFF\0\0\0\x83\x16a%lW[b\xFF\0\0\x83\x16a$\x84W[a\xFF\0\x83\x16a#\xA4W[`\xFF\x83\x16a\"\xCDW[\x02\x91\x1C`\xBF\x03\x1C\x90V[`\x80\x83\x16a#\x92W[\x83\x83\x16a#\x80W[` \x83\x16a#nW[`\x10\x83\x16a#\\W[`\x08\x83\x16a#JW[`\x04\x83\x16a#8W[`\x02\x83\x16a#&W[`\x01\x83\x16\x15a\"\xC3Wh\x01\0\0\0\0\0\0\0\x01\x02\x83\x1Ca\"\xC3V[h\x01\0\0\0\0\0\0\0\x01\x02\x83\x1Ca#\x0BV[h\x01\0\0\0\0\0\0\0\x03\x02\x83\x1Ca#\x02V[h\x01\0\0\0\0\0\0\0\x06\x02\x83\x1Ca\"\xF9V[h\x01\0\0\0\0\0\0\0\x0B\x02\x83\x1Ca\"\xF0V[h\x01\0\0\0\0\0\0\0\x16\x02\x83\x1Ca\"\xE7V[h\x01\0\0\0\0\0\0\0,\x02\x83\x1Ca\"\xDEV[h\x01\0\0\0\0\0\0\0Y\x02\x83\x1Ca\"\xD6V[a\x80\0\x83\x16a$rW[a@\0\x83\x16a$`W[a \0\x83\x16a$NW[a\x10\0\x83\x16a$<W[a\x08\0\x83\x16a$*W[a\x04\0\x83\x16a$\x18W[a\x02\0\x83\x16a$\x06W[a\x01\0\x83\x16\x15a\"\xBAWh\x01\0\0\0\0\0\0\0\xB1\x02\x83\x1Ca\"\xBAV[h\x01\0\0\0\0\0\0\x01c\x02\x83\x1Ca#\xEAV[h\x01\0\0\0\0\0\0\x02\xC6\x02\x83\x1Ca#\xE0V[h\x01\0\0\0\0\0\0\x05\x8C\x02\x83\x1Ca#\xD6V[h\x01\0\0\0\0\0\0\x0B\x17\x02\x83\x1Ca#\xCCV[h\x01\0\0\0\0\0\0\x16.\x02\x83\x1Ca#\xC2V[h\x01\0\0\0\0\0\0,]\x02\x83\x1Ca#\xB8V[h\x01\0\0\0\0\0\0X\xB9\x02\x83\x1Ca#\xAEV[b\x80\0\0\x83\x16a%ZW[b@\0\0\x83\x16a%HW[b \0\0\x83\x16a%6W[b\x10\0\0\x83\x16a%$W[b\x08\0\0\x83\x16a%\x12W[b\x04\0\0\x83\x16a%\0W[b\x02\0\0\x83\x16a$\xEEW[b\x01\0\0\x83\x16\x15a\"\xB0Wh\x01\0\0\0\0\0\0\xB1r\x02\x83\x1Ca\"\xB0V[h\x01\0\0\0\0\0\x01b\xE4\x02\x83\x1Ca$\xD1V[h\x01\0\0\0\0\0\x02\xC5\xC8\x02\x83\x1Ca$\xC6V[h\x01\0\0\0\0\0\x05\x8B\x91\x02\x83\x1Ca$\xBBV[h\x01\0\0\0\0\0\x0B\x17!\x02\x83\x1Ca$\xB0V[h\x01\0\0\0\0\0\x16.C\x02\x83\x1Ca$\xA5V[h\x01\0\0\0\0\0,\\\x86\x02\x83\x1Ca$\x9AV[h\x01\0\0\0\0\0X\xB9\x0C\x02\x83\x1Ca$\x8FV[c\x80\0\0\0\x83\x16a&JW[c@\0\0\0\x83\x16a&8W[c \0\0\0\x83\x16a&&W[c\x10\0\0\0\x83\x16a&\x14W[c\x08\0\0\0\x83\x16a&\x02W[c\x04\0\0\0\x83\x16a%\xF0W[c\x02\0\0\0\x83\x16a%\xDEW[c\x01\0\0\0\x83\x16\x15a\"\xA5Wh\x01\0\0\0\0\0\xB1r\x18\x02\x83\x1Ca\"\xA5V[h\x01\0\0\0\0\x01b\xE40\x02\x83\x1Ca%\xC0V[h\x01\0\0\0\0\x02\xC5\xC8`\x02\x83\x1Ca%\xB4V[h\x01\0\0\0\0\x05\x8B\x90\xC0\x02\x83\x1Ca%\xA8V[h\x01\0\0\0\0\x0B\x17!\x7F\x02\x83\x1Ca%\x9CV[h\x01\0\0\0\0\x16.B\xFF\x02\x83\x1Ca%\x90V[h\x01\0\0\0\0,\\\x85\xFE\x02\x83\x1Ca%\x84V[h\x01\0\0\0\0X\xB9\x0B\xFC\x02\x83\x1Ca%xV[d\x80\0\0\0\0\x83\x16a'BW[d@\0\0\0\0\x83\x16a'0W[d \0\0\0\0\x83\x16a'\x1EW[d\x10\0\0\0\0\x83\x16a'\x0CW[d\x08\0\0\0\0\x83\x16a&\xFAW[d\x04\0\0\0\0\x83\x16a&\xE8W[d\x02\0\0\0\0\x83\x16a&\xD6W[d\x01\0\0\0\0\x83\x16\x15a\"\x99Wh\x01\0\0\0\0\xB1r\x17\xF8\x02\x83\x1Ca\"\x99V[h\x01\0\0\0\x01b\xE4/\xF1\x02\x83\x1Ca&\xB7V[h\x01\0\0\0\x02\xC5\xC8_\xE3\x02\x83\x1Ca&\xAAV[h\x01\0\0\0\x05\x8B\x90\xBF\xCE\x02\x83\x1Ca&\x9DV[h\x01\0\0\0\x0B\x17!\x7F\xBB\x02\x83\x1Ca&\x90V[h\x01\0\0\0\x16.B\xFF\xF0\x02\x83\x1Ca&\x83V[h\x01\0\0\0,\\\x86\x01\xCC\x02\x83\x1Ca&vV[h\x01\0\0\0X\xB9\x0C\x0BI\x02\x83\x1Ca&iV[e\x80\0\0\0\0\0\x83\x16a(BW[e@\0\0\0\0\0\x83\x16a(0W[e \0\0\0\0\0\x83\x16a(\x1EW[e\x10\0\0\0\0\0\x83\x16a(\x0CW[e\x08\0\0\0\0\0\x83\x16a'\xFAW[e\x04\0\0\0\0\0\x83\x16a'\xE8W[e\x02\0\0\0\0\0\x83\x16a'\xD6W[e\x01\0\0\0\0\0\x83\x16\x15a\"\x8CWh\x01\0\0\0\xB1r\x185Q\x02\x83\x1Ca\"\x8CV[h\x01\0\0\x01b\xE40\xE5\xA2\x02\x83\x1Ca'\xB6V[h\x01\0\0\x02\xC5\xC8c\xB7?\x02\x83\x1Ca'\xA8V[h\x01\0\0\x05\x8B\x90\xCF\x1En\x02\x83\x1Ca'\x9AV[h\x01\0\0\x0B\x17!\xBC\xFC\x9A\x02\x83\x1Ca'\x8CV[h\x01\0\0\x16.C\xF4\xF81\x02\x83\x1Ca'~V[h\x01\0\0,\\\x89\xD5\xECm\x02\x83\x1Ca'pV[h\x01\0\0X\xB9\x1B[\xC9\xAE\x02\x83\x1Ca'bV[f\x80\0\0\0\0\0\0\x83\x16a)JW[f@\0\0\0\0\0\0\x83\x16a)8W[f \0\0\0\0\0\0\x83\x16a)&W[f\x10\0\0\0\0\0\0\x83\x16a)\x14W[f\x08\0\0\0\0\0\0\x83\x16a)\x02W[f\x04\0\0\0\0\0\0\x83\x16a(\xF0W[f\x02\0\0\0\0\0\0\x83\x16a(\xDEW[f\x01\0\0\0\0\0\0\x83\x16\x15a\"~Wh\x01\0\0\xB1rUw\\\x04\x02\x83\x1Ca\"~V[h\x01\0\x01b\xE5%\xEE\x05G\x02\x83\x1Ca(\xBDV[h\x01\0\x02\xC5\xCC7\xDA\x94\x92\x02\x83\x1Ca(\xAEV[h\x01\0\x05\x8B\xA0\x1F\xB9\xF9m\x02\x83\x1Ca(\x9FV[h\x01\0\x0B\x17^\xFF\xDCv\xBA\x02\x83\x1Ca(\x90V[h\x01\0\x16/9\x04\x05\x1F\xA1\x02\x83\x1Ca(\x81V[h\x01\0,`^.\x8C\xECP\x02\x83\x1Ca(rV[h\x01\0X\xC8m\xA1\xC0\x9E\xA2\x02\x83\x1Ca(cV[g\x80\0\0\0\0\0\0\0\x83\x16a*ZW[g@\0\0\0\0\0\0\0\x83\x16a*HW[g \0\0\0\0\0\0\0\x83\x16a*6W[g\x10\0\0\0\0\0\0\0\x83\x16a*$W[g\x08\0\0\0\0\0\0\0\x83\x16a*\x12W[g\x04\0\0\0\0\0\0\0\x83\x16a*\0W[g\x02\0\0\0\0\0\0\0\x83\x16a)\xEEW[g\x01\0\0\0\0\0\0\0\x83\x16\x15a\"oWh\x01\0\xB1\xAF\xA5\xAB\xCB\xEDa\x02\x83\x1Ca\"oV[h\x01\x01c\xDA\x9F\xB33V\xD8\x02\x83\x1Ca)\xCCV[h\x01\x02\xC9\xA3\xE7x\x06\x0E\xE7\x02\x83\x1Ca)\xBCV[h\x01\x05\x9B\r1XWC\xAE\x02\x83\x1Ca)\xACV[h\x01\x0BU\x86\xCF\x98\x90\xF6*\x02\x83\x1Ca)\x9CV[h\x01\x17+\x83\xC7\xD5\x17\xAD\xCE\x02\x83\x1Ca)\x8CV[h\x010o\xE0\xA3\x1BqR\xDF\x02\x83\x1Ca)|V[Pw\xB5\x04\xF33\xF9\xDEd\x84\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a)lV[`$\x90`@Q\x90\x7F\xB3\xB6\xBA\x1F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\xFD[\x90g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x81\x10a+\x88W\x82\x81\x04o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11`\x07\x1B\x90\x81\x1C\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x06\x1B\x91\x82\x1Cc\xFF\xFF\xFF\xFF\x81\x11`\x05\x1B\x90\x81\x1Ca\xFF\xFF\x81\x11`\x04\x1B\x90\x81\x1C`\xFF\x81\x11`\x03\x1B\x90\x81\x1C\x91`\x0F\x83\x11`\x02\x1B\x92\x83\x1C\x93`\x01\x96\x87`\x03\x87\x11\x81\x1B\x96\x87\x1C\x11\x96\x17\x17\x17\x17\x17\x17\x17\x91\x84\x83\x02\x92\x1C\x93\x80\x85\x14a+\x81Wg\x06\xF0[Y\xD3\xB2\0\0\x94\x85[a+TWP\x91\x93PPPV[\x80\x82\x91\x02\x04\x94g\x1B\xC1mgN\xC8\0\0\x86\x10\x15a+tW[\x82\x1C\x94\x85a+HV[\x80\x95\x93\x01\x92\x82\x1C\x94a+kV[P\x90\x92PPV[`$\x90`@Q\x90\x7F6\xD3.\xF0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\xFD[\x91\x90\x82\x01\x90\x81\x83\x11a\x02\xA6Wa\r\ra\t\xD6g\r\xE0\xB6\xB3\xA7d\0\0\x94a+\xDE\x94a\x1B\xBFV[\x04\x90V[\x90a+\xF6a+\xF0\x83\x83a\x1B\xBFV[\x93a\x1BaV[\x82\x01\x80\x92\x11a\x02\xA6Wa,\x15a+\xDE\x92g\r\xE0\xB6\xB3\xA7d\0\0\x94a \x13V[a\x1AbV[a+\xF6a+\xF0\x83\x83a\x1B\xBFV";
    /// The bytecode of the contract.
    pub static G3M_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10\x15a\0\x92W[`\x84`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01R\x7Fnor receive functions\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R\xFD[`\x005`\xE0\x1C\x80c\x08\xEA\xBD\xDA\x14a\x18~W\x80c\x15w\x0F\x92\x14a\x18`W\x80c\x16\xDC\x16[\x14a\x18,W\x80c\x194\xEB%\x14a\x01\xF8W\x80c\x1F\xDA\xBC'\x14a\x15,W\x80c4\xE1\x99\x07\x14a\x14\x83W\x80c69\xAA2\x14a\x14FW\x80cQ\xC6Y\n\x14a\x12fW\x80cT\xCF*\xEB\x14a\x05^W\x80cU\x9D\x16\x02\x14a\x03\tW\x80cp\xA0\x821\x14a\x12\x17W\x80cu\xAE\xE1\xDA\x14a\x11\xEFW\x80cvp\x166\x14a\x10\x12W\x80c\x84\x89PO\x14a\t4W\x80c\x87kU\xF1\x14a\x0F+W\x80c\x8AYS\xC7\x14a\x0F\rW\x80c\x9C\x8F\x9F#\x14a\x0C\xB6W\x80c\x9C\xE32\xD4\x14a\t9W\x80c\x9E\x1B\0E\x14a\t4W\x80c\xA0\xDBj\x82\x14a\x06\xF7W\x80c\xAD\xB5\x1D\xEE\x14a\x06\xDCW\x80c\xB7\xD1\x9F\xC4\x14a\x06\xA8W\x80c\xBC\xC1}\xC7\x14a\x05\xD6W\x80c\xC0\xFF\x1A\x15\x14a\x05|W\x80c\xD4\xCA\xDFh\x14a\x05^W\x80c\xDBy\x10C\x14a\x03TW\x80c\xDCv\xFA\xBC\x14a\x031W\x80c\xE3\x11\xCE\xC8\x14a\x03\tW\x80c\xF8Q\xA4@\x14a\x02\xD5W\x80c\xF9\xA1\xC8Z\x14a\x02HW\x80c\xFA\xDF\xA6[\x14a\x02*Wc\xFE\xD3\xDF\xDA\x03a\0\x0EW[4a\x02%W`\0`\x03\x196\x01\x12a\x02 W` g\r\xE0\xB6\xB3\xA7d\0\0`\x05T\x04`@Q\x90\x81R\xF3[a\x19 V[a\x18\x9CV[4a\x02%W`\0`\x03\x196\x01\x12a\x02 W` `\x05T`@Q\x90\x81R\xF3[4a\x02%Wa\x02V6a\x19\xC3V[\x90`\x08Tg\r\xE0\xB6\xB3\xA7d\0\0\x90\x80\x82\x03\x90\x82\x82\x11a\x02\xA6Wa\x02\x88\x84a\x02\x83a\x02\x8D\x94a\x02\x94\x98a\x1AoV[a\x1AoV[a\x1A\x82V[\x04\x90a\x1DEV[P`@\x80Q`\0\x81R`\0` \x82\x01R\xF3[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[4a\x02%W`\0`\x03\x196\x01\x12a\x02 W` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\0T\x16`@Q\x90\x81R\xF3[4a\x02%W`\0`\x03\x196\x01\x12a\x02 W` g\r\xE0\xB6\xB3\xA7d\0\0`\x04T\x04`@Q\x90\x81R\xF3[4a\x02%W`\0`\x03\x196\x01\x12a\x02 W` a\x03La!MV[`@Q\x90\x81R\xF3[4a\x02%W`@`\x03\x196\x01\x12a\x02 W`\x045`$5\x90a\x03\x8Fs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\0T\x163\x14a\x1A\xFCV[f#\x86\xF2o\xC1\0\0\x81\x10a\x05\0Wg\r\xBD/\xC17\xA3\0\0\x81\x11a\x04\xA2WB\x82\x11\x15a\x04DWa\x03\xF3a\x03\xBFa \xFDV[\x80`\x08UB`\tU\x82\x80\x82\x11`\0\x14a\x045Wa\x03\xDB\x91a\x1AbV[a\x03\xEDa\x03\xE8B\x86a\x1AbV[a\x1BaV[\x90a\x1B\xBFV[`\x0CU`\nU`\x0BU\x7F\x91\t\xD5\xCC\xAE\x12\x89A\xC9q\x94\xA8t\xB03\xDE'l\x81#\xF7!\xAC\x0F\x18\x17M\x11r\xBEj\xC2`@a\x04'a \xFDV[\x81Q\x90B\x82R` \x82\x01R\xA1\0[\x90a\x04?\x91a\x1AbV[a\x03\xDBV[`d`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x11`$\x82\x01R\x7FUpdate end pasted\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R\xFD[`d`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x11`$\x82\x01R\x7FWeight X too high\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R\xFD[`d`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x10`$\x82\x01R\x7FWeight X too low\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R\xFD[4a\x02%W`\0`\x03\x196\x01\x12a\x02 W` `\x03T`@Q\x90\x81R\xF3[4a\x02%W`\0`\x03\x196\x01\x12a\x02 W` g\r\xE0\xB6\xB3\xA7d\0\0a\x05\xCD`\x04Ta\x05\xC7a\x05\xA9a \xFDV[\x91a\x05\xC1`\x05T\x93a\x05\xC1a\x05\xBCa \xFDV[a\x19\xF2V[\x92a!oV[\x90a \x13V[\x04`@Q\x90\x81R\xF3[4a\x02%W`\0`\x03\x196\x01\x12a\x02 W\x7F\x90\x84\x9A\xC6O\xD5x\x0E\xD6\xE7\xD5\x9E\xBC\xAD\xA1\xFA\xED!\xB8y\x1A\xE1z\x1C\xA8\xF555\x8E\xF5%\xD2`@a\x06\x12a!MV[\x81Q\x90\x81RB` \x82\x01R\xA1`\x04T`\x05T`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91RB\x90\x82\x01R\x7F\x94a\x90\xF3N\x99(\xE7m\x01\xD1,\xBFP7\xB1~\xDAr=\x983\xCE\xF5R$\x97Do\x7F\x81\xD5\x90``\x90\xA1\x7F+\x8E\xE9\x04\xF2\xA9_()\x9E\x1D\x8Et\xB4\xEF \xD1;\xFC\rHxL(\xE5\x0F\xEC\xC1\xD5X\xDE\xF0``a\x06\x89a \xFDV[a\x06\x94a\x05\xBCa \xFDV[`@Q\x91\x82R` \x82\x01RB`@\x82\x01R\xA1\0[4a\x02%W`\0`\x03\x196\x01\x12a\x02 W` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x02T\x16`@Q\x90\x81R\xF3[4a\x02%W`\0`\x03\x196\x01\x12a\x02 W` a\x03La \xFDV[4a\x02%Wa\x07\x056a\x19\xA4V[\x90\x15a\t\x1FWa\x07\x1B\x81`\x04T`\x05T\x90a+\xE2V[\x90[a\x07na\x07ia\x078`\x04Ta\x072\x85a\x1BaV[\x90a\x1AbV[\x80`\x04Ua\x05\xC7a\x07N`\x05Ta\x072\x88a\x1BaV[\x91\x82`\x05Ua\x05\xC1a\x07^a \xFDV[a\x05\xC1a\x05\xBCa \xFDV[a\x1F>V[a\x07z\x81`\x06Ta\x1AbV[\x90`\x06U3`\0R` \x90`\x07\x82Ra\x07\x98\x81`@`\0 Ta\x1AbV[3`\0R`\x07\x83R`@`\0 U`@Q\x7FY\xC3\xA0\xB6\x0Cj\xB7\xDE\xB6.\x14@\xC9\xE7$A\xDBm\xB7\xDF\xE5\x14\xDB\xA8\xCB\x18\xE6\x0C\r\x89n\xFA3\x91\x80a\x07\xEA\x88\x88\x87\x84`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[\x03\x90\xA2s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x82`\x01T\x16\x92\x83;\x15a\x08\xFDW`@Q\x7F\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80\x82R3`\x04\x83\x01R`$\x82\x01\x87\x90R\x94\x83\x90\x82\x90`D\x90\x82\x90`\0\x90Z\xF1\x80\x15a\x08\xF1Wa\t\x02W[P`\x02T\x16\x92\x83;\x15a\x08\xFDW`@Q\x90\x81R3`\x04\x82\x01R`$\x81\x01\x86\x90R\x92\x81\x90\x84\x90\x81`\0\x81`D\x81\x01[\x03\x92Z\xF1\x92\x83\x15a\x08\xF1Wa\x08\xBF\x93a\x08\xC3W[PP`@Q\x93\x84\x93\x84`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[\x03\x90\xF3[\x81a\x08\xE2\x92\x90=\x10a\x08\xEAW[a\x08\xDA\x81\x83a\x1A\xBBV[\x81\x01\x90a\x1D-V[P\x84\x80a\x08\xA0V[P=a\x08\xD0V[`@Q=`\0\x82>=\x90\xFD[a\x1C\xA9V[a\t\x18\x90\x83=\x85\x11a\x08\xEAWa\x08\xDA\x81\x83a\x1A\xBBV[P\x86a\x08^V[a\t/\x81`\x04T`\x05T\x90a,\x1AV[a\x07\x1DV[a\x19\xD9V[4a\x02%Wa\tG6a\x19\xA4V[a\tOa \xFDV[\x91a\t[a\x05\xBCa \xFDV[\x91`\x04T\x92`\x05Ta\tza\tp\x87\x87a!oV[a\x05\xC7\x84\x84a!oV[\x90\x84\x15a\x0C\xB0W\x85[\x85\x15a\x0C\xA9W\x87\x90[\x86\x15a\x0C\x9FWa\t\xDB\x83\x91[\x88\x15a\x0C\x8FWa\x05\xC7a\t\xD6a\t\xBC\x89\x95[a\x03\xEDa\t\xB6\x8Da\x1BaV[\x82a\x1AbV[a\t\xD0g\r\xE0\xB6\xB3\xA7d\0\0\x97\x88\x97a\x1B\xBFV[\x90a!oV[a\x1A5V[\x04\x90\x80\x82\x02\x91\x80\x83\x04\x82\x14\x90\x15\x17\x15a\x02\xA6W`\x03T\x81\x03\x90\x81\x11a\x02\xA6Wa\n\x03\x91a\x1A\x82V[\x95\x85\x15a\x0CeWPP`\x04Ta\n\x18\x86a\x1BaV[\x81\x01\x80\x91\x11a\x02\xA6W`\x04Ua\n3`\x05Ta\x072\x85a\x1BaV[`\x05U[a\nN`\x04Ta\x05\xC7\x84a\x05\xC1\x8A`\x05T\x94a!oV[\x80\x82\x11a\x0C.WPP\x82\x15a\x0C\x0FWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80`\x01T\x16[\x16\x80;\x15a\x08\xFDW`@Q\x7F#\xB8r\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R3`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x86\x90R\x90` \x90\x82\x90`d\x90\x82\x90`\0\x90Z\xF1\x80\x15a\x08\xF1Wa\x0B\xF0W[P\x82\x15a\x0B\xD1Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80`\x02T\x16[\x16\x90\x81;\x15a\x08\xFDW`@Q\x7F\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R3`\x04\x82\x01R`$\x81\x01\x84\x90R\x91` \x90\x83\x90`D\x90\x82\x90`\0\x90Z\xF1\x80\x15a\x08\xF1W` \x96a\x0Bn\x93a\x03\xED\x92a\x0B\xB4W[Pa\x0Bh`\x04T\x93`\x05Ta\x1B\xBFV[\x92a\x1B\xBFV[\x90`@Q\x92\x15\x15\x83R\x83\x85\x84\x01R`@\x83\x01R``\x82\x01R\x7F\x17fH\xF1\xF1\x1C\xDA(L\x12D\x90\x08k\xE4*\x92m\xDF\n\xE8\x87\xEB\xE7\xB1\xD6\xB37\xD8\x94'V`\x803\x92\xA2`@Q\x90\x81R\xF3[a\x0B\xCA\x90\x89=\x8B\x11a\x08\xEAWa\x08\xDA\x81\x83a\x1A\xBBV[P\x88a\x0BXV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80`\x01T\x16a\n\xF6V[a\x0C\x08\x90` =` \x11a\x08\xEAWa\x08\xDA\x81\x83a\x1A\xBBV[P\x85a\n\xD4V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80`\x02T\x16a\nxV[`D\x92P`@Q\x91\x7F\xB2!\xD0\xE4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04\x83\x01R`$\x82\x01R\xFD[a\x0Cr\x90a\x072\x86a\x1BaV[`\x04Ua\x0C~\x86a\x1BaV[\x81\x01\x80\x91\x11a\x02\xA6W`\x05Ua\n7V[a\x05\xC7a\t\xD6a\t\xBC\x8D\x95a\t\xAAV[a\t\xDB\x88\x91a\t\x98V[\x83\x90a\t\x8CV[\x80a\t\x83V[4a\x02%W` \x80`\x03\x196\x01\x12a\x02 W`\x045\x903`\0R`\x07\x81R\x81`@`\0 T\x10a\x0E\xAFW`\x06T\x90g\r\xE0\xB6\xB3\xA7d\0\0a\r(\x81a\r\x12`\x04Ta\r\ra\x05\xBC\x88a\r\x08\x8B\x82a\x1AbV[a\x1B\xBFV[a \x13V[\x04\x93a\r\ra\x05\xBC`\x05T\x92a\r\x08\x89\x82a\x1AbV[\x04\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93\x84`\x01T\x16\x94\x85;\x15a\x08\xFDW`@Q\x7F\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80\x82R3`\x04\x83\x01R`$\x82\x01\x87\x90R\x96\x84\x90\x82\x90`D\x90\x82\x90`\0\x90Z\xF1\x80\x15a\x08\xF1Wa\x0E\x92W[P`\x02T\x16\x94\x85;\x15a\x08\xFDW`@Q\x90\x81R3`\x04\x82\x01R`$\x81\x01\x84\x90R\x94\x82\x90\x86\x90`D\x90\x82\x90`\0\x90Z\xF1\x94\x85\x15a\x08\xF1W`@\x95a\x0EuW[P3`\0R`\x07\x82Ra\r\xF0\x81\x86`\0 Ta\x1AbV[3`\0R`\x07\x83R\x85`\0 Ua\x0E\t\x81`\x06Ta\x1AbV[`\x06Ua\x0E\x1B`\x04Ta\x072\x86a\x1BaV[`\x04Ua\x0E-`\x05Ta\x072\x85a\x1BaV[`\x05U\x84Q\x90\x81R` \x81\x01\x84\x90R`@\x81\x01\x83\x90R3\x90\x7FY\xC3\xA0\xB6\x0Cj\xB7\xDE\xB6.\x14@\xC9\xE7$A\xDBm\xB7\xDF\xE5\x14\xDB\xA8\xCB\x18\xE6\x0C\r\x89n\xFA\x90``\x90\xA2\x83Q\x92\x83R\x82\x01R\xF3[a\x0E\x8B\x90\x83=\x85\x11a\x08\xEAWa\x08\xDA\x81\x83a\x1A\xBBV[P\x85a\r\xD9V[a\x0E\xA8\x90\x84=\x86\x11a\x08\xEAWa\x08\xDA\x81\x83a\x1A\xBBV[P\x86a\r\x9BV[`d\x90`@Q\x90\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R`\x16`$\x82\x01R\x7FInsufficient liquidity\0\0\0\0\0\0\0\0\0\0`D\x82\x01R\xFD[4a\x02%W`\0`\x03\x196\x01\x12a\x02 W` a\x03La\x05\xBCa \xFDV[4a\x02%W`\0`\x03\x196\x01\x12a\x02 Wa\x0FDa \xFDV[a\x0FOa\x05\xBCa \xFDV[`@Q\x90` \x92\x83\x83\x01R`@\x82\x01R`@\x81R``\x81\x01\x90\x80\x82\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x17a\x0F\xE3W\x81`@R\x82\x82R\x80Q\x92\x83`\x80\x83\x01R`\0[\x84\x81\x10a\x0F\xCFW\x83`@\x84\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x89`\0`\xA0\x82\x86\x01\x01R\x01\x16\x81\x01\x03\x01\x90\xF3[\x80\x82\x91\x84\x01`\xA0\x83\x82\x01Q\x91\x01R\x01a\x0F\x8EV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[4a\x02%Wa\x10 6a\x19\xA4V[`\x06T\x91a\x10/\x83\x15\x15a \x98V[\x15a\x11\xD8Wa\x10D\x81`\x04T`\x05T\x90a+\xE2V[\x91[`\x04Ta\x10R\x83a\x1BaV[\x81\x01\x80\x91\x11a\x02\xA6W\x80`\x04U`\x05Ta\x10k\x85a\x1BaV[\x81\x01\x80\x91\x11a\x02\xA6Wa\x07i\x81a\x05\xC7a\x10\x98\x94a\x10\x91\x94`\x05Ua\x05\xC1a\x07^a \xFDV[\x91\x82a\x1AbV[\x90`\x06U3`\0R` \x90`\x07\x82R`@`\0 T\x81\x81\x01\x80\x91\x11a\x02\xA6W3`\0R`\x07\x83R`@`\0 U`@Q\x7F\xBE\xB3\x88W\x86\xD67\xA4t\xCB\xC2\x87\xC0\xA4E\x87#\x163\xA0w\xF0\xBD05MZK\x18\x99o\xCE3\x91\x80a\x11\t\x88\x88\x87\x84`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[\x03\x90\xA2s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x82`\x01T\x16\x92\x83;\x15a\x08\xFDW`@Q\x7F#\xB8r\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80\x82R3`\x04\x83\x01R0`$\x83\x01R`D\x82\x01\x87\x90R\x94\x83\x90\x82\x90`d\x90\x82\x90`\0\x90Z\xF1\x80\x15a\x08\xF1Wa\x11\xBBW[P`\x02T\x16\x92\x83;\x15a\x08\xFDW`@Q\x90\x81R3`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x86\x90R\x92\x81\x90\x84\x90\x81`\0\x81`d\x81\x01a\x08\x8CV[a\x11\xD1\x90\x83=\x85\x11a\x08\xEAWa\x08\xDA\x81\x83a\x1A\xBBV[P\x86a\x11\x83V[\x90a\x11\xE9\x82`\x04T`\x05T\x90a,\x1AV[\x90a\x10FV[4a\x02%W`\0`\x03\x196\x01\x12a\x02 W` g\r\xE0\xB6\xB3\xA7d\0\0`\x06T\x04`@Q\x90\x81R\xF3[4a\x02%W` `\x03\x196\x01\x12a\x02 W`\x045s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x80\x91\x03a\x12aW`\0R`\x07` R` `@`\0 T`@Q\x90\x81R\xF3[`\0\x80\xFD[4a\x02%W` \x80`\x03\x196\x01\x12a\x02 W`\x06T`\x045\x91a\x12\x8A\x82\x15\x15a \x98V[a\x12\xA5a\x12\x9A`\x04T\x85\x85a+\xB9V[\x92\x84`\x05T\x91a+\xB9V[\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80`\x01T\x16\x90\x81;\x15a\x08\xFDW`@Q\x7F#\xB8r\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80\x82R3`\x04\x83\x01R0`$\x83\x01R`D\x82\x01\x87\x90R\x92\x84\x90\x82\x90`d\x90\x82\x90`\0\x90Z\xF1\x80\x15a\x08\xF1Wa\x14)W[P`\x02T\x16\x90\x81;\x15a\x08\xFDW`@Q\x90\x81R3`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x84\x90R\x90\x82\x90\x82\x90`d\x90\x82\x90`\0\x90Z\xF1\x80\x15a\x08\xF1Wa\x14\x0CW[P`@\x80Q\x85\x81R` \x81\x01\x85\x90R\x90\x81\x01\x83\x90R3\x90\x7F\xBE\xB3\x88W\x86\xD67\xA4t\xCB\xC2\x87\xC0\xA4E\x87#\x163\xA0w\xF0\xBD05MZK\x18\x99o\xCE\x90``\x90\xA2`\x04Ta\x13\xA5\x84a\x1BaV[\x81\x01\x80\x91\x11a\x02\xA6W`\x04U`\x05Ta\x13\xBD\x83a\x1BaV[\x81\x01\x80\x91\x11a\x02\xA6W`\x05U3`\0R`\x07\x81R`@`\0 T\x84\x81\x01\x80\x91\x11a\x02\xA6W3`\0R`\x07\x82R`@`\0 U`\x06T\x93\x84\x01\x80\x94\x11a\x02\xA6W`@\x93`\x06U\x83Q\x92\x83R\x82\x01R\xF3[a\x14\"\x90\x82=\x84\x11a\x08\xEAWa\x08\xDA\x81\x83a\x1A\xBBV[P\x84a\x13\\V[a\x14?\x90\x84=\x86\x11a\x08\xEAWa\x08\xDA\x81\x83a\x1A\xBBV[P\x86a\x13\x1CV[4a\x02%Wa\x14T6a\x19\xC3V[\x90`\x08Tg\r\xE0\xB6\xB3\xA7d\0\0\x90\x80\x82\x03\x90\x82\x82\x11a\x02\xA6Wa\x02\x88\x84a\x02\x83a\x02\x8D\x94a\x14\x81\x98a\x1AoV[\0[4a\x02%W` `\x03\x196\x01\x12a\x02 W`\x045a\x14\xBAs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\0T\x163\x14a\x1A\xFCV[g\r\xE0\xB6\xB3\xA7d\0\0\x81\x11a\x14\xCEW`\x03U\0[`d`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x11`$\x82\x01R\x7FSwap fee too high\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R\xFD[4a\x02%Wa\x15:6a\x19\xA4V[a\x15Ba \xFDV[\x91a\x15Na\x05\xBCa \xFDV[\x91`\x04T\x92`\x05\x80Ta\x15na\x15d\x88\x88a!oV[a\x05\xC7\x85\x84a!oV[\x90g\r\xE0\xB6\xB3\xA7d\0\0a\x15\x8F\x81a\x15\x88`\x03T\x89a\x1AoV[\x04\x87a\x1AbV[\x87\x15a\x18&W\x88[\x88\x15a\x18\x1FW\x8A\x91[\x89\x15a\x18\x18W\x84\x91[\x8A\x15a\x18\x0EWa\x15\xB9\x89\x92a\x1BaV[\x81\x01\x93\x84\x82\x11a\x02\xA6Wa\t\xD0a\x05\xC7\x93a\x0Bha\x15\xDA\x97a\x05\xBC\x95a\x1B\xBFV[\x04\x96\x86\x15a\x17\xE5WPP`\x04Ta\x15\xF0\x85a\x1BaV[\x81\x01\x80\x91\x11a\x02\xA6W`\x04Ua\x16\n\x82Ta\x072\x88a\x1BaV[\x82U[a\x16#`\x04Ta\x05\xC7\x85a\x05\xC1\x8B\x87T\x94a!oV[\x80\x82\x11a\x0C.WPP\x83\x15a\x17\xC6Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x01T\x16\x95[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x97\x16\x96\x87;\x15a\x08\xFDW`@Q\x7F#\xB8r\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R3`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x86\x90R` \x98\x89\x90\x82\x90`d\x90\x82\x90`\0\x90Z\xF1\x80\x15a\x08\xF1Wa\x17\xA9W[P\x85\x15a\x17\x9FW\x80`\x02T\x16[\x16\x90\x81;\x15a\x08\xFDW`@Q\x7F\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R3`\x04\x82\x01R`$\x81\x01\x88\x90R\x91\x88\x90\x83\x90`D\x90\x82\x90`\0\x90Z\xF1\x80\x15a\x08\xF1Wa\x17<\x94a\x03\xED\x93a\x0Bh\x92a\x17\x82W[P`\x04T\x94Ta\x1B\xBFV[\x90`@Q\x92\x15\x15\x83R\x84\x83\x01R\x82`@\x83\x01R``\x82\x01R\x7F\x17fH\xF1\xF1\x1C\xDA(L\x12D\x90\x08k\xE4*\x92m\xDF\n\xE8\x87\xEB\xE7\xB1\xD6\xB37\xD8\x94'V`\x803\x92\xA2`@Q\x90\x81R\xF3[a\x17\x98\x90\x8B=\x8D\x11a\x08\xEAWa\x08\xDA\x81\x83a\x1A\xBBV[P\x8Aa\x171V[\x80`\x01T\x16a\x16\xCFV[a\x17\xBF\x90\x89=\x8B\x11a\x08\xEAWa\x08\xDA\x81\x83a\x1A\xBBV[P\x88a\x16\xC2V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x02T\x16\x95a\x16MV[a\x17\xF2\x90a\x072\x89a\x1BaV[`\x04Ua\x17\xFE\x85a\x1BaV[\x81\x01\x80\x91\x11a\x02\xA6W\x82Ua\x16\rV[a\x15\xB9\x8D\x92a\x1BaV[\x8A\x91a\x15\xA9V[\x86\x91a\x15\xA0V[\x82a\x15\x97V[4a\x02%W`\0`\x03\x196\x01\x12a\x02 W` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x01T\x16`@Q\x90\x81R\xF3[4a\x02%W`\0`\x03\x196\x01\x12a\x02 W` `\x06T`@Q\x90\x81R\xF3[4a\x18\x9CW`\0`\x03\x196\x01\x12a\x02 W` `\x04T`@Q\x90\x81R\xF3[`\x84`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R\xFD[`\x84`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01R\x7Frt\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R\xFD[`\x03\x19`@\x91\x01\x12a\x02 W`\x045\x80\x15\x15\x81\x03a\x12aW\x90`$5\x90V[`\x03\x19`@\x91\x01\x12a\x02 W`\x045\x90`$5\x90V[4a\x02%W` a\x03La\x19\xEC6a\x19\xC3V[\x90a\x1DEV[\x90g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x03\x91\x82\x11a\x02\xA6WV[\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC\x18\x82\x01\x91\x82\x11a\x02\xA6WV[\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xF2\x1FILX\x9C\0\0\x82\x01\x91\x82\x11a\x02\xA6WV[\x91\x90\x82\x03\x91\x82\x11a\x02\xA6WV[\x81\x81\x02\x92\x91\x81\x15\x91\x84\x04\x14\x17\x15a\x02\xA6WV[\x81\x15a\x1A\x8CW\x04\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[\x90`\x1F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x0F\xE3W`@RV[\x15a\x1B\x03WV[`d`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\t`$\x82\x01R\x7FNot admin\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R\xFD[x\x12r]\xD1\xD2C\xAB\xA0\xE7_\xE6E\xCCHs\xF9\xE6Z\xFEh\x8C\x92\x8E\x1F!\x81\x11a\x1B\x8EWg\r\xE0\xB6\xB3\xA7d\0\0\x02\x90V[`$\x90`@Q\x90\x7F\x1C\xD9Q\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\xFD[g\r\xE0\xB6\xB3\xA7d\0\0\x91\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x83\t\x92\x80\x83\x02\x92\x83\x80\x86\x10\x95\x03\x94\x80\x86\x03\x95\x14a\x1C\x99W\x82\x85\x10\x15a\x1C]W\x90\x82\x91\t`\x01\x82\x19\x01\x82\x16\x80\x92\x04`\x02\x80\x82`\x03\x02\x18\x80\x83\x02\x82\x03\x02\x80\x83\x02\x82\x03\x02\x80\x83\x02\x82\x03\x02\x80\x83\x02\x82\x03\x02\x80\x83\x02\x82\x03\x02\x80\x92\x02\x90\x03\x02\x93`\x01\x83\x80`\0\x03\x04\x01\x90\x84\x83\x11\x90\x03\x02\x92\x03\x04\x17\x02\x90V[\x82`d\x92`@Q\x92\x7Fc\xA0Wx\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01R`$\x83\x01R`D\x82\x01R\xFD[PP\x90a\x1C\xA6\x92Pa\x1A\x82V[\x90V[`\x84`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R\xFD[\x90\x81` \x91\x03\x12a\x02 WQ\x80\x15\x15\x81\x03a\x12aW\x90V[\x91\x90`\x06T\x90\x81a\x1E\xE0Wa\x1DY\x84a\x1BaV[\x90a\x1Dc\x81a\x1BaV[\x94a\x1D\x97a\x07ia\x1Ds\x83a\x1BaV[a\x05\xC7a\x1D~a \xFDV[\x91a\x05\xC1a\x1D\x8B\x88a\x1BaV[\x93a\x05\xC1a\x05\xBCa \xFDV[\x93\x84\x81\x01\x80\x91\x11a\x02\xA6W`\x06Ua\x1D\xAE\x84a\x1A\x08V[\x90`\0\x963\x88R` \x94`\x07\x86R`@\x93\x84\x8A U\x88\x80Ra\x03\xE8\x84\x8A U`\x04U`\x05Us\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80`\x01T\x16\x80;\x15a\x08\xFDW\x83Q\x7F#\xB8r\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80\x82R3`\x04\x83\x01R0`$\x83\x01R`D\x82\x01\x94\x90\x94R\x90\x86\x90\x82\x90`d\x90\x82\x90\x8D\x90Z\xF1\x80\x15a\x1E\xD6Wa\x1E\xB9W[P`\x02T\x16\x92\x83;\x15a\x08\xFDW\x82Q\x91\x82R3`\x04\x83\x01R0`$\x83\x01R`D\x82\x01R\x91\x83\x90\x83\x90`d\x90\x82\x90\x8A\x90Z\xF1\x90\x81\x15a\x1E\xB0WPa\x1C\xA6\x94\x95Pa\x1E\x92W[PPa\x1A\x08V[\x81a\x1E\xA8\x92\x90=\x10a\x08\xEAWa\x08\xDA\x81\x83a\x1A\xBBV[P8\x80a\x1E\x8BV[Q=\x87\x82>=\x90\xFD[a\x1E\xCF\x90\x86=\x88\x11a\x08\xEAWa\x08\xDA\x81\x83a\x1A\xBBV[P8a\x1EGV[\x84Q=\x8B\x82>=\x90\xFD[`d`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FPool already initialized\0\0\0\0\0\0\0\0`D\x82\x01R\xFD[\x90g\x1B\xC1mgN\xC8\0\0\x91\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x82\t\x83\x82\x02\x91\x82\x80\x83\x10\x92\x03\x91\x80\x83\x03\x92\x14a \x02Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x83\x10\x15a\x1F\xCBW\x94\x7F\xAC\xCB\x18\x16[\xD6\xFE1\xAE\x1C\xF3\x18\xDC[Q\xEE\xE0\xE1\xBAV\x9B\x88\xCDt\xC1w;\x91\xFA\xC1\x06i\x94\x95\t\x90\x82\x82\x11\x90\x03`\xEE\x1B\x91\x03`\x12\x1C\x17\x02\x90V[`D\x90\x86`@Q\x91\x7FQsd\x8D\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04\x83\x01R`$\x82\x01R\xFD[PPg\r\xE0\xB6\xB3\xA7d\0\0\x90\x04\x91PV[\x91\x90\x91\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x82\t\x83\x82\x02\x91\x82\x80\x83\x10\x92\x03\x91\x80\x83\x03\x92\x14a \x02Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x83\x10\x15a\x1F\xCBW\x94\x7F\xAC\xCB\x18\x16[\xD6\xFE1\xAE\x1C\xF3\x18\xDC[Q\xEE\xE0\xE1\xBAV\x9B\x88\xCDt\xC1w;\x91\xFA\xC1\x06i\x94\x95\t\x90\x82\x82\x11\x90\x03`\xEE\x1B\x91\x03`\x12\x1C\x17\x02\x90V[\x15a \x9FWV[`d`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x14`$\x82\x01R\x7FPool not initialized\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R\xFD[`\x0BTB\x10\x15a!GWa!\"a!\x19a\x03\xE8`\tTBa\x1AbV[`\x0CT\x90a \x13V[`\x08T\x90`\nT\x82\x11`\0\x14a!;Wa\x1C\xA6\x91a\x1AbV[\x81\x01\x80\x91\x11a\x02\xA6W\x90V[`\nT\x90V[a\x1C\xA6`\x04Ta\x03\xEDa!^a \xFDV[a\x0Bh`\x05Ta\x03\xEDa\x05\xBCa \xFDV[\x90\x81\x15\x80\x15a!\x8EWP`\0\x91Pa\x1C\xA6WPg\r\xE0\xB6\xB3\xA7d\0\0\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x80\x84\x14a\"\x1EWP\x81a!\xB3WPPPg\r\xE0\xB6\xB3\xA7d\0\0\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x14a\"\x19Wg\r\xE0\xB6\xB3\xA7d\0\0\x83\x11\x15a!\xE8WPa!\xE3\x90a\r\ra\x1C\xA6\x93a*\xAAV[a\"%V[a\x1A\x8CWa!\xE3a\"\x0E\x91a\r\rn\xC0\x97\xCE{\xC9\x07\x15\xB3K\x9F\x10\0\0\0\0\x94\x85\x04a*\xAAV[\x90\x81\x15a\x1A\x8CW\x04\x90V[PP\x90V[\x92PPP\x90V[h\nh\x89\x06\xBD\x8A\xFF\xFF\xFF\x81\x11a*yWg\r\xE0\xB6\xB3\xA7d\0\0\x80`@\x92\x83\x1B\x04\x90w\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\0\0\0\0\0\0\0\x83\x16a)\\W[f\xFF\0\0\0\0\0\0\x83\x16a(TW[e\xFF\0\0\0\0\0\x83\x16a'TW[d\xFF\0\0\0\0\x83\x16a&\\W[c\xFF\0\0\0\x83\x16a%lW[b\xFF\0\0\x83\x16a$\x84W[a\xFF\0\x83\x16a#\xA4W[`\xFF\x83\x16a\"\xCDW[\x02\x91\x1C`\xBF\x03\x1C\x90V[`\x80\x83\x16a#\x92W[\x83\x83\x16a#\x80W[` \x83\x16a#nW[`\x10\x83\x16a#\\W[`\x08\x83\x16a#JW[`\x04\x83\x16a#8W[`\x02\x83\x16a#&W[`\x01\x83\x16\x15a\"\xC3Wh\x01\0\0\0\0\0\0\0\x01\x02\x83\x1Ca\"\xC3V[h\x01\0\0\0\0\0\0\0\x01\x02\x83\x1Ca#\x0BV[h\x01\0\0\0\0\0\0\0\x03\x02\x83\x1Ca#\x02V[h\x01\0\0\0\0\0\0\0\x06\x02\x83\x1Ca\"\xF9V[h\x01\0\0\0\0\0\0\0\x0B\x02\x83\x1Ca\"\xF0V[h\x01\0\0\0\0\0\0\0\x16\x02\x83\x1Ca\"\xE7V[h\x01\0\0\0\0\0\0\0,\x02\x83\x1Ca\"\xDEV[h\x01\0\0\0\0\0\0\0Y\x02\x83\x1Ca\"\xD6V[a\x80\0\x83\x16a$rW[a@\0\x83\x16a$`W[a \0\x83\x16a$NW[a\x10\0\x83\x16a$<W[a\x08\0\x83\x16a$*W[a\x04\0\x83\x16a$\x18W[a\x02\0\x83\x16a$\x06W[a\x01\0\x83\x16\x15a\"\xBAWh\x01\0\0\0\0\0\0\0\xB1\x02\x83\x1Ca\"\xBAV[h\x01\0\0\0\0\0\0\x01c\x02\x83\x1Ca#\xEAV[h\x01\0\0\0\0\0\0\x02\xC6\x02\x83\x1Ca#\xE0V[h\x01\0\0\0\0\0\0\x05\x8C\x02\x83\x1Ca#\xD6V[h\x01\0\0\0\0\0\0\x0B\x17\x02\x83\x1Ca#\xCCV[h\x01\0\0\0\0\0\0\x16.\x02\x83\x1Ca#\xC2V[h\x01\0\0\0\0\0\0,]\x02\x83\x1Ca#\xB8V[h\x01\0\0\0\0\0\0X\xB9\x02\x83\x1Ca#\xAEV[b\x80\0\0\x83\x16a%ZW[b@\0\0\x83\x16a%HW[b \0\0\x83\x16a%6W[b\x10\0\0\x83\x16a%$W[b\x08\0\0\x83\x16a%\x12W[b\x04\0\0\x83\x16a%\0W[b\x02\0\0\x83\x16a$\xEEW[b\x01\0\0\x83\x16\x15a\"\xB0Wh\x01\0\0\0\0\0\0\xB1r\x02\x83\x1Ca\"\xB0V[h\x01\0\0\0\0\0\x01b\xE4\x02\x83\x1Ca$\xD1V[h\x01\0\0\0\0\0\x02\xC5\xC8\x02\x83\x1Ca$\xC6V[h\x01\0\0\0\0\0\x05\x8B\x91\x02\x83\x1Ca$\xBBV[h\x01\0\0\0\0\0\x0B\x17!\x02\x83\x1Ca$\xB0V[h\x01\0\0\0\0\0\x16.C\x02\x83\x1Ca$\xA5V[h\x01\0\0\0\0\0,\\\x86\x02\x83\x1Ca$\x9AV[h\x01\0\0\0\0\0X\xB9\x0C\x02\x83\x1Ca$\x8FV[c\x80\0\0\0\x83\x16a&JW[c@\0\0\0\x83\x16a&8W[c \0\0\0\x83\x16a&&W[c\x10\0\0\0\x83\x16a&\x14W[c\x08\0\0\0\x83\x16a&\x02W[c\x04\0\0\0\x83\x16a%\xF0W[c\x02\0\0\0\x83\x16a%\xDEW[c\x01\0\0\0\x83\x16\x15a\"\xA5Wh\x01\0\0\0\0\0\xB1r\x18\x02\x83\x1Ca\"\xA5V[h\x01\0\0\0\0\x01b\xE40\x02\x83\x1Ca%\xC0V[h\x01\0\0\0\0\x02\xC5\xC8`\x02\x83\x1Ca%\xB4V[h\x01\0\0\0\0\x05\x8B\x90\xC0\x02\x83\x1Ca%\xA8V[h\x01\0\0\0\0\x0B\x17!\x7F\x02\x83\x1Ca%\x9CV[h\x01\0\0\0\0\x16.B\xFF\x02\x83\x1Ca%\x90V[h\x01\0\0\0\0,\\\x85\xFE\x02\x83\x1Ca%\x84V[h\x01\0\0\0\0X\xB9\x0B\xFC\x02\x83\x1Ca%xV[d\x80\0\0\0\0\x83\x16a'BW[d@\0\0\0\0\x83\x16a'0W[d \0\0\0\0\x83\x16a'\x1EW[d\x10\0\0\0\0\x83\x16a'\x0CW[d\x08\0\0\0\0\x83\x16a&\xFAW[d\x04\0\0\0\0\x83\x16a&\xE8W[d\x02\0\0\0\0\x83\x16a&\xD6W[d\x01\0\0\0\0\x83\x16\x15a\"\x99Wh\x01\0\0\0\0\xB1r\x17\xF8\x02\x83\x1Ca\"\x99V[h\x01\0\0\0\x01b\xE4/\xF1\x02\x83\x1Ca&\xB7V[h\x01\0\0\0\x02\xC5\xC8_\xE3\x02\x83\x1Ca&\xAAV[h\x01\0\0\0\x05\x8B\x90\xBF\xCE\x02\x83\x1Ca&\x9DV[h\x01\0\0\0\x0B\x17!\x7F\xBB\x02\x83\x1Ca&\x90V[h\x01\0\0\0\x16.B\xFF\xF0\x02\x83\x1Ca&\x83V[h\x01\0\0\0,\\\x86\x01\xCC\x02\x83\x1Ca&vV[h\x01\0\0\0X\xB9\x0C\x0BI\x02\x83\x1Ca&iV[e\x80\0\0\0\0\0\x83\x16a(BW[e@\0\0\0\0\0\x83\x16a(0W[e \0\0\0\0\0\x83\x16a(\x1EW[e\x10\0\0\0\0\0\x83\x16a(\x0CW[e\x08\0\0\0\0\0\x83\x16a'\xFAW[e\x04\0\0\0\0\0\x83\x16a'\xE8W[e\x02\0\0\0\0\0\x83\x16a'\xD6W[e\x01\0\0\0\0\0\x83\x16\x15a\"\x8CWh\x01\0\0\0\xB1r\x185Q\x02\x83\x1Ca\"\x8CV[h\x01\0\0\x01b\xE40\xE5\xA2\x02\x83\x1Ca'\xB6V[h\x01\0\0\x02\xC5\xC8c\xB7?\x02\x83\x1Ca'\xA8V[h\x01\0\0\x05\x8B\x90\xCF\x1En\x02\x83\x1Ca'\x9AV[h\x01\0\0\x0B\x17!\xBC\xFC\x9A\x02\x83\x1Ca'\x8CV[h\x01\0\0\x16.C\xF4\xF81\x02\x83\x1Ca'~V[h\x01\0\0,\\\x89\xD5\xECm\x02\x83\x1Ca'pV[h\x01\0\0X\xB9\x1B[\xC9\xAE\x02\x83\x1Ca'bV[f\x80\0\0\0\0\0\0\x83\x16a)JW[f@\0\0\0\0\0\0\x83\x16a)8W[f \0\0\0\0\0\0\x83\x16a)&W[f\x10\0\0\0\0\0\0\x83\x16a)\x14W[f\x08\0\0\0\0\0\0\x83\x16a)\x02W[f\x04\0\0\0\0\0\0\x83\x16a(\xF0W[f\x02\0\0\0\0\0\0\x83\x16a(\xDEW[f\x01\0\0\0\0\0\0\x83\x16\x15a\"~Wh\x01\0\0\xB1rUw\\\x04\x02\x83\x1Ca\"~V[h\x01\0\x01b\xE5%\xEE\x05G\x02\x83\x1Ca(\xBDV[h\x01\0\x02\xC5\xCC7\xDA\x94\x92\x02\x83\x1Ca(\xAEV[h\x01\0\x05\x8B\xA0\x1F\xB9\xF9m\x02\x83\x1Ca(\x9FV[h\x01\0\x0B\x17^\xFF\xDCv\xBA\x02\x83\x1Ca(\x90V[h\x01\0\x16/9\x04\x05\x1F\xA1\x02\x83\x1Ca(\x81V[h\x01\0,`^.\x8C\xECP\x02\x83\x1Ca(rV[h\x01\0X\xC8m\xA1\xC0\x9E\xA2\x02\x83\x1Ca(cV[g\x80\0\0\0\0\0\0\0\x83\x16a*ZW[g@\0\0\0\0\0\0\0\x83\x16a*HW[g \0\0\0\0\0\0\0\x83\x16a*6W[g\x10\0\0\0\0\0\0\0\x83\x16a*$W[g\x08\0\0\0\0\0\0\0\x83\x16a*\x12W[g\x04\0\0\0\0\0\0\0\x83\x16a*\0W[g\x02\0\0\0\0\0\0\0\x83\x16a)\xEEW[g\x01\0\0\0\0\0\0\0\x83\x16\x15a\"oWh\x01\0\xB1\xAF\xA5\xAB\xCB\xEDa\x02\x83\x1Ca\"oV[h\x01\x01c\xDA\x9F\xB33V\xD8\x02\x83\x1Ca)\xCCV[h\x01\x02\xC9\xA3\xE7x\x06\x0E\xE7\x02\x83\x1Ca)\xBCV[h\x01\x05\x9B\r1XWC\xAE\x02\x83\x1Ca)\xACV[h\x01\x0BU\x86\xCF\x98\x90\xF6*\x02\x83\x1Ca)\x9CV[h\x01\x17+\x83\xC7\xD5\x17\xAD\xCE\x02\x83\x1Ca)\x8CV[h\x010o\xE0\xA3\x1BqR\xDF\x02\x83\x1Ca)|V[Pw\xB5\x04\xF33\xF9\xDEd\x84\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a)lV[`$\x90`@Q\x90\x7F\xB3\xB6\xBA\x1F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\xFD[\x90g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x81\x10a+\x88W\x82\x81\x04o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11`\x07\x1B\x90\x81\x1C\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x06\x1B\x91\x82\x1Cc\xFF\xFF\xFF\xFF\x81\x11`\x05\x1B\x90\x81\x1Ca\xFF\xFF\x81\x11`\x04\x1B\x90\x81\x1C`\xFF\x81\x11`\x03\x1B\x90\x81\x1C\x91`\x0F\x83\x11`\x02\x1B\x92\x83\x1C\x93`\x01\x96\x87`\x03\x87\x11\x81\x1B\x96\x87\x1C\x11\x96\x17\x17\x17\x17\x17\x17\x17\x91\x84\x83\x02\x92\x1C\x93\x80\x85\x14a+\x81Wg\x06\xF0[Y\xD3\xB2\0\0\x94\x85[a+TWP\x91\x93PPPV[\x80\x82\x91\x02\x04\x94g\x1B\xC1mgN\xC8\0\0\x86\x10\x15a+tW[\x82\x1C\x94\x85a+HV[\x80\x95\x93\x01\x92\x82\x1C\x94a+kV[P\x90\x92PPV[`$\x90`@Q\x90\x7F6\xD3.\xF0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\xFD[\x91\x90\x82\x01\x90\x81\x83\x11a\x02\xA6Wa\r\ra\t\xD6g\r\xE0\xB6\xB3\xA7d\0\0\x94a+\xDE\x94a\x1B\xBFV[\x04\x90V[\x90a+\xF6a+\xF0\x83\x83a\x1B\xBFV[\x93a\x1BaV[\x82\x01\x80\x92\x11a\x02\xA6Wa,\x15a+\xDE\x92g\r\xE0\xB6\xB3\xA7d\0\0\x94a \x13V[a\x1AbV[a+\xF6a+\xF0\x83\x83a\x1B\xBFV";
    /// The deployed bytecode of the contract.
    pub static G3M_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct G3M<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for G3M<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for G3M<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for G3M<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for G3M<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(G3M)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> G3M<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    G3M_ABI.clone(),
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
                G3M_ABI.clone(),
                G3M_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `_initPool` (0x8489504f) function
        pub fn _init_pool(
            &self,
            amount_x: ::ethers::core::types::U256,
            amount_y: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([132, 137, 80, 79], (amount_x, amount_y))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `addLiquidity` (0x51c6590a) function
        pub fn add_liquidity(
            &self,
            liquidity: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([81, 198, 89, 10], liquidity)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `addLiquidity` (0x76701636) function
        pub fn add_liquidity_with_exact_x(
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
        ///Calls the contract's `admin` (0xf851a440) function
        pub fn admin(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([248, 81, 164, 64], ())
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
        ///Calls the contract's `initPool` (0x9e1b0045) function
        pub fn init_pool(
            &self,
            amount_x: ::ethers::core::types::U256,
            amount_y: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([158, 27, 0, 69], (amount_x, amount_y))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `instantiate` (0x3639aa32) function
        pub fn instantiate(
            &self,
            initial_x: ::ethers::core::types::U256,
            initial_price: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([54, 57, 170, 50], (initial_x, initial_price))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `liquidityWithoutPrecision` (0x75aee1da) function
        pub fn liquidity_without_precision(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([117, 174, 225, 218], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `logData` (0xbcc17dc7) function
        pub fn log_data(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([188, 193, 125, 199], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeLiquidity` (0x9c8f9f23) function
        pub fn remove_liquidity(
            &self,
            liquidity: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([156, 143, 159, 35], liquidity)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeLiquidity` (0xa0db6a82) function
        pub fn remove_liquidity_with_exact_x(
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
                .method_hash([160, 219, 106, 130], (exact_x, amount))
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
        ///Calls the contract's `reserveXWithoutPrecision` (0xe311cec8) function
        pub fn reserve_x_without_precision(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([227, 17, 206, 200], ())
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
        ///Calls the contract's `reserveYWithoutPrecision` (0x1934eb25) function
        pub fn reserve_y_without_precision(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([25, 52, 235, 37], ())
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
        ///Calls the contract's `setWeightX` (0xdb791043) function
        pub fn set_weight_x(
            &self,
            new_target_weight_x: ::ethers::core::types::U256,
            new_weight_x_update_end: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [219, 121, 16, 67],
                    (new_target_weight_x, new_weight_x_update_end),
                )
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
        ///Calls the contract's `swapAmountOut` (0x9ce332d4) function
        pub fn swap_amount_out(
            &self,
            swap_direction: bool,
            amount_out: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([156, 227, 50, 212], (swap_direction, amount_out))
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
        ///Calls the contract's `weightX` (0xadb51dee) function
        pub fn weight_x(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([173, 181, 29, 238], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `weightY` (0x8a5953c7) function
        pub fn weight_y(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([138, 89, 83, 199], ())
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
        ///Gets the contract's `LogPrices` event
        pub fn log_prices_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogPricesFilter,
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
        ///Gets the contract's `LogSyncingWeight` event
        pub fn log_syncing_weight_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogSyncingWeightFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `LogWeights` event
        pub fn log_weights_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogWeightsFilter,
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
        ///Gets the contract's `SetTargetWeightX` event
        pub fn set_target_weight_x_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SetTargetWeightXFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `SetWeightX` event
        pub fn set_weight_x_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SetWeightXFilter,
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
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, G3MEvents> {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for G3M<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `InvalidSwap` with signature `InvalidSwap(uint256,uint256)` and selector `0xb221d0e4`
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
    #[etherror(name = "InvalidSwap", abi = "InvalidSwap(uint256,uint256)")]
    pub struct InvalidSwap {
        pub invariant_before: ::ethers::core::types::U256,
        pub invariant_after: ::ethers::core::types::U256,
    }
    ///Custom Error type `PRBMath_MulDiv18_Overflow` with signature `PRBMath_MulDiv18_Overflow(uint256,uint256)` and selector `0x5173648d`
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
        name = "PRBMath_MulDiv18_Overflow",
        abi = "PRBMath_MulDiv18_Overflow(uint256,uint256)"
    )]
    pub struct PRBMath_MulDiv18_Overflow {
        pub x: ::ethers::core::types::U256,
        pub y: ::ethers::core::types::U256,
    }
    ///Custom Error type `PRBMath_MulDiv_Overflow` with signature `PRBMath_MulDiv_Overflow(uint256,uint256,uint256)` and selector `0x63a05778`
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
        name = "PRBMath_MulDiv_Overflow",
        abi = "PRBMath_MulDiv_Overflow(uint256,uint256,uint256)"
    )]
    pub struct PRBMath_MulDiv_Overflow {
        pub x: ::ethers::core::types::U256,
        pub y: ::ethers::core::types::U256,
        pub denominator: ::ethers::core::types::U256,
    }
    ///Custom Error type `PRBMath_UD60x18_Convert_Overflow` with signature `PRBMath_UD60x18_Convert_Overflow(uint256)` and selector `0x1cd951a7`
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
        name = "PRBMath_UD60x18_Convert_Overflow",
        abi = "PRBMath_UD60x18_Convert_Overflow(uint256)"
    )]
    pub struct PRBMath_UD60x18_Convert_Overflow {
        pub x: ::ethers::core::types::U256,
    }
    ///Custom Error type `PRBMath_UD60x18_Exp2_InputTooBig` with signature `PRBMath_UD60x18_Exp2_InputTooBig(uint256)` and selector `0xb3b6ba1f`
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
        name = "PRBMath_UD60x18_Exp2_InputTooBig",
        abi = "PRBMath_UD60x18_Exp2_InputTooBig(uint256)"
    )]
    pub struct PRBMath_UD60x18_Exp2_InputTooBig {
        pub x: ::ethers::core::types::U256,
    }
    ///Custom Error type `PRBMath_UD60x18_Log_InputTooSmall` with signature `PRBMath_UD60x18_Log_InputTooSmall(uint256)` and selector `0x36d32ef0`
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
        name = "PRBMath_UD60x18_Log_InputTooSmall",
        abi = "PRBMath_UD60x18_Log_InputTooSmall(uint256)"
    )]
    pub struct PRBMath_UD60x18_Log_InputTooSmall {
        pub x: ::ethers::core::types::U256,
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
    pub enum G3MErrors {
        InvalidSwap(InvalidSwap),
        PRBMath_MulDiv18_Overflow(PRBMath_MulDiv18_Overflow),
        PRBMath_MulDiv_Overflow(PRBMath_MulDiv_Overflow),
        PRBMath_UD60x18_Convert_Overflow(PRBMath_UD60x18_Convert_Overflow),
        PRBMath_UD60x18_Exp2_InputTooBig(PRBMath_UD60x18_Exp2_InputTooBig),
        PRBMath_UD60x18_Log_InputTooSmall(PRBMath_UD60x18_Log_InputTooSmall),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for G3MErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <InvalidSwap as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidSwap(decoded));
            }
            if let Ok(decoded) = <PRBMath_MulDiv18_Overflow as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PRBMath_MulDiv18_Overflow(decoded));
            }
            if let Ok(decoded) = <PRBMath_MulDiv_Overflow as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PRBMath_MulDiv_Overflow(decoded));
            }
            if let Ok(decoded) = <PRBMath_UD60x18_Convert_Overflow as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PRBMath_UD60x18_Convert_Overflow(decoded));
            }
            if let Ok(decoded) = <PRBMath_UD60x18_Exp2_InputTooBig as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PRBMath_UD60x18_Exp2_InputTooBig(decoded));
            }
            if let Ok(decoded) = <PRBMath_UD60x18_Log_InputTooSmall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PRBMath_UD60x18_Log_InputTooSmall(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for G3MErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::InvalidSwap(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PRBMath_MulDiv18_Overflow(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PRBMath_MulDiv_Overflow(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PRBMath_UD60x18_Convert_Overflow(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PRBMath_UD60x18_Exp2_InputTooBig(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PRBMath_UD60x18_Log_InputTooSmall(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for G3MErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <InvalidSwap as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <PRBMath_MulDiv18_Overflow as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <PRBMath_MulDiv_Overflow as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <PRBMath_UD60x18_Convert_Overflow as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <PRBMath_UD60x18_Exp2_InputTooBig as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <PRBMath_UD60x18_Log_InputTooSmall as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for G3MErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::InvalidSwap(element) => ::core::fmt::Display::fmt(element, f),
                Self::PRBMath_MulDiv18_Overflow(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PRBMath_MulDiv_Overflow(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PRBMath_UD60x18_Convert_Overflow(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PRBMath_UD60x18_Exp2_InputTooBig(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PRBMath_UD60x18_Log_InputTooSmall(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for G3MErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<InvalidSwap> for G3MErrors {
        fn from(value: InvalidSwap) -> Self {
            Self::InvalidSwap(value)
        }
    }
    impl ::core::convert::From<PRBMath_MulDiv18_Overflow> for G3MErrors {
        fn from(value: PRBMath_MulDiv18_Overflow) -> Self {
            Self::PRBMath_MulDiv18_Overflow(value)
        }
    }
    impl ::core::convert::From<PRBMath_MulDiv_Overflow> for G3MErrors {
        fn from(value: PRBMath_MulDiv_Overflow) -> Self {
            Self::PRBMath_MulDiv_Overflow(value)
        }
    }
    impl ::core::convert::From<PRBMath_UD60x18_Convert_Overflow> for G3MErrors {
        fn from(value: PRBMath_UD60x18_Convert_Overflow) -> Self {
            Self::PRBMath_UD60x18_Convert_Overflow(value)
        }
    }
    impl ::core::convert::From<PRBMath_UD60x18_Exp2_InputTooBig> for G3MErrors {
        fn from(value: PRBMath_UD60x18_Exp2_InputTooBig) -> Self {
            Self::PRBMath_UD60x18_Exp2_InputTooBig(value)
        }
    }
    impl ::core::convert::From<PRBMath_UD60x18_Log_InputTooSmall> for G3MErrors {
        fn from(value: PRBMath_UD60x18_Log_InputTooSmall) -> Self {
            Self::PRBMath_UD60x18_Log_InputTooSmall(value)
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
    #[ethevent(name = "LogPrices", abi = "LogPrices(uint256,uint256)")]
    pub struct LogPricesFilter {
        pub spot_price: ::ethers::core::types::U256,
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
        name = "LogSyncingWeight",
        abi = "LogSyncingWeight(uint256,uint256,uint256)"
    )]
    pub struct LogSyncingWeightFilter {
        pub weight_x: ::ethers::core::types::U256,
        pub weight_y: ::ethers::core::types::U256,
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
    #[ethevent(name = "LogWeights", abi = "LogWeights(uint256,uint256)")]
    pub struct LogWeightsFilter {
        pub block_timestamp: ::ethers::core::types::U256,
        pub weight_x: ::ethers::core::types::U256,
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
    #[ethevent(
        name = "SetTargetWeightX",
        abi = "SetTargetWeightX(uint256,uint256,uint256)"
    )]
    pub struct SetTargetWeightXFilter {
        pub new_target_weight_x: ::ethers::core::types::U256,
        pub new_weight_x_update_end: ::ethers::core::types::U256,
        pub new_weight_x_update_per_second: ::ethers::core::types::U256,
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
    #[ethevent(name = "SetWeightX", abi = "SetWeightX(uint256,uint256)")]
    pub struct SetWeightXFilter {
        pub old_weight_x: ::ethers::core::types::U256,
        pub new_weight_x: ::ethers::core::types::U256,
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
    pub enum G3MEvents {
        AddLiquidityFilter(AddLiquidityFilter),
        LogPricesFilter(LogPricesFilter),
        LogReservesFilter(LogReservesFilter),
        LogSyncingWeightFilter(LogSyncingWeightFilter),
        LogWeightsFilter(LogWeightsFilter),
        RemoveLiquidityFilter(RemoveLiquidityFilter),
        SetTargetWeightXFilter(SetTargetWeightXFilter),
        SetWeightXFilter(SetWeightXFilter),
        SwapFilter(SwapFilter),
    }
    impl ::ethers::contract::EthLogDecode for G3MEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AddLiquidityFilter::decode_log(log) {
                return Ok(G3MEvents::AddLiquidityFilter(decoded));
            }
            if let Ok(decoded) = LogPricesFilter::decode_log(log) {
                return Ok(G3MEvents::LogPricesFilter(decoded));
            }
            if let Ok(decoded) = LogReservesFilter::decode_log(log) {
                return Ok(G3MEvents::LogReservesFilter(decoded));
            }
            if let Ok(decoded) = LogSyncingWeightFilter::decode_log(log) {
                return Ok(G3MEvents::LogSyncingWeightFilter(decoded));
            }
            if let Ok(decoded) = LogWeightsFilter::decode_log(log) {
                return Ok(G3MEvents::LogWeightsFilter(decoded));
            }
            if let Ok(decoded) = RemoveLiquidityFilter::decode_log(log) {
                return Ok(G3MEvents::RemoveLiquidityFilter(decoded));
            }
            if let Ok(decoded) = SetTargetWeightXFilter::decode_log(log) {
                return Ok(G3MEvents::SetTargetWeightXFilter(decoded));
            }
            if let Ok(decoded) = SetWeightXFilter::decode_log(log) {
                return Ok(G3MEvents::SetWeightXFilter(decoded));
            }
            if let Ok(decoded) = SwapFilter::decode_log(log) {
                return Ok(G3MEvents::SwapFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for G3MEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddLiquidityFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LogPricesFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogReservesFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogSyncingWeightFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LogWeightsFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveLiquidityFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetTargetWeightXFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetWeightXFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SwapFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AddLiquidityFilter> for G3MEvents {
        fn from(value: AddLiquidityFilter) -> Self {
            Self::AddLiquidityFilter(value)
        }
    }
    impl ::core::convert::From<LogPricesFilter> for G3MEvents {
        fn from(value: LogPricesFilter) -> Self {
            Self::LogPricesFilter(value)
        }
    }
    impl ::core::convert::From<LogReservesFilter> for G3MEvents {
        fn from(value: LogReservesFilter) -> Self {
            Self::LogReservesFilter(value)
        }
    }
    impl ::core::convert::From<LogSyncingWeightFilter> for G3MEvents {
        fn from(value: LogSyncingWeightFilter) -> Self {
            Self::LogSyncingWeightFilter(value)
        }
    }
    impl ::core::convert::From<LogWeightsFilter> for G3MEvents {
        fn from(value: LogWeightsFilter) -> Self {
            Self::LogWeightsFilter(value)
        }
    }
    impl ::core::convert::From<RemoveLiquidityFilter> for G3MEvents {
        fn from(value: RemoveLiquidityFilter) -> Self {
            Self::RemoveLiquidityFilter(value)
        }
    }
    impl ::core::convert::From<SetTargetWeightXFilter> for G3MEvents {
        fn from(value: SetTargetWeightXFilter) -> Self {
            Self::SetTargetWeightXFilter(value)
        }
    }
    impl ::core::convert::From<SetWeightXFilter> for G3MEvents {
        fn from(value: SetWeightXFilter) -> Self {
            Self::SetWeightXFilter(value)
        }
    }
    impl ::core::convert::From<SwapFilter> for G3MEvents {
        fn from(value: SwapFilter) -> Self {
            Self::SwapFilter(value)
        }
    }
    ///Container type for all input parameters for the `_initPool` function with signature `_initPool(uint256,uint256)` and selector `0x8489504f`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "_initPool", abi = "_initPool(uint256,uint256)")]
    pub struct _InitPoolCall {
        pub amount_x: ::ethers::core::types::U256,
        pub amount_y: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `addLiquidity` function with signature `addLiquidity(uint256)` and selector `0x51c6590a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "addLiquidity", abi = "addLiquidity(uint256)")]
    pub struct AddLiquidityCall {
        pub liquidity: ::ethers::core::types::U256,
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
    pub struct AddLiquidityWithExactXCall {
        pub exact_x: bool,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `admin` function with signature `admin()` and selector `0xf851a440`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "admin", abi = "admin()")]
    pub struct AdminCall;
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
    ///Container type for all input parameters for the `initPool` function with signature `initPool(uint256,uint256)` and selector `0x9e1b0045`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "initPool", abi = "initPool(uint256,uint256)")]
    pub struct InitPoolCall {
        pub amount_x: ::ethers::core::types::U256,
        pub amount_y: ::ethers::core::types::U256,
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
        pub initial_x: ::ethers::core::types::U256,
        pub initial_price: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `liquidityWithoutPrecision` function with signature `liquidityWithoutPrecision()` and selector `0x75aee1da`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "liquidityWithoutPrecision", abi = "liquidityWithoutPrecision()")]
    pub struct LiquidityWithoutPrecisionCall;
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
    ///Container type for all input parameters for the `removeLiquidity` function with signature `removeLiquidity(uint256)` and selector `0x9c8f9f23`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "removeLiquidity", abi = "removeLiquidity(uint256)")]
    pub struct RemoveLiquidityCall {
        pub liquidity: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `removeLiquidity` function with signature `removeLiquidity(bool,uint256)` and selector `0xa0db6a82`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "removeLiquidity", abi = "removeLiquidity(bool,uint256)")]
    pub struct RemoveLiquidityWithExactXCall {
        pub exact_x: bool,
        pub amount: ::ethers::core::types::U256,
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
    ///Container type for all input parameters for the `reserveXWithoutPrecision` function with signature `reserveXWithoutPrecision()` and selector `0xe311cec8`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "reserveXWithoutPrecision", abi = "reserveXWithoutPrecision()")]
    pub struct ReserveXWithoutPrecisionCall;
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
    ///Container type for all input parameters for the `reserveYWithoutPrecision` function with signature `reserveYWithoutPrecision()` and selector `0x1934eb25`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "reserveYWithoutPrecision", abi = "reserveYWithoutPrecision()")]
    pub struct ReserveYWithoutPrecisionCall;
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
    ///Container type for all input parameters for the `setWeightX` function with signature `setWeightX(uint256,uint256)` and selector `0xdb791043`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "setWeightX", abi = "setWeightX(uint256,uint256)")]
    pub struct SetWeightXCall {
        pub new_target_weight_x: ::ethers::core::types::U256,
        pub new_weight_x_update_end: ::ethers::core::types::U256,
    }
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
    ///Container type for all input parameters for the `swapAmountOut` function with signature `swapAmountOut(bool,uint256)` and selector `0x9ce332d4`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "swapAmountOut", abi = "swapAmountOut(bool,uint256)")]
    pub struct SwapAmountOutCall {
        pub swap_direction: bool,
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
    ///Container type for all input parameters for the `weightX` function with signature `weightX()` and selector `0xadb51dee`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "weightX", abi = "weightX()")]
    pub struct WeightXCall;
    ///Container type for all input parameters for the `weightY` function with signature `weightY()` and selector `0x8a5953c7`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "weightY", abi = "weightY()")]
    pub struct WeightYCall;
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
    pub enum G3MCalls {
        _InitPool(_InitPoolCall),
        AddLiquidity(AddLiquidityCall),
        AddLiquidityWithExactX(AddLiquidityWithExactXCall),
        Admin(AdminCall),
        BalanceOf(BalanceOfCall),
        GetInvariant(GetInvariantCall),
        GetReserveX(GetReserveXCall),
        GetReserveY(GetReserveYCall),
        GetSpotPrice(GetSpotPriceCall),
        GetStrategyData(GetStrategyDataCall),
        GetSwapFee(GetSwapFeeCall),
        InitExactX(InitExactXCall),
        InitPool(InitPoolCall),
        Instantiate(InstantiateCall),
        LiquidityWithoutPrecision(LiquidityWithoutPrecisionCall),
        LogData(LogDataCall),
        RemoveLiquidity(RemoveLiquidityCall),
        RemoveLiquidityWithExactX(RemoveLiquidityWithExactXCall),
        ReserveX(ReserveXCall),
        ReserveXWithoutPrecision(ReserveXWithoutPrecisionCall),
        ReserveY(ReserveYCall),
        ReserveYWithoutPrecision(ReserveYWithoutPrecisionCall),
        SetSwapFee(SetSwapFeeCall),
        SetWeightX(SetWeightXCall),
        SwapAmountIn(SwapAmountInCall),
        SwapAmountOut(SwapAmountOutCall),
        SwapFee(SwapFeeCall),
        TokenX(TokenXCall),
        TokenY(TokenYCall),
        TotalLiquidity(TotalLiquidityCall),
        WeightX(WeightXCall),
        WeightY(WeightYCall),
    }
    impl ::ethers::core::abi::AbiDecode for G3MCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <_InitPoolCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::_InitPool(decoded));
            }
            if let Ok(decoded) = <AddLiquidityCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AddLiquidity(decoded));
            }
            if let Ok(decoded) = <AddLiquidityWithExactXCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AddLiquidityWithExactX(decoded));
            }
            if let Ok(decoded) = <AdminCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Admin(decoded));
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
            if let Ok(decoded) = <LiquidityWithoutPrecisionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LiquidityWithoutPrecision(decoded));
            }
            if let Ok(decoded) = <LogDataCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LogData(decoded));
            }
            if let Ok(decoded) = <RemoveLiquidityCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RemoveLiquidity(decoded));
            }
            if let Ok(decoded) = <RemoveLiquidityWithExactXCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RemoveLiquidityWithExactX(decoded));
            }
            if let Ok(decoded) = <ReserveXCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ReserveX(decoded));
            }
            if let Ok(decoded) = <ReserveXWithoutPrecisionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ReserveXWithoutPrecision(decoded));
            }
            if let Ok(decoded) = <ReserveYCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ReserveY(decoded));
            }
            if let Ok(decoded) = <ReserveYWithoutPrecisionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ReserveYWithoutPrecision(decoded));
            }
            if let Ok(decoded) = <SetSwapFeeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetSwapFee(decoded));
            }
            if let Ok(decoded) = <SetWeightXCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetWeightX(decoded));
            }
            if let Ok(decoded) = <SwapAmountInCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SwapAmountIn(decoded));
            }
            if let Ok(decoded) = <SwapAmountOutCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SwapAmountOut(decoded));
            }
            if let Ok(decoded) = <SwapFeeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SwapFee(decoded));
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
            if let Ok(decoded) = <WeightXCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::WeightX(decoded));
            }
            if let Ok(decoded) = <WeightYCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::WeightY(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for G3MCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::_InitPool(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddLiquidity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddLiquidityWithExactX(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Admin(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BalanceOf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetInvariant(element) => {
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
                Self::InitPool(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Instantiate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LiquidityWithoutPrecision(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LogData(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RemoveLiquidity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemoveLiquidityWithExactX(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ReserveX(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ReserveXWithoutPrecision(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ReserveY(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ReserveYWithoutPrecision(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetSwapFee(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetWeightX(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SwapAmountIn(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SwapAmountOut(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SwapFee(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TokenX(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TokenY(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TotalLiquidity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WeightX(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::WeightY(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for G3MCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::_InitPool(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddLiquidity(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddLiquidityWithExactX(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Admin(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetInvariant(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetReserveX(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetReserveY(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSpotPrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetStrategyData(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSwapFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::InitExactX(element) => ::core::fmt::Display::fmt(element, f),
                Self::InitPool(element) => ::core::fmt::Display::fmt(element, f),
                Self::Instantiate(element) => ::core::fmt::Display::fmt(element, f),
                Self::LiquidityWithoutPrecision(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LogData(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveLiquidity(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveLiquidityWithExactX(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ReserveX(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReserveXWithoutPrecision(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ReserveY(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReserveYWithoutPrecision(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetSwapFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetWeightX(element) => ::core::fmt::Display::fmt(element, f),
                Self::SwapAmountIn(element) => ::core::fmt::Display::fmt(element, f),
                Self::SwapAmountOut(element) => ::core::fmt::Display::fmt(element, f),
                Self::SwapFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::TokenX(element) => ::core::fmt::Display::fmt(element, f),
                Self::TokenY(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalLiquidity(element) => ::core::fmt::Display::fmt(element, f),
                Self::WeightX(element) => ::core::fmt::Display::fmt(element, f),
                Self::WeightY(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<_InitPoolCall> for G3MCalls {
        fn from(value: _InitPoolCall) -> Self {
            Self::_InitPool(value)
        }
    }
    impl ::core::convert::From<AddLiquidityCall> for G3MCalls {
        fn from(value: AddLiquidityCall) -> Self {
            Self::AddLiquidity(value)
        }
    }
    impl ::core::convert::From<AddLiquidityWithExactXCall> for G3MCalls {
        fn from(value: AddLiquidityWithExactXCall) -> Self {
            Self::AddLiquidityWithExactX(value)
        }
    }
    impl ::core::convert::From<AdminCall> for G3MCalls {
        fn from(value: AdminCall) -> Self {
            Self::Admin(value)
        }
    }
    impl ::core::convert::From<BalanceOfCall> for G3MCalls {
        fn from(value: BalanceOfCall) -> Self {
            Self::BalanceOf(value)
        }
    }
    impl ::core::convert::From<GetInvariantCall> for G3MCalls {
        fn from(value: GetInvariantCall) -> Self {
            Self::GetInvariant(value)
        }
    }
    impl ::core::convert::From<GetReserveXCall> for G3MCalls {
        fn from(value: GetReserveXCall) -> Self {
            Self::GetReserveX(value)
        }
    }
    impl ::core::convert::From<GetReserveYCall> for G3MCalls {
        fn from(value: GetReserveYCall) -> Self {
            Self::GetReserveY(value)
        }
    }
    impl ::core::convert::From<GetSpotPriceCall> for G3MCalls {
        fn from(value: GetSpotPriceCall) -> Self {
            Self::GetSpotPrice(value)
        }
    }
    impl ::core::convert::From<GetStrategyDataCall> for G3MCalls {
        fn from(value: GetStrategyDataCall) -> Self {
            Self::GetStrategyData(value)
        }
    }
    impl ::core::convert::From<GetSwapFeeCall> for G3MCalls {
        fn from(value: GetSwapFeeCall) -> Self {
            Self::GetSwapFee(value)
        }
    }
    impl ::core::convert::From<InitExactXCall> for G3MCalls {
        fn from(value: InitExactXCall) -> Self {
            Self::InitExactX(value)
        }
    }
    impl ::core::convert::From<InitPoolCall> for G3MCalls {
        fn from(value: InitPoolCall) -> Self {
            Self::InitPool(value)
        }
    }
    impl ::core::convert::From<InstantiateCall> for G3MCalls {
        fn from(value: InstantiateCall) -> Self {
            Self::Instantiate(value)
        }
    }
    impl ::core::convert::From<LiquidityWithoutPrecisionCall> for G3MCalls {
        fn from(value: LiquidityWithoutPrecisionCall) -> Self {
            Self::LiquidityWithoutPrecision(value)
        }
    }
    impl ::core::convert::From<LogDataCall> for G3MCalls {
        fn from(value: LogDataCall) -> Self {
            Self::LogData(value)
        }
    }
    impl ::core::convert::From<RemoveLiquidityCall> for G3MCalls {
        fn from(value: RemoveLiquidityCall) -> Self {
            Self::RemoveLiquidity(value)
        }
    }
    impl ::core::convert::From<RemoveLiquidityWithExactXCall> for G3MCalls {
        fn from(value: RemoveLiquidityWithExactXCall) -> Self {
            Self::RemoveLiquidityWithExactX(value)
        }
    }
    impl ::core::convert::From<ReserveXCall> for G3MCalls {
        fn from(value: ReserveXCall) -> Self {
            Self::ReserveX(value)
        }
    }
    impl ::core::convert::From<ReserveXWithoutPrecisionCall> for G3MCalls {
        fn from(value: ReserveXWithoutPrecisionCall) -> Self {
            Self::ReserveXWithoutPrecision(value)
        }
    }
    impl ::core::convert::From<ReserveYCall> for G3MCalls {
        fn from(value: ReserveYCall) -> Self {
            Self::ReserveY(value)
        }
    }
    impl ::core::convert::From<ReserveYWithoutPrecisionCall> for G3MCalls {
        fn from(value: ReserveYWithoutPrecisionCall) -> Self {
            Self::ReserveYWithoutPrecision(value)
        }
    }
    impl ::core::convert::From<SetSwapFeeCall> for G3MCalls {
        fn from(value: SetSwapFeeCall) -> Self {
            Self::SetSwapFee(value)
        }
    }
    impl ::core::convert::From<SetWeightXCall> for G3MCalls {
        fn from(value: SetWeightXCall) -> Self {
            Self::SetWeightX(value)
        }
    }
    impl ::core::convert::From<SwapAmountInCall> for G3MCalls {
        fn from(value: SwapAmountInCall) -> Self {
            Self::SwapAmountIn(value)
        }
    }
    impl ::core::convert::From<SwapAmountOutCall> for G3MCalls {
        fn from(value: SwapAmountOutCall) -> Self {
            Self::SwapAmountOut(value)
        }
    }
    impl ::core::convert::From<SwapFeeCall> for G3MCalls {
        fn from(value: SwapFeeCall) -> Self {
            Self::SwapFee(value)
        }
    }
    impl ::core::convert::From<TokenXCall> for G3MCalls {
        fn from(value: TokenXCall) -> Self {
            Self::TokenX(value)
        }
    }
    impl ::core::convert::From<TokenYCall> for G3MCalls {
        fn from(value: TokenYCall) -> Self {
            Self::TokenY(value)
        }
    }
    impl ::core::convert::From<TotalLiquidityCall> for G3MCalls {
        fn from(value: TotalLiquidityCall) -> Self {
            Self::TotalLiquidity(value)
        }
    }
    impl ::core::convert::From<WeightXCall> for G3MCalls {
        fn from(value: WeightXCall) -> Self {
            Self::WeightX(value)
        }
    }
    impl ::core::convert::From<WeightYCall> for G3MCalls {
        fn from(value: WeightYCall) -> Self {
            Self::WeightY(value)
        }
    }
    ///Container type for all return fields from the `_initPool` function with signature `_initPool(uint256,uint256)` and selector `0x8489504f`
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
    pub struct _InitPoolReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `addLiquidity` function with signature `addLiquidity(uint256)` and selector `0x51c6590a`
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
    pub struct AddLiquidityWithExactXReturn {
        pub amount_x: ::ethers::core::types::U256,
        pub amount_y: ::ethers::core::types::U256,
        pub liquidity: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `admin` function with signature `admin()` and selector `0xf851a440`
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
    pub struct AdminReturn(pub ::ethers::core::types::Address);
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
    pub struct GetStrategyDataReturn(pub ::ethers::core::types::Bytes);
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
    ///Container type for all return fields from the `initPool` function with signature `initPool(uint256,uint256)` and selector `0x9e1b0045`
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
    pub struct InitPoolReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `liquidityWithoutPrecision` function with signature `liquidityWithoutPrecision()` and selector `0x75aee1da`
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
    pub struct LiquidityWithoutPrecisionReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `removeLiquidity` function with signature `removeLiquidity(uint256)` and selector `0x9c8f9f23`
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
    pub struct RemoveLiquidityReturn {
        pub amount_x: ::ethers::core::types::U256,
        pub amount_y: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `removeLiquidity` function with signature `removeLiquidity(bool,uint256)` and selector `0xa0db6a82`
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
    pub struct RemoveLiquidityWithExactXReturn {
        pub amount_x: ::ethers::core::types::U256,
        pub amount_y: ::ethers::core::types::U256,
        pub liquidity: ::ethers::core::types::U256,
    }
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
    ///Container type for all return fields from the `reserveXWithoutPrecision` function with signature `reserveXWithoutPrecision()` and selector `0xe311cec8`
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
    pub struct ReserveXWithoutPrecisionReturn(pub ::ethers::core::types::U256);
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
    ///Container type for all return fields from the `reserveYWithoutPrecision` function with signature `reserveYWithoutPrecision()` and selector `0x1934eb25`
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
    pub struct ReserveYWithoutPrecisionReturn(pub ::ethers::core::types::U256);
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
    ///Container type for all return fields from the `swapAmountOut` function with signature `swapAmountOut(bool,uint256)` and selector `0x9ce332d4`
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
    pub struct SwapAmountOutReturn(pub ::ethers::core::types::U256);
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
    ///Container type for all return fields from the `weightX` function with signature `weightX()` and selector `0xadb51dee`
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
    pub struct WeightXReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `weightY` function with signature `weightY()` and selector `0x8a5953c7`
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
    pub struct WeightYReturn(pub ::ethers::core::types::U256);
}
