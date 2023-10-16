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
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15b\0\0^W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`@Qb\0$\xBF8\x03\x80b\0$\xBF\x839\x81\x01`@\x81\x90Rb\0\0\x81\x91b\0\x02CV[\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x03b\0\0\xD9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01RmInvalid tokens`\x90\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x80\x87\x16`\x01`\x01`\xA0\x1B\x03\x19\x92\x83\x16\x17\x90\x92U`\x02\x80T\x92\x86\x16\x92\x82\x16\x92\x90\x92\x17\x90\x91U`\0\x80T\x90\x91\x163\x17\x90Ub\0\x01&\x82f#\x86\xF2o\xC1\0\0\x11\x15\x90V[b\0\x01gW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01RoWeight X too low`\x80\x1B`D\x82\x01R`d\x01b\0\0\xD0V[b\0\x01z\x82g\r\xBD/\xC17\xA3\0\0\x10\x15\x90V[b\0\x01\xBCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01Rp\n\xEC\xAD,\xED\x0E\x84\x0B\x04\x0E\x8D\xED\xE4\r\r,\xED`{\x1B`D\x82\x01R`d\x01b\0\0\xD0V[`\n\x82\x90UB`\x0B\x81\x90U`\x08\x83\x90U`\tU`\x03Ta'\x10\x10\x15b\0\x02\x19W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01Rp\nn\xEC.\x04\x0C\xCC\xAC\xA4\x0E\x8D\xED\xE4\r\r,\xED`{\x1B`D\x82\x01R`d\x01b\0\0\xD0V[`\x03UPb\0\x02\xD6\x91PPV[\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x02>W`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15b\0\x02\xA5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[b\0\x02\xB0\x85b\0\x02&V[\x93Pb\0\x02\xC0` \x86\x01b\0\x02&V[`@\x86\x01Q``\x90\x96\x01Q\x94\x97\x90\x96P\x92PPPV[a!\xD9\x80b\0\x02\xE6`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x01\x84W`\x005`\xE0\x1C\x80c\x9C\x8F\x9F#\x11a\x01\x05W\x80c\xC0\xFF\x1A\x15\x11a\0\xC9W\x80c\xC0\xFF\x1A\x15\x14a\x03\x16W\x80c\xDBy\x10C\x14a\x03\x1EW\x80c\xDCv\xFA\xBC\x14a\x031W\x80c\xE3\x11\xCE\xC8\x14a\x039W\x80c\xF8Q\xA4@\x14a\x03AW\x80c\xFA\xDF\xA6[\x14a\x03TWa\x01\x84V[\x80c\x9C\x8F\x9F#\x14a\x02\xC2W\x80c\x9C\xE32\xD4\x14a\x02\xD5W\x80c\x9E\x1B\0E\x14a\x02\xE8W\x80c\xAD\xB5\x1D\xEE\x14a\x02\xFBW\x80c\xB7\xD1\x9F\xC4\x14a\x03\x03Wa\x01\x84V[\x80c4\xE1\x99\x07\x11a\x01LW\x80c4\xE1\x99\x07\x14a\x02TW\x80cQ\xC6Y\n\x14a\x02iW\x80cT\xCF*\xEB\x14a\x02\x91W\x80cp\xA0\x821\x14a\x02\x9AW\x80c\x8AYS\xC7\x14a\x02\xBAWa\x01\x84V[\x80c\x08\xEA\xBD\xDA\x14a\x01\xE9W\x80c\x15w\x0F\x92\x14a\x02\x05W\x80c\x16\xDC\x16[\x14a\x02\x0EW\x80c\x194\xEB%\x14a\x029W\x80c\x1F\xDA\xBC'\x14a\x02AW[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x01\xF2`\x04T\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\xF2`\x06T\x81V[`\x01Ta\x02!\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\xFCV[a\x01\xF2a\x03]V[a\x01\xF2a\x02O6`\x04a \x18V[a\x03oV[a\x02ga\x02b6`\x04a GV[a\x03\x86V[\0[a\x02|a\x02w6`\x04a GV[a\x04\x1CV[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x01\xFCV[a\x01\xF2`\x03T\x81V[a\x01\xF2a\x02\xA86`\x04a cV[`\x07` R`\0\x90\x81R`@\x90 T\x81V[a\x01\xF2a\x06\xC6V[a\x02|a\x02\xD06`\x04a GV[a\x06\xE1V[a\x01\xF2a\x02\xE36`\x04a \x18V[a\t\x8AV[a\x01\xF2a\x02\xF66`\x04a \x8FV[a\t\x98V[a\x01\xF2a\x0CMV[`\x02Ta\x02!\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01\xF2a\x0C\xBAV[a\x02ga\x03,6`\x04a \x8FV[a\x0C\xD5V[a\x01\xF2a\x0EbV[a\x01\xF2a\x0E\x82V[`\0Ta\x02!\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01\xF2`\x05T\x81V[`\0a\x03j`\x05Ta\x0E\x8BV[\x90P\x90V[`\0a\x03}\x83`\x01\x84a\x0E\x9FV[\x90P[\x92\x91PPV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x03\xD1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh'7\xBA\x100\xB26\xB4\xB7`\xB9\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[a'\x10\x81\x11\x15a\x04\x17W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01Rp\nn\xEC.\x04\x0C\xCC\xAC\xA4\x0E\x8D\xED\xE4\r\r,\xED`{\x1B`D\x82\x01R`d\x01a\x03\xC8V[`\x03UV[`\0\x80a\x04)`\x06T\x15\x90V[\x15a\x04mW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rs\x14\x1B\xDB\xDB\x08\x1B\x9B\xDD\x08\x1A[\x9A]\x1AX[\x1A^\x99Y`b\x1B`D\x82\x01R`d\x01a\x03\xC8V[a\x04|`\x06T\x84`\x04Ta\x12\x84V[\x91Pa\x04\x8D`\x06T\x84`\x05Ta\x12\x84V[`\x01T`@Qc#\xB8r\xDD`\xE0\x1B\x81R\x91\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c#\xB8r\xDD\x90a\x04\xC2\x903\x900\x90\x87\x90`\x04\x01a \xB4V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x05\x1AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a!\x84\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x05.W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05R\x91\x90a \xD8V[P`\x02T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\x05\x87\x903\x900\x90\x86\x90`\x04\x01a \xB4V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x05\xDFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a!\x84\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x05\xF3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\x17\x91\x90a \xD8V[P`@\x80Q\x84\x81R` \x81\x01\x84\x90R\x90\x81\x01\x82\x90R3\x90\x7F\xBE\xB3\x88W\x86\xD67\xA4t\xCB\xC2\x87\xC0\xA4E\x87#\x163\xA0w\xF0\xBD05MZK\x18\x99o\xCE\x90``\x01`@Q\x80\x91\x03\x90\xA2a\x06o`\x04Ta\x06j\x84a\x12\xC3V[a\x13\nV[`\x04U`\x05Ta\x06\x82\x90a\x06j\x83a\x12\xC3V[`\x05U3`\0\x90\x81R`\x07` R`@\x90 Ta\x06\x9F\x90\x84a\x13\nV[3`\0\x90\x81R`\x07` R`@\x90 U`\x06Ta\x06\xBC\x90\x84a\x13\nV[`\x06U\x90\x92\x90\x91PV[`\0a\x03jg\r\xE0\xB6\xB3\xA7d\0\0a\x06\xDCa\x0CMV[a\x13\x1CV[3`\0\x90\x81R`\x07` R`@\x81 T\x81\x90\x83\x11\x15a\x07;W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01RuInsufficient liquidity`P\x1B`D\x82\x01R`d\x01a\x03\xC8V[a\x07J`\x06T\x84`\x04Ta\x13+V[\x91Pa\x07[`\x06T\x84`\x05Ta\x13+V[`\x01T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x85\x90R\x91\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x07\xE6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a!\x84\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x07\xFAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\x1E\x91\x90a \xD8V[P`\x02T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x83\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x08\xA9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a!\x84\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x08\xBDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xE1\x91\x90a \xD8V[P3`\0\x90\x81R`\x07` R`@\x90 Ta\x08\xFC\x90\x84a\x13\x1CV[3`\0\x90\x81R`\x07` R`@\x90 U`\x06Ta\t\x19\x90\x84a\x13\x1CV[`\x06U`\x04Ta\t,\x90a\x06\xDC\x84a\x12\xC3V[`\x04U`\x05Ta\t?\x90a\x06\xDC\x83a\x12\xC3V[`\x05U`@\x80Q\x84\x81R` \x81\x01\x84\x90R\x90\x81\x01\x82\x90R3\x90\x7FY\xC3\xA0\xB6\x0Cj\xB7\xDE\xB6.\x14@\xC9\xE7$A\xDBm\xB7\xDF\xE5\x14\xDB\xA8\xCB\x18\xE6\x0C\r\x89n\xFA\x90``\x01`@Q\x80\x91\x03\x90\xA2\x91P\x91V[`\0a\x03}\x83`\0\x84a\x0E\x9FV[`\0a\t\xA4`\x06T\x15\x90V[a\t\xF0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FPool already initialized\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x03\xC8V[`\0a\t\xFB\x84a\x12\xC3V[\x90P`\0a\n\x08\x84a\x12\xC3V[\x90P`\0a\n6a\n\x18\x87a\x12\xC3V[a\n a\x0CMV[a\n)\x88a\x12\xC3V[a\n1a\x06\xC6V[a\x13RV[\x90P`\0a\nM\x82a\nH`\x02a\x12\xC3V[a\x13uV[\x90Pa\n[`\x06T\x82a\x13\nV[`\x06Ua\nj\x81a\x03\xE8a\x13\x1CV[3`\0\x81\x81R`\x07` R`@\x80\x82 \x93\x90\x93U\x80Ra\x03\xE8\x7FmRW N\xBE}\x88\xFD\x91\xAE\x87\x94\x1C\xB2\xDD\x9D\x80b\xB6J\xE5\xA2\xBD-(\xEC@\xB9\xFB\xF6\xDFU`\x04\x86\x81U`\x05\x86\x90U`\x01T\x92Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92c#\xB8r\xDD\x92a\n\xE0\x92\x90\x910\x91\x8D\x91\x01a \xB4V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0B8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a!\x84\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x0BLW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Bp\x91\x90a \xD8V[P`\x02T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\x0B\xA5\x903\x900\x90\x8B\x90`\x04\x01a \xB4V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0B\xFDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a!\x84\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x0C\x11W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C5\x91\x90a \xD8V[Pa\x0CB\x81a\x03\xE8a\x13\x1CV[\x97\x96PPPPPPPV[`\0`\x0BTB\x10a\x0C_WP`\nT\x90V[`\0`\tTBa\x0Co\x91\x90a!\x0EV[\x90P`\0a\x0C\x87a\x0C\x7F\x83a\x12\xC3V[`\x0CTa\x13uV[\x90Pa\x0C\x96`\x08T`\nT\x10\x90V[\x15a\x0C\xAEWa\x0C\xA7`\x08T\x82a\x13\x1CV[\x92PPP\x90V[a\x0C\xA7`\x08T\x82a\x13\nV[`\0a\x03j`\x04Ta\x0C\xCAa\x0CMV[`\x05Ta\n1a\x06\xC6V[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\r\x1BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh'7\xBA\x100\xB26\xB4\xB7`\xB9\x1B`D\x82\x01R`d\x01a\x03\xC8V[a\r,\x82f#\x86\xF2o\xC1\0\0\x11\x15\x90V[a\rkW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01RoWeight X too low`\x80\x1B`D\x82\x01R`d\x01a\x03\xC8V[a\r}\x82g\r\xBD/\xC17\xA3\0\0\x10\x15\x90V[a\r\xBDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01Rp\n\xEC\xAD,\xED\x0E\x84\x0B\x04\x0E\x8D\xED\xE4\r\r,\xED`{\x1B`D\x82\x01R`d\x01a\x03\xC8V[B\x81\x11a\x0E\0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01Rp\x15\\\x19\x18]\x19H\x19[\x99\x08\x1C\x18\\\xDD\x19Y`z\x1B`D\x82\x01R`d\x01a\x03\xC8V[a\x0E\x08a\x13\x84V[`\0a\x0E\x15`\x08T\x84\x10\x90V[a\x0E*Wa\x0E%\x83`\x08Ta\x13\x1CV[a\x0E6V[a\x0E6`\x08T\x84a\x13\x1CV[\x90Pa\x0ES\x81a\x0ENa\x0EIB\x86a!\x0EV[a\x12\xC3V[a\x13\x95V[`\x0CUP`\n\x91\x90\x91U`\x0BUV[`\0a\x03j`\x04Ta\x0Era\x0CMV[`\x05Ta\x0E}a\x06\xC6V[a\x13\xADV[`\0a\x03j`\x04T[`\0a\x03\x80g\r\xE0\xB6\xB3\xA7d\0\0\x83a!7V[`\0\x80a\x0E\xAAa\x0CMV[\x90P`\0a\x0E\xB6a\x06\xC6V[\x90P`\0a\x0E\xCA`\x04T\x84`\x05T\x85a\x13RV[\x90P`\0\x80\x87\x15a\x0FRW\x86\x91P`\0a'\x10`\x03T\x84a\x0E\xEB\x91\x90a!YV[a\x0E\xF5\x91\x90a!7V[\x90P`\0a\x0F\x03\x82\x85a!\x0EV[\x90Pa\x0FI\x81\x8Ca\x0F\x16W`\x05Ta\x0F\x1AV[`\x04T[\x8Da\x0F%W\x88a\x0F'V[\x89[\x8Ea\x0F4W`\x04Ta\x0F8V[`\x05T[\x8Fa\x0FCW\x8Ba\x13\xD4V[\x8Aa\x13\xD4V[\x92PPPa\x0F\xC6V[P\x85`\0a\x0F\x9A\x82\x8Ba\x0FgW`\x05Ta\x0FkV[`\x04T[\x8Ca\x0FvW\x87a\x0FxV[\x88[\x8Da\x0F\x85W`\x04Ta\x0F\x89V[`\x05T[\x8Ea\x0F\x94W\x8Aa\x14\x11V[\x89a\x14\x11V[\x90P`\x03Ta'\x10a\x0F\xAC\x91\x90a!\x0EV[a\x0F\xB8\x82a'\x10a!YV[a\x0F\xC2\x91\x90a!7V[\x92PP[\x88\x15a\x0F\xF6Wa\x0F\xDB`\x04Ta\x06j\x84a\x12\xC3V[`\x04U`\x05Ta\x0F\xEE\x90a\x06\xDC\x83a\x12\xC3V[`\x05Ua\x10\x1CV[a\x10\x05`\x04Ta\x06\xDC\x83a\x12\xC3V[`\x04U`\x05Ta\x10\x18\x90a\x06j\x84a\x12\xC3V[`\x05U[`\0a\x10.`\x04T\x87`\x05T\x88a\x13RV[\x90P\x83\x81\x10\x15a\x10[W`@Qc,\x88t9`\xE2\x1B\x81R`\x04\x81\x01\x85\x90R`$\x81\x01\x82\x90R`D\x01a\x03\xC8V[\x89a\x10qW`\x02T`\x01`\x01`\xA0\x1B\x03\x16a\x10~V[`\x01T`\x01`\x01`\xA0\x1B\x03\x16[`\x01`\x01`\xA0\x1B\x03\x16c#\xB8r\xDD30\x86`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x10\xAD\x93\x92\x91\x90a \xB4V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x11\x05W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a!\x84\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x11\x19W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11=\x91\x90a \xD8V[P\x89a\x11TW`\x01T`\x01`\x01`\xA0\x1B\x03\x16a\x11aV[`\x02T`\x01`\x01`\xA0\x1B\x03\x16[`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x11\xE9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a!\x84\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x11\xFDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12!\x91\x90a \xD8V[P`@\x80Q\x8B\x15\x15\x81R` \x81\x01\x85\x90R\x90\x81\x01\x83\x90R3\x90\x7F\xBF\xD5\n\x04\xF1\xE6\xE4\xAE\xE3D\xF5\xD0\xE7\xF1]t\xD0\xDB\xB5\x8C\xD1\xF7\x11\xDA\xA6F0\x94\xCA\x95\x08\xCD\x90``\x01`@Q\x80\x91\x03\x90\xA2\x88a\x12rW\x82a\x12tV[\x81[\x96PPPPPPP[\x93\x92PPPV[`\0a\x12\xBBa\x12\xB6a\x12\xB0a\x12\xA2a\x12\x9C\x88\x88a\x13\nV[\x88a\x13\x95V[g\r\xE0\xB6\xB3\xA7d\0\0a\x13\x1CV[\x84a\x13uV[a\x0E\x8BV[\x94\x93PPPPV[`\0a\x12\xD9g\r\xE0\xB6\xB3\xA7d\0\0`\0\x19a!7V[\x82\x11\x15a\x12\xFCW`@Qc\x1C\xD9Q\xA7`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R`$\x01a\x03\xC8V[Pg\r\xE0\xB6\xB3\xA7d\0\0\x02\x90V[`\0a\x03}a\x13\x19\x83\x85a!pV[\x90V[`\0a\x03}a\x13\x19\x83\x85a!\x0EV[`\0a\x12\xBBa\x12\xB6a\x12\xB0g\r\xE0\xB6\xB3\xA7d\0\0a\x06\xDCa\x13L\x89\x89a\x13\x1CV[\x89a\x13\x95V[`\0\x80a\x13_\x86\x86a\x14AV[\x90P`\0a\x13m\x85\x85a\x14AV[\x90Pa\x0CB\x82\x82[`\0a\x03}a\x13\x19\x84\x84a\x15NV[a\x13\x8Ca\x0CMV[`\x08UB`\tUV[`\0a\x03}a\x13\x19\x84g\r\xE0\xB6\xB3\xA7d\0\0\x85a\x16\x04V[`\0\x80a\x13\xBA\x84\x84a\x13\x95V[\x90P`\0a\x13\xC8\x87\x87a\x13\x95V[\x90Pa\x0CB\x82\x82a\x13\x95V[`\0\x80a\x13\xE8\x86a\x0EN\x88a\x06j\x8Ba\x12\xC3V[\x90Pa\x0CBa\x12\xB6\x85a\nHg\r\xE0\xB6\xB3\xA7d\0\0a\x06\xDCa\x14\n\x8B\x8Aa\x13\x95V[\x87\x90a\x14AV[`\0\x80a\x14%\x84a\x0EN\x86a\x06\xDC\x8Ba\x12\xC3V[\x90Pa\x0CBa\x12\xB6\x87a\nHa\x12\xA2a\x14>\x88\x8Ba\x13\x95V[\x86\x90[`\0\x82\x82\x81\x83\x03a\x14lW\x80\x15a\x14YW`\0a\x14cV[g\r\xE0\xB6\xB3\xA7d\0\0[\x92PPPa\x03\x80V[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x03a\x14\x8DWg\r\xE0\xB6\xB3\xA7d\0\0\x92PPPa\x03\x80V[\x80`\0\x03a\x14\xA7Wg\r\xE0\xB6\xB3\xA7d\0\0\x92PPPa\x03\x80V[g\r\xE0\xB6\xB3\xA7d\0\0\x81\x03a\x14\xC0W\x84\x92PPPa\x03\x80V[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x11\x15a\x14\xF1Wa\x14\xEAa\x14\xE5a\x14\xDF\x87a\x16\xD8V[\x86a\x13uV[a\x18\x06V[\x92Pa\x15FV[`\0a\x15\x0Fa\x13\x19\x84n\xC0\x97\xCE{\xC9\x07\x15\xB3K\x9F\x10\0\0\0\0a!7V[\x90P`\0a\x15(a\x14\xE5a\x15\"\x84a\x16\xD8V[\x88a\x13uV[\x90Pa\x0CBa\x13\x19\x82n\xC0\x97\xCE{\xC9\x07\x15\xB3K\x9F\x10\0\0\0\0a!7V[PP\x92\x91PPV[`\0\x80\x80`\0\x19\x84\x86\t\x84\x86\x02\x92P\x82\x81\x10\x83\x82\x03\x03\x91PP\x80`\0\x03a\x15\x82WPg\r\xE0\xB6\xB3\xA7d\0\0\x90\x04\x90Pa\x03\x80V[g\r\xE0\xB6\xB3\xA7d\0\0\x81\x10a\x15\xB4W`@QcQsd\x8D`\xE0\x1B\x81R`\x04\x81\x01\x86\x90R`$\x81\x01\x85\x90R`D\x01a\x03\xC8V[`\0g\r\xE0\xB6\xB3\xA7d\0\0\x85\x87\tb\x04\0\0\x81\x85\x03\x04\x93\x10\x90\x91\x03`\x01`\xEE\x1B\x02\x91\x90\x91\x17\x7F\xAC\xCB\x18\x16[\xD6\xFE1\xAE\x1C\xF3\x18\xDC[Q\xEE\xE0\xE1\xBAV\x9B\x88\xCDt\xC1w;\x91\xFA\xC1\x06i\x02\x91PP\x92\x91PPV[`\0\x80\x80`\0\x19\x85\x87\t\x85\x87\x02\x92P\x82\x81\x10\x83\x82\x03\x03\x91PP\x80`\0\x03a\x16>W\x83\x82\x81a\x164Wa\x164a!!V[\x04\x92PPPa\x12}V[\x83\x81\x10a\x16oW`@Qc\x0Ct\n\xEF`\xE3\x1B\x81R`\x04\x81\x01\x87\x90R`$\x81\x01\x86\x90R`D\x81\x01\x85\x90R`d\x01a\x03\xC8V[`\0\x84\x86\x88\t`\x02`\x01\x87\x19\x81\x01\x88\x16\x97\x88\x90\x04`\x03\x81\x02\x83\x18\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x90\x81\x02\x90\x92\x03\x90\x91\x02`\0\x88\x90\x03\x88\x90\x04\x90\x91\x01\x85\x83\x11\x90\x94\x03\x93\x90\x93\x02\x93\x03\x94\x90\x94\x04\x91\x90\x91\x17\x02\x94\x93PPPPV[`\0\x81g\r\xE0\xB6\xB3\xA7d\0\0\x81\x10\x15a\x17\x07W`@Qc\x03m2\xEF`\xE4\x1B\x81R`\x04\x81\x01\x84\x90R`$\x01a\x03\xC8V[`\0a\x17\x93g\r\xE0\xB6\xB3\xA7d\0\0\x83\x04`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x91\x82\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11`\x06\x1B\x90\x81\x1Cc\xFF\xFF\xFF\xFF\x81\x11`\x05\x1B\x90\x81\x1Ca\xFF\xFF\x81\x11`\x04\x1B\x90\x81\x1C`\xFF\x81\x11`\x03\x90\x81\x1B\x91\x82\x1C`\x0F\x81\x11`\x02\x1B\x90\x81\x1C\x91\x82\x11\x87\x1B\x91\x82\x1C\x96\x90\x96\x11\x94\x90\x96\x17\x92\x90\x92\x17\x17\x17\x91\x90\x91\x17\x91\x90\x91\x17\x17\x90V[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0\x81\x02\x82\x82\x1Cg\r\xE0\xB6\xB3\xA7c\xFF\xFF\x19\x81\x01a\x17\xBCWP\x94\x93PPPPV[g\x1B\xC1mgN\xC8\0\0g\x06\xF0[Y\xD3\xB2\0\0[\x80\x15a\x17\xFFWg\r\xE0\xB6\xB3\xA7d\0\0\x83\x80\x02\x04\x92P\x81\x83\x10a\x17\xF7W\x92\x83\x01\x92`\x01\x92\x90\x92\x1C\x91[`\x01\x1Ca\x17\xCFV[P\x82a\x0CBV[`\0\x81h\nh\x89\x06\xBD\x8A\xFF\xFF\xFF\x81\x11\x15a\x186W`@Qc\xB3\xB6\xBA\x1F`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x01a\x03\xC8V[`\0a\x18Ng\r\xE0\xB6\xB3\xA7d\0\0`@\x84\x90\x1Ba!7V[\x90Pa\x12\xBBa\x13\x19\x82`\x01`\xBF\x1Bg\xFF\0\0\0\0\0\0\0\x82\x16\x15a\x19dWg\x80\0\0\0\0\0\0\0\x82\x16\x15a\x18\x8BWh\x01j\t\xE6g\xF3\xBC\xC9\t\x02`@\x1C[g@\0\0\0\0\0\0\0\x82\x16\x15a\x18\xAAWh\x010o\xE0\xA3\x1BqR\xDF\x02`@\x1C[g \0\0\0\0\0\0\0\x82\x16\x15a\x18\xC9Wh\x01\x17+\x83\xC7\xD5\x17\xAD\xCE\x02`@\x1C[g\x10\0\0\0\0\0\0\0\x82\x16\x15a\x18\xE8Wh\x01\x0BU\x86\xCF\x98\x90\xF6*\x02`@\x1C[g\x08\0\0\0\0\0\0\0\x82\x16\x15a\x19\x07Wh\x01\x05\x9B\r1XWC\xAE\x02`@\x1C[g\x04\0\0\0\0\0\0\0\x82\x16\x15a\x19&Wh\x01\x02\xC9\xA3\xE7x\x06\x0E\xE7\x02`@\x1C[g\x02\0\0\0\0\0\0\0\x82\x16\x15a\x19EWh\x01\x01c\xDA\x9F\xB33V\xD8\x02`@\x1C[g\x01\0\0\0\0\0\0\0\x82\x16\x15a\x19dWh\x01\0\xB1\xAF\xA5\xAB\xCB\xEDa\x02`@\x1C[f\xFF\0\0\0\0\0\0\x82\x16\x15a\x1AcWf\x80\0\0\0\0\0\0\x82\x16\x15a\x19\x91Wh\x01\0X\xC8m\xA1\xC0\x9E\xA2\x02`@\x1C[f@\0\0\0\0\0\0\x82\x16\x15a\x19\xAFWh\x01\0,`^.\x8C\xECP\x02`@\x1C[f \0\0\0\0\0\0\x82\x16\x15a\x19\xCDWh\x01\0\x16/9\x04\x05\x1F\xA1\x02`@\x1C[f\x10\0\0\0\0\0\0\x82\x16\x15a\x19\xEBWh\x01\0\x0B\x17^\xFF\xDCv\xBA\x02`@\x1C[f\x08\0\0\0\0\0\0\x82\x16\x15a\x1A\tWh\x01\0\x05\x8B\xA0\x1F\xB9\xF9m\x02`@\x1C[f\x04\0\0\0\0\0\0\x82\x16\x15a\x1A'Wh\x01\0\x02\xC5\xCC7\xDA\x94\x92\x02`@\x1C[f\x02\0\0\0\0\0\0\x82\x16\x15a\x1AEWh\x01\0\x01b\xE5%\xEE\x05G\x02`@\x1C[f\x01\0\0\0\0\0\0\x82\x16\x15a\x1AcWh\x01\0\0\xB1rUw\\\x04\x02`@\x1C[e\xFF\0\0\0\0\0\x82\x16\x15a\x1BYWe\x80\0\0\0\0\0\x82\x16\x15a\x1A\x8EWh\x01\0\0X\xB9\x1B[\xC9\xAE\x02`@\x1C[e@\0\0\0\0\0\x82\x16\x15a\x1A\xABWh\x01\0\0,\\\x89\xD5\xECm\x02`@\x1C[e \0\0\0\0\0\x82\x16\x15a\x1A\xC8Wh\x01\0\0\x16.C\xF4\xF81\x02`@\x1C[e\x10\0\0\0\0\0\x82\x16\x15a\x1A\xE5Wh\x01\0\0\x0B\x17!\xBC\xFC\x9A\x02`@\x1C[e\x08\0\0\0\0\0\x82\x16\x15a\x1B\x02Wh\x01\0\0\x05\x8B\x90\xCF\x1En\x02`@\x1C[e\x04\0\0\0\0\0\x82\x16\x15a\x1B\x1FWh\x01\0\0\x02\xC5\xC8c\xB7?\x02`@\x1C[e\x02\0\0\0\0\0\x82\x16\x15a\x1B<Wh\x01\0\0\x01b\xE40\xE5\xA2\x02`@\x1C[e\x01\0\0\0\0\0\x82\x16\x15a\x1BYWh\x01\0\0\0\xB1r\x185Q\x02`@\x1C[d\xFF\0\0\0\0\x82\x16\x15a\x1CFWd\x80\0\0\0\0\x82\x16\x15a\x1B\x82Wh\x01\0\0\0X\xB9\x0C\x0BI\x02`@\x1C[d@\0\0\0\0\x82\x16\x15a\x1B\x9EWh\x01\0\0\0,\\\x86\x01\xCC\x02`@\x1C[d \0\0\0\0\x82\x16\x15a\x1B\xBAWh\x01\0\0\0\x16.B\xFF\xF0\x02`@\x1C[d\x10\0\0\0\0\x82\x16\x15a\x1B\xD6Wh\x01\0\0\0\x0B\x17!\x7F\xBB\x02`@\x1C[d\x08\0\0\0\0\x82\x16\x15a\x1B\xF2Wh\x01\0\0\0\x05\x8B\x90\xBF\xCE\x02`@\x1C[d\x04\0\0\0\0\x82\x16\x15a\x1C\x0EWh\x01\0\0\0\x02\xC5\xC8_\xE3\x02`@\x1C[d\x02\0\0\0\0\x82\x16\x15a\x1C*Wh\x01\0\0\0\x01b\xE4/\xF1\x02`@\x1C[d\x01\0\0\0\0\x82\x16\x15a\x1CFWh\x01\0\0\0\0\xB1r\x17\xF8\x02`@\x1C[c\xFF\0\0\0\x82\x16\x15a\x1D*Wc\x80\0\0\0\x82\x16\x15a\x1CmWh\x01\0\0\0\0X\xB9\x0B\xFC\x02`@\x1C[c@\0\0\0\x82\x16\x15a\x1C\x88Wh\x01\0\0\0\0,\\\x85\xFE\x02`@\x1C[c \0\0\0\x82\x16\x15a\x1C\xA3Wh\x01\0\0\0\0\x16.B\xFF\x02`@\x1C[c\x10\0\0\0\x82\x16\x15a\x1C\xBEWh\x01\0\0\0\0\x0B\x17!\x7F\x02`@\x1C[c\x08\0\0\0\x82\x16\x15a\x1C\xD9Wh\x01\0\0\0\0\x05\x8B\x90\xC0\x02`@\x1C[c\x04\0\0\0\x82\x16\x15a\x1C\xF4Wh\x01\0\0\0\0\x02\xC5\xC8`\x02`@\x1C[c\x02\0\0\0\x82\x16\x15a\x1D\x0FWh\x01\0\0\0\0\x01b\xE40\x02`@\x1C[c\x01\0\0\0\x82\x16\x15a\x1D*Wh\x01\0\0\0\0\0\xB1r\x18\x02`@\x1C[b\xFF\0\0\x82\x16\x15a\x1E\x05Wb\x80\0\0\x82\x16\x15a\x1DOWh\x01\0\0\0\0\0X\xB9\x0C\x02`@\x1C[b@\0\0\x82\x16\x15a\x1DiWh\x01\0\0\0\0\0,\\\x86\x02`@\x1C[b \0\0\x82\x16\x15a\x1D\x83Wh\x01\0\0\0\0\0\x16.C\x02`@\x1C[b\x10\0\0\x82\x16\x15a\x1D\x9DWh\x01\0\0\0\0\0\x0B\x17!\x02`@\x1C[b\x08\0\0\x82\x16\x15a\x1D\xB7Wh\x01\0\0\0\0\0\x05\x8B\x91\x02`@\x1C[b\x04\0\0\x82\x16\x15a\x1D\xD1Wh\x01\0\0\0\0\0\x02\xC5\xC8\x02`@\x1C[b\x02\0\0\x82\x16\x15a\x1D\xEBWh\x01\0\0\0\0\0\x01b\xE4\x02`@\x1C[b\x01\0\0\x82\x16\x15a\x1E\x05Wh\x01\0\0\0\0\0\0\xB1r\x02`@\x1C[a\xFF\0\x82\x16\x15a\x1E\xD7Wa\x80\0\x82\x16\x15a\x1E(Wh\x01\0\0\0\0\0\0X\xB9\x02`@\x1C[a@\0\x82\x16\x15a\x1EAWh\x01\0\0\0\0\0\0,]\x02`@\x1C[a \0\x82\x16\x15a\x1EZWh\x01\0\0\0\0\0\0\x16.\x02`@\x1C[a\x10\0\x82\x16\x15a\x1EsWh\x01\0\0\0\0\0\0\x0B\x17\x02`@\x1C[a\x08\0\x82\x16\x15a\x1E\x8CWh\x01\0\0\0\0\0\0\x05\x8C\x02`@\x1C[a\x04\0\x82\x16\x15a\x1E\xA5Wh\x01\0\0\0\0\0\0\x02\xC6\x02`@\x1C[a\x02\0\x82\x16\x15a\x1E\xBEWh\x01\0\0\0\0\0\0\x01c\x02`@\x1C[a\x01\0\x82\x16\x15a\x1E\xD7Wh\x01\0\0\0\0\0\0\0\xB1\x02`@\x1C[`\xFF\x82\x16\x15a\x1F\xA0W`\x80\x82\x16\x15a\x1E\xF8Wh\x01\0\0\0\0\0\0\0Y\x02`@\x1C[`@\x82\x16\x15a\x1F\x10Wh\x01\0\0\0\0\0\0\0,\x02`@\x1C[` \x82\x16\x15a\x1F(Wh\x01\0\0\0\0\0\0\0\x16\x02`@\x1C[`\x10\x82\x16\x15a\x1F@Wh\x01\0\0\0\0\0\0\0\x0B\x02`@\x1C[`\x08\x82\x16\x15a\x1FXWh\x01\0\0\0\0\0\0\0\x06\x02`@\x1C[`\x04\x82\x16\x15a\x1FpWh\x01\0\0\0\0\0\0\0\x03\x02`@\x1C[`\x02\x82\x16\x15a\x1F\x88Wh\x01\0\0\0\0\0\0\0\x01\x02`@\x1C[`\x01\x82\x16\x15a\x1F\xA0Wh\x01\0\0\0\0\0\0\0\x01\x02`@\x1C[g\r\xE0\xB6\xB3\xA7d\0\0\x02`@\x91\x90\x91\x1C`\xBF\x03\x1C\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[\x80\x15\x15\x81\x14a \x15W`\0\x80\xFD[PV[`\0\x80`@\x83\x85\x03\x12\x15a .Wa .a\x1F\xB7V[\x825a 9\x81a \x07V[\x94` \x93\x90\x93\x015\x93PPPV[`\0` \x82\x84\x03\x12\x15a \\Wa \\a\x1F\xB7V[P5\x91\x90PV[`\0` \x82\x84\x03\x12\x15a xWa xa\x1F\xB7V[\x815`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x12}W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a \xA5Wa \xA5a\x1F\xB7V[PP\x805\x92` \x90\x91\x015\x91PV[`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x81R\x91\x90\x92\x16` \x82\x01R`@\x81\x01\x91\x90\x91R``\x01\x90V[`\0` \x82\x84\x03\x12\x15a \xEDWa \xEDa\x1F\xB7V[\x81Qa\x12}\x81a \x07V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x03\x80Wa\x03\x80a \xF8V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a!TWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x03\x80Wa\x03\x80a \xF8V[\x80\x82\x01\x80\x82\x11\x15a\x03\x80Wa\x03\x80a \xF8V\xFETarget contract does not contain\xA2dipfsX\"\x12 \xD0vq\x15\x12\xE8J\xB4\xB3~s\x99eE\x01G\xBA\xD2N\x80\xF7\x17\xD9ADV\xB7\xDC\xBE\xCFs dsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static G3M_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x01\x84W`\x005`\xE0\x1C\x80c\x9C\x8F\x9F#\x11a\x01\x05W\x80c\xC0\xFF\x1A\x15\x11a\0\xC9W\x80c\xC0\xFF\x1A\x15\x14a\x03\x16W\x80c\xDBy\x10C\x14a\x03\x1EW\x80c\xDCv\xFA\xBC\x14a\x031W\x80c\xE3\x11\xCE\xC8\x14a\x039W\x80c\xF8Q\xA4@\x14a\x03AW\x80c\xFA\xDF\xA6[\x14a\x03TWa\x01\x84V[\x80c\x9C\x8F\x9F#\x14a\x02\xC2W\x80c\x9C\xE32\xD4\x14a\x02\xD5W\x80c\x9E\x1B\0E\x14a\x02\xE8W\x80c\xAD\xB5\x1D\xEE\x14a\x02\xFBW\x80c\xB7\xD1\x9F\xC4\x14a\x03\x03Wa\x01\x84V[\x80c4\xE1\x99\x07\x11a\x01LW\x80c4\xE1\x99\x07\x14a\x02TW\x80cQ\xC6Y\n\x14a\x02iW\x80cT\xCF*\xEB\x14a\x02\x91W\x80cp\xA0\x821\x14a\x02\x9AW\x80c\x8AYS\xC7\x14a\x02\xBAWa\x01\x84V[\x80c\x08\xEA\xBD\xDA\x14a\x01\xE9W\x80c\x15w\x0F\x92\x14a\x02\x05W\x80c\x16\xDC\x16[\x14a\x02\x0EW\x80c\x194\xEB%\x14a\x029W\x80c\x1F\xDA\xBC'\x14a\x02AW[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x01\xF2`\x04T\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\xF2`\x06T\x81V[`\x01Ta\x02!\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\xFCV[a\x01\xF2a\x03]V[a\x01\xF2a\x02O6`\x04a \x18V[a\x03oV[a\x02ga\x02b6`\x04a GV[a\x03\x86V[\0[a\x02|a\x02w6`\x04a GV[a\x04\x1CV[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x01\xFCV[a\x01\xF2`\x03T\x81V[a\x01\xF2a\x02\xA86`\x04a cV[`\x07` R`\0\x90\x81R`@\x90 T\x81V[a\x01\xF2a\x06\xC6V[a\x02|a\x02\xD06`\x04a GV[a\x06\xE1V[a\x01\xF2a\x02\xE36`\x04a \x18V[a\t\x8AV[a\x01\xF2a\x02\xF66`\x04a \x8FV[a\t\x98V[a\x01\xF2a\x0CMV[`\x02Ta\x02!\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01\xF2a\x0C\xBAV[a\x02ga\x03,6`\x04a \x8FV[a\x0C\xD5V[a\x01\xF2a\x0EbV[a\x01\xF2a\x0E\x82V[`\0Ta\x02!\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01\xF2`\x05T\x81V[`\0a\x03j`\x05Ta\x0E\x8BV[\x90P\x90V[`\0a\x03}\x83`\x01\x84a\x0E\x9FV[\x90P[\x92\x91PPV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x03\xD1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh'7\xBA\x100\xB26\xB4\xB7`\xB9\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[a'\x10\x81\x11\x15a\x04\x17W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01Rp\nn\xEC.\x04\x0C\xCC\xAC\xA4\x0E\x8D\xED\xE4\r\r,\xED`{\x1B`D\x82\x01R`d\x01a\x03\xC8V[`\x03UV[`\0\x80a\x04)`\x06T\x15\x90V[\x15a\x04mW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rs\x14\x1B\xDB\xDB\x08\x1B\x9B\xDD\x08\x1A[\x9A]\x1AX[\x1A^\x99Y`b\x1B`D\x82\x01R`d\x01a\x03\xC8V[a\x04|`\x06T\x84`\x04Ta\x12\x84V[\x91Pa\x04\x8D`\x06T\x84`\x05Ta\x12\x84V[`\x01T`@Qc#\xB8r\xDD`\xE0\x1B\x81R\x91\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c#\xB8r\xDD\x90a\x04\xC2\x903\x900\x90\x87\x90`\x04\x01a \xB4V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x05\x1AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a!\x84\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x05.W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05R\x91\x90a \xD8V[P`\x02T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\x05\x87\x903\x900\x90\x86\x90`\x04\x01a \xB4V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x05\xDFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a!\x84\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x05\xF3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\x17\x91\x90a \xD8V[P`@\x80Q\x84\x81R` \x81\x01\x84\x90R\x90\x81\x01\x82\x90R3\x90\x7F\xBE\xB3\x88W\x86\xD67\xA4t\xCB\xC2\x87\xC0\xA4E\x87#\x163\xA0w\xF0\xBD05MZK\x18\x99o\xCE\x90``\x01`@Q\x80\x91\x03\x90\xA2a\x06o`\x04Ta\x06j\x84a\x12\xC3V[a\x13\nV[`\x04U`\x05Ta\x06\x82\x90a\x06j\x83a\x12\xC3V[`\x05U3`\0\x90\x81R`\x07` R`@\x90 Ta\x06\x9F\x90\x84a\x13\nV[3`\0\x90\x81R`\x07` R`@\x90 U`\x06Ta\x06\xBC\x90\x84a\x13\nV[`\x06U\x90\x92\x90\x91PV[`\0a\x03jg\r\xE0\xB6\xB3\xA7d\0\0a\x06\xDCa\x0CMV[a\x13\x1CV[3`\0\x90\x81R`\x07` R`@\x81 T\x81\x90\x83\x11\x15a\x07;W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01RuInsufficient liquidity`P\x1B`D\x82\x01R`d\x01a\x03\xC8V[a\x07J`\x06T\x84`\x04Ta\x13+V[\x91Pa\x07[`\x06T\x84`\x05Ta\x13+V[`\x01T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x85\x90R\x91\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x07\xE6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a!\x84\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x07\xFAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\x1E\x91\x90a \xD8V[P`\x02T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x83\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x08\xA9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a!\x84\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x08\xBDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xE1\x91\x90a \xD8V[P3`\0\x90\x81R`\x07` R`@\x90 Ta\x08\xFC\x90\x84a\x13\x1CV[3`\0\x90\x81R`\x07` R`@\x90 U`\x06Ta\t\x19\x90\x84a\x13\x1CV[`\x06U`\x04Ta\t,\x90a\x06\xDC\x84a\x12\xC3V[`\x04U`\x05Ta\t?\x90a\x06\xDC\x83a\x12\xC3V[`\x05U`@\x80Q\x84\x81R` \x81\x01\x84\x90R\x90\x81\x01\x82\x90R3\x90\x7FY\xC3\xA0\xB6\x0Cj\xB7\xDE\xB6.\x14@\xC9\xE7$A\xDBm\xB7\xDF\xE5\x14\xDB\xA8\xCB\x18\xE6\x0C\r\x89n\xFA\x90``\x01`@Q\x80\x91\x03\x90\xA2\x91P\x91V[`\0a\x03}\x83`\0\x84a\x0E\x9FV[`\0a\t\xA4`\x06T\x15\x90V[a\t\xF0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FPool already initialized\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x03\xC8V[`\0a\t\xFB\x84a\x12\xC3V[\x90P`\0a\n\x08\x84a\x12\xC3V[\x90P`\0a\n6a\n\x18\x87a\x12\xC3V[a\n a\x0CMV[a\n)\x88a\x12\xC3V[a\n1a\x06\xC6V[a\x13RV[\x90P`\0a\nM\x82a\nH`\x02a\x12\xC3V[a\x13uV[\x90Pa\n[`\x06T\x82a\x13\nV[`\x06Ua\nj\x81a\x03\xE8a\x13\x1CV[3`\0\x81\x81R`\x07` R`@\x80\x82 \x93\x90\x93U\x80Ra\x03\xE8\x7FmRW N\xBE}\x88\xFD\x91\xAE\x87\x94\x1C\xB2\xDD\x9D\x80b\xB6J\xE5\xA2\xBD-(\xEC@\xB9\xFB\xF6\xDFU`\x04\x86\x81U`\x05\x86\x90U`\x01T\x92Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92c#\xB8r\xDD\x92a\n\xE0\x92\x90\x910\x91\x8D\x91\x01a \xB4V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0B8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a!\x84\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x0BLW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Bp\x91\x90a \xD8V[P`\x02T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\x0B\xA5\x903\x900\x90\x8B\x90`\x04\x01a \xB4V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0B\xFDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a!\x84\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x0C\x11W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C5\x91\x90a \xD8V[Pa\x0CB\x81a\x03\xE8a\x13\x1CV[\x97\x96PPPPPPPV[`\0`\x0BTB\x10a\x0C_WP`\nT\x90V[`\0`\tTBa\x0Co\x91\x90a!\x0EV[\x90P`\0a\x0C\x87a\x0C\x7F\x83a\x12\xC3V[`\x0CTa\x13uV[\x90Pa\x0C\x96`\x08T`\nT\x10\x90V[\x15a\x0C\xAEWa\x0C\xA7`\x08T\x82a\x13\x1CV[\x92PPP\x90V[a\x0C\xA7`\x08T\x82a\x13\nV[`\0a\x03j`\x04Ta\x0C\xCAa\x0CMV[`\x05Ta\n1a\x06\xC6V[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\r\x1BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh'7\xBA\x100\xB26\xB4\xB7`\xB9\x1B`D\x82\x01R`d\x01a\x03\xC8V[a\r,\x82f#\x86\xF2o\xC1\0\0\x11\x15\x90V[a\rkW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01RoWeight X too low`\x80\x1B`D\x82\x01R`d\x01a\x03\xC8V[a\r}\x82g\r\xBD/\xC17\xA3\0\0\x10\x15\x90V[a\r\xBDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01Rp\n\xEC\xAD,\xED\x0E\x84\x0B\x04\x0E\x8D\xED\xE4\r\r,\xED`{\x1B`D\x82\x01R`d\x01a\x03\xC8V[B\x81\x11a\x0E\0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01Rp\x15\\\x19\x18]\x19H\x19[\x99\x08\x1C\x18\\\xDD\x19Y`z\x1B`D\x82\x01R`d\x01a\x03\xC8V[a\x0E\x08a\x13\x84V[`\0a\x0E\x15`\x08T\x84\x10\x90V[a\x0E*Wa\x0E%\x83`\x08Ta\x13\x1CV[a\x0E6V[a\x0E6`\x08T\x84a\x13\x1CV[\x90Pa\x0ES\x81a\x0ENa\x0EIB\x86a!\x0EV[a\x12\xC3V[a\x13\x95V[`\x0CUP`\n\x91\x90\x91U`\x0BUV[`\0a\x03j`\x04Ta\x0Era\x0CMV[`\x05Ta\x0E}a\x06\xC6V[a\x13\xADV[`\0a\x03j`\x04T[`\0a\x03\x80g\r\xE0\xB6\xB3\xA7d\0\0\x83a!7V[`\0\x80a\x0E\xAAa\x0CMV[\x90P`\0a\x0E\xB6a\x06\xC6V[\x90P`\0a\x0E\xCA`\x04T\x84`\x05T\x85a\x13RV[\x90P`\0\x80\x87\x15a\x0FRW\x86\x91P`\0a'\x10`\x03T\x84a\x0E\xEB\x91\x90a!YV[a\x0E\xF5\x91\x90a!7V[\x90P`\0a\x0F\x03\x82\x85a!\x0EV[\x90Pa\x0FI\x81\x8Ca\x0F\x16W`\x05Ta\x0F\x1AV[`\x04T[\x8Da\x0F%W\x88a\x0F'V[\x89[\x8Ea\x0F4W`\x04Ta\x0F8V[`\x05T[\x8Fa\x0FCW\x8Ba\x13\xD4V[\x8Aa\x13\xD4V[\x92PPPa\x0F\xC6V[P\x85`\0a\x0F\x9A\x82\x8Ba\x0FgW`\x05Ta\x0FkV[`\x04T[\x8Ca\x0FvW\x87a\x0FxV[\x88[\x8Da\x0F\x85W`\x04Ta\x0F\x89V[`\x05T[\x8Ea\x0F\x94W\x8Aa\x14\x11V[\x89a\x14\x11V[\x90P`\x03Ta'\x10a\x0F\xAC\x91\x90a!\x0EV[a\x0F\xB8\x82a'\x10a!YV[a\x0F\xC2\x91\x90a!7V[\x92PP[\x88\x15a\x0F\xF6Wa\x0F\xDB`\x04Ta\x06j\x84a\x12\xC3V[`\x04U`\x05Ta\x0F\xEE\x90a\x06\xDC\x83a\x12\xC3V[`\x05Ua\x10\x1CV[a\x10\x05`\x04Ta\x06\xDC\x83a\x12\xC3V[`\x04U`\x05Ta\x10\x18\x90a\x06j\x84a\x12\xC3V[`\x05U[`\0a\x10.`\x04T\x87`\x05T\x88a\x13RV[\x90P\x83\x81\x10\x15a\x10[W`@Qc,\x88t9`\xE2\x1B\x81R`\x04\x81\x01\x85\x90R`$\x81\x01\x82\x90R`D\x01a\x03\xC8V[\x89a\x10qW`\x02T`\x01`\x01`\xA0\x1B\x03\x16a\x10~V[`\x01T`\x01`\x01`\xA0\x1B\x03\x16[`\x01`\x01`\xA0\x1B\x03\x16c#\xB8r\xDD30\x86`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x10\xAD\x93\x92\x91\x90a \xB4V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x11\x05W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a!\x84\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x11\x19W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11=\x91\x90a \xD8V[P\x89a\x11TW`\x01T`\x01`\x01`\xA0\x1B\x03\x16a\x11aV[`\x02T`\x01`\x01`\xA0\x1B\x03\x16[`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x11\xE9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a!\x84\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x11\xFDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12!\x91\x90a \xD8V[P`@\x80Q\x8B\x15\x15\x81R` \x81\x01\x85\x90R\x90\x81\x01\x83\x90R3\x90\x7F\xBF\xD5\n\x04\xF1\xE6\xE4\xAE\xE3D\xF5\xD0\xE7\xF1]t\xD0\xDB\xB5\x8C\xD1\xF7\x11\xDA\xA6F0\x94\xCA\x95\x08\xCD\x90``\x01`@Q\x80\x91\x03\x90\xA2\x88a\x12rW\x82a\x12tV[\x81[\x96PPPPPPP[\x93\x92PPPV[`\0a\x12\xBBa\x12\xB6a\x12\xB0a\x12\xA2a\x12\x9C\x88\x88a\x13\nV[\x88a\x13\x95V[g\r\xE0\xB6\xB3\xA7d\0\0a\x13\x1CV[\x84a\x13uV[a\x0E\x8BV[\x94\x93PPPPV[`\0a\x12\xD9g\r\xE0\xB6\xB3\xA7d\0\0`\0\x19a!7V[\x82\x11\x15a\x12\xFCW`@Qc\x1C\xD9Q\xA7`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R`$\x01a\x03\xC8V[Pg\r\xE0\xB6\xB3\xA7d\0\0\x02\x90V[`\0a\x03}a\x13\x19\x83\x85a!pV[\x90V[`\0a\x03}a\x13\x19\x83\x85a!\x0EV[`\0a\x12\xBBa\x12\xB6a\x12\xB0g\r\xE0\xB6\xB3\xA7d\0\0a\x06\xDCa\x13L\x89\x89a\x13\x1CV[\x89a\x13\x95V[`\0\x80a\x13_\x86\x86a\x14AV[\x90P`\0a\x13m\x85\x85a\x14AV[\x90Pa\x0CB\x82\x82[`\0a\x03}a\x13\x19\x84\x84a\x15NV[a\x13\x8Ca\x0CMV[`\x08UB`\tUV[`\0a\x03}a\x13\x19\x84g\r\xE0\xB6\xB3\xA7d\0\0\x85a\x16\x04V[`\0\x80a\x13\xBA\x84\x84a\x13\x95V[\x90P`\0a\x13\xC8\x87\x87a\x13\x95V[\x90Pa\x0CB\x82\x82a\x13\x95V[`\0\x80a\x13\xE8\x86a\x0EN\x88a\x06j\x8Ba\x12\xC3V[\x90Pa\x0CBa\x12\xB6\x85a\nHg\r\xE0\xB6\xB3\xA7d\0\0a\x06\xDCa\x14\n\x8B\x8Aa\x13\x95V[\x87\x90a\x14AV[`\0\x80a\x14%\x84a\x0EN\x86a\x06\xDC\x8Ba\x12\xC3V[\x90Pa\x0CBa\x12\xB6\x87a\nHa\x12\xA2a\x14>\x88\x8Ba\x13\x95V[\x86\x90[`\0\x82\x82\x81\x83\x03a\x14lW\x80\x15a\x14YW`\0a\x14cV[g\r\xE0\xB6\xB3\xA7d\0\0[\x92PPPa\x03\x80V[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x03a\x14\x8DWg\r\xE0\xB6\xB3\xA7d\0\0\x92PPPa\x03\x80V[\x80`\0\x03a\x14\xA7Wg\r\xE0\xB6\xB3\xA7d\0\0\x92PPPa\x03\x80V[g\r\xE0\xB6\xB3\xA7d\0\0\x81\x03a\x14\xC0W\x84\x92PPPa\x03\x80V[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x11\x15a\x14\xF1Wa\x14\xEAa\x14\xE5a\x14\xDF\x87a\x16\xD8V[\x86a\x13uV[a\x18\x06V[\x92Pa\x15FV[`\0a\x15\x0Fa\x13\x19\x84n\xC0\x97\xCE{\xC9\x07\x15\xB3K\x9F\x10\0\0\0\0a!7V[\x90P`\0a\x15(a\x14\xE5a\x15\"\x84a\x16\xD8V[\x88a\x13uV[\x90Pa\x0CBa\x13\x19\x82n\xC0\x97\xCE{\xC9\x07\x15\xB3K\x9F\x10\0\0\0\0a!7V[PP\x92\x91PPV[`\0\x80\x80`\0\x19\x84\x86\t\x84\x86\x02\x92P\x82\x81\x10\x83\x82\x03\x03\x91PP\x80`\0\x03a\x15\x82WPg\r\xE0\xB6\xB3\xA7d\0\0\x90\x04\x90Pa\x03\x80V[g\r\xE0\xB6\xB3\xA7d\0\0\x81\x10a\x15\xB4W`@QcQsd\x8D`\xE0\x1B\x81R`\x04\x81\x01\x86\x90R`$\x81\x01\x85\x90R`D\x01a\x03\xC8V[`\0g\r\xE0\xB6\xB3\xA7d\0\0\x85\x87\tb\x04\0\0\x81\x85\x03\x04\x93\x10\x90\x91\x03`\x01`\xEE\x1B\x02\x91\x90\x91\x17\x7F\xAC\xCB\x18\x16[\xD6\xFE1\xAE\x1C\xF3\x18\xDC[Q\xEE\xE0\xE1\xBAV\x9B\x88\xCDt\xC1w;\x91\xFA\xC1\x06i\x02\x91PP\x92\x91PPV[`\0\x80\x80`\0\x19\x85\x87\t\x85\x87\x02\x92P\x82\x81\x10\x83\x82\x03\x03\x91PP\x80`\0\x03a\x16>W\x83\x82\x81a\x164Wa\x164a!!V[\x04\x92PPPa\x12}V[\x83\x81\x10a\x16oW`@Qc\x0Ct\n\xEF`\xE3\x1B\x81R`\x04\x81\x01\x87\x90R`$\x81\x01\x86\x90R`D\x81\x01\x85\x90R`d\x01a\x03\xC8V[`\0\x84\x86\x88\t`\x02`\x01\x87\x19\x81\x01\x88\x16\x97\x88\x90\x04`\x03\x81\x02\x83\x18\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x90\x81\x02\x90\x92\x03\x90\x91\x02`\0\x88\x90\x03\x88\x90\x04\x90\x91\x01\x85\x83\x11\x90\x94\x03\x93\x90\x93\x02\x93\x03\x94\x90\x94\x04\x91\x90\x91\x17\x02\x94\x93PPPPV[`\0\x81g\r\xE0\xB6\xB3\xA7d\0\0\x81\x10\x15a\x17\x07W`@Qc\x03m2\xEF`\xE4\x1B\x81R`\x04\x81\x01\x84\x90R`$\x01a\x03\xC8V[`\0a\x17\x93g\r\xE0\xB6\xB3\xA7d\0\0\x83\x04`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x91\x82\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11`\x06\x1B\x90\x81\x1Cc\xFF\xFF\xFF\xFF\x81\x11`\x05\x1B\x90\x81\x1Ca\xFF\xFF\x81\x11`\x04\x1B\x90\x81\x1C`\xFF\x81\x11`\x03\x90\x81\x1B\x91\x82\x1C`\x0F\x81\x11`\x02\x1B\x90\x81\x1C\x91\x82\x11\x87\x1B\x91\x82\x1C\x96\x90\x96\x11\x94\x90\x96\x17\x92\x90\x92\x17\x17\x17\x91\x90\x91\x17\x91\x90\x91\x17\x17\x90V[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0\x81\x02\x82\x82\x1Cg\r\xE0\xB6\xB3\xA7c\xFF\xFF\x19\x81\x01a\x17\xBCWP\x94\x93PPPPV[g\x1B\xC1mgN\xC8\0\0g\x06\xF0[Y\xD3\xB2\0\0[\x80\x15a\x17\xFFWg\r\xE0\xB6\xB3\xA7d\0\0\x83\x80\x02\x04\x92P\x81\x83\x10a\x17\xF7W\x92\x83\x01\x92`\x01\x92\x90\x92\x1C\x91[`\x01\x1Ca\x17\xCFV[P\x82a\x0CBV[`\0\x81h\nh\x89\x06\xBD\x8A\xFF\xFF\xFF\x81\x11\x15a\x186W`@Qc\xB3\xB6\xBA\x1F`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x01a\x03\xC8V[`\0a\x18Ng\r\xE0\xB6\xB3\xA7d\0\0`@\x84\x90\x1Ba!7V[\x90Pa\x12\xBBa\x13\x19\x82`\x01`\xBF\x1Bg\xFF\0\0\0\0\0\0\0\x82\x16\x15a\x19dWg\x80\0\0\0\0\0\0\0\x82\x16\x15a\x18\x8BWh\x01j\t\xE6g\xF3\xBC\xC9\t\x02`@\x1C[g@\0\0\0\0\0\0\0\x82\x16\x15a\x18\xAAWh\x010o\xE0\xA3\x1BqR\xDF\x02`@\x1C[g \0\0\0\0\0\0\0\x82\x16\x15a\x18\xC9Wh\x01\x17+\x83\xC7\xD5\x17\xAD\xCE\x02`@\x1C[g\x10\0\0\0\0\0\0\0\x82\x16\x15a\x18\xE8Wh\x01\x0BU\x86\xCF\x98\x90\xF6*\x02`@\x1C[g\x08\0\0\0\0\0\0\0\x82\x16\x15a\x19\x07Wh\x01\x05\x9B\r1XWC\xAE\x02`@\x1C[g\x04\0\0\0\0\0\0\0\x82\x16\x15a\x19&Wh\x01\x02\xC9\xA3\xE7x\x06\x0E\xE7\x02`@\x1C[g\x02\0\0\0\0\0\0\0\x82\x16\x15a\x19EWh\x01\x01c\xDA\x9F\xB33V\xD8\x02`@\x1C[g\x01\0\0\0\0\0\0\0\x82\x16\x15a\x19dWh\x01\0\xB1\xAF\xA5\xAB\xCB\xEDa\x02`@\x1C[f\xFF\0\0\0\0\0\0\x82\x16\x15a\x1AcWf\x80\0\0\0\0\0\0\x82\x16\x15a\x19\x91Wh\x01\0X\xC8m\xA1\xC0\x9E\xA2\x02`@\x1C[f@\0\0\0\0\0\0\x82\x16\x15a\x19\xAFWh\x01\0,`^.\x8C\xECP\x02`@\x1C[f \0\0\0\0\0\0\x82\x16\x15a\x19\xCDWh\x01\0\x16/9\x04\x05\x1F\xA1\x02`@\x1C[f\x10\0\0\0\0\0\0\x82\x16\x15a\x19\xEBWh\x01\0\x0B\x17^\xFF\xDCv\xBA\x02`@\x1C[f\x08\0\0\0\0\0\0\x82\x16\x15a\x1A\tWh\x01\0\x05\x8B\xA0\x1F\xB9\xF9m\x02`@\x1C[f\x04\0\0\0\0\0\0\x82\x16\x15a\x1A'Wh\x01\0\x02\xC5\xCC7\xDA\x94\x92\x02`@\x1C[f\x02\0\0\0\0\0\0\x82\x16\x15a\x1AEWh\x01\0\x01b\xE5%\xEE\x05G\x02`@\x1C[f\x01\0\0\0\0\0\0\x82\x16\x15a\x1AcWh\x01\0\0\xB1rUw\\\x04\x02`@\x1C[e\xFF\0\0\0\0\0\x82\x16\x15a\x1BYWe\x80\0\0\0\0\0\x82\x16\x15a\x1A\x8EWh\x01\0\0X\xB9\x1B[\xC9\xAE\x02`@\x1C[e@\0\0\0\0\0\x82\x16\x15a\x1A\xABWh\x01\0\0,\\\x89\xD5\xECm\x02`@\x1C[e \0\0\0\0\0\x82\x16\x15a\x1A\xC8Wh\x01\0\0\x16.C\xF4\xF81\x02`@\x1C[e\x10\0\0\0\0\0\x82\x16\x15a\x1A\xE5Wh\x01\0\0\x0B\x17!\xBC\xFC\x9A\x02`@\x1C[e\x08\0\0\0\0\0\x82\x16\x15a\x1B\x02Wh\x01\0\0\x05\x8B\x90\xCF\x1En\x02`@\x1C[e\x04\0\0\0\0\0\x82\x16\x15a\x1B\x1FWh\x01\0\0\x02\xC5\xC8c\xB7?\x02`@\x1C[e\x02\0\0\0\0\0\x82\x16\x15a\x1B<Wh\x01\0\0\x01b\xE40\xE5\xA2\x02`@\x1C[e\x01\0\0\0\0\0\x82\x16\x15a\x1BYWh\x01\0\0\0\xB1r\x185Q\x02`@\x1C[d\xFF\0\0\0\0\x82\x16\x15a\x1CFWd\x80\0\0\0\0\x82\x16\x15a\x1B\x82Wh\x01\0\0\0X\xB9\x0C\x0BI\x02`@\x1C[d@\0\0\0\0\x82\x16\x15a\x1B\x9EWh\x01\0\0\0,\\\x86\x01\xCC\x02`@\x1C[d \0\0\0\0\x82\x16\x15a\x1B\xBAWh\x01\0\0\0\x16.B\xFF\xF0\x02`@\x1C[d\x10\0\0\0\0\x82\x16\x15a\x1B\xD6Wh\x01\0\0\0\x0B\x17!\x7F\xBB\x02`@\x1C[d\x08\0\0\0\0\x82\x16\x15a\x1B\xF2Wh\x01\0\0\0\x05\x8B\x90\xBF\xCE\x02`@\x1C[d\x04\0\0\0\0\x82\x16\x15a\x1C\x0EWh\x01\0\0\0\x02\xC5\xC8_\xE3\x02`@\x1C[d\x02\0\0\0\0\x82\x16\x15a\x1C*Wh\x01\0\0\0\x01b\xE4/\xF1\x02`@\x1C[d\x01\0\0\0\0\x82\x16\x15a\x1CFWh\x01\0\0\0\0\xB1r\x17\xF8\x02`@\x1C[c\xFF\0\0\0\x82\x16\x15a\x1D*Wc\x80\0\0\0\x82\x16\x15a\x1CmWh\x01\0\0\0\0X\xB9\x0B\xFC\x02`@\x1C[c@\0\0\0\x82\x16\x15a\x1C\x88Wh\x01\0\0\0\0,\\\x85\xFE\x02`@\x1C[c \0\0\0\x82\x16\x15a\x1C\xA3Wh\x01\0\0\0\0\x16.B\xFF\x02`@\x1C[c\x10\0\0\0\x82\x16\x15a\x1C\xBEWh\x01\0\0\0\0\x0B\x17!\x7F\x02`@\x1C[c\x08\0\0\0\x82\x16\x15a\x1C\xD9Wh\x01\0\0\0\0\x05\x8B\x90\xC0\x02`@\x1C[c\x04\0\0\0\x82\x16\x15a\x1C\xF4Wh\x01\0\0\0\0\x02\xC5\xC8`\x02`@\x1C[c\x02\0\0\0\x82\x16\x15a\x1D\x0FWh\x01\0\0\0\0\x01b\xE40\x02`@\x1C[c\x01\0\0\0\x82\x16\x15a\x1D*Wh\x01\0\0\0\0\0\xB1r\x18\x02`@\x1C[b\xFF\0\0\x82\x16\x15a\x1E\x05Wb\x80\0\0\x82\x16\x15a\x1DOWh\x01\0\0\0\0\0X\xB9\x0C\x02`@\x1C[b@\0\0\x82\x16\x15a\x1DiWh\x01\0\0\0\0\0,\\\x86\x02`@\x1C[b \0\0\x82\x16\x15a\x1D\x83Wh\x01\0\0\0\0\0\x16.C\x02`@\x1C[b\x10\0\0\x82\x16\x15a\x1D\x9DWh\x01\0\0\0\0\0\x0B\x17!\x02`@\x1C[b\x08\0\0\x82\x16\x15a\x1D\xB7Wh\x01\0\0\0\0\0\x05\x8B\x91\x02`@\x1C[b\x04\0\0\x82\x16\x15a\x1D\xD1Wh\x01\0\0\0\0\0\x02\xC5\xC8\x02`@\x1C[b\x02\0\0\x82\x16\x15a\x1D\xEBWh\x01\0\0\0\0\0\x01b\xE4\x02`@\x1C[b\x01\0\0\x82\x16\x15a\x1E\x05Wh\x01\0\0\0\0\0\0\xB1r\x02`@\x1C[a\xFF\0\x82\x16\x15a\x1E\xD7Wa\x80\0\x82\x16\x15a\x1E(Wh\x01\0\0\0\0\0\0X\xB9\x02`@\x1C[a@\0\x82\x16\x15a\x1EAWh\x01\0\0\0\0\0\0,]\x02`@\x1C[a \0\x82\x16\x15a\x1EZWh\x01\0\0\0\0\0\0\x16.\x02`@\x1C[a\x10\0\x82\x16\x15a\x1EsWh\x01\0\0\0\0\0\0\x0B\x17\x02`@\x1C[a\x08\0\x82\x16\x15a\x1E\x8CWh\x01\0\0\0\0\0\0\x05\x8C\x02`@\x1C[a\x04\0\x82\x16\x15a\x1E\xA5Wh\x01\0\0\0\0\0\0\x02\xC6\x02`@\x1C[a\x02\0\x82\x16\x15a\x1E\xBEWh\x01\0\0\0\0\0\0\x01c\x02`@\x1C[a\x01\0\x82\x16\x15a\x1E\xD7Wh\x01\0\0\0\0\0\0\0\xB1\x02`@\x1C[`\xFF\x82\x16\x15a\x1F\xA0W`\x80\x82\x16\x15a\x1E\xF8Wh\x01\0\0\0\0\0\0\0Y\x02`@\x1C[`@\x82\x16\x15a\x1F\x10Wh\x01\0\0\0\0\0\0\0,\x02`@\x1C[` \x82\x16\x15a\x1F(Wh\x01\0\0\0\0\0\0\0\x16\x02`@\x1C[`\x10\x82\x16\x15a\x1F@Wh\x01\0\0\0\0\0\0\0\x0B\x02`@\x1C[`\x08\x82\x16\x15a\x1FXWh\x01\0\0\0\0\0\0\0\x06\x02`@\x1C[`\x04\x82\x16\x15a\x1FpWh\x01\0\0\0\0\0\0\0\x03\x02`@\x1C[`\x02\x82\x16\x15a\x1F\x88Wh\x01\0\0\0\0\0\0\0\x01\x02`@\x1C[`\x01\x82\x16\x15a\x1F\xA0Wh\x01\0\0\0\0\0\0\0\x01\x02`@\x1C[g\r\xE0\xB6\xB3\xA7d\0\0\x02`@\x91\x90\x91\x1C`\xBF\x03\x1C\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[\x80\x15\x15\x81\x14a \x15W`\0\x80\xFD[PV[`\0\x80`@\x83\x85\x03\x12\x15a .Wa .a\x1F\xB7V[\x825a 9\x81a \x07V[\x94` \x93\x90\x93\x015\x93PPPV[`\0` \x82\x84\x03\x12\x15a \\Wa \\a\x1F\xB7V[P5\x91\x90PV[`\0` \x82\x84\x03\x12\x15a xWa xa\x1F\xB7V[\x815`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x12}W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a \xA5Wa \xA5a\x1F\xB7V[PP\x805\x92` \x90\x91\x015\x91PV[`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x81R\x91\x90\x92\x16` \x82\x01R`@\x81\x01\x91\x90\x91R``\x01\x90V[`\0` \x82\x84\x03\x12\x15a \xEDWa \xEDa\x1F\xB7V[\x81Qa\x12}\x81a \x07V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x03\x80Wa\x03\x80a \xF8V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a!TWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x03\x80Wa\x03\x80a \xF8V[\x80\x82\x01\x80\x82\x11\x15a\x03\x80Wa\x03\x80a \xF8V\xFETarget contract does not contain\xA2dipfsX\"\x12 \xD0vq\x15\x12\xE8J\xB4\xB3~s\x99eE\x01G\xBA\xD2N\x80\xF7\x17\xD9ADV\xB7\xDC\xBE\xCFs dsolcC\0\x08\x13\x003";
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
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([192, 255, 26, 21], ())
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
    #[ethevent(name = "Swap", abi = "Swap(address,bool,uint256,uint256)")]
    pub struct SwapFilter {
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
        pub swap_direction: bool,
        pub input: ::ethers::core::types::U256,
        pub output: ::ethers::core::types::U256,
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
        AddLiquidity(AddLiquidityCall),
        Admin(AdminCall),
        BalanceOf(BalanceOfCall),
        GetInvariant(GetInvariantCall),
        GetSpotPrice(GetSpotPriceCall),
        InitPool(InitPoolCall),
        RemoveLiquidity(RemoveLiquidityCall),
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
            if let Ok(decoded) = <AddLiquidityCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AddLiquidity(decoded));
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
            if let Ok(decoded) = <GetSpotPriceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetSpotPrice(decoded));
            }
            if let Ok(decoded) = <InitPoolCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InitPool(decoded));
            }
            if let Ok(decoded) = <RemoveLiquidityCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RemoveLiquidity(decoded));
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
                Self::AddLiquidity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Admin(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BalanceOf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetInvariant(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetSpotPrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InitPool(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemoveLiquidity(element) => {
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
                Self::AddLiquidity(element) => ::core::fmt::Display::fmt(element, f),
                Self::Admin(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetInvariant(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSpotPrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::InitPool(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveLiquidity(element) => ::core::fmt::Display::fmt(element, f),
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
    impl ::core::convert::From<AddLiquidityCall> for G3MCalls {
        fn from(value: AddLiquidityCall) -> Self {
            Self::AddLiquidity(value)
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
    impl ::core::convert::From<GetSpotPriceCall> for G3MCalls {
        fn from(value: GetSpotPriceCall) -> Self {
            Self::GetSpotPrice(value)
        }
    }
    impl ::core::convert::From<InitPoolCall> for G3MCalls {
        fn from(value: InitPoolCall) -> Self {
            Self::InitPool(value)
        }
    }
    impl ::core::convert::From<RemoveLiquidityCall> for G3MCalls {
        fn from(value: RemoveLiquidityCall) -> Self {
            Self::RemoveLiquidity(value)
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
    pub struct GetInvariantReturn(pub ::ethers::core::types::U256);
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
