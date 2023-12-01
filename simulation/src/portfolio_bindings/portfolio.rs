pub use portfolio::*;
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
pub mod portfolio {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("weth"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("registry"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("positionRenderer"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("POSITION_RENDERER"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("POSITION_RENDERER"),
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
                    ::std::borrow::ToOwned::to_owned("REGISTRY"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("REGISTRY"),
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
                    ::std::borrow::ToOwned::to_owned("VERSION"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("VERSION"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("WETH"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("WETH"),
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
                    ::std::borrow::ToOwned::to_owned("allocate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("allocate"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("useMax"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("recipient"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("deltaLiquidity"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint128"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("maxDeltaAsset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint128"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("maxDeltaQuote"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint128"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("deltaAsset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("deltaQuote"),
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
                    ::std::borrow::ToOwned::to_owned("balanceOf"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("balanceOf"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("balanceOfBatch"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("balanceOfBatch"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("owners"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("ids"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("balances"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("changeParameters"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("changeParameters"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("priorityFee"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint16"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("fee"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint16"),
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
                    ::std::borrow::ToOwned::to_owned("claimFee"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("claimFee"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
                    ::std::borrow::ToOwned::to_owned("createPair"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("createPair"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("quote"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("pairId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(24usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint24"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("createPool"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("createPool"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("pairId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(24usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint24"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("reserveXPerWad"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("reserveYPerWad"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("feeBasisPoints"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint16"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "priorityFeeBasisPoints",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint16"),
                                    ),
                                },
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
                                    name: ::std::borrow::ToOwned::to_owned("strategyArgs"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
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
                                    name: ::std::borrow::ToOwned::to_owned("useMax"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("deltaLiquidity"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint128"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("minDeltaAsset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint128"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("minDeltaQuote"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint128"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("deltaAsset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("deltaQuote"),
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
                    ::std::borrow::ToOwned::to_owned("getAmountOut"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getAmountOut"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sellAsset"),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("swapper"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("output"),
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
                    ::std::borrow::ToOwned::to_owned("getInvariant"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getInvariant"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
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
                    ::std::borrow::ToOwned::to_owned("getLiquidityDeltas"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getLiquidityDeltas"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("deltaLiquidity"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int128"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("deltaAsset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint128"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("deltaQuote"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint128"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getMaxLiquidity"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getMaxLiquidity"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount0"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount1"),
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
                                    name: ::std::borrow::ToOwned::to_owned("deltaLiquidity"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint128"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getMaxOrder"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getMaxOrder"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sellAsset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("swapper"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct Order"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getNetBalance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getNetBalance"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("getPairId"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getPairId"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(24usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint24"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getPairNonce"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getPairNonce"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(24usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint24"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getPoolNonce"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getPoolNonce"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(24usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint24"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getPoolReserves"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getPoolReserves"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("deltaAsset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("deltaQuote"),
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
                    ::std::borrow::ToOwned::to_owned("getReserve"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getReserve"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("getSpotPrice"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getSpotPrice"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
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
                    ::std::borrow::ToOwned::to_owned("getStrategy"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getStrategy"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                            ],
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
                    ::std::borrow::ToOwned::to_owned("isApprovedForAll"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isApprovedForAll"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                                    name: ::std::borrow::ToOwned::to_owned("results"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("pairs"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("pairs"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(24usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint24"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenAsset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("decimalsAsset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenQuote"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("decimalsQuote"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("virtualX"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint128"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("virtualY"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint128"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("liquidity"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint128"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("lastTimestamp"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("feeBasisPoints"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint16"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "priorityFeeBasisPoints",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint16"),
                                    ),
                                },
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
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("protocolFee"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("protocolFee"),
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
                    ::std::borrow::ToOwned::to_owned("protocolFees"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("protocolFees"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("safeBatchTransferFrom"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "safeBatchTransferFrom",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("ids"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amounts"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
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
                    ::std::borrow::ToOwned::to_owned("safeTransferFrom"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("safeTransferFrom"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("id"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
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
                    ::std::borrow::ToOwned::to_owned("setApprovalForAll"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setApprovalForAll"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("approved"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
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
                    ::std::borrow::ToOwned::to_owned("setProtocolFee"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setProtocolFee"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("fee"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
                    ::std::borrow::ToOwned::to_owned("simulateSwap"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("simulateSwap"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("order"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct Order"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("timestamp"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("swapper"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("success"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("prevInvariant"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("postInvariant"),
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
                    ::std::borrow::ToOwned::to_owned("supportsInterface"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("supportsInterface"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("interfaceId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        4usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4"),
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
                                    name: ::std::borrow::ToOwned::to_owned("args"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct Order"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("input"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("output"),
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
                    ::std::borrow::ToOwned::to_owned("uri"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("uri"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("id"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
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
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("quote"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("deltaAsset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("deltaQuote"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("deltaLiquidity"),
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
                    ::std::borrow::ToOwned::to_owned("ApprovalForAll"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ApprovalForAll"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("operator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("approved"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ChangeParameters"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ChangeParameters"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("priorityFee"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("fee"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ClaimFees"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ClaimFees"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
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
                    ::std::borrow::ToOwned::to_owned("CreatePair"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("CreatePair"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("pairId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(24usize),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("quote"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("decimalsAsset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("decimalsQuote"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("CreatePool"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("CreatePool"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("quote"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("reserveXPerWad"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("reserveYPerWad"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("feeBasisPoints"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "priorityFeeBasisPoints",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("controller"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("strategy"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
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
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("quote"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("deltaAsset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("deltaQuote"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("deltaLiquidity"),
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
                    ::std::borrow::ToOwned::to_owned("DecreaseReserveBalance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "DecreaseReserveBalance",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
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
                    ::std::borrow::ToOwned::to_owned("Deposit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Deposit"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
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
                    ::std::borrow::ToOwned::to_owned("IncreaseReserveBalance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "IncreaseReserveBalance",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
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
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sellAsset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("tokenIn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("input"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("tokenOut"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("output"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("feeAmountDec"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("invariantWad"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TransferBatch"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("TransferBatch"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("operator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("ids"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amounts"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TransferSingle"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("TransferSingle"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("operator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("id"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
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
                    ::std::borrow::ToOwned::to_owned("URI"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("URI"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("id"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("UpdateProtocolFee"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("UpdateProtocolFee"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("prevFee"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("nextFee"),
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
                    ::std::borrow::ToOwned::to_owned("EtherTransfer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("EtherTransfer"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InsufficientReserve"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InsufficientReserve",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("delta"),
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
                    ::std::borrow::ToOwned::to_owned("InvalidBalance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidBalance"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PoolLib_AlreadyCreated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "PoolLib_AlreadyCreated",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PoolLib_InvalidFee"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("PoolLib_InvalidFee"),
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
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PoolLib_InvalidPriorityFee"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "PoolLib_InvalidPriorityFee",
                            ),
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
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PoolLib_InvalidReserveX"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "PoolLib_InvalidReserveX",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PoolLib_InvalidReserveY"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "PoolLib_InvalidReserveY",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PoolLib_UpperLiquidityLimit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "PoolLib_UpperLiquidityLimit",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Portfolio_AfterCreateFail"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Portfolio_AfterCreateFail",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Portfolio_BeforeSwapFail"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Portfolio_BeforeSwapFail",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Portfolio_DuplicateToken"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Portfolio_DuplicateToken",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Portfolio_Insolvent"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Portfolio_Insolvent",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("net"),
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
                    ::std::borrow::ToOwned::to_owned("Portfolio_InsufficientLiquidity"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Portfolio_InsufficientLiquidity",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Portfolio_InvalidDecimals"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Portfolio_InvalidDecimals",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("decimals"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Portfolio_InvalidInvariant"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Portfolio_InvalidInvariant",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("prev"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("next"),
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
                    ::std::borrow::ToOwned::to_owned("Portfolio_InvalidMulticall"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Portfolio_InvalidMulticall",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Portfolio_InvalidPairNonce"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Portfolio_InvalidPairNonce",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Portfolio_InvalidPool"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Portfolio_InvalidPool",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Portfolio_InvalidProtocolFee"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Portfolio_InvalidProtocolFee",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("protocolFee"),
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
                    ::std::borrow::ToOwned::to_owned("Portfolio_InvalidReentrancy"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Portfolio_InvalidReentrancy",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Portfolio_InvalidSettlement"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Portfolio_InvalidSettlement",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Portfolio_MaxAssetExceeded"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Portfolio_MaxAssetExceeded",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Portfolio_MaxQuoteExceeded"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Portfolio_MaxQuoteExceeded",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Portfolio_MinAssetExceeded"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Portfolio_MinAssetExceeded",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Portfolio_MinQuoteExceeded"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Portfolio_MinQuoteExceeded",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Portfolio_NonExistentPool"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Portfolio_NonExistentPool",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Portfolio_NotController"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Portfolio_NotController",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Portfolio_PairExists"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Portfolio_PairExists",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("pairId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(24usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint24"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Portfolio_ZeroAmountsAllocate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Portfolio_ZeroAmountsAllocate",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Portfolio_ZeroLiquidityAllocate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Portfolio_ZeroLiquidityAllocate",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "Portfolio_ZeroLiquidityDeallocate",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Portfolio_ZeroLiquidityDeallocate",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Portfolio_ZeroSwapInput"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Portfolio_ZeroSwapInput",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Portfolio_ZeroSwapLiquidity"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Portfolio_ZeroSwapLiquidity",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Portfolio_ZeroSwapOutput"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Portfolio_ZeroSwapOutput",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SwapLib_OutputExceedsReserves"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "SwapLib_OutputExceedsReserves",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SwapLib_ProtocolFeeTooHigh"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "SwapLib_ProtocolFeeTooHigh",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SwapLib_ZeroXAdjustment"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "SwapLib_ZeroXAdjustment",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SwapLib_ZeroYAdjustment"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "SwapLib_ZeroYAdjustment",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TokenTransfer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("TokenTransfer"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TokenTransferFrom"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("TokenTransferFrom"),
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
    pub static PORTFOLIO_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xE0`@R`\x01`\x0CU4\x80\x15b\0\0cW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`@Qb\0x\x9C8\x03\x80b\0x\x9C\x839\x81\x01`@\x81\x90Rb\0\0\x86\x91b\0\0\xCEV[`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x80R\x90\x82\x16`\xA0R\x16`\xC0R`\x05\x80T`\xFF\x19\x16`\x01\x17\x90Ub\0\x01cV[\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\0\xC9W`\0\x80\xFD[\x91\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15b\0\x01/W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[b\0\x01:\x84b\0\0\xB1V[\x92Pb\0\x01J` \x85\x01b\0\0\xB1V[\x91Pb\0\x01Z`@\x85\x01b\0\0\xB1V[\x90P\x92P\x92P\x92V[`\x80Q`\xA0Q`\xC0Qav\xD8b\0\x01\xC4`\09`\0\x81\x81a\r\xC8\x01Ra\x12|\x01R`\0\x81\x81a\x04\x03\x01R\x81\x81a'\xDA\x01Ra4M\x01R`\0\x81\x81a\x02C\x01R\x81\x81a\rY\x01R\x81\x81aG|\x01R\x81\x81aPD\x01RaP\x80\x01Rav\xD8`\0\xF3\xFE`\x80`@R`\x046\x10a\x023W`\x005`\xE0\x1C\x80c\x89\x92\xF2\n\x11a\x01.W\x80c\xC9\xC6S\x96\x11a\0\xABW\x80c\xE9\x85\xE9\xC5\x11a\0oW\x80c\xE9\x85\xE9\xC5\x14a\x10PW\x80c\xF0{\x87\x9E\x14a\x10\xC6W\x80c\xF2BC*\x14a\x11.W\x80c\xF3:\xE1\xBC\x14a\x11\x89W\x80c\xFF\xA1\xADt\x14a\x11\xC1Wa\x02oV[\x80c\xC9\xC6S\x96\x14a\x0E\xACW\x80c\xD6\xB7\xDE\xC5\x14a\x0E\xBFW\x80c\xDC\xF8D\xA7\x14a\x0F2W\x80c\xDD\xA4\x07\x97\x14a\x0F\x9AW\x80c\xE31\xBA4\x14a\x0F\xF5Wa\x02oV[\x80c\xAC\x96P\xD8\x11a\0\xF2W\x80c\xAC\x96P\xD8\x14a\x0C\xECW\x80c\xAD\\FH\x14a\r\x0CW\x80c\xB0\xC3\xA9P\x14a\r{W\x80c\xB0\xE2\x1E\x8A\x14a\r\xEAW\x80c\xC9\xA3\x96\xE9\x14a\x0E;Wa\x02oV[\x80c\x89\x92\xF2\n\x14a\n\x11W\x80c\x89\xA5\xF0\x84\x14a\n\x8CW\x80c\x8Ag\x89g\x14a\x0B\xB3W\x80c\xA2,\xB4e\x14a\x0C\x0EW\x80c\xA5\xCD\x8AI\x14a\x0CiWa\x02oV[\x80c/\x9E8\xE2\x11a\x01\xBCW\x80cN\x12s\xF4\x11a\x01\x80W\x80cN\x12s\xF4\x14a\x07\xFDW\x80c[\xC5Td\x14a\x08eW\x80c^Gf<\x14a\x08xW\x80cx}\xCE=\x14a\t>W\x80c\x80\xAF\x9Dv\x14a\t\x99Wa\x02oV[\x80c/\x9E8\xE2\x14a\x06aW\x80c0$K\xE7\x14a\x06tW\x80c9CMZ\x14a\x06\xCFW\x80c?\x92\xA39\x14a\x07*W\x80cM\xC6\x8A\x90\x14a\x07\xA2Wa\x02oV[\x80c\x0E\x894\x1C\x11a\x02\x03W\x80c\x0E\x894\x1C\x14a\x04\xA8W\x80c\x19\x05x\x07\x14a\x05\x10W\x80c&z\x0C\xFE\x14a\x05kW\x80c*\xFB\x9D\xF8\x14a\x05\x96W\x80c.\xB2\xC2\xD6\x14a\x06\x06Wa\x02oV[\x80b\xFD\xD5\x8E\x14a\x02\xC8W\x80c\x01\xFF\xC9\xA7\x14a\x03KW\x80c\x06C;\x1B\x14a\x03\xB6W\x80c\x07\x88\x88\xD6\x14a\x04=Wa\x02oV[6a\x02oW3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x02mW`\0\x80\xFD[\0[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`)`$\x82\x01R\x7FUnknown signature and no fallbac`D\x82\x01\x90\x81Rh\x1A\xC8\x19\x19Y\x9A[\x99Y`\xBA\x1B`d\x83\x01R`\x84\x82\xFD[4\x80\x15a\x03\x0FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R`\0\x80Q` av\x98\x839\x81Q\x91R`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[Pa\x038a\x03\x1E6`\x04ab\x0FV[`\0` \x81\x81R\x92\x81R`@\x80\x82 \x90\x93R\x90\x81R T\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03\x92W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R`\0\x80Q` av\x98\x839\x81Q\x91R`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[Pa\x03\xA6a\x03\xA16`\x04abTV[a\x12\x11V[`@Q\x90\x15\x15\x81R` \x01a\x03BV[4\x80\x15a\x03\xFDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R`\0\x80Q` av\x98\x839\x81Q\x91R`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[Pa\x04%\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x03BV[4\x80\x15a\x04\x84W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R`\0\x80Q` av\x98\x839\x81Q\x91R`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x06Ta\x04\x94\x90b\xFF\xFF\xFF\x16\x81V[`@Qb\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x03BV[4\x80\x15a\x04\xEFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R`\0\x80Q` av\x98\x839\x81Q\x91R`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[Pa\x05\x03a\x04\xFE6`\x04abtV[a\x12cV[`@Qa\x03B\x91\x90ab\xE0V[4\x80\x15a\x05WW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R`\0\x80Q` av\x98\x839\x81Q\x91R`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[Pa\x038a\x05f6`\x04ac\x16V[a\x13@V[a\x05~a\x05y6`\x04ad\xEBV[a\x14+V[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01a\x03BV[4\x80\x15a\x05\xDDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R`\0\x80Q` av\x98\x839\x81Q\x91R`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[Pa\x05\xF1a\x05\xEC6`\x04ae\xA1V[a\x17.V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x03BV[4\x80\x15a\x06MW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R`\0\x80Q` av\x98\x839\x81Q\x91R`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[Pa\x02ma\x06\\6`\x04af\x0EV[a\x18\x85V[a\x05\xF1a\x06o6`\x04af\xEDV[a\x1B\x93V[4\x80\x15a\x06\xBBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R`\0\x80Q` av\x98\x839\x81Q\x91R`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[Pa\x04%a\x06\xCA6`\x04ae\xA1V[a!]V[4\x80\x15a\x07\x16W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R`\0\x80Q` av\x98\x839\x81Q\x91R`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[Pa\x038a\x07%6`\x04ae\xA1V[a!\x84V[4\x80\x15a\x07qW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R`\0\x80Q` av\x98\x839\x81Q\x91R`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[Pa\x04\x94a\x07\x806`\x04agrV[`\x0B` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 Tb\xFF\xFF\xFF\x16\x81V[4\x80\x15a\x07\xE9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R`\0\x80Q` av\x98\x839\x81Q\x91R`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[Pa\x038a\x07\xF86`\x04ag\xAEV[a\"PV[4\x80\x15a\x08DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R`\0\x80Q` av\x98\x839\x81Q\x91R`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[Pa\x08Xa\x08S6`\x04ag\xCEV[a\"^V[`@Qa\x03B\x91\x90ahBV[a\x05\xF1a\x08s6`\x04ah\x86V[a#\x92V[4\x80\x15a\x08\xBFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R`\0\x80Q` av\x98\x839\x81Q\x91R`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[Pa\t\na\x08\xCE6`\x04ah\xFAV[`\t` R`\0\x90\x81R`@\x90 \x80T`\x01\x90\x91\x01T`\x01`\x01`\xA0\x1B\x03\x80\x83\x16\x92`\xFF`\x01`\xA0\x1B\x91\x82\x90\x04\x81\x16\x93\x92\x83\x16\x92\x91\x90\x91\x04\x16\x84V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x95\x86\x16\x81R`\xFF\x94\x85\x16` \x82\x01R\x94\x90\x92\x16\x91\x84\x01\x91\x90\x91R\x16``\x82\x01R`\x80\x01a\x03BV[4\x80\x15a\t\x85W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R`\0\x80Q` av\x98\x839\x81Q\x91R`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[Pa\x02ma\t\x946`\x04abtV[a'\xD0V[4\x80\x15a\t\xE0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R`\0\x80Q` av\x98\x839\x81Q\x91R`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[Pa\t\xF4a\t\xEF6`\x04ajMV[a)XV[`@\x80Q\x93\x15\x15\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x03BV[4\x80\x15a\nXW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R`\0\x80Q` av\x98\x839\x81Q\x91R`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[Pa\nla\ng6`\x04aj\x91V[a*6V[`@\x80Q`\x01`\x01`\x80\x1B\x03\x93\x84\x16\x81R\x92\x90\x91\x16` \x83\x01R\x01a\x03BV[4\x80\x15a\n\xD3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R`\0\x80Q` av\x98\x839\x81Q\x91R`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[Pa\x0BOa\n\xE26`\x04ae\xA1V[`\n` R`\0\x90\x81R`@\x90 \x80T`\x01\x82\x01T`\x02\x83\x01T`\x03\x90\x93\x01T`\x01`\x01`\x80\x1B\x03\x80\x84\x16\x94`\x01`\x80\x1B\x94\x85\x90\x04\x82\x16\x94\x91\x84\x16\x93\x91\x82\x04c\xFF\xFF\xFF\xFF\x16\x92`\x01`\xA0\x1B\x83\x04a\xFF\xFF\x90\x81\x16\x93`\x01`\xB0\x1B\x90\x04\x16\x91`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x91\x16\x88V[`@\x80Q`\x01`\x01`\x80\x1B\x03\x99\x8A\x16\x81R\x97\x89\x16` \x89\x01R\x97\x90\x95\x16\x96\x86\x01\x96\x90\x96Rc\xFF\xFF\xFF\xFF\x90\x92\x16``\x85\x01Ra\xFF\xFF\x90\x81\x16`\x80\x85\x01R\x16`\xA0\x83\x01R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\xC0\x83\x01R\x91\x90\x91\x16`\xE0\x82\x01Ra\x01\0\x01a\x03BV[4\x80\x15a\x0B\xFAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R`\0\x80Q` av\x98\x839\x81Q\x91R`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[Pa\x02ma\x0C\t6`\x04aj\xC9V[a+\xFAV[4\x80\x15a\x0CUW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R`\0\x80Q` av\x98\x839\x81Q\x91R`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[Pa\x02ma\x0Cd6`\x04ak\x11V[a-xV[4\x80\x15a\x0C\xB0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R`\0\x80Q` av\x98\x839\x81Q\x91R`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[Pa\x0C\xD7a\x0C\xBF6`\x04ah\xFAV[`\x08` R`\0\x90\x81R`@\x90 Tc\xFF\xFF\xFF\xFF\x16\x81V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x03BV[a\x0C\xFFa\x0C\xFA6`\x04akBV[a-\xE4V[`@Qa\x03B\x91\x90ak\x89V[4\x80\x15a\rSW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R`\0\x80Q` av\x98\x839\x81Q\x91R`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[Pa\x04%\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\r\xC2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R`\0\x80Q` av\x98\x839\x81Q\x91R`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[Pa\x04%\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x0E1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R`\0\x80Q` av\x98\x839\x81Q\x91R`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[Pa\x038`\rT\x81V[4\x80\x15a\x0E\x82W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R`\0\x80Q` av\x98\x839\x81Q\x91R`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[Pa\x038a\x0E\x916`\x04ag\xAEV[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x02` R`@\x90 T\x90V[a\x04\x94a\x0E\xBA6`\x04agrV[a/WV[4\x80\x15a\x0F\x06W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R`\0\x80Q` av\x98\x839\x81Q\x91R`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[Pa\x0F\x1Aa\x0F\x156`\x04ak\xEBV[a3\x02V[`@Q`\x01`\x01`\x80\x1B\x03\x90\x91\x16\x81R` \x01a\x03BV[4\x80\x15a\x0FyW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R`\0\x80Q` av\x98\x839\x81Q\x91R`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[Pa\x038a\x0F\x886`\x04ag\xAEV[`\x07` R`\0\x90\x81R`@\x90 T\x81V[4\x80\x15a\x0F\xE1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R`\0\x80Q` av\x98\x839\x81Q\x91R`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[Pa\x02ma\x0F\xF06`\x04ab\x0FV[a4CV[4\x80\x15a\x10<W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R`\0\x80Q` av\x98\x839\x81Q\x91R`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[Pa\x038a\x10K6`\x04ae\xA1V[a6\xE4V[4\x80\x15a\x10\x97W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R`\0\x80Q` av\x98\x839\x81Q\x91R`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[Pa\x03\xA6a\x10\xA66`\x04agrV[`\x01` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T`\xFF\x16\x81V[4\x80\x15a\x11\rW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R`\0\x80Q` av\x98\x839\x81Q\x91R`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[Pa\x11!a\x11\x1C6`\x04al#V[a7&V[`@Qa\x03B\x91\x90al\xB0V[4\x80\x15a\x11uW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R`\0\x80Q` av\x98\x839\x81Q\x91R`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[Pa\x02ma\x11\x846`\x04al\xBEV[a82V[a\x11\x9Ca\x11\x976`\x04am?V[a:\x92V[`@\x80Q`\x01`\x01`@\x1B\x03\x90\x94\x16\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x03BV[4\x80\x15a\x12\x08W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R`\0\x80Q` av\x98\x839\x81Q\x91R`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[Pa\x05\x03aC\x94V[`\0c\x01\xFF\xC9\xA7`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x14\x80a\x12BWPcl\xDB=\x13`\xE1\x1B`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x14[\x80a\x12]WPc\x03\xA2M\x07`\xE2\x1B`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x14[\x92\x91PPV[`@Qc\x03\xA2M\x07`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R``\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\x0E\x894\x1C\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x13\x04W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` av\xB8\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x13\x18W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x12]\x91\x90\x81\x01\x90amZV[`\0a\x13K\x85a!]V[`@Qc\x19\x05x\x07`\xE0\x1B\x81R`\x01`\x01`@\x1B\x03\x87\x16`\x04\x82\x01R\x85\x15\x15`$\x82\x01R`D\x81\x01\x85\x90R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`d\x83\x01R\x91\x90\x91\x16\x90c\x19\x05x\x07\x90`\x84\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x13\xEAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` av\xB8\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x13\xFEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14\"\x91\x90anDV[\x95\x94PPPPPV[`\0a\x145aC\xB1V[`\0b\xFF\xFF\xFF\x8B\x16\x15a\x14HW\x8Aa\x14QV[`\x06Tb\xFF\xFF\xFF\x16[\x90P\x80b\xFF\xFF\xFF\x16`\0\x03a\x14yW`@Qc\xCCzs\x9B`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[b\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\x08` R`@\x81 \x80T\x82\x90a\x14\xA0\x90c\xFF\xFF\xFF\xFF\x16anvV[\x91\x90a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90U\x90Pa\x14\xE1`\0\x80`\x01`\x01`\xA0\x1B\x03\x16\x89`\x01`\x01`\xA0\x1B\x03\x16\x14\x15\x84\x84aD\x08V[`\x01`\x01`@\x1B\x03\x81\x16`\0\x90\x81R`\n` R`@\x90 \x90\x93Pa\x15\x12\x90\x8C\x8Ca\xFF\xFF\x80\x8E\x16\x90\x8D\x16\x8C\x8CaDfV[`\x0F\x80Th\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x19\x16a\x01\0`\x01`\x01`@\x1B\x03\x86\x16\x02\x17\x90U`\0a\x15>\x84a!]V[`\x01`\x01`\xA0\x1B\x03\x16c\xE0hx\x7F\x85\x88\x88`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x15m\x93\x92\x91\x90an\xC2V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x15\xC5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` av\xB8\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x15\xD9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\xFD\x91\x90an\xE5V[\x90P\x80a\x16\x1DW`@Qc\x1B\xE2\xB1K`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\t`\0\x84b\xFF\xFF\xFF\x16b\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\x01\x01`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16`\t`\0\x85b\xFF\xFF\xFF\x16b\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x01`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x85`\x01`\x01`@\x1B\x03\x16\x7Fa<|I?\x9A\xA3\x97M\xCD\xBB\x87\xE6\xFE\xA5L\xCD\xB9\x10>\xA7\x83.m\xF0\x8D\xB8\x1Co\x90\xD2\x9D\x8F\x8F\x8F\x8F\x8F\x8F`@Qa\x17\x0E\x96\x95\x94\x93\x92\x91\x90\x95\x86R` \x86\x01\x94\x90\x94Ra\xFF\xFF\x92\x83\x16`@\x86\x01R\x91\x16``\x84\x01R`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\x80\x84\x01R\x16`\xA0\x82\x01R`\xC0\x01\x90V[`@Q\x80\x91\x03\x90\xA4a\x17\x1EaF\xB5V[PPP\x99\x98PPPPPPPPPV[`\x01`\x01`@\x1B\x03\x81\x16`\0\x90\x81R`\n` \x90\x81R`@\x80\x83 \x81Qa\x01\0\x81\x01\x83R\x81T`\x01`\x01`\x80\x1B\x03\x80\x82\x16\x83R`\x01`\x80\x1B\x91\x82\x90\x04\x81\x16\x95\x83\x01\x95\x90\x95R`\x01\x83\x01T\x94\x85\x16\x93\x82\x01\x93\x90\x93R\x91\x83\x04c\xFF\xFF\xFF\xFF\x16``\x83\x01R`\x01`\xA0\x1B\x83\x04a\xFF\xFF\x90\x81\x16`\x80\x84\x01R`\x01`\xB0\x1B\x90\x93\x04\x90\x92\x16`\xA0\x82\x01R`\x02\x82\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\xC0\x83\x01R`\x03\x90\x92\x01T\x90\x91\x16`\xE0\x82\x01R\x81\x90\x81\x90\x81\x90a\x17\xE4\x90aG\0V[` \x87\x81\x1Cb\xFF\xFF\xFF\x16`\0\x90\x81R`\t\x82R`@\x90\x81\x90 \x81Q`\x80\x81\x01\x83R\x81T`\x01`\x01`\xA0\x1B\x03\x80\x82\x16\x83R`\xFF`\x01`\xA0\x1B\x92\x83\x90\x04\x81\x16\x96\x84\x01\x87\x90R`\x01\x90\x94\x01T\x90\x81\x16\x94\x83\x01\x94\x90\x94R\x90\x92\x04\x16``\x82\x01R`\x01`\x01`\x80\x1B\x03\x93\x84\x16\x95P\x91\x90\x92\x16\x92P\x90a\x18_\x90\x84\x90aGYV[\x94Pa\x18{\x81``\x01Q`\xFF\x16\x83aGY\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x93PPPP\x91P\x91V[\x84\x83\x14a\x18\xCBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rn\t\x88\xA9\xC8\xEA\x89\x0B\xE9\xA9*i\xA8*\x88i`\x8B\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[3`\x01`\x01`\xA0\x1B\x03\x89\x16\x14\x80a\x19\x05WP`\x01`\x01`\xA0\x1B\x03\x88\x16`\0\x90\x81R`\x01` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x90 T`\xFF\x16[a\x19BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rm\x13\x93\xD5\x17\xD0UU\x12\x13\xD4\x92V\x91Q`\x92\x1B`D\x82\x01R`d\x01a\x18\xC2V[`\0\x80`\0[\x87\x81\x10\x15a\x19\xFDW\x88\x88\x82\x81\x81\x10a\x19bWa\x19bao\x05V[\x90P` \x02\x015\x92P\x86\x86\x82\x81\x81\x10a\x19}Wa\x19}ao\x05V[`\x01`\x01`\xA0\x1B\x03\x8E\x16`\0\x90\x81R` \x81\x81R`@\x80\x83 \x89\x84R\x82R\x82 \x80T\x93\x90\x91\x02\x94\x90\x94\x015\x95P\x85\x93\x92P\x90a\x19\xBA\x90\x84\x90ao\x1BV[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x8A\x16`\0\x90\x81R` \x81\x81R`@\x80\x83 \x86\x84R\x90\x91R\x81 \x80T\x84\x92\x90a\x19\xF0\x90\x84\x90ao.V[\x90\x91UPP`\x01\x01a\x19HV[P\x88`\x01`\x01`\xA0\x1B\x03\x16\x8A`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x7FJ9\xDC\x06\xD4\xC0\xDB\xC6Kp\xAF\x90\xFDi\x8A#:Q\x8A\xA5\xD0~Y]\x98;\x8C\x05&\xC8\xF7\xFB\x8B\x8B\x8B\x8B`@Qa\x1AQ\x94\x93\x92\x91\x90ao\xBDV[`@Q\x80\x91\x03\x90\xA4`\x01`\x01`\xA0\x1B\x03\x89\x16;\x15a\x1BaW`@Qc\xBC\x19|\x81`\xE0\x1B\x80\x82R\x90`\x01`\x01`\xA0\x1B\x03\x8B\x16\x90c\xBC\x19|\x81\x90a\x1A\xA5\x903\x90\x8F\x90\x8E\x90\x8E\x90\x8E\x90\x8E\x90\x8E\x90\x8E\x90`\x04\x01ao\xE4V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1A\xFDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` av\xB8\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x1B\x11W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B5\x91\x90apHV[`\x01`\x01`\xE0\x1B\x03\x19\x16\x14a\x1B\\W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x18\xC2\x90aphV[a\x1B\x87V[`\x01`\x01`\xA0\x1B\x03\x89\x16a\x1B\x87W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x18\xC2\x90ap\x92V[PPPPPPPPPPV[`\0\x80a\x1B\x9EaC\xB1V[`\x0FT`\xFF\x16\x15\x15`\0\x03a\x1B\xB5Wa\x1B\xB5aGoV[\x85`\x01`\x01`@\x1B\x03\x16`\0\x03a\x1B\xDAW`\x0FTa\x01\0\x90\x04`\x01`\x01`@\x1B\x03\x16\x95P[`\0a\x1B\xE5\x87a!]V[`\x01`\x01`\xA0\x1B\x03\x16\x03a\x1C\x17W`@Qc\t\x0Bp}`\xE1\x1B\x81R`\x01`\x01`@\x1B\x03\x87\x16`\x04\x82\x01R`$\x01a\x18\xC2V[a\x1C \x86a!]V[`@Qc\xE6\x04{\x19`\xE0\x1B\x81R`\x01`\x01`@\x1B\x03\x88\x16`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90c\xE6\x04{\x19\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x1C\xA8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` av\xB8\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x1C\xBCW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C\xE0\x91\x90an\xE5V[a\x1D\x08W`@Qc\xBC'\xA5\x17`\xE0\x1B\x81R`\x01`\x01`@\x1B\x03\x87\x16`\x04\x82\x01R`$\x01a\x18\xC2V[a\x1D%\x86\x85`\x01`\x01`\x80\x1B\x03\x16\x85`\x01`\x01`\x80\x1B\x03\x16aG\xD7V[` \x88\x81\x1Cb\xFF\xFF\xFF\x16`\0\x90\x81R`\t\x82R`@\x90\x81\x90 \x81Q`\x80\x81\x01\x83R\x81T`\x01`\x01`\xA0\x1B\x03\x80\x82\x16\x83R`\xFF`\x01`\xA0\x1B\x92\x83\x90\x04\x81\x16\x96\x84\x01\x96\x90\x96R`\x01\x90\x93\x01T\x92\x83\x16\x93\x82\x01\x93\x90\x93R\x91\x90\x04\x90\x91\x16``\x82\x01R\x91\x95P\x93P\x88\x15a\x1E\xA2W`\0a\x1D\x9E\x82`\0\x01Qa\"PV[\x90P`\0a\x1D\xAF\x83`@\x01Qa\"PV[\x90P`\0\x82\x12\x15a\x1D\xBFW`\0\x91P[`\0\x81\x12\x15a\x1D\xCCWP`\0[`\0\x80a\x1D\xDA\x8B\x85\x85aG\xD7V[`\x01`\x01`@\x1B\x03\x8D\x16`\0\x90\x81R`\n` \x90\x81R`@\x91\x82\x90 \x82Qa\x01\0\x81\x01\x84R\x81T`\x01`\x01`\x80\x1B\x03\x81\x81\x16\x83R`\x01`\x80\x1B\x91\x82\x90\x04\x81\x16\x94\x83\x01\x94\x90\x94R`\x01\x83\x01T\x80\x85\x16\x95\x83\x01\x95\x90\x95Rc\xFF\xFF\xFF\xFF\x90\x85\x04\x81\x16``\x83\x01Ra\xFF\xFF`\x01`\xA0\x1B\x86\x04\x81\x16`\x80\x84\x01R`\x01`\xB0\x1B\x90\x95\x04\x90\x94\x16`\xA0\x82\x01R`\x02\x82\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\xC0\x83\x01R`\x03\x90\x92\x01T\x90\x91\x16`\xE0\x82\x01R\x93\x95P\x91\x93Pa\x1E\x9B\x92\x91\x80\x86\x16\x91\x90\x85\x16\x90aH\xB3\x16V[\x99PPPPP[\x85`\x01`\x01`\x80\x1B\x03\x16`\0\x03a\x1E\xCCW`@Qc\x90`\x9A}`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1F\x86a\x1E\xD8\x87aI\xAAV[`\x01`\x01`@\x1B\x03\x89\x16`\0\x90\x81R`\n` \x90\x81R`@\x91\x82\x90 \x82Qa\x01\0\x81\x01\x84R\x81T`\x01`\x01`\x80\x1B\x03\x80\x82\x16\x83R`\x01`\x80\x1B\x91\x82\x90\x04\x81\x16\x94\x83\x01\x94\x90\x94R`\x01\x83\x01T\x93\x84\x16\x94\x82\x01\x94\x90\x94Rc\xFF\xFF\xFF\xFF\x93\x83\x04\x84\x16``\x82\x01Ra\xFF\xFF`\x01`\xA0\x1B\x84\x04\x81\x16`\x80\x83\x01R`\x01`\xB0\x1B\x90\x93\x04\x90\x92\x16`\xA0\x83\x01R`\x02\x81\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\xC0\x84\x01R`\x03\x90\x91\x01T\x16`\xE0\x82\x01R\x91\x90aI\xC4\x16V[`\x01`\x01`\x80\x1B\x03\x91\x82\x16\x94P\x81\x16\x92P\x85\x16\x83\x11\x15a\x1F\xB9W`@QcVr\x0E\x1D`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x83`\x01`\x01`\x80\x1B\x03\x16\x82\x11\x15a\x1F\xE3W`@Qc!0\x16\x97`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0`@Q\x80a\x01\0\x01`@R\x80B\x81R` \x01\x85\x81R` \x01\x84\x81R` \x01a \x0C\x89aI\xAAV[`\x0F\x0B\x81R` \x01\x89`\x01`\x01`@\x1B\x03\x16\x81R` \x01\x8A`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83`@\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81RP\x90Pa a\x81aJ\xB4V[` \x82\x01Qa t\x90\x85\x90`\xFF\x16aGYV[``\x83\x01Qa \x87\x90\x85\x90`\xFF\x16aGYV[\x90\x94P\x92P\x83\x15\x80\x15a \x98WP\x82\x15[\x15a \xB6W`@Qce\x8B\x16\xED`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`@\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x82`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x89`\x01`\x01`@\x1B\x03\x16\x7F\xFD\xFF\xEC\xA7Q\xF0\xDC\xAA\xB7U1\xCB\x81<\x12\xBB\xFDV\xEE>\x96L\xC4q\xD7\xEFC\x93$\x02\xEE\x18\x87\x87\x8C`@Qa!)\x93\x92\x91\x90\x92\x83R` \x83\x01\x91\x90\x91R`\x01`\x01`\x80\x1B\x03\x16`@\x82\x01R``\x01\x90V[`@Q\x80\x91\x03\x90\xA4`\x0FT`\xFF\x16\x15\x15`\0\x03a!HWa!HaMpV[a!PaF\xB5V[PP\x96P\x96\x94PPPPPV[`\x01`\x01`@\x1B\x03\x16`\0\x90\x81R`\n` R`@\x90 `\x03\x01T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\0a!\x8F\x82a!]V[`@Qc\x1C\xA1\xA6\xAD`\xE1\x1B\x81R`\x01`\x01`@\x1B\x03\x84\x16`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90c9CMZ\x90`$\x01[` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\"\x18W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` av\xB8\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\",W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12]\x91\x90anDV[`\0a\x12]`\x02\x830aQ\xABV[``\x83\x82\x14a\"\xA1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rn\t\x88\xA9\xC8\xEA\x89\x0B\xE9\xA9*i\xA8*\x88i`\x8B\x1B`D\x82\x01R`d\x01a\x18\xC2V[\x83`\x01`\x01`@\x1B\x03\x81\x11\x15a\"\xB9Wa\"\xB9ai\x18V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\"\xE2W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x84\x81\x10\x15a#\x89W`\0\x80\x87\x87\x84\x81\x81\x10a#\x05Wa#\x05ao\x05V[\x90P` \x02\x01` \x81\x01\x90a#\x1A\x91\x90ag\xAEV[`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x85\x85\x84\x81\x81\x10a#NWa#Nao\x05V[\x90P` \x02\x015\x81R` \x01\x90\x81R` \x01`\0 T\x82\x82\x81Q\x81\x10a#vWa#vao\x05V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\"\xE8V[P\x94\x93PPPPV[`\0\x80a#\x9DaC\xB1V[`\x0FT`\xFF\x16\x15\x15`\0\x03a#\xB4Wa#\xB4aGoV[a#\xBD\x86a!]V[`@Qc\xE6\x04{\x19`\xE0\x1B\x81R`\x01`\x01`@\x1B\x03\x88\x16`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90c\xE6\x04{\x19\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a$EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` av\xB8\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a$YW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a$}\x91\x90an\xE5V[a$\xA5W`@Qc\t\x0Bp}`\xE1\x1B\x81R`\x01`\x01`@\x1B\x03\x87\x16`\x04\x82\x01R`$\x01a\x18\xC2V[a$\xC2\x86\x85`\x01`\x01`\x80\x1B\x03\x16\x85`\x01`\x01`\x80\x1B\x03\x16aG\xD7V[` \x88\x81\x1Cb\xFF\xFF\xFF\x16`\0\x90\x81R`\t\x82R`@\x90\x81\x90 \x81Q`\x80\x81\x01\x83R\x81T`\x01`\x01`\xA0\x1B\x03\x80\x82\x16\x80\x84R`\xFF`\x01`\xA0\x1B\x93\x84\x90\x04\x81\x16\x97\x85\x01\x97\x90\x97R`\x01\x90\x94\x01T\x90\x81\x16\x94\x83\x01\x85\x90R\x04\x90\x93\x16``\x84\x01R\x93\x97P\x91\x95P\x91\x90\x89\x15a%[W3`\0\x90\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`@\x1B\x03\x8D\x16\x84R\x90\x91R\x90 Ta%X\x90aR\xC3V[\x97P[\x87`\x01`\x01`\x80\x1B\x03\x16`\0\x03a%\x85W`@Qc\nw\xB0/`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a&Ha%\x91\x89aI\xAAV[a%\x9A\x90ap\xBDV[`\x01`\x01`@\x1B\x03\x8B\x16`\0\x90\x81R`\n` \x90\x81R`@\x91\x82\x90 \x82Qa\x01\0\x81\x01\x84R\x81T`\x01`\x01`\x80\x1B\x03\x80\x82\x16\x83R`\x01`\x80\x1B\x91\x82\x90\x04\x81\x16\x94\x83\x01\x94\x90\x94R`\x01\x83\x01T\x93\x84\x16\x94\x82\x01\x94\x90\x94Rc\xFF\xFF\xFF\xFF\x93\x83\x04\x84\x16``\x82\x01Ra\xFF\xFF`\x01`\xA0\x1B\x84\x04\x81\x16`\x80\x83\x01R`\x01`\xB0\x1B\x90\x93\x04\x90\x92\x16`\xA0\x83\x01R`\x02\x81\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\xC0\x84\x01R`\x03\x90\x91\x01T\x16`\xE0\x82\x01R\x91\x90aI\xC4\x16V[`\x01`\x01`\x80\x1B\x03\x91\x82\x16\x96P\x81\x16\x94P\x87\x16\x85\x10\x15a&{W`@Qc\x064HC`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x85`\x01`\x01`\x80\x1B\x03\x16\x84\x10\x15a&\xA5W`@QcVZ\xDE\xF5`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0`@Q\x80a\x01\0\x01`@R\x80B\x81R` \x01\x87\x81R` \x01\x86\x81R` \x01a&\xCE\x8BaI\xAAV[a&\xD7\x90ap\xBDV[`\x0F\x0B\x81R`\x01`\x01`@\x1B\x03\x8C\x16` \x82\x01R3`@\x82\x01R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16``\x83\x01R\x84\x16`\x80\x90\x91\x01R\x90Pa'\x14\x81aJ\xB4V[` \x84\x01Qa''\x90\x87\x90`\xFF\x16aGYV[``\x85\x01Qa':\x90\x87\x90`\xFF\x16aGYV[`@\x80Q\x83\x81R` \x81\x01\x83\x90R`\x01`\x01`\x80\x1B\x03\x8D\x16\x81\x83\x01R\x90Q\x92\x98P\x90\x96P`\x01`\x01`\xA0\x1B\x03\x84\x81\x16\x92\x90\x86\x16\x91`\x01`\x01`@\x1B\x03\x8E\x16\x91\x7F0\x84\xCA\xF4\x89f\\\xAB\x07E,\xFEO=\x0E\xB5\xE0\xDC\x15\xEA\xC6\xFCe\x80\x98\x85\x8Ec\x9Ep\xE5:\x91\x81\x90\x03``\x01\x90\xA4`\x0FT`\xFF\x16\x15\x15`\0\x03a'\xBAWa'\xBAaMpV[a'\xC2aF\xB5V[PPPP\x95P\x95\x93PPPPV[a'\xD8aC\xB1V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xF7|G\x91`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a(oW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` av\xB8\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a(\x83W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a(\xA7\x91\x90ap\xE3V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a(\xDBW`@Q`\x01b\x10X\xF5`\xE2\x1B\x03\x19\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x14\x81\x11\x80a(\xEAWP`\x04\x81\x10[\x15a)\x0BW`@QcdYtw`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x18\xC2V[`\r\x80T\x90\x82\x90U`@\x80Q\x82\x81R` \x81\x01\x84\x90R\x7F\x81\xC99\x14H\0(v\x03G\x9B\x97\xBB\xA9\xC1\x12\x88\xCEz\xBCZ\xCBH\x90y\xE1Y\xF3\\\xF9\x8B\xD1\x91\x01`@Q\x80\x91\x03\x90\xA1a)TaF\xB5V[PPV[`\0\x80`\0a)j\x86``\x01Qa!]V[`\x01`\x01`\xA0\x1B\x03\x16c\x80\xAF\x9Dv\x87\x87\x87`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a)\x99\x93\x92\x91\x90aq\x03V[```@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a)\xEFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` av\xB8\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a*\x03W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a*'\x91\x90aq0V[\x92P\x92P\x92P\x93P\x93P\x93\x90PV[`\x01`\x01`@\x1B\x03\x82\x16`\0\x90\x81R`\n` \x90\x81R`@\x80\x83 \x81Qa\x01\0\x81\x01\x83R\x81T`\x01`\x01`\x80\x1B\x03\x80\x82\x16\x83R`\x01`\x80\x1B\x91\x82\x90\x04\x81\x16\x95\x83\x01\x95\x90\x95R`\x01\x83\x01T\x94\x85\x16\x93\x82\x01\x93\x90\x93Rc\xFF\xFF\xFF\xFF\x92\x84\x04\x83\x16``\x82\x01Ra\xFF\xFF`\x01`\xA0\x1B\x85\x04\x81\x16`\x80\x83\x01R`\x01`\xB0\x1B\x90\x94\x04\x90\x93\x16`\xA0\x84\x01R`\x02\x81\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\xC0\x85\x01R`\x03\x90\x91\x01T\x16`\xE0\x83\x01R\x82\x91\x82\x91\x82\x91a*\xED\x91\x87\x90aI\xC4\x16V[` \x88\x81\x1Cb\xFF\xFF\xFF\x16`\0\x90\x81R`\t\x82R`@\x80\x82 \x81Q`\x80\x81\x01\x83R\x81T`\x01`\x01`\xA0\x1B\x03\x80\x82\x16\x83R`\xFF`\x01`\xA0\x1B\x92\x83\x90\x04\x81\x16\x97\x84\x01\x97\x90\x97R`\x01\x90\x93\x01T\x92\x83\x16\x93\x82\x01\x93\x90\x93R\x91\x90\x04\x90\x92\x16``\x83\x01R`\x01`\x01`\x80\x1B\x03\x93\x84\x16\x95P\x91\x90\x92\x16\x92P`\x0F\x87\x90\x0B\x12\x15a+\xB1Wa+\x8Ba+\x86\x82` \x01Q`\xFF\x16\x85aGY\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[aR\xC3V[\x94Pa+\xAAa+\x86\x82``\x01Q`\xFF\x16\x84aGY\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x93Pa+\xF0V[a+\xCEa+\x86\x82` \x01Q`\xFF\x16\x85aR\xD5\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x94Pa+\xEDa+\x86\x82``\x01Q`\xFF\x16\x84aR\xD5\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x93P[PPP\x92P\x92\x90PV[a,\x02aC\xB1V[`\x01`\x01`@\x1B\x03\x83\x16`\0\x90\x81R`\n` R`@\x90 `\x02\x81\x01T`\x01`\x01`\xA0\x1B\x03\x163\x14a,JW`@Q`\x01b\x10X\xF5`\xE2\x1B\x03\x19\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\xFF\xFF\x82\x16\x15a,\xB7Wa,wa\xFF\xFF\x83\x16`\x01a\x03\xE8\x81\x83\x14\x91\x83\x11\x91\x90\x91\x17\x82\x82\x14\x91\x90\x92\x10\x17\x16\x90V[a,\x9AW`@Qc\x97\x1B1\t`\xE0\x1B\x81Ra\xFF\xFF\x83\x16`\x04\x82\x01R`$\x01a\x18\xC2V[`\x01\x81\x01\x80Ta\xFF\xFF`\xA0\x1B\x19\x16`\x01`\xA0\x1Ba\xFF\xFF\x85\x16\x02\x17\x90U[a\xFF\xFF\x83\x16\x15a-*W`\x01\x81\x81\x01T`\x01`\xA0\x1B\x90\x04a\xFF\xFF\x90\x81\x16\x90\x85\x16\x81\x81\x10\x91\x81\x14\x91\x90\x91\x17\x82\x82\x11\x91\x90\x92\x14\x17\x16a-\rW`@Qc\xED\xDF\xE1\x19`\xE0\x1B\x81Ra\xFF\xFF\x84\x16`\x04\x82\x01R`$\x01a\x18\xC2V[`\x01\x81\x01\x80Ta\xFF\xFF`\xB0\x1B\x19\x16`\x01`\xB0\x1Ba\xFF\xFF\x86\x16\x02\x17\x90U[\x81a\xFF\xFF\x16\x83a\xFF\xFF\x16\x85`\x01`\x01`@\x1B\x03\x16\x7F\x80N\x0E\xF8\xEB\xD1\x19o\x98\xB3\xC6\xA20\xDE\xFF\xD8\xCF\xE0<\xB1\x92?\xE0\xE4\x02%-\x06\xD8\xD4v\xDA`@Q`@Q\x80\x91\x03\x90\xA4a-raF\xB5V[PPPPV[3`\0\x81\x81R`\x01` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x87\x16\x80\x85R\x90\x83R\x92\x81\x90 \x80T`\xFF\x19\x16\x86\x15\x15\x90\x81\x17\x90\x91U\x90Q\x90\x81R\x91\x92\x91\x7F\x170~\xAB9\xABa\x07\xE8\x89\x98E\xAD=Y\xBD\x96S\xF2\0\xF2 \x92\x04\x89\xCA+Y7il1\x91\x01`@Q\x80\x91\x03\x90\xA3PPV[`\x0FT``\x90`\xFF\x16\x15a.\x0BW`@QcU\xE1\xF7\xC5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a.\x13aC\xB1V[`\x0F\x80T`\xFF\x19\x16`\x01\x17\x90Ua.(aGoV[\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a.@Wa.@ai\x18V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a.sW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a.^W\x90P[P\x90P`\0[\x82\x81\x10\x15a/<W`\0\x800\x86\x86\x85\x81\x81\x10a.\x97Wa.\x97ao\x05V[\x90P` \x02\x81\x01\x90a.\xA9\x91\x90aq\xAFV[`@Qa.\xB7\x92\x91\x90arxV[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14a.\xF2W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a.\xF7V[``\x91P[P\x91P\x91P\x81a/\tW\x80Q\x81` \x01\xFD[\x80\x84\x84\x81Q\x81\x10a/\x1CWa/\x1Cao\x05V[` \x02` \x01\x01\x81\x90RPPP\x80\x80a/4\x90ar\x88V[\x91PPa.yV[P`\x0F\x80T`\xFF\x19\x16\x90Ua/OaMpV[a\x12]aF\xB5V[`\0a/aaC\xB1V[\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x03a/\x93W`@Qc\x01D\xD33`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P`\x01`\x01`\xA0\x1B\x03\x80\x83\x16`\0\x90\x81R`\x0B` \x90\x81R`@\x80\x83 \x93\x85\x16\x83R\x92\x90R Tb\xFF\xFF\xFF\x16\x80\x15a/\xE5W`@Qc\xB0\x98\x8CC`\xE0\x1B\x81Rb\xFF\xFF\xFF\x82\x16`\x04\x82\x01R`$\x01a\x18\xC2V[`\0\x80\x84`\x01`\x01`\xA0\x1B\x03\x16c1<\xE5g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a0_W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` av\xB8\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a0sW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a0\x97\x91\x90ar\xA1V[\x84`\x01`\x01`\xA0\x1B\x03\x16c1<\xE5g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a1\x0EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` av\xB8\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a1\"W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a1F\x91\x90ar\xA1V[\x90\x92P\x90Pa1l`\xFF\x83\x16`\x06`\x12\x81\x83\x14\x91\x83\x11\x91\x90\x91\x17\x82\x82\x14\x91\x90\x92\x10\x17\x16\x90V[a1\x8EW`@Qc\xC3\xDAwG`\xE0\x1B\x81R`\xFF\x83\x16`\x04\x82\x01R`$\x01a\x18\xC2V[a1\xAF`\xFF\x82\x16`\x06`\x12\x81\x83\x14\x91\x83\x11\x91\x90\x91\x17\x82\x82\x14\x91\x90\x92\x10\x17\x16\x90V[a1\xD1W`@Qc\xC3\xDAwG`\xE0\x1B\x81R`\xFF\x82\x16`\x04\x82\x01R`$\x01a\x18\xC2V[`\x06\x80T`\0\x90a1\xE6\x90b\xFF\xFF\xFF\x16ar\xC7V[\x82Ta\x01\0\x92\x90\x92\nb\xFF\xFF\xFF\x81\x81\x02\x19\x90\x93\x16\x92\x82\x16\x90\x81\x02\x92\x90\x92\x17\x90\x92U`\x01`\x01`\xA0\x1B\x03\x87\x81\x16`\0\x81\x81R`\x0B` \x90\x81R`@\x80\x83 \x8B\x86\x16\x80\x85R\x90\x83R\x81\x84 \x80Tb\xFF\xFF\xFF\x19\x16\x88\x17\x90U\x81Q`\x80\x81\x01\x83R\x85\x81R`\xFF\x8B\x81\x16\x82\x86\x01\x81\x81R\x83\x86\x01\x85\x81R\x8D\x84\x16``\x86\x01\x81\x81R\x8D\x8BR`\t\x8AR\x99\x88\x90 \x95Q\x86T\x93Q\x90\x8D\x16`\x01`\x01`\xA8\x1B\x03\x19\x94\x85\x16\x17`\x01`\xA0\x1B\x91\x87\x16\x82\x02\x17\x87U\x91Q`\x01\x96\x90\x96\x01\x80T\x9AQ\x96\x90\x9C\x16\x99\x90\x92\x16\x98\x90\x98\x17\x93\x90\x92\x16\x90\x96\x02\x91\x90\x91\x17\x90\x96U\x81Q\x93\x84R\x91\x83\x01\x94\x90\x94R\x94\x97P\x90\x92\x91\x7F\xC0\xC5\xDF\x98\xA4\xCA\x87\xA3!\xA3;\xF1'|\xF3-1\xA9{l\xE1K\x97G8!I\xB9\xE2c\x1E\xA3\x91\x01`@Q\x80\x91\x03\x90\xA4a2\xFAaF\xB5V[PP\x92\x91PPV[` \x83\x81\x1Cb\xFF\xFF\xFF\x16`\0\x90\x81R`\t\x82R`@\x80\x82 \x81Q`\x80\x81\x01\x83R\x81T`\x01`\x01`\xA0\x1B\x03\x80\x82\x16\x83R`\xFF`\x01`\xA0\x1B\x92\x83\x90\x04\x81\x16\x97\x84\x01\x88\x90R`\x01\x90\x94\x01T\x90\x81\x16\x94\x83\x01\x94\x90\x94R\x90\x92\x04\x16``\x82\x01R\x90\x91a3j\x90\x85\x90aS\x15V[``\x82\x01Qa3}\x90\x85\x90`\xFF\x16aS\x15V[`\x01`\x01`@\x1B\x03\x87\x16`\0\x90\x81R`\n` \x90\x81R`@\x91\x82\x90 \x82Qa\x01\0\x81\x01\x84R\x81T`\x01`\x01`\x80\x1B\x03\x80\x82\x16\x83R`\x01`\x80\x1B\x91\x82\x90\x04\x81\x16\x94\x83\x01\x94\x90\x94R`\x01\x83\x01T\x93\x84\x16\x94\x82\x01\x94\x90\x94Rc\xFF\xFF\xFF\xFF\x93\x83\x04\x84\x16``\x82\x01Ra\xFF\xFF`\x01`\xA0\x1B\x84\x04\x81\x16`\x80\x83\x01R`\x01`\xB0\x1B\x90\x93\x04\x90\x92\x16`\xA0\x83\x01R`\x02\x81\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\xC0\x84\x01R`\x03\x90\x91\x01T\x16`\xE0\x82\x01R\x92\x96P\x90\x94Pa48\x91\x90\x86\x90\x86\x90aH\xB3\x16V[\x91PP[\x93\x92PPPV[a4KaC\xB1V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xF7|G\x91`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a4\xE2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` av\xB8\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a4\xF6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a5\x1A\x91\x90ap\xE3V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a5NW`@Q`\x01b\x10X\xF5`\xE2\x1B\x03\x19\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80\x83`\x01`\x01`\xA0\x1B\x03\x16c1<\xE5g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a5\xC8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` av\xB8\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a5\xDCW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a6\0\x91\x90ar\xA1V[\x90P`\0\x19\x83\x03a6:W`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81R`\x07` R`@\x90 T\x91Pa63\x82`\xFF\x83\x16aGYV[\x92Pa6JV[a6G\x83`\xFF\x83\x16aS\x15V[\x91P[`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81R`\x07` R`@\x81 \x80T\x84\x92\x90a6r\x90\x84\x90ao\x1BV[\x90\x91UPa6\x82\x90P\x84\x83aS,V[\x83`\x01`\x01`\xA0\x1B\x03\x16\x7F\x1F\xDD\0 5\x88\x93U\x97\x13\xDE\xF8\xB4,\xADf\x1F\xFB\xC7U\xD1\xA2dY@'\x92\x14B\xBBV\xA0\x84`@Qa6\xBD\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2`\x0FT`\xFF\x16\x15\x15`\0\x03a6\xDCWa6\xDCaMpV[a-raF\xB5V[`\0a6\xEF\x82a!]V[`@Qc8\xCCn\x8D`\xE2\x1B\x81R`\x01`\x01`@\x1B\x03\x84\x16`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90c\xE31\xBA4\x90`$\x01a!\xC2V[`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x91\x90\x91Ra7Z\x84a!]V[`@Qcx=\xC3\xCF`\xE1\x1B\x81R`\x01`\x01`@\x1B\x03\x86\x16`\x04\x82\x01R\x84\x15\x15`$\x82\x01R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`D\x83\x01R\x91\x90\x91\x16\x90c\xF0{\x87\x9E\x90`d\x01`\xA0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a7\xF2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` av\xB8\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a8\x06W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a8*\x91\x90ar\xDFV[\x94\x93PPPPV[3`\x01`\x01`\xA0\x1B\x03\x87\x16\x14\x80a8lWP`\x01`\x01`\xA0\x1B\x03\x86\x16`\0\x90\x81R`\x01` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x90 T`\xFF\x16[a8\xA9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rm\x13\x93\xD5\x17\xD0UU\x12\x13\xD4\x92V\x91Q`\x92\x1B`D\x82\x01R`d\x01a\x18\xC2V[`\x01`\x01`\xA0\x1B\x03\x86\x16`\0\x90\x81R` \x81\x81R`@\x80\x83 \x87\x84R\x90\x91R\x81 \x80T\x85\x92\x90a8\xDA\x90\x84\x90ao\x1BV[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R` \x81\x81R`@\x80\x83 \x87\x84R\x90\x91R\x81 \x80T\x85\x92\x90a9\x10\x90\x84\x90ao.V[\x90\x91UPP`@\x80Q\x85\x81R` \x81\x01\x85\x90R`\x01`\x01`\xA0\x1B\x03\x80\x88\x16\x92\x90\x89\x16\x913\x91\x7F\xC3\xD5\x81h\xC5\xAEs\x97s\x1D\x06=[\xBF=exTBsC\xF4\xC0\x83$\x0Fz\xAC\xAA-\x0Fb\x91\x01`@Q\x80\x91\x03\x90\xA4`\x01`\x01`\xA0\x1B\x03\x85\x16;\x15a:dW`@Qc\xF2:na`\xE0\x1B\x80\x82R\x90`\x01`\x01`\xA0\x1B\x03\x87\x16\x90c\xF2:na\x90a9\xA8\x903\x90\x8B\x90\x8A\x90\x8A\x90\x8A\x90\x8A\x90`\x04\x01as\\V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a:\0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` av\xB8\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a:\x14W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a:8\x91\x90apHV[`\x01`\x01`\xE0\x1B\x03\x19\x16\x14a:_W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x18\xC2\x90aphV[a:\x8AV[`\x01`\x01`\xA0\x1B\x03\x85\x16a:\x8AW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x18\xC2\x90ap\x92V[PPPPPPV[`\0\x80`\0a:\x9FaC\xB1V[`\x0FT`\xFF\x16\x15\x15`\0\x03a:\xB6Wa:\xB6aGoV[a:\xC6`\x80\x85\x01``\x86\x01ae\xA1V[a:\xD3` \x86\x01\x86as\xA3V[a:\xE3`@\x87\x01` \x88\x01as\xA3V[\x91\x94P`\x01`\x01`\x80\x1B\x03\x90\x81\x16\x93P\x16\x90P`\0`\n\x81a;\x0B`\x80\x88\x01``\x89\x01ae\xA1V[`\x01`\x01`@\x1B\x03\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x90\x81\x01`\0 \x81Qa\x01\0\x81\x01\x83R\x81T`\x01`\x01`\x80\x1B\x03\x80\x82\x16\x83R`\x01`\x80\x1B\x91\x82\x90\x04\x81\x16\x95\x83\x01\x95\x90\x95R`\x01\x83\x01T\x94\x85\x16\x93\x82\x01\x93\x90\x93R\x91\x83\x04c\xFF\xFF\xFF\xFF\x16``\x83\x01\x81\x90R`\x01`\xA0\x1B\x84\x04a\xFF\xFF\x90\x81\x16`\x80\x85\x01R`\x01`\xB0\x1B\x90\x94\x04\x90\x93\x16`\xA0\x83\x01R`\x02\x81\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\xC0\x84\x01R`\x03\x82\x01T\x16`\xE0\x90\x92\x01\x91\x90\x91R\x91Pa;\xE0W`@Qc\t\x0Bp}`\xE1\x1B\x81R`\x01`\x01`@\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\x18\xC2V[`@\x80Qa\x01\0\x81\x01\x82R\x82T`\x01`\x01`\x80\x1B\x03\x80\x82\x16\x83R`\x01`\x80\x1B\x91\x82\x90\x04\x81\x16` \x84\x01R`\x01\x85\x01T\x90\x81\x16\x93\x83\x01\x93\x90\x93Rc\xFF\xFF\xFF\xFF\x90\x83\x04\x16``\x82\x01Ra\xFF\xFF`\x01`\xA0\x1B\x83\x04\x81\x16`\x80\x83\x01R`\x01`\xB0\x1B\x90\x92\x04\x90\x91\x16`\xA0\x82\x01R`\x02\x82\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\xC0\x83\x01R`\x03\x83\x01T\x16`\xE0\x82\x01Ra<q\x90aS\x7FV[\x15a<\x8FW`@Qcz\x95\xCB!`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a<\x9A\x85a!]V[`@Qc\xE6\x04{\x19`\xE0\x1B\x81R`\x01`\x01`@\x1B\x03\x87\x16`\x04\x82\x01R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c\xE6\x04{\x19\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a=#W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` av\xB8\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a=7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a=[\x91\x90an\xE5V[a=\x83W`@Qc\xBC'\xA5\x17`\xE0\x1B\x81R`\x01`\x01`@\x1B\x03\x86\x16`\x04\x82\x01R`$\x01a\x18\xC2V[a=\x8D\x82BaS\xADV[`\0\x80`\x01`\x01`\xA0\x1B\x03\x83\x16c\xECshT\x88a=\xB0`\xA0\x8C\x01`\x80\x8D\x01as\xC3V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81R`\x01`\x01`@\x1B\x03\x90\x92\x16`\x04\x83\x01R\x15\x15`$\x82\x01R3`D\x82\x01R`d\x01`@\x80Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a>;W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` av\xB8\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a>OW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a>s\x91\x90as\xE3V[\x91P\x91P\x81a>\x95W`@Qc.\xD0\xEA\x01`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x87\x81\x1Cb\xFF\xFF\xFF\x16`\0\x90\x81R`\t\x82R`@\x90\x81\x90 \x81Q`\x80\x81\x01\x83R\x81T`\x01`\x01`\xA0\x1B\x03\x80\x82\x16\x83R`\xFF`\x01`\xA0\x1B\x92\x83\x90\x04\x81\x16\x96\x84\x01\x96\x90\x96R`\x01\x90\x93\x01T\x92\x83\x16\x93\x82\x01\x93\x90\x93R\x91\x90\x04\x90\x91\x16``\x82\x01Ra>\xFCa`gV[a?\x0C`\xA0\x8B\x01`\x80\x8C\x01as\xC3V[\x15a?OW` \x82\x01Q`\xFF\x90\x81\x16a\x01\0\x83\x01R``\x83\x01Q\x16a\x01 \x82\x01R\x81Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\xC0\x83\x01R`@\x83\x01Q\x16`\xE0\x82\x01Ra?\x89V[``\x82\x01Q`\xFF\x90\x81\x16a\x01\0\x83\x01R` \x83\x01Q\x16a\x01 \x82\x01R`@\x82\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\xC0\x83\x01R\x82Q\x16`\xE0\x82\x01R[a?\x99``\x8B\x01`@\x8C\x01as\xC3V[\x15a?\xCFW`\0a?\xAD\x82`\xC0\x01Qa\"PV[\x90P`\0\x81\x13\x15a?\xCDWa?\xC1\x81aR\xC3V[`\x01`\x01`\x80\x1B\x03\x16\x98P[P[\x82\x81R`\x80\x81\x01\x88\x90R`\xA0\x81\x01\x87\x90Ra?\xE9\x81aS\xDCV[\x90P\x80`\xA0\x01Q`\0\x03a@\x10W`@Qc7\xC2\xD9\xA7`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80`\x80\x01Q`\0\x03a@5W`@Qc\x13\xFD\x9Bm`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a@F6\x8C\x90\x03\x8C\x01\x8Cat\x14V[\x90Pa@U\x82`\x80\x01QaR\xC3V[a@b\x83`\xA0\x01QaR\xC3V[`\x01`\x01`\x80\x1B\x03\x90\x81\x16` \x84\x01R\x16\x81R`\x02\x87\x01T`\0\x90`\x01`\x01`\xA0\x1B\x03\x163\x14a@\xA1W`\x01\x88\x01T`\x01`\xA0\x1B\x90\x04a\xFF\xFF\x16a@\xB2V[`\x01\x88\x01T`\x01`\xB0\x1B\x90\x04a\xFF\xFF\x16[\x88T`\rTa\xFF\xFF\x92\x90\x92\x16\x92P`\0\x91\x82\x91a@\xE8\x91\x86\x91`\x01`\x01`\x80\x1B\x03\x80\x82\x16\x92`\x01`\x80\x1B\x90\x92\x04\x16\x90\x87\x90aTXV[\x88`@\x01\x89``\x01\x82\x96P\x83\x97P\x84\x81RP\x84\x81RPPPPP`\0\x89`\x01`\x01`\xA0\x1B\x03\x16c\xA4G\x89\x19\x8F\x88`\0\x01Q\x86\x86`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aA[\x94\x93\x92\x91\x90`\x01`\x01`@\x1B\x03\x94\x90\x94\x16\x84R` \x84\x01\x92\x90\x92R`@\x83\x01R``\x82\x01R`\x80\x01\x90V[`@\x80Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15aA\xB0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` av\xB8\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15aA\xC4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aA\xE8\x91\x90as\xE3V[` \x88\x01R\x90P\x80aB\x1DW\x85Q` \x87\x01Q`@Qc\\\x9E\xF7\x05`\xE1\x1B\x81R`\x04\x81\x01\x92\x90\x92R`$\x82\x01R`D\x01a\x18\xC2V[PaBX\x92PaB6\x91PP`\xA0\x8D\x01`\x80\x8E\x01as\xC3V[\x83``\x01Q\x84`\x80\x01QaBJ\x91\x90ao\x1BV[`\xA0\x85\x01Q\x8A\x92\x91\x90aT\x8CV[aBj\x82`\xC0\x01Q\x83`\x80\x01QaV#V[aB|\x82`\xE0\x01Q\x83`\xA0\x01QaS,V[``\x82\x01Q\x15aB\xBDW``\x82\x01Q`\xC0\x83\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x07` R`@\x81 \x80T\x90\x91\x90aB\xB7\x90\x84\x90ao.V[\x90\x91UPP[aB\xC6\x82aVjV[\x91P\x81`\xE0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x82`\xC0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x8B`\x01`\x01`@\x1B\x03\x16\x7F\xCD\x80T&o\xAE;\xBD#\0\xC8\x9A@t\xC4\x11=\x12\xB9\xA5\x83\xF7\xCD#\x90\xD40\xAFp\xB1*X\x8E`\x80\x01` \x81\x01\x90aC\"\x91\x90as\xC3V[\x8D\x8D\x88`@\x01Q\x89` \x01Q`@QaC_\x95\x94\x93\x92\x91\x90\x94\x15\x15\x85R` \x85\x01\x93\x90\x93R`@\x84\x01\x91\x90\x91R``\x83\x01R`\x80\x82\x01R`\xA0\x01\x90V[`@Q\x80\x91\x03\x90\xA4`\x0FT`\xFF\x16\x15\x15`\0\x03aC~WaC~aMpV[aC\x86aF\xB5V[PPPPPPP\x91\x93\x90\x92PV[``` `\0Rk\x0Bv1.5.0-beta`+R```\0\xF3[`\x0CT`\x01\x14\x15\x80\x15aC\xD3WP`\x0FT`\xFF\x16\x15\x80aC\xD3WP`\x02`\x0CT\x11[\x15aC\xF1W`@Qc\x02\xB8\0-`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x0C\x80T\x90`\0aD\x01\x83ar\x88V[\x91\x90PUPV[`\0\x80aDK\x86aD\x1AW`\0aD\x1DV[`\x01[`\xF8\x1B\x86aD,W`\0aD/V[`\x01[`\x0F`\xF8\x1B`\xF8\x91\x90\x91\x1B\x16`\x04\x91\x90\x91\x1B`\x0F`\xFC\x1B\x16\x17\x90V[`\xF8\x1C\x90P\x82\x84` \x1B\x82`8\x1B\x17\x17\x91PP\x94\x93PPPPV[`@\x80Qa\x01\0\x81\x01\x82R\x88T`\x01`\x01`\x80\x1B\x03\x80\x82\x16\x83R`\x01`\x80\x1B\x91\x82\x90\x04\x81\x16` \x84\x01R`\x01\x8B\x01T\x90\x81\x16\x93\x83\x01\x93\x90\x93R\x82\x04c\xFF\xFF\xFF\xFF\x16``\x82\x01\x81\x90R`\x01`\xA0\x1B\x83\x04a\xFF\xFF\x90\x81\x16`\x80\x84\x01R`\x01`\xB0\x1B\x90\x93\x04\x90\x92\x16`\xA0\x82\x01R`\x02\x89\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\xC0\x83\x01R`\x03\x8A\x01T\x16`\xE0\x90\x91\x01R\x15aE\x0FW`@Qc\xE90\xCE\xDF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[aE\x19\x87BaS\xADV[\x85`\0\x03aE:W`@Qc\x17O\xD4\x1B`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[aEC\x86aR\xC3V[\x87T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x91\x90\x91\x16\x17\x87U`\0\x85\x90\x03aE\x80W`@Qc(i\xC5\xF3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[aE\x89\x85aR\xC3V[\x87T`\x01`\x01`\x80\x1B\x03\x91\x82\x16`\x01`\x80\x1B\x02\x91\x16\x17\x87U`\x01\x84\x81\x14\x90\x85\x11\x17a\x03\xE8\x85\x81\x14\x90\x86\x10\x17\x16aE\xD5W`@Qc\x97\x1B1\t`\xE0\x1B\x81R`\x04\x81\x01\x85\x90R`$\x01a\x18\xC2V[aE\xDE\x84aV\xDDV[`\x01\x88\x01\x80Ta\xFF\xFF\x92\x90\x92\x16`\x01`\xA0\x1B\x02a\xFF\xFF`\xA0\x1B\x19\x90\x92\x16\x91\x90\x91\x17\x90U`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x80\x15\x90aF\x89W\x84\x84\x10\x85\x85\x14\x17`\x01\x80\x86\x11\x90\x86\x14\x17\x16aFDW`@Qc\xED\xDF\xE1\x19`\xE0\x1B\x81R`\x04\x81\x01\x85\x90R`$\x01a\x18\xC2V[`\x02\x88\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x85\x16\x17\x90UaFj\x84aV\xDDV[\x88`\x01\x01`\x16a\x01\0\n\x81T\x81a\xFF\xFF\x02\x19\x16\x90\x83a\xFF\xFF\x16\x02\x17\x90UP[P`\x03\x96\x90\x96\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x97\x16\x96\x90\x96\x17\x90\x95UPPPPPV[`\x0C\x80T\x90`\0aF\xC5\x83at3V[\x90\x91UPP`\x05T`\xFF\x16\x15\x80\x15aF\xE0WP`\x0FT`\xFF\x16\x15[\x15aF\xFEW`@Qc2n\xFAC`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[`\0\x80`\x01`\x01`\x7F\x1B\x03\x83`@\x01Q`\x01`\x01`\x80\x1B\x03\x16\x11\x15aG8W`@Qc\xAC\xC9@{`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[aGP\x83`@\x01QaGI\x90ap\xBDV[\x84\x90aI\xC4V[\x91P\x91P\x91P\x91V[`\0\x80aGe\x83aV\xEEV[\x90\x93\x04\x93\x92PPPV[4\x15aF\xFEWaG\xA0`\x02\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0aW\x06V[`@Q4\x81R3\x90\x7F\xE1\xFF\xFC\xC4\x92=\x04\xB5Y\xF4\xD2\x9A\x8B\xFCl\xDA\x04\xEB[\r<F\x07Q\xC2@,\\\\\xC9\x10\x9C\x90` \x01`@Q\x80\x91\x03\x90\xA2V[` \x83\x81\x1Cb\xFF\xFF\xFF\x16`\0\x90\x81R`\t\x82R`@\x80\x82 \x81Q`\x80\x81\x01\x83R\x81T`\x01`\x01`\xA0\x1B\x03\x80\x82\x16\x83R`\xFF`\x01`\xA0\x1B\x92\x83\x90\x04\x81\x16\x97\x84\x01\x97\x90\x97R`\x01\x90\x93\x01T\x92\x83\x16\x93\x82\x01\x93\x90\x93R\x91\x90\x04\x90\x92\x16``\x83\x01R\x90\x81\x90aHA\x85aR\xC3V[\x92P`\x01`\x01`\x80\x1B\x03\x85\x14aHqWaHna+\x86\x82` \x01Q`\xFF\x16\x87aS\x15\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x92P[aHz\x84aR\xC3V[\x91P`\x01`\x01`\x80\x1B\x03\x84\x14aH\xAAWaH\xA7a+\x86\x82``\x01Q`\xFF\x16\x86aS\x15\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x91P[P\x93P\x93\x91PPV[`\0\x82\x15\x80\x15aH\xCCWP\x83Q`\x01`\x01`\x80\x1B\x03\x16\x15\x15[\x15aH\xD9WP`\0a4<V[\x81\x15\x80\x15aH\xF3WP` \x84\x01Q`\x01`\x01`\x80\x1B\x03\x16\x15\x15[\x15aI\0WP`\0a4<V[`\0\x80`\0aI\x0E\x87aS\x7FV[aI%W\x86`@\x01Q`\x01`\x01`\x80\x1B\x03\x16aI/V[g\r\xE0\xB6\xB3\xA7d\0\0[\x87Q\x90\x91P`\x01`\x01`\x80\x1B\x03\x16\x15aI]W\x86QaIZ\x90\x87\x90\x83\x90`\x01`\x01`\x80\x1B\x03\x16aW\x9DV[\x92P[` \x87\x01Q`\x01`\x01`\x80\x1B\x03\x16\x15aI\x8EW` \x87\x01QaI\x8B\x90\x86\x90\x83\x90`\x01`\x01`\x80\x1B\x03\x16aW\x9DV[\x91P[aI\x9F\x82\x84\x11\x83\x85\x03\x02\x84\x03aR\xC3V[\x97\x96PPPPPPPV[`\0`\x01`\x01`\x7F\x1B\x03\x82\x11\x15aI\xC0W`\0\x80\xFD[P\x90V[`\0\x80`\x0F\x83\x90\x0B\x15aJ\xADW`\0\x80\x85`@\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90P`\0\x85`\x0F\x0B\x13\x15aJWWaI\xF9\x86aS\x7FV[\x15aJ\tWPg\r\xE0\xB6\xB3\xA7d\0\0[\x85Q`\x01`\x01`\x80\x1B\x03\x80\x87\x16\x93PaJ)\x91a+\x86\x91\x85\x91\x16\x84aW\xBCV[\x93PaJPa+\x86\x87` \x01Q`\x01`\x01`\x80\x1B\x03\x16\x83\x85aW\xBC\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x92PaJ\xAAV[aJ`\x85ap\xBDV[\x86Q`\x01`\x01`\x80\x1B\x03\x91\x82\x16\x93PaJ\x80\x91a+\x86\x91\x85\x91\x16\x84aW\x9DV[\x93PaJ\xA7a+\x86\x87` \x01Q`\x01`\x01`\x80\x1B\x03\x16\x83\x85aW\x9D\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x92P[PP[\x92P\x92\x90PV[`\x80\x81\x01Q`\x01`\x01`@\x1B\x03\x16`\0\x90\x81R`\n` \x90\x81R`@\x82 \x90\x83\x01Q\x90\x91\x90\x81\x90aJ\xE4\x90aR\xC3V[aJ\xF1\x85`@\x01QaR\xC3V[``\x86\x01Q`\x01\x86\x01T\x92\x94P\x90\x92P\x90`\x01`\x01`\x80\x1B\x03\x16`\0\x03aKPW`\0\x84Uc;\x9A\xCA\0`\x0F\x82\x90\x0B\x12\x15aK?W`@Qc\xCBm\xABu`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[aKMc;\x9A\xCA\0\x82atJV[\x90P[`\0\x81`\x0F\x0B\x13\x15aK\x90WaK\x8B\x85`\xA0\x01Q\x86`\x80\x01Q`\x01`\x01`@\x1B\x03\x16\x83`\x0F\x0B`@Q\x80` \x01`@R\x80`\0\x81RPaW\xEAV[aK\xB8V[aK\xB8\x85`\xA0\x01Q\x86`\x80\x01Q`\x01`\x01`@\x1B\x03\x16\x83`\x0F\x0BaK\xB3\x90at\x80V[aY\x92V[``\x85\x01Q`\x80\x86\x01Q`\x01`\x01`@\x1B\x03\x16`\0\x90\x81R`\n` R`@\x90 aK\xE2\x91aZ\x16V[`\xC0\x85\x01Q`\xE0\x86\x01Q``\x87\x01Q`\0`\x0F\x91\x90\x91\x0B\x12\x15aL\xB5WaL\x12\x82\x86`\x01`\x01`\x80\x1B\x03\x16aS,V[aL%\x81\x85`\x01`\x01`\x80\x1B\x03\x16aS,V[\x85T\x85\x90\x87\x90`\0\x90aLB\x90\x84\x90`\x01`\x01`\x80\x1B\x03\x16at\x9CV[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UP\x83\x86`\0\x01`\x10\x82\x82\x82\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\x80\x1B\x03\x16aL\x8C\x91\x90at\x9CV[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UPaMgV[aL\xC8\x82\x86`\x01`\x01`\x80\x1B\x03\x16aV#V[aL\xDB\x81\x85`\x01`\x01`\x80\x1B\x03\x16aV#V[\x85T\x85\x90\x87\x90`\0\x90aL\xF8\x90\x84\x90`\x01`\x01`\x80\x1B\x03\x16at\xC3V[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UP\x83\x86`\0\x01`\x10\x82\x82\x82\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\x80\x1B\x03\x16aMB\x91\x90at\xC3V[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UP[PPPPPPPV[`\0`\x02\x80\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15aM\xCAW` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11aM\xACW[PPPPP\x90P`\0\x81Q\x90P\x80`\0\x03aM\xE9Wa)T`\x02aZUV[\x80[`\0\x83aM\xF9`\x01\x84ao\x1BV[\x81Q\x81\x10aN\tWaN\tao\x05V[` \x02` \x01\x01Q\x90P`\0\x80aN,\x830`\x02aZ\x87\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x91P\x91P\x81`\0\x14\x15\x80aN?WP\x80\x15\x15[\x15aN\xCBW`\x0E`@Q\x80`\x80\x01`@R\x80\x84\x81R` \x01\x83\x81R` \x01aNg\x860aZ\xE9V[\x81R`\x01`\x01`\xA0\x1B\x03\x86\x81\x16` \x92\x83\x01R\x83T`\x01\x80\x82\x01\x86U`\0\x95\x86R\x94\x83\x90 \x84Q`\x04\x90\x92\x02\x01\x90\x81U\x91\x83\x01Q\x93\x82\x01\x93\x90\x93U`@\x82\x01Q`\x02\x82\x01U``\x90\x91\x01Q`\x03\x90\x91\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x91\x90\x92\x16\x17\x90U[`\x04\x80T\x80aN\xDCWaN\xDCat\xE3V[`\0\x82\x81R` \x81 \x82\x01`\0\x19\x90\x81\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90U\x91\x82\x01\x90\x92U\x85\x01\x94\x90\x03`\x01\x01\x92PaM\xEB\x91PPW`\0`\x0E\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15aO\x93W`\0\x84\x81R` \x90\x81\x90 `@\x80Q`\x80\x81\x01\x82R`\x04\x86\x02\x90\x92\x01\x80T\x83R`\x01\x80\x82\x01T\x84\x86\x01R`\x02\x82\x01T\x92\x84\x01\x92\x90\x92R`\x03\x01T`\x01`\x01`\xA0\x1B\x03\x16``\x83\x01R\x90\x83R\x90\x92\x01\x91\x01aO6V[PP\x82Q\x92\x93PPP[\x80\x15aQ\x8EW`\0aO\xB0`\x01\x83ao\x1BV[\x90P`\0\x83\x82\x81Q\x81\x10aO\xC6WaO\xC6ao\x05V[` \x02` \x01\x01Q``\x01Q\x90P`\0\x80\x85\x84\x81Q\x81\x10aO\xE9WaO\xE9ao\x05V[` \x02` \x01\x01Q`\0\x01Q\x86\x85\x81Q\x81\x10aP\x07WaP\x07ao\x05V[` \x02` \x01\x01Q` \x01Q\x91P\x91P`\0\x82\x11\x15aQ\x17W`\0\x86\x85\x81Q\x81\x10aP4WaP4ao\x05V[` \x02` \x01\x01Q`@\x01Q\x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x03aP\xABWaP\xA6\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x003\x85a[\xC9V[aP\xB6V[aP\xB6\x843\x85a\\pV[`\0aP\xC2\x850aZ\xE9V[\x90P`\0aP\xD0\x85\x84ao\x1BV[\x90P\x80\x82\x10\x15aQ\x0FW\x85aP\xE5\x82\x84at\xF9V[`@QcU\x13N\xF1`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x04\x83\x01R`$\x82\x01R`D\x01a\x18\xC2V[PPPaQ~V[\x80\x15aQ~W`\0\x86\x85\x81Q\x81\x10aQ1WaQ1ao\x05V[` \x02` \x01\x01Q`@\x01Q\x90PaQK\x8430\x85a\\\xCDV[`\0aQW\x850aZ\xE9V[\x90P`\0aQe\x84\x84ao.V[\x90P\x80\x82\x10\x15aQzW\x85aP\xE5\x82\x84at\xF9V[PPP[\x84`\x01\x90\x03\x94PPPPPaO\x9DV[aQ\x98`\x02aZUV[aQ\xA4`\x0E`\0a`\xD2V[PPPPPV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x81\x81R` \x85\x81R`@\x80\x83 T\x81Qc1<\xE5g`\xE0\x1B\x81R\x91Q\x93\x94\x90\x93\x85\x93aR\x81\x93\x86\x93\x92c1<\xE5g\x92`\x04\x80\x84\x01\x93\x91\x92\x91\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15aRAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` av\xB8\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15aRUW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aRy\x91\x90ar\xA1V[`\xFF\x16aR\xD5V[\x90P`\0aR\x8F\x86\x86aZ\xE9V[\x90P`\x01`\x01`\xFF\x1B\x03\x82\x11\x15aR\xA5W`\0\x80\xFD[`\x01`\x01`\xFF\x1B\x03\x81\x11\x15aR\xB9W`\0\x80\xFD[aI\x9F\x82\x82at\xF9V[`\0`\x01`\x80\x1B\x82\x10aI\xC0W`\0\x80\xFD[`\0\x82`\0\x03aR\xE7WP`\0a\x12]V[`\0aR\xF2\x83aV\xEEV[\x90P\x80aS\0`\x01\x86ao\x1BV[aS\n\x91\x90au\x19V[a8*\x90`\x01ao.V[`\0\x80aS!\x83aV\xEEV[\x93\x90\x93\x02\x93\x92PPPV[aS8`\x02\x83\x83a].V[\x81`\x01`\x01`\xA0\x1B\x03\x16\x7F\x1Cq\x1E\xCA\x8D\x0BiK\xBC\xB0\xA1Db\xA7\0b\"\xE7!\x95K,_\xF7\x98\xF6\x06\x81~\xB1\x102\x82`@QaSs\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2PPV[`\0aS\x94\x82``\x01Qc\xFF\xFF\xFF\xFF\x16\x15\x15\x90V[\x80\x15a\x12]WPP`@\x01Q`\x01`\x01`\x80\x1B\x03\x16\x15\x90V[aS\xB6\x81a]\xAFV[\x82`\x01\x01`\x10a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPPV[aS\xE4a`gV[a\x01\0\x82\x01Q`@\x83\x01QaS\xFB\x91`\xFF\x16aS\x15V[`@\x83\x01Ra\x01\0\x82\x01Q``\x83\x01QaT\x17\x91`\xFF\x16aS\x15V[``\x83\x01Ra\x01\0\x82\x01Q`\x80\x83\x01QaT3\x91`\xFF\x16aS\x15V[`\x80\x83\x01Ra\x01 \x82\x01Q`\xA0\x83\x01QaTO\x91`\xFF\x16aS\x15V[`\xA0\x83\x01RP\x90V[`\0\x80\x80\x80aTh\x89\x87\x87a]\xC2V[\x90\x94P\x92PaTz\x89\x89\x89\x87\x87a^\x08V[\x94\x9A\x93\x99P\x97P\x92\x95P\x90\x93PPPPV[\x82\x15aU.WaT\x9B\x82aR\xC3V[\x84T\x85\x90`\0\x90aT\xB6\x90\x84\x90`\x01`\x01`\x80\x1B\x03\x16at\xC3V[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UPaT\xE3\x81aR\xC3V[\x84T\x85\x90`\x10\x90aU\x05\x90\x84\x90`\x01`\x80\x1B\x90\x04`\x01`\x01`\x80\x1B\x03\x16at\x9CV[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UPaU\xC6V[aU7\x81aR\xC3V[\x84T\x85\x90`\0\x90aUR\x90\x84\x90`\x01`\x01`\x80\x1B\x03\x16at\x9CV[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UPaU\x7F\x82aR\xC3V[\x84T\x85\x90`\x10\x90aU\xA1\x90\x84\x90`\x01`\x80\x1B\x90\x04`\x01`\x01`\x80\x1B\x03\x16at\xC3V[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UP[\x83T`\x01`\x01`\x80\x1B\x03\x16`\0\x03aU\xF1W`@Qc\x17O\xD4\x1B`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x83T`\x01`\x80\x1B\x90\x04`\x01`\x01`\x80\x1B\x03\x16`\0\x03a-rW`@Qc(i\xC5\xF3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[aV/`\x02\x83\x83a_@V[\x81`\x01`\x01`\xA0\x1B\x03\x16\x7F\x80\xB2\x17H\xC7\x87\xC5.\x87\xA6\xB2\"\x01\x1E\n\x0E\xD0\xF9\xCC \x15\xF0\xCE\xD4gHd-\xC6.\xE9\xF8\x82`@QaSs\x91\x81R` \x01\x90V[aVra`gV[a\x01\0\x82\x01Q`@\x83\x01QaV\x89\x91`\xFF\x16aGYV[`@\x83\x01Ra\x01\0\x82\x01Q``\x83\x01QaV\xA5\x91`\xFF\x16aGYV[``\x83\x01Ra\x01\0\x82\x01Q`\x80\x83\x01QaV\xC1\x91`\xFF\x16aGYV[`\x80\x83\x01Ra\x01 \x82\x01Q`\xA0\x83\x01QaTO\x91`\xFF\x16aGYV[`\0b\x01\0\0\x82\x10aI\xC0W`\0\x80\xFD[`\0aV\xFB\x82`\x12ao\x1BV[a\x12]\x90`\nav\x1FV[aW\x10\x82\x82a_|V[\x80`\x01`\x01`\xA0\x1B\x03\x16c\xD0\xE3\r\xB04`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15aW\x89W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` av\xB8\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15aMgW=`\0\x80>=`\0\xFD[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16aW\xB5W`\0\x80\xFD[\x04\x92\x91PPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16aW\xD4W`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81R` \x81\x81R`@\x80\x83 \x86\x84R\x90\x91R\x81 \x80T\x84\x92\x90aX\x1B\x90\x84\x90ao.V[\x90\x91UPP`@\x80Q\x84\x81R` \x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x86\x16\x91`\0\x913\x91\x7F\xC3\xD5\x81h\xC5\xAEs\x97s\x1D\x06=[\xBF=exTBsC\xF4\xC0\x83$\x0Fz\xAC\xAA-\x0Fb\x91\x01`@Q\x80\x91\x03\x90\xA4`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15aYlW`@Qc\xF2:na`\xE0\x1B\x80\x82R\x90`\x01`\x01`\xA0\x1B\x03\x86\x16\x90c\xF2:na\x90aX\xB0\x903\x90`\0\x90\x89\x90\x89\x90\x89\x90`\x04\x01av+V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15aY\x08W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` av\xB8\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15aY\x1CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aY@\x91\x90apHV[`\x01`\x01`\xE0\x1B\x03\x19\x16\x14aYgW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x18\xC2\x90aphV[a-rV[`\x01`\x01`\xA0\x1B\x03\x84\x16a-rW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x18\xC2\x90ap\x92V[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R` \x81\x81R`@\x80\x83 \x85\x84R\x90\x91R\x81 \x80T\x83\x92\x90aY\xC3\x90\x84\x90ao\x1BV[\x90\x91UPP`@\x80Q\x83\x81R` \x81\x01\x83\x90R`\0\x91`\x01`\x01`\xA0\x1B\x03\x86\x16\x913\x91\x7F\xC3\xD5\x81h\xC5\xAEs\x97s\x1D\x06=[\xBF=exTBsC\xF4\xC0\x83$\x0Fz\xAC\xAA-\x0Fb\x91\x01`@Q\x80\x91\x03\x90\xA4PPPV[`\x01\x82\x01TaZ.\x90`\x01`\x01`\x80\x1B\x03\x16\x82a`\nV[`\x01\x92\x90\x92\x01\x80T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x90\x93\x16\x92\x90\x92\x17\x90\x91UPV[`\x02\x81\x01T\x15aZgWaZgaveV[`\x03\x81\x01\x80T`\xFF\x19\x16`\x01\x17\x90UaZ\x84`\x02\x82\x01`\0a`\xF3V[PV[`\0\x80\x80aZ\x96\x86\x86\x86aQ\xABV[\x90P`\0\x81\x13\x15aZ\xA9W\x80\x92PaZ\xBEV[`\0\x81\x12\x15aZ\xBEWaZ\xBB\x81at\x80V[\x91P[P`\x01`\x01`\xA0\x1B\x03\x90\x93\x16`\0\x90\x81R`\x01\x90\x94\x01` R`@\x90\x93 \x80T`\xFF\x19\x16\x90UP\x90\x91V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`$\x80\x84\x01\x91\x90\x91R\x83Q\x80\x84\x03\x90\x91\x01\x81R`D\x90\x92\x01\x83R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16cp\xA0\x821`\xE0\x1B\x17\x90R\x91Q`\0\x92\x83\x92\x83\x92\x91\x87\x16\x91a[C\x91\x90av{V[`\0`@Q\x80\x83\x03\x81\x85Z\xFA\x91PP=\x80`\0\x81\x14a[~W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a[\x83V[``\x91P[P\x91P\x91P\x81\x15\x80a[\x97WP\x80Q` \x14\x15[\x15a[\xB5W`@Qc\xC5.>\xFF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x80` \x01\x90Q\x81\x01\x90a\x14\"\x91\x90anDV[`@Qc.\x1A}M`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90c.\x1A}M\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\\IW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` av\xB8\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\\]W=`\0\x80>=`\0\xFD[PPPPa\\k\x82\x82a`;V[PPPV[`\0`@Qc\xA9\x05\x9C\xBB`\xE0\x1B`\0R\x83`\x04R\x82`$R` `\0`D`\0\x80\x89Z\xF1=\x15`\x1F=\x11`\x01`\0Q\x14\x16\x17\x16\x91P`\0``R\x80`@RP\x80a-rW`@Qc:\xCB=?`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0`@Qc#\xB8r\xDD`\xE0\x1B`\0R\x84`\x04R\x83`$R\x82`DR` `\0`d`\0\x80\x8AZ\xF1=\x15`\x1F=\x11`\x01`\0Q\x14\x16\x17\x16\x91P`\0``R\x80`@RP\x80aQ\xA4W`@Qcn\x89\xEC\xA5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R` \x84\x90R`@\x90 T\x80\x82\x11\x15a]rW`@Qc1Rv\xC9`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x81\x01\x83\x90R`D\x01a\x18\xC2V[a]|\x84\x84a_|V[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R` \x85\x90R`@\x81 \x80T\x84\x92\x90a]\xA4\x90\x84\x90ao\x1BV[\x90\x91UPPPPPPV[`\0d\x01\0\0\0\0\x82\x10aI\xC0W`\0\x80\xFD[\x82Q`\0\x90\x81\x90a]\xDF\x90`\x01`\x01`\x80\x1B\x03\x16\x85a'\x10aW\xBCV[\x91P\x82\x15a^\0Wa]\xF1\x83\x83au\x19V[\x90Pa]\xFD\x81\x83ao\x1BV[\x91P[\x93P\x93\x91PPV[`\0\x80`\0\x87`\x80\x01Qa^\x1CW\x85a^\x1EV[\x86[\x90P`\0\x88`\x80\x01Qa^1W\x87a^3V[\x86[\x89Q\x90\x91Pa^K\x90`\x01`\x01`\x80\x1B\x03\x16\x83ao.V[\x91Pa^W\x86\x83ao\x1BV[\x91P\x81\x85\x11\x15a^zW`@QcvG\x0F\xE7`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a^\x84\x85\x83ao\x1BV[\x91P\x80\x89` \x01Q`\x01`\x01`\x80\x1B\x03\x16\x11\x15a^\xB4W`@Qc\x86j\x03+`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x89\x01Qa^\xCC\x90`\x01`\x01`\x80\x1B\x03\x16\x82ao\x1BV[\x90P\x88`\x80\x01Qa^\xDDW\x80a^\xDFV[\x81[\x93P\x88`\x80\x01Qa^\xF0W\x81a^\xF2V[\x80[\x92P\x83\x88\x03a_\x14W`@Qc9;xE`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82\x87\x03a_4W`@Qc\x1F\xB0\xB7\xDD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PP\x95P\x95\x93PPPPV[a_J\x83\x83a_|V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R` \x84\x90R`@\x81 \x80T\x83\x92\x90a_r\x90\x84\x90ao.V[\x90\x91UPPPPPV[`\x03\x82\x01T`\xFF\x16\x15a_\x96W`\x03\x82\x01\x80T`\xFF\x19\x16\x90U[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x01\x83\x01` R`@\x90 T`\xFF\x16a)TW`\x02\x82\x01\x80T`\x01\x80\x82\x01\x83U`\0\x92\x83R` \x80\x84 \x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x95\x90\x95\x16\x94\x85\x17\x90U\x92\x82R\x92\x82\x01\x90\x92R`@\x90\x91 \x80T`\xFF\x19\x16\x90\x91\x17\x90UV[`\0\x80\x82`\x0F\x0B\x13\x15a`(Wa`!\x82\x84at\xC3V[\x90Pa\x12]V[a`1\x82ap\xBDV[a4<\x90\x84at\x9CV[`\0\x80`\0\x80`\0\x85\x87Z\xF1\x90P\x80a\\kW`@Qc5e\x94\xAB`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Q\x80a\x01@\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\0`\xFF\x16\x81R` \x01`\0`\xFF\x16\x81RP\x90V[P\x80T`\0\x82U`\x04\x02\x90`\0R` `\0 \x90\x81\x01\x90aZ\x84\x91\x90aa\x11V[P\x80T`\0\x82U\x90`\0R` `\0 \x90\x81\x01\x90aZ\x84\x91\x90aaEV[[\x80\x82\x11\x15aI\xC0W`\0\x80\x82U`\x01\x82\x01\x81\x90U`\x02\x82\x01U`\x03\x81\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90U`\x04\x01aa\x12V[[\x80\x82\x11\x15aI\xC0W`\0\x81U`\x01\x01aaFV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14aZ\x84W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15ab%Wab%aaZV[\x825ab0\x81aa\xFAV[\x94` \x93\x90\x93\x015\x93PPPV[`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14aZ\x84W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15abiWabiaaZV[\x815a4<\x81ab>V[`\0` \x82\x84\x03\x12\x15ab\x89Wab\x89aaZV[P5\x91\x90PV[`\0[\x83\x81\x10\x15ab\xABW\x81\x81\x01Q\x83\x82\x01R` \x01ab\x93V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Rab\xCC\x81` \x86\x01` \x86\x01ab\x90V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a4<` \x83\x01\x84ab\xB4V[`\x01`\x01`@\x1B\x03\x81\x16\x81\x14aZ\x84W`\0\x80\xFD[\x80\x15\x15\x81\x14aZ\x84W`\0\x80\xFD[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15ac/Wac/aaZV[\x845ac:\x81ab\xF3V[\x93P` \x85\x015acJ\x81ac\x08V[\x92P`@\x85\x015\x91P``\x85\x015aca\x81aa\xFAV[\x93\x96\x92\x95P\x90\x93PPV[\x805b\xFF\xFF\xFF\x81\x16\x81\x14ac\x7FW`\0\x80\xFD[\x91\x90PV[\x805a\xFF\xFF\x81\x16\x81\x14ac\x7FW`\0\x80\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x81\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x0ENL/$\r\x8C\xAD\xCC\xEE\x8D`\xAB\x1B`d\x82\x01R`\x84\x81\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rjrray stride`\xA8\x1B`d\x82\x01R`\x84\x81\xFD[`\0\x80\x83`\x1F\x84\x01\x12ad\xB6Wad\xB6ac\x96V[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15ad\xD0Wad\xD0ac\xEFV[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15aJ\xADWaJ\xADadHV[`\0\x80`\0\x80`\0\x80`\0\x80`\0a\x01\0\x8A\x8C\x03\x12\x15ae\rWae\raaZV[ae\x16\x8AaclV[\x98P` \x8A\x015\x97P`@\x8A\x015\x96Pae2``\x8B\x01ac\x84V[\x95Pae@`\x80\x8B\x01ac\x84V[\x94P`\xA0\x8A\x015aeP\x81aa\xFAV[\x93P`\xC0\x8A\x015ae`\x81aa\xFAV[\x92P`\xE0\x8A\x015`\x01`\x01`@\x1B\x03\x81\x11\x15ae~Wae~aa\xAAV[ae\x8A\x8C\x82\x8D\x01ad\xA1V[\x91P\x80\x93PP\x80\x91PP\x92\x95\x98P\x92\x95\x98P\x92\x95\x98V[`\0` \x82\x84\x03\x12\x15ae\xB6Wae\xB6aaZV[\x815a4<\x81ab\xF3V[`\0\x80\x83`\x1F\x84\x01\x12ae\xD6Wae\xD6ac\x96V[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15ae\xF0Wae\xF0ac\xEFV[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15aJ\xADWaJ\xADadHV[`\0\x80`\0\x80`\0\x80`\0\x80`\xA0\x89\x8B\x03\x12\x15af-Waf-aaZV[\x885af8\x81aa\xFAV[\x97P` \x89\x015afH\x81aa\xFAV[\x96P`@\x89\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15afgWafgaa\xAAV[afs\x8C\x83\x8D\x01ae\xC1V[\x90\x98P\x96P``\x8B\x015\x91P\x80\x82\x11\x15af\x8FWaf\x8Faa\xAAV[af\x9B\x8C\x83\x8D\x01ae\xC1V[\x90\x96P\x94P`\x80\x8B\x015\x91P\x80\x82\x11\x15af\xB7Waf\xB7aa\xAAV[Paf\xC4\x8B\x82\x8C\x01ad\xA1V[\x99\x9C\x98\x9BP\x96\x99P\x94\x97\x93\x96\x92\x95\x94PPPV[`\x01`\x01`\x80\x1B\x03\x81\x16\x81\x14aZ\x84W`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15ag\tWag\taaZV[\x865ag\x14\x81ac\x08V[\x95P` \x87\x015ag$\x81aa\xFAV[\x94P`@\x87\x015ag4\x81ab\xF3V[\x93P``\x87\x015agD\x81af\xD8V[\x92P`\x80\x87\x015agT\x81af\xD8V[\x91P`\xA0\x87\x015agd\x81af\xD8V[\x80\x91PP\x92\x95P\x92\x95P\x92\x95V[`\0\x80`@\x83\x85\x03\x12\x15ag\x88Wag\x88aaZV[\x825ag\x93\x81aa\xFAV[\x91P` \x83\x015ag\xA3\x81aa\xFAV[\x80\x91PP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15ag\xC3Wag\xC3aaZV[\x815a4<\x81aa\xFAV[`\0\x80`\0\x80`@\x85\x87\x03\x12\x15ag\xE7Wag\xE7aaZV[\x845`\x01`\x01`@\x1B\x03\x80\x82\x11\x15ah\x01Wah\x01aa\xAAV[ah\r\x88\x83\x89\x01ae\xC1V[\x90\x96P\x94P` \x87\x015\x91P\x80\x82\x11\x15ah)Wah)aa\xAAV[Pah6\x87\x82\x88\x01ae\xC1V[\x95\x98\x94\x97P\x95PPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15ahzW\x83Q\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01ah^V[P\x90\x96\x95PPPPPPV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15ah\xA1Wah\xA1aaZV[\x855ah\xAC\x81ac\x08V[\x94P` \x86\x015ah\xBC\x81ab\xF3V[\x93P`@\x86\x015ah\xCC\x81af\xD8V[\x92P``\x86\x015ah\xDC\x81af\xD8V[\x91P`\x80\x86\x015ah\xEC\x81af\xD8V[\x80\x91PP\x92\x95P\x92\x95\x90\x93PV[`\0` \x82\x84\x03\x12\x15ai\x0FWai\x0FaaZV[a4<\x82aclV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\xA0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aiPWaiPai\x18V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15ai~Wai~ai\x18V[`@R\x91\x90PV[`\0`\xA0\x82\x84\x03\x12\x15ai\xE4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FABI decoding: struct data too sh`D\x82\x01Rb\x1B\xDC\x9D`\xEA\x1B`d\x82\x01R`\x84\x81\xFD[ai\xECai.V[\x90P\x815ai\xF9\x81af\xD8V[\x81R` \x82\x015aj\t\x81af\xD8V[` \x82\x01R`@\x82\x015aj\x1C\x81ac\x08V[`@\x82\x01R``\x82\x015aj/\x81ab\xF3V[``\x82\x01R`\x80\x82\x015ajB\x81ac\x08V[`\x80\x82\x01R\x92\x91PPV[`\0\x80`\0`\xE0\x84\x86\x03\x12\x15ajeWajeaaZV[ajo\x85\x85ai\x86V[\x92P`\xA0\x84\x015\x91P`\xC0\x84\x015aj\x86\x81aa\xFAV[\x80\x91PP\x92P\x92P\x92V[`\0\x80`@\x83\x85\x03\x12\x15aj\xA7Waj\xA7aaZV[\x825aj\xB2\x81ab\xF3V[\x91P` \x83\x015`\x0F\x81\x90\x0B\x81\x14ag\xA3W`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15aj\xE1Waj\xE1aaZV[\x835aj\xEC\x81ab\xF3V[\x92Paj\xFA` \x85\x01ac\x84V[\x91Pak\x08`@\x85\x01ac\x84V[\x90P\x92P\x92P\x92V[`\0\x80`@\x83\x85\x03\x12\x15ak'Wak'aaZV[\x825ak2\x81aa\xFAV[\x91P` \x83\x015ag\xA3\x81ac\x08V[`\0\x80` \x83\x85\x03\x12\x15akXWakXaaZV[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15akqWakqaa\xAAV[ak}\x85\x82\x86\x01ae\xC1V[\x90\x96\x90\x95P\x93PPPPV[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x86\x01\x91P`@\x81`\x05\x1B\x87\x01\x01\x92P\x83\x87\x01`\0[\x82\x81\x10\x15ak\xDEW`?\x19\x88\x86\x03\x01\x84Rak\xCC\x85\x83Qab\xB4V[\x94P\x92\x85\x01\x92\x90\x85\x01\x90`\x01\x01ak\xB0V[P\x92\x97\x96PPPPPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15al\x03Wal\x03aaZV[\x835al\x0E\x81ab\xF3V[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15al;Wal;aaZV[\x835alF\x81ab\xF3V[\x92P` \x84\x015alV\x81ac\x08V[\x91P`@\x84\x015aj\x86\x81aa\xFAV[`\x01`\x01`\x80\x1B\x03\x80\x82Q\x16\x83R\x80` \x83\x01Q\x16` \x84\x01RP`@\x81\x01Q\x15\x15`@\x83\x01R`\x01`\x01`@\x1B\x03``\x82\x01Q\x16``\x83\x01R`\x80\x81\x01Q\x15\x15`\x80\x83\x01RPPV[`\xA0\x81\x01a\x12]\x82\x84alfV[`\0\x80`\0\x80`\0\x80`\xA0\x87\x89\x03\x12\x15al\xDAWal\xDAaaZV[\x865al\xE5\x81aa\xFAV[\x95P` \x87\x015al\xF5\x81aa\xFAV[\x94P`@\x87\x015\x93P``\x87\x015\x92P`\x80\x87\x015`\x01`\x01`@\x1B\x03\x81\x11\x15am!Wam!aa\xAAV[am-\x89\x82\x8A\x01ad\xA1V[\x97\x9A\x96\x99P\x94\x97P\x92\x95\x93\x94\x92PPPV[`\0`\xA0\x82\x84\x03\x12\x15amTWamTaaZV[P\x91\x90PV[`\0` \x80\x83\x85\x03\x12\x15ampWampaaZV[\x82Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15am\x8AWam\x8Aaa\xAAV[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12am\xA1Wam\xA1ac\x96V[\x81Q\x81\x81\x11\x15am\xB3Wam\xB3ai\x18V[am\xC5`\x1F\x82\x01`\x1F\x19\x16\x85\x01aiVV[\x91P\x80\x82R\x86\x84\x82\x85\x01\x01\x11\x15an+W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x85\x90R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R`\x84\x81\xFD[an:\x81\x85\x84\x01\x86\x86\x01ab\x90V[P\x95\x94PPPPPV[`\0` \x82\x84\x03\x12\x15anYWanYaaZV[PQ\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x81\x03an\x8FWan\x8Fan`V[`\x01\x01\x93\x92PPPV[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[`\x01`\x01`@\x1B\x03\x84\x16\x81R`@` \x82\x01R`\0a\x14\"`@\x83\x01\x84\x86an\x99V[`\0` \x82\x84\x03\x12\x15an\xFAWan\xFAaaZV[\x81Qa4<\x81ac\x08V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x12]Wa\x12]an`V[\x80\x82\x01\x80\x82\x11\x15a\x12]Wa\x12]an`V[\x81\x83R`\0`\x01`\x01`\xFB\x1B\x03\x83\x11\x15ao\xA4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FABI encoding: array data too lon`D\x82\x01R`g`\xF8\x1B`d\x82\x01R`\x84\x81\xFD[\x82`\x05\x1B\x80\x83` \x87\x017\x93\x90\x93\x01` \x01\x93\x92PPPV[`@\x81R`\0ao\xD1`@\x83\x01\x86\x88aoAV[\x82\x81\x03` \x84\x01RaI\x9F\x81\x85\x87aoAV[`\x01`\x01`\xA0\x1B\x03\x89\x81\x16\x82R\x88\x16` \x82\x01R`\xA0`@\x82\x01\x81\x90R`\0\x90ap\x11\x90\x83\x01\x88\x8AaoAV[\x82\x81\x03``\x84\x01Rap$\x81\x87\x89aoAV[\x90P\x82\x81\x03`\x80\x84\x01Rap9\x81\x85\x87an\x99V[\x9B\x9APPPPPPPPPPPV[`\0` \x82\x84\x03\x12\x15ap]Wap]aaZV[\x81Qa4<\x81ab>V[` \x80\x82R`\x10\x90\x82\x01Ro\x15S\x94\xD0Q\x91W\xD4\x91P\xD2T\x12QS\x95`\x82\x1B`@\x82\x01R``\x01\x90V[` \x80\x82R`\x11\x90\x82\x01Rp\x12S\x95\x90S\x12Q\x17\xD4\x91P\xD2T\x12QS\x95`z\x1B`@\x82\x01R``\x01\x90V[`\0\x81`\x0F\x0B`\x01`\x01`\x7F\x1B\x03\x19\x81\x03ap\xDAWap\xDAan`V[`\0\x03\x92\x91PPV[`\0` \x82\x84\x03\x12\x15ap\xF8Wap\xF8aaZV[\x81Qa4<\x81aa\xFAV[`\xE0\x81\x01aq\x11\x82\x86alfV[`\xA0\x82\x01\x93\x90\x93R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16`\xC0\x90\x91\x01R\x91\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15aqHWaqHaaZV[\x83QaqS\x81ac\x08V[` \x85\x01Q`@\x90\x95\x01Q\x90\x96\x94\x95P\x93\x92PPPV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FCalldata tail too short\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x81\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12ar\x06W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FInvalid calldata tail offset\0\0\0\0`D\x82\x01R`d\x81\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15ar`W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FInvalid calldata tail length\0\0\0\0`D\x82\x01R`d\x81\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15aJ\xADWaJ\xADaqjV[\x81\x83\x827`\0\x91\x01\x90\x81R\x91\x90PV[`\0`\x01\x82\x01ar\x9AWar\x9Aan`V[P`\x01\x01\x90V[`\0` \x82\x84\x03\x12\x15ar\xB6War\xB6aaZV[\x81Q`\xFF\x81\x16\x81\x14a4<W`\0\x80\xFD[`\0b\xFF\xFF\xFF\x80\x83\x16\x81\x81\x03an\x8FWan\x8Fan`V[`\0`\xA0\x82\x84\x03\x12\x15ar\xF4War\xF4aaZV[ar\xFCai.V[\x82Qas\x07\x81af\xD8V[\x81R` \x83\x01Qas\x17\x81af\xD8V[` \x82\x01R`@\x83\x01Qas*\x81ac\x08V[`@\x82\x01R``\x83\x01Qas=\x81ab\xF3V[``\x82\x01R`\x80\x83\x01QasP\x81ac\x08V[`\x80\x82\x01R\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x87\x81\x16\x82R\x86\x16` \x82\x01R`@\x81\x01\x85\x90R``\x81\x01\x84\x90R`\xA0`\x80\x82\x01\x81\x90R`\0\x90as\x97\x90\x83\x01\x84\x86an\x99V[\x98\x97PPPPPPPPV[`\0` \x82\x84\x03\x12\x15as\xB8Was\xB8aaZV[\x815a4<\x81af\xD8V[`\0` \x82\x84\x03\x12\x15as\xD8Was\xD8aaZV[\x815a4<\x81ac\x08V[`\0\x80`@\x83\x85\x03\x12\x15as\xF9Was\xF9aaZV[\x82Qat\x04\x81ac\x08V[` \x93\x90\x93\x01Q\x92\x94\x92\x93PPPV[`\0`\xA0\x82\x84\x03\x12\x15at)Wat)aaZV[a4<\x83\x83ai\x86V[`\0\x81atBWatBan`V[P`\0\x19\x01\x90V[`\x0F\x82\x81\x0B\x90\x82\x90\x0B\x03o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x12`\x01`\x01`\x7F\x1B\x03\x82\x13\x17\x15a\x12]Wa\x12]an`V[`\0`\x01`\xFF\x1B\x82\x01at\x95Wat\x95an`V[P`\0\x03\x90V[`\x01`\x01`\x80\x1B\x03\x82\x81\x16\x82\x82\x16\x03\x90\x80\x82\x11\x15at\xBCWat\xBCan`V[P\x92\x91PPV[`\x01`\x01`\x80\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x80\x82\x11\x15at\xBCWat\xBCan`V[cNH{q`\xE0\x1B`\0R`1`\x04R`$`\0\xFD[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15at\xBCWat\xBCan`V[`\0\x82au6WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[`\x01\x81\x81[\x80\x85\x11\x15auvW\x81`\0\x19\x04\x82\x11\x15au\\Wau\\an`V[\x80\x85\x16\x15auiW\x91\x81\x02\x91[\x93\x84\x1C\x93\x90\x80\x02\x90au@V[P\x92P\x92\x90PV[`\0\x82au\x8DWP`\x01a\x12]V[\x81au\x9AWP`\0a\x12]V[\x81`\x01\x81\x14au\xB0W`\x02\x81\x14au\xBAWau\xD6V[`\x01\x91PPa\x12]V[`\xFF\x84\x11\x15au\xCBWau\xCBan`V[PP`\x01\x82\x1Ba\x12]V[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15au\xF9WP\x81\x81\na\x12]V[av\x03\x83\x83au;V[\x80`\0\x19\x04\x82\x11\x15av\x17Wav\x17an`V[\x02\x93\x92PPPV[`\0a4<\x83\x83au~V[`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x82R\x85\x16` \x82\x01R`@\x81\x01\x84\x90R``\x81\x01\x83\x90R`\xA0`\x80\x82\x01\x81\x90R`\0\x90aI\x9F\x90\x83\x01\x84ab\xB4V[cNH{q`\xE0\x1B`\0R`\x01`\x04R`$`\0\xFD[`\0\x82Qav\x8D\x81\x84` \x87\x01ab\x90V[\x91\x90\x91\x01\x92\x91PPV\xFEEther sent to non-payable functiTarget contract does not contain";
    /// The bytecode of the contract.
    pub static PORTFOLIO_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\x023W`\x005`\xE0\x1C\x80c\x89\x92\xF2\n\x11a\x01.W\x80c\xC9\xC6S\x96\x11a\0\xABW\x80c\xE9\x85\xE9\xC5\x11a\0oW\x80c\xE9\x85\xE9\xC5\x14a\x10PW\x80c\xF0{\x87\x9E\x14a\x10\xC6W\x80c\xF2BC*\x14a\x11.W\x80c\xF3:\xE1\xBC\x14a\x11\x89W\x80c\xFF\xA1\xADt\x14a\x11\xC1Wa\x02oV[\x80c\xC9\xC6S\x96\x14a\x0E\xACW\x80c\xD6\xB7\xDE\xC5\x14a\x0E\xBFW\x80c\xDC\xF8D\xA7\x14a\x0F2W\x80c\xDD\xA4\x07\x97\x14a\x0F\x9AW\x80c\xE31\xBA4\x14a\x0F\xF5Wa\x02oV[\x80c\xAC\x96P\xD8\x11a\0\xF2W\x80c\xAC\x96P\xD8\x14a\x0C\xECW\x80c\xAD\\FH\x14a\r\x0CW\x80c\xB0\xC3\xA9P\x14a\r{W\x80c\xB0\xE2\x1E\x8A\x14a\r\xEAW\x80c\xC9\xA3\x96\xE9\x14a\x0E;Wa\x02oV[\x80c\x89\x92\xF2\n\x14a\n\x11W\x80c\x89\xA5\xF0\x84\x14a\n\x8CW\x80c\x8Ag\x89g\x14a\x0B\xB3W\x80c\xA2,\xB4e\x14a\x0C\x0EW\x80c\xA5\xCD\x8AI\x14a\x0CiWa\x02oV[\x80c/\x9E8\xE2\x11a\x01\xBCW\x80cN\x12s\xF4\x11a\x01\x80W\x80cN\x12s\xF4\x14a\x07\xFDW\x80c[\xC5Td\x14a\x08eW\x80c^Gf<\x14a\x08xW\x80cx}\xCE=\x14a\t>W\x80c\x80\xAF\x9Dv\x14a\t\x99Wa\x02oV[\x80c/\x9E8\xE2\x14a\x06aW\x80c0$K\xE7\x14a\x06tW\x80c9CMZ\x14a\x06\xCFW\x80c?\x92\xA39\x14a\x07*W\x80cM\xC6\x8A\x90\x14a\x07\xA2Wa\x02oV[\x80c\x0E\x894\x1C\x11a\x02\x03W\x80c\x0E\x894\x1C\x14a\x04\xA8W\x80c\x19\x05x\x07\x14a\x05\x10W\x80c&z\x0C\xFE\x14a\x05kW\x80c*\xFB\x9D\xF8\x14a\x05\x96W\x80c.\xB2\xC2\xD6\x14a\x06\x06Wa\x02oV[\x80b\xFD\xD5\x8E\x14a\x02\xC8W\x80c\x01\xFF\xC9\xA7\x14a\x03KW\x80c\x06C;\x1B\x14a\x03\xB6W\x80c\x07\x88\x88\xD6\x14a\x04=Wa\x02oV[6a\x02oW3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x02mW`\0\x80\xFD[\0[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`)`$\x82\x01R\x7FUnknown signature and no fallbac`D\x82\x01\x90\x81Rh\x1A\xC8\x19\x19Y\x9A[\x99Y`\xBA\x1B`d\x83\x01R`\x84\x82\xFD[4\x80\x15a\x03\x0FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R`\0\x80Q` av\x98\x839\x81Q\x91R`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[Pa\x038a\x03\x1E6`\x04ab\x0FV[`\0` \x81\x81R\x92\x81R`@\x80\x82 \x90\x93R\x90\x81R T\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03\x92W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R`\0\x80Q` av\x98\x839\x81Q\x91R`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[Pa\x03\xA6a\x03\xA16`\x04abTV[a\x12\x11V[`@Q\x90\x15\x15\x81R` \x01a\x03BV[4\x80\x15a\x03\xFDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R`\0\x80Q` av\x98\x839\x81Q\x91R`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[Pa\x04%\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x03BV[4\x80\x15a\x04\x84W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R`\0\x80Q` av\x98\x839\x81Q\x91R`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x06Ta\x04\x94\x90b\xFF\xFF\xFF\x16\x81V[`@Qb\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x03BV[4\x80\x15a\x04\xEFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R`\0\x80Q` av\x98\x839\x81Q\x91R`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[Pa\x05\x03a\x04\xFE6`\x04abtV[a\x12cV[`@Qa\x03B\x91\x90ab\xE0V[4\x80\x15a\x05WW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R`\0\x80Q` av\x98\x839\x81Q\x91R`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[Pa\x038a\x05f6`\x04ac\x16V[a\x13@V[a\x05~a\x05y6`\x04ad\xEBV[a\x14+V[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01a\x03BV[4\x80\x15a\x05\xDDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R`\0\x80Q` av\x98\x839\x81Q\x91R`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[Pa\x05\xF1a\x05\xEC6`\x04ae\xA1V[a\x17.V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x03BV[4\x80\x15a\x06MW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R`\0\x80Q` av\x98\x839\x81Q\x91R`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[Pa\x02ma\x06\\6`\x04af\x0EV[a\x18\x85V[a\x05\xF1a\x06o6`\x04af\xEDV[a\x1B\x93V[4\x80\x15a\x06\xBBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R`\0\x80Q` av\x98\x839\x81Q\x91R`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[Pa\x04%a\x06\xCA6`\x04ae\xA1V[a!]V[4\x80\x15a\x07\x16W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R`\0\x80Q` av\x98\x839\x81Q\x91R`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[Pa\x038a\x07%6`\x04ae\xA1V[a!\x84V[4\x80\x15a\x07qW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R`\0\x80Q` av\x98\x839\x81Q\x91R`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[Pa\x04\x94a\x07\x806`\x04agrV[`\x0B` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 Tb\xFF\xFF\xFF\x16\x81V[4\x80\x15a\x07\xE9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R`\0\x80Q` av\x98\x839\x81Q\x91R`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[Pa\x038a\x07\xF86`\x04ag\xAEV[a\"PV[4\x80\x15a\x08DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R`\0\x80Q` av\x98\x839\x81Q\x91R`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[Pa\x08Xa\x08S6`\x04ag\xCEV[a\"^V[`@Qa\x03B\x91\x90ahBV[a\x05\xF1a\x08s6`\x04ah\x86V[a#\x92V[4\x80\x15a\x08\xBFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R`\0\x80Q` av\x98\x839\x81Q\x91R`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[Pa\t\na\x08\xCE6`\x04ah\xFAV[`\t` R`\0\x90\x81R`@\x90 \x80T`\x01\x90\x91\x01T`\x01`\x01`\xA0\x1B\x03\x80\x83\x16\x92`\xFF`\x01`\xA0\x1B\x91\x82\x90\x04\x81\x16\x93\x92\x83\x16\x92\x91\x90\x91\x04\x16\x84V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x95\x86\x16\x81R`\xFF\x94\x85\x16` \x82\x01R\x94\x90\x92\x16\x91\x84\x01\x91\x90\x91R\x16``\x82\x01R`\x80\x01a\x03BV[4\x80\x15a\t\x85W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R`\0\x80Q` av\x98\x839\x81Q\x91R`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[Pa\x02ma\t\x946`\x04abtV[a'\xD0V[4\x80\x15a\t\xE0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R`\0\x80Q` av\x98\x839\x81Q\x91R`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[Pa\t\xF4a\t\xEF6`\x04ajMV[a)XV[`@\x80Q\x93\x15\x15\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x03BV[4\x80\x15a\nXW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R`\0\x80Q` av\x98\x839\x81Q\x91R`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[Pa\nla\ng6`\x04aj\x91V[a*6V[`@\x80Q`\x01`\x01`\x80\x1B\x03\x93\x84\x16\x81R\x92\x90\x91\x16` \x83\x01R\x01a\x03BV[4\x80\x15a\n\xD3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R`\0\x80Q` av\x98\x839\x81Q\x91R`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[Pa\x0BOa\n\xE26`\x04ae\xA1V[`\n` R`\0\x90\x81R`@\x90 \x80T`\x01\x82\x01T`\x02\x83\x01T`\x03\x90\x93\x01T`\x01`\x01`\x80\x1B\x03\x80\x84\x16\x94`\x01`\x80\x1B\x94\x85\x90\x04\x82\x16\x94\x91\x84\x16\x93\x91\x82\x04c\xFF\xFF\xFF\xFF\x16\x92`\x01`\xA0\x1B\x83\x04a\xFF\xFF\x90\x81\x16\x93`\x01`\xB0\x1B\x90\x04\x16\x91`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x91\x16\x88V[`@\x80Q`\x01`\x01`\x80\x1B\x03\x99\x8A\x16\x81R\x97\x89\x16` \x89\x01R\x97\x90\x95\x16\x96\x86\x01\x96\x90\x96Rc\xFF\xFF\xFF\xFF\x90\x92\x16``\x85\x01Ra\xFF\xFF\x90\x81\x16`\x80\x85\x01R\x16`\xA0\x83\x01R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\xC0\x83\x01R\x91\x90\x91\x16`\xE0\x82\x01Ra\x01\0\x01a\x03BV[4\x80\x15a\x0B\xFAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R`\0\x80Q` av\x98\x839\x81Q\x91R`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[Pa\x02ma\x0C\t6`\x04aj\xC9V[a+\xFAV[4\x80\x15a\x0CUW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R`\0\x80Q` av\x98\x839\x81Q\x91R`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[Pa\x02ma\x0Cd6`\x04ak\x11V[a-xV[4\x80\x15a\x0C\xB0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R`\0\x80Q` av\x98\x839\x81Q\x91R`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[Pa\x0C\xD7a\x0C\xBF6`\x04ah\xFAV[`\x08` R`\0\x90\x81R`@\x90 Tc\xFF\xFF\xFF\xFF\x16\x81V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x03BV[a\x0C\xFFa\x0C\xFA6`\x04akBV[a-\xE4V[`@Qa\x03B\x91\x90ak\x89V[4\x80\x15a\rSW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R`\0\x80Q` av\x98\x839\x81Q\x91R`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[Pa\x04%\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\r\xC2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R`\0\x80Q` av\x98\x839\x81Q\x91R`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[Pa\x04%\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x0E1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R`\0\x80Q` av\x98\x839\x81Q\x91R`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[Pa\x038`\rT\x81V[4\x80\x15a\x0E\x82W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R`\0\x80Q` av\x98\x839\x81Q\x91R`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[Pa\x038a\x0E\x916`\x04ag\xAEV[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x02` R`@\x90 T\x90V[a\x04\x94a\x0E\xBA6`\x04agrV[a/WV[4\x80\x15a\x0F\x06W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R`\0\x80Q` av\x98\x839\x81Q\x91R`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[Pa\x0F\x1Aa\x0F\x156`\x04ak\xEBV[a3\x02V[`@Q`\x01`\x01`\x80\x1B\x03\x90\x91\x16\x81R` \x01a\x03BV[4\x80\x15a\x0FyW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R`\0\x80Q` av\x98\x839\x81Q\x91R`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[Pa\x038a\x0F\x886`\x04ag\xAEV[`\x07` R`\0\x90\x81R`@\x90 T\x81V[4\x80\x15a\x0F\xE1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R`\0\x80Q` av\x98\x839\x81Q\x91R`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[Pa\x02ma\x0F\xF06`\x04ab\x0FV[a4CV[4\x80\x15a\x10<W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R`\0\x80Q` av\x98\x839\x81Q\x91R`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[Pa\x038a\x10K6`\x04ae\xA1V[a6\xE4V[4\x80\x15a\x10\x97W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R`\0\x80Q` av\x98\x839\x81Q\x91R`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[Pa\x03\xA6a\x10\xA66`\x04agrV[`\x01` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T`\xFF\x16\x81V[4\x80\x15a\x11\rW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R`\0\x80Q` av\x98\x839\x81Q\x91R`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[Pa\x11!a\x11\x1C6`\x04al#V[a7&V[`@Qa\x03B\x91\x90al\xB0V[4\x80\x15a\x11uW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R`\0\x80Q` av\x98\x839\x81Q\x91R`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[Pa\x02ma\x11\x846`\x04al\xBEV[a82V[a\x11\x9Ca\x11\x976`\x04am?V[a:\x92V[`@\x80Q`\x01`\x01`@\x1B\x03\x90\x94\x16\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x03BV[4\x80\x15a\x12\x08W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R`\0\x80Q` av\x98\x839\x81Q\x91R`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[Pa\x05\x03aC\x94V[`\0c\x01\xFF\xC9\xA7`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x14\x80a\x12BWPcl\xDB=\x13`\xE1\x1B`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x14[\x80a\x12]WPc\x03\xA2M\x07`\xE2\x1B`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x14[\x92\x91PPV[`@Qc\x03\xA2M\x07`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R``\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\x0E\x894\x1C\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x13\x04W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` av\xB8\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x13\x18W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x12]\x91\x90\x81\x01\x90amZV[`\0a\x13K\x85a!]V[`@Qc\x19\x05x\x07`\xE0\x1B\x81R`\x01`\x01`@\x1B\x03\x87\x16`\x04\x82\x01R\x85\x15\x15`$\x82\x01R`D\x81\x01\x85\x90R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`d\x83\x01R\x91\x90\x91\x16\x90c\x19\x05x\x07\x90`\x84\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x13\xEAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` av\xB8\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x13\xFEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14\"\x91\x90anDV[\x95\x94PPPPPV[`\0a\x145aC\xB1V[`\0b\xFF\xFF\xFF\x8B\x16\x15a\x14HW\x8Aa\x14QV[`\x06Tb\xFF\xFF\xFF\x16[\x90P\x80b\xFF\xFF\xFF\x16`\0\x03a\x14yW`@Qc\xCCzs\x9B`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[b\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\x08` R`@\x81 \x80T\x82\x90a\x14\xA0\x90c\xFF\xFF\xFF\xFF\x16anvV[\x91\x90a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90U\x90Pa\x14\xE1`\0\x80`\x01`\x01`\xA0\x1B\x03\x16\x89`\x01`\x01`\xA0\x1B\x03\x16\x14\x15\x84\x84aD\x08V[`\x01`\x01`@\x1B\x03\x81\x16`\0\x90\x81R`\n` R`@\x90 \x90\x93Pa\x15\x12\x90\x8C\x8Ca\xFF\xFF\x80\x8E\x16\x90\x8D\x16\x8C\x8CaDfV[`\x0F\x80Th\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x19\x16a\x01\0`\x01`\x01`@\x1B\x03\x86\x16\x02\x17\x90U`\0a\x15>\x84a!]V[`\x01`\x01`\xA0\x1B\x03\x16c\xE0hx\x7F\x85\x88\x88`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x15m\x93\x92\x91\x90an\xC2V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x15\xC5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` av\xB8\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x15\xD9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\xFD\x91\x90an\xE5V[\x90P\x80a\x16\x1DW`@Qc\x1B\xE2\xB1K`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\t`\0\x84b\xFF\xFF\xFF\x16b\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\x01\x01`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16`\t`\0\x85b\xFF\xFF\xFF\x16b\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x01`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x85`\x01`\x01`@\x1B\x03\x16\x7Fa<|I?\x9A\xA3\x97M\xCD\xBB\x87\xE6\xFE\xA5L\xCD\xB9\x10>\xA7\x83.m\xF0\x8D\xB8\x1Co\x90\xD2\x9D\x8F\x8F\x8F\x8F\x8F\x8F`@Qa\x17\x0E\x96\x95\x94\x93\x92\x91\x90\x95\x86R` \x86\x01\x94\x90\x94Ra\xFF\xFF\x92\x83\x16`@\x86\x01R\x91\x16``\x84\x01R`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\x80\x84\x01R\x16`\xA0\x82\x01R`\xC0\x01\x90V[`@Q\x80\x91\x03\x90\xA4a\x17\x1EaF\xB5V[PPP\x99\x98PPPPPPPPPV[`\x01`\x01`@\x1B\x03\x81\x16`\0\x90\x81R`\n` \x90\x81R`@\x80\x83 \x81Qa\x01\0\x81\x01\x83R\x81T`\x01`\x01`\x80\x1B\x03\x80\x82\x16\x83R`\x01`\x80\x1B\x91\x82\x90\x04\x81\x16\x95\x83\x01\x95\x90\x95R`\x01\x83\x01T\x94\x85\x16\x93\x82\x01\x93\x90\x93R\x91\x83\x04c\xFF\xFF\xFF\xFF\x16``\x83\x01R`\x01`\xA0\x1B\x83\x04a\xFF\xFF\x90\x81\x16`\x80\x84\x01R`\x01`\xB0\x1B\x90\x93\x04\x90\x92\x16`\xA0\x82\x01R`\x02\x82\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\xC0\x83\x01R`\x03\x90\x92\x01T\x90\x91\x16`\xE0\x82\x01R\x81\x90\x81\x90\x81\x90a\x17\xE4\x90aG\0V[` \x87\x81\x1Cb\xFF\xFF\xFF\x16`\0\x90\x81R`\t\x82R`@\x90\x81\x90 \x81Q`\x80\x81\x01\x83R\x81T`\x01`\x01`\xA0\x1B\x03\x80\x82\x16\x83R`\xFF`\x01`\xA0\x1B\x92\x83\x90\x04\x81\x16\x96\x84\x01\x87\x90R`\x01\x90\x94\x01T\x90\x81\x16\x94\x83\x01\x94\x90\x94R\x90\x92\x04\x16``\x82\x01R`\x01`\x01`\x80\x1B\x03\x93\x84\x16\x95P\x91\x90\x92\x16\x92P\x90a\x18_\x90\x84\x90aGYV[\x94Pa\x18{\x81``\x01Q`\xFF\x16\x83aGY\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x93PPPP\x91P\x91V[\x84\x83\x14a\x18\xCBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rn\t\x88\xA9\xC8\xEA\x89\x0B\xE9\xA9*i\xA8*\x88i`\x8B\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[3`\x01`\x01`\xA0\x1B\x03\x89\x16\x14\x80a\x19\x05WP`\x01`\x01`\xA0\x1B\x03\x88\x16`\0\x90\x81R`\x01` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x90 T`\xFF\x16[a\x19BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rm\x13\x93\xD5\x17\xD0UU\x12\x13\xD4\x92V\x91Q`\x92\x1B`D\x82\x01R`d\x01a\x18\xC2V[`\0\x80`\0[\x87\x81\x10\x15a\x19\xFDW\x88\x88\x82\x81\x81\x10a\x19bWa\x19bao\x05V[\x90P` \x02\x015\x92P\x86\x86\x82\x81\x81\x10a\x19}Wa\x19}ao\x05V[`\x01`\x01`\xA0\x1B\x03\x8E\x16`\0\x90\x81R` \x81\x81R`@\x80\x83 \x89\x84R\x82R\x82 \x80T\x93\x90\x91\x02\x94\x90\x94\x015\x95P\x85\x93\x92P\x90a\x19\xBA\x90\x84\x90ao\x1BV[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x8A\x16`\0\x90\x81R` \x81\x81R`@\x80\x83 \x86\x84R\x90\x91R\x81 \x80T\x84\x92\x90a\x19\xF0\x90\x84\x90ao.V[\x90\x91UPP`\x01\x01a\x19HV[P\x88`\x01`\x01`\xA0\x1B\x03\x16\x8A`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x7FJ9\xDC\x06\xD4\xC0\xDB\xC6Kp\xAF\x90\xFDi\x8A#:Q\x8A\xA5\xD0~Y]\x98;\x8C\x05&\xC8\xF7\xFB\x8B\x8B\x8B\x8B`@Qa\x1AQ\x94\x93\x92\x91\x90ao\xBDV[`@Q\x80\x91\x03\x90\xA4`\x01`\x01`\xA0\x1B\x03\x89\x16;\x15a\x1BaW`@Qc\xBC\x19|\x81`\xE0\x1B\x80\x82R\x90`\x01`\x01`\xA0\x1B\x03\x8B\x16\x90c\xBC\x19|\x81\x90a\x1A\xA5\x903\x90\x8F\x90\x8E\x90\x8E\x90\x8E\x90\x8E\x90\x8E\x90\x8E\x90`\x04\x01ao\xE4V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1A\xFDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` av\xB8\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x1B\x11W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B5\x91\x90apHV[`\x01`\x01`\xE0\x1B\x03\x19\x16\x14a\x1B\\W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x18\xC2\x90aphV[a\x1B\x87V[`\x01`\x01`\xA0\x1B\x03\x89\x16a\x1B\x87W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x18\xC2\x90ap\x92V[PPPPPPPPPPV[`\0\x80a\x1B\x9EaC\xB1V[`\x0FT`\xFF\x16\x15\x15`\0\x03a\x1B\xB5Wa\x1B\xB5aGoV[\x85`\x01`\x01`@\x1B\x03\x16`\0\x03a\x1B\xDAW`\x0FTa\x01\0\x90\x04`\x01`\x01`@\x1B\x03\x16\x95P[`\0a\x1B\xE5\x87a!]V[`\x01`\x01`\xA0\x1B\x03\x16\x03a\x1C\x17W`@Qc\t\x0Bp}`\xE1\x1B\x81R`\x01`\x01`@\x1B\x03\x87\x16`\x04\x82\x01R`$\x01a\x18\xC2V[a\x1C \x86a!]V[`@Qc\xE6\x04{\x19`\xE0\x1B\x81R`\x01`\x01`@\x1B\x03\x88\x16`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90c\xE6\x04{\x19\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x1C\xA8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` av\xB8\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x1C\xBCW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C\xE0\x91\x90an\xE5V[a\x1D\x08W`@Qc\xBC'\xA5\x17`\xE0\x1B\x81R`\x01`\x01`@\x1B\x03\x87\x16`\x04\x82\x01R`$\x01a\x18\xC2V[a\x1D%\x86\x85`\x01`\x01`\x80\x1B\x03\x16\x85`\x01`\x01`\x80\x1B\x03\x16aG\xD7V[` \x88\x81\x1Cb\xFF\xFF\xFF\x16`\0\x90\x81R`\t\x82R`@\x90\x81\x90 \x81Q`\x80\x81\x01\x83R\x81T`\x01`\x01`\xA0\x1B\x03\x80\x82\x16\x83R`\xFF`\x01`\xA0\x1B\x92\x83\x90\x04\x81\x16\x96\x84\x01\x96\x90\x96R`\x01\x90\x93\x01T\x92\x83\x16\x93\x82\x01\x93\x90\x93R\x91\x90\x04\x90\x91\x16``\x82\x01R\x91\x95P\x93P\x88\x15a\x1E\xA2W`\0a\x1D\x9E\x82`\0\x01Qa\"PV[\x90P`\0a\x1D\xAF\x83`@\x01Qa\"PV[\x90P`\0\x82\x12\x15a\x1D\xBFW`\0\x91P[`\0\x81\x12\x15a\x1D\xCCWP`\0[`\0\x80a\x1D\xDA\x8B\x85\x85aG\xD7V[`\x01`\x01`@\x1B\x03\x8D\x16`\0\x90\x81R`\n` \x90\x81R`@\x91\x82\x90 \x82Qa\x01\0\x81\x01\x84R\x81T`\x01`\x01`\x80\x1B\x03\x81\x81\x16\x83R`\x01`\x80\x1B\x91\x82\x90\x04\x81\x16\x94\x83\x01\x94\x90\x94R`\x01\x83\x01T\x80\x85\x16\x95\x83\x01\x95\x90\x95Rc\xFF\xFF\xFF\xFF\x90\x85\x04\x81\x16``\x83\x01Ra\xFF\xFF`\x01`\xA0\x1B\x86\x04\x81\x16`\x80\x84\x01R`\x01`\xB0\x1B\x90\x95\x04\x90\x94\x16`\xA0\x82\x01R`\x02\x82\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\xC0\x83\x01R`\x03\x90\x92\x01T\x90\x91\x16`\xE0\x82\x01R\x93\x95P\x91\x93Pa\x1E\x9B\x92\x91\x80\x86\x16\x91\x90\x85\x16\x90aH\xB3\x16V[\x99PPPPP[\x85`\x01`\x01`\x80\x1B\x03\x16`\0\x03a\x1E\xCCW`@Qc\x90`\x9A}`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1F\x86a\x1E\xD8\x87aI\xAAV[`\x01`\x01`@\x1B\x03\x89\x16`\0\x90\x81R`\n` \x90\x81R`@\x91\x82\x90 \x82Qa\x01\0\x81\x01\x84R\x81T`\x01`\x01`\x80\x1B\x03\x80\x82\x16\x83R`\x01`\x80\x1B\x91\x82\x90\x04\x81\x16\x94\x83\x01\x94\x90\x94R`\x01\x83\x01T\x93\x84\x16\x94\x82\x01\x94\x90\x94Rc\xFF\xFF\xFF\xFF\x93\x83\x04\x84\x16``\x82\x01Ra\xFF\xFF`\x01`\xA0\x1B\x84\x04\x81\x16`\x80\x83\x01R`\x01`\xB0\x1B\x90\x93\x04\x90\x92\x16`\xA0\x83\x01R`\x02\x81\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\xC0\x84\x01R`\x03\x90\x91\x01T\x16`\xE0\x82\x01R\x91\x90aI\xC4\x16V[`\x01`\x01`\x80\x1B\x03\x91\x82\x16\x94P\x81\x16\x92P\x85\x16\x83\x11\x15a\x1F\xB9W`@QcVr\x0E\x1D`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x83`\x01`\x01`\x80\x1B\x03\x16\x82\x11\x15a\x1F\xE3W`@Qc!0\x16\x97`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0`@Q\x80a\x01\0\x01`@R\x80B\x81R` \x01\x85\x81R` \x01\x84\x81R` \x01a \x0C\x89aI\xAAV[`\x0F\x0B\x81R` \x01\x89`\x01`\x01`@\x1B\x03\x16\x81R` \x01\x8A`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83`@\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81RP\x90Pa a\x81aJ\xB4V[` \x82\x01Qa t\x90\x85\x90`\xFF\x16aGYV[``\x83\x01Qa \x87\x90\x85\x90`\xFF\x16aGYV[\x90\x94P\x92P\x83\x15\x80\x15a \x98WP\x82\x15[\x15a \xB6W`@Qce\x8B\x16\xED`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`@\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x82`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x89`\x01`\x01`@\x1B\x03\x16\x7F\xFD\xFF\xEC\xA7Q\xF0\xDC\xAA\xB7U1\xCB\x81<\x12\xBB\xFDV\xEE>\x96L\xC4q\xD7\xEFC\x93$\x02\xEE\x18\x87\x87\x8C`@Qa!)\x93\x92\x91\x90\x92\x83R` \x83\x01\x91\x90\x91R`\x01`\x01`\x80\x1B\x03\x16`@\x82\x01R``\x01\x90V[`@Q\x80\x91\x03\x90\xA4`\x0FT`\xFF\x16\x15\x15`\0\x03a!HWa!HaMpV[a!PaF\xB5V[PP\x96P\x96\x94PPPPPV[`\x01`\x01`@\x1B\x03\x16`\0\x90\x81R`\n` R`@\x90 `\x03\x01T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\0a!\x8F\x82a!]V[`@Qc\x1C\xA1\xA6\xAD`\xE1\x1B\x81R`\x01`\x01`@\x1B\x03\x84\x16`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90c9CMZ\x90`$\x01[` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\"\x18W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` av\xB8\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\",W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12]\x91\x90anDV[`\0a\x12]`\x02\x830aQ\xABV[``\x83\x82\x14a\"\xA1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rn\t\x88\xA9\xC8\xEA\x89\x0B\xE9\xA9*i\xA8*\x88i`\x8B\x1B`D\x82\x01R`d\x01a\x18\xC2V[\x83`\x01`\x01`@\x1B\x03\x81\x11\x15a\"\xB9Wa\"\xB9ai\x18V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\"\xE2W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x84\x81\x10\x15a#\x89W`\0\x80\x87\x87\x84\x81\x81\x10a#\x05Wa#\x05ao\x05V[\x90P` \x02\x01` \x81\x01\x90a#\x1A\x91\x90ag\xAEV[`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x85\x85\x84\x81\x81\x10a#NWa#Nao\x05V[\x90P` \x02\x015\x81R` \x01\x90\x81R` \x01`\0 T\x82\x82\x81Q\x81\x10a#vWa#vao\x05V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\"\xE8V[P\x94\x93PPPPV[`\0\x80a#\x9DaC\xB1V[`\x0FT`\xFF\x16\x15\x15`\0\x03a#\xB4Wa#\xB4aGoV[a#\xBD\x86a!]V[`@Qc\xE6\x04{\x19`\xE0\x1B\x81R`\x01`\x01`@\x1B\x03\x88\x16`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90c\xE6\x04{\x19\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a$EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` av\xB8\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a$YW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a$}\x91\x90an\xE5V[a$\xA5W`@Qc\t\x0Bp}`\xE1\x1B\x81R`\x01`\x01`@\x1B\x03\x87\x16`\x04\x82\x01R`$\x01a\x18\xC2V[a$\xC2\x86\x85`\x01`\x01`\x80\x1B\x03\x16\x85`\x01`\x01`\x80\x1B\x03\x16aG\xD7V[` \x88\x81\x1Cb\xFF\xFF\xFF\x16`\0\x90\x81R`\t\x82R`@\x90\x81\x90 \x81Q`\x80\x81\x01\x83R\x81T`\x01`\x01`\xA0\x1B\x03\x80\x82\x16\x80\x84R`\xFF`\x01`\xA0\x1B\x93\x84\x90\x04\x81\x16\x97\x85\x01\x97\x90\x97R`\x01\x90\x94\x01T\x90\x81\x16\x94\x83\x01\x85\x90R\x04\x90\x93\x16``\x84\x01R\x93\x97P\x91\x95P\x91\x90\x89\x15a%[W3`\0\x90\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`@\x1B\x03\x8D\x16\x84R\x90\x91R\x90 Ta%X\x90aR\xC3V[\x97P[\x87`\x01`\x01`\x80\x1B\x03\x16`\0\x03a%\x85W`@Qc\nw\xB0/`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a&Ha%\x91\x89aI\xAAV[a%\x9A\x90ap\xBDV[`\x01`\x01`@\x1B\x03\x8B\x16`\0\x90\x81R`\n` \x90\x81R`@\x91\x82\x90 \x82Qa\x01\0\x81\x01\x84R\x81T`\x01`\x01`\x80\x1B\x03\x80\x82\x16\x83R`\x01`\x80\x1B\x91\x82\x90\x04\x81\x16\x94\x83\x01\x94\x90\x94R`\x01\x83\x01T\x93\x84\x16\x94\x82\x01\x94\x90\x94Rc\xFF\xFF\xFF\xFF\x93\x83\x04\x84\x16``\x82\x01Ra\xFF\xFF`\x01`\xA0\x1B\x84\x04\x81\x16`\x80\x83\x01R`\x01`\xB0\x1B\x90\x93\x04\x90\x92\x16`\xA0\x83\x01R`\x02\x81\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\xC0\x84\x01R`\x03\x90\x91\x01T\x16`\xE0\x82\x01R\x91\x90aI\xC4\x16V[`\x01`\x01`\x80\x1B\x03\x91\x82\x16\x96P\x81\x16\x94P\x87\x16\x85\x10\x15a&{W`@Qc\x064HC`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x85`\x01`\x01`\x80\x1B\x03\x16\x84\x10\x15a&\xA5W`@QcVZ\xDE\xF5`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0`@Q\x80a\x01\0\x01`@R\x80B\x81R` \x01\x87\x81R` \x01\x86\x81R` \x01a&\xCE\x8BaI\xAAV[a&\xD7\x90ap\xBDV[`\x0F\x0B\x81R`\x01`\x01`@\x1B\x03\x8C\x16` \x82\x01R3`@\x82\x01R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16``\x83\x01R\x84\x16`\x80\x90\x91\x01R\x90Pa'\x14\x81aJ\xB4V[` \x84\x01Qa''\x90\x87\x90`\xFF\x16aGYV[``\x85\x01Qa':\x90\x87\x90`\xFF\x16aGYV[`@\x80Q\x83\x81R` \x81\x01\x83\x90R`\x01`\x01`\x80\x1B\x03\x8D\x16\x81\x83\x01R\x90Q\x92\x98P\x90\x96P`\x01`\x01`\xA0\x1B\x03\x84\x81\x16\x92\x90\x86\x16\x91`\x01`\x01`@\x1B\x03\x8E\x16\x91\x7F0\x84\xCA\xF4\x89f\\\xAB\x07E,\xFEO=\x0E\xB5\xE0\xDC\x15\xEA\xC6\xFCe\x80\x98\x85\x8Ec\x9Ep\xE5:\x91\x81\x90\x03``\x01\x90\xA4`\x0FT`\xFF\x16\x15\x15`\0\x03a'\xBAWa'\xBAaMpV[a'\xC2aF\xB5V[PPPP\x95P\x95\x93PPPPV[a'\xD8aC\xB1V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xF7|G\x91`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a(oW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` av\xB8\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a(\x83W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a(\xA7\x91\x90ap\xE3V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a(\xDBW`@Q`\x01b\x10X\xF5`\xE2\x1B\x03\x19\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x14\x81\x11\x80a(\xEAWP`\x04\x81\x10[\x15a)\x0BW`@QcdYtw`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x18\xC2V[`\r\x80T\x90\x82\x90U`@\x80Q\x82\x81R` \x81\x01\x84\x90R\x7F\x81\xC99\x14H\0(v\x03G\x9B\x97\xBB\xA9\xC1\x12\x88\xCEz\xBCZ\xCBH\x90y\xE1Y\xF3\\\xF9\x8B\xD1\x91\x01`@Q\x80\x91\x03\x90\xA1a)TaF\xB5V[PPV[`\0\x80`\0a)j\x86``\x01Qa!]V[`\x01`\x01`\xA0\x1B\x03\x16c\x80\xAF\x9Dv\x87\x87\x87`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a)\x99\x93\x92\x91\x90aq\x03V[```@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a)\xEFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` av\xB8\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a*\x03W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a*'\x91\x90aq0V[\x92P\x92P\x92P\x93P\x93P\x93\x90PV[`\x01`\x01`@\x1B\x03\x82\x16`\0\x90\x81R`\n` \x90\x81R`@\x80\x83 \x81Qa\x01\0\x81\x01\x83R\x81T`\x01`\x01`\x80\x1B\x03\x80\x82\x16\x83R`\x01`\x80\x1B\x91\x82\x90\x04\x81\x16\x95\x83\x01\x95\x90\x95R`\x01\x83\x01T\x94\x85\x16\x93\x82\x01\x93\x90\x93Rc\xFF\xFF\xFF\xFF\x92\x84\x04\x83\x16``\x82\x01Ra\xFF\xFF`\x01`\xA0\x1B\x85\x04\x81\x16`\x80\x83\x01R`\x01`\xB0\x1B\x90\x94\x04\x90\x93\x16`\xA0\x84\x01R`\x02\x81\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\xC0\x85\x01R`\x03\x90\x91\x01T\x16`\xE0\x83\x01R\x82\x91\x82\x91\x82\x91a*\xED\x91\x87\x90aI\xC4\x16V[` \x88\x81\x1Cb\xFF\xFF\xFF\x16`\0\x90\x81R`\t\x82R`@\x80\x82 \x81Q`\x80\x81\x01\x83R\x81T`\x01`\x01`\xA0\x1B\x03\x80\x82\x16\x83R`\xFF`\x01`\xA0\x1B\x92\x83\x90\x04\x81\x16\x97\x84\x01\x97\x90\x97R`\x01\x90\x93\x01T\x92\x83\x16\x93\x82\x01\x93\x90\x93R\x91\x90\x04\x90\x92\x16``\x83\x01R`\x01`\x01`\x80\x1B\x03\x93\x84\x16\x95P\x91\x90\x92\x16\x92P`\x0F\x87\x90\x0B\x12\x15a+\xB1Wa+\x8Ba+\x86\x82` \x01Q`\xFF\x16\x85aGY\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[aR\xC3V[\x94Pa+\xAAa+\x86\x82``\x01Q`\xFF\x16\x84aGY\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x93Pa+\xF0V[a+\xCEa+\x86\x82` \x01Q`\xFF\x16\x85aR\xD5\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x94Pa+\xEDa+\x86\x82``\x01Q`\xFF\x16\x84aR\xD5\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x93P[PPP\x92P\x92\x90PV[a,\x02aC\xB1V[`\x01`\x01`@\x1B\x03\x83\x16`\0\x90\x81R`\n` R`@\x90 `\x02\x81\x01T`\x01`\x01`\xA0\x1B\x03\x163\x14a,JW`@Q`\x01b\x10X\xF5`\xE2\x1B\x03\x19\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\xFF\xFF\x82\x16\x15a,\xB7Wa,wa\xFF\xFF\x83\x16`\x01a\x03\xE8\x81\x83\x14\x91\x83\x11\x91\x90\x91\x17\x82\x82\x14\x91\x90\x92\x10\x17\x16\x90V[a,\x9AW`@Qc\x97\x1B1\t`\xE0\x1B\x81Ra\xFF\xFF\x83\x16`\x04\x82\x01R`$\x01a\x18\xC2V[`\x01\x81\x01\x80Ta\xFF\xFF`\xA0\x1B\x19\x16`\x01`\xA0\x1Ba\xFF\xFF\x85\x16\x02\x17\x90U[a\xFF\xFF\x83\x16\x15a-*W`\x01\x81\x81\x01T`\x01`\xA0\x1B\x90\x04a\xFF\xFF\x90\x81\x16\x90\x85\x16\x81\x81\x10\x91\x81\x14\x91\x90\x91\x17\x82\x82\x11\x91\x90\x92\x14\x17\x16a-\rW`@Qc\xED\xDF\xE1\x19`\xE0\x1B\x81Ra\xFF\xFF\x84\x16`\x04\x82\x01R`$\x01a\x18\xC2V[`\x01\x81\x01\x80Ta\xFF\xFF`\xB0\x1B\x19\x16`\x01`\xB0\x1Ba\xFF\xFF\x86\x16\x02\x17\x90U[\x81a\xFF\xFF\x16\x83a\xFF\xFF\x16\x85`\x01`\x01`@\x1B\x03\x16\x7F\x80N\x0E\xF8\xEB\xD1\x19o\x98\xB3\xC6\xA20\xDE\xFF\xD8\xCF\xE0<\xB1\x92?\xE0\xE4\x02%-\x06\xD8\xD4v\xDA`@Q`@Q\x80\x91\x03\x90\xA4a-raF\xB5V[PPPPV[3`\0\x81\x81R`\x01` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x87\x16\x80\x85R\x90\x83R\x92\x81\x90 \x80T`\xFF\x19\x16\x86\x15\x15\x90\x81\x17\x90\x91U\x90Q\x90\x81R\x91\x92\x91\x7F\x170~\xAB9\xABa\x07\xE8\x89\x98E\xAD=Y\xBD\x96S\xF2\0\xF2 \x92\x04\x89\xCA+Y7il1\x91\x01`@Q\x80\x91\x03\x90\xA3PPV[`\x0FT``\x90`\xFF\x16\x15a.\x0BW`@QcU\xE1\xF7\xC5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a.\x13aC\xB1V[`\x0F\x80T`\xFF\x19\x16`\x01\x17\x90Ua.(aGoV[\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a.@Wa.@ai\x18V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a.sW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a.^W\x90P[P\x90P`\0[\x82\x81\x10\x15a/<W`\0\x800\x86\x86\x85\x81\x81\x10a.\x97Wa.\x97ao\x05V[\x90P` \x02\x81\x01\x90a.\xA9\x91\x90aq\xAFV[`@Qa.\xB7\x92\x91\x90arxV[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14a.\xF2W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a.\xF7V[``\x91P[P\x91P\x91P\x81a/\tW\x80Q\x81` \x01\xFD[\x80\x84\x84\x81Q\x81\x10a/\x1CWa/\x1Cao\x05V[` \x02` \x01\x01\x81\x90RPPP\x80\x80a/4\x90ar\x88V[\x91PPa.yV[P`\x0F\x80T`\xFF\x19\x16\x90Ua/OaMpV[a\x12]aF\xB5V[`\0a/aaC\xB1V[\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x03a/\x93W`@Qc\x01D\xD33`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P`\x01`\x01`\xA0\x1B\x03\x80\x83\x16`\0\x90\x81R`\x0B` \x90\x81R`@\x80\x83 \x93\x85\x16\x83R\x92\x90R Tb\xFF\xFF\xFF\x16\x80\x15a/\xE5W`@Qc\xB0\x98\x8CC`\xE0\x1B\x81Rb\xFF\xFF\xFF\x82\x16`\x04\x82\x01R`$\x01a\x18\xC2V[`\0\x80\x84`\x01`\x01`\xA0\x1B\x03\x16c1<\xE5g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a0_W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` av\xB8\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a0sW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a0\x97\x91\x90ar\xA1V[\x84`\x01`\x01`\xA0\x1B\x03\x16c1<\xE5g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a1\x0EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` av\xB8\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a1\"W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a1F\x91\x90ar\xA1V[\x90\x92P\x90Pa1l`\xFF\x83\x16`\x06`\x12\x81\x83\x14\x91\x83\x11\x91\x90\x91\x17\x82\x82\x14\x91\x90\x92\x10\x17\x16\x90V[a1\x8EW`@Qc\xC3\xDAwG`\xE0\x1B\x81R`\xFF\x83\x16`\x04\x82\x01R`$\x01a\x18\xC2V[a1\xAF`\xFF\x82\x16`\x06`\x12\x81\x83\x14\x91\x83\x11\x91\x90\x91\x17\x82\x82\x14\x91\x90\x92\x10\x17\x16\x90V[a1\xD1W`@Qc\xC3\xDAwG`\xE0\x1B\x81R`\xFF\x82\x16`\x04\x82\x01R`$\x01a\x18\xC2V[`\x06\x80T`\0\x90a1\xE6\x90b\xFF\xFF\xFF\x16ar\xC7V[\x82Ta\x01\0\x92\x90\x92\nb\xFF\xFF\xFF\x81\x81\x02\x19\x90\x93\x16\x92\x82\x16\x90\x81\x02\x92\x90\x92\x17\x90\x92U`\x01`\x01`\xA0\x1B\x03\x87\x81\x16`\0\x81\x81R`\x0B` \x90\x81R`@\x80\x83 \x8B\x86\x16\x80\x85R\x90\x83R\x81\x84 \x80Tb\xFF\xFF\xFF\x19\x16\x88\x17\x90U\x81Q`\x80\x81\x01\x83R\x85\x81R`\xFF\x8B\x81\x16\x82\x86\x01\x81\x81R\x83\x86\x01\x85\x81R\x8D\x84\x16``\x86\x01\x81\x81R\x8D\x8BR`\t\x8AR\x99\x88\x90 \x95Q\x86T\x93Q\x90\x8D\x16`\x01`\x01`\xA8\x1B\x03\x19\x94\x85\x16\x17`\x01`\xA0\x1B\x91\x87\x16\x82\x02\x17\x87U\x91Q`\x01\x96\x90\x96\x01\x80T\x9AQ\x96\x90\x9C\x16\x99\x90\x92\x16\x98\x90\x98\x17\x93\x90\x92\x16\x90\x96\x02\x91\x90\x91\x17\x90\x96U\x81Q\x93\x84R\x91\x83\x01\x94\x90\x94R\x94\x97P\x90\x92\x91\x7F\xC0\xC5\xDF\x98\xA4\xCA\x87\xA3!\xA3;\xF1'|\xF3-1\xA9{l\xE1K\x97G8!I\xB9\xE2c\x1E\xA3\x91\x01`@Q\x80\x91\x03\x90\xA4a2\xFAaF\xB5V[PP\x92\x91PPV[` \x83\x81\x1Cb\xFF\xFF\xFF\x16`\0\x90\x81R`\t\x82R`@\x80\x82 \x81Q`\x80\x81\x01\x83R\x81T`\x01`\x01`\xA0\x1B\x03\x80\x82\x16\x83R`\xFF`\x01`\xA0\x1B\x92\x83\x90\x04\x81\x16\x97\x84\x01\x88\x90R`\x01\x90\x94\x01T\x90\x81\x16\x94\x83\x01\x94\x90\x94R\x90\x92\x04\x16``\x82\x01R\x90\x91a3j\x90\x85\x90aS\x15V[``\x82\x01Qa3}\x90\x85\x90`\xFF\x16aS\x15V[`\x01`\x01`@\x1B\x03\x87\x16`\0\x90\x81R`\n` \x90\x81R`@\x91\x82\x90 \x82Qa\x01\0\x81\x01\x84R\x81T`\x01`\x01`\x80\x1B\x03\x80\x82\x16\x83R`\x01`\x80\x1B\x91\x82\x90\x04\x81\x16\x94\x83\x01\x94\x90\x94R`\x01\x83\x01T\x93\x84\x16\x94\x82\x01\x94\x90\x94Rc\xFF\xFF\xFF\xFF\x93\x83\x04\x84\x16``\x82\x01Ra\xFF\xFF`\x01`\xA0\x1B\x84\x04\x81\x16`\x80\x83\x01R`\x01`\xB0\x1B\x90\x93\x04\x90\x92\x16`\xA0\x83\x01R`\x02\x81\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\xC0\x84\x01R`\x03\x90\x91\x01T\x16`\xE0\x82\x01R\x92\x96P\x90\x94Pa48\x91\x90\x86\x90\x86\x90aH\xB3\x16V[\x91PP[\x93\x92PPPV[a4KaC\xB1V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xF7|G\x91`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a4\xE2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` av\xB8\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a4\xF6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a5\x1A\x91\x90ap\xE3V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a5NW`@Q`\x01b\x10X\xF5`\xE2\x1B\x03\x19\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80\x83`\x01`\x01`\xA0\x1B\x03\x16c1<\xE5g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a5\xC8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` av\xB8\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a5\xDCW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a6\0\x91\x90ar\xA1V[\x90P`\0\x19\x83\x03a6:W`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81R`\x07` R`@\x90 T\x91Pa63\x82`\xFF\x83\x16aGYV[\x92Pa6JV[a6G\x83`\xFF\x83\x16aS\x15V[\x91P[`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81R`\x07` R`@\x81 \x80T\x84\x92\x90a6r\x90\x84\x90ao\x1BV[\x90\x91UPa6\x82\x90P\x84\x83aS,V[\x83`\x01`\x01`\xA0\x1B\x03\x16\x7F\x1F\xDD\0 5\x88\x93U\x97\x13\xDE\xF8\xB4,\xADf\x1F\xFB\xC7U\xD1\xA2dY@'\x92\x14B\xBBV\xA0\x84`@Qa6\xBD\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2`\x0FT`\xFF\x16\x15\x15`\0\x03a6\xDCWa6\xDCaMpV[a-raF\xB5V[`\0a6\xEF\x82a!]V[`@Qc8\xCCn\x8D`\xE2\x1B\x81R`\x01`\x01`@\x1B\x03\x84\x16`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90c\xE31\xBA4\x90`$\x01a!\xC2V[`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x91\x90\x91Ra7Z\x84a!]V[`@Qcx=\xC3\xCF`\xE1\x1B\x81R`\x01`\x01`@\x1B\x03\x86\x16`\x04\x82\x01R\x84\x15\x15`$\x82\x01R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`D\x83\x01R\x91\x90\x91\x16\x90c\xF0{\x87\x9E\x90`d\x01`\xA0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a7\xF2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` av\xB8\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a8\x06W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a8*\x91\x90ar\xDFV[\x94\x93PPPPV[3`\x01`\x01`\xA0\x1B\x03\x87\x16\x14\x80a8lWP`\x01`\x01`\xA0\x1B\x03\x86\x16`\0\x90\x81R`\x01` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x90 T`\xFF\x16[a8\xA9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rm\x13\x93\xD5\x17\xD0UU\x12\x13\xD4\x92V\x91Q`\x92\x1B`D\x82\x01R`d\x01a\x18\xC2V[`\x01`\x01`\xA0\x1B\x03\x86\x16`\0\x90\x81R` \x81\x81R`@\x80\x83 \x87\x84R\x90\x91R\x81 \x80T\x85\x92\x90a8\xDA\x90\x84\x90ao\x1BV[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R` \x81\x81R`@\x80\x83 \x87\x84R\x90\x91R\x81 \x80T\x85\x92\x90a9\x10\x90\x84\x90ao.V[\x90\x91UPP`@\x80Q\x85\x81R` \x81\x01\x85\x90R`\x01`\x01`\xA0\x1B\x03\x80\x88\x16\x92\x90\x89\x16\x913\x91\x7F\xC3\xD5\x81h\xC5\xAEs\x97s\x1D\x06=[\xBF=exTBsC\xF4\xC0\x83$\x0Fz\xAC\xAA-\x0Fb\x91\x01`@Q\x80\x91\x03\x90\xA4`\x01`\x01`\xA0\x1B\x03\x85\x16;\x15a:dW`@Qc\xF2:na`\xE0\x1B\x80\x82R\x90`\x01`\x01`\xA0\x1B\x03\x87\x16\x90c\xF2:na\x90a9\xA8\x903\x90\x8B\x90\x8A\x90\x8A\x90\x8A\x90\x8A\x90`\x04\x01as\\V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a:\0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` av\xB8\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a:\x14W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a:8\x91\x90apHV[`\x01`\x01`\xE0\x1B\x03\x19\x16\x14a:_W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x18\xC2\x90aphV[a:\x8AV[`\x01`\x01`\xA0\x1B\x03\x85\x16a:\x8AW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x18\xC2\x90ap\x92V[PPPPPPV[`\0\x80`\0a:\x9FaC\xB1V[`\x0FT`\xFF\x16\x15\x15`\0\x03a:\xB6Wa:\xB6aGoV[a:\xC6`\x80\x85\x01``\x86\x01ae\xA1V[a:\xD3` \x86\x01\x86as\xA3V[a:\xE3`@\x87\x01` \x88\x01as\xA3V[\x91\x94P`\x01`\x01`\x80\x1B\x03\x90\x81\x16\x93P\x16\x90P`\0`\n\x81a;\x0B`\x80\x88\x01``\x89\x01ae\xA1V[`\x01`\x01`@\x1B\x03\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x90\x81\x01`\0 \x81Qa\x01\0\x81\x01\x83R\x81T`\x01`\x01`\x80\x1B\x03\x80\x82\x16\x83R`\x01`\x80\x1B\x91\x82\x90\x04\x81\x16\x95\x83\x01\x95\x90\x95R`\x01\x83\x01T\x94\x85\x16\x93\x82\x01\x93\x90\x93R\x91\x83\x04c\xFF\xFF\xFF\xFF\x16``\x83\x01\x81\x90R`\x01`\xA0\x1B\x84\x04a\xFF\xFF\x90\x81\x16`\x80\x85\x01R`\x01`\xB0\x1B\x90\x94\x04\x90\x93\x16`\xA0\x83\x01R`\x02\x81\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\xC0\x84\x01R`\x03\x82\x01T\x16`\xE0\x90\x92\x01\x91\x90\x91R\x91Pa;\xE0W`@Qc\t\x0Bp}`\xE1\x1B\x81R`\x01`\x01`@\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\x18\xC2V[`@\x80Qa\x01\0\x81\x01\x82R\x82T`\x01`\x01`\x80\x1B\x03\x80\x82\x16\x83R`\x01`\x80\x1B\x91\x82\x90\x04\x81\x16` \x84\x01R`\x01\x85\x01T\x90\x81\x16\x93\x83\x01\x93\x90\x93Rc\xFF\xFF\xFF\xFF\x90\x83\x04\x16``\x82\x01Ra\xFF\xFF`\x01`\xA0\x1B\x83\x04\x81\x16`\x80\x83\x01R`\x01`\xB0\x1B\x90\x92\x04\x90\x91\x16`\xA0\x82\x01R`\x02\x82\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\xC0\x83\x01R`\x03\x83\x01T\x16`\xE0\x82\x01Ra<q\x90aS\x7FV[\x15a<\x8FW`@Qcz\x95\xCB!`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a<\x9A\x85a!]V[`@Qc\xE6\x04{\x19`\xE0\x1B\x81R`\x01`\x01`@\x1B\x03\x87\x16`\x04\x82\x01R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c\xE6\x04{\x19\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a=#W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` av\xB8\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a=7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a=[\x91\x90an\xE5V[a=\x83W`@Qc\xBC'\xA5\x17`\xE0\x1B\x81R`\x01`\x01`@\x1B\x03\x86\x16`\x04\x82\x01R`$\x01a\x18\xC2V[a=\x8D\x82BaS\xADV[`\0\x80`\x01`\x01`\xA0\x1B\x03\x83\x16c\xECshT\x88a=\xB0`\xA0\x8C\x01`\x80\x8D\x01as\xC3V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81R`\x01`\x01`@\x1B\x03\x90\x92\x16`\x04\x83\x01R\x15\x15`$\x82\x01R3`D\x82\x01R`d\x01`@\x80Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a>;W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` av\xB8\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a>OW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a>s\x91\x90as\xE3V[\x91P\x91P\x81a>\x95W`@Qc.\xD0\xEA\x01`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x87\x81\x1Cb\xFF\xFF\xFF\x16`\0\x90\x81R`\t\x82R`@\x90\x81\x90 \x81Q`\x80\x81\x01\x83R\x81T`\x01`\x01`\xA0\x1B\x03\x80\x82\x16\x83R`\xFF`\x01`\xA0\x1B\x92\x83\x90\x04\x81\x16\x96\x84\x01\x96\x90\x96R`\x01\x90\x93\x01T\x92\x83\x16\x93\x82\x01\x93\x90\x93R\x91\x90\x04\x90\x91\x16``\x82\x01Ra>\xFCa`gV[a?\x0C`\xA0\x8B\x01`\x80\x8C\x01as\xC3V[\x15a?OW` \x82\x01Q`\xFF\x90\x81\x16a\x01\0\x83\x01R``\x83\x01Q\x16a\x01 \x82\x01R\x81Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\xC0\x83\x01R`@\x83\x01Q\x16`\xE0\x82\x01Ra?\x89V[``\x82\x01Q`\xFF\x90\x81\x16a\x01\0\x83\x01R` \x83\x01Q\x16a\x01 \x82\x01R`@\x82\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\xC0\x83\x01R\x82Q\x16`\xE0\x82\x01R[a?\x99``\x8B\x01`@\x8C\x01as\xC3V[\x15a?\xCFW`\0a?\xAD\x82`\xC0\x01Qa\"PV[\x90P`\0\x81\x13\x15a?\xCDWa?\xC1\x81aR\xC3V[`\x01`\x01`\x80\x1B\x03\x16\x98P[P[\x82\x81R`\x80\x81\x01\x88\x90R`\xA0\x81\x01\x87\x90Ra?\xE9\x81aS\xDCV[\x90P\x80`\xA0\x01Q`\0\x03a@\x10W`@Qc7\xC2\xD9\xA7`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80`\x80\x01Q`\0\x03a@5W`@Qc\x13\xFD\x9Bm`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a@F6\x8C\x90\x03\x8C\x01\x8Cat\x14V[\x90Pa@U\x82`\x80\x01QaR\xC3V[a@b\x83`\xA0\x01QaR\xC3V[`\x01`\x01`\x80\x1B\x03\x90\x81\x16` \x84\x01R\x16\x81R`\x02\x87\x01T`\0\x90`\x01`\x01`\xA0\x1B\x03\x163\x14a@\xA1W`\x01\x88\x01T`\x01`\xA0\x1B\x90\x04a\xFF\xFF\x16a@\xB2V[`\x01\x88\x01T`\x01`\xB0\x1B\x90\x04a\xFF\xFF\x16[\x88T`\rTa\xFF\xFF\x92\x90\x92\x16\x92P`\0\x91\x82\x91a@\xE8\x91\x86\x91`\x01`\x01`\x80\x1B\x03\x80\x82\x16\x92`\x01`\x80\x1B\x90\x92\x04\x16\x90\x87\x90aTXV[\x88`@\x01\x89``\x01\x82\x96P\x83\x97P\x84\x81RP\x84\x81RPPPPP`\0\x89`\x01`\x01`\xA0\x1B\x03\x16c\xA4G\x89\x19\x8F\x88`\0\x01Q\x86\x86`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aA[\x94\x93\x92\x91\x90`\x01`\x01`@\x1B\x03\x94\x90\x94\x16\x84R` \x84\x01\x92\x90\x92R`@\x83\x01R``\x82\x01R`\x80\x01\x90V[`@\x80Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15aA\xB0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` av\xB8\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15aA\xC4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aA\xE8\x91\x90as\xE3V[` \x88\x01R\x90P\x80aB\x1DW\x85Q` \x87\x01Q`@Qc\\\x9E\xF7\x05`\xE1\x1B\x81R`\x04\x81\x01\x92\x90\x92R`$\x82\x01R`D\x01a\x18\xC2V[PaBX\x92PaB6\x91PP`\xA0\x8D\x01`\x80\x8E\x01as\xC3V[\x83``\x01Q\x84`\x80\x01QaBJ\x91\x90ao\x1BV[`\xA0\x85\x01Q\x8A\x92\x91\x90aT\x8CV[aBj\x82`\xC0\x01Q\x83`\x80\x01QaV#V[aB|\x82`\xE0\x01Q\x83`\xA0\x01QaS,V[``\x82\x01Q\x15aB\xBDW``\x82\x01Q`\xC0\x83\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x07` R`@\x81 \x80T\x90\x91\x90aB\xB7\x90\x84\x90ao.V[\x90\x91UPP[aB\xC6\x82aVjV[\x91P\x81`\xE0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x82`\xC0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x8B`\x01`\x01`@\x1B\x03\x16\x7F\xCD\x80T&o\xAE;\xBD#\0\xC8\x9A@t\xC4\x11=\x12\xB9\xA5\x83\xF7\xCD#\x90\xD40\xAFp\xB1*X\x8E`\x80\x01` \x81\x01\x90aC\"\x91\x90as\xC3V[\x8D\x8D\x88`@\x01Q\x89` \x01Q`@QaC_\x95\x94\x93\x92\x91\x90\x94\x15\x15\x85R` \x85\x01\x93\x90\x93R`@\x84\x01\x91\x90\x91R``\x83\x01R`\x80\x82\x01R`\xA0\x01\x90V[`@Q\x80\x91\x03\x90\xA4`\x0FT`\xFF\x16\x15\x15`\0\x03aC~WaC~aMpV[aC\x86aF\xB5V[PPPPPPP\x91\x93\x90\x92PV[``` `\0Rk\x0Bv1.5.0-beta`+R```\0\xF3[`\x0CT`\x01\x14\x15\x80\x15aC\xD3WP`\x0FT`\xFF\x16\x15\x80aC\xD3WP`\x02`\x0CT\x11[\x15aC\xF1W`@Qc\x02\xB8\0-`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x0C\x80T\x90`\0aD\x01\x83ar\x88V[\x91\x90PUPV[`\0\x80aDK\x86aD\x1AW`\0aD\x1DV[`\x01[`\xF8\x1B\x86aD,W`\0aD/V[`\x01[`\x0F`\xF8\x1B`\xF8\x91\x90\x91\x1B\x16`\x04\x91\x90\x91\x1B`\x0F`\xFC\x1B\x16\x17\x90V[`\xF8\x1C\x90P\x82\x84` \x1B\x82`8\x1B\x17\x17\x91PP\x94\x93PPPPV[`@\x80Qa\x01\0\x81\x01\x82R\x88T`\x01`\x01`\x80\x1B\x03\x80\x82\x16\x83R`\x01`\x80\x1B\x91\x82\x90\x04\x81\x16` \x84\x01R`\x01\x8B\x01T\x90\x81\x16\x93\x83\x01\x93\x90\x93R\x82\x04c\xFF\xFF\xFF\xFF\x16``\x82\x01\x81\x90R`\x01`\xA0\x1B\x83\x04a\xFF\xFF\x90\x81\x16`\x80\x84\x01R`\x01`\xB0\x1B\x90\x93\x04\x90\x92\x16`\xA0\x82\x01R`\x02\x89\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\xC0\x83\x01R`\x03\x8A\x01T\x16`\xE0\x90\x91\x01R\x15aE\x0FW`@Qc\xE90\xCE\xDF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[aE\x19\x87BaS\xADV[\x85`\0\x03aE:W`@Qc\x17O\xD4\x1B`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[aEC\x86aR\xC3V[\x87T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x91\x90\x91\x16\x17\x87U`\0\x85\x90\x03aE\x80W`@Qc(i\xC5\xF3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[aE\x89\x85aR\xC3V[\x87T`\x01`\x01`\x80\x1B\x03\x91\x82\x16`\x01`\x80\x1B\x02\x91\x16\x17\x87U`\x01\x84\x81\x14\x90\x85\x11\x17a\x03\xE8\x85\x81\x14\x90\x86\x10\x17\x16aE\xD5W`@Qc\x97\x1B1\t`\xE0\x1B\x81R`\x04\x81\x01\x85\x90R`$\x01a\x18\xC2V[aE\xDE\x84aV\xDDV[`\x01\x88\x01\x80Ta\xFF\xFF\x92\x90\x92\x16`\x01`\xA0\x1B\x02a\xFF\xFF`\xA0\x1B\x19\x90\x92\x16\x91\x90\x91\x17\x90U`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x80\x15\x90aF\x89W\x84\x84\x10\x85\x85\x14\x17`\x01\x80\x86\x11\x90\x86\x14\x17\x16aFDW`@Qc\xED\xDF\xE1\x19`\xE0\x1B\x81R`\x04\x81\x01\x85\x90R`$\x01a\x18\xC2V[`\x02\x88\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x85\x16\x17\x90UaFj\x84aV\xDDV[\x88`\x01\x01`\x16a\x01\0\n\x81T\x81a\xFF\xFF\x02\x19\x16\x90\x83a\xFF\xFF\x16\x02\x17\x90UP[P`\x03\x96\x90\x96\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x97\x16\x96\x90\x96\x17\x90\x95UPPPPPV[`\x0C\x80T\x90`\0aF\xC5\x83at3V[\x90\x91UPP`\x05T`\xFF\x16\x15\x80\x15aF\xE0WP`\x0FT`\xFF\x16\x15[\x15aF\xFEW`@Qc2n\xFAC`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[`\0\x80`\x01`\x01`\x7F\x1B\x03\x83`@\x01Q`\x01`\x01`\x80\x1B\x03\x16\x11\x15aG8W`@Qc\xAC\xC9@{`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[aGP\x83`@\x01QaGI\x90ap\xBDV[\x84\x90aI\xC4V[\x91P\x91P\x91P\x91V[`\0\x80aGe\x83aV\xEEV[\x90\x93\x04\x93\x92PPPV[4\x15aF\xFEWaG\xA0`\x02\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0aW\x06V[`@Q4\x81R3\x90\x7F\xE1\xFF\xFC\xC4\x92=\x04\xB5Y\xF4\xD2\x9A\x8B\xFCl\xDA\x04\xEB[\r<F\x07Q\xC2@,\\\\\xC9\x10\x9C\x90` \x01`@Q\x80\x91\x03\x90\xA2V[` \x83\x81\x1Cb\xFF\xFF\xFF\x16`\0\x90\x81R`\t\x82R`@\x80\x82 \x81Q`\x80\x81\x01\x83R\x81T`\x01`\x01`\xA0\x1B\x03\x80\x82\x16\x83R`\xFF`\x01`\xA0\x1B\x92\x83\x90\x04\x81\x16\x97\x84\x01\x97\x90\x97R`\x01\x90\x93\x01T\x92\x83\x16\x93\x82\x01\x93\x90\x93R\x91\x90\x04\x90\x92\x16``\x83\x01R\x90\x81\x90aHA\x85aR\xC3V[\x92P`\x01`\x01`\x80\x1B\x03\x85\x14aHqWaHna+\x86\x82` \x01Q`\xFF\x16\x87aS\x15\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x92P[aHz\x84aR\xC3V[\x91P`\x01`\x01`\x80\x1B\x03\x84\x14aH\xAAWaH\xA7a+\x86\x82``\x01Q`\xFF\x16\x86aS\x15\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x91P[P\x93P\x93\x91PPV[`\0\x82\x15\x80\x15aH\xCCWP\x83Q`\x01`\x01`\x80\x1B\x03\x16\x15\x15[\x15aH\xD9WP`\0a4<V[\x81\x15\x80\x15aH\xF3WP` \x84\x01Q`\x01`\x01`\x80\x1B\x03\x16\x15\x15[\x15aI\0WP`\0a4<V[`\0\x80`\0aI\x0E\x87aS\x7FV[aI%W\x86`@\x01Q`\x01`\x01`\x80\x1B\x03\x16aI/V[g\r\xE0\xB6\xB3\xA7d\0\0[\x87Q\x90\x91P`\x01`\x01`\x80\x1B\x03\x16\x15aI]W\x86QaIZ\x90\x87\x90\x83\x90`\x01`\x01`\x80\x1B\x03\x16aW\x9DV[\x92P[` \x87\x01Q`\x01`\x01`\x80\x1B\x03\x16\x15aI\x8EW` \x87\x01QaI\x8B\x90\x86\x90\x83\x90`\x01`\x01`\x80\x1B\x03\x16aW\x9DV[\x91P[aI\x9F\x82\x84\x11\x83\x85\x03\x02\x84\x03aR\xC3V[\x97\x96PPPPPPPV[`\0`\x01`\x01`\x7F\x1B\x03\x82\x11\x15aI\xC0W`\0\x80\xFD[P\x90V[`\0\x80`\x0F\x83\x90\x0B\x15aJ\xADW`\0\x80\x85`@\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90P`\0\x85`\x0F\x0B\x13\x15aJWWaI\xF9\x86aS\x7FV[\x15aJ\tWPg\r\xE0\xB6\xB3\xA7d\0\0[\x85Q`\x01`\x01`\x80\x1B\x03\x80\x87\x16\x93PaJ)\x91a+\x86\x91\x85\x91\x16\x84aW\xBCV[\x93PaJPa+\x86\x87` \x01Q`\x01`\x01`\x80\x1B\x03\x16\x83\x85aW\xBC\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x92PaJ\xAAV[aJ`\x85ap\xBDV[\x86Q`\x01`\x01`\x80\x1B\x03\x91\x82\x16\x93PaJ\x80\x91a+\x86\x91\x85\x91\x16\x84aW\x9DV[\x93PaJ\xA7a+\x86\x87` \x01Q`\x01`\x01`\x80\x1B\x03\x16\x83\x85aW\x9D\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x92P[PP[\x92P\x92\x90PV[`\x80\x81\x01Q`\x01`\x01`@\x1B\x03\x16`\0\x90\x81R`\n` \x90\x81R`@\x82 \x90\x83\x01Q\x90\x91\x90\x81\x90aJ\xE4\x90aR\xC3V[aJ\xF1\x85`@\x01QaR\xC3V[``\x86\x01Q`\x01\x86\x01T\x92\x94P\x90\x92P\x90`\x01`\x01`\x80\x1B\x03\x16`\0\x03aKPW`\0\x84Uc;\x9A\xCA\0`\x0F\x82\x90\x0B\x12\x15aK?W`@Qc\xCBm\xABu`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[aKMc;\x9A\xCA\0\x82atJV[\x90P[`\0\x81`\x0F\x0B\x13\x15aK\x90WaK\x8B\x85`\xA0\x01Q\x86`\x80\x01Q`\x01`\x01`@\x1B\x03\x16\x83`\x0F\x0B`@Q\x80` \x01`@R\x80`\0\x81RPaW\xEAV[aK\xB8V[aK\xB8\x85`\xA0\x01Q\x86`\x80\x01Q`\x01`\x01`@\x1B\x03\x16\x83`\x0F\x0BaK\xB3\x90at\x80V[aY\x92V[``\x85\x01Q`\x80\x86\x01Q`\x01`\x01`@\x1B\x03\x16`\0\x90\x81R`\n` R`@\x90 aK\xE2\x91aZ\x16V[`\xC0\x85\x01Q`\xE0\x86\x01Q``\x87\x01Q`\0`\x0F\x91\x90\x91\x0B\x12\x15aL\xB5WaL\x12\x82\x86`\x01`\x01`\x80\x1B\x03\x16aS,V[aL%\x81\x85`\x01`\x01`\x80\x1B\x03\x16aS,V[\x85T\x85\x90\x87\x90`\0\x90aLB\x90\x84\x90`\x01`\x01`\x80\x1B\x03\x16at\x9CV[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UP\x83\x86`\0\x01`\x10\x82\x82\x82\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\x80\x1B\x03\x16aL\x8C\x91\x90at\x9CV[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UPaMgV[aL\xC8\x82\x86`\x01`\x01`\x80\x1B\x03\x16aV#V[aL\xDB\x81\x85`\x01`\x01`\x80\x1B\x03\x16aV#V[\x85T\x85\x90\x87\x90`\0\x90aL\xF8\x90\x84\x90`\x01`\x01`\x80\x1B\x03\x16at\xC3V[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UP\x83\x86`\0\x01`\x10\x82\x82\x82\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\x80\x1B\x03\x16aMB\x91\x90at\xC3V[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UP[PPPPPPPV[`\0`\x02\x80\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15aM\xCAW` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11aM\xACW[PPPPP\x90P`\0\x81Q\x90P\x80`\0\x03aM\xE9Wa)T`\x02aZUV[\x80[`\0\x83aM\xF9`\x01\x84ao\x1BV[\x81Q\x81\x10aN\tWaN\tao\x05V[` \x02` \x01\x01Q\x90P`\0\x80aN,\x830`\x02aZ\x87\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x91P\x91P\x81`\0\x14\x15\x80aN?WP\x80\x15\x15[\x15aN\xCBW`\x0E`@Q\x80`\x80\x01`@R\x80\x84\x81R` \x01\x83\x81R` \x01aNg\x860aZ\xE9V[\x81R`\x01`\x01`\xA0\x1B\x03\x86\x81\x16` \x92\x83\x01R\x83T`\x01\x80\x82\x01\x86U`\0\x95\x86R\x94\x83\x90 \x84Q`\x04\x90\x92\x02\x01\x90\x81U\x91\x83\x01Q\x93\x82\x01\x93\x90\x93U`@\x82\x01Q`\x02\x82\x01U``\x90\x91\x01Q`\x03\x90\x91\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x91\x90\x92\x16\x17\x90U[`\x04\x80T\x80aN\xDCWaN\xDCat\xE3V[`\0\x82\x81R` \x81 \x82\x01`\0\x19\x90\x81\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90U\x91\x82\x01\x90\x92U\x85\x01\x94\x90\x03`\x01\x01\x92PaM\xEB\x91PPW`\0`\x0E\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15aO\x93W`\0\x84\x81R` \x90\x81\x90 `@\x80Q`\x80\x81\x01\x82R`\x04\x86\x02\x90\x92\x01\x80T\x83R`\x01\x80\x82\x01T\x84\x86\x01R`\x02\x82\x01T\x92\x84\x01\x92\x90\x92R`\x03\x01T`\x01`\x01`\xA0\x1B\x03\x16``\x83\x01R\x90\x83R\x90\x92\x01\x91\x01aO6V[PP\x82Q\x92\x93PPP[\x80\x15aQ\x8EW`\0aO\xB0`\x01\x83ao\x1BV[\x90P`\0\x83\x82\x81Q\x81\x10aO\xC6WaO\xC6ao\x05V[` \x02` \x01\x01Q``\x01Q\x90P`\0\x80\x85\x84\x81Q\x81\x10aO\xE9WaO\xE9ao\x05V[` \x02` \x01\x01Q`\0\x01Q\x86\x85\x81Q\x81\x10aP\x07WaP\x07ao\x05V[` \x02` \x01\x01Q` \x01Q\x91P\x91P`\0\x82\x11\x15aQ\x17W`\0\x86\x85\x81Q\x81\x10aP4WaP4ao\x05V[` \x02` \x01\x01Q`@\x01Q\x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x03aP\xABWaP\xA6\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x003\x85a[\xC9V[aP\xB6V[aP\xB6\x843\x85a\\pV[`\0aP\xC2\x850aZ\xE9V[\x90P`\0aP\xD0\x85\x84ao\x1BV[\x90P\x80\x82\x10\x15aQ\x0FW\x85aP\xE5\x82\x84at\xF9V[`@QcU\x13N\xF1`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x04\x83\x01R`$\x82\x01R`D\x01a\x18\xC2V[PPPaQ~V[\x80\x15aQ~W`\0\x86\x85\x81Q\x81\x10aQ1WaQ1ao\x05V[` \x02` \x01\x01Q`@\x01Q\x90PaQK\x8430\x85a\\\xCDV[`\0aQW\x850aZ\xE9V[\x90P`\0aQe\x84\x84ao.V[\x90P\x80\x82\x10\x15aQzW\x85aP\xE5\x82\x84at\xF9V[PPP[\x84`\x01\x90\x03\x94PPPPPaO\x9DV[aQ\x98`\x02aZUV[aQ\xA4`\x0E`\0a`\xD2V[PPPPPV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x81\x81R` \x85\x81R`@\x80\x83 T\x81Qc1<\xE5g`\xE0\x1B\x81R\x91Q\x93\x94\x90\x93\x85\x93aR\x81\x93\x86\x93\x92c1<\xE5g\x92`\x04\x80\x84\x01\x93\x91\x92\x91\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15aRAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` av\xB8\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15aRUW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aRy\x91\x90ar\xA1V[`\xFF\x16aR\xD5V[\x90P`\0aR\x8F\x86\x86aZ\xE9V[\x90P`\x01`\x01`\xFF\x1B\x03\x82\x11\x15aR\xA5W`\0\x80\xFD[`\x01`\x01`\xFF\x1B\x03\x81\x11\x15aR\xB9W`\0\x80\xFD[aI\x9F\x82\x82at\xF9V[`\0`\x01`\x80\x1B\x82\x10aI\xC0W`\0\x80\xFD[`\0\x82`\0\x03aR\xE7WP`\0a\x12]V[`\0aR\xF2\x83aV\xEEV[\x90P\x80aS\0`\x01\x86ao\x1BV[aS\n\x91\x90au\x19V[a8*\x90`\x01ao.V[`\0\x80aS!\x83aV\xEEV[\x93\x90\x93\x02\x93\x92PPPV[aS8`\x02\x83\x83a].V[\x81`\x01`\x01`\xA0\x1B\x03\x16\x7F\x1Cq\x1E\xCA\x8D\x0BiK\xBC\xB0\xA1Db\xA7\0b\"\xE7!\x95K,_\xF7\x98\xF6\x06\x81~\xB1\x102\x82`@QaSs\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2PPV[`\0aS\x94\x82``\x01Qc\xFF\xFF\xFF\xFF\x16\x15\x15\x90V[\x80\x15a\x12]WPP`@\x01Q`\x01`\x01`\x80\x1B\x03\x16\x15\x90V[aS\xB6\x81a]\xAFV[\x82`\x01\x01`\x10a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPPV[aS\xE4a`gV[a\x01\0\x82\x01Q`@\x83\x01QaS\xFB\x91`\xFF\x16aS\x15V[`@\x83\x01Ra\x01\0\x82\x01Q``\x83\x01QaT\x17\x91`\xFF\x16aS\x15V[``\x83\x01Ra\x01\0\x82\x01Q`\x80\x83\x01QaT3\x91`\xFF\x16aS\x15V[`\x80\x83\x01Ra\x01 \x82\x01Q`\xA0\x83\x01QaTO\x91`\xFF\x16aS\x15V[`\xA0\x83\x01RP\x90V[`\0\x80\x80\x80aTh\x89\x87\x87a]\xC2V[\x90\x94P\x92PaTz\x89\x89\x89\x87\x87a^\x08V[\x94\x9A\x93\x99P\x97P\x92\x95P\x90\x93PPPPV[\x82\x15aU.WaT\x9B\x82aR\xC3V[\x84T\x85\x90`\0\x90aT\xB6\x90\x84\x90`\x01`\x01`\x80\x1B\x03\x16at\xC3V[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UPaT\xE3\x81aR\xC3V[\x84T\x85\x90`\x10\x90aU\x05\x90\x84\x90`\x01`\x80\x1B\x90\x04`\x01`\x01`\x80\x1B\x03\x16at\x9CV[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UPaU\xC6V[aU7\x81aR\xC3V[\x84T\x85\x90`\0\x90aUR\x90\x84\x90`\x01`\x01`\x80\x1B\x03\x16at\x9CV[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UPaU\x7F\x82aR\xC3V[\x84T\x85\x90`\x10\x90aU\xA1\x90\x84\x90`\x01`\x80\x1B\x90\x04`\x01`\x01`\x80\x1B\x03\x16at\xC3V[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UP[\x83T`\x01`\x01`\x80\x1B\x03\x16`\0\x03aU\xF1W`@Qc\x17O\xD4\x1B`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x83T`\x01`\x80\x1B\x90\x04`\x01`\x01`\x80\x1B\x03\x16`\0\x03a-rW`@Qc(i\xC5\xF3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[aV/`\x02\x83\x83a_@V[\x81`\x01`\x01`\xA0\x1B\x03\x16\x7F\x80\xB2\x17H\xC7\x87\xC5.\x87\xA6\xB2\"\x01\x1E\n\x0E\xD0\xF9\xCC \x15\xF0\xCE\xD4gHd-\xC6.\xE9\xF8\x82`@QaSs\x91\x81R` \x01\x90V[aVra`gV[a\x01\0\x82\x01Q`@\x83\x01QaV\x89\x91`\xFF\x16aGYV[`@\x83\x01Ra\x01\0\x82\x01Q``\x83\x01QaV\xA5\x91`\xFF\x16aGYV[``\x83\x01Ra\x01\0\x82\x01Q`\x80\x83\x01QaV\xC1\x91`\xFF\x16aGYV[`\x80\x83\x01Ra\x01 \x82\x01Q`\xA0\x83\x01QaTO\x91`\xFF\x16aGYV[`\0b\x01\0\0\x82\x10aI\xC0W`\0\x80\xFD[`\0aV\xFB\x82`\x12ao\x1BV[a\x12]\x90`\nav\x1FV[aW\x10\x82\x82a_|V[\x80`\x01`\x01`\xA0\x1B\x03\x16c\xD0\xE3\r\xB04`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15aW\x89W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` av\xB8\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15aMgW=`\0\x80>=`\0\xFD[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16aW\xB5W`\0\x80\xFD[\x04\x92\x91PPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16aW\xD4W`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81R` \x81\x81R`@\x80\x83 \x86\x84R\x90\x91R\x81 \x80T\x84\x92\x90aX\x1B\x90\x84\x90ao.V[\x90\x91UPP`@\x80Q\x84\x81R` \x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x86\x16\x91`\0\x913\x91\x7F\xC3\xD5\x81h\xC5\xAEs\x97s\x1D\x06=[\xBF=exTBsC\xF4\xC0\x83$\x0Fz\xAC\xAA-\x0Fb\x91\x01`@Q\x80\x91\x03\x90\xA4`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15aYlW`@Qc\xF2:na`\xE0\x1B\x80\x82R\x90`\x01`\x01`\xA0\x1B\x03\x86\x16\x90c\xF2:na\x90aX\xB0\x903\x90`\0\x90\x89\x90\x89\x90\x89\x90`\x04\x01av+V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15aY\x08W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` av\xB8\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15aY\x1CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aY@\x91\x90apHV[`\x01`\x01`\xE0\x1B\x03\x19\x16\x14aYgW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x18\xC2\x90aphV[a-rV[`\x01`\x01`\xA0\x1B\x03\x84\x16a-rW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x18\xC2\x90ap\x92V[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R` \x81\x81R`@\x80\x83 \x85\x84R\x90\x91R\x81 \x80T\x83\x92\x90aY\xC3\x90\x84\x90ao\x1BV[\x90\x91UPP`@\x80Q\x83\x81R` \x81\x01\x83\x90R`\0\x91`\x01`\x01`\xA0\x1B\x03\x86\x16\x913\x91\x7F\xC3\xD5\x81h\xC5\xAEs\x97s\x1D\x06=[\xBF=exTBsC\xF4\xC0\x83$\x0Fz\xAC\xAA-\x0Fb\x91\x01`@Q\x80\x91\x03\x90\xA4PPPV[`\x01\x82\x01TaZ.\x90`\x01`\x01`\x80\x1B\x03\x16\x82a`\nV[`\x01\x92\x90\x92\x01\x80T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x90\x93\x16\x92\x90\x92\x17\x90\x91UPV[`\x02\x81\x01T\x15aZgWaZgaveV[`\x03\x81\x01\x80T`\xFF\x19\x16`\x01\x17\x90UaZ\x84`\x02\x82\x01`\0a`\xF3V[PV[`\0\x80\x80aZ\x96\x86\x86\x86aQ\xABV[\x90P`\0\x81\x13\x15aZ\xA9W\x80\x92PaZ\xBEV[`\0\x81\x12\x15aZ\xBEWaZ\xBB\x81at\x80V[\x91P[P`\x01`\x01`\xA0\x1B\x03\x90\x93\x16`\0\x90\x81R`\x01\x90\x94\x01` R`@\x90\x93 \x80T`\xFF\x19\x16\x90UP\x90\x91V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`$\x80\x84\x01\x91\x90\x91R\x83Q\x80\x84\x03\x90\x91\x01\x81R`D\x90\x92\x01\x83R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16cp\xA0\x821`\xE0\x1B\x17\x90R\x91Q`\0\x92\x83\x92\x83\x92\x91\x87\x16\x91a[C\x91\x90av{V[`\0`@Q\x80\x83\x03\x81\x85Z\xFA\x91PP=\x80`\0\x81\x14a[~W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a[\x83V[``\x91P[P\x91P\x91P\x81\x15\x80a[\x97WP\x80Q` \x14\x15[\x15a[\xB5W`@Qc\xC5.>\xFF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x80` \x01\x90Q\x81\x01\x90a\x14\"\x91\x90anDV[`@Qc.\x1A}M`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90c.\x1A}M\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\\IW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` av\xB8\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\\]W=`\0\x80>=`\0\xFD[PPPPa\\k\x82\x82a`;V[PPPV[`\0`@Qc\xA9\x05\x9C\xBB`\xE0\x1B`\0R\x83`\x04R\x82`$R` `\0`D`\0\x80\x89Z\xF1=\x15`\x1F=\x11`\x01`\0Q\x14\x16\x17\x16\x91P`\0``R\x80`@RP\x80a-rW`@Qc:\xCB=?`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0`@Qc#\xB8r\xDD`\xE0\x1B`\0R\x84`\x04R\x83`$R\x82`DR` `\0`d`\0\x80\x8AZ\xF1=\x15`\x1F=\x11`\x01`\0Q\x14\x16\x17\x16\x91P`\0``R\x80`@RP\x80aQ\xA4W`@Qcn\x89\xEC\xA5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R` \x84\x90R`@\x90 T\x80\x82\x11\x15a]rW`@Qc1Rv\xC9`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x81\x01\x83\x90R`D\x01a\x18\xC2V[a]|\x84\x84a_|V[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R` \x85\x90R`@\x81 \x80T\x84\x92\x90a]\xA4\x90\x84\x90ao\x1BV[\x90\x91UPPPPPPV[`\0d\x01\0\0\0\0\x82\x10aI\xC0W`\0\x80\xFD[\x82Q`\0\x90\x81\x90a]\xDF\x90`\x01`\x01`\x80\x1B\x03\x16\x85a'\x10aW\xBCV[\x91P\x82\x15a^\0Wa]\xF1\x83\x83au\x19V[\x90Pa]\xFD\x81\x83ao\x1BV[\x91P[\x93P\x93\x91PPV[`\0\x80`\0\x87`\x80\x01Qa^\x1CW\x85a^\x1EV[\x86[\x90P`\0\x88`\x80\x01Qa^1W\x87a^3V[\x86[\x89Q\x90\x91Pa^K\x90`\x01`\x01`\x80\x1B\x03\x16\x83ao.V[\x91Pa^W\x86\x83ao\x1BV[\x91P\x81\x85\x11\x15a^zW`@QcvG\x0F\xE7`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a^\x84\x85\x83ao\x1BV[\x91P\x80\x89` \x01Q`\x01`\x01`\x80\x1B\x03\x16\x11\x15a^\xB4W`@Qc\x86j\x03+`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x89\x01Qa^\xCC\x90`\x01`\x01`\x80\x1B\x03\x16\x82ao\x1BV[\x90P\x88`\x80\x01Qa^\xDDW\x80a^\xDFV[\x81[\x93P\x88`\x80\x01Qa^\xF0W\x81a^\xF2V[\x80[\x92P\x83\x88\x03a_\x14W`@Qc9;xE`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82\x87\x03a_4W`@Qc\x1F\xB0\xB7\xDD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PP\x95P\x95\x93PPPPV[a_J\x83\x83a_|V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R` \x84\x90R`@\x81 \x80T\x83\x92\x90a_r\x90\x84\x90ao.V[\x90\x91UPPPPPV[`\x03\x82\x01T`\xFF\x16\x15a_\x96W`\x03\x82\x01\x80T`\xFF\x19\x16\x90U[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x01\x83\x01` R`@\x90 T`\xFF\x16a)TW`\x02\x82\x01\x80T`\x01\x80\x82\x01\x83U`\0\x92\x83R` \x80\x84 \x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x95\x90\x95\x16\x94\x85\x17\x90U\x92\x82R\x92\x82\x01\x90\x92R`@\x90\x91 \x80T`\xFF\x19\x16\x90\x91\x17\x90UV[`\0\x80\x82`\x0F\x0B\x13\x15a`(Wa`!\x82\x84at\xC3V[\x90Pa\x12]V[a`1\x82ap\xBDV[a4<\x90\x84at\x9CV[`\0\x80`\0\x80`\0\x85\x87Z\xF1\x90P\x80a\\kW`@Qc5e\x94\xAB`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Q\x80a\x01@\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\0`\xFF\x16\x81R` \x01`\0`\xFF\x16\x81RP\x90V[P\x80T`\0\x82U`\x04\x02\x90`\0R` `\0 \x90\x81\x01\x90aZ\x84\x91\x90aa\x11V[P\x80T`\0\x82U\x90`\0R` `\0 \x90\x81\x01\x90aZ\x84\x91\x90aaEV[[\x80\x82\x11\x15aI\xC0W`\0\x80\x82U`\x01\x82\x01\x81\x90U`\x02\x82\x01U`\x03\x81\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90U`\x04\x01aa\x12V[[\x80\x82\x11\x15aI\xC0W`\0\x81U`\x01\x01aaFV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14aZ\x84W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15ab%Wab%aaZV[\x825ab0\x81aa\xFAV[\x94` \x93\x90\x93\x015\x93PPPV[`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14aZ\x84W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15abiWabiaaZV[\x815a4<\x81ab>V[`\0` \x82\x84\x03\x12\x15ab\x89Wab\x89aaZV[P5\x91\x90PV[`\0[\x83\x81\x10\x15ab\xABW\x81\x81\x01Q\x83\x82\x01R` \x01ab\x93V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Rab\xCC\x81` \x86\x01` \x86\x01ab\x90V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a4<` \x83\x01\x84ab\xB4V[`\x01`\x01`@\x1B\x03\x81\x16\x81\x14aZ\x84W`\0\x80\xFD[\x80\x15\x15\x81\x14aZ\x84W`\0\x80\xFD[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15ac/Wac/aaZV[\x845ac:\x81ab\xF3V[\x93P` \x85\x015acJ\x81ac\x08V[\x92P`@\x85\x015\x91P``\x85\x015aca\x81aa\xFAV[\x93\x96\x92\x95P\x90\x93PPV[\x805b\xFF\xFF\xFF\x81\x16\x81\x14ac\x7FW`\0\x80\xFD[\x91\x90PV[\x805a\xFF\xFF\x81\x16\x81\x14ac\x7FW`\0\x80\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x81\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x0ENL/$\r\x8C\xAD\xCC\xEE\x8D`\xAB\x1B`d\x82\x01R`\x84\x81\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rjrray stride`\xA8\x1B`d\x82\x01R`\x84\x81\xFD[`\0\x80\x83`\x1F\x84\x01\x12ad\xB6Wad\xB6ac\x96V[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15ad\xD0Wad\xD0ac\xEFV[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15aJ\xADWaJ\xADadHV[`\0\x80`\0\x80`\0\x80`\0\x80`\0a\x01\0\x8A\x8C\x03\x12\x15ae\rWae\raaZV[ae\x16\x8AaclV[\x98P` \x8A\x015\x97P`@\x8A\x015\x96Pae2``\x8B\x01ac\x84V[\x95Pae@`\x80\x8B\x01ac\x84V[\x94P`\xA0\x8A\x015aeP\x81aa\xFAV[\x93P`\xC0\x8A\x015ae`\x81aa\xFAV[\x92P`\xE0\x8A\x015`\x01`\x01`@\x1B\x03\x81\x11\x15ae~Wae~aa\xAAV[ae\x8A\x8C\x82\x8D\x01ad\xA1V[\x91P\x80\x93PP\x80\x91PP\x92\x95\x98P\x92\x95\x98P\x92\x95\x98V[`\0` \x82\x84\x03\x12\x15ae\xB6Wae\xB6aaZV[\x815a4<\x81ab\xF3V[`\0\x80\x83`\x1F\x84\x01\x12ae\xD6Wae\xD6ac\x96V[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15ae\xF0Wae\xF0ac\xEFV[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15aJ\xADWaJ\xADadHV[`\0\x80`\0\x80`\0\x80`\0\x80`\xA0\x89\x8B\x03\x12\x15af-Waf-aaZV[\x885af8\x81aa\xFAV[\x97P` \x89\x015afH\x81aa\xFAV[\x96P`@\x89\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15afgWafgaa\xAAV[afs\x8C\x83\x8D\x01ae\xC1V[\x90\x98P\x96P``\x8B\x015\x91P\x80\x82\x11\x15af\x8FWaf\x8Faa\xAAV[af\x9B\x8C\x83\x8D\x01ae\xC1V[\x90\x96P\x94P`\x80\x8B\x015\x91P\x80\x82\x11\x15af\xB7Waf\xB7aa\xAAV[Paf\xC4\x8B\x82\x8C\x01ad\xA1V[\x99\x9C\x98\x9BP\x96\x99P\x94\x97\x93\x96\x92\x95\x94PPPV[`\x01`\x01`\x80\x1B\x03\x81\x16\x81\x14aZ\x84W`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15ag\tWag\taaZV[\x865ag\x14\x81ac\x08V[\x95P` \x87\x015ag$\x81aa\xFAV[\x94P`@\x87\x015ag4\x81ab\xF3V[\x93P``\x87\x015agD\x81af\xD8V[\x92P`\x80\x87\x015agT\x81af\xD8V[\x91P`\xA0\x87\x015agd\x81af\xD8V[\x80\x91PP\x92\x95P\x92\x95P\x92\x95V[`\0\x80`@\x83\x85\x03\x12\x15ag\x88Wag\x88aaZV[\x825ag\x93\x81aa\xFAV[\x91P` \x83\x015ag\xA3\x81aa\xFAV[\x80\x91PP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15ag\xC3Wag\xC3aaZV[\x815a4<\x81aa\xFAV[`\0\x80`\0\x80`@\x85\x87\x03\x12\x15ag\xE7Wag\xE7aaZV[\x845`\x01`\x01`@\x1B\x03\x80\x82\x11\x15ah\x01Wah\x01aa\xAAV[ah\r\x88\x83\x89\x01ae\xC1V[\x90\x96P\x94P` \x87\x015\x91P\x80\x82\x11\x15ah)Wah)aa\xAAV[Pah6\x87\x82\x88\x01ae\xC1V[\x95\x98\x94\x97P\x95PPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15ahzW\x83Q\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01ah^V[P\x90\x96\x95PPPPPPV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15ah\xA1Wah\xA1aaZV[\x855ah\xAC\x81ac\x08V[\x94P` \x86\x015ah\xBC\x81ab\xF3V[\x93P`@\x86\x015ah\xCC\x81af\xD8V[\x92P``\x86\x015ah\xDC\x81af\xD8V[\x91P`\x80\x86\x015ah\xEC\x81af\xD8V[\x80\x91PP\x92\x95P\x92\x95\x90\x93PV[`\0` \x82\x84\x03\x12\x15ai\x0FWai\x0FaaZV[a4<\x82aclV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\xA0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aiPWaiPai\x18V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15ai~Wai~ai\x18V[`@R\x91\x90PV[`\0`\xA0\x82\x84\x03\x12\x15ai\xE4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FABI decoding: struct data too sh`D\x82\x01Rb\x1B\xDC\x9D`\xEA\x1B`d\x82\x01R`\x84\x81\xFD[ai\xECai.V[\x90P\x815ai\xF9\x81af\xD8V[\x81R` \x82\x015aj\t\x81af\xD8V[` \x82\x01R`@\x82\x015aj\x1C\x81ac\x08V[`@\x82\x01R``\x82\x015aj/\x81ab\xF3V[``\x82\x01R`\x80\x82\x015ajB\x81ac\x08V[`\x80\x82\x01R\x92\x91PPV[`\0\x80`\0`\xE0\x84\x86\x03\x12\x15ajeWajeaaZV[ajo\x85\x85ai\x86V[\x92P`\xA0\x84\x015\x91P`\xC0\x84\x015aj\x86\x81aa\xFAV[\x80\x91PP\x92P\x92P\x92V[`\0\x80`@\x83\x85\x03\x12\x15aj\xA7Waj\xA7aaZV[\x825aj\xB2\x81ab\xF3V[\x91P` \x83\x015`\x0F\x81\x90\x0B\x81\x14ag\xA3W`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15aj\xE1Waj\xE1aaZV[\x835aj\xEC\x81ab\xF3V[\x92Paj\xFA` \x85\x01ac\x84V[\x91Pak\x08`@\x85\x01ac\x84V[\x90P\x92P\x92P\x92V[`\0\x80`@\x83\x85\x03\x12\x15ak'Wak'aaZV[\x825ak2\x81aa\xFAV[\x91P` \x83\x015ag\xA3\x81ac\x08V[`\0\x80` \x83\x85\x03\x12\x15akXWakXaaZV[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15akqWakqaa\xAAV[ak}\x85\x82\x86\x01ae\xC1V[\x90\x96\x90\x95P\x93PPPPV[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x86\x01\x91P`@\x81`\x05\x1B\x87\x01\x01\x92P\x83\x87\x01`\0[\x82\x81\x10\x15ak\xDEW`?\x19\x88\x86\x03\x01\x84Rak\xCC\x85\x83Qab\xB4V[\x94P\x92\x85\x01\x92\x90\x85\x01\x90`\x01\x01ak\xB0V[P\x92\x97\x96PPPPPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15al\x03Wal\x03aaZV[\x835al\x0E\x81ab\xF3V[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15al;Wal;aaZV[\x835alF\x81ab\xF3V[\x92P` \x84\x015alV\x81ac\x08V[\x91P`@\x84\x015aj\x86\x81aa\xFAV[`\x01`\x01`\x80\x1B\x03\x80\x82Q\x16\x83R\x80` \x83\x01Q\x16` \x84\x01RP`@\x81\x01Q\x15\x15`@\x83\x01R`\x01`\x01`@\x1B\x03``\x82\x01Q\x16``\x83\x01R`\x80\x81\x01Q\x15\x15`\x80\x83\x01RPPV[`\xA0\x81\x01a\x12]\x82\x84alfV[`\0\x80`\0\x80`\0\x80`\xA0\x87\x89\x03\x12\x15al\xDAWal\xDAaaZV[\x865al\xE5\x81aa\xFAV[\x95P` \x87\x015al\xF5\x81aa\xFAV[\x94P`@\x87\x015\x93P``\x87\x015\x92P`\x80\x87\x015`\x01`\x01`@\x1B\x03\x81\x11\x15am!Wam!aa\xAAV[am-\x89\x82\x8A\x01ad\xA1V[\x97\x9A\x96\x99P\x94\x97P\x92\x95\x93\x94\x92PPPV[`\0`\xA0\x82\x84\x03\x12\x15amTWamTaaZV[P\x91\x90PV[`\0` \x80\x83\x85\x03\x12\x15ampWampaaZV[\x82Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15am\x8AWam\x8Aaa\xAAV[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12am\xA1Wam\xA1ac\x96V[\x81Q\x81\x81\x11\x15am\xB3Wam\xB3ai\x18V[am\xC5`\x1F\x82\x01`\x1F\x19\x16\x85\x01aiVV[\x91P\x80\x82R\x86\x84\x82\x85\x01\x01\x11\x15an+W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x85\x90R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R`\x84\x81\xFD[an:\x81\x85\x84\x01\x86\x86\x01ab\x90V[P\x95\x94PPPPPV[`\0` \x82\x84\x03\x12\x15anYWanYaaZV[PQ\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x81\x03an\x8FWan\x8Fan`V[`\x01\x01\x93\x92PPPV[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[`\x01`\x01`@\x1B\x03\x84\x16\x81R`@` \x82\x01R`\0a\x14\"`@\x83\x01\x84\x86an\x99V[`\0` \x82\x84\x03\x12\x15an\xFAWan\xFAaaZV[\x81Qa4<\x81ac\x08V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x12]Wa\x12]an`V[\x80\x82\x01\x80\x82\x11\x15a\x12]Wa\x12]an`V[\x81\x83R`\0`\x01`\x01`\xFB\x1B\x03\x83\x11\x15ao\xA4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FABI encoding: array data too lon`D\x82\x01R`g`\xF8\x1B`d\x82\x01R`\x84\x81\xFD[\x82`\x05\x1B\x80\x83` \x87\x017\x93\x90\x93\x01` \x01\x93\x92PPPV[`@\x81R`\0ao\xD1`@\x83\x01\x86\x88aoAV[\x82\x81\x03` \x84\x01RaI\x9F\x81\x85\x87aoAV[`\x01`\x01`\xA0\x1B\x03\x89\x81\x16\x82R\x88\x16` \x82\x01R`\xA0`@\x82\x01\x81\x90R`\0\x90ap\x11\x90\x83\x01\x88\x8AaoAV[\x82\x81\x03``\x84\x01Rap$\x81\x87\x89aoAV[\x90P\x82\x81\x03`\x80\x84\x01Rap9\x81\x85\x87an\x99V[\x9B\x9APPPPPPPPPPPV[`\0` \x82\x84\x03\x12\x15ap]Wap]aaZV[\x81Qa4<\x81ab>V[` \x80\x82R`\x10\x90\x82\x01Ro\x15S\x94\xD0Q\x91W\xD4\x91P\xD2T\x12QS\x95`\x82\x1B`@\x82\x01R``\x01\x90V[` \x80\x82R`\x11\x90\x82\x01Rp\x12S\x95\x90S\x12Q\x17\xD4\x91P\xD2T\x12QS\x95`z\x1B`@\x82\x01R``\x01\x90V[`\0\x81`\x0F\x0B`\x01`\x01`\x7F\x1B\x03\x19\x81\x03ap\xDAWap\xDAan`V[`\0\x03\x92\x91PPV[`\0` \x82\x84\x03\x12\x15ap\xF8Wap\xF8aaZV[\x81Qa4<\x81aa\xFAV[`\xE0\x81\x01aq\x11\x82\x86alfV[`\xA0\x82\x01\x93\x90\x93R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16`\xC0\x90\x91\x01R\x91\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15aqHWaqHaaZV[\x83QaqS\x81ac\x08V[` \x85\x01Q`@\x90\x95\x01Q\x90\x96\x94\x95P\x93\x92PPPV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FCalldata tail too short\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x81\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12ar\x06W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FInvalid calldata tail offset\0\0\0\0`D\x82\x01R`d\x81\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15ar`W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FInvalid calldata tail length\0\0\0\0`D\x82\x01R`d\x81\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15aJ\xADWaJ\xADaqjV[\x81\x83\x827`\0\x91\x01\x90\x81R\x91\x90PV[`\0`\x01\x82\x01ar\x9AWar\x9Aan`V[P`\x01\x01\x90V[`\0` \x82\x84\x03\x12\x15ar\xB6War\xB6aaZV[\x81Q`\xFF\x81\x16\x81\x14a4<W`\0\x80\xFD[`\0b\xFF\xFF\xFF\x80\x83\x16\x81\x81\x03an\x8FWan\x8Fan`V[`\0`\xA0\x82\x84\x03\x12\x15ar\xF4War\xF4aaZV[ar\xFCai.V[\x82Qas\x07\x81af\xD8V[\x81R` \x83\x01Qas\x17\x81af\xD8V[` \x82\x01R`@\x83\x01Qas*\x81ac\x08V[`@\x82\x01R``\x83\x01Qas=\x81ab\xF3V[``\x82\x01R`\x80\x83\x01QasP\x81ac\x08V[`\x80\x82\x01R\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x87\x81\x16\x82R\x86\x16` \x82\x01R`@\x81\x01\x85\x90R``\x81\x01\x84\x90R`\xA0`\x80\x82\x01\x81\x90R`\0\x90as\x97\x90\x83\x01\x84\x86an\x99V[\x98\x97PPPPPPPPV[`\0` \x82\x84\x03\x12\x15as\xB8Was\xB8aaZV[\x815a4<\x81af\xD8V[`\0` \x82\x84\x03\x12\x15as\xD8Was\xD8aaZV[\x815a4<\x81ac\x08V[`\0\x80`@\x83\x85\x03\x12\x15as\xF9Was\xF9aaZV[\x82Qat\x04\x81ac\x08V[` \x93\x90\x93\x01Q\x92\x94\x92\x93PPPV[`\0`\xA0\x82\x84\x03\x12\x15at)Wat)aaZV[a4<\x83\x83ai\x86V[`\0\x81atBWatBan`V[P`\0\x19\x01\x90V[`\x0F\x82\x81\x0B\x90\x82\x90\x0B\x03o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x12`\x01`\x01`\x7F\x1B\x03\x82\x13\x17\x15a\x12]Wa\x12]an`V[`\0`\x01`\xFF\x1B\x82\x01at\x95Wat\x95an`V[P`\0\x03\x90V[`\x01`\x01`\x80\x1B\x03\x82\x81\x16\x82\x82\x16\x03\x90\x80\x82\x11\x15at\xBCWat\xBCan`V[P\x92\x91PPV[`\x01`\x01`\x80\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x80\x82\x11\x15at\xBCWat\xBCan`V[cNH{q`\xE0\x1B`\0R`1`\x04R`$`\0\xFD[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15at\xBCWat\xBCan`V[`\0\x82au6WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[`\x01\x81\x81[\x80\x85\x11\x15auvW\x81`\0\x19\x04\x82\x11\x15au\\Wau\\an`V[\x80\x85\x16\x15auiW\x91\x81\x02\x91[\x93\x84\x1C\x93\x90\x80\x02\x90au@V[P\x92P\x92\x90PV[`\0\x82au\x8DWP`\x01a\x12]V[\x81au\x9AWP`\0a\x12]V[\x81`\x01\x81\x14au\xB0W`\x02\x81\x14au\xBAWau\xD6V[`\x01\x91PPa\x12]V[`\xFF\x84\x11\x15au\xCBWau\xCBan`V[PP`\x01\x82\x1Ba\x12]V[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15au\xF9WP\x81\x81\na\x12]V[av\x03\x83\x83au;V[\x80`\0\x19\x04\x82\x11\x15av\x17Wav\x17an`V[\x02\x93\x92PPPV[`\0a4<\x83\x83au~V[`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x82R\x85\x16` \x82\x01R`@\x81\x01\x84\x90R``\x81\x01\x83\x90R`\xA0`\x80\x82\x01\x81\x90R`\0\x90aI\x9F\x90\x83\x01\x84ab\xB4V[cNH{q`\xE0\x1B`\0R`\x01`\x04R`$`\0\xFD[`\0\x82Qav\x8D\x81\x84` \x87\x01ab\x90V[\x91\x90\x91\x01\x92\x91PPV\xFEEther sent to non-payable functiTarget contract does not contain";
    /// The deployed bytecode of the contract.
    pub static PORTFOLIO_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct Portfolio<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Portfolio<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Portfolio<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Portfolio<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Portfolio<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Portfolio)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Portfolio<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    PORTFOLIO_ABI.clone(),
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
                PORTFOLIO_ABI.clone(),
                PORTFOLIO_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `POSITION_RENDERER` (0xb0c3a950) function
        pub fn position_renderer(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([176, 195, 169, 80], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `REGISTRY` (0x06433b1b) function
        pub fn registry(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([6, 67, 59, 27], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `VERSION` (0xffa1ad74) function
        pub fn version(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([255, 161, 173, 116], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `WETH` (0xad5c4648) function
        pub fn weth(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([173, 92, 70, 72], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `allocate` (0x2f9e38e2) function
        pub fn allocate(
            &self,
            use_max: bool,
            recipient: ::ethers::core::types::Address,
            pool_id: u64,
            delta_liquidity: u128,
            max_delta_asset: u128,
            max_delta_quote: u128,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash(
                    [47, 158, 56, 226],
                    (
                        use_max,
                        recipient,
                        pool_id,
                        delta_liquidity,
                        max_delta_asset,
                        max_delta_quote,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `balanceOf` (0x00fdd58e) function
        pub fn balance_of(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([0, 253, 213, 142], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `balanceOfBatch` (0x4e1273f4) function
        pub fn balance_of_batch(
            &self,
            owners: ::std::vec::Vec<::ethers::core::types::Address>,
            ids: ::std::vec::Vec<::ethers::core::types::U256>,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::U256>,
        > {
            self.0
                .method_hash([78, 18, 115, 244], (owners, ids))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `changeParameters` (0x8a678967) function
        pub fn change_parameters(
            &self,
            pool_id: u64,
            priority_fee: u16,
            fee: u16,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([138, 103, 137, 103], (pool_id, priority_fee, fee))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `claimFee` (0xdda40797) function
        pub fn claim_fee(
            &self,
            token: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([221, 164, 7, 151], (token, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `createPair` (0xc9c65396) function
        pub fn create_pair(
            &self,
            asset: ::ethers::core::types::Address,
            quote: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([201, 198, 83, 150], (asset, quote))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `createPool` (0x267a0cfe) function
        pub fn create_pool(
            &self,
            pair_id: u32,
            reserve_x_per_wad: ::ethers::core::types::U256,
            reserve_y_per_wad: ::ethers::core::types::U256,
            fee_basis_points: u16,
            priority_fee_basis_points: u16,
            controller: ::ethers::core::types::Address,
            strategy: ::ethers::core::types::Address,
            strategy_args: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash(
                    [38, 122, 12, 254],
                    (
                        pair_id,
                        reserve_x_per_wad,
                        reserve_y_per_wad,
                        fee_basis_points,
                        priority_fee_basis_points,
                        controller,
                        strategy,
                        strategy_args,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deallocate` (0x5bc55464) function
        pub fn deallocate(
            &self,
            use_max: bool,
            pool_id: u64,
            delta_liquidity: u128,
            min_delta_asset: u128,
            min_delta_quote: u128,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash(
                    [91, 197, 84, 100],
                    (use_max, pool_id, delta_liquidity, min_delta_asset, min_delta_quote),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getAmountOut` (0x19057807) function
        pub fn get_amount_out(
            &self,
            pool_id: u64,
            sell_asset: bool,
            amount_in: ::ethers::core::types::U256,
            swapper: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([25, 5, 120, 7], (pool_id, sell_asset, amount_in, swapper))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getInvariant` (0x39434d5a) function
        pub fn get_invariant(
            &self,
            pool_id: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([57, 67, 77, 90], pool_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getLiquidityDeltas` (0x8992f20a) function
        pub fn get_liquidity_deltas(
            &self,
            pool_id: u64,
            delta_liquidity: i128,
        ) -> ::ethers::contract::builders::ContractCall<M, (u128, u128)> {
            self.0
                .method_hash([137, 146, 242, 10], (pool_id, delta_liquidity))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getMaxLiquidity` (0xd6b7dec5) function
        pub fn get_max_liquidity(
            &self,
            pool_id: u64,
            amount_0: ::ethers::core::types::U256,
            amount_1: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash([214, 183, 222, 197], (pool_id, amount_0, amount_1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getMaxOrder` (0xf07b879e) function
        pub fn get_max_order(
            &self,
            pool_id: u64,
            sell_asset: bool,
            swapper: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, Order> {
            self.0
                .method_hash([240, 123, 135, 158], (pool_id, sell_asset, swapper))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getNetBalance` (0x4dc68a90) function
        pub fn get_net_balance(
            &self,
            token: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([77, 198, 138, 144], token)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPairId` (0x3f92a339) function
        pub fn get_pair_id(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([63, 146, 163, 57], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPairNonce` (0x078888d6) function
        pub fn get_pair_nonce(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([7, 136, 136, 214], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPoolNonce` (0xa5cd8a49) function
        pub fn get_pool_nonce(
            &self,
            p0: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([165, 205, 138, 73], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPoolReserves` (0x2afb9df8) function
        pub fn get_pool_reserves(
            &self,
            pool_id: u64,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([42, 251, 157, 248], pool_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getReserve` (0xc9a396e9) function
        pub fn get_reserve(
            &self,
            token: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([201, 163, 150, 233], token)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getSpotPrice` (0xe331ba34) function
        pub fn get_spot_price(
            &self,
            pool_id: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([227, 49, 186, 52], pool_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getStrategy` (0x30244be7) function
        pub fn get_strategy(
            &self,
            pool_id: u64,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([48, 36, 75, 231], pool_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isApprovedForAll` (0xe985e9c5) function
        pub fn is_approved_for_all(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([233, 133, 233, 197], (p0, p1))
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
        ///Calls the contract's `pairs` (0x5e47663c) function
        pub fn pairs(
            &self,
            p0: u32,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::Address, u8, ::ethers::core::types::Address, u8),
        > {
            self.0
                .method_hash([94, 71, 102, 60], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pools` (0x89a5f084) function
        pub fn pools(
            &self,
            p0: u64,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                u128,
                u128,
                u128,
                u32,
                u16,
                u16,
                ::ethers::core::types::Address,
                ::ethers::core::types::Address,
            ),
        > {
            self.0
                .method_hash([137, 165, 240, 132], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `protocolFee` (0xb0e21e8a) function
        pub fn protocol_fee(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([176, 226, 30, 138], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `protocolFees` (0xdcf844a7) function
        pub fn protocol_fees(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([220, 248, 68, 167], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `safeBatchTransferFrom` (0x2eb2c2d6) function
        pub fn safe_batch_transfer_from(
            &self,
            from: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            ids: ::std::vec::Vec<::ethers::core::types::U256>,
            amounts: ::std::vec::Vec<::ethers::core::types::U256>,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([46, 178, 194, 214], (from, to, ids, amounts, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `safeTransferFrom` (0xf242432a) function
        pub fn safe_transfer_from(
            &self,
            from: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            id: ::ethers::core::types::U256,
            amount: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 66, 67, 42], (from, to, id, amount, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setApprovalForAll` (0xa22cb465) function
        pub fn set_approval_for_all(
            &self,
            operator: ::ethers::core::types::Address,
            approved: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([162, 44, 180, 101], (operator, approved))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setProtocolFee` (0x787dce3d) function
        pub fn set_protocol_fee(
            &self,
            fee: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([120, 125, 206, 61], fee)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `simulateSwap` (0x80af9d76) function
        pub fn simulate_swap(
            &self,
            order: Order,
            timestamp: ::ethers::core::types::U256,
            swapper: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (bool, ::ethers::core::types::I256, ::ethers::core::types::I256),
        > {
            self.0
                .method_hash([128, 175, 157, 118], (order, timestamp, swapper))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `supportsInterface` (0x01ffc9a7) function
        pub fn supports_interface(
            &self,
            interface_id: [u8; 4],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([1, 255, 201, 167], interface_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `swap` (0xf33ae1bc) function
        pub fn swap(
            &self,
            args: Order,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (u64, ::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([243, 58, 225, 188], (args,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `uri` (0x0e89341c) function
        pub fn uri(
            &self,
            id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([14, 137, 52, 28], id)
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
        ///Gets the contract's `ApprovalForAll` event
        pub fn approval_for_all_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ApprovalForAllFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ChangeParameters` event
        pub fn change_parameters_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ChangeParametersFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ClaimFees` event
        pub fn claim_fees_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ClaimFeesFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `CreatePair` event
        pub fn create_pair_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            CreatePairFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `CreatePool` event
        pub fn create_pool_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            CreatePoolFilter,
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
        ///Gets the contract's `DecreaseReserveBalance` event
        pub fn decrease_reserve_balance_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DecreaseReserveBalanceFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Deposit` event
        pub fn deposit_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, DepositFilter> {
            self.0.event()
        }
        ///Gets the contract's `IncreaseReserveBalance` event
        pub fn increase_reserve_balance_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            IncreaseReserveBalanceFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Swap` event
        pub fn swap_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, SwapFilter> {
            self.0.event()
        }
        ///Gets the contract's `TransferBatch` event
        pub fn transfer_batch_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            TransferBatchFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `TransferSingle` event
        pub fn transfer_single_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            TransferSingleFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `URI` event
        pub fn uri_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, UriFilter> {
            self.0.event()
        }
        ///Gets the contract's `UpdateProtocolFee` event
        pub fn update_protocol_fee_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            UpdateProtocolFeeFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PortfolioEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Portfolio<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `EtherTransfer` with signature `EtherTransfer()` and selector `0x356594ab`
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
    #[etherror(name = "EtherTransfer", abi = "EtherTransfer()")]
    pub struct EtherTransfer;
    ///Custom Error type `InsufficientReserve` with signature `InsufficientReserve(uint256,uint256)` and selector `0x315276c9`
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
        name = "InsufficientReserve",
        abi = "InsufficientReserve(uint256,uint256)"
    )]
    pub struct InsufficientReserve {
        pub amount: ::ethers::core::types::U256,
        pub delta: ::ethers::core::types::U256,
    }
    ///Custom Error type `InvalidBalance` with signature `InvalidBalance()` and selector `0xc52e3eff`
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
    #[etherror(name = "InvalidBalance", abi = "InvalidBalance()")]
    pub struct InvalidBalance;
    ///Custom Error type `PoolLib_AlreadyCreated` with signature `PoolLib_AlreadyCreated()` and selector `0xe930cedf`
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
    #[etherror(name = "PoolLib_AlreadyCreated", abi = "PoolLib_AlreadyCreated()")]
    pub struct PoolLib_AlreadyCreated;
    ///Custom Error type `PoolLib_InvalidFee` with signature `PoolLib_InvalidFee(uint256)` and selector `0x971b3109`
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
    #[etherror(name = "PoolLib_InvalidFee", abi = "PoolLib_InvalidFee(uint256)")]
    pub struct PoolLib_InvalidFee(pub ::ethers::core::types::U256);
    ///Custom Error type `PoolLib_InvalidPriorityFee` with signature `PoolLib_InvalidPriorityFee(uint256)` and selector `0xeddfe119`
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
        name = "PoolLib_InvalidPriorityFee",
        abi = "PoolLib_InvalidPriorityFee(uint256)"
    )]
    pub struct PoolLib_InvalidPriorityFee(pub ::ethers::core::types::U256);
    ///Custom Error type `PoolLib_InvalidReserveX` with signature `PoolLib_InvalidReserveX()` and selector `0x5d3f506c`
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
    #[etherror(name = "PoolLib_InvalidReserveX", abi = "PoolLib_InvalidReserveX()")]
    pub struct PoolLib_InvalidReserveX;
    ///Custom Error type `PoolLib_InvalidReserveY` with signature `PoolLib_InvalidReserveY()` and selector `0x2869c5f3`
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
    #[etherror(name = "PoolLib_InvalidReserveY", abi = "PoolLib_InvalidReserveY()")]
    pub struct PoolLib_InvalidReserveY;
    ///Custom Error type `PoolLib_UpperLiquidityLimit` with signature `PoolLib_UpperLiquidityLimit()` and selector `0xacc9407b`
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
        name = "PoolLib_UpperLiquidityLimit",
        abi = "PoolLib_UpperLiquidityLimit()"
    )]
    pub struct PoolLib_UpperLiquidityLimit;
    ///Custom Error type `Portfolio_AfterCreateFail` with signature `Portfolio_AfterCreateFail()` and selector `0xdf158a58`
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
    #[etherror(name = "Portfolio_AfterCreateFail", abi = "Portfolio_AfterCreateFail()")]
    pub struct Portfolio_AfterCreateFail;
    ///Custom Error type `Portfolio_BeforeSwapFail` with signature `Portfolio_BeforeSwapFail()` and selector `0x2ed0ea01`
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
    #[etherror(name = "Portfolio_BeforeSwapFail", abi = "Portfolio_BeforeSwapFail()")]
    pub struct Portfolio_BeforeSwapFail;
    ///Custom Error type `Portfolio_DuplicateToken` with signature `Portfolio_DuplicateToken()` and selector `0x05134ccc`
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
    #[etherror(name = "Portfolio_DuplicateToken", abi = "Portfolio_DuplicateToken()")]
    pub struct Portfolio_DuplicateToken;
    ///Custom Error type `Portfolio_Insolvent` with signature `Portfolio_Insolvent(address,int256)` and selector `0xaa269de2`
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
        name = "Portfolio_Insolvent",
        abi = "Portfolio_Insolvent(address,int256)"
    )]
    pub struct Portfolio_Insolvent {
        pub token: ::ethers::core::types::Address,
        pub net: ::ethers::core::types::I256,
    }
    ///Custom Error type `Portfolio_InsufficientLiquidity` with signature `Portfolio_InsufficientLiquidity()` and selector `0xcb6dab75`
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
        name = "Portfolio_InsufficientLiquidity",
        abi = "Portfolio_InsufficientLiquidity()"
    )]
    pub struct Portfolio_InsufficientLiquidity;
    ///Custom Error type `Portfolio_InvalidDecimals` with signature `Portfolio_InvalidDecimals(uint8)` and selector `0xc3da7747`
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
        name = "Portfolio_InvalidDecimals",
        abi = "Portfolio_InvalidDecimals(uint8)"
    )]
    pub struct Portfolio_InvalidDecimals {
        pub decimals: u8,
    }
    ///Custom Error type `Portfolio_InvalidInvariant` with signature `Portfolio_InvalidInvariant(int256,int256)` and selector `0xb93dee0a`
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
        name = "Portfolio_InvalidInvariant",
        abi = "Portfolio_InvalidInvariant(int256,int256)"
    )]
    pub struct Portfolio_InvalidInvariant {
        pub prev: ::ethers::core::types::I256,
        pub next: ::ethers::core::types::I256,
    }
    ///Custom Error type `Portfolio_InvalidMulticall` with signature `Portfolio_InvalidMulticall()` and selector `0x55e1f7c5`
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
        name = "Portfolio_InvalidMulticall",
        abi = "Portfolio_InvalidMulticall()"
    )]
    pub struct Portfolio_InvalidMulticall;
    ///Custom Error type `Portfolio_InvalidPairNonce` with signature `Portfolio_InvalidPairNonce()` and selector `0xcc7a739b`
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
        name = "Portfolio_InvalidPairNonce",
        abi = "Portfolio_InvalidPairNonce()"
    )]
    pub struct Portfolio_InvalidPairNonce;
    ///Custom Error type `Portfolio_InvalidPool` with signature `Portfolio_InvalidPool(uint64)` and selector `0xbc27a517`
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
    #[etherror(name = "Portfolio_InvalidPool", abi = "Portfolio_InvalidPool(uint64)")]
    pub struct Portfolio_InvalidPool {
        pub pool_id: u64,
    }
    ///Custom Error type `Portfolio_InvalidProtocolFee` with signature `Portfolio_InvalidProtocolFee(uint256)` and selector `0x64597477`
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
        name = "Portfolio_InvalidProtocolFee",
        abi = "Portfolio_InvalidProtocolFee(uint256)"
    )]
    pub struct Portfolio_InvalidProtocolFee {
        pub protocol_fee: ::ethers::core::types::U256,
    }
    ///Custom Error type `Portfolio_InvalidReentrancy` with signature `Portfolio_InvalidReentrancy()` and selector `0x02b8002d`
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
        name = "Portfolio_InvalidReentrancy",
        abi = "Portfolio_InvalidReentrancy()"
    )]
    pub struct Portfolio_InvalidReentrancy;
    ///Custom Error type `Portfolio_InvalidSettlement` with signature `Portfolio_InvalidSettlement()` and selector `0x326efa43`
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
        name = "Portfolio_InvalidSettlement",
        abi = "Portfolio_InvalidSettlement()"
    )]
    pub struct Portfolio_InvalidSettlement;
    ///Custom Error type `Portfolio_MaxAssetExceeded` with signature `Portfolio_MaxAssetExceeded()` and selector `0xace41c3a`
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
        name = "Portfolio_MaxAssetExceeded",
        abi = "Portfolio_MaxAssetExceeded()"
    )]
    pub struct Portfolio_MaxAssetExceeded;
    ///Custom Error type `Portfolio_MaxQuoteExceeded` with signature `Portfolio_MaxQuoteExceeded()` and selector `0x84c05a5c`
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
        name = "Portfolio_MaxQuoteExceeded",
        abi = "Portfolio_MaxQuoteExceeded()"
    )]
    pub struct Portfolio_MaxQuoteExceeded;
    ///Custom Error type `Portfolio_MinAssetExceeded` with signature `Portfolio_MinAssetExceeded()` and selector `0x63448430`
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
        name = "Portfolio_MinAssetExceeded",
        abi = "Portfolio_MinAssetExceeded()"
    )]
    pub struct Portfolio_MinAssetExceeded;
    ///Custom Error type `Portfolio_MinQuoteExceeded` with signature `Portfolio_MinQuoteExceeded()` and selector `0xacb5bdea`
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
        name = "Portfolio_MinQuoteExceeded",
        abi = "Portfolio_MinQuoteExceeded()"
    )]
    pub struct Portfolio_MinQuoteExceeded;
    ///Custom Error type `Portfolio_NonExistentPool` with signature `Portfolio_NonExistentPool(uint64)` and selector `0x1216e0fa`
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
        name = "Portfolio_NonExistentPool",
        abi = "Portfolio_NonExistentPool(uint64)"
    )]
    pub struct Portfolio_NonExistentPool {
        pub pool_id: u64,
    }
    ///Custom Error type `Portfolio_NotController` with signature `Portfolio_NotController()` and selector `0xffbe9c2c`
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
    #[etherror(name = "Portfolio_NotController", abi = "Portfolio_NotController()")]
    pub struct Portfolio_NotController;
    ///Custom Error type `Portfolio_PairExists` with signature `Portfolio_PairExists(uint24)` and selector `0xb0988c43`
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
    #[etherror(name = "Portfolio_PairExists", abi = "Portfolio_PairExists(uint24)")]
    pub struct Portfolio_PairExists {
        pub pair_id: u32,
    }
    ///Custom Error type `Portfolio_ZeroAmountsAllocate` with signature `Portfolio_ZeroAmountsAllocate()` and selector `0x658b16ed`
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
        name = "Portfolio_ZeroAmountsAllocate",
        abi = "Portfolio_ZeroAmountsAllocate()"
    )]
    pub struct Portfolio_ZeroAmountsAllocate;
    ///Custom Error type `Portfolio_ZeroLiquidityAllocate` with signature `Portfolio_ZeroLiquidityAllocate()` and selector `0x90609a7d`
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
        name = "Portfolio_ZeroLiquidityAllocate",
        abi = "Portfolio_ZeroLiquidityAllocate()"
    )]
    pub struct Portfolio_ZeroLiquidityAllocate;
    ///Custom Error type `Portfolio_ZeroLiquidityDeallocate` with signature `Portfolio_ZeroLiquidityDeallocate()` and selector `0x14ef605e`
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
        name = "Portfolio_ZeroLiquidityDeallocate",
        abi = "Portfolio_ZeroLiquidityDeallocate()"
    )]
    pub struct Portfolio_ZeroLiquidityDeallocate;
    ///Custom Error type `Portfolio_ZeroSwapInput` with signature `Portfolio_ZeroSwapInput()` and selector `0x13fd9b6d`
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
    #[etherror(name = "Portfolio_ZeroSwapInput", abi = "Portfolio_ZeroSwapInput()")]
    pub struct Portfolio_ZeroSwapInput;
    ///Custom Error type `Portfolio_ZeroSwapLiquidity` with signature `Portfolio_ZeroSwapLiquidity()` and selector `0x7a95cb21`
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
        name = "Portfolio_ZeroSwapLiquidity",
        abi = "Portfolio_ZeroSwapLiquidity()"
    )]
    pub struct Portfolio_ZeroSwapLiquidity;
    ///Custom Error type `Portfolio_ZeroSwapOutput` with signature `Portfolio_ZeroSwapOutput()` and selector `0x6f85b34e`
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
    #[etherror(name = "Portfolio_ZeroSwapOutput", abi = "Portfolio_ZeroSwapOutput()")]
    pub struct Portfolio_ZeroSwapOutput;
    ///Custom Error type `SwapLib_OutputExceedsReserves` with signature `SwapLib_OutputExceedsReserves()` and selector `0x866a032b`
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
        name = "SwapLib_OutputExceedsReserves",
        abi = "SwapLib_OutputExceedsReserves()"
    )]
    pub struct SwapLib_OutputExceedsReserves;
    ///Custom Error type `SwapLib_ProtocolFeeTooHigh` with signature `SwapLib_ProtocolFeeTooHigh()` and selector `0xec8e1fce`
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
        name = "SwapLib_ProtocolFeeTooHigh",
        abi = "SwapLib_ProtocolFeeTooHigh()"
    )]
    pub struct SwapLib_ProtocolFeeTooHigh;
    ///Custom Error type `SwapLib_ZeroXAdjustment` with signature `SwapLib_ZeroXAdjustment()` and selector `0x7276f08a`
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
    #[etherror(name = "SwapLib_ZeroXAdjustment", abi = "SwapLib_ZeroXAdjustment()")]
    pub struct SwapLib_ZeroXAdjustment;
    ///Custom Error type `SwapLib_ZeroYAdjustment` with signature `SwapLib_ZeroYAdjustment()` and selector `0x1fb0b7dd`
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
    #[etherror(name = "SwapLib_ZeroYAdjustment", abi = "SwapLib_ZeroYAdjustment()")]
    pub struct SwapLib_ZeroYAdjustment;
    ///Custom Error type `TokenTransfer` with signature `TokenTransfer()` and selector `0xeb2cf4fc`
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
    #[etherror(name = "TokenTransfer", abi = "TokenTransfer()")]
    pub struct TokenTransfer;
    ///Custom Error type `TokenTransferFrom` with signature `TokenTransferFrom()` and selector `0x6e89eca5`
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
    #[etherror(name = "TokenTransferFrom", abi = "TokenTransferFrom()")]
    pub struct TokenTransferFrom;
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
    pub enum PortfolioErrors {
        EtherTransfer(EtherTransfer),
        InsufficientReserve(InsufficientReserve),
        InvalidBalance(InvalidBalance),
        PoolLib_AlreadyCreated(PoolLib_AlreadyCreated),
        PoolLib_InvalidFee(PoolLib_InvalidFee),
        PoolLib_InvalidPriorityFee(PoolLib_InvalidPriorityFee),
        PoolLib_InvalidReserveX(PoolLib_InvalidReserveX),
        PoolLib_InvalidReserveY(PoolLib_InvalidReserveY),
        PoolLib_UpperLiquidityLimit(PoolLib_UpperLiquidityLimit),
        Portfolio_AfterCreateFail(Portfolio_AfterCreateFail),
        Portfolio_BeforeSwapFail(Portfolio_BeforeSwapFail),
        Portfolio_DuplicateToken(Portfolio_DuplicateToken),
        Portfolio_Insolvent(Portfolio_Insolvent),
        Portfolio_InsufficientLiquidity(Portfolio_InsufficientLiquidity),
        Portfolio_InvalidDecimals(Portfolio_InvalidDecimals),
        Portfolio_InvalidInvariant(Portfolio_InvalidInvariant),
        Portfolio_InvalidMulticall(Portfolio_InvalidMulticall),
        Portfolio_InvalidPairNonce(Portfolio_InvalidPairNonce),
        Portfolio_InvalidPool(Portfolio_InvalidPool),
        Portfolio_InvalidProtocolFee(Portfolio_InvalidProtocolFee),
        Portfolio_InvalidReentrancy(Portfolio_InvalidReentrancy),
        Portfolio_InvalidSettlement(Portfolio_InvalidSettlement),
        Portfolio_MaxAssetExceeded(Portfolio_MaxAssetExceeded),
        Portfolio_MaxQuoteExceeded(Portfolio_MaxQuoteExceeded),
        Portfolio_MinAssetExceeded(Portfolio_MinAssetExceeded),
        Portfolio_MinQuoteExceeded(Portfolio_MinQuoteExceeded),
        Portfolio_NonExistentPool(Portfolio_NonExistentPool),
        Portfolio_NotController(Portfolio_NotController),
        Portfolio_PairExists(Portfolio_PairExists),
        Portfolio_ZeroAmountsAllocate(Portfolio_ZeroAmountsAllocate),
        Portfolio_ZeroLiquidityAllocate(Portfolio_ZeroLiquidityAllocate),
        Portfolio_ZeroLiquidityDeallocate(Portfolio_ZeroLiquidityDeallocate),
        Portfolio_ZeroSwapInput(Portfolio_ZeroSwapInput),
        Portfolio_ZeroSwapLiquidity(Portfolio_ZeroSwapLiquidity),
        Portfolio_ZeroSwapOutput(Portfolio_ZeroSwapOutput),
        SwapLib_OutputExceedsReserves(SwapLib_OutputExceedsReserves),
        SwapLib_ProtocolFeeTooHigh(SwapLib_ProtocolFeeTooHigh),
        SwapLib_ZeroXAdjustment(SwapLib_ZeroXAdjustment),
        SwapLib_ZeroYAdjustment(SwapLib_ZeroYAdjustment),
        TokenTransfer(TokenTransfer),
        TokenTransferFrom(TokenTransferFrom),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for PortfolioErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <EtherTransfer as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EtherTransfer(decoded));
            }
            if let Ok(decoded) = <InsufficientReserve as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InsufficientReserve(decoded));
            }
            if let Ok(decoded) = <InvalidBalance as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidBalance(decoded));
            }
            if let Ok(decoded) = <PoolLib_AlreadyCreated as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PoolLib_AlreadyCreated(decoded));
            }
            if let Ok(decoded) = <PoolLib_InvalidFee as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PoolLib_InvalidFee(decoded));
            }
            if let Ok(decoded) = <PoolLib_InvalidPriorityFee as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PoolLib_InvalidPriorityFee(decoded));
            }
            if let Ok(decoded) = <PoolLib_InvalidReserveX as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PoolLib_InvalidReserveX(decoded));
            }
            if let Ok(decoded) = <PoolLib_InvalidReserveY as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PoolLib_InvalidReserveY(decoded));
            }
            if let Ok(decoded) = <PoolLib_UpperLiquidityLimit as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PoolLib_UpperLiquidityLimit(decoded));
            }
            if let Ok(decoded) = <Portfolio_AfterCreateFail as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Portfolio_AfterCreateFail(decoded));
            }
            if let Ok(decoded) = <Portfolio_BeforeSwapFail as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Portfolio_BeforeSwapFail(decoded));
            }
            if let Ok(decoded) = <Portfolio_DuplicateToken as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Portfolio_DuplicateToken(decoded));
            }
            if let Ok(decoded) = <Portfolio_Insolvent as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Portfolio_Insolvent(decoded));
            }
            if let Ok(decoded) = <Portfolio_InsufficientLiquidity as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Portfolio_InsufficientLiquidity(decoded));
            }
            if let Ok(decoded) = <Portfolio_InvalidDecimals as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Portfolio_InvalidDecimals(decoded));
            }
            if let Ok(decoded) = <Portfolio_InvalidInvariant as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Portfolio_InvalidInvariant(decoded));
            }
            if let Ok(decoded) = <Portfolio_InvalidMulticall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Portfolio_InvalidMulticall(decoded));
            }
            if let Ok(decoded) = <Portfolio_InvalidPairNonce as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Portfolio_InvalidPairNonce(decoded));
            }
            if let Ok(decoded) = <Portfolio_InvalidPool as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Portfolio_InvalidPool(decoded));
            }
            if let Ok(decoded) = <Portfolio_InvalidProtocolFee as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Portfolio_InvalidProtocolFee(decoded));
            }
            if let Ok(decoded) = <Portfolio_InvalidReentrancy as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Portfolio_InvalidReentrancy(decoded));
            }
            if let Ok(decoded) = <Portfolio_InvalidSettlement as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Portfolio_InvalidSettlement(decoded));
            }
            if let Ok(decoded) = <Portfolio_MaxAssetExceeded as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Portfolio_MaxAssetExceeded(decoded));
            }
            if let Ok(decoded) = <Portfolio_MaxQuoteExceeded as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Portfolio_MaxQuoteExceeded(decoded));
            }
            if let Ok(decoded) = <Portfolio_MinAssetExceeded as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Portfolio_MinAssetExceeded(decoded));
            }
            if let Ok(decoded) = <Portfolio_MinQuoteExceeded as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Portfolio_MinQuoteExceeded(decoded));
            }
            if let Ok(decoded) = <Portfolio_NonExistentPool as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Portfolio_NonExistentPool(decoded));
            }
            if let Ok(decoded) = <Portfolio_NotController as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Portfolio_NotController(decoded));
            }
            if let Ok(decoded) = <Portfolio_PairExists as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Portfolio_PairExists(decoded));
            }
            if let Ok(decoded) = <Portfolio_ZeroAmountsAllocate as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Portfolio_ZeroAmountsAllocate(decoded));
            }
            if let Ok(decoded) = <Portfolio_ZeroLiquidityAllocate as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Portfolio_ZeroLiquidityAllocate(decoded));
            }
            if let Ok(decoded) = <Portfolio_ZeroLiquidityDeallocate as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Portfolio_ZeroLiquidityDeallocate(decoded));
            }
            if let Ok(decoded) = <Portfolio_ZeroSwapInput as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Portfolio_ZeroSwapInput(decoded));
            }
            if let Ok(decoded) = <Portfolio_ZeroSwapLiquidity as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Portfolio_ZeroSwapLiquidity(decoded));
            }
            if let Ok(decoded) = <Portfolio_ZeroSwapOutput as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Portfolio_ZeroSwapOutput(decoded));
            }
            if let Ok(decoded) = <SwapLib_OutputExceedsReserves as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SwapLib_OutputExceedsReserves(decoded));
            }
            if let Ok(decoded) = <SwapLib_ProtocolFeeTooHigh as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SwapLib_ProtocolFeeTooHigh(decoded));
            }
            if let Ok(decoded) = <SwapLib_ZeroXAdjustment as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SwapLib_ZeroXAdjustment(decoded));
            }
            if let Ok(decoded) = <SwapLib_ZeroYAdjustment as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SwapLib_ZeroYAdjustment(decoded));
            }
            if let Ok(decoded) = <TokenTransfer as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TokenTransfer(decoded));
            }
            if let Ok(decoded) = <TokenTransferFrom as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TokenTransferFrom(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for PortfolioErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::EtherTransfer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InsufficientReserve(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidBalance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PoolLib_AlreadyCreated(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PoolLib_InvalidFee(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PoolLib_InvalidPriorityFee(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PoolLib_InvalidReserveX(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PoolLib_InvalidReserveY(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PoolLib_UpperLiquidityLimit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Portfolio_AfterCreateFail(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Portfolio_BeforeSwapFail(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Portfolio_DuplicateToken(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Portfolio_Insolvent(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Portfolio_InsufficientLiquidity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Portfolio_InvalidDecimals(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Portfolio_InvalidInvariant(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Portfolio_InvalidMulticall(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Portfolio_InvalidPairNonce(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Portfolio_InvalidPool(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Portfolio_InvalidProtocolFee(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Portfolio_InvalidReentrancy(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Portfolio_InvalidSettlement(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Portfolio_MaxAssetExceeded(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Portfolio_MaxQuoteExceeded(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Portfolio_MinAssetExceeded(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Portfolio_MinQuoteExceeded(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Portfolio_NonExistentPool(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Portfolio_NotController(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Portfolio_PairExists(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Portfolio_ZeroAmountsAllocate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Portfolio_ZeroLiquidityAllocate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Portfolio_ZeroLiquidityDeallocate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Portfolio_ZeroSwapInput(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Portfolio_ZeroSwapLiquidity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Portfolio_ZeroSwapOutput(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SwapLib_OutputExceedsReserves(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SwapLib_ProtocolFeeTooHigh(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SwapLib_ZeroXAdjustment(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SwapLib_ZeroYAdjustment(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TokenTransfer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TokenTransferFrom(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for PortfolioErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <EtherTransfer as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InsufficientReserve as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidBalance as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <PoolLib_AlreadyCreated as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <PoolLib_InvalidFee as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <PoolLib_InvalidPriorityFee as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <PoolLib_InvalidReserveX as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <PoolLib_InvalidReserveY as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <PoolLib_UpperLiquidityLimit as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Portfolio_AfterCreateFail as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Portfolio_BeforeSwapFail as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Portfolio_DuplicateToken as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Portfolio_Insolvent as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Portfolio_InsufficientLiquidity as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Portfolio_InvalidDecimals as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Portfolio_InvalidInvariant as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Portfolio_InvalidMulticall as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Portfolio_InvalidPairNonce as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Portfolio_InvalidPool as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Portfolio_InvalidProtocolFee as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Portfolio_InvalidReentrancy as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Portfolio_InvalidSettlement as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Portfolio_MaxAssetExceeded as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Portfolio_MaxQuoteExceeded as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Portfolio_MinAssetExceeded as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Portfolio_MinQuoteExceeded as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Portfolio_NonExistentPool as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Portfolio_NotController as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Portfolio_PairExists as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Portfolio_ZeroAmountsAllocate as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Portfolio_ZeroLiquidityAllocate as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Portfolio_ZeroLiquidityDeallocate as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Portfolio_ZeroSwapInput as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Portfolio_ZeroSwapLiquidity as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Portfolio_ZeroSwapOutput as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <SwapLib_OutputExceedsReserves as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <SwapLib_ProtocolFeeTooHigh as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <SwapLib_ZeroXAdjustment as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <SwapLib_ZeroYAdjustment as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <TokenTransfer as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <TokenTransferFrom as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for PortfolioErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::EtherTransfer(element) => ::core::fmt::Display::fmt(element, f),
                Self::InsufficientReserve(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidBalance(element) => ::core::fmt::Display::fmt(element, f),
                Self::PoolLib_AlreadyCreated(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PoolLib_InvalidFee(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PoolLib_InvalidPriorityFee(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PoolLib_InvalidReserveX(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PoolLib_InvalidReserveY(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PoolLib_UpperLiquidityLimit(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Portfolio_AfterCreateFail(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Portfolio_BeforeSwapFail(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Portfolio_DuplicateToken(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Portfolio_Insolvent(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Portfolio_InsufficientLiquidity(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Portfolio_InvalidDecimals(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Portfolio_InvalidInvariant(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Portfolio_InvalidMulticall(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Portfolio_InvalidPairNonce(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Portfolio_InvalidPool(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Portfolio_InvalidProtocolFee(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Portfolio_InvalidReentrancy(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Portfolio_InvalidSettlement(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Portfolio_MaxAssetExceeded(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Portfolio_MaxQuoteExceeded(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Portfolio_MinAssetExceeded(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Portfolio_MinQuoteExceeded(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Portfolio_NonExistentPool(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Portfolio_NotController(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Portfolio_PairExists(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Portfolio_ZeroAmountsAllocate(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Portfolio_ZeroLiquidityAllocate(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Portfolio_ZeroLiquidityDeallocate(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Portfolio_ZeroSwapInput(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Portfolio_ZeroSwapLiquidity(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Portfolio_ZeroSwapOutput(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SwapLib_OutputExceedsReserves(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SwapLib_ProtocolFeeTooHigh(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SwapLib_ZeroXAdjustment(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SwapLib_ZeroYAdjustment(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TokenTransfer(element) => ::core::fmt::Display::fmt(element, f),
                Self::TokenTransferFrom(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for PortfolioErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<EtherTransfer> for PortfolioErrors {
        fn from(value: EtherTransfer) -> Self {
            Self::EtherTransfer(value)
        }
    }
    impl ::core::convert::From<InsufficientReserve> for PortfolioErrors {
        fn from(value: InsufficientReserve) -> Self {
            Self::InsufficientReserve(value)
        }
    }
    impl ::core::convert::From<InvalidBalance> for PortfolioErrors {
        fn from(value: InvalidBalance) -> Self {
            Self::InvalidBalance(value)
        }
    }
    impl ::core::convert::From<PoolLib_AlreadyCreated> for PortfolioErrors {
        fn from(value: PoolLib_AlreadyCreated) -> Self {
            Self::PoolLib_AlreadyCreated(value)
        }
    }
    impl ::core::convert::From<PoolLib_InvalidFee> for PortfolioErrors {
        fn from(value: PoolLib_InvalidFee) -> Self {
            Self::PoolLib_InvalidFee(value)
        }
    }
    impl ::core::convert::From<PoolLib_InvalidPriorityFee> for PortfolioErrors {
        fn from(value: PoolLib_InvalidPriorityFee) -> Self {
            Self::PoolLib_InvalidPriorityFee(value)
        }
    }
    impl ::core::convert::From<PoolLib_InvalidReserveX> for PortfolioErrors {
        fn from(value: PoolLib_InvalidReserveX) -> Self {
            Self::PoolLib_InvalidReserveX(value)
        }
    }
    impl ::core::convert::From<PoolLib_InvalidReserveY> for PortfolioErrors {
        fn from(value: PoolLib_InvalidReserveY) -> Self {
            Self::PoolLib_InvalidReserveY(value)
        }
    }
    impl ::core::convert::From<PoolLib_UpperLiquidityLimit> for PortfolioErrors {
        fn from(value: PoolLib_UpperLiquidityLimit) -> Self {
            Self::PoolLib_UpperLiquidityLimit(value)
        }
    }
    impl ::core::convert::From<Portfolio_AfterCreateFail> for PortfolioErrors {
        fn from(value: Portfolio_AfterCreateFail) -> Self {
            Self::Portfolio_AfterCreateFail(value)
        }
    }
    impl ::core::convert::From<Portfolio_BeforeSwapFail> for PortfolioErrors {
        fn from(value: Portfolio_BeforeSwapFail) -> Self {
            Self::Portfolio_BeforeSwapFail(value)
        }
    }
    impl ::core::convert::From<Portfolio_DuplicateToken> for PortfolioErrors {
        fn from(value: Portfolio_DuplicateToken) -> Self {
            Self::Portfolio_DuplicateToken(value)
        }
    }
    impl ::core::convert::From<Portfolio_Insolvent> for PortfolioErrors {
        fn from(value: Portfolio_Insolvent) -> Self {
            Self::Portfolio_Insolvent(value)
        }
    }
    impl ::core::convert::From<Portfolio_InsufficientLiquidity> for PortfolioErrors {
        fn from(value: Portfolio_InsufficientLiquidity) -> Self {
            Self::Portfolio_InsufficientLiquidity(value)
        }
    }
    impl ::core::convert::From<Portfolio_InvalidDecimals> for PortfolioErrors {
        fn from(value: Portfolio_InvalidDecimals) -> Self {
            Self::Portfolio_InvalidDecimals(value)
        }
    }
    impl ::core::convert::From<Portfolio_InvalidInvariant> for PortfolioErrors {
        fn from(value: Portfolio_InvalidInvariant) -> Self {
            Self::Portfolio_InvalidInvariant(value)
        }
    }
    impl ::core::convert::From<Portfolio_InvalidMulticall> for PortfolioErrors {
        fn from(value: Portfolio_InvalidMulticall) -> Self {
            Self::Portfolio_InvalidMulticall(value)
        }
    }
    impl ::core::convert::From<Portfolio_InvalidPairNonce> for PortfolioErrors {
        fn from(value: Portfolio_InvalidPairNonce) -> Self {
            Self::Portfolio_InvalidPairNonce(value)
        }
    }
    impl ::core::convert::From<Portfolio_InvalidPool> for PortfolioErrors {
        fn from(value: Portfolio_InvalidPool) -> Self {
            Self::Portfolio_InvalidPool(value)
        }
    }
    impl ::core::convert::From<Portfolio_InvalidProtocolFee> for PortfolioErrors {
        fn from(value: Portfolio_InvalidProtocolFee) -> Self {
            Self::Portfolio_InvalidProtocolFee(value)
        }
    }
    impl ::core::convert::From<Portfolio_InvalidReentrancy> for PortfolioErrors {
        fn from(value: Portfolio_InvalidReentrancy) -> Self {
            Self::Portfolio_InvalidReentrancy(value)
        }
    }
    impl ::core::convert::From<Portfolio_InvalidSettlement> for PortfolioErrors {
        fn from(value: Portfolio_InvalidSettlement) -> Self {
            Self::Portfolio_InvalidSettlement(value)
        }
    }
    impl ::core::convert::From<Portfolio_MaxAssetExceeded> for PortfolioErrors {
        fn from(value: Portfolio_MaxAssetExceeded) -> Self {
            Self::Portfolio_MaxAssetExceeded(value)
        }
    }
    impl ::core::convert::From<Portfolio_MaxQuoteExceeded> for PortfolioErrors {
        fn from(value: Portfolio_MaxQuoteExceeded) -> Self {
            Self::Portfolio_MaxQuoteExceeded(value)
        }
    }
    impl ::core::convert::From<Portfolio_MinAssetExceeded> for PortfolioErrors {
        fn from(value: Portfolio_MinAssetExceeded) -> Self {
            Self::Portfolio_MinAssetExceeded(value)
        }
    }
    impl ::core::convert::From<Portfolio_MinQuoteExceeded> for PortfolioErrors {
        fn from(value: Portfolio_MinQuoteExceeded) -> Self {
            Self::Portfolio_MinQuoteExceeded(value)
        }
    }
    impl ::core::convert::From<Portfolio_NonExistentPool> for PortfolioErrors {
        fn from(value: Portfolio_NonExistentPool) -> Self {
            Self::Portfolio_NonExistentPool(value)
        }
    }
    impl ::core::convert::From<Portfolio_NotController> for PortfolioErrors {
        fn from(value: Portfolio_NotController) -> Self {
            Self::Portfolio_NotController(value)
        }
    }
    impl ::core::convert::From<Portfolio_PairExists> for PortfolioErrors {
        fn from(value: Portfolio_PairExists) -> Self {
            Self::Portfolio_PairExists(value)
        }
    }
    impl ::core::convert::From<Portfolio_ZeroAmountsAllocate> for PortfolioErrors {
        fn from(value: Portfolio_ZeroAmountsAllocate) -> Self {
            Self::Portfolio_ZeroAmountsAllocate(value)
        }
    }
    impl ::core::convert::From<Portfolio_ZeroLiquidityAllocate> for PortfolioErrors {
        fn from(value: Portfolio_ZeroLiquidityAllocate) -> Self {
            Self::Portfolio_ZeroLiquidityAllocate(value)
        }
    }
    impl ::core::convert::From<Portfolio_ZeroLiquidityDeallocate> for PortfolioErrors {
        fn from(value: Portfolio_ZeroLiquidityDeallocate) -> Self {
            Self::Portfolio_ZeroLiquidityDeallocate(value)
        }
    }
    impl ::core::convert::From<Portfolio_ZeroSwapInput> for PortfolioErrors {
        fn from(value: Portfolio_ZeroSwapInput) -> Self {
            Self::Portfolio_ZeroSwapInput(value)
        }
    }
    impl ::core::convert::From<Portfolio_ZeroSwapLiquidity> for PortfolioErrors {
        fn from(value: Portfolio_ZeroSwapLiquidity) -> Self {
            Self::Portfolio_ZeroSwapLiquidity(value)
        }
    }
    impl ::core::convert::From<Portfolio_ZeroSwapOutput> for PortfolioErrors {
        fn from(value: Portfolio_ZeroSwapOutput) -> Self {
            Self::Portfolio_ZeroSwapOutput(value)
        }
    }
    impl ::core::convert::From<SwapLib_OutputExceedsReserves> for PortfolioErrors {
        fn from(value: SwapLib_OutputExceedsReserves) -> Self {
            Self::SwapLib_OutputExceedsReserves(value)
        }
    }
    impl ::core::convert::From<SwapLib_ProtocolFeeTooHigh> for PortfolioErrors {
        fn from(value: SwapLib_ProtocolFeeTooHigh) -> Self {
            Self::SwapLib_ProtocolFeeTooHigh(value)
        }
    }
    impl ::core::convert::From<SwapLib_ZeroXAdjustment> for PortfolioErrors {
        fn from(value: SwapLib_ZeroXAdjustment) -> Self {
            Self::SwapLib_ZeroXAdjustment(value)
        }
    }
    impl ::core::convert::From<SwapLib_ZeroYAdjustment> for PortfolioErrors {
        fn from(value: SwapLib_ZeroYAdjustment) -> Self {
            Self::SwapLib_ZeroYAdjustment(value)
        }
    }
    impl ::core::convert::From<TokenTransfer> for PortfolioErrors {
        fn from(value: TokenTransfer) -> Self {
            Self::TokenTransfer(value)
        }
    }
    impl ::core::convert::From<TokenTransferFrom> for PortfolioErrors {
        fn from(value: TokenTransferFrom) -> Self {
            Self::TokenTransferFrom(value)
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
        abi = "Allocate(uint64,address,address,uint256,uint256,uint256)"
    )]
    pub struct AllocateFilter {
        #[ethevent(indexed)]
        pub pool_id: u64,
        #[ethevent(indexed)]
        pub asset: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub quote: ::ethers::core::types::Address,
        pub delta_asset: ::ethers::core::types::U256,
        pub delta_quote: ::ethers::core::types::U256,
        pub delta_liquidity: ::ethers::core::types::U256,
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
    #[ethevent(name = "ApprovalForAll", abi = "ApprovalForAll(address,address,bool)")]
    pub struct ApprovalForAllFilter {
        #[ethevent(indexed)]
        pub owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub operator: ::ethers::core::types::Address,
        pub approved: bool,
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
        name = "ChangeParameters",
        abi = "ChangeParameters(uint64,uint16,uint16)"
    )]
    pub struct ChangeParametersFilter {
        #[ethevent(indexed)]
        pub pool_id: u64,
        #[ethevent(indexed)]
        pub priority_fee: u16,
        #[ethevent(indexed)]
        pub fee: u16,
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
    #[ethevent(name = "ClaimFees", abi = "ClaimFees(address,uint256)")]
    pub struct ClaimFeesFilter {
        #[ethevent(indexed)]
        pub token: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
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
        name = "CreatePair",
        abi = "CreatePair(uint24,address,address,uint8,uint8)"
    )]
    pub struct CreatePairFilter {
        #[ethevent(indexed)]
        pub pair_id: u32,
        #[ethevent(indexed)]
        pub asset: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub quote: ::ethers::core::types::Address,
        pub decimals_asset: u8,
        pub decimals_quote: u8,
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
        name = "CreatePool",
        abi = "CreatePool(uint64,address,address,uint256,uint256,uint16,uint16,address,address)"
    )]
    pub struct CreatePoolFilter {
        #[ethevent(indexed)]
        pub pool_id: u64,
        #[ethevent(indexed)]
        pub asset: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub quote: ::ethers::core::types::Address,
        pub reserve_x_per_wad: ::ethers::core::types::U256,
        pub reserve_y_per_wad: ::ethers::core::types::U256,
        pub fee_basis_points: u16,
        pub priority_fee_basis_points: u16,
        pub controller: ::ethers::core::types::Address,
        pub strategy: ::ethers::core::types::Address,
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
        abi = "Deallocate(uint64,address,address,uint256,uint256,uint256)"
    )]
    pub struct DeallocateFilter {
        #[ethevent(indexed)]
        pub pool_id: u64,
        #[ethevent(indexed)]
        pub asset: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub quote: ::ethers::core::types::Address,
        pub delta_asset: ::ethers::core::types::U256,
        pub delta_quote: ::ethers::core::types::U256,
        pub delta_liquidity: ::ethers::core::types::U256,
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
        name = "DecreaseReserveBalance",
        abi = "DecreaseReserveBalance(address,uint256)"
    )]
    pub struct DecreaseReserveBalanceFilter {
        #[ethevent(indexed)]
        pub token: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
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
    #[ethevent(name = "Deposit", abi = "Deposit(address,uint256)")]
    pub struct DepositFilter {
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
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
        name = "IncreaseReserveBalance",
        abi = "IncreaseReserveBalance(address,uint256)"
    )]
    pub struct IncreaseReserveBalanceFilter {
        #[ethevent(indexed)]
        pub token: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
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
        abi = "Swap(uint64,bool,address,uint256,address,uint256,uint256,int256)"
    )]
    pub struct SwapFilter {
        #[ethevent(indexed)]
        pub pool_id: u64,
        pub sell_asset: bool,
        #[ethevent(indexed)]
        pub token_in: ::ethers::core::types::Address,
        pub input: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub token_out: ::ethers::core::types::Address,
        pub output: ::ethers::core::types::U256,
        pub fee_amount_dec: ::ethers::core::types::U256,
        pub invariant_wad: ::ethers::core::types::I256,
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
        name = "TransferBatch",
        abi = "TransferBatch(address,address,address,uint256[],uint256[])"
    )]
    pub struct TransferBatchFilter {
        #[ethevent(indexed)]
        pub operator: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub from: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ::ethers::core::types::Address,
        pub ids: ::std::vec::Vec<::ethers::core::types::U256>,
        pub amounts: ::std::vec::Vec<::ethers::core::types::U256>,
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
        name = "TransferSingle",
        abi = "TransferSingle(address,address,address,uint256,uint256)"
    )]
    pub struct TransferSingleFilter {
        #[ethevent(indexed)]
        pub operator: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub from: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ::ethers::core::types::Address,
        pub id: ::ethers::core::types::U256,
        pub amount: ::ethers::core::types::U256,
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
    #[ethevent(name = "URI", abi = "URI(string,uint256)")]
    pub struct UriFilter {
        pub value: ::std::string::String,
        #[ethevent(indexed)]
        pub id: ::ethers::core::types::U256,
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
    #[ethevent(name = "UpdateProtocolFee", abi = "UpdateProtocolFee(uint256,uint256)")]
    pub struct UpdateProtocolFeeFilter {
        pub prev_fee: ::ethers::core::types::U256,
        pub next_fee: ::ethers::core::types::U256,
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
    pub enum PortfolioEvents {
        AllocateFilter(AllocateFilter),
        ApprovalForAllFilter(ApprovalForAllFilter),
        ChangeParametersFilter(ChangeParametersFilter),
        ClaimFeesFilter(ClaimFeesFilter),
        CreatePairFilter(CreatePairFilter),
        CreatePoolFilter(CreatePoolFilter),
        DeallocateFilter(DeallocateFilter),
        DecreaseReserveBalanceFilter(DecreaseReserveBalanceFilter),
        DepositFilter(DepositFilter),
        IncreaseReserveBalanceFilter(IncreaseReserveBalanceFilter),
        SwapFilter(SwapFilter),
        TransferBatchFilter(TransferBatchFilter),
        TransferSingleFilter(TransferSingleFilter),
        UriFilter(UriFilter),
        UpdateProtocolFeeFilter(UpdateProtocolFeeFilter),
    }
    impl ::ethers::contract::EthLogDecode for PortfolioEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AllocateFilter::decode_log(log) {
                return Ok(PortfolioEvents::AllocateFilter(decoded));
            }
            if let Ok(decoded) = ApprovalForAllFilter::decode_log(log) {
                return Ok(PortfolioEvents::ApprovalForAllFilter(decoded));
            }
            if let Ok(decoded) = ChangeParametersFilter::decode_log(log) {
                return Ok(PortfolioEvents::ChangeParametersFilter(decoded));
            }
            if let Ok(decoded) = ClaimFeesFilter::decode_log(log) {
                return Ok(PortfolioEvents::ClaimFeesFilter(decoded));
            }
            if let Ok(decoded) = CreatePairFilter::decode_log(log) {
                return Ok(PortfolioEvents::CreatePairFilter(decoded));
            }
            if let Ok(decoded) = CreatePoolFilter::decode_log(log) {
                return Ok(PortfolioEvents::CreatePoolFilter(decoded));
            }
            if let Ok(decoded) = DeallocateFilter::decode_log(log) {
                return Ok(PortfolioEvents::DeallocateFilter(decoded));
            }
            if let Ok(decoded) = DecreaseReserveBalanceFilter::decode_log(log) {
                return Ok(PortfolioEvents::DecreaseReserveBalanceFilter(decoded));
            }
            if let Ok(decoded) = DepositFilter::decode_log(log) {
                return Ok(PortfolioEvents::DepositFilter(decoded));
            }
            if let Ok(decoded) = IncreaseReserveBalanceFilter::decode_log(log) {
                return Ok(PortfolioEvents::IncreaseReserveBalanceFilter(decoded));
            }
            if let Ok(decoded) = SwapFilter::decode_log(log) {
                return Ok(PortfolioEvents::SwapFilter(decoded));
            }
            if let Ok(decoded) = TransferBatchFilter::decode_log(log) {
                return Ok(PortfolioEvents::TransferBatchFilter(decoded));
            }
            if let Ok(decoded) = TransferSingleFilter::decode_log(log) {
                return Ok(PortfolioEvents::TransferSingleFilter(decoded));
            }
            if let Ok(decoded) = UriFilter::decode_log(log) {
                return Ok(PortfolioEvents::UriFilter(decoded));
            }
            if let Ok(decoded) = UpdateProtocolFeeFilter::decode_log(log) {
                return Ok(PortfolioEvents::UpdateProtocolFeeFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for PortfolioEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AllocateFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ApprovalForAllFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ChangeParametersFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ClaimFeesFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::CreatePairFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::CreatePoolFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeallocateFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::DecreaseReserveBalanceFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DepositFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::IncreaseReserveBalanceFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SwapFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferBatchFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TransferSingleFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UriFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateProtocolFeeFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<AllocateFilter> for PortfolioEvents {
        fn from(value: AllocateFilter) -> Self {
            Self::AllocateFilter(value)
        }
    }
    impl ::core::convert::From<ApprovalForAllFilter> for PortfolioEvents {
        fn from(value: ApprovalForAllFilter) -> Self {
            Self::ApprovalForAllFilter(value)
        }
    }
    impl ::core::convert::From<ChangeParametersFilter> for PortfolioEvents {
        fn from(value: ChangeParametersFilter) -> Self {
            Self::ChangeParametersFilter(value)
        }
    }
    impl ::core::convert::From<ClaimFeesFilter> for PortfolioEvents {
        fn from(value: ClaimFeesFilter) -> Self {
            Self::ClaimFeesFilter(value)
        }
    }
    impl ::core::convert::From<CreatePairFilter> for PortfolioEvents {
        fn from(value: CreatePairFilter) -> Self {
            Self::CreatePairFilter(value)
        }
    }
    impl ::core::convert::From<CreatePoolFilter> for PortfolioEvents {
        fn from(value: CreatePoolFilter) -> Self {
            Self::CreatePoolFilter(value)
        }
    }
    impl ::core::convert::From<DeallocateFilter> for PortfolioEvents {
        fn from(value: DeallocateFilter) -> Self {
            Self::DeallocateFilter(value)
        }
    }
    impl ::core::convert::From<DecreaseReserveBalanceFilter> for PortfolioEvents {
        fn from(value: DecreaseReserveBalanceFilter) -> Self {
            Self::DecreaseReserveBalanceFilter(value)
        }
    }
    impl ::core::convert::From<DepositFilter> for PortfolioEvents {
        fn from(value: DepositFilter) -> Self {
            Self::DepositFilter(value)
        }
    }
    impl ::core::convert::From<IncreaseReserveBalanceFilter> for PortfolioEvents {
        fn from(value: IncreaseReserveBalanceFilter) -> Self {
            Self::IncreaseReserveBalanceFilter(value)
        }
    }
    impl ::core::convert::From<SwapFilter> for PortfolioEvents {
        fn from(value: SwapFilter) -> Self {
            Self::SwapFilter(value)
        }
    }
    impl ::core::convert::From<TransferBatchFilter> for PortfolioEvents {
        fn from(value: TransferBatchFilter) -> Self {
            Self::TransferBatchFilter(value)
        }
    }
    impl ::core::convert::From<TransferSingleFilter> for PortfolioEvents {
        fn from(value: TransferSingleFilter) -> Self {
            Self::TransferSingleFilter(value)
        }
    }
    impl ::core::convert::From<UriFilter> for PortfolioEvents {
        fn from(value: UriFilter) -> Self {
            Self::UriFilter(value)
        }
    }
    impl ::core::convert::From<UpdateProtocolFeeFilter> for PortfolioEvents {
        fn from(value: UpdateProtocolFeeFilter) -> Self {
            Self::UpdateProtocolFeeFilter(value)
        }
    }
    ///Container type for all input parameters for the `POSITION_RENDERER` function with signature `POSITION_RENDERER()` and selector `0xb0c3a950`
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
    #[ethcall(name = "POSITION_RENDERER", abi = "POSITION_RENDERER()")]
    pub struct PositionRendererCall;
    ///Container type for all input parameters for the `REGISTRY` function with signature `REGISTRY()` and selector `0x06433b1b`
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
    #[ethcall(name = "REGISTRY", abi = "REGISTRY()")]
    pub struct RegistryCall;
    ///Container type for all input parameters for the `VERSION` function with signature `VERSION()` and selector `0xffa1ad74`
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
    #[ethcall(name = "VERSION", abi = "VERSION()")]
    pub struct VersionCall;
    ///Container type for all input parameters for the `WETH` function with signature `WETH()` and selector `0xad5c4648`
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
    #[ethcall(name = "WETH", abi = "WETH()")]
    pub struct WethCall;
    ///Container type for all input parameters for the `allocate` function with signature `allocate(bool,address,uint64,uint128,uint128,uint128)` and selector `0x2f9e38e2`
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
        name = "allocate",
        abi = "allocate(bool,address,uint64,uint128,uint128,uint128)"
    )]
    pub struct AllocateCall {
        pub use_max: bool,
        pub recipient: ::ethers::core::types::Address,
        pub pool_id: u64,
        pub delta_liquidity: u128,
        pub max_delta_asset: u128,
        pub max_delta_quote: u128,
    }
    ///Container type for all input parameters for the `balanceOf` function with signature `balanceOf(address,uint256)` and selector `0x00fdd58e`
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
    #[ethcall(name = "balanceOf", abi = "balanceOf(address,uint256)")]
    pub struct BalanceOfCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all input parameters for the `balanceOfBatch` function with signature `balanceOfBatch(address[],uint256[])` and selector `0x4e1273f4`
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
    #[ethcall(name = "balanceOfBatch", abi = "balanceOfBatch(address[],uint256[])")]
    pub struct BalanceOfBatchCall {
        pub owners: ::std::vec::Vec<::ethers::core::types::Address>,
        pub ids: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///Container type for all input parameters for the `changeParameters` function with signature `changeParameters(uint64,uint16,uint16)` and selector `0x8a678967`
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
    #[ethcall(name = "changeParameters", abi = "changeParameters(uint64,uint16,uint16)")]
    pub struct ChangeParametersCall {
        pub pool_id: u64,
        pub priority_fee: u16,
        pub fee: u16,
    }
    ///Container type for all input parameters for the `claimFee` function with signature `claimFee(address,uint256)` and selector `0xdda40797`
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
    #[ethcall(name = "claimFee", abi = "claimFee(address,uint256)")]
    pub struct ClaimFeeCall {
        pub token: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `createPair` function with signature `createPair(address,address)` and selector `0xc9c65396`
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
    #[ethcall(name = "createPair", abi = "createPair(address,address)")]
    pub struct CreatePairCall {
        pub asset: ::ethers::core::types::Address,
        pub quote: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `createPool` function with signature `createPool(uint24,uint256,uint256,uint16,uint16,address,address,bytes)` and selector `0x267a0cfe`
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
        name = "createPool",
        abi = "createPool(uint24,uint256,uint256,uint16,uint16,address,address,bytes)"
    )]
    pub struct CreatePoolCall {
        pub pair_id: u32,
        pub reserve_x_per_wad: ::ethers::core::types::U256,
        pub reserve_y_per_wad: ::ethers::core::types::U256,
        pub fee_basis_points: u16,
        pub priority_fee_basis_points: u16,
        pub controller: ::ethers::core::types::Address,
        pub strategy: ::ethers::core::types::Address,
        pub strategy_args: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `deallocate` function with signature `deallocate(bool,uint64,uint128,uint128,uint128)` and selector `0x5bc55464`
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
        name = "deallocate",
        abi = "deallocate(bool,uint64,uint128,uint128,uint128)"
    )]
    pub struct DeallocateCall {
        pub use_max: bool,
        pub pool_id: u64,
        pub delta_liquidity: u128,
        pub min_delta_asset: u128,
        pub min_delta_quote: u128,
    }
    ///Container type for all input parameters for the `getAmountOut` function with signature `getAmountOut(uint64,bool,uint256,address)` and selector `0x19057807`
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
    #[ethcall(name = "getAmountOut", abi = "getAmountOut(uint64,bool,uint256,address)")]
    pub struct GetAmountOutCall {
        pub pool_id: u64,
        pub sell_asset: bool,
        pub amount_in: ::ethers::core::types::U256,
        pub swapper: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getInvariant` function with signature `getInvariant(uint64)` and selector `0x39434d5a`
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
    #[ethcall(name = "getInvariant", abi = "getInvariant(uint64)")]
    pub struct GetInvariantCall {
        pub pool_id: u64,
    }
    ///Container type for all input parameters for the `getLiquidityDeltas` function with signature `getLiquidityDeltas(uint64,int128)` and selector `0x8992f20a`
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
    #[ethcall(name = "getLiquidityDeltas", abi = "getLiquidityDeltas(uint64,int128)")]
    pub struct GetLiquidityDeltasCall {
        pub pool_id: u64,
        pub delta_liquidity: i128,
    }
    ///Container type for all input parameters for the `getMaxLiquidity` function with signature `getMaxLiquidity(uint64,uint256,uint256)` and selector `0xd6b7dec5`
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
    #[ethcall(name = "getMaxLiquidity", abi = "getMaxLiquidity(uint64,uint256,uint256)")]
    pub struct GetMaxLiquidityCall {
        pub pool_id: u64,
        pub amount_0: ::ethers::core::types::U256,
        pub amount_1: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getMaxOrder` function with signature `getMaxOrder(uint64,bool,address)` and selector `0xf07b879e`
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
    #[ethcall(name = "getMaxOrder", abi = "getMaxOrder(uint64,bool,address)")]
    pub struct GetMaxOrderCall {
        pub pool_id: u64,
        pub sell_asset: bool,
        pub swapper: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getNetBalance` function with signature `getNetBalance(address)` and selector `0x4dc68a90`
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
    #[ethcall(name = "getNetBalance", abi = "getNetBalance(address)")]
    pub struct GetNetBalanceCall {
        pub token: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getPairId` function with signature `getPairId(address,address)` and selector `0x3f92a339`
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
    #[ethcall(name = "getPairId", abi = "getPairId(address,address)")]
    pub struct GetPairIdCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
    );
    ///Container type for all input parameters for the `getPairNonce` function with signature `getPairNonce()` and selector `0x078888d6`
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
    #[ethcall(name = "getPairNonce", abi = "getPairNonce()")]
    pub struct GetPairNonceCall;
    ///Container type for all input parameters for the `getPoolNonce` function with signature `getPoolNonce(uint24)` and selector `0xa5cd8a49`
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
    #[ethcall(name = "getPoolNonce", abi = "getPoolNonce(uint24)")]
    pub struct GetPoolNonceCall(pub u32);
    ///Container type for all input parameters for the `getPoolReserves` function with signature `getPoolReserves(uint64)` and selector `0x2afb9df8`
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
    #[ethcall(name = "getPoolReserves", abi = "getPoolReserves(uint64)")]
    pub struct GetPoolReservesCall {
        pub pool_id: u64,
    }
    ///Container type for all input parameters for the `getReserve` function with signature `getReserve(address)` and selector `0xc9a396e9`
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
    #[ethcall(name = "getReserve", abi = "getReserve(address)")]
    pub struct GetReserveCall {
        pub token: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getSpotPrice` function with signature `getSpotPrice(uint64)` and selector `0xe331ba34`
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
    #[ethcall(name = "getSpotPrice", abi = "getSpotPrice(uint64)")]
    pub struct GetSpotPriceCall {
        pub pool_id: u64,
    }
    ///Container type for all input parameters for the `getStrategy` function with signature `getStrategy(uint64)` and selector `0x30244be7`
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
    #[ethcall(name = "getStrategy", abi = "getStrategy(uint64)")]
    pub struct GetStrategyCall {
        pub pool_id: u64,
    }
    ///Container type for all input parameters for the `isApprovedForAll` function with signature `isApprovedForAll(address,address)` and selector `0xe985e9c5`
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
    #[ethcall(name = "isApprovedForAll", abi = "isApprovedForAll(address,address)")]
    pub struct IsApprovedForAllCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
    );
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
    ///Container type for all input parameters for the `pairs` function with signature `pairs(uint24)` and selector `0x5e47663c`
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
    #[ethcall(name = "pairs", abi = "pairs(uint24)")]
    pub struct PairsCall(pub u32);
    ///Container type for all input parameters for the `pools` function with signature `pools(uint64)` and selector `0x89a5f084`
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
    #[ethcall(name = "pools", abi = "pools(uint64)")]
    pub struct PoolsCall(pub u64);
    ///Container type for all input parameters for the `protocolFee` function with signature `protocolFee()` and selector `0xb0e21e8a`
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
    #[ethcall(name = "protocolFee", abi = "protocolFee()")]
    pub struct ProtocolFeeCall;
    ///Container type for all input parameters for the `protocolFees` function with signature `protocolFees(address)` and selector `0xdcf844a7`
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
    #[ethcall(name = "protocolFees", abi = "protocolFees(address)")]
    pub struct ProtocolFeesCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `safeBatchTransferFrom` function with signature `safeBatchTransferFrom(address,address,uint256[],uint256[],bytes)` and selector `0x2eb2c2d6`
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
        name = "safeBatchTransferFrom",
        abi = "safeBatchTransferFrom(address,address,uint256[],uint256[],bytes)"
    )]
    pub struct SafeBatchTransferFromCall {
        pub from: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub ids: ::std::vec::Vec<::ethers::core::types::U256>,
        pub amounts: ::std::vec::Vec<::ethers::core::types::U256>,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `safeTransferFrom` function with signature `safeTransferFrom(address,address,uint256,uint256,bytes)` and selector `0xf242432a`
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
        name = "safeTransferFrom",
        abi = "safeTransferFrom(address,address,uint256,uint256,bytes)"
    )]
    pub struct SafeTransferFromCall {
        pub from: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub id: ::ethers::core::types::U256,
        pub amount: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `setApprovalForAll` function with signature `setApprovalForAll(address,bool)` and selector `0xa22cb465`
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
    #[ethcall(name = "setApprovalForAll", abi = "setApprovalForAll(address,bool)")]
    pub struct SetApprovalForAllCall {
        pub operator: ::ethers::core::types::Address,
        pub approved: bool,
    }
    ///Container type for all input parameters for the `setProtocolFee` function with signature `setProtocolFee(uint256)` and selector `0x787dce3d`
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
    #[ethcall(name = "setProtocolFee", abi = "setProtocolFee(uint256)")]
    pub struct SetProtocolFeeCall {
        pub fee: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `simulateSwap` function with signature `simulateSwap((uint128,uint128,bool,uint64,bool),uint256,address)` and selector `0x80af9d76`
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
        name = "simulateSwap",
        abi = "simulateSwap((uint128,uint128,bool,uint64,bool),uint256,address)"
    )]
    pub struct SimulateSwapCall {
        pub order: Order,
        pub timestamp: ::ethers::core::types::U256,
        pub swapper: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `0x01ffc9a7`
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
    #[ethcall(name = "supportsInterface", abi = "supportsInterface(bytes4)")]
    pub struct SupportsInterfaceCall {
        pub interface_id: [u8; 4],
    }
    ///Container type for all input parameters for the `swap` function with signature `swap((uint128,uint128,bool,uint64,bool))` and selector `0xf33ae1bc`
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
    #[ethcall(name = "swap", abi = "swap((uint128,uint128,bool,uint64,bool))")]
    pub struct SwapCall {
        pub args: Order,
    }
    ///Container type for all input parameters for the `uri` function with signature `uri(uint256)` and selector `0x0e89341c`
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
    #[ethcall(name = "uri", abi = "uri(uint256)")]
    pub struct UriCall {
        pub id: ::ethers::core::types::U256,
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
    pub enum PortfolioCalls {
        PositionRenderer(PositionRendererCall),
        Registry(RegistryCall),
        Version(VersionCall),
        Weth(WethCall),
        Allocate(AllocateCall),
        BalanceOf(BalanceOfCall),
        BalanceOfBatch(BalanceOfBatchCall),
        ChangeParameters(ChangeParametersCall),
        ClaimFee(ClaimFeeCall),
        CreatePair(CreatePairCall),
        CreatePool(CreatePoolCall),
        Deallocate(DeallocateCall),
        GetAmountOut(GetAmountOutCall),
        GetInvariant(GetInvariantCall),
        GetLiquidityDeltas(GetLiquidityDeltasCall),
        GetMaxLiquidity(GetMaxLiquidityCall),
        GetMaxOrder(GetMaxOrderCall),
        GetNetBalance(GetNetBalanceCall),
        GetPairId(GetPairIdCall),
        GetPairNonce(GetPairNonceCall),
        GetPoolNonce(GetPoolNonceCall),
        GetPoolReserves(GetPoolReservesCall),
        GetReserve(GetReserveCall),
        GetSpotPrice(GetSpotPriceCall),
        GetStrategy(GetStrategyCall),
        IsApprovedForAll(IsApprovedForAllCall),
        Multicall(MulticallCall),
        Pairs(PairsCall),
        Pools(PoolsCall),
        ProtocolFee(ProtocolFeeCall),
        ProtocolFees(ProtocolFeesCall),
        SafeBatchTransferFrom(SafeBatchTransferFromCall),
        SafeTransferFrom(SafeTransferFromCall),
        SetApprovalForAll(SetApprovalForAllCall),
        SetProtocolFee(SetProtocolFeeCall),
        SimulateSwap(SimulateSwapCall),
        SupportsInterface(SupportsInterfaceCall),
        Swap(SwapCall),
        Uri(UriCall),
    }
    impl ::ethers::core::abi::AbiDecode for PortfolioCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <PositionRendererCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PositionRenderer(decoded));
            }
            if let Ok(decoded) = <RegistryCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Registry(decoded));
            }
            if let Ok(decoded) = <VersionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Version(decoded));
            }
            if let Ok(decoded) = <WethCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Weth(decoded));
            }
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
            if let Ok(decoded) = <BalanceOfBatchCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BalanceOfBatch(decoded));
            }
            if let Ok(decoded) = <ChangeParametersCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ChangeParameters(decoded));
            }
            if let Ok(decoded) = <ClaimFeeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ClaimFee(decoded));
            }
            if let Ok(decoded) = <CreatePairCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CreatePair(decoded));
            }
            if let Ok(decoded) = <CreatePoolCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CreatePool(decoded));
            }
            if let Ok(decoded) = <DeallocateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Deallocate(decoded));
            }
            if let Ok(decoded) = <GetAmountOutCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetAmountOut(decoded));
            }
            if let Ok(decoded) = <GetInvariantCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetInvariant(decoded));
            }
            if let Ok(decoded) = <GetLiquidityDeltasCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetLiquidityDeltas(decoded));
            }
            if let Ok(decoded) = <GetMaxLiquidityCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetMaxLiquidity(decoded));
            }
            if let Ok(decoded) = <GetMaxOrderCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetMaxOrder(decoded));
            }
            if let Ok(decoded) = <GetNetBalanceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetNetBalance(decoded));
            }
            if let Ok(decoded) = <GetPairIdCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetPairId(decoded));
            }
            if let Ok(decoded) = <GetPairNonceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetPairNonce(decoded));
            }
            if let Ok(decoded) = <GetPoolNonceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetPoolNonce(decoded));
            }
            if let Ok(decoded) = <GetPoolReservesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetPoolReserves(decoded));
            }
            if let Ok(decoded) = <GetReserveCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetReserve(decoded));
            }
            if let Ok(decoded) = <GetSpotPriceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetSpotPrice(decoded));
            }
            if let Ok(decoded) = <GetStrategyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetStrategy(decoded));
            }
            if let Ok(decoded) = <IsApprovedForAllCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsApprovedForAll(decoded));
            }
            if let Ok(decoded) = <MulticallCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Multicall(decoded));
            }
            if let Ok(decoded) = <PairsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Pairs(decoded));
            }
            if let Ok(decoded) = <PoolsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Pools(decoded));
            }
            if let Ok(decoded) = <ProtocolFeeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ProtocolFee(decoded));
            }
            if let Ok(decoded) = <ProtocolFeesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ProtocolFees(decoded));
            }
            if let Ok(decoded) = <SafeBatchTransferFromCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SafeBatchTransferFrom(decoded));
            }
            if let Ok(decoded) = <SafeTransferFromCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SafeTransferFrom(decoded));
            }
            if let Ok(decoded) = <SetApprovalForAllCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetApprovalForAll(decoded));
            }
            if let Ok(decoded) = <SetProtocolFeeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetProtocolFee(decoded));
            }
            if let Ok(decoded) = <SimulateSwapCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SimulateSwap(decoded));
            }
            if let Ok(decoded) = <SupportsInterfaceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SupportsInterface(decoded));
            }
            if let Ok(decoded) = <SwapCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Swap(decoded));
            }
            if let Ok(decoded) = <UriCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Uri(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for PortfolioCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::PositionRenderer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Registry(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Version(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Weth(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Allocate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BalanceOf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BalanceOfBatch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ChangeParameters(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ClaimFee(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CreatePair(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CreatePool(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Deallocate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetAmountOut(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetInvariant(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetLiquidityDeltas(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetMaxLiquidity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetMaxOrder(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetNetBalance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetPairId(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetPairNonce(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetPoolNonce(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetPoolReserves(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetReserve(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetSpotPrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetStrategy(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsApprovedForAll(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Multicall(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Pairs(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Pools(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ProtocolFee(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProtocolFees(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SafeBatchTransferFrom(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SafeTransferFrom(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetApprovalForAll(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetProtocolFee(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SimulateSwap(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SupportsInterface(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Swap(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Uri(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for PortfolioCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::PositionRenderer(element) => ::core::fmt::Display::fmt(element, f),
                Self::Registry(element) => ::core::fmt::Display::fmt(element, f),
                Self::Version(element) => ::core::fmt::Display::fmt(element, f),
                Self::Weth(element) => ::core::fmt::Display::fmt(element, f),
                Self::Allocate(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceOfBatch(element) => ::core::fmt::Display::fmt(element, f),
                Self::ChangeParameters(element) => ::core::fmt::Display::fmt(element, f),
                Self::ClaimFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::CreatePair(element) => ::core::fmt::Display::fmt(element, f),
                Self::CreatePool(element) => ::core::fmt::Display::fmt(element, f),
                Self::Deallocate(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetAmountOut(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetInvariant(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetLiquidityDeltas(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetMaxLiquidity(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetMaxOrder(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetNetBalance(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPairId(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPairNonce(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPoolNonce(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPoolReserves(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetReserve(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSpotPrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetStrategy(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsApprovedForAll(element) => ::core::fmt::Display::fmt(element, f),
                Self::Multicall(element) => ::core::fmt::Display::fmt(element, f),
                Self::Pairs(element) => ::core::fmt::Display::fmt(element, f),
                Self::Pools(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProtocolFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProtocolFees(element) => ::core::fmt::Display::fmt(element, f),
                Self::SafeBatchTransferFrom(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SafeTransferFrom(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetApprovalForAll(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetProtocolFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::SimulateSwap(element) => ::core::fmt::Display::fmt(element, f),
                Self::SupportsInterface(element) => ::core::fmt::Display::fmt(element, f),
                Self::Swap(element) => ::core::fmt::Display::fmt(element, f),
                Self::Uri(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<PositionRendererCall> for PortfolioCalls {
        fn from(value: PositionRendererCall) -> Self {
            Self::PositionRenderer(value)
        }
    }
    impl ::core::convert::From<RegistryCall> for PortfolioCalls {
        fn from(value: RegistryCall) -> Self {
            Self::Registry(value)
        }
    }
    impl ::core::convert::From<VersionCall> for PortfolioCalls {
        fn from(value: VersionCall) -> Self {
            Self::Version(value)
        }
    }
    impl ::core::convert::From<WethCall> for PortfolioCalls {
        fn from(value: WethCall) -> Self {
            Self::Weth(value)
        }
    }
    impl ::core::convert::From<AllocateCall> for PortfolioCalls {
        fn from(value: AllocateCall) -> Self {
            Self::Allocate(value)
        }
    }
    impl ::core::convert::From<BalanceOfCall> for PortfolioCalls {
        fn from(value: BalanceOfCall) -> Self {
            Self::BalanceOf(value)
        }
    }
    impl ::core::convert::From<BalanceOfBatchCall> for PortfolioCalls {
        fn from(value: BalanceOfBatchCall) -> Self {
            Self::BalanceOfBatch(value)
        }
    }
    impl ::core::convert::From<ChangeParametersCall> for PortfolioCalls {
        fn from(value: ChangeParametersCall) -> Self {
            Self::ChangeParameters(value)
        }
    }
    impl ::core::convert::From<ClaimFeeCall> for PortfolioCalls {
        fn from(value: ClaimFeeCall) -> Self {
            Self::ClaimFee(value)
        }
    }
    impl ::core::convert::From<CreatePairCall> for PortfolioCalls {
        fn from(value: CreatePairCall) -> Self {
            Self::CreatePair(value)
        }
    }
    impl ::core::convert::From<CreatePoolCall> for PortfolioCalls {
        fn from(value: CreatePoolCall) -> Self {
            Self::CreatePool(value)
        }
    }
    impl ::core::convert::From<DeallocateCall> for PortfolioCalls {
        fn from(value: DeallocateCall) -> Self {
            Self::Deallocate(value)
        }
    }
    impl ::core::convert::From<GetAmountOutCall> for PortfolioCalls {
        fn from(value: GetAmountOutCall) -> Self {
            Self::GetAmountOut(value)
        }
    }
    impl ::core::convert::From<GetInvariantCall> for PortfolioCalls {
        fn from(value: GetInvariantCall) -> Self {
            Self::GetInvariant(value)
        }
    }
    impl ::core::convert::From<GetLiquidityDeltasCall> for PortfolioCalls {
        fn from(value: GetLiquidityDeltasCall) -> Self {
            Self::GetLiquidityDeltas(value)
        }
    }
    impl ::core::convert::From<GetMaxLiquidityCall> for PortfolioCalls {
        fn from(value: GetMaxLiquidityCall) -> Self {
            Self::GetMaxLiquidity(value)
        }
    }
    impl ::core::convert::From<GetMaxOrderCall> for PortfolioCalls {
        fn from(value: GetMaxOrderCall) -> Self {
            Self::GetMaxOrder(value)
        }
    }
    impl ::core::convert::From<GetNetBalanceCall> for PortfolioCalls {
        fn from(value: GetNetBalanceCall) -> Self {
            Self::GetNetBalance(value)
        }
    }
    impl ::core::convert::From<GetPairIdCall> for PortfolioCalls {
        fn from(value: GetPairIdCall) -> Self {
            Self::GetPairId(value)
        }
    }
    impl ::core::convert::From<GetPairNonceCall> for PortfolioCalls {
        fn from(value: GetPairNonceCall) -> Self {
            Self::GetPairNonce(value)
        }
    }
    impl ::core::convert::From<GetPoolNonceCall> for PortfolioCalls {
        fn from(value: GetPoolNonceCall) -> Self {
            Self::GetPoolNonce(value)
        }
    }
    impl ::core::convert::From<GetPoolReservesCall> for PortfolioCalls {
        fn from(value: GetPoolReservesCall) -> Self {
            Self::GetPoolReserves(value)
        }
    }
    impl ::core::convert::From<GetReserveCall> for PortfolioCalls {
        fn from(value: GetReserveCall) -> Self {
            Self::GetReserve(value)
        }
    }
    impl ::core::convert::From<GetSpotPriceCall> for PortfolioCalls {
        fn from(value: GetSpotPriceCall) -> Self {
            Self::GetSpotPrice(value)
        }
    }
    impl ::core::convert::From<GetStrategyCall> for PortfolioCalls {
        fn from(value: GetStrategyCall) -> Self {
            Self::GetStrategy(value)
        }
    }
    impl ::core::convert::From<IsApprovedForAllCall> for PortfolioCalls {
        fn from(value: IsApprovedForAllCall) -> Self {
            Self::IsApprovedForAll(value)
        }
    }
    impl ::core::convert::From<MulticallCall> for PortfolioCalls {
        fn from(value: MulticallCall) -> Self {
            Self::Multicall(value)
        }
    }
    impl ::core::convert::From<PairsCall> for PortfolioCalls {
        fn from(value: PairsCall) -> Self {
            Self::Pairs(value)
        }
    }
    impl ::core::convert::From<PoolsCall> for PortfolioCalls {
        fn from(value: PoolsCall) -> Self {
            Self::Pools(value)
        }
    }
    impl ::core::convert::From<ProtocolFeeCall> for PortfolioCalls {
        fn from(value: ProtocolFeeCall) -> Self {
            Self::ProtocolFee(value)
        }
    }
    impl ::core::convert::From<ProtocolFeesCall> for PortfolioCalls {
        fn from(value: ProtocolFeesCall) -> Self {
            Self::ProtocolFees(value)
        }
    }
    impl ::core::convert::From<SafeBatchTransferFromCall> for PortfolioCalls {
        fn from(value: SafeBatchTransferFromCall) -> Self {
            Self::SafeBatchTransferFrom(value)
        }
    }
    impl ::core::convert::From<SafeTransferFromCall> for PortfolioCalls {
        fn from(value: SafeTransferFromCall) -> Self {
            Self::SafeTransferFrom(value)
        }
    }
    impl ::core::convert::From<SetApprovalForAllCall> for PortfolioCalls {
        fn from(value: SetApprovalForAllCall) -> Self {
            Self::SetApprovalForAll(value)
        }
    }
    impl ::core::convert::From<SetProtocolFeeCall> for PortfolioCalls {
        fn from(value: SetProtocolFeeCall) -> Self {
            Self::SetProtocolFee(value)
        }
    }
    impl ::core::convert::From<SimulateSwapCall> for PortfolioCalls {
        fn from(value: SimulateSwapCall) -> Self {
            Self::SimulateSwap(value)
        }
    }
    impl ::core::convert::From<SupportsInterfaceCall> for PortfolioCalls {
        fn from(value: SupportsInterfaceCall) -> Self {
            Self::SupportsInterface(value)
        }
    }
    impl ::core::convert::From<SwapCall> for PortfolioCalls {
        fn from(value: SwapCall) -> Self {
            Self::Swap(value)
        }
    }
    impl ::core::convert::From<UriCall> for PortfolioCalls {
        fn from(value: UriCall) -> Self {
            Self::Uri(value)
        }
    }
    ///Container type for all return fields from the `POSITION_RENDERER` function with signature `POSITION_RENDERER()` and selector `0xb0c3a950`
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
    pub struct PositionRendererReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `REGISTRY` function with signature `REGISTRY()` and selector `0x06433b1b`
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
    pub struct RegistryReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `VERSION` function with signature `VERSION()` and selector `0xffa1ad74`
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
    pub struct VersionReturn(pub ::std::string::String);
    ///Container type for all return fields from the `WETH` function with signature `WETH()` and selector `0xad5c4648`
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
    ///Container type for all return fields from the `allocate` function with signature `allocate(bool,address,uint64,uint128,uint128,uint128)` and selector `0x2f9e38e2`
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
    pub struct AllocateReturn {
        pub delta_asset: ::ethers::core::types::U256,
        pub delta_quote: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `balanceOf` function with signature `balanceOf(address,uint256)` and selector `0x00fdd58e`
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
    pub struct BalanceOfReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `balanceOfBatch` function with signature `balanceOfBatch(address[],uint256[])` and selector `0x4e1273f4`
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
    pub struct BalanceOfBatchReturn {
        pub balances: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///Container type for all return fields from the `createPair` function with signature `createPair(address,address)` and selector `0xc9c65396`
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
    pub struct CreatePairReturn {
        pub pair_id: u32,
    }
    ///Container type for all return fields from the `createPool` function with signature `createPool(uint24,uint256,uint256,uint16,uint16,address,address,bytes)` and selector `0x267a0cfe`
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
    pub struct CreatePoolReturn {
        pub pool_id: u64,
    }
    ///Container type for all return fields from the `deallocate` function with signature `deallocate(bool,uint64,uint128,uint128,uint128)` and selector `0x5bc55464`
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
    pub struct DeallocateReturn {
        pub delta_asset: ::ethers::core::types::U256,
        pub delta_quote: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `getAmountOut` function with signature `getAmountOut(uint64,bool,uint256,address)` and selector `0x19057807`
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
    pub struct GetAmountOutReturn {
        pub output: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `getInvariant` function with signature `getInvariant(uint64)` and selector `0x39434d5a`
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
    pub struct GetInvariantReturn(pub ::ethers::core::types::I256);
    ///Container type for all return fields from the `getLiquidityDeltas` function with signature `getLiquidityDeltas(uint64,int128)` and selector `0x8992f20a`
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
    pub struct GetLiquidityDeltasReturn {
        pub delta_asset: u128,
        pub delta_quote: u128,
    }
    ///Container type for all return fields from the `getMaxLiquidity` function with signature `getMaxLiquidity(uint64,uint256,uint256)` and selector `0xd6b7dec5`
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
    pub struct GetMaxLiquidityReturn {
        pub delta_liquidity: u128,
    }
    ///Container type for all return fields from the `getMaxOrder` function with signature `getMaxOrder(uint64,bool,address)` and selector `0xf07b879e`
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
    pub struct GetMaxOrderReturn(pub Order);
    ///Container type for all return fields from the `getNetBalance` function with signature `getNetBalance(address)` and selector `0x4dc68a90`
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
    pub struct GetNetBalanceReturn(pub ::ethers::core::types::I256);
    ///Container type for all return fields from the `getPairId` function with signature `getPairId(address,address)` and selector `0x3f92a339`
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
    pub struct GetPairIdReturn(pub u32);
    ///Container type for all return fields from the `getPairNonce` function with signature `getPairNonce()` and selector `0x078888d6`
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
    pub struct GetPairNonceReturn(pub u32);
    ///Container type for all return fields from the `getPoolNonce` function with signature `getPoolNonce(uint24)` and selector `0xa5cd8a49`
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
    pub struct GetPoolNonceReturn(pub u32);
    ///Container type for all return fields from the `getPoolReserves` function with signature `getPoolReserves(uint64)` and selector `0x2afb9df8`
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
    pub struct GetPoolReservesReturn {
        pub delta_asset: ::ethers::core::types::U256,
        pub delta_quote: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `getReserve` function with signature `getReserve(address)` and selector `0xc9a396e9`
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
    pub struct GetReserveReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getSpotPrice` function with signature `getSpotPrice(uint64)` and selector `0xe331ba34`
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
    pub struct GetSpotPriceReturn {
        pub price: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `getStrategy` function with signature `getStrategy(uint64)` and selector `0x30244be7`
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
    pub struct GetStrategyReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `isApprovedForAll` function with signature `isApprovedForAll(address,address)` and selector `0xe985e9c5`
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
    pub struct IsApprovedForAllReturn(pub bool);
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
    pub struct MulticallReturn {
        pub results: ::std::vec::Vec<::ethers::core::types::Bytes>,
    }
    ///Container type for all return fields from the `pairs` function with signature `pairs(uint24)` and selector `0x5e47663c`
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
    pub struct PairsReturn {
        pub token_asset: ::ethers::core::types::Address,
        pub decimals_asset: u8,
        pub token_quote: ::ethers::core::types::Address,
        pub decimals_quote: u8,
    }
    ///Container type for all return fields from the `pools` function with signature `pools(uint64)` and selector `0x89a5f084`
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
        pub virtual_x: u128,
        pub virtual_y: u128,
        pub liquidity: u128,
        pub last_timestamp: u32,
        pub fee_basis_points: u16,
        pub priority_fee_basis_points: u16,
        pub controller: ::ethers::core::types::Address,
        pub strategy: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `protocolFee` function with signature `protocolFee()` and selector `0xb0e21e8a`
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
    pub struct ProtocolFeeReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `protocolFees` function with signature `protocolFees(address)` and selector `0xdcf844a7`
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
    pub struct ProtocolFeesReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `simulateSwap` function with signature `simulateSwap((uint128,uint128,bool,uint64,bool),uint256,address)` and selector `0x80af9d76`
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
    pub struct SimulateSwapReturn {
        pub success: bool,
        pub prev_invariant: ::ethers::core::types::I256,
        pub post_invariant: ::ethers::core::types::I256,
    }
    ///Container type for all return fields from the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `0x01ffc9a7`
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
    pub struct SupportsInterfaceReturn(pub bool);
    ///Container type for all return fields from the `swap` function with signature `swap((uint128,uint128,bool,uint64,bool))` and selector `0xf33ae1bc`
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
    pub struct SwapReturn {
        pub pool_id: u64,
        pub input: ::ethers::core::types::U256,
        pub output: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `uri` function with signature `uri(uint256)` and selector `0x0e89341c`
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
    pub struct UriReturn(pub ::std::string::String);
}
