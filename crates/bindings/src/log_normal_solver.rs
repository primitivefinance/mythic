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
    const __BYTECODE: &[u8] = b"`\x80\x80`@R4b\0\0\xECW`@Q`\x1Fb\0;\x878\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x91`\x01`\x01`@\x1B\x03\x83\x11\x84\x84\x10\x17b\0\0\xD6W\x80\x84\x92` \x94`@R\x839\x81\x01\x03\x12b\0\0\x86WQ`\x01`\x01`\xA0\x1B\x03\x81\x16\x90\x81\x90\x03b\0\0\x81W`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x91\x90\x91\x17\x90U`@Qa:M\x90\x81b\0\x01:\x829\xF3[`\0\x80\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[bF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01Ra7\xB7`\xF1\x1B`d\x82\x01R`\x84\x90\xFD\xFE`\x80`@R`\x046\x10a\x0C\x1CW`\x005`\xE0\x1C\x80c\x04 X\n\x14a\x01\xACW\x80c\x12\x06I\xC5\x14a\x01\xA7W\x80c\x13N\xAD\x12\x14a\x01\xA2W\x80c\x1E\x97\x8C\xB0\x14a\x01\x9DW\x80c0m\xB4k\x14a\x01\x98W\x80c3\"f\xF3\x14a\x01\x93W\x80c9(\xFF\x97\x14a\x01\x8EW\x80c;&\x8D]\x14a\x01\x89W\x80c;M\x100\x14a\x01\x84W\x80cN\x81\x7F\xD9\x14a\x01\x7FW\x80cO\xD6|X\x14a\x01zW\x80c^\xB4\x08\xFC\x14a\x01uW\x80cb7V\x9F\x14a\x01pW\x80cme\"\x99\x14a\x01kW\x80c\x7F\x17@\x9C\x14a\x01fW\x80c\x81\xB5\xFA\xC2\x14a\x01aW\x80c\x90.\xCA\xA2\x14a\x01\\W\x80c\xA8\xC6.v\x14a\x01WW\x80c\xAFNC\x7F\x14a\x01RW\x80c\xB0\x9D\x04\xE5\x14a\x01MW\x80c\xCB\x1FU2\x14a\x01HW\x80c\xCE\x15;\xF4\x14a\x01CW\x80c\xE9G\x16\xD5\x14a\x01>W\x80c\xEE>\x8C\xFB\x14a\x019W\x80c\xF3\r7\xF2\x14a\x014Wc\xF9\xC2\x82\x11\x03a\x0C\x1CWa\x0C\0V[a\x0B\xD0V[a\x0B\x9FV[a\x0BdV[a\x0B(V[a\n\xE3V[a\n\xB0V[a\n\x94V[a\nkV[a\nBV[a\n\x15V[a\tsV[a\tWV[a\x08\xEAV[a\x08\xCEV[a\x08\xA5V[a\x08\x89V[a\x08ZV[a\x08\x1FV[a\x05\x82V[a\x05+V[a\x04\xFCV[a\x04\xD7V[a\x04DV[a\x03(V[a\x02\xADV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01Ra7\xB7`\xF1\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[`\0[\x83\x81\x10a\x02dWPP`\0\x91\x01RV[\x81\x81\x01Q\x83\x82\x01R` \x01a\x02TV[\x90` \x91a\x02\x8D\x81Q\x80\x92\x81\x85R\x85\x80\x86\x01\x91\x01a\x02QV[`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[\x90` a\x02\xAA\x92\x81\x81R\x01\x90a\x02tV[\x90V[4a\x03\x05W`@6`\x03\x19\x01\x12a\x03\0Wa\x02\xFC`@Q`\x02` \x82\x01R`\x045`@\x82\x01R`$5``\x82\x01R``\x81Ra\x02\xE8\x81a\x03\xD4V[`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x02tV[\x03\x90\xF3[a\x02\x01V[a\x01\xB1V[`\x80\x90`\x03\x19\x01\x12a\x03\0W`\x045\x90`$5\x90`D5\x90`d5\x90V[4a\x03\x05W` a\x03Da\x03;6a\x03\nV[\x92\x91\x90\x91a\r\x04V[`@Q\x90\x81R\xF3[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FABI decoding: struct data too sh`D\x82\x01Rb\x1B\xDC\x9D`\xEA\x1B`d\x82\x01R`\x84\x90\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\xA0\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x03\xCFW`@RV[a\x03\x9DV[`\x80\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x03\xCFW`@RV[``\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x03\xCFW`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x03\xCFW`@RV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\x04?WV[`\0\x80\xFD[4a\x03\x05W`\xE06`\x03\x19\x01\x12a\x03\0W`\xA06`C\x19\x01\x12a\x04\xB8Wa\x02\xFCa\x04\xAC`@Qa\x04s\x81a\x03\xB3V[`D5\x81R`d5` \x82\x01R`\x845`@\x82\x01R`\xA45``\x82\x01R`\xC45a\x04\x9C\x81a\x04.V[`\x80\x82\x01R`$5`\x045a\x16\xB0V[`@Q\x91\x82\x91\x82a\x02\x99V[a\x03LV[``\x90`\x03\x19\x01\x12a\x03\0W`\x045\x90`$5\x90`D5\x90V[4a\x03\x05W` a\x03Da\x04\xF6a\x04\xED6a\x04\xBDV[\x91\x92\x90\x92a\x12+V[\x91a\x18\x99V[4a\x03\x05W` a\x03Da\x05\x0F6a\x04\xBDV[\x90a\x05\"a\x05\x1C\x84a\x12+V[\x93a\x13\xB2V[\x92\x91\x90\x91a\x19\x86V[4a\x03\x05W` a\x03Da\x05>6a\x04\xBDV[\x90a\x05Ka\x05\x1C\x84a\x12+V[\x92\x90Pa\x1C\xF0V[\x80\x15\x15\x03a\x04?WV[\x90\x92`\x80\x92a\x02\xAA\x95\x94\x15\x15\x83R` \x83\x01R`@\x82\x01R\x81``\x82\x01R\x01\x90a\x02tV[4a\x03\x05W``6`\x03\x19\x01\x12a\x03\0W`$5`\x045a\x05\xA2\x82a\x05SV[`D5\x90a\x05\xF5a\x05\xB1a\r\xFBV[\x92a\x05\xBAa\r\xFBV[\x93a\x05\xC4\x84a\x13\xB2V[` \x84\x99\x93\x95\x92\x99\x01\x94`@\x99\x8A\x86\x01\x92\x83R\x86R\x84Ra\x05\xE4\x87a\x12+V[\x95\x86\x91\x85Q\x90\x87Q\x90Q\x91\x8Aa\x12\xD9V[\x92\x15a\x07\x97W\x92\x82a\x06<a\x06C\x93a\x065a\x060a\x06(a\x06a\x98a\x06#``a\x06\x89\x9D\x9C\x01Q\x86a(\xE5V[a(\xE5V[\x86Q\x90a);V[a\x0EAV[\x93Qa\x0ETV[\x89Ra\x0ETV[a\x06U\x88\x88\x01\x91\x80\x83R\x88Q\x88a\r\xEEV[\x90\x87Q\x90Q\x90\x87a\r\x04V[\x90a\x06\x80a\x06u` \x88\x01\x93\x80\x85Ra\x0EAV[\x80\x84R\x82Q\x11a\x0E\xD5V[Q\x90Q\x90a\x0E\xC8V[\x93[\x83Q\x91` \x85\x01Q\x92a\x06\xCD\x83\x87\x01\x91a\x06\xBF\x83Q\x86Q\x97\x88\x93` \x85\x01`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[\x03`\x1F\x19\x81\x01\x86R\x85a\x04\x0CV[`\0Ta\x06\xF0\x90a\x06\xE4\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x80;\x15a\x07\x92W\x84`\xC0\x91a\x07\x1B\x97\x86Q\x80\x99\x81\x94\x82\x93c\r\x17\xA7\xC7`\xE3\x1B\x84R0`\x04\x85\x01a\x0FcV[\x03\x91Z\xFA\x94\x85\x15a\x07\x8DW`\0\x95a\x07MW[P\x90a\x07B\x91a\x02\xFC\x95\x96Q\x90Q\x90a\x18\x99V[\x90Q\x94\x85\x94\x85a\x05]V[a\x02\xFC\x95P\x90a\x07xa\x07B\x93\x92`\xC0=`\xC0\x11a\x07\x86W[a\x07p\x81\x83a\x04\x0CV[\x81\x01\x90a\x0F,V[PPPPP\x95P\x90\x91a\x07.V[P=a\x07fV[a\x0C\xF8V[a\x0C\x7FV[\x82a\x07\xE0a\x08\x19\x96a\x07\xD3a\x08\x05\x95a\x07\xCCa\x060a\x07\xC4a\x08\x10\x9Aa\x06#``a\x07\xFD\x9B\x01Q\x86a(\xE5V[\x85Q\x90a);V[\x92Qa\x0ETV[\x92` \x8C\x01\x93\x84Ra\x0ETV[a\x07\xF2\x8B\x8B\x01\x91\x80\x83R\x83Q\x8Ba\x0F\x87V[\x91Q\x90Q\x90\x89a\x0F\x94V[\x80\x88Ra\x0EAV[\x80\x87R\x82Q\x11a\x0EaV[Q\x84Q\x90a\x0E\xC8V[\x93a\x06\x8BV[4a\x03\x05W`@6`\x03\x19\x01\x12a\x03\0Wa\x02\xFC`@Q`\x04` \x82\x01R`\x045`@\x82\x01R`$5``\x82\x01R``\x81Ra\x02\xE8\x81a\x03\xD4V[4a\x03\x05W` 6`\x03\x19\x01\x12a\x03\0W` a\x03D`\x045a\x04\xF6a\x08\x7F\x82a\x13\xB2V[\x92\x91\x93\x90Pa\x12+V[4a\x03\x05W` a\x03Da\x08\x9Fa\x04\xED6a\x04\xBDV[\x91a\x1E\xB2V[4a\x03\x05W` a\x03Da\x08\xB86a\x04\xBDV[\x90a\x08\xC5a\x05\x1C\x84a\x12+V[\x92\x91\x90\x91a\x1F,V[4a\x03\x05W` a\x03Da\x08\xE16a\x03\nV[\x92\x91\x90\x91a\x0F\x94V[4a\x03\x05W`@6`\x03\x19\x01\x12a\x03\0Wa\t9`\x045a\x02\xFCa\t\x1Ba\t\x10\x83a\x13\xB2V[\x91\x90P`$5a\"\xBDV[\x93\x90\x92\x84\x84a\t3a\t,\x84a\x12+V[\x83\x83a\x18\x99V[\x92a\r\x04V[\x92`@Q\x93\x84\x93\x84`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[4a\x03\x05W`\x006`\x03\x19\x01\x12a\x03\0W` `@Q`\0\x81R\xF3[4a\x03\x05W`@6`\x03\x19\x01\x12a\x03\0W`\x045a\t\xC1a\x02\xFCa\t\xA3a\t\x99\x84a\x13\xB2V[\x91P`$5a\"\xEAV[\x92\x90\x93\x83\x85a\t\xBBa\t\xB4\x84a\x12+V[\x83\x83a\x1E\xB2V[\x92a\x0F\x94V[\x91`@Q\x93\x84\x93\x84`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[\x80Q\x82R` \x80\x82\x01Q\x90\x83\x01R`@\x80\x82\x01Q\x90\x83\x01R``\x80\x82\x01Q\x90\x83\x01R`\x80\x90\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x91\x01RV[4a\x03\x05W` 6`\x03\x19\x01\x12a\x03\0W`\xA0a\n3`\x045a\x12+V[a\n@`@Q\x80\x92a\t\xDFV[\xF3[4a\x03\x05W` a\x03Da\nU6a\x04\xBDV[\x90a\nba\x05\x1C\x84a\x12+V[\x92\x90\x91Pa#\x11V[4a\x03\x05W`\x006`\x03\x19\x01\x12a\x03\0W`\0T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x03\x05W` a\x03Da\n\xA76a\x03\nV[\x92\x91\x90\x91a\x12\xD9V[4a\x03\x05W` 6`\x03\x19\x01\x12a\x03\0Wa\x02\xFC`@Q`\x01` \x82\x01R`\x045`@\x82\x01R`@\x81Ra\x02\xE8\x81a\x03\xF0V[4a\x03\x05W` 6`\x03\x19\x01\x12a\x03\0Wa\x02\xFC`\x045a\x0B\x03\x81a\x04.V[`@\x80Q`\x05` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x82\x82\x01R\x81Ra\x02\xE8\x81a\x03\xF0V[4a\x03\x05W` 6`\x03\x19\x01\x12a\x03\0Wa\x02\xFCa\x0BG`\x045a\x13\xB2V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R\x90\x81\x90``\x82\x01\x90V[4a\x03\x05W`@6`\x03\x19\x01\x12a\x03\0Wa\x02\xFC`@Q`\x03` \x82\x01R`\x045`@\x82\x01R`$5``\x82\x01R``\x81Ra\x02\xE8\x81a\x03\xD4V[4a\x03\x05W`@6`\x03\x19\x01\x12a\x03\0Wa\t9`\x045a\x02\xFCa\t\x1Ba\x0B\xC5\x83a\x13\xB2V[\x91\x90P`$5a\"\xEAV[4a\x03\x05W`@6`\x03\x19\x01\x12a\x03\0W`\x045a\t\xC1a\x02\xFCa\t\xA3a\x0B\xF6\x84a\x13\xB2V[\x91P`$5a\"\xBDV[4a\x03\x05W`\x006`\x03\x19\x01\x12a\x03\0W` `@Q`x\x81R\xF3[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01Rtnor receive functions`X\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01Rd code`\xD8\x1B`d\x82\x01R`\x84\x90\xFD[\x90\x81` \x91\x03\x12a\x03\0WQ\x90V[`@\x90a\x02\xAA\x93\x92\x81R\x81` \x82\x01R\x01\x90a\x02tV[`@Q=`\0\x82>=\x90\xFD[\x90\x91\x92a\r\x1B\x90a\r\x14\x83a\x12+V[\x90\x85a\x14\xABV[\x90`@Q\x93a\rR\x85a\rD\x83\x86\x88` \x85\x01`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[\x03`\x1F\x19\x81\x01\x87R\x86a\x04\x0CV[`\0Ta\ri\x90a\x06\xE4\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x80;\x15a\x07\x92Wa\r\x92\x95` \x91`@Q\x80\x80\x99\x81\x94b.RK`\xE0\x1B\x83R\x88`\x04\x84\x01a\x0C\xE1V[\x03\x91Z\xFA\x91\x82\x15a\x07\x8DWa\x02\xAA\x95`\0\x93a\r\xB9W[Pa\r\xB3\x90a\x12+V[\x93a\x14\xF9V[a\r\xB3\x91\x93Pa\r\xE0\x90` =` \x11a\r\xE7W[a\r\xD8\x81\x83a\x04\x0CV[\x81\x01\x90a\x0C\xD2V[\x92\x90a\r\xA9V[P=a\r\xCEV[\x91a\x04\xF6a\x02\xAA\x93a\x12+V[`@Q\x90``\x82\x01\x82\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x03\xCFW`@R`\0`@\x83\x82\x81R\x82` \x82\x01R\x01RV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x90`\x01\x82\x01\x80\x92\x11a\x0EOWV[a\x0E+V[\x91\x90\x82\x01\x80\x92\x11a\x0EOWV[\x15a\x0EhWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: x reserve increase`D\x82\x01Rad!`\xF0\x1B`d\x82\x01R`\x84\x90\xFD[a\x03\xE7\x19\x81\x01\x91\x90\x82\x11a\x0EOWV[\x91\x90\x82\x03\x91\x82\x11a\x0EOWV[\x15a\x0E\xDCWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: y reserve increase`D\x82\x01Rad!`\xF0\x1B`d\x82\x01R`\x84\x90\xFD[\x91\x90\x82`\xC0\x91\x03\x12a\x03\0W\x81Qa\x0FC\x81a\x05SV[\x91` \x81\x01Q\x91`@\x82\x01Q\x91``\x81\x01Q\x91`\xA0`\x80\x83\x01Q\x92\x01Q\x90V[a\x02\xAA\x93\x92``\x92`\x01\x80`\xA0\x1B\x03\x16\x82R` \x82\x01R\x81`@\x82\x01R\x01\x90a\x02tV[\x91a\x08\x9Fa\x02\xAA\x93a\x12+V[\x90\x91\x92a\x0F\xAB\x90a\x0F\xA4\x83a\x12+V[\x90\x85a!:V[\x90`@Q\x93a\x0F\xD4\x85a\rD\x83\x87\x87` \x85\x01`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[`\0Ta\x0F\xEB\x90a\x06\xE4\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x80;\x15a\x07\x92Wa\x10\x14\x95` \x91`@Q\x80\x80\x99\x81\x94b.RK`\xE0\x1B\x83R\x88`\x04\x84\x01a\x0C\xE1V[\x03\x91Z\xFA\x91\x82\x15a\x07\x8DWa\x02\xAA\x95`\0\x93a\x10;W[Pa\x105\x90a\x12+V[\x93a!gV[a\x105\x91\x93Pa\x10Y\x90` =` \x11a\r\xE7Wa\r\xD8\x81\x83a\x04\x0CV[\x92\x90a\x10+V[\x91\x90` \x92\x83\x81\x83\x03\x12a\x03\0W\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x82\x82\x11a\x11xW\x01\x82`\x1F\x82\x01\x12\x15a\x11\x1FW\x80Q\x91\x82\x11a\x03\xCFW`@Q\x92a\x10\xAD`\x1F\x84\x01`\x1F\x19\x16\x87\x01\x85a\x04\x0CV[\x82\x84R\x85\x83\x83\x01\x01\x11a\x10\xCAW\x84\x83\x94\x95a\x02\xAA\x94\x01\x91\x01a\x02QV[`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x86\x90R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x86\x90R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x87\x90R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[\x91\x90\x82`\xA0\x91\x03\x12a\x04\xB8W`@Qa\x11\xE0\x81a\x03\xB3V[`\x80\x80\x82\x94\x80Q\x84R` \x81\x01Q` \x85\x01R`@\x81\x01Q`@\x85\x01R``\x81\x01Q``\x85\x01R\x01Q\x91a\x12\x13\x83a\x04.V[\x01RV[\x90`\xA0\x82\x82\x03\x12a\x03\0Wa\x02\xAA\x91a\x11\xC8V[`\x80\x90`@Qa\x12:\x81a\x03\xB3V[`\0\x92\x81\x84\x80\x93R\x82` \x82\x01R\x82`@\x82\x01R\x82``\x82\x01R\x01Ra\x12la\x06\xE4a\x06\xE4\x84T`\x01\x80`\xA0\x1B\x03\x16\x90V[\x80;\x15a\x07\x92W`@Qc\xDC\x17\x83U`\xE0\x1B\x81R`\x04\x81\x01\x92\x90\x92R\x82\x90\x82\x90`$\x90\x82\x90Z\xFA\x90\x81\x15a\x07\x8DW\x82a\x02\xAA\x93\x92a\x12\xB6W[PP` \x80\x82Q\x83\x01\x01\x91\x01a\x12\x17V[a\x12\xD2\x92P=\x80\x91\x83>a\x12\xCA\x81\x83a\x04\x0CV[\x81\x01\x90a\x10`V[8\x80a\x12\xA5V[`@\x80Q` \x81\x01\x84\x90R\x90\x81\x01\x84\x90R``\x81\x01\x85\x90R\x93\x91\x92a\x13\x01\x85`\x80\x81\x01a\rDV[`\0T`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x07\x92Wa\x136\x95` \x91`@Q\x80\x80\x99\x81\x94b.RK`\xE0\x1B\x83R\x88`\x04\x84\x01a\x0C\xE1V[\x03\x91Z\xFA\x91\x82\x15a\x07\x8DWa\x02\xAA\x95`\0\x93a\x13]W[Pa\x13W\x90a\x12+V[\x93a$\xF7V[a\x13W\x91\x93Pa\x13{\x90` =` \x11a\r\xE7Wa\r\xD8\x81\x83a\x04\x0CV[\x92\x90a\x13MV[\x90\x81` \x91\x03\x12a\x03\0WQa\x02\xAA\x81a\x04.V[\x90\x81``\x91\x03\x12a\x03\0W\x80Q\x91`@` \x83\x01Q\x92\x01Q\x90V[`\0T\x90\x91\x90a\x13\xCC\x90a\x06\xE4\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x80;\x15a\x07\x92W` `\x04\x91`@Q\x92\x83\x80\x92c+\xEE\x84\xF1`\xE2\x1B\x82RZ\xFA\x90\x81\x15a\x07\x8DW`\0\x91a\x14|W[P`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x07\x92W`@Qc3\x85N\xFD`\xE2\x1B\x81R`\x04\x81\x01\x93\x90\x93R``\x90\x83\x90`$\x90\x82\x90Z\xFA\x90\x81\x15a\x07\x8DW`\0\x80\x93`\0\x93a\x14EW[P\x92\x91\x90V[\x91\x93PPa\x14k\x91P``=``\x11a\x14uW[a\x14c\x81\x83a\x04\x0CV[\x81\x01\x90a\x13\x97V[\x92\x90\x92\x918a\x14?V[P=a\x14YV[a\x14\x9E\x91P` =` \x11a\x14\xA4W[a\x14\x96\x81\x83a\x04\x0CV[\x81\x01\x90a\x13\x82V[8a\x13\xFAV[P=a\x14\x8CV[a\x06#\x90a\x14\xCDa\x14\xC8a\x14\xC3\x86a\x02\xAA\x97\x96a&MV[a&\xC2V[a&\xF5V[\x92Qa(\xE5V[a\x14\xF7\x93``\x92\x96\x95\x93a\x01\0\x83\x01\x97\x83R` \x83\x01R`@\x82\x01R\x01\x90a\t\xDFV[V[\x92\x93`\0\x93\x85\x92\x91\x85\x85\x12\x15a\x16}W[\x85\x85\x12a\x16^W\x90a\x15,a\x15:\x92[`@\x96`@Q\x95\x86\x94` \x86\x01a\x14\xD4V[\x03`\x1F\x19\x81\x01\x83R\x82a\x04\x0CV[\x81\x85\x92\x85\x96\x82\x81\x11a\x16;Wa\x15P\x81\x85a6\xC2V[\x92a\x15[\x81\x86a6\xC2V[\x88a\x15f\x82\x87a\x18EV[\x13a\x16\x1AWP\x90a\x15z\x91\x97\x96\x92\x97a\x0E\xC8V[`\x01\x95\x91\x82\x91\x87\x80[a\x15\x95W[PPPPPPPPPP\x90V[\x15a\x15\xF6W[P\x86\x97\x98P\x81\x92a\x15\xB5a\x15\xAF\x8B\x89a\x0ETV[`\x01\x1C\x90V[\x99a\x15\xC0\x8B\x88a6\xC2V[\x90\x84a\x15\xCC\x88\x84a\x18EV[\x13a\x15\xEAWPP\x89\x93[\x88a\x15\xE1\x89\x87a\x0E\xC8V[\x92\x01\x94\x99a\x15\x83V[\x8B\x98P\x90\x95P\x93a\x15\xD6V[`\x14\x10\x80a\x16\x11W[\x15a\x16\nW\x88a\x15\x9BV[\x80\x80a\x15\x88V[P\x80\x83\x10a\x15\xFFV[`@Qc\x06\xF1\xBE]`\xE2\x1B\x81R`\x04\x81\x01\x86\x90R`$\x81\x01\x91\x90\x91R`D\x90\xFD[`@Qc0\x82\xDF\xDB`\xE1\x1B\x81R`\x04\x81\x01\x91\x90\x91R`$\x81\x01\x92\x90\x92RP`D\x90\xFD[\x93P\x91a\x16j\x90a)\x11V[\x91a\x16w\x84\x83\x85\x84a'\xECV[\x93a\x15\nV[\x85\x85\x13a\x16\x91W\x90a\x15,a\x15:\x92a\x15\x1AV[\x93P\x94a\x16\x9D\x90a'8V[\x94a\x16\xAA\x84\x83\x88\x84a'\xECV[\x93a\x16}V[\x91a\x16\xC1a\x14\xC8a\x14\xC3\x83\x85a2\x87V[\x91g\r\xE0\xB6\xB3\xA7d\0\0\x92\x83\x03\x92\x83\x11a\x0EOWa\x17\x18\x82a\x17\x04a\x16\xF9a\x14\xC8a\x14\xC3\x84a\x16\xF3a\x176\x9A\x8Ca);V[\x97a&MV[a\x06#\x85\x84Qa(\xE5V[\x92a\x17\x11\x82\x82\x86\x8Aa'\xECV[\x84\x88a$\xF7V[\x90`@Q\x94` \x86\x01R`@\x85\x01R``\x84\x01R`\x80\x83\x01\x90a\t\xDFV[a\x01\0\x81Ra\x01 \x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x03\xCFW`@R\x90V[\x90g\r\xE0\xB6\xB3\xA7d\0\0`\0\x83\x82\x03\x93\x12\x81\x84\x12\x81\x16\x91\x84\x13\x90\x15\x16\x17a\x0EOWV[\x90g\x1B\xC1mgN\xC8\0\0`\0\x83\x82\x03\x93\x12\x81\x84\x12\x81\x16\x91\x84\x13\x90\x15\x16\x17a\x0EOWV[\x90g\x11\x90\0\xAB\x10\x0F\xFB\xFF\x19\x82\x01\x91\x82\x13`\x01\x16a\x0EOWV[\x81\x81\x03\x92\x91`\0\x13\x80\x15\x82\x85\x13\x16\x91\x84\x12\x16\x17a\x0EOWV[\x90g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x81\x02\x92\x81\x84\x05\x14\x90\x15\x17\x15a\x0EOWV[\x90g\x1B\xC1mgN\xC8\0\0\x91\x82\x81\x02\x92\x81\x84\x05\x14\x90\x15\x17\x15a\x0EOWV[\x90c;\x9A\xCA\0\x91\x82\x81\x02\x92\x81\x84\x05\x14\x90\x15\x17\x15a\x0EOWV[`\0\x81\x90\x03\x91\x90`\x01`\xFF\x1B\x81\x14`\x01\x16a\x0EOW`\0\x19\x83\x05\x03a\x0EOWV[\x81\x81\x02\x92\x91`\0\x82\x12`\x01`\xFF\x1B\x82\x14\x16a\x0EOW\x81\x84\x05\x14\x90\x15\x17\x15a\x0EOWV[\x81\x15a\x18\x83W`\x01`\xFF\x1B\x81\x14`\0\x19\x83\x14\x16a\x0EOW\x05\x90V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[\x91\x90\x91a\x18\xD4` \x83\x01\x93a\x18\xCE\x85Qa\x18\xC6a\x18\xBC`@\x88\x01\x92\x83Q\x90a+\x86V[\x97Q\x82Q\x90a+\xAFV[\x90Q\x90a'WV[\x92a'xV[\x92g\r\xE0\xB6\xB3\xA7d\0\0\x80\x85\x12\x15a\x19RW`\0\x85\x13\x15a\x19GW`\0\x85\x82\x03\x95\x12\x81\x86\x12\x81\x16\x91\x86\x13\x90\x15\x16\x17a\x0EOWa\x19;a\x19@\x92a\x196a\x19(a\x14\xC8\x94a\x19#a\x02\xAA\x99a+\xCEV[a\x18EV[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[a\x17\xB8V[a,ZV[\x90Qa(\xE5V[PPPPP`\0\x19\x90V[PPPPP`\0\x90V[\x90\x95\x94\x92\x93a\x14\xF7\x94`\x80\x93a\x01 \x84\x01\x98\x84R` \x84\x01R`@\x83\x01R``\x82\x01R\x01\x90a\t\xDFV[\x90\x92\x91\x82\x86Q` \x88\x01Q`@\x89\x01Q\x90``\x8A\x01Q\x92a\x19\xA6\x83a.\x0BV[a\x19\xAF\x90a\x1BpV[\x93a\x19\xB9\x90a\x17YV[\x90a\x19\xC3\x8Aa.\xC9V[a\x19\xCC\x8Ba\x1C\x97V[a\x19\xD5\x91a/{V[a\x19\xDE\x8Ba\x1C\x97V[a\x19\xE7\x84a.\xF8V[a\x19\xF0\x91a\x17\xB8V[a\x19\xFA\x90\x88a/{V[a\x1A\x03\x91a/\xF8V[a\x1A\x0C\x90a0\x16V[\x93a\x1A\x16\x84a2\xFEV[a\x1A\x1F\x90a\x17\xEEV[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x05a\x1A3\x90a,ZV[\x90a\x1A=\x91a/{V[a\x1AF\x90a/\xAAV[a\x1AO\x90a\x1C\xDFV[\x84\x86a\x1AZ\x86a/LV[\x90a\x1Ad\x91a/{V[\x90a\x1An\x91a/{V[a\x1Aw\x91a\x1C\xC3V[a\x1A\x80\x90a,ZV[\x82a\x1A\x8B\x8C\x84a/{V[\x90a\x1A\x95\x91a/{V[a\x1A\x9E\x91a/{V[a\x1A\xA7\x8Ba\x1C\x97V[a\x1A\xB0\x84a.\xF8V[a\x1A\xB9\x91a\x17\xB8V[a\x1A\xC2\x91a/\xF8V[\x95a\x1A\xCC\x91a/{V[\x90a\x1A\xD6\x90a\x1C\xA9V[a\x1A\xDF\x91a/{V[\x92a\x1A\xE9\x91a/{V[a\x1A\xF2\x90a/\xD1V[\x90a\x1A\xFC\x91a\x17\xB8V[a\x1B\x05\x90a1gV[a\x1B\x0E\x91a/{V[a\x1B\x17\x86a.\xC9V[a\x1B \x91a/\xF8V[\x90a\x1B*\x84a\x1C\xDFV[\x90a\x1B4\x91a\x1C\xC3V[\x90a\x1B>\x91a\x1C\xC3V[`\0\x13a\x1BeWa\x02\xAA\x95a\x1B`\x93a\x15,\x92`@Q\x96\x87\x95` \x87\x01a\x19\\V[a)\x90V[PPPPPP`\0\x90V[\x90c;\x9A\xCA\0\x91\x82\x81\x02\x92\x81\x84\x04\x14\x90\x15\x17\x15a\x0EOWV[\x90\x81g \x05\xFEO&\x8E\xA0\0\x01\x91\x82\x12`\x01\x16a\x0EOWV[\x90\x81g\r\xC5R\x7Fd, \0\x01\x91\x82\x12`\x01\x16a\x0EOWV[\x90\x81g\r\xE0\xB6\xB3\xA7d\0\0\x01\x91\x82\x12`\x01\x16a\x0EOWV[\x90\x81g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19\x01\x91\x82\x12\x15`\x01\x16a\x0EOWV[\x90\x81g\x14\xA8EL\x19\xE1\xAC\0\x01\x91\x82\x12`\x01\x16a\x0EOWV[\x90\x81g\x0F\xC1\x0E\x01W\x82w\xFF\x19\x01\x91\x82\x12\x15`\x01\x16a\x0EOWV[\x90\x81g\x03\xDE\xBD\x08;\x8C|\0\x01\x91\x82\x12`\x01\x16a\x0EOWV[\x90\x81g\x02\x95\xD4\0\xEA2W\xFF\x19\x01\x91\x82\x12\x15`\x01\x16a\x0EOWV[\x90\x81g\x01W\xD8\xB2\xEC\xC7\x08\0\x01\x91\x82\x12`\x01\x16a\x0EOWV[\x90\x81g\x051\n\xA7\xD5!0\0\x01\x91\x82\x12`\x01\x16a\x0EOWV[\x90\x81g\r\xE0\xCC=\x15a\0\0\x01\x91\x82\x12`\x01\x16a\x0EOWV[\x90\x81a\x03\xE8\x01\x91\x82\x12`\x01\x16a\x0EOWV[\x90\x81g\r\xE0\xB6\xB3\xA7c\xFF\xFF\x19\x01\x91\x82\x12\x15`\x01\x16a\x0EOWV[\x91\x90\x91`\0\x83\x82\x01\x93\x84\x12\x91\x12\x90\x80\x15\x82\x16\x91\x15\x16\x17a\x0EOWV[`\x01`\xFF\x1B\x81\x14a\x0EOW`\0\x03\x90V[\x93\x92\x90\x91\x92\x80Q\x91` \x82\x01Q\x90`@\x83\x01Q\x92``\x01Q\x93a\x1D\x12\x84a.\x0BV[a\x1D\x1B\x90a\x1BpV[\x94a\x1D%\x90a\x17YV[\x91\x82a\x1D0\x88a.\xC9V[a\x1D:\x89\x84a\x1C\xC3V[a\x1DC\x91a/{V[a\x1DM\x89\x84a\x1C\xC3V[a\x1DW\x83\x85a/{V[a\x1D`\x91a\x17\xB8V[a\x1Dj\x90\x8Ba/{V[a\x1Ds\x91a/\xF8V[a\x1D|\x90a0\x16V[\x95a\x1D\x86\x86a2\xFEV[a\x1D\x8F\x90a\x17\xEEV[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x05a\x1D\xA3\x90a,ZV[\x90a\x1D\xAD\x91a/{V[a\x1D\xB6\x90a/\xAAV[a\x1D\xBF\x90a\x1C\xDFV[\x86\x88a\x1D\xCA\x88a/LV[\x90a\x1D\xD4\x91a/{V[\x90a\x1D\xDE\x91a/{V[a\x1D\xE7\x91a\x1C\xC3V[a\x1D\xF0\x90a,ZV[\x81a\x1D\xFB\x8A\x86a/{V[\x90a\x1E\x05\x91a/{V[a\x1E\x0E\x91a/{V[\x91a\x1E\x19\x89\x82a\x1C\xC3V[\x91a\x1E#\x91a/{V[a\x1E,\x91a\x17\xB8V[a\x1E5\x91a/\xF8V[\x96a\x1E?\x91a/{V[\x90a\x1EI\x90a\x1C\xA9V[a\x1ER\x91a/{V[\x92a\x1E\\\x91a/{V[a\x1Ee\x90a/\xD1V[\x90a\x1Eo\x91a\x17\xB8V[a\x1Ex\x90a1gV[a\x1E\x81\x91a/{V[\x90a\x1E\x8B\x90a.\xC9V[a\x1E\x94\x91a/\xF8V[\x91a\x1E\x9E\x90a\x1C\xDFV[\x90a\x1E\xA8\x91a\x1C\xC3V[\x90a\x02\xAA\x91a\x1C\xC3V[\x91\x90\x91a\x1E\xF0` \x83\x01\x91a\x1E\xEAa\x1E\xE2\x84Qa\x18\xC6a\x1E\xD8`@\x89\x01\x92\x83Q\x90a+\x86V[\x96Q\x82Q\x90a+\xAFV[\x95\x85Qa'WV[\x90a'xV[g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x82\x12\x15a\x19RW`\0\x82\x13\x15a\x19GWa\x02\xAA\x94a\x19@\x93a\x1F&a\x19;\x93a\x19#a\x14\xC8\x96a+\xCEV[\x05a\x1C\xC3V[\x90\x92\x91\x85Q` \x87\x01Q`@\x88\x01Q``\x89\x01Q\x91a\x1FJ\x82a.\x0BV[a\x1FS\x90a\x1BpV[\x92a\x1F]\x90a\x17YV[\x93a\x1Fg\x86a.\xC9V[a\x1Fp\x87a\x1C\x97V[a\x1Fy\x91a/{V[a\x1F\x83\x89\x83a/{V[a\x1F\x8C\x88a\x1C\x97V[a\x1F\x95\x91a/{V[a\x1F\x9E\x90a\x1C\xDFV[\x86a\x1F\xA9\x8B\x85a/{V[a\x1F\xB2\x90a/!V[\x90a\x1F\xBC\x91a/{V[a\x1F\xC5\x91a\x1C\xC3V[a\x1F\xCE\x91a/\xF8V[a\x1F\xD7\x90a\x1C\xDFV[a\x1F\xE0\x90a0\x16V[\x92a\x1F\xEA\x83a2\xFEV[a\x1F\xF3\x90a\x17\xEEV[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x05a \x07\x90a,ZV[\x90a \x11\x91a/{V[a \x1A\x90a/\xAAV[a #\x90a\x1C\xDFV[\x83\x85a .\x85a/LV[\x90a 8\x91a/{V[\x90a B\x91a/{V[a K\x91a\x1C\xC3V[a T\x90a,ZV[\x85a _\x88\x8Aa/{V[\x90a i\x91a/{V[a r\x91a/{V[\x90a |\x87a\x1C\x97V[a \x85\x87a.\xF8V[a \x8E\x91a\x17\xB8V[a \x97\x91a/{V[a \xA0\x91a/\xF8V[\x93a \xAB\x87\x89a/{V[\x90a \xB5\x90a\x1C\xA9V[a \xBE\x91a/{V[\x92a \xC8\x91a/{V[a \xD1\x90a/\xD1V[\x90a \xDB\x91a\x17\xB8V[a \xE4\x90a1gV[a \xED\x91a/{V[a \xF6\x83a.\xC9V[a \xFF\x91a/\xF8V[\x90a!\t\x90a\x1C\xA9V[\x90a!\x13\x91a\x1C\xC3V[`\0\x13a\x1BeWa\x02\xAA\x95a!5\x93a\x15,\x92`@Q\x96\x87\x95` \x87\x01a\x19\\V[a*\xAFV[\x91a\x14\xC3a\x14\xC8\x91a!K\x93a2\x87V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x03\x90\x81\x11a\x0EOWa\x02\xAA\x91a(\xE5V[\x92\x93`\0\x93\x85\x92\x91\x85\x85\x12\x15a\"\x8AW[\x85\x85\x12a\"kW\x90a\x15,a!\x99\x92`@\x96`@Q\x95\x86\x94` \x86\x01a\x14\xD4V[\x81\x85\x92\x85\x96\x82\x81\x11a\x16;Wa!\xAF\x81\x85a6\xE3V[\x92a!\xBA\x81\x86a6\xE3V[\x88a!\xC5\x82\x87a\x18EV[\x13a\x16\x1AWP\x90a!\xD9\x91\x97\x96\x92\x97a\x0E\xC8V[`\x01\x95\x91\x82\x91\x87\x80[a!\xF3WPPPPPPPPPP\x90V[\x15a\"NW[P\x86\x97\x98P\x81\x92a\"\ra\x15\xAF\x8B\x89a\x0ETV[\x99a\"\x18\x8B\x88a6\xE3V[\x90\x84a\"$\x88\x84a\x18EV[\x13a\"BWPP\x89\x93[\x88a\"9\x89\x87a\x0E\xC8V[\x92\x01\x94\x99a!\xE2V[\x8B\x98P\x90\x95P\x93a\".V[`\x14\x10\x80a\"bW[\x15a\x16\nW\x88a!\xF9V[P\x80\x83\x10a\"WV[\x93P\x91a\"w\x90a)\x11V[\x91a\"\x84\x84\x83\x83\x86a'\xECV[\x93a!xV[\x85\x85\x13a\"\x9EW\x90a\x15,a!\x99\x92a\x15\x1AV[\x93P\x94a\"\xAA\x90a'8V[\x94a\"\xB7\x84\x83\x83\x89a'\xECV[\x93a\"\x8AV[\x92\x91\x90a\"\xD3a\"\xCD\x82\x84a);V[\x85a(\xE5V[\x93\x81\x03\x90\x81\x11a\x0EOW\x92\x81\x03\x90\x81\x11a\x0EOW\x90V[\x92\x91\x90a\"\xFAa\"\xCD\x82\x84a);V[\x93\x81\x01\x80\x91\x11a\x0EOW\x92\x81\x01\x80\x91\x11a\x0EOW\x90V[\x93\x90\x92\x91\x81Q` \x83\x01Q\x91`@\x84\x01Q\x93``\x01Q\x94a#1\x85a.\x0BV[a#:\x90a\x1BpV[\x95a#D\x90a\x17YV[\x92\x83a#O\x89a.\xC9V[a#Y\x8A\x85a\x1C\xC3V[a#b\x91a/{V[a#l\x85\x84a/{V[a#v\x8B\x86a\x1C\xC3V[a#\x7F\x91a/{V[a#\x88\x90a\x1C\xDFV[\x82\x85a#\x94\x88\x87a/{V[\x90a#\x9E\x91a/{V[\x90a#\xA8\x91a/{V[a#\xB1\x91a\x1C\xC3V[a#\xBA\x91a/\xF8V[a#\xC3\x90a\x1C\xDFV[a#\xCC\x90a0\x16V[\x96a#\xD6\x87a2\xFEV[a#\xDF\x90a\x17\xEEV[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x05a#\xF3\x90a,ZV[\x90a#\xFD\x91a/{V[a$\x06\x90a/\xAAV[a$\x0F\x90a\x1C\xDFV[\x87\x89a$\x1A\x89a/LV[\x90a$$\x91a/{V[\x90a$.\x91a/{V[a$7\x91a\x1C\xC3V[a$@\x90a,ZV[\x81a$K\x8B\x8Da/{V[\x90a$U\x91a/{V[a$^\x91a/{V[\x92a$i\x8A\x82a\x1C\xC3V[\x91a$s\x91a/{V[a$|\x91a\x17\xB8V[a$\x85\x91a/{V[a$\x8E\x91a/\xF8V[\x96a$\x98\x91a/{V[\x90a$\xA2\x90a\x1C\xA9V[a$\xAB\x91a/{V[\x92a$\xB5\x91a/{V[a$\xBE\x90a/\xD1V[\x90a$\xC8\x91a\x17\xB8V[a$\xD1\x90a1gV[a$\xDA\x91a/{V[\x90a$\xE4\x90a.\xC9V[a$\xED\x91a/\xF8V[\x90a\x1E\xA8\x90a\x1C\xA9V[\x92\x93`\0\x93\x85\x92\x91\x85\x85\x12\x15a&\x1AW[\x85\x85\x12a%\xFBW\x90a\x15,a%)\x92`@\x96`@Q\x95\x86\x94` \x86\x01a\x14\xD4V[\x81\x85\x92\x85\x96\x82\x81\x11a\x16;Wa%?\x81\x85a7\x05V[\x92a%J\x81\x86a7\x05V[\x88a%U\x82\x87a\x18EV[\x13a\x16\x1AWP\x90a%i\x91\x97\x96\x92\x97a\x0E\xC8V[`\x01\x95\x91\x82\x91\x87\x80[a%\x83WPPPPPPPPPP\x90V[\x15a%\xDEW[P\x86\x97\x98P\x81\x92a%\x9Da\x15\xAF\x8B\x89a\x0ETV[\x99a%\xA8\x8B\x88a7\x05V[\x90\x84a%\xB4\x88\x84a\x18EV[\x13a%\xD2WPP\x89\x93[\x88a%\xC9\x89\x87a\x0E\xC8V[\x92\x01\x94\x99a%rV[\x8B\x98P\x90\x95P\x93a%\xBEV[`\x14\x10\x80a%\xF2W[\x15a\x16\nW\x88a%\x89V[P\x80\x83\x10a%\xE7V[\x93P\x94a&\x07\x90a'8V[\x94a&\x14\x84\x87\x84\x84a'\xECV[\x93a%\x08V[\x85\x85\x13a&.W\x90a\x15,a%)\x92a\x15\x1AV[\x93P\x91a&:\x90a)\x11V[\x91a&G\x84\x84\x84\x84a'\xECV[\x93a&\x1AV[a&\xBDa&\xB8a\x02\xAA\x93a&\xB2a&\xAD\x82Qa&\xA8g\r\xE0\xB6\xB3\xA7d\0\0a&\xA2a&\x9Da&\x97a&\x92`@` \x8B\x01Q\x9A\x01Q\x96a&\x8C\x88\x8Ca+\x86V[\x9Da);V[a2\xFEV[\x97a2\xFEV[a\x17\xEEV[\x05a,ZV[a'WV[a'\x9AV[\x90a\x17\xB8V[a\x17\xD1V[a\x18hV[a&\xF1a&\xB8a&\xECg\x13\xA0K\xBD\xFD\xC9\xBE\x88a&\xE6g\x1B\xC1mgN\xC8\0\0\x95a\x17\xD1V[\x05a\x1C\xDFV[a1gV[\x05\x90V[`\0\x81\x12a'\0W\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01RotoUint: negative`\x80\x1B`D\x82\x01R`d\x90\xFD[a\x03\xE7\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x04?Wa\x03\xE8\x90\x04\x90V[\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x04?Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x04\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17\x82\x15\x15\x16\x15a\x04?W\x04\x90V[g\x06\xF0[Y\xD3\xB2\0\0\x90\x80\x82\x02\x91\x82\x04\x14`\x01\x16\x15a\x04?Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x04\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x04?Wg\x1B\xC1mgN\xC8\0\0\x90\x04\x90V[\x90\x92\x82\x82\x10\x15a(\xA0Wa\x02\xAA\x93a(i\x92\x84g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82a(\x14\x83\x83a'xV[\x10a(\x8DWP`\x01`\x01`\xFF\x1B\x03\x95\x90P[\x83Q\x91a(<a(6\x83\x85a'WV[\x85a'xV[\x10a(nWP`\x01`\x01`\xFF\x1B\x03\x92a(c\x92P\x90P[`@` \x82\x01Q\x91\x01Q\x90a+\x86V[\x92a\x1C\xC3V[a\x1C\xC3V[a(c\x92a\x1E\xEAa(\x82\x92a(\x87\x94a'WV[a+\xCEV[\x91a(SV[a(\x9A\x91a(\x82\x91a'xV[\x94a(&V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FtradingFunction: invalid x\0\0\0\0\0\0`D\x82\x01R`d\x90\xFD[\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x04?W`\x01g\r\xE0\xB6\xB3\xA7d\0\0`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[a\x03\xE9\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x04?W`\x01a\x03\xE8`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[\x90g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x81\x02\x92\x81\x84\x04\x14\x90\x15\x17\x81\x15\x15\x16\x15a\x04?W`\x01\x90`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[a\x01\0\x81\x83\x03\x12a\x03\0W\x80Q\x92a\x02\xAA` \x83\x01Q\x93```@\x85\x01Q\x94\x01a\x11\xC8V[\x91\x90a\x03\xE8\x92`\0\x93`\0\x91\x83\x82\x11a*\x8EWa)\xAC\x81a7%V[a)\xB6\x85\x83a8dV[`\0a)\xC2\x82\x84a\x18EV[\x13a*oWPa)\xD3\x85\x96\x95a\x0E\xB8V[`\x01\x94`\0\x91\x86\x80[a)\xEDW[PPPPPPPP\x90PV[\x15a*JW[P\x85\x96\x97\x98P\x80\x91a*\x08a\x15\xAF\x8B\x88a\x0ETV[\x99a*\x13\x8B\x87a8dV[\x90\x83a*\x1F\x87\x84a\x18EV[\x13a*>WPP\x89\x92[\x87a*4\x88\x86a\x0E\xC8V[\x92\x01\x93\x99\x98a)\xDCV[\x8B\x97P\x90\x94P\x92a*)V[\x86\x10\x80a*dW[\x15a*]W\x88a)\xF3V[\x80\x80a)\xE1V[Pa\x01\0\x82\x10a*RV[`@Qc\x06\xF1\xBE]`\xE2\x1B\x81R`\x04\x81\x01\x92\x90\x92R`$\x82\x01R`D\x90\xFD[`@Qc0\x82\xDF\xDB`\xE1\x1B\x81Ra\x03\xE8`\x04\x82\x01R`$\x81\x01\x85\x90R`D\x90\xFD[\x91\x90a\x03\xE8\x92`\0\x93`\0\x91\x83\x82\x11a*\x8EWa*\xCB\x81a8\x86V[a*\xD5\x85\x83a9\xF6V[`\0a*\xE1\x82\x84a\x18EV[\x13a*oWPa*\xF2\x85\x96\x95a\x0E\xB8V[`\x01\x94`\0\x91\x86\x80[a+\x0BWPPPPPPPP\x90PV[\x15a+hW[P\x85\x96\x97\x98P\x80\x91a+&a\x15\xAF\x8B\x88a\x0ETV[\x99a+1\x8B\x87a9\xF6V[\x90\x83a+=\x87\x84a\x18EV[\x13a+\\WPP\x89\x92[\x87a+R\x88\x86a\x0E\xC8V[\x92\x01\x93\x99\x98a*\xFBV[\x8B\x97P\x90\x94P\x92a+GV[\x86\x10\x80a+{W[\x15a*]W\x88a+\x11V[Pa\x01\0\x82\x10a+pV[\x90a+\x90\x90a.\x0BV[c;\x9A\xCA\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17\x15a\x0EOWa\x02\xAA\x91a'WV[a\x02\xAA\x91a&\xA8g\r\xE0\xB6\xB3\xA7d\0\0a&\xA2a&\x9Da&\xAD\x95a2\xFEV[g\x06\xF0[Y\xD3\xB2\0\0\x81\x14a,TWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x12\x15a,BW\x80\x15a,0W\x80`\x01\x1B\x90\x81\x05`\x02\x03a\x0EOWa,\x0C\x90a0\x16V[\x90g\x13\xA0K\xBD\xFD\xC9\xBE\x88\x91\x80\x83\x02\x92\x83\x05\x14`\x01\x16\x15a\x04?Wa\x02\xAA\x91\x05a\x1C\xDFV[`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x90\xFD[`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x90\xFD[P`\0\x90V[h\x02H\xCE6\xA7\x0C\xB2k>\x19\x81\x13\x15a,TWh\x07U\xBFy\x8BJ\x1B\xF1\xE5\x81\x12\x15a-\xAAWe\x03x-\xAC\xE9\xD9\x90`N\x1B\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x91``\x90`\x01`_\x1B\x84\x82\x84\x1B\x05\x01\x82\x1D\x93\x84\x02\x90\x03\x80l\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x81\x01\x02\x82\x1D\x90n\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dPn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x82n\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x81m\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x81m\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x81m\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x81l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x02\x8D\x1D\x01\x02\x8B\x1D\x01\x02\x89\x1D\x01\x02\x87\x1D\x01\x02\x85\x1D\x01\x93m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x93m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEA\x81\x01\x90\x84m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x91\x01\x01\x02\x90\x1D\x01\x02\x01\x05\x02\x90`\xC3\x03\x1C\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x90\xFD[\x91\x90a\x01 \x83\x82\x03\x12a\x03\0W\x82Q\x92` \x81\x01Q\x92a\x02\xAA`@\x83\x01Q\x93`\x80``\x85\x01Q\x94\x01a\x11\xC8V[`\xB5\x81`\x01`\x88\x1B\x81\x10\x15a.\xB2W[\x80i\x01\0\0\0\0\0\0\0\0\0b\x01\0\0\x92\x10\x15a.\xA5W[e\x01\0\0\0\0\0\x81\x10\x15a.\x98W[c\x01\0\0\0\x81\x10\x15a.\x8BW[\x01\x02`\x12\x1C`\x01\x90\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x80\x80\x92\x04\x10\x90\x03\x90V[`\x10\x1C\x91`\x08\x1B\x91a.OV[` \x1C\x91`\x10\x1B\x91a.BV[`@\x1C\x91` \x1B\x91a.3V[Ph\xB5\0\0\0\0\0\0\0\0\x90P`\x80\x82\x90\x1Ca.\x1BV[g\x1B\xC1mgN\xC8\0\0\x81\x81\x02\x91`\x01`\xFF\x1B\x81\x13`\x01\x17\x91\x83\x05\x14\x16\x15a\x04?Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[a\x03\xE8\x81\x81\x02\x91`\x01`\xFF\x1B\x81\x13`\x01\x17\x91\x83\x05\x14\x16\x15a\x04?Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[a\x03\xE8\x80\x82\x02\x91`\x01`\0\x19\x82\x10\x17\x91\x81\x84\x05\x14\x90\x15\x17\x16\x15a\x04?Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\x13\xA0K\xBD\xE7\x8C\xC4\0\x81\x81\x02\x91`\x01`\xFF\x1B\x81\x13`\x01\x17\x91\x83\x05\x14\x16\x15a\x04?Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[\x81\x81\x02\x91\x81\x15\x82\x84\x05\x82\x14\x17`\0\x19\x90\x92\x10`\x01`\xFF\x1B\x90\x91\x13\x17\x16\x15a\x04?Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x82\x05\x14`\x01\x16\x15a\x04?Wg\x1B\xC1mgN\xC8\0\0\x90\x05\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x82\x05\x14`\x01\x16\x15a\x04?Wg\x13\xA0K\xBD\xE7\x8C\xC4\0\x90\x05\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x82\x05\x14\x82\x15\x15\x16\x15a\x04?W\x05\x90V[`\0\x81\x12\x80\x15a1VW[a1DW\x80\x15a,BWg\x1B\xC1mgN\xC8\0\0\x81\x14a,0Wg\r\xE0\xB6\xB3\xA7d\0\0\x81\x12\x90\x81\x15a15W\x90[a0W\x82a4\xCCV[\x80\x15a,BWa0\xC0a0\x84a0\x7Fa0za0ua0\xC5\x95a2\xFEV[a5\x8DV[a.\x0BV[a\x18\x0BV[a\x196a0\x98a0\x93\x83a4\xF7V[a\x1B\x89V[a0\xBAa0\xB5a0\xAFa0\xAA\x86a5\"V[a\x1B\xA1V[\x85a6\x04V[a\x1B\xB9V[\x90a5kV[a5\xB5V[\x91`\0\x90[`\x02\x82\x10a0\xE5WPP\x15a0\xDCW\x90V[a\x02\xAA\x90a\x1C\xDFV[\x90\x92a1-\x81a1'a0\xFD\x85a\x196`\x01\x96a1gV[a0\xBAa1\x1Da1\x18a\x19;a1\x13\x87\x80a6\x04V[a\x1C\xDFV[a5\xDDV[a&\xB2\x83\x86a6\x04V[\x90a\x1C\xC3V[\x93\x01\x90a0\xCAV[a1>\x90a\x17|V[\x90a0NV[`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x90\xFD[Pg\x1B\xC1mgN\xC8\0\0\x81\x13a0!V[\x80\x15a2zWgV\x98\xEE\xF0fp\0\0\x81\x12\x15a,TWgV\x98\xEE\xF0fo\xFF\xFF\x19\x81\x13\x15a2mW`\0a2]a1\x9C\x83a4\x9FV[a2%a\x19;a1\xB6a1\xB1a0\xB5\x85a'\xC1V[a5LV[\x92a(ia2Xa2Sa2La2Fa2Aa2;a26a20a2+\x8Da2%a2 a2\x1Aa2\x15a0\xAFa2\x10a2\na2\x05a1\xFFa1\xFA\x8Aa6%V[a\x1B\xD1V[\x89a6\x04V[a\x1B\xEBV[\x87a6\x04V[a\x1C\x03V[a\x1C\x1DV[\x83a6\x04V[a\x1C5V[\x90a6\x04V[a\x1COV[\x8Ca6\x04V[a\x1CgV[\x8Aa6\x04V[a\x1C\x7FV[\x88a6\x04V[\x93\x80a6\x04V[a\x18$V[a\x17\x9FV[\x91\x12\x15a\x02\xAAWa\x02\xAA\x90a\x17|V[Pg\x1B\xC1mgN\xC8\0\0\x90V[Pg\r\xE0\xB6\xB3\xA7d\0\0\x90V[a&\xBDa&\xB8a\x02\xAA\x93a1'a&\xAD\x82Qa&\xA8g\r\xE0\xB6\xB3\xA7d\0\0a&\xA2a&\x9Da&\x97a&\x92`@` \x8B\x01Q\x9A\x01Q\x96a&\x8C\x88\x8Ca+\x86V[\x15a2\xCDWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x90\xFD[}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x90a3*`\0\x82\x13a2\xC6V[q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06a3F\x82a6PV[``\x92\x83\x82`\x9F\x03\x01\x1B`\x9F\x1C\x90`_\x19\x01}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x02\x92l\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x82m\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x81m\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x81m\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x81m\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x81m\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x81lFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x02\x8C\x1D\x01\x02\x8A\x1D\x01\x02\x88\x1D\x01\x02\x86\x1D\x01\x02\x84\x1D\x01\x02\x82\x1D\x01\x91x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x91l\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x90\x82m\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x81l\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x81m\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x81m\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x81l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x81\x01\x02\x89\x1D\x01\x02\x87\x1D\x01\x02\x85\x1D\x01\x02\x83\x1D\x01\x02\x90\x1D\x01\x02\x01\x05\x02\x01\x01`\xAE\x1D\x90V[`\x01`\xFF\x1B\x81\x14a4\xBAW`\0\x81\x12\x15a\x02\xAAW\x19`\x01\x01\x90V[`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x90\xFD[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x04?Wg\x1B\xC1mgN\xC8\0\0\x90\x05\x90V[g\x03\xC1f\\z\xAB \0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x04?Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[f\x9F2u$b\xA0\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x04?Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[`\x01\x81\x15\x15\x16\x15a\x04?Wn\xC0\x97\xCE{\xC9\x07\x15\xB3K\x9F\x10\0\0\0\0\x05\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17\x82\x15\x15\x16\x15a\x04?W\x05\x90V[g\x1B\xC1mgN\xC7\xFF\xFF\x19\x81\x81\x02\x91`\x01\x91\x83\x05\x14\x16\x15a\x04?Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\t\xD0(\xCCo _\xFF\x19\x81\x81\x02\x91`\x01\x91\x83\x05\x14\x16\x15a\x04?Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x90\x80\x82\x02\x91\x82\x05\x14`\x01\x16\x15a\x04?Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x04?Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\x02_\x0F\xE1\x05\xA3\x14\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x04?Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[a6[\x81\x15\x15a2\xC6V[\x80o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x07\x1B\x81\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x81\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x81\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x81\x81\x1C`\xFF\x10`\x03\x1B\x17\x81\x81\x1C`\x0F\x10`\x02\x1B\x17\x81\x81\x1C`\x03\x10`\x01\x1B\x17\x90\x81\x1C`\x01\x10\x17\x90V[\x90a6\xD9a\x02\xAA\x92` \x80\x82Q\x83\x01\x01\x91\x01a)kV[\x93\x91\x92\x90Pa'\xECV[\x90a6\xFAa\x02\xAA\x92` \x80\x82Q\x83\x01\x01\x91\x01a)kV[\x93\x90P\x91\x90\x91a'\xECV[\x90a7\x1Ca\x02\xAA\x92` \x80\x82Q\x83\x01\x01\x91\x01a)kV[\x93\x92\x90Pa'\xECV[\x80Q\x81\x01` \x01\x90` \x01\x90a7:\x91a-\xDEV[\x80\x91\x93\x92PQ\x90` \x81\x01Q`@\x82\x01Q\x91``\x01Q\x92a7Z\x83a.\x0BV[a7c\x90a\x1BpV[\x93a7m\x90a\x17YV[\x90a7w\x86a.\xC9V[a7\x80\x87a\x1C\x97V[a7\x89\x91a/{V[a7\x92\x87a\x1C\x97V[a7\x9B\x84a.\xF8V[a7\xA4\x91a\x17\xB8V[a7\xAE\x90\x89a/{V[a7\xB7\x91a/\xF8V[a7\xC0\x90a0\x16V[\x93a7\xCA\x84a2\xFEV[a7\xD3\x90a\x17\xEEV[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x05a7\xE7\x90a,ZV[\x90a7\xF1\x91a/{V[a7\xFA\x90a/\xAAV[a8\x03\x90a\x1C\xDFV[\x84\x86a8\x0E\x86a/LV[\x90a8\x18\x91a/{V[\x90a8\"\x91a/{V[a8+\x91a\x1C\xC3V[a84\x90a,ZV[\x82a8?\x88\x84a/{V[\x90a8I\x91a/{V[a8R\x91a/{V[a8[\x87a\x1C\x97V[a\x1E#\x84a.\xF8V[\x90a8{a\x02\xAA\x92` \x80\x82Q\x83\x01\x01\x91\x01a-\xDEV[\x94\x93\x90\x92\x91Pa\x1C\xF0V[\x80Q\x81\x01` \x01\x90` \x01\x90a8\x9B\x91a-\xDEV[\x80\x91\x92\x93PQ` \x82\x01Q\x90`@\x83\x01Q\x92``\x01Q\x93a8\xBB\x84a.\x0BV[a8\xC4\x90a\x1BpV[\x94a8\xCE\x90a\x17YV[\x91a8\xD8\x87a.\xC9V[a8\xE1\x88a\x1C\x97V[a8\xEA\x91a/{V[a8\xF4\x83\x83a/{V[a8\xFD\x89a\x1C\x97V[a9\x06\x91a/{V[a9\x0F\x90a\x1C\xDFV[\x84a9\x1A\x85\x85a/{V[a9#\x90a/!V[\x90a9-\x91a/{V[a96\x91a\x1C\xC3V[a9?\x91a/\xF8V[a9H\x90a\x1C\xDFV[a9Q\x90a0\x16V[\x94a9[\x85a2\xFEV[a9d\x90a\x17\xEEV[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x05a9x\x90a,ZV[\x90a9\x82\x91a/{V[a9\x8B\x90a/\xAAV[a9\x94\x90a\x1C\xDFV[\x85\x87a9\x9F\x87a/LV[\x90a9\xA9\x91a/{V[\x90a9\xB3\x91a/{V[a9\xBC\x91a\x1C\xC3V[a9\xC5\x90a,ZV[\x83a9\xD0\x89\x8Ba/{V[\x90a9\xDA\x91a/{V[a9\xE3\x91a/{V[\x90a9\xED\x88a\x1C\x97V[a$s\x85a.\xF8V[\x90a:\ra\x02\xAA\x92` \x80\x82Q\x83\x01\x01\x91\x01a-\xDEV[\x94\x93\x90\x92Pa#\x11V\xFE\xA2dipfsX\"\x12 \x9F\xC8\xD6(\xD4d\x1Cp\x07\xB3\x1E\x8A\x8C\x8A\xD0\x8B9])g\xF2\x8B\xA9\x9CtUk\x08\r\x995mdsolcC\0\x08\x16\x003";
    /// The bytecode of the contract.
    pub static LOGNORMALSOLVER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\x0C\x1CW`\x005`\xE0\x1C\x80c\x04 X\n\x14a\x01\xACW\x80c\x12\x06I\xC5\x14a\x01\xA7W\x80c\x13N\xAD\x12\x14a\x01\xA2W\x80c\x1E\x97\x8C\xB0\x14a\x01\x9DW\x80c0m\xB4k\x14a\x01\x98W\x80c3\"f\xF3\x14a\x01\x93W\x80c9(\xFF\x97\x14a\x01\x8EW\x80c;&\x8D]\x14a\x01\x89W\x80c;M\x100\x14a\x01\x84W\x80cN\x81\x7F\xD9\x14a\x01\x7FW\x80cO\xD6|X\x14a\x01zW\x80c^\xB4\x08\xFC\x14a\x01uW\x80cb7V\x9F\x14a\x01pW\x80cme\"\x99\x14a\x01kW\x80c\x7F\x17@\x9C\x14a\x01fW\x80c\x81\xB5\xFA\xC2\x14a\x01aW\x80c\x90.\xCA\xA2\x14a\x01\\W\x80c\xA8\xC6.v\x14a\x01WW\x80c\xAFNC\x7F\x14a\x01RW\x80c\xB0\x9D\x04\xE5\x14a\x01MW\x80c\xCB\x1FU2\x14a\x01HW\x80c\xCE\x15;\xF4\x14a\x01CW\x80c\xE9G\x16\xD5\x14a\x01>W\x80c\xEE>\x8C\xFB\x14a\x019W\x80c\xF3\r7\xF2\x14a\x014Wc\xF9\xC2\x82\x11\x03a\x0C\x1CWa\x0C\0V[a\x0B\xD0V[a\x0B\x9FV[a\x0BdV[a\x0B(V[a\n\xE3V[a\n\xB0V[a\n\x94V[a\nkV[a\nBV[a\n\x15V[a\tsV[a\tWV[a\x08\xEAV[a\x08\xCEV[a\x08\xA5V[a\x08\x89V[a\x08ZV[a\x08\x1FV[a\x05\x82V[a\x05+V[a\x04\xFCV[a\x04\xD7V[a\x04DV[a\x03(V[a\x02\xADV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01Ra7\xB7`\xF1\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[`\0[\x83\x81\x10a\x02dWPP`\0\x91\x01RV[\x81\x81\x01Q\x83\x82\x01R` \x01a\x02TV[\x90` \x91a\x02\x8D\x81Q\x80\x92\x81\x85R\x85\x80\x86\x01\x91\x01a\x02QV[`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[\x90` a\x02\xAA\x92\x81\x81R\x01\x90a\x02tV[\x90V[4a\x03\x05W`@6`\x03\x19\x01\x12a\x03\0Wa\x02\xFC`@Q`\x02` \x82\x01R`\x045`@\x82\x01R`$5``\x82\x01R``\x81Ra\x02\xE8\x81a\x03\xD4V[`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x02tV[\x03\x90\xF3[a\x02\x01V[a\x01\xB1V[`\x80\x90`\x03\x19\x01\x12a\x03\0W`\x045\x90`$5\x90`D5\x90`d5\x90V[4a\x03\x05W` a\x03Da\x03;6a\x03\nV[\x92\x91\x90\x91a\r\x04V[`@Q\x90\x81R\xF3[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FABI decoding: struct data too sh`D\x82\x01Rb\x1B\xDC\x9D`\xEA\x1B`d\x82\x01R`\x84\x90\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\xA0\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x03\xCFW`@RV[a\x03\x9DV[`\x80\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x03\xCFW`@RV[``\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x03\xCFW`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x03\xCFW`@RV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\x04?WV[`\0\x80\xFD[4a\x03\x05W`\xE06`\x03\x19\x01\x12a\x03\0W`\xA06`C\x19\x01\x12a\x04\xB8Wa\x02\xFCa\x04\xAC`@Qa\x04s\x81a\x03\xB3V[`D5\x81R`d5` \x82\x01R`\x845`@\x82\x01R`\xA45``\x82\x01R`\xC45a\x04\x9C\x81a\x04.V[`\x80\x82\x01R`$5`\x045a\x16\xB0V[`@Q\x91\x82\x91\x82a\x02\x99V[a\x03LV[``\x90`\x03\x19\x01\x12a\x03\0W`\x045\x90`$5\x90`D5\x90V[4a\x03\x05W` a\x03Da\x04\xF6a\x04\xED6a\x04\xBDV[\x91\x92\x90\x92a\x12+V[\x91a\x18\x99V[4a\x03\x05W` a\x03Da\x05\x0F6a\x04\xBDV[\x90a\x05\"a\x05\x1C\x84a\x12+V[\x93a\x13\xB2V[\x92\x91\x90\x91a\x19\x86V[4a\x03\x05W` a\x03Da\x05>6a\x04\xBDV[\x90a\x05Ka\x05\x1C\x84a\x12+V[\x92\x90Pa\x1C\xF0V[\x80\x15\x15\x03a\x04?WV[\x90\x92`\x80\x92a\x02\xAA\x95\x94\x15\x15\x83R` \x83\x01R`@\x82\x01R\x81``\x82\x01R\x01\x90a\x02tV[4a\x03\x05W``6`\x03\x19\x01\x12a\x03\0W`$5`\x045a\x05\xA2\x82a\x05SV[`D5\x90a\x05\xF5a\x05\xB1a\r\xFBV[\x92a\x05\xBAa\r\xFBV[\x93a\x05\xC4\x84a\x13\xB2V[` \x84\x99\x93\x95\x92\x99\x01\x94`@\x99\x8A\x86\x01\x92\x83R\x86R\x84Ra\x05\xE4\x87a\x12+V[\x95\x86\x91\x85Q\x90\x87Q\x90Q\x91\x8Aa\x12\xD9V[\x92\x15a\x07\x97W\x92\x82a\x06<a\x06C\x93a\x065a\x060a\x06(a\x06a\x98a\x06#``a\x06\x89\x9D\x9C\x01Q\x86a(\xE5V[a(\xE5V[\x86Q\x90a);V[a\x0EAV[\x93Qa\x0ETV[\x89Ra\x0ETV[a\x06U\x88\x88\x01\x91\x80\x83R\x88Q\x88a\r\xEEV[\x90\x87Q\x90Q\x90\x87a\r\x04V[\x90a\x06\x80a\x06u` \x88\x01\x93\x80\x85Ra\x0EAV[\x80\x84R\x82Q\x11a\x0E\xD5V[Q\x90Q\x90a\x0E\xC8V[\x93[\x83Q\x91` \x85\x01Q\x92a\x06\xCD\x83\x87\x01\x91a\x06\xBF\x83Q\x86Q\x97\x88\x93` \x85\x01`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[\x03`\x1F\x19\x81\x01\x86R\x85a\x04\x0CV[`\0Ta\x06\xF0\x90a\x06\xE4\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x80;\x15a\x07\x92W\x84`\xC0\x91a\x07\x1B\x97\x86Q\x80\x99\x81\x94\x82\x93c\r\x17\xA7\xC7`\xE3\x1B\x84R0`\x04\x85\x01a\x0FcV[\x03\x91Z\xFA\x94\x85\x15a\x07\x8DW`\0\x95a\x07MW[P\x90a\x07B\x91a\x02\xFC\x95\x96Q\x90Q\x90a\x18\x99V[\x90Q\x94\x85\x94\x85a\x05]V[a\x02\xFC\x95P\x90a\x07xa\x07B\x93\x92`\xC0=`\xC0\x11a\x07\x86W[a\x07p\x81\x83a\x04\x0CV[\x81\x01\x90a\x0F,V[PPPPP\x95P\x90\x91a\x07.V[P=a\x07fV[a\x0C\xF8V[a\x0C\x7FV[\x82a\x07\xE0a\x08\x19\x96a\x07\xD3a\x08\x05\x95a\x07\xCCa\x060a\x07\xC4a\x08\x10\x9Aa\x06#``a\x07\xFD\x9B\x01Q\x86a(\xE5V[\x85Q\x90a);V[\x92Qa\x0ETV[\x92` \x8C\x01\x93\x84Ra\x0ETV[a\x07\xF2\x8B\x8B\x01\x91\x80\x83R\x83Q\x8Ba\x0F\x87V[\x91Q\x90Q\x90\x89a\x0F\x94V[\x80\x88Ra\x0EAV[\x80\x87R\x82Q\x11a\x0EaV[Q\x84Q\x90a\x0E\xC8V[\x93a\x06\x8BV[4a\x03\x05W`@6`\x03\x19\x01\x12a\x03\0Wa\x02\xFC`@Q`\x04` \x82\x01R`\x045`@\x82\x01R`$5``\x82\x01R``\x81Ra\x02\xE8\x81a\x03\xD4V[4a\x03\x05W` 6`\x03\x19\x01\x12a\x03\0W` a\x03D`\x045a\x04\xF6a\x08\x7F\x82a\x13\xB2V[\x92\x91\x93\x90Pa\x12+V[4a\x03\x05W` a\x03Da\x08\x9Fa\x04\xED6a\x04\xBDV[\x91a\x1E\xB2V[4a\x03\x05W` a\x03Da\x08\xB86a\x04\xBDV[\x90a\x08\xC5a\x05\x1C\x84a\x12+V[\x92\x91\x90\x91a\x1F,V[4a\x03\x05W` a\x03Da\x08\xE16a\x03\nV[\x92\x91\x90\x91a\x0F\x94V[4a\x03\x05W`@6`\x03\x19\x01\x12a\x03\0Wa\t9`\x045a\x02\xFCa\t\x1Ba\t\x10\x83a\x13\xB2V[\x91\x90P`$5a\"\xBDV[\x93\x90\x92\x84\x84a\t3a\t,\x84a\x12+V[\x83\x83a\x18\x99V[\x92a\r\x04V[\x92`@Q\x93\x84\x93\x84`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[4a\x03\x05W`\x006`\x03\x19\x01\x12a\x03\0W` `@Q`\0\x81R\xF3[4a\x03\x05W`@6`\x03\x19\x01\x12a\x03\0W`\x045a\t\xC1a\x02\xFCa\t\xA3a\t\x99\x84a\x13\xB2V[\x91P`$5a\"\xEAV[\x92\x90\x93\x83\x85a\t\xBBa\t\xB4\x84a\x12+V[\x83\x83a\x1E\xB2V[\x92a\x0F\x94V[\x91`@Q\x93\x84\x93\x84`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[\x80Q\x82R` \x80\x82\x01Q\x90\x83\x01R`@\x80\x82\x01Q\x90\x83\x01R``\x80\x82\x01Q\x90\x83\x01R`\x80\x90\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x91\x01RV[4a\x03\x05W` 6`\x03\x19\x01\x12a\x03\0W`\xA0a\n3`\x045a\x12+V[a\n@`@Q\x80\x92a\t\xDFV[\xF3[4a\x03\x05W` a\x03Da\nU6a\x04\xBDV[\x90a\nba\x05\x1C\x84a\x12+V[\x92\x90\x91Pa#\x11V[4a\x03\x05W`\x006`\x03\x19\x01\x12a\x03\0W`\0T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x03\x05W` a\x03Da\n\xA76a\x03\nV[\x92\x91\x90\x91a\x12\xD9V[4a\x03\x05W` 6`\x03\x19\x01\x12a\x03\0Wa\x02\xFC`@Q`\x01` \x82\x01R`\x045`@\x82\x01R`@\x81Ra\x02\xE8\x81a\x03\xF0V[4a\x03\x05W` 6`\x03\x19\x01\x12a\x03\0Wa\x02\xFC`\x045a\x0B\x03\x81a\x04.V[`@\x80Q`\x05` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x82\x82\x01R\x81Ra\x02\xE8\x81a\x03\xF0V[4a\x03\x05W` 6`\x03\x19\x01\x12a\x03\0Wa\x02\xFCa\x0BG`\x045a\x13\xB2V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R\x90\x81\x90``\x82\x01\x90V[4a\x03\x05W`@6`\x03\x19\x01\x12a\x03\0Wa\x02\xFC`@Q`\x03` \x82\x01R`\x045`@\x82\x01R`$5``\x82\x01R``\x81Ra\x02\xE8\x81a\x03\xD4V[4a\x03\x05W`@6`\x03\x19\x01\x12a\x03\0Wa\t9`\x045a\x02\xFCa\t\x1Ba\x0B\xC5\x83a\x13\xB2V[\x91\x90P`$5a\"\xEAV[4a\x03\x05W`@6`\x03\x19\x01\x12a\x03\0W`\x045a\t\xC1a\x02\xFCa\t\xA3a\x0B\xF6\x84a\x13\xB2V[\x91P`$5a\"\xBDV[4a\x03\x05W`\x006`\x03\x19\x01\x12a\x03\0W` `@Q`x\x81R\xF3[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01Rtnor receive functions`X\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01Rd code`\xD8\x1B`d\x82\x01R`\x84\x90\xFD[\x90\x81` \x91\x03\x12a\x03\0WQ\x90V[`@\x90a\x02\xAA\x93\x92\x81R\x81` \x82\x01R\x01\x90a\x02tV[`@Q=`\0\x82>=\x90\xFD[\x90\x91\x92a\r\x1B\x90a\r\x14\x83a\x12+V[\x90\x85a\x14\xABV[\x90`@Q\x93a\rR\x85a\rD\x83\x86\x88` \x85\x01`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[\x03`\x1F\x19\x81\x01\x87R\x86a\x04\x0CV[`\0Ta\ri\x90a\x06\xE4\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x80;\x15a\x07\x92Wa\r\x92\x95` \x91`@Q\x80\x80\x99\x81\x94b.RK`\xE0\x1B\x83R\x88`\x04\x84\x01a\x0C\xE1V[\x03\x91Z\xFA\x91\x82\x15a\x07\x8DWa\x02\xAA\x95`\0\x93a\r\xB9W[Pa\r\xB3\x90a\x12+V[\x93a\x14\xF9V[a\r\xB3\x91\x93Pa\r\xE0\x90` =` \x11a\r\xE7W[a\r\xD8\x81\x83a\x04\x0CV[\x81\x01\x90a\x0C\xD2V[\x92\x90a\r\xA9V[P=a\r\xCEV[\x91a\x04\xF6a\x02\xAA\x93a\x12+V[`@Q\x90``\x82\x01\x82\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x03\xCFW`@R`\0`@\x83\x82\x81R\x82` \x82\x01R\x01RV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x90`\x01\x82\x01\x80\x92\x11a\x0EOWV[a\x0E+V[\x91\x90\x82\x01\x80\x92\x11a\x0EOWV[\x15a\x0EhWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: x reserve increase`D\x82\x01Rad!`\xF0\x1B`d\x82\x01R`\x84\x90\xFD[a\x03\xE7\x19\x81\x01\x91\x90\x82\x11a\x0EOWV[\x91\x90\x82\x03\x91\x82\x11a\x0EOWV[\x15a\x0E\xDCWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: y reserve increase`D\x82\x01Rad!`\xF0\x1B`d\x82\x01R`\x84\x90\xFD[\x91\x90\x82`\xC0\x91\x03\x12a\x03\0W\x81Qa\x0FC\x81a\x05SV[\x91` \x81\x01Q\x91`@\x82\x01Q\x91``\x81\x01Q\x91`\xA0`\x80\x83\x01Q\x92\x01Q\x90V[a\x02\xAA\x93\x92``\x92`\x01\x80`\xA0\x1B\x03\x16\x82R` \x82\x01R\x81`@\x82\x01R\x01\x90a\x02tV[\x91a\x08\x9Fa\x02\xAA\x93a\x12+V[\x90\x91\x92a\x0F\xAB\x90a\x0F\xA4\x83a\x12+V[\x90\x85a!:V[\x90`@Q\x93a\x0F\xD4\x85a\rD\x83\x87\x87` \x85\x01`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[`\0Ta\x0F\xEB\x90a\x06\xE4\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x80;\x15a\x07\x92Wa\x10\x14\x95` \x91`@Q\x80\x80\x99\x81\x94b.RK`\xE0\x1B\x83R\x88`\x04\x84\x01a\x0C\xE1V[\x03\x91Z\xFA\x91\x82\x15a\x07\x8DWa\x02\xAA\x95`\0\x93a\x10;W[Pa\x105\x90a\x12+V[\x93a!gV[a\x105\x91\x93Pa\x10Y\x90` =` \x11a\r\xE7Wa\r\xD8\x81\x83a\x04\x0CV[\x92\x90a\x10+V[\x91\x90` \x92\x83\x81\x83\x03\x12a\x03\0W\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x82\x82\x11a\x11xW\x01\x82`\x1F\x82\x01\x12\x15a\x11\x1FW\x80Q\x91\x82\x11a\x03\xCFW`@Q\x92a\x10\xAD`\x1F\x84\x01`\x1F\x19\x16\x87\x01\x85a\x04\x0CV[\x82\x84R\x85\x83\x83\x01\x01\x11a\x10\xCAW\x84\x83\x94\x95a\x02\xAA\x94\x01\x91\x01a\x02QV[`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x86\x90R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x86\x90R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x87\x90R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[\x91\x90\x82`\xA0\x91\x03\x12a\x04\xB8W`@Qa\x11\xE0\x81a\x03\xB3V[`\x80\x80\x82\x94\x80Q\x84R` \x81\x01Q` \x85\x01R`@\x81\x01Q`@\x85\x01R``\x81\x01Q``\x85\x01R\x01Q\x91a\x12\x13\x83a\x04.V[\x01RV[\x90`\xA0\x82\x82\x03\x12a\x03\0Wa\x02\xAA\x91a\x11\xC8V[`\x80\x90`@Qa\x12:\x81a\x03\xB3V[`\0\x92\x81\x84\x80\x93R\x82` \x82\x01R\x82`@\x82\x01R\x82``\x82\x01R\x01Ra\x12la\x06\xE4a\x06\xE4\x84T`\x01\x80`\xA0\x1B\x03\x16\x90V[\x80;\x15a\x07\x92W`@Qc\xDC\x17\x83U`\xE0\x1B\x81R`\x04\x81\x01\x92\x90\x92R\x82\x90\x82\x90`$\x90\x82\x90Z\xFA\x90\x81\x15a\x07\x8DW\x82a\x02\xAA\x93\x92a\x12\xB6W[PP` \x80\x82Q\x83\x01\x01\x91\x01a\x12\x17V[a\x12\xD2\x92P=\x80\x91\x83>a\x12\xCA\x81\x83a\x04\x0CV[\x81\x01\x90a\x10`V[8\x80a\x12\xA5V[`@\x80Q` \x81\x01\x84\x90R\x90\x81\x01\x84\x90R``\x81\x01\x85\x90R\x93\x91\x92a\x13\x01\x85`\x80\x81\x01a\rDV[`\0T`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x07\x92Wa\x136\x95` \x91`@Q\x80\x80\x99\x81\x94b.RK`\xE0\x1B\x83R\x88`\x04\x84\x01a\x0C\xE1V[\x03\x91Z\xFA\x91\x82\x15a\x07\x8DWa\x02\xAA\x95`\0\x93a\x13]W[Pa\x13W\x90a\x12+V[\x93a$\xF7V[a\x13W\x91\x93Pa\x13{\x90` =` \x11a\r\xE7Wa\r\xD8\x81\x83a\x04\x0CV[\x92\x90a\x13MV[\x90\x81` \x91\x03\x12a\x03\0WQa\x02\xAA\x81a\x04.V[\x90\x81``\x91\x03\x12a\x03\0W\x80Q\x91`@` \x83\x01Q\x92\x01Q\x90V[`\0T\x90\x91\x90a\x13\xCC\x90a\x06\xE4\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x80;\x15a\x07\x92W` `\x04\x91`@Q\x92\x83\x80\x92c+\xEE\x84\xF1`\xE2\x1B\x82RZ\xFA\x90\x81\x15a\x07\x8DW`\0\x91a\x14|W[P`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x07\x92W`@Qc3\x85N\xFD`\xE2\x1B\x81R`\x04\x81\x01\x93\x90\x93R``\x90\x83\x90`$\x90\x82\x90Z\xFA\x90\x81\x15a\x07\x8DW`\0\x80\x93`\0\x93a\x14EW[P\x92\x91\x90V[\x91\x93PPa\x14k\x91P``=``\x11a\x14uW[a\x14c\x81\x83a\x04\x0CV[\x81\x01\x90a\x13\x97V[\x92\x90\x92\x918a\x14?V[P=a\x14YV[a\x14\x9E\x91P` =` \x11a\x14\xA4W[a\x14\x96\x81\x83a\x04\x0CV[\x81\x01\x90a\x13\x82V[8a\x13\xFAV[P=a\x14\x8CV[a\x06#\x90a\x14\xCDa\x14\xC8a\x14\xC3\x86a\x02\xAA\x97\x96a&MV[a&\xC2V[a&\xF5V[\x92Qa(\xE5V[a\x14\xF7\x93``\x92\x96\x95\x93a\x01\0\x83\x01\x97\x83R` \x83\x01R`@\x82\x01R\x01\x90a\t\xDFV[V[\x92\x93`\0\x93\x85\x92\x91\x85\x85\x12\x15a\x16}W[\x85\x85\x12a\x16^W\x90a\x15,a\x15:\x92[`@\x96`@Q\x95\x86\x94` \x86\x01a\x14\xD4V[\x03`\x1F\x19\x81\x01\x83R\x82a\x04\x0CV[\x81\x85\x92\x85\x96\x82\x81\x11a\x16;Wa\x15P\x81\x85a6\xC2V[\x92a\x15[\x81\x86a6\xC2V[\x88a\x15f\x82\x87a\x18EV[\x13a\x16\x1AWP\x90a\x15z\x91\x97\x96\x92\x97a\x0E\xC8V[`\x01\x95\x91\x82\x91\x87\x80[a\x15\x95W[PPPPPPPPPP\x90V[\x15a\x15\xF6W[P\x86\x97\x98P\x81\x92a\x15\xB5a\x15\xAF\x8B\x89a\x0ETV[`\x01\x1C\x90V[\x99a\x15\xC0\x8B\x88a6\xC2V[\x90\x84a\x15\xCC\x88\x84a\x18EV[\x13a\x15\xEAWPP\x89\x93[\x88a\x15\xE1\x89\x87a\x0E\xC8V[\x92\x01\x94\x99a\x15\x83V[\x8B\x98P\x90\x95P\x93a\x15\xD6V[`\x14\x10\x80a\x16\x11W[\x15a\x16\nW\x88a\x15\x9BV[\x80\x80a\x15\x88V[P\x80\x83\x10a\x15\xFFV[`@Qc\x06\xF1\xBE]`\xE2\x1B\x81R`\x04\x81\x01\x86\x90R`$\x81\x01\x91\x90\x91R`D\x90\xFD[`@Qc0\x82\xDF\xDB`\xE1\x1B\x81R`\x04\x81\x01\x91\x90\x91R`$\x81\x01\x92\x90\x92RP`D\x90\xFD[\x93P\x91a\x16j\x90a)\x11V[\x91a\x16w\x84\x83\x85\x84a'\xECV[\x93a\x15\nV[\x85\x85\x13a\x16\x91W\x90a\x15,a\x15:\x92a\x15\x1AV[\x93P\x94a\x16\x9D\x90a'8V[\x94a\x16\xAA\x84\x83\x88\x84a'\xECV[\x93a\x16}V[\x91a\x16\xC1a\x14\xC8a\x14\xC3\x83\x85a2\x87V[\x91g\r\xE0\xB6\xB3\xA7d\0\0\x92\x83\x03\x92\x83\x11a\x0EOWa\x17\x18\x82a\x17\x04a\x16\xF9a\x14\xC8a\x14\xC3\x84a\x16\xF3a\x176\x9A\x8Ca);V[\x97a&MV[a\x06#\x85\x84Qa(\xE5V[\x92a\x17\x11\x82\x82\x86\x8Aa'\xECV[\x84\x88a$\xF7V[\x90`@Q\x94` \x86\x01R`@\x85\x01R``\x84\x01R`\x80\x83\x01\x90a\t\xDFV[a\x01\0\x81Ra\x01 \x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x03\xCFW`@R\x90V[\x90g\r\xE0\xB6\xB3\xA7d\0\0`\0\x83\x82\x03\x93\x12\x81\x84\x12\x81\x16\x91\x84\x13\x90\x15\x16\x17a\x0EOWV[\x90g\x1B\xC1mgN\xC8\0\0`\0\x83\x82\x03\x93\x12\x81\x84\x12\x81\x16\x91\x84\x13\x90\x15\x16\x17a\x0EOWV[\x90g\x11\x90\0\xAB\x10\x0F\xFB\xFF\x19\x82\x01\x91\x82\x13`\x01\x16a\x0EOWV[\x81\x81\x03\x92\x91`\0\x13\x80\x15\x82\x85\x13\x16\x91\x84\x12\x16\x17a\x0EOWV[\x90g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x81\x02\x92\x81\x84\x05\x14\x90\x15\x17\x15a\x0EOWV[\x90g\x1B\xC1mgN\xC8\0\0\x91\x82\x81\x02\x92\x81\x84\x05\x14\x90\x15\x17\x15a\x0EOWV[\x90c;\x9A\xCA\0\x91\x82\x81\x02\x92\x81\x84\x05\x14\x90\x15\x17\x15a\x0EOWV[`\0\x81\x90\x03\x91\x90`\x01`\xFF\x1B\x81\x14`\x01\x16a\x0EOW`\0\x19\x83\x05\x03a\x0EOWV[\x81\x81\x02\x92\x91`\0\x82\x12`\x01`\xFF\x1B\x82\x14\x16a\x0EOW\x81\x84\x05\x14\x90\x15\x17\x15a\x0EOWV[\x81\x15a\x18\x83W`\x01`\xFF\x1B\x81\x14`\0\x19\x83\x14\x16a\x0EOW\x05\x90V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[\x91\x90\x91a\x18\xD4` \x83\x01\x93a\x18\xCE\x85Qa\x18\xC6a\x18\xBC`@\x88\x01\x92\x83Q\x90a+\x86V[\x97Q\x82Q\x90a+\xAFV[\x90Q\x90a'WV[\x92a'xV[\x92g\r\xE0\xB6\xB3\xA7d\0\0\x80\x85\x12\x15a\x19RW`\0\x85\x13\x15a\x19GW`\0\x85\x82\x03\x95\x12\x81\x86\x12\x81\x16\x91\x86\x13\x90\x15\x16\x17a\x0EOWa\x19;a\x19@\x92a\x196a\x19(a\x14\xC8\x94a\x19#a\x02\xAA\x99a+\xCEV[a\x18EV[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[a\x17\xB8V[a,ZV[\x90Qa(\xE5V[PPPPP`\0\x19\x90V[PPPPP`\0\x90V[\x90\x95\x94\x92\x93a\x14\xF7\x94`\x80\x93a\x01 \x84\x01\x98\x84R` \x84\x01R`@\x83\x01R``\x82\x01R\x01\x90a\t\xDFV[\x90\x92\x91\x82\x86Q` \x88\x01Q`@\x89\x01Q\x90``\x8A\x01Q\x92a\x19\xA6\x83a.\x0BV[a\x19\xAF\x90a\x1BpV[\x93a\x19\xB9\x90a\x17YV[\x90a\x19\xC3\x8Aa.\xC9V[a\x19\xCC\x8Ba\x1C\x97V[a\x19\xD5\x91a/{V[a\x19\xDE\x8Ba\x1C\x97V[a\x19\xE7\x84a.\xF8V[a\x19\xF0\x91a\x17\xB8V[a\x19\xFA\x90\x88a/{V[a\x1A\x03\x91a/\xF8V[a\x1A\x0C\x90a0\x16V[\x93a\x1A\x16\x84a2\xFEV[a\x1A\x1F\x90a\x17\xEEV[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x05a\x1A3\x90a,ZV[\x90a\x1A=\x91a/{V[a\x1AF\x90a/\xAAV[a\x1AO\x90a\x1C\xDFV[\x84\x86a\x1AZ\x86a/LV[\x90a\x1Ad\x91a/{V[\x90a\x1An\x91a/{V[a\x1Aw\x91a\x1C\xC3V[a\x1A\x80\x90a,ZV[\x82a\x1A\x8B\x8C\x84a/{V[\x90a\x1A\x95\x91a/{V[a\x1A\x9E\x91a/{V[a\x1A\xA7\x8Ba\x1C\x97V[a\x1A\xB0\x84a.\xF8V[a\x1A\xB9\x91a\x17\xB8V[a\x1A\xC2\x91a/\xF8V[\x95a\x1A\xCC\x91a/{V[\x90a\x1A\xD6\x90a\x1C\xA9V[a\x1A\xDF\x91a/{V[\x92a\x1A\xE9\x91a/{V[a\x1A\xF2\x90a/\xD1V[\x90a\x1A\xFC\x91a\x17\xB8V[a\x1B\x05\x90a1gV[a\x1B\x0E\x91a/{V[a\x1B\x17\x86a.\xC9V[a\x1B \x91a/\xF8V[\x90a\x1B*\x84a\x1C\xDFV[\x90a\x1B4\x91a\x1C\xC3V[\x90a\x1B>\x91a\x1C\xC3V[`\0\x13a\x1BeWa\x02\xAA\x95a\x1B`\x93a\x15,\x92`@Q\x96\x87\x95` \x87\x01a\x19\\V[a)\x90V[PPPPPP`\0\x90V[\x90c;\x9A\xCA\0\x91\x82\x81\x02\x92\x81\x84\x04\x14\x90\x15\x17\x15a\x0EOWV[\x90\x81g \x05\xFEO&\x8E\xA0\0\x01\x91\x82\x12`\x01\x16a\x0EOWV[\x90\x81g\r\xC5R\x7Fd, \0\x01\x91\x82\x12`\x01\x16a\x0EOWV[\x90\x81g\r\xE0\xB6\xB3\xA7d\0\0\x01\x91\x82\x12`\x01\x16a\x0EOWV[\x90\x81g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19\x01\x91\x82\x12\x15`\x01\x16a\x0EOWV[\x90\x81g\x14\xA8EL\x19\xE1\xAC\0\x01\x91\x82\x12`\x01\x16a\x0EOWV[\x90\x81g\x0F\xC1\x0E\x01W\x82w\xFF\x19\x01\x91\x82\x12\x15`\x01\x16a\x0EOWV[\x90\x81g\x03\xDE\xBD\x08;\x8C|\0\x01\x91\x82\x12`\x01\x16a\x0EOWV[\x90\x81g\x02\x95\xD4\0\xEA2W\xFF\x19\x01\x91\x82\x12\x15`\x01\x16a\x0EOWV[\x90\x81g\x01W\xD8\xB2\xEC\xC7\x08\0\x01\x91\x82\x12`\x01\x16a\x0EOWV[\x90\x81g\x051\n\xA7\xD5!0\0\x01\x91\x82\x12`\x01\x16a\x0EOWV[\x90\x81g\r\xE0\xCC=\x15a\0\0\x01\x91\x82\x12`\x01\x16a\x0EOWV[\x90\x81a\x03\xE8\x01\x91\x82\x12`\x01\x16a\x0EOWV[\x90\x81g\r\xE0\xB6\xB3\xA7c\xFF\xFF\x19\x01\x91\x82\x12\x15`\x01\x16a\x0EOWV[\x91\x90\x91`\0\x83\x82\x01\x93\x84\x12\x91\x12\x90\x80\x15\x82\x16\x91\x15\x16\x17a\x0EOWV[`\x01`\xFF\x1B\x81\x14a\x0EOW`\0\x03\x90V[\x93\x92\x90\x91\x92\x80Q\x91` \x82\x01Q\x90`@\x83\x01Q\x92``\x01Q\x93a\x1D\x12\x84a.\x0BV[a\x1D\x1B\x90a\x1BpV[\x94a\x1D%\x90a\x17YV[\x91\x82a\x1D0\x88a.\xC9V[a\x1D:\x89\x84a\x1C\xC3V[a\x1DC\x91a/{V[a\x1DM\x89\x84a\x1C\xC3V[a\x1DW\x83\x85a/{V[a\x1D`\x91a\x17\xB8V[a\x1Dj\x90\x8Ba/{V[a\x1Ds\x91a/\xF8V[a\x1D|\x90a0\x16V[\x95a\x1D\x86\x86a2\xFEV[a\x1D\x8F\x90a\x17\xEEV[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x05a\x1D\xA3\x90a,ZV[\x90a\x1D\xAD\x91a/{V[a\x1D\xB6\x90a/\xAAV[a\x1D\xBF\x90a\x1C\xDFV[\x86\x88a\x1D\xCA\x88a/LV[\x90a\x1D\xD4\x91a/{V[\x90a\x1D\xDE\x91a/{V[a\x1D\xE7\x91a\x1C\xC3V[a\x1D\xF0\x90a,ZV[\x81a\x1D\xFB\x8A\x86a/{V[\x90a\x1E\x05\x91a/{V[a\x1E\x0E\x91a/{V[\x91a\x1E\x19\x89\x82a\x1C\xC3V[\x91a\x1E#\x91a/{V[a\x1E,\x91a\x17\xB8V[a\x1E5\x91a/\xF8V[\x96a\x1E?\x91a/{V[\x90a\x1EI\x90a\x1C\xA9V[a\x1ER\x91a/{V[\x92a\x1E\\\x91a/{V[a\x1Ee\x90a/\xD1V[\x90a\x1Eo\x91a\x17\xB8V[a\x1Ex\x90a1gV[a\x1E\x81\x91a/{V[\x90a\x1E\x8B\x90a.\xC9V[a\x1E\x94\x91a/\xF8V[\x91a\x1E\x9E\x90a\x1C\xDFV[\x90a\x1E\xA8\x91a\x1C\xC3V[\x90a\x02\xAA\x91a\x1C\xC3V[\x91\x90\x91a\x1E\xF0` \x83\x01\x91a\x1E\xEAa\x1E\xE2\x84Qa\x18\xC6a\x1E\xD8`@\x89\x01\x92\x83Q\x90a+\x86V[\x96Q\x82Q\x90a+\xAFV[\x95\x85Qa'WV[\x90a'xV[g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x82\x12\x15a\x19RW`\0\x82\x13\x15a\x19GWa\x02\xAA\x94a\x19@\x93a\x1F&a\x19;\x93a\x19#a\x14\xC8\x96a+\xCEV[\x05a\x1C\xC3V[\x90\x92\x91\x85Q` \x87\x01Q`@\x88\x01Q``\x89\x01Q\x91a\x1FJ\x82a.\x0BV[a\x1FS\x90a\x1BpV[\x92a\x1F]\x90a\x17YV[\x93a\x1Fg\x86a.\xC9V[a\x1Fp\x87a\x1C\x97V[a\x1Fy\x91a/{V[a\x1F\x83\x89\x83a/{V[a\x1F\x8C\x88a\x1C\x97V[a\x1F\x95\x91a/{V[a\x1F\x9E\x90a\x1C\xDFV[\x86a\x1F\xA9\x8B\x85a/{V[a\x1F\xB2\x90a/!V[\x90a\x1F\xBC\x91a/{V[a\x1F\xC5\x91a\x1C\xC3V[a\x1F\xCE\x91a/\xF8V[a\x1F\xD7\x90a\x1C\xDFV[a\x1F\xE0\x90a0\x16V[\x92a\x1F\xEA\x83a2\xFEV[a\x1F\xF3\x90a\x17\xEEV[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x05a \x07\x90a,ZV[\x90a \x11\x91a/{V[a \x1A\x90a/\xAAV[a #\x90a\x1C\xDFV[\x83\x85a .\x85a/LV[\x90a 8\x91a/{V[\x90a B\x91a/{V[a K\x91a\x1C\xC3V[a T\x90a,ZV[\x85a _\x88\x8Aa/{V[\x90a i\x91a/{V[a r\x91a/{V[\x90a |\x87a\x1C\x97V[a \x85\x87a.\xF8V[a \x8E\x91a\x17\xB8V[a \x97\x91a/{V[a \xA0\x91a/\xF8V[\x93a \xAB\x87\x89a/{V[\x90a \xB5\x90a\x1C\xA9V[a \xBE\x91a/{V[\x92a \xC8\x91a/{V[a \xD1\x90a/\xD1V[\x90a \xDB\x91a\x17\xB8V[a \xE4\x90a1gV[a \xED\x91a/{V[a \xF6\x83a.\xC9V[a \xFF\x91a/\xF8V[\x90a!\t\x90a\x1C\xA9V[\x90a!\x13\x91a\x1C\xC3V[`\0\x13a\x1BeWa\x02\xAA\x95a!5\x93a\x15,\x92`@Q\x96\x87\x95` \x87\x01a\x19\\V[a*\xAFV[\x91a\x14\xC3a\x14\xC8\x91a!K\x93a2\x87V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x03\x90\x81\x11a\x0EOWa\x02\xAA\x91a(\xE5V[\x92\x93`\0\x93\x85\x92\x91\x85\x85\x12\x15a\"\x8AW[\x85\x85\x12a\"kW\x90a\x15,a!\x99\x92`@\x96`@Q\x95\x86\x94` \x86\x01a\x14\xD4V[\x81\x85\x92\x85\x96\x82\x81\x11a\x16;Wa!\xAF\x81\x85a6\xE3V[\x92a!\xBA\x81\x86a6\xE3V[\x88a!\xC5\x82\x87a\x18EV[\x13a\x16\x1AWP\x90a!\xD9\x91\x97\x96\x92\x97a\x0E\xC8V[`\x01\x95\x91\x82\x91\x87\x80[a!\xF3WPPPPPPPPPP\x90V[\x15a\"NW[P\x86\x97\x98P\x81\x92a\"\ra\x15\xAF\x8B\x89a\x0ETV[\x99a\"\x18\x8B\x88a6\xE3V[\x90\x84a\"$\x88\x84a\x18EV[\x13a\"BWPP\x89\x93[\x88a\"9\x89\x87a\x0E\xC8V[\x92\x01\x94\x99a!\xE2V[\x8B\x98P\x90\x95P\x93a\".V[`\x14\x10\x80a\"bW[\x15a\x16\nW\x88a!\xF9V[P\x80\x83\x10a\"WV[\x93P\x91a\"w\x90a)\x11V[\x91a\"\x84\x84\x83\x83\x86a'\xECV[\x93a!xV[\x85\x85\x13a\"\x9EW\x90a\x15,a!\x99\x92a\x15\x1AV[\x93P\x94a\"\xAA\x90a'8V[\x94a\"\xB7\x84\x83\x83\x89a'\xECV[\x93a\"\x8AV[\x92\x91\x90a\"\xD3a\"\xCD\x82\x84a);V[\x85a(\xE5V[\x93\x81\x03\x90\x81\x11a\x0EOW\x92\x81\x03\x90\x81\x11a\x0EOW\x90V[\x92\x91\x90a\"\xFAa\"\xCD\x82\x84a);V[\x93\x81\x01\x80\x91\x11a\x0EOW\x92\x81\x01\x80\x91\x11a\x0EOW\x90V[\x93\x90\x92\x91\x81Q` \x83\x01Q\x91`@\x84\x01Q\x93``\x01Q\x94a#1\x85a.\x0BV[a#:\x90a\x1BpV[\x95a#D\x90a\x17YV[\x92\x83a#O\x89a.\xC9V[a#Y\x8A\x85a\x1C\xC3V[a#b\x91a/{V[a#l\x85\x84a/{V[a#v\x8B\x86a\x1C\xC3V[a#\x7F\x91a/{V[a#\x88\x90a\x1C\xDFV[\x82\x85a#\x94\x88\x87a/{V[\x90a#\x9E\x91a/{V[\x90a#\xA8\x91a/{V[a#\xB1\x91a\x1C\xC3V[a#\xBA\x91a/\xF8V[a#\xC3\x90a\x1C\xDFV[a#\xCC\x90a0\x16V[\x96a#\xD6\x87a2\xFEV[a#\xDF\x90a\x17\xEEV[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x05a#\xF3\x90a,ZV[\x90a#\xFD\x91a/{V[a$\x06\x90a/\xAAV[a$\x0F\x90a\x1C\xDFV[\x87\x89a$\x1A\x89a/LV[\x90a$$\x91a/{V[\x90a$.\x91a/{V[a$7\x91a\x1C\xC3V[a$@\x90a,ZV[\x81a$K\x8B\x8Da/{V[\x90a$U\x91a/{V[a$^\x91a/{V[\x92a$i\x8A\x82a\x1C\xC3V[\x91a$s\x91a/{V[a$|\x91a\x17\xB8V[a$\x85\x91a/{V[a$\x8E\x91a/\xF8V[\x96a$\x98\x91a/{V[\x90a$\xA2\x90a\x1C\xA9V[a$\xAB\x91a/{V[\x92a$\xB5\x91a/{V[a$\xBE\x90a/\xD1V[\x90a$\xC8\x91a\x17\xB8V[a$\xD1\x90a1gV[a$\xDA\x91a/{V[\x90a$\xE4\x90a.\xC9V[a$\xED\x91a/\xF8V[\x90a\x1E\xA8\x90a\x1C\xA9V[\x92\x93`\0\x93\x85\x92\x91\x85\x85\x12\x15a&\x1AW[\x85\x85\x12a%\xFBW\x90a\x15,a%)\x92`@\x96`@Q\x95\x86\x94` \x86\x01a\x14\xD4V[\x81\x85\x92\x85\x96\x82\x81\x11a\x16;Wa%?\x81\x85a7\x05V[\x92a%J\x81\x86a7\x05V[\x88a%U\x82\x87a\x18EV[\x13a\x16\x1AWP\x90a%i\x91\x97\x96\x92\x97a\x0E\xC8V[`\x01\x95\x91\x82\x91\x87\x80[a%\x83WPPPPPPPPPP\x90V[\x15a%\xDEW[P\x86\x97\x98P\x81\x92a%\x9Da\x15\xAF\x8B\x89a\x0ETV[\x99a%\xA8\x8B\x88a7\x05V[\x90\x84a%\xB4\x88\x84a\x18EV[\x13a%\xD2WPP\x89\x93[\x88a%\xC9\x89\x87a\x0E\xC8V[\x92\x01\x94\x99a%rV[\x8B\x98P\x90\x95P\x93a%\xBEV[`\x14\x10\x80a%\xF2W[\x15a\x16\nW\x88a%\x89V[P\x80\x83\x10a%\xE7V[\x93P\x94a&\x07\x90a'8V[\x94a&\x14\x84\x87\x84\x84a'\xECV[\x93a%\x08V[\x85\x85\x13a&.W\x90a\x15,a%)\x92a\x15\x1AV[\x93P\x91a&:\x90a)\x11V[\x91a&G\x84\x84\x84\x84a'\xECV[\x93a&\x1AV[a&\xBDa&\xB8a\x02\xAA\x93a&\xB2a&\xAD\x82Qa&\xA8g\r\xE0\xB6\xB3\xA7d\0\0a&\xA2a&\x9Da&\x97a&\x92`@` \x8B\x01Q\x9A\x01Q\x96a&\x8C\x88\x8Ca+\x86V[\x9Da);V[a2\xFEV[\x97a2\xFEV[a\x17\xEEV[\x05a,ZV[a'WV[a'\x9AV[\x90a\x17\xB8V[a\x17\xD1V[a\x18hV[a&\xF1a&\xB8a&\xECg\x13\xA0K\xBD\xFD\xC9\xBE\x88a&\xE6g\x1B\xC1mgN\xC8\0\0\x95a\x17\xD1V[\x05a\x1C\xDFV[a1gV[\x05\x90V[`\0\x81\x12a'\0W\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01RotoUint: negative`\x80\x1B`D\x82\x01R`d\x90\xFD[a\x03\xE7\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x04?Wa\x03\xE8\x90\x04\x90V[\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x04?Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x04\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17\x82\x15\x15\x16\x15a\x04?W\x04\x90V[g\x06\xF0[Y\xD3\xB2\0\0\x90\x80\x82\x02\x91\x82\x04\x14`\x01\x16\x15a\x04?Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x04\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x04?Wg\x1B\xC1mgN\xC8\0\0\x90\x04\x90V[\x90\x92\x82\x82\x10\x15a(\xA0Wa\x02\xAA\x93a(i\x92\x84g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82a(\x14\x83\x83a'xV[\x10a(\x8DWP`\x01`\x01`\xFF\x1B\x03\x95\x90P[\x83Q\x91a(<a(6\x83\x85a'WV[\x85a'xV[\x10a(nWP`\x01`\x01`\xFF\x1B\x03\x92a(c\x92P\x90P[`@` \x82\x01Q\x91\x01Q\x90a+\x86V[\x92a\x1C\xC3V[a\x1C\xC3V[a(c\x92a\x1E\xEAa(\x82\x92a(\x87\x94a'WV[a+\xCEV[\x91a(SV[a(\x9A\x91a(\x82\x91a'xV[\x94a(&V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FtradingFunction: invalid x\0\0\0\0\0\0`D\x82\x01R`d\x90\xFD[\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x04?W`\x01g\r\xE0\xB6\xB3\xA7d\0\0`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[a\x03\xE9\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x04?W`\x01a\x03\xE8`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[\x90g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x81\x02\x92\x81\x84\x04\x14\x90\x15\x17\x81\x15\x15\x16\x15a\x04?W`\x01\x90`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[a\x01\0\x81\x83\x03\x12a\x03\0W\x80Q\x92a\x02\xAA` \x83\x01Q\x93```@\x85\x01Q\x94\x01a\x11\xC8V[\x91\x90a\x03\xE8\x92`\0\x93`\0\x91\x83\x82\x11a*\x8EWa)\xAC\x81a7%V[a)\xB6\x85\x83a8dV[`\0a)\xC2\x82\x84a\x18EV[\x13a*oWPa)\xD3\x85\x96\x95a\x0E\xB8V[`\x01\x94`\0\x91\x86\x80[a)\xEDW[PPPPPPPP\x90PV[\x15a*JW[P\x85\x96\x97\x98P\x80\x91a*\x08a\x15\xAF\x8B\x88a\x0ETV[\x99a*\x13\x8B\x87a8dV[\x90\x83a*\x1F\x87\x84a\x18EV[\x13a*>WPP\x89\x92[\x87a*4\x88\x86a\x0E\xC8V[\x92\x01\x93\x99\x98a)\xDCV[\x8B\x97P\x90\x94P\x92a*)V[\x86\x10\x80a*dW[\x15a*]W\x88a)\xF3V[\x80\x80a)\xE1V[Pa\x01\0\x82\x10a*RV[`@Qc\x06\xF1\xBE]`\xE2\x1B\x81R`\x04\x81\x01\x92\x90\x92R`$\x82\x01R`D\x90\xFD[`@Qc0\x82\xDF\xDB`\xE1\x1B\x81Ra\x03\xE8`\x04\x82\x01R`$\x81\x01\x85\x90R`D\x90\xFD[\x91\x90a\x03\xE8\x92`\0\x93`\0\x91\x83\x82\x11a*\x8EWa*\xCB\x81a8\x86V[a*\xD5\x85\x83a9\xF6V[`\0a*\xE1\x82\x84a\x18EV[\x13a*oWPa*\xF2\x85\x96\x95a\x0E\xB8V[`\x01\x94`\0\x91\x86\x80[a+\x0BWPPPPPPPP\x90PV[\x15a+hW[P\x85\x96\x97\x98P\x80\x91a+&a\x15\xAF\x8B\x88a\x0ETV[\x99a+1\x8B\x87a9\xF6V[\x90\x83a+=\x87\x84a\x18EV[\x13a+\\WPP\x89\x92[\x87a+R\x88\x86a\x0E\xC8V[\x92\x01\x93\x99\x98a*\xFBV[\x8B\x97P\x90\x94P\x92a+GV[\x86\x10\x80a+{W[\x15a*]W\x88a+\x11V[Pa\x01\0\x82\x10a+pV[\x90a+\x90\x90a.\x0BV[c;\x9A\xCA\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17\x15a\x0EOWa\x02\xAA\x91a'WV[a\x02\xAA\x91a&\xA8g\r\xE0\xB6\xB3\xA7d\0\0a&\xA2a&\x9Da&\xAD\x95a2\xFEV[g\x06\xF0[Y\xD3\xB2\0\0\x81\x14a,TWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x12\x15a,BW\x80\x15a,0W\x80`\x01\x1B\x90\x81\x05`\x02\x03a\x0EOWa,\x0C\x90a0\x16V[\x90g\x13\xA0K\xBD\xFD\xC9\xBE\x88\x91\x80\x83\x02\x92\x83\x05\x14`\x01\x16\x15a\x04?Wa\x02\xAA\x91\x05a\x1C\xDFV[`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x90\xFD[`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x90\xFD[P`\0\x90V[h\x02H\xCE6\xA7\x0C\xB2k>\x19\x81\x13\x15a,TWh\x07U\xBFy\x8BJ\x1B\xF1\xE5\x81\x12\x15a-\xAAWe\x03x-\xAC\xE9\xD9\x90`N\x1B\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x91``\x90`\x01`_\x1B\x84\x82\x84\x1B\x05\x01\x82\x1D\x93\x84\x02\x90\x03\x80l\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x81\x01\x02\x82\x1D\x90n\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dPn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x82n\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x81m\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x81m\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x81m\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x81l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x02\x8D\x1D\x01\x02\x8B\x1D\x01\x02\x89\x1D\x01\x02\x87\x1D\x01\x02\x85\x1D\x01\x93m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x93m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEA\x81\x01\x90\x84m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x91\x01\x01\x02\x90\x1D\x01\x02\x01\x05\x02\x90`\xC3\x03\x1C\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x90\xFD[\x91\x90a\x01 \x83\x82\x03\x12a\x03\0W\x82Q\x92` \x81\x01Q\x92a\x02\xAA`@\x83\x01Q\x93`\x80``\x85\x01Q\x94\x01a\x11\xC8V[`\xB5\x81`\x01`\x88\x1B\x81\x10\x15a.\xB2W[\x80i\x01\0\0\0\0\0\0\0\0\0b\x01\0\0\x92\x10\x15a.\xA5W[e\x01\0\0\0\0\0\x81\x10\x15a.\x98W[c\x01\0\0\0\x81\x10\x15a.\x8BW[\x01\x02`\x12\x1C`\x01\x90\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x80\x80\x92\x04\x10\x90\x03\x90V[`\x10\x1C\x91`\x08\x1B\x91a.OV[` \x1C\x91`\x10\x1B\x91a.BV[`@\x1C\x91` \x1B\x91a.3V[Ph\xB5\0\0\0\0\0\0\0\0\x90P`\x80\x82\x90\x1Ca.\x1BV[g\x1B\xC1mgN\xC8\0\0\x81\x81\x02\x91`\x01`\xFF\x1B\x81\x13`\x01\x17\x91\x83\x05\x14\x16\x15a\x04?Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[a\x03\xE8\x81\x81\x02\x91`\x01`\xFF\x1B\x81\x13`\x01\x17\x91\x83\x05\x14\x16\x15a\x04?Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[a\x03\xE8\x80\x82\x02\x91`\x01`\0\x19\x82\x10\x17\x91\x81\x84\x05\x14\x90\x15\x17\x16\x15a\x04?Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\x13\xA0K\xBD\xE7\x8C\xC4\0\x81\x81\x02\x91`\x01`\xFF\x1B\x81\x13`\x01\x17\x91\x83\x05\x14\x16\x15a\x04?Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[\x81\x81\x02\x91\x81\x15\x82\x84\x05\x82\x14\x17`\0\x19\x90\x92\x10`\x01`\xFF\x1B\x90\x91\x13\x17\x16\x15a\x04?Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x82\x05\x14`\x01\x16\x15a\x04?Wg\x1B\xC1mgN\xC8\0\0\x90\x05\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x82\x05\x14`\x01\x16\x15a\x04?Wg\x13\xA0K\xBD\xE7\x8C\xC4\0\x90\x05\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x82\x05\x14\x82\x15\x15\x16\x15a\x04?W\x05\x90V[`\0\x81\x12\x80\x15a1VW[a1DW\x80\x15a,BWg\x1B\xC1mgN\xC8\0\0\x81\x14a,0Wg\r\xE0\xB6\xB3\xA7d\0\0\x81\x12\x90\x81\x15a15W\x90[a0W\x82a4\xCCV[\x80\x15a,BWa0\xC0a0\x84a0\x7Fa0za0ua0\xC5\x95a2\xFEV[a5\x8DV[a.\x0BV[a\x18\x0BV[a\x196a0\x98a0\x93\x83a4\xF7V[a\x1B\x89V[a0\xBAa0\xB5a0\xAFa0\xAA\x86a5\"V[a\x1B\xA1V[\x85a6\x04V[a\x1B\xB9V[\x90a5kV[a5\xB5V[\x91`\0\x90[`\x02\x82\x10a0\xE5WPP\x15a0\xDCW\x90V[a\x02\xAA\x90a\x1C\xDFV[\x90\x92a1-\x81a1'a0\xFD\x85a\x196`\x01\x96a1gV[a0\xBAa1\x1Da1\x18a\x19;a1\x13\x87\x80a6\x04V[a\x1C\xDFV[a5\xDDV[a&\xB2\x83\x86a6\x04V[\x90a\x1C\xC3V[\x93\x01\x90a0\xCAV[a1>\x90a\x17|V[\x90a0NV[`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x90\xFD[Pg\x1B\xC1mgN\xC8\0\0\x81\x13a0!V[\x80\x15a2zWgV\x98\xEE\xF0fp\0\0\x81\x12\x15a,TWgV\x98\xEE\xF0fo\xFF\xFF\x19\x81\x13\x15a2mW`\0a2]a1\x9C\x83a4\x9FV[a2%a\x19;a1\xB6a1\xB1a0\xB5\x85a'\xC1V[a5LV[\x92a(ia2Xa2Sa2La2Fa2Aa2;a26a20a2+\x8Da2%a2 a2\x1Aa2\x15a0\xAFa2\x10a2\na2\x05a1\xFFa1\xFA\x8Aa6%V[a\x1B\xD1V[\x89a6\x04V[a\x1B\xEBV[\x87a6\x04V[a\x1C\x03V[a\x1C\x1DV[\x83a6\x04V[a\x1C5V[\x90a6\x04V[a\x1COV[\x8Ca6\x04V[a\x1CgV[\x8Aa6\x04V[a\x1C\x7FV[\x88a6\x04V[\x93\x80a6\x04V[a\x18$V[a\x17\x9FV[\x91\x12\x15a\x02\xAAWa\x02\xAA\x90a\x17|V[Pg\x1B\xC1mgN\xC8\0\0\x90V[Pg\r\xE0\xB6\xB3\xA7d\0\0\x90V[a&\xBDa&\xB8a\x02\xAA\x93a1'a&\xAD\x82Qa&\xA8g\r\xE0\xB6\xB3\xA7d\0\0a&\xA2a&\x9Da&\x97a&\x92`@` \x8B\x01Q\x9A\x01Q\x96a&\x8C\x88\x8Ca+\x86V[\x15a2\xCDWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x90\xFD[}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x90a3*`\0\x82\x13a2\xC6V[q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06a3F\x82a6PV[``\x92\x83\x82`\x9F\x03\x01\x1B`\x9F\x1C\x90`_\x19\x01}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x02\x92l\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x82m\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x81m\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x81m\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x81m\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x81m\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x81lFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x02\x8C\x1D\x01\x02\x8A\x1D\x01\x02\x88\x1D\x01\x02\x86\x1D\x01\x02\x84\x1D\x01\x02\x82\x1D\x01\x91x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x91l\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x90\x82m\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x81l\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x81m\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x81m\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x81l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x81\x01\x02\x89\x1D\x01\x02\x87\x1D\x01\x02\x85\x1D\x01\x02\x83\x1D\x01\x02\x90\x1D\x01\x02\x01\x05\x02\x01\x01`\xAE\x1D\x90V[`\x01`\xFF\x1B\x81\x14a4\xBAW`\0\x81\x12\x15a\x02\xAAW\x19`\x01\x01\x90V[`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x90\xFD[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x04?Wg\x1B\xC1mgN\xC8\0\0\x90\x05\x90V[g\x03\xC1f\\z\xAB \0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x04?Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[f\x9F2u$b\xA0\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x04?Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[`\x01\x81\x15\x15\x16\x15a\x04?Wn\xC0\x97\xCE{\xC9\x07\x15\xB3K\x9F\x10\0\0\0\0\x05\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17\x82\x15\x15\x16\x15a\x04?W\x05\x90V[g\x1B\xC1mgN\xC7\xFF\xFF\x19\x81\x81\x02\x91`\x01\x91\x83\x05\x14\x16\x15a\x04?Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\t\xD0(\xCCo _\xFF\x19\x81\x81\x02\x91`\x01\x91\x83\x05\x14\x16\x15a\x04?Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x90\x80\x82\x02\x91\x82\x05\x14`\x01\x16\x15a\x04?Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x04?Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\x02_\x0F\xE1\x05\xA3\x14\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x04?Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[a6[\x81\x15\x15a2\xC6V[\x80o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x07\x1B\x81\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x81\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x81\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x81\x81\x1C`\xFF\x10`\x03\x1B\x17\x81\x81\x1C`\x0F\x10`\x02\x1B\x17\x81\x81\x1C`\x03\x10`\x01\x1B\x17\x90\x81\x1C`\x01\x10\x17\x90V[\x90a6\xD9a\x02\xAA\x92` \x80\x82Q\x83\x01\x01\x91\x01a)kV[\x93\x91\x92\x90Pa'\xECV[\x90a6\xFAa\x02\xAA\x92` \x80\x82Q\x83\x01\x01\x91\x01a)kV[\x93\x90P\x91\x90\x91a'\xECV[\x90a7\x1Ca\x02\xAA\x92` \x80\x82Q\x83\x01\x01\x91\x01a)kV[\x93\x92\x90Pa'\xECV[\x80Q\x81\x01` \x01\x90` \x01\x90a7:\x91a-\xDEV[\x80\x91\x93\x92PQ\x90` \x81\x01Q`@\x82\x01Q\x91``\x01Q\x92a7Z\x83a.\x0BV[a7c\x90a\x1BpV[\x93a7m\x90a\x17YV[\x90a7w\x86a.\xC9V[a7\x80\x87a\x1C\x97V[a7\x89\x91a/{V[a7\x92\x87a\x1C\x97V[a7\x9B\x84a.\xF8V[a7\xA4\x91a\x17\xB8V[a7\xAE\x90\x89a/{V[a7\xB7\x91a/\xF8V[a7\xC0\x90a0\x16V[\x93a7\xCA\x84a2\xFEV[a7\xD3\x90a\x17\xEEV[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x05a7\xE7\x90a,ZV[\x90a7\xF1\x91a/{V[a7\xFA\x90a/\xAAV[a8\x03\x90a\x1C\xDFV[\x84\x86a8\x0E\x86a/LV[\x90a8\x18\x91a/{V[\x90a8\"\x91a/{V[a8+\x91a\x1C\xC3V[a84\x90a,ZV[\x82a8?\x88\x84a/{V[\x90a8I\x91a/{V[a8R\x91a/{V[a8[\x87a\x1C\x97V[a\x1E#\x84a.\xF8V[\x90a8{a\x02\xAA\x92` \x80\x82Q\x83\x01\x01\x91\x01a-\xDEV[\x94\x93\x90\x92\x91Pa\x1C\xF0V[\x80Q\x81\x01` \x01\x90` \x01\x90a8\x9B\x91a-\xDEV[\x80\x91\x92\x93PQ` \x82\x01Q\x90`@\x83\x01Q\x92``\x01Q\x93a8\xBB\x84a.\x0BV[a8\xC4\x90a\x1BpV[\x94a8\xCE\x90a\x17YV[\x91a8\xD8\x87a.\xC9V[a8\xE1\x88a\x1C\x97V[a8\xEA\x91a/{V[a8\xF4\x83\x83a/{V[a8\xFD\x89a\x1C\x97V[a9\x06\x91a/{V[a9\x0F\x90a\x1C\xDFV[\x84a9\x1A\x85\x85a/{V[a9#\x90a/!V[\x90a9-\x91a/{V[a96\x91a\x1C\xC3V[a9?\x91a/\xF8V[a9H\x90a\x1C\xDFV[a9Q\x90a0\x16V[\x94a9[\x85a2\xFEV[a9d\x90a\x17\xEEV[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x05a9x\x90a,ZV[\x90a9\x82\x91a/{V[a9\x8B\x90a/\xAAV[a9\x94\x90a\x1C\xDFV[\x85\x87a9\x9F\x87a/LV[\x90a9\xA9\x91a/{V[\x90a9\xB3\x91a/{V[a9\xBC\x91a\x1C\xC3V[a9\xC5\x90a,ZV[\x83a9\xD0\x89\x8Ba/{V[\x90a9\xDA\x91a/{V[a9\xE3\x91a/{V[\x90a9\xED\x88a\x1C\x97V[a$s\x85a.\xF8V[\x90a:\ra\x02\xAA\x92` \x80\x82Q\x83\x01\x01\x91\x01a-\xDEV[\x94\x93\x90\x92Pa#\x11V\xFE\xA2dipfsX\"\x12 \x9F\xC8\xD6(\xD4d\x1Cp\x07\xB3\x1E\x8A\x8C\x8A\xD0\x8B9])g\xF2\x8B\xA9\x9CtUk\x08\r\x995mdsolcC\0\x08\x16\x003";
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
