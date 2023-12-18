pub use log_normal::*;
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
pub mod log_normal {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                    name: ::std::borrow::ToOwned::to_owned("swapFeePercentageWad_"),
                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                    internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                        "uint256"
                    ),),
                },],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("APPROXIMATED_MINIMUM_X_INPUT"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("APPROXIMATED_MINIMUM_X_INPUT",),
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
                    ::std::borrow::ToOwned::to_owned("BISECTION_EPSILON"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("BISECTION_EPSILON"),
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
                    ::std::borrow::ToOwned::to_owned("HALF_WAD"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("HALF_WAD"),
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
                    ::std::borrow::ToOwned::to_owned("INFINITY_IS_NOT_REAL"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("INFINITY_IS_NOT_REAL",),
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
                    ::std::borrow::ToOwned::to_owned("MAX_BISECTION_ITERS"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("MAX_BISECTION_ITERS",),
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
                    ::std::borrow::ToOwned::to_owned("TWO_WAD"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("TWO_WAD"),
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
                    ::std::borrow::ToOwned::to_owned("WAD"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("WAD"),
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
                    ::std::borrow::ToOwned::to_owned("ZERO"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("ZERO"),
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
                    ::std::borrow::ToOwned::to_owned("__slot__"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("__slot__"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("strikePriceWad"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("sigmaPercentWad"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("tauYearsWad"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("computeD1"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("computeD1"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("priceWad"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("params"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct Parameters"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("d1"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("int256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("computeD2"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("computeD2"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("priceWad"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("params"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct Parameters"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("d2"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("int256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("computeHalfSigmaSquared"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("computeHalfSigmaSquared",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("sigmaPercentWad"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
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
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("computePrice"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("computePrice"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("reserveXWad"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("totalLiquidity"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("strikePriceWad"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("sigmaPercentWad"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("tauYearsWad"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("price"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("computeSwapConstant"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("computeSwapConstant",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("data"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes"),
                            ),
                        },],
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
                    ::std::borrow::ToOwned::to_owned("dynamicSlot"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("dynamicSlot"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("params"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct Parameters"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("encodeInitData"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("encodeInitData"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("reserveXWad"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("reseveYWad"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("totalLiquidity"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("params"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct Parameters"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("encodeValidateData"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("encodeValidateData"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("adjustedReserveXWad",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("adjustedReserveYWad",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("adjustedLiquidity"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("findLiquidity"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("findLiquidity"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("reserveXWad"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("reserveYWad"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
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
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("params"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct Parameters"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("liquidity"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("findX"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("findX"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("reserveYWad"),
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
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("swapConstant"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("params"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct Parameters"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("reserveY"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("findY"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("findY"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("reserveXWad"),
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
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("swapConstant"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("params"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct Parameters"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("reserveY"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getNextLiquidity"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getNextLiquidity"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("reserveXWad"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("reserveYWad"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("totalLiquidity"),
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
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getParams"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getParams"),
                        inputs: ::std::vec![],
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
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("init"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("init"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("data"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes"),
                            ),
                        },],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("valid"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bool"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("swapConstantGrowth",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("reserveXWad"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("reserveYWad"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("totalLiquidity"),
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
                    ::std::borrow::ToOwned::to_owned("internalPrice"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("internalPrice"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("reserveXWad"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("totalLiquidity"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("price"),
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
                    ::std::borrow::ToOwned::to_owned("lx"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("lx"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("reserveXWad"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("priceWad"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("params"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct Parameters"),
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
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ly"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("ly"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("reserveYWad"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("priceWad"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("params"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct Parameters"),
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
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("mulidiv"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("mulidiv"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("x"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("y"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("denominator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("z"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("int256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("mulidivUp"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("mulidivUp"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("x"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("y"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("denominator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("z"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("int256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setSigma"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setSigma"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("newTargetSigma"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("newSigmaUpdateEnd"),
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
                    ::std::borrow::ToOwned::to_owned("setStrikePrice"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setStrikePrice"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("newTargetStrike"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("newStrikeUpdateEnd",),
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
                    ::std::borrow::ToOwned::to_owned("setTau"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setTau"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("newTargetTau"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("newTauUpdateEnd"),
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
                    ::std::borrow::ToOwned::to_owned("simulateSwap"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("simulateSwap"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("swapXIn"),
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
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("staticSlot"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("staticSlot"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct Parameters"),
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
                    ::std::borrow::ToOwned::to_owned("swapFeePercentageWad"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("swapFeePercentageWad",),
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
                    ::std::borrow::ToOwned::to_owned("targetSigma"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("targetSigma"),
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
                    ::std::borrow::ToOwned::to_owned("targetStrike"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("targetStrike"),
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
                    ::std::borrow::ToOwned::to_owned("toUint"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("toUint"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("x"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("int256"),
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
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("validate"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("validate"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("data"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes"),
                            ),
                        },],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("valid"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bool"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("swapConstantGrowth",),
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
                                name: ::std::borrow::ToOwned::to_owned("adjustedReserveXWad",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("adjustedReserveYWad",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("adjustedLiquidity"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("xl"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("xl"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("totalLiquidity"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("priceWad"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("params"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct Parameters"),
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
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("yl"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("yl"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("totalLiquidity"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("priceWad"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("params"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct Parameters"),
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
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
            ]),
            events: ::core::convert::From::from([(
                ::std::borrow::ToOwned::to_owned("LogParameters"),
                ::std::vec![::ethers::core::abi::ethabi::Event {
                    name: ::std::borrow::ToOwned::to_owned("LogParameters"),
                    inputs: ::std::vec![
                        ::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("sigma"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            indexed: false,
                        },
                        ::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("strikePrice"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            indexed: false,
                        },
                        ::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("tau"),
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
            )]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("BisectionLib_InvalidBounds"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("BisectionLib_InvalidBounds",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("lower"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("upper"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("BisectionLib_RootOutsideBounds"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("BisectionLib_RootOutsideBounds",),
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
                    },],
                ),
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
    pub static LOGNORMAL_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15b\0\0^W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`@Qb\x000v8\x03\x80b\x000v\x839\x81\x01`@\x81\x90Rb\0\0\x81\x91b\0\0\xF9V[g\r\xE0\xB6\xB3\xA7d\0\0\x81\x10b\0\0\xF0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7Fswap fee percentage must be less`D\x82\x01Ri than 100%`\xB0\x1B`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[`\0Ub\0\x01^V[`\0` \x82\x84\x03\x12\x15b\0\x01WW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[PQ\x91\x90PV[a/\x08\x80b\0\x01n`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x02\xB6W`\x005`\xE0\x1C\x80c\x8DR\xA1\xFC\x11a\x01\x9EW\x80c\xC1\xE0\x04;\x11a\x01\x10W\x80c\xDB\xAF\x11B\x11a\0\xD4W\x80c\xDB\xAF\x11B\x14a\x06*W\x80c\xE3\xD0\xDC\xA5\x14a\x06=W\x80c\xE4\x93t(\x14a\x06FW\x80c\xEC)\xD8\xE6\x14a\x06YW\x80c\xF9\xC2\x82\x11\x14a\x06lW\x80c\xFF\xB3bf\x14a\x06tWa\x02\xB6V[\x80c\xC1\xE0\x04;\x14a\x05\xECW\x80c\xC5)\x87\xCF\x14a\x05\xF4W\x80c\xCDd\xAE\xA2\x14a\x05\xFCW\x80c\xCF\xC4\xAFU\x14a\x06\x0FW\x80c\xD7\xF9{\xEF\x14a\x06\x17Wa\x02\xB6V[\x80c\xA6\xD34\x98\x11a\x01bW\x80c\xA6\xD34\x98\x14a\x055W\x80c\xAF\xDF1\xCD\x14a\x05HW\x80c\xB8\xF0\x0Br\x14a\x05PW\x80c\xBDB-(\x14a\x05cW\x80c\xC1e&\x12\x14a\x05\xA1W\x80c\xC1nP\xEF\x14a\x05\xAAWa\x02\xB6V[\x80c\x8DR\xA1\xFC\x14a\x04\xD6W\x80c\x97\x16\xAE\xBB\x14a\x04\xE9W\x80c\x99\x87\xFEd\x14a\x04\xFCW\x80c\x9D\x81\x9E\x83\x14a\x05\x0FW\x80c\xA1\x9C\xD3\xD1\x14a\x05\"Wa\x02\xB6V[\x80cV\x08\xBE\xA1\x11a\x027W\x80cj\x14`$\x11a\x01\xFBW\x80cj\x14`$\x14a\x04\x80W\x80cme\"\x99\x14a\x04\x8FW\x80c\x7F\x0EL\x8C\x14a\x04\x97W\x80c\x84\xC8*&\x14a\x04\xACW\x80c\x85\xAAEN\x14a\x04\xB4W\x80c\x88;m\xC5\x14a\x04\xC3Wa\x02\xB6V[\x80cV\x08\xBE\xA1\x14a\x04\x1DW\x80cX\xFAc\xCA\x14a\x040W\x80c^aZk\x14a\x048W\x80cd\x17\xD4\xB5\x14a\x04[W\x80cf\x8F\x90U\x14a\x04nWa\x02\xB6V[\x80c']g\xC8\x11a\x02~W\x80c']g\xC8\x14a\x03\xAAW\x80c.-yi\x14a\x03\xB9W\x80c>\x1E3\x92\x14a\x03\xCEW\x80c@\xB41i\x14a\x03\xD7W\x80cM\xDFG\xD4\x14a\x03\xE0Wa\x02\xB6V[\x80c\x06%\xA6#\x14a\x03\x1BW\x80c\n\xC304\x14a\x03AW\x80c\x0F\n\xA3\x95\x14a\x03dW\x80c\x18M0\xBA\x14a\x03\x84W\x80c\x1D\x9C\xF7\"\x14a\x03\x97W[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x03.a\x03)6`\x04a&\xC9V[a\x06\x87V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x03Ta\x03O6`\x04a'\x07V[a\x07sV[`@Qa\x038\x94\x93\x92\x91\x90a'\x81V[a\x03wa\x03r6`\x04a(\xB0V[a\x0B\x05V[`@Qa\x038\x91\x90a(\xF3V[a\x03.a\x03\x926`\x04a)\x06V[a\x0B7V[a\x03.a\x03\xA56`\x04a)?V[a\x0B\x89V[a\x03.g\x1B\xC1mgN\xC8\0\0\x81V[a\x03\xC1a\x0B\xD2V[`@Qa\x038\x91\x90a)[V[a\x03.`\nT\x81V[a\x03.`\0T\x81V[a\x03\xF3a\x03\xEE6`\x04a)\xD5V[a\x0C\x1BV[`@\x80Q\x95\x15\x15\x86R` \x86\x01\x94\x90\x94R\x92\x84\x01\x91\x90\x91R``\x83\x01R`\x80\x82\x01R`\xA0\x01a\x038V[a\x03.a\x04+6`\x04a)\x06V[a\x0C\x80V[a\x03.`\0\x81V[a\x04@a\x0C\xC1V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x038V[a\x03.a\x04i6`\x04a(\xB0V[a\x0C\xE9V[`\x01T`\x02T`\x03Ta\x04@\x92\x91\x90\x83V[a\x03.g\r\xE0\xB6\xB3\xA7d\0\0\x81V[a\x03.`\x01\x81V[a\x04\xAAa\x04\xA56`\x04a*\xFAV[a\rBV[\0[a\x03.`\n\x81V[a\x03.g\x06\xF0[Y\xD3\xB2\0\0\x81V[a\x03.a\x04\xD16`\x04a(\xB0V[a\x0E\x19V[a\x04\xAAa\x04\xE46`\x04a*\xFAV[a\x0EqV[a\x03.a\x04\xF76`\x04a+\x1FV[a\x0F\x0FV[a\x04\xAAa\x05\n6`\x04a*\xFAV[a\x0F\xACV[a\x03.a\x05\x1D6`\x04a)\x06V[a\x10JV[a\x03.a\x0506`\x04a+OV[a\x10\xDFV[a\x03.a\x05C6`\x04a)?V[a\x11\rV[a\x03.a\x116V[a\x03.a\x05^6`\x04a*\xFAV[a\x11\xA6V[a\x03wa\x05q6`\x04a,=V[`@\x80Q` \x81\x01\x94\x90\x94R\x83\x81\x01\x92\x90\x92R``\x80\x84\x01\x91\x90\x91R\x81Q\x80\x84\x03\x90\x91\x01\x81R`\x80\x90\x92\x01\x90R\x90V[a\x03.`\x05T\x81V[a\x05\xBDa\x05\xB86`\x04a+OV[a\x11\xD4V[`@\x80Q\x96\x15\x15\x87R` \x87\x01\x95\x90\x95R\x93\x85\x01\x92\x90\x92R``\x84\x01R`\x80\x83\x01R`\xA0\x82\x01R`\xC0\x01a\x038V[a\x03\xC1a\x14\x15V[a\x03.a\x14`V[a\x03.a\x06\n6`\x04a(\xB0V[a\x14\xCBV[a\x03.a\x15\x13V[a\x03.a\x06%6`\x04a+\x1FV[a\x15~V[a\x03.a\x0686`\x04a)\x06V[a\x15\xA6V[a\x03.`\0\x19\x81V[a\x03.a\x06T6`\x04a,=V[a\x15\xE8V[a\x03.a\x06g6`\x04a,=V[a\x16fV[a\x03.`\xFA\x81V[a\x03.a\x06\x826`\x04a,=V[a\x16\x9FV[`\0\x80a\x06\x94\x84\x84a\x17\x1EV[\x90P`\0a\x06\xA1\x85a\x11\rV[\x90P`\0a\x06\xAF\x82\x86a\x17SV[\x90P`\0a\x06\xBD\x8A\x8Aa\x17hV[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0\x81\x12a\x06\xDBW`\0\x94PPPPPa\x07jV[`\0\x81\x13a\x06\xF1W`\0\x19\x94PPPPPa\x07jV[`\0a\x07\ra\x07\x08\x83g\r\xE0\xB6\xB3\xA7d\0\0a,\x82V[a\x17}V[\x90P`\0\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x07%\x88\x85a,\xA9V[a\x07/\x91\x90a,\xEFV[a\x079\x91\x90a,\x82V[\x90P`\0a\x07F\x82a\x18\x1AV[\x90P`\0a\x07S\x82a\x0B\x89V[\x90Pa\x07_\x8C\x82a\x19\xC3V[\x98PPPPPPPPP[\x95\x94PPPPPV[`\0\x80`\0```\0\x80`\x003`\x01`\x01`\xA0\x1B\x03\x16c\xEB\xAD\xEF\x01`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01```@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x08\x08W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x08\x1CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08@\x91\x90a-\x1DV[\x92P\x92P\x92Pa\x08Q\x83\x83\x83a\x16fV[`@\x80Q` \x81\x01\x86\x90R\x90\x81\x01\x84\x90R``\x81\x01\x82\x90R\x90\x91P`\0\x90a\x08\x8B\x90`\x80\x01[`@Q` \x81\x83\x03\x03\x81R\x90`@Ra\x10\xDFV[\x90P`\0\x8A\x15a\t\xB3W`\0\x80Ta\x08\xA4\x90\x8C\x90a\x19\xC3V[\x90P`\0a\x08\xBC\x87a\x08\xB6\x84\x88a\x19\xC3V[\x90a\x19\xD8V[\x90Pa\x08\xC9`\x01\x82a-NV[\x90Pa\x08\xD5\x8C\x88a-NV[\x96Pa\x08\xE1\x81\x86a-NV[\x94P\x85a\x08\xF2\x88\x87\x87a\x04ia\x14\x15V[\x96Pa\x08\xFF`\x01\x88a-NV[\x96P\x80\x87\x10a\t`W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: y reserve increase`D\x82\x01Rad!`\xF0\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[a\tj\x87\x82a-aV[\x93Pa\t\xAB`@Q\x80`@\x01`@R\x80`\x1C\x81R` \x01\x7FEsimated Y reserve to submit\0\0\0\0\x81RP\x88a\x19\xEDV[PPPa\n\x83V[`\0\x80Ta\t\xC2\x90\x8C\x90a\x19\xC3V[\x90P`\0a\t\xD4\x86a\x08\xB6\x84\x88a\x19\xC3V[\x90Pa\t\xE1`\x01\x82a-NV[\x90Pa\t\xED\x8C\x87a-NV[\x95Pa\t\xF9\x81\x86a-NV[\x94P\x86a\n\n\x87\x87\x87a\x06\na\x14\x15V[\x97Pa\n\x17`\x01\x89a-NV[\x97P\x80\x88\x10a\nsW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: x reserve increase`D\x82\x01Rad!`\xF0\x1B`d\x82\x01R`\x84\x01a\tWV[a\n}\x88\x82a-aV[\x93PPPP[`@\x80Q` \x81\x01\x87\x90R\x90\x81\x01\x85\x90R``\x81\x01\x84\x90R`\0\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`\0a\n\xBD\x82a\x11\xD4V[PPPPP\x90P`\0a\n\xCEa\x14\x15V[\x90P\x81\x84a\n\xEB\x8A\x89\x85`\0\x01Q\x86` \x01Q\x87`@\x01Qa\x06\x87V[\x85\x9BP\x9BP\x9BP\x9BPPPPPPPPP\x92\x95\x91\x94P\x92PV[``\x84\x84\x84\x84`@Q` \x01a\x0B\x1E\x94\x93\x92\x91\x90a-tV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x94\x93PPPPV[`\0\x80a\x0BD\x84\x84a\x0F\x0FV[\x90P`\0a\x0BQ\x82a\x1A6V[\x90P`\0a\x0B^\x82a\x0B\x89V[\x90Pa\x0B|a\x0Bu\x82g\r\xE0\xB6\xB3\xA7d\0\0a-aV[\x88\x90a\x19\xC3V[\x93PPPP[\x93\x92PPPV[`\0\x80\x82\x12\x15a\x0B\xCEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01RotoUint: negative`\x80\x1B`D\x82\x01R`d\x01a\tWV[P\x90V[a\x0B\xF6`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[P`@\x80Q``\x81\x01\x82R`\x01T\x81R`\x02T` \x82\x01R`\x03T\x91\x81\x01\x91\x90\x91R\x90V[`\0\x80\x80\x80\x80a\x0C-\x86\x88\x01\x88a(\xB0V[\x80Q`\x01U` \x81\x01Q`\x02U`@\x01Q`\x03U\x91\x94P\x92P\x90Pa\x0CPa\x1A\x9FV[a\x0Cc\x83\x83\x83a\x0C^a\x14\x15V[a\x1A\xEEV[\x93Pa\x0Cq`\0`\x03a-\xA8V[\x84\x12\x15\x94P\x92\x95P\x92\x95\x90\x93PV[`\0\x80a\x0C\x8D\x84\x84a\x15~V[\x90P`\0a\x0C\x9A\x82a\x1A6V[\x90P`\0a\x0C\xA7\x82a\x0B\x89V[\x85Q\x90\x91Pa\x0B|\x90a\x0C\xBA\x90\x83a\x17SV[\x88\x90a\x17hV[`\0\x80`\0a\x0C\xCEa\x14`V[a\x0C\xD6a\x116V[a\x0C\xDEa\x15\x13V[\x92P\x92P\x92P\x90\x91\x92V[\x80Q`\0\x90`\n\x90\x82\x90a\x0C\xFE\x90\x83\x90a-aV[\x90Pa\r7\x87\x87\x87\x87`@Q` \x01a\r\x1A\x94\x93\x92\x91\x90a-tV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83\x83`\x01`\xFAa\x1B\x99a\x1B\xD4V[\x97\x96PPPPPPPV[B\x81\x11a\raW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\tW\x90a-\xD0V[a\ria\x1C\xE5V[`\0\x82`\x0ET\x11a\r\x86W`\x0ETa\r\x81\x90\x84a-aV[a\r\x94V[\x82`\x0ETa\r\x94\x91\x90a-aV[\x90Pa\r\xA0B\x83a-aV[a\r\xAA\x90\x82a-\xFBV[`\x11U`\x0F\x83\x90U`\x12\x82\x90U\x7Fs\xEBa\xB7\x83\x9B\xB3\xC27\xA0\xB3\xAE\xAB\x18iQ\0\xBCQ.q9\xF0\xE3@e]\xD7\x02\xDA*\xA1a\r\xE0a\x116V[a\r\xE8a\x14`V[a\r\xF0a\x15\x13V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01RB``\x82\x01R`\x80\x01`@Q\x80\x91\x03\x90\xA1PPPV[`\0\x80a\x0E'\x86`\x01a-NV[\x90P`\0k\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x90Pa\r7\x87\x87\x87\x87`@Q` \x01a\x0ET\x94\x93\x92\x91\x90a-tV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83\x83`\x01`\xFAa\x1C\xF6a\x1B\xD4V[B\x81\x11a\x0E\x90W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\tW\x90a-\xD0V[a\x0E\x98a\x1D&V[`\0\x82`\tT\x11a\x0E\xB5W`\tTa\x0E\xB0\x90\x84a-aV[a\x0E\xC3V[\x82`\tTa\x0E\xC3\x91\x90a-aV[\x90Pa\x0E\xCFB\x83a-aV[a\x0E\xD9\x90\x82a-\xFBV[`\x0CU`\n\x83\x90U`\r\x82\x90U\x7Fs\xEBa\xB7\x83\x9B\xB3\xC27\xA0\xB3\xAE\xAB\x18iQ\0\xBCQ.q9\xF0\xE3@e]\xD7\x02\xDA*\xA1a\r\xE0a\x116V[`\0\x80a\x0F$\x83` \x01Q\x84`@\x01Qa\x17\x1EV[\x90P`\0a\x0F5\x84` \x01Qa\x11\rV[\x90P`\0a\x0FXa\x0FS\x86`\0\x01Q\x88a\x19\xD8\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x1D7V[\x90P\x80a\x0Fd\x81a.\x0FV[\x91PP`\0a\x0F\x80\x86`@\x01Q\x84a\x17S\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x0F\x8A\x90\x83a-\xA8V[\x90P\x83a\x0F\xA0\x82g\r\xE0\xB6\xB3\xA7d\0\0\x83a\x16\x9FV[\x98\x97PPPPPPPPV[B\x81\x11a\x0F\xCBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\tW\x90a-\xD0V[a\x0F\xD3a\x1F\x12V[`\0\x82`\x04T\x11a\x0F\xF0W`\x04Ta\x0F\xEB\x90\x84a-aV[a\x0F\xFEV[\x82`\x04Ta\x0F\xFE\x91\x90a-aV[\x90Pa\x10\nB\x83a-aV[a\x10\x14\x90\x82a-\xFBV[`\x07U`\x05\x83\x90U`\x08\x82\x90U\x7Fs\xEBa\xB7\x83\x9B\xB3\xC27\xA0\xB3\xAE\xAB\x18iQ\0\xBCQ.q9\xF0\xE3@e]\xD7\x02\xDA*\xA1a\r\xE0a\x116V[`\0\x80a\x10W\x84\x84a\x0F\x0FV[\x90P`\0a\x10d\x82a\x1A6V[\x90P`\0a\x10q\x82a\x0B\x89V[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0\x81\x10a\x10\xCAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7Flx: denominator is zero\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\tWV[a\x0B|a\x0C\xBA\x82g\r\xE0\xB6\xB3\xA7d\0\0a-aV[`\0\x80`\0\x80\x84\x80` \x01\x90Q\x81\x01\x90a\x10\xF9\x91\x90a-\x1DV[\x92P\x92P\x92Pa\x07j\x83\x83\x83a\x0C^a\x14\x15V[`\0\x80a\x11\"\x83g\x1B\xC1mgN\xC8\0\0a\x1F#V[\x90Pa\x0B\x82g\x06\xF0[Y\xD3\xB2\0\0\x82a\x17SV[`\0`\x08TB\x10a\x11HWP`\x05T\x90V[`\x05T`\x04T\x11a\x11\x7FW`\x07T`\x06Ta\x11c\x90Ba-aV[a\x11m\x91\x90a..V[`\x04Ta\x11z\x91\x90a-NV[\x90P\x90V[`\x07T`\x06Ta\x11\x8F\x90Ba-aV[a\x11\x99\x91\x90a..V[`\x04Ta\x11z\x91\x90a-aV[`\0\x80a\x11\xB1a\x14\x15V[\x90Pa\x11\xCC\x84\x84\x83`\0\x01Q\x84` \x01Q\x85`@\x01Qa\x06\x87V[\x94\x93PPPPV[`\0\x80`\0\x80`\0\x80`\0\x80`\x003`\x01`\x01`\xA0\x1B\x03\x16c\xEB\xAD\xEF\x01`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01```@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x12kW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x12\x7FW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\xA3\x91\x90a-\x1DV[\x92P\x92P\x92Pa\x12\xB4\x83\x83\x83a\x16fV[\x90P\x89\x80` \x01\x90Q\x81\x01\x90a\x12\xCA\x91\x90a-\x1DV[\x91\x97P\x95P\x93P`\x01\x83\x87\x11\x15a\x13#W`\0a\x12\xE7\x85\x89a-aV[\x90P`\0a\x13\0`\0T\x83a\x19\xC3\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa\x13\x10\x86a\x08\xB6\x83\x87a\x19\xC3V[a\x13\x1A\x90\x84a-NV[\x92PPPa\x13\xC1V[\x82\x86\x11\x15a\x13`W`\0a\x137\x84\x88a-aV[\x90P`\0a\x13P`\0T\x83a\x19\xC3\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa\x13\x10\x85a\x08\xB6\x83\x87a\x19\xC3V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`0`$\x82\x01R\x7Finvalid swap: inputs x and y hav`D\x82\x01Roe the same sign!`\x80\x1B`d\x82\x01R`\x84\x01a\tWV[a\x13\xCB\x82\x86a,\x82V[\x97Pa\x13\xDB\x84\x84\x84a\x0C^a\x14\x15V[a\x13\xE9\x88\x88\x88a\x0C^a\x14\x15V[a\x13\xF3\x91\x90a,\x82V[\x98P`\0\x89\x12\x15\x80\x15a\x14\x06WP\x80\x88\x12\x15[\x99PPPPP\x91\x93\x95P\x91\x93\x95V[a\x149`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[a\x14Aa\x14`V[a\x14Ia\x116V[a\x14Qa\x15\x13V[`@\x84\x01R` \x83\x01R\x81R\x90V[`\0`\rTB\x10a\x14rWP`\nT\x90V[`\nT`\tT\x11a\x14\xA4W`\x0CT`\x0BTa\x14\x8D\x90Ba-aV[a\x14\x97\x91\x90a..V[`\tTa\x11z\x91\x90a-NV[`\x0CT`\x0BTa\x14\xB4\x90Ba-aV[a\x14\xBE\x91\x90a..V[`\tTa\x11z\x91\x90a-aV[`\0`\n\x81a\x14\xDA\x82\x87a-aV[\x90Pa\r7\x87\x87\x87\x87`@Q` \x01a\x14\xF6\x94\x93\x92\x91\x90a-tV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83\x83`\x01`\xFAa\x1FTa\x1B\xD4V[`\0`\x12TB\x10a\x15%WP`\x0FT\x90V[`\x0FT`\x0ET\x11a\x15WW`\x11T`\x10Ta\x15@\x90Ba-aV[a\x15J\x91\x90a..V[`\x0ETa\x11z\x91\x90a-NV[`\x11T`\x10Ta\x15g\x90Ba-aV[a\x15q\x91\x90a..V[`\x0ETa\x11z\x91\x90a-aV[`\0a\x15\x92\x82` \x01Q\x83`@\x01Qa\x17\x1EV[a\x15\x9C\x84\x84a\x0F\x0FV[a\x0B\x82\x91\x90a,\x82V[`\0\x80a\x15\xB3\x84\x84a\x15~V[\x90P`\0a\x15\xC0\x82a\x1A6V[\x90P`\0a\x15\xCD\x82a\x0B\x89V[\x85Q\x90\x91Pa\x0B|\x90\x82\x90a\x15\xE2\x90\x8Aa\x19\xC3V[\x90a\x19\xC3V[\x82\x82\x02\x81\x15\x80\x15\x90a\x16\x10WP\x83\x15\x80a\x16\x10WP\x82\x84\x82\x81a\x16\rWa\x16\ra,\xD9V[\x05\x14[a\x16NW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rn\x1B][\x1AY\x1A]\x88\x1A[\x9D\x98[\x1AY`\x8A\x1B`D\x82\x01R`d\x01a\tWV[\x81\x81\x81a\x16]Wa\x16]a,\xD9V[\x05\x94\x93PPPPV[`@\x80Q` \x81\x01\x85\x90R\x90\x81\x01\x83\x90R``\x81\x01\x82\x90R`\0\x90\x81\x90a\x16\x8F\x90`\x80\x01a\x08wV[\x90Pa\x07j\x85\x85\x83a\x04\xD1a\x14\x15V[`\0a\x16\xAC\x84\x84\x84a\x15\xE8V[\x90P\x81a\x16\xB9\x84\x86a,\xA9V[a\x16\xC3\x91\x90a.EV[\x15a\x0B\x82W`\x01`\x01`\xFF\x1B\x03\x81\x12a\x17\x13W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01RqmulidivUp overflow`p\x1B`D\x82\x01R`d\x01a\tWV[a\x11\xCC`\x01\x82a-\xA8V[`\0\x80a\x17*\x83a\x1F\x85V[\x90P`\0a\x17<c;\x9A\xCA\0\x83a..V[\x90Pa\x17H\x85\x82a\x17SV[\x92PPP[\x92\x91PPV[`\0a\x0B\x82\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a )V[`\0a\x0B\x82\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a )V[`\0g\x06\xF0[Y\xD3\xB2\0\0\x82\x03a\x17\x96WP`\0\x91\x90PV[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x12a\x17\xBEW`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\x17\xDFW`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x17\xEC\x83`\x02a,\xA9V[\x90P`\0a\x17\xF9\x82a HV[\x90P`\0a\x18\x0Fg\x13\xA0K\xBD\xFD\xC9\xBE\x88\x83a\"\xC6V[\x90Pa\x07j\x81a.YV[`\0h\x02H\xCE6\xA7\x0C\xB2k>\x19\x82\x13a\x185WP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a\x18|W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x01a\tWV[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05`\x01`_\x1B\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1Dm\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1Dn\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[`\0a\x0B\x82\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\"\xDBV[`\0a\x0B\x82\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\"\xDBV[a\x1A2\x82\x82`@Q`$\x01a\x1A\x03\x92\x91\x90a.uV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c-\x83\x9C\xB3`\xE2\x1B\x17\x90Ra#\tV[PPV[`\0\x80g\x13\xA0K\xBD\xFD\xC9\xBE\x88a\x1ATg\r\xE0\xB6\xB3\xA7d\0\0\x85a,\xA9V[a\x1A^\x91\x90a,\xEFV[\x90P`\0a\x1Ak\x82a.YV[\x90P`\0a\x1Ax\x82a#\x15V[\x90Pg\x1B\xC1mgN\xC8\0\0a\x1A\x95g\r\xE0\xB6\xB3\xA7d\0\0\x83a,\xA9V[a\x07j\x91\x90a,\xEFV[`\0a\x1A\xA9a\x0B\xD2V[` \x81\x01Q`\x05\x81\x90U`\x04UB`\x08\x81\x90U`\x06\x81\x90U\x81Q`\n\x81\x90U`\tU`\r\x81\x90U`\x0B\x81\x90U`@\x90\x91\x01Q`\x0F\x81\x90U`\x0EU`\x12\x81\x90U`\x10UPV[`\0\x82\x85\x10a\x1B?W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FtradingFunction: invalid x\0\0\0\0\0\0`D\x82\x01R`d\x01a\tWV[`\0a\x1BNa\x07\x08\x87\x86a\x17hV[\x90P`\0a\x1Bla\x07\x08\x87a\x1Bg\x87`\0\x01Q\x89a\x17SV[a\x17hV[\x90P`\0a\x1B\x82\x85` \x01Q\x86`@\x01Qa\x17\x1EV[\x90P\x80a\x1B\x8F\x83\x85a-\xA8V[a\x0F\xA0\x91\x90a-\xA8V[`\0\x80`\0\x80`\0\x86\x80` \x01\x90Q\x81\x01\x90a\x1B\xB5\x91\x90a.\x97V[\x93P\x93P\x93P\x93P\x81a\x1B\xCA\x85\x88\x86\x85a\x1A\xEEV[a\r7\x91\x90a,\x82V[`\0\x84\x86\x11\x15a\x1C\x01W`@Qc0\x82\xDF\xDB`\xE1\x1B\x81R`\x04\x81\x01\x87\x90R`$\x81\x01\x86\x90R`D\x01a\tWV[`\0a\x1C\x11\x88\x88\x85c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x1C#\x89\x88\x86c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x1C1\x82\x84a,\xA9V[\x13\x15a\x1CZW`@Qc\x06\xF1\xBE]`\xE2\x1B\x81R`\x04\x81\x01\x83\x90R`$\x81\x01\x82\x90R`D\x01a\tWV[`\0a\x1Cf\x89\x89a-aV[\x90P`\0[`\x02a\x1Cw\x8A\x8Ca-NV[a\x1C\x81\x91\x90a-\xFBV[\x94P`\0a\x1C\x93\x8C\x87\x89c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x1C\xA1\x86\x83a,\xA9V[\x13a\x1C\xAEW\x85\x99Pa\x1C\xB5V[\x85\x9AP\x80\x94P[a\x1C\xBF\x8B\x8Ba-aV[\x92PP`\x01\x01\x87\x82\x11\x80\x15a\x1C\xD3WP\x86\x81\x10[a\x1CkWPPPP\x96\x95PPPPPPV[a\x1C\xEDa\x15\x13V[`\x0EUB`\x10UV[`\0\x80`\0\x80`\0\x86\x80` \x01\x90Q\x81\x01\x90a\x1D\x12\x91\x90a.\x97V[\x93P\x93P\x93P\x93Pa\r7\x84\x84\x88\x84a\x1A\xEEV[a\x1D.a\x14`V[`\tUB`\x0BUV[`\0\x80\x82\x13a\x1DtW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\tWV[`\0``a\x1D\x81\x84a$\xF9V[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[a\x1F\x1Aa\x116V[`\x04UB`\x06UV[`\0a\x0B\x82g\r\xE0\xB6\xB3\xA7d\0\0\x83a\x1F;\x86a\x1D7V[a\x1FE\x91\x90a,\xA9V[a\x1FO\x91\x90a,\xEFV[a\x18\x1AV[`\0\x80`\0\x80`\0\x86\x80` \x01\x90Q\x81\x01\x90a\x1Fp\x91\x90a.\x97V[\x93P\x93P\x93P\x93P\x81a\x1B\xCA\x87\x86\x86\x85a\x1A\xEEV[`\xB5\x81`\x01`\x88\x1B\x81\x10a\x1F\x9EW`@\x91\x90\x91\x1B\x90`\x80\x1C[i\x01\0\0\0\0\0\0\0\0\0\x81\x10a\x1F\xBAW` \x91\x90\x91\x1B\x90`@\x1C[e\x01\0\0\0\0\0\x81\x10a\x1F\xD2W`\x10\x91\x90\x91\x1B\x90` \x1C[c\x01\0\0\0\x81\x10a\x1F\xE8W`\x08\x91\x90\x91\x1B\x90`\x10\x1C[b\x01\0\0\x01\x02`\x12\x1C\x80\x82\x04\x01`\x01\x90\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x90\x81\x90\x04\x81\x11\x90\x03\x90V[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a AW`\0\x80\xFD[\x04\x92\x91PPV[`\0\x80\x82\x12\x80a _WPg\x1B\xC1mgN\xC8\0\0\x82\x13[\x15a }W`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a \x9EW`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81g\x1B\xC1mgN\xC8\0\0\x03a \xC6W`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x15a \xD1W\x91\x90PV[`\0g\r\xE0\xB6\xB3\xA7d\0\0\x83\x12a \xF9Wa \xF4\x83g\x1B\xC1mgN\xC8\0\0a,\x82V[a \xFBV[\x82[\x90P`\0a!\x11\x82g\x1B\xC1mgN\xC8\0\0a%\xA1V[\x90P\x80`\0\x03a!4W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a!?\x82a\x1D7V[\x90P`\0c;\x9A\xCA\0a!ja!ea!_g\x1B\xC1mgN\xC8\0\0a.YV[\x85a\"\xC6V[a\x1F\x85V[a!t\x91\x90a,\xA9V[\x90P`\0\x80a!\x8B\x83g\x03\xC1f\\z\xAB \0a\"\xC6V[a!\x9D\x90g \x05\xFEO&\x8E\xA0\0a-\xA8V[\x90P`\0a!\xCD\x84a!\xB6\x86f\x9F2u$b\xA0\0a\"\xC6V[a!\xC8\x90g\r\xC5R\x7Fd, \0a-\xA8V[a\"\xC6V[a!\xDF\x90g\r\xE0\xB6\xB3\xA7d\0\0a-\xA8V[\x90Pa\"\x03g\t\xD0(\xCCo _\xFF\x19\x85a!\xF9\x85\x85a%\xA1V[a!\xC8\x91\x90a,\x82V[\x92PPP`\0[`\x02\x81\x10\x15a\"\x9EW`\0\x86a\"\x1F\x84a#\x15V[a\")\x91\x90a,\x82V[\x90P`\0a\"7\x84\x85a\"\xC6V[a\"@\x90a.YV[\x90P`\0a\"M\x82a\x18\x1AV[\x90P`\0a\"[\x86\x85a\"\xC6V[a\"mg\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x84a\"\xC6V[a\"w\x91\x90a,\x82V[\x90Pa\"\x83\x84\x82a%\xA1V[a\"\x8D\x90\x87a-\xA8V[\x95P\x84`\x01\x01\x94PPPPPa\"\nV[g\r\xE0\xB6\xB3\xA7d\0\0\x88\x12a\"\xBBWa\"\xB6\x82a.YV[a\x0F\xA0V[P\x96\x95PPPPPPV[`\0a\x0B\x82\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a%\xB2V[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\"\xF3W`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[a#\x12\x81a%\xD1V[PV[`\0\x81`\0\x03a#.WPg\r\xE0\xB6\xB3\xA7d\0\0\x91\x90PV[gV\x98\xEE\xF0fp\0\0\x82\x12a#EWP`\0\x91\x90PV[a#VgV\x98\xEE\xF0fp\0\0a.YV[\x82\x13a#kWPg\x1B\xC1mgN\xC8\0\0\x91\x90PV[`\0a#v\x83a%\xF2V[\x90P`\0a#\xAFg\r\xE0\xB6\xB3\xA7d\0\0a#\x98\x84g\x1B\xC1mgN\xC8\0\0a\x17hV[a#\xAA\x90g\r\xE0\xB6\xB3\xA7d\0\0a-\xA8V[a%\xA1V[\x90P`\0\x80\x82a$\x0B\x81a#\xF8\x81a#\xE6\x81a#\xD3\x81g\x02_\x0F\xE1\x05\xA3\x14\0a\"\xC6V[a!\xC8\x90g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19a-\xA8V[a!\xC8\x90g\x14\xA8EL\x19\xE1\xAC\0a-\xA8V[a!\xC8\x90g\x0F\xC1\x0E\x01W\x82w\xFF\x19a-\xA8V[a$\x1D\x90g\x03\xDE\xBD\x08;\x8C|\0a-\xA8V[\x91P\x83\x90Pa$\x85\x81a$s\x81a$a\x81a$O\x81a$<\x81\x8Ba\"\xC6V[a!\xC8\x90g\x02\x95\xD4\0\xEA2W\xFF\x19a-\xA8V[a!\xC8\x90g\x01W\xD8\xB2\xEC\xC7\x08\0a-\xA8V[a!\xC8\x90g\x051\n\xA7\xD5!0\0a-\xA8V[a!\xC8\x90g\r\xE0\xCC=\x15a\0\0a-\xA8V[\x91P\x81g\x11\x90\0\xAB\x10\x0F\xFC\0a$\x9B\x87\x88a\"\xC6V[a$\xA7\x90`\0\x19a,\xA9V[a$\xB1\x91\x90a,\x82V[a$\xBB\x91\x90a-\xA8V[\x92PP`\0a$\xC9\x83a\x18\x1AV[\x90P`\0a$\xD7\x85\x83a\"\xC6V[\x90P`\0\x88\x12a$\xE7W\x80a\x0F\xA0V[a\x0F\xA0\x81g\x1B\xC1mgN\xC8\0\0a,\x82V[`\0\x80\x82\x11a%6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\tWV[P`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x82\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[`\0a\x0B\x82\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x05\x85\x14\x17\x16a%\xCAW`\0\x80\xFD[\x05\x92\x91PPV[\x80Qjconsole.log` \x83\x01`\0\x80\x84\x83\x85Z\xFAPPPPPV[`\0`\x01`\xFF\x1B\x82\x03a&\x18W`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x12\x15a\x0B\xCEWP\x19`\x01\x01\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a&\xE4Wa&\xE4a&)V[PP\x835\x95` \x85\x015\x95P`@\x85\x015\x94``\x81\x015\x94P`\x80\x015\x92P\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a'\x1DWa'\x1Da&)V[\x825\x80\x15\x15\x81\x14a'-W`\0\x80\xFD[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a'aW` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a'EV[P`\0` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[\x84\x15\x15\x81R\x83` \x82\x01R\x82`@\x82\x01R`\x80``\x82\x01R`\0a'\xA8`\x80\x83\x01\x84a';V[\x96\x95PPPPPPV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FABI decoding: struct data too sh`D\x82\x01Rb\x1B\xDC\x9D`\xEA\x1B`d\x82\x01R`\x84\x81\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a(<Wa(<a(\x03V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a(kWa(ka(\x03V[`@R\x91\x90PV[`\0``\x82\x84\x03\x12\x15a(\x88Wa(\x88a'\xB2V[a(\x90a(\x19V[\x90P\x815\x81R` \x82\x015` \x82\x01R`@\x82\x015`@\x82\x01R\x92\x91PPV[`\0\x80`\0\x80`\xC0\x85\x87\x03\x12\x15a(\xC9Wa(\xC9a&)V[\x845\x93P` \x85\x015\x92P`@\x85\x015\x91Pa(\xE8\x86``\x87\x01a(sV[\x90P\x92\x95\x91\x94P\x92PV[` \x81R`\0a\x0B\x82` \x83\x01\x84a';V[`\0\x80`\0`\xA0\x84\x86\x03\x12\x15a)\x1EWa)\x1Ea&)V[\x835\x92P` \x84\x015\x91Pa)6\x85`@\x86\x01a(sV[\x90P\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a)TWa)Ta&)V[P5\x91\x90PV[\x81Q\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x80\x83\x01Q\x90\x82\x01R``\x81\x01a\x17MV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x81\xFD[`\0\x80` \x83\x85\x03\x12\x15a)\xEBWa)\xEBa&)V[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a*\x06Wa*\x06a&yV[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a*\x1DWa*\x1Da)|V[\x815\x81\x81\x11\x15a*\x80W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x0ENL/$\r\x8C\xAD\xCC\xEE\x8D`\xAB\x1B`d\x82\x01R`\x84\x81\xFD[\x86` \x82\x85\x01\x01\x11\x15a*\xE8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rjrray stride`\xA8\x1B`d\x82\x01R\x91P`\x84\x82\xFD[` \x92\x90\x92\x01\x96\x91\x95P\x90\x93PPPPV[`\0\x80`@\x83\x85\x03\x12\x15a+\x10Wa+\x10a&)V[PP\x805\x92` \x90\x91\x015\x91PV[`\0\x80`\x80\x83\x85\x03\x12\x15a+5Wa+5a&)V[\x825\x91Pa+F\x84` \x85\x01a(sV[\x90P\x92P\x92\x90PV[`\0` \x80\x83\x85\x03\x12\x15a+eWa+ea&)V[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a+\x80Wa+\x80a&yV[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a+\x97Wa+\x97a)|V[\x815\x81\x81\x11\x15a+\xA9Wa+\xA9a(\x03V[a+\xBB`\x1F\x82\x01`\x1F\x19\x16\x85\x01a(BV[\x91P\x80\x82R\x86\x84\x82\x85\x01\x01\x11\x15a,!W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x85\x90R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R`\x84\x81\xFD[\x80\x84\x84\x01\x85\x84\x017`\0\x90\x82\x01\x90\x93\x01\x92\x90\x92RP\x93\x92PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a,UWa,Ua&)V[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a,\xA2Wa,\xA2a,lV[P\x92\x91PPV[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15a,\xC5Wa,\xC5a,lV[\x81\x81\x05\x83\x14\x82\x15\x17a\x17MWa\x17Ma,lV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a,\xFEWa,\xFEa,\xD9V[`\x01`\xFF\x1B\x82\x14`\0\x19\x84\x14\x16\x15a-\x18Wa-\x18a,lV[P\x05\x90V[`\0\x80`\0``\x84\x86\x03\x12\x15a-5Wa-5a&)V[\x83Q\x92P` \x84\x01Q\x91P`@\x84\x01Q\x90P\x92P\x92P\x92V[\x80\x82\x01\x80\x82\x11\x15a\x17MWa\x17Ma,lV[\x81\x81\x03\x81\x81\x11\x15a\x17MWa\x17Ma,lV[\x84\x81R` \x80\x82\x01\x85\x90R`@\x80\x83\x01\x85\x90R\x83Q``\x84\x01R\x90\x83\x01Q`\x80\x83\x01R\x82\x01Q`\xA0\x82\x01R`\xC0\x81\x01a\x07jV[\x80\x82\x01\x82\x81\x12`\0\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a-\xC8Wa-\xC8a,lV[PP\x92\x91PPV[` \x80\x82R`\x11\x90\x82\x01Rp\x15\\\x19\x18]\x19H\x19[\x99\x08\x1C\x18\\\xDC\xD9Y`z\x1B`@\x82\x01R``\x01\x90V[`\0\x82a.\nWa.\na,\xD9V[P\x04\x90V[`\0`\x01`\x01`\xFF\x1B\x01\x82\x01a.'Wa.'a,lV[P`\x01\x01\x90V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x17MWa\x17Ma,lV[`\0\x82a.TWa.Ta,\xD9V[P\x07\x90V[`\0`\x01`\xFF\x1B\x82\x01a.nWa.na,lV[P`\0\x03\x90V[`@\x81R`\0a.\x88`@\x83\x01\x85a';V[\x90P\x82` \x83\x01R\x93\x92PPPV[`\0\x80`\0\x80\x84\x86\x03`\xC0\x81\x12\x15a.\xB1Wa.\xB1a&)V[\x85Q\x94P` \x86\x01Q\x93P`@\x86\x01Q\x92P```_\x19\x82\x01\x12\x15a.\xD8Wa.\xD8a'\xB2V[Pa.\xE1a(\x19V[``\x86\x01Q\x81R`\x80\x86\x01Q` \x82\x01R`\xA0\x90\x95\x01Q`@\x86\x01RP\x91\x94\x90\x93P\x90\x91\x90V";
    /// The bytecode of the contract.
    pub static LOGNORMAL_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x02\xB6W`\x005`\xE0\x1C\x80c\x8DR\xA1\xFC\x11a\x01\x9EW\x80c\xC1\xE0\x04;\x11a\x01\x10W\x80c\xDB\xAF\x11B\x11a\0\xD4W\x80c\xDB\xAF\x11B\x14a\x06*W\x80c\xE3\xD0\xDC\xA5\x14a\x06=W\x80c\xE4\x93t(\x14a\x06FW\x80c\xEC)\xD8\xE6\x14a\x06YW\x80c\xF9\xC2\x82\x11\x14a\x06lW\x80c\xFF\xB3bf\x14a\x06tWa\x02\xB6V[\x80c\xC1\xE0\x04;\x14a\x05\xECW\x80c\xC5)\x87\xCF\x14a\x05\xF4W\x80c\xCDd\xAE\xA2\x14a\x05\xFCW\x80c\xCF\xC4\xAFU\x14a\x06\x0FW\x80c\xD7\xF9{\xEF\x14a\x06\x17Wa\x02\xB6V[\x80c\xA6\xD34\x98\x11a\x01bW\x80c\xA6\xD34\x98\x14a\x055W\x80c\xAF\xDF1\xCD\x14a\x05HW\x80c\xB8\xF0\x0Br\x14a\x05PW\x80c\xBDB-(\x14a\x05cW\x80c\xC1e&\x12\x14a\x05\xA1W\x80c\xC1nP\xEF\x14a\x05\xAAWa\x02\xB6V[\x80c\x8DR\xA1\xFC\x14a\x04\xD6W\x80c\x97\x16\xAE\xBB\x14a\x04\xE9W\x80c\x99\x87\xFEd\x14a\x04\xFCW\x80c\x9D\x81\x9E\x83\x14a\x05\x0FW\x80c\xA1\x9C\xD3\xD1\x14a\x05\"Wa\x02\xB6V[\x80cV\x08\xBE\xA1\x11a\x027W\x80cj\x14`$\x11a\x01\xFBW\x80cj\x14`$\x14a\x04\x80W\x80cme\"\x99\x14a\x04\x8FW\x80c\x7F\x0EL\x8C\x14a\x04\x97W\x80c\x84\xC8*&\x14a\x04\xACW\x80c\x85\xAAEN\x14a\x04\xB4W\x80c\x88;m\xC5\x14a\x04\xC3Wa\x02\xB6V[\x80cV\x08\xBE\xA1\x14a\x04\x1DW\x80cX\xFAc\xCA\x14a\x040W\x80c^aZk\x14a\x048W\x80cd\x17\xD4\xB5\x14a\x04[W\x80cf\x8F\x90U\x14a\x04nWa\x02\xB6V[\x80c']g\xC8\x11a\x02~W\x80c']g\xC8\x14a\x03\xAAW\x80c.-yi\x14a\x03\xB9W\x80c>\x1E3\x92\x14a\x03\xCEW\x80c@\xB41i\x14a\x03\xD7W\x80cM\xDFG\xD4\x14a\x03\xE0Wa\x02\xB6V[\x80c\x06%\xA6#\x14a\x03\x1BW\x80c\n\xC304\x14a\x03AW\x80c\x0F\n\xA3\x95\x14a\x03dW\x80c\x18M0\xBA\x14a\x03\x84W\x80c\x1D\x9C\xF7\"\x14a\x03\x97W[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x03.a\x03)6`\x04a&\xC9V[a\x06\x87V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x03Ta\x03O6`\x04a'\x07V[a\x07sV[`@Qa\x038\x94\x93\x92\x91\x90a'\x81V[a\x03wa\x03r6`\x04a(\xB0V[a\x0B\x05V[`@Qa\x038\x91\x90a(\xF3V[a\x03.a\x03\x926`\x04a)\x06V[a\x0B7V[a\x03.a\x03\xA56`\x04a)?V[a\x0B\x89V[a\x03.g\x1B\xC1mgN\xC8\0\0\x81V[a\x03\xC1a\x0B\xD2V[`@Qa\x038\x91\x90a)[V[a\x03.`\nT\x81V[a\x03.`\0T\x81V[a\x03\xF3a\x03\xEE6`\x04a)\xD5V[a\x0C\x1BV[`@\x80Q\x95\x15\x15\x86R` \x86\x01\x94\x90\x94R\x92\x84\x01\x91\x90\x91R``\x83\x01R`\x80\x82\x01R`\xA0\x01a\x038V[a\x03.a\x04+6`\x04a)\x06V[a\x0C\x80V[a\x03.`\0\x81V[a\x04@a\x0C\xC1V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x038V[a\x03.a\x04i6`\x04a(\xB0V[a\x0C\xE9V[`\x01T`\x02T`\x03Ta\x04@\x92\x91\x90\x83V[a\x03.g\r\xE0\xB6\xB3\xA7d\0\0\x81V[a\x03.`\x01\x81V[a\x04\xAAa\x04\xA56`\x04a*\xFAV[a\rBV[\0[a\x03.`\n\x81V[a\x03.g\x06\xF0[Y\xD3\xB2\0\0\x81V[a\x03.a\x04\xD16`\x04a(\xB0V[a\x0E\x19V[a\x04\xAAa\x04\xE46`\x04a*\xFAV[a\x0EqV[a\x03.a\x04\xF76`\x04a+\x1FV[a\x0F\x0FV[a\x04\xAAa\x05\n6`\x04a*\xFAV[a\x0F\xACV[a\x03.a\x05\x1D6`\x04a)\x06V[a\x10JV[a\x03.a\x0506`\x04a+OV[a\x10\xDFV[a\x03.a\x05C6`\x04a)?V[a\x11\rV[a\x03.a\x116V[a\x03.a\x05^6`\x04a*\xFAV[a\x11\xA6V[a\x03wa\x05q6`\x04a,=V[`@\x80Q` \x81\x01\x94\x90\x94R\x83\x81\x01\x92\x90\x92R``\x80\x84\x01\x91\x90\x91R\x81Q\x80\x84\x03\x90\x91\x01\x81R`\x80\x90\x92\x01\x90R\x90V[a\x03.`\x05T\x81V[a\x05\xBDa\x05\xB86`\x04a+OV[a\x11\xD4V[`@\x80Q\x96\x15\x15\x87R` \x87\x01\x95\x90\x95R\x93\x85\x01\x92\x90\x92R``\x84\x01R`\x80\x83\x01R`\xA0\x82\x01R`\xC0\x01a\x038V[a\x03\xC1a\x14\x15V[a\x03.a\x14`V[a\x03.a\x06\n6`\x04a(\xB0V[a\x14\xCBV[a\x03.a\x15\x13V[a\x03.a\x06%6`\x04a+\x1FV[a\x15~V[a\x03.a\x0686`\x04a)\x06V[a\x15\xA6V[a\x03.`\0\x19\x81V[a\x03.a\x06T6`\x04a,=V[a\x15\xE8V[a\x03.a\x06g6`\x04a,=V[a\x16fV[a\x03.`\xFA\x81V[a\x03.a\x06\x826`\x04a,=V[a\x16\x9FV[`\0\x80a\x06\x94\x84\x84a\x17\x1EV[\x90P`\0a\x06\xA1\x85a\x11\rV[\x90P`\0a\x06\xAF\x82\x86a\x17SV[\x90P`\0a\x06\xBD\x8A\x8Aa\x17hV[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0\x81\x12a\x06\xDBW`\0\x94PPPPPa\x07jV[`\0\x81\x13a\x06\xF1W`\0\x19\x94PPPPPa\x07jV[`\0a\x07\ra\x07\x08\x83g\r\xE0\xB6\xB3\xA7d\0\0a,\x82V[a\x17}V[\x90P`\0\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x07%\x88\x85a,\xA9V[a\x07/\x91\x90a,\xEFV[a\x079\x91\x90a,\x82V[\x90P`\0a\x07F\x82a\x18\x1AV[\x90P`\0a\x07S\x82a\x0B\x89V[\x90Pa\x07_\x8C\x82a\x19\xC3V[\x98PPPPPPPPP[\x95\x94PPPPPV[`\0\x80`\0```\0\x80`\x003`\x01`\x01`\xA0\x1B\x03\x16c\xEB\xAD\xEF\x01`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01```@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x08\x08W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x08\x1CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08@\x91\x90a-\x1DV[\x92P\x92P\x92Pa\x08Q\x83\x83\x83a\x16fV[`@\x80Q` \x81\x01\x86\x90R\x90\x81\x01\x84\x90R``\x81\x01\x82\x90R\x90\x91P`\0\x90a\x08\x8B\x90`\x80\x01[`@Q` \x81\x83\x03\x03\x81R\x90`@Ra\x10\xDFV[\x90P`\0\x8A\x15a\t\xB3W`\0\x80Ta\x08\xA4\x90\x8C\x90a\x19\xC3V[\x90P`\0a\x08\xBC\x87a\x08\xB6\x84\x88a\x19\xC3V[\x90a\x19\xD8V[\x90Pa\x08\xC9`\x01\x82a-NV[\x90Pa\x08\xD5\x8C\x88a-NV[\x96Pa\x08\xE1\x81\x86a-NV[\x94P\x85a\x08\xF2\x88\x87\x87a\x04ia\x14\x15V[\x96Pa\x08\xFF`\x01\x88a-NV[\x96P\x80\x87\x10a\t`W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: y reserve increase`D\x82\x01Rad!`\xF0\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[a\tj\x87\x82a-aV[\x93Pa\t\xAB`@Q\x80`@\x01`@R\x80`\x1C\x81R` \x01\x7FEsimated Y reserve to submit\0\0\0\0\x81RP\x88a\x19\xEDV[PPPa\n\x83V[`\0\x80Ta\t\xC2\x90\x8C\x90a\x19\xC3V[\x90P`\0a\t\xD4\x86a\x08\xB6\x84\x88a\x19\xC3V[\x90Pa\t\xE1`\x01\x82a-NV[\x90Pa\t\xED\x8C\x87a-NV[\x95Pa\t\xF9\x81\x86a-NV[\x94P\x86a\n\n\x87\x87\x87a\x06\na\x14\x15V[\x97Pa\n\x17`\x01\x89a-NV[\x97P\x80\x88\x10a\nsW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: x reserve increase`D\x82\x01Rad!`\xF0\x1B`d\x82\x01R`\x84\x01a\tWV[a\n}\x88\x82a-aV[\x93PPPP[`@\x80Q` \x81\x01\x87\x90R\x90\x81\x01\x85\x90R``\x81\x01\x84\x90R`\0\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`\0a\n\xBD\x82a\x11\xD4V[PPPPP\x90P`\0a\n\xCEa\x14\x15V[\x90P\x81\x84a\n\xEB\x8A\x89\x85`\0\x01Q\x86` \x01Q\x87`@\x01Qa\x06\x87V[\x85\x9BP\x9BP\x9BP\x9BPPPPPPPPP\x92\x95\x91\x94P\x92PV[``\x84\x84\x84\x84`@Q` \x01a\x0B\x1E\x94\x93\x92\x91\x90a-tV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x94\x93PPPPV[`\0\x80a\x0BD\x84\x84a\x0F\x0FV[\x90P`\0a\x0BQ\x82a\x1A6V[\x90P`\0a\x0B^\x82a\x0B\x89V[\x90Pa\x0B|a\x0Bu\x82g\r\xE0\xB6\xB3\xA7d\0\0a-aV[\x88\x90a\x19\xC3V[\x93PPPP[\x93\x92PPPV[`\0\x80\x82\x12\x15a\x0B\xCEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01RotoUint: negative`\x80\x1B`D\x82\x01R`d\x01a\tWV[P\x90V[a\x0B\xF6`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[P`@\x80Q``\x81\x01\x82R`\x01T\x81R`\x02T` \x82\x01R`\x03T\x91\x81\x01\x91\x90\x91R\x90V[`\0\x80\x80\x80\x80a\x0C-\x86\x88\x01\x88a(\xB0V[\x80Q`\x01U` \x81\x01Q`\x02U`@\x01Q`\x03U\x91\x94P\x92P\x90Pa\x0CPa\x1A\x9FV[a\x0Cc\x83\x83\x83a\x0C^a\x14\x15V[a\x1A\xEEV[\x93Pa\x0Cq`\0`\x03a-\xA8V[\x84\x12\x15\x94P\x92\x95P\x92\x95\x90\x93PV[`\0\x80a\x0C\x8D\x84\x84a\x15~V[\x90P`\0a\x0C\x9A\x82a\x1A6V[\x90P`\0a\x0C\xA7\x82a\x0B\x89V[\x85Q\x90\x91Pa\x0B|\x90a\x0C\xBA\x90\x83a\x17SV[\x88\x90a\x17hV[`\0\x80`\0a\x0C\xCEa\x14`V[a\x0C\xD6a\x116V[a\x0C\xDEa\x15\x13V[\x92P\x92P\x92P\x90\x91\x92V[\x80Q`\0\x90`\n\x90\x82\x90a\x0C\xFE\x90\x83\x90a-aV[\x90Pa\r7\x87\x87\x87\x87`@Q` \x01a\r\x1A\x94\x93\x92\x91\x90a-tV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83\x83`\x01`\xFAa\x1B\x99a\x1B\xD4V[\x97\x96PPPPPPPV[B\x81\x11a\raW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\tW\x90a-\xD0V[a\ria\x1C\xE5V[`\0\x82`\x0ET\x11a\r\x86W`\x0ETa\r\x81\x90\x84a-aV[a\r\x94V[\x82`\x0ETa\r\x94\x91\x90a-aV[\x90Pa\r\xA0B\x83a-aV[a\r\xAA\x90\x82a-\xFBV[`\x11U`\x0F\x83\x90U`\x12\x82\x90U\x7Fs\xEBa\xB7\x83\x9B\xB3\xC27\xA0\xB3\xAE\xAB\x18iQ\0\xBCQ.q9\xF0\xE3@e]\xD7\x02\xDA*\xA1a\r\xE0a\x116V[a\r\xE8a\x14`V[a\r\xF0a\x15\x13V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01RB``\x82\x01R`\x80\x01`@Q\x80\x91\x03\x90\xA1PPPV[`\0\x80a\x0E'\x86`\x01a-NV[\x90P`\0k\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x90Pa\r7\x87\x87\x87\x87`@Q` \x01a\x0ET\x94\x93\x92\x91\x90a-tV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83\x83`\x01`\xFAa\x1C\xF6a\x1B\xD4V[B\x81\x11a\x0E\x90W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\tW\x90a-\xD0V[a\x0E\x98a\x1D&V[`\0\x82`\tT\x11a\x0E\xB5W`\tTa\x0E\xB0\x90\x84a-aV[a\x0E\xC3V[\x82`\tTa\x0E\xC3\x91\x90a-aV[\x90Pa\x0E\xCFB\x83a-aV[a\x0E\xD9\x90\x82a-\xFBV[`\x0CU`\n\x83\x90U`\r\x82\x90U\x7Fs\xEBa\xB7\x83\x9B\xB3\xC27\xA0\xB3\xAE\xAB\x18iQ\0\xBCQ.q9\xF0\xE3@e]\xD7\x02\xDA*\xA1a\r\xE0a\x116V[`\0\x80a\x0F$\x83` \x01Q\x84`@\x01Qa\x17\x1EV[\x90P`\0a\x0F5\x84` \x01Qa\x11\rV[\x90P`\0a\x0FXa\x0FS\x86`\0\x01Q\x88a\x19\xD8\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x1D7V[\x90P\x80a\x0Fd\x81a.\x0FV[\x91PP`\0a\x0F\x80\x86`@\x01Q\x84a\x17S\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x0F\x8A\x90\x83a-\xA8V[\x90P\x83a\x0F\xA0\x82g\r\xE0\xB6\xB3\xA7d\0\0\x83a\x16\x9FV[\x98\x97PPPPPPPPV[B\x81\x11a\x0F\xCBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\tW\x90a-\xD0V[a\x0F\xD3a\x1F\x12V[`\0\x82`\x04T\x11a\x0F\xF0W`\x04Ta\x0F\xEB\x90\x84a-aV[a\x0F\xFEV[\x82`\x04Ta\x0F\xFE\x91\x90a-aV[\x90Pa\x10\nB\x83a-aV[a\x10\x14\x90\x82a-\xFBV[`\x07U`\x05\x83\x90U`\x08\x82\x90U\x7Fs\xEBa\xB7\x83\x9B\xB3\xC27\xA0\xB3\xAE\xAB\x18iQ\0\xBCQ.q9\xF0\xE3@e]\xD7\x02\xDA*\xA1a\r\xE0a\x116V[`\0\x80a\x10W\x84\x84a\x0F\x0FV[\x90P`\0a\x10d\x82a\x1A6V[\x90P`\0a\x10q\x82a\x0B\x89V[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0\x81\x10a\x10\xCAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7Flx: denominator is zero\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\tWV[a\x0B|a\x0C\xBA\x82g\r\xE0\xB6\xB3\xA7d\0\0a-aV[`\0\x80`\0\x80\x84\x80` \x01\x90Q\x81\x01\x90a\x10\xF9\x91\x90a-\x1DV[\x92P\x92P\x92Pa\x07j\x83\x83\x83a\x0C^a\x14\x15V[`\0\x80a\x11\"\x83g\x1B\xC1mgN\xC8\0\0a\x1F#V[\x90Pa\x0B\x82g\x06\xF0[Y\xD3\xB2\0\0\x82a\x17SV[`\0`\x08TB\x10a\x11HWP`\x05T\x90V[`\x05T`\x04T\x11a\x11\x7FW`\x07T`\x06Ta\x11c\x90Ba-aV[a\x11m\x91\x90a..V[`\x04Ta\x11z\x91\x90a-NV[\x90P\x90V[`\x07T`\x06Ta\x11\x8F\x90Ba-aV[a\x11\x99\x91\x90a..V[`\x04Ta\x11z\x91\x90a-aV[`\0\x80a\x11\xB1a\x14\x15V[\x90Pa\x11\xCC\x84\x84\x83`\0\x01Q\x84` \x01Q\x85`@\x01Qa\x06\x87V[\x94\x93PPPPV[`\0\x80`\0\x80`\0\x80`\0\x80`\x003`\x01`\x01`\xA0\x1B\x03\x16c\xEB\xAD\xEF\x01`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01```@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x12kW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x12\x7FW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\xA3\x91\x90a-\x1DV[\x92P\x92P\x92Pa\x12\xB4\x83\x83\x83a\x16fV[\x90P\x89\x80` \x01\x90Q\x81\x01\x90a\x12\xCA\x91\x90a-\x1DV[\x91\x97P\x95P\x93P`\x01\x83\x87\x11\x15a\x13#W`\0a\x12\xE7\x85\x89a-aV[\x90P`\0a\x13\0`\0T\x83a\x19\xC3\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa\x13\x10\x86a\x08\xB6\x83\x87a\x19\xC3V[a\x13\x1A\x90\x84a-NV[\x92PPPa\x13\xC1V[\x82\x86\x11\x15a\x13`W`\0a\x137\x84\x88a-aV[\x90P`\0a\x13P`\0T\x83a\x19\xC3\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa\x13\x10\x85a\x08\xB6\x83\x87a\x19\xC3V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`0`$\x82\x01R\x7Finvalid swap: inputs x and y hav`D\x82\x01Roe the same sign!`\x80\x1B`d\x82\x01R`\x84\x01a\tWV[a\x13\xCB\x82\x86a,\x82V[\x97Pa\x13\xDB\x84\x84\x84a\x0C^a\x14\x15V[a\x13\xE9\x88\x88\x88a\x0C^a\x14\x15V[a\x13\xF3\x91\x90a,\x82V[\x98P`\0\x89\x12\x15\x80\x15a\x14\x06WP\x80\x88\x12\x15[\x99PPPPP\x91\x93\x95P\x91\x93\x95V[a\x149`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[a\x14Aa\x14`V[a\x14Ia\x116V[a\x14Qa\x15\x13V[`@\x84\x01R` \x83\x01R\x81R\x90V[`\0`\rTB\x10a\x14rWP`\nT\x90V[`\nT`\tT\x11a\x14\xA4W`\x0CT`\x0BTa\x14\x8D\x90Ba-aV[a\x14\x97\x91\x90a..V[`\tTa\x11z\x91\x90a-NV[`\x0CT`\x0BTa\x14\xB4\x90Ba-aV[a\x14\xBE\x91\x90a..V[`\tTa\x11z\x91\x90a-aV[`\0`\n\x81a\x14\xDA\x82\x87a-aV[\x90Pa\r7\x87\x87\x87\x87`@Q` \x01a\x14\xF6\x94\x93\x92\x91\x90a-tV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83\x83`\x01`\xFAa\x1FTa\x1B\xD4V[`\0`\x12TB\x10a\x15%WP`\x0FT\x90V[`\x0FT`\x0ET\x11a\x15WW`\x11T`\x10Ta\x15@\x90Ba-aV[a\x15J\x91\x90a..V[`\x0ETa\x11z\x91\x90a-NV[`\x11T`\x10Ta\x15g\x90Ba-aV[a\x15q\x91\x90a..V[`\x0ETa\x11z\x91\x90a-aV[`\0a\x15\x92\x82` \x01Q\x83`@\x01Qa\x17\x1EV[a\x15\x9C\x84\x84a\x0F\x0FV[a\x0B\x82\x91\x90a,\x82V[`\0\x80a\x15\xB3\x84\x84a\x15~V[\x90P`\0a\x15\xC0\x82a\x1A6V[\x90P`\0a\x15\xCD\x82a\x0B\x89V[\x85Q\x90\x91Pa\x0B|\x90\x82\x90a\x15\xE2\x90\x8Aa\x19\xC3V[\x90a\x19\xC3V[\x82\x82\x02\x81\x15\x80\x15\x90a\x16\x10WP\x83\x15\x80a\x16\x10WP\x82\x84\x82\x81a\x16\rWa\x16\ra,\xD9V[\x05\x14[a\x16NW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rn\x1B][\x1AY\x1A]\x88\x1A[\x9D\x98[\x1AY`\x8A\x1B`D\x82\x01R`d\x01a\tWV[\x81\x81\x81a\x16]Wa\x16]a,\xD9V[\x05\x94\x93PPPPV[`@\x80Q` \x81\x01\x85\x90R\x90\x81\x01\x83\x90R``\x81\x01\x82\x90R`\0\x90\x81\x90a\x16\x8F\x90`\x80\x01a\x08wV[\x90Pa\x07j\x85\x85\x83a\x04\xD1a\x14\x15V[`\0a\x16\xAC\x84\x84\x84a\x15\xE8V[\x90P\x81a\x16\xB9\x84\x86a,\xA9V[a\x16\xC3\x91\x90a.EV[\x15a\x0B\x82W`\x01`\x01`\xFF\x1B\x03\x81\x12a\x17\x13W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01RqmulidivUp overflow`p\x1B`D\x82\x01R`d\x01a\tWV[a\x11\xCC`\x01\x82a-\xA8V[`\0\x80a\x17*\x83a\x1F\x85V[\x90P`\0a\x17<c;\x9A\xCA\0\x83a..V[\x90Pa\x17H\x85\x82a\x17SV[\x92PPP[\x92\x91PPV[`\0a\x0B\x82\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a )V[`\0a\x0B\x82\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a )V[`\0g\x06\xF0[Y\xD3\xB2\0\0\x82\x03a\x17\x96WP`\0\x91\x90PV[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x12a\x17\xBEW`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\x17\xDFW`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x17\xEC\x83`\x02a,\xA9V[\x90P`\0a\x17\xF9\x82a HV[\x90P`\0a\x18\x0Fg\x13\xA0K\xBD\xFD\xC9\xBE\x88\x83a\"\xC6V[\x90Pa\x07j\x81a.YV[`\0h\x02H\xCE6\xA7\x0C\xB2k>\x19\x82\x13a\x185WP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a\x18|W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x01a\tWV[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05`\x01`_\x1B\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1Dm\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1Dn\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[`\0a\x0B\x82\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\"\xDBV[`\0a\x0B\x82\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\"\xDBV[a\x1A2\x82\x82`@Q`$\x01a\x1A\x03\x92\x91\x90a.uV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c-\x83\x9C\xB3`\xE2\x1B\x17\x90Ra#\tV[PPV[`\0\x80g\x13\xA0K\xBD\xFD\xC9\xBE\x88a\x1ATg\r\xE0\xB6\xB3\xA7d\0\0\x85a,\xA9V[a\x1A^\x91\x90a,\xEFV[\x90P`\0a\x1Ak\x82a.YV[\x90P`\0a\x1Ax\x82a#\x15V[\x90Pg\x1B\xC1mgN\xC8\0\0a\x1A\x95g\r\xE0\xB6\xB3\xA7d\0\0\x83a,\xA9V[a\x07j\x91\x90a,\xEFV[`\0a\x1A\xA9a\x0B\xD2V[` \x81\x01Q`\x05\x81\x90U`\x04UB`\x08\x81\x90U`\x06\x81\x90U\x81Q`\n\x81\x90U`\tU`\r\x81\x90U`\x0B\x81\x90U`@\x90\x91\x01Q`\x0F\x81\x90U`\x0EU`\x12\x81\x90U`\x10UPV[`\0\x82\x85\x10a\x1B?W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FtradingFunction: invalid x\0\0\0\0\0\0`D\x82\x01R`d\x01a\tWV[`\0a\x1BNa\x07\x08\x87\x86a\x17hV[\x90P`\0a\x1Bla\x07\x08\x87a\x1Bg\x87`\0\x01Q\x89a\x17SV[a\x17hV[\x90P`\0a\x1B\x82\x85` \x01Q\x86`@\x01Qa\x17\x1EV[\x90P\x80a\x1B\x8F\x83\x85a-\xA8V[a\x0F\xA0\x91\x90a-\xA8V[`\0\x80`\0\x80`\0\x86\x80` \x01\x90Q\x81\x01\x90a\x1B\xB5\x91\x90a.\x97V[\x93P\x93P\x93P\x93P\x81a\x1B\xCA\x85\x88\x86\x85a\x1A\xEEV[a\r7\x91\x90a,\x82V[`\0\x84\x86\x11\x15a\x1C\x01W`@Qc0\x82\xDF\xDB`\xE1\x1B\x81R`\x04\x81\x01\x87\x90R`$\x81\x01\x86\x90R`D\x01a\tWV[`\0a\x1C\x11\x88\x88\x85c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x1C#\x89\x88\x86c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x1C1\x82\x84a,\xA9V[\x13\x15a\x1CZW`@Qc\x06\xF1\xBE]`\xE2\x1B\x81R`\x04\x81\x01\x83\x90R`$\x81\x01\x82\x90R`D\x01a\tWV[`\0a\x1Cf\x89\x89a-aV[\x90P`\0[`\x02a\x1Cw\x8A\x8Ca-NV[a\x1C\x81\x91\x90a-\xFBV[\x94P`\0a\x1C\x93\x8C\x87\x89c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x1C\xA1\x86\x83a,\xA9V[\x13a\x1C\xAEW\x85\x99Pa\x1C\xB5V[\x85\x9AP\x80\x94P[a\x1C\xBF\x8B\x8Ba-aV[\x92PP`\x01\x01\x87\x82\x11\x80\x15a\x1C\xD3WP\x86\x81\x10[a\x1CkWPPPP\x96\x95PPPPPPV[a\x1C\xEDa\x15\x13V[`\x0EUB`\x10UV[`\0\x80`\0\x80`\0\x86\x80` \x01\x90Q\x81\x01\x90a\x1D\x12\x91\x90a.\x97V[\x93P\x93P\x93P\x93Pa\r7\x84\x84\x88\x84a\x1A\xEEV[a\x1D.a\x14`V[`\tUB`\x0BUV[`\0\x80\x82\x13a\x1DtW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\tWV[`\0``a\x1D\x81\x84a$\xF9V[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[a\x1F\x1Aa\x116V[`\x04UB`\x06UV[`\0a\x0B\x82g\r\xE0\xB6\xB3\xA7d\0\0\x83a\x1F;\x86a\x1D7V[a\x1FE\x91\x90a,\xA9V[a\x1FO\x91\x90a,\xEFV[a\x18\x1AV[`\0\x80`\0\x80`\0\x86\x80` \x01\x90Q\x81\x01\x90a\x1Fp\x91\x90a.\x97V[\x93P\x93P\x93P\x93P\x81a\x1B\xCA\x87\x86\x86\x85a\x1A\xEEV[`\xB5\x81`\x01`\x88\x1B\x81\x10a\x1F\x9EW`@\x91\x90\x91\x1B\x90`\x80\x1C[i\x01\0\0\0\0\0\0\0\0\0\x81\x10a\x1F\xBAW` \x91\x90\x91\x1B\x90`@\x1C[e\x01\0\0\0\0\0\x81\x10a\x1F\xD2W`\x10\x91\x90\x91\x1B\x90` \x1C[c\x01\0\0\0\x81\x10a\x1F\xE8W`\x08\x91\x90\x91\x1B\x90`\x10\x1C[b\x01\0\0\x01\x02`\x12\x1C\x80\x82\x04\x01`\x01\x90\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x90\x81\x90\x04\x81\x11\x90\x03\x90V[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a AW`\0\x80\xFD[\x04\x92\x91PPV[`\0\x80\x82\x12\x80a _WPg\x1B\xC1mgN\xC8\0\0\x82\x13[\x15a }W`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a \x9EW`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81g\x1B\xC1mgN\xC8\0\0\x03a \xC6W`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x15a \xD1W\x91\x90PV[`\0g\r\xE0\xB6\xB3\xA7d\0\0\x83\x12a \xF9Wa \xF4\x83g\x1B\xC1mgN\xC8\0\0a,\x82V[a \xFBV[\x82[\x90P`\0a!\x11\x82g\x1B\xC1mgN\xC8\0\0a%\xA1V[\x90P\x80`\0\x03a!4W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a!?\x82a\x1D7V[\x90P`\0c;\x9A\xCA\0a!ja!ea!_g\x1B\xC1mgN\xC8\0\0a.YV[\x85a\"\xC6V[a\x1F\x85V[a!t\x91\x90a,\xA9V[\x90P`\0\x80a!\x8B\x83g\x03\xC1f\\z\xAB \0a\"\xC6V[a!\x9D\x90g \x05\xFEO&\x8E\xA0\0a-\xA8V[\x90P`\0a!\xCD\x84a!\xB6\x86f\x9F2u$b\xA0\0a\"\xC6V[a!\xC8\x90g\r\xC5R\x7Fd, \0a-\xA8V[a\"\xC6V[a!\xDF\x90g\r\xE0\xB6\xB3\xA7d\0\0a-\xA8V[\x90Pa\"\x03g\t\xD0(\xCCo _\xFF\x19\x85a!\xF9\x85\x85a%\xA1V[a!\xC8\x91\x90a,\x82V[\x92PPP`\0[`\x02\x81\x10\x15a\"\x9EW`\0\x86a\"\x1F\x84a#\x15V[a\")\x91\x90a,\x82V[\x90P`\0a\"7\x84\x85a\"\xC6V[a\"@\x90a.YV[\x90P`\0a\"M\x82a\x18\x1AV[\x90P`\0a\"[\x86\x85a\"\xC6V[a\"mg\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x84a\"\xC6V[a\"w\x91\x90a,\x82V[\x90Pa\"\x83\x84\x82a%\xA1V[a\"\x8D\x90\x87a-\xA8V[\x95P\x84`\x01\x01\x94PPPPPa\"\nV[g\r\xE0\xB6\xB3\xA7d\0\0\x88\x12a\"\xBBWa\"\xB6\x82a.YV[a\x0F\xA0V[P\x96\x95PPPPPPV[`\0a\x0B\x82\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a%\xB2V[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\"\xF3W`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[a#\x12\x81a%\xD1V[PV[`\0\x81`\0\x03a#.WPg\r\xE0\xB6\xB3\xA7d\0\0\x91\x90PV[gV\x98\xEE\xF0fp\0\0\x82\x12a#EWP`\0\x91\x90PV[a#VgV\x98\xEE\xF0fp\0\0a.YV[\x82\x13a#kWPg\x1B\xC1mgN\xC8\0\0\x91\x90PV[`\0a#v\x83a%\xF2V[\x90P`\0a#\xAFg\r\xE0\xB6\xB3\xA7d\0\0a#\x98\x84g\x1B\xC1mgN\xC8\0\0a\x17hV[a#\xAA\x90g\r\xE0\xB6\xB3\xA7d\0\0a-\xA8V[a%\xA1V[\x90P`\0\x80\x82a$\x0B\x81a#\xF8\x81a#\xE6\x81a#\xD3\x81g\x02_\x0F\xE1\x05\xA3\x14\0a\"\xC6V[a!\xC8\x90g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19a-\xA8V[a!\xC8\x90g\x14\xA8EL\x19\xE1\xAC\0a-\xA8V[a!\xC8\x90g\x0F\xC1\x0E\x01W\x82w\xFF\x19a-\xA8V[a$\x1D\x90g\x03\xDE\xBD\x08;\x8C|\0a-\xA8V[\x91P\x83\x90Pa$\x85\x81a$s\x81a$a\x81a$O\x81a$<\x81\x8Ba\"\xC6V[a!\xC8\x90g\x02\x95\xD4\0\xEA2W\xFF\x19a-\xA8V[a!\xC8\x90g\x01W\xD8\xB2\xEC\xC7\x08\0a-\xA8V[a!\xC8\x90g\x051\n\xA7\xD5!0\0a-\xA8V[a!\xC8\x90g\r\xE0\xCC=\x15a\0\0a-\xA8V[\x91P\x81g\x11\x90\0\xAB\x10\x0F\xFC\0a$\x9B\x87\x88a\"\xC6V[a$\xA7\x90`\0\x19a,\xA9V[a$\xB1\x91\x90a,\x82V[a$\xBB\x91\x90a-\xA8V[\x92PP`\0a$\xC9\x83a\x18\x1AV[\x90P`\0a$\xD7\x85\x83a\"\xC6V[\x90P`\0\x88\x12a$\xE7W\x80a\x0F\xA0V[a\x0F\xA0\x81g\x1B\xC1mgN\xC8\0\0a,\x82V[`\0\x80\x82\x11a%6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\tWV[P`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x82\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[`\0a\x0B\x82\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x05\x85\x14\x17\x16a%\xCAW`\0\x80\xFD[\x05\x92\x91PPV[\x80Qjconsole.log` \x83\x01`\0\x80\x84\x83\x85Z\xFAPPPPPV[`\0`\x01`\xFF\x1B\x82\x03a&\x18W`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x12\x15a\x0B\xCEWP\x19`\x01\x01\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a&\xE4Wa&\xE4a&)V[PP\x835\x95` \x85\x015\x95P`@\x85\x015\x94``\x81\x015\x94P`\x80\x015\x92P\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a'\x1DWa'\x1Da&)V[\x825\x80\x15\x15\x81\x14a'-W`\0\x80\xFD[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a'aW` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a'EV[P`\0` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[\x84\x15\x15\x81R\x83` \x82\x01R\x82`@\x82\x01R`\x80``\x82\x01R`\0a'\xA8`\x80\x83\x01\x84a';V[\x96\x95PPPPPPV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FABI decoding: struct data too sh`D\x82\x01Rb\x1B\xDC\x9D`\xEA\x1B`d\x82\x01R`\x84\x81\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a(<Wa(<a(\x03V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a(kWa(ka(\x03V[`@R\x91\x90PV[`\0``\x82\x84\x03\x12\x15a(\x88Wa(\x88a'\xB2V[a(\x90a(\x19V[\x90P\x815\x81R` \x82\x015` \x82\x01R`@\x82\x015`@\x82\x01R\x92\x91PPV[`\0\x80`\0\x80`\xC0\x85\x87\x03\x12\x15a(\xC9Wa(\xC9a&)V[\x845\x93P` \x85\x015\x92P`@\x85\x015\x91Pa(\xE8\x86``\x87\x01a(sV[\x90P\x92\x95\x91\x94P\x92PV[` \x81R`\0a\x0B\x82` \x83\x01\x84a';V[`\0\x80`\0`\xA0\x84\x86\x03\x12\x15a)\x1EWa)\x1Ea&)V[\x835\x92P` \x84\x015\x91Pa)6\x85`@\x86\x01a(sV[\x90P\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a)TWa)Ta&)V[P5\x91\x90PV[\x81Q\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x80\x83\x01Q\x90\x82\x01R``\x81\x01a\x17MV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x81\xFD[`\0\x80` \x83\x85\x03\x12\x15a)\xEBWa)\xEBa&)V[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a*\x06Wa*\x06a&yV[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a*\x1DWa*\x1Da)|V[\x815\x81\x81\x11\x15a*\x80W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x0ENL/$\r\x8C\xAD\xCC\xEE\x8D`\xAB\x1B`d\x82\x01R`\x84\x81\xFD[\x86` \x82\x85\x01\x01\x11\x15a*\xE8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rjrray stride`\xA8\x1B`d\x82\x01R\x91P`\x84\x82\xFD[` \x92\x90\x92\x01\x96\x91\x95P\x90\x93PPPPV[`\0\x80`@\x83\x85\x03\x12\x15a+\x10Wa+\x10a&)V[PP\x805\x92` \x90\x91\x015\x91PV[`\0\x80`\x80\x83\x85\x03\x12\x15a+5Wa+5a&)V[\x825\x91Pa+F\x84` \x85\x01a(sV[\x90P\x92P\x92\x90PV[`\0` \x80\x83\x85\x03\x12\x15a+eWa+ea&)V[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a+\x80Wa+\x80a&yV[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a+\x97Wa+\x97a)|V[\x815\x81\x81\x11\x15a+\xA9Wa+\xA9a(\x03V[a+\xBB`\x1F\x82\x01`\x1F\x19\x16\x85\x01a(BV[\x91P\x80\x82R\x86\x84\x82\x85\x01\x01\x11\x15a,!W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x85\x90R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R`\x84\x81\xFD[\x80\x84\x84\x01\x85\x84\x017`\0\x90\x82\x01\x90\x93\x01\x92\x90\x92RP\x93\x92PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a,UWa,Ua&)V[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a,\xA2Wa,\xA2a,lV[P\x92\x91PPV[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15a,\xC5Wa,\xC5a,lV[\x81\x81\x05\x83\x14\x82\x15\x17a\x17MWa\x17Ma,lV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a,\xFEWa,\xFEa,\xD9V[`\x01`\xFF\x1B\x82\x14`\0\x19\x84\x14\x16\x15a-\x18Wa-\x18a,lV[P\x05\x90V[`\0\x80`\0``\x84\x86\x03\x12\x15a-5Wa-5a&)V[\x83Q\x92P` \x84\x01Q\x91P`@\x84\x01Q\x90P\x92P\x92P\x92V[\x80\x82\x01\x80\x82\x11\x15a\x17MWa\x17Ma,lV[\x81\x81\x03\x81\x81\x11\x15a\x17MWa\x17Ma,lV[\x84\x81R` \x80\x82\x01\x85\x90R`@\x80\x83\x01\x85\x90R\x83Q``\x84\x01R\x90\x83\x01Q`\x80\x83\x01R\x82\x01Q`\xA0\x82\x01R`\xC0\x81\x01a\x07jV[\x80\x82\x01\x82\x81\x12`\0\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a-\xC8Wa-\xC8a,lV[PP\x92\x91PPV[` \x80\x82R`\x11\x90\x82\x01Rp\x15\\\x19\x18]\x19H\x19[\x99\x08\x1C\x18\\\xDC\xD9Y`z\x1B`@\x82\x01R``\x01\x90V[`\0\x82a.\nWa.\na,\xD9V[P\x04\x90V[`\0`\x01`\x01`\xFF\x1B\x01\x82\x01a.'Wa.'a,lV[P`\x01\x01\x90V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x17MWa\x17Ma,lV[`\0\x82a.TWa.Ta,\xD9V[P\x07\x90V[`\0`\x01`\xFF\x1B\x82\x01a.nWa.na,lV[P`\0\x03\x90V[`@\x81R`\0a.\x88`@\x83\x01\x85a';V[\x90P\x82` \x83\x01R\x93\x92PPPV[`\0\x80`\0\x80\x84\x86\x03`\xC0\x81\x12\x15a.\xB1Wa.\xB1a&)V[\x85Q\x94P` \x86\x01Q\x93P`@\x86\x01Q\x92P```_\x19\x82\x01\x12\x15a.\xD8Wa.\xD8a'\xB2V[Pa.\xE1a(\x19V[``\x86\x01Q\x81R`\x80\x86\x01Q` \x82\x01R`\xA0\x90\x95\x01Q`@\x86\x01RP\x91\x94\x90\x93P\x90\x91\x90V";
    /// The deployed bytecode of the contract.
    pub static LOGNORMAL_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
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
            f.debug_tuple(::core::stringify!(LogNormal))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> LogNormal<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                LOGNORMAL_ABI.clone(),
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
                LOGNORMAL_ABI.clone(),
                LOGNORMAL_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `APPROXIMATED_MINIMUM_X_INPUT` (0x84c82a26) function
        pub fn approximated_minimum_x_input(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([132, 200, 42, 38], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `BISECTION_EPSILON` (0x6d652299) function
        pub fn bisection_epsilon(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([109, 101, 34, 153], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `HALF_WAD` (0x85aa454e) function
        pub fn half_wad(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([133, 170, 69, 78], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `INFINITY_IS_NOT_REAL` (0xe3d0dca5) function
        pub fn infinity_is_not_real(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([227, 208, 220, 165], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `MAX_BISECTION_ITERS` (0xf9c28211) function
        pub fn max_bisection_iters(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([249, 194, 130, 17], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `TWO_WAD` (0x275d67c8) function
        pub fn two_wad(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([39, 93, 103, 200], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `WAD` (0x6a146024) function
        pub fn wad(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([106, 20, 96, 36], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ZERO` (0x58fa63ca) function
        pub fn zero(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([88, 250, 99, 202], ())
                .expect("method not found (this should never happen)")
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
        ///Calls the contract's `computeD1` (0x9716aebb) function
        pub fn compute_d1(
            &self,
            price_wad: ::ethers::core::types::U256,
            params: Parameters,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([151, 22, 174, 187], (price_wad, params))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `computeD2` (0xd7f97bef) function
        pub fn compute_d2(
            &self,
            price_wad: ::ethers::core::types::U256,
            params: Parameters,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([215, 249, 123, 239], (price_wad, params))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `computeHalfSigmaSquared` (0xa6d33498) function
        pub fn compute_half_sigma_squared(
            &self,
            sigma_percent_wad: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([166, 211, 52, 152], sigma_percent_wad)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `computePrice` (0x0625a623) function
        pub fn compute_price(
            &self,
            reserve_x_wad: ::ethers::core::types::U256,
            total_liquidity: ::ethers::core::types::U256,
            strike_price_wad: ::ethers::core::types::U256,
            sigma_percent_wad: ::ethers::core::types::U256,
            tau_years_wad: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [6, 37, 166, 35],
                    (
                        reserve_x_wad,
                        total_liquidity,
                        strike_price_wad,
                        sigma_percent_wad,
                        tau_years_wad,
                    ),
                )
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
        ///Calls the contract's `dynamicSlot` (0xc1e0043b) function
        pub fn dynamic_slot(&self) -> ::ethers::contract::builders::ContractCall<M, Parameters> {
            self.0
                .method_hash([193, 224, 4, 59], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `encodeInitData` (0x0f0aa395) function
        pub fn encode_init_data(
            &self,
            reserve_x_wad: ::ethers::core::types::U256,
            reseve_y_wad: ::ethers::core::types::U256,
            total_liquidity: ::ethers::core::types::U256,
            params: Parameters,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Bytes> {
            self.0
                .method_hash(
                    [15, 10, 163, 149],
                    (reserve_x_wad, reseve_y_wad, total_liquidity, params),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `encodeValidateData` (0xbd422d28) function
        pub fn encode_validate_data(
            &self,
            adjusted_reserve_x_wad: ::ethers::core::types::U256,
            adjusted_reserve_y_wad: ::ethers::core::types::U256,
            adjusted_liquidity: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Bytes> {
            self.0
                .method_hash(
                    [189, 66, 45, 40],
                    (
                        adjusted_reserve_x_wad,
                        adjusted_reserve_y_wad,
                        adjusted_liquidity,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `findLiquidity` (0x883b6dc5) function
        pub fn find_liquidity(
            &self,
            reserve_x_wad: ::ethers::core::types::U256,
            reserve_y_wad: ::ethers::core::types::U256,
            swap_constant: ::ethers::core::types::I256,
            params: Parameters,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [136, 59, 109, 197],
                    (reserve_x_wad, reserve_y_wad, swap_constant, params),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `findX` (0xcd64aea2) function
        pub fn find_x(
            &self,
            reserve_y_wad: ::ethers::core::types::U256,
            liquidity: ::ethers::core::types::U256,
            swap_constant: ::ethers::core::types::I256,
            params: Parameters,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [205, 100, 174, 162],
                    (reserve_y_wad, liquidity, swap_constant, params),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `findY` (0x6417d4b5) function
        pub fn find_y(
            &self,
            reserve_x_wad: ::ethers::core::types::U256,
            liquidity: ::ethers::core::types::U256,
            swap_constant: ::ethers::core::types::I256,
            params: Parameters,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [100, 23, 212, 181],
                    (reserve_x_wad, liquidity, swap_constant, params),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getNextLiquidity` (0xec29d8e6) function
        pub fn get_next_liquidity(
            &self,
            reserve_x_wad: ::ethers::core::types::U256,
            reserve_y_wad: ::ethers::core::types::U256,
            total_liquidity: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [236, 41, 216, 230],
                    (reserve_x_wad, reserve_y_wad, total_liquidity),
                )
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
        ///Calls the contract's `internalPrice` (0xb8f00b72) function
        pub fn internal_price(
            &self,
            reserve_x_wad: ::ethers::core::types::U256,
            total_liquidity: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([184, 240, 11, 114], (reserve_x_wad, total_liquidity))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lx` (0x9d819e83) function
        pub fn lx(
            &self,
            reserve_x_wad: ::ethers::core::types::U256,
            price_wad: ::ethers::core::types::U256,
            params: Parameters,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([157, 129, 158, 131], (reserve_x_wad, price_wad, params))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ly` (0x5608bea1) function
        pub fn ly(
            &self,
            reserve_y_wad: ::ethers::core::types::U256,
            price_wad: ::ethers::core::types::U256,
            params: Parameters,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([86, 8, 190, 161], (reserve_y_wad, price_wad, params))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mulidiv` (0xe4937428) function
        pub fn mulidiv(
            &self,
            x: ::ethers::core::types::I256,
            y: ::ethers::core::types::I256,
            denominator: ::ethers::core::types::I256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([228, 147, 116, 40], (x, y, denominator))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mulidivUp` (0xffb36266) function
        pub fn mulidiv_up(
            &self,
            x: ::ethers::core::types::I256,
            y: ::ethers::core::types::I256,
            denominator: ::ethers::core::types::I256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([255, 179, 98, 102], (x, y, denominator))
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
        ///Calls the contract's `simulateSwap` (0x0ac33034) function
        pub fn simulate_swap(
            &self,
            swap_x_in: bool,
            amount_in: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                bool,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::Bytes,
            ),
        > {
            self.0
                .method_hash([10, 195, 48, 52], (swap_x_in, amount_in))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `staticSlot` (0x2e2d7969) function
        pub fn static_slot(&self) -> ::ethers::contract::builders::ContractCall<M, Parameters> {
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
        ///Calls the contract's `toUint` (0x1d9cf722) function
        pub fn to_uint(
            &self,
            x: ::ethers::core::types::I256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([29, 156, 247, 34], x)
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
        ///Calls the contract's `xl` (0x184d30ba) function
        pub fn xl(
            &self,
            total_liquidity: ::ethers::core::types::U256,
            price_wad: ::ethers::core::types::U256,
            params: Parameters,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([24, 77, 48, 186], (total_liquidity, price_wad, params))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `yl` (0xdbaf1142) function
        pub fn yl(
            &self,
            total_liquidity: ::ethers::core::types::U256,
            price_wad: ::ethers::core::types::U256,
            params: Parameters,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([219, 175, 17, 66], (total_liquidity, price_wad, params))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `LogParameters` event
        pub fn log_parameters_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LogParametersFilter>
        {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LogParametersFilter>
        {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for LogNormal<M> {
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
        Hash,
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
        Hash,
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
    pub enum LogNormalErrors {
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
    impl ::ethers::core::abi::AbiDecode for LogNormalErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) =
                <BisectionLib_InvalidBounds as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::BisectionLib_InvalidBounds(decoded));
            }
            if let Ok(decoded) =
                <BisectionLib_RootOutsideBounds as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::BisectionLib_RootOutsideBounds(decoded));
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
    impl ::ethers::core::abi::AbiEncode for LogNormalErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::BisectionLib_InvalidBounds(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BisectionLib_RootOutsideBounds(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Infinity(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Min(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NegativeInfinity(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OutOfBounds(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for LogNormalErrors {
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
    impl ::core::fmt::Display for LogNormalErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::BisectionLib_InvalidBounds(element) => ::core::fmt::Display::fmt(element, f),
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
    impl ::core::convert::From<::std::string::String> for LogNormalErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<BisectionLib_InvalidBounds> for LogNormalErrors {
        fn from(value: BisectionLib_InvalidBounds) -> Self {
            Self::BisectionLib_InvalidBounds(value)
        }
    }
    impl ::core::convert::From<BisectionLib_RootOutsideBounds> for LogNormalErrors {
        fn from(value: BisectionLib_RootOutsideBounds) -> Self {
            Self::BisectionLib_RootOutsideBounds(value)
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
        Hash,
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
    ///Container type for all input parameters for the `APPROXIMATED_MINIMUM_X_INPUT` function with signature `APPROXIMATED_MINIMUM_X_INPUT()` and selector `0x84c82a26`
    #[derive(
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
        name = "APPROXIMATED_MINIMUM_X_INPUT",
        abi = "APPROXIMATED_MINIMUM_X_INPUT()"
    )]
    pub struct ApproximatedMinimumXInputCall;
    ///Container type for all input parameters for the `BISECTION_EPSILON` function with signature `BISECTION_EPSILON()` and selector `0x6d652299`
    #[derive(
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
    #[ethcall(name = "BISECTION_EPSILON", abi = "BISECTION_EPSILON()")]
    pub struct BisectionEpsilonCall;
    ///Container type for all input parameters for the `HALF_WAD` function with signature `HALF_WAD()` and selector `0x85aa454e`
    #[derive(
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
    #[ethcall(name = "HALF_WAD", abi = "HALF_WAD()")]
    pub struct HalfWadCall;
    ///Container type for all input parameters for the `INFINITY_IS_NOT_REAL` function with signature `INFINITY_IS_NOT_REAL()` and selector `0xe3d0dca5`
    #[derive(
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
    #[ethcall(name = "INFINITY_IS_NOT_REAL", abi = "INFINITY_IS_NOT_REAL()")]
    pub struct InfinityIsNotRealCall;
    ///Container type for all input parameters for the `MAX_BISECTION_ITERS` function with signature `MAX_BISECTION_ITERS()` and selector `0xf9c28211`
    #[derive(
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
    #[ethcall(name = "MAX_BISECTION_ITERS", abi = "MAX_BISECTION_ITERS()")]
    pub struct MaxBisectionItersCall;
    ///Container type for all input parameters for the `TWO_WAD` function with signature `TWO_WAD()` and selector `0x275d67c8`
    #[derive(
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
    #[ethcall(name = "TWO_WAD", abi = "TWO_WAD()")]
    pub struct TwoWadCall;
    ///Container type for all input parameters for the `WAD` function with signature `WAD()` and selector `0x6a146024`
    #[derive(
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
    #[ethcall(name = "WAD", abi = "WAD()")]
    pub struct WadCall;
    ///Container type for all input parameters for the `ZERO` function with signature `ZERO()` and selector `0x58fa63ca`
    #[derive(
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
    #[ethcall(name = "ZERO", abi = "ZERO()")]
    pub struct ZeroCall;
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
        Hash,
    )]
    #[ethcall(name = "__slot__", abi = "__slot__()")]
    pub struct SlotCall;
    ///Container type for all input parameters for the `computeD1` function with signature `computeD1(uint256,(uint256,uint256,uint256))` and selector `0x9716aebb`
    #[derive(
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
        name = "computeD1",
        abi = "computeD1(uint256,(uint256,uint256,uint256))"
    )]
    pub struct ComputeD1Call {
        pub price_wad: ::ethers::core::types::U256,
        pub params: Parameters,
    }
    ///Container type for all input parameters for the `computeD2` function with signature `computeD2(uint256,(uint256,uint256,uint256))` and selector `0xd7f97bef`
    #[derive(
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
        name = "computeD2",
        abi = "computeD2(uint256,(uint256,uint256,uint256))"
    )]
    pub struct ComputeD2Call {
        pub price_wad: ::ethers::core::types::U256,
        pub params: Parameters,
    }
    ///Container type for all input parameters for the `computeHalfSigmaSquared` function with signature `computeHalfSigmaSquared(uint256)` and selector `0xa6d33498`
    #[derive(
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
        name = "computeHalfSigmaSquared",
        abi = "computeHalfSigmaSquared(uint256)"
    )]
    pub struct ComputeHalfSigmaSquaredCall {
        pub sigma_percent_wad: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `computePrice` function with signature `computePrice(uint256,uint256,uint256,uint256,uint256)` and selector `0x0625a623`
    #[derive(
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
        name = "computePrice",
        abi = "computePrice(uint256,uint256,uint256,uint256,uint256)"
    )]
    pub struct ComputePriceCall {
        pub reserve_x_wad: ::ethers::core::types::U256,
        pub total_liquidity: ::ethers::core::types::U256,
        pub strike_price_wad: ::ethers::core::types::U256,
        pub sigma_percent_wad: ::ethers::core::types::U256,
        pub tau_years_wad: ::ethers::core::types::U256,
    }
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
        Hash,
    )]
    #[ethcall(name = "computeSwapConstant", abi = "computeSwapConstant(bytes)")]
    pub struct ComputeSwapConstantCall {
        pub data: ::ethers::core::types::Bytes,
    }
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
        Hash,
    )]
    #[ethcall(name = "dynamicSlot", abi = "dynamicSlot()")]
    pub struct DynamicSlotCall;
    ///Container type for all input parameters for the `encodeInitData` function with signature `encodeInitData(uint256,uint256,uint256,(uint256,uint256,uint256))` and selector `0x0f0aa395`
    #[derive(
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
        name = "encodeInitData",
        abi = "encodeInitData(uint256,uint256,uint256,(uint256,uint256,uint256))"
    )]
    pub struct EncodeInitDataCall {
        pub reserve_x_wad: ::ethers::core::types::U256,
        pub reseve_y_wad: ::ethers::core::types::U256,
        pub total_liquidity: ::ethers::core::types::U256,
        pub params: Parameters,
    }
    ///Container type for all input parameters for the `encodeValidateData` function with signature `encodeValidateData(uint256,uint256,uint256)` and selector `0xbd422d28`
    #[derive(
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
        name = "encodeValidateData",
        abi = "encodeValidateData(uint256,uint256,uint256)"
    )]
    pub struct EncodeValidateDataCall {
        pub adjusted_reserve_x_wad: ::ethers::core::types::U256,
        pub adjusted_reserve_y_wad: ::ethers::core::types::U256,
        pub adjusted_liquidity: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `findLiquidity` function with signature `findLiquidity(uint256,uint256,int256,(uint256,uint256,uint256))` and selector `0x883b6dc5`
    #[derive(
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
        name = "findLiquidity",
        abi = "findLiquidity(uint256,uint256,int256,(uint256,uint256,uint256))"
    )]
    pub struct FindLiquidityCall {
        pub reserve_x_wad: ::ethers::core::types::U256,
        pub reserve_y_wad: ::ethers::core::types::U256,
        pub swap_constant: ::ethers::core::types::I256,
        pub params: Parameters,
    }
    ///Container type for all input parameters for the `findX` function with signature `findX(uint256,uint256,int256,(uint256,uint256,uint256))` and selector `0xcd64aea2`
    #[derive(
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
        name = "findX",
        abi = "findX(uint256,uint256,int256,(uint256,uint256,uint256))"
    )]
    pub struct FindXCall {
        pub reserve_y_wad: ::ethers::core::types::U256,
        pub liquidity: ::ethers::core::types::U256,
        pub swap_constant: ::ethers::core::types::I256,
        pub params: Parameters,
    }
    ///Container type for all input parameters for the `findY` function with signature `findY(uint256,uint256,int256,(uint256,uint256,uint256))` and selector `0x6417d4b5`
    #[derive(
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
        name = "findY",
        abi = "findY(uint256,uint256,int256,(uint256,uint256,uint256))"
    )]
    pub struct FindYCall {
        pub reserve_x_wad: ::ethers::core::types::U256,
        pub liquidity: ::ethers::core::types::U256,
        pub swap_constant: ::ethers::core::types::I256,
        pub params: Parameters,
    }
    ///Container type for all input parameters for the `getNextLiquidity` function with signature `getNextLiquidity(uint256,uint256,uint256)` and selector `0xec29d8e6`
    #[derive(
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
        name = "getNextLiquidity",
        abi = "getNextLiquidity(uint256,uint256,uint256)"
    )]
    pub struct GetNextLiquidityCall {
        pub reserve_x_wad: ::ethers::core::types::U256,
        pub reserve_y_wad: ::ethers::core::types::U256,
        pub total_liquidity: ::ethers::core::types::U256,
    }
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
        Hash,
    )]
    #[ethcall(name = "getParams", abi = "getParams()")]
    pub struct GetParamsCall;
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
        Hash,
    )]
    #[ethcall(name = "init", abi = "init(bytes)")]
    pub struct InitCall {
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `internalPrice` function with signature `internalPrice(uint256,uint256)` and selector `0xb8f00b72`
    #[derive(
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
    #[ethcall(name = "internalPrice", abi = "internalPrice(uint256,uint256)")]
    pub struct InternalPriceCall {
        pub reserve_x_wad: ::ethers::core::types::U256,
        pub total_liquidity: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `lx` function with signature `lx(uint256,uint256,(uint256,uint256,uint256))` and selector `0x9d819e83`
    #[derive(
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
    #[ethcall(name = "lx", abi = "lx(uint256,uint256,(uint256,uint256,uint256))")]
    pub struct LxCall {
        pub reserve_x_wad: ::ethers::core::types::U256,
        pub price_wad: ::ethers::core::types::U256,
        pub params: Parameters,
    }
    ///Container type for all input parameters for the `ly` function with signature `ly(uint256,uint256,(uint256,uint256,uint256))` and selector `0x5608bea1`
    #[derive(
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
    #[ethcall(name = "ly", abi = "ly(uint256,uint256,(uint256,uint256,uint256))")]
    pub struct LyCall {
        pub reserve_y_wad: ::ethers::core::types::U256,
        pub price_wad: ::ethers::core::types::U256,
        pub params: Parameters,
    }
    ///Container type for all input parameters for the `mulidiv` function with signature `mulidiv(int256,int256,int256)` and selector `0xe4937428`
    #[derive(
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
    #[ethcall(name = "mulidiv", abi = "mulidiv(int256,int256,int256)")]
    pub struct MulidivCall {
        pub x: ::ethers::core::types::I256,
        pub y: ::ethers::core::types::I256,
        pub denominator: ::ethers::core::types::I256,
    }
    ///Container type for all input parameters for the `mulidivUp` function with signature `mulidivUp(int256,int256,int256)` and selector `0xffb36266`
    #[derive(
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
    #[ethcall(name = "mulidivUp", abi = "mulidivUp(int256,int256,int256)")]
    pub struct MulidivUpCall {
        pub x: ::ethers::core::types::I256,
        pub y: ::ethers::core::types::I256,
        pub denominator: ::ethers::core::types::I256,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
    )]
    #[ethcall(name = "sigma", abi = "sigma()")]
    pub struct SigmaCall;
    ///Container type for all input parameters for the `simulateSwap` function with signature `simulateSwap(bool,uint256)` and selector `0x0ac33034`
    #[derive(
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
    #[ethcall(name = "simulateSwap", abi = "simulateSwap(bool,uint256)")]
    pub struct SimulateSwapCall {
        pub swap_x_in: bool,
        pub amount_in: ::ethers::core::types::U256,
    }
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
    )]
    #[ethcall(name = "tau", abi = "tau()")]
    pub struct TauCall;
    ///Container type for all input parameters for the `toUint` function with signature `toUint(int256)` and selector `0x1d9cf722`
    #[derive(
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
    #[ethcall(name = "toUint", abi = "toUint(int256)")]
    pub struct ToUintCall {
        pub x: ::ethers::core::types::I256,
    }
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
        Hash,
    )]
    #[ethcall(name = "validate", abi = "validate(bytes)")]
    pub struct ValidateCall {
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `xl` function with signature `xl(uint256,uint256,(uint256,uint256,uint256))` and selector `0x184d30ba`
    #[derive(
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
    #[ethcall(name = "xl", abi = "xl(uint256,uint256,(uint256,uint256,uint256))")]
    pub struct XlCall {
        pub total_liquidity: ::ethers::core::types::U256,
        pub price_wad: ::ethers::core::types::U256,
        pub params: Parameters,
    }
    ///Container type for all input parameters for the `yl` function with signature `yl(uint256,uint256,(uint256,uint256,uint256))` and selector `0xdbaf1142`
    #[derive(
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
    #[ethcall(name = "yl", abi = "yl(uint256,uint256,(uint256,uint256,uint256))")]
    pub struct YlCall {
        pub total_liquidity: ::ethers::core::types::U256,
        pub price_wad: ::ethers::core::types::U256,
        pub params: Parameters,
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
        Hash,
    )]
    pub enum LogNormalCalls {
        ApproximatedMinimumXInput(ApproximatedMinimumXInputCall),
        BisectionEpsilon(BisectionEpsilonCall),
        HalfWad(HalfWadCall),
        InfinityIsNotReal(InfinityIsNotRealCall),
        MaxBisectionIters(MaxBisectionItersCall),
        TwoWad(TwoWadCall),
        Wad(WadCall),
        Zero(ZeroCall),
        Slot(SlotCall),
        ComputeD1(ComputeD1Call),
        ComputeD2(ComputeD2Call),
        ComputeHalfSigmaSquared(ComputeHalfSigmaSquaredCall),
        ComputePrice(ComputePriceCall),
        ComputeSwapConstant(ComputeSwapConstantCall),
        DynamicSlot(DynamicSlotCall),
        EncodeInitData(EncodeInitDataCall),
        EncodeValidateData(EncodeValidateDataCall),
        FindLiquidity(FindLiquidityCall),
        FindX(FindXCall),
        FindY(FindYCall),
        GetNextLiquidity(GetNextLiquidityCall),
        GetParams(GetParamsCall),
        Init(InitCall),
        InternalPrice(InternalPriceCall),
        Lx(LxCall),
        Ly(LyCall),
        Mulidiv(MulidivCall),
        MulidivUp(MulidivUpCall),
        SetSigma(SetSigmaCall),
        SetStrikePrice(SetStrikePriceCall),
        SetTau(SetTauCall),
        Sigma(SigmaCall),
        SimulateSwap(SimulateSwapCall),
        StaticSlot(StaticSlotCall),
        StrikePrice(StrikePriceCall),
        SwapFeePercentageWad(SwapFeePercentageWadCall),
        TargetSigma(TargetSigmaCall),
        TargetStrike(TargetStrikeCall),
        Tau(TauCall),
        ToUint(ToUintCall),
        Validate(ValidateCall),
        Xl(XlCall),
        Yl(YlCall),
    }
    impl ::ethers::core::abi::AbiDecode for LogNormalCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <ApproximatedMinimumXInputCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ApproximatedMinimumXInput(decoded));
            }
            if let Ok(decoded) =
                <BisectionEpsilonCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::BisectionEpsilon(decoded));
            }
            if let Ok(decoded) = <HalfWadCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::HalfWad(decoded));
            }
            if let Ok(decoded) =
                <InfinityIsNotRealCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InfinityIsNotReal(decoded));
            }
            if let Ok(decoded) =
                <MaxBisectionItersCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::MaxBisectionIters(decoded));
            }
            if let Ok(decoded) = <TwoWadCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TwoWad(decoded));
            }
            if let Ok(decoded) = <WadCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Wad(decoded));
            }
            if let Ok(decoded) = <ZeroCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Zero(decoded));
            }
            if let Ok(decoded) = <SlotCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Slot(decoded));
            }
            if let Ok(decoded) = <ComputeD1Call as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ComputeD1(decoded));
            }
            if let Ok(decoded) = <ComputeD2Call as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ComputeD2(decoded));
            }
            if let Ok(decoded) =
                <ComputeHalfSigmaSquaredCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ComputeHalfSigmaSquared(decoded));
            }
            if let Ok(decoded) = <ComputePriceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ComputePrice(decoded));
            }
            if let Ok(decoded) =
                <ComputeSwapConstantCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ComputeSwapConstant(decoded));
            }
            if let Ok(decoded) = <DynamicSlotCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DynamicSlot(decoded));
            }
            if let Ok(decoded) =
                <EncodeInitDataCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::EncodeInitData(decoded));
            }
            if let Ok(decoded) =
                <EncodeValidateDataCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::EncodeValidateData(decoded));
            }
            if let Ok(decoded) = <FindLiquidityCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::FindLiquidity(decoded));
            }
            if let Ok(decoded) = <FindXCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::FindX(decoded));
            }
            if let Ok(decoded) = <FindYCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::FindY(decoded));
            }
            if let Ok(decoded) =
                <GetNextLiquidityCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetNextLiquidity(decoded));
            }
            if let Ok(decoded) = <GetParamsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetParams(decoded));
            }
            if let Ok(decoded) = <InitCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Init(decoded));
            }
            if let Ok(decoded) = <InternalPriceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InternalPrice(decoded));
            }
            if let Ok(decoded) = <LxCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Lx(decoded));
            }
            if let Ok(decoded) = <LyCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Ly(decoded));
            }
            if let Ok(decoded) = <MulidivCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Mulidiv(decoded));
            }
            if let Ok(decoded) = <MulidivUpCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MulidivUp(decoded));
            }
            if let Ok(decoded) = <SetSigmaCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetSigma(decoded));
            }
            if let Ok(decoded) =
                <SetStrikePriceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetStrikePrice(decoded));
            }
            if let Ok(decoded) = <SetTauCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetTau(decoded));
            }
            if let Ok(decoded) = <SigmaCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Sigma(decoded));
            }
            if let Ok(decoded) = <SimulateSwapCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SimulateSwap(decoded));
            }
            if let Ok(decoded) = <StaticSlotCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::StaticSlot(decoded));
            }
            if let Ok(decoded) = <StrikePriceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::StrikePrice(decoded));
            }
            if let Ok(decoded) =
                <SwapFeePercentageWadCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SwapFeePercentageWad(decoded));
            }
            if let Ok(decoded) = <TargetSigmaCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TargetSigma(decoded));
            }
            if let Ok(decoded) = <TargetStrikeCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TargetStrike(decoded));
            }
            if let Ok(decoded) = <TauCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Tau(decoded));
            }
            if let Ok(decoded) = <ToUintCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ToUint(decoded));
            }
            if let Ok(decoded) = <ValidateCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Validate(decoded));
            }
            if let Ok(decoded) = <XlCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Xl(decoded));
            }
            if let Ok(decoded) = <YlCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Yl(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for LogNormalCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::ApproximatedMinimumXInput(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BisectionEpsilon(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::HalfWad(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InfinityIsNotReal(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MaxBisectionIters(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TwoWad(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Wad(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Zero(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Slot(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ComputeD1(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ComputeD2(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ComputeHalfSigmaSquared(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ComputePrice(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ComputeSwapConstant(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DynamicSlot(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::EncodeInitData(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::EncodeValidateData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FindLiquidity(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::FindX(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::FindY(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetNextLiquidity(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetParams(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Init(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InternalPrice(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Lx(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Ly(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Mulidiv(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MulidivUp(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetSigma(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetStrikePrice(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetTau(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Sigma(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SimulateSwap(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::StaticSlot(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::StrikePrice(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SwapFeePercentageWad(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TargetSigma(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TargetStrike(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Tau(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ToUint(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Validate(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Xl(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Yl(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for LogNormalCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ApproximatedMinimumXInput(element) => ::core::fmt::Display::fmt(element, f),
                Self::BisectionEpsilon(element) => ::core::fmt::Display::fmt(element, f),
                Self::HalfWad(element) => ::core::fmt::Display::fmt(element, f),
                Self::InfinityIsNotReal(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaxBisectionIters(element) => ::core::fmt::Display::fmt(element, f),
                Self::TwoWad(element) => ::core::fmt::Display::fmt(element, f),
                Self::Wad(element) => ::core::fmt::Display::fmt(element, f),
                Self::Zero(element) => ::core::fmt::Display::fmt(element, f),
                Self::Slot(element) => ::core::fmt::Display::fmt(element, f),
                Self::ComputeD1(element) => ::core::fmt::Display::fmt(element, f),
                Self::ComputeD2(element) => ::core::fmt::Display::fmt(element, f),
                Self::ComputeHalfSigmaSquared(element) => ::core::fmt::Display::fmt(element, f),
                Self::ComputePrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::ComputeSwapConstant(element) => ::core::fmt::Display::fmt(element, f),
                Self::DynamicSlot(element) => ::core::fmt::Display::fmt(element, f),
                Self::EncodeInitData(element) => ::core::fmt::Display::fmt(element, f),
                Self::EncodeValidateData(element) => ::core::fmt::Display::fmt(element, f),
                Self::FindLiquidity(element) => ::core::fmt::Display::fmt(element, f),
                Self::FindX(element) => ::core::fmt::Display::fmt(element, f),
                Self::FindY(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetNextLiquidity(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetParams(element) => ::core::fmt::Display::fmt(element, f),
                Self::Init(element) => ::core::fmt::Display::fmt(element, f),
                Self::InternalPrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::Lx(element) => ::core::fmt::Display::fmt(element, f),
                Self::Ly(element) => ::core::fmt::Display::fmt(element, f),
                Self::Mulidiv(element) => ::core::fmt::Display::fmt(element, f),
                Self::MulidivUp(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetSigma(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetStrikePrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetTau(element) => ::core::fmt::Display::fmt(element, f),
                Self::Sigma(element) => ::core::fmt::Display::fmt(element, f),
                Self::SimulateSwap(element) => ::core::fmt::Display::fmt(element, f),
                Self::StaticSlot(element) => ::core::fmt::Display::fmt(element, f),
                Self::StrikePrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::SwapFeePercentageWad(element) => ::core::fmt::Display::fmt(element, f),
                Self::TargetSigma(element) => ::core::fmt::Display::fmt(element, f),
                Self::TargetStrike(element) => ::core::fmt::Display::fmt(element, f),
                Self::Tau(element) => ::core::fmt::Display::fmt(element, f),
                Self::ToUint(element) => ::core::fmt::Display::fmt(element, f),
                Self::Validate(element) => ::core::fmt::Display::fmt(element, f),
                Self::Xl(element) => ::core::fmt::Display::fmt(element, f),
                Self::Yl(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ApproximatedMinimumXInputCall> for LogNormalCalls {
        fn from(value: ApproximatedMinimumXInputCall) -> Self {
            Self::ApproximatedMinimumXInput(value)
        }
    }
    impl ::core::convert::From<BisectionEpsilonCall> for LogNormalCalls {
        fn from(value: BisectionEpsilonCall) -> Self {
            Self::BisectionEpsilon(value)
        }
    }
    impl ::core::convert::From<HalfWadCall> for LogNormalCalls {
        fn from(value: HalfWadCall) -> Self {
            Self::HalfWad(value)
        }
    }
    impl ::core::convert::From<InfinityIsNotRealCall> for LogNormalCalls {
        fn from(value: InfinityIsNotRealCall) -> Self {
            Self::InfinityIsNotReal(value)
        }
    }
    impl ::core::convert::From<MaxBisectionItersCall> for LogNormalCalls {
        fn from(value: MaxBisectionItersCall) -> Self {
            Self::MaxBisectionIters(value)
        }
    }
    impl ::core::convert::From<TwoWadCall> for LogNormalCalls {
        fn from(value: TwoWadCall) -> Self {
            Self::TwoWad(value)
        }
    }
    impl ::core::convert::From<WadCall> for LogNormalCalls {
        fn from(value: WadCall) -> Self {
            Self::Wad(value)
        }
    }
    impl ::core::convert::From<ZeroCall> for LogNormalCalls {
        fn from(value: ZeroCall) -> Self {
            Self::Zero(value)
        }
    }
    impl ::core::convert::From<SlotCall> for LogNormalCalls {
        fn from(value: SlotCall) -> Self {
            Self::Slot(value)
        }
    }
    impl ::core::convert::From<ComputeD1Call> for LogNormalCalls {
        fn from(value: ComputeD1Call) -> Self {
            Self::ComputeD1(value)
        }
    }
    impl ::core::convert::From<ComputeD2Call> for LogNormalCalls {
        fn from(value: ComputeD2Call) -> Self {
            Self::ComputeD2(value)
        }
    }
    impl ::core::convert::From<ComputeHalfSigmaSquaredCall> for LogNormalCalls {
        fn from(value: ComputeHalfSigmaSquaredCall) -> Self {
            Self::ComputeHalfSigmaSquared(value)
        }
    }
    impl ::core::convert::From<ComputePriceCall> for LogNormalCalls {
        fn from(value: ComputePriceCall) -> Self {
            Self::ComputePrice(value)
        }
    }
    impl ::core::convert::From<ComputeSwapConstantCall> for LogNormalCalls {
        fn from(value: ComputeSwapConstantCall) -> Self {
            Self::ComputeSwapConstant(value)
        }
    }
    impl ::core::convert::From<DynamicSlotCall> for LogNormalCalls {
        fn from(value: DynamicSlotCall) -> Self {
            Self::DynamicSlot(value)
        }
    }
    impl ::core::convert::From<EncodeInitDataCall> for LogNormalCalls {
        fn from(value: EncodeInitDataCall) -> Self {
            Self::EncodeInitData(value)
        }
    }
    impl ::core::convert::From<EncodeValidateDataCall> for LogNormalCalls {
        fn from(value: EncodeValidateDataCall) -> Self {
            Self::EncodeValidateData(value)
        }
    }
    impl ::core::convert::From<FindLiquidityCall> for LogNormalCalls {
        fn from(value: FindLiquidityCall) -> Self {
            Self::FindLiquidity(value)
        }
    }
    impl ::core::convert::From<FindXCall> for LogNormalCalls {
        fn from(value: FindXCall) -> Self {
            Self::FindX(value)
        }
    }
    impl ::core::convert::From<FindYCall> for LogNormalCalls {
        fn from(value: FindYCall) -> Self {
            Self::FindY(value)
        }
    }
    impl ::core::convert::From<GetNextLiquidityCall> for LogNormalCalls {
        fn from(value: GetNextLiquidityCall) -> Self {
            Self::GetNextLiquidity(value)
        }
    }
    impl ::core::convert::From<GetParamsCall> for LogNormalCalls {
        fn from(value: GetParamsCall) -> Self {
            Self::GetParams(value)
        }
    }
    impl ::core::convert::From<InitCall> for LogNormalCalls {
        fn from(value: InitCall) -> Self {
            Self::Init(value)
        }
    }
    impl ::core::convert::From<InternalPriceCall> for LogNormalCalls {
        fn from(value: InternalPriceCall) -> Self {
            Self::InternalPrice(value)
        }
    }
    impl ::core::convert::From<LxCall> for LogNormalCalls {
        fn from(value: LxCall) -> Self {
            Self::Lx(value)
        }
    }
    impl ::core::convert::From<LyCall> for LogNormalCalls {
        fn from(value: LyCall) -> Self {
            Self::Ly(value)
        }
    }
    impl ::core::convert::From<MulidivCall> for LogNormalCalls {
        fn from(value: MulidivCall) -> Self {
            Self::Mulidiv(value)
        }
    }
    impl ::core::convert::From<MulidivUpCall> for LogNormalCalls {
        fn from(value: MulidivUpCall) -> Self {
            Self::MulidivUp(value)
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
    impl ::core::convert::From<SimulateSwapCall> for LogNormalCalls {
        fn from(value: SimulateSwapCall) -> Self {
            Self::SimulateSwap(value)
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
    impl ::core::convert::From<ToUintCall> for LogNormalCalls {
        fn from(value: ToUintCall) -> Self {
            Self::ToUint(value)
        }
    }
    impl ::core::convert::From<ValidateCall> for LogNormalCalls {
        fn from(value: ValidateCall) -> Self {
            Self::Validate(value)
        }
    }
    impl ::core::convert::From<XlCall> for LogNormalCalls {
        fn from(value: XlCall) -> Self {
            Self::Xl(value)
        }
    }
    impl ::core::convert::From<YlCall> for LogNormalCalls {
        fn from(value: YlCall) -> Self {
            Self::Yl(value)
        }
    }
    ///Container type for all return fields from the `APPROXIMATED_MINIMUM_X_INPUT` function with signature `APPROXIMATED_MINIMUM_X_INPUT()` and selector `0x84c82a26`
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
    pub struct ApproximatedMinimumXInputReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `BISECTION_EPSILON` function with signature `BISECTION_EPSILON()` and selector `0x6d652299`
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
    pub struct BisectionEpsilonReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `HALF_WAD` function with signature `HALF_WAD()` and selector `0x85aa454e`
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
    pub struct HalfWadReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `INFINITY_IS_NOT_REAL` function with signature `INFINITY_IS_NOT_REAL()` and selector `0xe3d0dca5`
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
    pub struct InfinityIsNotRealReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `MAX_BISECTION_ITERS` function with signature `MAX_BISECTION_ITERS()` and selector `0xf9c28211`
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
    pub struct MaxBisectionItersReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `TWO_WAD` function with signature `TWO_WAD()` and selector `0x275d67c8`
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
    pub struct TwoWadReturn(pub ::ethers::core::types::I256);
    ///Container type for all return fields from the `WAD` function with signature `WAD()` and selector `0x6a146024`
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
    pub struct WadReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `ZERO` function with signature `ZERO()` and selector `0x58fa63ca`
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
    pub struct ZeroReturn(pub ::ethers::core::types::U256);
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
        Hash,
    )]
    pub struct SlotReturn {
        pub strike_price_wad: ::ethers::core::types::U256,
        pub sigma_percent_wad: ::ethers::core::types::U256,
        pub tau_years_wad: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `computeD1` function with signature `computeD1(uint256,(uint256,uint256,uint256))` and selector `0x9716aebb`
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
    pub struct ComputeD1Return {
        pub d_1: ::ethers::core::types::I256,
    }
    ///Container type for all return fields from the `computeD2` function with signature `computeD2(uint256,(uint256,uint256,uint256))` and selector `0xd7f97bef`
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
    pub struct ComputeD2Return {
        pub d_2: ::ethers::core::types::I256,
    }
    ///Container type for all return fields from the `computeHalfSigmaSquared` function with signature `computeHalfSigmaSquared(uint256)` and selector `0xa6d33498`
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
    pub struct ComputeHalfSigmaSquaredReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `computePrice` function with signature `computePrice(uint256,uint256,uint256,uint256,uint256)` and selector `0x0625a623`
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
    pub struct ComputePriceReturn {
        pub price: ::ethers::core::types::U256,
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
        Hash,
    )]
    pub struct ComputeSwapConstantReturn(pub ::ethers::core::types::I256);
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
        Hash,
    )]
    pub struct DynamicSlotReturn {
        pub params: Parameters,
    }
    ///Container type for all return fields from the `encodeInitData` function with signature `encodeInitData(uint256,uint256,uint256,(uint256,uint256,uint256))` and selector `0x0f0aa395`
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
    pub struct EncodeInitDataReturn(pub ::ethers::core::types::Bytes);
    ///Container type for all return fields from the `encodeValidateData` function with signature `encodeValidateData(uint256,uint256,uint256)` and selector `0xbd422d28`
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
    pub struct EncodeValidateDataReturn(pub ::ethers::core::types::Bytes);
    ///Container type for all return fields from the `findLiquidity` function with signature `findLiquidity(uint256,uint256,int256,(uint256,uint256,uint256))` and selector `0x883b6dc5`
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
    pub struct FindLiquidityReturn {
        pub liquidity: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `findX` function with signature `findX(uint256,uint256,int256,(uint256,uint256,uint256))` and selector `0xcd64aea2`
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
    pub struct FindXReturn {
        pub reserve_y: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `findY` function with signature `findY(uint256,uint256,int256,(uint256,uint256,uint256))` and selector `0x6417d4b5`
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
    pub struct FindYReturn {
        pub reserve_y: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `getNextLiquidity` function with signature `getNextLiquidity(uint256,uint256,uint256)` and selector `0xec29d8e6`
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
        Hash,
    )]
    pub struct GetParamsReturn(
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
        Hash,
    )]
    pub struct InitReturn {
        pub valid: bool,
        pub swap_constant_growth: ::ethers::core::types::I256,
        pub reserve_x_wad: ::ethers::core::types::U256,
        pub reserve_y_wad: ::ethers::core::types::U256,
        pub total_liquidity: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `internalPrice` function with signature `internalPrice(uint256,uint256)` and selector `0xb8f00b72`
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
    pub struct InternalPriceReturn {
        pub price: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `lx` function with signature `lx(uint256,uint256,(uint256,uint256,uint256))` and selector `0x9d819e83`
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
    pub struct LxReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `ly` function with signature `ly(uint256,uint256,(uint256,uint256,uint256))` and selector `0x5608bea1`
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
    pub struct LyReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `mulidiv` function with signature `mulidiv(int256,int256,int256)` and selector `0xe4937428`
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
    pub struct MulidivReturn {
        pub z: ::ethers::core::types::I256,
    }
    ///Container type for all return fields from the `mulidivUp` function with signature `mulidivUp(int256,int256,int256)` and selector `0xffb36266`
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
    pub struct MulidivUpReturn {
        pub z: ::ethers::core::types::I256,
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
        Hash,
    )]
    pub struct SigmaReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `simulateSwap` function with signature `simulateSwap(bool,uint256)` and selector `0x0ac33034`
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
    pub struct SimulateSwapReturn(
        pub bool,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::Bytes,
    );
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
    )]
    pub struct TauReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `toUint` function with signature `toUint(int256)` and selector `0x1d9cf722`
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
    pub struct ToUintReturn(pub ::ethers::core::types::U256);
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
        Hash,
    )]
    pub struct ValidateReturn {
        pub valid: bool,
        pub swap_constant_growth: ::ethers::core::types::I256,
        pub liquidity_delta: ::ethers::core::types::I256,
        pub adjusted_reserve_x_wad: ::ethers::core::types::U256,
        pub adjusted_reserve_y_wad: ::ethers::core::types::U256,
        pub adjusted_liquidity: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `xl` function with signature `xl(uint256,uint256,(uint256,uint256,uint256))` and selector `0x184d30ba`
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
    pub struct XlReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `yl` function with signature `yl(uint256,uint256,(uint256,uint256,uint256))` and selector `0xdbaf1142`
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
    pub struct YlReturn(pub ::ethers::core::types::U256);
    ///`Parameters(uint256,uint256,uint256)`
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
    pub struct Parameters {
        pub strike_price_wad: ::ethers::core::types::U256,
        pub sigma_percent_wad: ::ethers::core::types::U256,
        pub tau_years_wad: ::ethers::core::types::U256,
    }
}
