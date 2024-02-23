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
                        name: ::std::borrow::ToOwned::to_owned("solverAddress"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
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
                    ::std::borrow::ToOwned::to_owned("cdf"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("cdf"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("input"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("output"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
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
                    ::std::borrow::ToOwned::to_owned("divWadDown"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("divWadDown"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("x"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("y"),
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
                                    name: ::std::borrow::ToOwned::to_owned("z"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("divWadUp"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("divWadUp"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("x"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("y"),
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
                                    name: ::std::borrow::ToOwned::to_owned("z"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
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
                    ::std::borrow::ToOwned::to_owned("log"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("log"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("x"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("z"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("logData"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("logData"),
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
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
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
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("mulWadDown"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("mulWadDown"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("x"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("y"),
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
                                    name: ::std::borrow::ToOwned::to_owned("z"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("mulWadUp"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("mulWadUp"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("x"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("y"),
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
                                    name: ::std::borrow::ToOwned::to_owned("z"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("pdf"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("pdf"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("input"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("output"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ppf"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("ppf"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("input"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("output"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
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
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("solver"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("solver"),
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
                    ::std::borrow::ToOwned::to_owned("sqrt"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("sqrt"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("x"),
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
                                    name: ::std::borrow::ToOwned::to_owned("z"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
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
                    ::std::borrow::ToOwned::to_owned("strategyName"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("strategyName"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("LogArbData"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("LogArbData"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("xBalance"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("yBalance"),
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
                    ::std::borrow::ToOwned::to_owned("LogAssetData"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("LogAssetData"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("lexPrice"),
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
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("LogDfmmData"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("LogDfmmData"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("price"),
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
                                    name: ::std::borrow::ToOwned::to_owned("liq"),
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
                                    name: ::std::borrow::ToOwned::to_owned("sigma"),
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
                            ],
                            anonymous: false,
                        },
                    ],
                ),
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
                    ::std::borrow::ToOwned::to_owned("Price"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Price"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("price"),
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
                    ::std::borrow::ToOwned::to_owned("AttemptedProfit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("AttemptedProfit"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("profit"),
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
                    ::std::borrow::ToOwned::to_owned("Infinity"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("Infinity"),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("Min"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("Min"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NegativeInfinity"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("NegativeInfinity"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OutOfBounds"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("OutOfBounds"),
                            inputs: ::std::vec![],
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
    const __BYTECODE: &[u8] = b"`\x80`@\x90\x80\x82R4b\0\x04\xAFWPb\0-\xCD`\xA0\x818\x03\x91\x82\x84Q\x93\x84\x92b\0\0*\x82\x85b\0\x04\xFCV[\x839\x81\x01\x03\x12b\0\x04RWb\0\0@\x81b\0\x05\x86V[` \x91b\0\0P\x83\x82\x01b\0\x05\x86V[b\0\0]\x85\x83\x01b\0\x05\x86V[\x90b\0\0z`\x80b\0\0r``\x86\x01b\0\x05\x86V[\x94\x01b\0\x05\x86V[\x93`\x01\x93\x84a\xFF\xFF\x19`\x0BT\x16\x17`\x0BU`\x01\x80`\xA0\x1B\x03\x80\x80\x93\x16\x91\x81`\x01\x80`\xA0\x1B\x03\x19\x95\x84\x87`\x02T\x16\x17`\x02U\x16\x85\x88T\x16\x17\x87U\x81`\0\x96\x16\x85\x87T\x16\x17\x86U\x16\x83`\x03T\x16\x17`\x03U\x81`\x04\x96\x16\x83\x87T\x16\x17\x86U\x80;\x15b\0\x04bW\x86\x86\x91\x89Q\x92\x83\x80\x92cTc\x17;`\xE1\x1B\x82RZ\xFA\x90\x81\x15b\0\x04\xA5W\x84\x91b\0\x04hW[P\x16\x80\x91`\x05T\x16\x17`\x05U\x80;\x15b\0\x04bW\x81\x84\x91\x87Q\x92\x83\x80\x92c\x06\xFD\xDE\x03`\xE0\x1B\x82RZ\xFA\x90\x81\x15b\0\x04XW\x82\x91b\0\x02\xA4W[P\x80Q\x93`\x01`\x01`@\x1B\x03\x85\x11b\0\x02\x91W`\x06T\x90\x84\x82\x81\x1C\x92\x16\x80\x15b\0\x02\x86W[\x87\x83\x10\x14b\0\x02sWP`\x1F\x81\x11b\0\x02'W[P\x84\x91`\x1F\x85\x11`\x01\x14b\0\x01\xBEW\x93\x94P\x84\x92\x91\x90\x83b\0\x01\xB2W[PP\x1B\x91`\0\x19\x90`\x03\x1B\x1C\x19\x16\x17`\x06U[Qa'\xD9\x90\x81b\0\x05\xF4\x829\xF3[\x01Q\x92P8\x80b\0\x01\x91V[`\x06\x81R\x85\x81 \x93\x95\x85\x91`\x1F\x19\x83\x16\x91[\x88\x83\x83\x10b\0\x02\x0CWPPP\x10b\0\x01\xF2W[PPP\x81\x1B\x01`\x06Ub\0\x01\xA4V[\x01Q`\0\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U8\x80\x80b\0\x01\xE3V[\x85\x87\x01Q\x88U\x90\x96\x01\x95\x94\x85\x01\x94\x87\x93P\x90\x81\x01\x90b\0\x01\xD0V[`\x06\x83R\x85\x83 `\x1F\x86\x01`\x05\x1C\x81\x01\x91\x87\x87\x10b\0\x02hW[`\x1F\x01`\x05\x1C\x01\x90\x84\x90[\x82\x81\x10b\0\x02\\WPPb\0\x01tV[\x84\x81U\x01\x84\x90b\0\x02LV[\x90\x91P\x81\x90b\0\x02AV[cNH{q`\xE0\x1B\x84R`\"\x90R`$\x83\xFD[\x91`\x7F\x16\x91b\0\x01`V[cNH{q`\xE0\x1B\x83R`A\x90R`$\x82\xFD[\x90P=\x80\x83\x83>b\0\x02\xB7\x81\x83b\0\x04\xFCV[\x81\x01\x90\x85\x81\x83\x03\x12b\0\x04RW\x80Q`\x01`\x01`@\x1B\x03\x91\x82\x82\x11b\0\x04\x04W\x01\x82`\x1F\x82\x01\x12\x15b\0\x03\xADW\x80Q\x91\x82\x11b\0\x03\x9AW\x87Q\x92b\0\x03\x06`\x1F\x84\x01`\x1F\x19\x16\x89\x01\x85b\0\x04\xFCV[\x82\x84R\x87\x83\x83\x01\x01\x11b\0\x03GW\x83\x91\x87\x91\x83[\x82\x81\x10b\0\x03/WPP\x83\x01\x01R8b\0\x01;V[\x81\x81\x01\x84\x01Q\x86\x82\x01\x85\x01R\x86\x94P\x89\x93\x01b\0\x03\x1AV[\x87QbF\x1B\xCD`\xE5\x1B\x81R\x80\x87\x01\x88\x90R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R`\x84\x90\xFD[cNH{q`\xE0\x1B\x84R`A\x86R`$\x84\xFD[\x87QbF\x1B\xCD`\xE5\x1B\x81R\x80\x87\x01\x88\x90R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x90\xFD[\x88QbF\x1B\xCD`\xE5\x1B\x81R\x80\x88\x01\x89\x90R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[b\0\x056V[\x86Q=\x84\x82>=\x90\xFD[b\0\x05\xA0V[\x90P\x86\x81\x81=\x83\x11b\0\x04\x9DW[b\0\x04\x82\x81\x83b\0\x04\xFCV[\x81\x01\x03\x12b\0\x04RWb\0\x04\x96\x90b\0\x05\x86V[8b\0\x01\x02V[P=b\0\x04vV[\x88Q=\x86\x82>=\x90\xFD[bF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01Ra7\xB7`\xF1\x1B`d\x82\x01R`\x84\x90\xFD[`\x1F\x90\x91\x01`\x1F\x19\x16\x81\x01\x90`\x01`\x01`@\x1B\x03\x82\x11\x90\x82\x10\x17b\0\x05 W`@RV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[Q\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03b\0\x05\x9BWV[`\0\x80\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01Rd code`\xD8\x1B`d\x82\x01R`\x84\x90\xFD\xFE`\x80`@R`\x046\x10a\x0FkW`\x005`\xE0\x1C\x80c-[l\xB9\x14a\x01\xACW\x80c6yr:\x14a\x01\xA7W\x80c7\xC6\xA4J\x14a\x01\xA2W\x80c8\xD5.\x0F\x14a\x01\x9DW\x80c9(\xFF\x97\x14a\x01\x98W\x80cI\xA7\xA2m\x14a\x01\x93W\x80cdI\xFCW\x14a\x01\x8EW\x80cgsB\xCE\x14a\x01\x89W\x80cr\xB9\x82F\x14a\x01\x84W\x80c\x85\xB3\x19\xFF\x14a\x01\x7FW\x80c\x93e \xC3\x14a\x01zW\x80c\x96\xFB\xEE\x1D\x14a\x01uW\x80c\x99\x9B\x93\xAF\x14a\x01pW\x80c\x9F'\xEFO\x14a\x01kW\x80c\xA8\xC6.v\x14a\x01fW\x80c\xAE\x97h\xA8\x14a\x01aW\x80c\xB3\xB2\xBF+\x14a\x01\\W\x80c\xBD%-\x06\x14a\x01WW\x80c\xD0\xB7\x1B\x1E\x14a\x01RW\x80c\xD2L\xE6\xE5\x14a\x01MW\x80c\xD2\xF7&Z\x14a\x01HW\x80c\xE1s\xAD%\x14a\x01CW\x80c\xE5$\xF8I\x14a\x01>W\x80c\xF3\xC9s\xCF\x14a\x019W\x80c\xF9\0^\xB5\x14a\x014Wc\xFA.Y\x94\x03a\x0FkWa\x0FMV[a\x0E\x9DV[a\x0EzV[a\x0E:V[a\rWV[a\x0C\xA3V[a\x0CFV[a\x0B\xFFV[a\x0B\xB2V[a\x06\xC2V[a\x06vV[a\x06MV[a\x06$V[a\x05\xFBV[a\x05%V[a\x05\x07V[a\x04\xE9V[a\x04\xCBV[a\x04\xADV[a\x04\x87V[a\x04^V[a\x03\x9AV[a\x02\xF7V[a\x02\xA5V[a\x02\x8DV[a\x02hV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01Ra7\xB7`\xF1\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[` \x90`\x03\x19\x01\x12a\x02cW`\x045\x90V[a\x02\x01V[4a\x02\x88W` a\x02\x80a\x02{6a\x02QV[a\x13KV[`@Q\x90\x81R\xF3[a\x01\xB1V[4a\x02\x88W` a\x02\x80a\x02\xA06a\x02QV[a\x15jV[4a\x02\x88W`@6`\x03\x19\x01\x12a\x02cW`\x045`$5\x90g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17\x82\x15\x15\x16\x15a\x02\xE7W` \x91`@Q\x91\x04\x81R\xF3[`\0\x80\xFD[`\0\x91\x03\x12a\x02cWV[4a\x02\x88W`\x006`\x03\x19\x01\x12a\x02cW`\x03T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[\x80\x15\x15\x03a\x02\xE7WV[`\0[\x83\x81\x10a\x03=WPP`\0\x91\x01RV[\x81\x81\x01Q\x83\x82\x01R` \x01a\x03-V[\x90` \x91a\x03f\x81Q\x80\x92\x81\x85R\x85\x80\x86\x01\x91\x01a\x03*V[`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[\x90\x92`\x80\x92a\x03\x97\x95\x94\x15\x15\x83R` \x83\x01R`@\x82\x01R\x81``\x82\x01R\x01\x90a\x03MV[\x90V[4a\x02\x88W``6`\x03\x19\x01\x12a\x02cW`$5a\x03\xB7\x81a\x03 V[`\x02T`\x01`\x01`\xA0\x1B\x03\x16\x90\x81;\x15a\x04YW`@Qc9(\xFF\x97`\xE0\x1B\x81R`\x04\x805\x90\x82\x01R\x90\x15\x15`$\x82\x01R`D\x805\x90\x82\x01R\x90`\0\x90\x81\x90\x83\x90`d\x90\x82\x90Z\xFA\x80\x15a\x04TW\x81\x80\x93\x81\x80\x93a\x04'W[PP\x90a\x04#\x91`@Q\x94\x85\x94\x85a\x03rV[\x03\x90\xF3[\x91P\x91Pa\x04#\x93Pa\x04L\x92P=\x80\x91\x83>a\x04D\x81\x83a\r5V[\x81\x01\x90a\x10=V[\x91\x938a\x04\x10V[a\x11\xB5V[a\x0F\xCEV[4a\x02\x88W`\x006`\x03\x19\x01\x12a\x02cW`\x02T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x02\x88W`\x006`\x03\x19\x01\x12a\x02cW` `\xFF`\x0BT`\x08\x1C\x16`@Q\x90\x15\x15\x81R\xF3[4a\x02\x88W` 6`\x03\x19\x01\x12a\x02cW` a\x02\x80`\x045a\x17=V[4a\x02\x88W`\x006`\x03\x19\x01\x12a\x02cW` `\tT`@Q\x90\x81R\xF3[4a\x02\x88W`\x006`\x03\x19\x01\x12a\x02cW` `\nT`@Q\x90\x81R\xF3[4a\x02\x88W`\x006`\x03\x19\x01\x12a\x02cW` `\x07T`@Q\x90\x81R\xF3[4a\x02\x88W`@6`\x03\x19\x01\x12a\x02cWa\x05S`$5a\x05E\x81a\x17\xFBV[`\x0BT`\x08\x1C`\xFF\x16a\x19\xFCV[`\x0BT`\xFF\x16`\x03Ta\x05|\x90a\x05p\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x90\x81;\x15a\x04YW`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x91` \x90\x83\x90`$\x90\x82\x90Z\xFA\x90\x81\x15a\x04TWa\x05\xC0\x92`\0\x92a\x05\xCAW[P`\x045a\x1C\xF5V[a\x05\xC8a\x1F\xC1V[\0[a\x05\xED\x91\x92P` =` \x11a\x05\xF4W[a\x05\xE5\x81\x83a\r5V[\x81\x01\x90a\x11\xC1V[\x908a\x05\xB7V[P=a\x05\xDBV[4a\x02\x88W`\x006`\x03\x19\x01\x12a\x02cW`\x04T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x02\x88W`\x006`\x03\x19\x01\x12a\x02cW`\0T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x02\x88W`\x006`\x03\x19\x01\x12a\x02cW`\x05T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x02\x88W`@6`\x03\x19\x01\x12a\x02cW`\x045`$5\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x02\xE7W` \x90`@Q\x90`\x01g\r\xE0\xB6\xB3\xA7d\0\0`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x81R\xF3[4a\x02\x88W` \x80`\x03\x196\x01\x12a\x02cW`\x04\x90\x815\x91`@\x92\x83Q\x83\x81\x01\x81a\x06\xEC\x82a\x11\xD0V[\x03\x91a\x07\0`\x1F\x19\x93\x84\x81\x01\x83R\x82a\r5V[Q\x90 \x90\x85Qa\x07B\x86\x82\x01\x92\x82a\x076\x85``\x90` \x81R`\t` \x82\x01Rh\x13\x1B\xD9\xD3\x9B\xDC\x9BX[`\xBA\x1B`@\x82\x01R\x01\x90V[\x03\x90\x81\x01\x83R\x82a\r5V[Q\x90 \x14a\t\xE0W[P`\0\x80T\x90\x93\x90a\x07g\x90a\x05p\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x90\x81;\x15a\x04YW\x83\x85\x92\x84\x83Q\x80\x95\x81\x93cP\x1A\xD8\xFF`\xE1\x1B\x83RZ\xF1\x91\x82\x15a\x04TW\x85\x92a\t\xC1W[P`\x03Ta\x07\xAB\x90a\x05p\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x85T\x90\x93\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x84;\x15a\x04YW\x82Qcp\xA0\x821`\xE0\x1B\x80\x82R`\x01`\x01`\xA0\x1B\x03\x84\x16\x83\x83\x01\x90\x81R\x90\x93\x91\x90\x88\x90\x82\x90\x81\x90` \x01\x03\x81\x8AZ\xFA\x90\x81\x15a\x04TW\x89\x91a\t\xA4W[P\x82Ta\x08\x15\x90a\x05p\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x95\x86;\x15a\x04YW\x85Q\x85\x81R`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x84\x84\x01\x90\x81R\x89\x90\x84\x90\x81\x90` \x01\x03\x81\x8AZ\xFA\x92\x83\x15a\x04TW\x7F\xBC}\xF0Z&tit\x1Er\xD9\x8D\xFC\x1F\xBE\xFAP*\x8D]\x8BD\t\xA6R\x13\x1B\x8E\x14\xFD,\xEE\x93a\x08\x9A\x91\x8C\x91a\t\x87W[P\x87Q\x93\x84\x93B\x90\x85\x90\x94\x93\x92``\x92`\x80\x83\x01\x96\x83R` \x83\x01R`@\x82\x01R\x01RV[\x03\x90\xA1\x84;\x15a\x04YW\x82Q\x82\x81R3\x82\x82\x01\x90\x81R\x90\x95\x87\x91\x87\x91\x90\x82\x90\x81\x90` \x01\x03\x91Z\xFA\x94\x85\x15a\x04TW\x87\x95a\thW[P\x83;\x15a\x04YW\x82Q\x91\x82R3\x90\x82\x01\x90\x81R\x90\x92\x85\x91\x84\x91\x90\x82\x90\x81\x90` \x01\x03\x91Z\xFA\x91\x82\x15a\x04TW\x7F[\x88\x97\xA4\xB3\xA7sN;\x9A8\xAFo>m5\xC4\xF5g2n#*KTJ\xDB\xD8?\x94$\xFD\x94\x86\x93a\tCW[PPQ\x91\x82R` \x82\x01RB`@\x82\x01R\x80``\x81\x01[\x03\x90\xA1\x80\xF3[a\t=\x92\x93P\x90\x81a\t`\x92\x90=\x10a\x05\xF4Wa\x05\xE5\x81\x83a\r5V[\x91\x908a\t&V[a\t\x80\x91\x95P\x86=\x88\x11a\x05\xF4Wa\x05\xE5\x81\x83a\r5V[\x938a\x08\xD0V[a\t\x9E\x91P\x8B=\x8D\x11a\x05\xF4Wa\x05\xE5\x81\x83a\r5V[8a\x08uV[a\t\xBB\x91P\x88=\x8A\x11a\x05\xF4Wa\x05\xE5\x81\x83a\r5V[8a\x07\xFEV[a\t\xD9\x91\x92P\x84=\x86\x11a\x05\xF4Wa\x05\xE5\x81\x83a\r5V[\x908a\x07\x93V[`\x02Ta\t\xF7\x90a\x05p\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x80;\x15a\x04YW\x84Q\x90c3\x85N\xFD`\xE2\x1B\x82R``\x82\x80a\n \x86\x88\x83\x01\x91\x90` \x83\x01\x92RV[\x03\x81\x84Z\xFA\x92\x83\x15a\x04TW`\0\x92`\0\x90`\0\x95a\x0BzW[P\x82;\x15a\x04YW\x87Q\x92c\x03\xB4\xD1\x03`\xE4\x1B\x84R\x87\x84\x80a\nc\x86\x8B\x83\x01\x91\x90` \x83\x01\x92RV[\x03\x81\x84Z\xFA\x93\x84\x15a\x04TW`\0\x94a\x0B[W[P\x80;\x15a\x04YW\x88Qc@\xDA\xFDa`\xE1\x1B\x81R\x87\x81\x01\x93\x84R\x92`\xA0\x91\x84\x91\x82\x90\x81\x90` \x01\x03\x91Z\xFA\x80\x15a\x04TW\x7Ft\xBB\x813\xEFY\x05\xC0\xA7\xE9\xCF\xB2\xAF,9p\xF3\xE3\xB7.\xD7\x0C\xCB\xB6'\x95\x96\xED\x86}3x\x95a\x0B!\x93`\0\x92a\x0B*W[P\x81Q\x89\x83\x01Q\x92\x8B\x01Q\x8BQ\x96\x87RB` \x88\x01R`@\x87\x01\x97\x90\x97R``\x86\x01\x93\x90\x93R`\x80\x85\x01R`\xA0\x84\x01\x91\x90\x91R`\xC0\x83\x01R`\xE0\x82\x01\x92\x90\x92R\x90\x81\x90a\x01\0\x82\x01\x90V[\x03\x90\xA18a\x07KV[a\x0BM\x91\x92P`\xA0=`\xA0\x11a\x0BTW[a\x0BE\x81\x83a\r5V[\x81\x01\x90a\x12\xABV[\x908a\n\xD6V[P=a\x0B;V[a\x0Bs\x91\x94P\x88=\x8A\x11a\x05\xF4Wa\x05\xE5\x81\x83a\r5V[\x928a\nwV[\x91\x93PPa\x0B\xA1\x91\x93P``=``\x11a\x0B\xABW[a\x0B\x99\x81\x83a\r5V[\x81\x01\x90a\x12\x90V[\x93\x91\x92\x908a\n:V[P=a\x0B\x8FV[4a\x02\x88W`@6`\x03\x19\x01\x12a\x02cW`\x045`$5g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x81\x02\x92\x81\x84\x04\x14\x90\x15\x17\x81\x15\x15\x16\x15a\x02\xE7W` \x91`\x01`@Q\x92`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x81R\xF3[4a\x02\x88W` g\x1B\xC1mgN\xC8\0\0a\x0C=a\x0C-a\x0C8g\x13\xA0K\xBD\xFD\xC9\xBE\x88a\x0C2a\x0C-6a\x02QV[a\x15\x02V[\x05a\x15YV[a#\"V[\x05`@Q\x90\x81R\xF3[4a\x02\x88Wa\x0CT6a\x02QV[a\x0C]\x81a\x15YV[\x80\x82\x02\x91`\0\x82\x12`\x01`\xFF\x1B\x82\x14\x16a\x0C\x9EW\x81\x83\x05\x14\x90\x15\x17\x15a\x0C\x9EWg\"\xC9U\"\x95T\xC1\xB6a\x0C=a\x0C-g\x1B\xC1mgN\xC8\0\0` \x94\x05a$GV[a\x14\xECV[4a\x02\x88W`\x006`\x03\x19\x01\x12a\x02cW`\x01T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[\x90`\x01\x82\x81\x1C\x92\x16\x80\x15a\x0C\xFCW[` \x83\x10\x14a\x0C\xE6WV[cNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[\x91`\x7F\x16\x91a\x0C\xDBV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\r0W`@RV[a\r\x06V[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\r0W`@RV[4a\x02\x88W`\x006`\x03\x19\x01\x12a\x02cW`@Q`\x06T`\0\x82a\rz\x83a\x0C\xCCV[\x91\x82\x82R` \x93`\x01\x90\x85`\x01\x82\x16\x91\x82`\0\x14a\x0E\x1AWPP`\x01\x14a\r\xBDW[Pa\r\xA9\x92P\x03\x83a\r5V[a\x04#`@Q\x92\x82\x84\x93\x84R\x83\x01\x90a\x03MV[\x84\x91P`\x06`\0R\x7F\xF6R\"#\x13\xE2\x84YR\x8D\x92\x0Be\x11\\\x16\xC0O>\xFC\x82\xAA\xED\xC9{\xE5\x9F?7|\r?\x90`\0\x91[\x85\x83\x10a\x0E\x02WPPa\r\xA9\x93P\x82\x01\x018a\r\x9CV[\x80T\x83\x89\x01\x85\x01R\x87\x94P\x86\x93\x90\x92\x01\x91\x81\x01a\r\xEBV[`\xFF\x19\x16\x85\x82\x01Ra\r\xA9\x95\x15\x15`\x05\x1B\x85\x01\x01\x92P8\x91Pa\r\x9C\x90PV[4a\x02\x88W`@6`\x03\x19\x01\x12a\x02cW`\x045`$5\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x02\xE7W` \x90g\r\xE0\xB6\xB3\xA7d\0\0`@Q\x91\x04\x81R\xF3[4a\x02\x88W`\x006`\x03\x19\x01\x12a\x02cW` `\xFF`\x0BT\x16`@Q\x90\x15\x15\x81R\xF3[4a\x02\x88W`@6`\x03\x19\x01\x12a\x02cWa\x0E\xCE`$5a\x0E\xBD\x81a\x17\xFBV[`\x0BT`\x08\x1C`\xFF\x16`\x045a\x1C\xF5V[`\x0BT`\xFF\x16`\x03Ta\x0E\xEB\x90a\x05p\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x90\x81;\x15a\x04YW`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x91` \x90\x83\x90`$\x90\x82\x90Z\xFA\x90\x81\x15a\x04TWa\x05\xC0\x92`\0\x92a\x0F,W[Pa\x19\xFCV[a\x0FF\x91\x92P` =` \x11a\x05\xF4Wa\x05\xE5\x81\x83a\r5V[\x908a\x0F&V[4a\x02\x88W`\x006`\x03\x19\x01\x12a\x02cW` `\x08T`@Q\x90\x81R\xF3[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01Rtnor receive functions`X\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01Rd code`\xD8\x1B`d\x82\x01R`\x84\x90\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\r0W`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\x80\x81\x83\x03\x12a\x02cW\x80Qa\x10R\x81a\x03 V[\x92` \x92\x83\x83\x01Q\x93`@\x84\x01Q\x93``\x81\x01Q\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x11eW\x01\x82`\x1F\x82\x01\x12\x15a\x11\x0CW\x80Q\x91a\x10\x8E\x83a\x10!V[\x93a\x10\x9C`@Q\x95\x86a\r5V[\x83\x85R\x81\x84\x84\x01\x01\x11a\x10\xB8Wa\x03\x97\x92\x91\x84\x82\x01\x91\x01a\x03*V[`\x84\x90`@Q\x90bF\x1B\xCD`\xE5\x1B\x82R`\x04\x82\x01R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x83\x90R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x84\x90R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[`@Q=`\0\x82>=\x90\xFD[\x90\x81` \x91\x03\x12a\x02cWQ\x90V[` \x80\x82R`\x06T`\0\x92a\x11\xE4\x82a\x0C\xCCV[\x91\x82` \x83\x01R`@`\x01\x91`\x01\x81\x16\x90\x81`\0\x14a\x12mWP`\x01\x14a\x12\x0EW[PPPPP\x90V[`\x06`\0\x90\x81R\x94\x96\x95P\x91\x92\x91\x7F\xF6R\"#\x13\xE2\x84YR\x8D\x92\x0Be\x11\\\x16\xC0O>\xFC\x82\xAA\xED\xC9{\xE5\x9F?7|\r?[\x83\x86\x10a\x12ZWPPPP`@\x92\x93P\x01\x018\x80\x80\x80\x80a\x12\x06V[\x80T\x85\x87\x01\x83\x01R\x94\x87\x01\x94\x82\x01a\x12>V[\x93\x95PPPP`@\x93P`\xFF\x19\x16\x83\x83\x01R\x15\x15`\x05\x1B\x01\x018\x80\x80\x80\x80a\x12\x06V[\x90\x81``\x91\x03\x12a\x02cW\x80Q\x91`@` \x83\x01Q\x92\x01Q\x90V[\x90\x81`\xA0\x91\x03\x12a\x02cW`@Q\x90`\xA0\x82\x01\x90\x82\x82\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x17a\r0W`\x80\x91`@R\x80Q\x83R` \x81\x01Q` \x84\x01R`@\x81\x01Q`@\x84\x01R``\x81\x01Q``\x84\x01R\x01Q`\x01\x80`\xA0\x1B\x03\x81\x16\x81\x03a\x02\xE7W`\x80\x82\x01R\x90V[\x15a\x13\x1AWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x90\xFD[}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x90a\x13w`\0\x82\x13a\x13\x13V[q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06a\x13\x93\x82a![V[``\x92\x83\x82`\x9F\x03\x01\x1B`\x9F\x1C\x90`_\x19\x01}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x02\x92l\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x82m\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x81m\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x81m\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x81m\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x81m\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x81lFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x02\x8C\x1D\x01\x02\x8A\x1D\x01\x02\x88\x1D\x01\x02\x86\x1D\x01\x02\x84\x1D\x01\x02\x82\x1D\x01\x91x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x91l\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x90\x82m\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x81l\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x81m\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x81m\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x81l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x81\x01\x02\x89\x1D\x01\x02\x87\x1D\x01\x02\x85\x1D\x01\x02\x83\x1D\x01\x02\x90\x1D\x01\x02\x01\x05\x02\x01\x01`\xAE\x1D\x90V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x90g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x81\x02\x92\x81\x84\x05\x14\x90\x15\x17\x15a\x0C\x9EWV[\x90c;\x9A\xCA\0\x91\x82\x81\x02\x92\x81\x84\x05\x14\x90\x15\x17\x15a\x0C\x9EWV[`\0\x81\x90\x03\x91\x90`\x01`\xFF\x1B\x81\x14`\x01\x16a\x0C\x9EW`\0\x19\x83\x05\x03a\x0C\x9EWV[`\x01`\xFF\x1B\x81\x14a\x0C\x9EW`\0\x03\x90V[g\x06\xF0[Y\xD3\xB2\0\0\x81\x14a\x177Wg\r\xE0\xB6\xB3\xA7d\0\0\x80\x82\x12\x15a\x16\xE1W\x81\x15a\x17\x02W`\x01\x82`\x01\x1B\x91`\x02\x93\x83\x05`\x02\x03a\x0C\x9EW`\0\x83\x12\x80\x15a\x17&W[a\x17\x14W\x82\x15a\x16\xE1Wg\x1B\xC1mgN\xC8\0\0\x83\x14a\x17\x02W\x82\x12\x91\x82\x15a\x16\xF3W\x92[a\x15\xDB\x84a%\xF2V[\x80\x15a\x16\xE1Wa\x16Ma\x16\x0Ca\x16\x07a\x16\x02a\x15\xFDa\x16R\x95\x99\x97\x96\x99a\x13KV[a&\xB3V[a\x17=V[a\x15\x1FV[a\x16Ha\x16 a\x16\x1B\x83a&\x1DV[a!\xCDV[a\x16Ba\x16=a\x167a\x162\x86a&HV[a!\xE5V[\x85a'*V[a!\xFDV[\x90a&\x91V[a\x1F\x9BV[a&\xDBV[\x93`\0\x92[\x81\x84\x10a\x16\x89WPPPPa\x03\x97\x91a\x16v\x91`\0\x14a\x16{Wa%\xCBV[a\x15YV[a\x16\x84\x90a\x15YV[a%\xCBV[\x90\x91a\x16\xD7\x86a\x16\xD1a\x16\xA1\x85a\x16H\x86\x99\x9Ba#\"V[a\x16Ba\x16\xC1a\x16\xBCa\x16\xB7a\x16v\x87\x80a'*V[a$GV[a'\x03V[a\x16\xCB\x83\x86a'*V[\x90a\x1F\x9BV[\x90a\"\xDBV[\x95\x01\x92\x91\x90a\x16WV[`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x90\xFD[a\x16\xFC\x90a\x1F_V[\x92a\x15\xD2V[`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x90\xFD[`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x90\xFD[Pg\x1B\xC1mgN\xC8\0\0\x83\x13a\x15\xAEV[P`\0\x90V[`\xB5\x81`\x01`\x88\x1B\x81\x10\x15a\x17\xE4W[\x80i\x01\0\0\0\0\0\0\0\0\0b\x01\0\0\x92\x10\x15a\x17\xD7W[e\x01\0\0\0\0\0\x81\x10\x15a\x17\xCAW[c\x01\0\0\0\x81\x10\x15a\x17\xBDW[\x01\x02`\x12\x1C`\x01\x90\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x80\x80\x92\x04\x10\x90\x03\x90V[`\x10\x1C\x91`\x08\x1B\x91a\x17\x81V[` \x1C\x91`\x10\x1B\x91a\x17tV[`@\x1C\x91` \x1B\x91a\x17eV[Ph\xB5\0\0\0\0\0\0\0\0\x90P`\x80\x82\x90\x1Ca\x17MV[`\x04\x80Ta\x18\x13\x90a\x05p\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x90\x81;\x15a\x04YW`@\x80Qcp\xA0\x821`\xE0\x1B\x80\x82R3\x82\x85\x01\x90\x81R` \x96\x91\x95\x93\x94\x93\x91\x92\x90\x87\x90\x82\x90\x81\x90\x83\x01\x03\x81\x85Z\xFA\x90\x81\x15a\x04TW`\0\x91a\x19\xDFW[P\x82\x81\x10a\x19\xBCWP\x80;\x15a\x04YW\x83Qcn\xB1v\x9F`\xE1\x1B\x81R3\x84\x82\x01\x90\x81R0` \x82\x01R\x87\x90\x82\x90\x81\x90`@\x01\x03\x81\x85Z\xFA\x90\x81\x15a\x04TW`\0\x91a\x19\x9FW[P\x82\x81\x10a\x19xWP\x80;\x15a\x04YW\x83Qc#\xB8r\xDD`\xE0\x1B\x81R3\x84\x82\x01\x90\x81R0` \x82\x01R`@\x81\x01\x93\x90\x93R\x91`\0\x91\x83\x91\x82\x90\x84\x90\x82\x90``\x01\x03\x92Z\xF1\x80\x15a\x04TWa\x19_W[P\x80Ta\x19\x04\x90a\x05p\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x80;\x15a\x04YW\x91Q\x92\x83R0\x90\x83\x01\x90\x81R\x83\x91\x83\x91\x82\x90\x81\x90` \x01\x03\x91Z\xFA\x90\x81\x15a\x04TWa\x19@\x92`\0\x92a\x19BW[PP`\x08UV[V[a\x19X\x92P\x80=\x10a\x05\xF4Wa\x05\xE5\x81\x83a\r5V[8\x80a\x199V[\x80a\x19la\x19r\x92a\r\x1CV[\x80a\x02\xECV[8a\x18\xEDV[\x84Qc\xDAV\xD3\xC5`\xE0\x1B\x81R\x80\x85\x01\x91\x82R` \x82\x01\x84\x90R\x90\x81\x90`@\x01\x03\x90\xFD[\x03\x90\xFD[a\x19\xB6\x91P\x87=\x89\x11a\x05\xF4Wa\x05\xE5\x81\x83a\r5V[8a\x18\x9EV[\x84Qc\n\xBEZ\x89`\xE0\x1B\x81R\x80\x85\x01\x91\x82R` \x82\x01\x84\x90R\x90\x81\x90`@\x01\x03\x90\xFD[a\x19\xF6\x91P\x87=\x89\x11a\x05\xF4Wa\x05\xE5\x81\x83a\r5V[8a\x18XV[\x15a\x1A\xF7W`\x03Ta\x1A\x18\x90a\x05p\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\0\x80T\x90\x91\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x80;\x15a\x04YW`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16`\x04\x83\x01R`$\x82\x01\x84\x90R\x82\x90\x82\x90`D\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x04TWa\x1A\xE4W[P\x80Ta\x1A\x84\x90a\x05p\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x03T`\x01`\x01`\xA0\x1B\x03\x16\x92\x90\x80;\x15a\x04YW`@Qc\xD0\x04\xF0\xF7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x94\x90\x94\x16`\x04\x85\x01R`$\x84\x01\x91\x90\x91R\x82\x90`D\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x04TWa\x1A\xD7WPV[\x80a\x19la\x19@\x92a\r\x1CV[\x80a\x19la\x1A\xF1\x92a\r\x1CV[8a\x1AmV[`\x04Ta\x1B\x0E\x90a\x05p\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\0\x80T\x90\x92\x91\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x80;\x15a\x04YW`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16`\x04\x83\x01R`$\x82\x01\x83\x90R\x83\x90\x82\x90`D\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x04TWa\x1CVW[P\x81Ta\x1B{\x90a\x05p\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x04T\x90\x91\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x82;\x15a\x04YW`@Qc\xD0\x04\xF0\xF7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16`\x04\x83\x01R`$\x82\x01R\x90\x82\x90\x82\x90`D\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x04TWa\x1CCW[P`\x03Ta\x1B\xE6\x90a\x05p\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x80;\x15a\x04YW`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90` \x90\x82\x90`$\x90\x82\x90Z\xFA\x90\x81\x15a\x04TWa\x19@\x92\x91a\x1C$W[P`\x07UV[a\x1C=\x91P` =` \x11a\x05\xF4Wa\x05\xE5\x81\x83a\r5V[8a\x1C\x1EV[\x80a\x19la\x1CP\x92a\r\x1CV[8a\x1B\xCEV[\x80a\x19la\x1Cc\x92a\r\x1CV[8a\x1BdV[`@\x90a\x03\x97\x93\x92\x81R\x81` \x82\x01R\x01\x90a\x03MV[=\x15a\x1C\xABW=\x90a\x1C\x91\x82a\x10!V[\x91a\x1C\x9F`@Q\x93\x84a\r5V[\x82R=`\0` \x84\x01>V[``\x90V[\x90`\x80a\x03\x97\x92`@\x81R`\x19`@\x82\x01R\x7FDEX swap failed with data\0\0\0\0\0\0\0``\x82\x01R\x81` \x82\x01R\x01\x90a\x03MV[\x81\x15a\x1E\xD0W`\x03Ta\x1D\x12\x90a\x05p\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x01T`\x01`\x01`\xA0\x1B\x03\x16\x90\x80;\x15a\x04YW`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16`\x04\x83\x01R`$\x82\x01\x85\x90R`\0\x90\x82\x90`D\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x04TWa\x1E\xBDW[P[`\x02Ta\x1D}\x90a\x05p\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x90\x81;\x15a\x04YW`@Qc9(\xFF\x97`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R\x83\x15\x15`$\x82\x01R`D\x81\x01\x94\x90\x94R`\0\x93\x91\x84\x90\x83\x90`d\x90\x82\x90Z\xFA\x91\x82\x15a\x04TW\x84\x85\x91\x86\x90\x87\x95a\x1E\x9BW[P\x81\x15a\x1E}WPP`\x01Ta\x1D\xEC\x91Pa\x05p\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x91\x82;\x15a\x04YWa\x1E\x17\x92\x85\x92\x83`@Q\x80\x96\x81\x95\x82\x94c\xBD\x06%\xAB`\xE0\x1B\x84R`\x04\x84\x01a\x1CiV[\x03\x92Z\xF1\x90\x81a\x1EjW[Pa\x1EKWa\x19\x9Ba\x1E2a\x1C\x80V[`@Qcg\xA1k\x8D`\xE1\x1B\x81R\x91\x82\x91`\x04\x83\x01a\x1C\xB0V[\x15a\x1ESWPV[`\x03Ta\x1B\xE6\x90a\x05p\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x80a\x19la\x1Ew\x92a\r\x1CV[8a\x1E\"V[\x84a\x19\x9B\x91`@Q\x94\x85\x94c\x03\x14\xE6#`\xE3\x1B\x86R`\x04\x86\x01a\x03rV[\x92PPPa\x1E\xB4\x91\x92P=\x80\x86\x83>a\x04D\x81\x83a\r5V[\x93\x92\x908a\x1D\xCBV[\x80a\x19la\x1E\xCA\x92a\r\x1CV[8a\x1DdV[`\x04Ta\x1E\xE7\x90a\x05p\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x01T`\x01`\x01`\xA0\x1B\x03\x16\x90\x80;\x15a\x04YW`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16`\x04\x83\x01R`$\x82\x01\x85\x90R`\0\x90\x82\x90`D\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x04TWa\x1F?W[Pa\x1DfV[\x80a\x19la\x1FL\x92a\r\x1CV[8a\x1F9V[\x91\x90\x82\x03\x91\x82\x11a\x0C\x9EWV[\x90g\x1B\xC1mgN\xC8\0\0`\0\x83\x82\x03\x93\x12\x81\x84\x12\x81\x16\x91\x84\x13\x90\x15\x16\x17a\x0C\x9EWV[\x90g\x11\x90\0\xAB\x10\x0F\xFB\xFF\x19\x82\x01\x91\x82\x13`\x01\x16a\x0C\x9EWV[\x81\x81\x03\x92\x91`\0\x13\x80\x15\x82\x85\x13\x16\x91\x84\x12\x16\x17a\x0C\x9EWV[\x91\x90\x82\x01\x80\x92\x11a\x0C\x9EWV[`\x04Ta\x1F\xD8\x90a\x05p\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x80;\x15a\x04YW`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90` \x90\x82\x90`$\x90\x82\x90Z\xFA\x80\x15a\x04TWa \x17\x91`\0\x91a!<W[P`\tUV[`\tT`\x08T\x90\x81\x81\x10a \xD2Wa S\x7F5}\x90_\x181 \x97\x97\xDFMU\xD7\x9C\\[\xF1\xD9\xF71\x1C\x97j\xFD\x05\xE1=\x88\x1E\xAB\x9B\xC8\x92a w\x92a\x1FRV[a ga b\x82`\nTa\x1F\xB4V[`\nUV[`@Q\x90\x81R\x90\x81\x90` \x82\x01\x90V[\x03\x90\xA1`\x04Ta \x91\x90a\x05p\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\tT\x90\x80;\x15a\x04YW`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x92\x90\x92R`\0\x90\x82\x90`D\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x04TWa\x1A\xD7WPV[\x90\x7F\xB6[.\x08]}\x04\x0C1?}N\x1A\xC9\x0FY7\x02o\xEEI~\x0E$\xA7\xEF\xF1jU\xE1\xC5\xEAa!\x01a g\x84\x84a\x1FRV[\x03\x90\xA1a\x19\x9Ba!\x11\x83\x83a\x1F\x9BV[\x19`@Q\x93\x84\x93c\xB1n7\x83`\xE0\x1B\x85R`\x04\x85\x01`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[a!U\x91P` =` \x11a\x05\xF4Wa\x05\xE5\x81\x83a\r5V[8a \x11V[a!f\x81\x15\x15a\x13\x13V[\x80o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x07\x1B\x81\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x81\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x81\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x81\x81\x1C`\xFF\x10`\x03\x1B\x17\x81\x81\x1C`\x0F\x10`\x02\x1B\x17\x81\x81\x1C`\x03\x10`\x01\x1B\x17\x90\x81\x1C`\x01\x10\x17\x90V[\x90\x81g \x05\xFEO&\x8E\xA0\0\x01\x91\x82\x12`\x01\x16a\x0C\x9EWV[\x90\x81g\r\xC5R\x7Fd, \0\x01\x91\x82\x12`\x01\x16a\x0C\x9EWV[\x90\x81g\r\xE0\xB6\xB3\xA7d\0\0\x01\x91\x82\x12`\x01\x16a\x0C\x9EWV[\x90\x81g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19\x01\x91\x82\x12\x15`\x01\x16a\x0C\x9EWV[\x90\x81g\x14\xA8EL\x19\xE1\xAC\0\x01\x91\x82\x12`\x01\x16a\x0C\x9EWV[\x90\x81g\x0F\xC1\x0E\x01W\x82w\xFF\x19\x01\x91\x82\x12\x15`\x01\x16a\x0C\x9EWV[\x90\x81g\x03\xDE\xBD\x08;\x8C|\0\x01\x91\x82\x12`\x01\x16a\x0C\x9EWV[\x90\x81g\x02\x95\xD4\0\xEA2W\xFF\x19\x01\x91\x82\x12\x15`\x01\x16a\x0C\x9EWV[\x90\x81g\x01W\xD8\xB2\xEC\xC7\x08\0\x01\x91\x82\x12`\x01\x16a\x0C\x9EWV[\x90\x81g\x051\n\xA7\xD5!0\0\x01\x91\x82\x12`\x01\x16a\x0C\x9EWV[\x90\x81g\r\xE0\xCC=\x15a\0\0\x01\x91\x82\x12`\x01\x16a\x0C\x9EWV[\x91\x90\x91`\0\x83\x82\x01\x93\x84\x12\x91\x12\x90\x80\x15\x82\x16\x91\x15\x16\x17a\x0C\x9EWV[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x02\xE7Wg\x1B\xC1mgN\xC8\0\0\x90\x04\x90V[\x80\x15a$:WgV\x98\xEE\xF0fp\0\0\x81\x12\x15a\x177WgV\x98\xEE\xF0fo\xFF\xFF\x19\x81\x13\x15a$-W`\0a$\x1Da#W\x83a'vV[a#\xE0a\x16\xB7a#qa#la\x16=\x85a\"\xF7V[a&rV[\x92a$\x18a$\x13a$\x0Ea$\x07a$\x01a#\xFCa#\xF6a#\xF1a#\xEBa#\xE6\x8Da#\xE0a#\xDBa#\xD5a#\xD0a\x167a#\xCBa#\xC5a#\xC0a#\xBAa#\xB5\x8Aa'KV[a\"\x15V[\x89a'*V[a\"/V[\x87a'*V[a\"GV[a\"aV[\x83a'*V[a\"yV[\x90a'*V[a\"\x93V[\x8Ca'*V[a\"\xABV[\x8Aa'*V[a\"\xC3V[\x88a'*V[\x93\x80a'*V[a\x158V[a\x1F\x82V[a\"\xDBV[\x91\x12\x15a\x03\x97Wa\x03\x97\x90a\x1F_V[Pg\x1B\xC1mgN\xC8\0\0\x90V[Pg\r\xE0\xB6\xB3\xA7d\0\0\x90V[h\x02H\xCE6\xA7\x0C\xB2k>\x19\x81\x13\x15a\x177Wh\x07U\xBFy\x8BJ\x1B\xF1\xE5\x81\x12\x15a%\x97We\x03x-\xAC\xE9\xD9\x90`N\x1B\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x91``\x90`\x01`_\x1B\x84\x82\x84\x1B\x05\x01\x82\x1D\x93\x84\x02\x90\x03\x80l\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x81\x01\x02\x82\x1D\x90n\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dPn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x82n\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x81m\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x81m\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x81m\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x81l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x02\x8D\x1D\x01\x02\x8B\x1D\x01\x02\x89\x1D\x01\x02\x87\x1D\x01\x02\x85\x1D\x01\x93m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x93m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEA\x81\x01\x90\x84m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x91\x01\x01\x02\x90\x1D\x01\x02\x01\x05\x02\x90`\xC3\x03\x1C\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x90\xFD[g\x13\xA0K\xBD\xFD\xC9\xBE\x88\x90\x80\x82\x02\x91\x82\x05\x14`\x01\x16\x15a\x02\xE7Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x02\xE7Wg\x1B\xC1mgN\xC8\0\0\x90\x05\x90V[g\x03\xC1f\\z\xAB \0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x02\xE7Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[f\x9F2u$b\xA0\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x02\xE7Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[`\x01\x81\x15\x15\x16\x15a\x02\xE7Wn\xC0\x97\xCE{\xC9\x07\x15\xB3K\x9F\x10\0\0\0\0\x05\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17\x82\x15\x15\x16\x15a\x02\xE7W\x05\x90V[g\x1B\xC1mgN\xC7\xFF\xFF\x19\x81\x81\x02\x91`\x01\x91\x83\x05\x14\x16\x15a\x02\xE7Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\t\xD0(\xCCo _\xFF\x19\x81\x81\x02\x91`\x01\x91\x83\x05\x14\x16\x15a\x02\xE7Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x90\x80\x82\x02\x91\x82\x05\x14`\x01\x16\x15a\x02\xE7Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x02\xE7Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\x02_\x0F\xE1\x05\xA3\x14\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x02\xE7Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[`\x01`\xFF\x1B\x81\x14a'\x91W`\0\x81\x12\x15a\x03\x97W\x19`\x01\x01\x90V[`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x90\xFD\xFE\xA2dipfsX\"\x12 \xE0\xFBB\x08\xE0\xB85\x07\xEA\x9F\xD1\x004u\xAEW&\x92E\xD1Z\xA6\x19\xB2\xD1/\xC70F\xCD\x94\xFBdsolcC\0\x08\x16\x003";
    /// The bytecode of the contract.
    pub static ATOMICV2_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\x0FkW`\x005`\xE0\x1C\x80c-[l\xB9\x14a\x01\xACW\x80c6yr:\x14a\x01\xA7W\x80c7\xC6\xA4J\x14a\x01\xA2W\x80c8\xD5.\x0F\x14a\x01\x9DW\x80c9(\xFF\x97\x14a\x01\x98W\x80cI\xA7\xA2m\x14a\x01\x93W\x80cdI\xFCW\x14a\x01\x8EW\x80cgsB\xCE\x14a\x01\x89W\x80cr\xB9\x82F\x14a\x01\x84W\x80c\x85\xB3\x19\xFF\x14a\x01\x7FW\x80c\x93e \xC3\x14a\x01zW\x80c\x96\xFB\xEE\x1D\x14a\x01uW\x80c\x99\x9B\x93\xAF\x14a\x01pW\x80c\x9F'\xEFO\x14a\x01kW\x80c\xA8\xC6.v\x14a\x01fW\x80c\xAE\x97h\xA8\x14a\x01aW\x80c\xB3\xB2\xBF+\x14a\x01\\W\x80c\xBD%-\x06\x14a\x01WW\x80c\xD0\xB7\x1B\x1E\x14a\x01RW\x80c\xD2L\xE6\xE5\x14a\x01MW\x80c\xD2\xF7&Z\x14a\x01HW\x80c\xE1s\xAD%\x14a\x01CW\x80c\xE5$\xF8I\x14a\x01>W\x80c\xF3\xC9s\xCF\x14a\x019W\x80c\xF9\0^\xB5\x14a\x014Wc\xFA.Y\x94\x03a\x0FkWa\x0FMV[a\x0E\x9DV[a\x0EzV[a\x0E:V[a\rWV[a\x0C\xA3V[a\x0CFV[a\x0B\xFFV[a\x0B\xB2V[a\x06\xC2V[a\x06vV[a\x06MV[a\x06$V[a\x05\xFBV[a\x05%V[a\x05\x07V[a\x04\xE9V[a\x04\xCBV[a\x04\xADV[a\x04\x87V[a\x04^V[a\x03\x9AV[a\x02\xF7V[a\x02\xA5V[a\x02\x8DV[a\x02hV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01Ra7\xB7`\xF1\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[` \x90`\x03\x19\x01\x12a\x02cW`\x045\x90V[a\x02\x01V[4a\x02\x88W` a\x02\x80a\x02{6a\x02QV[a\x13KV[`@Q\x90\x81R\xF3[a\x01\xB1V[4a\x02\x88W` a\x02\x80a\x02\xA06a\x02QV[a\x15jV[4a\x02\x88W`@6`\x03\x19\x01\x12a\x02cW`\x045`$5\x90g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17\x82\x15\x15\x16\x15a\x02\xE7W` \x91`@Q\x91\x04\x81R\xF3[`\0\x80\xFD[`\0\x91\x03\x12a\x02cWV[4a\x02\x88W`\x006`\x03\x19\x01\x12a\x02cW`\x03T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[\x80\x15\x15\x03a\x02\xE7WV[`\0[\x83\x81\x10a\x03=WPP`\0\x91\x01RV[\x81\x81\x01Q\x83\x82\x01R` \x01a\x03-V[\x90` \x91a\x03f\x81Q\x80\x92\x81\x85R\x85\x80\x86\x01\x91\x01a\x03*V[`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[\x90\x92`\x80\x92a\x03\x97\x95\x94\x15\x15\x83R` \x83\x01R`@\x82\x01R\x81``\x82\x01R\x01\x90a\x03MV[\x90V[4a\x02\x88W``6`\x03\x19\x01\x12a\x02cW`$5a\x03\xB7\x81a\x03 V[`\x02T`\x01`\x01`\xA0\x1B\x03\x16\x90\x81;\x15a\x04YW`@Qc9(\xFF\x97`\xE0\x1B\x81R`\x04\x805\x90\x82\x01R\x90\x15\x15`$\x82\x01R`D\x805\x90\x82\x01R\x90`\0\x90\x81\x90\x83\x90`d\x90\x82\x90Z\xFA\x80\x15a\x04TW\x81\x80\x93\x81\x80\x93a\x04'W[PP\x90a\x04#\x91`@Q\x94\x85\x94\x85a\x03rV[\x03\x90\xF3[\x91P\x91Pa\x04#\x93Pa\x04L\x92P=\x80\x91\x83>a\x04D\x81\x83a\r5V[\x81\x01\x90a\x10=V[\x91\x938a\x04\x10V[a\x11\xB5V[a\x0F\xCEV[4a\x02\x88W`\x006`\x03\x19\x01\x12a\x02cW`\x02T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x02\x88W`\x006`\x03\x19\x01\x12a\x02cW` `\xFF`\x0BT`\x08\x1C\x16`@Q\x90\x15\x15\x81R\xF3[4a\x02\x88W` 6`\x03\x19\x01\x12a\x02cW` a\x02\x80`\x045a\x17=V[4a\x02\x88W`\x006`\x03\x19\x01\x12a\x02cW` `\tT`@Q\x90\x81R\xF3[4a\x02\x88W`\x006`\x03\x19\x01\x12a\x02cW` `\nT`@Q\x90\x81R\xF3[4a\x02\x88W`\x006`\x03\x19\x01\x12a\x02cW` `\x07T`@Q\x90\x81R\xF3[4a\x02\x88W`@6`\x03\x19\x01\x12a\x02cWa\x05S`$5a\x05E\x81a\x17\xFBV[`\x0BT`\x08\x1C`\xFF\x16a\x19\xFCV[`\x0BT`\xFF\x16`\x03Ta\x05|\x90a\x05p\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x90\x81;\x15a\x04YW`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x91` \x90\x83\x90`$\x90\x82\x90Z\xFA\x90\x81\x15a\x04TWa\x05\xC0\x92`\0\x92a\x05\xCAW[P`\x045a\x1C\xF5V[a\x05\xC8a\x1F\xC1V[\0[a\x05\xED\x91\x92P` =` \x11a\x05\xF4W[a\x05\xE5\x81\x83a\r5V[\x81\x01\x90a\x11\xC1V[\x908a\x05\xB7V[P=a\x05\xDBV[4a\x02\x88W`\x006`\x03\x19\x01\x12a\x02cW`\x04T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x02\x88W`\x006`\x03\x19\x01\x12a\x02cW`\0T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x02\x88W`\x006`\x03\x19\x01\x12a\x02cW`\x05T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x02\x88W`@6`\x03\x19\x01\x12a\x02cW`\x045`$5\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x02\xE7W` \x90`@Q\x90`\x01g\r\xE0\xB6\xB3\xA7d\0\0`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x81R\xF3[4a\x02\x88W` \x80`\x03\x196\x01\x12a\x02cW`\x04\x90\x815\x91`@\x92\x83Q\x83\x81\x01\x81a\x06\xEC\x82a\x11\xD0V[\x03\x91a\x07\0`\x1F\x19\x93\x84\x81\x01\x83R\x82a\r5V[Q\x90 \x90\x85Qa\x07B\x86\x82\x01\x92\x82a\x076\x85``\x90` \x81R`\t` \x82\x01Rh\x13\x1B\xD9\xD3\x9B\xDC\x9BX[`\xBA\x1B`@\x82\x01R\x01\x90V[\x03\x90\x81\x01\x83R\x82a\r5V[Q\x90 \x14a\t\xE0W[P`\0\x80T\x90\x93\x90a\x07g\x90a\x05p\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x90\x81;\x15a\x04YW\x83\x85\x92\x84\x83Q\x80\x95\x81\x93cP\x1A\xD8\xFF`\xE1\x1B\x83RZ\xF1\x91\x82\x15a\x04TW\x85\x92a\t\xC1W[P`\x03Ta\x07\xAB\x90a\x05p\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x85T\x90\x93\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x84;\x15a\x04YW\x82Qcp\xA0\x821`\xE0\x1B\x80\x82R`\x01`\x01`\xA0\x1B\x03\x84\x16\x83\x83\x01\x90\x81R\x90\x93\x91\x90\x88\x90\x82\x90\x81\x90` \x01\x03\x81\x8AZ\xFA\x90\x81\x15a\x04TW\x89\x91a\t\xA4W[P\x82Ta\x08\x15\x90a\x05p\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x95\x86;\x15a\x04YW\x85Q\x85\x81R`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x84\x84\x01\x90\x81R\x89\x90\x84\x90\x81\x90` \x01\x03\x81\x8AZ\xFA\x92\x83\x15a\x04TW\x7F\xBC}\xF0Z&tit\x1Er\xD9\x8D\xFC\x1F\xBE\xFAP*\x8D]\x8BD\t\xA6R\x13\x1B\x8E\x14\xFD,\xEE\x93a\x08\x9A\x91\x8C\x91a\t\x87W[P\x87Q\x93\x84\x93B\x90\x85\x90\x94\x93\x92``\x92`\x80\x83\x01\x96\x83R` \x83\x01R`@\x82\x01R\x01RV[\x03\x90\xA1\x84;\x15a\x04YW\x82Q\x82\x81R3\x82\x82\x01\x90\x81R\x90\x95\x87\x91\x87\x91\x90\x82\x90\x81\x90` \x01\x03\x91Z\xFA\x94\x85\x15a\x04TW\x87\x95a\thW[P\x83;\x15a\x04YW\x82Q\x91\x82R3\x90\x82\x01\x90\x81R\x90\x92\x85\x91\x84\x91\x90\x82\x90\x81\x90` \x01\x03\x91Z\xFA\x91\x82\x15a\x04TW\x7F[\x88\x97\xA4\xB3\xA7sN;\x9A8\xAFo>m5\xC4\xF5g2n#*KTJ\xDB\xD8?\x94$\xFD\x94\x86\x93a\tCW[PPQ\x91\x82R` \x82\x01RB`@\x82\x01R\x80``\x81\x01[\x03\x90\xA1\x80\xF3[a\t=\x92\x93P\x90\x81a\t`\x92\x90=\x10a\x05\xF4Wa\x05\xE5\x81\x83a\r5V[\x91\x908a\t&V[a\t\x80\x91\x95P\x86=\x88\x11a\x05\xF4Wa\x05\xE5\x81\x83a\r5V[\x938a\x08\xD0V[a\t\x9E\x91P\x8B=\x8D\x11a\x05\xF4Wa\x05\xE5\x81\x83a\r5V[8a\x08uV[a\t\xBB\x91P\x88=\x8A\x11a\x05\xF4Wa\x05\xE5\x81\x83a\r5V[8a\x07\xFEV[a\t\xD9\x91\x92P\x84=\x86\x11a\x05\xF4Wa\x05\xE5\x81\x83a\r5V[\x908a\x07\x93V[`\x02Ta\t\xF7\x90a\x05p\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x80;\x15a\x04YW\x84Q\x90c3\x85N\xFD`\xE2\x1B\x82R``\x82\x80a\n \x86\x88\x83\x01\x91\x90` \x83\x01\x92RV[\x03\x81\x84Z\xFA\x92\x83\x15a\x04TW`\0\x92`\0\x90`\0\x95a\x0BzW[P\x82;\x15a\x04YW\x87Q\x92c\x03\xB4\xD1\x03`\xE4\x1B\x84R\x87\x84\x80a\nc\x86\x8B\x83\x01\x91\x90` \x83\x01\x92RV[\x03\x81\x84Z\xFA\x93\x84\x15a\x04TW`\0\x94a\x0B[W[P\x80;\x15a\x04YW\x88Qc@\xDA\xFDa`\xE1\x1B\x81R\x87\x81\x01\x93\x84R\x92`\xA0\x91\x84\x91\x82\x90\x81\x90` \x01\x03\x91Z\xFA\x80\x15a\x04TW\x7Ft\xBB\x813\xEFY\x05\xC0\xA7\xE9\xCF\xB2\xAF,9p\xF3\xE3\xB7.\xD7\x0C\xCB\xB6'\x95\x96\xED\x86}3x\x95a\x0B!\x93`\0\x92a\x0B*W[P\x81Q\x89\x83\x01Q\x92\x8B\x01Q\x8BQ\x96\x87RB` \x88\x01R`@\x87\x01\x97\x90\x97R``\x86\x01\x93\x90\x93R`\x80\x85\x01R`\xA0\x84\x01\x91\x90\x91R`\xC0\x83\x01R`\xE0\x82\x01\x92\x90\x92R\x90\x81\x90a\x01\0\x82\x01\x90V[\x03\x90\xA18a\x07KV[a\x0BM\x91\x92P`\xA0=`\xA0\x11a\x0BTW[a\x0BE\x81\x83a\r5V[\x81\x01\x90a\x12\xABV[\x908a\n\xD6V[P=a\x0B;V[a\x0Bs\x91\x94P\x88=\x8A\x11a\x05\xF4Wa\x05\xE5\x81\x83a\r5V[\x928a\nwV[\x91\x93PPa\x0B\xA1\x91\x93P``=``\x11a\x0B\xABW[a\x0B\x99\x81\x83a\r5V[\x81\x01\x90a\x12\x90V[\x93\x91\x92\x908a\n:V[P=a\x0B\x8FV[4a\x02\x88W`@6`\x03\x19\x01\x12a\x02cW`\x045`$5g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x81\x02\x92\x81\x84\x04\x14\x90\x15\x17\x81\x15\x15\x16\x15a\x02\xE7W` \x91`\x01`@Q\x92`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x81R\xF3[4a\x02\x88W` g\x1B\xC1mgN\xC8\0\0a\x0C=a\x0C-a\x0C8g\x13\xA0K\xBD\xFD\xC9\xBE\x88a\x0C2a\x0C-6a\x02QV[a\x15\x02V[\x05a\x15YV[a#\"V[\x05`@Q\x90\x81R\xF3[4a\x02\x88Wa\x0CT6a\x02QV[a\x0C]\x81a\x15YV[\x80\x82\x02\x91`\0\x82\x12`\x01`\xFF\x1B\x82\x14\x16a\x0C\x9EW\x81\x83\x05\x14\x90\x15\x17\x15a\x0C\x9EWg\"\xC9U\"\x95T\xC1\xB6a\x0C=a\x0C-g\x1B\xC1mgN\xC8\0\0` \x94\x05a$GV[a\x14\xECV[4a\x02\x88W`\x006`\x03\x19\x01\x12a\x02cW`\x01T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[\x90`\x01\x82\x81\x1C\x92\x16\x80\x15a\x0C\xFCW[` \x83\x10\x14a\x0C\xE6WV[cNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[\x91`\x7F\x16\x91a\x0C\xDBV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\r0W`@RV[a\r\x06V[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\r0W`@RV[4a\x02\x88W`\x006`\x03\x19\x01\x12a\x02cW`@Q`\x06T`\0\x82a\rz\x83a\x0C\xCCV[\x91\x82\x82R` \x93`\x01\x90\x85`\x01\x82\x16\x91\x82`\0\x14a\x0E\x1AWPP`\x01\x14a\r\xBDW[Pa\r\xA9\x92P\x03\x83a\r5V[a\x04#`@Q\x92\x82\x84\x93\x84R\x83\x01\x90a\x03MV[\x84\x91P`\x06`\0R\x7F\xF6R\"#\x13\xE2\x84YR\x8D\x92\x0Be\x11\\\x16\xC0O>\xFC\x82\xAA\xED\xC9{\xE5\x9F?7|\r?\x90`\0\x91[\x85\x83\x10a\x0E\x02WPPa\r\xA9\x93P\x82\x01\x018a\r\x9CV[\x80T\x83\x89\x01\x85\x01R\x87\x94P\x86\x93\x90\x92\x01\x91\x81\x01a\r\xEBV[`\xFF\x19\x16\x85\x82\x01Ra\r\xA9\x95\x15\x15`\x05\x1B\x85\x01\x01\x92P8\x91Pa\r\x9C\x90PV[4a\x02\x88W`@6`\x03\x19\x01\x12a\x02cW`\x045`$5\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x02\xE7W` \x90g\r\xE0\xB6\xB3\xA7d\0\0`@Q\x91\x04\x81R\xF3[4a\x02\x88W`\x006`\x03\x19\x01\x12a\x02cW` `\xFF`\x0BT\x16`@Q\x90\x15\x15\x81R\xF3[4a\x02\x88W`@6`\x03\x19\x01\x12a\x02cWa\x0E\xCE`$5a\x0E\xBD\x81a\x17\xFBV[`\x0BT`\x08\x1C`\xFF\x16`\x045a\x1C\xF5V[`\x0BT`\xFF\x16`\x03Ta\x0E\xEB\x90a\x05p\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x90\x81;\x15a\x04YW`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x91` \x90\x83\x90`$\x90\x82\x90Z\xFA\x90\x81\x15a\x04TWa\x05\xC0\x92`\0\x92a\x0F,W[Pa\x19\xFCV[a\x0FF\x91\x92P` =` \x11a\x05\xF4Wa\x05\xE5\x81\x83a\r5V[\x908a\x0F&V[4a\x02\x88W`\x006`\x03\x19\x01\x12a\x02cW` `\x08T`@Q\x90\x81R\xF3[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01Rtnor receive functions`X\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01Rd code`\xD8\x1B`d\x82\x01R`\x84\x90\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\r0W`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\x80\x81\x83\x03\x12a\x02cW\x80Qa\x10R\x81a\x03 V[\x92` \x92\x83\x83\x01Q\x93`@\x84\x01Q\x93``\x81\x01Q\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x11eW\x01\x82`\x1F\x82\x01\x12\x15a\x11\x0CW\x80Q\x91a\x10\x8E\x83a\x10!V[\x93a\x10\x9C`@Q\x95\x86a\r5V[\x83\x85R\x81\x84\x84\x01\x01\x11a\x10\xB8Wa\x03\x97\x92\x91\x84\x82\x01\x91\x01a\x03*V[`\x84\x90`@Q\x90bF\x1B\xCD`\xE5\x1B\x82R`\x04\x82\x01R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x83\x90R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x84\x90R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[`@Q=`\0\x82>=\x90\xFD[\x90\x81` \x91\x03\x12a\x02cWQ\x90V[` \x80\x82R`\x06T`\0\x92a\x11\xE4\x82a\x0C\xCCV[\x91\x82` \x83\x01R`@`\x01\x91`\x01\x81\x16\x90\x81`\0\x14a\x12mWP`\x01\x14a\x12\x0EW[PPPPP\x90V[`\x06`\0\x90\x81R\x94\x96\x95P\x91\x92\x91\x7F\xF6R\"#\x13\xE2\x84YR\x8D\x92\x0Be\x11\\\x16\xC0O>\xFC\x82\xAA\xED\xC9{\xE5\x9F?7|\r?[\x83\x86\x10a\x12ZWPPPP`@\x92\x93P\x01\x018\x80\x80\x80\x80a\x12\x06V[\x80T\x85\x87\x01\x83\x01R\x94\x87\x01\x94\x82\x01a\x12>V[\x93\x95PPPP`@\x93P`\xFF\x19\x16\x83\x83\x01R\x15\x15`\x05\x1B\x01\x018\x80\x80\x80\x80a\x12\x06V[\x90\x81``\x91\x03\x12a\x02cW\x80Q\x91`@` \x83\x01Q\x92\x01Q\x90V[\x90\x81`\xA0\x91\x03\x12a\x02cW`@Q\x90`\xA0\x82\x01\x90\x82\x82\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x17a\r0W`\x80\x91`@R\x80Q\x83R` \x81\x01Q` \x84\x01R`@\x81\x01Q`@\x84\x01R``\x81\x01Q``\x84\x01R\x01Q`\x01\x80`\xA0\x1B\x03\x81\x16\x81\x03a\x02\xE7W`\x80\x82\x01R\x90V[\x15a\x13\x1AWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x90\xFD[}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x90a\x13w`\0\x82\x13a\x13\x13V[q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06a\x13\x93\x82a![V[``\x92\x83\x82`\x9F\x03\x01\x1B`\x9F\x1C\x90`_\x19\x01}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x02\x92l\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x82m\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x81m\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x81m\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x81m\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x81m\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x81lFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x02\x8C\x1D\x01\x02\x8A\x1D\x01\x02\x88\x1D\x01\x02\x86\x1D\x01\x02\x84\x1D\x01\x02\x82\x1D\x01\x91x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x91l\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x90\x82m\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x81l\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x81m\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x81m\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x81l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x81\x01\x02\x89\x1D\x01\x02\x87\x1D\x01\x02\x85\x1D\x01\x02\x83\x1D\x01\x02\x90\x1D\x01\x02\x01\x05\x02\x01\x01`\xAE\x1D\x90V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x90g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x81\x02\x92\x81\x84\x05\x14\x90\x15\x17\x15a\x0C\x9EWV[\x90c;\x9A\xCA\0\x91\x82\x81\x02\x92\x81\x84\x05\x14\x90\x15\x17\x15a\x0C\x9EWV[`\0\x81\x90\x03\x91\x90`\x01`\xFF\x1B\x81\x14`\x01\x16a\x0C\x9EW`\0\x19\x83\x05\x03a\x0C\x9EWV[`\x01`\xFF\x1B\x81\x14a\x0C\x9EW`\0\x03\x90V[g\x06\xF0[Y\xD3\xB2\0\0\x81\x14a\x177Wg\r\xE0\xB6\xB3\xA7d\0\0\x80\x82\x12\x15a\x16\xE1W\x81\x15a\x17\x02W`\x01\x82`\x01\x1B\x91`\x02\x93\x83\x05`\x02\x03a\x0C\x9EW`\0\x83\x12\x80\x15a\x17&W[a\x17\x14W\x82\x15a\x16\xE1Wg\x1B\xC1mgN\xC8\0\0\x83\x14a\x17\x02W\x82\x12\x91\x82\x15a\x16\xF3W\x92[a\x15\xDB\x84a%\xF2V[\x80\x15a\x16\xE1Wa\x16Ma\x16\x0Ca\x16\x07a\x16\x02a\x15\xFDa\x16R\x95\x99\x97\x96\x99a\x13KV[a&\xB3V[a\x17=V[a\x15\x1FV[a\x16Ha\x16 a\x16\x1B\x83a&\x1DV[a!\xCDV[a\x16Ba\x16=a\x167a\x162\x86a&HV[a!\xE5V[\x85a'*V[a!\xFDV[\x90a&\x91V[a\x1F\x9BV[a&\xDBV[\x93`\0\x92[\x81\x84\x10a\x16\x89WPPPPa\x03\x97\x91a\x16v\x91`\0\x14a\x16{Wa%\xCBV[a\x15YV[a\x16\x84\x90a\x15YV[a%\xCBV[\x90\x91a\x16\xD7\x86a\x16\xD1a\x16\xA1\x85a\x16H\x86\x99\x9Ba#\"V[a\x16Ba\x16\xC1a\x16\xBCa\x16\xB7a\x16v\x87\x80a'*V[a$GV[a'\x03V[a\x16\xCB\x83\x86a'*V[\x90a\x1F\x9BV[\x90a\"\xDBV[\x95\x01\x92\x91\x90a\x16WV[`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x90\xFD[a\x16\xFC\x90a\x1F_V[\x92a\x15\xD2V[`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x90\xFD[`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x90\xFD[Pg\x1B\xC1mgN\xC8\0\0\x83\x13a\x15\xAEV[P`\0\x90V[`\xB5\x81`\x01`\x88\x1B\x81\x10\x15a\x17\xE4W[\x80i\x01\0\0\0\0\0\0\0\0\0b\x01\0\0\x92\x10\x15a\x17\xD7W[e\x01\0\0\0\0\0\x81\x10\x15a\x17\xCAW[c\x01\0\0\0\x81\x10\x15a\x17\xBDW[\x01\x02`\x12\x1C`\x01\x90\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x80\x80\x92\x04\x10\x90\x03\x90V[`\x10\x1C\x91`\x08\x1B\x91a\x17\x81V[` \x1C\x91`\x10\x1B\x91a\x17tV[`@\x1C\x91` \x1B\x91a\x17eV[Ph\xB5\0\0\0\0\0\0\0\0\x90P`\x80\x82\x90\x1Ca\x17MV[`\x04\x80Ta\x18\x13\x90a\x05p\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x90\x81;\x15a\x04YW`@\x80Qcp\xA0\x821`\xE0\x1B\x80\x82R3\x82\x85\x01\x90\x81R` \x96\x91\x95\x93\x94\x93\x91\x92\x90\x87\x90\x82\x90\x81\x90\x83\x01\x03\x81\x85Z\xFA\x90\x81\x15a\x04TW`\0\x91a\x19\xDFW[P\x82\x81\x10a\x19\xBCWP\x80;\x15a\x04YW\x83Qcn\xB1v\x9F`\xE1\x1B\x81R3\x84\x82\x01\x90\x81R0` \x82\x01R\x87\x90\x82\x90\x81\x90`@\x01\x03\x81\x85Z\xFA\x90\x81\x15a\x04TW`\0\x91a\x19\x9FW[P\x82\x81\x10a\x19xWP\x80;\x15a\x04YW\x83Qc#\xB8r\xDD`\xE0\x1B\x81R3\x84\x82\x01\x90\x81R0` \x82\x01R`@\x81\x01\x93\x90\x93R\x91`\0\x91\x83\x91\x82\x90\x84\x90\x82\x90``\x01\x03\x92Z\xF1\x80\x15a\x04TWa\x19_W[P\x80Ta\x19\x04\x90a\x05p\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x80;\x15a\x04YW\x91Q\x92\x83R0\x90\x83\x01\x90\x81R\x83\x91\x83\x91\x82\x90\x81\x90` \x01\x03\x91Z\xFA\x90\x81\x15a\x04TWa\x19@\x92`\0\x92a\x19BW[PP`\x08UV[V[a\x19X\x92P\x80=\x10a\x05\xF4Wa\x05\xE5\x81\x83a\r5V[8\x80a\x199V[\x80a\x19la\x19r\x92a\r\x1CV[\x80a\x02\xECV[8a\x18\xEDV[\x84Qc\xDAV\xD3\xC5`\xE0\x1B\x81R\x80\x85\x01\x91\x82R` \x82\x01\x84\x90R\x90\x81\x90`@\x01\x03\x90\xFD[\x03\x90\xFD[a\x19\xB6\x91P\x87=\x89\x11a\x05\xF4Wa\x05\xE5\x81\x83a\r5V[8a\x18\x9EV[\x84Qc\n\xBEZ\x89`\xE0\x1B\x81R\x80\x85\x01\x91\x82R` \x82\x01\x84\x90R\x90\x81\x90`@\x01\x03\x90\xFD[a\x19\xF6\x91P\x87=\x89\x11a\x05\xF4Wa\x05\xE5\x81\x83a\r5V[8a\x18XV[\x15a\x1A\xF7W`\x03Ta\x1A\x18\x90a\x05p\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\0\x80T\x90\x91\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x80;\x15a\x04YW`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16`\x04\x83\x01R`$\x82\x01\x84\x90R\x82\x90\x82\x90`D\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x04TWa\x1A\xE4W[P\x80Ta\x1A\x84\x90a\x05p\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x03T`\x01`\x01`\xA0\x1B\x03\x16\x92\x90\x80;\x15a\x04YW`@Qc\xD0\x04\xF0\xF7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x94\x90\x94\x16`\x04\x85\x01R`$\x84\x01\x91\x90\x91R\x82\x90`D\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x04TWa\x1A\xD7WPV[\x80a\x19la\x19@\x92a\r\x1CV[\x80a\x19la\x1A\xF1\x92a\r\x1CV[8a\x1AmV[`\x04Ta\x1B\x0E\x90a\x05p\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\0\x80T\x90\x92\x91\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x80;\x15a\x04YW`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16`\x04\x83\x01R`$\x82\x01\x83\x90R\x83\x90\x82\x90`D\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x04TWa\x1CVW[P\x81Ta\x1B{\x90a\x05p\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x04T\x90\x91\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x82;\x15a\x04YW`@Qc\xD0\x04\xF0\xF7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16`\x04\x83\x01R`$\x82\x01R\x90\x82\x90\x82\x90`D\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x04TWa\x1CCW[P`\x03Ta\x1B\xE6\x90a\x05p\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x80;\x15a\x04YW`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90` \x90\x82\x90`$\x90\x82\x90Z\xFA\x90\x81\x15a\x04TWa\x19@\x92\x91a\x1C$W[P`\x07UV[a\x1C=\x91P` =` \x11a\x05\xF4Wa\x05\xE5\x81\x83a\r5V[8a\x1C\x1EV[\x80a\x19la\x1CP\x92a\r\x1CV[8a\x1B\xCEV[\x80a\x19la\x1Cc\x92a\r\x1CV[8a\x1BdV[`@\x90a\x03\x97\x93\x92\x81R\x81` \x82\x01R\x01\x90a\x03MV[=\x15a\x1C\xABW=\x90a\x1C\x91\x82a\x10!V[\x91a\x1C\x9F`@Q\x93\x84a\r5V[\x82R=`\0` \x84\x01>V[``\x90V[\x90`\x80a\x03\x97\x92`@\x81R`\x19`@\x82\x01R\x7FDEX swap failed with data\0\0\0\0\0\0\0``\x82\x01R\x81` \x82\x01R\x01\x90a\x03MV[\x81\x15a\x1E\xD0W`\x03Ta\x1D\x12\x90a\x05p\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x01T`\x01`\x01`\xA0\x1B\x03\x16\x90\x80;\x15a\x04YW`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16`\x04\x83\x01R`$\x82\x01\x85\x90R`\0\x90\x82\x90`D\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x04TWa\x1E\xBDW[P[`\x02Ta\x1D}\x90a\x05p\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x90\x81;\x15a\x04YW`@Qc9(\xFF\x97`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R\x83\x15\x15`$\x82\x01R`D\x81\x01\x94\x90\x94R`\0\x93\x91\x84\x90\x83\x90`d\x90\x82\x90Z\xFA\x91\x82\x15a\x04TW\x84\x85\x91\x86\x90\x87\x95a\x1E\x9BW[P\x81\x15a\x1E}WPP`\x01Ta\x1D\xEC\x91Pa\x05p\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x91\x82;\x15a\x04YWa\x1E\x17\x92\x85\x92\x83`@Q\x80\x96\x81\x95\x82\x94c\xBD\x06%\xAB`\xE0\x1B\x84R`\x04\x84\x01a\x1CiV[\x03\x92Z\xF1\x90\x81a\x1EjW[Pa\x1EKWa\x19\x9Ba\x1E2a\x1C\x80V[`@Qcg\xA1k\x8D`\xE1\x1B\x81R\x91\x82\x91`\x04\x83\x01a\x1C\xB0V[\x15a\x1ESWPV[`\x03Ta\x1B\xE6\x90a\x05p\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x80a\x19la\x1Ew\x92a\r\x1CV[8a\x1E\"V[\x84a\x19\x9B\x91`@Q\x94\x85\x94c\x03\x14\xE6#`\xE3\x1B\x86R`\x04\x86\x01a\x03rV[\x92PPPa\x1E\xB4\x91\x92P=\x80\x86\x83>a\x04D\x81\x83a\r5V[\x93\x92\x908a\x1D\xCBV[\x80a\x19la\x1E\xCA\x92a\r\x1CV[8a\x1DdV[`\x04Ta\x1E\xE7\x90a\x05p\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x01T`\x01`\x01`\xA0\x1B\x03\x16\x90\x80;\x15a\x04YW`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16`\x04\x83\x01R`$\x82\x01\x85\x90R`\0\x90\x82\x90`D\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x04TWa\x1F?W[Pa\x1DfV[\x80a\x19la\x1FL\x92a\r\x1CV[8a\x1F9V[\x91\x90\x82\x03\x91\x82\x11a\x0C\x9EWV[\x90g\x1B\xC1mgN\xC8\0\0`\0\x83\x82\x03\x93\x12\x81\x84\x12\x81\x16\x91\x84\x13\x90\x15\x16\x17a\x0C\x9EWV[\x90g\x11\x90\0\xAB\x10\x0F\xFB\xFF\x19\x82\x01\x91\x82\x13`\x01\x16a\x0C\x9EWV[\x81\x81\x03\x92\x91`\0\x13\x80\x15\x82\x85\x13\x16\x91\x84\x12\x16\x17a\x0C\x9EWV[\x91\x90\x82\x01\x80\x92\x11a\x0C\x9EWV[`\x04Ta\x1F\xD8\x90a\x05p\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x80;\x15a\x04YW`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90` \x90\x82\x90`$\x90\x82\x90Z\xFA\x80\x15a\x04TWa \x17\x91`\0\x91a!<W[P`\tUV[`\tT`\x08T\x90\x81\x81\x10a \xD2Wa S\x7F5}\x90_\x181 \x97\x97\xDFMU\xD7\x9C\\[\xF1\xD9\xF71\x1C\x97j\xFD\x05\xE1=\x88\x1E\xAB\x9B\xC8\x92a w\x92a\x1FRV[a ga b\x82`\nTa\x1F\xB4V[`\nUV[`@Q\x90\x81R\x90\x81\x90` \x82\x01\x90V[\x03\x90\xA1`\x04Ta \x91\x90a\x05p\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\tT\x90\x80;\x15a\x04YW`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x92\x90\x92R`\0\x90\x82\x90`D\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x04TWa\x1A\xD7WPV[\x90\x7F\xB6[.\x08]}\x04\x0C1?}N\x1A\xC9\x0FY7\x02o\xEEI~\x0E$\xA7\xEF\xF1jU\xE1\xC5\xEAa!\x01a g\x84\x84a\x1FRV[\x03\x90\xA1a\x19\x9Ba!\x11\x83\x83a\x1F\x9BV[\x19`@Q\x93\x84\x93c\xB1n7\x83`\xE0\x1B\x85R`\x04\x85\x01`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[a!U\x91P` =` \x11a\x05\xF4Wa\x05\xE5\x81\x83a\r5V[8a \x11V[a!f\x81\x15\x15a\x13\x13V[\x80o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x07\x1B\x81\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x81\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x81\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x81\x81\x1C`\xFF\x10`\x03\x1B\x17\x81\x81\x1C`\x0F\x10`\x02\x1B\x17\x81\x81\x1C`\x03\x10`\x01\x1B\x17\x90\x81\x1C`\x01\x10\x17\x90V[\x90\x81g \x05\xFEO&\x8E\xA0\0\x01\x91\x82\x12`\x01\x16a\x0C\x9EWV[\x90\x81g\r\xC5R\x7Fd, \0\x01\x91\x82\x12`\x01\x16a\x0C\x9EWV[\x90\x81g\r\xE0\xB6\xB3\xA7d\0\0\x01\x91\x82\x12`\x01\x16a\x0C\x9EWV[\x90\x81g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19\x01\x91\x82\x12\x15`\x01\x16a\x0C\x9EWV[\x90\x81g\x14\xA8EL\x19\xE1\xAC\0\x01\x91\x82\x12`\x01\x16a\x0C\x9EWV[\x90\x81g\x0F\xC1\x0E\x01W\x82w\xFF\x19\x01\x91\x82\x12\x15`\x01\x16a\x0C\x9EWV[\x90\x81g\x03\xDE\xBD\x08;\x8C|\0\x01\x91\x82\x12`\x01\x16a\x0C\x9EWV[\x90\x81g\x02\x95\xD4\0\xEA2W\xFF\x19\x01\x91\x82\x12\x15`\x01\x16a\x0C\x9EWV[\x90\x81g\x01W\xD8\xB2\xEC\xC7\x08\0\x01\x91\x82\x12`\x01\x16a\x0C\x9EWV[\x90\x81g\x051\n\xA7\xD5!0\0\x01\x91\x82\x12`\x01\x16a\x0C\x9EWV[\x90\x81g\r\xE0\xCC=\x15a\0\0\x01\x91\x82\x12`\x01\x16a\x0C\x9EWV[\x91\x90\x91`\0\x83\x82\x01\x93\x84\x12\x91\x12\x90\x80\x15\x82\x16\x91\x15\x16\x17a\x0C\x9EWV[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x02\xE7Wg\x1B\xC1mgN\xC8\0\0\x90\x04\x90V[\x80\x15a$:WgV\x98\xEE\xF0fp\0\0\x81\x12\x15a\x177WgV\x98\xEE\xF0fo\xFF\xFF\x19\x81\x13\x15a$-W`\0a$\x1Da#W\x83a'vV[a#\xE0a\x16\xB7a#qa#la\x16=\x85a\"\xF7V[a&rV[\x92a$\x18a$\x13a$\x0Ea$\x07a$\x01a#\xFCa#\xF6a#\xF1a#\xEBa#\xE6\x8Da#\xE0a#\xDBa#\xD5a#\xD0a\x167a#\xCBa#\xC5a#\xC0a#\xBAa#\xB5\x8Aa'KV[a\"\x15V[\x89a'*V[a\"/V[\x87a'*V[a\"GV[a\"aV[\x83a'*V[a\"yV[\x90a'*V[a\"\x93V[\x8Ca'*V[a\"\xABV[\x8Aa'*V[a\"\xC3V[\x88a'*V[\x93\x80a'*V[a\x158V[a\x1F\x82V[a\"\xDBV[\x91\x12\x15a\x03\x97Wa\x03\x97\x90a\x1F_V[Pg\x1B\xC1mgN\xC8\0\0\x90V[Pg\r\xE0\xB6\xB3\xA7d\0\0\x90V[h\x02H\xCE6\xA7\x0C\xB2k>\x19\x81\x13\x15a\x177Wh\x07U\xBFy\x8BJ\x1B\xF1\xE5\x81\x12\x15a%\x97We\x03x-\xAC\xE9\xD9\x90`N\x1B\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x91``\x90`\x01`_\x1B\x84\x82\x84\x1B\x05\x01\x82\x1D\x93\x84\x02\x90\x03\x80l\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x81\x01\x02\x82\x1D\x90n\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dPn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x82n\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x81m\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x81m\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x81m\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x81l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x02\x8D\x1D\x01\x02\x8B\x1D\x01\x02\x89\x1D\x01\x02\x87\x1D\x01\x02\x85\x1D\x01\x93m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x93m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEA\x81\x01\x90\x84m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x91\x01\x01\x02\x90\x1D\x01\x02\x01\x05\x02\x90`\xC3\x03\x1C\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x90\xFD[g\x13\xA0K\xBD\xFD\xC9\xBE\x88\x90\x80\x82\x02\x91\x82\x05\x14`\x01\x16\x15a\x02\xE7Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x02\xE7Wg\x1B\xC1mgN\xC8\0\0\x90\x05\x90V[g\x03\xC1f\\z\xAB \0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x02\xE7Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[f\x9F2u$b\xA0\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x02\xE7Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[`\x01\x81\x15\x15\x16\x15a\x02\xE7Wn\xC0\x97\xCE{\xC9\x07\x15\xB3K\x9F\x10\0\0\0\0\x05\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17\x82\x15\x15\x16\x15a\x02\xE7W\x05\x90V[g\x1B\xC1mgN\xC7\xFF\xFF\x19\x81\x81\x02\x91`\x01\x91\x83\x05\x14\x16\x15a\x02\xE7Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\t\xD0(\xCCo _\xFF\x19\x81\x81\x02\x91`\x01\x91\x83\x05\x14\x16\x15a\x02\xE7Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x90\x80\x82\x02\x91\x82\x05\x14`\x01\x16\x15a\x02\xE7Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x02\xE7Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\x02_\x0F\xE1\x05\xA3\x14\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x02\xE7Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[`\x01`\xFF\x1B\x81\x14a'\x91W`\0\x81\x12\x15a\x03\x97W\x19`\x01\x01\x90V[`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x90\xFD\xFE\xA2dipfsX\"\x12 \xE0\xFBB\x08\xE0\xB85\x07\xEA\x9F\xD1\x004u\xAEW&\x92E\xD1Z\xA6\x19\xB2\xD1/\xC70F\xCD\x94\xFBdsolcC\0\x08\x16\x003";
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
        ///Calls the contract's `cdf` (0xd0b71b1e) function
        pub fn cdf(
            &self,
            input: ::ethers::core::types::I256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([208, 183, 27, 30], input)
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
        ///Calls the contract's `divWadDown` (0x37c6a44a) function
        pub fn div_wad_down(
            &self,
            x: ::ethers::core::types::U256,
            y: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([55, 198, 164, 74], (x, y))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `divWadUp` (0xbd252d06) function
        pub fn div_wad_up(
            &self,
            x: ::ethers::core::types::U256,
            y: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([189, 37, 45, 6], (x, y))
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
        ///Calls the contract's `log` (0x2d5b6cb9) function
        pub fn log(
            &self,
            x: ::ethers::core::types::I256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([45, 91, 108, 185], x)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `logData` (0xb3b2bf2b) function
        pub fn log_data(
            &self,
            pool_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([179, 178, 191, 43], pool_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lower_exchange_price` (0x96fbee1d) function
        pub fn lower_exchange_price(
            &self,
            pool_id: ::ethers::core::types::U256,
            input: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([150, 251, 238, 29], (pool_id, input))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mulWadDown` (0xe524f849) function
        pub fn mul_wad_down(
            &self,
            x: ::ethers::core::types::U256,
            y: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([229, 36, 248, 73], (x, y))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mulWadUp` (0xae9768a8) function
        pub fn mul_wad_up(
            &self,
            x: ::ethers::core::types::U256,
            y: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([174, 151, 104, 168], (x, y))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pdf` (0xd24ce6e5) function
        pub fn pdf(
            &self,
            input: ::ethers::core::types::I256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([210, 76, 230, 229], input)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ppf` (0x3679723a) function
        pub fn ppf(
            &self,
            input: ::ethers::core::types::I256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([54, 121, 114, 58], input)
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
        ///Calls the contract's `raise_exchange_price` (0xf9005eb5) function
        pub fn raise_exchange_price(
            &self,
            pool_id: ::ethers::core::types::U256,
            input: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([249, 0, 94, 181], (pool_id, input))
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
        ///Calls the contract's `solver` (0x49a7a26d) function
        pub fn solver(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([73, 167, 162, 109], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sqrt` (0x677342ce) function
        pub fn sqrt(
            &self,
            x: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([103, 115, 66, 206], x)
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
        ///Calls the contract's `strategyName` (0xe173ad25) function
        pub fn strategy_name(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([225, 115, 173, 37], ())
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `LogArbData` event
        pub fn log_arb_data_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogArbDataFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `LogAssetData` event
        pub fn log_asset_data_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogAssetDataFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `LogDfmmData` event
        pub fn log_dfmm_data_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogDfmmDataFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Loss` event
        pub fn loss_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LossFilter> {
            self.0.event()
        }
        ///Gets the contract's `Price` event
        pub fn price_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, PriceFilter> {
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
        Hash
    )]
    #[etherror(name = "AttemptedProfit", abi = "AttemptedProfit(int256)")]
    pub struct AttemptedProfit {
        pub profit: ::ethers::core::types::I256,
    }
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
    ///Custom Error type `Infinity` with signature `Infinity()` and selector `0x07a02127`
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
    #[etherror(name = "Infinity", abi = "Infinity()")]
    pub struct Infinity;
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
    ///Custom Error type `NegativeInfinity` with signature `NegativeInfinity()` and selector `0x8bb56614`
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
    #[etherror(name = "NegativeInfinity", abi = "NegativeInfinity()")]
    pub struct NegativeInfinity;
    ///Custom Error type `OutOfBounds` with signature `OutOfBounds()` and selector `0xb4120f14`
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
    #[etherror(name = "OutOfBounds", abi = "OutOfBounds()")]
    pub struct OutOfBounds;
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
        AttemptedProfit(AttemptedProfit),
        DexSwapFailure(DexSwapFailure),
        Infinity(Infinity),
        InsufficientApprovalY(InsufficientApprovalY),
        InsufficientBalanceX(InsufficientBalanceX),
        InsufficientBalanceY(InsufficientBalanceY),
        Min(Min),
        NegativeInfinity(NegativeInfinity),
        OutOfBounds(OutOfBounds),
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
            if let Ok(decoded) = <AttemptedProfit as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AttemptedProfit(decoded));
            }
            if let Ok(decoded) = <DexSwapFailure as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DexSwapFailure(decoded));
            }
            if let Ok(decoded) = <Infinity as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Infinity(decoded));
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
            if let Ok(decoded) = <Min as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Min(decoded));
            }
            if let Ok(decoded) = <NegativeInfinity as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NegativeInfinity(decoded));
            }
            if let Ok(decoded) = <OutOfBounds as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OutOfBounds(decoded));
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
                Self::AttemptedProfit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DexSwapFailure(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Infinity(element) => {
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
                Self::Min(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NegativeInfinity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OutOfBounds(element) => {
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
                    == <AttemptedProfit as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <DexSwapFailure as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Infinity as ::ethers::contract::EthError>::selector() => true,
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
                _ if selector == <Min as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NegativeInfinity as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OutOfBounds as ::ethers::contract::EthError>::selector() => true,
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
                Self::AttemptedProfit(element) => ::core::fmt::Display::fmt(element, f),
                Self::DexSwapFailure(element) => ::core::fmt::Display::fmt(element, f),
                Self::Infinity(element) => ::core::fmt::Display::fmt(element, f),
                Self::InsufficientApprovalY(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InsufficientBalanceX(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InsufficientBalanceY(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Min(element) => ::core::fmt::Display::fmt(element, f),
                Self::NegativeInfinity(element) => ::core::fmt::Display::fmt(element, f),
                Self::OutOfBounds(element) => ::core::fmt::Display::fmt(element, f),
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
    impl ::core::convert::From<AttemptedProfit> for AtomicV2Errors {
        fn from(value: AttemptedProfit) -> Self {
            Self::AttemptedProfit(value)
        }
    }
    impl ::core::convert::From<DexSwapFailure> for AtomicV2Errors {
        fn from(value: DexSwapFailure) -> Self {
            Self::DexSwapFailure(value)
        }
    }
    impl ::core::convert::From<Infinity> for AtomicV2Errors {
        fn from(value: Infinity) -> Self {
            Self::Infinity(value)
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
    impl ::core::convert::From<Min> for AtomicV2Errors {
        fn from(value: Min) -> Self {
            Self::Min(value)
        }
    }
    impl ::core::convert::From<NegativeInfinity> for AtomicV2Errors {
        fn from(value: NegativeInfinity) -> Self {
            Self::NegativeInfinity(value)
        }
    }
    impl ::core::convert::From<OutOfBounds> for AtomicV2Errors {
        fn from(value: OutOfBounds) -> Self {
            Self::OutOfBounds(value)
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
    #[ethevent(name = "LogArbData", abi = "LogArbData(uint256,uint256,uint256)")]
    pub struct LogArbDataFilter {
        pub x_balance: ::ethers::core::types::U256,
        pub y_balance: ::ethers::core::types::U256,
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
        name = "LogAssetData",
        abi = "LogAssetData(uint256,uint256,uint256,uint256)"
    )]
    pub struct LogAssetDataFilter {
        pub lex_price: ::ethers::core::types::U256,
        pub timestamp: ::ethers::core::types::U256,
        pub rx: ::ethers::core::types::U256,
        pub ry: ::ethers::core::types::U256,
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
        name = "LogDfmmData",
        abi = "LogDfmmData(uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256)"
    )]
    pub struct LogDfmmDataFilter {
        pub price: ::ethers::core::types::U256,
        pub timestamp: ::ethers::core::types::U256,
        pub rx: ::ethers::core::types::U256,
        pub ry: ::ethers::core::types::U256,
        pub liq: ::ethers::core::types::U256,
        pub strike: ::ethers::core::types::U256,
        pub sigma: ::ethers::core::types::U256,
        pub tau: ::ethers::core::types::U256,
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
    #[ethevent(name = "Price", abi = "Price(uint256,uint256)")]
    pub struct PriceFilter {
        pub price: ::ethers::core::types::U256,
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
        LogArbDataFilter(LogArbDataFilter),
        LogAssetDataFilter(LogAssetDataFilter),
        LogDfmmDataFilter(LogDfmmDataFilter),
        LossFilter(LossFilter),
        PriceFilter(PriceFilter),
        ProfitFilter(ProfitFilter),
    }
    impl ::ethers::contract::EthLogDecode for AtomicV2Events {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = LogArbDataFilter::decode_log(log) {
                return Ok(AtomicV2Events::LogArbDataFilter(decoded));
            }
            if let Ok(decoded) = LogAssetDataFilter::decode_log(log) {
                return Ok(AtomicV2Events::LogAssetDataFilter(decoded));
            }
            if let Ok(decoded) = LogDfmmDataFilter::decode_log(log) {
                return Ok(AtomicV2Events::LogDfmmDataFilter(decoded));
            }
            if let Ok(decoded) = LossFilter::decode_log(log) {
                return Ok(AtomicV2Events::LossFilter(decoded));
            }
            if let Ok(decoded) = PriceFilter::decode_log(log) {
                return Ok(AtomicV2Events::PriceFilter(decoded));
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
                Self::LogArbDataFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogAssetDataFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LogDfmmDataFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LossFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::PriceFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProfitFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<LogArbDataFilter> for AtomicV2Events {
        fn from(value: LogArbDataFilter) -> Self {
            Self::LogArbDataFilter(value)
        }
    }
    impl ::core::convert::From<LogAssetDataFilter> for AtomicV2Events {
        fn from(value: LogAssetDataFilter) -> Self {
            Self::LogAssetDataFilter(value)
        }
    }
    impl ::core::convert::From<LogDfmmDataFilter> for AtomicV2Events {
        fn from(value: LogDfmmDataFilter) -> Self {
            Self::LogDfmmDataFilter(value)
        }
    }
    impl ::core::convert::From<LossFilter> for AtomicV2Events {
        fn from(value: LossFilter) -> Self {
            Self::LossFilter(value)
        }
    }
    impl ::core::convert::From<PriceFilter> for AtomicV2Events {
        fn from(value: PriceFilter) -> Self {
            Self::PriceFilter(value)
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
    ///Container type for all input parameters for the `cdf` function with signature `cdf(int256)` and selector `0xd0b71b1e`
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
    #[ethcall(name = "cdf", abi = "cdf(int256)")]
    pub struct CdfCall {
        pub input: ::ethers::core::types::I256,
    }
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
    ///Container type for all input parameters for the `divWadDown` function with signature `divWadDown(uint256,uint256)` and selector `0x37c6a44a`
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
    #[ethcall(name = "divWadDown", abi = "divWadDown(uint256,uint256)")]
    pub struct DivWadDownCall {
        pub x: ::ethers::core::types::U256,
        pub y: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `divWadUp` function with signature `divWadUp(uint256,uint256)` and selector `0xbd252d06`
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
    #[ethcall(name = "divWadUp", abi = "divWadUp(uint256,uint256)")]
    pub struct DivWadUpCall {
        pub x: ::ethers::core::types::U256,
        pub y: ::ethers::core::types::U256,
    }
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
    ///Container type for all input parameters for the `log` function with signature `log(int256)` and selector `0x2d5b6cb9`
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
    #[ethcall(name = "log", abi = "log(int256)")]
    pub struct LogCall {
        pub x: ::ethers::core::types::I256,
    }
    ///Container type for all input parameters for the `logData` function with signature `logData(uint256)` and selector `0xb3b2bf2b`
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
    #[ethcall(name = "logData", abi = "logData(uint256)")]
    pub struct LogDataCall {
        pub pool_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `lower_exchange_price` function with signature `lower_exchange_price(uint256,uint256)` and selector `0x96fbee1d`
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
        name = "lower_exchange_price",
        abi = "lower_exchange_price(uint256,uint256)"
    )]
    pub struct LowerExchangePriceCall {
        pub pool_id: ::ethers::core::types::U256,
        pub input: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `mulWadDown` function with signature `mulWadDown(uint256,uint256)` and selector `0xe524f849`
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
    #[ethcall(name = "mulWadDown", abi = "mulWadDown(uint256,uint256)")]
    pub struct MulWadDownCall {
        pub x: ::ethers::core::types::U256,
        pub y: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `mulWadUp` function with signature `mulWadUp(uint256,uint256)` and selector `0xae9768a8`
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
    #[ethcall(name = "mulWadUp", abi = "mulWadUp(uint256,uint256)")]
    pub struct MulWadUpCall {
        pub x: ::ethers::core::types::U256,
        pub y: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `pdf` function with signature `pdf(int256)` and selector `0xd24ce6e5`
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
    #[ethcall(name = "pdf", abi = "pdf(int256)")]
    pub struct PdfCall {
        pub input: ::ethers::core::types::I256,
    }
    ///Container type for all input parameters for the `ppf` function with signature `ppf(int256)` and selector `0x3679723a`
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
    #[ethcall(name = "ppf", abi = "ppf(int256)")]
    pub struct PpfCall {
        pub input: ::ethers::core::types::I256,
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
    ///Container type for all input parameters for the `raise_exchange_price` function with signature `raise_exchange_price(uint256,uint256)` and selector `0xf9005eb5`
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
        name = "raise_exchange_price",
        abi = "raise_exchange_price(uint256,uint256)"
    )]
    pub struct RaiseExchangePriceCall {
        pub pool_id: ::ethers::core::types::U256,
        pub input: ::ethers::core::types::U256,
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
    ///Container type for all input parameters for the `solver` function with signature `solver()` and selector `0x49a7a26d`
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
    #[ethcall(name = "solver", abi = "solver()")]
    pub struct SolverCall;
    ///Container type for all input parameters for the `sqrt` function with signature `sqrt(uint256)` and selector `0x677342ce`
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
    #[ethcall(name = "sqrt", abi = "sqrt(uint256)")]
    pub struct SqrtCall {
        pub x: ::ethers::core::types::U256,
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
    ///Container type for all input parameters for the `strategyName` function with signature `strategyName()` and selector `0xe173ad25`
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
    #[ethcall(name = "strategyName", abi = "strategyName()")]
    pub struct StrategyNameCall;
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
        Cdf(CdfCall),
        CumulativeProfit(CumulativeProfitCall),
        DivWadDown(DivWadDownCall),
        DivWadUp(DivWadUpCall),
        Exchange(ExchangeCall),
        IntermediateTokenXBalance(IntermediateTokenXBalanceCall),
        IntermediateTokenYEndBalance(IntermediateTokenYEndBalanceCall),
        IntermediateTokenYStartBalance(IntermediateTokenYStartBalanceCall),
        LiquidExchange(LiquidExchangeCall),
        Log(LogCall),
        LogData(LogDataCall),
        LowerExchangePrice(LowerExchangePriceCall),
        MulWadDown(MulWadDownCall),
        MulWadUp(MulWadUpCall),
        Pdf(PdfCall),
        Ppf(PpfCall),
        Quote(QuoteCall),
        RaiseExchangePrice(RaiseExchangePriceCall),
        SimulateSwap(SimulateSwapCall),
        Solver(SolverCall),
        Sqrt(SqrtCall),
        Strategy(StrategyCall),
        StrategyName(StrategyNameCall),
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
            if let Ok(decoded) = <CdfCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Cdf(decoded));
            }
            if let Ok(decoded) = <CumulativeProfitCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CumulativeProfit(decoded));
            }
            if let Ok(decoded) = <DivWadDownCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DivWadDown(decoded));
            }
            if let Ok(decoded) = <DivWadUpCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DivWadUp(decoded));
            }
            if let Ok(decoded) = <ExchangeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Exchange(decoded));
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
            if let Ok(decoded) = <LogCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log(decoded));
            }
            if let Ok(decoded) = <LogDataCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LogData(decoded));
            }
            if let Ok(decoded) = <LowerExchangePriceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LowerExchangePrice(decoded));
            }
            if let Ok(decoded) = <MulWadDownCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MulWadDown(decoded));
            }
            if let Ok(decoded) = <MulWadUpCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MulWadUp(decoded));
            }
            if let Ok(decoded) = <PdfCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Pdf(decoded));
            }
            if let Ok(decoded) = <PpfCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Ppf(decoded));
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
            if let Ok(decoded) = <SimulateSwapCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SimulateSwap(decoded));
            }
            if let Ok(decoded) = <SolverCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Solver(decoded));
            }
            if let Ok(decoded) = <SqrtCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Sqrt(decoded));
            }
            if let Ok(decoded) = <StrategyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Strategy(decoded));
            }
            if let Ok(decoded) = <StrategyNameCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::StrategyName(decoded));
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
                Self::Cdf(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CumulativeProfit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DivWadDown(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DivWadUp(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Exchange(element) => {
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
                Self::Log(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LogData(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LowerExchangePrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MulWadDown(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MulWadUp(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Pdf(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Ppf(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Quote(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RaiseExchangePrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SimulateSwap(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Solver(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Sqrt(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Strategy(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StrategyName(element) => {
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
                Self::Cdf(element) => ::core::fmt::Display::fmt(element, f),
                Self::CumulativeProfit(element) => ::core::fmt::Display::fmt(element, f),
                Self::DivWadDown(element) => ::core::fmt::Display::fmt(element, f),
                Self::DivWadUp(element) => ::core::fmt::Display::fmt(element, f),
                Self::Exchange(element) => ::core::fmt::Display::fmt(element, f),
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
                Self::Log(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogData(element) => ::core::fmt::Display::fmt(element, f),
                Self::LowerExchangePrice(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MulWadDown(element) => ::core::fmt::Display::fmt(element, f),
                Self::MulWadUp(element) => ::core::fmt::Display::fmt(element, f),
                Self::Pdf(element) => ::core::fmt::Display::fmt(element, f),
                Self::Ppf(element) => ::core::fmt::Display::fmt(element, f),
                Self::Quote(element) => ::core::fmt::Display::fmt(element, f),
                Self::RaiseExchangePrice(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SimulateSwap(element) => ::core::fmt::Display::fmt(element, f),
                Self::Solver(element) => ::core::fmt::Display::fmt(element, f),
                Self::Sqrt(element) => ::core::fmt::Display::fmt(element, f),
                Self::Strategy(element) => ::core::fmt::Display::fmt(element, f),
                Self::StrategyName(element) => ::core::fmt::Display::fmt(element, f),
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
    impl ::core::convert::From<CdfCall> for AtomicV2Calls {
        fn from(value: CdfCall) -> Self {
            Self::Cdf(value)
        }
    }
    impl ::core::convert::From<CumulativeProfitCall> for AtomicV2Calls {
        fn from(value: CumulativeProfitCall) -> Self {
            Self::CumulativeProfit(value)
        }
    }
    impl ::core::convert::From<DivWadDownCall> for AtomicV2Calls {
        fn from(value: DivWadDownCall) -> Self {
            Self::DivWadDown(value)
        }
    }
    impl ::core::convert::From<DivWadUpCall> for AtomicV2Calls {
        fn from(value: DivWadUpCall) -> Self {
            Self::DivWadUp(value)
        }
    }
    impl ::core::convert::From<ExchangeCall> for AtomicV2Calls {
        fn from(value: ExchangeCall) -> Self {
            Self::Exchange(value)
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
    impl ::core::convert::From<LogCall> for AtomicV2Calls {
        fn from(value: LogCall) -> Self {
            Self::Log(value)
        }
    }
    impl ::core::convert::From<LogDataCall> for AtomicV2Calls {
        fn from(value: LogDataCall) -> Self {
            Self::LogData(value)
        }
    }
    impl ::core::convert::From<LowerExchangePriceCall> for AtomicV2Calls {
        fn from(value: LowerExchangePriceCall) -> Self {
            Self::LowerExchangePrice(value)
        }
    }
    impl ::core::convert::From<MulWadDownCall> for AtomicV2Calls {
        fn from(value: MulWadDownCall) -> Self {
            Self::MulWadDown(value)
        }
    }
    impl ::core::convert::From<MulWadUpCall> for AtomicV2Calls {
        fn from(value: MulWadUpCall) -> Self {
            Self::MulWadUp(value)
        }
    }
    impl ::core::convert::From<PdfCall> for AtomicV2Calls {
        fn from(value: PdfCall) -> Self {
            Self::Pdf(value)
        }
    }
    impl ::core::convert::From<PpfCall> for AtomicV2Calls {
        fn from(value: PpfCall) -> Self {
            Self::Ppf(value)
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
    impl ::core::convert::From<SimulateSwapCall> for AtomicV2Calls {
        fn from(value: SimulateSwapCall) -> Self {
            Self::SimulateSwap(value)
        }
    }
    impl ::core::convert::From<SolverCall> for AtomicV2Calls {
        fn from(value: SolverCall) -> Self {
            Self::Solver(value)
        }
    }
    impl ::core::convert::From<SqrtCall> for AtomicV2Calls {
        fn from(value: SqrtCall) -> Self {
            Self::Sqrt(value)
        }
    }
    impl ::core::convert::From<StrategyCall> for AtomicV2Calls {
        fn from(value: StrategyCall) -> Self {
            Self::Strategy(value)
        }
    }
    impl ::core::convert::From<StrategyNameCall> for AtomicV2Calls {
        fn from(value: StrategyNameCall) -> Self {
            Self::StrategyName(value)
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
    ///Container type for all return fields from the `cdf` function with signature `cdf(int256)` and selector `0xd0b71b1e`
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
    pub struct CdfReturn {
        pub output: ::ethers::core::types::I256,
    }
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
    ///Container type for all return fields from the `divWadDown` function with signature `divWadDown(uint256,uint256)` and selector `0x37c6a44a`
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
    pub struct DivWadDownReturn {
        pub z: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `divWadUp` function with signature `divWadUp(uint256,uint256)` and selector `0xbd252d06`
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
    pub struct DivWadUpReturn {
        pub z: ::ethers::core::types::U256,
    }
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
    ///Container type for all return fields from the `log` function with signature `log(int256)` and selector `0x2d5b6cb9`
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
    pub struct LogReturn {
        pub z: ::ethers::core::types::I256,
    }
    ///Container type for all return fields from the `mulWadDown` function with signature `mulWadDown(uint256,uint256)` and selector `0xe524f849`
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
    pub struct MulWadDownReturn {
        pub z: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `mulWadUp` function with signature `mulWadUp(uint256,uint256)` and selector `0xae9768a8`
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
    pub struct MulWadUpReturn {
        pub z: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `pdf` function with signature `pdf(int256)` and selector `0xd24ce6e5`
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
    pub struct PdfReturn {
        pub output: ::ethers::core::types::I256,
    }
    ///Container type for all return fields from the `ppf` function with signature `ppf(int256)` and selector `0x3679723a`
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
    pub struct PpfReturn {
        pub output: ::ethers::core::types::I256,
    }
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
    pub struct SimulateSwapReturn {
        pub valid: bool,
        pub estimated_out: ::ethers::core::types::U256,
        pub estimated_price: ::ethers::core::types::U256,
        pub payload: ::ethers::core::types::Bytes,
    }
    ///Container type for all return fields from the `solver` function with signature `solver()` and selector `0x49a7a26d`
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
    pub struct SolverReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `sqrt` function with signature `sqrt(uint256)` and selector `0x677342ce`
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
    pub struct SqrtReturn {
        pub z: ::ethers::core::types::U256,
    }
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
    ///Container type for all return fields from the `strategyName` function with signature `strategyName()` and selector `0xe173ad25`
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
    pub struct StrategyNameReturn(pub ::std::string::String);
}
