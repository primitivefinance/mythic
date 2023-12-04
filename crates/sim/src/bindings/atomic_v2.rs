pub use atomic_v2::*;
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
pub mod atomic_v2 {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("exchangeAddress"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("liquidExchangeAddress"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("assetAddress"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("quoteAddress"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("XTOY"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("XTOY"),
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
                    ::std::borrow::ToOwned::to_owned("YTOX"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("YTOX"),
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
                    ::std::borrow::ToOwned::to_owned("asset"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("asset"),
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
                    ::std::borrow::ToOwned::to_owned("cumulativeProfit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("cumulativeProfit"),
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
                    ::std::borrow::ToOwned::to_owned("exchange"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("exchange"),
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
                    ::std::borrow::ToOwned::to_owned("findTrade"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("findTrade"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("swapXIn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("priceTarget"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("epsilon"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("max_iters"),
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
                                    name: ::std::borrow::ToOwned::to_owned("amountIn"),
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
                    ::std::borrow::ToOwned::to_owned("intermediateTokenXBalance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "intermediateTokenXBalance",
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
                    ::std::borrow::ToOwned::to_owned("intermediateTokenYEndBalance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "intermediateTokenYEndBalance",
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
                    ::std::borrow::ToOwned::to_owned("intermediateTokenYStartBalance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "intermediateTokenYStartBalance",
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
                    ::std::borrow::ToOwned::to_owned("liquidExchange"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("liquidExchange"),
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
                    ::std::borrow::ToOwned::to_owned("lower_exchange_price"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "lower_exchange_price",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("input"),
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
                    ::std::borrow::ToOwned::to_owned("quote"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("quote"),
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
                    ::std::borrow::ToOwned::to_owned("raise_exchange_price"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "raise_exchange_price",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("input"),
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
                    ::std::borrow::ToOwned::to_owned("try_arbitrage_until_target_price"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "try_arbitrage_until_target_price",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("target_price"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("max_iterations"),
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
                                    name: ::std::borrow::ToOwned::to_owned("amountIn"),
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
                    ::std::borrow::ToOwned::to_owned("Loss"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Loss"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("loss"),
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
                    ::std::borrow::ToOwned::to_owned("Profit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Profit"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("profit"),
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
                    ::std::borrow::ToOwned::to_owned("CatastrophicSwapFailure"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "CatastrophicSwapFailure",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("DexSwapFailure"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("DexSwapFailure"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("reason"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("err"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("FindingTradeError"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("FindingTradeError"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("err"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InsufficientApprovalY"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InsufficientApprovalY",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("allowance"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("payment"),
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
                    ::std::borrow::ToOwned::to_owned("InsufficientBalanceX"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InsufficientBalanceX",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("balance"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("payment"),
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
                    ::std::borrow::ToOwned::to_owned("InsufficientBalanceY"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InsufficientBalanceY",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("balance"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("payment"),
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
                    ::std::borrow::ToOwned::to_owned("NotProfitable"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("NotProfitable"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("first_swap_output"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "second_swap_output",
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
                    ::std::borrow::ToOwned::to_owned("SimulatedSwapFailure"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "SimulatedSwapFailure",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("valid"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("estimatedOut"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("estimatedPrice"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("payload"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("UnprofitableArbitrage"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "UnprofitableArbitrage",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("start_y_balance"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("end_y_balance"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "absolute_difference",
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
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static ATOMICV2_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R`\x08\x80Ta\xFF\xFF\x19\x16`\x01\x17\x90U4\x80\x15b\0\0lW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`@Qb\0$#8\x03\x80b\0$#\x839\x81\x01`@\x81\x90Rb\0\0\x8F\x91b\0\x01\0V[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x95\x86\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x82\x16\x17\x90\x91U`\x01\x80T\x94\x86\x16\x94\x82\x16\x94\x90\x94\x17\x90\x93U`\x02\x80T\x92\x85\x16\x92\x84\x16\x92\x90\x92\x17\x90\x91U`\x03\x80T\x91\x90\x93\x16\x91\x16\x17\x90Ub\0\x01\xA8V[\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\0\xFBW`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15b\0\x01bW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[b\0\x01m\x85b\0\0\xE3V[\x93Pb\0\x01}` \x86\x01b\0\0\xE3V[\x92Pb\0\x01\x8D`@\x86\x01b\0\0\xE3V[\x91Pb\0\x01\x9D``\x86\x01b\0\0\xE3V[\x90P\x92\x95\x91\x94P\x92PV[a\"k\x80b\0\x01\xB8`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x017W`\x005`\xE0\x1C\x80c\x99\x9B\x93\xAF\x11a\0\xD9W\x80c\xB9\x93>\x84\x11a\0\xB3W\x80c\xB9\x93>\x84\x14a\x02xW\x80c\xD2\xF7&Z\x14a\x02\x8BW\x80c\xF3\xC9s\xCF\x14a\x02\x9EW\x80c\xFA.Y\x94\x14a\x02\xABWa\x017V[\x80c\x99\x9B\x93\xAF\x14a\x02?W\x80c\x9F'\xEFO\x14a\x02RW\x80c\xA6\xB8|5\x14a\x02eWa\x017V[\x80cr\xB9\x82F\x11a\x01\x15W\x80cr\xB9\x82F\x14a\x02\x03W\x80c\x85\xB3\x19\xFF\x14a\x02\x1AW\x80c\x8A/\xA5J\x14a\x02#W\x80c\x93e \xC3\x14a\x026Wa\x017V[\x80c5\xA9\x9A\xD0\x14a\x01\x9CW\x80c8\xD5.\x0F\x14a\x01\xB1W\x80cdI\xFCW\x14a\x01\xE1W[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x01\xAFa\x01\xAA6`\x04a\x1D\x82V[a\x02\xB4V[\0[`\x02Ta\x01\xC4\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[`\x08Ta\x01\xF3\x90a\x01\0\x90\x04`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x01\xD8V[a\x02\x0C`\x06T\x81V[`@Q\x90\x81R` \x01a\x01\xD8V[a\x02\x0C`\x07T\x81V[a\x01\xAFa\x0216`\x04a\x1D\x82V[a\x03\xA4V[a\x02\x0C`\x04T\x81V[`\x03Ta\x01\xC4\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x01Ta\x01\xC4\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x02\x0Ca\x02s6`\x04a\x1D\x9EV[a\x04\x89V[a\x02\x0Ca\x02\x866`\x04a\x1D\xD1V[a\tMV[`\0Ta\x01\xC4\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x08Ta\x01\xF3\x90`\xFF\x16\x81V[a\x02\x0C`\x05T\x81V[a\x02\xBD\x81a\x0F[V[`\x08Ta\x02\xD2\x90a\x01\0\x90\x04`\xFF\x16\x82a\x12\x98V[`\x08T`\x02T`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01Ra\x03\x99\x92`\xFF\x16\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x03\\W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\"K\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x03pW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\x94\x91\x90a\x1E V[a\x15\xFBV[a\x03\xA1a\x19\xFDV[PV[a\x03\xAD\x81a\x0F[V[`\x08Ta\x03\xC2\x90a\x01\0\x90\x04`\xFF\x16\x82a\x15\xFBV[`\x08T`\x02T`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01Ra\x03\x99\x92`\xFF\x16\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x04LW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\"K\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x04`W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\x84\x91\x90a\x1E V[a\x12\x98V[`\0\x80T`@\x80QcB\xC9\x88[`\xE1\x1B\x81R\x90Q\x83\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x85\x93\x10\xB6\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\x05\x0CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\"K\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x05 W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05D\x91\x90a\x1E V[\x90P`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cg\xE8(\xBF`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x05\xD3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\"K\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x05\xE7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\x0B\x91\x90a\x1E<V[`\x01`\x01`\xA0\x1B\x03\x16c\xC1\xE0\x04;`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01```@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x06\x81W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\"K\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x06\x95W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xB9\x91\x90a\x1EoV[PP\x90P`\0\x80`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEB\xAD\xEF\x01`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01```@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x07MW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\"K\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x07aW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x85\x91\x90a\x1EoV[\x92P\x92P\x92P\x87\x85\x10\x15a\x08PW`\n`\0\x82a\x07\xAA\x85g\r\xE0\xB6\xB3\xA7d\0\0a\x1E\xB6V[a\x07\xB4\x91\x90a\x1E\xD3V[a\x07\xBE\x90\x87a\x1E\xF5V[\x90Pa\x07\xCB`\n\x82a\x1E\xF5V[\x90Pa\x07\xFA`@Q\x80`@\x01`@R\x80`\x0B\x81R` \x01j6\xB4\xB7 \xB6\xB7\xBA\xB7:$\xB7`\xA9\x1B\x81RP\x83a\x1CHV[a\x08'`@Q\x80`@\x01`@R\x80`\x0B\x81R` \x01j6\xB0\xBC \xB6\xB7\xBA\xB7:$\xB7`\xA9\x1B\x81RP\x82a\x1CHV[a\x08G`\x08`\x01\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x8B\x84\x84`\x05a\x01\0a\tMV[\x97PPPa\tBV[`@\x80Q\x80\x82\x01\x90\x91R`\x0F\x81Rn!\xB7\xB6\xB8:\xBA2\xB2\x106\xB0\xBC\x104\xB7`\x89\x1B` \x82\x01R`\n\x90a\x08\xB1\x90\x83a\x08\x90\x87g\r\xE0\xB6\xB3\xA7d\0\0a\x1E\xB6V[a\x08\x9A\x91\x90a\x1E\xD3V[a\x08\xAC\x90g\r\xE0\xB6\xB3\xA7d\0\0a\x1E\xF5V[a\x1CHV[`\0a\x08\xBD\x85\x84a\x1E\xF5V[\x90Pa\x08\xCA`\n\x82a\x1E\xF5V[\x90Pa\x08\xF9`@Q\x80`@\x01`@R\x80`\x0B\x81R` \x01j6\xB4\xB7 \xB6\xB7\xBA\xB7:$\xB7`\xA9\x1B\x81RP\x83a\x1CHV[a\t&`@Q\x80`@\x01`@R\x80`\x0B\x81R` \x01j6\xB0\xBC \xB6\xB7\xBA\xB7:$\xB7`\xA9\x1B\x81RP\x82a\x1CHV[`\x08Ta\t=\x90`\xFF\x16\x8B\x84\x84`\x05a\x01\0a\tMV[\x97PPP[PPPPP\x92\x91PPV[`\0\x83\x85\x10a\t\xA3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7Flower must be less than upper\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x80T`@Qc\x02\xB0\xCC\r`\xE2\x1B\x81R\x89\x15\x15`\x04\x82\x01R`$\x81\x01\x88\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\n\xC304\x90`D\x01`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\n.W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\"K\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x92PPP\x80\x15a\ncWP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\n`\x91\x90\x81\x01\x90a\x1FBV[`\x01[a\n\xE3W=\x80\x80\x15a\n\x91W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\n\x96V[``\x91P[Pa\n\xC8`@Q\x80`@\x01`@R\x80`\x10\x81R` \x01o67\xBB\xB2\xB9(94\xB1\xB2\x902\xB997\xB9`\x81\x1B\x81RPa\x1C\x8DV[\x80`@Qc\x1AC\x9E\xD1`\xE0\x1B\x81R`\x04\x01a\t\x9A\x91\x90a!3V[P`\0\x80T`@Qc\x02\xB0\xCC\r`\xE2\x1B\x81R\x8D\x15\x15`\x04\x82\x01R`$\x81\x01\x8B\x90R\x92\x95P\x90\x93P`\x01`\x01`\xA0\x1B\x03\x16\x91Pc\n\xC304\x90`D\x01`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0BtW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\"K\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x92PPP\x80\x15a\x0B\xA9WP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0B\xA6\x91\x90\x81\x01\x90a\x1FBV[`\x01[a\x0C\x0EW=\x80\x80\x15a\x0B\xD7W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x0B\xDCV[``\x91P[Pa\n\xC8`@Q\x80`@\x01`@R\x80`\x10\x81R` \x01o:\xB882\xB9(94\xB1\xB2\x902\xB997\xB9`\x81\x1B\x81RPa\x1C\x8DV[P\x92PPPa\x0C?`@Q\x80`@\x01`@R\x80`\n\x81R` \x01ilowerPrice`\xB0\x1B\x81RP\x83a\x1CHV[a\x0Ck`@Q\x80`@\x01`@R\x80`\n\x81R` \x01iupperPrice`\xB0\x1B\x81RP\x82a\x1CHV[a\x0C\x97`@Q\x80`@\x01`@R\x80`\n\x81R` \x01itargtPrice`\xB0\x1B\x81RP\x89a\x1CHV[`\0a\x0C\xA3\x89\x84a!FV[\x90P`\0a\x0C\xB1\x8A\x84a!FV[\x90Pa\x0C\xBC\x82a\x1C\xD0V[a\x0C\xC5\x81a\x1C\xD0V[`\0a\x0C\xD1\x82\x84a!mV[\x12a\r\x1EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7Froot must be between bounds\0\0\0\0\0`D\x82\x01R`d\x01a\t\x9AV[`\0a\r*\x8A\x8Aa\x1E\xF5V[\x90P`\0[`\x02a\r;\x8B\x8Da!\x9DV[a\rE\x91\x90a\x1E\xD3V[\x96P`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\n\xC304\x8F\x8A`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\r\x96\x92\x91\x90\x91\x15\x15\x82R` \x82\x01R`@\x01\x90V[`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\r\xECW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\"K\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x92PPP\x80\x15a\x0E!WP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0E\x1E\x91\x90\x81\x01\x90a\x1FBV[`\x01[a\x0E[W=\x80\x80\x15a\x0EOW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x0ETV[``\x91P[PPa\x0EaV[P\x92PPP[a\x0E\x8B`@Q\x80`@\x01`@R\x80`\x08\x81R` \x01g0\xB6\xB7\xBA\xB7:$\xB7`\xC1\x1B\x81RP\x89a\x1CHV[a\x0E\xB9`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01ktarget price`\xA0\x1B\x81RP\x8Ea\x1CHV[a\x0E\xE7`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01kresult price`\xA0\x1B\x81RP\x82a\x1CHV[`\0a\x0E\xF3\x8E\x83a!FV[\x90Pa\x0E\xFE\x81a\x1C\xD0V[`\0a\x0F\n\x87\x83a!mV[\x13a\x0F\x17W\x88\x9BPa\x0F\x1EV[\x88\x9CP\x80\x95P[a\x0F(\x8D\x8Da\x1E\xF5V[\x93Pa\x0F5`\x01\x84a!\x9DV[\x92PPP\x88\x82\x11\x80\x15a\x0FGWP\x87\x81\x10[a\r/WPPPPPP\x96\x95PPPPPPV[`\x03T`@Qcp\xA0\x821`\xE0\x1B\x81R3`\x04\x82\x01R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0F\xDDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\"K\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0F\xF1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\x15\x91\x90a\x1E V[\x90P\x81\x81\x10\x15a\x10BW`@Qc\n\xBEZ\x89`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x81\x01\x83\x90R`D\x01a\t\x9AV[`\x03T`@Qcn\xB1v\x9F`\xE1\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xDDb\xED>\x90`D\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x10\xCAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\"K\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x10\xDEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\x02\x91\x90a\x1E V[\x90P\x82\x81\x10\x15a\x11/W`@Qc\xDAV\xD3\xC5`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x81\x01\x84\x90R`D\x01a\t\x9AV[`\x03T`@Qc#\xB8r\xDD`\xE0\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x85\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90`d\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x11\xBFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\"K\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x11\xD3W=`\0\x80>=`\0\xFD[PP`\x03T`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x92Pcp\xA0\x821\x91P`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x12XW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\"K\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x12lW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\x90\x91\x90a\x1E V[`\x05UPPPV[\x81\x15a\x13\xEEW`\x02T`\x01T`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x84\x90R\x91\x16\x90c\t^\xA7\xB3\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x13,W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\"K\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x13@W=`\0\x80>=`\0\xFD[PP`\x01T`\x02T`@Qc\xD0\x04\xF0\xF7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x86\x90R\x91\x16\x92Pc\xD0\x04\xF0\xF7\x91P`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x13\xD2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\"K\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x13\xE6W=`\0\x80>=`\0\xFD[PPPPPPV[`\x03T`\x01T`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x84\x90R\x91\x16\x90c\t^\xA7\xB3\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x14|W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\"K\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x14\x90W=`\0\x80>=`\0\xFD[PP`\x01T`\x03T`@Qc\xD0\x04\xF0\xF7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x86\x90R\x91\x16\x92Pc\xD0\x04\xF0\xF7\x91P`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x15\"W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\"K\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x156W=`\0\x80>=`\0\xFD[PP`\x02T`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x92Pcp\xA0\x821\x91P`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x15\xBBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\"K\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x15\xCFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\xF3\x91\x90a\x1E V[`\x04U[PPV[\x81\x15a\x16\xACW`\x02T`\0T`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x84\x90R\x91\x16\x90c\t^\xA7\xB3\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x16\x8FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\"K\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x16\xA3W=`\0\x80>=`\0\xFD[PPPPa\x17SV[`\x03T`\0T`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x84\x90R\x91\x16\x90c\t^\xA7\xB3\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x17:W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\"K\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x17NW=`\0\x80>=`\0\xFD[PPPP[`\0\x80T`@Qc\x02\xB0\xCC\r`\xE2\x1B\x81R\x84\x15\x15`\x04\x82\x01R`$\x81\x01\x84\x90R\x82\x91\x82\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\n\xC304\x90`D\x01`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x17\xE2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\"K\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x17\xF6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x18\x1E\x91\x90\x81\x01\x90a\x1FBV[\x93P\x93P\x93P\x93P\x83a\x18LW\x83\x83\x83\x83`@Qc\x03\x14\xE6#`\xE3\x1B\x81R`\x04\x01a\t\x9A\x94\x93\x92\x91\x90a!\xB0V[`\0T`@Qc1>\xEA\xB5`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cb}\xD5j\x90a\x18|\x90\x84\x90`\x04\x01a!3V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x18\xD4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\"K\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x92PPP\x80\x15a\x18\xE5WP`\x01[a\x194W=\x80\x80\x15a\x19\x13W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x19\x18V[``\x91P[P\x80`@Qcg\xA1k\x8D`\xE1\x1B\x81R`\x04\x01a\t\x9A\x91\x90a!\xE1V[\x85a\x13\xE6W`\x02T`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x19\xBAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\"K\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x19\xCEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19\xF2\x91\x90a\x1E V[`\x04UPPPPPPV[`\x03T`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x1A~W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\"K\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x1A\x92W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A\xB6\x91\x90a\x1E V[`\x06\x81\x90U`\x05T\x11\x15a\x1BAW\x7F\xB6[.\x08]}\x04\x0C1?}N\x1A\xC9\x0FY7\x02o\xEEI~\x0E$\xA7\xEF\xF1jU\xE1\xC5\xEA`\x06T`\x05Ta\x1A\xF5\x91\x90a\x1E\xF5V[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xA1`\x05T`\x06Ta\x1B\x16\x81\x83a!FV[`@Qc\xB1n7\x83`\xE0\x1B\x81R`\x04\x81\x01\x93\x90\x93R`$\x83\x01\x91\x90\x91R\x19`D\x82\x01R`d\x01a\t\x9AV[`\0`\x05T`\x06Ta\x1BS\x91\x90a\x1E\xF5V[\x90P\x80`\x07`\0\x82\x82Ta\x1Bg\x91\x90a!\x9DV[\x90\x91UPP`@Q\x81\x81R\x7F5}\x90_\x181 \x97\x97\xDFMU\xD7\x9C\\[\xF1\xD9\xF71\x1C\x97j\xFD\x05\xE1=\x88\x1E\xAB\x9B\xC8\x90` \x01`@Q\x80\x91\x03\x90\xA1`\x03T`\x06T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1C-W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\"K\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x1CAW=`\0\x80>=`\0\xFD[PPPPPV[a\x15\xF7\x82\x82`@Q`$\x01a\x1C^\x92\x91\x90a\"(V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c-\x83\x9C\xB3`\xE2\x1B\x17\x90Ra\x1D\x11V[a\x03\xA1\x81`@Q`$\x01a\x1C\xA1\x91\x90a!3V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\x10L\x13\xEB`\xE2\x1B\x17\x90Ra\x1D\x11V[a\x03\xA1\x81`@Q`$\x01a\x1C\xE6\x91\x81R` \x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c-[l\xB9`\xE0\x1B\x17\x90R[\x80Qjconsole.log` \x83\x01`\0\x80\x84\x83\x85Z\xFAPPPPPV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`\0` \x82\x84\x03\x12\x15a\x1D\x97Wa\x1D\x97a\x1D2V[P5\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x1D\xB4Wa\x1D\xB4a\x1D2V[PP\x805\x92` \x90\x91\x015\x91PV[\x80\x15\x15\x81\x14a\x03\xA1W`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a\x1D\xEDWa\x1D\xEDa\x1D2V[\x865a\x1D\xF8\x81a\x1D\xC3V[\x98` \x88\x015\x98P`@\x88\x015\x97``\x81\x015\x97P`\x80\x81\x015\x96P`\xA0\x015\x94P\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x1E5Wa\x1E5a\x1D2V[PQ\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x1EQWa\x1EQa\x1D2V[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x1EhW`\0\x80\xFD[\x93\x92PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x1E\x87Wa\x1E\x87a\x1D2V[\x83Q\x92P` \x84\x01Q\x91P`@\x84\x01Q\x90P\x92P\x92P\x92V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x1E\xCDWa\x1E\xCDa\x1E\xA0V[\x92\x91PPV[`\0\x82a\x1E\xF0WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[\x81\x81\x03\x81\x81\x11\x15a\x1E\xCDWa\x1E\xCDa\x1E\xA0V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0[\x83\x81\x10\x15a\x1F9W\x81\x81\x01Q\x83\x82\x01R` \x01a\x1F!V[PP`\0\x91\x01RV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x1F[Wa\x1F[a\x1D2V[\x84Qa\x1Ff\x81a\x1D\xC3V[\x80\x94PP` \x80\x86\x01Q\x93P`@\x80\x87\x01Q\x93P``\x87\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x1F\xDFW\x82QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x85\x90R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[\x81\x89\x01\x91P\x89`\x1F\x83\x01\x12a FW\x82QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x85\x90R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x81\xFD[\x81Q\x81\x81\x11\x15a XWa Xa\x1F\x08V[\x83Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a \x7FWa \x7Fa\x1F\x08V[\x81\x86R\x82\x81R\x8C\x87\x84\x87\x01\x01\x11\x15a \xE7W\x85QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x88\x90R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R\x93P`\x84\x84\xFD[a \xF6\x83\x88\x83\x01\x89\x88\x01a\x1F\x1EV[\x99\x9C\x98\x9BP\x96\x99PPPPPPPPV[`\0\x81Q\x80\x84Ra!\x1F\x81` \x86\x01` \x86\x01a\x1F\x1EV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x1Eh` \x83\x01\x84a!\x07V[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a!fWa!fa\x1E\xA0V[P\x92\x91PPV[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15a!\x89Wa!\x89a\x1E\xA0V[\x81\x81\x05\x83\x14\x82\x15\x17a\x1E\xCDWa\x1E\xCDa\x1E\xA0V[\x80\x82\x01\x80\x82\x11\x15a\x1E\xCDWa\x1E\xCDa\x1E\xA0V[\x84\x15\x15\x81R\x83` \x82\x01R\x82`@\x82\x01R`\x80``\x82\x01R`\0a!\xD7`\x80\x83\x01\x84a!\x07V[\x96\x95PPPPPPV[`@\x81R`\x19`@\x82\x01R\x7FDEX swap failed with data\0\0\0\0\0\0\0``\x82\x01R`\x80` \x82\x01R`\0a\x1Eh`\x80\x83\x01\x84a!\x07V[`@\x81R`\0a\";`@\x83\x01\x85a!\x07V[\x90P\x82` \x83\x01R\x93\x92PPPV\xFETarget contract does not contain";
    /// The bytecode of the contract.
    pub static ATOMICV2_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x017W`\x005`\xE0\x1C\x80c\x99\x9B\x93\xAF\x11a\0\xD9W\x80c\xB9\x93>\x84\x11a\0\xB3W\x80c\xB9\x93>\x84\x14a\x02xW\x80c\xD2\xF7&Z\x14a\x02\x8BW\x80c\xF3\xC9s\xCF\x14a\x02\x9EW\x80c\xFA.Y\x94\x14a\x02\xABWa\x017V[\x80c\x99\x9B\x93\xAF\x14a\x02?W\x80c\x9F'\xEFO\x14a\x02RW\x80c\xA6\xB8|5\x14a\x02eWa\x017V[\x80cr\xB9\x82F\x11a\x01\x15W\x80cr\xB9\x82F\x14a\x02\x03W\x80c\x85\xB3\x19\xFF\x14a\x02\x1AW\x80c\x8A/\xA5J\x14a\x02#W\x80c\x93e \xC3\x14a\x026Wa\x017V[\x80c5\xA9\x9A\xD0\x14a\x01\x9CW\x80c8\xD5.\x0F\x14a\x01\xB1W\x80cdI\xFCW\x14a\x01\xE1W[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x01\xAFa\x01\xAA6`\x04a\x1D\x82V[a\x02\xB4V[\0[`\x02Ta\x01\xC4\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[`\x08Ta\x01\xF3\x90a\x01\0\x90\x04`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x01\xD8V[a\x02\x0C`\x06T\x81V[`@Q\x90\x81R` \x01a\x01\xD8V[a\x02\x0C`\x07T\x81V[a\x01\xAFa\x0216`\x04a\x1D\x82V[a\x03\xA4V[a\x02\x0C`\x04T\x81V[`\x03Ta\x01\xC4\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x01Ta\x01\xC4\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x02\x0Ca\x02s6`\x04a\x1D\x9EV[a\x04\x89V[a\x02\x0Ca\x02\x866`\x04a\x1D\xD1V[a\tMV[`\0Ta\x01\xC4\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x08Ta\x01\xF3\x90`\xFF\x16\x81V[a\x02\x0C`\x05T\x81V[a\x02\xBD\x81a\x0F[V[`\x08Ta\x02\xD2\x90a\x01\0\x90\x04`\xFF\x16\x82a\x12\x98V[`\x08T`\x02T`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01Ra\x03\x99\x92`\xFF\x16\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x03\\W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\"K\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x03pW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\x94\x91\x90a\x1E V[a\x15\xFBV[a\x03\xA1a\x19\xFDV[PV[a\x03\xAD\x81a\x0F[V[`\x08Ta\x03\xC2\x90a\x01\0\x90\x04`\xFF\x16\x82a\x15\xFBV[`\x08T`\x02T`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01Ra\x03\x99\x92`\xFF\x16\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x04LW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\"K\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x04`W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\x84\x91\x90a\x1E V[a\x12\x98V[`\0\x80T`@\x80QcB\xC9\x88[`\xE1\x1B\x81R\x90Q\x83\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x85\x93\x10\xB6\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\x05\x0CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\"K\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x05 W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05D\x91\x90a\x1E V[\x90P`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cg\xE8(\xBF`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x05\xD3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\"K\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x05\xE7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\x0B\x91\x90a\x1E<V[`\x01`\x01`\xA0\x1B\x03\x16c\xC1\xE0\x04;`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01```@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x06\x81W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\"K\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x06\x95W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xB9\x91\x90a\x1EoV[PP\x90P`\0\x80`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEB\xAD\xEF\x01`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01```@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x07MW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\"K\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x07aW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x85\x91\x90a\x1EoV[\x92P\x92P\x92P\x87\x85\x10\x15a\x08PW`\n`\0\x82a\x07\xAA\x85g\r\xE0\xB6\xB3\xA7d\0\0a\x1E\xB6V[a\x07\xB4\x91\x90a\x1E\xD3V[a\x07\xBE\x90\x87a\x1E\xF5V[\x90Pa\x07\xCB`\n\x82a\x1E\xF5V[\x90Pa\x07\xFA`@Q\x80`@\x01`@R\x80`\x0B\x81R` \x01j6\xB4\xB7 \xB6\xB7\xBA\xB7:$\xB7`\xA9\x1B\x81RP\x83a\x1CHV[a\x08'`@Q\x80`@\x01`@R\x80`\x0B\x81R` \x01j6\xB0\xBC \xB6\xB7\xBA\xB7:$\xB7`\xA9\x1B\x81RP\x82a\x1CHV[a\x08G`\x08`\x01\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x8B\x84\x84`\x05a\x01\0a\tMV[\x97PPPa\tBV[`@\x80Q\x80\x82\x01\x90\x91R`\x0F\x81Rn!\xB7\xB6\xB8:\xBA2\xB2\x106\xB0\xBC\x104\xB7`\x89\x1B` \x82\x01R`\n\x90a\x08\xB1\x90\x83a\x08\x90\x87g\r\xE0\xB6\xB3\xA7d\0\0a\x1E\xB6V[a\x08\x9A\x91\x90a\x1E\xD3V[a\x08\xAC\x90g\r\xE0\xB6\xB3\xA7d\0\0a\x1E\xF5V[a\x1CHV[`\0a\x08\xBD\x85\x84a\x1E\xF5V[\x90Pa\x08\xCA`\n\x82a\x1E\xF5V[\x90Pa\x08\xF9`@Q\x80`@\x01`@R\x80`\x0B\x81R` \x01j6\xB4\xB7 \xB6\xB7\xBA\xB7:$\xB7`\xA9\x1B\x81RP\x83a\x1CHV[a\t&`@Q\x80`@\x01`@R\x80`\x0B\x81R` \x01j6\xB0\xBC \xB6\xB7\xBA\xB7:$\xB7`\xA9\x1B\x81RP\x82a\x1CHV[`\x08Ta\t=\x90`\xFF\x16\x8B\x84\x84`\x05a\x01\0a\tMV[\x97PPP[PPPPP\x92\x91PPV[`\0\x83\x85\x10a\t\xA3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7Flower must be less than upper\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x80T`@Qc\x02\xB0\xCC\r`\xE2\x1B\x81R\x89\x15\x15`\x04\x82\x01R`$\x81\x01\x88\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\n\xC304\x90`D\x01`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\n.W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\"K\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x92PPP\x80\x15a\ncWP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\n`\x91\x90\x81\x01\x90a\x1FBV[`\x01[a\n\xE3W=\x80\x80\x15a\n\x91W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\n\x96V[``\x91P[Pa\n\xC8`@Q\x80`@\x01`@R\x80`\x10\x81R` \x01o67\xBB\xB2\xB9(94\xB1\xB2\x902\xB997\xB9`\x81\x1B\x81RPa\x1C\x8DV[\x80`@Qc\x1AC\x9E\xD1`\xE0\x1B\x81R`\x04\x01a\t\x9A\x91\x90a!3V[P`\0\x80T`@Qc\x02\xB0\xCC\r`\xE2\x1B\x81R\x8D\x15\x15`\x04\x82\x01R`$\x81\x01\x8B\x90R\x92\x95P\x90\x93P`\x01`\x01`\xA0\x1B\x03\x16\x91Pc\n\xC304\x90`D\x01`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0BtW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\"K\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x92PPP\x80\x15a\x0B\xA9WP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0B\xA6\x91\x90\x81\x01\x90a\x1FBV[`\x01[a\x0C\x0EW=\x80\x80\x15a\x0B\xD7W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x0B\xDCV[``\x91P[Pa\n\xC8`@Q\x80`@\x01`@R\x80`\x10\x81R` \x01o:\xB882\xB9(94\xB1\xB2\x902\xB997\xB9`\x81\x1B\x81RPa\x1C\x8DV[P\x92PPPa\x0C?`@Q\x80`@\x01`@R\x80`\n\x81R` \x01ilowerPrice`\xB0\x1B\x81RP\x83a\x1CHV[a\x0Ck`@Q\x80`@\x01`@R\x80`\n\x81R` \x01iupperPrice`\xB0\x1B\x81RP\x82a\x1CHV[a\x0C\x97`@Q\x80`@\x01`@R\x80`\n\x81R` \x01itargtPrice`\xB0\x1B\x81RP\x89a\x1CHV[`\0a\x0C\xA3\x89\x84a!FV[\x90P`\0a\x0C\xB1\x8A\x84a!FV[\x90Pa\x0C\xBC\x82a\x1C\xD0V[a\x0C\xC5\x81a\x1C\xD0V[`\0a\x0C\xD1\x82\x84a!mV[\x12a\r\x1EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7Froot must be between bounds\0\0\0\0\0`D\x82\x01R`d\x01a\t\x9AV[`\0a\r*\x8A\x8Aa\x1E\xF5V[\x90P`\0[`\x02a\r;\x8B\x8Da!\x9DV[a\rE\x91\x90a\x1E\xD3V[\x96P`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\n\xC304\x8F\x8A`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\r\x96\x92\x91\x90\x91\x15\x15\x82R` \x82\x01R`@\x01\x90V[`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\r\xECW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\"K\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x92PPP\x80\x15a\x0E!WP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0E\x1E\x91\x90\x81\x01\x90a\x1FBV[`\x01[a\x0E[W=\x80\x80\x15a\x0EOW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x0ETV[``\x91P[PPa\x0EaV[P\x92PPP[a\x0E\x8B`@Q\x80`@\x01`@R\x80`\x08\x81R` \x01g0\xB6\xB7\xBA\xB7:$\xB7`\xC1\x1B\x81RP\x89a\x1CHV[a\x0E\xB9`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01ktarget price`\xA0\x1B\x81RP\x8Ea\x1CHV[a\x0E\xE7`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01kresult price`\xA0\x1B\x81RP\x82a\x1CHV[`\0a\x0E\xF3\x8E\x83a!FV[\x90Pa\x0E\xFE\x81a\x1C\xD0V[`\0a\x0F\n\x87\x83a!mV[\x13a\x0F\x17W\x88\x9BPa\x0F\x1EV[\x88\x9CP\x80\x95P[a\x0F(\x8D\x8Da\x1E\xF5V[\x93Pa\x0F5`\x01\x84a!\x9DV[\x92PPP\x88\x82\x11\x80\x15a\x0FGWP\x87\x81\x10[a\r/WPPPPPP\x96\x95PPPPPPV[`\x03T`@Qcp\xA0\x821`\xE0\x1B\x81R3`\x04\x82\x01R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0F\xDDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\"K\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0F\xF1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\x15\x91\x90a\x1E V[\x90P\x81\x81\x10\x15a\x10BW`@Qc\n\xBEZ\x89`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x81\x01\x83\x90R`D\x01a\t\x9AV[`\x03T`@Qcn\xB1v\x9F`\xE1\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xDDb\xED>\x90`D\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x10\xCAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\"K\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x10\xDEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\x02\x91\x90a\x1E V[\x90P\x82\x81\x10\x15a\x11/W`@Qc\xDAV\xD3\xC5`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x81\x01\x84\x90R`D\x01a\t\x9AV[`\x03T`@Qc#\xB8r\xDD`\xE0\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x85\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90`d\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x11\xBFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\"K\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x11\xD3W=`\0\x80>=`\0\xFD[PP`\x03T`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x92Pcp\xA0\x821\x91P`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x12XW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\"K\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x12lW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\x90\x91\x90a\x1E V[`\x05UPPPV[\x81\x15a\x13\xEEW`\x02T`\x01T`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x84\x90R\x91\x16\x90c\t^\xA7\xB3\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x13,W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\"K\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x13@W=`\0\x80>=`\0\xFD[PP`\x01T`\x02T`@Qc\xD0\x04\xF0\xF7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x86\x90R\x91\x16\x92Pc\xD0\x04\xF0\xF7\x91P`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x13\xD2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\"K\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x13\xE6W=`\0\x80>=`\0\xFD[PPPPPPV[`\x03T`\x01T`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x84\x90R\x91\x16\x90c\t^\xA7\xB3\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x14|W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\"K\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x14\x90W=`\0\x80>=`\0\xFD[PP`\x01T`\x03T`@Qc\xD0\x04\xF0\xF7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x86\x90R\x91\x16\x92Pc\xD0\x04\xF0\xF7\x91P`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x15\"W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\"K\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x156W=`\0\x80>=`\0\xFD[PP`\x02T`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x92Pcp\xA0\x821\x91P`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x15\xBBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\"K\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x15\xCFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\xF3\x91\x90a\x1E V[`\x04U[PPV[\x81\x15a\x16\xACW`\x02T`\0T`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x84\x90R\x91\x16\x90c\t^\xA7\xB3\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x16\x8FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\"K\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x16\xA3W=`\0\x80>=`\0\xFD[PPPPa\x17SV[`\x03T`\0T`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x84\x90R\x91\x16\x90c\t^\xA7\xB3\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x17:W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\"K\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x17NW=`\0\x80>=`\0\xFD[PPPP[`\0\x80T`@Qc\x02\xB0\xCC\r`\xE2\x1B\x81R\x84\x15\x15`\x04\x82\x01R`$\x81\x01\x84\x90R\x82\x91\x82\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\n\xC304\x90`D\x01`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x17\xE2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\"K\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x17\xF6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x18\x1E\x91\x90\x81\x01\x90a\x1FBV[\x93P\x93P\x93P\x93P\x83a\x18LW\x83\x83\x83\x83`@Qc\x03\x14\xE6#`\xE3\x1B\x81R`\x04\x01a\t\x9A\x94\x93\x92\x91\x90a!\xB0V[`\0T`@Qc1>\xEA\xB5`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cb}\xD5j\x90a\x18|\x90\x84\x90`\x04\x01a!3V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x18\xD4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\"K\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x92PPP\x80\x15a\x18\xE5WP`\x01[a\x194W=\x80\x80\x15a\x19\x13W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x19\x18V[``\x91P[P\x80`@Qcg\xA1k\x8D`\xE1\x1B\x81R`\x04\x01a\t\x9A\x91\x90a!\xE1V[\x85a\x13\xE6W`\x02T`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x19\xBAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\"K\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x19\xCEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19\xF2\x91\x90a\x1E V[`\x04UPPPPPPV[`\x03T`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x1A~W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\"K\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x1A\x92W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A\xB6\x91\x90a\x1E V[`\x06\x81\x90U`\x05T\x11\x15a\x1BAW\x7F\xB6[.\x08]}\x04\x0C1?}N\x1A\xC9\x0FY7\x02o\xEEI~\x0E$\xA7\xEF\xF1jU\xE1\xC5\xEA`\x06T`\x05Ta\x1A\xF5\x91\x90a\x1E\xF5V[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xA1`\x05T`\x06Ta\x1B\x16\x81\x83a!FV[`@Qc\xB1n7\x83`\xE0\x1B\x81R`\x04\x81\x01\x93\x90\x93R`$\x83\x01\x91\x90\x91R\x19`D\x82\x01R`d\x01a\t\x9AV[`\0`\x05T`\x06Ta\x1BS\x91\x90a\x1E\xF5V[\x90P\x80`\x07`\0\x82\x82Ta\x1Bg\x91\x90a!\x9DV[\x90\x91UPP`@Q\x81\x81R\x7F5}\x90_\x181 \x97\x97\xDFMU\xD7\x9C\\[\xF1\xD9\xF71\x1C\x97j\xFD\x05\xE1=\x88\x1E\xAB\x9B\xC8\x90` \x01`@Q\x80\x91\x03\x90\xA1`\x03T`\x06T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1C-W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\"K\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x1CAW=`\0\x80>=`\0\xFD[PPPPPV[a\x15\xF7\x82\x82`@Q`$\x01a\x1C^\x92\x91\x90a\"(V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c-\x83\x9C\xB3`\xE2\x1B\x17\x90Ra\x1D\x11V[a\x03\xA1\x81`@Q`$\x01a\x1C\xA1\x91\x90a!3V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\x10L\x13\xEB`\xE2\x1B\x17\x90Ra\x1D\x11V[a\x03\xA1\x81`@Q`$\x01a\x1C\xE6\x91\x81R` \x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c-[l\xB9`\xE0\x1B\x17\x90R[\x80Qjconsole.log` \x83\x01`\0\x80\x84\x83\x85Z\xFAPPPPPV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`\0` \x82\x84\x03\x12\x15a\x1D\x97Wa\x1D\x97a\x1D2V[P5\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x1D\xB4Wa\x1D\xB4a\x1D2V[PP\x805\x92` \x90\x91\x015\x91PV[\x80\x15\x15\x81\x14a\x03\xA1W`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a\x1D\xEDWa\x1D\xEDa\x1D2V[\x865a\x1D\xF8\x81a\x1D\xC3V[\x98` \x88\x015\x98P`@\x88\x015\x97``\x81\x015\x97P`\x80\x81\x015\x96P`\xA0\x015\x94P\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x1E5Wa\x1E5a\x1D2V[PQ\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x1EQWa\x1EQa\x1D2V[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x1EhW`\0\x80\xFD[\x93\x92PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x1E\x87Wa\x1E\x87a\x1D2V[\x83Q\x92P` \x84\x01Q\x91P`@\x84\x01Q\x90P\x92P\x92P\x92V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x1E\xCDWa\x1E\xCDa\x1E\xA0V[\x92\x91PPV[`\0\x82a\x1E\xF0WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[\x81\x81\x03\x81\x81\x11\x15a\x1E\xCDWa\x1E\xCDa\x1E\xA0V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0[\x83\x81\x10\x15a\x1F9W\x81\x81\x01Q\x83\x82\x01R` \x01a\x1F!V[PP`\0\x91\x01RV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x1F[Wa\x1F[a\x1D2V[\x84Qa\x1Ff\x81a\x1D\xC3V[\x80\x94PP` \x80\x86\x01Q\x93P`@\x80\x87\x01Q\x93P``\x87\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x1F\xDFW\x82QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x85\x90R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[\x81\x89\x01\x91P\x89`\x1F\x83\x01\x12a FW\x82QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x85\x90R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x81\xFD[\x81Q\x81\x81\x11\x15a XWa Xa\x1F\x08V[\x83Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a \x7FWa \x7Fa\x1F\x08V[\x81\x86R\x82\x81R\x8C\x87\x84\x87\x01\x01\x11\x15a \xE7W\x85QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x88\x90R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R\x93P`\x84\x84\xFD[a \xF6\x83\x88\x83\x01\x89\x88\x01a\x1F\x1EV[\x99\x9C\x98\x9BP\x96\x99PPPPPPPPV[`\0\x81Q\x80\x84Ra!\x1F\x81` \x86\x01` \x86\x01a\x1F\x1EV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x1Eh` \x83\x01\x84a!\x07V[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a!fWa!fa\x1E\xA0V[P\x92\x91PPV[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15a!\x89Wa!\x89a\x1E\xA0V[\x81\x81\x05\x83\x14\x82\x15\x17a\x1E\xCDWa\x1E\xCDa\x1E\xA0V[\x80\x82\x01\x80\x82\x11\x15a\x1E\xCDWa\x1E\xCDa\x1E\xA0V[\x84\x15\x15\x81R\x83` \x82\x01R\x82`@\x82\x01R`\x80``\x82\x01R`\0a!\xD7`\x80\x83\x01\x84a!\x07V[\x96\x95PPPPPPV[`@\x81R`\x19`@\x82\x01R\x7FDEX swap failed with data\0\0\0\0\0\0\0``\x82\x01R`\x80` \x82\x01R`\0a\x1Eh`\x80\x83\x01\x84a!\x07V[`@\x81R`\0a\";`@\x83\x01\x85a!\x07V[\x90P\x82` \x83\x01R\x93\x92PPPV\xFETarget contract does not contain";
    /// The deployed bytecode of the contract.
    pub static ATOMICV2_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct AtomicV2<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for AtomicV2<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for AtomicV2<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for AtomicV2<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for AtomicV2<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(AtomicV2)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> AtomicV2<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    ATOMICV2_ABI.clone(),
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
                ATOMICV2_ABI.clone(),
                ATOMICV2_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `XTOY` (0xf3c973cf) function
        pub fn xtoy(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([243, 201, 115, 207], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `YTOX` (0x6449fc57) function
        pub fn ytox(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([100, 73, 252, 87], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `asset` (0x38d52e0f) function
        pub fn asset(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([56, 213, 46, 15], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `cumulativeProfit` (0x85b319ff) function
        pub fn cumulative_profit(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([133, 179, 25, 255], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `exchange` (0xd2f7265a) function
        pub fn exchange(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([210, 247, 38, 90], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `findTrade` (0xb9933e84) function
        pub fn find_trade(
            &self,
            swap_x_in: bool,
            price_target: ::ethers::core::types::U256,
            lower: ::ethers::core::types::U256,
            upper: ::ethers::core::types::U256,
            epsilon: ::ethers::core::types::U256,
            max_iters: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [185, 147, 62, 132],
                    (swap_x_in, price_target, lower, upper, epsilon, max_iters),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `intermediateTokenXBalance` (0x936520c3) function
        pub fn intermediate_token_x_balance(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([147, 101, 32, 195], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `intermediateTokenYEndBalance` (0x72b98246) function
        pub fn intermediate_token_y_end_balance(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([114, 185, 130, 70], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `intermediateTokenYStartBalance` (0xfa2e5994) function
        pub fn intermediate_token_y_start_balance(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([250, 46, 89, 148], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `liquidExchange` (0x9f27ef4f) function
        pub fn liquid_exchange(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([159, 39, 239, 79], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lower_exchange_price` (0x35a99ad0) function
        pub fn lower_exchange_price(
            &self,
            input: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([53, 169, 154, 208], input)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `quote` (0x999b93af) function
        pub fn quote(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([153, 155, 147, 175], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `raise_exchange_price` (0x8a2fa54a) function
        pub fn raise_exchange_price(
            &self,
            input: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([138, 47, 165, 74], input)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `try_arbitrage_until_target_price` (0xa6b87c35) function
        pub fn try_arbitrage_until_target_price(
            &self,
            target_price: ::ethers::core::types::U256,
            max_iterations: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([166, 184, 124, 53], (target_price, max_iterations))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `Loss` event
        pub fn loss_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LossFilter> {
            self.0.event()
        }
        ///Gets the contract's `Profit` event
        pub fn profit_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ProfitFilter> {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AtomicV2Events,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for AtomicV2<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `CatastrophicSwapFailure` with signature `CatastrophicSwapFailure()` and selector `0x3203791f`
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
    #[etherror(name = "CatastrophicSwapFailure", abi = "CatastrophicSwapFailure()")]
    pub struct CatastrophicSwapFailure;
    ///Custom Error type `DexSwapFailure` with signature `DexSwapFailure(string,bytes)` and selector `0xcf42d71a`
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
    #[etherror(name = "DexSwapFailure", abi = "DexSwapFailure(string,bytes)")]
    pub struct DexSwapFailure {
        pub reason: ::std::string::String,
        pub err: ::ethers::core::types::Bytes,
    }
    ///Custom Error type `FindingTradeError` with signature `FindingTradeError(bytes)` and selector `0x1a439ed1`
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
    #[etherror(name = "FindingTradeError", abi = "FindingTradeError(bytes)")]
    pub struct FindingTradeError {
        pub err: ::ethers::core::types::Bytes,
    }
    ///Custom Error type `InsufficientApprovalY` with signature `InsufficientApprovalY(uint256,uint256)` and selector `0xda56d3c5`
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
        name = "InsufficientApprovalY",
        abi = "InsufficientApprovalY(uint256,uint256)"
    )]
    pub struct InsufficientApprovalY {
        pub allowance: ::ethers::core::types::U256,
        pub payment: ::ethers::core::types::U256,
    }
    ///Custom Error type `InsufficientBalanceX` with signature `InsufficientBalanceX(uint256,uint256)` and selector `0x0295b09c`
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
        name = "InsufficientBalanceX",
        abi = "InsufficientBalanceX(uint256,uint256)"
    )]
    pub struct InsufficientBalanceX {
        pub balance: ::ethers::core::types::U256,
        pub payment: ::ethers::core::types::U256,
    }
    ///Custom Error type `InsufficientBalanceY` with signature `InsufficientBalanceY(uint256,uint256)` and selector `0x0abe5a89`
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
        name = "InsufficientBalanceY",
        abi = "InsufficientBalanceY(uint256,uint256)"
    )]
    pub struct InsufficientBalanceY {
        pub balance: ::ethers::core::types::U256,
        pub payment: ::ethers::core::types::U256,
    }
    ///Custom Error type `NotProfitable` with signature `NotProfitable(uint256,uint256)` and selector `0x843e30ec`
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
    #[etherror(name = "NotProfitable", abi = "NotProfitable(uint256,uint256)")]
    pub struct NotProfitable {
        pub first_swap_output: ::ethers::core::types::U256,
        pub second_swap_output: ::ethers::core::types::U256,
    }
    ///Custom Error type `SimulatedSwapFailure` with signature `SimulatedSwapFailure(bool,uint256,uint256,bytes)` and selector `0x18a73118`
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
        name = "SimulatedSwapFailure",
        abi = "SimulatedSwapFailure(bool,uint256,uint256,bytes)"
    )]
    pub struct SimulatedSwapFailure {
        pub valid: bool,
        pub estimated_out: ::ethers::core::types::U256,
        pub estimated_price: ::ethers::core::types::U256,
        pub payload: ::ethers::core::types::Bytes,
    }
    ///Custom Error type `UnprofitableArbitrage` with signature `UnprofitableArbitrage(uint256,uint256,uint256)` and selector `0xb16e3783`
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
        name = "UnprofitableArbitrage",
        abi = "UnprofitableArbitrage(uint256,uint256,uint256)"
    )]
    pub struct UnprofitableArbitrage {
        pub start_y_balance: ::ethers::core::types::U256,
        pub end_y_balance: ::ethers::core::types::U256,
        pub absolute_difference: ::ethers::core::types::U256,
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
    pub enum AtomicV2Errors {
        CatastrophicSwapFailure(CatastrophicSwapFailure),
        DexSwapFailure(DexSwapFailure),
        FindingTradeError(FindingTradeError),
        InsufficientApprovalY(InsufficientApprovalY),
        InsufficientBalanceX(InsufficientBalanceX),
        InsufficientBalanceY(InsufficientBalanceY),
        NotProfitable(NotProfitable),
        SimulatedSwapFailure(SimulatedSwapFailure),
        UnprofitableArbitrage(UnprofitableArbitrage),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for AtomicV2Errors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <CatastrophicSwapFailure as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CatastrophicSwapFailure(decoded));
            }
            if let Ok(decoded) = <DexSwapFailure as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DexSwapFailure(decoded));
            }
            if let Ok(decoded) = <FindingTradeError as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FindingTradeError(decoded));
            }
            if let Ok(decoded) = <InsufficientApprovalY as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InsufficientApprovalY(decoded));
            }
            if let Ok(decoded) = <InsufficientBalanceX as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InsufficientBalanceX(decoded));
            }
            if let Ok(decoded) = <InsufficientBalanceY as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InsufficientBalanceY(decoded));
            }
            if let Ok(decoded) = <NotProfitable as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NotProfitable(decoded));
            }
            if let Ok(decoded) = <SimulatedSwapFailure as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SimulatedSwapFailure(decoded));
            }
            if let Ok(decoded) = <UnprofitableArbitrage as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UnprofitableArbitrage(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for AtomicV2Errors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::CatastrophicSwapFailure(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DexSwapFailure(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FindingTradeError(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InsufficientApprovalY(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InsufficientBalanceX(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InsufficientBalanceY(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NotProfitable(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SimulatedSwapFailure(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UnprofitableArbitrage(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for AtomicV2Errors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <CatastrophicSwapFailure as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <DexSwapFailure as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <FindingTradeError as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InsufficientApprovalY as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InsufficientBalanceX as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InsufficientBalanceY as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NotProfitable as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <SimulatedSwapFailure as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <UnprofitableArbitrage as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for AtomicV2Errors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CatastrophicSwapFailure(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DexSwapFailure(element) => ::core::fmt::Display::fmt(element, f),
                Self::FindingTradeError(element) => ::core::fmt::Display::fmt(element, f),
                Self::InsufficientApprovalY(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InsufficientBalanceX(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InsufficientBalanceY(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NotProfitable(element) => ::core::fmt::Display::fmt(element, f),
                Self::SimulatedSwapFailure(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UnprofitableArbitrage(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for AtomicV2Errors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<CatastrophicSwapFailure> for AtomicV2Errors {
        fn from(value: CatastrophicSwapFailure) -> Self {
            Self::CatastrophicSwapFailure(value)
        }
    }
    impl ::core::convert::From<DexSwapFailure> for AtomicV2Errors {
        fn from(value: DexSwapFailure) -> Self {
            Self::DexSwapFailure(value)
        }
    }
    impl ::core::convert::From<FindingTradeError> for AtomicV2Errors {
        fn from(value: FindingTradeError) -> Self {
            Self::FindingTradeError(value)
        }
    }
    impl ::core::convert::From<InsufficientApprovalY> for AtomicV2Errors {
        fn from(value: InsufficientApprovalY) -> Self {
            Self::InsufficientApprovalY(value)
        }
    }
    impl ::core::convert::From<InsufficientBalanceX> for AtomicV2Errors {
        fn from(value: InsufficientBalanceX) -> Self {
            Self::InsufficientBalanceX(value)
        }
    }
    impl ::core::convert::From<InsufficientBalanceY> for AtomicV2Errors {
        fn from(value: InsufficientBalanceY) -> Self {
            Self::InsufficientBalanceY(value)
        }
    }
    impl ::core::convert::From<NotProfitable> for AtomicV2Errors {
        fn from(value: NotProfitable) -> Self {
            Self::NotProfitable(value)
        }
    }
    impl ::core::convert::From<SimulatedSwapFailure> for AtomicV2Errors {
        fn from(value: SimulatedSwapFailure) -> Self {
            Self::SimulatedSwapFailure(value)
        }
    }
    impl ::core::convert::From<UnprofitableArbitrage> for AtomicV2Errors {
        fn from(value: UnprofitableArbitrage) -> Self {
            Self::UnprofitableArbitrage(value)
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
    #[ethevent(name = "Loss", abi = "Loss(uint256)")]
    pub struct LossFilter {
        pub loss: ::ethers::core::types::U256,
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
    #[ethevent(name = "Profit", abi = "Profit(uint256)")]
    pub struct ProfitFilter {
        pub profit: ::ethers::core::types::U256,
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
    pub enum AtomicV2Events {
        LossFilter(LossFilter),
        ProfitFilter(ProfitFilter),
    }
    impl ::ethers::contract::EthLogDecode for AtomicV2Events {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = LossFilter::decode_log(log) {
                return Ok(AtomicV2Events::LossFilter(decoded));
            }
            if let Ok(decoded) = ProfitFilter::decode_log(log) {
                return Ok(AtomicV2Events::ProfitFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for AtomicV2Events {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::LossFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProfitFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<LossFilter> for AtomicV2Events {
        fn from(value: LossFilter) -> Self {
            Self::LossFilter(value)
        }
    }
    impl ::core::convert::From<ProfitFilter> for AtomicV2Events {
        fn from(value: ProfitFilter) -> Self {
            Self::ProfitFilter(value)
        }
    }
    ///Container type for all input parameters for the `XTOY` function with signature `XTOY()` and selector `0xf3c973cf`
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
    #[ethcall(name = "XTOY", abi = "XTOY()")]
    pub struct XtoyCall;
    ///Container type for all input parameters for the `YTOX` function with signature `YTOX()` and selector `0x6449fc57`
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
    #[ethcall(name = "YTOX", abi = "YTOX()")]
    pub struct YtoxCall;
    ///Container type for all input parameters for the `asset` function with signature `asset()` and selector `0x38d52e0f`
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
    #[ethcall(name = "asset", abi = "asset()")]
    pub struct AssetCall;
    ///Container type for all input parameters for the `cumulativeProfit` function with signature `cumulativeProfit()` and selector `0x85b319ff`
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
    #[ethcall(name = "cumulativeProfit", abi = "cumulativeProfit()")]
    pub struct CumulativeProfitCall;
    ///Container type for all input parameters for the `exchange` function with signature `exchange()` and selector `0xd2f7265a`
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
    #[ethcall(name = "exchange", abi = "exchange()")]
    pub struct ExchangeCall;
    ///Container type for all input parameters for the `findTrade` function with signature `findTrade(bool,uint256,uint256,uint256,uint256,uint256)` and selector `0xb9933e84`
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
        name = "findTrade",
        abi = "findTrade(bool,uint256,uint256,uint256,uint256,uint256)"
    )]
    pub struct FindTradeCall {
        pub swap_x_in: bool,
        pub price_target: ::ethers::core::types::U256,
        pub lower: ::ethers::core::types::U256,
        pub upper: ::ethers::core::types::U256,
        pub epsilon: ::ethers::core::types::U256,
        pub max_iters: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `intermediateTokenXBalance` function with signature `intermediateTokenXBalance()` and selector `0x936520c3`
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
    #[ethcall(name = "intermediateTokenXBalance", abi = "intermediateTokenXBalance()")]
    pub struct IntermediateTokenXBalanceCall;
    ///Container type for all input parameters for the `intermediateTokenYEndBalance` function with signature `intermediateTokenYEndBalance()` and selector `0x72b98246`
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
        name = "intermediateTokenYEndBalance",
        abi = "intermediateTokenYEndBalance()"
    )]
    pub struct IntermediateTokenYEndBalanceCall;
    ///Container type for all input parameters for the `intermediateTokenYStartBalance` function with signature `intermediateTokenYStartBalance()` and selector `0xfa2e5994`
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
        name = "intermediateTokenYStartBalance",
        abi = "intermediateTokenYStartBalance()"
    )]
    pub struct IntermediateTokenYStartBalanceCall;
    ///Container type for all input parameters for the `liquidExchange` function with signature `liquidExchange()` and selector `0x9f27ef4f`
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
    #[ethcall(name = "liquidExchange", abi = "liquidExchange()")]
    pub struct LiquidExchangeCall;
    ///Container type for all input parameters for the `lower_exchange_price` function with signature `lower_exchange_price(uint256)` and selector `0x35a99ad0`
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
    #[ethcall(name = "lower_exchange_price", abi = "lower_exchange_price(uint256)")]
    pub struct LowerExchangePriceCall {
        pub input: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `quote` function with signature `quote()` and selector `0x999b93af`
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
    #[ethcall(name = "quote", abi = "quote()")]
    pub struct QuoteCall;
    ///Container type for all input parameters for the `raise_exchange_price` function with signature `raise_exchange_price(uint256)` and selector `0x8a2fa54a`
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
    #[ethcall(name = "raise_exchange_price", abi = "raise_exchange_price(uint256)")]
    pub struct RaiseExchangePriceCall {
        pub input: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `try_arbitrage_until_target_price` function with signature `try_arbitrage_until_target_price(uint256,uint256)` and selector `0xa6b87c35`
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
        name = "try_arbitrage_until_target_price",
        abi = "try_arbitrage_until_target_price(uint256,uint256)"
    )]
    pub struct TryArbitrageUntilTargetPriceCall {
        pub target_price: ::ethers::core::types::U256,
        pub max_iterations: ::ethers::core::types::U256,
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
    pub enum AtomicV2Calls {
        Xtoy(XtoyCall),
        Ytox(YtoxCall),
        Asset(AssetCall),
        CumulativeProfit(CumulativeProfitCall),
        Exchange(ExchangeCall),
        FindTrade(FindTradeCall),
        IntermediateTokenXBalance(IntermediateTokenXBalanceCall),
        IntermediateTokenYEndBalance(IntermediateTokenYEndBalanceCall),
        IntermediateTokenYStartBalance(IntermediateTokenYStartBalanceCall),
        LiquidExchange(LiquidExchangeCall),
        LowerExchangePrice(LowerExchangePriceCall),
        Quote(QuoteCall),
        RaiseExchangePrice(RaiseExchangePriceCall),
        TryArbitrageUntilTargetPrice(TryArbitrageUntilTargetPriceCall),
    }
    impl ::ethers::core::abi::AbiDecode for AtomicV2Calls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <XtoyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Xtoy(decoded));
            }
            if let Ok(decoded) = <YtoxCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Ytox(decoded));
            }
            if let Ok(decoded) = <AssetCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Asset(decoded));
            }
            if let Ok(decoded) = <CumulativeProfitCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CumulativeProfit(decoded));
            }
            if let Ok(decoded) = <ExchangeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Exchange(decoded));
            }
            if let Ok(decoded) = <FindTradeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FindTrade(decoded));
            }
            if let Ok(decoded) = <IntermediateTokenXBalanceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IntermediateTokenXBalance(decoded));
            }
            if let Ok(decoded) = <IntermediateTokenYEndBalanceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IntermediateTokenYEndBalance(decoded));
            }
            if let Ok(decoded) = <IntermediateTokenYStartBalanceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IntermediateTokenYStartBalance(decoded));
            }
            if let Ok(decoded) = <LiquidExchangeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LiquidExchange(decoded));
            }
            if let Ok(decoded) = <LowerExchangePriceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LowerExchangePrice(decoded));
            }
            if let Ok(decoded) = <QuoteCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Quote(decoded));
            }
            if let Ok(decoded) = <RaiseExchangePriceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RaiseExchangePrice(decoded));
            }
            if let Ok(decoded) = <TryArbitrageUntilTargetPriceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TryArbitrageUntilTargetPrice(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for AtomicV2Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Xtoy(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Ytox(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Asset(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CumulativeProfit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Exchange(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FindTrade(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IntermediateTokenXBalance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IntermediateTokenYEndBalance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IntermediateTokenYStartBalance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LiquidExchange(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LowerExchangePrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Quote(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RaiseExchangePrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TryArbitrageUntilTargetPrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for AtomicV2Calls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Xtoy(element) => ::core::fmt::Display::fmt(element, f),
                Self::Ytox(element) => ::core::fmt::Display::fmt(element, f),
                Self::Asset(element) => ::core::fmt::Display::fmt(element, f),
                Self::CumulativeProfit(element) => ::core::fmt::Display::fmt(element, f),
                Self::Exchange(element) => ::core::fmt::Display::fmt(element, f),
                Self::FindTrade(element) => ::core::fmt::Display::fmt(element, f),
                Self::IntermediateTokenXBalance(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IntermediateTokenYEndBalance(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IntermediateTokenYStartBalance(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LiquidExchange(element) => ::core::fmt::Display::fmt(element, f),
                Self::LowerExchangePrice(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Quote(element) => ::core::fmt::Display::fmt(element, f),
                Self::RaiseExchangePrice(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TryArbitrageUntilTargetPrice(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<XtoyCall> for AtomicV2Calls {
        fn from(value: XtoyCall) -> Self {
            Self::Xtoy(value)
        }
    }
    impl ::core::convert::From<YtoxCall> for AtomicV2Calls {
        fn from(value: YtoxCall) -> Self {
            Self::Ytox(value)
        }
    }
    impl ::core::convert::From<AssetCall> for AtomicV2Calls {
        fn from(value: AssetCall) -> Self {
            Self::Asset(value)
        }
    }
    impl ::core::convert::From<CumulativeProfitCall> for AtomicV2Calls {
        fn from(value: CumulativeProfitCall) -> Self {
            Self::CumulativeProfit(value)
        }
    }
    impl ::core::convert::From<ExchangeCall> for AtomicV2Calls {
        fn from(value: ExchangeCall) -> Self {
            Self::Exchange(value)
        }
    }
    impl ::core::convert::From<FindTradeCall> for AtomicV2Calls {
        fn from(value: FindTradeCall) -> Self {
            Self::FindTrade(value)
        }
    }
    impl ::core::convert::From<IntermediateTokenXBalanceCall> for AtomicV2Calls {
        fn from(value: IntermediateTokenXBalanceCall) -> Self {
            Self::IntermediateTokenXBalance(value)
        }
    }
    impl ::core::convert::From<IntermediateTokenYEndBalanceCall> for AtomicV2Calls {
        fn from(value: IntermediateTokenYEndBalanceCall) -> Self {
            Self::IntermediateTokenYEndBalance(value)
        }
    }
    impl ::core::convert::From<IntermediateTokenYStartBalanceCall> for AtomicV2Calls {
        fn from(value: IntermediateTokenYStartBalanceCall) -> Self {
            Self::IntermediateTokenYStartBalance(value)
        }
    }
    impl ::core::convert::From<LiquidExchangeCall> for AtomicV2Calls {
        fn from(value: LiquidExchangeCall) -> Self {
            Self::LiquidExchange(value)
        }
    }
    impl ::core::convert::From<LowerExchangePriceCall> for AtomicV2Calls {
        fn from(value: LowerExchangePriceCall) -> Self {
            Self::LowerExchangePrice(value)
        }
    }
    impl ::core::convert::From<QuoteCall> for AtomicV2Calls {
        fn from(value: QuoteCall) -> Self {
            Self::Quote(value)
        }
    }
    impl ::core::convert::From<RaiseExchangePriceCall> for AtomicV2Calls {
        fn from(value: RaiseExchangePriceCall) -> Self {
            Self::RaiseExchangePrice(value)
        }
    }
    impl ::core::convert::From<TryArbitrageUntilTargetPriceCall> for AtomicV2Calls {
        fn from(value: TryArbitrageUntilTargetPriceCall) -> Self {
            Self::TryArbitrageUntilTargetPrice(value)
        }
    }
    ///Container type for all return fields from the `XTOY` function with signature `XTOY()` and selector `0xf3c973cf`
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
    pub struct XtoyReturn(pub bool);
    ///Container type for all return fields from the `YTOX` function with signature `YTOX()` and selector `0x6449fc57`
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
    pub struct YtoxReturn(pub bool);
    ///Container type for all return fields from the `asset` function with signature `asset()` and selector `0x38d52e0f`
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
    pub struct AssetReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `cumulativeProfit` function with signature `cumulativeProfit()` and selector `0x85b319ff`
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
    pub struct CumulativeProfitReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `exchange` function with signature `exchange()` and selector `0xd2f7265a`
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
    pub struct ExchangeReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `findTrade` function with signature `findTrade(bool,uint256,uint256,uint256,uint256,uint256)` and selector `0xb9933e84`
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
    pub struct FindTradeReturn {
        pub amount_in: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `intermediateTokenXBalance` function with signature `intermediateTokenXBalance()` and selector `0x936520c3`
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
    pub struct IntermediateTokenXBalanceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `intermediateTokenYEndBalance` function with signature `intermediateTokenYEndBalance()` and selector `0x72b98246`
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
    pub struct IntermediateTokenYEndBalanceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `intermediateTokenYStartBalance` function with signature `intermediateTokenYStartBalance()` and selector `0xfa2e5994`
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
    pub struct IntermediateTokenYStartBalanceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `liquidExchange` function with signature `liquidExchange()` and selector `0x9f27ef4f`
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
    pub struct LiquidExchangeReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `quote` function with signature `quote()` and selector `0x999b93af`
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
    pub struct QuoteReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `try_arbitrage_until_target_price` function with signature `try_arbitrage_until_target_price(uint256,uint256)` and selector `0xa6b87c35`
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
    pub struct TryArbitrageUntilTargetPriceReturn {
        pub amount_in: ::ethers::core::types::U256,
    }
}
