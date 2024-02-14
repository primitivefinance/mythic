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
    const __BYTECODE: &[u8] = b"`\x80\x80`@R4a\0\xE6W`@Q`\x1Fa*\xD38\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x91`\x01`\x01`@\x1B\x03\x83\x11\x84\x84\x10\x17a\0\xD0W\x80\x84\x92` \x94`@R\x839\x81\x01\x03\x12a\0\x80WQ`\x01`\x01`\xA0\x1B\x03\x81\x16\x90\x81\x90\x03a\0{W`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x91\x90\x91\x17\x90U`@Qa)\x9F\x90\x81a\x014\x829\xF3[`\0\x80\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[bF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01Ra7\xB7`\xF1\x1B`d\x82\x01R`\x84\x90\xFD\xFE`\x80`@R`\x046\x10a\x0CcW`\x005`\xE0\x1C\x80c\x0FAf\xB8\x14a\x01\\W\x80c%\th\xD9\x14a\x01WW\x80c0m\xB4k\x14a\x01RW\x80c3\"f\xF3\x14a\x01MW\x80c9(\xFF\x97\x14a\x01HW\x80c;M\x100\x14a\x01CW\x80cO\xD6|X\x14a\x01>W\x80cZ\x93\xB8\xCE\x14a\x019W\x80cb7V\x9F\x14a\x014W\x80c\x7F\x17@\x9C\x14a\x01/W\x80c\x81\xB5\xFA\xC2\x14a\x01*W\x80c\x90.\xCA\xA2\x14a\x01%W\x80c\xA8\xC6.v\x14a\x01 W\x80c\xB0\x9D\x04\xE5\x14a\x01\x1BW\x80c\xCB\x1FU2\x14a\x01\x16W\x80c\xCE\x15;\xF4\x14a\x01\x11W\x80c\xDE\xF1_\x92\x14a\x01\x0CW\x80c\xEC)\xD8\xE6\x14a\x01\x07W\x80c\xEE>\x8C\xFB\x14a\x01\x02W\x80c\xF2\xDEz{\x14a\0\xFDWc\xF3\r7\xF2\x03a\x0CcWa\x0C0V[a\x0C\x14V[a\x0B\xE0V[a\x0B\xCAV[a\x0BYV[a\n;V[a\t\xF6V[a\t\xB2V[a\t\x89V[a\t`V[a\t\x0CV[a\x08\xACV[a\x08KV[a\x08&V[a\x07\xFDV[a\x07\xCBV[a\x057V[a\x04\xDAV[a\x04\xA3V[a\x04:V[a\x02\xAAV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01Ra7\xB7`\xF1\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x90\xFD[4a\x03\xD9W`@6`\x03\x19\x01\x12a\x03\xD4Wg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`$5\x81\x81\x11a\x03\xCFW6`#\x82\x01\x12\x15a\x03\xCAW\x80`\x04\x015\x91\x82\x11a\x03qW6`$\x83\x83\x01\x01\x11a\x03\x18Wa\x03\x14\x91`$a\x03\x04\x92\x01`\x045a\x0C\xC6V[`@Q\x90\x81R\x90\x81\x90` \x82\x01\x90V[\x03\x90\xF3[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rjrray stride`\xA8\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x0ENL/$\r\x8C\xAD\xCC\xEE\x8D`\xAB\x1B`d\x82\x01R`\x84\x90\xFD[a\x02QV[a\x02\x01V[a\x01\xB1V[a\x01aV[`\0[\x83\x81\x10a\x03\xF1WPP`\0\x91\x01RV[\x81\x81\x01Q\x83\x82\x01R` \x01a\x03\xE1V[\x90` \x91a\x04\x1A\x81Q\x80\x92\x81\x85R\x85\x80\x86\x01\x91\x01a\x03\xDEV[`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[\x90` a\x047\x92\x81\x81R\x01\x90a\x04\x01V[\x90V[4a\x03\xD9W`@6`\x03\x19\x01\x12a\x03\xD4Wa\x03\x14`@Q`\x02` \x82\x01R`\x045`@\x82\x01R`$5``\x82\x01R``\x81Ra\x04u\x81a\n\xDEV[`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x04\x01V[``\x90`\x03\x19\x01\x12a\x03\xD4W`\x045\x90`$5\x90`D5\x90V[4a\x03\xD9W` a\x04\xD2a\x04\xB66a\x04\x89V[\x90a\x04\xC9a\x04\xC3\x84a\x10IV[\x93a\x11#V[\x92\x91\x90\x91a\x12\xD5V[`@Q\x90\x81R\xF3[4a\x03\xD9W` a\x04\xD2a\x04\xED6a\x04\x89V[\x90a\x04\xFAa\x04\xC3\x84a\x10IV[\x92\x91\x90\x91a\x14\xEEV[\x80\x15\x15\x03a\x05\rWV[`\0\x80\xFD[\x90\x92`\x80\x92a\x047\x95\x94\x15\x15\x83R` \x83\x01R`@\x82\x01R\x81``\x82\x01R\x01\x90a\x04\x01V[4a\x03\xD9W``6`\x03\x19\x01\x12a\x03\xD4W`\x045`$5a\x05W\x81a\x05\x03V[`D5\x91a\x05ca\x0C\xF2V[\x91a\x05la\x0C\xF2V[\x92a\x05v\x83a\x11#V[\x92\x90\x91\x93` \x94\x85\x83\x01\x93`@\x95\x86\x85\x01R\x84R\x82Ra\x05\x95\x86a\x10IV[\x97\x85\x89\x86a\x05\xAF\x86Q\x88Qa\x05\xA9\x8Da\x10IV[\x91a\x16\x87V[\x94\x15a\x07EWa\x06\r\x93a\x05\xFFa\x05\xFAa\x06A\x99\x98\x95a\x05\xF4\x86a\x05\xDDa\x06\x06\x97a\x06\x1A\x9C\x99\x01Q\x87a#\"V[\x92a\x05\xEB\x8DQ\x8BQ\x90a#NV[\x91\x01Q\x90a\x16\xC9V[\x90a#\"V[a\r8V[\x93Qa\r[V[\x8ARa\r[V[\x80\x85\x89\x01R\x87Q\x87a\x12\x1CV[\x90a\x068a\x06-\x86\x89\x01\x93\x80\x85Ra\r8V[\x80\x84R\x82Q\x11a\x0E\x02V[Q\x90Q\x90a\r\xF5V[\x94[\x84Q\x83\x86\x01\x80Q\x84\x88\x01Q\x85Q\x96\x87\x01\x93\x84R` \x84\x01\x91\x90\x91R`@\x83\x01R\x90a\x06{\x90\x85\x90``\x01\x03`\x1F\x19\x81\x01\x86R\x85a\x0B7V[`\0Ta\x06\x9E\x90a\x06\x92\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x80;\x15a\x07@W\x84`\xC0\x91a\x06\xC9\x97\x86Q\x80\x99\x81\x94\x82\x93c\r\x17\xA7\xC7`\xE3\x1B\x84R0`\x04\x85\x01a\x0E\xE3V[\x03\x91Z\xFA\x94\x85\x15a\x07;W`\0\x95a\x06\xFBW[P\x90a\x06\xF0\x91a\x03\x14\x95\x96Q\x90Q\x90a\x18\x89V[\x90Q\x94\x85\x94\x85a\x05\x12V[a\x03\x14\x95P\x90a\x07&a\x06\xF0\x93\x92`\xC0=`\xC0\x11a\x074W[a\x07\x1E\x81\x83a\x0B7V[\x81\x01\x90a\x0E\xACV[PPPPP\x95P\x90\x91a\x06\xDCV[P=a\x07\x14V[a\x0F\x07V[a\x0EYV[a\x07\xA9\x92Pa\x07\xC5\x96a\x07\x90a\x07\xB1\x95a\x07\x89a\x05\xFA\x8Aa\x05\xF4\x88a\x07\x81a\x07wa\x07\x9C\x9Aa\x07\xBC\x9F\x9C\x01Q\x88a#\"V[\x93Q\x89Q\x90a#NV[\x90Q\x90a\x16\xC9V[\x92Qa\r[V[\x92\x8A\x8D\x01\x93\x84Ra\r[V[\x90\x81\x88\x8C\x01RQ\x89a\x0F\x13V[\x80\x89Ra\r8V[\x80\x88R\x82Q\x11a\rhV[Q\x85Q\x90a\r\xF5V[\x94a\x06CV[4a\x03\xD9W` 6`\x03\x19\x01\x12a\x03\xD4W` a\x04\xD2`\x045a\x07\xF6a\x07\xF0\x82a\x10IV[\x91a\x11#V[P\x90a\x18\x89V[4a\x03\xD9W` a\x04\xD2a\x08\x106a\x04\x89V[\x90a\x08\x1Da\x04\xC3\x84a\x10IV[\x92\x91\x90\x91a\x18\xAEV[4a\x03\xD9W` a\x04\xD2a\x08Ea\x08<6a\x04\x89V[\x91\x92\x90\x92a\x10IV[\x91a\x1A\x1CV[4a\x03\xD9W`@6`\x03\x19\x01\x12a\x03\xD4Wa\x08\x87`\x045a\x03\x14a\x08\x8Ea\x08\x7Fa\x08t\x84a\x11#V[\x91\x90P`$5a\x1AIV[\x94\x90\x93a\x10IV[\x84\x84a \xC1V[\x92`@Q\x93\x84\x93\x84`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[4a\x03\xD9W`@6`\x03\x19\x01\x12a\x03\xD4W`\x045a\x08\xE7a\x03\x14a\x08\xEEa\x08\xDFa\x08\xD5\x85a\x11#V[\x91P`$5a\x1AvV[\x93\x90\x94a\x10IV[\x83\x85a\x1A\x1CV[\x91`@Q\x93\x84\x93\x84`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[4a\x03\xD9W` 6`\x03\x19\x01\x12a\x03\xD4W`\x80a\t*`\x045a\x10IV[a\t^`@Q\x80\x92``\x90\x80Q\x83R` \x81\x01Q` \x84\x01R`@\x81\x01Q`@\x84\x01R\x81`\x01\x80`\xA0\x1B\x03\x91\x01Q\x16\x91\x01RV[\xF3[4a\x03\xD9W` a\x04\xD2a\ts6a\x04\x89V[\x90a\t\x80a\x04\xC3\x84a\x10IV[\x92\x91\x90\x91a\x1D\xC5V[4a\x03\xD9W`\x006`\x03\x19\x01\x12a\x03\xD4W`\0T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x03\xD9W` 6`\x03\x19\x01\x12a\x03\xD4Wa\x03\x14`@Q`\x01` \x82\x01R`\x045`@\x82\x01R`@\x81Ra\x04u\x81a\n\xFFV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\x05\rWV[4a\x03\xD9W` 6`\x03\x19\x01\x12a\x03\xD4Wa\x03\x14`\x045a\n\x16\x81a\t\xE5V[`@\x80Q`\x03` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x82\x82\x01R\x81Ra\x04u\x81a\n\xFFV[4a\x03\xD9W` 6`\x03\x19\x01\x12a\x03\xD4Wa\x03\x14a\nZ`\x045a\x11#V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R\x90\x81\x90``\x82\x01\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FABI decoding: struct data too sh`D\x82\x01Rb\x1B\xDC\x9D`\xEA\x1B`d\x82\x01R`\x84\x90\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\x80\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\n\xFAW`@RV[a\n\xC8V[``\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\n\xFAW`@RV[`@\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\n\xFAW`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\n\xFAW`@RV[4a\x03\xD9W`\xC06`\x03\x19\x01\x12a\x03\xD4W`\x806`C\x19\x01\x12a\x0B\xC5Wa\x03\x14a\x0B\xB9`@Qa\x0B\x88\x81a\n\xDEV[`D5\x81R`d5` \x82\x01R`\x845`@\x82\x01R`\xA45a\x0B\xA9\x81a\t\xE5V[``\x82\x01R`$5`\x045a\x1F\x86V[`@Q\x91\x82\x91\x82a\x04&V[a\nwV[4a\x03\xD9W` a\x04\xD2a\x05\xA9a\x08<6a\x04\x89V[4a\x03\xD9W`@6`\x03\x19\x01\x12a\x03\xD4Wa\x08\x87`\x045a\x03\x14a\x08\x8Ea\x08\x7Fa\x0C\t\x84a\x11#V[\x91\x90P`$5a\x1AvV[4a\x03\xD9W` a\x04\xD2a\x0C*a\x08<6a\x04\x89V[\x91a \xC1V[4a\x03\xD9W`@6`\x03\x19\x01\x12a\x03\xD4W`\x045a\x08\xE7a\x03\x14a\x08\xEEa\x08\xDFa\x0CY\x85a\x11#V[\x91P`$5a\x1AIV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01Rtnor receive functions`X\x1B`d\x82\x01R`\x84\x90\xFD[\x91\x81``\x91\x81\x01\x03\x12a\x03\xD4Wa\x0C\xDFa\x047\x92a\x10IV[\x90`@\x81\x015\x90` \x81\x015\x905a\x12BV[`@Q\x90``\x82\x01\x82\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\n\xFAW`@R`\0`@\x83\x82\x81R\x82` \x82\x01R\x01RV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x90`\x01\x82\x01\x80\x92\x11a\rFWV[a\r\"V[\x90a\x03\xE8\x91\x82\x01\x80\x92\x11a\rFWV[\x91\x90\x82\x01\x80\x92\x11a\rFWV[\x15a\roWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: x reserve increase`D\x82\x01Rad!`\xF0\x1B`d\x82\x01R`\x84\x90\xFD[\x90g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x03\x91\x82\x11a\rFWV[\x90a\x03\xE8\x91\x82\x03\x91\x82\x11a\rFWV[a\x03\xE7\x19\x81\x01\x91\x90\x82\x11a\rFWV[\x91\x90\x82\x03\x91\x82\x11a\rFWV[\x15a\x0E\tWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: y reserve increase`D\x82\x01Rad!`\xF0\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01Rd code`\xD8\x1B`d\x82\x01R`\x84\x90\xFD[\x91\x90\x82`\xC0\x91\x03\x12a\x03\xD4W\x81Qa\x0E\xC3\x81a\x05\x03V[\x91` \x81\x01Q\x91`@\x82\x01Q\x91``\x81\x01Q\x91`\xA0`\x80\x83\x01Q\x92\x01Q\x90V[a\x047\x93\x92``\x92`\x01\x80`\xA0\x1B\x03\x16\x82R` \x82\x01R\x81`@\x82\x01R\x01\x90a\x04\x01V[`@Q=`\0\x82>=\x90\xFD[\x91a\x08Ea\x047\x93a\x10IV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\n\xFAW`\x1F\x01`\x1F\x19\x16` \x01\x90V[` \x81\x83\x03\x12a\x03\xD4W\x80Q\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x03\xCFW\x01\x81`\x1F\x82\x01\x12\x15a\x03\xCAW\x80Qa\x0Fo\x81a\x0F V[\x92a\x0F}`@Q\x94\x85a\x0B7V[\x81\x84R` \x82\x84\x01\x01\x11a\x0F\x9BWa\x047\x91` \x80\x85\x01\x91\x01a\x03\xDEV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R`\x84\x90\xFD[\x91\x90\x82`\x80\x91\x03\x12a\x0B\xC5W`@Qa\x10\x08\x81a\n\xDEV[``\x80\x82\x94\x80Q\x84R` \x81\x01Q` \x85\x01R`@\x81\x01Q`@\x85\x01R\x01Q\x91a\x101\x83a\t\xE5V[\x01RV[\x90`\x80\x82\x82\x03\x12a\x03\xD4Wa\x047\x91a\x0F\xF0V[``\x90`@Qa\x10X\x81a\n\xDEV[`\0\x80\x82R` \x82\x01\x81\x90R`@\x82\x01\x81\x90R\x92\x01\x82\x90R\x81Ta\x10\x86\x90a\x06\x92\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x80;\x15a\x07@W`@Qc\xDC\x17\x83U`\xE0\x1B\x81R`\x04\x81\x01\x92\x90\x92R\x82\x90\x82\x90`$\x90\x82\x90Z\xFA\x90\x81\x15a\x07;W\x82a\x047\x93\x92a\x10\xD0W[PP` \x80\x82Q\x83\x01\x01\x91\x01a\x105V[a\x10\xEC\x92P=\x80\x91\x83>a\x10\xE4\x81\x83a\x0B7V[\x81\x01\x90a\x0F<V[8\x80a\x10\xBFV[\x90\x81` \x91\x03\x12a\x03\xD4WQa\x047\x81a\t\xE5V[\x90\x81``\x91\x03\x12a\x03\xD4W\x80Q\x91`@` \x83\x01Q\x92\x01Q\x90V[`\0T\x90\x91\x90a\x11=\x90a\x06\x92\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x80;\x15a\x07@W` `\x04\x91`@Q\x92\x83\x80\x92c+\xEE\x84\xF1`\xE2\x1B\x82RZ\xFA\x90\x81\x15a\x07;W`\0\x91a\x11\xEDW[P`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x07@W`@Qc3\x85N\xFD`\xE2\x1B\x81R`\x04\x81\x01\x93\x90\x93R``\x90\x83\x90`$\x90\x82\x90Z\xFA\x90\x81\x15a\x07;W`\0\x80\x93`\0\x93a\x11\xB6W[P\x92\x91\x90V[\x91\x93PPa\x11\xDC\x91P``=``\x11a\x11\xE6W[a\x11\xD4\x81\x83a\x0B7V[\x81\x01\x90a\x11\x08V[\x92\x90\x92\x918a\x11\xB0V[P=a\x11\xCAV[a\x12\x0F\x91P` =` \x11a\x12\x15W[a\x12\x07\x81\x83a\x0B7V[\x81\x01\x90a\x10\xF3V[8a\x11kV[P=a\x11\xFDV[\x91a\x0C*a\x047\x93a\x10IV[\x81\x81\x03\x92\x91`\0\x13\x80\x15\x82\x85\x13\x16\x91\x84\x12\x16\x17a\rFWV[\x92` a\x05\xEB\x84a\x12ea\x12]a\x05\xF4\x96\x97a\x12k\x99a&kV[\x85Q\x90a\x16\xC9V[\x95a&kV[g\r\xE0\xB6\xB3\xA7c\xFF\xFF\x19\x81\x01\x90\x81\x13`\x01\x16a\rFW\x90V[\x90\x81R` \x80\x82\x01\x92\x90\x92R`@\x80\x82\x01\x93\x90\x93R``\x80\x82\x01\x94\x90\x94R\x84Q`\x80\x82\x01R\x90\x84\x01Q`\xA0\x82\x01R\x90\x83\x01Q`\xC0\x82\x01R\x91\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\xE0\x82\x01Ra\x01\0\x01\x90V[V[\x90\x92\x91\x85Q` \x87\x01Q`@\x88\x01Qa\x12\xED\x90a\r\xBFV[\x91a\x12\xF8\x87\x85a&kV[a\x13\x02\x82\x82a\x16\xC9V[\x92a\x13\x0C\x91a\x16\xC9V[\x89Q\x85\x89\x85\x81a\x13\x1C\x85\x8Da&\xAEV[\x90a\x13&\x91a&\xAEV[\x90a\x130\x91a&\xAEV[\x92a\x13:\x90a&\x8DV[a\x13C\x90a\r\xD5V[\x90a\x13M\x91a\r[V[\x90a\x13W\x91a&\xAEV[a\x13`\x86a\r\xBFV[a\x13i\x91a&\xAEV[\x92a\x13s\x8Aa\rKV[\x90a\x13}\x90a\x14\xA7V[a\x13\x86\x91a\x16\xC9V[\x91a\x13\x90\x90a&\x8DV[a\x13\x99\x86a\r\xBFV[a\x13\xA2\x91a&\xAEV[a\x13\xAC\x90\x89a\r[V[\x92a\x13\xB6\x91a\r\xF5V[\x91a\x13\xC0\x91a&\xAEV[\x89Qa\x13\xCB\x90a\r\xBFV[a\x13\xD4\x90a&LV[a\x13\xDD\x91a\x16\xC9V[a\x13\xE6\x91a&\xAEV[\x91\x88Qa\x13\xF2\x90a\r\xBFV[a\x13\xFB\x88a\rKV[\x92a\x14\x06\x89\x89a&\xAEV[\x90a\x14\x10\x91a&\xAEV[\x91a\x14\x1A\x86a&\x8DV[\x90a\x14$\x90a\r\xBFV[a\x14-\x91a&\xAEV[\x92a\x147\x91a&\xAEV[\x91a\x14A\x91a\r[V[a\x14J\x91a&\xAEV[\x90a\x14T\x84a\x14\xA7V[\x91a\x14^\x91a&kV[a\x14g\x91a\x14\xD2V[`\0\x13a\x14\x9CWa\x047\x95a\x14\x97\x93a\x14\x89\x92`@Q\x96\x87\x95` \x87\x01a\x12\x84V[\x03`\x1F\x19\x81\x01\x83R\x82a\x0B7V[a!\x0FV[PPPPPP`\0\x90V[`\x01`\xFF\x1B\x81\x14a\rFW`\0\x03\x90V[\x90\x81g\r\xE0\xB6\xB3\xA7c\xFF\xFF\x19\x01\x91\x82\x12\x15`\x01\x16a\rFWV[\x91\x90\x91`\0\x83\x82\x01\x93\x84\x12\x91\x12\x90\x80\x15\x82\x16\x91\x15\x16\x17a\rFWV[\x94\x93\x92\x90\x92\x84Q\x90` \x86\x01Q`@\x87\x01Qa\x15\t\x90a\r\xBFV[\x92a\x15\x14\x87\x87a&kV[a\x15\x1E\x82\x82a\x16\xC9V[\x92a\x15(\x91a\x16\xC9V[\x88Q\x87\x89\x85\x81a\x158\x85\x8Ca&\xAEV[\x90a\x15B\x91a&\xAEV[\x90a\x15L\x91a&\xAEV[\x92a\x15W\x90\x88a&\xAEV[a\x15a\x90\x88a\r\xF5V[\x90a\x15k\x91a\r[V[\x90a\x15u\x91a&\xAEV[a\x15~\x87a\r\xBFV[a\x15\x87\x91a&\xAEV[\x92a\x15\x92\x8A\x87a\r[V[\x90a\x15\x9C\x90a\x14\xA7V[a\x15\xA5\x91a\x16\xC9V[\x91a\x15\xB0\x90\x86a&\xAEV[a\x15\xB9\x87a\r\xBFV[a\x15\xC2\x91a&\xAEV[a\x15\xCC\x90\x88a\r[V[\x92a\x15\xD6\x91a\r\xF5V[\x91a\x15\xE0\x91a&\xAEV[\x88Qa\x15\xEB\x90a\r\xBFV[a\x15\xF4\x90a&LV[a\x15\xFD\x91a\x16\xC9V[a\x16\x06\x91a&\xAEV[\x96Qa\x16\x11\x90a\r\xBFV[\x93a\x16\x1C\x87\x84a\r[V[\x96a\x16&\x91a&\xAEV[\x90a\x160\x91a&\xAEV[\x93a\x16:\x91a&\xAEV[\x90a\x16D\x90a\r\xBFV[a\x16M\x91a&\xAEV[\x92a\x16W\x91a&\xAEV[\x91a\x16a\x91a\r[V[a\x16j\x91a&\xAEV[\x91a\x16t\x90a\x14\xA7V[\x91a\x16~\x91a&kV[a\x047\x91a\x14\xD2V[a\x047\x92\x91` a\x16\x9Da\x05\xF4\x93\x85Q\x90a\x16\xC9V[\x93\x01Q\x90a\x16\xC9V[\x81\x81\x02\x92\x91`\0\x82\x12`\x01`\xFF\x1B\x82\x14\x16a\rFW\x81\x84\x05\x14\x90\x15\x17\x15a\rFWV[a\x18va\x047\x92}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84a\x18\x84\x93a\x16\xFF`\0\x82\x13a#\xCAV[q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06a\x17\x1B\x82a&\xEEV[``\x92\x83\x82`\x9F\x03\x01\x1B`\x9F\x1C\x90`_\x19\x01}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x02\x92l\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x82m\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x81m\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x81m\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x81m\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x81m\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x81lFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x02\x8C\x1D\x01\x02\x8A\x1D\x01\x02\x88\x1D\x01\x02\x86\x1D\x01\x02\x84\x1D\x01\x02\x82\x1D\x01\x91x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x91l\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x90\x82m\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x81l\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x81m\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x81m\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x81l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x81\x01\x02\x89\x1D\x01\x02\x87\x1D\x01\x02\x85\x1D\x01\x02\x83\x1D\x01\x02\x90\x1D\x01\x02\x01\x05\x02\x01\x01`\xAE\x1Da\x16\xA6V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[a$\x02V[a\x18\xA8\x90a\x18\xA0a\x047\x94\x93` \x85\x01Q\x90a&kV[\x92Q\x90a&kV[\x90a&kV[\x91\x92\x94\x90\x93\x94`\0\x93\x84a\x18\xC5\x84\x84\x84\x8A\x89a\x1B\x82V[\x12a\x1A\x12W\x91a\x18\xE3\x91a\x18\xF1\x94\x93`@Q\x97\x88\x95` \x87\x01a\x12\x84V[\x03`\x1F\x19\x81\x01\x84R\x83a\x0B7V[\x82\x91a\x03\xE8\x90\x82\x94\x80\x83\x11a\x19\xF0Wa\x19\t\x82a'{V[\x90a\x19\x14\x81\x84a'\x9AV[\x85a\x19\x1F\x82\x85a\x16\xA6V[\x13a\x19\xCFWPa\x19.\x90a\r\xE5V[\x93\x80`\x01\x95\x86`\x01\x90[a\x19IW[PPPPPPPPP\x90V[\x15a\x19\xAAW[P\x85\x96\x97P\x80\x91a\x19ia\x19c\x8A\x88a\r[V[`\x01\x1C\x90V[\x98a\x19t\x8A\x87a'\x9AV[\x90\x83a\x19\x80\x87\x84a\x16\xA6V[\x13a\x19\x9EWPP\x88\x92[\x87a\x19\x95\x88\x86a\r\xF5V[\x92\x01\x93\x98a\x198V[\x8A\x97P\x90\x94P\x92a\x19\x8AV[\x86\x10\x80a\x19\xC4W[\x15a\x19\xBDW\x87a\x19OV[\x80\x80a\x19=V[Pa\x01\0\x82\x10a\x19\xB2V[`@Qc\x06\xF1\xBE]`\xE2\x1B\x81R`\x04\x81\x01\x84\x90R`$\x81\x01\x91\x90\x91R`D\x90\xFD[`@Qc0\x82\xDF\xDB`\xE1\x1B\x81Ra\x03\xE8`\x04\x82\x01R`$\x81\x01\x91\x90\x91R`D\x90\xFD[PPPP\x91PP\x90V[a\x1A<a\x047\x93\x92a\x1A6a\x1AC\x93` \x86\x01Q\x90a\x16\xC9V[\x90a#NV[\x91Qa#~V[\x90a\x16\xC9V[\x92\x91\x90a\x1A_a\x1AY\x82\x84a#NV[\x85a#\"V[\x93\x81\x03\x90\x81\x11a\rFW\x92\x81\x03\x90\x81\x11a\rFW\x90V[\x92\x91\x90a\x1A\x86a\x1AY\x82\x84a#NV[\x93\x81\x01\x80\x91\x11a\rFW\x92\x81\x01\x80\x91\x11a\rFW\x90V[`@Q\x90a\x1A\xAA\x82a\x0B\x1BV[`\x05\x82Rd\x19\x9A\\\x9C\xDD`\xDA\x1B` \x83\x01RV[`@Q\x90a\x1A\xCB\x82a\x0B\x1BV[`\x06\x82Re\x1C\xD9X\xDB\xDB\x99`\xD2\x1B` \x83\x01RV[`@Q\x90a\x1A\xED\x82a\x0B\x1BV[`\x05\x82Rd\x1D\x1A\x1A\\\x99`\xDA\x1B` \x83\x01RV[`@Q\x90a\x1B\x0E\x82a\x0B\x1BV[`\x06\x82Re\x0C\xCD\xEE\xAEN\x8D`\xD3\x1B` \x83\x01RV[`@Q\x90a\x1B0\x82a\x0B\x1BV[`\x05\x82Rd\x0C\xCD,\xCE\x8D`\xDB\x1B` \x83\x01RV[`@Q\x90a\x1BQ\x82a\x0B\x1BV[`\x03\x82Rbnum`\xE8\x1B` \x83\x01RV[`@Q\x90a\x1Bp\x82a\x0B\x1BV[`\x03\x82Rb22\xB7`\xE9\x1B` \x83\x01RV[\x90\x92\x82\x85Q\x80\x95`@\x88\x01Qa\x1B\x97\x90a\r\xBFV[\x95a\x1B\xA1\x91a&kV[\x90a\x1B\xAB\x91a\x16\xC9V[\x94a\x1B\xB5\x82a\rKV[\x90a\x1B\xBF\x90a\x14\xB8V[a\x1B\xC8\x91a\x16\xC9V[\x90a\x1B\xD2\x86a&\x8DV[\x90a\x1B\xDC\x86a\r\xBFV[a\x1B\xE6\x90\x83a&\xAEV[a\x1B\xF0\x90\x85a\r[V[\x88Qa\x1B\xFB\x83a\rKV[a\x1C\x04\x91a&\xAEV[\x93\x84a\x1C\x0Ea\x1A\x9DV[\x90a\x1C\x18\x91a%\x8CV[\x81\x80a\x1C\"a\x1A\xBEV[\x90a\x1C,\x91a%\x8CV[a\x1C5\x91a&\xAEV[\x89Qa\x1C@\x90a&LV[a\x1CI\x91a\x16\xC9V[\x91\x82a\x1CSa\x1A\xE0V[\x90a\x1C]\x91a%\x8CV[\x89Qa\x1Ch\x90a\r\xBFV[a\x1Cr\x90\x87a&\xAEV[\x98\x89a\x1C|a\x1B\x01V[\x90a\x1C\x86\x91a%\x8CV[\x81\x8BQa\x1C\x92\x90a&\x8DV[\x90a\x1C\x9C\x91a\r[V[a\x1C\xA5\x91a&\xAEV[a\x1C\xAE\x89a\r\xBFV[a\x1C\xB7\x91a&\xAEV[\x80a\x1C\xC0a\x1B#V[\x90a\x1C\xCA\x91a%\x8CV[a\x1Dh\x99a\x1D[\x98a\x1DU\x97a\x1D8\x96a\x1D>\x96\x88\x95\x80\x86\x10\x15a\x1D\x9BWa\x1D\x18\x92a\x1D\ra\x1D\x08\x96\x95\x93a\x1D\ra\x1D\x08a\x1D\x13\x95a\x1D\x1E\x9Ba\x12)V[a\x14\xA7V[\x94a&\xAEV[a&\xAEV[\x90a\x12)V[\x9A[a\x1D1\x8Ca\x1D,a\x1BDV[a%\xD4V[Q\x91a\rKV[\x90a&\xAEV[\x95a\x1DP\x87a\x1DKa\x1A\x9DV[a%\x8CV[a\r\xBFV[\x90a\r[V[\x90a\x1D\x13\x82a\x1DKa\x1A\xBEV[a\x1Dt\x81a\x1DKa\x1BcV[`\0\x82\x13\x15a\x1D\x8AWa\x1D\x08\x90a\x047\x92a&kV[a\x1D\x96a\x047\x92a\x14\xA7V[a&kV[a\x1D\xB9a\x1D\xBF\x96\x93a\x1D\xB3a\x1D8\x94a\x1D\x18\x97a&\xAEV[\x96a&\xAEV[\x92a\r\xF5V[\x9Aa\x1D V[\x91\x80\x93\x91\x94\x86Q\x80\x96`@\x89\x01Qa\x1D\xDC\x90a\r\xBFV[\x96a\x1D\xE6\x91a&kV[\x90a\x1D\xF0\x91a\x16\xC9V[\x95a\x1D\xFB\x83\x83a\r[V[\x90a\x1E\x05\x90a\x14\xB8V[a\x1E\x0E\x91a\x16\xC9V[\x91a\x1E\x19\x87\x83a&\xAEV[\x91a\x1E#\x87a\r\xBFV[a\x1E-\x90\x84a&\xAEV[a\x1E7\x90\x86a\r[V[\x90\x89Qa\x1ED\x84\x83a\r[V[a\x1EM\x91a&\xAEV[\x94\x85a\x1EWa\x1A\x9DV[\x90a\x1Ea\x91a%\x8CV[\x82\x80a\x1Eka\x1A\xBEV[\x90a\x1Eu\x91a%\x8CV[a\x1E~\x91a&\xAEV[\x8AQa\x1E\x89\x90a&LV[a\x1E\x92\x91a\x16\xC9V[\x92\x83a\x1E\x9Ca\x1A\xE0V[\x90a\x1E\xA6\x91a%\x8CV[\x8AQa\x1E\xB1\x90a\r\xBFV[a\x1E\xBB\x90\x88a&\xAEV[\x99\x8Aa\x1E\xC5a\x1B\x01V[\x90a\x1E\xCF\x91a%\x8CV[\x81\x8CQa\x1E\xDC\x90\x85a&\xAEV[\x90a\x1E\xE6\x91a\r[V[a\x1E\xEF\x91a&\xAEV[a\x1E\xF8\x8Aa\r\xBFV[a\x1F\x01\x91a&\xAEV[\x90\x81a\x1F\x0Ba\x1B#V[\x90a\x1F\x15\x91a%\x8CV[a\x1Dh\x9Aa\x1D[\x99a\x1DU\x98a\x1D8\x97a\x1D>\x97\x89\x96\x80\x86\x10\x15a\x1FhWa\x1D\x18\x92a\x1D\ra\x1D\x08\x96\x95\x93a\x1D\ra\x1D\x08a\x1D\x13\x95a\x1FS\x9Ba\x12)V[\x9B[a\x1Fa\x8Da\x1D,a\x1BDV[Q\x92a\r[V[a\x1D\xB9a\x1F\x80\x96\x93a\x1D\xB3a\x1D8\x94a\x1D\x18\x97a&\xAEV[\x9Ba\x1FUV[\x92\x91\x90\x83a\x1D\x13a\x1F\xA1\x92a\x1D\x13` \x86\x01Q\x86Q\x90a&kV[\x90a\x1F\xAD\x81\x83\x86a\x16\x87V[\x93a\x1F\xBA\x82\x86\x85\x84a\x12BV[\x85\x90`\0\x80\x82\x12\x15a \x83W[\x80\x82\x12a eWPa \x0Ca Y\x92a\x047\x96\x97\x98\x86\x93[a\x1F\xF3`@Q\x98\x89\x92\x8C\x8A` \x86\x01a&\x03V[\x03\x96a \x07`\x1F\x19\x98\x89\x81\x01\x83R\x82a\x0B7V[a\".V[\x81Q`@\x80\x84\x01Q``\x94\x85\x01Q\x82Q` \x81\x01\x98\x90\x98R\x91\x87\x01\x99\x90\x99R\x92\x85\x01\x91\x90\x91R`\x80\x84\x01R`\xA0\x83\x01\x95\x90\x95R`\x01`\x01`\xA0\x1B\x03\x90\x94\x16`\xC0\x82\x01R\x92\x83\x90`\xE0\x82\x01\x90V[\x03\x90\x81\x01\x83R\x82a\x0B7V[\x96a p\x91Pa&\xCFV[\x95a }\x84\x88\x87\x86a\x12BV[\x90a\x1F\xC7V[\x96\x91\x96[\x80\x82\x13a \xA3WPa \x0Ca\x047\x95\x96\x97a Y\x93\x86\x93a\x1F\xDFV[\x96a \xAE\x91Pa#\xA0V[\x95a \xBB\x84\x88\x87\x86a\x12BV[\x90a \x87V[` a \xDAa\x047\x94\x93a\x1A6a\x1AC\x94\x86Q\x90a\x16\xC9V[\x92\x01Qa#~V[\x91\x90a\x01\0\x83\x82\x03\x12a\x03\xD4W\x82Q\x92` \x81\x01Q\x92a\x047`@\x83\x01Q\x93`\x80``\x85\x01Q\x94\x01a\x0F\xF0V[\x91\x90a\x03\xE8\x92`\0\x93`\0\x91\x83\x82\x11a\"\rWa!+\x81a'\xBBV[a!5\x85\x83a)\x14V[`\0a!A\x82\x84a\x16\xA6V[\x13a!\xEEWPa!R\x85\x96\x95a\r\xE5V[`\x01\x94`\0\x91\x86\x80[a!lW[PPPPPPPP\x90PV[\x15a!\xC9W[P\x85\x96\x97\x98P\x80\x91a!\x87a\x19c\x8B\x88a\r[V[\x99a!\x92\x8B\x87a)\x14V[\x90\x83a!\x9E\x87\x84a\x16\xA6V[\x13a!\xBDWPP\x89\x92[\x87a!\xB3\x88\x86a\r\xF5V[\x92\x01\x93\x99\x98a![V[\x8B\x97P\x90\x94P\x92a!\xA8V[\x86\x10\x80a!\xE3W[\x15a!\xDCW\x88a!rV[\x80\x80a!`V[Pa\x01\0\x82\x10a!\xD1V[`@Qc\x06\xF1\xBE]`\xE2\x1B\x81R`\x04\x81\x01\x92\x90\x92R`$\x82\x01R`D\x90\xFD[`@Qc0\x82\xDF\xDB`\xE1\x1B\x81Ra\x03\xE8`\x04\x82\x01R`$\x81\x01\x85\x90R`D\x90\xFD[`\0\x93\x92\x91\x84\x91\x83\x82\x11a#\x02Wa\"F\x82\x82a)5V[a\"P\x85\x83a)5V[`\0a\"\\\x82\x84a\x16\xA6V[\x13a!\xEEWPa\"n\x83\x86\x97\x96a\r\xF5V[`\x01\x94`\0\x91\x86\x80[a\"\x87WPPPPPPPP\x90PV[\x15a\"\xE4W[P\x85\x96\x97\x98P\x80\x91a\"\xA2a\x19c\x8B\x88a\r[V[\x99a\"\xAD\x8B\x87a)5V[\x90\x83a\"\xB9\x87\x84a\x16\xA6V[\x13a\"\xD8WPP\x89\x92[\x87a\"\xCE\x88\x86a\r\xF5V[\x92\x01\x93\x99\x98a\"wV[\x8B\x97P\x90\x94P\x92a\"\xC3V[\x86\x10\x80a\"\xF7W[\x15a!\xDCW\x88a\"\x8DV[Pa\x01\0\x82\x10a\"\xECV[`@Qc0\x82\xDF\xDB`\xE1\x1B\x81R`\x04\x81\x01\x83\x90R`$\x81\x01\x85\x90R`D\x90\xFD[\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x05\rW`\x01g\r\xE0\xB6\xB3\xA7d\0\0`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[\x90g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x81\x02\x92\x81\x84\x04\x14\x90\x15\x17\x81\x15\x15\x16\x15a\x05\rW`\x01\x90`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[`\x01\x81\x15\x15\x16\x15a\x05\rWn\xC0\x97\xCE{\xC9\x07\x15\xB3K\x9F\x0F\xFF\xFF\xFF\xFF\x04`\x01\x01\x90V[a\x03\xE9\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x05\rW`\x01a\x03\xE8`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[\x15a#\xD1WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x90\xFD[h\x02H\xCE6\xA7\x0C\xB2k>\x19\x81\x13\x15a%\x86Wh\x07U\xBFy\x8BJ\x1B\xF1\xE5\x81\x12\x15a%RWe\x03x-\xAC\xE9\xD9\x90`N\x1B\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x91``\x90`\x01`_\x1B\x84\x82\x84\x1B\x05\x01\x82\x1D\x93\x84\x02\x90\x03\x80l\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x81\x01\x02\x82\x1D\x90n\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dPn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x82n\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x81m\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x81m\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x81m\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x81l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x02\x8D\x1D\x01\x02\x8B\x1D\x01\x02\x89\x1D\x01\x02\x87\x1D\x01\x02\x85\x1D\x01\x93m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x93m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEA\x81\x01\x90\x84m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x91\x01\x01\x02\x90\x1D\x01\x02\x01\x05\x02\x90`\xC3\x03\x1C\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x90\xFD[P`\0\x90V[a%\xCFa%\xBB\x91a\x12\xD3\x93`@Q\x93\x84\x92c-\x83\x9C\xB3`\xE2\x1B` \x85\x01R`@`$\x85\x01R`d\x84\x01\x90a\x04\x01V[\x90`D\x83\x01R\x03`\x1F\x19\x81\x01\x83R\x82a\x0B7V[a'`V[a%\xCFa%\xBB\x91a\x12\xD3\x93`@Q\x93\x84\x92c\x1ES\x13G`\xE1\x1B` \x85\x01R`@`$\x85\x01R`d\x84\x01\x90a\x04\x01V[a\x12\xD3\x93``\x92\x96\x95\x93`\xE0\x83\x01\x97\x83R` \x83\x01R`@\x82\x01R\x01\x90``\x90\x80Q\x83R` \x81\x01Q` \x84\x01R`@\x81\x01Q`@\x84\x01R\x81`\x01\x80`\xA0\x1B\x03\x91\x01Q\x16\x91\x01RV[`\x01\x81\x15\x15\x16\x15a\x05\rWn\xC0\x97\xCE{\xC9\x07\x15\xB3K\x9F\x10\0\0\0\0\x04\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17\x82\x15\x15\x16\x15a\x05\rW\x04\x90V[a\x03\xE8\x90\x80\x82\x02\x91\x82\x04\x14`\x01\x16\x15a\x05\rWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x04\x90V[\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x05\rWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x04\x90V[a\x03\xE7\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x05\rWa\x03\xE8\x90\x04\x90V[a&\xF9\x81\x15\x15a#\xCAV[\x80o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x07\x1B\x81\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x81\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x81\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x81\x81\x1C`\xFF\x10`\x03\x1B\x17\x81\x81\x1C`\x0F\x10`\x02\x1B\x17\x81\x81\x1C`\x03\x10`\x01\x1B\x17\x90\x81\x1C`\x01\x10\x17\x90V[`\0\x80\x91` \x81Q\x91\x01jconsole.logZ\xFAPV[a'\x91a\x047\x91` \x80\x82Q\x83\x01\x01\x91\x01a \xE2V[\x93\x92\x90\x92a\x1B\x82V[\x90a'\xB1a\x047\x92` \x80\x82Q\x83\x01\x01\x91\x01a \xE2V[\x94\x93\x92\x90\x92a\x1D\xC5V[\x80Q\x81\x01` \x01\x90` \x01\x90a'\xD0\x91a \xE2V[\x92\x91\x90\x83Q` \x85\x01Q`@\x86\x01Qa'\xE8\x90a\r\xBFV[\x91a'\xF3\x86\x86a&kV[a'\xFD\x82\x82a\x16\xC9V[\x92a(\x07\x91a\x16\xC9V[\x87Q\x86\x88\x85\x81a(\x17\x85\x8Ba&\xAEV[\x90a(!\x91a&\xAEV[\x90a(+\x91a&\xAEV[\x92a(5\x90a&\x8DV[a(>\x90a\r\xD5V[\x90a(H\x91a\r[V[\x90a(R\x91a&\xAEV[a([\x86a\r\xBFV[a(d\x91a&\xAEV[\x92a(n\x89a\rKV[\x90a(x\x90a\x14\xA7V[a(\x81\x91a\x16\xC9V[\x91a(\x8B\x90a&\x8DV[a(\x94\x86a\r\xBFV[a(\x9D\x91a&\xAEV[a(\xA7\x90\x87a\r[V[\x92a(\xB1\x91a\r\xF5V[\x91a(\xBB\x91a&\xAEV[\x87Qa(\xC6\x90a\r\xBFV[a(\xCF\x90a&LV[a(\xD8\x91a\x16\xC9V[a(\xE1\x91a&\xAEV[\x95Qa(\xEC\x90a\r\xBFV[\x92a(\xF6\x86a\rKV[\x95a)\0\x91a&\xAEV[\x90a)\n\x91a&\xAEV[\x92a\x16:\x90a&\x8DV[\x90a)+a\x047\x92` \x80\x82Q\x83\x01\x01\x91\x01a \xE2V[\x94\x93\x92\x90\x92a\x14\xEEV[\x80Q\x81\x01\x91`\xE0\x82\x84\x03\x12a\x03\xD4Wa\x047\x92a)c` \x84\x01Q\x93`\x80` `@\x83\x01Q\x94\x01\x91\x01a\x0F\xF0V[\x92a\x12BV\xFE\xA2dipfsX\"\x12 \xB7\xF2\x05\x0B4\xA8*d\xE7\xC2\xF0<`\xD5\x16dK\x8E\xC1'\xED\xFD\xC8\xC8\xE0g\xD5\xEC6\xC2\x1D\xABdsolcC\0\x08\x16\x003";
    /// The bytecode of the contract.
    pub static G3MSOLVER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\x0CcW`\x005`\xE0\x1C\x80c\x0FAf\xB8\x14a\x01\\W\x80c%\th\xD9\x14a\x01WW\x80c0m\xB4k\x14a\x01RW\x80c3\"f\xF3\x14a\x01MW\x80c9(\xFF\x97\x14a\x01HW\x80c;M\x100\x14a\x01CW\x80cO\xD6|X\x14a\x01>W\x80cZ\x93\xB8\xCE\x14a\x019W\x80cb7V\x9F\x14a\x014W\x80c\x7F\x17@\x9C\x14a\x01/W\x80c\x81\xB5\xFA\xC2\x14a\x01*W\x80c\x90.\xCA\xA2\x14a\x01%W\x80c\xA8\xC6.v\x14a\x01 W\x80c\xB0\x9D\x04\xE5\x14a\x01\x1BW\x80c\xCB\x1FU2\x14a\x01\x16W\x80c\xCE\x15;\xF4\x14a\x01\x11W\x80c\xDE\xF1_\x92\x14a\x01\x0CW\x80c\xEC)\xD8\xE6\x14a\x01\x07W\x80c\xEE>\x8C\xFB\x14a\x01\x02W\x80c\xF2\xDEz{\x14a\0\xFDWc\xF3\r7\xF2\x03a\x0CcWa\x0C0V[a\x0C\x14V[a\x0B\xE0V[a\x0B\xCAV[a\x0BYV[a\n;V[a\t\xF6V[a\t\xB2V[a\t\x89V[a\t`V[a\t\x0CV[a\x08\xACV[a\x08KV[a\x08&V[a\x07\xFDV[a\x07\xCBV[a\x057V[a\x04\xDAV[a\x04\xA3V[a\x04:V[a\x02\xAAV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01Ra7\xB7`\xF1\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x90\xFD[4a\x03\xD9W`@6`\x03\x19\x01\x12a\x03\xD4Wg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`$5\x81\x81\x11a\x03\xCFW6`#\x82\x01\x12\x15a\x03\xCAW\x80`\x04\x015\x91\x82\x11a\x03qW6`$\x83\x83\x01\x01\x11a\x03\x18Wa\x03\x14\x91`$a\x03\x04\x92\x01`\x045a\x0C\xC6V[`@Q\x90\x81R\x90\x81\x90` \x82\x01\x90V[\x03\x90\xF3[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rjrray stride`\xA8\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x0ENL/$\r\x8C\xAD\xCC\xEE\x8D`\xAB\x1B`d\x82\x01R`\x84\x90\xFD[a\x02QV[a\x02\x01V[a\x01\xB1V[a\x01aV[`\0[\x83\x81\x10a\x03\xF1WPP`\0\x91\x01RV[\x81\x81\x01Q\x83\x82\x01R` \x01a\x03\xE1V[\x90` \x91a\x04\x1A\x81Q\x80\x92\x81\x85R\x85\x80\x86\x01\x91\x01a\x03\xDEV[`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[\x90` a\x047\x92\x81\x81R\x01\x90a\x04\x01V[\x90V[4a\x03\xD9W`@6`\x03\x19\x01\x12a\x03\xD4Wa\x03\x14`@Q`\x02` \x82\x01R`\x045`@\x82\x01R`$5``\x82\x01R``\x81Ra\x04u\x81a\n\xDEV[`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x04\x01V[``\x90`\x03\x19\x01\x12a\x03\xD4W`\x045\x90`$5\x90`D5\x90V[4a\x03\xD9W` a\x04\xD2a\x04\xB66a\x04\x89V[\x90a\x04\xC9a\x04\xC3\x84a\x10IV[\x93a\x11#V[\x92\x91\x90\x91a\x12\xD5V[`@Q\x90\x81R\xF3[4a\x03\xD9W` a\x04\xD2a\x04\xED6a\x04\x89V[\x90a\x04\xFAa\x04\xC3\x84a\x10IV[\x92\x91\x90\x91a\x14\xEEV[\x80\x15\x15\x03a\x05\rWV[`\0\x80\xFD[\x90\x92`\x80\x92a\x047\x95\x94\x15\x15\x83R` \x83\x01R`@\x82\x01R\x81``\x82\x01R\x01\x90a\x04\x01V[4a\x03\xD9W``6`\x03\x19\x01\x12a\x03\xD4W`\x045`$5a\x05W\x81a\x05\x03V[`D5\x91a\x05ca\x0C\xF2V[\x91a\x05la\x0C\xF2V[\x92a\x05v\x83a\x11#V[\x92\x90\x91\x93` \x94\x85\x83\x01\x93`@\x95\x86\x85\x01R\x84R\x82Ra\x05\x95\x86a\x10IV[\x97\x85\x89\x86a\x05\xAF\x86Q\x88Qa\x05\xA9\x8Da\x10IV[\x91a\x16\x87V[\x94\x15a\x07EWa\x06\r\x93a\x05\xFFa\x05\xFAa\x06A\x99\x98\x95a\x05\xF4\x86a\x05\xDDa\x06\x06\x97a\x06\x1A\x9C\x99\x01Q\x87a#\"V[\x92a\x05\xEB\x8DQ\x8BQ\x90a#NV[\x91\x01Q\x90a\x16\xC9V[\x90a#\"V[a\r8V[\x93Qa\r[V[\x8ARa\r[V[\x80\x85\x89\x01R\x87Q\x87a\x12\x1CV[\x90a\x068a\x06-\x86\x89\x01\x93\x80\x85Ra\r8V[\x80\x84R\x82Q\x11a\x0E\x02V[Q\x90Q\x90a\r\xF5V[\x94[\x84Q\x83\x86\x01\x80Q\x84\x88\x01Q\x85Q\x96\x87\x01\x93\x84R` \x84\x01\x91\x90\x91R`@\x83\x01R\x90a\x06{\x90\x85\x90``\x01\x03`\x1F\x19\x81\x01\x86R\x85a\x0B7V[`\0Ta\x06\x9E\x90a\x06\x92\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x80;\x15a\x07@W\x84`\xC0\x91a\x06\xC9\x97\x86Q\x80\x99\x81\x94\x82\x93c\r\x17\xA7\xC7`\xE3\x1B\x84R0`\x04\x85\x01a\x0E\xE3V[\x03\x91Z\xFA\x94\x85\x15a\x07;W`\0\x95a\x06\xFBW[P\x90a\x06\xF0\x91a\x03\x14\x95\x96Q\x90Q\x90a\x18\x89V[\x90Q\x94\x85\x94\x85a\x05\x12V[a\x03\x14\x95P\x90a\x07&a\x06\xF0\x93\x92`\xC0=`\xC0\x11a\x074W[a\x07\x1E\x81\x83a\x0B7V[\x81\x01\x90a\x0E\xACV[PPPPP\x95P\x90\x91a\x06\xDCV[P=a\x07\x14V[a\x0F\x07V[a\x0EYV[a\x07\xA9\x92Pa\x07\xC5\x96a\x07\x90a\x07\xB1\x95a\x07\x89a\x05\xFA\x8Aa\x05\xF4\x88a\x07\x81a\x07wa\x07\x9C\x9Aa\x07\xBC\x9F\x9C\x01Q\x88a#\"V[\x93Q\x89Q\x90a#NV[\x90Q\x90a\x16\xC9V[\x92Qa\r[V[\x92\x8A\x8D\x01\x93\x84Ra\r[V[\x90\x81\x88\x8C\x01RQ\x89a\x0F\x13V[\x80\x89Ra\r8V[\x80\x88R\x82Q\x11a\rhV[Q\x85Q\x90a\r\xF5V[\x94a\x06CV[4a\x03\xD9W` 6`\x03\x19\x01\x12a\x03\xD4W` a\x04\xD2`\x045a\x07\xF6a\x07\xF0\x82a\x10IV[\x91a\x11#V[P\x90a\x18\x89V[4a\x03\xD9W` a\x04\xD2a\x08\x106a\x04\x89V[\x90a\x08\x1Da\x04\xC3\x84a\x10IV[\x92\x91\x90\x91a\x18\xAEV[4a\x03\xD9W` a\x04\xD2a\x08Ea\x08<6a\x04\x89V[\x91\x92\x90\x92a\x10IV[\x91a\x1A\x1CV[4a\x03\xD9W`@6`\x03\x19\x01\x12a\x03\xD4Wa\x08\x87`\x045a\x03\x14a\x08\x8Ea\x08\x7Fa\x08t\x84a\x11#V[\x91\x90P`$5a\x1AIV[\x94\x90\x93a\x10IV[\x84\x84a \xC1V[\x92`@Q\x93\x84\x93\x84`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[4a\x03\xD9W`@6`\x03\x19\x01\x12a\x03\xD4W`\x045a\x08\xE7a\x03\x14a\x08\xEEa\x08\xDFa\x08\xD5\x85a\x11#V[\x91P`$5a\x1AvV[\x93\x90\x94a\x10IV[\x83\x85a\x1A\x1CV[\x91`@Q\x93\x84\x93\x84`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[4a\x03\xD9W` 6`\x03\x19\x01\x12a\x03\xD4W`\x80a\t*`\x045a\x10IV[a\t^`@Q\x80\x92``\x90\x80Q\x83R` \x81\x01Q` \x84\x01R`@\x81\x01Q`@\x84\x01R\x81`\x01\x80`\xA0\x1B\x03\x91\x01Q\x16\x91\x01RV[\xF3[4a\x03\xD9W` a\x04\xD2a\ts6a\x04\x89V[\x90a\t\x80a\x04\xC3\x84a\x10IV[\x92\x91\x90\x91a\x1D\xC5V[4a\x03\xD9W`\x006`\x03\x19\x01\x12a\x03\xD4W`\0T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x03\xD9W` 6`\x03\x19\x01\x12a\x03\xD4Wa\x03\x14`@Q`\x01` \x82\x01R`\x045`@\x82\x01R`@\x81Ra\x04u\x81a\n\xFFV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\x05\rWV[4a\x03\xD9W` 6`\x03\x19\x01\x12a\x03\xD4Wa\x03\x14`\x045a\n\x16\x81a\t\xE5V[`@\x80Q`\x03` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x82\x82\x01R\x81Ra\x04u\x81a\n\xFFV[4a\x03\xD9W` 6`\x03\x19\x01\x12a\x03\xD4Wa\x03\x14a\nZ`\x045a\x11#V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R\x90\x81\x90``\x82\x01\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FABI decoding: struct data too sh`D\x82\x01Rb\x1B\xDC\x9D`\xEA\x1B`d\x82\x01R`\x84\x90\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\x80\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\n\xFAW`@RV[a\n\xC8V[``\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\n\xFAW`@RV[`@\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\n\xFAW`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\n\xFAW`@RV[4a\x03\xD9W`\xC06`\x03\x19\x01\x12a\x03\xD4W`\x806`C\x19\x01\x12a\x0B\xC5Wa\x03\x14a\x0B\xB9`@Qa\x0B\x88\x81a\n\xDEV[`D5\x81R`d5` \x82\x01R`\x845`@\x82\x01R`\xA45a\x0B\xA9\x81a\t\xE5V[``\x82\x01R`$5`\x045a\x1F\x86V[`@Q\x91\x82\x91\x82a\x04&V[a\nwV[4a\x03\xD9W` a\x04\xD2a\x05\xA9a\x08<6a\x04\x89V[4a\x03\xD9W`@6`\x03\x19\x01\x12a\x03\xD4Wa\x08\x87`\x045a\x03\x14a\x08\x8Ea\x08\x7Fa\x0C\t\x84a\x11#V[\x91\x90P`$5a\x1AvV[4a\x03\xD9W` a\x04\xD2a\x0C*a\x08<6a\x04\x89V[\x91a \xC1V[4a\x03\xD9W`@6`\x03\x19\x01\x12a\x03\xD4W`\x045a\x08\xE7a\x03\x14a\x08\xEEa\x08\xDFa\x0CY\x85a\x11#V[\x91P`$5a\x1AIV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01Rtnor receive functions`X\x1B`d\x82\x01R`\x84\x90\xFD[\x91\x81``\x91\x81\x01\x03\x12a\x03\xD4Wa\x0C\xDFa\x047\x92a\x10IV[\x90`@\x81\x015\x90` \x81\x015\x905a\x12BV[`@Q\x90``\x82\x01\x82\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\n\xFAW`@R`\0`@\x83\x82\x81R\x82` \x82\x01R\x01RV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x90`\x01\x82\x01\x80\x92\x11a\rFWV[a\r\"V[\x90a\x03\xE8\x91\x82\x01\x80\x92\x11a\rFWV[\x91\x90\x82\x01\x80\x92\x11a\rFWV[\x15a\roWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: x reserve increase`D\x82\x01Rad!`\xF0\x1B`d\x82\x01R`\x84\x90\xFD[\x90g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x03\x91\x82\x11a\rFWV[\x90a\x03\xE8\x91\x82\x03\x91\x82\x11a\rFWV[a\x03\xE7\x19\x81\x01\x91\x90\x82\x11a\rFWV[\x91\x90\x82\x03\x91\x82\x11a\rFWV[\x15a\x0E\tWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: y reserve increase`D\x82\x01Rad!`\xF0\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01Rd code`\xD8\x1B`d\x82\x01R`\x84\x90\xFD[\x91\x90\x82`\xC0\x91\x03\x12a\x03\xD4W\x81Qa\x0E\xC3\x81a\x05\x03V[\x91` \x81\x01Q\x91`@\x82\x01Q\x91``\x81\x01Q\x91`\xA0`\x80\x83\x01Q\x92\x01Q\x90V[a\x047\x93\x92``\x92`\x01\x80`\xA0\x1B\x03\x16\x82R` \x82\x01R\x81`@\x82\x01R\x01\x90a\x04\x01V[`@Q=`\0\x82>=\x90\xFD[\x91a\x08Ea\x047\x93a\x10IV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\n\xFAW`\x1F\x01`\x1F\x19\x16` \x01\x90V[` \x81\x83\x03\x12a\x03\xD4W\x80Q\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x03\xCFW\x01\x81`\x1F\x82\x01\x12\x15a\x03\xCAW\x80Qa\x0Fo\x81a\x0F V[\x92a\x0F}`@Q\x94\x85a\x0B7V[\x81\x84R` \x82\x84\x01\x01\x11a\x0F\x9BWa\x047\x91` \x80\x85\x01\x91\x01a\x03\xDEV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R`\x84\x90\xFD[\x91\x90\x82`\x80\x91\x03\x12a\x0B\xC5W`@Qa\x10\x08\x81a\n\xDEV[``\x80\x82\x94\x80Q\x84R` \x81\x01Q` \x85\x01R`@\x81\x01Q`@\x85\x01R\x01Q\x91a\x101\x83a\t\xE5V[\x01RV[\x90`\x80\x82\x82\x03\x12a\x03\xD4Wa\x047\x91a\x0F\xF0V[``\x90`@Qa\x10X\x81a\n\xDEV[`\0\x80\x82R` \x82\x01\x81\x90R`@\x82\x01\x81\x90R\x92\x01\x82\x90R\x81Ta\x10\x86\x90a\x06\x92\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x80;\x15a\x07@W`@Qc\xDC\x17\x83U`\xE0\x1B\x81R`\x04\x81\x01\x92\x90\x92R\x82\x90\x82\x90`$\x90\x82\x90Z\xFA\x90\x81\x15a\x07;W\x82a\x047\x93\x92a\x10\xD0W[PP` \x80\x82Q\x83\x01\x01\x91\x01a\x105V[a\x10\xEC\x92P=\x80\x91\x83>a\x10\xE4\x81\x83a\x0B7V[\x81\x01\x90a\x0F<V[8\x80a\x10\xBFV[\x90\x81` \x91\x03\x12a\x03\xD4WQa\x047\x81a\t\xE5V[\x90\x81``\x91\x03\x12a\x03\xD4W\x80Q\x91`@` \x83\x01Q\x92\x01Q\x90V[`\0T\x90\x91\x90a\x11=\x90a\x06\x92\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x80;\x15a\x07@W` `\x04\x91`@Q\x92\x83\x80\x92c+\xEE\x84\xF1`\xE2\x1B\x82RZ\xFA\x90\x81\x15a\x07;W`\0\x91a\x11\xEDW[P`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x07@W`@Qc3\x85N\xFD`\xE2\x1B\x81R`\x04\x81\x01\x93\x90\x93R``\x90\x83\x90`$\x90\x82\x90Z\xFA\x90\x81\x15a\x07;W`\0\x80\x93`\0\x93a\x11\xB6W[P\x92\x91\x90V[\x91\x93PPa\x11\xDC\x91P``=``\x11a\x11\xE6W[a\x11\xD4\x81\x83a\x0B7V[\x81\x01\x90a\x11\x08V[\x92\x90\x92\x918a\x11\xB0V[P=a\x11\xCAV[a\x12\x0F\x91P` =` \x11a\x12\x15W[a\x12\x07\x81\x83a\x0B7V[\x81\x01\x90a\x10\xF3V[8a\x11kV[P=a\x11\xFDV[\x91a\x0C*a\x047\x93a\x10IV[\x81\x81\x03\x92\x91`\0\x13\x80\x15\x82\x85\x13\x16\x91\x84\x12\x16\x17a\rFWV[\x92` a\x05\xEB\x84a\x12ea\x12]a\x05\xF4\x96\x97a\x12k\x99a&kV[\x85Q\x90a\x16\xC9V[\x95a&kV[g\r\xE0\xB6\xB3\xA7c\xFF\xFF\x19\x81\x01\x90\x81\x13`\x01\x16a\rFW\x90V[\x90\x81R` \x80\x82\x01\x92\x90\x92R`@\x80\x82\x01\x93\x90\x93R``\x80\x82\x01\x94\x90\x94R\x84Q`\x80\x82\x01R\x90\x84\x01Q`\xA0\x82\x01R\x90\x83\x01Q`\xC0\x82\x01R\x91\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\xE0\x82\x01Ra\x01\0\x01\x90V[V[\x90\x92\x91\x85Q` \x87\x01Q`@\x88\x01Qa\x12\xED\x90a\r\xBFV[\x91a\x12\xF8\x87\x85a&kV[a\x13\x02\x82\x82a\x16\xC9V[\x92a\x13\x0C\x91a\x16\xC9V[\x89Q\x85\x89\x85\x81a\x13\x1C\x85\x8Da&\xAEV[\x90a\x13&\x91a&\xAEV[\x90a\x130\x91a&\xAEV[\x92a\x13:\x90a&\x8DV[a\x13C\x90a\r\xD5V[\x90a\x13M\x91a\r[V[\x90a\x13W\x91a&\xAEV[a\x13`\x86a\r\xBFV[a\x13i\x91a&\xAEV[\x92a\x13s\x8Aa\rKV[\x90a\x13}\x90a\x14\xA7V[a\x13\x86\x91a\x16\xC9V[\x91a\x13\x90\x90a&\x8DV[a\x13\x99\x86a\r\xBFV[a\x13\xA2\x91a&\xAEV[a\x13\xAC\x90\x89a\r[V[\x92a\x13\xB6\x91a\r\xF5V[\x91a\x13\xC0\x91a&\xAEV[\x89Qa\x13\xCB\x90a\r\xBFV[a\x13\xD4\x90a&LV[a\x13\xDD\x91a\x16\xC9V[a\x13\xE6\x91a&\xAEV[\x91\x88Qa\x13\xF2\x90a\r\xBFV[a\x13\xFB\x88a\rKV[\x92a\x14\x06\x89\x89a&\xAEV[\x90a\x14\x10\x91a&\xAEV[\x91a\x14\x1A\x86a&\x8DV[\x90a\x14$\x90a\r\xBFV[a\x14-\x91a&\xAEV[\x92a\x147\x91a&\xAEV[\x91a\x14A\x91a\r[V[a\x14J\x91a&\xAEV[\x90a\x14T\x84a\x14\xA7V[\x91a\x14^\x91a&kV[a\x14g\x91a\x14\xD2V[`\0\x13a\x14\x9CWa\x047\x95a\x14\x97\x93a\x14\x89\x92`@Q\x96\x87\x95` \x87\x01a\x12\x84V[\x03`\x1F\x19\x81\x01\x83R\x82a\x0B7V[a!\x0FV[PPPPPP`\0\x90V[`\x01`\xFF\x1B\x81\x14a\rFW`\0\x03\x90V[\x90\x81g\r\xE0\xB6\xB3\xA7c\xFF\xFF\x19\x01\x91\x82\x12\x15`\x01\x16a\rFWV[\x91\x90\x91`\0\x83\x82\x01\x93\x84\x12\x91\x12\x90\x80\x15\x82\x16\x91\x15\x16\x17a\rFWV[\x94\x93\x92\x90\x92\x84Q\x90` \x86\x01Q`@\x87\x01Qa\x15\t\x90a\r\xBFV[\x92a\x15\x14\x87\x87a&kV[a\x15\x1E\x82\x82a\x16\xC9V[\x92a\x15(\x91a\x16\xC9V[\x88Q\x87\x89\x85\x81a\x158\x85\x8Ca&\xAEV[\x90a\x15B\x91a&\xAEV[\x90a\x15L\x91a&\xAEV[\x92a\x15W\x90\x88a&\xAEV[a\x15a\x90\x88a\r\xF5V[\x90a\x15k\x91a\r[V[\x90a\x15u\x91a&\xAEV[a\x15~\x87a\r\xBFV[a\x15\x87\x91a&\xAEV[\x92a\x15\x92\x8A\x87a\r[V[\x90a\x15\x9C\x90a\x14\xA7V[a\x15\xA5\x91a\x16\xC9V[\x91a\x15\xB0\x90\x86a&\xAEV[a\x15\xB9\x87a\r\xBFV[a\x15\xC2\x91a&\xAEV[a\x15\xCC\x90\x88a\r[V[\x92a\x15\xD6\x91a\r\xF5V[\x91a\x15\xE0\x91a&\xAEV[\x88Qa\x15\xEB\x90a\r\xBFV[a\x15\xF4\x90a&LV[a\x15\xFD\x91a\x16\xC9V[a\x16\x06\x91a&\xAEV[\x96Qa\x16\x11\x90a\r\xBFV[\x93a\x16\x1C\x87\x84a\r[V[\x96a\x16&\x91a&\xAEV[\x90a\x160\x91a&\xAEV[\x93a\x16:\x91a&\xAEV[\x90a\x16D\x90a\r\xBFV[a\x16M\x91a&\xAEV[\x92a\x16W\x91a&\xAEV[\x91a\x16a\x91a\r[V[a\x16j\x91a&\xAEV[\x91a\x16t\x90a\x14\xA7V[\x91a\x16~\x91a&kV[a\x047\x91a\x14\xD2V[a\x047\x92\x91` a\x16\x9Da\x05\xF4\x93\x85Q\x90a\x16\xC9V[\x93\x01Q\x90a\x16\xC9V[\x81\x81\x02\x92\x91`\0\x82\x12`\x01`\xFF\x1B\x82\x14\x16a\rFW\x81\x84\x05\x14\x90\x15\x17\x15a\rFWV[a\x18va\x047\x92}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84a\x18\x84\x93a\x16\xFF`\0\x82\x13a#\xCAV[q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06a\x17\x1B\x82a&\xEEV[``\x92\x83\x82`\x9F\x03\x01\x1B`\x9F\x1C\x90`_\x19\x01}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x02\x92l\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x82m\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x81m\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x81m\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x81m\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x81m\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x81lFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x02\x8C\x1D\x01\x02\x8A\x1D\x01\x02\x88\x1D\x01\x02\x86\x1D\x01\x02\x84\x1D\x01\x02\x82\x1D\x01\x91x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x91l\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x90\x82m\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x81l\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x81m\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x81m\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x81l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x81\x01\x02\x89\x1D\x01\x02\x87\x1D\x01\x02\x85\x1D\x01\x02\x83\x1D\x01\x02\x90\x1D\x01\x02\x01\x05\x02\x01\x01`\xAE\x1Da\x16\xA6V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[a$\x02V[a\x18\xA8\x90a\x18\xA0a\x047\x94\x93` \x85\x01Q\x90a&kV[\x92Q\x90a&kV[\x90a&kV[\x91\x92\x94\x90\x93\x94`\0\x93\x84a\x18\xC5\x84\x84\x84\x8A\x89a\x1B\x82V[\x12a\x1A\x12W\x91a\x18\xE3\x91a\x18\xF1\x94\x93`@Q\x97\x88\x95` \x87\x01a\x12\x84V[\x03`\x1F\x19\x81\x01\x84R\x83a\x0B7V[\x82\x91a\x03\xE8\x90\x82\x94\x80\x83\x11a\x19\xF0Wa\x19\t\x82a'{V[\x90a\x19\x14\x81\x84a'\x9AV[\x85a\x19\x1F\x82\x85a\x16\xA6V[\x13a\x19\xCFWPa\x19.\x90a\r\xE5V[\x93\x80`\x01\x95\x86`\x01\x90[a\x19IW[PPPPPPPPP\x90V[\x15a\x19\xAAW[P\x85\x96\x97P\x80\x91a\x19ia\x19c\x8A\x88a\r[V[`\x01\x1C\x90V[\x98a\x19t\x8A\x87a'\x9AV[\x90\x83a\x19\x80\x87\x84a\x16\xA6V[\x13a\x19\x9EWPP\x88\x92[\x87a\x19\x95\x88\x86a\r\xF5V[\x92\x01\x93\x98a\x198V[\x8A\x97P\x90\x94P\x92a\x19\x8AV[\x86\x10\x80a\x19\xC4W[\x15a\x19\xBDW\x87a\x19OV[\x80\x80a\x19=V[Pa\x01\0\x82\x10a\x19\xB2V[`@Qc\x06\xF1\xBE]`\xE2\x1B\x81R`\x04\x81\x01\x84\x90R`$\x81\x01\x91\x90\x91R`D\x90\xFD[`@Qc0\x82\xDF\xDB`\xE1\x1B\x81Ra\x03\xE8`\x04\x82\x01R`$\x81\x01\x91\x90\x91R`D\x90\xFD[PPPP\x91PP\x90V[a\x1A<a\x047\x93\x92a\x1A6a\x1AC\x93` \x86\x01Q\x90a\x16\xC9V[\x90a#NV[\x91Qa#~V[\x90a\x16\xC9V[\x92\x91\x90a\x1A_a\x1AY\x82\x84a#NV[\x85a#\"V[\x93\x81\x03\x90\x81\x11a\rFW\x92\x81\x03\x90\x81\x11a\rFW\x90V[\x92\x91\x90a\x1A\x86a\x1AY\x82\x84a#NV[\x93\x81\x01\x80\x91\x11a\rFW\x92\x81\x01\x80\x91\x11a\rFW\x90V[`@Q\x90a\x1A\xAA\x82a\x0B\x1BV[`\x05\x82Rd\x19\x9A\\\x9C\xDD`\xDA\x1B` \x83\x01RV[`@Q\x90a\x1A\xCB\x82a\x0B\x1BV[`\x06\x82Re\x1C\xD9X\xDB\xDB\x99`\xD2\x1B` \x83\x01RV[`@Q\x90a\x1A\xED\x82a\x0B\x1BV[`\x05\x82Rd\x1D\x1A\x1A\\\x99`\xDA\x1B` \x83\x01RV[`@Q\x90a\x1B\x0E\x82a\x0B\x1BV[`\x06\x82Re\x0C\xCD\xEE\xAEN\x8D`\xD3\x1B` \x83\x01RV[`@Q\x90a\x1B0\x82a\x0B\x1BV[`\x05\x82Rd\x0C\xCD,\xCE\x8D`\xDB\x1B` \x83\x01RV[`@Q\x90a\x1BQ\x82a\x0B\x1BV[`\x03\x82Rbnum`\xE8\x1B` \x83\x01RV[`@Q\x90a\x1Bp\x82a\x0B\x1BV[`\x03\x82Rb22\xB7`\xE9\x1B` \x83\x01RV[\x90\x92\x82\x85Q\x80\x95`@\x88\x01Qa\x1B\x97\x90a\r\xBFV[\x95a\x1B\xA1\x91a&kV[\x90a\x1B\xAB\x91a\x16\xC9V[\x94a\x1B\xB5\x82a\rKV[\x90a\x1B\xBF\x90a\x14\xB8V[a\x1B\xC8\x91a\x16\xC9V[\x90a\x1B\xD2\x86a&\x8DV[\x90a\x1B\xDC\x86a\r\xBFV[a\x1B\xE6\x90\x83a&\xAEV[a\x1B\xF0\x90\x85a\r[V[\x88Qa\x1B\xFB\x83a\rKV[a\x1C\x04\x91a&\xAEV[\x93\x84a\x1C\x0Ea\x1A\x9DV[\x90a\x1C\x18\x91a%\x8CV[\x81\x80a\x1C\"a\x1A\xBEV[\x90a\x1C,\x91a%\x8CV[a\x1C5\x91a&\xAEV[\x89Qa\x1C@\x90a&LV[a\x1CI\x91a\x16\xC9V[\x91\x82a\x1CSa\x1A\xE0V[\x90a\x1C]\x91a%\x8CV[\x89Qa\x1Ch\x90a\r\xBFV[a\x1Cr\x90\x87a&\xAEV[\x98\x89a\x1C|a\x1B\x01V[\x90a\x1C\x86\x91a%\x8CV[\x81\x8BQa\x1C\x92\x90a&\x8DV[\x90a\x1C\x9C\x91a\r[V[a\x1C\xA5\x91a&\xAEV[a\x1C\xAE\x89a\r\xBFV[a\x1C\xB7\x91a&\xAEV[\x80a\x1C\xC0a\x1B#V[\x90a\x1C\xCA\x91a%\x8CV[a\x1Dh\x99a\x1D[\x98a\x1DU\x97a\x1D8\x96a\x1D>\x96\x88\x95\x80\x86\x10\x15a\x1D\x9BWa\x1D\x18\x92a\x1D\ra\x1D\x08\x96\x95\x93a\x1D\ra\x1D\x08a\x1D\x13\x95a\x1D\x1E\x9Ba\x12)V[a\x14\xA7V[\x94a&\xAEV[a&\xAEV[\x90a\x12)V[\x9A[a\x1D1\x8Ca\x1D,a\x1BDV[a%\xD4V[Q\x91a\rKV[\x90a&\xAEV[\x95a\x1DP\x87a\x1DKa\x1A\x9DV[a%\x8CV[a\r\xBFV[\x90a\r[V[\x90a\x1D\x13\x82a\x1DKa\x1A\xBEV[a\x1Dt\x81a\x1DKa\x1BcV[`\0\x82\x13\x15a\x1D\x8AWa\x1D\x08\x90a\x047\x92a&kV[a\x1D\x96a\x047\x92a\x14\xA7V[a&kV[a\x1D\xB9a\x1D\xBF\x96\x93a\x1D\xB3a\x1D8\x94a\x1D\x18\x97a&\xAEV[\x96a&\xAEV[\x92a\r\xF5V[\x9Aa\x1D V[\x91\x80\x93\x91\x94\x86Q\x80\x96`@\x89\x01Qa\x1D\xDC\x90a\r\xBFV[\x96a\x1D\xE6\x91a&kV[\x90a\x1D\xF0\x91a\x16\xC9V[\x95a\x1D\xFB\x83\x83a\r[V[\x90a\x1E\x05\x90a\x14\xB8V[a\x1E\x0E\x91a\x16\xC9V[\x91a\x1E\x19\x87\x83a&\xAEV[\x91a\x1E#\x87a\r\xBFV[a\x1E-\x90\x84a&\xAEV[a\x1E7\x90\x86a\r[V[\x90\x89Qa\x1ED\x84\x83a\r[V[a\x1EM\x91a&\xAEV[\x94\x85a\x1EWa\x1A\x9DV[\x90a\x1Ea\x91a%\x8CV[\x82\x80a\x1Eka\x1A\xBEV[\x90a\x1Eu\x91a%\x8CV[a\x1E~\x91a&\xAEV[\x8AQa\x1E\x89\x90a&LV[a\x1E\x92\x91a\x16\xC9V[\x92\x83a\x1E\x9Ca\x1A\xE0V[\x90a\x1E\xA6\x91a%\x8CV[\x8AQa\x1E\xB1\x90a\r\xBFV[a\x1E\xBB\x90\x88a&\xAEV[\x99\x8Aa\x1E\xC5a\x1B\x01V[\x90a\x1E\xCF\x91a%\x8CV[\x81\x8CQa\x1E\xDC\x90\x85a&\xAEV[\x90a\x1E\xE6\x91a\r[V[a\x1E\xEF\x91a&\xAEV[a\x1E\xF8\x8Aa\r\xBFV[a\x1F\x01\x91a&\xAEV[\x90\x81a\x1F\x0Ba\x1B#V[\x90a\x1F\x15\x91a%\x8CV[a\x1Dh\x9Aa\x1D[\x99a\x1DU\x98a\x1D8\x97a\x1D>\x97\x89\x96\x80\x86\x10\x15a\x1FhWa\x1D\x18\x92a\x1D\ra\x1D\x08\x96\x95\x93a\x1D\ra\x1D\x08a\x1D\x13\x95a\x1FS\x9Ba\x12)V[\x9B[a\x1Fa\x8Da\x1D,a\x1BDV[Q\x92a\r[V[a\x1D\xB9a\x1F\x80\x96\x93a\x1D\xB3a\x1D8\x94a\x1D\x18\x97a&\xAEV[\x9Ba\x1FUV[\x92\x91\x90\x83a\x1D\x13a\x1F\xA1\x92a\x1D\x13` \x86\x01Q\x86Q\x90a&kV[\x90a\x1F\xAD\x81\x83\x86a\x16\x87V[\x93a\x1F\xBA\x82\x86\x85\x84a\x12BV[\x85\x90`\0\x80\x82\x12\x15a \x83W[\x80\x82\x12a eWPa \x0Ca Y\x92a\x047\x96\x97\x98\x86\x93[a\x1F\xF3`@Q\x98\x89\x92\x8C\x8A` \x86\x01a&\x03V[\x03\x96a \x07`\x1F\x19\x98\x89\x81\x01\x83R\x82a\x0B7V[a\".V[\x81Q`@\x80\x84\x01Q``\x94\x85\x01Q\x82Q` \x81\x01\x98\x90\x98R\x91\x87\x01\x99\x90\x99R\x92\x85\x01\x91\x90\x91R`\x80\x84\x01R`\xA0\x83\x01\x95\x90\x95R`\x01`\x01`\xA0\x1B\x03\x90\x94\x16`\xC0\x82\x01R\x92\x83\x90`\xE0\x82\x01\x90V[\x03\x90\x81\x01\x83R\x82a\x0B7V[\x96a p\x91Pa&\xCFV[\x95a }\x84\x88\x87\x86a\x12BV[\x90a\x1F\xC7V[\x96\x91\x96[\x80\x82\x13a \xA3WPa \x0Ca\x047\x95\x96\x97a Y\x93\x86\x93a\x1F\xDFV[\x96a \xAE\x91Pa#\xA0V[\x95a \xBB\x84\x88\x87\x86a\x12BV[\x90a \x87V[` a \xDAa\x047\x94\x93a\x1A6a\x1AC\x94\x86Q\x90a\x16\xC9V[\x92\x01Qa#~V[\x91\x90a\x01\0\x83\x82\x03\x12a\x03\xD4W\x82Q\x92` \x81\x01Q\x92a\x047`@\x83\x01Q\x93`\x80``\x85\x01Q\x94\x01a\x0F\xF0V[\x91\x90a\x03\xE8\x92`\0\x93`\0\x91\x83\x82\x11a\"\rWa!+\x81a'\xBBV[a!5\x85\x83a)\x14V[`\0a!A\x82\x84a\x16\xA6V[\x13a!\xEEWPa!R\x85\x96\x95a\r\xE5V[`\x01\x94`\0\x91\x86\x80[a!lW[PPPPPPPP\x90PV[\x15a!\xC9W[P\x85\x96\x97\x98P\x80\x91a!\x87a\x19c\x8B\x88a\r[V[\x99a!\x92\x8B\x87a)\x14V[\x90\x83a!\x9E\x87\x84a\x16\xA6V[\x13a!\xBDWPP\x89\x92[\x87a!\xB3\x88\x86a\r\xF5V[\x92\x01\x93\x99\x98a![V[\x8B\x97P\x90\x94P\x92a!\xA8V[\x86\x10\x80a!\xE3W[\x15a!\xDCW\x88a!rV[\x80\x80a!`V[Pa\x01\0\x82\x10a!\xD1V[`@Qc\x06\xF1\xBE]`\xE2\x1B\x81R`\x04\x81\x01\x92\x90\x92R`$\x82\x01R`D\x90\xFD[`@Qc0\x82\xDF\xDB`\xE1\x1B\x81Ra\x03\xE8`\x04\x82\x01R`$\x81\x01\x85\x90R`D\x90\xFD[`\0\x93\x92\x91\x84\x91\x83\x82\x11a#\x02Wa\"F\x82\x82a)5V[a\"P\x85\x83a)5V[`\0a\"\\\x82\x84a\x16\xA6V[\x13a!\xEEWPa\"n\x83\x86\x97\x96a\r\xF5V[`\x01\x94`\0\x91\x86\x80[a\"\x87WPPPPPPPP\x90PV[\x15a\"\xE4W[P\x85\x96\x97\x98P\x80\x91a\"\xA2a\x19c\x8B\x88a\r[V[\x99a\"\xAD\x8B\x87a)5V[\x90\x83a\"\xB9\x87\x84a\x16\xA6V[\x13a\"\xD8WPP\x89\x92[\x87a\"\xCE\x88\x86a\r\xF5V[\x92\x01\x93\x99\x98a\"wV[\x8B\x97P\x90\x94P\x92a\"\xC3V[\x86\x10\x80a\"\xF7W[\x15a!\xDCW\x88a\"\x8DV[Pa\x01\0\x82\x10a\"\xECV[`@Qc0\x82\xDF\xDB`\xE1\x1B\x81R`\x04\x81\x01\x83\x90R`$\x81\x01\x85\x90R`D\x90\xFD[\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x05\rW`\x01g\r\xE0\xB6\xB3\xA7d\0\0`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[\x90g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x81\x02\x92\x81\x84\x04\x14\x90\x15\x17\x81\x15\x15\x16\x15a\x05\rW`\x01\x90`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[`\x01\x81\x15\x15\x16\x15a\x05\rWn\xC0\x97\xCE{\xC9\x07\x15\xB3K\x9F\x0F\xFF\xFF\xFF\xFF\x04`\x01\x01\x90V[a\x03\xE9\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x05\rW`\x01a\x03\xE8`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[\x15a#\xD1WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x90\xFD[h\x02H\xCE6\xA7\x0C\xB2k>\x19\x81\x13\x15a%\x86Wh\x07U\xBFy\x8BJ\x1B\xF1\xE5\x81\x12\x15a%RWe\x03x-\xAC\xE9\xD9\x90`N\x1B\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x91``\x90`\x01`_\x1B\x84\x82\x84\x1B\x05\x01\x82\x1D\x93\x84\x02\x90\x03\x80l\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x81\x01\x02\x82\x1D\x90n\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dPn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x82n\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x81m\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x81m\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x81m\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x81l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x02\x8D\x1D\x01\x02\x8B\x1D\x01\x02\x89\x1D\x01\x02\x87\x1D\x01\x02\x85\x1D\x01\x93m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x93m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEA\x81\x01\x90\x84m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x91\x01\x01\x02\x90\x1D\x01\x02\x01\x05\x02\x90`\xC3\x03\x1C\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x90\xFD[P`\0\x90V[a%\xCFa%\xBB\x91a\x12\xD3\x93`@Q\x93\x84\x92c-\x83\x9C\xB3`\xE2\x1B` \x85\x01R`@`$\x85\x01R`d\x84\x01\x90a\x04\x01V[\x90`D\x83\x01R\x03`\x1F\x19\x81\x01\x83R\x82a\x0B7V[a'`V[a%\xCFa%\xBB\x91a\x12\xD3\x93`@Q\x93\x84\x92c\x1ES\x13G`\xE1\x1B` \x85\x01R`@`$\x85\x01R`d\x84\x01\x90a\x04\x01V[a\x12\xD3\x93``\x92\x96\x95\x93`\xE0\x83\x01\x97\x83R` \x83\x01R`@\x82\x01R\x01\x90``\x90\x80Q\x83R` \x81\x01Q` \x84\x01R`@\x81\x01Q`@\x84\x01R\x81`\x01\x80`\xA0\x1B\x03\x91\x01Q\x16\x91\x01RV[`\x01\x81\x15\x15\x16\x15a\x05\rWn\xC0\x97\xCE{\xC9\x07\x15\xB3K\x9F\x10\0\0\0\0\x04\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17\x82\x15\x15\x16\x15a\x05\rW\x04\x90V[a\x03\xE8\x90\x80\x82\x02\x91\x82\x04\x14`\x01\x16\x15a\x05\rWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x04\x90V[\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x05\rWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x04\x90V[a\x03\xE7\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x05\rWa\x03\xE8\x90\x04\x90V[a&\xF9\x81\x15\x15a#\xCAV[\x80o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x07\x1B\x81\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x81\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x81\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x81\x81\x1C`\xFF\x10`\x03\x1B\x17\x81\x81\x1C`\x0F\x10`\x02\x1B\x17\x81\x81\x1C`\x03\x10`\x01\x1B\x17\x90\x81\x1C`\x01\x10\x17\x90V[`\0\x80\x91` \x81Q\x91\x01jconsole.logZ\xFAPV[a'\x91a\x047\x91` \x80\x82Q\x83\x01\x01\x91\x01a \xE2V[\x93\x92\x90\x92a\x1B\x82V[\x90a'\xB1a\x047\x92` \x80\x82Q\x83\x01\x01\x91\x01a \xE2V[\x94\x93\x92\x90\x92a\x1D\xC5V[\x80Q\x81\x01` \x01\x90` \x01\x90a'\xD0\x91a \xE2V[\x92\x91\x90\x83Q` \x85\x01Q`@\x86\x01Qa'\xE8\x90a\r\xBFV[\x91a'\xF3\x86\x86a&kV[a'\xFD\x82\x82a\x16\xC9V[\x92a(\x07\x91a\x16\xC9V[\x87Q\x86\x88\x85\x81a(\x17\x85\x8Ba&\xAEV[\x90a(!\x91a&\xAEV[\x90a(+\x91a&\xAEV[\x92a(5\x90a&\x8DV[a(>\x90a\r\xD5V[\x90a(H\x91a\r[V[\x90a(R\x91a&\xAEV[a([\x86a\r\xBFV[a(d\x91a&\xAEV[\x92a(n\x89a\rKV[\x90a(x\x90a\x14\xA7V[a(\x81\x91a\x16\xC9V[\x91a(\x8B\x90a&\x8DV[a(\x94\x86a\r\xBFV[a(\x9D\x91a&\xAEV[a(\xA7\x90\x87a\r[V[\x92a(\xB1\x91a\r\xF5V[\x91a(\xBB\x91a&\xAEV[\x87Qa(\xC6\x90a\r\xBFV[a(\xCF\x90a&LV[a(\xD8\x91a\x16\xC9V[a(\xE1\x91a&\xAEV[\x95Qa(\xEC\x90a\r\xBFV[\x92a(\xF6\x86a\rKV[\x95a)\0\x91a&\xAEV[\x90a)\n\x91a&\xAEV[\x92a\x16:\x90a&\x8DV[\x90a)+a\x047\x92` \x80\x82Q\x83\x01\x01\x91\x01a \xE2V[\x94\x93\x92\x90\x92a\x14\xEEV[\x80Q\x81\x01\x91`\xE0\x82\x84\x03\x12a\x03\xD4Wa\x047\x92a)c` \x84\x01Q\x93`\x80` `@\x83\x01Q\x94\x01\x91\x01a\x0F\xF0V[\x92a\x12BV\xFE\xA2dipfsX\"\x12 \xB7\xF2\x05\x0B4\xA8*d\xE7\xC2\xF0<`\xD5\x16dK\x8E\xC1'\xED\xFD\xC8\xC8\xE0g\xD5\xEC6\xC2\x1D\xABdsolcC\0\x08\x16\x003";
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
