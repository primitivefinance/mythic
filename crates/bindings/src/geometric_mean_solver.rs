pub use geometric_mean_solver::*;
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
pub mod geometric_mean_solver {
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
                    ::std::borrow::ToOwned::to_owned("checkSwapConstant"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("checkSwapConstant"),
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
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
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
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct GeometricMeanParams",
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
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct GeometricMeanParams",
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
                                    name: ::std::borrow::ToOwned::to_owned("data"),
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
                    ::std::borrow::ToOwned::to_owned("prepareWeightXUpdate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "prepareWeightXUpdate",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("targetWeightX"),
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
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static GEOMETRICMEANSOLVER_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15b\0\0^W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`@Qb\0)\xE98\x03\x80b\0)\xE9\x839\x81\x01`@\x81\x90Rb\0\0\x81\x91b\0\0\xA7V[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90Ub\0\x01$V[`\0` \x82\x84\x03\x12\x15b\0\x01\x05W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x01\x1DW`\0\x80\xFD[\x93\x92PPPV[a(\xB5\x80b\0\x014`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x01\x84W`\x005`\xE0\x1C\x80c\x81\xB5\xFA\xC2\x11a\x01\x05W\x80c\xCE\x15;\xF4\x11a\0\xC9W\x80c\xCE\x15;\xF4\x14a\x03vW\x80c\xDE\xF1_\x92\x14a\x03\x89W\x80c\xEC)\xD8\xE6\x14a\x03\x9CW\x80c\xEE>\x8C\xFB\x14a\x03\xAFW\x80c\xF2\xDEz{\x14a\x03\xC2W\x80c\xF3\r7\xF2\x14a\x03\xD5Wa\x01\x84V[\x80c\x81\xB5\xFA\xC2\x14a\x02\xF2W\x80c\x90.\xCA\xA2\x14a\x03\x12W\x80c\xA8\xC6.v\x14a\x03%W\x80c\xB0\x9D\x04\xE5\x14a\x03PW\x80c\xCB\x1FU2\x14a\x03cWa\x01\x84V[\x80c;M\x100\x11a\x01LW\x80c;M\x100\x14a\x02xW\x80cO\xD6|X\x14a\x02\x8BW\x80cZ\x93\xB8\xCE\x14a\x02\x9EW\x80cb7V\x9F\x14a\x02\xB1W\x80c\x7F\x17@\x9C\x14a\x02\xDFWa\x01\x84V[\x80c\x0FAf\xB8\x14a\x01\xE9W\x80c%\th\xD9\x14a\x02\x0FW\x80c0m\xB4k\x14a\x02/W\x80c3\"f\xF3\x14a\x02BW\x80c9(\xFF\x97\x14a\x02UW[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x01\xFCa\x01\xF76`\x04a\x1F\x80V[a\x03\xE8V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x02\"a\x02\x1D6`\x04a \xAFV[a\x04$V[`@Qa\x02\x06\x91\x90a!$V[a\x01\xFCa\x02=6`\x04a!7V[a\x049V[a\x01\xFCa\x02P6`\x04a!7V[a\x04iV[a\x02ha\x02c6`\x04a!wV[a\x04\x99V[`@Qa\x02\x06\x94\x93\x92\x91\x90a!\xB2V[a\x01\xFCa\x02\x866`\x04a!\xD9V[a\x08\xAFV[a\x01\xFCa\x02\x996`\x04a!7V[a\x08\xE2V[a\x01\xFCa\x02\xAC6`\x04a!7V[a\t\x12V[a\x02\xC4a\x02\xBF6`\x04a \xAFV[a\t/V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x02\x06V[a\x02\xC4a\x02\xED6`\x04a \xAFV[a\txV[a\x03\x05a\x03\x006`\x04a!\xD9V[a\t\xC0V[`@Qa\x02\x06\x91\x90a\"!V[a\x01\xFCa\x03 6`\x04a!7V[a\n\xC5V[`\0Ta\x038\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\x06V[a\x02\"a\x03^6`\x04a!\xD9V[a\n\xF5V[a\x02\"a\x03q6`\x04a\"DV[a\x0B\0V[a\x02\xC4a\x03\x846`\x04a!\xD9V[a\x0B\x0BV[a\x02\"a\x03\x976`\x04a#,V[a\x0C\x9BV[a\x01\xFCa\x03\xAA6`\x04a!7V[a\x0C\xA8V[a\x02\xC4a\x03\xBD6`\x04a \xAFV[a\x0C\xB8V[a\x01\xFCa\x03\xD06`\x04a!7V[a\x0C\xDEV[a\x02\xC4a\x03\xE36`\x04a \xAFV[a\x0C\xF3V[`\0\x80\x80\x80a\x03\xF9\x85\x87\x01\x87a!7V[\x92P\x92P\x92P`\0a\x04\n\x88a\t\xC0V[\x90Pa\x04\x18\x84\x84\x84\x84a\r\x19V[\x98\x97PPPPPPPPV[``a\x040\x83\x83a\rwV[\x90P[\x92\x91PPV[`\0\x80a\x04E\x85a\t\xC0V[\x90P`\0\x80`\0a\x04U\x88a\x0B\x0BV[\x92P\x92P\x92Pa\x04\x18\x87\x84\x84\x84\x8A\x89a\r\xA6V[`\0\x80a\x04u\x85a\t\xC0V[\x90P`\0\x80`\0a\x04\x85\x88a\x0B\x0BV[\x92P\x92P\x92Pa\x04\x18\x87\x84\x84\x84\x8A\x89a\x0E\x1CV[`\0\x80`\0``a\x04\xC4`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[a\x04\xE8`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[a\x04\xF1\x89a\x0B\x0BV[`@\x85\x01R` \x84\x01R\x82R`\0a\x05\x08\x8Aa\t\xC0V[\x90P`\0\x80a\x05(\x85`\0\x01Q\x86` \x01Qa\x05#\x8Fa\t\xC0V[a\x0EQV[\x90P\x8A\x15a\x06fW`\0a\x05I\x84`@\x01Q\x8Ca\x0E|\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x05v\x85` \x01Qa\x05p\x89`\0\x01Q\x8A` \x01Qa\x0E\x91\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90a\x0E\xA6V[\x90P`\0a\x05\x84\x83\x83a\x0E|V[\x90Pa\x05\x91`\x01\x82a#\xBEV[\x88Q\x90\x91Pa\x05\xA1\x90\x8E\x90a#\xBEV[\x87Ra\x05\xAD\x81\x85a#\xBEV[\x87`@\x01\x81\x81RPPa\x05\xC9\x8F\x88`\0\x01Q\x89`@\x01Qa\x0C\xDEV[` \x88\x01\x81\x81R`\x01\x91a\x05\xDE\x90\x83\x90a#\xBEV[\x90RP` \x80\x89\x01Q\x90\x88\x01Q\x10a\x06HW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: y reserve increase`D\x82\x01Rad!`\xF0\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[\x86` \x01Q\x88` \x01Qa\x06\\\x91\x90a#\xD1V[\x94PPPPa\x07{V[`\0a\x06\x7F\x84`@\x01Q\x8Ca\x0E|\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x84Q` \x88\x01Q\x88Q\x92\x93P`\0\x92a\x06\x9D\x92\x91a\x05p\x91\x90a\x0E\x91V[\x90P`\0a\x06\xAB\x83\x83a\x0E|V[\x90Pa\x06\xB8`\x01\x82a#\xBEV[\x90P\x8C\x88` \x01Qa\x06\xCA\x91\x90a#\xBEV[` \x88\x01Ra\x06\xD9\x81\x85a#\xBEV[\x87`@\x01\x81\x81RPPa\x06\xF5\x8F\x88` \x01Q\x89`@\x01Qa\t\x12V[\x80\x88R`\x01\x90\x88\x90a\x07\x08\x90\x83\x90a#\xBEV[\x90RP\x87Q\x87Q\x10a\x07gW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: x reserve increase`D\x82\x01Rad!`\xF0\x1B`d\x82\x01R`\x84\x01a\x06?V[\x86Q\x88Qa\x07u\x91\x90a#\xD1V[\x94PPPP[\x83Q` \x80\x86\x01Q`@\x80\x88\x01Q\x81Q\x93\x84\x01\x94\x90\x94R\x82\x01R``\x81\x01\x91\x90\x91R`\0\x90`\x80\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R`\0\x80Tc\r\x17\xA7\xC7`\xE3\x1B\x84R\x91\x93P\x8F\x92\x90\x91`\x01`\x01`\xA0\x1B\x03\x16\x90ch\xBD>8\x90a\x07\xEB\x900\x90\x86\x90\x88\x90`\x04\x01a#\xE4V[`\xC0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x08AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a(`\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x08UW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08y\x91\x90a$\x0BV[PPPPP\x90P\x80\x85a\x08\x95\x89`\0\x01Q\x8A` \x01Q\x8Aa\x0E\xD7V[\x85\x9BP\x9BP\x9BP\x9BPPPPPPPPP\x93P\x93P\x93P\x93V[`\0\x80a\x08\xBB\x83a\t\xC0V[\x90P`\0\x80a\x08\xC9\x85a\x0B\x0BV[P\x91P\x91Pa\x08\xD9\x82\x82\x85a\x0E\xD7V[\x95\x94PPPPPV[`\0\x80a\x08\xEE\x85a\t\xC0V[\x90P`\0\x80`\0a\x08\xFE\x88a\x0B\x0BV[\x92P\x92P\x92Pa\x04\x18\x87\x84\x84\x84\x8A\x89a\x0F\x10V[`\0a\t'\x83\x83a\t\"\x87a\t\xC0V[a\x0FvV[\x94\x93PPPPV[`\0\x80`\0\x80`\0a\t@\x87a\x0B\x0BV[\x92PP\x91P`\0\x80a\tU`\0\x89\x86\x86a\x0F\xB3V[\x91P\x91P`\0a\tf\x8A\x84\x84a\x0C\xDEV[\x92\x97P\x91\x95P\x93PPPP\x92P\x92P\x92V[`\0\x80`\0\x80`\0a\t\x89\x87a\x0B\x0BV[\x92P\x92PP`\0\x80a\t\x9E`\x01\x89\x86\x86a\x0F\xB3V[\x91P\x91P`\0a\t\xAF\x8A\x84\x84a\t\x12V[\x97P\x91\x95P\x93PPPP\x92P\x92P\x92V[a\t\xF4`@Q\x80`\x80\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81RP\x90V[`\0T`@Qc\xDC\x17\x83U`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xDC\x17\x83U\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\nvW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a(`\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\n\x8AW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\n\xB2\x91\x90\x81\x01\x90a$aV[\x80` \x01\x90Q\x81\x01\x90a\x043\x91\x90a%\x9CV[`\0\x80a\n\xD1\x85a\t\xC0V[\x90P`\0\x80`\0a\n\xE1\x88a\x0B\x0BV[\x92P\x92P\x92Pa\x04\x18\x87\x84\x84\x84\x8A\x89a\x10\x1CV[``a\x043\x82a\x10\xC0V[``a\x043\x82a\x10\xECV[`\0\x80`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xAF\xBA\x13\xC4`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0B\x9BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a(`\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0B\xAFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\xD3\x91\x90a%\xBBV[`\x01`\x01`\xA0\x1B\x03\x16c\xCE\x15;\xF4\x85`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0C\0\x91\x81R` \x01\x90V[```@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0CVW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a(`\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0CjW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\x8E\x91\x90a%\xDBV[\x92P\x92P\x92P\x91\x93\x90\x92PV[``a\t'\x84\x84\x84a\x11\x02V[`\0a\t'\x83\x83a\x05#\x87a\t\xC0V[`\0\x80`\0\x80`\0a\x0C\xC9\x87a\x0B\x0BV[\x92PP\x91P`\0\x80a\tU`\x01\x89\x86\x86a\x0F\xB3V[`\0a\t'\x83\x83a\x0C\xEE\x87a\t\xC0V[a\x11\xA0V[`\0\x80`\0\x80`\0a\r\x04\x87a\x0B\x0BV[\x92P\x92PP`\0\x80a\t\x9E`\0\x89\x86\x86a\x0F\xB3V[\x80Q`\0\x90\x81\x90a\r.\x90a\x05p\x88\x87a\x11\xD5V[\x90P`\0a\rM\x84` \x01Qa\x05p\x87\x89a\x11\xD5\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0a\rb\x83\x83a\x0E|V[a\rl\x91\x90a&\x0CV[\x97\x96PPPPPPPV[```\x02\x83\x83`@Q` \x01a\r\x8F\x93\x92\x91\x90a&UV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x92\x91PPV[`\0\x82a\x03\xE8\x82a\r\xBB\x8A\x8A\x8A\x8A\x86\x8Aa\x0E\x1CV[\x90P`\0\x81\x12\x15a\r\xD2W`\0\x93PPPPa\x0E\x12V[a\x0E\x0C\x8A\x8A\x8A\x8A\x89`@Q` \x01a\r\xEE\x95\x94\x93\x92\x91\x90a&tV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83\x85`\x01a\x01\0a\x11\xEAa\x12\x1FV[\x93PPPP[\x96\x95PPPPPPV[`\0\x80\x82`@\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x0E7\x91\x90a#\xD1V[\x90Pa\x04\x18\x83`\0\x01Q\x84` \x01Q\x89\x89\x8C\x8A\x8A\x88a\x130V[`\0a\t'a\x0Em\x83` \x01Q\x85a\x0E\xA6\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x83Qa\x0Ez\x90\x87\x90a\x0E\xA6V[\x90[`\0a\x040\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x13\xDFV[`\0a\x040\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x13\xDFV[`\0a\x040g\r\xE0\xB6\xB3\xA7d\0\0\x83a\x0E\xBE\x86a\x14\rV[a\x0E\xC8\x91\x90a&\x9FV[a\x0E\xD2\x91\x90a&\xE5V[a\x15\xE8V[`\0\x80a\x0E\xF1\x83` \x01Q\x85a\x11\xD5\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x83Q\x90\x91P`\0\x90a\x0F\x04\x90\x87\x90a\x11\xD5V[\x90Pa\x0E\x12\x82\x82a\x11\xD5V[`\0\x82a\x03\xE8\x82a\x0F%\x8A\x8A\x8A\x8A\x86\x8Aa\x10\x1CV[\x90P`\0\x81\x12\x15a\x0F<W`\0\x93PPPPa\x0E\x12V[a\x0E\x0C\x8A\x8A\x8A\x8A\x89`@Q` \x01a\x0FX\x95\x94\x93\x92\x91\x90a&tV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83\x85`\x01a\x01\0a\x17\x91a\x12\x1FV[\x80Q`\0\x90a\t'\x90a\x0F\x92\x90g\r\xE0\xB6\xB3\xA7d\0\0\x90a\x0E\x91V[a\x05pa\x0F\xAC\x85` \x01Q\x88a\x0E\xA6\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x86\x90a\x0E\x91V[`\0\x80\x80a\x0F\xC1\x84\x86a\x0E\x91V[\x90P`\0a\x0F\xCF\x87\x83a\x0E|V[\x90P\x87a\x0F\xE5Wa\x0F\xE0\x87\x87a#\xD1V[a\x0F\xEFV[a\x0F\xEF\x87\x87a#\xBEV[\x93P\x87a\x10\x05Wa\x10\0\x81\x86a#\xD1V[a\x10\x0FV[a\x10\x0F\x81\x86a#\xBEV[\x92PPP\x94P\x94\x92PPPV[`\0\x80a\x106\x83`\0\x01Q\x88\x86\x89\x89\x8D\x89`@\x01Qa\x17\xC6V[\x90P`\0a\x10C\x82a\x19\x0BV[\x90P`\0\x80a\x10]a\x10U\x8A\x89a'\x13V[\x87Q\x90a\x1A0V[\x90P`\0a\x10\x91\x85`\xE0\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x10{\x90a';V[a\x10\x85\x91\x90a'\x13V[a\x01 \x87\x01Q\x90a\x1A0V[a\x10\x9A\x8Aa';V[a\x10\xA4\x91\x90a'\x13V[\x90Pa\x10\xB0\x82\x82a\x1A0V[\x92Pa\x0E\x0C\x91P\x83\x90P\x82a\x1AcV[```\x01\x82`@Q` \x01a\x10\xD6\x92\x91\x90a'WV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x91\x90PV[```\x03\x82`@Q` \x01a\x10\xD6\x92\x91\x90a'rV[```\0a\x11\x11\x85\x85\x85a\x1A\x87V[\x90P`\0a\x11 \x86\x83\x86a\x1A\xB2V[\x90P`\0a\x110\x87\x84\x84\x88a\r\x19V[\x90Pa\x11?\x87\x84\x83\x85\x89a\x1A\xEBV[\x85Q`@\x80\x88\x01Q``\x80\x8A\x01Q\x83Q` \x81\x01\x8E\x90R\x93\x84\x01\x89\x90R\x90\x83\x01\x85\x90R`\x80\x83\x01\x93\x90\x93R`\xA0\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\xC0\x82\x01R\x90\x92P`\xE0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x93PPPP\x93\x92PPPV[`\0a\t'a\x11\xC4\x83` \x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x0E\x91\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x83Qa\x05p\x90a\x0F\xAC\x90\x88\x90a\x0E\xA6V[`\0a\x040\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x1B\x9DV[`\0\x80`\0\x80`\0\x80\x87\x80` \x01\x90Q\x81\x01\x90a\x12\x07\x91\x90a'\x98V[\x94P\x94P\x94P\x94P\x94Pa\x04\x18\x85\x85\x85\x85\x8B\x86a\x0E\x1CV[`\0\x84\x86\x11\x15a\x12LW`@Qc0\x82\xDF\xDB`\xE1\x1B\x81R`\x04\x81\x01\x87\x90R`$\x81\x01\x86\x90R`D\x01a\x06?V[`\0a\x12\\\x88\x88\x85c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x12n\x89\x88\x86c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x12|\x82\x84a&\x9FV[\x13\x15a\x12\xA5W`@Qc\x06\xF1\xBE]`\xE2\x1B\x81R`\x04\x81\x01\x83\x90R`$\x81\x01\x82\x90R`D\x01a\x06?V[`\0a\x12\xB1\x89\x89a#\xD1V[\x90P`\0[`\x02a\x12\xC2\x8A\x8Ca#\xBEV[a\x12\xCC\x91\x90a'\xE6V[\x94P`\0a\x12\xDE\x8C\x87\x89c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x12\xEC\x86\x83a&\x9FV[\x13a\x12\xF9W\x85\x99Pa\x13\0V[\x85\x9AP\x80\x94P[a\x13\n\x8B\x8Ba#\xD1V[\x92PP`\x01\x01\x87\x82\x11\x80\x15a\x13\x1EWP\x86\x81\x10[a\x12\xB6WPPPP\x96\x95PPPPPPV[`\0\x80a\x13=\x87\x89a\x11\xD5V[\x90P`\0`@Q\x80a\x01\0\x01`@R\x80\x8C\x81R` \x01\x8A\x81R` \x01\x89\x81R` \x01\x87\x81R` \x01\x86\x81R` \x01a\x13~\x8D\x85a\x0E\xA6\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x81R` \x01a\x13\x8D\x84\x8Da\x0E\xA6V[\x81R` \x01\x85\x90R\x90P`\0a\x13\xA2\x82a\x1B\xBCV[\x90P`\0a\x13\xAF\x83a\x1C\xFAV[\x90Pa\x13\xBB\x82\x82a\x11\xD5V[a\x13\xC4\x8Aa';V[a\x13\xCE\x91\x90a'\x13V[\x9D\x9CPPPPPPPPPPPPPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x13\xF7W`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[`\0\x80\x82\x13a\x14JW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x06?V[`\0``a\x14W\x84a\x1D\x9DV[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[`\0h\x02H\xCE6\xA7\x0C\xB2k>\x19\x82\x13a\x16\x03WP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a\x16JW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x01a\x06?V[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05`\x01`_\x1B\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1Dm\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1Dn\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[`\0\x80`\0\x80`\0\x80\x87\x80` \x01\x90Q\x81\x01\x90a\x17\xAE\x91\x90a'\x98V[\x94P\x94P\x94P\x94P\x94Pa\x04\x18\x85\x85\x85\x85\x8B\x86a\x10\x1CV[a\x18\x1C`@Q\x80a\x01@\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`\0a\x18G\x89a\x183g\r\xE0\xB6\xB3\xA7d\0\0a';V[a\x18=\x91\x90a'\x13V[a\x05p\x88\x8Aa'\x13V[\x90P`\0a\x18Y\x8Aa\x05p\x8B\x8Aa\x1AcV[\x90P`\0a\x18g\x89\x83a\x1A0V[\x90P`\0a\x18}\x86g\r\xE0\xB6\xB3\xA7d\0\0a&\x0CV[\x90P`\0a\x18\xA7\x82a\x18\x96g\r\xE0\xB6\xB3\xA7d\0\0a';V[a\x18\xA0\x91\x90a'\x13V[\x84\x90a\x1A0V[a\x18\xB1\x90\x8Aa&\x0CV[\x90P`@Q\x80a\x01@\x01`@R\x80\x8E\x81R` \x01\x8C\x81R` \x01\x8B\x81R` \x01\x82\x81R` \x01\x86\x81R` \x01\x8A\x81R` \x01\x85\x81R` \x01\x83\x81R` \x01\x89\x81R` \x01\x84\x81RP\x95PPPPPP\x97\x96PPPPPPPV[`\0\x80a\x19-\x83`@\x01Q\x84` \x01Qa\x19%\x91\x90a'\x13V[\x84Q\x90a\x1A0V[\x90P`\0a\x19fa\x19S\x85`\0\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x1Ac\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[``\x86\x01Q`\x80\x87\x01Qa\x05p\x91a\x1A0V[\x90P`\0a\x19\x99\x85`\0\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x19\x84\x90a';V[a\x19\x8E\x91\x90a'\x13V[`\xA0\x87\x01Q\x90a\x1A0V[\x90P`\0a\x19\xF7\x86`\xE0\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x19\xB7\x90a';V[a\x19\xC1\x91\x90a'\x13V[`@\x88\x01Q\x88Q` \x8A\x01Qa\x19\xF1\x92\x91a\x19\xDC\x91\x90a\x1A0V[a\x19\xE6\x91\x90a'\x13V[`\xC0\x8A\x01Q\x90a\x1A0V[\x90a\x1A0V[\x90Pa\x1A\x16a\x1A\x06\x82\x84a&\x0CV[a\x01\0\x88\x01Qa\x19\xF1\x90\x86a\x1A0V[``\x87\x01Qa\x1A&\x90\x86\x90a\x1A0V[a\x0E\x12\x91\x90a'\x13V[\x81\x81\x02\x82\x15\x83\x82\x05\x83\x14\x17`\0\x19\x84\x10`\x01`\xFF\x1B\x84\x13\x17\x16a\x1ARW`\0\x80\xFD[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x92\x91PPV[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x81\x02\x90\x81\x05\x83\x14\x82\x15\x15\x16a\x1A\x81W`\0\x80\xFD[\x05\x91\x90PV[`\0a\t'\x84a\x1A\xAC\x85a\x1A\xAC\x86`\0\x01Q\x87` \x01Qa\x11\xD5\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90a\x1EEV[\x80Q`\0\x90\x81\x90a\x1A\xC4\x90\x86\x90a\x0E\xA6V[\x90P`\0a\x1A\xDF\x84` \x01Q\x86a\x0E\xA6\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa\x0E\x12\x82\x82a\x0E|V[`\0\x82\x80\x85\x83\x81\x12\x15a\x1B+W[`\0\x81\x12\x15a\x1B&Wa\x1B\x11\x82a\x03\xE7a\x03\xE8a\x1B\x9DV[\x91Pa\x1B\x1F\x89\x89\x84\x88a\r\x19V[\x90Pa\x1A\xF9V[a\x1BXV[`\0\x81\x13\x15a\x1BXWa\x1BC\x83a\x03\xE9a\x03\xE8a\x13\xDFV[\x92Pa\x1BQ\x89\x89\x85\x88a\r\x19V[\x90Pa\x1B+V[a\x1B\x90\x89\x89\x83\x88`@Q` \x01a\x1Br\x94\x93\x92\x91\x90a'\xFAV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83\x85`\x01a\x01\0a\x1EZa\x12\x1FV[\x99\x98PPPPPPPPPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x1B\xB5W`\0\x80\xFD[\x04\x92\x91PPV[`\0\x80a\x1B\xEA\x83`\xA0\x01Qa\x1A\xAC\x85` \x01Qa\x1A\xAC\x87`\0\x01Q\x88``\x01Qa\x1EE\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x1CJ\x84`\xE0\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x1C\t\x91\x90a#\xD1V[a\x1A\xAC\x86`@\x01Q\x87` \x01Qa\x1C1\x89`\0\x01Q\x8A`\x80\x01Qa\x1EE\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x89`\x80\x01Qa\x1C@\x91\x90a#\xD1V[a\x1A\xAC\x91\x90a#\xBEV[\x90P`\0a\x1Cs\x85`\0\x01Qa\x1C_\x90a';V[\x86` \x01Q\x87`\x80\x01Qa\x05p\x91\x90a#\xBEV[\x90P`\0a\x1C\xA5\x86`\xE0\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x1C\x92\x91\x90a#\xD1V[`\xC0\x88\x01Q`\x80\x89\x01Qa\x1A\xAC\x91a\x1EEV[\x86``\x01Qa\x1C\xB4\x91\x90a#\xBEV[\x90Pa\x0E\x12a\x1C\xF0a\x1C\xE6\x88`\0\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x1C\xD7\x91\x90a#\xD1V[g\r\xE0\xB6\xB3\xA7d\0\0\x90a\x11\xD5V[a\x05p\x85\x85a\x1EEV[a\x1A\xAC\x85\x87a#\xD1V[\x80Q`\0\x90\x81\x90a\x1D\x13\x90g\r\xE0\xB6\xB3\xA7d\0\0a#\xD1V[\x90P`\0\x83` \x01Q\x84`\x80\x01Qa\x1D+\x91\x90a#\xBEV[\x90P`\0a\x1DR\x85`\xA0\x01Qa\x1A\xAC\x87` \x01Q\x88``\x01Qa\x1EE\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x1D\x84\x86`\xE0\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x1Dq\x91\x90a#\xD1V[`@\x88\x01Q`\x80\x89\x01Qa\x1A\xAC\x91a\x1EEV[\x90Pa\x0E\x12a\x1D\x93\x82\x84a#\xBEV[a\x1A\xAC\x86\x86a\x1EEV[`\0\x80\x82\x11a\x1D\xDAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x06?V[P`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x82\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[`\0a\x040\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x1B\x9DV[`\0\x80`\0\x80\x85\x80` \x01\x90Q\x81\x01\x90a\x1Et\x91\x90a(\x1CV[\x93PP\x92P\x92Pa\x0E\x12\x83\x83\x87\x84a\r\x19V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x81\xFD[`\0\x80`\0`@\x84\x86\x03\x12\x15a\x1F\x98Wa\x1F\x98a\x1E\x87V[\x835\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x1F\xBAWa\x1F\xBAa\x1E\xD7V[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x1F\xD1Wa\x1F\xD1a\x1F'V[\x815\x81\x81\x11\x15a 4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x0ENL/$\r\x8C\xAD\xCC\xEE\x8D`\xAB\x1B`d\x82\x01R`\x84\x81\xFD[\x87` \x82\x85\x01\x01\x11\x15a \x9CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rjrray stride`\xA8\x1B`d\x82\x01R\x91P`\x84\x82\xFD[` \x83\x01\x94P\x80\x93PPPP\x92P\x92P\x92V[`\0\x80`@\x83\x85\x03\x12\x15a \xC5Wa \xC5a\x1E\x87V[PP\x805\x92` \x90\x91\x015\x91PV[`\0[\x83\x81\x10\x15a \xEFW\x81\x81\x01Q\x83\x82\x01R` \x01a \xD7V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra!\x10\x81` \x86\x01` \x86\x01a \xD4V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x040` \x83\x01\x84a \xF8V[`\0\x80`\0``\x84\x86\x03\x12\x15a!OWa!Oa\x1E\x87V[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[\x80\x15\x15\x81\x14a!tW`\0\x80\xFD[PV[`\0\x80`\0``\x84\x86\x03\x12\x15a!\x8FWa!\x8Fa\x1E\x87V[\x835\x92P` \x84\x015a!\xA1\x81a!fV[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[\x84\x15\x15\x81R\x83` \x82\x01R\x82`@\x82\x01R`\x80``\x82\x01R`\0a\x0E\x12`\x80\x83\x01\x84a \xF8V[`\0` \x82\x84\x03\x12\x15a!\xEEWa!\xEEa\x1E\x87V[P5\x91\x90PV[\x80Q\x82R` \x80\x82\x01Q\x90\x83\x01R`@\x80\x82\x01Q\x90\x83\x01R``\x90\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x91\x01RV[`\x80\x81\x01a\x043\x82\x84a!\xF5V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a!tW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\"YWa\"Ya\x1E\x87V[\x815a\"d\x81a\"/V[\x93\x92PPPV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FABI decoding: struct data too sh`D\x82\x01Rb\x1B\xDC\x9D`\xEA\x1B`d\x82\x01R`\x84\x81\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x80\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\"\xF5Wa\"\xF5a\"\xBCV[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a#$Wa#$a\"\xBCV[`@R\x91\x90PV[`\0\x80`\0\x83\x85\x03`\xC0\x81\x12\x15a#EWa#Ea\x1E\x87V[\x845\x93P` \x85\x015\x92P`\x80`?\x19\x82\x01\x12\x15a#eWa#ea\"kV[Pa#na\"\xD2V[`@\x85\x015\x81R``\x85\x015` \x82\x01R`\x80\x85\x015`@\x82\x01R`\xA0\x85\x015a#\x97\x81a\"/V[``\x82\x01R\x92\x95\x91\x94P\x91\x92P\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x043Wa\x043a#\xA8V[\x81\x81\x03\x81\x81\x11\x15a\x043Wa\x043a#\xA8V[`\x01\x80`\xA0\x1B\x03\x84\x16\x81R\x82` \x82\x01R```@\x82\x01R`\0a\x08\xD9``\x83\x01\x84a \xF8V[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a$'Wa$'a\x1E\x87V[\x86Qa$2\x81a!fV[` \x88\x01Q`@\x89\x01Q``\x8A\x01Q`\x80\x8B\x01Q`\xA0\x90\x9B\x01Q\x93\x9C\x92\x9BP\x90\x99\x90\x98P\x96P\x90\x94P\x92PPPV[`\0` \x80\x83\x85\x03\x12\x15a$wWa$wa\x1E\x87V[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a$\x92Wa$\x92a\x1E\xD7V[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a$\xA9Wa$\xA9a\x1F'V[\x81Q\x81\x81\x11\x15a$\xBBWa$\xBBa\"\xBCV[a$\xCD`\x1F\x82\x01`\x1F\x19\x16\x85\x01a\"\xFBV[\x91P\x80\x82R\x86\x84\x82\x85\x01\x01\x11\x15a%3W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x85\x90R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R`\x84\x81\xFD[a%B\x81\x85\x84\x01\x86\x86\x01a \xD4V[P\x95\x94PPPPPV[`\0`\x80\x82\x84\x03\x12\x15a%aWa%aa\"kV[a%ia\"\xD2V[\x90P\x81Q\x81R` \x82\x01Q` \x82\x01R`@\x82\x01Q`@\x82\x01R``\x82\x01Qa%\x91\x81a\"/V[``\x82\x01R\x92\x91PPV[`\0`\x80\x82\x84\x03\x12\x15a%\xB1Wa%\xB1a\x1E\x87V[a\x040\x83\x83a%LV[`\0` \x82\x84\x03\x12\x15a%\xD0Wa%\xD0a\x1E\x87V[\x81Qa\"d\x81a\"/V[`\0\x80`\0``\x84\x86\x03\x12\x15a%\xF3Wa%\xF3a\x1E\x87V[\x83Q\x92P` \x84\x01Q\x91P`@\x84\x01Q\x90P\x92P\x92P\x92V[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a&,Wa&,a#\xA8V[P\x92\x91PPV[`\x04\x81\x10a&QWcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x90RV[``\x81\x01a&c\x82\x86a&3V[` \x82\x01\x93\x90\x93R`@\x01R\x91\x90PV[`\0a\x01\0\x82\x01\x90P\x86\x82R\x85` \x83\x01R\x84`@\x83\x01R\x83``\x83\x01Ra\x0E\x12`\x80\x83\x01\x84a!\xF5V[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15a&\xBBWa&\xBBa#\xA8V[\x81\x81\x05\x83\x14\x82\x15\x17a\x043Wa\x043a#\xA8V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a&\xF4Wa&\xF4a&\xCFV[`\x01`\xFF\x1B\x82\x14`\0\x19\x84\x14\x16\x15a'\x0EWa'\x0Ea#\xA8V[P\x05\x90V[\x80\x82\x01\x82\x81\x12`\0\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a'3Wa'3a#\xA8V[PP\x92\x91PPV[`\0`\x01`\xFF\x1B\x82\x01a'PWa'Pa#\xA8V[P`\0\x03\x90V[`@\x81\x01a'e\x82\x85a&3V[\x82` \x83\x01R\x93\x92PPPV[`@\x81\x01a'\x80\x82\x85a&3V[`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16` \x91\x90\x91\x01R\x91\x90PV[`\0\x80`\0\x80`\0a\x01\0\x86\x88\x03\x12\x15a'\xB4Wa'\xB4a\x1E\x87V[\x85Q\x94P` \x86\x01Q\x93P`@\x86\x01Q\x92P``\x86\x01Q\x91Pa'\xDA\x87`\x80\x88\x01a%LV[\x90P\x92\x95P\x92\x95\x90\x93PV[`\0\x82a'\xF5Wa'\xF5a&\xCFV[P\x04\x90V[\x84\x81R` \x81\x01\x84\x90R`@\x81\x01\x83\x90R`\xE0\x81\x01a\x08\xD9``\x83\x01\x84a!\xF5V[`\0\x80`\0\x80`\xE0\x85\x87\x03\x12\x15a(5Wa(5a\x1E\x87V[\x84Q\x93P` \x85\x01Q\x92P`@\x85\x01Q\x91Pa(T\x86``\x87\x01a%LV[\x90P\x92\x95\x91\x94P\x92PV\xFETarget contract does not contain\xA2dipfsX\"\x12 \xF3Z|\xDE\xE8@\xE2l!\xC6!j\x88*+\xAA\x86H\xCC\x91\x981\xB7vr\xC5#\xFB\xE3\"\xBD\x0EdsolcC\0\x08\x16\x003";
    /// The bytecode of the contract.
    pub static GEOMETRICMEANSOLVER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x01\x84W`\x005`\xE0\x1C\x80c\x81\xB5\xFA\xC2\x11a\x01\x05W\x80c\xCE\x15;\xF4\x11a\0\xC9W\x80c\xCE\x15;\xF4\x14a\x03vW\x80c\xDE\xF1_\x92\x14a\x03\x89W\x80c\xEC)\xD8\xE6\x14a\x03\x9CW\x80c\xEE>\x8C\xFB\x14a\x03\xAFW\x80c\xF2\xDEz{\x14a\x03\xC2W\x80c\xF3\r7\xF2\x14a\x03\xD5Wa\x01\x84V[\x80c\x81\xB5\xFA\xC2\x14a\x02\xF2W\x80c\x90.\xCA\xA2\x14a\x03\x12W\x80c\xA8\xC6.v\x14a\x03%W\x80c\xB0\x9D\x04\xE5\x14a\x03PW\x80c\xCB\x1FU2\x14a\x03cWa\x01\x84V[\x80c;M\x100\x11a\x01LW\x80c;M\x100\x14a\x02xW\x80cO\xD6|X\x14a\x02\x8BW\x80cZ\x93\xB8\xCE\x14a\x02\x9EW\x80cb7V\x9F\x14a\x02\xB1W\x80c\x7F\x17@\x9C\x14a\x02\xDFWa\x01\x84V[\x80c\x0FAf\xB8\x14a\x01\xE9W\x80c%\th\xD9\x14a\x02\x0FW\x80c0m\xB4k\x14a\x02/W\x80c3\"f\xF3\x14a\x02BW\x80c9(\xFF\x97\x14a\x02UW[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x01\xFCa\x01\xF76`\x04a\x1F\x80V[a\x03\xE8V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x02\"a\x02\x1D6`\x04a \xAFV[a\x04$V[`@Qa\x02\x06\x91\x90a!$V[a\x01\xFCa\x02=6`\x04a!7V[a\x049V[a\x01\xFCa\x02P6`\x04a!7V[a\x04iV[a\x02ha\x02c6`\x04a!wV[a\x04\x99V[`@Qa\x02\x06\x94\x93\x92\x91\x90a!\xB2V[a\x01\xFCa\x02\x866`\x04a!\xD9V[a\x08\xAFV[a\x01\xFCa\x02\x996`\x04a!7V[a\x08\xE2V[a\x01\xFCa\x02\xAC6`\x04a!7V[a\t\x12V[a\x02\xC4a\x02\xBF6`\x04a \xAFV[a\t/V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x02\x06V[a\x02\xC4a\x02\xED6`\x04a \xAFV[a\txV[a\x03\x05a\x03\x006`\x04a!\xD9V[a\t\xC0V[`@Qa\x02\x06\x91\x90a\"!V[a\x01\xFCa\x03 6`\x04a!7V[a\n\xC5V[`\0Ta\x038\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\x06V[a\x02\"a\x03^6`\x04a!\xD9V[a\n\xF5V[a\x02\"a\x03q6`\x04a\"DV[a\x0B\0V[a\x02\xC4a\x03\x846`\x04a!\xD9V[a\x0B\x0BV[a\x02\"a\x03\x976`\x04a#,V[a\x0C\x9BV[a\x01\xFCa\x03\xAA6`\x04a!7V[a\x0C\xA8V[a\x02\xC4a\x03\xBD6`\x04a \xAFV[a\x0C\xB8V[a\x01\xFCa\x03\xD06`\x04a!7V[a\x0C\xDEV[a\x02\xC4a\x03\xE36`\x04a \xAFV[a\x0C\xF3V[`\0\x80\x80\x80a\x03\xF9\x85\x87\x01\x87a!7V[\x92P\x92P\x92P`\0a\x04\n\x88a\t\xC0V[\x90Pa\x04\x18\x84\x84\x84\x84a\r\x19V[\x98\x97PPPPPPPPV[``a\x040\x83\x83a\rwV[\x90P[\x92\x91PPV[`\0\x80a\x04E\x85a\t\xC0V[\x90P`\0\x80`\0a\x04U\x88a\x0B\x0BV[\x92P\x92P\x92Pa\x04\x18\x87\x84\x84\x84\x8A\x89a\r\xA6V[`\0\x80a\x04u\x85a\t\xC0V[\x90P`\0\x80`\0a\x04\x85\x88a\x0B\x0BV[\x92P\x92P\x92Pa\x04\x18\x87\x84\x84\x84\x8A\x89a\x0E\x1CV[`\0\x80`\0``a\x04\xC4`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[a\x04\xE8`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[a\x04\xF1\x89a\x0B\x0BV[`@\x85\x01R` \x84\x01R\x82R`\0a\x05\x08\x8Aa\t\xC0V[\x90P`\0\x80a\x05(\x85`\0\x01Q\x86` \x01Qa\x05#\x8Fa\t\xC0V[a\x0EQV[\x90P\x8A\x15a\x06fW`\0a\x05I\x84`@\x01Q\x8Ca\x0E|\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x05v\x85` \x01Qa\x05p\x89`\0\x01Q\x8A` \x01Qa\x0E\x91\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90a\x0E\xA6V[\x90P`\0a\x05\x84\x83\x83a\x0E|V[\x90Pa\x05\x91`\x01\x82a#\xBEV[\x88Q\x90\x91Pa\x05\xA1\x90\x8E\x90a#\xBEV[\x87Ra\x05\xAD\x81\x85a#\xBEV[\x87`@\x01\x81\x81RPPa\x05\xC9\x8F\x88`\0\x01Q\x89`@\x01Qa\x0C\xDEV[` \x88\x01\x81\x81R`\x01\x91a\x05\xDE\x90\x83\x90a#\xBEV[\x90RP` \x80\x89\x01Q\x90\x88\x01Q\x10a\x06HW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: y reserve increase`D\x82\x01Rad!`\xF0\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[\x86` \x01Q\x88` \x01Qa\x06\\\x91\x90a#\xD1V[\x94PPPPa\x07{V[`\0a\x06\x7F\x84`@\x01Q\x8Ca\x0E|\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x84Q` \x88\x01Q\x88Q\x92\x93P`\0\x92a\x06\x9D\x92\x91a\x05p\x91\x90a\x0E\x91V[\x90P`\0a\x06\xAB\x83\x83a\x0E|V[\x90Pa\x06\xB8`\x01\x82a#\xBEV[\x90P\x8C\x88` \x01Qa\x06\xCA\x91\x90a#\xBEV[` \x88\x01Ra\x06\xD9\x81\x85a#\xBEV[\x87`@\x01\x81\x81RPPa\x06\xF5\x8F\x88` \x01Q\x89`@\x01Qa\t\x12V[\x80\x88R`\x01\x90\x88\x90a\x07\x08\x90\x83\x90a#\xBEV[\x90RP\x87Q\x87Q\x10a\x07gW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: x reserve increase`D\x82\x01Rad!`\xF0\x1B`d\x82\x01R`\x84\x01a\x06?V[\x86Q\x88Qa\x07u\x91\x90a#\xD1V[\x94PPPP[\x83Q` \x80\x86\x01Q`@\x80\x88\x01Q\x81Q\x93\x84\x01\x94\x90\x94R\x82\x01R``\x81\x01\x91\x90\x91R`\0\x90`\x80\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R`\0\x80Tc\r\x17\xA7\xC7`\xE3\x1B\x84R\x91\x93P\x8F\x92\x90\x91`\x01`\x01`\xA0\x1B\x03\x16\x90ch\xBD>8\x90a\x07\xEB\x900\x90\x86\x90\x88\x90`\x04\x01a#\xE4V[`\xC0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x08AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a(`\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x08UW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08y\x91\x90a$\x0BV[PPPPP\x90P\x80\x85a\x08\x95\x89`\0\x01Q\x8A` \x01Q\x8Aa\x0E\xD7V[\x85\x9BP\x9BP\x9BP\x9BPPPPPPPPP\x93P\x93P\x93P\x93V[`\0\x80a\x08\xBB\x83a\t\xC0V[\x90P`\0\x80a\x08\xC9\x85a\x0B\x0BV[P\x91P\x91Pa\x08\xD9\x82\x82\x85a\x0E\xD7V[\x95\x94PPPPPV[`\0\x80a\x08\xEE\x85a\t\xC0V[\x90P`\0\x80`\0a\x08\xFE\x88a\x0B\x0BV[\x92P\x92P\x92Pa\x04\x18\x87\x84\x84\x84\x8A\x89a\x0F\x10V[`\0a\t'\x83\x83a\t\"\x87a\t\xC0V[a\x0FvV[\x94\x93PPPPV[`\0\x80`\0\x80`\0a\t@\x87a\x0B\x0BV[\x92PP\x91P`\0\x80a\tU`\0\x89\x86\x86a\x0F\xB3V[\x91P\x91P`\0a\tf\x8A\x84\x84a\x0C\xDEV[\x92\x97P\x91\x95P\x93PPPP\x92P\x92P\x92V[`\0\x80`\0\x80`\0a\t\x89\x87a\x0B\x0BV[\x92P\x92PP`\0\x80a\t\x9E`\x01\x89\x86\x86a\x0F\xB3V[\x91P\x91P`\0a\t\xAF\x8A\x84\x84a\t\x12V[\x97P\x91\x95P\x93PPPP\x92P\x92P\x92V[a\t\xF4`@Q\x80`\x80\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81RP\x90V[`\0T`@Qc\xDC\x17\x83U`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xDC\x17\x83U\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\nvW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a(`\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\n\x8AW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\n\xB2\x91\x90\x81\x01\x90a$aV[\x80` \x01\x90Q\x81\x01\x90a\x043\x91\x90a%\x9CV[`\0\x80a\n\xD1\x85a\t\xC0V[\x90P`\0\x80`\0a\n\xE1\x88a\x0B\x0BV[\x92P\x92P\x92Pa\x04\x18\x87\x84\x84\x84\x8A\x89a\x10\x1CV[``a\x043\x82a\x10\xC0V[``a\x043\x82a\x10\xECV[`\0\x80`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xAF\xBA\x13\xC4`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0B\x9BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a(`\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0B\xAFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\xD3\x91\x90a%\xBBV[`\x01`\x01`\xA0\x1B\x03\x16c\xCE\x15;\xF4\x85`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0C\0\x91\x81R` \x01\x90V[```@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0CVW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a(`\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0CjW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\x8E\x91\x90a%\xDBV[\x92P\x92P\x92P\x91\x93\x90\x92PV[``a\t'\x84\x84\x84a\x11\x02V[`\0a\t'\x83\x83a\x05#\x87a\t\xC0V[`\0\x80`\0\x80`\0a\x0C\xC9\x87a\x0B\x0BV[\x92PP\x91P`\0\x80a\tU`\x01\x89\x86\x86a\x0F\xB3V[`\0a\t'\x83\x83a\x0C\xEE\x87a\t\xC0V[a\x11\xA0V[`\0\x80`\0\x80`\0a\r\x04\x87a\x0B\x0BV[\x92P\x92PP`\0\x80a\t\x9E`\0\x89\x86\x86a\x0F\xB3V[\x80Q`\0\x90\x81\x90a\r.\x90a\x05p\x88\x87a\x11\xD5V[\x90P`\0a\rM\x84` \x01Qa\x05p\x87\x89a\x11\xD5\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0a\rb\x83\x83a\x0E|V[a\rl\x91\x90a&\x0CV[\x97\x96PPPPPPPV[```\x02\x83\x83`@Q` \x01a\r\x8F\x93\x92\x91\x90a&UV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x92\x91PPV[`\0\x82a\x03\xE8\x82a\r\xBB\x8A\x8A\x8A\x8A\x86\x8Aa\x0E\x1CV[\x90P`\0\x81\x12\x15a\r\xD2W`\0\x93PPPPa\x0E\x12V[a\x0E\x0C\x8A\x8A\x8A\x8A\x89`@Q` \x01a\r\xEE\x95\x94\x93\x92\x91\x90a&tV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83\x85`\x01a\x01\0a\x11\xEAa\x12\x1FV[\x93PPPP[\x96\x95PPPPPPV[`\0\x80\x82`@\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x0E7\x91\x90a#\xD1V[\x90Pa\x04\x18\x83`\0\x01Q\x84` \x01Q\x89\x89\x8C\x8A\x8A\x88a\x130V[`\0a\t'a\x0Em\x83` \x01Q\x85a\x0E\xA6\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x83Qa\x0Ez\x90\x87\x90a\x0E\xA6V[\x90[`\0a\x040\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x13\xDFV[`\0a\x040\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x13\xDFV[`\0a\x040g\r\xE0\xB6\xB3\xA7d\0\0\x83a\x0E\xBE\x86a\x14\rV[a\x0E\xC8\x91\x90a&\x9FV[a\x0E\xD2\x91\x90a&\xE5V[a\x15\xE8V[`\0\x80a\x0E\xF1\x83` \x01Q\x85a\x11\xD5\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x83Q\x90\x91P`\0\x90a\x0F\x04\x90\x87\x90a\x11\xD5V[\x90Pa\x0E\x12\x82\x82a\x11\xD5V[`\0\x82a\x03\xE8\x82a\x0F%\x8A\x8A\x8A\x8A\x86\x8Aa\x10\x1CV[\x90P`\0\x81\x12\x15a\x0F<W`\0\x93PPPPa\x0E\x12V[a\x0E\x0C\x8A\x8A\x8A\x8A\x89`@Q` \x01a\x0FX\x95\x94\x93\x92\x91\x90a&tV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83\x85`\x01a\x01\0a\x17\x91a\x12\x1FV[\x80Q`\0\x90a\t'\x90a\x0F\x92\x90g\r\xE0\xB6\xB3\xA7d\0\0\x90a\x0E\x91V[a\x05pa\x0F\xAC\x85` \x01Q\x88a\x0E\xA6\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x86\x90a\x0E\x91V[`\0\x80\x80a\x0F\xC1\x84\x86a\x0E\x91V[\x90P`\0a\x0F\xCF\x87\x83a\x0E|V[\x90P\x87a\x0F\xE5Wa\x0F\xE0\x87\x87a#\xD1V[a\x0F\xEFV[a\x0F\xEF\x87\x87a#\xBEV[\x93P\x87a\x10\x05Wa\x10\0\x81\x86a#\xD1V[a\x10\x0FV[a\x10\x0F\x81\x86a#\xBEV[\x92PPP\x94P\x94\x92PPPV[`\0\x80a\x106\x83`\0\x01Q\x88\x86\x89\x89\x8D\x89`@\x01Qa\x17\xC6V[\x90P`\0a\x10C\x82a\x19\x0BV[\x90P`\0\x80a\x10]a\x10U\x8A\x89a'\x13V[\x87Q\x90a\x1A0V[\x90P`\0a\x10\x91\x85`\xE0\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x10{\x90a';V[a\x10\x85\x91\x90a'\x13V[a\x01 \x87\x01Q\x90a\x1A0V[a\x10\x9A\x8Aa';V[a\x10\xA4\x91\x90a'\x13V[\x90Pa\x10\xB0\x82\x82a\x1A0V[\x92Pa\x0E\x0C\x91P\x83\x90P\x82a\x1AcV[```\x01\x82`@Q` \x01a\x10\xD6\x92\x91\x90a'WV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x91\x90PV[```\x03\x82`@Q` \x01a\x10\xD6\x92\x91\x90a'rV[```\0a\x11\x11\x85\x85\x85a\x1A\x87V[\x90P`\0a\x11 \x86\x83\x86a\x1A\xB2V[\x90P`\0a\x110\x87\x84\x84\x88a\r\x19V[\x90Pa\x11?\x87\x84\x83\x85\x89a\x1A\xEBV[\x85Q`@\x80\x88\x01Q``\x80\x8A\x01Q\x83Q` \x81\x01\x8E\x90R\x93\x84\x01\x89\x90R\x90\x83\x01\x85\x90R`\x80\x83\x01\x93\x90\x93R`\xA0\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\xC0\x82\x01R\x90\x92P`\xE0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x93PPPP\x93\x92PPPV[`\0a\t'a\x11\xC4\x83` \x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x0E\x91\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x83Qa\x05p\x90a\x0F\xAC\x90\x88\x90a\x0E\xA6V[`\0a\x040\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x1B\x9DV[`\0\x80`\0\x80`\0\x80\x87\x80` \x01\x90Q\x81\x01\x90a\x12\x07\x91\x90a'\x98V[\x94P\x94P\x94P\x94P\x94Pa\x04\x18\x85\x85\x85\x85\x8B\x86a\x0E\x1CV[`\0\x84\x86\x11\x15a\x12LW`@Qc0\x82\xDF\xDB`\xE1\x1B\x81R`\x04\x81\x01\x87\x90R`$\x81\x01\x86\x90R`D\x01a\x06?V[`\0a\x12\\\x88\x88\x85c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x12n\x89\x88\x86c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x12|\x82\x84a&\x9FV[\x13\x15a\x12\xA5W`@Qc\x06\xF1\xBE]`\xE2\x1B\x81R`\x04\x81\x01\x83\x90R`$\x81\x01\x82\x90R`D\x01a\x06?V[`\0a\x12\xB1\x89\x89a#\xD1V[\x90P`\0[`\x02a\x12\xC2\x8A\x8Ca#\xBEV[a\x12\xCC\x91\x90a'\xE6V[\x94P`\0a\x12\xDE\x8C\x87\x89c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x12\xEC\x86\x83a&\x9FV[\x13a\x12\xF9W\x85\x99Pa\x13\0V[\x85\x9AP\x80\x94P[a\x13\n\x8B\x8Ba#\xD1V[\x92PP`\x01\x01\x87\x82\x11\x80\x15a\x13\x1EWP\x86\x81\x10[a\x12\xB6WPPPP\x96\x95PPPPPPV[`\0\x80a\x13=\x87\x89a\x11\xD5V[\x90P`\0`@Q\x80a\x01\0\x01`@R\x80\x8C\x81R` \x01\x8A\x81R` \x01\x89\x81R` \x01\x87\x81R` \x01\x86\x81R` \x01a\x13~\x8D\x85a\x0E\xA6\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x81R` \x01a\x13\x8D\x84\x8Da\x0E\xA6V[\x81R` \x01\x85\x90R\x90P`\0a\x13\xA2\x82a\x1B\xBCV[\x90P`\0a\x13\xAF\x83a\x1C\xFAV[\x90Pa\x13\xBB\x82\x82a\x11\xD5V[a\x13\xC4\x8Aa';V[a\x13\xCE\x91\x90a'\x13V[\x9D\x9CPPPPPPPPPPPPPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x13\xF7W`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[`\0\x80\x82\x13a\x14JW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x06?V[`\0``a\x14W\x84a\x1D\x9DV[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[`\0h\x02H\xCE6\xA7\x0C\xB2k>\x19\x82\x13a\x16\x03WP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a\x16JW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x01a\x06?V[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05`\x01`_\x1B\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1Dm\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1Dn\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[`\0\x80`\0\x80`\0\x80\x87\x80` \x01\x90Q\x81\x01\x90a\x17\xAE\x91\x90a'\x98V[\x94P\x94P\x94P\x94P\x94Pa\x04\x18\x85\x85\x85\x85\x8B\x86a\x10\x1CV[a\x18\x1C`@Q\x80a\x01@\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`\0a\x18G\x89a\x183g\r\xE0\xB6\xB3\xA7d\0\0a';V[a\x18=\x91\x90a'\x13V[a\x05p\x88\x8Aa'\x13V[\x90P`\0a\x18Y\x8Aa\x05p\x8B\x8Aa\x1AcV[\x90P`\0a\x18g\x89\x83a\x1A0V[\x90P`\0a\x18}\x86g\r\xE0\xB6\xB3\xA7d\0\0a&\x0CV[\x90P`\0a\x18\xA7\x82a\x18\x96g\r\xE0\xB6\xB3\xA7d\0\0a';V[a\x18\xA0\x91\x90a'\x13V[\x84\x90a\x1A0V[a\x18\xB1\x90\x8Aa&\x0CV[\x90P`@Q\x80a\x01@\x01`@R\x80\x8E\x81R` \x01\x8C\x81R` \x01\x8B\x81R` \x01\x82\x81R` \x01\x86\x81R` \x01\x8A\x81R` \x01\x85\x81R` \x01\x83\x81R` \x01\x89\x81R` \x01\x84\x81RP\x95PPPPPP\x97\x96PPPPPPPV[`\0\x80a\x19-\x83`@\x01Q\x84` \x01Qa\x19%\x91\x90a'\x13V[\x84Q\x90a\x1A0V[\x90P`\0a\x19fa\x19S\x85`\0\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x1Ac\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[``\x86\x01Q`\x80\x87\x01Qa\x05p\x91a\x1A0V[\x90P`\0a\x19\x99\x85`\0\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x19\x84\x90a';V[a\x19\x8E\x91\x90a'\x13V[`\xA0\x87\x01Q\x90a\x1A0V[\x90P`\0a\x19\xF7\x86`\xE0\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x19\xB7\x90a';V[a\x19\xC1\x91\x90a'\x13V[`@\x88\x01Q\x88Q` \x8A\x01Qa\x19\xF1\x92\x91a\x19\xDC\x91\x90a\x1A0V[a\x19\xE6\x91\x90a'\x13V[`\xC0\x8A\x01Q\x90a\x1A0V[\x90a\x1A0V[\x90Pa\x1A\x16a\x1A\x06\x82\x84a&\x0CV[a\x01\0\x88\x01Qa\x19\xF1\x90\x86a\x1A0V[``\x87\x01Qa\x1A&\x90\x86\x90a\x1A0V[a\x0E\x12\x91\x90a'\x13V[\x81\x81\x02\x82\x15\x83\x82\x05\x83\x14\x17`\0\x19\x84\x10`\x01`\xFF\x1B\x84\x13\x17\x16a\x1ARW`\0\x80\xFD[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x92\x91PPV[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x81\x02\x90\x81\x05\x83\x14\x82\x15\x15\x16a\x1A\x81W`\0\x80\xFD[\x05\x91\x90PV[`\0a\t'\x84a\x1A\xAC\x85a\x1A\xAC\x86`\0\x01Q\x87` \x01Qa\x11\xD5\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90a\x1EEV[\x80Q`\0\x90\x81\x90a\x1A\xC4\x90\x86\x90a\x0E\xA6V[\x90P`\0a\x1A\xDF\x84` \x01Q\x86a\x0E\xA6\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa\x0E\x12\x82\x82a\x0E|V[`\0\x82\x80\x85\x83\x81\x12\x15a\x1B+W[`\0\x81\x12\x15a\x1B&Wa\x1B\x11\x82a\x03\xE7a\x03\xE8a\x1B\x9DV[\x91Pa\x1B\x1F\x89\x89\x84\x88a\r\x19V[\x90Pa\x1A\xF9V[a\x1BXV[`\0\x81\x13\x15a\x1BXWa\x1BC\x83a\x03\xE9a\x03\xE8a\x13\xDFV[\x92Pa\x1BQ\x89\x89\x85\x88a\r\x19V[\x90Pa\x1B+V[a\x1B\x90\x89\x89\x83\x88`@Q` \x01a\x1Br\x94\x93\x92\x91\x90a'\xFAV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83\x85`\x01a\x01\0a\x1EZa\x12\x1FV[\x99\x98PPPPPPPPPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x1B\xB5W`\0\x80\xFD[\x04\x92\x91PPV[`\0\x80a\x1B\xEA\x83`\xA0\x01Qa\x1A\xAC\x85` \x01Qa\x1A\xAC\x87`\0\x01Q\x88``\x01Qa\x1EE\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x1CJ\x84`\xE0\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x1C\t\x91\x90a#\xD1V[a\x1A\xAC\x86`@\x01Q\x87` \x01Qa\x1C1\x89`\0\x01Q\x8A`\x80\x01Qa\x1EE\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x89`\x80\x01Qa\x1C@\x91\x90a#\xD1V[a\x1A\xAC\x91\x90a#\xBEV[\x90P`\0a\x1Cs\x85`\0\x01Qa\x1C_\x90a';V[\x86` \x01Q\x87`\x80\x01Qa\x05p\x91\x90a#\xBEV[\x90P`\0a\x1C\xA5\x86`\xE0\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x1C\x92\x91\x90a#\xD1V[`\xC0\x88\x01Q`\x80\x89\x01Qa\x1A\xAC\x91a\x1EEV[\x86``\x01Qa\x1C\xB4\x91\x90a#\xBEV[\x90Pa\x0E\x12a\x1C\xF0a\x1C\xE6\x88`\0\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x1C\xD7\x91\x90a#\xD1V[g\r\xE0\xB6\xB3\xA7d\0\0\x90a\x11\xD5V[a\x05p\x85\x85a\x1EEV[a\x1A\xAC\x85\x87a#\xD1V[\x80Q`\0\x90\x81\x90a\x1D\x13\x90g\r\xE0\xB6\xB3\xA7d\0\0a#\xD1V[\x90P`\0\x83` \x01Q\x84`\x80\x01Qa\x1D+\x91\x90a#\xBEV[\x90P`\0a\x1DR\x85`\xA0\x01Qa\x1A\xAC\x87` \x01Q\x88``\x01Qa\x1EE\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x1D\x84\x86`\xE0\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x1Dq\x91\x90a#\xD1V[`@\x88\x01Q`\x80\x89\x01Qa\x1A\xAC\x91a\x1EEV[\x90Pa\x0E\x12a\x1D\x93\x82\x84a#\xBEV[a\x1A\xAC\x86\x86a\x1EEV[`\0\x80\x82\x11a\x1D\xDAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x06?V[P`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x82\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[`\0a\x040\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x1B\x9DV[`\0\x80`\0\x80\x85\x80` \x01\x90Q\x81\x01\x90a\x1Et\x91\x90a(\x1CV[\x93PP\x92P\x92Pa\x0E\x12\x83\x83\x87\x84a\r\x19V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x81\xFD[`\0\x80`\0`@\x84\x86\x03\x12\x15a\x1F\x98Wa\x1F\x98a\x1E\x87V[\x835\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x1F\xBAWa\x1F\xBAa\x1E\xD7V[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x1F\xD1Wa\x1F\xD1a\x1F'V[\x815\x81\x81\x11\x15a 4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x0ENL/$\r\x8C\xAD\xCC\xEE\x8D`\xAB\x1B`d\x82\x01R`\x84\x81\xFD[\x87` \x82\x85\x01\x01\x11\x15a \x9CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rjrray stride`\xA8\x1B`d\x82\x01R\x91P`\x84\x82\xFD[` \x83\x01\x94P\x80\x93PPPP\x92P\x92P\x92V[`\0\x80`@\x83\x85\x03\x12\x15a \xC5Wa \xC5a\x1E\x87V[PP\x805\x92` \x90\x91\x015\x91PV[`\0[\x83\x81\x10\x15a \xEFW\x81\x81\x01Q\x83\x82\x01R` \x01a \xD7V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra!\x10\x81` \x86\x01` \x86\x01a \xD4V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x040` \x83\x01\x84a \xF8V[`\0\x80`\0``\x84\x86\x03\x12\x15a!OWa!Oa\x1E\x87V[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[\x80\x15\x15\x81\x14a!tW`\0\x80\xFD[PV[`\0\x80`\0``\x84\x86\x03\x12\x15a!\x8FWa!\x8Fa\x1E\x87V[\x835\x92P` \x84\x015a!\xA1\x81a!fV[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[\x84\x15\x15\x81R\x83` \x82\x01R\x82`@\x82\x01R`\x80``\x82\x01R`\0a\x0E\x12`\x80\x83\x01\x84a \xF8V[`\0` \x82\x84\x03\x12\x15a!\xEEWa!\xEEa\x1E\x87V[P5\x91\x90PV[\x80Q\x82R` \x80\x82\x01Q\x90\x83\x01R`@\x80\x82\x01Q\x90\x83\x01R``\x90\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x91\x01RV[`\x80\x81\x01a\x043\x82\x84a!\xF5V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a!tW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\"YWa\"Ya\x1E\x87V[\x815a\"d\x81a\"/V[\x93\x92PPPV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FABI decoding: struct data too sh`D\x82\x01Rb\x1B\xDC\x9D`\xEA\x1B`d\x82\x01R`\x84\x81\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x80\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\"\xF5Wa\"\xF5a\"\xBCV[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a#$Wa#$a\"\xBCV[`@R\x91\x90PV[`\0\x80`\0\x83\x85\x03`\xC0\x81\x12\x15a#EWa#Ea\x1E\x87V[\x845\x93P` \x85\x015\x92P`\x80`?\x19\x82\x01\x12\x15a#eWa#ea\"kV[Pa#na\"\xD2V[`@\x85\x015\x81R``\x85\x015` \x82\x01R`\x80\x85\x015`@\x82\x01R`\xA0\x85\x015a#\x97\x81a\"/V[``\x82\x01R\x92\x95\x91\x94P\x91\x92P\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x043Wa\x043a#\xA8V[\x81\x81\x03\x81\x81\x11\x15a\x043Wa\x043a#\xA8V[`\x01\x80`\xA0\x1B\x03\x84\x16\x81R\x82` \x82\x01R```@\x82\x01R`\0a\x08\xD9``\x83\x01\x84a \xF8V[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a$'Wa$'a\x1E\x87V[\x86Qa$2\x81a!fV[` \x88\x01Q`@\x89\x01Q``\x8A\x01Q`\x80\x8B\x01Q`\xA0\x90\x9B\x01Q\x93\x9C\x92\x9BP\x90\x99\x90\x98P\x96P\x90\x94P\x92PPPV[`\0` \x80\x83\x85\x03\x12\x15a$wWa$wa\x1E\x87V[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a$\x92Wa$\x92a\x1E\xD7V[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a$\xA9Wa$\xA9a\x1F'V[\x81Q\x81\x81\x11\x15a$\xBBWa$\xBBa\"\xBCV[a$\xCD`\x1F\x82\x01`\x1F\x19\x16\x85\x01a\"\xFBV[\x91P\x80\x82R\x86\x84\x82\x85\x01\x01\x11\x15a%3W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x85\x90R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R`\x84\x81\xFD[a%B\x81\x85\x84\x01\x86\x86\x01a \xD4V[P\x95\x94PPPPPV[`\0`\x80\x82\x84\x03\x12\x15a%aWa%aa\"kV[a%ia\"\xD2V[\x90P\x81Q\x81R` \x82\x01Q` \x82\x01R`@\x82\x01Q`@\x82\x01R``\x82\x01Qa%\x91\x81a\"/V[``\x82\x01R\x92\x91PPV[`\0`\x80\x82\x84\x03\x12\x15a%\xB1Wa%\xB1a\x1E\x87V[a\x040\x83\x83a%LV[`\0` \x82\x84\x03\x12\x15a%\xD0Wa%\xD0a\x1E\x87V[\x81Qa\"d\x81a\"/V[`\0\x80`\0``\x84\x86\x03\x12\x15a%\xF3Wa%\xF3a\x1E\x87V[\x83Q\x92P` \x84\x01Q\x91P`@\x84\x01Q\x90P\x92P\x92P\x92V[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a&,Wa&,a#\xA8V[P\x92\x91PPV[`\x04\x81\x10a&QWcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x90RV[``\x81\x01a&c\x82\x86a&3V[` \x82\x01\x93\x90\x93R`@\x01R\x91\x90PV[`\0a\x01\0\x82\x01\x90P\x86\x82R\x85` \x83\x01R\x84`@\x83\x01R\x83``\x83\x01Ra\x0E\x12`\x80\x83\x01\x84a!\xF5V[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15a&\xBBWa&\xBBa#\xA8V[\x81\x81\x05\x83\x14\x82\x15\x17a\x043Wa\x043a#\xA8V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a&\xF4Wa&\xF4a&\xCFV[`\x01`\xFF\x1B\x82\x14`\0\x19\x84\x14\x16\x15a'\x0EWa'\x0Ea#\xA8V[P\x05\x90V[\x80\x82\x01\x82\x81\x12`\0\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a'3Wa'3a#\xA8V[PP\x92\x91PPV[`\0`\x01`\xFF\x1B\x82\x01a'PWa'Pa#\xA8V[P`\0\x03\x90V[`@\x81\x01a'e\x82\x85a&3V[\x82` \x83\x01R\x93\x92PPPV[`@\x81\x01a'\x80\x82\x85a&3V[`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16` \x91\x90\x91\x01R\x91\x90PV[`\0\x80`\0\x80`\0a\x01\0\x86\x88\x03\x12\x15a'\xB4Wa'\xB4a\x1E\x87V[\x85Q\x94P` \x86\x01Q\x93P`@\x86\x01Q\x92P``\x86\x01Q\x91Pa'\xDA\x87`\x80\x88\x01a%LV[\x90P\x92\x95P\x92\x95\x90\x93PV[`\0\x82a'\xF5Wa'\xF5a&\xCFV[P\x04\x90V[\x84\x81R` \x81\x01\x84\x90R`@\x81\x01\x83\x90R`\xE0\x81\x01a\x08\xD9``\x83\x01\x84a!\xF5V[`\0\x80`\0\x80`\xE0\x85\x87\x03\x12\x15a(5Wa(5a\x1E\x87V[\x84Q\x93P` \x85\x01Q\x92P`@\x85\x01Q\x91Pa(T\x86``\x87\x01a%LV[\x90P\x92\x95\x91\x94P\x92PV\xFETarget contract does not contain\xA2dipfsX\"\x12 \xF3Z|\xDE\xE8@\xE2l!\xC6!j\x88*+\xAA\x86H\xCC\x91\x981\xB7vr\xC5#\xFB\xE3\"\xBD\x0EdsolcC\0\x08\x16\x003";
    /// The deployed bytecode of the contract.
    pub static GEOMETRICMEANSOLVER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct GeometricMeanSolver<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for GeometricMeanSolver<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for GeometricMeanSolver<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for GeometricMeanSolver<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for GeometricMeanSolver<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(GeometricMeanSolver))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> GeometricMeanSolver<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    GEOMETRICMEANSOLVER_ABI.clone(),
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
                GEOMETRICMEANSOLVER_ABI.clone(),
                GEOMETRICMEANSOLVER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
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
        ///Calls the contract's `checkSwapConstant` (0x0f4166b8) function
        pub fn check_swap_constant(
            &self,
            pool_id: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([15, 65, 102, 184], (pool_id, data))
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
        ) -> ::ethers::contract::builders::ContractCall<M, GeometricMeanParams> {
            self.0
                .method_hash([129, 181, 250, 194], pool_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getInitialPoolData` (0xdef15f92) function
        pub fn get_initial_pool_data(
            &self,
            rx: ::ethers::core::types::U256,
            s: ::ethers::core::types::U256,
            params: GeometricMeanParams,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([222, 241, 95, 146], (rx, s, params))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getNextLiquidity` (0xec29d8e6) function
        pub fn get_next_liquidity(
            &self,
            pool_id: ::ethers::core::types::U256,
            rx: ::ethers::core::types::U256,
            ry: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([236, 41, 216, 230], (pool_id, rx, ry))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getNextReserveX` (0x5a93b8ce) function
        pub fn get_next_reserve_x(
            &self,
            pool_id: ::ethers::core::types::U256,
            ry: ::ethers::core::types::U256,
            l: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([90, 147, 184, 206], (pool_id, ry, l))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getNextReserveY` (0xf2de7a7b) function
        pub fn get_next_reserve_y(
            &self,
            pool_id: ::ethers::core::types::U256,
            rx: ::ethers::core::types::U256,
            l: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([242, 222, 122, 123], (pool_id, rx, l))
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
        ///Calls the contract's `prepareWeightXUpdate` (0x250968d9) function
        pub fn prepare_weight_x_update(
            &self,
            target_weight_x: ::ethers::core::types::U256,
            target_timestamp: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([37, 9, 104, 217], (target_weight_x, target_timestamp))
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
    for GeometricMeanSolver<M> {
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
    pub enum GeometricMeanSolverErrors {
        BisectionLib_InvalidBounds(BisectionLib_InvalidBounds),
        BisectionLib_RootOutsideBounds(BisectionLib_RootOutsideBounds),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for GeometricMeanSolverErrors {
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
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for GeometricMeanSolverErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::BisectionLib_InvalidBounds(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BisectionLib_RootOutsideBounds(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for GeometricMeanSolverErrors {
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
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for GeometricMeanSolverErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::BisectionLib_InvalidBounds(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BisectionLib_RootOutsideBounds(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for GeometricMeanSolverErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<BisectionLib_InvalidBounds>
    for GeometricMeanSolverErrors {
        fn from(value: BisectionLib_InvalidBounds) -> Self {
            Self::BisectionLib_InvalidBounds(value)
        }
    }
    impl ::core::convert::From<BisectionLib_RootOutsideBounds>
    for GeometricMeanSolverErrors {
        fn from(value: BisectionLib_RootOutsideBounds) -> Self {
            Self::BisectionLib_RootOutsideBounds(value)
        }
    }
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
    ///Container type for all input parameters for the `checkSwapConstant` function with signature `checkSwapConstant(uint256,bytes)` and selector `0x0f4166b8`
    #[derive(
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
    #[ethcall(name = "checkSwapConstant", abi = "checkSwapConstant(uint256,bytes)")]
    pub struct CheckSwapConstantCall {
        pub pool_id: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
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
    ///Container type for all input parameters for the `getInitialPoolData` function with signature `getInitialPoolData(uint256,uint256,(uint256,uint256,uint256,address))` and selector `0xdef15f92`
    #[derive(
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
        abi = "getInitialPoolData(uint256,uint256,(uint256,uint256,uint256,address))"
    )]
    pub struct GetInitialPoolDataCall {
        pub rx: ::ethers::core::types::U256,
        pub s: ::ethers::core::types::U256,
        pub params: GeometricMeanParams,
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
        Hash
    )]
    #[ethcall(
        name = "getNextLiquidity",
        abi = "getNextLiquidity(uint256,uint256,uint256)"
    )]
    pub struct GetNextLiquidityCall {
        pub pool_id: ::ethers::core::types::U256,
        pub rx: ::ethers::core::types::U256,
        pub ry: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getNextReserveX` function with signature `getNextReserveX(uint256,uint256,uint256)` and selector `0x5a93b8ce`
    #[derive(
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
        abi = "getNextReserveX(uint256,uint256,uint256)"
    )]
    pub struct GetNextReserveXCall {
        pub pool_id: ::ethers::core::types::U256,
        pub ry: ::ethers::core::types::U256,
        pub l: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getNextReserveY` function with signature `getNextReserveY(uint256,uint256,uint256)` and selector `0xf2de7a7b`
    #[derive(
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
        abi = "getNextReserveY(uint256,uint256,uint256)"
    )]
    pub struct GetNextReserveYCall {
        pub pool_id: ::ethers::core::types::U256,
        pub rx: ::ethers::core::types::U256,
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
    ///Container type for all input parameters for the `prepareWeightXUpdate` function with signature `prepareWeightXUpdate(uint256,uint256)` and selector `0x250968d9`
    #[derive(
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
        name = "prepareWeightXUpdate",
        abi = "prepareWeightXUpdate(uint256,uint256)"
    )]
    pub struct PrepareWeightXUpdateCall {
        pub target_weight_x: ::ethers::core::types::U256,
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
    pub enum GeometricMeanSolverCalls {
        AllocateGivenX(AllocateGivenXCall),
        AllocateGivenY(AllocateGivenYCall),
        CalculateDiffLower(CalculateDiffLowerCall),
        CalculateDiffRaise(CalculateDiffRaiseCall),
        CheckSwapConstant(CheckSwapConstantCall),
        ComputeOptimalArbLowerPrice(ComputeOptimalArbLowerPriceCall),
        ComputeOptimalArbRaisePrice(ComputeOptimalArbRaisePriceCall),
        DeallocateGivenX(DeallocateGivenXCall),
        DeallocateGivenY(DeallocateGivenYCall),
        FetchPoolParams(FetchPoolParamsCall),
        GetInitialPoolData(GetInitialPoolDataCall),
        GetNextLiquidity(GetNextLiquidityCall),
        GetNextReserveX(GetNextReserveXCall),
        GetNextReserveY(GetNextReserveYCall),
        GetReservesAndLiquidity(GetReservesAndLiquidityCall),
        InternalPrice(InternalPriceCall),
        PrepareControllerUpdate(PrepareControllerUpdateCall),
        PrepareFeeUpdate(PrepareFeeUpdateCall),
        PrepareWeightXUpdate(PrepareWeightXUpdateCall),
        SimulateSwap(SimulateSwapCall),
        Strategy(StrategyCall),
    }
    impl ::ethers::core::abi::AbiDecode for GeometricMeanSolverCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
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
            if let Ok(decoded) = <CheckSwapConstantCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CheckSwapConstant(decoded));
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
            if let Ok(decoded) = <PrepareWeightXUpdateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PrepareWeightXUpdate(decoded));
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
    impl ::ethers::core::abi::AbiEncode for GeometricMeanSolverCalls {
        fn encode(self) -> Vec<u8> {
            match self {
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
                Self::CheckSwapConstant(element) => {
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
                Self::PrepareWeightXUpdate(element) => {
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
    impl ::core::fmt::Display for GeometricMeanSolverCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AllocateGivenX(element) => ::core::fmt::Display::fmt(element, f),
                Self::AllocateGivenY(element) => ::core::fmt::Display::fmt(element, f),
                Self::CalculateDiffLower(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CalculateDiffRaise(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CheckSwapConstant(element) => ::core::fmt::Display::fmt(element, f),
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
                Self::GetReservesAndLiquidity(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InternalPrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::PrepareControllerUpdate(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PrepareFeeUpdate(element) => ::core::fmt::Display::fmt(element, f),
                Self::PrepareWeightXUpdate(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SimulateSwap(element) => ::core::fmt::Display::fmt(element, f),
                Self::Strategy(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AllocateGivenXCall> for GeometricMeanSolverCalls {
        fn from(value: AllocateGivenXCall) -> Self {
            Self::AllocateGivenX(value)
        }
    }
    impl ::core::convert::From<AllocateGivenYCall> for GeometricMeanSolverCalls {
        fn from(value: AllocateGivenYCall) -> Self {
            Self::AllocateGivenY(value)
        }
    }
    impl ::core::convert::From<CalculateDiffLowerCall> for GeometricMeanSolverCalls {
        fn from(value: CalculateDiffLowerCall) -> Self {
            Self::CalculateDiffLower(value)
        }
    }
    impl ::core::convert::From<CalculateDiffRaiseCall> for GeometricMeanSolverCalls {
        fn from(value: CalculateDiffRaiseCall) -> Self {
            Self::CalculateDiffRaise(value)
        }
    }
    impl ::core::convert::From<CheckSwapConstantCall> for GeometricMeanSolverCalls {
        fn from(value: CheckSwapConstantCall) -> Self {
            Self::CheckSwapConstant(value)
        }
    }
    impl ::core::convert::From<ComputeOptimalArbLowerPriceCall>
    for GeometricMeanSolverCalls {
        fn from(value: ComputeOptimalArbLowerPriceCall) -> Self {
            Self::ComputeOptimalArbLowerPrice(value)
        }
    }
    impl ::core::convert::From<ComputeOptimalArbRaisePriceCall>
    for GeometricMeanSolverCalls {
        fn from(value: ComputeOptimalArbRaisePriceCall) -> Self {
            Self::ComputeOptimalArbRaisePrice(value)
        }
    }
    impl ::core::convert::From<DeallocateGivenXCall> for GeometricMeanSolverCalls {
        fn from(value: DeallocateGivenXCall) -> Self {
            Self::DeallocateGivenX(value)
        }
    }
    impl ::core::convert::From<DeallocateGivenYCall> for GeometricMeanSolverCalls {
        fn from(value: DeallocateGivenYCall) -> Self {
            Self::DeallocateGivenY(value)
        }
    }
    impl ::core::convert::From<FetchPoolParamsCall> for GeometricMeanSolverCalls {
        fn from(value: FetchPoolParamsCall) -> Self {
            Self::FetchPoolParams(value)
        }
    }
    impl ::core::convert::From<GetInitialPoolDataCall> for GeometricMeanSolverCalls {
        fn from(value: GetInitialPoolDataCall) -> Self {
            Self::GetInitialPoolData(value)
        }
    }
    impl ::core::convert::From<GetNextLiquidityCall> for GeometricMeanSolverCalls {
        fn from(value: GetNextLiquidityCall) -> Self {
            Self::GetNextLiquidity(value)
        }
    }
    impl ::core::convert::From<GetNextReserveXCall> for GeometricMeanSolverCalls {
        fn from(value: GetNextReserveXCall) -> Self {
            Self::GetNextReserveX(value)
        }
    }
    impl ::core::convert::From<GetNextReserveYCall> for GeometricMeanSolverCalls {
        fn from(value: GetNextReserveYCall) -> Self {
            Self::GetNextReserveY(value)
        }
    }
    impl ::core::convert::From<GetReservesAndLiquidityCall>
    for GeometricMeanSolverCalls {
        fn from(value: GetReservesAndLiquidityCall) -> Self {
            Self::GetReservesAndLiquidity(value)
        }
    }
    impl ::core::convert::From<InternalPriceCall> for GeometricMeanSolverCalls {
        fn from(value: InternalPriceCall) -> Self {
            Self::InternalPrice(value)
        }
    }
    impl ::core::convert::From<PrepareControllerUpdateCall>
    for GeometricMeanSolverCalls {
        fn from(value: PrepareControllerUpdateCall) -> Self {
            Self::PrepareControllerUpdate(value)
        }
    }
    impl ::core::convert::From<PrepareFeeUpdateCall> for GeometricMeanSolverCalls {
        fn from(value: PrepareFeeUpdateCall) -> Self {
            Self::PrepareFeeUpdate(value)
        }
    }
    impl ::core::convert::From<PrepareWeightXUpdateCall> for GeometricMeanSolverCalls {
        fn from(value: PrepareWeightXUpdateCall) -> Self {
            Self::PrepareWeightXUpdate(value)
        }
    }
    impl ::core::convert::From<SimulateSwapCall> for GeometricMeanSolverCalls {
        fn from(value: SimulateSwapCall) -> Self {
            Self::SimulateSwap(value)
        }
    }
    impl ::core::convert::From<StrategyCall> for GeometricMeanSolverCalls {
        fn from(value: StrategyCall) -> Self {
            Self::Strategy(value)
        }
    }
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
    ///Container type for all return fields from the `checkSwapConstant` function with signature `checkSwapConstant(uint256,bytes)` and selector `0x0f4166b8`
    #[derive(
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
    pub struct CheckSwapConstantReturn(pub ::ethers::core::types::I256);
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
    pub struct FetchPoolParamsReturn(pub GeometricMeanParams);
    ///Container type for all return fields from the `getInitialPoolData` function with signature `getInitialPoolData(uint256,uint256,(uint256,uint256,uint256,address))` and selector `0xdef15f92`
    #[derive(
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
        Hash
    )]
    pub struct GetNextLiquidityReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getNextReserveX` function with signature `getNextReserveX(uint256,uint256,uint256)` and selector `0x5a93b8ce`
    #[derive(
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
    ///Container type for all return fields from the `getNextReserveY` function with signature `getNextReserveY(uint256,uint256,uint256)` and selector `0xf2de7a7b`
    #[derive(
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
    pub struct PrepareFeeUpdateReturn {
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all return fields from the `prepareWeightXUpdate` function with signature `prepareWeightXUpdate(uint256,uint256)` and selector `0x250968d9`
    #[derive(
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
    pub struct PrepareWeightXUpdateReturn(pub ::ethers::core::types::Bytes);
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
    ///`GeometricMeanParams(uint256,uint256,uint256,address)`
    #[derive(
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
    pub struct GeometricMeanParams {
        pub w_x: ::ethers::core::types::U256,
        pub w_y: ::ethers::core::types::U256,
        pub swap_fee: ::ethers::core::types::U256,
        pub controller: ::ethers::core::types::Address,
    }
}
