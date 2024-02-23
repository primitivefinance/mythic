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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
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
    const __BYTECODE: &[u8] = b"`\x80\x80`@R4b\0\0\xECW`@Q`\x1Fb\0:p8\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x91`\x01`\x01`@\x1B\x03\x83\x11\x84\x84\x10\x17b\0\0\xD6W\x80\x84\x92` \x94`@R\x839\x81\x01\x03\x12b\0\0\x86WQ`\x01`\x01`\xA0\x1B\x03\x81\x16\x90\x81\x90\x03b\0\0\x81W`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x91\x90\x91\x17\x90U`@Qa96\x90\x81b\0\x01:\x829\xF3[`\0\x80\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[bF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01Ra7\xB7`\xF1\x1B`d\x82\x01R`\x84\x90\xFD\xFE`\x80`@R`\x046\x10a\x0B\xF6W`\x005`\xE0\x1C\x80c\x04 X\n\x14a\x01\xACW\x80c\x12\x06I\xC5\x14a\x01\xA7W\x80c\x13N\xAD\x12\x14a\x01\xA2W\x80c\x1E\x97\x8C\xB0\x14a\x01\x9DW\x80c0m\xB4k\x14a\x01\x98W\x80c3\"f\xF3\x14a\x01\x93W\x80c9(\xFF\x97\x14a\x01\x8EW\x80c;&\x8D]\x14a\x01\x89W\x80c;M\x100\x14a\x01\x84W\x80cN\x81\x7F\xD9\x14a\x01\x7FW\x80cO\xD6|X\x14a\x01zW\x80c^\xB4\x08\xFC\x14a\x01uW\x80cb7V\x9F\x14a\x01pW\x80cme\"\x99\x14a\x01kW\x80c\x7F\x17@\x9C\x14a\x01fW\x80c\x81\xB5\xFA\xC2\x14a\x01aW\x80c\x90.\xCA\xA2\x14a\x01\\W\x80c\xA8\xC6.v\x14a\x01WW\x80c\xAFNC\x7F\x14a\x01RW\x80c\xB0\x9D\x04\xE5\x14a\x01MW\x80c\xCB\x1FU2\x14a\x01HW\x80c\xCE\x15;\xF4\x14a\x01CW\x80c\xE9G\x16\xD5\x14a\x01>W\x80c\xEE>\x8C\xFB\x14a\x019W\x80c\xF3\r7\xF2\x14a\x014Wc\xF9\xC2\x82\x11\x03a\x0B\xF6Wa\x0B\xDAV[a\x0B\xAAV[a\x0ByV[a\x0B>V[a\x0B\x02V[a\n\xBDV[a\n\x8AV[a\nnV[a\nEV[a\n\x1CV[a\t\xEFV[a\tMV[a\t1V[a\x08\xC4V[a\x08\xA8V[a\x08\x7FV[a\x08cV[a\x084V[a\x07\xF9V[a\x05\x82V[a\x05+V[a\x04\xFCV[a\x04\xD7V[a\x04DV[a\x03(V[a\x02\xADV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01Ra7\xB7`\xF1\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[`\0[\x83\x81\x10a\x02dWPP`\0\x91\x01RV[\x81\x81\x01Q\x83\x82\x01R` \x01a\x02TV[\x90` \x91a\x02\x8D\x81Q\x80\x92\x81\x85R\x85\x80\x86\x01\x91\x01a\x02QV[`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[\x90` a\x02\xAA\x92\x81\x81R\x01\x90a\x02tV[\x90V[4a\x03\x05W`@6`\x03\x19\x01\x12a\x03\0Wa\x02\xFC`@Q`\x02` \x82\x01R`\x045`@\x82\x01R`$5``\x82\x01R``\x81Ra\x02\xE8\x81a\x03\xD4V[`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x02tV[\x03\x90\xF3[a\x02\x01V[a\x01\xB1V[`\x80\x90`\x03\x19\x01\x12a\x03\0W`\x045\x90`$5\x90`D5\x90`d5\x90V[4a\x03\x05W` a\x03Da\x03;6a\x03\nV[\x92\x91\x90\x91a\x0C\xDEV[`@Q\x90\x81R\xF3[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FABI decoding: struct data too sh`D\x82\x01Rb\x1B\xDC\x9D`\xEA\x1B`d\x82\x01R`\x84\x90\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\xA0\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x03\xCFW`@RV[a\x03\x9DV[`\x80\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x03\xCFW`@RV[``\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x03\xCFW`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x03\xCFW`@RV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\x04?WV[`\0\x80\xFD[4a\x03\x05W`\xE06`\x03\x19\x01\x12a\x03\0W`\xA06`C\x19\x01\x12a\x04\xB8Wa\x02\xFCa\x04\xAC`@Qa\x04s\x81a\x03\xB3V[`D5\x81R`d5` \x82\x01R`\x845`@\x82\x01R`\xA45``\x82\x01R`\xC45a\x04\x9C\x81a\x04.V[`\x80\x82\x01R`$5`\x045a\x16\x8FV[`@Q\x91\x82\x91\x82a\x02\x99V[a\x03LV[``\x90`\x03\x19\x01\x12a\x03\0W`\x045\x90`$5\x90`D5\x90V[4a\x03\x05W` a\x03Da\x04\xF6a\x04\xED6a\x04\xBDV[\x91\x92\x90\x92a\x12\x05V[\x91a\x18xV[4a\x03\x05W` a\x03Da\x05\x0F6a\x04\xBDV[\x90a\x05\"a\x05\x1C\x84a\x12\x05V[\x93a\x13\x8CV[\x92\x91\x90\x91a\x19eV[4a\x03\x05W` a\x03Da\x05>6a\x04\xBDV[\x90a\x05Ka\x05\x1C\x84a\x12\x05V[\x92\x90Pa\x1C\xBBV[\x80\x15\x15\x03a\x04?WV[\x90\x92`\x80\x92a\x02\xAA\x95\x94\x15\x15\x83R` \x83\x01R`@\x82\x01R\x81``\x82\x01R\x01\x90a\x02tV[4a\x03\x05W``6`\x03\x19\x01\x12a\x03\0W`\x045`$5a\x05\xA2\x81a\x05SV[`D5\x91a\x05\xAEa\r\xD5V[a\x05\xB6a\r\xD5V[\x92a\x05\xC0\x83a\x13\x8CV[\x93\x91\x92\x90\x96` \x83\x01\x93`@\x98\x89\x85\x01\x96\x87R\x85R\x83Ra\x05\xF2a\x05\xE3\x87a\x12\x05V[\x95\x84Q\x90\x86Q\x90Q\x91\x89a\x12\xB3V[\x91\x15a\x07xWa\x06j\x93\x92a\x06B\x92a\x06\x1Da\x06$\x93a\x06\x16``\x8A\x01Q\x82a(\x13V[\x93Qa\x0E.V[\x89Ra\x0E.V[a\x066\x88\x88\x01\x91\x80\x83R\x88Q\x88a\r\xC8V[\x90\x87Q\x90Q\x90\x87a\x0C\xDEV[\x90a\x06aa\x06V` \x88\x01\x93\x80\x85Ra\x0E\x1BV[\x80\x84R\x82Q\x11a\x0E\xAFV[Q\x90Q\x90a\x0E\xA2V[\x93[\x83Q\x91` \x85\x01Q\x92a\x06\xAE\x83\x87\x01\x91a\x06\xA0\x83Q\x86Q\x97\x88\x93` \x85\x01`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[\x03`\x1F\x19\x81\x01\x86R\x85a\x04\x0CV[`\0Ta\x06\xD1\x90a\x06\xC5\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x80;\x15a\x07sW\x84`\xC0\x91a\x06\xFC\x97\x86Q\x80\x99\x81\x94\x82\x93c\r\x17\xA7\xC7`\xE3\x1B\x84R0`\x04\x85\x01a\x0F=V[\x03\x91Z\xFA\x94\x85\x15a\x07nW`\0\x95a\x07.W[P\x90a\x07#\x91a\x02\xFC\x95\x96Q\x90Q\x90a\x18xV[\x90Q\x94\x85\x94\x85a\x05]V[a\x02\xFC\x95P\x90a\x07Ya\x07#\x93\x92`\xC0=`\xC0\x11a\x07gW[a\x07Q\x81\x83a\x04\x0CV[\x81\x01\x90a\x0F\x06V[PPPPP\x95P\x90\x91a\x07\x0FV[P=a\x07GV[a\x0C\xD2V[a\x0CYV[a\x07\xD7a\x07\xEA\x92a\x07\xBAa\x07\xF3\x96a\x07\xADa\x07\xDF\x95a\x07\xA6a\x07\x9E``\x8D\x01Q\x83a(\x13V[\x8CQ\x90a(iV[\x92Qa\x0E.V[\x92` \x8C\x01\x93\x84Ra\x0E.V[a\x07\xCC\x8B\x8B\x01\x91\x80\x83R\x83Q\x8Ba\x0FaV[\x91Q\x90Q\x90\x89a\x0FnV[\x80\x88Ra\x0E\x1BV[\x80\x87R\x82Q\x11a\x0E;V[Q\x84Q\x90a\x0E\xA2V[\x93a\x06lV[4a\x03\x05W`@6`\x03\x19\x01\x12a\x03\0Wa\x02\xFC`@Q`\x04` \x82\x01R`\x045`@\x82\x01R`$5``\x82\x01R``\x81Ra\x02\xE8\x81a\x03\xD4V[4a\x03\x05W` 6`\x03\x19\x01\x12a\x03\0W` a\x03D`\x045a\x04\xF6a\x08Y\x82a\x13\x8CV[\x92\x91\x93\x90Pa\x12\x05V[4a\x03\x05W` a\x03Da\x08ya\x04\xED6a\x04\xBDV[\x91a\x1EMV[4a\x03\x05W` a\x03Da\x08\x926a\x04\xBDV[\x90a\x08\x9Fa\x05\x1C\x84a\x12\x05V[\x92\x91\x90\x91a\x1E\xC7V[4a\x03\x05W` a\x03Da\x08\xBB6a\x03\nV[\x92\x91\x90\x91a\x0FnV[4a\x03\x05W`@6`\x03\x19\x01\x12a\x03\0Wa\t\x13`\x045a\x02\xFCa\x08\xF5a\x08\xEA\x83a\x13\x8CV[\x91\x90P`$5a\" V[\x93\x90\x92\x84\x84a\t\ra\t\x06\x84a\x12\x05V[\x83\x83a\x18xV[\x92a\x0C\xDEV[\x92`@Q\x93\x84\x93\x84`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[4a\x03\x05W`\x006`\x03\x19\x01\x12a\x03\0W` `@Q`\0\x81R\xF3[4a\x03\x05W`@6`\x03\x19\x01\x12a\x03\0W`\x045a\t\x9Ba\x02\xFCa\t}a\ts\x84a\x13\x8CV[\x91P`$5a\"MV[\x92\x90\x93\x83\x85a\t\x95a\t\x8E\x84a\x12\x05V[\x83\x83a\x1EMV[\x92a\x0FnV[\x91`@Q\x93\x84\x93\x84`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[\x80Q\x82R` \x80\x82\x01Q\x90\x83\x01R`@\x80\x82\x01Q\x90\x83\x01R``\x80\x82\x01Q\x90\x83\x01R`\x80\x90\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x91\x01RV[4a\x03\x05W` 6`\x03\x19\x01\x12a\x03\0W`\xA0a\n\r`\x045a\x12\x05V[a\n\x1A`@Q\x80\x92a\t\xB9V[\xF3[4a\x03\x05W` a\x03Da\n/6a\x04\xBDV[\x90a\n<a\x05\x1C\x84a\x12\x05V[\x92\x90\x91Pa\"tV[4a\x03\x05W`\x006`\x03\x19\x01\x12a\x03\0W`\0T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x03\x05W` a\x03Da\n\x816a\x03\nV[\x92\x91\x90\x91a\x12\xB3V[4a\x03\x05W` 6`\x03\x19\x01\x12a\x03\0Wa\x02\xFC`@Q`\x01` \x82\x01R`\x045`@\x82\x01R`@\x81Ra\x02\xE8\x81a\x03\xF0V[4a\x03\x05W` 6`\x03\x19\x01\x12a\x03\0Wa\x02\xFC`\x045a\n\xDD\x81a\x04.V[`@\x80Q`\x05` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x82\x82\x01R\x81Ra\x02\xE8\x81a\x03\xF0V[4a\x03\x05W` 6`\x03\x19\x01\x12a\x03\0Wa\x02\xFCa\x0B!`\x045a\x13\x8CV[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R\x90\x81\x90``\x82\x01\x90V[4a\x03\x05W`@6`\x03\x19\x01\x12a\x03\0Wa\x02\xFC`@Q`\x03` \x82\x01R`\x045`@\x82\x01R`$5``\x82\x01R``\x81Ra\x02\xE8\x81a\x03\xD4V[4a\x03\x05W`@6`\x03\x19\x01\x12a\x03\0Wa\t\x13`\x045a\x02\xFCa\x08\xF5a\x0B\x9F\x83a\x13\x8CV[\x91\x90P`$5a\"MV[4a\x03\x05W`@6`\x03\x19\x01\x12a\x03\0W`\x045a\t\x9Ba\x02\xFCa\t}a\x0B\xD0\x84a\x13\x8CV[\x91P`$5a\" V[4a\x03\x05W`\x006`\x03\x19\x01\x12a\x03\0W` `@Q`x\x81R\xF3[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01Rtnor receive functions`X\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01Rd code`\xD8\x1B`d\x82\x01R`\x84\x90\xFD[\x90\x81` \x91\x03\x12a\x03\0WQ\x90V[`@\x90a\x02\xAA\x93\x92\x81R\x81` \x82\x01R\x01\x90a\x02tV[`@Q=`\0\x82>=\x90\xFD[\x90\x91\x92a\x0C\xF5\x90a\x0C\xEE\x83a\x12\x05V[\x90\x85a\x14\x85V[\x90`@Q\x93a\r,\x85a\r\x1E\x83\x86\x88` \x85\x01`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[\x03`\x1F\x19\x81\x01\x87R\x86a\x04\x0CV[`\0Ta\rC\x90a\x06\xC5\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x80;\x15a\x07sWa\rl\x95` \x91`@Q\x80\x80\x99\x81\x94b.RK`\xE0\x1B\x83R\x88`\x04\x84\x01a\x0C\xBBV[\x03\x91Z\xFA\x91\x82\x15a\x07nWa\x02\xAA\x95`\0\x93a\r\x93W[Pa\r\x8D\x90a\x12\x05V[\x93a\x14\xD8V[a\r\x8D\x91\x93Pa\r\xBA\x90` =` \x11a\r\xC1W[a\r\xB2\x81\x83a\x04\x0CV[\x81\x01\x90a\x0C\xACV[\x92\x90a\r\x83V[P=a\r\xA8V[\x91a\x04\xF6a\x02\xAA\x93a\x12\x05V[`@Q\x90``\x82\x01\x82\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x03\xCFW`@R`\0`@\x83\x82\x81R\x82` \x82\x01R\x01RV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x90`\x01\x82\x01\x80\x92\x11a\x0E)WV[a\x0E\x05V[\x91\x90\x82\x01\x80\x92\x11a\x0E)WV[\x15a\x0EBWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: x reserve increase`D\x82\x01Rad!`\xF0\x1B`d\x82\x01R`\x84\x90\xFD[a\x03\xE7\x19\x81\x01\x91\x90\x82\x11a\x0E)WV[\x91\x90\x82\x03\x91\x82\x11a\x0E)WV[\x15a\x0E\xB6WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: y reserve increase`D\x82\x01Rad!`\xF0\x1B`d\x82\x01R`\x84\x90\xFD[\x91\x90\x82`\xC0\x91\x03\x12a\x03\0W\x81Qa\x0F\x1D\x81a\x05SV[\x91` \x81\x01Q\x91`@\x82\x01Q\x91``\x81\x01Q\x91`\xA0`\x80\x83\x01Q\x92\x01Q\x90V[a\x02\xAA\x93\x92``\x92`\x01\x80`\xA0\x1B\x03\x16\x82R` \x82\x01R\x81`@\x82\x01R\x01\x90a\x02tV[\x91a\x08ya\x02\xAA\x93a\x12\x05V[\x90\x91\x92a\x0F\x85\x90a\x0F~\x83a\x12\x05V[\x90\x85a \x9DV[\x90`@Q\x93a\x0F\xAE\x85a\r\x1E\x83\x87\x87` \x85\x01`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[`\0Ta\x0F\xC5\x90a\x06\xC5\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x80;\x15a\x07sWa\x0F\xEE\x95` \x91`@Q\x80\x80\x99\x81\x94b.RK`\xE0\x1B\x83R\x88`\x04\x84\x01a\x0C\xBBV[\x03\x91Z\xFA\x91\x82\x15a\x07nWa\x02\xAA\x95`\0\x93a\x10\x15W[Pa\x10\x0F\x90a\x12\x05V[\x93a \xCAV[a\x10\x0F\x91\x93Pa\x103\x90` =` \x11a\r\xC1Wa\r\xB2\x81\x83a\x04\x0CV[\x92\x90a\x10\x05V[\x91\x90` \x92\x83\x81\x83\x03\x12a\x03\0W\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x82\x82\x11a\x11RW\x01\x82`\x1F\x82\x01\x12\x15a\x10\xF9W\x80Q\x91\x82\x11a\x03\xCFW`@Q\x92a\x10\x87`\x1F\x84\x01`\x1F\x19\x16\x87\x01\x85a\x04\x0CV[\x82\x84R\x85\x83\x83\x01\x01\x11a\x10\xA4W\x84\x83\x94\x95a\x02\xAA\x94\x01\x91\x01a\x02QV[`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x86\x90R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x86\x90R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x87\x90R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[\x91\x90\x82`\xA0\x91\x03\x12a\x04\xB8W`@Qa\x11\xBA\x81a\x03\xB3V[`\x80\x80\x82\x94\x80Q\x84R` \x81\x01Q` \x85\x01R`@\x81\x01Q`@\x85\x01R``\x81\x01Q``\x85\x01R\x01Q\x91a\x11\xED\x83a\x04.V[\x01RV[\x90`\xA0\x82\x82\x03\x12a\x03\0Wa\x02\xAA\x91a\x11\xA2V[`\x80\x90`@Qa\x12\x14\x81a\x03\xB3V[`\0\x92\x81\x84\x80\x93R\x82` \x82\x01R\x82`@\x82\x01R\x82``\x82\x01R\x01Ra\x12Fa\x06\xC5a\x06\xC5\x84T`\x01\x80`\xA0\x1B\x03\x16\x90V[\x80;\x15a\x07sW`@Qc\xDC\x17\x83U`\xE0\x1B\x81R`\x04\x81\x01\x92\x90\x92R\x82\x90\x82\x90`$\x90\x82\x90Z\xFA\x90\x81\x15a\x07nW\x82a\x02\xAA\x93\x92a\x12\x90W[PP` \x80\x82Q\x83\x01\x01\x91\x01a\x11\xF1V[a\x12\xAC\x92P=\x80\x91\x83>a\x12\xA4\x81\x83a\x04\x0CV[\x81\x01\x90a\x10:V[8\x80a\x12\x7FV[`@\x80Q` \x81\x01\x84\x90R\x90\x81\x01\x84\x90R``\x81\x01\x85\x90R\x93\x91\x92a\x12\xDB\x85`\x80\x81\x01a\r\x1EV[`\0T`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x07sWa\x13\x10\x95` \x91`@Q\x80\x80\x99\x81\x94b.RK`\xE0\x1B\x83R\x88`\x04\x84\x01a\x0C\xBBV[\x03\x91Z\xFA\x91\x82\x15a\x07nWa\x02\xAA\x95`\0\x93a\x137W[Pa\x131\x90a\x12\x05V[\x93a$%V[a\x131\x91\x93Pa\x13U\x90` =` \x11a\r\xC1Wa\r\xB2\x81\x83a\x04\x0CV[\x92\x90a\x13'V[\x90\x81` \x91\x03\x12a\x03\0WQa\x02\xAA\x81a\x04.V[\x90\x81``\x91\x03\x12a\x03\0W\x80Q\x91`@` \x83\x01Q\x92\x01Q\x90V[`\0T\x90\x91\x90a\x13\xA6\x90a\x06\xC5\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x80;\x15a\x07sW` `\x04\x91`@Q\x92\x83\x80\x92c+\xEE\x84\xF1`\xE2\x1B\x82RZ\xFA\x90\x81\x15a\x07nW`\0\x91a\x14VW[P`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x07sW`@Qc3\x85N\xFD`\xE2\x1B\x81R`\x04\x81\x01\x93\x90\x93R``\x90\x83\x90`$\x90\x82\x90Z\xFA\x90\x81\x15a\x07nW`\0\x80\x93`\0\x93a\x14\x1FW[P\x92\x91\x90V[\x91\x93PPa\x14E\x91P``=``\x11a\x14OW[a\x14=\x81\x83a\x04\x0CV[\x81\x01\x90a\x13qV[\x92\x90\x92\x918a\x14\x19V[P=a\x143V[a\x14x\x91P` =` \x11a\x14~W[a\x14p\x81\x83a\x04\x0CV[\x81\x01\x90a\x13\\V[8a\x13\xD4V[P=a\x14fV[a\x14\xAE\x90a\x14\xA7a\x14\xA2a\x14\x9D\x86a\x02\xAA\x97\x96a%{V[a%\xF0V[a&#V[\x92Qa(\x13V[a(\x13V[a\x14\xD6\x93``\x92\x96\x95\x93a\x01\0\x83\x01\x97\x83R` \x83\x01R`@\x82\x01R\x01\x90a\t\xB9V[V[\x92\x93`\0\x93\x85\x92\x91\x85\x85\x12\x15a\x16\\W[\x85\x85\x12a\x16=W\x90a\x15\x0Ba\x15\x19\x92[`@\x96`@Q\x95\x86\x94` \x86\x01a\x14\xB3V[\x03`\x1F\x19\x81\x01\x83R\x82a\x04\x0CV[\x81\x85\x92\x85\x96\x82\x81\x11a\x16\x1AWa\x15/\x81\x85a5\xF4V[\x92a\x15:\x81\x86a5\xF4V[\x88a\x15E\x82\x87a\x18$V[\x13a\x15\xF9WP\x90a\x15Y\x91\x97\x96\x92\x97a\x0E\xA2V[`\x01\x95\x91\x82\x91\x87\x80[a\x15tW[PPPPPPPPPP\x90V[\x15a\x15\xD5W[P\x86\x97\x98P\x81\x92a\x15\x94a\x15\x8E\x8B\x89a\x0E.V[`\x01\x1C\x90V[\x99a\x15\x9F\x8B\x88a5\xF4V[\x90\x84a\x15\xAB\x88\x84a\x18$V[\x13a\x15\xC9WPP\x89\x93[\x88a\x15\xC0\x89\x87a\x0E\xA2V[\x92\x01\x94\x99a\x15bV[\x8B\x98P\x90\x95P\x93a\x15\xB5V[`\x14\x10\x80a\x15\xF0W[\x15a\x15\xE9W\x88a\x15zV[\x80\x80a\x15gV[P\x80\x83\x10a\x15\xDEV[`@Qc\x06\xF1\xBE]`\xE2\x1B\x81R`\x04\x81\x01\x86\x90R`$\x81\x01\x91\x90\x91R`D\x90\xFD[`@Qc0\x82\xDF\xDB`\xE1\x1B\x81R`\x04\x81\x01\x91\x90\x91R`$\x81\x01\x92\x90\x92RP`D\x90\xFD[\x93P\x91a\x16I\x90a(?V[\x91a\x16V\x84\x83\x85\x84a'\x1AV[\x93a\x14\xE9V[\x85\x85\x13a\x16pW\x90a\x15\x0Ba\x15\x19\x92a\x14\xF9V[\x93P\x94a\x16|\x90a&fV[\x94a\x16\x89\x84\x83\x88\x84a'\x1AV[\x93a\x16\\V[\x91a\x16\xA0a\x14\xA2a\x14\x9D\x83\x85a1\xB9V[\x91g\r\xE0\xB6\xB3\xA7d\0\0\x92\x83\x03\x92\x83\x11a\x0E)Wa\x16\xF7\x82a\x16\xE3a\x16\xD8a\x14\xA2a\x14\x9D\x84a\x16\xD2a\x17\x15\x9A\x8Ca(iV[\x97a%{V[a\x14\xAE\x85\x84Qa(\x13V[\x92a\x16\xF0\x82\x82\x86\x8Aa'\x1AV[\x84\x88a$%V[\x90`@Q\x94` \x86\x01R`@\x85\x01R``\x84\x01R`\x80\x83\x01\x90a\t\xB9V[a\x01\0\x81Ra\x01 \x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x03\xCFW`@R\x90V[\x90g\r\xE0\xB6\xB3\xA7d\0\0`\0\x83\x82\x03\x93\x12\x81\x84\x12\x81\x16\x91\x84\x13\x90\x15\x16\x17a\x0E)WV[\x90g\x1B\xC1mgN\xC8\0\0`\0\x83\x82\x03\x93\x12\x81\x84\x12\x81\x16\x91\x84\x13\x90\x15\x16\x17a\x0E)WV[\x90g\x11\x90\0\xAB\x10\x0F\xFB\xFF\x19\x82\x01\x91\x82\x13`\x01\x16a\x0E)WV[\x81\x81\x03\x92\x91`\0\x13\x80\x15\x82\x85\x13\x16\x91\x84\x12\x16\x17a\x0E)WV[\x90g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x81\x02\x92\x81\x84\x05\x14\x90\x15\x17\x15a\x0E)WV[\x90g\x1B\xC1mgN\xC8\0\0\x91\x82\x81\x02\x92\x81\x84\x05\x14\x90\x15\x17\x15a\x0E)WV[\x90c;\x9A\xCA\0\x91\x82\x81\x02\x92\x81\x84\x05\x14\x90\x15\x17\x15a\x0E)WV[`\0\x81\x90\x03\x91\x90`\x01`\xFF\x1B\x81\x14`\x01\x16a\x0E)W`\0\x19\x83\x05\x03a\x0E)WV[\x81\x81\x02\x92\x91`\0\x82\x12`\x01`\xFF\x1B\x82\x14\x16a\x0E)W\x81\x84\x05\x14\x90\x15\x17\x15a\x0E)WV[\x81\x15a\x18bW`\x01`\xFF\x1B\x81\x14`\0\x19\x83\x14\x16a\x0E)W\x05\x90V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[\x91\x90\x91a\x18\xB3` \x83\x01\x93a\x18\xAD\x85Qa\x18\xA5a\x18\x9B`@\x88\x01\x92\x83Q\x90a*\xB4V[\x97Q\x82Q\x90a*\xDDV[\x90Q\x90a&\x85V[\x92a&\xA6V[\x92g\r\xE0\xB6\xB3\xA7d\0\0\x80\x85\x12\x15a\x191W`\0\x85\x13\x15a\x19&W`\0\x85\x82\x03\x95\x12\x81\x86\x12\x81\x16\x91\x86\x13\x90\x15\x16\x17a\x0E)Wa\x19\x1Aa\x19\x1F\x92a\x19\x15a\x19\x07a\x14\xA2\x94a\x19\x02a\x02\xAA\x99a*\xFCV[a\x18$V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[a\x17\x97V[a+\x88V[\x90Qa(\x13V[PPPPP`\0\x19\x90V[PPPPP`\0\x90V[\x90\x95\x94\x92\x93a\x14\xD6\x94`\x80\x93a\x01 \x84\x01\x98\x84R` \x84\x01R`@\x83\x01R``\x82\x01R\x01\x90a\t\xB9V[\x90\x92\x91\x85Q` \x87\x01Q`@\x88\x01Q``\x89\x01Q\x91a\x19\x83\x82a-9V[a\x19\x8C\x90a\x1B\x1DV[\x92a\x19\x96\x90a\x178V[a\x19\x9F\x89a\x1CDV[a\x19\xA8\x90a-\xF7V[a\x19\xB1\x89a\x1CVV[a\x19\xBA\x83a.UV[a\x19\xC3\x91a\x17\x97V[a\x19\xCC\x91a/*V[a\x19\xD5\x90a/HV[\x92a\x19\xE0\x83\x80a.\xADV[\x90a\x19\xEA\x91a.\xADV[a\x19\xF3\x90a.\xDCV[a\x19\xFC\x90a\x1C\xAAV[\x83\x85a\x1A\x07\x85a.~V[\x90a\x1A\x11\x91a.\xADV[\x90a\x1A\x1B\x91a.\xADV[a\x1A$\x91a\x1C\x8EV[a\x1A-\x90a+\x88V[a\x1A6\x82a\x1CtV[a\x1A@\x90\x8Ba.\xADV[a\x1AJ\x90\x8Aa\x1C\x8EV[a\x1AT\x90\x87a.\xADV[a\x1A]\x91a.\xADV[a\x1Af\x89a\x1CVV[a\x1Ao\x83a.UV[a\x1Ax\x91a\x17\x97V[a\x1A\x81\x91a/*V[\x94a\x1A\x8B\x90a.&V[\x90a\x1A\x95\x90a\x1CtV[a\x1A\x9E\x91a.\xADV[\x92a\x1A\xA8\x91a.\xADV[a\x1A\xB1\x90a/\x03V[\x90a\x1A\xBB\x91a\x17\x97V[a\x1A\xC4\x90a0\x99V[a\x1A\xCD\x91a.\xADV[\x90a\x1A\xD7\x84a\x1C\xAAV[\x90a\x1A\xE1\x91a\x1C\x8EV[\x90a\x1A\xEB\x91a\x1C\x8EV[`\0\x13a\x1B\x12Wa\x02\xAA\x95a\x1B\r\x93a\x15\x0B\x92`@Q\x96\x87\x95` \x87\x01a\x19;V[a(\xBEV[PPPPPP`\0\x90V[\x90c;\x9A\xCA\0\x91\x82\x81\x02\x92\x81\x84\x04\x14\x90\x15\x17\x15a\x0E)WV[\x90\x81g \x05\xFEO&\x8E\xA0\0\x01\x91\x82\x12`\x01\x16a\x0E)WV[\x90\x81g\r\xC5R\x7Fd, \0\x01\x91\x82\x12`\x01\x16a\x0E)WV[\x90\x81g\r\xE0\xB6\xB3\xA7d\0\0\x01\x91\x82\x12`\x01\x16a\x0E)WV[\x90\x81g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19\x01\x91\x82\x12\x15`\x01\x16a\x0E)WV[\x90\x81g\x14\xA8EL\x19\xE1\xAC\0\x01\x91\x82\x12`\x01\x16a\x0E)WV[\x90\x81g\x0F\xC1\x0E\x01W\x82w\xFF\x19\x01\x91\x82\x12\x15`\x01\x16a\x0E)WV[\x90\x81g\x03\xDE\xBD\x08;\x8C|\0\x01\x91\x82\x12`\x01\x16a\x0E)WV[\x90\x81g\x02\x95\xD4\0\xEA2W\xFF\x19\x01\x91\x82\x12\x15`\x01\x16a\x0E)WV[\x90\x81g\x01W\xD8\xB2\xEC\xC7\x08\0\x01\x91\x82\x12`\x01\x16a\x0E)WV[\x90\x81g\x051\n\xA7\xD5!0\0\x01\x91\x82\x12`\x01\x16a\x0E)WV[\x90\x81g\r\xE0\xCC=\x15a\0\0\x01\x91\x82\x12`\x01\x16a\x0E)WV[\x90\x81a\x03\xE8\x01\x91\x82\x12`\x01\x16a\x0E)WV[\x90a\x03\xE8\x91`\0\x83\x82\x01\x93\x84\x12\x91\x12\x90\x80\x15\x82\x16\x91\x15\x16\x17a\x0E)WV[\x90\x81g\r\xE0\xB6\xB3\xA7c\xFF\xFF\x19\x01\x91\x82\x12\x15`\x01\x16a\x0E)WV[\x91\x90\x91`\0\x83\x82\x01\x93\x84\x12\x91\x12\x90\x80\x15\x82\x16\x91\x15\x16\x17a\x0E)WV[`\x01`\xFF\x1B\x81\x14a\x0E)W`\0\x03\x90V[\x93\x90\x91\x92\x81Q\x93` \x83\x01Q\x91`@\x84\x01Q\x93``\x01Q\x94\x81a\x1C\xDD\x86a-9V[a\x1C\xE6\x90a\x1B\x1DV[\x96a\x1C\xF0\x90a\x178V[\x93\x82a\x1C\xFD\x86\x94\x84a\x1C\x8EV[a\x1D\x06\x90a-\xF7V[a\x1D\x10\x84\x84a\x1C\x8EV[a\x1D\x1A\x86\x86a.\xADV[a\x1D#\x91a\x17\x97V[a\x1D,\x91a/*V[a\x1D5\x90a/HV[\x97a\x1D@\x88\x80a.\xADV[\x90a\x1DJ\x91a.\xADV[a\x1DS\x90a.\xDCV[a\x1D\\\x90a\x1C\xAAV[\x88\x8Aa\x1Dg\x8Aa.~V[\x90a\x1Dq\x91a.\xADV[\x90a\x1D{\x91a.\xADV[a\x1D\x84\x91a\x1C\x8EV[a\x1D\x8D\x90a+\x88V[\x90a\x1D\x97\x85a\x1CtV[a\x1D\xA0\x91a.\xADV[a\x1D\xAA\x90\x83a\x1C\x8EV[a\x1D\xB4\x90\x8Ba.\xADV[a\x1D\xBD\x91a.\xADV[\x93a\x1D\xC7\x91a\x1C\x8EV[\x91a\x1D\xD1\x91a.\xADV[a\x1D\xDA\x91a\x17\x97V[a\x1D\xE3\x91a/*V[\x94a\x1D\xED\x90a.&V[\x90a\x1D\xF7\x90a\x1CtV[a\x1E\0\x91a.\xADV[\x92a\x1E\n\x91a.\xADV[a\x1E\x13\x90a/\x03V[\x90a\x1E\x1D\x91a\x17\x97V[a\x1E&\x90a0\x99V[a\x1E/\x91a.\xADV[\x91a\x1E9\x90a\x1C\xAAV[\x90a\x1EC\x91a\x1C\x8EV[\x90a\x02\xAA\x91a\x1C\x8EV[\x91\x90\x91a\x1E\x8B` \x83\x01\x91a\x1E\x85a\x1E}\x84Qa\x18\xA5a\x1Es`@\x89\x01\x92\x83Q\x90a*\xB4V[\x96Q\x82Q\x90a*\xDDV[\x95\x85Qa&\x85V[\x90a&\xA6V[g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x82\x12\x15a\x191W`\0\x82\x13\x15a\x19&Wa\x02\xAA\x94a\x19\x1F\x93a\x1E\xC1a\x19\x1A\x93a\x19\x02a\x14\xA2\x96a*\xFCV[\x05a\x1C\x8EV[\x90\x92\x91\x85Q` \x87\x01Q\x90`@\x88\x01Q``\x89\x01Q\x90a\x1E\xE6\x81a-9V[a\x1E\xEF\x90a\x1B\x1DV[\x91a\x1E\xF9\x90a\x178V[\x93a\x1F\x03\x86a\x1CDV[a\x1F\x0C\x90a-\xF7V[a\x1F\x16\x89\x86a.\xADV[a\x1F\x1F\x90a\x1CVV[a\x1F(\x87a.UV[a\x1F1\x91a\x17\x97V[a\x1F:\x91a/*V[a\x1FC\x90a/HV[\x91a\x1FN\x82\x80a.\xADV[\x90a\x1FX\x91a.\xADV[a\x1Fa\x90a.\xDCV[a\x1Fj\x90a\x1C\xAAV[\x82\x84a\x1Fu\x84a.~V[\x90a\x1F\x7F\x91a.\xADV[\x90a\x1F\x89\x91a.\xADV[a\x1F\x92\x91a\x1C\x8EV[a\x1F\x9B\x90a+\x88V[a\x1F\xA5\x89\x86a.\xADV[a\x1F\xAE\x87a\x1CtV[a\x1F\xB8\x90\x89a.\xADV[a\x1F\xC1\x91a\x1C\x8EV[a\x1F\xCB\x90\x89a.\xADV[a\x1F\xD4\x91a.\xADV[a\x1F\xDE\x89\x86a.\xADV[a\x1F\xE7\x90a\x1CVV[a\x1F\xF0\x87a.UV[a\x1F\xF9\x91a\x17\x97V[a \x03\x90\x86a.\xADV[a \x0C\x91a/*V[\x94a \x16\x90a\x1CtV[a  \x90\x88a.\xADV[\x92a *\x91a.\xADV[a 3\x90a/\x03V[\x90a =\x91a\x17\x97V[a F\x90a0\x99V[a O\x91a.\xADV[\x90a Y\x90a-\xF7V[a b\x91a/*V[\x90a l\x90a\x1CtV[\x90a v\x91a\x1C\x8EV[`\0\x13a\x1B\x12Wa\x02\xAA\x95a \x98\x93a\x15\x0B\x92`@Q\x96\x87\x95` \x87\x01a\x19;V[a)\xDDV[\x91a\x14\x9Da\x14\xA2\x91a \xAE\x93a1\xB9V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x03\x90\x81\x11a\x0E)Wa\x02\xAA\x91a(\x13V[\x92\x93`\0\x93\x85\x92\x91\x85\x85\x12\x15a!\xEDW[\x85\x85\x12a!\xCEW\x90a\x15\x0Ba \xFC\x92`@\x96`@Q\x95\x86\x94` \x86\x01a\x14\xB3V[\x81\x85\x92\x85\x96\x82\x81\x11a\x16\x1AWa!\x12\x81\x85a6\x15V[\x92a!\x1D\x81\x86a6\x15V[\x88a!(\x82\x87a\x18$V[\x13a\x15\xF9WP\x90a!<\x91\x97\x96\x92\x97a\x0E\xA2V[`\x01\x95\x91\x82\x91\x87\x80[a!VWPPPPPPPPPP\x90V[\x15a!\xB1W[P\x86\x97\x98P\x81\x92a!pa\x15\x8E\x8B\x89a\x0E.V[\x99a!{\x8B\x88a6\x15V[\x90\x84a!\x87\x88\x84a\x18$V[\x13a!\xA5WPP\x89\x93[\x88a!\x9C\x89\x87a\x0E\xA2V[\x92\x01\x94\x99a!EV[\x8B\x98P\x90\x95P\x93a!\x91V[`\x14\x10\x80a!\xC5W[\x15a\x15\xE9W\x88a!\\V[P\x80\x83\x10a!\xBAV[\x93P\x91a!\xDA\x90a(?V[\x91a!\xE7\x84\x83\x83\x86a'\x1AV[\x93a \xDBV[\x85\x85\x13a\"\x01W\x90a\x15\x0Ba \xFC\x92a\x14\xF9V[\x93P\x94a\"\r\x90a&fV[\x94a\"\x1A\x84\x83\x83\x89a'\x1AV[\x93a!\xEDV[\x92\x91\x90a\"6a\"0\x82\x84a(iV[\x85a(\x13V[\x93\x81\x03\x90\x81\x11a\x0E)W\x92\x81\x03\x90\x81\x11a\x0E)W\x90V[\x92\x91\x90a\"]a\"0\x82\x84a(iV[\x93\x81\x01\x80\x91\x11a\x0E)W\x92\x81\x01\x80\x91\x11a\x0E)W\x90V[\x92\x93\x90\x91\x81Q\x93` \x83\x01Q\x91`@\x84\x01Q\x93``\x01Q\x94\x87a\"\x96\x86a-9V[a\"\x9F\x90a\x1B\x1DV[\x96a\"\xA9\x90a\x178V[\x98\x82a\"\xB6\x8B\x94\x83a\x1C\x8EV[a\"\xBF\x90a-\xF7V[\x82a\"\xCA\x87\x8Da.\xADV[\x90a\"\xD4\x91a\x1C\x8EV[a\"\xDE\x86\x85a.\xADV[a\"\xE7\x91a\x17\x97V[a\"\xF0\x91a/*V[a\"\xF9\x90a/HV[\x97a#\x04\x88\x80a.\xADV[\x90a#\x0E\x91a.\xADV[a#\x17\x90a.\xDCV[a# \x90a\x1C\xAAV[\x88\x8Aa#+\x8Aa.~V[\x90a#5\x91a.\xADV[\x90a#?\x91a.\xADV[a#H\x91a\x1C\x8EV[a#Q\x90a+\x88V[\x90a#\\\x86\x8Ca.\xADV[\x90a#f\x86a\x1CtV[a#o\x91a.\xADV[a#x\x91a\x1C\x8EV[a#\x82\x90\x87a.\xADV[a#\x8B\x91a.\xADV[\x93a#\x96\x90\x8Aa.\xADV[\x90a#\xA0\x91a\x1C\x8EV[\x91a#\xAA\x91a.\xADV[a#\xB3\x91a\x17\x97V[a#\xBD\x90\x87a.\xADV[a#\xC6\x91a/*V[\x95a#\xD0\x90a\x1CtV[a#\xD9\x91a.\xADV[\x92a#\xE3\x91a.\xADV[a#\xEC\x90a/\x03V[\x90a#\xF6\x91a\x17\x97V[a#\xFF\x90a0\x99V[a$\x08\x91a.\xADV[\x90a$\x12\x90a-\xF7V[a$\x1B\x91a/*V[\x90a\x1EC\x90a\x1CtV[\x92\x93`\0\x93\x85\x92\x91\x85\x85\x12\x15a%HW[\x85\x85\x12a%)W\x90a\x15\x0Ba$W\x92`@\x96`@Q\x95\x86\x94` \x86\x01a\x14\xB3V[\x81\x85\x92\x85\x96\x82\x81\x11a\x16\x1AWa$m\x81\x85a67V[\x92a$x\x81\x86a67V[\x88a$\x83\x82\x87a\x18$V[\x13a\x15\xF9WP\x90a$\x97\x91\x97\x96\x92\x97a\x0E\xA2V[`\x01\x95\x91\x82\x91\x87\x80[a$\xB1WPPPPPPPPPP\x90V[\x15a%\x0CW[P\x86\x97\x98P\x81\x92a$\xCBa\x15\x8E\x8B\x89a\x0E.V[\x99a$\xD6\x8B\x88a67V[\x90\x84a$\xE2\x88\x84a\x18$V[\x13a%\0WPP\x89\x93[\x88a$\xF7\x89\x87a\x0E\xA2V[\x92\x01\x94\x99a$\xA0V[\x8B\x98P\x90\x95P\x93a$\xECV[`\x14\x10\x80a% W[\x15a\x15\xE9W\x88a$\xB7V[P\x80\x83\x10a%\x15V[\x93P\x94a%5\x90a&fV[\x94a%B\x84\x87\x84\x84a'\x1AV[\x93a$6V[\x85\x85\x13a%\\W\x90a\x15\x0Ba$W\x92a\x14\xF9V[\x93P\x91a%h\x90a(?V[\x91a%u\x84\x84\x84\x84a'\x1AV[\x93a%HV[a%\xEBa%\xE6a\x02\xAA\x93a%\xE0a%\xDB\x82Qa%\xD6g\r\xE0\xB6\xB3\xA7d\0\0a%\xD0a%\xCBa%\xC5a%\xC0`@` \x8B\x01Q\x9A\x01Q\x96a%\xBA\x88\x8Ca*\xB4V[\x9Da(iV[a20V[\x97a20V[a\x17\xCDV[\x05a+\x88V[a&\x85V[a&\xC8V[\x90a\x17\x97V[a\x17\xB0V[a\x18GV[a&\x1Fa%\xE6a&\x1Ag\x13\xA0K\xBD\xFD\xC9\xBE\x88a&\x14g\x1B\xC1mgN\xC8\0\0\x95a\x17\xB0V[\x05a\x1C\xAAV[a0\x99V[\x05\x90V[`\0\x81\x12a&.W\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01RotoUint: negative`\x80\x1B`D\x82\x01R`d\x90\xFD[a\x03\xE7\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x04?Wa\x03\xE8\x90\x04\x90V[\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x04?Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x04\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17\x82\x15\x15\x16\x15a\x04?W\x04\x90V[g\x06\xF0[Y\xD3\xB2\0\0\x90\x80\x82\x02\x91\x82\x04\x14`\x01\x16\x15a\x04?Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x04\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x04?Wg\x1B\xC1mgN\xC8\0\0\x90\x04\x90V[\x90\x92\x82\x82\x10\x15a'\xCEWa\x02\xAA\x93a'\x97\x92\x84g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82a'B\x83\x83a&\xA6V[\x10a'\xBBWP`\x01`\x01`\xFF\x1B\x03\x95\x90P[\x83Q\x91a'ja'd\x83\x85a&\x85V[\x85a&\xA6V[\x10a'\x9CWP`\x01`\x01`\xFF\x1B\x03\x92a'\x91\x92P\x90P[`@` \x82\x01Q\x91\x01Q\x90a*\xB4V[\x92a\x1C\x8EV[a\x1C\x8EV[a'\x91\x92a\x1E\x85a'\xB0\x92a'\xB5\x94a&\x85V[a*\xFCV[\x91a'\x81V[a'\xC8\x91a'\xB0\x91a&\xA6V[\x94a'TV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FtradingFunction: invalid x\0\0\0\0\0\0`D\x82\x01R`d\x90\xFD[\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x04?W`\x01g\r\xE0\xB6\xB3\xA7d\0\0`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[a\x03\xE9\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x04?W`\x01a\x03\xE8`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[\x90g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x81\x02\x92\x81\x84\x04\x14\x90\x15\x17\x81\x15\x15\x16\x15a\x04?W`\x01\x90`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[a\x01\0\x81\x83\x03\x12a\x03\0W\x80Q\x92a\x02\xAA` \x83\x01Q\x93```@\x85\x01Q\x94\x01a\x11\xA2V[\x91\x90a\x03\xE8\x92`\0\x93`\0\x91\x83\x82\x11a)\xBCWa(\xDA\x81a6WV[a(\xE4\x85\x83a7zV[`\0a(\xF0\x82\x84a\x18$V[\x13a)\x9DWPa)\x01\x85\x96\x95a\x0E\x92V[`\x01\x94`\0\x91\x86\x80[a)\x1BW[PPPPPPPP\x90PV[\x15a)xW[P\x85\x96\x97\x98P\x80\x91a)6a\x15\x8E\x8B\x88a\x0E.V[\x99a)A\x8B\x87a7zV[\x90\x83a)M\x87\x84a\x18$V[\x13a)lWPP\x89\x92[\x87a)b\x88\x86a\x0E\xA2V[\x92\x01\x93\x99\x98a)\nV[\x8B\x97P\x90\x94P\x92a)WV[\x86\x10\x80a)\x92W[\x15a)\x8BW\x88a)!V[\x80\x80a)\x0FV[Pa\x01\0\x82\x10a)\x80V[`@Qc\x06\xF1\xBE]`\xE2\x1B\x81R`\x04\x81\x01\x92\x90\x92R`$\x82\x01R`D\x90\xFD[`@Qc0\x82\xDF\xDB`\xE1\x1B\x81Ra\x03\xE8`\x04\x82\x01R`$\x81\x01\x85\x90R`D\x90\xFD[\x91\x90a\x03\xE8\x92`\0\x93`\0\x91\x83\x82\x11a)\xBCWa)\xF9\x81a7\x9CV[a*\x03\x85\x83a8\xDFV[`\0a*\x0F\x82\x84a\x18$V[\x13a)\x9DWPa* \x85\x96\x95a\x0E\x92V[`\x01\x94`\0\x91\x86\x80[a*9WPPPPPPPP\x90PV[\x15a*\x96W[P\x85\x96\x97\x98P\x80\x91a*Ta\x15\x8E\x8B\x88a\x0E.V[\x99a*_\x8B\x87a8\xDFV[\x90\x83a*k\x87\x84a\x18$V[\x13a*\x8AWPP\x89\x92[\x87a*\x80\x88\x86a\x0E\xA2V[\x92\x01\x93\x99\x98a*)V[\x8B\x97P\x90\x94P\x92a*uV[\x86\x10\x80a*\xA9W[\x15a)\x8BW\x88a*?V[Pa\x01\0\x82\x10a*\x9EV[\x90a*\xBE\x90a-9V[c;\x9A\xCA\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17\x15a\x0E)Wa\x02\xAA\x91a&\x85V[a\x02\xAA\x91a%\xD6g\r\xE0\xB6\xB3\xA7d\0\0a%\xD0a%\xCBa%\xDB\x95a20V[g\x06\xF0[Y\xD3\xB2\0\0\x81\x14a+\x82Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x12\x15a+pW\x80\x15a+^W\x80`\x01\x1B\x90\x81\x05`\x02\x03a\x0E)Wa+:\x90a/HV[\x90g\x13\xA0K\xBD\xFD\xC9\xBE\x88\x91\x80\x83\x02\x92\x83\x05\x14`\x01\x16\x15a\x04?Wa\x02\xAA\x91\x05a\x1C\xAAV[`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x90\xFD[`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x90\xFD[P`\0\x90V[h\x02H\xCE6\xA7\x0C\xB2k>\x19\x81\x13\x15a+\x82Wh\x07U\xBFy\x8BJ\x1B\xF1\xE5\x81\x12\x15a,\xD8We\x03x-\xAC\xE9\xD9\x90`N\x1B\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x91``\x90`\x01`_\x1B\x84\x82\x84\x1B\x05\x01\x82\x1D\x93\x84\x02\x90\x03\x80l\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x81\x01\x02\x82\x1D\x90n\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dPn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x82n\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x81m\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x81m\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x81m\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x81l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x02\x8D\x1D\x01\x02\x8B\x1D\x01\x02\x89\x1D\x01\x02\x87\x1D\x01\x02\x85\x1D\x01\x93m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x93m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEA\x81\x01\x90\x84m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x91\x01\x01\x02\x90\x1D\x01\x02\x01\x05\x02\x90`\xC3\x03\x1C\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x90\xFD[\x91\x90a\x01 \x83\x82\x03\x12a\x03\0W\x82Q\x92` \x81\x01Q\x92a\x02\xAA`@\x83\x01Q\x93`\x80``\x85\x01Q\x94\x01a\x11\xA2V[`\xB5\x81`\x01`\x88\x1B\x81\x10\x15a-\xE0W[\x80i\x01\0\0\0\0\0\0\0\0\0b\x01\0\0\x92\x10\x15a-\xD3W[e\x01\0\0\0\0\0\x81\x10\x15a-\xC6W[c\x01\0\0\0\x81\x10\x15a-\xB9W[\x01\x02`\x12\x1C`\x01\x90\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x80\x80\x92\x04\x10\x90\x03\x90V[`\x10\x1C\x91`\x08\x1B\x91a-}V[` \x1C\x91`\x10\x1B\x91a-pV[`@\x1C\x91` \x1B\x91a-aV[Ph\xB5\0\0\0\0\0\0\0\0\x90P`\x80\x82\x90\x1Ca-IV[g\x1B\xC1mgN\xC8\0\0\x81\x81\x02\x91`\x01`\xFF\x1B\x81\x13`\x01\x17\x91\x83\x05\x14\x16\x15a\x04?Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\x06\xF0[Y\xD3\xB2\0\0\x81\x81\x02\x91`\x01`\xFF\x1B\x81\x13`\x01\x17\x91\x83\x05\x14\x16\x15a\x04?Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[a\x03\xE8\x81\x81\x02\x91`\x01`\xFF\x1B\x81\x13`\x01\x17\x91\x83\x05\x14\x16\x15a\x04?Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\x13\xA0K\xBD\xE7\x8C\xC4\0\x81\x81\x02\x91`\x01`\xFF\x1B\x81\x13`\x01\x17\x91\x83\x05\x14\x16\x15a\x04?Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[\x81\x81\x02\x91\x81\x15\x82\x84\x05\x82\x14\x17`\0\x19\x90\x92\x10`\x01`\xFF\x1B\x90\x91\x13\x17\x16\x15a\x04?Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x82\x05\x14`\x01\x16\x15a\x04?Wg\x1B\xC1mgN\xC8\0\0\x90\x05\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x82\x05\x14`\x01\x16\x15a\x04?Wg\x13\xA0K\xBD\xE7\x8C\xC4\0\x90\x05\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x82\x05\x14\x82\x15\x15\x16\x15a\x04?W\x05\x90V[`\0\x81\x12\x80\x15a0\x88W[a0vW\x80\x15a+pWg\x1B\xC1mgN\xC8\0\0\x81\x14a+^Wg\r\xE0\xB6\xB3\xA7d\0\0\x81\x12\x90\x81\x15a0gW\x90[a/\x89\x82a3\xFEV[\x80\x15a+pWa/\xF2a/\xB6a/\xB1a/\xACa/\xA7a/\xF7\x95a20V[a4\xBFV[a-9V[a\x17\xEAV[a\x19\x15a/\xCAa/\xC5\x83a4)V[a\x1B6V[a/\xECa/\xE7a/\xE1a/\xDC\x86a4TV[a\x1BNV[\x85a56V[a\x1BfV[\x90a4\x9DV[a4\xE7V[\x91`\0\x90[`\x02\x82\x10a0\x17WPP\x15a0\x0EW\x90V[a\x02\xAA\x90a\x1C\xAAV[\x90\x92a0_\x81a0Ya0/\x85a\x19\x15`\x01\x96a0\x99V[a/\xECa0Oa0Ja\x19\x1Aa0E\x87\x80a56V[a\x1C\xAAV[a5\x0FV[a%\xE0\x83\x86a56V[\x90a\x1C\x8EV[\x93\x01\x90a/\xFCV[a0p\x90a\x17[V[\x90a/\x80V[`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x90\xFD[Pg\x1B\xC1mgN\xC8\0\0\x81\x13a/SV[\x80\x15a1\xACWgV\x98\xEE\xF0fp\0\0\x81\x12\x15a+\x82WgV\x98\xEE\xF0fo\xFF\xFF\x19\x81\x13\x15a1\x9FW`\0a1\x8Fa0\xCE\x83a3\xD1V[a1Wa\x19\x1Aa0\xE8a0\xE3a/\xE7\x85a&\xEFV[a4~V[\x92a'\x97a1\x8Aa1\x85a1~a1xa1sa1ma1ha1ba1]\x8Da1Wa1Ra1La1Ga/\xE1a1Ba1<a17a11a1,\x8Aa5WV[a\x1B~V[\x89a56V[a\x1B\x98V[\x87a56V[a\x1B\xB0V[a\x1B\xCAV[\x83a56V[a\x1B\xE2V[\x90a56V[a\x1B\xFCV[\x8Ca56V[a\x1C\x14V[\x8Aa56V[a\x1C,V[\x88a56V[\x93\x80a56V[a\x18\x03V[a\x17~V[\x91\x12\x15a\x02\xAAWa\x02\xAA\x90a\x17[V[Pg\x1B\xC1mgN\xC8\0\0\x90V[Pg\r\xE0\xB6\xB3\xA7d\0\0\x90V[a%\xEBa%\xE6a\x02\xAA\x93a0Ya%\xDB\x82Qa%\xD6g\r\xE0\xB6\xB3\xA7d\0\0a%\xD0a%\xCBa%\xC5a%\xC0`@` \x8B\x01Q\x9A\x01Q\x96a%\xBA\x88\x8Ca*\xB4V[\x15a1\xFFWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x90\xFD[}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x90a2\\`\0\x82\x13a1\xF8V[q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06a2x\x82a5\x82V[``\x92\x83\x82`\x9F\x03\x01\x1B`\x9F\x1C\x90`_\x19\x01}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x02\x92l\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x82m\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x81m\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x81m\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x81m\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x81m\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x81lFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x02\x8C\x1D\x01\x02\x8A\x1D\x01\x02\x88\x1D\x01\x02\x86\x1D\x01\x02\x84\x1D\x01\x02\x82\x1D\x01\x91x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x91l\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x90\x82m\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x81l\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x81m\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x81m\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x81l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x81\x01\x02\x89\x1D\x01\x02\x87\x1D\x01\x02\x85\x1D\x01\x02\x83\x1D\x01\x02\x90\x1D\x01\x02\x01\x05\x02\x01\x01`\xAE\x1D\x90V[`\x01`\xFF\x1B\x81\x14a3\xECW`\0\x81\x12\x15a\x02\xAAW\x19`\x01\x01\x90V[`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x90\xFD[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x04?Wg\x1B\xC1mgN\xC8\0\0\x90\x05\x90V[g\x03\xC1f\\z\xAB \0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x04?Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[f\x9F2u$b\xA0\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x04?Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[`\x01\x81\x15\x15\x16\x15a\x04?Wn\xC0\x97\xCE{\xC9\x07\x15\xB3K\x9F\x10\0\0\0\0\x05\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17\x82\x15\x15\x16\x15a\x04?W\x05\x90V[g\x1B\xC1mgN\xC7\xFF\xFF\x19\x81\x81\x02\x91`\x01\x91\x83\x05\x14\x16\x15a\x04?Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\t\xD0(\xCCo _\xFF\x19\x81\x81\x02\x91`\x01\x91\x83\x05\x14\x16\x15a\x04?Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x90\x80\x82\x02\x91\x82\x05\x14`\x01\x16\x15a\x04?Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x04?Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\x02_\x0F\xE1\x05\xA3\x14\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x04?Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[a5\x8D\x81\x15\x15a1\xF8V[\x80o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x07\x1B\x81\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x81\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x81\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x81\x81\x1C`\xFF\x10`\x03\x1B\x17\x81\x81\x1C`\x0F\x10`\x02\x1B\x17\x81\x81\x1C`\x03\x10`\x01\x1B\x17\x90\x81\x1C`\x01\x10\x17\x90V[\x90a6\x0Ba\x02\xAA\x92` \x80\x82Q\x83\x01\x01\x91\x01a(\x99V[\x93\x91\x92\x90Pa'\x1AV[\x90a6,a\x02\xAA\x92` \x80\x82Q\x83\x01\x01\x91\x01a(\x99V[\x93\x90P\x91\x90\x91a'\x1AV[\x90a6Na\x02\xAA\x92` \x80\x82Q\x83\x01\x01\x91\x01a(\x99V[\x93\x92\x90Pa'\x1AV[\x80Q\x81\x01` \x01\x90` \x01\x90a6l\x91a-\x0CV[\x80\x91\x92PQ\x92` \x82\x01Q\x90`@\x83\x01Q\x92``\x01Q\x93a6\x8C\x84a-9V[a6\x95\x90a\x1B\x1DV[\x94a6\x9F\x90a\x178V[\x91a6\xA9\x81a\x1CDV[a6\xB2\x90a-\xF7V[a6\xBB\x83a\x1CVV[a6\xC4\x85a.UV[a6\xCD\x91a\x17\x97V[a6\xD6\x91a/*V[a6\xDF\x90a/HV[\x94a6\xEA\x85\x80a.\xADV[\x90a6\xF4\x91a.\xADV[a6\xFD\x90a.\xDCV[a7\x06\x90a\x1C\xAAV[\x85\x87a7\x11\x87a.~V[\x90a7\x1B\x91a.\xADV[\x90a7%\x91a.\xADV[a7.\x91a\x1C\x8EV[a77\x90a+\x88V[\x90a7A\x84a\x1CtV[a7J\x91a.\xADV[a7T\x90\x83a\x1C\x8EV[a7^\x90\x88a.\xADV[a7g\x91a.\xADV[\x90a7q\x90a\x1CVV[a\x1D\xD1\x83a.UV[\x90a7\x91a\x02\xAA\x92` \x80\x82Q\x83\x01\x01\x91\x01a-\x0CV[\x94\x93\x90\x92\x91Pa\x1C\xBBV[\x80Q\x81\x01` \x01\x90` \x01\x90a7\xB1\x91a-\x0CV[\x80\x91\x92\x94\x93PQ\x92` \x82\x01Q\x90`@\x83\x01Q\x92``\x01Q\x93a7\xD3\x84a-9V[a7\xDC\x90a\x1B\x1DV[\x94a7\xE6\x90a\x178V[\x96a7\xF0\x81a\x1CDV[a7\xF9\x90a-\xF7V[a8\x03\x83\x89a.\xADV[a8\x0C\x90a\x1CVV[a8\x15\x8Aa.UV[a8\x1E\x91a\x17\x97V[a8'\x91a/*V[a80\x90a/HV[\x94a8;\x85\x80a.\xADV[\x90a8E\x91a.\xADV[a8N\x90a.\xDCV[a8W\x90a\x1C\xAAV[\x85\x87a8b\x87a.~V[\x90a8l\x91a.\xADV[\x90a8v\x91a.\xADV[a8\x7F\x91a\x1C\x8EV[a8\x88\x90a+\x88V[\x90a8\x93\x83\x89a.\xADV[\x90a8\x9D\x8Aa\x1CtV[a8\xA6\x91a.\xADV[a8\xAF\x91a\x1C\x8EV[a8\xB9\x90\x84a.\xADV[a8\xC2\x91a.\xADV[\x90a8\xCD\x90\x87a.\xADV[a8\xD6\x90a\x1CVV[a#\xAA\x88a.UV[\x90a8\xF6a\x02\xAA\x92` \x80\x82Q\x83\x01\x01\x91\x01a-\x0CV[\x94\x93\x90\x92Pa\"tV\xFE\xA2dipfsX\"\x12 \xB5\x86C\xEE\xF7?&\xF5+\xBE$<\xCF\xA1\x07w\x17\xC6\x15/\xE7\xC9\xA07\xFE\xC4\xAE#s~\x04\xD8dsolcC\0\x08\x16\x003";
    /// The bytecode of the contract.
    pub static LOGNORMALSOLVER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\x0B\xF6W`\x005`\xE0\x1C\x80c\x04 X\n\x14a\x01\xACW\x80c\x12\x06I\xC5\x14a\x01\xA7W\x80c\x13N\xAD\x12\x14a\x01\xA2W\x80c\x1E\x97\x8C\xB0\x14a\x01\x9DW\x80c0m\xB4k\x14a\x01\x98W\x80c3\"f\xF3\x14a\x01\x93W\x80c9(\xFF\x97\x14a\x01\x8EW\x80c;&\x8D]\x14a\x01\x89W\x80c;M\x100\x14a\x01\x84W\x80cN\x81\x7F\xD9\x14a\x01\x7FW\x80cO\xD6|X\x14a\x01zW\x80c^\xB4\x08\xFC\x14a\x01uW\x80cb7V\x9F\x14a\x01pW\x80cme\"\x99\x14a\x01kW\x80c\x7F\x17@\x9C\x14a\x01fW\x80c\x81\xB5\xFA\xC2\x14a\x01aW\x80c\x90.\xCA\xA2\x14a\x01\\W\x80c\xA8\xC6.v\x14a\x01WW\x80c\xAFNC\x7F\x14a\x01RW\x80c\xB0\x9D\x04\xE5\x14a\x01MW\x80c\xCB\x1FU2\x14a\x01HW\x80c\xCE\x15;\xF4\x14a\x01CW\x80c\xE9G\x16\xD5\x14a\x01>W\x80c\xEE>\x8C\xFB\x14a\x019W\x80c\xF3\r7\xF2\x14a\x014Wc\xF9\xC2\x82\x11\x03a\x0B\xF6Wa\x0B\xDAV[a\x0B\xAAV[a\x0ByV[a\x0B>V[a\x0B\x02V[a\n\xBDV[a\n\x8AV[a\nnV[a\nEV[a\n\x1CV[a\t\xEFV[a\tMV[a\t1V[a\x08\xC4V[a\x08\xA8V[a\x08\x7FV[a\x08cV[a\x084V[a\x07\xF9V[a\x05\x82V[a\x05+V[a\x04\xFCV[a\x04\xD7V[a\x04DV[a\x03(V[a\x02\xADV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01Ra7\xB7`\xF1\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[`\0[\x83\x81\x10a\x02dWPP`\0\x91\x01RV[\x81\x81\x01Q\x83\x82\x01R` \x01a\x02TV[\x90` \x91a\x02\x8D\x81Q\x80\x92\x81\x85R\x85\x80\x86\x01\x91\x01a\x02QV[`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[\x90` a\x02\xAA\x92\x81\x81R\x01\x90a\x02tV[\x90V[4a\x03\x05W`@6`\x03\x19\x01\x12a\x03\0Wa\x02\xFC`@Q`\x02` \x82\x01R`\x045`@\x82\x01R`$5``\x82\x01R``\x81Ra\x02\xE8\x81a\x03\xD4V[`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x02tV[\x03\x90\xF3[a\x02\x01V[a\x01\xB1V[`\x80\x90`\x03\x19\x01\x12a\x03\0W`\x045\x90`$5\x90`D5\x90`d5\x90V[4a\x03\x05W` a\x03Da\x03;6a\x03\nV[\x92\x91\x90\x91a\x0C\xDEV[`@Q\x90\x81R\xF3[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FABI decoding: struct data too sh`D\x82\x01Rb\x1B\xDC\x9D`\xEA\x1B`d\x82\x01R`\x84\x90\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\xA0\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x03\xCFW`@RV[a\x03\x9DV[`\x80\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x03\xCFW`@RV[``\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x03\xCFW`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x03\xCFW`@RV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\x04?WV[`\0\x80\xFD[4a\x03\x05W`\xE06`\x03\x19\x01\x12a\x03\0W`\xA06`C\x19\x01\x12a\x04\xB8Wa\x02\xFCa\x04\xAC`@Qa\x04s\x81a\x03\xB3V[`D5\x81R`d5` \x82\x01R`\x845`@\x82\x01R`\xA45``\x82\x01R`\xC45a\x04\x9C\x81a\x04.V[`\x80\x82\x01R`$5`\x045a\x16\x8FV[`@Q\x91\x82\x91\x82a\x02\x99V[a\x03LV[``\x90`\x03\x19\x01\x12a\x03\0W`\x045\x90`$5\x90`D5\x90V[4a\x03\x05W` a\x03Da\x04\xF6a\x04\xED6a\x04\xBDV[\x91\x92\x90\x92a\x12\x05V[\x91a\x18xV[4a\x03\x05W` a\x03Da\x05\x0F6a\x04\xBDV[\x90a\x05\"a\x05\x1C\x84a\x12\x05V[\x93a\x13\x8CV[\x92\x91\x90\x91a\x19eV[4a\x03\x05W` a\x03Da\x05>6a\x04\xBDV[\x90a\x05Ka\x05\x1C\x84a\x12\x05V[\x92\x90Pa\x1C\xBBV[\x80\x15\x15\x03a\x04?WV[\x90\x92`\x80\x92a\x02\xAA\x95\x94\x15\x15\x83R` \x83\x01R`@\x82\x01R\x81``\x82\x01R\x01\x90a\x02tV[4a\x03\x05W``6`\x03\x19\x01\x12a\x03\0W`\x045`$5a\x05\xA2\x81a\x05SV[`D5\x91a\x05\xAEa\r\xD5V[a\x05\xB6a\r\xD5V[\x92a\x05\xC0\x83a\x13\x8CV[\x93\x91\x92\x90\x96` \x83\x01\x93`@\x98\x89\x85\x01\x96\x87R\x85R\x83Ra\x05\xF2a\x05\xE3\x87a\x12\x05V[\x95\x84Q\x90\x86Q\x90Q\x91\x89a\x12\xB3V[\x91\x15a\x07xWa\x06j\x93\x92a\x06B\x92a\x06\x1Da\x06$\x93a\x06\x16``\x8A\x01Q\x82a(\x13V[\x93Qa\x0E.V[\x89Ra\x0E.V[a\x066\x88\x88\x01\x91\x80\x83R\x88Q\x88a\r\xC8V[\x90\x87Q\x90Q\x90\x87a\x0C\xDEV[\x90a\x06aa\x06V` \x88\x01\x93\x80\x85Ra\x0E\x1BV[\x80\x84R\x82Q\x11a\x0E\xAFV[Q\x90Q\x90a\x0E\xA2V[\x93[\x83Q\x91` \x85\x01Q\x92a\x06\xAE\x83\x87\x01\x91a\x06\xA0\x83Q\x86Q\x97\x88\x93` \x85\x01`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[\x03`\x1F\x19\x81\x01\x86R\x85a\x04\x0CV[`\0Ta\x06\xD1\x90a\x06\xC5\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x80;\x15a\x07sW\x84`\xC0\x91a\x06\xFC\x97\x86Q\x80\x99\x81\x94\x82\x93c\r\x17\xA7\xC7`\xE3\x1B\x84R0`\x04\x85\x01a\x0F=V[\x03\x91Z\xFA\x94\x85\x15a\x07nW`\0\x95a\x07.W[P\x90a\x07#\x91a\x02\xFC\x95\x96Q\x90Q\x90a\x18xV[\x90Q\x94\x85\x94\x85a\x05]V[a\x02\xFC\x95P\x90a\x07Ya\x07#\x93\x92`\xC0=`\xC0\x11a\x07gW[a\x07Q\x81\x83a\x04\x0CV[\x81\x01\x90a\x0F\x06V[PPPPP\x95P\x90\x91a\x07\x0FV[P=a\x07GV[a\x0C\xD2V[a\x0CYV[a\x07\xD7a\x07\xEA\x92a\x07\xBAa\x07\xF3\x96a\x07\xADa\x07\xDF\x95a\x07\xA6a\x07\x9E``\x8D\x01Q\x83a(\x13V[\x8CQ\x90a(iV[\x92Qa\x0E.V[\x92` \x8C\x01\x93\x84Ra\x0E.V[a\x07\xCC\x8B\x8B\x01\x91\x80\x83R\x83Q\x8Ba\x0FaV[\x91Q\x90Q\x90\x89a\x0FnV[\x80\x88Ra\x0E\x1BV[\x80\x87R\x82Q\x11a\x0E;V[Q\x84Q\x90a\x0E\xA2V[\x93a\x06lV[4a\x03\x05W`@6`\x03\x19\x01\x12a\x03\0Wa\x02\xFC`@Q`\x04` \x82\x01R`\x045`@\x82\x01R`$5``\x82\x01R``\x81Ra\x02\xE8\x81a\x03\xD4V[4a\x03\x05W` 6`\x03\x19\x01\x12a\x03\0W` a\x03D`\x045a\x04\xF6a\x08Y\x82a\x13\x8CV[\x92\x91\x93\x90Pa\x12\x05V[4a\x03\x05W` a\x03Da\x08ya\x04\xED6a\x04\xBDV[\x91a\x1EMV[4a\x03\x05W` a\x03Da\x08\x926a\x04\xBDV[\x90a\x08\x9Fa\x05\x1C\x84a\x12\x05V[\x92\x91\x90\x91a\x1E\xC7V[4a\x03\x05W` a\x03Da\x08\xBB6a\x03\nV[\x92\x91\x90\x91a\x0FnV[4a\x03\x05W`@6`\x03\x19\x01\x12a\x03\0Wa\t\x13`\x045a\x02\xFCa\x08\xF5a\x08\xEA\x83a\x13\x8CV[\x91\x90P`$5a\" V[\x93\x90\x92\x84\x84a\t\ra\t\x06\x84a\x12\x05V[\x83\x83a\x18xV[\x92a\x0C\xDEV[\x92`@Q\x93\x84\x93\x84`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[4a\x03\x05W`\x006`\x03\x19\x01\x12a\x03\0W` `@Q`\0\x81R\xF3[4a\x03\x05W`@6`\x03\x19\x01\x12a\x03\0W`\x045a\t\x9Ba\x02\xFCa\t}a\ts\x84a\x13\x8CV[\x91P`$5a\"MV[\x92\x90\x93\x83\x85a\t\x95a\t\x8E\x84a\x12\x05V[\x83\x83a\x1EMV[\x92a\x0FnV[\x91`@Q\x93\x84\x93\x84`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[\x80Q\x82R` \x80\x82\x01Q\x90\x83\x01R`@\x80\x82\x01Q\x90\x83\x01R``\x80\x82\x01Q\x90\x83\x01R`\x80\x90\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x91\x01RV[4a\x03\x05W` 6`\x03\x19\x01\x12a\x03\0W`\xA0a\n\r`\x045a\x12\x05V[a\n\x1A`@Q\x80\x92a\t\xB9V[\xF3[4a\x03\x05W` a\x03Da\n/6a\x04\xBDV[\x90a\n<a\x05\x1C\x84a\x12\x05V[\x92\x90\x91Pa\"tV[4a\x03\x05W`\x006`\x03\x19\x01\x12a\x03\0W`\0T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x03\x05W` a\x03Da\n\x816a\x03\nV[\x92\x91\x90\x91a\x12\xB3V[4a\x03\x05W` 6`\x03\x19\x01\x12a\x03\0Wa\x02\xFC`@Q`\x01` \x82\x01R`\x045`@\x82\x01R`@\x81Ra\x02\xE8\x81a\x03\xF0V[4a\x03\x05W` 6`\x03\x19\x01\x12a\x03\0Wa\x02\xFC`\x045a\n\xDD\x81a\x04.V[`@\x80Q`\x05` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x82\x82\x01R\x81Ra\x02\xE8\x81a\x03\xF0V[4a\x03\x05W` 6`\x03\x19\x01\x12a\x03\0Wa\x02\xFCa\x0B!`\x045a\x13\x8CV[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R\x90\x81\x90``\x82\x01\x90V[4a\x03\x05W`@6`\x03\x19\x01\x12a\x03\0Wa\x02\xFC`@Q`\x03` \x82\x01R`\x045`@\x82\x01R`$5``\x82\x01R``\x81Ra\x02\xE8\x81a\x03\xD4V[4a\x03\x05W`@6`\x03\x19\x01\x12a\x03\0Wa\t\x13`\x045a\x02\xFCa\x08\xF5a\x0B\x9F\x83a\x13\x8CV[\x91\x90P`$5a\"MV[4a\x03\x05W`@6`\x03\x19\x01\x12a\x03\0W`\x045a\t\x9Ba\x02\xFCa\t}a\x0B\xD0\x84a\x13\x8CV[\x91P`$5a\" V[4a\x03\x05W`\x006`\x03\x19\x01\x12a\x03\0W` `@Q`x\x81R\xF3[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01Rtnor receive functions`X\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01Rd code`\xD8\x1B`d\x82\x01R`\x84\x90\xFD[\x90\x81` \x91\x03\x12a\x03\0WQ\x90V[`@\x90a\x02\xAA\x93\x92\x81R\x81` \x82\x01R\x01\x90a\x02tV[`@Q=`\0\x82>=\x90\xFD[\x90\x91\x92a\x0C\xF5\x90a\x0C\xEE\x83a\x12\x05V[\x90\x85a\x14\x85V[\x90`@Q\x93a\r,\x85a\r\x1E\x83\x86\x88` \x85\x01`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[\x03`\x1F\x19\x81\x01\x87R\x86a\x04\x0CV[`\0Ta\rC\x90a\x06\xC5\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x80;\x15a\x07sWa\rl\x95` \x91`@Q\x80\x80\x99\x81\x94b.RK`\xE0\x1B\x83R\x88`\x04\x84\x01a\x0C\xBBV[\x03\x91Z\xFA\x91\x82\x15a\x07nWa\x02\xAA\x95`\0\x93a\r\x93W[Pa\r\x8D\x90a\x12\x05V[\x93a\x14\xD8V[a\r\x8D\x91\x93Pa\r\xBA\x90` =` \x11a\r\xC1W[a\r\xB2\x81\x83a\x04\x0CV[\x81\x01\x90a\x0C\xACV[\x92\x90a\r\x83V[P=a\r\xA8V[\x91a\x04\xF6a\x02\xAA\x93a\x12\x05V[`@Q\x90``\x82\x01\x82\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x03\xCFW`@R`\0`@\x83\x82\x81R\x82` \x82\x01R\x01RV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x90`\x01\x82\x01\x80\x92\x11a\x0E)WV[a\x0E\x05V[\x91\x90\x82\x01\x80\x92\x11a\x0E)WV[\x15a\x0EBWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: x reserve increase`D\x82\x01Rad!`\xF0\x1B`d\x82\x01R`\x84\x90\xFD[a\x03\xE7\x19\x81\x01\x91\x90\x82\x11a\x0E)WV[\x91\x90\x82\x03\x91\x82\x11a\x0E)WV[\x15a\x0E\xB6WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: y reserve increase`D\x82\x01Rad!`\xF0\x1B`d\x82\x01R`\x84\x90\xFD[\x91\x90\x82`\xC0\x91\x03\x12a\x03\0W\x81Qa\x0F\x1D\x81a\x05SV[\x91` \x81\x01Q\x91`@\x82\x01Q\x91``\x81\x01Q\x91`\xA0`\x80\x83\x01Q\x92\x01Q\x90V[a\x02\xAA\x93\x92``\x92`\x01\x80`\xA0\x1B\x03\x16\x82R` \x82\x01R\x81`@\x82\x01R\x01\x90a\x02tV[\x91a\x08ya\x02\xAA\x93a\x12\x05V[\x90\x91\x92a\x0F\x85\x90a\x0F~\x83a\x12\x05V[\x90\x85a \x9DV[\x90`@Q\x93a\x0F\xAE\x85a\r\x1E\x83\x87\x87` \x85\x01`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[`\0Ta\x0F\xC5\x90a\x06\xC5\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x80;\x15a\x07sWa\x0F\xEE\x95` \x91`@Q\x80\x80\x99\x81\x94b.RK`\xE0\x1B\x83R\x88`\x04\x84\x01a\x0C\xBBV[\x03\x91Z\xFA\x91\x82\x15a\x07nWa\x02\xAA\x95`\0\x93a\x10\x15W[Pa\x10\x0F\x90a\x12\x05V[\x93a \xCAV[a\x10\x0F\x91\x93Pa\x103\x90` =` \x11a\r\xC1Wa\r\xB2\x81\x83a\x04\x0CV[\x92\x90a\x10\x05V[\x91\x90` \x92\x83\x81\x83\x03\x12a\x03\0W\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x82\x82\x11a\x11RW\x01\x82`\x1F\x82\x01\x12\x15a\x10\xF9W\x80Q\x91\x82\x11a\x03\xCFW`@Q\x92a\x10\x87`\x1F\x84\x01`\x1F\x19\x16\x87\x01\x85a\x04\x0CV[\x82\x84R\x85\x83\x83\x01\x01\x11a\x10\xA4W\x84\x83\x94\x95a\x02\xAA\x94\x01\x91\x01a\x02QV[`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x86\x90R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x86\x90R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x87\x90R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[\x91\x90\x82`\xA0\x91\x03\x12a\x04\xB8W`@Qa\x11\xBA\x81a\x03\xB3V[`\x80\x80\x82\x94\x80Q\x84R` \x81\x01Q` \x85\x01R`@\x81\x01Q`@\x85\x01R``\x81\x01Q``\x85\x01R\x01Q\x91a\x11\xED\x83a\x04.V[\x01RV[\x90`\xA0\x82\x82\x03\x12a\x03\0Wa\x02\xAA\x91a\x11\xA2V[`\x80\x90`@Qa\x12\x14\x81a\x03\xB3V[`\0\x92\x81\x84\x80\x93R\x82` \x82\x01R\x82`@\x82\x01R\x82``\x82\x01R\x01Ra\x12Fa\x06\xC5a\x06\xC5\x84T`\x01\x80`\xA0\x1B\x03\x16\x90V[\x80;\x15a\x07sW`@Qc\xDC\x17\x83U`\xE0\x1B\x81R`\x04\x81\x01\x92\x90\x92R\x82\x90\x82\x90`$\x90\x82\x90Z\xFA\x90\x81\x15a\x07nW\x82a\x02\xAA\x93\x92a\x12\x90W[PP` \x80\x82Q\x83\x01\x01\x91\x01a\x11\xF1V[a\x12\xAC\x92P=\x80\x91\x83>a\x12\xA4\x81\x83a\x04\x0CV[\x81\x01\x90a\x10:V[8\x80a\x12\x7FV[`@\x80Q` \x81\x01\x84\x90R\x90\x81\x01\x84\x90R``\x81\x01\x85\x90R\x93\x91\x92a\x12\xDB\x85`\x80\x81\x01a\r\x1EV[`\0T`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x07sWa\x13\x10\x95` \x91`@Q\x80\x80\x99\x81\x94b.RK`\xE0\x1B\x83R\x88`\x04\x84\x01a\x0C\xBBV[\x03\x91Z\xFA\x91\x82\x15a\x07nWa\x02\xAA\x95`\0\x93a\x137W[Pa\x131\x90a\x12\x05V[\x93a$%V[a\x131\x91\x93Pa\x13U\x90` =` \x11a\r\xC1Wa\r\xB2\x81\x83a\x04\x0CV[\x92\x90a\x13'V[\x90\x81` \x91\x03\x12a\x03\0WQa\x02\xAA\x81a\x04.V[\x90\x81``\x91\x03\x12a\x03\0W\x80Q\x91`@` \x83\x01Q\x92\x01Q\x90V[`\0T\x90\x91\x90a\x13\xA6\x90a\x06\xC5\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x80;\x15a\x07sW` `\x04\x91`@Q\x92\x83\x80\x92c+\xEE\x84\xF1`\xE2\x1B\x82RZ\xFA\x90\x81\x15a\x07nW`\0\x91a\x14VW[P`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x07sW`@Qc3\x85N\xFD`\xE2\x1B\x81R`\x04\x81\x01\x93\x90\x93R``\x90\x83\x90`$\x90\x82\x90Z\xFA\x90\x81\x15a\x07nW`\0\x80\x93`\0\x93a\x14\x1FW[P\x92\x91\x90V[\x91\x93PPa\x14E\x91P``=``\x11a\x14OW[a\x14=\x81\x83a\x04\x0CV[\x81\x01\x90a\x13qV[\x92\x90\x92\x918a\x14\x19V[P=a\x143V[a\x14x\x91P` =` \x11a\x14~W[a\x14p\x81\x83a\x04\x0CV[\x81\x01\x90a\x13\\V[8a\x13\xD4V[P=a\x14fV[a\x14\xAE\x90a\x14\xA7a\x14\xA2a\x14\x9D\x86a\x02\xAA\x97\x96a%{V[a%\xF0V[a&#V[\x92Qa(\x13V[a(\x13V[a\x14\xD6\x93``\x92\x96\x95\x93a\x01\0\x83\x01\x97\x83R` \x83\x01R`@\x82\x01R\x01\x90a\t\xB9V[V[\x92\x93`\0\x93\x85\x92\x91\x85\x85\x12\x15a\x16\\W[\x85\x85\x12a\x16=W\x90a\x15\x0Ba\x15\x19\x92[`@\x96`@Q\x95\x86\x94` \x86\x01a\x14\xB3V[\x03`\x1F\x19\x81\x01\x83R\x82a\x04\x0CV[\x81\x85\x92\x85\x96\x82\x81\x11a\x16\x1AWa\x15/\x81\x85a5\xF4V[\x92a\x15:\x81\x86a5\xF4V[\x88a\x15E\x82\x87a\x18$V[\x13a\x15\xF9WP\x90a\x15Y\x91\x97\x96\x92\x97a\x0E\xA2V[`\x01\x95\x91\x82\x91\x87\x80[a\x15tW[PPPPPPPPPP\x90V[\x15a\x15\xD5W[P\x86\x97\x98P\x81\x92a\x15\x94a\x15\x8E\x8B\x89a\x0E.V[`\x01\x1C\x90V[\x99a\x15\x9F\x8B\x88a5\xF4V[\x90\x84a\x15\xAB\x88\x84a\x18$V[\x13a\x15\xC9WPP\x89\x93[\x88a\x15\xC0\x89\x87a\x0E\xA2V[\x92\x01\x94\x99a\x15bV[\x8B\x98P\x90\x95P\x93a\x15\xB5V[`\x14\x10\x80a\x15\xF0W[\x15a\x15\xE9W\x88a\x15zV[\x80\x80a\x15gV[P\x80\x83\x10a\x15\xDEV[`@Qc\x06\xF1\xBE]`\xE2\x1B\x81R`\x04\x81\x01\x86\x90R`$\x81\x01\x91\x90\x91R`D\x90\xFD[`@Qc0\x82\xDF\xDB`\xE1\x1B\x81R`\x04\x81\x01\x91\x90\x91R`$\x81\x01\x92\x90\x92RP`D\x90\xFD[\x93P\x91a\x16I\x90a(?V[\x91a\x16V\x84\x83\x85\x84a'\x1AV[\x93a\x14\xE9V[\x85\x85\x13a\x16pW\x90a\x15\x0Ba\x15\x19\x92a\x14\xF9V[\x93P\x94a\x16|\x90a&fV[\x94a\x16\x89\x84\x83\x88\x84a'\x1AV[\x93a\x16\\V[\x91a\x16\xA0a\x14\xA2a\x14\x9D\x83\x85a1\xB9V[\x91g\r\xE0\xB6\xB3\xA7d\0\0\x92\x83\x03\x92\x83\x11a\x0E)Wa\x16\xF7\x82a\x16\xE3a\x16\xD8a\x14\xA2a\x14\x9D\x84a\x16\xD2a\x17\x15\x9A\x8Ca(iV[\x97a%{V[a\x14\xAE\x85\x84Qa(\x13V[\x92a\x16\xF0\x82\x82\x86\x8Aa'\x1AV[\x84\x88a$%V[\x90`@Q\x94` \x86\x01R`@\x85\x01R``\x84\x01R`\x80\x83\x01\x90a\t\xB9V[a\x01\0\x81Ra\x01 \x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x03\xCFW`@R\x90V[\x90g\r\xE0\xB6\xB3\xA7d\0\0`\0\x83\x82\x03\x93\x12\x81\x84\x12\x81\x16\x91\x84\x13\x90\x15\x16\x17a\x0E)WV[\x90g\x1B\xC1mgN\xC8\0\0`\0\x83\x82\x03\x93\x12\x81\x84\x12\x81\x16\x91\x84\x13\x90\x15\x16\x17a\x0E)WV[\x90g\x11\x90\0\xAB\x10\x0F\xFB\xFF\x19\x82\x01\x91\x82\x13`\x01\x16a\x0E)WV[\x81\x81\x03\x92\x91`\0\x13\x80\x15\x82\x85\x13\x16\x91\x84\x12\x16\x17a\x0E)WV[\x90g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x81\x02\x92\x81\x84\x05\x14\x90\x15\x17\x15a\x0E)WV[\x90g\x1B\xC1mgN\xC8\0\0\x91\x82\x81\x02\x92\x81\x84\x05\x14\x90\x15\x17\x15a\x0E)WV[\x90c;\x9A\xCA\0\x91\x82\x81\x02\x92\x81\x84\x05\x14\x90\x15\x17\x15a\x0E)WV[`\0\x81\x90\x03\x91\x90`\x01`\xFF\x1B\x81\x14`\x01\x16a\x0E)W`\0\x19\x83\x05\x03a\x0E)WV[\x81\x81\x02\x92\x91`\0\x82\x12`\x01`\xFF\x1B\x82\x14\x16a\x0E)W\x81\x84\x05\x14\x90\x15\x17\x15a\x0E)WV[\x81\x15a\x18bW`\x01`\xFF\x1B\x81\x14`\0\x19\x83\x14\x16a\x0E)W\x05\x90V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[\x91\x90\x91a\x18\xB3` \x83\x01\x93a\x18\xAD\x85Qa\x18\xA5a\x18\x9B`@\x88\x01\x92\x83Q\x90a*\xB4V[\x97Q\x82Q\x90a*\xDDV[\x90Q\x90a&\x85V[\x92a&\xA6V[\x92g\r\xE0\xB6\xB3\xA7d\0\0\x80\x85\x12\x15a\x191W`\0\x85\x13\x15a\x19&W`\0\x85\x82\x03\x95\x12\x81\x86\x12\x81\x16\x91\x86\x13\x90\x15\x16\x17a\x0E)Wa\x19\x1Aa\x19\x1F\x92a\x19\x15a\x19\x07a\x14\xA2\x94a\x19\x02a\x02\xAA\x99a*\xFCV[a\x18$V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[a\x17\x97V[a+\x88V[\x90Qa(\x13V[PPPPP`\0\x19\x90V[PPPPP`\0\x90V[\x90\x95\x94\x92\x93a\x14\xD6\x94`\x80\x93a\x01 \x84\x01\x98\x84R` \x84\x01R`@\x83\x01R``\x82\x01R\x01\x90a\t\xB9V[\x90\x92\x91\x85Q` \x87\x01Q`@\x88\x01Q``\x89\x01Q\x91a\x19\x83\x82a-9V[a\x19\x8C\x90a\x1B\x1DV[\x92a\x19\x96\x90a\x178V[a\x19\x9F\x89a\x1CDV[a\x19\xA8\x90a-\xF7V[a\x19\xB1\x89a\x1CVV[a\x19\xBA\x83a.UV[a\x19\xC3\x91a\x17\x97V[a\x19\xCC\x91a/*V[a\x19\xD5\x90a/HV[\x92a\x19\xE0\x83\x80a.\xADV[\x90a\x19\xEA\x91a.\xADV[a\x19\xF3\x90a.\xDCV[a\x19\xFC\x90a\x1C\xAAV[\x83\x85a\x1A\x07\x85a.~V[\x90a\x1A\x11\x91a.\xADV[\x90a\x1A\x1B\x91a.\xADV[a\x1A$\x91a\x1C\x8EV[a\x1A-\x90a+\x88V[a\x1A6\x82a\x1CtV[a\x1A@\x90\x8Ba.\xADV[a\x1AJ\x90\x8Aa\x1C\x8EV[a\x1AT\x90\x87a.\xADV[a\x1A]\x91a.\xADV[a\x1Af\x89a\x1CVV[a\x1Ao\x83a.UV[a\x1Ax\x91a\x17\x97V[a\x1A\x81\x91a/*V[\x94a\x1A\x8B\x90a.&V[\x90a\x1A\x95\x90a\x1CtV[a\x1A\x9E\x91a.\xADV[\x92a\x1A\xA8\x91a.\xADV[a\x1A\xB1\x90a/\x03V[\x90a\x1A\xBB\x91a\x17\x97V[a\x1A\xC4\x90a0\x99V[a\x1A\xCD\x91a.\xADV[\x90a\x1A\xD7\x84a\x1C\xAAV[\x90a\x1A\xE1\x91a\x1C\x8EV[\x90a\x1A\xEB\x91a\x1C\x8EV[`\0\x13a\x1B\x12Wa\x02\xAA\x95a\x1B\r\x93a\x15\x0B\x92`@Q\x96\x87\x95` \x87\x01a\x19;V[a(\xBEV[PPPPPP`\0\x90V[\x90c;\x9A\xCA\0\x91\x82\x81\x02\x92\x81\x84\x04\x14\x90\x15\x17\x15a\x0E)WV[\x90\x81g \x05\xFEO&\x8E\xA0\0\x01\x91\x82\x12`\x01\x16a\x0E)WV[\x90\x81g\r\xC5R\x7Fd, \0\x01\x91\x82\x12`\x01\x16a\x0E)WV[\x90\x81g\r\xE0\xB6\xB3\xA7d\0\0\x01\x91\x82\x12`\x01\x16a\x0E)WV[\x90\x81g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19\x01\x91\x82\x12\x15`\x01\x16a\x0E)WV[\x90\x81g\x14\xA8EL\x19\xE1\xAC\0\x01\x91\x82\x12`\x01\x16a\x0E)WV[\x90\x81g\x0F\xC1\x0E\x01W\x82w\xFF\x19\x01\x91\x82\x12\x15`\x01\x16a\x0E)WV[\x90\x81g\x03\xDE\xBD\x08;\x8C|\0\x01\x91\x82\x12`\x01\x16a\x0E)WV[\x90\x81g\x02\x95\xD4\0\xEA2W\xFF\x19\x01\x91\x82\x12\x15`\x01\x16a\x0E)WV[\x90\x81g\x01W\xD8\xB2\xEC\xC7\x08\0\x01\x91\x82\x12`\x01\x16a\x0E)WV[\x90\x81g\x051\n\xA7\xD5!0\0\x01\x91\x82\x12`\x01\x16a\x0E)WV[\x90\x81g\r\xE0\xCC=\x15a\0\0\x01\x91\x82\x12`\x01\x16a\x0E)WV[\x90\x81a\x03\xE8\x01\x91\x82\x12`\x01\x16a\x0E)WV[\x90a\x03\xE8\x91`\0\x83\x82\x01\x93\x84\x12\x91\x12\x90\x80\x15\x82\x16\x91\x15\x16\x17a\x0E)WV[\x90\x81g\r\xE0\xB6\xB3\xA7c\xFF\xFF\x19\x01\x91\x82\x12\x15`\x01\x16a\x0E)WV[\x91\x90\x91`\0\x83\x82\x01\x93\x84\x12\x91\x12\x90\x80\x15\x82\x16\x91\x15\x16\x17a\x0E)WV[`\x01`\xFF\x1B\x81\x14a\x0E)W`\0\x03\x90V[\x93\x90\x91\x92\x81Q\x93` \x83\x01Q\x91`@\x84\x01Q\x93``\x01Q\x94\x81a\x1C\xDD\x86a-9V[a\x1C\xE6\x90a\x1B\x1DV[\x96a\x1C\xF0\x90a\x178V[\x93\x82a\x1C\xFD\x86\x94\x84a\x1C\x8EV[a\x1D\x06\x90a-\xF7V[a\x1D\x10\x84\x84a\x1C\x8EV[a\x1D\x1A\x86\x86a.\xADV[a\x1D#\x91a\x17\x97V[a\x1D,\x91a/*V[a\x1D5\x90a/HV[\x97a\x1D@\x88\x80a.\xADV[\x90a\x1DJ\x91a.\xADV[a\x1DS\x90a.\xDCV[a\x1D\\\x90a\x1C\xAAV[\x88\x8Aa\x1Dg\x8Aa.~V[\x90a\x1Dq\x91a.\xADV[\x90a\x1D{\x91a.\xADV[a\x1D\x84\x91a\x1C\x8EV[a\x1D\x8D\x90a+\x88V[\x90a\x1D\x97\x85a\x1CtV[a\x1D\xA0\x91a.\xADV[a\x1D\xAA\x90\x83a\x1C\x8EV[a\x1D\xB4\x90\x8Ba.\xADV[a\x1D\xBD\x91a.\xADV[\x93a\x1D\xC7\x91a\x1C\x8EV[\x91a\x1D\xD1\x91a.\xADV[a\x1D\xDA\x91a\x17\x97V[a\x1D\xE3\x91a/*V[\x94a\x1D\xED\x90a.&V[\x90a\x1D\xF7\x90a\x1CtV[a\x1E\0\x91a.\xADV[\x92a\x1E\n\x91a.\xADV[a\x1E\x13\x90a/\x03V[\x90a\x1E\x1D\x91a\x17\x97V[a\x1E&\x90a0\x99V[a\x1E/\x91a.\xADV[\x91a\x1E9\x90a\x1C\xAAV[\x90a\x1EC\x91a\x1C\x8EV[\x90a\x02\xAA\x91a\x1C\x8EV[\x91\x90\x91a\x1E\x8B` \x83\x01\x91a\x1E\x85a\x1E}\x84Qa\x18\xA5a\x1Es`@\x89\x01\x92\x83Q\x90a*\xB4V[\x96Q\x82Q\x90a*\xDDV[\x95\x85Qa&\x85V[\x90a&\xA6V[g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x82\x12\x15a\x191W`\0\x82\x13\x15a\x19&Wa\x02\xAA\x94a\x19\x1F\x93a\x1E\xC1a\x19\x1A\x93a\x19\x02a\x14\xA2\x96a*\xFCV[\x05a\x1C\x8EV[\x90\x92\x91\x85Q` \x87\x01Q\x90`@\x88\x01Q``\x89\x01Q\x90a\x1E\xE6\x81a-9V[a\x1E\xEF\x90a\x1B\x1DV[\x91a\x1E\xF9\x90a\x178V[\x93a\x1F\x03\x86a\x1CDV[a\x1F\x0C\x90a-\xF7V[a\x1F\x16\x89\x86a.\xADV[a\x1F\x1F\x90a\x1CVV[a\x1F(\x87a.UV[a\x1F1\x91a\x17\x97V[a\x1F:\x91a/*V[a\x1FC\x90a/HV[\x91a\x1FN\x82\x80a.\xADV[\x90a\x1FX\x91a.\xADV[a\x1Fa\x90a.\xDCV[a\x1Fj\x90a\x1C\xAAV[\x82\x84a\x1Fu\x84a.~V[\x90a\x1F\x7F\x91a.\xADV[\x90a\x1F\x89\x91a.\xADV[a\x1F\x92\x91a\x1C\x8EV[a\x1F\x9B\x90a+\x88V[a\x1F\xA5\x89\x86a.\xADV[a\x1F\xAE\x87a\x1CtV[a\x1F\xB8\x90\x89a.\xADV[a\x1F\xC1\x91a\x1C\x8EV[a\x1F\xCB\x90\x89a.\xADV[a\x1F\xD4\x91a.\xADV[a\x1F\xDE\x89\x86a.\xADV[a\x1F\xE7\x90a\x1CVV[a\x1F\xF0\x87a.UV[a\x1F\xF9\x91a\x17\x97V[a \x03\x90\x86a.\xADV[a \x0C\x91a/*V[\x94a \x16\x90a\x1CtV[a  \x90\x88a.\xADV[\x92a *\x91a.\xADV[a 3\x90a/\x03V[\x90a =\x91a\x17\x97V[a F\x90a0\x99V[a O\x91a.\xADV[\x90a Y\x90a-\xF7V[a b\x91a/*V[\x90a l\x90a\x1CtV[\x90a v\x91a\x1C\x8EV[`\0\x13a\x1B\x12Wa\x02\xAA\x95a \x98\x93a\x15\x0B\x92`@Q\x96\x87\x95` \x87\x01a\x19;V[a)\xDDV[\x91a\x14\x9Da\x14\xA2\x91a \xAE\x93a1\xB9V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x03\x90\x81\x11a\x0E)Wa\x02\xAA\x91a(\x13V[\x92\x93`\0\x93\x85\x92\x91\x85\x85\x12\x15a!\xEDW[\x85\x85\x12a!\xCEW\x90a\x15\x0Ba \xFC\x92`@\x96`@Q\x95\x86\x94` \x86\x01a\x14\xB3V[\x81\x85\x92\x85\x96\x82\x81\x11a\x16\x1AWa!\x12\x81\x85a6\x15V[\x92a!\x1D\x81\x86a6\x15V[\x88a!(\x82\x87a\x18$V[\x13a\x15\xF9WP\x90a!<\x91\x97\x96\x92\x97a\x0E\xA2V[`\x01\x95\x91\x82\x91\x87\x80[a!VWPPPPPPPPPP\x90V[\x15a!\xB1W[P\x86\x97\x98P\x81\x92a!pa\x15\x8E\x8B\x89a\x0E.V[\x99a!{\x8B\x88a6\x15V[\x90\x84a!\x87\x88\x84a\x18$V[\x13a!\xA5WPP\x89\x93[\x88a!\x9C\x89\x87a\x0E\xA2V[\x92\x01\x94\x99a!EV[\x8B\x98P\x90\x95P\x93a!\x91V[`\x14\x10\x80a!\xC5W[\x15a\x15\xE9W\x88a!\\V[P\x80\x83\x10a!\xBAV[\x93P\x91a!\xDA\x90a(?V[\x91a!\xE7\x84\x83\x83\x86a'\x1AV[\x93a \xDBV[\x85\x85\x13a\"\x01W\x90a\x15\x0Ba \xFC\x92a\x14\xF9V[\x93P\x94a\"\r\x90a&fV[\x94a\"\x1A\x84\x83\x83\x89a'\x1AV[\x93a!\xEDV[\x92\x91\x90a\"6a\"0\x82\x84a(iV[\x85a(\x13V[\x93\x81\x03\x90\x81\x11a\x0E)W\x92\x81\x03\x90\x81\x11a\x0E)W\x90V[\x92\x91\x90a\"]a\"0\x82\x84a(iV[\x93\x81\x01\x80\x91\x11a\x0E)W\x92\x81\x01\x80\x91\x11a\x0E)W\x90V[\x92\x93\x90\x91\x81Q\x93` \x83\x01Q\x91`@\x84\x01Q\x93``\x01Q\x94\x87a\"\x96\x86a-9V[a\"\x9F\x90a\x1B\x1DV[\x96a\"\xA9\x90a\x178V[\x98\x82a\"\xB6\x8B\x94\x83a\x1C\x8EV[a\"\xBF\x90a-\xF7V[\x82a\"\xCA\x87\x8Da.\xADV[\x90a\"\xD4\x91a\x1C\x8EV[a\"\xDE\x86\x85a.\xADV[a\"\xE7\x91a\x17\x97V[a\"\xF0\x91a/*V[a\"\xF9\x90a/HV[\x97a#\x04\x88\x80a.\xADV[\x90a#\x0E\x91a.\xADV[a#\x17\x90a.\xDCV[a# \x90a\x1C\xAAV[\x88\x8Aa#+\x8Aa.~V[\x90a#5\x91a.\xADV[\x90a#?\x91a.\xADV[a#H\x91a\x1C\x8EV[a#Q\x90a+\x88V[\x90a#\\\x86\x8Ca.\xADV[\x90a#f\x86a\x1CtV[a#o\x91a.\xADV[a#x\x91a\x1C\x8EV[a#\x82\x90\x87a.\xADV[a#\x8B\x91a.\xADV[\x93a#\x96\x90\x8Aa.\xADV[\x90a#\xA0\x91a\x1C\x8EV[\x91a#\xAA\x91a.\xADV[a#\xB3\x91a\x17\x97V[a#\xBD\x90\x87a.\xADV[a#\xC6\x91a/*V[\x95a#\xD0\x90a\x1CtV[a#\xD9\x91a.\xADV[\x92a#\xE3\x91a.\xADV[a#\xEC\x90a/\x03V[\x90a#\xF6\x91a\x17\x97V[a#\xFF\x90a0\x99V[a$\x08\x91a.\xADV[\x90a$\x12\x90a-\xF7V[a$\x1B\x91a/*V[\x90a\x1EC\x90a\x1CtV[\x92\x93`\0\x93\x85\x92\x91\x85\x85\x12\x15a%HW[\x85\x85\x12a%)W\x90a\x15\x0Ba$W\x92`@\x96`@Q\x95\x86\x94` \x86\x01a\x14\xB3V[\x81\x85\x92\x85\x96\x82\x81\x11a\x16\x1AWa$m\x81\x85a67V[\x92a$x\x81\x86a67V[\x88a$\x83\x82\x87a\x18$V[\x13a\x15\xF9WP\x90a$\x97\x91\x97\x96\x92\x97a\x0E\xA2V[`\x01\x95\x91\x82\x91\x87\x80[a$\xB1WPPPPPPPPPP\x90V[\x15a%\x0CW[P\x86\x97\x98P\x81\x92a$\xCBa\x15\x8E\x8B\x89a\x0E.V[\x99a$\xD6\x8B\x88a67V[\x90\x84a$\xE2\x88\x84a\x18$V[\x13a%\0WPP\x89\x93[\x88a$\xF7\x89\x87a\x0E\xA2V[\x92\x01\x94\x99a$\xA0V[\x8B\x98P\x90\x95P\x93a$\xECV[`\x14\x10\x80a% W[\x15a\x15\xE9W\x88a$\xB7V[P\x80\x83\x10a%\x15V[\x93P\x94a%5\x90a&fV[\x94a%B\x84\x87\x84\x84a'\x1AV[\x93a$6V[\x85\x85\x13a%\\W\x90a\x15\x0Ba$W\x92a\x14\xF9V[\x93P\x91a%h\x90a(?V[\x91a%u\x84\x84\x84\x84a'\x1AV[\x93a%HV[a%\xEBa%\xE6a\x02\xAA\x93a%\xE0a%\xDB\x82Qa%\xD6g\r\xE0\xB6\xB3\xA7d\0\0a%\xD0a%\xCBa%\xC5a%\xC0`@` \x8B\x01Q\x9A\x01Q\x96a%\xBA\x88\x8Ca*\xB4V[\x9Da(iV[a20V[\x97a20V[a\x17\xCDV[\x05a+\x88V[a&\x85V[a&\xC8V[\x90a\x17\x97V[a\x17\xB0V[a\x18GV[a&\x1Fa%\xE6a&\x1Ag\x13\xA0K\xBD\xFD\xC9\xBE\x88a&\x14g\x1B\xC1mgN\xC8\0\0\x95a\x17\xB0V[\x05a\x1C\xAAV[a0\x99V[\x05\x90V[`\0\x81\x12a&.W\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01RotoUint: negative`\x80\x1B`D\x82\x01R`d\x90\xFD[a\x03\xE7\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x04?Wa\x03\xE8\x90\x04\x90V[\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x04?Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x04\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17\x82\x15\x15\x16\x15a\x04?W\x04\x90V[g\x06\xF0[Y\xD3\xB2\0\0\x90\x80\x82\x02\x91\x82\x04\x14`\x01\x16\x15a\x04?Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x04\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x04?Wg\x1B\xC1mgN\xC8\0\0\x90\x04\x90V[\x90\x92\x82\x82\x10\x15a'\xCEWa\x02\xAA\x93a'\x97\x92\x84g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82a'B\x83\x83a&\xA6V[\x10a'\xBBWP`\x01`\x01`\xFF\x1B\x03\x95\x90P[\x83Q\x91a'ja'd\x83\x85a&\x85V[\x85a&\xA6V[\x10a'\x9CWP`\x01`\x01`\xFF\x1B\x03\x92a'\x91\x92P\x90P[`@` \x82\x01Q\x91\x01Q\x90a*\xB4V[\x92a\x1C\x8EV[a\x1C\x8EV[a'\x91\x92a\x1E\x85a'\xB0\x92a'\xB5\x94a&\x85V[a*\xFCV[\x91a'\x81V[a'\xC8\x91a'\xB0\x91a&\xA6V[\x94a'TV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FtradingFunction: invalid x\0\0\0\0\0\0`D\x82\x01R`d\x90\xFD[\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x04?W`\x01g\r\xE0\xB6\xB3\xA7d\0\0`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[a\x03\xE9\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x04?W`\x01a\x03\xE8`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[\x90g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x81\x02\x92\x81\x84\x04\x14\x90\x15\x17\x81\x15\x15\x16\x15a\x04?W`\x01\x90`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[a\x01\0\x81\x83\x03\x12a\x03\0W\x80Q\x92a\x02\xAA` \x83\x01Q\x93```@\x85\x01Q\x94\x01a\x11\xA2V[\x91\x90a\x03\xE8\x92`\0\x93`\0\x91\x83\x82\x11a)\xBCWa(\xDA\x81a6WV[a(\xE4\x85\x83a7zV[`\0a(\xF0\x82\x84a\x18$V[\x13a)\x9DWPa)\x01\x85\x96\x95a\x0E\x92V[`\x01\x94`\0\x91\x86\x80[a)\x1BW[PPPPPPPP\x90PV[\x15a)xW[P\x85\x96\x97\x98P\x80\x91a)6a\x15\x8E\x8B\x88a\x0E.V[\x99a)A\x8B\x87a7zV[\x90\x83a)M\x87\x84a\x18$V[\x13a)lWPP\x89\x92[\x87a)b\x88\x86a\x0E\xA2V[\x92\x01\x93\x99\x98a)\nV[\x8B\x97P\x90\x94P\x92a)WV[\x86\x10\x80a)\x92W[\x15a)\x8BW\x88a)!V[\x80\x80a)\x0FV[Pa\x01\0\x82\x10a)\x80V[`@Qc\x06\xF1\xBE]`\xE2\x1B\x81R`\x04\x81\x01\x92\x90\x92R`$\x82\x01R`D\x90\xFD[`@Qc0\x82\xDF\xDB`\xE1\x1B\x81Ra\x03\xE8`\x04\x82\x01R`$\x81\x01\x85\x90R`D\x90\xFD[\x91\x90a\x03\xE8\x92`\0\x93`\0\x91\x83\x82\x11a)\xBCWa)\xF9\x81a7\x9CV[a*\x03\x85\x83a8\xDFV[`\0a*\x0F\x82\x84a\x18$V[\x13a)\x9DWPa* \x85\x96\x95a\x0E\x92V[`\x01\x94`\0\x91\x86\x80[a*9WPPPPPPPP\x90PV[\x15a*\x96W[P\x85\x96\x97\x98P\x80\x91a*Ta\x15\x8E\x8B\x88a\x0E.V[\x99a*_\x8B\x87a8\xDFV[\x90\x83a*k\x87\x84a\x18$V[\x13a*\x8AWPP\x89\x92[\x87a*\x80\x88\x86a\x0E\xA2V[\x92\x01\x93\x99\x98a*)V[\x8B\x97P\x90\x94P\x92a*uV[\x86\x10\x80a*\xA9W[\x15a)\x8BW\x88a*?V[Pa\x01\0\x82\x10a*\x9EV[\x90a*\xBE\x90a-9V[c;\x9A\xCA\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17\x15a\x0E)Wa\x02\xAA\x91a&\x85V[a\x02\xAA\x91a%\xD6g\r\xE0\xB6\xB3\xA7d\0\0a%\xD0a%\xCBa%\xDB\x95a20V[g\x06\xF0[Y\xD3\xB2\0\0\x81\x14a+\x82Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x12\x15a+pW\x80\x15a+^W\x80`\x01\x1B\x90\x81\x05`\x02\x03a\x0E)Wa+:\x90a/HV[\x90g\x13\xA0K\xBD\xFD\xC9\xBE\x88\x91\x80\x83\x02\x92\x83\x05\x14`\x01\x16\x15a\x04?Wa\x02\xAA\x91\x05a\x1C\xAAV[`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x90\xFD[`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x90\xFD[P`\0\x90V[h\x02H\xCE6\xA7\x0C\xB2k>\x19\x81\x13\x15a+\x82Wh\x07U\xBFy\x8BJ\x1B\xF1\xE5\x81\x12\x15a,\xD8We\x03x-\xAC\xE9\xD9\x90`N\x1B\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x91``\x90`\x01`_\x1B\x84\x82\x84\x1B\x05\x01\x82\x1D\x93\x84\x02\x90\x03\x80l\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x81\x01\x02\x82\x1D\x90n\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dPn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x82n\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x81m\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x81m\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x81m\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x81l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x02\x8D\x1D\x01\x02\x8B\x1D\x01\x02\x89\x1D\x01\x02\x87\x1D\x01\x02\x85\x1D\x01\x93m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x93m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEA\x81\x01\x90\x84m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x91\x01\x01\x02\x90\x1D\x01\x02\x01\x05\x02\x90`\xC3\x03\x1C\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x90\xFD[\x91\x90a\x01 \x83\x82\x03\x12a\x03\0W\x82Q\x92` \x81\x01Q\x92a\x02\xAA`@\x83\x01Q\x93`\x80``\x85\x01Q\x94\x01a\x11\xA2V[`\xB5\x81`\x01`\x88\x1B\x81\x10\x15a-\xE0W[\x80i\x01\0\0\0\0\0\0\0\0\0b\x01\0\0\x92\x10\x15a-\xD3W[e\x01\0\0\0\0\0\x81\x10\x15a-\xC6W[c\x01\0\0\0\x81\x10\x15a-\xB9W[\x01\x02`\x12\x1C`\x01\x90\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x80\x80\x92\x04\x10\x90\x03\x90V[`\x10\x1C\x91`\x08\x1B\x91a-}V[` \x1C\x91`\x10\x1B\x91a-pV[`@\x1C\x91` \x1B\x91a-aV[Ph\xB5\0\0\0\0\0\0\0\0\x90P`\x80\x82\x90\x1Ca-IV[g\x1B\xC1mgN\xC8\0\0\x81\x81\x02\x91`\x01`\xFF\x1B\x81\x13`\x01\x17\x91\x83\x05\x14\x16\x15a\x04?Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\x06\xF0[Y\xD3\xB2\0\0\x81\x81\x02\x91`\x01`\xFF\x1B\x81\x13`\x01\x17\x91\x83\x05\x14\x16\x15a\x04?Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[a\x03\xE8\x81\x81\x02\x91`\x01`\xFF\x1B\x81\x13`\x01\x17\x91\x83\x05\x14\x16\x15a\x04?Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\x13\xA0K\xBD\xE7\x8C\xC4\0\x81\x81\x02\x91`\x01`\xFF\x1B\x81\x13`\x01\x17\x91\x83\x05\x14\x16\x15a\x04?Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[\x81\x81\x02\x91\x81\x15\x82\x84\x05\x82\x14\x17`\0\x19\x90\x92\x10`\x01`\xFF\x1B\x90\x91\x13\x17\x16\x15a\x04?Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x82\x05\x14`\x01\x16\x15a\x04?Wg\x1B\xC1mgN\xC8\0\0\x90\x05\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x82\x05\x14`\x01\x16\x15a\x04?Wg\x13\xA0K\xBD\xE7\x8C\xC4\0\x90\x05\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x82\x05\x14\x82\x15\x15\x16\x15a\x04?W\x05\x90V[`\0\x81\x12\x80\x15a0\x88W[a0vW\x80\x15a+pWg\x1B\xC1mgN\xC8\0\0\x81\x14a+^Wg\r\xE0\xB6\xB3\xA7d\0\0\x81\x12\x90\x81\x15a0gW\x90[a/\x89\x82a3\xFEV[\x80\x15a+pWa/\xF2a/\xB6a/\xB1a/\xACa/\xA7a/\xF7\x95a20V[a4\xBFV[a-9V[a\x17\xEAV[a\x19\x15a/\xCAa/\xC5\x83a4)V[a\x1B6V[a/\xECa/\xE7a/\xE1a/\xDC\x86a4TV[a\x1BNV[\x85a56V[a\x1BfV[\x90a4\x9DV[a4\xE7V[\x91`\0\x90[`\x02\x82\x10a0\x17WPP\x15a0\x0EW\x90V[a\x02\xAA\x90a\x1C\xAAV[\x90\x92a0_\x81a0Ya0/\x85a\x19\x15`\x01\x96a0\x99V[a/\xECa0Oa0Ja\x19\x1Aa0E\x87\x80a56V[a\x1C\xAAV[a5\x0FV[a%\xE0\x83\x86a56V[\x90a\x1C\x8EV[\x93\x01\x90a/\xFCV[a0p\x90a\x17[V[\x90a/\x80V[`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x90\xFD[Pg\x1B\xC1mgN\xC8\0\0\x81\x13a/SV[\x80\x15a1\xACWgV\x98\xEE\xF0fp\0\0\x81\x12\x15a+\x82WgV\x98\xEE\xF0fo\xFF\xFF\x19\x81\x13\x15a1\x9FW`\0a1\x8Fa0\xCE\x83a3\xD1V[a1Wa\x19\x1Aa0\xE8a0\xE3a/\xE7\x85a&\xEFV[a4~V[\x92a'\x97a1\x8Aa1\x85a1~a1xa1sa1ma1ha1ba1]\x8Da1Wa1Ra1La1Ga/\xE1a1Ba1<a17a11a1,\x8Aa5WV[a\x1B~V[\x89a56V[a\x1B\x98V[\x87a56V[a\x1B\xB0V[a\x1B\xCAV[\x83a56V[a\x1B\xE2V[\x90a56V[a\x1B\xFCV[\x8Ca56V[a\x1C\x14V[\x8Aa56V[a\x1C,V[\x88a56V[\x93\x80a56V[a\x18\x03V[a\x17~V[\x91\x12\x15a\x02\xAAWa\x02\xAA\x90a\x17[V[Pg\x1B\xC1mgN\xC8\0\0\x90V[Pg\r\xE0\xB6\xB3\xA7d\0\0\x90V[a%\xEBa%\xE6a\x02\xAA\x93a0Ya%\xDB\x82Qa%\xD6g\r\xE0\xB6\xB3\xA7d\0\0a%\xD0a%\xCBa%\xC5a%\xC0`@` \x8B\x01Q\x9A\x01Q\x96a%\xBA\x88\x8Ca*\xB4V[\x15a1\xFFWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x90\xFD[}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x90a2\\`\0\x82\x13a1\xF8V[q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06a2x\x82a5\x82V[``\x92\x83\x82`\x9F\x03\x01\x1B`\x9F\x1C\x90`_\x19\x01}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x02\x92l\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x82m\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x81m\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x81m\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x81m\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x81m\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x81lFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x02\x8C\x1D\x01\x02\x8A\x1D\x01\x02\x88\x1D\x01\x02\x86\x1D\x01\x02\x84\x1D\x01\x02\x82\x1D\x01\x91x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x91l\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x90\x82m\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x81l\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x81m\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x81m\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x81l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x81\x01\x02\x89\x1D\x01\x02\x87\x1D\x01\x02\x85\x1D\x01\x02\x83\x1D\x01\x02\x90\x1D\x01\x02\x01\x05\x02\x01\x01`\xAE\x1D\x90V[`\x01`\xFF\x1B\x81\x14a3\xECW`\0\x81\x12\x15a\x02\xAAW\x19`\x01\x01\x90V[`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x90\xFD[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x04?Wg\x1B\xC1mgN\xC8\0\0\x90\x05\x90V[g\x03\xC1f\\z\xAB \0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x04?Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[f\x9F2u$b\xA0\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x04?Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[`\x01\x81\x15\x15\x16\x15a\x04?Wn\xC0\x97\xCE{\xC9\x07\x15\xB3K\x9F\x10\0\0\0\0\x05\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17\x82\x15\x15\x16\x15a\x04?W\x05\x90V[g\x1B\xC1mgN\xC7\xFF\xFF\x19\x81\x81\x02\x91`\x01\x91\x83\x05\x14\x16\x15a\x04?Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\t\xD0(\xCCo _\xFF\x19\x81\x81\x02\x91`\x01\x91\x83\x05\x14\x16\x15a\x04?Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x90\x80\x82\x02\x91\x82\x05\x14`\x01\x16\x15a\x04?Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x04?Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\x02_\x0F\xE1\x05\xA3\x14\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x04?Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[a5\x8D\x81\x15\x15a1\xF8V[\x80o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x07\x1B\x81\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x81\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x81\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x81\x81\x1C`\xFF\x10`\x03\x1B\x17\x81\x81\x1C`\x0F\x10`\x02\x1B\x17\x81\x81\x1C`\x03\x10`\x01\x1B\x17\x90\x81\x1C`\x01\x10\x17\x90V[\x90a6\x0Ba\x02\xAA\x92` \x80\x82Q\x83\x01\x01\x91\x01a(\x99V[\x93\x91\x92\x90Pa'\x1AV[\x90a6,a\x02\xAA\x92` \x80\x82Q\x83\x01\x01\x91\x01a(\x99V[\x93\x90P\x91\x90\x91a'\x1AV[\x90a6Na\x02\xAA\x92` \x80\x82Q\x83\x01\x01\x91\x01a(\x99V[\x93\x92\x90Pa'\x1AV[\x80Q\x81\x01` \x01\x90` \x01\x90a6l\x91a-\x0CV[\x80\x91\x92PQ\x92` \x82\x01Q\x90`@\x83\x01Q\x92``\x01Q\x93a6\x8C\x84a-9V[a6\x95\x90a\x1B\x1DV[\x94a6\x9F\x90a\x178V[\x91a6\xA9\x81a\x1CDV[a6\xB2\x90a-\xF7V[a6\xBB\x83a\x1CVV[a6\xC4\x85a.UV[a6\xCD\x91a\x17\x97V[a6\xD6\x91a/*V[a6\xDF\x90a/HV[\x94a6\xEA\x85\x80a.\xADV[\x90a6\xF4\x91a.\xADV[a6\xFD\x90a.\xDCV[a7\x06\x90a\x1C\xAAV[\x85\x87a7\x11\x87a.~V[\x90a7\x1B\x91a.\xADV[\x90a7%\x91a.\xADV[a7.\x91a\x1C\x8EV[a77\x90a+\x88V[\x90a7A\x84a\x1CtV[a7J\x91a.\xADV[a7T\x90\x83a\x1C\x8EV[a7^\x90\x88a.\xADV[a7g\x91a.\xADV[\x90a7q\x90a\x1CVV[a\x1D\xD1\x83a.UV[\x90a7\x91a\x02\xAA\x92` \x80\x82Q\x83\x01\x01\x91\x01a-\x0CV[\x94\x93\x90\x92\x91Pa\x1C\xBBV[\x80Q\x81\x01` \x01\x90` \x01\x90a7\xB1\x91a-\x0CV[\x80\x91\x92\x94\x93PQ\x92` \x82\x01Q\x90`@\x83\x01Q\x92``\x01Q\x93a7\xD3\x84a-9V[a7\xDC\x90a\x1B\x1DV[\x94a7\xE6\x90a\x178V[\x96a7\xF0\x81a\x1CDV[a7\xF9\x90a-\xF7V[a8\x03\x83\x89a.\xADV[a8\x0C\x90a\x1CVV[a8\x15\x8Aa.UV[a8\x1E\x91a\x17\x97V[a8'\x91a/*V[a80\x90a/HV[\x94a8;\x85\x80a.\xADV[\x90a8E\x91a.\xADV[a8N\x90a.\xDCV[a8W\x90a\x1C\xAAV[\x85\x87a8b\x87a.~V[\x90a8l\x91a.\xADV[\x90a8v\x91a.\xADV[a8\x7F\x91a\x1C\x8EV[a8\x88\x90a+\x88V[\x90a8\x93\x83\x89a.\xADV[\x90a8\x9D\x8Aa\x1CtV[a8\xA6\x91a.\xADV[a8\xAF\x91a\x1C\x8EV[a8\xB9\x90\x84a.\xADV[a8\xC2\x91a.\xADV[\x90a8\xCD\x90\x87a.\xADV[a8\xD6\x90a\x1CVV[a#\xAA\x88a.UV[\x90a8\xF6a\x02\xAA\x92` \x80\x82Q\x83\x01\x01\x91\x01a-\x0CV[\x94\x93\x90\x92Pa\"tV\xFE\xA2dipfsX\"\x12 \xB5\x86C\xEE\xF7?&\xF5+\xBE$<\xCF\xA1\x07w\x17\xC6\x15/\xE7\xC9\xA07\xFE\xC4\xAE#s~\x04\xD8dsolcC\0\x08\x16\x003";
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
        ComputeOptimalArbLowerPrice(ComputeOptimalArbLowerPriceCall),
        ComputeOptimalArbRaisePrice(ComputeOptimalArbRaisePriceCall),
        DeallocateGivenX(DeallocateGivenXCall),
        DeallocateGivenY(DeallocateGivenYCall),
        FetchPoolParams(FetchPoolParamsCall),
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
                Self::ComputeOptimalArbLowerPrice(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ComputeOptimalArbRaisePrice(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DeallocateGivenX(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeallocateGivenY(element) => ::core::fmt::Display::fmt(element, f),
                Self::FetchPoolParams(element) => ::core::fmt::Display::fmt(element, f),
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
