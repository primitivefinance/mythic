pub use atomic_v2::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types
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
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("XTOY"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bool"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("YTOX"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("YTOX"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bool"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("asset"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("asset"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("cumulativeProfit"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("cumulativeProfit"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("exchange"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("exchange"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("findTrade"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
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
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("lower"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("upper"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("epsilon"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("max_iters"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("amountIn"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("intermediateTokenXBalance"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("intermediateTokenXBalance",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("intermediateTokenYEndBalance"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("intermediateTokenYEndBalance",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("intermediateTokenYStartBalance"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("intermediateTokenYStartBalance",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("liquidExchange"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("liquidExchange"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("lower_exchange_price"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("lower_exchange_price",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("input_y_amount"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("profitFinder"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("profitFinder"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("contract ProfitFinder"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("quote"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("quote"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("raise_exchange_price"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("raise_exchange_price",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("input_y_amount"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("searchLowerPrice"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("searchLowerPrice"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("max_iters"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("epsilon"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amountIn"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("profit"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("searchRaisePrice"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("searchRaisePrice"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("max_iters"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("epsilon"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amountIn"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("profit"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("tradePacket"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("tradePacket"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("block_number"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("block_timestamp"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("raisePrice"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bool"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amountIn"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("profit"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("try_arbitrage_until_target_price"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("try_arbitrage_until_target_price",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("target_price"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("max_iterations"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("amountIn"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("try_lower_exchange_price"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("try_lower_exchange_price",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("input_y_amount"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("try_raise_exchange_price"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("try_raise_exchange_price",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("input_y_amount"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("Loss"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("Loss"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("loss"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            indexed: false,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Profit"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("Profit"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("profit"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            indexed: false,
                        },],
                        anonymous: false,
                    },],
                ),
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("AttemptedProfit"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("AttemptedProfit"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("profit"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("int256"),
                            ),
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("CatastrophicSwapFailure"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("CatastrophicSwapFailure",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("DexSwapFailure"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
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
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("FindingTradeError"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("FindingTradeError"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("err"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes"),
                            ),
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InsufficientApprovalY"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InsufficientApprovalY",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("allowance"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("payment"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InsufficientBalanceX"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InsufficientBalanceX",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("balance"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("payment"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InsufficientBalanceY"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InsufficientBalanceY",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("balance"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("payment"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("MaximizingProfitTrade"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("MaximizingProfitTrade",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("trade"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("profit"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NotProfitable"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("NotProfitable"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("first_swap_output"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("second_swap_output",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SimulatedSwapFailure"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("SimulatedSwapFailure",),
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
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("estimatedPrice"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
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
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("UnprofitableArbitrage"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("UnprofitableArbitrage",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("start_y_balance"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("end_y_balance"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("absolute_difference",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                    },],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static ATOMICV2_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R`\t\x80Ta\xFF\xFF\x19\x16`\x01\x17\x90U4\x80\x15b\0\0lW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`@Qb\0<\x838\x03\x80b\0<\x83\x839\x81\x01`@\x81\x90Rb\0\0\x8F\x91b\0\x01[V[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x80\x87\x16`\x01`\x01`\xA0\x1B\x03\x19\x92\x83\x16\x17\x90\x92U`\x01\x80T\x86\x84\x16\x90\x83\x16\x17\x90U`\x02\x80T\x85\x84\x16\x90\x83\x16\x17\x90U`\x03\x80T\x92\x84\x16\x92\x90\x91\x16\x91\x90\x91\x17\x90U`@Qb\0\0\xE7\x90b\0\x010V[`@Q\x80\x91\x03\x90`\0\xF0\x80\x15\x80\x15b\0\x01\x04W=`\0\x80>=`\0\xFD[P`\x04\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UPb\0\x02\x03\x92PPPV[a\x12\xC8\x80b\0)\xBB\x839\x01\x90V[\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x01VW`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15b\0\x01\xBDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[b\0\x01\xC8\x85b\0\x01>V[\x93Pb\0\x01\xD8` \x86\x01b\0\x01>V[\x92Pb\0\x01\xE8`@\x86\x01b\0\x01>V[\x91Pb\0\x01\xF8``\x86\x01b\0\x01>V[\x90P\x92\x95\x91\x94P\x92PV[a'\xA8\x80b\0\x02\x13`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x01yW`\x005`\xE0\x1C\x80c\x99\x9B\x93\xAF\x11a\0\xFAW\x80c\xD2\xF7&Z\x11a\0\xBEW\x80c\xD2\xF7&Z\x14a\x03gW\x80c\xD7\x82=\xF7\x14a\x03zW\x80c\xEF\xA3Lx\x14a\x03\x8DW\x80c\xF3\xC9s\xCF\x14a\x03\xA0W\x80c\xFA.Y\x94\x14a\x03\xADWa\x01yV[\x80c\x99\x9B\x93\xAF\x14a\x02\xCFW\x80c\x9F'\xEFO\x14a\x02\xE2W\x80c\xA1S\x87\x89\x14a\x02\xF5W\x80c\xA6\xB8|5\x14a\x03AW\x80c\xB9\x93>\x84\x14a\x03TWa\x01yV[\x80cr\xB9\x82F\x11a\x01AW\x80cr\xB9\x82F\x14a\x02\x80W\x80c\x85\xB3\x19\xFF\x14a\x02\x97W\x80c\x8A/\xA5J\x14a\x02\xA0W\x80c\x93e \xC3\x14a\x02\xB3W\x80c\x98\xD8\x83M\x14a\x02\xBCWa\x01yV[\x80c\x0F\xF5s\x1D\x14a\x01\xDEW\x80c5(\x93\0\x14a\x02\x0BW\x80c5\xA9\x9A\xD0\x14a\x02\x1EW\x80c8\xD5.\x0F\x14a\x023W\x80cdI\xFCW\x14a\x02^W[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x01\xF1a\x01\xEC6`\x04a\"\x98V[a\x03\xB6V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01[`@Q\x80\x91\x03\x90\xF3[a\x01\xF1a\x02\x196`\x04a\"\x98V[a\x04\xF8V[a\x021a\x02,6`\x04a\"\xBDV[a\x066V[\0[`\x02Ta\x02F\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\x02V[`\tTa\x02p\x90a\x01\0\x90\x04`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x02\x02V[a\x02\x89`\x07T\x81V[`@Q\x90\x81R` \x01a\x02\x02V[a\x02\x89`\x08T\x81V[a\x021a\x02\xAE6`\x04a\"\xBDV[a\x07'V[a\x02\x89`\x05T\x81V[`\x04Ta\x02F\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x03Ta\x02F\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x01Ta\x02F\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\nT`\x0BT`\x0CT`\rT`\x0ETa\x03\x12\x94\x93\x92`\xFF\x16\x91\x90\x85V[`@Qa\x02\x02\x95\x94\x93\x92\x91\x90\x94\x85R` \x85\x01\x93\x90\x93R\x90\x15\x15`@\x84\x01R``\x83\x01R`\x80\x82\x01R`\xA0\x01\x90V[a\x02\x89a\x03O6`\x04a\"\x98V[a\x08\rV[a\x02\x89a\x03b6`\x04a\"\xE7V[a\x0C\xCBV[`\0Ta\x02F\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x021a\x03\x886`\x04a\"\xBDV[a\x12\xD9V[a\x021a\x03\x9B6`\x04a\"\xBDV[a\x14\x16V[`\tTa\x02p\x90`\xFF\x16\x81V[a\x02\x89`\x06T\x81V[`\x04\x80T`@Qc\x0F\xF5s\x1D`\xE0\x1B\x81R\x91\x82\x01\x84\x90R`$\x82\x01\x83\x90R`\0\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\x0F\xF5s\x1D\x90`D\x01`@\x80Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x04CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a'\x88\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x92PPP\x80\x15a\x04sWP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\x04p\x91\x81\x01\x90a#6V[`\x01[a\x04\xCDW=\x80\x80\x15a\x04\xA1W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x04\xA6V[``\x91P[P`\x04\x81\x01\x90P\x80\x80` \x01\x90Q\x81\x01\x90a\x04\xC1\x91\x90a#6V[\x90\x93P\x91Pa\x04\xD0\x90PV[PP[C`\nUB`\x0BU`\x0C\x80T`\xFF\x19\x16`\x01\x17\x90U`\r\x82\x90U`\x0E\x81\x90U\x90\x93\x90\x92P\x90PV[`\x04\x80T`@Qb5(\x93`\xE8\x1B\x81R\x91\x82\x01\x84\x90R`$\x82\x01\x83\x90R`\0\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c5(\x93\0\x90`D\x01`@\x80Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x05\x84W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a'\x88\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x92PPP\x80\x15a\x05\xB4WP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\x05\xB1\x91\x81\x01\x90a#6V[`\x01[a\x06\x0EW=\x80\x80\x15a\x05\xE2W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x05\xE7V[``\x91P[P`\x04\x81\x01\x90P\x80\x80` \x01\x90Q\x81\x01\x90a\x06\x02\x91\x90a#6V[\x90\x93P\x91Pa\x06\x11\x90PV[PP[C`\nUB`\x0BU`\x0C\x80T`\xFF\x19\x16\x90U`\r\x82\x90U`\x0E\x81\x90U\x90\x93\x90\x92P\x90PV[a\x06?\x81a\x14mV[`\tTa\x06T\x90a\x01\0\x90\x04`\xFF\x16\x82a\x17\xAAV[`\tT`\x02T`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01Ra\x07\x1C\x92`\xFF\x16\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01[` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x06\xDFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a'\x88\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x06\xF3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x17\x91\x90a#]V[a\x1B\rV[a\x07$a\x1F\x0FV[PV[a\x070\x81a\x14mV[`\tTa\x07E\x90a\x01\0\x90\x04`\xFF\x16\x82a\x1B\rV[`\tT`\x02T`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01Ra\x07\x1C\x92`\xFF\x16\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01[` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x07\xD0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a'\x88\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x07\xE4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\x08\x91\x90a#]V[a\x17\xAAV[`\0\x80T`@\x80QcB\xC9\x88[`\xE1\x1B\x81R\x90Q\x83\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x85\x93\x10\xB6\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\x08\x90W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a'\x88\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x08\xA4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xC8\x91\x90a#]V[\x90P`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cg\xE8(\xBF`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\tWW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a'\x88\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\tkW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\x8F\x91\x90a#yV[`\x01`\x01`\xA0\x1B\x03\x16c\xC1\xE0\x04;`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01```@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\n\x05W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a'\x88\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\n\x19W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n=\x91\x90a#\xACV[PP\x90P`\0\x80`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEB\xAD\xEF\x01`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01```@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\n\xD1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a'\x88\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\n\xE5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\t\x91\x90a#\xACV[\x92P\x92P\x92P\x87\x85\x10\x15a\x0B\xCFW`\n`\0\x82a\x0B.\x85g\r\xE0\xB6\xB3\xA7d\0\0a#\xF3V[a\x0B8\x91\x90a$\x10V[a\x0BB\x90\x87a$2V[\x90Pa\x0BO`\n\x82a$2V[\x90Pa\x0B~`@Q\x80`@\x01`@R\x80`\x0B\x81R` \x01j6\xB4\xB7 \xB6\xB7\xBA\xB7:$\xB7`\xA9\x1B\x81RP\x83a!ZV[a\x0B\xAB`@Q\x80`@\x01`@R\x80`\x0B\x81R` \x01j6\xB0\xBC \xB6\xB7\xBA\xB7:$\xB7`\xA9\x1B\x81RP\x82a!ZV[`\tTa\x0B\xC6\x90a\x01\0\x90\x04`\xFF\x16\x8B\x84\x84`\x05`\x03a\x0C\xCBV[\x97PPPa\x0C\xC0V[`@\x80Q\x80\x82\x01\x90\x91R`\x0F\x81Rn!\xB7\xB6\xB8:\xBA2\xB2\x106\xB0\xBC\x104\xB7`\x89\x1B` \x82\x01R`\n\x90a\x0C0\x90\x83a\x0C\x0F\x87g\r\xE0\xB6\xB3\xA7d\0\0a#\xF3V[a\x0C\x19\x91\x90a$\x10V[a\x0C+\x90g\r\xE0\xB6\xB3\xA7d\0\0a$2V[a!ZV[`\0a\x0C<\x85\x84a$2V[\x90Pa\x0CI`\n\x82a$2V[\x90Pa\x0Cx`@Q\x80`@\x01`@R\x80`\x0B\x81R` \x01j6\xB4\xB7 \xB6\xB7\xBA\xB7:$\xB7`\xA9\x1B\x81RP\x83a!ZV[a\x0C\xA5`@Q\x80`@\x01`@R\x80`\x0B\x81R` \x01j6\xB0\xBC \xB6\xB7\xBA\xB7:$\xB7`\xA9\x1B\x81RP\x82a!ZV[`\tTa\x0C\xBB\x90`\xFF\x16\x8B\x84\x84`\x05`\x03a\x0C\xCBV[\x97PPP[PPPPP\x92\x91PPV[`\0\x83\x85\x10a\r!W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7Flower must be less than upper\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x80T`@Qc\x02\xB0\xCC\r`\xE2\x1B\x81R\x89\x15\x15`\x04\x82\x01R`$\x81\x01\x88\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\n\xC304\x90`D\x01`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\r\xACW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a'\x88\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x92PPP\x80\x15a\r\xE1WP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\r\xDE\x91\x90\x81\x01\x90a$\x7FV[`\x01[a\x0EaW=\x80\x80\x15a\x0E\x0FW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x0E\x14V[``\x91P[Pa\x0EF`@Q\x80`@\x01`@R\x80`\x10\x81R` \x01o67\xBB\xB2\xB9(94\xB1\xB2\x902\xB997\xB9`\x81\x1B\x81RPa!\x9FV[\x80`@Qc\x1AC\x9E\xD1`\xE0\x1B\x81R`\x04\x01a\r\x18\x91\x90a&pV[P`\0\x80T`@Qc\x02\xB0\xCC\r`\xE2\x1B\x81R\x8D\x15\x15`\x04\x82\x01R`$\x81\x01\x8B\x90R\x92\x95P\x90\x93P`\x01`\x01`\xA0\x1B\x03\x16\x91Pc\n\xC304\x90`D\x01`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0E\xF2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a'\x88\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x92PPP\x80\x15a\x0F'WP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0F$\x91\x90\x81\x01\x90a$\x7FV[`\x01[a\x0F\x8CW=\x80\x80\x15a\x0FUW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x0FZV[``\x91P[Pa\x0EF`@Q\x80`@\x01`@R\x80`\x10\x81R` \x01o:\xB882\xB9(94\xB1\xB2\x902\xB997\xB9`\x81\x1B\x81RPa!\x9FV[P\x92PPPa\x0F\xBD`@Q\x80`@\x01`@R\x80`\n\x81R` \x01ilowerPrice`\xB0\x1B\x81RP\x83a!ZV[a\x0F\xE9`@Q\x80`@\x01`@R\x80`\n\x81R` \x01iupperPrice`\xB0\x1B\x81RP\x82a!ZV[a\x10\x15`@Q\x80`@\x01`@R\x80`\n\x81R` \x01itargtPrice`\xB0\x1B\x81RP\x89a!ZV[`\0a\x10!\x89\x84a&\x83V[\x90P`\0a\x10/\x8A\x84a&\x83V[\x90Pa\x10:\x82a!\xE2V[a\x10C\x81a!\xE2V[`\0a\x10O\x82\x84a&\xAAV[\x12a\x10\x9CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7Froot must be between bounds\0\0\0\0\0`D\x82\x01R`d\x01a\r\x18V[`\0a\x10\xA8\x8A\x8Aa$2V[\x90P`\0[`\x02a\x10\xB9\x8B\x8Da&\xDAV[a\x10\xC3\x91\x90a$\x10V[\x96P`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\n\xC304\x8F\x8A`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x11\x14\x92\x91\x90\x91\x15\x15\x82R` \x82\x01R`@\x01\x90V[`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x11jW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a'\x88\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x92PPP\x80\x15a\x11\x9FWP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x11\x9C\x91\x90\x81\x01\x90a$\x7FV[`\x01[a\x11\xD9W=\x80\x80\x15a\x11\xCDW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x11\xD2V[``\x91P[PPa\x11\xDFV[P\x92PPP[a\x12\t`@Q\x80`@\x01`@R\x80`\x08\x81R` \x01g0\xB6\xB7\xBA\xB7:$\xB7`\xC1\x1B\x81RP\x89a!ZV[a\x127`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01ktarget price`\xA0\x1B\x81RP\x8Ea!ZV[a\x12e`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01kresult price`\xA0\x1B\x81RP\x82a!ZV[`\0a\x12q\x8E\x83a&\x83V[\x90Pa\x12|\x81a!\xE2V[`\0a\x12\x88\x87\x83a&\xAAV[\x13a\x12\x95W\x88\x9BPa\x12\x9CV[\x88\x9CP\x80\x95P[a\x12\xA6\x8D\x8Da$2V[\x93Pa\x12\xB3`\x01\x84a&\xDAV[\x92PPP\x88\x82\x11\x80\x15a\x12\xC5WP\x87\x81\x10[a\x10\xADWPPPPPP\x96\x95PPPPPPV[a\x12\xE2\x81a\x14mV[`\tTa\x12\xF7\x90a\x01\0\x90\x04`\xFF\x16\x82a\x1B\rV[`\tT`\x02T`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01Ra\x130\x92`\xFF\x16\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01a\x07zV[`\x03T`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x13\xB1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a'\x88\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x13\xC5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\xE9\x91\x90a#]V[`\x07\x81\x90U`\x06Ta\x13\xFA\x91a&\x83V[`@QcB\xD5\xD4o`\xE1\x1B\x81R`\x04\x01a\r\x18\x91\x81R` \x01\x90V[a\x14\x1F\x81a\x14mV[`\tTa\x144\x90a\x01\0\x90\x04`\xFF\x16\x82a\x17\xAAV[`\tT`\x02T`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01Ra\x130\x92`\xFF\x16\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01a\x06\x89V[`\x03T`@Qcp\xA0\x821`\xE0\x1B\x81R3`\x04\x82\x01R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x14\xEFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a'\x88\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x15\x03W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15'\x91\x90a#]V[\x90P\x81\x81\x10\x15a\x15TW`@Qc\n\xBEZ\x89`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x81\x01\x83\x90R`D\x01a\r\x18V[`\x03T`@Qcn\xB1v\x9F`\xE1\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xDDb\xED>\x90`D\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x15\xDCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a'\x88\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x15\xF0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\x14\x91\x90a#]V[\x90P\x82\x81\x10\x15a\x16AW`@Qc\xDAV\xD3\xC5`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x81\x01\x84\x90R`D\x01a\r\x18V[`\x03T`@Qc#\xB8r\xDD`\xE0\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x85\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90`d\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x16\xD1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a'\x88\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x16\xE5W=`\0\x80>=`\0\xFD[PP`\x03T`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x92Pcp\xA0\x821\x91P`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x17jW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a'\x88\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x17~W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17\xA2\x91\x90a#]V[`\x06UPPPV[\x81\x15a\x19\0W`\x02T`\x01T`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x84\x90R\x91\x16\x90c\t^\xA7\xB3\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x18>W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a'\x88\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x18RW=`\0\x80>=`\0\xFD[PP`\x01T`\x02T`@Qc\xD0\x04\xF0\xF7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x86\x90R\x91\x16\x92Pc\xD0\x04\xF0\xF7\x91P`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x18\xE4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a'\x88\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x18\xF8W=`\0\x80>=`\0\xFD[PPPPPPV[`\x03T`\x01T`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x84\x90R\x91\x16\x90c\t^\xA7\xB3\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x19\x8EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a'\x88\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x19\xA2W=`\0\x80>=`\0\xFD[PP`\x01T`\x03T`@Qc\xD0\x04\xF0\xF7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x86\x90R\x91\x16\x92Pc\xD0\x04\xF0\xF7\x91P`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1A4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a'\x88\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x1AHW=`\0\x80>=`\0\xFD[PP`\x02T`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x92Pcp\xA0\x821\x91P`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x1A\xCDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a'\x88\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x1A\xE1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B\x05\x91\x90a#]V[`\x05U[PPV[\x81\x15a\x1B\xBEW`\x02T`\0T`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x84\x90R\x91\x16\x90c\t^\xA7\xB3\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1B\xA1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a'\x88\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x1B\xB5W=`\0\x80>=`\0\xFD[PPPPa\x1CeV[`\x03T`\0T`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x84\x90R\x91\x16\x90c\t^\xA7\xB3\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1CLW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a'\x88\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x1C`W=`\0\x80>=`\0\xFD[PPPP[`\0\x80T`@Qc\x02\xB0\xCC\r`\xE2\x1B\x81R\x84\x15\x15`\x04\x82\x01R`$\x81\x01\x84\x90R\x82\x91\x82\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\n\xC304\x90`D\x01`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x1C\xF4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a'\x88\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x1D\x08W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1D0\x91\x90\x81\x01\x90a$\x7FV[\x93P\x93P\x93P\x93P\x83a\x1D^W\x83\x83\x83\x83`@Qc\x03\x14\xE6#`\xE3\x1B\x81R`\x04\x01a\r\x18\x94\x93\x92\x91\x90a&\xEDV[`\0T`@Qc1>\xEA\xB5`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cb}\xD5j\x90a\x1D\x8E\x90\x84\x90`\x04\x01a&pV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1D\xE6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a'\x88\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x92PPP\x80\x15a\x1D\xF7WP`\x01[a\x1EFW=\x80\x80\x15a\x1E%W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x1E*V[``\x91P[P\x80`@Qcg\xA1k\x8D`\xE1\x1B\x81R`\x04\x01a\r\x18\x91\x90a'\x1EV[\x85a\x18\xF8W`\x02T`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x1E\xCCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a'\x88\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x1E\xE0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1F\x04\x91\x90a#]V[`\x05UPPPPPPV[`\x03T`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x1F\x90W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a'\x88\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x1F\xA4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1F\xC8\x91\x90a#]V[`\x07\x81\x90U`\x06T\x11\x15a SW\x7F\xB6[.\x08]}\x04\x0C1?}N\x1A\xC9\x0FY7\x02o\xEEI~\x0E$\xA7\xEF\xF1jU\xE1\xC5\xEA`\x07T`\x06Ta \x07\x91\x90a$2V[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xA1`\x06T`\x07Ta (\x81\x83a&\x83V[`@Qc\xB1n7\x83`\xE0\x1B\x81R`\x04\x81\x01\x93\x90\x93R`$\x83\x01\x91\x90\x91R\x19`D\x82\x01R`d\x01a\r\x18V[`\0`\x06T`\x07Ta e\x91\x90a$2V[\x90P\x80`\x08`\0\x82\x82Ta y\x91\x90a&\xDAV[\x90\x91UPP`@Q\x81\x81R\x7F5}\x90_\x181 \x97\x97\xDFMU\xD7\x9C\\[\xF1\xD9\xF71\x1C\x97j\xFD\x05\xE1=\x88\x1E\xAB\x9B\xC8\x90` \x01`@Q\x80\x91\x03\x90\xA1`\x03T`\x07T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a!?W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a'\x88\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a!SW=`\0\x80>=`\0\xFD[PPPPPV[a\x1B\t\x82\x82`@Q`$\x01a!p\x92\x91\x90a'eV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c-\x83\x9C\xB3`\xE2\x1B\x17\x90Ra\"#V[a\x07$\x81`@Q`$\x01a!\xB3\x91\x90a&pV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\x10L\x13\xEB`\xE2\x1B\x17\x90Ra\"#V[a\x07$\x81`@Q`$\x01a!\xF8\x91\x81R` \x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c-[l\xB9`\xE0\x1B\x17\x90R[a\x07$\x81\x80Qjconsole.log` \x83\x01`\0\x80\x84\x83\x85Z\xFAPPPPPV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\"\xAEWa\"\xAEa\"HV[PP\x805\x92` \x90\x91\x015\x91PV[`\0` \x82\x84\x03\x12\x15a\"\xD2Wa\"\xD2a\"HV[P5\x91\x90PV[\x80\x15\x15\x81\x14a\x07$W`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a#\x03Wa#\x03a\"HV[\x865a#\x0E\x81a\"\xD9V[\x98` \x88\x015\x98P`@\x88\x015\x97``\x81\x015\x97P`\x80\x81\x015\x96P`\xA0\x015\x94P\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a#LWa#La\"HV[PP\x80Q` \x90\x91\x01Q\x90\x92\x90\x91PV[`\0` \x82\x84\x03\x12\x15a#rWa#ra\"HV[PQ\x91\x90PV[`\0` \x82\x84\x03\x12\x15a#\x8EWa#\x8Ea\"HV[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a#\xA5W`\0\x80\xFD[\x93\x92PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a#\xC4Wa#\xC4a\"HV[\x83Q\x92P` \x84\x01Q\x91P`@\x84\x01Q\x90P\x92P\x92P\x92V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a$\nWa$\na#\xDDV[\x92\x91PPV[`\0\x82a$-WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[\x81\x81\x03\x81\x81\x11\x15a$\nWa$\na#\xDDV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0[\x83\x81\x10\x15a$vW\x81\x81\x01Q\x83\x82\x01R` \x01a$^V[PP`\0\x91\x01RV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a$\x98Wa$\x98a\"HV[\x84Qa$\xA3\x81a\"\xD9V[\x80\x94PP` \x80\x86\x01Q\x93P`@\x80\x87\x01Q\x93P``\x87\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a%\x1CW\x82QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x85\x90R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[\x81\x89\x01\x91P\x89`\x1F\x83\x01\x12a%\x83W\x82QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x85\x90R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x81\xFD[\x81Q\x81\x81\x11\x15a%\x95Wa%\x95a$EV[\x83Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a%\xBCWa%\xBCa$EV[\x81\x86R\x82\x81R\x8C\x87\x84\x87\x01\x01\x11\x15a&$W\x85QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x88\x90R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R\x93P`\x84\x84\xFD[a&3\x83\x88\x83\x01\x89\x88\x01a$[V[\x99\x9C\x98\x9BP\x96\x99PPPPPPPPV[`\0\x81Q\x80\x84Ra&\\\x81` \x86\x01` \x86\x01a$[V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a#\xA5` \x83\x01\x84a&DV[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a&\xA3Wa&\xA3a#\xDDV[P\x92\x91PPV[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15a&\xC6Wa&\xC6a#\xDDV[\x81\x81\x05\x83\x14\x82\x15\x17a$\nWa$\na#\xDDV[\x80\x82\x01\x80\x82\x11\x15a$\nWa$\na#\xDDV[\x84\x15\x15\x81R\x83` \x82\x01R\x82`@\x82\x01R`\x80``\x82\x01R`\0a'\x14`\x80\x83\x01\x84a&DV[\x96\x95PPPPPPV[`@\x81R`\x19`@\x82\x01R\x7FDEX swap failed with data\0\0\0\0\0\0\0``\x82\x01R`\x80` \x82\x01R`\0a#\xA5`\x80\x83\x01\x84a&DV[`@\x81R`\0a'x`@\x83\x01\x85a&DV[\x90P\x82` \x83\x01R\x93\x92PPPV\xFETarget contract does not contain`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x163\x17\x90Ua\x12I\x80a\0\x7F`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\0\x8EW`\x005`\xE0\x1C\x80c\x0F\xF5s\x1D\x14a\0\xF3W\x80c5(\x93\0\x14a\x01 W\x80c\x91\xE6\xECB\x14a\x013W[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x01\x06a\x01\x016`\x04a\x10\x94V[a\x01^V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01[`@Q\x80\x91\x03\x90\xF3[a\x01\x06a\x01.6`\x04a\x10\x94V[a\x08`V[`\0Ta\x01F\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\x17V[`\0\x80a\x03\xE8\x83\x10a\x01\xB7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7Fepsilon must be less than 1000\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[P`\x01`\xFF\x1B\x90P`\x01\x80g\r\xE0\xB6\xB3\xA7d\0\0`\0\x80a\x01\xD8\x84\x84a\x10\xCFV[\x90P`\0\x80[`\x03a\x01\xEA\x87\x87a\x10\xCFV[a\x01\xF4\x91\x90a\x10\xE8V[a\x01\xFE\x90\x87a\x11\nV[\x91P`\x03a\x02\x0C\x87\x87a\x10\xCFV[a\x02\x16\x91\x90a\x10\xE8V[a\x02 \x90\x86a\x10\xCFV[\x90P`\0\x80T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x99\x9B\x93\xAF`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x02\xACW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x12\x04\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x02\xC0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\xE4\x91\x90a\x11\x1DV[`\x01`\x01`\xA0\x1B\x03\x16c@\xC1\x0F\x190a\x02\xFD\x84\x86a\x11\nV[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x04\x83\x01R`$\x82\x01R`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x03\x81W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x12\x04\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x03\x95W=`\0\x80>=`\0\xFD[PPPP`\0\x80T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x99\x9B\x93\xAF`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x04#W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x12\x04\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x047W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04[\x91\x90a\x11\x1DV[`\0T`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x91c\t^\xA7\xB3\x91\x16a\x04{\x84\x86a\x11\nV[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x04\x83\x01R`$\x82\x01R`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x04\xFFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x12\x04\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x05\x13W=`\0\x80>=`\0\xFD[PP`\0\x80T`@Qc\xD7\x82=\xF7`\xE0\x1B\x81R`\x04\x81\x01\x87\x90R\x91\x93P\x83\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\xD7\x82=\xF7\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x05\x9EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x12\x04\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x92PPP\x80\x15a\x05\xAFWP`\x01[a\x06\x05W=\x80\x80\x15a\x05\xDDW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x05\xE2V[``\x91P[P`\x04\x81\x01\x90P\x80\x80` \x01\x90Q\x81\x01\x90a\x05\xFD\x91\x90a\x11PV[\x92PPa\x06\rV[`\x01`\xFF\x1B\x91P[`\0T`@Qc\xD7\x82=\xF7`\xE0\x1B\x81R`\x04\x81\x01\x85\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xD7\x82=\xF7\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x06\x91W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x12\x04\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x92PPP\x80\x15a\x06\xA2WP`\x01[a\x06\xF8W=\x80\x80\x15a\x06\xD0W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x06\xD5V[``\x91P[P`\x04\x81\x01\x90P\x80\x80` \x01\x90Q\x81\x01\x90a\x06\xF0\x91\x90a\x11PV[\x91PPa\x06\xFFV[P`\x01`\xFF\x1B[a\x07'`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01eTrade1`\xD0\x1B\x81RP\x85a\x0F:V[a\x07P`@Q\x80`@\x01`@R\x80`\x07\x81R` \x01fProfit1`\xC8\x1B\x81RP\x83a\x0F\x83V[a\x07x`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01e*90\xB22\x99`\xD1\x1B\x81RP\x84a\x0F:V[a\x07\xA1`@Q\x80`@\x01`@R\x80`\x07\x81R` \x01f(97\xB34\xBA\x19`\xC9\x1B\x81RP\x82a\x0F\x83V[a\x07\xCC`@Q\x80`@\x01`@R\x80`\t\x81R` \x01h\x13X^\x14\x1C\x9B\xD9\x9A]`\xBA\x1B\x81RP\x8Ba\x0F\x83V[\x80\x82\x12\x15a\x07\xE2W\x83\x97P\x80\x99P\x82\x98Pa\x07\xECV[\x82\x96P\x81\x99P\x83\x98P[a\x07\xF6\x88\x88a\x10\xCFV[\x94Pa\x08\x03`\x01\x87a\x11\nV[\x95PPP\x88\x83\x11\x80\x15a\x08\x15WP\x89\x84\x10[a\x01\xDEWa\x08=`@Q\x80``\x01`@R\x80`%\x81R` \x01a\x12$`%\x919\x85\x89\x8Ba\x0F\xC8V[`@Qc*6\x9F#`\xE0\x1B\x81R`\x04\x81\x01\x88\x90R`$\x81\x01\x89\x90R`D\x01a\x01\xAEV[`\0\x80a\x03\xE8\x83\x10a\x08\xB4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7Fepsilon must be less than 1000\0\0`D\x82\x01R`d\x01a\x01\xAEV[P`\x01`\xFF\x1B\x90P`\x01\x80g\r\xE0\xB6\xB3\xA7d\0\0`\0\x80a\x08\xD5\x84\x84a\x10\xCFV[\x90P`\0\x80[`\x03a\x08\xE7\x87\x87a\x10\xCFV[a\x08\xF1\x91\x90a\x10\xE8V[a\x08\xFB\x90\x87a\x11\nV[\x91P`\x03a\t\t\x87\x87a\x10\xCFV[a\t\x13\x91\x90a\x10\xE8V[a\t\x1D\x90\x86a\x10\xCFV[\x90P`\0\x80T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x99\x9B\x93\xAF`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\t\xA9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x12\x04\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\t\xBDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\xE1\x91\x90a\x11\x1DV[`\x01`\x01`\xA0\x1B\x03\x16c@\xC1\x0F\x190a\t\xFA\x84\x86a\x11\nV[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x04\x83\x01R`$\x82\x01R`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\n~W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x12\x04\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\n\x92W=`\0\x80>=`\0\xFD[PPPP`\0\x80T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x99\x9B\x93\xAF`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0B W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x12\x04\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0B4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0BX\x91\x90a\x11\x1DV[`\0T`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x91c\t^\xA7\xB3\x91\x16a\x0Bx\x84\x86a\x11\nV[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x04\x83\x01R`$\x82\x01R`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0B\xFCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x12\x04\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x0C\x10W=`\0\x80>=`\0\xFD[PP`\0\x80T`@Qc\x1D\xF4i\x8F`\xE3\x1B\x81R`\x04\x81\x01\x87\x90R\x91\x93P\x83\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\xEF\xA3Lx\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0C\x9BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x12\x04\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x92PPP\x80\x15a\x0C\xACWP`\x01[a\r\x02W=\x80\x80\x15a\x0C\xDAW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x0C\xDFV[``\x91P[P`\x04\x81\x01\x90P\x80\x80` \x01\x90Q\x81\x01\x90a\x0C\xFA\x91\x90a\x11PV[\x92PPa\r\nV[`\x01`\xFF\x1B\x91P[`\0T`@Qc\x1D\xF4i\x8F`\xE3\x1B\x81R`\x04\x81\x01\x85\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xEF\xA3Lx\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\r\x8EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x12\x04\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x92PPP\x80\x15a\r\x9FWP`\x01[a\r\xF5W=\x80\x80\x15a\r\xCDW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\r\xD2V[``\x91P[P`\x04\x81\x01\x90P\x80\x80` \x01\x90Q\x81\x01\x90a\r\xED\x91\x90a\x11PV[\x91PPa\r\xFCV[P`\x01`\xFF\x1B[a\x0E$`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01eTrade1`\xD0\x1B\x81RP\x85a\x0F:V[a\x0EM`@Q\x80`@\x01`@R\x80`\x07\x81R` \x01fProfit1`\xC8\x1B\x81RP\x83a\x0F\x83V[a\x0Eu`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01e*90\xB22\x99`\xD1\x1B\x81RP\x84a\x0F:V[a\x0E\x9E`@Q\x80`@\x01`@R\x80`\x07\x81R` \x01f(97\xB34\xBA\x19`\xC9\x1B\x81RP\x82a\x0F\x83V[a\x0E\xC9`@Q\x80`@\x01`@R\x80`\t\x81R` \x01h\x13X^\x14\x1C\x9B\xD9\x9A]`\xBA\x1B\x81RP\x8Ba\x0F\x83V[\x80\x82\x12\x15a\x0E\xDFW\x83\x97P\x80\x99P\x82\x98Pa\x0E\xE9V[\x82\x96P\x81\x99P\x83\x98P[a\x0E\xF3\x88\x88a\x10\xCFV[\x94Pa\x0F\0`\x01\x87a\x11\nV[\x95PPP\x88\x83\x11\x80\x15a\x0F\x12WP\x89\x84\x10[a\x08\xDBWa\x08=`@Q\x80``\x01`@R\x80`%\x81R` \x01a\x12$`%\x919\x85\x89\x8Ba\x0F\xC8V[a\x0F\x7F\x82\x82`@Q`$\x01a\x0FP\x92\x91\x90a\x11\xB2V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c-\x83\x9C\xB3`\xE2\x1B\x17\x90Ra\x10\x17V[PPV[a\x0F\x7F\x82\x82`@Q`$\x01a\x0F\x99\x92\x91\x90a\x11\xB2V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\x1ES\x13G`\xE1\x1B\x17\x90Ra\x10\x17V[a\x10\x11\x84\x84\x84\x84`@Q`$\x01a\x0F\xE2\x94\x93\x92\x91\x90a\x11\xD4V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\xA7\xA8xS`\xE0\x1B\x17\x90Ra\x10\x17V[PPPPV[a\x10 \x81a\x10#V[PV[\x80Qjconsole.log` \x83\x01`\0\x80\x84\x83\x85Z\xFAPPPPPV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x10\xAAWa\x10\xAAa\x10DV[PP\x805\x92` \x90\x91\x015\x91PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x10\xE2Wa\x10\xE2a\x10\xB9V[\x92\x91PPV[`\0\x82a\x11\x05WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[\x80\x82\x01\x80\x82\x11\x15a\x10\xE2Wa\x10\xE2a\x10\xB9V[`\0` \x82\x84\x03\x12\x15a\x112Wa\x112a\x10DV[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x11IW`\0\x80\xFD[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x11eWa\x11ea\x10DV[PQ\x91\x90PV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a\x11\x92W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\x11vV[P`\0` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[`@\x81R`\0a\x11\xC5`@\x83\x01\x85a\x11lV[\x90P\x82` \x83\x01R\x93\x92PPPV[`\x80\x81R`\0a\x11\xE7`\x80\x83\x01\x87a\x11lV[` \x83\x01\x95\x90\x95RP`@\x81\x01\x92\x90\x92R``\x90\x91\x01R\x91\x90PV\xFETarget contract does not containProfitFinder[FOUND] (i,trade,profit):";
    /// The bytecode of the contract.
    pub static ATOMICV2_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x01yW`\x005`\xE0\x1C\x80c\x99\x9B\x93\xAF\x11a\0\xFAW\x80c\xD2\xF7&Z\x11a\0\xBEW\x80c\xD2\xF7&Z\x14a\x03gW\x80c\xD7\x82=\xF7\x14a\x03zW\x80c\xEF\xA3Lx\x14a\x03\x8DW\x80c\xF3\xC9s\xCF\x14a\x03\xA0W\x80c\xFA.Y\x94\x14a\x03\xADWa\x01yV[\x80c\x99\x9B\x93\xAF\x14a\x02\xCFW\x80c\x9F'\xEFO\x14a\x02\xE2W\x80c\xA1S\x87\x89\x14a\x02\xF5W\x80c\xA6\xB8|5\x14a\x03AW\x80c\xB9\x93>\x84\x14a\x03TWa\x01yV[\x80cr\xB9\x82F\x11a\x01AW\x80cr\xB9\x82F\x14a\x02\x80W\x80c\x85\xB3\x19\xFF\x14a\x02\x97W\x80c\x8A/\xA5J\x14a\x02\xA0W\x80c\x93e \xC3\x14a\x02\xB3W\x80c\x98\xD8\x83M\x14a\x02\xBCWa\x01yV[\x80c\x0F\xF5s\x1D\x14a\x01\xDEW\x80c5(\x93\0\x14a\x02\x0BW\x80c5\xA9\x9A\xD0\x14a\x02\x1EW\x80c8\xD5.\x0F\x14a\x023W\x80cdI\xFCW\x14a\x02^W[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x01\xF1a\x01\xEC6`\x04a\"\x98V[a\x03\xB6V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01[`@Q\x80\x91\x03\x90\xF3[a\x01\xF1a\x02\x196`\x04a\"\x98V[a\x04\xF8V[a\x021a\x02,6`\x04a\"\xBDV[a\x066V[\0[`\x02Ta\x02F\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\x02V[`\tTa\x02p\x90a\x01\0\x90\x04`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x02\x02V[a\x02\x89`\x07T\x81V[`@Q\x90\x81R` \x01a\x02\x02V[a\x02\x89`\x08T\x81V[a\x021a\x02\xAE6`\x04a\"\xBDV[a\x07'V[a\x02\x89`\x05T\x81V[`\x04Ta\x02F\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x03Ta\x02F\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x01Ta\x02F\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\nT`\x0BT`\x0CT`\rT`\x0ETa\x03\x12\x94\x93\x92`\xFF\x16\x91\x90\x85V[`@Qa\x02\x02\x95\x94\x93\x92\x91\x90\x94\x85R` \x85\x01\x93\x90\x93R\x90\x15\x15`@\x84\x01R``\x83\x01R`\x80\x82\x01R`\xA0\x01\x90V[a\x02\x89a\x03O6`\x04a\"\x98V[a\x08\rV[a\x02\x89a\x03b6`\x04a\"\xE7V[a\x0C\xCBV[`\0Ta\x02F\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x021a\x03\x886`\x04a\"\xBDV[a\x12\xD9V[a\x021a\x03\x9B6`\x04a\"\xBDV[a\x14\x16V[`\tTa\x02p\x90`\xFF\x16\x81V[a\x02\x89`\x06T\x81V[`\x04\x80T`@Qc\x0F\xF5s\x1D`\xE0\x1B\x81R\x91\x82\x01\x84\x90R`$\x82\x01\x83\x90R`\0\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\x0F\xF5s\x1D\x90`D\x01`@\x80Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x04CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a'\x88\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x92PPP\x80\x15a\x04sWP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\x04p\x91\x81\x01\x90a#6V[`\x01[a\x04\xCDW=\x80\x80\x15a\x04\xA1W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x04\xA6V[``\x91P[P`\x04\x81\x01\x90P\x80\x80` \x01\x90Q\x81\x01\x90a\x04\xC1\x91\x90a#6V[\x90\x93P\x91Pa\x04\xD0\x90PV[PP[C`\nUB`\x0BU`\x0C\x80T`\xFF\x19\x16`\x01\x17\x90U`\r\x82\x90U`\x0E\x81\x90U\x90\x93\x90\x92P\x90PV[`\x04\x80T`@Qb5(\x93`\xE8\x1B\x81R\x91\x82\x01\x84\x90R`$\x82\x01\x83\x90R`\0\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c5(\x93\0\x90`D\x01`@\x80Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x05\x84W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a'\x88\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x92PPP\x80\x15a\x05\xB4WP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\x05\xB1\x91\x81\x01\x90a#6V[`\x01[a\x06\x0EW=\x80\x80\x15a\x05\xE2W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x05\xE7V[``\x91P[P`\x04\x81\x01\x90P\x80\x80` \x01\x90Q\x81\x01\x90a\x06\x02\x91\x90a#6V[\x90\x93P\x91Pa\x06\x11\x90PV[PP[C`\nUB`\x0BU`\x0C\x80T`\xFF\x19\x16\x90U`\r\x82\x90U`\x0E\x81\x90U\x90\x93\x90\x92P\x90PV[a\x06?\x81a\x14mV[`\tTa\x06T\x90a\x01\0\x90\x04`\xFF\x16\x82a\x17\xAAV[`\tT`\x02T`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01Ra\x07\x1C\x92`\xFF\x16\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01[` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x06\xDFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a'\x88\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x06\xF3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x17\x91\x90a#]V[a\x1B\rV[a\x07$a\x1F\x0FV[PV[a\x070\x81a\x14mV[`\tTa\x07E\x90a\x01\0\x90\x04`\xFF\x16\x82a\x1B\rV[`\tT`\x02T`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01Ra\x07\x1C\x92`\xFF\x16\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01[` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x07\xD0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a'\x88\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x07\xE4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\x08\x91\x90a#]V[a\x17\xAAV[`\0\x80T`@\x80QcB\xC9\x88[`\xE1\x1B\x81R\x90Q\x83\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x85\x93\x10\xB6\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\x08\x90W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a'\x88\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x08\xA4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xC8\x91\x90a#]V[\x90P`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cg\xE8(\xBF`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\tWW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a'\x88\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\tkW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\x8F\x91\x90a#yV[`\x01`\x01`\xA0\x1B\x03\x16c\xC1\xE0\x04;`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01```@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\n\x05W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a'\x88\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\n\x19W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n=\x91\x90a#\xACV[PP\x90P`\0\x80`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEB\xAD\xEF\x01`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01```@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\n\xD1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a'\x88\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\n\xE5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\t\x91\x90a#\xACV[\x92P\x92P\x92P\x87\x85\x10\x15a\x0B\xCFW`\n`\0\x82a\x0B.\x85g\r\xE0\xB6\xB3\xA7d\0\0a#\xF3V[a\x0B8\x91\x90a$\x10V[a\x0BB\x90\x87a$2V[\x90Pa\x0BO`\n\x82a$2V[\x90Pa\x0B~`@Q\x80`@\x01`@R\x80`\x0B\x81R` \x01j6\xB4\xB7 \xB6\xB7\xBA\xB7:$\xB7`\xA9\x1B\x81RP\x83a!ZV[a\x0B\xAB`@Q\x80`@\x01`@R\x80`\x0B\x81R` \x01j6\xB0\xBC \xB6\xB7\xBA\xB7:$\xB7`\xA9\x1B\x81RP\x82a!ZV[`\tTa\x0B\xC6\x90a\x01\0\x90\x04`\xFF\x16\x8B\x84\x84`\x05`\x03a\x0C\xCBV[\x97PPPa\x0C\xC0V[`@\x80Q\x80\x82\x01\x90\x91R`\x0F\x81Rn!\xB7\xB6\xB8:\xBA2\xB2\x106\xB0\xBC\x104\xB7`\x89\x1B` \x82\x01R`\n\x90a\x0C0\x90\x83a\x0C\x0F\x87g\r\xE0\xB6\xB3\xA7d\0\0a#\xF3V[a\x0C\x19\x91\x90a$\x10V[a\x0C+\x90g\r\xE0\xB6\xB3\xA7d\0\0a$2V[a!ZV[`\0a\x0C<\x85\x84a$2V[\x90Pa\x0CI`\n\x82a$2V[\x90Pa\x0Cx`@Q\x80`@\x01`@R\x80`\x0B\x81R` \x01j6\xB4\xB7 \xB6\xB7\xBA\xB7:$\xB7`\xA9\x1B\x81RP\x83a!ZV[a\x0C\xA5`@Q\x80`@\x01`@R\x80`\x0B\x81R` \x01j6\xB0\xBC \xB6\xB7\xBA\xB7:$\xB7`\xA9\x1B\x81RP\x82a!ZV[`\tTa\x0C\xBB\x90`\xFF\x16\x8B\x84\x84`\x05`\x03a\x0C\xCBV[\x97PPP[PPPPP\x92\x91PPV[`\0\x83\x85\x10a\r!W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7Flower must be less than upper\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x80T`@Qc\x02\xB0\xCC\r`\xE2\x1B\x81R\x89\x15\x15`\x04\x82\x01R`$\x81\x01\x88\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\n\xC304\x90`D\x01`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\r\xACW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a'\x88\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x92PPP\x80\x15a\r\xE1WP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\r\xDE\x91\x90\x81\x01\x90a$\x7FV[`\x01[a\x0EaW=\x80\x80\x15a\x0E\x0FW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x0E\x14V[``\x91P[Pa\x0EF`@Q\x80`@\x01`@R\x80`\x10\x81R` \x01o67\xBB\xB2\xB9(94\xB1\xB2\x902\xB997\xB9`\x81\x1B\x81RPa!\x9FV[\x80`@Qc\x1AC\x9E\xD1`\xE0\x1B\x81R`\x04\x01a\r\x18\x91\x90a&pV[P`\0\x80T`@Qc\x02\xB0\xCC\r`\xE2\x1B\x81R\x8D\x15\x15`\x04\x82\x01R`$\x81\x01\x8B\x90R\x92\x95P\x90\x93P`\x01`\x01`\xA0\x1B\x03\x16\x91Pc\n\xC304\x90`D\x01`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0E\xF2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a'\x88\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x92PPP\x80\x15a\x0F'WP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0F$\x91\x90\x81\x01\x90a$\x7FV[`\x01[a\x0F\x8CW=\x80\x80\x15a\x0FUW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x0FZV[``\x91P[Pa\x0EF`@Q\x80`@\x01`@R\x80`\x10\x81R` \x01o:\xB882\xB9(94\xB1\xB2\x902\xB997\xB9`\x81\x1B\x81RPa!\x9FV[P\x92PPPa\x0F\xBD`@Q\x80`@\x01`@R\x80`\n\x81R` \x01ilowerPrice`\xB0\x1B\x81RP\x83a!ZV[a\x0F\xE9`@Q\x80`@\x01`@R\x80`\n\x81R` \x01iupperPrice`\xB0\x1B\x81RP\x82a!ZV[a\x10\x15`@Q\x80`@\x01`@R\x80`\n\x81R` \x01itargtPrice`\xB0\x1B\x81RP\x89a!ZV[`\0a\x10!\x89\x84a&\x83V[\x90P`\0a\x10/\x8A\x84a&\x83V[\x90Pa\x10:\x82a!\xE2V[a\x10C\x81a!\xE2V[`\0a\x10O\x82\x84a&\xAAV[\x12a\x10\x9CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7Froot must be between bounds\0\0\0\0\0`D\x82\x01R`d\x01a\r\x18V[`\0a\x10\xA8\x8A\x8Aa$2V[\x90P`\0[`\x02a\x10\xB9\x8B\x8Da&\xDAV[a\x10\xC3\x91\x90a$\x10V[\x96P`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\n\xC304\x8F\x8A`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x11\x14\x92\x91\x90\x91\x15\x15\x82R` \x82\x01R`@\x01\x90V[`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x11jW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a'\x88\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x92PPP\x80\x15a\x11\x9FWP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x11\x9C\x91\x90\x81\x01\x90a$\x7FV[`\x01[a\x11\xD9W=\x80\x80\x15a\x11\xCDW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x11\xD2V[``\x91P[PPa\x11\xDFV[P\x92PPP[a\x12\t`@Q\x80`@\x01`@R\x80`\x08\x81R` \x01g0\xB6\xB7\xBA\xB7:$\xB7`\xC1\x1B\x81RP\x89a!ZV[a\x127`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01ktarget price`\xA0\x1B\x81RP\x8Ea!ZV[a\x12e`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01kresult price`\xA0\x1B\x81RP\x82a!ZV[`\0a\x12q\x8E\x83a&\x83V[\x90Pa\x12|\x81a!\xE2V[`\0a\x12\x88\x87\x83a&\xAAV[\x13a\x12\x95W\x88\x9BPa\x12\x9CV[\x88\x9CP\x80\x95P[a\x12\xA6\x8D\x8Da$2V[\x93Pa\x12\xB3`\x01\x84a&\xDAV[\x92PPP\x88\x82\x11\x80\x15a\x12\xC5WP\x87\x81\x10[a\x10\xADWPPPPPP\x96\x95PPPPPPV[a\x12\xE2\x81a\x14mV[`\tTa\x12\xF7\x90a\x01\0\x90\x04`\xFF\x16\x82a\x1B\rV[`\tT`\x02T`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01Ra\x130\x92`\xFF\x16\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01a\x07zV[`\x03T`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x13\xB1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a'\x88\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x13\xC5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\xE9\x91\x90a#]V[`\x07\x81\x90U`\x06Ta\x13\xFA\x91a&\x83V[`@QcB\xD5\xD4o`\xE1\x1B\x81R`\x04\x01a\r\x18\x91\x81R` \x01\x90V[a\x14\x1F\x81a\x14mV[`\tTa\x144\x90a\x01\0\x90\x04`\xFF\x16\x82a\x17\xAAV[`\tT`\x02T`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01Ra\x130\x92`\xFF\x16\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01a\x06\x89V[`\x03T`@Qcp\xA0\x821`\xE0\x1B\x81R3`\x04\x82\x01R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x14\xEFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a'\x88\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x15\x03W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15'\x91\x90a#]V[\x90P\x81\x81\x10\x15a\x15TW`@Qc\n\xBEZ\x89`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x81\x01\x83\x90R`D\x01a\r\x18V[`\x03T`@Qcn\xB1v\x9F`\xE1\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xDDb\xED>\x90`D\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x15\xDCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a'\x88\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x15\xF0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\x14\x91\x90a#]V[\x90P\x82\x81\x10\x15a\x16AW`@Qc\xDAV\xD3\xC5`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x81\x01\x84\x90R`D\x01a\r\x18V[`\x03T`@Qc#\xB8r\xDD`\xE0\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x85\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90`d\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x16\xD1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a'\x88\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x16\xE5W=`\0\x80>=`\0\xFD[PP`\x03T`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x92Pcp\xA0\x821\x91P`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x17jW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a'\x88\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x17~W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17\xA2\x91\x90a#]V[`\x06UPPPV[\x81\x15a\x19\0W`\x02T`\x01T`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x84\x90R\x91\x16\x90c\t^\xA7\xB3\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x18>W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a'\x88\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x18RW=`\0\x80>=`\0\xFD[PP`\x01T`\x02T`@Qc\xD0\x04\xF0\xF7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x86\x90R\x91\x16\x92Pc\xD0\x04\xF0\xF7\x91P`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x18\xE4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a'\x88\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x18\xF8W=`\0\x80>=`\0\xFD[PPPPPPV[`\x03T`\x01T`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x84\x90R\x91\x16\x90c\t^\xA7\xB3\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x19\x8EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a'\x88\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x19\xA2W=`\0\x80>=`\0\xFD[PP`\x01T`\x03T`@Qc\xD0\x04\xF0\xF7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x86\x90R\x91\x16\x92Pc\xD0\x04\xF0\xF7\x91P`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1A4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a'\x88\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x1AHW=`\0\x80>=`\0\xFD[PP`\x02T`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x92Pcp\xA0\x821\x91P`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x1A\xCDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a'\x88\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x1A\xE1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B\x05\x91\x90a#]V[`\x05U[PPV[\x81\x15a\x1B\xBEW`\x02T`\0T`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x84\x90R\x91\x16\x90c\t^\xA7\xB3\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1B\xA1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a'\x88\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x1B\xB5W=`\0\x80>=`\0\xFD[PPPPa\x1CeV[`\x03T`\0T`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x84\x90R\x91\x16\x90c\t^\xA7\xB3\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1CLW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a'\x88\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x1C`W=`\0\x80>=`\0\xFD[PPPP[`\0\x80T`@Qc\x02\xB0\xCC\r`\xE2\x1B\x81R\x84\x15\x15`\x04\x82\x01R`$\x81\x01\x84\x90R\x82\x91\x82\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\n\xC304\x90`D\x01`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x1C\xF4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a'\x88\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x1D\x08W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1D0\x91\x90\x81\x01\x90a$\x7FV[\x93P\x93P\x93P\x93P\x83a\x1D^W\x83\x83\x83\x83`@Qc\x03\x14\xE6#`\xE3\x1B\x81R`\x04\x01a\r\x18\x94\x93\x92\x91\x90a&\xEDV[`\0T`@Qc1>\xEA\xB5`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cb}\xD5j\x90a\x1D\x8E\x90\x84\x90`\x04\x01a&pV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1D\xE6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a'\x88\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x92PPP\x80\x15a\x1D\xF7WP`\x01[a\x1EFW=\x80\x80\x15a\x1E%W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x1E*V[``\x91P[P\x80`@Qcg\xA1k\x8D`\xE1\x1B\x81R`\x04\x01a\r\x18\x91\x90a'\x1EV[\x85a\x18\xF8W`\x02T`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x1E\xCCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a'\x88\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x1E\xE0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1F\x04\x91\x90a#]V[`\x05UPPPPPPV[`\x03T`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x1F\x90W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a'\x88\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x1F\xA4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1F\xC8\x91\x90a#]V[`\x07\x81\x90U`\x06T\x11\x15a SW\x7F\xB6[.\x08]}\x04\x0C1?}N\x1A\xC9\x0FY7\x02o\xEEI~\x0E$\xA7\xEF\xF1jU\xE1\xC5\xEA`\x07T`\x06Ta \x07\x91\x90a$2V[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xA1`\x06T`\x07Ta (\x81\x83a&\x83V[`@Qc\xB1n7\x83`\xE0\x1B\x81R`\x04\x81\x01\x93\x90\x93R`$\x83\x01\x91\x90\x91R\x19`D\x82\x01R`d\x01a\r\x18V[`\0`\x06T`\x07Ta e\x91\x90a$2V[\x90P\x80`\x08`\0\x82\x82Ta y\x91\x90a&\xDAV[\x90\x91UPP`@Q\x81\x81R\x7F5}\x90_\x181 \x97\x97\xDFMU\xD7\x9C\\[\xF1\xD9\xF71\x1C\x97j\xFD\x05\xE1=\x88\x1E\xAB\x9B\xC8\x90` \x01`@Q\x80\x91\x03\x90\xA1`\x03T`\x07T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a!?W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a'\x88\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a!SW=`\0\x80>=`\0\xFD[PPPPPV[a\x1B\t\x82\x82`@Q`$\x01a!p\x92\x91\x90a'eV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c-\x83\x9C\xB3`\xE2\x1B\x17\x90Ra\"#V[a\x07$\x81`@Q`$\x01a!\xB3\x91\x90a&pV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\x10L\x13\xEB`\xE2\x1B\x17\x90Ra\"#V[a\x07$\x81`@Q`$\x01a!\xF8\x91\x81R` \x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c-[l\xB9`\xE0\x1B\x17\x90R[a\x07$\x81\x80Qjconsole.log` \x83\x01`\0\x80\x84\x83\x85Z\xFAPPPPPV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\"\xAEWa\"\xAEa\"HV[PP\x805\x92` \x90\x91\x015\x91PV[`\0` \x82\x84\x03\x12\x15a\"\xD2Wa\"\xD2a\"HV[P5\x91\x90PV[\x80\x15\x15\x81\x14a\x07$W`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a#\x03Wa#\x03a\"HV[\x865a#\x0E\x81a\"\xD9V[\x98` \x88\x015\x98P`@\x88\x015\x97``\x81\x015\x97P`\x80\x81\x015\x96P`\xA0\x015\x94P\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a#LWa#La\"HV[PP\x80Q` \x90\x91\x01Q\x90\x92\x90\x91PV[`\0` \x82\x84\x03\x12\x15a#rWa#ra\"HV[PQ\x91\x90PV[`\0` \x82\x84\x03\x12\x15a#\x8EWa#\x8Ea\"HV[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a#\xA5W`\0\x80\xFD[\x93\x92PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a#\xC4Wa#\xC4a\"HV[\x83Q\x92P` \x84\x01Q\x91P`@\x84\x01Q\x90P\x92P\x92P\x92V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a$\nWa$\na#\xDDV[\x92\x91PPV[`\0\x82a$-WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[\x81\x81\x03\x81\x81\x11\x15a$\nWa$\na#\xDDV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0[\x83\x81\x10\x15a$vW\x81\x81\x01Q\x83\x82\x01R` \x01a$^V[PP`\0\x91\x01RV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a$\x98Wa$\x98a\"HV[\x84Qa$\xA3\x81a\"\xD9V[\x80\x94PP` \x80\x86\x01Q\x93P`@\x80\x87\x01Q\x93P``\x87\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a%\x1CW\x82QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x85\x90R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[\x81\x89\x01\x91P\x89`\x1F\x83\x01\x12a%\x83W\x82QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x85\x90R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x81\xFD[\x81Q\x81\x81\x11\x15a%\x95Wa%\x95a$EV[\x83Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a%\xBCWa%\xBCa$EV[\x81\x86R\x82\x81R\x8C\x87\x84\x87\x01\x01\x11\x15a&$W\x85QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x88\x90R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R\x93P`\x84\x84\xFD[a&3\x83\x88\x83\x01\x89\x88\x01a$[V[\x99\x9C\x98\x9BP\x96\x99PPPPPPPPV[`\0\x81Q\x80\x84Ra&\\\x81` \x86\x01` \x86\x01a$[V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a#\xA5` \x83\x01\x84a&DV[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a&\xA3Wa&\xA3a#\xDDV[P\x92\x91PPV[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15a&\xC6Wa&\xC6a#\xDDV[\x81\x81\x05\x83\x14\x82\x15\x17a$\nWa$\na#\xDDV[\x80\x82\x01\x80\x82\x11\x15a$\nWa$\na#\xDDV[\x84\x15\x15\x81R\x83` \x82\x01R\x82`@\x82\x01R`\x80``\x82\x01R`\0a'\x14`\x80\x83\x01\x84a&DV[\x96\x95PPPPPPV[`@\x81R`\x19`@\x82\x01R\x7FDEX swap failed with data\0\0\0\0\0\0\0``\x82\x01R`\x80` \x82\x01R`\0a#\xA5`\x80\x83\x01\x84a&DV[`@\x81R`\0a'x`@\x83\x01\x85a&DV[\x90P\x82` \x83\x01R\x93\x92PPPV\xFETarget contract does not contain";
    /// The deployed bytecode of the contract.
    pub static ATOMICV2_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
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
            f.debug_tuple(::core::stringify!(AtomicV2))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> AtomicV2<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                ATOMICV2_ABI.clone(),
                client,
            ))
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
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
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
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
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
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([159, 39, 239, 79], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lower_exchange_price` (0x35a99ad0) function
        pub fn lower_exchange_price(
            &self,
            input_y_amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([53, 169, 154, 208], input_y_amount)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `profitFinder` (0x98d8834d) function
        pub fn profit_finder(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([152, 216, 131, 77], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `quote` (0x999b93af) function
        pub fn quote(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([153, 155, 147, 175], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `raise_exchange_price` (0x8a2fa54a) function
        pub fn raise_exchange_price(
            &self,
            input_y_amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([138, 47, 165, 74], input_y_amount)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `searchLowerPrice` (0x35289300) function
        pub fn search_lower_price(
            &self,
            max_iters: ::ethers::core::types::U256,
            epsilon: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([53, 40, 147, 0], (max_iters, epsilon))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `searchRaisePrice` (0x0ff5731d) function
        pub fn search_raise_price(
            &self,
            max_iters: ::ethers::core::types::U256,
            epsilon: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([15, 245, 115, 29], (max_iters, epsilon))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `tradePacket` (0xa1538789) function
        pub fn trade_packet(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                bool,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([161, 83, 135, 137], ())
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
        ///Calls the contract's `try_lower_exchange_price` (0xefa34c78) function
        pub fn try_lower_exchange_price(
            &self,
            input_y_amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([239, 163, 76, 120], input_y_amount)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `try_raise_exchange_price` (0xd7823df7) function
        pub fn try_raise_exchange_price(
            &self,
            input_y_amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([215, 130, 61, 247], input_y_amount)
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
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, AtomicV2Events> {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for AtomicV2<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `AttemptedProfit` with signature `AttemptedProfit(int256)` and selector `0x85aba8de`
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
        Hash,
    )]
    #[etherror(name = "AttemptedProfit", abi = "AttemptedProfit(int256)")]
    pub struct AttemptedProfit {
        pub profit: ::ethers::core::types::I256,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
    )]
    #[etherror(
        name = "InsufficientBalanceY",
        abi = "InsufficientBalanceY(uint256,uint256)"
    )]
    pub struct InsufficientBalanceY {
        pub balance: ::ethers::core::types::U256,
        pub payment: ::ethers::core::types::U256,
    }
    ///Custom Error type `MaximizingProfitTrade` with signature `MaximizingProfitTrade(uint256,uint256)` and selector `0x2a369f23`
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
        Hash,
    )]
    #[etherror(
        name = "MaximizingProfitTrade",
        abi = "MaximizingProfitTrade(uint256,uint256)"
    )]
    pub struct MaximizingProfitTrade {
        pub trade: ::ethers::core::types::U256,
        pub profit: ::ethers::core::types::U256,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
    )]
    pub enum AtomicV2Errors {
        AttemptedProfit(AttemptedProfit),
        CatastrophicSwapFailure(CatastrophicSwapFailure),
        DexSwapFailure(DexSwapFailure),
        FindingTradeError(FindingTradeError),
        InsufficientApprovalY(InsufficientApprovalY),
        InsufficientBalanceX(InsufficientBalanceX),
        InsufficientBalanceY(InsufficientBalanceY),
        MaximizingProfitTrade(MaximizingProfitTrade),
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
            if let Ok(decoded) =
                <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <AttemptedProfit as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AttemptedProfit(decoded));
            }
            if let Ok(decoded) =
                <CatastrophicSwapFailure as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CatastrophicSwapFailure(decoded));
            }
            if let Ok(decoded) = <DexSwapFailure as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DexSwapFailure(decoded));
            }
            if let Ok(decoded) = <FindingTradeError as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::FindingTradeError(decoded));
            }
            if let Ok(decoded) =
                <InsufficientApprovalY as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InsufficientApprovalY(decoded));
            }
            if let Ok(decoded) =
                <InsufficientBalanceX as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InsufficientBalanceX(decoded));
            }
            if let Ok(decoded) =
                <InsufficientBalanceY as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InsufficientBalanceY(decoded));
            }
            if let Ok(decoded) =
                <MaximizingProfitTrade as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::MaximizingProfitTrade(decoded));
            }
            if let Ok(decoded) = <NotProfitable as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NotProfitable(decoded));
            }
            if let Ok(decoded) =
                <SimulatedSwapFailure as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SimulatedSwapFailure(decoded));
            }
            if let Ok(decoded) =
                <UnprofitableArbitrage as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UnprofitableArbitrage(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for AtomicV2Errors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::AttemptedProfit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CatastrophicSwapFailure(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DexSwapFailure(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::FindingTradeError(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InsufficientApprovalY(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InsufficientBalanceX(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InsufficientBalanceY(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MaximizingProfitTrade(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NotProfitable(element) => ::ethers::core::abi::AbiEncode::encode(element),
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
                _ if selector == <AttemptedProfit as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <CatastrophicSwapFailure as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector == <DexSwapFailure as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <FindingTradeError as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <InsufficientApprovalY as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <InsufficientBalanceX as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <InsufficientBalanceY as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <MaximizingProfitTrade as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector == <NotProfitable as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <SimulatedSwapFailure as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <UnprofitableArbitrage as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for AtomicV2Errors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AttemptedProfit(element) => ::core::fmt::Display::fmt(element, f),
                Self::CatastrophicSwapFailure(element) => ::core::fmt::Display::fmt(element, f),
                Self::DexSwapFailure(element) => ::core::fmt::Display::fmt(element, f),
                Self::FindingTradeError(element) => ::core::fmt::Display::fmt(element, f),
                Self::InsufficientApprovalY(element) => ::core::fmt::Display::fmt(element, f),
                Self::InsufficientBalanceX(element) => ::core::fmt::Display::fmt(element, f),
                Self::InsufficientBalanceY(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaximizingProfitTrade(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotProfitable(element) => ::core::fmt::Display::fmt(element, f),
                Self::SimulatedSwapFailure(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnprofitableArbitrage(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for AtomicV2Errors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<AttemptedProfit> for AtomicV2Errors {
        fn from(value: AttemptedProfit) -> Self {
            Self::AttemptedProfit(value)
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
    impl ::core::convert::From<MaximizingProfitTrade> for AtomicV2Errors {
        fn from(value: MaximizingProfitTrade) -> Self {
            Self::MaximizingProfitTrade(value)
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
    )]
    #[ethcall(
        name = "intermediateTokenXBalance",
        abi = "intermediateTokenXBalance()"
    )]
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
    )]
    #[ethcall(name = "lower_exchange_price", abi = "lower_exchange_price(uint256)")]
    pub struct LowerExchangePriceCall {
        pub input_y_amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `profitFinder` function with signature `profitFinder()` and selector `0x98d8834d`
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
        Hash,
    )]
    #[ethcall(name = "profitFinder", abi = "profitFinder()")]
    pub struct ProfitFinderCall;
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
        Hash,
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
        Hash,
    )]
    #[ethcall(name = "raise_exchange_price", abi = "raise_exchange_price(uint256)")]
    pub struct RaiseExchangePriceCall {
        pub input_y_amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `searchLowerPrice` function with signature `searchLowerPrice(uint256,uint256)` and selector `0x35289300`
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
        Hash,
    )]
    #[ethcall(name = "searchLowerPrice", abi = "searchLowerPrice(uint256,uint256)")]
    pub struct SearchLowerPriceCall {
        pub max_iters: ::ethers::core::types::U256,
        pub epsilon: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `searchRaisePrice` function with signature `searchRaisePrice(uint256,uint256)` and selector `0x0ff5731d`
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
        Hash,
    )]
    #[ethcall(name = "searchRaisePrice", abi = "searchRaisePrice(uint256,uint256)")]
    pub struct SearchRaisePriceCall {
        pub max_iters: ::ethers::core::types::U256,
        pub epsilon: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `tradePacket` function with signature `tradePacket()` and selector `0xa1538789`
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
        Hash,
    )]
    #[ethcall(name = "tradePacket", abi = "tradePacket()")]
    pub struct TradePacketCall;
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
        Hash,
    )]
    #[ethcall(
        name = "try_arbitrage_until_target_price",
        abi = "try_arbitrage_until_target_price(uint256,uint256)"
    )]
    pub struct TryArbitrageUntilTargetPriceCall {
        pub target_price: ::ethers::core::types::U256,
        pub max_iterations: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `try_lower_exchange_price` function with signature `try_lower_exchange_price(uint256)` and selector `0xefa34c78`
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
        Hash,
    )]
    #[ethcall(
        name = "try_lower_exchange_price",
        abi = "try_lower_exchange_price(uint256)"
    )]
    pub struct TryLowerExchangePriceCall {
        pub input_y_amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `try_raise_exchange_price` function with signature `try_raise_exchange_price(uint256)` and selector `0xd7823df7`
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
        Hash,
    )]
    #[ethcall(
        name = "try_raise_exchange_price",
        abi = "try_raise_exchange_price(uint256)"
    )]
    pub struct TryRaiseExchangePriceCall {
        pub input_y_amount: ::ethers::core::types::U256,
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
        Hash,
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
        ProfitFinder(ProfitFinderCall),
        Quote(QuoteCall),
        RaiseExchangePrice(RaiseExchangePriceCall),
        SearchLowerPrice(SearchLowerPriceCall),
        SearchRaisePrice(SearchRaisePriceCall),
        TradePacket(TradePacketCall),
        TryArbitrageUntilTargetPrice(TryArbitrageUntilTargetPriceCall),
        TryLowerExchangePrice(TryLowerExchangePriceCall),
        TryRaiseExchangePrice(TryRaiseExchangePriceCall),
    }
    impl ::ethers::core::abi::AbiDecode for AtomicV2Calls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <XtoyCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Xtoy(decoded));
            }
            if let Ok(decoded) = <YtoxCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Ytox(decoded));
            }
            if let Ok(decoded) = <AssetCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Asset(decoded));
            }
            if let Ok(decoded) =
                <CumulativeProfitCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CumulativeProfit(decoded));
            }
            if let Ok(decoded) = <ExchangeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Exchange(decoded));
            }
            if let Ok(decoded) = <FindTradeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::FindTrade(decoded));
            }
            if let Ok(decoded) =
                <IntermediateTokenXBalanceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::IntermediateTokenXBalance(decoded));
            }
            if let Ok(decoded) =
                <IntermediateTokenYEndBalanceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::IntermediateTokenYEndBalance(decoded));
            }
            if let Ok(decoded) =
                <IntermediateTokenYStartBalanceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::IntermediateTokenYStartBalance(decoded));
            }
            if let Ok(decoded) =
                <LiquidExchangeCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::LiquidExchange(decoded));
            }
            if let Ok(decoded) =
                <LowerExchangePriceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::LowerExchangePrice(decoded));
            }
            if let Ok(decoded) = <ProfitFinderCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ProfitFinder(decoded));
            }
            if let Ok(decoded) = <QuoteCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Quote(decoded));
            }
            if let Ok(decoded) =
                <RaiseExchangePriceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RaiseExchangePrice(decoded));
            }
            if let Ok(decoded) =
                <SearchLowerPriceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SearchLowerPrice(decoded));
            }
            if let Ok(decoded) =
                <SearchRaisePriceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SearchRaisePrice(decoded));
            }
            if let Ok(decoded) = <TradePacketCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TradePacket(decoded));
            }
            if let Ok(decoded) =
                <TryArbitrageUntilTargetPriceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TryArbitrageUntilTargetPrice(decoded));
            }
            if let Ok(decoded) =
                <TryLowerExchangePriceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TryLowerExchangePrice(decoded));
            }
            if let Ok(decoded) =
                <TryRaiseExchangePriceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TryRaiseExchangePrice(decoded));
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
                Self::CumulativeProfit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Exchange(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::FindTrade(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IntermediateTokenXBalance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IntermediateTokenYEndBalance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IntermediateTokenYStartBalance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LiquidExchange(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LowerExchangePrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProfitFinder(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Quote(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RaiseExchangePrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SearchLowerPrice(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SearchRaisePrice(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TradePacket(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TryArbitrageUntilTargetPrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TryLowerExchangePrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TryRaiseExchangePrice(element) => {
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
                Self::IntermediateTokenXBalance(element) => ::core::fmt::Display::fmt(element, f),
                Self::IntermediateTokenYEndBalance(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IntermediateTokenYStartBalance(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LiquidExchange(element) => ::core::fmt::Display::fmt(element, f),
                Self::LowerExchangePrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProfitFinder(element) => ::core::fmt::Display::fmt(element, f),
                Self::Quote(element) => ::core::fmt::Display::fmt(element, f),
                Self::RaiseExchangePrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::SearchLowerPrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::SearchRaisePrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::TradePacket(element) => ::core::fmt::Display::fmt(element, f),
                Self::TryArbitrageUntilTargetPrice(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TryLowerExchangePrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::TryRaiseExchangePrice(element) => ::core::fmt::Display::fmt(element, f),
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
    impl ::core::convert::From<ProfitFinderCall> for AtomicV2Calls {
        fn from(value: ProfitFinderCall) -> Self {
            Self::ProfitFinder(value)
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
    impl ::core::convert::From<SearchLowerPriceCall> for AtomicV2Calls {
        fn from(value: SearchLowerPriceCall) -> Self {
            Self::SearchLowerPrice(value)
        }
    }
    impl ::core::convert::From<SearchRaisePriceCall> for AtomicV2Calls {
        fn from(value: SearchRaisePriceCall) -> Self {
            Self::SearchRaisePrice(value)
        }
    }
    impl ::core::convert::From<TradePacketCall> for AtomicV2Calls {
        fn from(value: TradePacketCall) -> Self {
            Self::TradePacket(value)
        }
    }
    impl ::core::convert::From<TryArbitrageUntilTargetPriceCall> for AtomicV2Calls {
        fn from(value: TryArbitrageUntilTargetPriceCall) -> Self {
            Self::TryArbitrageUntilTargetPrice(value)
        }
    }
    impl ::core::convert::From<TryLowerExchangePriceCall> for AtomicV2Calls {
        fn from(value: TryLowerExchangePriceCall) -> Self {
            Self::TryLowerExchangePrice(value)
        }
    }
    impl ::core::convert::From<TryRaiseExchangePriceCall> for AtomicV2Calls {
        fn from(value: TryRaiseExchangePriceCall) -> Self {
            Self::TryRaiseExchangePrice(value)
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
    )]
    pub struct LiquidExchangeReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `profitFinder` function with signature `profitFinder()` and selector `0x98d8834d`
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
        Hash,
    )]
    pub struct ProfitFinderReturn(pub ::ethers::core::types::Address);
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
        Hash,
    )]
    pub struct QuoteReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `searchLowerPrice` function with signature `searchLowerPrice(uint256,uint256)` and selector `0x35289300`
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
        Hash,
    )]
    pub struct SearchLowerPriceReturn {
        pub amount_in: ::ethers::core::types::U256,
        pub profit: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `searchRaisePrice` function with signature `searchRaisePrice(uint256,uint256)` and selector `0x0ff5731d`
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
        Hash,
    )]
    pub struct SearchRaisePriceReturn {
        pub amount_in: ::ethers::core::types::U256,
        pub profit: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `tradePacket` function with signature `tradePacket()` and selector `0xa1538789`
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
        Hash,
    )]
    pub struct TradePacketReturn {
        pub block_number: ::ethers::core::types::U256,
        pub block_timestamp: ::ethers::core::types::U256,
        pub raise_price: bool,
        pub amount_in: ::ethers::core::types::U256,
        pub profit: ::ethers::core::types::U256,
    }
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
        Hash,
    )]
    pub struct TryArbitrageUntilTargetPriceReturn {
        pub amount_in: ::ethers::core::types::U256,
    }
}
