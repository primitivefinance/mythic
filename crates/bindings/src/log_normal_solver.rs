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
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15b\0\0^W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`@Qb\0/\xBA8\x03\x80b\0/\xBA\x839\x81\x01`@\x81\x90Rb\0\0\x81\x91b\0\0\xA7V[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90Ub\0\x01$V[`\0` \x82\x84\x03\x12\x15b\0\x01\x05W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x01\x1DW`\0\x80\xFD[\x93\x92PPPV[a.\x86\x80b\0\x014`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x01\x8FW`\x005`\xE0\x1C\x80c\x7F\x17@\x9C\x11a\x01\x05W\x80c\xCB\x1FU2\x11a\0\xC9W\x80c\xCB\x1FU2\x14a\x03\x8DW\x80c\xCE\x15;\xF4\x14a\x03\xA0W\x80c\xE9G\x16\xD5\x14a\x03\xB3W\x80c\xEE>\x8C\xFB\x14a\x03\xC6W\x80c\xF3\r7\xF2\x14a\x03\xD9W\x80c\xF9\xC2\x82\x11\x14a\x03\xECWa\x01\x8FV[\x80c\x7F\x17@\x9C\x14a\x03\tW\x80c\x81\xB5\xFA\xC2\x14a\x03\x1CW\x80c\xA8\xC6.v\x14a\x03<W\x80c\xAFNC\x7F\x14a\x03gW\x80c\xB0\x9D\x04\xE5\x14a\x03zWa\x01\x8FV[\x80c;&\x8D]\x11a\x01WW\x80c;&\x8D]\x14a\x02\x87W\x80c;M\x100\x14a\x02\x9AW\x80cN\x81\x7F\xD9\x14a\x02\xADW\x80c^\xB4\x08\xFC\x14a\x02\xC0W\x80cb7V\x9F\x14a\x02\xD3W\x80cme\"\x99\x14a\x03\x01Wa\x01\x8FV[\x80c\x04 X\n\x14a\x01\xF4W\x80c\x12\x06I\xC5\x14a\x02\x1DW\x80c\x13N\xAD\x12\x14a\x02>W\x80c\x1E\x97\x8C\xB0\x14a\x02QW\x80c9(\xFF\x97\x14a\x02dW[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x02\x07a\x02\x026`\x04a%\x9BV[a\x03\xF4V[`@Qa\x02\x14\x91\x90a&\x10V[`@Q\x80\x91\x03\x90\xF3[a\x020a\x02+6`\x04a&#V[a\x04\tV[`@Q\x90\x81R` \x01a\x02\x14V[a\x02\x07a\x02L6`\x04a'1V[a\x05-V[a\x020a\x02_6`\x04a'\xB7V[a\x05DV[a\x02wa\x02r6`\x04a'\xF4V[a\x05YV[`@Qa\x02\x14\x94\x93\x92\x91\x90a(/V[a\x02\x07a\x02\x956`\x04a%\x9BV[a\t~V[a\x020a\x02\xA86`\x04a(VV[a\t\x8AV[a\x020a\x02\xBB6`\x04a'\xB7V[a\t\xB3V[a\x020a\x02\xCE6`\x04a&#V[a\t\xC8V[a\x02\xE6a\x02\xE16`\x04a%\x9BV[a\n\xE0V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x02\x14V[a\x020`\0\x81V[a\x02\xE6a\x03\x176`\x04a%\x9BV[a\x0B:V[a\x03/a\x03*6`\x04a(VV[a\x0B\x93V[`@Qa\x02\x14\x91\x90a(rV[`\0Ta\x03O\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\x14V[a\x020a\x03u6`\x04a&#V[a\x0C\x9FV[a\x02\x07a\x03\x886`\x04a(VV[a\r\xACV[a\x02\x07a\x03\x9B6`\x04a(\xB0V[a\r\xB7V[a\x02\xE6a\x03\xAE6`\x04a(VV[a\r\xC2V[a\x02\x07a\x03\xC16`\x04a%\x9BV[a\x0FRV[a\x02\xE6a\x03\xD46`\x04a%\x9BV[a\x0F^V[a\x02\xE6a\x03\xE76`\x04a%\x9BV[a\x0F\x84V[a\x020`x\x81V[``a\x04\0\x83\x83a\x0F\xAAV[\x90P[\x92\x91PPV[`\0\x80a\x04\x1F\x84\x84a\x04\x1A\x89a\x0B\x93V[a\x0F\xD9V[`@\x80Q` \x81\x01\x88\x90R\x80\x82\x01\x83\x90R``\x80\x82\x01\x88\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x80\x82\x01\x92\x83\x90R`\0\x80Tb.RK`\xE0\x1B\x90\x94R\x93\x94P\x92\x91`\x01`\x01`\xA0\x1B\x03\x16\x90b.RK\x90a\x04|\x90\x8B\x90\x86\x90`\x84\x01a(\xD0V[` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x04\xD2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a.1\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x04\xE6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\n\x91\x90a(\xE9V[\x90Pa\x05!\x87\x87\x83\x86a\x05\x1C\x8Da\x0B\x93V[a\x10\x1BV[\x98\x97PPPPPPPPV[``a\x05:\x84\x84\x84a\x10\xCCV[\x90P[\x93\x92PPPV[`\0a\x05:\x83\x83a\x05T\x87a\x0B\x93V[a\x11=V[`\0\x80`\0``a\x05\x84`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[a\x05\xA8`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[a\x05\xB1\x89a\r\xC2V[`@\x85\x01R` \x84\x01R\x82R`\0a\x05\xC8\x8Aa\x0B\x93V[\x90P`\0\x80a\x05\xE5\x8C\x86`\0\x01Q\x87` \x01Q\x88`@\x01Qa\x0C\x9FV[\x90P\x8A\x15a\x07\x1CW`\0a\x06\x06\x84``\x01Q\x8Ca\x12F\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x86Q\x90\x91P`\0\x90a\x06\"\x90a\x06\x1C\x84\x86a\x12FV[\x90a\x12[V[\x90Pa\x06/`\x01\x82a)\x1BV[\x87Q\x90\x91Pa\x06?\x90\x8D\x90a)\x1BV[\x86Ra\x06K\x81\x84a)\x1BV[\x86`@\x01\x81\x81RPP`\0a\x06i\x8F\x88`\0\x01Q\x89`@\x01Qa\x05DV[\x90Pa\x06\x7F\x8F\x88`\0\x01Q\x89`@\x01Q\x84a\x04\tV[` \x88\x01\x81\x81R`\x01\x91a\x06\x94\x90\x83\x90a)\x1BV[\x90RP` \x80\x89\x01Q\x90\x88\x01Q\x10a\x06\xFEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: y reserve increase`D\x82\x01Rad!`\xF0\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[\x86` \x01Q\x88` \x01Qa\x07\x12\x91\x90a).V[\x94PPPPa\x08<V[`\0a\x075\x84``\x01Q\x8Ca\x12F\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x07T\x87` \x01Qa\x06\x1C\x85\x85a\x12F\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa\x07a`\x01\x82a)\x1BV[\x90P\x8B\x87` \x01Qa\x07s\x91\x90a)\x1BV[` \x87\x01Ra\x07\x82\x81\x84a)\x1BV[\x86`@\x01\x81\x81RPP`\0a\x07\xA0\x8F\x88` \x01Q\x89`@\x01Qa\t\xB3V[\x90Pa\x07\xB6\x8F\x88` \x01Q\x89`@\x01Q\x84a\t\xC8V[\x80\x88R`\x01\x90\x88\x90a\x07\xC9\x90\x83\x90a)\x1BV[\x90RP\x87Q\x87Q\x10a\x08(W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: x reserve increase`D\x82\x01Rad!`\xF0\x1B`d\x82\x01R`\x84\x01a\x06\xF5V[\x86Q\x88Qa\x086\x91\x90a).V[\x94PPPP[P\x82Q` \x80\x85\x01Q`@\x80\x87\x01Q\x81Q\x93\x84\x01\x94\x90\x94R\x82\x01R``\x81\x01\x91\x90\x91R`\0\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16ch\xBD>80\x8F\x85`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x08\xBC\x93\x92\x91\x90a)AV[`\xC0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\t\x12W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a.1\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\t&W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\tJ\x91\x90a)hV[PPPPP\x90P\x80\x83a\tf\x87`\0\x01Q\x88`@\x01Q\x88a\x11=V[\x84\x99P\x99P\x99P\x99PPPPPPP\x93P\x93P\x93P\x93V[``a\x04\0\x83\x83a\x12pV[`\0\x80`\0a\t\x98\x84a\r\xC2V[\x92PP\x91Pa\t\xAB\x82\x82a\x05T\x87a\x0B\x93V[\x94\x93PPPPV[`\0a\x05:\x83\x83a\t\xC3\x87a\x0B\x93V[a\x12\x88V[`\0\x80a\t\xDE\x84\x84a\t\xD9\x89a\x0B\x93V[a\x13VV[`@\x80Q` \x81\x01\x83\x90R\x80\x82\x01\x88\x90R``\x80\x82\x01\x88\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x80\x82\x01\x92\x83\x90R`\0\x80Tb.RK`\xE0\x1B\x90\x94R\x93\x94P\x92\x91`\x01`\x01`\xA0\x1B\x03\x16\x90b.RK\x90a\n;\x90\x8B\x90\x86\x90`\x84\x01a(\xD0V[` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\n\x91W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a.1\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\n\xA5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\xC9\x91\x90a(\xE9V[\x90Pa\x05!\x87\x87\x83\x86a\n\xDB\x8Da\x0B\x93V[a\x13\x9BV[`\0\x80`\0\x80`\0a\n\xF1\x87a\r\xC2V[\x92PP\x91P`\0\x80a\x0B\x06`\0\x89\x86\x86a\x14?V[\x91P\x91P`\0a\x0B\x17\x8A\x84\x84a\x05DV[\x90P`\0a\x0B'\x8B\x85\x85\x85a\x04\tV[\x93\x9B\x93\x9AP\x91\x98P\x91\x96PPPPPPPV[`\0\x80`\0\x80`\0a\x0BK\x87a\r\xC2V[\x92P\x92PP`\0\x80a\x0B``\x01\x89\x86\x86a\x14?V[\x91P\x91P`\0a\x0Bq\x8A\x84\x84a\t\xB3V[\x90P`\0a\x0B\x81\x8B\x85\x85\x85a\t\xC8V[\x9B\x93\x9AP\x91\x98P\x91\x96PPPPPPPV[a\x0B\xCE`@Q\x80`\xA0\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81RP\x90V[`\0T`@Qc\xDC\x17\x83U`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xDC\x17\x83U\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0CPW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a.1\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0CdW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0C\x8C\x91\x90\x81\x01\x90a)\xBEV[\x80` \x01\x90Q\x81\x01\x90a\x04\x03\x91\x90a+\x9CV[`@\x80Q` \x81\x01\x85\x90R\x80\x82\x01\x84\x90R``\x80\x82\x01\x84\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x80\x82\x01\x92\x83\x90R`\0\x80Tb.RK`\xE0\x1B\x90\x94R\x92\x90\x91\x83\x91`\x01`\x01`\xA0\x1B\x03\x16\x90b.RK\x90a\x0C\xFC\x90\x8A\x90\x86\x90`\x84\x01a(\xD0V[` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\rRW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a.1\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\rfW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\x8A\x91\x90a(\xE9V[\x90Pa\r\xA1\x86\x86\x83\x87a\r\x9C\x8Ca\x0B\x93V[a\x14\xA8V[\x97\x96PPPPPPPV[``a\x04\x03\x82a\x15LV[``a\x04\x03\x82a\x15xV[`\0\x80`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xAF\xBA\x13\xC4`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0ERW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a.1\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0EfW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\x8A\x91\x90a+\xBBV[`\x01`\x01`\xA0\x1B\x03\x16c\xCE\x15;\xF4\x85`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0E\xB7\x91\x81R` \x01\x90V[```@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0F\rW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a.1\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0F!W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0FE\x91\x90a+\xDBV[\x92P\x92P\x92P\x91\x93\x90\x92PV[``a\x04\0\x83\x83a\x15\x8EV[`\0\x80`\0\x80`\0a\x0Fo\x87a\r\xC2V[\x92PP\x91P`\0\x80a\x0B\x06`\x01\x89\x86\x86a\x14?V[`\0\x80`\0\x80`\0a\x0F\x95\x87a\r\xC2V[\x92P\x92PP`\0\x80a\x0B``\0\x89\x86\x86a\x14?V[```\x02\x83\x83`@Q` \x01a\x0F\xC2\x93\x92\x91\x90a,.V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x92\x91PPV[`\0\x80a\x0F\xE6\x84\x84a\x15\xA6V[\x90P`\0a\x0F\xF3\x82a\x16\x07V[\x90P`\0a\x10\0\x82a\x16yV[\x85Q\x90\x91Pa\r\xA1\x90\x82\x90a\x10\x15\x90\x8Aa\x12FV[\x90a\x12FV[`\0\x82\x80\x85\x83\x81\x12\x15a\x10[W[`\0\x81\x12\x15a\x10VWa\x10A\x83a\x03\xE9a\x03\xE8a\x16\xC2V[\x92Pa\x10O\x89\x84\x8A\x88a\x16\xF0V[\x90Pa\x10)V[a\x10\x88V[`\0\x81\x13\x15a\x10\x88Wa\x10s\x82a\x03\xE7a\x03\xE8a\x17\xF1V[\x91Pa\x10\x81\x89\x83\x8A\x88a\x16\xF0V[\x90Pa\x10[V[a\x10\xBF\x89\x89\x83\x88`@Q` \x01a\x10\xA2\x94\x93\x92\x91\x90a,MV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83\x85`\x14`@a\x18\x10a\x18GV[\x99\x98PPPPPPPPPV[```\0a\x10\xDB\x85\x85\x85a\x19XV[\x90P`\0a\x10\xEA\x82\x86\x86a\x0F\xD9V[\x90P`\0a\x10\xFA\x87\x83\x85\x88a\x16\xF0V[\x90Pa\x11\t\x87\x83\x83\x86\x89a\x14\xA8V[\x92P\x86\x82\x84\x87`@Q` \x01a\x11\"\x94\x93\x92\x91\x90a,MV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x93PPPP\x93\x92PPPV[`\0\x80a\x11R\x83` \x01Q\x84`@\x01Qa\x19\x9DV[\x90P`\0a\x11h\x84` \x01Q\x85`@\x01Qa\x19\xC3V[\x90P`\0a\x11\x83\x85`@\x01Q\x83a\x19\xF2\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x11\x91\x88\x88a\x1A\x07V[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0\x81\x12a\x11\xAFW`\0\x94PPPPPa\x05=V[`\0\x81\x13a\x11\xC5W`\0\x19\x94PPPPPa\x05=V[`\0a\x11\xE1a\x11\xDC\x83g\r\xE0\xB6\xB3\xA7d\0\0a,\xA1V[a\x1A\x1CV[\x90P`\0\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x11\xF9\x88\x85a,\xC8V[a\x12\x03\x91\x90a-\x0EV[a\x12\r\x91\x90a,\xA1V[\x90P`\0a\x12\x1A\x82a\x1A\xB9V[\x90P`\0a\x12'\x82a\x16yV[\x8AQ\x90\x91Pa\x126\x90\x82a\x12FV[\x9C\x9BPPPPPPPPPPPPV[`\0a\x04\0\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x16\xC2V[`\0a\x04\0\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x16\xC2V[```\x04\x83\x83`@Q` \x01a\x0F\xC2\x93\x92\x91\x90a,.V[`\0\x80a\x12\x9D\x83` \x01Q\x84`@\x01Qa\x19\x9DV[\x90P`\0a\x12\xB3\x84` \x01Q\x85`@\x01Qa\x19\xC3V[\x90P`\0a\x12\xCE\x85`@\x01Q\x83a\x19\xF2\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x85Q\x90\x91P`\0\x90a\x12\xEB\x90a\x12\xE4\x90\x89a\x19\xF2V[\x89\x90a\x1A\x07V[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0\x81\x12a\x13\tW`\0\x94PPPPPa\x05=V[`\0\x81\x13a\x13\x1FW`\0\x19\x94PPPPPa\x05=V[`\0a\x13*\x82a\x1A\x1CV[\x90P`\0\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x13B\x88\x85a,\xC8V[a\x13L\x91\x90a-\x0EV[a\x12\r\x91\x90a-<V[`\0\x80a\x13c\x84\x84a\x1CbV[\x90P`\0a\x13p\x82a\x16\x07V[\x90P`\0a\x13}\x82a\x16yV[\x90Pa\r\xA1a\x13\x94\x82g\r\xE0\xB6\xB3\xA7d\0\0a).V[\x88\x90a\x12FV[`\0\x82\x80\x85\x83\x81\x12\x15a\x13\xDBW[`\0\x81\x12\x15a\x13\xD6Wa\x13\xC1\x83a\x03\xE9a\x03\xE8a\x16\xC2V[\x92Pa\x13\xCF\x83\x8A\x8A\x88a\x16\xF0V[\x90Pa\x13\xA9V[a\x14\x08V[`\0\x81\x13\x15a\x14\x08Wa\x13\xF3\x82a\x03\xE7a\x03\xE8a\x17\xF1V[\x91Pa\x14\x01\x82\x8A\x8A\x88a\x16\xF0V[\x90Pa\x13\xDBV[a\x10\xBF\x89\x89\x83\x88`@Q` \x01a\x14\"\x94\x93\x92\x91\x90a,MV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83\x85`\x14`@a\x1C\xA7a\x18GV[`\0\x80\x80a\x14M\x84\x86a\x12[V[\x90P`\0a\x14[\x87\x83a\x12FV[\x90P\x87a\x14qWa\x14l\x87\x87a).V[a\x14{V[a\x14{\x87\x87a)\x1BV[\x93P\x87a\x14\x91Wa\x14\x8C\x81\x86a).V[a\x14\x9BV[a\x14\x9B\x81\x86a)\x1BV[\x92PPP\x94P\x94\x92PPPV[`\0\x82\x80\x85\x83\x81\x12\x15a\x14\xE8W[`\0\x81\x12\x15a\x14\xE3Wa\x14\xCE\x82a\x03\xE7a\x03\xE8a\x17\xF1V[\x91Pa\x14\xDC\x89\x89\x84\x88a\x16\xF0V[\x90Pa\x14\xB6V[a\x15\x15V[`\0\x81\x13\x15a\x15\x15Wa\x15\0\x83a\x03\xE9a\x03\xE8a\x16\xC2V[\x92Pa\x15\x0E\x89\x89\x85\x88a\x16\xF0V[\x90Pa\x14\xE8V[a\x10\xBF\x89\x89\x83\x88`@Q` \x01a\x15/\x94\x93\x92\x91\x90a,MV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83\x85`\x14`@a\x1C\xD4a\x18GV[```\x01\x82`@Q` \x01a\x15b\x92\x91\x90a-dV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x91\x90PV[```\x05\x82`@Q` \x01a\x15b\x92\x91\x90a-\x7FV[```\x03\x83\x83`@Q` \x01a\x0F\xC2\x93\x92\x91\x90a,.V[\x80Q` \x82\x01Q`@\x83\x01Q`\0\x92\x91\x90\x83a\x15\xC2\x83\x83a\x19\x9DV[\x90P`\0a\x15\xD0\x88\x86a\x1D\x01V[\x90P`\0a\x15\xDE\x85\x85a\x19\xC3V[\x90P\x82a\x15\xEB\x82\x84a,\xA1V[a\x15\xFD\x90g\r\xE0\xB6\xB3\xA7d\0\0a,\xC8V[a\x10\xBF\x91\x90a-\x0EV[`\0\x80g\x13\xA0K\xBD\xFD\xC9\xBE\x88a\x16%g\r\xE0\xB6\xB3\xA7d\0\0\x85a,\xC8V[a\x16/\x91\x90a-\x0EV[\x90P`\0a\x16<\x82a-\xA5V[\x90P`\0a\x16I\x82a\x1D\x15V[\x90Pg\x1B\xC1mgN\xC8\0\0a\x16fg\r\xE0\xB6\xB3\xA7d\0\0\x83a,\xC8V[a\x16p\x91\x90a-\x0EV[\x95\x94PPPPPV[`\0\x80\x82\x12\x15a\x16\xBEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01RotoUint: negative`\x80\x1B`D\x82\x01R`d\x01a\x06\xF5V[P\x90V[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x16\xDAW`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[`\0\x82\x85\x10a\x17AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FtradingFunction: invalid x\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06\xF5V[`\0\x80g\r\xE0\xB6\xB3\xA7d\0\0a\x17W\x88\x87a\x1A\x07V[\x10a\x17kW`\x01`\x01`\xFF\x1B\x03\x91Pa\x17{V[a\x17xa\x11\xDC\x88\x87a\x1A\x07V[\x91P[g\r\xE0\xB6\xB3\xA7d\0\0a\x17\x9B\x87a\x17\x96\x87`\0\x01Q\x89a\x19\xF2V[a\x1A\x07V[\x10a\x17\xAEWP`\x01`\x01`\xFF\x1B\x03a\x17\xC6V[a\x17\xC3a\x11\xDC\x87a\x17\x96\x87`\0\x01Q\x89a\x19\xF2V[\x90P[`\0a\x17\xDA\x85` \x01Q\x86`@\x01Qa\x19\x9DV[\x90P\x80a\x17\xE7\x83\x85a-<V[a\x05!\x91\x90a-<V[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x18\tW`\0\x80\xFD[\x04\x92\x91PPV[`\0\x80`\0\x80\x85\x80` \x01\x90Q\x81\x01\x90a\x18*\x91\x90a-\xC1V[\x93PP\x92P\x92Pa\x18=\x83\x86\x84\x84a\x16\xF0V[\x96\x95PPPPPPV[`\0\x84\x86\x11\x15a\x18tW`@Qc0\x82\xDF\xDB`\xE1\x1B\x81R`\x04\x81\x01\x87\x90R`$\x81\x01\x86\x90R`D\x01a\x06\xF5V[`\0a\x18\x84\x88\x88\x85c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x18\x96\x89\x88\x86c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x18\xA4\x82\x84a,\xC8V[\x13\x15a\x18\xCDW`@Qc\x06\xF1\xBE]`\xE2\x1B\x81R`\x04\x81\x01\x83\x90R`$\x81\x01\x82\x90R`D\x01a\x06\xF5V[`\0a\x18\xD9\x89\x89a).V[\x90P`\0[`\x02a\x18\xEA\x8A\x8Ca)\x1BV[a\x18\xF4\x91\x90a.\x05V[\x94P`\0a\x19\x06\x8C\x87\x89c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x19\x14\x86\x83a,\xC8V[\x13a\x19!W\x85\x99Pa\x19(V[\x85\x9AP\x80\x94P[a\x192\x8B\x8Ba).V[\x92PP`\x01\x01\x87\x82\x11\x80\x15a\x19FWP\x86\x81\x10[a\x18\xDEWPPPP\x96\x95PPPPPPV[`\0\x80a\x19e\x84\x84a\x1CbV[\x90P`\0a\x19r\x82a\x16\x07V[\x90P`\0a\x19\x7F\x82a\x16yV[\x90Pa\r\xA1a\x19\x96\x82g\r\xE0\xB6\xB3\xA7d\0\0a).V[\x88\x90a\x12[V[`\0\x80a\x19\xA9\x83a\x1E\xFEV[a\x19\xB7\x90c;\x9A\xCA\0a.\x19V[\x90Pa\t\xAB\x84\x82a\x19\xF2V[`\0\x80a\x19\xE2\x83a\x19\xDC\x86g\x1B\xC1mgN\xC8\0\0a\x1F\xA2V[\x90a\x19\xF2V[\x90Pa\t\xABg\x06\xF0[Y\xD3\xB2\0\0\x82[`\0a\x04\0\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x17\xF1V[`\0a\x04\0\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x17\xF1V[`\0g\x06\xF0[Y\xD3\xB2\0\0\x82\x03a\x1A5WP`\0\x91\x90PV[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x12a\x1A]W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\x1A~W`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x1A\x8B\x83`\x02a,\xC8V[\x90P`\0a\x1A\x98\x82a\x1F\xD3V[\x90P`\0a\x1A\xAEg\x13\xA0K\xBD\xFD\xC9\xBE\x88\x83a\"LV[\x90Pa\x16p\x81a-\xA5V[`\0h\x02H\xCE6\xA7\x0C\xB2k>\x19\x82\x13a\x1A\xD4WP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a\x1B\x1BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x01a\x06\xF5V[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05`\x01`_\x1B\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1Dm\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1Dn\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[\x80Q` \x82\x01Q`@\x83\x01Q`\0\x92\x91\x90\x83a\x1C~\x83\x83a\x19\x9DV[\x90P`\0a\x1C\x8C\x88\x86a\x1D\x01V[\x90P`\0a\x1C\x9A\x85\x85a\x19\xC3V[\x90P\x82a\x15\xEB\x82\x84a-<V[`\0\x80`\0\x80\x85\x80` \x01\x90Q\x81\x01\x90a\x1C\xC1\x91\x90a-\xC1V[\x93PP\x92P\x92Pa\x18=\x85\x84\x84\x84a\x16\xF0V[`\0\x80`\0\x80\x85\x80` \x01\x90Q\x81\x01\x90a\x1C\xEE\x91\x90a-\xC1V[\x93PP\x92P\x92Pa\x18=\x83\x83\x87\x84a\x16\xF0V[`\0a\x04\0a\x1D\x10\x84\x84a\x12[V[a\"aV[`\0\x81`\0\x03a\x1D.WPg\r\xE0\xB6\xB3\xA7d\0\0\x91\x90PV[gV\x98\xEE\xF0fp\0\0\x82\x12a\x1DEWP`\0\x91\x90PV[a\x1DVgV\x98\xEE\xF0fp\0\0a-\xA5V[\x82\x13a\x1DkWPg\x1B\xC1mgN\xC8\0\0\x91\x90PV[`\0a\x1Dv\x83a$<V[\x90P`\0a\x1D\xAFg\r\xE0\xB6\xB3\xA7d\0\0a\x1D\x98\x84g\x1B\xC1mgN\xC8\0\0a\x1A\x07V[a\x1D\xAA\x90g\r\xE0\xB6\xB3\xA7d\0\0a-<V[a$sV[\x90P`\0\x80\x82a\x1E\x10\x81a\x1D\xFD\x81a\x1D\xEB\x81a\x1D\xD3\x81g\x02_\x0F\xE1\x05\xA3\x14\0a\"LV[a\x1D\xE6\x90g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19a-<V[a\"LV[a\x1D\xE6\x90g\x14\xA8EL\x19\xE1\xAC\0a-<V[a\x1D\xE6\x90g\x0F\xC1\x0E\x01W\x82w\xFF\x19a-<V[a\x1E\"\x90g\x03\xDE\xBD\x08;\x8C|\0a-<V[\x91P\x83\x90Pa\x1E\x8A\x81a\x1Ex\x81a\x1Ef\x81a\x1ET\x81a\x1EA\x81\x8Ba\"LV[a\x1D\xE6\x90g\x02\x95\xD4\0\xEA2W\xFF\x19a-<V[a\x1D\xE6\x90g\x01W\xD8\xB2\xEC\xC7\x08\0a-<V[a\x1D\xE6\x90g\x051\n\xA7\xD5!0\0a-<V[a\x1D\xE6\x90g\r\xE0\xCC=\x15a\0\0a-<V[\x91P\x81g\x11\x90\0\xAB\x10\x0F\xFC\0a\x1E\xA0\x87\x88a\"LV[a\x1E\xAC\x90`\0\x19a,\xC8V[a\x1E\xB6\x91\x90a,\xA1V[a\x1E\xC0\x91\x90a-<V[\x92PP`\0a\x1E\xCE\x83a\x1A\xB9V[\x90P`\0a\x1E\xDC\x85\x83a\"LV[\x90P`\0\x88\x12a\x1E\xECW\x80a\x05!V[a\x05!\x81g\x1B\xC1mgN\xC8\0\0a,\xA1V[`\xB5\x81`\x01`\x88\x1B\x81\x10a\x1F\x17W`@\x91\x90\x91\x1B\x90`\x80\x1C[i\x01\0\0\0\0\0\0\0\0\0\x81\x10a\x1F3W` \x91\x90\x91\x1B\x90`@\x1C[e\x01\0\0\0\0\0\x81\x10a\x1FKW`\x10\x91\x90\x91\x1B\x90` \x1C[c\x01\0\0\0\x81\x10a\x1FaW`\x08\x91\x90\x91\x1B\x90`\x10\x1C[b\x01\0\0\x01\x02`\x12\x1C\x80\x82\x04\x01`\x01\x90\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x90\x81\x90\x04\x81\x11\x90\x03\x90V[`\0a\x04\0g\r\xE0\xB6\xB3\xA7d\0\0\x83a\x1F\xBA\x86a\"aV[a\x1F\xC4\x91\x90a,\xC8V[a\x1F\xCE\x91\x90a-\x0EV[a\x1A\xB9V[`\0\x80\x82\x12\x80a\x1F\xEAWPg\x1B\xC1mgN\xC8\0\0\x82\x13[\x15a \x08W`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a )W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81g\x1B\xC1mgN\xC8\0\0\x03a QW`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x15a \\W\x91\x90PV[`\0g\r\xE0\xB6\xB3\xA7d\0\0\x83\x12a \x84Wa \x7F\x83g\x1B\xC1mgN\xC8\0\0a,\xA1V[a \x86V[\x82[\x90P`\0a \x9C\x82g\x1B\xC1mgN\xC8\0\0a$sV[\x90P\x80`\0\x03a \xBFW`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a \xCA\x82a\"aV[\x90P`\0c;\x9A\xCA\0a \xF5a \xF0a \xEAg\x1B\xC1mgN\xC8\0\0a-\xA5V[\x85a\"LV[a\x1E\xFEV[a \xFF\x91\x90a,\xC8V[\x90P`\0\x80a!\x16\x83g\x03\xC1f\\z\xAB \0a\"LV[a!(\x90g \x05\xFEO&\x8E\xA0\0a-<V[\x90P`\0a!S\x84a!A\x86f\x9F2u$b\xA0\0a\"LV[a\x1D\xE6\x90g\r\xC5R\x7Fd, \0a-<V[a!e\x90g\r\xE0\xB6\xB3\xA7d\0\0a-<V[\x90Pa!\x89g\t\xD0(\xCCo _\xFF\x19\x85a!\x7F\x85\x85a$sV[a\x1D\xE6\x91\x90a,\xA1V[\x92PPP`\0[`\x02\x81\x10\x15a\"$W`\0\x86a!\xA5\x84a\x1D\x15V[a!\xAF\x91\x90a,\xA1V[\x90P`\0a!\xBD\x84\x85a\"LV[a!\xC6\x90a-\xA5V[\x90P`\0a!\xD3\x82a\x1A\xB9V[\x90P`\0a!\xE1\x86\x85a\"LV[a!\xF3g\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x84a\"LV[a!\xFD\x91\x90a,\xA1V[\x90Pa\"\t\x84\x82a$sV[a\"\x13\x90\x87a-<V[\x95P\x84`\x01\x01\x94PPPPPa!\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x88\x12a\"AWa\"<\x82a-\xA5V[a\x05!V[P\x96\x95PPPPPPV[`\0a\x04\0\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a$\x84V[`\0\x80\x82\x13a\"\x9EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x06\xF5V[`\0``a\"\xAB\x84a$\xA3V[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[`\0`\x01`\xFF\x1B\x82\x03a$bW`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x12\x15a\x16\xBEWP\x19`\x01\x01\x90V[`\0a\x04\0\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x05\x85\x14\x17\x16a$\x9CW`\0\x80\xFD[\x05\x92\x91PPV[`\0\x80\x82\x11a$\xE0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x06\xF5V[P`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x82\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`\0\x80`@\x83\x85\x03\x12\x15a%\xB1Wa%\xB1a%KV[PP\x805\x92` \x90\x91\x015\x91PV[`\0[\x83\x81\x10\x15a%\xDBW\x81\x81\x01Q\x83\x82\x01R` \x01a%\xC3V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra%\xFC\x81` \x86\x01` \x86\x01a%\xC0V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x04\0` \x83\x01\x84a%\xE4V[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a&<Wa&<a%KV[PP\x825\x94` \x84\x015\x94P`@\x84\x015\x93``\x015\x92P\x90PV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FABI decoding: struct data too sh`D\x82\x01Rb\x1B\xDC\x9D`\xEA\x1B`d\x82\x01R`\x84\x81\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a&\xE2Wa&\xE2a&\xA9V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a'\x11Wa'\x11a&\xA9V[`@R\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a'.W`\0\x80\xFD[PV[`\0\x80`\0\x83\x85\x03`\xE0\x81\x12\x15a'JWa'Ja%KV[\x845\x93P` \x85\x015\x92P`\xA0`?\x19\x82\x01\x12\x15a'jWa'ja&XV[Pa'sa&\xBFV[`@\x85\x015\x81R``\x85\x015` \x82\x01R`\x80\x85\x015`@\x82\x01R`\xA0\x85\x015``\x82\x01R`\xC0\x85\x015a'\xA6\x81a'\x19V[`\x80\x82\x01R\x92\x95\x91\x94P\x91\x92P\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a'\xCFWa'\xCFa%KV[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[\x80\x15\x15\x81\x14a'.W`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15a(\x0CWa(\x0Ca%KV[\x835\x92P` \x84\x015a(\x1E\x81a'\xE6V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[\x84\x15\x15\x81R\x83` \x82\x01R\x82`@\x82\x01R`\x80``\x82\x01R`\0a\x18=`\x80\x83\x01\x84a%\xE4V[`\0` \x82\x84\x03\x12\x15a(kWa(ka%KV[P5\x91\x90PV[\x81Q\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x80\x83\x01Q\x90\x82\x01R``\x80\x83\x01Q\x90\x82\x01R`\x80\x80\x83\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x90\x82\x01R`\xA0\x81\x01a\x04\x03V[`\0` \x82\x84\x03\x12\x15a(\xC5Wa(\xC5a%KV[\x815a\x05=\x81a'\x19V[\x82\x81R`@` \x82\x01R`\0a\x05:`@\x83\x01\x84a%\xE4V[`\0` \x82\x84\x03\x12\x15a(\xFEWa(\xFEa%KV[PQ\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x04\x03Wa\x04\x03a)\x05V[\x81\x81\x03\x81\x81\x11\x15a\x04\x03Wa\x04\x03a)\x05V[`\x01\x80`\xA0\x1B\x03\x84\x16\x81R\x82` \x82\x01R```@\x82\x01R`\0a\x16p``\x83\x01\x84a%\xE4V[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a)\x84Wa)\x84a%KV[\x86Qa)\x8F\x81a'\xE6V[` \x88\x01Q`@\x89\x01Q``\x8A\x01Q`\x80\x8B\x01Q`\xA0\x90\x9B\x01Q\x93\x9C\x92\x9BP\x90\x99\x90\x98P\x96P\x90\x94P\x92PPPV[`\0` \x80\x83\x85\x03\x12\x15a)\xD4Wa)\xD4a%KV[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a*7W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x84\x90R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a*\x9FW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x84\x90R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x81\xFD[\x81Q\x81\x81\x11\x15a*\xB1Wa*\xB1a&\xA9V[a*\xC3`\x1F\x82\x01`\x1F\x19\x16\x85\x01a&\xE8V[\x91P\x80\x82R\x86\x84\x82\x85\x01\x01\x11\x15a+)W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x85\x90R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R`\x84\x81\xFD[a+8\x81\x85\x84\x01\x86\x86\x01a%\xC0V[P\x95\x94PPPPPV[`\0`\xA0\x82\x84\x03\x12\x15a+WWa+Wa&XV[a+_a&\xBFV[\x90P\x81Q\x81R` \x82\x01Q` \x82\x01R`@\x82\x01Q`@\x82\x01R``\x82\x01Q``\x82\x01R`\x80\x82\x01Qa+\x91\x81a'\x19V[`\x80\x82\x01R\x92\x91PPV[`\0`\xA0\x82\x84\x03\x12\x15a+\xB1Wa+\xB1a%KV[a\x04\0\x83\x83a+BV[`\0` \x82\x84\x03\x12\x15a+\xD0Wa+\xD0a%KV[\x81Qa\x05=\x81a'\x19V[`\0\x80`\0``\x84\x86\x03\x12\x15a+\xF3Wa+\xF3a%KV[\x83Q\x92P` \x84\x01Q\x91P`@\x84\x01Q\x90P\x92P\x92P\x92V[`\x06\x81\x10a,*WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x90RV[``\x81\x01a,<\x82\x86a,\x0CV[` \x82\x01\x93\x90\x93R`@\x01R\x91\x90PV[\x84\x81R` \x81\x01\x84\x90R`@\x81\x01\x83\x90Ra\x01\0\x81\x01a\x16p``\x83\x01\x84\x80Q\x82R` \x80\x82\x01Q\x90\x83\x01R`@\x80\x82\x01Q\x90\x83\x01R``\x80\x82\x01Q\x90\x83\x01R`\x80\x90\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x91\x01RV[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a,\xC1Wa,\xC1a)\x05V[P\x92\x91PPV[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15a,\xE4Wa,\xE4a)\x05V[\x81\x81\x05\x83\x14\x82\x15\x17a\x04\x03Wa\x04\x03a)\x05V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a-\x1DWa-\x1Da,\xF8V[`\x01`\xFF\x1B\x82\x14`\0\x19\x84\x14\x16\x15a-7Wa-7a)\x05V[P\x05\x90V[\x80\x82\x01\x82\x81\x12`\0\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a-\\Wa-\\a)\x05V[PP\x92\x91PPV[`@\x81\x01a-r\x82\x85a,\x0CV[\x82` \x83\x01R\x93\x92PPPV[`@\x81\x01a-\x8D\x82\x85a,\x0CV[`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16` \x91\x90\x91\x01R\x91\x90PV[`\0`\x01`\xFF\x1B\x82\x01a-\xBAWa-\xBAa)\x05V[P`\0\x03\x90V[`\0\x80`\0\x80a\x01\0\x85\x87\x03\x12\x15a-\xDBWa-\xDBa%KV[\x84Q\x93P` \x85\x01Q\x92P`@\x85\x01Q\x91Pa-\xFA\x86``\x87\x01a+BV[\x90P\x92\x95\x91\x94P\x92PV[`\0\x82a.\x14Wa.\x14a,\xF8V[P\x04\x90V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x04\x03Wa\x04\x03a)\x05V\xFETarget contract does not contain\xA2dipfsX\"\x12 6u\xAB\x1B\x04a\xCFM\xD1\x19$\xAD\xCD\xFB\x08+\x0B\xCF\xE1p\x8D\x8A\xB3\x19\x0CX\xB1\xFFP\x18\\\xB9dsolcC\0\x08\x16\x003";
    /// The bytecode of the contract.
    pub static LOGNORMALSOLVER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x01\x8FW`\x005`\xE0\x1C\x80c\x7F\x17@\x9C\x11a\x01\x05W\x80c\xCB\x1FU2\x11a\0\xC9W\x80c\xCB\x1FU2\x14a\x03\x8DW\x80c\xCE\x15;\xF4\x14a\x03\xA0W\x80c\xE9G\x16\xD5\x14a\x03\xB3W\x80c\xEE>\x8C\xFB\x14a\x03\xC6W\x80c\xF3\r7\xF2\x14a\x03\xD9W\x80c\xF9\xC2\x82\x11\x14a\x03\xECWa\x01\x8FV[\x80c\x7F\x17@\x9C\x14a\x03\tW\x80c\x81\xB5\xFA\xC2\x14a\x03\x1CW\x80c\xA8\xC6.v\x14a\x03<W\x80c\xAFNC\x7F\x14a\x03gW\x80c\xB0\x9D\x04\xE5\x14a\x03zWa\x01\x8FV[\x80c;&\x8D]\x11a\x01WW\x80c;&\x8D]\x14a\x02\x87W\x80c;M\x100\x14a\x02\x9AW\x80cN\x81\x7F\xD9\x14a\x02\xADW\x80c^\xB4\x08\xFC\x14a\x02\xC0W\x80cb7V\x9F\x14a\x02\xD3W\x80cme\"\x99\x14a\x03\x01Wa\x01\x8FV[\x80c\x04 X\n\x14a\x01\xF4W\x80c\x12\x06I\xC5\x14a\x02\x1DW\x80c\x13N\xAD\x12\x14a\x02>W\x80c\x1E\x97\x8C\xB0\x14a\x02QW\x80c9(\xFF\x97\x14a\x02dW[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x02\x07a\x02\x026`\x04a%\x9BV[a\x03\xF4V[`@Qa\x02\x14\x91\x90a&\x10V[`@Q\x80\x91\x03\x90\xF3[a\x020a\x02+6`\x04a&#V[a\x04\tV[`@Q\x90\x81R` \x01a\x02\x14V[a\x02\x07a\x02L6`\x04a'1V[a\x05-V[a\x020a\x02_6`\x04a'\xB7V[a\x05DV[a\x02wa\x02r6`\x04a'\xF4V[a\x05YV[`@Qa\x02\x14\x94\x93\x92\x91\x90a(/V[a\x02\x07a\x02\x956`\x04a%\x9BV[a\t~V[a\x020a\x02\xA86`\x04a(VV[a\t\x8AV[a\x020a\x02\xBB6`\x04a'\xB7V[a\t\xB3V[a\x020a\x02\xCE6`\x04a&#V[a\t\xC8V[a\x02\xE6a\x02\xE16`\x04a%\x9BV[a\n\xE0V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x02\x14V[a\x020`\0\x81V[a\x02\xE6a\x03\x176`\x04a%\x9BV[a\x0B:V[a\x03/a\x03*6`\x04a(VV[a\x0B\x93V[`@Qa\x02\x14\x91\x90a(rV[`\0Ta\x03O\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\x14V[a\x020a\x03u6`\x04a&#V[a\x0C\x9FV[a\x02\x07a\x03\x886`\x04a(VV[a\r\xACV[a\x02\x07a\x03\x9B6`\x04a(\xB0V[a\r\xB7V[a\x02\xE6a\x03\xAE6`\x04a(VV[a\r\xC2V[a\x02\x07a\x03\xC16`\x04a%\x9BV[a\x0FRV[a\x02\xE6a\x03\xD46`\x04a%\x9BV[a\x0F^V[a\x02\xE6a\x03\xE76`\x04a%\x9BV[a\x0F\x84V[a\x020`x\x81V[``a\x04\0\x83\x83a\x0F\xAAV[\x90P[\x92\x91PPV[`\0\x80a\x04\x1F\x84\x84a\x04\x1A\x89a\x0B\x93V[a\x0F\xD9V[`@\x80Q` \x81\x01\x88\x90R\x80\x82\x01\x83\x90R``\x80\x82\x01\x88\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x80\x82\x01\x92\x83\x90R`\0\x80Tb.RK`\xE0\x1B\x90\x94R\x93\x94P\x92\x91`\x01`\x01`\xA0\x1B\x03\x16\x90b.RK\x90a\x04|\x90\x8B\x90\x86\x90`\x84\x01a(\xD0V[` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x04\xD2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a.1\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x04\xE6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\n\x91\x90a(\xE9V[\x90Pa\x05!\x87\x87\x83\x86a\x05\x1C\x8Da\x0B\x93V[a\x10\x1BV[\x98\x97PPPPPPPPV[``a\x05:\x84\x84\x84a\x10\xCCV[\x90P[\x93\x92PPPV[`\0a\x05:\x83\x83a\x05T\x87a\x0B\x93V[a\x11=V[`\0\x80`\0``a\x05\x84`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[a\x05\xA8`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[a\x05\xB1\x89a\r\xC2V[`@\x85\x01R` \x84\x01R\x82R`\0a\x05\xC8\x8Aa\x0B\x93V[\x90P`\0\x80a\x05\xE5\x8C\x86`\0\x01Q\x87` \x01Q\x88`@\x01Qa\x0C\x9FV[\x90P\x8A\x15a\x07\x1CW`\0a\x06\x06\x84``\x01Q\x8Ca\x12F\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x86Q\x90\x91P`\0\x90a\x06\"\x90a\x06\x1C\x84\x86a\x12FV[\x90a\x12[V[\x90Pa\x06/`\x01\x82a)\x1BV[\x87Q\x90\x91Pa\x06?\x90\x8D\x90a)\x1BV[\x86Ra\x06K\x81\x84a)\x1BV[\x86`@\x01\x81\x81RPP`\0a\x06i\x8F\x88`\0\x01Q\x89`@\x01Qa\x05DV[\x90Pa\x06\x7F\x8F\x88`\0\x01Q\x89`@\x01Q\x84a\x04\tV[` \x88\x01\x81\x81R`\x01\x91a\x06\x94\x90\x83\x90a)\x1BV[\x90RP` \x80\x89\x01Q\x90\x88\x01Q\x10a\x06\xFEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: y reserve increase`D\x82\x01Rad!`\xF0\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[\x86` \x01Q\x88` \x01Qa\x07\x12\x91\x90a).V[\x94PPPPa\x08<V[`\0a\x075\x84``\x01Q\x8Ca\x12F\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x07T\x87` \x01Qa\x06\x1C\x85\x85a\x12F\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa\x07a`\x01\x82a)\x1BV[\x90P\x8B\x87` \x01Qa\x07s\x91\x90a)\x1BV[` \x87\x01Ra\x07\x82\x81\x84a)\x1BV[\x86`@\x01\x81\x81RPP`\0a\x07\xA0\x8F\x88` \x01Q\x89`@\x01Qa\t\xB3V[\x90Pa\x07\xB6\x8F\x88` \x01Q\x89`@\x01Q\x84a\t\xC8V[\x80\x88R`\x01\x90\x88\x90a\x07\xC9\x90\x83\x90a)\x1BV[\x90RP\x87Q\x87Q\x10a\x08(W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: x reserve increase`D\x82\x01Rad!`\xF0\x1B`d\x82\x01R`\x84\x01a\x06\xF5V[\x86Q\x88Qa\x086\x91\x90a).V[\x94PPPP[P\x82Q` \x80\x85\x01Q`@\x80\x87\x01Q\x81Q\x93\x84\x01\x94\x90\x94R\x82\x01R``\x81\x01\x91\x90\x91R`\0\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16ch\xBD>80\x8F\x85`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x08\xBC\x93\x92\x91\x90a)AV[`\xC0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\t\x12W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a.1\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\t&W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\tJ\x91\x90a)hV[PPPPP\x90P\x80\x83a\tf\x87`\0\x01Q\x88`@\x01Q\x88a\x11=V[\x84\x99P\x99P\x99P\x99PPPPPPP\x93P\x93P\x93P\x93V[``a\x04\0\x83\x83a\x12pV[`\0\x80`\0a\t\x98\x84a\r\xC2V[\x92PP\x91Pa\t\xAB\x82\x82a\x05T\x87a\x0B\x93V[\x94\x93PPPPV[`\0a\x05:\x83\x83a\t\xC3\x87a\x0B\x93V[a\x12\x88V[`\0\x80a\t\xDE\x84\x84a\t\xD9\x89a\x0B\x93V[a\x13VV[`@\x80Q` \x81\x01\x83\x90R\x80\x82\x01\x88\x90R``\x80\x82\x01\x88\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x80\x82\x01\x92\x83\x90R`\0\x80Tb.RK`\xE0\x1B\x90\x94R\x93\x94P\x92\x91`\x01`\x01`\xA0\x1B\x03\x16\x90b.RK\x90a\n;\x90\x8B\x90\x86\x90`\x84\x01a(\xD0V[` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\n\x91W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a.1\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\n\xA5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\xC9\x91\x90a(\xE9V[\x90Pa\x05!\x87\x87\x83\x86a\n\xDB\x8Da\x0B\x93V[a\x13\x9BV[`\0\x80`\0\x80`\0a\n\xF1\x87a\r\xC2V[\x92PP\x91P`\0\x80a\x0B\x06`\0\x89\x86\x86a\x14?V[\x91P\x91P`\0a\x0B\x17\x8A\x84\x84a\x05DV[\x90P`\0a\x0B'\x8B\x85\x85\x85a\x04\tV[\x93\x9B\x93\x9AP\x91\x98P\x91\x96PPPPPPPV[`\0\x80`\0\x80`\0a\x0BK\x87a\r\xC2V[\x92P\x92PP`\0\x80a\x0B``\x01\x89\x86\x86a\x14?V[\x91P\x91P`\0a\x0Bq\x8A\x84\x84a\t\xB3V[\x90P`\0a\x0B\x81\x8B\x85\x85\x85a\t\xC8V[\x9B\x93\x9AP\x91\x98P\x91\x96PPPPPPPV[a\x0B\xCE`@Q\x80`\xA0\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81RP\x90V[`\0T`@Qc\xDC\x17\x83U`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xDC\x17\x83U\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0CPW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a.1\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0CdW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0C\x8C\x91\x90\x81\x01\x90a)\xBEV[\x80` \x01\x90Q\x81\x01\x90a\x04\x03\x91\x90a+\x9CV[`@\x80Q` \x81\x01\x85\x90R\x80\x82\x01\x84\x90R``\x80\x82\x01\x84\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x80\x82\x01\x92\x83\x90R`\0\x80Tb.RK`\xE0\x1B\x90\x94R\x92\x90\x91\x83\x91`\x01`\x01`\xA0\x1B\x03\x16\x90b.RK\x90a\x0C\xFC\x90\x8A\x90\x86\x90`\x84\x01a(\xD0V[` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\rRW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a.1\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\rfW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\x8A\x91\x90a(\xE9V[\x90Pa\r\xA1\x86\x86\x83\x87a\r\x9C\x8Ca\x0B\x93V[a\x14\xA8V[\x97\x96PPPPPPPV[``a\x04\x03\x82a\x15LV[``a\x04\x03\x82a\x15xV[`\0\x80`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xAF\xBA\x13\xC4`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0ERW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a.1\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0EfW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\x8A\x91\x90a+\xBBV[`\x01`\x01`\xA0\x1B\x03\x16c\xCE\x15;\xF4\x85`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0E\xB7\x91\x81R` \x01\x90V[```@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0F\rW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a.1\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0F!W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0FE\x91\x90a+\xDBV[\x92P\x92P\x92P\x91\x93\x90\x92PV[``a\x04\0\x83\x83a\x15\x8EV[`\0\x80`\0\x80`\0a\x0Fo\x87a\r\xC2V[\x92PP\x91P`\0\x80a\x0B\x06`\x01\x89\x86\x86a\x14?V[`\0\x80`\0\x80`\0a\x0F\x95\x87a\r\xC2V[\x92P\x92PP`\0\x80a\x0B``\0\x89\x86\x86a\x14?V[```\x02\x83\x83`@Q` \x01a\x0F\xC2\x93\x92\x91\x90a,.V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x92\x91PPV[`\0\x80a\x0F\xE6\x84\x84a\x15\xA6V[\x90P`\0a\x0F\xF3\x82a\x16\x07V[\x90P`\0a\x10\0\x82a\x16yV[\x85Q\x90\x91Pa\r\xA1\x90\x82\x90a\x10\x15\x90\x8Aa\x12FV[\x90a\x12FV[`\0\x82\x80\x85\x83\x81\x12\x15a\x10[W[`\0\x81\x12\x15a\x10VWa\x10A\x83a\x03\xE9a\x03\xE8a\x16\xC2V[\x92Pa\x10O\x89\x84\x8A\x88a\x16\xF0V[\x90Pa\x10)V[a\x10\x88V[`\0\x81\x13\x15a\x10\x88Wa\x10s\x82a\x03\xE7a\x03\xE8a\x17\xF1V[\x91Pa\x10\x81\x89\x83\x8A\x88a\x16\xF0V[\x90Pa\x10[V[a\x10\xBF\x89\x89\x83\x88`@Q` \x01a\x10\xA2\x94\x93\x92\x91\x90a,MV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83\x85`\x14`@a\x18\x10a\x18GV[\x99\x98PPPPPPPPPV[```\0a\x10\xDB\x85\x85\x85a\x19XV[\x90P`\0a\x10\xEA\x82\x86\x86a\x0F\xD9V[\x90P`\0a\x10\xFA\x87\x83\x85\x88a\x16\xF0V[\x90Pa\x11\t\x87\x83\x83\x86\x89a\x14\xA8V[\x92P\x86\x82\x84\x87`@Q` \x01a\x11\"\x94\x93\x92\x91\x90a,MV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x93PPPP\x93\x92PPPV[`\0\x80a\x11R\x83` \x01Q\x84`@\x01Qa\x19\x9DV[\x90P`\0a\x11h\x84` \x01Q\x85`@\x01Qa\x19\xC3V[\x90P`\0a\x11\x83\x85`@\x01Q\x83a\x19\xF2\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x11\x91\x88\x88a\x1A\x07V[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0\x81\x12a\x11\xAFW`\0\x94PPPPPa\x05=V[`\0\x81\x13a\x11\xC5W`\0\x19\x94PPPPPa\x05=V[`\0a\x11\xE1a\x11\xDC\x83g\r\xE0\xB6\xB3\xA7d\0\0a,\xA1V[a\x1A\x1CV[\x90P`\0\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x11\xF9\x88\x85a,\xC8V[a\x12\x03\x91\x90a-\x0EV[a\x12\r\x91\x90a,\xA1V[\x90P`\0a\x12\x1A\x82a\x1A\xB9V[\x90P`\0a\x12'\x82a\x16yV[\x8AQ\x90\x91Pa\x126\x90\x82a\x12FV[\x9C\x9BPPPPPPPPPPPPV[`\0a\x04\0\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x16\xC2V[`\0a\x04\0\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x16\xC2V[```\x04\x83\x83`@Q` \x01a\x0F\xC2\x93\x92\x91\x90a,.V[`\0\x80a\x12\x9D\x83` \x01Q\x84`@\x01Qa\x19\x9DV[\x90P`\0a\x12\xB3\x84` \x01Q\x85`@\x01Qa\x19\xC3V[\x90P`\0a\x12\xCE\x85`@\x01Q\x83a\x19\xF2\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x85Q\x90\x91P`\0\x90a\x12\xEB\x90a\x12\xE4\x90\x89a\x19\xF2V[\x89\x90a\x1A\x07V[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0\x81\x12a\x13\tW`\0\x94PPPPPa\x05=V[`\0\x81\x13a\x13\x1FW`\0\x19\x94PPPPPa\x05=V[`\0a\x13*\x82a\x1A\x1CV[\x90P`\0\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x13B\x88\x85a,\xC8V[a\x13L\x91\x90a-\x0EV[a\x12\r\x91\x90a-<V[`\0\x80a\x13c\x84\x84a\x1CbV[\x90P`\0a\x13p\x82a\x16\x07V[\x90P`\0a\x13}\x82a\x16yV[\x90Pa\r\xA1a\x13\x94\x82g\r\xE0\xB6\xB3\xA7d\0\0a).V[\x88\x90a\x12FV[`\0\x82\x80\x85\x83\x81\x12\x15a\x13\xDBW[`\0\x81\x12\x15a\x13\xD6Wa\x13\xC1\x83a\x03\xE9a\x03\xE8a\x16\xC2V[\x92Pa\x13\xCF\x83\x8A\x8A\x88a\x16\xF0V[\x90Pa\x13\xA9V[a\x14\x08V[`\0\x81\x13\x15a\x14\x08Wa\x13\xF3\x82a\x03\xE7a\x03\xE8a\x17\xF1V[\x91Pa\x14\x01\x82\x8A\x8A\x88a\x16\xF0V[\x90Pa\x13\xDBV[a\x10\xBF\x89\x89\x83\x88`@Q` \x01a\x14\"\x94\x93\x92\x91\x90a,MV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83\x85`\x14`@a\x1C\xA7a\x18GV[`\0\x80\x80a\x14M\x84\x86a\x12[V[\x90P`\0a\x14[\x87\x83a\x12FV[\x90P\x87a\x14qWa\x14l\x87\x87a).V[a\x14{V[a\x14{\x87\x87a)\x1BV[\x93P\x87a\x14\x91Wa\x14\x8C\x81\x86a).V[a\x14\x9BV[a\x14\x9B\x81\x86a)\x1BV[\x92PPP\x94P\x94\x92PPPV[`\0\x82\x80\x85\x83\x81\x12\x15a\x14\xE8W[`\0\x81\x12\x15a\x14\xE3Wa\x14\xCE\x82a\x03\xE7a\x03\xE8a\x17\xF1V[\x91Pa\x14\xDC\x89\x89\x84\x88a\x16\xF0V[\x90Pa\x14\xB6V[a\x15\x15V[`\0\x81\x13\x15a\x15\x15Wa\x15\0\x83a\x03\xE9a\x03\xE8a\x16\xC2V[\x92Pa\x15\x0E\x89\x89\x85\x88a\x16\xF0V[\x90Pa\x14\xE8V[a\x10\xBF\x89\x89\x83\x88`@Q` \x01a\x15/\x94\x93\x92\x91\x90a,MV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83\x85`\x14`@a\x1C\xD4a\x18GV[```\x01\x82`@Q` \x01a\x15b\x92\x91\x90a-dV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x91\x90PV[```\x05\x82`@Q` \x01a\x15b\x92\x91\x90a-\x7FV[```\x03\x83\x83`@Q` \x01a\x0F\xC2\x93\x92\x91\x90a,.V[\x80Q` \x82\x01Q`@\x83\x01Q`\0\x92\x91\x90\x83a\x15\xC2\x83\x83a\x19\x9DV[\x90P`\0a\x15\xD0\x88\x86a\x1D\x01V[\x90P`\0a\x15\xDE\x85\x85a\x19\xC3V[\x90P\x82a\x15\xEB\x82\x84a,\xA1V[a\x15\xFD\x90g\r\xE0\xB6\xB3\xA7d\0\0a,\xC8V[a\x10\xBF\x91\x90a-\x0EV[`\0\x80g\x13\xA0K\xBD\xFD\xC9\xBE\x88a\x16%g\r\xE0\xB6\xB3\xA7d\0\0\x85a,\xC8V[a\x16/\x91\x90a-\x0EV[\x90P`\0a\x16<\x82a-\xA5V[\x90P`\0a\x16I\x82a\x1D\x15V[\x90Pg\x1B\xC1mgN\xC8\0\0a\x16fg\r\xE0\xB6\xB3\xA7d\0\0\x83a,\xC8V[a\x16p\x91\x90a-\x0EV[\x95\x94PPPPPV[`\0\x80\x82\x12\x15a\x16\xBEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01RotoUint: negative`\x80\x1B`D\x82\x01R`d\x01a\x06\xF5V[P\x90V[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x16\xDAW`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[`\0\x82\x85\x10a\x17AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FtradingFunction: invalid x\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06\xF5V[`\0\x80g\r\xE0\xB6\xB3\xA7d\0\0a\x17W\x88\x87a\x1A\x07V[\x10a\x17kW`\x01`\x01`\xFF\x1B\x03\x91Pa\x17{V[a\x17xa\x11\xDC\x88\x87a\x1A\x07V[\x91P[g\r\xE0\xB6\xB3\xA7d\0\0a\x17\x9B\x87a\x17\x96\x87`\0\x01Q\x89a\x19\xF2V[a\x1A\x07V[\x10a\x17\xAEWP`\x01`\x01`\xFF\x1B\x03a\x17\xC6V[a\x17\xC3a\x11\xDC\x87a\x17\x96\x87`\0\x01Q\x89a\x19\xF2V[\x90P[`\0a\x17\xDA\x85` \x01Q\x86`@\x01Qa\x19\x9DV[\x90P\x80a\x17\xE7\x83\x85a-<V[a\x05!\x91\x90a-<V[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x18\tW`\0\x80\xFD[\x04\x92\x91PPV[`\0\x80`\0\x80\x85\x80` \x01\x90Q\x81\x01\x90a\x18*\x91\x90a-\xC1V[\x93PP\x92P\x92Pa\x18=\x83\x86\x84\x84a\x16\xF0V[\x96\x95PPPPPPV[`\0\x84\x86\x11\x15a\x18tW`@Qc0\x82\xDF\xDB`\xE1\x1B\x81R`\x04\x81\x01\x87\x90R`$\x81\x01\x86\x90R`D\x01a\x06\xF5V[`\0a\x18\x84\x88\x88\x85c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x18\x96\x89\x88\x86c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x18\xA4\x82\x84a,\xC8V[\x13\x15a\x18\xCDW`@Qc\x06\xF1\xBE]`\xE2\x1B\x81R`\x04\x81\x01\x83\x90R`$\x81\x01\x82\x90R`D\x01a\x06\xF5V[`\0a\x18\xD9\x89\x89a).V[\x90P`\0[`\x02a\x18\xEA\x8A\x8Ca)\x1BV[a\x18\xF4\x91\x90a.\x05V[\x94P`\0a\x19\x06\x8C\x87\x89c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x19\x14\x86\x83a,\xC8V[\x13a\x19!W\x85\x99Pa\x19(V[\x85\x9AP\x80\x94P[a\x192\x8B\x8Ba).V[\x92PP`\x01\x01\x87\x82\x11\x80\x15a\x19FWP\x86\x81\x10[a\x18\xDEWPPPP\x96\x95PPPPPPV[`\0\x80a\x19e\x84\x84a\x1CbV[\x90P`\0a\x19r\x82a\x16\x07V[\x90P`\0a\x19\x7F\x82a\x16yV[\x90Pa\r\xA1a\x19\x96\x82g\r\xE0\xB6\xB3\xA7d\0\0a).V[\x88\x90a\x12[V[`\0\x80a\x19\xA9\x83a\x1E\xFEV[a\x19\xB7\x90c;\x9A\xCA\0a.\x19V[\x90Pa\t\xAB\x84\x82a\x19\xF2V[`\0\x80a\x19\xE2\x83a\x19\xDC\x86g\x1B\xC1mgN\xC8\0\0a\x1F\xA2V[\x90a\x19\xF2V[\x90Pa\t\xABg\x06\xF0[Y\xD3\xB2\0\0\x82[`\0a\x04\0\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x17\xF1V[`\0a\x04\0\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x17\xF1V[`\0g\x06\xF0[Y\xD3\xB2\0\0\x82\x03a\x1A5WP`\0\x91\x90PV[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x12a\x1A]W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\x1A~W`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x1A\x8B\x83`\x02a,\xC8V[\x90P`\0a\x1A\x98\x82a\x1F\xD3V[\x90P`\0a\x1A\xAEg\x13\xA0K\xBD\xFD\xC9\xBE\x88\x83a\"LV[\x90Pa\x16p\x81a-\xA5V[`\0h\x02H\xCE6\xA7\x0C\xB2k>\x19\x82\x13a\x1A\xD4WP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a\x1B\x1BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x01a\x06\xF5V[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05`\x01`_\x1B\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1Dm\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1Dn\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[\x80Q` \x82\x01Q`@\x83\x01Q`\0\x92\x91\x90\x83a\x1C~\x83\x83a\x19\x9DV[\x90P`\0a\x1C\x8C\x88\x86a\x1D\x01V[\x90P`\0a\x1C\x9A\x85\x85a\x19\xC3V[\x90P\x82a\x15\xEB\x82\x84a-<V[`\0\x80`\0\x80\x85\x80` \x01\x90Q\x81\x01\x90a\x1C\xC1\x91\x90a-\xC1V[\x93PP\x92P\x92Pa\x18=\x85\x84\x84\x84a\x16\xF0V[`\0\x80`\0\x80\x85\x80` \x01\x90Q\x81\x01\x90a\x1C\xEE\x91\x90a-\xC1V[\x93PP\x92P\x92Pa\x18=\x83\x83\x87\x84a\x16\xF0V[`\0a\x04\0a\x1D\x10\x84\x84a\x12[V[a\"aV[`\0\x81`\0\x03a\x1D.WPg\r\xE0\xB6\xB3\xA7d\0\0\x91\x90PV[gV\x98\xEE\xF0fp\0\0\x82\x12a\x1DEWP`\0\x91\x90PV[a\x1DVgV\x98\xEE\xF0fp\0\0a-\xA5V[\x82\x13a\x1DkWPg\x1B\xC1mgN\xC8\0\0\x91\x90PV[`\0a\x1Dv\x83a$<V[\x90P`\0a\x1D\xAFg\r\xE0\xB6\xB3\xA7d\0\0a\x1D\x98\x84g\x1B\xC1mgN\xC8\0\0a\x1A\x07V[a\x1D\xAA\x90g\r\xE0\xB6\xB3\xA7d\0\0a-<V[a$sV[\x90P`\0\x80\x82a\x1E\x10\x81a\x1D\xFD\x81a\x1D\xEB\x81a\x1D\xD3\x81g\x02_\x0F\xE1\x05\xA3\x14\0a\"LV[a\x1D\xE6\x90g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19a-<V[a\"LV[a\x1D\xE6\x90g\x14\xA8EL\x19\xE1\xAC\0a-<V[a\x1D\xE6\x90g\x0F\xC1\x0E\x01W\x82w\xFF\x19a-<V[a\x1E\"\x90g\x03\xDE\xBD\x08;\x8C|\0a-<V[\x91P\x83\x90Pa\x1E\x8A\x81a\x1Ex\x81a\x1Ef\x81a\x1ET\x81a\x1EA\x81\x8Ba\"LV[a\x1D\xE6\x90g\x02\x95\xD4\0\xEA2W\xFF\x19a-<V[a\x1D\xE6\x90g\x01W\xD8\xB2\xEC\xC7\x08\0a-<V[a\x1D\xE6\x90g\x051\n\xA7\xD5!0\0a-<V[a\x1D\xE6\x90g\r\xE0\xCC=\x15a\0\0a-<V[\x91P\x81g\x11\x90\0\xAB\x10\x0F\xFC\0a\x1E\xA0\x87\x88a\"LV[a\x1E\xAC\x90`\0\x19a,\xC8V[a\x1E\xB6\x91\x90a,\xA1V[a\x1E\xC0\x91\x90a-<V[\x92PP`\0a\x1E\xCE\x83a\x1A\xB9V[\x90P`\0a\x1E\xDC\x85\x83a\"LV[\x90P`\0\x88\x12a\x1E\xECW\x80a\x05!V[a\x05!\x81g\x1B\xC1mgN\xC8\0\0a,\xA1V[`\xB5\x81`\x01`\x88\x1B\x81\x10a\x1F\x17W`@\x91\x90\x91\x1B\x90`\x80\x1C[i\x01\0\0\0\0\0\0\0\0\0\x81\x10a\x1F3W` \x91\x90\x91\x1B\x90`@\x1C[e\x01\0\0\0\0\0\x81\x10a\x1FKW`\x10\x91\x90\x91\x1B\x90` \x1C[c\x01\0\0\0\x81\x10a\x1FaW`\x08\x91\x90\x91\x1B\x90`\x10\x1C[b\x01\0\0\x01\x02`\x12\x1C\x80\x82\x04\x01`\x01\x90\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x90\x81\x90\x04\x81\x11\x90\x03\x90V[`\0a\x04\0g\r\xE0\xB6\xB3\xA7d\0\0\x83a\x1F\xBA\x86a\"aV[a\x1F\xC4\x91\x90a,\xC8V[a\x1F\xCE\x91\x90a-\x0EV[a\x1A\xB9V[`\0\x80\x82\x12\x80a\x1F\xEAWPg\x1B\xC1mgN\xC8\0\0\x82\x13[\x15a \x08W`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a )W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81g\x1B\xC1mgN\xC8\0\0\x03a QW`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x15a \\W\x91\x90PV[`\0g\r\xE0\xB6\xB3\xA7d\0\0\x83\x12a \x84Wa \x7F\x83g\x1B\xC1mgN\xC8\0\0a,\xA1V[a \x86V[\x82[\x90P`\0a \x9C\x82g\x1B\xC1mgN\xC8\0\0a$sV[\x90P\x80`\0\x03a \xBFW`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a \xCA\x82a\"aV[\x90P`\0c;\x9A\xCA\0a \xF5a \xF0a \xEAg\x1B\xC1mgN\xC8\0\0a-\xA5V[\x85a\"LV[a\x1E\xFEV[a \xFF\x91\x90a,\xC8V[\x90P`\0\x80a!\x16\x83g\x03\xC1f\\z\xAB \0a\"LV[a!(\x90g \x05\xFEO&\x8E\xA0\0a-<V[\x90P`\0a!S\x84a!A\x86f\x9F2u$b\xA0\0a\"LV[a\x1D\xE6\x90g\r\xC5R\x7Fd, \0a-<V[a!e\x90g\r\xE0\xB6\xB3\xA7d\0\0a-<V[\x90Pa!\x89g\t\xD0(\xCCo _\xFF\x19\x85a!\x7F\x85\x85a$sV[a\x1D\xE6\x91\x90a,\xA1V[\x92PPP`\0[`\x02\x81\x10\x15a\"$W`\0\x86a!\xA5\x84a\x1D\x15V[a!\xAF\x91\x90a,\xA1V[\x90P`\0a!\xBD\x84\x85a\"LV[a!\xC6\x90a-\xA5V[\x90P`\0a!\xD3\x82a\x1A\xB9V[\x90P`\0a!\xE1\x86\x85a\"LV[a!\xF3g\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x84a\"LV[a!\xFD\x91\x90a,\xA1V[\x90Pa\"\t\x84\x82a$sV[a\"\x13\x90\x87a-<V[\x95P\x84`\x01\x01\x94PPPPPa!\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x88\x12a\"AWa\"<\x82a-\xA5V[a\x05!V[P\x96\x95PPPPPPV[`\0a\x04\0\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a$\x84V[`\0\x80\x82\x13a\"\x9EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x06\xF5V[`\0``a\"\xAB\x84a$\xA3V[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[`\0`\x01`\xFF\x1B\x82\x03a$bW`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x12\x15a\x16\xBEWP\x19`\x01\x01\x90V[`\0a\x04\0\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x05\x85\x14\x17\x16a$\x9CW`\0\x80\xFD[\x05\x92\x91PPV[`\0\x80\x82\x11a$\xE0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x06\xF5V[P`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x82\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`\0\x80`@\x83\x85\x03\x12\x15a%\xB1Wa%\xB1a%KV[PP\x805\x92` \x90\x91\x015\x91PV[`\0[\x83\x81\x10\x15a%\xDBW\x81\x81\x01Q\x83\x82\x01R` \x01a%\xC3V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra%\xFC\x81` \x86\x01` \x86\x01a%\xC0V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x04\0` \x83\x01\x84a%\xE4V[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a&<Wa&<a%KV[PP\x825\x94` \x84\x015\x94P`@\x84\x015\x93``\x015\x92P\x90PV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FABI decoding: struct data too sh`D\x82\x01Rb\x1B\xDC\x9D`\xEA\x1B`d\x82\x01R`\x84\x81\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a&\xE2Wa&\xE2a&\xA9V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a'\x11Wa'\x11a&\xA9V[`@R\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a'.W`\0\x80\xFD[PV[`\0\x80`\0\x83\x85\x03`\xE0\x81\x12\x15a'JWa'Ja%KV[\x845\x93P` \x85\x015\x92P`\xA0`?\x19\x82\x01\x12\x15a'jWa'ja&XV[Pa'sa&\xBFV[`@\x85\x015\x81R``\x85\x015` \x82\x01R`\x80\x85\x015`@\x82\x01R`\xA0\x85\x015``\x82\x01R`\xC0\x85\x015a'\xA6\x81a'\x19V[`\x80\x82\x01R\x92\x95\x91\x94P\x91\x92P\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a'\xCFWa'\xCFa%KV[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[\x80\x15\x15\x81\x14a'.W`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15a(\x0CWa(\x0Ca%KV[\x835\x92P` \x84\x015a(\x1E\x81a'\xE6V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[\x84\x15\x15\x81R\x83` \x82\x01R\x82`@\x82\x01R`\x80``\x82\x01R`\0a\x18=`\x80\x83\x01\x84a%\xE4V[`\0` \x82\x84\x03\x12\x15a(kWa(ka%KV[P5\x91\x90PV[\x81Q\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x80\x83\x01Q\x90\x82\x01R``\x80\x83\x01Q\x90\x82\x01R`\x80\x80\x83\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x90\x82\x01R`\xA0\x81\x01a\x04\x03V[`\0` \x82\x84\x03\x12\x15a(\xC5Wa(\xC5a%KV[\x815a\x05=\x81a'\x19V[\x82\x81R`@` \x82\x01R`\0a\x05:`@\x83\x01\x84a%\xE4V[`\0` \x82\x84\x03\x12\x15a(\xFEWa(\xFEa%KV[PQ\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x04\x03Wa\x04\x03a)\x05V[\x81\x81\x03\x81\x81\x11\x15a\x04\x03Wa\x04\x03a)\x05V[`\x01\x80`\xA0\x1B\x03\x84\x16\x81R\x82` \x82\x01R```@\x82\x01R`\0a\x16p``\x83\x01\x84a%\xE4V[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a)\x84Wa)\x84a%KV[\x86Qa)\x8F\x81a'\xE6V[` \x88\x01Q`@\x89\x01Q``\x8A\x01Q`\x80\x8B\x01Q`\xA0\x90\x9B\x01Q\x93\x9C\x92\x9BP\x90\x99\x90\x98P\x96P\x90\x94P\x92PPPV[`\0` \x80\x83\x85\x03\x12\x15a)\xD4Wa)\xD4a%KV[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a*7W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x84\x90R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a*\x9FW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x84\x90R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x81\xFD[\x81Q\x81\x81\x11\x15a*\xB1Wa*\xB1a&\xA9V[a*\xC3`\x1F\x82\x01`\x1F\x19\x16\x85\x01a&\xE8V[\x91P\x80\x82R\x86\x84\x82\x85\x01\x01\x11\x15a+)W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x85\x90R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R`\x84\x81\xFD[a+8\x81\x85\x84\x01\x86\x86\x01a%\xC0V[P\x95\x94PPPPPV[`\0`\xA0\x82\x84\x03\x12\x15a+WWa+Wa&XV[a+_a&\xBFV[\x90P\x81Q\x81R` \x82\x01Q` \x82\x01R`@\x82\x01Q`@\x82\x01R``\x82\x01Q``\x82\x01R`\x80\x82\x01Qa+\x91\x81a'\x19V[`\x80\x82\x01R\x92\x91PPV[`\0`\xA0\x82\x84\x03\x12\x15a+\xB1Wa+\xB1a%KV[a\x04\0\x83\x83a+BV[`\0` \x82\x84\x03\x12\x15a+\xD0Wa+\xD0a%KV[\x81Qa\x05=\x81a'\x19V[`\0\x80`\0``\x84\x86\x03\x12\x15a+\xF3Wa+\xF3a%KV[\x83Q\x92P` \x84\x01Q\x91P`@\x84\x01Q\x90P\x92P\x92P\x92V[`\x06\x81\x10a,*WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x90RV[``\x81\x01a,<\x82\x86a,\x0CV[` \x82\x01\x93\x90\x93R`@\x01R\x91\x90PV[\x84\x81R` \x81\x01\x84\x90R`@\x81\x01\x83\x90Ra\x01\0\x81\x01a\x16p``\x83\x01\x84\x80Q\x82R` \x80\x82\x01Q\x90\x83\x01R`@\x80\x82\x01Q\x90\x83\x01R``\x80\x82\x01Q\x90\x83\x01R`\x80\x90\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x91\x01RV[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a,\xC1Wa,\xC1a)\x05V[P\x92\x91PPV[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15a,\xE4Wa,\xE4a)\x05V[\x81\x81\x05\x83\x14\x82\x15\x17a\x04\x03Wa\x04\x03a)\x05V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a-\x1DWa-\x1Da,\xF8V[`\x01`\xFF\x1B\x82\x14`\0\x19\x84\x14\x16\x15a-7Wa-7a)\x05V[P\x05\x90V[\x80\x82\x01\x82\x81\x12`\0\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a-\\Wa-\\a)\x05V[PP\x92\x91PPV[`@\x81\x01a-r\x82\x85a,\x0CV[\x82` \x83\x01R\x93\x92PPPV[`@\x81\x01a-\x8D\x82\x85a,\x0CV[`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16` \x91\x90\x91\x01R\x91\x90PV[`\0`\x01`\xFF\x1B\x82\x01a-\xBAWa-\xBAa)\x05V[P`\0\x03\x90V[`\0\x80`\0\x80a\x01\0\x85\x87\x03\x12\x15a-\xDBWa-\xDBa%KV[\x84Q\x93P` \x85\x01Q\x92P`@\x85\x01Q\x91Pa-\xFA\x86``\x87\x01a+BV[\x90P\x92\x95\x91\x94P\x92PV[`\0\x82a.\x14Wa.\x14a,\xF8V[P\x04\x90V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x04\x03Wa\x04\x03a)\x05V\xFETarget contract does not contain\xA2dipfsX\"\x12 6u\xAB\x1B\x04a\xCFM\xD1\x19$\xAD\xCD\xFB\x08+\x0B\xCF\xE1p\x8D\x8A\xB3\x19\x0CX\xB1\xFFP\x18\\\xB9dsolcC\0\x08\x16\x003";
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
