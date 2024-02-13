pub use g3m_solver::*;
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
pub mod g3m_solver {
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
                                        ::std::borrow::ToOwned::to_owned("struct G3M.G3MParams"),
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
                                        ::std::borrow::ToOwned::to_owned("struct G3M.G3MParams"),
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
    pub static G3MSOLVER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80\x80`@R4a\0\xE6W`@Q`\x1Fa%\x1E8\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x91`\x01`\x01`@\x1B\x03\x83\x11\x84\x84\x10\x17a\0\xD0W\x80\x84\x92` \x94`@R\x839\x81\x01\x03\x12a\0\x80WQ`\x01`\x01`\xA0\x1B\x03\x81\x16\x90\x81\x90\x03a\0{W`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x91\x90\x91\x17\x90U`@Qa#\xEA\x90\x81a\x014\x829\xF3[`\0\x80\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[bF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01Ra7\xB7`\xF1\x1B`d\x82\x01R`\x84\x90\xFD\xFE`\x80`@R`\x046\x10a\x0E\xB3W`\x005`\xE0\x1C\x80c\x0FAf\xB8\x14a\x01\\W\x80c%\th\xD9\x14a\x01WW\x80c0m\xB4k\x14a\x01RW\x80c3\"f\xF3\x14a\x01MW\x80c9(\xFF\x97\x14a\x01HW\x80c;M\x100\x14a\x01CW\x80cO\xD6|X\x14a\x01>W\x80cZ\x93\xB8\xCE\x14a\x019W\x80cb7V\x9F\x14a\x014W\x80c\x7F\x17@\x9C\x14a\x01/W\x80c\x81\xB5\xFA\xC2\x14a\x01*W\x80c\x90.\xCA\xA2\x14a\x01%W\x80c\xA8\xC6.v\x14a\x01 W\x80c\xB0\x9D\x04\xE5\x14a\x01\x1BW\x80c\xCB\x1FU2\x14a\x01\x16W\x80c\xCE\x15;\xF4\x14a\x01\x11W\x80c\xDE\xF1_\x92\x14a\x01\x0CW\x80c\xEC)\xD8\xE6\x14a\x01\x07W\x80c\xEE>\x8C\xFB\x14a\x01\x02W\x80c\xF2\xDEz{\x14a\0\xFDWc\xF3\r7\xF2\x03a\x0E\xB3Wa\x0E\x80V[a\x0EdV[a\x0E0V[a\x0E\x1AV[a\r\xA9V[a\x0C\x8BV[a\x0CFV[a\x0C\x02V[a\x0B\xD9V[a\x0B\xB0V[a\x0B\\V[a\n\xFCV[a\n\x9BV[a\nvV[a\t\x8FV[a\t]V[a\x06\xC9V[a\x06dV[a\x04\xA3V[a\x04:V[a\x02\xAAV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01Ra7\xB7`\xF1\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x90\xFD[4a\x03\xD9W`@6`\x03\x19\x01\x12a\x03\xD4Wg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`$5\x81\x81\x11a\x03\xCFW6`#\x82\x01\x12\x15a\x03\xCAW\x80`\x04\x015\x91\x82\x11a\x03qW6`$\x83\x83\x01\x01\x11a\x03\x18Wa\x03\x14\x91`$a\x03\x04\x92\x01`\x045a\x0F\x16V[`@Q\x90\x81R\x90\x81\x90` \x82\x01\x90V[\x03\x90\xF3[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rjrray stride`\xA8\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x0ENL/$\r\x8C\xAD\xCC\xEE\x8D`\xAB\x1B`d\x82\x01R`\x84\x90\xFD[a\x02QV[a\x02\x01V[a\x01\xB1V[a\x01aV[`\0[\x83\x81\x10a\x03\xF1WPP`\0\x91\x01RV[\x81\x81\x01Q\x83\x82\x01R` \x01a\x03\xE1V[\x90` \x91a\x04\x1A\x81Q\x80\x92\x81\x85R\x85\x80\x86\x01\x91\x01a\x03\xDEV[`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[\x90` a\x047\x92\x81\x81R\x01\x90a\x04\x01V[\x90V[4a\x03\xD9W`@6`\x03\x19\x01\x12a\x03\xD4Wa\x03\x14`@Q`\x02` \x82\x01R`\x045`@\x82\x01R`$5``\x82\x01R``\x81Ra\x04u\x81a\r.V[`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x04\x01V[``\x90`\x03\x19\x01\x12a\x03\xD4W`\x045\x90`$5\x90`D5\x90V[4a\x03\xD9Wa\x053a\x05Aa\x04\xB76a\x04\x89V[\x92\x90a\x04\xCBa\x04\xC5\x84a\x12iV[\x93a\x13CV[a\x04\xD9\x87\x98\x93\x96\x92\x98a\x1D\xA6V[\x97`@Q\x96\x87\x95` \x87\x01\x90\x81R` \x80\x82\x01\x92\x90\x92R`@\x80\x82\x01\x93\x90\x93R``\x80\x82\x01\x94\x90\x94R\x84Q`\x80\x82\x01R\x90\x84\x01Q`\xA0\x82\x01R\x90\x83\x01Q`\xC0\x82\x01R\x91\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\xE0\x82\x01Ra\x01\0\x01\x90V[\x03`\x1F\x19\x81\x01\x83R\x82a\r\x87V[`\0\x92\x82\x91\x81\x83\x81\x11a\x06CWa\x05X\x81\x83a#>V[\x94a\x05c\x81\x84a#>V[\x87a\x05n\x82\x89a\x16\xA5V[\x13a\x06\"WP\x90a\x05\x81\x91\x95\x94\x95a\x10\x15V[\x92\x85`\x01\x94\x85`\x01\x90[a\x05\x9BW[` \x89`@Q\x90\x81R\xF3[\x15a\x05\xFDW[P\x84\x95\x96P`\0\x90a\x05\xBCa\x05\xB6\x89\x87a\x0F\x9BV[`\x01\x1C\x90V[\x97a\x05\xC7\x89\x86a#>V[\x90\x83a\x05\xD3\x86\x84a\x16\xA5V[\x13a\x05\xF1WPP\x87\x91[\x86a\x05\xE8\x87\x85a\x10\x15V[\x92\x01\x92\x97a\x05\x8BV[\x89\x96P\x90\x93P\x91a\x05\xDDV[\x85\x10\x80a\x06\x17W[\x15a\x06\x10W\x86a\x05\xA1V[\x80\x80a\x05\x90V[Pa\x01\0\x81\x10a\x06\x05V[`@Qc\x06\xF1\xBE]`\xE2\x1B\x81R`\x04\x81\x01\x88\x90R`$\x81\x01\x91\x90\x91R`D\x90\xFD[`@Qc0\x82\xDF\xDB`\xE1\x1B\x81R`\x04\x81\x01\x91\x90\x91R`$\x81\x01\x85\x90R`D\x90\xFD[4a\x03\xD9W` a\x06\x8Da\x06w6a\x04\x89V[\x90a\x06\x84a\x04\xC5\x84a\x12iV[\x92\x91\x90\x91a\x14\xEDV[`@Q\x90\x81R\xF3[\x80\x15\x15\x03a\x06\x9FWV[`\0\x80\xFD[\x90\x92`\x80\x92a\x047\x95\x94\x15\x15\x83R` \x83\x01R`@\x82\x01R\x81``\x82\x01R\x01\x90a\x04\x01V[4a\x03\xD9W``6`\x03\x19\x01\x12a\x03\xD4W`\x045`$5a\x06\xE9\x81a\x06\x95V[`D5\x91a\x06\xF5a\x0FBV[\x91a\x06\xFEa\x0FBV[\x92a\x07\x08\x83a\x13CV[\x92\x90\x91\x93` \x94\x85\x83\x01\x93`@\x95\x86\x85\x01R\x84R\x82Ra\x07'\x86a\x12iV[\x97\x85\x89\x86a\x07A\x86Q\x88Qa\x07;\x8Da\x12iV[\x91a\x16\x86V[\x94\x15a\x08\xD7Wa\x07\x9F\x93a\x07\x91a\x07\x8Ca\x07\xD3\x99\x98\x95a\x07\x86\x86a\x07oa\x07\x98\x97a\x07\xAC\x9C\x99\x01Q\x87a\x1F\x87V[\x92a\x07}\x8DQ\x8BQ\x90a\x1F\xB3V[\x91\x01Q\x90a\x16\xC8V[\x90a\x1F\x87V[a\x0F\x88V[\x93Qa\x0F\x9BV[\x8ARa\x0F\x9BV[\x80\x85\x89\x01R\x87Q\x87a\x14<V[\x90a\x07\xCAa\x07\xBF\x86\x89\x01\x93\x80\x85Ra\x0F\x88V[\x80\x84R\x82Q\x11a\x10\"V[Q\x90Q\x90a\x10\x15V[\x94[\x84Q\x83\x86\x01\x80Q\x84\x88\x01Q\x85Q\x96\x87\x01\x93\x84R` \x84\x01\x91\x90\x91R`@\x83\x01R\x90a\x08\r\x90\x85\x90``\x01\x03`\x1F\x19\x81\x01\x86R\x85a\r\x87V[`\0Ta\x080\x90a\x08$\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x80;\x15a\x08\xD2W\x84`\xC0\x91a\x08[\x97\x86Q\x80\x99\x81\x94\x82\x93c\r\x17\xA7\xC7`\xE3\x1B\x84R0`\x04\x85\x01a\x11\x03V[\x03\x91Z\xFA\x94\x85\x15a\x08\xCDW`\0\x95a\x08\x8DW[P\x90a\x08\x82\x91a\x03\x14\x95\x96Q\x90Q\x90a\x18\x88V[\x90Q\x94\x85\x94\x85a\x06\xA4V[a\x03\x14\x95P\x90a\x08\xB8a\x08\x82\x93\x92`\xC0=`\xC0\x11a\x08\xC6W[a\x08\xB0\x81\x83a\r\x87V[\x81\x01\x90a\x10\xCCV[PPPPP\x95P\x90\x91a\x08nV[P=a\x08\xA6V[a\x11'V[a\x10yV[a\t;\x92Pa\tW\x96a\t\"a\tC\x95a\t\x1Ba\x07\x8C\x8Aa\x07\x86\x88a\t\x13a\t\ta\t.\x9Aa\tN\x9F\x9C\x01Q\x88a\x1F\x87V[\x93Q\x89Q\x90a\x1F\xB3V[\x90Q\x90a\x16\xC8V[\x92Qa\x0F\x9BV[\x92\x8A\x8D\x01\x93\x84Ra\x0F\x9BV[\x90\x81\x88\x8C\x01RQ\x89a\x113V[\x80\x89Ra\x0F\x88V[\x80\x88R\x82Q\x11a\x0F\xA8V[Q\x85Q\x90a\x10\x15V[\x94a\x07\xD5V[4a\x03\xD9W` 6`\x03\x19\x01\x12a\x03\xD4W` a\x06\x8D`\x045a\t\x88a\t\x82\x82a\x12iV[\x91a\x13CV[P\x90a\x18\x88V[4a\x03\xD9Wa\x053a\t\xA3a\x04\xB76a\x04\x89V[`\0\x92\x82\x91\x81\x83\x81\x11a\x06CWa\t\xBA\x81\x83a#_V[\x94a\t\xC5\x81\x84a#_V[\x87a\t\xD0\x82\x89a\x16\xA5V[\x13a\x06\"WP\x90a\t\xE3\x91\x95\x94\x95a\x10\x15V[\x92\x85`\x01\x94\x85`\x01\x90[a\t\xFCW` \x89`@Q\x90\x81R\xF3[\x15a\nXW[P\x84\x95\x96P`\0\x90a\n\x17a\x05\xB6\x89\x87a\x0F\x9BV[\x97a\n\"\x89\x86a#_V[\x90\x83a\n.\x86\x84a\x16\xA5V[\x13a\nLWPP\x87\x91[\x86a\nC\x87\x85a\x10\x15V[\x92\x01\x92\x97a\t\xEDV[\x89\x96P\x90\x93P\x91a\n8V[\x85\x10\x80a\nkW[\x15a\x06\x10W\x86a\n\x02V[Pa\x01\0\x81\x10a\n`V[4a\x03\xD9W` a\x06\x8Da\n\x95a\n\x8C6a\x04\x89V[\x91\x92\x90\x92a\x12iV[\x91a\x18\xADV[4a\x03\xD9W`@6`\x03\x19\x01\x12a\x03\xD4Wa\n\xD7`\x045a\x03\x14a\n\xDEa\n\xCFa\n\xC4\x84a\x13CV[\x91\x90P`$5a\x18\xDAV[\x94\x90\x93a\x12iV[\x84\x84a\x1D\x85V[\x92`@Q\x93\x84\x93\x84`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[4a\x03\xD9W`@6`\x03\x19\x01\x12a\x03\xD4W`\x045a\x0B7a\x03\x14a\x0B>a\x0B/a\x0B%\x85a\x13CV[\x91P`$5a\x19\x07V[\x93\x90\x94a\x12iV[\x83\x85a\x18\xADV[\x91`@Q\x93\x84\x93\x84`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[4a\x03\xD9W` 6`\x03\x19\x01\x12a\x03\xD4W`\x80a\x0Bz`\x045a\x12iV[a\x0B\xAE`@Q\x80\x92``\x90\x80Q\x83R` \x81\x01Q` \x84\x01R`@\x81\x01Q`@\x84\x01R\x81`\x01\x80`\xA0\x1B\x03\x91\x01Q\x16\x91\x01RV[\xF3[4a\x03\xD9W` a\x06\x8Da\x0B\xC36a\x04\x89V[\x90a\x0B\xD0a\x04\xC5\x84a\x12iV[\x92\x91\x90\x91a\x1A\x13V[4a\x03\xD9W`\x006`\x03\x19\x01\x12a\x03\xD4W`\0T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x03\xD9W` 6`\x03\x19\x01\x12a\x03\xD4Wa\x03\x14`@Q`\x01` \x82\x01R`\x045`@\x82\x01R`@\x81Ra\x04u\x81a\rOV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\x06\x9FWV[4a\x03\xD9W` 6`\x03\x19\x01\x12a\x03\xD4Wa\x03\x14`\x045a\x0Cf\x81a\x0C5V[`@\x80Q`\x03` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x82\x82\x01R\x81Ra\x04u\x81a\rOV[4a\x03\xD9W` 6`\x03\x19\x01\x12a\x03\xD4Wa\x03\x14a\x0C\xAA`\x045a\x13CV[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R\x90\x81\x90``\x82\x01\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FABI decoding: struct data too sh`D\x82\x01Rb\x1B\xDC\x9D`\xEA\x1B`d\x82\x01R`\x84\x90\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\x80\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\rJW`@RV[a\r\x18V[``\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\rJW`@RV[`@\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\rJW`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\rJW`@RV[4a\x03\xD9W`\xC06`\x03\x19\x01\x12a\x03\xD4W`\x806`C\x19\x01\x12a\x0E\x15Wa\x03\x14a\x0E\t`@Qa\r\xD8\x81a\r.V[`D5\x81R`d5` \x82\x01R`\x845`@\x82\x01R`\xA45a\r\xF9\x81a\x0C5V[``\x82\x01R`$5`\x045a\x1CEV[`@Q\x91\x82\x91\x82a\x04&V[a\x0C\xC7V[4a\x03\xD9W` a\x06\x8Da\x07;a\n\x8C6a\x04\x89V[4a\x03\xD9W`@6`\x03\x19\x01\x12a\x03\xD4Wa\n\xD7`\x045a\x03\x14a\n\xDEa\n\xCFa\x0EY\x84a\x13CV[\x91\x90P`$5a\x19\x07V[4a\x03\xD9W` a\x06\x8Da\x0Eza\n\x8C6a\x04\x89V[\x91a\x1D\x85V[4a\x03\xD9W`@6`\x03\x19\x01\x12a\x03\xD4W`\x045a\x0B7a\x03\x14a\x0B>a\x0B/a\x0E\xA9\x85a\x13CV[\x91P`$5a\x18\xDAV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01Rtnor receive functions`X\x1B`d\x82\x01R`\x84\x90\xFD[\x91\x81``\x91\x81\x01\x03\x12a\x03\xD4Wa\x0F/a\x047\x92a\x12iV[\x90`@\x81\x015\x90` \x81\x015\x905a\x14bV[`@Q\x90``\x82\x01\x82\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\rJW`@R`\0`@\x83\x82\x81R\x82` \x82\x01R\x01RV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x90`\x01\x82\x01\x80\x92\x11a\x0F\x96WV[a\x0FrV[\x91\x90\x82\x01\x80\x92\x11a\x0F\x96WV[\x15a\x0F\xAFWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: x reserve increase`D\x82\x01Rad!`\xF0\x1B`d\x82\x01R`\x84\x90\xFD[\x90g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x03\x91\x82\x11a\x0F\x96WV[\x91\x90\x82\x03\x91\x82\x11a\x0F\x96WV[\x15a\x10)WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: y reserve increase`D\x82\x01Rad!`\xF0\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01Rd code`\xD8\x1B`d\x82\x01R`\x84\x90\xFD[\x91\x90\x82`\xC0\x91\x03\x12a\x03\xD4W\x81Qa\x10\xE3\x81a\x06\x95V[\x91` \x81\x01Q\x91`@\x82\x01Q\x91``\x81\x01Q\x91`\xA0`\x80\x83\x01Q\x92\x01Q\x90V[a\x047\x93\x92``\x92`\x01\x80`\xA0\x1B\x03\x16\x82R` \x82\x01R\x81`@\x82\x01R\x01\x90a\x04\x01V[`@Q=`\0\x82>=\x90\xFD[\x91a\n\x95a\x047\x93a\x12iV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\rJW`\x1F\x01`\x1F\x19\x16` \x01\x90V[` \x81\x83\x03\x12a\x03\xD4W\x80Q\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x03\xCFW\x01\x81`\x1F\x82\x01\x12\x15a\x03\xCAW\x80Qa\x11\x8F\x81a\x11@V[\x92a\x11\x9D`@Q\x94\x85a\r\x87V[\x81\x84R` \x82\x84\x01\x01\x11a\x11\xBBWa\x047\x91` \x80\x85\x01\x91\x01a\x03\xDEV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R`\x84\x90\xFD[\x91\x90\x82`\x80\x91\x03\x12a\x0E\x15W`@Qa\x12(\x81a\r.V[``\x80\x82\x94\x80Q\x84R` \x81\x01Q` \x85\x01R`@\x81\x01Q`@\x85\x01R\x01Q\x91a\x12Q\x83a\x0C5V[\x01RV[\x90`\x80\x82\x82\x03\x12a\x03\xD4Wa\x047\x91a\x12\x10V[``\x90`@Qa\x12x\x81a\r.V[`\0\x80\x82R` \x82\x01\x81\x90R`@\x82\x01\x81\x90R\x92\x01\x82\x90R\x81Ta\x12\xA6\x90a\x08$\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x80;\x15a\x08\xD2W`@Qc\xDC\x17\x83U`\xE0\x1B\x81R`\x04\x81\x01\x92\x90\x92R\x82\x90\x82\x90`$\x90\x82\x90Z\xFA\x90\x81\x15a\x08\xCDW\x82a\x047\x93\x92a\x12\xF0W[PP` \x80\x82Q\x83\x01\x01\x91\x01a\x12UV[a\x13\x0C\x92P=\x80\x91\x83>a\x13\x04\x81\x83a\r\x87V[\x81\x01\x90a\x11\\V[8\x80a\x12\xDFV[\x90\x81` \x91\x03\x12a\x03\xD4WQa\x047\x81a\x0C5V[\x90\x81``\x91\x03\x12a\x03\xD4W\x80Q\x91`@` \x83\x01Q\x92\x01Q\x90V[`\0T\x90\x91\x90a\x13]\x90a\x08$\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x80;\x15a\x08\xD2W` `\x04\x91`@Q\x92\x83\x80\x92c+\xEE\x84\xF1`\xE2\x1B\x82RZ\xFA\x90\x81\x15a\x08\xCDW`\0\x91a\x14\rW[P`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x08\xD2W`@Qc3\x85N\xFD`\xE2\x1B\x81R`\x04\x81\x01\x93\x90\x93R``\x90\x83\x90`$\x90\x82\x90Z\xFA\x90\x81\x15a\x08\xCDW`\0\x80\x93`\0\x93a\x13\xD6W[P\x92\x91\x90V[\x91\x93PPa\x13\xFC\x91P``=``\x11a\x14\x06W[a\x13\xF4\x81\x83a\r\x87V[\x81\x01\x90a\x13(V[\x92\x90\x92\x918a\x13\xD0V[P=a\x13\xEAV[a\x14/\x91P` =` \x11a\x145W[a\x14'\x81\x83a\r\x87V[\x81\x01\x90a\x13\x13V[8a\x13\x8BV[P=a\x14\x1DV[\x91a\x0Eza\x047\x93a\x12iV[\x81\x81\x03\x92\x91`\0\x13\x80\x15\x82\x85\x13\x16\x91\x84\x12\x16\x17a\x0F\x96WV[\x92` a\x07}\x84a\x14\x85a\x14}a\x07\x86\x96\x97a\x14\x8B\x99a\x1D\xDDV[\x85Q\x90a\x16\xC8V[\x95a\x1D\xDDV[g\r\xE0\xB6\xB3\xA7c\xFF\xFF\x19\x81\x01\x90\x81\x13`\x01\x16a\x0F\x96W\x90V[V[`\x01`\xFF\x1B\x81\x14a\x0F\x96W`\0\x03\x90V[\x90\x81g\r\xE0\xB6\xB3\xA7c\xFF\xFF\x19\x01\x91\x82\x12\x15`\x01\x16a\x0F\x96WV[\x91\x90\x91`\0\x83\x82\x01\x93\x84\x12\x91\x12\x90\x80\x15\x82\x16\x91\x15\x16\x17a\x0F\x96WV[\x94\x93\x92\x90\x92\x84Q\x90` \x86\x01Q`@\x87\x01Qa\x15\x08\x90a\x0F\xFFV[\x92a\x15\x13\x87\x87a\x1D\xDDV[a\x15\x1D\x82\x82a\x16\xC8V[\x92a\x15'\x91a\x16\xC8V[\x88Q\x87\x89\x85\x81a\x157\x85\x8Ca\x1D\xFFV[\x90a\x15A\x91a\x1D\xFFV[\x90a\x15K\x91a\x1D\xFFV[\x92a\x15V\x90\x88a\x1D\xFFV[a\x15`\x90\x88a\x10\x15V[\x90a\x15j\x91a\x0F\x9BV[\x90a\x15t\x91a\x1D\xFFV[a\x15}\x87a\x0F\xFFV[a\x15\x86\x91a\x1D\xFFV[\x92a\x15\x91\x8A\x87a\x0F\x9BV[\x90a\x15\x9B\x90a\x14\xA6V[a\x15\xA4\x91a\x16\xC8V[\x91a\x15\xAF\x90\x86a\x1D\xFFV[a\x15\xB8\x87a\x0F\xFFV[a\x15\xC1\x91a\x1D\xFFV[a\x15\xCB\x90\x88a\x0F\x9BV[\x92a\x15\xD5\x91a\x10\x15V[\x91a\x15\xDF\x91a\x1D\xFFV[\x88Qa\x15\xEA\x90a\x0F\xFFV[a\x15\xF3\x90a\x1D\xBEV[a\x15\xFC\x91a\x16\xC8V[a\x16\x05\x91a\x1D\xFFV[\x96Qa\x16\x10\x90a\x0F\xFFV[\x93a\x16\x1B\x87\x84a\x0F\x9BV[\x96a\x16%\x91a\x1D\xFFV[\x90a\x16/\x91a\x1D\xFFV[\x93a\x169\x91a\x1D\xFFV[\x90a\x16C\x90a\x0F\xFFV[a\x16L\x91a\x1D\xFFV[\x92a\x16V\x91a\x1D\xFFV[\x91a\x16`\x91a\x0F\x9BV[a\x16i\x91a\x1D\xFFV[\x91a\x16s\x90a\x14\xA6V[\x91a\x16}\x91a\x1D\xDDV[a\x047\x91a\x14\xD1V[a\x047\x92\x91` a\x16\x9Ca\x07\x86\x93\x85Q\x90a\x16\xC8V[\x93\x01Q\x90a\x16\xC8V[\x81\x81\x02\x92\x91`\0\x82\x12`\x01`\xFF\x1B\x82\x14\x16a\x0F\x96W\x81\x84\x05\x14\x90\x15\x17\x15a\x0F\x96WV[a\x18ua\x047\x92}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84a\x18\x83\x93a\x16\xFE`\0\x82\x13a /V[q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06a\x17\x1A\x82a\"\xB1V[``\x92\x83\x82`\x9F\x03\x01\x1B`\x9F\x1C\x90`_\x19\x01}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x02\x92l\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x82m\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x81m\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x81m\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x81m\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x81m\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x81lFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x02\x8C\x1D\x01\x02\x8A\x1D\x01\x02\x88\x1D\x01\x02\x86\x1D\x01\x02\x84\x1D\x01\x02\x82\x1D\x01\x91x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x91l\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x90\x82m\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x81l\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x81m\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x81m\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x81l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x81\x01\x02\x89\x1D\x01\x02\x87\x1D\x01\x02\x85\x1D\x01\x02\x83\x1D\x01\x02\x90\x1D\x01\x02\x01\x05\x02\x01\x01`\xAE\x1Da\x16\xA5V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[a gV[a\x18\xA7\x90a\x18\x9Fa\x047\x94\x93` \x85\x01Q\x90a\x1D\xDDV[\x92Q\x90a\x1D\xDDV[\x90a\x1D\xDDV[a\x18\xCDa\x047\x93\x92a\x18\xC7a\x18\xD4\x93` \x86\x01Q\x90a\x16\xC8V[\x90a\x1F\xB3V[\x91Qa\x1F\xE3V[\x90a\x16\xC8V[\x92\x91\x90a\x18\xF0a\x18\xEA\x82\x84a\x1F\xB3V[\x85a\x1F\x87V[\x93\x81\x03\x90\x81\x11a\x0F\x96W\x92\x81\x03\x90\x81\x11a\x0F\x96W\x90V[\x92\x91\x90a\x19\x17a\x18\xEA\x82\x84a\x1F\xB3V[\x93\x81\x01\x80\x91\x11a\x0F\x96W\x92\x81\x01\x80\x91\x11a\x0F\x96W\x90V[`@Q\x90a\x19;\x82a\rkV[`\x05\x82Rd\x19\x9A\\\x9C\xDD`\xDA\x1B` \x83\x01RV[`@Q\x90a\x19\\\x82a\rkV[`\x06\x82Re\x1C\xD9X\xDB\xDB\x99`\xD2\x1B` \x83\x01RV[`@Q\x90a\x19~\x82a\rkV[`\x05\x82Rd\x1D\x1A\x1A\\\x99`\xDA\x1B` \x83\x01RV[`@Q\x90a\x19\x9F\x82a\rkV[`\x06\x82Re\x0C\xCD\xEE\xAEN\x8D`\xD3\x1B` \x83\x01RV[`@Q\x90a\x19\xC1\x82a\rkV[`\x05\x82Rd\x0C\xCD,\xCE\x8D`\xDB\x1B` \x83\x01RV[`@Q\x90a\x19\xE2\x82a\rkV[`\x03\x82Rbnum`\xE8\x1B` \x83\x01RV[`@Q\x90a\x1A\x01\x82a\rkV[`\x03\x82Rb22\xB7`\xE9\x1B` \x83\x01RV[\x93\x82\x94\x92\x91\x92\x86Q\x80\x94`@\x89\x01Qa\x1A+\x90a\x0F\xFFV[\x97a\x1A5\x91a\x1D\xDDV[\x90a\x1A?\x91a\x16\xC8V[\x93a\x1AJ\x84\x84a\x0F\x9BV[\x90a\x1AT\x90a\x14\xB7V[a\x1A]\x91a\x16\xC8V[\x90a\x1Ah\x85\x84a\x1D\xFFV[\x94a\x1Ar\x88a\x0F\xFFV[a\x1A|\x90\x87a\x1D\xFFV[a\x1A\x86\x90\x88a\x0F\x9BV[\x91\x89Qa\x1A\x93\x87\x87a\x0F\x9BV[a\x1A\x9C\x91a\x1D\xFFV[\x93\x84a\x1A\xA6a\x19.V[\x90a\x1A\xB0\x91a!\xF1V[\x83\x80a\x1A\xBAa\x19OV[\x90a\x1A\xC4\x91a!\xF1V[a\x1A\xCD\x91a\x1D\xFFV[\x8AQa\x1A\xD8\x90a\x1D\xBEV[a\x1A\xE1\x91a\x16\xC8V[\x93\x84a\x1A\xEBa\x19qV[\x90a\x1A\xF5\x91a!\xF1V[\x8AQa\x1B\0\x90a\x0F\xFFV[a\x1B\n\x90\x8Aa\x1D\xFFV[\x92\x83a\x1B\x14a\x19\x92V[\x90a\x1B\x1E\x91a!\xF1V[\x87\x8CQa\x1B+\x90\x89a\x1D\xFFV[\x90a\x1B5\x91a\x0F\x9BV[a\x1B>\x91a\x1D\xFFV[a\x1BG\x8Ba\x0F\xFFV[a\x1BP\x91a\x1D\xFFV[\x93\x84a\x1BZa\x19\xB4V[\x90a\x1Bd\x91a!\xF1V[a\x1Bm\x91a\x1D\xFFV[\x93a\x1Bw\x91a\x1D\xFFV[\x91a\x1B\x81\x91a\x10\x15V[a\x1B\x8A\x91a\x1D\xFFV[a\x1B\x93\x91a\x14IV[\x95\x86a\x1B\x9Da\x19\xD5V[\x90a\x1B\xA7\x91a\"9V[Q\x91a\x1B\xB2\x91a\x0F\x9BV[a\x1B\xBB\x91a\x1D\xFFV[\x92\x83a\x1B\xC5a\x19.V[\x90a\x1B\xCF\x91a!\xF1V[a\x1B\xD8\x90a\x0F\xFFV[a\x1B\xE1\x91a\x1D\xFFV[a\x1B\xEA\x91a\x0F\x9BV[\x80a\x1B\xF3a\x19OV[\x90a\x1B\xFD\x91a!\xF1V[a\x1C\x06\x91a\x1D\xFFV[\x80a\x1C\x0Fa\x19\xF4V[\x90a\x1C\x19\x91a!\xF1V[`\0\x82\x13\x15a\x1C4Wa\x1C/\x90a\x047\x92a\x1D\xDDV[a\x14\xA6V[a\x1C@a\x047\x92a\x14\xA6V[a\x1D\xDDV[\x92\x91\x90\x83a\x1C`a\x1Ce\x92a\x1C`` \x86\x01Q\x86Q\x90a\x1D\xDDV[a\x1D\xFFV[\x90a\x1Cq\x81\x83\x86a\x16\x86V[\x93a\x1C~\x82\x86\x85\x84a\x14bV[\x85\x90`\0\x80\x82\x12\x15a\x1DGW[\x80\x82\x12a\x1D)WPa\x1C\xD0a\x1D\x1D\x92a\x047\x96\x97\x98\x86\x93[a\x1C\xB7`@Q\x98\x89\x92\x8C\x8A` \x86\x01a\"hV[\x03\x96a\x1C\xCB`\x1F\x19\x98\x89\x81\x01\x83R\x82a\r\x87V[a\x1ElV[\x81Q`@\x80\x84\x01Q``\x94\x85\x01Q\x82Q` \x81\x01\x98\x90\x98R\x91\x87\x01\x99\x90\x99R\x92\x85\x01\x91\x90\x91R`\x80\x84\x01R`\xA0\x83\x01\x95\x90\x95R`\x01`\x01`\xA0\x1B\x03\x90\x94\x16`\xC0\x82\x01R\x92\x83\x90`\xE0\x82\x01\x90V[\x03\x90\x81\x01\x83R\x82a\r\x87V[\x96a\x1D4\x91Pa\x1E V[\x95a\x1DA\x84\x88\x87\x86a\x14bV[\x90a\x1C\x8BV[\x96\x91\x96[\x80\x82\x13a\x1DgWPa\x1C\xD0a\x047\x95\x96\x97a\x1D\x1D\x93\x86\x93a\x1C\xA3V[\x96a\x1Dr\x91Pa \x05V[\x95a\x1D\x7F\x84\x88\x87\x86a\x14bV[\x90a\x1DKV[` a\x1D\x9Ea\x047\x94\x93a\x18\xC7a\x18\xD4\x94\x86Q\x90a\x16\xC8V[\x92\x01Qa\x1F\xE3V[`\x01\x81\x80\x04\x14\x81\x15\x17`\x01\x16\x15a\x06\x9FW`d\x90\x04\x90V[`\x01\x81\x15\x15\x16\x15a\x06\x9FWn\xC0\x97\xCE{\xC9\x07\x15\xB3K\x9F\x10\0\0\0\0\x04\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17\x82\x15\x15\x16\x15a\x06\x9FW\x04\x90V[\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x06\x9FWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x04\x90V[a\x03\xE7\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x06\x9FWa\x03\xE8\x90\x04\x90V[\x91\x90a\x01\0\x83\x82\x03\x12a\x03\xD4W\x82Q\x92` \x81\x01Q\x92a\x047`@\x83\x01Q\x93`\x80``\x85\x01Q\x94\x01a\x12\x10V[`\0\x93\x92\x91\x84\x91\x83\x82\x11a\x1FgWa\x1E\x84\x82\x82a#\x80V[a\x1E\x8E\x85\x83a#\x80V[`\0a\x1E\x9A\x82\x84a\x16\xA5V[\x13a\x1FHWPa\x1E\xAC\x83\x86\x97\x96a\x10\x15V[`\x01\x94`\0\x91\x86\x80[a\x1E\xC6W[PPPPPPPP\x90PV[\x15a\x1F#W[P\x85\x96\x97\x98P\x80\x91a\x1E\xE1a\x05\xB6\x8B\x88a\x0F\x9BV[\x99a\x1E\xEC\x8B\x87a#\x80V[\x90\x83a\x1E\xF8\x87\x84a\x16\xA5V[\x13a\x1F\x17WPP\x89\x92[\x87a\x1F\r\x88\x86a\x10\x15V[\x92\x01\x93\x99\x98a\x1E\xB5V[\x8B\x97P\x90\x94P\x92a\x1F\x02V[\x86\x10\x80a\x1F=W[\x15a\x1F6W\x88a\x1E\xCCV[\x80\x80a\x1E\xBAV[Pa\x01\0\x82\x10a\x1F+V[`@Qc\x06\xF1\xBE]`\xE2\x1B\x81R`\x04\x81\x01\x92\x90\x92R`$\x82\x01R`D\x90\xFD[`@Qc0\x82\xDF\xDB`\xE1\x1B\x81R`\x04\x81\x01\x83\x90R`$\x81\x01\x85\x90R`D\x90\xFD[\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x06\x9FW`\x01g\r\xE0\xB6\xB3\xA7d\0\0`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[\x90g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x81\x02\x92\x81\x84\x04\x14\x90\x15\x17\x81\x15\x15\x16\x15a\x06\x9FW`\x01\x90`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[`\x01\x81\x15\x15\x16\x15a\x06\x9FWn\xC0\x97\xCE{\xC9\x07\x15\xB3K\x9F\x0F\xFF\xFF\xFF\xFF\x04`\x01\x01\x90V[a\x03\xE9\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x06\x9FW`\x01a\x03\xE8`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[\x15a 6WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x90\xFD[h\x02H\xCE6\xA7\x0C\xB2k>\x19\x81\x13\x15a!\xEBWh\x07U\xBFy\x8BJ\x1B\xF1\xE5\x81\x12\x15a!\xB7We\x03x-\xAC\xE9\xD9\x90`N\x1B\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x91``\x90`\x01`_\x1B\x84\x82\x84\x1B\x05\x01\x82\x1D\x93\x84\x02\x90\x03\x80l\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x81\x01\x02\x82\x1D\x90n\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dPn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x82n\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x81m\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x81m\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x81m\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x81l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x02\x8D\x1D\x01\x02\x8B\x1D\x01\x02\x89\x1D\x01\x02\x87\x1D\x01\x02\x85\x1D\x01\x93m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x93m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEA\x81\x01\x90\x84m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x91\x01\x01\x02\x90\x1D\x01\x02\x01\x05\x02\x90`\xC3\x03\x1C\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x90\xFD[P`\0\x90V[a\"4a\" \x91a\x14\xA4\x93`@Q\x93\x84\x92c-\x83\x9C\xB3`\xE2\x1B` \x85\x01R`@`$\x85\x01R`d\x84\x01\x90a\x04\x01V[\x90`D\x83\x01R\x03`\x1F\x19\x81\x01\x83R\x82a\r\x87V[a##V[a\"4a\" \x91a\x14\xA4\x93`@Q\x93\x84\x92c\x1ES\x13G`\xE1\x1B` \x85\x01R`@`$\x85\x01R`d\x84\x01\x90a\x04\x01V[a\x14\xA4\x93``\x92\x96\x95\x93`\xE0\x83\x01\x97\x83R` \x83\x01R`@\x82\x01R\x01\x90``\x90\x80Q\x83R` \x81\x01Q` \x84\x01R`@\x81\x01Q`@\x84\x01R\x81`\x01\x80`\xA0\x1B\x03\x91\x01Q\x16\x91\x01RV[a\"\xBC\x81\x15\x15a /V[\x80o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x07\x1B\x81\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x81\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x81\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x81\x81\x1C`\xFF\x10`\x03\x1B\x17\x81\x81\x1C`\x0F\x10`\x02\x1B\x17\x81\x81\x1C`\x03\x10`\x01\x1B\x17\x90\x81\x1C`\x01\x10\x17\x90V[`\0\x80\x91` \x81Q\x91\x01jconsole.logZ\xFAPV[\x90a#Ua\x047\x92` \x80\x82Q\x83\x01\x01\x91\x01a\x1E?V[\x94\x93\x92\x90\x92a\x14\xEDV[\x90a#va\x047\x92` \x80\x82Q\x83\x01\x01\x91\x01a\x1E?V[\x94\x93\x92\x90\x92a\x1A\x13V[\x80Q\x81\x01\x91`\xE0\x82\x84\x03\x12a\x03\xD4Wa\x047\x92a#\xAE` \x84\x01Q\x93`\x80` `@\x83\x01Q\x94\x01\x91\x01a\x12\x10V[\x92a\x14bV\xFE\xA2dipfsX\"\x12 \x0F\x88xl\x94f-.\x97{\xB6DM\xD9\xF7JsM~\xB6\x98\xF8$f\xAF\xE2\xC8\xFC\xEE\xE5/\xD2dsolcC\0\x08\x16\x003";
    /// The bytecode of the contract.
    pub static G3MSOLVER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\x0E\xB3W`\x005`\xE0\x1C\x80c\x0FAf\xB8\x14a\x01\\W\x80c%\th\xD9\x14a\x01WW\x80c0m\xB4k\x14a\x01RW\x80c3\"f\xF3\x14a\x01MW\x80c9(\xFF\x97\x14a\x01HW\x80c;M\x100\x14a\x01CW\x80cO\xD6|X\x14a\x01>W\x80cZ\x93\xB8\xCE\x14a\x019W\x80cb7V\x9F\x14a\x014W\x80c\x7F\x17@\x9C\x14a\x01/W\x80c\x81\xB5\xFA\xC2\x14a\x01*W\x80c\x90.\xCA\xA2\x14a\x01%W\x80c\xA8\xC6.v\x14a\x01 W\x80c\xB0\x9D\x04\xE5\x14a\x01\x1BW\x80c\xCB\x1FU2\x14a\x01\x16W\x80c\xCE\x15;\xF4\x14a\x01\x11W\x80c\xDE\xF1_\x92\x14a\x01\x0CW\x80c\xEC)\xD8\xE6\x14a\x01\x07W\x80c\xEE>\x8C\xFB\x14a\x01\x02W\x80c\xF2\xDEz{\x14a\0\xFDWc\xF3\r7\xF2\x03a\x0E\xB3Wa\x0E\x80V[a\x0EdV[a\x0E0V[a\x0E\x1AV[a\r\xA9V[a\x0C\x8BV[a\x0CFV[a\x0C\x02V[a\x0B\xD9V[a\x0B\xB0V[a\x0B\\V[a\n\xFCV[a\n\x9BV[a\nvV[a\t\x8FV[a\t]V[a\x06\xC9V[a\x06dV[a\x04\xA3V[a\x04:V[a\x02\xAAV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01Ra7\xB7`\xF1\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x90\xFD[4a\x03\xD9W`@6`\x03\x19\x01\x12a\x03\xD4Wg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`$5\x81\x81\x11a\x03\xCFW6`#\x82\x01\x12\x15a\x03\xCAW\x80`\x04\x015\x91\x82\x11a\x03qW6`$\x83\x83\x01\x01\x11a\x03\x18Wa\x03\x14\x91`$a\x03\x04\x92\x01`\x045a\x0F\x16V[`@Q\x90\x81R\x90\x81\x90` \x82\x01\x90V[\x03\x90\xF3[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rjrray stride`\xA8\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x0ENL/$\r\x8C\xAD\xCC\xEE\x8D`\xAB\x1B`d\x82\x01R`\x84\x90\xFD[a\x02QV[a\x02\x01V[a\x01\xB1V[a\x01aV[`\0[\x83\x81\x10a\x03\xF1WPP`\0\x91\x01RV[\x81\x81\x01Q\x83\x82\x01R` \x01a\x03\xE1V[\x90` \x91a\x04\x1A\x81Q\x80\x92\x81\x85R\x85\x80\x86\x01\x91\x01a\x03\xDEV[`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[\x90` a\x047\x92\x81\x81R\x01\x90a\x04\x01V[\x90V[4a\x03\xD9W`@6`\x03\x19\x01\x12a\x03\xD4Wa\x03\x14`@Q`\x02` \x82\x01R`\x045`@\x82\x01R`$5``\x82\x01R``\x81Ra\x04u\x81a\r.V[`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x04\x01V[``\x90`\x03\x19\x01\x12a\x03\xD4W`\x045\x90`$5\x90`D5\x90V[4a\x03\xD9Wa\x053a\x05Aa\x04\xB76a\x04\x89V[\x92\x90a\x04\xCBa\x04\xC5\x84a\x12iV[\x93a\x13CV[a\x04\xD9\x87\x98\x93\x96\x92\x98a\x1D\xA6V[\x97`@Q\x96\x87\x95` \x87\x01\x90\x81R` \x80\x82\x01\x92\x90\x92R`@\x80\x82\x01\x93\x90\x93R``\x80\x82\x01\x94\x90\x94R\x84Q`\x80\x82\x01R\x90\x84\x01Q`\xA0\x82\x01R\x90\x83\x01Q`\xC0\x82\x01R\x91\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\xE0\x82\x01Ra\x01\0\x01\x90V[\x03`\x1F\x19\x81\x01\x83R\x82a\r\x87V[`\0\x92\x82\x91\x81\x83\x81\x11a\x06CWa\x05X\x81\x83a#>V[\x94a\x05c\x81\x84a#>V[\x87a\x05n\x82\x89a\x16\xA5V[\x13a\x06\"WP\x90a\x05\x81\x91\x95\x94\x95a\x10\x15V[\x92\x85`\x01\x94\x85`\x01\x90[a\x05\x9BW[` \x89`@Q\x90\x81R\xF3[\x15a\x05\xFDW[P\x84\x95\x96P`\0\x90a\x05\xBCa\x05\xB6\x89\x87a\x0F\x9BV[`\x01\x1C\x90V[\x97a\x05\xC7\x89\x86a#>V[\x90\x83a\x05\xD3\x86\x84a\x16\xA5V[\x13a\x05\xF1WPP\x87\x91[\x86a\x05\xE8\x87\x85a\x10\x15V[\x92\x01\x92\x97a\x05\x8BV[\x89\x96P\x90\x93P\x91a\x05\xDDV[\x85\x10\x80a\x06\x17W[\x15a\x06\x10W\x86a\x05\xA1V[\x80\x80a\x05\x90V[Pa\x01\0\x81\x10a\x06\x05V[`@Qc\x06\xF1\xBE]`\xE2\x1B\x81R`\x04\x81\x01\x88\x90R`$\x81\x01\x91\x90\x91R`D\x90\xFD[`@Qc0\x82\xDF\xDB`\xE1\x1B\x81R`\x04\x81\x01\x91\x90\x91R`$\x81\x01\x85\x90R`D\x90\xFD[4a\x03\xD9W` a\x06\x8Da\x06w6a\x04\x89V[\x90a\x06\x84a\x04\xC5\x84a\x12iV[\x92\x91\x90\x91a\x14\xEDV[`@Q\x90\x81R\xF3[\x80\x15\x15\x03a\x06\x9FWV[`\0\x80\xFD[\x90\x92`\x80\x92a\x047\x95\x94\x15\x15\x83R` \x83\x01R`@\x82\x01R\x81``\x82\x01R\x01\x90a\x04\x01V[4a\x03\xD9W``6`\x03\x19\x01\x12a\x03\xD4W`\x045`$5a\x06\xE9\x81a\x06\x95V[`D5\x91a\x06\xF5a\x0FBV[\x91a\x06\xFEa\x0FBV[\x92a\x07\x08\x83a\x13CV[\x92\x90\x91\x93` \x94\x85\x83\x01\x93`@\x95\x86\x85\x01R\x84R\x82Ra\x07'\x86a\x12iV[\x97\x85\x89\x86a\x07A\x86Q\x88Qa\x07;\x8Da\x12iV[\x91a\x16\x86V[\x94\x15a\x08\xD7Wa\x07\x9F\x93a\x07\x91a\x07\x8Ca\x07\xD3\x99\x98\x95a\x07\x86\x86a\x07oa\x07\x98\x97a\x07\xAC\x9C\x99\x01Q\x87a\x1F\x87V[\x92a\x07}\x8DQ\x8BQ\x90a\x1F\xB3V[\x91\x01Q\x90a\x16\xC8V[\x90a\x1F\x87V[a\x0F\x88V[\x93Qa\x0F\x9BV[\x8ARa\x0F\x9BV[\x80\x85\x89\x01R\x87Q\x87a\x14<V[\x90a\x07\xCAa\x07\xBF\x86\x89\x01\x93\x80\x85Ra\x0F\x88V[\x80\x84R\x82Q\x11a\x10\"V[Q\x90Q\x90a\x10\x15V[\x94[\x84Q\x83\x86\x01\x80Q\x84\x88\x01Q\x85Q\x96\x87\x01\x93\x84R` \x84\x01\x91\x90\x91R`@\x83\x01R\x90a\x08\r\x90\x85\x90``\x01\x03`\x1F\x19\x81\x01\x86R\x85a\r\x87V[`\0Ta\x080\x90a\x08$\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x80;\x15a\x08\xD2W\x84`\xC0\x91a\x08[\x97\x86Q\x80\x99\x81\x94\x82\x93c\r\x17\xA7\xC7`\xE3\x1B\x84R0`\x04\x85\x01a\x11\x03V[\x03\x91Z\xFA\x94\x85\x15a\x08\xCDW`\0\x95a\x08\x8DW[P\x90a\x08\x82\x91a\x03\x14\x95\x96Q\x90Q\x90a\x18\x88V[\x90Q\x94\x85\x94\x85a\x06\xA4V[a\x03\x14\x95P\x90a\x08\xB8a\x08\x82\x93\x92`\xC0=`\xC0\x11a\x08\xC6W[a\x08\xB0\x81\x83a\r\x87V[\x81\x01\x90a\x10\xCCV[PPPPP\x95P\x90\x91a\x08nV[P=a\x08\xA6V[a\x11'V[a\x10yV[a\t;\x92Pa\tW\x96a\t\"a\tC\x95a\t\x1Ba\x07\x8C\x8Aa\x07\x86\x88a\t\x13a\t\ta\t.\x9Aa\tN\x9F\x9C\x01Q\x88a\x1F\x87V[\x93Q\x89Q\x90a\x1F\xB3V[\x90Q\x90a\x16\xC8V[\x92Qa\x0F\x9BV[\x92\x8A\x8D\x01\x93\x84Ra\x0F\x9BV[\x90\x81\x88\x8C\x01RQ\x89a\x113V[\x80\x89Ra\x0F\x88V[\x80\x88R\x82Q\x11a\x0F\xA8V[Q\x85Q\x90a\x10\x15V[\x94a\x07\xD5V[4a\x03\xD9W` 6`\x03\x19\x01\x12a\x03\xD4W` a\x06\x8D`\x045a\t\x88a\t\x82\x82a\x12iV[\x91a\x13CV[P\x90a\x18\x88V[4a\x03\xD9Wa\x053a\t\xA3a\x04\xB76a\x04\x89V[`\0\x92\x82\x91\x81\x83\x81\x11a\x06CWa\t\xBA\x81\x83a#_V[\x94a\t\xC5\x81\x84a#_V[\x87a\t\xD0\x82\x89a\x16\xA5V[\x13a\x06\"WP\x90a\t\xE3\x91\x95\x94\x95a\x10\x15V[\x92\x85`\x01\x94\x85`\x01\x90[a\t\xFCW` \x89`@Q\x90\x81R\xF3[\x15a\nXW[P\x84\x95\x96P`\0\x90a\n\x17a\x05\xB6\x89\x87a\x0F\x9BV[\x97a\n\"\x89\x86a#_V[\x90\x83a\n.\x86\x84a\x16\xA5V[\x13a\nLWPP\x87\x91[\x86a\nC\x87\x85a\x10\x15V[\x92\x01\x92\x97a\t\xEDV[\x89\x96P\x90\x93P\x91a\n8V[\x85\x10\x80a\nkW[\x15a\x06\x10W\x86a\n\x02V[Pa\x01\0\x81\x10a\n`V[4a\x03\xD9W` a\x06\x8Da\n\x95a\n\x8C6a\x04\x89V[\x91\x92\x90\x92a\x12iV[\x91a\x18\xADV[4a\x03\xD9W`@6`\x03\x19\x01\x12a\x03\xD4Wa\n\xD7`\x045a\x03\x14a\n\xDEa\n\xCFa\n\xC4\x84a\x13CV[\x91\x90P`$5a\x18\xDAV[\x94\x90\x93a\x12iV[\x84\x84a\x1D\x85V[\x92`@Q\x93\x84\x93\x84`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[4a\x03\xD9W`@6`\x03\x19\x01\x12a\x03\xD4W`\x045a\x0B7a\x03\x14a\x0B>a\x0B/a\x0B%\x85a\x13CV[\x91P`$5a\x19\x07V[\x93\x90\x94a\x12iV[\x83\x85a\x18\xADV[\x91`@Q\x93\x84\x93\x84`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[4a\x03\xD9W` 6`\x03\x19\x01\x12a\x03\xD4W`\x80a\x0Bz`\x045a\x12iV[a\x0B\xAE`@Q\x80\x92``\x90\x80Q\x83R` \x81\x01Q` \x84\x01R`@\x81\x01Q`@\x84\x01R\x81`\x01\x80`\xA0\x1B\x03\x91\x01Q\x16\x91\x01RV[\xF3[4a\x03\xD9W` a\x06\x8Da\x0B\xC36a\x04\x89V[\x90a\x0B\xD0a\x04\xC5\x84a\x12iV[\x92\x91\x90\x91a\x1A\x13V[4a\x03\xD9W`\x006`\x03\x19\x01\x12a\x03\xD4W`\0T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x03\xD9W` 6`\x03\x19\x01\x12a\x03\xD4Wa\x03\x14`@Q`\x01` \x82\x01R`\x045`@\x82\x01R`@\x81Ra\x04u\x81a\rOV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\x06\x9FWV[4a\x03\xD9W` 6`\x03\x19\x01\x12a\x03\xD4Wa\x03\x14`\x045a\x0Cf\x81a\x0C5V[`@\x80Q`\x03` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x82\x82\x01R\x81Ra\x04u\x81a\rOV[4a\x03\xD9W` 6`\x03\x19\x01\x12a\x03\xD4Wa\x03\x14a\x0C\xAA`\x045a\x13CV[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R\x90\x81\x90``\x82\x01\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FABI decoding: struct data too sh`D\x82\x01Rb\x1B\xDC\x9D`\xEA\x1B`d\x82\x01R`\x84\x90\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\x80\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\rJW`@RV[a\r\x18V[``\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\rJW`@RV[`@\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\rJW`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\rJW`@RV[4a\x03\xD9W`\xC06`\x03\x19\x01\x12a\x03\xD4W`\x806`C\x19\x01\x12a\x0E\x15Wa\x03\x14a\x0E\t`@Qa\r\xD8\x81a\r.V[`D5\x81R`d5` \x82\x01R`\x845`@\x82\x01R`\xA45a\r\xF9\x81a\x0C5V[``\x82\x01R`$5`\x045a\x1CEV[`@Q\x91\x82\x91\x82a\x04&V[a\x0C\xC7V[4a\x03\xD9W` a\x06\x8Da\x07;a\n\x8C6a\x04\x89V[4a\x03\xD9W`@6`\x03\x19\x01\x12a\x03\xD4Wa\n\xD7`\x045a\x03\x14a\n\xDEa\n\xCFa\x0EY\x84a\x13CV[\x91\x90P`$5a\x19\x07V[4a\x03\xD9W` a\x06\x8Da\x0Eza\n\x8C6a\x04\x89V[\x91a\x1D\x85V[4a\x03\xD9W`@6`\x03\x19\x01\x12a\x03\xD4W`\x045a\x0B7a\x03\x14a\x0B>a\x0B/a\x0E\xA9\x85a\x13CV[\x91P`$5a\x18\xDAV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01Rtnor receive functions`X\x1B`d\x82\x01R`\x84\x90\xFD[\x91\x81``\x91\x81\x01\x03\x12a\x03\xD4Wa\x0F/a\x047\x92a\x12iV[\x90`@\x81\x015\x90` \x81\x015\x905a\x14bV[`@Q\x90``\x82\x01\x82\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\rJW`@R`\0`@\x83\x82\x81R\x82` \x82\x01R\x01RV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x90`\x01\x82\x01\x80\x92\x11a\x0F\x96WV[a\x0FrV[\x91\x90\x82\x01\x80\x92\x11a\x0F\x96WV[\x15a\x0F\xAFWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: x reserve increase`D\x82\x01Rad!`\xF0\x1B`d\x82\x01R`\x84\x90\xFD[\x90g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x03\x91\x82\x11a\x0F\x96WV[\x91\x90\x82\x03\x91\x82\x11a\x0F\x96WV[\x15a\x10)WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: y reserve increase`D\x82\x01Rad!`\xF0\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01Rd code`\xD8\x1B`d\x82\x01R`\x84\x90\xFD[\x91\x90\x82`\xC0\x91\x03\x12a\x03\xD4W\x81Qa\x10\xE3\x81a\x06\x95V[\x91` \x81\x01Q\x91`@\x82\x01Q\x91``\x81\x01Q\x91`\xA0`\x80\x83\x01Q\x92\x01Q\x90V[a\x047\x93\x92``\x92`\x01\x80`\xA0\x1B\x03\x16\x82R` \x82\x01R\x81`@\x82\x01R\x01\x90a\x04\x01V[`@Q=`\0\x82>=\x90\xFD[\x91a\n\x95a\x047\x93a\x12iV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\rJW`\x1F\x01`\x1F\x19\x16` \x01\x90V[` \x81\x83\x03\x12a\x03\xD4W\x80Q\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x03\xCFW\x01\x81`\x1F\x82\x01\x12\x15a\x03\xCAW\x80Qa\x11\x8F\x81a\x11@V[\x92a\x11\x9D`@Q\x94\x85a\r\x87V[\x81\x84R` \x82\x84\x01\x01\x11a\x11\xBBWa\x047\x91` \x80\x85\x01\x91\x01a\x03\xDEV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R`\x84\x90\xFD[\x91\x90\x82`\x80\x91\x03\x12a\x0E\x15W`@Qa\x12(\x81a\r.V[``\x80\x82\x94\x80Q\x84R` \x81\x01Q` \x85\x01R`@\x81\x01Q`@\x85\x01R\x01Q\x91a\x12Q\x83a\x0C5V[\x01RV[\x90`\x80\x82\x82\x03\x12a\x03\xD4Wa\x047\x91a\x12\x10V[``\x90`@Qa\x12x\x81a\r.V[`\0\x80\x82R` \x82\x01\x81\x90R`@\x82\x01\x81\x90R\x92\x01\x82\x90R\x81Ta\x12\xA6\x90a\x08$\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x80;\x15a\x08\xD2W`@Qc\xDC\x17\x83U`\xE0\x1B\x81R`\x04\x81\x01\x92\x90\x92R\x82\x90\x82\x90`$\x90\x82\x90Z\xFA\x90\x81\x15a\x08\xCDW\x82a\x047\x93\x92a\x12\xF0W[PP` \x80\x82Q\x83\x01\x01\x91\x01a\x12UV[a\x13\x0C\x92P=\x80\x91\x83>a\x13\x04\x81\x83a\r\x87V[\x81\x01\x90a\x11\\V[8\x80a\x12\xDFV[\x90\x81` \x91\x03\x12a\x03\xD4WQa\x047\x81a\x0C5V[\x90\x81``\x91\x03\x12a\x03\xD4W\x80Q\x91`@` \x83\x01Q\x92\x01Q\x90V[`\0T\x90\x91\x90a\x13]\x90a\x08$\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x80;\x15a\x08\xD2W` `\x04\x91`@Q\x92\x83\x80\x92c+\xEE\x84\xF1`\xE2\x1B\x82RZ\xFA\x90\x81\x15a\x08\xCDW`\0\x91a\x14\rW[P`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x08\xD2W`@Qc3\x85N\xFD`\xE2\x1B\x81R`\x04\x81\x01\x93\x90\x93R``\x90\x83\x90`$\x90\x82\x90Z\xFA\x90\x81\x15a\x08\xCDW`\0\x80\x93`\0\x93a\x13\xD6W[P\x92\x91\x90V[\x91\x93PPa\x13\xFC\x91P``=``\x11a\x14\x06W[a\x13\xF4\x81\x83a\r\x87V[\x81\x01\x90a\x13(V[\x92\x90\x92\x918a\x13\xD0V[P=a\x13\xEAV[a\x14/\x91P` =` \x11a\x145W[a\x14'\x81\x83a\r\x87V[\x81\x01\x90a\x13\x13V[8a\x13\x8BV[P=a\x14\x1DV[\x91a\x0Eza\x047\x93a\x12iV[\x81\x81\x03\x92\x91`\0\x13\x80\x15\x82\x85\x13\x16\x91\x84\x12\x16\x17a\x0F\x96WV[\x92` a\x07}\x84a\x14\x85a\x14}a\x07\x86\x96\x97a\x14\x8B\x99a\x1D\xDDV[\x85Q\x90a\x16\xC8V[\x95a\x1D\xDDV[g\r\xE0\xB6\xB3\xA7c\xFF\xFF\x19\x81\x01\x90\x81\x13`\x01\x16a\x0F\x96W\x90V[V[`\x01`\xFF\x1B\x81\x14a\x0F\x96W`\0\x03\x90V[\x90\x81g\r\xE0\xB6\xB3\xA7c\xFF\xFF\x19\x01\x91\x82\x12\x15`\x01\x16a\x0F\x96WV[\x91\x90\x91`\0\x83\x82\x01\x93\x84\x12\x91\x12\x90\x80\x15\x82\x16\x91\x15\x16\x17a\x0F\x96WV[\x94\x93\x92\x90\x92\x84Q\x90` \x86\x01Q`@\x87\x01Qa\x15\x08\x90a\x0F\xFFV[\x92a\x15\x13\x87\x87a\x1D\xDDV[a\x15\x1D\x82\x82a\x16\xC8V[\x92a\x15'\x91a\x16\xC8V[\x88Q\x87\x89\x85\x81a\x157\x85\x8Ca\x1D\xFFV[\x90a\x15A\x91a\x1D\xFFV[\x90a\x15K\x91a\x1D\xFFV[\x92a\x15V\x90\x88a\x1D\xFFV[a\x15`\x90\x88a\x10\x15V[\x90a\x15j\x91a\x0F\x9BV[\x90a\x15t\x91a\x1D\xFFV[a\x15}\x87a\x0F\xFFV[a\x15\x86\x91a\x1D\xFFV[\x92a\x15\x91\x8A\x87a\x0F\x9BV[\x90a\x15\x9B\x90a\x14\xA6V[a\x15\xA4\x91a\x16\xC8V[\x91a\x15\xAF\x90\x86a\x1D\xFFV[a\x15\xB8\x87a\x0F\xFFV[a\x15\xC1\x91a\x1D\xFFV[a\x15\xCB\x90\x88a\x0F\x9BV[\x92a\x15\xD5\x91a\x10\x15V[\x91a\x15\xDF\x91a\x1D\xFFV[\x88Qa\x15\xEA\x90a\x0F\xFFV[a\x15\xF3\x90a\x1D\xBEV[a\x15\xFC\x91a\x16\xC8V[a\x16\x05\x91a\x1D\xFFV[\x96Qa\x16\x10\x90a\x0F\xFFV[\x93a\x16\x1B\x87\x84a\x0F\x9BV[\x96a\x16%\x91a\x1D\xFFV[\x90a\x16/\x91a\x1D\xFFV[\x93a\x169\x91a\x1D\xFFV[\x90a\x16C\x90a\x0F\xFFV[a\x16L\x91a\x1D\xFFV[\x92a\x16V\x91a\x1D\xFFV[\x91a\x16`\x91a\x0F\x9BV[a\x16i\x91a\x1D\xFFV[\x91a\x16s\x90a\x14\xA6V[\x91a\x16}\x91a\x1D\xDDV[a\x047\x91a\x14\xD1V[a\x047\x92\x91` a\x16\x9Ca\x07\x86\x93\x85Q\x90a\x16\xC8V[\x93\x01Q\x90a\x16\xC8V[\x81\x81\x02\x92\x91`\0\x82\x12`\x01`\xFF\x1B\x82\x14\x16a\x0F\x96W\x81\x84\x05\x14\x90\x15\x17\x15a\x0F\x96WV[a\x18ua\x047\x92}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84a\x18\x83\x93a\x16\xFE`\0\x82\x13a /V[q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06a\x17\x1A\x82a\"\xB1V[``\x92\x83\x82`\x9F\x03\x01\x1B`\x9F\x1C\x90`_\x19\x01}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x02\x92l\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x82m\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x81m\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x81m\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x81m\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x81m\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x81lFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x02\x8C\x1D\x01\x02\x8A\x1D\x01\x02\x88\x1D\x01\x02\x86\x1D\x01\x02\x84\x1D\x01\x02\x82\x1D\x01\x91x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x91l\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x90\x82m\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x81l\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x81m\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x81m\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x81l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x81\x01\x02\x89\x1D\x01\x02\x87\x1D\x01\x02\x85\x1D\x01\x02\x83\x1D\x01\x02\x90\x1D\x01\x02\x01\x05\x02\x01\x01`\xAE\x1Da\x16\xA5V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[a gV[a\x18\xA7\x90a\x18\x9Fa\x047\x94\x93` \x85\x01Q\x90a\x1D\xDDV[\x92Q\x90a\x1D\xDDV[\x90a\x1D\xDDV[a\x18\xCDa\x047\x93\x92a\x18\xC7a\x18\xD4\x93` \x86\x01Q\x90a\x16\xC8V[\x90a\x1F\xB3V[\x91Qa\x1F\xE3V[\x90a\x16\xC8V[\x92\x91\x90a\x18\xF0a\x18\xEA\x82\x84a\x1F\xB3V[\x85a\x1F\x87V[\x93\x81\x03\x90\x81\x11a\x0F\x96W\x92\x81\x03\x90\x81\x11a\x0F\x96W\x90V[\x92\x91\x90a\x19\x17a\x18\xEA\x82\x84a\x1F\xB3V[\x93\x81\x01\x80\x91\x11a\x0F\x96W\x92\x81\x01\x80\x91\x11a\x0F\x96W\x90V[`@Q\x90a\x19;\x82a\rkV[`\x05\x82Rd\x19\x9A\\\x9C\xDD`\xDA\x1B` \x83\x01RV[`@Q\x90a\x19\\\x82a\rkV[`\x06\x82Re\x1C\xD9X\xDB\xDB\x99`\xD2\x1B` \x83\x01RV[`@Q\x90a\x19~\x82a\rkV[`\x05\x82Rd\x1D\x1A\x1A\\\x99`\xDA\x1B` \x83\x01RV[`@Q\x90a\x19\x9F\x82a\rkV[`\x06\x82Re\x0C\xCD\xEE\xAEN\x8D`\xD3\x1B` \x83\x01RV[`@Q\x90a\x19\xC1\x82a\rkV[`\x05\x82Rd\x0C\xCD,\xCE\x8D`\xDB\x1B` \x83\x01RV[`@Q\x90a\x19\xE2\x82a\rkV[`\x03\x82Rbnum`\xE8\x1B` \x83\x01RV[`@Q\x90a\x1A\x01\x82a\rkV[`\x03\x82Rb22\xB7`\xE9\x1B` \x83\x01RV[\x93\x82\x94\x92\x91\x92\x86Q\x80\x94`@\x89\x01Qa\x1A+\x90a\x0F\xFFV[\x97a\x1A5\x91a\x1D\xDDV[\x90a\x1A?\x91a\x16\xC8V[\x93a\x1AJ\x84\x84a\x0F\x9BV[\x90a\x1AT\x90a\x14\xB7V[a\x1A]\x91a\x16\xC8V[\x90a\x1Ah\x85\x84a\x1D\xFFV[\x94a\x1Ar\x88a\x0F\xFFV[a\x1A|\x90\x87a\x1D\xFFV[a\x1A\x86\x90\x88a\x0F\x9BV[\x91\x89Qa\x1A\x93\x87\x87a\x0F\x9BV[a\x1A\x9C\x91a\x1D\xFFV[\x93\x84a\x1A\xA6a\x19.V[\x90a\x1A\xB0\x91a!\xF1V[\x83\x80a\x1A\xBAa\x19OV[\x90a\x1A\xC4\x91a!\xF1V[a\x1A\xCD\x91a\x1D\xFFV[\x8AQa\x1A\xD8\x90a\x1D\xBEV[a\x1A\xE1\x91a\x16\xC8V[\x93\x84a\x1A\xEBa\x19qV[\x90a\x1A\xF5\x91a!\xF1V[\x8AQa\x1B\0\x90a\x0F\xFFV[a\x1B\n\x90\x8Aa\x1D\xFFV[\x92\x83a\x1B\x14a\x19\x92V[\x90a\x1B\x1E\x91a!\xF1V[\x87\x8CQa\x1B+\x90\x89a\x1D\xFFV[\x90a\x1B5\x91a\x0F\x9BV[a\x1B>\x91a\x1D\xFFV[a\x1BG\x8Ba\x0F\xFFV[a\x1BP\x91a\x1D\xFFV[\x93\x84a\x1BZa\x19\xB4V[\x90a\x1Bd\x91a!\xF1V[a\x1Bm\x91a\x1D\xFFV[\x93a\x1Bw\x91a\x1D\xFFV[\x91a\x1B\x81\x91a\x10\x15V[a\x1B\x8A\x91a\x1D\xFFV[a\x1B\x93\x91a\x14IV[\x95\x86a\x1B\x9Da\x19\xD5V[\x90a\x1B\xA7\x91a\"9V[Q\x91a\x1B\xB2\x91a\x0F\x9BV[a\x1B\xBB\x91a\x1D\xFFV[\x92\x83a\x1B\xC5a\x19.V[\x90a\x1B\xCF\x91a!\xF1V[a\x1B\xD8\x90a\x0F\xFFV[a\x1B\xE1\x91a\x1D\xFFV[a\x1B\xEA\x91a\x0F\x9BV[\x80a\x1B\xF3a\x19OV[\x90a\x1B\xFD\x91a!\xF1V[a\x1C\x06\x91a\x1D\xFFV[\x80a\x1C\x0Fa\x19\xF4V[\x90a\x1C\x19\x91a!\xF1V[`\0\x82\x13\x15a\x1C4Wa\x1C/\x90a\x047\x92a\x1D\xDDV[a\x14\xA6V[a\x1C@a\x047\x92a\x14\xA6V[a\x1D\xDDV[\x92\x91\x90\x83a\x1C`a\x1Ce\x92a\x1C`` \x86\x01Q\x86Q\x90a\x1D\xDDV[a\x1D\xFFV[\x90a\x1Cq\x81\x83\x86a\x16\x86V[\x93a\x1C~\x82\x86\x85\x84a\x14bV[\x85\x90`\0\x80\x82\x12\x15a\x1DGW[\x80\x82\x12a\x1D)WPa\x1C\xD0a\x1D\x1D\x92a\x047\x96\x97\x98\x86\x93[a\x1C\xB7`@Q\x98\x89\x92\x8C\x8A` \x86\x01a\"hV[\x03\x96a\x1C\xCB`\x1F\x19\x98\x89\x81\x01\x83R\x82a\r\x87V[a\x1ElV[\x81Q`@\x80\x84\x01Q``\x94\x85\x01Q\x82Q` \x81\x01\x98\x90\x98R\x91\x87\x01\x99\x90\x99R\x92\x85\x01\x91\x90\x91R`\x80\x84\x01R`\xA0\x83\x01\x95\x90\x95R`\x01`\x01`\xA0\x1B\x03\x90\x94\x16`\xC0\x82\x01R\x92\x83\x90`\xE0\x82\x01\x90V[\x03\x90\x81\x01\x83R\x82a\r\x87V[\x96a\x1D4\x91Pa\x1E V[\x95a\x1DA\x84\x88\x87\x86a\x14bV[\x90a\x1C\x8BV[\x96\x91\x96[\x80\x82\x13a\x1DgWPa\x1C\xD0a\x047\x95\x96\x97a\x1D\x1D\x93\x86\x93a\x1C\xA3V[\x96a\x1Dr\x91Pa \x05V[\x95a\x1D\x7F\x84\x88\x87\x86a\x14bV[\x90a\x1DKV[` a\x1D\x9Ea\x047\x94\x93a\x18\xC7a\x18\xD4\x94\x86Q\x90a\x16\xC8V[\x92\x01Qa\x1F\xE3V[`\x01\x81\x80\x04\x14\x81\x15\x17`\x01\x16\x15a\x06\x9FW`d\x90\x04\x90V[`\x01\x81\x15\x15\x16\x15a\x06\x9FWn\xC0\x97\xCE{\xC9\x07\x15\xB3K\x9F\x10\0\0\0\0\x04\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17\x82\x15\x15\x16\x15a\x06\x9FW\x04\x90V[\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x06\x9FWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x04\x90V[a\x03\xE7\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x06\x9FWa\x03\xE8\x90\x04\x90V[\x91\x90a\x01\0\x83\x82\x03\x12a\x03\xD4W\x82Q\x92` \x81\x01Q\x92a\x047`@\x83\x01Q\x93`\x80``\x85\x01Q\x94\x01a\x12\x10V[`\0\x93\x92\x91\x84\x91\x83\x82\x11a\x1FgWa\x1E\x84\x82\x82a#\x80V[a\x1E\x8E\x85\x83a#\x80V[`\0a\x1E\x9A\x82\x84a\x16\xA5V[\x13a\x1FHWPa\x1E\xAC\x83\x86\x97\x96a\x10\x15V[`\x01\x94`\0\x91\x86\x80[a\x1E\xC6W[PPPPPPPP\x90PV[\x15a\x1F#W[P\x85\x96\x97\x98P\x80\x91a\x1E\xE1a\x05\xB6\x8B\x88a\x0F\x9BV[\x99a\x1E\xEC\x8B\x87a#\x80V[\x90\x83a\x1E\xF8\x87\x84a\x16\xA5V[\x13a\x1F\x17WPP\x89\x92[\x87a\x1F\r\x88\x86a\x10\x15V[\x92\x01\x93\x99\x98a\x1E\xB5V[\x8B\x97P\x90\x94P\x92a\x1F\x02V[\x86\x10\x80a\x1F=W[\x15a\x1F6W\x88a\x1E\xCCV[\x80\x80a\x1E\xBAV[Pa\x01\0\x82\x10a\x1F+V[`@Qc\x06\xF1\xBE]`\xE2\x1B\x81R`\x04\x81\x01\x92\x90\x92R`$\x82\x01R`D\x90\xFD[`@Qc0\x82\xDF\xDB`\xE1\x1B\x81R`\x04\x81\x01\x83\x90R`$\x81\x01\x85\x90R`D\x90\xFD[\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x06\x9FW`\x01g\r\xE0\xB6\xB3\xA7d\0\0`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[\x90g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x81\x02\x92\x81\x84\x04\x14\x90\x15\x17\x81\x15\x15\x16\x15a\x06\x9FW`\x01\x90`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[`\x01\x81\x15\x15\x16\x15a\x06\x9FWn\xC0\x97\xCE{\xC9\x07\x15\xB3K\x9F\x0F\xFF\xFF\xFF\xFF\x04`\x01\x01\x90V[a\x03\xE9\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x06\x9FW`\x01a\x03\xE8`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[\x15a 6WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x90\xFD[h\x02H\xCE6\xA7\x0C\xB2k>\x19\x81\x13\x15a!\xEBWh\x07U\xBFy\x8BJ\x1B\xF1\xE5\x81\x12\x15a!\xB7We\x03x-\xAC\xE9\xD9\x90`N\x1B\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x91``\x90`\x01`_\x1B\x84\x82\x84\x1B\x05\x01\x82\x1D\x93\x84\x02\x90\x03\x80l\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x81\x01\x02\x82\x1D\x90n\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dPn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x82n\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x81m\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x81m\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x81m\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x81l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x02\x8D\x1D\x01\x02\x8B\x1D\x01\x02\x89\x1D\x01\x02\x87\x1D\x01\x02\x85\x1D\x01\x93m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x93m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEA\x81\x01\x90\x84m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x91\x01\x01\x02\x90\x1D\x01\x02\x01\x05\x02\x90`\xC3\x03\x1C\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x90\xFD[P`\0\x90V[a\"4a\" \x91a\x14\xA4\x93`@Q\x93\x84\x92c-\x83\x9C\xB3`\xE2\x1B` \x85\x01R`@`$\x85\x01R`d\x84\x01\x90a\x04\x01V[\x90`D\x83\x01R\x03`\x1F\x19\x81\x01\x83R\x82a\r\x87V[a##V[a\"4a\" \x91a\x14\xA4\x93`@Q\x93\x84\x92c\x1ES\x13G`\xE1\x1B` \x85\x01R`@`$\x85\x01R`d\x84\x01\x90a\x04\x01V[a\x14\xA4\x93``\x92\x96\x95\x93`\xE0\x83\x01\x97\x83R` \x83\x01R`@\x82\x01R\x01\x90``\x90\x80Q\x83R` \x81\x01Q` \x84\x01R`@\x81\x01Q`@\x84\x01R\x81`\x01\x80`\xA0\x1B\x03\x91\x01Q\x16\x91\x01RV[a\"\xBC\x81\x15\x15a /V[\x80o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x07\x1B\x81\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x81\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x81\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x81\x81\x1C`\xFF\x10`\x03\x1B\x17\x81\x81\x1C`\x0F\x10`\x02\x1B\x17\x81\x81\x1C`\x03\x10`\x01\x1B\x17\x90\x81\x1C`\x01\x10\x17\x90V[`\0\x80\x91` \x81Q\x91\x01jconsole.logZ\xFAPV[\x90a#Ua\x047\x92` \x80\x82Q\x83\x01\x01\x91\x01a\x1E?V[\x94\x93\x92\x90\x92a\x14\xEDV[\x90a#va\x047\x92` \x80\x82Q\x83\x01\x01\x91\x01a\x1E?V[\x94\x93\x92\x90\x92a\x1A\x13V[\x80Q\x81\x01\x91`\xE0\x82\x84\x03\x12a\x03\xD4Wa\x047\x92a#\xAE` \x84\x01Q\x93`\x80` `@\x83\x01Q\x94\x01\x91\x01a\x12\x10V[\x92a\x14bV\xFE\xA2dipfsX\"\x12 \x0F\x88xl\x94f-.\x97{\xB6DM\xD9\xF7JsM~\xB6\x98\xF8$f\xAF\xE2\xC8\xFC\xEE\xE5/\xD2dsolcC\0\x08\x16\x003";
    /// The deployed bytecode of the contract.
    pub static G3MSOLVER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct G3MSolver<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for G3MSolver<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for G3MSolver<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for G3MSolver<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for G3MSolver<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(G3MSolver)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> G3MSolver<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    G3MSOLVER_ABI.clone(),
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
                G3MSOLVER_ABI.clone(),
                G3MSOLVER_BYTECODE.clone().into(),
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
        ) -> ::ethers::contract::builders::ContractCall<M, G3Mparams> {
            self.0
                .method_hash([129, 181, 250, 194], pool_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getInitialPoolData` (0xdef15f92) function
        pub fn get_initial_pool_data(
            &self,
            rx: ::ethers::core::types::U256,
            s: ::ethers::core::types::U256,
            params: G3Mparams,
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
    for G3MSolver<M> {
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
    pub enum G3MSolverErrors {
        BisectionLib_InvalidBounds(BisectionLib_InvalidBounds),
        BisectionLib_RootOutsideBounds(BisectionLib_RootOutsideBounds),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for G3MSolverErrors {
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
    impl ::ethers::core::abi::AbiEncode for G3MSolverErrors {
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
    impl ::ethers::contract::ContractRevert for G3MSolverErrors {
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
    impl ::core::fmt::Display for G3MSolverErrors {
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
    impl ::core::convert::From<::std::string::String> for G3MSolverErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<BisectionLib_InvalidBounds> for G3MSolverErrors {
        fn from(value: BisectionLib_InvalidBounds) -> Self {
            Self::BisectionLib_InvalidBounds(value)
        }
    }
    impl ::core::convert::From<BisectionLib_RootOutsideBounds> for G3MSolverErrors {
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
        pub params: G3Mparams,
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
    pub enum G3MSolverCalls {
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
    impl ::ethers::core::abi::AbiDecode for G3MSolverCalls {
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
    impl ::ethers::core::abi::AbiEncode for G3MSolverCalls {
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
    impl ::core::fmt::Display for G3MSolverCalls {
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
    impl ::core::convert::From<AllocateGivenXCall> for G3MSolverCalls {
        fn from(value: AllocateGivenXCall) -> Self {
            Self::AllocateGivenX(value)
        }
    }
    impl ::core::convert::From<AllocateGivenYCall> for G3MSolverCalls {
        fn from(value: AllocateGivenYCall) -> Self {
            Self::AllocateGivenY(value)
        }
    }
    impl ::core::convert::From<CalculateDiffLowerCall> for G3MSolverCalls {
        fn from(value: CalculateDiffLowerCall) -> Self {
            Self::CalculateDiffLower(value)
        }
    }
    impl ::core::convert::From<CalculateDiffRaiseCall> for G3MSolverCalls {
        fn from(value: CalculateDiffRaiseCall) -> Self {
            Self::CalculateDiffRaise(value)
        }
    }
    impl ::core::convert::From<CheckSwapConstantCall> for G3MSolverCalls {
        fn from(value: CheckSwapConstantCall) -> Self {
            Self::CheckSwapConstant(value)
        }
    }
    impl ::core::convert::From<ComputeOptimalArbLowerPriceCall> for G3MSolverCalls {
        fn from(value: ComputeOptimalArbLowerPriceCall) -> Self {
            Self::ComputeOptimalArbLowerPrice(value)
        }
    }
    impl ::core::convert::From<ComputeOptimalArbRaisePriceCall> for G3MSolverCalls {
        fn from(value: ComputeOptimalArbRaisePriceCall) -> Self {
            Self::ComputeOptimalArbRaisePrice(value)
        }
    }
    impl ::core::convert::From<DeallocateGivenXCall> for G3MSolverCalls {
        fn from(value: DeallocateGivenXCall) -> Self {
            Self::DeallocateGivenX(value)
        }
    }
    impl ::core::convert::From<DeallocateGivenYCall> for G3MSolverCalls {
        fn from(value: DeallocateGivenYCall) -> Self {
            Self::DeallocateGivenY(value)
        }
    }
    impl ::core::convert::From<FetchPoolParamsCall> for G3MSolverCalls {
        fn from(value: FetchPoolParamsCall) -> Self {
            Self::FetchPoolParams(value)
        }
    }
    impl ::core::convert::From<GetInitialPoolDataCall> for G3MSolverCalls {
        fn from(value: GetInitialPoolDataCall) -> Self {
            Self::GetInitialPoolData(value)
        }
    }
    impl ::core::convert::From<GetNextLiquidityCall> for G3MSolverCalls {
        fn from(value: GetNextLiquidityCall) -> Self {
            Self::GetNextLiquidity(value)
        }
    }
    impl ::core::convert::From<GetNextReserveXCall> for G3MSolverCalls {
        fn from(value: GetNextReserveXCall) -> Self {
            Self::GetNextReserveX(value)
        }
    }
    impl ::core::convert::From<GetNextReserveYCall> for G3MSolverCalls {
        fn from(value: GetNextReserveYCall) -> Self {
            Self::GetNextReserveY(value)
        }
    }
    impl ::core::convert::From<GetReservesAndLiquidityCall> for G3MSolverCalls {
        fn from(value: GetReservesAndLiquidityCall) -> Self {
            Self::GetReservesAndLiquidity(value)
        }
    }
    impl ::core::convert::From<InternalPriceCall> for G3MSolverCalls {
        fn from(value: InternalPriceCall) -> Self {
            Self::InternalPrice(value)
        }
    }
    impl ::core::convert::From<PrepareControllerUpdateCall> for G3MSolverCalls {
        fn from(value: PrepareControllerUpdateCall) -> Self {
            Self::PrepareControllerUpdate(value)
        }
    }
    impl ::core::convert::From<PrepareFeeUpdateCall> for G3MSolverCalls {
        fn from(value: PrepareFeeUpdateCall) -> Self {
            Self::PrepareFeeUpdate(value)
        }
    }
    impl ::core::convert::From<PrepareWeightXUpdateCall> for G3MSolverCalls {
        fn from(value: PrepareWeightXUpdateCall) -> Self {
            Self::PrepareWeightXUpdate(value)
        }
    }
    impl ::core::convert::From<SimulateSwapCall> for G3MSolverCalls {
        fn from(value: SimulateSwapCall) -> Self {
            Self::SimulateSwap(value)
        }
    }
    impl ::core::convert::From<StrategyCall> for G3MSolverCalls {
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
    pub struct FetchPoolParamsReturn(pub G3Mparams);
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
    ///`G3Mparams(uint256,uint256,uint256,address)`
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
    pub struct G3Mparams {
        pub w_x: ::ethers::core::types::U256,
        pub w_y: ::ethers::core::types::U256,
        pub swap_fee: ::ethers::core::types::U256,
        pub controller: ::ethers::core::types::Address,
    }
}
