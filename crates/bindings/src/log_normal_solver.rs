pub use log_normal_solver::*;
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
pub mod log_normal_solver {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_strategy"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("BISECTION_EPSILON"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("BISECTION_EPSILON"),
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
                    ::std::borrow::ToOwned::to_owned("MAX_BISECTION_ITERS"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "MAX_BISECTION_ITERS",
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
                    ::std::borrow::ToOwned::to_owned("allocateGivenX"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("allocateGivenX"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
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
                    ::std::borrow::ToOwned::to_owned("allocateGivenY"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("allocateGivenY"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
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
                    ::std::borrow::ToOwned::to_owned("calculateDiffLower"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("calculateDiffLower"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("S"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("v"),
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
                    ::std::borrow::ToOwned::to_owned("calculateDiffRaise"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("calculateDiffRaise"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("S"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("v"),
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
                    ::std::borrow::ToOwned::to_owned("callIerfc"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("callIerfc"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("x"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
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
                    ::std::borrow::ToOwned::to_owned("computeDeltaL"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("computeDeltaL"),
                            inputs: ::std::vec![
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
                                    name: ::std::borrow::ToOwned::to_owned("swapFee"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("L"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("reserve"),
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
                                    name: ::std::borrow::ToOwned::to_owned("deltaL"),
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
                    ::std::borrow::ToOwned::to_owned("computeDeltaLXIn"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("computeDeltaLXIn"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
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
                                    name: ::std::borrow::ToOwned::to_owned("rx"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("ry"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("L"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("params"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct LogNormal.LogNormalParams",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("deltaL"),
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
                    ::std::borrow::ToOwned::to_owned("computeDeltaLYIn"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("computeDeltaLYIn"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
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
                                    name: ::std::borrow::ToOwned::to_owned("rx"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("ry"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("L"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("params"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct LogNormal.LogNormalParams",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("deltaL"),
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
                    ::std::borrow::ToOwned::to_owned("computeOptimalArbLowerPrice"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "computeOptimalArbLowerPrice",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("S"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("vUpper"),
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
                    ::std::borrow::ToOwned::to_owned("computeOptimalArbRaisePrice"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "computeOptimalArbRaisePrice",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("S"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("vUpper"),
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
                    ::std::borrow::ToOwned::to_owned("deallocateGivenX"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("deallocateGivenX"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
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
                    ::std::borrow::ToOwned::to_owned("deallocateGivenY"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("deallocateGivenY"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
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
                    ::std::borrow::ToOwned::to_owned("fetchPoolParams"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("fetchPoolParams"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct LogNormal.LogNormalParams",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getDxGivenS"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getDxGivenS"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("S"),
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
                    ::std::borrow::ToOwned::to_owned("getDyGivenS"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getDyGivenS"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("S"),
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
                    ::std::borrow::ToOwned::to_owned("getInitialPoolData"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getInitialPoolData"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("rx"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("S"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("params"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct LogNormal.LogNormalParams",
                                        ),
                                    ),
                                },
                            ],
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
                    ::std::borrow::ToOwned::to_owned("getNextLiquidity"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getNextLiquidity"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("rx"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("ry"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("L"),
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
                    ::std::borrow::ToOwned::to_owned("getNextReserveX"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getNextReserveX"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("ry"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("L"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("S"),
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
                    ::std::borrow::ToOwned::to_owned("getNextReserveY"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getNextReserveY"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("rx"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("L"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("S"),
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
                    ::std::borrow::ToOwned::to_owned("getPriceGivenXL"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getPriceGivenXL"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("rx"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("L"),
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
                    ::std::borrow::ToOwned::to_owned("getPriceGivenYL"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getPriceGivenYL"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("ry"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("L"),
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
                    ::std::borrow::ToOwned::to_owned("getReservesAndLiquidity"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getReservesAndLiquidity",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
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
                    ::std::borrow::ToOwned::to_owned("internalPrice"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("internalPrice"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
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
                    ::std::borrow::ToOwned::to_owned("prepareControllerUpdate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "prepareControllerUpdate",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("controller"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("prepareFeeUpdate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("prepareFeeUpdate"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("swapFee"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("prepareSigmaUpdate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("prepareSigmaUpdate"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("targetSigma"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("targetTimestamp"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("prepareStrikeUpdate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "prepareStrikeUpdate",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("targetStrike"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("targetTimestamp"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("prepareTauUpdate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("prepareTauUpdate"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("targetTau"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("targetTimestamp"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
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
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("swapXIn"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
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
                    ::std::borrow::ToOwned::to_owned("solverTradingFunction"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "solverTradingFunction",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("rx"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("ry"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("L"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("params"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct LogNormal.LogNormalParams",
                                        ),
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
                    ::std::borrow::ToOwned::to_owned("strategy"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("strategy"),
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
            ]),
            events: ::std::collections::BTreeMap::new(),
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
    pub static LOGNORMALSOLVER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15b\0\0^W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`@Qb\0D\xDC8\x03\x80b\0D\xDC\x839\x81\x01`@\x81\x90Rb\0\0\x81\x91b\0\0\xA7V[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90Ub\0\x01$V[`\0` \x82\x84\x03\x12\x15b\0\x01\x05W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x01\x1DW`\0\x80\xFD[\x93\x92PPPV[aC\xA8\x80b\0\x014`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x02HW`\x005`\xE0\x1C\x80cme\"\x99\x11a\x01gW\x80c\xB0\x9D\x04\xE5\x11a\0\xFAW\x80c\xE4]Y<\x11a\0\xC9W\x80c\xE4]Y<\x14a\x05*W\x80c\xE9G\x16\xD5\x14a\x05=W\x80c\xEE>\x8C\xFB\x14a\x05PW\x80c\xF3\r7\xF2\x14a\x05cW\x80c\xF9\xC2\x82\x11\x14a\x05vWa\x02HV[\x80c\xB0\x9D\x04\xE5\x14a\x04\xDEW\x80c\xC5\xE71i\x14a\x04\xF1W\x80c\xCB\x1FU2\x14a\x05\x04W\x80c\xCE\x15;\xF4\x14a\x05\x17Wa\x02HV[\x80c\x9An\xC1b\x11a\x016W\x80c\x9An\xC1b\x14a\x04zW\x80c\x9B\xDC\x03\x1C\x14a\x04\x8DW\x80c\xA8\xC6.v\x14a\x04\xA0W\x80c\xAFNC\x7F\x14a\x04\xCBWa\x02HV[\x80cme\"\x99\x14a\x04,W\x80c\x7F\x17@\x9C\x14a\x044W\x80c\x81\xB5\xFA\xC2\x14a\x04GW\x80c\x90.\xCA\xA2\x14a\x04gWa\x02HV[\x80c9(\xFF\x97\x11a\x01\xDFW\x80cO\xD6|X\x11a\x01\xAEW\x80cO\xD6|X\x14a\x03\xC5W\x80cU\xF0\x11\xC6\x14a\x03\xD8W\x80c^\xB4\x08\xFC\x14a\x03\xEBW\x80cb7V\x9F\x14a\x03\xFEWa\x02HV[\x80c9(\xFF\x97\x14a\x03iW\x80c;&\x8D]\x14a\x03\x8CW\x80c;M\x100\x14a\x03\x9FW\x80cN\x81\x7F\xD9\x14a\x03\xB2Wa\x02HV[\x80c\x1E\x97\x8C\xB0\x11a\x02\x1BW\x80c\x1E\x97\x8C\xB0\x14a\x03\x1DW\x80c$\xDE%s\x14a\x030W\x80c0m\xB4k\x14a\x03CW\x80c3\"f\xF3\x14a\x03VWa\x02HV[\x80c\x04 X\n\x14a\x02\xADW\x80c\x070U<\x14a\x02\xD6W\x80c\x12\x06I\xC5\x14a\x02\xF7W\x80c\x13N\xAD\x12\x14a\x03\nW[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x02\xC0a\x02\xBB6`\x04a:\rV[a\x05~V[`@Qa\x02\xCD\x91\x90a:\x82V[`@Q\x80\x91\x03\x90\xF3[a\x02\xE9a\x02\xE46`\x04a:\x95V[a\x05\x93V[`@Q\x90\x81R` \x01a\x02\xCDV[a\x02\xE9a\x03\x056`\x04a:\x95V[a\x05\xC6V[a\x02\xC0a\x03\x186`\x04a;\xFAV[a\x06\xEAV[a\x02\xE9a\x03+6`\x04a<3V[a\x07\x01V[a\x02\xE9a\x03>6`\x04a<bV[a\x07\x16V[a\x02\xE9a\x03Q6`\x04a<3V[a\t$V[a\x02\xE9a\x03d6`\x04a<3V[a\t[V[a\x03|a\x03w6`\x04a<\xB4V[a\t\x87V[`@Qa\x02\xCD\x94\x93\x92\x91\x90a<\xEFV[a\x02\xC0a\x03\x9A6`\x04a:\rV[a\x0F\x97V[a\x02\xE9a\x03\xAD6`\x04a=\x16V[a\x0F\xA3V[a\x02\xE9a\x03\xC06`\x04a<3V[a\x0F\xCCV[a\x02\xE9a\x03\xD36`\x04a<3V[a\x0F\xE1V[a\x02\xE9a\x03\xE66`\x04a:\rV[a\x10\rV[a\x02\xE9a\x03\xF96`\x04a:\x95V[a\x10BV[a\x04\x11a\x04\x0C6`\x04a:\rV[a\x11\x9FV[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x02\xCDV[a\x02\xE9`\0\x81V[a\x04\x11a\x04B6`\x04a:\rV[a\x11\xF9V[a\x04Za\x04U6`\x04a=\x16V[a\x12RV[`@Qa\x02\xCD\x91\x90a=2V[a\x02\xE9a\x04u6`\x04a<3V[a\x13^V[a\x02\xE9a\x04\x886`\x04a=pV[a\x13\x8AV[a\x02\xE9a\x04\x9B6`\x04a=pV[a\x13\xE1V[`\0Ta\x04\xB3\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\xCDV[a\x02\xE9a\x04\xD96`\x04a:\x95V[a\x14\nV[a\x02\xC0a\x04\xEC6`\x04a=\x16V[a\x15\x8EV[a\x02\xE9a\x04\xFF6`\x04a=\x16V[a\x15\x99V[a\x02\xC0a\x05\x126`\x04a=\xC6V[a\x15\xA4V[a\x04\x11a\x05%6`\x04a=\x16V[a\x15\xAFV[a\x02\xE9a\x0586`\x04a:\rV[a\x17?V[a\x02\xC0a\x05K6`\x04a:\rV[a\x17\xA4V[a\x04\x11a\x05^6`\x04a:\rV[a\x17\xB0V[a\x04\x11a\x05q6`\x04a:\rV[a\x17\xD6V[a\x02\xE9`x\x81V[``a\x05\x8A\x83\x83a\x17\xFCV[\x90P[\x92\x91PPV[`\0a\x05\xBDg\x06\xF0[Y\xD3\xB2\0\0a\x05\xB1\x84a\x05\xB7\x87\x83\x8B\x8Ba\x18+V[\x90a\x18+V[\x90a\x18@V[\x95\x94PPPPPV[`\0\x80a\x05\xDC\x84\x84a\x05\xD7\x89a\x12RV[a\x18UV[`@\x80Q` \x81\x01\x88\x90R\x80\x82\x01\x83\x90R``\x80\x82\x01\x88\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x80\x82\x01\x92\x83\x90R`\0\x80Tb.RK`\xE0\x1B\x90\x94R\x93\x94P\x92\x91`\x01`\x01`\xA0\x1B\x03\x16\x90b.RK\x90a\x069\x90\x8B\x90\x86\x90`\x84\x01a=\xE6V[` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x06\x8FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` aCS\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x06\xA3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xC7\x91\x90a=\xFFV[\x90Pa\x06\xDE\x87\x87\x83\x86a\x06\xD9\x8Da\x12RV[a\x18\x91V[\x98\x97PPPPPPPPV[``a\x06\xF7\x84\x84\x84a\x195V[\x90P[\x93\x92PPPV[`\0a\x06\xF7\x83\x83a\x07\x11\x87a\x12RV[a\x19\xA6V[`\0\x82\x85\x10a\x07lW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FtradingFunction: invalid x\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x80g\r\xE0\xB6\xB3\xA7d\0\0a\x07\x82\x88\x87a\x1A\xAAV[\x10a\x07\x96W`\x01`\x01`\xFF\x1B\x03\x91Pa\x07\xCDV[a\x07\xA8a\x07\xA3\x88\x87a\x1A\xAAV[a\x1A\xBFV[\x91Pa\x07\xCD`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01`a`\xF8\x1B\x81RP\x83a\x1B\\V[g\r\xE0\xB6\xB3\xA7d\0\0a\x07\xED\x87a\x07\xE8\x87`\0\x01Q\x89a\x1B\xA5V[a\x1A\xAAV[\x10a\x08VWa\x08$`@Q\x80`@\x01`@R\x80`\x11\x81R` \x01p\r-\xC4\r\xAC/\x04\r-\xCE\x84\x0CNL-\xCCm`{\x1B\x81RPa\x1B\xBAV[`\x01`\x01`\xFF\x1B\x03\x90Pa\x08Q`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01`1`\xF9\x1B\x81RP\x82a\x1B\\V[a\x08nV[a\x08ka\x07\xA3\x87a\x07\xE8\x87`\0\x01Q\x89a\x1B\xA5V[\x90P[`\0a\x08\x82\x85` \x01Q\x86`@\x01Qa\x1C\0V[\x90Pa\x08\xA7`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01`c`\xF8\x1B\x81RP\x82a\x1B\\V[`@\x80Q\x80\x82\x01\x90\x91R`\x05\x81Rda + c`\xD8\x1B` \x82\x01Ra\x08\xD6\x90a\x08\xD1\x83\x86a>1V[a\x1B\\V[`@\x80Q\x80\x82\x01\x90\x91R`\t\x81Rha + b + c`\xB8\x1B` \x82\x01Ra\t\x0F\x90\x82a\t\x05\x85\x87a>1V[a\x08\xD1\x91\x90a>1V[\x80a\t\x1A\x83\x85a>1V[a\x06\xDE\x91\x90a>1V[`\0\x80a\t0\x85a\x12RV[\x90P`\0\x80a\t>\x87a\x15\xAFV[\x92PP\x91Pa\tP\x86\x83\x83\x88\x87a\x1C&V[\x97\x96PPPPPPPV[`\0\x80a\tg\x85a\x12RV[\x90P`\0\x80a\tu\x87a\x15\xAFV[\x92PP\x91Pa\tP\x86\x83\x83\x88\x87a\x1C\x88V[`\0\x80`\0``a\t\xB9`@Q\x80`\x80\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[a\t\xE4`@Q\x80`\x80\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[a\t\xED\x89a\x15\xAFV[`@\x85\x01R` \x84\x01R\x82R`\0a\n\x04\x8Aa\x12RV[\x90P`\0a\n6`@Q\x80`@\x01`@R\x80`\x08\x81R` \x01g\x0En\x8C.N\x84\x0EO`\xC3\x1B\x81RP\x85`\0\x01Qa\x1D\x04V[a\nd`@Q\x80`@\x01`@R\x80`\x08\x81R` \x01gstart ry`\xC0\x1B\x81RP\x85` \x01Qa\x1D\x04V[a\n\x91`@Q\x80`@\x01`@R\x80`\x07\x81R` \x01f\x1C\xDD\x18\\\x9D\x08\x13`\xCA\x1B\x81RP\x85`@\x01Qa\x1D\x04V[a\n\xC2`@Q\x80`@\x01`@R\x80`\x10\x81R` \x01o\x18\xDB\xDB\\\x1D]\x1A[\x99\xC8\x1B\x99^\x1D\x08\x13`\x82\x1B\x81RPa\x1B\xBAV[`\0a\n\xDC\x8C\x86`\0\x01Q\x87` \x01Q\x88`@\x01Qa\x14\nV[\x90Pa\x0B\x0E`@Q\x80`@\x01`@R\x80`\x0F\x81R` \x01n\x18\xDB\xDB\\\x1D]\x19Y\x08\x1B\x99^\x1D\x08\x13`\x8A\x1B\x81RPa\x1B\xBAV[\x8A\x15a\x0C_W\x84Qa\x0B!\x90\x8B\x90a>YV[\x84R\x84Q` \x86\x01Q`@\x87\x01Qa\x0B=\x92\x8F\x92\x8E\x92\x88a\x13\x8AV[a\x0BG\x90\x82a>YV[`@\x85\x01\x81\x90R\x84Qa\x0B\\\x91\x8E\x91\x90a\x07\x01V[``\x85\x01R`@\x80Q\x80\x82\x01\x90\x91R`\x10\x81Rocomputing next Y`\x80\x1B` \x82\x01Ra\x0B\x91\x90a\x1B\xBAV[a\x0B\xA9\x8C\x85`\0\x01Q\x86`@\x01Q\x87``\x01Qa\x05\xC6V[\x84` \x01\x81\x81RPPa\x0B\xE2`@Q\x80`@\x01`@R\x80`\x0F\x81R` \x01ncomputed next Y`\x88\x1B\x81RPa\x1B\xBAV[\x84` \x01Q\x84` \x01Q\x10a\x0CDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: y reserve increase`D\x82\x01Rad!`\xF0\x1B`d\x82\x01R`\x84\x01a\x07cV[\x83` \x01Q\x85` \x01Qa\x0CX\x91\x90a>lV[\x91Pa\x0EUV[\x89\x85` \x01Qa\x0Co\x91\x90a>YV[\x84` \x01\x81\x81RPPa\x0C\x92\x8C\x8B\x87`\0\x01Q\x88` \x01Q\x89`@\x01Q\x88a\x13\xE1V[a\x0C\x9C\x90\x82a>YV[\x84`@\x01\x81\x81RPPa\x0C\xD1`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01eend ry`\xD0\x1B\x81RP\x85` \x01Qa\x1D\x04V[a\x0C\xFC`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01d\x19[\x99\x08\x13`\xDA\x1B\x81RP\x85`@\x01Qa\x1D\x04V[a\r\x0F\x8C\x85` \x01Q\x86`@\x01Qa\x0F\xCCV[``\x85\x01R`@\x80Q\x80\x82\x01\x90\x91R`\x10\x81Ro\x0Cm\xED\xAE\x0E\xAE\x8D-\xCC\xE4\r\xCC\xAF\x0E\x84\x0B`\x83\x1B` \x82\x01Ra\rD\x90a\x1B\xBAV[a\r\\\x8C\x85` \x01Q\x86`@\x01Q\x87``\x01Qa\x10BV[\x84R`@\x80Q\x80\x82\x01\x90\x91R`\x0F\x81Rn\x0Cm\xED\xAE\x0E\xAE\x8C\xAC\x84\r\xCC\xAF\x0E\x84\x0B`\x8B\x1B` \x82\x01Ra\r\x8D\x90a\x1B\xBAV[a\r\xBA`@Q\x80`@\x01`@R\x80`\x07\x81R` \x01f72\xBC:\x10<\x1D`\xC9\x1B\x81RP\x85`\0\x01Qa\x1D\x04V[a\r\xE8`@Q\x80`@\x01`@R\x80`\x08\x81R` \x01g9\xBA0\xB9:\x10<\x1D`\xC1\x1B\x81RP\x86`\0\x01Qa\x1D\x04V[\x84Q\x84Q\x10a\x0EDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: x reserve increase`D\x82\x01Rad!`\xF0\x1B`d\x82\x01R`\x84\x01a\x07cV[\x83Q\x85Qa\x0ER\x91\x90a>lV[\x91P[P\x82Q` \x80\x85\x01Q`@\x80\x87\x01Q\x81Q\x93\x84\x01\x94\x90\x94R\x82\x01R``\x81\x01\x91\x90\x91R`\0\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16ch\xBD>80\x8F\x85`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0E\xD5\x93\x92\x91\x90a>\x7FV[`\xC0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0F+W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` aCS\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0F?W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Fc\x91\x90a>\xA6V[PPPPP\x90P\x80\x83a\x0F\x7F\x87`\0\x01Q\x88`@\x01Q\x88a\x19\xA6V[\x84\x99P\x99P\x99P\x99PPPPPPP\x93P\x93P\x93P\x93V[``a\x05\x8A\x83\x83a\x1DIV[`\0\x80`\0a\x0F\xB1\x84a\x15\xAFV[\x92PP\x91Pa\x0F\xC4\x82\x82a\x07\x11\x87a\x12RV[\x94\x93PPPPV[`\0a\x06\xF7\x83\x83a\x0F\xDC\x87a\x12RV[a\x1DaV[`\0\x80a\x0F\xED\x85a\x12RV[\x90P`\0\x80a\x0F\xFB\x87a\x15\xAFV[\x92P\x92PPa\tP\x86\x83\x83\x88\x87a\x1E/V[`\0\x80a\x10\x19\x84a\x12RV[\x90P`\0\x80a\x10'\x86a\x15\xAFV[\x92PP\x91Pa\x108\x85\x83\x83\x86a\x1E\x91V[\x96\x95PPPPPPV[`\0\x80a\x10X\x84\x84a\x10S\x89a\x12RV[a\x1F%V[\x90Pa\x10\x99`@Q\x80`@\x01`@R\x80`\x1B\x81R` \x01\x7Fapproximated rx given price\0\0\0\0\0\x81RP\x82a\x1D\x04V[`@\x80Q` \x81\x01\x83\x90R\x90\x81\x01\x86\x90R``\x81\x01\x85\x90R`\0\x90`\x80\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R`\0\x80Tb.RK`\xE0\x1B\x84R\x91\x93P\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90b.RK\x90a\x10\xFA\x90\x8B\x90\x86\x90`\x04\x01a=\xE6V[` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x11PW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` aCS\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x11dW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\x88\x91\x90a=\xFFV[\x90Pa\x06\xDE\x87\x87\x83\x86a\x11\x9A\x8Da\x12RV[a\x1FjV[`\0\x80`\0\x80`\0a\x11\xB0\x87a\x15\xAFV[\x92PP\x91P`\0\x80a\x11\xC5`\0\x89\x86\x86a \xEAV[\x91P\x91P`\0a\x11\xD6\x8A\x84\x84a\x07\x01V[\x90P`\0a\x11\xE6\x8B\x85\x85\x85a\x05\xC6V[\x93\x9B\x93\x9AP\x91\x98P\x91\x96PPPPPPPV[`\0\x80`\0\x80`\0a\x12\n\x87a\x15\xAFV[\x92P\x92PP`\0\x80a\x12\x1F`\x01\x89\x86\x86a \xEAV[\x91P\x91P`\0a\x120\x8A\x84\x84a\x0F\xCCV[\x90P`\0a\x12@\x8B\x85\x85\x85a\x10BV[\x9B\x93\x9AP\x91\x98P\x91\x96PPPPPPPV[a\x12\x8D`@Q\x80`\xA0\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81RP\x90V[`\0T`@Qc\xDC\x17\x83U`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xDC\x17\x83U\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x13\x0FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` aCS\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x13#W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x13K\x91\x90\x81\x01\x90a>\xFCV[\x80` \x01\x90Q\x81\x01\x90a\x05\x8D\x91\x90a@\xCFV[`\0\x80a\x13j\x85a\x12RV[\x90P`\0\x80a\x13x\x87a\x15\xAFV[\x92P\x92PPa\tP\x86\x83\x83\x88\x87a!SV[`\0\x80a\x13\xA4\x87\x84``\x01Qa\x18+\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x13\xB3\x89\x88\x87a\x07\x01V[\x90Pa\x13\xD4\x86a\x13\xC3\x83\x8Aa\x1B\xA5V[a\x13\xCD\x91\x90a>YV[\x83\x90a\x1A\xAAV[\x99\x98PPPPPPPPPV[`\0\x80a\x13\xFB\x87\x84``\x01Qa\x18+\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x13\xB3\x89\x87\x87a\x0F\xCCV[`@\x80Q` \x81\x01\x85\x90R\x90\x81\x01\x83\x90R``\x81\x01\x82\x90R`\0\x90\x81\x90`\x80\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82\x01\x90\x91R`\x0B\x82Rj\x1C\x9E\x08\x1A[\x88\x1B\x99^\x1D\x13`\xAA\x1B` \x83\x01R\x91Pa\x14a\x90\x86a\x1D\x04V[a\x14\x8D`@Q\x80`@\x01`@R\x80`\n\x81R` \x01i\x13\x08\x1A[\x88\x1B\x99^\x1D\x13`\xB2\x1B\x81RP\x84a\x1D\x04V[`\0\x80T`@Qb.RK`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90b.RK\x90a\x14\xBE\x90\x8A\x90\x86\x90`\x04\x01a=\xE6V[` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x15\x14W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` aCS\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x15(W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15L\x91\x90a=\xFFV[\x90Pa\x15y`@Q\x80`@\x01`@R\x80`\t\x81R` \x01h\x1A[\x9D\x98\\\x9AX[\x9D`\xBA\x1B\x81RP\x82a\x1B\\V[a\tP\x86\x86\x83\x87a\x15\x89\x8Ca\x12RV[a!\xAFV[``a\x05\x8D\x82a\"\xA8V[`\0a\x05\x8D\x82a\"\xD4V[``a\x05\x8D\x82a\"\xDFV[`\0\x80`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xAF\xBA\x13\xC4`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x16?W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` aCS\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x16SW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16w\x91\x90a@\xEEV[`\x01`\x01`\xA0\x1B\x03\x16c\xCE\x15;\xF4\x85`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x16\xA4\x91\x81R` \x01\x90V[```@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x16\xFAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` aCS\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x17\x0EW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x172\x91\x90aA\x0EV[\x92P\x92P\x92P\x91\x93\x90\x92PV[`\0\x80a\x17K\x84a\x12RV[\x90P`\0\x80a\x17Y\x86a\x15\xAFV[\x92P\x92PP`\0a\x17l\x86\x84\x84\x87a\"\xF5V[\x90Pa\x108`@Q\x80`@\x01`@R\x80`\x14\x81R` \x01s====computed dy=====``\x1B\x81RP\x82a\x1B\\V[``a\x05\x8A\x83\x83a#\x82V[`\0\x80`\0\x80`\0a\x17\xC1\x87a\x15\xAFV[\x92PP\x91P`\0\x80a\x11\xC5`\x01\x89\x86\x86a \xEAV[`\0\x80`\0\x80`\0a\x17\xE7\x87a\x15\xAFV[\x92P\x92PP`\0\x80a\x12\x1F`\0\x89\x86\x86a \xEAV[```\x02\x83\x83`@Q` \x01a\x18\x14\x93\x92\x91\x90aAaV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x92\x91PPV[`\0a\x05\x8A\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a#\x9AV[`\0a\x05\x8A\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a#\x9AV[`\0\x80a\x18b\x84\x84a#\xC8V[\x90P`\0a\x18o\x82a$)V[\x90P`\0a\x18|\x82a$\x92V[\x85Q\x90\x91Pa\tP\x90\x82\x90a\x05\xB1\x90\x8Aa\x18+V[`\0\x82\x80\x85\x83\x81\x12\x15a\x18\xD1W[`\0\x81\x12\x15a\x18\xCCWa\x18\xB7\x83a\x03\xE9a\x03\xE8a#\x9AV[\x92Pa\x18\xC5\x89\x84\x8A\x88a$\xDBV[\x90Pa\x18\x9FV[a\x18\xFEV[`\0\x81\x13\x15a\x18\xFEWa\x18\xE9\x82a\x03\xE7a\x03\xE8a%\xCDV[\x91Pa\x18\xF7\x89\x83\x8A\x88a$\xDBV[\x90Pa\x18\xD1V[a\x13\xD4\x89\x89\x83\x88`@Q` \x01a\x19\x18\x94\x93\x92\x91\x90aA\x80V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83\x85`\x14`@a%\xECa&\x19V[```\0a\x19D\x85\x85\x85a'*V[\x90P`\0a\x19S\x82\x86\x86a\x18UV[\x90P`\0a\x19c\x87\x83\x85\x88a$\xDBV[\x90Pa\x19r\x87\x83\x83\x86\x89a!\xAFV[\x92P\x86\x82\x84\x87`@Q` \x01a\x19\x8B\x94\x93\x92\x91\x90aA\x80V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x93PPPP\x93\x92PPPV[`\0\x80a\x19\xBB\x83` \x01Q\x84`@\x01Qa\x1C\0V[\x90P`\0a\x19\xD1\x84` \x01Q\x85`@\x01Qa'oV[\x90P`\0a\x19\xEC\x85`@\x01Q\x83a\x1B\xA5\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x19\xFA\x88\x88a\x1A\xAAV[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0\x81\x12a\x1A\x18W`\0\x94PPPPPa\x06\xFAV[`\0\x81\x13a\x1A.W`\0\x19\x94PPPPPa\x06\xFAV[`\0a\x1AEa\x07\xA3\x83g\r\xE0\xB6\xB3\xA7d\0\0aA\xD4V[\x90P`\0\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x1A]\x88\x85aA\xFBV[a\x1Ag\x91\x90aBAV[a\x1Aq\x91\x90aA\xD4V[\x90P`\0a\x1A~\x82a'\xA2V[\x90P`\0a\x1A\x8B\x82a$\x92V[\x8AQ\x90\x91Pa\x1A\x9A\x90\x82a\x18+V[\x9C\x9BPPPPPPPPPPPPV[`\0a\x05\x8A\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a%\xCDV[`\0g\x06\xF0[Y\xD3\xB2\0\0\x82\x03a\x1A\xD8WP`\0\x91\x90PV[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x12a\x1B\0W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\x1B!W`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x1B.\x83`\x02aA\xFBV[\x90P`\0a\x1B;\x82a)KV[\x90P`\0a\x1BQg\x13\xA0K\xBD\xFD\xC9\xBE\x88\x83a+\xC9V[\x90Pa\x05\xBD\x81aBoV[a\x1B\xA1\x82\x82`@Q`$\x01a\x1Br\x92\x91\x90aB\x8BV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\x1ES\x13G`\xE1\x1B\x17\x90Ra+\xDEV[PPV[`\0a\x05\x8A\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a%\xCDV[a\x1B\xFD\x81`@Q`$\x01a\x1B\xCE\x91\x90a:\x82V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\x10L\x13\xEB`\xE2\x1B\x17\x90Ra+\xDEV[PV[`\0\x80a\x1C\x0C\x83a+\xFFV[a\x1C\x1A\x90c;\x9A\xCA\0aB\xADV[\x90Pa\x0F\xC4\x84\x82a\x1B\xA5V[`\0\x82`\x01\x82a\x1C9\x89\x89\x89\x85\x89a\x1C\x88V[\x90P`\0\x81\x12\x15a\x1CPW`\0\x93PPPPa\x05\xBDV[a\x13\xD4\x89\x89\x89\x88`@Q` \x01a\x1Cj\x94\x93\x92\x91\x90aA\x80V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83\x85`\x01a\x01\0a,\xA3a&\x19V[`\0\x80\x82``\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x1C\xA3\x91\x90aA\xD4V[\x90P`\0a\x1C\xB5\x88\x88\x88\x85\x89\x89a,\xD4V[\x90P`\0a\x1C\xC2\x82a./V[\x90P`\0a\x1C\xCF\x83a/^V[\x90P\x80\x82\x84a\x01\0\x01Qa\x1C\xE2\x90aBoV[a\x1C\xEC\x91\x90a>1V[a\x1C\xF6\x91\x90a>1V[\x9A\x99PPPPPPPPPPV[a\x1B\xA1\x82\x82`@Q`$\x01a\x1D\x1A\x92\x91\x90aB\x8BV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c-\x83\x9C\xB3`\xE2\x1B\x17\x90Ra+\xDEV[```\x04\x83\x83`@Q` \x01a\x18\x14\x93\x92\x91\x90aAaV[`\0\x80a\x1Dv\x83` \x01Q\x84`@\x01Qa\x1C\0V[\x90P`\0a\x1D\x8C\x84` \x01Q\x85`@\x01Qa'oV[\x90P`\0a\x1D\xA7\x85`@\x01Q\x83a\x1B\xA5\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x85Q\x90\x91P`\0\x90a\x1D\xC4\x90a\x1D\xBD\x90\x89a\x1B\xA5V[\x89\x90a\x1A\xAAV[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0\x81\x12a\x1D\xE2W`\0\x94PPPPPa\x06\xFAV[`\0\x81\x13a\x1D\xF8W`\0\x19\x94PPPPPa\x06\xFAV[`\0a\x1E\x03\x82a\x1A\xBFV[\x90P`\0\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x1E\x1B\x88\x85aA\xFBV[a\x1E%\x91\x90aBAV[a\x1Aq\x91\x90a>1V[`\0\x82`\x01\x82a\x1EB\x89\x89\x89\x85\x89a!SV[\x90P`\0\x81\x12\x15a\x1EYW`\0\x93PPPPa\x05\xBDV[a\x13\xD4\x89\x89\x89\x88`@Q` \x01a\x1Es\x94\x93\x92\x91\x90aA\x80V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83\x85`\x01a\x01\0a/\xEAa&\x19V[`\0\x80\x82``\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x1E\xAC\x91\x90aA\xD4V[\x90P`\0\x83` \x01Q\x90P`\0a\x1E\xC7\x88\x86`\0\x01Qa0\x1BV[\x90P`\0a\x1E\xF9a\x1E\xE0\x84g\x1B\xC1mgN\xC8\0\0a0/V[a\x1E\xEA\x84\x86a0/V[a\x1E\xF4\x91\x90a>1V[a$)V[\x90P`\0a\x1F\x19a\x1F\x12\x83g\r\xE0\xB6\xB3\xA7d\0\0aA\xD4V[\x89\x90a0SV[\x90Pa\x1C\xF6\x89\x82aA\xD4V[`\0\x80a\x1F2\x84\x84a0\x86V[\x90P`\0a\x1F?\x82a$)V[\x90P`\0a\x1FL\x82a$\x92V[\x90Pa\tPa\x1Fc\x82g\r\xE0\xB6\xB3\xA7d\0\0a>lV[\x88\x90a\x18+V[`@\x80Q\x80\x82\x01\x90\x91R`\x0E\x81Rm\x0C.\x0E\x0EM\xEF\r-\xAC.\x8C\xAC\x8AO`\x93\x1B` \x82\x01R`\0\x90\x83\x90\x81\x90a\x1F\xA0\x90\x82a\x1D\x04V[`@\x80Q\x80\x82\x01\x90\x91R`\x11\x81Rp\x18\xDB\xDB\\\x1D]\x19Y\x12[\x9D\x98\\\x9AX[\x9D`z\x1B` \x82\x01R\x86\x90a\x1F\xD4\x90\x82a\x1B\\V[`\0\x81\x12\x15a KW[`\0\x81\x12\x15a FWa\x1F\xF6\x83a\x03\xE9a\x03\xE8a#\x9AV[\x92Pa \"`@Q\x80`@\x01`@R\x80`\x08\x81R` \x01g9<\x10:\xB882\xB9`\xC1\x1B\x81RP\x84a\x1D\x04V[\x87\x83\x11a /W\x82a 1V[\x87[\x92Pa ?\x83\x8A\x8A\x88a$\xDBV[\x90Pa\x1F\xDEV[a \xB3V[`\0\x81\x13\x15a \xB3Wa c\x82a\x03\xE7a\x03\xE8a%\xCDV[\x91Pa \x8F`@Q\x80`@\x01`@R\x80`\x08\x81R` \x01g9<\x1067\xBB\xB2\xB9`\xC1\x1B\x81RP\x83a\x1D\x04V[\x87\x82\x11a \x9CW\x81a \x9EV[\x87[\x91Pa \xAC\x82\x8A\x8A\x88a$\xDBV[\x90Pa KV[a\x13\xD4\x89\x89\x83\x88`@Q` \x01a \xCD\x94\x93\x92\x91\x90aA\x80V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83\x85`\x14`@a0\xCBa&\x19V[`\0\x80\x80a \xF8\x84\x86a\x18@V[\x90P`\0a!\x06\x87\x83a\x18+V[\x90P\x87a!\x1CWa!\x17\x87\x87a>lV[a!&V[a!&\x87\x87a>YV[\x93P\x87a!<Wa!7\x81\x86a>lV[a!FV[a!F\x81\x86a>YV[\x92PPP\x94P\x94\x92PPPV[`\0\x80\x82``\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a!n\x91\x90aA\xD4V[\x90P`\0a!\x80\x88\x88\x88\x85\x89\x89a0\xF8V[\x90P`\0a!\x8D\x82a2\xB8V[\x90P`\0a!\x9A\x83a3\xE6V[\x90P\x80\x82a\x1C\xE2g\r\xE0\xB6\xB3\xA7d\0\0aBoV[`\0\x82\x80\x85\x83\x81\x12\x15a\"DW[`\0\x81\x12\x15a\"?Wa!\xD5\x82a\x03\xE7a\x03\xE8a%\xCDV[\x85Q\x90\x92P`\0\x90a!\xE8\x90\x8A\x90a\x1A\xAAV[\x8A\x11a\"\x0CW\x85Qa!\xFB\x90\x8A\x90a\x1A\xAAV[a\"\x07\x90a\x03\xE8a>YV[a\"\x18V[a\"\x18\x8Aa\x03\xE8a>YV[\x90P\x89\x83\x10a\"'W\x82a\")V[\x80[\x92Pa\"7\x8A\x8A\x85\x89a$\xDBV[\x91PPa!\xBDV[a\"qV[`\0\x81\x13\x15a\"qWa\"\\\x83a\x03\xE9a\x03\xE8a#\x9AV[\x92Pa\"j\x89\x89\x85\x88a$\xDBV[\x90Pa\"DV[a\x13\xD4\x89\x89\x83\x88`@Q` \x01a\"\x8B\x94\x93\x92\x91\x90aA\x80V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83\x85`\x14`@a4\x92a&\x19V[```\x01\x82`@Q` \x01a\"\xBE\x92\x91\x90aB\xC4V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x91\x90PV[`\0a\x05\x8D\x82a)KV[```\x05\x82`@Q` \x01a\"\xBE\x92\x91\x90aB\xDFV[`\0\x80\x82``\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a#\x10\x91\x90aA\xD4V[\x83Q` \x85\x01Q\x91\x92P\x90`\0a#'\x89\x84a0\x1BV[\x90P`\0a#=\x83g\x1B\xC1mgN\xC8\0\0a0/V[a#G\x83\x85a0/V[a#Q\x91\x90aA\xD4V[\x90P`\0a#^\x82a$)V[\x90P`\0a#v\x82a#p\x8C\x89a0SV[\x90a0SV[\x90Pa\x1A\x9A\x8B\x82aA\xD4V[```\x03\x83\x83`@Q` \x01a\x18\x14\x93\x92\x91\x90aAaV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a#\xB2W`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[\x80Q` \x82\x01Q`@\x83\x01Q`\0\x92\x91\x90\x83a#\xE4\x83\x83a\x1C\0V[\x90P`\0a#\xF2\x88\x86a0\x1BV[\x90P`\0a$\0\x85\x85a'oV[\x90P\x82a$\r\x82\x84aA\xD4V[a$\x1F\x90g\r\xE0\xB6\xB3\xA7d\0\0aA\xFBV[a\x13\xD4\x91\x90aBAV[`\0\x80g\x13\xA0K\xBD\xFD\xC9\xBE\x88a$Gg\r\xE0\xB6\xB3\xA7d\0\0\x85aA\xFBV[a$Q\x91\x90aBAV[\x90P`\0a$^\x82aBoV[\x90P`\0a$k\x82a4\xBFV[\x90Pg\x1B\xC1mgN\xC8\0\0a$\x88g\r\xE0\xB6\xB3\xA7d\0\0\x83aA\xFBV[a\x05\xBD\x91\x90aBAV[`\0\x80\x82\x12\x15a$\xD7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01RotoUint: negative`\x80\x1B`D\x82\x01R`d\x01a\x07cV[P\x90V[`\0\x82\x85\x10a%,W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FtradingFunction: invalid x\0\0\0\0\0\0`D\x82\x01R`d\x01a\x07cV[`\0\x80g\r\xE0\xB6\xB3\xA7d\0\0a%B\x88\x87a\x1A\xAAV[\x10a%VW`\x01`\x01`\xFF\x1B\x03\x91Pa%fV[a%ca\x07\xA3\x88\x87a\x1A\xAAV[\x91P[g\r\xE0\xB6\xB3\xA7d\0\0a%\x81\x87a\x07\xE8\x87`\0\x01Q\x89a\x1B\xA5V[\x10a%\x94WP`\x01`\x01`\xFF\x1B\x03a%\xACV[a%\xA9a\x07\xA3\x87a\x07\xE8\x87`\0\x01Q\x89a\x1B\xA5V[\x90P[`\0a%\xC0\x85` \x01Q\x86`@\x01Qa\x1C\0V[\x90P\x80a\t\x1A\x83\x85a>1V[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a%\xE5W`\0\x80\xFD[\x04\x92\x91PPV[`\0\x80`\0\x80\x85\x80` \x01\x90Q\x81\x01\x90a&\x06\x91\x90aC\x05V[\x93PP\x92P\x92Pa\x108\x83\x86\x84\x84a$\xDBV[`\0\x84\x86\x11\x15a&FW`@Qc0\x82\xDF\xDB`\xE1\x1B\x81R`\x04\x81\x01\x87\x90R`$\x81\x01\x86\x90R`D\x01a\x07cV[`\0a&V\x88\x88\x85c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a&h\x89\x88\x86c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a&v\x82\x84aA\xFBV[\x13\x15a&\x9FW`@Qc\x06\xF1\xBE]`\xE2\x1B\x81R`\x04\x81\x01\x83\x90R`$\x81\x01\x82\x90R`D\x01a\x07cV[`\0a&\xAB\x89\x89a>lV[\x90P`\0[`\x02a&\xBC\x8A\x8Ca>YV[a&\xC6\x91\x90aC>V[\x94P`\0a&\xD8\x8C\x87\x89c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a&\xE6\x86\x83aA\xFBV[\x13a&\xF3W\x85\x99Pa&\xFAV[\x85\x9AP\x80\x94P[a'\x04\x8B\x8Ba>lV[\x92PP`\x01\x01\x87\x82\x11\x80\x15a'\x18WP\x86\x81\x10[a&\xB0WPPPP\x96\x95PPPPPPV[`\0\x80a'7\x84\x84a0\x86V[\x90P`\0a'D\x82a$)V[\x90P`\0a'Q\x82a$\x92V[\x90Pa\tPa'h\x82g\r\xE0\xB6\xB3\xA7d\0\0a>lV[\x88\x90a\x18@V[`\0\x80a'\x8E\x83a'\x88\x86g\x1B\xC1mgN\xC8\0\0a6\xA3V[\x90a\x1B\xA5V[\x90Pa\x0F\xC4g\x06\xF0[Y\xD3\xB2\0\0\x82a\x1B\xA5V[`\0h\x02H\xCE6\xA7\x0C\xB2k>\x19\x82\x13a'\xBDWP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a(\x04W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x01a\x07cV[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05`\x01`_\x1B\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1Dm\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1Dn\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[`\0\x80\x82\x12\x80a)bWPg\x1B\xC1mgN\xC8\0\0\x82\x13[\x15a)\x80W`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a)\xA1W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81g\x1B\xC1mgN\xC8\0\0\x03a)\xC9W`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x15a)\xD4W\x91\x90PV[`\0g\r\xE0\xB6\xB3\xA7d\0\0\x83\x12a)\xFCWa)\xF7\x83g\x1B\xC1mgN\xC8\0\0aA\xD4V[a)\xFEV[\x82[\x90P`\0a*\x14\x82g\x1B\xC1mgN\xC8\0\0a6\xCFV[\x90P\x80`\0\x03a*7W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a*B\x82a6\xE4V[\x90P`\0c;\x9A\xCA\0a*ma*ha*bg\x1B\xC1mgN\xC8\0\0aBoV[\x85a+\xC9V[a+\xFFV[a*w\x91\x90aA\xFBV[\x90P`\0\x80a*\x8E\x83g\x03\xC1f\\z\xAB \0a+\xC9V[a*\xA0\x90g \x05\xFEO&\x8E\xA0\0a>1V[\x90P`\0a*\xD0\x84a*\xB9\x86f\x9F2u$b\xA0\0a+\xC9V[a*\xCB\x90g\r\xC5R\x7Fd, \0a>1V[a+\xC9V[a*\xE2\x90g\r\xE0\xB6\xB3\xA7d\0\0a>1V[\x90Pa+\x06g\t\xD0(\xCCo _\xFF\x19\x85a*\xFC\x85\x85a6\xCFV[a*\xCB\x91\x90aA\xD4V[\x92PPP`\0[`\x02\x81\x10\x15a+\xA1W`\0\x86a+\"\x84a4\xBFV[a+,\x91\x90aA\xD4V[\x90P`\0a+:\x84\x85a+\xC9V[a+C\x90aBoV[\x90P`\0a+P\x82a'\xA2V[\x90P`\0a+^\x86\x85a+\xC9V[a+pg\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x84a+\xC9V[a+z\x91\x90aA\xD4V[\x90Pa+\x86\x84\x82a6\xCFV[a+\x90\x90\x87a>1V[\x95P\x84`\x01\x01\x94PPPPPa+\rV[g\r\xE0\xB6\xB3\xA7d\0\0\x88\x12a+\xBEWa+\xB9\x82aBoV[a\x06\xDEV[P\x96\x95PPPPPPV[`\0a\x05\x8A\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a8\xBFV[\x80Qjconsole.log` \x83\x01`\0\x80\x84\x83\x85Z\xFAPPPPPV[`\xB5\x81`\x01`\x88\x1B\x81\x10a,\x18W`@\x91\x90\x91\x1B\x90`\x80\x1C[i\x01\0\0\0\0\0\0\0\0\0\x81\x10a,4W` \x91\x90\x91\x1B\x90`@\x1C[e\x01\0\0\0\0\0\x81\x10a,LW`\x10\x91\x90\x91\x1B\x90` \x1C[c\x01\0\0\0\x81\x10a,bW`\x08\x91\x90\x91\x1B\x90`\x10\x1C[b\x01\0\0\x01\x02`\x12\x1C\x80\x82\x04\x01`\x01\x90\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x90\x81\x90\x04\x81\x11\x90\x03\x90V[`\0\x80`\0\x80`\0\x86\x80` \x01\x90Q\x81\x01\x90a,\xBF\x91\x90aC\x05V[\x93P\x93P\x93P\x93Pa\tP\x84\x84\x84\x89\x85a\x1C\x88V[a-1`@Q\x80a\x01`\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`\0a-Oa-@\x88\x86a>1V[g\x1B\xC1mgN\xC8\0\0\x90a0SV[\x90P`\0a-]\x85\x87a0SV[a-g\x86\x89a>1V[a-q\x91\x90aA\xD4V[\x90P`\0a-\x87a-\x82\x84\x84a0/V[a)KV[\x90P`\0a-\x9Cg\x1B\xC1mgN\xC8\0\0a+\xFFV[a-\xAA\x90c;\x9A\xCA\0aB\xADV[\x90P`\0a-\xBB\x87`@\x01Qa+\xFFV[a-\xC9\x90c;\x9A\xCA\0aB\xADV[`@\x80Qa\x01`\x81\x01\x82R\x94\x85R\x88Q` \x80\x87\x01\x91\x90\x91R\x89\x01Q\x85\x82\x01R\x97\x90\x97\x01Q``\x84\x01RP`\x80\x82\x01\x97\x90\x97R`\xA0\x81\x01\x98\x90\x98RPPP`\xC0\x85\x01\x93\x90\x93R`\xE0\x84\x01Ra\x01\0\x83\x01\x93\x90\x93Ra\x01 \x82\x01Ra\x01@\x81\x01\x91\x90\x91R\x90V[`\0\x80a.gg\x1B\xC1mgN\xC8\0\0a.a\x85``\x01Qa#p\x87`@\x01Q\x88`@\x01Qa0S\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90a0/V[a.p\x90aBoV[\x90P`\0a.\xA1\x84`\0\x01Qa#p\x86a\x01 \x01Qa#p\x88`@\x01Q\x89a\x01@\x01Qa0S\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a.\xB7a.\xB2\x83\x85a>1V[a'\xA2V[\x90P`\0a/\x07a.\xED\x87`\x80\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a.\xD8\x90aBoV[a.\xE2\x91\x90a>1V[`\xA0\x89\x01Q\x90a0SV[\x87`\xC0\x01Qa.\xFC\x91\x90a>1V[` \x88\x01Q\x90a0SV[\x90P`\0a/\x15\x83\x83a0SV[\x90P`\0a/4\x88`\x80\x01Q\x89`\xE0\x01Qa0S\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x88`\xE0\x01Q\x89`\xC0\x01Qa/H\x91\x90a>1V[a/R\x91\x90aA\xD4V[\x90Pa\x06\xDE\x82\x82a0/V[`\0\x80a/\x9D\x83`\x80\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a/{\x90aBoV[a/\x85\x91\x90a>1V[` \x85\x01Qa#p\x90g\x06\xF0[Y\xD3\xB2\0\0\x90a0SV[\x90P`\0a/\xC6\x84a\x01@\x01Qa.a\x86a\x01 \x01Q\x87`@\x01Qa0S\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa\x0F\xC4a/\xE3\x85`\0\x01Q\x83a/\xDE\x91\x90aA\xD4V[a4\xBFV[\x83\x90a0SV[`\0\x80`\0\x80`\0\x86\x80` \x01\x90Q\x81\x01\x90a0\x06\x91\x90aC\x05V[\x93P\x93P\x93P\x93Pa\tP\x84\x84\x84\x89\x85a!SV[`\0a\x05\x8Aa0*\x84\x84a\x18@V[a6\xE4V[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x81\x02\x90\x81\x05\x83\x14\x82\x15\x15\x16a0MW`\0\x80\xFD[\x05\x91\x90PV[\x81\x81\x02\x82\x15\x83\x82\x05\x83\x14\x17`\0\x19\x84\x10`\x01`\xFF\x1B\x84\x13\x17\x16a0uW`\0\x80\xFD[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x92\x91PPV[\x80Q` \x82\x01Q`@\x83\x01Q`\0\x92\x91\x90\x83a0\xA2\x83\x83a\x1C\0V[\x90P`\0a0\xB0\x88\x86a0\x1BV[\x90P`\0a0\xBE\x85\x85a'oV[\x90P\x82a$\r\x82\x84a>1V[`\0\x80`\0\x80\x85\x80` \x01\x90Q\x81\x01\x90a0\xE5\x91\x90aC\x05V[\x93PP\x92P\x92Pa\x108\x85\x84\x84\x84a$\xDBV[a1U`@Q\x80a\x01`\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`\0a1da-@\x88\x86a>1V[\x90P`\0a1r\x85\x87a0SV[\x84Q\x86\x90a1\x80\x90\x8Aa0SV[a1\x8A\x91\x90a>1V[a1\x94\x91\x90aA\xD4V[\x90P`\0a1\xA2\x83\x83a0/V[\x90Pa1\xD2`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01kintermediate`\xA0\x1B\x81RP\x82a\x1B\\V[`\0a1\xE1a-\x82\x85\x85a0/V[\x90Pa2\x0E`@Q\x80`@\x01`@R\x80`\t\x81R` \x01hierfc res`\xB8\x1B\x81RP\x82a\x1B\\V[`\0a2!g\x1B\xC1mgN\xC8\0\0a+\xFFV[a2/\x90c;\x9A\xCA\0aB\xADV[\x90P`\0a2@\x88`@\x01Qa+\xFFV[a2N\x90c;\x9A\xCA\0aB\xADV[`@\x80Qa\x01`\x81\x01\x82R\x94\x85R\x89Q` \x80\x87\x01\x91\x90\x91R\x8A\x01Q\x85\x82\x01R\x98\x90\x98\x01Q``\x84\x01RP`\x80\x82\x01\x98\x90\x98R`\xA0\x81\x01\x99\x90\x99RPPPP`\xC0\x85\x01\x93\x90\x93R`\xE0\x84\x01Ra\x01\0\x83\x01\x93\x90\x93Ra\x01 \x82\x01\x92\x90\x92Ra\x01@\x81\x01\x91\x90\x91R\x90V[`\0\x80a2\xEAg\x1B\xC1mgN\xC8\0\0a.a\x85``\x01Qa#p\x87`@\x01Q\x88`@\x01Qa0S\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a2\xF3\x90aBoV[\x90P`\0a3$\x84`\0\x01Qa#p\x86a\x01@\x01Qa#p\x88`@\x01Q\x89a\x01 \x01Qa0S\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a35a.\xB2\x83\x85a>1V[\x90P`\0a3\x7Fa3V\x87`\x80\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a.\xD8\x90aBoV[`\xC0\x88\x01Q` \x89\x01Qa3i\x91a0SV[a3s\x91\x90a>1V[a\x01\0\x88\x01Q\x90a0SV[\x90P`\0a3\x8D\x83\x83a0SV[\x90P`\0a/Ra3\xAF\x89`\x80\x01Q\x8A`\xE0\x01Qa0S\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[`\xE0\x8A\x01Q`\xC0\x8B\x01Q` \x8C\x01Qa3\xC7\x91a0SV[a3\xD1\x91\x90a>1V[a3\xDB\x91\x90aA\xD4V[` \x8A\x01Q\x90a0SV[`\0\x80a4\x19\x83`\x80\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a4\x03\x90aBoV[a4\r\x91\x90a>1V[a\x01\0\x85\x01Q\x90a0SV[\x90P`\0a4B\x84a\x01 \x01Qa.a\x86a\x01@\x01Q\x87`@\x01Qa0S\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a4ca4\\\x86`\0\x01Q\x84a/\xDE\x91\x90aA\xD4V[\x84\x90a0SV[\x90P`\0a4\x86\x86` \x01Qg\x1B\xC1mgN\xC8\0\0a0S\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa\x108\x82\x82a0/V[`\0\x80`\0\x80\x85\x80` \x01\x90Q\x81\x01\x90a4\xAC\x91\x90aC\x05V[\x93PP\x92P\x92Pa\x108\x83\x83\x87\x84a$\xDBV[`\0\x81`\0\x03a4\xD8WPg\r\xE0\xB6\xB3\xA7d\0\0\x91\x90PV[gV\x98\xEE\xF0fp\0\0\x82\x12a4\xEFWP`\0\x91\x90PV[a5\0gV\x98\xEE\xF0fp\0\0aBoV[\x82\x13a5\x15WPg\x1B\xC1mgN\xC8\0\0\x91\x90PV[`\0a5 \x83a8\xDEV[\x90P`\0a5Yg\r\xE0\xB6\xB3\xA7d\0\0a5B\x84g\x1B\xC1mgN\xC8\0\0a\x1A\xAAV[a5T\x90g\r\xE0\xB6\xB3\xA7d\0\0a>1V[a6\xCFV[\x90P`\0\x80\x82a5\xB5\x81a5\xA2\x81a5\x90\x81a5}\x81g\x02_\x0F\xE1\x05\xA3\x14\0a+\xC9V[a*\xCB\x90g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19a>1V[a*\xCB\x90g\x14\xA8EL\x19\xE1\xAC\0a>1V[a*\xCB\x90g\x0F\xC1\x0E\x01W\x82w\xFF\x19a>1V[a5\xC7\x90g\x03\xDE\xBD\x08;\x8C|\0a>1V[\x91P\x83\x90Pa6/\x81a6\x1D\x81a6\x0B\x81a5\xF9\x81a5\xE6\x81\x8Ba+\xC9V[a*\xCB\x90g\x02\x95\xD4\0\xEA2W\xFF\x19a>1V[a*\xCB\x90g\x01W\xD8\xB2\xEC\xC7\x08\0a>1V[a*\xCB\x90g\x051\n\xA7\xD5!0\0a>1V[a*\xCB\x90g\r\xE0\xCC=\x15a\0\0a>1V[\x91P\x81g\x11\x90\0\xAB\x10\x0F\xFC\0a6E\x87\x88a+\xC9V[a6Q\x90`\0\x19aA\xFBV[a6[\x91\x90aA\xD4V[a6e\x91\x90a>1V[\x92PP`\0a6s\x83a'\xA2V[\x90P`\0a6\x81\x85\x83a+\xC9V[\x90P`\0\x88\x12a6\x91W\x80a\x06\xDEV[a\x06\xDE\x81g\x1B\xC1mgN\xC8\0\0aA\xD4V[`\0a\x05\x8Ag\r\xE0\xB6\xB3\xA7d\0\0\x83a6\xBB\x86a6\xE4V[a6\xC5\x91\x90aA\xFBV[a.\xB2\x91\x90aBAV[`\0a\x05\x8A\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a8\xBFV[`\0\x80\x82\x13a7!W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x07cV[`\0``a7.\x84a9\x15V[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x05\x85\x14\x17\x16a8\xD7W`\0\x80\xFD[\x05\x92\x91PPV[`\0`\x01`\xFF\x1B\x82\x03a9\x04W`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x12\x15a$\xD7WP\x19`\x01\x01\x90V[`\0\x80\x82\x11a9RW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x07cV[P`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x82\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`\0\x80`@\x83\x85\x03\x12\x15a:#Wa:#a9\xBDV[PP\x805\x92` \x90\x91\x015\x91PV[`\0[\x83\x81\x10\x15a:MW\x81\x81\x01Q\x83\x82\x01R` \x01a:5V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra:n\x81` \x86\x01` \x86\x01a:2V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x05\x8A` \x83\x01\x84a:VV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a:\xAEWa:\xAEa9\xBDV[PP\x825\x94` \x84\x015\x94P`@\x84\x015\x93``\x015\x92P\x90PV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FABI decoding: struct data too sh`D\x82\x01Rb\x1B\xDC\x9D`\xEA\x1B`d\x82\x01R`\x84\x81\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a;TWa;Ta;\x1BV[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a;\x83Wa;\x83a;\x1BV[`@R\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x1B\xFDW`\0\x80\xFD[`\0`\xA0\x82\x84\x03\x12\x15a;\xB5Wa;\xB5a:\xCAV[a;\xBDa;1V[\x90P\x815\x81R` \x82\x015` \x82\x01R`@\x82\x015`@\x82\x01R``\x82\x015``\x82\x01R`\x80\x82\x015a;\xEF\x81a;\x8BV[`\x80\x82\x01R\x92\x91PPV[`\0\x80`\0`\xE0\x84\x86\x03\x12\x15a<\x12Wa<\x12a9\xBDV[\x835\x92P` \x84\x015\x91Pa<*\x85`@\x86\x01a;\xA0V[\x90P\x92P\x92P\x92V[`\0\x80`\0``\x84\x86\x03\x12\x15a<KWa<Ka9\xBDV[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[`\0\x80`\0\x80a\x01\0\x85\x87\x03\x12\x15a<|Wa<|a9\xBDV[\x845\x93P` \x85\x015\x92P`@\x85\x015\x91Pa<\x9B\x86``\x87\x01a;\xA0V[\x90P\x92\x95\x91\x94P\x92PV[\x80\x15\x15\x81\x14a\x1B\xFDW`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15a<\xCCWa<\xCCa9\xBDV[\x835\x92P` \x84\x015a<\xDE\x81a<\xA6V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[\x84\x15\x15\x81R\x83` \x82\x01R\x82`@\x82\x01R`\x80``\x82\x01R`\0a\x108`\x80\x83\x01\x84a:VV[`\0` \x82\x84\x03\x12\x15a=+Wa=+a9\xBDV[P5\x91\x90PV[\x81Q\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x80\x83\x01Q\x90\x82\x01R``\x80\x83\x01Q\x90\x82\x01R`\x80\x80\x83\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x90\x82\x01R`\xA0\x81\x01a\x05\x8DV[`\0\x80`\0\x80`\0\x80a\x01@\x87\x89\x03\x12\x15a=\x8DWa=\x8Da9\xBDV[\x865\x95P` \x87\x015\x94P`@\x87\x015\x93P``\x87\x015\x92P`\x80\x87\x015\x91Pa=\xBA\x88`\xA0\x89\x01a;\xA0V[\x90P\x92\x95P\x92\x95P\x92\x95V[`\0` \x82\x84\x03\x12\x15a=\xDBWa=\xDBa9\xBDV[\x815a\x06\xFA\x81a;\x8BV[\x82\x81R`@` \x82\x01R`\0a\x06\xF7`@\x83\x01\x84a:VV[`\0` \x82\x84\x03\x12\x15a>\x14Wa>\x14a9\xBDV[PQ\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x82\x81\x12`\0\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a>QWa>Qa>\x1BV[PP\x92\x91PPV[\x80\x82\x01\x80\x82\x11\x15a\x05\x8DWa\x05\x8Da>\x1BV[\x81\x81\x03\x81\x81\x11\x15a\x05\x8DWa\x05\x8Da>\x1BV[`\x01\x80`\xA0\x1B\x03\x84\x16\x81R\x82` \x82\x01R```@\x82\x01R`\0a\x05\xBD``\x83\x01\x84a:VV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a>\xC2Wa>\xC2a9\xBDV[\x86Qa>\xCD\x81a<\xA6V[` \x88\x01Q`@\x89\x01Q``\x8A\x01Q`\x80\x8B\x01Q`\xA0\x90\x9B\x01Q\x93\x9C\x92\x9BP\x90\x99\x90\x98P\x96P\x90\x94P\x92PPPV[`\0` \x80\x83\x85\x03\x12\x15a?\x12Wa?\x12a9\xBDV[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a?uW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x84\x90R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a?\xDDW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x84\x90R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x81\xFD[\x81Q\x81\x81\x11\x15a?\xEFWa?\xEFa;\x1BV[a@\x01`\x1F\x82\x01`\x1F\x19\x16\x85\x01a;ZV[\x91P\x80\x82R\x86\x84\x82\x85\x01\x01\x11\x15a@gW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x85\x90R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R`\x84\x81\xFD[a@v\x81\x85\x84\x01\x86\x86\x01a:2V[P\x95\x94PPPPPV[`\0`\xA0\x82\x84\x03\x12\x15a@\x95Wa@\x95a:\xCAV[a@\x9Da;1V[\x90P\x81Q\x81R` \x82\x01Q` \x82\x01R`@\x82\x01Q`@\x82\x01R``\x82\x01Q``\x82\x01R`\x80\x82\x01Qa;\xEF\x81a;\x8BV[`\0`\xA0\x82\x84\x03\x12\x15a@\xE4Wa@\xE4a9\xBDV[a\x05\x8A\x83\x83a@\x80V[`\0` \x82\x84\x03\x12\x15aA\x03WaA\x03a9\xBDV[\x81Qa\x06\xFA\x81a;\x8BV[`\0\x80`\0``\x84\x86\x03\x12\x15aA&WaA&a9\xBDV[\x83Q\x92P` \x84\x01Q\x91P`@\x84\x01Q\x90P\x92P\x92P\x92V[`\x06\x81\x10aA]WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x90RV[``\x81\x01aAo\x82\x86aA?V[` \x82\x01\x93\x90\x93R`@\x01R\x91\x90PV[\x84\x81R` \x81\x01\x84\x90R`@\x81\x01\x83\x90Ra\x01\0\x81\x01a\x05\xBD``\x83\x01\x84\x80Q\x82R` \x80\x82\x01Q\x90\x83\x01R`@\x80\x82\x01Q\x90\x83\x01R``\x80\x82\x01Q\x90\x83\x01R`\x80\x90\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x91\x01RV[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15aA\xF4WaA\xF4a>\x1BV[P\x92\x91PPV[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15aB\x17WaB\x17a>\x1BV[\x81\x81\x05\x83\x14\x82\x15\x17a\x05\x8DWa\x05\x8Da>\x1BV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82aBPWaBPaB+V[`\x01`\xFF\x1B\x82\x14`\0\x19\x84\x14\x16\x15aBjWaBja>\x1BV[P\x05\x90V[`\0`\x01`\xFF\x1B\x82\x01aB\x84WaB\x84a>\x1BV[P`\0\x03\x90V[`@\x81R`\0aB\x9E`@\x83\x01\x85a:VV[\x90P\x82` \x83\x01R\x93\x92PPPV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x05\x8DWa\x05\x8Da>\x1BV[`@\x81\x01aB\xD2\x82\x85aA?V[\x82` \x83\x01R\x93\x92PPPV[`@\x81\x01aB\xED\x82\x85aA?V[`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16` \x91\x90\x91\x01R\x91\x90PV[`\0\x80`\0\x80a\x01\0\x85\x87\x03\x12\x15aC\x1FWaC\x1Fa9\xBDV[\x84Q\x93P` \x85\x01Q\x92P`@\x85\x01Q\x91Pa<\x9B\x86``\x87\x01a@\x80V[`\0\x82aCMWaCMaB+V[P\x04\x90V\xFETarget contract does not contain\xA2dipfsX\"\x12 .\x96\xCD\xB3h\xA7uSqUP\x99\xBA>\xCF\x97\xEE\xCF\xBD&\xC9\xB6 \x06\xBC\x87\xCB\x9CcF\xE5}dsolcC\0\x08\x16\x003";
    /// The bytecode of the contract.
    pub static LOGNORMALSOLVER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x02HW`\x005`\xE0\x1C\x80cme\"\x99\x11a\x01gW\x80c\xB0\x9D\x04\xE5\x11a\0\xFAW\x80c\xE4]Y<\x11a\0\xC9W\x80c\xE4]Y<\x14a\x05*W\x80c\xE9G\x16\xD5\x14a\x05=W\x80c\xEE>\x8C\xFB\x14a\x05PW\x80c\xF3\r7\xF2\x14a\x05cW\x80c\xF9\xC2\x82\x11\x14a\x05vWa\x02HV[\x80c\xB0\x9D\x04\xE5\x14a\x04\xDEW\x80c\xC5\xE71i\x14a\x04\xF1W\x80c\xCB\x1FU2\x14a\x05\x04W\x80c\xCE\x15;\xF4\x14a\x05\x17Wa\x02HV[\x80c\x9An\xC1b\x11a\x016W\x80c\x9An\xC1b\x14a\x04zW\x80c\x9B\xDC\x03\x1C\x14a\x04\x8DW\x80c\xA8\xC6.v\x14a\x04\xA0W\x80c\xAFNC\x7F\x14a\x04\xCBWa\x02HV[\x80cme\"\x99\x14a\x04,W\x80c\x7F\x17@\x9C\x14a\x044W\x80c\x81\xB5\xFA\xC2\x14a\x04GW\x80c\x90.\xCA\xA2\x14a\x04gWa\x02HV[\x80c9(\xFF\x97\x11a\x01\xDFW\x80cO\xD6|X\x11a\x01\xAEW\x80cO\xD6|X\x14a\x03\xC5W\x80cU\xF0\x11\xC6\x14a\x03\xD8W\x80c^\xB4\x08\xFC\x14a\x03\xEBW\x80cb7V\x9F\x14a\x03\xFEWa\x02HV[\x80c9(\xFF\x97\x14a\x03iW\x80c;&\x8D]\x14a\x03\x8CW\x80c;M\x100\x14a\x03\x9FW\x80cN\x81\x7F\xD9\x14a\x03\xB2Wa\x02HV[\x80c\x1E\x97\x8C\xB0\x11a\x02\x1BW\x80c\x1E\x97\x8C\xB0\x14a\x03\x1DW\x80c$\xDE%s\x14a\x030W\x80c0m\xB4k\x14a\x03CW\x80c3\"f\xF3\x14a\x03VWa\x02HV[\x80c\x04 X\n\x14a\x02\xADW\x80c\x070U<\x14a\x02\xD6W\x80c\x12\x06I\xC5\x14a\x02\xF7W\x80c\x13N\xAD\x12\x14a\x03\nW[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x02\xC0a\x02\xBB6`\x04a:\rV[a\x05~V[`@Qa\x02\xCD\x91\x90a:\x82V[`@Q\x80\x91\x03\x90\xF3[a\x02\xE9a\x02\xE46`\x04a:\x95V[a\x05\x93V[`@Q\x90\x81R` \x01a\x02\xCDV[a\x02\xE9a\x03\x056`\x04a:\x95V[a\x05\xC6V[a\x02\xC0a\x03\x186`\x04a;\xFAV[a\x06\xEAV[a\x02\xE9a\x03+6`\x04a<3V[a\x07\x01V[a\x02\xE9a\x03>6`\x04a<bV[a\x07\x16V[a\x02\xE9a\x03Q6`\x04a<3V[a\t$V[a\x02\xE9a\x03d6`\x04a<3V[a\t[V[a\x03|a\x03w6`\x04a<\xB4V[a\t\x87V[`@Qa\x02\xCD\x94\x93\x92\x91\x90a<\xEFV[a\x02\xC0a\x03\x9A6`\x04a:\rV[a\x0F\x97V[a\x02\xE9a\x03\xAD6`\x04a=\x16V[a\x0F\xA3V[a\x02\xE9a\x03\xC06`\x04a<3V[a\x0F\xCCV[a\x02\xE9a\x03\xD36`\x04a<3V[a\x0F\xE1V[a\x02\xE9a\x03\xE66`\x04a:\rV[a\x10\rV[a\x02\xE9a\x03\xF96`\x04a:\x95V[a\x10BV[a\x04\x11a\x04\x0C6`\x04a:\rV[a\x11\x9FV[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x02\xCDV[a\x02\xE9`\0\x81V[a\x04\x11a\x04B6`\x04a:\rV[a\x11\xF9V[a\x04Za\x04U6`\x04a=\x16V[a\x12RV[`@Qa\x02\xCD\x91\x90a=2V[a\x02\xE9a\x04u6`\x04a<3V[a\x13^V[a\x02\xE9a\x04\x886`\x04a=pV[a\x13\x8AV[a\x02\xE9a\x04\x9B6`\x04a=pV[a\x13\xE1V[`\0Ta\x04\xB3\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\xCDV[a\x02\xE9a\x04\xD96`\x04a:\x95V[a\x14\nV[a\x02\xC0a\x04\xEC6`\x04a=\x16V[a\x15\x8EV[a\x02\xE9a\x04\xFF6`\x04a=\x16V[a\x15\x99V[a\x02\xC0a\x05\x126`\x04a=\xC6V[a\x15\xA4V[a\x04\x11a\x05%6`\x04a=\x16V[a\x15\xAFV[a\x02\xE9a\x0586`\x04a:\rV[a\x17?V[a\x02\xC0a\x05K6`\x04a:\rV[a\x17\xA4V[a\x04\x11a\x05^6`\x04a:\rV[a\x17\xB0V[a\x04\x11a\x05q6`\x04a:\rV[a\x17\xD6V[a\x02\xE9`x\x81V[``a\x05\x8A\x83\x83a\x17\xFCV[\x90P[\x92\x91PPV[`\0a\x05\xBDg\x06\xF0[Y\xD3\xB2\0\0a\x05\xB1\x84a\x05\xB7\x87\x83\x8B\x8Ba\x18+V[\x90a\x18+V[\x90a\x18@V[\x95\x94PPPPPV[`\0\x80a\x05\xDC\x84\x84a\x05\xD7\x89a\x12RV[a\x18UV[`@\x80Q` \x81\x01\x88\x90R\x80\x82\x01\x83\x90R``\x80\x82\x01\x88\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x80\x82\x01\x92\x83\x90R`\0\x80Tb.RK`\xE0\x1B\x90\x94R\x93\x94P\x92\x91`\x01`\x01`\xA0\x1B\x03\x16\x90b.RK\x90a\x069\x90\x8B\x90\x86\x90`\x84\x01a=\xE6V[` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x06\x8FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` aCS\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x06\xA3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xC7\x91\x90a=\xFFV[\x90Pa\x06\xDE\x87\x87\x83\x86a\x06\xD9\x8Da\x12RV[a\x18\x91V[\x98\x97PPPPPPPPV[``a\x06\xF7\x84\x84\x84a\x195V[\x90P[\x93\x92PPPV[`\0a\x06\xF7\x83\x83a\x07\x11\x87a\x12RV[a\x19\xA6V[`\0\x82\x85\x10a\x07lW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FtradingFunction: invalid x\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x80g\r\xE0\xB6\xB3\xA7d\0\0a\x07\x82\x88\x87a\x1A\xAAV[\x10a\x07\x96W`\x01`\x01`\xFF\x1B\x03\x91Pa\x07\xCDV[a\x07\xA8a\x07\xA3\x88\x87a\x1A\xAAV[a\x1A\xBFV[\x91Pa\x07\xCD`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01`a`\xF8\x1B\x81RP\x83a\x1B\\V[g\r\xE0\xB6\xB3\xA7d\0\0a\x07\xED\x87a\x07\xE8\x87`\0\x01Q\x89a\x1B\xA5V[a\x1A\xAAV[\x10a\x08VWa\x08$`@Q\x80`@\x01`@R\x80`\x11\x81R` \x01p\r-\xC4\r\xAC/\x04\r-\xCE\x84\x0CNL-\xCCm`{\x1B\x81RPa\x1B\xBAV[`\x01`\x01`\xFF\x1B\x03\x90Pa\x08Q`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01`1`\xF9\x1B\x81RP\x82a\x1B\\V[a\x08nV[a\x08ka\x07\xA3\x87a\x07\xE8\x87`\0\x01Q\x89a\x1B\xA5V[\x90P[`\0a\x08\x82\x85` \x01Q\x86`@\x01Qa\x1C\0V[\x90Pa\x08\xA7`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01`c`\xF8\x1B\x81RP\x82a\x1B\\V[`@\x80Q\x80\x82\x01\x90\x91R`\x05\x81Rda + c`\xD8\x1B` \x82\x01Ra\x08\xD6\x90a\x08\xD1\x83\x86a>1V[a\x1B\\V[`@\x80Q\x80\x82\x01\x90\x91R`\t\x81Rha + b + c`\xB8\x1B` \x82\x01Ra\t\x0F\x90\x82a\t\x05\x85\x87a>1V[a\x08\xD1\x91\x90a>1V[\x80a\t\x1A\x83\x85a>1V[a\x06\xDE\x91\x90a>1V[`\0\x80a\t0\x85a\x12RV[\x90P`\0\x80a\t>\x87a\x15\xAFV[\x92PP\x91Pa\tP\x86\x83\x83\x88\x87a\x1C&V[\x97\x96PPPPPPPV[`\0\x80a\tg\x85a\x12RV[\x90P`\0\x80a\tu\x87a\x15\xAFV[\x92PP\x91Pa\tP\x86\x83\x83\x88\x87a\x1C\x88V[`\0\x80`\0``a\t\xB9`@Q\x80`\x80\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[a\t\xE4`@Q\x80`\x80\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[a\t\xED\x89a\x15\xAFV[`@\x85\x01R` \x84\x01R\x82R`\0a\n\x04\x8Aa\x12RV[\x90P`\0a\n6`@Q\x80`@\x01`@R\x80`\x08\x81R` \x01g\x0En\x8C.N\x84\x0EO`\xC3\x1B\x81RP\x85`\0\x01Qa\x1D\x04V[a\nd`@Q\x80`@\x01`@R\x80`\x08\x81R` \x01gstart ry`\xC0\x1B\x81RP\x85` \x01Qa\x1D\x04V[a\n\x91`@Q\x80`@\x01`@R\x80`\x07\x81R` \x01f\x1C\xDD\x18\\\x9D\x08\x13`\xCA\x1B\x81RP\x85`@\x01Qa\x1D\x04V[a\n\xC2`@Q\x80`@\x01`@R\x80`\x10\x81R` \x01o\x18\xDB\xDB\\\x1D]\x1A[\x99\xC8\x1B\x99^\x1D\x08\x13`\x82\x1B\x81RPa\x1B\xBAV[`\0a\n\xDC\x8C\x86`\0\x01Q\x87` \x01Q\x88`@\x01Qa\x14\nV[\x90Pa\x0B\x0E`@Q\x80`@\x01`@R\x80`\x0F\x81R` \x01n\x18\xDB\xDB\\\x1D]\x19Y\x08\x1B\x99^\x1D\x08\x13`\x8A\x1B\x81RPa\x1B\xBAV[\x8A\x15a\x0C_W\x84Qa\x0B!\x90\x8B\x90a>YV[\x84R\x84Q` \x86\x01Q`@\x87\x01Qa\x0B=\x92\x8F\x92\x8E\x92\x88a\x13\x8AV[a\x0BG\x90\x82a>YV[`@\x85\x01\x81\x90R\x84Qa\x0B\\\x91\x8E\x91\x90a\x07\x01V[``\x85\x01R`@\x80Q\x80\x82\x01\x90\x91R`\x10\x81Rocomputing next Y`\x80\x1B` \x82\x01Ra\x0B\x91\x90a\x1B\xBAV[a\x0B\xA9\x8C\x85`\0\x01Q\x86`@\x01Q\x87``\x01Qa\x05\xC6V[\x84` \x01\x81\x81RPPa\x0B\xE2`@Q\x80`@\x01`@R\x80`\x0F\x81R` \x01ncomputed next Y`\x88\x1B\x81RPa\x1B\xBAV[\x84` \x01Q\x84` \x01Q\x10a\x0CDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: y reserve increase`D\x82\x01Rad!`\xF0\x1B`d\x82\x01R`\x84\x01a\x07cV[\x83` \x01Q\x85` \x01Qa\x0CX\x91\x90a>lV[\x91Pa\x0EUV[\x89\x85` \x01Qa\x0Co\x91\x90a>YV[\x84` \x01\x81\x81RPPa\x0C\x92\x8C\x8B\x87`\0\x01Q\x88` \x01Q\x89`@\x01Q\x88a\x13\xE1V[a\x0C\x9C\x90\x82a>YV[\x84`@\x01\x81\x81RPPa\x0C\xD1`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01eend ry`\xD0\x1B\x81RP\x85` \x01Qa\x1D\x04V[a\x0C\xFC`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01d\x19[\x99\x08\x13`\xDA\x1B\x81RP\x85`@\x01Qa\x1D\x04V[a\r\x0F\x8C\x85` \x01Q\x86`@\x01Qa\x0F\xCCV[``\x85\x01R`@\x80Q\x80\x82\x01\x90\x91R`\x10\x81Ro\x0Cm\xED\xAE\x0E\xAE\x8D-\xCC\xE4\r\xCC\xAF\x0E\x84\x0B`\x83\x1B` \x82\x01Ra\rD\x90a\x1B\xBAV[a\r\\\x8C\x85` \x01Q\x86`@\x01Q\x87``\x01Qa\x10BV[\x84R`@\x80Q\x80\x82\x01\x90\x91R`\x0F\x81Rn\x0Cm\xED\xAE\x0E\xAE\x8C\xAC\x84\r\xCC\xAF\x0E\x84\x0B`\x8B\x1B` \x82\x01Ra\r\x8D\x90a\x1B\xBAV[a\r\xBA`@Q\x80`@\x01`@R\x80`\x07\x81R` \x01f72\xBC:\x10<\x1D`\xC9\x1B\x81RP\x85`\0\x01Qa\x1D\x04V[a\r\xE8`@Q\x80`@\x01`@R\x80`\x08\x81R` \x01g9\xBA0\xB9:\x10<\x1D`\xC1\x1B\x81RP\x86`\0\x01Qa\x1D\x04V[\x84Q\x84Q\x10a\x0EDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: x reserve increase`D\x82\x01Rad!`\xF0\x1B`d\x82\x01R`\x84\x01a\x07cV[\x83Q\x85Qa\x0ER\x91\x90a>lV[\x91P[P\x82Q` \x80\x85\x01Q`@\x80\x87\x01Q\x81Q\x93\x84\x01\x94\x90\x94R\x82\x01R``\x81\x01\x91\x90\x91R`\0\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16ch\xBD>80\x8F\x85`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0E\xD5\x93\x92\x91\x90a>\x7FV[`\xC0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0F+W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` aCS\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0F?W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Fc\x91\x90a>\xA6V[PPPPP\x90P\x80\x83a\x0F\x7F\x87`\0\x01Q\x88`@\x01Q\x88a\x19\xA6V[\x84\x99P\x99P\x99P\x99PPPPPPP\x93P\x93P\x93P\x93V[``a\x05\x8A\x83\x83a\x1DIV[`\0\x80`\0a\x0F\xB1\x84a\x15\xAFV[\x92PP\x91Pa\x0F\xC4\x82\x82a\x07\x11\x87a\x12RV[\x94\x93PPPPV[`\0a\x06\xF7\x83\x83a\x0F\xDC\x87a\x12RV[a\x1DaV[`\0\x80a\x0F\xED\x85a\x12RV[\x90P`\0\x80a\x0F\xFB\x87a\x15\xAFV[\x92P\x92PPa\tP\x86\x83\x83\x88\x87a\x1E/V[`\0\x80a\x10\x19\x84a\x12RV[\x90P`\0\x80a\x10'\x86a\x15\xAFV[\x92PP\x91Pa\x108\x85\x83\x83\x86a\x1E\x91V[\x96\x95PPPPPPV[`\0\x80a\x10X\x84\x84a\x10S\x89a\x12RV[a\x1F%V[\x90Pa\x10\x99`@Q\x80`@\x01`@R\x80`\x1B\x81R` \x01\x7Fapproximated rx given price\0\0\0\0\0\x81RP\x82a\x1D\x04V[`@\x80Q` \x81\x01\x83\x90R\x90\x81\x01\x86\x90R``\x81\x01\x85\x90R`\0\x90`\x80\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R`\0\x80Tb.RK`\xE0\x1B\x84R\x91\x93P\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90b.RK\x90a\x10\xFA\x90\x8B\x90\x86\x90`\x04\x01a=\xE6V[` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x11PW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` aCS\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x11dW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\x88\x91\x90a=\xFFV[\x90Pa\x06\xDE\x87\x87\x83\x86a\x11\x9A\x8Da\x12RV[a\x1FjV[`\0\x80`\0\x80`\0a\x11\xB0\x87a\x15\xAFV[\x92PP\x91P`\0\x80a\x11\xC5`\0\x89\x86\x86a \xEAV[\x91P\x91P`\0a\x11\xD6\x8A\x84\x84a\x07\x01V[\x90P`\0a\x11\xE6\x8B\x85\x85\x85a\x05\xC6V[\x93\x9B\x93\x9AP\x91\x98P\x91\x96PPPPPPPV[`\0\x80`\0\x80`\0a\x12\n\x87a\x15\xAFV[\x92P\x92PP`\0\x80a\x12\x1F`\x01\x89\x86\x86a \xEAV[\x91P\x91P`\0a\x120\x8A\x84\x84a\x0F\xCCV[\x90P`\0a\x12@\x8B\x85\x85\x85a\x10BV[\x9B\x93\x9AP\x91\x98P\x91\x96PPPPPPPV[a\x12\x8D`@Q\x80`\xA0\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81RP\x90V[`\0T`@Qc\xDC\x17\x83U`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xDC\x17\x83U\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x13\x0FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` aCS\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x13#W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x13K\x91\x90\x81\x01\x90a>\xFCV[\x80` \x01\x90Q\x81\x01\x90a\x05\x8D\x91\x90a@\xCFV[`\0\x80a\x13j\x85a\x12RV[\x90P`\0\x80a\x13x\x87a\x15\xAFV[\x92P\x92PPa\tP\x86\x83\x83\x88\x87a!SV[`\0\x80a\x13\xA4\x87\x84``\x01Qa\x18+\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x13\xB3\x89\x88\x87a\x07\x01V[\x90Pa\x13\xD4\x86a\x13\xC3\x83\x8Aa\x1B\xA5V[a\x13\xCD\x91\x90a>YV[\x83\x90a\x1A\xAAV[\x99\x98PPPPPPPPPV[`\0\x80a\x13\xFB\x87\x84``\x01Qa\x18+\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x13\xB3\x89\x87\x87a\x0F\xCCV[`@\x80Q` \x81\x01\x85\x90R\x90\x81\x01\x83\x90R``\x81\x01\x82\x90R`\0\x90\x81\x90`\x80\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82\x01\x90\x91R`\x0B\x82Rj\x1C\x9E\x08\x1A[\x88\x1B\x99^\x1D\x13`\xAA\x1B` \x83\x01R\x91Pa\x14a\x90\x86a\x1D\x04V[a\x14\x8D`@Q\x80`@\x01`@R\x80`\n\x81R` \x01i\x13\x08\x1A[\x88\x1B\x99^\x1D\x13`\xB2\x1B\x81RP\x84a\x1D\x04V[`\0\x80T`@Qb.RK`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90b.RK\x90a\x14\xBE\x90\x8A\x90\x86\x90`\x04\x01a=\xE6V[` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x15\x14W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` aCS\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x15(W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15L\x91\x90a=\xFFV[\x90Pa\x15y`@Q\x80`@\x01`@R\x80`\t\x81R` \x01h\x1A[\x9D\x98\\\x9AX[\x9D`\xBA\x1B\x81RP\x82a\x1B\\V[a\tP\x86\x86\x83\x87a\x15\x89\x8Ca\x12RV[a!\xAFV[``a\x05\x8D\x82a\"\xA8V[`\0a\x05\x8D\x82a\"\xD4V[``a\x05\x8D\x82a\"\xDFV[`\0\x80`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xAF\xBA\x13\xC4`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x16?W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` aCS\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x16SW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16w\x91\x90a@\xEEV[`\x01`\x01`\xA0\x1B\x03\x16c\xCE\x15;\xF4\x85`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x16\xA4\x91\x81R` \x01\x90V[```@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x16\xFAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` aCS\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x17\x0EW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x172\x91\x90aA\x0EV[\x92P\x92P\x92P\x91\x93\x90\x92PV[`\0\x80a\x17K\x84a\x12RV[\x90P`\0\x80a\x17Y\x86a\x15\xAFV[\x92P\x92PP`\0a\x17l\x86\x84\x84\x87a\"\xF5V[\x90Pa\x108`@Q\x80`@\x01`@R\x80`\x14\x81R` \x01s====computed dy=====``\x1B\x81RP\x82a\x1B\\V[``a\x05\x8A\x83\x83a#\x82V[`\0\x80`\0\x80`\0a\x17\xC1\x87a\x15\xAFV[\x92PP\x91P`\0\x80a\x11\xC5`\x01\x89\x86\x86a \xEAV[`\0\x80`\0\x80`\0a\x17\xE7\x87a\x15\xAFV[\x92P\x92PP`\0\x80a\x12\x1F`\0\x89\x86\x86a \xEAV[```\x02\x83\x83`@Q` \x01a\x18\x14\x93\x92\x91\x90aAaV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x92\x91PPV[`\0a\x05\x8A\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a#\x9AV[`\0a\x05\x8A\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a#\x9AV[`\0\x80a\x18b\x84\x84a#\xC8V[\x90P`\0a\x18o\x82a$)V[\x90P`\0a\x18|\x82a$\x92V[\x85Q\x90\x91Pa\tP\x90\x82\x90a\x05\xB1\x90\x8Aa\x18+V[`\0\x82\x80\x85\x83\x81\x12\x15a\x18\xD1W[`\0\x81\x12\x15a\x18\xCCWa\x18\xB7\x83a\x03\xE9a\x03\xE8a#\x9AV[\x92Pa\x18\xC5\x89\x84\x8A\x88a$\xDBV[\x90Pa\x18\x9FV[a\x18\xFEV[`\0\x81\x13\x15a\x18\xFEWa\x18\xE9\x82a\x03\xE7a\x03\xE8a%\xCDV[\x91Pa\x18\xF7\x89\x83\x8A\x88a$\xDBV[\x90Pa\x18\xD1V[a\x13\xD4\x89\x89\x83\x88`@Q` \x01a\x19\x18\x94\x93\x92\x91\x90aA\x80V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83\x85`\x14`@a%\xECa&\x19V[```\0a\x19D\x85\x85\x85a'*V[\x90P`\0a\x19S\x82\x86\x86a\x18UV[\x90P`\0a\x19c\x87\x83\x85\x88a$\xDBV[\x90Pa\x19r\x87\x83\x83\x86\x89a!\xAFV[\x92P\x86\x82\x84\x87`@Q` \x01a\x19\x8B\x94\x93\x92\x91\x90aA\x80V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x93PPPP\x93\x92PPPV[`\0\x80a\x19\xBB\x83` \x01Q\x84`@\x01Qa\x1C\0V[\x90P`\0a\x19\xD1\x84` \x01Q\x85`@\x01Qa'oV[\x90P`\0a\x19\xEC\x85`@\x01Q\x83a\x1B\xA5\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x19\xFA\x88\x88a\x1A\xAAV[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0\x81\x12a\x1A\x18W`\0\x94PPPPPa\x06\xFAV[`\0\x81\x13a\x1A.W`\0\x19\x94PPPPPa\x06\xFAV[`\0a\x1AEa\x07\xA3\x83g\r\xE0\xB6\xB3\xA7d\0\0aA\xD4V[\x90P`\0\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x1A]\x88\x85aA\xFBV[a\x1Ag\x91\x90aBAV[a\x1Aq\x91\x90aA\xD4V[\x90P`\0a\x1A~\x82a'\xA2V[\x90P`\0a\x1A\x8B\x82a$\x92V[\x8AQ\x90\x91Pa\x1A\x9A\x90\x82a\x18+V[\x9C\x9BPPPPPPPPPPPPV[`\0a\x05\x8A\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a%\xCDV[`\0g\x06\xF0[Y\xD3\xB2\0\0\x82\x03a\x1A\xD8WP`\0\x91\x90PV[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x12a\x1B\0W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\x1B!W`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x1B.\x83`\x02aA\xFBV[\x90P`\0a\x1B;\x82a)KV[\x90P`\0a\x1BQg\x13\xA0K\xBD\xFD\xC9\xBE\x88\x83a+\xC9V[\x90Pa\x05\xBD\x81aBoV[a\x1B\xA1\x82\x82`@Q`$\x01a\x1Br\x92\x91\x90aB\x8BV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\x1ES\x13G`\xE1\x1B\x17\x90Ra+\xDEV[PPV[`\0a\x05\x8A\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a%\xCDV[a\x1B\xFD\x81`@Q`$\x01a\x1B\xCE\x91\x90a:\x82V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\x10L\x13\xEB`\xE2\x1B\x17\x90Ra+\xDEV[PV[`\0\x80a\x1C\x0C\x83a+\xFFV[a\x1C\x1A\x90c;\x9A\xCA\0aB\xADV[\x90Pa\x0F\xC4\x84\x82a\x1B\xA5V[`\0\x82`\x01\x82a\x1C9\x89\x89\x89\x85\x89a\x1C\x88V[\x90P`\0\x81\x12\x15a\x1CPW`\0\x93PPPPa\x05\xBDV[a\x13\xD4\x89\x89\x89\x88`@Q` \x01a\x1Cj\x94\x93\x92\x91\x90aA\x80V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83\x85`\x01a\x01\0a,\xA3a&\x19V[`\0\x80\x82``\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x1C\xA3\x91\x90aA\xD4V[\x90P`\0a\x1C\xB5\x88\x88\x88\x85\x89\x89a,\xD4V[\x90P`\0a\x1C\xC2\x82a./V[\x90P`\0a\x1C\xCF\x83a/^V[\x90P\x80\x82\x84a\x01\0\x01Qa\x1C\xE2\x90aBoV[a\x1C\xEC\x91\x90a>1V[a\x1C\xF6\x91\x90a>1V[\x9A\x99PPPPPPPPPPV[a\x1B\xA1\x82\x82`@Q`$\x01a\x1D\x1A\x92\x91\x90aB\x8BV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c-\x83\x9C\xB3`\xE2\x1B\x17\x90Ra+\xDEV[```\x04\x83\x83`@Q` \x01a\x18\x14\x93\x92\x91\x90aAaV[`\0\x80a\x1Dv\x83` \x01Q\x84`@\x01Qa\x1C\0V[\x90P`\0a\x1D\x8C\x84` \x01Q\x85`@\x01Qa'oV[\x90P`\0a\x1D\xA7\x85`@\x01Q\x83a\x1B\xA5\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x85Q\x90\x91P`\0\x90a\x1D\xC4\x90a\x1D\xBD\x90\x89a\x1B\xA5V[\x89\x90a\x1A\xAAV[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0\x81\x12a\x1D\xE2W`\0\x94PPPPPa\x06\xFAV[`\0\x81\x13a\x1D\xF8W`\0\x19\x94PPPPPa\x06\xFAV[`\0a\x1E\x03\x82a\x1A\xBFV[\x90P`\0\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x1E\x1B\x88\x85aA\xFBV[a\x1E%\x91\x90aBAV[a\x1Aq\x91\x90a>1V[`\0\x82`\x01\x82a\x1EB\x89\x89\x89\x85\x89a!SV[\x90P`\0\x81\x12\x15a\x1EYW`\0\x93PPPPa\x05\xBDV[a\x13\xD4\x89\x89\x89\x88`@Q` \x01a\x1Es\x94\x93\x92\x91\x90aA\x80V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83\x85`\x01a\x01\0a/\xEAa&\x19V[`\0\x80\x82``\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x1E\xAC\x91\x90aA\xD4V[\x90P`\0\x83` \x01Q\x90P`\0a\x1E\xC7\x88\x86`\0\x01Qa0\x1BV[\x90P`\0a\x1E\xF9a\x1E\xE0\x84g\x1B\xC1mgN\xC8\0\0a0/V[a\x1E\xEA\x84\x86a0/V[a\x1E\xF4\x91\x90a>1V[a$)V[\x90P`\0a\x1F\x19a\x1F\x12\x83g\r\xE0\xB6\xB3\xA7d\0\0aA\xD4V[\x89\x90a0SV[\x90Pa\x1C\xF6\x89\x82aA\xD4V[`\0\x80a\x1F2\x84\x84a0\x86V[\x90P`\0a\x1F?\x82a$)V[\x90P`\0a\x1FL\x82a$\x92V[\x90Pa\tPa\x1Fc\x82g\r\xE0\xB6\xB3\xA7d\0\0a>lV[\x88\x90a\x18+V[`@\x80Q\x80\x82\x01\x90\x91R`\x0E\x81Rm\x0C.\x0E\x0EM\xEF\r-\xAC.\x8C\xAC\x8AO`\x93\x1B` \x82\x01R`\0\x90\x83\x90\x81\x90a\x1F\xA0\x90\x82a\x1D\x04V[`@\x80Q\x80\x82\x01\x90\x91R`\x11\x81Rp\x18\xDB\xDB\\\x1D]\x19Y\x12[\x9D\x98\\\x9AX[\x9D`z\x1B` \x82\x01R\x86\x90a\x1F\xD4\x90\x82a\x1B\\V[`\0\x81\x12\x15a KW[`\0\x81\x12\x15a FWa\x1F\xF6\x83a\x03\xE9a\x03\xE8a#\x9AV[\x92Pa \"`@Q\x80`@\x01`@R\x80`\x08\x81R` \x01g9<\x10:\xB882\xB9`\xC1\x1B\x81RP\x84a\x1D\x04V[\x87\x83\x11a /W\x82a 1V[\x87[\x92Pa ?\x83\x8A\x8A\x88a$\xDBV[\x90Pa\x1F\xDEV[a \xB3V[`\0\x81\x13\x15a \xB3Wa c\x82a\x03\xE7a\x03\xE8a%\xCDV[\x91Pa \x8F`@Q\x80`@\x01`@R\x80`\x08\x81R` \x01g9<\x1067\xBB\xB2\xB9`\xC1\x1B\x81RP\x83a\x1D\x04V[\x87\x82\x11a \x9CW\x81a \x9EV[\x87[\x91Pa \xAC\x82\x8A\x8A\x88a$\xDBV[\x90Pa KV[a\x13\xD4\x89\x89\x83\x88`@Q` \x01a \xCD\x94\x93\x92\x91\x90aA\x80V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83\x85`\x14`@a0\xCBa&\x19V[`\0\x80\x80a \xF8\x84\x86a\x18@V[\x90P`\0a!\x06\x87\x83a\x18+V[\x90P\x87a!\x1CWa!\x17\x87\x87a>lV[a!&V[a!&\x87\x87a>YV[\x93P\x87a!<Wa!7\x81\x86a>lV[a!FV[a!F\x81\x86a>YV[\x92PPP\x94P\x94\x92PPPV[`\0\x80\x82``\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a!n\x91\x90aA\xD4V[\x90P`\0a!\x80\x88\x88\x88\x85\x89\x89a0\xF8V[\x90P`\0a!\x8D\x82a2\xB8V[\x90P`\0a!\x9A\x83a3\xE6V[\x90P\x80\x82a\x1C\xE2g\r\xE0\xB6\xB3\xA7d\0\0aBoV[`\0\x82\x80\x85\x83\x81\x12\x15a\"DW[`\0\x81\x12\x15a\"?Wa!\xD5\x82a\x03\xE7a\x03\xE8a%\xCDV[\x85Q\x90\x92P`\0\x90a!\xE8\x90\x8A\x90a\x1A\xAAV[\x8A\x11a\"\x0CW\x85Qa!\xFB\x90\x8A\x90a\x1A\xAAV[a\"\x07\x90a\x03\xE8a>YV[a\"\x18V[a\"\x18\x8Aa\x03\xE8a>YV[\x90P\x89\x83\x10a\"'W\x82a\")V[\x80[\x92Pa\"7\x8A\x8A\x85\x89a$\xDBV[\x91PPa!\xBDV[a\"qV[`\0\x81\x13\x15a\"qWa\"\\\x83a\x03\xE9a\x03\xE8a#\x9AV[\x92Pa\"j\x89\x89\x85\x88a$\xDBV[\x90Pa\"DV[a\x13\xD4\x89\x89\x83\x88`@Q` \x01a\"\x8B\x94\x93\x92\x91\x90aA\x80V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83\x85`\x14`@a4\x92a&\x19V[```\x01\x82`@Q` \x01a\"\xBE\x92\x91\x90aB\xC4V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x91\x90PV[`\0a\x05\x8D\x82a)KV[```\x05\x82`@Q` \x01a\"\xBE\x92\x91\x90aB\xDFV[`\0\x80\x82``\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a#\x10\x91\x90aA\xD4V[\x83Q` \x85\x01Q\x91\x92P\x90`\0a#'\x89\x84a0\x1BV[\x90P`\0a#=\x83g\x1B\xC1mgN\xC8\0\0a0/V[a#G\x83\x85a0/V[a#Q\x91\x90aA\xD4V[\x90P`\0a#^\x82a$)V[\x90P`\0a#v\x82a#p\x8C\x89a0SV[\x90a0SV[\x90Pa\x1A\x9A\x8B\x82aA\xD4V[```\x03\x83\x83`@Q` \x01a\x18\x14\x93\x92\x91\x90aAaV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a#\xB2W`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[\x80Q` \x82\x01Q`@\x83\x01Q`\0\x92\x91\x90\x83a#\xE4\x83\x83a\x1C\0V[\x90P`\0a#\xF2\x88\x86a0\x1BV[\x90P`\0a$\0\x85\x85a'oV[\x90P\x82a$\r\x82\x84aA\xD4V[a$\x1F\x90g\r\xE0\xB6\xB3\xA7d\0\0aA\xFBV[a\x13\xD4\x91\x90aBAV[`\0\x80g\x13\xA0K\xBD\xFD\xC9\xBE\x88a$Gg\r\xE0\xB6\xB3\xA7d\0\0\x85aA\xFBV[a$Q\x91\x90aBAV[\x90P`\0a$^\x82aBoV[\x90P`\0a$k\x82a4\xBFV[\x90Pg\x1B\xC1mgN\xC8\0\0a$\x88g\r\xE0\xB6\xB3\xA7d\0\0\x83aA\xFBV[a\x05\xBD\x91\x90aBAV[`\0\x80\x82\x12\x15a$\xD7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01RotoUint: negative`\x80\x1B`D\x82\x01R`d\x01a\x07cV[P\x90V[`\0\x82\x85\x10a%,W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FtradingFunction: invalid x\0\0\0\0\0\0`D\x82\x01R`d\x01a\x07cV[`\0\x80g\r\xE0\xB6\xB3\xA7d\0\0a%B\x88\x87a\x1A\xAAV[\x10a%VW`\x01`\x01`\xFF\x1B\x03\x91Pa%fV[a%ca\x07\xA3\x88\x87a\x1A\xAAV[\x91P[g\r\xE0\xB6\xB3\xA7d\0\0a%\x81\x87a\x07\xE8\x87`\0\x01Q\x89a\x1B\xA5V[\x10a%\x94WP`\x01`\x01`\xFF\x1B\x03a%\xACV[a%\xA9a\x07\xA3\x87a\x07\xE8\x87`\0\x01Q\x89a\x1B\xA5V[\x90P[`\0a%\xC0\x85` \x01Q\x86`@\x01Qa\x1C\0V[\x90P\x80a\t\x1A\x83\x85a>1V[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a%\xE5W`\0\x80\xFD[\x04\x92\x91PPV[`\0\x80`\0\x80\x85\x80` \x01\x90Q\x81\x01\x90a&\x06\x91\x90aC\x05V[\x93PP\x92P\x92Pa\x108\x83\x86\x84\x84a$\xDBV[`\0\x84\x86\x11\x15a&FW`@Qc0\x82\xDF\xDB`\xE1\x1B\x81R`\x04\x81\x01\x87\x90R`$\x81\x01\x86\x90R`D\x01a\x07cV[`\0a&V\x88\x88\x85c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a&h\x89\x88\x86c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a&v\x82\x84aA\xFBV[\x13\x15a&\x9FW`@Qc\x06\xF1\xBE]`\xE2\x1B\x81R`\x04\x81\x01\x83\x90R`$\x81\x01\x82\x90R`D\x01a\x07cV[`\0a&\xAB\x89\x89a>lV[\x90P`\0[`\x02a&\xBC\x8A\x8Ca>YV[a&\xC6\x91\x90aC>V[\x94P`\0a&\xD8\x8C\x87\x89c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a&\xE6\x86\x83aA\xFBV[\x13a&\xF3W\x85\x99Pa&\xFAV[\x85\x9AP\x80\x94P[a'\x04\x8B\x8Ba>lV[\x92PP`\x01\x01\x87\x82\x11\x80\x15a'\x18WP\x86\x81\x10[a&\xB0WPPPP\x96\x95PPPPPPV[`\0\x80a'7\x84\x84a0\x86V[\x90P`\0a'D\x82a$)V[\x90P`\0a'Q\x82a$\x92V[\x90Pa\tPa'h\x82g\r\xE0\xB6\xB3\xA7d\0\0a>lV[\x88\x90a\x18@V[`\0\x80a'\x8E\x83a'\x88\x86g\x1B\xC1mgN\xC8\0\0a6\xA3V[\x90a\x1B\xA5V[\x90Pa\x0F\xC4g\x06\xF0[Y\xD3\xB2\0\0\x82a\x1B\xA5V[`\0h\x02H\xCE6\xA7\x0C\xB2k>\x19\x82\x13a'\xBDWP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a(\x04W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x01a\x07cV[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05`\x01`_\x1B\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1Dm\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1Dn\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[`\0\x80\x82\x12\x80a)bWPg\x1B\xC1mgN\xC8\0\0\x82\x13[\x15a)\x80W`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a)\xA1W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81g\x1B\xC1mgN\xC8\0\0\x03a)\xC9W`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x15a)\xD4W\x91\x90PV[`\0g\r\xE0\xB6\xB3\xA7d\0\0\x83\x12a)\xFCWa)\xF7\x83g\x1B\xC1mgN\xC8\0\0aA\xD4V[a)\xFEV[\x82[\x90P`\0a*\x14\x82g\x1B\xC1mgN\xC8\0\0a6\xCFV[\x90P\x80`\0\x03a*7W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a*B\x82a6\xE4V[\x90P`\0c;\x9A\xCA\0a*ma*ha*bg\x1B\xC1mgN\xC8\0\0aBoV[\x85a+\xC9V[a+\xFFV[a*w\x91\x90aA\xFBV[\x90P`\0\x80a*\x8E\x83g\x03\xC1f\\z\xAB \0a+\xC9V[a*\xA0\x90g \x05\xFEO&\x8E\xA0\0a>1V[\x90P`\0a*\xD0\x84a*\xB9\x86f\x9F2u$b\xA0\0a+\xC9V[a*\xCB\x90g\r\xC5R\x7Fd, \0a>1V[a+\xC9V[a*\xE2\x90g\r\xE0\xB6\xB3\xA7d\0\0a>1V[\x90Pa+\x06g\t\xD0(\xCCo _\xFF\x19\x85a*\xFC\x85\x85a6\xCFV[a*\xCB\x91\x90aA\xD4V[\x92PPP`\0[`\x02\x81\x10\x15a+\xA1W`\0\x86a+\"\x84a4\xBFV[a+,\x91\x90aA\xD4V[\x90P`\0a+:\x84\x85a+\xC9V[a+C\x90aBoV[\x90P`\0a+P\x82a'\xA2V[\x90P`\0a+^\x86\x85a+\xC9V[a+pg\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x84a+\xC9V[a+z\x91\x90aA\xD4V[\x90Pa+\x86\x84\x82a6\xCFV[a+\x90\x90\x87a>1V[\x95P\x84`\x01\x01\x94PPPPPa+\rV[g\r\xE0\xB6\xB3\xA7d\0\0\x88\x12a+\xBEWa+\xB9\x82aBoV[a\x06\xDEV[P\x96\x95PPPPPPV[`\0a\x05\x8A\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a8\xBFV[\x80Qjconsole.log` \x83\x01`\0\x80\x84\x83\x85Z\xFAPPPPPV[`\xB5\x81`\x01`\x88\x1B\x81\x10a,\x18W`@\x91\x90\x91\x1B\x90`\x80\x1C[i\x01\0\0\0\0\0\0\0\0\0\x81\x10a,4W` \x91\x90\x91\x1B\x90`@\x1C[e\x01\0\0\0\0\0\x81\x10a,LW`\x10\x91\x90\x91\x1B\x90` \x1C[c\x01\0\0\0\x81\x10a,bW`\x08\x91\x90\x91\x1B\x90`\x10\x1C[b\x01\0\0\x01\x02`\x12\x1C\x80\x82\x04\x01`\x01\x90\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x90\x81\x90\x04\x81\x11\x90\x03\x90V[`\0\x80`\0\x80`\0\x86\x80` \x01\x90Q\x81\x01\x90a,\xBF\x91\x90aC\x05V[\x93P\x93P\x93P\x93Pa\tP\x84\x84\x84\x89\x85a\x1C\x88V[a-1`@Q\x80a\x01`\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`\0a-Oa-@\x88\x86a>1V[g\x1B\xC1mgN\xC8\0\0\x90a0SV[\x90P`\0a-]\x85\x87a0SV[a-g\x86\x89a>1V[a-q\x91\x90aA\xD4V[\x90P`\0a-\x87a-\x82\x84\x84a0/V[a)KV[\x90P`\0a-\x9Cg\x1B\xC1mgN\xC8\0\0a+\xFFV[a-\xAA\x90c;\x9A\xCA\0aB\xADV[\x90P`\0a-\xBB\x87`@\x01Qa+\xFFV[a-\xC9\x90c;\x9A\xCA\0aB\xADV[`@\x80Qa\x01`\x81\x01\x82R\x94\x85R\x88Q` \x80\x87\x01\x91\x90\x91R\x89\x01Q\x85\x82\x01R\x97\x90\x97\x01Q``\x84\x01RP`\x80\x82\x01\x97\x90\x97R`\xA0\x81\x01\x98\x90\x98RPPP`\xC0\x85\x01\x93\x90\x93R`\xE0\x84\x01Ra\x01\0\x83\x01\x93\x90\x93Ra\x01 \x82\x01Ra\x01@\x81\x01\x91\x90\x91R\x90V[`\0\x80a.gg\x1B\xC1mgN\xC8\0\0a.a\x85``\x01Qa#p\x87`@\x01Q\x88`@\x01Qa0S\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90a0/V[a.p\x90aBoV[\x90P`\0a.\xA1\x84`\0\x01Qa#p\x86a\x01 \x01Qa#p\x88`@\x01Q\x89a\x01@\x01Qa0S\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a.\xB7a.\xB2\x83\x85a>1V[a'\xA2V[\x90P`\0a/\x07a.\xED\x87`\x80\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a.\xD8\x90aBoV[a.\xE2\x91\x90a>1V[`\xA0\x89\x01Q\x90a0SV[\x87`\xC0\x01Qa.\xFC\x91\x90a>1V[` \x88\x01Q\x90a0SV[\x90P`\0a/\x15\x83\x83a0SV[\x90P`\0a/4\x88`\x80\x01Q\x89`\xE0\x01Qa0S\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x88`\xE0\x01Q\x89`\xC0\x01Qa/H\x91\x90a>1V[a/R\x91\x90aA\xD4V[\x90Pa\x06\xDE\x82\x82a0/V[`\0\x80a/\x9D\x83`\x80\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a/{\x90aBoV[a/\x85\x91\x90a>1V[` \x85\x01Qa#p\x90g\x06\xF0[Y\xD3\xB2\0\0\x90a0SV[\x90P`\0a/\xC6\x84a\x01@\x01Qa.a\x86a\x01 \x01Q\x87`@\x01Qa0S\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa\x0F\xC4a/\xE3\x85`\0\x01Q\x83a/\xDE\x91\x90aA\xD4V[a4\xBFV[\x83\x90a0SV[`\0\x80`\0\x80`\0\x86\x80` \x01\x90Q\x81\x01\x90a0\x06\x91\x90aC\x05V[\x93P\x93P\x93P\x93Pa\tP\x84\x84\x84\x89\x85a!SV[`\0a\x05\x8Aa0*\x84\x84a\x18@V[a6\xE4V[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x81\x02\x90\x81\x05\x83\x14\x82\x15\x15\x16a0MW`\0\x80\xFD[\x05\x91\x90PV[\x81\x81\x02\x82\x15\x83\x82\x05\x83\x14\x17`\0\x19\x84\x10`\x01`\xFF\x1B\x84\x13\x17\x16a0uW`\0\x80\xFD[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x92\x91PPV[\x80Q` \x82\x01Q`@\x83\x01Q`\0\x92\x91\x90\x83a0\xA2\x83\x83a\x1C\0V[\x90P`\0a0\xB0\x88\x86a0\x1BV[\x90P`\0a0\xBE\x85\x85a'oV[\x90P\x82a$\r\x82\x84a>1V[`\0\x80`\0\x80\x85\x80` \x01\x90Q\x81\x01\x90a0\xE5\x91\x90aC\x05V[\x93PP\x92P\x92Pa\x108\x85\x84\x84\x84a$\xDBV[a1U`@Q\x80a\x01`\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`\0a1da-@\x88\x86a>1V[\x90P`\0a1r\x85\x87a0SV[\x84Q\x86\x90a1\x80\x90\x8Aa0SV[a1\x8A\x91\x90a>1V[a1\x94\x91\x90aA\xD4V[\x90P`\0a1\xA2\x83\x83a0/V[\x90Pa1\xD2`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01kintermediate`\xA0\x1B\x81RP\x82a\x1B\\V[`\0a1\xE1a-\x82\x85\x85a0/V[\x90Pa2\x0E`@Q\x80`@\x01`@R\x80`\t\x81R` \x01hierfc res`\xB8\x1B\x81RP\x82a\x1B\\V[`\0a2!g\x1B\xC1mgN\xC8\0\0a+\xFFV[a2/\x90c;\x9A\xCA\0aB\xADV[\x90P`\0a2@\x88`@\x01Qa+\xFFV[a2N\x90c;\x9A\xCA\0aB\xADV[`@\x80Qa\x01`\x81\x01\x82R\x94\x85R\x89Q` \x80\x87\x01\x91\x90\x91R\x8A\x01Q\x85\x82\x01R\x98\x90\x98\x01Q``\x84\x01RP`\x80\x82\x01\x98\x90\x98R`\xA0\x81\x01\x99\x90\x99RPPPP`\xC0\x85\x01\x93\x90\x93R`\xE0\x84\x01Ra\x01\0\x83\x01\x93\x90\x93Ra\x01 \x82\x01\x92\x90\x92Ra\x01@\x81\x01\x91\x90\x91R\x90V[`\0\x80a2\xEAg\x1B\xC1mgN\xC8\0\0a.a\x85``\x01Qa#p\x87`@\x01Q\x88`@\x01Qa0S\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a2\xF3\x90aBoV[\x90P`\0a3$\x84`\0\x01Qa#p\x86a\x01@\x01Qa#p\x88`@\x01Q\x89a\x01 \x01Qa0S\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a35a.\xB2\x83\x85a>1V[\x90P`\0a3\x7Fa3V\x87`\x80\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a.\xD8\x90aBoV[`\xC0\x88\x01Q` \x89\x01Qa3i\x91a0SV[a3s\x91\x90a>1V[a\x01\0\x88\x01Q\x90a0SV[\x90P`\0a3\x8D\x83\x83a0SV[\x90P`\0a/Ra3\xAF\x89`\x80\x01Q\x8A`\xE0\x01Qa0S\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[`\xE0\x8A\x01Q`\xC0\x8B\x01Q` \x8C\x01Qa3\xC7\x91a0SV[a3\xD1\x91\x90a>1V[a3\xDB\x91\x90aA\xD4V[` \x8A\x01Q\x90a0SV[`\0\x80a4\x19\x83`\x80\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a4\x03\x90aBoV[a4\r\x91\x90a>1V[a\x01\0\x85\x01Q\x90a0SV[\x90P`\0a4B\x84a\x01 \x01Qa.a\x86a\x01@\x01Q\x87`@\x01Qa0S\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a4ca4\\\x86`\0\x01Q\x84a/\xDE\x91\x90aA\xD4V[\x84\x90a0SV[\x90P`\0a4\x86\x86` \x01Qg\x1B\xC1mgN\xC8\0\0a0S\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa\x108\x82\x82a0/V[`\0\x80`\0\x80\x85\x80` \x01\x90Q\x81\x01\x90a4\xAC\x91\x90aC\x05V[\x93PP\x92P\x92Pa\x108\x83\x83\x87\x84a$\xDBV[`\0\x81`\0\x03a4\xD8WPg\r\xE0\xB6\xB3\xA7d\0\0\x91\x90PV[gV\x98\xEE\xF0fp\0\0\x82\x12a4\xEFWP`\0\x91\x90PV[a5\0gV\x98\xEE\xF0fp\0\0aBoV[\x82\x13a5\x15WPg\x1B\xC1mgN\xC8\0\0\x91\x90PV[`\0a5 \x83a8\xDEV[\x90P`\0a5Yg\r\xE0\xB6\xB3\xA7d\0\0a5B\x84g\x1B\xC1mgN\xC8\0\0a\x1A\xAAV[a5T\x90g\r\xE0\xB6\xB3\xA7d\0\0a>1V[a6\xCFV[\x90P`\0\x80\x82a5\xB5\x81a5\xA2\x81a5\x90\x81a5}\x81g\x02_\x0F\xE1\x05\xA3\x14\0a+\xC9V[a*\xCB\x90g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19a>1V[a*\xCB\x90g\x14\xA8EL\x19\xE1\xAC\0a>1V[a*\xCB\x90g\x0F\xC1\x0E\x01W\x82w\xFF\x19a>1V[a5\xC7\x90g\x03\xDE\xBD\x08;\x8C|\0a>1V[\x91P\x83\x90Pa6/\x81a6\x1D\x81a6\x0B\x81a5\xF9\x81a5\xE6\x81\x8Ba+\xC9V[a*\xCB\x90g\x02\x95\xD4\0\xEA2W\xFF\x19a>1V[a*\xCB\x90g\x01W\xD8\xB2\xEC\xC7\x08\0a>1V[a*\xCB\x90g\x051\n\xA7\xD5!0\0a>1V[a*\xCB\x90g\r\xE0\xCC=\x15a\0\0a>1V[\x91P\x81g\x11\x90\0\xAB\x10\x0F\xFC\0a6E\x87\x88a+\xC9V[a6Q\x90`\0\x19aA\xFBV[a6[\x91\x90aA\xD4V[a6e\x91\x90a>1V[\x92PP`\0a6s\x83a'\xA2V[\x90P`\0a6\x81\x85\x83a+\xC9V[\x90P`\0\x88\x12a6\x91W\x80a\x06\xDEV[a\x06\xDE\x81g\x1B\xC1mgN\xC8\0\0aA\xD4V[`\0a\x05\x8Ag\r\xE0\xB6\xB3\xA7d\0\0\x83a6\xBB\x86a6\xE4V[a6\xC5\x91\x90aA\xFBV[a.\xB2\x91\x90aBAV[`\0a\x05\x8A\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a8\xBFV[`\0\x80\x82\x13a7!W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x07cV[`\0``a7.\x84a9\x15V[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x05\x85\x14\x17\x16a8\xD7W`\0\x80\xFD[\x05\x92\x91PPV[`\0`\x01`\xFF\x1B\x82\x03a9\x04W`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x12\x15a$\xD7WP\x19`\x01\x01\x90V[`\0\x80\x82\x11a9RW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x07cV[P`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x82\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`\0\x80`@\x83\x85\x03\x12\x15a:#Wa:#a9\xBDV[PP\x805\x92` \x90\x91\x015\x91PV[`\0[\x83\x81\x10\x15a:MW\x81\x81\x01Q\x83\x82\x01R` \x01a:5V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra:n\x81` \x86\x01` \x86\x01a:2V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x05\x8A` \x83\x01\x84a:VV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a:\xAEWa:\xAEa9\xBDV[PP\x825\x94` \x84\x015\x94P`@\x84\x015\x93``\x015\x92P\x90PV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FABI decoding: struct data too sh`D\x82\x01Rb\x1B\xDC\x9D`\xEA\x1B`d\x82\x01R`\x84\x81\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a;TWa;Ta;\x1BV[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a;\x83Wa;\x83a;\x1BV[`@R\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x1B\xFDW`\0\x80\xFD[`\0`\xA0\x82\x84\x03\x12\x15a;\xB5Wa;\xB5a:\xCAV[a;\xBDa;1V[\x90P\x815\x81R` \x82\x015` \x82\x01R`@\x82\x015`@\x82\x01R``\x82\x015``\x82\x01R`\x80\x82\x015a;\xEF\x81a;\x8BV[`\x80\x82\x01R\x92\x91PPV[`\0\x80`\0`\xE0\x84\x86\x03\x12\x15a<\x12Wa<\x12a9\xBDV[\x835\x92P` \x84\x015\x91Pa<*\x85`@\x86\x01a;\xA0V[\x90P\x92P\x92P\x92V[`\0\x80`\0``\x84\x86\x03\x12\x15a<KWa<Ka9\xBDV[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[`\0\x80`\0\x80a\x01\0\x85\x87\x03\x12\x15a<|Wa<|a9\xBDV[\x845\x93P` \x85\x015\x92P`@\x85\x015\x91Pa<\x9B\x86``\x87\x01a;\xA0V[\x90P\x92\x95\x91\x94P\x92PV[\x80\x15\x15\x81\x14a\x1B\xFDW`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15a<\xCCWa<\xCCa9\xBDV[\x835\x92P` \x84\x015a<\xDE\x81a<\xA6V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[\x84\x15\x15\x81R\x83` \x82\x01R\x82`@\x82\x01R`\x80``\x82\x01R`\0a\x108`\x80\x83\x01\x84a:VV[`\0` \x82\x84\x03\x12\x15a=+Wa=+a9\xBDV[P5\x91\x90PV[\x81Q\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x80\x83\x01Q\x90\x82\x01R``\x80\x83\x01Q\x90\x82\x01R`\x80\x80\x83\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x90\x82\x01R`\xA0\x81\x01a\x05\x8DV[`\0\x80`\0\x80`\0\x80a\x01@\x87\x89\x03\x12\x15a=\x8DWa=\x8Da9\xBDV[\x865\x95P` \x87\x015\x94P`@\x87\x015\x93P``\x87\x015\x92P`\x80\x87\x015\x91Pa=\xBA\x88`\xA0\x89\x01a;\xA0V[\x90P\x92\x95P\x92\x95P\x92\x95V[`\0` \x82\x84\x03\x12\x15a=\xDBWa=\xDBa9\xBDV[\x815a\x06\xFA\x81a;\x8BV[\x82\x81R`@` \x82\x01R`\0a\x06\xF7`@\x83\x01\x84a:VV[`\0` \x82\x84\x03\x12\x15a>\x14Wa>\x14a9\xBDV[PQ\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x82\x81\x12`\0\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a>QWa>Qa>\x1BV[PP\x92\x91PPV[\x80\x82\x01\x80\x82\x11\x15a\x05\x8DWa\x05\x8Da>\x1BV[\x81\x81\x03\x81\x81\x11\x15a\x05\x8DWa\x05\x8Da>\x1BV[`\x01\x80`\xA0\x1B\x03\x84\x16\x81R\x82` \x82\x01R```@\x82\x01R`\0a\x05\xBD``\x83\x01\x84a:VV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a>\xC2Wa>\xC2a9\xBDV[\x86Qa>\xCD\x81a<\xA6V[` \x88\x01Q`@\x89\x01Q``\x8A\x01Q`\x80\x8B\x01Q`\xA0\x90\x9B\x01Q\x93\x9C\x92\x9BP\x90\x99\x90\x98P\x96P\x90\x94P\x92PPPV[`\0` \x80\x83\x85\x03\x12\x15a?\x12Wa?\x12a9\xBDV[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a?uW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x84\x90R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a?\xDDW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x84\x90R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x81\xFD[\x81Q\x81\x81\x11\x15a?\xEFWa?\xEFa;\x1BV[a@\x01`\x1F\x82\x01`\x1F\x19\x16\x85\x01a;ZV[\x91P\x80\x82R\x86\x84\x82\x85\x01\x01\x11\x15a@gW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x85\x90R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R`\x84\x81\xFD[a@v\x81\x85\x84\x01\x86\x86\x01a:2V[P\x95\x94PPPPPV[`\0`\xA0\x82\x84\x03\x12\x15a@\x95Wa@\x95a:\xCAV[a@\x9Da;1V[\x90P\x81Q\x81R` \x82\x01Q` \x82\x01R`@\x82\x01Q`@\x82\x01R``\x82\x01Q``\x82\x01R`\x80\x82\x01Qa;\xEF\x81a;\x8BV[`\0`\xA0\x82\x84\x03\x12\x15a@\xE4Wa@\xE4a9\xBDV[a\x05\x8A\x83\x83a@\x80V[`\0` \x82\x84\x03\x12\x15aA\x03WaA\x03a9\xBDV[\x81Qa\x06\xFA\x81a;\x8BV[`\0\x80`\0``\x84\x86\x03\x12\x15aA&WaA&a9\xBDV[\x83Q\x92P` \x84\x01Q\x91P`@\x84\x01Q\x90P\x92P\x92P\x92V[`\x06\x81\x10aA]WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x90RV[``\x81\x01aAo\x82\x86aA?V[` \x82\x01\x93\x90\x93R`@\x01R\x91\x90PV[\x84\x81R` \x81\x01\x84\x90R`@\x81\x01\x83\x90Ra\x01\0\x81\x01a\x05\xBD``\x83\x01\x84\x80Q\x82R` \x80\x82\x01Q\x90\x83\x01R`@\x80\x82\x01Q\x90\x83\x01R``\x80\x82\x01Q\x90\x83\x01R`\x80\x90\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x91\x01RV[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15aA\xF4WaA\xF4a>\x1BV[P\x92\x91PPV[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15aB\x17WaB\x17a>\x1BV[\x81\x81\x05\x83\x14\x82\x15\x17a\x05\x8DWa\x05\x8Da>\x1BV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82aBPWaBPaB+V[`\x01`\xFF\x1B\x82\x14`\0\x19\x84\x14\x16\x15aBjWaBja>\x1BV[P\x05\x90V[`\0`\x01`\xFF\x1B\x82\x01aB\x84WaB\x84a>\x1BV[P`\0\x03\x90V[`@\x81R`\0aB\x9E`@\x83\x01\x85a:VV[\x90P\x82` \x83\x01R\x93\x92PPPV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x05\x8DWa\x05\x8Da>\x1BV[`@\x81\x01aB\xD2\x82\x85aA?V[\x82` \x83\x01R\x93\x92PPPV[`@\x81\x01aB\xED\x82\x85aA?V[`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16` \x91\x90\x91\x01R\x91\x90PV[`\0\x80`\0\x80a\x01\0\x85\x87\x03\x12\x15aC\x1FWaC\x1Fa9\xBDV[\x84Q\x93P` \x85\x01Q\x92P`@\x85\x01Q\x91Pa<\x9B\x86``\x87\x01a@\x80V[`\0\x82aCMWaCMaB+V[P\x04\x90V\xFETarget contract does not contain\xA2dipfsX\"\x12 .\x96\xCD\xB3h\xA7uSqUP\x99\xBA>\xCF\x97\xEE\xCF\xBD&\xC9\xB6 \x06\xBC\x87\xCB\x9CcF\xE5}dsolcC\0\x08\x16\x003";
    /// The deployed bytecode of the contract.
    pub static LOGNORMALSOLVER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct LogNormalSolver<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for LogNormalSolver<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for LogNormalSolver<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for LogNormalSolver<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for LogNormalSolver<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(LogNormalSolver))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> LogNormalSolver<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    LOGNORMALSOLVER_ABI.clone(),
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
                LOGNORMALSOLVER_ABI.clone(),
                LOGNORMALSOLVER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `BISECTION_EPSILON` (0x6d652299) function
        pub fn bisection_epsilon(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([109, 101, 34, 153], ())
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
        ///Calls the contract's `allocateGivenX` (0xee3e8cfb) function
        pub fn allocate_given_x(
            &self,
            pool_id: ::ethers::core::types::U256,
            amount_x: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([238, 62, 140, 251], (pool_id, amount_x))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `allocateGivenY` (0x7f17409c) function
        pub fn allocate_given_y(
            &self,
            pool_id: ::ethers::core::types::U256,
            amount_y: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([127, 23, 64, 156], (pool_id, amount_y))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `calculateDiffLower` (0x332266f3) function
        pub fn calculate_diff_lower(
            &self,
            pool_id: ::ethers::core::types::U256,
            s: ::ethers::core::types::U256,
            v: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([51, 34, 102, 243], (pool_id, s, v))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `calculateDiffRaise` (0x902ecaa2) function
        pub fn calculate_diff_raise(
            &self,
            pool_id: ::ethers::core::types::U256,
            s: ::ethers::core::types::U256,
            v: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([144, 46, 202, 162], (pool_id, s, v))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `callIerfc` (0xc5e73169) function
        pub fn call_ierfc(
            &self,
            x: ::ethers::core::types::I256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([197, 231, 49, 105], x)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `computeDeltaL` (0x0730553c) function
        pub fn compute_delta_l(
            &self,
            amount_in: ::ethers::core::types::U256,
            swap_fee: ::ethers::core::types::U256,
            l: ::ethers::core::types::U256,
            reserve: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([7, 48, 85, 60], (amount_in, swap_fee, l, reserve))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `computeDeltaLXIn` (0x9a6ec162) function
        pub fn compute_delta_lx_in(
            &self,
            pool_id: ::ethers::core::types::U256,
            amount_in: ::ethers::core::types::U256,
            rx: ::ethers::core::types::U256,
            ry: ::ethers::core::types::U256,
            l: ::ethers::core::types::U256,
            params: LogNormalParams,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [154, 110, 193, 98],
                    (pool_id, amount_in, rx, ry, l, params),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `computeDeltaLYIn` (0x9bdc031c) function
        pub fn compute_delta_ly_in(
            &self,
            pool_id: ::ethers::core::types::U256,
            amount_in: ::ethers::core::types::U256,
            rx: ::ethers::core::types::U256,
            ry: ::ethers::core::types::U256,
            l: ::ethers::core::types::U256,
            params: LogNormalParams,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([155, 220, 3, 28], (pool_id, amount_in, rx, ry, l, params))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `computeOptimalArbLowerPrice` (0x306db46b) function
        pub fn compute_optimal_arb_lower_price(
            &self,
            pool_id: ::ethers::core::types::U256,
            s: ::ethers::core::types::U256,
            v_upper: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([48, 109, 180, 107], (pool_id, s, v_upper))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `computeOptimalArbRaisePrice` (0x4fd67c58) function
        pub fn compute_optimal_arb_raise_price(
            &self,
            pool_id: ::ethers::core::types::U256,
            s: ::ethers::core::types::U256,
            v_upper: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([79, 214, 124, 88], (pool_id, s, v_upper))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deallocateGivenX` (0x6237569f) function
        pub fn deallocate_given_x(
            &self,
            pool_id: ::ethers::core::types::U256,
            amount_x: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([98, 55, 86, 159], (pool_id, amount_x))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deallocateGivenY` (0xf30d37f2) function
        pub fn deallocate_given_y(
            &self,
            pool_id: ::ethers::core::types::U256,
            amount_y: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([243, 13, 55, 242], (pool_id, amount_y))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `fetchPoolParams` (0x81b5fac2) function
        pub fn fetch_pool_params(
            &self,
            pool_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, LogNormalParams> {
            self.0
                .method_hash([129, 181, 250, 194], pool_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getDxGivenS` (0x55f011c6) function
        pub fn get_dx_given_s(
            &self,
            pool_id: ::ethers::core::types::U256,
            s: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([85, 240, 17, 198], (pool_id, s))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getDyGivenS` (0xe45d593c) function
        pub fn get_dy_given_s(
            &self,
            pool_id: ::ethers::core::types::U256,
            s: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([228, 93, 89, 60], (pool_id, s))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getInitialPoolData` (0x134ead12) function
        pub fn get_initial_pool_data(
            &self,
            rx: ::ethers::core::types::U256,
            s: ::ethers::core::types::U256,
            params: LogNormalParams,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([19, 78, 173, 18], (rx, s, params))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getNextLiquidity` (0xaf4e437f) function
        pub fn get_next_liquidity(
            &self,
            pool_id: ::ethers::core::types::U256,
            rx: ::ethers::core::types::U256,
            ry: ::ethers::core::types::U256,
            l: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([175, 78, 67, 127], (pool_id, rx, ry, l))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getNextReserveX` (0x5eb408fc) function
        pub fn get_next_reserve_x(
            &self,
            pool_id: ::ethers::core::types::U256,
            ry: ::ethers::core::types::U256,
            l: ::ethers::core::types::U256,
            s: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([94, 180, 8, 252], (pool_id, ry, l, s))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getNextReserveY` (0x120649c5) function
        pub fn get_next_reserve_y(
            &self,
            pool_id: ::ethers::core::types::U256,
            rx: ::ethers::core::types::U256,
            l: ::ethers::core::types::U256,
            s: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([18, 6, 73, 197], (pool_id, rx, l, s))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPriceGivenXL` (0x1e978cb0) function
        pub fn get_price_given_xl(
            &self,
            pool_id: ::ethers::core::types::U256,
            rx: ::ethers::core::types::U256,
            l: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([30, 151, 140, 176], (pool_id, rx, l))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPriceGivenYL` (0x4e817fd9) function
        pub fn get_price_given_yl(
            &self,
            pool_id: ::ethers::core::types::U256,
            ry: ::ethers::core::types::U256,
            l: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([78, 129, 127, 217], (pool_id, ry, l))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getReservesAndLiquidity` (0xce153bf4) function
        pub fn get_reserves_and_liquidity(
            &self,
            pool_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([206, 21, 59, 244], pool_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `internalPrice` (0x3b4d1030) function
        pub fn internal_price(
            &self,
            pool_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([59, 77, 16, 48], pool_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `prepareControllerUpdate` (0xcb1f5532) function
        pub fn prepare_controller_update(
            &self,
            controller: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([203, 31, 85, 50], controller)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `prepareFeeUpdate` (0xb09d04e5) function
        pub fn prepare_fee_update(
            &self,
            swap_fee: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([176, 157, 4, 229], swap_fee)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `prepareSigmaUpdate` (0xe94716d5) function
        pub fn prepare_sigma_update(
            &self,
            target_sigma: ::ethers::core::types::U256,
            target_timestamp: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([233, 71, 22, 213], (target_sigma, target_timestamp))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `prepareStrikeUpdate` (0x0420580a) function
        pub fn prepare_strike_update(
            &self,
            target_strike: ::ethers::core::types::U256,
            target_timestamp: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([4, 32, 88, 10], (target_strike, target_timestamp))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `prepareTauUpdate` (0x3b268d5d) function
        pub fn prepare_tau_update(
            &self,
            target_tau: ::ethers::core::types::U256,
            target_timestamp: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([59, 38, 141, 93], (target_tau, target_timestamp))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `simulateSwap` (0x3928ff97) function
        pub fn simulate_swap(
            &self,
            pool_id: ::ethers::core::types::U256,
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
                .method_hash([57, 40, 255, 151], (pool_id, swap_x_in, amount_in))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `solverTradingFunction` (0x24de2573) function
        pub fn solver_trading_function(
            &self,
            rx: ::ethers::core::types::U256,
            ry: ::ethers::core::types::U256,
            l: ::ethers::core::types::U256,
            params: LogNormalParams,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([36, 222, 37, 115], (rx, ry, l, params))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `strategy` (0xa8c62e76) function
        pub fn strategy(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([168, 198, 46, 118], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for LogNormalSolver<M> {
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
    pub enum LogNormalSolverErrors {
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
    impl ::ethers::core::abi::AbiDecode for LogNormalSolverErrors {
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
    impl ::ethers::core::abi::AbiEncode for LogNormalSolverErrors {
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
    impl ::ethers::contract::ContractRevert for LogNormalSolverErrors {
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
    impl ::core::fmt::Display for LogNormalSolverErrors {
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
    impl ::core::convert::From<::std::string::String> for LogNormalSolverErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<BisectionLib_InvalidBounds> for LogNormalSolverErrors {
        fn from(value: BisectionLib_InvalidBounds) -> Self {
            Self::BisectionLib_InvalidBounds(value)
        }
    }
    impl ::core::convert::From<BisectionLib_RootOutsideBounds>
    for LogNormalSolverErrors {
        fn from(value: BisectionLib_RootOutsideBounds) -> Self {
            Self::BisectionLib_RootOutsideBounds(value)
        }
    }
    impl ::core::convert::From<Infinity> for LogNormalSolverErrors {
        fn from(value: Infinity) -> Self {
            Self::Infinity(value)
        }
    }
    impl ::core::convert::From<Min> for LogNormalSolverErrors {
        fn from(value: Min) -> Self {
            Self::Min(value)
        }
    }
    impl ::core::convert::From<NegativeInfinity> for LogNormalSolverErrors {
        fn from(value: NegativeInfinity) -> Self {
            Self::NegativeInfinity(value)
        }
    }
    impl ::core::convert::From<OutOfBounds> for LogNormalSolverErrors {
        fn from(value: OutOfBounds) -> Self {
            Self::OutOfBounds(value)
        }
    }
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
        Hash
    )]
    #[ethcall(name = "BISECTION_EPSILON", abi = "BISECTION_EPSILON()")]
    pub struct BisectionEpsilonCall;
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
        Hash
    )]
    #[ethcall(name = "MAX_BISECTION_ITERS", abi = "MAX_BISECTION_ITERS()")]
    pub struct MaxBisectionItersCall;
    ///Container type for all input parameters for the `allocateGivenX` function with signature `allocateGivenX(uint256,uint256)` and selector `0xee3e8cfb`
    #[derive(
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
    #[ethcall(name = "allocateGivenX", abi = "allocateGivenX(uint256,uint256)")]
    pub struct AllocateGivenXCall {
        pub pool_id: ::ethers::core::types::U256,
        pub amount_x: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `allocateGivenY` function with signature `allocateGivenY(uint256,uint256)` and selector `0x7f17409c`
    #[derive(
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
    #[ethcall(name = "allocateGivenY", abi = "allocateGivenY(uint256,uint256)")]
    pub struct AllocateGivenYCall {
        pub pool_id: ::ethers::core::types::U256,
        pub amount_y: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `calculateDiffLower` function with signature `calculateDiffLower(uint256,uint256,uint256)` and selector `0x332266f3`
    #[derive(
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
        name = "calculateDiffLower",
        abi = "calculateDiffLower(uint256,uint256,uint256)"
    )]
    pub struct CalculateDiffLowerCall {
        pub pool_id: ::ethers::core::types::U256,
        pub s: ::ethers::core::types::U256,
        pub v: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `calculateDiffRaise` function with signature `calculateDiffRaise(uint256,uint256,uint256)` and selector `0x902ecaa2`
    #[derive(
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
        name = "calculateDiffRaise",
        abi = "calculateDiffRaise(uint256,uint256,uint256)"
    )]
    pub struct CalculateDiffRaiseCall {
        pub pool_id: ::ethers::core::types::U256,
        pub s: ::ethers::core::types::U256,
        pub v: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `callIerfc` function with signature `callIerfc(int256)` and selector `0xc5e73169`
    #[derive(
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
    #[ethcall(name = "callIerfc", abi = "callIerfc(int256)")]
    pub struct CallIerfcCall {
        pub x: ::ethers::core::types::I256,
    }
    ///Container type for all input parameters for the `computeDeltaL` function with signature `computeDeltaL(uint256,uint256,uint256,uint256)` and selector `0x0730553c`
    #[derive(
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
        name = "computeDeltaL",
        abi = "computeDeltaL(uint256,uint256,uint256,uint256)"
    )]
    pub struct ComputeDeltaLCall {
        pub amount_in: ::ethers::core::types::U256,
        pub swap_fee: ::ethers::core::types::U256,
        pub l: ::ethers::core::types::U256,
        pub reserve: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `computeDeltaLXIn` function with signature `computeDeltaLXIn(uint256,uint256,uint256,uint256,uint256,(uint256,uint256,uint256,uint256,address))` and selector `0x9a6ec162`
    #[derive(
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
        name = "computeDeltaLXIn",
        abi = "computeDeltaLXIn(uint256,uint256,uint256,uint256,uint256,(uint256,uint256,uint256,uint256,address))"
    )]
    pub struct ComputeDeltaLXInCall {
        pub pool_id: ::ethers::core::types::U256,
        pub amount_in: ::ethers::core::types::U256,
        pub rx: ::ethers::core::types::U256,
        pub ry: ::ethers::core::types::U256,
        pub l: ::ethers::core::types::U256,
        pub params: LogNormalParams,
    }
    ///Container type for all input parameters for the `computeDeltaLYIn` function with signature `computeDeltaLYIn(uint256,uint256,uint256,uint256,uint256,(uint256,uint256,uint256,uint256,address))` and selector `0x9bdc031c`
    #[derive(
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
        name = "computeDeltaLYIn",
        abi = "computeDeltaLYIn(uint256,uint256,uint256,uint256,uint256,(uint256,uint256,uint256,uint256,address))"
    )]
    pub struct ComputeDeltaLYInCall {
        pub pool_id: ::ethers::core::types::U256,
        pub amount_in: ::ethers::core::types::U256,
        pub rx: ::ethers::core::types::U256,
        pub ry: ::ethers::core::types::U256,
        pub l: ::ethers::core::types::U256,
        pub params: LogNormalParams,
    }
    ///Container type for all input parameters for the `computeOptimalArbLowerPrice` function with signature `computeOptimalArbLowerPrice(uint256,uint256,uint256)` and selector `0x306db46b`
    #[derive(
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
        name = "computeOptimalArbLowerPrice",
        abi = "computeOptimalArbLowerPrice(uint256,uint256,uint256)"
    )]
    pub struct ComputeOptimalArbLowerPriceCall {
        pub pool_id: ::ethers::core::types::U256,
        pub s: ::ethers::core::types::U256,
        pub v_upper: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `computeOptimalArbRaisePrice` function with signature `computeOptimalArbRaisePrice(uint256,uint256,uint256)` and selector `0x4fd67c58`
    #[derive(
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
        name = "computeOptimalArbRaisePrice",
        abi = "computeOptimalArbRaisePrice(uint256,uint256,uint256)"
    )]
    pub struct ComputeOptimalArbRaisePriceCall {
        pub pool_id: ::ethers::core::types::U256,
        pub s: ::ethers::core::types::U256,
        pub v_upper: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `deallocateGivenX` function with signature `deallocateGivenX(uint256,uint256)` and selector `0x6237569f`
    #[derive(
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
    #[ethcall(name = "deallocateGivenX", abi = "deallocateGivenX(uint256,uint256)")]
    pub struct DeallocateGivenXCall {
        pub pool_id: ::ethers::core::types::U256,
        pub amount_x: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `deallocateGivenY` function with signature `deallocateGivenY(uint256,uint256)` and selector `0xf30d37f2`
    #[derive(
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
    #[ethcall(name = "deallocateGivenY", abi = "deallocateGivenY(uint256,uint256)")]
    pub struct DeallocateGivenYCall {
        pub pool_id: ::ethers::core::types::U256,
        pub amount_y: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `fetchPoolParams` function with signature `fetchPoolParams(uint256)` and selector `0x81b5fac2`
    #[derive(
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
    #[ethcall(name = "fetchPoolParams", abi = "fetchPoolParams(uint256)")]
    pub struct FetchPoolParamsCall {
        pub pool_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getDxGivenS` function with signature `getDxGivenS(uint256,uint256)` and selector `0x55f011c6`
    #[derive(
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
    #[ethcall(name = "getDxGivenS", abi = "getDxGivenS(uint256,uint256)")]
    pub struct GetDxGivenSCall {
        pub pool_id: ::ethers::core::types::U256,
        pub s: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getDyGivenS` function with signature `getDyGivenS(uint256,uint256)` and selector `0xe45d593c`
    #[derive(
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
    #[ethcall(name = "getDyGivenS", abi = "getDyGivenS(uint256,uint256)")]
    pub struct GetDyGivenSCall {
        pub pool_id: ::ethers::core::types::U256,
        pub s: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getInitialPoolData` function with signature `getInitialPoolData(uint256,uint256,(uint256,uint256,uint256,uint256,address))` and selector `0x134ead12`
    #[derive(
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
        name = "getInitialPoolData",
        abi = "getInitialPoolData(uint256,uint256,(uint256,uint256,uint256,uint256,address))"
    )]
    pub struct GetInitialPoolDataCall {
        pub rx: ::ethers::core::types::U256,
        pub s: ::ethers::core::types::U256,
        pub params: LogNormalParams,
    }
    ///Container type for all input parameters for the `getNextLiquidity` function with signature `getNextLiquidity(uint256,uint256,uint256,uint256)` and selector `0xaf4e437f`
    #[derive(
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
        name = "getNextLiquidity",
        abi = "getNextLiquidity(uint256,uint256,uint256,uint256)"
    )]
    pub struct GetNextLiquidityCall {
        pub pool_id: ::ethers::core::types::U256,
        pub rx: ::ethers::core::types::U256,
        pub ry: ::ethers::core::types::U256,
        pub l: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getNextReserveX` function with signature `getNextReserveX(uint256,uint256,uint256,uint256)` and selector `0x5eb408fc`
    #[derive(
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
        name = "getNextReserveX",
        abi = "getNextReserveX(uint256,uint256,uint256,uint256)"
    )]
    pub struct GetNextReserveXCall {
        pub pool_id: ::ethers::core::types::U256,
        pub ry: ::ethers::core::types::U256,
        pub l: ::ethers::core::types::U256,
        pub s: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getNextReserveY` function with signature `getNextReserveY(uint256,uint256,uint256,uint256)` and selector `0x120649c5`
    #[derive(
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
        name = "getNextReserveY",
        abi = "getNextReserveY(uint256,uint256,uint256,uint256)"
    )]
    pub struct GetNextReserveYCall {
        pub pool_id: ::ethers::core::types::U256,
        pub rx: ::ethers::core::types::U256,
        pub l: ::ethers::core::types::U256,
        pub s: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getPriceGivenXL` function with signature `getPriceGivenXL(uint256,uint256,uint256)` and selector `0x1e978cb0`
    #[derive(
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
        name = "getPriceGivenXL",
        abi = "getPriceGivenXL(uint256,uint256,uint256)"
    )]
    pub struct GetPriceGivenXLCall {
        pub pool_id: ::ethers::core::types::U256,
        pub rx: ::ethers::core::types::U256,
        pub l: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getPriceGivenYL` function with signature `getPriceGivenYL(uint256,uint256,uint256)` and selector `0x4e817fd9`
    #[derive(
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
        name = "getPriceGivenYL",
        abi = "getPriceGivenYL(uint256,uint256,uint256)"
    )]
    pub struct GetPriceGivenYLCall {
        pub pool_id: ::ethers::core::types::U256,
        pub ry: ::ethers::core::types::U256,
        pub l: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getReservesAndLiquidity` function with signature `getReservesAndLiquidity(uint256)` and selector `0xce153bf4`
    #[derive(
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
        name = "getReservesAndLiquidity",
        abi = "getReservesAndLiquidity(uint256)"
    )]
    pub struct GetReservesAndLiquidityCall {
        pub pool_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `internalPrice` function with signature `internalPrice(uint256)` and selector `0x3b4d1030`
    #[derive(
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
    #[ethcall(name = "internalPrice", abi = "internalPrice(uint256)")]
    pub struct InternalPriceCall {
        pub pool_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `prepareControllerUpdate` function with signature `prepareControllerUpdate(address)` and selector `0xcb1f5532`
    #[derive(
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
        name = "prepareControllerUpdate",
        abi = "prepareControllerUpdate(address)"
    )]
    pub struct PrepareControllerUpdateCall {
        pub controller: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `prepareFeeUpdate` function with signature `prepareFeeUpdate(uint256)` and selector `0xb09d04e5`
    #[derive(
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
    #[ethcall(name = "prepareFeeUpdate", abi = "prepareFeeUpdate(uint256)")]
    pub struct PrepareFeeUpdateCall {
        pub swap_fee: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `prepareSigmaUpdate` function with signature `prepareSigmaUpdate(uint256,uint256)` and selector `0xe94716d5`
    #[derive(
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
    #[ethcall(name = "prepareSigmaUpdate", abi = "prepareSigmaUpdate(uint256,uint256)")]
    pub struct PrepareSigmaUpdateCall {
        pub target_sigma: ::ethers::core::types::U256,
        pub target_timestamp: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `prepareStrikeUpdate` function with signature `prepareStrikeUpdate(uint256,uint256)` and selector `0x0420580a`
    #[derive(
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
        name = "prepareStrikeUpdate",
        abi = "prepareStrikeUpdate(uint256,uint256)"
    )]
    pub struct PrepareStrikeUpdateCall {
        pub target_strike: ::ethers::core::types::U256,
        pub target_timestamp: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `prepareTauUpdate` function with signature `prepareTauUpdate(uint256,uint256)` and selector `0x3b268d5d`
    #[derive(
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
    #[ethcall(name = "prepareTauUpdate", abi = "prepareTauUpdate(uint256,uint256)")]
    pub struct PrepareTauUpdateCall {
        pub target_tau: ::ethers::core::types::U256,
        pub target_timestamp: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `simulateSwap` function with signature `simulateSwap(uint256,bool,uint256)` and selector `0x3928ff97`
    #[derive(
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
    #[ethcall(name = "simulateSwap", abi = "simulateSwap(uint256,bool,uint256)")]
    pub struct SimulateSwapCall {
        pub pool_id: ::ethers::core::types::U256,
        pub swap_x_in: bool,
        pub amount_in: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `solverTradingFunction` function with signature `solverTradingFunction(uint256,uint256,uint256,(uint256,uint256,uint256,uint256,address))` and selector `0x24de2573`
    #[derive(
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
        name = "solverTradingFunction",
        abi = "solverTradingFunction(uint256,uint256,uint256,(uint256,uint256,uint256,uint256,address))"
    )]
    pub struct SolverTradingFunctionCall {
        pub rx: ::ethers::core::types::U256,
        pub ry: ::ethers::core::types::U256,
        pub l: ::ethers::core::types::U256,
        pub params: LogNormalParams,
    }
    ///Container type for all input parameters for the `strategy` function with signature `strategy()` and selector `0xa8c62e76`
    #[derive(
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
    #[ethcall(name = "strategy", abi = "strategy()")]
    pub struct StrategyCall;
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
    pub enum LogNormalSolverCalls {
        BisectionEpsilon(BisectionEpsilonCall),
        MaxBisectionIters(MaxBisectionItersCall),
        AllocateGivenX(AllocateGivenXCall),
        AllocateGivenY(AllocateGivenYCall),
        CalculateDiffLower(CalculateDiffLowerCall),
        CalculateDiffRaise(CalculateDiffRaiseCall),
        CallIerfc(CallIerfcCall),
        ComputeDeltaL(ComputeDeltaLCall),
        ComputeDeltaLXIn(ComputeDeltaLXInCall),
        ComputeDeltaLYIn(ComputeDeltaLYInCall),
        ComputeOptimalArbLowerPrice(ComputeOptimalArbLowerPriceCall),
        ComputeOptimalArbRaisePrice(ComputeOptimalArbRaisePriceCall),
        DeallocateGivenX(DeallocateGivenXCall),
        DeallocateGivenY(DeallocateGivenYCall),
        FetchPoolParams(FetchPoolParamsCall),
        GetDxGivenS(GetDxGivenSCall),
        GetDyGivenS(GetDyGivenSCall),
        GetInitialPoolData(GetInitialPoolDataCall),
        GetNextLiquidity(GetNextLiquidityCall),
        GetNextReserveX(GetNextReserveXCall),
        GetNextReserveY(GetNextReserveYCall),
        GetPriceGivenXL(GetPriceGivenXLCall),
        GetPriceGivenYL(GetPriceGivenYLCall),
        GetReservesAndLiquidity(GetReservesAndLiquidityCall),
        InternalPrice(InternalPriceCall),
        PrepareControllerUpdate(PrepareControllerUpdateCall),
        PrepareFeeUpdate(PrepareFeeUpdateCall),
        PrepareSigmaUpdate(PrepareSigmaUpdateCall),
        PrepareStrikeUpdate(PrepareStrikeUpdateCall),
        PrepareTauUpdate(PrepareTauUpdateCall),
        SimulateSwap(SimulateSwapCall),
        SolverTradingFunction(SolverTradingFunctionCall),
        Strategy(StrategyCall),
    }
    impl ::ethers::core::abi::AbiDecode for LogNormalSolverCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <BisectionEpsilonCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BisectionEpsilon(decoded));
            }
            if let Ok(decoded) = <MaxBisectionItersCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MaxBisectionIters(decoded));
            }
            if let Ok(decoded) = <AllocateGivenXCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AllocateGivenX(decoded));
            }
            if let Ok(decoded) = <AllocateGivenYCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AllocateGivenY(decoded));
            }
            if let Ok(decoded) = <CalculateDiffLowerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CalculateDiffLower(decoded));
            }
            if let Ok(decoded) = <CalculateDiffRaiseCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CalculateDiffRaise(decoded));
            }
            if let Ok(decoded) = <CallIerfcCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CallIerfc(decoded));
            }
            if let Ok(decoded) = <ComputeDeltaLCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ComputeDeltaL(decoded));
            }
            if let Ok(decoded) = <ComputeDeltaLXInCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ComputeDeltaLXIn(decoded));
            }
            if let Ok(decoded) = <ComputeDeltaLYInCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ComputeDeltaLYIn(decoded));
            }
            if let Ok(decoded) = <ComputeOptimalArbLowerPriceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ComputeOptimalArbLowerPrice(decoded));
            }
            if let Ok(decoded) = <ComputeOptimalArbRaisePriceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ComputeOptimalArbRaisePrice(decoded));
            }
            if let Ok(decoded) = <DeallocateGivenXCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DeallocateGivenX(decoded));
            }
            if let Ok(decoded) = <DeallocateGivenYCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DeallocateGivenY(decoded));
            }
            if let Ok(decoded) = <FetchPoolParamsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FetchPoolParams(decoded));
            }
            if let Ok(decoded) = <GetDxGivenSCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetDxGivenS(decoded));
            }
            if let Ok(decoded) = <GetDyGivenSCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetDyGivenS(decoded));
            }
            if let Ok(decoded) = <GetInitialPoolDataCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetInitialPoolData(decoded));
            }
            if let Ok(decoded) = <GetNextLiquidityCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetNextLiquidity(decoded));
            }
            if let Ok(decoded) = <GetNextReserveXCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetNextReserveX(decoded));
            }
            if let Ok(decoded) = <GetNextReserveYCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetNextReserveY(decoded));
            }
            if let Ok(decoded) = <GetPriceGivenXLCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetPriceGivenXL(decoded));
            }
            if let Ok(decoded) = <GetPriceGivenYLCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetPriceGivenYL(decoded));
            }
            if let Ok(decoded) = <GetReservesAndLiquidityCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetReservesAndLiquidity(decoded));
            }
            if let Ok(decoded) = <InternalPriceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InternalPrice(decoded));
            }
            if let Ok(decoded) = <PrepareControllerUpdateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PrepareControllerUpdate(decoded));
            }
            if let Ok(decoded) = <PrepareFeeUpdateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PrepareFeeUpdate(decoded));
            }
            if let Ok(decoded) = <PrepareSigmaUpdateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PrepareSigmaUpdate(decoded));
            }
            if let Ok(decoded) = <PrepareStrikeUpdateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PrepareStrikeUpdate(decoded));
            }
            if let Ok(decoded) = <PrepareTauUpdateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PrepareTauUpdate(decoded));
            }
            if let Ok(decoded) = <SimulateSwapCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SimulateSwap(decoded));
            }
            if let Ok(decoded) = <SolverTradingFunctionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SolverTradingFunction(decoded));
            }
            if let Ok(decoded) = <StrategyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Strategy(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for LogNormalSolverCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::BisectionEpsilon(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MaxBisectionIters(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AllocateGivenX(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AllocateGivenY(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CalculateDiffLower(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CalculateDiffRaise(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CallIerfc(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ComputeDeltaL(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ComputeDeltaLXIn(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ComputeDeltaLYIn(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ComputeOptimalArbLowerPrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ComputeOptimalArbRaisePrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DeallocateGivenX(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DeallocateGivenY(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FetchPoolParams(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetDxGivenS(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetDyGivenS(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetInitialPoolData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetNextLiquidity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetNextReserveX(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetNextReserveY(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetPriceGivenXL(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetPriceGivenYL(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetReservesAndLiquidity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InternalPrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PrepareControllerUpdate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PrepareFeeUpdate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PrepareSigmaUpdate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PrepareStrikeUpdate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PrepareTauUpdate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SimulateSwap(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SolverTradingFunction(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Strategy(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for LogNormalSolverCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::BisectionEpsilon(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaxBisectionIters(element) => ::core::fmt::Display::fmt(element, f),
                Self::AllocateGivenX(element) => ::core::fmt::Display::fmt(element, f),
                Self::AllocateGivenY(element) => ::core::fmt::Display::fmt(element, f),
                Self::CalculateDiffLower(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CalculateDiffRaise(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CallIerfc(element) => ::core::fmt::Display::fmt(element, f),
                Self::ComputeDeltaL(element) => ::core::fmt::Display::fmt(element, f),
                Self::ComputeDeltaLXIn(element) => ::core::fmt::Display::fmt(element, f),
                Self::ComputeDeltaLYIn(element) => ::core::fmt::Display::fmt(element, f),
                Self::ComputeOptimalArbLowerPrice(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ComputeOptimalArbRaisePrice(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DeallocateGivenX(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeallocateGivenY(element) => ::core::fmt::Display::fmt(element, f),
                Self::FetchPoolParams(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetDxGivenS(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetDyGivenS(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetInitialPoolData(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetNextLiquidity(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetNextReserveX(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetNextReserveY(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPriceGivenXL(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPriceGivenYL(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetReservesAndLiquidity(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InternalPrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::PrepareControllerUpdate(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PrepareFeeUpdate(element) => ::core::fmt::Display::fmt(element, f),
                Self::PrepareSigmaUpdate(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PrepareStrikeUpdate(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PrepareTauUpdate(element) => ::core::fmt::Display::fmt(element, f),
                Self::SimulateSwap(element) => ::core::fmt::Display::fmt(element, f),
                Self::SolverTradingFunction(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Strategy(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<BisectionEpsilonCall> for LogNormalSolverCalls {
        fn from(value: BisectionEpsilonCall) -> Self {
            Self::BisectionEpsilon(value)
        }
    }
    impl ::core::convert::From<MaxBisectionItersCall> for LogNormalSolverCalls {
        fn from(value: MaxBisectionItersCall) -> Self {
            Self::MaxBisectionIters(value)
        }
    }
    impl ::core::convert::From<AllocateGivenXCall> for LogNormalSolverCalls {
        fn from(value: AllocateGivenXCall) -> Self {
            Self::AllocateGivenX(value)
        }
    }
    impl ::core::convert::From<AllocateGivenYCall> for LogNormalSolverCalls {
        fn from(value: AllocateGivenYCall) -> Self {
            Self::AllocateGivenY(value)
        }
    }
    impl ::core::convert::From<CalculateDiffLowerCall> for LogNormalSolverCalls {
        fn from(value: CalculateDiffLowerCall) -> Self {
            Self::CalculateDiffLower(value)
        }
    }
    impl ::core::convert::From<CalculateDiffRaiseCall> for LogNormalSolverCalls {
        fn from(value: CalculateDiffRaiseCall) -> Self {
            Self::CalculateDiffRaise(value)
        }
    }
    impl ::core::convert::From<CallIerfcCall> for LogNormalSolverCalls {
        fn from(value: CallIerfcCall) -> Self {
            Self::CallIerfc(value)
        }
    }
    impl ::core::convert::From<ComputeDeltaLCall> for LogNormalSolverCalls {
        fn from(value: ComputeDeltaLCall) -> Self {
            Self::ComputeDeltaL(value)
        }
    }
    impl ::core::convert::From<ComputeDeltaLXInCall> for LogNormalSolverCalls {
        fn from(value: ComputeDeltaLXInCall) -> Self {
            Self::ComputeDeltaLXIn(value)
        }
    }
    impl ::core::convert::From<ComputeDeltaLYInCall> for LogNormalSolverCalls {
        fn from(value: ComputeDeltaLYInCall) -> Self {
            Self::ComputeDeltaLYIn(value)
        }
    }
    impl ::core::convert::From<ComputeOptimalArbLowerPriceCall>
    for LogNormalSolverCalls {
        fn from(value: ComputeOptimalArbLowerPriceCall) -> Self {
            Self::ComputeOptimalArbLowerPrice(value)
        }
    }
    impl ::core::convert::From<ComputeOptimalArbRaisePriceCall>
    for LogNormalSolverCalls {
        fn from(value: ComputeOptimalArbRaisePriceCall) -> Self {
            Self::ComputeOptimalArbRaisePrice(value)
        }
    }
    impl ::core::convert::From<DeallocateGivenXCall> for LogNormalSolverCalls {
        fn from(value: DeallocateGivenXCall) -> Self {
            Self::DeallocateGivenX(value)
        }
    }
    impl ::core::convert::From<DeallocateGivenYCall> for LogNormalSolverCalls {
        fn from(value: DeallocateGivenYCall) -> Self {
            Self::DeallocateGivenY(value)
        }
    }
    impl ::core::convert::From<FetchPoolParamsCall> for LogNormalSolverCalls {
        fn from(value: FetchPoolParamsCall) -> Self {
            Self::FetchPoolParams(value)
        }
    }
    impl ::core::convert::From<GetDxGivenSCall> for LogNormalSolverCalls {
        fn from(value: GetDxGivenSCall) -> Self {
            Self::GetDxGivenS(value)
        }
    }
    impl ::core::convert::From<GetDyGivenSCall> for LogNormalSolverCalls {
        fn from(value: GetDyGivenSCall) -> Self {
            Self::GetDyGivenS(value)
        }
    }
    impl ::core::convert::From<GetInitialPoolDataCall> for LogNormalSolverCalls {
        fn from(value: GetInitialPoolDataCall) -> Self {
            Self::GetInitialPoolData(value)
        }
    }
    impl ::core::convert::From<GetNextLiquidityCall> for LogNormalSolverCalls {
        fn from(value: GetNextLiquidityCall) -> Self {
            Self::GetNextLiquidity(value)
        }
    }
    impl ::core::convert::From<GetNextReserveXCall> for LogNormalSolverCalls {
        fn from(value: GetNextReserveXCall) -> Self {
            Self::GetNextReserveX(value)
        }
    }
    impl ::core::convert::From<GetNextReserveYCall> for LogNormalSolverCalls {
        fn from(value: GetNextReserveYCall) -> Self {
            Self::GetNextReserveY(value)
        }
    }
    impl ::core::convert::From<GetPriceGivenXLCall> for LogNormalSolverCalls {
        fn from(value: GetPriceGivenXLCall) -> Self {
            Self::GetPriceGivenXL(value)
        }
    }
    impl ::core::convert::From<GetPriceGivenYLCall> for LogNormalSolverCalls {
        fn from(value: GetPriceGivenYLCall) -> Self {
            Self::GetPriceGivenYL(value)
        }
    }
    impl ::core::convert::From<GetReservesAndLiquidityCall> for LogNormalSolverCalls {
        fn from(value: GetReservesAndLiquidityCall) -> Self {
            Self::GetReservesAndLiquidity(value)
        }
    }
    impl ::core::convert::From<InternalPriceCall> for LogNormalSolverCalls {
        fn from(value: InternalPriceCall) -> Self {
            Self::InternalPrice(value)
        }
    }
    impl ::core::convert::From<PrepareControllerUpdateCall> for LogNormalSolverCalls {
        fn from(value: PrepareControllerUpdateCall) -> Self {
            Self::PrepareControllerUpdate(value)
        }
    }
    impl ::core::convert::From<PrepareFeeUpdateCall> for LogNormalSolverCalls {
        fn from(value: PrepareFeeUpdateCall) -> Self {
            Self::PrepareFeeUpdate(value)
        }
    }
    impl ::core::convert::From<PrepareSigmaUpdateCall> for LogNormalSolverCalls {
        fn from(value: PrepareSigmaUpdateCall) -> Self {
            Self::PrepareSigmaUpdate(value)
        }
    }
    impl ::core::convert::From<PrepareStrikeUpdateCall> for LogNormalSolverCalls {
        fn from(value: PrepareStrikeUpdateCall) -> Self {
            Self::PrepareStrikeUpdate(value)
        }
    }
    impl ::core::convert::From<PrepareTauUpdateCall> for LogNormalSolverCalls {
        fn from(value: PrepareTauUpdateCall) -> Self {
            Self::PrepareTauUpdate(value)
        }
    }
    impl ::core::convert::From<SimulateSwapCall> for LogNormalSolverCalls {
        fn from(value: SimulateSwapCall) -> Self {
            Self::SimulateSwap(value)
        }
    }
    impl ::core::convert::From<SolverTradingFunctionCall> for LogNormalSolverCalls {
        fn from(value: SolverTradingFunctionCall) -> Self {
            Self::SolverTradingFunction(value)
        }
    }
    impl ::core::convert::From<StrategyCall> for LogNormalSolverCalls {
        fn from(value: StrategyCall) -> Self {
            Self::Strategy(value)
        }
    }
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
        Hash
    )]
    pub struct BisectionEpsilonReturn(pub ::ethers::core::types::U256);
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
        Hash
    )]
    pub struct MaxBisectionItersReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `allocateGivenX` function with signature `allocateGivenX(uint256,uint256)` and selector `0xee3e8cfb`
    #[derive(
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
    pub struct AllocateGivenXReturn(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `allocateGivenY` function with signature `allocateGivenY(uint256,uint256)` and selector `0x7f17409c`
    #[derive(
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
    pub struct AllocateGivenYReturn(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `calculateDiffLower` function with signature `calculateDiffLower(uint256,uint256,uint256)` and selector `0x332266f3`
    #[derive(
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
    pub struct CalculateDiffLowerReturn(pub ::ethers::core::types::I256);
    ///Container type for all return fields from the `calculateDiffRaise` function with signature `calculateDiffRaise(uint256,uint256,uint256)` and selector `0x902ecaa2`
    #[derive(
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
    pub struct CalculateDiffRaiseReturn(pub ::ethers::core::types::I256);
    ///Container type for all return fields from the `callIerfc` function with signature `callIerfc(int256)` and selector `0xc5e73169`
    #[derive(
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
    pub struct CallIerfcReturn(pub ::ethers::core::types::I256);
    ///Container type for all return fields from the `computeDeltaL` function with signature `computeDeltaL(uint256,uint256,uint256,uint256)` and selector `0x0730553c`
    #[derive(
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
    pub struct ComputeDeltaLReturn {
        pub delta_l: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `computeDeltaLXIn` function with signature `computeDeltaLXIn(uint256,uint256,uint256,uint256,uint256,(uint256,uint256,uint256,uint256,address))` and selector `0x9a6ec162`
    #[derive(
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
    pub struct ComputeDeltaLXInReturn {
        pub delta_l: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `computeDeltaLYIn` function with signature `computeDeltaLYIn(uint256,uint256,uint256,uint256,uint256,(uint256,uint256,uint256,uint256,address))` and selector `0x9bdc031c`
    #[derive(
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
    pub struct ComputeDeltaLYInReturn {
        pub delta_l: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `computeOptimalArbLowerPrice` function with signature `computeOptimalArbLowerPrice(uint256,uint256,uint256)` and selector `0x306db46b`
    #[derive(
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
    pub struct ComputeOptimalArbLowerPriceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `computeOptimalArbRaisePrice` function with signature `computeOptimalArbRaisePrice(uint256,uint256,uint256)` and selector `0x4fd67c58`
    #[derive(
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
    pub struct ComputeOptimalArbRaisePriceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `deallocateGivenX` function with signature `deallocateGivenX(uint256,uint256)` and selector `0x6237569f`
    #[derive(
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
    pub struct DeallocateGivenXReturn(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `deallocateGivenY` function with signature `deallocateGivenY(uint256,uint256)` and selector `0xf30d37f2`
    #[derive(
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
    pub struct DeallocateGivenYReturn(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `fetchPoolParams` function with signature `fetchPoolParams(uint256)` and selector `0x81b5fac2`
    #[derive(
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
    pub struct FetchPoolParamsReturn(pub LogNormalParams);
    ///Container type for all return fields from the `getDxGivenS` function with signature `getDxGivenS(uint256,uint256)` and selector `0x55f011c6`
    #[derive(
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
    pub struct GetDxGivenSReturn(pub ::ethers::core::types::I256);
    ///Container type for all return fields from the `getDyGivenS` function with signature `getDyGivenS(uint256,uint256)` and selector `0xe45d593c`
    #[derive(
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
    pub struct GetDyGivenSReturn(pub ::ethers::core::types::I256);
    ///Container type for all return fields from the `getInitialPoolData` function with signature `getInitialPoolData(uint256,uint256,(uint256,uint256,uint256,uint256,address))` and selector `0x134ead12`
    #[derive(
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
    pub struct GetInitialPoolDataReturn(pub ::ethers::core::types::Bytes);
    ///Container type for all return fields from the `getNextLiquidity` function with signature `getNextLiquidity(uint256,uint256,uint256,uint256)` and selector `0xaf4e437f`
    #[derive(
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
    ///Container type for all return fields from the `getNextReserveX` function with signature `getNextReserveX(uint256,uint256,uint256,uint256)` and selector `0x5eb408fc`
    #[derive(
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
    pub struct GetNextReserveXReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getNextReserveY` function with signature `getNextReserveY(uint256,uint256,uint256,uint256)` and selector `0x120649c5`
    #[derive(
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
    pub struct GetNextReserveYReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getPriceGivenXL` function with signature `getPriceGivenXL(uint256,uint256,uint256)` and selector `0x1e978cb0`
    #[derive(
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
    pub struct GetPriceGivenXLReturn {
        pub price: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `getPriceGivenYL` function with signature `getPriceGivenYL(uint256,uint256,uint256)` and selector `0x4e817fd9`
    #[derive(
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
    pub struct GetPriceGivenYLReturn {
        pub price: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `getReservesAndLiquidity` function with signature `getReservesAndLiquidity(uint256)` and selector `0xce153bf4`
    #[derive(
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
    pub struct GetReservesAndLiquidityReturn(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `internalPrice` function with signature `internalPrice(uint256)` and selector `0x3b4d1030`
    #[derive(
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
    pub struct InternalPriceReturn {
        pub price: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `prepareControllerUpdate` function with signature `prepareControllerUpdate(address)` and selector `0xcb1f5532`
    #[derive(
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
    pub struct PrepareControllerUpdateReturn(pub ::ethers::core::types::Bytes);
    ///Container type for all return fields from the `prepareFeeUpdate` function with signature `prepareFeeUpdate(uint256)` and selector `0xb09d04e5`
    #[derive(
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
    pub struct PrepareFeeUpdateReturn(pub ::ethers::core::types::Bytes);
    ///Container type for all return fields from the `prepareSigmaUpdate` function with signature `prepareSigmaUpdate(uint256,uint256)` and selector `0xe94716d5`
    #[derive(
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
    pub struct PrepareSigmaUpdateReturn(pub ::ethers::core::types::Bytes);
    ///Container type for all return fields from the `prepareStrikeUpdate` function with signature `prepareStrikeUpdate(uint256,uint256)` and selector `0x0420580a`
    #[derive(
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
    pub struct PrepareStrikeUpdateReturn(pub ::ethers::core::types::Bytes);
    ///Container type for all return fields from the `prepareTauUpdate` function with signature `prepareTauUpdate(uint256,uint256)` and selector `0x3b268d5d`
    #[derive(
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
    pub struct PrepareTauUpdateReturn(pub ::ethers::core::types::Bytes);
    ///Container type for all return fields from the `simulateSwap` function with signature `simulateSwap(uint256,bool,uint256)` and selector `0x3928ff97`
    #[derive(
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
    pub struct SimulateSwapReturn(
        pub bool,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::Bytes,
    );
    ///Container type for all return fields from the `solverTradingFunction` function with signature `solverTradingFunction(uint256,uint256,uint256,(uint256,uint256,uint256,uint256,address))` and selector `0x24de2573`
    #[derive(
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
    pub struct SolverTradingFunctionReturn(pub ::ethers::core::types::I256);
    ///Container type for all return fields from the `strategy` function with signature `strategy()` and selector `0xa8c62e76`
    #[derive(
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
    pub struct StrategyReturn(pub ::ethers::core::types::Address);
    ///`LogNormalParams(uint256,uint256,uint256,uint256,address)`
    #[derive(
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
    pub struct LogNormalParams {
        pub strike: ::ethers::core::types::U256,
        pub sigma: ::ethers::core::types::U256,
        pub tau: ::ethers::core::types::U256,
        pub swap_fee: ::ethers::core::types::U256,
        pub controller: ::ethers::core::types::Address,
    }
}
