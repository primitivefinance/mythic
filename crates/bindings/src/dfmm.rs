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
                    ::std::borrow::ToOwned::to_owned("multicall"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("multicall"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes[]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
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
                                    name: ::std::borrow::ToOwned::to_owned("controller"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
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
                    ::std::borrow::ToOwned::to_owned("updateController"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("updateController"),
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
                                    name: ::std::borrow::ToOwned::to_owned("newController"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("LogPoolStats"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("LogPoolStats"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("rx"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("ry"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("L"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("invariant"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sigma"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("strike"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("tau"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("timestamp"),
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
                    ::std::borrow::ToOwned::to_owned("NotController"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("NotController"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NotInitialized"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("NotInitialized"),
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
    const __BYTECODE: &[u8] = b"`\xA0`@\x90\x80\x82R4b\0\x01[WP`\x01\x80U\x80Qa\x11g\x80\x82\x01\x91`\x01`\x01`@\x1B\x03\x91\x82\x84\x11\x82\x85\x10\x17b\0\x01EWb\0\"\xCD\x829\x80`\0\x93\x03\x90\x83\xF0\x80\x15b\0\x01;W`\x01`\x01`\xA0\x1B\x03\x16`\x80\x81\x90R\x80;\x15b\0\0\xE9W\x90\x82\x80\x92`\x84\x86Q\x80\x96\x81\x93c&lE\xBB`\xE1\x1B\x83R\x89`\x04\x84\x01R\x81`D\x84\x01R```$\x84\x01R\x81`d\x84\x01RZ\xF1\x80\x15b\0\0\xDFWb\0\0\xB8W[\x83Qa!$\x90\x81b\0\x01\xA9\x829`\x80Q\x81\x81\x81a\x0F\xA2\x01Ra\x13\x88\x01R\xF3[\x82\x11b\0\0\xCBWP\x81R8\x80\x80b\0\0\x99V[cNH{q`\xE0\x1B\x81R`A`\x04R`$\x90\xFD[\x84Q=\x84\x82>=\x90\xFD[\x83QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01Rd code`\xD8\x1B`d\x82\x01R`\x84\x90\xFD[\x83Q=\x84\x82>=\x90\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[bF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01Ra7\xB7`\xF1\x1B`d\x82\x01R`\x84\x90\xFD\xFE`\x80`@R`\x046\x10a\x11{W`\x005`\xE0\x1C\x80c\x02\x16\xB88\x14a\0\xDCW\x80c\x06\x8B\xCD\x8D\x14a\0\xD7W\x80c\x14U\xF1\xFC\x14a\0\xD2W\x80c.\xC3\x81\x88\x14a\0\xCDW\x80c;\xE6\xA3A\x14a\0\xC8W\x80c\x9D\x94/\x9A\x14a\0\xC3W\x80c\x9F\xE1\xC1n\x14a\0\xBEW\x80c\xACJ\xFA8\x14a\0\xB9W\x80c\xAC\x96P\xD8\x14a\0\xB4W\x80c\xAF\xFE\xD0\xE0\x14a\0\xAFW\x80c\xB4b\xCD%\x14a\0\xAAW\x80c\xBD\x06%\xAB\x14a\0\xA5Wc\xCE\x15;\xF4\x03a\x11{Wa\x11\x1DV[a\x0F\xD1V[a\x0F\x8CV[a\x0FnV[a\x0E#V[a\x0CGV[a\x0BxV[a\t\xC5V[a\x08\x98V[a\x06cV[a\x05\xACV[a\x04@V[a\x03=V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01Ra7\xB7`\xF1\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rjrray stride`\xA8\x1B`d\x82\x01R`\x84\x90\xFD[`@`\x03\x19\x82\x01\x12a\x038W`\x045\x91`$5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x83\x82\x11a\x033W\x80`#\x83\x01\x12\x15a\x03.W\x81`\x04\x015\x93\x84\x11a\x02\xD5W`$\x84\x83\x01\x01\x11a\x02\xD0W`$\x01\x91\x90V[a\x02*V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x0ENL/$\r\x8C\xAD\xCC\xEE\x8D`\xAB\x1B`d\x82\x01R`\x84\x90\xFD[a\x01\xD1V[a\x01\x81V[a\x011V[4a\x04;Wa\x03K6a\x02\x83V[`\x02`\x01\x93\x92\x93T\x14a\x04)W`\x02`\x01Ua\x03\x86a\x03za\x03l\x84a\x0C\tV[PT`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x90V[3\x03a\x04\x17Wa\x03\xAFa\x03za\x03z`\x01a\x03\xA0\x86a\x0C\tV[P\x01T`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x80;\x15a\x04\x12Wa\x03\xDA\x93`\0\x80\x94`@Q\x96\x87\x95\x86\x94\x85\x93bB\xD7\x07`\xE3\x1B\x85R`\x04\x85\x01a\x17\xB8V[\x03\x92Z\xF1\x80\x15a\x04\rWa\x03\xF4W[a\x03\xF2`\x01\x80UV[\0[\x80a\x04\x01a\x04\x07\x92a\x0C\xF5V[\x80a\x0FcV[8a\x03\xE9V[a\x17\xE6V[a\x17*V[`@Qc#\x01\x9Eg`\xE0\x1B\x81R`\x04\x90\xFD[`@Qc\x03\xCB\x96\xDB`\xE2\x1B\x81R`\x04\x90\xFD[a\0\xE1V[4a\x04;W` 6`\x03\x19\x01\x12a\x038W`@Qa\x04]\x81a\r\x0EV[`\xE0`\0\x91\x82\x81R\x82` \x82\x01R\x82`@\x82\x01R\x82``\x82\x01R\x82`\x80\x82\x01R\x82`\xA0\x82\x01R\x82`\xC0\x82\x01R\x01Ra\x05\xA8a\x04\x99`\x045a\x0C\tV[Pa\x05=a\x05-`\x07a\x04\xAAa\rMV[\x84T`\x01`\x01`\xA0\x1B\x03\x16\x81R\x93`\x01\x81\x01T`\x01`\x01`\xA0\x1B\x03\x16` \x86\x01R`\x02\x81\x01T`\x01`\x01`\xA0\x1B\x03\x16`@\x86\x01R`\x03\x81\x01Ta\x05\x01\x90`\x01`\x01`\xA0\x1B\x03\x16[`\x01`\x01`\xA0\x1B\x03\x16``\x87\x01RV[`\x04\x81\x01T`\x80\x86\x01R`\x05\x81\x01T`\xA0\x86\x01R`\x06\x81\x01T`\xC0\x86\x01R\x01T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16`\xE0\x83\x01RV[`@Q\x91\x82\x91\x82\x91\x90\x91`\xE0a\x01\0\x82\x01\x93\x81`\x01\x80`\xA0\x1B\x03\x91\x82\x81Q\x16\x85R\x82` \x82\x01Q\x16` \x86\x01R\x82`@\x82\x01Q\x16`@\x86\x01R\x82``\x82\x01Q\x16``\x86\x01R`\x80\x81\x01Q`\x80\x86\x01R`\xA0\x81\x01Q`\xA0\x86\x01R`\xC0\x81\x01Q`\xC0\x86\x01R\x01Q\x16\x91\x01RV[\x03\x90\xF3[4a\x04;W`\x03\x19` 6\x82\x01\x12a\x038W`\x045\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x033W`\x80\x90\x826\x03\x01\x12a\x06\x0EWa\x05\xECa\x05\xA8\x91`\x04\x01a\x12\xCEV[`@\x80Q\x94\x85R` \x85\x01\x93\x90\x93R\x91\x83\x01R``\x82\x01R\x90\x81\x90`\x80\x82\x01\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FABI decoding: struct calldata to`D\x82\x01Rf\x1B\xC8\x1C\xDA\x1B\xDC\x9D`\xCA\x1B`d\x82\x01R`\x84\x90\xFD[4a\x04;Wa\x06q6a\x02\x83V[\x91\x90`\x02`\x01T\x14a\x04)W`\x02`\x01Ua\x06\x96a\x03za\x03z`\x01a\x03\xA0\x86a\x0C\tV[\x92\x83;\x15a\x04\x12W`@Qb#\x8Bu`\xEA\x1B\x81R\x93`\xA0\x92\x85\x92\x83\x91\x82\x91a\x06\xC2\x91\x88`\x04\x85\x01a\x17\xB8V[\x03\x91Z\xFA\x90\x81\x15a\x04\rW`\0\x90\x81\x82\x80\x95\x81\x95a\x08FW[P\x15a\x08\x0CWP\x90\x81a\x06\xFE`\x04a\x06\xF5a\x05\xA8\x95a\x0C\tV[P\x01T\x83a\x18\xF2V[\x93a\x07\x16`\x05a\x07\r\x84a\x0C\tV[P\x01T\x87a\x18\xF2V[\x95a\x07%`\x06a\x06\xF5\x85a\x0C\tV[\x93a\x070\x85\x85a\x1E\x8DV[`\x04a\x07;\x85a\x0C\tV[P\x01U`\x05a\x07I\x84a\x0C\tV[P\x01U`\x06a\x07W\x83a\x0C\tV[P\x01Ua\x07\x9F\x85`\x02a\x07\x87\x87a\x07m\x86a\x0C\tV[P\x90\x92\x01T`\x01`\x01`\xA0\x1B\x03\x92\x900\x903\x90\x85\x16a\x18\xFFV[a\x07\x90\x84a\x0C\tV[P0\x91`\x033\x92\x01T\x16a\x18\xFFV[`@\x80Q\x91\x82R` \x82\x01\x85\x90R\x81\x01\x85\x90R``\x81\x01\x82\x90R3\x90\x7F\x95\x97W\x7F3\x93 w^c\xD3\xFE\xD7\xD5\xDD\xE66[\xAD\xCC\x9F\xCC\xDBf\xB3H\x94c\x0C\xA9\x8Bo\x90\x80`\x80\x81\x01[\x03\x90\xA2a\x07\xEF`\x01\x80UV[`@Q\x93\x84\x93\x84`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[\x80a\x08Ba\x08\x1B`\0\x93a\x19\x80V[`@Qcw`m)`\xE1\x1B\x81R\x93\x90\x92\x12`\x04\x84\x01R`$\x83\x01\x91\x90\x91R\x81\x90`D\x82\x01\x90V[\x03\x90\xFD[\x93PPP\x92Pa\x08m\x91P`\xA0=\x81\x11a\x08{W[a\x08e\x81\x83a\r+V[\x81\x01\x90a\x17\x8AV[\x94\x91\x90\x92\x90\x92\x94\x938a\x06\xDBV[P=a\x08[V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\x08\x93WV[`\0\x80\xFD[4a\x04;W`@6`\x03\x19\x01\x12a\x038W`\x045a\x08\xB5\x81a\x08\x82V[`$5\x90a\x08\xCDa\x03za\x03z`\x07a\x03\xA0\x86a\x0C\tV[\x90\x81;\x15a\x04\x12W`@Qcp\xA0\x821`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16`\x04\x82\x01R` \x80\x82`$\x81\x86Z\xFA\x91\x82\x15a\x04\rW`\0\x92a\t\xA6W[P\x82;\x15a\x04\x12W\x80`\x04\x93`@Q\x94\x85\x80\x92c\x18\x16\r\xDD`\xE0\x1B\x82RZ\xFA\x80\x15a\x04\rWa\x05\xA8\x94a\t`\x94a\tZ\x93`\0\x93a\tpW[PPa\tR`\x06\x91a\x0C\tV[P\x01Ta \x1DV[\x90a ?V[`@Q\x90\x81R\x90\x81\x90` \x82\x01\x90V[`\x06\x92\x93Pa\tR\x91\x81a\t\x98\x92\x90=\x10a\t\x9FW[a\t\x90\x81\x83a\r+V[\x81\x01\x90a\x1A\xA8V[\x92\x91a\tEV[P=a\t\x86V[\x81a\t\xBE\x92\x93P=\x84\x11a\t\x9FWa\t\x90\x81\x83a\r+V[\x908a\t\x0CV[4a\x04;Wa\t\xD36a\x02\x83V[`\x02`\x01T\x14a\x04)Wa\t\xEC\x91`\x02`\x01U\x83a\x1D\x9BV[a\n\x02a\x03za\x03z`\x02a\x03\xA0\x88\x96\x98a\x0C\tV[\x91\x82;\x15a\x04\x12W`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x80\x82R3`\x04\x83\x01R`$\x82\x01\x86\x90R\x93` \x91\x90\x82\x90\x82\x90`D\x90\x82\x90`\0\x90Z\xF1\x80\x15a\x04\rWa\x0B[W[Pa\nXa\x03za\x03z`\x03a\x03\xA0\x86a\x0C\tV[\x93\x84;\x15a\x04\x12W`@Q\x90\x81R3`\x04\x82\x01R`$\x81\x01\x87\x90R\x93\x81\x90\x85\x90`D\x90\x82\x90`\0\x90Z\xF1\x93\x84\x15a\x04\rWa\x05\xA8\x94a\x0B-W[PPa\n\xBF\x84a\n\xA1\x83a\x0C\tV[P`\x02\x01T3\x90a\n\xBA\x90`\x01`\x01`\xA0\x1B\x03\x16a\x03zV[a\x19\xC1V[a\n\xE5\x85a\n\xCC\x83a\x0C\tV[P`\x03\x01T3\x90a\n\xBA\x90`\x01`\x01`\xA0\x1B\x03\x16a\x03zV[`@\x80Q\x91\x82R` \x82\x01\x85\x90R\x81\x01\x85\x90R``\x81\x01\x82\x90R3\x90\x7F\xAC\xBE\x12~\x93\xA8\xA0\xB2x\xD8\xE0n' [=\xF9\xD1\xF3\x81$\x14\xBC\x89\x17\xC7t\xA87\x101n\x90\x80`\x80\x81\x01a\x07\xE3V[\x81a\x0BL\x92\x90=\x10a\x0BTW[a\x0BD\x81\x83a\r+V[\x81\x01\x90a\x19\xADV[P8\x80a\n\x92V[P=a\x0B:V[a\x0Bq\x90\x82=\x84\x11a\x0BTWa\x0BD\x81\x83a\r+V[P8a\nCV[4a\x04;W`@6`\x03\x19\x01\x12a\x038W`\x045`$5a\x0B\x98\x81a\x08\x82V[`\x02`\x01T\x14a\x04)W`\x02`\x01Ua\x0B\xB0\x82a\x0C\tV[PT`\x01`\x01`\xA0\x1B\x03\x163\x03a\x04\x17Wa\x0B\xCDa\x0B\xED\x92a\x0C\tV[P\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x01\x80U\0[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0T\x81\x10\x15a\x0CBW`\0\x80R`\x03\x1B\x7F)\r\xEC\xD9T\x8Bb\xA8\xD6\x03E\xA9\x888o\xC8K\xA6\xBC\x95H@\x08\xF66/\x93\x16\x0E\xF3\xE5c\x01\x90`\0\x90V[a\x0B\xF3V[4a\x04;W` 6`\x03\x19\x01\x12a\x038W`\x045`\0T\x81\x10\x15a\x08\x93Wa\x0Cn\x90a\x0C\tV[P\x80T`\x01\x82\x01T`\x02\x83\x01T`\x03\x84\x01T`\x04\x85\x01T`\x05\x86\x01T`\x06\x87\x01T`\x07\x90\x97\x01T`@\x80Q`\x01`\x01`\xA0\x1B\x03\x98\x89\x16\x81R\x96\x88\x16` \x88\x01R\x94\x87\x16\x94\x86\x01\x94\x90\x94R\x91\x85\x16``\x85\x01R`\x80\x84\x01R`\xA0\x83\x01R`\xC0\x82\x01\x93\x90\x93R\x91\x16`\xE0\x82\x01Ra\x01\0\x90\xF3[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\r\tW`@RV[a\x0C\xDFV[a\x01\0\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\r\tW`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\r\tW`@RV[`@Q\x90a\rZ\x82a\r\x0EV[V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\r\tW`\x05\x1B` \x01\x90V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\r\tW`\x1F\x01`\x1F\x19\x16` \x01\x90V[` \x80\x82\x01\x81\x83R\x83Q\x80\x91R`@\x83\x01\x91\x80`@\x83`\x05\x1B\x86\x01\x01\x95\x01\x93`\0\x80\x91[\x84\x83\x10a\r\xC5WPPPPPPP\x90V[\x90\x91\x92\x93\x94\x95\x96`?\x19\x82\x82\x03\x01\x87R\x84\x88Q\x80Q\x90\x81\x84R\x85[\x82\x81\x10a\x0E\x0FWPP\x80\x83\x01\x82\x01\x85\x90R`\x1F\x01`\x1F\x19\x16\x90\x91\x01\x81\x01\x97\x81\x01\x96\x01\x94\x93\x92`\x01\x01\x91\x90a\r\xB4V[\x81\x81\x01\x84\x01Q\x85\x82\x01\x85\x01R\x88\x93\x01a\r\xE0V[4a\x04;W` \x80`\x03\x196\x01\x12a\x038W`\x04\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x825\x82\x81\x11a\x033W6`#\x82\x01\x12\x15a\x03.W\x80\x84\x015\x92a\x0Ed\x84a\r\\V[\x93`@\x95a\x0Et\x87Q\x96\x87a\r+V[\x81\x86R\x84\x86\x01\x92`$\x80\x93`\x05\x1B\x86\x01\x01\x946\x86\x11a\x02\xD0W\x83\x81\x01\x94[\x86\x86\x10a\x0E\xB1Wa\x05\xA8\x8Aa\x0E\xA6\x8Ba\x12\"V[\x90Q\x91\x82\x91\x82a\r\x90V[\x855\x83\x81\x11a\x03.W\x82\x016`C\x82\x01\x12\x15a\x03.W\x85\x81\x015a\x0E\xD4\x81a\rtV[\x91a\x0E\xE1\x8DQ\x93\x84a\r+V[\x81\x83R`D\x906\x82\x84\x83\x01\x01\x11a\x0F\x11W\x8B\x83\x81\x96\x94\x82\x96\x94`\0\x94\x01\x83\x86\x017\x83\x01\x01R\x81R\x01\x95\x01\x94a\x0E\x92V[P\x86\x7FABI decoding: invalid byte array`\x84\x92\x8F\x8B\x8F`'\x92Q\x95bF\x1B\xCD`\xE5\x1B\x87R\x86\x01R\x84\x01R\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R\xFD[`\0\x91\x03\x12a\x038WV[4a\x04;W`\x006`\x03\x19\x01\x12a\x038W` `\0T`@Q\x90\x81R\xF3[4a\x04;W`\x006`\x03\x19\x01\x12a\x038W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x04;Wa\x0F\xDF6a\x02\x83V[\x91\x90`\x02`\x01T\x14a\x04)W`\x02`\x01Ua\x10\x04a\x03za\x03z`\x01a\x03\xA0\x86a\x0C\tV[\x92\x83;\x15a\x04\x12W`@Qc2\x14\x89\x0F`\xE0\x1B\x81R\x93`\xC0\x92\x85\x92\x83\x91\x82\x91a\x101\x91\x88`\x04\x85\x01a\x17\xB8V[\x03\x91Z\xFA\x91\x82\x15a\x04\rW`\0\x80\x93\x81\x80\x93\x81\x92a\x10\xE1W[P\x15a\x10\xD2W\x83\x94P`\x06a\x10aa\x10j\x95a\x0C\tV[P\x01U\x83a\x1A\xC4V[\x94\x92P\x92\x90P\x7FL}\xEF\x84\xE4++\xC0\xA5\xAA\xB2\"\x86\x8D\xD7\xA0\x92\xB53w\xA4\xB57\xAB\xCD\x944Zz\x85'\xED`@Q\x80a\x10\xB5\x87\x873\x96\x84`@\x91\x94\x93\x92``\x82\x01\x95\x15\x15\x82R` \x82\x01R\x01RV[\x03\x90\xA3a\x10\xC1`\x01\x80UV[`@\x80Q\x91\x82R` \x82\x01\x92\x90\x92R\xF3[`\0\x85a\x08Ba\x08\x1B\x82a\x19\x80V[\x93PPPPa\x11\x08\x91\x92P`\xC0=\x81\x11a\x11\x16W[a\x11\0\x81\x83a\r+V[\x81\x01\x90a\x1A8V[\x93\x95\x94\x90\x93\x91\x92P8a\x10JV[P=a\x10\xF6V[4a\x04;W` 6`\x03\x19\x01\x12a\x038W`\x045`\x04a\x11<\x82a\x0C\tV[P\x01Ta\x05\xA8`\x06a\x11[`\x05a\x11R\x86a\x0C\tV[P\x01T\x94a\x0C\tV[P\x01T`@Q\x93\x84\x93\x84`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01Rtnor receive functions`X\x1B`d\x82\x01R`\x84\x90\xFD[\x80Q\x82\x10\x15a\x0CBW` \x91`\x05\x1B\x01\x01\x90V[=\x15a\x12\x1DW=\x90a\x12\x03\x82a\rtV[\x91a\x12\x11`@Q\x93\x84a\r+V[\x82R=`\0` \x84\x01>V[``\x90V[\x90\x81Q\x91a\x12/\x83a\r\\V[\x92a\x12=`@Q\x94\x85a\r+V[\x80\x84Ra\x12L`\x1F\x19\x91a\r\\V[\x01`\0[\x81\x81\x10a\x12\xBDWPP`\0[\x81Q\x81\x03a\x12\xB9Wa\x12n\x81\x83a\x11\xDEV[Q`\0\x80\x82Q` \x80\x94\x010Z\xF4\x90a\x12\x85a\x11\xF2V[\x91\x15a\x12\xACWP\x90`\x01\x91a\x12\x9A\x82\x87a\x11\xDEV[Ra\x12\xA5\x81\x86a\x11\xDEV[P\x01a\x12\\V[\x90\x80Q\x91\x82\x15a\x08\x93W\x01\xFD[PPV[\x80``` \x80\x93\x88\x01\x01R\x01a\x12PV[`\x02`\x01T\x14a\x04)W`\x02`\x01U` \x81\x01\x90a\x12\xEB\x82a\x16\x16V[\x92`@\x92\x83\x83\x01a\x12\xFEa\x03z\x82a\x16\x16V[`\x01`\x01`\xA0\x1B\x03\x96\x87\x16\x14a\x16\x05Wa\x13\x1Da\x03za\x03z\x86a\x16\x16V[\x94`\0\x92\x83Ta\x130``\x88\x01\x88a\x16hV[\x90\x98\x80;\x15a\x04\x12Wa\x13]\x99\x87`\xA0\x94\x87Q\x9C\x8D\x95\x86\x94\x85\x93c\x9F\x83\x13{`\xE0\x1B\x85R`\x04\x85\x01a\x17\xB8V[\x03\x92Z\xF1\x95\x86\x15a\x04\rW\x84\x98\x85\x86\x9A\x87\x9A\x88\x9Aa\x15\xD4W[P\x15a\x15\x9FWPa\x13\xACa\x03za\x03z\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a \xBCV[\x95\x86;\x15a\x04\x12W\x84Qc&lE\xBB`\xE1\x1B\x81R`@`\x04\x82\x01R`\0`D\x82\x01\x81\x90R```$\x83\x01R`d\x82\x01R\x81\x81`\x84\x81\x83\x8CZ\xF1\x80\x15a\x04\rWa\x15\x8CW[P\x86;\x15a\x04\x12W\x84Qc@\xC1\x0F\x19`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x8A\x90R\x96\x81\x88`D\x81\x83\x85Z\xF1\x97\x88\x15a\x04\rWa\x15&\x85a\x15!\x8F\x9B\x8F\x90\x8F\x89\x98\x8F\x8F\x8F\x8F\x91\x9B\x7F\xF7\xC0\x95\xAF\xDDB\r*7\x8A\x88r\x05\xCFW\xEB\xEE/m\xD0\x07\xDD\xD7c\xC8Z\xEA\xC0Z\xE3\xD7\xCA\x9F\x95a\x14\xF2\x94a\x14\xCCa\x152\x9F\x94a\x15,\x9Fa\x14\xED\x96a\x14\x9Ba\x14\x95a\x14\x8Fa\x15\x0E\x9F\x9Ea\x14\xF8\x9F\x95a\x04\xF1\x96a\x15yW[Pa\x16\x16V[\x9Aa\x16\x16V[\x91a\x16\x16V[\x92a\x14\xBBa\x14\xA7a\rMV[3\x81R\x9A`\x01`\x01`\xA0\x1B\x03\x16` \x8C\x01RV[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90\x89\x01RV[`\x80\x85\x01R`\xA0\x84\x01\x89\x90R`\xC0\x84\x01R`\x01`\x01`\xA0\x1B\x03\x16`\xE0\x83\x01RV[a\x17\xF2V[Ta\x18\xDEV[\x9Da\x15\x05a\x03z\x8Da\x16\x16V[0\x903\x90a\x18\xFFV[03a\x15\x1Ca\x03z\x8Fa\x16\x16V[a\x18\xFFV[a\x16\x16V[\x95a\x16\x16V[\x96a\x16\x16V[\x94Q`\x01`\x01`\xA0\x1B\x03\x94\x90\x94\x16\x84R` \x84\x01\x87\x90R`@\x84\x01\x8B\x90R``\x84\x01\x8A\x90R`\x80\x84\x01\x89\x90R\x90\x93\x16\x93\x90\x92\x16\x913\x91`\xA0\x90\xA4\x93\x92\x91\x90a\rZ`\x01\x80UV[\x80a\x04\x01a\x15\x86\x92a\x0C\xF5V[8a\x14\x89V[\x80a\x04\x01a\x15\x99\x92a\x0C\xF5V[8a\x13\xF0V[\x80a\x08B\x86a\x15\xAE\x8A\x94a\x19\x80V[\x90Qcw`m)`\xE1\x1B\x81R\x93\x90\x92\x12`\x04\x84\x01R`$\x83\x01\x91\x90\x91R\x81\x90`D\x82\x01\x90V[\x93\x9BPPP\x96Pa\x15\xF4\x91\x97P`\xA0=\x81\x11a\x08{Wa\x08e\x81\x83a\r+V[\x91\x9A\x90\x99\x91\x98\x91\x93\x90\x92\x908a\x13vV[\x84Qc3\x91\n\xEF`\xE1\x1B\x81R`\x04\x90\xFD[5a\x16 \x81a\x08\x82V[\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FCalldata tail too short\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x90\xFD[\x905\x90`\x1E\x19\x816\x03\x01\x82\x12\x15a\x16\xE5W\x01\x805\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x16\xA0W` \x01\x91\x816\x03\x83\x13a\x16\x9BWV[a\x16#V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FInvalid calldata tail length\0\0\0\0`D\x82\x01R`d\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FInvalid calldata tail offset\0\0\0\0`D\x82\x01R`d\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01Rd code`\xD8\x1B`d\x82\x01R`\x84\x90\xFD[Q\x90\x81\x15\x15\x82\x03a\x08\x93WV[\x90\x81`\xA0\x91\x03\x12a\x038Wa\x17\x9E\x81a\x17}V[\x91` \x82\x01Q\x91`@\x81\x01Q\x91`\x80``\x83\x01Q\x92\x01Q\x90V[\x91\x92``\x93\x81\x92\x84R`@` \x85\x01R\x81`@\x85\x01R\x84\x84\x017`\0\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[`@Q=`\0\x82>=\x90\xFD[`\0Th\x01\0\0\0\0\0\0\0\0\x81\x10\x15a\r\tW\x80`\x01a\x18\x16\x92\x01`\0Ua\x0C\tV[a\x18\xB2W\x81Q\x81T`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x82\x16\x17\x83U` \x84\x01Q`\x01\x84\x01\x80T\x91\x84\x16\x91\x83\x16\x91\x90\x91\x17\x90U`@\x84\x01Q`\x02\x84\x01\x80T\x91\x84\x16\x91\x83\x16\x91\x90\x91\x17\x90U``\x84\x01Q`\x03\x84\x01\x80T\x91\x84\x16\x91\x83\x16\x91\x90\x91\x17\x90U`\x80\x84\x01Q`\x04\x84\x01U`\xA0\x84\x01Q`\x05\x84\x01U`\xC0\x84\x01Q`\x06\x84\x01U`\xE0\x90\x93\x01Q`\x07\x90\x92\x01\x80T\x90\x93\x16\x91\x16\x17\x90UV[cNH{q`\xE0\x1B`\0R`\0`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x19\x81\x01\x91\x90\x82\x11a\x18\xEDWV[a\x18\xC8V[\x91\x90\x82\x03\x91\x82\x11a\x18\xEDWV[\x91\x92`d` \x92\x94`@Q\x95`\0\x95\x86\x94\x85\x93\x84\x93c#\xB8r\xDD`\xE0\x1B\x85R`\x04R`$R`DRZ\xF1=\x15`\x1F=\x11`\x01\x84Q\x14\x16\x17\x16\x90``R\x81`@R\x15a\x19GWPV[bF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rs\x15\x14\x90S\x94\xD1\x91T\x97\xD1\x94\x93\xD3W\xD1\x90RS\x11Q`b\x1B`D\x82\x01R`d\x90\xFD[`\x01`\xFF\x1B\x81\x14a\x19\x9BW`\0\x81\x12\x15a\x16 W\x19`\x01\x01\x90V[`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x90\xFD[\x90\x81` \x91\x03\x12a\x038Wa\x16 \x90a\x17}V[\x90\x91` \x90`D`@Q\x94`\0\x80\x95\x81\x94\x82\x93c\xA9\x05\x9C\xBB`\xE0\x1B\x84R`\x04R`$RZ\xF1=\x15`\x1F=\x11`\x01\x84Q\x14\x16\x17\x16\x90``R\x81`@R\x15a\x1A\x04WPV[bF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rn\x15\x14\x90S\x94\xD1\x91T\x97\xD1\x90RS\x11Q`\x8A\x1B`D\x82\x01R`d\x90\xFD[\x91\x90\x82`\xC0\x91\x03\x12a\x038Wa\x1AM\x82a\x17}V[\x91` \x81\x01Q\x91`@\x82\x01Q\x91``\x81\x01Q\x91`\xA0`\x80\x83\x01Q\x92\x01Q\x90V[\x15a\x1AtWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01Rk\x06\x96\xE7f\x16\xC6\x96B\x077v\x17`\xA4\x1B`D\x82\x01R`d\x90\xFD[\x90\x81` \x91\x03\x12a\x038WQ\x90V[\x91\x90\x82\x01\x80\x92\x11a\x18\xEDWV[\x92\x90a\x1A\xCF\x84a\x0C\tV[P\x91`\x04\x80\x93\x01T\x93`\x05a\x1A\xE3\x87a\x0C\tV[P\x01T\x93\x85\x84\x11\x91\x82\x15a\x1D\x8BWa\x1A\xFC\x81\x87\x11a\x1AmV[\x82\x15a\x1DrW`\x05a\x1Bfa\x1B\x15`\x02a\x03\xA0\x8Ca\x0C\tV[\x99[\x85\x15a\x1D^Wa\x1B+`\x03a\x03\xA0\x83a\x0C\tV[\x99[\x86\x15a\x1DMW\x84a\x1B>\x82\x8Ba\x18\xF2V[\x9A[\x8A\x89\x15a\x1D;WPa\x1BR\x92Pa\x18\xF2V[\x97[\x85a\x1B^\x83a\x0C\tV[P\x01Ua\x0C\tV[P\x01U`\x01`\x01`\xA0\x1B\x03\x87\x81\x16\x80;\x15a\x04\x12W`@\x80Qcp\xA0\x821`\xE0\x1B\x80\x82R0\x86\x83\x01\x90\x81R\x92\x94\x91\x93\x90\x92\x90\x91` \x91\x82\x90\x86\x90\x81\x90\x83\x01\x03\x81\x86Z\xFA\x94\x85\x15a\x04\rW`\0\x95a\x1D\x1CW[P\x8B\x16\x93\x84;\x15a\x04\x12W\x85Q\x84\x81R0\x88\x82\x01\x90\x81R\x90\x94\x90\x83\x90\x86\x90\x81\x90` \x01\x03\x81\x89Z\xFA\x94\x85\x15a\x04\rW`\0\x95a\x1C\xFDW[Pa\x1B\xFC\x8C03\x87a\x18\xFFV[a\x1C\x07\x8B3\x88a\x19\xC1V[\x83;\x15a\x04\x12W\x86Q\x81\x81R0\x89\x82\x01\x90\x81R\x90\x94\x84\x91\x86\x91\x90\x82\x90\x81\x90` \x01\x03\x91Z\xFA\x93\x84\x15a\x04\rW`\0\x94a\x1C\xDEW[P\x85;\x15a\x04\x12W\x86Q\x90\x81R0\x88\x82\x01\x90\x81R\x90\x95\x83\x91\x87\x91\x90\x82\x90\x81\x90` \x01\x03\x91Z\xFA\x94\x85\x15a\x04\rW\x8B\x92`\0\x96a\x1C\xB7W[PP\x90a\x1C~\x91a\x1A\xB7V[\x11a\x1C\xA8W\x86a\x1C\x8D\x91a\x18\xF2V[\x11a\x1C\x9BWPP\x94\x93\x92\x91\x90V[Qc\xF3\xCB\xBC\x87`\xE0\x1B\x81R\xFD[PPQc =\x90\x1D`\xE2\x1B\x81R\xFD[a\x1C~\x93\x92\x96P\x90\x81a\x1C\xD5\x92\x90=\x10a\t\x9FWa\t\x90\x81\x83a\r+V[\x94\x90\x918a\x1CrV[a\x1C\xF6\x91\x94P\x83=\x85\x11a\t\x9FWa\t\x90\x81\x83a\r+V[\x928a\x1C;V[a\x1D\x15\x91\x95P\x83=\x85\x11a\t\x9FWa\t\x90\x81\x83a\r+V[\x938a\x1B\xEFV[a\x1D4\x91\x95P\x82=\x84\x11a\t\x9FWa\t\x90\x81\x83a\r+V[\x938a\x1B\xB8V[\x91PPa\x1DG\x91a\x18\xF2V[\x97a\x1BTV[\x84a\x1DX\x8B\x82a\x18\xF2V[\x9Aa\x1B@V[a\x1Dl`\x02a\x03\xA0\x83a\x0C\tV[\x99a\x1B-V[`\x05a\x1Bfa\x1D\x85`\x03a\x03\xA0\x8Ca\x0C\tV[\x99a\x1B\x17V[a\x1D\x96\x85\x88\x11a\x1AmV[a\x1A\xFCV[\x92\x90a\x1D\xB1a\x03za\x03z`\x01a\x03\xA0\x88a\x0C\tV[\x92\x83;\x15a\x04\x12W`@Qb#\x8Bu`\xEA\x1B\x81R\x93`\xA0\x92\x85\x92\x83\x91\x82\x91a\x1D\xDD\x91\x8A`\x04\x85\x01a\x17\xB8V[\x03\x91Z\xFA\x90\x81\x15a\x04\rW`\0\x91\x82\x83\x80\x95\x81\x93a\x1EaW[P\x15a\x08\x0CWP`\x06a\x1E\\a\x1E\x19\x85`\x04a\x1E\x11\x8Aa\x0C\tV[P\x01Ta\x18\xF2V[\x96a\x1E)\x87`\x05a\x1E\x11\x84a\x0C\tV[\x96a\x1E8\x85\x85a\x1E\x11\x85a\x0C\tV[\x96a\x1EC\x88\x84a\x1F_V[`\x04a\x1EN\x84a\x0C\tV[P\x01U`\x05a\x1B^\x83a\x0C\tV[P\x01UV[\x93\x95PPPPa\x1E\x7F\x91P`\xA0=\x81\x11a\x08{Wa\x08e\x81\x83a\r+V[\x90\x94\x91\x93\x91\x92\x90\x918a\x1D\xF6V[a\x1E\xA1a\x03za\x03z`\x07a\x03\xA0\x85a\x0C\tV[\x91\x82;\x15a\x04\x12W`@Qc\x18\x16\r\xDD`\xE0\x1B\x81R` \x81`\x04\x81\x87Z\xFA\x80\x15a\x04\rWa\x1E\xEA\x93a\tZ\x92`\0\x92a\x1F8W[Pa\x1E\xE1`\x06\x91a\x0C\tV[P\x01T\x90a \x1DV[\x90\x80;\x15a\x04\x12W`@Qc@\xC1\x0F\x19`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x92\x90\x92R`\0\x90\x82\x90\x81\x83\x81`D\x81\x01[\x03\x92Z\xF1\x80\x15a\x04\rWa\x1F+WPV[\x80a\x04\x01a\rZ\x92a\x0C\xF5V[`\x06\x91\x92Pa\x1FWa\x1E\xE1\x91` =\x81\x11a\t\x9FWa\t\x90\x81\x83a\r+V[\x92\x91Pa\x1E\xD5V[a\x1Fsa\x03za\x03z`\x07a\x03\xA0\x85a\x0C\tV[\x91\x82;\x15a\x04\x12W`@Qc\x18\x16\r\xDD`\xE0\x1B\x81R` \x81`\x04\x81\x87Z\xFA\x80\x15a\x04\rWa\x1F\xC2\x93a\x1F\xBC\x92`\0\x92a\x1F\xF6W[Pa\x1F\xB3`\x06\x91a\x0C\tV[P\x01T\x90a `V[\x90a \x90V[\x90\x80;\x15a\x04\x12W`@Qc'p\xA7\xEB`\xE2\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x92\x90\x92R`\0\x90\x82\x90\x81\x83\x81`D\x81\x01a\x1F\x1AV[`\x06\x91\x92Pa \x15a\x1F\xB3\x91` =\x81\x11a\t\x9FWa\t\x90\x81\x83a\r+V[\x92\x91Pa\x1F\xA7V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17\x82\x15\x15\x16\x15a\x08\x93W\x04\x90V[\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x08\x93Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x04\x90V[\x90g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x81\x02\x92\x81\x84\x04\x14\x90\x15\x17\x81\x15\x15\x16\x15a\x08\x93W`\x01\x90`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x08\x93W`\x01g\r\xE0\xB6\xB3\xA7d\0\0`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[nZ\xF4=\x82\x80>\x90=\x91`+W\xFD[\xF3\x90v=`-\x80`\n=9\x81\xF36==7===6=s\0\0\0b\xFF\xFF\xFF\x82`\x88\x1C\x16\x17`\0R`x\x1B\x17` R`7`\t`\0\xF0\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x15a!\x12WV[`@Qc0\xBE\x1A=`\xE2\x1B\x81R`\x04\x90\xFD`\x80\x80`@R4a\0\x19W`@Qa\x11\0\x90\x81a\0g\x829\xF3[bF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01Ra7\xB7`\xF1\x1B`d\x82\x01R`\x84\x90\xFD\xFE`@`\x80\x81R`\x04\x90\x816\x10\x15a\0rW[\x90` `\x84\x92Q\x91bF\x1B\xCD`\xE5\x1B\x83R\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01Rtnor receive functions`X\x1B`d\x82\x01R\xFD[`\0\x805`\xE0\x1C\x80c\x06\xFD\xDE\x03\x14a\x0B\xA0W\x80c\t^\xA7\xB3\x14a\x0B1W\x80c\x15\x8E\xF9>\x14a\x0B\x0CW\x80c\x18\x16\r\xDD\x14a\n\xEFW\x80c#\xB8r\xDD\x14a\n/W\x80c1<\xE5g\x14a\n\x15W\x80c6D\xE5\x15\x14a\t\xF3W\x80c@\xC1\x0F\x19\x14a\tfW\x80cL\xD8\x8Bv\x14a\x06%W\x80cp\xA0\x821\x14a\x05\xEDW\x80c~\xCE\xBE\0\x14a\x05\xB5W\x80c\x95\xD8\x9BA\x14a\x04\xD6W\x80c\x9D\xC2\x9F\xAC\x14a\x04cW\x80c\xA9\x05\x9C\xBB\x14a\x03\xF1W\x80c\xAF\xBA\x13\xC4\x14a\x03\xCAW\x80c\xD5\x05\xAC\xCF\x14a\x01\x8BWc\xDDb\xED>\x14a\x019WPa\0\x11V[\x914a\x01\x86W\x81`\x03\x196\x01\x12a\x01\x81W` \x92\x82\x91a\x01Wa\r\xE1V[a\x01_a\r\xFCV[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x84R\x91\x86R\x83\x83 \x91\x16\x82R\x84R T\x90Q\x90\x81R\xF3[a\x0C\x86V[a\x0C6V[P\x904a\x01\x86W`\xE06`\x03\x19\x01\x12a\x01\x81Wa\x01\xA6a\r\xE1V[\x90a\x01\xAFa\r\xFCV[\x91`D5`d5\x92`\x845\x92`\xFF\x84\x16\x80\x94\x03a\x03\xC6WB\x85\x10a\x03\x83Wa\x01\xD5a\x0FrV[\x95`\x01\x80`\xA0\x1B\x03\x80\x92\x16\x95\x86\x89R` \x95`\x07\x87R\x84\x8A \x98\x89T\x99`\x01\x8B\x01\x90U\x85Q\x92\x85\x89\x85\x01\x95\x7Fnq\xED\xAE\x12\xB1\xB9\x7FM\x1F`7\x0F\xEF\x10\x10_\xA2\xFA\xAE\x01&\x11J\x16\x9Cd\x84]a&\xC9\x87R\x8B\x89\x87\x01R\x16\x9A\x8B``\x86\x01R\x88`\x80\x86\x01R`\xA0\x85\x01R`\xC0\x84\x01R`\xC0\x83R`\xE0\x83\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x94\x84\x82\x10\x86\x83\x11\x17a\x03oW\x81\x88R\x84Q\x90 a\x01\0\x85\x01\x92a\x19\x01`\xF0\x1B\x84Ra\x01\x02\x86\x01Ra\x01\"\x85\x01R`B\x81Ra\x01`\x84\x01\x94\x81\x86\x10\x90\x86\x11\x17a\x03\\W\x84\x87RQ\x90 \x83Ra\x01\x80\x82\x01R`\xA45a\x01\xA0\x82\x01R`\xC45a\x01\xC0\x90\x91\x01R\x87\x80R\x84\x90\x88\x90`\x80\x90`\x01Z\xFA\x15a\x03RW\x86Q\x16\x96\x87\x15\x15\x80a\x03IW[\x15a\x03\x17W\x86\x97\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x95\x96\x97R\x83R\x80\x87 \x86\x88R\x83R\x81\x81\x88 UQ\x90\x81R\xA3\x80\xF3[\x83`d\x92Q\x91bF\x1B\xCD`\xE5\x1B\x83R\x82\x01R`\x0E`$\x82\x01Rm$\xA7+ \xA6$\xA2/\xA9\xA4\xA3\xA7\"\xA9`\x91\x1B`D\x82\x01R\xFD[P\x84\x88\x14a\x02\xD4V[\x81Q=\x88\x82>=\x90\xFD[cNH{q`\xE0\x1B\x8CR`A\x8DR`$\x8C\xFD[PcNH{q`\xE0\x1B\x8CR`A\x8DR`$\x8C\xFD[\x81QbF\x1B\xCD`\xE5\x1B\x81R` \x81\x8A\x01R`\x17`$\x82\x01R\x7FPERMIT_DEADLINE_EXPIRED\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x90\xFD[\x86\x80\xFD[P4a\x01\x86W6`\x03\x19\x01\x12a\x01\x81W`\x08T\x90Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[P\x904a\x01\x86W\x80`\x03\x196\x01\x12a\x01\x81W` \x91a\x04\x0Ea\r\xE1V[\x82`$5\x913\x84R`\x03\x86R\x81\x84 a\x04(\x84\x82Ta\x0FOV[\x90U`\x01`\x01`\xA0\x1B\x03\x16\x80\x84R`\x03\x86R\x92 \x80T\x82\x01\x90U\x82Q\x90\x81R3\x90`\0\x80Q` a\x10\xE0\x839\x81Q\x91R\x90\x85\x90\xA3Q`\x01\x81R\xF3[P\x904a\x01\x86W\x80`\x03\x196\x01\x12a\x01\x81W\x81\x90`\0\x80Q` a\x10\xE0\x839\x81Q\x91R` a\x04\x90a\r\xE1V[`\x08T`$5\x91`\x01`\x01`\xA0\x1B\x03\x91a\x04\xAD\x90\x83\x163\x14a\x0F\x17V[\x16\x93\x84\x86R`\x03\x83R\x80\x86 a\x04\xC4\x83\x82Ta\x0FOV[\x90U\x81`\x02T\x03`\x02UQ\x90\x81R\xA3\x80\xF3[P\x904a\x01\x86W\x81`\x03\x196\x01\x12a\x01\x81W\x80Q\x90\x82`\x01\x80T\x91a\x04\xFA\x83a\x0C\xD6V[\x80\x86R\x92\x82\x81\x16\x90\x81\x15a\x05\x8DWP`\x01\x14a\x051W[PPPa\x05#\x82a\x05-\x94\x03\x83a\r\x10V[Q\x91\x82\x91\x82a\rHV[\x03\x90\xF3[\x94P\x80\x85R\x7F\xB1\x0E-Rv\x12\x07;&\xEE\xCD\xFDq~j2\x0C\xF4KJ\xFA\xC2\xB0s-\x9F\xCB\xE2\xB7\xFA\x0C\xF6[\x82\x86\x10a\x05uWPPPa\x05#\x82` a\x05-\x95\x82\x01\x01\x94a\x05\x11V[\x80T` \x87\x87\x01\x81\x01\x91\x90\x91R\x90\x95\x01\x94\x81\x01a\x05XV[a\x05-\x97P\x86\x93P` \x92Pa\x05#\x94\x91P`\xFF\x19\x16\x82\x84\x01R\x15\x15`\x05\x1B\x82\x01\x01\x94a\x05\x11V[P\x904a\x01\x86W` 6`\x03\x19\x01\x12a\x01\x81W` \x91\x81\x90`\x01`\x01`\xA0\x1B\x03a\x05\xDDa\r\xE1V[\x16\x81R`\x07\x84R T\x90Q\x90\x81R\xF3[P\x904a\x01\x86W` 6`\x03\x19\x01\x12a\x01\x81W` \x91\x81\x90`\x01`\x01`\xA0\x1B\x03a\x06\x15a\r\xE1V[\x16\x81R`\x03\x84R T\x90Q\x90\x81R\xF3[P\x824a\x01\x86W\x82`\x03\x196\x01\x12a\x01\x81Wg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x815\x81\x81\x11a\taWa\x06V\x906\x90\x84\x01a\x0E\x12V[\x91`$5\x82\x81\x11a\taWa\x06n\x906\x90\x83\x01a\x0E\x12V[\x94`\x08T\x90`\xFF\x82`\xA0\x1C\x16a\t)WP`\x01`\x01`\xA0\x1B\x03\x19\x163\x17`\x08U\x82Q\x82\x81\x11a\t\x16W\x80a\x06\xA2\x86Ta\x0C\xD6V[\x94`\x1F\x95\x86\x81\x11a\x08\xBDW[P` \x90\x86\x83\x11`\x01\x14a\x08NW\x87\x92a\x08CW[PP\x81`\x01\x1B\x91`\0\x19\x90`\x03\x1B\x1C\x19\x16\x17\x84U[\x84Q\x91\x82\x11a\x080WP`\x01\x91a\x06\xEF\x83Ta\x0C\xD6V[\x81\x81\x11a\x07\xCEW[P` \x90\x82\x11`\x01\x14a\x07SW\x83\x94\x82\x93\x94\x92a\x07HW[PP`\0\x19`\x03\x83\x90\x1B\x1C\x19\x16\x90\x82\x1B\x17\x90U[F`\x05Ua\x07/a\x0F\x8CV[`\x06U`\x08\x80T`\xFF`\xA0\x1B\x19\x16`\x01`\xA0\x1B\x17\x90U\x80\xF3[\x01Q\x90P\x84\x80a\x07\x0FV[\x82\x84R\x7F\xB1\x0E-Rv\x12\x07;&\xEE\xCD\xFDq~j2\x0C\xF4KJ\xFA\xC2\xB0s-\x9F\xCB\xE2\xB7\xFA\x0C\xF6\x90`\x1F\x19\x83\x16\x85[\x81\x81\x10a\x07\xB8WP\x95\x83\x85\x96\x97\x10a\x07\x9FW[PPP\x81\x1B\x01\x90Ua\x07#V[\x01Q`\0\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U\x84\x80\x80a\x07\x92V[\x87\x83\x01Q\x84U\x92\x85\x01\x92` \x92\x83\x01\x92\x01a\x07\x7FV[\x83\x85R\x7F\xB1\x0E-Rv\x12\x07;&\xEE\xCD\xFDq~j2\x0C\xF4KJ\xFA\xC2\xB0s-\x9F\xCB\xE2\xB7\xFA\x0C\xF6\x82\x80\x85\x01`\x05\x1C\x82\x01\x92` \x86\x10a\x08'W[\x01`\x05\x1C\x01\x90\x84\x90[\x82\x81\x10a\x08\x1CWPPa\x06\xF7V[\x86\x81U\x01\x84\x90a\x08\x0EV[\x92P\x81\x92a\x08\x05V[cNH{q`\xE0\x1B\x84R`A\x90R`$\x83\xFD[\x01Q\x90P\x87\x80a\x06\xC3V[\x87\x80R`\0\x80Q` a\x10\xC0\x839\x81Q\x91R\x92P`\x1F\x19\x84\x16\x88[\x81\x81\x10a\x08\xA5WP\x90\x84`\x01\x95\x94\x93\x92\x10a\x08\x8CW[PPP\x81\x1B\x01\x84Ua\x06\xD8V[\x01Q`\0\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U\x87\x80\x80a\x08\x7FV[\x92\x93` `\x01\x81\x92\x87\x86\x01Q\x81U\x01\x95\x01\x93\x01a\x08iV[\x90\x91P\x86\x80R`\0\x80Q` a\x10\xC0\x839\x81Q\x91R\x86\x80\x85\x01`\x05\x1C\x82\x01\x92` \x86\x10a\t\rW[\x90\x85\x94\x93\x92\x91\x01`\x05\x1C\x01\x90[\x81\x81\x10a\x08\xFFWPa\x06\xAEV[\x88\x81U\x84\x93P`\x01\x01a\x08\xF2V[\x92P\x81\x92a\x08\xE5V[cNH{q`\xE0\x1B\x85R`A\x82R`$\x85\xFD[QbF\x1B\xCD`\xE5\x1B\x81R` \x81\x84\x01R`\x13`$\x82\x01Rr\x10[\x1C\x99XY\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`j\x1B`D\x82\x01R`d\x90\xFD[a\r\x91V[P\x914a\x01\x86W\x81`\x03\x196\x01\x12a\x01\x81Wa\t\x80a\r\xE1V[`\x08T`$5\x92`\x01`\x01`\xA0\x1B\x03\x92\x91a\t\x9E\x90\x84\x163\x14a\x0F\x17V[`\x02T\x90\x84\x82\x01\x80\x92\x11a\t\xE0WP\x92`\0\x80Q` a\x10\xE0\x839\x81Q\x91R\x92` \x92\x87\x95`\x02U\x16\x94\x85\x85R`\x03\x83R\x80\x85 \x82\x81T\x01\x90UQ\x90\x81R\xA3\x80\xF3[cNH{q`\xE0\x1B\x87R`\x11\x90R`$\x86\xFD[P4a\x01\x86W6`\x03\x19\x01\x12a\x01\x81W` \x90a\n\x0Ea\x0FrV[\x90Q\x90\x81R\xF3[P4a\x01\x86W6`\x03\x19\x01\x12a\x01\x81W` \x90Q`\x12\x81R\xF3[P4a\x01\x86W``6`\x03\x19\x01\x12a\x01\x81Wa\nIa\r\xE1V[`\0\x80Q` a\x10\xE0\x839\x81Q\x91Ra\n`a\r\xFCV[`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x80\x85R` \x87\x81R\x86\x86 3\x87R\x81R\x86\x86 T\x90\x97\x91\x94\x88\x93`D5\x93\x89\x93\x85`\x01\x82\x01a\n\xCCW[PPP\x86\x88R`\x03\x85R\x82\x88 a\n\xAD\x85\x82Ta\x0FOV[\x90U\x16\x95\x86\x81R`\x03\x84R \x81\x81T\x01\x90U\x85Q\x90\x81R\xA3Q`\x01\x81R\xF3[a\n\xD5\x91a\x0FOV[\x90\x88\x8AR\x86R\x83\x89 3\x8AR\x86R\x83\x89 U8\x80\x85a\n\x95V[P4a\x01\x86W6`\x03\x19\x01\x12a\x01\x81W` \x90`\x02T\x90Q\x90\x81R\xF3[P4a\x01\x86W6`\x03\x19\x01\x12a\x01\x81W` \x90`\xFF`\x08T`\xA0\x1C\x16\x90Q\x90\x15\x15\x81R\xF3[P\x914a\x01\x86W\x81`\x03\x196\x01\x12a\x01\x81W` \x92a\x0BNa\r\xE1V[\x91\x83`$5\x92\x83\x923\x82R\x87R\x81\x81 \x94`\x01\x80`\xA0\x1B\x03\x16\x94\x85\x82R\x87R U\x82Q\x90\x81R\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x843\x92\xA3Q`\x01\x81R\xF3[P\x904a\x0C6W\x81`\x03\x196\x01\x12a\x01\x81W\x80Q\x90\x82\x80Ta\x0B\xC1\x81a\x0C\xD6V[\x80\x85R\x91`\x01\x91\x80\x83\x16\x90\x81\x15a\x05\x8DWP`\x01\x14a\x0B\xECWPPPa\x05#\x82a\x05-\x94\x03\x83a\r\x10V[\x80\x80\x96PR`\0\x80Q` a\x10\xC0\x839\x81Q\x91R[\x82\x86\x10a\x0C\x1EWPPPa\x05#\x82` a\x05-\x95\x82\x01\x01\x94a\x05\x11V[\x80T` \x87\x87\x01\x81\x01\x91\x90\x91R\x90\x95\x01\x94\x81\x01a\x0C\x01V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01Ra7\xB7`\xF1\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[\x90`\x01\x82\x81\x1C\x92\x16\x80\x15a\r\x06W[` \x83\x10\x14a\x0C\xF0WV[cNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[\x91`\x7F\x16\x91a\x0C\xE5V[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\r2W`@RV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[` \x80\x82R\x82Q\x81\x83\x01\x81\x90R\x90\x93\x92`\0[\x82\x81\x10a\r}WPP`@\x92\x93P`\0\x83\x82\x84\x01\x01R`\x1F\x80\x19\x91\x01\x16\x01\x01\x90V[\x81\x81\x01\x86\x01Q\x84\x82\x01`@\x01R\x85\x01a\r[V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[`\x045\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\r\xF7WV[`\0\x80\xFD[`$5\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\r\xF7WV[\x90\x80`\x1F\x83\x01\x12\x15a\x0E\xBEW\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\r2W`@Q\x92` \x92a\x0EI`\x1F\x84\x01`\x1F\x19\x16\x85\x01\x86a\r\x10V[\x82\x85R\x83\x83\x83\x01\x01\x11a\x0EiW\x90\x80\x83`\0\x94\x93\x01\x83\x86\x017\x83\x01\x01R\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x84\x90R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x90\xFD[\x15a\x0F\x1EWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01RhOnly DFMM`\xB8\x1B`D\x82\x01R`d\x90\xFD[\x91\x90\x82\x03\x91\x82\x11a\x0F\\WV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\x05TF\x03a\x0F\x81W`\x06T\x90V[a\x0F\x89a\x0F\x8CV[\x90V[`@Q`\0\x90\x81T\x90a\x0F\x9E\x82a\x0C\xD6V[\x80\x82R\x81` \x94\x85\x82\x01\x94`\x01\x90\x87\x82\x82\x16\x91\x82`\0\x14a\x10\xA1WPP`\x01\x14a\x10YW[Pa\x0F\xD0\x92P\x03\x82a\r\x10V[Q\x90 \x90`@Q\x90\x81\x01\x91\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x83R`@\x82\x01R\x7F\xC8\x9E\xFD\xAAT\xC0\xF2\x0Cz\xDFa(\x82\xDF\tP\xF5\xA9Qc~\x03\x07\xCD\xCBLg/)\x8B\x8B\xC6``\x82\x01RF`\x80\x82\x01R0`\xA0\x82\x01R`\xA0\x81R`\xC0\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\r2W`@RQ\x90 \x90V[`\0\x80\x80R\x87\x92P\x90`\0\x80Q` a\x10\xC0\x839\x81Q\x91R[\x85\x83\x10a\x10\x89WPPa\x0F\xD0\x93P\x82\x01\x018a\x0F\xC3V[\x80T\x83\x88\x01\x85\x01R\x86\x94P\x88\x93\x90\x92\x01\x91\x81\x01a\x10rV[`\xFF\x19\x16\x88Ra\x0F\xD0\x95\x15\x15`\x05\x1B\x85\x01\x01\x92P8\x91Pa\x0F\xC3\x90PV\xFE)\r\xEC\xD9T\x8Bb\xA8\xD6\x03E\xA9\x888o\xC8K\xA6\xBC\x95H@\x08\xF66/\x93\x16\x0E\xF3\xE5c\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF";
    /// The bytecode of the contract.
    pub static DFMM_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\x11{W`\x005`\xE0\x1C\x80c\x02\x16\xB88\x14a\0\xDCW\x80c\x06\x8B\xCD\x8D\x14a\0\xD7W\x80c\x14U\xF1\xFC\x14a\0\xD2W\x80c.\xC3\x81\x88\x14a\0\xCDW\x80c;\xE6\xA3A\x14a\0\xC8W\x80c\x9D\x94/\x9A\x14a\0\xC3W\x80c\x9F\xE1\xC1n\x14a\0\xBEW\x80c\xACJ\xFA8\x14a\0\xB9W\x80c\xAC\x96P\xD8\x14a\0\xB4W\x80c\xAF\xFE\xD0\xE0\x14a\0\xAFW\x80c\xB4b\xCD%\x14a\0\xAAW\x80c\xBD\x06%\xAB\x14a\0\xA5Wc\xCE\x15;\xF4\x03a\x11{Wa\x11\x1DV[a\x0F\xD1V[a\x0F\x8CV[a\x0FnV[a\x0E#V[a\x0CGV[a\x0BxV[a\t\xC5V[a\x08\x98V[a\x06cV[a\x05\xACV[a\x04@V[a\x03=V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01Ra7\xB7`\xF1\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rjrray stride`\xA8\x1B`d\x82\x01R`\x84\x90\xFD[`@`\x03\x19\x82\x01\x12a\x038W`\x045\x91`$5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x83\x82\x11a\x033W\x80`#\x83\x01\x12\x15a\x03.W\x81`\x04\x015\x93\x84\x11a\x02\xD5W`$\x84\x83\x01\x01\x11a\x02\xD0W`$\x01\x91\x90V[a\x02*V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x0ENL/$\r\x8C\xAD\xCC\xEE\x8D`\xAB\x1B`d\x82\x01R`\x84\x90\xFD[a\x01\xD1V[a\x01\x81V[a\x011V[4a\x04;Wa\x03K6a\x02\x83V[`\x02`\x01\x93\x92\x93T\x14a\x04)W`\x02`\x01Ua\x03\x86a\x03za\x03l\x84a\x0C\tV[PT`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x90V[3\x03a\x04\x17Wa\x03\xAFa\x03za\x03z`\x01a\x03\xA0\x86a\x0C\tV[P\x01T`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x80;\x15a\x04\x12Wa\x03\xDA\x93`\0\x80\x94`@Q\x96\x87\x95\x86\x94\x85\x93bB\xD7\x07`\xE3\x1B\x85R`\x04\x85\x01a\x17\xB8V[\x03\x92Z\xF1\x80\x15a\x04\rWa\x03\xF4W[a\x03\xF2`\x01\x80UV[\0[\x80a\x04\x01a\x04\x07\x92a\x0C\xF5V[\x80a\x0FcV[8a\x03\xE9V[a\x17\xE6V[a\x17*V[`@Qc#\x01\x9Eg`\xE0\x1B\x81R`\x04\x90\xFD[`@Qc\x03\xCB\x96\xDB`\xE2\x1B\x81R`\x04\x90\xFD[a\0\xE1V[4a\x04;W` 6`\x03\x19\x01\x12a\x038W`@Qa\x04]\x81a\r\x0EV[`\xE0`\0\x91\x82\x81R\x82` \x82\x01R\x82`@\x82\x01R\x82``\x82\x01R\x82`\x80\x82\x01R\x82`\xA0\x82\x01R\x82`\xC0\x82\x01R\x01Ra\x05\xA8a\x04\x99`\x045a\x0C\tV[Pa\x05=a\x05-`\x07a\x04\xAAa\rMV[\x84T`\x01`\x01`\xA0\x1B\x03\x16\x81R\x93`\x01\x81\x01T`\x01`\x01`\xA0\x1B\x03\x16` \x86\x01R`\x02\x81\x01T`\x01`\x01`\xA0\x1B\x03\x16`@\x86\x01R`\x03\x81\x01Ta\x05\x01\x90`\x01`\x01`\xA0\x1B\x03\x16[`\x01`\x01`\xA0\x1B\x03\x16``\x87\x01RV[`\x04\x81\x01T`\x80\x86\x01R`\x05\x81\x01T`\xA0\x86\x01R`\x06\x81\x01T`\xC0\x86\x01R\x01T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16`\xE0\x83\x01RV[`@Q\x91\x82\x91\x82\x91\x90\x91`\xE0a\x01\0\x82\x01\x93\x81`\x01\x80`\xA0\x1B\x03\x91\x82\x81Q\x16\x85R\x82` \x82\x01Q\x16` \x86\x01R\x82`@\x82\x01Q\x16`@\x86\x01R\x82``\x82\x01Q\x16``\x86\x01R`\x80\x81\x01Q`\x80\x86\x01R`\xA0\x81\x01Q`\xA0\x86\x01R`\xC0\x81\x01Q`\xC0\x86\x01R\x01Q\x16\x91\x01RV[\x03\x90\xF3[4a\x04;W`\x03\x19` 6\x82\x01\x12a\x038W`\x045\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x033W`\x80\x90\x826\x03\x01\x12a\x06\x0EWa\x05\xECa\x05\xA8\x91`\x04\x01a\x12\xCEV[`@\x80Q\x94\x85R` \x85\x01\x93\x90\x93R\x91\x83\x01R``\x82\x01R\x90\x81\x90`\x80\x82\x01\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FABI decoding: struct calldata to`D\x82\x01Rf\x1B\xC8\x1C\xDA\x1B\xDC\x9D`\xCA\x1B`d\x82\x01R`\x84\x90\xFD[4a\x04;Wa\x06q6a\x02\x83V[\x91\x90`\x02`\x01T\x14a\x04)W`\x02`\x01Ua\x06\x96a\x03za\x03z`\x01a\x03\xA0\x86a\x0C\tV[\x92\x83;\x15a\x04\x12W`@Qb#\x8Bu`\xEA\x1B\x81R\x93`\xA0\x92\x85\x92\x83\x91\x82\x91a\x06\xC2\x91\x88`\x04\x85\x01a\x17\xB8V[\x03\x91Z\xFA\x90\x81\x15a\x04\rW`\0\x90\x81\x82\x80\x95\x81\x95a\x08FW[P\x15a\x08\x0CWP\x90\x81a\x06\xFE`\x04a\x06\xF5a\x05\xA8\x95a\x0C\tV[P\x01T\x83a\x18\xF2V[\x93a\x07\x16`\x05a\x07\r\x84a\x0C\tV[P\x01T\x87a\x18\xF2V[\x95a\x07%`\x06a\x06\xF5\x85a\x0C\tV[\x93a\x070\x85\x85a\x1E\x8DV[`\x04a\x07;\x85a\x0C\tV[P\x01U`\x05a\x07I\x84a\x0C\tV[P\x01U`\x06a\x07W\x83a\x0C\tV[P\x01Ua\x07\x9F\x85`\x02a\x07\x87\x87a\x07m\x86a\x0C\tV[P\x90\x92\x01T`\x01`\x01`\xA0\x1B\x03\x92\x900\x903\x90\x85\x16a\x18\xFFV[a\x07\x90\x84a\x0C\tV[P0\x91`\x033\x92\x01T\x16a\x18\xFFV[`@\x80Q\x91\x82R` \x82\x01\x85\x90R\x81\x01\x85\x90R``\x81\x01\x82\x90R3\x90\x7F\x95\x97W\x7F3\x93 w^c\xD3\xFE\xD7\xD5\xDD\xE66[\xAD\xCC\x9F\xCC\xDBf\xB3H\x94c\x0C\xA9\x8Bo\x90\x80`\x80\x81\x01[\x03\x90\xA2a\x07\xEF`\x01\x80UV[`@Q\x93\x84\x93\x84`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[\x80a\x08Ba\x08\x1B`\0\x93a\x19\x80V[`@Qcw`m)`\xE1\x1B\x81R\x93\x90\x92\x12`\x04\x84\x01R`$\x83\x01\x91\x90\x91R\x81\x90`D\x82\x01\x90V[\x03\x90\xFD[\x93PPP\x92Pa\x08m\x91P`\xA0=\x81\x11a\x08{W[a\x08e\x81\x83a\r+V[\x81\x01\x90a\x17\x8AV[\x94\x91\x90\x92\x90\x92\x94\x938a\x06\xDBV[P=a\x08[V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\x08\x93WV[`\0\x80\xFD[4a\x04;W`@6`\x03\x19\x01\x12a\x038W`\x045a\x08\xB5\x81a\x08\x82V[`$5\x90a\x08\xCDa\x03za\x03z`\x07a\x03\xA0\x86a\x0C\tV[\x90\x81;\x15a\x04\x12W`@Qcp\xA0\x821`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16`\x04\x82\x01R` \x80\x82`$\x81\x86Z\xFA\x91\x82\x15a\x04\rW`\0\x92a\t\xA6W[P\x82;\x15a\x04\x12W\x80`\x04\x93`@Q\x94\x85\x80\x92c\x18\x16\r\xDD`\xE0\x1B\x82RZ\xFA\x80\x15a\x04\rWa\x05\xA8\x94a\t`\x94a\tZ\x93`\0\x93a\tpW[PPa\tR`\x06\x91a\x0C\tV[P\x01Ta \x1DV[\x90a ?V[`@Q\x90\x81R\x90\x81\x90` \x82\x01\x90V[`\x06\x92\x93Pa\tR\x91\x81a\t\x98\x92\x90=\x10a\t\x9FW[a\t\x90\x81\x83a\r+V[\x81\x01\x90a\x1A\xA8V[\x92\x91a\tEV[P=a\t\x86V[\x81a\t\xBE\x92\x93P=\x84\x11a\t\x9FWa\t\x90\x81\x83a\r+V[\x908a\t\x0CV[4a\x04;Wa\t\xD36a\x02\x83V[`\x02`\x01T\x14a\x04)Wa\t\xEC\x91`\x02`\x01U\x83a\x1D\x9BV[a\n\x02a\x03za\x03z`\x02a\x03\xA0\x88\x96\x98a\x0C\tV[\x91\x82;\x15a\x04\x12W`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x80\x82R3`\x04\x83\x01R`$\x82\x01\x86\x90R\x93` \x91\x90\x82\x90\x82\x90`D\x90\x82\x90`\0\x90Z\xF1\x80\x15a\x04\rWa\x0B[W[Pa\nXa\x03za\x03z`\x03a\x03\xA0\x86a\x0C\tV[\x93\x84;\x15a\x04\x12W`@Q\x90\x81R3`\x04\x82\x01R`$\x81\x01\x87\x90R\x93\x81\x90\x85\x90`D\x90\x82\x90`\0\x90Z\xF1\x93\x84\x15a\x04\rWa\x05\xA8\x94a\x0B-W[PPa\n\xBF\x84a\n\xA1\x83a\x0C\tV[P`\x02\x01T3\x90a\n\xBA\x90`\x01`\x01`\xA0\x1B\x03\x16a\x03zV[a\x19\xC1V[a\n\xE5\x85a\n\xCC\x83a\x0C\tV[P`\x03\x01T3\x90a\n\xBA\x90`\x01`\x01`\xA0\x1B\x03\x16a\x03zV[`@\x80Q\x91\x82R` \x82\x01\x85\x90R\x81\x01\x85\x90R``\x81\x01\x82\x90R3\x90\x7F\xAC\xBE\x12~\x93\xA8\xA0\xB2x\xD8\xE0n' [=\xF9\xD1\xF3\x81$\x14\xBC\x89\x17\xC7t\xA87\x101n\x90\x80`\x80\x81\x01a\x07\xE3V[\x81a\x0BL\x92\x90=\x10a\x0BTW[a\x0BD\x81\x83a\r+V[\x81\x01\x90a\x19\xADV[P8\x80a\n\x92V[P=a\x0B:V[a\x0Bq\x90\x82=\x84\x11a\x0BTWa\x0BD\x81\x83a\r+V[P8a\nCV[4a\x04;W`@6`\x03\x19\x01\x12a\x038W`\x045`$5a\x0B\x98\x81a\x08\x82V[`\x02`\x01T\x14a\x04)W`\x02`\x01Ua\x0B\xB0\x82a\x0C\tV[PT`\x01`\x01`\xA0\x1B\x03\x163\x03a\x04\x17Wa\x0B\xCDa\x0B\xED\x92a\x0C\tV[P\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x01\x80U\0[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0T\x81\x10\x15a\x0CBW`\0\x80R`\x03\x1B\x7F)\r\xEC\xD9T\x8Bb\xA8\xD6\x03E\xA9\x888o\xC8K\xA6\xBC\x95H@\x08\xF66/\x93\x16\x0E\xF3\xE5c\x01\x90`\0\x90V[a\x0B\xF3V[4a\x04;W` 6`\x03\x19\x01\x12a\x038W`\x045`\0T\x81\x10\x15a\x08\x93Wa\x0Cn\x90a\x0C\tV[P\x80T`\x01\x82\x01T`\x02\x83\x01T`\x03\x84\x01T`\x04\x85\x01T`\x05\x86\x01T`\x06\x87\x01T`\x07\x90\x97\x01T`@\x80Q`\x01`\x01`\xA0\x1B\x03\x98\x89\x16\x81R\x96\x88\x16` \x88\x01R\x94\x87\x16\x94\x86\x01\x94\x90\x94R\x91\x85\x16``\x85\x01R`\x80\x84\x01R`\xA0\x83\x01R`\xC0\x82\x01\x93\x90\x93R\x91\x16`\xE0\x82\x01Ra\x01\0\x90\xF3[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\r\tW`@RV[a\x0C\xDFV[a\x01\0\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\r\tW`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\r\tW`@RV[`@Q\x90a\rZ\x82a\r\x0EV[V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\r\tW`\x05\x1B` \x01\x90V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\r\tW`\x1F\x01`\x1F\x19\x16` \x01\x90V[` \x80\x82\x01\x81\x83R\x83Q\x80\x91R`@\x83\x01\x91\x80`@\x83`\x05\x1B\x86\x01\x01\x95\x01\x93`\0\x80\x91[\x84\x83\x10a\r\xC5WPPPPPPP\x90V[\x90\x91\x92\x93\x94\x95\x96`?\x19\x82\x82\x03\x01\x87R\x84\x88Q\x80Q\x90\x81\x84R\x85[\x82\x81\x10a\x0E\x0FWPP\x80\x83\x01\x82\x01\x85\x90R`\x1F\x01`\x1F\x19\x16\x90\x91\x01\x81\x01\x97\x81\x01\x96\x01\x94\x93\x92`\x01\x01\x91\x90a\r\xB4V[\x81\x81\x01\x84\x01Q\x85\x82\x01\x85\x01R\x88\x93\x01a\r\xE0V[4a\x04;W` \x80`\x03\x196\x01\x12a\x038W`\x04\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x825\x82\x81\x11a\x033W6`#\x82\x01\x12\x15a\x03.W\x80\x84\x015\x92a\x0Ed\x84a\r\\V[\x93`@\x95a\x0Et\x87Q\x96\x87a\r+V[\x81\x86R\x84\x86\x01\x92`$\x80\x93`\x05\x1B\x86\x01\x01\x946\x86\x11a\x02\xD0W\x83\x81\x01\x94[\x86\x86\x10a\x0E\xB1Wa\x05\xA8\x8Aa\x0E\xA6\x8Ba\x12\"V[\x90Q\x91\x82\x91\x82a\r\x90V[\x855\x83\x81\x11a\x03.W\x82\x016`C\x82\x01\x12\x15a\x03.W\x85\x81\x015a\x0E\xD4\x81a\rtV[\x91a\x0E\xE1\x8DQ\x93\x84a\r+V[\x81\x83R`D\x906\x82\x84\x83\x01\x01\x11a\x0F\x11W\x8B\x83\x81\x96\x94\x82\x96\x94`\0\x94\x01\x83\x86\x017\x83\x01\x01R\x81R\x01\x95\x01\x94a\x0E\x92V[P\x86\x7FABI decoding: invalid byte array`\x84\x92\x8F\x8B\x8F`'\x92Q\x95bF\x1B\xCD`\xE5\x1B\x87R\x86\x01R\x84\x01R\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R\xFD[`\0\x91\x03\x12a\x038WV[4a\x04;W`\x006`\x03\x19\x01\x12a\x038W` `\0T`@Q\x90\x81R\xF3[4a\x04;W`\x006`\x03\x19\x01\x12a\x038W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x04;Wa\x0F\xDF6a\x02\x83V[\x91\x90`\x02`\x01T\x14a\x04)W`\x02`\x01Ua\x10\x04a\x03za\x03z`\x01a\x03\xA0\x86a\x0C\tV[\x92\x83;\x15a\x04\x12W`@Qc2\x14\x89\x0F`\xE0\x1B\x81R\x93`\xC0\x92\x85\x92\x83\x91\x82\x91a\x101\x91\x88`\x04\x85\x01a\x17\xB8V[\x03\x91Z\xFA\x91\x82\x15a\x04\rW`\0\x80\x93\x81\x80\x93\x81\x92a\x10\xE1W[P\x15a\x10\xD2W\x83\x94P`\x06a\x10aa\x10j\x95a\x0C\tV[P\x01U\x83a\x1A\xC4V[\x94\x92P\x92\x90P\x7FL}\xEF\x84\xE4++\xC0\xA5\xAA\xB2\"\x86\x8D\xD7\xA0\x92\xB53w\xA4\xB57\xAB\xCD\x944Zz\x85'\xED`@Q\x80a\x10\xB5\x87\x873\x96\x84`@\x91\x94\x93\x92``\x82\x01\x95\x15\x15\x82R` \x82\x01R\x01RV[\x03\x90\xA3a\x10\xC1`\x01\x80UV[`@\x80Q\x91\x82R` \x82\x01\x92\x90\x92R\xF3[`\0\x85a\x08Ba\x08\x1B\x82a\x19\x80V[\x93PPPPa\x11\x08\x91\x92P`\xC0=\x81\x11a\x11\x16W[a\x11\0\x81\x83a\r+V[\x81\x01\x90a\x1A8V[\x93\x95\x94\x90\x93\x91\x92P8a\x10JV[P=a\x10\xF6V[4a\x04;W` 6`\x03\x19\x01\x12a\x038W`\x045`\x04a\x11<\x82a\x0C\tV[P\x01Ta\x05\xA8`\x06a\x11[`\x05a\x11R\x86a\x0C\tV[P\x01T\x94a\x0C\tV[P\x01T`@Q\x93\x84\x93\x84`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01Rtnor receive functions`X\x1B`d\x82\x01R`\x84\x90\xFD[\x80Q\x82\x10\x15a\x0CBW` \x91`\x05\x1B\x01\x01\x90V[=\x15a\x12\x1DW=\x90a\x12\x03\x82a\rtV[\x91a\x12\x11`@Q\x93\x84a\r+V[\x82R=`\0` \x84\x01>V[``\x90V[\x90\x81Q\x91a\x12/\x83a\r\\V[\x92a\x12=`@Q\x94\x85a\r+V[\x80\x84Ra\x12L`\x1F\x19\x91a\r\\V[\x01`\0[\x81\x81\x10a\x12\xBDWPP`\0[\x81Q\x81\x03a\x12\xB9Wa\x12n\x81\x83a\x11\xDEV[Q`\0\x80\x82Q` \x80\x94\x010Z\xF4\x90a\x12\x85a\x11\xF2V[\x91\x15a\x12\xACWP\x90`\x01\x91a\x12\x9A\x82\x87a\x11\xDEV[Ra\x12\xA5\x81\x86a\x11\xDEV[P\x01a\x12\\V[\x90\x80Q\x91\x82\x15a\x08\x93W\x01\xFD[PPV[\x80``` \x80\x93\x88\x01\x01R\x01a\x12PV[`\x02`\x01T\x14a\x04)W`\x02`\x01U` \x81\x01\x90a\x12\xEB\x82a\x16\x16V[\x92`@\x92\x83\x83\x01a\x12\xFEa\x03z\x82a\x16\x16V[`\x01`\x01`\xA0\x1B\x03\x96\x87\x16\x14a\x16\x05Wa\x13\x1Da\x03za\x03z\x86a\x16\x16V[\x94`\0\x92\x83Ta\x130``\x88\x01\x88a\x16hV[\x90\x98\x80;\x15a\x04\x12Wa\x13]\x99\x87`\xA0\x94\x87Q\x9C\x8D\x95\x86\x94\x85\x93c\x9F\x83\x13{`\xE0\x1B\x85R`\x04\x85\x01a\x17\xB8V[\x03\x92Z\xF1\x95\x86\x15a\x04\rW\x84\x98\x85\x86\x9A\x87\x9A\x88\x9Aa\x15\xD4W[P\x15a\x15\x9FWPa\x13\xACa\x03za\x03z\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a \xBCV[\x95\x86;\x15a\x04\x12W\x84Qc&lE\xBB`\xE1\x1B\x81R`@`\x04\x82\x01R`\0`D\x82\x01\x81\x90R```$\x83\x01R`d\x82\x01R\x81\x81`\x84\x81\x83\x8CZ\xF1\x80\x15a\x04\rWa\x15\x8CW[P\x86;\x15a\x04\x12W\x84Qc@\xC1\x0F\x19`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x8A\x90R\x96\x81\x88`D\x81\x83\x85Z\xF1\x97\x88\x15a\x04\rWa\x15&\x85a\x15!\x8F\x9B\x8F\x90\x8F\x89\x98\x8F\x8F\x8F\x8F\x91\x9B\x7F\xF7\xC0\x95\xAF\xDDB\r*7\x8A\x88r\x05\xCFW\xEB\xEE/m\xD0\x07\xDD\xD7c\xC8Z\xEA\xC0Z\xE3\xD7\xCA\x9F\x95a\x14\xF2\x94a\x14\xCCa\x152\x9F\x94a\x15,\x9Fa\x14\xED\x96a\x14\x9Ba\x14\x95a\x14\x8Fa\x15\x0E\x9F\x9Ea\x14\xF8\x9F\x95a\x04\xF1\x96a\x15yW[Pa\x16\x16V[\x9Aa\x16\x16V[\x91a\x16\x16V[\x92a\x14\xBBa\x14\xA7a\rMV[3\x81R\x9A`\x01`\x01`\xA0\x1B\x03\x16` \x8C\x01RV[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90\x89\x01RV[`\x80\x85\x01R`\xA0\x84\x01\x89\x90R`\xC0\x84\x01R`\x01`\x01`\xA0\x1B\x03\x16`\xE0\x83\x01RV[a\x17\xF2V[Ta\x18\xDEV[\x9Da\x15\x05a\x03z\x8Da\x16\x16V[0\x903\x90a\x18\xFFV[03a\x15\x1Ca\x03z\x8Fa\x16\x16V[a\x18\xFFV[a\x16\x16V[\x95a\x16\x16V[\x96a\x16\x16V[\x94Q`\x01`\x01`\xA0\x1B\x03\x94\x90\x94\x16\x84R` \x84\x01\x87\x90R`@\x84\x01\x8B\x90R``\x84\x01\x8A\x90R`\x80\x84\x01\x89\x90R\x90\x93\x16\x93\x90\x92\x16\x913\x91`\xA0\x90\xA4\x93\x92\x91\x90a\rZ`\x01\x80UV[\x80a\x04\x01a\x15\x86\x92a\x0C\xF5V[8a\x14\x89V[\x80a\x04\x01a\x15\x99\x92a\x0C\xF5V[8a\x13\xF0V[\x80a\x08B\x86a\x15\xAE\x8A\x94a\x19\x80V[\x90Qcw`m)`\xE1\x1B\x81R\x93\x90\x92\x12`\x04\x84\x01R`$\x83\x01\x91\x90\x91R\x81\x90`D\x82\x01\x90V[\x93\x9BPPP\x96Pa\x15\xF4\x91\x97P`\xA0=\x81\x11a\x08{Wa\x08e\x81\x83a\r+V[\x91\x9A\x90\x99\x91\x98\x91\x93\x90\x92\x908a\x13vV[\x84Qc3\x91\n\xEF`\xE1\x1B\x81R`\x04\x90\xFD[5a\x16 \x81a\x08\x82V[\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FCalldata tail too short\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x90\xFD[\x905\x90`\x1E\x19\x816\x03\x01\x82\x12\x15a\x16\xE5W\x01\x805\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x16\xA0W` \x01\x91\x816\x03\x83\x13a\x16\x9BWV[a\x16#V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FInvalid calldata tail length\0\0\0\0`D\x82\x01R`d\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FInvalid calldata tail offset\0\0\0\0`D\x82\x01R`d\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01Rd code`\xD8\x1B`d\x82\x01R`\x84\x90\xFD[Q\x90\x81\x15\x15\x82\x03a\x08\x93WV[\x90\x81`\xA0\x91\x03\x12a\x038Wa\x17\x9E\x81a\x17}V[\x91` \x82\x01Q\x91`@\x81\x01Q\x91`\x80``\x83\x01Q\x92\x01Q\x90V[\x91\x92``\x93\x81\x92\x84R`@` \x85\x01R\x81`@\x85\x01R\x84\x84\x017`\0\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[`@Q=`\0\x82>=\x90\xFD[`\0Th\x01\0\0\0\0\0\0\0\0\x81\x10\x15a\r\tW\x80`\x01a\x18\x16\x92\x01`\0Ua\x0C\tV[a\x18\xB2W\x81Q\x81T`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x82\x16\x17\x83U` \x84\x01Q`\x01\x84\x01\x80T\x91\x84\x16\x91\x83\x16\x91\x90\x91\x17\x90U`@\x84\x01Q`\x02\x84\x01\x80T\x91\x84\x16\x91\x83\x16\x91\x90\x91\x17\x90U``\x84\x01Q`\x03\x84\x01\x80T\x91\x84\x16\x91\x83\x16\x91\x90\x91\x17\x90U`\x80\x84\x01Q`\x04\x84\x01U`\xA0\x84\x01Q`\x05\x84\x01U`\xC0\x84\x01Q`\x06\x84\x01U`\xE0\x90\x93\x01Q`\x07\x90\x92\x01\x80T\x90\x93\x16\x91\x16\x17\x90UV[cNH{q`\xE0\x1B`\0R`\0`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x19\x81\x01\x91\x90\x82\x11a\x18\xEDWV[a\x18\xC8V[\x91\x90\x82\x03\x91\x82\x11a\x18\xEDWV[\x91\x92`d` \x92\x94`@Q\x95`\0\x95\x86\x94\x85\x93\x84\x93c#\xB8r\xDD`\xE0\x1B\x85R`\x04R`$R`DRZ\xF1=\x15`\x1F=\x11`\x01\x84Q\x14\x16\x17\x16\x90``R\x81`@R\x15a\x19GWPV[bF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rs\x15\x14\x90S\x94\xD1\x91T\x97\xD1\x94\x93\xD3W\xD1\x90RS\x11Q`b\x1B`D\x82\x01R`d\x90\xFD[`\x01`\xFF\x1B\x81\x14a\x19\x9BW`\0\x81\x12\x15a\x16 W\x19`\x01\x01\x90V[`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x90\xFD[\x90\x81` \x91\x03\x12a\x038Wa\x16 \x90a\x17}V[\x90\x91` \x90`D`@Q\x94`\0\x80\x95\x81\x94\x82\x93c\xA9\x05\x9C\xBB`\xE0\x1B\x84R`\x04R`$RZ\xF1=\x15`\x1F=\x11`\x01\x84Q\x14\x16\x17\x16\x90``R\x81`@R\x15a\x1A\x04WPV[bF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rn\x15\x14\x90S\x94\xD1\x91T\x97\xD1\x90RS\x11Q`\x8A\x1B`D\x82\x01R`d\x90\xFD[\x91\x90\x82`\xC0\x91\x03\x12a\x038Wa\x1AM\x82a\x17}V[\x91` \x81\x01Q\x91`@\x82\x01Q\x91``\x81\x01Q\x91`\xA0`\x80\x83\x01Q\x92\x01Q\x90V[\x15a\x1AtWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01Rk\x06\x96\xE7f\x16\xC6\x96B\x077v\x17`\xA4\x1B`D\x82\x01R`d\x90\xFD[\x90\x81` \x91\x03\x12a\x038WQ\x90V[\x91\x90\x82\x01\x80\x92\x11a\x18\xEDWV[\x92\x90a\x1A\xCF\x84a\x0C\tV[P\x91`\x04\x80\x93\x01T\x93`\x05a\x1A\xE3\x87a\x0C\tV[P\x01T\x93\x85\x84\x11\x91\x82\x15a\x1D\x8BWa\x1A\xFC\x81\x87\x11a\x1AmV[\x82\x15a\x1DrW`\x05a\x1Bfa\x1B\x15`\x02a\x03\xA0\x8Ca\x0C\tV[\x99[\x85\x15a\x1D^Wa\x1B+`\x03a\x03\xA0\x83a\x0C\tV[\x99[\x86\x15a\x1DMW\x84a\x1B>\x82\x8Ba\x18\xF2V[\x9A[\x8A\x89\x15a\x1D;WPa\x1BR\x92Pa\x18\xF2V[\x97[\x85a\x1B^\x83a\x0C\tV[P\x01Ua\x0C\tV[P\x01U`\x01`\x01`\xA0\x1B\x03\x87\x81\x16\x80;\x15a\x04\x12W`@\x80Qcp\xA0\x821`\xE0\x1B\x80\x82R0\x86\x83\x01\x90\x81R\x92\x94\x91\x93\x90\x92\x90\x91` \x91\x82\x90\x86\x90\x81\x90\x83\x01\x03\x81\x86Z\xFA\x94\x85\x15a\x04\rW`\0\x95a\x1D\x1CW[P\x8B\x16\x93\x84;\x15a\x04\x12W\x85Q\x84\x81R0\x88\x82\x01\x90\x81R\x90\x94\x90\x83\x90\x86\x90\x81\x90` \x01\x03\x81\x89Z\xFA\x94\x85\x15a\x04\rW`\0\x95a\x1C\xFDW[Pa\x1B\xFC\x8C03\x87a\x18\xFFV[a\x1C\x07\x8B3\x88a\x19\xC1V[\x83;\x15a\x04\x12W\x86Q\x81\x81R0\x89\x82\x01\x90\x81R\x90\x94\x84\x91\x86\x91\x90\x82\x90\x81\x90` \x01\x03\x91Z\xFA\x93\x84\x15a\x04\rW`\0\x94a\x1C\xDEW[P\x85;\x15a\x04\x12W\x86Q\x90\x81R0\x88\x82\x01\x90\x81R\x90\x95\x83\x91\x87\x91\x90\x82\x90\x81\x90` \x01\x03\x91Z\xFA\x94\x85\x15a\x04\rW\x8B\x92`\0\x96a\x1C\xB7W[PP\x90a\x1C~\x91a\x1A\xB7V[\x11a\x1C\xA8W\x86a\x1C\x8D\x91a\x18\xF2V[\x11a\x1C\x9BWPP\x94\x93\x92\x91\x90V[Qc\xF3\xCB\xBC\x87`\xE0\x1B\x81R\xFD[PPQc =\x90\x1D`\xE2\x1B\x81R\xFD[a\x1C~\x93\x92\x96P\x90\x81a\x1C\xD5\x92\x90=\x10a\t\x9FWa\t\x90\x81\x83a\r+V[\x94\x90\x918a\x1CrV[a\x1C\xF6\x91\x94P\x83=\x85\x11a\t\x9FWa\t\x90\x81\x83a\r+V[\x928a\x1C;V[a\x1D\x15\x91\x95P\x83=\x85\x11a\t\x9FWa\t\x90\x81\x83a\r+V[\x938a\x1B\xEFV[a\x1D4\x91\x95P\x82=\x84\x11a\t\x9FWa\t\x90\x81\x83a\r+V[\x938a\x1B\xB8V[\x91PPa\x1DG\x91a\x18\xF2V[\x97a\x1BTV[\x84a\x1DX\x8B\x82a\x18\xF2V[\x9Aa\x1B@V[a\x1Dl`\x02a\x03\xA0\x83a\x0C\tV[\x99a\x1B-V[`\x05a\x1Bfa\x1D\x85`\x03a\x03\xA0\x8Ca\x0C\tV[\x99a\x1B\x17V[a\x1D\x96\x85\x88\x11a\x1AmV[a\x1A\xFCV[\x92\x90a\x1D\xB1a\x03za\x03z`\x01a\x03\xA0\x88a\x0C\tV[\x92\x83;\x15a\x04\x12W`@Qb#\x8Bu`\xEA\x1B\x81R\x93`\xA0\x92\x85\x92\x83\x91\x82\x91a\x1D\xDD\x91\x8A`\x04\x85\x01a\x17\xB8V[\x03\x91Z\xFA\x90\x81\x15a\x04\rW`\0\x91\x82\x83\x80\x95\x81\x93a\x1EaW[P\x15a\x08\x0CWP`\x06a\x1E\\a\x1E\x19\x85`\x04a\x1E\x11\x8Aa\x0C\tV[P\x01Ta\x18\xF2V[\x96a\x1E)\x87`\x05a\x1E\x11\x84a\x0C\tV[\x96a\x1E8\x85\x85a\x1E\x11\x85a\x0C\tV[\x96a\x1EC\x88\x84a\x1F_V[`\x04a\x1EN\x84a\x0C\tV[P\x01U`\x05a\x1B^\x83a\x0C\tV[P\x01UV[\x93\x95PPPPa\x1E\x7F\x91P`\xA0=\x81\x11a\x08{Wa\x08e\x81\x83a\r+V[\x90\x94\x91\x93\x91\x92\x90\x918a\x1D\xF6V[a\x1E\xA1a\x03za\x03z`\x07a\x03\xA0\x85a\x0C\tV[\x91\x82;\x15a\x04\x12W`@Qc\x18\x16\r\xDD`\xE0\x1B\x81R` \x81`\x04\x81\x87Z\xFA\x80\x15a\x04\rWa\x1E\xEA\x93a\tZ\x92`\0\x92a\x1F8W[Pa\x1E\xE1`\x06\x91a\x0C\tV[P\x01T\x90a \x1DV[\x90\x80;\x15a\x04\x12W`@Qc@\xC1\x0F\x19`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x92\x90\x92R`\0\x90\x82\x90\x81\x83\x81`D\x81\x01[\x03\x92Z\xF1\x80\x15a\x04\rWa\x1F+WPV[\x80a\x04\x01a\rZ\x92a\x0C\xF5V[`\x06\x91\x92Pa\x1FWa\x1E\xE1\x91` =\x81\x11a\t\x9FWa\t\x90\x81\x83a\r+V[\x92\x91Pa\x1E\xD5V[a\x1Fsa\x03za\x03z`\x07a\x03\xA0\x85a\x0C\tV[\x91\x82;\x15a\x04\x12W`@Qc\x18\x16\r\xDD`\xE0\x1B\x81R` \x81`\x04\x81\x87Z\xFA\x80\x15a\x04\rWa\x1F\xC2\x93a\x1F\xBC\x92`\0\x92a\x1F\xF6W[Pa\x1F\xB3`\x06\x91a\x0C\tV[P\x01T\x90a `V[\x90a \x90V[\x90\x80;\x15a\x04\x12W`@Qc'p\xA7\xEB`\xE2\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x92\x90\x92R`\0\x90\x82\x90\x81\x83\x81`D\x81\x01a\x1F\x1AV[`\x06\x91\x92Pa \x15a\x1F\xB3\x91` =\x81\x11a\t\x9FWa\t\x90\x81\x83a\r+V[\x92\x91Pa\x1F\xA7V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17\x82\x15\x15\x16\x15a\x08\x93W\x04\x90V[\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x08\x93Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x04\x90V[\x90g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x81\x02\x92\x81\x84\x04\x14\x90\x15\x17\x81\x15\x15\x16\x15a\x08\x93W`\x01\x90`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x08\x93W`\x01g\r\xE0\xB6\xB3\xA7d\0\0`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[nZ\xF4=\x82\x80>\x90=\x91`+W\xFD[\xF3\x90v=`-\x80`\n=9\x81\xF36==7===6=s\0\0\0b\xFF\xFF\xFF\x82`\x88\x1C\x16\x17`\0R`x\x1B\x17` R`7`\t`\0\xF0\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x15a!\x12WV[`@Qc0\xBE\x1A=`\xE2\x1B\x81R`\x04\x90\xFD";
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
        ///Calls the contract's `multicall` (0xac9650d8) function
        pub fn multicall(
            &self,
            data: ::std::vec::Vec<::ethers::core::types::Bytes>,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Bytes>,
        > {
            self.0
                .method_hash([172, 150, 80, 216], data)
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
        ///Calls the contract's `updateController` (0x9fe1c16e) function
        pub fn update_controller(
            &self,
            pool_id: ::ethers::core::types::U256,
            new_controller: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([159, 225, 193, 110], (pool_id, new_controller))
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
        ///Gets the contract's `LogPoolStats` event
        pub fn log_pool_stats_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogPoolStatsFilter,
        > {
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
    ///Custom Error type `NotController` with signature `NotController()` and selector `0x23019e67`
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
    #[etherror(name = "NotController", abi = "NotController()")]
    pub struct NotController;
    ///Custom Error type `NotInitialized` with signature `NotInitialized()` and selector `0x87138d5c`
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
    #[etherror(name = "NotInitialized", abi = "NotInitialized()")]
    pub struct NotInitialized;
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
        InvalidSwapInputTransfer(InvalidSwapInputTransfer),
        InvalidSwapOutputTransfer(InvalidSwapOutputTransfer),
        InvalidTokens(InvalidTokens),
        Locked(Locked),
        Min(Min),
        NotController(NotController),
        NotInitialized(NotInitialized),
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
            if let Ok(decoded) = <NotController as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NotController(decoded));
            }
            if let Ok(decoded) = <NotInitialized as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NotInitialized(decoded));
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
                Self::NotController(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NotInitialized(element) => {
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
                    == <NotController as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NotInitialized as ::ethers::contract::EthError>::selector() => {
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
                Self::InvalidSwapInputTransfer(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidSwapOutputTransfer(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidTokens(element) => ::core::fmt::Display::fmt(element, f),
                Self::Locked(element) => ::core::fmt::Display::fmt(element, f),
                Self::Min(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotController(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotInitialized(element) => ::core::fmt::Display::fmt(element, f),
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
    impl ::core::convert::From<NotController> for DFMMErrors {
        fn from(value: NotController) -> Self {
            Self::NotController(value)
        }
    }
    impl ::core::convert::From<NotInitialized> for DFMMErrors {
        fn from(value: NotInitialized) -> Self {
            Self::NotInitialized(value)
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
    #[ethevent(
        name = "LogPoolStats",
        abi = "LogPoolStats(uint256,uint256,uint256,int256,uint256,uint256,uint256,uint256)"
    )]
    pub struct LogPoolStatsFilter {
        pub rx: ::ethers::core::types::U256,
        pub ry: ::ethers::core::types::U256,
        pub l: ::ethers::core::types::U256,
        pub invariant: ::ethers::core::types::I256,
        pub sigma: ::ethers::core::types::U256,
        pub strike: ::ethers::core::types::U256,
        pub tau: ::ethers::core::types::U256,
        pub timestamp: ::ethers::core::types::U256,
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
        LogPoolStatsFilter(LogPoolStatsFilter),
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
            if let Ok(decoded) = LogPoolStatsFilter::decode_log(log) {
                return Ok(DFMMEvents::LogPoolStatsFilter(decoded));
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
                Self::LogPoolStatsFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
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
    impl ::core::convert::From<LogPoolStatsFilter> for DFMMEvents {
        fn from(value: LogPoolStatsFilter) -> Self {
            Self::LogPoolStatsFilter(value)
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
    ///Container type for all input parameters for the `multicall` function with signature `multicall(bytes[])` and selector `0xac9650d8`
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
    #[ethcall(name = "multicall", abi = "multicall(bytes[])")]
    pub struct MulticallCall {
        pub data: ::std::vec::Vec<::ethers::core::types::Bytes>,
    }
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
    ///Container type for all input parameters for the `updateController` function with signature `updateController(uint256,address)` and selector `0x9fe1c16e`
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
    #[ethcall(name = "updateController", abi = "updateController(uint256,address)")]
    pub struct UpdateControllerCall {
        pub pool_id: ::ethers::core::types::U256,
        pub new_controller: ::ethers::core::types::Address,
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
        Multicall(MulticallCall),
        Nonce(NonceCall),
        Pools(PoolsCall),
        Swap(SwapCall),
        Update(UpdateCall),
        UpdateController(UpdateControllerCall),
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
            if let Ok(decoded) = <MulticallCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Multicall(decoded));
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
            if let Ok(decoded) = <UpdateControllerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpdateController(decoded));
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
                Self::Multicall(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Nonce(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Pools(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Swap(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Update(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UpdateController(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
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
                Self::Multicall(element) => ::core::fmt::Display::fmt(element, f),
                Self::Nonce(element) => ::core::fmt::Display::fmt(element, f),
                Self::Pools(element) => ::core::fmt::Display::fmt(element, f),
                Self::Swap(element) => ::core::fmt::Display::fmt(element, f),
                Self::Update(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateController(element) => ::core::fmt::Display::fmt(element, f),
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
    impl ::core::convert::From<MulticallCall> for DFMMCalls {
        fn from(value: MulticallCall) -> Self {
            Self::Multicall(value)
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
    impl ::core::convert::From<UpdateControllerCall> for DFMMCalls {
        fn from(value: UpdateControllerCall) -> Self {
            Self::UpdateController(value)
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
    ///Container type for all return fields from the `multicall` function with signature `multicall(bytes[])` and selector `0xac9650d8`
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
    pub struct MulticallReturn(pub ::std::vec::Vec<::ethers::core::types::Bytes>);
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
        pub controller: ::ethers::core::types::Address,
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
    ///`Pool(address,address,address,address,uint256,uint256,uint256,address)`
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
        pub controller: ::ethers::core::types::Address,
        pub strategy: ::ethers::core::types::Address,
        pub token_x: ::ethers::core::types::Address,
        pub token_y: ::ethers::core::types::Address,
        pub reserve_x: ::ethers::core::types::U256,
        pub reserve_y: ::ethers::core::types::U256,
        pub total_liquidity: ::ethers::core::types::U256,
        pub liquidity_token: ::ethers::core::types::Address,
    }
}
