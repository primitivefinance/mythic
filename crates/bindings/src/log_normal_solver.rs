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
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15b\0\0^W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`@Qb\0;\xA98\x03\x80b\0;\xA9\x839\x81\x01`@\x81\x90Rb\0\0\x81\x91b\0\0\xA7V[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90Ub\0\x01$V[`\0` \x82\x84\x03\x12\x15b\0\x01\x05W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x01\x1DW`\0\x80\xFD[\x93\x92PPPV[a:u\x80b\0\x014`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x02\x11W`\x005`\xE0\x1C\x80cme\"\x99\x11a\x01FW\x80c\xCB\x1FU2\x11a\0\xE4W\x80c\xE9G\x16\xD5\x11a\0\xBEW\x80c\xE9G\x16\xD5\x14a\x04\xA7W\x80c\xEE>\x8C\xFB\x14a\x04\xBAW\x80c\xF3\r7\xF2\x14a\x04\xCDW\x80c\xF9\xC2\x82\x11\x14a\x04\xE0Wa\x02\x11V[\x80c\xCB\x1FU2\x14a\x04nW\x80c\xCE\x15;\xF4\x14a\x04\x81W\x80c\xE4]Y<\x14a\x04\x94Wa\x02\x11V[\x80c\x90.\xCA\xA2\x11a\x01 W\x80c\x90.\xCA\xA2\x14a\x04\nW\x80c\xA8\xC6.v\x14a\x04\x1DW\x80c\xAFNC\x7F\x14a\x04HW\x80c\xB0\x9D\x04\xE5\x14a\x04[Wa\x02\x11V[\x80cme\"\x99\x14a\x03\xCFW\x80c\x7F\x17@\x9C\x14a\x03\xD7W\x80c\x81\xB5\xFA\xC2\x14a\x03\xEAWa\x02\x11V[\x80c;&\x8D]\x11a\x01\xB3W\x80cO\xD6|X\x11a\x01\x8DW\x80cO\xD6|X\x14a\x03hW\x80cU\xF0\x11\xC6\x14a\x03{W\x80c^\xB4\x08\xFC\x14a\x03\x8EW\x80cb7V\x9F\x14a\x03\xA1Wa\x02\x11V[\x80c;&\x8D]\x14a\x03/W\x80c;M\x100\x14a\x03BW\x80cN\x81\x7F\xD9\x14a\x03UWa\x02\x11V[\x80c\x1E\x97\x8C\xB0\x11a\x01\xEFW\x80c\x1E\x97\x8C\xB0\x14a\x02\xD3W\x80c0m\xB4k\x14a\x02\xE6W\x80c3\"f\xF3\x14a\x02\xF9W\x80c9(\xFF\x97\x14a\x03\x0CWa\x02\x11V[\x80c\x04 X\n\x14a\x02vW\x80c\x12\x06I\xC5\x14a\x02\x9FW\x80c\x13N\xAD\x12\x14a\x02\xC0W[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x02\x89a\x02\x846`\x04a1\x8AV[a\x04\xE8V[`@Qa\x02\x96\x91\x90a1\xFFV[`@Q\x80\x91\x03\x90\xF3[a\x02\xB2a\x02\xAD6`\x04a2\x12V[a\x04\xFDV[`@Q\x90\x81R` \x01a\x02\x96V[a\x02\x89a\x02\xCE6`\x04a3 V[a\x06!V[a\x02\xB2a\x02\xE16`\x04a3\xA6V[a\x068V[a\x02\xB2a\x02\xF46`\x04a3\xA6V[a\x06MV[a\x02\xB2a\x03\x076`\x04a3\xA6V[a\x06\x84V[a\x03\x1Fa\x03\x1A6`\x04a3\xE3V[a\x06\xB0V[`@Qa\x02\x96\x94\x93\x92\x91\x90a4\x1EV[a\x02\x89a\x03=6`\x04a1\x8AV[a\n\x84V[a\x02\xB2a\x03P6`\x04a4EV[a\n\x90V[a\x02\xB2a\x03c6`\x04a3\xA6V[a\n\xB9V[a\x02\xB2a\x03v6`\x04a3\xA6V[a\n\xCEV[a\x02\xB2a\x03\x896`\x04a1\x8AV[a\n\xFAV[a\x02\xB2a\x03\x9C6`\x04a2\x12V[a\x0B/V[a\x03\xB4a\x03\xAF6`\x04a1\x8AV[a\x0CGV[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x02\x96V[a\x02\xB2`\0\x81V[a\x03\xB4a\x03\xE56`\x04a1\x8AV[a\x0C\xA1V[a\x03\xFDa\x03\xF86`\x04a4EV[a\x0C\xFAV[`@Qa\x02\x96\x91\x90a4aV[a\x02\xB2a\x04\x186`\x04a3\xA6V[a\x0E\x06V[`\0Ta\x040\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\x96V[a\x02\xB2a\x04V6`\x04a2\x12V[a\x0E2V[a\x02\x89a\x04i6`\x04a4EV[a\x0F4V[a\x02\x89a\x04|6`\x04a4\x9FV[a\x0F?V[a\x03\xB4a\x04\x8F6`\x04a4EV[a\x0FJV[a\x02\xB2a\x04\xA26`\x04a1\x8AV[a\x10\xDAV[a\x02\x89a\x04\xB56`\x04a1\x8AV[a\x11\x05V[a\x03\xB4a\x04\xC86`\x04a1\x8AV[a\x11\x11V[a\x03\xB4a\x04\xDB6`\x04a1\x8AV[a\x117V[a\x02\xB2`x\x81V[``a\x04\xF4\x83\x83a\x11]V[\x90P[\x92\x91PPV[`\0\x80a\x05\x13\x84\x84a\x05\x0E\x89a\x0C\xFAV[a\x11\x8CV[`@\x80Q` \x81\x01\x88\x90R\x80\x82\x01\x83\x90R``\x80\x82\x01\x88\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x80\x82\x01\x92\x83\x90R`\0\x80Tb.RK`\xE0\x1B\x90\x94R\x93\x94P\x92\x91`\x01`\x01`\xA0\x1B\x03\x16\x90b.RK\x90a\x05p\x90\x8B\x90\x86\x90`\x84\x01a4\xBFV[` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x05\xC6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a: \x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x05\xDAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xFE\x91\x90a4\xD8V[\x90Pa\x06\x15\x87\x87\x83\x86a\x06\x10\x8Da\x0C\xFAV[a\x11\xCEV[\x98\x97PPPPPPPPV[``a\x06.\x84\x84\x84a\x12\x7FV[\x90P[\x93\x92PPPV[`\0a\x06.\x83\x83a\x06H\x87a\x0C\xFAV[a\x12\xF0V[`\0\x80a\x06Y\x85a\x0C\xFAV[\x90P`\0\x80a\x06g\x87a\x0FJV[\x92PP\x91Pa\x06y\x86\x83\x83\x88\x87a\x13\xF9V[\x97\x96PPPPPPPV[`\0\x80a\x06\x90\x85a\x0C\xFAV[\x90P`\0\x80a\x06\x9E\x87a\x0FJV[\x92PP\x91Pa\x06y\x86\x83\x83\x88\x87a\x14dV[`\0\x80`\0``a\x06\xDB`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[a\x06\xFF`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[a\x07\x08\x89a\x0FJV[`@\x85\x01R` \x84\x01R\x82R`\0a\x07\x1F\x8Aa\x0C\xFAV[\x90P`\0\x80a\x07<\x8C\x86`\0\x01Q\x87` \x01Q\x88`@\x01Qa\x0E2V[\x90P\x8A\x15a\x08IW`\0a\x07]\x84``\x01Q\x8Ca\x14\xE0\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x86Q\x90\x91Pa\x07m\x90\x8C\x90a5\nV[\x85Ra\x07y\x81\x83a5\nV[\x85`@\x01\x81\x81RPP`\0a\x07\x97\x8E\x87`\0\x01Q\x88`@\x01Qa\x068V[\x90Pa\x07\xAD\x8E\x87`\0\x01Q\x88`@\x01Q\x84a\x04\xFDV[` \x87\x01\x81\x81R`\x01\x91a\x07\xC2\x90\x83\x90a5\nV[\x90RP` \x80\x88\x01Q\x90\x87\x01Q\x10a\x08,W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: y reserve increase`D\x82\x01Rad!`\xF0\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[\x85` \x01Q\x87` \x01Qa\x08@\x91\x90a5\x1DV[\x93PPPa\tBV[\x82Q``\x84\x01Q`\0\x91a\x08h\x91a\x08b\x90\x8E\x90a\x14\xE0V[\x90a\x14\xF5V[\x90P\x8A\x86` \x01Qa\x08z\x91\x90a5\nV[` \x86\x01Ra\x08\x89\x81\x83a5\nV[\x85`@\x01\x81\x81RPP`\0a\x08\xA7\x8E\x87` \x01Q\x88`@\x01Qa\n\xB9V[\x90Pa\x08\xBD\x8E\x87` \x01Q\x88`@\x01Q\x84a\x0B/V[\x80\x87R`\x01\x90\x87\x90a\x08\xD0\x90\x83\x90a5\nV[\x90RP\x86Q\x86Q\x10a\t/W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: x reserve increase`D\x82\x01Rad!`\xF0\x1B`d\x82\x01R`\x84\x01a\x08#V[\x85Q\x87Qa\t=\x91\x90a5\x1DV[\x93PPP[P\x82Q` \x80\x85\x01Q`@\x80\x87\x01Q\x81Q\x93\x84\x01\x94\x90\x94R\x82\x01R``\x81\x01\x91\x90\x91R`\0\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16ch\xBD>80\x8F\x85`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\t\xC2\x93\x92\x91\x90a50V[`\xC0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\n\x18W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a: \x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\n,W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\nP\x91\x90a5WV[PPPPP\x90P\x80\x83a\nl\x87`\0\x01Q\x88`@\x01Q\x88a\x12\xF0V[\x84\x99P\x99P\x99P\x99PPPPPPP\x93P\x93P\x93P\x93V[``a\x04\xF4\x83\x83a\x15\nV[`\0\x80`\0a\n\x9E\x84a\x0FJV[\x92PP\x91Pa\n\xB1\x82\x82a\x06H\x87a\x0C\xFAV[\x94\x93PPPPV[`\0a\x06.\x83\x83a\n\xC9\x87a\x0C\xFAV[a\x15\"V[`\0\x80a\n\xDA\x85a\x0C\xFAV[\x90P`\0\x80a\n\xE8\x87a\x0FJV[\x92P\x92PPa\x06y\x86\x83\x83\x88\x87a\x15\xF0V[`\0\x80a\x0B\x06\x84a\x0C\xFAV[\x90P`\0\x80a\x0B\x14\x86a\x0FJV[\x92PP\x91Pa\x0B%\x85\x83\x83\x86a\x16RV[\x96\x95PPPPPPV[`\0\x80a\x0BE\x84\x84a\x0B@\x89a\x0C\xFAV[a\x17<V[`@\x80Q` \x81\x01\x83\x90R\x80\x82\x01\x88\x90R``\x80\x82\x01\x88\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x80\x82\x01\x92\x83\x90R`\0\x80Tb.RK`\xE0\x1B\x90\x94R\x93\x94P\x92\x91`\x01`\x01`\xA0\x1B\x03\x16\x90b.RK\x90a\x0B\xA2\x90\x8B\x90\x86\x90`\x84\x01a4\xBFV[` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0B\xF8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a: \x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0C\x0CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C0\x91\x90a4\xD8V[\x90Pa\x06\x15\x87\x87\x83\x86a\x0CB\x8Da\x0C\xFAV[a\x17\x81V[`\0\x80`\0\x80`\0a\x0CX\x87a\x0FJV[\x92PP\x91P`\0\x80a\x0Cm`\0\x89\x86\x86a\x18%V[\x91P\x91P`\0a\x0C~\x8A\x84\x84a\x068V[\x90P`\0a\x0C\x8E\x8B\x85\x85\x85a\x04\xFDV[\x93\x9B\x93\x9AP\x91\x98P\x91\x96PPPPPPPV[`\0\x80`\0\x80`\0a\x0C\xB2\x87a\x0FJV[\x92P\x92PP`\0\x80a\x0C\xC7`\x01\x89\x86\x86a\x18%V[\x91P\x91P`\0a\x0C\xD8\x8A\x84\x84a\n\xB9V[\x90P`\0a\x0C\xE8\x8B\x85\x85\x85a\x0B/V[\x9B\x93\x9AP\x91\x98P\x91\x96PPPPPPPV[a\r5`@Q\x80`\xA0\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81RP\x90V[`\0T`@Qc\xDC\x17\x83U`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xDC\x17\x83U\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\r\xB7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a: \x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\r\xCBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\r\xF3\x91\x90\x81\x01\x90a5\xADV[\x80` \x01\x90Q\x81\x01\x90a\x04\xF7\x91\x90a7\x8BV[`\0\x80a\x0E\x12\x85a\x0C\xFAV[\x90P`\0\x80a\x0E \x87a\x0FJV[\x92P\x92PPa\x06y\x86\x83\x83\x88\x87a\x18\x8EV[`@\x80Q` \x81\x01\x85\x90R\x80\x82\x01\x84\x90R``\x80\x82\x01\x84\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x80\x82\x01\x92\x83\x90R`\0\x80Tb.RK`\xE0\x1B\x90\x94R\x92\x90\x91\x83\x91`\x01`\x01`\xA0\x1B\x03\x16\x90b.RK\x90a\x0E\x8F\x90\x8A\x90\x86\x90`\x84\x01a4\xBFV[` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0E\xE5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a: \x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0E\xF9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\x1D\x91\x90a4\xD8V[\x90Pa\x06y\x86\x86\x83\x87a\x0F/\x8Ca\x0C\xFAV[a\x18\xEAV[``a\x04\xF7\x82a\x19\x8EV[``a\x04\xF7\x82a\x19\xBAV[`\0\x80`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xAF\xBA\x13\xC4`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0F\xDAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a: \x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0F\xEEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\x12\x91\x90a7\xAAV[`\x01`\x01`\xA0\x1B\x03\x16c\xCE\x15;\xF4\x85`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x10?\x91\x81R` \x01\x90V[```@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x10\x95W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a: \x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x10\xA9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\xCD\x91\x90a7\xCAV[\x92P\x92P\x92P\x91\x93\x90\x92PV[`\0\x80a\x10\xE6\x84a\x0C\xFAV[\x90P`\0\x80a\x10\xF4\x86a\x0FJV[\x92P\x92PPa\x0B%\x85\x83\x83\x86a\x19\xD0V[``a\x04\xF4\x83\x83a\x1A\x9AV[`\0\x80`\0\x80`\0a\x11\"\x87a\x0FJV[\x92PP\x91P`\0\x80a\x0Cm`\x01\x89\x86\x86a\x18%V[`\0\x80`\0\x80`\0a\x11H\x87a\x0FJV[\x92P\x92PP`\0\x80a\x0C\xC7`\0\x89\x86\x86a\x18%V[```\x02\x83\x83`@Q` \x01a\x11u\x93\x92\x91\x90a8\x1DV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x92\x91PPV[`\0\x80a\x11\x99\x84\x84a\x1A\xB2V[\x90P`\0a\x11\xA6\x82a\x1B\x13V[\x90P`\0a\x11\xB3\x82a\x1B|V[\x85Q\x90\x91Pa\x06y\x90\x82\x90a\x11\xC8\x90\x8Aa\x14\xE0V[\x90a\x14\xE0V[`\0\x82\x80\x85\x83\x81\x12\x15a\x12\x0EW[`\0\x81\x12\x15a\x12\tWa\x11\xF4\x83a\x03\xE9a\x03\xE8a\x1B\xC5V[\x92Pa\x12\x02\x89\x84\x8A\x88a\x1B\xF3V[\x90Pa\x11\xDCV[a\x12;V[`\0\x81\x13\x15a\x12;Wa\x12&\x82a\x03\xE7a\x03\xE8a\x1C\xF4V[\x91Pa\x124\x89\x83\x8A\x88a\x1B\xF3V[\x90Pa\x12\x0EV[a\x12r\x89\x89\x83\x88`@Q` \x01a\x12U\x94\x93\x92\x91\x90a8<V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83\x85`\x14`@a\x1D\x13a\x1D@V[\x99\x98PPPPPPPPPV[```\0a\x12\x8E\x85\x85\x85a\x1EQV[\x90P`\0a\x12\x9D\x82\x86\x86a\x11\x8CV[\x90P`\0a\x12\xAD\x87\x83\x85\x88a\x1B\xF3V[\x90Pa\x12\xBC\x87\x83\x83\x86\x89a\x18\xEAV[\x92P\x86\x82\x84\x87`@Q` \x01a\x12\xD5\x94\x93\x92\x91\x90a8<V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x93PPPP\x93\x92PPPV[`\0\x80a\x13\x05\x83` \x01Q\x84`@\x01Qa\x1E\x96V[\x90P`\0a\x13\x1B\x84` \x01Q\x85`@\x01Qa\x1E\xBCV[\x90P`\0a\x136\x85`@\x01Q\x83a\x1E\xEB\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x13D\x88\x88a\x1F\0V[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0\x81\x12a\x13bW`\0\x94PPPPPa\x061V[`\0\x81\x13a\x13xW`\0\x19\x94PPPPPa\x061V[`\0a\x13\x94a\x13\x8F\x83g\r\xE0\xB6\xB3\xA7d\0\0a8\x90V[a\x1F\x15V[\x90P`\0\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x13\xAC\x88\x85a8\xB7V[a\x13\xB6\x91\x90a8\xFDV[a\x13\xC0\x91\x90a8\x90V[\x90P`\0a\x13\xCD\x82a\x1F\xB2V[\x90P`\0a\x13\xDA\x82a\x1B|V[\x8AQ\x90\x91Pa\x13\xE9\x90\x82a\x14\xE0V[\x9C\x9BPPPPPPPPPPPPV[`\0\x82`d\x82a\x14\x0C\x89\x89\x89\x85\x89a\x14dV[\x90P`\0\x81\x12\x15a\x14#W`\0\x93PPPPa\x14[V[a\x12r\x89\x89\x89\x88`@Q` \x01a\x14=\x94\x93\x92\x91\x90a8<V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83\x85`\x01a\x01\0a![a\x1D@V[\x95\x94PPPPPV[`\0\x80\x82``\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x14\x7F\x91\x90a8\x90V[\x90P`\0a\x14\x91\x88\x88\x88\x85\x89\x89a!\x8CV[\x90P`\0a\x14\x9E\x82a\"\xE7V[\x90P`\0a\x14\xAB\x83a$\x10V[\x90P\x80\x82\x84a\x01\0\x01Qa\x14\xBE\x90a9+V[a\x14\xC8\x91\x90a9GV[a\x14\xD2\x91\x90a9GV[\x9A\x99PPPPPPPPPPV[`\0a\x04\xF4\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x1B\xC5V[`\0a\x04\xF4\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x1B\xC5V[```\x04\x83\x83`@Q` \x01a\x11u\x93\x92\x91\x90a8\x1DV[`\0\x80a\x157\x83` \x01Q\x84`@\x01Qa\x1E\x96V[\x90P`\0a\x15M\x84` \x01Q\x85`@\x01Qa\x1E\xBCV[\x90P`\0a\x15h\x85`@\x01Q\x83a\x1E\xEB\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x85Q\x90\x91P`\0\x90a\x15\x85\x90a\x15~\x90\x89a\x1E\xEBV[\x89\x90a\x1F\0V[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0\x81\x12a\x15\xA3W`\0\x94PPPPPa\x061V[`\0\x81\x13a\x15\xB9W`\0\x19\x94PPPPPa\x061V[`\0a\x15\xC4\x82a\x1F\x15V[\x90P`\0\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x15\xDC\x88\x85a8\xB7V[a\x15\xE6\x91\x90a8\xFDV[a\x13\xC0\x91\x90a9GV[`\0\x82`d\x82a\x16\x03\x89\x89\x89\x85\x89a\x18\x8EV[\x90P`\0\x81\x12\x15a\x16\x1AW`\0\x93PPPPa\x14[V[a\x12r\x89\x89\x89\x88`@Q` \x01a\x164\x94\x93\x92\x91\x90a8<V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83\x85`\x01a\x01\0a$\x9Ca\x1D@V[`\0\x80\x82``\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x16m\x91\x90a8\x90V[\x90P`\0\x83` \x01Q\x90P`\0a\x16\x88\x88\x86`\0\x01Qa$\xCDV[\x90P`\0a\x16\xBAa\x16\xA1\x84g\x1B\xC1mgN\xC8\0\0a$\xE1V[a\x16\xAB\x84\x86a$\xE1V[a\x16\xB5\x91\x90a9GV[a\x1B\x13V[\x90P`\0a\x16\xDAa\x16\xD3\x83g\r\xE0\xB6\xB3\xA7d\0\0a8\x90V[\x89\x90a%\x05V[\x90Pa\x14\xD2g\r\xE0\xB6\xB3\xA7d\0\0a\x17(a\x16\xF5\x8C\x8Ca$\xE1V[a\x17\"a\x17\n\x87g\r\xE0\xB6\xB3\xA7d\0\0a8\x90V[a\x17\x1Cg\r\xE0\xB6\xB3\xA7d\0\0\x8Ca8\x90V[\x90a%\x05V[\x90a$\xE1V[a\x172\x91\x90a9GV[a\x17\"\x8B\x84a8\x90V[`\0\x80a\x17I\x84\x84a%8V[\x90P`\0a\x17V\x82a\x1B\x13V[\x90P`\0a\x17c\x82a\x1B|V[\x90Pa\x06ya\x17z\x82g\r\xE0\xB6\xB3\xA7d\0\0a5\x1DV[\x88\x90a\x14\xE0V[`\0\x82\x80\x85\x83\x81\x12\x15a\x17\xC1W[`\0\x81\x12\x15a\x17\xBCWa\x17\xA7\x83a\x03\xE9a\x03\xE8a\x1B\xC5V[\x92Pa\x17\xB5\x83\x8A\x8A\x88a\x1B\xF3V[\x90Pa\x17\x8FV[a\x17\xEEV[`\0\x81\x13\x15a\x17\xEEWa\x17\xD9\x82a\x03\xE7a\x03\xE8a\x1C\xF4V[\x91Pa\x17\xE7\x82\x8A\x8A\x88a\x1B\xF3V[\x90Pa\x17\xC1V[a\x12r\x89\x89\x83\x88`@Q` \x01a\x18\x08\x94\x93\x92\x91\x90a8<V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83\x85`\x14`@a%}a\x1D@V[`\0\x80\x80a\x183\x84\x86a\x14\xF5V[\x90P`\0a\x18A\x87\x83a\x14\xE0V[\x90P\x87a\x18WWa\x18R\x87\x87a5\x1DV[a\x18aV[a\x18a\x87\x87a5\nV[\x93P\x87a\x18wWa\x18r\x81\x86a5\x1DV[a\x18\x81V[a\x18\x81\x81\x86a5\nV[\x92PPP\x94P\x94\x92PPPV[`\0\x80\x82``\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x18\xA9\x91\x90a8\x90V[\x90P`\0a\x18\xBB\x88\x88\x88\x85\x89\x89a%\xAAV[\x90P`\0a\x18\xC8\x82a'\x02V[\x90P`\0a\x18\xD5\x83a(0V[\x90P\x80\x82a\x14\xBEg\r\xE0\xB6\xB3\xA7d\0\0a9+V[`\0\x82\x80\x85\x83\x81\x12\x15a\x19*W[`\0\x81\x12\x15a\x19%Wa\x19\x10\x82a\x03\xE7a\x03\xE8a\x1C\xF4V[\x91Pa\x19\x1E\x89\x89\x84\x88a\x1B\xF3V[\x90Pa\x18\xF8V[a\x19WV[`\0\x81\x13\x15a\x19WWa\x19B\x83a\x03\xE9a\x03\xE8a\x1B\xC5V[\x92Pa\x19P\x89\x89\x85\x88a\x1B\xF3V[\x90Pa\x19*V[a\x12r\x89\x89\x83\x88`@Q` \x01a\x19q\x94\x93\x92\x91\x90a8<V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83\x85`\x14`@a(\xDCa\x1D@V[```\x01\x82`@Q` \x01a\x19\xA4\x92\x91\x90a9oV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x91\x90PV[```\x05\x82`@Q` \x01a\x19\xA4\x92\x91\x90a9\x8AV[`\0\x80\x82``\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x19\xEB\x91\x90a8\x90V[\x83Q` \x85\x01Q\x91\x92P\x90`\0a\x1A\x02\x89\x84a$\xCDV[\x90P`\0a\x1A\x18\x83g\x1B\xC1mgN\xC8\0\0a$\xE1V[a\x1A\"\x83\x85a$\xE1V[a\x1A,\x91\x90a8\x90V[\x90P`\0a\x1A9\x82a\x1B\x13V[\x90P`\0a\x1AK\x82a\x17\x1C\x8C\x89a%\x05V[\x90Pa\x13\xE9g\r\xE0\xB6\xB3\xA7d\0\0a\x1A\x86a\x1Apa\x1Ai\x8A\x8Fa%\x05V[\x8F\x90a$\xE1V[a\x17\"\x86a\x17\x1Cg\r\xE0\xB6\xB3\xA7d\0\0\x8Ea8\x90V[a\x1A\x90\x91\x90a9GV[a\x17\"\x8D\x84a8\x90V[```\x03\x83\x83`@Q` \x01a\x11u\x93\x92\x91\x90a8\x1DV[\x80Q` \x82\x01Q`@\x83\x01Q`\0\x92\x91\x90\x83a\x1A\xCE\x83\x83a\x1E\x96V[\x90P`\0a\x1A\xDC\x88\x86a$\xCDV[\x90P`\0a\x1A\xEA\x85\x85a\x1E\xBCV[\x90P\x82a\x1A\xF7\x82\x84a8\x90V[a\x1B\t\x90g\r\xE0\xB6\xB3\xA7d\0\0a8\xB7V[a\x12r\x91\x90a8\xFDV[`\0\x80g\x13\xA0K\xBD\xFD\xC9\xBE\x88a\x1B1g\r\xE0\xB6\xB3\xA7d\0\0\x85a8\xB7V[a\x1B;\x91\x90a8\xFDV[\x90P`\0a\x1BH\x82a9+V[\x90P`\0a\x1BU\x82a)\tV[\x90Pg\x1B\xC1mgN\xC8\0\0a\x1Brg\r\xE0\xB6\xB3\xA7d\0\0\x83a8\xB7V[a\x14[\x91\x90a8\xFDV[`\0\x80\x82\x12\x15a\x1B\xC1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01RotoUint: negative`\x80\x1B`D\x82\x01R`d\x01a\x08#V[P\x90V[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x1B\xDDW`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[`\0\x82\x85\x10a\x1CDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FtradingFunction: invalid x\0\0\0\0\0\0`D\x82\x01R`d\x01a\x08#V[`\0\x80g\r\xE0\xB6\xB3\xA7d\0\0a\x1CZ\x88\x87a\x1F\0V[\x10a\x1CnW`\x01`\x01`\xFF\x1B\x03\x91Pa\x1C~V[a\x1C{a\x13\x8F\x88\x87a\x1F\0V[\x91P[g\r\xE0\xB6\xB3\xA7d\0\0a\x1C\x9E\x87a\x1C\x99\x87`\0\x01Q\x89a\x1E\xEBV[a\x1F\0V[\x10a\x1C\xB1WP`\x01`\x01`\xFF\x1B\x03a\x1C\xC9V[a\x1C\xC6a\x13\x8F\x87a\x1C\x99\x87`\0\x01Q\x89a\x1E\xEBV[\x90P[`\0a\x1C\xDD\x85` \x01Q\x86`@\x01Qa\x1E\x96V[\x90P\x80a\x1C\xEA\x83\x85a9GV[a\x06\x15\x91\x90a9GV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x1D\x0CW`\0\x80\xFD[\x04\x92\x91PPV[`\0\x80`\0\x80\x85\x80` \x01\x90Q\x81\x01\x90a\x1D-\x91\x90a9\xB0V[\x93PP\x92P\x92Pa\x0B%\x83\x86\x84\x84a\x1B\xF3V[`\0\x84\x86\x11\x15a\x1DmW`@Qc0\x82\xDF\xDB`\xE1\x1B\x81R`\x04\x81\x01\x87\x90R`$\x81\x01\x86\x90R`D\x01a\x08#V[`\0a\x1D}\x88\x88\x85c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x1D\x8F\x89\x88\x86c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x1D\x9D\x82\x84a8\xB7V[\x13\x15a\x1D\xC6W`@Qc\x06\xF1\xBE]`\xE2\x1B\x81R`\x04\x81\x01\x83\x90R`$\x81\x01\x82\x90R`D\x01a\x08#V[`\0a\x1D\xD2\x89\x89a5\x1DV[\x90P`\0[`\x02a\x1D\xE3\x8A\x8Ca5\nV[a\x1D\xED\x91\x90a9\xF4V[\x94P`\0a\x1D\xFF\x8C\x87\x89c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x1E\r\x86\x83a8\xB7V[\x13a\x1E\x1AW\x85\x99Pa\x1E!V[\x85\x9AP\x80\x94P[a\x1E+\x8B\x8Ba5\x1DV[\x92PP`\x01\x01\x87\x82\x11\x80\x15a\x1E?WP\x86\x81\x10[a\x1D\xD7WPPPP\x96\x95PPPPPPV[`\0\x80a\x1E^\x84\x84a%8V[\x90P`\0a\x1Ek\x82a\x1B\x13V[\x90P`\0a\x1Ex\x82a\x1B|V[\x90Pa\x06ya\x1E\x8F\x82g\r\xE0\xB6\xB3\xA7d\0\0a5\x1DV[\x88\x90a\x14\xF5V[`\0\x80a\x1E\xA2\x83a*\xF2V[a\x1E\xB0\x90c;\x9A\xCA\0a:\x08V[\x90Pa\n\xB1\x84\x82a\x1E\xEBV[`\0\x80a\x1E\xDB\x83a\x1E\xD5\x86g\x1B\xC1mgN\xC8\0\0a+\x96V[\x90a\x1E\xEBV[\x90Pa\n\xB1g\x06\xF0[Y\xD3\xB2\0\0\x82[`\0a\x04\xF4\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x1C\xF4V[`\0a\x04\xF4\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x1C\xF4V[`\0g\x06\xF0[Y\xD3\xB2\0\0\x82\x03a\x1F.WP`\0\x91\x90PV[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x12a\x1FVW`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\x1FwW`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x1F\x84\x83`\x02a8\xB7V[\x90P`\0a\x1F\x91\x82a+\xC2V[\x90P`\0a\x1F\xA7g\x13\xA0K\xBD\xFD\xC9\xBE\x88\x83a.;V[\x90Pa\x14[\x81a9+V[`\0h\x02H\xCE6\xA7\x0C\xB2k>\x19\x82\x13a\x1F\xCDWP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a \x14W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x01a\x08#V[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05`\x01`_\x1B\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1Dm\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1Dn\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[`\0\x80`\0\x80`\0\x86\x80` \x01\x90Q\x81\x01\x90a!w\x91\x90a9\xB0V[\x93P\x93P\x93P\x93Pa\x06y\x84\x84\x84\x89\x85a\x14dV[a!\xE9`@Q\x80a\x01`\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`\0a\"\x07a!\xF8\x88\x86a9GV[g\x1B\xC1mgN\xC8\0\0\x90a%\x05V[\x90P`\0a\"\x15\x85\x87a%\x05V[a\"\x1F\x86\x89a9GV[a\")\x91\x90a8\x90V[\x90P`\0a\"?a\":\x84\x84a$\xE1V[a+\xC2V[\x90P`\0a\"Tg\x1B\xC1mgN\xC8\0\0a*\xF2V[a\"b\x90c;\x9A\xCA\0a:\x08V[\x90P`\0a\"s\x87`@\x01Qa*\xF2V[a\"\x81\x90c;\x9A\xCA\0a:\x08V[`@\x80Qa\x01`\x81\x01\x82R\x94\x85R\x88Q` \x80\x87\x01\x91\x90\x91R\x89\x01Q\x85\x82\x01R\x97\x90\x97\x01Q``\x84\x01RP`\x80\x82\x01\x97\x90\x97R`\xA0\x81\x01\x98\x90\x98RPPP`\xC0\x85\x01\x93\x90\x93R`\xE0\x84\x01Ra\x01\0\x83\x01\x93\x90\x93Ra\x01 \x82\x01Ra\x01@\x81\x01\x91\x90\x91R\x90V[`\0\x80a#\x19g\x1B\xC1mgN\xC8\0\0a\x17\"\x85``\x01Qa\x17\x1C\x87`@\x01Q\x88`@\x01Qa%\x05\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a#\"\x90a9+V[\x90P`\0a#S\x84`\0\x01Qa\x17\x1C\x86a\x01 \x01Qa\x17\x1C\x88`@\x01Q\x89a\x01@\x01Qa%\x05\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a#ia#d\x83\x85a9GV[a\x1F\xB2V[\x90P`\0a#\xB9a#\x9F\x87`\x80\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a#\x8A\x90a9+V[a#\x94\x91\x90a9GV[`\xA0\x89\x01Q\x90a%\x05V[\x87`\xC0\x01Qa#\xAE\x91\x90a9GV[` \x88\x01Q\x90a%\x05V[\x90P`\0a#\xC7\x83\x83a%\x05V[\x90P`\0a#\xE6\x88`\x80\x01Q\x89`\xE0\x01Qa%\x05\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x88`\xE0\x01Q\x89`\xC0\x01Qa#\xFA\x91\x90a9GV[a$\x04\x91\x90a8\x90V[\x90Pa\x06\x15\x82\x82a$\xE1V[`\0\x80a$O\x83`\x80\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a$-\x90a9+V[a$7\x91\x90a9GV[` \x85\x01Qa\x17\x1C\x90g\x06\xF0[Y\xD3\xB2\0\0\x90a%\x05V[\x90P`\0a$x\x84a\x01@\x01Qa\x17\"\x86a\x01 \x01Q\x87`@\x01Qa%\x05\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa\n\xB1a$\x95\x85`\0\x01Q\x83a$\x90\x91\x90a8\x90V[a)\tV[\x83\x90a%\x05V[`\0\x80`\0\x80`\0\x86\x80` \x01\x90Q\x81\x01\x90a$\xB8\x91\x90a9\xB0V[\x93P\x93P\x93P\x93Pa\x06y\x84\x84\x84\x89\x85a\x18\x8EV[`\0a\x04\xF4a$\xDC\x84\x84a\x14\xF5V[a.PV[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x81\x02\x90\x81\x05\x83\x14\x82\x15\x15\x16a$\xFFW`\0\x80\xFD[\x05\x91\x90PV[\x81\x81\x02\x82\x15\x83\x82\x05\x83\x14\x17`\0\x19\x84\x10`\x01`\xFF\x1B\x84\x13\x17\x16a%'W`\0\x80\xFD[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x92\x91PPV[\x80Q` \x82\x01Q`@\x83\x01Q`\0\x92\x91\x90\x83a%T\x83\x83a\x1E\x96V[\x90P`\0a%b\x88\x86a$\xCDV[\x90P`\0a%p\x85\x85a\x1E\xBCV[\x90P\x82a\x1A\xF7\x82\x84a9GV[`\0\x80`\0\x80\x85\x80` \x01\x90Q\x81\x01\x90a%\x97\x91\x90a9\xB0V[\x93PP\x92P\x92Pa\x0B%\x85\x84\x84\x84a\x1B\xF3V[a&\x07`@Q\x80a\x01`\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`\0a&\x16a!\xF8\x88\x86a9GV[\x90P`\0a&$\x85\x87a%\x05V[\x84Q\x86\x90a&2\x90\x8Aa%\x05V[a&<\x91\x90a9GV[a&F\x91\x90a8\x90V[\x90P`\0a&Wa\":\x84\x84a$\xE1V[\x90P`\0a&lg\x1B\xC1mgN\xC8\0\0a*\xF2V[a&z\x90c;\x9A\xCA\0a:\x08V[\x90P`\0a&\x8B\x87`@\x01Qa*\xF2V[a&\x99\x90c;\x9A\xCA\0a:\x08V[`@\x80Qa\x01`\x81\x01\x82R\x94\x85R\x88Q` \x80\x87\x01\x91\x90\x91R\x89\x01Q\x85\x82\x01R\x97\x90\x97\x01Q``\x84\x01RP`\x80\x82\x01\x97\x90\x97R`\xA0\x81\x01\x98\x90\x98RPPP`\xC0\x85\x01\x93\x90\x93R`\xE0\x84\x01Ra\x01\0\x83\x01\x93\x90\x93Ra\x01 \x82\x01\x92\x90\x92Ra\x01@\x81\x01\x91\x90\x91R\x90V[`\0\x80a'4g\x1B\xC1mgN\xC8\0\0a\x17\"\x85``\x01Qa\x17\x1C\x87`@\x01Q\x88`@\x01Qa%\x05\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a'=\x90a9+V[\x90P`\0a'n\x84`\0\x01Qa\x17\x1C\x86a\x01@\x01Qa\x17\x1C\x88`@\x01Q\x89a\x01 \x01Qa%\x05\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a'\x7Fa#d\x83\x85a9GV[\x90P`\0a'\xC9a'\xA0\x87`\x80\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a#\x8A\x90a9+V[`\xC0\x88\x01Q` \x89\x01Qa'\xB3\x91a%\x05V[a'\xBD\x91\x90a9GV[a\x01\0\x88\x01Q\x90a%\x05V[\x90P`\0a'\xD7\x83\x83a%\x05V[\x90P`\0a$\x04a'\xF9\x89`\x80\x01Q\x8A`\xE0\x01Qa%\x05\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[`\xE0\x8A\x01Q`\xC0\x8B\x01Q` \x8C\x01Qa(\x11\x91a%\x05V[a(\x1B\x91\x90a9GV[a(%\x91\x90a8\x90V[` \x8A\x01Q\x90a%\x05V[`\0\x80a(c\x83`\x80\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a(M\x90a9+V[a(W\x91\x90a9GV[a\x01\0\x85\x01Q\x90a%\x05V[\x90P`\0a(\x8C\x84a\x01 \x01Qa\x17\"\x86a\x01@\x01Q\x87`@\x01Qa%\x05\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a(\xADa(\xA6\x86`\0\x01Q\x84a$\x90\x91\x90a8\x90V[\x84\x90a%\x05V[\x90P`\0a(\xD0\x86` \x01Qg\x1B\xC1mgN\xC8\0\0a%\x05\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa\x0B%\x82\x82a$\xE1V[`\0\x80`\0\x80\x85\x80` \x01\x90Q\x81\x01\x90a(\xF6\x91\x90a9\xB0V[\x93PP\x92P\x92Pa\x0B%\x83\x83\x87\x84a\x1B\xF3V[`\0\x81`\0\x03a)\"WPg\r\xE0\xB6\xB3\xA7d\0\0\x91\x90PV[gV\x98\xEE\xF0fp\0\0\x82\x12a)9WP`\0\x91\x90PV[a)JgV\x98\xEE\xF0fp\0\0a9+V[\x82\x13a)_WPg\x1B\xC1mgN\xC8\0\0\x91\x90PV[`\0a)j\x83a0+V[\x90P`\0a)\xA3g\r\xE0\xB6\xB3\xA7d\0\0a)\x8C\x84g\x1B\xC1mgN\xC8\0\0a\x1F\0V[a)\x9E\x90g\r\xE0\xB6\xB3\xA7d\0\0a9GV[a0bV[\x90P`\0\x80\x82a*\x04\x81a)\xF1\x81a)\xDF\x81a)\xC7\x81g\x02_\x0F\xE1\x05\xA3\x14\0a.;V[a)\xDA\x90g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19a9GV[a.;V[a)\xDA\x90g\x14\xA8EL\x19\xE1\xAC\0a9GV[a)\xDA\x90g\x0F\xC1\x0E\x01W\x82w\xFF\x19a9GV[a*\x16\x90g\x03\xDE\xBD\x08;\x8C|\0a9GV[\x91P\x83\x90Pa*~\x81a*l\x81a*Z\x81a*H\x81a*5\x81\x8Ba.;V[a)\xDA\x90g\x02\x95\xD4\0\xEA2W\xFF\x19a9GV[a)\xDA\x90g\x01W\xD8\xB2\xEC\xC7\x08\0a9GV[a)\xDA\x90g\x051\n\xA7\xD5!0\0a9GV[a)\xDA\x90g\r\xE0\xCC=\x15a\0\0a9GV[\x91P\x81g\x11\x90\0\xAB\x10\x0F\xFC\0a*\x94\x87\x88a.;V[a*\xA0\x90`\0\x19a8\xB7V[a*\xAA\x91\x90a8\x90V[a*\xB4\x91\x90a9GV[\x92PP`\0a*\xC2\x83a\x1F\xB2V[\x90P`\0a*\xD0\x85\x83a.;V[\x90P`\0\x88\x12a*\xE0W\x80a\x06\x15V[a\x06\x15\x81g\x1B\xC1mgN\xC8\0\0a8\x90V[`\xB5\x81`\x01`\x88\x1B\x81\x10a+\x0BW`@\x91\x90\x91\x1B\x90`\x80\x1C[i\x01\0\0\0\0\0\0\0\0\0\x81\x10a+'W` \x91\x90\x91\x1B\x90`@\x1C[e\x01\0\0\0\0\0\x81\x10a+?W`\x10\x91\x90\x91\x1B\x90` \x1C[c\x01\0\0\0\x81\x10a+UW`\x08\x91\x90\x91\x1B\x90`\x10\x1C[b\x01\0\0\x01\x02`\x12\x1C\x80\x82\x04\x01`\x01\x90\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x90\x81\x90\x04\x81\x11\x90\x03\x90V[`\0a\x04\xF4g\r\xE0\xB6\xB3\xA7d\0\0\x83a+\xAE\x86a.PV[a+\xB8\x91\x90a8\xB7V[a#d\x91\x90a8\xFDV[`\0\x80\x82\x12\x80a+\xD9WPg\x1B\xC1mgN\xC8\0\0\x82\x13[\x15a+\xF7W`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a,\x18W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81g\x1B\xC1mgN\xC8\0\0\x03a,@W`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x15a,KW\x91\x90PV[`\0g\r\xE0\xB6\xB3\xA7d\0\0\x83\x12a,sWa,n\x83g\x1B\xC1mgN\xC8\0\0a8\x90V[a,uV[\x82[\x90P`\0a,\x8B\x82g\x1B\xC1mgN\xC8\0\0a0bV[\x90P\x80`\0\x03a,\xAEW`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a,\xB9\x82a.PV[\x90P`\0c;\x9A\xCA\0a,\xE4a,\xDFa,\xD9g\x1B\xC1mgN\xC8\0\0a9+V[\x85a.;V[a*\xF2V[a,\xEE\x91\x90a8\xB7V[\x90P`\0\x80a-\x05\x83g\x03\xC1f\\z\xAB \0a.;V[a-\x17\x90g \x05\xFEO&\x8E\xA0\0a9GV[\x90P`\0a-B\x84a-0\x86f\x9F2u$b\xA0\0a.;V[a)\xDA\x90g\r\xC5R\x7Fd, \0a9GV[a-T\x90g\r\xE0\xB6\xB3\xA7d\0\0a9GV[\x90Pa-xg\t\xD0(\xCCo _\xFF\x19\x85a-n\x85\x85a0bV[a)\xDA\x91\x90a8\x90V[\x92PPP`\0[`\x02\x81\x10\x15a.\x13W`\0\x86a-\x94\x84a)\tV[a-\x9E\x91\x90a8\x90V[\x90P`\0a-\xAC\x84\x85a.;V[a-\xB5\x90a9+V[\x90P`\0a-\xC2\x82a\x1F\xB2V[\x90P`\0a-\xD0\x86\x85a.;V[a-\xE2g\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x84a.;V[a-\xEC\x91\x90a8\x90V[\x90Pa-\xF8\x84\x82a0bV[a.\x02\x90\x87a9GV[\x95P\x84`\x01\x01\x94PPPPPa-\x7FV[g\r\xE0\xB6\xB3\xA7d\0\0\x88\x12a.0Wa.+\x82a9+V[a\x06\x15V[P\x96\x95PPPPPPV[`\0a\x04\xF4\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a0sV[`\0\x80\x82\x13a.\x8DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x08#V[`\0``a.\x9A\x84a0\x92V[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[`\0`\x01`\xFF\x1B\x82\x03a0QW`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x12\x15a\x1B\xC1WP\x19`\x01\x01\x90V[`\0a\x04\xF4\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x05\x85\x14\x17\x16a0\x8BW`\0\x80\xFD[\x05\x92\x91PPV[`\0\x80\x82\x11a0\xCFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x08#V[P`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x82\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`\0\x80`@\x83\x85\x03\x12\x15a1\xA0Wa1\xA0a1:V[PP\x805\x92` \x90\x91\x015\x91PV[`\0[\x83\x81\x10\x15a1\xCAW\x81\x81\x01Q\x83\x82\x01R` \x01a1\xB2V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra1\xEB\x81` \x86\x01` \x86\x01a1\xAFV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x04\xF4` \x83\x01\x84a1\xD3V[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a2+Wa2+a1:V[PP\x825\x94` \x84\x015\x94P`@\x84\x015\x93``\x015\x92P\x90PV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FABI decoding: struct data too sh`D\x82\x01Rb\x1B\xDC\x9D`\xEA\x1B`d\x82\x01R`\x84\x81\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a2\xD1Wa2\xD1a2\x98V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a3\0Wa3\0a2\x98V[`@R\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a3\x1DW`\0\x80\xFD[PV[`\0\x80`\0\x83\x85\x03`\xE0\x81\x12\x15a39Wa39a1:V[\x845\x93P` \x85\x015\x92P`\xA0`?\x19\x82\x01\x12\x15a3YWa3Ya2GV[Pa3ba2\xAEV[`@\x85\x015\x81R``\x85\x015` \x82\x01R`\x80\x85\x015`@\x82\x01R`\xA0\x85\x015``\x82\x01R`\xC0\x85\x015a3\x95\x81a3\x08V[`\x80\x82\x01R\x92\x95\x91\x94P\x91\x92P\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a3\xBEWa3\xBEa1:V[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[\x80\x15\x15\x81\x14a3\x1DW`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15a3\xFBWa3\xFBa1:V[\x835\x92P` \x84\x015a4\r\x81a3\xD5V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[\x84\x15\x15\x81R\x83` \x82\x01R\x82`@\x82\x01R`\x80``\x82\x01R`\0a\x0B%`\x80\x83\x01\x84a1\xD3V[`\0` \x82\x84\x03\x12\x15a4ZWa4Za1:V[P5\x91\x90PV[\x81Q\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x80\x83\x01Q\x90\x82\x01R``\x80\x83\x01Q\x90\x82\x01R`\x80\x80\x83\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x90\x82\x01R`\xA0\x81\x01a\x04\xF7V[`\0` \x82\x84\x03\x12\x15a4\xB4Wa4\xB4a1:V[\x815a\x061\x81a3\x08V[\x82\x81R`@` \x82\x01R`\0a\x06.`@\x83\x01\x84a1\xD3V[`\0` \x82\x84\x03\x12\x15a4\xEDWa4\xEDa1:V[PQ\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x04\xF7Wa\x04\xF7a4\xF4V[\x81\x81\x03\x81\x81\x11\x15a\x04\xF7Wa\x04\xF7a4\xF4V[`\x01\x80`\xA0\x1B\x03\x84\x16\x81R\x82` \x82\x01R```@\x82\x01R`\0a\x14[``\x83\x01\x84a1\xD3V[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a5sWa5sa1:V[\x86Qa5~\x81a3\xD5V[` \x88\x01Q`@\x89\x01Q``\x8A\x01Q`\x80\x8B\x01Q`\xA0\x90\x9B\x01Q\x93\x9C\x92\x9BP\x90\x99\x90\x98P\x96P\x90\x94P\x92PPPV[`\0` \x80\x83\x85\x03\x12\x15a5\xC3Wa5\xC3a1:V[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a6&W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x84\x90R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a6\x8EW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x84\x90R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x81\xFD[\x81Q\x81\x81\x11\x15a6\xA0Wa6\xA0a2\x98V[a6\xB2`\x1F\x82\x01`\x1F\x19\x16\x85\x01a2\xD7V[\x91P\x80\x82R\x86\x84\x82\x85\x01\x01\x11\x15a7\x18W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x85\x90R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R`\x84\x81\xFD[a7'\x81\x85\x84\x01\x86\x86\x01a1\xAFV[P\x95\x94PPPPPV[`\0`\xA0\x82\x84\x03\x12\x15a7FWa7Fa2GV[a7Na2\xAEV[\x90P\x81Q\x81R` \x82\x01Q` \x82\x01R`@\x82\x01Q`@\x82\x01R``\x82\x01Q``\x82\x01R`\x80\x82\x01Qa7\x80\x81a3\x08V[`\x80\x82\x01R\x92\x91PPV[`\0`\xA0\x82\x84\x03\x12\x15a7\xA0Wa7\xA0a1:V[a\x04\xF4\x83\x83a71V[`\0` \x82\x84\x03\x12\x15a7\xBFWa7\xBFa1:V[\x81Qa\x061\x81a3\x08V[`\0\x80`\0``\x84\x86\x03\x12\x15a7\xE2Wa7\xE2a1:V[\x83Q\x92P` \x84\x01Q\x91P`@\x84\x01Q\x90P\x92P\x92P\x92V[`\x06\x81\x10a8\x19WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x90RV[``\x81\x01a8+\x82\x86a7\xFBV[` \x82\x01\x93\x90\x93R`@\x01R\x91\x90PV[\x84\x81R` \x81\x01\x84\x90R`@\x81\x01\x83\x90Ra\x01\0\x81\x01a\x14[``\x83\x01\x84\x80Q\x82R` \x80\x82\x01Q\x90\x83\x01R`@\x80\x82\x01Q\x90\x83\x01R``\x80\x82\x01Q\x90\x83\x01R`\x80\x90\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x91\x01RV[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a8\xB0Wa8\xB0a4\xF4V[P\x92\x91PPV[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15a8\xD3Wa8\xD3a4\xF4V[\x81\x81\x05\x83\x14\x82\x15\x17a\x04\xF7Wa\x04\xF7a4\xF4V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a9\x0CWa9\x0Ca8\xE7V[`\x01`\xFF\x1B\x82\x14`\0\x19\x84\x14\x16\x15a9&Wa9&a4\xF4V[P\x05\x90V[`\0`\x01`\xFF\x1B\x82\x01a9@Wa9@a4\xF4V[P`\0\x03\x90V[\x80\x82\x01\x82\x81\x12`\0\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a9gWa9ga4\xF4V[PP\x92\x91PPV[`@\x81\x01a9}\x82\x85a7\xFBV[\x82` \x83\x01R\x93\x92PPPV[`@\x81\x01a9\x98\x82\x85a7\xFBV[`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16` \x91\x90\x91\x01R\x91\x90PV[`\0\x80`\0\x80a\x01\0\x85\x87\x03\x12\x15a9\xCAWa9\xCAa1:V[\x84Q\x93P` \x85\x01Q\x92P`@\x85\x01Q\x91Pa9\xE9\x86``\x87\x01a71V[\x90P\x92\x95\x91\x94P\x92PV[`\0\x82a:\x03Wa:\x03a8\xE7V[P\x04\x90V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x04\xF7Wa\x04\xF7a4\xF4V\xFETarget contract does not contain\xA2dipfsX\"\x12 }\xB4\xEB\xE4\xBF\x1BO9\xF7\xD7\x84\x7FW\xB5s\x8C\xB8Q\x19\x99\xA8\x13no\x8B\xB0{\xEACt\x1E_dsolcC\0\x08\x16\x003";
    /// The bytecode of the contract.
    pub static LOGNORMALSOLVER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x02\x11W`\x005`\xE0\x1C\x80cme\"\x99\x11a\x01FW\x80c\xCB\x1FU2\x11a\0\xE4W\x80c\xE9G\x16\xD5\x11a\0\xBEW\x80c\xE9G\x16\xD5\x14a\x04\xA7W\x80c\xEE>\x8C\xFB\x14a\x04\xBAW\x80c\xF3\r7\xF2\x14a\x04\xCDW\x80c\xF9\xC2\x82\x11\x14a\x04\xE0Wa\x02\x11V[\x80c\xCB\x1FU2\x14a\x04nW\x80c\xCE\x15;\xF4\x14a\x04\x81W\x80c\xE4]Y<\x14a\x04\x94Wa\x02\x11V[\x80c\x90.\xCA\xA2\x11a\x01 W\x80c\x90.\xCA\xA2\x14a\x04\nW\x80c\xA8\xC6.v\x14a\x04\x1DW\x80c\xAFNC\x7F\x14a\x04HW\x80c\xB0\x9D\x04\xE5\x14a\x04[Wa\x02\x11V[\x80cme\"\x99\x14a\x03\xCFW\x80c\x7F\x17@\x9C\x14a\x03\xD7W\x80c\x81\xB5\xFA\xC2\x14a\x03\xEAWa\x02\x11V[\x80c;&\x8D]\x11a\x01\xB3W\x80cO\xD6|X\x11a\x01\x8DW\x80cO\xD6|X\x14a\x03hW\x80cU\xF0\x11\xC6\x14a\x03{W\x80c^\xB4\x08\xFC\x14a\x03\x8EW\x80cb7V\x9F\x14a\x03\xA1Wa\x02\x11V[\x80c;&\x8D]\x14a\x03/W\x80c;M\x100\x14a\x03BW\x80cN\x81\x7F\xD9\x14a\x03UWa\x02\x11V[\x80c\x1E\x97\x8C\xB0\x11a\x01\xEFW\x80c\x1E\x97\x8C\xB0\x14a\x02\xD3W\x80c0m\xB4k\x14a\x02\xE6W\x80c3\"f\xF3\x14a\x02\xF9W\x80c9(\xFF\x97\x14a\x03\x0CWa\x02\x11V[\x80c\x04 X\n\x14a\x02vW\x80c\x12\x06I\xC5\x14a\x02\x9FW\x80c\x13N\xAD\x12\x14a\x02\xC0W[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x02\x89a\x02\x846`\x04a1\x8AV[a\x04\xE8V[`@Qa\x02\x96\x91\x90a1\xFFV[`@Q\x80\x91\x03\x90\xF3[a\x02\xB2a\x02\xAD6`\x04a2\x12V[a\x04\xFDV[`@Q\x90\x81R` \x01a\x02\x96V[a\x02\x89a\x02\xCE6`\x04a3 V[a\x06!V[a\x02\xB2a\x02\xE16`\x04a3\xA6V[a\x068V[a\x02\xB2a\x02\xF46`\x04a3\xA6V[a\x06MV[a\x02\xB2a\x03\x076`\x04a3\xA6V[a\x06\x84V[a\x03\x1Fa\x03\x1A6`\x04a3\xE3V[a\x06\xB0V[`@Qa\x02\x96\x94\x93\x92\x91\x90a4\x1EV[a\x02\x89a\x03=6`\x04a1\x8AV[a\n\x84V[a\x02\xB2a\x03P6`\x04a4EV[a\n\x90V[a\x02\xB2a\x03c6`\x04a3\xA6V[a\n\xB9V[a\x02\xB2a\x03v6`\x04a3\xA6V[a\n\xCEV[a\x02\xB2a\x03\x896`\x04a1\x8AV[a\n\xFAV[a\x02\xB2a\x03\x9C6`\x04a2\x12V[a\x0B/V[a\x03\xB4a\x03\xAF6`\x04a1\x8AV[a\x0CGV[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x02\x96V[a\x02\xB2`\0\x81V[a\x03\xB4a\x03\xE56`\x04a1\x8AV[a\x0C\xA1V[a\x03\xFDa\x03\xF86`\x04a4EV[a\x0C\xFAV[`@Qa\x02\x96\x91\x90a4aV[a\x02\xB2a\x04\x186`\x04a3\xA6V[a\x0E\x06V[`\0Ta\x040\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\x96V[a\x02\xB2a\x04V6`\x04a2\x12V[a\x0E2V[a\x02\x89a\x04i6`\x04a4EV[a\x0F4V[a\x02\x89a\x04|6`\x04a4\x9FV[a\x0F?V[a\x03\xB4a\x04\x8F6`\x04a4EV[a\x0FJV[a\x02\xB2a\x04\xA26`\x04a1\x8AV[a\x10\xDAV[a\x02\x89a\x04\xB56`\x04a1\x8AV[a\x11\x05V[a\x03\xB4a\x04\xC86`\x04a1\x8AV[a\x11\x11V[a\x03\xB4a\x04\xDB6`\x04a1\x8AV[a\x117V[a\x02\xB2`x\x81V[``a\x04\xF4\x83\x83a\x11]V[\x90P[\x92\x91PPV[`\0\x80a\x05\x13\x84\x84a\x05\x0E\x89a\x0C\xFAV[a\x11\x8CV[`@\x80Q` \x81\x01\x88\x90R\x80\x82\x01\x83\x90R``\x80\x82\x01\x88\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x80\x82\x01\x92\x83\x90R`\0\x80Tb.RK`\xE0\x1B\x90\x94R\x93\x94P\x92\x91`\x01`\x01`\xA0\x1B\x03\x16\x90b.RK\x90a\x05p\x90\x8B\x90\x86\x90`\x84\x01a4\xBFV[` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x05\xC6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a: \x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x05\xDAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xFE\x91\x90a4\xD8V[\x90Pa\x06\x15\x87\x87\x83\x86a\x06\x10\x8Da\x0C\xFAV[a\x11\xCEV[\x98\x97PPPPPPPPV[``a\x06.\x84\x84\x84a\x12\x7FV[\x90P[\x93\x92PPPV[`\0a\x06.\x83\x83a\x06H\x87a\x0C\xFAV[a\x12\xF0V[`\0\x80a\x06Y\x85a\x0C\xFAV[\x90P`\0\x80a\x06g\x87a\x0FJV[\x92PP\x91Pa\x06y\x86\x83\x83\x88\x87a\x13\xF9V[\x97\x96PPPPPPPV[`\0\x80a\x06\x90\x85a\x0C\xFAV[\x90P`\0\x80a\x06\x9E\x87a\x0FJV[\x92PP\x91Pa\x06y\x86\x83\x83\x88\x87a\x14dV[`\0\x80`\0``a\x06\xDB`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[a\x06\xFF`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[a\x07\x08\x89a\x0FJV[`@\x85\x01R` \x84\x01R\x82R`\0a\x07\x1F\x8Aa\x0C\xFAV[\x90P`\0\x80a\x07<\x8C\x86`\0\x01Q\x87` \x01Q\x88`@\x01Qa\x0E2V[\x90P\x8A\x15a\x08IW`\0a\x07]\x84``\x01Q\x8Ca\x14\xE0\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x86Q\x90\x91Pa\x07m\x90\x8C\x90a5\nV[\x85Ra\x07y\x81\x83a5\nV[\x85`@\x01\x81\x81RPP`\0a\x07\x97\x8E\x87`\0\x01Q\x88`@\x01Qa\x068V[\x90Pa\x07\xAD\x8E\x87`\0\x01Q\x88`@\x01Q\x84a\x04\xFDV[` \x87\x01\x81\x81R`\x01\x91a\x07\xC2\x90\x83\x90a5\nV[\x90RP` \x80\x88\x01Q\x90\x87\x01Q\x10a\x08,W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: y reserve increase`D\x82\x01Rad!`\xF0\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[\x85` \x01Q\x87` \x01Qa\x08@\x91\x90a5\x1DV[\x93PPPa\tBV[\x82Q``\x84\x01Q`\0\x91a\x08h\x91a\x08b\x90\x8E\x90a\x14\xE0V[\x90a\x14\xF5V[\x90P\x8A\x86` \x01Qa\x08z\x91\x90a5\nV[` \x86\x01Ra\x08\x89\x81\x83a5\nV[\x85`@\x01\x81\x81RPP`\0a\x08\xA7\x8E\x87` \x01Q\x88`@\x01Qa\n\xB9V[\x90Pa\x08\xBD\x8E\x87` \x01Q\x88`@\x01Q\x84a\x0B/V[\x80\x87R`\x01\x90\x87\x90a\x08\xD0\x90\x83\x90a5\nV[\x90RP\x86Q\x86Q\x10a\t/W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: x reserve increase`D\x82\x01Rad!`\xF0\x1B`d\x82\x01R`\x84\x01a\x08#V[\x85Q\x87Qa\t=\x91\x90a5\x1DV[\x93PPP[P\x82Q` \x80\x85\x01Q`@\x80\x87\x01Q\x81Q\x93\x84\x01\x94\x90\x94R\x82\x01R``\x81\x01\x91\x90\x91R`\0\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16ch\xBD>80\x8F\x85`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\t\xC2\x93\x92\x91\x90a50V[`\xC0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\n\x18W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a: \x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\n,W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\nP\x91\x90a5WV[PPPPP\x90P\x80\x83a\nl\x87`\0\x01Q\x88`@\x01Q\x88a\x12\xF0V[\x84\x99P\x99P\x99P\x99PPPPPPP\x93P\x93P\x93P\x93V[``a\x04\xF4\x83\x83a\x15\nV[`\0\x80`\0a\n\x9E\x84a\x0FJV[\x92PP\x91Pa\n\xB1\x82\x82a\x06H\x87a\x0C\xFAV[\x94\x93PPPPV[`\0a\x06.\x83\x83a\n\xC9\x87a\x0C\xFAV[a\x15\"V[`\0\x80a\n\xDA\x85a\x0C\xFAV[\x90P`\0\x80a\n\xE8\x87a\x0FJV[\x92P\x92PPa\x06y\x86\x83\x83\x88\x87a\x15\xF0V[`\0\x80a\x0B\x06\x84a\x0C\xFAV[\x90P`\0\x80a\x0B\x14\x86a\x0FJV[\x92PP\x91Pa\x0B%\x85\x83\x83\x86a\x16RV[\x96\x95PPPPPPV[`\0\x80a\x0BE\x84\x84a\x0B@\x89a\x0C\xFAV[a\x17<V[`@\x80Q` \x81\x01\x83\x90R\x80\x82\x01\x88\x90R``\x80\x82\x01\x88\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x80\x82\x01\x92\x83\x90R`\0\x80Tb.RK`\xE0\x1B\x90\x94R\x93\x94P\x92\x91`\x01`\x01`\xA0\x1B\x03\x16\x90b.RK\x90a\x0B\xA2\x90\x8B\x90\x86\x90`\x84\x01a4\xBFV[` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0B\xF8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a: \x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0C\x0CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C0\x91\x90a4\xD8V[\x90Pa\x06\x15\x87\x87\x83\x86a\x0CB\x8Da\x0C\xFAV[a\x17\x81V[`\0\x80`\0\x80`\0a\x0CX\x87a\x0FJV[\x92PP\x91P`\0\x80a\x0Cm`\0\x89\x86\x86a\x18%V[\x91P\x91P`\0a\x0C~\x8A\x84\x84a\x068V[\x90P`\0a\x0C\x8E\x8B\x85\x85\x85a\x04\xFDV[\x93\x9B\x93\x9AP\x91\x98P\x91\x96PPPPPPPV[`\0\x80`\0\x80`\0a\x0C\xB2\x87a\x0FJV[\x92P\x92PP`\0\x80a\x0C\xC7`\x01\x89\x86\x86a\x18%V[\x91P\x91P`\0a\x0C\xD8\x8A\x84\x84a\n\xB9V[\x90P`\0a\x0C\xE8\x8B\x85\x85\x85a\x0B/V[\x9B\x93\x9AP\x91\x98P\x91\x96PPPPPPPV[a\r5`@Q\x80`\xA0\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81RP\x90V[`\0T`@Qc\xDC\x17\x83U`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xDC\x17\x83U\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\r\xB7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a: \x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\r\xCBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\r\xF3\x91\x90\x81\x01\x90a5\xADV[\x80` \x01\x90Q\x81\x01\x90a\x04\xF7\x91\x90a7\x8BV[`\0\x80a\x0E\x12\x85a\x0C\xFAV[\x90P`\0\x80a\x0E \x87a\x0FJV[\x92P\x92PPa\x06y\x86\x83\x83\x88\x87a\x18\x8EV[`@\x80Q` \x81\x01\x85\x90R\x80\x82\x01\x84\x90R``\x80\x82\x01\x84\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x80\x82\x01\x92\x83\x90R`\0\x80Tb.RK`\xE0\x1B\x90\x94R\x92\x90\x91\x83\x91`\x01`\x01`\xA0\x1B\x03\x16\x90b.RK\x90a\x0E\x8F\x90\x8A\x90\x86\x90`\x84\x01a4\xBFV[` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0E\xE5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a: \x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0E\xF9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\x1D\x91\x90a4\xD8V[\x90Pa\x06y\x86\x86\x83\x87a\x0F/\x8Ca\x0C\xFAV[a\x18\xEAV[``a\x04\xF7\x82a\x19\x8EV[``a\x04\xF7\x82a\x19\xBAV[`\0\x80`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xAF\xBA\x13\xC4`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0F\xDAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a: \x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0F\xEEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\x12\x91\x90a7\xAAV[`\x01`\x01`\xA0\x1B\x03\x16c\xCE\x15;\xF4\x85`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x10?\x91\x81R` \x01\x90V[```@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x10\x95W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a: \x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x10\xA9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\xCD\x91\x90a7\xCAV[\x92P\x92P\x92P\x91\x93\x90\x92PV[`\0\x80a\x10\xE6\x84a\x0C\xFAV[\x90P`\0\x80a\x10\xF4\x86a\x0FJV[\x92P\x92PPa\x0B%\x85\x83\x83\x86a\x19\xD0V[``a\x04\xF4\x83\x83a\x1A\x9AV[`\0\x80`\0\x80`\0a\x11\"\x87a\x0FJV[\x92PP\x91P`\0\x80a\x0Cm`\x01\x89\x86\x86a\x18%V[`\0\x80`\0\x80`\0a\x11H\x87a\x0FJV[\x92P\x92PP`\0\x80a\x0C\xC7`\0\x89\x86\x86a\x18%V[```\x02\x83\x83`@Q` \x01a\x11u\x93\x92\x91\x90a8\x1DV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x92\x91PPV[`\0\x80a\x11\x99\x84\x84a\x1A\xB2V[\x90P`\0a\x11\xA6\x82a\x1B\x13V[\x90P`\0a\x11\xB3\x82a\x1B|V[\x85Q\x90\x91Pa\x06y\x90\x82\x90a\x11\xC8\x90\x8Aa\x14\xE0V[\x90a\x14\xE0V[`\0\x82\x80\x85\x83\x81\x12\x15a\x12\x0EW[`\0\x81\x12\x15a\x12\tWa\x11\xF4\x83a\x03\xE9a\x03\xE8a\x1B\xC5V[\x92Pa\x12\x02\x89\x84\x8A\x88a\x1B\xF3V[\x90Pa\x11\xDCV[a\x12;V[`\0\x81\x13\x15a\x12;Wa\x12&\x82a\x03\xE7a\x03\xE8a\x1C\xF4V[\x91Pa\x124\x89\x83\x8A\x88a\x1B\xF3V[\x90Pa\x12\x0EV[a\x12r\x89\x89\x83\x88`@Q` \x01a\x12U\x94\x93\x92\x91\x90a8<V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83\x85`\x14`@a\x1D\x13a\x1D@V[\x99\x98PPPPPPPPPV[```\0a\x12\x8E\x85\x85\x85a\x1EQV[\x90P`\0a\x12\x9D\x82\x86\x86a\x11\x8CV[\x90P`\0a\x12\xAD\x87\x83\x85\x88a\x1B\xF3V[\x90Pa\x12\xBC\x87\x83\x83\x86\x89a\x18\xEAV[\x92P\x86\x82\x84\x87`@Q` \x01a\x12\xD5\x94\x93\x92\x91\x90a8<V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x93PPPP\x93\x92PPPV[`\0\x80a\x13\x05\x83` \x01Q\x84`@\x01Qa\x1E\x96V[\x90P`\0a\x13\x1B\x84` \x01Q\x85`@\x01Qa\x1E\xBCV[\x90P`\0a\x136\x85`@\x01Q\x83a\x1E\xEB\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x13D\x88\x88a\x1F\0V[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0\x81\x12a\x13bW`\0\x94PPPPPa\x061V[`\0\x81\x13a\x13xW`\0\x19\x94PPPPPa\x061V[`\0a\x13\x94a\x13\x8F\x83g\r\xE0\xB6\xB3\xA7d\0\0a8\x90V[a\x1F\x15V[\x90P`\0\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x13\xAC\x88\x85a8\xB7V[a\x13\xB6\x91\x90a8\xFDV[a\x13\xC0\x91\x90a8\x90V[\x90P`\0a\x13\xCD\x82a\x1F\xB2V[\x90P`\0a\x13\xDA\x82a\x1B|V[\x8AQ\x90\x91Pa\x13\xE9\x90\x82a\x14\xE0V[\x9C\x9BPPPPPPPPPPPPV[`\0\x82`d\x82a\x14\x0C\x89\x89\x89\x85\x89a\x14dV[\x90P`\0\x81\x12\x15a\x14#W`\0\x93PPPPa\x14[V[a\x12r\x89\x89\x89\x88`@Q` \x01a\x14=\x94\x93\x92\x91\x90a8<V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83\x85`\x01a\x01\0a![a\x1D@V[\x95\x94PPPPPV[`\0\x80\x82``\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x14\x7F\x91\x90a8\x90V[\x90P`\0a\x14\x91\x88\x88\x88\x85\x89\x89a!\x8CV[\x90P`\0a\x14\x9E\x82a\"\xE7V[\x90P`\0a\x14\xAB\x83a$\x10V[\x90P\x80\x82\x84a\x01\0\x01Qa\x14\xBE\x90a9+V[a\x14\xC8\x91\x90a9GV[a\x14\xD2\x91\x90a9GV[\x9A\x99PPPPPPPPPPV[`\0a\x04\xF4\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x1B\xC5V[`\0a\x04\xF4\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x1B\xC5V[```\x04\x83\x83`@Q` \x01a\x11u\x93\x92\x91\x90a8\x1DV[`\0\x80a\x157\x83` \x01Q\x84`@\x01Qa\x1E\x96V[\x90P`\0a\x15M\x84` \x01Q\x85`@\x01Qa\x1E\xBCV[\x90P`\0a\x15h\x85`@\x01Q\x83a\x1E\xEB\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x85Q\x90\x91P`\0\x90a\x15\x85\x90a\x15~\x90\x89a\x1E\xEBV[\x89\x90a\x1F\0V[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0\x81\x12a\x15\xA3W`\0\x94PPPPPa\x061V[`\0\x81\x13a\x15\xB9W`\0\x19\x94PPPPPa\x061V[`\0a\x15\xC4\x82a\x1F\x15V[\x90P`\0\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x15\xDC\x88\x85a8\xB7V[a\x15\xE6\x91\x90a8\xFDV[a\x13\xC0\x91\x90a9GV[`\0\x82`d\x82a\x16\x03\x89\x89\x89\x85\x89a\x18\x8EV[\x90P`\0\x81\x12\x15a\x16\x1AW`\0\x93PPPPa\x14[V[a\x12r\x89\x89\x89\x88`@Q` \x01a\x164\x94\x93\x92\x91\x90a8<V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83\x85`\x01a\x01\0a$\x9Ca\x1D@V[`\0\x80\x82``\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x16m\x91\x90a8\x90V[\x90P`\0\x83` \x01Q\x90P`\0a\x16\x88\x88\x86`\0\x01Qa$\xCDV[\x90P`\0a\x16\xBAa\x16\xA1\x84g\x1B\xC1mgN\xC8\0\0a$\xE1V[a\x16\xAB\x84\x86a$\xE1V[a\x16\xB5\x91\x90a9GV[a\x1B\x13V[\x90P`\0a\x16\xDAa\x16\xD3\x83g\r\xE0\xB6\xB3\xA7d\0\0a8\x90V[\x89\x90a%\x05V[\x90Pa\x14\xD2g\r\xE0\xB6\xB3\xA7d\0\0a\x17(a\x16\xF5\x8C\x8Ca$\xE1V[a\x17\"a\x17\n\x87g\r\xE0\xB6\xB3\xA7d\0\0a8\x90V[a\x17\x1Cg\r\xE0\xB6\xB3\xA7d\0\0\x8Ca8\x90V[\x90a%\x05V[\x90a$\xE1V[a\x172\x91\x90a9GV[a\x17\"\x8B\x84a8\x90V[`\0\x80a\x17I\x84\x84a%8V[\x90P`\0a\x17V\x82a\x1B\x13V[\x90P`\0a\x17c\x82a\x1B|V[\x90Pa\x06ya\x17z\x82g\r\xE0\xB6\xB3\xA7d\0\0a5\x1DV[\x88\x90a\x14\xE0V[`\0\x82\x80\x85\x83\x81\x12\x15a\x17\xC1W[`\0\x81\x12\x15a\x17\xBCWa\x17\xA7\x83a\x03\xE9a\x03\xE8a\x1B\xC5V[\x92Pa\x17\xB5\x83\x8A\x8A\x88a\x1B\xF3V[\x90Pa\x17\x8FV[a\x17\xEEV[`\0\x81\x13\x15a\x17\xEEWa\x17\xD9\x82a\x03\xE7a\x03\xE8a\x1C\xF4V[\x91Pa\x17\xE7\x82\x8A\x8A\x88a\x1B\xF3V[\x90Pa\x17\xC1V[a\x12r\x89\x89\x83\x88`@Q` \x01a\x18\x08\x94\x93\x92\x91\x90a8<V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83\x85`\x14`@a%}a\x1D@V[`\0\x80\x80a\x183\x84\x86a\x14\xF5V[\x90P`\0a\x18A\x87\x83a\x14\xE0V[\x90P\x87a\x18WWa\x18R\x87\x87a5\x1DV[a\x18aV[a\x18a\x87\x87a5\nV[\x93P\x87a\x18wWa\x18r\x81\x86a5\x1DV[a\x18\x81V[a\x18\x81\x81\x86a5\nV[\x92PPP\x94P\x94\x92PPPV[`\0\x80\x82``\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x18\xA9\x91\x90a8\x90V[\x90P`\0a\x18\xBB\x88\x88\x88\x85\x89\x89a%\xAAV[\x90P`\0a\x18\xC8\x82a'\x02V[\x90P`\0a\x18\xD5\x83a(0V[\x90P\x80\x82a\x14\xBEg\r\xE0\xB6\xB3\xA7d\0\0a9+V[`\0\x82\x80\x85\x83\x81\x12\x15a\x19*W[`\0\x81\x12\x15a\x19%Wa\x19\x10\x82a\x03\xE7a\x03\xE8a\x1C\xF4V[\x91Pa\x19\x1E\x89\x89\x84\x88a\x1B\xF3V[\x90Pa\x18\xF8V[a\x19WV[`\0\x81\x13\x15a\x19WWa\x19B\x83a\x03\xE9a\x03\xE8a\x1B\xC5V[\x92Pa\x19P\x89\x89\x85\x88a\x1B\xF3V[\x90Pa\x19*V[a\x12r\x89\x89\x83\x88`@Q` \x01a\x19q\x94\x93\x92\x91\x90a8<V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83\x85`\x14`@a(\xDCa\x1D@V[```\x01\x82`@Q` \x01a\x19\xA4\x92\x91\x90a9oV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x91\x90PV[```\x05\x82`@Q` \x01a\x19\xA4\x92\x91\x90a9\x8AV[`\0\x80\x82``\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x19\xEB\x91\x90a8\x90V[\x83Q` \x85\x01Q\x91\x92P\x90`\0a\x1A\x02\x89\x84a$\xCDV[\x90P`\0a\x1A\x18\x83g\x1B\xC1mgN\xC8\0\0a$\xE1V[a\x1A\"\x83\x85a$\xE1V[a\x1A,\x91\x90a8\x90V[\x90P`\0a\x1A9\x82a\x1B\x13V[\x90P`\0a\x1AK\x82a\x17\x1C\x8C\x89a%\x05V[\x90Pa\x13\xE9g\r\xE0\xB6\xB3\xA7d\0\0a\x1A\x86a\x1Apa\x1Ai\x8A\x8Fa%\x05V[\x8F\x90a$\xE1V[a\x17\"\x86a\x17\x1Cg\r\xE0\xB6\xB3\xA7d\0\0\x8Ea8\x90V[a\x1A\x90\x91\x90a9GV[a\x17\"\x8D\x84a8\x90V[```\x03\x83\x83`@Q` \x01a\x11u\x93\x92\x91\x90a8\x1DV[\x80Q` \x82\x01Q`@\x83\x01Q`\0\x92\x91\x90\x83a\x1A\xCE\x83\x83a\x1E\x96V[\x90P`\0a\x1A\xDC\x88\x86a$\xCDV[\x90P`\0a\x1A\xEA\x85\x85a\x1E\xBCV[\x90P\x82a\x1A\xF7\x82\x84a8\x90V[a\x1B\t\x90g\r\xE0\xB6\xB3\xA7d\0\0a8\xB7V[a\x12r\x91\x90a8\xFDV[`\0\x80g\x13\xA0K\xBD\xFD\xC9\xBE\x88a\x1B1g\r\xE0\xB6\xB3\xA7d\0\0\x85a8\xB7V[a\x1B;\x91\x90a8\xFDV[\x90P`\0a\x1BH\x82a9+V[\x90P`\0a\x1BU\x82a)\tV[\x90Pg\x1B\xC1mgN\xC8\0\0a\x1Brg\r\xE0\xB6\xB3\xA7d\0\0\x83a8\xB7V[a\x14[\x91\x90a8\xFDV[`\0\x80\x82\x12\x15a\x1B\xC1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01RotoUint: negative`\x80\x1B`D\x82\x01R`d\x01a\x08#V[P\x90V[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x1B\xDDW`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[`\0\x82\x85\x10a\x1CDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FtradingFunction: invalid x\0\0\0\0\0\0`D\x82\x01R`d\x01a\x08#V[`\0\x80g\r\xE0\xB6\xB3\xA7d\0\0a\x1CZ\x88\x87a\x1F\0V[\x10a\x1CnW`\x01`\x01`\xFF\x1B\x03\x91Pa\x1C~V[a\x1C{a\x13\x8F\x88\x87a\x1F\0V[\x91P[g\r\xE0\xB6\xB3\xA7d\0\0a\x1C\x9E\x87a\x1C\x99\x87`\0\x01Q\x89a\x1E\xEBV[a\x1F\0V[\x10a\x1C\xB1WP`\x01`\x01`\xFF\x1B\x03a\x1C\xC9V[a\x1C\xC6a\x13\x8F\x87a\x1C\x99\x87`\0\x01Q\x89a\x1E\xEBV[\x90P[`\0a\x1C\xDD\x85` \x01Q\x86`@\x01Qa\x1E\x96V[\x90P\x80a\x1C\xEA\x83\x85a9GV[a\x06\x15\x91\x90a9GV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x1D\x0CW`\0\x80\xFD[\x04\x92\x91PPV[`\0\x80`\0\x80\x85\x80` \x01\x90Q\x81\x01\x90a\x1D-\x91\x90a9\xB0V[\x93PP\x92P\x92Pa\x0B%\x83\x86\x84\x84a\x1B\xF3V[`\0\x84\x86\x11\x15a\x1DmW`@Qc0\x82\xDF\xDB`\xE1\x1B\x81R`\x04\x81\x01\x87\x90R`$\x81\x01\x86\x90R`D\x01a\x08#V[`\0a\x1D}\x88\x88\x85c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x1D\x8F\x89\x88\x86c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x1D\x9D\x82\x84a8\xB7V[\x13\x15a\x1D\xC6W`@Qc\x06\xF1\xBE]`\xE2\x1B\x81R`\x04\x81\x01\x83\x90R`$\x81\x01\x82\x90R`D\x01a\x08#V[`\0a\x1D\xD2\x89\x89a5\x1DV[\x90P`\0[`\x02a\x1D\xE3\x8A\x8Ca5\nV[a\x1D\xED\x91\x90a9\xF4V[\x94P`\0a\x1D\xFF\x8C\x87\x89c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x1E\r\x86\x83a8\xB7V[\x13a\x1E\x1AW\x85\x99Pa\x1E!V[\x85\x9AP\x80\x94P[a\x1E+\x8B\x8Ba5\x1DV[\x92PP`\x01\x01\x87\x82\x11\x80\x15a\x1E?WP\x86\x81\x10[a\x1D\xD7WPPPP\x96\x95PPPPPPV[`\0\x80a\x1E^\x84\x84a%8V[\x90P`\0a\x1Ek\x82a\x1B\x13V[\x90P`\0a\x1Ex\x82a\x1B|V[\x90Pa\x06ya\x1E\x8F\x82g\r\xE0\xB6\xB3\xA7d\0\0a5\x1DV[\x88\x90a\x14\xF5V[`\0\x80a\x1E\xA2\x83a*\xF2V[a\x1E\xB0\x90c;\x9A\xCA\0a:\x08V[\x90Pa\n\xB1\x84\x82a\x1E\xEBV[`\0\x80a\x1E\xDB\x83a\x1E\xD5\x86g\x1B\xC1mgN\xC8\0\0a+\x96V[\x90a\x1E\xEBV[\x90Pa\n\xB1g\x06\xF0[Y\xD3\xB2\0\0\x82[`\0a\x04\xF4\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x1C\xF4V[`\0a\x04\xF4\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x1C\xF4V[`\0g\x06\xF0[Y\xD3\xB2\0\0\x82\x03a\x1F.WP`\0\x91\x90PV[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x12a\x1FVW`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\x1FwW`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x1F\x84\x83`\x02a8\xB7V[\x90P`\0a\x1F\x91\x82a+\xC2V[\x90P`\0a\x1F\xA7g\x13\xA0K\xBD\xFD\xC9\xBE\x88\x83a.;V[\x90Pa\x14[\x81a9+V[`\0h\x02H\xCE6\xA7\x0C\xB2k>\x19\x82\x13a\x1F\xCDWP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a \x14W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x01a\x08#V[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05`\x01`_\x1B\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1Dm\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1Dn\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[`\0\x80`\0\x80`\0\x86\x80` \x01\x90Q\x81\x01\x90a!w\x91\x90a9\xB0V[\x93P\x93P\x93P\x93Pa\x06y\x84\x84\x84\x89\x85a\x14dV[a!\xE9`@Q\x80a\x01`\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`\0a\"\x07a!\xF8\x88\x86a9GV[g\x1B\xC1mgN\xC8\0\0\x90a%\x05V[\x90P`\0a\"\x15\x85\x87a%\x05V[a\"\x1F\x86\x89a9GV[a\")\x91\x90a8\x90V[\x90P`\0a\"?a\":\x84\x84a$\xE1V[a+\xC2V[\x90P`\0a\"Tg\x1B\xC1mgN\xC8\0\0a*\xF2V[a\"b\x90c;\x9A\xCA\0a:\x08V[\x90P`\0a\"s\x87`@\x01Qa*\xF2V[a\"\x81\x90c;\x9A\xCA\0a:\x08V[`@\x80Qa\x01`\x81\x01\x82R\x94\x85R\x88Q` \x80\x87\x01\x91\x90\x91R\x89\x01Q\x85\x82\x01R\x97\x90\x97\x01Q``\x84\x01RP`\x80\x82\x01\x97\x90\x97R`\xA0\x81\x01\x98\x90\x98RPPP`\xC0\x85\x01\x93\x90\x93R`\xE0\x84\x01Ra\x01\0\x83\x01\x93\x90\x93Ra\x01 \x82\x01Ra\x01@\x81\x01\x91\x90\x91R\x90V[`\0\x80a#\x19g\x1B\xC1mgN\xC8\0\0a\x17\"\x85``\x01Qa\x17\x1C\x87`@\x01Q\x88`@\x01Qa%\x05\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a#\"\x90a9+V[\x90P`\0a#S\x84`\0\x01Qa\x17\x1C\x86a\x01 \x01Qa\x17\x1C\x88`@\x01Q\x89a\x01@\x01Qa%\x05\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a#ia#d\x83\x85a9GV[a\x1F\xB2V[\x90P`\0a#\xB9a#\x9F\x87`\x80\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a#\x8A\x90a9+V[a#\x94\x91\x90a9GV[`\xA0\x89\x01Q\x90a%\x05V[\x87`\xC0\x01Qa#\xAE\x91\x90a9GV[` \x88\x01Q\x90a%\x05V[\x90P`\0a#\xC7\x83\x83a%\x05V[\x90P`\0a#\xE6\x88`\x80\x01Q\x89`\xE0\x01Qa%\x05\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x88`\xE0\x01Q\x89`\xC0\x01Qa#\xFA\x91\x90a9GV[a$\x04\x91\x90a8\x90V[\x90Pa\x06\x15\x82\x82a$\xE1V[`\0\x80a$O\x83`\x80\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a$-\x90a9+V[a$7\x91\x90a9GV[` \x85\x01Qa\x17\x1C\x90g\x06\xF0[Y\xD3\xB2\0\0\x90a%\x05V[\x90P`\0a$x\x84a\x01@\x01Qa\x17\"\x86a\x01 \x01Q\x87`@\x01Qa%\x05\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa\n\xB1a$\x95\x85`\0\x01Q\x83a$\x90\x91\x90a8\x90V[a)\tV[\x83\x90a%\x05V[`\0\x80`\0\x80`\0\x86\x80` \x01\x90Q\x81\x01\x90a$\xB8\x91\x90a9\xB0V[\x93P\x93P\x93P\x93Pa\x06y\x84\x84\x84\x89\x85a\x18\x8EV[`\0a\x04\xF4a$\xDC\x84\x84a\x14\xF5V[a.PV[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x81\x02\x90\x81\x05\x83\x14\x82\x15\x15\x16a$\xFFW`\0\x80\xFD[\x05\x91\x90PV[\x81\x81\x02\x82\x15\x83\x82\x05\x83\x14\x17`\0\x19\x84\x10`\x01`\xFF\x1B\x84\x13\x17\x16a%'W`\0\x80\xFD[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x92\x91PPV[\x80Q` \x82\x01Q`@\x83\x01Q`\0\x92\x91\x90\x83a%T\x83\x83a\x1E\x96V[\x90P`\0a%b\x88\x86a$\xCDV[\x90P`\0a%p\x85\x85a\x1E\xBCV[\x90P\x82a\x1A\xF7\x82\x84a9GV[`\0\x80`\0\x80\x85\x80` \x01\x90Q\x81\x01\x90a%\x97\x91\x90a9\xB0V[\x93PP\x92P\x92Pa\x0B%\x85\x84\x84\x84a\x1B\xF3V[a&\x07`@Q\x80a\x01`\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`\0a&\x16a!\xF8\x88\x86a9GV[\x90P`\0a&$\x85\x87a%\x05V[\x84Q\x86\x90a&2\x90\x8Aa%\x05V[a&<\x91\x90a9GV[a&F\x91\x90a8\x90V[\x90P`\0a&Wa\":\x84\x84a$\xE1V[\x90P`\0a&lg\x1B\xC1mgN\xC8\0\0a*\xF2V[a&z\x90c;\x9A\xCA\0a:\x08V[\x90P`\0a&\x8B\x87`@\x01Qa*\xF2V[a&\x99\x90c;\x9A\xCA\0a:\x08V[`@\x80Qa\x01`\x81\x01\x82R\x94\x85R\x88Q` \x80\x87\x01\x91\x90\x91R\x89\x01Q\x85\x82\x01R\x97\x90\x97\x01Q``\x84\x01RP`\x80\x82\x01\x97\x90\x97R`\xA0\x81\x01\x98\x90\x98RPPP`\xC0\x85\x01\x93\x90\x93R`\xE0\x84\x01Ra\x01\0\x83\x01\x93\x90\x93Ra\x01 \x82\x01\x92\x90\x92Ra\x01@\x81\x01\x91\x90\x91R\x90V[`\0\x80a'4g\x1B\xC1mgN\xC8\0\0a\x17\"\x85``\x01Qa\x17\x1C\x87`@\x01Q\x88`@\x01Qa%\x05\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a'=\x90a9+V[\x90P`\0a'n\x84`\0\x01Qa\x17\x1C\x86a\x01@\x01Qa\x17\x1C\x88`@\x01Q\x89a\x01 \x01Qa%\x05\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a'\x7Fa#d\x83\x85a9GV[\x90P`\0a'\xC9a'\xA0\x87`\x80\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a#\x8A\x90a9+V[`\xC0\x88\x01Q` \x89\x01Qa'\xB3\x91a%\x05V[a'\xBD\x91\x90a9GV[a\x01\0\x88\x01Q\x90a%\x05V[\x90P`\0a'\xD7\x83\x83a%\x05V[\x90P`\0a$\x04a'\xF9\x89`\x80\x01Q\x8A`\xE0\x01Qa%\x05\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[`\xE0\x8A\x01Q`\xC0\x8B\x01Q` \x8C\x01Qa(\x11\x91a%\x05V[a(\x1B\x91\x90a9GV[a(%\x91\x90a8\x90V[` \x8A\x01Q\x90a%\x05V[`\0\x80a(c\x83`\x80\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a(M\x90a9+V[a(W\x91\x90a9GV[a\x01\0\x85\x01Q\x90a%\x05V[\x90P`\0a(\x8C\x84a\x01 \x01Qa\x17\"\x86a\x01@\x01Q\x87`@\x01Qa%\x05\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a(\xADa(\xA6\x86`\0\x01Q\x84a$\x90\x91\x90a8\x90V[\x84\x90a%\x05V[\x90P`\0a(\xD0\x86` \x01Qg\x1B\xC1mgN\xC8\0\0a%\x05\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa\x0B%\x82\x82a$\xE1V[`\0\x80`\0\x80\x85\x80` \x01\x90Q\x81\x01\x90a(\xF6\x91\x90a9\xB0V[\x93PP\x92P\x92Pa\x0B%\x83\x83\x87\x84a\x1B\xF3V[`\0\x81`\0\x03a)\"WPg\r\xE0\xB6\xB3\xA7d\0\0\x91\x90PV[gV\x98\xEE\xF0fp\0\0\x82\x12a)9WP`\0\x91\x90PV[a)JgV\x98\xEE\xF0fp\0\0a9+V[\x82\x13a)_WPg\x1B\xC1mgN\xC8\0\0\x91\x90PV[`\0a)j\x83a0+V[\x90P`\0a)\xA3g\r\xE0\xB6\xB3\xA7d\0\0a)\x8C\x84g\x1B\xC1mgN\xC8\0\0a\x1F\0V[a)\x9E\x90g\r\xE0\xB6\xB3\xA7d\0\0a9GV[a0bV[\x90P`\0\x80\x82a*\x04\x81a)\xF1\x81a)\xDF\x81a)\xC7\x81g\x02_\x0F\xE1\x05\xA3\x14\0a.;V[a)\xDA\x90g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19a9GV[a.;V[a)\xDA\x90g\x14\xA8EL\x19\xE1\xAC\0a9GV[a)\xDA\x90g\x0F\xC1\x0E\x01W\x82w\xFF\x19a9GV[a*\x16\x90g\x03\xDE\xBD\x08;\x8C|\0a9GV[\x91P\x83\x90Pa*~\x81a*l\x81a*Z\x81a*H\x81a*5\x81\x8Ba.;V[a)\xDA\x90g\x02\x95\xD4\0\xEA2W\xFF\x19a9GV[a)\xDA\x90g\x01W\xD8\xB2\xEC\xC7\x08\0a9GV[a)\xDA\x90g\x051\n\xA7\xD5!0\0a9GV[a)\xDA\x90g\r\xE0\xCC=\x15a\0\0a9GV[\x91P\x81g\x11\x90\0\xAB\x10\x0F\xFC\0a*\x94\x87\x88a.;V[a*\xA0\x90`\0\x19a8\xB7V[a*\xAA\x91\x90a8\x90V[a*\xB4\x91\x90a9GV[\x92PP`\0a*\xC2\x83a\x1F\xB2V[\x90P`\0a*\xD0\x85\x83a.;V[\x90P`\0\x88\x12a*\xE0W\x80a\x06\x15V[a\x06\x15\x81g\x1B\xC1mgN\xC8\0\0a8\x90V[`\xB5\x81`\x01`\x88\x1B\x81\x10a+\x0BW`@\x91\x90\x91\x1B\x90`\x80\x1C[i\x01\0\0\0\0\0\0\0\0\0\x81\x10a+'W` \x91\x90\x91\x1B\x90`@\x1C[e\x01\0\0\0\0\0\x81\x10a+?W`\x10\x91\x90\x91\x1B\x90` \x1C[c\x01\0\0\0\x81\x10a+UW`\x08\x91\x90\x91\x1B\x90`\x10\x1C[b\x01\0\0\x01\x02`\x12\x1C\x80\x82\x04\x01`\x01\x90\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x90\x81\x90\x04\x81\x11\x90\x03\x90V[`\0a\x04\xF4g\r\xE0\xB6\xB3\xA7d\0\0\x83a+\xAE\x86a.PV[a+\xB8\x91\x90a8\xB7V[a#d\x91\x90a8\xFDV[`\0\x80\x82\x12\x80a+\xD9WPg\x1B\xC1mgN\xC8\0\0\x82\x13[\x15a+\xF7W`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a,\x18W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81g\x1B\xC1mgN\xC8\0\0\x03a,@W`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x15a,KW\x91\x90PV[`\0g\r\xE0\xB6\xB3\xA7d\0\0\x83\x12a,sWa,n\x83g\x1B\xC1mgN\xC8\0\0a8\x90V[a,uV[\x82[\x90P`\0a,\x8B\x82g\x1B\xC1mgN\xC8\0\0a0bV[\x90P\x80`\0\x03a,\xAEW`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a,\xB9\x82a.PV[\x90P`\0c;\x9A\xCA\0a,\xE4a,\xDFa,\xD9g\x1B\xC1mgN\xC8\0\0a9+V[\x85a.;V[a*\xF2V[a,\xEE\x91\x90a8\xB7V[\x90P`\0\x80a-\x05\x83g\x03\xC1f\\z\xAB \0a.;V[a-\x17\x90g \x05\xFEO&\x8E\xA0\0a9GV[\x90P`\0a-B\x84a-0\x86f\x9F2u$b\xA0\0a.;V[a)\xDA\x90g\r\xC5R\x7Fd, \0a9GV[a-T\x90g\r\xE0\xB6\xB3\xA7d\0\0a9GV[\x90Pa-xg\t\xD0(\xCCo _\xFF\x19\x85a-n\x85\x85a0bV[a)\xDA\x91\x90a8\x90V[\x92PPP`\0[`\x02\x81\x10\x15a.\x13W`\0\x86a-\x94\x84a)\tV[a-\x9E\x91\x90a8\x90V[\x90P`\0a-\xAC\x84\x85a.;V[a-\xB5\x90a9+V[\x90P`\0a-\xC2\x82a\x1F\xB2V[\x90P`\0a-\xD0\x86\x85a.;V[a-\xE2g\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x84a.;V[a-\xEC\x91\x90a8\x90V[\x90Pa-\xF8\x84\x82a0bV[a.\x02\x90\x87a9GV[\x95P\x84`\x01\x01\x94PPPPPa-\x7FV[g\r\xE0\xB6\xB3\xA7d\0\0\x88\x12a.0Wa.+\x82a9+V[a\x06\x15V[P\x96\x95PPPPPPV[`\0a\x04\xF4\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a0sV[`\0\x80\x82\x13a.\x8DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x08#V[`\0``a.\x9A\x84a0\x92V[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[`\0`\x01`\xFF\x1B\x82\x03a0QW`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x12\x15a\x1B\xC1WP\x19`\x01\x01\x90V[`\0a\x04\xF4\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x05\x85\x14\x17\x16a0\x8BW`\0\x80\xFD[\x05\x92\x91PPV[`\0\x80\x82\x11a0\xCFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x08#V[P`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x82\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`\0\x80`@\x83\x85\x03\x12\x15a1\xA0Wa1\xA0a1:V[PP\x805\x92` \x90\x91\x015\x91PV[`\0[\x83\x81\x10\x15a1\xCAW\x81\x81\x01Q\x83\x82\x01R` \x01a1\xB2V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra1\xEB\x81` \x86\x01` \x86\x01a1\xAFV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x04\xF4` \x83\x01\x84a1\xD3V[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a2+Wa2+a1:V[PP\x825\x94` \x84\x015\x94P`@\x84\x015\x93``\x015\x92P\x90PV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FABI decoding: struct data too sh`D\x82\x01Rb\x1B\xDC\x9D`\xEA\x1B`d\x82\x01R`\x84\x81\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a2\xD1Wa2\xD1a2\x98V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a3\0Wa3\0a2\x98V[`@R\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a3\x1DW`\0\x80\xFD[PV[`\0\x80`\0\x83\x85\x03`\xE0\x81\x12\x15a39Wa39a1:V[\x845\x93P` \x85\x015\x92P`\xA0`?\x19\x82\x01\x12\x15a3YWa3Ya2GV[Pa3ba2\xAEV[`@\x85\x015\x81R``\x85\x015` \x82\x01R`\x80\x85\x015`@\x82\x01R`\xA0\x85\x015``\x82\x01R`\xC0\x85\x015a3\x95\x81a3\x08V[`\x80\x82\x01R\x92\x95\x91\x94P\x91\x92P\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a3\xBEWa3\xBEa1:V[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[\x80\x15\x15\x81\x14a3\x1DW`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15a3\xFBWa3\xFBa1:V[\x835\x92P` \x84\x015a4\r\x81a3\xD5V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[\x84\x15\x15\x81R\x83` \x82\x01R\x82`@\x82\x01R`\x80``\x82\x01R`\0a\x0B%`\x80\x83\x01\x84a1\xD3V[`\0` \x82\x84\x03\x12\x15a4ZWa4Za1:V[P5\x91\x90PV[\x81Q\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x80\x83\x01Q\x90\x82\x01R``\x80\x83\x01Q\x90\x82\x01R`\x80\x80\x83\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x90\x82\x01R`\xA0\x81\x01a\x04\xF7V[`\0` \x82\x84\x03\x12\x15a4\xB4Wa4\xB4a1:V[\x815a\x061\x81a3\x08V[\x82\x81R`@` \x82\x01R`\0a\x06.`@\x83\x01\x84a1\xD3V[`\0` \x82\x84\x03\x12\x15a4\xEDWa4\xEDa1:V[PQ\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x04\xF7Wa\x04\xF7a4\xF4V[\x81\x81\x03\x81\x81\x11\x15a\x04\xF7Wa\x04\xF7a4\xF4V[`\x01\x80`\xA0\x1B\x03\x84\x16\x81R\x82` \x82\x01R```@\x82\x01R`\0a\x14[``\x83\x01\x84a1\xD3V[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a5sWa5sa1:V[\x86Qa5~\x81a3\xD5V[` \x88\x01Q`@\x89\x01Q``\x8A\x01Q`\x80\x8B\x01Q`\xA0\x90\x9B\x01Q\x93\x9C\x92\x9BP\x90\x99\x90\x98P\x96P\x90\x94P\x92PPPV[`\0` \x80\x83\x85\x03\x12\x15a5\xC3Wa5\xC3a1:V[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a6&W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x84\x90R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a6\x8EW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x84\x90R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x81\xFD[\x81Q\x81\x81\x11\x15a6\xA0Wa6\xA0a2\x98V[a6\xB2`\x1F\x82\x01`\x1F\x19\x16\x85\x01a2\xD7V[\x91P\x80\x82R\x86\x84\x82\x85\x01\x01\x11\x15a7\x18W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x85\x90R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R`\x84\x81\xFD[a7'\x81\x85\x84\x01\x86\x86\x01a1\xAFV[P\x95\x94PPPPPV[`\0`\xA0\x82\x84\x03\x12\x15a7FWa7Fa2GV[a7Na2\xAEV[\x90P\x81Q\x81R` \x82\x01Q` \x82\x01R`@\x82\x01Q`@\x82\x01R``\x82\x01Q``\x82\x01R`\x80\x82\x01Qa7\x80\x81a3\x08V[`\x80\x82\x01R\x92\x91PPV[`\0`\xA0\x82\x84\x03\x12\x15a7\xA0Wa7\xA0a1:V[a\x04\xF4\x83\x83a71V[`\0` \x82\x84\x03\x12\x15a7\xBFWa7\xBFa1:V[\x81Qa\x061\x81a3\x08V[`\0\x80`\0``\x84\x86\x03\x12\x15a7\xE2Wa7\xE2a1:V[\x83Q\x92P` \x84\x01Q\x91P`@\x84\x01Q\x90P\x92P\x92P\x92V[`\x06\x81\x10a8\x19WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x90RV[``\x81\x01a8+\x82\x86a7\xFBV[` \x82\x01\x93\x90\x93R`@\x01R\x91\x90PV[\x84\x81R` \x81\x01\x84\x90R`@\x81\x01\x83\x90Ra\x01\0\x81\x01a\x14[``\x83\x01\x84\x80Q\x82R` \x80\x82\x01Q\x90\x83\x01R`@\x80\x82\x01Q\x90\x83\x01R``\x80\x82\x01Q\x90\x83\x01R`\x80\x90\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x91\x01RV[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a8\xB0Wa8\xB0a4\xF4V[P\x92\x91PPV[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15a8\xD3Wa8\xD3a4\xF4V[\x81\x81\x05\x83\x14\x82\x15\x17a\x04\xF7Wa\x04\xF7a4\xF4V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a9\x0CWa9\x0Ca8\xE7V[`\x01`\xFF\x1B\x82\x14`\0\x19\x84\x14\x16\x15a9&Wa9&a4\xF4V[P\x05\x90V[`\0`\x01`\xFF\x1B\x82\x01a9@Wa9@a4\xF4V[P`\0\x03\x90V[\x80\x82\x01\x82\x81\x12`\0\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a9gWa9ga4\xF4V[PP\x92\x91PPV[`@\x81\x01a9}\x82\x85a7\xFBV[\x82` \x83\x01R\x93\x92PPPV[`@\x81\x01a9\x98\x82\x85a7\xFBV[`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16` \x91\x90\x91\x01R\x91\x90PV[`\0\x80`\0\x80a\x01\0\x85\x87\x03\x12\x15a9\xCAWa9\xCAa1:V[\x84Q\x93P` \x85\x01Q\x92P`@\x85\x01Q\x91Pa9\xE9\x86``\x87\x01a71V[\x90P\x92\x95\x91\x94P\x92PV[`\0\x82a:\x03Wa:\x03a8\xE7V[P\x04\x90V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x04\xF7Wa\x04\xF7a4\xF4V\xFETarget contract does not contain\xA2dipfsX\"\x12 }\xB4\xEB\xE4\xBF\x1BO9\xF7\xD7\x84\x7FW\xB5s\x8C\xB8Q\x19\x99\xA8\x13no\x8B\xB0{\xEACt\x1E_dsolcC\0\x08\x16\x003";
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
