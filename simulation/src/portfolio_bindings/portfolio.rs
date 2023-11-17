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
    const __BYTECODE: &[u8] = b"`\xE0`@\x90\x80\x82R4b\0\x01EWP\x80Q`\x1Fb\0n\x178\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x91`\x01`\x01`@\x1B\x03\x83\x11\x84\x84\x10\x17b\0\x01/W\x80\x84\x92``\x94\x87R\x839\x81\x01\x03\x12b\0\0\xE0Wb\0\0U\x81b\0\x01\x92V[\x90b\0\0q\x83b\0\0i` \x84\x01b\0\x01\x92V[\x92\x01b\0\x01\x92V[\x91`\x01`\x0CU`\x80R`\xA0R`\xC0R`\x01`\xFF\x19`\x05T\x16\x17`\x05UQalj\x90\x81b\0\x01\xAD\x829`\x80Q\x81\x81\x81a(#\x01R\x81\x81a<\xBC\x01R\x81\x81aK\xB9\x01RaR}\x01R`\xA0Q\x81\x81\x81a\x05W\x01R\x81\x81a\x1C\x8D\x01Ra-\xD1\x01R`\xC0Q\x81\x81\x81a\x064\x01Ra(g\x01R\xF3[\x81QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[bF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01Ra7\xB7`\xF1\x1B`d\x82\x01R`\x84\x90\xFD[Q\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03b\0\x01\xA7WV[`\0\x80\xFD\xFE`\x80`@R`\x046\x10\x15a\0\x1DW[6a<HWa\0\x1Ba<\xB2V[\0[`\x005`\xE0\x1C\x80b\xFD\xD5\x8E\x14a\x02\x8CW\x80c\x01\xFF\xC9\xA7\x14a\x02\x87W\x80c\x06C;\x1B\x14a\x02\x82W\x80c\x07\x88\x88\xD6\x14a\x02}W\x80c\x0E\x894\x1C\x14a\x02xW\x80c\x19\x05x\x07\x14a\x02sW\x80c&z\x0C\xFE\x14a\x02nW\x80c*\xFB\x9D\xF8\x14a\x02iW\x80c.\xB2\xC2\xD6\x14a\x02dW\x80c/\x9E8\xE2\x14a\x02_W\x80c0$K\xE7\x14a\x02ZW\x80c9CMZ\x14a\x02UW\x80c?\x92\xA39\x14a\x02PW\x80cM\xC6\x8A\x90\x14a\x02KW\x80cN\x12s\xF4\x14a\x02FW\x80c[\xC5Td\x14a\x02AW\x80c^Gf<\x14a\x02<W\x80cx}\xCE=\x14a\x027W\x80c\x80\xAF\x9Dv\x14a\x022W\x80c\x89\x92\xF2\n\x14a\x02-W\x80c\x89\xA5\xF0\x84\x14a\x02(W\x80c\x8Ag\x89g\x14a\x02#W\x80c\xA2,\xB4e\x14a\x02\x1EW\x80c\xA5\xCD\x8AI\x14a\x02\x19W\x80c\xAC\x96P\xD8\x14a\x02\x14W\x80c\xAD\\FH\x14a\x02\x0FW\x80c\xB0\xC3\xA9P\x14a\x02\nW\x80c\xB0\xE2\x1E\x8A\x14a\x02\x05W\x80c\xC9\xA3\x96\xE9\x14a\x02\0W\x80c\xC9\xC6S\x96\x14a\x01\xFBW\x80c\xD6\xB7\xDE\xC5\x14a\x01\xF6W\x80c\xDC\xF8D\xA7\x14a\x01\xF1W\x80c\xDD\xA4\x07\x97\x14a\x01\xECW\x80c\xE31\xBA4\x14a\x01\xE7W\x80c\xE9\x85\xE9\xC5\x14a\x01\xE2W\x80c\xF0{\x87\x9E\x14a\x01\xDDW\x80c\xF2BC*\x14a\x01\xD8W\x80c\xF3:\xE1\xBC\x14a\x01\xD3Wc\xFF\xA1\xADt\x03a\0\x0EWa<\x1CV[a3\xC8V[a2%V[a0\xEEV[a0DV[a/\xA7V[a-\x9DV[a-_V[a,wV[a(\xE7V[a(\xA9V[a(\x8BV[a(GV[a(\x03V[a&\x9CV[a%\xE0V[a%%V[a\"\xE2V[a!\xFDV[a \xD9V[a\x1F\xB8V[a\x1CfV[a\x1C\x02V[a\x17\xF3V[a\x16\xDAV[a\x16nV[a\x16\x1FV[a\x15]V[a\x15\x03V[a\x0F^V[a\x0C\x95V[a\x0B@V[a\n\xA3V[a\x07\xC9V[a\x06\x18V[a\x05{V[a\x057V[a\x04eV[a\x03\xE5V[`\x84`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R\xFD[`\x84`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01R\x7Frt\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R\xFD[`\x84`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01R\x7Fet\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\x03\xE0WV[`\0\x80\xFD[4a\x046W`@`\x03\x196\x01\x12a\x041W`\x01`\x01`\xA0\x1B\x03`\x045a\x04\n\x81a\x03\xCFV[\x16`\0R`\0` R`@`\0 `$5`\0R` R` `@`\0 T`@Q\x90\x81R\xF3[a\x02\xFBV[a\x02\x91V[\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x03a\x03\xE0WV[4a\x046W` `\x03\x196\x01\x12a\x041W` \x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x045a\x04\xA5\x81a\x04;V[\x16\x7F\x01\xFF\xC9\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x14\x90\x81\x15a\x05\rW[\x81\x15a\x04\xE3W[P`@Q\x90\x15\x15\x81R\xF3[\x7F\x0E\x894\x1C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91P\x148a\x04\xD8V[\x7F\xD9\xB6z&\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x14\x91Pa\x04\xD1V[4a\x046W`\0`\x03\x196\x01\x12a\x041W` `@Q`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[4a\x046W`\0`\x03\x196\x01\x12a\x041W` b\xFF\xFF\xFF`\x06T\x16`@Q\x90\x81R\xF3[`\0[\x83\x81\x10a\x05\xB1WPP`\0\x91\x01RV[\x81\x81\x01Q\x83\x82\x01R` \x01a\x05\xA1V[\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F` \x93a\x05\xFD\x81Q\x80\x92\x81\x87R\x87\x80\x88\x01\x91\x01a\x05\x9EV[\x01\x16\x01\x01\x90V[\x90` a\x06\x15\x92\x81\x81R\x01\x90a\x05\xC1V[\x90V[4a\x046W` \x80`\x03\x196\x01\x12a\x041W`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x80;\x15a\x07\xA8W`\0`$\x91`@Q\x92\x83\x80\x92\x7F\x0E\x894\x1C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x045`\x04\x83\x01RZ\xFA\x90\x81\x15a\x07\xA3W`\0\x91a\x06\xB5W[`@Q\x80a\x06\xB1\x84\x82a\x06\x04V[\x03\x90\xF3[\x90=\x80\x91\x83>a\x06\xC5\x81\x83a\x1E\x83V[\x81\x01\x90\x82\x81\x83\x03\x12a\x041W\x80Q\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x07\x9EW\x01\x81`\x1F\x82\x01\x12\x15a\x07\x99W\x80Q\x90a\x06\xFB\x82a=NV[\x92a\x07\t`@Q\x94\x85a\x1E\x83V[\x82\x84R\x84\x83\x83\x01\x01\x11a\x07/W\x90a\x07)\x91\x84a\x06\xB1\x95\x85\x01\x91\x01a\x05\x9EV[8a\x06\xA3V[`\x84\x84`@Q\x90bF\x1B\xCD`\xE5\x1B\x82R`\x04\x82\x01R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01R\x7F length\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R\xFD[a\t-V[a\x03eV[a=\x88V[a<\xE4V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x03a\x03\xE0WV[\x80\x15\x15\x03a\x03\xE0WV[4a\x046W`\x80`\x03\x196\x01\x12a\x041W`\x045a\x07\xE6\x81a\x07\xADV[`$5a\x07\xF2\x81a\x07\xBFV[`d5a\x07\xFE\x81a\x03\xCFV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16`\0R`\n` Ra\x08<a\x080a\x080`\x03`@`\0 \x01`\x01`\x01`\xA0\x1B\x03\x90T\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x80;\x15a\x07\xA8W`@Q\x7F\x19\x05x\x07\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x94\x16`\x04\x85\x01R\x91\x15\x15`$\x84\x01R`D\x805\x90\x84\x01R`\x01`\x01`\xA0\x1B\x03\x16`d\x83\x01R` \x90\x82\x90\x81\x80`\x84\x81\x01[\x03\x91Z\xFA\x80\x15a\x07\xA3Wa\x06\xB1\x91`\0\x91a\x08\xCBW[P`@Q\x90\x81R\x90\x81\x90` \x82\x01\x90V[a\x08\xEC\x91P` =\x81\x11a\x08\xF2W[a\x08\xE4\x81\x83a\x1E\x83V[\x81\x01\x90aTeV[8a\x08\xBAV[P=a\x08\xDAV[`\x045\x90b\xFF\xFF\xFF\x82\x16\x82\x03a\x03\xE0WV[`$5\x90a\xFF\xFF\x82\x16\x82\x03a\x03\xE0WV[`D5\x90a\xFF\xFF\x82\x16\x82\x03a\x03\xE0WV[`\x84`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01R\x7Frray offset\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R\xFD[`\x84`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01R\x7Frray length\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R\xFD[`\x84`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01R\x7Frray stride\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R\xFD[\x91\x81`\x1F\x84\x01\x12\x15a\x07\x99W\x825\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\n\x9EW` \x83\x81\x86\x01\x95\x01\x01\x11a\n\x99WV[a\n\x01V[a\t\x97V[a\x01\0`\x03\x196\x01\x12a\x041Wa\n\xB8a\x08\xF9V[a\xFF\xFF\x90`d5\x82\x81\x16\x81\x03a\x03\xE0W`\x845\x92\x83\x16\x83\x03a\x03\xE0W`\xA45\x91a\n\xE1\x83a\x03\xCFV[`\xC45a\n\xED\x81a\x03\xCFV[`\xE45\x93g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x11a\x07\x9EWa\x06\xB1\x95a\x0B\x15a\x0B%\x966\x90`\x04\x01a\nkV[\x95\x90\x94`D5\x90`$5\x90aH\xBAV[`@Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R\x90\x81\x90` \x82\x01\x90V[4a\x046W` `\x03\x196\x01\x12a\x041W`\x045a\x0B]\x81a\x07\xADV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16`\0R`\n` Ra\x0B}`@`\0 aAOV[`@\x81\x01\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFo\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x84Q\x16\x11a\x0C:Wa\x0C\x18a\x0C\x12``a\x0C\x1Fa\x0B\xFFa\x0B\xFAa\x0B\xDEb\xFF\xFF\xFF\x98a\x0B\xD8\x89a\x0C(\x9CQ\x16`\x0F\x0BaB&V[\x90ab\x86V[\x98\x90\x9A` \x1C\x16b\xFF\xFF\xFF\x16`\0R`\t` R`@`\0 \x90V[aA\x0CV[\x97\x85a\x0C\x18a\x0C\x12` \x8C\x01Q`\xFF\x16\x90V[`\xFF\x16\x90V[\x91\x16aZ\xEBV[\x96\x01Q`\xFF\x16\x90V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x90\xF3[`\x04`@Q\x7F\xAC\xC9@{\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xFD[\x91\x81`\x1F\x84\x01\x12\x15a\x07\x99W\x825\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\n\x9EW` \x80\x85\x01\x94\x84`\x05\x1B\x01\x01\x11a\n\x99WV[4a\x046W`\xA0`\x03\x196\x01\x12a\x041W`\x045a\x0C\xB2\x81a\x03\xCFV[`$5\x90a\x0C\xBF\x82a\x03\xCFV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91`D5\x83\x81\x11a\x07\x9EWa\x0C\xE1\x906\x90`\x04\x01a\x0CdV[\x93\x90\x91`d5\x82\x81\x11a\x07\x9EWa\x0C\xFC\x906\x90`\x04\x01a\x0CdV[\x95\x90\x92`\x845\x90\x81\x11a\x07\x9EWa\r\x17\x906\x90`\x04\x01a\nkV[\x90\x94a\r$\x88\x84\x14ah\xA2V[`\x01`\x01`\xA0\x1B\x03\x93\x84\x88\x16\x94\x853\x14\x80\x15a\x0E\xF6W[a\rD\x90agwV[\x84`\0\x8A\x89\x86\x8E[\x85\x85\x10a\x0EeWPPPPPP\x81\x16\x80\x95\x8A\x7FJ9\xDC\x06\xD4\xC0\xDB\xC6Kp\xAF\x90\xFDi\x8A#:Q\x8A\xA5\xD0~Y]\x98;\x8C\x05&\xC8\xF7\xFB`@Q\x80a\r\x913\x94\x8D\x8C\x8B\x85ai\xA4V[\x03\x90\xA4;\x15a\x0ERW\x83;\x15a\x07\xA8Wa\r\xE3`\0\x92` \x97`@Q\x9A\x8B\x98\x89\x97\x88\x96\x7F\xBC\x19|\x81\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x9D\x8E\x89R3`\x04\x8A\x01ai\xCBV[\x03\x92Z\xF1\x91\x82\x15a\x07\xA3Wa\0\x1B\x92\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91`\0\x91a\x0E$W[P\x16\x14ahWV[a\x0EE\x91P` =\x81\x11a\x0EKW[a\x0E=\x81\x83a\x1E\x83V[\x81\x01\x90ah\rV[8a\x0E\x1CV[P=a\x0E3V[PPP\x92PPPa\0\x1B\x91P\x15\x15ag\xC2V[a\x0E\xB6a\x0E\xE8\x93a\x0E\xA7a\x0E\x8C\x88a\x0E\x83\x81`\x01\x9Ca\x0E\xE0\x99ah\xEDV[5\x95\x86\x94ah\xEDV[5\x96`\x01`\x01`\xA0\x1B\x03\x16`\0R`\0` R`@`\0 \x90V[\x90`\0R` R`@`\0 \x90V[a\x0E\xC1\x85\x82TaF\xE1V[\x90Ua\x0E\xA7\x88`\x01`\x01`\xA0\x1B\x03\x16`\0R`\0` R`@`\0 \x90V[\x91\x82TaF\xEEV[\x90U\x01\x85\x90\x8A\x89\x86\x8Ea\rLV[Pa\rDa\x0F=a\x0F63a\x0F\x1E\x8D`\x01`\x01`\xA0\x1B\x03\x16`\0R`\x01` R`@`\0 \x90V[\x90`\x01`\x01`\xA0\x1B\x03\x16`\0R` R`@`\0 \x90V[T`\xFF\x16\x90V[\x90Pa\r;V[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x03a\x03\xE0WV[`\xC0`\x03\x196\x01\x12a\x041W`\x04\x805\x90a\x0Fx\x82a\x07\xBFV[`$5a\x0F\x84\x81a\x03\xCFV[`D5\x90a\x0F\x91\x82a\x07\xADV[`d5\x90a\x0F\x9E\x82a\x0FDV[`\x845\x91a\x0F\xAB\x83a\x0FDV[`\xA45\x93a\x0F\xB8\x85a\x0FDV[\x90\x81a\x0F\xC2a@<V[`\x0FT`\xFF\x16\x15a\x14\xF6W[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x91\x16\x15a\x14\xDDW[`\x01`\x01`\xA0\x1B\x03\x97\x88a\x10\x1B`\x03a\x10\r\x87g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0R`\n` R`@`\0 \x90V[\x01T`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x16\x15a\x14\x9DWa\x10Ja\x080a\x080`\x03a\x10\r\x88g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0R`\n` R`@`\0 \x90V[\x95\x86;\x15a\x07\xA8W`@\x80Q\x7F\xE6\x04{\x19\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x81\x8C\x01\x90\x81R\x91\x98` \x93\x92\x84\x91\x83\x91\x82\x90\x81\x90\x85\x01\x03\x91Z\xFA\x90\x81\x15a\x07\xA3W`\0\x91a\x14pW[P\x15a\x14-Wa\x10\xD4\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x80\x9B\x16\x91\x16\x87aM\x9FV[a\x10\xF9a\x0B\xFAb\xFF\xFF\xFF\x89\x86\x95\x96\x1C\x16b\xFF\xFF\xFF\x16`\0R`\t` R`@`\0 \x90V[\x93a\x13\x99W[\x89\x86\x16\x15a\x13qW\x89\x90\x81\x80a\x11=a\x114a\x11/\x8Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0R`\n` R`@`\0 \x90V[aAOV[a\x0B\xD8\x8BaZ\x96V[\x9D\x90\x9D\x16\x9C\x16\x94\x16\x84\x11a\x13IW\x16\x89\x11a\x13!W\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x96\x88\x84\x01\x97\x88Qa\x11t\x90`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x90a\x11~\x88aZ\x96V[\x92a\x11\x87a\x1E\xC4V[\x93B\x85R\x86\x86\x86\x01R\x8D\x8D\x86\x01R``\x85\x01\x90a\x11\xA6\x91\x90`\x0F\x0B\x90RV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8A\x16`\x80\x85\x01R`\x01`\x01`\xA0\x1B\x03\x16`\xA0\x84\x01R`\x01`\x01`\xA0\x1B\x03\x16`\xC0\x83\x01R`\x01`\x01`\xA0\x1B\x03\x16`\xE0\x82\x01Ra\x11\xE9\x90aC,V[\x82\x01Q`\xFF\x16`\xFF\x16a\x11\xFB\x91aZ\xEBV[\x96``\x82\x01Qa\x12\x0B\x90`\xFF\x16\x90V[`\xFF\x16a\x12\x17\x91aZ\xEBV[\x97\x87\x15\x80a\x13\x19W[a\x12\xF2WP\x97\x7F\xFD\xFF\xEC\xA7Q\xF0\xDC\xAA\xB7U1\xCB\x81<\x12\xBB\xFDV\xEE>\x96L\xC4q\xD7\xEFC\x93$\x02\xEE\x18\x91a\x12\xAAa\x12ra\x12da\x06\xB1\x9A\x9B\x9CQ`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x97Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x88Q\x8B\x81R` \x81\x01\x8D\x90Ro\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x96\x16`@\x87\x01R\x83\x16\x96\x90\x92\x16\x94\x16\x92\x90\x81\x90``\x82\x01\x90V[\x03\x90\xA4a\x12\xC1a\x12\xBC`\x0FT`\xFF\x16\x90V[\x15\x15\x90V[\x15a\x12\xE5W[a\x12\xCFa@\x9DV[Q\x91\x82R` \x82\x01\x92\x90\x92R\x90\x81\x90`@\x82\x01\x90V[a\x12\xEDaQ\xD9V[a\x12\xC7V[\x86Q\x7Fe\x8B\x16\xED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xFD[P\x88\x15a\x12 V[\x89\x88Q\x7F\x84\xC0Z\\\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xFD[\x8B\x8AQ\x7F\xAC\xE4\x1C:\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xFD[\x8A\x89Q\x7F\x90`\x9A}\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xFD[\x94Pa\x14\x16a\x13\xE9a\x13\xBAa\x13\xB5\x86Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[a=\x94V[a\x13\xD0a\x13\xB5\x8C\x88\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x90`\0\x81\x12a\x14%W[`\0\x82\x12a\x14\x1CW[\x89aM\x9FV[\x8B\x80a\x14\x0Ca\x11/\x8Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0R`\n` R`@`\0 \x90V[\x92\x16\x92\x16\x90aa\x17V[\x94a\x10\xFFV[`\0\x91Pa\x13\xE3V[P`\0a\x13\xDAV[\x87Q\x7F\xBC'\xA5\x17\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x81\x8C\x01\x90\x81R\x81\x90` \x01\x03\x90\xFD[\x03\x90\xFD[a\x14\x90\x91P\x83=\x85\x11a\x14\x96W[a\x14\x88\x81\x83a\x1E\x83V[\x81\x01\x90a@\xF7V[8a\x10\xADV[P=a\x14~V[`@Q\x7F\x12\x16\xE0\xFA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x81\x8A\x01\x90\x81R\x81\x90` \x01\x03\x90\xFD[`\x0FT\x90\x92P`\x08\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91a\x0F\xE0V[a\x14\xFEaK\xA8V[a\x0F\xCEV[4a\x046W` `\x03\x196\x01\x12a\x041W` a\x15L`\x045a\x15%\x81a\x07\xADV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0R`\n` R`\x01`\x01`\xA0\x1B\x03`\x03`@`\0 \x01T\x16\x90V[`\x01`\x01`\xA0\x1B\x03`@Q\x91\x16\x81R\xF3[4a\x046W` `\x03\x196\x01\x12a\x041W`\x045a\x15z\x81a\x07\xADV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16`\0R`\n` Ra\x15\xACa\x080a\x080`\x03`@`\0 \x01`\x01`\x01`\xA0\x1B\x03\x90T\x16\x90V[\x80;\x15a\x07\xA8W`@Q\x7F9CMZ\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16`\x04\x83\x01R` \x90\x82\x90\x81\x80`$\x81\x01a\x08\xA4V[`\x03\x19`@\x91\x01\x12a\x041W`\x045a\x16\x12\x81a\x03\xCFV[\x90`$5a\x06\x15\x81a\x03\xCFV[4a\x046W` b\xFF\xFF\xFFa\x16d`\x01`\x01`\xA0\x1B\x03a\x16>6a\x15\xFAV[\x91\x16`\0R`\x0B\x84R`@`\0 \x90`\x01`\x01`\xA0\x1B\x03\x16`\0R` R`@`\0 \x90V[T\x16`@Q\x90\x81R\xF3[4a\x046W` `\x03\x196\x01\x12a\x041W` a\x16\x97`\x045a\x16\x90\x81a\x03\xCFV[0\x90aY\xD5V[`@Q\x90\x81R\xF3[` \x90\x81`@\x81\x83\x01\x92\x82\x81R\x85Q\x80\x94R\x01\x93\x01\x91`\0[\x82\x81\x10a\x16\xC6WPPPP\x90V[\x83Q\x85R\x93\x81\x01\x93\x92\x81\x01\x92`\x01\x01a\x16\xB8V[4a\x046W`@\x80`\x03\x196\x01\x12a\x041Wg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90`\x045\x82\x81\x11a\x07\x9EWa\x17\x0E\x906\x90`\x04\x01a\x0CdV[\x91\x90\x92`$5\x90\x81\x11a\x07\x9EWa\x17)\x906\x90`\x04\x01a\x0CdV[\x93\x90a\x176\x85\x85\x14ah\xA2V[a\x17?\x84a=\x9FV[\x93a\x17L\x84Q\x95\x86a\x1E\x83V[\x80\x85R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0a\x17y\x82a=\x9FV[\x01` \x906\x82\x88\x017`\0\x91\x82[\x81\x81\x10a\x17\x9BW\x86Q\x80a\x06\xB1\x8A\x82a\x16\x9FV[\x80`\x01`\x01`\xA0\x1B\x03a\x17\xB1`\x01\x93\x85\x8Aah\xEDV[5a\x17\xBB\x81a\x03\xCFV[\x16\x85R\x84\x84Ra\x17\xE1\x88\x86 a\x17\xD2\x83\x8D\x8Aah\xEDV[5`\0R` R`@`\0 \x90V[Ta\x17\xEC\x82\x8Ba@(V[R\x01a\x17\x87V[`\xA0`\x03\x196\x01\x12a\x041W`\x04\x805a\x18\x0C\x81a\x07\xBFV[`$5\x91a\x18\x19\x83a\x07\xADV[`D5\x92a\x18&\x84a\x0FDV[`d5\x90a\x183\x82a\x0FDV[`\x845\x94a\x18@\x86a\x0FDV[\x90a\x18Ia@<V[`\x0FT`\xFF\x16\x15a\x1B\xF5W[a\x18~a\x080a\x080`\x03a\x10\r\x85g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0R`\n` R`@`\0 \x90V[\x91\x82;\x15a\x07\xA8W`@\x93\x84Q\x80\x94\x7F\xE6\x04{\x19\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R` \x95\x86\x91\x81\x80a\x18\xD4\x89\x8D\x83\x01\x91\x90\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x82\x01\x93\x16\x90RV[\x03\x91Z\xFA\x90\x81\x15a\x07\xA3W`\0\x91a\x1B\xD8W[P\x15a\x1B\x99Wa\x19\x0E\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x80\x9A\x16\x91\x16\x84aM\x9FV[\x93a\x192a\x0B\xFAb\xFF\xFF\xFF\x86\x84\x1C\x16b\xFF\xFF\xFF\x16`\0R`\t` R`@`\0 \x90V[\x96a\x19D\x88Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x95a\x19X\x88\x8A\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x99a\x1BNW[\x8A\x85\x16\x15a\x1B&W\x8A\x90\x81\x80a\x19\x9Fa\x19\x8Ea\x11/\x8Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0R`\n` R`@`\0 \x90V[a\x0B\xD8a\x19\x9A\x8BaZ\x96V[aB&V[\x9E\x90\x9E\x16\x9D\x16\x95\x16\x85\x10a\x1A\xFEW\x16\x8A\x10a\x1A\xD7WPa\x1A\x97\x89a\x1A\x88a\x0C\x12``a\x1A\x8E\x87\x8E\x9F\x8E\x9Fa\x06\xB1\x9F\x99\x8F\x9A\x8F\x8F\x8F\x7F0\x84\xCA\xF4\x89f\\\xAB\x07E,\xFEO=\x0E\xB5\xE0\xDC\x15\xEA\xC6\xFCe\x80\x98\x85\x8Ec\x9Ep\xE5:\x9F\x95g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x9Fa\x1A\x80\x94a\x1AU\x88\x95a\x1ADa\x0C\x12\x9Ba\x1A\x88\x9Da\x1Ak\x96a\x1A'a\x19\x9Aa\x1A{\x9BaZ\x96V[\x92a\x1A0a\x1E\xC4V[B\x81R\x9B\x8C\x01R\x8A\x01R`\x0F\x0B``\x89\x01RV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x80\x87\x01RV[3`\xA0\x86\x01R`\x01`\x01`\xA0\x1B\x03\x16`\xC0\x85\x01RV[`\x01`\x01`\xA0\x1B\x03\x16`\xE0\x83\x01RV[aC,V[\x01Q`\xFF\x16\x90V[\x90aZ\xEBV[\x9C\x01Q`\xFF\x16\x90V[\x86Q\x89\x81R` \x81\x01\x82\x90Ro\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x94\x16`@\x85\x01R\x98`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x95\x16\x93\x16\x91\x80``\x81\x01a\x12\xAAV[\x86Q\x7F\xAC\xB5\xBD\xEA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xFD[\x82\x89Q\x7FcD\x840\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xFD[P\x86Q\x7F\x14\xEF`^\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xFD[\x93Pa\x1B\x93a\x1B\x8D\x86a\x1Bt3`\x01`\x01`\xA0\x1B\x03\x16`\0R`\0` R`@`\0 \x90V[\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0R` R`@`\0 \x90V[Tal<V[\x93a\x19^V[\x84Q\x7F\x12\x16\xE0\xFA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x81\x88\x01\x90\x81R\x81\x90` \x01\x03\x90\xFD[a\x1B\xEF\x91P\x85=\x87\x11a\x14\x96Wa\x14\x88\x81\x83a\x1E\x83V[8a\x18\xE7V[a\x1B\xFDaK\xA8V[a\x18UV[4a\x046W` `\x03\x196\x01\x12a\x041Wb\xFF\xFF\xFFa\x1C\x1Fa\x08\xF9V[\x16`\0R`\t` R`\x80`@`\0 `\xFF`\x01\x82T\x92\x01T`@Q\x92\x82`\x01`\x01`\xA0\x1B\x03\x91\x82\x81\x16\x86R`\xA0\x1C\x16` \x85\x01R\x81\x16`@\x84\x01R`\xA0\x1C\x16``\x82\x01R\xF3[4a\x046W` `\x03\x196\x01\x12a\x041W`\x045a\x1C\x82a@<V[`\x01`\x01`\xA0\x1B\x03\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x80;\x15a\x07\xA8W` `\x04\x91`@Q\x92\x83\x80\x92\x7F\xF7|G\x91\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x90\x81\x15a\x07\xA3W`\0\x91a\x1D\xB8W[P\x163\x03a\x1D\x8EW`\x14\x81\x11\x80\x15a\x1D\x84W[a\x1DQW\x7F\x81\xC99\x14H\0(v\x03G\x9B\x97\xBB\xA9\xC1\x12\x88\xCEz\xBCZ\xCBH\x90y\xE1Y\xF3\\\xF9\x8B\xD1\x90`\rTa\x1D9\x82`\rUV[`@\x80Q\x91\x82R` \x82\x01\x92\x90\x92R\xA1a\0\x1Ba@\x9DV[`@Q\x7FdYtw\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x91\x90\x91R`$\x90\xFD[P`\x04\x81\x10a\x1D\x07V[`\x04`@Q\x7F\xFF\xBE\x9C,\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xFD[a\x1D\xD9\x91P` =\x81\x11a\x1D\xDFW[a\x1D\xD1\x81\x83a\x1E\x83V[\x81\x01\x90aTPV[8a\x1C\xF4V[P=a\x1D\xC7V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`\xA0\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x1E1W`@RV[a\x1D\xE6V[`\x80\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x1E1W`@RV[a\x01\0\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x1E1W`@RV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x1E1W`@RV[\x90`\x1F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x1E1W`@RV[`@Q\x90a\x1E\xD1\x82a\x1ERV[V[`@Q\x90a\x1E\xD1\x82a\x1E6V[`\x03\x19`\xA0\x91\x01\x12a\x1FNW`@Q\x90a\x1E\xF9\x82a\x1E\x15V[\x81`\x045a\x1F\x06\x81a\x0FDV[\x81R`$5a\x1F\x14\x81a\x0FDV[` \x82\x01R`D5a\x1F%\x81a\x07\xBFV[`@\x82\x01R`d5a\x1F6\x81a\x07\xADV[``\x82\x01R`\x80`\x845\x91a\x1FJ\x83a\x07\xBFV[\x01RV[`\x84`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FABI decoding: struct data too sh`D\x82\x01R\x7Fort\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R\xFD[4a\x046W`\xE0`\x03\x196\x01\x12a\x041Wa\x1F\xD26a\x1E\xE0V[`\xC45\x90a\x1F\xDF\x82a\x03\xCFV[`\x01`\x01`\xA0\x1B\x03a  g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF``\x84\x01Q\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0R`\n` R`\x01`\x01`\xA0\x1B\x03`\x03`@`\0 \x01T\x16\x90V[\x16\x91\x82;\x15a\x07\xA8Wa i\x92``\x92`@Q\x80\x95\x81\x94\x82\x93\x7F\x80\xAF\x9Dv\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\xA45\x90`\x04\x85\x01aU\x0BV[\x03\x91Z\xFA\x80\x15a\x07\xA3W`\0\x90\x81\x92\x82\x91a \xA3W[Pa\x06\xB1\x90`@Q\x93\x84\x93\x84`@\x91\x94\x93\x92``\x82\x01\x95\x15\x15\x82R` \x82\x01R\x01RV[\x90Pa\x06\xB1\x92Pa \xCB\x91P``=\x81\x11a \xD2W[a \xC3\x81\x83a\x1E\x83V[\x81\x01\x90aT\xE7V[\x90\x92a \x7FV[P=a \xB9V[4a\x046W`@`\x03\x196\x01\x12a\x041W`\x045a \xF6\x81a\x07\xADV[`$5\x90\x81`\x0F\x0B\x91\x82\x81\x03a\x03\xE0Wb\xFF\xFF\xFF\x90a!oa\x0B\xFAa!=`\0\x93a!8a\x11/\x88g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0R`\n` R`@`\0 \x90V[ab\x86V[\x94\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x91\x16\x95\x16\x95` \x1C\x16b\xFF\xFF\xFF\x16`\0R`\t` R`@`\0 \x90V[\x93\x12\x15a!\xCAWa!\xA2\x91a\x1A\x88a\x0C\x12``a\x0C\x1Fa!\x9Da!\x9D\x96a\x1A\x88a\x0C\x12` \x8C\x01Q`\xFF\x16\x90V[al<V[\x90[`@\x80Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x83\x16\x81R\x92\x90\x91\x16` \x83\x01R\x90\xF3[a!\xF7\x91a!\xF1a\x0C\x12``a\x0C\x1Fa!\x9Da!\x9D\x96a!\xF1a\x0C\x12` \x8C\x01Q`\xFF\x16\x90V[\x90a[>V[\x90a!\xA4V[4a\x046W` `\x03\x196\x01\x12a\x041Wg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x045a\"#\x81a\x07\xADV[\x16`\0R`\n` R`@`\0 \x80Ta\x06\xB1`\x01\x83\x01T\x92a\"d`\x03a\"U`\x02\x84\x01T`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x92\x01T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`@\x80Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x81\x16\x82R`\x80\x96\x87\x1C` \x83\x01R\x87\x16\x91\x81\x01\x91\x90\x91Rc\xFF\xFF\xFF\xFF\x86\x86\x1C\x16``\x82\x01Ra\xFF\xFF`\xA0\x87\x81\x1C\x82\x16\x96\x83\x01\x96\x90\x96R`\xB0\x96\x90\x96\x1C\x90\x95\x16\x93\x85\x01\x93\x90\x93R`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\xC0\x85\x01R\x90\x91\x16`\xE0\x83\x01R\x81\x90a\x01\0\x82\x01\x90V[4a\x046W```\x03\x196\x01\x12a\x041W`\x045a\"\xFF\x81a\x07\xADV[a#\x07a\t\x0BV[a#\x0Fa\t\x1CV[\x91a#\x18a@<V[a#6\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0R`\n` R`@`\0 \x90V[\x91a#K`\x02\x84\x01T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x033\x91\x16\x03a\x1D\x8EWa\xFF\xFF\x80\x85\x16\x94\x85a$\x80W[P\x81\x16\x92\x83a#\xABW[PPg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x80N\x0E\xF8\xEB\xD1\x19o\x98\xB3\xC6\xA20\xDE\xFF\xD8\xCF\xE0<\xB1\x92?\xE0\xE4\x02%-\x06\xD8\xD4v\xDA`\0\x80\xA4a\0\x1Ba@\x9DV[`\x01\x01a#\xE8a#\xE4a#\xCEa#\xC7\x84Ta\xFF\xFF\x90`\xA0\x1C\x16\x90V[a\xFF\xFF\x16\x90V[\x86\x90\x80\x82\x10\x90\x82\x14\x17\x90`\x01\x80\x82\x11\x91\x14\x17\x16\x90V[\x15\x90V[a$KWg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x91a$D\x91\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFw\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83T\x92`\xB0\x1B\x16\x91\x16\x17\x90UV[\x908a#rV[`@Q\x7F\xED\xDF\xE1\x19\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Ra\xFF\xFF\x83\x16`\x04\x82\x01R`$\x90\xFD[a\x03\xE8\x80\x87\x10\x90\x87\x14\x17`\x01\x87\x11`\x01\x88\x14\x17\x16\x15a$\xEEWa$\xE8\x90`\x01\x86\x01\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFu\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83T\x92`\xA0\x1B\x16\x91\x16\x17\x90UV[8a#hV[`@Q\x7F\x97\x1B1\t\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Ra\xFF\xFF\x91\x90\x91\x16`\x04\x82\x01R`$\x90\xFD[4a\x046W`@`\x03\x196\x01\x12a\x041W`\x045a%B\x81a\x03\xCFV[`\x01`\x01`\xA0\x1B\x03`$5\x91a%W\x83a\x07\xBFV[3`\0R`\x01` Ra%\x81\x81`@`\0 \x90`\x01`\x01`\xA0\x1B\x03\x16`\0R` R`@`\0 \x90V[\x92\x15\x15\x92\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x81T\x16`\xFF\x85\x16\x17\x90U`@Q\x92\x83R\x16\x90\x7F\x170~\xAB9\xABa\x07\xE8\x89\x98E\xAD=Y\xBD\x96S\xF2\0\xF2 \x92\x04\x89\xCA+Y7il1` 3\x92\xA3\0[4a\x046W` `\x03\x196\x01\x12a\x041Wb\xFF\xFF\xFFa%\xFDa\x08\xF9V[\x16`\0R`\x08` R` c\xFF\xFF\xFF\xFF`@`\0 T\x16`@Q\x90\x81R\xF3[` \x80\x82\x01\x90\x80\x83R\x83Q\x80\x92R`@\x83\x01\x92\x81`@\x84`\x05\x1B\x83\x01\x01\x95\x01\x93`\0\x91[\x84\x83\x10a&PWPPPPPP\x90V[\x90\x91\x92\x93\x94\x95\x84\x80a&\x8C\x83\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x86`\x01\x96\x03\x01\x87R\x8AQa\x05\xC1V[\x98\x01\x93\x01\x93\x01\x91\x94\x93\x92\x90a&@V[` \x80`\x03\x196\x01\x12a\x041W`\x045g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x07\x9EWa&\xC9\x906\x90`\x04\x01a\x0CdV[\x91a&\xD6`\x0FT`\xFF\x16\x90V[a'\xD9Wa&\xE2a@<V[a'\x12`\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0`\x0FT\x16\x17`\x0FUV[a'\x1AaK\xA8V[a'#\x83a=\xB7V[\x92`\0\x90\x81[\x81\x81\x10a'}Wa\x06\xB1\x86a'a\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0`\x0FT\x16`\x0FUV[a'iaQ\xD9V[a'qa@\x9DV[`@Q\x91\x82\x91\x82a&\x1CV[\x82\x80a'\x8A\x83\x85\x89a>\xD4V[\x90a'\x9A`@Q\x80\x93\x81\x93a?\xC6V[\x03\x900Z\xF4a'\xA7a?\xF8V[\x90\x15a'\xD2W\x90a'\xCD\x91a'\xBC\x82\x89a@(V[Ra'\xC7\x81\x88a@(V[Pa>MV[a')V[\x80Q\x90\x85\x01\xFD[`\x04`@Q\x7FU\xE1\xF7\xC5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xFD[4a\x046W`\0`\x03\x196\x01\x12a\x041W` `@Q`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[4a\x046W`\0`\x03\x196\x01\x12a\x041W` `@Q`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[4a\x046W`\0`\x03\x196\x01\x12a\x041W` `\rT`@Q\x90\x81R\xF3[4a\x046W` `\x03\x196\x01\x12a\x041W`\x01`\x01`\xA0\x1B\x03`\x045a(\xCE\x81a\x03\xCFV[\x16`\0R`\x02` R` `@`\0 T`@Q\x90\x81R\xF3[a(\xF06a\x15\xFAV[\x90a(\xF9a@<V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x83\x16\x91\x81\x16\x90\x82\x82\x14a,MWa)=a)4\x85a\x0F\x1E\x84`\x01`\x01`\xA0\x1B\x03\x16`\0R`\x0B` R`@`\0 \x90V[Tb\xFF\xFF\xFF\x16\x90V[\x93b\xFF\xFF\xFF\x94\x85\x81\x16a,\x15WP\x82;\x15a\x07\xA8W`@\x94\x85Q\x92\x7F1<\xE5g\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x92\x83\x85R` \x80\x86`\x04\x81\x8AZ\xFA\x95\x86\x15a\x07\xA3W`\0\x96a+\xF6W[P\x87;\x15a\x07\xA8W\x88Q\x94\x85R\x80\x85`\x04\x81\x8BZ\xFA\x94\x85\x15a\x07\xA3W`\0\x95a+\xC7W[P`\xFF\x86\x16`\x12\x81\x10`\x12\x82\x14\x17\x90`\x06\x80\x82\x11\x91\x14\x17\x16\x15a+\x94Wa)\xF7a#\xE4`\xFF\x87\x16`\x12\x81\x10`\x12\x82\x14\x17\x90`\x06\x80\x82\x11\x91\x14\x17\x16\x90V[a+aW\x92a++\x7F\xC0\xC5\xDF\x98\xA4\xCA\x87\xA3!\xA3;\xF1'|\xF3-1\xA9{l\xE1K\x97G8!I\xB9\xE2c\x1E\xA3\x93a+\x04a\x06\xB1\x9B\x99\x97\x94a*\xF5\x8Aa*\xD1\x9D\x9B\x99a*Ka*F`\x06Tb\xFF\xFF\xFF\x16\x90V[aG\x14V[\x9E\x8Fa*\x81\x81b\xFF\xFF\xFF\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0`\x06T\x16\x17`\x06UV[a*\xA2\x86a\x0F\x1E\x8A`\x01`\x01`\xA0\x1B\x03\x16`\0R`\x0B` R`@`\0 \x90V[\x90b\xFF\xFF\xFF\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\x82T\x16\x17\x90UV[a*\xEBa*\xDCa\x1E\xD3V[`\x01`\x01`\xA0\x1B\x03\x90\x97\x16\x87RV[\x85\x01\x90`\xFF\x16\x90RV[`\x01`\x01`\xA0\x1B\x03\x16\x82\x8B\x01RV[`\xFF\x84\x16``\x82\x01Ra+&\x8Ab\xFF\xFF\xFF\x16`\0R`\t` R`@`\0 \x90V[aG(V[\x86Q`\xFF\x94\x85\x16\x81R\x91\x90\x93\x16` \x82\x01R\x91\x86\x16\x91`@\x90\xA4a+Ma@\x9DV[Qb\xFF\xFF\xFF\x90\x91\x16\x81R\x90\x81\x90` \x82\x01\x90V[\x88Q\x7F\xC3\xDAwG\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\xFF\x86\x16`\x04\x82\x01R`$\x90\xFD[\x88Q\x7F\xC3\xDAwG\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\xFF\x87\x16`\x04\x82\x01R`$\x90\xFD[\x81a+\xE8\x92\x96P=\x87\x11a+\xEFW[a+\xE0\x81\x83a\x1E\x83V[\x81\x01\x90aF\xFBV[\x938a)\xBAV[P=a+\xD6V[\x81a,\x0E\x92\x97P=\x88\x11a+\xEFWa+\xE0\x81\x83a\x1E\x83V[\x948a)\x96V[`@Q\x7F\xB0\x98\x8CC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rb\xFF\xFF\xFF\x91\x90\x91\x16`\x04\x82\x01R`$\x90\xFD[`\x04`@Q\x7F\x05\x13L\xCC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xFD[4a\x046W```\x03\x196\x01\x12a\x041Wa\x06\xB1a-<`\x045a,\x9A\x81a\x07\xADV[b\xFF\xFF\xFF\x81` \x1C\x16`\0R`\t` R`@`\0 a-7a\x11/a-\x1Ca-\x14a\x0C\x12a-\x0C`@Q\x96a,\xCF\x88a\x1E6V[`\xFF``\x82T\x99`\x01\x83`\x01`\x01`\xA0\x1B\x03\x9C\x8D\x81\x16\x84R`\xA0\x1C\x16\x94\x85` \x84\x01R\x01T\x9A\x8B\x16`@\x82\x01R\x01\x98`\xA0\x1C\x16\x88R`$5aZ\xB0V[\x95Q`\xFF\x16\x90V[`D5aZ\xB0V[\x93g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0R`\n` R`@`\0 \x90V[aa\x17V[`@Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R\x90\x81\x90` \x82\x01\x90V[4a\x046W` `\x03\x196\x01\x12a\x041W`\x01`\x01`\xA0\x1B\x03`\x045a-\x84\x81a\x03\xCFV[\x16`\0R`\x07` R` `@`\0 T`@Q\x90\x81R\xF3[4a\x046W`@`\x03\x196\x01\x12a\x041W`\x045a-\xBA\x81a\x03\xCFV[`$5\x90a-\xC6a@<V[`\x01`\x01`\xA0\x1B\x03\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x91\x82;\x15a\x07\xA8W`@Q\x80\x93\x7F\xF7|G\x91\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R\x81`\x04` \x96\x87\x93Z\xFA\x80\x15a\x07\xA3W\x83\x91`\0\x91a/\x8AW[P\x163\x03a\x1D\x8EW`\0\x91\x81\x16\x93\x84;\x15a\x07\xA8W`@Q\x90\x7F1<\xE5g\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R\x84\x82`\x04\x81\x89Z\xFA\x91\x82\x15a\x07\xA3W\x7F\x1F\xDD\0 5\x88\x93U\x97\x13\xDE\xF8\xB4,\xADf\x1F\xFB\xC7U\xD1\xA2dY@'\x92\x14B\xBBV\xA0\x95a//\x95\x93a/kW[PP`\0\x19\x81\x03a/SWPa/\x1F\x90a.\xEC`\xFFa.\xE4\x85`\x01`\x01`\xA0\x1B\x03\x16`\0R`\x07` R`@`\0 \x90V[T\x92\x16aZ\xD7V[\x81\x04\x92[a/\r\x81`\x01`\x01`\xA0\x1B\x03\x16`\0R`\x07` R`@`\0 \x90V[a/\x18\x83\x82TaF\xE1V[\x90UaL\xD1V[`@Q\x90\x81R\x90\x81\x90` \x82\x01\x90V[\x03\x90\xA2`\x0FT`\xFF\x16\x15a/FW[a\0\x1Ba@\x9DV[a/NaQ\xD9V[a/>V[\x91a/c`\xFFa/\x1F\x93\x16aZ\xD7V[\x83\x02\x90a.\xF0V[a/\x82\x92\x93P\x80=\x10a+\xEFWa+\xE0\x81\x83a\x1E\x83V[\x908\x80a.\xB2V[a/\xA1\x91P\x85=\x87\x11a\x1D\xDFWa\x1D\xD1\x81\x83a\x1E\x83V[8a.;V[4a\x046W` `\x03\x196\x01\x12a\x041W`\x045a/\xC4\x81a\x07\xADV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16`\0R`\n` Ra/\xF6a\x080a\x080`\x03`@`\0 \x01`\x01`\x01`\xA0\x1B\x03\x90T\x16\x90V[\x80;\x15a\x07\xA8W`@Q\x7F\xE31\xBA4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16`\x04\x83\x01R` \x90\x82\x90\x81\x80`$\x81\x01a\x08\xA4V[4a\x046W` `\xFFa0\x87`\x01`\x01`\xA0\x1B\x03a0a6a\x15\xFAV[\x91\x16`\0R`\x01\x84R`@`\0 \x90`\x01`\x01`\xA0\x1B\x03\x16`\0R` R`@`\0 \x90V[T\x16`@Q\x90\x15\x15\x81R\xF3[a\x1E\xD1\x90\x92\x91\x92`\xA0\x81\x01\x93`\x80\x80\x91o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82Q\x16\x85R` \x82\x01Q\x16` \x85\x01R`@\x81\x01Q\x15\x15`@\x85\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF``\x82\x01Q\x16``\x85\x01R\x01Q\x15\x15\x91\x01RV[4a\x046W```\x03\x196\x01\x12a\x041W`\x80`\x045a1\r\x81a\x07\xADV[`$5\x90a1\x1A\x82a\x07\xBFV[`D5a1&\x81a\x03\xCFV[`@Qa12\x81a\x1E\x15V[`\0\x94\x81\x86\x80\x93R\x82` \x82\x01R\x82`@\x82\x01R\x82``\x82\x01R\x01Ra1wa\x080a\x080`\x03a\x10\r\x86g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0R`\n` R`@`\0 \x90V[\x80;\x15a\x07\xA8W`@Q\x7F\xF0{\x87\x9E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93\x90\x93\x16`\x04\x84\x01R\x92\x15\x15`$\x83\x01R`\x01`\x01`\xA0\x1B\x03\x16`D\x82\x01R\x90`\xA0\x90\x82\x90`d\x90\x82\x90Z\xFA\x90\x81\x15a\x07\xA3Wa\x06\xB1\x92\x91a1\xF7W[P`@Q\x91\x82\x91\x82a0\x93V[a2\x18\x91P`\xA0=\x81\x11a2\x1EW[a2\x10\x81\x83a\x1E\x83V[\x81\x01\x90aTtV[8a1\xEAV[P=a2\x06V[4a\x046W`\xA0`\x03\x196\x01\x12a\x041W`\x045a2B\x81a\x03\xCFV[`$5\x90a2O\x82a\x03\xCFV[`D5`d5`\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x07\x9EWa2u\x906\x90`\x04\x01a\nkV[\x94\x90\x91`\x01`\x01`\xA0\x1B\x03\x91\x82\x86\x16\x92\x833\x14\x80\x15a3\x99W[a2\x98\x90agwV[a2\xB9\x86a\x0E\xA7\x89`\x01`\x01`\xA0\x1B\x03\x16`\0R`\0` R`@`\0 \x90V[a2\xC4\x84\x82TaF\xE1V[\x90Ua2\xE7\x86a\x0E\xA7\x84`\x01`\x01`\xA0\x1B\x03\x16`\0R`\0` R`@`\0 \x90V[a2\xF2\x84\x82TaF\xEEV[\x90U\x81\x16\x80\x93`@Q\x7F\xC3\xD5\x81h\xC5\xAEs\x97s\x1D\x06=[\xBF=exTBsC\xF4\xC0\x83$\x0Fz\xAC\xAA-\x0Fb3\x91\x80a37\x88\x8C\x83` \x90\x93\x92\x91\x93`@\x81\x01\x94\x81R\x01RV[\x03\x90\xA4;\x15a3\x88W\x81;\x15a\x07\xA8W`\0` \x94a\r\xE3`@Q\x98\x89\x96\x87\x95\x86\x94\x7F\xF2:na\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x9B\x8C\x87R3`\x04\x88\x01ah\"V[P\x92PPPa\0\x1B\x91P\x15\x15ag\xC2V[Pa2\x98a3\xC1a\x0F63a\x0F\x1E\x8B`\x01`\x01`\xA0\x1B\x03\x16`\0R`\x01` R`@`\0 \x90V[\x90Pa2\x8FV[`\xA0`\x03\x196\x01\x12a\x041Wa3\xDCa@<V[`\x0FT`\xFF\x16\x15a<\x0FW[a3\xF0aF\x19V[a3\xF8aF%V[\x90a4\x19a4\x04aF1V[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x93\x16\x91a4Ta4:aF\x19V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0R`\n` R`@`\0 \x90V[a4ra#\xE4a4c\x83aAOV[``\x01Qc\xFF\xFF\xFF\xFF\x16\x15\x15\x90V[a;\xD4Wa4\x87a4\x82\x82aAOV[a`\xE4V[a;\xAAWa4\xB4a\x080a\x080`\x03a\x10\r\x87g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0R`\n` R`@`\0 \x90V[\x94\x85;\x15a\x07\xA8W`@\x91\x82Q\x7F\xE6\x04{\x19\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x97`\x04\x91\x89\x81\x80a5\n\x8B\x87\x83\x01\x91\x90\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x82\x01\x93\x16\x90RV[\x03\x81\x85Z\xFA\x90\x81\x15a\x07\xA3W`\0\x91a;\x8DW[P\x15a;NWa5.B\x84a^\xE3V[a56aF=V[\x81;\x15a\x07\xA8W\x85Q\x7F\xECshT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x89\x16\x84\x82\x01\x90\x81R\x91\x15\x15` \x83\x01R3`@\x83\x01R\x90\x86\x90\x82\x90\x81\x90``\x01\x03\x81`\0\x86Z\xF1\x90\x81\x15a\x07\xA3W`\0\x90\x81\x92a;.W[P\x15a;\x06Wa5\xCCa\x0B\xFAb\xFF\xFF\xFF\x8A\x8D\x1C\x16b\xFF\xFF\xFF\x16`\0R`\t` R`@`\0 \x90V[\x90\x8A\x87a5\xD7aFtV[\x93a5\xE0aF=V[\x15a:\x8FW\x90\x81a6\x08a5\xFDa6f\x95a6V\x95\x01Q`\xFF\x16\x90V[`\xFF\x16a\x01\0\x88\x01RV[a6$a6\x19``\x83\x01Q`\xFF\x16\x90V[`\xFF\x16a\x01 \x88\x01RV[a6Ha68\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16`\xC0\x88\x01RV[\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16`\xE0\x84\x01RV[a6naFIV[a:NW[\x90a6\x8B\x91\x81R\x89`\x80\x82\x01R\x87`\xA0\x82\x01Raf\x9CV[\x92`\xA0\x84\x01\x90\x81Q\x15a:&W`\x80\x85\x01\x93\x84Q\x15a9\xFFWa7F\x8Ca6\xFBa6\xB46a\x1E\xE0V[\x91a6\xE5a6\xC2\x8AQal<V[\x91a6\xCD\x89Qal<V[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x85\x01RV[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82RV[`\x01`\x01`\xA0\x1B\x03\x983\x8Aa7\x1A`\x02\x88\x01T`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x16\x03a9\xEDW`\x01\x85\x01T`\xB0\x1Ca\xFF\xFF\x16\x91[\x85T\x90a\xFF\xFF`\rT\x94\x16\x92\x82`\x80\x1C\x92\x16\x90ad\xABV[\x92``\x8A\x98\x93\x98\x01\x97\x88R\x8B\x8A\x01R\x88Q\x91\x80;\x15a\x07\xA8W\x8B\x92\x8Ea7\xBA\x85Q\x96\x87\x95\x86\x94\x85\x94\x7F\xA4G\x89\x19\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86R\x8B\x86\x01\x90\x94\x93\x92``\x92g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x84\x01\x97\x16\x83R` \x83\x01R`@\x82\x01R\x01RV[\x03\x91Z\xFA\x90\x81\x15a\x07\xA3W\x8D`\0\x91\x82\x93a9\xBCW[P\x82\x90\x89\x01R\x15a9~WPP\x93\x88\x99\x9A\x93\x7F\xCD\x80T&o\xAE;\xBD#\0\xC8\x9A@t\xC4\x11=\x12\xB9\xA5\x83\xF7\xCD#\x90\xD40\xAFp\xB1*X\x93a8ha8w\x94a81a\x06\xB1\x9D\x99a8\x1BaF=V[a8(\x87Q\x87Q\x90aF\xE1V[\x90\x84Q\x92a_\xBEV[a8R`\xC0\x86\x01\x94a8J\x86Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x90Q\x90aLeV[`\xE0\x85\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x90Q\x90aL\xD1V[Q\x90\x81a9EW[PPag\x0CV[\x93a8\x80aF=V[\x92a8\xF3\x8Ba8\x99`\xC0\x89\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x97\x8Aa8\xAF`\xE0\x83\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x82\x8C\x01Q\x92\x90\x95\x01Q\x8BQ\x98\x15\x15\x89R` \x89\x01\x93\x90\x93R`@\x88\x01R``\x87\x01R`\x80\x86\x01R\x90\x82\x16\x95\x90\x91\x16\x93g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x92\x90\x81\x90`\xA0\x82\x01\x90V[\x03\x90\xA4a9\x05a\x12\xBC`\x0FT`\xFF\x16\x90V[\x15a98W[a9\x13a@\x9DV[Q\x93\x84\x93\x84`@\x91\x94\x93\x92g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF``\x83\x01\x96\x16\x82R` \x82\x01R\x01RV[a9@aQ\xD9V[a9\x0BV[a\x0E\xE0a9\\a9u\x92Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16`\0R`\x07` R`@`\0 \x90V[\x90U8\x80a8pV[\x86Q\x89Q\x7F\xB9=\xEE\n\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x92\x83\x01\x90\x81R` \x81\x01\x91\x90\x91R\x81\x90`@\x01\x03\x90\xFD[\x90\x92Pa9\xDF\x91P\x8A=\x8C\x11a9\xE6W[a9\xD7\x81\x83a\x1E\x83V[\x81\x01\x90aFUV[\x918a7\xD0V[P=a9\xCDV[`\x01\x85\x01T`\xA0\x1Ca\xFF\xFF\x16\x91a7.V[\x87Q\x7F\x13\xFD\x9Bm\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xFD[\x83\x87Q\x7Fo\x85\xB3N\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xFD[a:ea\x13\xB5`\xC0\x84\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\0\x81\x13a:tW[Pa6sV[a6\x8B\x92\x91\x9APa4\x04a:\x87\x91al<V[\x99\x90\x91a:nV[a6V\x91Pa:\xCEa:\xC3a;\x01\x94a:\xBAa:\xAF``\x86\x01Q`\xFF\x16\x90V[`\xFF\x16a\x01\0\x8A\x01RV[\x83\x01Q`\xFF\x16\x90V[`\xFF\x16a\x01 \x87\x01RV[a:\xF4a:\xE4\x8B\x83\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16`\xC0\x87\x01RV[Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[a6fV[\x82\x86Q\x7F.\xD0\xEA\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xFD[\x90Pa;G\x91P\x86=\x88\x11a9\xE6Wa9\xD7\x81\x83a\x1E\x83V[\x908a5\xA3V[\x84Q\x7F\xBC'\xA5\x17\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x88\x16\x81\x84\x01\x90\x81R\x81\x90` \x01\x03\x90\xFD[a;\xA4\x91P\x8A=\x8C\x11a\x14\x96Wa\x14\x88\x81\x83a\x1E\x83V[8a5\x1EV[`\x04`@Q\x7Fz\x95\xCB!\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xFD[`@Q\x7F\x12\x16\xE0\xFA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R`$\x90\xFD[a<\x17aK\xA8V[a3\xE8V[4a\x046W`\0`\x03\x196\x01\x12a\x041W` `\0Rk\x0Bv1.5.0-beta`+R```\0\xF3[`\x84`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`)`$\x82\x01R\x7FUnknown signature and no fallbac`D\x82\x01R\x7Fk defined\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R\xFD[`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x163\x03a\x03\xE0WV[`\x84`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x1E1W`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16` \x01\x90V[`@Q=`\0\x82>=\x90\xFD[a\x06\x15\x900\x90aY\xD5V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x1E1W`\x05\x1B` \x01\x90V[\x90a=\xC1\x82a=\x9FV[a=\xCE`@Q\x91\x82a\x1E\x83V[\x82\x81R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0a=\xFC\x82\x94a=\x9FV[\x01\x90`\0[\x82\x81\x10a>\rWPPPV[\x80``` \x80\x93\x85\x01\x01R\x01a>\x01V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0\x19\x81\x14a>\\W`\x01\x01\x90V[a>\x1EV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[`d`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FCalldata tail too short\0\0\0\0\0\0\0\0\0`D\x82\x01R\xFD[\x91\x90\x81\x10\x15a?\xC1W`\x05\x1B\x81\x015\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x816\x03\x01\x82\x12\x15a?}W\x01\x90\x815\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a?9W` \x01\x826\x03\x81\x13a?4W\x91\x90V[a>\x90V[`d`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FInvalid calldata tail length\0\0\0\0`D\x82\x01R\xFD[`d`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FInvalid calldata tail offset\0\0\0\0`D\x82\x01R\xFD[a>aV[\x90\x80\x92\x91\x827\x01`\0\x81R\x90V[`@Q\x90` \x82\x01\x82\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x1E1W`@R`\0\x82RV[=\x15a@#W=\x90a@\t\x82a=NV[\x91a@\x17`@Q\x93\x84a\x1E\x83V[\x82R=`\0` \x84\x01>V[``\x90V[\x80Q\x82\x10\x15a?\xC1W` \x91`\x05\x1B\x01\x01\x90V[`\x0CT`\x01\x81\x14\x15\x80a@\x86W[a@\\Wa@W\x90a>MV[`\x0CUV[`\x04`@Q\x7F\x02\xB8\0-\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xFD[P`\xFF`\x0FT\x16\x15\x80a@JWP`\x02\x81\x11a@JV[`\x0CT\x80\x15a>\\W`\0\x19\x01`\x0CU`\xFF`\x05T\x16\x15\x80a@\xEAW[a@\xC0WV[`\x04`@Q\x7F2n\xFAC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xFD[P`\xFF`\x0FT\x16\x15a@\xBAV[\x90\x81` \x91\x03\x12a\x041WQa\x06\x15\x81a\x07\xBFV[\x90`@QaA\x19\x81a\x1E6V[```\xFF\x82\x94`\x01\x81T\x91\x83`\x01`\x01`\xA0\x1B\x03\x93\x84\x81\x16\x88R`\xA0\x1C\x16` \x87\x01R\x01T\x90\x81\x16`@\x85\x01R`\xA0\x1C\x16\x91\x01RV[\x90a\x1E\xD1`@QaA_\x81a\x1ERV[`\xE0aB\x18`\x03\x83\x96aB\x01\x81TaA\xAAo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91aA\xA0\x83\x82\x16\x8A\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90RV[`\x80\x1C` \x89\x01RV[aA\xD0`\x01\x84\x01T\x91\x82\x16`@\x89\x01\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90RV[`\x80\x81\x90\x1Cc\xFF\xFF\xFF\xFF\x16``\x88\x01Ra\xFF\xFF`\xA0\x82\x90\x1C\x81\x16`\x80\x89\x01R\x90`\xB0\x1C\x16`\xA0\x87\x01\x90a\xFF\xFF\x16\x90RV[a\x10\ra:\xE4`\x02\x83\x01T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x91\x01RV[`\x0F\x0B\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x14a>\\W`\0\x03\x90V[`\x0F\x0B\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC4e6\0\x01\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x12o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x13\x17a>\\WV[\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x14a>\\W`\0\x03\x90V[\x91\x90\x91o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x80\x94\x16\x91\x16\x01\x91\x82\x11a>\\WV[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x82\x16\x90\x82\x16\x03\x91\x90\x82\x11a>\\WV[`\x80\x81\x01\x90aCFa4:\x83Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x90aCT` \x82\x01Qal<V[\x90aCb`@\x82\x01Qal<V[\x93``\x82\x01\x91aCs\x83Q`\x0F\x0B\x90V[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x83aC\xA3`\x01\x89\x01To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x16\x15aE\xBFW[a4:aD\x14\x91aD\x1F\x93`\x0F\x0B`\0\x81\x13`\0\x14aEvWaD\x06\x90aC\xDB`\xA0\x88\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFaC\xF7\x85Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x16aD\0a?\xD4V[\x92ajOV[Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x84Q`\x0F\x0B\x90a_8V[`\0aD]aDWaDO`\xE0aD@`\xC0\x87\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x95\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x95Q`\x0F\x0B\x90V[`\x0F\x0B\x90V[\x12\x15aE+W\x83aD\x8Ba\x1E\xD1\x97\x94\x84aD\x83aD\xA4\x95aD\xE1\x97aD\xEB\x9A\x16\x90aL\xD1V[\x86\x16\x90aL\xD1V[\x85To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aC\nV[\x84\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82T\x16\x17\x90UV[\x82T`\x80\x1CaC\nV[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83T\x92`\x80\x1B\x16\x91\x16\x17\x90UV[\x83aESa\x1E\xD1\x97\x94\x84aEKaD\xA4\x95aEl\x97aD\xEB\x9A\x16\x90aLeV[\x86\x16\x90aLeV[\x85To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aB\xE6V[\x82T`\x80\x1CaB\xE6V[aE\xBA\x90aE\x8E`\xA0\x88\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFaE\xB2aE\xAC\x86Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x93aB\xB9V[\x92\x16\x90akmV[aD\x06V[`\0\x87Uc;\x9A\xCA\0`\x0F\x83\x90\x0B\x12aE\xEFWa4:aD\x14\x91aE\xE5aD\x1F\x94aBVV[\x93P\x91PPaC\xAAV[`\x04`@Q\x7F\xCBm\xABu\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xFD[`d5a\x06\x15\x81a\x07\xADV[`\x045a\x06\x15\x81a\x0FDV[`$5a\x06\x15\x81a\x0FDV[`\x845a\x06\x15\x81a\x07\xBFV[`D5a\x06\x15\x81a\x07\xBFV[\x91\x90\x82`@\x91\x03\x12a\x041W` \x82QaFn\x81a\x07\xBFV[\x92\x01Q\x90V[`@Q\x90a\x01@\x82\x01\x82\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x1E1W`@R\x81a\x01 `\0\x91\x82\x81R\x82` \x82\x01R\x82`@\x82\x01R\x82``\x82\x01R\x82`\x80\x82\x01R\x82`\xA0\x82\x01R\x82`\xC0\x82\x01R\x82`\xE0\x82\x01R\x82a\x01\0\x82\x01R\x01RV[\x90`\0\x19\x82\x01\x91\x82\x11a>\\WV[\x91\x90\x82\x03\x91\x82\x11a>\\WV[\x91\x90\x82\x01\x80\x92\x11a>\\WV[\x90\x81` \x91\x03\x12a\x041WQ`\xFF\x81\x16\x81\x03a\x03\xE0W\x90V[b\xFF\xFF\xFF\x80\x91\x16\x90\x81\x14a>\\W`\x01\x01\x90V[`\xFF``a\x1E\xD1\x93aG\xFE`\x01`\x01`\x01`\xA0\x1B\x03\x95aGy\x87\x85Q\x16\x82\x90`\x01`\x01`\xA0\x1B\x03\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82T\x16\x17\x90UV[` \x84\x01Q\x81T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x87\x16`\xA0\x1Bt\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x17\x81U\x01\x94`@\x83\x01Q\x16\x85\x90`\x01`\x01`\xA0\x1B\x03\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82T\x16\x17\x90UV[\x01Q\x82T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91\x16`\xA0\x1Bt\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x17\x90UV[c\xFF\xFF\xFF\xFF\x80\x91\x16\x90\x81\x14a>\\W`\x01\x01\x90V[`\x1F\x82` \x94\x93\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x93\x81\x86R\x86\x86\x017`\0\x85\x82\x86\x01\x01R\x01\x16\x01\x01\x90V[`@\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x06\x15\x95\x93\x16\x81R\x81` \x82\x01R\x01\x91aHZV[\x96\x98\x97\x92\x90\x94\x95aH\xC9a@<V[b\xFF\xFF\xFF\x97\x80\x89\x16aK\xA2WP`\x06Tb\xFF\xFF\xFF\x16\x97[\x88\x16\x15aKxWaI\0\x88b\xFF\xFF\xFF\x16`\0R`\x08` R`@`\0 \x90V[\x93\x84TaI\x10\x90c\xFF\xFF\xFF\xFF\x16\x90V[aI\x19\x90aHEV[\x85T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\x16c\xFF\xFF\xFF\xFF\x82\x16\x17\x90\x95U`\x01`\x01`\xA0\x1B\x03\x94aI`\x90\x8A\x8A\x88\x16\x15\x15a[\x83V[\x9AaI\x7F\x8Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0R`\n` R`@`\0 \x90V[\x83\x8Aa\xFF\xFF\x8B\x89\x82\x8D\x16\x92\x8A\x16\x91aI\x96\x96a[\xB6V[aI\xD5\x8C\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\xFFh\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0`\x0FT\x92`\x08\x1B\x16\x91\x16\x17`\x0FUV[aI\xF3\x8Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0R`\n` R`@`\0 \x90V[`\x03\x01T`\x01`\x01`\xA0\x1B\x03\x16\x91\x82;\x15a\x07\xA8WaJH\x92` \x92`\0\x8F`@Q\x96\x87\x95\x86\x94\x85\x93\x7F\xE0hx\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85R`\x04\x85\x01aH\x99V[\x03\x92Z\xF1\x90\x81\x15a\x07\xA3W`\0\x91aKZW[P\x15aK0W\x89\x96\x7Fa<|I?\x9A\xA3\x97M\xCD\xBB\x87\xE6\xFE\xA5L\xCD\xB9\x10>\xA7\x83.m\xF0\x8D\xB8\x1Co\x90\xD2\x9D\x95aK%\x93aJ\xD0`\x01a\x10\raJ\xBAaJ\xAD\x8Fb\xFF\xFF\xFF\x16`\0R`\t` R`@`\0 \x90V[T`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x9Db\xFF\xFF\xFF\x16`\0R`\t` R`@`\0 \x90V[\x95g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x88`@Q\x99\x8A\x99\x16\x9D\x16\x9B\x16\x99\x87\x94\x92\x90\x96\x95\x91\x93`\xA0\x94`\xC0\x87\x01\x98\x87R` \x87\x01Ra\xFF\xFF\x80\x92\x16`@\x87\x01R\x16``\x85\x01R`\x01`\x01`\xA0\x1B\x03\x80\x92\x16`\x80\x85\x01R\x16\x91\x01RV[\x03\x90\xA4a\x1E\xD1a@\x9DV[`\x04`@Q\x7F\xDF\x15\x8AX\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xFD[aKr\x91P` =\x81\x11a\x14\x96Wa\x14\x88\x81\x83a\x1E\x83V[8aJ[V[`\x04`@Q\x7F\xCCzs\x9B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xFD[\x97aH\xE0V[4aK\xAFWV[`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0aK\xE1\x81aXZV[\x16\x80;\x15a\x07\xA8W`\0`\x04\x91`@Q\x92\x83\x80\x92\x7F\xD0\xE3\r\xB0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R4\x90Z\xF1\x80\x15a\x07\xA3WaLVW[P`@Q4\x81R\x7F\xE1\xFF\xFC\xC4\x92=\x04\xB5Y\xF4\xD2\x9A\x8B\xFCl\xDA\x04\xEB[\r<F\x07Q\xC2@,\\\\\xC9\x10\x9C` 3\x92\xA2V[aL_\x90a\x1EoV[8aL'V[aLn\x81aXZV[aL\x8B\x81`\x01`\x01`\xA0\x1B\x03\x16`\0R`\x02` R`@`\0 \x90V[\x80T\x90\x83\x82\x01\x80\x92\x11a>\\WU`@Q\x91\x82R`\x01`\x01`\xA0\x1B\x03\x16\x90\x7F\x80\xB2\x17H\xC7\x87\xC5.\x87\xA6\xB2\"\x01\x1E\n\x0E\xD0\xF9\xCC \x15\xF0\xCE\xD4gHd-\xC6.\xE9\xF8\x90` \x90\xA2V[aL\xEE\x81`\x01`\x01`\xA0\x1B\x03\x16`\0R`\x02` R`@`\0 \x90V[T\x80\x83\x11aMcWPaM\0\x81aXZV[aM\x1D\x81`\x01`\x01`\xA0\x1B\x03\x16`\0R`\x02` R`@`\0 \x90V[\x80T\x90\x83\x82\x03\x91\x82\x11a>\\WU`@Q\x91\x82R`\x01`\x01`\xA0\x1B\x03\x16\x90\x7F\x1Cq\x1E\xCA\x8D\x0BiK\xBC\xB0\xA1Db\xA7\0b\"\xE7!\x95K,_\xF7\x98\xF6\x06\x81~\xB1\x102\x90` \x90\xA2V[`@Q\x7F1Rv\xC9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x91\x90\x91R`$\x81\x01\x92\x90\x92RP`D\x90\xFD[b\xFF\xFF\xFF\x90\x93\x91\x92\x93` \x1C\x16`\0R`\t` R`@`\0 \x92`@QaM\xC6\x81a\x1E6V[`\xFF``\x86T\x92`\x01\x83`\x01`\x01`\xA0\x1B\x03\x95\x86\x81\x16\x84R`\xA0\x1C\x16\x98\x89` \x84\x01R\x01T\x93\x84\x16`@\x82\x01R\x01\x91`\xA0\x1C\x16\x81RaN\x04\x84al<V[\x94o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x94\x85\x81\x03aNSW[PPaN*\x82al<V[\x93\x82\x03aN5WPPV[a\x06\x15\x92\x93P\x90aNMa\x0C\x12a!\x9D\x93Q`\xFF\x16\x90V[\x90aZ\xB0V[aNc\x92\x96P\x90a!\x9D\x91aZ\xB0V[\x938\x80aN\x1FV[`@Q\x90`\x04T\x80\x83R\x82` \x91\x82\x82\x01\x90`\x04`\0R\x7F\x8A5\xAC\xFB\xC1_\xF8\x1A9\xAE}4O\xD7\t\xF2\x8E\x86\0\xB4\xAA\x8Ce\xC6\xB6K\xFE\x7F\xE3k\xD1\x9B\x93`\0\x90[\x82\x82\x10aN\xBEWPPPa\x1E\xD1\x92P\x03\x83a\x1E\x83V[\x85T`\x01`\x01`\xA0\x1B\x03\x16\x84R`\x01\x95\x86\x01\x95\x88\x95P\x93\x81\x01\x93\x90\x91\x01\x90aN\xA8V[`\x0ET\x90h\x01\0\0\0\0\0\0\0\0\x82\x10\x15a\x1E1W`\x01\x82\x01\x80`\x0EU\x82\x10\x15a?\xC1W\x7F\xBB{JEM\xC3I9#H/\x07\x82#)\xED\x19\xE8$N\xFFX,\xC2\x04\xF8UL6 \xC4\0`\x01`\x01`\xA0\x1B\x03``a\x1E\xD1\x94`\x0E`\0R`\x02\x1B\x93\x80Q\x85\x7F\xBB{JEM\xC3I9#H/\x07\x82#)\xED\x19\xE8$N\xFFX,\xC2\x04\xF8UL6 \xC3\xFD\x01U` \x81\x01Q\x7F\xBB{JEM\xC3I9#H/\x07\x82#)\xED\x19\xE8$N\xFFX,\xC2\x04\xF8UL6 \xC3\xFE\x86\x01U`@\x81\x01Q\x7F\xBB{JEM\xC3I9#H/\x07\x82#)\xED\x19\xE8$N\xFFX,\xC2\x04\xF8UL6 \xC3\xFF\x86\x01U\x01Q\x16\x91\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82T\x16\x17\x90UV[`\x04T\x80\x15aPWW`\0\x19\x81\x01\x90\x80\x82\x10\x15a?\xC1W\x7F\x8A5\xAC\xFB\xC1_\xF8\x1A9\xAE}4O\xD7\t\xF2\x8E\x86\0\xB4\xAA\x8Ce\xC6\xB6K\xFE\x7F\xE3k\xD1\x9A\x90`\x04`\0R\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81T\x16\x90U`\x04UV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`1`\x04R`$`\0\xFD[`\x0ET\x90aP\x93\x82a=\x9FV[\x91`@aP\xA2\x81Q\x94\x85a\x1E\x83V[\x81\x84R\x83` \x80\x91\x01\x91`\x0E`\0R\x7F\xBB{JEM\xC3I9#H/\x07\x82#)\xED\x19\xE8$N\xFFX,\xC2\x04\xF8UL6 \xC3\xFD\x90`\0\x93[\x85\x85\x10aP\xE6WPPPPPPV[`\x04\x84`\x01\x92\x84QaP\xF7\x81a\x1E6V[\x86T\x81R\x84\x87\x01T\x83\x82\x01R`\x02\x87\x01T\x86\x82\x01R`\x01`\x01`\xA0\x1B\x03`\x03\x88\x01T\x16``\x82\x01R\x81R\x01\x93\x01\x94\x01\x93\x91aP\xD7V[\x81\x81\x03\x92\x91`\0\x13\x80\x15\x82\x85\x13\x16\x91\x84\x12\x16\x17a>\\WV[`\x0ET`\0\x80`\x0EU\x81aQXWPPV[\x7F?\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a>\\W`\x0E\x81R`\x02\x7F\xBB{JEM\xC3I9#H/\x07\x82#)\xED\x19\xE8$N\xFFX,\xC2\x04\xF8UL6 \xC3\xFD\x92\x81\x1B\x83\x01\x92[\x83\x81\x10aQ\xBCWPPPPV[\x80\x83`\x04\x92U\x83`\x01\x82\x01U\x83\x83\x82\x01U\x83`\x03\x82\x01U\x01aQ\xAFV[aQ\xE1aNkV[\x80Q\x80\x15aTFW`\x01\x90\x81\x80[aS\xA0W[PPPPaR\0aP\x86V[\x80Q\x80[aR\x1AWPPaR\x12aYmV[a\x1E\xD1aQFV[aR#\x81aF\xD2V[\x90aRB``aR3\x84\x86a@(V[Q\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[aRL\x83\x85a@(V[QQ` aRZ\x85\x87a@(V[Q\x01Q\x81\x15aSHWPaR\xCBaRw\x93\x94`@\x94\x85\x91\x88a@(V[Q\x01Q\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x90\x83\x16\x03aS7WaR\xBB\x913\x90aV>V[aR\xC50\x85aU}V[\x92aF\xE1V[\x90\x81\x81\x10aR\xE3WPPPP`\0\x19\x90[\x01\x80aR\x04V[a\x14l\x91aR\xF0\x91aQ-V[\x92Q\x92\x83\x92\x7F\xAA&\x9D\xE2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01` \x90\x93\x92\x91\x93`\x01`\x01`\xA0\x1B\x03`@\x82\x01\x95\x16\x81R\x01RV[aSC\x91P3\x86ad'V[aR\xBBV[\x80\x91PaS\\W[PP`\0\x19\x91PaR\xDCV[aS\x8FaSo\x93\x94`@\x94\x85\x91\x88a@(V[Q\x01Q\x91aS\x7F\x8103\x87ac\x9EV[aS\x890\x85aU}V[\x92aF\xEEV[\x90\x81\x81\x10aR\xE3WP\x83\x92PaSPV[\x15aT;W[`\0\x19`\0\x91aS\xC1a:\xF4aS\xBB\x83aF\xD2V[\x87a@(V[aS\xCB0\x82aV\xDFV[\x81\x15\x80\x15\x90aT2W[aS\xEDW[PPPaS\xE5aO\xEDV[\x01\x90\x82aQ\xEFV[aT*\x92aT%\x91aS\xFF0\x83aU}V[\x90aT\x08a\x1E\xD3V[\x94\x85R` \x85\x01R`@\x84\x01R`\x01`\x01`\xA0\x1B\x03\x16``\x83\x01RV[aN\xE1V[8\x80\x80aS\xDAV[P\x80\x15\x15aS\xD5V[\x80aS\xA6W\x80aQ\xF4V[PPa\x1E\xD1aYmV[\x90\x81` \x91\x03\x12a\x041WQa\x06\x15\x81a\x03\xCFV[\x90\x81` \x91\x03\x12a\x041WQ\x90V[\x90\x81`\xA0\x91\x03\x12a\x041W`\x80`@Q\x91aT\x8E\x83a\x1E\x15V[\x80QaT\x99\x81a\x0FDV[\x83R` \x81\x01QaT\xA9\x81a\x0FDV[` \x84\x01R`@\x81\x01QaT\xBC\x81a\x07\xBFV[`@\x84\x01R``\x81\x01QaT\xCF\x81a\x07\xADV[``\x84\x01R\x01QaT\xDF\x81a\x07\xBFV[`\x80\x82\x01R\x90V[\x90\x81``\x91\x03\x12a\x041W\x80QaT\xFD\x81a\x07\xBFV[\x91`@` \x83\x01Q\x92\x01Q\x90V[\x91`\xC0\x91\x94\x93`\x01`\x01`\xA0\x1B\x03\x91aUr\x85`\xE0\x81\x01\x98`\x80\x80\x91o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82Q\x16\x85R` \x82\x01Q\x16` \x85\x01R`@\x81\x01Q\x15\x15`@\x85\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF``\x82\x01Q\x16``\x85\x01R\x01Q\x15\x15\x91\x01RV[`\xA0\x85\x01R\x16\x91\x01RV[\x90`@Q\x91`\x01`\x01`\xA0\x1B\x03` \x84\x01\x92\x7Fp\xA0\x821\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R\x16`$\x84\x01R`$\x83R``\x83\x01\x91\x83\x83\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x11\x17a\x1E1W`\0\x93\x84\x93`@RQ\x91Z\xFAaU\xE8a?\xF8V[\x90\x15\x80\x15aV2W[aV\x08W\x80` \x80a\x06\x15\x93Q\x83\x01\x01\x91\x01aTeV[`\x04`@Q\x7F\xC5.>\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xFD[P` \x81Q\x14\x15aU\xF1V[`\x01`\x01`\xA0\x1B\x03\x16\x90\x81;\x15a\x07\xA8W`@Q\x80\x92\x7F.\x1A}M\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R\x81`$`\0\x95\x86\x80\x94\x89`\x04\x84\x01RZ\xF1\x80\x15a\x07\xA3WaV\xCCW[P\x81\x80\x93\x81\x92Z\xF1\x15aV\xA2WV[`\x04`@Q\x7F5e\x94\xAB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xFD[aV\xD8\x90\x92\x91\x92a\x1EoV[\x908aV\x93V[\x91`\0\x80\x81\x93aW\x02\x86`\x01`\x01`\xA0\x1B\x03\x16`\0R`\x02` R`@`\0 \x90V[T`\x01`\x01`\xA0\x1B\x03\x87\x16\x90\x81;\x15a\x07\xA8W` `\x04\x92`@Q\x93\x84\x80\x92\x7F1<\xE5g\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x90\x81\x15a\x07\xA3W`\xFFaWf\x92aWm\x94\x88\x91aX<W[P\x16\x90a[>V[\x91\x87aU}V[\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x83\x11aX8W\x81\x11aX4W\x90aW\xA5\x91aQ-V[\x91\x80\x83\x13\x15aW\xF6WPPaW\xCE\x90\x93`\x01`\x01`\xA0\x1B\x03\x16`\0R`\x03` R`@`\0 \x90V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x81T\x16\x90UV[\x82\x91\x95\x92\x12aX\x1EW[PaW\xCE\x90`\x01`\x01`\xA0\x1B\x03\x16`\0R`\x03` R`@`\0 \x90V[aW\xCE\x91\x93PaX-\x90aB\xB9V[\x92\x90aX\0V[\x83\x80\xFD[\x84\x80\xFD[aXT\x91P` =\x81\x11a+\xEFWa+\xE0\x81\x83a\x1E\x83V[8aW^V[`\x05T`\xFF\x81\x16aYBW[P`\x01`\x01`\xA0\x1B\x03\x81\x16\x90\x81`\0R`\x03` R`\xFF`@`\0 T\x16\x15aX\x8DWPPV[`\x04T\x90h\x01\0\0\0\0\0\0\0\0\x82\x10\x15a\x1E1W`\x01\x82\x01\x80`\x04U\x82\x10\x15a?\xC1WaY\n\x91\x7F\x8A5\xAC\xFB\xC1_\xF8\x1A9\xAE}4O\xD7\t\xF2\x8E\x86\0\xB4\xAA\x8Ce\xC6\xB6K\xFE\x7F\xE3k\xD1\x9B\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82T\x16\x17\x90UV[`\0R`\x03` R`@`\0 `\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x82T\x16\x17\x90UV[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x05U8aXfV[`\x04TaY\xA6W`\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0`\x05T\x16\x17`\x05U`\0`\x04UV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x01`\x04R`$`\0\xFD[\x90aY\xF3\x82`\x01`\x01`\xA0\x1B\x03\x16`\0R`\x02` R`@`\0 \x90V[T\x90`\x01`\x01`\xA0\x1B\x03\x83\x16\x91\x82;\x15a\x07\xA8W` `\x04\x93`@Q\x94\x85\x80\x92\x7F1<\xE5g\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x90\x81\x15a\x07\xA3W`\xFFaZX\x92aZ^\x95`\0\x91aX<WP\x16\x90a[>V[\x92aU}V[\x90\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11a\x03\xE0W\x82\x11a\x03\xE0Wa\x06\x15\x91aQ-V[o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x03\xE0W\x90V[\x90`\x12\x03`\x12\x81\x11a>\\WaZ\xC5\x90aZ\xC9V[\x02\x90V[`M\x81\x11a>\\W`\n\n\x90V[`\x12\x03`\x12\x81\x11a>\\Wa\x06\x15\x90aZ\xC9V[\x90`\x12\x03`\x12\x81\x11a>\\Wa[\0\x90aZ\xC9V[\x90\x04\x90V[\x81\x15a[\x0FW\x04\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[\x90\x81\x15a[|W`\x12\x03`\x12\x81\x11a>\\Wa[Y\x90aZ\xC9V[`\0\x19\x82\x01\x91\x82\x11a>\\Wa[n\x91a[\x05V[`\x01\x81\x01\x80\x91\x11a>\\W\x90V[PP`\0\x90V[`\0\x90\x15a[\xA6WPg\x0F\0\0\0\0\0\0\0`\x01\x91[` \x1B\x91`8\x1B\x16\x17\x17\x90V[g\x0F\0\0\0\0\0\0\0\x90\x91a[\x99V[\x93\x96\x95\x94\x91\x90a[\xC8a4c\x86aAOV[a^\xA7Wa[\xD6B\x86a^\xE3V[\x80\x15a^}Wa[\xE8a\\%\x91al<V[\x85\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82T\x16\x17\x90UV[\x80\x15a^SWa\\7a\\x\x91al<V[\x84To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x80\x91\x90\x91\x1B\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x17\x84UV[a\x03\xE8\x80\x83\x10\x90\x83\x14\x17`\x01\x83\x11`\x01\x84\x14\x17\x16\x15a^!Wa\\\x9A\x82a^\xD1V[\x91a\\\xEB`\x01\x85\x01\x93\x84\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFu\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83T\x92`\xA0\x1B\x16\x91\x16\x17\x90UV[`\x01`\x01`\xA0\x1B\x03\x82\x16a]9W[PPPa\x1E\xD1\x92\x93P`\x03\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82T\x16\x17\x90UV[\x80\x87\x10\x90\x87\x14\x17`\x01\x87\x11`\x01\x88\x14\x17\x16\x15a]\xEFW\x82\x91a]\x9Ea\x1E\xD1\x96\x97a]\x99a]\xE6\x94`\x02`\x03\x98\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82T\x16\x17\x90UV[a^\xD1V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFw\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83T\x92`\xB0\x1B\x16\x91\x16\x17\x90UV[\x90\x84\x938a\\\xFAV[`@Q\x7F\xED\xDF\xE1\x19\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x87\x90R`$\x90\xFD[`@Q\x7F\x97\x1B1\t\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x83\x90R`$\x90\xFD[`\x04`@Q\x7F(i\xC5\xF3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xFD[`\x04`@Q\x7F]?Pl\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xFD[`\x04`@Q\x7F\xE90\xCE\xDF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xFD[b\x01\0\0\x81\x10\x15a\x03\xE0Wa\xFF\xFF\x16\x90V[d\x01\0\0\0\0\x82\x10\x15a\x03\xE0W`\x01\x01\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFs\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83T\x92`\x80\x1B\x16\x91\x16\x17\x90UV[\x90`\x01a\x1E\xD1\x92\x01\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x83T\x16\x90`\0\x83`\x0F\x0B\x13`\0\x14a_\xABWa_p\x92\x16\x90aB\xE6V[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82T\x16\x17\x90UV[a_\xB7a_p\x93aB&V[\x16\x90aC\nV[\x92\x91\x90\x15a`\xA3Wa`5a`\x80\x92a!\x9Da_\xF8a_\xDFa`?\x95al<V[\x87To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aB\xE6V[\x86\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82T\x16\x17\x90UV[\x83T`\x80\x1CaC\nV[\x82To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x80\x91\x90\x91\x1B\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x17\x82UV[To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x15a^}W`\x80\x1C\x15a^SWV[a`\xD5a`?\x91a!\x9Da_\xF8a`\xBCa`\xDF\x96al<V[\x87To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aC\nV[\x83T`\x80\x1CaB\xE6V[a`\x80V[c\xFF\xFF\xFF\xFF``\x82\x01Q\x16\x15\x15\x90\x81a`\xFBWP\x90V[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91P`@\x01Q\x16\x15\x90V[\x81\x15\x80abRW[ab\x13W\x82\x15\x80ab\x1BW[ab\x13Wa\x06\x15\x92`\0\x92\x83\x92aaA\x81a`\xE4V[\x15aa\xEDWg\r\xE0\xB6\xB3\xA7d\0\0\x91[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFaa|\x83Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x16\x90\x81aa\xCFW[PP` \x01Qaa\xA5\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a4\x04V[\x91\x82aa\xBDW[PPP\x80\x82\x11\x90\x82\x03\x02\x90\x03al<V[aa\xC7\x93Pak\xD6V[8\x80\x80aa\xACV[aa\xA5\x92\x96Paa\xE5a4\x04\x92\x85` \x93ak\xD6V[\x96\x92Paa\x84V[ab\ra4\x04`@\x83\x01Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x91aaQV[PPP`\0\x90V[Po\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFabJ` \x83\x01Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x16\x15\x15aa+V[Po\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFab~\x82Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x16\x15\x15aa\x1FV[\x91\x90\x80`\x0F\x0B\x90\x81\x15ac\x92W`\0ab\xB5a4\x04`@\x87\x01Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x92\x13\x15acDWa\x06\x15\x91a!\x9D\x91ab\xCD\x86a`\xE4V[ac4W[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16ac.a4\x04` ac\x16a!\x9D\x86ac\x10a4\x04\x8DQo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x87al\x14V[\x98\x01Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x90al\x14V[g\r\xE0\xB6\xB3\xA7d\0\0\x91Pab\xD2V[a\x06\x15\x91acZa4\x04a4\x04a!\x9D\x94aB&V[ac\x8Ca4\x04` ac\x16a!\x9D\x86ac\x86a4\x04\x8DQo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x87ak\xD6V[\x90ak\xD6V[PP\x90P`\0\x90`\0\x90V[\x91\x92`d` \x92\x94`@Q\x95`\0\x95\x86\x94\x85\x93\x84\x93\x7F#\xB8r\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85R`\x04R`$R`DRZ\xF1=\x15`\x1F=\x11`\x01\x84Q\x14\x16\x17\x16\x90``R\x81`@R\x15ac\xFFWPV[\x80\x7Fn\x89\xEC\xA5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x92R\xFD[\x90\x91` \x90`D`@Q\x94`\0\x80\x95\x81\x94\x82\x93\x7F\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04R`$RZ\xF1=\x15`\x1F=\x11`\x01\x84Q\x14\x16\x17\x16\x90``R\x81`@R\x15ad\x83WPV[\x80\x7F\xEB,\xF4\xFC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x92R\xFD[\x91\x94\x92\x94`\0ad\xCFo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x97\x88\x86Q\x16ak\xEEV[\x95\x80af\x7FW[P\x80\x95\x80\x97`\x80\x86\x01\x92ae*ad\xED\x85Q\x15\x15\x90V[\x93\x84\x15afxW\x87\x94[\x15afnWae%\x87\x95[ae\x1Fa4\x04\x8CQo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x90aF\xEEV[aF\xE1V[\x90\x81\x81\x11afDWae?` \x91\x85\x93aF\xE1V[\x97\x01\x91ae\\\x83Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x16\x11af\x1AWae\x93\x91ae\x86a4\x04ae\x8C\x93Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x90aF\xE1V[\x91Q\x15\x15\x90V[\x90\x81\x15af\x11W\x84\x85\x92[\x15af\tWP\x92[\x14ae\xDFW\x81\x14ae\xB5W\x90\x91V[`\x04`@Q\x7F\x1F\xB0\xB7\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xFD[`\x04`@Q\x7Frv\xF0\x8A\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xFD[\x90P\x92ae\xA6V[\x80\x94\x85\x92ae\x9EV[`\x04`@Q\x7F\x86j\x03+\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xFD[`\x04`@Q\x7F\xEC\x8E\x1F\xCE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xFD[ae%\x88\x95ae\x02V[\x86\x94ad\xF7V[af\x8A\x91P\x86a[\x05V[\x94\x85\x81\x03\x90\x81\x11a>\\W\x948ad\xD6V[af\xA4aFtV[P`@\x81\x01\x80Q\x90af\xC0a\x01\0\x84\x01\x92`\xFF\x84Q\x16\x90aZ\xB0V[\x90R``\x82\x01af\xD6\x81Q`\xFF\x84Q\x16\x90aZ\xB0V[\x90Raf\xED`\x80\x83\x01\x91`\xFF\x83Q\x91Q\x16\x90aZ\xB0V[\x90R`\xA0\x81\x01ag\x07\x81Q`\xFFa\x01 \x85\x01Q\x16\x90aZ\xB0V[\x90R\x90V[ag\x14aFtV[P`@\x81\x01\x80Q\x90ag0a\x01\0\x84\x01\x92`\xFF\x84Q\x16\x90aZ\xEBV[\x90R``\x82\x01agF\x81Q`\xFF\x84Q\x16\x90aZ\xEBV[\x90Rag]`\x80\x83\x01\x91`\xFF\x83Q\x91Q\x16\x90aZ\xEBV[\x90R`\xA0\x81\x01ag\x07\x81Q`\xFFa\x01 \x85\x01Q\x16\x90aZ\xEBV[\x15ag~WV[`d`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01R\x7FNOT_AUTHORIZED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R\xFD[\x15ag\xC9WV[`d`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01R\x7FINVALID_RECIPIENT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R\xFD[\x90\x81` \x91\x03\x12a\x041WQa\x06\x15\x81a\x04;V[\x91\x92a\x06\x15\x96\x94\x91`\xA0\x94`\x01`\x01`\xA0\x1B\x03\x80\x92\x16\x85R\x16` \x84\x01R`@\x83\x01R``\x82\x01R\x81`\x80\x82\x01R\x01\x91aHZV[\x15ah^WV[`d`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01R\x7FUNSAFE_RECIPIENT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R\xFD[\x15ah\xA9WV[`d`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01R\x7FLENGTH_MISMATCH\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R\xFD[\x91\x90\x81\x10\x15a?\xC1W`\x05\x1B\x01\x90V[\x90\x91\x82\x81R\x7F\x07\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11ai:W` \x92`\x05\x1B\x80\x92\x84\x83\x017\x01\x01\x90V[`\x84`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FABI encoding: array data too lon`D\x82\x01R\x7Fg\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R\xFD[\x92\x90ai\xBD\x90a\x06\x15\x95\x93`@\x86R`@\x86\x01\x91ah\xFDV[\x92` \x81\x85\x03\x91\x01Rah\xFDV[\x96\x94\x92aj\x0E\x94aj\0\x92a\x06\x15\x9A\x98\x94`\x01`\x01`\xA0\x1B\x03\x80\x92\x16\x8BR\x16` \x8A\x01R`\xA0`@\x8A\x01R`\xA0\x89\x01\x91ah\xFDV[\x91\x86\x83\x03``\x88\x01Rah\xFDV[\x92`\x80\x81\x85\x03\x91\x01RaHZV[\x90\x92`\xA0\x92`\x01`\x01`\xA0\x1B\x03a\x06\x15\x96\x95\x16\x83R`\0` \x84\x01R`@\x83\x01R``\x82\x01R\x81`\x80\x82\x01R\x01\x90a\x05\xC1V[\x92\x91\x90\x91ajt\x83a\x0E\xA7\x86`\x01`\x01`\xA0\x1B\x03\x16`\0R`\0` R`@`\0 \x90V[aj\x7F\x83\x82TaF\xEEV[\x90U`\x01`\x01`\xA0\x1B\x03\x84\x16\x93\x84`\0`@Q\x7F\xC3\xD5\x81h\xC5\xAEs\x97s\x1D\x06=[\xBF=exTBsC\xF4\xC0\x83$\x0Fz\xAC\xAA-\x0Fb3\x91\x80aj\xCE\x89\x8B\x83` \x90\x93\x92\x91\x93`@\x81\x01\x94\x81R\x01RV[\x03\x90\xA4;\x15ak_W\x83;\x15a\x07\xA8Wak\x1F\x93` \x92`\0`@Q\x80\x97\x81\x95\x82\x94\x7F\xF2:na\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x99\x8A\x85R3`\x04\x86\x01aj\x1CV[\x03\x92Z\xF1\x91\x82\x15a\x07\xA3Wa\x1E\xD1\x92\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91`\0\x91a\x0E$WP\x16\x14ahWV[PPPa\x1E\xD1\x90\x15\x15ag\xC2V[`\x01`\x01`\xA0\x1B\x03\x16\x90`\0\x92\x82\x84R\x83` R`@\x84 \x82\x85R` R`@\x84 \x80T\x91\x80\x83\x03\x92\x83\x11a>\\W\x91\x90U`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R3\x91\x7F\xC3\xD5\x81h\xC5\xAEs\x97s\x1D\x06=[\xBF=exTBsC\xF4\xC0\x83$\x0Fz\xAC\xAA-\x0Fb\x91\x90\xA4V[\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17\x82\x15\x15\x16\x15a\x03\xE0W\x04\x90V[\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x03\xE0W`\x01a'\x10`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[\x91\x90\x91\x82\x81\x02\x92\x81\x84\x04\x14\x90\x15\x17\x81\x15\x15\x16\x15a\x03\xE0W`\x01\x90`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x10\x15a\x03\xE0Wo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V";
    /// The bytecode of the contract.
    pub static PORTFOLIO_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10\x15a\0\x1DW[6a<HWa\0\x1Ba<\xB2V[\0[`\x005`\xE0\x1C\x80b\xFD\xD5\x8E\x14a\x02\x8CW\x80c\x01\xFF\xC9\xA7\x14a\x02\x87W\x80c\x06C;\x1B\x14a\x02\x82W\x80c\x07\x88\x88\xD6\x14a\x02}W\x80c\x0E\x894\x1C\x14a\x02xW\x80c\x19\x05x\x07\x14a\x02sW\x80c&z\x0C\xFE\x14a\x02nW\x80c*\xFB\x9D\xF8\x14a\x02iW\x80c.\xB2\xC2\xD6\x14a\x02dW\x80c/\x9E8\xE2\x14a\x02_W\x80c0$K\xE7\x14a\x02ZW\x80c9CMZ\x14a\x02UW\x80c?\x92\xA39\x14a\x02PW\x80cM\xC6\x8A\x90\x14a\x02KW\x80cN\x12s\xF4\x14a\x02FW\x80c[\xC5Td\x14a\x02AW\x80c^Gf<\x14a\x02<W\x80cx}\xCE=\x14a\x027W\x80c\x80\xAF\x9Dv\x14a\x022W\x80c\x89\x92\xF2\n\x14a\x02-W\x80c\x89\xA5\xF0\x84\x14a\x02(W\x80c\x8Ag\x89g\x14a\x02#W\x80c\xA2,\xB4e\x14a\x02\x1EW\x80c\xA5\xCD\x8AI\x14a\x02\x19W\x80c\xAC\x96P\xD8\x14a\x02\x14W\x80c\xAD\\FH\x14a\x02\x0FW\x80c\xB0\xC3\xA9P\x14a\x02\nW\x80c\xB0\xE2\x1E\x8A\x14a\x02\x05W\x80c\xC9\xA3\x96\xE9\x14a\x02\0W\x80c\xC9\xC6S\x96\x14a\x01\xFBW\x80c\xD6\xB7\xDE\xC5\x14a\x01\xF6W\x80c\xDC\xF8D\xA7\x14a\x01\xF1W\x80c\xDD\xA4\x07\x97\x14a\x01\xECW\x80c\xE31\xBA4\x14a\x01\xE7W\x80c\xE9\x85\xE9\xC5\x14a\x01\xE2W\x80c\xF0{\x87\x9E\x14a\x01\xDDW\x80c\xF2BC*\x14a\x01\xD8W\x80c\xF3:\xE1\xBC\x14a\x01\xD3Wc\xFF\xA1\xADt\x03a\0\x0EWa<\x1CV[a3\xC8V[a2%V[a0\xEEV[a0DV[a/\xA7V[a-\x9DV[a-_V[a,wV[a(\xE7V[a(\xA9V[a(\x8BV[a(GV[a(\x03V[a&\x9CV[a%\xE0V[a%%V[a\"\xE2V[a!\xFDV[a \xD9V[a\x1F\xB8V[a\x1CfV[a\x1C\x02V[a\x17\xF3V[a\x16\xDAV[a\x16nV[a\x16\x1FV[a\x15]V[a\x15\x03V[a\x0F^V[a\x0C\x95V[a\x0B@V[a\n\xA3V[a\x07\xC9V[a\x06\x18V[a\x05{V[a\x057V[a\x04eV[a\x03\xE5V[`\x84`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R\xFD[`\x84`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01R\x7Frt\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R\xFD[`\x84`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01R\x7Fet\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\x03\xE0WV[`\0\x80\xFD[4a\x046W`@`\x03\x196\x01\x12a\x041W`\x01`\x01`\xA0\x1B\x03`\x045a\x04\n\x81a\x03\xCFV[\x16`\0R`\0` R`@`\0 `$5`\0R` R` `@`\0 T`@Q\x90\x81R\xF3[a\x02\xFBV[a\x02\x91V[\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x03a\x03\xE0WV[4a\x046W` `\x03\x196\x01\x12a\x041W` \x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x045a\x04\xA5\x81a\x04;V[\x16\x7F\x01\xFF\xC9\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x14\x90\x81\x15a\x05\rW[\x81\x15a\x04\xE3W[P`@Q\x90\x15\x15\x81R\xF3[\x7F\x0E\x894\x1C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91P\x148a\x04\xD8V[\x7F\xD9\xB6z&\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x14\x91Pa\x04\xD1V[4a\x046W`\0`\x03\x196\x01\x12a\x041W` `@Q`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[4a\x046W`\0`\x03\x196\x01\x12a\x041W` b\xFF\xFF\xFF`\x06T\x16`@Q\x90\x81R\xF3[`\0[\x83\x81\x10a\x05\xB1WPP`\0\x91\x01RV[\x81\x81\x01Q\x83\x82\x01R` \x01a\x05\xA1V[\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F` \x93a\x05\xFD\x81Q\x80\x92\x81\x87R\x87\x80\x88\x01\x91\x01a\x05\x9EV[\x01\x16\x01\x01\x90V[\x90` a\x06\x15\x92\x81\x81R\x01\x90a\x05\xC1V[\x90V[4a\x046W` \x80`\x03\x196\x01\x12a\x041W`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x80;\x15a\x07\xA8W`\0`$\x91`@Q\x92\x83\x80\x92\x7F\x0E\x894\x1C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x045`\x04\x83\x01RZ\xFA\x90\x81\x15a\x07\xA3W`\0\x91a\x06\xB5W[`@Q\x80a\x06\xB1\x84\x82a\x06\x04V[\x03\x90\xF3[\x90=\x80\x91\x83>a\x06\xC5\x81\x83a\x1E\x83V[\x81\x01\x90\x82\x81\x83\x03\x12a\x041W\x80Q\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x07\x9EW\x01\x81`\x1F\x82\x01\x12\x15a\x07\x99W\x80Q\x90a\x06\xFB\x82a=NV[\x92a\x07\t`@Q\x94\x85a\x1E\x83V[\x82\x84R\x84\x83\x83\x01\x01\x11a\x07/W\x90a\x07)\x91\x84a\x06\xB1\x95\x85\x01\x91\x01a\x05\x9EV[8a\x06\xA3V[`\x84\x84`@Q\x90bF\x1B\xCD`\xE5\x1B\x82R`\x04\x82\x01R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01R\x7F length\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R\xFD[a\t-V[a\x03eV[a=\x88V[a<\xE4V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x03a\x03\xE0WV[\x80\x15\x15\x03a\x03\xE0WV[4a\x046W`\x80`\x03\x196\x01\x12a\x041W`\x045a\x07\xE6\x81a\x07\xADV[`$5a\x07\xF2\x81a\x07\xBFV[`d5a\x07\xFE\x81a\x03\xCFV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16`\0R`\n` Ra\x08<a\x080a\x080`\x03`@`\0 \x01`\x01`\x01`\xA0\x1B\x03\x90T\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x80;\x15a\x07\xA8W`@Q\x7F\x19\x05x\x07\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x94\x16`\x04\x85\x01R\x91\x15\x15`$\x84\x01R`D\x805\x90\x84\x01R`\x01`\x01`\xA0\x1B\x03\x16`d\x83\x01R` \x90\x82\x90\x81\x80`\x84\x81\x01[\x03\x91Z\xFA\x80\x15a\x07\xA3Wa\x06\xB1\x91`\0\x91a\x08\xCBW[P`@Q\x90\x81R\x90\x81\x90` \x82\x01\x90V[a\x08\xEC\x91P` =\x81\x11a\x08\xF2W[a\x08\xE4\x81\x83a\x1E\x83V[\x81\x01\x90aTeV[8a\x08\xBAV[P=a\x08\xDAV[`\x045\x90b\xFF\xFF\xFF\x82\x16\x82\x03a\x03\xE0WV[`$5\x90a\xFF\xFF\x82\x16\x82\x03a\x03\xE0WV[`D5\x90a\xFF\xFF\x82\x16\x82\x03a\x03\xE0WV[`\x84`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01R\x7Frray offset\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R\xFD[`\x84`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01R\x7Frray length\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R\xFD[`\x84`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01R\x7Frray stride\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R\xFD[\x91\x81`\x1F\x84\x01\x12\x15a\x07\x99W\x825\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\n\x9EW` \x83\x81\x86\x01\x95\x01\x01\x11a\n\x99WV[a\n\x01V[a\t\x97V[a\x01\0`\x03\x196\x01\x12a\x041Wa\n\xB8a\x08\xF9V[a\xFF\xFF\x90`d5\x82\x81\x16\x81\x03a\x03\xE0W`\x845\x92\x83\x16\x83\x03a\x03\xE0W`\xA45\x91a\n\xE1\x83a\x03\xCFV[`\xC45a\n\xED\x81a\x03\xCFV[`\xE45\x93g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x11a\x07\x9EWa\x06\xB1\x95a\x0B\x15a\x0B%\x966\x90`\x04\x01a\nkV[\x95\x90\x94`D5\x90`$5\x90aH\xBAV[`@Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R\x90\x81\x90` \x82\x01\x90V[4a\x046W` `\x03\x196\x01\x12a\x041W`\x045a\x0B]\x81a\x07\xADV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16`\0R`\n` Ra\x0B}`@`\0 aAOV[`@\x81\x01\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFo\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x84Q\x16\x11a\x0C:Wa\x0C\x18a\x0C\x12``a\x0C\x1Fa\x0B\xFFa\x0B\xFAa\x0B\xDEb\xFF\xFF\xFF\x98a\x0B\xD8\x89a\x0C(\x9CQ\x16`\x0F\x0BaB&V[\x90ab\x86V[\x98\x90\x9A` \x1C\x16b\xFF\xFF\xFF\x16`\0R`\t` R`@`\0 \x90V[aA\x0CV[\x97\x85a\x0C\x18a\x0C\x12` \x8C\x01Q`\xFF\x16\x90V[`\xFF\x16\x90V[\x91\x16aZ\xEBV[\x96\x01Q`\xFF\x16\x90V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x90\xF3[`\x04`@Q\x7F\xAC\xC9@{\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xFD[\x91\x81`\x1F\x84\x01\x12\x15a\x07\x99W\x825\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\n\x9EW` \x80\x85\x01\x94\x84`\x05\x1B\x01\x01\x11a\n\x99WV[4a\x046W`\xA0`\x03\x196\x01\x12a\x041W`\x045a\x0C\xB2\x81a\x03\xCFV[`$5\x90a\x0C\xBF\x82a\x03\xCFV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91`D5\x83\x81\x11a\x07\x9EWa\x0C\xE1\x906\x90`\x04\x01a\x0CdV[\x93\x90\x91`d5\x82\x81\x11a\x07\x9EWa\x0C\xFC\x906\x90`\x04\x01a\x0CdV[\x95\x90\x92`\x845\x90\x81\x11a\x07\x9EWa\r\x17\x906\x90`\x04\x01a\nkV[\x90\x94a\r$\x88\x84\x14ah\xA2V[`\x01`\x01`\xA0\x1B\x03\x93\x84\x88\x16\x94\x853\x14\x80\x15a\x0E\xF6W[a\rD\x90agwV[\x84`\0\x8A\x89\x86\x8E[\x85\x85\x10a\x0EeWPPPPPP\x81\x16\x80\x95\x8A\x7FJ9\xDC\x06\xD4\xC0\xDB\xC6Kp\xAF\x90\xFDi\x8A#:Q\x8A\xA5\xD0~Y]\x98;\x8C\x05&\xC8\xF7\xFB`@Q\x80a\r\x913\x94\x8D\x8C\x8B\x85ai\xA4V[\x03\x90\xA4;\x15a\x0ERW\x83;\x15a\x07\xA8Wa\r\xE3`\0\x92` \x97`@Q\x9A\x8B\x98\x89\x97\x88\x96\x7F\xBC\x19|\x81\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x9D\x8E\x89R3`\x04\x8A\x01ai\xCBV[\x03\x92Z\xF1\x91\x82\x15a\x07\xA3Wa\0\x1B\x92\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91`\0\x91a\x0E$W[P\x16\x14ahWV[a\x0EE\x91P` =\x81\x11a\x0EKW[a\x0E=\x81\x83a\x1E\x83V[\x81\x01\x90ah\rV[8a\x0E\x1CV[P=a\x0E3V[PPP\x92PPPa\0\x1B\x91P\x15\x15ag\xC2V[a\x0E\xB6a\x0E\xE8\x93a\x0E\xA7a\x0E\x8C\x88a\x0E\x83\x81`\x01\x9Ca\x0E\xE0\x99ah\xEDV[5\x95\x86\x94ah\xEDV[5\x96`\x01`\x01`\xA0\x1B\x03\x16`\0R`\0` R`@`\0 \x90V[\x90`\0R` R`@`\0 \x90V[a\x0E\xC1\x85\x82TaF\xE1V[\x90Ua\x0E\xA7\x88`\x01`\x01`\xA0\x1B\x03\x16`\0R`\0` R`@`\0 \x90V[\x91\x82TaF\xEEV[\x90U\x01\x85\x90\x8A\x89\x86\x8Ea\rLV[Pa\rDa\x0F=a\x0F63a\x0F\x1E\x8D`\x01`\x01`\xA0\x1B\x03\x16`\0R`\x01` R`@`\0 \x90V[\x90`\x01`\x01`\xA0\x1B\x03\x16`\0R` R`@`\0 \x90V[T`\xFF\x16\x90V[\x90Pa\r;V[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x03a\x03\xE0WV[`\xC0`\x03\x196\x01\x12a\x041W`\x04\x805\x90a\x0Fx\x82a\x07\xBFV[`$5a\x0F\x84\x81a\x03\xCFV[`D5\x90a\x0F\x91\x82a\x07\xADV[`d5\x90a\x0F\x9E\x82a\x0FDV[`\x845\x91a\x0F\xAB\x83a\x0FDV[`\xA45\x93a\x0F\xB8\x85a\x0FDV[\x90\x81a\x0F\xC2a@<V[`\x0FT`\xFF\x16\x15a\x14\xF6W[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x91\x16\x15a\x14\xDDW[`\x01`\x01`\xA0\x1B\x03\x97\x88a\x10\x1B`\x03a\x10\r\x87g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0R`\n` R`@`\0 \x90V[\x01T`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x16\x15a\x14\x9DWa\x10Ja\x080a\x080`\x03a\x10\r\x88g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0R`\n` R`@`\0 \x90V[\x95\x86;\x15a\x07\xA8W`@\x80Q\x7F\xE6\x04{\x19\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x81\x8C\x01\x90\x81R\x91\x98` \x93\x92\x84\x91\x83\x91\x82\x90\x81\x90\x85\x01\x03\x91Z\xFA\x90\x81\x15a\x07\xA3W`\0\x91a\x14pW[P\x15a\x14-Wa\x10\xD4\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x80\x9B\x16\x91\x16\x87aM\x9FV[a\x10\xF9a\x0B\xFAb\xFF\xFF\xFF\x89\x86\x95\x96\x1C\x16b\xFF\xFF\xFF\x16`\0R`\t` R`@`\0 \x90V[\x93a\x13\x99W[\x89\x86\x16\x15a\x13qW\x89\x90\x81\x80a\x11=a\x114a\x11/\x8Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0R`\n` R`@`\0 \x90V[aAOV[a\x0B\xD8\x8BaZ\x96V[\x9D\x90\x9D\x16\x9C\x16\x94\x16\x84\x11a\x13IW\x16\x89\x11a\x13!W\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x96\x88\x84\x01\x97\x88Qa\x11t\x90`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x90a\x11~\x88aZ\x96V[\x92a\x11\x87a\x1E\xC4V[\x93B\x85R\x86\x86\x86\x01R\x8D\x8D\x86\x01R``\x85\x01\x90a\x11\xA6\x91\x90`\x0F\x0B\x90RV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8A\x16`\x80\x85\x01R`\x01`\x01`\xA0\x1B\x03\x16`\xA0\x84\x01R`\x01`\x01`\xA0\x1B\x03\x16`\xC0\x83\x01R`\x01`\x01`\xA0\x1B\x03\x16`\xE0\x82\x01Ra\x11\xE9\x90aC,V[\x82\x01Q`\xFF\x16`\xFF\x16a\x11\xFB\x91aZ\xEBV[\x96``\x82\x01Qa\x12\x0B\x90`\xFF\x16\x90V[`\xFF\x16a\x12\x17\x91aZ\xEBV[\x97\x87\x15\x80a\x13\x19W[a\x12\xF2WP\x97\x7F\xFD\xFF\xEC\xA7Q\xF0\xDC\xAA\xB7U1\xCB\x81<\x12\xBB\xFDV\xEE>\x96L\xC4q\xD7\xEFC\x93$\x02\xEE\x18\x91a\x12\xAAa\x12ra\x12da\x06\xB1\x9A\x9B\x9CQ`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x97Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x88Q\x8B\x81R` \x81\x01\x8D\x90Ro\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x96\x16`@\x87\x01R\x83\x16\x96\x90\x92\x16\x94\x16\x92\x90\x81\x90``\x82\x01\x90V[\x03\x90\xA4a\x12\xC1a\x12\xBC`\x0FT`\xFF\x16\x90V[\x15\x15\x90V[\x15a\x12\xE5W[a\x12\xCFa@\x9DV[Q\x91\x82R` \x82\x01\x92\x90\x92R\x90\x81\x90`@\x82\x01\x90V[a\x12\xEDaQ\xD9V[a\x12\xC7V[\x86Q\x7Fe\x8B\x16\xED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xFD[P\x88\x15a\x12 V[\x89\x88Q\x7F\x84\xC0Z\\\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xFD[\x8B\x8AQ\x7F\xAC\xE4\x1C:\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xFD[\x8A\x89Q\x7F\x90`\x9A}\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xFD[\x94Pa\x14\x16a\x13\xE9a\x13\xBAa\x13\xB5\x86Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[a=\x94V[a\x13\xD0a\x13\xB5\x8C\x88\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x90`\0\x81\x12a\x14%W[`\0\x82\x12a\x14\x1CW[\x89aM\x9FV[\x8B\x80a\x14\x0Ca\x11/\x8Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0R`\n` R`@`\0 \x90V[\x92\x16\x92\x16\x90aa\x17V[\x94a\x10\xFFV[`\0\x91Pa\x13\xE3V[P`\0a\x13\xDAV[\x87Q\x7F\xBC'\xA5\x17\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x81\x8C\x01\x90\x81R\x81\x90` \x01\x03\x90\xFD[\x03\x90\xFD[a\x14\x90\x91P\x83=\x85\x11a\x14\x96W[a\x14\x88\x81\x83a\x1E\x83V[\x81\x01\x90a@\xF7V[8a\x10\xADV[P=a\x14~V[`@Q\x7F\x12\x16\xE0\xFA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x81\x8A\x01\x90\x81R\x81\x90` \x01\x03\x90\xFD[`\x0FT\x90\x92P`\x08\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91a\x0F\xE0V[a\x14\xFEaK\xA8V[a\x0F\xCEV[4a\x046W` `\x03\x196\x01\x12a\x041W` a\x15L`\x045a\x15%\x81a\x07\xADV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0R`\n` R`\x01`\x01`\xA0\x1B\x03`\x03`@`\0 \x01T\x16\x90V[`\x01`\x01`\xA0\x1B\x03`@Q\x91\x16\x81R\xF3[4a\x046W` `\x03\x196\x01\x12a\x041W`\x045a\x15z\x81a\x07\xADV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16`\0R`\n` Ra\x15\xACa\x080a\x080`\x03`@`\0 \x01`\x01`\x01`\xA0\x1B\x03\x90T\x16\x90V[\x80;\x15a\x07\xA8W`@Q\x7F9CMZ\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16`\x04\x83\x01R` \x90\x82\x90\x81\x80`$\x81\x01a\x08\xA4V[`\x03\x19`@\x91\x01\x12a\x041W`\x045a\x16\x12\x81a\x03\xCFV[\x90`$5a\x06\x15\x81a\x03\xCFV[4a\x046W` b\xFF\xFF\xFFa\x16d`\x01`\x01`\xA0\x1B\x03a\x16>6a\x15\xFAV[\x91\x16`\0R`\x0B\x84R`@`\0 \x90`\x01`\x01`\xA0\x1B\x03\x16`\0R` R`@`\0 \x90V[T\x16`@Q\x90\x81R\xF3[4a\x046W` `\x03\x196\x01\x12a\x041W` a\x16\x97`\x045a\x16\x90\x81a\x03\xCFV[0\x90aY\xD5V[`@Q\x90\x81R\xF3[` \x90\x81`@\x81\x83\x01\x92\x82\x81R\x85Q\x80\x94R\x01\x93\x01\x91`\0[\x82\x81\x10a\x16\xC6WPPPP\x90V[\x83Q\x85R\x93\x81\x01\x93\x92\x81\x01\x92`\x01\x01a\x16\xB8V[4a\x046W`@\x80`\x03\x196\x01\x12a\x041Wg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90`\x045\x82\x81\x11a\x07\x9EWa\x17\x0E\x906\x90`\x04\x01a\x0CdV[\x91\x90\x92`$5\x90\x81\x11a\x07\x9EWa\x17)\x906\x90`\x04\x01a\x0CdV[\x93\x90a\x176\x85\x85\x14ah\xA2V[a\x17?\x84a=\x9FV[\x93a\x17L\x84Q\x95\x86a\x1E\x83V[\x80\x85R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0a\x17y\x82a=\x9FV[\x01` \x906\x82\x88\x017`\0\x91\x82[\x81\x81\x10a\x17\x9BW\x86Q\x80a\x06\xB1\x8A\x82a\x16\x9FV[\x80`\x01`\x01`\xA0\x1B\x03a\x17\xB1`\x01\x93\x85\x8Aah\xEDV[5a\x17\xBB\x81a\x03\xCFV[\x16\x85R\x84\x84Ra\x17\xE1\x88\x86 a\x17\xD2\x83\x8D\x8Aah\xEDV[5`\0R` R`@`\0 \x90V[Ta\x17\xEC\x82\x8Ba@(V[R\x01a\x17\x87V[`\xA0`\x03\x196\x01\x12a\x041W`\x04\x805a\x18\x0C\x81a\x07\xBFV[`$5\x91a\x18\x19\x83a\x07\xADV[`D5\x92a\x18&\x84a\x0FDV[`d5\x90a\x183\x82a\x0FDV[`\x845\x94a\x18@\x86a\x0FDV[\x90a\x18Ia@<V[`\x0FT`\xFF\x16\x15a\x1B\xF5W[a\x18~a\x080a\x080`\x03a\x10\r\x85g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0R`\n` R`@`\0 \x90V[\x91\x82;\x15a\x07\xA8W`@\x93\x84Q\x80\x94\x7F\xE6\x04{\x19\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R` \x95\x86\x91\x81\x80a\x18\xD4\x89\x8D\x83\x01\x91\x90\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x82\x01\x93\x16\x90RV[\x03\x91Z\xFA\x90\x81\x15a\x07\xA3W`\0\x91a\x1B\xD8W[P\x15a\x1B\x99Wa\x19\x0E\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x80\x9A\x16\x91\x16\x84aM\x9FV[\x93a\x192a\x0B\xFAb\xFF\xFF\xFF\x86\x84\x1C\x16b\xFF\xFF\xFF\x16`\0R`\t` R`@`\0 \x90V[\x96a\x19D\x88Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x95a\x19X\x88\x8A\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x99a\x1BNW[\x8A\x85\x16\x15a\x1B&W\x8A\x90\x81\x80a\x19\x9Fa\x19\x8Ea\x11/\x8Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0R`\n` R`@`\0 \x90V[a\x0B\xD8a\x19\x9A\x8BaZ\x96V[aB&V[\x9E\x90\x9E\x16\x9D\x16\x95\x16\x85\x10a\x1A\xFEW\x16\x8A\x10a\x1A\xD7WPa\x1A\x97\x89a\x1A\x88a\x0C\x12``a\x1A\x8E\x87\x8E\x9F\x8E\x9Fa\x06\xB1\x9F\x99\x8F\x9A\x8F\x8F\x8F\x7F0\x84\xCA\xF4\x89f\\\xAB\x07E,\xFEO=\x0E\xB5\xE0\xDC\x15\xEA\xC6\xFCe\x80\x98\x85\x8Ec\x9Ep\xE5:\x9F\x95g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x9Fa\x1A\x80\x94a\x1AU\x88\x95a\x1ADa\x0C\x12\x9Ba\x1A\x88\x9Da\x1Ak\x96a\x1A'a\x19\x9Aa\x1A{\x9BaZ\x96V[\x92a\x1A0a\x1E\xC4V[B\x81R\x9B\x8C\x01R\x8A\x01R`\x0F\x0B``\x89\x01RV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x80\x87\x01RV[3`\xA0\x86\x01R`\x01`\x01`\xA0\x1B\x03\x16`\xC0\x85\x01RV[`\x01`\x01`\xA0\x1B\x03\x16`\xE0\x83\x01RV[aC,V[\x01Q`\xFF\x16\x90V[\x90aZ\xEBV[\x9C\x01Q`\xFF\x16\x90V[\x86Q\x89\x81R` \x81\x01\x82\x90Ro\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x94\x16`@\x85\x01R\x98`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x95\x16\x93\x16\x91\x80``\x81\x01a\x12\xAAV[\x86Q\x7F\xAC\xB5\xBD\xEA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xFD[\x82\x89Q\x7FcD\x840\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xFD[P\x86Q\x7F\x14\xEF`^\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xFD[\x93Pa\x1B\x93a\x1B\x8D\x86a\x1Bt3`\x01`\x01`\xA0\x1B\x03\x16`\0R`\0` R`@`\0 \x90V[\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0R` R`@`\0 \x90V[Tal<V[\x93a\x19^V[\x84Q\x7F\x12\x16\xE0\xFA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x81\x88\x01\x90\x81R\x81\x90` \x01\x03\x90\xFD[a\x1B\xEF\x91P\x85=\x87\x11a\x14\x96Wa\x14\x88\x81\x83a\x1E\x83V[8a\x18\xE7V[a\x1B\xFDaK\xA8V[a\x18UV[4a\x046W` `\x03\x196\x01\x12a\x041Wb\xFF\xFF\xFFa\x1C\x1Fa\x08\xF9V[\x16`\0R`\t` R`\x80`@`\0 `\xFF`\x01\x82T\x92\x01T`@Q\x92\x82`\x01`\x01`\xA0\x1B\x03\x91\x82\x81\x16\x86R`\xA0\x1C\x16` \x85\x01R\x81\x16`@\x84\x01R`\xA0\x1C\x16``\x82\x01R\xF3[4a\x046W` `\x03\x196\x01\x12a\x041W`\x045a\x1C\x82a@<V[`\x01`\x01`\xA0\x1B\x03\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x80;\x15a\x07\xA8W` `\x04\x91`@Q\x92\x83\x80\x92\x7F\xF7|G\x91\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x90\x81\x15a\x07\xA3W`\0\x91a\x1D\xB8W[P\x163\x03a\x1D\x8EW`\x14\x81\x11\x80\x15a\x1D\x84W[a\x1DQW\x7F\x81\xC99\x14H\0(v\x03G\x9B\x97\xBB\xA9\xC1\x12\x88\xCEz\xBCZ\xCBH\x90y\xE1Y\xF3\\\xF9\x8B\xD1\x90`\rTa\x1D9\x82`\rUV[`@\x80Q\x91\x82R` \x82\x01\x92\x90\x92R\xA1a\0\x1Ba@\x9DV[`@Q\x7FdYtw\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x91\x90\x91R`$\x90\xFD[P`\x04\x81\x10a\x1D\x07V[`\x04`@Q\x7F\xFF\xBE\x9C,\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xFD[a\x1D\xD9\x91P` =\x81\x11a\x1D\xDFW[a\x1D\xD1\x81\x83a\x1E\x83V[\x81\x01\x90aTPV[8a\x1C\xF4V[P=a\x1D\xC7V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`\xA0\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x1E1W`@RV[a\x1D\xE6V[`\x80\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x1E1W`@RV[a\x01\0\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x1E1W`@RV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x1E1W`@RV[\x90`\x1F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x1E1W`@RV[`@Q\x90a\x1E\xD1\x82a\x1ERV[V[`@Q\x90a\x1E\xD1\x82a\x1E6V[`\x03\x19`\xA0\x91\x01\x12a\x1FNW`@Q\x90a\x1E\xF9\x82a\x1E\x15V[\x81`\x045a\x1F\x06\x81a\x0FDV[\x81R`$5a\x1F\x14\x81a\x0FDV[` \x82\x01R`D5a\x1F%\x81a\x07\xBFV[`@\x82\x01R`d5a\x1F6\x81a\x07\xADV[``\x82\x01R`\x80`\x845\x91a\x1FJ\x83a\x07\xBFV[\x01RV[`\x84`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FABI decoding: struct data too sh`D\x82\x01R\x7Fort\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R\xFD[4a\x046W`\xE0`\x03\x196\x01\x12a\x041Wa\x1F\xD26a\x1E\xE0V[`\xC45\x90a\x1F\xDF\x82a\x03\xCFV[`\x01`\x01`\xA0\x1B\x03a  g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF``\x84\x01Q\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0R`\n` R`\x01`\x01`\xA0\x1B\x03`\x03`@`\0 \x01T\x16\x90V[\x16\x91\x82;\x15a\x07\xA8Wa i\x92``\x92`@Q\x80\x95\x81\x94\x82\x93\x7F\x80\xAF\x9Dv\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\xA45\x90`\x04\x85\x01aU\x0BV[\x03\x91Z\xFA\x80\x15a\x07\xA3W`\0\x90\x81\x92\x82\x91a \xA3W[Pa\x06\xB1\x90`@Q\x93\x84\x93\x84`@\x91\x94\x93\x92``\x82\x01\x95\x15\x15\x82R` \x82\x01R\x01RV[\x90Pa\x06\xB1\x92Pa \xCB\x91P``=\x81\x11a \xD2W[a \xC3\x81\x83a\x1E\x83V[\x81\x01\x90aT\xE7V[\x90\x92a \x7FV[P=a \xB9V[4a\x046W`@`\x03\x196\x01\x12a\x041W`\x045a \xF6\x81a\x07\xADV[`$5\x90\x81`\x0F\x0B\x91\x82\x81\x03a\x03\xE0Wb\xFF\xFF\xFF\x90a!oa\x0B\xFAa!=`\0\x93a!8a\x11/\x88g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0R`\n` R`@`\0 \x90V[ab\x86V[\x94\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x91\x16\x95\x16\x95` \x1C\x16b\xFF\xFF\xFF\x16`\0R`\t` R`@`\0 \x90V[\x93\x12\x15a!\xCAWa!\xA2\x91a\x1A\x88a\x0C\x12``a\x0C\x1Fa!\x9Da!\x9D\x96a\x1A\x88a\x0C\x12` \x8C\x01Q`\xFF\x16\x90V[al<V[\x90[`@\x80Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x83\x16\x81R\x92\x90\x91\x16` \x83\x01R\x90\xF3[a!\xF7\x91a!\xF1a\x0C\x12``a\x0C\x1Fa!\x9Da!\x9D\x96a!\xF1a\x0C\x12` \x8C\x01Q`\xFF\x16\x90V[\x90a[>V[\x90a!\xA4V[4a\x046W` `\x03\x196\x01\x12a\x041Wg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x045a\"#\x81a\x07\xADV[\x16`\0R`\n` R`@`\0 \x80Ta\x06\xB1`\x01\x83\x01T\x92a\"d`\x03a\"U`\x02\x84\x01T`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x92\x01T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`@\x80Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x81\x16\x82R`\x80\x96\x87\x1C` \x83\x01R\x87\x16\x91\x81\x01\x91\x90\x91Rc\xFF\xFF\xFF\xFF\x86\x86\x1C\x16``\x82\x01Ra\xFF\xFF`\xA0\x87\x81\x1C\x82\x16\x96\x83\x01\x96\x90\x96R`\xB0\x96\x90\x96\x1C\x90\x95\x16\x93\x85\x01\x93\x90\x93R`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\xC0\x85\x01R\x90\x91\x16`\xE0\x83\x01R\x81\x90a\x01\0\x82\x01\x90V[4a\x046W```\x03\x196\x01\x12a\x041W`\x045a\"\xFF\x81a\x07\xADV[a#\x07a\t\x0BV[a#\x0Fa\t\x1CV[\x91a#\x18a@<V[a#6\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0R`\n` R`@`\0 \x90V[\x91a#K`\x02\x84\x01T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x033\x91\x16\x03a\x1D\x8EWa\xFF\xFF\x80\x85\x16\x94\x85a$\x80W[P\x81\x16\x92\x83a#\xABW[PPg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x80N\x0E\xF8\xEB\xD1\x19o\x98\xB3\xC6\xA20\xDE\xFF\xD8\xCF\xE0<\xB1\x92?\xE0\xE4\x02%-\x06\xD8\xD4v\xDA`\0\x80\xA4a\0\x1Ba@\x9DV[`\x01\x01a#\xE8a#\xE4a#\xCEa#\xC7\x84Ta\xFF\xFF\x90`\xA0\x1C\x16\x90V[a\xFF\xFF\x16\x90V[\x86\x90\x80\x82\x10\x90\x82\x14\x17\x90`\x01\x80\x82\x11\x91\x14\x17\x16\x90V[\x15\x90V[a$KWg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x91a$D\x91\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFw\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83T\x92`\xB0\x1B\x16\x91\x16\x17\x90UV[\x908a#rV[`@Q\x7F\xED\xDF\xE1\x19\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Ra\xFF\xFF\x83\x16`\x04\x82\x01R`$\x90\xFD[a\x03\xE8\x80\x87\x10\x90\x87\x14\x17`\x01\x87\x11`\x01\x88\x14\x17\x16\x15a$\xEEWa$\xE8\x90`\x01\x86\x01\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFu\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83T\x92`\xA0\x1B\x16\x91\x16\x17\x90UV[8a#hV[`@Q\x7F\x97\x1B1\t\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Ra\xFF\xFF\x91\x90\x91\x16`\x04\x82\x01R`$\x90\xFD[4a\x046W`@`\x03\x196\x01\x12a\x041W`\x045a%B\x81a\x03\xCFV[`\x01`\x01`\xA0\x1B\x03`$5\x91a%W\x83a\x07\xBFV[3`\0R`\x01` Ra%\x81\x81`@`\0 \x90`\x01`\x01`\xA0\x1B\x03\x16`\0R` R`@`\0 \x90V[\x92\x15\x15\x92\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x81T\x16`\xFF\x85\x16\x17\x90U`@Q\x92\x83R\x16\x90\x7F\x170~\xAB9\xABa\x07\xE8\x89\x98E\xAD=Y\xBD\x96S\xF2\0\xF2 \x92\x04\x89\xCA+Y7il1` 3\x92\xA3\0[4a\x046W` `\x03\x196\x01\x12a\x041Wb\xFF\xFF\xFFa%\xFDa\x08\xF9V[\x16`\0R`\x08` R` c\xFF\xFF\xFF\xFF`@`\0 T\x16`@Q\x90\x81R\xF3[` \x80\x82\x01\x90\x80\x83R\x83Q\x80\x92R`@\x83\x01\x92\x81`@\x84`\x05\x1B\x83\x01\x01\x95\x01\x93`\0\x91[\x84\x83\x10a&PWPPPPPP\x90V[\x90\x91\x92\x93\x94\x95\x84\x80a&\x8C\x83\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x86`\x01\x96\x03\x01\x87R\x8AQa\x05\xC1V[\x98\x01\x93\x01\x93\x01\x91\x94\x93\x92\x90a&@V[` \x80`\x03\x196\x01\x12a\x041W`\x045g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x07\x9EWa&\xC9\x906\x90`\x04\x01a\x0CdV[\x91a&\xD6`\x0FT`\xFF\x16\x90V[a'\xD9Wa&\xE2a@<V[a'\x12`\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0`\x0FT\x16\x17`\x0FUV[a'\x1AaK\xA8V[a'#\x83a=\xB7V[\x92`\0\x90\x81[\x81\x81\x10a'}Wa\x06\xB1\x86a'a\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0`\x0FT\x16`\x0FUV[a'iaQ\xD9V[a'qa@\x9DV[`@Q\x91\x82\x91\x82a&\x1CV[\x82\x80a'\x8A\x83\x85\x89a>\xD4V[\x90a'\x9A`@Q\x80\x93\x81\x93a?\xC6V[\x03\x900Z\xF4a'\xA7a?\xF8V[\x90\x15a'\xD2W\x90a'\xCD\x91a'\xBC\x82\x89a@(V[Ra'\xC7\x81\x88a@(V[Pa>MV[a')V[\x80Q\x90\x85\x01\xFD[`\x04`@Q\x7FU\xE1\xF7\xC5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xFD[4a\x046W`\0`\x03\x196\x01\x12a\x041W` `@Q`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[4a\x046W`\0`\x03\x196\x01\x12a\x041W` `@Q`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[4a\x046W`\0`\x03\x196\x01\x12a\x041W` `\rT`@Q\x90\x81R\xF3[4a\x046W` `\x03\x196\x01\x12a\x041W`\x01`\x01`\xA0\x1B\x03`\x045a(\xCE\x81a\x03\xCFV[\x16`\0R`\x02` R` `@`\0 T`@Q\x90\x81R\xF3[a(\xF06a\x15\xFAV[\x90a(\xF9a@<V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x83\x16\x91\x81\x16\x90\x82\x82\x14a,MWa)=a)4\x85a\x0F\x1E\x84`\x01`\x01`\xA0\x1B\x03\x16`\0R`\x0B` R`@`\0 \x90V[Tb\xFF\xFF\xFF\x16\x90V[\x93b\xFF\xFF\xFF\x94\x85\x81\x16a,\x15WP\x82;\x15a\x07\xA8W`@\x94\x85Q\x92\x7F1<\xE5g\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x92\x83\x85R` \x80\x86`\x04\x81\x8AZ\xFA\x95\x86\x15a\x07\xA3W`\0\x96a+\xF6W[P\x87;\x15a\x07\xA8W\x88Q\x94\x85R\x80\x85`\x04\x81\x8BZ\xFA\x94\x85\x15a\x07\xA3W`\0\x95a+\xC7W[P`\xFF\x86\x16`\x12\x81\x10`\x12\x82\x14\x17\x90`\x06\x80\x82\x11\x91\x14\x17\x16\x15a+\x94Wa)\xF7a#\xE4`\xFF\x87\x16`\x12\x81\x10`\x12\x82\x14\x17\x90`\x06\x80\x82\x11\x91\x14\x17\x16\x90V[a+aW\x92a++\x7F\xC0\xC5\xDF\x98\xA4\xCA\x87\xA3!\xA3;\xF1'|\xF3-1\xA9{l\xE1K\x97G8!I\xB9\xE2c\x1E\xA3\x93a+\x04a\x06\xB1\x9B\x99\x97\x94a*\xF5\x8Aa*\xD1\x9D\x9B\x99a*Ka*F`\x06Tb\xFF\xFF\xFF\x16\x90V[aG\x14V[\x9E\x8Fa*\x81\x81b\xFF\xFF\xFF\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0`\x06T\x16\x17`\x06UV[a*\xA2\x86a\x0F\x1E\x8A`\x01`\x01`\xA0\x1B\x03\x16`\0R`\x0B` R`@`\0 \x90V[\x90b\xFF\xFF\xFF\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\x82T\x16\x17\x90UV[a*\xEBa*\xDCa\x1E\xD3V[`\x01`\x01`\xA0\x1B\x03\x90\x97\x16\x87RV[\x85\x01\x90`\xFF\x16\x90RV[`\x01`\x01`\xA0\x1B\x03\x16\x82\x8B\x01RV[`\xFF\x84\x16``\x82\x01Ra+&\x8Ab\xFF\xFF\xFF\x16`\0R`\t` R`@`\0 \x90V[aG(V[\x86Q`\xFF\x94\x85\x16\x81R\x91\x90\x93\x16` \x82\x01R\x91\x86\x16\x91`@\x90\xA4a+Ma@\x9DV[Qb\xFF\xFF\xFF\x90\x91\x16\x81R\x90\x81\x90` \x82\x01\x90V[\x88Q\x7F\xC3\xDAwG\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\xFF\x86\x16`\x04\x82\x01R`$\x90\xFD[\x88Q\x7F\xC3\xDAwG\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\xFF\x87\x16`\x04\x82\x01R`$\x90\xFD[\x81a+\xE8\x92\x96P=\x87\x11a+\xEFW[a+\xE0\x81\x83a\x1E\x83V[\x81\x01\x90aF\xFBV[\x938a)\xBAV[P=a+\xD6V[\x81a,\x0E\x92\x97P=\x88\x11a+\xEFWa+\xE0\x81\x83a\x1E\x83V[\x948a)\x96V[`@Q\x7F\xB0\x98\x8CC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rb\xFF\xFF\xFF\x91\x90\x91\x16`\x04\x82\x01R`$\x90\xFD[`\x04`@Q\x7F\x05\x13L\xCC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xFD[4a\x046W```\x03\x196\x01\x12a\x041Wa\x06\xB1a-<`\x045a,\x9A\x81a\x07\xADV[b\xFF\xFF\xFF\x81` \x1C\x16`\0R`\t` R`@`\0 a-7a\x11/a-\x1Ca-\x14a\x0C\x12a-\x0C`@Q\x96a,\xCF\x88a\x1E6V[`\xFF``\x82T\x99`\x01\x83`\x01`\x01`\xA0\x1B\x03\x9C\x8D\x81\x16\x84R`\xA0\x1C\x16\x94\x85` \x84\x01R\x01T\x9A\x8B\x16`@\x82\x01R\x01\x98`\xA0\x1C\x16\x88R`$5aZ\xB0V[\x95Q`\xFF\x16\x90V[`D5aZ\xB0V[\x93g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0R`\n` R`@`\0 \x90V[aa\x17V[`@Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R\x90\x81\x90` \x82\x01\x90V[4a\x046W` `\x03\x196\x01\x12a\x041W`\x01`\x01`\xA0\x1B\x03`\x045a-\x84\x81a\x03\xCFV[\x16`\0R`\x07` R` `@`\0 T`@Q\x90\x81R\xF3[4a\x046W`@`\x03\x196\x01\x12a\x041W`\x045a-\xBA\x81a\x03\xCFV[`$5\x90a-\xC6a@<V[`\x01`\x01`\xA0\x1B\x03\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x91\x82;\x15a\x07\xA8W`@Q\x80\x93\x7F\xF7|G\x91\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R\x81`\x04` \x96\x87\x93Z\xFA\x80\x15a\x07\xA3W\x83\x91`\0\x91a/\x8AW[P\x163\x03a\x1D\x8EW`\0\x91\x81\x16\x93\x84;\x15a\x07\xA8W`@Q\x90\x7F1<\xE5g\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R\x84\x82`\x04\x81\x89Z\xFA\x91\x82\x15a\x07\xA3W\x7F\x1F\xDD\0 5\x88\x93U\x97\x13\xDE\xF8\xB4,\xADf\x1F\xFB\xC7U\xD1\xA2dY@'\x92\x14B\xBBV\xA0\x95a//\x95\x93a/kW[PP`\0\x19\x81\x03a/SWPa/\x1F\x90a.\xEC`\xFFa.\xE4\x85`\x01`\x01`\xA0\x1B\x03\x16`\0R`\x07` R`@`\0 \x90V[T\x92\x16aZ\xD7V[\x81\x04\x92[a/\r\x81`\x01`\x01`\xA0\x1B\x03\x16`\0R`\x07` R`@`\0 \x90V[a/\x18\x83\x82TaF\xE1V[\x90UaL\xD1V[`@Q\x90\x81R\x90\x81\x90` \x82\x01\x90V[\x03\x90\xA2`\x0FT`\xFF\x16\x15a/FW[a\0\x1Ba@\x9DV[a/NaQ\xD9V[a/>V[\x91a/c`\xFFa/\x1F\x93\x16aZ\xD7V[\x83\x02\x90a.\xF0V[a/\x82\x92\x93P\x80=\x10a+\xEFWa+\xE0\x81\x83a\x1E\x83V[\x908\x80a.\xB2V[a/\xA1\x91P\x85=\x87\x11a\x1D\xDFWa\x1D\xD1\x81\x83a\x1E\x83V[8a.;V[4a\x046W` `\x03\x196\x01\x12a\x041W`\x045a/\xC4\x81a\x07\xADV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16`\0R`\n` Ra/\xF6a\x080a\x080`\x03`@`\0 \x01`\x01`\x01`\xA0\x1B\x03\x90T\x16\x90V[\x80;\x15a\x07\xA8W`@Q\x7F\xE31\xBA4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16`\x04\x83\x01R` \x90\x82\x90\x81\x80`$\x81\x01a\x08\xA4V[4a\x046W` `\xFFa0\x87`\x01`\x01`\xA0\x1B\x03a0a6a\x15\xFAV[\x91\x16`\0R`\x01\x84R`@`\0 \x90`\x01`\x01`\xA0\x1B\x03\x16`\0R` R`@`\0 \x90V[T\x16`@Q\x90\x15\x15\x81R\xF3[a\x1E\xD1\x90\x92\x91\x92`\xA0\x81\x01\x93`\x80\x80\x91o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82Q\x16\x85R` \x82\x01Q\x16` \x85\x01R`@\x81\x01Q\x15\x15`@\x85\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF``\x82\x01Q\x16``\x85\x01R\x01Q\x15\x15\x91\x01RV[4a\x046W```\x03\x196\x01\x12a\x041W`\x80`\x045a1\r\x81a\x07\xADV[`$5\x90a1\x1A\x82a\x07\xBFV[`D5a1&\x81a\x03\xCFV[`@Qa12\x81a\x1E\x15V[`\0\x94\x81\x86\x80\x93R\x82` \x82\x01R\x82`@\x82\x01R\x82``\x82\x01R\x01Ra1wa\x080a\x080`\x03a\x10\r\x86g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0R`\n` R`@`\0 \x90V[\x80;\x15a\x07\xA8W`@Q\x7F\xF0{\x87\x9E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93\x90\x93\x16`\x04\x84\x01R\x92\x15\x15`$\x83\x01R`\x01`\x01`\xA0\x1B\x03\x16`D\x82\x01R\x90`\xA0\x90\x82\x90`d\x90\x82\x90Z\xFA\x90\x81\x15a\x07\xA3Wa\x06\xB1\x92\x91a1\xF7W[P`@Q\x91\x82\x91\x82a0\x93V[a2\x18\x91P`\xA0=\x81\x11a2\x1EW[a2\x10\x81\x83a\x1E\x83V[\x81\x01\x90aTtV[8a1\xEAV[P=a2\x06V[4a\x046W`\xA0`\x03\x196\x01\x12a\x041W`\x045a2B\x81a\x03\xCFV[`$5\x90a2O\x82a\x03\xCFV[`D5`d5`\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x07\x9EWa2u\x906\x90`\x04\x01a\nkV[\x94\x90\x91`\x01`\x01`\xA0\x1B\x03\x91\x82\x86\x16\x92\x833\x14\x80\x15a3\x99W[a2\x98\x90agwV[a2\xB9\x86a\x0E\xA7\x89`\x01`\x01`\xA0\x1B\x03\x16`\0R`\0` R`@`\0 \x90V[a2\xC4\x84\x82TaF\xE1V[\x90Ua2\xE7\x86a\x0E\xA7\x84`\x01`\x01`\xA0\x1B\x03\x16`\0R`\0` R`@`\0 \x90V[a2\xF2\x84\x82TaF\xEEV[\x90U\x81\x16\x80\x93`@Q\x7F\xC3\xD5\x81h\xC5\xAEs\x97s\x1D\x06=[\xBF=exTBsC\xF4\xC0\x83$\x0Fz\xAC\xAA-\x0Fb3\x91\x80a37\x88\x8C\x83` \x90\x93\x92\x91\x93`@\x81\x01\x94\x81R\x01RV[\x03\x90\xA4;\x15a3\x88W\x81;\x15a\x07\xA8W`\0` \x94a\r\xE3`@Q\x98\x89\x96\x87\x95\x86\x94\x7F\xF2:na\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x9B\x8C\x87R3`\x04\x88\x01ah\"V[P\x92PPPa\0\x1B\x91P\x15\x15ag\xC2V[Pa2\x98a3\xC1a\x0F63a\x0F\x1E\x8B`\x01`\x01`\xA0\x1B\x03\x16`\0R`\x01` R`@`\0 \x90V[\x90Pa2\x8FV[`\xA0`\x03\x196\x01\x12a\x041Wa3\xDCa@<V[`\x0FT`\xFF\x16\x15a<\x0FW[a3\xF0aF\x19V[a3\xF8aF%V[\x90a4\x19a4\x04aF1V[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x93\x16\x91a4Ta4:aF\x19V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0R`\n` R`@`\0 \x90V[a4ra#\xE4a4c\x83aAOV[``\x01Qc\xFF\xFF\xFF\xFF\x16\x15\x15\x90V[a;\xD4Wa4\x87a4\x82\x82aAOV[a`\xE4V[a;\xAAWa4\xB4a\x080a\x080`\x03a\x10\r\x87g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0R`\n` R`@`\0 \x90V[\x94\x85;\x15a\x07\xA8W`@\x91\x82Q\x7F\xE6\x04{\x19\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x97`\x04\x91\x89\x81\x80a5\n\x8B\x87\x83\x01\x91\x90\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x82\x01\x93\x16\x90RV[\x03\x81\x85Z\xFA\x90\x81\x15a\x07\xA3W`\0\x91a;\x8DW[P\x15a;NWa5.B\x84a^\xE3V[a56aF=V[\x81;\x15a\x07\xA8W\x85Q\x7F\xECshT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x89\x16\x84\x82\x01\x90\x81R\x91\x15\x15` \x83\x01R3`@\x83\x01R\x90\x86\x90\x82\x90\x81\x90``\x01\x03\x81`\0\x86Z\xF1\x90\x81\x15a\x07\xA3W`\0\x90\x81\x92a;.W[P\x15a;\x06Wa5\xCCa\x0B\xFAb\xFF\xFF\xFF\x8A\x8D\x1C\x16b\xFF\xFF\xFF\x16`\0R`\t` R`@`\0 \x90V[\x90\x8A\x87a5\xD7aFtV[\x93a5\xE0aF=V[\x15a:\x8FW\x90\x81a6\x08a5\xFDa6f\x95a6V\x95\x01Q`\xFF\x16\x90V[`\xFF\x16a\x01\0\x88\x01RV[a6$a6\x19``\x83\x01Q`\xFF\x16\x90V[`\xFF\x16a\x01 \x88\x01RV[a6Ha68\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16`\xC0\x88\x01RV[\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16`\xE0\x84\x01RV[a6naFIV[a:NW[\x90a6\x8B\x91\x81R\x89`\x80\x82\x01R\x87`\xA0\x82\x01Raf\x9CV[\x92`\xA0\x84\x01\x90\x81Q\x15a:&W`\x80\x85\x01\x93\x84Q\x15a9\xFFWa7F\x8Ca6\xFBa6\xB46a\x1E\xE0V[\x91a6\xE5a6\xC2\x8AQal<V[\x91a6\xCD\x89Qal<V[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x85\x01RV[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82RV[`\x01`\x01`\xA0\x1B\x03\x983\x8Aa7\x1A`\x02\x88\x01T`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x16\x03a9\xEDW`\x01\x85\x01T`\xB0\x1Ca\xFF\xFF\x16\x91[\x85T\x90a\xFF\xFF`\rT\x94\x16\x92\x82`\x80\x1C\x92\x16\x90ad\xABV[\x92``\x8A\x98\x93\x98\x01\x97\x88R\x8B\x8A\x01R\x88Q\x91\x80;\x15a\x07\xA8W\x8B\x92\x8Ea7\xBA\x85Q\x96\x87\x95\x86\x94\x85\x94\x7F\xA4G\x89\x19\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86R\x8B\x86\x01\x90\x94\x93\x92``\x92g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x84\x01\x97\x16\x83R` \x83\x01R`@\x82\x01R\x01RV[\x03\x91Z\xFA\x90\x81\x15a\x07\xA3W\x8D`\0\x91\x82\x93a9\xBCW[P\x82\x90\x89\x01R\x15a9~WPP\x93\x88\x99\x9A\x93\x7F\xCD\x80T&o\xAE;\xBD#\0\xC8\x9A@t\xC4\x11=\x12\xB9\xA5\x83\xF7\xCD#\x90\xD40\xAFp\xB1*X\x93a8ha8w\x94a81a\x06\xB1\x9D\x99a8\x1BaF=V[a8(\x87Q\x87Q\x90aF\xE1V[\x90\x84Q\x92a_\xBEV[a8R`\xC0\x86\x01\x94a8J\x86Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x90Q\x90aLeV[`\xE0\x85\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x90Q\x90aL\xD1V[Q\x90\x81a9EW[PPag\x0CV[\x93a8\x80aF=V[\x92a8\xF3\x8Ba8\x99`\xC0\x89\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x97\x8Aa8\xAF`\xE0\x83\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x82\x8C\x01Q\x92\x90\x95\x01Q\x8BQ\x98\x15\x15\x89R` \x89\x01\x93\x90\x93R`@\x88\x01R``\x87\x01R`\x80\x86\x01R\x90\x82\x16\x95\x90\x91\x16\x93g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x92\x90\x81\x90`\xA0\x82\x01\x90V[\x03\x90\xA4a9\x05a\x12\xBC`\x0FT`\xFF\x16\x90V[\x15a98W[a9\x13a@\x9DV[Q\x93\x84\x93\x84`@\x91\x94\x93\x92g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF``\x83\x01\x96\x16\x82R` \x82\x01R\x01RV[a9@aQ\xD9V[a9\x0BV[a\x0E\xE0a9\\a9u\x92Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16`\0R`\x07` R`@`\0 \x90V[\x90U8\x80a8pV[\x86Q\x89Q\x7F\xB9=\xEE\n\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x92\x83\x01\x90\x81R` \x81\x01\x91\x90\x91R\x81\x90`@\x01\x03\x90\xFD[\x90\x92Pa9\xDF\x91P\x8A=\x8C\x11a9\xE6W[a9\xD7\x81\x83a\x1E\x83V[\x81\x01\x90aFUV[\x918a7\xD0V[P=a9\xCDV[`\x01\x85\x01T`\xA0\x1Ca\xFF\xFF\x16\x91a7.V[\x87Q\x7F\x13\xFD\x9Bm\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xFD[\x83\x87Q\x7Fo\x85\xB3N\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xFD[a:ea\x13\xB5`\xC0\x84\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\0\x81\x13a:tW[Pa6sV[a6\x8B\x92\x91\x9APa4\x04a:\x87\x91al<V[\x99\x90\x91a:nV[a6V\x91Pa:\xCEa:\xC3a;\x01\x94a:\xBAa:\xAF``\x86\x01Q`\xFF\x16\x90V[`\xFF\x16a\x01\0\x8A\x01RV[\x83\x01Q`\xFF\x16\x90V[`\xFF\x16a\x01 \x87\x01RV[a:\xF4a:\xE4\x8B\x83\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16`\xC0\x87\x01RV[Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[a6fV[\x82\x86Q\x7F.\xD0\xEA\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xFD[\x90Pa;G\x91P\x86=\x88\x11a9\xE6Wa9\xD7\x81\x83a\x1E\x83V[\x908a5\xA3V[\x84Q\x7F\xBC'\xA5\x17\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x88\x16\x81\x84\x01\x90\x81R\x81\x90` \x01\x03\x90\xFD[a;\xA4\x91P\x8A=\x8C\x11a\x14\x96Wa\x14\x88\x81\x83a\x1E\x83V[8a5\x1EV[`\x04`@Q\x7Fz\x95\xCB!\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xFD[`@Q\x7F\x12\x16\xE0\xFA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R`$\x90\xFD[a<\x17aK\xA8V[a3\xE8V[4a\x046W`\0`\x03\x196\x01\x12a\x041W` `\0Rk\x0Bv1.5.0-beta`+R```\0\xF3[`\x84`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`)`$\x82\x01R\x7FUnknown signature and no fallbac`D\x82\x01R\x7Fk defined\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R\xFD[`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x163\x03a\x03\xE0WV[`\x84`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x1E1W`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16` \x01\x90V[`@Q=`\0\x82>=\x90\xFD[a\x06\x15\x900\x90aY\xD5V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x1E1W`\x05\x1B` \x01\x90V[\x90a=\xC1\x82a=\x9FV[a=\xCE`@Q\x91\x82a\x1E\x83V[\x82\x81R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0a=\xFC\x82\x94a=\x9FV[\x01\x90`\0[\x82\x81\x10a>\rWPPPV[\x80``` \x80\x93\x85\x01\x01R\x01a>\x01V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0\x19\x81\x14a>\\W`\x01\x01\x90V[a>\x1EV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[`d`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FCalldata tail too short\0\0\0\0\0\0\0\0\0`D\x82\x01R\xFD[\x91\x90\x81\x10\x15a?\xC1W`\x05\x1B\x81\x015\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x816\x03\x01\x82\x12\x15a?}W\x01\x90\x815\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a?9W` \x01\x826\x03\x81\x13a?4W\x91\x90V[a>\x90V[`d`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FInvalid calldata tail length\0\0\0\0`D\x82\x01R\xFD[`d`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FInvalid calldata tail offset\0\0\0\0`D\x82\x01R\xFD[a>aV[\x90\x80\x92\x91\x827\x01`\0\x81R\x90V[`@Q\x90` \x82\x01\x82\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x1E1W`@R`\0\x82RV[=\x15a@#W=\x90a@\t\x82a=NV[\x91a@\x17`@Q\x93\x84a\x1E\x83V[\x82R=`\0` \x84\x01>V[``\x90V[\x80Q\x82\x10\x15a?\xC1W` \x91`\x05\x1B\x01\x01\x90V[`\x0CT`\x01\x81\x14\x15\x80a@\x86W[a@\\Wa@W\x90a>MV[`\x0CUV[`\x04`@Q\x7F\x02\xB8\0-\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xFD[P`\xFF`\x0FT\x16\x15\x80a@JWP`\x02\x81\x11a@JV[`\x0CT\x80\x15a>\\W`\0\x19\x01`\x0CU`\xFF`\x05T\x16\x15\x80a@\xEAW[a@\xC0WV[`\x04`@Q\x7F2n\xFAC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xFD[P`\xFF`\x0FT\x16\x15a@\xBAV[\x90\x81` \x91\x03\x12a\x041WQa\x06\x15\x81a\x07\xBFV[\x90`@QaA\x19\x81a\x1E6V[```\xFF\x82\x94`\x01\x81T\x91\x83`\x01`\x01`\xA0\x1B\x03\x93\x84\x81\x16\x88R`\xA0\x1C\x16` \x87\x01R\x01T\x90\x81\x16`@\x85\x01R`\xA0\x1C\x16\x91\x01RV[\x90a\x1E\xD1`@QaA_\x81a\x1ERV[`\xE0aB\x18`\x03\x83\x96aB\x01\x81TaA\xAAo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91aA\xA0\x83\x82\x16\x8A\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90RV[`\x80\x1C` \x89\x01RV[aA\xD0`\x01\x84\x01T\x91\x82\x16`@\x89\x01\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90RV[`\x80\x81\x90\x1Cc\xFF\xFF\xFF\xFF\x16``\x88\x01Ra\xFF\xFF`\xA0\x82\x90\x1C\x81\x16`\x80\x89\x01R\x90`\xB0\x1C\x16`\xA0\x87\x01\x90a\xFF\xFF\x16\x90RV[a\x10\ra:\xE4`\x02\x83\x01T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x91\x01RV[`\x0F\x0B\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x14a>\\W`\0\x03\x90V[`\x0F\x0B\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC4e6\0\x01\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x12o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x13\x17a>\\WV[\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x14a>\\W`\0\x03\x90V[\x91\x90\x91o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x80\x94\x16\x91\x16\x01\x91\x82\x11a>\\WV[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x82\x16\x90\x82\x16\x03\x91\x90\x82\x11a>\\WV[`\x80\x81\x01\x90aCFa4:\x83Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x90aCT` \x82\x01Qal<V[\x90aCb`@\x82\x01Qal<V[\x93``\x82\x01\x91aCs\x83Q`\x0F\x0B\x90V[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x83aC\xA3`\x01\x89\x01To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x16\x15aE\xBFW[a4:aD\x14\x91aD\x1F\x93`\x0F\x0B`\0\x81\x13`\0\x14aEvWaD\x06\x90aC\xDB`\xA0\x88\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFaC\xF7\x85Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x16aD\0a?\xD4V[\x92ajOV[Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x84Q`\x0F\x0B\x90a_8V[`\0aD]aDWaDO`\xE0aD@`\xC0\x87\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x95\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x95Q`\x0F\x0B\x90V[`\x0F\x0B\x90V[\x12\x15aE+W\x83aD\x8Ba\x1E\xD1\x97\x94\x84aD\x83aD\xA4\x95aD\xE1\x97aD\xEB\x9A\x16\x90aL\xD1V[\x86\x16\x90aL\xD1V[\x85To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aC\nV[\x84\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82T\x16\x17\x90UV[\x82T`\x80\x1CaC\nV[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83T\x92`\x80\x1B\x16\x91\x16\x17\x90UV[\x83aESa\x1E\xD1\x97\x94\x84aEKaD\xA4\x95aEl\x97aD\xEB\x9A\x16\x90aLeV[\x86\x16\x90aLeV[\x85To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aB\xE6V[\x82T`\x80\x1CaB\xE6V[aE\xBA\x90aE\x8E`\xA0\x88\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFaE\xB2aE\xAC\x86Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x93aB\xB9V[\x92\x16\x90akmV[aD\x06V[`\0\x87Uc;\x9A\xCA\0`\x0F\x83\x90\x0B\x12aE\xEFWa4:aD\x14\x91aE\xE5aD\x1F\x94aBVV[\x93P\x91PPaC\xAAV[`\x04`@Q\x7F\xCBm\xABu\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xFD[`d5a\x06\x15\x81a\x07\xADV[`\x045a\x06\x15\x81a\x0FDV[`$5a\x06\x15\x81a\x0FDV[`\x845a\x06\x15\x81a\x07\xBFV[`D5a\x06\x15\x81a\x07\xBFV[\x91\x90\x82`@\x91\x03\x12a\x041W` \x82QaFn\x81a\x07\xBFV[\x92\x01Q\x90V[`@Q\x90a\x01@\x82\x01\x82\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x1E1W`@R\x81a\x01 `\0\x91\x82\x81R\x82` \x82\x01R\x82`@\x82\x01R\x82``\x82\x01R\x82`\x80\x82\x01R\x82`\xA0\x82\x01R\x82`\xC0\x82\x01R\x82`\xE0\x82\x01R\x82a\x01\0\x82\x01R\x01RV[\x90`\0\x19\x82\x01\x91\x82\x11a>\\WV[\x91\x90\x82\x03\x91\x82\x11a>\\WV[\x91\x90\x82\x01\x80\x92\x11a>\\WV[\x90\x81` \x91\x03\x12a\x041WQ`\xFF\x81\x16\x81\x03a\x03\xE0W\x90V[b\xFF\xFF\xFF\x80\x91\x16\x90\x81\x14a>\\W`\x01\x01\x90V[`\xFF``a\x1E\xD1\x93aG\xFE`\x01`\x01`\x01`\xA0\x1B\x03\x95aGy\x87\x85Q\x16\x82\x90`\x01`\x01`\xA0\x1B\x03\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82T\x16\x17\x90UV[` \x84\x01Q\x81T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x87\x16`\xA0\x1Bt\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x17\x81U\x01\x94`@\x83\x01Q\x16\x85\x90`\x01`\x01`\xA0\x1B\x03\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82T\x16\x17\x90UV[\x01Q\x82T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91\x16`\xA0\x1Bt\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x17\x90UV[c\xFF\xFF\xFF\xFF\x80\x91\x16\x90\x81\x14a>\\W`\x01\x01\x90V[`\x1F\x82` \x94\x93\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x93\x81\x86R\x86\x86\x017`\0\x85\x82\x86\x01\x01R\x01\x16\x01\x01\x90V[`@\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x06\x15\x95\x93\x16\x81R\x81` \x82\x01R\x01\x91aHZV[\x96\x98\x97\x92\x90\x94\x95aH\xC9a@<V[b\xFF\xFF\xFF\x97\x80\x89\x16aK\xA2WP`\x06Tb\xFF\xFF\xFF\x16\x97[\x88\x16\x15aKxWaI\0\x88b\xFF\xFF\xFF\x16`\0R`\x08` R`@`\0 \x90V[\x93\x84TaI\x10\x90c\xFF\xFF\xFF\xFF\x16\x90V[aI\x19\x90aHEV[\x85T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\x16c\xFF\xFF\xFF\xFF\x82\x16\x17\x90\x95U`\x01`\x01`\xA0\x1B\x03\x94aI`\x90\x8A\x8A\x88\x16\x15\x15a[\x83V[\x9AaI\x7F\x8Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0R`\n` R`@`\0 \x90V[\x83\x8Aa\xFF\xFF\x8B\x89\x82\x8D\x16\x92\x8A\x16\x91aI\x96\x96a[\xB6V[aI\xD5\x8C\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\xFFh\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0`\x0FT\x92`\x08\x1B\x16\x91\x16\x17`\x0FUV[aI\xF3\x8Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0R`\n` R`@`\0 \x90V[`\x03\x01T`\x01`\x01`\xA0\x1B\x03\x16\x91\x82;\x15a\x07\xA8WaJH\x92` \x92`\0\x8F`@Q\x96\x87\x95\x86\x94\x85\x93\x7F\xE0hx\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85R`\x04\x85\x01aH\x99V[\x03\x92Z\xF1\x90\x81\x15a\x07\xA3W`\0\x91aKZW[P\x15aK0W\x89\x96\x7Fa<|I?\x9A\xA3\x97M\xCD\xBB\x87\xE6\xFE\xA5L\xCD\xB9\x10>\xA7\x83.m\xF0\x8D\xB8\x1Co\x90\xD2\x9D\x95aK%\x93aJ\xD0`\x01a\x10\raJ\xBAaJ\xAD\x8Fb\xFF\xFF\xFF\x16`\0R`\t` R`@`\0 \x90V[T`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x9Db\xFF\xFF\xFF\x16`\0R`\t` R`@`\0 \x90V[\x95g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x88`@Q\x99\x8A\x99\x16\x9D\x16\x9B\x16\x99\x87\x94\x92\x90\x96\x95\x91\x93`\xA0\x94`\xC0\x87\x01\x98\x87R` \x87\x01Ra\xFF\xFF\x80\x92\x16`@\x87\x01R\x16``\x85\x01R`\x01`\x01`\xA0\x1B\x03\x80\x92\x16`\x80\x85\x01R\x16\x91\x01RV[\x03\x90\xA4a\x1E\xD1a@\x9DV[`\x04`@Q\x7F\xDF\x15\x8AX\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xFD[aKr\x91P` =\x81\x11a\x14\x96Wa\x14\x88\x81\x83a\x1E\x83V[8aJ[V[`\x04`@Q\x7F\xCCzs\x9B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xFD[\x97aH\xE0V[4aK\xAFWV[`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0aK\xE1\x81aXZV[\x16\x80;\x15a\x07\xA8W`\0`\x04\x91`@Q\x92\x83\x80\x92\x7F\xD0\xE3\r\xB0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R4\x90Z\xF1\x80\x15a\x07\xA3WaLVW[P`@Q4\x81R\x7F\xE1\xFF\xFC\xC4\x92=\x04\xB5Y\xF4\xD2\x9A\x8B\xFCl\xDA\x04\xEB[\r<F\x07Q\xC2@,\\\\\xC9\x10\x9C` 3\x92\xA2V[aL_\x90a\x1EoV[8aL'V[aLn\x81aXZV[aL\x8B\x81`\x01`\x01`\xA0\x1B\x03\x16`\0R`\x02` R`@`\0 \x90V[\x80T\x90\x83\x82\x01\x80\x92\x11a>\\WU`@Q\x91\x82R`\x01`\x01`\xA0\x1B\x03\x16\x90\x7F\x80\xB2\x17H\xC7\x87\xC5.\x87\xA6\xB2\"\x01\x1E\n\x0E\xD0\xF9\xCC \x15\xF0\xCE\xD4gHd-\xC6.\xE9\xF8\x90` \x90\xA2V[aL\xEE\x81`\x01`\x01`\xA0\x1B\x03\x16`\0R`\x02` R`@`\0 \x90V[T\x80\x83\x11aMcWPaM\0\x81aXZV[aM\x1D\x81`\x01`\x01`\xA0\x1B\x03\x16`\0R`\x02` R`@`\0 \x90V[\x80T\x90\x83\x82\x03\x91\x82\x11a>\\WU`@Q\x91\x82R`\x01`\x01`\xA0\x1B\x03\x16\x90\x7F\x1Cq\x1E\xCA\x8D\x0BiK\xBC\xB0\xA1Db\xA7\0b\"\xE7!\x95K,_\xF7\x98\xF6\x06\x81~\xB1\x102\x90` \x90\xA2V[`@Q\x7F1Rv\xC9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x91\x90\x91R`$\x81\x01\x92\x90\x92RP`D\x90\xFD[b\xFF\xFF\xFF\x90\x93\x91\x92\x93` \x1C\x16`\0R`\t` R`@`\0 \x92`@QaM\xC6\x81a\x1E6V[`\xFF``\x86T\x92`\x01\x83`\x01`\x01`\xA0\x1B\x03\x95\x86\x81\x16\x84R`\xA0\x1C\x16\x98\x89` \x84\x01R\x01T\x93\x84\x16`@\x82\x01R\x01\x91`\xA0\x1C\x16\x81RaN\x04\x84al<V[\x94o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x94\x85\x81\x03aNSW[PPaN*\x82al<V[\x93\x82\x03aN5WPPV[a\x06\x15\x92\x93P\x90aNMa\x0C\x12a!\x9D\x93Q`\xFF\x16\x90V[\x90aZ\xB0V[aNc\x92\x96P\x90a!\x9D\x91aZ\xB0V[\x938\x80aN\x1FV[`@Q\x90`\x04T\x80\x83R\x82` \x91\x82\x82\x01\x90`\x04`\0R\x7F\x8A5\xAC\xFB\xC1_\xF8\x1A9\xAE}4O\xD7\t\xF2\x8E\x86\0\xB4\xAA\x8Ce\xC6\xB6K\xFE\x7F\xE3k\xD1\x9B\x93`\0\x90[\x82\x82\x10aN\xBEWPPPa\x1E\xD1\x92P\x03\x83a\x1E\x83V[\x85T`\x01`\x01`\xA0\x1B\x03\x16\x84R`\x01\x95\x86\x01\x95\x88\x95P\x93\x81\x01\x93\x90\x91\x01\x90aN\xA8V[`\x0ET\x90h\x01\0\0\0\0\0\0\0\0\x82\x10\x15a\x1E1W`\x01\x82\x01\x80`\x0EU\x82\x10\x15a?\xC1W\x7F\xBB{JEM\xC3I9#H/\x07\x82#)\xED\x19\xE8$N\xFFX,\xC2\x04\xF8UL6 \xC4\0`\x01`\x01`\xA0\x1B\x03``a\x1E\xD1\x94`\x0E`\0R`\x02\x1B\x93\x80Q\x85\x7F\xBB{JEM\xC3I9#H/\x07\x82#)\xED\x19\xE8$N\xFFX,\xC2\x04\xF8UL6 \xC3\xFD\x01U` \x81\x01Q\x7F\xBB{JEM\xC3I9#H/\x07\x82#)\xED\x19\xE8$N\xFFX,\xC2\x04\xF8UL6 \xC3\xFE\x86\x01U`@\x81\x01Q\x7F\xBB{JEM\xC3I9#H/\x07\x82#)\xED\x19\xE8$N\xFFX,\xC2\x04\xF8UL6 \xC3\xFF\x86\x01U\x01Q\x16\x91\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82T\x16\x17\x90UV[`\x04T\x80\x15aPWW`\0\x19\x81\x01\x90\x80\x82\x10\x15a?\xC1W\x7F\x8A5\xAC\xFB\xC1_\xF8\x1A9\xAE}4O\xD7\t\xF2\x8E\x86\0\xB4\xAA\x8Ce\xC6\xB6K\xFE\x7F\xE3k\xD1\x9A\x90`\x04`\0R\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81T\x16\x90U`\x04UV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`1`\x04R`$`\0\xFD[`\x0ET\x90aP\x93\x82a=\x9FV[\x91`@aP\xA2\x81Q\x94\x85a\x1E\x83V[\x81\x84R\x83` \x80\x91\x01\x91`\x0E`\0R\x7F\xBB{JEM\xC3I9#H/\x07\x82#)\xED\x19\xE8$N\xFFX,\xC2\x04\xF8UL6 \xC3\xFD\x90`\0\x93[\x85\x85\x10aP\xE6WPPPPPPV[`\x04\x84`\x01\x92\x84QaP\xF7\x81a\x1E6V[\x86T\x81R\x84\x87\x01T\x83\x82\x01R`\x02\x87\x01T\x86\x82\x01R`\x01`\x01`\xA0\x1B\x03`\x03\x88\x01T\x16``\x82\x01R\x81R\x01\x93\x01\x94\x01\x93\x91aP\xD7V[\x81\x81\x03\x92\x91`\0\x13\x80\x15\x82\x85\x13\x16\x91\x84\x12\x16\x17a>\\WV[`\x0ET`\0\x80`\x0EU\x81aQXWPPV[\x7F?\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a>\\W`\x0E\x81R`\x02\x7F\xBB{JEM\xC3I9#H/\x07\x82#)\xED\x19\xE8$N\xFFX,\xC2\x04\xF8UL6 \xC3\xFD\x92\x81\x1B\x83\x01\x92[\x83\x81\x10aQ\xBCWPPPPV[\x80\x83`\x04\x92U\x83`\x01\x82\x01U\x83\x83\x82\x01U\x83`\x03\x82\x01U\x01aQ\xAFV[aQ\xE1aNkV[\x80Q\x80\x15aTFW`\x01\x90\x81\x80[aS\xA0W[PPPPaR\0aP\x86V[\x80Q\x80[aR\x1AWPPaR\x12aYmV[a\x1E\xD1aQFV[aR#\x81aF\xD2V[\x90aRB``aR3\x84\x86a@(V[Q\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[aRL\x83\x85a@(V[QQ` aRZ\x85\x87a@(V[Q\x01Q\x81\x15aSHWPaR\xCBaRw\x93\x94`@\x94\x85\x91\x88a@(V[Q\x01Q\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x90\x83\x16\x03aS7WaR\xBB\x913\x90aV>V[aR\xC50\x85aU}V[\x92aF\xE1V[\x90\x81\x81\x10aR\xE3WPPPP`\0\x19\x90[\x01\x80aR\x04V[a\x14l\x91aR\xF0\x91aQ-V[\x92Q\x92\x83\x92\x7F\xAA&\x9D\xE2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01` \x90\x93\x92\x91\x93`\x01`\x01`\xA0\x1B\x03`@\x82\x01\x95\x16\x81R\x01RV[aSC\x91P3\x86ad'V[aR\xBBV[\x80\x91PaS\\W[PP`\0\x19\x91PaR\xDCV[aS\x8FaSo\x93\x94`@\x94\x85\x91\x88a@(V[Q\x01Q\x91aS\x7F\x8103\x87ac\x9EV[aS\x890\x85aU}V[\x92aF\xEEV[\x90\x81\x81\x10aR\xE3WP\x83\x92PaSPV[\x15aT;W[`\0\x19`\0\x91aS\xC1a:\xF4aS\xBB\x83aF\xD2V[\x87a@(V[aS\xCB0\x82aV\xDFV[\x81\x15\x80\x15\x90aT2W[aS\xEDW[PPPaS\xE5aO\xEDV[\x01\x90\x82aQ\xEFV[aT*\x92aT%\x91aS\xFF0\x83aU}V[\x90aT\x08a\x1E\xD3V[\x94\x85R` \x85\x01R`@\x84\x01R`\x01`\x01`\xA0\x1B\x03\x16``\x83\x01RV[aN\xE1V[8\x80\x80aS\xDAV[P\x80\x15\x15aS\xD5V[\x80aS\xA6W\x80aQ\xF4V[PPa\x1E\xD1aYmV[\x90\x81` \x91\x03\x12a\x041WQa\x06\x15\x81a\x03\xCFV[\x90\x81` \x91\x03\x12a\x041WQ\x90V[\x90\x81`\xA0\x91\x03\x12a\x041W`\x80`@Q\x91aT\x8E\x83a\x1E\x15V[\x80QaT\x99\x81a\x0FDV[\x83R` \x81\x01QaT\xA9\x81a\x0FDV[` \x84\x01R`@\x81\x01QaT\xBC\x81a\x07\xBFV[`@\x84\x01R``\x81\x01QaT\xCF\x81a\x07\xADV[``\x84\x01R\x01QaT\xDF\x81a\x07\xBFV[`\x80\x82\x01R\x90V[\x90\x81``\x91\x03\x12a\x041W\x80QaT\xFD\x81a\x07\xBFV[\x91`@` \x83\x01Q\x92\x01Q\x90V[\x91`\xC0\x91\x94\x93`\x01`\x01`\xA0\x1B\x03\x91aUr\x85`\xE0\x81\x01\x98`\x80\x80\x91o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82Q\x16\x85R` \x82\x01Q\x16` \x85\x01R`@\x81\x01Q\x15\x15`@\x85\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF``\x82\x01Q\x16``\x85\x01R\x01Q\x15\x15\x91\x01RV[`\xA0\x85\x01R\x16\x91\x01RV[\x90`@Q\x91`\x01`\x01`\xA0\x1B\x03` \x84\x01\x92\x7Fp\xA0\x821\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R\x16`$\x84\x01R`$\x83R``\x83\x01\x91\x83\x83\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x11\x17a\x1E1W`\0\x93\x84\x93`@RQ\x91Z\xFAaU\xE8a?\xF8V[\x90\x15\x80\x15aV2W[aV\x08W\x80` \x80a\x06\x15\x93Q\x83\x01\x01\x91\x01aTeV[`\x04`@Q\x7F\xC5.>\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xFD[P` \x81Q\x14\x15aU\xF1V[`\x01`\x01`\xA0\x1B\x03\x16\x90\x81;\x15a\x07\xA8W`@Q\x80\x92\x7F.\x1A}M\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R\x81`$`\0\x95\x86\x80\x94\x89`\x04\x84\x01RZ\xF1\x80\x15a\x07\xA3WaV\xCCW[P\x81\x80\x93\x81\x92Z\xF1\x15aV\xA2WV[`\x04`@Q\x7F5e\x94\xAB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xFD[aV\xD8\x90\x92\x91\x92a\x1EoV[\x908aV\x93V[\x91`\0\x80\x81\x93aW\x02\x86`\x01`\x01`\xA0\x1B\x03\x16`\0R`\x02` R`@`\0 \x90V[T`\x01`\x01`\xA0\x1B\x03\x87\x16\x90\x81;\x15a\x07\xA8W` `\x04\x92`@Q\x93\x84\x80\x92\x7F1<\xE5g\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x90\x81\x15a\x07\xA3W`\xFFaWf\x92aWm\x94\x88\x91aX<W[P\x16\x90a[>V[\x91\x87aU}V[\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x83\x11aX8W\x81\x11aX4W\x90aW\xA5\x91aQ-V[\x91\x80\x83\x13\x15aW\xF6WPPaW\xCE\x90\x93`\x01`\x01`\xA0\x1B\x03\x16`\0R`\x03` R`@`\0 \x90V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x81T\x16\x90UV[\x82\x91\x95\x92\x12aX\x1EW[PaW\xCE\x90`\x01`\x01`\xA0\x1B\x03\x16`\0R`\x03` R`@`\0 \x90V[aW\xCE\x91\x93PaX-\x90aB\xB9V[\x92\x90aX\0V[\x83\x80\xFD[\x84\x80\xFD[aXT\x91P` =\x81\x11a+\xEFWa+\xE0\x81\x83a\x1E\x83V[8aW^V[`\x05T`\xFF\x81\x16aYBW[P`\x01`\x01`\xA0\x1B\x03\x81\x16\x90\x81`\0R`\x03` R`\xFF`@`\0 T\x16\x15aX\x8DWPPV[`\x04T\x90h\x01\0\0\0\0\0\0\0\0\x82\x10\x15a\x1E1W`\x01\x82\x01\x80`\x04U\x82\x10\x15a?\xC1WaY\n\x91\x7F\x8A5\xAC\xFB\xC1_\xF8\x1A9\xAE}4O\xD7\t\xF2\x8E\x86\0\xB4\xAA\x8Ce\xC6\xB6K\xFE\x7F\xE3k\xD1\x9B\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82T\x16\x17\x90UV[`\0R`\x03` R`@`\0 `\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x82T\x16\x17\x90UV[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x05U8aXfV[`\x04TaY\xA6W`\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0`\x05T\x16\x17`\x05U`\0`\x04UV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x01`\x04R`$`\0\xFD[\x90aY\xF3\x82`\x01`\x01`\xA0\x1B\x03\x16`\0R`\x02` R`@`\0 \x90V[T\x90`\x01`\x01`\xA0\x1B\x03\x83\x16\x91\x82;\x15a\x07\xA8W` `\x04\x93`@Q\x94\x85\x80\x92\x7F1<\xE5g\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x90\x81\x15a\x07\xA3W`\xFFaZX\x92aZ^\x95`\0\x91aX<WP\x16\x90a[>V[\x92aU}V[\x90\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11a\x03\xE0W\x82\x11a\x03\xE0Wa\x06\x15\x91aQ-V[o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x03\xE0W\x90V[\x90`\x12\x03`\x12\x81\x11a>\\WaZ\xC5\x90aZ\xC9V[\x02\x90V[`M\x81\x11a>\\W`\n\n\x90V[`\x12\x03`\x12\x81\x11a>\\Wa\x06\x15\x90aZ\xC9V[\x90`\x12\x03`\x12\x81\x11a>\\Wa[\0\x90aZ\xC9V[\x90\x04\x90V[\x81\x15a[\x0FW\x04\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[\x90\x81\x15a[|W`\x12\x03`\x12\x81\x11a>\\Wa[Y\x90aZ\xC9V[`\0\x19\x82\x01\x91\x82\x11a>\\Wa[n\x91a[\x05V[`\x01\x81\x01\x80\x91\x11a>\\W\x90V[PP`\0\x90V[`\0\x90\x15a[\xA6WPg\x0F\0\0\0\0\0\0\0`\x01\x91[` \x1B\x91`8\x1B\x16\x17\x17\x90V[g\x0F\0\0\0\0\0\0\0\x90\x91a[\x99V[\x93\x96\x95\x94\x91\x90a[\xC8a4c\x86aAOV[a^\xA7Wa[\xD6B\x86a^\xE3V[\x80\x15a^}Wa[\xE8a\\%\x91al<V[\x85\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82T\x16\x17\x90UV[\x80\x15a^SWa\\7a\\x\x91al<V[\x84To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x80\x91\x90\x91\x1B\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x17\x84UV[a\x03\xE8\x80\x83\x10\x90\x83\x14\x17`\x01\x83\x11`\x01\x84\x14\x17\x16\x15a^!Wa\\\x9A\x82a^\xD1V[\x91a\\\xEB`\x01\x85\x01\x93\x84\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFu\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83T\x92`\xA0\x1B\x16\x91\x16\x17\x90UV[`\x01`\x01`\xA0\x1B\x03\x82\x16a]9W[PPPa\x1E\xD1\x92\x93P`\x03\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82T\x16\x17\x90UV[\x80\x87\x10\x90\x87\x14\x17`\x01\x87\x11`\x01\x88\x14\x17\x16\x15a]\xEFW\x82\x91a]\x9Ea\x1E\xD1\x96\x97a]\x99a]\xE6\x94`\x02`\x03\x98\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82T\x16\x17\x90UV[a^\xD1V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFw\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83T\x92`\xB0\x1B\x16\x91\x16\x17\x90UV[\x90\x84\x938a\\\xFAV[`@Q\x7F\xED\xDF\xE1\x19\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x87\x90R`$\x90\xFD[`@Q\x7F\x97\x1B1\t\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x83\x90R`$\x90\xFD[`\x04`@Q\x7F(i\xC5\xF3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xFD[`\x04`@Q\x7F]?Pl\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xFD[`\x04`@Q\x7F\xE90\xCE\xDF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xFD[b\x01\0\0\x81\x10\x15a\x03\xE0Wa\xFF\xFF\x16\x90V[d\x01\0\0\0\0\x82\x10\x15a\x03\xE0W`\x01\x01\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFs\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83T\x92`\x80\x1B\x16\x91\x16\x17\x90UV[\x90`\x01a\x1E\xD1\x92\x01\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x83T\x16\x90`\0\x83`\x0F\x0B\x13`\0\x14a_\xABWa_p\x92\x16\x90aB\xE6V[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82T\x16\x17\x90UV[a_\xB7a_p\x93aB&V[\x16\x90aC\nV[\x92\x91\x90\x15a`\xA3Wa`5a`\x80\x92a!\x9Da_\xF8a_\xDFa`?\x95al<V[\x87To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aB\xE6V[\x86\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82T\x16\x17\x90UV[\x83T`\x80\x1CaC\nV[\x82To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x80\x91\x90\x91\x1B\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x17\x82UV[To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x15a^}W`\x80\x1C\x15a^SWV[a`\xD5a`?\x91a!\x9Da_\xF8a`\xBCa`\xDF\x96al<V[\x87To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aC\nV[\x83T`\x80\x1CaB\xE6V[a`\x80V[c\xFF\xFF\xFF\xFF``\x82\x01Q\x16\x15\x15\x90\x81a`\xFBWP\x90V[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91P`@\x01Q\x16\x15\x90V[\x81\x15\x80abRW[ab\x13W\x82\x15\x80ab\x1BW[ab\x13Wa\x06\x15\x92`\0\x92\x83\x92aaA\x81a`\xE4V[\x15aa\xEDWg\r\xE0\xB6\xB3\xA7d\0\0\x91[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFaa|\x83Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x16\x90\x81aa\xCFW[PP` \x01Qaa\xA5\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a4\x04V[\x91\x82aa\xBDW[PPP\x80\x82\x11\x90\x82\x03\x02\x90\x03al<V[aa\xC7\x93Pak\xD6V[8\x80\x80aa\xACV[aa\xA5\x92\x96Paa\xE5a4\x04\x92\x85` \x93ak\xD6V[\x96\x92Paa\x84V[ab\ra4\x04`@\x83\x01Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x91aaQV[PPP`\0\x90V[Po\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFabJ` \x83\x01Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x16\x15\x15aa+V[Po\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFab~\x82Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x16\x15\x15aa\x1FV[\x91\x90\x80`\x0F\x0B\x90\x81\x15ac\x92W`\0ab\xB5a4\x04`@\x87\x01Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x92\x13\x15acDWa\x06\x15\x91a!\x9D\x91ab\xCD\x86a`\xE4V[ac4W[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16ac.a4\x04` ac\x16a!\x9D\x86ac\x10a4\x04\x8DQo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x87al\x14V[\x98\x01Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x90al\x14V[g\r\xE0\xB6\xB3\xA7d\0\0\x91Pab\xD2V[a\x06\x15\x91acZa4\x04a4\x04a!\x9D\x94aB&V[ac\x8Ca4\x04` ac\x16a!\x9D\x86ac\x86a4\x04\x8DQo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x87ak\xD6V[\x90ak\xD6V[PP\x90P`\0\x90`\0\x90V[\x91\x92`d` \x92\x94`@Q\x95`\0\x95\x86\x94\x85\x93\x84\x93\x7F#\xB8r\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85R`\x04R`$R`DRZ\xF1=\x15`\x1F=\x11`\x01\x84Q\x14\x16\x17\x16\x90``R\x81`@R\x15ac\xFFWPV[\x80\x7Fn\x89\xEC\xA5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x92R\xFD[\x90\x91` \x90`D`@Q\x94`\0\x80\x95\x81\x94\x82\x93\x7F\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04R`$RZ\xF1=\x15`\x1F=\x11`\x01\x84Q\x14\x16\x17\x16\x90``R\x81`@R\x15ad\x83WPV[\x80\x7F\xEB,\xF4\xFC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x92R\xFD[\x91\x94\x92\x94`\0ad\xCFo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x97\x88\x86Q\x16ak\xEEV[\x95\x80af\x7FW[P\x80\x95\x80\x97`\x80\x86\x01\x92ae*ad\xED\x85Q\x15\x15\x90V[\x93\x84\x15afxW\x87\x94[\x15afnWae%\x87\x95[ae\x1Fa4\x04\x8CQo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x90aF\xEEV[aF\xE1V[\x90\x81\x81\x11afDWae?` \x91\x85\x93aF\xE1V[\x97\x01\x91ae\\\x83Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x16\x11af\x1AWae\x93\x91ae\x86a4\x04ae\x8C\x93Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x90aF\xE1V[\x91Q\x15\x15\x90V[\x90\x81\x15af\x11W\x84\x85\x92[\x15af\tWP\x92[\x14ae\xDFW\x81\x14ae\xB5W\x90\x91V[`\x04`@Q\x7F\x1F\xB0\xB7\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xFD[`\x04`@Q\x7Frv\xF0\x8A\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xFD[\x90P\x92ae\xA6V[\x80\x94\x85\x92ae\x9EV[`\x04`@Q\x7F\x86j\x03+\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xFD[`\x04`@Q\x7F\xEC\x8E\x1F\xCE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xFD[ae%\x88\x95ae\x02V[\x86\x94ad\xF7V[af\x8A\x91P\x86a[\x05V[\x94\x85\x81\x03\x90\x81\x11a>\\W\x948ad\xD6V[af\xA4aFtV[P`@\x81\x01\x80Q\x90af\xC0a\x01\0\x84\x01\x92`\xFF\x84Q\x16\x90aZ\xB0V[\x90R``\x82\x01af\xD6\x81Q`\xFF\x84Q\x16\x90aZ\xB0V[\x90Raf\xED`\x80\x83\x01\x91`\xFF\x83Q\x91Q\x16\x90aZ\xB0V[\x90R`\xA0\x81\x01ag\x07\x81Q`\xFFa\x01 \x85\x01Q\x16\x90aZ\xB0V[\x90R\x90V[ag\x14aFtV[P`@\x81\x01\x80Q\x90ag0a\x01\0\x84\x01\x92`\xFF\x84Q\x16\x90aZ\xEBV[\x90R``\x82\x01agF\x81Q`\xFF\x84Q\x16\x90aZ\xEBV[\x90Rag]`\x80\x83\x01\x91`\xFF\x83Q\x91Q\x16\x90aZ\xEBV[\x90R`\xA0\x81\x01ag\x07\x81Q`\xFFa\x01 \x85\x01Q\x16\x90aZ\xEBV[\x15ag~WV[`d`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01R\x7FNOT_AUTHORIZED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R\xFD[\x15ag\xC9WV[`d`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01R\x7FINVALID_RECIPIENT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R\xFD[\x90\x81` \x91\x03\x12a\x041WQa\x06\x15\x81a\x04;V[\x91\x92a\x06\x15\x96\x94\x91`\xA0\x94`\x01`\x01`\xA0\x1B\x03\x80\x92\x16\x85R\x16` \x84\x01R`@\x83\x01R``\x82\x01R\x81`\x80\x82\x01R\x01\x91aHZV[\x15ah^WV[`d`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01R\x7FUNSAFE_RECIPIENT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R\xFD[\x15ah\xA9WV[`d`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01R\x7FLENGTH_MISMATCH\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R\xFD[\x91\x90\x81\x10\x15a?\xC1W`\x05\x1B\x01\x90V[\x90\x91\x82\x81R\x7F\x07\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11ai:W` \x92`\x05\x1B\x80\x92\x84\x83\x017\x01\x01\x90V[`\x84`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FABI encoding: array data too lon`D\x82\x01R\x7Fg\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R\xFD[\x92\x90ai\xBD\x90a\x06\x15\x95\x93`@\x86R`@\x86\x01\x91ah\xFDV[\x92` \x81\x85\x03\x91\x01Rah\xFDV[\x96\x94\x92aj\x0E\x94aj\0\x92a\x06\x15\x9A\x98\x94`\x01`\x01`\xA0\x1B\x03\x80\x92\x16\x8BR\x16` \x8A\x01R`\xA0`@\x8A\x01R`\xA0\x89\x01\x91ah\xFDV[\x91\x86\x83\x03``\x88\x01Rah\xFDV[\x92`\x80\x81\x85\x03\x91\x01RaHZV[\x90\x92`\xA0\x92`\x01`\x01`\xA0\x1B\x03a\x06\x15\x96\x95\x16\x83R`\0` \x84\x01R`@\x83\x01R``\x82\x01R\x81`\x80\x82\x01R\x01\x90a\x05\xC1V[\x92\x91\x90\x91ajt\x83a\x0E\xA7\x86`\x01`\x01`\xA0\x1B\x03\x16`\0R`\0` R`@`\0 \x90V[aj\x7F\x83\x82TaF\xEEV[\x90U`\x01`\x01`\xA0\x1B\x03\x84\x16\x93\x84`\0`@Q\x7F\xC3\xD5\x81h\xC5\xAEs\x97s\x1D\x06=[\xBF=exTBsC\xF4\xC0\x83$\x0Fz\xAC\xAA-\x0Fb3\x91\x80aj\xCE\x89\x8B\x83` \x90\x93\x92\x91\x93`@\x81\x01\x94\x81R\x01RV[\x03\x90\xA4;\x15ak_W\x83;\x15a\x07\xA8Wak\x1F\x93` \x92`\0`@Q\x80\x97\x81\x95\x82\x94\x7F\xF2:na\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x99\x8A\x85R3`\x04\x86\x01aj\x1CV[\x03\x92Z\xF1\x91\x82\x15a\x07\xA3Wa\x1E\xD1\x92\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91`\0\x91a\x0E$WP\x16\x14ahWV[PPPa\x1E\xD1\x90\x15\x15ag\xC2V[`\x01`\x01`\xA0\x1B\x03\x16\x90`\0\x92\x82\x84R\x83` R`@\x84 \x82\x85R` R`@\x84 \x80T\x91\x80\x83\x03\x92\x83\x11a>\\W\x91\x90U`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R3\x91\x7F\xC3\xD5\x81h\xC5\xAEs\x97s\x1D\x06=[\xBF=exTBsC\xF4\xC0\x83$\x0Fz\xAC\xAA-\x0Fb\x91\x90\xA4V[\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17\x82\x15\x15\x16\x15a\x03\xE0W\x04\x90V[\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x03\xE0W`\x01a'\x10`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[\x91\x90\x91\x82\x81\x02\x92\x81\x84\x04\x14\x90\x15\x17\x81\x15\x15\x16\x15a\x03\xE0W`\x01\x90`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x10\x15a\x03\xE0Wo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V";
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
