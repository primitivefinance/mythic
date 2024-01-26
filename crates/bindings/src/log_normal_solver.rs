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
    pub use super::super::shared_types::*;
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
    const __BYTECODE: &[u8] = b"`\x80\x80`@R4a\0\xE6W`@Q`\x1Fa'\xD68\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x91`\x01`\x01`@\x1B\x03\x83\x11\x84\x84\x10\x17a\0\xD0W\x80\x84\x92` \x94`@R\x839\x81\x01\x03\x12a\0\x80WQ`\x01`\x01`\xA0\x1B\x03\x81\x16\x90\x81\x90\x03a\0{W`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x91\x90\x91\x17\x90U`@Qa&\xA2\x90\x81a\x014\x829\xF3[`\0\x80\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[bF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01Ra7\xB7`\xF1\x1B`d\x82\x01R`\x84\x90\xFD\xFE`\x80`@R`\x046\x10a\x08\x8BW`\x005`\xE0\x1C\x80c8\x8E\xCE\xE7\x14a\0\xFCW\x80c9(\xFF\x97\x14a\0\xF7W\x80c;M\x100\x14a\0\xF2W\x80cZ\x93\xB8\xCE\x14a\0\xEDW\x80cb7V\x9F\x14a\0\xE8W\x80cme\"\x99\x14a\0\xE3W\x80c\x7F\x17@\x9C\x14a\0\xDEW\x80c\x81\xB5\xFA\xC2\x14a\0\xD9W\x80c\xA8\xC6.v\x14a\0\xD4W\x80c\xAFNC\x7F\x14a\0\xCFW\x80c\xCE\x15;\xF4\x14a\0\xCAW\x80c\xEE>\x8C\xFB\x14a\0\xC5W\x80c\xF2\xDEz{\x14a\0\xC0W\x80c\xF3\r7\xF2\x14a\0\xBBWc\xF9\xC2\x82\x11\x03a\x08\x8BWa\x08oV[a\x08?V[a\x08&V[a\x07\xF5V[a\x07\xB9V[a\x07\x92V[a\x07iV[a\x07\x1EV[a\x06\xC6V[a\x06\xAAV[a\x06QV[a\x068V[a\x05\xD5V[a\x03QV[a\x02\xA7V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01Ra7\xB7`\xF1\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FABI decoding: struct data too sh`D\x82\x01Rb\x1B\xDC\x9D`\xEA\x1B`d\x82\x01R`\x84\x90\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\x80\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x02$W`@RV[a\x01\xF2V[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x02$W`@RV[`\0[\x83\x81\x10a\x02^WPP`\0\x91\x01RV[\x81\x81\x01Q\x83\x82\x01R` \x01a\x02NV[\x90` \x91a\x02\x87\x81Q\x80\x92\x81\x85R\x85\x80\x86\x01\x91\x01a\x02KV[`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[\x90` a\x02\xA4\x92\x81\x81R\x01\x90a\x02nV[\x90V[4a\x03\x18W`\xC06`\x03\x19\x01\x12a\x03\x13W`\x806`C\x19\x01\x12a\x03\x0EWa\x03\na\x02\xFE`@Qa\x02\xD6\x81a\x02\x08V[`D5\x81R`d5` \x82\x01R`\x845`@\x82\x01R`\xA45``\x82\x01R`$5`\x045a\r!V[`@Q\x91\x82\x91\x82a\x02\x93V[\x03\x90\xF3[a\x01\xA1V[a\x01QV[a\x01\x01V[\x80\x15\x15\x03a\x03'WV[`\0\x80\xFD[\x90\x92`\x80\x92a\x02\xA4\x95\x94\x15\x15\x83R` \x83\x01R`@\x82\x01R\x81``\x82\x01R\x01\x90a\x02nV[4a\x03\x18W``6`\x03\x19\x01\x12a\x03\x13Wa\x03\xC5`$5`\x045a\x03t\x82a\x03\x1DV[`D5a\x03\x7Fa$hV[\x90a\x03\x88a$hV[\x93a\x03\x92\x84a\x0B\xE9V[\x94\x91\x92\x90\x93` \x95\x86\x84\x01\x94`@\x96\x87\x86\x01\x92\x83R\x86R\x84Ra\x03\xB4\x88a\x0B\x05V[\x99\x8A\x91\x85Q\x90\x87Q\x90Q\x91\x8Ba\x1F\xEBV[\x92\x15a\x05^W\x92\x82a\x04\x0Ca\x04\x13\x93a\x04\x05a\x04\0a\x03\xF8a\x04 \x98a\x03\xF3``a\x04G\x9D\x9C\x01Q\x86a\x0E\xFEV[a\x0E\xFEV[\x86Q\x90a\x0E\xCEV[a\x17+V[\x93Qa\x179V[\x8ARa\x179V[\x80\x85\x89\x01R\x87Q\x87a\"\xBAV[\x90a\x04>a\x043\x86\x89\x01\x93\x80\x85Ra\x17+V[\x80\x84R\x82Q\x11a$\xEFV[Q\x90Q\x90a\x0E\xBCV[\x94[\x84Q\x90\x83\x86\x01Q\x93a\x04\x89\x84\x88\x01\x93a\x04{\x85Q\x87Q\x98\x89\x93\x86\x85\x01`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[\x03`\x1F\x19\x81\x01\x87R\x86a\x02)V[`\0Ta\x04\xAC\x90a\x04\xA0\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x80;\x15a\x05YW\x85`\xC0\x91a\x04\xD6\x98\x87Q\x80\x9A\x81\x94\x82\x93c2\x14\x89\x0F`\xE0\x1B\x84R`\x04\x84\x01a\x1F\xD4V[\x03\x91Z\xFA\x95\x86\x15a\x05TW`\0\x96a\x05\x13W[P\x90a\x05\x08\x92\x91a\x03\n\x96\x97Q\x92Q\x85\x83Q\x92\x84\x01Q\x93\x01Q\x93a%}V[\x90Q\x94\x85\x94\x85a\x03,V[a\x03\n\x96P\x90a\x05>a\x05\x08\x94\x93\x92`\xC0=\x81\x11a\x05MW[a\x056\x81\x83a\x02)V[\x81\x01\x90a%FV[PPPPP\x96P\x90\x91\x92a\x04\xE9V[P=a\x05,V[a\n\xA9V[a\x08\xEEV[\x82a\x05\xA6a\x05\xCF\x96a\x05\x9Aa\x05\xBB\x95a\x05\x93a\x04\0a\x05\x8Ba\x05\xC6\x9Aa\x03\xF3``a\x05\xB3\x9B\x01Q\x86a\x0E\xFEV[\x85Q\x90a\x0E\xCEV[\x92Qa\x179V[\x92\x8A\x8D\x01\x93\x84Ra\x179V[\x90\x81\x88\x8C\x01RQ\x89a \xA3V[\x80\x89Ra\x17+V[\x80\x88R\x82Q\x11a$\x98V[Q\x85Q\x90a\x0E\xBCV[\x94a\x04IV[4a\x03\x18W` 6`\x03\x19\x01\x12a\x03\x13W` a\x06\x16`\x045a\x06\0a\x05\xFA\x82a\x0B\x05V[\x91a\x0B\xE9V[\x90P\x82\x91\x92Q\x90`@\x86\x84\x01Q\x93\x01Q\x93a%}V[`@Q\x90\x81R\xF3[``\x90`\x03\x19\x01\x12a\x03\x13W`\x045\x90`$5\x90`D5\x90V[4a\x03\x18W` a\x06\x16a\x06K6a\x06\x1EV[\x91a \xA3V[4a\x03\x18W`@6`\x03\x19\x01\x12a\x03\x13Wa\x06\x8C`\x045a\x03\na\x06\x82a\x06w\x83a\x0B\xE9V[\x91\x90P`$5a\x1FqV[\x81\x81\x95\x92\x94a\"\xBAV[\x92`@Q\x93\x84\x93\x84`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[4a\x03\x18W`\x006`\x03\x19\x01\x12a\x03\x13W` `@Q`\x01\x81R\xF3[4a\x03\x18W`@6`\x03\x19\x01\x12a\x03\x13W`\x045a\x07\0a\x03\na\x06\xF6a\x06\xEC\x84a\x0B\xE9V[\x91P`$5a\x1F\x9EV[\x81\x81\x94\x92\x95a \xA3V[\x91`@Q\x93\x84\x93\x84`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[4a\x03\x18W` 6`\x03\x19\x01\x12a\x03\x13W`\x80a\x07<`\x045a\x0B\x05V[a\x07g`@Q\x80\x92``\x80\x91\x80Q\x84R` \x81\x01Q` \x85\x01R`@\x81\x01Q`@\x85\x01R\x01Q\x91\x01RV[\xF3[4a\x03\x18W`\x006`\x03\x19\x01\x12a\x03\x13W`\0T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x03\x18W`\x806`\x03\x19\x01\x12a\x03\x13W` a\x06\x16`d5`D5`$5`\x045a\x1F\xEBV[4a\x03\x18W` 6`\x03\x19\x01\x12a\x03\x13Wa\x03\na\x07\xD8`\x045a\x0B\xE9V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R\x90\x81\x90``\x82\x01\x90V[4a\x03\x18W`@6`\x03\x19\x01\x12a\x03\x13Wa\x06\x8C`\x045a\x03\na\x06\x82a\x08\x1B\x83a\x0B\xE9V[\x91\x90P`$5a\x1F\x9EV[4a\x03\x18W` a\x06\x16a\x0896a\x06\x1EV[\x91a\"\xBAV[4a\x03\x18W`@6`\x03\x19\x01\x12a\x03\x13W`\x045a\x07\0a\x03\na\x06\xF6a\x08e\x84a\x0B\xE9V[\x91P`$5a\x1FqV[4a\x03\x18W`\x006`\x03\x19\x01\x12a\x03\x13W` `@Q`Z\x81R\xF3[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01Rtnor receive functions`X\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01Rd code`\xD8\x1B`d\x82\x01R`\x84\x90\xFD[\x91\x90` \x92\x83\x81\x83\x03\x12a\x03\x13W\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x82\x82\x11a\nYW\x01\x82`\x1F\x82\x01\x12\x15a\n\0W\x80Q\x91\x82\x11a\x02$W`@Q\x92a\t\x8E`\x1F\x84\x01`\x1F\x19\x16\x87\x01\x85a\x02)V[\x82\x84R\x85\x83\x83\x01\x01\x11a\t\xABW\x84\x83\x94\x95a\x02\xA4\x94\x01\x91\x01a\x02KV[`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x86\x90R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x86\x90R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x87\x90R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[`@Q=`\0\x82>=\x90\xFD[\x91\x90\x82`\x80\x91\x03\x12a\x03\x0EW`@Qa\n\xCD\x81a\x02\x08V[``\x80\x82\x94\x80Q\x84R` \x81\x01Q` \x85\x01R`@\x81\x01Q`@\x85\x01R\x01Q\x91\x01RV[\x90`\x80\x82\x82\x03\x12a\x03\x13Wa\x02\xA4\x91a\n\xB5V[``\x90`@Qa\x0B\x14\x81a\x02\x08V[`\0\x80\x82R` \x82\x01\x81\x90R`@\x82\x01\x81\x90R\x92\x01\x82\x90R\x81Ta\x0BB\x90a\x04\xA0\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x80;\x15a\x05YW`@Qc\xDC\x17\x83U`\xE0\x1B\x81R`\x04\x81\x01\x92\x90\x92R\x82\x90\x82\x90`$\x90\x82\x90Z\xFA\x90\x81\x15a\x05TW\x82a\x02\xA4\x93\x92a\x0B\x8CW[PP` \x80\x82Q\x83\x01\x01\x91\x01a\n\xF1V[a\x0B\xA8\x92P=\x80\x91\x83>a\x0B\xA0\x81\x83a\x02)V[\x81\x01\x90a\tAV[8\x80a\x0B{V[\x90\x81` \x91\x03\x12a\x03\x13WQ`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x03'W\x90V[\x90\x81``\x91\x03\x12a\x03\x13W\x80Q\x91`@` \x83\x01Q\x92\x01Q\x90V[`\0T\x90\x91\x90a\x0C\x03\x90a\x04\xA0\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x80;\x15a\x05YW` `\x04\x91`@Q\x92\x83\x80\x92c+\xEE\x84\xF1`\xE2\x1B\x82RZ\xFA\x90\x81\x15a\x05TW`\0\x91a\x0C\xB1W[P`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x05YW`@Qc3\x85N\xFD`\xE2\x1B\x81R`\x04\x81\x01\x93\x90\x93R``\x90\x83\x90`$\x90\x82\x90Z\xFA\x90\x81\x15a\x05TW`\0\x80\x93\x81\x93a\x0C{W[P\x92\x91\x90V[\x91\x93PPa\x0C\xA0\x91P``=\x81\x11a\x0C\xAAW[a\x0C\x98\x81\x83a\x02)V[\x81\x01\x90a\x0B\xCEV[\x92\x90\x92\x918a\x0CuV[P=a\x0C\x8EV[a\x0C\xD2\x91P` =\x81\x11a\x0C\xD8W[a\x0C\xCA\x81\x83a\x02)V[\x81\x01\x90a\x0B\xAFV[8a\x0C1V[P=a\x0C\xC0V[a\r\x1F\x93``\x92\x96\x95\x93`\xE0\x83\x01\x97\x83R` \x83\x01R`@\x82\x01R\x01\x90``\x80\x91\x80Q\x84R` \x81\x01Q` \x85\x01R`@\x81\x01Q`@\x85\x01R\x01Q\x91\x01RV[V[\x82Q` \x84\x01\x90\x81Q\x93`@\x86\x01\x91\x82Qa\r<\x81\x88a\x1FHV[\x91a\rG\x90\x84a\x0E\xCEV[a\rP\x90a\x1C4V[\x90g\r\xE0\xB6\xB3\xA7d\0\0a\rd\x81\x99a\x1C4V[a\rm\x90a\x0F\xA9V[\x05a\rw\x90a\x13\x1DV[\x90a\r\x81\x91a\x14\xEEV[a\r\x8A\x90a\x15\x0FV[a\r\x93\x91a\x11qV[a\r\x9C\x90a\x0FRV[\x90a\r\xA6\x91a\x0F\xE9V[a\r\xAF\x90a\x10+V[a\r\xB8\x90a\x1F\x05V[\x85\x03\x85\x81\x11a\x0E\xA6Wa\r\xCB\x90\x85a\x0E\xCEV[\x94\x86Q\x93Q\x92Q\x91a\r\xDD\x83\x85a\x1FHV[\x94a\r\xE7\x91a\x0E\xCEV[a\r\xF0\x90a\x1C4V[\x92a\r\xFA\x90a\x1C4V[a\x0E\x03\x90a\x0F\xA9V[\x05a\x0E\r\x90a\x13\x1DV[\x90a\x0E\x17\x91a\x14\xEEV[a\x0E \x90a\x15\x0FV[a\x0E)\x91a\x11\xC9V[a\x0E2\x90a\x0FRV[\x90a\x0E<\x91a\x0F\xE9V[a\x0EE\x90a\x10+V[a\x0EN\x90a\x1F\x05V[\x82\x84Q\x90a\x0E[\x91a\x0E\xFEV[\x90a\x0Ee\x91a\x0E\xFEV[\x91\x83a\x0Es\x81\x83\x86\x86a\x19GV[\x91a\x0E\x7F\x92\x85\x85a\x17FV[\x92`@Q\x93\x84\x93` \x85\x01\x93a\x0E\x94\x94a\x0C\xDFV[\x03`\x1F\x19\x81\x01\x82Ra\x02\xA4\x90\x82a\x02)V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x91\x90\x82\x03\x91\x82\x11a\x0E\xC9WV[a\x0E\xA6V[\x90g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x81\x02\x92\x81\x84\x04\x14\x90\x15\x17\x81\x15\x15\x16\x15a\x03'W`\x01\x90`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x03'W`\x01g\r\xE0\xB6\xB3\xA7d\0\0`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[`\x96\x81\x02\x90`\x96\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x03'W`\x01`d`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[\x90g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x81\x02\x92\x81\x84\x05\x14\x90\x15\x17\x15a\x0E\xC9WV[`\0\x81\x90\x03\x91\x90`\x01`\xFF\x1B\x81\x14`\x01\x16a\x0E\xC9W`\0\x19\x83\x05\x03a\x0E\xC9WV[\x90c;\x9A\xCA\0\x91\x82\x81\x02\x92\x81\x84\x05\x14\x90\x15\x17\x15a\x0E\xC9WV[\x90g\x1B\xC1mgN\xC8\0\0\x91\x82\x81\x02\x92\x81\x84\x05\x14\x90\x15\x17\x15a\x0E\xC9WV[\x81\x81\x02\x92\x91`\0\x82\x12`\x01`\xFF\x1B\x82\x14\x16a\x0E\xC9W\x81\x84\x05\x14\x90\x15\x17\x15a\x0E\xC9WV[\x81\x15a\x10\x04W`\x01`\xFF\x1B\x81\x14`\0\x19\x83\x14\x16a\x0E\xC9W\x05\x90V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\x01`\xFF\x1B\x81\x14a\x0E\xC9W`\0\x03\x90V[a\x10_a\x10Za\x10Ug\x13\xA0K\xBD\xFD\xC9\xBE\x88a\x10Og\x1B\xC1mgN\xC8\0\0\x95a\x0FRV[\x05a\x10\x1AV[a\x11\xE2V[a\x0FRV[\x05\x90V[\x90\x81g\r\xE0\xB6\xB3\xA7d\0\0\x01\x91\x82\x12`\x01\x16a\x0E\xC9WV[\x90\x81g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19\x01\x91\x82\x12\x15`\x01\x16a\x0E\xC9WV[\x90\x81g\x14\xA8EL\x19\xE1\xAC\0\x01\x91\x82\x12`\x01\x16a\x0E\xC9WV[\x90\x81g\x0F\xC1\x0E\x01W\x82w\xFF\x19\x01\x91\x82\x12\x15`\x01\x16a\x0E\xC9WV[\x90\x81g\x03\xDE\xBD\x08;\x8C|\0\x01\x91\x82\x12`\x01\x16a\x0E\xC9WV[\x90\x81g\x02\x95\xD4\0\xEA2W\xFF\x19\x01\x91\x82\x12\x15`\x01\x16a\x0E\xC9WV[\x90\x81g\x01W\xD8\xB2\xEC\xC7\x08\0\x01\x91\x82\x12`\x01\x16a\x0E\xC9WV[\x90\x81g\x051\n\xA7\xD5!0\0\x01\x91\x82\x12`\x01\x16a\x0E\xC9WV[\x90\x81g\r\xE0\xCC=\x15a\0\0\x01\x91\x82\x12`\x01\x16a\x0E\xC9WV[\x90\x81g \x05\xFEO&\x8E\xA0\0\x01\x91\x82\x12`\x01\x16a\x0E\xC9WV[\x90\x81g\r\xC5R\x7Fd, \0\x01\x91\x82\x12`\x01\x16a\x0E\xC9WV[\x91\x90\x91`\0\x83\x82\x01\x93\x84\x12\x91\x12\x90\x80\x15\x82\x16\x91\x15\x16\x17a\x0E\xC9WV[\x90g\x11\x90\0\xAB\x10\x0F\xFB\xFF\x19\x82\x01\x91\x82\x13`\x01\x16a\x0E\xC9WV[\x90g\x1B\xC1mgN\xC8\0\0`\0\x83\x82\x03\x93\x12\x81\x84\x12\x81\x16\x91\x84\x13\x90\x15\x16\x17a\x0E\xC9WV[\x81\x81\x03\x92\x91`\0\x13\x80\x15\x82\x85\x13\x16\x91\x84\x12\x16\x17a\x0E\xC9WV[\x80\x15a\x13\x10WgV\x98\xEE\xF0fp\0\0\x81\x12\x15a\x13\nWgV\x98\xEE\xF0fo\xFF\xFF\x19\x81\x13\x15a\x12\xFDWa\x12\xEBa\x12\x15\x82a\x15SV[a\x12\xA9a\x12\xE6a\x124a\x12/a\x12*\x85a\x14\xA1V[a\x10cV[a\x16CV[\x92a\x12\xE1a\x12\xDCa\x12\xD7a\x12\xD0a\x12\xCAa\x12\xC5a\x12\xBFa\x12\xBAa\x12\xB4a\x12\xAF\x8Da\x12\xA9a\x12\xA4a\x12\x9Ea\x12\x99a\x12\x93a\x12\x8Ea\x12\x88a\x12\x83a\x12}a\x12x\x8Aa\x15\x80V[a\x10{V[\x89a\x16\"V[a\x10\x95V[\x87a\x16\"V[a\x10\xADV[\x85a\x16\"V[a\x10\xC7V[\x83a\x16\"V[a\x10\xDFV[\x90a\x16\"V[a\x10\xF9V[\x8Ca\x16\"V[a\x11\x11V[\x8Aa\x16\"V[a\x11)V[\x88a\x16\"V[\x93\x80a\x16\"V[a\x0FoV[a\x11\x8DV[a\x11qV[a\x13\x1DV[\x90`\0\x13\x15a\x02\xA4Wa\x02\xA4\x90a\x11\xA6V[Pg\x1B\xC1mgN\xC8\0\0\x90V[P`\0\x90V[Pg\r\xE0\xB6\xB3\xA7d\0\0\x90V[h\x02H\xCE6\xA7\x0C\xB2k>\x19\x81\x13\x15a\x13\nWh\x07U\xBFy\x8BJ\x1B\xF1\xE5\x81\x12\x15a\x14mWe\x03x-\xAC\xE9\xD9\x90`N\x1B\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x91``\x90`\x01`_\x1B\x84\x82\x84\x1B\x05\x01\x82\x1D\x93\x84\x02\x90\x03\x80l\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x81\x01\x02\x82\x1D\x90n\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dPn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x82n\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x81m\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x81m\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x81m\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x81l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x02\x8D\x1D\x01\x02\x8B\x1D\x01\x02\x89\x1D\x01\x02\x87\x1D\x01\x02\x85\x1D\x01\x93m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x93m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEA\x81\x01\x90\x84m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x91\x01\x01\x02\x90\x1D\x01\x02\x01\x05\x02\x90`\xC3\x03\x1C\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x90\xFD[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x03'Wg\x1B\xC1mgN\xC8\0\0\x90\x04\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17\x82\x15\x15\x16\x15a\x03'W\x04\x90V[\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x03'Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x04\x90V[g\x06\xF0[Y\xD3\xB2\0\0\x90\x80\x82\x02\x91\x82\x04\x14`\x01\x16\x15a\x03'Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x04\x90V[`2\x81\x02\x90`2\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x03'W`d\x90\x04\x90V[`\x01`\xFF\x1B\x81\x14a\x15nW`\0\x81\x12\x15a\x02\xA4W\x19`\x01\x01\x90V[`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x90\xFD[g\x02_\x0F\xE1\x05\xA3\x14\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x03'Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\t\xD0(\xCCo _\xFF\x19\x81\x81\x02\x91`\x01\x91\x83\x05\x14\x16\x15a\x03'Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x90\x80\x82\x02\x91\x82\x05\x14`\x01\x16\x15a\x03'Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\x1B\xC1mgN\xC7\xFF\xFF\x19\x81\x81\x02\x91`\x01\x91\x83\x05\x14\x16\x15a\x03'Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x03'Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[`\x01\x81\x15\x15\x16\x15a\x03'Wn\xC0\x97\xCE{\xC9\x07\x15\xB3K\x9F\x10\0\0\0\0\x05\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17\x82\x15\x15\x16\x15a\x03'W\x05\x90V[g\x13\xA0K\xBD\xFD\xC9\xBE\x88\x90\x80\x82\x02\x91\x82\x05\x14`\x01\x16\x15a\x03'Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x03'Wg\x1B\xC1mgN\xC8\0\0\x90\x05\x90V[g\x03\xC1f\\z\xAB \0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x03'Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[f\x9F2u$b\xA0\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x03'Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[\x90`\x01\x82\x01\x80\x92\x11a\x0E\xC9WV[\x91\x90\x82\x01\x80\x92\x11a\x0E\xC9WV[\x93\x92\x93`\0\x91a\x17W\x85Q\x82a\x14\xCCV[\x91`\x1E\x93\x84\x86\x12\x80a\x19\x18W[\x15a\x17sWPPPPPPP\x90V[\x85\x96\x97\x91\x92\x93\x94\x95\x12`\0\x14a\x18\xF7W\x92a\x17\xC1\x91\x90\x80\x82\x11\x15a\x18\xE6WPa\x17\xB3a\x17\x9E\x82a\x17+V[\x97a\x01\0\x97[`@Q\x95\x86\x94` \x86\x01a\x0C\xDFV[\x03`\x1F\x19\x81\x01\x83R\x82a\x02)V[\x84\x91\x80`\0\x96\x82\x81\x11a\x18\xC3Wa\x17\xD8\x81\x85a&?V[\x92a\x17\xE3\x81\x86a&?V[\x89a\x17\xEE\x82\x87a\x0F\xC6V[\x13a\x18\xA2WP\x90a\x18\x02\x91\x97\x96\x92\x97a\x0E\xBCV[\x94\x87\x91`\x01\x96\x87\x80[a\x18\x1DW[PPPPPPPPPP\x90V[\x15a\x18\x7FW[P\x86\x97\x98P`\0\x92a\x18>a\x188\x8B\x89a\x179V[`\x01\x1C\x90V[\x99a\x18I\x8B\x88a&?V[\x90\x85a\x18U\x88\x84a\x0F\xC6V[\x13a\x18sWPP\x89\x93[\x88a\x18j\x89\x87a\x0E\xBCV[\x92\x01\x94\x99a\x18\x0BV[\x8B\x98P\x90\x95P\x93a\x18_V[\x81\x10\x80a\x18\x99W[\x15a\x18\x92W\x88a\x18#V[\x80\x80a\x18\x10V[P\x81\x83\x10a\x18\x87V[`@Qc\x06\xF1\xBE]`\xE2\x1B\x81R`\x04\x81\x01\x86\x90R`$\x81\x01\x91\x90\x91R`D\x90\xFD[`@Qc0\x82\xDF\xDB`\xE1\x1B\x81R`\x04\x81\x01\x91\x90\x91R`$\x81\x01\x92\x90\x92RP`D\x90\xFD[a\x18\xF2a\x17\xB3\x91a\x17+V[a\x17\x9EV[\x92Pa\x17\xC1\x90a\x17\xB3k\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x94\x97a\x01\0\x97a\x17\xA4V[P`\x1D\x19\x86\x13a\x17dV[`\xE0\x81\x83\x03\x12a\x03\x13W\x80Q\x92a\x02\xA4` \x83\x01Q\x93```@\x85\x01Q\x94\x01a\n\xB5V[\x90\x92\x82\x82\x10\x15a\x19\xFCWa\x02\xA4\x93a\x12\xE1\x92\x84g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82a\x19o\x83\x83a\x14\xCCV[\x10a\x19\xE9WP`\x01`\x01`\xFF\x1B\x03\x95\x90P[\x83Q\x91a\x19\x97a\x19\x91\x83\x85a\x14\xEEV[\x85a\x14\xCCV[\x10a\x19\xC4WP`\x01`\x01`\xFF\x1B\x03\x92a\x19\xBE\x92P\x90P[`@` \x82\x01Q\x91\x01Q\x90a\x1FHV[\x92a\x11qV[a\x19\xBE\x92a\x19\xD8a\x19\xDE\x92a\x19\xE3\x94a\x14\xEEV[\x90a\x14\xCCV[a\x1AAV[\x91a\x19\xAEV[a\x19\xF6\x91a\x19\xDE\x91a\x14\xCCV[\x94a\x19\x81V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FtradingFunction: invalid x\0\0\0\0\0\0`D\x82\x01R`d\x90\xFD[g\x06\xF0[Y\xD3\xB2\0\0\x81\x14a\x13\nWg\r\xE0\xB6\xB3\xA7d\0\0\x80\x82\x12\x15a\x1B\xA6W\x81\x15a\x1B\xC7W`\x01\x82\x81\x1B\x91`\x02\x93\x83\x05\x84\x03a\x0E\xC9W`\0\x83\x12\x80\x15a\x1B\xEBW[a\x1B\xD9W\x82\x15a\x1B\xA6Wg\x1B\xC1mgN\xC8\0\0\x83\x14a\x1B\xC7W\x82\x12\x91\x82\x15a\x1B\xB8W\x92[a\x1A\xB0\x84a\x16\xABV[\x80\x15a\x1B\xA6Wa\x1B\x17a\x1A\xE1a\x1A\xDCa\x1A\xD7a\x1A\xD2a\x1B\x1C\x95\x99\x97\x96\x99a\x1C4V[a\x15\xFAV[a\x1D\xD5V[a\x0F\x90V[a\x1B\x12a\x1A\xF5a\x1A\xF0\x83a\x16\xD6V[a\x11AV[a\x1B\x0Ca\x12*a\x12\x93a\x1B\x07\x86a\x17\x01V[a\x11YV[\x90a\x16bV[a\x11\xC9V[a\x15\xABV[\x93`\0\x92[\x81\x84\x10a\x1BSWPPPPa\x02\xA4\x91a\x1B@\x91`\0\x14a\x1BEWa\x16\x84V[a\x10\x1AV[a\x1BN\x90a\x10\x1AV[a\x16\x84V[\x90\x91a\x1B\x9C\x86a\x1B\x96a\x1Bk\x85a\x1B\x12\x86\x99\x9Ba\x11\xE2V[a\x1B\x0Ca\x1B\x86a\x1B\x81a\x12\xE6a\x1B@\x87\x80a\x16\"V[a\x15\xD3V[a\x1B\x90\x83\x86a\x16\"V[\x90a\x11\xC9V[\x90a\x11qV[\x95\x01\x92\x91\x90a\x1B!V[`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x90\xFD[a\x1B\xC1\x90a\x11\xA6V[\x92a\x1A\xA7V[`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x90\xFD[`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x90\xFD[Pg\x1B\xC1mgN\xC8\0\0\x83\x13a\x1A\x83V[\x15a\x1C\x03WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x90\xFD[}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x90a\x1C``\0\x82\x13a\x1B\xFCV[q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06a\x1C|\x82a\x1E\x93V[``\x92\x83\x82`\x9F\x03\x01\x1B`\x9F\x1C\x90`_\x19\x01}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x02\x92l\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x82m\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x81m\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x81m\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x81m\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x81m\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x81lFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x02\x8C\x1D\x01\x02\x8A\x1D\x01\x02\x88\x1D\x01\x02\x86\x1D\x01\x02\x84\x1D\x01\x02\x82\x1D\x01\x91x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x91l\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x90\x82m\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x81l\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x81m\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x81m\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x81l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x81\x01\x02\x89\x1D\x01\x02\x87\x1D\x01\x02\x85\x1D\x01\x02\x83\x1D\x01\x02\x90\x1D\x01\x02\x01\x05\x02\x01\x01`\xAE\x1D\x90V[`\xB5\x81`\x01`\x88\x1B\x81\x10\x15a\x1E|W[\x80i\x01\0\0\0\0\0\0\0\0\0b\x01\0\0\x92\x10\x15a\x1EoW[e\x01\0\0\0\0\0\x81\x10\x15a\x1EbW[c\x01\0\0\0\x81\x10\x15a\x1EUW[\x01\x02`\x12\x1C`\x01\x90\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x80\x80\x92\x04\x10\x90\x03\x90V[`\x10\x1C\x91`\x08\x1B\x91a\x1E\x19V[` \x1C\x91`\x10\x1B\x91a\x1E\x0CV[`@\x1C\x91` \x1B\x91a\x1D\xFDV[Ph\xB5\0\0\0\0\0\0\0\0\x90P`\x80\x82\x90\x1Ca\x1D\xE5V[a\x1E\x9E\x81\x15\x15a\x1B\xFCV[\x80o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x07\x1B\x81\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x81\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x81\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x81\x81\x1C`\xFF\x10`\x03\x1B\x17\x81\x81\x1C`\x0F\x10`\x02\x1B\x17\x81\x81\x1C`\x03\x10`\x01\x1B\x17\x90\x81\x1C`\x01\x10\x17\x90V[`\0\x81\x12a\x1F\x10W\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01RotoUint: negative`\x80\x1B`D\x82\x01R`d\x90\xFD[\x90a\x1FR\x90a\x1D\xD5V[c;\x9A\xCA\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17\x15a\x0E\xC9Wa\x02\xA4\x91a\x14\xEEV[\x92\x91\x90a\x1F\x87a\x1F\x81\x82\x84a\x0E\xCEV[\x85a\x0E\xFEV[\x93\x81\x03\x90\x81\x11a\x0E\xC9W\x92\x81\x03\x90\x81\x11a\x0E\xC9W\x90V[\x92\x91\x90a\x1F\xAEa\x1F\x81\x82\x84a\x0E\xCEV[\x93\x81\x01\x80\x91\x11a\x0E\xC9W\x92\x81\x01\x80\x91\x11a\x0E\xC9W\x90V[\x90\x81` \x91\x03\x12a\x03\x13WQ\x90V[`@\x90a\x02\xA4\x93\x92\x81R\x81` \x82\x01R\x01\x90a\x02nV[`@\x80Q` \x81\x01\x84\x90R\x90\x81\x01\x84\x90R``\x81\x01\x85\x90R\x93\x91\x92a \x13\x85`\x80\x81\x01a\x04{V[`\0T`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x05YWa H\x95` \x91`@Q\x80\x80\x99\x81\x94b.RK`\xE0\x1B\x83R\x88`\x04\x84\x01a\x1F\xD4V[\x03\x91Z\xFA\x91\x82\x15a\x05TWa\x02\xA4\x95`\0\x93a oW[Pa i\x90a\x0B\x05V[\x93a\x17FV[a i\x91\x93Pa \x95\x90` =\x81\x11a \x9CW[a \x8D\x81\x83a\x02)V[\x81\x01\x90a\x1F\xC5V[\x92\x90a _V[P=a \x83V[a \xAC\x81a\x0B\xE9V[PP\x90`@Q\x93a \xD7\x85a\x04{\x83\x87\x87` \x85\x01`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[`\0Ta \xEC\x90`\x01`\x01`\xA0\x1B\x03\x16a\x04\xA0V[\x80;\x15a\x05YWa!\x15\x95` \x91`@Q\x80\x80\x99\x81\x94b.RK`\xE0\x1B\x83R\x88`\x04\x84\x01a\x1F\xD4V[\x03\x91Z\xFA\x91\x82\x15a\x05TWa\x02\xA4\x95`\0\x93a!<W[Pa!6\x90a\x0B\x05V[\x93a!`V[a!6\x91\x93Pa!Y\x90` =\x81\x11a \x9CWa \x8D\x81\x83a\x02)V[\x92\x90a!,V[a\x17\xB3a!\x8E\x92\x93\x94\x95`\0\x86\x12`\0\x14a\"\xAAWa!~\x87a\x0F*V[\x95`@Q\x95\x86\x94` \x86\x01a\x0C\xDFV[\x82\x82\x91`\0\x94\x84\x81\x11a\"\x89Wa!\xA5\x81\x83a&_V[\x94a!\xB0\x81\x84a&_V[\x87a!\xBB\x82\x89a\x0F\xC6V[\x13a\"hWP\x90a!\xCE\x91\x95\x94\x95a\x0E\xBCV[\x92\x85`\x01\x94\x85\x80[a!\xE6W[PPPPPPPP\x90V[\x15a\"BW[P\x84\x95\x96P`\0\x90a\"\x01a\x188\x89\x87a\x179V[\x97a\"\x0C\x89\x86a&_V[\x90\x83a\"\x18\x86\x84a\x0F\xC6V[\x13a\"6WPP\x87\x91[\x86a\"-\x87\x85a\x0E\xBCV[\x92\x01\x92\x97a!\xD6V[\x89\x96P\x90\x93P\x91a\"\"V[`\x1E\x10\x80a\"]W[\x15a\"VW\x86a!\xECV[\x80\x80a!\xDBV[Pa\x01\0\x81\x10a\"KV[`@Qc\x06\xF1\xBE]`\xE2\x1B\x81R`\x04\x81\x01\x88\x90R`$\x81\x01\x91\x90\x91R`D\x90\xFD[`@Qc0\x82\xDF\xDB`\xE1\x1B\x81R`\x04\x81\x01\x91\x90\x91R`$\x81\x01\x85\x90R`D\x90\xFD[a\"\xB3\x87a\x156V[\x96\x95a\x17\xA4V[a\"\xC3\x81a\x0B\xE9V[P\x91\x90P`@Q\x93a\"\xEF\x85a\x04{\x83\x86\x88` \x85\x01`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[`\0Ta#\x04\x90`\x01`\x01`\xA0\x1B\x03\x16a\x04\xA0V[\x80;\x15a\x05YWa#-\x95` \x91`@Q\x80\x80\x99\x81\x94b.RK`\xE0\x1B\x83R\x88`\x04\x84\x01a\x1F\xD4V[\x03\x91Z\xFA\x91\x82\x15a\x05TWa\x02\xA4\x95`\0\x93a#TW[Pa#N\x90a\x0B\x05V[\x93a#xV[a#N\x91\x93Pa#q\x90` =\x81\x11a \x9CWa \x8D\x81\x83a\x02)V[\x92\x90a#DV[a\x17\xB3a#\x96\x92\x93\x94\x95`\0\x86\x12`\0\x14a\"\xAAWa!~\x87a\x0F*V[\x82\x82\x91`\0\x94\x84\x81\x11a\"\x89Wa#\xAD\x81\x83a&\x81V[\x94a#\xB8\x81\x84a&\x81V[\x87a#\xC3\x82\x89a\x0F\xC6V[\x13a\"hWP\x90a#\xD6\x91\x95\x94\x95a\x0E\xBCV[\x92\x85`\x01\x94\x85\x80[a#\xEDWPPPPPPPP\x90V[\x15a$IW[P\x84\x95\x96P`\0\x90a$\x08a\x188\x89\x87a\x179V[\x97a$\x13\x89\x86a&\x81V[\x90\x83a$\x1F\x86\x84a\x0F\xC6V[\x13a$=WPP\x87\x91[\x86a$4\x87\x85a\x0E\xBCV[\x92\x01\x92\x97a#\xDEV[\x89\x96P\x90\x93P\x91a$)V[`\x1E\x10\x80a$]W[\x15a\"VW\x86a#\xF3V[Pa\x01\0\x81\x10a$RV[`@Q\x90``\x82\x01\x82\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x02$W`@R`\0`@\x83\x82\x81R\x82` \x82\x01R\x01RV[\x15a$\x9FWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: x reserve increase`D\x82\x01Rad!`\xF0\x1B`d\x82\x01R`\x84\x90\xFD[\x15a$\xF6WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: y reserve increase`D\x82\x01Rad!`\xF0\x1B`d\x82\x01R`\x84\x90\xFD[\x91\x90\x82`\xC0\x91\x03\x12a\x03\x13W\x81Qa%]\x81a\x03\x1DV[\x91` \x81\x01Q\x91`@\x82\x01Q\x91``\x81\x01Q\x91`\xA0`\x80\x83\x01Q\x92\x01Q\x90V[\x93a%\xB4\x92\x93\x91a%\xC9a%\xCF\x92a%\xBFa%\xC4\x82a%\xBFa%\x9F\x82\x8Aa\x1FHV[\x98a%\xB9g\r\xE0\xB6\xB3\xA7d\0\0\x9B\x8C\x92a\x1C4V[a\x0F\xA9V[\x05a\x13\x1DV[a\x14\xEEV[a\x15\x0FV[\x95a\x14\xCCV[\x90\x82\x82\x12\x15a&5W`\0\x82\x13\x15a&*W`\0\x82\x84\x03\x92\x12\x83\x83\x12\x81\x16\x90\x84\x84\x13\x90\x15\x16\x17a\x0E\xC9Wa\x02\xA4\x94a&$\x93a&\x19a\x12\xE6\x93a&\x14a&\x1F\x96a\x1AAV[a\x0F\xC6V[\x05a\x11\xC9V[a\x1F\x05V[\x90a\x0E\xFEV[PPPPP`\0\x19\x90V[PPPPP`\0\x90V[\x90a&Va\x02\xA4\x92` \x80\x82Q\x83\x01\x01\x91\x01a\x19#V[\x93\x92\x90Pa\x19GV[\x90a&va\x02\xA4\x92` \x80\x82Q\x83\x01\x01\x91\x01a\x19#V[\x93\x90P\x91\x90\x91a\x19GV[\x90a&\x98a\x02\xA4\x92` \x80\x82Q\x83\x01\x01\x91\x01a\x19#V[\x93\x91\x92\x90Pa\x19GV";
    /// The bytecode of the contract.
    pub static LOGNORMALSOLVER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\x08\x8BW`\x005`\xE0\x1C\x80c8\x8E\xCE\xE7\x14a\0\xFCW\x80c9(\xFF\x97\x14a\0\xF7W\x80c;M\x100\x14a\0\xF2W\x80cZ\x93\xB8\xCE\x14a\0\xEDW\x80cb7V\x9F\x14a\0\xE8W\x80cme\"\x99\x14a\0\xE3W\x80c\x7F\x17@\x9C\x14a\0\xDEW\x80c\x81\xB5\xFA\xC2\x14a\0\xD9W\x80c\xA8\xC6.v\x14a\0\xD4W\x80c\xAFNC\x7F\x14a\0\xCFW\x80c\xCE\x15;\xF4\x14a\0\xCAW\x80c\xEE>\x8C\xFB\x14a\0\xC5W\x80c\xF2\xDEz{\x14a\0\xC0W\x80c\xF3\r7\xF2\x14a\0\xBBWc\xF9\xC2\x82\x11\x03a\x08\x8BWa\x08oV[a\x08?V[a\x08&V[a\x07\xF5V[a\x07\xB9V[a\x07\x92V[a\x07iV[a\x07\x1EV[a\x06\xC6V[a\x06\xAAV[a\x06QV[a\x068V[a\x05\xD5V[a\x03QV[a\x02\xA7V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01Ra7\xB7`\xF1\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FABI decoding: struct data too sh`D\x82\x01Rb\x1B\xDC\x9D`\xEA\x1B`d\x82\x01R`\x84\x90\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\x80\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x02$W`@RV[a\x01\xF2V[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x02$W`@RV[`\0[\x83\x81\x10a\x02^WPP`\0\x91\x01RV[\x81\x81\x01Q\x83\x82\x01R` \x01a\x02NV[\x90` \x91a\x02\x87\x81Q\x80\x92\x81\x85R\x85\x80\x86\x01\x91\x01a\x02KV[`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[\x90` a\x02\xA4\x92\x81\x81R\x01\x90a\x02nV[\x90V[4a\x03\x18W`\xC06`\x03\x19\x01\x12a\x03\x13W`\x806`C\x19\x01\x12a\x03\x0EWa\x03\na\x02\xFE`@Qa\x02\xD6\x81a\x02\x08V[`D5\x81R`d5` \x82\x01R`\x845`@\x82\x01R`\xA45``\x82\x01R`$5`\x045a\r!V[`@Q\x91\x82\x91\x82a\x02\x93V[\x03\x90\xF3[a\x01\xA1V[a\x01QV[a\x01\x01V[\x80\x15\x15\x03a\x03'WV[`\0\x80\xFD[\x90\x92`\x80\x92a\x02\xA4\x95\x94\x15\x15\x83R` \x83\x01R`@\x82\x01R\x81``\x82\x01R\x01\x90a\x02nV[4a\x03\x18W``6`\x03\x19\x01\x12a\x03\x13Wa\x03\xC5`$5`\x045a\x03t\x82a\x03\x1DV[`D5a\x03\x7Fa$hV[\x90a\x03\x88a$hV[\x93a\x03\x92\x84a\x0B\xE9V[\x94\x91\x92\x90\x93` \x95\x86\x84\x01\x94`@\x96\x87\x86\x01\x92\x83R\x86R\x84Ra\x03\xB4\x88a\x0B\x05V[\x99\x8A\x91\x85Q\x90\x87Q\x90Q\x91\x8Ba\x1F\xEBV[\x92\x15a\x05^W\x92\x82a\x04\x0Ca\x04\x13\x93a\x04\x05a\x04\0a\x03\xF8a\x04 \x98a\x03\xF3``a\x04G\x9D\x9C\x01Q\x86a\x0E\xFEV[a\x0E\xFEV[\x86Q\x90a\x0E\xCEV[a\x17+V[\x93Qa\x179V[\x8ARa\x179V[\x80\x85\x89\x01R\x87Q\x87a\"\xBAV[\x90a\x04>a\x043\x86\x89\x01\x93\x80\x85Ra\x17+V[\x80\x84R\x82Q\x11a$\xEFV[Q\x90Q\x90a\x0E\xBCV[\x94[\x84Q\x90\x83\x86\x01Q\x93a\x04\x89\x84\x88\x01\x93a\x04{\x85Q\x87Q\x98\x89\x93\x86\x85\x01`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[\x03`\x1F\x19\x81\x01\x87R\x86a\x02)V[`\0Ta\x04\xAC\x90a\x04\xA0\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x80;\x15a\x05YW\x85`\xC0\x91a\x04\xD6\x98\x87Q\x80\x9A\x81\x94\x82\x93c2\x14\x89\x0F`\xE0\x1B\x84R`\x04\x84\x01a\x1F\xD4V[\x03\x91Z\xFA\x95\x86\x15a\x05TW`\0\x96a\x05\x13W[P\x90a\x05\x08\x92\x91a\x03\n\x96\x97Q\x92Q\x85\x83Q\x92\x84\x01Q\x93\x01Q\x93a%}V[\x90Q\x94\x85\x94\x85a\x03,V[a\x03\n\x96P\x90a\x05>a\x05\x08\x94\x93\x92`\xC0=\x81\x11a\x05MW[a\x056\x81\x83a\x02)V[\x81\x01\x90a%FV[PPPPP\x96P\x90\x91\x92a\x04\xE9V[P=a\x05,V[a\n\xA9V[a\x08\xEEV[\x82a\x05\xA6a\x05\xCF\x96a\x05\x9Aa\x05\xBB\x95a\x05\x93a\x04\0a\x05\x8Ba\x05\xC6\x9Aa\x03\xF3``a\x05\xB3\x9B\x01Q\x86a\x0E\xFEV[\x85Q\x90a\x0E\xCEV[\x92Qa\x179V[\x92\x8A\x8D\x01\x93\x84Ra\x179V[\x90\x81\x88\x8C\x01RQ\x89a \xA3V[\x80\x89Ra\x17+V[\x80\x88R\x82Q\x11a$\x98V[Q\x85Q\x90a\x0E\xBCV[\x94a\x04IV[4a\x03\x18W` 6`\x03\x19\x01\x12a\x03\x13W` a\x06\x16`\x045a\x06\0a\x05\xFA\x82a\x0B\x05V[\x91a\x0B\xE9V[\x90P\x82\x91\x92Q\x90`@\x86\x84\x01Q\x93\x01Q\x93a%}V[`@Q\x90\x81R\xF3[``\x90`\x03\x19\x01\x12a\x03\x13W`\x045\x90`$5\x90`D5\x90V[4a\x03\x18W` a\x06\x16a\x06K6a\x06\x1EV[\x91a \xA3V[4a\x03\x18W`@6`\x03\x19\x01\x12a\x03\x13Wa\x06\x8C`\x045a\x03\na\x06\x82a\x06w\x83a\x0B\xE9V[\x91\x90P`$5a\x1FqV[\x81\x81\x95\x92\x94a\"\xBAV[\x92`@Q\x93\x84\x93\x84`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[4a\x03\x18W`\x006`\x03\x19\x01\x12a\x03\x13W` `@Q`\x01\x81R\xF3[4a\x03\x18W`@6`\x03\x19\x01\x12a\x03\x13W`\x045a\x07\0a\x03\na\x06\xF6a\x06\xEC\x84a\x0B\xE9V[\x91P`$5a\x1F\x9EV[\x81\x81\x94\x92\x95a \xA3V[\x91`@Q\x93\x84\x93\x84`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[4a\x03\x18W` 6`\x03\x19\x01\x12a\x03\x13W`\x80a\x07<`\x045a\x0B\x05V[a\x07g`@Q\x80\x92``\x80\x91\x80Q\x84R` \x81\x01Q` \x85\x01R`@\x81\x01Q`@\x85\x01R\x01Q\x91\x01RV[\xF3[4a\x03\x18W`\x006`\x03\x19\x01\x12a\x03\x13W`\0T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x03\x18W`\x806`\x03\x19\x01\x12a\x03\x13W` a\x06\x16`d5`D5`$5`\x045a\x1F\xEBV[4a\x03\x18W` 6`\x03\x19\x01\x12a\x03\x13Wa\x03\na\x07\xD8`\x045a\x0B\xE9V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R\x90\x81\x90``\x82\x01\x90V[4a\x03\x18W`@6`\x03\x19\x01\x12a\x03\x13Wa\x06\x8C`\x045a\x03\na\x06\x82a\x08\x1B\x83a\x0B\xE9V[\x91\x90P`$5a\x1F\x9EV[4a\x03\x18W` a\x06\x16a\x0896a\x06\x1EV[\x91a\"\xBAV[4a\x03\x18W`@6`\x03\x19\x01\x12a\x03\x13W`\x045a\x07\0a\x03\na\x06\xF6a\x08e\x84a\x0B\xE9V[\x91P`$5a\x1FqV[4a\x03\x18W`\x006`\x03\x19\x01\x12a\x03\x13W` `@Q`Z\x81R\xF3[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01Rtnor receive functions`X\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01Rd code`\xD8\x1B`d\x82\x01R`\x84\x90\xFD[\x91\x90` \x92\x83\x81\x83\x03\x12a\x03\x13W\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x82\x82\x11a\nYW\x01\x82`\x1F\x82\x01\x12\x15a\n\0W\x80Q\x91\x82\x11a\x02$W`@Q\x92a\t\x8E`\x1F\x84\x01`\x1F\x19\x16\x87\x01\x85a\x02)V[\x82\x84R\x85\x83\x83\x01\x01\x11a\t\xABW\x84\x83\x94\x95a\x02\xA4\x94\x01\x91\x01a\x02KV[`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x86\x90R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x86\x90R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x87\x90R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[`@Q=`\0\x82>=\x90\xFD[\x91\x90\x82`\x80\x91\x03\x12a\x03\x0EW`@Qa\n\xCD\x81a\x02\x08V[``\x80\x82\x94\x80Q\x84R` \x81\x01Q` \x85\x01R`@\x81\x01Q`@\x85\x01R\x01Q\x91\x01RV[\x90`\x80\x82\x82\x03\x12a\x03\x13Wa\x02\xA4\x91a\n\xB5V[``\x90`@Qa\x0B\x14\x81a\x02\x08V[`\0\x80\x82R` \x82\x01\x81\x90R`@\x82\x01\x81\x90R\x92\x01\x82\x90R\x81Ta\x0BB\x90a\x04\xA0\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x80;\x15a\x05YW`@Qc\xDC\x17\x83U`\xE0\x1B\x81R`\x04\x81\x01\x92\x90\x92R\x82\x90\x82\x90`$\x90\x82\x90Z\xFA\x90\x81\x15a\x05TW\x82a\x02\xA4\x93\x92a\x0B\x8CW[PP` \x80\x82Q\x83\x01\x01\x91\x01a\n\xF1V[a\x0B\xA8\x92P=\x80\x91\x83>a\x0B\xA0\x81\x83a\x02)V[\x81\x01\x90a\tAV[8\x80a\x0B{V[\x90\x81` \x91\x03\x12a\x03\x13WQ`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x03'W\x90V[\x90\x81``\x91\x03\x12a\x03\x13W\x80Q\x91`@` \x83\x01Q\x92\x01Q\x90V[`\0T\x90\x91\x90a\x0C\x03\x90a\x04\xA0\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x80;\x15a\x05YW` `\x04\x91`@Q\x92\x83\x80\x92c+\xEE\x84\xF1`\xE2\x1B\x82RZ\xFA\x90\x81\x15a\x05TW`\0\x91a\x0C\xB1W[P`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x05YW`@Qc3\x85N\xFD`\xE2\x1B\x81R`\x04\x81\x01\x93\x90\x93R``\x90\x83\x90`$\x90\x82\x90Z\xFA\x90\x81\x15a\x05TW`\0\x80\x93\x81\x93a\x0C{W[P\x92\x91\x90V[\x91\x93PPa\x0C\xA0\x91P``=\x81\x11a\x0C\xAAW[a\x0C\x98\x81\x83a\x02)V[\x81\x01\x90a\x0B\xCEV[\x92\x90\x92\x918a\x0CuV[P=a\x0C\x8EV[a\x0C\xD2\x91P` =\x81\x11a\x0C\xD8W[a\x0C\xCA\x81\x83a\x02)V[\x81\x01\x90a\x0B\xAFV[8a\x0C1V[P=a\x0C\xC0V[a\r\x1F\x93``\x92\x96\x95\x93`\xE0\x83\x01\x97\x83R` \x83\x01R`@\x82\x01R\x01\x90``\x80\x91\x80Q\x84R` \x81\x01Q` \x85\x01R`@\x81\x01Q`@\x85\x01R\x01Q\x91\x01RV[V[\x82Q` \x84\x01\x90\x81Q\x93`@\x86\x01\x91\x82Qa\r<\x81\x88a\x1FHV[\x91a\rG\x90\x84a\x0E\xCEV[a\rP\x90a\x1C4V[\x90g\r\xE0\xB6\xB3\xA7d\0\0a\rd\x81\x99a\x1C4V[a\rm\x90a\x0F\xA9V[\x05a\rw\x90a\x13\x1DV[\x90a\r\x81\x91a\x14\xEEV[a\r\x8A\x90a\x15\x0FV[a\r\x93\x91a\x11qV[a\r\x9C\x90a\x0FRV[\x90a\r\xA6\x91a\x0F\xE9V[a\r\xAF\x90a\x10+V[a\r\xB8\x90a\x1F\x05V[\x85\x03\x85\x81\x11a\x0E\xA6Wa\r\xCB\x90\x85a\x0E\xCEV[\x94\x86Q\x93Q\x92Q\x91a\r\xDD\x83\x85a\x1FHV[\x94a\r\xE7\x91a\x0E\xCEV[a\r\xF0\x90a\x1C4V[\x92a\r\xFA\x90a\x1C4V[a\x0E\x03\x90a\x0F\xA9V[\x05a\x0E\r\x90a\x13\x1DV[\x90a\x0E\x17\x91a\x14\xEEV[a\x0E \x90a\x15\x0FV[a\x0E)\x91a\x11\xC9V[a\x0E2\x90a\x0FRV[\x90a\x0E<\x91a\x0F\xE9V[a\x0EE\x90a\x10+V[a\x0EN\x90a\x1F\x05V[\x82\x84Q\x90a\x0E[\x91a\x0E\xFEV[\x90a\x0Ee\x91a\x0E\xFEV[\x91\x83a\x0Es\x81\x83\x86\x86a\x19GV[\x91a\x0E\x7F\x92\x85\x85a\x17FV[\x92`@Q\x93\x84\x93` \x85\x01\x93a\x0E\x94\x94a\x0C\xDFV[\x03`\x1F\x19\x81\x01\x82Ra\x02\xA4\x90\x82a\x02)V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x91\x90\x82\x03\x91\x82\x11a\x0E\xC9WV[a\x0E\xA6V[\x90g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x81\x02\x92\x81\x84\x04\x14\x90\x15\x17\x81\x15\x15\x16\x15a\x03'W`\x01\x90`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x03'W`\x01g\r\xE0\xB6\xB3\xA7d\0\0`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[`\x96\x81\x02\x90`\x96\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x03'W`\x01`d`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[\x90g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x81\x02\x92\x81\x84\x05\x14\x90\x15\x17\x15a\x0E\xC9WV[`\0\x81\x90\x03\x91\x90`\x01`\xFF\x1B\x81\x14`\x01\x16a\x0E\xC9W`\0\x19\x83\x05\x03a\x0E\xC9WV[\x90c;\x9A\xCA\0\x91\x82\x81\x02\x92\x81\x84\x05\x14\x90\x15\x17\x15a\x0E\xC9WV[\x90g\x1B\xC1mgN\xC8\0\0\x91\x82\x81\x02\x92\x81\x84\x05\x14\x90\x15\x17\x15a\x0E\xC9WV[\x81\x81\x02\x92\x91`\0\x82\x12`\x01`\xFF\x1B\x82\x14\x16a\x0E\xC9W\x81\x84\x05\x14\x90\x15\x17\x15a\x0E\xC9WV[\x81\x15a\x10\x04W`\x01`\xFF\x1B\x81\x14`\0\x19\x83\x14\x16a\x0E\xC9W\x05\x90V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\x01`\xFF\x1B\x81\x14a\x0E\xC9W`\0\x03\x90V[a\x10_a\x10Za\x10Ug\x13\xA0K\xBD\xFD\xC9\xBE\x88a\x10Og\x1B\xC1mgN\xC8\0\0\x95a\x0FRV[\x05a\x10\x1AV[a\x11\xE2V[a\x0FRV[\x05\x90V[\x90\x81g\r\xE0\xB6\xB3\xA7d\0\0\x01\x91\x82\x12`\x01\x16a\x0E\xC9WV[\x90\x81g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19\x01\x91\x82\x12\x15`\x01\x16a\x0E\xC9WV[\x90\x81g\x14\xA8EL\x19\xE1\xAC\0\x01\x91\x82\x12`\x01\x16a\x0E\xC9WV[\x90\x81g\x0F\xC1\x0E\x01W\x82w\xFF\x19\x01\x91\x82\x12\x15`\x01\x16a\x0E\xC9WV[\x90\x81g\x03\xDE\xBD\x08;\x8C|\0\x01\x91\x82\x12`\x01\x16a\x0E\xC9WV[\x90\x81g\x02\x95\xD4\0\xEA2W\xFF\x19\x01\x91\x82\x12\x15`\x01\x16a\x0E\xC9WV[\x90\x81g\x01W\xD8\xB2\xEC\xC7\x08\0\x01\x91\x82\x12`\x01\x16a\x0E\xC9WV[\x90\x81g\x051\n\xA7\xD5!0\0\x01\x91\x82\x12`\x01\x16a\x0E\xC9WV[\x90\x81g\r\xE0\xCC=\x15a\0\0\x01\x91\x82\x12`\x01\x16a\x0E\xC9WV[\x90\x81g \x05\xFEO&\x8E\xA0\0\x01\x91\x82\x12`\x01\x16a\x0E\xC9WV[\x90\x81g\r\xC5R\x7Fd, \0\x01\x91\x82\x12`\x01\x16a\x0E\xC9WV[\x91\x90\x91`\0\x83\x82\x01\x93\x84\x12\x91\x12\x90\x80\x15\x82\x16\x91\x15\x16\x17a\x0E\xC9WV[\x90g\x11\x90\0\xAB\x10\x0F\xFB\xFF\x19\x82\x01\x91\x82\x13`\x01\x16a\x0E\xC9WV[\x90g\x1B\xC1mgN\xC8\0\0`\0\x83\x82\x03\x93\x12\x81\x84\x12\x81\x16\x91\x84\x13\x90\x15\x16\x17a\x0E\xC9WV[\x81\x81\x03\x92\x91`\0\x13\x80\x15\x82\x85\x13\x16\x91\x84\x12\x16\x17a\x0E\xC9WV[\x80\x15a\x13\x10WgV\x98\xEE\xF0fp\0\0\x81\x12\x15a\x13\nWgV\x98\xEE\xF0fo\xFF\xFF\x19\x81\x13\x15a\x12\xFDWa\x12\xEBa\x12\x15\x82a\x15SV[a\x12\xA9a\x12\xE6a\x124a\x12/a\x12*\x85a\x14\xA1V[a\x10cV[a\x16CV[\x92a\x12\xE1a\x12\xDCa\x12\xD7a\x12\xD0a\x12\xCAa\x12\xC5a\x12\xBFa\x12\xBAa\x12\xB4a\x12\xAF\x8Da\x12\xA9a\x12\xA4a\x12\x9Ea\x12\x99a\x12\x93a\x12\x8Ea\x12\x88a\x12\x83a\x12}a\x12x\x8Aa\x15\x80V[a\x10{V[\x89a\x16\"V[a\x10\x95V[\x87a\x16\"V[a\x10\xADV[\x85a\x16\"V[a\x10\xC7V[\x83a\x16\"V[a\x10\xDFV[\x90a\x16\"V[a\x10\xF9V[\x8Ca\x16\"V[a\x11\x11V[\x8Aa\x16\"V[a\x11)V[\x88a\x16\"V[\x93\x80a\x16\"V[a\x0FoV[a\x11\x8DV[a\x11qV[a\x13\x1DV[\x90`\0\x13\x15a\x02\xA4Wa\x02\xA4\x90a\x11\xA6V[Pg\x1B\xC1mgN\xC8\0\0\x90V[P`\0\x90V[Pg\r\xE0\xB6\xB3\xA7d\0\0\x90V[h\x02H\xCE6\xA7\x0C\xB2k>\x19\x81\x13\x15a\x13\nWh\x07U\xBFy\x8BJ\x1B\xF1\xE5\x81\x12\x15a\x14mWe\x03x-\xAC\xE9\xD9\x90`N\x1B\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x91``\x90`\x01`_\x1B\x84\x82\x84\x1B\x05\x01\x82\x1D\x93\x84\x02\x90\x03\x80l\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x81\x01\x02\x82\x1D\x90n\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dPn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x82n\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x81m\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x81m\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x81m\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x81l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x02\x8D\x1D\x01\x02\x8B\x1D\x01\x02\x89\x1D\x01\x02\x87\x1D\x01\x02\x85\x1D\x01\x93m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x93m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEA\x81\x01\x90\x84m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x91\x01\x01\x02\x90\x1D\x01\x02\x01\x05\x02\x90`\xC3\x03\x1C\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x90\xFD[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x03'Wg\x1B\xC1mgN\xC8\0\0\x90\x04\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17\x82\x15\x15\x16\x15a\x03'W\x04\x90V[\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x03'Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x04\x90V[g\x06\xF0[Y\xD3\xB2\0\0\x90\x80\x82\x02\x91\x82\x04\x14`\x01\x16\x15a\x03'Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x04\x90V[`2\x81\x02\x90`2\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x03'W`d\x90\x04\x90V[`\x01`\xFF\x1B\x81\x14a\x15nW`\0\x81\x12\x15a\x02\xA4W\x19`\x01\x01\x90V[`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x90\xFD[g\x02_\x0F\xE1\x05\xA3\x14\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x03'Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\t\xD0(\xCCo _\xFF\x19\x81\x81\x02\x91`\x01\x91\x83\x05\x14\x16\x15a\x03'Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x90\x80\x82\x02\x91\x82\x05\x14`\x01\x16\x15a\x03'Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\x1B\xC1mgN\xC7\xFF\xFF\x19\x81\x81\x02\x91`\x01\x91\x83\x05\x14\x16\x15a\x03'Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x03'Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[`\x01\x81\x15\x15\x16\x15a\x03'Wn\xC0\x97\xCE{\xC9\x07\x15\xB3K\x9F\x10\0\0\0\0\x05\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17\x82\x15\x15\x16\x15a\x03'W\x05\x90V[g\x13\xA0K\xBD\xFD\xC9\xBE\x88\x90\x80\x82\x02\x91\x82\x05\x14`\x01\x16\x15a\x03'Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x03'Wg\x1B\xC1mgN\xC8\0\0\x90\x05\x90V[g\x03\xC1f\\z\xAB \0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x03'Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[f\x9F2u$b\xA0\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x03'Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[\x90`\x01\x82\x01\x80\x92\x11a\x0E\xC9WV[\x91\x90\x82\x01\x80\x92\x11a\x0E\xC9WV[\x93\x92\x93`\0\x91a\x17W\x85Q\x82a\x14\xCCV[\x91`\x1E\x93\x84\x86\x12\x80a\x19\x18W[\x15a\x17sWPPPPPPP\x90V[\x85\x96\x97\x91\x92\x93\x94\x95\x12`\0\x14a\x18\xF7W\x92a\x17\xC1\x91\x90\x80\x82\x11\x15a\x18\xE6WPa\x17\xB3a\x17\x9E\x82a\x17+V[\x97a\x01\0\x97[`@Q\x95\x86\x94` \x86\x01a\x0C\xDFV[\x03`\x1F\x19\x81\x01\x83R\x82a\x02)V[\x84\x91\x80`\0\x96\x82\x81\x11a\x18\xC3Wa\x17\xD8\x81\x85a&?V[\x92a\x17\xE3\x81\x86a&?V[\x89a\x17\xEE\x82\x87a\x0F\xC6V[\x13a\x18\xA2WP\x90a\x18\x02\x91\x97\x96\x92\x97a\x0E\xBCV[\x94\x87\x91`\x01\x96\x87\x80[a\x18\x1DW[PPPPPPPPPP\x90V[\x15a\x18\x7FW[P\x86\x97\x98P`\0\x92a\x18>a\x188\x8B\x89a\x179V[`\x01\x1C\x90V[\x99a\x18I\x8B\x88a&?V[\x90\x85a\x18U\x88\x84a\x0F\xC6V[\x13a\x18sWPP\x89\x93[\x88a\x18j\x89\x87a\x0E\xBCV[\x92\x01\x94\x99a\x18\x0BV[\x8B\x98P\x90\x95P\x93a\x18_V[\x81\x10\x80a\x18\x99W[\x15a\x18\x92W\x88a\x18#V[\x80\x80a\x18\x10V[P\x81\x83\x10a\x18\x87V[`@Qc\x06\xF1\xBE]`\xE2\x1B\x81R`\x04\x81\x01\x86\x90R`$\x81\x01\x91\x90\x91R`D\x90\xFD[`@Qc0\x82\xDF\xDB`\xE1\x1B\x81R`\x04\x81\x01\x91\x90\x91R`$\x81\x01\x92\x90\x92RP`D\x90\xFD[a\x18\xF2a\x17\xB3\x91a\x17+V[a\x17\x9EV[\x92Pa\x17\xC1\x90a\x17\xB3k\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x94\x97a\x01\0\x97a\x17\xA4V[P`\x1D\x19\x86\x13a\x17dV[`\xE0\x81\x83\x03\x12a\x03\x13W\x80Q\x92a\x02\xA4` \x83\x01Q\x93```@\x85\x01Q\x94\x01a\n\xB5V[\x90\x92\x82\x82\x10\x15a\x19\xFCWa\x02\xA4\x93a\x12\xE1\x92\x84g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82a\x19o\x83\x83a\x14\xCCV[\x10a\x19\xE9WP`\x01`\x01`\xFF\x1B\x03\x95\x90P[\x83Q\x91a\x19\x97a\x19\x91\x83\x85a\x14\xEEV[\x85a\x14\xCCV[\x10a\x19\xC4WP`\x01`\x01`\xFF\x1B\x03\x92a\x19\xBE\x92P\x90P[`@` \x82\x01Q\x91\x01Q\x90a\x1FHV[\x92a\x11qV[a\x19\xBE\x92a\x19\xD8a\x19\xDE\x92a\x19\xE3\x94a\x14\xEEV[\x90a\x14\xCCV[a\x1AAV[\x91a\x19\xAEV[a\x19\xF6\x91a\x19\xDE\x91a\x14\xCCV[\x94a\x19\x81V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FtradingFunction: invalid x\0\0\0\0\0\0`D\x82\x01R`d\x90\xFD[g\x06\xF0[Y\xD3\xB2\0\0\x81\x14a\x13\nWg\r\xE0\xB6\xB3\xA7d\0\0\x80\x82\x12\x15a\x1B\xA6W\x81\x15a\x1B\xC7W`\x01\x82\x81\x1B\x91`\x02\x93\x83\x05\x84\x03a\x0E\xC9W`\0\x83\x12\x80\x15a\x1B\xEBW[a\x1B\xD9W\x82\x15a\x1B\xA6Wg\x1B\xC1mgN\xC8\0\0\x83\x14a\x1B\xC7W\x82\x12\x91\x82\x15a\x1B\xB8W\x92[a\x1A\xB0\x84a\x16\xABV[\x80\x15a\x1B\xA6Wa\x1B\x17a\x1A\xE1a\x1A\xDCa\x1A\xD7a\x1A\xD2a\x1B\x1C\x95\x99\x97\x96\x99a\x1C4V[a\x15\xFAV[a\x1D\xD5V[a\x0F\x90V[a\x1B\x12a\x1A\xF5a\x1A\xF0\x83a\x16\xD6V[a\x11AV[a\x1B\x0Ca\x12*a\x12\x93a\x1B\x07\x86a\x17\x01V[a\x11YV[\x90a\x16bV[a\x11\xC9V[a\x15\xABV[\x93`\0\x92[\x81\x84\x10a\x1BSWPPPPa\x02\xA4\x91a\x1B@\x91`\0\x14a\x1BEWa\x16\x84V[a\x10\x1AV[a\x1BN\x90a\x10\x1AV[a\x16\x84V[\x90\x91a\x1B\x9C\x86a\x1B\x96a\x1Bk\x85a\x1B\x12\x86\x99\x9Ba\x11\xE2V[a\x1B\x0Ca\x1B\x86a\x1B\x81a\x12\xE6a\x1B@\x87\x80a\x16\"V[a\x15\xD3V[a\x1B\x90\x83\x86a\x16\"V[\x90a\x11\xC9V[\x90a\x11qV[\x95\x01\x92\x91\x90a\x1B!V[`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x90\xFD[a\x1B\xC1\x90a\x11\xA6V[\x92a\x1A\xA7V[`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x90\xFD[`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x90\xFD[Pg\x1B\xC1mgN\xC8\0\0\x83\x13a\x1A\x83V[\x15a\x1C\x03WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x90\xFD[}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x90a\x1C``\0\x82\x13a\x1B\xFCV[q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06a\x1C|\x82a\x1E\x93V[``\x92\x83\x82`\x9F\x03\x01\x1B`\x9F\x1C\x90`_\x19\x01}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x02\x92l\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x82m\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x81m\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x81m\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x81m\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x81m\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x81lFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x02\x8C\x1D\x01\x02\x8A\x1D\x01\x02\x88\x1D\x01\x02\x86\x1D\x01\x02\x84\x1D\x01\x02\x82\x1D\x01\x91x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x91l\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x90\x82m\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x81l\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x81m\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x81m\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x81l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x81\x01\x02\x89\x1D\x01\x02\x87\x1D\x01\x02\x85\x1D\x01\x02\x83\x1D\x01\x02\x90\x1D\x01\x02\x01\x05\x02\x01\x01`\xAE\x1D\x90V[`\xB5\x81`\x01`\x88\x1B\x81\x10\x15a\x1E|W[\x80i\x01\0\0\0\0\0\0\0\0\0b\x01\0\0\x92\x10\x15a\x1EoW[e\x01\0\0\0\0\0\x81\x10\x15a\x1EbW[c\x01\0\0\0\x81\x10\x15a\x1EUW[\x01\x02`\x12\x1C`\x01\x90\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x80\x80\x92\x04\x10\x90\x03\x90V[`\x10\x1C\x91`\x08\x1B\x91a\x1E\x19V[` \x1C\x91`\x10\x1B\x91a\x1E\x0CV[`@\x1C\x91` \x1B\x91a\x1D\xFDV[Ph\xB5\0\0\0\0\0\0\0\0\x90P`\x80\x82\x90\x1Ca\x1D\xE5V[a\x1E\x9E\x81\x15\x15a\x1B\xFCV[\x80o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x07\x1B\x81\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x81\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x81\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x81\x81\x1C`\xFF\x10`\x03\x1B\x17\x81\x81\x1C`\x0F\x10`\x02\x1B\x17\x81\x81\x1C`\x03\x10`\x01\x1B\x17\x90\x81\x1C`\x01\x10\x17\x90V[`\0\x81\x12a\x1F\x10W\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01RotoUint: negative`\x80\x1B`D\x82\x01R`d\x90\xFD[\x90a\x1FR\x90a\x1D\xD5V[c;\x9A\xCA\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17\x15a\x0E\xC9Wa\x02\xA4\x91a\x14\xEEV[\x92\x91\x90a\x1F\x87a\x1F\x81\x82\x84a\x0E\xCEV[\x85a\x0E\xFEV[\x93\x81\x03\x90\x81\x11a\x0E\xC9W\x92\x81\x03\x90\x81\x11a\x0E\xC9W\x90V[\x92\x91\x90a\x1F\xAEa\x1F\x81\x82\x84a\x0E\xCEV[\x93\x81\x01\x80\x91\x11a\x0E\xC9W\x92\x81\x01\x80\x91\x11a\x0E\xC9W\x90V[\x90\x81` \x91\x03\x12a\x03\x13WQ\x90V[`@\x90a\x02\xA4\x93\x92\x81R\x81` \x82\x01R\x01\x90a\x02nV[`@\x80Q` \x81\x01\x84\x90R\x90\x81\x01\x84\x90R``\x81\x01\x85\x90R\x93\x91\x92a \x13\x85`\x80\x81\x01a\x04{V[`\0T`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x05YWa H\x95` \x91`@Q\x80\x80\x99\x81\x94b.RK`\xE0\x1B\x83R\x88`\x04\x84\x01a\x1F\xD4V[\x03\x91Z\xFA\x91\x82\x15a\x05TWa\x02\xA4\x95`\0\x93a oW[Pa i\x90a\x0B\x05V[\x93a\x17FV[a i\x91\x93Pa \x95\x90` =\x81\x11a \x9CW[a \x8D\x81\x83a\x02)V[\x81\x01\x90a\x1F\xC5V[\x92\x90a _V[P=a \x83V[a \xAC\x81a\x0B\xE9V[PP\x90`@Q\x93a \xD7\x85a\x04{\x83\x87\x87` \x85\x01`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[`\0Ta \xEC\x90`\x01`\x01`\xA0\x1B\x03\x16a\x04\xA0V[\x80;\x15a\x05YWa!\x15\x95` \x91`@Q\x80\x80\x99\x81\x94b.RK`\xE0\x1B\x83R\x88`\x04\x84\x01a\x1F\xD4V[\x03\x91Z\xFA\x91\x82\x15a\x05TWa\x02\xA4\x95`\0\x93a!<W[Pa!6\x90a\x0B\x05V[\x93a!`V[a!6\x91\x93Pa!Y\x90` =\x81\x11a \x9CWa \x8D\x81\x83a\x02)V[\x92\x90a!,V[a\x17\xB3a!\x8E\x92\x93\x94\x95`\0\x86\x12`\0\x14a\"\xAAWa!~\x87a\x0F*V[\x95`@Q\x95\x86\x94` \x86\x01a\x0C\xDFV[\x82\x82\x91`\0\x94\x84\x81\x11a\"\x89Wa!\xA5\x81\x83a&_V[\x94a!\xB0\x81\x84a&_V[\x87a!\xBB\x82\x89a\x0F\xC6V[\x13a\"hWP\x90a!\xCE\x91\x95\x94\x95a\x0E\xBCV[\x92\x85`\x01\x94\x85\x80[a!\xE6W[PPPPPPPP\x90V[\x15a\"BW[P\x84\x95\x96P`\0\x90a\"\x01a\x188\x89\x87a\x179V[\x97a\"\x0C\x89\x86a&_V[\x90\x83a\"\x18\x86\x84a\x0F\xC6V[\x13a\"6WPP\x87\x91[\x86a\"-\x87\x85a\x0E\xBCV[\x92\x01\x92\x97a!\xD6V[\x89\x96P\x90\x93P\x91a\"\"V[`\x1E\x10\x80a\"]W[\x15a\"VW\x86a!\xECV[\x80\x80a!\xDBV[Pa\x01\0\x81\x10a\"KV[`@Qc\x06\xF1\xBE]`\xE2\x1B\x81R`\x04\x81\x01\x88\x90R`$\x81\x01\x91\x90\x91R`D\x90\xFD[`@Qc0\x82\xDF\xDB`\xE1\x1B\x81R`\x04\x81\x01\x91\x90\x91R`$\x81\x01\x85\x90R`D\x90\xFD[a\"\xB3\x87a\x156V[\x96\x95a\x17\xA4V[a\"\xC3\x81a\x0B\xE9V[P\x91\x90P`@Q\x93a\"\xEF\x85a\x04{\x83\x86\x88` \x85\x01`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[`\0Ta#\x04\x90`\x01`\x01`\xA0\x1B\x03\x16a\x04\xA0V[\x80;\x15a\x05YWa#-\x95` \x91`@Q\x80\x80\x99\x81\x94b.RK`\xE0\x1B\x83R\x88`\x04\x84\x01a\x1F\xD4V[\x03\x91Z\xFA\x91\x82\x15a\x05TWa\x02\xA4\x95`\0\x93a#TW[Pa#N\x90a\x0B\x05V[\x93a#xV[a#N\x91\x93Pa#q\x90` =\x81\x11a \x9CWa \x8D\x81\x83a\x02)V[\x92\x90a#DV[a\x17\xB3a#\x96\x92\x93\x94\x95`\0\x86\x12`\0\x14a\"\xAAWa!~\x87a\x0F*V[\x82\x82\x91`\0\x94\x84\x81\x11a\"\x89Wa#\xAD\x81\x83a&\x81V[\x94a#\xB8\x81\x84a&\x81V[\x87a#\xC3\x82\x89a\x0F\xC6V[\x13a\"hWP\x90a#\xD6\x91\x95\x94\x95a\x0E\xBCV[\x92\x85`\x01\x94\x85\x80[a#\xEDWPPPPPPPP\x90V[\x15a$IW[P\x84\x95\x96P`\0\x90a$\x08a\x188\x89\x87a\x179V[\x97a$\x13\x89\x86a&\x81V[\x90\x83a$\x1F\x86\x84a\x0F\xC6V[\x13a$=WPP\x87\x91[\x86a$4\x87\x85a\x0E\xBCV[\x92\x01\x92\x97a#\xDEV[\x89\x96P\x90\x93P\x91a$)V[`\x1E\x10\x80a$]W[\x15a\"VW\x86a#\xF3V[Pa\x01\0\x81\x10a$RV[`@Q\x90``\x82\x01\x82\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x02$W`@R`\0`@\x83\x82\x81R\x82` \x82\x01R\x01RV[\x15a$\x9FWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: x reserve increase`D\x82\x01Rad!`\xF0\x1B`d\x82\x01R`\x84\x90\xFD[\x15a$\xF6WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: y reserve increase`D\x82\x01Rad!`\xF0\x1B`d\x82\x01R`\x84\x90\xFD[\x91\x90\x82`\xC0\x91\x03\x12a\x03\x13W\x81Qa%]\x81a\x03\x1DV[\x91` \x81\x01Q\x91`@\x82\x01Q\x91``\x81\x01Q\x91`\xA0`\x80\x83\x01Q\x92\x01Q\x90V[\x93a%\xB4\x92\x93\x91a%\xC9a%\xCF\x92a%\xBFa%\xC4\x82a%\xBFa%\x9F\x82\x8Aa\x1FHV[\x98a%\xB9g\r\xE0\xB6\xB3\xA7d\0\0\x9B\x8C\x92a\x1C4V[a\x0F\xA9V[\x05a\x13\x1DV[a\x14\xEEV[a\x15\x0FV[\x95a\x14\xCCV[\x90\x82\x82\x12\x15a&5W`\0\x82\x13\x15a&*W`\0\x82\x84\x03\x92\x12\x83\x83\x12\x81\x16\x90\x84\x84\x13\x90\x15\x16\x17a\x0E\xC9Wa\x02\xA4\x94a&$\x93a&\x19a\x12\xE6\x93a&\x14a&\x1F\x96a\x1AAV[a\x0F\xC6V[\x05a\x11\xC9V[a\x1F\x05V[\x90a\x0E\xFEV[PPPPP`\0\x19\x90V[PPPPP`\0\x90V[\x90a&Va\x02\xA4\x92` \x80\x82Q\x83\x01\x01\x91\x01a\x19#V[\x93\x92\x90Pa\x19GV[\x90a&va\x02\xA4\x92` \x80\x82Q\x83\x01\x01\x91\x01a\x19#V[\x93\x90P\x91\x90\x91a\x19GV[\x90a&\x98a\x02\xA4\x92` \x80\x82Q\x83\x01\x01\x91\x01a\x19#V[\x93\x91\x92\x90Pa\x19GV";
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
        ///Calls the contract's `getInitialPoolData` (0x388ecee7) function
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
                .method_hash([56, 142, 206, 231], (rx, s, params))
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
    ///Container type for all input parameters for the `getInitialPoolData` function with signature `getInitialPoolData(uint256,uint256,(uint256,uint256,uint256,uint256))` and selector `0x388ecee7`
    #[derive(
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
        abi = "getInitialPoolData(uint256,uint256,(uint256,uint256,uint256,uint256))"
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
        DeallocateGivenX(DeallocateGivenXCall),
        DeallocateGivenY(DeallocateGivenYCall),
        FetchPoolParams(FetchPoolParamsCall),
        GetInitialPoolData(GetInitialPoolDataCall),
        GetNextLiquidity(GetNextLiquidityCall),
        GetNextReserveX(GetNextReserveXCall),
        GetNextReserveY(GetNextReserveYCall),
        GetReservesAndLiquidity(GetReservesAndLiquidityCall),
        InternalPrice(InternalPriceCall),
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
    ///Container type for all return fields from the `getInitialPoolData` function with signature `getInitialPoolData(uint256,uint256,(uint256,uint256,uint256,uint256))` and selector `0x388ecee7`
    #[derive(
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
}
