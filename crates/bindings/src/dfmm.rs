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
                inputs: ::std::vec![],
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
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
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static DFMM_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA0`@R`\x01\x80U4\x80\x15b\0\0bW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`@Qb\0\0q\x90b\0\x01[V[`@Q\x80\x91\x03\x90`\0\xF0\x80\x15\x80\x15b\0\0\x8EW=`\0\x80>=`\0\xFD[P`\x01`\x01`\xA0\x1B\x03\x16`\x80\x81\x90R`@\x80Qc&lE\xBB`\xE1\x1B\x81R`\x04\x81\x01\x91\x90\x91R`\0`D\x82\x01\x81\x90R```$\x83\x01R`d\x82\x01RcL\xD8\x8Bv\x90`\x84\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15b\0\x01<W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15b\0\x01QW=`\0\x80>=`\0\xFD[PPPPb\0\x01iV[a\x11\x18\x80b\0*\\\x839\x01\x90V[`\x80Qa(\xD0b\0\x01\x8C`\09`\0\x81\x81a\x02\xE7\x01Ra\x06\xC8\x01Ra(\xD0`\0\xF3\xFE`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\0\xF6W`\x005`\xE0\x1C\x80c\x9D\x94/\x9A\x11a\0\xBEW\x80c\x9D\x94/\x9A\x14a\x02oW\x80c\xACJ\xFA8\x14a\x02\x82W\x80c\xAF\xFE\xD0\xE0\x14a\x02\xDAW\x80c\xB4b\xCD%\x14a\x02\xE2W\x80c\xBD\x06%\xAB\x14a\x03!W\x80c\xCE\x15;\xF4\x14a\x03IWa\0\xF6V[\x80c\x02\x16\xB88\x14a\x01[W\x80c\x06\x8B\xCD\x8D\x14a\x01pW\x80c\x14U\xF1\xFC\x14a\x01\xEDW\x80c.\xC3\x81\x88\x14a\x02 W\x80c;\xE6\xA3A\x14a\x02NW[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x01na\x01i6`\x04a\"\x97V[a\x03\\V[\0[a\x01\x83a\x01~6`\x04a#\xC4V[a\x04UV[`@Qa\x01\xE4\x91\x90\x81Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R` \x80\x84\x01Q\x82\x16\x90\x83\x01R`@\x80\x84\x01Q\x82\x16\x90\x83\x01R``\x80\x84\x01Q\x90\x83\x01R`\x80\x80\x84\x01Q\x90\x83\x01R`\xA0\x83\x81\x01Q\x90\x83\x01R`\xC0\x92\x83\x01Q\x16\x91\x81\x01\x91\x90\x91R`\xE0\x01\x90V[`@Q\x80\x91\x03\x90\xF3[a\x02\0a\x01\xFB6`\x04a#\xE0V[a\x05\x19V[`@\x80Q\x94\x85R` \x85\x01\x93\x90\x93R\x91\x83\x01R``\x82\x01R`\x80\x01a\x01\xE4V[a\x023a\x02.6`\x04a\"\x97V[a\x0BlV[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x01\xE4V[a\x02aa\x02\\6`\x04a$\x88V[a\x0C\x84V[`@Q\x90\x81R` \x01a\x01\xE4V[a\x023a\x02}6`\x04a\"\x97V[a\x0EpV[a\x02\x95a\x02\x906`\x04a#\xC4V[a\x0FnV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x98\x89\x16\x81R\x96\x88\x16` \x88\x01R\x94\x87\x16\x94\x86\x01\x94\x90\x94R``\x85\x01\x92\x90\x92R`\x80\x84\x01R`\xA0\x83\x01R\x90\x91\x16`\xC0\x82\x01R`\xE0\x01a\x01\xE4V[`\0Ta\x02aV[a\x03\t\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\xE4V[a\x034a\x03/6`\x04a\"\x97V[a\x0F\xCDV[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x01\xE4V[a\x023a\x03W6`\x04a#\xC4V[a\x11\xA4V[`\x01T`\x02\x03a\x03\x7FW`@Qc\x03\xCB\x96\xDB`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x01U`\0\x80T\x84\x90\x81\x10a\x03\x98Wa\x03\x98a$\xB5V[`\0\x91\x82R` \x90\x91 `\x07\x90\x91\x02\x01T`@Qc\xAC\xAD)\x89`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xAC\xAD)\x89\x90a\x03\xDC\x903\x90\x87\x90\x87\x90\x87\x90`\x04\x01a$\xCBV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x044W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a({\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x04HW=`\0\x80>=`\0\xFD[PP`\x01\x80UPPPPPV[`@\x80Q`\xE0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R`\xC0\x81\x01\x91\x90\x91R`\0\x82\x81T\x81\x10a\x04\xA1Wa\x04\xA1a$\xB5V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q`\xE0\x81\x01\x82R`\x07\x90\x93\x02\x90\x91\x01\x80T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x84R`\x01\x82\x01T\x81\x16\x94\x84\x01\x94\x90\x94R`\x02\x81\x01T\x84\x16\x91\x83\x01\x91\x90\x91R`\x03\x81\x01T``\x83\x01R`\x04\x81\x01T`\x80\x83\x01R`\x05\x81\x01T`\xA0\x83\x01R`\x06\x01T\x90\x91\x16`\xC0\x82\x01R\x92\x91PPV[`\0\x80`\0\x80`\x01T`\x02\x03a\x05BW`@Qc\x03\xCB\x96\xDB`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x01Ua\x05W``\x86\x01`@\x87\x01a%\x13V[`\x01`\x01`\xA0\x1B\x03\x16a\x05p`@\x87\x01` \x88\x01a%\x13V[`\x01`\x01`\xA0\x1B\x03\x16\x03a\x05\x97W`@Qc3\x91\n\xEF`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80\x80\x80\x80a\x05\xAA` \x8B\x01\x8Ba%\x13V[`\x01`\x01`\xA0\x1B\x03\x16cs\xCB-\x033`\0\x80T\x90P\x8D\x80``\x01\x90a\x05\xCF\x91\x90a%vV[`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x05\xEE\x94\x93\x92\x91\x90a$\xCBV[`\xA0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x06FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a({\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x06ZW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06~\x91\x90a&WV[\x94P\x94P\x94P\x94P\x94P\x84a\x06\xC1W`\0\x84\x12a\x06\x9A\x85a\x12$V[`@Qcw`m)`\xE1\x1B\x81R\x91\x15\x15`\x04\x83\x01R`$\x82\x01R`D\x01[`@Q\x80\x91\x03\x90\xFD[`\0a\x06\xEC\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x12cV[`@\x80Qc&lE\xBB`\xE1\x1B\x81R`\x04\x81\x01\x91\x90\x91R`\0`D\x82\x01\x81\x90R```$\x83\x01R`d\x82\x01R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x82\x16\x90cL\xD8\x8Bv\x90`\x84\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x07\x86W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a({\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x07\x9AW=`\0\x80>=`\0\xFD[PPPP\x80`\x01`\x01`\xA0\x1B\x03\x16c@\xC1\x0F\x193a\x03\xE8\x85a\x07\xBC\x91\x90a&\xB7V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x04\x83\x01R`$\x82\x01R`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x08@W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a({\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x08TW=`\0\x80>=`\0\xFD[PP`@Qc@\xC1\x0F\x19`\xE0\x1B\x81R`\0`\x04\x82\x01Ra\x03\xE8`$\x82\x01R`\x01`\x01`\xA0\x1B\x03\x84\x16\x92Pc@\xC1\x0F\x19\x91P`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x08\xE0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a({\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x08\xF4W=`\0\x80>=`\0\xFD[PP`@\x80Q`\xE0\x81\x01\x90\x91R`\0\x92P\x90P\x80a\t\x15` \x8F\x01\x8Fa%\x13V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8D` \x01` \x81\x01\x90a\t6\x91\x90a%\x13V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8D`@\x01` \x81\x01\x90a\tW\x91\x90a%\x13V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R` \x80\x83\x01\x89\x90R`@\x80\x84\x01\x89\x90R``\x80\x85\x01\x89\x90R\x87\x84\x16`\x80\x95\x86\x01R`\0\x80T`\x01\x81\x81\x01\x83U\x82\x80R\x88Q`\x07\x90\x92\x02\x7F)\r\xEC\xD9T\x8Bb\xA8\xD6\x03E\xA9\x888o\xC8K\xA6\xBC\x95H@\x08\xF66/\x93\x16\x0E\xF3\xE5c\x81\x01\x80T\x93\x89\x16`\x01`\x01`\xA0\x1B\x03\x19\x94\x85\x16\x17\x90U\x95\x89\x01Q\x7F)\r\xEC\xD9T\x8Bb\xA8\xD6\x03E\xA9\x888o\xC8K\xA6\xBC\x95H@\x08\xF66/\x93\x16\x0E\xF3\xE5d\x87\x01\x80T\x91\x89\x16\x91\x84\x16\x91\x90\x91\x17\x90U\x93\x88\x01Q\x7F)\r\xEC\xD9T\x8Bb\xA8\xD6\x03E\xA9\x888o\xC8K\xA6\xBC\x95H@\x08\xF66/\x93\x16\x0E\xF3\xE5e\x86\x01\x80T\x91\x88\x16\x91\x83\x16\x91\x90\x91\x17\x90U\x91\x87\x01Q\x7F)\r\xEC\xD9T\x8Bb\xA8\xD6\x03E\xA9\x888o\xC8K\xA6\xBC\x95H@\x08\xF66/\x93\x16\x0E\xF3\xE5f\x85\x01U\x94\x86\x01Q\x7F)\r\xEC\xD9T\x8Bb\xA8\xD6\x03E\xA9\x888o\xC8K\xA6\xBC\x95H@\x08\xF66/\x93\x16\x0E\xF3\xE5g\x84\x01U`\xA0\x86\x01Q\x7F)\r\xEC\xD9T\x8Bb\xA8\xD6\x03E\xA9\x888o\xC8K\xA6\xBC\x95H@\x08\xF66/\x93\x16\x0E\xF3\xE5h\x84\x01U`\xC0\x86\x01Q\x7F)\r\xEC\xD9T\x8Bb\xA8\xD6\x03E\xA9\x888o\xC8K\xA6\xBC\x95H@\x08\xF66/\x93\x16\x0E\xF3\xE5i\x90\x93\x01\x80T\x93\x90\x94\x16\x92\x16\x91\x90\x91\x17\x90\x91U\x81T\x92\x93P\x90\x91a\n\xFD\x91\x90a&\xB7V[\x90Pa\x0B\x1B\x8D` \x01` \x81\x01\x90a\x0B\x15\x91\x90a%\x13V[\x87a\x12\xD0V[a\x0B7\x8D`@\x01` \x81\x01\x90a\x0B1\x91\x90a%\x13V[\x86a\x12\xD0V[a\x0B@\x81a\x12\xF7V[\x80\x86\x86a\x0BOa\x03\xE8\x88a&\xB7V[\x9BP\x9BP\x9BP\x9BPPPPPPPPP`\x01\x80U\x92\x94\x91\x93P\x91\x90V[`\0\x80`\0`\x01T`\x02\x03a\x0B\x94W`@Qc\x03\xCB\x96\xDB`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x01\x81\x90UP`\0\x80`\0a\x0B\xAE\x89`\x01\x8A\x8Aa\x13\xD9V[\x92P\x92P\x92Pa\x0B\xED`\0\x8A\x81T\x81\x10a\x0B\xCAWa\x0B\xCAa$\xB5V[`\0\x91\x82R` \x90\x91 `\x01`\x07\x90\x92\x02\x01\x01T`\x01`\x01`\xA0\x1B\x03\x16\x84a\x12\xD0V[a\x0C&`\0\x8A\x81T\x81\x10a\x0C\x03Wa\x0C\x03a$\xB5V[`\0\x91\x82R` \x90\x91 `\x02`\x07\x90\x92\x02\x01\x01T`\x01`\x01`\xA0\x1B\x03\x16\x83a\x12\xD0V[`@\x80Q\x8A\x81R` \x81\x01\x85\x90R\x90\x81\x01\x83\x90R``\x81\x01\x82\x90R3\x90\x7F\x95\x97W\x7F3\x93 w^c\xD3\xFE\xD7\xD5\xDD\xE66[\xAD\xCC\x9F\xCC\xDBf\xB3H\x94c\x0C\xA9\x8Bo\x90`\x80\x01[`@Q\x80\x91\x03\x90\xA2`\x01\x80U\x91\x98\x90\x97P\x90\x95P\x93PPPPV[`\0\x80`\0\x83\x81T\x81\x10a\x0C\x9AWa\x0C\x9Aa$\xB5V[`\0\x91\x82R` \x82 `\x07\x91\x90\x91\x02\x01`\x06\x01T`@Qcp\xA0\x821`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x87\x81\x16`\x04\x83\x01R\x90\x91\x16\x92P\x82\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\r1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a({\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\rEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\ri\x91\x90a&\xCAV[\x90P`\0\x82`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\r\xE4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a({\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\r\xF8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\x1C\x91\x90a&\xCAV[\x90P`\0\x80\x86\x81T\x81\x10a\x0E2Wa\x0E2a$\xB5V[\x90`\0R` `\0 \x90`\x07\x02\x01`\x05\x01T\x90Pa\x0Eca\x0E\\\x83\x83a\x16\xD1\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x84\x90a\x16\xEDV[\x94PPPPP[\x92\x91PPV[`\0\x80`\0`\x01T`\x02\x03a\x0E\x98W`@Qc\x03\xCB\x96\xDB`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x01U`\0\x80\x80a\x0E\xAD\x89\x82\x8A\x8Aa\x13\xD9V[\x92P\x92P\x92Pa\x0E\xED`\0\x8A\x81T\x81\x10a\x0E\xC9Wa\x0E\xC9a$\xB5V[`\0\x91\x82R` \x90\x91 `\x01`\x07\x90\x92\x02\x01\x01T`\x01`\x01`\xA0\x1B\x03\x163\x85a\x17\x02V[a\x0F'`\0\x8A\x81T\x81\x10a\x0F\x03Wa\x0F\x03a$\xB5V[`\0\x91\x82R` \x90\x91 `\x02`\x07\x90\x92\x02\x01\x01T`\x01`\x01`\xA0\x1B\x03\x163\x84a\x17\x02V[`@\x80Q\x8A\x81R` \x81\x01\x85\x90R\x90\x81\x01\x83\x90R``\x81\x01\x82\x90R3\x90\x7F\xAC\xBE\x12~\x93\xA8\xA0\xB2x\xD8\xE0n' [=\xF9\xD1\xF3\x81$\x14\xBC\x89\x17\xC7t\xA87\x101n\x90`\x80\x01a\x0CiV[`\0\x81\x81T\x81\x10a\x0F~W`\0\x80\xFD[`\0\x91\x82R` \x90\x91 `\x07\x90\x91\x02\x01\x80T`\x01\x82\x01T`\x02\x83\x01T`\x03\x84\x01T`\x04\x85\x01T`\x05\x86\x01T`\x06\x90\x96\x01T`\x01`\x01`\xA0\x1B\x03\x95\x86\x16\x97P\x93\x85\x16\x95\x92\x85\x16\x94\x91\x93\x90\x92\x91\x16\x87V[`\0\x80`\x01T`\x02\x03a\x0F\xF3W`@Qc\x03\xCB\x96\xDB`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x01\x81\x90UP`\0\x80`\0\x80`\0\x80\x8A\x81T\x81\x10a\x10\x15Wa\x10\x15a$\xB5V[`\0\x91\x82R` \x90\x91 `\x07\x90\x91\x02\x01T`@Qc\r\x17\xA7\xC7`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90ch\xBD>8\x90a\x10Y\x903\x90\x8E\x90\x8E\x90\x8E\x90`\x04\x01a$\xCBV[`\xC0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x10\xAFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a({\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x10\xC3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\xE7\x91\x90a&\xE6V[\x95P\x95P\x95PP\x94P\x94P\x84a\x11\x04W`\0\x84\x12a\x06\x9A\x85a\x12$V[\x80`\0\x8B\x81T\x81\x10a\x11\x18Wa\x11\x18a$\xB5V[\x90`\0R` `\0 \x90`\x07\x02\x01`\x05\x01\x81\x90UP`\0\x80`\0a\x11=\x8D\x87\x87a\x17)V[`@\x80Q\x86\x15\x15\x81R` \x81\x01\x84\x90R\x90\x81\x01\x82\x90R\x94\x97P\x90\x95P\x93P\x8F\x923\x92P\x7FL}\xEF\x84\xE4++\xC0\xA5\xAA\xB2\"\x86\x8D\xD7\xA0\x92\xB53w\xA4\xB57\xAB\xCD\x944Zz\x85'\xED\x91P``\x01`@Q\x80\x91\x03\x90\xA3`\x01\x80U\x90\x9C\x90\x9BP\x99PPPPPPPPPPV[`\0\x80`\0\x80\x84\x81T\x81\x10a\x11\xBBWa\x11\xBBa$\xB5V[\x90`\0R` `\0 \x90`\x07\x02\x01`\x03\x01T`\0\x85\x81T\x81\x10a\x11\xE0Wa\x11\xE0a$\xB5V[\x90`\0R` `\0 \x90`\x07\x02\x01`\x04\x01T`\0\x86\x81T\x81\x10a\x12\x05Wa\x12\x05a$\xB5V[\x90`\0R` `\0 \x90`\x07\x02\x01`\x05\x01T\x92P\x92P\x92P\x91\x93\x90\x92PV[`\0`\x01`\xFF\x1B\x82\x03a\x12JW`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x12\x15a\x12[WP\x19`\x01\x01\x90V[P\x80[\x91\x90PV[`\0v=`-\x80`\n=9\x81\xF36==7===6=s\0\0\0\x82``\x1B`\xE8\x1C\x17`\0RnZ\xF4=\x82\x80>\x90=\x91`+W\xFD[\xF3\x82`x\x1B\x17` R`7`\t`\0\xF0\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16a\x12^W`@Qc0\xBE\x1A=`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x12\xE4\x82a\x12\xDF\x85a\x1C\x82V[a\x1DmV[\x90Pa\x12\xF2\x8330\x84a\x1DyV[PPPV[`\0\x80\x82\x81T\x81\x10a\x13\x0BWa\x13\x0Ba$\xB5V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q`\xE0\x81\x01\x82R`\x07\x93\x90\x93\x02\x90\x91\x01\x80T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x80\x85R`\x01\x83\x01T\x82\x16\x85\x87\x01\x81\x90R`\x02\x84\x01T\x83\x16\x86\x86\x01\x81\x90R`\x03\x85\x01T``\x80\x89\x01\x82\x90R`\x04\x87\x01T`\x80\x80\x8B\x01\x82\x90R`\x05\x89\x01T`\xA0\x80\x8D\x01\x82\x90R`\x06\x90\x9A\x01T\x90\x98\x16`\xC0\x8C\x01R\x89Q\x96\x87R\x9A\x86\x01\x8D\x90R\x85\x89\x01\x92\x90\x92R\x84\x01R\x96\x82\x01\x92\x90\x92R\x92Q\x93\x95P\x923\x92\x7F\xF7\xC0\x95\xAF\xDDB\r*7\x8A\x88r\x05\xCFW\xEB\xEE/m\xD0\x07\xDD\xD7c\xC8Z\xEA\xC0Z\xE3\xD7\xCA\x92\x90\x82\x90\x03\x01\x90\xA4PPV[`\0\x80`\0\x80`\0\x80`\0\x80`\0\x8C\x81T\x81\x10a\x13\xF8Wa\x13\xF8a$\xB5V[\x90`\0R` `\0 \x90`\x07\x02\x01`\0\x01`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x8A\x04\xBD\xD53\x8E\x8D\x8D`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x14N\x94\x93\x92\x91\x90a$\xCBV[`\xA0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x14\xA4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a({\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x14\xB8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14\xDC\x91\x90a&WV[\x94P\x94P\x94P\x94P\x94P\x84a\x14\xF8W`\0\x84\x12a\x06\x9A\x85a\x12$V[\x8Aa\x152W\x82`\0\x8D\x81T\x81\x10a\x15\x11Wa\x15\x11a$\xB5V[\x90`\0R` `\0 \x90`\x07\x02\x01`\x03\x01Ta\x15-\x91\x90a&\xB7V[a\x15bV[`\0\x8C\x81T\x81\x10a\x15EWa\x15Ea$\xB5V[\x90`\0R` `\0 \x90`\x07\x02\x01`\x03\x01T\x83a\x15b\x91\x90a&\xB7V[\x97P\x8Aa\x15\x9EW\x81`\0\x8D\x81T\x81\x10a\x15}Wa\x15}a$\xB5V[\x90`\0R` `\0 \x90`\x07\x02\x01`\x04\x01Ta\x15\x99\x91\x90a&\xB7V[a\x15\xCEV[`\0\x8C\x81T\x81\x10a\x15\xB1Wa\x15\xB1a$\xB5V[\x90`\0R` `\0 \x90`\x07\x02\x01`\x04\x01T\x82a\x15\xCE\x91\x90a&\xB7V[\x96P\x8Aa\x16\nW\x80`\0\x8D\x81T\x81\x10a\x15\xE9Wa\x15\xE9a$\xB5V[\x90`\0R` `\0 \x90`\x07\x02\x01`\x05\x01Ta\x16\x05\x91\x90a&\xB7V[a\x16:V[`\0\x8C\x81T\x81\x10a\x16\x1DWa\x16\x1Da$\xB5V[\x90`\0R` `\0 \x90`\x07\x02\x01`\x05\x01T\x81a\x16:\x91\x90a&\xB7V[\x95Pa\x16G\x8C\x8C\x88a\x1E\x07V[\x82`\0\x8D\x81T\x81\x10a\x16[Wa\x16[a$\xB5V[\x90`\0R` `\0 \x90`\x07\x02\x01`\x03\x01\x81\x90UP\x81`\0\x8D\x81T\x81\x10a\x16\x84Wa\x16\x84a$\xB5V[\x90`\0R` `\0 \x90`\x07\x02\x01`\x04\x01\x81\x90UP\x80`\0\x8D\x81T\x81\x10a\x16\xADWa\x16\xADa$\xB5V[\x90`\0R` `\0 \x90`\x07\x02\x01`\x05\x01\x81\x90UPPPPPP\x94P\x94P\x94\x91PPV[`\0a\x16\xE6\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a \xA1V[\x93\x92PPPV[`\0a\x16\xE6\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a \xA1V[`\0a\x17\x16\x82a\x17\x11\x86a\x1C\x82V[a \xC0V[\x90Pa\x17#\x84\x84\x83a \xCCV[PPPPV[`\0\x80`\0\x80`\0\x80`\0\x89\x81T\x81\x10a\x17EWa\x17Ea$\xB5V[\x90`\0R` `\0 \x90`\x07\x02\x01`\x03\x01T\x90P`\0\x80\x8A\x81T\x81\x10a\x17mWa\x17ma$\xB5V[\x90`\0R` `\0 \x90`\x07\x02\x01`\x04\x01T\x90P\x81\x89\x11\x96P\x86\x15a\x18-W\x80\x88\x10a\x17\xACW`@Qc\x11\x15vg`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x8A\x81T\x81\x10a\x17\xBFWa\x17\xBFa$\xB5V[`\0\x91\x82R` \x82 `\x01`\x07\x90\x92\x02\x01\x01T\x81T`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x97P\x8B\x90\x81\x10a\x17\xF1Wa\x17\xF1a$\xB5V[`\0\x91\x82R` \x90\x91 `\x02`\x07\x90\x92\x02\x01\x01T`\x01`\x01`\xA0\x1B\x03\x16\x94Pa\x18\x1A\x82\x8Aa&\xB7V[\x93Pa\x18&\x88\x82a&\xB7V[\x92Pa\x18\xCAV[\x81\x89\x10a\x18MW`@Qc\x11\x15vg`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x8A\x81T\x81\x10a\x18`Wa\x18`a$\xB5V[`\0\x91\x82R` \x82 `\x02`\x07\x90\x92\x02\x01\x01T\x81T`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x97P\x8B\x90\x81\x10a\x18\x92Wa\x18\x92a$\xB5V[`\0\x91\x82R` \x90\x91 `\x01`\x07\x90\x92\x02\x01\x01T`\x01`\x01`\xA0\x1B\x03\x16\x94Pa\x18\xBB\x81\x89a&\xB7V[\x93Pa\x18\xC7\x89\x83a&\xB7V[\x92P[\x88`\0\x8B\x81T\x81\x10a\x18\xDEWa\x18\xDEa$\xB5V[\x90`\0R` `\0 \x90`\x07\x02\x01`\x03\x01\x81\x90UP\x87`\0\x8B\x81T\x81\x10a\x19\x07Wa\x19\x07a$\xB5V[`\0\x91\x82R` \x82 `\x04`\x07\x90\x92\x02\x01\x81\x01\x92\x90\x92U`@Qcp\xA0\x821`\xE0\x1B\x81R0\x92\x81\x01\x92\x90\x92R\x90`\x01`\x01`\xA0\x1B\x03\x88\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x19\x9EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a({\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x19\xB2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19\xD6\x91\x90a&\xCAV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x88\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x1AYW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a({\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x1AmW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A\x91\x91\x90a&\xCAV[\x90Pa\x1A\x9D\x88\x87a\x12\xD0V[a\x1A\xA8\x873\x87a\x17\x02V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x90`\x01`\x01`\xA0\x1B\x03\x8A\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x1B(W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a({\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x1B<W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B`\x91\x90a&\xCAV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x8A\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x1B\xE3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a({\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x1B\xF7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C\x1B\x91\x90a&\xCAV[\x90Pa\x1C'\x88\x85a':V[\x82\x10\x15a\x1CGW`@Qc =\x90\x1D`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1CQ\x87\x84a&\xB7V[\x81\x10\x15a\x1CqW`@Qc\xF3\xCB\xBC\x87`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPP\x93\x97\x92\x96P\x93P\x93PV[`\0\x80\x82`\x01`\x01`\xA0\x1B\x03\x16c1<\xE5g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x1C\xFCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a({\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x1D\x10W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1D4\x91\x90a'MV[`\xFF\x16\x90P`\0a\x1DF\x82`\x12a&\xB7V[\x90Pa\x1DS\x81`\na(WV[a\x1De\x90g\r\xE0\xB6\xB3\xA7d\0\0a(cV[\x94\x93PPPPV[`\0a\x16\xE6\x83\x83a!JV[`\0`@Qc#\xB8r\xDD`\xE0\x1B`\0R\x84`\x04R\x83`$R\x82`DR` `\0`d`\0\x80\x8AZ\xF1=\x15`\x1F=\x11`\x01`\0Q\x14\x16\x17\x16\x91P`\0``R\x80`@RP\x80a\x1E\0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rs\x15\x14\x90S\x94\xD1\x91T\x97\xD1\x94\x93\xD3W\xD1\x90RS\x11Q`b\x1B`D\x82\x01R`d\x01a\x06\xB8V[PPPPPV[`\0\x80\x84\x81T\x81\x10a\x1E\x1BWa\x1E\x1Ba$\xB5V[\x90`\0R` `\0 \x90`\x07\x02\x01`\x06\x01`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x90P`\0\x81`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x1E\xBBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a({\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x1E\xCFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E\xF3\x91\x90a&\xCAV[\x90P`\0\x80\x86\x81T\x81\x10a\x1F\tWa\x1F\ta$\xB5V[\x90`\0R` `\0 \x90`\x07\x02\x01`\x05\x01T\x90P\x84\x15a\x1F\xE0W`\0a\x1F9a\x1F2\x84\x84a\x16\xD1V[\x86\x90a\x16\xEDV[`@Qc@\xC1\x0F\x19`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x82\x90R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x85\x16\x90c@\xC1\x0F\x19\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1F\xC2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a({\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x1F\xD6W=`\0\x80>=`\0\xFD[PPPPPa \x99V[`\0a\x1F\xF6a\x1F\xEF\x84\x84a!JV[\x86\x90a!_V[`@Qc'p\xA7\xEB`\xE2\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x82\x90R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x85\x16\x90c\x9D\xC2\x9F\xAC\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a \x7FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a({\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a \x93W=`\0\x80>=`\0\xFD[PPPPP[PPPPPPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a \xB9W`\0\x80\xFD[\x04\x92\x91PPV[`\0a\x16\xE6\x83\x83a\x16\xD1V[`\0`@Qc\xA9\x05\x9C\xBB`\xE0\x1B`\0R\x83`\x04R\x82`$R` `\0`D`\0\x80\x89Z\xF1=\x15`\x1F=\x11`\x01`\0Q\x14\x16\x17\x16\x91P`\0``R\x80`@RP\x80a\x17#W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rn\x15\x14\x90S\x94\xD1\x91T\x97\xD1\x90RS\x11Q`\x8A\x1B`D\x82\x01R`d\x01a\x06\xB8V[`\0a\x16\xE6\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a!pV[`\0a\x16\xE6\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a!\x88W`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rjrray stride`\xA8\x1B`d\x82\x01R`\x84\x81\xFD[`\0\x80`\0`@\x84\x86\x03\x12\x15a\"\xAFWa\"\xAFa!\x9EV[\x835\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\"\xD1Wa\"\xD1a!\xEEV[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a#9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x81\xFD[\x815\x81\x81\x11\x15a#\x9CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x0ENL/$\r\x8C\xAD\xCC\xEE\x8D`\xAB\x1B`d\x82\x01R`\x84\x81\xFD[\x87` \x82\x85\x01\x01\x11\x15a#\xB1Wa#\xB1a\">V[` \x83\x01\x94P\x80\x93PPPP\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a#\xD9Wa#\xD9a!\x9EV[P5\x91\x90PV[`\0` \x82\x84\x03\x12\x15a#\xF5Wa#\xF5a!\x9EV[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a$\x0FWa$\x0Fa!\xEEV[\x82\x01`\x80\x81\x85\x03\x12\x15a\x16\xE6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FABI decoding: struct calldata to`D\x82\x01Rf\x1B\xC8\x1C\xDA\x1B\xDC\x9D`\xCA\x1B`d\x82\x01R`\x84\x81\xFD[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x12^W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a$\x9EWa$\x9Ea!\x9EV[a$\xA7\x83a$qV[\x94` \x93\x90\x93\x015\x93PPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\x01`\x01`\xA0\x1B\x03\x85\x16\x81R` \x81\x01\x84\x90R```@\x82\x01\x81\x90R\x81\x01\x82\x90R\x81\x83`\x80\x83\x017`\0\x81\x83\x01`\x80\x90\x81\x01\x91\x90\x91R`\x1F\x90\x92\x01`\x1F\x19\x16\x01\x01\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a%(Wa%(a!\x9EV[a\x16\xE6\x82a$qV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FCalldata tail too short\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x81\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a%\xCDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FInvalid calldata tail offset\0\0\0\0`D\x82\x01R`d\x81\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a&(W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FInvalid calldata tail length\0\0\0\0`D\x82\x01R`d\x81\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a&@Wa&@a%1V[\x92P\x92\x90PV[\x80Q\x80\x15\x15\x81\x14a\x12^W`\0\x80\xFD[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a&rWa&ra!\x9EV[a&{\x86a&GV[` \x87\x01Q`@\x88\x01Q``\x89\x01Q`\x80\x90\x99\x01Q\x92\x9A\x91\x99P\x97\x96P\x90\x94P\x92PPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x0EjWa\x0Eja&\xA1V[`\0` \x82\x84\x03\x12\x15a&\xDFWa&\xDFa!\x9EV[PQ\x91\x90PV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a'\x02Wa'\x02a!\x9EV[a'\x0B\x87a&GV[\x95P` \x87\x01Q\x94P`@\x87\x01Q\x93P``\x87\x01Q\x92P`\x80\x87\x01Q\x91P`\xA0\x87\x01Q\x90P\x92\x95P\x92\x95P\x92\x95V[\x80\x82\x01\x80\x82\x11\x15a\x0EjWa\x0Eja&\xA1V[`\0` \x82\x84\x03\x12\x15a'bWa'ba!\x9EV[\x81Q`\xFF\x81\x16\x81\x14a\x16\xE6W`\0\x80\xFD[`\x01\x81\x81[\x80\x85\x11\x15a'\xAEW\x81`\0\x19\x04\x82\x11\x15a'\x94Wa'\x94a&\xA1V[\x80\x85\x16\x15a'\xA1W\x91\x81\x02\x91[\x93\x84\x1C\x93\x90\x80\x02\x90a'xV[P\x92P\x92\x90PV[`\0\x82a'\xC5WP`\x01a\x0EjV[\x81a'\xD2WP`\0a\x0EjV[\x81`\x01\x81\x14a'\xE8W`\x02\x81\x14a'\xF2Wa(\x0EV[`\x01\x91PPa\x0EjV[`\xFF\x84\x11\x15a(\x03Wa(\x03a&\xA1V[PP`\x01\x82\x1Ba\x0EjV[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a(1WP\x81\x81\na\x0EjV[a(;\x83\x83a'sV[\x80`\0\x19\x04\x82\x11\x15a(OWa(Oa&\xA1V[\x02\x93\x92PPPV[`\0a\x16\xE6\x83\x83a'\xB6V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x0EjWa\x0Eja&\xA1V\xFETarget contract does not contain\xA2dipfsX\"\x12 \x82\xE7\x16\xCD\x0C\xB4 D\\vb%|\x925\xC6pBt[$\x0Ca\xAA\xC0U\xD7\xE0*\xF4\xBE.dsolcC\0\x08\x16\x003`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[Pa\x10\xAB\x80a\0m`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x01XW`\x005`\xE0\x1C\x80cL\xD8\x8Bv\x11a\0\xEFW\x80c\x9D\xC2\x9F\xAC\x11a\0\xBEW\x80c\x9D\xC2\x9F\xAC\x14a\x02\xCEW\x80c\xA9\x05\x9C\xBB\x14a\x02\xE1W\x80c\xAF\xBA\x13\xC4\x14a\x02\xF4W\x80c\xD5\x05\xAC\xCF\x14a\x03\x1FW\x80c\xDDb\xED>\x14a\x032Wa\x01XV[\x80cL\xD8\x8Bv\x14a\x02sW\x80cp\xA0\x821\x14a\x02\x86W\x80c~\xCE\xBE\0\x14a\x02\xA6W\x80c\x95\xD8\x9BA\x14a\x02\xC6Wa\x01XV[\x80c#\xB8r\xDD\x11a\x01+W\x80c#\xB8r\xDD\x14a\x02)W\x80c1<\xE5g\x14a\x02<W\x80c6D\xE5\x15\x14a\x02VW\x80c@\xC1\x0F\x19\x14a\x02^Wa\x01XV[\x80c\x06\xFD\xDE\x03\x14a\x01\xBDW\x80c\t^\xA7\xB3\x14a\x01\xDBW\x80c\x15\x8E\xF9>\x14a\x01\xFEW\x80c\x18\x16\r\xDD\x14a\x02\x12W[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x01\xC5a\x03]V[`@Qa\x01\xD2\x91\x90a\nSV[`@Q\x80\x91\x03\x90\xF3[a\x01\xEEa\x01\xE96`\x04a\x0B^V[a\x03\xEBV[`@Q\x90\x15\x15\x81R` \x01a\x01\xD2V[`\x08Ta\x01\xEE\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16\x81V[a\x02\x1B`\x02T\x81V[`@Q\x90\x81R` \x01a\x01\xD2V[a\x01\xEEa\x0276`\x04a\x0B\x8BV[a\x04XV[a\x02D`\x12\x81V[`@Q`\xFF\x90\x91\x16\x81R` \x01a\x01\xD2V[a\x02\x1Ba\x058V[a\x02qa\x02l6`\x04a\x0B^V[a\x05WV[\0[a\x02qa\x02\x816`\x04a\r\x1AV[a\x05\x90V[a\x02\x1Ba\x02\x946`\x04a\r\x87V[`\x03` R`\0\x90\x81R`@\x90 T\x81V[a\x02\x1Ba\x02\xB46`\x04a\r\x87V[`\x07` R`\0\x90\x81R`@\x90 T\x81V[a\x01\xC5a\x06\x0CV[a\x02qa\x02\xDC6`\x04a\x0B^V[a\x06\x19V[a\x01\xEEa\x02\xEF6`\x04a\x0B^V[a\x06NV[`\x08Ta\x03\x07\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\xD2V[a\x02qa\x03-6`\x04a\r\xACV[a\x06\xB4V[a\x02\x1Ba\x03@6`\x04a\x0E\"V[`\x04` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[`\0\x80Ta\x03j\x90a\x0EXV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x03\x96\x90a\x0EXV[\x80\x15a\x03\xE3W\x80`\x1F\x10a\x03\xB8Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x03\xE3V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x03\xC6W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[3`\0\x81\x81R`\x04` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x87\x16\x80\x85R\x92R\x80\x83 \x85\x90UQ\x91\x92\x90\x91\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x90a\x04F\x90\x86\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3P`\x01[\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x04` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x81 T`\0\x19\x81\x14a\x04\xB4Wa\x04\x8F\x83\x82a\x0E\xA8V[`\x01`\x01`\xA0\x1B\x03\x86\x16`\0\x90\x81R`\x04` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x90 U[`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R`\x03` R`@\x81 \x80T\x85\x92\x90a\x04\xDC\x90\x84\x90a\x0E\xA8V[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\0\x81\x81R`\x03` R`@\x90\x81\x90 \x80T\x87\x01\x90UQ\x90\x91\x87\x16\x90`\0\x80Q` a\x10V\x839\x81Q\x91R\x90a\x05%\x90\x87\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3P`\x01\x94\x93PPPPV[`\0`\x05TF\x14a\x05PWa\x05Ka\x08\xFDV[\x90P\x90V[P`\x06T\x90V[`\x08T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x05\x82W`@QchS\xCB\xA7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x05\x8C\x82\x82a\t\x97V[PPV[`\x08T`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15a\x05\xBAW`@Qb\xDC\x14\x9F`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x08\x80T`\x01`\x01`\xA0\x1B\x03\x19\x163\x17\x90U`\0a\x05\xD8\x83\x82a\x0F\x0CV[P`\x01a\x05\xE5\x82\x82a\x0F\x0CV[PF`\x05Ua\x05\xF2a\x08\xFDV[`\x06UPP`\x08\x80T`\xFF`\xA0\x1B\x19\x16`\x01`\xA0\x1B\x17\x90UV[`\x01\x80Ta\x03j\x90a\x0EXV[`\x08T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x06DW`@QchS\xCB\xA7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x05\x8C\x82\x82a\t\xF1V[3`\0\x90\x81R`\x03` R`@\x81 \x80T\x83\x91\x90\x83\x90a\x06o\x90\x84\x90a\x0E\xA8V[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x81\x81R`\x03` R`@\x90\x81\x90 \x80T\x85\x01\x90UQ3\x90`\0\x80Q` a\x10V\x839\x81Q\x91R\x90a\x04F\x90\x86\x81R` \x01\x90V[B\x84\x10\x15a\x07\tW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FPERMIT_DEADLINE_EXPIRED\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0`\x01a\x07\x15a\x058V[`\x01`\x01`\xA0\x1B\x03\x8A\x81\x16`\0\x81\x81R`\x07` \x90\x81R`@\x91\x82\x90 \x80T`\x01\x81\x01\x90\x91U\x82Q\x7Fnq\xED\xAE\x12\xB1\xB9\x7FM\x1F`7\x0F\xEF\x10\x10_\xA2\xFA\xAE\x01&\x11J\x16\x9Cd\x84]a&\xC9\x81\x84\x01R\x80\x84\x01\x94\x90\x94R\x93\x8D\x16``\x84\x01R`\x80\x83\x01\x8C\x90R`\xA0\x83\x01\x93\x90\x93R`\xC0\x80\x83\x01\x8B\x90R\x81Q\x80\x84\x03\x90\x91\x01\x81R`\xE0\x83\x01\x90\x91R\x80Q\x92\x01\x91\x90\x91 a\x19\x01`\xF0\x1Ba\x01\0\x83\x01Ra\x01\x02\x82\x01\x92\x90\x92Ra\x01\"\x81\x01\x91\x90\x91Ra\x01B\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 `\0\x84R\x90\x83\x01\x80\x83RR`\xFF\x87\x16\x90\x82\x01R``\x81\x01\x85\x90R`\x80\x81\x01\x84\x90R`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x08!W=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16\x15\x80\x15\x90a\x08WWP\x87`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14[a\x08\x94W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rm$\xA7+ \xA6$\xA2/\xA9\xA4\xA3\xA7\"\xA9`\x91\x1B`D\x82\x01R`d\x01a\x07\0V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\0\x90\x81R`\x04` \x90\x81R`@\x80\x83 \x8A\x85\x16\x80\x85R\x90\x83R\x92\x81\x90 \x89\x90UQ\x88\x81R\x91\x92\x8A\x16\x91\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x91\x01`@Q\x80\x91\x03\x90\xA3PPPPPPPV[`\0\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F`\0`@Qa\t/\x91\x90a\x0F\xCCV[`@\x80Q\x91\x82\x90\x03\x82 ` \x83\x01\x93\x90\x93R\x81\x01\x91\x90\x91R\x7F\xC8\x9E\xFD\xAAT\xC0\xF2\x0Cz\xDFa(\x82\xDF\tP\xF5\xA9Qc~\x03\x07\xCD\xCBLg/)\x8B\x8B\xC6``\x82\x01RF`\x80\x82\x01R0`\xA0\x82\x01R`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[\x80`\x02`\0\x82\x82Ta\t\xA9\x91\x90a\x10BV[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x81\x81R`\x03` \x90\x81R`@\x80\x83 \x80T\x86\x01\x90UQ\x84\x81R`\0\x80Q` a\x10V\x839\x81Q\x91R\x91\x01[`@Q\x80\x91\x03\x90\xA3PPV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x03` R`@\x81 \x80T\x83\x92\x90a\n\x19\x90\x84\x90a\x0E\xA8V[\x90\x91UPP`\x02\x80T\x82\x90\x03\x90U`@Q\x81\x81R`\0\x90`\x01`\x01`\xA0\x1B\x03\x84\x16\x90`\0\x80Q` a\x10V\x839\x81Q\x91R\x90` \x01a\t\xE5V[`\0` \x80\x83R\x83Q\x80` \x85\x01R`\0[\x81\x81\x10\x15a\n\x81W\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a\neV[P`\0`@\x82\x86\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x92PPP\x92\x91PPV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0BYW`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x0BtWa\x0Bta\n\xA2V[a\x0B}\x83a\x0BBV[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x0B\xA3Wa\x0B\xA3a\n\xA2V[a\x0B\xAC\x84a\x0BBV[\x92Pa\x0B\xBA` \x85\x01a\x0BBV[\x91P`@\x84\x015\x90P\x92P\x92P\x92V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x82\x81\x837P`\0\x91\x01RV[`\0\x82`\x1F\x83\x01\x12a\x0CQW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x81\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0ClWa\x0Cla\x0B\xCAV[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x0C\x94Wa\x0C\x94a\x0B\xCAV[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a\x0C\xFFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R\x92P`\x84\x83\xFD[a\r\x10\x84` \x83\x01` \x89\x01a\x0B\xE0V[\x96\x95PPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\r0Wa\r0a\n\xA2V[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\rKWa\rKa\n\xF2V[a\rW\x86\x83\x87\x01a\x0B\xECV[\x93P` \x85\x015\x91P\x80\x82\x11\x15a\rpWa\rpa\n\xF2V[Pa\r}\x85\x82\x86\x01a\x0B\xECV[\x91PP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\r\x9CWa\r\x9Ca\n\xA2V[a\r\xA5\x82a\x0BBV[\x93\x92PPPV[`\0\x80`\0\x80`\0\x80`\0`\xE0\x88\x8A\x03\x12\x15a\r\xCAWa\r\xCAa\n\xA2V[a\r\xD3\x88a\x0BBV[\x96Pa\r\xE1` \x89\x01a\x0BBV[\x95P`@\x88\x015\x94P``\x88\x015\x93P`\x80\x88\x015`\xFF\x81\x16\x81\x14a\x0E\x05W`\0\x80\xFD[\x96\x99\x95\x98P\x93\x96\x92\x95\x94`\xA0\x84\x015\x94P`\xC0\x90\x93\x015\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a\x0E8Wa\x0E8a\n\xA2V[a\x0EA\x83a\x0BBV[\x91Pa\x0EO` \x84\x01a\x0BBV[\x90P\x92P\x92\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x0ElW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x0E\x8CWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x04RWa\x04Ra\x0E\x92V[`\x1F\x82\x11\x15a\x0F\x07W`\0\x81`\0R` `\0 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a\x0E\xE4WP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x0F\x03W\x82\x81U`\x01\x01a\x0E\xF0V[PPP[PPPV[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0F&Wa\x0F&a\x0B\xCAV[a\x0F:\x81a\x0F4\x84Ta\x0EXV[\x84a\x0E\xBBV[` \x80`\x1F\x83\x11`\x01\x81\x14a\x0FoW`\0\x84\x15a\x0FWWP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ua\x0F\x03V[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15a\x0F\x9EW\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01a\x0F\x7FV[P\x85\x82\x10\x15a\x0F\xBCW\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[`\0\x80\x83Ta\x0F\xDA\x81a\x0EXV[`\x01\x82\x81\x16\x80\x15a\x0F\xF2W`\x01\x81\x14a\x10\x07Wa\x106V[`\xFF\x19\x84\x16\x87R\x82\x15\x15\x83\x02\x87\x01\x94Pa\x106V[\x87`\0R` \x80`\0 `\0[\x85\x81\x10\x15a\x10-W\x81T\x8A\x82\x01R\x90\x84\x01\x90\x82\x01a\x10\x14V[PPP\x82\x87\x01\x94P[P\x92\x96\x95PPPPPPV[\x80\x82\x01\x80\x82\x11\x15a\x04RWa\x04Ra\x0E\x92V\xFE\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\xA2dipfsX\"\x12 \0\xCCz\xAF(h\xA1\xB5\xB4$L\xADk4\xA8\xF5\xDB\xFF\x04\xD5\xAD\xCEt\xE7\xB40\xCB7\x7F\xA8%\xB9dsolcC\0\x08\x16\x003";
    /// The bytecode of the contract.
    pub static DFMM_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\0\xF6W`\x005`\xE0\x1C\x80c\x9D\x94/\x9A\x11a\0\xBEW\x80c\x9D\x94/\x9A\x14a\x02oW\x80c\xACJ\xFA8\x14a\x02\x82W\x80c\xAF\xFE\xD0\xE0\x14a\x02\xDAW\x80c\xB4b\xCD%\x14a\x02\xE2W\x80c\xBD\x06%\xAB\x14a\x03!W\x80c\xCE\x15;\xF4\x14a\x03IWa\0\xF6V[\x80c\x02\x16\xB88\x14a\x01[W\x80c\x06\x8B\xCD\x8D\x14a\x01pW\x80c\x14U\xF1\xFC\x14a\x01\xEDW\x80c.\xC3\x81\x88\x14a\x02 W\x80c;\xE6\xA3A\x14a\x02NW[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x01na\x01i6`\x04a\"\x97V[a\x03\\V[\0[a\x01\x83a\x01~6`\x04a#\xC4V[a\x04UV[`@Qa\x01\xE4\x91\x90\x81Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R` \x80\x84\x01Q\x82\x16\x90\x83\x01R`@\x80\x84\x01Q\x82\x16\x90\x83\x01R``\x80\x84\x01Q\x90\x83\x01R`\x80\x80\x84\x01Q\x90\x83\x01R`\xA0\x83\x81\x01Q\x90\x83\x01R`\xC0\x92\x83\x01Q\x16\x91\x81\x01\x91\x90\x91R`\xE0\x01\x90V[`@Q\x80\x91\x03\x90\xF3[a\x02\0a\x01\xFB6`\x04a#\xE0V[a\x05\x19V[`@\x80Q\x94\x85R` \x85\x01\x93\x90\x93R\x91\x83\x01R``\x82\x01R`\x80\x01a\x01\xE4V[a\x023a\x02.6`\x04a\"\x97V[a\x0BlV[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x01\xE4V[a\x02aa\x02\\6`\x04a$\x88V[a\x0C\x84V[`@Q\x90\x81R` \x01a\x01\xE4V[a\x023a\x02}6`\x04a\"\x97V[a\x0EpV[a\x02\x95a\x02\x906`\x04a#\xC4V[a\x0FnV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x98\x89\x16\x81R\x96\x88\x16` \x88\x01R\x94\x87\x16\x94\x86\x01\x94\x90\x94R``\x85\x01\x92\x90\x92R`\x80\x84\x01R`\xA0\x83\x01R\x90\x91\x16`\xC0\x82\x01R`\xE0\x01a\x01\xE4V[`\0Ta\x02aV[a\x03\t\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\xE4V[a\x034a\x03/6`\x04a\"\x97V[a\x0F\xCDV[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x01\xE4V[a\x023a\x03W6`\x04a#\xC4V[a\x11\xA4V[`\x01T`\x02\x03a\x03\x7FW`@Qc\x03\xCB\x96\xDB`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x01U`\0\x80T\x84\x90\x81\x10a\x03\x98Wa\x03\x98a$\xB5V[`\0\x91\x82R` \x90\x91 `\x07\x90\x91\x02\x01T`@Qc\xAC\xAD)\x89`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xAC\xAD)\x89\x90a\x03\xDC\x903\x90\x87\x90\x87\x90\x87\x90`\x04\x01a$\xCBV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x044W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a({\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x04HW=`\0\x80>=`\0\xFD[PP`\x01\x80UPPPPPV[`@\x80Q`\xE0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R`\xC0\x81\x01\x91\x90\x91R`\0\x82\x81T\x81\x10a\x04\xA1Wa\x04\xA1a$\xB5V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q`\xE0\x81\x01\x82R`\x07\x90\x93\x02\x90\x91\x01\x80T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x84R`\x01\x82\x01T\x81\x16\x94\x84\x01\x94\x90\x94R`\x02\x81\x01T\x84\x16\x91\x83\x01\x91\x90\x91R`\x03\x81\x01T``\x83\x01R`\x04\x81\x01T`\x80\x83\x01R`\x05\x81\x01T`\xA0\x83\x01R`\x06\x01T\x90\x91\x16`\xC0\x82\x01R\x92\x91PPV[`\0\x80`\0\x80`\x01T`\x02\x03a\x05BW`@Qc\x03\xCB\x96\xDB`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x01Ua\x05W``\x86\x01`@\x87\x01a%\x13V[`\x01`\x01`\xA0\x1B\x03\x16a\x05p`@\x87\x01` \x88\x01a%\x13V[`\x01`\x01`\xA0\x1B\x03\x16\x03a\x05\x97W`@Qc3\x91\n\xEF`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80\x80\x80\x80a\x05\xAA` \x8B\x01\x8Ba%\x13V[`\x01`\x01`\xA0\x1B\x03\x16cs\xCB-\x033`\0\x80T\x90P\x8D\x80``\x01\x90a\x05\xCF\x91\x90a%vV[`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x05\xEE\x94\x93\x92\x91\x90a$\xCBV[`\xA0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x06FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a({\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x06ZW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06~\x91\x90a&WV[\x94P\x94P\x94P\x94P\x94P\x84a\x06\xC1W`\0\x84\x12a\x06\x9A\x85a\x12$V[`@Qcw`m)`\xE1\x1B\x81R\x91\x15\x15`\x04\x83\x01R`$\x82\x01R`D\x01[`@Q\x80\x91\x03\x90\xFD[`\0a\x06\xEC\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x12cV[`@\x80Qc&lE\xBB`\xE1\x1B\x81R`\x04\x81\x01\x91\x90\x91R`\0`D\x82\x01\x81\x90R```$\x83\x01R`d\x82\x01R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x82\x16\x90cL\xD8\x8Bv\x90`\x84\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x07\x86W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a({\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x07\x9AW=`\0\x80>=`\0\xFD[PPPP\x80`\x01`\x01`\xA0\x1B\x03\x16c@\xC1\x0F\x193a\x03\xE8\x85a\x07\xBC\x91\x90a&\xB7V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x04\x83\x01R`$\x82\x01R`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x08@W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a({\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x08TW=`\0\x80>=`\0\xFD[PP`@Qc@\xC1\x0F\x19`\xE0\x1B\x81R`\0`\x04\x82\x01Ra\x03\xE8`$\x82\x01R`\x01`\x01`\xA0\x1B\x03\x84\x16\x92Pc@\xC1\x0F\x19\x91P`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x08\xE0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a({\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x08\xF4W=`\0\x80>=`\0\xFD[PP`@\x80Q`\xE0\x81\x01\x90\x91R`\0\x92P\x90P\x80a\t\x15` \x8F\x01\x8Fa%\x13V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8D` \x01` \x81\x01\x90a\t6\x91\x90a%\x13V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8D`@\x01` \x81\x01\x90a\tW\x91\x90a%\x13V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R` \x80\x83\x01\x89\x90R`@\x80\x84\x01\x89\x90R``\x80\x85\x01\x89\x90R\x87\x84\x16`\x80\x95\x86\x01R`\0\x80T`\x01\x81\x81\x01\x83U\x82\x80R\x88Q`\x07\x90\x92\x02\x7F)\r\xEC\xD9T\x8Bb\xA8\xD6\x03E\xA9\x888o\xC8K\xA6\xBC\x95H@\x08\xF66/\x93\x16\x0E\xF3\xE5c\x81\x01\x80T\x93\x89\x16`\x01`\x01`\xA0\x1B\x03\x19\x94\x85\x16\x17\x90U\x95\x89\x01Q\x7F)\r\xEC\xD9T\x8Bb\xA8\xD6\x03E\xA9\x888o\xC8K\xA6\xBC\x95H@\x08\xF66/\x93\x16\x0E\xF3\xE5d\x87\x01\x80T\x91\x89\x16\x91\x84\x16\x91\x90\x91\x17\x90U\x93\x88\x01Q\x7F)\r\xEC\xD9T\x8Bb\xA8\xD6\x03E\xA9\x888o\xC8K\xA6\xBC\x95H@\x08\xF66/\x93\x16\x0E\xF3\xE5e\x86\x01\x80T\x91\x88\x16\x91\x83\x16\x91\x90\x91\x17\x90U\x91\x87\x01Q\x7F)\r\xEC\xD9T\x8Bb\xA8\xD6\x03E\xA9\x888o\xC8K\xA6\xBC\x95H@\x08\xF66/\x93\x16\x0E\xF3\xE5f\x85\x01U\x94\x86\x01Q\x7F)\r\xEC\xD9T\x8Bb\xA8\xD6\x03E\xA9\x888o\xC8K\xA6\xBC\x95H@\x08\xF66/\x93\x16\x0E\xF3\xE5g\x84\x01U`\xA0\x86\x01Q\x7F)\r\xEC\xD9T\x8Bb\xA8\xD6\x03E\xA9\x888o\xC8K\xA6\xBC\x95H@\x08\xF66/\x93\x16\x0E\xF3\xE5h\x84\x01U`\xC0\x86\x01Q\x7F)\r\xEC\xD9T\x8Bb\xA8\xD6\x03E\xA9\x888o\xC8K\xA6\xBC\x95H@\x08\xF66/\x93\x16\x0E\xF3\xE5i\x90\x93\x01\x80T\x93\x90\x94\x16\x92\x16\x91\x90\x91\x17\x90\x91U\x81T\x92\x93P\x90\x91a\n\xFD\x91\x90a&\xB7V[\x90Pa\x0B\x1B\x8D` \x01` \x81\x01\x90a\x0B\x15\x91\x90a%\x13V[\x87a\x12\xD0V[a\x0B7\x8D`@\x01` \x81\x01\x90a\x0B1\x91\x90a%\x13V[\x86a\x12\xD0V[a\x0B@\x81a\x12\xF7V[\x80\x86\x86a\x0BOa\x03\xE8\x88a&\xB7V[\x9BP\x9BP\x9BP\x9BPPPPPPPPP`\x01\x80U\x92\x94\x91\x93P\x91\x90V[`\0\x80`\0`\x01T`\x02\x03a\x0B\x94W`@Qc\x03\xCB\x96\xDB`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x01\x81\x90UP`\0\x80`\0a\x0B\xAE\x89`\x01\x8A\x8Aa\x13\xD9V[\x92P\x92P\x92Pa\x0B\xED`\0\x8A\x81T\x81\x10a\x0B\xCAWa\x0B\xCAa$\xB5V[`\0\x91\x82R` \x90\x91 `\x01`\x07\x90\x92\x02\x01\x01T`\x01`\x01`\xA0\x1B\x03\x16\x84a\x12\xD0V[a\x0C&`\0\x8A\x81T\x81\x10a\x0C\x03Wa\x0C\x03a$\xB5V[`\0\x91\x82R` \x90\x91 `\x02`\x07\x90\x92\x02\x01\x01T`\x01`\x01`\xA0\x1B\x03\x16\x83a\x12\xD0V[`@\x80Q\x8A\x81R` \x81\x01\x85\x90R\x90\x81\x01\x83\x90R``\x81\x01\x82\x90R3\x90\x7F\x95\x97W\x7F3\x93 w^c\xD3\xFE\xD7\xD5\xDD\xE66[\xAD\xCC\x9F\xCC\xDBf\xB3H\x94c\x0C\xA9\x8Bo\x90`\x80\x01[`@Q\x80\x91\x03\x90\xA2`\x01\x80U\x91\x98\x90\x97P\x90\x95P\x93PPPPV[`\0\x80`\0\x83\x81T\x81\x10a\x0C\x9AWa\x0C\x9Aa$\xB5V[`\0\x91\x82R` \x82 `\x07\x91\x90\x91\x02\x01`\x06\x01T`@Qcp\xA0\x821`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x87\x81\x16`\x04\x83\x01R\x90\x91\x16\x92P\x82\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\r1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a({\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\rEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\ri\x91\x90a&\xCAV[\x90P`\0\x82`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\r\xE4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a({\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\r\xF8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\x1C\x91\x90a&\xCAV[\x90P`\0\x80\x86\x81T\x81\x10a\x0E2Wa\x0E2a$\xB5V[\x90`\0R` `\0 \x90`\x07\x02\x01`\x05\x01T\x90Pa\x0Eca\x0E\\\x83\x83a\x16\xD1\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x84\x90a\x16\xEDV[\x94PPPPP[\x92\x91PPV[`\0\x80`\0`\x01T`\x02\x03a\x0E\x98W`@Qc\x03\xCB\x96\xDB`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x01U`\0\x80\x80a\x0E\xAD\x89\x82\x8A\x8Aa\x13\xD9V[\x92P\x92P\x92Pa\x0E\xED`\0\x8A\x81T\x81\x10a\x0E\xC9Wa\x0E\xC9a$\xB5V[`\0\x91\x82R` \x90\x91 `\x01`\x07\x90\x92\x02\x01\x01T`\x01`\x01`\xA0\x1B\x03\x163\x85a\x17\x02V[a\x0F'`\0\x8A\x81T\x81\x10a\x0F\x03Wa\x0F\x03a$\xB5V[`\0\x91\x82R` \x90\x91 `\x02`\x07\x90\x92\x02\x01\x01T`\x01`\x01`\xA0\x1B\x03\x163\x84a\x17\x02V[`@\x80Q\x8A\x81R` \x81\x01\x85\x90R\x90\x81\x01\x83\x90R``\x81\x01\x82\x90R3\x90\x7F\xAC\xBE\x12~\x93\xA8\xA0\xB2x\xD8\xE0n' [=\xF9\xD1\xF3\x81$\x14\xBC\x89\x17\xC7t\xA87\x101n\x90`\x80\x01a\x0CiV[`\0\x81\x81T\x81\x10a\x0F~W`\0\x80\xFD[`\0\x91\x82R` \x90\x91 `\x07\x90\x91\x02\x01\x80T`\x01\x82\x01T`\x02\x83\x01T`\x03\x84\x01T`\x04\x85\x01T`\x05\x86\x01T`\x06\x90\x96\x01T`\x01`\x01`\xA0\x1B\x03\x95\x86\x16\x97P\x93\x85\x16\x95\x92\x85\x16\x94\x91\x93\x90\x92\x91\x16\x87V[`\0\x80`\x01T`\x02\x03a\x0F\xF3W`@Qc\x03\xCB\x96\xDB`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x01\x81\x90UP`\0\x80`\0\x80`\0\x80\x8A\x81T\x81\x10a\x10\x15Wa\x10\x15a$\xB5V[`\0\x91\x82R` \x90\x91 `\x07\x90\x91\x02\x01T`@Qc\r\x17\xA7\xC7`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90ch\xBD>8\x90a\x10Y\x903\x90\x8E\x90\x8E\x90\x8E\x90`\x04\x01a$\xCBV[`\xC0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x10\xAFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a({\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x10\xC3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\xE7\x91\x90a&\xE6V[\x95P\x95P\x95PP\x94P\x94P\x84a\x11\x04W`\0\x84\x12a\x06\x9A\x85a\x12$V[\x80`\0\x8B\x81T\x81\x10a\x11\x18Wa\x11\x18a$\xB5V[\x90`\0R` `\0 \x90`\x07\x02\x01`\x05\x01\x81\x90UP`\0\x80`\0a\x11=\x8D\x87\x87a\x17)V[`@\x80Q\x86\x15\x15\x81R` \x81\x01\x84\x90R\x90\x81\x01\x82\x90R\x94\x97P\x90\x95P\x93P\x8F\x923\x92P\x7FL}\xEF\x84\xE4++\xC0\xA5\xAA\xB2\"\x86\x8D\xD7\xA0\x92\xB53w\xA4\xB57\xAB\xCD\x944Zz\x85'\xED\x91P``\x01`@Q\x80\x91\x03\x90\xA3`\x01\x80U\x90\x9C\x90\x9BP\x99PPPPPPPPPPV[`\0\x80`\0\x80\x84\x81T\x81\x10a\x11\xBBWa\x11\xBBa$\xB5V[\x90`\0R` `\0 \x90`\x07\x02\x01`\x03\x01T`\0\x85\x81T\x81\x10a\x11\xE0Wa\x11\xE0a$\xB5V[\x90`\0R` `\0 \x90`\x07\x02\x01`\x04\x01T`\0\x86\x81T\x81\x10a\x12\x05Wa\x12\x05a$\xB5V[\x90`\0R` `\0 \x90`\x07\x02\x01`\x05\x01T\x92P\x92P\x92P\x91\x93\x90\x92PV[`\0`\x01`\xFF\x1B\x82\x03a\x12JW`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x12\x15a\x12[WP\x19`\x01\x01\x90V[P\x80[\x91\x90PV[`\0v=`-\x80`\n=9\x81\xF36==7===6=s\0\0\0\x82``\x1B`\xE8\x1C\x17`\0RnZ\xF4=\x82\x80>\x90=\x91`+W\xFD[\xF3\x82`x\x1B\x17` R`7`\t`\0\xF0\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16a\x12^W`@Qc0\xBE\x1A=`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x12\xE4\x82a\x12\xDF\x85a\x1C\x82V[a\x1DmV[\x90Pa\x12\xF2\x8330\x84a\x1DyV[PPPV[`\0\x80\x82\x81T\x81\x10a\x13\x0BWa\x13\x0Ba$\xB5V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q`\xE0\x81\x01\x82R`\x07\x93\x90\x93\x02\x90\x91\x01\x80T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x80\x85R`\x01\x83\x01T\x82\x16\x85\x87\x01\x81\x90R`\x02\x84\x01T\x83\x16\x86\x86\x01\x81\x90R`\x03\x85\x01T``\x80\x89\x01\x82\x90R`\x04\x87\x01T`\x80\x80\x8B\x01\x82\x90R`\x05\x89\x01T`\xA0\x80\x8D\x01\x82\x90R`\x06\x90\x9A\x01T\x90\x98\x16`\xC0\x8C\x01R\x89Q\x96\x87R\x9A\x86\x01\x8D\x90R\x85\x89\x01\x92\x90\x92R\x84\x01R\x96\x82\x01\x92\x90\x92R\x92Q\x93\x95P\x923\x92\x7F\xF7\xC0\x95\xAF\xDDB\r*7\x8A\x88r\x05\xCFW\xEB\xEE/m\xD0\x07\xDD\xD7c\xC8Z\xEA\xC0Z\xE3\xD7\xCA\x92\x90\x82\x90\x03\x01\x90\xA4PPV[`\0\x80`\0\x80`\0\x80`\0\x80`\0\x8C\x81T\x81\x10a\x13\xF8Wa\x13\xF8a$\xB5V[\x90`\0R` `\0 \x90`\x07\x02\x01`\0\x01`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x8A\x04\xBD\xD53\x8E\x8D\x8D`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x14N\x94\x93\x92\x91\x90a$\xCBV[`\xA0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x14\xA4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a({\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x14\xB8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14\xDC\x91\x90a&WV[\x94P\x94P\x94P\x94P\x94P\x84a\x14\xF8W`\0\x84\x12a\x06\x9A\x85a\x12$V[\x8Aa\x152W\x82`\0\x8D\x81T\x81\x10a\x15\x11Wa\x15\x11a$\xB5V[\x90`\0R` `\0 \x90`\x07\x02\x01`\x03\x01Ta\x15-\x91\x90a&\xB7V[a\x15bV[`\0\x8C\x81T\x81\x10a\x15EWa\x15Ea$\xB5V[\x90`\0R` `\0 \x90`\x07\x02\x01`\x03\x01T\x83a\x15b\x91\x90a&\xB7V[\x97P\x8Aa\x15\x9EW\x81`\0\x8D\x81T\x81\x10a\x15}Wa\x15}a$\xB5V[\x90`\0R` `\0 \x90`\x07\x02\x01`\x04\x01Ta\x15\x99\x91\x90a&\xB7V[a\x15\xCEV[`\0\x8C\x81T\x81\x10a\x15\xB1Wa\x15\xB1a$\xB5V[\x90`\0R` `\0 \x90`\x07\x02\x01`\x04\x01T\x82a\x15\xCE\x91\x90a&\xB7V[\x96P\x8Aa\x16\nW\x80`\0\x8D\x81T\x81\x10a\x15\xE9Wa\x15\xE9a$\xB5V[\x90`\0R` `\0 \x90`\x07\x02\x01`\x05\x01Ta\x16\x05\x91\x90a&\xB7V[a\x16:V[`\0\x8C\x81T\x81\x10a\x16\x1DWa\x16\x1Da$\xB5V[\x90`\0R` `\0 \x90`\x07\x02\x01`\x05\x01T\x81a\x16:\x91\x90a&\xB7V[\x95Pa\x16G\x8C\x8C\x88a\x1E\x07V[\x82`\0\x8D\x81T\x81\x10a\x16[Wa\x16[a$\xB5V[\x90`\0R` `\0 \x90`\x07\x02\x01`\x03\x01\x81\x90UP\x81`\0\x8D\x81T\x81\x10a\x16\x84Wa\x16\x84a$\xB5V[\x90`\0R` `\0 \x90`\x07\x02\x01`\x04\x01\x81\x90UP\x80`\0\x8D\x81T\x81\x10a\x16\xADWa\x16\xADa$\xB5V[\x90`\0R` `\0 \x90`\x07\x02\x01`\x05\x01\x81\x90UPPPPPP\x94P\x94P\x94\x91PPV[`\0a\x16\xE6\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a \xA1V[\x93\x92PPPV[`\0a\x16\xE6\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a \xA1V[`\0a\x17\x16\x82a\x17\x11\x86a\x1C\x82V[a \xC0V[\x90Pa\x17#\x84\x84\x83a \xCCV[PPPPV[`\0\x80`\0\x80`\0\x80`\0\x89\x81T\x81\x10a\x17EWa\x17Ea$\xB5V[\x90`\0R` `\0 \x90`\x07\x02\x01`\x03\x01T\x90P`\0\x80\x8A\x81T\x81\x10a\x17mWa\x17ma$\xB5V[\x90`\0R` `\0 \x90`\x07\x02\x01`\x04\x01T\x90P\x81\x89\x11\x96P\x86\x15a\x18-W\x80\x88\x10a\x17\xACW`@Qc\x11\x15vg`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x8A\x81T\x81\x10a\x17\xBFWa\x17\xBFa$\xB5V[`\0\x91\x82R` \x82 `\x01`\x07\x90\x92\x02\x01\x01T\x81T`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x97P\x8B\x90\x81\x10a\x17\xF1Wa\x17\xF1a$\xB5V[`\0\x91\x82R` \x90\x91 `\x02`\x07\x90\x92\x02\x01\x01T`\x01`\x01`\xA0\x1B\x03\x16\x94Pa\x18\x1A\x82\x8Aa&\xB7V[\x93Pa\x18&\x88\x82a&\xB7V[\x92Pa\x18\xCAV[\x81\x89\x10a\x18MW`@Qc\x11\x15vg`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x8A\x81T\x81\x10a\x18`Wa\x18`a$\xB5V[`\0\x91\x82R` \x82 `\x02`\x07\x90\x92\x02\x01\x01T\x81T`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x97P\x8B\x90\x81\x10a\x18\x92Wa\x18\x92a$\xB5V[`\0\x91\x82R` \x90\x91 `\x01`\x07\x90\x92\x02\x01\x01T`\x01`\x01`\xA0\x1B\x03\x16\x94Pa\x18\xBB\x81\x89a&\xB7V[\x93Pa\x18\xC7\x89\x83a&\xB7V[\x92P[\x88`\0\x8B\x81T\x81\x10a\x18\xDEWa\x18\xDEa$\xB5V[\x90`\0R` `\0 \x90`\x07\x02\x01`\x03\x01\x81\x90UP\x87`\0\x8B\x81T\x81\x10a\x19\x07Wa\x19\x07a$\xB5V[`\0\x91\x82R` \x82 `\x04`\x07\x90\x92\x02\x01\x81\x01\x92\x90\x92U`@Qcp\xA0\x821`\xE0\x1B\x81R0\x92\x81\x01\x92\x90\x92R\x90`\x01`\x01`\xA0\x1B\x03\x88\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x19\x9EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a({\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x19\xB2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19\xD6\x91\x90a&\xCAV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x88\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x1AYW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a({\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x1AmW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A\x91\x91\x90a&\xCAV[\x90Pa\x1A\x9D\x88\x87a\x12\xD0V[a\x1A\xA8\x873\x87a\x17\x02V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x90`\x01`\x01`\xA0\x1B\x03\x8A\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x1B(W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a({\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x1B<W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B`\x91\x90a&\xCAV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x8A\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x1B\xE3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a({\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x1B\xF7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C\x1B\x91\x90a&\xCAV[\x90Pa\x1C'\x88\x85a':V[\x82\x10\x15a\x1CGW`@Qc =\x90\x1D`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1CQ\x87\x84a&\xB7V[\x81\x10\x15a\x1CqW`@Qc\xF3\xCB\xBC\x87`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPP\x93\x97\x92\x96P\x93P\x93PV[`\0\x80\x82`\x01`\x01`\xA0\x1B\x03\x16c1<\xE5g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x1C\xFCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a({\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x1D\x10W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1D4\x91\x90a'MV[`\xFF\x16\x90P`\0a\x1DF\x82`\x12a&\xB7V[\x90Pa\x1DS\x81`\na(WV[a\x1De\x90g\r\xE0\xB6\xB3\xA7d\0\0a(cV[\x94\x93PPPPV[`\0a\x16\xE6\x83\x83a!JV[`\0`@Qc#\xB8r\xDD`\xE0\x1B`\0R\x84`\x04R\x83`$R\x82`DR` `\0`d`\0\x80\x8AZ\xF1=\x15`\x1F=\x11`\x01`\0Q\x14\x16\x17\x16\x91P`\0``R\x80`@RP\x80a\x1E\0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rs\x15\x14\x90S\x94\xD1\x91T\x97\xD1\x94\x93\xD3W\xD1\x90RS\x11Q`b\x1B`D\x82\x01R`d\x01a\x06\xB8V[PPPPPV[`\0\x80\x84\x81T\x81\x10a\x1E\x1BWa\x1E\x1Ba$\xB5V[\x90`\0R` `\0 \x90`\x07\x02\x01`\x06\x01`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x90P`\0\x81`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x1E\xBBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a({\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x1E\xCFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E\xF3\x91\x90a&\xCAV[\x90P`\0\x80\x86\x81T\x81\x10a\x1F\tWa\x1F\ta$\xB5V[\x90`\0R` `\0 \x90`\x07\x02\x01`\x05\x01T\x90P\x84\x15a\x1F\xE0W`\0a\x1F9a\x1F2\x84\x84a\x16\xD1V[\x86\x90a\x16\xEDV[`@Qc@\xC1\x0F\x19`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x82\x90R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x85\x16\x90c@\xC1\x0F\x19\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1F\xC2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a({\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x1F\xD6W=`\0\x80>=`\0\xFD[PPPPPa \x99V[`\0a\x1F\xF6a\x1F\xEF\x84\x84a!JV[\x86\x90a!_V[`@Qc'p\xA7\xEB`\xE2\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x82\x90R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x85\x16\x90c\x9D\xC2\x9F\xAC\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a \x7FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a({\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a \x93W=`\0\x80>=`\0\xFD[PPPPP[PPPPPPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a \xB9W`\0\x80\xFD[\x04\x92\x91PPV[`\0a\x16\xE6\x83\x83a\x16\xD1V[`\0`@Qc\xA9\x05\x9C\xBB`\xE0\x1B`\0R\x83`\x04R\x82`$R` `\0`D`\0\x80\x89Z\xF1=\x15`\x1F=\x11`\x01`\0Q\x14\x16\x17\x16\x91P`\0``R\x80`@RP\x80a\x17#W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rn\x15\x14\x90S\x94\xD1\x91T\x97\xD1\x90RS\x11Q`\x8A\x1B`D\x82\x01R`d\x01a\x06\xB8V[`\0a\x16\xE6\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a!pV[`\0a\x16\xE6\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a!\x88W`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rjrray stride`\xA8\x1B`d\x82\x01R`\x84\x81\xFD[`\0\x80`\0`@\x84\x86\x03\x12\x15a\"\xAFWa\"\xAFa!\x9EV[\x835\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\"\xD1Wa\"\xD1a!\xEEV[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a#9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x81\xFD[\x815\x81\x81\x11\x15a#\x9CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x0ENL/$\r\x8C\xAD\xCC\xEE\x8D`\xAB\x1B`d\x82\x01R`\x84\x81\xFD[\x87` \x82\x85\x01\x01\x11\x15a#\xB1Wa#\xB1a\">V[` \x83\x01\x94P\x80\x93PPPP\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a#\xD9Wa#\xD9a!\x9EV[P5\x91\x90PV[`\0` \x82\x84\x03\x12\x15a#\xF5Wa#\xF5a!\x9EV[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a$\x0FWa$\x0Fa!\xEEV[\x82\x01`\x80\x81\x85\x03\x12\x15a\x16\xE6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FABI decoding: struct calldata to`D\x82\x01Rf\x1B\xC8\x1C\xDA\x1B\xDC\x9D`\xCA\x1B`d\x82\x01R`\x84\x81\xFD[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x12^W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a$\x9EWa$\x9Ea!\x9EV[a$\xA7\x83a$qV[\x94` \x93\x90\x93\x015\x93PPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\x01`\x01`\xA0\x1B\x03\x85\x16\x81R` \x81\x01\x84\x90R```@\x82\x01\x81\x90R\x81\x01\x82\x90R\x81\x83`\x80\x83\x017`\0\x81\x83\x01`\x80\x90\x81\x01\x91\x90\x91R`\x1F\x90\x92\x01`\x1F\x19\x16\x01\x01\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a%(Wa%(a!\x9EV[a\x16\xE6\x82a$qV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FCalldata tail too short\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x81\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a%\xCDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FInvalid calldata tail offset\0\0\0\0`D\x82\x01R`d\x81\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a&(W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FInvalid calldata tail length\0\0\0\0`D\x82\x01R`d\x81\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a&@Wa&@a%1V[\x92P\x92\x90PV[\x80Q\x80\x15\x15\x81\x14a\x12^W`\0\x80\xFD[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a&rWa&ra!\x9EV[a&{\x86a&GV[` \x87\x01Q`@\x88\x01Q``\x89\x01Q`\x80\x90\x99\x01Q\x92\x9A\x91\x99P\x97\x96P\x90\x94P\x92PPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x0EjWa\x0Eja&\xA1V[`\0` \x82\x84\x03\x12\x15a&\xDFWa&\xDFa!\x9EV[PQ\x91\x90PV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a'\x02Wa'\x02a!\x9EV[a'\x0B\x87a&GV[\x95P` \x87\x01Q\x94P`@\x87\x01Q\x93P``\x87\x01Q\x92P`\x80\x87\x01Q\x91P`\xA0\x87\x01Q\x90P\x92\x95P\x92\x95P\x92\x95V[\x80\x82\x01\x80\x82\x11\x15a\x0EjWa\x0Eja&\xA1V[`\0` \x82\x84\x03\x12\x15a'bWa'ba!\x9EV[\x81Q`\xFF\x81\x16\x81\x14a\x16\xE6W`\0\x80\xFD[`\x01\x81\x81[\x80\x85\x11\x15a'\xAEW\x81`\0\x19\x04\x82\x11\x15a'\x94Wa'\x94a&\xA1V[\x80\x85\x16\x15a'\xA1W\x91\x81\x02\x91[\x93\x84\x1C\x93\x90\x80\x02\x90a'xV[P\x92P\x92\x90PV[`\0\x82a'\xC5WP`\x01a\x0EjV[\x81a'\xD2WP`\0a\x0EjV[\x81`\x01\x81\x14a'\xE8W`\x02\x81\x14a'\xF2Wa(\x0EV[`\x01\x91PPa\x0EjV[`\xFF\x84\x11\x15a(\x03Wa(\x03a&\xA1V[PP`\x01\x82\x1Ba\x0EjV[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a(1WP\x81\x81\na\x0EjV[a(;\x83\x83a'sV[\x80`\0\x19\x04\x82\x11\x15a(OWa(Oa&\xA1V[\x02\x93\x92PPPV[`\0a\x16\xE6\x83\x83a'\xB6V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x0EjWa\x0Eja&\xA1V\xFETarget contract does not contain\xA2dipfsX\"\x12 \x82\xE7\x16\xCD\x0C\xB4 D\\vb%|\x925\xC6pBt[$\x0Ca\xAA\xC0U\xD7\xE0*\xF4\xBE.dsolcC\0\x08\x16\x003";
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
        abi = "Init(address,address,address,address,uint256,uint256,uint256,uint256)"
    )]
    pub struct InitFilter {
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        pub strategy: ::ethers::core::types::Address,
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
