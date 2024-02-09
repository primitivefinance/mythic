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
                    ::std::borrow::ToOwned::to_owned("ErrorA"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("ErrorA"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("test"),
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
                    ::std::borrow::ToOwned::to_owned("ErrorB"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("ErrorB"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("test"),
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
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15b\0\0^W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`@Qb\0-\x078\x03\x80b\0-\x07\x839\x81\x01`@\x81\x90Rb\0\0\x81\x91b\0\0\xA7V[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90Ub\0\x01$V[`\0` \x82\x84\x03\x12\x15b\0\x01\x05W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x01\x1DW`\0\x80\xFD[\x93\x92PPPV[a+\xD3\x80b\0\x014`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x01XW`\x005`\xE0\x1C\x80cme\"\x99\x11a\0\xEFW\x80c\xAFNC\x7F\x11a\0\xBEW\x80c\xAFNC\x7F\x14a\x03\x06W\x80c\xCE\x15;\xF4\x14a\x03\x19W\x80c\xEE>\x8C\xFB\x14a\x03,W\x80c\xF3\r7\xF2\x14a\x03?W\x80c\xF9\xC2\x82\x11\x14a\x03RWa\x01XV[\x80cme\"\x99\x14a\x02\xA0W\x80c\x7F\x17@\x9C\x14a\x02\xA8W\x80c\x81\xB5\xFA\xC2\x14a\x02\xBBW\x80c\xA8\xC6.v\x14a\x02\xDBWa\x01XV[\x80c;M\x100\x11a\x01+W\x80c;M\x100\x14a\x029W\x80cN\x81\x7F\xD9\x14a\x02LW\x80c^\xB4\x08\xFC\x14a\x02_W\x80cb7V\x9F\x14a\x02rWa\x01XV[\x80c\x12\x06I\xC5\x14a\x01\xBDW\x80c\x1E\x97\x8C\xB0\x14a\x01\xE3W\x80c8\x8E\xCE\xE7\x14a\x01\xF6W\x80c9(\xFF\x97\x14a\x02\x16W[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x01\xD0a\x01\xCB6`\x04a$;V[a\x03ZV[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\xD0a\x01\xF16`\x04a$pV[a\x04~V[a\x02\ta\x02\x046`\x04a%`V[a\x04\x9BV[`@Qa\x01\xDA\x91\x90a&\"V[a\x02)a\x02$6`\x04a&FV[a\x04\xAFV[`@Qa\x01\xDA\x94\x93\x92\x91\x90a&\x81V[a\x01\xD0a\x02G6`\x04a&\xA8V[a\x08\xD2V[a\x01\xD0a\x02Z6`\x04a$pV[a\x08\xF3V[a\x01\xD0a\x02m6`\x04a$;V[a\t\x08V[a\x02\x85a\x02\x806`\x04a&\xC4V[a\n V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x01\xDAV[a\x01\xD0`\0\x81V[a\x02\x85a\x02\xB66`\x04a&\xC4V[a\nzV[a\x02\xCEa\x02\xC96`\x04a&\xA8V[a\n\xD3V[`@Qa\x01\xDA\x91\x90a&\xE9V[`\0Ta\x02\xEE\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\xDAV[a\x01\xD0a\x03\x146`\x04a$;V[a\x0B\xD5V[a\x02\x85a\x03'6`\x04a&\xA8V[a\x0C\xE2V[a\x02\x85a\x03:6`\x04a&\xC4V[a\x0ErV[a\x02\x85a\x03M6`\x04a&\xC4V[a\x0E\x98V[a\x01\xD0`x\x81V[`\0\x80a\x03p\x84\x84a\x03k\x89a\n\xD3V[a\x0E\xBEV[`@\x80Q` \x81\x01\x88\x90R\x80\x82\x01\x83\x90R``\x80\x82\x01\x88\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x80\x82\x01\x92\x83\x90R`\0\x80Tb.RK`\xE0\x1B\x90\x94R\x93\x94P\x92\x91`\x01`\x01`\xA0\x1B\x03\x16\x90b.RK\x90a\x03\xCD\x90\x8B\x90\x86\x90`\x84\x01a'\x14V[` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x04#W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a+\xB3\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x047W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04[\x91\x90a'-V[\x90Pa\x04r\x87\x87\x83\x86a\x04m\x8Da\n\xD3V[a\x0F\0V[\x98\x97PPPPPPPPV[`\0a\x04\x93\x83\x83a\x04\x8E\x87a\n\xD3V[a\x0F\xB1V[\x94\x93PPPPV[``a\x04\x93\x84\x84\x84a\x10\xBAV[\x93\x92PPPV[`\0\x80`\0``a\x04\xDA`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[a\x04\xFE`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[a\x05\x07\x89a\x0C\xE2V[`@\x85\x01R` \x84\x01R\x82R`\0a\x05\x1E\x8Aa\n\xD3V[\x90P`\0\x80a\x05;\x8C\x86`\0\x01Q\x87` \x01Q\x88`@\x01Qa\x0B\xD5V[\x90P\x8A\x15a\x06rW`\0a\x05\\\x84``\x01Q\x8Ca\x11+\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x86Q\x90\x91P`\0\x90a\x05x\x90a\x05r\x84\x86a\x11+V[\x90a\x11@V[\x90Pa\x05\x85`\x01\x82a'_V[\x87Q\x90\x91Pa\x05\x95\x90\x8D\x90a'_V[\x86Ra\x05\xA1\x81\x84a'_V[\x86`@\x01\x81\x81RPP`\0a\x05\xBF\x8F\x88`\0\x01Q\x89`@\x01Qa\x04~V[\x90Pa\x05\xD5\x8F\x88`\0\x01Q\x89`@\x01Q\x84a\x03ZV[` \x88\x01\x81\x81R`\x01\x91a\x05\xEA\x90\x83\x90a'_V[\x90RP` \x80\x89\x01Q\x90\x88\x01Q\x10a\x06TW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: y reserve increase`D\x82\x01Rad!`\xF0\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[\x86` \x01Q\x88` \x01Qa\x06h\x91\x90a'rV[\x94PPPPa\x07\x92V[`\0a\x06\x8B\x84``\x01Q\x8Ca\x11+\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x06\xAA\x87` \x01Qa\x05r\x85\x85a\x11+\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa\x06\xB7`\x01\x82a'_V[\x90P\x8B\x87` \x01Qa\x06\xC9\x91\x90a'_V[` \x87\x01Ra\x06\xD8\x81\x84a'_V[\x86`@\x01\x81\x81RPP`\0a\x06\xF6\x8F\x88` \x01Q\x89`@\x01Qa\x08\xF3V[\x90Pa\x07\x0C\x8F\x88` \x01Q\x89`@\x01Q\x84a\t\x08V[\x80\x88R`\x01\x90\x88\x90a\x07\x1F\x90\x83\x90a'_V[\x90RP\x87Q\x87Q\x10a\x07~W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: x reserve increase`D\x82\x01Rad!`\xF0\x1B`d\x82\x01R`\x84\x01a\x06KV[\x86Q\x88Qa\x07\x8C\x91\x90a'rV[\x94PPPP[P\x82Q` \x80\x85\x01Q`@\x80\x87\x01Q\x81Q\x93\x84\x01\x94\x90\x94R\x82\x01R``\x81\x01\x91\x90\x91R`\0\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c2\x14\x89\x0F\x8E\x84`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x08\x10\x92\x91\x90a'\x14V[`\xC0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x08fW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a+\xB3\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x08zW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\x9E\x91\x90a'\x85V[PPPPP\x90P\x80\x83a\x08\xBA\x87`\0\x01Q\x88`@\x01Q\x88a\x0F\xB1V[\x84\x99P\x99P\x99P\x99PPPPPPP\x93P\x93P\x93P\x93V[`\0\x80`\0a\x08\xE0\x84a\x0C\xE2V[\x92PP\x91Pa\x04\x93\x82\x82a\x04\x8E\x87a\n\xD3V[`\0a\x04\x93\x83\x83a\t\x03\x87a\n\xD3V[a\x11UV[`\0\x80a\t\x1E\x84\x84a\t\x19\x89a\n\xD3V[a\x12#V[`@\x80Q` \x81\x01\x83\x90R\x80\x82\x01\x88\x90R``\x80\x82\x01\x88\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x80\x82\x01\x92\x83\x90R`\0\x80Tb.RK`\xE0\x1B\x90\x94R\x93\x94P\x92\x91`\x01`\x01`\xA0\x1B\x03\x16\x90b.RK\x90a\t{\x90\x8B\x90\x86\x90`\x84\x01a'\x14V[` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\t\xD1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a+\xB3\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\t\xE5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\t\x91\x90a'-V[\x90Pa\x04r\x87\x87\x83\x86a\n\x1B\x8Da\n\xD3V[a\x12hV[`\0\x80`\0\x80`\0a\n1\x87a\x0C\xE2V[\x92PP\x91P`\0\x80a\nF`\0\x89\x86\x86a\x13\x0CV[\x91P\x91P`\0a\nW\x8A\x84\x84a\x04~V[\x90P`\0a\ng\x8B\x85\x85\x85a\x03ZV[\x93\x9B\x93\x9AP\x91\x98P\x91\x96PPPPPPPV[`\0\x80`\0\x80`\0a\n\x8B\x87a\x0C\xE2V[\x92P\x92PP`\0\x80a\n\xA0`\x01\x89\x86\x86a\x13\x0CV[\x91P\x91P`\0a\n\xB1\x8A\x84\x84a\x08\xF3V[\x90P`\0a\n\xC1\x8B\x85\x85\x85a\t\x08V[\x9B\x93\x9AP\x91\x98P\x91\x96PPPPPPPV[a\n\xFE`@Q\x80`\x80\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`\0T`@Qc\xDC\x17\x83U`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xDC\x17\x83U\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0B\x80W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a+\xB3\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0B\x94W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0B\xBC\x91\x90\x81\x01\x90a'\xDBV[\x80` \x01\x90Q\x81\x01\x90a\x0B\xCF\x91\x90a)\xA6V[\x92\x91PPV[`@\x80Q` \x81\x01\x85\x90R\x80\x82\x01\x84\x90R``\x80\x82\x01\x84\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x80\x82\x01\x92\x83\x90R`\0\x80Tb.RK`\xE0\x1B\x90\x94R\x92\x90\x91\x83\x91`\x01`\x01`\xA0\x1B\x03\x16\x90b.RK\x90a\x0C2\x90\x8A\x90\x86\x90`\x84\x01a'\x14V[` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0C\x88W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a+\xB3\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0C\x9CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xC0\x91\x90a'-V[\x90Pa\x0C\xD7\x86\x86\x83\x87a\x0C\xD2\x8Ca\n\xD3V[a\x13uV[\x97\x96PPPPPPPV[`\0\x80`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xAF\xBA\x13\xC4`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\rrW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a+\xB3\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\r\x86W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\xAA\x91\x90a)\xC5V[`\x01`\x01`\xA0\x1B\x03\x16c\xCE\x15;\xF4\x85`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\r\xD7\x91\x81R` \x01\x90V[```@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0E-W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a+\xB3\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0EAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Ee\x91\x90a)\xF1V[\x92P\x92P\x92P\x91\x93\x90\x92PV[`\0\x80`\0\x80`\0a\x0E\x83\x87a\x0C\xE2V[\x92PP\x91P`\0\x80a\nF`\x01\x89\x86\x86a\x13\x0CV[`\0\x80`\0\x80`\0a\x0E\xA9\x87a\x0C\xE2V[\x92P\x92PP`\0\x80a\n\xA0`\0\x89\x86\x86a\x13\x0CV[`\0\x80a\x0E\xCB\x84\x84a\x14\x19V[\x90P`\0a\x0E\xD8\x82a\x14zV[\x90P`\0a\x0E\xE5\x82a\x14\xECV[\x85Q\x90\x91Pa\x0C\xD7\x90\x82\x90a\x0E\xFA\x90\x8Aa\x11+V[\x90a\x11+V[`\0\x82\x80\x85\x83\x81\x12\x15a\x0F@W[`\0\x81\x12\x15a\x0F;Wa\x0F&\x83a\x03\xE9a\x03\xE8a\x155V[\x92Pa\x0F4\x89\x84\x8A\x88a\x15cV[\x90Pa\x0F\x0EV[a\x0FmV[`\0\x81\x13\x15a\x0FmWa\x0FX\x82a\x03\xE7a\x03\xE8a\x16\x91V[\x91Pa\x0Ff\x89\x83\x8A\x88a\x15cV[\x90Pa\x0F@V[a\x0F\xA4\x89\x89\x83\x88`@Q` \x01a\x0F\x87\x94\x93\x92\x91\x90a*\"V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83\x85`\x14`@a\x16\xB0a\x16\xE7V[\x99\x98PPPPPPPPPV[`\0\x80a\x0F\xC6\x83` \x01Q\x84`@\x01Qa\x17\xF8V[\x90P`\0a\x0F\xDC\x84` \x01Q\x85`@\x01Qa\x18\x1EV[\x90P`\0a\x0F\xF7\x85`@\x01Q\x83a\x18M\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x10\x05\x88\x88a\x18bV[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0\x81\x12a\x10#W`\0\x94PPPPPa\x04\xA8V[`\0\x81\x13a\x109W`\0\x19\x94PPPPPa\x04\xA8V[`\0a\x10Ua\x10P\x83g\r\xE0\xB6\xB3\xA7d\0\0a*eV[a\x18wV[\x90P`\0\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x10m\x88\x85a*\x8CV[a\x10w\x91\x90a*\xD2V[a\x10\x81\x91\x90a*eV[\x90P`\0a\x10\x8E\x82a\x19\x14V[\x90P`\0a\x10\x9B\x82a\x14\xECV[\x8AQ\x90\x91Pa\x10\xAA\x90\x82a\x11+V[\x9C\x9BPPPPPPPPPPPPV[```\0a\x10\xC9\x85\x85\x85a\x1A\xBDV[\x90P`\0a\x10\xD8\x82\x86\x86a\x0E\xBEV[\x90P`\0a\x10\xE8\x87\x83\x85\x88a\x15cV[\x90Pa\x10\xF7\x87\x83\x83\x86\x89a\x13uV[\x92P\x86\x82\x84\x87`@Q` \x01a\x11\x10\x94\x93\x92\x91\x90a*\"V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x93PPPP\x93\x92PPPV[`\0a\x04\xA8\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x155V[`\0a\x04\xA8\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x155V[`\0\x80a\x11j\x83` \x01Q\x84`@\x01Qa\x17\xF8V[\x90P`\0a\x11\x80\x84` \x01Q\x85`@\x01Qa\x18\x1EV[\x90P`\0a\x11\x9B\x85`@\x01Q\x83a\x18M\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x85Q\x90\x91P`\0\x90a\x11\xB8\x90a\x11\xB1\x90\x89a\x18MV[\x89\x90a\x18bV[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0\x81\x12a\x11\xD6W`\0\x94PPPPPa\x04\xA8V[`\0\x81\x13a\x11\xECW`\0\x19\x94PPPPPa\x04\xA8V[`\0a\x11\xF7\x82a\x18wV[\x90P`\0\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x12\x0F\x88\x85a*\x8CV[a\x12\x19\x91\x90a*\xD2V[a\x10\x81\x91\x90a+\0V[`\0\x80a\x120\x84\x84a\x1B\x02V[\x90P`\0a\x12=\x82a\x14zV[\x90P`\0a\x12J\x82a\x14\xECV[\x90Pa\x0C\xD7a\x12a\x82g\r\xE0\xB6\xB3\xA7d\0\0a'rV[\x88\x90a\x11+V[`\0\x82\x80\x85\x83\x81\x12\x15a\x12\xA8W[`\0\x81\x12\x15a\x12\xA3Wa\x12\x8E\x83a\x03\xE9a\x03\xE8a\x155V[\x92Pa\x12\x9C\x83\x8A\x8A\x88a\x15cV[\x90Pa\x12vV[a\x12\xD5V[`\0\x81\x13\x15a\x12\xD5Wa\x12\xC0\x82a\x03\xE7a\x03\xE8a\x16\x91V[\x91Pa\x12\xCE\x82\x8A\x8A\x88a\x15cV[\x90Pa\x12\xA8V[a\x0F\xA4\x89\x89\x83\x88`@Q` \x01a\x12\xEF\x94\x93\x92\x91\x90a*\"V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83\x85`\x14`@a\x1BGa\x16\xE7V[`\0\x80\x80a\x13\x1A\x84\x86a\x11@V[\x90P`\0a\x13(\x87\x83a\x11+V[\x90P\x87a\x13>Wa\x139\x87\x87a'rV[a\x13HV[a\x13H\x87\x87a'_V[\x93P\x87a\x13^Wa\x13Y\x81\x86a'rV[a\x13hV[a\x13h\x81\x86a'_V[\x92PPP\x94P\x94\x92PPPV[`\0\x82\x80\x85\x83\x81\x12\x15a\x13\xB5W[`\0\x81\x12\x15a\x13\xB0Wa\x13\x9B\x82a\x03\xE7a\x03\xE8a\x16\x91V[\x91Pa\x13\xA9\x89\x89\x84\x88a\x15cV[\x90Pa\x13\x83V[a\x13\xE2V[`\0\x81\x13\x15a\x13\xE2Wa\x13\xCD\x83a\x03\xE9a\x03\xE8a\x155V[\x92Pa\x13\xDB\x89\x89\x85\x88a\x15cV[\x90Pa\x13\xB5V[a\x0F\xA4\x89\x89\x83\x88`@Q` \x01a\x13\xFC\x94\x93\x92\x91\x90a*\"V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83\x85`\x14`@a\x1Bta\x16\xE7V[\x80Q` \x82\x01Q`@\x83\x01Q`\0\x92\x91\x90\x83a\x145\x83\x83a\x17\xF8V[\x90P`\0a\x14C\x88\x86a\x1B\xA1V[\x90P`\0a\x14Q\x85\x85a\x18\x1EV[\x90P\x82a\x14^\x82\x84a*eV[a\x14p\x90g\r\xE0\xB6\xB3\xA7d\0\0a*\x8CV[a\x0F\xA4\x91\x90a*\xD2V[`\0\x80g\x13\xA0K\xBD\xFD\xC9\xBE\x88a\x14\x98g\r\xE0\xB6\xB3\xA7d\0\0\x85a*\x8CV[a\x14\xA2\x91\x90a*\xD2V[\x90P`\0a\x14\xAF\x82a+(V[\x90P`\0a\x14\xBC\x82a\x1B\xB5V[\x90Pg\x1B\xC1mgN\xC8\0\0a\x14\xD9g\r\xE0\xB6\xB3\xA7d\0\0\x83a*\x8CV[a\x14\xE3\x91\x90a*\xD2V[\x95\x94PPPPPV[`\0\x80\x82\x12\x15a\x151W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01RotoUint: negative`\x80\x1B`D\x82\x01R`d\x01a\x06KV[P\x90V[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x15MW`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[`\0\x82\x85\x10a\x15\xB4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FtradingFunction: invalid x\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06KV[`\0\x80g\r\xE0\xB6\xB3\xA7d\0\0a\x15\xCA\x88\x87a\x18bV[\x10a\x15\xF5W`@Qc\x1A,\x17\xAB`\xE1\x1B\x81R`\x01`\x01`\xFF\x1B\x03`\x04\x82\x01\x81\x90R\x92P`$\x01a\x06KV[a\x16\x02a\x10P\x88\x87a\x18bV[\x91Pg\r\xE0\xB6\xB3\xA7d\0\0a\x16$\x87a\x16\x1F\x87`\0\x01Q\x89a\x18MV[a\x18bV[\x10a\x16OWP`@Qc\t\x81\x8FE`\xE2\x1B\x81R`\x01`\x01`\xFF\x1B\x03`\x04\x82\x01\x81\x90R\x90`$\x01a\x06KV[a\x16da\x10P\x87a\x16\x1F\x87`\0\x01Q\x89a\x18MV[\x90P`\0a\x16z\x85` \x01Q\x86`@\x01Qa\x17\xF8V[\x90P\x80a\x16\x87\x83\x85a+\0V[a\x04r\x91\x90a+\0V[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x16\xA9W`\0\x80\xFD[\x04\x92\x91PPV[`\0\x80`\0\x80\x85\x80` \x01\x90Q\x81\x01\x90a\x16\xCA\x91\x90a+DV[\x93PP\x92P\x92Pa\x16\xDD\x83\x86\x84\x84a\x15cV[\x96\x95PPPPPPV[`\0\x84\x86\x11\x15a\x17\x14W`@Qc0\x82\xDF\xDB`\xE1\x1B\x81R`\x04\x81\x01\x87\x90R`$\x81\x01\x86\x90R`D\x01a\x06KV[`\0a\x17$\x88\x88\x85c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x176\x89\x88\x86c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x17D\x82\x84a*\x8CV[\x13\x15a\x17mW`@Qc\x06\xF1\xBE]`\xE2\x1B\x81R`\x04\x81\x01\x83\x90R`$\x81\x01\x82\x90R`D\x01a\x06KV[`\0a\x17y\x89\x89a'rV[\x90P`\0[`\x02a\x17\x8A\x8A\x8Ca'_V[a\x17\x94\x91\x90a+\x87V[\x94P`\0a\x17\xA6\x8C\x87\x89c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x17\xB4\x86\x83a*\x8CV[\x13a\x17\xC1W\x85\x99Pa\x17\xC8V[\x85\x9AP\x80\x94P[a\x17\xD2\x8B\x8Ba'rV[\x92PP`\x01\x01\x87\x82\x11\x80\x15a\x17\xE6WP\x86\x81\x10[a\x17~WPPPP\x96\x95PPPPPPV[`\0\x80a\x18\x04\x83a\x1D\x9EV[a\x18\x12\x90c;\x9A\xCA\0a+\x9BV[\x90Pa\x04\x93\x84\x82a\x18MV[`\0\x80a\x18=\x83a\x187\x86g\x1B\xC1mgN\xC8\0\0a\x1EBV[\x90a\x18MV[\x90Pa\x04\x93g\x06\xF0[Y\xD3\xB2\0\0\x82[`\0a\x04\xA8\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x16\x91V[`\0a\x04\xA8\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x16\x91V[`\0g\x06\xF0[Y\xD3\xB2\0\0\x82\x03a\x18\x90WP`\0\x91\x90PV[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x12a\x18\xB8W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\x18\xD9W`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x18\xE6\x83`\x02a*\x8CV[\x90P`\0a\x18\xF3\x82a\x1EsV[\x90P`\0a\x19\tg\x13\xA0K\xBD\xFD\xC9\xBE\x88\x83a \xECV[\x90Pa\x14\xE3\x81a+(V[`\0h\x02H\xCE6\xA7\x0C\xB2k>\x19\x82\x13a\x19/WP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a\x19vW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x01a\x06KV[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05`\x01`_\x1B\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1Dm\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1Dn\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[`\0\x80a\x1A\xCA\x84\x84a\x1B\x02V[\x90P`\0a\x1A\xD7\x82a\x14zV[\x90P`\0a\x1A\xE4\x82a\x14\xECV[\x90Pa\x0C\xD7a\x1A\xFB\x82g\r\xE0\xB6\xB3\xA7d\0\0a'rV[\x88\x90a\x11@V[\x80Q` \x82\x01Q`@\x83\x01Q`\0\x92\x91\x90\x83a\x1B\x1E\x83\x83a\x17\xF8V[\x90P`\0a\x1B,\x88\x86a\x1B\xA1V[\x90P`\0a\x1B:\x85\x85a\x18\x1EV[\x90P\x82a\x14^\x82\x84a+\0V[`\0\x80`\0\x80\x85\x80` \x01\x90Q\x81\x01\x90a\x1Ba\x91\x90a+DV[\x93PP\x92P\x92Pa\x16\xDD\x85\x84\x84\x84a\x15cV[`\0\x80`\0\x80\x85\x80` \x01\x90Q\x81\x01\x90a\x1B\x8E\x91\x90a+DV[\x93PP\x92P\x92Pa\x16\xDD\x83\x83\x87\x84a\x15cV[`\0a\x04\xA8a\x1B\xB0\x84\x84a\x11@V[a!\x01V[`\0\x81`\0\x03a\x1B\xCEWPg\r\xE0\xB6\xB3\xA7d\0\0\x91\x90PV[gV\x98\xEE\xF0fp\0\0\x82\x12a\x1B\xE5WP`\0\x91\x90PV[a\x1B\xF6gV\x98\xEE\xF0fp\0\0a+(V[\x82\x13a\x1C\x0BWPg\x1B\xC1mgN\xC8\0\0\x91\x90PV[`\0a\x1C\x16\x83a\"\xDCV[\x90P`\0a\x1COg\r\xE0\xB6\xB3\xA7d\0\0a\x1C8\x84g\x1B\xC1mgN\xC8\0\0a\x18bV[a\x1CJ\x90g\r\xE0\xB6\xB3\xA7d\0\0a+\0V[a#\x13V[\x90P`\0\x80\x82a\x1C\xB0\x81a\x1C\x9D\x81a\x1C\x8B\x81a\x1Cs\x81g\x02_\x0F\xE1\x05\xA3\x14\0a \xECV[a\x1C\x86\x90g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19a+\0V[a \xECV[a\x1C\x86\x90g\x14\xA8EL\x19\xE1\xAC\0a+\0V[a\x1C\x86\x90g\x0F\xC1\x0E\x01W\x82w\xFF\x19a+\0V[a\x1C\xC2\x90g\x03\xDE\xBD\x08;\x8C|\0a+\0V[\x91P\x83\x90Pa\x1D*\x81a\x1D\x18\x81a\x1D\x06\x81a\x1C\xF4\x81a\x1C\xE1\x81\x8Ba \xECV[a\x1C\x86\x90g\x02\x95\xD4\0\xEA2W\xFF\x19a+\0V[a\x1C\x86\x90g\x01W\xD8\xB2\xEC\xC7\x08\0a+\0V[a\x1C\x86\x90g\x051\n\xA7\xD5!0\0a+\0V[a\x1C\x86\x90g\r\xE0\xCC=\x15a\0\0a+\0V[\x91P\x81g\x11\x90\0\xAB\x10\x0F\xFC\0a\x1D@\x87\x88a \xECV[a\x1DL\x90`\0\x19a*\x8CV[a\x1DV\x91\x90a*eV[a\x1D`\x91\x90a+\0V[\x92PP`\0a\x1Dn\x83a\x19\x14V[\x90P`\0a\x1D|\x85\x83a \xECV[\x90P`\0\x88\x12a\x1D\x8CW\x80a\x04rV[a\x04r\x81g\x1B\xC1mgN\xC8\0\0a*eV[`\xB5\x81`\x01`\x88\x1B\x81\x10a\x1D\xB7W`@\x91\x90\x91\x1B\x90`\x80\x1C[i\x01\0\0\0\0\0\0\0\0\0\x81\x10a\x1D\xD3W` \x91\x90\x91\x1B\x90`@\x1C[e\x01\0\0\0\0\0\x81\x10a\x1D\xEBW`\x10\x91\x90\x91\x1B\x90` \x1C[c\x01\0\0\0\x81\x10a\x1E\x01W`\x08\x91\x90\x91\x1B\x90`\x10\x1C[b\x01\0\0\x01\x02`\x12\x1C\x80\x82\x04\x01`\x01\x90\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x90\x81\x90\x04\x81\x11\x90\x03\x90V[`\0a\x04\xA8g\r\xE0\xB6\xB3\xA7d\0\0\x83a\x1EZ\x86a!\x01V[a\x1Ed\x91\x90a*\x8CV[a\x1En\x91\x90a*\xD2V[a\x19\x14V[`\0\x80\x82\x12\x80a\x1E\x8AWPg\x1B\xC1mgN\xC8\0\0\x82\x13[\x15a\x1E\xA8W`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\x1E\xC9W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81g\x1B\xC1mgN\xC8\0\0\x03a\x1E\xF1W`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x15a\x1E\xFCW\x91\x90PV[`\0g\r\xE0\xB6\xB3\xA7d\0\0\x83\x12a\x1F$Wa\x1F\x1F\x83g\x1B\xC1mgN\xC8\0\0a*eV[a\x1F&V[\x82[\x90P`\0a\x1F<\x82g\x1B\xC1mgN\xC8\0\0a#\x13V[\x90P\x80`\0\x03a\x1F_W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x1Fj\x82a!\x01V[\x90P`\0c;\x9A\xCA\0a\x1F\x95a\x1F\x90a\x1F\x8Ag\x1B\xC1mgN\xC8\0\0a+(V[\x85a \xECV[a\x1D\x9EV[a\x1F\x9F\x91\x90a*\x8CV[\x90P`\0\x80a\x1F\xB6\x83g\x03\xC1f\\z\xAB \0a \xECV[a\x1F\xC8\x90g \x05\xFEO&\x8E\xA0\0a+\0V[\x90P`\0a\x1F\xF3\x84a\x1F\xE1\x86f\x9F2u$b\xA0\0a \xECV[a\x1C\x86\x90g\r\xC5R\x7Fd, \0a+\0V[a \x05\x90g\r\xE0\xB6\xB3\xA7d\0\0a+\0V[\x90Pa )g\t\xD0(\xCCo _\xFF\x19\x85a \x1F\x85\x85a#\x13V[a\x1C\x86\x91\x90a*eV[\x92PPP`\0[`\x02\x81\x10\x15a \xC4W`\0\x86a E\x84a\x1B\xB5V[a O\x91\x90a*eV[\x90P`\0a ]\x84\x85a \xECV[a f\x90a+(V[\x90P`\0a s\x82a\x19\x14V[\x90P`\0a \x81\x86\x85a \xECV[a \x93g\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x84a \xECV[a \x9D\x91\x90a*eV[\x90Pa \xA9\x84\x82a#\x13V[a \xB3\x90\x87a+\0V[\x95P\x84`\x01\x01\x94PPPPPa 0V[g\r\xE0\xB6\xB3\xA7d\0\0\x88\x12a \xE1Wa \xDC\x82a+(V[a\x04rV[P\x96\x95PPPPPPV[`\0a\x04\xA8\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a#$V[`\0\x80\x82\x13a!>W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x06KV[`\0``a!K\x84a#CV[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[`\0`\x01`\xFF\x1B\x82\x03a#\x02W`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x12\x15a\x151WP\x19`\x01\x01\x90V[`\0a\x04\xA8\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x05\x85\x14\x17\x16a#<W`\0\x80\xFD[\x05\x92\x91PPV[`\0\x80\x82\x11a#\x80W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x06KV[P`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x82\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a$TWa$Ta#\xEBV[PP\x825\x94` \x84\x015\x94P`@\x84\x015\x93``\x015\x92P\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a$\x88Wa$\x88a#\xEBV[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FABI decoding: struct data too sh`D\x82\x01Rb\x1B\xDC\x9D`\xEA\x1B`d\x82\x01R`\x84\x81\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x80\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a%)Wa%)a$\xF0V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a%XWa%Xa$\xF0V[`@R\x91\x90PV[`\0\x80`\0\x83\x85\x03`\xC0\x81\x12\x15a%yWa%ya#\xEBV[\x845\x93P` \x85\x015\x92P`\x80`?\x19\x82\x01\x12\x15a%\x99Wa%\x99a$\x9FV[Pa%\xA2a%\x06V[`@\x85\x015\x81R``\x85\x015` \x82\x01R`\x80\x85\x015`@\x82\x01R`\xA0\x85\x015``\x82\x01R\x80\x91PP\x92P\x92P\x92V[`\0[\x83\x81\x10\x15a%\xEDW\x81\x81\x01Q\x83\x82\x01R` \x01a%\xD5V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra&\x0E\x81` \x86\x01` \x86\x01a%\xD2V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x04\xA8` \x83\x01\x84a%\xF6V[\x80\x15\x15\x81\x14a&CW`\0\x80\xFD[PV[`\0\x80`\0``\x84\x86\x03\x12\x15a&^Wa&^a#\xEBV[\x835\x92P` \x84\x015a&p\x81a&5V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[\x84\x15\x15\x81R\x83` \x82\x01R\x82`@\x82\x01R`\x80``\x82\x01R`\0a\x16\xDD`\x80\x83\x01\x84a%\xF6V[`\0` \x82\x84\x03\x12\x15a&\xBDWa&\xBDa#\xEBV[P5\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a&\xDAWa&\xDAa#\xEBV[PP\x805\x92` \x90\x91\x015\x91PV[\x81Q\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x80\x83\x01Q\x90\x82\x01R``\x80\x83\x01Q\x90\x82\x01R`\x80\x81\x01a\x0B\xCFV[\x82\x81R`@` \x82\x01R`\0a\x04\x93`@\x83\x01\x84a%\xF6V[`\0` \x82\x84\x03\x12\x15a'BWa'Ba#\xEBV[PQ\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x0B\xCFWa\x0B\xCFa'IV[\x81\x81\x03\x81\x81\x11\x15a\x0B\xCFWa\x0B\xCFa'IV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a'\xA1Wa'\xA1a#\xEBV[\x86Qa'\xAC\x81a&5V[` \x88\x01Q`@\x89\x01Q``\x8A\x01Q`\x80\x8B\x01Q`\xA0\x90\x9B\x01Q\x93\x9C\x92\x9BP\x90\x99\x90\x98P\x96P\x90\x94P\x92PPPV[`\0` \x80\x83\x85\x03\x12\x15a'\xF1Wa'\xF1a#\xEBV[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a(TW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x84\x90R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a(\xBCW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x84\x90R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x81\xFD[\x81Q\x81\x81\x11\x15a(\xCEWa(\xCEa$\xF0V[a(\xE0`\x1F\x82\x01`\x1F\x19\x16\x85\x01a%/V[\x91P\x80\x82R\x86\x84\x82\x85\x01\x01\x11\x15a)FW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x85\x90R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R`\x84\x81\xFD[a)U\x81\x85\x84\x01\x86\x86\x01a%\xD2V[P\x95\x94PPPPPV[`\0`\x80\x82\x84\x03\x12\x15a)tWa)ta$\x9FV[a)|a%\x06V[\x90P\x81Q\x81R` \x82\x01Q` \x82\x01R`@\x82\x01Q`@\x82\x01R``\x82\x01Q``\x82\x01R\x92\x91PPV[`\0`\x80\x82\x84\x03\x12\x15a)\xBBWa)\xBBa#\xEBV[a\x04\xA8\x83\x83a)_V[`\0` \x82\x84\x03\x12\x15a)\xDAWa)\xDAa#\xEBV[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x04\xA8W`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15a*\tWa*\ta#\xEBV[\x83Q\x92P` \x84\x01Q\x91P`@\x84\x01Q\x90P\x92P\x92P\x92V[\x84\x81R` \x81\x01\x84\x90R`@\x81\x01\x83\x90R`\xE0\x81\x01a\x14\xE3``\x83\x01\x84\x80Q\x82R` \x81\x01Q` \x83\x01R`@\x81\x01Q`@\x83\x01R``\x81\x01Q``\x83\x01RPPV[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a*\x85Wa*\x85a'IV[P\x92\x91PPV[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15a*\xA8Wa*\xA8a'IV[\x81\x81\x05\x83\x14\x82\x15\x17a\x0B\xCFWa\x0B\xCFa'IV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a*\xE1Wa*\xE1a*\xBCV[`\x01`\xFF\x1B\x82\x14`\0\x19\x84\x14\x16\x15a*\xFBWa*\xFBa'IV[P\x05\x90V[\x80\x82\x01\x82\x81\x12`\0\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a+ Wa+ a'IV[PP\x92\x91PPV[`\0`\x01`\xFF\x1B\x82\x01a+=Wa+=a'IV[P`\0\x03\x90V[`\0\x80`\0\x80`\xE0\x85\x87\x03\x12\x15a+]Wa+]a#\xEBV[\x84Q\x93P` \x85\x01Q\x92P`@\x85\x01Q\x91Pa+|\x86``\x87\x01a)_V[\x90P\x92\x95\x91\x94P\x92PV[`\0\x82a+\x96Wa+\x96a*\xBCV[P\x04\x90V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x0B\xCFWa\x0B\xCFa'IV\xFETarget contract does not contain";
    /// The bytecode of the contract.
    pub static LOGNORMALSOLVER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x01XW`\x005`\xE0\x1C\x80cme\"\x99\x11a\0\xEFW\x80c\xAFNC\x7F\x11a\0\xBEW\x80c\xAFNC\x7F\x14a\x03\x06W\x80c\xCE\x15;\xF4\x14a\x03\x19W\x80c\xEE>\x8C\xFB\x14a\x03,W\x80c\xF3\r7\xF2\x14a\x03?W\x80c\xF9\xC2\x82\x11\x14a\x03RWa\x01XV[\x80cme\"\x99\x14a\x02\xA0W\x80c\x7F\x17@\x9C\x14a\x02\xA8W\x80c\x81\xB5\xFA\xC2\x14a\x02\xBBW\x80c\xA8\xC6.v\x14a\x02\xDBWa\x01XV[\x80c;M\x100\x11a\x01+W\x80c;M\x100\x14a\x029W\x80cN\x81\x7F\xD9\x14a\x02LW\x80c^\xB4\x08\xFC\x14a\x02_W\x80cb7V\x9F\x14a\x02rWa\x01XV[\x80c\x12\x06I\xC5\x14a\x01\xBDW\x80c\x1E\x97\x8C\xB0\x14a\x01\xE3W\x80c8\x8E\xCE\xE7\x14a\x01\xF6W\x80c9(\xFF\x97\x14a\x02\x16W[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x01\xD0a\x01\xCB6`\x04a$;V[a\x03ZV[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\xD0a\x01\xF16`\x04a$pV[a\x04~V[a\x02\ta\x02\x046`\x04a%`V[a\x04\x9BV[`@Qa\x01\xDA\x91\x90a&\"V[a\x02)a\x02$6`\x04a&FV[a\x04\xAFV[`@Qa\x01\xDA\x94\x93\x92\x91\x90a&\x81V[a\x01\xD0a\x02G6`\x04a&\xA8V[a\x08\xD2V[a\x01\xD0a\x02Z6`\x04a$pV[a\x08\xF3V[a\x01\xD0a\x02m6`\x04a$;V[a\t\x08V[a\x02\x85a\x02\x806`\x04a&\xC4V[a\n V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x01\xDAV[a\x01\xD0`\0\x81V[a\x02\x85a\x02\xB66`\x04a&\xC4V[a\nzV[a\x02\xCEa\x02\xC96`\x04a&\xA8V[a\n\xD3V[`@Qa\x01\xDA\x91\x90a&\xE9V[`\0Ta\x02\xEE\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\xDAV[a\x01\xD0a\x03\x146`\x04a$;V[a\x0B\xD5V[a\x02\x85a\x03'6`\x04a&\xA8V[a\x0C\xE2V[a\x02\x85a\x03:6`\x04a&\xC4V[a\x0ErV[a\x02\x85a\x03M6`\x04a&\xC4V[a\x0E\x98V[a\x01\xD0`x\x81V[`\0\x80a\x03p\x84\x84a\x03k\x89a\n\xD3V[a\x0E\xBEV[`@\x80Q` \x81\x01\x88\x90R\x80\x82\x01\x83\x90R``\x80\x82\x01\x88\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x80\x82\x01\x92\x83\x90R`\0\x80Tb.RK`\xE0\x1B\x90\x94R\x93\x94P\x92\x91`\x01`\x01`\xA0\x1B\x03\x16\x90b.RK\x90a\x03\xCD\x90\x8B\x90\x86\x90`\x84\x01a'\x14V[` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x04#W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a+\xB3\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x047W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04[\x91\x90a'-V[\x90Pa\x04r\x87\x87\x83\x86a\x04m\x8Da\n\xD3V[a\x0F\0V[\x98\x97PPPPPPPPV[`\0a\x04\x93\x83\x83a\x04\x8E\x87a\n\xD3V[a\x0F\xB1V[\x94\x93PPPPV[``a\x04\x93\x84\x84\x84a\x10\xBAV[\x93\x92PPPV[`\0\x80`\0``a\x04\xDA`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[a\x04\xFE`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[a\x05\x07\x89a\x0C\xE2V[`@\x85\x01R` \x84\x01R\x82R`\0a\x05\x1E\x8Aa\n\xD3V[\x90P`\0\x80a\x05;\x8C\x86`\0\x01Q\x87` \x01Q\x88`@\x01Qa\x0B\xD5V[\x90P\x8A\x15a\x06rW`\0a\x05\\\x84``\x01Q\x8Ca\x11+\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x86Q\x90\x91P`\0\x90a\x05x\x90a\x05r\x84\x86a\x11+V[\x90a\x11@V[\x90Pa\x05\x85`\x01\x82a'_V[\x87Q\x90\x91Pa\x05\x95\x90\x8D\x90a'_V[\x86Ra\x05\xA1\x81\x84a'_V[\x86`@\x01\x81\x81RPP`\0a\x05\xBF\x8F\x88`\0\x01Q\x89`@\x01Qa\x04~V[\x90Pa\x05\xD5\x8F\x88`\0\x01Q\x89`@\x01Q\x84a\x03ZV[` \x88\x01\x81\x81R`\x01\x91a\x05\xEA\x90\x83\x90a'_V[\x90RP` \x80\x89\x01Q\x90\x88\x01Q\x10a\x06TW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: y reserve increase`D\x82\x01Rad!`\xF0\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[\x86` \x01Q\x88` \x01Qa\x06h\x91\x90a'rV[\x94PPPPa\x07\x92V[`\0a\x06\x8B\x84``\x01Q\x8Ca\x11+\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x06\xAA\x87` \x01Qa\x05r\x85\x85a\x11+\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa\x06\xB7`\x01\x82a'_V[\x90P\x8B\x87` \x01Qa\x06\xC9\x91\x90a'_V[` \x87\x01Ra\x06\xD8\x81\x84a'_V[\x86`@\x01\x81\x81RPP`\0a\x06\xF6\x8F\x88` \x01Q\x89`@\x01Qa\x08\xF3V[\x90Pa\x07\x0C\x8F\x88` \x01Q\x89`@\x01Q\x84a\t\x08V[\x80\x88R`\x01\x90\x88\x90a\x07\x1F\x90\x83\x90a'_V[\x90RP\x87Q\x87Q\x10a\x07~W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: x reserve increase`D\x82\x01Rad!`\xF0\x1B`d\x82\x01R`\x84\x01a\x06KV[\x86Q\x88Qa\x07\x8C\x91\x90a'rV[\x94PPPP[P\x82Q` \x80\x85\x01Q`@\x80\x87\x01Q\x81Q\x93\x84\x01\x94\x90\x94R\x82\x01R``\x81\x01\x91\x90\x91R`\0\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c2\x14\x89\x0F\x8E\x84`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x08\x10\x92\x91\x90a'\x14V[`\xC0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x08fW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a+\xB3\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x08zW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\x9E\x91\x90a'\x85V[PPPPP\x90P\x80\x83a\x08\xBA\x87`\0\x01Q\x88`@\x01Q\x88a\x0F\xB1V[\x84\x99P\x99P\x99P\x99PPPPPPP\x93P\x93P\x93P\x93V[`\0\x80`\0a\x08\xE0\x84a\x0C\xE2V[\x92PP\x91Pa\x04\x93\x82\x82a\x04\x8E\x87a\n\xD3V[`\0a\x04\x93\x83\x83a\t\x03\x87a\n\xD3V[a\x11UV[`\0\x80a\t\x1E\x84\x84a\t\x19\x89a\n\xD3V[a\x12#V[`@\x80Q` \x81\x01\x83\x90R\x80\x82\x01\x88\x90R``\x80\x82\x01\x88\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x80\x82\x01\x92\x83\x90R`\0\x80Tb.RK`\xE0\x1B\x90\x94R\x93\x94P\x92\x91`\x01`\x01`\xA0\x1B\x03\x16\x90b.RK\x90a\t{\x90\x8B\x90\x86\x90`\x84\x01a'\x14V[` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\t\xD1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a+\xB3\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\t\xE5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\t\x91\x90a'-V[\x90Pa\x04r\x87\x87\x83\x86a\n\x1B\x8Da\n\xD3V[a\x12hV[`\0\x80`\0\x80`\0a\n1\x87a\x0C\xE2V[\x92PP\x91P`\0\x80a\nF`\0\x89\x86\x86a\x13\x0CV[\x91P\x91P`\0a\nW\x8A\x84\x84a\x04~V[\x90P`\0a\ng\x8B\x85\x85\x85a\x03ZV[\x93\x9B\x93\x9AP\x91\x98P\x91\x96PPPPPPPV[`\0\x80`\0\x80`\0a\n\x8B\x87a\x0C\xE2V[\x92P\x92PP`\0\x80a\n\xA0`\x01\x89\x86\x86a\x13\x0CV[\x91P\x91P`\0a\n\xB1\x8A\x84\x84a\x08\xF3V[\x90P`\0a\n\xC1\x8B\x85\x85\x85a\t\x08V[\x9B\x93\x9AP\x91\x98P\x91\x96PPPPPPPV[a\n\xFE`@Q\x80`\x80\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`\0T`@Qc\xDC\x17\x83U`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xDC\x17\x83U\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0B\x80W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a+\xB3\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0B\x94W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0B\xBC\x91\x90\x81\x01\x90a'\xDBV[\x80` \x01\x90Q\x81\x01\x90a\x0B\xCF\x91\x90a)\xA6V[\x92\x91PPV[`@\x80Q` \x81\x01\x85\x90R\x80\x82\x01\x84\x90R``\x80\x82\x01\x84\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x80\x82\x01\x92\x83\x90R`\0\x80Tb.RK`\xE0\x1B\x90\x94R\x92\x90\x91\x83\x91`\x01`\x01`\xA0\x1B\x03\x16\x90b.RK\x90a\x0C2\x90\x8A\x90\x86\x90`\x84\x01a'\x14V[` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0C\x88W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a+\xB3\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0C\x9CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xC0\x91\x90a'-V[\x90Pa\x0C\xD7\x86\x86\x83\x87a\x0C\xD2\x8Ca\n\xD3V[a\x13uV[\x97\x96PPPPPPPV[`\0\x80`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xAF\xBA\x13\xC4`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\rrW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a+\xB3\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\r\x86W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\xAA\x91\x90a)\xC5V[`\x01`\x01`\xA0\x1B\x03\x16c\xCE\x15;\xF4\x85`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\r\xD7\x91\x81R` \x01\x90V[```@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0E-W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a+\xB3\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0EAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Ee\x91\x90a)\xF1V[\x92P\x92P\x92P\x91\x93\x90\x92PV[`\0\x80`\0\x80`\0a\x0E\x83\x87a\x0C\xE2V[\x92PP\x91P`\0\x80a\nF`\x01\x89\x86\x86a\x13\x0CV[`\0\x80`\0\x80`\0a\x0E\xA9\x87a\x0C\xE2V[\x92P\x92PP`\0\x80a\n\xA0`\0\x89\x86\x86a\x13\x0CV[`\0\x80a\x0E\xCB\x84\x84a\x14\x19V[\x90P`\0a\x0E\xD8\x82a\x14zV[\x90P`\0a\x0E\xE5\x82a\x14\xECV[\x85Q\x90\x91Pa\x0C\xD7\x90\x82\x90a\x0E\xFA\x90\x8Aa\x11+V[\x90a\x11+V[`\0\x82\x80\x85\x83\x81\x12\x15a\x0F@W[`\0\x81\x12\x15a\x0F;Wa\x0F&\x83a\x03\xE9a\x03\xE8a\x155V[\x92Pa\x0F4\x89\x84\x8A\x88a\x15cV[\x90Pa\x0F\x0EV[a\x0FmV[`\0\x81\x13\x15a\x0FmWa\x0FX\x82a\x03\xE7a\x03\xE8a\x16\x91V[\x91Pa\x0Ff\x89\x83\x8A\x88a\x15cV[\x90Pa\x0F@V[a\x0F\xA4\x89\x89\x83\x88`@Q` \x01a\x0F\x87\x94\x93\x92\x91\x90a*\"V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83\x85`\x14`@a\x16\xB0a\x16\xE7V[\x99\x98PPPPPPPPPV[`\0\x80a\x0F\xC6\x83` \x01Q\x84`@\x01Qa\x17\xF8V[\x90P`\0a\x0F\xDC\x84` \x01Q\x85`@\x01Qa\x18\x1EV[\x90P`\0a\x0F\xF7\x85`@\x01Q\x83a\x18M\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x10\x05\x88\x88a\x18bV[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0\x81\x12a\x10#W`\0\x94PPPPPa\x04\xA8V[`\0\x81\x13a\x109W`\0\x19\x94PPPPPa\x04\xA8V[`\0a\x10Ua\x10P\x83g\r\xE0\xB6\xB3\xA7d\0\0a*eV[a\x18wV[\x90P`\0\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x10m\x88\x85a*\x8CV[a\x10w\x91\x90a*\xD2V[a\x10\x81\x91\x90a*eV[\x90P`\0a\x10\x8E\x82a\x19\x14V[\x90P`\0a\x10\x9B\x82a\x14\xECV[\x8AQ\x90\x91Pa\x10\xAA\x90\x82a\x11+V[\x9C\x9BPPPPPPPPPPPPV[```\0a\x10\xC9\x85\x85\x85a\x1A\xBDV[\x90P`\0a\x10\xD8\x82\x86\x86a\x0E\xBEV[\x90P`\0a\x10\xE8\x87\x83\x85\x88a\x15cV[\x90Pa\x10\xF7\x87\x83\x83\x86\x89a\x13uV[\x92P\x86\x82\x84\x87`@Q` \x01a\x11\x10\x94\x93\x92\x91\x90a*\"V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x93PPPP\x93\x92PPPV[`\0a\x04\xA8\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x155V[`\0a\x04\xA8\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x155V[`\0\x80a\x11j\x83` \x01Q\x84`@\x01Qa\x17\xF8V[\x90P`\0a\x11\x80\x84` \x01Q\x85`@\x01Qa\x18\x1EV[\x90P`\0a\x11\x9B\x85`@\x01Q\x83a\x18M\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x85Q\x90\x91P`\0\x90a\x11\xB8\x90a\x11\xB1\x90\x89a\x18MV[\x89\x90a\x18bV[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0\x81\x12a\x11\xD6W`\0\x94PPPPPa\x04\xA8V[`\0\x81\x13a\x11\xECW`\0\x19\x94PPPPPa\x04\xA8V[`\0a\x11\xF7\x82a\x18wV[\x90P`\0\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x12\x0F\x88\x85a*\x8CV[a\x12\x19\x91\x90a*\xD2V[a\x10\x81\x91\x90a+\0V[`\0\x80a\x120\x84\x84a\x1B\x02V[\x90P`\0a\x12=\x82a\x14zV[\x90P`\0a\x12J\x82a\x14\xECV[\x90Pa\x0C\xD7a\x12a\x82g\r\xE0\xB6\xB3\xA7d\0\0a'rV[\x88\x90a\x11+V[`\0\x82\x80\x85\x83\x81\x12\x15a\x12\xA8W[`\0\x81\x12\x15a\x12\xA3Wa\x12\x8E\x83a\x03\xE9a\x03\xE8a\x155V[\x92Pa\x12\x9C\x83\x8A\x8A\x88a\x15cV[\x90Pa\x12vV[a\x12\xD5V[`\0\x81\x13\x15a\x12\xD5Wa\x12\xC0\x82a\x03\xE7a\x03\xE8a\x16\x91V[\x91Pa\x12\xCE\x82\x8A\x8A\x88a\x15cV[\x90Pa\x12\xA8V[a\x0F\xA4\x89\x89\x83\x88`@Q` \x01a\x12\xEF\x94\x93\x92\x91\x90a*\"V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83\x85`\x14`@a\x1BGa\x16\xE7V[`\0\x80\x80a\x13\x1A\x84\x86a\x11@V[\x90P`\0a\x13(\x87\x83a\x11+V[\x90P\x87a\x13>Wa\x139\x87\x87a'rV[a\x13HV[a\x13H\x87\x87a'_V[\x93P\x87a\x13^Wa\x13Y\x81\x86a'rV[a\x13hV[a\x13h\x81\x86a'_V[\x92PPP\x94P\x94\x92PPPV[`\0\x82\x80\x85\x83\x81\x12\x15a\x13\xB5W[`\0\x81\x12\x15a\x13\xB0Wa\x13\x9B\x82a\x03\xE7a\x03\xE8a\x16\x91V[\x91Pa\x13\xA9\x89\x89\x84\x88a\x15cV[\x90Pa\x13\x83V[a\x13\xE2V[`\0\x81\x13\x15a\x13\xE2Wa\x13\xCD\x83a\x03\xE9a\x03\xE8a\x155V[\x92Pa\x13\xDB\x89\x89\x85\x88a\x15cV[\x90Pa\x13\xB5V[a\x0F\xA4\x89\x89\x83\x88`@Q` \x01a\x13\xFC\x94\x93\x92\x91\x90a*\"V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83\x85`\x14`@a\x1Bta\x16\xE7V[\x80Q` \x82\x01Q`@\x83\x01Q`\0\x92\x91\x90\x83a\x145\x83\x83a\x17\xF8V[\x90P`\0a\x14C\x88\x86a\x1B\xA1V[\x90P`\0a\x14Q\x85\x85a\x18\x1EV[\x90P\x82a\x14^\x82\x84a*eV[a\x14p\x90g\r\xE0\xB6\xB3\xA7d\0\0a*\x8CV[a\x0F\xA4\x91\x90a*\xD2V[`\0\x80g\x13\xA0K\xBD\xFD\xC9\xBE\x88a\x14\x98g\r\xE0\xB6\xB3\xA7d\0\0\x85a*\x8CV[a\x14\xA2\x91\x90a*\xD2V[\x90P`\0a\x14\xAF\x82a+(V[\x90P`\0a\x14\xBC\x82a\x1B\xB5V[\x90Pg\x1B\xC1mgN\xC8\0\0a\x14\xD9g\r\xE0\xB6\xB3\xA7d\0\0\x83a*\x8CV[a\x14\xE3\x91\x90a*\xD2V[\x95\x94PPPPPV[`\0\x80\x82\x12\x15a\x151W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01RotoUint: negative`\x80\x1B`D\x82\x01R`d\x01a\x06KV[P\x90V[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x15MW`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[`\0\x82\x85\x10a\x15\xB4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FtradingFunction: invalid x\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06KV[`\0\x80g\r\xE0\xB6\xB3\xA7d\0\0a\x15\xCA\x88\x87a\x18bV[\x10a\x15\xF5W`@Qc\x1A,\x17\xAB`\xE1\x1B\x81R`\x01`\x01`\xFF\x1B\x03`\x04\x82\x01\x81\x90R\x92P`$\x01a\x06KV[a\x16\x02a\x10P\x88\x87a\x18bV[\x91Pg\r\xE0\xB6\xB3\xA7d\0\0a\x16$\x87a\x16\x1F\x87`\0\x01Q\x89a\x18MV[a\x18bV[\x10a\x16OWP`@Qc\t\x81\x8FE`\xE2\x1B\x81R`\x01`\x01`\xFF\x1B\x03`\x04\x82\x01\x81\x90R\x90`$\x01a\x06KV[a\x16da\x10P\x87a\x16\x1F\x87`\0\x01Q\x89a\x18MV[\x90P`\0a\x16z\x85` \x01Q\x86`@\x01Qa\x17\xF8V[\x90P\x80a\x16\x87\x83\x85a+\0V[a\x04r\x91\x90a+\0V[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x16\xA9W`\0\x80\xFD[\x04\x92\x91PPV[`\0\x80`\0\x80\x85\x80` \x01\x90Q\x81\x01\x90a\x16\xCA\x91\x90a+DV[\x93PP\x92P\x92Pa\x16\xDD\x83\x86\x84\x84a\x15cV[\x96\x95PPPPPPV[`\0\x84\x86\x11\x15a\x17\x14W`@Qc0\x82\xDF\xDB`\xE1\x1B\x81R`\x04\x81\x01\x87\x90R`$\x81\x01\x86\x90R`D\x01a\x06KV[`\0a\x17$\x88\x88\x85c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x176\x89\x88\x86c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x17D\x82\x84a*\x8CV[\x13\x15a\x17mW`@Qc\x06\xF1\xBE]`\xE2\x1B\x81R`\x04\x81\x01\x83\x90R`$\x81\x01\x82\x90R`D\x01a\x06KV[`\0a\x17y\x89\x89a'rV[\x90P`\0[`\x02a\x17\x8A\x8A\x8Ca'_V[a\x17\x94\x91\x90a+\x87V[\x94P`\0a\x17\xA6\x8C\x87\x89c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x17\xB4\x86\x83a*\x8CV[\x13a\x17\xC1W\x85\x99Pa\x17\xC8V[\x85\x9AP\x80\x94P[a\x17\xD2\x8B\x8Ba'rV[\x92PP`\x01\x01\x87\x82\x11\x80\x15a\x17\xE6WP\x86\x81\x10[a\x17~WPPPP\x96\x95PPPPPPV[`\0\x80a\x18\x04\x83a\x1D\x9EV[a\x18\x12\x90c;\x9A\xCA\0a+\x9BV[\x90Pa\x04\x93\x84\x82a\x18MV[`\0\x80a\x18=\x83a\x187\x86g\x1B\xC1mgN\xC8\0\0a\x1EBV[\x90a\x18MV[\x90Pa\x04\x93g\x06\xF0[Y\xD3\xB2\0\0\x82[`\0a\x04\xA8\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x16\x91V[`\0a\x04\xA8\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x16\x91V[`\0g\x06\xF0[Y\xD3\xB2\0\0\x82\x03a\x18\x90WP`\0\x91\x90PV[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x12a\x18\xB8W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\x18\xD9W`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x18\xE6\x83`\x02a*\x8CV[\x90P`\0a\x18\xF3\x82a\x1EsV[\x90P`\0a\x19\tg\x13\xA0K\xBD\xFD\xC9\xBE\x88\x83a \xECV[\x90Pa\x14\xE3\x81a+(V[`\0h\x02H\xCE6\xA7\x0C\xB2k>\x19\x82\x13a\x19/WP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a\x19vW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x01a\x06KV[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05`\x01`_\x1B\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1Dm\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1Dn\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[`\0\x80a\x1A\xCA\x84\x84a\x1B\x02V[\x90P`\0a\x1A\xD7\x82a\x14zV[\x90P`\0a\x1A\xE4\x82a\x14\xECV[\x90Pa\x0C\xD7a\x1A\xFB\x82g\r\xE0\xB6\xB3\xA7d\0\0a'rV[\x88\x90a\x11@V[\x80Q` \x82\x01Q`@\x83\x01Q`\0\x92\x91\x90\x83a\x1B\x1E\x83\x83a\x17\xF8V[\x90P`\0a\x1B,\x88\x86a\x1B\xA1V[\x90P`\0a\x1B:\x85\x85a\x18\x1EV[\x90P\x82a\x14^\x82\x84a+\0V[`\0\x80`\0\x80\x85\x80` \x01\x90Q\x81\x01\x90a\x1Ba\x91\x90a+DV[\x93PP\x92P\x92Pa\x16\xDD\x85\x84\x84\x84a\x15cV[`\0\x80`\0\x80\x85\x80` \x01\x90Q\x81\x01\x90a\x1B\x8E\x91\x90a+DV[\x93PP\x92P\x92Pa\x16\xDD\x83\x83\x87\x84a\x15cV[`\0a\x04\xA8a\x1B\xB0\x84\x84a\x11@V[a!\x01V[`\0\x81`\0\x03a\x1B\xCEWPg\r\xE0\xB6\xB3\xA7d\0\0\x91\x90PV[gV\x98\xEE\xF0fp\0\0\x82\x12a\x1B\xE5WP`\0\x91\x90PV[a\x1B\xF6gV\x98\xEE\xF0fp\0\0a+(V[\x82\x13a\x1C\x0BWPg\x1B\xC1mgN\xC8\0\0\x91\x90PV[`\0a\x1C\x16\x83a\"\xDCV[\x90P`\0a\x1COg\r\xE0\xB6\xB3\xA7d\0\0a\x1C8\x84g\x1B\xC1mgN\xC8\0\0a\x18bV[a\x1CJ\x90g\r\xE0\xB6\xB3\xA7d\0\0a+\0V[a#\x13V[\x90P`\0\x80\x82a\x1C\xB0\x81a\x1C\x9D\x81a\x1C\x8B\x81a\x1Cs\x81g\x02_\x0F\xE1\x05\xA3\x14\0a \xECV[a\x1C\x86\x90g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19a+\0V[a \xECV[a\x1C\x86\x90g\x14\xA8EL\x19\xE1\xAC\0a+\0V[a\x1C\x86\x90g\x0F\xC1\x0E\x01W\x82w\xFF\x19a+\0V[a\x1C\xC2\x90g\x03\xDE\xBD\x08;\x8C|\0a+\0V[\x91P\x83\x90Pa\x1D*\x81a\x1D\x18\x81a\x1D\x06\x81a\x1C\xF4\x81a\x1C\xE1\x81\x8Ba \xECV[a\x1C\x86\x90g\x02\x95\xD4\0\xEA2W\xFF\x19a+\0V[a\x1C\x86\x90g\x01W\xD8\xB2\xEC\xC7\x08\0a+\0V[a\x1C\x86\x90g\x051\n\xA7\xD5!0\0a+\0V[a\x1C\x86\x90g\r\xE0\xCC=\x15a\0\0a+\0V[\x91P\x81g\x11\x90\0\xAB\x10\x0F\xFC\0a\x1D@\x87\x88a \xECV[a\x1DL\x90`\0\x19a*\x8CV[a\x1DV\x91\x90a*eV[a\x1D`\x91\x90a+\0V[\x92PP`\0a\x1Dn\x83a\x19\x14V[\x90P`\0a\x1D|\x85\x83a \xECV[\x90P`\0\x88\x12a\x1D\x8CW\x80a\x04rV[a\x04r\x81g\x1B\xC1mgN\xC8\0\0a*eV[`\xB5\x81`\x01`\x88\x1B\x81\x10a\x1D\xB7W`@\x91\x90\x91\x1B\x90`\x80\x1C[i\x01\0\0\0\0\0\0\0\0\0\x81\x10a\x1D\xD3W` \x91\x90\x91\x1B\x90`@\x1C[e\x01\0\0\0\0\0\x81\x10a\x1D\xEBW`\x10\x91\x90\x91\x1B\x90` \x1C[c\x01\0\0\0\x81\x10a\x1E\x01W`\x08\x91\x90\x91\x1B\x90`\x10\x1C[b\x01\0\0\x01\x02`\x12\x1C\x80\x82\x04\x01`\x01\x90\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x90\x81\x90\x04\x81\x11\x90\x03\x90V[`\0a\x04\xA8g\r\xE0\xB6\xB3\xA7d\0\0\x83a\x1EZ\x86a!\x01V[a\x1Ed\x91\x90a*\x8CV[a\x1En\x91\x90a*\xD2V[a\x19\x14V[`\0\x80\x82\x12\x80a\x1E\x8AWPg\x1B\xC1mgN\xC8\0\0\x82\x13[\x15a\x1E\xA8W`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\x1E\xC9W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81g\x1B\xC1mgN\xC8\0\0\x03a\x1E\xF1W`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x15a\x1E\xFCW\x91\x90PV[`\0g\r\xE0\xB6\xB3\xA7d\0\0\x83\x12a\x1F$Wa\x1F\x1F\x83g\x1B\xC1mgN\xC8\0\0a*eV[a\x1F&V[\x82[\x90P`\0a\x1F<\x82g\x1B\xC1mgN\xC8\0\0a#\x13V[\x90P\x80`\0\x03a\x1F_W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x1Fj\x82a!\x01V[\x90P`\0c;\x9A\xCA\0a\x1F\x95a\x1F\x90a\x1F\x8Ag\x1B\xC1mgN\xC8\0\0a+(V[\x85a \xECV[a\x1D\x9EV[a\x1F\x9F\x91\x90a*\x8CV[\x90P`\0\x80a\x1F\xB6\x83g\x03\xC1f\\z\xAB \0a \xECV[a\x1F\xC8\x90g \x05\xFEO&\x8E\xA0\0a+\0V[\x90P`\0a\x1F\xF3\x84a\x1F\xE1\x86f\x9F2u$b\xA0\0a \xECV[a\x1C\x86\x90g\r\xC5R\x7Fd, \0a+\0V[a \x05\x90g\r\xE0\xB6\xB3\xA7d\0\0a+\0V[\x90Pa )g\t\xD0(\xCCo _\xFF\x19\x85a \x1F\x85\x85a#\x13V[a\x1C\x86\x91\x90a*eV[\x92PPP`\0[`\x02\x81\x10\x15a \xC4W`\0\x86a E\x84a\x1B\xB5V[a O\x91\x90a*eV[\x90P`\0a ]\x84\x85a \xECV[a f\x90a+(V[\x90P`\0a s\x82a\x19\x14V[\x90P`\0a \x81\x86\x85a \xECV[a \x93g\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x84a \xECV[a \x9D\x91\x90a*eV[\x90Pa \xA9\x84\x82a#\x13V[a \xB3\x90\x87a+\0V[\x95P\x84`\x01\x01\x94PPPPPa 0V[g\r\xE0\xB6\xB3\xA7d\0\0\x88\x12a \xE1Wa \xDC\x82a+(V[a\x04rV[P\x96\x95PPPPPPV[`\0a\x04\xA8\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a#$V[`\0\x80\x82\x13a!>W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x06KV[`\0``a!K\x84a#CV[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[`\0`\x01`\xFF\x1B\x82\x03a#\x02W`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x12\x15a\x151WP\x19`\x01\x01\x90V[`\0a\x04\xA8\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x05\x85\x14\x17\x16a#<W`\0\x80\xFD[\x05\x92\x91PPV[`\0\x80\x82\x11a#\x80W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x06KV[P`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x82\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a$TWa$Ta#\xEBV[PP\x825\x94` \x84\x015\x94P`@\x84\x015\x93``\x015\x92P\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a$\x88Wa$\x88a#\xEBV[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FABI decoding: struct data too sh`D\x82\x01Rb\x1B\xDC\x9D`\xEA\x1B`d\x82\x01R`\x84\x81\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x80\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a%)Wa%)a$\xF0V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a%XWa%Xa$\xF0V[`@R\x91\x90PV[`\0\x80`\0\x83\x85\x03`\xC0\x81\x12\x15a%yWa%ya#\xEBV[\x845\x93P` \x85\x015\x92P`\x80`?\x19\x82\x01\x12\x15a%\x99Wa%\x99a$\x9FV[Pa%\xA2a%\x06V[`@\x85\x015\x81R``\x85\x015` \x82\x01R`\x80\x85\x015`@\x82\x01R`\xA0\x85\x015``\x82\x01R\x80\x91PP\x92P\x92P\x92V[`\0[\x83\x81\x10\x15a%\xEDW\x81\x81\x01Q\x83\x82\x01R` \x01a%\xD5V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra&\x0E\x81` \x86\x01` \x86\x01a%\xD2V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x04\xA8` \x83\x01\x84a%\xF6V[\x80\x15\x15\x81\x14a&CW`\0\x80\xFD[PV[`\0\x80`\0``\x84\x86\x03\x12\x15a&^Wa&^a#\xEBV[\x835\x92P` \x84\x015a&p\x81a&5V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[\x84\x15\x15\x81R\x83` \x82\x01R\x82`@\x82\x01R`\x80``\x82\x01R`\0a\x16\xDD`\x80\x83\x01\x84a%\xF6V[`\0` \x82\x84\x03\x12\x15a&\xBDWa&\xBDa#\xEBV[P5\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a&\xDAWa&\xDAa#\xEBV[PP\x805\x92` \x90\x91\x015\x91PV[\x81Q\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x80\x83\x01Q\x90\x82\x01R``\x80\x83\x01Q\x90\x82\x01R`\x80\x81\x01a\x0B\xCFV[\x82\x81R`@` \x82\x01R`\0a\x04\x93`@\x83\x01\x84a%\xF6V[`\0` \x82\x84\x03\x12\x15a'BWa'Ba#\xEBV[PQ\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x0B\xCFWa\x0B\xCFa'IV[\x81\x81\x03\x81\x81\x11\x15a\x0B\xCFWa\x0B\xCFa'IV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a'\xA1Wa'\xA1a#\xEBV[\x86Qa'\xAC\x81a&5V[` \x88\x01Q`@\x89\x01Q``\x8A\x01Q`\x80\x8B\x01Q`\xA0\x90\x9B\x01Q\x93\x9C\x92\x9BP\x90\x99\x90\x98P\x96P\x90\x94P\x92PPPV[`\0` \x80\x83\x85\x03\x12\x15a'\xF1Wa'\xF1a#\xEBV[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a(TW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x84\x90R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a(\xBCW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x84\x90R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x81\xFD[\x81Q\x81\x81\x11\x15a(\xCEWa(\xCEa$\xF0V[a(\xE0`\x1F\x82\x01`\x1F\x19\x16\x85\x01a%/V[\x91P\x80\x82R\x86\x84\x82\x85\x01\x01\x11\x15a)FW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x85\x90R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R`\x84\x81\xFD[a)U\x81\x85\x84\x01\x86\x86\x01a%\xD2V[P\x95\x94PPPPPV[`\0`\x80\x82\x84\x03\x12\x15a)tWa)ta$\x9FV[a)|a%\x06V[\x90P\x81Q\x81R` \x82\x01Q` \x82\x01R`@\x82\x01Q`@\x82\x01R``\x82\x01Q``\x82\x01R\x92\x91PPV[`\0`\x80\x82\x84\x03\x12\x15a)\xBBWa)\xBBa#\xEBV[a\x04\xA8\x83\x83a)_V[`\0` \x82\x84\x03\x12\x15a)\xDAWa)\xDAa#\xEBV[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x04\xA8W`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15a*\tWa*\ta#\xEBV[\x83Q\x92P` \x84\x01Q\x91P`@\x84\x01Q\x90P\x92P\x92P\x92V[\x84\x81R` \x81\x01\x84\x90R`@\x81\x01\x83\x90R`\xE0\x81\x01a\x14\xE3``\x83\x01\x84\x80Q\x82R` \x81\x01Q` \x83\x01R`@\x81\x01Q`@\x83\x01R``\x81\x01Q``\x83\x01RPPV[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a*\x85Wa*\x85a'IV[P\x92\x91PPV[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15a*\xA8Wa*\xA8a'IV[\x81\x81\x05\x83\x14\x82\x15\x17a\x0B\xCFWa\x0B\xCFa'IV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a*\xE1Wa*\xE1a*\xBCV[`\x01`\xFF\x1B\x82\x14`\0\x19\x84\x14\x16\x15a*\xFBWa*\xFBa'IV[P\x05\x90V[\x80\x82\x01\x82\x81\x12`\0\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a+ Wa+ a'IV[PP\x92\x91PPV[`\0`\x01`\xFF\x1B\x82\x01a+=Wa+=a'IV[P`\0\x03\x90V[`\0\x80`\0\x80`\xE0\x85\x87\x03\x12\x15a+]Wa+]a#\xEBV[\x84Q\x93P` \x85\x01Q\x92P`@\x85\x01Q\x91Pa+|\x86``\x87\x01a)_V[\x90P\x92\x95\x91\x94P\x92PV[`\0\x82a+\x96Wa+\x96a*\xBCV[P\x04\x90V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x0B\xCFWa\x0B\xCFa'IV\xFETarget contract does not contain";
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
    ///Custom Error type `ErrorA` with signature `ErrorA(int256)` and selector `0x34582f56`
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
    #[etherror(name = "ErrorA", abi = "ErrorA(int256)")]
    pub struct ErrorA {
        pub test: ::ethers::core::types::I256,
    }
    ///Custom Error type `ErrorB` with signature `ErrorB(int256)` and selector `0x26063d14`
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
    #[etherror(name = "ErrorB", abi = "ErrorB(int256)")]
    pub struct ErrorB {
        pub test: ::ethers::core::types::I256,
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
        ErrorA(ErrorA),
        ErrorB(ErrorB),
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
            if let Ok(decoded) = <ErrorA as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ErrorA(decoded));
            }
            if let Ok(decoded) = <ErrorB as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ErrorB(decoded));
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
                Self::ErrorA(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ErrorB(element) => ::ethers::core::abi::AbiEncode::encode(element),
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
                _ if selector == <ErrorA as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector == <ErrorB as ::ethers::contract::EthError>::selector() => {
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
                Self::ErrorA(element) => ::core::fmt::Display::fmt(element, f),
                Self::ErrorB(element) => ::core::fmt::Display::fmt(element, f),
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
    impl ::core::convert::From<ErrorA> for LogNormalSolverErrors {
        fn from(value: ErrorA) -> Self {
            Self::ErrorA(value)
        }
    }
    impl ::core::convert::From<ErrorB> for LogNormalSolverErrors {
        fn from(value: ErrorB) -> Self {
            Self::ErrorB(value)
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
        GetPriceGivenXL(GetPriceGivenXLCall),
        GetPriceGivenYL(GetPriceGivenYLCall),
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
                Self::GetPriceGivenXL(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPriceGivenYL(element) => ::core::fmt::Display::fmt(element, f),
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
