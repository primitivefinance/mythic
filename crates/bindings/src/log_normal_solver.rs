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
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct LogNormParameters"),
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
                    ::std::borrow::ToOwned::to_owned("getPoolParams"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getPoolParams"),
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
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct LogNormParameters"),
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
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15b\0\0^W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`@Qb\0(\x908\x03\x80b\0(\x90\x839\x81\x01`@\x81\x90Rb\0\0\x81\x91b\0\0\xA7V[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90Ub\0\x01$V[`\0` \x82\x84\x03\x12\x15b\0\x01\x05W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x01\x1DW`\0\x80\xFD[\x93\x92PPPV[a'\\\x80b\0\x014`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x01BW`\x005`\xE0\x1C\x80c\xA8\xC6.v\x11a\0\xE4W\x80c\xEE>\x8C\xFB\x11a\0\xB3W\x80c\xEE>\x8C\xFB\x14a\x02\xE1W\x80c\xF2\xDEz{\x14a\x02\xF4W\x80c\xF3\r7\xF2\x14a\x03\x07W\x80c\xF9\xC2\x82\x11\x14a\x03\x1AWa\x01BV[\x80c\xA8\xC6.v\x14a\x02pW\x80c\xAFNC\x7F\x14a\x02\x9BW\x80c\xCE\x15;\xF4\x14a\x02\xAEW\x80c\xDC\x17\x83U\x14a\x02\xC1Wa\x01BV[\x80cZ\x93\xB8\xCE\x11a\x01 W\x80cZ\x93\xB8\xCE\x14a\x02\x14W\x80cb7V\x9F\x14a\x02'W\x80cme\"\x99\x14a\x02UW\x80c\x7F\x17@\x9C\x14a\x02]Wa\x01BV[\x80c\x1DZ\xC3\xC9\x14a\x01\xA7W\x80c9(\xFF\x97\x14a\x01\xD0W\x80c;M\x100\x14a\x01\xF3W[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x01\xBAa\x01\xB56`\x04a\"lV[a\x03\"V[`@Qa\x01\xC7\x91\x90a#\x1AV[`@Q\x80\x91\x03\x90\xF3[a\x01\xE3a\x01\xDE6`\x04a#>V[a\x037V[`@Qa\x01\xC7\x94\x93\x92\x91\x90a#yV[a\x02\x06a\x02\x016`\x04a#\xA0V[a\x07\xEFV[`@Q\x90\x81R` \x01a\x01\xC7V[a\x02\x06a\x02\"6`\x04a#\xBCV[a\x080V[a\x02:a\x0256`\x04a#\xEBV[a\tKV[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x01\xC7V[a\x02\x06`\x01\x81V[a\x02:a\x02k6`\x04a#\xEBV[a\t\x94V[`\0Ta\x02\x83\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\xC7V[a\x02\x06a\x02\xA96`\x04a$\x10V[a\t\xDCV[a\x02:a\x02\xBC6`\x04a#\xA0V[a\n\xDEV[a\x02\xD4a\x02\xCF6`\x04a#\xA0V[a\x0B\xAAV[`@Qa\x01\xC7\x91\x90a$EV[a\x02:a\x02\xEF6`\x04a#\xEBV[a\x0C\x8EV[a\x02\x06a\x03\x026`\x04a#\xBCV[a\x0C\xB4V[a\x02:a\x03\x156`\x04a#\xEBV[a\r\xC5V[a\x02\x06`Z\x81V[``a\x03/\x84\x84\x84a\r\xEBV[\x94\x93PPPPV[`\0\x80`\0``a\x03b`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[a\x03\x86`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[a\x03\x8F\x89a\n\xDEV[`@\x85\x01R` \x84\x01R\x82R`\0a\x03\xA6\x8Aa\x0B\xAAV[\x90P`\0\x80`\0\x80T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cT\xCF*\xEB`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x045W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a'<\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x04IW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04m\x91\x90a$fV[\x90P`\0a\x04\x89\x8D\x87`\0\x01Q\x88` \x01Q\x89`@\x01Qa\t\xDCV[\x90P\x8B\x15a\x05\xA0W`\0a\x04\x9D\x8C\x84a\x0E\\V[\x87Q\x90\x91P`\0\x90a\x04\xB9\x90a\x04\xB3\x84\x86a\x0E\\V[\x90a\x0ExV[\x90Pa\x04\xC6`\x01\x82a$\x98V[\x88Q\x90\x91Pa\x04\xD6\x90\x8E\x90a$\x98V[\x87R`@\x88\x01Qa\x04\xE8\x90\x82\x90a$\x98V[\x87`@\x01\x81\x81RPPa\x05\x04\x8F\x88`\0\x01Q\x89`@\x01Qa\x0C\xB4V[` \x88\x01\x81\x81R`\x01\x91a\x05\x19\x90\x83\x90a$\x98V[\x90RP` \x80\x89\x01Q\x90\x88\x01Q\x10a\x05\x83W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: y reserve increase`D\x82\x01Rad!`\xF0\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[\x86` \x01Q\x88` \x01Qa\x05\x97\x91\x90a$\xABV[\x94PPPa\x06\xA0V[`\0a\x05\xAC\x8C\x84a\x0E\\V[\x90P`\0a\x05\xCB\x88` \x01Qa\x04\xB3\x85\x85a\x0E\\\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa\x05\xD8`\x01\x82a$\x98V[\x90P\x8C\x88` \x01Qa\x05\xEA\x91\x90a$\x98V[` \x88\x01R`@\x88\x01Qa\x05\xFF\x90\x82\x90a$\x98V[\x87`@\x01\x81\x81RPPa\x06\x1B\x8F\x88` \x01Q\x89`@\x01Qa\x080V[\x80\x88R`\x01\x90\x88\x90a\x06.\x90\x83\x90a$\x98V[\x90RP\x87Q\x87Q\x10a\x06\x8DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: x reserve increase`D\x82\x01Rad!`\xF0\x1B`d\x82\x01R`\x84\x01a\x05zV[\x86Q\x88Qa\x06\x9B\x91\x90a$\xABV[\x94PPP[PP\x82Q` \x80\x85\x01Q`@\x80\x87\x01Q\x81Q\x93\x84\x01\x94\x90\x94R\x82\x01R``\x81\x01\x91\x90\x91R`\0\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c2\x14\x89\x0F\x8E\x84`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x07\x1F\x92\x91\x90a$\xBEV[`\xC0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x07uW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a'<\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x07\x89W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\xAD\x91\x90a$\xD7V[PPPPP\x90P\x80\x83a\x07\xD7\x87`\0\x01Q\x88`@\x01Q\x88`\0\x01Q\x89` \x01Q\x8A`@\x01Qa\x0E\x8DV[\x84\x99P\x99P\x99P\x99PPPPPPP\x93P\x93P\x93P\x93V[`\0\x80a\x07\xFB\x83a\x0B\xAAV[\x90P`\0\x80a\x08\t\x85a\n\xDEV[\x92PP\x91Pa\x08'\x82\x82\x85`\0\x01Q\x86` \x01Q\x87`@\x01Qa\x0E\x8DV[\x95\x94PPPPPV[`\0\x80a\x08<\x85a\n\xDEV[PP`@\x80Q` \x81\x01\x83\x90R\x80\x82\x01\x87\x90R``\x80\x82\x01\x87\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x80\x82\x01\x92\x83\x90R`\0\x80Tb.RK`\xE0\x1B\x90\x94R\x93\x94P\x92\x91`\x01`\x01`\xA0\x1B\x03\x16\x90b.RK\x90a\x08\x9B\x90\x8A\x90\x86\x90`\x84\x01a$\xBEV[` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x08\xF1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a'<\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\t\x05W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t)\x91\x90a$fV[\x90Pa\t@\x86\x86\x83\x86a\t;\x8Ca\x0B\xAAV[a\x0FxV[\x97\x96PPPPPPPV[`\0\x80`\0\x80`\0a\t\\\x87a\n\xDEV[\x92PP\x91P`\0\x80a\tq`\0\x89\x86\x86a\x0F\xF2V[\x91P\x91P`\0a\t\x82\x8A\x84\x84a\x0C\xB4V[\x92\x97P\x91\x95P\x93PPPP\x92P\x92P\x92V[`\0\x80`\0\x80`\0a\t\xA5\x87a\n\xDEV[\x92P\x92PP`\0\x80a\t\xBA`\x01\x89\x86\x86a\x0F\xF2V[\x91P\x91P`\0a\t\xCB\x8A\x84\x84a\x080V[\x97P\x91\x95P\x93PPPP\x92P\x92P\x92V[`@\x80Q` \x81\x01\x85\x90R\x80\x82\x01\x84\x90R``\x80\x82\x01\x84\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x80\x82\x01\x92\x83\x90R`\0\x80Tb.RK`\xE0\x1B\x90\x94R\x92\x90\x91\x83\x91`\x01`\x01`\xA0\x1B\x03\x16\x90b.RK\x90a\n9\x90\x8A\x90\x86\x90`\x84\x01a$\xBEV[` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\n\x8FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a'<\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\n\xA3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\xC7\x91\x90a$fV[\x90Pa\t@\x86\x86\x83\x87a\n\xD9\x8Ca\x0B\xAAV[a\x10[V[`\0\x80T`@Qc3\x85N\xFD`\xE2\x1B\x81R`\x04\x81\x01\x84\x90R\x82\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xCE\x15;\xF4\x90`$\x01```@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0BeW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a'<\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0ByW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\x9D\x91\x90a%-V[\x92P\x92P\x92P\x91\x93\x90\x92PV[a\x0B\xCE`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`\0T`@Qc\xECM\xC1[`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xECM\xC1[\x90`$\x01```@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0CPW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a'<\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0CdW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\x88\x91\x90a%\x9BV[\x92\x91PPV[`\0\x80`\0\x80`\0a\x0C\x9F\x87a\n\xDEV[\x92PP\x91P`\0\x80a\tq`\x01\x89\x86\x86a\x0F\xF2V[`\0\x80a\x0C\xC0\x85a\n\xDEV[P`@\x80Q` \x81\x01\x88\x90R\x80\x82\x01\x83\x90R``\x80\x82\x01\x88\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x80\x82\x01\x92\x83\x90R`\0\x80Tb.RK`\xE0\x1B\x90\x94R\x93\x95P\x93P`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90b.RK\x90a\r \x90\x8A\x90\x86\x90`\x84\x01a$\xBEV[` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\rvW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a'<\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\r\x8AW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\xAE\x91\x90a$fV[\x90Pa\t@\x86\x86\x83\x86a\r\xC0\x8Ca\x0B\xAAV[a\x11;V[`\0\x80`\0\x80`\0a\r\xD6\x87a\n\xDEV[\x92P\x92PP`\0\x80a\t\xBA`\0\x89\x86\x86a\x0F\xF2V[```\0a\r\xFA\x85\x85\x85a\x11\xA9V[\x90P`\0a\x0E\t\x82\x86\x86a\x11\xEEV[\x90P`\0a\x0E\x19\x87\x83\x85\x88a\x120V[\x90Pa\x0E(\x87\x83\x83\x86\x89a\x10[V[\x92P\x86\x82\x84\x87`@Q` \x01a\x0EA\x94\x93\x92\x91\x90a%\xBAV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x93PPPP\x93\x92PPPV[`\0a\x0Eq\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x131V[\x93\x92PPPV[`\0a\x0Eq\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x131V[`\0\x80a\x0E\x9A\x84\x84a\x13_V[\x90P`\0a\x0E\xA8\x85\x85a\x13\x85V[\x90P`\0a\x0E\xB6\x82\x86a\x13\xB4V[\x90P`\0a\x0E\xC4\x8A\x8Aa\x13\xC9V[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0\x81\x12a\x0E\xE2W`\0\x94PPPPPa\x08'V[`\0\x81\x13a\x0E\xF8W`\0\x19\x94PPPPPa\x08'V[`\0a\x0F\x14a\x0F\x0F\x83g\r\xE0\xB6\xB3\xA7d\0\0a%\xEEV[a\x13\xDEV[\x90P`\0\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x0F,\x88\x85a&\x15V[a\x0F6\x91\x90a&[V[a\x0F@\x91\x90a%\xEEV[\x90P`\0a\x0FM\x82a\x14{V[\x90P`\0a\x0FZ\x82a\x16$V[\x90Pa\x0Ff\x8C\x82a\x0E\\V[\x9E\x9DPPPPPPPPPPPPPPV[`\0\x80`\0\x80\x86\x12\x15a\x0F\x9BWP\x83a\x0F\x94\x81`\x96`da\x131V[\x91Pa\x0F\xAEV[a\x0F\xA8\x85`2`da\x16mV[\x90P\x84\x91P[a\x0F\xE6\x88\x88\x88\x87`@Q` \x01a\x0F\xC8\x94\x93\x92\x91\x90a%\xBAV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x82\x84`\na\x01\0a\x16\x8Ca\x16\xC3V[\x98\x97PPPPPPPPV[`\0\x80\x80a\x10\0\x84\x86a\x0ExV[\x90P`\0a\x10\x0E\x87\x83a\x0E\\V[\x90P\x87a\x10$Wa\x10\x1F\x87\x87a$\xABV[a\x10.V[a\x10.\x87\x87a$\x98V[\x93P\x87a\x10DWa\x10?\x81\x86a$\xABV[a\x10NV[a\x10N\x81\x86a$\x98V[\x92PPP\x94P\x94\x92PPPV[`\0\x80`\0\x80`\0a\x10z\x86`\0\x01Q\x8Aa\x13\xC9\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\n\x88\x12\x80\x15a\x10\x94WPa\x10\x91`\na&\x89V[\x88\x13[\x15a\x10\xA5W\x86\x94PPPPPa\x08'V[`\0\x88\x12\x15a\x10\xDFW\x86\x92P\x80\x8A\x11a\x10\xC8Wa\x10\xC3\x81`\x01a$\x98V[a\x10\xD3V[a\x10\xD3\x8A`\x01a$\x98V[\x93Pa\x01\0\x91Pa\x10\xF7V[k\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x92P\x86\x93Pa\x01\0\x91P[a\x11-\x8A\x8A\x8A\x89`@Q` \x01a\x11\x11\x94\x93\x92\x91\x90a%\xBAV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x85\x85`\n\x86a\x17\xD4a\x16\xC3V[\x9A\x99PPPPPPPPPPV[`\0\x80`\0\x80\x86\x12\x15a\x11^WP\x83a\x11W\x81`\x96`da\x131V[\x91Pa\x11qV[a\x11k\x85`2`da\x16mV[\x90P\x84\x91P[a\x0F\xE6\x88\x88\x88\x87`@Q` \x01a\x11\x8B\x94\x93\x92\x91\x90a%\xBAV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x82\x84`\na\x01\0a\x18\x01a\x16\xC3V[`\0\x80a\x11\xB6\x84\x84a\x18.V[\x90P`\0a\x11\xC3\x82a\x18\x9CV[\x90P`\0a\x11\xD0\x82a\x16$V[\x90Pa\t@a\x11\xE7\x82g\r\xE0\xB6\xB3\xA7d\0\0a$\xABV[\x88\x90a\x0ExV[`\0\x80a\x11\xFB\x84\x84a\x19\x05V[\x90P`\0a\x12\x08\x82a\x18\x9CV[\x90P`\0a\x12\x15\x82a\x16$V[\x85Q\x90\x91Pa\t@\x90\x82\x90a\x12*\x90\x8Aa\x0E\\V[\x90a\x0E\\V[`\0\x82\x85\x10a\x12\x81W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FtradingFunction: invalid x\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05zV[`\0\x80g\r\xE0\xB6\xB3\xA7d\0\0a\x12\x97\x88\x87a\x13\xC9V[\x10a\x12\xABW`\x01`\x01`\xFF\x1B\x03\x91Pa\x12\xBBV[a\x12\xB8a\x0F\x0F\x88\x87a\x13\xC9V[\x91P[g\r\xE0\xB6\xB3\xA7d\0\0a\x12\xDB\x87a\x12\xD6\x87`\0\x01Q\x89a\x13\xB4V[a\x13\xC9V[\x10a\x12\xEEWP`\x01`\x01`\xFF\x1B\x03a\x13\x06V[a\x13\x03a\x0F\x0F\x87a\x12\xD6\x87`\0\x01Q\x89a\x13\xB4V[\x90P[`\0a\x13\x1A\x85` \x01Q\x86`@\x01Qa\x13_V[\x90P\x80a\x13'\x83\x85a&\xA5V[a\x0F\xE6\x91\x90a&\xA5V[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x13IW`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[`\0\x80a\x13k\x83a\x19JV[a\x13y\x90c;\x9A\xCA\0a&\xCDV[\x90Pa\x03/\x84\x82a\x13\xB4V[`\0\x80a\x13\xA4\x83a\x13\x9E\x86g\x1B\xC1mgN\xC8\0\0a\x19\xEEV[\x90a\x13\xB4V[\x90Pa\x03/g\x06\xF0[Y\xD3\xB2\0\0\x82[`\0a\x0Eq\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x16mV[`\0a\x0Eq\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x16mV[`\0g\x06\xF0[Y\xD3\xB2\0\0\x82\x03a\x13\xF7WP`\0\x91\x90PV[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x12a\x14\x1FW`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\x14@W`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x14M\x83`\x02a&\x15V[\x90P`\0a\x14Z\x82a\x1A\x1FV[\x90P`\0a\x14pg\x13\xA0K\xBD\xFD\xC9\xBE\x88\x83a\x1C\x9DV[\x90Pa\x08'\x81a&\x89V[`\0h\x02H\xCE6\xA7\x0C\xB2k>\x19\x82\x13a\x14\x96WP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a\x14\xDDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x01a\x05zV[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05`\x01`_\x1B\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1Dm\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1Dn\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[`\0\x80\x82\x12\x15a\x16iW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01RotoUint: negative`\x80\x1B`D\x82\x01R`d\x01a\x05zV[P\x90V[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x16\x85W`\0\x80\xFD[\x04\x92\x91PPV[`\0\x80`\0\x80\x85\x80` \x01\x90Q\x81\x01\x90a\x16\xA6\x91\x90a&\xE4V[\x93PP\x92P\x92Pa\x16\xB9\x85\x84\x84\x84a\x120V[\x96\x95PPPPPPV[`\0\x84\x86\x11\x15a\x16\xF0W`@Qc0\x82\xDF\xDB`\xE1\x1B\x81R`\x04\x81\x01\x87\x90R`$\x81\x01\x86\x90R`D\x01a\x05zV[`\0a\x17\0\x88\x88\x85c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x17\x12\x89\x88\x86c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x17 \x82\x84a&\x15V[\x13\x15a\x17IW`@Qc\x06\xF1\xBE]`\xE2\x1B\x81R`\x04\x81\x01\x83\x90R`$\x81\x01\x82\x90R`D\x01a\x05zV[`\0a\x17U\x89\x89a$\xABV[\x90P`\0[`\x02a\x17f\x8A\x8Ca$\x98V[a\x17p\x91\x90a''V[\x94P`\0a\x17\x82\x8C\x87\x89c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x17\x90\x86\x83a&\x15V[\x13a\x17\x9DW\x85\x99Pa\x17\xA4V[\x85\x9AP\x80\x94P[a\x17\xAE\x8B\x8Ba$\xABV[\x92PP`\x01\x01\x87\x82\x11\x80\x15a\x17\xC2WP\x86\x81\x10[a\x17ZWPPPP\x96\x95PPPPPPV[`\0\x80`\0\x80\x85\x80` \x01\x90Q\x81\x01\x90a\x17\xEE\x91\x90a&\xE4V[\x93PP\x92P\x92Pa\x16\xB9\x83\x83\x87\x84a\x120V[`\0\x80`\0\x80\x85\x80` \x01\x90Q\x81\x01\x90a\x18\x1B\x91\x90a&\xE4V[\x93PP\x92P\x92Pa\x16\xB9\x83\x86\x84\x84a\x120V[\x80Q` \x82\x01Q`@\x83\x01Q`\0\x92\x91\x90\x83a\x18J\x83\x83a\x13_V[\x90P`\0a\x18X\x88\x86a\x1C\xB2V[\x90P`\0a\x18f\x85\x85a\x13\x85V[\x90P\x82a\x18s\x82\x84a&\xA5V[a\x18\x85\x90g\r\xE0\xB6\xB3\xA7d\0\0a&\x15V[a\x18\x8F\x91\x90a&[V[\x99\x98PPPPPPPPPV[`\0\x80g\x13\xA0K\xBD\xFD\xC9\xBE\x88a\x18\xBAg\r\xE0\xB6\xB3\xA7d\0\0\x85a&\x15V[a\x18\xC4\x91\x90a&[V[\x90P`\0a\x18\xD1\x82a&\x89V[\x90P`\0a\x18\xDE\x82a\x1C\xC6V[\x90Pg\x1B\xC1mgN\xC8\0\0a\x18\xFBg\r\xE0\xB6\xB3\xA7d\0\0\x83a&\x15V[a\x08'\x91\x90a&[V[\x80Q` \x82\x01Q`@\x83\x01Q`\0\x92\x91\x90\x83a\x19!\x83\x83a\x13_V[\x90P`\0a\x19/\x88\x86a\x1C\xB2V[\x90P`\0a\x19=\x85\x85a\x13\x85V[\x90P\x82a\x18s\x82\x84a%\xEEV[`\xB5\x81`\x01`\x88\x1B\x81\x10a\x19cW`@\x91\x90\x91\x1B\x90`\x80\x1C[i\x01\0\0\0\0\0\0\0\0\0\x81\x10a\x19\x7FW` \x91\x90\x91\x1B\x90`@\x1C[e\x01\0\0\0\0\0\x81\x10a\x19\x97W`\x10\x91\x90\x91\x1B\x90` \x1C[c\x01\0\0\0\x81\x10a\x19\xADW`\x08\x91\x90\x91\x1B\x90`\x10\x1C[b\x01\0\0\x01\x02`\x12\x1C\x80\x82\x04\x01`\x01\x90\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x90\x81\x90\x04\x81\x11\x90\x03\x90V[`\0a\x0Eqg\r\xE0\xB6\xB3\xA7d\0\0\x83a\x1A\x06\x86a\x1E\xAAV[a\x1A\x10\x91\x90a&\x15V[a\x1A\x1A\x91\x90a&[V[a\x14{V[`\0\x80\x82\x12\x80a\x1A6WPg\x1B\xC1mgN\xC8\0\0\x82\x13[\x15a\x1ATW`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\x1AuW`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81g\x1B\xC1mgN\xC8\0\0\x03a\x1A\x9DW`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x15a\x1A\xA8W\x91\x90PV[`\0g\r\xE0\xB6\xB3\xA7d\0\0\x83\x12a\x1A\xD0Wa\x1A\xCB\x83g\x1B\xC1mgN\xC8\0\0a%\xEEV[a\x1A\xD2V[\x82[\x90P`\0a\x1A\xE8\x82g\x1B\xC1mgN\xC8\0\0a \x85V[\x90P\x80`\0\x03a\x1B\x0BW`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x1B\x16\x82a\x1E\xAAV[\x90P`\0c;\x9A\xCA\0a\x1BAa\x1B<a\x1B6g\x1B\xC1mgN\xC8\0\0a&\x89V[\x85a\x1C\x9DV[a\x19JV[a\x1BK\x91\x90a&\x15V[\x90P`\0\x80a\x1Bb\x83g\x03\xC1f\\z\xAB \0a\x1C\x9DV[a\x1Bt\x90g \x05\xFEO&\x8E\xA0\0a&\xA5V[\x90P`\0a\x1B\xA4\x84a\x1B\x8D\x86f\x9F2u$b\xA0\0a\x1C\x9DV[a\x1B\x9F\x90g\r\xC5R\x7Fd, \0a&\xA5V[a\x1C\x9DV[a\x1B\xB6\x90g\r\xE0\xB6\xB3\xA7d\0\0a&\xA5V[\x90Pa\x1B\xDAg\t\xD0(\xCCo _\xFF\x19\x85a\x1B\xD0\x85\x85a \x85V[a\x1B\x9F\x91\x90a%\xEEV[\x92PPP`\0[`\x02\x81\x10\x15a\x1CuW`\0\x86a\x1B\xF6\x84a\x1C\xC6V[a\x1C\0\x91\x90a%\xEEV[\x90P`\0a\x1C\x0E\x84\x85a\x1C\x9DV[a\x1C\x17\x90a&\x89V[\x90P`\0a\x1C$\x82a\x14{V[\x90P`\0a\x1C2\x86\x85a\x1C\x9DV[a\x1CDg\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x84a\x1C\x9DV[a\x1CN\x91\x90a%\xEEV[\x90Pa\x1CZ\x84\x82a \x85V[a\x1Cd\x90\x87a&\xA5V[\x95P\x84`\x01\x01\x94PPPPPa\x1B\xE1V[g\r\xE0\xB6\xB3\xA7d\0\0\x88\x12a\x1C\x92Wa\x1C\x8D\x82a&\x89V[a\x0F\xE6V[P\x96\x95PPPPPPV[`\0a\x0Eq\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a \x96V[`\0a\x0Eqa\x1C\xC1\x84\x84a\x0ExV[a\x1E\xAAV[`\0\x81`\0\x03a\x1C\xDFWPg\r\xE0\xB6\xB3\xA7d\0\0\x91\x90PV[gV\x98\xEE\xF0fp\0\0\x82\x12a\x1C\xF6WP`\0\x91\x90PV[a\x1D\x07gV\x98\xEE\xF0fp\0\0a&\x89V[\x82\x13a\x1D\x1CWPg\x1B\xC1mgN\xC8\0\0\x91\x90PV[`\0a\x1D'\x83a \xB5V[\x90P`\0a\x1D`g\r\xE0\xB6\xB3\xA7d\0\0a\x1DI\x84g\x1B\xC1mgN\xC8\0\0a\x13\xC9V[a\x1D[\x90g\r\xE0\xB6\xB3\xA7d\0\0a&\xA5V[a \x85V[\x90P`\0\x80\x82a\x1D\xBC\x81a\x1D\xA9\x81a\x1D\x97\x81a\x1D\x84\x81g\x02_\x0F\xE1\x05\xA3\x14\0a\x1C\x9DV[a\x1B\x9F\x90g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19a&\xA5V[a\x1B\x9F\x90g\x14\xA8EL\x19\xE1\xAC\0a&\xA5V[a\x1B\x9F\x90g\x0F\xC1\x0E\x01W\x82w\xFF\x19a&\xA5V[a\x1D\xCE\x90g\x03\xDE\xBD\x08;\x8C|\0a&\xA5V[\x91P\x83\x90Pa\x1E6\x81a\x1E$\x81a\x1E\x12\x81a\x1E\0\x81a\x1D\xED\x81\x8Ba\x1C\x9DV[a\x1B\x9F\x90g\x02\x95\xD4\0\xEA2W\xFF\x19a&\xA5V[a\x1B\x9F\x90g\x01W\xD8\xB2\xEC\xC7\x08\0a&\xA5V[a\x1B\x9F\x90g\x051\n\xA7\xD5!0\0a&\xA5V[a\x1B\x9F\x90g\r\xE0\xCC=\x15a\0\0a&\xA5V[\x91P\x81g\x11\x90\0\xAB\x10\x0F\xFC\0a\x1EL\x87\x88a\x1C\x9DV[a\x1EX\x90`\0\x19a&\x15V[a\x1Eb\x91\x90a%\xEEV[a\x1El\x91\x90a&\xA5V[\x92PP`\0a\x1Ez\x83a\x14{V[\x90P`\0a\x1E\x88\x85\x83a\x1C\x9DV[\x90P`\0\x88\x12a\x1E\x98W\x80a\x0F\xE6V[a\x0F\xE6\x81g\x1B\xC1mgN\xC8\0\0a%\xEEV[`\0\x80\x82\x13a\x1E\xE7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x05zV[`\0``a\x1E\xF4\x84a \xECV[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[`\0a\x0Eq\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x05\x85\x14\x17\x16a \xAEW`\0\x80\xFD[\x05\x92\x91PPV[`\0`\x01`\xFF\x1B\x82\x03a \xDBW`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x12\x15a\x16iWP\x19`\x01\x01\x90V[`\0\x80\x82\x11a!)W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x05zV[P`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x82\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FABI decoding: struct data too sh`D\x82\x01Rb\x1B\xDC\x9D`\xEA\x1B`d\x82\x01R`\x84\x81\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\"fWcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@R\x90V[`\0\x80`\0\x83\x85\x03`\xA0\x81\x12\x15a\"\x85Wa\"\x85a!\x94V[\x845\x93P` \x85\x015\x92P```?\x19\x82\x01\x12\x15a\"\xA5Wa\"\xA5a!\xE4V[Pa\"\xAEa\"5V[`@\x85\x015\x81R``\x85\x015` \x82\x01R`\x80\x85\x015`@\x82\x01R\x80\x91PP\x92P\x92P\x92V[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a\"\xFAW` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\"\xDEV[P`\0` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R`\0a\x0Eq` \x83\x01\x84a\"\xD4V[\x80\x15\x15\x81\x14a#;W`\0\x80\xFD[PV[`\0\x80`\0``\x84\x86\x03\x12\x15a#VWa#Va!\x94V[\x835\x92P` \x84\x015a#h\x81a#-V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[\x84\x15\x15\x81R\x83` \x82\x01R\x82`@\x82\x01R`\x80``\x82\x01R`\0a\x16\xB9`\x80\x83\x01\x84a\"\xD4V[`\0` \x82\x84\x03\x12\x15a#\xB5Wa#\xB5a!\x94V[P5\x91\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a#\xD4Wa#\xD4a!\x94V[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a$\x01Wa$\x01a!\x94V[PP\x805\x92` \x90\x91\x015\x91PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a$)Wa$)a!\x94V[PP\x825\x94` \x84\x015\x94P`@\x84\x015\x93``\x015\x92P\x90PV[\x81Q\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x80\x83\x01Q\x90\x82\x01R``\x81\x01a\x0C\x88V[`\0` \x82\x84\x03\x12\x15a${Wa${a!\x94V[PQ\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x0C\x88Wa\x0C\x88a$\x82V[\x81\x81\x03\x81\x81\x11\x15a\x0C\x88Wa\x0C\x88a$\x82V[\x82\x81R`@` \x82\x01R`\0a\x03/`@\x83\x01\x84a\"\xD4V[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a$\xF3Wa$\xF3a!\x94V[\x86Qa$\xFE\x81a#-V[` \x88\x01Q`@\x89\x01Q``\x8A\x01Q`\x80\x8B\x01Q`\xA0\x90\x9B\x01Q\x93\x9C\x92\x9BP\x90\x99\x90\x98P\x96P\x90\x94P\x92PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a%EWa%Ea!\x94V[\x83Q\x92P` \x84\x01Q\x91P`@\x84\x01Q\x90P\x92P\x92P\x92V[`\0``\x82\x84\x03\x12\x15a%sWa%sa!\xE4V[a%{a\"5V[\x90P\x81Q\x81R` \x82\x01Q` \x82\x01R`@\x82\x01Q`@\x82\x01R\x92\x91PPV[`\0``\x82\x84\x03\x12\x15a%\xB0Wa%\xB0a!\x94V[a\x0Eq\x83\x83a%^V[\x84\x81R` \x80\x82\x01\x85\x90R`@\x80\x83\x01\x85\x90R\x83Q``\x84\x01R\x90\x83\x01Q`\x80\x83\x01R\x82\x01Q`\xA0\x82\x01R`\xC0\x81\x01a\x08'V[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a&\x0EWa&\x0Ea$\x82V[P\x92\x91PPV[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15a&1Wa&1a$\x82V[\x81\x81\x05\x83\x14\x82\x15\x17a\x0C\x88Wa\x0C\x88a$\x82V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a&jWa&ja&EV[`\x01`\xFF\x1B\x82\x14`\0\x19\x84\x14\x16\x15a&\x84Wa&\x84a$\x82V[P\x05\x90V[`\0`\x01`\xFF\x1B\x82\x01a&\x9EWa&\x9Ea$\x82V[P`\0\x03\x90V[\x80\x82\x01\x82\x81\x12`\0\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a&\xC5Wa&\xC5a$\x82V[PP\x92\x91PPV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x0C\x88Wa\x0C\x88a$\x82V[`\0\x80`\0\x80`\xC0\x85\x87\x03\x12\x15a&\xFDWa&\xFDa!\x94V[\x84Q\x93P` \x85\x01Q\x92P`@\x85\x01Q\x91Pa'\x1C\x86``\x87\x01a%^V[\x90P\x92\x95\x91\x94P\x92PV[`\0\x82a'6Wa'6a&EV[P\x04\x90V\xFETarget contract does not contain";
    /// The bytecode of the contract.
    pub static LOGNORMALSOLVER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x01BW`\x005`\xE0\x1C\x80c\xA8\xC6.v\x11a\0\xE4W\x80c\xEE>\x8C\xFB\x11a\0\xB3W\x80c\xEE>\x8C\xFB\x14a\x02\xE1W\x80c\xF2\xDEz{\x14a\x02\xF4W\x80c\xF3\r7\xF2\x14a\x03\x07W\x80c\xF9\xC2\x82\x11\x14a\x03\x1AWa\x01BV[\x80c\xA8\xC6.v\x14a\x02pW\x80c\xAFNC\x7F\x14a\x02\x9BW\x80c\xCE\x15;\xF4\x14a\x02\xAEW\x80c\xDC\x17\x83U\x14a\x02\xC1Wa\x01BV[\x80cZ\x93\xB8\xCE\x11a\x01 W\x80cZ\x93\xB8\xCE\x14a\x02\x14W\x80cb7V\x9F\x14a\x02'W\x80cme\"\x99\x14a\x02UW\x80c\x7F\x17@\x9C\x14a\x02]Wa\x01BV[\x80c\x1DZ\xC3\xC9\x14a\x01\xA7W\x80c9(\xFF\x97\x14a\x01\xD0W\x80c;M\x100\x14a\x01\xF3W[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x01\xBAa\x01\xB56`\x04a\"lV[a\x03\"V[`@Qa\x01\xC7\x91\x90a#\x1AV[`@Q\x80\x91\x03\x90\xF3[a\x01\xE3a\x01\xDE6`\x04a#>V[a\x037V[`@Qa\x01\xC7\x94\x93\x92\x91\x90a#yV[a\x02\x06a\x02\x016`\x04a#\xA0V[a\x07\xEFV[`@Q\x90\x81R` \x01a\x01\xC7V[a\x02\x06a\x02\"6`\x04a#\xBCV[a\x080V[a\x02:a\x0256`\x04a#\xEBV[a\tKV[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x01\xC7V[a\x02\x06`\x01\x81V[a\x02:a\x02k6`\x04a#\xEBV[a\t\x94V[`\0Ta\x02\x83\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\xC7V[a\x02\x06a\x02\xA96`\x04a$\x10V[a\t\xDCV[a\x02:a\x02\xBC6`\x04a#\xA0V[a\n\xDEV[a\x02\xD4a\x02\xCF6`\x04a#\xA0V[a\x0B\xAAV[`@Qa\x01\xC7\x91\x90a$EV[a\x02:a\x02\xEF6`\x04a#\xEBV[a\x0C\x8EV[a\x02\x06a\x03\x026`\x04a#\xBCV[a\x0C\xB4V[a\x02:a\x03\x156`\x04a#\xEBV[a\r\xC5V[a\x02\x06`Z\x81V[``a\x03/\x84\x84\x84a\r\xEBV[\x94\x93PPPPV[`\0\x80`\0``a\x03b`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[a\x03\x86`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[a\x03\x8F\x89a\n\xDEV[`@\x85\x01R` \x84\x01R\x82R`\0a\x03\xA6\x8Aa\x0B\xAAV[\x90P`\0\x80`\0\x80T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cT\xCF*\xEB`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x045W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a'<\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x04IW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04m\x91\x90a$fV[\x90P`\0a\x04\x89\x8D\x87`\0\x01Q\x88` \x01Q\x89`@\x01Qa\t\xDCV[\x90P\x8B\x15a\x05\xA0W`\0a\x04\x9D\x8C\x84a\x0E\\V[\x87Q\x90\x91P`\0\x90a\x04\xB9\x90a\x04\xB3\x84\x86a\x0E\\V[\x90a\x0ExV[\x90Pa\x04\xC6`\x01\x82a$\x98V[\x88Q\x90\x91Pa\x04\xD6\x90\x8E\x90a$\x98V[\x87R`@\x88\x01Qa\x04\xE8\x90\x82\x90a$\x98V[\x87`@\x01\x81\x81RPPa\x05\x04\x8F\x88`\0\x01Q\x89`@\x01Qa\x0C\xB4V[` \x88\x01\x81\x81R`\x01\x91a\x05\x19\x90\x83\x90a$\x98V[\x90RP` \x80\x89\x01Q\x90\x88\x01Q\x10a\x05\x83W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: y reserve increase`D\x82\x01Rad!`\xF0\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[\x86` \x01Q\x88` \x01Qa\x05\x97\x91\x90a$\xABV[\x94PPPa\x06\xA0V[`\0a\x05\xAC\x8C\x84a\x0E\\V[\x90P`\0a\x05\xCB\x88` \x01Qa\x04\xB3\x85\x85a\x0E\\\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa\x05\xD8`\x01\x82a$\x98V[\x90P\x8C\x88` \x01Qa\x05\xEA\x91\x90a$\x98V[` \x88\x01R`@\x88\x01Qa\x05\xFF\x90\x82\x90a$\x98V[\x87`@\x01\x81\x81RPPa\x06\x1B\x8F\x88` \x01Q\x89`@\x01Qa\x080V[\x80\x88R`\x01\x90\x88\x90a\x06.\x90\x83\x90a$\x98V[\x90RP\x87Q\x87Q\x10a\x06\x8DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: x reserve increase`D\x82\x01Rad!`\xF0\x1B`d\x82\x01R`\x84\x01a\x05zV[\x86Q\x88Qa\x06\x9B\x91\x90a$\xABV[\x94PPP[PP\x82Q` \x80\x85\x01Q`@\x80\x87\x01Q\x81Q\x93\x84\x01\x94\x90\x94R\x82\x01R``\x81\x01\x91\x90\x91R`\0\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c2\x14\x89\x0F\x8E\x84`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x07\x1F\x92\x91\x90a$\xBEV[`\xC0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x07uW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a'<\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x07\x89W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\xAD\x91\x90a$\xD7V[PPPPP\x90P\x80\x83a\x07\xD7\x87`\0\x01Q\x88`@\x01Q\x88`\0\x01Q\x89` \x01Q\x8A`@\x01Qa\x0E\x8DV[\x84\x99P\x99P\x99P\x99PPPPPPP\x93P\x93P\x93P\x93V[`\0\x80a\x07\xFB\x83a\x0B\xAAV[\x90P`\0\x80a\x08\t\x85a\n\xDEV[\x92PP\x91Pa\x08'\x82\x82\x85`\0\x01Q\x86` \x01Q\x87`@\x01Qa\x0E\x8DV[\x95\x94PPPPPV[`\0\x80a\x08<\x85a\n\xDEV[PP`@\x80Q` \x81\x01\x83\x90R\x80\x82\x01\x87\x90R``\x80\x82\x01\x87\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x80\x82\x01\x92\x83\x90R`\0\x80Tb.RK`\xE0\x1B\x90\x94R\x93\x94P\x92\x91`\x01`\x01`\xA0\x1B\x03\x16\x90b.RK\x90a\x08\x9B\x90\x8A\x90\x86\x90`\x84\x01a$\xBEV[` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x08\xF1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a'<\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\t\x05W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t)\x91\x90a$fV[\x90Pa\t@\x86\x86\x83\x86a\t;\x8Ca\x0B\xAAV[a\x0FxV[\x97\x96PPPPPPPV[`\0\x80`\0\x80`\0a\t\\\x87a\n\xDEV[\x92PP\x91P`\0\x80a\tq`\0\x89\x86\x86a\x0F\xF2V[\x91P\x91P`\0a\t\x82\x8A\x84\x84a\x0C\xB4V[\x92\x97P\x91\x95P\x93PPPP\x92P\x92P\x92V[`\0\x80`\0\x80`\0a\t\xA5\x87a\n\xDEV[\x92P\x92PP`\0\x80a\t\xBA`\x01\x89\x86\x86a\x0F\xF2V[\x91P\x91P`\0a\t\xCB\x8A\x84\x84a\x080V[\x97P\x91\x95P\x93PPPP\x92P\x92P\x92V[`@\x80Q` \x81\x01\x85\x90R\x80\x82\x01\x84\x90R``\x80\x82\x01\x84\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x80\x82\x01\x92\x83\x90R`\0\x80Tb.RK`\xE0\x1B\x90\x94R\x92\x90\x91\x83\x91`\x01`\x01`\xA0\x1B\x03\x16\x90b.RK\x90a\n9\x90\x8A\x90\x86\x90`\x84\x01a$\xBEV[` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\n\x8FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a'<\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\n\xA3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\xC7\x91\x90a$fV[\x90Pa\t@\x86\x86\x83\x87a\n\xD9\x8Ca\x0B\xAAV[a\x10[V[`\0\x80T`@Qc3\x85N\xFD`\xE2\x1B\x81R`\x04\x81\x01\x84\x90R\x82\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xCE\x15;\xF4\x90`$\x01```@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0BeW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a'<\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0ByW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\x9D\x91\x90a%-V[\x92P\x92P\x92P\x91\x93\x90\x92PV[a\x0B\xCE`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`\0T`@Qc\xECM\xC1[`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xECM\xC1[\x90`$\x01```@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0CPW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a'<\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0CdW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\x88\x91\x90a%\x9BV[\x92\x91PPV[`\0\x80`\0\x80`\0a\x0C\x9F\x87a\n\xDEV[\x92PP\x91P`\0\x80a\tq`\x01\x89\x86\x86a\x0F\xF2V[`\0\x80a\x0C\xC0\x85a\n\xDEV[P`@\x80Q` \x81\x01\x88\x90R\x80\x82\x01\x83\x90R``\x80\x82\x01\x88\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x80\x82\x01\x92\x83\x90R`\0\x80Tb.RK`\xE0\x1B\x90\x94R\x93\x95P\x93P`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90b.RK\x90a\r \x90\x8A\x90\x86\x90`\x84\x01a$\xBEV[` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\rvW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a'<\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\r\x8AW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\xAE\x91\x90a$fV[\x90Pa\t@\x86\x86\x83\x86a\r\xC0\x8Ca\x0B\xAAV[a\x11;V[`\0\x80`\0\x80`\0a\r\xD6\x87a\n\xDEV[\x92P\x92PP`\0\x80a\t\xBA`\0\x89\x86\x86a\x0F\xF2V[```\0a\r\xFA\x85\x85\x85a\x11\xA9V[\x90P`\0a\x0E\t\x82\x86\x86a\x11\xEEV[\x90P`\0a\x0E\x19\x87\x83\x85\x88a\x120V[\x90Pa\x0E(\x87\x83\x83\x86\x89a\x10[V[\x92P\x86\x82\x84\x87`@Q` \x01a\x0EA\x94\x93\x92\x91\x90a%\xBAV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x93PPPP\x93\x92PPPV[`\0a\x0Eq\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x131V[\x93\x92PPPV[`\0a\x0Eq\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x131V[`\0\x80a\x0E\x9A\x84\x84a\x13_V[\x90P`\0a\x0E\xA8\x85\x85a\x13\x85V[\x90P`\0a\x0E\xB6\x82\x86a\x13\xB4V[\x90P`\0a\x0E\xC4\x8A\x8Aa\x13\xC9V[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0\x81\x12a\x0E\xE2W`\0\x94PPPPPa\x08'V[`\0\x81\x13a\x0E\xF8W`\0\x19\x94PPPPPa\x08'V[`\0a\x0F\x14a\x0F\x0F\x83g\r\xE0\xB6\xB3\xA7d\0\0a%\xEEV[a\x13\xDEV[\x90P`\0\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x0F,\x88\x85a&\x15V[a\x0F6\x91\x90a&[V[a\x0F@\x91\x90a%\xEEV[\x90P`\0a\x0FM\x82a\x14{V[\x90P`\0a\x0FZ\x82a\x16$V[\x90Pa\x0Ff\x8C\x82a\x0E\\V[\x9E\x9DPPPPPPPPPPPPPPV[`\0\x80`\0\x80\x86\x12\x15a\x0F\x9BWP\x83a\x0F\x94\x81`\x96`da\x131V[\x91Pa\x0F\xAEV[a\x0F\xA8\x85`2`da\x16mV[\x90P\x84\x91P[a\x0F\xE6\x88\x88\x88\x87`@Q` \x01a\x0F\xC8\x94\x93\x92\x91\x90a%\xBAV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x82\x84`\na\x01\0a\x16\x8Ca\x16\xC3V[\x98\x97PPPPPPPPV[`\0\x80\x80a\x10\0\x84\x86a\x0ExV[\x90P`\0a\x10\x0E\x87\x83a\x0E\\V[\x90P\x87a\x10$Wa\x10\x1F\x87\x87a$\xABV[a\x10.V[a\x10.\x87\x87a$\x98V[\x93P\x87a\x10DWa\x10?\x81\x86a$\xABV[a\x10NV[a\x10N\x81\x86a$\x98V[\x92PPP\x94P\x94\x92PPPV[`\0\x80`\0\x80`\0a\x10z\x86`\0\x01Q\x8Aa\x13\xC9\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\n\x88\x12\x80\x15a\x10\x94WPa\x10\x91`\na&\x89V[\x88\x13[\x15a\x10\xA5W\x86\x94PPPPPa\x08'V[`\0\x88\x12\x15a\x10\xDFW\x86\x92P\x80\x8A\x11a\x10\xC8Wa\x10\xC3\x81`\x01a$\x98V[a\x10\xD3V[a\x10\xD3\x8A`\x01a$\x98V[\x93Pa\x01\0\x91Pa\x10\xF7V[k\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x92P\x86\x93Pa\x01\0\x91P[a\x11-\x8A\x8A\x8A\x89`@Q` \x01a\x11\x11\x94\x93\x92\x91\x90a%\xBAV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x85\x85`\n\x86a\x17\xD4a\x16\xC3V[\x9A\x99PPPPPPPPPPV[`\0\x80`\0\x80\x86\x12\x15a\x11^WP\x83a\x11W\x81`\x96`da\x131V[\x91Pa\x11qV[a\x11k\x85`2`da\x16mV[\x90P\x84\x91P[a\x0F\xE6\x88\x88\x88\x87`@Q` \x01a\x11\x8B\x94\x93\x92\x91\x90a%\xBAV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x82\x84`\na\x01\0a\x18\x01a\x16\xC3V[`\0\x80a\x11\xB6\x84\x84a\x18.V[\x90P`\0a\x11\xC3\x82a\x18\x9CV[\x90P`\0a\x11\xD0\x82a\x16$V[\x90Pa\t@a\x11\xE7\x82g\r\xE0\xB6\xB3\xA7d\0\0a$\xABV[\x88\x90a\x0ExV[`\0\x80a\x11\xFB\x84\x84a\x19\x05V[\x90P`\0a\x12\x08\x82a\x18\x9CV[\x90P`\0a\x12\x15\x82a\x16$V[\x85Q\x90\x91Pa\t@\x90\x82\x90a\x12*\x90\x8Aa\x0E\\V[\x90a\x0E\\V[`\0\x82\x85\x10a\x12\x81W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FtradingFunction: invalid x\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05zV[`\0\x80g\r\xE0\xB6\xB3\xA7d\0\0a\x12\x97\x88\x87a\x13\xC9V[\x10a\x12\xABW`\x01`\x01`\xFF\x1B\x03\x91Pa\x12\xBBV[a\x12\xB8a\x0F\x0F\x88\x87a\x13\xC9V[\x91P[g\r\xE0\xB6\xB3\xA7d\0\0a\x12\xDB\x87a\x12\xD6\x87`\0\x01Q\x89a\x13\xB4V[a\x13\xC9V[\x10a\x12\xEEWP`\x01`\x01`\xFF\x1B\x03a\x13\x06V[a\x13\x03a\x0F\x0F\x87a\x12\xD6\x87`\0\x01Q\x89a\x13\xB4V[\x90P[`\0a\x13\x1A\x85` \x01Q\x86`@\x01Qa\x13_V[\x90P\x80a\x13'\x83\x85a&\xA5V[a\x0F\xE6\x91\x90a&\xA5V[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x13IW`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[`\0\x80a\x13k\x83a\x19JV[a\x13y\x90c;\x9A\xCA\0a&\xCDV[\x90Pa\x03/\x84\x82a\x13\xB4V[`\0\x80a\x13\xA4\x83a\x13\x9E\x86g\x1B\xC1mgN\xC8\0\0a\x19\xEEV[\x90a\x13\xB4V[\x90Pa\x03/g\x06\xF0[Y\xD3\xB2\0\0\x82[`\0a\x0Eq\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x16mV[`\0a\x0Eq\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x16mV[`\0g\x06\xF0[Y\xD3\xB2\0\0\x82\x03a\x13\xF7WP`\0\x91\x90PV[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x12a\x14\x1FW`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\x14@W`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x14M\x83`\x02a&\x15V[\x90P`\0a\x14Z\x82a\x1A\x1FV[\x90P`\0a\x14pg\x13\xA0K\xBD\xFD\xC9\xBE\x88\x83a\x1C\x9DV[\x90Pa\x08'\x81a&\x89V[`\0h\x02H\xCE6\xA7\x0C\xB2k>\x19\x82\x13a\x14\x96WP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a\x14\xDDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x01a\x05zV[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05`\x01`_\x1B\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1Dm\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1Dn\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[`\0\x80\x82\x12\x15a\x16iW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01RotoUint: negative`\x80\x1B`D\x82\x01R`d\x01a\x05zV[P\x90V[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x16\x85W`\0\x80\xFD[\x04\x92\x91PPV[`\0\x80`\0\x80\x85\x80` \x01\x90Q\x81\x01\x90a\x16\xA6\x91\x90a&\xE4V[\x93PP\x92P\x92Pa\x16\xB9\x85\x84\x84\x84a\x120V[\x96\x95PPPPPPV[`\0\x84\x86\x11\x15a\x16\xF0W`@Qc0\x82\xDF\xDB`\xE1\x1B\x81R`\x04\x81\x01\x87\x90R`$\x81\x01\x86\x90R`D\x01a\x05zV[`\0a\x17\0\x88\x88\x85c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x17\x12\x89\x88\x86c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x17 \x82\x84a&\x15V[\x13\x15a\x17IW`@Qc\x06\xF1\xBE]`\xE2\x1B\x81R`\x04\x81\x01\x83\x90R`$\x81\x01\x82\x90R`D\x01a\x05zV[`\0a\x17U\x89\x89a$\xABV[\x90P`\0[`\x02a\x17f\x8A\x8Ca$\x98V[a\x17p\x91\x90a''V[\x94P`\0a\x17\x82\x8C\x87\x89c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x17\x90\x86\x83a&\x15V[\x13a\x17\x9DW\x85\x99Pa\x17\xA4V[\x85\x9AP\x80\x94P[a\x17\xAE\x8B\x8Ba$\xABV[\x92PP`\x01\x01\x87\x82\x11\x80\x15a\x17\xC2WP\x86\x81\x10[a\x17ZWPPPP\x96\x95PPPPPPV[`\0\x80`\0\x80\x85\x80` \x01\x90Q\x81\x01\x90a\x17\xEE\x91\x90a&\xE4V[\x93PP\x92P\x92Pa\x16\xB9\x83\x83\x87\x84a\x120V[`\0\x80`\0\x80\x85\x80` \x01\x90Q\x81\x01\x90a\x18\x1B\x91\x90a&\xE4V[\x93PP\x92P\x92Pa\x16\xB9\x83\x86\x84\x84a\x120V[\x80Q` \x82\x01Q`@\x83\x01Q`\0\x92\x91\x90\x83a\x18J\x83\x83a\x13_V[\x90P`\0a\x18X\x88\x86a\x1C\xB2V[\x90P`\0a\x18f\x85\x85a\x13\x85V[\x90P\x82a\x18s\x82\x84a&\xA5V[a\x18\x85\x90g\r\xE0\xB6\xB3\xA7d\0\0a&\x15V[a\x18\x8F\x91\x90a&[V[\x99\x98PPPPPPPPPV[`\0\x80g\x13\xA0K\xBD\xFD\xC9\xBE\x88a\x18\xBAg\r\xE0\xB6\xB3\xA7d\0\0\x85a&\x15V[a\x18\xC4\x91\x90a&[V[\x90P`\0a\x18\xD1\x82a&\x89V[\x90P`\0a\x18\xDE\x82a\x1C\xC6V[\x90Pg\x1B\xC1mgN\xC8\0\0a\x18\xFBg\r\xE0\xB6\xB3\xA7d\0\0\x83a&\x15V[a\x08'\x91\x90a&[V[\x80Q` \x82\x01Q`@\x83\x01Q`\0\x92\x91\x90\x83a\x19!\x83\x83a\x13_V[\x90P`\0a\x19/\x88\x86a\x1C\xB2V[\x90P`\0a\x19=\x85\x85a\x13\x85V[\x90P\x82a\x18s\x82\x84a%\xEEV[`\xB5\x81`\x01`\x88\x1B\x81\x10a\x19cW`@\x91\x90\x91\x1B\x90`\x80\x1C[i\x01\0\0\0\0\0\0\0\0\0\x81\x10a\x19\x7FW` \x91\x90\x91\x1B\x90`@\x1C[e\x01\0\0\0\0\0\x81\x10a\x19\x97W`\x10\x91\x90\x91\x1B\x90` \x1C[c\x01\0\0\0\x81\x10a\x19\xADW`\x08\x91\x90\x91\x1B\x90`\x10\x1C[b\x01\0\0\x01\x02`\x12\x1C\x80\x82\x04\x01`\x01\x90\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x90\x81\x90\x04\x81\x11\x90\x03\x90V[`\0a\x0Eqg\r\xE0\xB6\xB3\xA7d\0\0\x83a\x1A\x06\x86a\x1E\xAAV[a\x1A\x10\x91\x90a&\x15V[a\x1A\x1A\x91\x90a&[V[a\x14{V[`\0\x80\x82\x12\x80a\x1A6WPg\x1B\xC1mgN\xC8\0\0\x82\x13[\x15a\x1ATW`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\x1AuW`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81g\x1B\xC1mgN\xC8\0\0\x03a\x1A\x9DW`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x15a\x1A\xA8W\x91\x90PV[`\0g\r\xE0\xB6\xB3\xA7d\0\0\x83\x12a\x1A\xD0Wa\x1A\xCB\x83g\x1B\xC1mgN\xC8\0\0a%\xEEV[a\x1A\xD2V[\x82[\x90P`\0a\x1A\xE8\x82g\x1B\xC1mgN\xC8\0\0a \x85V[\x90P\x80`\0\x03a\x1B\x0BW`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x1B\x16\x82a\x1E\xAAV[\x90P`\0c;\x9A\xCA\0a\x1BAa\x1B<a\x1B6g\x1B\xC1mgN\xC8\0\0a&\x89V[\x85a\x1C\x9DV[a\x19JV[a\x1BK\x91\x90a&\x15V[\x90P`\0\x80a\x1Bb\x83g\x03\xC1f\\z\xAB \0a\x1C\x9DV[a\x1Bt\x90g \x05\xFEO&\x8E\xA0\0a&\xA5V[\x90P`\0a\x1B\xA4\x84a\x1B\x8D\x86f\x9F2u$b\xA0\0a\x1C\x9DV[a\x1B\x9F\x90g\r\xC5R\x7Fd, \0a&\xA5V[a\x1C\x9DV[a\x1B\xB6\x90g\r\xE0\xB6\xB3\xA7d\0\0a&\xA5V[\x90Pa\x1B\xDAg\t\xD0(\xCCo _\xFF\x19\x85a\x1B\xD0\x85\x85a \x85V[a\x1B\x9F\x91\x90a%\xEEV[\x92PPP`\0[`\x02\x81\x10\x15a\x1CuW`\0\x86a\x1B\xF6\x84a\x1C\xC6V[a\x1C\0\x91\x90a%\xEEV[\x90P`\0a\x1C\x0E\x84\x85a\x1C\x9DV[a\x1C\x17\x90a&\x89V[\x90P`\0a\x1C$\x82a\x14{V[\x90P`\0a\x1C2\x86\x85a\x1C\x9DV[a\x1CDg\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x84a\x1C\x9DV[a\x1CN\x91\x90a%\xEEV[\x90Pa\x1CZ\x84\x82a \x85V[a\x1Cd\x90\x87a&\xA5V[\x95P\x84`\x01\x01\x94PPPPPa\x1B\xE1V[g\r\xE0\xB6\xB3\xA7d\0\0\x88\x12a\x1C\x92Wa\x1C\x8D\x82a&\x89V[a\x0F\xE6V[P\x96\x95PPPPPPV[`\0a\x0Eq\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a \x96V[`\0a\x0Eqa\x1C\xC1\x84\x84a\x0ExV[a\x1E\xAAV[`\0\x81`\0\x03a\x1C\xDFWPg\r\xE0\xB6\xB3\xA7d\0\0\x91\x90PV[gV\x98\xEE\xF0fp\0\0\x82\x12a\x1C\xF6WP`\0\x91\x90PV[a\x1D\x07gV\x98\xEE\xF0fp\0\0a&\x89V[\x82\x13a\x1D\x1CWPg\x1B\xC1mgN\xC8\0\0\x91\x90PV[`\0a\x1D'\x83a \xB5V[\x90P`\0a\x1D`g\r\xE0\xB6\xB3\xA7d\0\0a\x1DI\x84g\x1B\xC1mgN\xC8\0\0a\x13\xC9V[a\x1D[\x90g\r\xE0\xB6\xB3\xA7d\0\0a&\xA5V[a \x85V[\x90P`\0\x80\x82a\x1D\xBC\x81a\x1D\xA9\x81a\x1D\x97\x81a\x1D\x84\x81g\x02_\x0F\xE1\x05\xA3\x14\0a\x1C\x9DV[a\x1B\x9F\x90g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19a&\xA5V[a\x1B\x9F\x90g\x14\xA8EL\x19\xE1\xAC\0a&\xA5V[a\x1B\x9F\x90g\x0F\xC1\x0E\x01W\x82w\xFF\x19a&\xA5V[a\x1D\xCE\x90g\x03\xDE\xBD\x08;\x8C|\0a&\xA5V[\x91P\x83\x90Pa\x1E6\x81a\x1E$\x81a\x1E\x12\x81a\x1E\0\x81a\x1D\xED\x81\x8Ba\x1C\x9DV[a\x1B\x9F\x90g\x02\x95\xD4\0\xEA2W\xFF\x19a&\xA5V[a\x1B\x9F\x90g\x01W\xD8\xB2\xEC\xC7\x08\0a&\xA5V[a\x1B\x9F\x90g\x051\n\xA7\xD5!0\0a&\xA5V[a\x1B\x9F\x90g\r\xE0\xCC=\x15a\0\0a&\xA5V[\x91P\x81g\x11\x90\0\xAB\x10\x0F\xFC\0a\x1EL\x87\x88a\x1C\x9DV[a\x1EX\x90`\0\x19a&\x15V[a\x1Eb\x91\x90a%\xEEV[a\x1El\x91\x90a&\xA5V[\x92PP`\0a\x1Ez\x83a\x14{V[\x90P`\0a\x1E\x88\x85\x83a\x1C\x9DV[\x90P`\0\x88\x12a\x1E\x98W\x80a\x0F\xE6V[a\x0F\xE6\x81g\x1B\xC1mgN\xC8\0\0a%\xEEV[`\0\x80\x82\x13a\x1E\xE7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x05zV[`\0``a\x1E\xF4\x84a \xECV[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[`\0a\x0Eq\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x05\x85\x14\x17\x16a \xAEW`\0\x80\xFD[\x05\x92\x91PPV[`\0`\x01`\xFF\x1B\x82\x03a \xDBW`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x12\x15a\x16iWP\x19`\x01\x01\x90V[`\0\x80\x82\x11a!)W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x05zV[P`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x82\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FABI decoding: struct data too sh`D\x82\x01Rb\x1B\xDC\x9D`\xEA\x1B`d\x82\x01R`\x84\x81\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\"fWcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@R\x90V[`\0\x80`\0\x83\x85\x03`\xA0\x81\x12\x15a\"\x85Wa\"\x85a!\x94V[\x845\x93P` \x85\x015\x92P```?\x19\x82\x01\x12\x15a\"\xA5Wa\"\xA5a!\xE4V[Pa\"\xAEa\"5V[`@\x85\x015\x81R``\x85\x015` \x82\x01R`\x80\x85\x015`@\x82\x01R\x80\x91PP\x92P\x92P\x92V[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a\"\xFAW` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\"\xDEV[P`\0` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R`\0a\x0Eq` \x83\x01\x84a\"\xD4V[\x80\x15\x15\x81\x14a#;W`\0\x80\xFD[PV[`\0\x80`\0``\x84\x86\x03\x12\x15a#VWa#Va!\x94V[\x835\x92P` \x84\x015a#h\x81a#-V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[\x84\x15\x15\x81R\x83` \x82\x01R\x82`@\x82\x01R`\x80``\x82\x01R`\0a\x16\xB9`\x80\x83\x01\x84a\"\xD4V[`\0` \x82\x84\x03\x12\x15a#\xB5Wa#\xB5a!\x94V[P5\x91\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a#\xD4Wa#\xD4a!\x94V[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a$\x01Wa$\x01a!\x94V[PP\x805\x92` \x90\x91\x015\x91PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a$)Wa$)a!\x94V[PP\x825\x94` \x84\x015\x94P`@\x84\x015\x93``\x015\x92P\x90PV[\x81Q\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x80\x83\x01Q\x90\x82\x01R``\x81\x01a\x0C\x88V[`\0` \x82\x84\x03\x12\x15a${Wa${a!\x94V[PQ\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x0C\x88Wa\x0C\x88a$\x82V[\x81\x81\x03\x81\x81\x11\x15a\x0C\x88Wa\x0C\x88a$\x82V[\x82\x81R`@` \x82\x01R`\0a\x03/`@\x83\x01\x84a\"\xD4V[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a$\xF3Wa$\xF3a!\x94V[\x86Qa$\xFE\x81a#-V[` \x88\x01Q`@\x89\x01Q``\x8A\x01Q`\x80\x8B\x01Q`\xA0\x90\x9B\x01Q\x93\x9C\x92\x9BP\x90\x99\x90\x98P\x96P\x90\x94P\x92PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a%EWa%Ea!\x94V[\x83Q\x92P` \x84\x01Q\x91P`@\x84\x01Q\x90P\x92P\x92P\x92V[`\0``\x82\x84\x03\x12\x15a%sWa%sa!\xE4V[a%{a\"5V[\x90P\x81Q\x81R` \x82\x01Q` \x82\x01R`@\x82\x01Q`@\x82\x01R\x92\x91PPV[`\0``\x82\x84\x03\x12\x15a%\xB0Wa%\xB0a!\x94V[a\x0Eq\x83\x83a%^V[\x84\x81R` \x80\x82\x01\x85\x90R`@\x80\x83\x01\x85\x90R\x83Q``\x84\x01R\x90\x83\x01Q`\x80\x83\x01R\x82\x01Q`\xA0\x82\x01R`\xC0\x81\x01a\x08'V[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a&\x0EWa&\x0Ea$\x82V[P\x92\x91PPV[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15a&1Wa&1a$\x82V[\x81\x81\x05\x83\x14\x82\x15\x17a\x0C\x88Wa\x0C\x88a$\x82V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a&jWa&ja&EV[`\x01`\xFF\x1B\x82\x14`\0\x19\x84\x14\x16\x15a&\x84Wa&\x84a$\x82V[P\x05\x90V[`\0`\x01`\xFF\x1B\x82\x01a&\x9EWa&\x9Ea$\x82V[P`\0\x03\x90V[\x80\x82\x01\x82\x81\x12`\0\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a&\xC5Wa&\xC5a$\x82V[PP\x92\x91PPV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x0C\x88Wa\x0C\x88a$\x82V[`\0\x80`\0\x80`\xC0\x85\x87\x03\x12\x15a&\xFDWa&\xFDa!\x94V[\x84Q\x93P` \x85\x01Q\x92P`@\x85\x01Q\x91Pa'\x1C\x86``\x87\x01a%^V[\x90P\x92\x95\x91\x94P\x92PV[`\0\x82a'6Wa'6a&EV[P\x04\x90V\xFETarget contract does not contain";
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
        ///Calls the contract's `getInitialPoolData` (0x1d5ac3c9) function
        pub fn get_initial_pool_data(
            &self,
            rx: ::ethers::core::types::U256,
            s: ::ethers::core::types::U256,
            params: LogNormParameters,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([29, 90, 195, 201], (rx, s, params))
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
        ///Calls the contract's `getPoolParams` (0xdc178355) function
        pub fn get_pool_params(
            &self,
            pool_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, LogNormParameters> {
            self.0
                .method_hash([220, 23, 131, 85], pool_id)
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
    ///Container type for all input parameters for the `getInitialPoolData` function with signature `getInitialPoolData(uint256,uint256,(uint256,uint256,uint256))` and selector `0x1d5ac3c9`
    #[derive(
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
        abi = "getInitialPoolData(uint256,uint256,(uint256,uint256,uint256))"
    )]
    pub struct GetInitialPoolDataCall {
        pub rx: ::ethers::core::types::U256,
        pub s: ::ethers::core::types::U256,
        pub params: LogNormParameters,
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
    ///Container type for all input parameters for the `getPoolParams` function with signature `getPoolParams(uint256)` and selector `0xdc178355`
    #[derive(
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
    #[ethcall(name = "getPoolParams", abi = "getPoolParams(uint256)")]
    pub struct GetPoolParamsCall {
        pub pool_id: ::ethers::core::types::U256,
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
        GetInitialPoolData(GetInitialPoolDataCall),
        GetNextLiquidity(GetNextLiquidityCall),
        GetNextReserveX(GetNextReserveXCall),
        GetNextReserveY(GetNextReserveYCall),
        GetPoolParams(GetPoolParamsCall),
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
            if let Ok(decoded) = <GetPoolParamsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetPoolParams(decoded));
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
                Self::GetPoolParams(element) => {
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
                Self::GetInitialPoolData(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetNextLiquidity(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetNextReserveX(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetNextReserveY(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPoolParams(element) => ::core::fmt::Display::fmt(element, f),
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
    impl ::core::convert::From<GetPoolParamsCall> for LogNormalSolverCalls {
        fn from(value: GetPoolParamsCall) -> Self {
            Self::GetPoolParams(value)
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
    ///Container type for all return fields from the `getInitialPoolData` function with signature `getInitialPoolData(uint256,uint256,(uint256,uint256,uint256))` and selector `0x1d5ac3c9`
    #[derive(
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
    ///Container type for all return fields from the `getPoolParams` function with signature `getPoolParams(uint256)` and selector `0xdc178355`
    #[derive(
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
    pub struct GetPoolParamsReturn(pub LogNormParameters);
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
