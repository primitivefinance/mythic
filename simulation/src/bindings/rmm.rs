pub use rmm::*;
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
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("addLiquidity"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
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
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("addLiquidityExactX"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("addLiquidityExactX"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("amountX"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
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
                    ::std::borrow::ToOwned::to_owned("addLiquidityExactY"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("addLiquidityExactY"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("amountY"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
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
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("gamma"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("gamma"),
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
                    ::std::borrow::ToOwned::to_owned("initExactY"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("initExactY"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amountY"),
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
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("removeLiquidityExactX"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("removeLiquidityExactX",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("amountX"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
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
                    ::std::borrow::ToOwned::to_owned("reserveX"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("reserveX"),
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
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("sigma"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("sigma"),
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
                    ::std::borrow::ToOwned::to_owned("strikePrice"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("strikePrice"),
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
                    ::std::borrow::ToOwned::to_owned("swap"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("swap"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("amountX"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("amountY"),
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
                    ::std::borrow::ToOwned::to_owned("tau"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("tau"),
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
                                ::std::borrow::ToOwned::to_owned("contract ERC20"),
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
                                ::std::borrow::ToOwned::to_owned("contract ERC20"),
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
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("Infinity"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("Infinity"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Min"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("Min"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NegativeInfinity"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("NegativeInfinity"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OutOfBounds"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("OutOfBounds"),
                        inputs: ::std::vec![],
                    },],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static RMM_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"a\x01\0`@R`\x1E`\xE0R4\x80\x15b\0\0dW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`@Qb\0-E8\x03\x80b\0-E\x839\x81\x01`@\x81\x90Rb\0\0\x87\x91b\0\0\xE6V[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x96\x87\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x82\x16\x17\x90\x91U`\x01\x80T\x95\x90\x96\x16\x94\x16\x93\x90\x93\x17\x90\x93U`\x80R`\xA0\x91\x90\x91R`\xC0Rb\0\x01\x84V[\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\0\xE1W`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15b\0\x01JW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[b\0\x01U\x86b\0\0\xC9V[\x94Pb\0\x01e` \x87\x01b\0\0\xC9V[`@\x87\x01Q``\x88\x01Q`\x80\x90\x98\x01Q\x96\x99\x91\x98P\x96\x95\x94P\x92PPPV[`\x80Q`\xA0Q`\xC0Q`\xE0Qa*\x1Bb\0\x03*`\09`\0\x81\x81a\x03\x06\x01Ra\x0EY\x01R`\0\x81\x81a\x03z\x01R\x81\x81a\x04\n\x01R\x81\x81a\n\xC1\x01R\x81\x81a\x0E\xE7\x01R\x81\x81a\x10m\x01Ra\x14\xE2\x01R`\0\x81\x81a\x03@\x01R\x81\x81a\x03\xC8\x01R\x81\x81a\x04F\x01R\x81\x81a\x04\x96\x01R\x81\x81a\x07:\x01R\x81\x81a\x07\x88\x01R\x81\x81a\x07\xDE\x01R\x81\x81a\x08,\x01R\x81\x81a\n\x7F\x01R\x81\x81a\x0B\x0E\x01R\x81\x81a\x0B\x95\x01R\x81\x81a\x0B\xE5\x01R\x81\x81a\x0E\xA5\x01R\x81\x81a\x0F\x16\x01R\x81\x81a\x0Ff\x01R\x81\x81a\x0F\xC0\x01R\x81\x81a\x10+\x01R\x81\x81a\x10\xB1\x01R\x81\x81a\x12X\x01R\x81\x81a\x12\xA8\x01R\x81\x81a\x14\xA0\x01R\x81\x81a\x15&\x01R\x81\x81a\x16\xCE\x01Ra\x17\x1E\x01R`\0\x81\x81a\x02\xDF\x01R\x81\x81a\x03\xE9\x01R\x81\x81a\x04g\x01R\x81\x81a\x04\xB7\x01R\x81\x81a\x07[\x01R\x81\x81a\x07\xA9\x01R\x81\x81a\x07\xFF\x01R\x81\x81a\x08M\x01R\x81\x81a\n\xA0\x01R\x81\x81a\x0B/\x01R\x81\x81a\x0B\xB6\x01R\x81\x81a\x0C\x06\x01R\x81\x81a\x0E\xC6\x01R\x81\x81a\x0F7\x01R\x81\x81a\x0F\x87\x01R\x81\x81a\x0F\xE1\x01R\x81\x81a\x10L\x01R\x81\x81a\x10\xD2\x01R\x81\x81a\x12y\x01R\x81\x81a\x12\xC9\x01R\x81\x81a\x14\xC1\x01R\x81\x81a\x15G\x01R\x81\x81a\x16\xEF\x01Ra\x17?\x01Ra*\x1B`\0\xF3\xFE`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x01cW`\x005`\xE0\x1C\x80c\xA5\x9C\x18o\x11a\0\xEFW\x80c\xC5)\x87\xCF\x11a\0\xBEW\x80c\xC5)\x87\xCF\x14a\x03;W\x80c\xCC\xD1\xE4\xBE\x14a\x03bW\x80c\xCF\xC4\xAFU\x14a\x03uW\x80c\xF9\xA1\xC8Z\x14a\x03\x9CW\x80c\xFA\xDF\xA6[\x14a\x03\xAFWa\x01cV[\x80c\xA5\x9C\x18o\x14a\x02\xC7W\x80c\xAF\xDF1\xCD\x14a\x02\xDAW\x80c\xB179)\x14a\x03\x01W\x80c\xB7\xD1\x9F\xC4\x14a\x03(Wa\x01cV[\x80c\x16\xDC\x16[\x11a\x016W\x80c\x16\xDC\x16[\x14a\x02CW\x80cp\xA0\x821\x14a\x02nW\x80cvp\x166\x14a\x02\x8EW\x80c\x94\xB9\x18\xDE\x14a\x02\xA1W\x80c\x9C\x9D\xA9\xEA\x14a\x02\xB4Wa\x01cV[\x80c\x02\xC2\xE5]\x14a\x01\xC8W\x80c\x04\xAF\xA8\"\x14a\x01\xF5W\x80c\x08\xEA\xBD\xDA\x14a\x02#W\x80c\x15w\x0F\x92\x14a\x02:W[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x01\xDBa\x01\xD66`\x04a'OV[a\x03\xB8V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01[`@Q\x80\x91\x03\x90\xF3[a\x02\x08a\x02\x036`\x04a'|V[a\x06\xCEV[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x01\xECV[a\x02,`\x02T\x81V[`@Q\x90\x81R` \x01a\x01\xECV[a\x02,`\x04T\x81V[`\0Ta\x02V\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\xECV[a\x02,a\x02|6`\x04a'\xB4V[`\x05` R`\0\x90\x81R`@\x90 T\x81V[a\x02\x08a\x02\x9C6`\x04a'\xE7V[a\n%V[a\x02,a\x02\xAF6`\x04a'OV[a\x0ENV[a\x01\xDBa\x02\xC26`\x04a'OV[a\x10\x1BV[a\x01\xDBa\x02\xD56`\x04a(\x16V[a\x12LV[a\x02,\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02,\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\x01Ta\x02V\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x02,\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01\xDBa\x03p6`\x04a'OV[a\x14\x90V[a\x02,\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01\xDBa\x03\xAA6`\x04a(\x16V[a\x16\xC2V[a\x02,`\x03T\x81V[`\0\x80`\0a\x04.`\x02T`\x04T\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x18jV[\x90P`\0a\x04\x8B\x85`\x02Ta\x04C\x91\x90a(QV[\x83\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x19\x14V[\x90P`\0a\x04\xDB\x82\x84\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x19\xA0V[\x90P`\0`\x03T\x82a\x04\xED\x91\x90a(dV[\x90P`\0`\x04T\x84a\x04\xFF\x91\x90a(dV[\x90P\x83`\x04\x81\x90UP\x87`\x02`\0\x82\x82Ta\x05\x1A\x91\x90a(QV[\x92PP\x81\x90UP\x81`\x03`\0\x82\x82Ta\x053\x91\x90a(QV[\x90\x91UPP`\0T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\x05l\x903\x900\x90\x8D\x90`\x04\x01a(wV[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x05\xC4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a)\xC6\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x05\xD8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xFC\x91\x90a(\x9BV[P`\x01T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\x061\x903\x900\x90\x87\x90`\x04\x01a(wV[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x06\x89W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a)\xC6\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x06\x9DW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xC1\x91\x90a(\x9BV[P\x97\x90\x96P\x94PPPPPV[`\0\x80`\0`\x04T`\0\x14a\x07*W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FPool already initialized\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[\x85\x15a\x07\xD4W\x84\x92Pa\x07\x7F\x83\x85\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x19\x14V[\x90Pa\x07\xCD\x81\x85\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x19\xA0V[\x91Pa\x08tV[\x84\x91Pa\x08#\x82\x85\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1A,V[\x90Pa\x08q\x81\x85\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1A\x91V[\x92P[`\x04\x81\x81U`\x02\x84\x90U`\x03\x83\x90U3`\0\x81\x81R`\x05` R`@\x80\x82 \x85\x90U\x90T\x90Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x92c#\xB8r\xDD\x92a\x08\xC6\x92\x90\x910\x91\x89\x91\x01a(wV[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\t\x1EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a)\xC6\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\t2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\tV\x91\x90a(\x9BV[P`\x01T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\t\x8B\x903\x900\x90\x87\x90`\x04\x01a(wV[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\t\xE3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a)\xC6\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\t\xF7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\x1B\x91\x90a(\x9BV[P\x93P\x93P\x93\x90PV[`\0\x80`\0\x80`\x04T\x11a\nrW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rs\x14\x1B\xDB\xDB\x08\x1B\x9B\xDD\x08\x1A[\x9A]\x1AX[\x1A^\x99Y`b\x1B`D\x82\x01R`d\x01a\x07!V[`\0a\n\xE5`\x02T`\x04T\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x18jV[\x90P\x85\x15a\x0B|W\x84\x93P`\0a\x0B\x03\x85`\x02Ta\x04C\x91\x90a(QV[\x90P`\0a\x0BS\x82\x84\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x19\xA0V[\x90P`\x03T\x81a\x0Bc\x91\x90a(dV[\x94P`\x04T\x82a\x0Bs\x91\x90a(dV[\x93PPPa\x0COV[\x84\x92P`\0a\x0B\xDA\x84`\x03Ta\x0B\x92\x91\x90a(QV[\x83\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1A,V[\x90P`\0a\x0C*\x82\x84\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1A\x91V[\x90P`\x02T\x81a\x0C:\x91\x90a(dV[\x95P`\x04T\x82a\x0CJ\x91\x90a(dV[\x93PPP[\x81`\x04`\0\x82\x82Ta\x0Ca\x91\x90a(QV[\x92PP\x81\x90UP\x83`\x02`\0\x82\x82Ta\x0Cz\x91\x90a(QV[\x92PP\x81\x90UP\x82`\x03`\0\x82\x82Ta\x0C\x93\x91\x90a(QV[\x90\x91UPP3`\0\x90\x81R`\x05` R`@\x81 \x80T\x84\x92\x90a\x0C\xB7\x90\x84\x90a(QV[\x90\x91UPP`\0T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\x0C\xF0\x903\x900\x90\x89\x90`\x04\x01a(wV[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\rHW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a)\xC6\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\r\\W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\x80\x91\x90a(\x9BV[P`\x01T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\r\xB5\x903\x900\x90\x88\x90`\x04\x01a(wV[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0E\rW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a)\xC6\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x0E!W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0EE\x91\x90a(\x9BV[PP\x92P\x92P\x92V[`\0\x80a'\x10a\x0E~\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85a(\xBBV[a\x0E\x88\x91\x90a(\xE8V[\x90P`\0a\x0E\x96\x82\x85a(dV[\x90P`\0a\x0F\x0B`\x02T`\x04T\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x18jV[\x90P`\0a\x0F[\x83\x83\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x19\x14V[\x90P`\0a\x0F\xAB\x82\x84\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x19\xA0V[\x90P`\x01a\x10\x05`\x02T\x89`\x03T\x85`\x04T\x88\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1A\xEDV[a\x10\x0F\x91\x90a(\xFCV[\x19\x97\x96PPPPPPPV[`\0\x80`\0a\x10\x91`\x02T`\x04T\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x18jV[\x90P`\0a\x10\xA6\x85`\x03Ta\x0B\x92\x91\x90a(QV[\x90P`\0a\x10\xF6\x82\x84\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1A\x91V[\x90P`\0`\x02T\x82a\x11\x08\x91\x90a(dV[\x90P`\0`\x04T\x84a\x11\x1A\x91\x90a(dV[\x90P\x83`\x04\x81\x90UP\x81`\x02`\0\x82\x82Ta\x115\x91\x90a(QV[\x92PP\x81\x90UP\x87`\x03`\0\x82\x82Ta\x11N\x91\x90a(QV[\x90\x91UPP`\0T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\x11\x87\x903\x900\x90\x87\x90`\x04\x01a(wV[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x11\xDFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a)\xC6\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x11\xF3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\x17\x91\x90a(\x9BV[P`\x01T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\x061\x903\x900\x90\x8D\x90`\x04\x01a(wV[`\0\x80`\0a\x12\x9D\x85\x85\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1A,V[\x90P`\0a\x12\xED\x82\x86\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1A\x91V[`\x04\x83\x81U`\x02\x82\x90U`\x03\x88\x90U`\0T`@Qc#\xB8r\xDD`\xE0\x1B\x81R\x92\x93P`\x01`\x01`\xA0\x1B\x03\x16\x91c#\xB8r\xDD\x91a\x13/\x913\x910\x91\x87\x91\x01a(wV[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x13\x87W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a)\xC6\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x13\x9BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\xBF\x91\x90a(\x9BV[P`\x01T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\x13\xF4\x903\x900\x90\x8B\x90`\x04\x01a(wV[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x14LW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a)\xC6\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x14`W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14\x84\x91\x90a(\x9BV[P\x90\x95\x90\x94P\x92PPPV[`\0\x80`\0a\x15\x06`\x02T`\x04T\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x18jV[\x90P`\0a\x15\x1B\x85`\x02Ta\x04C\x91\x90a(dV[\x90P`\0a\x15k\x82\x84\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x19\xA0V[\x90P`\0\x81`\x03Ta\x15}\x91\x90a(dV[\x90P`\0\x83`\x04Ta\x15\x8F\x91\x90a(dV[\x90P\x83`\x04\x81\x90UP\x87`\x02`\0\x82\x82Ta\x15\xAA\x91\x90a(dV[\x92PP\x81\x90UP\x81`\x03`\0\x82\x82Ta\x15\xC3\x91\x90a(dV[\x90\x91UPP`\0T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x8A\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x16RW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a)\xC6\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x16fW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\x8A\x91\x90a(\x9BV[P`\x01T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01a\x061V[`\0\x80`\0a\x17\x13\x85\x85\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x19\x14V[\x90P`\0a\x17c\x82\x86\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x19\xA0V[`\x04\x83\x81U`\x02\x88\x90U`\x03\x82\x90U`\0T`@Qc#\xB8r\xDD`\xE0\x1B\x81R\x92\x93P`\x01`\x01`\xA0\x1B\x03\x16\x91c#\xB8r\xDD\x91a\x17\xA5\x913\x910\x91\x8C\x91\x01a(wV[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x17\xFDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a)\xC6\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x18\x11W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x185\x91\x90a(\x9BV[P`\x01T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\x13\xF4\x903\x900\x90\x86\x90`\x04\x01a(wV[`\0\x80a\x18\x99g\x06\xF0[Y\xD3\xB2\0\0a\x18\x94a\x18\x8E\x87g\x1B\xC1mgN\xC8\0\0a\x1BhV[\x86a\x1B\x9DV[a\x1B\x9DV[\x90P`\0a\x18\xAA\x85a\x18\x94\x86a\x1B\xB2V[a\x18\xB8\x90c;\x9A\xCA\0a(\xBBV[\x90P`\0a\x18\xC6\x89\x89a\x1CVV[\x90Pa\x19\x07\x87a\x19\x02\x85a\x18\xF3a\x18\xEDa\x18\xE8\x87g\r\xE0\xB6\xB3\xA7d\0\0a(dV[a\x1CkV[\x87a\x1B\x9DV[a\x18\xFD\x91\x90a(dV[a\x1D\x11V[a\x1E\xBAV[\x99\x98PPPPPPPPPV[`\0\x80a\x19)a\x19$\x86\x86a\x1E\xCFV[a\x1E\xE4V[\x90P`\0a\x19Kg\x06\xF0[Y\xD3\xB2\0\0a\x19\x02\x86g\x1B\xC1mgN\xC8\0\0a\x1BhV[\x90P`\0a\x19~\x85a\x19]\x84\x86a)#V[a\x19o\x90g\r\xE0\xB6\xB3\xA7d\0\0a)KV[a\x19y\x91\x90a){V[a \xBFV[\x90P`\0a\x19\x94\x82g\r\xE0\xB6\xB3\xA7d\0\0a(\xFCV[\x90Pa\x19\x07\x89\x82a\x1E\xCFV[`\0\x80a\x19\xB0a\x19$\x86\x86a\x1E\xCFV[\x90P`\0a\x19\xD2g\x06\xF0[Y\xD3\xB2\0\0a\x19\x02\x86g\x1B\xC1mgN\xC8\0\0a\x1BhV[\x90P`\0a\x19\xE0\x82\x84a(\xFCV[\x90P`\0\x85a\x19\xF7\x83g\r\xE0\xB6\xB3\xA7d\0\0a)KV[a\x1A\x01\x91\x90a){V[\x90P`\0a\x1A\x0E\x82a \xBFV[\x90Pa\x1A\x1E\x88a\x19\x02\x8C\x84a\x1E\xBAV[\x9A\x99PPPPPPPPPPV[`\0\x80a\x1A<a\x19$\x86\x86a\x1E\xCFV[\x90P`\0a\x1A^g\x06\xF0[Y\xD3\xB2\0\0a\x19\x02\x86g\x1B\xC1mgN\xC8\0\0a\x1BhV[\x90P`\0a\x1Ap\x85a\x19]\x84\x86a(\xFCV[\x90Pa\x1A\x85\x88a\x1A\x80\x88\x84a\x1E\xBAV[a\x1E\xCFV[\x98\x97PPPPPPPPV[`\0\x80a\x1A\xA1a\x19$\x86\x86a\x1E\xCFV[\x90P`\0a\x1A\xC3g\x06\xF0[Y\xD3\xB2\0\0a\x19\x02\x86g\x1B\xC1mgN\xC8\0\0a\x1BhV[\x90P`\0a\x1A\xD5\x85a\x19]\x84\x86a)#V[\x90Pa\x1A\x85\x88a\x19\x02\x83g\r\xE0\xB6\xB3\xA7d\0\0a(\xFCV[`\0\x80a\x1A\xFE\x84a\x18\x94\x87\x89a(QV[\x90P`\0a\x1B7a\x1B$a\x18\xE8a\x1B\x15\x8D\x8Fa(QV[a\x1B\x1F\x8A\x8Ca(QV[a\x1CVV[a\x1B-\x86a)\xA9V[a\x19y\x91\x90a(\xFCV[\x90P\x87\x89a\x1BE\x84\x84a\x1B\x9DV[a\x1BO\x91\x90a(\xFCV[a\x1BY\x91\x90a(\xFCV[\x9B\x9APPPPPPPPPPPV[`\0a\x1B\x94g\r\xE0\xB6\xB3\xA7d\0\0\x83a\x1B\x80\x86a\x1E\xE4V[a\x1B\x8A\x91\x90a)KV[a\x18\xFD\x91\x90a){V[\x90P[\x92\x91PPV[`\0a\x1B\x94\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a!(V[`\xB5\x81`\x01`\x88\x1B\x81\x10a\x1B\xCBW`@\x91\x90\x91\x1B\x90`\x80\x1C[i\x01\0\0\0\0\0\0\0\0\0\x81\x10a\x1B\xE7W` \x91\x90\x91\x1B\x90`@\x1C[e\x01\0\0\0\0\0\x81\x10a\x1B\xFFW`\x10\x91\x90\x91\x1B\x90` \x1C[c\x01\0\0\0\x81\x10a\x1C\x15W`\x08\x91\x90\x91\x1B\x90`\x10\x1C[b\x01\0\0\x01\x02`\x12\x1C\x80\x82\x04\x01`\x01\x90\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x90\x81\x90\x04\x81\x11\x90\x03\x90V[`\0a\x1B\x94\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a!(V[`\0g\x06\xF0[Y\xD3\xB2\0\0\x82\x03a\x1C\x84WP`\0\x91\x90PV[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x12a\x1C\xACW`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\x1C\xCDW`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x1C\xDA\x83`\x02a)KV[\x90P`\0a\x1C\xE7\x82a!GV[\x90P`\0a\x1C\xFDg\x13\xA0K\xBD\xFD\xC9\xBE\x88\x83a#\xC5V[\x90Pa\x1D\x08\x81a)\xA9V[\x95\x94PPPPPV[`\0h\x02H\xCE6\xA7\x0C\xB2k>\x19\x82\x13a\x1D,WP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a\x1DsW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x01a\x07!V[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05`\x01`_\x1B\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1Dm\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1Dn\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[`\0a\x1B\x94\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a#\xDAV[`\0a\x1B\x94\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a#\xDAV[`\0\x80\x82\x13a\x1F!W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x07!V[`\0``a\x1F.\x84a$\x08V[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[`\0\x80g\x13\xA0K\xBD\xFD\xC9\xBE\x88a \xDDg\r\xE0\xB6\xB3\xA7d\0\0\x85a)KV[a \xE7\x91\x90a){V[\x90P`\0a \xF4\x82a)\xA9V[\x90P`\0a!\x01\x82a$\xB0V[\x90Pg\x1B\xC1mgN\xC8\0\0a!\x1Eg\r\xE0\xB6\xB3\xA7d\0\0\x83a)KV[a\x1D\x08\x91\x90a){V[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a!@W`\0\x80\xFD[\x04\x92\x91PPV[`\0\x80\x82\x12\x80a!^WPg\x1B\xC1mgN\xC8\0\0\x82\x13[\x15a!|W`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a!\x9DW`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81g\x1B\xC1mgN\xC8\0\0\x03a!\xC5W`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x15a!\xD0W\x91\x90PV[`\0g\r\xE0\xB6\xB3\xA7d\0\0\x83\x12a!\xF8Wa!\xF3\x83g\x1B\xC1mgN\xC8\0\0a(\xFCV[a!\xFAV[\x82[\x90P`\0a\"\x10\x82g\x1B\xC1mgN\xC8\0\0a&\x94V[\x90P\x80`\0\x03a\"3W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\">\x82a\x1E\xE4V[\x90P`\0c;\x9A\xCA\0a\"ia\"da\"^g\x1B\xC1mgN\xC8\0\0a)\xA9V[\x85a#\xC5V[a\x1B\xB2V[a\"s\x91\x90a)KV[\x90P`\0\x80a\"\x8A\x83g\x03\xC1f\\z\xAB \0a#\xC5V[a\"\x9C\x90g \x05\xFEO&\x8E\xA0\0a)#V[\x90P`\0a\"\xCC\x84a\"\xB5\x86f\x9F2u$b\xA0\0a#\xC5V[a\"\xC7\x90g\r\xC5R\x7Fd, \0a)#V[a#\xC5V[a\"\xDE\x90g\r\xE0\xB6\xB3\xA7d\0\0a)#V[\x90Pa#\x02g\t\xD0(\xCCo _\xFF\x19\x85a\"\xF8\x85\x85a&\x94V[a\"\xC7\x91\x90a(\xFCV[\x92PPP`\0[`\x02\x81\x10\x15a#\x9DW`\0\x86a#\x1E\x84a$\xB0V[a#(\x91\x90a(\xFCV[\x90P`\0a#6\x84\x85a#\xC5V[a#?\x90a)\xA9V[\x90P`\0a#L\x82a\x1D\x11V[\x90P`\0a#Z\x86\x85a#\xC5V[a#lg\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x84a#\xC5V[a#v\x91\x90a(\xFCV[\x90Pa#\x82\x84\x82a&\x94V[a#\x8C\x90\x87a)#V[\x95P\x84`\x01\x01\x94PPPPPa#\tV[g\r\xE0\xB6\xB3\xA7d\0\0\x88\x12a#\xBAWa#\xB5\x82a)\xA9V[a\x1A\x85V[P\x96\x95PPPPPPV[`\0a\x1B\x94\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a&\xA5V[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a#\xF2W`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[`\0\x80\x82\x11a$EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x07!V[P`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x82\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[`\0\x81`\0\x03a$\xC9WPg\r\xE0\xB6\xB3\xA7d\0\0\x91\x90PV[gV\x98\xEE\xF0fp\0\0\x82\x12a$\xE0WP`\0\x91\x90PV[a$\xF1gV\x98\xEE\xF0fp\0\0a)\xA9V[\x82\x13a%\x06WPg\x1B\xC1mgN\xC8\0\0\x91\x90PV[`\0a%\x11\x83a&\xC4V[\x90P`\0a%Jg\r\xE0\xB6\xB3\xA7d\0\0a%3\x84g\x1B\xC1mgN\xC8\0\0a\x1CVV[a%E\x90g\r\xE0\xB6\xB3\xA7d\0\0a)#V[a&\x94V[\x90P`\0\x80\x82a%\xA6\x81a%\x93\x81a%\x81\x81a%n\x81g\x02_\x0F\xE1\x05\xA3\x14\0a#\xC5V[a\"\xC7\x90g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19a)#V[a\"\xC7\x90g\x14\xA8EL\x19\xE1\xAC\0a)#V[a\"\xC7\x90g\x0F\xC1\x0E\x01W\x82w\xFF\x19a)#V[a%\xB8\x90g\x03\xDE\xBD\x08;\x8C|\0a)#V[\x91P\x83\x90Pa& \x81a&\x0E\x81a%\xFC\x81a%\xEA\x81a%\xD7\x81\x8Ba#\xC5V[a\"\xC7\x90g\x02\x95\xD4\0\xEA2W\xFF\x19a)#V[a\"\xC7\x90g\x01W\xD8\xB2\xEC\xC7\x08\0a)#V[a\"\xC7\x90g\x051\n\xA7\xD5!0\0a)#V[a\"\xC7\x90g\r\xE0\xCC=\x15a\0\0a)#V[\x91P\x81g\x11\x90\0\xAB\x10\x0F\xFC\0a&6\x87\x88a#\xC5V[a&B\x90`\0\x19a)KV[a&L\x91\x90a(\xFCV[a&V\x91\x90a)#V[\x92PP`\0a&d\x83a\x1D\x11V[\x90P`\0a&r\x85\x83a#\xC5V[\x90P`\0\x88\x12a&\x82W\x80a\x1A\x85V[a\x1A\x85\x81g\x1B\xC1mgN\xC8\0\0a(\xFCV[`\0a\x1B\x94\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x05\x85\x14\x17\x16a&\xBDW`\0\x80\xFD[\x05\x92\x91PPV[`\0`\x01`\xFF\x1B\x82\x03a&\xEAW`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x12\x15a&\xFBWP\x19`\x01\x01\x90V[P\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`\0` \x82\x84\x03\x12\x15a'dWa'da&\xFFV[P5\x91\x90PV[\x80\x15\x15\x81\x14a'yW`\0\x80\xFD[PV[`\0\x80`\0``\x84\x86\x03\x12\x15a'\x94Wa'\x94a&\xFFV[\x835a'\x9F\x81a'kV[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a'\xC9Wa'\xC9a&\xFFV[\x815`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a'\xE0W`\0\x80\xFD[\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a'\xFDWa'\xFDa&\xFFV[\x825a(\x08\x81a'kV[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`@\x83\x85\x03\x12\x15a(,Wa(,a&\xFFV[PP\x805\x92` \x90\x91\x015\x91PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x1B\x97Wa\x1B\x97a(;V[\x81\x81\x03\x81\x81\x11\x15a\x1B\x97Wa\x1B\x97a(;V[`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x81R\x91\x90\x92\x16` \x82\x01R`@\x81\x01\x91\x90\x91R``\x01\x90V[`\0` \x82\x84\x03\x12\x15a(\xB0Wa(\xB0a&\xFFV[\x81Qa'\xE0\x81a'kV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x1B\x97Wa\x1B\x97a(;V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a(\xF7Wa(\xF7a(\xD2V[P\x04\x90V[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a)\x1CWa)\x1Ca(;V[P\x92\x91PPV[\x80\x82\x01\x82\x81\x12`\0\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a)CWa)Ca(;V[PP\x92\x91PPV[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15a)gWa)ga(;V[\x81\x81\x05\x83\x14\x82\x15\x17a\x1B\x97Wa\x1B\x97a(;V[`\0\x82a)\x8AWa)\x8Aa(\xD2V[`\x01`\xFF\x1B\x82\x14`\0\x19\x84\x14\x16\x15a)\xA4Wa)\xA4a(;V[P\x05\x90V[`\0`\x01`\xFF\x1B\x82\x01a)\xBEWa)\xBEa(;V[P`\0\x03\x90V\xFETarget contract does not contain\xA2dipfsX\"\x12 Hk\xB0[F\xE2\xC7s\"G\x95Wc\xD0\xC7\x84\xDA\xA4e\x10*b\xCC+\x8A]a\xC4\xB1\x80n?dsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static RMM_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x01cW`\x005`\xE0\x1C\x80c\xA5\x9C\x18o\x11a\0\xEFW\x80c\xC5)\x87\xCF\x11a\0\xBEW\x80c\xC5)\x87\xCF\x14a\x03;W\x80c\xCC\xD1\xE4\xBE\x14a\x03bW\x80c\xCF\xC4\xAFU\x14a\x03uW\x80c\xF9\xA1\xC8Z\x14a\x03\x9CW\x80c\xFA\xDF\xA6[\x14a\x03\xAFWa\x01cV[\x80c\xA5\x9C\x18o\x14a\x02\xC7W\x80c\xAF\xDF1\xCD\x14a\x02\xDAW\x80c\xB179)\x14a\x03\x01W\x80c\xB7\xD1\x9F\xC4\x14a\x03(Wa\x01cV[\x80c\x16\xDC\x16[\x11a\x016W\x80c\x16\xDC\x16[\x14a\x02CW\x80cp\xA0\x821\x14a\x02nW\x80cvp\x166\x14a\x02\x8EW\x80c\x94\xB9\x18\xDE\x14a\x02\xA1W\x80c\x9C\x9D\xA9\xEA\x14a\x02\xB4Wa\x01cV[\x80c\x02\xC2\xE5]\x14a\x01\xC8W\x80c\x04\xAF\xA8\"\x14a\x01\xF5W\x80c\x08\xEA\xBD\xDA\x14a\x02#W\x80c\x15w\x0F\x92\x14a\x02:W[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x01\xDBa\x01\xD66`\x04a'OV[a\x03\xB8V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01[`@Q\x80\x91\x03\x90\xF3[a\x02\x08a\x02\x036`\x04a'|V[a\x06\xCEV[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x01\xECV[a\x02,`\x02T\x81V[`@Q\x90\x81R` \x01a\x01\xECV[a\x02,`\x04T\x81V[`\0Ta\x02V\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\xECV[a\x02,a\x02|6`\x04a'\xB4V[`\x05` R`\0\x90\x81R`@\x90 T\x81V[a\x02\x08a\x02\x9C6`\x04a'\xE7V[a\n%V[a\x02,a\x02\xAF6`\x04a'OV[a\x0ENV[a\x01\xDBa\x02\xC26`\x04a'OV[a\x10\x1BV[a\x01\xDBa\x02\xD56`\x04a(\x16V[a\x12LV[a\x02,\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02,\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\x01Ta\x02V\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x02,\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01\xDBa\x03p6`\x04a'OV[a\x14\x90V[a\x02,\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01\xDBa\x03\xAA6`\x04a(\x16V[a\x16\xC2V[a\x02,`\x03T\x81V[`\0\x80`\0a\x04.`\x02T`\x04T\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x18jV[\x90P`\0a\x04\x8B\x85`\x02Ta\x04C\x91\x90a(QV[\x83\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x19\x14V[\x90P`\0a\x04\xDB\x82\x84\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x19\xA0V[\x90P`\0`\x03T\x82a\x04\xED\x91\x90a(dV[\x90P`\0`\x04T\x84a\x04\xFF\x91\x90a(dV[\x90P\x83`\x04\x81\x90UP\x87`\x02`\0\x82\x82Ta\x05\x1A\x91\x90a(QV[\x92PP\x81\x90UP\x81`\x03`\0\x82\x82Ta\x053\x91\x90a(QV[\x90\x91UPP`\0T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\x05l\x903\x900\x90\x8D\x90`\x04\x01a(wV[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x05\xC4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a)\xC6\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x05\xD8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xFC\x91\x90a(\x9BV[P`\x01T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\x061\x903\x900\x90\x87\x90`\x04\x01a(wV[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x06\x89W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a)\xC6\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x06\x9DW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xC1\x91\x90a(\x9BV[P\x97\x90\x96P\x94PPPPPV[`\0\x80`\0`\x04T`\0\x14a\x07*W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FPool already initialized\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[\x85\x15a\x07\xD4W\x84\x92Pa\x07\x7F\x83\x85\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x19\x14V[\x90Pa\x07\xCD\x81\x85\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x19\xA0V[\x91Pa\x08tV[\x84\x91Pa\x08#\x82\x85\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1A,V[\x90Pa\x08q\x81\x85\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1A\x91V[\x92P[`\x04\x81\x81U`\x02\x84\x90U`\x03\x83\x90U3`\0\x81\x81R`\x05` R`@\x80\x82 \x85\x90U\x90T\x90Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x92c#\xB8r\xDD\x92a\x08\xC6\x92\x90\x910\x91\x89\x91\x01a(wV[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\t\x1EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a)\xC6\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\t2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\tV\x91\x90a(\x9BV[P`\x01T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\t\x8B\x903\x900\x90\x87\x90`\x04\x01a(wV[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\t\xE3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a)\xC6\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\t\xF7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\x1B\x91\x90a(\x9BV[P\x93P\x93P\x93\x90PV[`\0\x80`\0\x80`\x04T\x11a\nrW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rs\x14\x1B\xDB\xDB\x08\x1B\x9B\xDD\x08\x1A[\x9A]\x1AX[\x1A^\x99Y`b\x1B`D\x82\x01R`d\x01a\x07!V[`\0a\n\xE5`\x02T`\x04T\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x18jV[\x90P\x85\x15a\x0B|W\x84\x93P`\0a\x0B\x03\x85`\x02Ta\x04C\x91\x90a(QV[\x90P`\0a\x0BS\x82\x84\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x19\xA0V[\x90P`\x03T\x81a\x0Bc\x91\x90a(dV[\x94P`\x04T\x82a\x0Bs\x91\x90a(dV[\x93PPPa\x0COV[\x84\x92P`\0a\x0B\xDA\x84`\x03Ta\x0B\x92\x91\x90a(QV[\x83\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1A,V[\x90P`\0a\x0C*\x82\x84\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1A\x91V[\x90P`\x02T\x81a\x0C:\x91\x90a(dV[\x95P`\x04T\x82a\x0CJ\x91\x90a(dV[\x93PPP[\x81`\x04`\0\x82\x82Ta\x0Ca\x91\x90a(QV[\x92PP\x81\x90UP\x83`\x02`\0\x82\x82Ta\x0Cz\x91\x90a(QV[\x92PP\x81\x90UP\x82`\x03`\0\x82\x82Ta\x0C\x93\x91\x90a(QV[\x90\x91UPP3`\0\x90\x81R`\x05` R`@\x81 \x80T\x84\x92\x90a\x0C\xB7\x90\x84\x90a(QV[\x90\x91UPP`\0T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\x0C\xF0\x903\x900\x90\x89\x90`\x04\x01a(wV[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\rHW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a)\xC6\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\r\\W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\x80\x91\x90a(\x9BV[P`\x01T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\r\xB5\x903\x900\x90\x88\x90`\x04\x01a(wV[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0E\rW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a)\xC6\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x0E!W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0EE\x91\x90a(\x9BV[PP\x92P\x92P\x92V[`\0\x80a'\x10a\x0E~\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85a(\xBBV[a\x0E\x88\x91\x90a(\xE8V[\x90P`\0a\x0E\x96\x82\x85a(dV[\x90P`\0a\x0F\x0B`\x02T`\x04T\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x18jV[\x90P`\0a\x0F[\x83\x83\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x19\x14V[\x90P`\0a\x0F\xAB\x82\x84\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x19\xA0V[\x90P`\x01a\x10\x05`\x02T\x89`\x03T\x85`\x04T\x88\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1A\xEDV[a\x10\x0F\x91\x90a(\xFCV[\x19\x97\x96PPPPPPPV[`\0\x80`\0a\x10\x91`\x02T`\x04T\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x18jV[\x90P`\0a\x10\xA6\x85`\x03Ta\x0B\x92\x91\x90a(QV[\x90P`\0a\x10\xF6\x82\x84\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1A\x91V[\x90P`\0`\x02T\x82a\x11\x08\x91\x90a(dV[\x90P`\0`\x04T\x84a\x11\x1A\x91\x90a(dV[\x90P\x83`\x04\x81\x90UP\x81`\x02`\0\x82\x82Ta\x115\x91\x90a(QV[\x92PP\x81\x90UP\x87`\x03`\0\x82\x82Ta\x11N\x91\x90a(QV[\x90\x91UPP`\0T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\x11\x87\x903\x900\x90\x87\x90`\x04\x01a(wV[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x11\xDFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a)\xC6\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x11\xF3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\x17\x91\x90a(\x9BV[P`\x01T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\x061\x903\x900\x90\x8D\x90`\x04\x01a(wV[`\0\x80`\0a\x12\x9D\x85\x85\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1A,V[\x90P`\0a\x12\xED\x82\x86\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1A\x91V[`\x04\x83\x81U`\x02\x82\x90U`\x03\x88\x90U`\0T`@Qc#\xB8r\xDD`\xE0\x1B\x81R\x92\x93P`\x01`\x01`\xA0\x1B\x03\x16\x91c#\xB8r\xDD\x91a\x13/\x913\x910\x91\x87\x91\x01a(wV[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x13\x87W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a)\xC6\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x13\x9BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\xBF\x91\x90a(\x9BV[P`\x01T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\x13\xF4\x903\x900\x90\x8B\x90`\x04\x01a(wV[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x14LW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a)\xC6\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x14`W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14\x84\x91\x90a(\x9BV[P\x90\x95\x90\x94P\x92PPPV[`\0\x80`\0a\x15\x06`\x02T`\x04T\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x18jV[\x90P`\0a\x15\x1B\x85`\x02Ta\x04C\x91\x90a(dV[\x90P`\0a\x15k\x82\x84\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x19\xA0V[\x90P`\0\x81`\x03Ta\x15}\x91\x90a(dV[\x90P`\0\x83`\x04Ta\x15\x8F\x91\x90a(dV[\x90P\x83`\x04\x81\x90UP\x87`\x02`\0\x82\x82Ta\x15\xAA\x91\x90a(dV[\x92PP\x81\x90UP\x81`\x03`\0\x82\x82Ta\x15\xC3\x91\x90a(dV[\x90\x91UPP`\0T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x8A\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x16RW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a)\xC6\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x16fW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\x8A\x91\x90a(\x9BV[P`\x01T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01a\x061V[`\0\x80`\0a\x17\x13\x85\x85\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x19\x14V[\x90P`\0a\x17c\x82\x86\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x19\xA0V[`\x04\x83\x81U`\x02\x88\x90U`\x03\x82\x90U`\0T`@Qc#\xB8r\xDD`\xE0\x1B\x81R\x92\x93P`\x01`\x01`\xA0\x1B\x03\x16\x91c#\xB8r\xDD\x91a\x17\xA5\x913\x910\x91\x8C\x91\x01a(wV[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x17\xFDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a)\xC6\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x18\x11W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x185\x91\x90a(\x9BV[P`\x01T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\x13\xF4\x903\x900\x90\x86\x90`\x04\x01a(wV[`\0\x80a\x18\x99g\x06\xF0[Y\xD3\xB2\0\0a\x18\x94a\x18\x8E\x87g\x1B\xC1mgN\xC8\0\0a\x1BhV[\x86a\x1B\x9DV[a\x1B\x9DV[\x90P`\0a\x18\xAA\x85a\x18\x94\x86a\x1B\xB2V[a\x18\xB8\x90c;\x9A\xCA\0a(\xBBV[\x90P`\0a\x18\xC6\x89\x89a\x1CVV[\x90Pa\x19\x07\x87a\x19\x02\x85a\x18\xF3a\x18\xEDa\x18\xE8\x87g\r\xE0\xB6\xB3\xA7d\0\0a(dV[a\x1CkV[\x87a\x1B\x9DV[a\x18\xFD\x91\x90a(dV[a\x1D\x11V[a\x1E\xBAV[\x99\x98PPPPPPPPPV[`\0\x80a\x19)a\x19$\x86\x86a\x1E\xCFV[a\x1E\xE4V[\x90P`\0a\x19Kg\x06\xF0[Y\xD3\xB2\0\0a\x19\x02\x86g\x1B\xC1mgN\xC8\0\0a\x1BhV[\x90P`\0a\x19~\x85a\x19]\x84\x86a)#V[a\x19o\x90g\r\xE0\xB6\xB3\xA7d\0\0a)KV[a\x19y\x91\x90a){V[a \xBFV[\x90P`\0a\x19\x94\x82g\r\xE0\xB6\xB3\xA7d\0\0a(\xFCV[\x90Pa\x19\x07\x89\x82a\x1E\xCFV[`\0\x80a\x19\xB0a\x19$\x86\x86a\x1E\xCFV[\x90P`\0a\x19\xD2g\x06\xF0[Y\xD3\xB2\0\0a\x19\x02\x86g\x1B\xC1mgN\xC8\0\0a\x1BhV[\x90P`\0a\x19\xE0\x82\x84a(\xFCV[\x90P`\0\x85a\x19\xF7\x83g\r\xE0\xB6\xB3\xA7d\0\0a)KV[a\x1A\x01\x91\x90a){V[\x90P`\0a\x1A\x0E\x82a \xBFV[\x90Pa\x1A\x1E\x88a\x19\x02\x8C\x84a\x1E\xBAV[\x9A\x99PPPPPPPPPPV[`\0\x80a\x1A<a\x19$\x86\x86a\x1E\xCFV[\x90P`\0a\x1A^g\x06\xF0[Y\xD3\xB2\0\0a\x19\x02\x86g\x1B\xC1mgN\xC8\0\0a\x1BhV[\x90P`\0a\x1Ap\x85a\x19]\x84\x86a(\xFCV[\x90Pa\x1A\x85\x88a\x1A\x80\x88\x84a\x1E\xBAV[a\x1E\xCFV[\x98\x97PPPPPPPPV[`\0\x80a\x1A\xA1a\x19$\x86\x86a\x1E\xCFV[\x90P`\0a\x1A\xC3g\x06\xF0[Y\xD3\xB2\0\0a\x19\x02\x86g\x1B\xC1mgN\xC8\0\0a\x1BhV[\x90P`\0a\x1A\xD5\x85a\x19]\x84\x86a)#V[\x90Pa\x1A\x85\x88a\x19\x02\x83g\r\xE0\xB6\xB3\xA7d\0\0a(\xFCV[`\0\x80a\x1A\xFE\x84a\x18\x94\x87\x89a(QV[\x90P`\0a\x1B7a\x1B$a\x18\xE8a\x1B\x15\x8D\x8Fa(QV[a\x1B\x1F\x8A\x8Ca(QV[a\x1CVV[a\x1B-\x86a)\xA9V[a\x19y\x91\x90a(\xFCV[\x90P\x87\x89a\x1BE\x84\x84a\x1B\x9DV[a\x1BO\x91\x90a(\xFCV[a\x1BY\x91\x90a(\xFCV[\x9B\x9APPPPPPPPPPPV[`\0a\x1B\x94g\r\xE0\xB6\xB3\xA7d\0\0\x83a\x1B\x80\x86a\x1E\xE4V[a\x1B\x8A\x91\x90a)KV[a\x18\xFD\x91\x90a){V[\x90P[\x92\x91PPV[`\0a\x1B\x94\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a!(V[`\xB5\x81`\x01`\x88\x1B\x81\x10a\x1B\xCBW`@\x91\x90\x91\x1B\x90`\x80\x1C[i\x01\0\0\0\0\0\0\0\0\0\x81\x10a\x1B\xE7W` \x91\x90\x91\x1B\x90`@\x1C[e\x01\0\0\0\0\0\x81\x10a\x1B\xFFW`\x10\x91\x90\x91\x1B\x90` \x1C[c\x01\0\0\0\x81\x10a\x1C\x15W`\x08\x91\x90\x91\x1B\x90`\x10\x1C[b\x01\0\0\x01\x02`\x12\x1C\x80\x82\x04\x01`\x01\x90\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x90\x81\x90\x04\x81\x11\x90\x03\x90V[`\0a\x1B\x94\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a!(V[`\0g\x06\xF0[Y\xD3\xB2\0\0\x82\x03a\x1C\x84WP`\0\x91\x90PV[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x12a\x1C\xACW`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\x1C\xCDW`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x1C\xDA\x83`\x02a)KV[\x90P`\0a\x1C\xE7\x82a!GV[\x90P`\0a\x1C\xFDg\x13\xA0K\xBD\xFD\xC9\xBE\x88\x83a#\xC5V[\x90Pa\x1D\x08\x81a)\xA9V[\x95\x94PPPPPV[`\0h\x02H\xCE6\xA7\x0C\xB2k>\x19\x82\x13a\x1D,WP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a\x1DsW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x01a\x07!V[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05`\x01`_\x1B\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1Dm\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1Dn\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[`\0a\x1B\x94\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a#\xDAV[`\0a\x1B\x94\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a#\xDAV[`\0\x80\x82\x13a\x1F!W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x07!V[`\0``a\x1F.\x84a$\x08V[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[`\0\x80g\x13\xA0K\xBD\xFD\xC9\xBE\x88a \xDDg\r\xE0\xB6\xB3\xA7d\0\0\x85a)KV[a \xE7\x91\x90a){V[\x90P`\0a \xF4\x82a)\xA9V[\x90P`\0a!\x01\x82a$\xB0V[\x90Pg\x1B\xC1mgN\xC8\0\0a!\x1Eg\r\xE0\xB6\xB3\xA7d\0\0\x83a)KV[a\x1D\x08\x91\x90a){V[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a!@W`\0\x80\xFD[\x04\x92\x91PPV[`\0\x80\x82\x12\x80a!^WPg\x1B\xC1mgN\xC8\0\0\x82\x13[\x15a!|W`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a!\x9DW`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81g\x1B\xC1mgN\xC8\0\0\x03a!\xC5W`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x15a!\xD0W\x91\x90PV[`\0g\r\xE0\xB6\xB3\xA7d\0\0\x83\x12a!\xF8Wa!\xF3\x83g\x1B\xC1mgN\xC8\0\0a(\xFCV[a!\xFAV[\x82[\x90P`\0a\"\x10\x82g\x1B\xC1mgN\xC8\0\0a&\x94V[\x90P\x80`\0\x03a\"3W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\">\x82a\x1E\xE4V[\x90P`\0c;\x9A\xCA\0a\"ia\"da\"^g\x1B\xC1mgN\xC8\0\0a)\xA9V[\x85a#\xC5V[a\x1B\xB2V[a\"s\x91\x90a)KV[\x90P`\0\x80a\"\x8A\x83g\x03\xC1f\\z\xAB \0a#\xC5V[a\"\x9C\x90g \x05\xFEO&\x8E\xA0\0a)#V[\x90P`\0a\"\xCC\x84a\"\xB5\x86f\x9F2u$b\xA0\0a#\xC5V[a\"\xC7\x90g\r\xC5R\x7Fd, \0a)#V[a#\xC5V[a\"\xDE\x90g\r\xE0\xB6\xB3\xA7d\0\0a)#V[\x90Pa#\x02g\t\xD0(\xCCo _\xFF\x19\x85a\"\xF8\x85\x85a&\x94V[a\"\xC7\x91\x90a(\xFCV[\x92PPP`\0[`\x02\x81\x10\x15a#\x9DW`\0\x86a#\x1E\x84a$\xB0V[a#(\x91\x90a(\xFCV[\x90P`\0a#6\x84\x85a#\xC5V[a#?\x90a)\xA9V[\x90P`\0a#L\x82a\x1D\x11V[\x90P`\0a#Z\x86\x85a#\xC5V[a#lg\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x84a#\xC5V[a#v\x91\x90a(\xFCV[\x90Pa#\x82\x84\x82a&\x94V[a#\x8C\x90\x87a)#V[\x95P\x84`\x01\x01\x94PPPPPa#\tV[g\r\xE0\xB6\xB3\xA7d\0\0\x88\x12a#\xBAWa#\xB5\x82a)\xA9V[a\x1A\x85V[P\x96\x95PPPPPPV[`\0a\x1B\x94\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a&\xA5V[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a#\xF2W`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[`\0\x80\x82\x11a$EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x07!V[P`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x82\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[`\0\x81`\0\x03a$\xC9WPg\r\xE0\xB6\xB3\xA7d\0\0\x91\x90PV[gV\x98\xEE\xF0fp\0\0\x82\x12a$\xE0WP`\0\x91\x90PV[a$\xF1gV\x98\xEE\xF0fp\0\0a)\xA9V[\x82\x13a%\x06WPg\x1B\xC1mgN\xC8\0\0\x91\x90PV[`\0a%\x11\x83a&\xC4V[\x90P`\0a%Jg\r\xE0\xB6\xB3\xA7d\0\0a%3\x84g\x1B\xC1mgN\xC8\0\0a\x1CVV[a%E\x90g\r\xE0\xB6\xB3\xA7d\0\0a)#V[a&\x94V[\x90P`\0\x80\x82a%\xA6\x81a%\x93\x81a%\x81\x81a%n\x81g\x02_\x0F\xE1\x05\xA3\x14\0a#\xC5V[a\"\xC7\x90g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19a)#V[a\"\xC7\x90g\x14\xA8EL\x19\xE1\xAC\0a)#V[a\"\xC7\x90g\x0F\xC1\x0E\x01W\x82w\xFF\x19a)#V[a%\xB8\x90g\x03\xDE\xBD\x08;\x8C|\0a)#V[\x91P\x83\x90Pa& \x81a&\x0E\x81a%\xFC\x81a%\xEA\x81a%\xD7\x81\x8Ba#\xC5V[a\"\xC7\x90g\x02\x95\xD4\0\xEA2W\xFF\x19a)#V[a\"\xC7\x90g\x01W\xD8\xB2\xEC\xC7\x08\0a)#V[a\"\xC7\x90g\x051\n\xA7\xD5!0\0a)#V[a\"\xC7\x90g\r\xE0\xCC=\x15a\0\0a)#V[\x91P\x81g\x11\x90\0\xAB\x10\x0F\xFC\0a&6\x87\x88a#\xC5V[a&B\x90`\0\x19a)KV[a&L\x91\x90a(\xFCV[a&V\x91\x90a)#V[\x92PP`\0a&d\x83a\x1D\x11V[\x90P`\0a&r\x85\x83a#\xC5V[\x90P`\0\x88\x12a&\x82W\x80a\x1A\x85V[a\x1A\x85\x81g\x1B\xC1mgN\xC8\0\0a(\xFCV[`\0a\x1B\x94\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x05\x85\x14\x17\x16a&\xBDW`\0\x80\xFD[\x05\x92\x91PPV[`\0`\x01`\xFF\x1B\x82\x03a&\xEAW`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x12\x15a&\xFBWP\x19`\x01\x01\x90V[P\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`\0` \x82\x84\x03\x12\x15a'dWa'da&\xFFV[P5\x91\x90PV[\x80\x15\x15\x81\x14a'yW`\0\x80\xFD[PV[`\0\x80`\0``\x84\x86\x03\x12\x15a'\x94Wa'\x94a&\xFFV[\x835a'\x9F\x81a'kV[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a'\xC9Wa'\xC9a&\xFFV[\x815`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a'\xE0W`\0\x80\xFD[\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a'\xFDWa'\xFDa&\xFFV[\x825a(\x08\x81a'kV[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`@\x83\x85\x03\x12\x15a(,Wa(,a&\xFFV[PP\x805\x92` \x90\x91\x015\x91PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x1B\x97Wa\x1B\x97a(;V[\x81\x81\x03\x81\x81\x11\x15a\x1B\x97Wa\x1B\x97a(;V[`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x81R\x91\x90\x92\x16` \x82\x01R`@\x81\x01\x91\x90\x91R``\x01\x90V[`\0` \x82\x84\x03\x12\x15a(\xB0Wa(\xB0a&\xFFV[\x81Qa'\xE0\x81a'kV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x1B\x97Wa\x1B\x97a(;V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a(\xF7Wa(\xF7a(\xD2V[P\x04\x90V[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a)\x1CWa)\x1Ca(;V[P\x92\x91PPV[\x80\x82\x01\x82\x81\x12`\0\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a)CWa)Ca(;V[PP\x92\x91PPV[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15a)gWa)ga(;V[\x81\x81\x05\x83\x14\x82\x15\x17a\x1B\x97Wa\x1B\x97a(;V[`\0\x82a)\x8AWa)\x8Aa(\xD2V[`\x01`\xFF\x1B\x82\x14`\0\x19\x84\x14\x16\x15a)\xA4Wa)\xA4a(;V[P\x05\x90V[`\0`\x01`\xFF\x1B\x82\x01a)\xBEWa)\xBEa(;V[P`\0\x03\x90V\xFETarget contract does not contain\xA2dipfsX\"\x12 Hk\xB0[F\xE2\xC7s\"G\x95Wc\xD0\xC7\x84\xDA\xA4e\x10*b\xCC+\x8A]a\xC4\xB1\x80n?dsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static RMM_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
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
            f.debug_tuple(::core::stringify!(RMM))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> RMM<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                RMM_ABI.clone(),
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
        ///Calls the contract's `gamma` (0xb1373929) function
        pub fn gamma(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([177, 55, 57, 41], ())
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
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for RMM<M> {
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
            if let Ok(decoded) =
                <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <Infinity as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Infinity(decoded));
            }
            if let Ok(decoded) = <Min as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Min(decoded));
            }
            if let Ok(decoded) = <NegativeInfinity as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::NegativeInfinity(decoded));
            }
            if let Ok(decoded) = <OutOfBounds as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::OutOfBounds(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for RMMErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::Infinity(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Min(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NegativeInfinity(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OutOfBounds(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for RMMErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector == <Infinity as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <Min as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <NegativeInfinity as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector == <OutOfBounds as ::ethers::contract::EthError>::selector() => true,
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
        Hash,
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
        Hash,
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
        Hash,
    )]
    #[ethcall(name = "balanceOf", abi = "balanceOf(address)")]
    pub struct BalanceOfCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `gamma` function with signature `gamma()` and selector `0xb1373929`
    #[derive(
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
    #[ethcall(name = "gamma", abi = "gamma()")]
    pub struct GammaCall;
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
        Hash,
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
        Hash,
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
        Hash,
    )]
    #[ethcall(name = "removeLiquidityExactX", abi = "removeLiquidityExactX(uint256)")]
    pub struct RemoveLiquidityExactXCall {
        pub amount_x: ::ethers::core::types::U256,
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
        Hash,
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
        Hash,
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
        Hash,
    )]
    #[ethcall(name = "swap", abi = "swap(uint256)")]
    pub struct SwapCall {
        pub amount_x: ::ethers::core::types::U256,
    }
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
        Hash,
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
    pub enum RMMCalls {
        AddLiquidity(AddLiquidityCall),
        AddLiquidityExactX(AddLiquidityExactXCall),
        AddLiquidityExactY(AddLiquidityExactYCall),
        BalanceOf(BalanceOfCall),
        Gamma(GammaCall),
        InitExactX(InitExactXCall),
        InitExactY(InitExactYCall),
        InitPool(InitPoolCall),
        RemoveLiquidityExactX(RemoveLiquidityExactXCall),
        ReserveX(ReserveXCall),
        ReserveY(ReserveYCall),
        Sigma(SigmaCall),
        StrikePrice(StrikePriceCall),
        Swap(SwapCall),
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
            if let Ok(decoded) = <AddLiquidityCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AddLiquidity(decoded));
            }
            if let Ok(decoded) =
                <AddLiquidityExactXCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AddLiquidityExactX(decoded));
            }
            if let Ok(decoded) =
                <AddLiquidityExactYCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AddLiquidityExactY(decoded));
            }
            if let Ok(decoded) = <BalanceOfCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BalanceOf(decoded));
            }
            if let Ok(decoded) = <GammaCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Gamma(decoded));
            }
            if let Ok(decoded) = <InitExactXCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InitExactX(decoded));
            }
            if let Ok(decoded) = <InitExactYCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InitExactY(decoded));
            }
            if let Ok(decoded) = <InitPoolCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InitPool(decoded));
            }
            if let Ok(decoded) =
                <RemoveLiquidityExactXCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RemoveLiquidityExactX(decoded));
            }
            if let Ok(decoded) = <ReserveXCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ReserveX(decoded));
            }
            if let Ok(decoded) = <ReserveYCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ReserveY(decoded));
            }
            if let Ok(decoded) = <SigmaCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Sigma(decoded));
            }
            if let Ok(decoded) = <StrikePriceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::StrikePrice(decoded));
            }
            if let Ok(decoded) = <SwapCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Swap(decoded));
            }
            if let Ok(decoded) = <TauCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Tau(decoded));
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
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for RMMCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AddLiquidity(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AddLiquidityExactX(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddLiquidityExactY(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BalanceOf(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Gamma(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InitExactX(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InitExactY(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InitPool(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RemoveLiquidityExactX(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ReserveX(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ReserveY(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Sigma(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::StrikePrice(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Swap(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Tau(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TokenX(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TokenY(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TotalLiquidity(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for RMMCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddLiquidity(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddLiquidityExactX(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddLiquidityExactY(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::Gamma(element) => ::core::fmt::Display::fmt(element, f),
                Self::InitExactX(element) => ::core::fmt::Display::fmt(element, f),
                Self::InitExactY(element) => ::core::fmt::Display::fmt(element, f),
                Self::InitPool(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveLiquidityExactX(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReserveX(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReserveY(element) => ::core::fmt::Display::fmt(element, f),
                Self::Sigma(element) => ::core::fmt::Display::fmt(element, f),
                Self::StrikePrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::Swap(element) => ::core::fmt::Display::fmt(element, f),
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
    impl ::core::convert::From<GammaCall> for RMMCalls {
        fn from(value: GammaCall) -> Self {
            Self::Gamma(value)
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
    )]
    pub struct BalanceOfReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `gamma` function with signature `gamma()` and selector `0xb1373929`
    #[derive(
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
    pub struct GammaReturn(pub ::ethers::core::types::U256);
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
        Hash,
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
        Hash,
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
        Hash,
    )]
    pub struct RemoveLiquidityExactXReturn(
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
    )]
    pub struct SwapReturn {
        pub amount_y: ::ethers::core::types::U256,
    }
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
        Hash,
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
}
