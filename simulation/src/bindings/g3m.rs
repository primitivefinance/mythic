pub use g3m::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types
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
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("_initPool"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amountX"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amountY"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("UD60x18"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("addLiquidity"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("addLiquidity"),
                            inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("liquidity"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("UD60x18"),
                                ),
                            },],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountX"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountY"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability:
                                ::ethers::core::abi::ethabi::StateMutability::NonPayable,
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountX"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountY"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("liquidity"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("UD60x18"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability:
                                ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("admin"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("admin"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("balanceOf"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("balanceOf"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("UD60x18"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getInvariant"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getInvariant"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("int256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getLiquidity"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getLiquidity"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getReserveX"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getReserveX"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getReserveY"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getReserveY"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getSpotPrice"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getSpotPrice"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getStrategyData"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getStrategyData"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getSwapFee"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getSwapFee"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("initExactX"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("initExactX"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amountX"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("price"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("initPool"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("initPool"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amountX"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amountY"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("UD60x18"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("instantiate"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("instantiate"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("initial_x"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("initial_price"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("liquidityWithoutPrecision"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("liquidityWithoutPrecision",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("logData"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("logData"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("removeLiquidity"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("removeLiquidity"),
                            inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("liquidity"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("UD60x18"),
                                ),
                            },],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountX"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountY"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability:
                                ::ethers::core::abi::ethabi::StateMutability::NonPayable,
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountX"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountY"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("liquidity"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("UD60x18"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability:
                                ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("reserveX"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("reserveX"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("UD60x18"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("reserveXWithoutPrecision"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("reserveXWithoutPrecision",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("reserveY"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("reserveY"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("UD60x18"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("reserveYWithoutPrecision"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("reserveYWithoutPrecision",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setSwapFee"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setSwapFee"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("newSwapFee"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setWeightX"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setWeightX"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("newTargetWeightX"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("UD60x18"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("newWeightXUpdateEnd",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("swapAmountIn"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
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
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("swapAmountOut"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
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
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("swapFee"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("swapFee"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("tokenX"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("tokenX"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("tokenY"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("tokenY"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("totalLiquidity"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("totalLiquidity"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("UD60x18"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("weightX"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("weightX"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("UD60x18"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("weightY"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("weightY"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("UD60x18"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("AddLiquidity"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("AddLiquidity"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("sender"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("liquidity"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("amountX"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("amountY"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("LogPrices"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("LogPrices"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("spotPrice"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("blockTimestamp"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("LogReserves"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("LogReserves"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("reserveX"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("reserveY"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("blockTimestamp"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("LogSyncingWeight"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("LogSyncingWeight"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("weightX"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("weightY"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("blockTimestamp"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("LogWeights"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("LogWeights"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("blockTimestamp"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("weightX"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RemoveLiquidity"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("RemoveLiquidity"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("sender"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("liquidity"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("amountX"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("amountY"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SetTargetWeightX"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("SetTargetWeightX"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("newTargetWeightX"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("newWeightXUpdateEnd",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("newWeightXUpdatePerSecond",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SetWeightX"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("SetWeightX"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("oldWeightX"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("newWeightX"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Swap"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
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
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("output"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("newPrice"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("InvalidSwap"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InvalidSwap"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("invariantBefore"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("UD60x18"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("invariantAfter"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("UD60x18"),
                                ),
                            },
                        ],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PRBMath_MulDiv18_Overflow"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("PRBMath_MulDiv18_Overflow",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("x"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("y"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PRBMath_MulDiv_Overflow"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("PRBMath_MulDiv_Overflow",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("x"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("y"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("denominator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PRBMath_UD60x18_Convert_Overflow"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("PRBMath_UD60x18_Convert_Overflow",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("x"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PRBMath_UD60x18_Exp2_InputTooBig"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("PRBMath_UD60x18_Exp2_InputTooBig",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("x"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("UD60x18"),
                            ),
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PRBMath_UD60x18_Log_InputTooSmall"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("PRBMath_UD60x18_Log_InputTooSmall",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("x"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("UD60x18"),
                            ),
                        },],
                    },],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static G3M_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15b\0\0^W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`@Qb\0-.8\x03\x80b\0-.\x839\x81\x01`@\x81\x90Rb\0\0\x81\x91b\0\x02GV[\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x03b\0\0\xD9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01RmInvalid tokens`\x90\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x80\x87\x16`\x01`\x01`\xA0\x1B\x03\x19\x92\x83\x16\x17\x90\x92U`\x02\x80T\x92\x86\x16\x92\x82\x16\x92\x90\x92\x17\x90\x91U`\0\x80T\x90\x91\x163\x17\x90Ub\0\x01&\x82f#\x86\xF2o\xC1\0\0\x11\x15\x90V[b\0\x01gW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01RoWeight X too low`\x80\x1B`D\x82\x01R`d\x01b\0\0\xD0V[b\0\x01z\x82g\r\xBD/\xC17\xA3\0\0\x10\x15\x90V[b\0\x01\xBCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01Rp\n\xEC\xAD,\xED\x0E\x84\x0B\x04\x0E\x8D\xED\xE4\r\r,\xED`{\x1B`D\x82\x01R`d\x01b\0\0\xD0V[`\n\x82\x90UB`\x0B\x81\x90U`\x08\x83\x90U`\tUg\r\xE0\xB6\xB3\xA7d\0\0\x81\x11\x15b\0\x02\x1DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01Rp\nn\xEC.\x04\x0C\xCC\xAC\xA4\x0E\x8D\xED\xE4\r\r,\xED`{\x1B`D\x82\x01R`d\x01b\0\0\xD0V[`\x03UPb\0\x02\xDA\x91PPV[\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x02BW`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15b\0\x02\xA9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[b\0\x02\xB4\x85b\0\x02*V[\x93Pb\0\x02\xC4` \x86\x01b\0\x02*V[`@\x86\x01Q``\x90\x96\x01Q\x94\x97\x90\x96P\x92PPPV[a*D\x80b\0\x02\xEA`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x02=W`\x005`\xE0\x1C\x80c\x8AYS\xC7\x11a\x01\\W\x80c\xD4\xCA\xDFh\x11a\0\xEFW\x80c\xF8Q\xA4@\x11a\0\xBEW\x80c\xF8Q\xA4@\x14a\x04\x8EW\x80c\xF9\xA1\xC8Z\x14a\x04\xA1W\x80c\xFA\xDF\xA6[\x14a\x04\xB4W\x80c\xFE\xD3\xDF\xDA\x14a\x04\xBDWa\x02=V[\x80c\xD4\xCA\xDFh\x14a\x04cW\x80c\xDBy\x10C\x14a\x04kW\x80c\xDCv\xFA\xBC\x14a\x04~W\x80c\xE3\x11\xCE\xC8\x14a\x04\x86Wa\x02=V[\x80c\xA0\xDBj\x82\x11a\x01+W\x80c\xA0\xDBj\x82\x14a\x04-W\x80c\xAD\xB5\x1D\xEE\x14a\x04@W\x80c\xB7\xD1\x9F\xC4\x14a\x04HW\x80c\xC0\xFF\x1A\x15\x14a\x04[Wa\x02=V[\x80c\x8AYS\xC7\x14a\x03\xECW\x80c\x9C\x8F\x9F#\x14a\x03\xF4W\x80c\x9C\xE32\xD4\x14a\x04\x07W\x80c\x9E\x1B\0E\x14a\x04\x1AWa\x02=V[\x80cQ\xC6Y\n\x11a\x01\xD4W\x80cu\xAE\xE1\xDA\x11a\x01\xA3W\x80cu\xAE\xE1\xDA\x14a\x02\xBEW\x80cvp\x166\x14a\x03\x96W\x80c\x84\x89PO\x14a\x03\xC4W\x80c\x87kU\xF1\x14a\x03\xD7Wa\x02=V[\x80cQ\xC6Y\n\x14a\x03=W\x80cT\xCF*\xEB\x14a\x03eW\x80cU\x9D\x16\x02\x14a\x03nW\x80cp\xA0\x821\x14a\x03vWa\x02=V[\x80c\x194\xEB%\x11a\x02\x10W\x80c\x194\xEB%\x14a\x02\xFAW\x80c\x1F\xDA\xBC'\x14a\x03\x02W\x80c4\xE1\x99\x07\x14a\x03\x15W\x80c69\xAA2\x14a\x03*Wa\x02=V[\x80c\x08\xEA\xBD\xDA\x14a\x02\xA2W\x80c\t\x10\xA5\x10\x14a\x02\xBEW\x80c\x15w\x0F\x92\x14a\x02\xC6W\x80c\x16\xDC\x16[\x14a\x02\xCFW[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x02\xAB`\x04T\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x02\xABa\x04\xC5V[a\x02\xAB`\x06T\x81V[`\x01Ta\x02\xE2\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\xB5V[a\x02\xABa\x04\xD7V[a\x02\xABa\x03\x106`\x04a(jV[a\x04\xE4V[a\x03(a\x03#6`\x04a(\x99V[a\x04\xFBV[\0[a\x03(a\x0386`\x04a(\xB5V[a\x05\x97V[a\x03Pa\x03K6`\x04a(\x99V[a\x05\xF9V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x02\xB5V[a\x02\xAB`\x03T\x81V[a\x02\xABa\x08\xA3V[a\x02\xABa\x03\x846`\x04a(\xDAV[`\x07` R`\0\x90\x81R`@\x90 T\x81V[a\x03\xA9a\x03\xA46`\x04a(jV[a\x08\xADV[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x02\xB5V[a\x02\xABa\x03\xD26`\x04a(\xB5V[a\x0B\xAAV[a\x03\xDFa\x0EUV[`@Qa\x02\xB5\x91\x90a)\x06V[a\x02\xABa\x0E\x8DV[a\x03Pa\x04\x026`\x04a(\x99V[a\x0E\xA8V[a\x02\xABa\x04\x156`\x04a(jV[a\x11QV[a\x02\xABa\x04(6`\x04a(\xB5V[a\x11_V[a\x03\xA9a\x04;6`\x04a(jV[a\x11kV[a\x02\xABa\x13lV[`\x02Ta\x02\xE2\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x02\xABa\x13\xD9V[`\x03Ta\x02\xABV[a\x03(a\x04y6`\x04a(\xB5V[a\x13\xF1V[a\x02\xABa\x15\xC3V[a\x02\xABa\x15\xE3V[`\0Ta\x02\xE2\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x03Pa\x04\xAF6`\x04a(\xB5V[a\x15\xF0V[a\x02\xAB`\x05T\x81V[a\x02\xABa\x16YV[`\0a\x04\xD2`\x06Ta\x16cV[\x90P\x90V[`\0a\x04\xD2`\x05Ta\x16cV[`\0a\x04\xF2\x83`\x01\x84a\x16wV[\x90P[\x92\x91PPV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x05FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh'7\xBA\x100\xB26\xB4\xB7`\xB9\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[g\r\xE0\xB6\xB3\xA7d\0\0\x81\x11\x15a\x05\x92W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01Rp\nn\xEC.\x04\x0C\xCC\xAC\xA4\x0E\x8D\xED\xE4\r\r,\xED`{\x1B`D\x82\x01R`d\x01a\x05=V[`\x03UV[`\x08T`\0a\x05\xAE\x82g\r\xE0\xB6\xB3\xA7d\0\0a)jV[\x90P`\0g\r\xE0\xB6\xB3\xA7d\0\0\x83\x86a\x05\xC7\x85\x88a)}V[a\x05\xD1\x91\x90a)}V[a\x05\xDB\x91\x90a)\xAAV[a\x05\xE5\x91\x90a)\xAAV[\x90Pa\x05\xF1\x85\x82a\x0B\xAAV[PPPPPPV[`\0\x80a\x06\x06`\x06T\x15\x90V[\x15a\x06JW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rs\x14\x1B\xDB\xDB\x08\x1B\x9B\xDD\x08\x1A[\x9A]\x1AX[\x1A^\x99Y`b\x1B`D\x82\x01R`d\x01a\x05=V[a\x06Y`\x06T\x84`\x04Ta\x1A\x8BV[\x91Pa\x06j`\x06T\x84`\x05Ta\x1A\x8BV[`\x01T`@Qc#\xB8r\xDD`\xE0\x1B\x81R\x91\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c#\xB8r\xDD\x90a\x06\x9F\x903\x900\x90\x87\x90`\x04\x01a)\xCCV[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x06\xF7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a*$\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x07\x0BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07/\x91\x90a)\xF0V[P`\x02T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\x07d\x903\x900\x90\x86\x90`\x04\x01a)\xCCV[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x07\xBCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a*$\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x07\xD0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\xF4\x91\x90a)\xF0V[P`@\x80Q\x84\x81R` \x81\x01\x84\x90R\x90\x81\x01\x82\x90R3\x90\x7F\xBE\xB3\x88W\x86\xD67\xA4t\xCB\xC2\x87\xC0\xA4E\x87#\x163\xA0w\xF0\xBD05MZK\x18\x99o\xCE\x90``\x01`@Q\x80\x91\x03\x90\xA2a\x08L`\x04Ta\x08G\x84a\x1A\xC5V[a\x1B\x0CV[`\x04U`\x05Ta\x08_\x90a\x08G\x83a\x1A\xC5V[`\x05U3`\0\x90\x81R`\x07` R`@\x90 Ta\x08|\x90\x84a\x1B\x0CV[3`\0\x90\x81R`\x07` R`@\x90 U`\x06Ta\x08\x99\x90\x84a\x1B\x0CV[`\x06U\x90\x92\x90\x91PV[`\0a\x04\xD2a\x15\xE3V[`\0\x80`\0a\x08\xBC`\x06T\x15\x90V[\x15a\t\0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rs\x14\x1B\xDB\xDB\x08\x1B\x9B\xDD\x08\x1A[\x9A]\x1AX[\x1A^\x99Y`b\x1B`D\x82\x01R`d\x01a\x05=V[\x84\x15a\t\x1FW\x83\x92Pa\t\x18`\x04T`\x05T\x86a\x1B\x1EV[\x91Pa\t4V[\x83\x91Pa\t1`\x04T`\x05T\x86a\x1BFV[\x92P[a\tC`\x04Ta\x08G\x85a\x1A\xC5V[`\x04U`\x05Ta\tV\x90a\x08G\x84a\x1A\xC5V[`\x05U`\x04T`\0\x90a\t{\x90a\tka\x13lV[`\x05Ta\tva\x0E\x8DV[a\x1BnV[\x90P`\0a\t\x92\x82a\t\x8D`\x02a\x1A\xC5V[a\x1B\x91V[\x90Pa\t\xA0\x81`\x06Ta\x1B\xA0V[`\x06\x82\x90U3`\0\x90\x81R`\x07` R`@\x90 T\x90\x93Pa\t\xC2\x90\x84a\x1B\x0CV[3`\0\x81\x81R`\x07` \x90\x81R`@\x91\x82\x90 \x93\x90\x93U\x80Q\x86\x81R\x92\x83\x01\x88\x90R\x82\x01\x86\x90R\x90\x7F\xBE\xB3\x88W\x86\xD67\xA4t\xCB\xC2\x87\xC0\xA4E\x87#\x163\xA0w\xF0\xBD05MZK\x18\x99o\xCE\x90``\x01`@Q\x80\x91\x03\x90\xA2`\x01T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\nK\x903\x900\x90\x8A\x90`\x04\x01a)\xCCV[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\n\xA3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a*$\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\n\xB7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\xDB\x91\x90a)\xF0V[P`\x02T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\x0B\x10\x903\x900\x90\x89\x90`\x04\x01a)\xCCV[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0BhW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a*$\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x0B|W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\xA0\x91\x90a)\xF0V[PPP\x92P\x92P\x92V[`\0a\x0B\xB6`\x06T\x15\x90V[a\x0C\x02W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FPool already initialized\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05=V[`\0a\x0C\r\x84a\x1A\xC5V[\x90P`\0a\x0C\x1A\x84a\x1A\xC5V[\x90P`\0a\x0CCa\x0C*\x87a\x1A\xC5V[a\x0C2a\x13lV[a\x0C;\x88a\x1A\xC5V[a\tva\x0E\x8DV[\x90P`\0a\x0CU\x82a\t\x8D`\x02a\x1A\xC5V[\x90Pa\x0Cc`\x06T\x82a\x1B\x0CV[`\x06Ua\x0Cr\x81a\x03\xE8a\x1B\xA0V[3`\0\x81\x81R`\x07` R`@\x80\x82 \x93\x90\x93U\x80Ra\x03\xE8\x7FmRW N\xBE}\x88\xFD\x91\xAE\x87\x94\x1C\xB2\xDD\x9D\x80b\xB6J\xE5\xA2\xBD-(\xEC@\xB9\xFB\xF6\xDFU`\x04\x86\x81U`\x05\x86\x90U`\x01T\x92Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92c#\xB8r\xDD\x92a\x0C\xE8\x92\x90\x910\x91\x8D\x91\x01a)\xCCV[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\r@W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a*$\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\rTW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\rx\x91\x90a)\xF0V[P`\x02T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\r\xAD\x903\x900\x90\x8B\x90`\x04\x01a)\xCCV[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0E\x05W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a*$\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x0E\x19W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E=\x91\x90a)\xF0V[Pa\x0EJ\x81a\x03\xE8a\x1B\xA0V[\x97\x96PPPPPPPV[``a\x0E_a\x13lV[a\x0Ega\x0E\x8DV[`@\x80Q` \x81\x01\x93\x90\x93R\x82\x01R``\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[`\0a\x04\xD2g\r\xE0\xB6\xB3\xA7d\0\0a\x0E\xA3a\x13lV[a\x1B\xA0V[3`\0\x90\x81R`\x07` R`@\x81 T\x81\x90\x83\x11\x15a\x0F\x02W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01RuInsufficient liquidity`P\x1B`D\x82\x01R`d\x01a\x05=V[a\x0F\x11`\x06T\x84`\x04Ta\x1B\xAFV[\x91Pa\x0F\"`\x06T\x84`\x05Ta\x1B\xAFV[`\x01T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x85\x90R\x91\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0F\xADW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a*$\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x0F\xC1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xE5\x91\x90a)\xF0V[P`\x02T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x83\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x10pW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a*$\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x10\x84W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\xA8\x91\x90a)\xF0V[P3`\0\x90\x81R`\x07` R`@\x90 Ta\x10\xC3\x90\x84a\x1B\xA0V[3`\0\x90\x81R`\x07` R`@\x90 U`\x06Ta\x10\xE0\x90\x84a\x1B\xA0V[`\x06U`\x04Ta\x10\xF3\x90a\x0E\xA3\x84a\x1A\xC5V[`\x04U`\x05Ta\x11\x06\x90a\x0E\xA3\x83a\x1A\xC5V[`\x05U`@\x80Q\x84\x81R` \x81\x01\x84\x90R\x90\x81\x01\x82\x90R3\x90\x7FY\xC3\xA0\xB6\x0Cj\xB7\xDE\xB6.\x14@\xC9\xE7$A\xDBm\xB7\xDF\xE5\x14\xDB\xA8\xCB\x18\xE6\x0C\r\x89n\xFA\x90``\x01`@Q\x80\x91\x03\x90\xA2\x91P\x91V[`\0a\x04\xF2\x83`\0\x84a\x16wV[`\0a\x04\xF2\x83\x83a\x0B\xAAV[`\0\x80`\0\x84\x15a\x11\x8FW\x83\x92Pa\x11\x88`\x04T`\x05T\x86a\x1B\x1EV[\x91Pa\x11\xA4V[\x83\x91Pa\x11\xA1`\x04T`\x05T\x86a\x1BFV[\x92P[a\x11\xB3`\x04Ta\x0E\xA3\x85a\x1A\xC5V[`\x04U`\x05Ta\x11\xC6\x90a\x0E\xA3\x84a\x1A\xC5V[`\x05U`\x04T`\0\x90a\x11\xDB\x90a\tka\x13lV[\x90P`\0a\x11\xED\x82a\t\x8D`\x02a\x1A\xC5V[\x90Pa\x11\xFB`\x06T\x82a\x1B\xA0V[`\x06\x82\x90U3`\0\x90\x81R`\x07` R`@\x90 T\x90\x93Pa\x12\x1D\x90\x84a\x1B\xA0V[3`\0\x81\x81R`\x07` \x90\x81R`@\x91\x82\x90 \x93\x90\x93U\x80Q\x86\x81R\x92\x83\x01\x88\x90R\x82\x01\x86\x90R\x90\x7FY\xC3\xA0\xB6\x0Cj\xB7\xDE\xB6.\x14@\xC9\xE7$A\xDBm\xB7\xDF\xE5\x14\xDB\xA8\xCB\x18\xE6\x0C\r\x89n\xFA\x90``\x01`@Q\x80\x91\x03\x90\xA2`\x01T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x87\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x12\xFCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a*$\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x13\x10W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x134\x91\x90a)\xF0V[P`\x02T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x86\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01a\x0B\x10V[`\0`\x0BTB\x10a\x13~WP`\nT\x90V[`\0`\tTBa\x13\x8E\x91\x90a)jV[\x90P`\0a\x13\xA6a\x13\x9E\x83a\x1A\xC5V[`\x0CTa\x1B\x91V[\x90Pa\x13\xB5`\x08T`\nT\x10\x90V[\x15a\x13\xCDWa\x13\xC6`\x08T\x82a\x1B\xA0V[\x92PPP\x90V[a\x13\xC6`\x08T\x82a\x1B\x0CV[`\0a\x04\xD2a\x13\xEC`\x04Ta\tka\x13lV[a\x16cV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x147W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh'7\xBA\x100\xB26\xB4\xB7`\xB9\x1B`D\x82\x01R`d\x01a\x05=V[a\x14H\x82f#\x86\xF2o\xC1\0\0\x11\x15\x90V[a\x14\x87W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01RoWeight X too low`\x80\x1B`D\x82\x01R`d\x01a\x05=V[a\x14\x99\x82g\r\xBD/\xC17\xA3\0\0\x10\x15\x90V[a\x14\xD9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01Rp\n\xEC\xAD,\xED\x0E\x84\x0B\x04\x0E\x8D\xED\xE4\r\r,\xED`{\x1B`D\x82\x01R`d\x01a\x05=V[B\x81\x11a\x15\x1CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01Rp\x15\\\x19\x18]\x19H\x19[\x99\x08\x1C\x18\\\xDD\x19Y`z\x1B`D\x82\x01R`d\x01a\x05=V[a\x15$a\x1B\xD6V[`\0a\x151`\x08T\x84\x10\x90V[a\x15FWa\x15A\x83`\x08Ta\x1B\xA0V[a\x15RV[a\x15R`\x08T\x84a\x1B\xA0V[\x90Pa\x15o\x81a\x15ja\x15eB\x86a)jV[a\x1A\xC5V[a\x1B\xE7V[`\x0CU`\n\x83\x90U`\x0B\x82\x90U\x7F\x91\t\xD5\xCC\xAE\x12\x89A\xC9q\x94\xA8t\xB03\xDE'l\x81#\xF7!\xAC\x0F\x18\x17M\x11r\xBEj\xC2Ba\x15\xA6a\x13lV[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01`@Q\x80\x91\x03\x90\xA1PPPV[`\0a\x04\xD2`\x04Ta\x15\xD3a\x13lV[`\x05Ta\x15\xDEa\x0E\x8DV[a\x1B\xFFV[`\0a\x04\xD2`\x04Ta\x16cV[`\x08T`\0\x90\x81\x90\x81a\x16\x0B\x82g\r\xE0\xB6\xB3\xA7d\0\0a)jV[\x90P`\0g\r\xE0\xB6\xB3\xA7d\0\0\x83\x88a\x16$\x85\x8Aa)}V[a\x16.\x91\x90a)}V[a\x168\x91\x90a)\xAAV[a\x16B\x91\x90a)\xAAV[\x90Pa\x16N\x87\x82a\x0B\xAAV[PPPP\x92P\x92\x90PV[`\0a\x04\xD2a\x04\xD7V[`\0a\x04\xF5g\r\xE0\xB6\xB3\xA7d\0\0\x83a)\xAAV[`\0\x80a\x16\x82a\x13lV[\x90P`\0a\x16\x8Ea\x0E\x8DV[\x90P`\0a\x16\xA2`\x04T\x84`\x05T\x85a\x1BnV[\x90P`\0\x80\x87\x15a\x170W\x86\x91P`\0g\r\xE0\xB6\xB3\xA7d\0\0`\x03T\x84a\x16\xC9\x91\x90a)}V[a\x16\xD3\x91\x90a)\xAAV[\x90P`\0a\x16\xE1\x82\x85a)jV[\x90Pa\x17'\x81\x8Ca\x16\xF4W`\x05Ta\x16\xF8V[`\x04T[\x8Da\x17\x03W\x88a\x17\x05V[\x89[\x8Ea\x17\x12W`\x04Ta\x17\x16V[`\x05T[\x8Fa\x17!W\x8Ba\x1C&V[\x8Aa\x1C&V[\x92PPPa\x17\xB0V[P\x85`\0a\x17x\x82\x8Ba\x17EW`\x05Ta\x17IV[`\x04T[\x8Ca\x17TW\x87a\x17VV[\x88[\x8Da\x17cW`\x04Ta\x17gV[`\x05T[\x8Ea\x17rW\x8Aa\x1CcV[\x89a\x1CcV[\x90P`\x03Tg\r\xE0\xB6\xB3\xA7d\0\0a\x17\x90\x91\x90a)jV[a\x17\xA2\x82g\r\xE0\xB6\xB3\xA7d\0\0a)}V[a\x17\xAC\x91\x90a)\xAAV[\x92PP[\x88\x15a\x17\xE0Wa\x17\xC5`\x04Ta\x08G\x84a\x1A\xC5V[`\x04U`\x05Ta\x17\xD8\x90a\x0E\xA3\x83a\x1A\xC5V[`\x05Ua\x18\x06V[a\x17\xEF`\x04Ta\x0E\xA3\x83a\x1A\xC5V[`\x04U`\x05Ta\x18\x02\x90a\x08G\x84a\x1A\xC5V[`\x05U[`\0a\x18\x18`\x04T\x87`\x05T\x88a\x1BnV[\x90P\x83\x81\x10\x15a\x18EW`@Qc,\x88t9`\xE2\x1B\x81R`\x04\x81\x01\x85\x90R`$\x81\x01\x82\x90R`D\x01a\x05=V[\x89a\x18[W`\x02T`\x01`\x01`\xA0\x1B\x03\x16a\x18hV[`\x01T`\x01`\x01`\xA0\x1B\x03\x16[`\x01`\x01`\xA0\x1B\x03\x16c#\xB8r\xDD30\x86`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x18\x97\x93\x92\x91\x90a)\xCCV[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x18\xEFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a*$\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x19\x03W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19'\x91\x90a)\xF0V[P\x89a\x19>W`\x01T`\x01`\x01`\xA0\x1B\x03\x16a\x19KV[`\x02T`\x01`\x01`\xA0\x1B\x03\x16[`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x19\xD3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a*$\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x19\xE7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A\x0B\x91\x90a)\xF0V[P`\0a\x1A\x1E`\x04T\x88`\x05T\x89a\x1B\xFFV[`@\x80Q\x8D\x15\x15\x81R` \x81\x01\x87\x90R\x90\x81\x01\x85\x90R``\x81\x01\x82\x90R\x90\x91P3\x90\x7F\x17fH\xF1\xF1\x1C\xDA(L\x12D\x90\x08k\xE4*\x92m\xDF\n\xE8\x87\xEB\xE7\xB1\xD6\xB37\xD8\x94'V\x90`\x80\x01`@Q\x80\x91\x03\x90\xA2\x89a\x1AxW\x83a\x1AzV[\x82[\x97PPPPPPPP[\x93\x92PPPV[`\0a\x1A\xBDa\x13\xECa\x1A\xB7a\x1A\xA9a\x1A\xA3\x88\x88a\x1B\x0CV[\x88a\x1B\xE7V[g\r\xE0\xB6\xB3\xA7d\0\0a\x1B\xA0V[\x84a\x1B\x91V[\x94\x93PPPPV[`\0a\x1A\xDBg\r\xE0\xB6\xB3\xA7d\0\0`\0\x19a)\xAAV[\x82\x11\x15a\x1A\xFEW`@Qc\x1C\xD9Q\xA7`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R`$\x01a\x05=V[Pg\r\xE0\xB6\xB3\xA7d\0\0\x02\x90V[`\0a\x04\xF2a\x1B\x1B\x83\x85a*\x10V[\x90V[`\0a\x1A\xBDa\x13\xECa\x1B@a\x1B3\x86\x88a\x1B\xE7V[a\t\x8D\x88a\x08G\x88a\x1A\xC5V[\x85a\x1B\xA0V[`\0a\x1A\xBDa\x13\xECa\x1Bha\x1B[\x87\x87a\x1B\xE7V[a\t\x8D\x87a\x08G\x88a\x1A\xC5V[\x86a\x1B\xA0V[`\0\x80a\x1B{\x86\x86a\x1C\x93V[\x90P`\0a\x1B\x89\x85\x85a\x1C\x93V[\x90Pa\x0EJ\x82\x82[`\0a\x04\xF2a\x1B\x1B\x84\x84a\x1D\xA0V[`\0a\x04\xF2a\x1B\x1B\x83\x85a)jV[`\0a\x1A\xBDa\x13\xECa\x1A\xB7g\r\xE0\xB6\xB3\xA7d\0\0a\x0E\xA3a\x1B\xD0\x89\x89a\x1B\xA0V[\x89a\x1B\xE7V[a\x1B\xDEa\x13lV[`\x08UB`\tUV[`\0a\x04\xF2a\x1B\x1B\x84g\r\xE0\xB6\xB3\xA7d\0\0\x85a\x1EVV[`\0\x80a\x1C\x0C\x84\x84a\x1B\xE7V[\x90P`\0a\x1C\x1A\x87\x87a\x1B\xE7V[\x90Pa\x0EJ\x82\x82a\x1B\xE7V[`\0\x80a\x1C:\x86a\x15j\x88a\x08G\x8Ba\x1A\xC5V[\x90Pa\x0EJa\x13\xEC\x85a\t\x8Dg\r\xE0\xB6\xB3\xA7d\0\0a\x0E\xA3a\x1C\\\x8B\x8Aa\x1B\xE7V[\x87\x90a\x1C\x93V[`\0\x80a\x1Cw\x84a\x15j\x86a\x0E\xA3\x8Ba\x1A\xC5V[\x90Pa\x0EJa\x13\xEC\x87a\t\x8Da\x1A\xA9a\x1C\x90\x88\x8Ba\x1B\xE7V[\x86\x90[`\0\x82\x82\x81\x83\x03a\x1C\xBEW\x80\x15a\x1C\xABW`\0a\x1C\xB5V[g\r\xE0\xB6\xB3\xA7d\0\0[\x92PPPa\x04\xF5V[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x03a\x1C\xDFWg\r\xE0\xB6\xB3\xA7d\0\0\x92PPPa\x04\xF5V[\x80`\0\x03a\x1C\xF9Wg\r\xE0\xB6\xB3\xA7d\0\0\x92PPPa\x04\xF5V[g\r\xE0\xB6\xB3\xA7d\0\0\x81\x03a\x1D\x12W\x84\x92PPPa\x04\xF5V[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x11\x15a\x1DCWa\x1D<a\x1D7a\x1D1\x87a\x1F*V[\x86a\x1B\x91V[a XV[\x92Pa\x1D\x98V[`\0a\x1Daa\x1B\x1B\x84n\xC0\x97\xCE{\xC9\x07\x15\xB3K\x9F\x10\0\0\0\0a)\xAAV[\x90P`\0a\x1Dza\x1D7a\x1Dt\x84a\x1F*V[\x88a\x1B\x91V[\x90Pa\x0EJa\x1B\x1B\x82n\xC0\x97\xCE{\xC9\x07\x15\xB3K\x9F\x10\0\0\0\0a)\xAAV[PP\x92\x91PPV[`\0\x80\x80`\0\x19\x84\x86\t\x84\x86\x02\x92P\x82\x81\x10\x83\x82\x03\x03\x91PP\x80`\0\x03a\x1D\xD4WPg\r\xE0\xB6\xB3\xA7d\0\0\x90\x04\x90Pa\x04\xF5V[g\r\xE0\xB6\xB3\xA7d\0\0\x81\x10a\x1E\x06W`@QcQsd\x8D`\xE0\x1B\x81R`\x04\x81\x01\x86\x90R`$\x81\x01\x85\x90R`D\x01a\x05=V[`\0g\r\xE0\xB6\xB3\xA7d\0\0\x85\x87\tb\x04\0\0\x81\x85\x03\x04\x93\x10\x90\x91\x03`\x01`\xEE\x1B\x02\x91\x90\x91\x17\x7F\xAC\xCB\x18\x16[\xD6\xFE1\xAE\x1C\xF3\x18\xDC[Q\xEE\xE0\xE1\xBAV\x9B\x88\xCDt\xC1w;\x91\xFA\xC1\x06i\x02\x91PP\x92\x91PPV[`\0\x80\x80`\0\x19\x85\x87\t\x85\x87\x02\x92P\x82\x81\x10\x83\x82\x03\x03\x91PP\x80`\0\x03a\x1E\x90W\x83\x82\x81a\x1E\x86Wa\x1E\x86a)\x94V[\x04\x92PPPa\x1A\x84V[\x83\x81\x10a\x1E\xC1W`@Qc\x0Ct\n\xEF`\xE3\x1B\x81R`\x04\x81\x01\x87\x90R`$\x81\x01\x86\x90R`D\x81\x01\x85\x90R`d\x01a\x05=V[`\0\x84\x86\x88\t`\x02`\x01\x87\x19\x81\x01\x88\x16\x97\x88\x90\x04`\x03\x81\x02\x83\x18\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x90\x81\x02\x90\x92\x03\x90\x91\x02`\0\x88\x90\x03\x88\x90\x04\x90\x91\x01\x85\x83\x11\x90\x94\x03\x93\x90\x93\x02\x93\x03\x94\x90\x94\x04\x91\x90\x91\x17\x02\x94\x93PPPPV[`\0\x81g\r\xE0\xB6\xB3\xA7d\0\0\x81\x10\x15a\x1FYW`@Qc\x03m2\xEF`\xE4\x1B\x81R`\x04\x81\x01\x84\x90R`$\x01a\x05=V[`\0a\x1F\xE5g\r\xE0\xB6\xB3\xA7d\0\0\x83\x04`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x91\x82\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11`\x06\x1B\x90\x81\x1Cc\xFF\xFF\xFF\xFF\x81\x11`\x05\x1B\x90\x81\x1Ca\xFF\xFF\x81\x11`\x04\x1B\x90\x81\x1C`\xFF\x81\x11`\x03\x90\x81\x1B\x91\x82\x1C`\x0F\x81\x11`\x02\x1B\x90\x81\x1C\x91\x82\x11\x87\x1B\x91\x82\x1C\x96\x90\x96\x11\x94\x90\x96\x17\x92\x90\x92\x17\x17\x17\x91\x90\x91\x17\x91\x90\x91\x17\x17\x90V[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0\x81\x02\x82\x82\x1Cg\r\xE0\xB6\xB3\xA7c\xFF\xFF\x19\x81\x01a \x0EWP\x94\x93PPPPV[g\x1B\xC1mgN\xC8\0\0g\x06\xF0[Y\xD3\xB2\0\0[\x80\x15a QWg\r\xE0\xB6\xB3\xA7d\0\0\x83\x80\x02\x04\x92P\x81\x83\x10a IW\x92\x83\x01\x92`\x01\x92\x90\x92\x1C\x91[`\x01\x1Ca !V[P\x82a\x0EJV[`\0\x81h\nh\x89\x06\xBD\x8A\xFF\xFF\xFF\x81\x11\x15a \x88W`@Qc\xB3\xB6\xBA\x1F`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x01a\x05=V[`\0a \xA0g\r\xE0\xB6\xB3\xA7d\0\0`@\x84\x90\x1Ba)\xAAV[\x90Pa\x1A\xBDa\x1B\x1B\x82`\x01`\xBF\x1Bg\xFF\0\0\0\0\0\0\0\x82\x16\x15a!\xB6Wg\x80\0\0\0\0\0\0\0\x82\x16\x15a \xDDWh\x01j\t\xE6g\xF3\xBC\xC9\t\x02`@\x1C[g@\0\0\0\0\0\0\0\x82\x16\x15a \xFCWh\x010o\xE0\xA3\x1BqR\xDF\x02`@\x1C[g \0\0\0\0\0\0\0\x82\x16\x15a!\x1BWh\x01\x17+\x83\xC7\xD5\x17\xAD\xCE\x02`@\x1C[g\x10\0\0\0\0\0\0\0\x82\x16\x15a!:Wh\x01\x0BU\x86\xCF\x98\x90\xF6*\x02`@\x1C[g\x08\0\0\0\0\0\0\0\x82\x16\x15a!YWh\x01\x05\x9B\r1XWC\xAE\x02`@\x1C[g\x04\0\0\0\0\0\0\0\x82\x16\x15a!xWh\x01\x02\xC9\xA3\xE7x\x06\x0E\xE7\x02`@\x1C[g\x02\0\0\0\0\0\0\0\x82\x16\x15a!\x97Wh\x01\x01c\xDA\x9F\xB33V\xD8\x02`@\x1C[g\x01\0\0\0\0\0\0\0\x82\x16\x15a!\xB6Wh\x01\0\xB1\xAF\xA5\xAB\xCB\xEDa\x02`@\x1C[f\xFF\0\0\0\0\0\0\x82\x16\x15a\"\xB5Wf\x80\0\0\0\0\0\0\x82\x16\x15a!\xE3Wh\x01\0X\xC8m\xA1\xC0\x9E\xA2\x02`@\x1C[f@\0\0\0\0\0\0\x82\x16\x15a\"\x01Wh\x01\0,`^.\x8C\xECP\x02`@\x1C[f \0\0\0\0\0\0\x82\x16\x15a\"\x1FWh\x01\0\x16/9\x04\x05\x1F\xA1\x02`@\x1C[f\x10\0\0\0\0\0\0\x82\x16\x15a\"=Wh\x01\0\x0B\x17^\xFF\xDCv\xBA\x02`@\x1C[f\x08\0\0\0\0\0\0\x82\x16\x15a\"[Wh\x01\0\x05\x8B\xA0\x1F\xB9\xF9m\x02`@\x1C[f\x04\0\0\0\0\0\0\x82\x16\x15a\"yWh\x01\0\x02\xC5\xCC7\xDA\x94\x92\x02`@\x1C[f\x02\0\0\0\0\0\0\x82\x16\x15a\"\x97Wh\x01\0\x01b\xE5%\xEE\x05G\x02`@\x1C[f\x01\0\0\0\0\0\0\x82\x16\x15a\"\xB5Wh\x01\0\0\xB1rUw\\\x04\x02`@\x1C[e\xFF\0\0\0\0\0\x82\x16\x15a#\xABWe\x80\0\0\0\0\0\x82\x16\x15a\"\xE0Wh\x01\0\0X\xB9\x1B[\xC9\xAE\x02`@\x1C[e@\0\0\0\0\0\x82\x16\x15a\"\xFDWh\x01\0\0,\\\x89\xD5\xECm\x02`@\x1C[e \0\0\0\0\0\x82\x16\x15a#\x1AWh\x01\0\0\x16.C\xF4\xF81\x02`@\x1C[e\x10\0\0\0\0\0\x82\x16\x15a#7Wh\x01\0\0\x0B\x17!\xBC\xFC\x9A\x02`@\x1C[e\x08\0\0\0\0\0\x82\x16\x15a#TWh\x01\0\0\x05\x8B\x90\xCF\x1En\x02`@\x1C[e\x04\0\0\0\0\0\x82\x16\x15a#qWh\x01\0\0\x02\xC5\xC8c\xB7?\x02`@\x1C[e\x02\0\0\0\0\0\x82\x16\x15a#\x8EWh\x01\0\0\x01b\xE40\xE5\xA2\x02`@\x1C[e\x01\0\0\0\0\0\x82\x16\x15a#\xABWh\x01\0\0\0\xB1r\x185Q\x02`@\x1C[d\xFF\0\0\0\0\x82\x16\x15a$\x98Wd\x80\0\0\0\0\x82\x16\x15a#\xD4Wh\x01\0\0\0X\xB9\x0C\x0BI\x02`@\x1C[d@\0\0\0\0\x82\x16\x15a#\xF0Wh\x01\0\0\0,\\\x86\x01\xCC\x02`@\x1C[d \0\0\0\0\x82\x16\x15a$\x0CWh\x01\0\0\0\x16.B\xFF\xF0\x02`@\x1C[d\x10\0\0\0\0\x82\x16\x15a$(Wh\x01\0\0\0\x0B\x17!\x7F\xBB\x02`@\x1C[d\x08\0\0\0\0\x82\x16\x15a$DWh\x01\0\0\0\x05\x8B\x90\xBF\xCE\x02`@\x1C[d\x04\0\0\0\0\x82\x16\x15a$`Wh\x01\0\0\0\x02\xC5\xC8_\xE3\x02`@\x1C[d\x02\0\0\0\0\x82\x16\x15a$|Wh\x01\0\0\0\x01b\xE4/\xF1\x02`@\x1C[d\x01\0\0\0\0\x82\x16\x15a$\x98Wh\x01\0\0\0\0\xB1r\x17\xF8\x02`@\x1C[c\xFF\0\0\0\x82\x16\x15a%|Wc\x80\0\0\0\x82\x16\x15a$\xBFWh\x01\0\0\0\0X\xB9\x0B\xFC\x02`@\x1C[c@\0\0\0\x82\x16\x15a$\xDAWh\x01\0\0\0\0,\\\x85\xFE\x02`@\x1C[c \0\0\0\x82\x16\x15a$\xF5Wh\x01\0\0\0\0\x16.B\xFF\x02`@\x1C[c\x10\0\0\0\x82\x16\x15a%\x10Wh\x01\0\0\0\0\x0B\x17!\x7F\x02`@\x1C[c\x08\0\0\0\x82\x16\x15a%+Wh\x01\0\0\0\0\x05\x8B\x90\xC0\x02`@\x1C[c\x04\0\0\0\x82\x16\x15a%FWh\x01\0\0\0\0\x02\xC5\xC8`\x02`@\x1C[c\x02\0\0\0\x82\x16\x15a%aWh\x01\0\0\0\0\x01b\xE40\x02`@\x1C[c\x01\0\0\0\x82\x16\x15a%|Wh\x01\0\0\0\0\0\xB1r\x18\x02`@\x1C[b\xFF\0\0\x82\x16\x15a&WWb\x80\0\0\x82\x16\x15a%\xA1Wh\x01\0\0\0\0\0X\xB9\x0C\x02`@\x1C[b@\0\0\x82\x16\x15a%\xBBWh\x01\0\0\0\0\0,\\\x86\x02`@\x1C[b \0\0\x82\x16\x15a%\xD5Wh\x01\0\0\0\0\0\x16.C\x02`@\x1C[b\x10\0\0\x82\x16\x15a%\xEFWh\x01\0\0\0\0\0\x0B\x17!\x02`@\x1C[b\x08\0\0\x82\x16\x15a&\tWh\x01\0\0\0\0\0\x05\x8B\x91\x02`@\x1C[b\x04\0\0\x82\x16\x15a&#Wh\x01\0\0\0\0\0\x02\xC5\xC8\x02`@\x1C[b\x02\0\0\x82\x16\x15a&=Wh\x01\0\0\0\0\0\x01b\xE4\x02`@\x1C[b\x01\0\0\x82\x16\x15a&WWh\x01\0\0\0\0\0\0\xB1r\x02`@\x1C[a\xFF\0\x82\x16\x15a')Wa\x80\0\x82\x16\x15a&zWh\x01\0\0\0\0\0\0X\xB9\x02`@\x1C[a@\0\x82\x16\x15a&\x93Wh\x01\0\0\0\0\0\0,]\x02`@\x1C[a \0\x82\x16\x15a&\xACWh\x01\0\0\0\0\0\0\x16.\x02`@\x1C[a\x10\0\x82\x16\x15a&\xC5Wh\x01\0\0\0\0\0\0\x0B\x17\x02`@\x1C[a\x08\0\x82\x16\x15a&\xDEWh\x01\0\0\0\0\0\0\x05\x8C\x02`@\x1C[a\x04\0\x82\x16\x15a&\xF7Wh\x01\0\0\0\0\0\0\x02\xC6\x02`@\x1C[a\x02\0\x82\x16\x15a'\x10Wh\x01\0\0\0\0\0\0\x01c\x02`@\x1C[a\x01\0\x82\x16\x15a')Wh\x01\0\0\0\0\0\0\0\xB1\x02`@\x1C[`\xFF\x82\x16\x15a'\xF2W`\x80\x82\x16\x15a'JWh\x01\0\0\0\0\0\0\0Y\x02`@\x1C[`@\x82\x16\x15a'bWh\x01\0\0\0\0\0\0\0,\x02`@\x1C[` \x82\x16\x15a'zWh\x01\0\0\0\0\0\0\0\x16\x02`@\x1C[`\x10\x82\x16\x15a'\x92Wh\x01\0\0\0\0\0\0\0\x0B\x02`@\x1C[`\x08\x82\x16\x15a'\xAAWh\x01\0\0\0\0\0\0\0\x06\x02`@\x1C[`\x04\x82\x16\x15a'\xC2Wh\x01\0\0\0\0\0\0\0\x03\x02`@\x1C[`\x02\x82\x16\x15a'\xDAWh\x01\0\0\0\0\0\0\0\x01\x02`@\x1C[`\x01\x82\x16\x15a'\xF2Wh\x01\0\0\0\0\0\0\0\x01\x02`@\x1C[g\r\xE0\xB6\xB3\xA7d\0\0\x02`@\x91\x90\x91\x1C`\xBF\x03\x1C\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[\x80\x15\x15\x81\x14a(gW`\0\x80\xFD[PV[`\0\x80`@\x83\x85\x03\x12\x15a(\x80Wa(\x80a(\tV[\x825a(\x8B\x81a(YV[\x94` \x93\x90\x93\x015\x93PPPV[`\0` \x82\x84\x03\x12\x15a(\xAEWa(\xAEa(\tV[P5\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a(\xCBWa(\xCBa(\tV[PP\x805\x92` \x90\x91\x015\x91PV[`\0` \x82\x84\x03\x12\x15a(\xEFWa(\xEFa(\tV[\x815`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x1A\x84W`\0\x80\xFD[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a)3W\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a)\x17V[P`\0`@\x82\x86\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x92PPP\x92\x91PPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x04\xF5Wa\x04\xF5a)TV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x04\xF5Wa\x04\xF5a)TV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a)\xC7WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x81R\x91\x90\x92\x16` \x82\x01R`@\x81\x01\x91\x90\x91R``\x01\x90V[`\0` \x82\x84\x03\x12\x15a*\x05Wa*\x05a(\tV[\x81Qa\x1A\x84\x81a(YV[\x80\x82\x01\x80\x82\x11\x15a\x04\xF5Wa\x04\xF5a)TV\xFETarget contract does not contain";
    /// The bytecode of the contract.
    pub static G3M_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x02=W`\x005`\xE0\x1C\x80c\x8AYS\xC7\x11a\x01\\W\x80c\xD4\xCA\xDFh\x11a\0\xEFW\x80c\xF8Q\xA4@\x11a\0\xBEW\x80c\xF8Q\xA4@\x14a\x04\x8EW\x80c\xF9\xA1\xC8Z\x14a\x04\xA1W\x80c\xFA\xDF\xA6[\x14a\x04\xB4W\x80c\xFE\xD3\xDF\xDA\x14a\x04\xBDWa\x02=V[\x80c\xD4\xCA\xDFh\x14a\x04cW\x80c\xDBy\x10C\x14a\x04kW\x80c\xDCv\xFA\xBC\x14a\x04~W\x80c\xE3\x11\xCE\xC8\x14a\x04\x86Wa\x02=V[\x80c\xA0\xDBj\x82\x11a\x01+W\x80c\xA0\xDBj\x82\x14a\x04-W\x80c\xAD\xB5\x1D\xEE\x14a\x04@W\x80c\xB7\xD1\x9F\xC4\x14a\x04HW\x80c\xC0\xFF\x1A\x15\x14a\x04[Wa\x02=V[\x80c\x8AYS\xC7\x14a\x03\xECW\x80c\x9C\x8F\x9F#\x14a\x03\xF4W\x80c\x9C\xE32\xD4\x14a\x04\x07W\x80c\x9E\x1B\0E\x14a\x04\x1AWa\x02=V[\x80cQ\xC6Y\n\x11a\x01\xD4W\x80cu\xAE\xE1\xDA\x11a\x01\xA3W\x80cu\xAE\xE1\xDA\x14a\x02\xBEW\x80cvp\x166\x14a\x03\x96W\x80c\x84\x89PO\x14a\x03\xC4W\x80c\x87kU\xF1\x14a\x03\xD7Wa\x02=V[\x80cQ\xC6Y\n\x14a\x03=W\x80cT\xCF*\xEB\x14a\x03eW\x80cU\x9D\x16\x02\x14a\x03nW\x80cp\xA0\x821\x14a\x03vWa\x02=V[\x80c\x194\xEB%\x11a\x02\x10W\x80c\x194\xEB%\x14a\x02\xFAW\x80c\x1F\xDA\xBC'\x14a\x03\x02W\x80c4\xE1\x99\x07\x14a\x03\x15W\x80c69\xAA2\x14a\x03*Wa\x02=V[\x80c\x08\xEA\xBD\xDA\x14a\x02\xA2W\x80c\t\x10\xA5\x10\x14a\x02\xBEW\x80c\x15w\x0F\x92\x14a\x02\xC6W\x80c\x16\xDC\x16[\x14a\x02\xCFW[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x02\xAB`\x04T\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x02\xABa\x04\xC5V[a\x02\xAB`\x06T\x81V[`\x01Ta\x02\xE2\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\xB5V[a\x02\xABa\x04\xD7V[a\x02\xABa\x03\x106`\x04a(jV[a\x04\xE4V[a\x03(a\x03#6`\x04a(\x99V[a\x04\xFBV[\0[a\x03(a\x0386`\x04a(\xB5V[a\x05\x97V[a\x03Pa\x03K6`\x04a(\x99V[a\x05\xF9V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x02\xB5V[a\x02\xAB`\x03T\x81V[a\x02\xABa\x08\xA3V[a\x02\xABa\x03\x846`\x04a(\xDAV[`\x07` R`\0\x90\x81R`@\x90 T\x81V[a\x03\xA9a\x03\xA46`\x04a(jV[a\x08\xADV[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x02\xB5V[a\x02\xABa\x03\xD26`\x04a(\xB5V[a\x0B\xAAV[a\x03\xDFa\x0EUV[`@Qa\x02\xB5\x91\x90a)\x06V[a\x02\xABa\x0E\x8DV[a\x03Pa\x04\x026`\x04a(\x99V[a\x0E\xA8V[a\x02\xABa\x04\x156`\x04a(jV[a\x11QV[a\x02\xABa\x04(6`\x04a(\xB5V[a\x11_V[a\x03\xA9a\x04;6`\x04a(jV[a\x11kV[a\x02\xABa\x13lV[`\x02Ta\x02\xE2\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x02\xABa\x13\xD9V[`\x03Ta\x02\xABV[a\x03(a\x04y6`\x04a(\xB5V[a\x13\xF1V[a\x02\xABa\x15\xC3V[a\x02\xABa\x15\xE3V[`\0Ta\x02\xE2\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x03Pa\x04\xAF6`\x04a(\xB5V[a\x15\xF0V[a\x02\xAB`\x05T\x81V[a\x02\xABa\x16YV[`\0a\x04\xD2`\x06Ta\x16cV[\x90P\x90V[`\0a\x04\xD2`\x05Ta\x16cV[`\0a\x04\xF2\x83`\x01\x84a\x16wV[\x90P[\x92\x91PPV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x05FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh'7\xBA\x100\xB26\xB4\xB7`\xB9\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[g\r\xE0\xB6\xB3\xA7d\0\0\x81\x11\x15a\x05\x92W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01Rp\nn\xEC.\x04\x0C\xCC\xAC\xA4\x0E\x8D\xED\xE4\r\r,\xED`{\x1B`D\x82\x01R`d\x01a\x05=V[`\x03UV[`\x08T`\0a\x05\xAE\x82g\r\xE0\xB6\xB3\xA7d\0\0a)jV[\x90P`\0g\r\xE0\xB6\xB3\xA7d\0\0\x83\x86a\x05\xC7\x85\x88a)}V[a\x05\xD1\x91\x90a)}V[a\x05\xDB\x91\x90a)\xAAV[a\x05\xE5\x91\x90a)\xAAV[\x90Pa\x05\xF1\x85\x82a\x0B\xAAV[PPPPPPV[`\0\x80a\x06\x06`\x06T\x15\x90V[\x15a\x06JW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rs\x14\x1B\xDB\xDB\x08\x1B\x9B\xDD\x08\x1A[\x9A]\x1AX[\x1A^\x99Y`b\x1B`D\x82\x01R`d\x01a\x05=V[a\x06Y`\x06T\x84`\x04Ta\x1A\x8BV[\x91Pa\x06j`\x06T\x84`\x05Ta\x1A\x8BV[`\x01T`@Qc#\xB8r\xDD`\xE0\x1B\x81R\x91\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c#\xB8r\xDD\x90a\x06\x9F\x903\x900\x90\x87\x90`\x04\x01a)\xCCV[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x06\xF7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a*$\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x07\x0BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07/\x91\x90a)\xF0V[P`\x02T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\x07d\x903\x900\x90\x86\x90`\x04\x01a)\xCCV[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x07\xBCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a*$\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x07\xD0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\xF4\x91\x90a)\xF0V[P`@\x80Q\x84\x81R` \x81\x01\x84\x90R\x90\x81\x01\x82\x90R3\x90\x7F\xBE\xB3\x88W\x86\xD67\xA4t\xCB\xC2\x87\xC0\xA4E\x87#\x163\xA0w\xF0\xBD05MZK\x18\x99o\xCE\x90``\x01`@Q\x80\x91\x03\x90\xA2a\x08L`\x04Ta\x08G\x84a\x1A\xC5V[a\x1B\x0CV[`\x04U`\x05Ta\x08_\x90a\x08G\x83a\x1A\xC5V[`\x05U3`\0\x90\x81R`\x07` R`@\x90 Ta\x08|\x90\x84a\x1B\x0CV[3`\0\x90\x81R`\x07` R`@\x90 U`\x06Ta\x08\x99\x90\x84a\x1B\x0CV[`\x06U\x90\x92\x90\x91PV[`\0a\x04\xD2a\x15\xE3V[`\0\x80`\0a\x08\xBC`\x06T\x15\x90V[\x15a\t\0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rs\x14\x1B\xDB\xDB\x08\x1B\x9B\xDD\x08\x1A[\x9A]\x1AX[\x1A^\x99Y`b\x1B`D\x82\x01R`d\x01a\x05=V[\x84\x15a\t\x1FW\x83\x92Pa\t\x18`\x04T`\x05T\x86a\x1B\x1EV[\x91Pa\t4V[\x83\x91Pa\t1`\x04T`\x05T\x86a\x1BFV[\x92P[a\tC`\x04Ta\x08G\x85a\x1A\xC5V[`\x04U`\x05Ta\tV\x90a\x08G\x84a\x1A\xC5V[`\x05U`\x04T`\0\x90a\t{\x90a\tka\x13lV[`\x05Ta\tva\x0E\x8DV[a\x1BnV[\x90P`\0a\t\x92\x82a\t\x8D`\x02a\x1A\xC5V[a\x1B\x91V[\x90Pa\t\xA0\x81`\x06Ta\x1B\xA0V[`\x06\x82\x90U3`\0\x90\x81R`\x07` R`@\x90 T\x90\x93Pa\t\xC2\x90\x84a\x1B\x0CV[3`\0\x81\x81R`\x07` \x90\x81R`@\x91\x82\x90 \x93\x90\x93U\x80Q\x86\x81R\x92\x83\x01\x88\x90R\x82\x01\x86\x90R\x90\x7F\xBE\xB3\x88W\x86\xD67\xA4t\xCB\xC2\x87\xC0\xA4E\x87#\x163\xA0w\xF0\xBD05MZK\x18\x99o\xCE\x90``\x01`@Q\x80\x91\x03\x90\xA2`\x01T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\nK\x903\x900\x90\x8A\x90`\x04\x01a)\xCCV[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\n\xA3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a*$\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\n\xB7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\xDB\x91\x90a)\xF0V[P`\x02T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\x0B\x10\x903\x900\x90\x89\x90`\x04\x01a)\xCCV[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0BhW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a*$\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x0B|W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\xA0\x91\x90a)\xF0V[PPP\x92P\x92P\x92V[`\0a\x0B\xB6`\x06T\x15\x90V[a\x0C\x02W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FPool already initialized\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05=V[`\0a\x0C\r\x84a\x1A\xC5V[\x90P`\0a\x0C\x1A\x84a\x1A\xC5V[\x90P`\0a\x0CCa\x0C*\x87a\x1A\xC5V[a\x0C2a\x13lV[a\x0C;\x88a\x1A\xC5V[a\tva\x0E\x8DV[\x90P`\0a\x0CU\x82a\t\x8D`\x02a\x1A\xC5V[\x90Pa\x0Cc`\x06T\x82a\x1B\x0CV[`\x06Ua\x0Cr\x81a\x03\xE8a\x1B\xA0V[3`\0\x81\x81R`\x07` R`@\x80\x82 \x93\x90\x93U\x80Ra\x03\xE8\x7FmRW N\xBE}\x88\xFD\x91\xAE\x87\x94\x1C\xB2\xDD\x9D\x80b\xB6J\xE5\xA2\xBD-(\xEC@\xB9\xFB\xF6\xDFU`\x04\x86\x81U`\x05\x86\x90U`\x01T\x92Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92c#\xB8r\xDD\x92a\x0C\xE8\x92\x90\x910\x91\x8D\x91\x01a)\xCCV[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\r@W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a*$\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\rTW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\rx\x91\x90a)\xF0V[P`\x02T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\r\xAD\x903\x900\x90\x8B\x90`\x04\x01a)\xCCV[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0E\x05W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a*$\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x0E\x19W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E=\x91\x90a)\xF0V[Pa\x0EJ\x81a\x03\xE8a\x1B\xA0V[\x97\x96PPPPPPPV[``a\x0E_a\x13lV[a\x0Ega\x0E\x8DV[`@\x80Q` \x81\x01\x93\x90\x93R\x82\x01R``\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[`\0a\x04\xD2g\r\xE0\xB6\xB3\xA7d\0\0a\x0E\xA3a\x13lV[a\x1B\xA0V[3`\0\x90\x81R`\x07` R`@\x81 T\x81\x90\x83\x11\x15a\x0F\x02W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01RuInsufficient liquidity`P\x1B`D\x82\x01R`d\x01a\x05=V[a\x0F\x11`\x06T\x84`\x04Ta\x1B\xAFV[\x91Pa\x0F\"`\x06T\x84`\x05Ta\x1B\xAFV[`\x01T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x85\x90R\x91\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0F\xADW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a*$\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x0F\xC1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xE5\x91\x90a)\xF0V[P`\x02T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x83\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x10pW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a*$\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x10\x84W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\xA8\x91\x90a)\xF0V[P3`\0\x90\x81R`\x07` R`@\x90 Ta\x10\xC3\x90\x84a\x1B\xA0V[3`\0\x90\x81R`\x07` R`@\x90 U`\x06Ta\x10\xE0\x90\x84a\x1B\xA0V[`\x06U`\x04Ta\x10\xF3\x90a\x0E\xA3\x84a\x1A\xC5V[`\x04U`\x05Ta\x11\x06\x90a\x0E\xA3\x83a\x1A\xC5V[`\x05U`@\x80Q\x84\x81R` \x81\x01\x84\x90R\x90\x81\x01\x82\x90R3\x90\x7FY\xC3\xA0\xB6\x0Cj\xB7\xDE\xB6.\x14@\xC9\xE7$A\xDBm\xB7\xDF\xE5\x14\xDB\xA8\xCB\x18\xE6\x0C\r\x89n\xFA\x90``\x01`@Q\x80\x91\x03\x90\xA2\x91P\x91V[`\0a\x04\xF2\x83`\0\x84a\x16wV[`\0a\x04\xF2\x83\x83a\x0B\xAAV[`\0\x80`\0\x84\x15a\x11\x8FW\x83\x92Pa\x11\x88`\x04T`\x05T\x86a\x1B\x1EV[\x91Pa\x11\xA4V[\x83\x91Pa\x11\xA1`\x04T`\x05T\x86a\x1BFV[\x92P[a\x11\xB3`\x04Ta\x0E\xA3\x85a\x1A\xC5V[`\x04U`\x05Ta\x11\xC6\x90a\x0E\xA3\x84a\x1A\xC5V[`\x05U`\x04T`\0\x90a\x11\xDB\x90a\tka\x13lV[\x90P`\0a\x11\xED\x82a\t\x8D`\x02a\x1A\xC5V[\x90Pa\x11\xFB`\x06T\x82a\x1B\xA0V[`\x06\x82\x90U3`\0\x90\x81R`\x07` R`@\x90 T\x90\x93Pa\x12\x1D\x90\x84a\x1B\xA0V[3`\0\x81\x81R`\x07` \x90\x81R`@\x91\x82\x90 \x93\x90\x93U\x80Q\x86\x81R\x92\x83\x01\x88\x90R\x82\x01\x86\x90R\x90\x7FY\xC3\xA0\xB6\x0Cj\xB7\xDE\xB6.\x14@\xC9\xE7$A\xDBm\xB7\xDF\xE5\x14\xDB\xA8\xCB\x18\xE6\x0C\r\x89n\xFA\x90``\x01`@Q\x80\x91\x03\x90\xA2`\x01T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x87\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x12\xFCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a*$\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x13\x10W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x134\x91\x90a)\xF0V[P`\x02T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x86\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01a\x0B\x10V[`\0`\x0BTB\x10a\x13~WP`\nT\x90V[`\0`\tTBa\x13\x8E\x91\x90a)jV[\x90P`\0a\x13\xA6a\x13\x9E\x83a\x1A\xC5V[`\x0CTa\x1B\x91V[\x90Pa\x13\xB5`\x08T`\nT\x10\x90V[\x15a\x13\xCDWa\x13\xC6`\x08T\x82a\x1B\xA0V[\x92PPP\x90V[a\x13\xC6`\x08T\x82a\x1B\x0CV[`\0a\x04\xD2a\x13\xEC`\x04Ta\tka\x13lV[a\x16cV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x147W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh'7\xBA\x100\xB26\xB4\xB7`\xB9\x1B`D\x82\x01R`d\x01a\x05=V[a\x14H\x82f#\x86\xF2o\xC1\0\0\x11\x15\x90V[a\x14\x87W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01RoWeight X too low`\x80\x1B`D\x82\x01R`d\x01a\x05=V[a\x14\x99\x82g\r\xBD/\xC17\xA3\0\0\x10\x15\x90V[a\x14\xD9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01Rp\n\xEC\xAD,\xED\x0E\x84\x0B\x04\x0E\x8D\xED\xE4\r\r,\xED`{\x1B`D\x82\x01R`d\x01a\x05=V[B\x81\x11a\x15\x1CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01Rp\x15\\\x19\x18]\x19H\x19[\x99\x08\x1C\x18\\\xDD\x19Y`z\x1B`D\x82\x01R`d\x01a\x05=V[a\x15$a\x1B\xD6V[`\0a\x151`\x08T\x84\x10\x90V[a\x15FWa\x15A\x83`\x08Ta\x1B\xA0V[a\x15RV[a\x15R`\x08T\x84a\x1B\xA0V[\x90Pa\x15o\x81a\x15ja\x15eB\x86a)jV[a\x1A\xC5V[a\x1B\xE7V[`\x0CU`\n\x83\x90U`\x0B\x82\x90U\x7F\x91\t\xD5\xCC\xAE\x12\x89A\xC9q\x94\xA8t\xB03\xDE'l\x81#\xF7!\xAC\x0F\x18\x17M\x11r\xBEj\xC2Ba\x15\xA6a\x13lV[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01`@Q\x80\x91\x03\x90\xA1PPPV[`\0a\x04\xD2`\x04Ta\x15\xD3a\x13lV[`\x05Ta\x15\xDEa\x0E\x8DV[a\x1B\xFFV[`\0a\x04\xD2`\x04Ta\x16cV[`\x08T`\0\x90\x81\x90\x81a\x16\x0B\x82g\r\xE0\xB6\xB3\xA7d\0\0a)jV[\x90P`\0g\r\xE0\xB6\xB3\xA7d\0\0\x83\x88a\x16$\x85\x8Aa)}V[a\x16.\x91\x90a)}V[a\x168\x91\x90a)\xAAV[a\x16B\x91\x90a)\xAAV[\x90Pa\x16N\x87\x82a\x0B\xAAV[PPPP\x92P\x92\x90PV[`\0a\x04\xD2a\x04\xD7V[`\0a\x04\xF5g\r\xE0\xB6\xB3\xA7d\0\0\x83a)\xAAV[`\0\x80a\x16\x82a\x13lV[\x90P`\0a\x16\x8Ea\x0E\x8DV[\x90P`\0a\x16\xA2`\x04T\x84`\x05T\x85a\x1BnV[\x90P`\0\x80\x87\x15a\x170W\x86\x91P`\0g\r\xE0\xB6\xB3\xA7d\0\0`\x03T\x84a\x16\xC9\x91\x90a)}V[a\x16\xD3\x91\x90a)\xAAV[\x90P`\0a\x16\xE1\x82\x85a)jV[\x90Pa\x17'\x81\x8Ca\x16\xF4W`\x05Ta\x16\xF8V[`\x04T[\x8Da\x17\x03W\x88a\x17\x05V[\x89[\x8Ea\x17\x12W`\x04Ta\x17\x16V[`\x05T[\x8Fa\x17!W\x8Ba\x1C&V[\x8Aa\x1C&V[\x92PPPa\x17\xB0V[P\x85`\0a\x17x\x82\x8Ba\x17EW`\x05Ta\x17IV[`\x04T[\x8Ca\x17TW\x87a\x17VV[\x88[\x8Da\x17cW`\x04Ta\x17gV[`\x05T[\x8Ea\x17rW\x8Aa\x1CcV[\x89a\x1CcV[\x90P`\x03Tg\r\xE0\xB6\xB3\xA7d\0\0a\x17\x90\x91\x90a)jV[a\x17\xA2\x82g\r\xE0\xB6\xB3\xA7d\0\0a)}V[a\x17\xAC\x91\x90a)\xAAV[\x92PP[\x88\x15a\x17\xE0Wa\x17\xC5`\x04Ta\x08G\x84a\x1A\xC5V[`\x04U`\x05Ta\x17\xD8\x90a\x0E\xA3\x83a\x1A\xC5V[`\x05Ua\x18\x06V[a\x17\xEF`\x04Ta\x0E\xA3\x83a\x1A\xC5V[`\x04U`\x05Ta\x18\x02\x90a\x08G\x84a\x1A\xC5V[`\x05U[`\0a\x18\x18`\x04T\x87`\x05T\x88a\x1BnV[\x90P\x83\x81\x10\x15a\x18EW`@Qc,\x88t9`\xE2\x1B\x81R`\x04\x81\x01\x85\x90R`$\x81\x01\x82\x90R`D\x01a\x05=V[\x89a\x18[W`\x02T`\x01`\x01`\xA0\x1B\x03\x16a\x18hV[`\x01T`\x01`\x01`\xA0\x1B\x03\x16[`\x01`\x01`\xA0\x1B\x03\x16c#\xB8r\xDD30\x86`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x18\x97\x93\x92\x91\x90a)\xCCV[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x18\xEFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a*$\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x19\x03W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19'\x91\x90a)\xF0V[P\x89a\x19>W`\x01T`\x01`\x01`\xA0\x1B\x03\x16a\x19KV[`\x02T`\x01`\x01`\xA0\x1B\x03\x16[`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x19\xD3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a*$\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x19\xE7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A\x0B\x91\x90a)\xF0V[P`\0a\x1A\x1E`\x04T\x88`\x05T\x89a\x1B\xFFV[`@\x80Q\x8D\x15\x15\x81R` \x81\x01\x87\x90R\x90\x81\x01\x85\x90R``\x81\x01\x82\x90R\x90\x91P3\x90\x7F\x17fH\xF1\xF1\x1C\xDA(L\x12D\x90\x08k\xE4*\x92m\xDF\n\xE8\x87\xEB\xE7\xB1\xD6\xB37\xD8\x94'V\x90`\x80\x01`@Q\x80\x91\x03\x90\xA2\x89a\x1AxW\x83a\x1AzV[\x82[\x97PPPPPPPP[\x93\x92PPPV[`\0a\x1A\xBDa\x13\xECa\x1A\xB7a\x1A\xA9a\x1A\xA3\x88\x88a\x1B\x0CV[\x88a\x1B\xE7V[g\r\xE0\xB6\xB3\xA7d\0\0a\x1B\xA0V[\x84a\x1B\x91V[\x94\x93PPPPV[`\0a\x1A\xDBg\r\xE0\xB6\xB3\xA7d\0\0`\0\x19a)\xAAV[\x82\x11\x15a\x1A\xFEW`@Qc\x1C\xD9Q\xA7`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R`$\x01a\x05=V[Pg\r\xE0\xB6\xB3\xA7d\0\0\x02\x90V[`\0a\x04\xF2a\x1B\x1B\x83\x85a*\x10V[\x90V[`\0a\x1A\xBDa\x13\xECa\x1B@a\x1B3\x86\x88a\x1B\xE7V[a\t\x8D\x88a\x08G\x88a\x1A\xC5V[\x85a\x1B\xA0V[`\0a\x1A\xBDa\x13\xECa\x1Bha\x1B[\x87\x87a\x1B\xE7V[a\t\x8D\x87a\x08G\x88a\x1A\xC5V[\x86a\x1B\xA0V[`\0\x80a\x1B{\x86\x86a\x1C\x93V[\x90P`\0a\x1B\x89\x85\x85a\x1C\x93V[\x90Pa\x0EJ\x82\x82[`\0a\x04\xF2a\x1B\x1B\x84\x84a\x1D\xA0V[`\0a\x04\xF2a\x1B\x1B\x83\x85a)jV[`\0a\x1A\xBDa\x13\xECa\x1A\xB7g\r\xE0\xB6\xB3\xA7d\0\0a\x0E\xA3a\x1B\xD0\x89\x89a\x1B\xA0V[\x89a\x1B\xE7V[a\x1B\xDEa\x13lV[`\x08UB`\tUV[`\0a\x04\xF2a\x1B\x1B\x84g\r\xE0\xB6\xB3\xA7d\0\0\x85a\x1EVV[`\0\x80a\x1C\x0C\x84\x84a\x1B\xE7V[\x90P`\0a\x1C\x1A\x87\x87a\x1B\xE7V[\x90Pa\x0EJ\x82\x82a\x1B\xE7V[`\0\x80a\x1C:\x86a\x15j\x88a\x08G\x8Ba\x1A\xC5V[\x90Pa\x0EJa\x13\xEC\x85a\t\x8Dg\r\xE0\xB6\xB3\xA7d\0\0a\x0E\xA3a\x1C\\\x8B\x8Aa\x1B\xE7V[\x87\x90a\x1C\x93V[`\0\x80a\x1Cw\x84a\x15j\x86a\x0E\xA3\x8Ba\x1A\xC5V[\x90Pa\x0EJa\x13\xEC\x87a\t\x8Da\x1A\xA9a\x1C\x90\x88\x8Ba\x1B\xE7V[\x86\x90[`\0\x82\x82\x81\x83\x03a\x1C\xBEW\x80\x15a\x1C\xABW`\0a\x1C\xB5V[g\r\xE0\xB6\xB3\xA7d\0\0[\x92PPPa\x04\xF5V[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x03a\x1C\xDFWg\r\xE0\xB6\xB3\xA7d\0\0\x92PPPa\x04\xF5V[\x80`\0\x03a\x1C\xF9Wg\r\xE0\xB6\xB3\xA7d\0\0\x92PPPa\x04\xF5V[g\r\xE0\xB6\xB3\xA7d\0\0\x81\x03a\x1D\x12W\x84\x92PPPa\x04\xF5V[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x11\x15a\x1DCWa\x1D<a\x1D7a\x1D1\x87a\x1F*V[\x86a\x1B\x91V[a XV[\x92Pa\x1D\x98V[`\0a\x1Daa\x1B\x1B\x84n\xC0\x97\xCE{\xC9\x07\x15\xB3K\x9F\x10\0\0\0\0a)\xAAV[\x90P`\0a\x1Dza\x1D7a\x1Dt\x84a\x1F*V[\x88a\x1B\x91V[\x90Pa\x0EJa\x1B\x1B\x82n\xC0\x97\xCE{\xC9\x07\x15\xB3K\x9F\x10\0\0\0\0a)\xAAV[PP\x92\x91PPV[`\0\x80\x80`\0\x19\x84\x86\t\x84\x86\x02\x92P\x82\x81\x10\x83\x82\x03\x03\x91PP\x80`\0\x03a\x1D\xD4WPg\r\xE0\xB6\xB3\xA7d\0\0\x90\x04\x90Pa\x04\xF5V[g\r\xE0\xB6\xB3\xA7d\0\0\x81\x10a\x1E\x06W`@QcQsd\x8D`\xE0\x1B\x81R`\x04\x81\x01\x86\x90R`$\x81\x01\x85\x90R`D\x01a\x05=V[`\0g\r\xE0\xB6\xB3\xA7d\0\0\x85\x87\tb\x04\0\0\x81\x85\x03\x04\x93\x10\x90\x91\x03`\x01`\xEE\x1B\x02\x91\x90\x91\x17\x7F\xAC\xCB\x18\x16[\xD6\xFE1\xAE\x1C\xF3\x18\xDC[Q\xEE\xE0\xE1\xBAV\x9B\x88\xCDt\xC1w;\x91\xFA\xC1\x06i\x02\x91PP\x92\x91PPV[`\0\x80\x80`\0\x19\x85\x87\t\x85\x87\x02\x92P\x82\x81\x10\x83\x82\x03\x03\x91PP\x80`\0\x03a\x1E\x90W\x83\x82\x81a\x1E\x86Wa\x1E\x86a)\x94V[\x04\x92PPPa\x1A\x84V[\x83\x81\x10a\x1E\xC1W`@Qc\x0Ct\n\xEF`\xE3\x1B\x81R`\x04\x81\x01\x87\x90R`$\x81\x01\x86\x90R`D\x81\x01\x85\x90R`d\x01a\x05=V[`\0\x84\x86\x88\t`\x02`\x01\x87\x19\x81\x01\x88\x16\x97\x88\x90\x04`\x03\x81\x02\x83\x18\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x90\x81\x02\x90\x92\x03\x90\x91\x02`\0\x88\x90\x03\x88\x90\x04\x90\x91\x01\x85\x83\x11\x90\x94\x03\x93\x90\x93\x02\x93\x03\x94\x90\x94\x04\x91\x90\x91\x17\x02\x94\x93PPPPV[`\0\x81g\r\xE0\xB6\xB3\xA7d\0\0\x81\x10\x15a\x1FYW`@Qc\x03m2\xEF`\xE4\x1B\x81R`\x04\x81\x01\x84\x90R`$\x01a\x05=V[`\0a\x1F\xE5g\r\xE0\xB6\xB3\xA7d\0\0\x83\x04`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x91\x82\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11`\x06\x1B\x90\x81\x1Cc\xFF\xFF\xFF\xFF\x81\x11`\x05\x1B\x90\x81\x1Ca\xFF\xFF\x81\x11`\x04\x1B\x90\x81\x1C`\xFF\x81\x11`\x03\x90\x81\x1B\x91\x82\x1C`\x0F\x81\x11`\x02\x1B\x90\x81\x1C\x91\x82\x11\x87\x1B\x91\x82\x1C\x96\x90\x96\x11\x94\x90\x96\x17\x92\x90\x92\x17\x17\x17\x91\x90\x91\x17\x91\x90\x91\x17\x17\x90V[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0\x81\x02\x82\x82\x1Cg\r\xE0\xB6\xB3\xA7c\xFF\xFF\x19\x81\x01a \x0EWP\x94\x93PPPPV[g\x1B\xC1mgN\xC8\0\0g\x06\xF0[Y\xD3\xB2\0\0[\x80\x15a QWg\r\xE0\xB6\xB3\xA7d\0\0\x83\x80\x02\x04\x92P\x81\x83\x10a IW\x92\x83\x01\x92`\x01\x92\x90\x92\x1C\x91[`\x01\x1Ca !V[P\x82a\x0EJV[`\0\x81h\nh\x89\x06\xBD\x8A\xFF\xFF\xFF\x81\x11\x15a \x88W`@Qc\xB3\xB6\xBA\x1F`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x01a\x05=V[`\0a \xA0g\r\xE0\xB6\xB3\xA7d\0\0`@\x84\x90\x1Ba)\xAAV[\x90Pa\x1A\xBDa\x1B\x1B\x82`\x01`\xBF\x1Bg\xFF\0\0\0\0\0\0\0\x82\x16\x15a!\xB6Wg\x80\0\0\0\0\0\0\0\x82\x16\x15a \xDDWh\x01j\t\xE6g\xF3\xBC\xC9\t\x02`@\x1C[g@\0\0\0\0\0\0\0\x82\x16\x15a \xFCWh\x010o\xE0\xA3\x1BqR\xDF\x02`@\x1C[g \0\0\0\0\0\0\0\x82\x16\x15a!\x1BWh\x01\x17+\x83\xC7\xD5\x17\xAD\xCE\x02`@\x1C[g\x10\0\0\0\0\0\0\0\x82\x16\x15a!:Wh\x01\x0BU\x86\xCF\x98\x90\xF6*\x02`@\x1C[g\x08\0\0\0\0\0\0\0\x82\x16\x15a!YWh\x01\x05\x9B\r1XWC\xAE\x02`@\x1C[g\x04\0\0\0\0\0\0\0\x82\x16\x15a!xWh\x01\x02\xC9\xA3\xE7x\x06\x0E\xE7\x02`@\x1C[g\x02\0\0\0\0\0\0\0\x82\x16\x15a!\x97Wh\x01\x01c\xDA\x9F\xB33V\xD8\x02`@\x1C[g\x01\0\0\0\0\0\0\0\x82\x16\x15a!\xB6Wh\x01\0\xB1\xAF\xA5\xAB\xCB\xEDa\x02`@\x1C[f\xFF\0\0\0\0\0\0\x82\x16\x15a\"\xB5Wf\x80\0\0\0\0\0\0\x82\x16\x15a!\xE3Wh\x01\0X\xC8m\xA1\xC0\x9E\xA2\x02`@\x1C[f@\0\0\0\0\0\0\x82\x16\x15a\"\x01Wh\x01\0,`^.\x8C\xECP\x02`@\x1C[f \0\0\0\0\0\0\x82\x16\x15a\"\x1FWh\x01\0\x16/9\x04\x05\x1F\xA1\x02`@\x1C[f\x10\0\0\0\0\0\0\x82\x16\x15a\"=Wh\x01\0\x0B\x17^\xFF\xDCv\xBA\x02`@\x1C[f\x08\0\0\0\0\0\0\x82\x16\x15a\"[Wh\x01\0\x05\x8B\xA0\x1F\xB9\xF9m\x02`@\x1C[f\x04\0\0\0\0\0\0\x82\x16\x15a\"yWh\x01\0\x02\xC5\xCC7\xDA\x94\x92\x02`@\x1C[f\x02\0\0\0\0\0\0\x82\x16\x15a\"\x97Wh\x01\0\x01b\xE5%\xEE\x05G\x02`@\x1C[f\x01\0\0\0\0\0\0\x82\x16\x15a\"\xB5Wh\x01\0\0\xB1rUw\\\x04\x02`@\x1C[e\xFF\0\0\0\0\0\x82\x16\x15a#\xABWe\x80\0\0\0\0\0\x82\x16\x15a\"\xE0Wh\x01\0\0X\xB9\x1B[\xC9\xAE\x02`@\x1C[e@\0\0\0\0\0\x82\x16\x15a\"\xFDWh\x01\0\0,\\\x89\xD5\xECm\x02`@\x1C[e \0\0\0\0\0\x82\x16\x15a#\x1AWh\x01\0\0\x16.C\xF4\xF81\x02`@\x1C[e\x10\0\0\0\0\0\x82\x16\x15a#7Wh\x01\0\0\x0B\x17!\xBC\xFC\x9A\x02`@\x1C[e\x08\0\0\0\0\0\x82\x16\x15a#TWh\x01\0\0\x05\x8B\x90\xCF\x1En\x02`@\x1C[e\x04\0\0\0\0\0\x82\x16\x15a#qWh\x01\0\0\x02\xC5\xC8c\xB7?\x02`@\x1C[e\x02\0\0\0\0\0\x82\x16\x15a#\x8EWh\x01\0\0\x01b\xE40\xE5\xA2\x02`@\x1C[e\x01\0\0\0\0\0\x82\x16\x15a#\xABWh\x01\0\0\0\xB1r\x185Q\x02`@\x1C[d\xFF\0\0\0\0\x82\x16\x15a$\x98Wd\x80\0\0\0\0\x82\x16\x15a#\xD4Wh\x01\0\0\0X\xB9\x0C\x0BI\x02`@\x1C[d@\0\0\0\0\x82\x16\x15a#\xF0Wh\x01\0\0\0,\\\x86\x01\xCC\x02`@\x1C[d \0\0\0\0\x82\x16\x15a$\x0CWh\x01\0\0\0\x16.B\xFF\xF0\x02`@\x1C[d\x10\0\0\0\0\x82\x16\x15a$(Wh\x01\0\0\0\x0B\x17!\x7F\xBB\x02`@\x1C[d\x08\0\0\0\0\x82\x16\x15a$DWh\x01\0\0\0\x05\x8B\x90\xBF\xCE\x02`@\x1C[d\x04\0\0\0\0\x82\x16\x15a$`Wh\x01\0\0\0\x02\xC5\xC8_\xE3\x02`@\x1C[d\x02\0\0\0\0\x82\x16\x15a$|Wh\x01\0\0\0\x01b\xE4/\xF1\x02`@\x1C[d\x01\0\0\0\0\x82\x16\x15a$\x98Wh\x01\0\0\0\0\xB1r\x17\xF8\x02`@\x1C[c\xFF\0\0\0\x82\x16\x15a%|Wc\x80\0\0\0\x82\x16\x15a$\xBFWh\x01\0\0\0\0X\xB9\x0B\xFC\x02`@\x1C[c@\0\0\0\x82\x16\x15a$\xDAWh\x01\0\0\0\0,\\\x85\xFE\x02`@\x1C[c \0\0\0\x82\x16\x15a$\xF5Wh\x01\0\0\0\0\x16.B\xFF\x02`@\x1C[c\x10\0\0\0\x82\x16\x15a%\x10Wh\x01\0\0\0\0\x0B\x17!\x7F\x02`@\x1C[c\x08\0\0\0\x82\x16\x15a%+Wh\x01\0\0\0\0\x05\x8B\x90\xC0\x02`@\x1C[c\x04\0\0\0\x82\x16\x15a%FWh\x01\0\0\0\0\x02\xC5\xC8`\x02`@\x1C[c\x02\0\0\0\x82\x16\x15a%aWh\x01\0\0\0\0\x01b\xE40\x02`@\x1C[c\x01\0\0\0\x82\x16\x15a%|Wh\x01\0\0\0\0\0\xB1r\x18\x02`@\x1C[b\xFF\0\0\x82\x16\x15a&WWb\x80\0\0\x82\x16\x15a%\xA1Wh\x01\0\0\0\0\0X\xB9\x0C\x02`@\x1C[b@\0\0\x82\x16\x15a%\xBBWh\x01\0\0\0\0\0,\\\x86\x02`@\x1C[b \0\0\x82\x16\x15a%\xD5Wh\x01\0\0\0\0\0\x16.C\x02`@\x1C[b\x10\0\0\x82\x16\x15a%\xEFWh\x01\0\0\0\0\0\x0B\x17!\x02`@\x1C[b\x08\0\0\x82\x16\x15a&\tWh\x01\0\0\0\0\0\x05\x8B\x91\x02`@\x1C[b\x04\0\0\x82\x16\x15a&#Wh\x01\0\0\0\0\0\x02\xC5\xC8\x02`@\x1C[b\x02\0\0\x82\x16\x15a&=Wh\x01\0\0\0\0\0\x01b\xE4\x02`@\x1C[b\x01\0\0\x82\x16\x15a&WWh\x01\0\0\0\0\0\0\xB1r\x02`@\x1C[a\xFF\0\x82\x16\x15a')Wa\x80\0\x82\x16\x15a&zWh\x01\0\0\0\0\0\0X\xB9\x02`@\x1C[a@\0\x82\x16\x15a&\x93Wh\x01\0\0\0\0\0\0,]\x02`@\x1C[a \0\x82\x16\x15a&\xACWh\x01\0\0\0\0\0\0\x16.\x02`@\x1C[a\x10\0\x82\x16\x15a&\xC5Wh\x01\0\0\0\0\0\0\x0B\x17\x02`@\x1C[a\x08\0\x82\x16\x15a&\xDEWh\x01\0\0\0\0\0\0\x05\x8C\x02`@\x1C[a\x04\0\x82\x16\x15a&\xF7Wh\x01\0\0\0\0\0\0\x02\xC6\x02`@\x1C[a\x02\0\x82\x16\x15a'\x10Wh\x01\0\0\0\0\0\0\x01c\x02`@\x1C[a\x01\0\x82\x16\x15a')Wh\x01\0\0\0\0\0\0\0\xB1\x02`@\x1C[`\xFF\x82\x16\x15a'\xF2W`\x80\x82\x16\x15a'JWh\x01\0\0\0\0\0\0\0Y\x02`@\x1C[`@\x82\x16\x15a'bWh\x01\0\0\0\0\0\0\0,\x02`@\x1C[` \x82\x16\x15a'zWh\x01\0\0\0\0\0\0\0\x16\x02`@\x1C[`\x10\x82\x16\x15a'\x92Wh\x01\0\0\0\0\0\0\0\x0B\x02`@\x1C[`\x08\x82\x16\x15a'\xAAWh\x01\0\0\0\0\0\0\0\x06\x02`@\x1C[`\x04\x82\x16\x15a'\xC2Wh\x01\0\0\0\0\0\0\0\x03\x02`@\x1C[`\x02\x82\x16\x15a'\xDAWh\x01\0\0\0\0\0\0\0\x01\x02`@\x1C[`\x01\x82\x16\x15a'\xF2Wh\x01\0\0\0\0\0\0\0\x01\x02`@\x1C[g\r\xE0\xB6\xB3\xA7d\0\0\x02`@\x91\x90\x91\x1C`\xBF\x03\x1C\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[\x80\x15\x15\x81\x14a(gW`\0\x80\xFD[PV[`\0\x80`@\x83\x85\x03\x12\x15a(\x80Wa(\x80a(\tV[\x825a(\x8B\x81a(YV[\x94` \x93\x90\x93\x015\x93PPPV[`\0` \x82\x84\x03\x12\x15a(\xAEWa(\xAEa(\tV[P5\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a(\xCBWa(\xCBa(\tV[PP\x805\x92` \x90\x91\x015\x91PV[`\0` \x82\x84\x03\x12\x15a(\xEFWa(\xEFa(\tV[\x815`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x1A\x84W`\0\x80\xFD[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a)3W\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a)\x17V[P`\0`@\x82\x86\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x92PPP\x92\x91PPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x04\xF5Wa\x04\xF5a)TV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x04\xF5Wa\x04\xF5a)TV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a)\xC7WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x81R\x91\x90\x92\x16` \x82\x01R`@\x81\x01\x91\x90\x91R``\x01\x90V[`\0` \x82\x84\x03\x12\x15a*\x05Wa*\x05a(\tV[\x81Qa\x1A\x84\x81a(YV[\x80\x82\x01\x80\x82\x11\x15a\x04\xF5Wa\x04\xF5a)TV\xFETarget contract does not contain";
    /// The deployed bytecode of the contract.
    pub static G3M_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
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
            f.debug_tuple(::core::stringify!(G3M))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> G3M<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                G3M_ABI.clone(),
                client,
            ))
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
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
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
        ///Calls the contract's `getLiquidity` (0x0910a510) function
        pub fn get_liquidity(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([9, 16, 165, 16], ())
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
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Bytes> {
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
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([22, 220, 22, 91], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `tokenY` (0xb7d19fc4) function
        pub fn token_y(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
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
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, AddLiquidityFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `LogPrices` event
        pub fn log_prices_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LogPricesFilter> {
            self.0.event()
        }
        ///Gets the contract's `LogReserves` event
        pub fn log_reserves_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LogReservesFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `LogSyncingWeight` event
        pub fn log_syncing_weight_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LogSyncingWeightFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `LogWeights` event
        pub fn log_weights_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LogWeightsFilter> {
            self.0.event()
        }
        ///Gets the contract's `RemoveLiquidity` event
        pub fn remove_liquidity_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, RemoveLiquidityFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `SetTargetWeightX` event
        pub fn set_target_weight_x_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, SetTargetWeightXFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `SetWeightX` event
        pub fn set_weight_x_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, SetWeightXFilter> {
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
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for G3M<M> {
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
            if let Ok(decoded) =
                <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <InvalidSwap as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidSwap(decoded));
            }
            if let Ok(decoded) =
                <PRBMath_MulDiv18_Overflow as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PRBMath_MulDiv18_Overflow(decoded));
            }
            if let Ok(decoded) =
                <PRBMath_MulDiv_Overflow as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PRBMath_MulDiv_Overflow(decoded));
            }
            if let Ok(decoded) =
                <PRBMath_UD60x18_Convert_Overflow as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PRBMath_UD60x18_Convert_Overflow(decoded));
            }
            if let Ok(decoded) =
                <PRBMath_UD60x18_Exp2_InputTooBig as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PRBMath_UD60x18_Exp2_InputTooBig(decoded));
            }
            if let Ok(decoded) =
                <PRBMath_UD60x18_Log_InputTooSmall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PRBMath_UD60x18_Log_InputTooSmall(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for G3MErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::InvalidSwap(element) => ::ethers::core::abi::AbiEncode::encode(element),
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
                Self::PRBMath_MulDiv18_Overflow(element) => ::core::fmt::Display::fmt(element, f),
                Self::PRBMath_MulDiv_Overflow(element) => ::core::fmt::Display::fmt(element, f),
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
                Self::AddLiquidityFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogPricesFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogReservesFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogSyncingWeightFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogWeightsFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveLiquidityFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetTargetWeightXFilter(element) => ::core::fmt::Display::fmt(element, f),
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
    )]
    #[ethcall(name = "getLiquidity", abi = "getLiquidity()")]
    pub struct GetLiquidityCall;
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
    )]
    #[ethcall(
        name = "liquidityWithoutPrecision",
        abi = "liquidityWithoutPrecision()"
    )]
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
    )]
    pub enum G3MCalls {
        _InitPool(_InitPoolCall),
        AddLiquidity(AddLiquidityCall),
        AddLiquidityWithExactX(AddLiquidityWithExactXCall),
        Admin(AdminCall),
        BalanceOf(BalanceOfCall),
        GetInvariant(GetInvariantCall),
        GetLiquidity(GetLiquidityCall),
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
            if let Ok(decoded) = <_InitPoolCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::_InitPool(decoded));
            }
            if let Ok(decoded) = <AddLiquidityCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AddLiquidity(decoded));
            }
            if let Ok(decoded) =
                <AddLiquidityWithExactXCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AddLiquidityWithExactX(decoded));
            }
            if let Ok(decoded) = <AdminCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Admin(decoded));
            }
            if let Ok(decoded) = <BalanceOfCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BalanceOf(decoded));
            }
            if let Ok(decoded) = <GetInvariantCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetInvariant(decoded));
            }
            if let Ok(decoded) = <GetLiquidityCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetLiquidity(decoded));
            }
            if let Ok(decoded) = <GetReserveXCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetReserveX(decoded));
            }
            if let Ok(decoded) = <GetReserveYCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetReserveY(decoded));
            }
            if let Ok(decoded) = <GetSpotPriceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetSpotPrice(decoded));
            }
            if let Ok(decoded) =
                <GetStrategyDataCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetStrategyData(decoded));
            }
            if let Ok(decoded) = <GetSwapFeeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetSwapFee(decoded));
            }
            if let Ok(decoded) = <InitExactXCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InitExactX(decoded));
            }
            if let Ok(decoded) = <InitPoolCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InitPool(decoded));
            }
            if let Ok(decoded) = <InstantiateCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Instantiate(decoded));
            }
            if let Ok(decoded) =
                <LiquidityWithoutPrecisionCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::LiquidityWithoutPrecision(decoded));
            }
            if let Ok(decoded) = <LogDataCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LogData(decoded));
            }
            if let Ok(decoded) =
                <RemoveLiquidityCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RemoveLiquidity(decoded));
            }
            if let Ok(decoded) =
                <RemoveLiquidityWithExactXCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RemoveLiquidityWithExactX(decoded));
            }
            if let Ok(decoded) = <ReserveXCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ReserveX(decoded));
            }
            if let Ok(decoded) =
                <ReserveXWithoutPrecisionCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ReserveXWithoutPrecision(decoded));
            }
            if let Ok(decoded) = <ReserveYCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ReserveY(decoded));
            }
            if let Ok(decoded) =
                <ReserveYWithoutPrecisionCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ReserveYWithoutPrecision(decoded));
            }
            if let Ok(decoded) = <SetSwapFeeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetSwapFee(decoded));
            }
            if let Ok(decoded) = <SetWeightXCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetWeightX(decoded));
            }
            if let Ok(decoded) = <SwapAmountInCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SwapAmountIn(decoded));
            }
            if let Ok(decoded) = <SwapAmountOutCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SwapAmountOut(decoded));
            }
            if let Ok(decoded) = <SwapFeeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SwapFee(decoded));
            }
            if let Ok(decoded) = <TokenXCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TokenX(decoded));
            }
            if let Ok(decoded) = <TokenYCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TokenY(decoded));
            }
            if let Ok(decoded) =
                <TotalLiquidityCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TotalLiquidity(decoded));
            }
            if let Ok(decoded) = <WeightXCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::WeightX(decoded));
            }
            if let Ok(decoded) = <WeightYCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::WeightY(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for G3MCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::_InitPool(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AddLiquidity(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AddLiquidityWithExactX(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Admin(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BalanceOf(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetInvariant(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetLiquidity(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetReserveX(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetReserveY(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetSpotPrice(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetStrategyData(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetSwapFee(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InitExactX(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InitPool(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Instantiate(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LiquidityWithoutPrecision(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LogData(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RemoveLiquidity(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RemoveLiquidityWithExactX(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ReserveX(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ReserveXWithoutPrecision(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ReserveY(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ReserveYWithoutPrecision(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetSwapFee(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetWeightX(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SwapAmountIn(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SwapAmountOut(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SwapFee(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TokenX(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TokenY(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TotalLiquidity(element) => ::ethers::core::abi::AbiEncode::encode(element),
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
                Self::AddLiquidityWithExactX(element) => ::core::fmt::Display::fmt(element, f),
                Self::Admin(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetInvariant(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetLiquidity(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetReserveX(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetReserveY(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSpotPrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetStrategyData(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSwapFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::InitExactX(element) => ::core::fmt::Display::fmt(element, f),
                Self::InitPool(element) => ::core::fmt::Display::fmt(element, f),
                Self::Instantiate(element) => ::core::fmt::Display::fmt(element, f),
                Self::LiquidityWithoutPrecision(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogData(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveLiquidity(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveLiquidityWithExactX(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReserveX(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReserveXWithoutPrecision(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReserveY(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReserveYWithoutPrecision(element) => ::core::fmt::Display::fmt(element, f),
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
    impl ::core::convert::From<GetLiquidityCall> for G3MCalls {
        fn from(value: GetLiquidityCall) -> Self {
            Self::GetLiquidity(value)
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
    )]
    pub struct GetLiquidityReturn(pub ::ethers::core::types::U256);
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
    )]
    pub struct WeightYReturn(pub ::ethers::core::types::U256);
}
