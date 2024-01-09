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
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("isLogNormal"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("bool"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("tokenX_"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("tokenY_"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("swapFeePercentageWad"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
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
                    ::std::borrow::ToOwned::to_owned("balanceOf"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("balanceOf"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("balance"),
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
                    ::std::borrow::ToOwned::to_owned("deallocate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("deallocate"),
                            inputs: ::std::vec![
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
                    ::std::borrow::ToOwned::to_owned("feeGrowth"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("feeGrowth"),
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
                    ::std::borrow::ToOwned::to_owned("feeGrowthLast"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("feeGrowthLast"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("balance"),
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
                    ::std::borrow::ToOwned::to_owned("inited"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("inited"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("locked"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("locked"),
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
                    ::std::borrow::ToOwned::to_owned("reserveXWad"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("reserveXWad"),
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
                    ::std::borrow::ToOwned::to_owned("reserveYWad"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("reserveYWad"),
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
                (
                    ::std::borrow::ToOwned::to_owned("swap"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("swap"),
                            inputs: ::std::vec![
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
                    ::std::borrow::ToOwned::to_owned("tokenX"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("tokenX"),
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
                    ::std::borrow::ToOwned::to_owned("tokenY"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("tokenY"),
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
                    ::std::borrow::ToOwned::to_owned("totalLiquidity"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("totalLiquidity"),
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
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("Allocate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Allocate"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("x"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("y"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("l"),
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
                                    name: ::std::borrow::ToOwned::to_owned("x"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("y"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("l"),
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
                                    name: ::std::borrow::ToOwned::to_owned("swapper"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("strategy"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("XXXXXXX"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("YYYYYY"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("LLLLLL"),
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
                                    name: ::std::borrow::ToOwned::to_owned("swapper"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("source"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("tokenIn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("tokenOut"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amountIn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amountOut"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("liquidityDelta"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
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
                    ::std::borrow::ToOwned::to_owned("Min"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("Min"),
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
    const __BYTECODE: &[u8] = b"`\x80`@R`\x01\x80U4\x80\x15b\0\0bW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`@Qb\0^\xCD8\x03\x80b\0^\xCD\x839\x81\x01`@\x81\x90Rb\0\0\x85\x91b\0\x01\xCCV[`\x02\x80T`\x01`\x01`\xA0\x1B\x03\x80\x86\x16`\x01`\x01`\xA0\x1B\x03\x19\x92\x83\x16\x17\x90\x92U`\x03\x80T\x92\x85\x16\x92\x90\x91\x16\x91\x90\x91\x17\x90U\x83\x15b\0\x01%W0\x81`@Qb\0\0\xCC\x90b\0\x01\x93V[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x82R` \x82\x01R`@\x01`@Q\x80\x91\x03\x90`\0\xF0\x80\x15\x80\x15b\0\0\xFEW=`\0\x80>=`\0\xFD[P`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90Ub\0\x01\x89V[0\x81`@Qb\0\x015\x90b\0\x01\xA1V[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x82R` \x82\x01R`@\x01`@Q\x80\x91\x03\x90`\0\xF0\x80\x15\x80\x15b\0\x01gW=`\0\x80>=`\0\xFD[P`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U[PPPPb\0\x02oV[a'\x03\x80b\0\x1EI\x839\x01\x90V[a\x19\x81\x80b\0EL\x839\x01\x90V[\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x01\xC7W`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15b\0\x02.W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[\x84Q\x80\x15\x15\x81\x14b\0\x02?W`\0\x80\xFD[\x93Pb\0\x02O` \x86\x01b\0\x01\xAFV[\x92Pb\0\x02_`@\x86\x01b\0\x01\xAFV[``\x95\x90\x95\x01Q\x93\x96\x92\x95PPPV[a\x1B\xCA\x80b\0\x02\x7F`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x01MW`\x005`\xE0\x1C\x80c\xA8\xC6.v\x11a\0\xE4W\x80c\xEA\xE5\xEA\x08\x11a\0\xB3W\x80c\xEA\xE5\xEA\x08\x14a\x02\xEBW\x80c\xEB\xAD\xEF\x01\x14a\x02\xF4W\x80c\xF0e\x95\x7F\x14a\x03\x02W\x80c\xFA\n4^\x14a\x03\x0BWa\x01MV[\x80c\xA8\xC6.v\x14a\x02\x9CW\x80c\xB7\xD1\x9F\xC4\x14a\x02\xAFW\x80c\xBA\xD5&O\x14a\x02\xC2W\x80c\xCF0\x90\x12\x14a\x02\xE2Wa\x01MV[\x80cZ\xEFFz\x11a\x01 W\x80cZ\xEFFz\x14a\x02KW\x80c^\"}X\x14a\x02^W\x80cb}\xD5j\x14a\x02gW\x80cp\xA0\x821\x14a\x02|Wa\x01MV[\x80c\x15w\x0F\x92\x14a\x01\xB2W\x80c\x16\xDC\x16[\x14a\x01\xCEW\x80cC\xC8\x85\xBA\x14a\x01\xF9W\x80cM\xDFG\xD4\x14a\x02\x1DW[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x01\xBB`\x06T\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[`\x02Ta\x01\xE1\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\xC5V[`\0Ta\x02\r\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x01\xC5V[a\x020a\x02+6`\x04a\x18)V[a\x03\x1EV[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x01\xC5V[a\x020a\x02Y6`\x04a\x18)V[a\x06\x96V[a\x01\xBB`\x05T\x81V[a\x02za\x02u6`\x04a\x18)V[a\n\x96V[\0[a\x01\xBBa\x02\x8A6`\x04a\x19\x91V[`\x08` R`\0\x90\x81R`@\x90 T\x81V[`\0Ta\x01\xE1\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x03Ta\x01\xE1\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01\xBBa\x02\xD06`\x04a\x19\x91V[`\t` R`\0\x90\x81R`@\x90 T\x81V[a\x01\xBB`\x01T\x81V[a\x01\xBB`\x07T\x81V[`\x04T`\x05T`\x06Ta\x020V[a\x01\xBB`\x04T\x81V[a\x020a\x03\x196`\x04a\x18)V[a\x0C\x13V[`\0\x80`\0`\x01T`\x01\x14a\x03NW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03E\x90a\x19\xC4V[`@Q\x80\x91\x03\x90\xFD[`\0`\x01\x81\x90U\x80T`@Qc\x13w\xD1\xF5`\xE2\x1B\x81R\x82\x91\x82\x91\x82\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cM\xDFG\xD4\x90a\x03\x8E\x90\x8D\x90\x8D\x90`\x04\x01a\x19\xE4V[`\xA0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x03\xE6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x1B\xAA\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x03\xFAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\x1E\x91\x90a\x1A#V[\x94P\x94P\x94P\x94P\x94P\x84a\x04\\W`\0\x84\x12a\x04:\x85a\x10-V[`@Qcw`m)`\xE1\x1B\x81R\x91\x15\x15`\x04\x83\x01R`$\x82\x01R`D\x01a\x03EV[`\0\x80T`\xFF`\xA0\x1B\x19\x16`\x01`\xA0\x1B\x17\x81U`\x04\x84\x81U`\x05\x84\x90U`\x06\x83\x90U3\x80\x83R`\x08` \x90\x81R`@\x80\x85 \x86\x90Ug\r\xE0\xB6\xB3\xA7d\0\0`\x07\x81\x90U`\t\x90\x92R\x93\x84\x90 U`\x02T\x92Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x92c#\xB8r\xDD\x92a\x04\xDA\x92\x910\x91\x89\x91\x01a\x1AmV[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x052W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x1B\xAA\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x05FW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05j\x91\x90a\x1A\x91V[P`\x03T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\x05\x9F\x903\x900\x90\x87\x90`\x04\x01a\x1AmV[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x05\xF7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x1B\xAA\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x06\x0BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06/\x91\x90a\x1A\x91V[P`\0T`@\x80Q\x85\x81R` \x81\x01\x85\x90R\x90\x81\x01\x83\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x903\x90\x7F?\x1Fc\xA0\x1B[6Ff\x17!O4 E\xD7SS\x025<\xE7\t\xC2\x9E\xD3\xECL\x02H\xBD5\x90``\x01`@Q\x80\x91\x03\x90\xA3`\x01\x80U\x91\x99\x90\x98P\x90\x96P\x94PPPPPV[`\0\x80`\0`\x01T`\x01\x14a\x06\xBDW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03E\x90a\x19\xC4V[`\0`\x01\x81\x90U\x80T`@QcH}k\xAB`\xE1\x1B\x81R\x82\x91\x82\x91\x82\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x90\xFA\xD7V\x90a\x06\xFD\x90\x8D\x90\x8D\x90`\x04\x01a\x19\xE4V[`\xA0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x07SW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x1B\xAA\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x07gW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x8B\x91\x90a\x1A#V[\x94P\x94P\x94P\x94P\x94P\x84a\x07\xA7W`\0\x84\x12a\x04:\x85a\x10-V[3`\0\x90\x81R`\x08` R`@\x90 T\x15\x80\x15\x90a\x07\xD6WP3`\0\x90\x81R`\t` R`@\x90 T`\x07T\x14\x15[\x15a\x08&W3`\0\x90\x81R`\t` R`@\x81 T`\x07Ta\x07\xF7\x91a\x10lV[3`\0\x90\x81R`\x08` R`@\x90 T\x90\x91Pa\x08\x14\x90\x82a\x10lV[3`\0\x90\x81R`\x08` R`@\x90 UP[`\0`\x04T\x84a\x086\x91\x90a\x1A\xC5V[\x90P`\0`\x05T\x84a\x08H\x91\x90a\x1A\xC5V[\x90P`\0`\x06T\x84a\x08Z\x91\x90a\x1A\xC5V[`\x04\x87\x90U`\x05\x86\x90U`\x06\x85\x90U3`\0\x90\x81R`\x08` R`@\x81 \x80T\x92\x93P\x83\x92\x90\x91\x90a\x08\x8D\x90\x84\x90a\x1A\xD8V[\x90\x91UPP`\x07T3`\0\x81\x81R`\t` R`@\x90\x81\x90 \x92\x90\x92U`\x02T\x91Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c#\xB8r\xDD\x91a\x08\xDB\x910\x90\x88\x90`\x04\x01a\x1AmV[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\t3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x1B\xAA\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\tGW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\tk\x91\x90a\x1A\x91V[P`\x03T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\t\xA0\x903\x900\x90\x87\x90`\x04\x01a\x1AmV[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\t\xF8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x1B\xAA\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\n\x0CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n0\x91\x90a\x1A\x91V[P`@\x80Q\x84\x81R` \x81\x01\x84\x90R\x90\x81\x01\x82\x90R\x7F\x92\x8F-\xEA\x06G\xB6\xB9A\xA5K\xBC\x03\xCE\x9B\xCB\x0E\xC1\xE0\x8E\xAAl\x12\x98.{\xF4H\xD9?\xCCt\x90``\x01[`@Q\x80\x91\x03\x90\xA1PP`\x04T`\x05T`\x06T`\x01\x80U\x91\x9D\x90\x9CP\x90\x9AP\x98PPPPPPPPPV[`\x01T`\x01\x14a\n\xB8W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03E\x90a\x19\xC4V[`\0`\x01\x81\x90UT`\x01`\xA0\x1B\x90\x04`\xFF\x16a\n\xE7W`@Qc!\xC4\xE3W`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80T`@Qc\xB2\xB2\xE7\xDD`\xE0\x1B\x81R\x82\x91\x82\x91\x82\x91\x82\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xB2\xB2\xE7\xDD\x90a\x0B\"\x90\x8B\x90\x8B\x90`\x04\x01a\x19\xE4V[`\xC0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0BxW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x1B\xAA\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0B\x8CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\xB0\x91\x90a\x1A\xEBV[\x95P\x95P\x95P\x95P\x95P\x95P\x85a\x0B\xCEW`\0\x85\x12a\x04:\x86a\x10-V[`\x06\x80T\x90\x82\x90U`\0a\x0B\xE2\x83\x83a\x10\x8AV[`\x07T\x90\x91Pa\x0B\xF2\x90\x82a\x10lV[`\x07Ua\x0B\xFF\x85\x85a\x10\x9FV[PP`\x01\x80UPPPPPPPPPPPPV[`\0\x80`\0`\x01T`\x01\x14a\x0C:W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03E\x90a\x19\xC4V[`\0`\x01\x81\x90U\x80T`@QcH}k\xAB`\xE1\x1B\x81R\x82\x91\x82\x91\x82\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x90\xFA\xD7V\x90a\x0Cz\x90\x8D\x90\x8D\x90`\x04\x01a\x19\xE4V[`\xA0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0C\xD0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x1B\xAA\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0C\xE4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\x08\x91\x90a\x1A#V[\x94P\x94P\x94P\x94P\x94P\x84a\r$W`\0\x84\x12a\x04:\x85a\x10-V[3`\0\x90\x81R`\x08` R`@\x90 T\x15\x80\x15\x90a\rSWP3`\0\x90\x81R`\t` R`@\x90 T`\x07T\x14\x15[\x15a\r\xE4W3`\0\x90\x81R`\t` R`@\x81 T`\x07Ta\rt\x91a\x10lV[3`\0\x90\x81R`\x08` R`@\x90 T\x90\x91Pa\r\x91\x90\x82a\x10lV[`\x08`\x003`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 \x81\x90UPa\r\xE2`@Q\x80`@\x01`@R\x80`\x07\x81R` \x01fin here`\xC8\x1B\x81RPa\x16\xF6V[P[`\0\x83`\x04Ta\r\xF4\x91\x90a\x1A\xC5V[\x90P`\0\x83`\x05Ta\x0E\x06\x91\x90a\x1A\xC5V[\x90P`\0\x83`\x06Ta\x0E\x18\x91\x90a\x1A\xC5V[`\x04\x87\x90U`\x05\x86\x90U`\x06\x85\x90U3`\0\x90\x81R`\x08` R`@\x81 \x80T\x92\x93P\x83\x92\x90\x91\x90a\x0EK\x90\x84\x90a\x1A\xC5V[\x90\x91UPP`\x07T3`\0\x81\x81R`\t` R`@\x90\x81\x90 \x92\x90\x92U`\x02T\x91Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R`\x04\x81\x01\x91\x90\x91R`$\x81\x01\x85\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0E\xF3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x1B\xAA\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x0F\x07W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F+\x91\x90a\x1A\x91V[P`\x03T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0F\xB6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x1B\xAA\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x0F\xCAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xEE\x91\x90a\x1A\x91V[P`@\x80Q\x84\x81R` \x81\x01\x84\x90R\x90\x81\x01\x82\x90R\x7FDO\xFE\xDB%A\xFF,*\xC9\xA6\x16\xD97|\xE4\x0Ev*\x8F\x86\xC2\xF9-AJ\xB8\x8E\x14\xE7Z\x85\x90``\x01a\nkV[`\0`\x01`\xFF\x1B\x82\x03a\x10SW`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x12\x15a\x10dWP\x19`\x01\x01\x90V[P\x80[\x91\x90PV[`\0a\x10\x81\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x17<V[\x90P[\x92\x91PPV[`\0a\x10\x81\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x17<V[`\x04T`\x05T`\0\x91\x82\x91\x82\x91\x82\x91\x81\x88\x11\x15a\x118W`\x02T`\x03T`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x97P\x16\x94Pa\x10\xD6\x82\x89a\x1A\xC5V[\x93P\x86\x81\x11a\x11'W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7Finvalid swap: inputs x and y\0\0\0\0`D\x82\x01R`d\x01a\x03EV[a\x111\x87\x82a\x1A\xC5V[\x92Pa\x11\xB6V[`\x03T`\x02T`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x97P\x16\x94Pa\x11X\x81\x88a\x1A\xC5V[\x93P\x87\x82\x11a\x11\xA9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7Finvalid swap: inputs x and y\0\0\0\0`D\x82\x01R`d\x01a\x03EV[a\x11\xB3\x88\x83a\x1A\xC5V[\x92P[`\x04\x88\x81U`\x05\x88\x90U`@Qcp\xA0\x821`\xE0\x1B\x81R0\x91\x81\x01\x91\x90\x91R`\0\x90`\x01`\x01`\xA0\x1B\x03\x88\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x12BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x1B\xAA\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x12VW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12z\x91\x90a\x1B?V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x88\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x12\xFDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x1B\xAA\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x13\x11W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x135\x91\x90a\x1B?V[`@Qc#\xB8r\xDD`\xE0\x1B\x81R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x89\x16\x90c#\xB8r\xDD\x90a\x13h\x903\x900\x90\x8B\x90`\x04\x01a\x1AmV[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x13\xC0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x1B\xAA\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x13\xD4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\xF8\x91\x90a\x1A\x91V[P`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x86\x90R`\x01`\x01`\xA0\x1B\x03\x88\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x14\x7FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x1B\xAA\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x14\x93W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14\xB7\x91\x90a\x1A\x91V[Pa\x14\xC2\x86\x83a\x1A\xD8V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x8A\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x15?W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x1B\xAA\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x15SW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15w\x91\x90a\x1B?V[\x10\x15a\x15\xD0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: input token transf`D\x82\x01Ra2\xB9`\xF1\x1B`d\x82\x01R`\x84\x01a\x03EV[a\x15\xDA\x85\x82a\x1A\xC5V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x89\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x16WW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x1B\xAA\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x16kW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\x8F\x91\x90a\x1B?V[\x10\x15a\x16\xE9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7Finvalid swap: output token trans`D\x82\x01Rb32\xB9`\xE9\x1B`d\x82\x01R`\x84\x01a\x03EV[PPPP\x92\x95\x91\x94P\x92PV[a\x179\x81`@Q`$\x01a\x17\n\x91\x90a\x1B[V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\x10L\x13\xEB`\xE2\x1B\x17\x90Ra\x17[V[PV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x17TW`\0\x80\xFD[\x04\x92\x91PPV[a\x179\x81\x80Qjconsole.log` \x83\x01`\0\x80\x84\x83\x85Z\xFAPPPPPV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rjrray stride`\xA8\x1B`d\x82\x01R`\x84\x81\xFD[`\0\x80` \x80\x84\x86\x03\x12\x15a\x18@Wa\x18@a\x17\x80V[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x18\xA3W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x84\x90R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x19\x0BW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x84\x90R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x81\xFD[\x815\x81\x81\x11\x15a\x19nW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x85\x90R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x0ENL/$\r\x8C\xAD\xCC\xEE\x8D`\xAB\x1B`d\x82\x01R`\x84\x81\xFD[\x87\x84\x82\x85\x01\x01\x11\x15a\x19\x82Wa\x19\x82a\x17\xD0V[\x91\x90\x92\x01\x96\x90\x95P\x93PPPPV[`\0` \x82\x84\x03\x12\x15a\x19\xA6Wa\x19\xA6a\x17\x80V[\x815`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x19\xBDW`\0\x80\xFD[\x93\x92PPPV[` \x80\x82R`\x06\x90\x82\x01Re\x1B\x1B\xD8\xDA\xD9Y`\xD2\x1B`@\x82\x01R``\x01\x90V[` \x81R\x81` \x82\x01R\x81\x83`@\x83\x017`\0\x81\x83\x01`@\x90\x81\x01\x91\x90\x91R`\x1F\x90\x92\x01`\x1F\x19\x16\x01\x01\x91\x90PV[\x80Q\x80\x15\x15\x81\x14a\x10gW`\0\x80\xFD[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\x1A>Wa\x1A>a\x17\x80V[a\x1AG\x86a\x1A\x13V[` \x87\x01Q`@\x88\x01Q``\x89\x01Q`\x80\x90\x99\x01Q\x92\x9A\x91\x99P\x97\x96P\x90\x94P\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x81R\x91\x90\x92\x16` \x82\x01R`@\x81\x01\x91\x90\x91R``\x01\x90V[`\0` \x82\x84\x03\x12\x15a\x1A\xA6Wa\x1A\xA6a\x17\x80V[a\x10\x81\x82a\x1A\x13V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x10\x84Wa\x10\x84a\x1A\xAFV[\x80\x82\x01\x80\x82\x11\x15a\x10\x84Wa\x10\x84a\x1A\xAFV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a\x1B\x07Wa\x1B\x07a\x17\x80V[a\x1B\x10\x87a\x1A\x13V[\x95P` \x87\x01Q\x94P`@\x87\x01Q\x93P``\x87\x01Q\x92P`\x80\x87\x01Q\x91P`\xA0\x87\x01Q\x90P\x92\x95P\x92\x95P\x92\x95V[`\0` \x82\x84\x03\x12\x15a\x1BTWa\x1BTa\x17\x80V[PQ\x91\x90PV[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a\x1B\x88W\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a\x1BlV[P`\0`@\x82\x86\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x92PPP\x92\x91PPV\xFETarget contract does not contain`\x80`@R4\x80\x15b\0\0^W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`@Qb\0'\x038\x03\x80b\0'\x03\x839\x81\x01`@\x81\x90Rb\0\0\x81\x91b\0\x01\x19V[g\r\xE0\xB6\xB3\xA7d\0\0\x81\x10b\0\0\xF0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7Fswap fee percentage must be less`D\x82\x01Ri than 100%`\xB0\x1B`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[`\x01U`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90Ub\0\x01\xA0V[`\0\x80`@\x83\x85\x03\x12\x15b\0\x01xW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[\x82Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x01\x90W`\0\x80\xFD[` \x93\x90\x93\x01Q\x92\x94\x92\x93PPPV[a%S\x80b\0\x01\xB0`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x01\x83W`\x005`\xE0\x1C\x80c\x8C3\x88\xDC\x11a\x01\x05W\x80c\xCE\x15;\xF4\x11a\0\xC9W\x80c\xCE\x15;\xF4\x14a\x04TW\x80c\xD7\x10\x955\x14a\x04gW\x80c\xE8\0\xBDD\x14a\x04zW\x80c\xECM\xC1[\x14a\x04\x8DW\x80c\xF2\xF4\xEB&\x14a\x04\xC2W\x80c\xFC\x13\x05\xE5\x14a\x04\xEDWa\x01\x83V[\x80c\x8C3\x88\xDC\x14a\x03\x93W\x80c\x8C\xC1\xC1\xAD\x14a\x03\xD1W\x80c\x8E-\xD4\0\x14a\x03\xE4W\x80c\x9F\x83\x13{\x14a\x04!W\x80c\xA4\xD4z^\x14a\x044Wa\x01\x83V[\x80c8}\xD9\xE9\x11a\x01LW\x80c8}\xD9\xE9\x14a\x02\xF6W\x80c?C\xBA\xAC\x14a\x03$W\x80cT\xCF*\xEB\x14a\x039W\x80crQ0\x8C\x14a\x03BW\x80cy$\x17\xB1\x14a\x03UWa\x01\x83V[\x80b.RK\x14a\x01\xE8W\x80c\x08\xA4\xF0r\x14a\x02\x0EW\x80c\x1336\xAA\x14a\x02<W\x80c\x16\x98\xAAI\x14a\x02OW\x80c2\x14\x89\x0F\x14a\x02\xB4W[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x01\xFBa\x01\xF66`\x04a mV[a\x05\0V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x02!a\x02\x1C6`\x04a!eV[a\x05@V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x02\x05V[a\x01\xFBa\x02J6`\x04a!eV[a\x05mV[a\x02\x8Ca\x02]6`\x04a!eV[`\x05` R`\0\x90\x81R`@\x90 \x80T`\x01\x82\x01T`\x02\x83\x01T`\x03\x84\x01T`\x04\x90\x94\x01T\x92\x93\x91\x92\x90\x91\x90\x85V[`@\x80Q\x95\x86R` \x86\x01\x94\x90\x94R\x92\x84\x01\x91\x90\x91R``\x83\x01R`\x80\x82\x01R`\xA0\x01a\x02\x05V[a\x02\xC7a\x02\xC26`\x04a mV[a\x065V[`@\x80Q\x96\x15\x15\x87R` \x87\x01\x95\x90\x95R\x93\x85\x01\x92\x90\x92R``\x84\x01R`\x80\x83\x01R`\xA0\x82\x01R`\xC0\x01a\x02\x05V[a\x02!a\x03\x046`\x04a!eV[`\x02` \x81\x90R`\0\x91\x82R`@\x90\x91 \x80T`\x01\x82\x01T\x91\x90\x92\x01T\x83V[a\x037a\x0326`\x04a!\x81V[a\x07\xC4V[\0[a\x01\xFB`\x01T\x81V[a\x037a\x03P6`\x04a!\x81V[a\t;V[a\x02\x8Ca\x03c6`\x04a!eV[`\x04` \x81\x90R`\0\x91\x82R`@\x90\x91 \x80T`\x01\x82\x01T`\x02\x83\x01T`\x03\x84\x01T\x93\x90\x94\x01T\x91\x93\x90\x92\x90\x91\x85V[a\x02\x8Ca\x03\xA16`\x04a!eV[`\x03` \x81\x90R`\0\x91\x82R`@\x90\x91 \x80T`\x01\x82\x01T`\x02\x83\x01T\x93\x83\x01T`\x04\x90\x93\x01T\x91\x93\x90\x92\x90\x91\x85V[a\x01\xFBa\x03\xDF6`\x04a!eV[a\nwV[a\x03\xF7a\x03\xF26`\x04a!\xB0V[a\n\xCBV[`@\x80Q\x95\x15\x15\x86R` \x86\x01\x94\x90\x94R\x92\x84\x01\x91\x90\x91R``\x83\x01R`\x80\x82\x01R`\xA0\x01a\x02\x05V[a\x03\xF7a\x04/6`\x04a!\xB0V[a\x0B\x1AV[a\x04Ga\x04B6`\x04a!eV[a\x0BrV[`@Qa\x02\x05\x91\x90a\"\xDFV[a\x02!a\x04b6`\x04a!eV[a\x0B\xBFV[a\x037a\x04u6`\x04a!\x81V[a\x0C\x90V[a\x01\xFBa\x04\x886`\x04a!eV[a\r\xC6V[a\x04\xA0a\x04\x9B6`\x04a!eV[a\x0E\x1BV[`@\x80Q\x82Q\x81R` \x80\x84\x01Q\x90\x82\x01R\x91\x81\x01Q\x90\x82\x01R``\x01a\x02\x05V[`\0Ta\x04\xD5\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\x05V[a\x04\xA0a\x04\xFB6`\x04a!eV[a\x0EkV[`\0\x80`\0\x80\x84\x80` \x01\x90Q\x81\x01\x90a\x05\x1A\x91\x90a#-V[\x92P\x92P\x92Pa\x054\x83\x83\x83a\x05/\x8Aa\x0E\x1BV[a\x0E\xC8V[\x93PPPP[\x92\x91PPV[`\0\x80`\0a\x05N\x84a\r\xC6V[a\x05W\x85a\x05mV[a\x05`\x86a\nwV[\x92P\x92P\x92P\x91\x93\x90\x92PV[`\0\x81\x81R`\x03` \x81\x81R`@\x80\x84 \x81Q`\xA0\x81\x01\x83R\x81T\x81R`\x01\x82\x01T\x93\x81\x01\x93\x90\x93R`\x02\x81\x01T\x91\x83\x01\x82\x90R\x92\x83\x01T``\x83\x01R`\x04\x90\x92\x01T`\x80\x82\x01R\x90B\x10a\x05\xC3WQ\x92\x91PPV[\x80Q` \x82\x01Q\x11a\x06\x01W``\x81\x01Q`\x80\x82\x01Qa\x05\xE3\x90Ba#tV[a\x05\xED\x91\x90a#\x87V[\x81` \x01Qa\x05\xFC\x91\x90a#\x9EV[a\x06.V[``\x81\x01Q`\x80\x82\x01Qa\x06\x15\x90Ba#tV[a\x06\x1F\x91\x90a#\x87V[\x81` \x01Qa\x06.\x91\x90a#tV[\x93\x92PPPV[`\0\x80`\0\x80`\0\x80`\0\x80`\0a\x06L\x8Ba\x0B\xBFV[\x92P\x92P\x92P\x89\x80` \x01\x90Q\x81\x01\x90a\x06f\x91\x90a#-V[\x91\x97P\x95P\x93P`\0\x80\x80\x85\x89\x11\x15a\x06\xC1Wa\x06\x83\x86\x8Aa#tV[\x91Pa\x06\x9A`\x01T\x83a\x0F\xDA\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa\x06\xB0\x86a\x06\xAA\x83\x87a\x0F\xDAV[\x90a\x0F\xEFV[a\x06\xBA\x90\x84a#\x9EV[\x92Pa\x07`V[\x84\x88\x11\x15a\x06\xFAWa\x06\xD3\x85\x89a#tV[\x91Pa\x06\xEA`\x01T\x83a\x0F\xDA\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa\x06\xB0\x85a\x06\xAA\x83\x87a\x0F\xDAV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`0`$\x82\x01R\x7Finvalid swap: inputs x and y hav`D\x82\x01Roe the same sign!`\x80\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[a\x07j\x84\x88a#\xB1V[\x99P`\0a\x07w\x8Fa\x0E\x1BV[\x90Pa\x07\x85\x8A\x8A\x8A\x84a\x0E\xC8V[\x9BP`\0\x8Ca\x07\x94`\x1Ea#\xD8V[\x12\x80\x15a\x07\xA1WP`\x1E\x8D\x12[\x90P\x80\x80\x15a\x07\xB0WP\x84\x8C\x12\x15[\x9DPPPPPPPPP\x92\x95P\x92\x95P\x92\x95V[B\x81\x11a\x07\xE3W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07W\x90a#\xF4V[a\x07\xEC\x83a\x10\x04V[`\0\x83\x81R`\x05` \x90\x81R`@\x80\x83 \x81Q`\xA0\x81\x01\x83R\x81T\x81R`\x01\x82\x01T\x93\x81\x01\x84\x90R`\x02\x82\x01T\x92\x81\x01\x92\x90\x92R`\x03\x81\x01T``\x83\x01R`\x04\x01T`\x80\x82\x01R\x91\x90\x84\x10a\x08OW` \x82\x01Qa\x08J\x90\x85a#tV[a\x08_V[\x83\x82` \x01Qa\x08_\x91\x90a#tV[\x90Pa\x08kB\x84a#tV[a\x08u\x90\x82a$\x1FV[``\x83\x01\x90\x81R\x84\x83R`@\x80\x84\x01\x85\x81R`\0\x88\x81R`\x05` \x90\x81R\x92\x90 \x85Q\x80\x82U\x92\x86\x01Q`\x01\x82\x01\x81\x90U\x91Q`\x02\x82\x01\x81\x90U\x93Q`\x03\x82\x01U`\x80\x86\x01Q`\x04\x90\x91\x01U\x7F\x1EK Gg&%\xF4\xDF!\xD4|\xA7\xA0\x0C \x12\xCC\xF7x\xF2w\xBF2(\xDD\xF4Y=o\x84\xAA\x92\x81\x83\x11a\t\0W\x85Q` \x87\x01Qa\x08\xFB\x91\x90a#tV[a\t\x11V[` \x86\x01Q\x86Qa\t\x11\x91\x90a#tV[`@\x80Q\x94\x85R` \x85\x01\x93\x90\x93R\x91\x83\x01R``\x82\x01R`\x80\x01`@Q\x80\x91\x03\x90\xA1PPPPPV[B\x81\x11a\tZW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07W\x90a#\xF4V[a\tc\x83a\x10\xA1V[`\0\x83\x81R`\x03` \x81\x81R`@\x80\x84 \x81Q`\xA0\x81\x01\x83R\x81T\x81R`\x01\x82\x01T\x93\x81\x01\x84\x90R`\x02\x82\x01T\x92\x81\x01\x92\x90\x92R\x92\x83\x01T``\x82\x01R`\x04\x90\x92\x01T`\x80\x83\x01R\x90\x91\x90\x84\x10a\t\xC8W` \x82\x01Qa\t\xC3\x90\x85a#tV[a\t\xD8V[\x83\x82` \x01Qa\t\xD8\x91\x90a#tV[\x90Pa\t\xE4B\x84a#tV[a\t\xEE\x90\x82a$\x1FV[``\x83\x01\x90\x81R\x84\x83R`@\x80\x84\x01\x85\x81R`\0\x88\x81R`\x03` \x81\x81R\x93\x90\x91 \x86Q\x80\x82U\x93\x87\x01Q`\x01\x82\x01\x81\x90U\x92Q`\x02\x82\x01\x81\x90U\x94Q\x91\x81\x01\x91\x90\x91U`\x80\x86\x01Q`\x04\x90\x91\x01U\x7F\xB9hO\xE86\x87_\xAF\x05m\xB2]\xE9\x85S\x859`\xB3&\x08\x8A\x10!\xAF\xA7[\xA1`\x12d\xF5\x92\x81\x83\x11a\t\0W\x85Q` \x87\x01Qa\x08\xFB\x91\x90a#tV[`\0\x81\x81R`\x05` \x90\x81R`@\x80\x83 \x81Q`\xA0\x81\x01\x83R\x81T\x81R`\x01\x82\x01T\x93\x81\x01\x93\x90\x93R`\x02\x81\x01T\x91\x83\x01\x82\x90R`\x03\x81\x01T``\x84\x01R`\x04\x01T`\x80\x83\x01RB\x10a\x05\xC3WQ\x92\x91PPV[`\0\x80\x80\x80\x80a\n\xDD\x86\x88\x01\x88a!\x81V[\x91\x94P\x92P\x90Pa\n\xF3\x83\x83\x83a\x05/\x8Ca\x0E\x1BV[\x93P\x83a\x0B\0`\x1Ea#\xD8V[\x12\x80\x15a\x0B\rWP`\x1E\x84\x12[\x94P\x93\x97\x92\x96P\x93P\x93PV[`\0\x80\x80\x80\x80a\x0B,\x86\x88\x01\x88a$AV[`\0\x8C\x81R`\x02` \x81\x81R`@\x92\x83\x90 \x84Q\x81U\x90\x84\x01Q`\x01\x82\x01U\x92\x90\x91\x01Q\x91\x01U\x91\x94P\x92P\x90Pa\x0Bc\x88a\x11@V[a\n\xF3\x83\x83\x83a\x05/\x8Ca\x0E\x1BV[``a\x0B}\x82a\r\xC6V[a\x0B\x86\x83a\x05mV[a\x0B\x8F\x84a\nwV[`@\x80Q` \x81\x01\x94\x90\x94R\x83\x01\x91\x90\x91R``\x82\x01R`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x91\x90PV[`\0\x80T`@Qc3\x85N\xFD`\xE2\x1B\x81R`\x04\x81\x01\x84\x90R\x82\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xCE\x15;\xF4\x90`$\x01```@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0CXW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0ClW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05`\x91\x90a#-V[B\x81\x11a\x0C\xAFW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07W\x90a#\xF4V[a\x0C\xB8\x83a\x13kV[`\0\x83\x81R`\x04` \x81\x81R`@\x80\x84 \x81Q`\xA0\x81\x01\x83R\x81T\x81R`\x01\x82\x01T\x93\x81\x01\x84\x90R`\x02\x82\x01T\x92\x81\x01\x92\x90\x92R`\x03\x81\x01T``\x83\x01R\x90\x92\x01T`\x80\x83\x01R\x90\x91\x90\x84\x10a\r\x1CW` \x82\x01Qa\r\x17\x90\x85a#tV[a\r,V[\x83\x82` \x01Qa\r,\x91\x90a#tV[\x90Pa\r8B\x84a#tV[a\rB\x90\x82a$\x1FV[``\x83\x01\x90\x81R\x84\x83R`@\x80\x84\x01\x85\x81R`\0\x88\x81R`\x04` \x81\x81R\x93\x90\x91 \x86Q\x80\x82U\x93\x87\x01Q`\x01\x82\x01\x81\x90U\x92Q`\x02\x82\x01\x81\x90U\x94Q`\x03\x82\x01U`\x80\x87\x01Q\x91\x01U\x7F$\xBBEgU#\x9Cf@Y\xB5\x19\xE4c\xB4\xE5\xEA\xEF\x05\x08~\x87\xB0\xDF\xED\xF6\x06\x11iR\xAC\xEE\x92\x81\x83\x11a\t\0W\x85Q` \x87\x01Qa\x08\xFB\x91\x90a#tV[`\0\x81\x81R`\x04` \x81\x81R`@\x80\x84 \x81Q`\xA0\x81\x01\x83R\x81T\x81R`\x01\x82\x01T\x93\x81\x01\x93\x90\x93R`\x02\x81\x01T\x91\x83\x01\x82\x90R`\x03\x81\x01T``\x84\x01R\x90\x92\x01T`\x80\x82\x01R\x90B\x10a\x05\xC3WQ\x92\x91PPV[a\x0E?`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[a\x0EH\x82a\r\xC6V[a\x0EQ\x83a\x05mV[a\x0EZ\x84a\nwV[`@\x84\x01R` \x83\x01R\x81R\x91\x90PV[a\x0E\x8F`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[P`\0\x90\x81R`\x02` \x81\x81R`@\x92\x83\x90 \x83Q``\x81\x01\x85R\x81T\x81R`\x01\x82\x01T\x92\x81\x01\x92\x90\x92R\x90\x91\x01T\x91\x81\x01\x91\x90\x91R\x90V[`\0\x82\x85\x10a\x0F\x19W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FtradingFunction: invalid x\0\0\0\0\0\0`D\x82\x01R`d\x01a\x07WV[`\0\x80g\r\xE0\xB6\xB3\xA7d\0\0a\x0F/\x88\x87a\x14\x03V[\x10a\x0FCW`\x01`\x01`\xFF\x1B\x03\x91Pa\x0FXV[a\x0FUa\x0FP\x88\x87a\x14\x03V[a\x14\x18V[\x91P[g\r\xE0\xB6\xB3\xA7d\0\0a\x0Fx\x87a\x0Fs\x87`\0\x01Q\x89a\x14\xBEV[a\x14\x03V[\x10a\x0F\x8BWP`\x01`\x01`\xFF\x1B\x03a\x0F\xA3V[a\x0F\xA0a\x0FP\x87a\x0Fs\x87`\0\x01Q\x89a\x14\xBEV[\x90P[`\0a\x0F\xB7\x85` \x01Q\x86`@\x01Qa\x14\xD3V[\x90P\x80a\x0F\xC4\x83\x85a$\xFBV[a\x0F\xCE\x91\x90a$\xFBV[\x98\x97PPPPPPPPV[`\0a\x06.\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x15\x01V[`\0a\x06.\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x15\x01V[`\0\x81\x81R`\x05` \x90\x81R`@\x91\x82\x90 \x82Q`\xA0\x81\x01\x84R\x81T\x81R`\x01\x82\x01T\x92\x81\x01\x92\x90\x92R`\x02\x81\x01T\x92\x82\x01\x92\x90\x92R`\x03\x82\x01T``\x82\x01R`\x04\x90\x91\x01T`\x80\x82\x01Ra\x10X\x82a\nwV[` \x80\x83\x01\x91\x82RB`\x80\x84\x01\x90\x81R`\0\x94\x85R`\x05\x90\x91R`@\x93\x84\x90 \x83Q\x81U\x91Q`\x01\x83\x01U\x92\x82\x01Q`\x02\x82\x01U``\x90\x91\x01Q`\x03\x82\x01U\x90Q`\x04\x90\x91\x01UV[`\0\x81\x81R`\x03` \x81\x81R`@\x92\x83\x90 \x83Q`\xA0\x81\x01\x85R\x81T\x81R`\x01\x82\x01T\x92\x81\x01\x92\x90\x92R`\x02\x81\x01T\x93\x82\x01\x93\x90\x93R\x90\x82\x01T``\x82\x01R`\x04\x90\x91\x01T`\x80\x82\x01Ra\x10\xF4\x82a\x05mV[` \x80\x83\x01\x91\x82RB`\x80\x84\x01\x90\x81R`\0\x94\x85R`\x03\x91\x82\x90R`@\x94\x85\x90 \x84Q\x81U\x92Q`\x01\x84\x01U\x93\x83\x01Q`\x02\x83\x01U``\x90\x92\x01Q\x91\x81\x01\x91\x90\x91U\x90Q`\x04\x90\x91\x01UV[`\0\x81\x81R`\x02` \x81\x81R`@\x80\x84 \x81Q``\x80\x82\x01\x84R\x82T\x82R`\x01\x83\x01T\x82\x86\x01R\x91\x90\x94\x01T\x84\x83\x01R\x81Q`\xA0\x81\x01\x83R\x85\x81R\x92\x83\x01\x85\x90R\x90\x82\x01\x84\x90R\x81\x01\x83\x90R`\x80\x81\x01\x92\x90\x92R\x90a\x11\xC7`@Q\x80`\xA0\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[a\x11\xF9`@Q\x80`\xA0\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[\x83` \x01Q\x83`\0\x01\x81\x81RPP\x83` \x01Q\x83` \x01\x81\x81RPPB\x83`@\x01\x81\x81RPPB\x83`\x80\x01\x81\x81RPP\x83`\0\x01Q\x82`\0\x01\x81\x81RPP\x83`\0\x01Q\x82` \x01\x81\x81RPPB\x82`@\x01\x81\x81RPPB\x82`\x80\x01\x81\x81RPP\x83`@\x01Q\x81`\0\x01\x81\x81RPP\x83`@\x01Q\x81` \x01\x81\x81RPPB\x81`@\x01\x81\x81RPPB\x81`\x80\x01\x81\x81RPP\x82`\x03`\0\x87\x81R` \x01\x90\x81R` \x01`\0 `\0\x82\x01Q\x81`\0\x01U` \x82\x01Q\x81`\x01\x01U`@\x82\x01Q\x81`\x02\x01U``\x82\x01Q\x81`\x03\x01U`\x80\x82\x01Q\x81`\x04\x01U\x90PP\x81`\x04`\0\x87\x81R` \x01\x90\x81R` \x01`\0 `\0\x82\x01Q\x81`\0\x01U` \x82\x01Q\x81`\x01\x01U`@\x82\x01Q\x81`\x02\x01U``\x82\x01Q\x81`\x03\x01U`\x80\x82\x01Q\x81`\x04\x01U\x90PP\x80`\x05`\0\x87\x81R` \x01\x90\x81R` \x01`\0 `\0\x82\x01Q\x81`\0\x01U` \x82\x01Q\x81`\x01\x01U`@\x82\x01Q\x81`\x02\x01U``\x82\x01Q\x81`\x03\x01U`\x80\x82\x01Q\x81`\x04\x01U\x90PPPPPPPV[`\0\x81\x81R`\x04` \x81\x81R`@\x92\x83\x90 \x83Q`\xA0\x81\x01\x85R\x81T\x81R`\x01\x82\x01T\x92\x81\x01\x92\x90\x92R`\x02\x81\x01T\x93\x82\x01\x93\x90\x93R`\x03\x83\x01T``\x82\x01R\x91\x01T`\x80\x82\x01Ra\x13\xBC\x82a\r\xC6V[` \x80\x83\x01\x91\x82RB`\x80\x84\x01\x90\x81R`\0\x94\x85R`\x04\x91\x82\x90R`@\x94\x85\x90 \x84Q\x81U\x92Q`\x01\x84\x01U\x93\x83\x01Q`\x02\x83\x01U``\x90\x92\x01Q`\x03\x82\x01U\x91Q\x91\x01UV[`\0a\x06.\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x15/V[`\0g\x06\xF0[Y\xD3\xB2\0\0\x82\x03a\x141WP`\0\x91\x90PV[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x12a\x14YW`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\x14zW`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x14\x87\x83`\x02a%#V[\x90P`\0a\x14\x94\x82a\x15NV[\x90P`\0a\x14\xAAg\x13\xA0K\xBD\xFD\xC9\xBE\x88\x83a\x17\xCCV[\x90Pa\x14\xB5\x81a#\xD8V[\x95\x94PPPPPV[`\0a\x06.\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x15/V[`\0\x80a\x14\xDF\x83a\x17\xE1V[a\x14\xED\x90c;\x9A\xCA\0a#\x87V[\x90Pa\x14\xF9\x84\x82a\x14\xBEV[\x94\x93PPPPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x15\x19W`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x15GW`\0\x80\xFD[\x04\x92\x91PPV[`\0\x80\x82\x12\x80a\x15eWPg\x1B\xC1mgN\xC8\0\0\x82\x13[\x15a\x15\x83W`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\x15\xA4W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81g\x1B\xC1mgN\xC8\0\0\x03a\x15\xCCW`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x15a\x15\xD7W\x91\x90PV[`\0g\r\xE0\xB6\xB3\xA7d\0\0\x83\x12a\x15\xFFWa\x15\xFA\x83g\x1B\xC1mgN\xC8\0\0a#\xB1V[a\x16\x01V[\x82[\x90P`\0a\x16\x17\x82g\x1B\xC1mgN\xC8\0\0a\x18\x85V[\x90P\x80`\0\x03a\x16:W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x16E\x82a\x18\x9AV[\x90P`\0c;\x9A\xCA\0a\x16pa\x16ka\x16eg\x1B\xC1mgN\xC8\0\0a#\xD8V[\x85a\x17\xCCV[a\x17\xE1V[a\x16z\x91\x90a%#V[\x90P`\0\x80a\x16\x91\x83g\x03\xC1f\\z\xAB \0a\x17\xCCV[a\x16\xA3\x90g \x05\xFEO&\x8E\xA0\0a$\xFBV[\x90P`\0a\x16\xD3\x84a\x16\xBC\x86f\x9F2u$b\xA0\0a\x17\xCCV[a\x16\xCE\x90g\r\xC5R\x7Fd, \0a$\xFBV[a\x17\xCCV[a\x16\xE5\x90g\r\xE0\xB6\xB3\xA7d\0\0a$\xFBV[\x90Pa\x17\tg\t\xD0(\xCCo _\xFF\x19\x85a\x16\xFF\x85\x85a\x18\x85V[a\x16\xCE\x91\x90a#\xB1V[\x92PPP`\0[`\x02\x81\x10\x15a\x17\xA4W`\0\x86a\x17%\x84a\x1AuV[a\x17/\x91\x90a#\xB1V[\x90P`\0a\x17=\x84\x85a\x17\xCCV[a\x17F\x90a#\xD8V[\x90P`\0a\x17S\x82a\x1CYV[\x90P`\0a\x17a\x86\x85a\x17\xCCV[a\x17sg\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x84a\x17\xCCV[a\x17}\x91\x90a#\xB1V[\x90Pa\x17\x89\x84\x82a\x18\x85V[a\x17\x93\x90\x87a$\xFBV[\x95P\x84`\x01\x01\x94PPPPPa\x17\x10V[g\r\xE0\xB6\xB3\xA7d\0\0\x88\x12a\x17\xC1Wa\x17\xBC\x82a#\xD8V[a\x0F\xCEV[P\x96\x95PPPPPPV[`\0a\x06.\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x1E\x02V[`\xB5\x81`\x01`\x88\x1B\x81\x10a\x17\xFAW`@\x91\x90\x91\x1B\x90`\x80\x1C[i\x01\0\0\0\0\0\0\0\0\0\x81\x10a\x18\x16W` \x91\x90\x91\x1B\x90`@\x1C[e\x01\0\0\0\0\0\x81\x10a\x18.W`\x10\x91\x90\x91\x1B\x90` \x1C[c\x01\0\0\0\x81\x10a\x18DW`\x08\x91\x90\x91\x1B\x90`\x10\x1C[b\x01\0\0\x01\x02`\x12\x1C\x80\x82\x04\x01`\x01\x90\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x90\x81\x90\x04\x81\x11\x90\x03\x90V[`\0a\x06.\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x1E\x02V[`\0\x80\x82\x13a\x18\xD7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x07WV[`\0``a\x18\xE4\x84a\x1E!V[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[`\0\x81`\0\x03a\x1A\x8EWPg\r\xE0\xB6\xB3\xA7d\0\0\x91\x90PV[gV\x98\xEE\xF0fp\0\0\x82\x12a\x1A\xA5WP`\0\x91\x90PV[a\x1A\xB6gV\x98\xEE\xF0fp\0\0a#\xD8V[\x82\x13a\x1A\xCBWPg\x1B\xC1mgN\xC8\0\0\x91\x90PV[`\0a\x1A\xD6\x83a\x1E\xC9V[\x90P`\0a\x1B\x0Fg\r\xE0\xB6\xB3\xA7d\0\0a\x1A\xF8\x84g\x1B\xC1mgN\xC8\0\0a\x14\x03V[a\x1B\n\x90g\r\xE0\xB6\xB3\xA7d\0\0a$\xFBV[a\x18\x85V[\x90P`\0\x80\x82a\x1Bk\x81a\x1BX\x81a\x1BF\x81a\x1B3\x81g\x02_\x0F\xE1\x05\xA3\x14\0a\x17\xCCV[a\x16\xCE\x90g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19a$\xFBV[a\x16\xCE\x90g\x14\xA8EL\x19\xE1\xAC\0a$\xFBV[a\x16\xCE\x90g\x0F\xC1\x0E\x01W\x82w\xFF\x19a$\xFBV[a\x1B}\x90g\x03\xDE\xBD\x08;\x8C|\0a$\xFBV[\x91P\x83\x90Pa\x1B\xE5\x81a\x1B\xD3\x81a\x1B\xC1\x81a\x1B\xAF\x81a\x1B\x9C\x81\x8Ba\x17\xCCV[a\x16\xCE\x90g\x02\x95\xD4\0\xEA2W\xFF\x19a$\xFBV[a\x16\xCE\x90g\x01W\xD8\xB2\xEC\xC7\x08\0a$\xFBV[a\x16\xCE\x90g\x051\n\xA7\xD5!0\0a$\xFBV[a\x16\xCE\x90g\r\xE0\xCC=\x15a\0\0a$\xFBV[\x91P\x81g\x11\x90\0\xAB\x10\x0F\xFC\0a\x1B\xFB\x87\x88a\x17\xCCV[a\x1C\x07\x90`\0\x19a%#V[a\x1C\x11\x91\x90a#\xB1V[a\x1C\x1B\x91\x90a$\xFBV[\x92PP`\0a\x1C)\x83a\x1CYV[\x90P`\0a\x1C7\x85\x83a\x17\xCCV[\x90P`\0\x88\x12a\x1CGW\x80a\x0F\xCEV[a\x0F\xCE\x81g\x1B\xC1mgN\xC8\0\0a#\xB1V[`\0h\x02H\xCE6\xA7\x0C\xB2k>\x19\x82\x13a\x1CtWP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a\x1C\xBBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x01a\x07WV[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05`\x01`_\x1B\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1Dm\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1Dn\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x05\x85\x14\x17\x16a\x1E\x1AW`\0\x80\xFD[\x05\x92\x91PPV[`\0\x80\x82\x11a\x1E^W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x07WV[P`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x82\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[`\0`\x01`\xFF\x1B\x82\x03a\x1E\xEFW`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x12\x15a\x1F\0WP\x19`\x01\x01\x90V[P\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x81\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a 6Wa 6a\x1F\xFDV[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a eWa ea\x1F\xFDV[`@R\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a \x83Wa \x83a\x1F\x04V[\x825\x91P` \x80\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a \xA6Wa \xA6a\x1FTV[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a \xBDWa \xBDa\x1F\xA4V[\x815\x81\x81\x11\x15a \xCFWa \xCFa\x1F\xFDV[a \xE1`\x1F\x82\x01`\x1F\x19\x16\x85\x01a <V[\x91P\x80\x82R\x87\x84\x82\x85\x01\x01\x11\x15a!GW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x85\x90R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R`\x84\x81\xFD[\x80\x84\x84\x01\x85\x84\x017`\0\x84\x82\x84\x01\x01RP\x80\x93PPPP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a!zWa!za\x1F\x04V[P5\x91\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a!\x99Wa!\x99a\x1F\x04V[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[`\0\x80`\0`@\x84\x86\x03\x12\x15a!\xC8Wa!\xC8a\x1F\x04V[\x835\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a!\xEAWa!\xEAa\x1FTV[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\"\x01Wa\"\x01a\x1F\xA4V[\x815\x81\x81\x11\x15a\"dW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x0ENL/$\r\x8C\xAD\xCC\xEE\x8D`\xAB\x1B`d\x82\x01R`\x84\x81\xFD[\x87` \x82\x85\x01\x01\x11\x15a\"\xCCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rjrray stride`\xA8\x1B`d\x82\x01R\x91P`\x84\x82\xFD[` \x83\x01\x94P\x80\x93PPPP\x92P\x92P\x92V[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a#\x0CW\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a\"\xF0V[P`\0`@\x82\x86\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x92PPP\x92\x91PPV[`\0\x80`\0``\x84\x86\x03\x12\x15a#EWa#Ea\x1F\x04V[\x83Q\x92P` \x84\x01Q\x91P`@\x84\x01Q\x90P\x92P\x92P\x92V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x05:Wa\x05:a#^V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x05:Wa\x05:a#^V[\x80\x82\x01\x80\x82\x11\x15a\x05:Wa\x05:a#^V[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a#\xD1Wa#\xD1a#^V[P\x92\x91PPV[`\0`\x01`\xFF\x1B\x82\x01a#\xEDWa#\xEDa#^V[P`\0\x03\x90V[` \x80\x82R`\x11\x90\x82\x01Rp\x15\\\x19\x18]\x19H\x19[\x99\x08\x1C\x18\\\xDC\xD9Y`z\x1B`@\x82\x01R``\x01\x90V[`\0\x82a$<WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[`\0\x80`\0\x80\x84\x86\x03`\xC0\x81\x12\x15a$[Wa$[a\x1F\x04V[\x855\x94P` \x86\x015\x93P`@\x86\x015\x92P```_\x19\x82\x01\x12\x15a$\xCBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FABI decoding: struct data too sh`D\x82\x01Rb\x1B\xDC\x9D`\xEA\x1B`d\x82\x01R`\x84\x81\xFD[Pa$\xD4a \x13V[``\x86\x015\x81R`\x80\x86\x015` \x82\x01R`\xA0\x90\x95\x015`@\x86\x01RP\x91\x94\x90\x93P\x90\x91\x90V[\x80\x82\x01\x82\x81\x12`\0\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a%\x1BWa%\x1Ba#^V[PP\x92\x91PPV[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15a%?Wa%?a#^V[\x81\x81\x05\x83\x14\x82\x15\x17a\x05:Wa\x05:a#^V`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`@Qa\x19\x818\x03\x80a\x19\x81\x839\x81\x01`@\x81\x90Ra\0|\x91a\x01\x12V[g\r\xE0\xB6\xB3\xA7d\0\0\x81\x10a\0\xEAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7Fswap fee percentage must be less`D\x82\x01Ri than 100%`\xB0\x1B`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[`\x01U`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90Ua\x01\x97V[`\0\x80`@\x83\x85\x03\x12\x15a\x01pW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[\x82Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01\x87W`\0\x80\xFD[` \x93\x90\x93\x01Q\x92\x94\x92\x93PPPV[a\x17\xDB\x80a\x01\xA6`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x01LW`\x005`\xE0\x1C\x80c\x9F\x83\x13{\x11a\0\xE4W\x80c\xCE\x15;\xF4\x11a\0\xB3W\x80c\xCE\x15;\xF4\x14a\x03\x82W\x80c\xECM\xC1[\x14a\x03\xB0W\x80c\xF2\xF4\xEB&\x14a\x03\xDEW\x80c\xFC\x13\x05\xE5\x14a\x04\tWa\x01LV[\x80c\x9F\x83\x13{\x14a\x02\xD6W\x80c\xA4\x13\x0B\xD9\x14a\x02\xE9W\x80c\xA4\xD4z^\x14a\x02\xFCW\x80c\xB5\xF1c\xFF\x14a\x03\x1CWa\x01LV[\x80cT\xCF*\xEB\x11a\x01 W\x80cT\xCF*\xEB\x14a\x02hW\x80cT\xDEh\xF3\x14a\x02qW\x80c\x8E-\xD4\0\x14a\x02\x86W\x80c\x92\x871\xED\x14a\x02\xC3Wa\x01LV[\x80b.RK\x14a\x01\xB1W\x80c\x08\xA4\xF0r\x14a\x01\xD7W\x80c2\x14\x89\x0F\x14a\x01\xFFW\x80c8}\xD9\xE9\x14a\x02AW[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x01\xC4a\x01\xBF6`\x04a\x13\x1DV[a\x04UV[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\xEAa\x01\xE56`\x04a\x14\x15V[a\x04\x95V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x01\xCEV[a\x02\x12a\x02\r6`\x04a\x13\x1DV[a\x04\xB3V[`@\x80Q\x96\x15\x15\x87R` \x87\x01\x95\x90\x95R\x93\x85\x01\x92\x90\x92R``\x84\x01R`\x80\x83\x01R`\xA0\x82\x01R`\xC0\x01a\x01\xCEV[a\x01\xEAa\x02O6`\x04a\x14\x15V[`\x02` R`\0\x90\x81R`@\x90 \x80T`\x01\x90\x91\x01T\x82V[a\x01\xC4`\x01T\x81V[a\x02\x84a\x02\x7F6`\x04a\x141V[a\x06\x93V[\0[a\x02\x99a\x02\x946`\x04a\x14`V[a\x083V[`@\x80Q\x95\x15\x15\x86R` \x86\x01\x94\x90\x94R\x92\x84\x01\x91\x90\x91R``\x83\x01R`\x80\x82\x01R`\xA0\x01a\x01\xCEV[a\x01\xC4a\x02\xD16`\x04a\x14\x15V[a\x08\x82V[a\x02\x99a\x02\xE46`\x04a\x14`V[a\t9V[a\x01\xC4a\x02\xF76`\x04a\x14\x15V[a\njV[a\x03\x0Fa\x03\n6`\x04a\x14\x15V[a\n\x87V[`@Qa\x01\xCE\x91\x90a\x15\x8FV[a\x03Za\x03*6`\x04a\x14\x15V[`\x03` \x81\x90R`\0\x91\x82R`@\x90\x91 \x80T`\x01\x82\x01T`\x02\x83\x01T\x93\x83\x01T`\x04\x90\x93\x01T\x91\x93\x90\x92\x90\x91\x85V[`@\x80Q\x95\x86R` \x86\x01\x94\x90\x94R\x92\x84\x01\x91\x90\x91R``\x83\x01R`\x80\x82\x01R`\xA0\x01a\x01\xCEV[a\x03\x95a\x03\x906`\x04a\x14\x15V[a\n\xC3V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x01\xCEV[a\x03\xC3a\x03\xBE6`\x04a\x14\x15V[a\x0B\xA1V[`@\x80Q\x82Q\x81R` \x92\x83\x01Q\x92\x81\x01\x92\x90\x92R\x01a\x01\xCEV[`\0Ta\x03\xF1\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\xCEV[a\x03\xC3a\x04\x176`\x04a\x14\x15V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01RP`\0\x90\x81R`\x02` \x90\x81R`@\x91\x82\x90 \x82Q\x80\x84\x01\x90\x93R\x80T\x83R`\x01\x01T\x90\x82\x01R\x90V[`\0\x80`\0\x80\x84\x80` \x01\x90Q\x81\x01\x90a\x04o\x91\x90a\x15\xDDV[\x92P\x92P\x92Pa\x04\x89\x83\x83\x83a\x04\x84\x8Aa\x0B\xA1V[a\x0B\xD3V[\x93PPPP[\x92\x91PPV[`\0\x80a\x04\xA1\x83a\x08\x82V[a\x04\xAA\x84a\njV[\x91P\x91P\x91P\x91V[`\0\x80`\0\x80`\0\x80`\0\x80a\x04\xC8\x8Aa\n\xC3V[P\x91P\x91P`\0a\x04\xE2\x83\x83a\x04\xDD\x8Ea\x0B\xA1V[a\x0C\"V[\x90P\x89\x80` \x01\x90Q\x81\x01\x90a\x04\xF8\x91\x90a\x15\xDDV[\x91\x97P\x95P\x93P`\0\x80\x80\x8D\x86\x8A\x11\x15a\x05uWa\x05\x16\x87\x8Ba\x16$V[\x92Pa\x05-`\x01T\x84a\x0CY\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x91P`\0a\x05Ma\x05=\x83a\x08\x82V[a\x05G\x8A\x8Aa\x0CuV[\x90a\x0C\x8AV[\x90Pa\x05c\x88a\x05]\x85\x84a\x0CYV[\x90a\x0CuV[a\x05m\x90\x86a\x167V[\x94PPa\x06.V[\x85\x89\x11\x15a\x05\xC8Wa\x05\x87\x86\x8Aa\x16$V[\x92Pa\x05\x9E`\x01T\x84a\x0CY\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x91P`\0a\x05\xB8a\x05\xAE\x83a\njV[a\x05G\x89\x8Ba\x0CuV[\x90Pa\x05c\x87a\x05]\x85\x84a\x0CYV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`0`$\x82\x01R\x7Finvalid swap: inputs x and y hav`D\x82\x01Roe the same sign!`\x80\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[a\x068\x85\x89a\x16JV[\x9AP`\0a\x06E\x82a\x0B\xA1V[\x90Pa\x06S\x8B\x8B\x8B\x84a\x0B\xD3V[\x9CP`\0\x8Da\x06b`\x1Ea\x16qV[\x12\x80\x15a\x06oWP`\x1E\x8E\x12[\x90P\x80\x80\x15a\x06~WP\x85\x8D\x12\x15[\x9EPPPPPPPPPP\x92\x95P\x92\x95P\x92\x95V[B\x81\x11a\x06\xD6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01Rp\x15\\\x19\x18]\x19H\x19[\x99\x08\x1C\x18\\\xDC\xD9Y`z\x1B`D\x82\x01R`d\x01a\x06%V[a\x06\xDF\x83a\x0C\xBBV[`\0\x83\x81R`\x03` \x81\x81R`@\x80\x84 \x81Q`\xA0\x81\x01\x83R\x81T\x81R`\x01\x82\x01T\x93\x81\x01\x84\x90R`\x02\x82\x01T\x92\x81\x01\x92\x90\x92R\x92\x83\x01T``\x82\x01R`\x04\x90\x92\x01T`\x80\x83\x01R\x90\x91\x90\x84\x10a\x07DW` \x82\x01Qa\x07?\x90\x85a\x16$V[a\x07TV[\x83\x82` \x01Qa\x07T\x91\x90a\x16$V[\x90Pa\x07`B\x84a\x16$V[a\x07j\x90\x82a\x16\xA3V[``\x83\x01\x90\x81R\x84\x83R`@\x80\x84\x01\x85\x81R`\0\x88\x81R`\x03` \x81\x81R\x93\x90\x91 \x86Q\x80\x82U\x93\x87\x01Q`\x01\x82\x01\x81\x90U\x92Q`\x02\x82\x01\x81\x90U\x94Q\x91\x81\x01\x91\x90\x91U`\x80\x86\x01Q`\x04\x90\x91\x01U\x7F\t\xD2\x95\xF6\x80Kn^ql\xA8\x8D\xBB\xB1V\xCF\xC7\x80\xE5l+\xF1\xD6!ii\xF6g\x9E\xEFm\xF8\x92\x81\x83\x11a\x07\xF8W\x85Q` \x87\x01Qa\x07\xF3\x91\x90a\x16$V[a\x08\tV[` \x86\x01Q\x86Qa\x08\t\x91\x90a\x16$V[`@\x80Q\x94\x85R` \x85\x01\x93\x90\x93R\x91\x83\x01R``\x82\x01R`\x80\x01`@Q\x80\x91\x03\x90\xA1PPPPPV[`\0\x80\x80\x80\x80a\x08E\x86\x88\x01\x88a\x141V[\x91\x94P\x92P\x90Pa\x08[\x83\x83\x83a\x04\x84\x8Ca\x0B\xA1V[\x93P\x83a\x08h`\x1Ea\x16qV[\x12\x80\x15a\x08uWP`\x1E\x84\x12[\x94P\x93\x97\x92\x96P\x93P\x93PV[`\0\x81\x81R`\x03` \x81\x81R`@\x80\x84 \x81Q`\xA0\x81\x01\x83R\x81T\x81R`\x01\x82\x01T\x93\x81\x01\x93\x90\x93R`\x02\x81\x01T\x91\x83\x01\x82\x90R\x92\x83\x01T``\x83\x01R`\x04\x90\x92\x01T`\x80\x82\x01R\x90B\x10a\x08\xD8WQ\x92\x91PPV[`\0\x81`\x80\x01QBa\x08\xEA\x91\x90a\x16$V[\x90P`\0\x82``\x01Q\x82a\x08\xFE\x91\x90a\x16\xB7V[\x90P\x82`\0\x01Q\x83` \x01Q\x11\x15a\t)W\x80\x83` \x01Qa\t \x91\x90a\x16$V[\x95\x94PPPPPV[\x80\x83` \x01Qa\t \x91\x90a\x167V[`\0\x80\x80\x80\x80a\tK\x86\x88\x01\x88a\x16\xCEV[`\0\x8C\x81R`\x02` \x90\x81R`@\x90\x91 \x82Q\x80\x82U\x92\x90\x91\x01Q`\x01\x90\x91\x01\x81\x90U\x93\x96P\x91\x94P\x92Pg\r\xE0\xB6\xB3\xA7d\0\0\x91a\t\x89\x91a\x167V[\x14a\t\xC8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01RnInvalid weights`\x88\x1B`D\x82\x01R`d\x01a\x06%V[a\n[\x88`\0\x81\x81R`\x02` \x81\x81R`@\x80\x84 \x81Q\x80\x83\x01\x83R\x81T\x81R`\x01\x91\x82\x01T\x81\x85\x01R\x82Q`\xA0\x81\x01\x84R\x86\x81R\x80\x85\x01\x87\x81R\x81\x85\x01\x88\x81R``\x83\x01\x89\x81R`\x80\x84\x01\x8A\x81R\x85Q\x85R\x94Q\x83RB\x80\x83R\x85R\x99\x89R`\x03\x96\x87\x90R\x94\x90\x97 \x90Q\x81U\x95Q\x91\x86\x01\x91\x90\x91U\x90Q\x92\x84\x01\x92\x90\x92U\x92Q\x92\x82\x01\x92\x90\x92U\x90Q`\x04\x90\x91\x01UV[a\x08[\x83\x83\x83a\x04\x84\x8Ca\x0B\xA1V[`\0a\nu\x82a\x08\x82V[a\x04\x8F\x90g\r\xE0\xB6\xB3\xA7d\0\0a\x16$V[``a\n\x92\x82a\x08\x82V[a\n\x9B\x83a\njV[`@\x80Q` \x81\x01\x93\x90\x93R\x82\x01R``\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x91\x90PV[`\0\x80T`@Qc3\x85N\xFD`\xE2\x1B\x81R`\x04\x81\x01\x84\x90R\x82\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xCE\x15;\xF4\x90`$\x01```@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0B\\W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0BpW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\x94\x91\x90a\x15\xDDV[\x92P\x92P\x92P\x91\x93\x90\x92PV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x0B\xBE\x82a\x08\x82V[\x81Ra\x0B\xC9\x82a\njV[` \x82\x01R\x91\x90PV[\x80Q`\0\x90\x81\x90a\x0B\xE5\x90\x87\x90a\x0C\x8AV[\x90P`\0a\x0C\0\x84` \x01Q\x87a\x0C\x8A\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x84a\x0C\r\x83\x83a\x0CYV[a\x0C\x17\x91\x90a\x16JV[\x97\x96PPPPPPPV[`\0a\x0CQa\x0C>\x83` \x01Q\x85a\x0C\x8A\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x83Qa\x0CK\x90\x87\x90a\x0C\x8AV[\x90a\x0CYV[\x94\x93PPPPV[`\0a\x0Cn\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\rZV[\x93\x92PPPV[`\0a\x0Cn\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\rZV[`\0a\x0Cng\r\xE0\xB6\xB3\xA7d\0\0\x83a\x0C\xA2\x86a\r\x88V[a\x0C\xAC\x91\x90a\x17}V[a\x0C\xB6\x91\x90a\x17\xADV[a\x0FcV[`\0\x81\x81R`\x03` \x81\x81R`@\x92\x83\x90 \x83Q`\xA0\x81\x01\x85R\x81T\x81R`\x01\x82\x01T\x92\x81\x01\x92\x90\x92R`\x02\x81\x01T\x93\x82\x01\x93\x90\x93R\x90\x82\x01T``\x82\x01R`\x04\x90\x91\x01T`\x80\x82\x01Ra\r\x0E\x82a\x08\x82V[` \x80\x83\x01\x91\x82RB`\x80\x84\x01\x90\x81R`\0\x94\x85R`\x03\x91\x82\x90R`@\x94\x85\x90 \x84Q\x81U\x92Q`\x01\x84\x01U\x93\x83\x01Q`\x02\x83\x01U``\x90\x92\x01Q\x91\x81\x01\x91\x90\x91U\x90Q`\x04\x90\x91\x01UV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\rrW`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[`\0\x80\x82\x13a\r\xC5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x06%V[`\0``a\r\xD2\x84a\x11\x0CV[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[`\0h\x02H\xCE6\xA7\x0C\xB2k>\x19\x82\x13a\x0F~WP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a\x0F\xC5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x01a\x06%V[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05`\x01`_\x1B\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1Dm\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1Dn\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[`\0\x80\x82\x11a\x11IW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x06%V[P`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x82\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x81\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x12\xE6Wa\x12\xE6a\x12\xADV[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x13\x15Wa\x13\x15a\x12\xADV[`@R\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x133Wa\x133a\x11\xB4V[\x825\x91P` \x80\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x13VWa\x13Va\x12\x04V[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x13mWa\x13ma\x12TV[\x815\x81\x81\x11\x15a\x13\x7FWa\x13\x7Fa\x12\xADV[a\x13\x91`\x1F\x82\x01`\x1F\x19\x16\x85\x01a\x12\xECV[\x91P\x80\x82R\x87\x84\x82\x85\x01\x01\x11\x15a\x13\xF7W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x85\x90R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R`\x84\x81\xFD[\x80\x84\x84\x01\x85\x84\x017`\0\x84\x82\x84\x01\x01RP\x80\x93PPPP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\x14*Wa\x14*a\x11\xB4V[P5\x91\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x14IWa\x14Ia\x11\xB4V[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[`\0\x80`\0`@\x84\x86\x03\x12\x15a\x14xWa\x14xa\x11\xB4V[\x835\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x14\x9AWa\x14\x9Aa\x12\x04V[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x14\xB1Wa\x14\xB1a\x12TV[\x815\x81\x81\x11\x15a\x15\x14W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x0ENL/$\r\x8C\xAD\xCC\xEE\x8D`\xAB\x1B`d\x82\x01R`\x84\x81\xFD[\x87` \x82\x85\x01\x01\x11\x15a\x15|W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rjrray stride`\xA8\x1B`d\x82\x01R\x91P`\x84\x82\xFD[` \x83\x01\x94P\x80\x93PPPP\x92P\x92P\x92V[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a\x15\xBCW\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a\x15\xA0V[P`\0`@\x82\x86\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x92PPP\x92\x91PPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x15\xF5Wa\x15\xF5a\x11\xB4V[\x83Q\x92P` \x84\x01Q\x91P`@\x84\x01Q\x90P\x92P\x92P\x92V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x04\x8FWa\x04\x8Fa\x16\x0EV[\x80\x82\x01\x80\x82\x11\x15a\x04\x8FWa\x04\x8Fa\x16\x0EV[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a\x16jWa\x16ja\x16\x0EV[P\x92\x91PPV[`\0`\x01`\xFF\x1B\x82\x01a\x16\x86Wa\x16\x86a\x16\x0EV[P`\0\x03\x90V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a\x16\xB2Wa\x16\xB2a\x16\x8DV[P\x04\x90V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x04\x8FWa\x04\x8Fa\x16\x0EV[`\0\x80`\0\x80\x84\x86\x03`\xA0\x81\x12\x15a\x16\xE8Wa\x16\xE8a\x11\xB4V[\x855\x94P` \x86\x015\x93P`@\x80\x87\x015\x93P`_\x19\x82\x01\x12\x15a\x17WW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FABI decoding: struct data too sh`D\x82\x01Rb\x1B\xDC\x9D`\xEA\x1B`d\x82\x01R`\x84\x81\xFD[Pa\x17`a\x12\xC3V[``\x86\x015\x81R`\x80\x90\x95\x015` \x86\x01RP\x91\x94\x90\x93P\x90\x91\x90V[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15a\x17\x99Wa\x17\x99a\x16\x0EV[\x81\x81\x05\x83\x14\x82\x15\x17a\x04\x8FWa\x04\x8Fa\x16\x0EV[`\0\x82a\x17\xBCWa\x17\xBCa\x16\x8DV[`\x01`\xFF\x1B\x82\x14`\0\x19\x84\x14\x16\x15a\x17\xD6Wa\x17\xD6a\x16\x0EV[P\x05\x90V";
    /// The bytecode of the contract.
    pub static DFMM_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x01MW`\x005`\xE0\x1C\x80c\xA8\xC6.v\x11a\0\xE4W\x80c\xEA\xE5\xEA\x08\x11a\0\xB3W\x80c\xEA\xE5\xEA\x08\x14a\x02\xEBW\x80c\xEB\xAD\xEF\x01\x14a\x02\xF4W\x80c\xF0e\x95\x7F\x14a\x03\x02W\x80c\xFA\n4^\x14a\x03\x0BWa\x01MV[\x80c\xA8\xC6.v\x14a\x02\x9CW\x80c\xB7\xD1\x9F\xC4\x14a\x02\xAFW\x80c\xBA\xD5&O\x14a\x02\xC2W\x80c\xCF0\x90\x12\x14a\x02\xE2Wa\x01MV[\x80cZ\xEFFz\x11a\x01 W\x80cZ\xEFFz\x14a\x02KW\x80c^\"}X\x14a\x02^W\x80cb}\xD5j\x14a\x02gW\x80cp\xA0\x821\x14a\x02|Wa\x01MV[\x80c\x15w\x0F\x92\x14a\x01\xB2W\x80c\x16\xDC\x16[\x14a\x01\xCEW\x80cC\xC8\x85\xBA\x14a\x01\xF9W\x80cM\xDFG\xD4\x14a\x02\x1DW[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x01\xBB`\x06T\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[`\x02Ta\x01\xE1\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\xC5V[`\0Ta\x02\r\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x01\xC5V[a\x020a\x02+6`\x04a\x18)V[a\x03\x1EV[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x01\xC5V[a\x020a\x02Y6`\x04a\x18)V[a\x06\x96V[a\x01\xBB`\x05T\x81V[a\x02za\x02u6`\x04a\x18)V[a\n\x96V[\0[a\x01\xBBa\x02\x8A6`\x04a\x19\x91V[`\x08` R`\0\x90\x81R`@\x90 T\x81V[`\0Ta\x01\xE1\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x03Ta\x01\xE1\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01\xBBa\x02\xD06`\x04a\x19\x91V[`\t` R`\0\x90\x81R`@\x90 T\x81V[a\x01\xBB`\x01T\x81V[a\x01\xBB`\x07T\x81V[`\x04T`\x05T`\x06Ta\x020V[a\x01\xBB`\x04T\x81V[a\x020a\x03\x196`\x04a\x18)V[a\x0C\x13V[`\0\x80`\0`\x01T`\x01\x14a\x03NW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03E\x90a\x19\xC4V[`@Q\x80\x91\x03\x90\xFD[`\0`\x01\x81\x90U\x80T`@Qc\x13w\xD1\xF5`\xE2\x1B\x81R\x82\x91\x82\x91\x82\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cM\xDFG\xD4\x90a\x03\x8E\x90\x8D\x90\x8D\x90`\x04\x01a\x19\xE4V[`\xA0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x03\xE6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x1B\xAA\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x03\xFAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\x1E\x91\x90a\x1A#V[\x94P\x94P\x94P\x94P\x94P\x84a\x04\\W`\0\x84\x12a\x04:\x85a\x10-V[`@Qcw`m)`\xE1\x1B\x81R\x91\x15\x15`\x04\x83\x01R`$\x82\x01R`D\x01a\x03EV[`\0\x80T`\xFF`\xA0\x1B\x19\x16`\x01`\xA0\x1B\x17\x81U`\x04\x84\x81U`\x05\x84\x90U`\x06\x83\x90U3\x80\x83R`\x08` \x90\x81R`@\x80\x85 \x86\x90Ug\r\xE0\xB6\xB3\xA7d\0\0`\x07\x81\x90U`\t\x90\x92R\x93\x84\x90 U`\x02T\x92Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x92c#\xB8r\xDD\x92a\x04\xDA\x92\x910\x91\x89\x91\x01a\x1AmV[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x052W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x1B\xAA\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x05FW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05j\x91\x90a\x1A\x91V[P`\x03T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\x05\x9F\x903\x900\x90\x87\x90`\x04\x01a\x1AmV[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x05\xF7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x1B\xAA\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x06\x0BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06/\x91\x90a\x1A\x91V[P`\0T`@\x80Q\x85\x81R` \x81\x01\x85\x90R\x90\x81\x01\x83\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x903\x90\x7F?\x1Fc\xA0\x1B[6Ff\x17!O4 E\xD7SS\x025<\xE7\t\xC2\x9E\xD3\xECL\x02H\xBD5\x90``\x01`@Q\x80\x91\x03\x90\xA3`\x01\x80U\x91\x99\x90\x98P\x90\x96P\x94PPPPPV[`\0\x80`\0`\x01T`\x01\x14a\x06\xBDW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03E\x90a\x19\xC4V[`\0`\x01\x81\x90U\x80T`@QcH}k\xAB`\xE1\x1B\x81R\x82\x91\x82\x91\x82\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x90\xFA\xD7V\x90a\x06\xFD\x90\x8D\x90\x8D\x90`\x04\x01a\x19\xE4V[`\xA0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x07SW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x1B\xAA\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x07gW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x8B\x91\x90a\x1A#V[\x94P\x94P\x94P\x94P\x94P\x84a\x07\xA7W`\0\x84\x12a\x04:\x85a\x10-V[3`\0\x90\x81R`\x08` R`@\x90 T\x15\x80\x15\x90a\x07\xD6WP3`\0\x90\x81R`\t` R`@\x90 T`\x07T\x14\x15[\x15a\x08&W3`\0\x90\x81R`\t` R`@\x81 T`\x07Ta\x07\xF7\x91a\x10lV[3`\0\x90\x81R`\x08` R`@\x90 T\x90\x91Pa\x08\x14\x90\x82a\x10lV[3`\0\x90\x81R`\x08` R`@\x90 UP[`\0`\x04T\x84a\x086\x91\x90a\x1A\xC5V[\x90P`\0`\x05T\x84a\x08H\x91\x90a\x1A\xC5V[\x90P`\0`\x06T\x84a\x08Z\x91\x90a\x1A\xC5V[`\x04\x87\x90U`\x05\x86\x90U`\x06\x85\x90U3`\0\x90\x81R`\x08` R`@\x81 \x80T\x92\x93P\x83\x92\x90\x91\x90a\x08\x8D\x90\x84\x90a\x1A\xD8V[\x90\x91UPP`\x07T3`\0\x81\x81R`\t` R`@\x90\x81\x90 \x92\x90\x92U`\x02T\x91Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c#\xB8r\xDD\x91a\x08\xDB\x910\x90\x88\x90`\x04\x01a\x1AmV[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\t3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x1B\xAA\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\tGW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\tk\x91\x90a\x1A\x91V[P`\x03T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90a\t\xA0\x903\x900\x90\x87\x90`\x04\x01a\x1AmV[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\t\xF8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x1B\xAA\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\n\x0CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n0\x91\x90a\x1A\x91V[P`@\x80Q\x84\x81R` \x81\x01\x84\x90R\x90\x81\x01\x82\x90R\x7F\x92\x8F-\xEA\x06G\xB6\xB9A\xA5K\xBC\x03\xCE\x9B\xCB\x0E\xC1\xE0\x8E\xAAl\x12\x98.{\xF4H\xD9?\xCCt\x90``\x01[`@Q\x80\x91\x03\x90\xA1PP`\x04T`\x05T`\x06T`\x01\x80U\x91\x9D\x90\x9CP\x90\x9AP\x98PPPPPPPPPV[`\x01T`\x01\x14a\n\xB8W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03E\x90a\x19\xC4V[`\0`\x01\x81\x90UT`\x01`\xA0\x1B\x90\x04`\xFF\x16a\n\xE7W`@Qc!\xC4\xE3W`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80T`@Qc\xB2\xB2\xE7\xDD`\xE0\x1B\x81R\x82\x91\x82\x91\x82\x91\x82\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xB2\xB2\xE7\xDD\x90a\x0B\"\x90\x8B\x90\x8B\x90`\x04\x01a\x19\xE4V[`\xC0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0BxW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x1B\xAA\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0B\x8CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\xB0\x91\x90a\x1A\xEBV[\x95P\x95P\x95P\x95P\x95P\x95P\x85a\x0B\xCEW`\0\x85\x12a\x04:\x86a\x10-V[`\x06\x80T\x90\x82\x90U`\0a\x0B\xE2\x83\x83a\x10\x8AV[`\x07T\x90\x91Pa\x0B\xF2\x90\x82a\x10lV[`\x07Ua\x0B\xFF\x85\x85a\x10\x9FV[PP`\x01\x80UPPPPPPPPPPPPV[`\0\x80`\0`\x01T`\x01\x14a\x0C:W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03E\x90a\x19\xC4V[`\0`\x01\x81\x90U\x80T`@QcH}k\xAB`\xE1\x1B\x81R\x82\x91\x82\x91\x82\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x90\xFA\xD7V\x90a\x0Cz\x90\x8D\x90\x8D\x90`\x04\x01a\x19\xE4V[`\xA0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0C\xD0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x1B\xAA\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0C\xE4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\x08\x91\x90a\x1A#V[\x94P\x94P\x94P\x94P\x94P\x84a\r$W`\0\x84\x12a\x04:\x85a\x10-V[3`\0\x90\x81R`\x08` R`@\x90 T\x15\x80\x15\x90a\rSWP3`\0\x90\x81R`\t` R`@\x90 T`\x07T\x14\x15[\x15a\r\xE4W3`\0\x90\x81R`\t` R`@\x81 T`\x07Ta\rt\x91a\x10lV[3`\0\x90\x81R`\x08` R`@\x90 T\x90\x91Pa\r\x91\x90\x82a\x10lV[`\x08`\x003`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 \x81\x90UPa\r\xE2`@Q\x80`@\x01`@R\x80`\x07\x81R` \x01fin here`\xC8\x1B\x81RPa\x16\xF6V[P[`\0\x83`\x04Ta\r\xF4\x91\x90a\x1A\xC5V[\x90P`\0\x83`\x05Ta\x0E\x06\x91\x90a\x1A\xC5V[\x90P`\0\x83`\x06Ta\x0E\x18\x91\x90a\x1A\xC5V[`\x04\x87\x90U`\x05\x86\x90U`\x06\x85\x90U3`\0\x90\x81R`\x08` R`@\x81 \x80T\x92\x93P\x83\x92\x90\x91\x90a\x0EK\x90\x84\x90a\x1A\xC5V[\x90\x91UPP`\x07T3`\0\x81\x81R`\t` R`@\x90\x81\x90 \x92\x90\x92U`\x02T\x91Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R`\x04\x81\x01\x91\x90\x91R`$\x81\x01\x85\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0E\xF3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x1B\xAA\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x0F\x07W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F+\x91\x90a\x1A\x91V[P`\x03T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0F\xB6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x1B\xAA\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x0F\xCAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xEE\x91\x90a\x1A\x91V[P`@\x80Q\x84\x81R` \x81\x01\x84\x90R\x90\x81\x01\x82\x90R\x7FDO\xFE\xDB%A\xFF,*\xC9\xA6\x16\xD97|\xE4\x0Ev*\x8F\x86\xC2\xF9-AJ\xB8\x8E\x14\xE7Z\x85\x90``\x01a\nkV[`\0`\x01`\xFF\x1B\x82\x03a\x10SW`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x12\x15a\x10dWP\x19`\x01\x01\x90V[P\x80[\x91\x90PV[`\0a\x10\x81\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x17<V[\x90P[\x92\x91PPV[`\0a\x10\x81\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x17<V[`\x04T`\x05T`\0\x91\x82\x91\x82\x91\x82\x91\x81\x88\x11\x15a\x118W`\x02T`\x03T`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x97P\x16\x94Pa\x10\xD6\x82\x89a\x1A\xC5V[\x93P\x86\x81\x11a\x11'W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7Finvalid swap: inputs x and y\0\0\0\0`D\x82\x01R`d\x01a\x03EV[a\x111\x87\x82a\x1A\xC5V[\x92Pa\x11\xB6V[`\x03T`\x02T`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x97P\x16\x94Pa\x11X\x81\x88a\x1A\xC5V[\x93P\x87\x82\x11a\x11\xA9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7Finvalid swap: inputs x and y\0\0\0\0`D\x82\x01R`d\x01a\x03EV[a\x11\xB3\x88\x83a\x1A\xC5V[\x92P[`\x04\x88\x81U`\x05\x88\x90U`@Qcp\xA0\x821`\xE0\x1B\x81R0\x91\x81\x01\x91\x90\x91R`\0\x90`\x01`\x01`\xA0\x1B\x03\x88\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x12BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x1B\xAA\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x12VW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12z\x91\x90a\x1B?V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x88\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x12\xFDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x1B\xAA\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x13\x11W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x135\x91\x90a\x1B?V[`@Qc#\xB8r\xDD`\xE0\x1B\x81R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x89\x16\x90c#\xB8r\xDD\x90a\x13h\x903\x900\x90\x8B\x90`\x04\x01a\x1AmV[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x13\xC0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x1B\xAA\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x13\xD4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\xF8\x91\x90a\x1A\x91V[P`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x86\x90R`\x01`\x01`\xA0\x1B\x03\x88\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x14\x7FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x1B\xAA\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x14\x93W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14\xB7\x91\x90a\x1A\x91V[Pa\x14\xC2\x86\x83a\x1A\xD8V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x8A\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x15?W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x1B\xAA\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x15SW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15w\x91\x90a\x1B?V[\x10\x15a\x15\xD0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: input token transf`D\x82\x01Ra2\xB9`\xF1\x1B`d\x82\x01R`\x84\x01a\x03EV[a\x15\xDA\x85\x82a\x1A\xC5V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x89\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x16WW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x1B\xAA\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x16kW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\x8F\x91\x90a\x1B?V[\x10\x15a\x16\xE9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7Finvalid swap: output token trans`D\x82\x01Rb32\xB9`\xE9\x1B`d\x82\x01R`\x84\x01a\x03EV[PPPP\x92\x95\x91\x94P\x92PV[a\x179\x81`@Q`$\x01a\x17\n\x91\x90a\x1B[V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\x10L\x13\xEB`\xE2\x1B\x17\x90Ra\x17[V[PV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x17TW`\0\x80\xFD[\x04\x92\x91PPV[a\x179\x81\x80Qjconsole.log` \x83\x01`\0\x80\x84\x83\x85Z\xFAPPPPPV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rjrray stride`\xA8\x1B`d\x82\x01R`\x84\x81\xFD[`\0\x80` \x80\x84\x86\x03\x12\x15a\x18@Wa\x18@a\x17\x80V[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x18\xA3W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x84\x90R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x19\x0BW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x84\x90R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x81\xFD[\x815\x81\x81\x11\x15a\x19nW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x85\x90R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x0ENL/$\r\x8C\xAD\xCC\xEE\x8D`\xAB\x1B`d\x82\x01R`\x84\x81\xFD[\x87\x84\x82\x85\x01\x01\x11\x15a\x19\x82Wa\x19\x82a\x17\xD0V[\x91\x90\x92\x01\x96\x90\x95P\x93PPPPV[`\0` \x82\x84\x03\x12\x15a\x19\xA6Wa\x19\xA6a\x17\x80V[\x815`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x19\xBDW`\0\x80\xFD[\x93\x92PPPV[` \x80\x82R`\x06\x90\x82\x01Re\x1B\x1B\xD8\xDA\xD9Y`\xD2\x1B`@\x82\x01R``\x01\x90V[` \x81R\x81` \x82\x01R\x81\x83`@\x83\x017`\0\x81\x83\x01`@\x90\x81\x01\x91\x90\x91R`\x1F\x90\x92\x01`\x1F\x19\x16\x01\x01\x91\x90PV[\x80Q\x80\x15\x15\x81\x14a\x10gW`\0\x80\xFD[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\x1A>Wa\x1A>a\x17\x80V[a\x1AG\x86a\x1A\x13V[` \x87\x01Q`@\x88\x01Q``\x89\x01Q`\x80\x90\x99\x01Q\x92\x9A\x91\x99P\x97\x96P\x90\x94P\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x81R\x91\x90\x92\x16` \x82\x01R`@\x81\x01\x91\x90\x91R``\x01\x90V[`\0` \x82\x84\x03\x12\x15a\x1A\xA6Wa\x1A\xA6a\x17\x80V[a\x10\x81\x82a\x1A\x13V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x10\x84Wa\x10\x84a\x1A\xAFV[\x80\x82\x01\x80\x82\x11\x15a\x10\x84Wa\x10\x84a\x1A\xAFV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a\x1B\x07Wa\x1B\x07a\x17\x80V[a\x1B\x10\x87a\x1A\x13V[\x95P` \x87\x01Q\x94P`@\x87\x01Q\x93P``\x87\x01Q\x92P`\x80\x87\x01Q\x91P`\xA0\x87\x01Q\x90P\x92\x95P\x92\x95P\x92\x95V[`\0` \x82\x84\x03\x12\x15a\x1BTWa\x1BTa\x17\x80V[PQ\x91\x90PV[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a\x1B\x88W\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a\x1BlV[P`\0`@\x82\x86\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x92PPP\x92\x91PPV\xFETarget contract does not contain";
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
        ///Calls the contract's `allocate` (0x5aef467a) function
        pub fn allocate(
            &self,
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
                .method_hash([90, 239, 70, 122], data)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `balanceOf` (0x70a08231) function
        pub fn balance_of(
            &self,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], account)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deallocate` (0xfa0a345e) function
        pub fn deallocate(
            &self,
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
                .method_hash([250, 10, 52, 94], data)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `feeGrowth` (0xeae5ea08) function
        pub fn fee_growth(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([234, 229, 234, 8], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `feeGrowthLast` (0xbad5264f) function
        pub fn fee_growth_last(
            &self,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([186, 213, 38, 79], account)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getReservesAndLiquidity` (0xebadef01) function
        pub fn get_reserves_and_liquidity(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([235, 173, 239, 1], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `init` (0x4ddf47d4) function
        pub fn init(
            &self,
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
                .method_hash([77, 223, 71, 212], data)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `inited` (0x43c885ba) function
        pub fn inited(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([67, 200, 133, 186], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `locked` (0xcf309012) function
        pub fn locked(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([207, 48, 144, 18], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `reserveXWad` (0xf065957f) function
        pub fn reserve_x_wad(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([240, 101, 149, 127], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `reserveYWad` (0x5e227d58) function
        pub fn reserve_y_wad(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([94, 34, 125, 88], ())
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
        ///Calls the contract's `swap` (0x627dd56a) function
        pub fn swap(
            &self,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([98, 125, 213, 106], data)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `tokenX` (0x16dc165b) function
        pub fn token_x(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([22, 220, 22, 91], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `tokenY` (0xb7d19fc4) function
        pub fn token_y(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([183, 209, 159, 196], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `totalLiquidity` (0x15770f92) function
        pub fn total_liquidity(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([21, 119, 15, 146], ())
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
        Invalid(Invalid),
        InvalidSwap(InvalidSwap),
        Min(Min),
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
            if let Ok(decoded) = <Min as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Min(decoded));
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
                Self::Invalid(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InvalidSwap(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Min(element) => ::ethers::core::abi::AbiEncode::encode(element),
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
                    == <Invalid as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <InvalidSwap as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <Min as ::ethers::contract::EthError>::selector() => {
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
                Self::Invalid(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidSwap(element) => ::core::fmt::Display::fmt(element, f),
                Self::Min(element) => ::core::fmt::Display::fmt(element, f),
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
    impl ::core::convert::From<Min> for DFMMErrors {
        fn from(value: Min) -> Self {
            Self::Min(value)
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
    #[ethevent(name = "Allocate", abi = "Allocate(uint256,uint256,uint256)")]
    pub struct AllocateFilter {
        pub x: ::ethers::core::types::U256,
        pub y: ::ethers::core::types::U256,
        pub l: ::ethers::core::types::U256,
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
    #[ethevent(name = "Deallocate", abi = "Deallocate(uint256,uint256,uint256)")]
    pub struct DeallocateFilter {
        pub x: ::ethers::core::types::U256,
        pub y: ::ethers::core::types::U256,
        pub l: ::ethers::core::types::U256,
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
    #[ethevent(name = "Init", abi = "Init(address,address,uint256,uint256,uint256)")]
    pub struct InitFilter {
        #[ethevent(indexed)]
        pub swapper: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub strategy: ::ethers::core::types::Address,
        pub xxxxxxx: ::ethers::core::types::U256,
        pub yyyyyy: ::ethers::core::types::U256,
        pub llllll: ::ethers::core::types::U256,
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
    #[ethevent(
        name = "Swap",
        abi = "Swap(address,address,address,address,uint256,uint256,int256)"
    )]
    pub struct SwapFilter {
        #[ethevent(indexed)]
        pub swapper: ::ethers::core::types::Address,
        pub source: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub token_in: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub token_out: ::ethers::core::types::Address,
        pub amount_in: ::ethers::core::types::U256,
        pub amount_out: ::ethers::core::types::U256,
        pub liquidity_delta: ::ethers::core::types::I256,
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
    ///Container type for all input parameters for the `allocate` function with signature `allocate(bytes)` and selector `0x5aef467a`
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
    #[ethcall(name = "allocate", abi = "allocate(bytes)")]
    pub struct AllocateCall {
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `balanceOf` function with signature `balanceOf(address)` and selector `0x70a08231`
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
    #[ethcall(name = "balanceOf", abi = "balanceOf(address)")]
    pub struct BalanceOfCall {
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `deallocate` function with signature `deallocate(bytes)` and selector `0xfa0a345e`
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
    #[ethcall(name = "deallocate", abi = "deallocate(bytes)")]
    pub struct DeallocateCall {
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `feeGrowth` function with signature `feeGrowth()` and selector `0xeae5ea08`
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
    #[ethcall(name = "feeGrowth", abi = "feeGrowth()")]
    pub struct FeeGrowthCall;
    ///Container type for all input parameters for the `feeGrowthLast` function with signature `feeGrowthLast(address)` and selector `0xbad5264f`
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
    #[ethcall(name = "feeGrowthLast", abi = "feeGrowthLast(address)")]
    pub struct FeeGrowthLastCall {
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getReservesAndLiquidity` function with signature `getReservesAndLiquidity()` and selector `0xebadef01`
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
    #[ethcall(name = "getReservesAndLiquidity", abi = "getReservesAndLiquidity()")]
    pub struct GetReservesAndLiquidityCall;
    ///Container type for all input parameters for the `init` function with signature `init(bytes)` and selector `0x4ddf47d4`
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
    #[ethcall(name = "init", abi = "init(bytes)")]
    pub struct InitCall {
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `inited` function with signature `inited()` and selector `0x43c885ba`
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
    #[ethcall(name = "inited", abi = "inited()")]
    pub struct InitedCall;
    ///Container type for all input parameters for the `locked` function with signature `locked()` and selector `0xcf309012`
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
    #[ethcall(name = "locked", abi = "locked()")]
    pub struct LockedCall;
    ///Container type for all input parameters for the `reserveXWad` function with signature `reserveXWad()` and selector `0xf065957f`
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
    #[ethcall(name = "reserveXWad", abi = "reserveXWad()")]
    pub struct ReserveXWadCall;
    ///Container type for all input parameters for the `reserveYWad` function with signature `reserveYWad()` and selector `0x5e227d58`
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
    #[ethcall(name = "reserveYWad", abi = "reserveYWad()")]
    pub struct ReserveYWadCall;
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
    ///Container type for all input parameters for the `swap` function with signature `swap(bytes)` and selector `0x627dd56a`
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
    #[ethcall(name = "swap", abi = "swap(bytes)")]
    pub struct SwapCall {
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `tokenX` function with signature `tokenX()` and selector `0x16dc165b`
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
    #[ethcall(name = "tokenX", abi = "tokenX()")]
    pub struct TokenXCall;
    ///Container type for all input parameters for the `tokenY` function with signature `tokenY()` and selector `0xb7d19fc4`
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
    #[ethcall(name = "tokenY", abi = "tokenY()")]
    pub struct TokenYCall;
    ///Container type for all input parameters for the `totalLiquidity` function with signature `totalLiquidity()` and selector `0x15770f92`
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
    #[ethcall(name = "totalLiquidity", abi = "totalLiquidity()")]
    pub struct TotalLiquidityCall;
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
        BalanceOf(BalanceOfCall),
        Deallocate(DeallocateCall),
        FeeGrowth(FeeGrowthCall),
        FeeGrowthLast(FeeGrowthLastCall),
        GetReservesAndLiquidity(GetReservesAndLiquidityCall),
        Init(InitCall),
        Inited(InitedCall),
        Locked(LockedCall),
        ReserveXWad(ReserveXWadCall),
        ReserveYWad(ReserveYWadCall),
        Strategy(StrategyCall),
        Swap(SwapCall),
        TokenX(TokenXCall),
        TokenY(TokenYCall),
        TotalLiquidity(TotalLiquidityCall),
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
            if let Ok(decoded) = <BalanceOfCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BalanceOf(decoded));
            }
            if let Ok(decoded) = <DeallocateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Deallocate(decoded));
            }
            if let Ok(decoded) = <FeeGrowthCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FeeGrowth(decoded));
            }
            if let Ok(decoded) = <FeeGrowthLastCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FeeGrowthLast(decoded));
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
            if let Ok(decoded) = <InitedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Inited(decoded));
            }
            if let Ok(decoded) = <LockedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Locked(decoded));
            }
            if let Ok(decoded) = <ReserveXWadCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ReserveXWad(decoded));
            }
            if let Ok(decoded) = <ReserveYWadCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ReserveYWad(decoded));
            }
            if let Ok(decoded) = <StrategyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Strategy(decoded));
            }
            if let Ok(decoded) = <SwapCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Swap(decoded));
            }
            if let Ok(decoded) = <TokenXCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TokenX(decoded));
            }
            if let Ok(decoded) = <TokenYCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TokenY(decoded));
            }
            if let Ok(decoded) = <TotalLiquidityCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TotalLiquidity(decoded));
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
                Self::BalanceOf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Deallocate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FeeGrowth(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FeeGrowthLast(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetReservesAndLiquidity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Init(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Inited(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Locked(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ReserveXWad(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ReserveYWad(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Strategy(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Swap(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TokenX(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TokenY(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TotalLiquidity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for DFMMCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Allocate(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::Deallocate(element) => ::core::fmt::Display::fmt(element, f),
                Self::FeeGrowth(element) => ::core::fmt::Display::fmt(element, f),
                Self::FeeGrowthLast(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetReservesAndLiquidity(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Init(element) => ::core::fmt::Display::fmt(element, f),
                Self::Inited(element) => ::core::fmt::Display::fmt(element, f),
                Self::Locked(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReserveXWad(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReserveYWad(element) => ::core::fmt::Display::fmt(element, f),
                Self::Strategy(element) => ::core::fmt::Display::fmt(element, f),
                Self::Swap(element) => ::core::fmt::Display::fmt(element, f),
                Self::TokenX(element) => ::core::fmt::Display::fmt(element, f),
                Self::TokenY(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalLiquidity(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AllocateCall> for DFMMCalls {
        fn from(value: AllocateCall) -> Self {
            Self::Allocate(value)
        }
    }
    impl ::core::convert::From<BalanceOfCall> for DFMMCalls {
        fn from(value: BalanceOfCall) -> Self {
            Self::BalanceOf(value)
        }
    }
    impl ::core::convert::From<DeallocateCall> for DFMMCalls {
        fn from(value: DeallocateCall) -> Self {
            Self::Deallocate(value)
        }
    }
    impl ::core::convert::From<FeeGrowthCall> for DFMMCalls {
        fn from(value: FeeGrowthCall) -> Self {
            Self::FeeGrowth(value)
        }
    }
    impl ::core::convert::From<FeeGrowthLastCall> for DFMMCalls {
        fn from(value: FeeGrowthLastCall) -> Self {
            Self::FeeGrowthLast(value)
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
    impl ::core::convert::From<InitedCall> for DFMMCalls {
        fn from(value: InitedCall) -> Self {
            Self::Inited(value)
        }
    }
    impl ::core::convert::From<LockedCall> for DFMMCalls {
        fn from(value: LockedCall) -> Self {
            Self::Locked(value)
        }
    }
    impl ::core::convert::From<ReserveXWadCall> for DFMMCalls {
        fn from(value: ReserveXWadCall) -> Self {
            Self::ReserveXWad(value)
        }
    }
    impl ::core::convert::From<ReserveYWadCall> for DFMMCalls {
        fn from(value: ReserveYWadCall) -> Self {
            Self::ReserveYWad(value)
        }
    }
    impl ::core::convert::From<StrategyCall> for DFMMCalls {
        fn from(value: StrategyCall) -> Self {
            Self::Strategy(value)
        }
    }
    impl ::core::convert::From<SwapCall> for DFMMCalls {
        fn from(value: SwapCall) -> Self {
            Self::Swap(value)
        }
    }
    impl ::core::convert::From<TokenXCall> for DFMMCalls {
        fn from(value: TokenXCall) -> Self {
            Self::TokenX(value)
        }
    }
    impl ::core::convert::From<TokenYCall> for DFMMCalls {
        fn from(value: TokenYCall) -> Self {
            Self::TokenY(value)
        }
    }
    impl ::core::convert::From<TotalLiquidityCall> for DFMMCalls {
        fn from(value: TotalLiquidityCall) -> Self {
            Self::TotalLiquidity(value)
        }
    }
    ///Container type for all return fields from the `allocate` function with signature `allocate(bytes)` and selector `0x5aef467a`
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
    ///Container type for all return fields from the `balanceOf` function with signature `balanceOf(address)` and selector `0x70a08231`
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
    pub struct BalanceOfReturn {
        pub balance: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `deallocate` function with signature `deallocate(bytes)` and selector `0xfa0a345e`
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
    ///Container type for all return fields from the `feeGrowth` function with signature `feeGrowth()` and selector `0xeae5ea08`
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
    pub struct FeeGrowthReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `feeGrowthLast` function with signature `feeGrowthLast(address)` and selector `0xbad5264f`
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
    pub struct FeeGrowthLastReturn {
        pub balance: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `getReservesAndLiquidity` function with signature `getReservesAndLiquidity()` and selector `0xebadef01`
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
    ///Container type for all return fields from the `init` function with signature `init(bytes)` and selector `0x4ddf47d4`
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
    );
    ///Container type for all return fields from the `inited` function with signature `inited()` and selector `0x43c885ba`
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
    pub struct InitedReturn(pub bool);
    ///Container type for all return fields from the `locked` function with signature `locked()` and selector `0xcf309012`
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
    pub struct LockedReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `reserveXWad` function with signature `reserveXWad()` and selector `0xf065957f`
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
    pub struct ReserveXWadReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `reserveYWad` function with signature `reserveYWad()` and selector `0x5e227d58`
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
    pub struct ReserveYWadReturn(pub ::ethers::core::types::U256);
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
    ///Container type for all return fields from the `tokenX` function with signature `tokenX()` and selector `0x16dc165b`
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
    pub struct TokenXReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `tokenY` function with signature `tokenY()` and selector `0xb7d19fc4`
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
    pub struct TokenYReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `totalLiquidity` function with signature `totalLiquidity()` and selector `0x15770f92`
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
    pub struct TotalLiquidityReturn(pub ::ethers::core::types::U256);
}
