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
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("strategy"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
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
    const __BYTECODE: &[u8] = b"`\xA0`@R`\x01\x80U4\x80\x15b\0\0bW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`@Qb\0\0q\x90b\0\x01[V[`@Q\x80\x91\x03\x90`\0\xF0\x80\x15\x80\x15b\0\0\x8EW=`\0\x80>=`\0\xFD[P`\x01`\x01`\xA0\x1B\x03\x16`\x80\x81\x90R`@\x80Qc&lE\xBB`\xE1\x1B\x81R`\x04\x81\x01\x91\x90\x91R`\0`D\x82\x01\x81\x90R```$\x83\x01R`d\x82\x01RcL\xD8\x8Bv\x90`\x84\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15b\0\x01<W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15b\0\x01QW=`\0\x80>=`\0\xFD[PPPPb\0\x01iV[a\x11<\x80b\0.i\x839\x01\x90V[`\x80Qa,\xDDb\0\x01\x8C`\09`\0\x81\x81a\x03V\x01Ra\x07\x95\x01Ra,\xDD`\0\xF3\xFE`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x01\x1CW`\x005`\xE0\x1C\x80c\x9F\xE1\xC1n\x11a\0\xD9W\x80c\xAF\xFE\xD0\xE0\x11a\0\xB3W\x80c\xAF\xFE\xD0\xE0\x14a\x03IW\x80c\xB4b\xCD%\x14a\x03QW\x80c\xBD\x06%\xAB\x14a\x03\x90W\x80c\xCE\x15;\xF4\x14a\x03\xB8Wa\x01\x1CV[\x80c\x9F\xE1\xC1n\x14a\x02\xB5W\x80c\xACJ\xFA8\x14a\x02\xC8W\x80c\xAC\x96P\xD8\x14a\x03)Wa\x01\x1CV[\x80c\x02\x16\xB88\x14a\x01\x81W\x80c\x06\x8B\xCD\x8D\x14a\x01\x96W\x80c\x14U\xF1\xFC\x14a\x02 W\x80c.\xC3\x81\x88\x14a\x02SW\x80c;\xE6\xA3A\x14a\x02\x81W\x80c\x9D\x94/\x9A\x14a\x02\xA2W[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x01\x94a\x01\x8F6`\x04a%\xA4V[a\x03\xCBV[\0[a\x01\xA9a\x01\xA46`\x04a&\x80V[a\x05\x10V[`@Qa\x02\x17\x91\x90\x81Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R` \x80\x84\x01Q\x82\x16\x90\x83\x01R`@\x80\x84\x01Q\x82\x16\x90\x83\x01R``\x80\x84\x01Q\x82\x16\x90\x83\x01R`\x80\x80\x84\x01Q\x90\x83\x01R`\xA0\x83\x81\x01Q\x90\x83\x01R`\xC0\x80\x84\x01Q\x90\x83\x01R`\xE0\x92\x83\x01Q\x16\x91\x81\x01\x91\x90\x91Ra\x01\0\x01\x90V[`@Q\x80\x91\x03\x90\xF3[a\x023a\x02.6`\x04a&\x9CV[a\x05\xE9V[`@\x80Q\x94\x85R` \x85\x01\x93\x90\x93R\x91\x83\x01R``\x82\x01R`\x80\x01a\x02\x17V[a\x02fa\x02a6`\x04a%\xA4V[a\x0CEV[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x02\x17V[a\x02\x94a\x02\x8F6`\x04a'DV[a\raV[`@Q\x90\x81R` \x01a\x02\x17V[a\x02fa\x02\xB06`\x04a%\xA4V[a\x0FMV[a\x01\x94a\x02\xC36`\x04a'qV[a\x12\x19V[a\x02\xDBa\x02\xD66`\x04a&\x80V[a\x12\xD8V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x99\x8A\x16\x81R\x97\x89\x16` \x89\x01R\x95\x88\x16\x95\x87\x01\x95\x90\x95R\x92\x86\x16``\x86\x01R`\x80\x85\x01\x91\x90\x91R`\xA0\x84\x01R`\xC0\x83\x01R\x90\x91\x16`\xE0\x82\x01Ra\x01\0\x01a\x02\x17V[a\x03<a\x0376`\x04a'\xE7V[a\x13AV[`@Qa\x02\x17\x91\x90a)\x82V[`\0Ta\x02\x94V[a\x03x\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\x17V[a\x03\xA3a\x03\x9E6`\x04a%\xA4V[a\x14cV[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x02\x17V[a\x02fa\x03\xC66`\x04a&\x80V[a\x16;V[`\x01T`\x02\x03a\x03\xEEW`@Qc\x03\xCB\x96\xDB`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x01U`\0\x80T\x84\x90\x81\x10a\x04\x07Wa\x04\x07a)\xFCV[`\0\x91\x82R` \x90\x91 `\x08\x90\x91\x02\x01T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x04@W`@Qc#\x01\x9Eg`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x83\x81T\x81\x10a\x04SWa\x04Sa)\xFCV[`\0\x91\x82R` \x90\x91 `\x01`\x08\x90\x92\x02\x01\x01T`@QbB\xD7\x07`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x02\x16\xB88\x90a\x04\x97\x90\x86\x90\x86\x90\x86\x90`\x04\x01a*\x12V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x04\xEFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a,\xBD\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x05\x03W=`\0\x80>=`\0\xFD[PP`\x01\x80UPPPPPV[`@\x80Qa\x01\0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R`\xC0\x81\x01\x82\x90R`\xE0\x81\x01\x91\x90\x91R`\0\x82\x81T\x81\x10a\x05dWa\x05da)\xFCV[`\0\x91\x82R` \x91\x82\x90 `@\x80Qa\x01\0\x81\x01\x82R`\x08\x90\x93\x02\x90\x91\x01\x80T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x84R`\x01\x82\x01T\x81\x16\x94\x84\x01\x94\x90\x94R`\x02\x81\x01T\x84\x16\x91\x83\x01\x91\x90\x91R`\x03\x81\x01T\x83\x16``\x83\x01R`\x04\x81\x01T`\x80\x83\x01R`\x05\x81\x01T`\xA0\x83\x01R`\x06\x81\x01T`\xC0\x83\x01R`\x07\x01T\x90\x91\x16`\xE0\x82\x01R\x92\x91PPV[`\0\x80`\0\x80`\x01T`\x02\x03a\x06\x12W`@Qc\x03\xCB\x96\xDB`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x01Ua\x06'``\x86\x01`@\x87\x01a*HV[`\x01`\x01`\xA0\x1B\x03\x16a\x06@`@\x87\x01` \x88\x01a*HV[`\x01`\x01`\xA0\x1B\x03\x16\x03a\x06gW`@Qc3\x91\n\xEF`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80\x80\x80\x80a\x06z` \x8B\x01\x8Ba*HV[`\0T`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90c\x9F\x83\x13{\x90a\x06\x9D``\x8E\x01\x8Ea*\xABV[`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x06\xBB\x93\x92\x91\x90a*\x12V[`\xA0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x07\x13W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a,\xBD\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x07'W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07K\x91\x90a+\x8CV[\x94P\x94P\x94P\x94P\x94P\x84a\x07\x8EW`\0\x84\x12a\x07g\x85a\x16\xBBV[`@Qcw`m)`\xE1\x1B\x81R\x91\x15\x15`\x04\x83\x01R`$\x82\x01R`D\x01[`@Q\x80\x91\x03\x90\xFD[`\0a\x07\xB9\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x16\xFAV[`@\x80Qc&lE\xBB`\xE1\x1B\x81R`\x04\x81\x01\x91\x90\x91R`\0`D\x82\x01\x81\x90R```$\x83\x01R`d\x82\x01R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x82\x16\x90cL\xD8\x8Bv\x90`\x84\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x08SW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a,\xBD\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x08gW=`\0\x80>=`\0\xFD[PP`@Qc@\xC1\x0F\x19`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x85\x90R`\x01`\x01`\xA0\x1B\x03\x84\x16\x92Pc@\xC1\x0F\x19\x91P`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x08\xF1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a,\xBD\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\t\x05W=`\0\x80>=`\0\xFD[PP`@\x80Qa\x01\0\x81\x01\x90\x91R3\x81R`\0\x92P\x90P` \x80\x82\x01\x90a\t.\x90\x8F\x01\x8Fa*HV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8D` \x01` \x81\x01\x90a\tO\x91\x90a*HV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8D`@\x01` \x81\x01\x90a\tp\x91\x90a*HV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R` \x80\x83\x01\x89\x90R`@\x80\x84\x01\x89\x90R``\x80\x85\x01\x89\x90R\x87\x84\x16`\x80\x95\x86\x01R`\0\x80T`\x01\x80\x82\x01\x83U\x82\x80R\x88Q\x7F)\r\xEC\xD9T\x8Bb\xA8\xD6\x03E\xA9\x888o\xC8K\xA6\xBC\x95H@\x08\xF66/\x93\x16\x0E\xF3\xE5c`\x08\x90\x93\x02\x92\x83\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16\x92\x8A\x16\x92\x90\x92\x17\x90U\x95\x89\x01Q\x7F)\r\xEC\xD9T\x8Bb\xA8\xD6\x03E\xA9\x888o\xC8K\xA6\xBC\x95H@\x08\xF66/\x93\x16\x0E\xF3\xE5d\x83\x01\x80T\x88\x16\x91\x89\x16\x91\x90\x91\x17\x90U\x93\x88\x01Q\x7F)\r\xEC\xD9T\x8Bb\xA8\xD6\x03E\xA9\x888o\xC8K\xA6\xBC\x95H@\x08\xF66/\x93\x16\x0E\xF3\xE5e\x82\x01\x80T\x87\x16\x91\x88\x16\x91\x90\x91\x17\x90U\x91\x87\x01Q\x7F)\r\xEC\xD9T\x8Bb\xA8\xD6\x03E\xA9\x888o\xC8K\xA6\xBC\x95H@\x08\xF66/\x93\x16\x0E\xF3\xE5f\x83\x01\x80T\x86\x16\x91\x87\x16\x91\x90\x91\x17\x90U\x94\x86\x01Q\x7F)\r\xEC\xD9T\x8Bb\xA8\xD6\x03E\xA9\x888o\xC8K\xA6\xBC\x95H@\x08\xF66/\x93\x16\x0E\xF3\xE5g\x82\x01U`\xA0\x86\x01Q\x7F)\r\xEC\xD9T\x8Bb\xA8\xD6\x03E\xA9\x888o\xC8K\xA6\xBC\x95H@\x08\xF66/\x93\x16\x0E\xF3\xE5h\x82\x01U`\xC0\x86\x01Q\x7F)\r\xEC\xD9T\x8Bb\xA8\xD6\x03E\xA9\x888o\xC8K\xA6\xBC\x95H@\x08\xF66/\x93\x16\x0E\xF3\xE5i\x82\x01U`\xE0\x86\x01Q\x7F)\r\xEC\xD9T\x8Bb\xA8\xD6\x03E\xA9\x888o\xC8K\xA6\xBC\x95H@\x08\xF66/\x93\x16\x0E\xF3\xE5j\x90\x91\x01\x80T\x90\x93\x16\x93\x16\x92\x90\x92\x17\x90U\x81T\x92\x93P\x90\x91a\x0BK\x91\x90a+\xECV[\x90Pa\x0Bk\x8D` \x01` \x81\x01\x90a\x0Bc\x91\x90a*HV[30\x89a\x17gV[a\x0B\x89\x8D`@\x01` \x81\x01\x90a\x0B\x81\x91\x90a*HV[30\x88a\x17gV[a\x0B\x99``\x8E\x01`@\x8F\x01a*HV[`\x01`\x01`\xA0\x1B\x03\x16\x8D` \x01` \x81\x01\x90a\x0B\xB5\x91\x90a*HV[`\x01`\x01`\xA0\x1B\x03\x16\x8E`\0\x01` \x81\x01\x90a\x0B\xD1\x91\x90a*HV[`@\x80Q3\x81R` \x81\x01\x86\x90R\x90\x81\x01\x8A\x90R``\x81\x01\x89\x90R`\x80\x81\x01\x88\x90R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90\x7F\xF7\xC0\x95\xAF\xDDB\r*7\x8A\x88r\x05\xCFW\xEB\xEE/m\xD0\x07\xDD\xD7c\xC8Z\xEA\xC0Z\xE3\xD7\xCA\x90`\xA0\x01`@Q\x80\x91\x03\x90\xA4`\x01\x80U\x9C\x94\x9BP\x92\x99P\x90\x97P\x91\x95PPPPPPV[`\0\x80`\0`\x01T`\x02\x03a\x0CmW`@Qc\x03\xCB\x96\xDB`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x01\x81\x90UP`\0\x80`\0a\x0C\x87\x89`\x01\x8A\x8Aa\x17\xF5V[\x92P\x92P\x92Pa\x0C\xC8`\0\x8A\x81T\x81\x10a\x0C\xA3Wa\x0C\xA3a)\xFCV[`\0\x91\x82R` \x90\x91 `\x02`\x08\x90\x92\x02\x01\x01T`\x01`\x01`\xA0\x1B\x03\x1630\x86a\x17gV[a\r\x03`\0\x8A\x81T\x81\x10a\x0C\xDEWa\x0C\xDEa)\xFCV[`\0\x91\x82R` \x90\x91 `\x03`\x08\x90\x92\x02\x01\x01T`\x01`\x01`\xA0\x1B\x03\x1630\x85a\x17gV[`@\x80Q\x8A\x81R` \x81\x01\x85\x90R\x90\x81\x01\x83\x90R``\x81\x01\x82\x90R3\x90\x7F\x95\x97W\x7F3\x93 w^c\xD3\xFE\xD7\xD5\xDD\xE66[\xAD\xCC\x9F\xCC\xDBf\xB3H\x94c\x0C\xA9\x8Bo\x90`\x80\x01[`@Q\x80\x91\x03\x90\xA2`\x01\x80U\x91\x98\x90\x97P\x90\x95P\x93PPPPV[`\0\x80`\0\x83\x81T\x81\x10a\rwWa\rwa)\xFCV[`\0\x91\x82R` \x82 `\x08\x91\x90\x91\x02\x01`\x07\x01T`@Qcp\xA0\x821`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x87\x81\x16`\x04\x83\x01R\x90\x91\x16\x92P\x82\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0E\x0EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a,\xBD\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0E\"W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0EF\x91\x90a+\xFFV[\x90P`\0\x82`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0E\xC1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a,\xBD\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0E\xD5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\xF9\x91\x90a+\xFFV[\x90P`\0\x80\x86\x81T\x81\x10a\x0F\x0FWa\x0F\x0Fa)\xFCV[\x90`\0R` `\0 \x90`\x08\x02\x01`\x06\x01T\x90Pa\x0F@a\x0F9\x83\x83a\x1A\xDB\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x84\x90a\x1A\xF7V[\x94PPPPP[\x92\x91PPV[`\0\x80`\0`\x01T`\x02\x03a\x0FuW`@Qc\x03\xCB\x96\xDB`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x01U`\0\x80\x80a\x0F\x8A\x89\x82\x8A\x8Aa\x17\xF5V[\x92P\x92P\x92P`\0\x89\x81T\x81\x10a\x0F\xA3Wa\x0F\xA3a)\xFCV[`\0\x91\x82R` \x90\x91 `\x08\x90\x91\x02\x01`\x02\x01T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x85\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x10>W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a,\xBD\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x10RW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10v\x91\x90a,\x1BV[P`\0\x89\x81T\x81\x10a\x10\x8AWa\x10\x8Aa)\xFCV[`\0\x91\x82R` \x90\x91 `\x08\x90\x91\x02\x01`\x03\x01T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x11%W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a,\xBD\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x119W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11]\x91\x90a,\x1BV[Pa\x11\x98`\0\x8A\x81T\x81\x10a\x11tWa\x11ta)\xFCV[`\0\x91\x82R` \x90\x91 `\x02`\x08\x90\x92\x02\x01\x01T`\x01`\x01`\xA0\x1B\x03\x163\x85a\x1B\x0CV[a\x11\xD2`\0\x8A\x81T\x81\x10a\x11\xAEWa\x11\xAEa)\xFCV[`\0\x91\x82R` \x90\x91 `\x03`\x08\x90\x92\x02\x01\x01T`\x01`\x01`\xA0\x1B\x03\x163\x84a\x1B\x0CV[`@\x80Q\x8A\x81R` \x81\x01\x85\x90R\x90\x81\x01\x83\x90R``\x81\x01\x82\x90R3\x90\x7F\xAC\xBE\x12~\x93\xA8\xA0\xB2x\xD8\xE0n' [=\xF9\xD1\xF3\x81$\x14\xBC\x89\x17\xC7t\xA87\x101n\x90`\x80\x01a\rFV[`\x01T`\x02\x03a\x12<W`@Qc\x03\xCB\x96\xDB`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x01U`\0\x80T\x83\x90\x81\x10a\x12UWa\x12Ua)\xFCV[`\0\x91\x82R` \x90\x91 `\x08\x90\x91\x02\x01T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x12\x8EW`@Qc#\x01\x9Eg`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80`\0\x83\x81T\x81\x10a\x12\xA2Wa\x12\xA2a)\xFCV[`\0\x91\x82R` \x90\x91 `\x08\x90\x91\x02\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UPP`\x01\x80UV[`\0\x81\x81T\x81\x10a\x12\xE8W`\0\x80\xFD[`\0\x91\x82R` \x90\x91 `\x08\x90\x91\x02\x01\x80T`\x01\x82\x01T`\x02\x83\x01T`\x03\x84\x01T`\x04\x85\x01T`\x05\x86\x01T`\x06\x87\x01T`\x07\x90\x97\x01T`\x01`\x01`\xA0\x1B\x03\x96\x87\x16\x98P\x94\x86\x16\x96\x93\x86\x16\x95\x92\x83\x16\x94\x91\x93\x90\x92\x90\x91\x16\x88V[```\0\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x13_Wa\x13_a'\xA0V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x13\x92W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x13}W\x90P[P\x90P`\0[\x83Q\x81\x03a\x14\\W`\0\x800`\x01`\x01`\xA0\x1B\x03\x16\x86\x84\x81Q\x81\x10a\x13\xBFWa\x13\xBFa)\xFCV[` \x02` \x01\x01Q`@Qa\x13\xD4\x91\x90a,9V[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14a\x14\x0FW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x14\x14V[``\x91P[P\x91P\x91P\x81a\x144W\x80Q`\0\x03a\x14,W`\0\x80\xFD[\x80Q\x81` \x01\xFD[\x80\x84\x84\x81Q\x81\x10a\x14GWa\x14Ga)\xFCV[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x01a\x13\x98V[P\x92\x91PPV[`\0\x80`\x01T`\x02\x03a\x14\x89W`@Qc\x03\xCB\x96\xDB`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x01\x81\x90UP`\0\x80`\0\x80`\0\x80\x8A\x81T\x81\x10a\x14\xABWa\x14\xABa)\xFCV[`\0\x91\x82R` \x90\x91 `\x01`\x08\x90\x92\x02\x01\x01T`@Qc2\x14\x89\x0F`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c2\x14\x89\x0F\x90a\x14\xF0\x90\x8D\x90\x8D\x90\x8D\x90`\x04\x01a*\x12V[`\xC0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x15FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a,\xBD\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x15ZW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15~\x91\x90a,UV[\x95P\x95P\x95PP\x94P\x94P\x84a\x15\x9BW`\0\x84\x12a\x07g\x85a\x16\xBBV[\x80`\0\x8B\x81T\x81\x10a\x15\xAFWa\x15\xAFa)\xFCV[\x90`\0R` `\0 \x90`\x08\x02\x01`\x06\x01\x81\x90UP`\0\x80`\0a\x15\xD4\x8D\x87\x87a\x1B\x90V[`@\x80Q\x86\x15\x15\x81R` \x81\x01\x84\x90R\x90\x81\x01\x82\x90R\x94\x97P\x90\x95P\x93P\x8F\x923\x92P\x7FL}\xEF\x84\xE4++\xC0\xA5\xAA\xB2\"\x86\x8D\xD7\xA0\x92\xB53w\xA4\xB57\xAB\xCD\x944Zz\x85'\xED\x91P``\x01`@Q\x80\x91\x03\x90\xA3`\x01\x80U\x90\x9C\x90\x9BP\x99PPPPPPPPPPV[`\0\x80`\0\x80\x84\x81T\x81\x10a\x16RWa\x16Ra)\xFCV[\x90`\0R` `\0 \x90`\x08\x02\x01`\x04\x01T`\0\x85\x81T\x81\x10a\x16wWa\x16wa)\xFCV[\x90`\0R` `\0 \x90`\x08\x02\x01`\x05\x01T`\0\x86\x81T\x81\x10a\x16\x9CWa\x16\x9Ca)\xFCV[\x90`\0R` `\0 \x90`\x08\x02\x01`\x06\x01T\x92P\x92P\x92P\x91\x93\x90\x92PV[`\0`\x01`\xFF\x1B\x82\x03a\x16\xE1W`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x12\x15a\x16\xF2WP\x19`\x01\x01\x90V[P\x80[\x91\x90PV[`\0v=`-\x80`\n=9\x81\xF36==7===6=s\0\0\0\x82``\x1B`\xE8\x1C\x17`\0RnZ\xF4=\x82\x80>\x90=\x91`+W\xFD[\xF3\x82`x\x1B\x17` R`7`\t`\0\xF0\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16a\x16\xF5W`@Qc0\xBE\x1A=`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0`@Qc#\xB8r\xDD`\xE0\x1B`\0R\x84`\x04R\x83`$R\x82`DR` `\0`d`\0\x80\x8AZ\xF1=\x15`\x1F=\x11`\x01`\0Q\x14\x16\x17\x16\x91P`\0``R\x80`@RP\x80a\x17\xEEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rs\x15\x14\x90S\x94\xD1\x91T\x97\xD1\x94\x93\xD3W\xD1\x90RS\x11Q`b\x1B`D\x82\x01R`d\x01a\x07\x85V[PPPPPV[`\0\x80`\0\x80`\0\x80`\0\x80`\0\x8C\x81T\x81\x10a\x18\x14Wa\x18\x14a)\xFCV[`\0\x91\x82R` \x90\x91 `\x01`\x08\x90\x92\x02\x01\x01T`@Qb#\x8Bu`\xEA\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x8E-\xD4\0\x90a\x18X\x90\x8F\x90\x8E\x90\x8E\x90`\x04\x01a*\x12V[`\xA0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x18\xAEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a,\xBD\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x18\xC2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18\xE6\x91\x90a+\x8CV[\x94P\x94P\x94P\x94P\x94P\x84a\x19\x02W`\0\x84\x12a\x07g\x85a\x16\xBBV[\x8Aa\x19<W\x82`\0\x8D\x81T\x81\x10a\x19\x1BWa\x19\x1Ba)\xFCV[\x90`\0R` `\0 \x90`\x08\x02\x01`\x04\x01Ta\x197\x91\x90a+\xECV[a\x19lV[`\0\x8C\x81T\x81\x10a\x19OWa\x19Oa)\xFCV[\x90`\0R` `\0 \x90`\x08\x02\x01`\x04\x01T\x83a\x19l\x91\x90a+\xECV[\x97P\x8Aa\x19\xA8W\x81`\0\x8D\x81T\x81\x10a\x19\x87Wa\x19\x87a)\xFCV[\x90`\0R` `\0 \x90`\x08\x02\x01`\x05\x01Ta\x19\xA3\x91\x90a+\xECV[a\x19\xD8V[`\0\x8C\x81T\x81\x10a\x19\xBBWa\x19\xBBa)\xFCV[\x90`\0R` `\0 \x90`\x08\x02\x01`\x05\x01T\x82a\x19\xD8\x91\x90a+\xECV[\x96P\x8Aa\x1A\x14W\x80`\0\x8D\x81T\x81\x10a\x19\xF3Wa\x19\xF3a)\xFCV[\x90`\0R` `\0 \x90`\x08\x02\x01`\x06\x01Ta\x1A\x0F\x91\x90a+\xECV[a\x1ADV[`\0\x8C\x81T\x81\x10a\x1A'Wa\x1A'a)\xFCV[\x90`\0R` `\0 \x90`\x08\x02\x01`\x06\x01T\x81a\x1AD\x91\x90a+\xECV[\x95Pa\x1AQ\x8C\x8C\x88a!EV[\x82`\0\x8D\x81T\x81\x10a\x1AeWa\x1Aea)\xFCV[\x90`\0R` `\0 \x90`\x08\x02\x01`\x04\x01\x81\x90UP\x81`\0\x8D\x81T\x81\x10a\x1A\x8EWa\x1A\x8Ea)\xFCV[\x90`\0R` `\0 \x90`\x08\x02\x01`\x05\x01\x81\x90UP\x80`\0\x8D\x81T\x81\x10a\x1A\xB7Wa\x1A\xB7a)\xFCV[\x90`\0R` `\0 \x90`\x08\x02\x01`\x06\x01\x81\x90UPPPPPP\x94P\x94P\x94\x91PPV[`\0a\x1A\xF0\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a#\xDFV[\x93\x92PPPV[`\0a\x1A\xF0\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a#\xDFV[`\0`@Qc\xA9\x05\x9C\xBB`\xE0\x1B`\0R\x83`\x04R\x82`$R` `\0`D`\0\x80\x89Z\xF1=\x15`\x1F=\x11`\x01`\0Q\x14\x16\x17\x16\x91P`\0``R\x80`@RP\x80a\x1B\x8AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rn\x15\x14\x90S\x94\xD1\x91T\x97\xD1\x90RS\x11Q`\x8A\x1B`D\x82\x01R`d\x01a\x07\x85V[PPPPV[`\0\x80`\0\x80`\0\x80`\0\x89\x81T\x81\x10a\x1B\xACWa\x1B\xACa)\xFCV[\x90`\0R` `\0 \x90`\x08\x02\x01`\x04\x01T\x90P`\0\x80\x8A\x81T\x81\x10a\x1B\xD4Wa\x1B\xD4a)\xFCV[\x90`\0R` `\0 \x90`\x08\x02\x01`\x05\x01T\x90P\x81\x89\x11\x96P\x86\x15a\x1C6W\x87\x81\x11a\x1C1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01Rk\x06\x96\xE7f\x16\xC6\x96B\x077v\x17`\xA4\x1B`D\x82\x01R`d\x01a\x07\x85V[a\x1CtV[\x88\x82\x11a\x1CtW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01Rk\x06\x96\xE7f\x16\xC6\x96B\x077v\x17`\xA4\x1B`D\x82\x01R`d\x01a\x07\x85V[\x86a\x1C\xAEW`\0\x8A\x81T\x81\x10a\x1C\x8CWa\x1C\x8Ca)\xFCV[`\0\x91\x82R` \x90\x91 `\x03`\x08\x90\x92\x02\x01\x01T`\x01`\x01`\xA0\x1B\x03\x16a\x1C\xDFV[`\0\x8A\x81T\x81\x10a\x1C\xC1Wa\x1C\xC1a)\xFCV[`\0\x91\x82R` \x90\x91 `\x02`\x08\x90\x92\x02\x01\x01T`\x01`\x01`\xA0\x1B\x03\x16[\x95P\x86a\x1D\x1BW`\0\x8A\x81T\x81\x10a\x1C\xF9Wa\x1C\xF9a)\xFCV[`\0\x91\x82R` \x90\x91 `\x02`\x08\x90\x92\x02\x01\x01T`\x01`\x01`\xA0\x1B\x03\x16a\x1DLV[`\0\x8A\x81T\x81\x10a\x1D.Wa\x1D.a)\xFCV[`\0\x91\x82R` \x90\x91 `\x03`\x08\x90\x92\x02\x01\x01T`\x01`\x01`\xA0\x1B\x03\x16[\x94P\x86a\x1DbWa\x1D]\x81\x89a+\xECV[a\x1DlV[a\x1Dl\x82\x8Aa+\xECV[\x93P\x86a\x1D\x82Wa\x1D}\x89\x83a+\xECV[a\x1D\x8CV[a\x1D\x8C\x88\x82a+\xECV[\x92P\x88`\0\x8B\x81T\x81\x10a\x1D\xA2Wa\x1D\xA2a)\xFCV[\x90`\0R` `\0 \x90`\x08\x02\x01`\x04\x01\x81\x90UP\x87`\0\x8B\x81T\x81\x10a\x1D\xCBWa\x1D\xCBa)\xFCV[`\0\x91\x82R` \x82 `\x08\x91\x90\x91\x02\x01`\x05\x01\x91\x90\x91U`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x88\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x1E_W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a,\xBD\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x1EsW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E\x97\x91\x90a+\xFFV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x88\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x1F\x1AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a,\xBD\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x1F.W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1FR\x91\x90a+\xFFV[\x90Pa\x1F`\x8830\x89a\x17gV[a\x1Fk\x873\x87a\x1B\x0CV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x90`\x01`\x01`\xA0\x1B\x03\x8A\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x1F\xEBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a,\xBD\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x1F\xFFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a #\x91\x90a+\xFFV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x8A\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a \xA6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a,\xBD\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a \xBAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a \xDE\x91\x90a+\xFFV[\x90Pa \xEA\x88\x85a,\xA9V[\x82\x10\x15a!\nW`@Qc =\x90\x1D`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a!\x14\x87\x84a+\xECV[\x81\x10\x15a!4W`@Qc\xF3\xCB\xBC\x87`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPP\x93\x97\x92\x96P\x93P\x93PV[`\0\x80\x84\x81T\x81\x10a!YWa!Ya)\xFCV[\x90`\0R` `\0 \x90`\x08\x02\x01`\x07\x01`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x90P`\0\x81`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a!\xF9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a,\xBD\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\"\rW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\"1\x91\x90a+\xFFV[\x90P`\0\x80\x86\x81T\x81\x10a\"GWa\"Ga)\xFCV[\x90`\0R` `\0 \x90`\x08\x02\x01`\x06\x01T\x90P\x84\x15a#\x1EW`\0a\"wa\"p\x84\x84a\x1A\xDBV[\x86\x90a\x1A\xF7V[`@Qc@\xC1\x0F\x19`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x82\x90R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x85\x16\x90c@\xC1\x0F\x19\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a#\0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a,\xBD\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a#\x14W=`\0\x80>=`\0\xFD[PPPPPa#\xD7V[`\0a#4a#-\x84\x84a#\xFEV[\x86\x90a$\x13V[`@Qc'p\xA7\xEB`\xE2\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x82\x90R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x85\x16\x90c\x9D\xC2\x9F\xAC\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a#\xBDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a,\xBD\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a#\xD1W=`\0\x80>=`\0\xFD[PPPPP[PPPPPPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a#\xF7W`\0\x80\xFD[\x04\x92\x91PPV[`\0a\x1A\xF0\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a$$V[`\0a\x1A\xF0\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a$<W`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x81\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rjrray stride`\xA8\x1B`d\x82\x01R`\x84\x81\xFD[`\0\x80`\0`@\x84\x86\x03\x12\x15a%\xBCWa%\xBCa$RV[\x835\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a%\xDEWa%\xDEa$\xA2V[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a%\xF5Wa%\xF5a$\xF2V[\x815\x81\x81\x11\x15a&XW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x0ENL/$\r\x8C\xAD\xCC\xEE\x8D`\xAB\x1B`d\x82\x01R`\x84\x81\xFD[\x87` \x82\x85\x01\x01\x11\x15a&mWa&ma%KV[` \x83\x01\x94P\x80\x93PPPP\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a&\x95Wa&\x95a$RV[P5\x91\x90PV[`\0` \x82\x84\x03\x12\x15a&\xB1Wa&\xB1a$RV[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a&\xCBWa&\xCBa$\xA2V[\x82\x01`\x80\x81\x85\x03\x12\x15a\x1A\xF0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FABI decoding: struct calldata to`D\x82\x01Rf\x1B\xC8\x1C\xDA\x1B\xDC\x9D`\xCA\x1B`d\x82\x01R`\x84\x81\xFD[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x16\xF5W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a'ZWa'Za$RV[a'c\x83a'-V[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`@\x83\x85\x03\x12\x15a'\x87Wa'\x87a$RV[\x825\x91Pa'\x97` \x84\x01a'-V[\x90P\x92P\x92\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a'\xDFWa'\xDFa'\xA0V[`@R\x91\x90PV[`\0` \x80\x83\x85\x03\x12\x15a'\xFDWa'\xFDa$RV[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a(\x18Wa(\x18a$\xA2V[\x81\x85\x01\x91P`\x1F\x86\x81\x84\x01\x12a(0Wa(0a$\xF2V[\x825\x82\x81\x11\x15a(BWa(Ba'\xA0V[\x80`\x05\x1Ba(Q\x86\x82\x01a'\xB6V[\x91\x82R\x84\x81\x01\x86\x01\x91\x86\x81\x01\x90\x8A\x84\x11\x15a(nWa(na%KV[\x87\x87\x01\x92P[\x83\x83\x10\x15a)PW\x825\x86\x81\x11\x15a(\x8EWa(\x8Ea$\xF2V[\x87\x01`?\x81\x01\x8C\x13a(\xA2Wa(\xA2a$\xF2V[\x88\x81\x015`@\x88\x82\x11\x15a(\xB8Wa(\xB8a'\xA0V[a(\xC9\x82\x89\x01`\x1F\x19\x16\x8C\x01a'\xB6V[\x82\x81R\x8E\x82\x84\x86\x01\x01\x11\x15a),W\x81QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x8D\x90R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R`\x84\x81\xFD[\x82\x82\x85\x01\x8D\x83\x017`\0\x92\x81\x01\x8C\x01\x92\x90\x92RP\x83RP\x91\x87\x01\x91\x90\x87\x01\x90a(tV[\x9A\x99PPPPPPPPPPV[`\0[\x83\x81\x10\x15a)yW\x81\x81\x01Q\x83\x82\x01R` \x01a)aV[PP`\0\x91\x01RV[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x86\x01\x91P`@\x81`\x05\x1B\x87\x01\x01\x92P\x83\x87\x01`\0[\x82\x81\x10\x15a)\xEFW\x87\x85\x03`?\x19\x01\x84R\x81Q\x80Q\x80\x87Ra)\xD0\x81\x89\x89\x01\x8A\x85\x01a)^V[`\x1F\x01`\x1F\x19\x16\x95\x90\x95\x01\x86\x01\x94P\x92\x85\x01\x92\x90\x85\x01\x90`\x01\x01a)\xA9V[P\x92\x97\x96PPPPPPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[\x83\x81R`@` \x82\x01R\x81`@\x82\x01R\x81\x83``\x83\x017`\0\x81\x83\x01``\x90\x81\x01\x91\x90\x91R`\x1F\x90\x92\x01`\x1F\x19\x16\x01\x01\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a*]Wa*]a$RV[a\x1A\xF0\x82a'-V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FCalldata tail too short\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x81\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a+\x02W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FInvalid calldata tail offset\0\0\0\0`D\x82\x01R`d\x81\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a+]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FInvalid calldata tail length\0\0\0\0`D\x82\x01R`d\x81\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a+uWa+ua*fV[\x92P\x92\x90PV[\x80Q\x80\x15\x15\x81\x14a\x16\xF5W`\0\x80\xFD[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a+\xA7Wa+\xA7a$RV[a+\xB0\x86a+|V[` \x87\x01Q`@\x88\x01Q``\x89\x01Q`\x80\x90\x99\x01Q\x92\x9A\x91\x99P\x97\x96P\x90\x94P\x92PPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x0FGWa\x0FGa+\xD6V[`\0` \x82\x84\x03\x12\x15a,\x14Wa,\x14a$RV[PQ\x91\x90PV[`\0` \x82\x84\x03\x12\x15a,0Wa,0a$RV[a\x1A\xF0\x82a+|V[`\0\x82Qa,K\x81\x84` \x87\x01a)^V[\x91\x90\x91\x01\x92\x91PPV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a,qWa,qa$RV[a,z\x87a+|V[\x95P` \x87\x01Q\x94P`@\x87\x01Q\x93P``\x87\x01Q\x92P`\x80\x87\x01Q\x91P`\xA0\x87\x01Q\x90P\x92\x95P\x92\x95P\x92\x95V[\x80\x82\x01\x80\x82\x11\x15a\x0FGWa\x0FGa+\xD6V\xFETarget contract does not contain`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[Pa\x10\xCF\x80a\0m`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x01XW`\x005`\xE0\x1C\x80cL\xD8\x8Bv\x11a\0\xEFW\x80c\x9D\xC2\x9F\xAC\x11a\0\xBEW\x80c\x9D\xC2\x9F\xAC\x14a\x02\xCEW\x80c\xA9\x05\x9C\xBB\x14a\x02\xE1W\x80c\xAF\xBA\x13\xC4\x14a\x02\xF4W\x80c\xD5\x05\xAC\xCF\x14a\x03\x1FW\x80c\xDDb\xED>\x14a\x032Wa\x01XV[\x80cL\xD8\x8Bv\x14a\x02sW\x80cp\xA0\x821\x14a\x02\x86W\x80c~\xCE\xBE\0\x14a\x02\xA6W\x80c\x95\xD8\x9BA\x14a\x02\xC6Wa\x01XV[\x80c#\xB8r\xDD\x11a\x01+W\x80c#\xB8r\xDD\x14a\x02)W\x80c1<\xE5g\x14a\x02<W\x80c6D\xE5\x15\x14a\x02VW\x80c@\xC1\x0F\x19\x14a\x02^Wa\x01XV[\x80c\x06\xFD\xDE\x03\x14a\x01\xBDW\x80c\t^\xA7\xB3\x14a\x01\xDBW\x80c\x15\x8E\xF9>\x14a\x01\xFEW\x80c\x18\x16\r\xDD\x14a\x02\x12W[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x01\xC5a\x03]V[`@Qa\x01\xD2\x91\x90a\n\xAFV[`@Q\x80\x91\x03\x90\xF3[a\x01\xEEa\x01\xE96`\x04a\x0B\xB9V[a\x03\xEBV[`@Q\x90\x15\x15\x81R` \x01a\x01\xD2V[`\x08Ta\x01\xEE\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16\x81V[a\x02\x1B`\x02T\x81V[`@Q\x90\x81R` \x01a\x01\xD2V[a\x01\xEEa\x0276`\x04a\x0B\xE6V[a\x04XV[a\x02D`\x12\x81V[`@Q`\xFF\x90\x91\x16\x81R` \x01a\x01\xD2V[a\x02\x1Ba\x058V[a\x02qa\x02l6`\x04a\x0B\xB9V[a\x05WV[\0[a\x02qa\x02\x816`\x04a\ruV[a\x05\xB0V[a\x02\x1Ba\x02\x946`\x04a\r\xE2V[`\x03` R`\0\x90\x81R`@\x90 T\x81V[a\x02\x1Ba\x02\xB46`\x04a\r\xE2V[`\x07` R`\0\x90\x81R`@\x90 T\x81V[a\x01\xC5a\x06RV[a\x02qa\x02\xDC6`\x04a\x0B\xB9V[a\x06_V[a\x01\xEEa\x02\xEF6`\x04a\x0B\xB9V[a\x06\xAFV[`\x08Ta\x03\x07\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\xD2V[a\x02qa\x03-6`\x04a\x0E\x07V[a\x07\x15V[a\x02\x1Ba\x03@6`\x04a\x0E}V[`\x04` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[`\0\x80Ta\x03j\x90a\x0E\xB3V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x03\x96\x90a\x0E\xB3V[\x80\x15a\x03\xE3W\x80`\x1F\x10a\x03\xB8Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x03\xE3V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x03\xC6W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[3`\0\x81\x81R`\x04` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x87\x16\x80\x85R\x92R\x80\x83 \x85\x90UQ\x91\x92\x90\x91\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x90a\x04F\x90\x86\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3P`\x01[\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x04` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x81 T`\0\x19\x81\x14a\x04\xB4Wa\x04\x8F\x83\x82a\x0F\x03V[`\x01`\x01`\xA0\x1B\x03\x86\x16`\0\x90\x81R`\x04` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x90 U[`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R`\x03` R`@\x81 \x80T\x85\x92\x90a\x04\xDC\x90\x84\x90a\x0F\x03V[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\0\x81\x81R`\x03` R`@\x90\x81\x90 \x80T\x87\x01\x90UQ\x90\x91\x87\x16\x90`\0\x80Q` a\x10\xAF\x839\x81Q\x91R\x90a\x05%\x90\x87\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3P`\x01\x94\x93PPPPV[`\0`\x05TF\x14a\x05PWa\x05Ka\tYV[\x90P\x90V[P`\x06T\x90V[`\x08T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x05\xA2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01RhOnly DFMM`\xB8\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[a\x05\xAC\x82\x82a\t\xF3V[PPV[`\x08T`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15a\x06\0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01Rr\x10[\x1C\x99XY\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`j\x1B`D\x82\x01R`d\x01a\x05\x99V[`\x08\x80T`\x01`\x01`\xA0\x1B\x03\x19\x163\x17\x90U`\0a\x06\x1E\x83\x82a\x0FeV[P`\x01a\x06+\x82\x82a\x0FeV[PF`\x05Ua\x068a\tYV[`\x06UPP`\x08\x80T`\xFF`\xA0\x1B\x19\x16`\x01`\xA0\x1B\x17\x90UV[`\x01\x80Ta\x03j\x90a\x0E\xB3V[`\x08T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x06\xA5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01RhOnly DFMM`\xB8\x1B`D\x82\x01R`d\x01a\x05\x99V[a\x05\xAC\x82\x82a\nMV[3`\0\x90\x81R`\x03` R`@\x81 \x80T\x83\x91\x90\x83\x90a\x06\xD0\x90\x84\x90a\x0F\x03V[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x81\x81R`\x03` R`@\x90\x81\x90 \x80T\x85\x01\x90UQ3\x90`\0\x80Q` a\x10\xAF\x839\x81Q\x91R\x90a\x04F\x90\x86\x81R` \x01\x90V[B\x84\x10\x15a\x07eW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FPERMIT_DEADLINE_EXPIRED\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\x99V[`\0`\x01a\x07qa\x058V[`\x01`\x01`\xA0\x1B\x03\x8A\x81\x16`\0\x81\x81R`\x07` \x90\x81R`@\x91\x82\x90 \x80T`\x01\x81\x01\x90\x91U\x82Q\x7Fnq\xED\xAE\x12\xB1\xB9\x7FM\x1F`7\x0F\xEF\x10\x10_\xA2\xFA\xAE\x01&\x11J\x16\x9Cd\x84]a&\xC9\x81\x84\x01R\x80\x84\x01\x94\x90\x94R\x93\x8D\x16``\x84\x01R`\x80\x83\x01\x8C\x90R`\xA0\x83\x01\x93\x90\x93R`\xC0\x80\x83\x01\x8B\x90R\x81Q\x80\x84\x03\x90\x91\x01\x81R`\xE0\x83\x01\x90\x91R\x80Q\x92\x01\x91\x90\x91 a\x19\x01`\xF0\x1Ba\x01\0\x83\x01Ra\x01\x02\x82\x01\x92\x90\x92Ra\x01\"\x81\x01\x91\x90\x91Ra\x01B\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 `\0\x84R\x90\x83\x01\x80\x83RR`\xFF\x87\x16\x90\x82\x01R``\x81\x01\x85\x90R`\x80\x81\x01\x84\x90R`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x08}W=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16\x15\x80\x15\x90a\x08\xB3WP\x87`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14[a\x08\xF0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rm$\xA7+ \xA6$\xA2/\xA9\xA4\xA3\xA7\"\xA9`\x91\x1B`D\x82\x01R`d\x01a\x05\x99V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\0\x90\x81R`\x04` \x90\x81R`@\x80\x83 \x8A\x85\x16\x80\x85R\x90\x83R\x92\x81\x90 \x89\x90UQ\x88\x81R\x91\x92\x8A\x16\x91\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x91\x01`@Q\x80\x91\x03\x90\xA3PPPPPPPV[`\0\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F`\0`@Qa\t\x8B\x91\x90a\x10%V[`@\x80Q\x91\x82\x90\x03\x82 ` \x83\x01\x93\x90\x93R\x81\x01\x91\x90\x91R\x7F\xC8\x9E\xFD\xAAT\xC0\xF2\x0Cz\xDFa(\x82\xDF\tP\xF5\xA9Qc~\x03\x07\xCD\xCBLg/)\x8B\x8B\xC6``\x82\x01RF`\x80\x82\x01R0`\xA0\x82\x01R`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[\x80`\x02`\0\x82\x82Ta\n\x05\x91\x90a\x10\x9BV[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x81\x81R`\x03` \x90\x81R`@\x80\x83 \x80T\x86\x01\x90UQ\x84\x81R`\0\x80Q` a\x10\xAF\x839\x81Q\x91R\x91\x01[`@Q\x80\x91\x03\x90\xA3PPV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x03` R`@\x81 \x80T\x83\x92\x90a\nu\x90\x84\x90a\x0F\x03V[\x90\x91UPP`\x02\x80T\x82\x90\x03\x90U`@Q\x81\x81R`\0\x90`\x01`\x01`\xA0\x1B\x03\x84\x16\x90`\0\x80Q` a\x10\xAF\x839\x81Q\x91R\x90` \x01a\nAV[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a\n\xDCW\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a\n\xC0V[P`\0`@\x82\x86\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x92PPP\x92\x91PPV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0B\xB4W`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x0B\xCFWa\x0B\xCFa\n\xFDV[a\x0B\xD8\x83a\x0B\x9DV[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x0B\xFEWa\x0B\xFEa\n\xFDV[a\x0C\x07\x84a\x0B\x9DV[\x92Pa\x0C\x15` \x85\x01a\x0B\x9DV[\x91P`@\x84\x015\x90P\x92P\x92P\x92V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x82\x81\x837P`\0\x91\x01RV[`\0\x82`\x1F\x83\x01\x12a\x0C\xACW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x81\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0C\xC7Wa\x0C\xC7a\x0C%V[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x0C\xEFWa\x0C\xEFa\x0C%V[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a\rZW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R\x92P`\x84\x83\xFD[a\rk\x84` \x83\x01` \x89\x01a\x0C;V[\x96\x95PPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\r\x8BWa\r\x8Ba\n\xFDV[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\r\xA6Wa\r\xA6a\x0BMV[a\r\xB2\x86\x83\x87\x01a\x0CGV[\x93P` \x85\x015\x91P\x80\x82\x11\x15a\r\xCBWa\r\xCBa\x0BMV[Pa\r\xD8\x85\x82\x86\x01a\x0CGV[\x91PP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\r\xF7Wa\r\xF7a\n\xFDV[a\x0E\0\x82a\x0B\x9DV[\x93\x92PPPV[`\0\x80`\0\x80`\0\x80`\0`\xE0\x88\x8A\x03\x12\x15a\x0E%Wa\x0E%a\n\xFDV[a\x0E.\x88a\x0B\x9DV[\x96Pa\x0E<` \x89\x01a\x0B\x9DV[\x95P`@\x88\x015\x94P``\x88\x015\x93P`\x80\x88\x015`\xFF\x81\x16\x81\x14a\x0E`W`\0\x80\xFD[\x96\x99\x95\x98P\x93\x96\x92\x95\x94`\xA0\x84\x015\x94P`\xC0\x90\x93\x015\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a\x0E\x93Wa\x0E\x93a\n\xFDV[a\x0E\x9C\x83a\x0B\x9DV[\x91Pa\x0E\xAA` \x84\x01a\x0B\x9DV[\x90P\x92P\x92\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x0E\xC7W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x0E\xE7WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x04RWa\x04Ra\x0E\xEDV[`\x1F\x82\x11\x15a\x0F`W`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a\x0F=WP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x0F\\W\x82\x81U`\x01\x01a\x0FIV[PPP[PPPV[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0F\x7FWa\x0F\x7Fa\x0C%V[a\x0F\x93\x81a\x0F\x8D\x84Ta\x0E\xB3V[\x84a\x0F\x16V[` \x80`\x1F\x83\x11`\x01\x81\x14a\x0F\xC8W`\0\x84\x15a\x0F\xB0WP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ua\x0F\\V[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15a\x0F\xF7W\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01a\x0F\xD8V[P\x85\x82\x10\x15a\x10\x15W\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[`\0\x80\x83Ta\x103\x81a\x0E\xB3V[`\x01\x82\x81\x16\x80\x15a\x10KW`\x01\x81\x14a\x10`Wa\x10\x8FV[`\xFF\x19\x84\x16\x87R\x82\x15\x15\x83\x02\x87\x01\x94Pa\x10\x8FV[\x87`\0R` \x80`\0 `\0[\x85\x81\x10\x15a\x10\x86W\x81T\x8A\x82\x01R\x90\x84\x01\x90\x82\x01a\x10mV[PPP\x82\x87\x01\x94P[P\x92\x96\x95PPPPPPV[\x80\x82\x01\x80\x82\x11\x15a\x04RWa\x04Ra\x0E\xEDV\xFE\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF";
    /// The bytecode of the contract.
    pub static DFMM_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x01\x1CW`\x005`\xE0\x1C\x80c\x9F\xE1\xC1n\x11a\0\xD9W\x80c\xAF\xFE\xD0\xE0\x11a\0\xB3W\x80c\xAF\xFE\xD0\xE0\x14a\x03IW\x80c\xB4b\xCD%\x14a\x03QW\x80c\xBD\x06%\xAB\x14a\x03\x90W\x80c\xCE\x15;\xF4\x14a\x03\xB8Wa\x01\x1CV[\x80c\x9F\xE1\xC1n\x14a\x02\xB5W\x80c\xACJ\xFA8\x14a\x02\xC8W\x80c\xAC\x96P\xD8\x14a\x03)Wa\x01\x1CV[\x80c\x02\x16\xB88\x14a\x01\x81W\x80c\x06\x8B\xCD\x8D\x14a\x01\x96W\x80c\x14U\xF1\xFC\x14a\x02 W\x80c.\xC3\x81\x88\x14a\x02SW\x80c;\xE6\xA3A\x14a\x02\x81W\x80c\x9D\x94/\x9A\x14a\x02\xA2W[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x01\x94a\x01\x8F6`\x04a%\xA4V[a\x03\xCBV[\0[a\x01\xA9a\x01\xA46`\x04a&\x80V[a\x05\x10V[`@Qa\x02\x17\x91\x90\x81Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R` \x80\x84\x01Q\x82\x16\x90\x83\x01R`@\x80\x84\x01Q\x82\x16\x90\x83\x01R``\x80\x84\x01Q\x82\x16\x90\x83\x01R`\x80\x80\x84\x01Q\x90\x83\x01R`\xA0\x83\x81\x01Q\x90\x83\x01R`\xC0\x80\x84\x01Q\x90\x83\x01R`\xE0\x92\x83\x01Q\x16\x91\x81\x01\x91\x90\x91Ra\x01\0\x01\x90V[`@Q\x80\x91\x03\x90\xF3[a\x023a\x02.6`\x04a&\x9CV[a\x05\xE9V[`@\x80Q\x94\x85R` \x85\x01\x93\x90\x93R\x91\x83\x01R``\x82\x01R`\x80\x01a\x02\x17V[a\x02fa\x02a6`\x04a%\xA4V[a\x0CEV[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x02\x17V[a\x02\x94a\x02\x8F6`\x04a'DV[a\raV[`@Q\x90\x81R` \x01a\x02\x17V[a\x02fa\x02\xB06`\x04a%\xA4V[a\x0FMV[a\x01\x94a\x02\xC36`\x04a'qV[a\x12\x19V[a\x02\xDBa\x02\xD66`\x04a&\x80V[a\x12\xD8V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x99\x8A\x16\x81R\x97\x89\x16` \x89\x01R\x95\x88\x16\x95\x87\x01\x95\x90\x95R\x92\x86\x16``\x86\x01R`\x80\x85\x01\x91\x90\x91R`\xA0\x84\x01R`\xC0\x83\x01R\x90\x91\x16`\xE0\x82\x01Ra\x01\0\x01a\x02\x17V[a\x03<a\x0376`\x04a'\xE7V[a\x13AV[`@Qa\x02\x17\x91\x90a)\x82V[`\0Ta\x02\x94V[a\x03x\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\x17V[a\x03\xA3a\x03\x9E6`\x04a%\xA4V[a\x14cV[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x02\x17V[a\x02fa\x03\xC66`\x04a&\x80V[a\x16;V[`\x01T`\x02\x03a\x03\xEEW`@Qc\x03\xCB\x96\xDB`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x01U`\0\x80T\x84\x90\x81\x10a\x04\x07Wa\x04\x07a)\xFCV[`\0\x91\x82R` \x90\x91 `\x08\x90\x91\x02\x01T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x04@W`@Qc#\x01\x9Eg`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x83\x81T\x81\x10a\x04SWa\x04Sa)\xFCV[`\0\x91\x82R` \x90\x91 `\x01`\x08\x90\x92\x02\x01\x01T`@QbB\xD7\x07`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x02\x16\xB88\x90a\x04\x97\x90\x86\x90\x86\x90\x86\x90`\x04\x01a*\x12V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x04\xEFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a,\xBD\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x05\x03W=`\0\x80>=`\0\xFD[PP`\x01\x80UPPPPPV[`@\x80Qa\x01\0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R`\xC0\x81\x01\x82\x90R`\xE0\x81\x01\x91\x90\x91R`\0\x82\x81T\x81\x10a\x05dWa\x05da)\xFCV[`\0\x91\x82R` \x91\x82\x90 `@\x80Qa\x01\0\x81\x01\x82R`\x08\x90\x93\x02\x90\x91\x01\x80T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x84R`\x01\x82\x01T\x81\x16\x94\x84\x01\x94\x90\x94R`\x02\x81\x01T\x84\x16\x91\x83\x01\x91\x90\x91R`\x03\x81\x01T\x83\x16``\x83\x01R`\x04\x81\x01T`\x80\x83\x01R`\x05\x81\x01T`\xA0\x83\x01R`\x06\x81\x01T`\xC0\x83\x01R`\x07\x01T\x90\x91\x16`\xE0\x82\x01R\x92\x91PPV[`\0\x80`\0\x80`\x01T`\x02\x03a\x06\x12W`@Qc\x03\xCB\x96\xDB`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x01Ua\x06'``\x86\x01`@\x87\x01a*HV[`\x01`\x01`\xA0\x1B\x03\x16a\x06@`@\x87\x01` \x88\x01a*HV[`\x01`\x01`\xA0\x1B\x03\x16\x03a\x06gW`@Qc3\x91\n\xEF`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80\x80\x80\x80a\x06z` \x8B\x01\x8Ba*HV[`\0T`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90c\x9F\x83\x13{\x90a\x06\x9D``\x8E\x01\x8Ea*\xABV[`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x06\xBB\x93\x92\x91\x90a*\x12V[`\xA0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x07\x13W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a,\xBD\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x07'W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07K\x91\x90a+\x8CV[\x94P\x94P\x94P\x94P\x94P\x84a\x07\x8EW`\0\x84\x12a\x07g\x85a\x16\xBBV[`@Qcw`m)`\xE1\x1B\x81R\x91\x15\x15`\x04\x83\x01R`$\x82\x01R`D\x01[`@Q\x80\x91\x03\x90\xFD[`\0a\x07\xB9\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x16\xFAV[`@\x80Qc&lE\xBB`\xE1\x1B\x81R`\x04\x81\x01\x91\x90\x91R`\0`D\x82\x01\x81\x90R```$\x83\x01R`d\x82\x01R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x82\x16\x90cL\xD8\x8Bv\x90`\x84\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x08SW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a,\xBD\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x08gW=`\0\x80>=`\0\xFD[PP`@Qc@\xC1\x0F\x19`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x85\x90R`\x01`\x01`\xA0\x1B\x03\x84\x16\x92Pc@\xC1\x0F\x19\x91P`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x08\xF1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a,\xBD\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\t\x05W=`\0\x80>=`\0\xFD[PP`@\x80Qa\x01\0\x81\x01\x90\x91R3\x81R`\0\x92P\x90P` \x80\x82\x01\x90a\t.\x90\x8F\x01\x8Fa*HV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8D` \x01` \x81\x01\x90a\tO\x91\x90a*HV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8D`@\x01` \x81\x01\x90a\tp\x91\x90a*HV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R` \x80\x83\x01\x89\x90R`@\x80\x84\x01\x89\x90R``\x80\x85\x01\x89\x90R\x87\x84\x16`\x80\x95\x86\x01R`\0\x80T`\x01\x80\x82\x01\x83U\x82\x80R\x88Q\x7F)\r\xEC\xD9T\x8Bb\xA8\xD6\x03E\xA9\x888o\xC8K\xA6\xBC\x95H@\x08\xF66/\x93\x16\x0E\xF3\xE5c`\x08\x90\x93\x02\x92\x83\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16\x92\x8A\x16\x92\x90\x92\x17\x90U\x95\x89\x01Q\x7F)\r\xEC\xD9T\x8Bb\xA8\xD6\x03E\xA9\x888o\xC8K\xA6\xBC\x95H@\x08\xF66/\x93\x16\x0E\xF3\xE5d\x83\x01\x80T\x88\x16\x91\x89\x16\x91\x90\x91\x17\x90U\x93\x88\x01Q\x7F)\r\xEC\xD9T\x8Bb\xA8\xD6\x03E\xA9\x888o\xC8K\xA6\xBC\x95H@\x08\xF66/\x93\x16\x0E\xF3\xE5e\x82\x01\x80T\x87\x16\x91\x88\x16\x91\x90\x91\x17\x90U\x91\x87\x01Q\x7F)\r\xEC\xD9T\x8Bb\xA8\xD6\x03E\xA9\x888o\xC8K\xA6\xBC\x95H@\x08\xF66/\x93\x16\x0E\xF3\xE5f\x83\x01\x80T\x86\x16\x91\x87\x16\x91\x90\x91\x17\x90U\x94\x86\x01Q\x7F)\r\xEC\xD9T\x8Bb\xA8\xD6\x03E\xA9\x888o\xC8K\xA6\xBC\x95H@\x08\xF66/\x93\x16\x0E\xF3\xE5g\x82\x01U`\xA0\x86\x01Q\x7F)\r\xEC\xD9T\x8Bb\xA8\xD6\x03E\xA9\x888o\xC8K\xA6\xBC\x95H@\x08\xF66/\x93\x16\x0E\xF3\xE5h\x82\x01U`\xC0\x86\x01Q\x7F)\r\xEC\xD9T\x8Bb\xA8\xD6\x03E\xA9\x888o\xC8K\xA6\xBC\x95H@\x08\xF66/\x93\x16\x0E\xF3\xE5i\x82\x01U`\xE0\x86\x01Q\x7F)\r\xEC\xD9T\x8Bb\xA8\xD6\x03E\xA9\x888o\xC8K\xA6\xBC\x95H@\x08\xF66/\x93\x16\x0E\xF3\xE5j\x90\x91\x01\x80T\x90\x93\x16\x93\x16\x92\x90\x92\x17\x90U\x81T\x92\x93P\x90\x91a\x0BK\x91\x90a+\xECV[\x90Pa\x0Bk\x8D` \x01` \x81\x01\x90a\x0Bc\x91\x90a*HV[30\x89a\x17gV[a\x0B\x89\x8D`@\x01` \x81\x01\x90a\x0B\x81\x91\x90a*HV[30\x88a\x17gV[a\x0B\x99``\x8E\x01`@\x8F\x01a*HV[`\x01`\x01`\xA0\x1B\x03\x16\x8D` \x01` \x81\x01\x90a\x0B\xB5\x91\x90a*HV[`\x01`\x01`\xA0\x1B\x03\x16\x8E`\0\x01` \x81\x01\x90a\x0B\xD1\x91\x90a*HV[`@\x80Q3\x81R` \x81\x01\x86\x90R\x90\x81\x01\x8A\x90R``\x81\x01\x89\x90R`\x80\x81\x01\x88\x90R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90\x7F\xF7\xC0\x95\xAF\xDDB\r*7\x8A\x88r\x05\xCFW\xEB\xEE/m\xD0\x07\xDD\xD7c\xC8Z\xEA\xC0Z\xE3\xD7\xCA\x90`\xA0\x01`@Q\x80\x91\x03\x90\xA4`\x01\x80U\x9C\x94\x9BP\x92\x99P\x90\x97P\x91\x95PPPPPPV[`\0\x80`\0`\x01T`\x02\x03a\x0CmW`@Qc\x03\xCB\x96\xDB`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x01\x81\x90UP`\0\x80`\0a\x0C\x87\x89`\x01\x8A\x8Aa\x17\xF5V[\x92P\x92P\x92Pa\x0C\xC8`\0\x8A\x81T\x81\x10a\x0C\xA3Wa\x0C\xA3a)\xFCV[`\0\x91\x82R` \x90\x91 `\x02`\x08\x90\x92\x02\x01\x01T`\x01`\x01`\xA0\x1B\x03\x1630\x86a\x17gV[a\r\x03`\0\x8A\x81T\x81\x10a\x0C\xDEWa\x0C\xDEa)\xFCV[`\0\x91\x82R` \x90\x91 `\x03`\x08\x90\x92\x02\x01\x01T`\x01`\x01`\xA0\x1B\x03\x1630\x85a\x17gV[`@\x80Q\x8A\x81R` \x81\x01\x85\x90R\x90\x81\x01\x83\x90R``\x81\x01\x82\x90R3\x90\x7F\x95\x97W\x7F3\x93 w^c\xD3\xFE\xD7\xD5\xDD\xE66[\xAD\xCC\x9F\xCC\xDBf\xB3H\x94c\x0C\xA9\x8Bo\x90`\x80\x01[`@Q\x80\x91\x03\x90\xA2`\x01\x80U\x91\x98\x90\x97P\x90\x95P\x93PPPPV[`\0\x80`\0\x83\x81T\x81\x10a\rwWa\rwa)\xFCV[`\0\x91\x82R` \x82 `\x08\x91\x90\x91\x02\x01`\x07\x01T`@Qcp\xA0\x821`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x87\x81\x16`\x04\x83\x01R\x90\x91\x16\x92P\x82\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0E\x0EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a,\xBD\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0E\"W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0EF\x91\x90a+\xFFV[\x90P`\0\x82`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0E\xC1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a,\xBD\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0E\xD5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\xF9\x91\x90a+\xFFV[\x90P`\0\x80\x86\x81T\x81\x10a\x0F\x0FWa\x0F\x0Fa)\xFCV[\x90`\0R` `\0 \x90`\x08\x02\x01`\x06\x01T\x90Pa\x0F@a\x0F9\x83\x83a\x1A\xDB\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x84\x90a\x1A\xF7V[\x94PPPPP[\x92\x91PPV[`\0\x80`\0`\x01T`\x02\x03a\x0FuW`@Qc\x03\xCB\x96\xDB`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x01U`\0\x80\x80a\x0F\x8A\x89\x82\x8A\x8Aa\x17\xF5V[\x92P\x92P\x92P`\0\x89\x81T\x81\x10a\x0F\xA3Wa\x0F\xA3a)\xFCV[`\0\x91\x82R` \x90\x91 `\x08\x90\x91\x02\x01`\x02\x01T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x85\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x10>W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a,\xBD\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x10RW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10v\x91\x90a,\x1BV[P`\0\x89\x81T\x81\x10a\x10\x8AWa\x10\x8Aa)\xFCV[`\0\x91\x82R` \x90\x91 `\x08\x90\x91\x02\x01`\x03\x01T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x11%W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a,\xBD\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x119W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11]\x91\x90a,\x1BV[Pa\x11\x98`\0\x8A\x81T\x81\x10a\x11tWa\x11ta)\xFCV[`\0\x91\x82R` \x90\x91 `\x02`\x08\x90\x92\x02\x01\x01T`\x01`\x01`\xA0\x1B\x03\x163\x85a\x1B\x0CV[a\x11\xD2`\0\x8A\x81T\x81\x10a\x11\xAEWa\x11\xAEa)\xFCV[`\0\x91\x82R` \x90\x91 `\x03`\x08\x90\x92\x02\x01\x01T`\x01`\x01`\xA0\x1B\x03\x163\x84a\x1B\x0CV[`@\x80Q\x8A\x81R` \x81\x01\x85\x90R\x90\x81\x01\x83\x90R``\x81\x01\x82\x90R3\x90\x7F\xAC\xBE\x12~\x93\xA8\xA0\xB2x\xD8\xE0n' [=\xF9\xD1\xF3\x81$\x14\xBC\x89\x17\xC7t\xA87\x101n\x90`\x80\x01a\rFV[`\x01T`\x02\x03a\x12<W`@Qc\x03\xCB\x96\xDB`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x01U`\0\x80T\x83\x90\x81\x10a\x12UWa\x12Ua)\xFCV[`\0\x91\x82R` \x90\x91 `\x08\x90\x91\x02\x01T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x12\x8EW`@Qc#\x01\x9Eg`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80`\0\x83\x81T\x81\x10a\x12\xA2Wa\x12\xA2a)\xFCV[`\0\x91\x82R` \x90\x91 `\x08\x90\x91\x02\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UPP`\x01\x80UV[`\0\x81\x81T\x81\x10a\x12\xE8W`\0\x80\xFD[`\0\x91\x82R` \x90\x91 `\x08\x90\x91\x02\x01\x80T`\x01\x82\x01T`\x02\x83\x01T`\x03\x84\x01T`\x04\x85\x01T`\x05\x86\x01T`\x06\x87\x01T`\x07\x90\x97\x01T`\x01`\x01`\xA0\x1B\x03\x96\x87\x16\x98P\x94\x86\x16\x96\x93\x86\x16\x95\x92\x83\x16\x94\x91\x93\x90\x92\x90\x91\x16\x88V[```\0\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x13_Wa\x13_a'\xA0V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x13\x92W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x13}W\x90P[P\x90P`\0[\x83Q\x81\x03a\x14\\W`\0\x800`\x01`\x01`\xA0\x1B\x03\x16\x86\x84\x81Q\x81\x10a\x13\xBFWa\x13\xBFa)\xFCV[` \x02` \x01\x01Q`@Qa\x13\xD4\x91\x90a,9V[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14a\x14\x0FW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x14\x14V[``\x91P[P\x91P\x91P\x81a\x144W\x80Q`\0\x03a\x14,W`\0\x80\xFD[\x80Q\x81` \x01\xFD[\x80\x84\x84\x81Q\x81\x10a\x14GWa\x14Ga)\xFCV[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x01a\x13\x98V[P\x92\x91PPV[`\0\x80`\x01T`\x02\x03a\x14\x89W`@Qc\x03\xCB\x96\xDB`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x01\x81\x90UP`\0\x80`\0\x80`\0\x80\x8A\x81T\x81\x10a\x14\xABWa\x14\xABa)\xFCV[`\0\x91\x82R` \x90\x91 `\x01`\x08\x90\x92\x02\x01\x01T`@Qc2\x14\x89\x0F`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c2\x14\x89\x0F\x90a\x14\xF0\x90\x8D\x90\x8D\x90\x8D\x90`\x04\x01a*\x12V[`\xC0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x15FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a,\xBD\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x15ZW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15~\x91\x90a,UV[\x95P\x95P\x95PP\x94P\x94P\x84a\x15\x9BW`\0\x84\x12a\x07g\x85a\x16\xBBV[\x80`\0\x8B\x81T\x81\x10a\x15\xAFWa\x15\xAFa)\xFCV[\x90`\0R` `\0 \x90`\x08\x02\x01`\x06\x01\x81\x90UP`\0\x80`\0a\x15\xD4\x8D\x87\x87a\x1B\x90V[`@\x80Q\x86\x15\x15\x81R` \x81\x01\x84\x90R\x90\x81\x01\x82\x90R\x94\x97P\x90\x95P\x93P\x8F\x923\x92P\x7FL}\xEF\x84\xE4++\xC0\xA5\xAA\xB2\"\x86\x8D\xD7\xA0\x92\xB53w\xA4\xB57\xAB\xCD\x944Zz\x85'\xED\x91P``\x01`@Q\x80\x91\x03\x90\xA3`\x01\x80U\x90\x9C\x90\x9BP\x99PPPPPPPPPPV[`\0\x80`\0\x80\x84\x81T\x81\x10a\x16RWa\x16Ra)\xFCV[\x90`\0R` `\0 \x90`\x08\x02\x01`\x04\x01T`\0\x85\x81T\x81\x10a\x16wWa\x16wa)\xFCV[\x90`\0R` `\0 \x90`\x08\x02\x01`\x05\x01T`\0\x86\x81T\x81\x10a\x16\x9CWa\x16\x9Ca)\xFCV[\x90`\0R` `\0 \x90`\x08\x02\x01`\x06\x01T\x92P\x92P\x92P\x91\x93\x90\x92PV[`\0`\x01`\xFF\x1B\x82\x03a\x16\xE1W`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x12\x15a\x16\xF2WP\x19`\x01\x01\x90V[P\x80[\x91\x90PV[`\0v=`-\x80`\n=9\x81\xF36==7===6=s\0\0\0\x82``\x1B`\xE8\x1C\x17`\0RnZ\xF4=\x82\x80>\x90=\x91`+W\xFD[\xF3\x82`x\x1B\x17` R`7`\t`\0\xF0\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16a\x16\xF5W`@Qc0\xBE\x1A=`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0`@Qc#\xB8r\xDD`\xE0\x1B`\0R\x84`\x04R\x83`$R\x82`DR` `\0`d`\0\x80\x8AZ\xF1=\x15`\x1F=\x11`\x01`\0Q\x14\x16\x17\x16\x91P`\0``R\x80`@RP\x80a\x17\xEEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rs\x15\x14\x90S\x94\xD1\x91T\x97\xD1\x94\x93\xD3W\xD1\x90RS\x11Q`b\x1B`D\x82\x01R`d\x01a\x07\x85V[PPPPPV[`\0\x80`\0\x80`\0\x80`\0\x80`\0\x8C\x81T\x81\x10a\x18\x14Wa\x18\x14a)\xFCV[`\0\x91\x82R` \x90\x91 `\x01`\x08\x90\x92\x02\x01\x01T`@Qb#\x8Bu`\xEA\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x8E-\xD4\0\x90a\x18X\x90\x8F\x90\x8E\x90\x8E\x90`\x04\x01a*\x12V[`\xA0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x18\xAEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a,\xBD\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x18\xC2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18\xE6\x91\x90a+\x8CV[\x94P\x94P\x94P\x94P\x94P\x84a\x19\x02W`\0\x84\x12a\x07g\x85a\x16\xBBV[\x8Aa\x19<W\x82`\0\x8D\x81T\x81\x10a\x19\x1BWa\x19\x1Ba)\xFCV[\x90`\0R` `\0 \x90`\x08\x02\x01`\x04\x01Ta\x197\x91\x90a+\xECV[a\x19lV[`\0\x8C\x81T\x81\x10a\x19OWa\x19Oa)\xFCV[\x90`\0R` `\0 \x90`\x08\x02\x01`\x04\x01T\x83a\x19l\x91\x90a+\xECV[\x97P\x8Aa\x19\xA8W\x81`\0\x8D\x81T\x81\x10a\x19\x87Wa\x19\x87a)\xFCV[\x90`\0R` `\0 \x90`\x08\x02\x01`\x05\x01Ta\x19\xA3\x91\x90a+\xECV[a\x19\xD8V[`\0\x8C\x81T\x81\x10a\x19\xBBWa\x19\xBBa)\xFCV[\x90`\0R` `\0 \x90`\x08\x02\x01`\x05\x01T\x82a\x19\xD8\x91\x90a+\xECV[\x96P\x8Aa\x1A\x14W\x80`\0\x8D\x81T\x81\x10a\x19\xF3Wa\x19\xF3a)\xFCV[\x90`\0R` `\0 \x90`\x08\x02\x01`\x06\x01Ta\x1A\x0F\x91\x90a+\xECV[a\x1ADV[`\0\x8C\x81T\x81\x10a\x1A'Wa\x1A'a)\xFCV[\x90`\0R` `\0 \x90`\x08\x02\x01`\x06\x01T\x81a\x1AD\x91\x90a+\xECV[\x95Pa\x1AQ\x8C\x8C\x88a!EV[\x82`\0\x8D\x81T\x81\x10a\x1AeWa\x1Aea)\xFCV[\x90`\0R` `\0 \x90`\x08\x02\x01`\x04\x01\x81\x90UP\x81`\0\x8D\x81T\x81\x10a\x1A\x8EWa\x1A\x8Ea)\xFCV[\x90`\0R` `\0 \x90`\x08\x02\x01`\x05\x01\x81\x90UP\x80`\0\x8D\x81T\x81\x10a\x1A\xB7Wa\x1A\xB7a)\xFCV[\x90`\0R` `\0 \x90`\x08\x02\x01`\x06\x01\x81\x90UPPPPPP\x94P\x94P\x94\x91PPV[`\0a\x1A\xF0\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a#\xDFV[\x93\x92PPPV[`\0a\x1A\xF0\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a#\xDFV[`\0`@Qc\xA9\x05\x9C\xBB`\xE0\x1B`\0R\x83`\x04R\x82`$R` `\0`D`\0\x80\x89Z\xF1=\x15`\x1F=\x11`\x01`\0Q\x14\x16\x17\x16\x91P`\0``R\x80`@RP\x80a\x1B\x8AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rn\x15\x14\x90S\x94\xD1\x91T\x97\xD1\x90RS\x11Q`\x8A\x1B`D\x82\x01R`d\x01a\x07\x85V[PPPPV[`\0\x80`\0\x80`\0\x80`\0\x89\x81T\x81\x10a\x1B\xACWa\x1B\xACa)\xFCV[\x90`\0R` `\0 \x90`\x08\x02\x01`\x04\x01T\x90P`\0\x80\x8A\x81T\x81\x10a\x1B\xD4Wa\x1B\xD4a)\xFCV[\x90`\0R` `\0 \x90`\x08\x02\x01`\x05\x01T\x90P\x81\x89\x11\x96P\x86\x15a\x1C6W\x87\x81\x11a\x1C1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01Rk\x06\x96\xE7f\x16\xC6\x96B\x077v\x17`\xA4\x1B`D\x82\x01R`d\x01a\x07\x85V[a\x1CtV[\x88\x82\x11a\x1CtW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01Rk\x06\x96\xE7f\x16\xC6\x96B\x077v\x17`\xA4\x1B`D\x82\x01R`d\x01a\x07\x85V[\x86a\x1C\xAEW`\0\x8A\x81T\x81\x10a\x1C\x8CWa\x1C\x8Ca)\xFCV[`\0\x91\x82R` \x90\x91 `\x03`\x08\x90\x92\x02\x01\x01T`\x01`\x01`\xA0\x1B\x03\x16a\x1C\xDFV[`\0\x8A\x81T\x81\x10a\x1C\xC1Wa\x1C\xC1a)\xFCV[`\0\x91\x82R` \x90\x91 `\x02`\x08\x90\x92\x02\x01\x01T`\x01`\x01`\xA0\x1B\x03\x16[\x95P\x86a\x1D\x1BW`\0\x8A\x81T\x81\x10a\x1C\xF9Wa\x1C\xF9a)\xFCV[`\0\x91\x82R` \x90\x91 `\x02`\x08\x90\x92\x02\x01\x01T`\x01`\x01`\xA0\x1B\x03\x16a\x1DLV[`\0\x8A\x81T\x81\x10a\x1D.Wa\x1D.a)\xFCV[`\0\x91\x82R` \x90\x91 `\x03`\x08\x90\x92\x02\x01\x01T`\x01`\x01`\xA0\x1B\x03\x16[\x94P\x86a\x1DbWa\x1D]\x81\x89a+\xECV[a\x1DlV[a\x1Dl\x82\x8Aa+\xECV[\x93P\x86a\x1D\x82Wa\x1D}\x89\x83a+\xECV[a\x1D\x8CV[a\x1D\x8C\x88\x82a+\xECV[\x92P\x88`\0\x8B\x81T\x81\x10a\x1D\xA2Wa\x1D\xA2a)\xFCV[\x90`\0R` `\0 \x90`\x08\x02\x01`\x04\x01\x81\x90UP\x87`\0\x8B\x81T\x81\x10a\x1D\xCBWa\x1D\xCBa)\xFCV[`\0\x91\x82R` \x82 `\x08\x91\x90\x91\x02\x01`\x05\x01\x91\x90\x91U`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x88\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x1E_W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a,\xBD\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x1EsW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E\x97\x91\x90a+\xFFV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x88\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x1F\x1AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a,\xBD\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x1F.W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1FR\x91\x90a+\xFFV[\x90Pa\x1F`\x8830\x89a\x17gV[a\x1Fk\x873\x87a\x1B\x0CV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x90`\x01`\x01`\xA0\x1B\x03\x8A\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x1F\xEBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a,\xBD\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x1F\xFFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a #\x91\x90a+\xFFV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x8A\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a \xA6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a,\xBD\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a \xBAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a \xDE\x91\x90a+\xFFV[\x90Pa \xEA\x88\x85a,\xA9V[\x82\x10\x15a!\nW`@Qc =\x90\x1D`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a!\x14\x87\x84a+\xECV[\x81\x10\x15a!4W`@Qc\xF3\xCB\xBC\x87`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPP\x93\x97\x92\x96P\x93P\x93PV[`\0\x80\x84\x81T\x81\x10a!YWa!Ya)\xFCV[\x90`\0R` `\0 \x90`\x08\x02\x01`\x07\x01`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x90P`\0\x81`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a!\xF9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a,\xBD\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\"\rW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\"1\x91\x90a+\xFFV[\x90P`\0\x80\x86\x81T\x81\x10a\"GWa\"Ga)\xFCV[\x90`\0R` `\0 \x90`\x08\x02\x01`\x06\x01T\x90P\x84\x15a#\x1EW`\0a\"wa\"p\x84\x84a\x1A\xDBV[\x86\x90a\x1A\xF7V[`@Qc@\xC1\x0F\x19`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x82\x90R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x85\x16\x90c@\xC1\x0F\x19\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a#\0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a,\xBD\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a#\x14W=`\0\x80>=`\0\xFD[PPPPPa#\xD7V[`\0a#4a#-\x84\x84a#\xFEV[\x86\x90a$\x13V[`@Qc'p\xA7\xEB`\xE2\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x82\x90R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x85\x16\x90c\x9D\xC2\x9F\xAC\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a#\xBDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a,\xBD\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a#\xD1W=`\0\x80>=`\0\xFD[PPPPP[PPPPPPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a#\xF7W`\0\x80\xFD[\x04\x92\x91PPV[`\0a\x1A\xF0\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a$$V[`\0a\x1A\xF0\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a$<W`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x81\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rjrray stride`\xA8\x1B`d\x82\x01R`\x84\x81\xFD[`\0\x80`\0`@\x84\x86\x03\x12\x15a%\xBCWa%\xBCa$RV[\x835\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a%\xDEWa%\xDEa$\xA2V[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a%\xF5Wa%\xF5a$\xF2V[\x815\x81\x81\x11\x15a&XW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x0ENL/$\r\x8C\xAD\xCC\xEE\x8D`\xAB\x1B`d\x82\x01R`\x84\x81\xFD[\x87` \x82\x85\x01\x01\x11\x15a&mWa&ma%KV[` \x83\x01\x94P\x80\x93PPPP\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a&\x95Wa&\x95a$RV[P5\x91\x90PV[`\0` \x82\x84\x03\x12\x15a&\xB1Wa&\xB1a$RV[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a&\xCBWa&\xCBa$\xA2V[\x82\x01`\x80\x81\x85\x03\x12\x15a\x1A\xF0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FABI decoding: struct calldata to`D\x82\x01Rf\x1B\xC8\x1C\xDA\x1B\xDC\x9D`\xCA\x1B`d\x82\x01R`\x84\x81\xFD[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x16\xF5W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a'ZWa'Za$RV[a'c\x83a'-V[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`@\x83\x85\x03\x12\x15a'\x87Wa'\x87a$RV[\x825\x91Pa'\x97` \x84\x01a'-V[\x90P\x92P\x92\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a'\xDFWa'\xDFa'\xA0V[`@R\x91\x90PV[`\0` \x80\x83\x85\x03\x12\x15a'\xFDWa'\xFDa$RV[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a(\x18Wa(\x18a$\xA2V[\x81\x85\x01\x91P`\x1F\x86\x81\x84\x01\x12a(0Wa(0a$\xF2V[\x825\x82\x81\x11\x15a(BWa(Ba'\xA0V[\x80`\x05\x1Ba(Q\x86\x82\x01a'\xB6V[\x91\x82R\x84\x81\x01\x86\x01\x91\x86\x81\x01\x90\x8A\x84\x11\x15a(nWa(na%KV[\x87\x87\x01\x92P[\x83\x83\x10\x15a)PW\x825\x86\x81\x11\x15a(\x8EWa(\x8Ea$\xF2V[\x87\x01`?\x81\x01\x8C\x13a(\xA2Wa(\xA2a$\xF2V[\x88\x81\x015`@\x88\x82\x11\x15a(\xB8Wa(\xB8a'\xA0V[a(\xC9\x82\x89\x01`\x1F\x19\x16\x8C\x01a'\xB6V[\x82\x81R\x8E\x82\x84\x86\x01\x01\x11\x15a),W\x81QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x8D\x90R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R`\x84\x81\xFD[\x82\x82\x85\x01\x8D\x83\x017`\0\x92\x81\x01\x8C\x01\x92\x90\x92RP\x83RP\x91\x87\x01\x91\x90\x87\x01\x90a(tV[\x9A\x99PPPPPPPPPPV[`\0[\x83\x81\x10\x15a)yW\x81\x81\x01Q\x83\x82\x01R` \x01a)aV[PP`\0\x91\x01RV[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x86\x01\x91P`@\x81`\x05\x1B\x87\x01\x01\x92P\x83\x87\x01`\0[\x82\x81\x10\x15a)\xEFW\x87\x85\x03`?\x19\x01\x84R\x81Q\x80Q\x80\x87Ra)\xD0\x81\x89\x89\x01\x8A\x85\x01a)^V[`\x1F\x01`\x1F\x19\x16\x95\x90\x95\x01\x86\x01\x94P\x92\x85\x01\x92\x90\x85\x01\x90`\x01\x01a)\xA9V[P\x92\x97\x96PPPPPPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[\x83\x81R`@` \x82\x01R\x81`@\x82\x01R\x81\x83``\x83\x017`\0\x81\x83\x01``\x90\x81\x01\x91\x90\x91R`\x1F\x90\x92\x01`\x1F\x19\x16\x01\x01\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a*]Wa*]a$RV[a\x1A\xF0\x82a'-V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FCalldata tail too short\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x81\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a+\x02W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FInvalid calldata tail offset\0\0\0\0`D\x82\x01R`d\x81\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a+]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FInvalid calldata tail length\0\0\0\0`D\x82\x01R`d\x81\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a+uWa+ua*fV[\x92P\x92\x90PV[\x80Q\x80\x15\x15\x81\x14a\x16\xF5W`\0\x80\xFD[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a+\xA7Wa+\xA7a$RV[a+\xB0\x86a+|V[` \x87\x01Q`@\x88\x01Q``\x89\x01Q`\x80\x90\x99\x01Q\x92\x9A\x91\x99P\x97\x96P\x90\x94P\x92PPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x0FGWa\x0FGa+\xD6V[`\0` \x82\x84\x03\x12\x15a,\x14Wa,\x14a$RV[PQ\x91\x90PV[`\0` \x82\x84\x03\x12\x15a,0Wa,0a$RV[a\x1A\xF0\x82a+|V[`\0\x82Qa,K\x81\x84` \x87\x01a)^V[\x91\x90\x91\x01\x92\x91PPV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a,qWa,qa$RV[a,z\x87a+|V[\x95P` \x87\x01Q\x94P`@\x87\x01Q\x93P``\x87\x01Q\x92P`\x80\x87\x01Q\x91P`\xA0\x87\x01Q\x90P\x92\x95P\x92\x95P\x92\x95V[\x80\x82\x01\x80\x82\x11\x15a\x0FGWa\x0FGa+\xD6V\xFETarget contract does not contain";
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
        pub account: ::ethers::core::types::Address,
        #[ethevent(indexed)]
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
