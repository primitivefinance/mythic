pub use dfmm::*;
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
pub mod dfmm {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("weth_"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("allocate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("allocate"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("deallocate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("deallocate"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getPool"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getPool"),
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
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct IDFMM.Pool"),
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
                    ::std::borrow::ToOwned::to_owned("init"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("init"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("params"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct IDFMM.InitParams"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("liquidityOf"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("liquidityOf"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
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
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("lpTokenImplementation"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "lpTokenImplementation",
                            ),
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
                (
                    ::std::borrow::ToOwned::to_owned("nonce"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("nonce"),
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
                    ::std::borrow::ToOwned::to_owned("pools"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("pools"),
                            inputs: ::std::vec![
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("strategy"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenX"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenY"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("reserveX"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("reserveY"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("totalLiquidity"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("liquidityToken"),
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
                (
                    ::std::borrow::ToOwned::to_owned("swap"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("swap"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("update"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("update"),
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
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("weth"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("weth"),
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
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("Allocate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Allocate"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("deltaX"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("deltaY"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("deltaL"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Deallocate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Deallocate"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("deltaX"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("deltaY"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("deltaL"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Init"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Init"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("strategy"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("lpToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("tokenX"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("tokenY"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("reserveX"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("reserveY"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("totalLiquidity"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Swap"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Swap"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("isSwapXForY"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("inputAmount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("outputAmount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("ERC1167FailedCreateClone"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ERC1167FailedCreateClone",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Invalid"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("Invalid"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("negative"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "swapConstantGrowth",
                                    ),
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
                    ::std::borrow::ToOwned::to_owned("InvalidSwap"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidSwap"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidSwapInputTransfer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InvalidSwapInputTransfer",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidSwapOutputTransfer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InvalidSwapOutputTransfer",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidTokens"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidTokens"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Locked"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("Locked"),
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
                    ::std::borrow::ToOwned::to_owned("OnlyWETH"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("OnlyWETH"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
            ]),
            receive: true,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static DFMM_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xC0`@\x90\x80\x82R4b\0\x02\x13WP\x80Q`\x01`\x01`@\x1B\x03\x90`\x1Fb\x006@8\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x91\x84\x83\x11\x84\x84\x10\x17b\0\x01\xA9W\x80\x84\x92` \x94\x88R\x839\x81\x01\x03\x12b\0\x01\xC4WQ`\x01`\x01`\xA0\x1B\x03\x91\x90\x82\x81\x16\x81\x03b\0\x01\xBFW`\x01\x80U`\xA0R\x82Q\x91a\x11D\x92\x83\x81\x01\x93\x81\x85\x10\x84\x86\x11\x17b\0\x01\xA9Wb\0$\xFC\x829\x80`\0\x94\x03\x90\x84\xF0\x80\x15b\0\x01\x9FW\x16\x80`\x80R\x80;\x15b\0\x01MW\x90\x82\x80\x92`\x84\x86Q\x80\x96\x81\x93c&lE\xBB`\xE1\x1B\x83R\x89`\x04\x84\x01R\x81`D\x84\x01R```$\x84\x01R\x81`d\x84\x01RZ\xF1\x80\x15b\0\x01CWb\0\x01\x1CW[\x83Qa\"\x9B\x90\x81b\0\x02a\x829`\x80Q\x81\x81\x81a\x0B\xC6\x01Ra\x10T\x01R`\xA0Q\x81\x81\x81a\x08\xE3\x01R\x81\x81a\x15\xEB\x01R\x81\x81a\x19<\x01Ra\x1Ai\x01R\xF3[\x82\x11b\0\x01/WP\x81R8\x80\x80b\0\0\xDFV[cNH{q`\xE0\x1B\x81R`A`\x04R`$\x90\xFD[\x84Q=\x84\x82>=\x90\xFD[\x83QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01Rd code`\xD8\x1B`d\x82\x01R`\x84\x90\xFD[\x84Q=\x85\x82>=\x90\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x80\xFD[\x82QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[bF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01Ra7\xB7`\xF1\x1B`d\x82\x01R`\x84\x90\xFD\xFE`\x80`@R`\x046\x10\x15a\0\x1DW[6a\r\x9FWa\0\x1Ba\x15\xE9V[\0[`\x005`\xE0\x1C\x80c\x02\x16\xB88\x14a\0\xDDW\x80c\x06\x8B\xCD\x8D\x14a\0\xD8W\x80c\x14U\xF1\xFC\x14a\0\xD3W\x80c.\xC3\x81\x88\x14a\0\xCEW\x80c;\xE6\xA3A\x14a\0\xC9W\x80c?\xC8\xCE\xF3\x14a\0\xC4W\x80c\x9D\x94/\x9A\x14a\0\xBFW\x80c\xACJ\xFA8\x14a\0\xBAW\x80c\xAF\xFE\xD0\xE0\x14a\0\xB5W\x80c\xB4b\xCD%\x14a\0\xB0W\x80c\xBD\x06%\xAB\x14a\0\xABWc\xCE\x15;\xF4\x03a\0\x0EWa\rAV[a\x0B\xF5V[a\x0B\xB0V[a\x0B\x92V[a\x0B\x05V[a\t\x12V[a\x08\xCDV[a\x07\x86V[a\x05LV[a\x04\x96V[a\x03\xE7V[a\x039V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01Ra7\xB7`\xF1\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x90\xFD[`@`\x03\x19\x82\x01\x12a\x034W`\x045\x91`$5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x83\x82\x11a\x03/W\x80`#\x83\x01\x12\x15a\x03*W\x81`\x04\x015\x93\x84\x11a\x02\xD1W`$\x84\x83\x01\x01\x11a\x02xW`$\x01\x91\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rjrray stride`\xA8\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x0ENL/$\r\x8C\xAD\xCC\xEE\x8D`\xAB\x1B`d\x82\x01R`\x84\x90\xFD[a\x01\xD2V[a\x01\x82V[a\x012V[4a\x03\xE2Wa\x03G6a\x02+V[\x91`\x01T\x92`\x02`\0\x94\x14a\x03\xD0W`\x02`\x01Ua\x03d\x82a\n\xB7V[PT`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x03\xCBWa\x03\x9B\x93\x85\x80\x94`@Q\x96\x87\x95\x86\x94\x85\x93c\xAC\xAD)\x89`\xE0\x1B\x85R3`\x04\x86\x01a\x0E\xB6V[\x03\x92Z\xF1\x80\x15a\x03\xC6Wa\x03\xB7W[Pa\x03\xB4`\x01\x80UV[\x80\xF3[a\x03\xC0\x90a\x0E_V[8a\x03\xAAV[a\x0E\xF1V[a\r\xF6V[`@Qc\x03\xCB\x96\xDB`\xE2\x1B\x81R`\x04\x90\xFD[a\0\xE2V[4a\x03\xE2W` 6`\x03\x19\x01\x12a\x034W`@Qa\x04\x04\x81a\x0ExV[`\xC0`\0\x91\x82\x81R\x82` \x82\x01R\x82`@\x82\x01R\x82``\x82\x01R\x82`\x80\x82\x01R\x82`\xA0\x82\x01R\x01R`\xE0a\x04Ba\x04<`\x045a\n\xB7V[Pa\x0F\x0CV[`@Q\x90`\xC0`\x01\x80`\xA0\x1B\x03\x91\x82\x81Q\x16\x84R\x82` \x82\x01Q\x16` \x85\x01R\x82`@\x82\x01Q\x16`@\x85\x01R``\x81\x01Q``\x85\x01R`\x80\x81\x01Q`\x80\x85\x01R`\xA0\x81\x01Q`\xA0\x85\x01R\x01Q\x16`\xC0\x82\x01R\xF3[`\x03\x19` 6\x82\x01\x12a\x034W`\x045\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x03/W`\x80\x90\x826\x03\x01\x12a\x04\xF7Wa\x04\xD1a\x04\xF3\x91`\x04\x01a\x0F\x9DV[`@\x80Q\x94\x85R` \x85\x01\x93\x90\x93R\x91\x83\x01R``\x82\x01R\x90\x81\x90`\x80\x82\x01\x90V[\x03\x90\xF3[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FABI decoding: struct calldata to`D\x82\x01Rf\x1B\xC8\x1C\xDA\x1B\xDC\x9D`\xCA\x1B`d\x82\x01R`\x84\x90\xFD[a\x05U6a\x02+V[\x91\x90`\x02`\x01T\x14a\x03\xD0W`\x02`\x01Ua\x05\x92a\x05\x86a\x05\x86a\x05x\x85a\n\xB7V[PT`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x92\x83;\x15a\x03\xCBW`@Qc\x8A\x04\xBD\xD5`\xE0\x1B\x81R\x93`\xA0\x92\x85\x92\x83\x91\x82\x91a\x05\xC0\x91\x883`\x04\x86\x01a\x0E\xB6V[\x03\x91Z\xFA\x90\x81\x15a\x03\xC6W`\0\x90\x81\x82\x80\x95\x81\x95a\x073W[P\x15a\x06\xF9WP\x90\x81a\x05\xFC`\x03a\x05\xF3a\x04\xF3\x95a\n\xB7V[P\x01T\x83a\x14\xDAV[\x93a\x06\x14`\x04a\x06\x0B\x84a\n\xB7V[P\x01T\x87a\x14\xDAV[\x95a\x06#`\x05a\x05\xF3\x85a\n\xB7V[\x93a\x06.\x85\x85a 4V[`\x03a\x069\x85a\n\xB7V[P\x01U`\x04a\x06G\x84a\n\xB7V[P\x01U`\x05a\x06U\x83a\n\xB7V[P\x01Ua\x06\x91\x85`\x01a\x06}\x87a\x06k\x86a\n\xB7V[P\x83\x80`\xA0\x1B\x03\x93\x84\x91\x01T\x16a\x192V[`\x02a\x06\x88\x85a\n\xB7V[P\x01T\x16a\x192V[`@\x80Q\x91\x82R` \x82\x01\x85\x90R\x81\x01\x85\x90R``\x81\x01\x82\x90R3\x90\x7F\x95\x97W\x7F3\x93 w^c\xD3\xFE\xD7\xD5\xDD\xE66[\xAD\xCC\x9F\xCC\xDBf\xB3H\x94c\x0C\xA9\x8Bo\x90\x80`\x80\x81\x01[\x03\x90\xA2`\x01\x80U`@Q\x93\x84\x93\x84`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[\x80a\x07/a\x07\x08`\0\x93a\x16-V[`@Qcw`m)`\xE1\x1B\x81R\x93\x90\x92\x12`\x04\x84\x01R`$\x83\x01\x91\x90\x91R\x81\x90`D\x82\x01\x90V[\x03\x90\xFD[\x93PPP\x92Pa\x07[\x91P`\xA0=`\xA0\x11a\x07iW[a\x07S\x81\x83a\x0E\x94V[\x81\x01\x90a\x13\xF7V[\x94\x91\x90\x92\x90\x92\x94\x938a\x05\xD9V[P=a\x07IV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\x07\x81WV[`\0\x80\xFD[4a\x03\xE2W`@6`\x03\x19\x01\x12a\x034W`\x045a\x07\xA3\x81a\x07pV[`$5\x90a\x07\xCAa\x05\x86a\x05\x86`\x06a\x07\xBB\x86a\n\xB7V[P\x01T`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x90\x81;\x15a\x03\xCBW`@Qcp\xA0\x821`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16`\x04\x82\x01R` \x80\x82`$\x81\x86Z\xFA\x91\x82\x15a\x03\xC6W`\0\x92a\x08\xA3W[P\x82;\x15a\x03\xCBW\x80`\x04\x93`@Q\x94\x85\x80\x92c\x18\x16\r\xDD`\xE0\x1B\x82RZ\xFA\x80\x15a\x03\xC6Wa\x04\xF3\x94a\x08]\x94a\x08W\x93`\0\x93a\x08mW[PPa\x08O`\x05\x91a\n\xB7V[P\x01Ta!\xC6V[\x90a!\xE8V[`@Q\x90\x81R\x90\x81\x90` \x82\x01\x90V[`\x05\x92\x93Pa\x08O\x91\x81a\x08\x95\x92\x90=\x10a\x08\x9CW[a\x08\x8D\x81\x83a\x0E\x94V[\x81\x01\x90a\x15\xA5V[\x92\x91a\x08BV[P=a\x08\x83V[\x81a\x08\xBB\x92\x93P=\x84\x11a\x08\x9CWa\x08\x8D\x81\x83a\x0E\x94V[\x908a\x08\tV[`\0\x91\x03\x12a\x034WV[4a\x03\xE2W`\x006`\x03\x19\x01\x12a\x034W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x03\xE2Wa\t 6a\x02+V[\x91\x90`\x02`\x01T\x14a\x03\xD0W`\x02`\x01Ua\tCa\x05\x86a\x05\x86a\x05x\x85a\n\xB7V[\x92\x83;\x15a\x03\xCBW`@Qc\x8A\x04\xBD\xD5`\xE0\x1B\x81R\x93`\xA0\x92\x85\x92\x83\x91\x82\x91a\tq\x91\x883`\x04\x86\x01a\x0E\xB6V[\x03\x91Z\xFA\x90\x81\x15a\x03\xC6W`\0\x90\x81\x82\x80\x95\x81\x95a\n\x8AW[P\x15a\x06\xF9WP\x90\x81a\t\xAD\x82`\x03a\t\xA5a\x04\xF3\x96a\n\xB7V[P\x01Ta\x14\xDAV[\x93a\t\xBD\x86`\x04a\t\xA5\x85a\n\xB7V[\x95a\t\xCD\x82`\x05a\t\xA5\x86a\n\xB7V[\x93a\t\xD8\x85\x85a!\x07V[`\x03a\t\xE3\x85a\n\xB7V[P\x01U`\x04a\t\xF1\x84a\n\xB7V[P\x01U`\x05a\t\xFF\x83a\n\xB7V[P\x01Ua\nB\x85a\n\x0F\x83a\n\xB7V[P`\x01\x01T`\x01`\x01`\xA0\x1B\x03\x90a\n,\x90\x88\x903\x90\x84\x16a\x1A_V[a\n5\x84a\n\xB7V[P`\x02\x01T3\x91\x16a\x1A_V[`@\x80Q\x91\x82R` \x82\x01\x85\x90R\x81\x01\x85\x90R``\x81\x01\x82\x90R3\x90\x7F\xAC\xBE\x12~\x93\xA8\xA0\xB2x\xD8\xE0n' [=\xF9\xD1\xF3\x81$\x14\xBC\x89\x17\xC7t\xA87\x101n\x90\x80`\x80\x81\x01a\x06\xD5V[\x93PPP\x92Pa\n\xA9\x91P`\xA0=`\xA0\x11a\x07iWa\x07S\x81\x83a\x0E\x94V[\x94\x91\x90\x92\x90\x92\x94\x938a\t\x8AV[\x90`\0\x91\x82T\x81\x10\x15a\n\xF1W`\x07\x90\x83\x80R\x02\x7F)\r\xEC\xD9T\x8Bb\xA8\xD6\x03E\xA9\x888o\xC8K\xA6\xBC\x95H@\x08\xF66/\x93\x16\x0E\xF3\xE5c\x01\x91\x90V[cNH{q`\xE0\x1B\x83R`2`\x04R`$\x83\xFD[4a\x03\xE2W` 6`\x03\x19\x01\x12a\x034W`\x045`\0T\x81\x10\x15a\x07\x81Wa\x0B,\x90a\n\xB7V[P\x80T`\x01\x82\x01T`\x02\x83\x01T`\x03\x84\x01T`\x04\x85\x01T`\x05\x86\x01T`\x06\x90\x96\x01T`@\x80Q`\x01`\x01`\xA0\x1B\x03\x97\x88\x16\x81R\x95\x87\x16` \x87\x01R\x93\x86\x16\x93\x85\x01\x93\x90\x93R``\x84\x01\x91\x90\x91R`\x80\x83\x01R`\xA0\x82\x01\x93\x90\x93R\x91\x16`\xC0\x82\x01R`\xE0\x90\xF3[4a\x03\xE2W`\x006`\x03\x19\x01\x12a\x034W` `\0T`@Q\x90\x81R\xF3[4a\x03\xE2W`\x006`\x03\x19\x01\x12a\x034W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x03\xE2Wa\x0C\x036a\x02+V[\x91\x90`\x02`\x01T\x14a\x03\xD0W`\x02`\x01Ua\x0C&a\x05\x86a\x05\x86a\x05x\x85a\n\xB7V[\x92\x83;\x15a\x03\xCBW`@Qc\r\x17\xA7\xC7`\xE3\x1B\x81R\x93`\xC0\x92\x85\x92\x83\x91\x82\x91a\x0CT\x91\x883`\x04\x86\x01a\x0E\xB6V[\x03\x91Z\xFA\x91\x82\x15a\x03\xC6W`\0\x80\x93\x81\x80\x93\x81\x92a\r\x04W[P\x15a\x0C\xF5W\x83\x94P`\x05a\x0C\x84a\x0C\x8D\x95a\n\xB7V[P\x01U\x83a\x1B\x85V[\x94\x92P\x92\x90P\x7FL}\xEF\x84\xE4++\xC0\xA5\xAA\xB2\"\x86\x8D\xD7\xA0\x92\xB53w\xA4\xB57\xAB\xCD\x944Zz\x85'\xED`@Q\x80a\x0C\xD8\x87\x873\x96\x84`@\x91\x94\x93\x92``\x82\x01\x95\x15\x15\x82R` \x82\x01R\x01RV[\x03\x90\xA3a\x0C\xE4`\x01\x80UV[`@\x80Q\x91\x82R` \x82\x01\x92\x90\x92R\xF3[`\0\x85a\x07/a\x07\x08\x82a\x16-V[\x93PPPPa\r,\x91\x92P`\xC0=`\xC0\x11a\r:W[a\r$\x81\x83a\x0E\x94V[\x81\x01\x90a\x15\xB4V[\x93\x95\x94\x90\x93\x91\x92P8a\x0CmV[P=a\r\x1AV[4a\x03\xE2W` 6`\x03\x19\x01\x12a\x034W`\x045`\x03a\r`\x82a\n\xB7V[P\x01Ta\x04\xF3`\x05a\r\x7F`\x04a\rv\x86a\n\xB7V[P\x01T\x94a\n\xB7V[P\x01T`@Q\x93\x84\x93\x84`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`)`$\x82\x01R\x7FUnknown signature and no fallbac`D\x82\x01Rh\x1A\xC8\x19\x19Y\x9A[\x99Y`\xBA\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01Rd code`\xD8\x1B`d\x82\x01R`\x84\x90\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x0EsW`@RV[a\x0EIV[`\xE0\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x0EsW`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x0EsW`@RV[\x92\x84\x92`\x80\x95\x92`\x01\x80`\xA0\x1B\x03\x16\x85R` \x85\x01R```@\x85\x01R\x81``\x85\x01R\x84\x84\x017`\0\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[`@Q=`\0\x82>=\x90\xFD[`@Q\x90a\x0F\n\x82a\x0ExV[V[\x90a\x0F\n`@Qa\x0F\x1C\x81a\x0ExV[`\xC0a\x0F\x8F`\x06\x83\x96`\x01\x80`\xA0\x1B\x03\x80\x82T\x16\x86R`\x01\x82\x01T\x16` \x86\x01Ra\x0Fca\x0FS`\x02\x83\x01T`\x01\x80`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16`@\x87\x01RV[`\x03\x81\x01T``\x86\x01R`\x04\x81\x01T`\x80\x86\x01R`\x05\x81\x01T`\xA0\x86\x01R\x01T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x91\x01RV[`\x02`\x01T\x14a\x03\xD0W`\x02`\x01U` \x81\x01a\x0F\xB9\x81a\x12\xD6V[\x92`@\x92\x83\x81\x01\x93a\x0F\xCDa\x05\x86\x86a\x12\xD6V[`\x01`\x01`\xA0\x1B\x03\x96\x87\x16\x14a\x12\xC6Wa\x0F\xECa\x05\x86a\x05\x86\x84a\x12\xD6V[\x94`\0\x96\x87Ta\x0F\xFF``\x86\x01\x86a\x13(V[\x90\x98\x80;\x15a\x03\xCBWa\x10-\x99\x8B`\xA0\x94\x88Q\x9C\x8D\x95\x86\x94\x85\x93cs\xCB-\x03`\xE0\x1B\x85R3`\x04\x86\x01a\x0E\xB6V[\x03\x92Z\xF1\x92\x83\x15a\x03\xC6W\x88\x80\x81\x9A\x82\x9A\x83\x97a\x12\x94W[P\x15a\x12aWPa\x10xa\x05\x86\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x16ZV[\x92a\x10\x9Da\x10\x85\x88a\x12\xD6V[a\x10\x8E\x8Ba\x12\xD6V[a\x10\x97\x88a\x12\xD6V[\x91a\x18*V[\x93\x16\x92\x83;\x15a\x03\xCBW\x81a\x10\xC6\x91\x84Q\x80\x93\x81\x92c&lE\xBB`\xE1\x1B\x83R\x80`\x04\x84\x01a\x14mV[\x03\x81\x83\x88Z\xF1\x80\x15a\x03\xC6Wa\x12NW[Pa\x10\xE1\x85a\x14\xA8V[\x90\x83;\x15a\x03\xCBW\x82Qc@\xC1\x0F\x19`\xE0\x1B\x80\x82R3`\x04\x83\x01R`$\x82\x01\x93\x90\x93R\x81\x81`D\x81\x83\x89Z\xF1\x80\x15a\x03\xC6Wa\x12;W[P\x83;\x15a\x03\xCBW\x82Q\x91\x82R`\0`\x04\x83\x01Ra\x03\xE8`$\x83\x01R\x80\x82`D\x81\x83\x88Z\xF1\x94\x85\x15a\x03\xC6Wa\x12\x04\x8Ca\x11\xFFa\x11\xF9a\x11\xFF\x95\x8F\x97\x8Fa\x11\xCAa\x12\x14\x9F\x9Ba\x11\x81a\x12\x0F\x9Fa\x12\t\x9Ea\x11\xF3\x96a\x11{\x92a\x12\"W[Pa\x12\xD6V[\x93a\x12\xD6V[\x90a\x11\xB9a\x11\x8E\x8Ca\x12\xD6V[\x92a\x11\xA9a\x11\x9Aa\x0E\xFDV[`\x01`\x01`\xA0\x1B\x03\x90\x97\x16\x87RV[`\x01`\x01`\xA0\x1B\x03\x16` \x86\x01RV[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90\x83\x01RV[``\x81\x01\x86\x90R`\x80\x81\x01\x8A\x90R`\xA0\x81\x01\x8E\x90R`\x01`\x01`\xA0\x1B\x03\x8C\x16`\xC0\x82\x01Ra\x14\xE7V[Ta\x14\xBDV[\x9Ca\x12\xD6V[a\x192V[a\x12\xD6V[\x85a\x19\xDCV[a\x14\xA8V[\x91\x93\x92\x91\x90a\x0F\n`\x01\x80UV[\x80a\x12/a\x125\x92a\x0E_V[\x80a\x08\xC2V[8a\x11uV[\x80a\x12/a\x12H\x92a\x0E_V[8a\x11\x18V[\x80a\x12/a\x12[\x92a\x0E_V[8a\x10\xD7V[a\x07/\x83a\x12n\x83a\x16-V[\x90Qcw`m)`\xE1\x1B\x81R\x93\x90\x92\x12`\x04\x84\x01R`$\x83\x01\x91\x90\x91R\x81\x90`D\x82\x01\x90V[\x93\x9BPPP\x93Pa\x12\xB5\x91\x97P`\xA0=`\xA0\x11a\x07iWa\x07S\x81\x83a\x0E\x94V[\x91\x9A\x90\x99\x91\x95\x91\x93\x90\x92\x908a\x10EV[Qc3\x91\n\xEF`\xE1\x1B\x81R`\x04\x90\xFD[5a\x12\xE0\x81a\x07pV[\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FCalldata tail too short\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x90\xFD[\x905\x90`\x1E\x19\x816\x03\x01\x82\x12\x15a\x13\xA5W\x01\x805\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x13`W` \x01\x91\x816\x03\x83\x13a\x13[WV[a\x12\xE3V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FInvalid calldata tail length\0\0\0\0`D\x82\x01R`d\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FInvalid calldata tail offset\0\0\0\0`D\x82\x01R`d\x90\xFD[Q\x90\x81\x15\x15\x82\x03a\x07\x81WV[\x90\x81`\xA0\x91\x03\x12a\x034Wa\x14\x0B\x81a\x13\xEAV[\x91` \x82\x01Q\x91`@\x81\x01Q\x91`\x80``\x83\x01Q\x92\x01Q\x90V[`\0[\x83\x81\x10a\x148WPP`\0\x91\x01RV[\x81\x81\x01Q\x83\x82\x01R` \x01a\x14(V[\x90` \x91a\x14a\x81Q\x80\x92\x81\x85R\x85\x80\x86\x01\x91\x01a\x14%V[`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[\x90\x91a\x14\x84a\x12\xE0\x93`@\x84R`@\x84\x01\x90a\x14HV[\x91` \x81\x84\x03\x91\x01Ra\x14HV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[a\x03\xE7\x19\x81\x01\x91\x90\x82\x11a\x14\xB8WV[a\x14\x92V[`\0\x19\x81\x01\x91\x90\x82\x11a\x14\xB8WV[`\x12\x03\x90`\x12\x82\x11a\x14\xB8WV[\x91\x90\x82\x03\x91\x82\x11a\x14\xB8WV[`\0Th\x01\0\0\0\0\0\0\0\0\x81\x10\x15a\x0EsW\x80`\x01a\x15\x0B\x92\x01`\0Ua\n\xB7V[a\x15\x8FW\x81Q\x81T`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x82\x16\x17\x83U` \x84\x01Q`\x01\x84\x01\x80T\x91\x84\x16\x91\x83\x16\x91\x90\x91\x17\x90U`@\x84\x01Q`\x02\x84\x01\x80T\x83\x16\x91\x84\x16\x91\x90\x91\x17\x90U``\x84\x01Q`\x03\x84\x01U`\x80\x84\x01Q`\x04\x84\x01U`\xA0\x84\x01Q`\x05\x84\x01U`\xC0\x90\x93\x01Q`\x06\x90\x92\x01\x80T\x90\x93\x16\x91\x16\x17\x90UV[cNH{q`\xE0\x1B`\0R`\0`\x04R`$`\0\xFD[\x90\x81` \x91\x03\x12a\x034WQ\x90V[\x91\x90\x82`\xC0\x91\x03\x12a\x034Wa\x15\xC9\x82a\x13\xEAV[\x91` \x81\x01Q\x91`@\x82\x01Q\x91``\x81\x01Q\x91`\xA0`\x80\x83\x01Q\x92\x01Q\x90V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x163\x03a\x16\x1BWV[`@Qc\x01\xF1\x80\xC9`\xE0\x1B\x81R`\x04\x90\xFD[`\x01`\xFF\x1B\x81\x14a\x16HW`\0\x81\x12\x15a\x12\xE0W\x19`\x01\x01\x90V[`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x90\xFD[nZ\xF4=\x82\x80>\x90=\x91`+W\xFD[\xF3\x90v=`-\x80`\n=9\x81\xF36==7===6=s\0\0\0b\xFF\xFF\xFF\x82`\x88\x1C\x16\x17`\0R`x\x1B\x17` R`7`\t`\0\xF0\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x15a\x16\xB0WV[`@Qc0\xBE\x1A=`\xE2\x1B\x81R`\x04\x90\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x0EsW`\x1F\x01`\x1F\x19\x16` \x01\x90V[` \x81\x83\x03\x12a\x034W\x80Q\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x03/W\x01\x81`\x1F\x82\x01\x12\x15a\x03*W\x80Qa\x17\x11\x81a\x16\xC2V[\x92a\x17\x1F`@Q\x94\x85a\x0E\x94V[\x81\x84R` \x82\x84\x01\x01\x11a\x17=Wa\x12\xE0\x91` \x80\x85\x01\x91\x01a\x14%V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R`\x84\x90\xFD[a\x0F\n\x92\x94\x93`(\x92`@Q\x96\x87\x93dDFMM-`\xD8\x1B` \x86\x01Ra\x17\xC3\x81Q\x80\x92` `%\x89\x01\x91\x01a\x14%V[\x84\x01\x91`-`\xF8\x1B\x92\x83`%\x82\x01Ra\x17\xE6\x82Q\x80\x93` `&\x85\x01\x91\x01a\x14%V[\x01\x82`&\x82\x01Ra\x18\x01\x82Q\x80\x93` `'\x85\x01\x91\x01a\x14%V[\x01\x90`'\x82\x01Ra\x18\x1B\x82Q\x80\x93` \x87\x85\x01\x91\x01a\x14%V[\x01\x03`\x08\x81\x01\x85R\x01\x83a\x0E\x94V[`\x01`\x01`\xA0\x1B\x03\x92\x90\x83\x16\x90\x81;\x15a\x03\xCBW`@Q\x80\x92c\x06\xFD\xDE\x03`\xE0\x1B\x82R\x81`\x04`\0\x95\x86\x93Z\xFA\x93\x84\x15a\x03\xC6W\x85\x91\x84\x95a\x19\x16W[P\x16\x93\x84;\x15a\x03\xCBW`@Q\x91\x83\x83`\x04\x81c\x95\xD8\x9BA`\xE0\x1B\x99\x8A\x82RZ\xFA\x92\x83\x15a\x03\xC6W\x84\x93a\x18\xFAW[P\x16\x93\x84;\x15a\x03\xCBW\x82\x90`\x04`@Q\x80\x97\x81\x93\x82RZ\xFA\x91\x82\x15a\x03\xC6Wa\x12\xE0\x94\x81\x93a\x18\xD2W[Pa\x18\xCC\x90Ta\x1E=V[\x92a\x17\x92V[a\x18\xCC\x91\x93Pa\x18\xF3\x90=\x80\x86\x83>a\x18\xEB\x81\x83a\x0E\x94V[\x81\x01\x90a\x16\xDEV[\x92\x90a\x18\xC1V[a\x19\x0F\x91\x93P=\x80\x86\x83>a\x18\xEB\x81\x83a\x0E\x94V[\x918a\x18\x96V[a\x19+\x91\x95P=\x80\x86\x83>a\x18\xEB\x81\x83a\x0E\x94V[\x938a\x18gV[G\x82\x11a\x19\xB3WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x03\xCBW`\0\x90`\x04`@Q\x80\x94\x81\x93c\r\x0E0\xDB`\xE4\x1B\x83RZ\xF1\x80\x15a\x03\xC6Wa\x19\xA0W[PGa\x19\x96WV[a\x0F\nG3a\x1F\xE9V[\x80a\x12/a\x19\xAD\x92a\x0E_V[8a\x19\x8EV[a\x19\xC9a\x0F\n\x92a\x19\xC3\x83a\x1E\xD6V[\x90a\"\tV[\x900\x903\x90`\x01`\x01`\xA0\x1B\x03\x16a\x1FhV[\x90a\x19\xE9a\x04<\x83a\n\xB7V[`\x01\x80`\xA0\x1B\x03\x91\x82\x82Q\x16\x91\x83` \x82\x01Q\x16\x93\x80`@\x83\x01Q\x16\x95``\x83\x01Q\x91`\xA0`\x80\x85\x01Q\x94\x01Q\x94`@Q\x96\x87R\x16` \x86\x01R`@\x85\x01R``\x84\x01R`\x80\x83\x01R`\xA0\x82\x01R\x7FF\x0B?F\x8A\xE9\xCC\x90\xB3\xD7w\x08\x15\xDEW\n\x18w\xE2\x19\xD9\x9C\x9C\xDD\nf\xB4\x04\x10\xFF\x81\x8E`\xC03\x92\xA4V[`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x93\x92\x91\x90\x81\x16\x90\x81\x85\x03a\x1A\xEDWPP\x82;\x15a\x03\xCBW`@Qc.\x1A}M`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R\x92`\0\x90\x84\x90`$\x90\x82\x90\x84\x90Z\xF1\x92\x83\x15a\x03\xC6Wa\x0F\n\x93a\x1A\xDAW[Pa\x1F\xE9V[\x80a\x12/a\x1A\xE7\x92a\x0E_V[8a\x1A\xD4V[` \x92\x94P\x92a\x1B\0a\x1B\x06\x92\x94a\x1E\xD6V[\x90a!\xC6V[`D`@Q\x94`\0\x80\x95\x81\x94\x82\x93c\xA9\x05\x9C\xBB`\xE0\x1B\x84R`\x04R`$RZ\xF1=\x15`\x1F=\x11`\x01\x84Q\x14\x16\x17\x16\x90``R\x81`@R\x15a\x1BDWPV[bF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rn\x15\x14\x90S\x94\xD1\x91T\x97\xD1\x90RS\x11Q`\x8A\x1B`D\x82\x01R`d\x90\xFD[\x91\x90\x82\x01\x80\x92\x11a\x14\xB8WV[\x92\x91\x90`\x03a\x1B\x93\x85a\n\xB7V[P\x01T\x92a\x1B\xA0\x85a\n\xB7V[P`\x04\x90\x81\x01T\x93\x85\x84\x11\x91\x90\x82\x15a\x1D\xFDW\x85\x81\x10\x15a\x1D\xEDW\x81a\x1C\x05a\x1B\xCD`\x01a\x07\xBB\x8Ca\n\xB7V[\x99a\x1B\xF0\x84a\x1B\xEAa\x1B\xE3`\x02a\x07\xBB\x86a\n\xB7V[\x9C\x8Ba\x14\xDAV[\x9Aa\x14\xDAV[\x97[`\x03a\x1B\xFD\x83a\n\xB7V[P\x01Ua\n\xB7V[P\x01U`\x01`\x01`\xA0\x1B\x03\x87\x81\x16\x80;\x15a\x03\xCBW`@\x80Qcp\xA0\x821`\xE0\x1B\x80\x82R0\x86\x83\x01\x90\x81R\x92\x94\x91\x93\x90\x92\x90\x91` \x91\x82\x90\x86\x90\x81\x90\x83\x01\x03\x81\x86Z\xFA\x94\x85\x15a\x03\xC6W`\0\x95a\x1D\xCEW[P\x8B\x16\x93\x84;\x15a\x03\xCBW\x85Q\x84\x81R0\x88\x82\x01\x90\x81R\x90\x94\x90\x83\x90\x86\x90\x81\x90` \x01\x03\x81\x89Z\xFA\x94\x85\x15a\x03\xC6W\x8D\x8F\x8E\x90\x8E\x93`\0\x99a\x1D\xA1W[Pa\x1C\xAB\x93\x92\x91a\x1C\xA4\x91a\x192V[3\x90a\x1A_V[\x83;\x15a\x03\xCBW\x86Q\x81\x81R0\x89\x82\x01\x90\x81R\x90\x94\x84\x91\x86\x91\x90\x82\x90\x81\x90` \x01\x03\x91Z\xFA\x93\x84\x15a\x03\xC6W`\0\x94a\x1D\x82W[P\x85;\x15a\x03\xCBW\x86Q\x90\x81R0\x88\x82\x01\x90\x81R\x90\x95\x83\x91\x87\x91\x90\x82\x90\x81\x90` \x01\x03\x91Z\xFA\x94\x85\x15a\x03\xC6W\x8B\x92`\0\x96a\x1D[W[PP\x90a\x1D\"\x91a\x1BxV[\x11a\x1DLW\x86a\x1D1\x91a\x14\xDAV[\x11a\x1D?WPP\x94\x93\x92\x91\x90V[Qc\xF3\xCB\xBC\x87`\xE0\x1B\x81R\xFD[PPQc =\x90\x1D`\xE2\x1B\x81R\xFD[a\x1D\"\x93\x92\x96P\x90\x81a\x1Dy\x92\x90=\x10a\x08\x9CWa\x08\x8D\x81\x83a\x0E\x94V[\x94\x90\x918a\x1D\x16V[a\x1D\x9A\x91\x94P\x83=\x85\x11a\x08\x9CWa\x08\x8D\x81\x83a\x0E\x94V[\x928a\x1C\xDFV[a\x1C\xA4\x91\x99P\x91a\x1D\xC3a\x1C\xAB\x95\x94\x93\x89=\x8B\x11a\x08\x9CWa\x08\x8D\x81\x83a\x0E\x94V[\x99\x91P\x91\x92\x93a\x1C\x94V[a\x1D\xE6\x91\x95P\x82=\x84\x11a\x08\x9CWa\x08\x8D\x81\x83a\x0E\x94V[\x938a\x1CWV[P`@Qc\x11\x15vg`\xE0\x1B\x81R\xFD[\x86\x85\x97\x96\x97\x10\x15a\x1D\xEDW\x81a\x1C\x05a\x1E\x1A`\x02a\x07\xBB\x8Ca\n\xB7V[\x99a\x1E7\x88a\x1B\xEAa\x1E0`\x01a\x07\xBB\x86a\n\xB7V[\x9C\x87a\x14\xDAV[\x97a\x1B\xF2V[\x90\x81\x15a\x1E\x80W`N\x91`@Q\x90\x83\x82R\x80`\x80\x83\x01`@R[a\x1EfWP\x82\x01\x91`N\x03\x82RV[\x92`\n\x90\x81\x85\x06`0\x01\x81\x84\x01R`\0\x19\x01\x93\x04\x80a\x1EWV[\x90P`@Q`@\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x0EsW`@R`\x01\x81R`\x03`\xFC\x1B` \x82\x01R\x90V[`M\x81\x11a\x14\xB8W`\n\n\x90V[\x90g\r\xE0\xB6\xB3\xA7d\0\0\x91\x80\x83\x02\x92\x83\x04\x03a\x14\xB8WV[`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x03\xCBW` `\x04\x91`@Q\x92\x83\x80\x92c1<\xE5g`\xE0\x1B\x82RZ\xFA\x90\x81\x15a\x03\xC6W`\0\x91a\x1F*W[Pa\x1F%a\x1F `\xFFa\x12\xE0\x93\x16a\x14\xCCV[a\x1E\xB0V[a\x1E\xBEV[\x90P` \x81=` \x11a\x1F`W[\x81a\x1FE` \x93\x83a\x0E\x94V[\x81\x01\x03\x12a\x034WQ`\xFF\x81\x16\x81\x03a\x07\x81Wa\x1F%a\x1F\rV[=\x91Pa\x1F8V[\x91\x92`d` \x92\x94`@Q\x95`\0\x95\x86\x94\x85\x93\x84\x93c#\xB8r\xDD`\xE0\x1B\x85R`\x04R`$R`DRZ\xF1=\x15`\x1F=\x11`\x01\x84Q\x14\x16\x17\x16\x90``R\x81`@R\x15a\x1F\xB0WPV[bF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rs\x15\x14\x90S\x94\xD1\x91T\x97\xD1\x94\x93\xD3W\xD1\x90RS\x11Q`b\x1B`D\x82\x01R`d\x90\xFD[`\0\x80\x80\x93\x81\x93Z\xF1\x15a\x1F\xF9WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01Rr\x11U\x12\x17\xD5\x14\x90S\x94\xD1\x91T\x97\xD1\x90RS\x11Q`j\x1B`D\x82\x01R`d\x90\xFD[a Ha\x05\x86a\x05\x86`\x06a\x07\xBB\x85a\n\xB7V[\x91\x82;\x15a\x03\xCBW`@Qc\x18\x16\r\xDD`\xE0\x1B\x81R` \x81`\x04\x81\x87Z\xFA\x80\x15a\x03\xC6Wa \x91\x93a\x08W\x92`\0\x92a \xDFW[Pa \x88`\x05\x91a\n\xB7V[P\x01T\x90a!\xC6V[\x90\x80;\x15a\x03\xCBW`@Qc@\xC1\x0F\x19`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x92\x90\x92R`\0\x90\x82\x90\x81\x83\x81`D\x81\x01[\x03\x92Z\xF1\x80\x15a\x03\xC6Wa \xD2WPV[\x80a\x12/a\x0F\n\x92a\x0E_V[`\x05\x91\x92Pa \xFFa \x88\x91` =` \x11a\x08\x9CWa\x08\x8D\x81\x83a\x0E\x94V[\x92\x91Pa |V[a!\x1Ba\x05\x86a\x05\x86`\x06a\x07\xBB\x85a\n\xB7V[\x91\x82;\x15a\x03\xCBW`@Qc\x18\x16\r\xDD`\xE0\x1B\x81R` \x81`\x04\x81\x87Z\xFA\x80\x15a\x03\xC6Wa!j\x93a!d\x92`\0\x92a!\x9EW[Pa![`\x05\x91a\n\xB7V[P\x01T\x90a\"\tV[\x90a\"9V[\x90\x80;\x15a\x03\xCBW`@Qc'p\xA7\xEB`\xE2\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x92\x90\x92R`\0\x90\x82\x90\x81\x83\x81`D\x81\x01a \xC1V[`\x05\x91\x92Pa!\xBEa![\x91` =` \x11a\x08\x9CWa\x08\x8D\x81\x83a\x0E\x94V[\x92\x91Pa!OV[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17\x82\x15\x15\x16\x15a\x07\x81W\x04\x90V[\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x07\x81Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x04\x90V[\x90g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x81\x02\x92\x81\x84\x04\x14\x90\x15\x17\x81\x15\x15\x16\x15a\x07\x81W`\x01\x90`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x07\x81W`\x01g\r\xE0\xB6\xB3\xA7d\0\0`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V\xFE\xA2dipfsX\"\x12 \x03\x7F\x02\x08\xFB\x95T\xFDH\x1Dqr,\xD0\x1FJ\x17\0'\x15\xED\xC4\x17\xA0\x8D\x0F.e\xA7\xBF\x08\xF7dsolcC\0\x08\x16\x003`\x80\x80`@R4a\0\x19W`@Qa\x10\xDD\x90\x81a\0g\x829\xF3[bF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01Ra7\xB7`\xF1\x1B`d\x82\x01R`\x84\x90\xFD\xFE`@`\x80\x81R`\x04\x90\x816\x10\x15a\0rW[\x90` `\x84\x92Q\x91bF\x1B\xCD`\xE5\x1B\x83R\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01Rtnor receive functions`X\x1B`d\x82\x01R\xFD[`\0\x805`\xE0\x1C\x80c\x06\xFD\xDE\x03\x14a\x0B\x82W\x80c\t^\xA7\xB3\x14a\x0B\x13W\x80c\x15\x8E\xF9>\x14a\n\xEEW\x80c\x18\x16\r\xDD\x14a\n\xD1W\x80c#\xB8r\xDD\x14a\n\x11W\x80c1<\xE5g\x14a\t\xF7W\x80c6D\xE5\x15\x14a\t\xD5W\x80c@\xC1\x0F\x19\x14a\tMW\x80cL\xD8\x8Bv\x14a\x066W\x80cp\xA0\x821\x14a\x05\xFEW\x80c~\xCE\xBE\0\x14a\x05\xC6W\x80c\x95\xD8\x9BA\x14a\x04\xE4W\x80c\x9D\xC2\x9F\xAC\x14a\x04cW\x80c\xA9\x05\x9C\xBB\x14a\x03\xF1W\x80c\xAF\xBA\x13\xC4\x14a\x03\xCAW\x80c\xD5\x05\xAC\xCF\x14a\x01\x8BWc\xDDb\xED>\x14a\x019WPa\0\x11V[\x914a\x01\x86W\x81`\x03\x196\x01\x12a\x01\x81W` \x92\x82\x91a\x01Wa\r\xC0V[a\x01_a\r\xDBV[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x84R\x91\x86R\x83\x83 \x91\x16\x82R\x84R T\x90Q\x90\x81R\xF3[a\x0CeV[a\x0C\x15V[P\x904a\x01\x86W`\xE06`\x03\x19\x01\x12a\x01\x81Wa\x01\xA6a\r\xC0V[\x90a\x01\xAFa\r\xDBV[\x91`D5`d5\x92`\x845\x92`\xFF\x84\x16\x80\x94\x03a\x03\xC6WB\x85\x10a\x03\x83Wa\x01\xD5a\x0F\x18V[\x95`\x01\x80`\xA0\x1B\x03\x80\x92\x16\x95\x86\x89R` \x95`\x07\x87R\x84\x8A \x98\x89T\x99`\x01\x8B\x01\x90U\x85Q\x92\x85\x89\x85\x01\x95\x7Fnq\xED\xAE\x12\xB1\xB9\x7FM\x1F`7\x0F\xEF\x10\x10_\xA2\xFA\xAE\x01&\x11J\x16\x9Cd\x84]a&\xC9\x87R\x8B\x89\x87\x01R\x16\x9A\x8B``\x86\x01R\x88`\x80\x86\x01R`\xA0\x85\x01R`\xC0\x84\x01R`\xC0\x83R`\xE0\x83\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x94\x84\x82\x10\x86\x83\x11\x17a\x03oW\x81\x88R\x84Q\x90 a\x01\0\x85\x01\x92a\x19\x01`\xF0\x1B\x84Ra\x01\x02\x86\x01Ra\x01\"\x85\x01R`B\x81Ra\x01`\x84\x01\x94\x81\x86\x10\x90\x86\x11\x17a\x03\\W\x84\x87RQ\x90 \x83Ra\x01\x80\x82\x01R`\xA45a\x01\xA0\x82\x01R`\xC45a\x01\xC0\x90\x91\x01R\x87\x80R\x84\x90\x88\x90`\x80\x90`\x01Z\xFA\x15a\x03RW\x86Q\x16\x96\x87\x15\x15\x80a\x03IW[\x15a\x03\x17W\x86\x97\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x95\x96\x97R\x83R\x80\x87 \x86\x88R\x83R\x81\x81\x88 UQ\x90\x81R\xA3\x80\xF3[\x83`d\x92Q\x91bF\x1B\xCD`\xE5\x1B\x83R\x82\x01R`\x0E`$\x82\x01Rm$\xA7+ \xA6$\xA2/\xA9\xA4\xA3\xA7\"\xA9`\x91\x1B`D\x82\x01R\xFD[P\x84\x88\x14a\x02\xD4V[\x81Q=\x88\x82>=\x90\xFD[cNH{q`\xE0\x1B\x8CR`A\x8DR`$\x8C\xFD[PcNH{q`\xE0\x1B\x8CR`A\x8DR`$\x8C\xFD[\x81QbF\x1B\xCD`\xE5\x1B\x81R` \x81\x8A\x01R`\x17`$\x82\x01R\x7FPERMIT_DEADLINE_EXPIRED\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x90\xFD[\x86\x80\xFD[P4a\x01\x86W6`\x03\x19\x01\x12a\x01\x81W`\x08T\x90Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[P\x904a\x01\x86W\x80`\x03\x196\x01\x12a\x01\x81W` \x91a\x04\x0Ea\r\xC0V[\x82`$5\x913\x84R`\x03\x86R\x81\x84 a\x04(\x84\x82Ta\x0E\xF5V[\x90U`\x01`\x01`\xA0\x1B\x03\x16\x80\x84R`\x03\x86R\x92 \x80T\x82\x01\x90U\x82Q\x90\x81R3\x90`\0\x80Q` a\x10\x88\x839\x81Q\x91R\x90\x85\x90\xA3Q`\x01\x81R\xF3[P\x914a\x01\x86W\x81`\x03\x196\x01\x12a\x01\x81Wa\x04}a\r\xC0V[`\x08T`$5\x92`\x01`\x01`\xA0\x1B\x03\x92\x91\x83\x163\x03a\x04\xD6WP\x84\x93\x92`\0\x80Q` a\x10\x88\x839\x81Q\x91R\x92` \x92\x16\x93\x84\x86R`\x03\x83R\x80\x86 a\x04\xC4\x83\x82Ta\x0E\xF5V[\x90U\x81`\x02T\x03`\x02UQ\x90\x81R\xA3\x80\xF3[\x84QchS\xCB\xA7`\xE0\x1B\x81R\xFD[P4a\x01\x86W\x80`\x03\x196\x01\x12a\x01\x81W\x81Q\x90\x80`\x01\x80T\x90a\x05\x07\x82a\x0C\xB5V[\x80\x86R\x92` \x92`\x01\x81\x16\x90\x81\x15a\x05\x99WP`\x01\x14a\x05AW[a\x05=\x86\x88a\x053\x82\x89\x03\x83a\x0C\xEFV[Q\x91\x82\x91\x82a\r'V[\x03\x90\xF3[\x93P`\x01\x84R\x7F\xB1\x0E-Rv\x12\x07;&\xEE\xCD\xFDq~j2\x0C\xF4KJ\xFA\xC2\xB0s-\x9F\xCB\xE2\xB7\xFA\x0C\xF6[\x83\x85\x10a\x05\x86WPPPP\x81\x01` \x01a\x053\x82a\x05=\x86a\x05\"V[\x80T\x86\x86\x01\x84\x01R\x93\x82\x01\x93\x81\x01a\x05iV[\x90Pa\x05=\x97\x95P\x86\x93P` \x92Pa\x053\x94\x91P`\xFF\x19\x16\x82\x84\x01R\x15\x15`\x05\x1B\x82\x01\x01\x92\x94\x86a\x05\"V[P\x904a\x01\x86W` 6`\x03\x19\x01\x12a\x01\x81W` \x91\x81\x90`\x01`\x01`\xA0\x1B\x03a\x05\xEEa\r\xC0V[\x16\x81R`\x07\x84R T\x90Q\x90\x81R\xF3[P\x904a\x01\x86W` 6`\x03\x19\x01\x12a\x01\x81W` \x91\x81\x90`\x01`\x01`\xA0\x1B\x03a\x06&a\r\xC0V[\x16\x81R`\x03\x84R T\x90Q\x90\x81R\xF3[P\x824a\x01\x86W\x82`\x03\x196\x01\x12a\x01\x81Wg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x815\x81\x81\x11a\tHWa\x06g\x906\x90\x84\x01a\r\xF1V[\x91`$5\x82\x81\x11a\tHWa\x06\x7F\x906\x90\x83\x01a\r\xF1V[\x94`\x08T\x90`\xFF\x82`\xA0\x1C\x16a\t:WP`\x01`\x01`\xA0\x1B\x03\x19\x163\x17`\x08U\x82Q\x82\x81\x11a\t'W\x80a\x06\xB3\x86Ta\x0C\xB5V[\x94`\x1F\x95\x86\x81\x11a\x08\xCEW[P` \x90\x86\x83\x11`\x01\x14a\x08_W\x87\x92a\x08TW[PP\x81`\x01\x1B\x91`\0\x19\x90`\x03\x1B\x1C\x19\x16\x17\x84U[\x84Q\x91\x82\x11a\x08AWP`\x01\x91a\x07\0\x83Ta\x0C\xB5V[\x81\x81\x11a\x07\xDFW[P` \x90\x82\x11`\x01\x14a\x07dW\x83\x94\x82\x93\x94\x92a\x07YW[PP`\0\x19`\x03\x83\x90\x1B\x1C\x19\x16\x90\x82\x1B\x17\x90U[F`\x05Ua\x07@a\x0F2V[`\x06U`\x08\x80T`\xFF`\xA0\x1B\x19\x16`\x01`\xA0\x1B\x17\x90U\x80\xF3[\x01Q\x90P\x84\x80a\x07 V[\x82\x84R\x7F\xB1\x0E-Rv\x12\x07;&\xEE\xCD\xFDq~j2\x0C\xF4KJ\xFA\xC2\xB0s-\x9F\xCB\xE2\xB7\xFA\x0C\xF6\x90`\x1F\x19\x83\x16\x85[\x81\x81\x10a\x07\xC9WP\x95\x83\x85\x96\x97\x10a\x07\xB0W[PPP\x81\x1B\x01\x90Ua\x074V[\x01Q`\0\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U\x84\x80\x80a\x07\xA3V[\x87\x83\x01Q\x84U\x92\x85\x01\x92` \x92\x83\x01\x92\x01a\x07\x90V[\x83\x85R\x7F\xB1\x0E-Rv\x12\x07;&\xEE\xCD\xFDq~j2\x0C\xF4KJ\xFA\xC2\xB0s-\x9F\xCB\xE2\xB7\xFA\x0C\xF6\x82\x80\x85\x01`\x05\x1C\x82\x01\x92` \x86\x10a\x088W[\x01`\x05\x1C\x01\x90\x84\x90[\x82\x81\x10a\x08-WPPa\x07\x08V[\x86\x81U\x01\x84\x90a\x08\x1FV[\x92P\x81\x92a\x08\x16V[cNH{q`\xE0\x1B\x84R`A\x90R`$\x83\xFD[\x01Q\x90P\x87\x80a\x06\xD4V[\x87\x80R`\0\x80Q` a\x10h\x839\x81Q\x91R\x92P`\x1F\x19\x84\x16\x88[\x81\x81\x10a\x08\xB6WP\x90\x84`\x01\x95\x94\x93\x92\x10a\x08\x9DW[PPP\x81\x1B\x01\x84Ua\x06\xE9V[\x01Q`\0\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U\x87\x80\x80a\x08\x90V[\x92\x93` `\x01\x81\x92\x87\x86\x01Q\x81U\x01\x95\x01\x93\x01a\x08zV[\x90\x91P\x86\x80R`\0\x80Q` a\x10h\x839\x81Q\x91R\x86\x80\x85\x01`\x05\x1C\x82\x01\x92` \x86\x10a\t\x1EW[\x90\x85\x94\x93\x92\x91\x01`\x05\x1C\x01\x90[\x81\x81\x10a\t\x10WPa\x06\xBFV[\x88\x81U\x84\x93P`\x01\x01a\t\x03V[\x92P\x81\x92a\x08\xF6V[cNH{q`\xE0\x1B\x85R`A\x82R`$\x85\xFD[Qb\xDC\x14\x9F`\xE4\x1B\x81R\x90P\xFD[a\rpV[P\x914a\x01\x86W\x81`\x03\x196\x01\x12a\x01\x81Wa\tga\r\xC0V[`\x08T`$5\x92`\x01`\x01`\xA0\x1B\x03\x92\x91\x83\x163\x03a\x04\xD6W`\x02T\x90\x84\x82\x01\x80\x92\x11a\t\xC2WP\x92`\0\x80Q` a\x10\x88\x839\x81Q\x91R\x92` \x92\x87\x95`\x02U\x16\x94\x85\x85R`\x03\x83R\x80\x85 \x82\x81T\x01\x90UQ\x90\x81R\xA3\x80\xF3[cNH{q`\xE0\x1B\x87R`\x11\x90R`$\x86\xFD[P4a\x01\x86W6`\x03\x19\x01\x12a\x01\x81W` \x90a\t\xF0a\x0F\x18V[\x90Q\x90\x81R\xF3[P4a\x01\x86W6`\x03\x19\x01\x12a\x01\x81W` \x90Q`\x12\x81R\xF3[P4a\x01\x86W``6`\x03\x19\x01\x12a\x01\x81Wa\n+a\r\xC0V[`\0\x80Q` a\x10\x88\x839\x81Q\x91Ra\nBa\r\xDBV[`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x80\x85R` \x87\x81R\x86\x86 3\x87R\x81R\x86\x86 T\x90\x97\x91\x94\x88\x93`D5\x93\x89\x93\x85`\x01\x82\x01a\n\xAEW[PPP\x86\x88R`\x03\x85R\x82\x88 a\n\x8F\x85\x82Ta\x0E\xF5V[\x90U\x16\x95\x86\x81R`\x03\x84R \x81\x81T\x01\x90U\x85Q\x90\x81R\xA3Q`\x01\x81R\xF3[a\n\xB7\x91a\x0E\xF5V[\x90\x88\x8AR\x86R\x83\x89 3\x8AR\x86R\x83\x89 U8\x80\x85a\nwV[P4a\x01\x86W6`\x03\x19\x01\x12a\x01\x81W` \x90`\x02T\x90Q\x90\x81R\xF3[P4a\x01\x86W6`\x03\x19\x01\x12a\x01\x81W` \x90`\xFF`\x08T`\xA0\x1C\x16\x90Q\x90\x15\x15\x81R\xF3[P\x914a\x01\x86W\x81`\x03\x196\x01\x12a\x01\x81W` \x92a\x0B0a\r\xC0V[\x91\x83`$5\x92\x83\x923\x82R\x87R\x81\x81 \x94`\x01\x80`\xA0\x1B\x03\x16\x94\x85\x82R\x87R U\x82Q\x90\x81R\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x843\x92\xA3Q`\x01\x81R\xF3[P4a\x0C\x15W\x80`\x03\x196\x01\x12a\x01\x81W\x81Q\x90\x80\x80T\x90a\x0B\xA3\x82a\x0C\xB5V[\x80\x85R\x91` \x91`\x01\x91\x82\x81\x16\x90\x81\x15a\x05\x99WP`\x01\x14a\x0B\xD0Wa\x05=\x86\x88a\x053\x82\x89\x03\x83a\x0C\xEFV[\x80\x80\x95PR`\0\x80Q` a\x10h\x839\x81Q\x91R[\x83\x85\x10a\x0C\x02WPPPP\x81\x01` \x01a\x053\x82a\x05=\x86a\x05\"V[\x80T\x86\x86\x01\x84\x01R\x93\x82\x01\x93\x81\x01a\x0B\xE5V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01Ra7\xB7`\xF1\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[\x90`\x01\x82\x81\x1C\x92\x16\x80\x15a\x0C\xE5W[` \x83\x10\x14a\x0C\xCFWV[cNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[\x91`\x7F\x16\x91a\x0C\xC4V[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\r\x11W`@RV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[` \x80\x82R\x82Q\x81\x83\x01\x81\x90R\x90\x93\x92`\0[\x82\x81\x10a\r\\WPP`@\x92\x93P`\0\x83\x82\x84\x01\x01R`\x1F\x80\x19\x91\x01\x16\x01\x01\x90V[\x81\x81\x01\x86\x01Q\x84\x82\x01`@\x01R\x85\x01a\r:V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[`\x045\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\r\xD6WV[`\0\x80\xFD[`$5\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\r\xD6WV[\x81`\x1F\x82\x01\x12\x15a\x0E\x9CW\x805` \x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\r\x11W`@Q\x93a\x0E'`\x1F\x84\x01`\x1F\x19\x16\x85\x01\x86a\x0C\xEFV[\x82\x85R\x83\x83\x83\x01\x01\x11a\x0EGW\x90\x80\x83`\0\x94\x93\x01\x83\x86\x017\x83\x01\x01R\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x84\x90R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x90\xFD[\x91\x90\x82\x03\x91\x82\x11a\x0F\x02WV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\x05TF\x03a\x0F'W`\x06T\x90V[a\x0F/a\x0F2V[\x90V[`@Q`\0\x90`\0T\x90a\x0FE\x82a\x0C\xB5V[\x80\x82R\x81` \x94\x85\x82\x01\x94`\x01\x90\x87`\x01\x82\x16\x91\x82`\0\x14a\x10IWPP`\x01\x14a\x10\x01W[Pa\x0Fx\x92P\x03\x82a\x0C\xEFV[Q\x90 \x90`@Q\x90\x81\x01\x91\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x83R`@\x82\x01R\x7F\xC8\x9E\xFD\xAAT\xC0\xF2\x0Cz\xDFa(\x82\xDF\tP\xF5\xA9Qc~\x03\x07\xCD\xCBLg/)\x8B\x8B\xC6``\x82\x01RF`\x80\x82\x01R0`\xA0\x82\x01R`\xA0\x81R`\xC0\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\r\x11W`@RQ\x90 \x90V[`\0\x80\x80R\x87\x92P\x90`\0\x80Q` a\x10h\x839\x81Q\x91R[\x85\x83\x10a\x101WPPa\x0Fx\x93P\x82\x01\x018a\x0FkV[\x80T\x83\x88\x01\x85\x01R\x86\x94P\x88\x93\x90\x92\x01\x91\x81\x01a\x10\x1AV[`\xFF\x19\x16\x88Ra\x0Fx\x95\x15\x15`\x05\x1B\x85\x01\x01\x92P8\x91Pa\x0Fk\x90PV\xFE)\r\xEC\xD9T\x8Bb\xA8\xD6\x03E\xA9\x888o\xC8K\xA6\xBC\x95H@\x08\xF66/\x93\x16\x0E\xF3\xE5c\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\xA2dipfsX\"\x12 \x81t\xD6|I\xB2\\\xC2\xD92\x9E\x11\xC1\x0B)\xA6f9\xD1zG\x812\xF7~\xD9\xBC\x17bl\xCA5dsolcC\0\x08\x16\x003";
    /// The bytecode of the contract.
    pub static DFMM_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10\x15a\0\x1DW[6a\r\x9FWa\0\x1Ba\x15\xE9V[\0[`\x005`\xE0\x1C\x80c\x02\x16\xB88\x14a\0\xDDW\x80c\x06\x8B\xCD\x8D\x14a\0\xD8W\x80c\x14U\xF1\xFC\x14a\0\xD3W\x80c.\xC3\x81\x88\x14a\0\xCEW\x80c;\xE6\xA3A\x14a\0\xC9W\x80c?\xC8\xCE\xF3\x14a\0\xC4W\x80c\x9D\x94/\x9A\x14a\0\xBFW\x80c\xACJ\xFA8\x14a\0\xBAW\x80c\xAF\xFE\xD0\xE0\x14a\0\xB5W\x80c\xB4b\xCD%\x14a\0\xB0W\x80c\xBD\x06%\xAB\x14a\0\xABWc\xCE\x15;\xF4\x03a\0\x0EWa\rAV[a\x0B\xF5V[a\x0B\xB0V[a\x0B\x92V[a\x0B\x05V[a\t\x12V[a\x08\xCDV[a\x07\x86V[a\x05LV[a\x04\x96V[a\x03\xE7V[a\x039V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01Ra7\xB7`\xF1\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x90\xFD[`@`\x03\x19\x82\x01\x12a\x034W`\x045\x91`$5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x83\x82\x11a\x03/W\x80`#\x83\x01\x12\x15a\x03*W\x81`\x04\x015\x93\x84\x11a\x02\xD1W`$\x84\x83\x01\x01\x11a\x02xW`$\x01\x91\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rjrray stride`\xA8\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x0ENL/$\r\x8C\xAD\xCC\xEE\x8D`\xAB\x1B`d\x82\x01R`\x84\x90\xFD[a\x01\xD2V[a\x01\x82V[a\x012V[4a\x03\xE2Wa\x03G6a\x02+V[\x91`\x01T\x92`\x02`\0\x94\x14a\x03\xD0W`\x02`\x01Ua\x03d\x82a\n\xB7V[PT`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x03\xCBWa\x03\x9B\x93\x85\x80\x94`@Q\x96\x87\x95\x86\x94\x85\x93c\xAC\xAD)\x89`\xE0\x1B\x85R3`\x04\x86\x01a\x0E\xB6V[\x03\x92Z\xF1\x80\x15a\x03\xC6Wa\x03\xB7W[Pa\x03\xB4`\x01\x80UV[\x80\xF3[a\x03\xC0\x90a\x0E_V[8a\x03\xAAV[a\x0E\xF1V[a\r\xF6V[`@Qc\x03\xCB\x96\xDB`\xE2\x1B\x81R`\x04\x90\xFD[a\0\xE2V[4a\x03\xE2W` 6`\x03\x19\x01\x12a\x034W`@Qa\x04\x04\x81a\x0ExV[`\xC0`\0\x91\x82\x81R\x82` \x82\x01R\x82`@\x82\x01R\x82``\x82\x01R\x82`\x80\x82\x01R\x82`\xA0\x82\x01R\x01R`\xE0a\x04Ba\x04<`\x045a\n\xB7V[Pa\x0F\x0CV[`@Q\x90`\xC0`\x01\x80`\xA0\x1B\x03\x91\x82\x81Q\x16\x84R\x82` \x82\x01Q\x16` \x85\x01R\x82`@\x82\x01Q\x16`@\x85\x01R``\x81\x01Q``\x85\x01R`\x80\x81\x01Q`\x80\x85\x01R`\xA0\x81\x01Q`\xA0\x85\x01R\x01Q\x16`\xC0\x82\x01R\xF3[`\x03\x19` 6\x82\x01\x12a\x034W`\x045\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x03/W`\x80\x90\x826\x03\x01\x12a\x04\xF7Wa\x04\xD1a\x04\xF3\x91`\x04\x01a\x0F\x9DV[`@\x80Q\x94\x85R` \x85\x01\x93\x90\x93R\x91\x83\x01R``\x82\x01R\x90\x81\x90`\x80\x82\x01\x90V[\x03\x90\xF3[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FABI decoding: struct calldata to`D\x82\x01Rf\x1B\xC8\x1C\xDA\x1B\xDC\x9D`\xCA\x1B`d\x82\x01R`\x84\x90\xFD[a\x05U6a\x02+V[\x91\x90`\x02`\x01T\x14a\x03\xD0W`\x02`\x01Ua\x05\x92a\x05\x86a\x05\x86a\x05x\x85a\n\xB7V[PT`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x92\x83;\x15a\x03\xCBW`@Qc\x8A\x04\xBD\xD5`\xE0\x1B\x81R\x93`\xA0\x92\x85\x92\x83\x91\x82\x91a\x05\xC0\x91\x883`\x04\x86\x01a\x0E\xB6V[\x03\x91Z\xFA\x90\x81\x15a\x03\xC6W`\0\x90\x81\x82\x80\x95\x81\x95a\x073W[P\x15a\x06\xF9WP\x90\x81a\x05\xFC`\x03a\x05\xF3a\x04\xF3\x95a\n\xB7V[P\x01T\x83a\x14\xDAV[\x93a\x06\x14`\x04a\x06\x0B\x84a\n\xB7V[P\x01T\x87a\x14\xDAV[\x95a\x06#`\x05a\x05\xF3\x85a\n\xB7V[\x93a\x06.\x85\x85a 4V[`\x03a\x069\x85a\n\xB7V[P\x01U`\x04a\x06G\x84a\n\xB7V[P\x01U`\x05a\x06U\x83a\n\xB7V[P\x01Ua\x06\x91\x85`\x01a\x06}\x87a\x06k\x86a\n\xB7V[P\x83\x80`\xA0\x1B\x03\x93\x84\x91\x01T\x16a\x192V[`\x02a\x06\x88\x85a\n\xB7V[P\x01T\x16a\x192V[`@\x80Q\x91\x82R` \x82\x01\x85\x90R\x81\x01\x85\x90R``\x81\x01\x82\x90R3\x90\x7F\x95\x97W\x7F3\x93 w^c\xD3\xFE\xD7\xD5\xDD\xE66[\xAD\xCC\x9F\xCC\xDBf\xB3H\x94c\x0C\xA9\x8Bo\x90\x80`\x80\x81\x01[\x03\x90\xA2`\x01\x80U`@Q\x93\x84\x93\x84`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[\x80a\x07/a\x07\x08`\0\x93a\x16-V[`@Qcw`m)`\xE1\x1B\x81R\x93\x90\x92\x12`\x04\x84\x01R`$\x83\x01\x91\x90\x91R\x81\x90`D\x82\x01\x90V[\x03\x90\xFD[\x93PPP\x92Pa\x07[\x91P`\xA0=`\xA0\x11a\x07iW[a\x07S\x81\x83a\x0E\x94V[\x81\x01\x90a\x13\xF7V[\x94\x91\x90\x92\x90\x92\x94\x938a\x05\xD9V[P=a\x07IV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\x07\x81WV[`\0\x80\xFD[4a\x03\xE2W`@6`\x03\x19\x01\x12a\x034W`\x045a\x07\xA3\x81a\x07pV[`$5\x90a\x07\xCAa\x05\x86a\x05\x86`\x06a\x07\xBB\x86a\n\xB7V[P\x01T`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x90\x81;\x15a\x03\xCBW`@Qcp\xA0\x821`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16`\x04\x82\x01R` \x80\x82`$\x81\x86Z\xFA\x91\x82\x15a\x03\xC6W`\0\x92a\x08\xA3W[P\x82;\x15a\x03\xCBW\x80`\x04\x93`@Q\x94\x85\x80\x92c\x18\x16\r\xDD`\xE0\x1B\x82RZ\xFA\x80\x15a\x03\xC6Wa\x04\xF3\x94a\x08]\x94a\x08W\x93`\0\x93a\x08mW[PPa\x08O`\x05\x91a\n\xB7V[P\x01Ta!\xC6V[\x90a!\xE8V[`@Q\x90\x81R\x90\x81\x90` \x82\x01\x90V[`\x05\x92\x93Pa\x08O\x91\x81a\x08\x95\x92\x90=\x10a\x08\x9CW[a\x08\x8D\x81\x83a\x0E\x94V[\x81\x01\x90a\x15\xA5V[\x92\x91a\x08BV[P=a\x08\x83V[\x81a\x08\xBB\x92\x93P=\x84\x11a\x08\x9CWa\x08\x8D\x81\x83a\x0E\x94V[\x908a\x08\tV[`\0\x91\x03\x12a\x034WV[4a\x03\xE2W`\x006`\x03\x19\x01\x12a\x034W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x03\xE2Wa\t 6a\x02+V[\x91\x90`\x02`\x01T\x14a\x03\xD0W`\x02`\x01Ua\tCa\x05\x86a\x05\x86a\x05x\x85a\n\xB7V[\x92\x83;\x15a\x03\xCBW`@Qc\x8A\x04\xBD\xD5`\xE0\x1B\x81R\x93`\xA0\x92\x85\x92\x83\x91\x82\x91a\tq\x91\x883`\x04\x86\x01a\x0E\xB6V[\x03\x91Z\xFA\x90\x81\x15a\x03\xC6W`\0\x90\x81\x82\x80\x95\x81\x95a\n\x8AW[P\x15a\x06\xF9WP\x90\x81a\t\xAD\x82`\x03a\t\xA5a\x04\xF3\x96a\n\xB7V[P\x01Ta\x14\xDAV[\x93a\t\xBD\x86`\x04a\t\xA5\x85a\n\xB7V[\x95a\t\xCD\x82`\x05a\t\xA5\x86a\n\xB7V[\x93a\t\xD8\x85\x85a!\x07V[`\x03a\t\xE3\x85a\n\xB7V[P\x01U`\x04a\t\xF1\x84a\n\xB7V[P\x01U`\x05a\t\xFF\x83a\n\xB7V[P\x01Ua\nB\x85a\n\x0F\x83a\n\xB7V[P`\x01\x01T`\x01`\x01`\xA0\x1B\x03\x90a\n,\x90\x88\x903\x90\x84\x16a\x1A_V[a\n5\x84a\n\xB7V[P`\x02\x01T3\x91\x16a\x1A_V[`@\x80Q\x91\x82R` \x82\x01\x85\x90R\x81\x01\x85\x90R``\x81\x01\x82\x90R3\x90\x7F\xAC\xBE\x12~\x93\xA8\xA0\xB2x\xD8\xE0n' [=\xF9\xD1\xF3\x81$\x14\xBC\x89\x17\xC7t\xA87\x101n\x90\x80`\x80\x81\x01a\x06\xD5V[\x93PPP\x92Pa\n\xA9\x91P`\xA0=`\xA0\x11a\x07iWa\x07S\x81\x83a\x0E\x94V[\x94\x91\x90\x92\x90\x92\x94\x938a\t\x8AV[\x90`\0\x91\x82T\x81\x10\x15a\n\xF1W`\x07\x90\x83\x80R\x02\x7F)\r\xEC\xD9T\x8Bb\xA8\xD6\x03E\xA9\x888o\xC8K\xA6\xBC\x95H@\x08\xF66/\x93\x16\x0E\xF3\xE5c\x01\x91\x90V[cNH{q`\xE0\x1B\x83R`2`\x04R`$\x83\xFD[4a\x03\xE2W` 6`\x03\x19\x01\x12a\x034W`\x045`\0T\x81\x10\x15a\x07\x81Wa\x0B,\x90a\n\xB7V[P\x80T`\x01\x82\x01T`\x02\x83\x01T`\x03\x84\x01T`\x04\x85\x01T`\x05\x86\x01T`\x06\x90\x96\x01T`@\x80Q`\x01`\x01`\xA0\x1B\x03\x97\x88\x16\x81R\x95\x87\x16` \x87\x01R\x93\x86\x16\x93\x85\x01\x93\x90\x93R``\x84\x01\x91\x90\x91R`\x80\x83\x01R`\xA0\x82\x01\x93\x90\x93R\x91\x16`\xC0\x82\x01R`\xE0\x90\xF3[4a\x03\xE2W`\x006`\x03\x19\x01\x12a\x034W` `\0T`@Q\x90\x81R\xF3[4a\x03\xE2W`\x006`\x03\x19\x01\x12a\x034W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x03\xE2Wa\x0C\x036a\x02+V[\x91\x90`\x02`\x01T\x14a\x03\xD0W`\x02`\x01Ua\x0C&a\x05\x86a\x05\x86a\x05x\x85a\n\xB7V[\x92\x83;\x15a\x03\xCBW`@Qc\r\x17\xA7\xC7`\xE3\x1B\x81R\x93`\xC0\x92\x85\x92\x83\x91\x82\x91a\x0CT\x91\x883`\x04\x86\x01a\x0E\xB6V[\x03\x91Z\xFA\x91\x82\x15a\x03\xC6W`\0\x80\x93\x81\x80\x93\x81\x92a\r\x04W[P\x15a\x0C\xF5W\x83\x94P`\x05a\x0C\x84a\x0C\x8D\x95a\n\xB7V[P\x01U\x83a\x1B\x85V[\x94\x92P\x92\x90P\x7FL}\xEF\x84\xE4++\xC0\xA5\xAA\xB2\"\x86\x8D\xD7\xA0\x92\xB53w\xA4\xB57\xAB\xCD\x944Zz\x85'\xED`@Q\x80a\x0C\xD8\x87\x873\x96\x84`@\x91\x94\x93\x92``\x82\x01\x95\x15\x15\x82R` \x82\x01R\x01RV[\x03\x90\xA3a\x0C\xE4`\x01\x80UV[`@\x80Q\x91\x82R` \x82\x01\x92\x90\x92R\xF3[`\0\x85a\x07/a\x07\x08\x82a\x16-V[\x93PPPPa\r,\x91\x92P`\xC0=`\xC0\x11a\r:W[a\r$\x81\x83a\x0E\x94V[\x81\x01\x90a\x15\xB4V[\x93\x95\x94\x90\x93\x91\x92P8a\x0CmV[P=a\r\x1AV[4a\x03\xE2W` 6`\x03\x19\x01\x12a\x034W`\x045`\x03a\r`\x82a\n\xB7V[P\x01Ta\x04\xF3`\x05a\r\x7F`\x04a\rv\x86a\n\xB7V[P\x01T\x94a\n\xB7V[P\x01T`@Q\x93\x84\x93\x84`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`)`$\x82\x01R\x7FUnknown signature and no fallbac`D\x82\x01Rh\x1A\xC8\x19\x19Y\x9A[\x99Y`\xBA\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01Rd code`\xD8\x1B`d\x82\x01R`\x84\x90\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x0EsW`@RV[a\x0EIV[`\xE0\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x0EsW`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x0EsW`@RV[\x92\x84\x92`\x80\x95\x92`\x01\x80`\xA0\x1B\x03\x16\x85R` \x85\x01R```@\x85\x01R\x81``\x85\x01R\x84\x84\x017`\0\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[`@Q=`\0\x82>=\x90\xFD[`@Q\x90a\x0F\n\x82a\x0ExV[V[\x90a\x0F\n`@Qa\x0F\x1C\x81a\x0ExV[`\xC0a\x0F\x8F`\x06\x83\x96`\x01\x80`\xA0\x1B\x03\x80\x82T\x16\x86R`\x01\x82\x01T\x16` \x86\x01Ra\x0Fca\x0FS`\x02\x83\x01T`\x01\x80`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16`@\x87\x01RV[`\x03\x81\x01T``\x86\x01R`\x04\x81\x01T`\x80\x86\x01R`\x05\x81\x01T`\xA0\x86\x01R\x01T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x91\x01RV[`\x02`\x01T\x14a\x03\xD0W`\x02`\x01U` \x81\x01a\x0F\xB9\x81a\x12\xD6V[\x92`@\x92\x83\x81\x01\x93a\x0F\xCDa\x05\x86\x86a\x12\xD6V[`\x01`\x01`\xA0\x1B\x03\x96\x87\x16\x14a\x12\xC6Wa\x0F\xECa\x05\x86a\x05\x86\x84a\x12\xD6V[\x94`\0\x96\x87Ta\x0F\xFF``\x86\x01\x86a\x13(V[\x90\x98\x80;\x15a\x03\xCBWa\x10-\x99\x8B`\xA0\x94\x88Q\x9C\x8D\x95\x86\x94\x85\x93cs\xCB-\x03`\xE0\x1B\x85R3`\x04\x86\x01a\x0E\xB6V[\x03\x92Z\xF1\x92\x83\x15a\x03\xC6W\x88\x80\x81\x9A\x82\x9A\x83\x97a\x12\x94W[P\x15a\x12aWPa\x10xa\x05\x86\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x16ZV[\x92a\x10\x9Da\x10\x85\x88a\x12\xD6V[a\x10\x8E\x8Ba\x12\xD6V[a\x10\x97\x88a\x12\xD6V[\x91a\x18*V[\x93\x16\x92\x83;\x15a\x03\xCBW\x81a\x10\xC6\x91\x84Q\x80\x93\x81\x92c&lE\xBB`\xE1\x1B\x83R\x80`\x04\x84\x01a\x14mV[\x03\x81\x83\x88Z\xF1\x80\x15a\x03\xC6Wa\x12NW[Pa\x10\xE1\x85a\x14\xA8V[\x90\x83;\x15a\x03\xCBW\x82Qc@\xC1\x0F\x19`\xE0\x1B\x80\x82R3`\x04\x83\x01R`$\x82\x01\x93\x90\x93R\x81\x81`D\x81\x83\x89Z\xF1\x80\x15a\x03\xC6Wa\x12;W[P\x83;\x15a\x03\xCBW\x82Q\x91\x82R`\0`\x04\x83\x01Ra\x03\xE8`$\x83\x01R\x80\x82`D\x81\x83\x88Z\xF1\x94\x85\x15a\x03\xC6Wa\x12\x04\x8Ca\x11\xFFa\x11\xF9a\x11\xFF\x95\x8F\x97\x8Fa\x11\xCAa\x12\x14\x9F\x9Ba\x11\x81a\x12\x0F\x9Fa\x12\t\x9Ea\x11\xF3\x96a\x11{\x92a\x12\"W[Pa\x12\xD6V[\x93a\x12\xD6V[\x90a\x11\xB9a\x11\x8E\x8Ca\x12\xD6V[\x92a\x11\xA9a\x11\x9Aa\x0E\xFDV[`\x01`\x01`\xA0\x1B\x03\x90\x97\x16\x87RV[`\x01`\x01`\xA0\x1B\x03\x16` \x86\x01RV[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90\x83\x01RV[``\x81\x01\x86\x90R`\x80\x81\x01\x8A\x90R`\xA0\x81\x01\x8E\x90R`\x01`\x01`\xA0\x1B\x03\x8C\x16`\xC0\x82\x01Ra\x14\xE7V[Ta\x14\xBDV[\x9Ca\x12\xD6V[a\x192V[a\x12\xD6V[\x85a\x19\xDCV[a\x14\xA8V[\x91\x93\x92\x91\x90a\x0F\n`\x01\x80UV[\x80a\x12/a\x125\x92a\x0E_V[\x80a\x08\xC2V[8a\x11uV[\x80a\x12/a\x12H\x92a\x0E_V[8a\x11\x18V[\x80a\x12/a\x12[\x92a\x0E_V[8a\x10\xD7V[a\x07/\x83a\x12n\x83a\x16-V[\x90Qcw`m)`\xE1\x1B\x81R\x93\x90\x92\x12`\x04\x84\x01R`$\x83\x01\x91\x90\x91R\x81\x90`D\x82\x01\x90V[\x93\x9BPPP\x93Pa\x12\xB5\x91\x97P`\xA0=`\xA0\x11a\x07iWa\x07S\x81\x83a\x0E\x94V[\x91\x9A\x90\x99\x91\x95\x91\x93\x90\x92\x908a\x10EV[Qc3\x91\n\xEF`\xE1\x1B\x81R`\x04\x90\xFD[5a\x12\xE0\x81a\x07pV[\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FCalldata tail too short\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x90\xFD[\x905\x90`\x1E\x19\x816\x03\x01\x82\x12\x15a\x13\xA5W\x01\x805\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x13`W` \x01\x91\x816\x03\x83\x13a\x13[WV[a\x12\xE3V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FInvalid calldata tail length\0\0\0\0`D\x82\x01R`d\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FInvalid calldata tail offset\0\0\0\0`D\x82\x01R`d\x90\xFD[Q\x90\x81\x15\x15\x82\x03a\x07\x81WV[\x90\x81`\xA0\x91\x03\x12a\x034Wa\x14\x0B\x81a\x13\xEAV[\x91` \x82\x01Q\x91`@\x81\x01Q\x91`\x80``\x83\x01Q\x92\x01Q\x90V[`\0[\x83\x81\x10a\x148WPP`\0\x91\x01RV[\x81\x81\x01Q\x83\x82\x01R` \x01a\x14(V[\x90` \x91a\x14a\x81Q\x80\x92\x81\x85R\x85\x80\x86\x01\x91\x01a\x14%V[`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[\x90\x91a\x14\x84a\x12\xE0\x93`@\x84R`@\x84\x01\x90a\x14HV[\x91` \x81\x84\x03\x91\x01Ra\x14HV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[a\x03\xE7\x19\x81\x01\x91\x90\x82\x11a\x14\xB8WV[a\x14\x92V[`\0\x19\x81\x01\x91\x90\x82\x11a\x14\xB8WV[`\x12\x03\x90`\x12\x82\x11a\x14\xB8WV[\x91\x90\x82\x03\x91\x82\x11a\x14\xB8WV[`\0Th\x01\0\0\0\0\0\0\0\0\x81\x10\x15a\x0EsW\x80`\x01a\x15\x0B\x92\x01`\0Ua\n\xB7V[a\x15\x8FW\x81Q\x81T`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x82\x16\x17\x83U` \x84\x01Q`\x01\x84\x01\x80T\x91\x84\x16\x91\x83\x16\x91\x90\x91\x17\x90U`@\x84\x01Q`\x02\x84\x01\x80T\x83\x16\x91\x84\x16\x91\x90\x91\x17\x90U``\x84\x01Q`\x03\x84\x01U`\x80\x84\x01Q`\x04\x84\x01U`\xA0\x84\x01Q`\x05\x84\x01U`\xC0\x90\x93\x01Q`\x06\x90\x92\x01\x80T\x90\x93\x16\x91\x16\x17\x90UV[cNH{q`\xE0\x1B`\0R`\0`\x04R`$`\0\xFD[\x90\x81` \x91\x03\x12a\x034WQ\x90V[\x91\x90\x82`\xC0\x91\x03\x12a\x034Wa\x15\xC9\x82a\x13\xEAV[\x91` \x81\x01Q\x91`@\x82\x01Q\x91``\x81\x01Q\x91`\xA0`\x80\x83\x01Q\x92\x01Q\x90V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x163\x03a\x16\x1BWV[`@Qc\x01\xF1\x80\xC9`\xE0\x1B\x81R`\x04\x90\xFD[`\x01`\xFF\x1B\x81\x14a\x16HW`\0\x81\x12\x15a\x12\xE0W\x19`\x01\x01\x90V[`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x90\xFD[nZ\xF4=\x82\x80>\x90=\x91`+W\xFD[\xF3\x90v=`-\x80`\n=9\x81\xF36==7===6=s\0\0\0b\xFF\xFF\xFF\x82`\x88\x1C\x16\x17`\0R`x\x1B\x17` R`7`\t`\0\xF0\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x15a\x16\xB0WV[`@Qc0\xBE\x1A=`\xE2\x1B\x81R`\x04\x90\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x0EsW`\x1F\x01`\x1F\x19\x16` \x01\x90V[` \x81\x83\x03\x12a\x034W\x80Q\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x03/W\x01\x81`\x1F\x82\x01\x12\x15a\x03*W\x80Qa\x17\x11\x81a\x16\xC2V[\x92a\x17\x1F`@Q\x94\x85a\x0E\x94V[\x81\x84R` \x82\x84\x01\x01\x11a\x17=Wa\x12\xE0\x91` \x80\x85\x01\x91\x01a\x14%V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R`\x84\x90\xFD[a\x0F\n\x92\x94\x93`(\x92`@Q\x96\x87\x93dDFMM-`\xD8\x1B` \x86\x01Ra\x17\xC3\x81Q\x80\x92` `%\x89\x01\x91\x01a\x14%V[\x84\x01\x91`-`\xF8\x1B\x92\x83`%\x82\x01Ra\x17\xE6\x82Q\x80\x93` `&\x85\x01\x91\x01a\x14%V[\x01\x82`&\x82\x01Ra\x18\x01\x82Q\x80\x93` `'\x85\x01\x91\x01a\x14%V[\x01\x90`'\x82\x01Ra\x18\x1B\x82Q\x80\x93` \x87\x85\x01\x91\x01a\x14%V[\x01\x03`\x08\x81\x01\x85R\x01\x83a\x0E\x94V[`\x01`\x01`\xA0\x1B\x03\x92\x90\x83\x16\x90\x81;\x15a\x03\xCBW`@Q\x80\x92c\x06\xFD\xDE\x03`\xE0\x1B\x82R\x81`\x04`\0\x95\x86\x93Z\xFA\x93\x84\x15a\x03\xC6W\x85\x91\x84\x95a\x19\x16W[P\x16\x93\x84;\x15a\x03\xCBW`@Q\x91\x83\x83`\x04\x81c\x95\xD8\x9BA`\xE0\x1B\x99\x8A\x82RZ\xFA\x92\x83\x15a\x03\xC6W\x84\x93a\x18\xFAW[P\x16\x93\x84;\x15a\x03\xCBW\x82\x90`\x04`@Q\x80\x97\x81\x93\x82RZ\xFA\x91\x82\x15a\x03\xC6Wa\x12\xE0\x94\x81\x93a\x18\xD2W[Pa\x18\xCC\x90Ta\x1E=V[\x92a\x17\x92V[a\x18\xCC\x91\x93Pa\x18\xF3\x90=\x80\x86\x83>a\x18\xEB\x81\x83a\x0E\x94V[\x81\x01\x90a\x16\xDEV[\x92\x90a\x18\xC1V[a\x19\x0F\x91\x93P=\x80\x86\x83>a\x18\xEB\x81\x83a\x0E\x94V[\x918a\x18\x96V[a\x19+\x91\x95P=\x80\x86\x83>a\x18\xEB\x81\x83a\x0E\x94V[\x938a\x18gV[G\x82\x11a\x19\xB3WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x03\xCBW`\0\x90`\x04`@Q\x80\x94\x81\x93c\r\x0E0\xDB`\xE4\x1B\x83RZ\xF1\x80\x15a\x03\xC6Wa\x19\xA0W[PGa\x19\x96WV[a\x0F\nG3a\x1F\xE9V[\x80a\x12/a\x19\xAD\x92a\x0E_V[8a\x19\x8EV[a\x19\xC9a\x0F\n\x92a\x19\xC3\x83a\x1E\xD6V[\x90a\"\tV[\x900\x903\x90`\x01`\x01`\xA0\x1B\x03\x16a\x1FhV[\x90a\x19\xE9a\x04<\x83a\n\xB7V[`\x01\x80`\xA0\x1B\x03\x91\x82\x82Q\x16\x91\x83` \x82\x01Q\x16\x93\x80`@\x83\x01Q\x16\x95``\x83\x01Q\x91`\xA0`\x80\x85\x01Q\x94\x01Q\x94`@Q\x96\x87R\x16` \x86\x01R`@\x85\x01R``\x84\x01R`\x80\x83\x01R`\xA0\x82\x01R\x7FF\x0B?F\x8A\xE9\xCC\x90\xB3\xD7w\x08\x15\xDEW\n\x18w\xE2\x19\xD9\x9C\x9C\xDD\nf\xB4\x04\x10\xFF\x81\x8E`\xC03\x92\xA4V[`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x93\x92\x91\x90\x81\x16\x90\x81\x85\x03a\x1A\xEDWPP\x82;\x15a\x03\xCBW`@Qc.\x1A}M`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R\x92`\0\x90\x84\x90`$\x90\x82\x90\x84\x90Z\xF1\x92\x83\x15a\x03\xC6Wa\x0F\n\x93a\x1A\xDAW[Pa\x1F\xE9V[\x80a\x12/a\x1A\xE7\x92a\x0E_V[8a\x1A\xD4V[` \x92\x94P\x92a\x1B\0a\x1B\x06\x92\x94a\x1E\xD6V[\x90a!\xC6V[`D`@Q\x94`\0\x80\x95\x81\x94\x82\x93c\xA9\x05\x9C\xBB`\xE0\x1B\x84R`\x04R`$RZ\xF1=\x15`\x1F=\x11`\x01\x84Q\x14\x16\x17\x16\x90``R\x81`@R\x15a\x1BDWPV[bF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rn\x15\x14\x90S\x94\xD1\x91T\x97\xD1\x90RS\x11Q`\x8A\x1B`D\x82\x01R`d\x90\xFD[\x91\x90\x82\x01\x80\x92\x11a\x14\xB8WV[\x92\x91\x90`\x03a\x1B\x93\x85a\n\xB7V[P\x01T\x92a\x1B\xA0\x85a\n\xB7V[P`\x04\x90\x81\x01T\x93\x85\x84\x11\x91\x90\x82\x15a\x1D\xFDW\x85\x81\x10\x15a\x1D\xEDW\x81a\x1C\x05a\x1B\xCD`\x01a\x07\xBB\x8Ca\n\xB7V[\x99a\x1B\xF0\x84a\x1B\xEAa\x1B\xE3`\x02a\x07\xBB\x86a\n\xB7V[\x9C\x8Ba\x14\xDAV[\x9Aa\x14\xDAV[\x97[`\x03a\x1B\xFD\x83a\n\xB7V[P\x01Ua\n\xB7V[P\x01U`\x01`\x01`\xA0\x1B\x03\x87\x81\x16\x80;\x15a\x03\xCBW`@\x80Qcp\xA0\x821`\xE0\x1B\x80\x82R0\x86\x83\x01\x90\x81R\x92\x94\x91\x93\x90\x92\x90\x91` \x91\x82\x90\x86\x90\x81\x90\x83\x01\x03\x81\x86Z\xFA\x94\x85\x15a\x03\xC6W`\0\x95a\x1D\xCEW[P\x8B\x16\x93\x84;\x15a\x03\xCBW\x85Q\x84\x81R0\x88\x82\x01\x90\x81R\x90\x94\x90\x83\x90\x86\x90\x81\x90` \x01\x03\x81\x89Z\xFA\x94\x85\x15a\x03\xC6W\x8D\x8F\x8E\x90\x8E\x93`\0\x99a\x1D\xA1W[Pa\x1C\xAB\x93\x92\x91a\x1C\xA4\x91a\x192V[3\x90a\x1A_V[\x83;\x15a\x03\xCBW\x86Q\x81\x81R0\x89\x82\x01\x90\x81R\x90\x94\x84\x91\x86\x91\x90\x82\x90\x81\x90` \x01\x03\x91Z\xFA\x93\x84\x15a\x03\xC6W`\0\x94a\x1D\x82W[P\x85;\x15a\x03\xCBW\x86Q\x90\x81R0\x88\x82\x01\x90\x81R\x90\x95\x83\x91\x87\x91\x90\x82\x90\x81\x90` \x01\x03\x91Z\xFA\x94\x85\x15a\x03\xC6W\x8B\x92`\0\x96a\x1D[W[PP\x90a\x1D\"\x91a\x1BxV[\x11a\x1DLW\x86a\x1D1\x91a\x14\xDAV[\x11a\x1D?WPP\x94\x93\x92\x91\x90V[Qc\xF3\xCB\xBC\x87`\xE0\x1B\x81R\xFD[PPQc =\x90\x1D`\xE2\x1B\x81R\xFD[a\x1D\"\x93\x92\x96P\x90\x81a\x1Dy\x92\x90=\x10a\x08\x9CWa\x08\x8D\x81\x83a\x0E\x94V[\x94\x90\x918a\x1D\x16V[a\x1D\x9A\x91\x94P\x83=\x85\x11a\x08\x9CWa\x08\x8D\x81\x83a\x0E\x94V[\x928a\x1C\xDFV[a\x1C\xA4\x91\x99P\x91a\x1D\xC3a\x1C\xAB\x95\x94\x93\x89=\x8B\x11a\x08\x9CWa\x08\x8D\x81\x83a\x0E\x94V[\x99\x91P\x91\x92\x93a\x1C\x94V[a\x1D\xE6\x91\x95P\x82=\x84\x11a\x08\x9CWa\x08\x8D\x81\x83a\x0E\x94V[\x938a\x1CWV[P`@Qc\x11\x15vg`\xE0\x1B\x81R\xFD[\x86\x85\x97\x96\x97\x10\x15a\x1D\xEDW\x81a\x1C\x05a\x1E\x1A`\x02a\x07\xBB\x8Ca\n\xB7V[\x99a\x1E7\x88a\x1B\xEAa\x1E0`\x01a\x07\xBB\x86a\n\xB7V[\x9C\x87a\x14\xDAV[\x97a\x1B\xF2V[\x90\x81\x15a\x1E\x80W`N\x91`@Q\x90\x83\x82R\x80`\x80\x83\x01`@R[a\x1EfWP\x82\x01\x91`N\x03\x82RV[\x92`\n\x90\x81\x85\x06`0\x01\x81\x84\x01R`\0\x19\x01\x93\x04\x80a\x1EWV[\x90P`@Q`@\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x0EsW`@R`\x01\x81R`\x03`\xFC\x1B` \x82\x01R\x90V[`M\x81\x11a\x14\xB8W`\n\n\x90V[\x90g\r\xE0\xB6\xB3\xA7d\0\0\x91\x80\x83\x02\x92\x83\x04\x03a\x14\xB8WV[`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x03\xCBW` `\x04\x91`@Q\x92\x83\x80\x92c1<\xE5g`\xE0\x1B\x82RZ\xFA\x90\x81\x15a\x03\xC6W`\0\x91a\x1F*W[Pa\x1F%a\x1F `\xFFa\x12\xE0\x93\x16a\x14\xCCV[a\x1E\xB0V[a\x1E\xBEV[\x90P` \x81=` \x11a\x1F`W[\x81a\x1FE` \x93\x83a\x0E\x94V[\x81\x01\x03\x12a\x034WQ`\xFF\x81\x16\x81\x03a\x07\x81Wa\x1F%a\x1F\rV[=\x91Pa\x1F8V[\x91\x92`d` \x92\x94`@Q\x95`\0\x95\x86\x94\x85\x93\x84\x93c#\xB8r\xDD`\xE0\x1B\x85R`\x04R`$R`DRZ\xF1=\x15`\x1F=\x11`\x01\x84Q\x14\x16\x17\x16\x90``R\x81`@R\x15a\x1F\xB0WPV[bF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rs\x15\x14\x90S\x94\xD1\x91T\x97\xD1\x94\x93\xD3W\xD1\x90RS\x11Q`b\x1B`D\x82\x01R`d\x90\xFD[`\0\x80\x80\x93\x81\x93Z\xF1\x15a\x1F\xF9WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01Rr\x11U\x12\x17\xD5\x14\x90S\x94\xD1\x91T\x97\xD1\x90RS\x11Q`j\x1B`D\x82\x01R`d\x90\xFD[a Ha\x05\x86a\x05\x86`\x06a\x07\xBB\x85a\n\xB7V[\x91\x82;\x15a\x03\xCBW`@Qc\x18\x16\r\xDD`\xE0\x1B\x81R` \x81`\x04\x81\x87Z\xFA\x80\x15a\x03\xC6Wa \x91\x93a\x08W\x92`\0\x92a \xDFW[Pa \x88`\x05\x91a\n\xB7V[P\x01T\x90a!\xC6V[\x90\x80;\x15a\x03\xCBW`@Qc@\xC1\x0F\x19`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x92\x90\x92R`\0\x90\x82\x90\x81\x83\x81`D\x81\x01[\x03\x92Z\xF1\x80\x15a\x03\xC6Wa \xD2WPV[\x80a\x12/a\x0F\n\x92a\x0E_V[`\x05\x91\x92Pa \xFFa \x88\x91` =` \x11a\x08\x9CWa\x08\x8D\x81\x83a\x0E\x94V[\x92\x91Pa |V[a!\x1Ba\x05\x86a\x05\x86`\x06a\x07\xBB\x85a\n\xB7V[\x91\x82;\x15a\x03\xCBW`@Qc\x18\x16\r\xDD`\xE0\x1B\x81R` \x81`\x04\x81\x87Z\xFA\x80\x15a\x03\xC6Wa!j\x93a!d\x92`\0\x92a!\x9EW[Pa![`\x05\x91a\n\xB7V[P\x01T\x90a\"\tV[\x90a\"9V[\x90\x80;\x15a\x03\xCBW`@Qc'p\xA7\xEB`\xE2\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x92\x90\x92R`\0\x90\x82\x90\x81\x83\x81`D\x81\x01a \xC1V[`\x05\x91\x92Pa!\xBEa![\x91` =` \x11a\x08\x9CWa\x08\x8D\x81\x83a\x0E\x94V[\x92\x91Pa!OV[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17\x82\x15\x15\x16\x15a\x07\x81W\x04\x90V[\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x07\x81Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x04\x90V[\x90g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x81\x02\x92\x81\x84\x04\x14\x90\x15\x17\x81\x15\x15\x16\x15a\x07\x81W`\x01\x90`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x07\x81W`\x01g\r\xE0\xB6\xB3\xA7d\0\0`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V\xFE\xA2dipfsX\"\x12 \x03\x7F\x02\x08\xFB\x95T\xFDH\x1Dqr,\xD0\x1FJ\x17\0'\x15\xED\xC4\x17\xA0\x8D\x0F.e\xA7\xBF\x08\xF7dsolcC\0\x08\x16\x003";
    /// The deployed bytecode of the contract.
    pub static DFMM_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct DFMM<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for DFMM<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for DFMM<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for DFMM<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for DFMM<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(DFMM)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> DFMM<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    DFMM_ABI.clone(),
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
                DFMM_ABI.clone(),
                DFMM_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `allocate` (0x2ec38188) function
        pub fn allocate(
            &self,
            pool_id: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([46, 195, 129, 136], (pool_id, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deallocate` (0x9d942f9a) function
        pub fn deallocate(
            &self,
            pool_id: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([157, 148, 47, 154], (pool_id, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPool` (0x068bcd8d) function
        pub fn get_pool(
            &self,
            pool_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, Pool> {
            self.0
                .method_hash([6, 139, 205, 141], pool_id)
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
        ///Calls the contract's `init` (0x1455f1fc) function
        pub fn init(
            &self,
            params: InitParams,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([20, 85, 241, 252], (params,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `liquidityOf` (0x3be6a341) function
        pub fn liquidity_of(
            &self,
            account: ::ethers::core::types::Address,
            pool_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([59, 230, 163, 65], (account, pool_id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lpTokenImplementation` (0xb462cd25) function
        pub fn lp_token_implementation(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([180, 98, 205, 37], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `nonce` (0xaffed0e0) function
        pub fn nonce(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([175, 254, 208, 224], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pools` (0xac4afa38) function
        pub fn pools(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::Address,
                ::ethers::core::types::Address,
                ::ethers::core::types::Address,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::Address,
            ),
        > {
            self.0
                .method_hash([172, 74, 250, 56], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `swap` (0xbd0625ab) function
        pub fn swap(
            &self,
            pool_id: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([189, 6, 37, 171], (pool_id, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `update` (0x0216b838) function
        pub fn update(
            &self,
            pool_id: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([2, 22, 184, 56], (pool_id, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `weth` (0x3fc8cef3) function
        pub fn weth(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([63, 200, 206, 243], ())
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `Allocate` event
        pub fn allocate_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AllocateFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Deallocate` event
        pub fn deallocate_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DeallocateFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Init` event
        pub fn init_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, InitFilter> {
            self.0.event()
        }
        ///Gets the contract's `Swap` event
        pub fn swap_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, SwapFilter> {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, DFMMEvents> {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for DFMM<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `ERC1167FailedCreateClone` with signature `ERC1167FailedCreateClone()` and selector `0xc2f868f4`
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
    #[etherror(name = "ERC1167FailedCreateClone", abi = "ERC1167FailedCreateClone()")]
    pub struct ERC1167FailedCreateClone;
    ///Custom Error type `Invalid` with signature `Invalid(bool,uint256)` and selector `0xeec0da52`
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
    #[etherror(name = "Invalid", abi = "Invalid(bool,uint256)")]
    pub struct Invalid {
        pub negative: bool,
        pub swap_constant_growth: ::ethers::core::types::U256,
    }
    ///Custom Error type `InvalidSwap` with signature `InvalidSwap()` and selector `0x11157667`
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
    #[etherror(name = "InvalidSwap", abi = "InvalidSwap()")]
    pub struct InvalidSwap;
    ///Custom Error type `InvalidSwapInputTransfer` with signature `InvalidSwapInputTransfer()` and selector `0x80f64074`
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
    #[etherror(name = "InvalidSwapInputTransfer", abi = "InvalidSwapInputTransfer()")]
    pub struct InvalidSwapInputTransfer;
    ///Custom Error type `InvalidSwapOutputTransfer` with signature `InvalidSwapOutputTransfer()` and selector `0xf3cbbc87`
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
    #[etherror(name = "InvalidSwapOutputTransfer", abi = "InvalidSwapOutputTransfer()")]
    pub struct InvalidSwapOutputTransfer;
    ///Custom Error type `InvalidTokens` with signature `InvalidTokens()` and selector `0x672215de`
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
    #[etherror(name = "InvalidTokens", abi = "InvalidTokens()")]
    pub struct InvalidTokens;
    ///Custom Error type `Locked` with signature `Locked()` and selector `0x0f2e5b6c`
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
    #[etherror(name = "Locked", abi = "Locked()")]
    pub struct Locked;
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
    ///Custom Error type `OnlyWETH` with signature `OnlyWETH()` and selector `0x01f180c9`
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
    #[etherror(name = "OnlyWETH", abi = "OnlyWETH()")]
    pub struct OnlyWETH;
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
    pub enum DFMMErrors {
        ERC1167FailedCreateClone(ERC1167FailedCreateClone),
        Invalid(Invalid),
        InvalidSwap(InvalidSwap),
        InvalidSwapInputTransfer(InvalidSwapInputTransfer),
        InvalidSwapOutputTransfer(InvalidSwapOutputTransfer),
        InvalidTokens(InvalidTokens),
        Locked(Locked),
        Min(Min),
        OnlyWETH(OnlyWETH),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for DFMMErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <ERC1167FailedCreateClone as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ERC1167FailedCreateClone(decoded));
            }
            if let Ok(decoded) = <Invalid as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Invalid(decoded));
            }
            if let Ok(decoded) = <InvalidSwap as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidSwap(decoded));
            }
            if let Ok(decoded) = <InvalidSwapInputTransfer as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidSwapInputTransfer(decoded));
            }
            if let Ok(decoded) = <InvalidSwapOutputTransfer as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidSwapOutputTransfer(decoded));
            }
            if let Ok(decoded) = <InvalidTokens as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidTokens(decoded));
            }
            if let Ok(decoded) = <Locked as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Locked(decoded));
            }
            if let Ok(decoded) = <Min as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Min(decoded));
            }
            if let Ok(decoded) = <OnlyWETH as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OnlyWETH(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for DFMMErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::ERC1167FailedCreateClone(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Invalid(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InvalidSwap(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidSwapInputTransfer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidSwapOutputTransfer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidTokens(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Locked(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Min(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OnlyWETH(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for DFMMErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <ERC1167FailedCreateClone as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Invalid as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <InvalidSwap as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <InvalidSwapInputTransfer as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidSwapOutputTransfer as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidTokens as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector == <Locked as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector == <Min as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OnlyWETH as ::ethers::contract::EthError>::selector() => true,
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for DFMMErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ERC1167FailedCreateClone(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Invalid(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidSwap(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidSwapInputTransfer(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidSwapOutputTransfer(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidTokens(element) => ::core::fmt::Display::fmt(element, f),
                Self::Locked(element) => ::core::fmt::Display::fmt(element, f),
                Self::Min(element) => ::core::fmt::Display::fmt(element, f),
                Self::OnlyWETH(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for DFMMErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<ERC1167FailedCreateClone> for DFMMErrors {
        fn from(value: ERC1167FailedCreateClone) -> Self {
            Self::ERC1167FailedCreateClone(value)
        }
    }
    impl ::core::convert::From<Invalid> for DFMMErrors {
        fn from(value: Invalid) -> Self {
            Self::Invalid(value)
        }
    }
    impl ::core::convert::From<InvalidSwap> for DFMMErrors {
        fn from(value: InvalidSwap) -> Self {
            Self::InvalidSwap(value)
        }
    }
    impl ::core::convert::From<InvalidSwapInputTransfer> for DFMMErrors {
        fn from(value: InvalidSwapInputTransfer) -> Self {
            Self::InvalidSwapInputTransfer(value)
        }
    }
    impl ::core::convert::From<InvalidSwapOutputTransfer> for DFMMErrors {
        fn from(value: InvalidSwapOutputTransfer) -> Self {
            Self::InvalidSwapOutputTransfer(value)
        }
    }
    impl ::core::convert::From<InvalidTokens> for DFMMErrors {
        fn from(value: InvalidTokens) -> Self {
            Self::InvalidTokens(value)
        }
    }
    impl ::core::convert::From<Locked> for DFMMErrors {
        fn from(value: Locked) -> Self {
            Self::Locked(value)
        }
    }
    impl ::core::convert::From<Min> for DFMMErrors {
        fn from(value: Min) -> Self {
            Self::Min(value)
        }
    }
    impl ::core::convert::From<OnlyWETH> for DFMMErrors {
        fn from(value: OnlyWETH) -> Self {
            Self::OnlyWETH(value)
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "Allocate",
        abi = "Allocate(address,uint256,uint256,uint256,uint256)"
    )]
    pub struct AllocateFilter {
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        pub pool_id: ::ethers::core::types::U256,
        pub delta_x: ::ethers::core::types::U256,
        pub delta_y: ::ethers::core::types::U256,
        pub delta_l: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "Deallocate",
        abi = "Deallocate(address,uint256,uint256,uint256,uint256)"
    )]
    pub struct DeallocateFilter {
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        pub pool_id: ::ethers::core::types::U256,
        pub delta_x: ::ethers::core::types::U256,
        pub delta_y: ::ethers::core::types::U256,
        pub delta_l: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "Init",
        abi = "Init(address,address,address,address,address,uint256,uint256,uint256,uint256)"
    )]
    pub struct InitFilter {
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        pub strategy: ::ethers::core::types::Address,
        pub lp_token: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub token_x: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub token_y: ::ethers::core::types::Address,
        pub pool_id: ::ethers::core::types::U256,
        pub reserve_x: ::ethers::core::types::U256,
        pub reserve_y: ::ethers::core::types::U256,
        pub total_liquidity: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "Swap", abi = "Swap(address,uint256,bool,uint256,uint256)")]
    pub struct SwapFilter {
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub pool_id: ::ethers::core::types::U256,
        pub is_swap_x_for_y: bool,
        pub input_amount: ::ethers::core::types::U256,
        pub output_amount: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's events
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
    pub enum DFMMEvents {
        AllocateFilter(AllocateFilter),
        DeallocateFilter(DeallocateFilter),
        InitFilter(InitFilter),
        SwapFilter(SwapFilter),
    }
    impl ::ethers::contract::EthLogDecode for DFMMEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AllocateFilter::decode_log(log) {
                return Ok(DFMMEvents::AllocateFilter(decoded));
            }
            if let Ok(decoded) = DeallocateFilter::decode_log(log) {
                return Ok(DFMMEvents::DeallocateFilter(decoded));
            }
            if let Ok(decoded) = InitFilter::decode_log(log) {
                return Ok(DFMMEvents::InitFilter(decoded));
            }
            if let Ok(decoded) = SwapFilter::decode_log(log) {
                return Ok(DFMMEvents::SwapFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for DFMMEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AllocateFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeallocateFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::InitFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SwapFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AllocateFilter> for DFMMEvents {
        fn from(value: AllocateFilter) -> Self {
            Self::AllocateFilter(value)
        }
    }
    impl ::core::convert::From<DeallocateFilter> for DFMMEvents {
        fn from(value: DeallocateFilter) -> Self {
            Self::DeallocateFilter(value)
        }
    }
    impl ::core::convert::From<InitFilter> for DFMMEvents {
        fn from(value: InitFilter) -> Self {
            Self::InitFilter(value)
        }
    }
    impl ::core::convert::From<SwapFilter> for DFMMEvents {
        fn from(value: SwapFilter) -> Self {
            Self::SwapFilter(value)
        }
    }
    ///Container type for all input parameters for the `allocate` function with signature `allocate(uint256,bytes)` and selector `0x2ec38188`
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
    #[ethcall(name = "allocate", abi = "allocate(uint256,bytes)")]
    pub struct AllocateCall {
        pub pool_id: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `deallocate` function with signature `deallocate(uint256,bytes)` and selector `0x9d942f9a`
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
    #[ethcall(name = "deallocate", abi = "deallocate(uint256,bytes)")]
    pub struct DeallocateCall {
        pub pool_id: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `getPool` function with signature `getPool(uint256)` and selector `0x068bcd8d`
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
    #[ethcall(name = "getPool", abi = "getPool(uint256)")]
    pub struct GetPoolCall {
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
    ///Container type for all input parameters for the `init` function with signature `init((address,address,address,bytes))` and selector `0x1455f1fc`
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
    #[ethcall(name = "init", abi = "init((address,address,address,bytes))")]
    pub struct InitCall {
        pub params: InitParams,
    }
    ///Container type for all input parameters for the `liquidityOf` function with signature `liquidityOf(address,uint256)` and selector `0x3be6a341`
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
    #[ethcall(name = "liquidityOf", abi = "liquidityOf(address,uint256)")]
    pub struct LiquidityOfCall {
        pub account: ::ethers::core::types::Address,
        pub pool_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `lpTokenImplementation` function with signature `lpTokenImplementation()` and selector `0xb462cd25`
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
    #[ethcall(name = "lpTokenImplementation", abi = "lpTokenImplementation()")]
    pub struct LpTokenImplementationCall;
    ///Container type for all input parameters for the `nonce` function with signature `nonce()` and selector `0xaffed0e0`
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
    #[ethcall(name = "nonce", abi = "nonce()")]
    pub struct NonceCall;
    ///Container type for all input parameters for the `pools` function with signature `pools(uint256)` and selector `0xac4afa38`
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
    #[ethcall(name = "pools", abi = "pools(uint256)")]
    pub struct PoolsCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `swap` function with signature `swap(uint256,bytes)` and selector `0xbd0625ab`
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
    #[ethcall(name = "swap", abi = "swap(uint256,bytes)")]
    pub struct SwapCall {
        pub pool_id: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `update` function with signature `update(uint256,bytes)` and selector `0x0216b838`
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
    #[ethcall(name = "update", abi = "update(uint256,bytes)")]
    pub struct UpdateCall {
        pub pool_id: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `weth` function with signature `weth()` and selector `0x3fc8cef3`
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
    #[ethcall(name = "weth", abi = "weth()")]
    pub struct WethCall;
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
    pub enum DFMMCalls {
        Allocate(AllocateCall),
        Deallocate(DeallocateCall),
        GetPool(GetPoolCall),
        GetReservesAndLiquidity(GetReservesAndLiquidityCall),
        Init(InitCall),
        LiquidityOf(LiquidityOfCall),
        LpTokenImplementation(LpTokenImplementationCall),
        Nonce(NonceCall),
        Pools(PoolsCall),
        Swap(SwapCall),
        Update(UpdateCall),
        Weth(WethCall),
    }
    impl ::ethers::core::abi::AbiDecode for DFMMCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AllocateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Allocate(decoded));
            }
            if let Ok(decoded) = <DeallocateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Deallocate(decoded));
            }
            if let Ok(decoded) = <GetPoolCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetPool(decoded));
            }
            if let Ok(decoded) = <GetReservesAndLiquidityCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetReservesAndLiquidity(decoded));
            }
            if let Ok(decoded) = <InitCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Init(decoded));
            }
            if let Ok(decoded) = <LiquidityOfCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LiquidityOf(decoded));
            }
            if let Ok(decoded) = <LpTokenImplementationCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LpTokenImplementation(decoded));
            }
            if let Ok(decoded) = <NonceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Nonce(decoded));
            }
            if let Ok(decoded) = <PoolsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Pools(decoded));
            }
            if let Ok(decoded) = <SwapCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Swap(decoded));
            }
            if let Ok(decoded) = <UpdateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Update(decoded));
            }
            if let Ok(decoded) = <WethCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Weth(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for DFMMCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Allocate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Deallocate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetPool(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetReservesAndLiquidity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Init(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LiquidityOf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LpTokenImplementation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Nonce(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Pools(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Swap(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Update(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Weth(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for DFMMCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Allocate(element) => ::core::fmt::Display::fmt(element, f),
                Self::Deallocate(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPool(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetReservesAndLiquidity(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Init(element) => ::core::fmt::Display::fmt(element, f),
                Self::LiquidityOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::LpTokenImplementation(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Nonce(element) => ::core::fmt::Display::fmt(element, f),
                Self::Pools(element) => ::core::fmt::Display::fmt(element, f),
                Self::Swap(element) => ::core::fmt::Display::fmt(element, f),
                Self::Update(element) => ::core::fmt::Display::fmt(element, f),
                Self::Weth(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AllocateCall> for DFMMCalls {
        fn from(value: AllocateCall) -> Self {
            Self::Allocate(value)
        }
    }
    impl ::core::convert::From<DeallocateCall> for DFMMCalls {
        fn from(value: DeallocateCall) -> Self {
            Self::Deallocate(value)
        }
    }
    impl ::core::convert::From<GetPoolCall> for DFMMCalls {
        fn from(value: GetPoolCall) -> Self {
            Self::GetPool(value)
        }
    }
    impl ::core::convert::From<GetReservesAndLiquidityCall> for DFMMCalls {
        fn from(value: GetReservesAndLiquidityCall) -> Self {
            Self::GetReservesAndLiquidity(value)
        }
    }
    impl ::core::convert::From<InitCall> for DFMMCalls {
        fn from(value: InitCall) -> Self {
            Self::Init(value)
        }
    }
    impl ::core::convert::From<LiquidityOfCall> for DFMMCalls {
        fn from(value: LiquidityOfCall) -> Self {
            Self::LiquidityOf(value)
        }
    }
    impl ::core::convert::From<LpTokenImplementationCall> for DFMMCalls {
        fn from(value: LpTokenImplementationCall) -> Self {
            Self::LpTokenImplementation(value)
        }
    }
    impl ::core::convert::From<NonceCall> for DFMMCalls {
        fn from(value: NonceCall) -> Self {
            Self::Nonce(value)
        }
    }
    impl ::core::convert::From<PoolsCall> for DFMMCalls {
        fn from(value: PoolsCall) -> Self {
            Self::Pools(value)
        }
    }
    impl ::core::convert::From<SwapCall> for DFMMCalls {
        fn from(value: SwapCall) -> Self {
            Self::Swap(value)
        }
    }
    impl ::core::convert::From<UpdateCall> for DFMMCalls {
        fn from(value: UpdateCall) -> Self {
            Self::Update(value)
        }
    }
    impl ::core::convert::From<WethCall> for DFMMCalls {
        fn from(value: WethCall) -> Self {
            Self::Weth(value)
        }
    }
    ///Container type for all return fields from the `allocate` function with signature `allocate(uint256,bytes)` and selector `0x2ec38188`
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
    pub struct AllocateReturn(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `deallocate` function with signature `deallocate(uint256,bytes)` and selector `0x9d942f9a`
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
    pub struct DeallocateReturn(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `getPool` function with signature `getPool(uint256)` and selector `0x068bcd8d`
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
    pub struct GetPoolReturn(pub Pool);
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
    ///Container type for all return fields from the `init` function with signature `init((address,address,address,bytes))` and selector `0x1455f1fc`
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
    pub struct InitReturn(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `liquidityOf` function with signature `liquidityOf(address,uint256)` and selector `0x3be6a341`
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
    pub struct LiquidityOfReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `lpTokenImplementation` function with signature `lpTokenImplementation()` and selector `0xb462cd25`
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
    pub struct LpTokenImplementationReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `nonce` function with signature `nonce()` and selector `0xaffed0e0`
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
    pub struct NonceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `pools` function with signature `pools(uint256)` and selector `0xac4afa38`
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
    pub struct PoolsReturn {
        pub strategy: ::ethers::core::types::Address,
        pub token_x: ::ethers::core::types::Address,
        pub token_y: ::ethers::core::types::Address,
        pub reserve_x: ::ethers::core::types::U256,
        pub reserve_y: ::ethers::core::types::U256,
        pub total_liquidity: ::ethers::core::types::U256,
        pub liquidity_token: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `swap` function with signature `swap(uint256,bytes)` and selector `0xbd0625ab`
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
    pub struct SwapReturn(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `weth` function with signature `weth()` and selector `0x3fc8cef3`
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
    pub struct WethReturn(pub ::ethers::core::types::Address);
    ///`Pool(address,address,address,uint256,uint256,uint256,address)`
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
    pub struct Pool {
        pub strategy: ::ethers::core::types::Address,
        pub token_x: ::ethers::core::types::Address,
        pub token_y: ::ethers::core::types::Address,
        pub reserve_x: ::ethers::core::types::U256,
        pub reserve_y: ::ethers::core::types::U256,
        pub total_liquidity: ::ethers::core::types::U256,
        pub liquidity_token: ::ethers::core::types::Address,
    }
}
