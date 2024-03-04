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
    const __BYTECODE: &[u8] = b"`\x80`@R`\x0B\x80Ta\xFF\xFF\x19\x16`\x01\x17\x90U4\x80\x15b\0\0lW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`@Qb\x006i8\x03\x80b\x006i\x839\x81\x01`@\x81\x90Rb\0\0\x8F\x91b\0\x02\xEEV[`\x02\x80T`\x01`\x01`\xA0\x1B\x03\x80\x88\x16`\x01`\x01`\xA0\x1B\x03\x19\x92\x83\x16\x81\x17\x90\x93U`\x01\x80T\x88\x83\x16\x90\x84\x16\x17\x90U`\0\x80T\x87\x83\x16\x90\x84\x16\x17\x90U`\x03\x80T\x86\x83\x16\x90\x84\x16\x17\x90U`\x04\x80T\x91\x85\x16\x91\x90\x92\x16\x17\x81U`@\x80QcTc\x17;`\xE1\x1B\x81R\x90Qc\xA8\xC6.v\x92\x82\x81\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15b\0\x01WW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` b\x006I\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15b\0\x01lW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x01\x92\x91\x90b\0\x03cV[`\x05\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x82\x17\x90U`@\x80Qc\x06\xFD\xDE\x03`\xE0\x1B\x81R\x90Qc\x06\xFD\xDE\x03\x91`\x04\x80\x82\x01\x92`\0\x92\x90\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15b\0\x02'W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` b\x006I\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15b\0\x02<W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Rb\0\x02f\x91\x90\x81\x01\x90b\0\x03\xC9V[`\x06\x90b\0\x02u\x90\x82b\0\x06\x07V[PPPPPPb\0\x06\xD3V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x02\xE9W`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15b\0\x03\x0CWb\0\x03\x0Cb\0\x02\x81V[b\0\x03\x17\x86b\0\x02\xD1V[\x94Pb\0\x03'` \x87\x01b\0\x02\xD1V[\x93Pb\0\x037`@\x87\x01b\0\x02\xD1V[\x92Pb\0\x03G``\x87\x01b\0\x02\xD1V[\x91Pb\0\x03W`\x80\x87\x01b\0\x02\xD1V[\x90P\x92\x95P\x92\x95\x90\x93PV[`\0` \x82\x84\x03\x12\x15b\0\x03{Wb\0\x03{b\0\x02\x81V[b\0\x03\x86\x82b\0\x02\xD1V[\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0[\x83\x81\x10\x15b\0\x03\xC0W\x81\x81\x01Q\x83\x82\x01R` \x01b\0\x03\xA6V[PP`\0\x91\x01RV[`\0` \x80\x83\x85\x03\x12\x15b\0\x03\xE2Wb\0\x03\xE2b\0\x02\x81V[\x82Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15b\0\x04EW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x84\x90R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12b\0\x04\xAEW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x84\x90R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x81\xFD[\x81Q\x81\x81\x11\x15b\0\x04\xC3Wb\0\x04\xC3b\0\x03\x8DV[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15b\0\x04\xEEWb\0\x04\xEEb\0\x03\x8DV[\x81`@R\x82\x81R\x88\x86\x84\x87\x01\x01\x11\x15b\0\x05YW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x87\x90R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R\x93P`\x84\x84\xFD[b\0\x05j\x83\x87\x83\x01\x88\x88\x01b\0\x03\xA3V[\x98\x97PPPPPPPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80b\0\x05\x8BW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03b\0\x05\xACWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15b\0\x06\x02W`\0\x81`\0R` `\0 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15b\0\x05\xDDWP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15b\0\x05\xFEW\x82\x81U`\x01\x01b\0\x05\xE9V[PPP[PPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15b\0\x06#Wb\0\x06#b\0\x03\x8DV[b\0\x06;\x81b\0\x064\x84Tb\0\x05vV[\x84b\0\x05\xB2V[` \x80`\x1F\x83\x11`\x01\x81\x14b\0\x06sW`\0\x84\x15b\0\x06ZWP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ub\0\x05\xFEV[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15b\0\x06\xA4W\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01b\0\x06\x83V[P\x85\x82\x10\x15b\0\x06\xC3W\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[a/f\x80b\0\x06\xE3`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x01\xDBW`\x005`\xE0\x1C\x80c\x9F'\xEFO\x11a\x01+W\x80c\xD2L\xE6\xE5\x11a\0\xE4W\x80c\xE5$\xF8I\x11a\0\xBEW\x80c\xE5$\xF8I\x14a\x04\x12W\x80c\xF3\xC9s\xCF\x14a\x04%W\x80c\xF9\0^\xB5\x14a\x042W\x80c\xFA.Y\x94\x14a\x04EWa\x01\xDBV[\x80c\xD2L\xE6\xE5\x14a\x03\xD7W\x80c\xD2\xF7&Z\x14a\x03\xEAW\x80c\xE1s\xAD%\x14a\x03\xFDWa\x01\xDBV[\x80c\x9F'\xEFO\x14a\x03eW\x80c\xA8\xC6.v\x14a\x03xW\x80c\xAE\x97h\xA8\x14a\x03\x8BW\x80c\xB3\xB2\xBF+\x14a\x03\x9EW\x80c\xBD%-\x06\x14a\x03\xB1W\x80c\xD0\xB7\x1B\x1E\x14a\x03\xC4Wa\x01\xDBV[\x80cdI\xFCW\x11a\x01\x98W\x80c\x85\xB3\x19\xFF\x11a\x01rW\x80c\x85\xB3\x19\xFF\x14a\x03+W\x80c\x93e \xC3\x14a\x034W\x80c\x96\xFB\xEE\x1D\x14a\x03=W\x80c\x99\x9B\x93\xAF\x14a\x03RWa\x01\xDBV[\x80cdI\xFCW\x14a\x02\xEDW\x80cgsB\xCE\x14a\x03\x0FW\x80cr\xB9\x82F\x14a\x03\"Wa\x01\xDBV[\x80c-[l\xB9\x14a\x02@W\x80c6yr:\x14a\x02fW\x80c7\xC6\xA4J\x14a\x02yW\x80c8\xD5.\x0F\x14a\x02\x8CW\x80c9(\xFF\x97\x14a\x02\xB7W\x80cI\xA7\xA2m\x14a\x02\xDAW[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x02Sa\x02N6`\x04a(\xC6V[a\x04NV[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x02Sa\x02t6`\x04a(\xC6V[a\x04_V[a\x02Sa\x02\x876`\x04a(\xE2V[a\x04jV[`\x03Ta\x02\x9F\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02]V[a\x02\xCAa\x02\xC56`\x04a)\x18V[a\x04}V[`@Qa\x02]\x94\x93\x92\x91\x90a)\xA3V[`\x02Ta\x02\x9F\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x0BTa\x02\xFF\x90a\x01\0\x90\x04`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x02]V[a\x02Sa\x03\x1D6`\x04a(\xC6V[a\x05eV[a\x02S`\tT\x81V[a\x02S`\nT\x81V[a\x02S`\x07T\x81V[a\x03Pa\x03K6`\x04a(\xE2V[a\x05pV[\0[`\x04Ta\x02\x9F\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\0Ta\x02\x9F\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x05Ta\x02\x9F\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x02Sa\x03\x996`\x04a(\xE2V[a\x06gV[a\x03Pa\x03\xAC6`\x04a(\xC6V[a\x06sV[a\x02Sa\x03\xBF6`\x04a(\xE2V[a\r\xFFV[a\x02Sa\x03\xD26`\x04a(\xC6V[a\x0E\x0BV[a\x02Sa\x03\xE56`\x04a(\xC6V[a\x0E\x16V[`\x01Ta\x02\x9F\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x04\x05a\x0E!V[`@Qa\x02]\x91\x90a)\xD4V[a\x02Sa\x04 6`\x04a(\xE2V[a\x0E\xAFV[`\x0BTa\x02\xFF\x90`\xFF\x16\x81V[a\x03Pa\x04@6`\x04a(\xE2V[a\x0E\xBBV[a\x02S`\x08T\x81V[`\0a\x04Y\x82a\x0F\xA2V[\x92\x91PPV[`\0a\x04Y\x82a\x11\x82V[`\0a\x04v\x83\x83a\x12(V[\x93\x92PPPV[`\x02T`@Qc9(\xFF\x97`\xE0\x1B\x81R`\x04\x81\x01\x85\x90R\x83\x15\x15`$\x82\x01R`D\x81\x01\x83\x90R`\0\x91\x82\x91\x82\x91``\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c9(\xFF\x97\x90`d\x01`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x05\x18W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/\x11\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x05,W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x05T\x91\x90\x81\x01\x90a*.V[\x93P\x93P\x93P\x93P\x93P\x93P\x93P\x93V[`\0a\x04Y\x82a\x12=V[a\x05y\x81a\x12\xE1V[`\x0BTa\x05\x8E\x90a\x01\0\x90\x04`\xFF\x16\x82a\x16*V[`\x0BT`\x03T`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01Ra\x06[\x92\x85\x92`\xFF\x90\x91\x16\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x06\x1EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/\x11\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x062W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06V\x91\x90a+\xDAV[a\x19\x92V[a\x06ca\x1D\xA5V[PPV[`\0a\x04v\x83\x83a\x1F\xEFV[`@Q` \x01a\x06\xA0\x90` \x80\x82R`\t\x90\x82\x01Rh\x13\x1B\xD9\xD3\x9B\xDC\x9BX[`\xBA\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `\x06`@Q` \x01a\x06\xC8\x91\x90a,0V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x03a\t\xA2W`\x02T`@Qc3\x85N\xFD`\xE2\x1B\x81R`\x04\x81\x01\x83\x90R`\0\x91\x82\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xCE\x15;\xF4\x90`$\x01```@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x07jW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/\x11\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x07~W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\xA2\x91\x90a,\xE1V[`\x02T`@Qc\x03\xB4\xD1\x03`\xE4\x1B\x81R`\x04\x81\x01\x89\x90R\x93\x96P\x91\x94P\x92P`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c;M\x100\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x08/W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/\x11\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x08CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08g\x91\x90a+\xDAV[`\x02T`@Qc@\xDA\xFDa`\xE1\x1B\x81R`\x04\x81\x01\x88\x90R\x91\x92P`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x81\xB5\xFA\xC2\x90`$\x01`\xA0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x08\xEFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/\x11\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\t\x03W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t'\x91\x90a-\x12V[\x80Q` \x80\x83\x01Q`@\x80\x85\x01Q\x81Q\x88\x81RB\x94\x81\x01\x94\x90\x94R\x90\x83\x01\x8A\x90R``\x83\x01\x89\x90R`\x80\x83\x01\x88\x90R`\xA0\x83\x01\x93\x90\x93R`\xC0\x82\x01R`\xE0\x81\x01\x91\x90\x91R\x90\x91P\x7Ft\xBB\x813\xEFY\x05\xC0\xA7\xE9\xCF\xB2\xAF,9p\xF3\xE3\xB7.\xD7\x0C\xCB\xB6'\x95\x96\xED\x86}3x\x90a\x01\0\x01`@Q\x80\x91\x03\x90\xA1PPPPP[`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xA05\xB1\xFE`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\n1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/\x11\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\nEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\ni\x91\x90a+\xDAV[`\x03T`\0\x80T`@Qcp\xA0\x821`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x93\x94P\x90\x92\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\n\xF4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/\x11\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0B\x08W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B,\x91\x90a+\xDAV[`\x04\x80T`\0\x80T`@Qcp\xA0\x821`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x94\x81\x01\x94\x90\x94R\x93\x94P\x92\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0B\xB8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/\x11\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0B\xCCW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\xF0\x91\x90a+\xDAV[`@\x80Q\x85\x81RB` \x82\x01R\x90\x81\x01\x84\x90R``\x81\x01\x82\x90R\x90\x91P\x7F\xBC}\xF0Z&tit\x1Er\xD9\x8D\xFC\x1F\xBE\xFAP*\x8D]\x8BD\t\xA6R\x13\x1B\x8E\x14\xFD,\xEE\x90`\x80\x01`@Q\x80\x91\x03\x90\xA1`\x03T`@Qcp\xA0\x821`\xE0\x1B\x81R3`\x04\x82\x01R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0C\xBCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/\x11\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0C\xD0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xF4\x91\x90a+\xDAV[`\x04\x80T`@Qcp\xA0\x821`\xE0\x1B\x81R3\x92\x81\x01\x92\x90\x92R\x91\x92P`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\r|W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/\x11\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\r\x90W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\xB4\x91\x90a+\xDAV[`@\x80Q\x84\x81R` \x81\x01\x83\x90RB\x81\x83\x01R\x90Q\x91\x92P\x7F[\x88\x97\xA4\xB3\xA7sN;\x9A8\xAFo>m5\xC4\xF5g2n#*KTJ\xDB\xD8?\x94$\xFD\x91\x90\x81\x90\x03``\x01\x90\xA1PPPPPPV[`\0a\x04v\x83\x83a \x04V[`\0a\x04Y\x82a \x19V[`\0a\x04Y\x82a \x82V[`\x06\x80Ta\x0E.\x90a+\xF6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0EZ\x90a+\xF6V[\x80\x15a\x0E\xA7W\x80`\x1F\x10a\x0E|Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0E\xA7V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0E\x8AW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`\0a\x04v\x83\x83a \xDEV[a\x0E\xC4\x81a\x12\xE1V[`\x0BTa\x0E\xDB\x90\x83\x90a\x01\0\x90\x04`\xFF\x16\x83a\x19\x92V[`\x0BT`\x03T`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01Ra\x06[\x92`\xFF\x16\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0FeW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/\x11\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0FyW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\x9D\x91\x90a+\xDAV[a\x16*V[`\0\x80\x82\x13a\x0F\xE4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0``a\x0F\xF1\x84a \xF3V[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[`\0g\x06\xF0[Y\xD3\xB2\0\0\x82\x03a\x11\x9BWP`\0\x91\x90PV[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x12a\x11\xC3W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\x11\xE4W`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x11\xF1\x83`\x02a-\xABV[\x90P`\0a\x11\xFE\x82a!\x9BV[\x90P`\0a\x12\x14g\x13\xA0K\xBD\xFD\xC9\xBE\x88\x83a$\x1CV[\x90Pa\x12\x1F\x81a-\xDBV[\x95\x94PPPPPV[`\0a\x04v\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a$1V[`\xB5\x81`\x01`\x88\x1B\x81\x10a\x12VW`@\x91\x90\x91\x1B\x90`\x80\x1C[i\x01\0\0\0\0\0\0\0\0\0\x81\x10a\x12rW` \x91\x90\x91\x1B\x90`@\x1C[e\x01\0\0\0\0\0\x81\x10a\x12\x8AW`\x10\x91\x90\x91\x1B\x90` \x1C[c\x01\0\0\0\x81\x10a\x12\xA0W`\x08\x91\x90\x91\x1B\x90`\x10\x1C[b\x01\0\0\x01\x02`\x12\x1C\x80\x82\x04\x01`\x01\x90\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x90\x81\x90\x04\x81\x11\x90\x03\x90V[`\x04\x80T`@Qcp\xA0\x821`\xE0\x1B\x81R3\x92\x81\x01\x92\x90\x92R`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x13hW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/\x11\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x13|W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\xA0\x91\x90a+\xDAV[\x90P\x81\x81\x10\x15a\x13\xCDW`@Qc\n\xBEZ\x89`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x81\x01\x83\x90R`D\x01a\x0F\xDBV[`\x04\x80T`@Qcn\xB1v\x9F`\xE1\x1B\x81R3\x92\x81\x01\x92\x90\x92R0`$\x83\x01R`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xDDb\xED>\x90`D\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x14ZW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/\x11\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x14nW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14\x92\x91\x90a+\xDAV[\x90P\x82\x81\x10\x15a\x14\xBFW`@Qc\xDAV\xD3\xC5`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x81\x01\x84\x90R`D\x01a\x0F\xDBV[`\x04\x80T`@Qc#\xB8r\xDD`\xE0\x1B\x81R3\x92\x81\x01\x92\x90\x92R0`$\x83\x01R`D\x82\x01\x85\x90R`\x01`\x01`\xA0\x1B\x03\x16\x90c#\xB8r\xDD\x90`d\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x15PW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/\x11\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x15dW=`\0\x80>=`\0\xFD[PP`\x04\x80T`@Qcp\xA0\x821`\xE0\x1B\x81R0\x92\x81\x01\x92\x90\x92R`\x01`\x01`\xA0\x1B\x03\x16\x92Pcp\xA0\x821\x91P`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x15\xEAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/\x11\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x15\xFEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\"\x91\x90a+\xDAV[`\x08UPPPV[\x81\x15a\x17\x80W`\x03T`\0T`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x84\x90R\x91\x16\x90c\t^\xA7\xB3\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x16\xBEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/\x11\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x16\xD2W=`\0\x80>=`\0\xFD[PP`\0T`\x03T`@Qc\xD0\x04\xF0\xF7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x86\x90R\x91\x16\x92Pc\xD0\x04\xF0\xF7\x91P`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x17dW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/\x11\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x17xW=`\0\x80>=`\0\xFD[PPPPPPV[`\x04\x80T`\0T`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x93\x81\x01\x93\x90\x93R`$\x83\x01\x84\x90R\x16\x90c\t^\xA7\xB3\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x18\x10W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/\x11\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x18$W=`\0\x80>=`\0\xFD[PP`\0T`\x04\x80T`@Qc\xD0\x04\xF0\xF7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x92\x81\x01\x92\x90\x92R`$\x82\x01\x86\x90R\x90\x91\x16\x92Pc\xD0\x04\xF0\xF7\x91P`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x18\xBAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/\x11\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x18\xCEW=`\0\x80>=`\0\xFD[PP`\x03T`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x92Pcp\xA0\x821\x91P`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x19SW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/\x11\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x19gW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19\x8B\x91\x90a+\xDAV[`\x07UPPV[\x81\x15a\x1ACW`\x03T`\x01T`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x84\x90R\x91\x16\x90c\t^\xA7\xB3\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1A&W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/\x11\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x1A:W=`\0\x80>=`\0\xFD[PPPPa\x1A\xECV[`\x04\x80T`\x01T`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x93\x81\x01\x93\x90\x93R`$\x83\x01\x84\x90R\x16\x90c\t^\xA7\xB3\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1A\xD3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/\x11\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x1A\xE7W=`\0\x80>=`\0\xFD[PPPP[`\x02T`@Qc9(\xFF\x97`\xE0\x1B\x81R`\x04\x81\x01\x85\x90R\x83\x15\x15`$\x82\x01R`D\x81\x01\x83\x90R`\0\x91\x82\x91\x82\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c9(\xFF\x97\x90`d\x01`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x1B\x86W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/\x11\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x1B\x9AW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1B\xC2\x91\x90\x81\x01\x90a*.V[\x93P\x93P\x93P\x93P\x83a\x1B\xF0W\x83\x83\x83\x83`@Qc\x03\x14\xE6#`\xE3\x1B\x81R`\x04\x01a\x0F\xDB\x94\x93\x92\x91\x90a)\xA3V[`\x01T`@Qc\xBD\x06%\xAB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xBD\x06%\xAB\x90a\x1C\"\x90\x8A\x90\x85\x90`\x04\x01a-\xF7V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1CzW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/\x11\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x92PPP\x80\x15a\x1C\x8BWP`\x01[a\x1C\xDAW=\x80\x80\x15a\x1C\xB9W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x1C\xBEV[``\x91P[P\x80`@Qcg\xA1k\x8D`\xE1\x1B\x81R`\x04\x01a\x0F\xDB\x91\x90a.\x18V[\x85a\x1D\x9CW`\x03T`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x1D`W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/\x11\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x1DtW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1D\x98\x91\x90a+\xDAV[`\x07U[PPPPPPPV[`\x04\x80T`@Qcp\xA0\x821`\xE0\x1B\x81R0\x92\x81\x01\x92\x90\x92R`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x1E'W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/\x11\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x1E;W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E_\x91\x90a+\xDAV[`\t\x81\x90U`\x08T\x11\x15a\x1E\xEAW\x7F\xB6[.\x08]}\x04\x0C1?}N\x1A\xC9\x0FY7\x02o\xEEI~\x0E$\xA7\xEF\xF1jU\xE1\xC5\xEA`\tT`\x08Ta\x1E\x9E\x91\x90a._V[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xA1`\x08T`\tTa\x1E\xBF\x81\x83a.rV[`@Qc\xB1n7\x83`\xE0\x1B\x81R`\x04\x81\x01\x93\x90\x93R`$\x83\x01\x91\x90\x91R\x19`D\x82\x01R`d\x01a\x0F\xDBV[`\0`\x08T`\tTa\x1E\xFC\x91\x90a._V[\x90P\x80`\n`\0\x82\x82Ta\x1F\x10\x91\x90a.\x99V[\x90\x91UPP`@Q\x81\x81R\x7F5}\x90_\x181 \x97\x97\xDFMU\xD7\x9C\\[\xF1\xD9\xF71\x1C\x97j\xFD\x05\xE1=\x88\x1E\xAB\x9B\xC8\x90` \x01`@Q\x80\x91\x03\x90\xA1`\x04\x80T`\tT`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3\x93\x81\x01\x93\x90\x93R`$\x83\x01R`\x01`\x01`\xA0\x1B\x03\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1F\xD4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/\x11\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x1F\xE8W=`\0\x80>=`\0\xFD[PPPPPV[`\0a\x04v\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a$PV[`\0a\x04v\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a$PV[`\0\x80g\x13\xA0K\xBD\xFD\xC9\xBE\x88a 7g\r\xE0\xB6\xB3\xA7d\0\0\x85a-\xABV[a A\x91\x90a.\xACV[\x90P`\0a N\x82a-\xDBV[\x90P`\0a [\x82a$~V[\x90Pg\x1B\xC1mgN\xC8\0\0a xg\r\xE0\xB6\xB3\xA7d\0\0\x83a-\xABV[a\x12\x1F\x91\x90a.\xACV[`\0\x80g\x1B\xC1mgN\xC8\0\0\x83a \x98\x81a-\xDBV[a \xA2\x91\x90a-\xABV[a \xAC\x91\x90a.\xACV[\x90Pa \xB7\x81a&bV[\x90Pg\"\xC9U\"\x95T\xC1\xB6a \xD4g\r\xE0\xB6\xB3\xA7d\0\0\x83a-\xABV[a\x04v\x91\x90a.\xACV[`\0a\x04v\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a$1V[`\0\x80\x82\x11a!0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x0F\xDBV[P`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x82\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[`\0\x80\x82\x12\x80a!\xB2WPg\x1B\xC1mgN\xC8\0\0\x82\x13[\x15a!\xD0W`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a!\xF1W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81g\x1B\xC1mgN\xC8\0\0\x03a\"\x19W`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x15a\"$W\x91\x90PV[`\0g\r\xE0\xB6\xB3\xA7d\0\0\x83\x12a\"LWa\"G\x83g\x1B\xC1mgN\xC8\0\0a.rV[a\"NV[\x82[\x90P`\0a\"d\x82g\x1B\xC1mgN\xC8\0\0a(\x0BV[\x90P\x80`\0\x03a\"\x87W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\"\x92\x82a\x0F\xA2V[\x90P`\0c;\x9A\xCA\0a\"\xBDa\"\xB8a\"\xB2g\x1B\xC1mgN\xC8\0\0a-\xDBV[\x85a$\x1CV[a\x12=V[a\"\xC7\x91\x90a-\xABV[\x90P`\0\x80a\"\xDE\x83g\x03\xC1f\\z\xAB \0a$\x1CV[a\"\xF0\x90g \x05\xFEO&\x8E\xA0\0a.\xE8V[\x90P`\0a# \x84a#\t\x86f\x9F2u$b\xA0\0a$\x1CV[a#\x1B\x90g\r\xC5R\x7Fd, \0a.\xE8V[a$\x1CV[a#2\x90g\r\xE0\xB6\xB3\xA7d\0\0a.\xE8V[\x90Pa#Vg\t\xD0(\xCCo _\xFF\x19\x85a#L\x85\x85a(\x0BV[a#\x1B\x91\x90a.rV[\x92PPP`\0[`\x02\x81\x10\x15a#\xF1W`\0\x86a#r\x84a$~V[a#|\x91\x90a.rV[\x90P`\0a#\x8A\x84\x85a$\x1CV[a#\x93\x90a-\xDBV[\x90P`\0a#\xA0\x82a&bV[\x90P`\0a#\xAE\x86\x85a$\x1CV[a#\xC0g\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x84a$\x1CV[a#\xCA\x91\x90a.rV[\x90Pa#\xD6\x84\x82a(\x0BV[a#\xE0\x90\x87a.\xE8V[\x95P\x84`\x01\x01\x94PPPPPa#]V[g\r\xE0\xB6\xB3\xA7d\0\0\x88\x12a$\x0EWa$\t\x82a-\xDBV[a$\x10V[\x81[\x98\x97PPPPPPPPV[`\0a\x04v\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a(\x1CV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a$IW`\0\x80\xFD[\x04\x92\x91PPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a$hW`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[`\0\x81`\0\x03a$\x97WPg\r\xE0\xB6\xB3\xA7d\0\0\x91\x90PV[gV\x98\xEE\xF0fp\0\0\x82\x12a$\xAEWP`\0\x91\x90PV[a$\xBFgV\x98\xEE\xF0fp\0\0a-\xDBV[\x82\x13a$\xD4WPg\x1B\xC1mgN\xC8\0\0\x91\x90PV[`\0a$\xDF\x83a(;V[\x90P`\0a%\x18g\r\xE0\xB6\xB3\xA7d\0\0a%\x01\x84g\x1B\xC1mgN\xC8\0\0a\x12(V[a%\x13\x90g\r\xE0\xB6\xB3\xA7d\0\0a.\xE8V[a(\x0BV[\x90P`\0\x80\x82a%t\x81a%a\x81a%O\x81a%<\x81g\x02_\x0F\xE1\x05\xA3\x14\0a$\x1CV[a#\x1B\x90g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19a.\xE8V[a#\x1B\x90g\x14\xA8EL\x19\xE1\xAC\0a.\xE8V[a#\x1B\x90g\x0F\xC1\x0E\x01W\x82w\xFF\x19a.\xE8V[a%\x86\x90g\x03\xDE\xBD\x08;\x8C|\0a.\xE8V[\x91P\x83\x90Pa%\xEE\x81a%\xDC\x81a%\xCA\x81a%\xB8\x81a%\xA5\x81\x8Ba$\x1CV[a#\x1B\x90g\x02\x95\xD4\0\xEA2W\xFF\x19a.\xE8V[a#\x1B\x90g\x01W\xD8\xB2\xEC\xC7\x08\0a.\xE8V[a#\x1B\x90g\x051\n\xA7\xD5!0\0a.\xE8V[a#\x1B\x90g\r\xE0\xCC=\x15a\0\0a.\xE8V[\x91P\x81g\x11\x90\0\xAB\x10\x0F\xFC\0a&\x04\x87\x88a$\x1CV[a&\x10\x90`\0\x19a-\xABV[a&\x1A\x91\x90a.rV[a&$\x91\x90a.\xE8V[\x92PP`\0a&2\x83a&bV[\x90P`\0a&@\x85\x83a$\x1CV[\x90P`\0\x88\x12a&PW\x80a$\x10V[a$\x10\x81g\x1B\xC1mgN\xC8\0\0a.rV[`\0h\x02H\xCE6\xA7\x0C\xB2k>\x19\x82\x13a&}WP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a&\xC4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x01a\x0F\xDBV[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05`\x01`_\x1B\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1Dm\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1Dn\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[`\0a\x04v\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x05\x85\x14\x17\x16a(4W`\0\x80\xFD[\x05\x92\x91PPV[`\0`\x01`\xFF\x1B\x82\x03a(aW`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x12\x15a(rWP\x19`\x01\x01\x90V[P\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`\0` \x82\x84\x03\x12\x15a(\xDBWa(\xDBa(vV[P5\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a(\xF8Wa(\xF8a(vV[PP\x805\x92` \x90\x91\x015\x91PV[\x80\x15\x15\x81\x14a)\x15W`\0\x80\xFD[PV[`\0\x80`\0``\x84\x86\x03\x12\x15a)0Wa)0a(vV[\x835\x92P` \x84\x015a)B\x81a)\x07V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0[\x83\x81\x10\x15a)nW\x81\x81\x01Q\x83\x82\x01R` \x01a)VV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra)\x8F\x81` \x86\x01` \x86\x01a)SV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[\x84\x15\x15\x81R\x83` \x82\x01R\x82`@\x82\x01R`\x80``\x82\x01R`\0a)\xCA`\x80\x83\x01\x84a)wV[\x96\x95PPPPPPV[` \x81R`\0a\x04v` \x83\x01\x84a)wV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a*&Wa*&a)\xE7V[`@R\x91\x90PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a*GWa*Ga(vV[\x84Qa*R\x81a)\x07V[\x80\x94PP` \x80\x86\x01Q\x93P`@\x86\x01Q\x92P``\x86\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a*\xCBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x84\x90R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[\x81\x88\x01\x91P\x88`\x1F\x83\x01\x12a+3W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x84\x90R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x81\xFD[\x81Q\x81\x81\x11\x15a+EWa+Ea)\xE7V[a+W`\x1F\x82\x01`\x1F\x19\x16\x85\x01a)\xFDV[\x91P\x80\x82R\x89\x84\x82\x85\x01\x01\x11\x15a+\xBDW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x85\x90R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R`\x84\x81\xFD[a+\xCC\x81\x85\x84\x01\x86\x86\x01a)SV[P\x95\x98\x94\x97P\x92\x95PPPPV[`\0` \x82\x84\x03\x12\x15a+\xEFWa+\xEFa(vV[PQ\x91\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a,\nW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a,*WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\0` \x80\x83R`\0\x84T\x81`\x01\x82`\x01\x1C\x91P`\x01\x83\x16\x80a,TW`\x7F\x83\x16\x92P[` \x83\x10\x81\x03a,rWcNH{q`\xE0\x1B\x85R`\"`\x04R`$\x85\xFD[` \x88\x01\x83\x90R`@\x88\x01\x81\x80\x15a,\x91W`\x01\x81\x14a,\xA7Wa,\xD2V[`\xFF\x19\x86\x16\x82R\x84\x15\x15`\x05\x1B\x82\x01\x96Pa,\xD2V[`\0\x8B\x81R` \x90 `\0[\x86\x81\x10\x15a,\xCCW\x81T\x84\x82\x01R\x90\x85\x01\x90\x89\x01a,\xB3V[\x83\x01\x97PP[P\x94\x99\x98PPPPPPPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a,\xF9Wa,\xF9a(vV[\x83Q\x92P` \x84\x01Q\x91P`@\x84\x01Q\x90P\x92P\x92P\x92V[`\0`\xA0\x82\x84\x03\x12\x15a-'Wa-'a(vV[`@Q`\xA0\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a-JWa-Ja)\xE7V[`@\x90\x81R\x83Q\x82R` \x80\x85\x01Q\x90\x83\x01R\x83\x81\x01Q\x90\x82\x01R``\x80\x84\x01Q\x90\x82\x01R`\x80\x83\x01Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a-\x89W`\0\x80\xFD[`\x80\x82\x01R\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15a-\xC7Wa-\xC7a-\x95V[\x81\x81\x05\x83\x14\x82\x15\x17a\x04YWa\x04Ya-\x95V[`\0`\x01`\xFF\x1B\x82\x01a-\xF0Wa-\xF0a-\x95V[P`\0\x03\x90V[\x82\x81R`@` \x82\x01R`\0a.\x10`@\x83\x01\x84a)wV[\x94\x93PPPPV[`@\x81R`\x19`@\x82\x01R\x7FDEX swap failed with data\0\0\0\0\0\0\0``\x82\x01R`\x80` \x82\x01R`\0a\x04v`\x80\x83\x01\x84a)wV[\x81\x81\x03\x81\x81\x11\x15a\x04YWa\x04Ya-\x95V[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a.\x92Wa.\x92a-\x95V[P\x92\x91PPV[\x80\x82\x01\x80\x82\x11\x15a\x04YWa\x04Ya-\x95V[`\0\x82a.\xC9WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\x01`\xFF\x1B\x82\x14`\0\x19\x84\x14\x16\x15a.\xE3Wa.\xE3a-\x95V[P\x05\x90V[\x80\x82\x01\x82\x81\x12`\0\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a/\x08Wa/\x08a-\x95V[PP\x92\x91PPV\xFETarget contract does not contain\xA2dipfsX\"\x12 k\x1F(\xA8\xD0\xAD7\xD8\x934\xD5<\xA4\xB0\x05\xE2\xD3\x99q*@#K\xBDVn3E\x92\xAE\xE2\xE5dsolcC\0\x08\x16\x003Target contract does not contain";
    /// The bytecode of the contract.
    pub static ATOMICV2_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x01\xDBW`\x005`\xE0\x1C\x80c\x9F'\xEFO\x11a\x01+W\x80c\xD2L\xE6\xE5\x11a\0\xE4W\x80c\xE5$\xF8I\x11a\0\xBEW\x80c\xE5$\xF8I\x14a\x04\x12W\x80c\xF3\xC9s\xCF\x14a\x04%W\x80c\xF9\0^\xB5\x14a\x042W\x80c\xFA.Y\x94\x14a\x04EWa\x01\xDBV[\x80c\xD2L\xE6\xE5\x14a\x03\xD7W\x80c\xD2\xF7&Z\x14a\x03\xEAW\x80c\xE1s\xAD%\x14a\x03\xFDWa\x01\xDBV[\x80c\x9F'\xEFO\x14a\x03eW\x80c\xA8\xC6.v\x14a\x03xW\x80c\xAE\x97h\xA8\x14a\x03\x8BW\x80c\xB3\xB2\xBF+\x14a\x03\x9EW\x80c\xBD%-\x06\x14a\x03\xB1W\x80c\xD0\xB7\x1B\x1E\x14a\x03\xC4Wa\x01\xDBV[\x80cdI\xFCW\x11a\x01\x98W\x80c\x85\xB3\x19\xFF\x11a\x01rW\x80c\x85\xB3\x19\xFF\x14a\x03+W\x80c\x93e \xC3\x14a\x034W\x80c\x96\xFB\xEE\x1D\x14a\x03=W\x80c\x99\x9B\x93\xAF\x14a\x03RWa\x01\xDBV[\x80cdI\xFCW\x14a\x02\xEDW\x80cgsB\xCE\x14a\x03\x0FW\x80cr\xB9\x82F\x14a\x03\"Wa\x01\xDBV[\x80c-[l\xB9\x14a\x02@W\x80c6yr:\x14a\x02fW\x80c7\xC6\xA4J\x14a\x02yW\x80c8\xD5.\x0F\x14a\x02\x8CW\x80c9(\xFF\x97\x14a\x02\xB7W\x80cI\xA7\xA2m\x14a\x02\xDAW[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x02Sa\x02N6`\x04a(\xC6V[a\x04NV[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x02Sa\x02t6`\x04a(\xC6V[a\x04_V[a\x02Sa\x02\x876`\x04a(\xE2V[a\x04jV[`\x03Ta\x02\x9F\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02]V[a\x02\xCAa\x02\xC56`\x04a)\x18V[a\x04}V[`@Qa\x02]\x94\x93\x92\x91\x90a)\xA3V[`\x02Ta\x02\x9F\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x0BTa\x02\xFF\x90a\x01\0\x90\x04`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x02]V[a\x02Sa\x03\x1D6`\x04a(\xC6V[a\x05eV[a\x02S`\tT\x81V[a\x02S`\nT\x81V[a\x02S`\x07T\x81V[a\x03Pa\x03K6`\x04a(\xE2V[a\x05pV[\0[`\x04Ta\x02\x9F\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\0Ta\x02\x9F\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x05Ta\x02\x9F\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x02Sa\x03\x996`\x04a(\xE2V[a\x06gV[a\x03Pa\x03\xAC6`\x04a(\xC6V[a\x06sV[a\x02Sa\x03\xBF6`\x04a(\xE2V[a\r\xFFV[a\x02Sa\x03\xD26`\x04a(\xC6V[a\x0E\x0BV[a\x02Sa\x03\xE56`\x04a(\xC6V[a\x0E\x16V[`\x01Ta\x02\x9F\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x04\x05a\x0E!V[`@Qa\x02]\x91\x90a)\xD4V[a\x02Sa\x04 6`\x04a(\xE2V[a\x0E\xAFV[`\x0BTa\x02\xFF\x90`\xFF\x16\x81V[a\x03Pa\x04@6`\x04a(\xE2V[a\x0E\xBBV[a\x02S`\x08T\x81V[`\0a\x04Y\x82a\x0F\xA2V[\x92\x91PPV[`\0a\x04Y\x82a\x11\x82V[`\0a\x04v\x83\x83a\x12(V[\x93\x92PPPV[`\x02T`@Qc9(\xFF\x97`\xE0\x1B\x81R`\x04\x81\x01\x85\x90R\x83\x15\x15`$\x82\x01R`D\x81\x01\x83\x90R`\0\x91\x82\x91\x82\x91``\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c9(\xFF\x97\x90`d\x01`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x05\x18W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/\x11\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x05,W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x05T\x91\x90\x81\x01\x90a*.V[\x93P\x93P\x93P\x93P\x93P\x93P\x93P\x93V[`\0a\x04Y\x82a\x12=V[a\x05y\x81a\x12\xE1V[`\x0BTa\x05\x8E\x90a\x01\0\x90\x04`\xFF\x16\x82a\x16*V[`\x0BT`\x03T`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01Ra\x06[\x92\x85\x92`\xFF\x90\x91\x16\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x06\x1EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/\x11\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x062W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06V\x91\x90a+\xDAV[a\x19\x92V[a\x06ca\x1D\xA5V[PPV[`\0a\x04v\x83\x83a\x1F\xEFV[`@Q` \x01a\x06\xA0\x90` \x80\x82R`\t\x90\x82\x01Rh\x13\x1B\xD9\xD3\x9B\xDC\x9BX[`\xBA\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `\x06`@Q` \x01a\x06\xC8\x91\x90a,0V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x03a\t\xA2W`\x02T`@Qc3\x85N\xFD`\xE2\x1B\x81R`\x04\x81\x01\x83\x90R`\0\x91\x82\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xCE\x15;\xF4\x90`$\x01```@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x07jW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/\x11\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x07~W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\xA2\x91\x90a,\xE1V[`\x02T`@Qc\x03\xB4\xD1\x03`\xE4\x1B\x81R`\x04\x81\x01\x89\x90R\x93\x96P\x91\x94P\x92P`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c;M\x100\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x08/W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/\x11\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x08CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08g\x91\x90a+\xDAV[`\x02T`@Qc@\xDA\xFDa`\xE1\x1B\x81R`\x04\x81\x01\x88\x90R\x91\x92P`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x81\xB5\xFA\xC2\x90`$\x01`\xA0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x08\xEFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/\x11\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\t\x03W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t'\x91\x90a-\x12V[\x80Q` \x80\x83\x01Q`@\x80\x85\x01Q\x81Q\x88\x81RB\x94\x81\x01\x94\x90\x94R\x90\x83\x01\x8A\x90R``\x83\x01\x89\x90R`\x80\x83\x01\x88\x90R`\xA0\x83\x01\x93\x90\x93R`\xC0\x82\x01R`\xE0\x81\x01\x91\x90\x91R\x90\x91P\x7Ft\xBB\x813\xEFY\x05\xC0\xA7\xE9\xCF\xB2\xAF,9p\xF3\xE3\xB7.\xD7\x0C\xCB\xB6'\x95\x96\xED\x86}3x\x90a\x01\0\x01`@Q\x80\x91\x03\x90\xA1PPPPP[`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xA05\xB1\xFE`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\n1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/\x11\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\nEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\ni\x91\x90a+\xDAV[`\x03T`\0\x80T`@Qcp\xA0\x821`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x93\x94P\x90\x92\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\n\xF4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/\x11\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0B\x08W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B,\x91\x90a+\xDAV[`\x04\x80T`\0\x80T`@Qcp\xA0\x821`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x94\x81\x01\x94\x90\x94R\x93\x94P\x92\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0B\xB8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/\x11\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0B\xCCW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\xF0\x91\x90a+\xDAV[`@\x80Q\x85\x81RB` \x82\x01R\x90\x81\x01\x84\x90R``\x81\x01\x82\x90R\x90\x91P\x7F\xBC}\xF0Z&tit\x1Er\xD9\x8D\xFC\x1F\xBE\xFAP*\x8D]\x8BD\t\xA6R\x13\x1B\x8E\x14\xFD,\xEE\x90`\x80\x01`@Q\x80\x91\x03\x90\xA1`\x03T`@Qcp\xA0\x821`\xE0\x1B\x81R3`\x04\x82\x01R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0C\xBCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/\x11\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0C\xD0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xF4\x91\x90a+\xDAV[`\x04\x80T`@Qcp\xA0\x821`\xE0\x1B\x81R3\x92\x81\x01\x92\x90\x92R\x91\x92P`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\r|W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/\x11\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\r\x90W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\xB4\x91\x90a+\xDAV[`@\x80Q\x84\x81R` \x81\x01\x83\x90RB\x81\x83\x01R\x90Q\x91\x92P\x7F[\x88\x97\xA4\xB3\xA7sN;\x9A8\xAFo>m5\xC4\xF5g2n#*KTJ\xDB\xD8?\x94$\xFD\x91\x90\x81\x90\x03``\x01\x90\xA1PPPPPPV[`\0a\x04v\x83\x83a \x04V[`\0a\x04Y\x82a \x19V[`\0a\x04Y\x82a \x82V[`\x06\x80Ta\x0E.\x90a+\xF6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0EZ\x90a+\xF6V[\x80\x15a\x0E\xA7W\x80`\x1F\x10a\x0E|Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0E\xA7V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0E\x8AW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`\0a\x04v\x83\x83a \xDEV[a\x0E\xC4\x81a\x12\xE1V[`\x0BTa\x0E\xDB\x90\x83\x90a\x01\0\x90\x04`\xFF\x16\x83a\x19\x92V[`\x0BT`\x03T`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01Ra\x06[\x92`\xFF\x16\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0FeW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/\x11\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0FyW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\x9D\x91\x90a+\xDAV[a\x16*V[`\0\x80\x82\x13a\x0F\xE4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0``a\x0F\xF1\x84a \xF3V[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[`\0g\x06\xF0[Y\xD3\xB2\0\0\x82\x03a\x11\x9BWP`\0\x91\x90PV[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x12a\x11\xC3W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\x11\xE4W`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x11\xF1\x83`\x02a-\xABV[\x90P`\0a\x11\xFE\x82a!\x9BV[\x90P`\0a\x12\x14g\x13\xA0K\xBD\xFD\xC9\xBE\x88\x83a$\x1CV[\x90Pa\x12\x1F\x81a-\xDBV[\x95\x94PPPPPV[`\0a\x04v\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a$1V[`\xB5\x81`\x01`\x88\x1B\x81\x10a\x12VW`@\x91\x90\x91\x1B\x90`\x80\x1C[i\x01\0\0\0\0\0\0\0\0\0\x81\x10a\x12rW` \x91\x90\x91\x1B\x90`@\x1C[e\x01\0\0\0\0\0\x81\x10a\x12\x8AW`\x10\x91\x90\x91\x1B\x90` \x1C[c\x01\0\0\0\x81\x10a\x12\xA0W`\x08\x91\x90\x91\x1B\x90`\x10\x1C[b\x01\0\0\x01\x02`\x12\x1C\x80\x82\x04\x01`\x01\x90\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x90\x81\x90\x04\x81\x11\x90\x03\x90V[`\x04\x80T`@Qcp\xA0\x821`\xE0\x1B\x81R3\x92\x81\x01\x92\x90\x92R`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x13hW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/\x11\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x13|W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\xA0\x91\x90a+\xDAV[\x90P\x81\x81\x10\x15a\x13\xCDW`@Qc\n\xBEZ\x89`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x81\x01\x83\x90R`D\x01a\x0F\xDBV[`\x04\x80T`@Qcn\xB1v\x9F`\xE1\x1B\x81R3\x92\x81\x01\x92\x90\x92R0`$\x83\x01R`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xDDb\xED>\x90`D\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x14ZW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/\x11\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x14nW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14\x92\x91\x90a+\xDAV[\x90P\x82\x81\x10\x15a\x14\xBFW`@Qc\xDAV\xD3\xC5`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x81\x01\x84\x90R`D\x01a\x0F\xDBV[`\x04\x80T`@Qc#\xB8r\xDD`\xE0\x1B\x81R3\x92\x81\x01\x92\x90\x92R0`$\x83\x01R`D\x82\x01\x85\x90R`\x01`\x01`\xA0\x1B\x03\x16\x90c#\xB8r\xDD\x90`d\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x15PW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/\x11\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x15dW=`\0\x80>=`\0\xFD[PP`\x04\x80T`@Qcp\xA0\x821`\xE0\x1B\x81R0\x92\x81\x01\x92\x90\x92R`\x01`\x01`\xA0\x1B\x03\x16\x92Pcp\xA0\x821\x91P`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x15\xEAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/\x11\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x15\xFEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\"\x91\x90a+\xDAV[`\x08UPPPV[\x81\x15a\x17\x80W`\x03T`\0T`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x84\x90R\x91\x16\x90c\t^\xA7\xB3\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x16\xBEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/\x11\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x16\xD2W=`\0\x80>=`\0\xFD[PP`\0T`\x03T`@Qc\xD0\x04\xF0\xF7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x86\x90R\x91\x16\x92Pc\xD0\x04\xF0\xF7\x91P`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x17dW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/\x11\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x17xW=`\0\x80>=`\0\xFD[PPPPPPV[`\x04\x80T`\0T`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x93\x81\x01\x93\x90\x93R`$\x83\x01\x84\x90R\x16\x90c\t^\xA7\xB3\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x18\x10W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/\x11\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x18$W=`\0\x80>=`\0\xFD[PP`\0T`\x04\x80T`@Qc\xD0\x04\xF0\xF7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x92\x81\x01\x92\x90\x92R`$\x82\x01\x86\x90R\x90\x91\x16\x92Pc\xD0\x04\xF0\xF7\x91P`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x18\xBAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/\x11\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x18\xCEW=`\0\x80>=`\0\xFD[PP`\x03T`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x92Pcp\xA0\x821\x91P`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x19SW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/\x11\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x19gW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19\x8B\x91\x90a+\xDAV[`\x07UPPV[\x81\x15a\x1ACW`\x03T`\x01T`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x84\x90R\x91\x16\x90c\t^\xA7\xB3\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1A&W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/\x11\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x1A:W=`\0\x80>=`\0\xFD[PPPPa\x1A\xECV[`\x04\x80T`\x01T`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x93\x81\x01\x93\x90\x93R`$\x83\x01\x84\x90R\x16\x90c\t^\xA7\xB3\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1A\xD3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/\x11\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x1A\xE7W=`\0\x80>=`\0\xFD[PPPP[`\x02T`@Qc9(\xFF\x97`\xE0\x1B\x81R`\x04\x81\x01\x85\x90R\x83\x15\x15`$\x82\x01R`D\x81\x01\x83\x90R`\0\x91\x82\x91\x82\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c9(\xFF\x97\x90`d\x01`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x1B\x86W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/\x11\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x1B\x9AW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1B\xC2\x91\x90\x81\x01\x90a*.V[\x93P\x93P\x93P\x93P\x83a\x1B\xF0W\x83\x83\x83\x83`@Qc\x03\x14\xE6#`\xE3\x1B\x81R`\x04\x01a\x0F\xDB\x94\x93\x92\x91\x90a)\xA3V[`\x01T`@Qc\xBD\x06%\xAB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xBD\x06%\xAB\x90a\x1C\"\x90\x8A\x90\x85\x90`\x04\x01a-\xF7V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1CzW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/\x11\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x92PPP\x80\x15a\x1C\x8BWP`\x01[a\x1C\xDAW=\x80\x80\x15a\x1C\xB9W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x1C\xBEV[``\x91P[P\x80`@Qcg\xA1k\x8D`\xE1\x1B\x81R`\x04\x01a\x0F\xDB\x91\x90a.\x18V[\x85a\x1D\x9CW`\x03T`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x1D`W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/\x11\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x1DtW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1D\x98\x91\x90a+\xDAV[`\x07U[PPPPPPPV[`\x04\x80T`@Qcp\xA0\x821`\xE0\x1B\x81R0\x92\x81\x01\x92\x90\x92R`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x1E'W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/\x11\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x1E;W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E_\x91\x90a+\xDAV[`\t\x81\x90U`\x08T\x11\x15a\x1E\xEAW\x7F\xB6[.\x08]}\x04\x0C1?}N\x1A\xC9\x0FY7\x02o\xEEI~\x0E$\xA7\xEF\xF1jU\xE1\xC5\xEA`\tT`\x08Ta\x1E\x9E\x91\x90a._V[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xA1`\x08T`\tTa\x1E\xBF\x81\x83a.rV[`@Qc\xB1n7\x83`\xE0\x1B\x81R`\x04\x81\x01\x93\x90\x93R`$\x83\x01\x91\x90\x91R\x19`D\x82\x01R`d\x01a\x0F\xDBV[`\0`\x08T`\tTa\x1E\xFC\x91\x90a._V[\x90P\x80`\n`\0\x82\x82Ta\x1F\x10\x91\x90a.\x99V[\x90\x91UPP`@Q\x81\x81R\x7F5}\x90_\x181 \x97\x97\xDFMU\xD7\x9C\\[\xF1\xD9\xF71\x1C\x97j\xFD\x05\xE1=\x88\x1E\xAB\x9B\xC8\x90` \x01`@Q\x80\x91\x03\x90\xA1`\x04\x80T`\tT`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3\x93\x81\x01\x93\x90\x93R`$\x83\x01R`\x01`\x01`\xA0\x1B\x03\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1F\xD4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/\x11\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x1F\xE8W=`\0\x80>=`\0\xFD[PPPPPV[`\0a\x04v\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a$PV[`\0a\x04v\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a$PV[`\0\x80g\x13\xA0K\xBD\xFD\xC9\xBE\x88a 7g\r\xE0\xB6\xB3\xA7d\0\0\x85a-\xABV[a A\x91\x90a.\xACV[\x90P`\0a N\x82a-\xDBV[\x90P`\0a [\x82a$~V[\x90Pg\x1B\xC1mgN\xC8\0\0a xg\r\xE0\xB6\xB3\xA7d\0\0\x83a-\xABV[a\x12\x1F\x91\x90a.\xACV[`\0\x80g\x1B\xC1mgN\xC8\0\0\x83a \x98\x81a-\xDBV[a \xA2\x91\x90a-\xABV[a \xAC\x91\x90a.\xACV[\x90Pa \xB7\x81a&bV[\x90Pg\"\xC9U\"\x95T\xC1\xB6a \xD4g\r\xE0\xB6\xB3\xA7d\0\0\x83a-\xABV[a\x04v\x91\x90a.\xACV[`\0a\x04v\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a$1V[`\0\x80\x82\x11a!0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x0F\xDBV[P`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x82\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[`\0\x80\x82\x12\x80a!\xB2WPg\x1B\xC1mgN\xC8\0\0\x82\x13[\x15a!\xD0W`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a!\xF1W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81g\x1B\xC1mgN\xC8\0\0\x03a\"\x19W`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x15a\"$W\x91\x90PV[`\0g\r\xE0\xB6\xB3\xA7d\0\0\x83\x12a\"LWa\"G\x83g\x1B\xC1mgN\xC8\0\0a.rV[a\"NV[\x82[\x90P`\0a\"d\x82g\x1B\xC1mgN\xC8\0\0a(\x0BV[\x90P\x80`\0\x03a\"\x87W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\"\x92\x82a\x0F\xA2V[\x90P`\0c;\x9A\xCA\0a\"\xBDa\"\xB8a\"\xB2g\x1B\xC1mgN\xC8\0\0a-\xDBV[\x85a$\x1CV[a\x12=V[a\"\xC7\x91\x90a-\xABV[\x90P`\0\x80a\"\xDE\x83g\x03\xC1f\\z\xAB \0a$\x1CV[a\"\xF0\x90g \x05\xFEO&\x8E\xA0\0a.\xE8V[\x90P`\0a# \x84a#\t\x86f\x9F2u$b\xA0\0a$\x1CV[a#\x1B\x90g\r\xC5R\x7Fd, \0a.\xE8V[a$\x1CV[a#2\x90g\r\xE0\xB6\xB3\xA7d\0\0a.\xE8V[\x90Pa#Vg\t\xD0(\xCCo _\xFF\x19\x85a#L\x85\x85a(\x0BV[a#\x1B\x91\x90a.rV[\x92PPP`\0[`\x02\x81\x10\x15a#\xF1W`\0\x86a#r\x84a$~V[a#|\x91\x90a.rV[\x90P`\0a#\x8A\x84\x85a$\x1CV[a#\x93\x90a-\xDBV[\x90P`\0a#\xA0\x82a&bV[\x90P`\0a#\xAE\x86\x85a$\x1CV[a#\xC0g\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x84a$\x1CV[a#\xCA\x91\x90a.rV[\x90Pa#\xD6\x84\x82a(\x0BV[a#\xE0\x90\x87a.\xE8V[\x95P\x84`\x01\x01\x94PPPPPa#]V[g\r\xE0\xB6\xB3\xA7d\0\0\x88\x12a$\x0EWa$\t\x82a-\xDBV[a$\x10V[\x81[\x98\x97PPPPPPPPV[`\0a\x04v\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a(\x1CV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a$IW`\0\x80\xFD[\x04\x92\x91PPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a$hW`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[`\0\x81`\0\x03a$\x97WPg\r\xE0\xB6\xB3\xA7d\0\0\x91\x90PV[gV\x98\xEE\xF0fp\0\0\x82\x12a$\xAEWP`\0\x91\x90PV[a$\xBFgV\x98\xEE\xF0fp\0\0a-\xDBV[\x82\x13a$\xD4WPg\x1B\xC1mgN\xC8\0\0\x91\x90PV[`\0a$\xDF\x83a(;V[\x90P`\0a%\x18g\r\xE0\xB6\xB3\xA7d\0\0a%\x01\x84g\x1B\xC1mgN\xC8\0\0a\x12(V[a%\x13\x90g\r\xE0\xB6\xB3\xA7d\0\0a.\xE8V[a(\x0BV[\x90P`\0\x80\x82a%t\x81a%a\x81a%O\x81a%<\x81g\x02_\x0F\xE1\x05\xA3\x14\0a$\x1CV[a#\x1B\x90g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19a.\xE8V[a#\x1B\x90g\x14\xA8EL\x19\xE1\xAC\0a.\xE8V[a#\x1B\x90g\x0F\xC1\x0E\x01W\x82w\xFF\x19a.\xE8V[a%\x86\x90g\x03\xDE\xBD\x08;\x8C|\0a.\xE8V[\x91P\x83\x90Pa%\xEE\x81a%\xDC\x81a%\xCA\x81a%\xB8\x81a%\xA5\x81\x8Ba$\x1CV[a#\x1B\x90g\x02\x95\xD4\0\xEA2W\xFF\x19a.\xE8V[a#\x1B\x90g\x01W\xD8\xB2\xEC\xC7\x08\0a.\xE8V[a#\x1B\x90g\x051\n\xA7\xD5!0\0a.\xE8V[a#\x1B\x90g\r\xE0\xCC=\x15a\0\0a.\xE8V[\x91P\x81g\x11\x90\0\xAB\x10\x0F\xFC\0a&\x04\x87\x88a$\x1CV[a&\x10\x90`\0\x19a-\xABV[a&\x1A\x91\x90a.rV[a&$\x91\x90a.\xE8V[\x92PP`\0a&2\x83a&bV[\x90P`\0a&@\x85\x83a$\x1CV[\x90P`\0\x88\x12a&PW\x80a$\x10V[a$\x10\x81g\x1B\xC1mgN\xC8\0\0a.rV[`\0h\x02H\xCE6\xA7\x0C\xB2k>\x19\x82\x13a&}WP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a&\xC4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x01a\x0F\xDBV[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05`\x01`_\x1B\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1Dm\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1Dn\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[`\0a\x04v\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x05\x85\x14\x17\x16a(4W`\0\x80\xFD[\x05\x92\x91PPV[`\0`\x01`\xFF\x1B\x82\x03a(aW`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x12\x15a(rWP\x19`\x01\x01\x90V[P\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`\0` \x82\x84\x03\x12\x15a(\xDBWa(\xDBa(vV[P5\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a(\xF8Wa(\xF8a(vV[PP\x805\x92` \x90\x91\x015\x91PV[\x80\x15\x15\x81\x14a)\x15W`\0\x80\xFD[PV[`\0\x80`\0``\x84\x86\x03\x12\x15a)0Wa)0a(vV[\x835\x92P` \x84\x015a)B\x81a)\x07V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0[\x83\x81\x10\x15a)nW\x81\x81\x01Q\x83\x82\x01R` \x01a)VV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra)\x8F\x81` \x86\x01` \x86\x01a)SV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[\x84\x15\x15\x81R\x83` \x82\x01R\x82`@\x82\x01R`\x80``\x82\x01R`\0a)\xCA`\x80\x83\x01\x84a)wV[\x96\x95PPPPPPV[` \x81R`\0a\x04v` \x83\x01\x84a)wV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a*&Wa*&a)\xE7V[`@R\x91\x90PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a*GWa*Ga(vV[\x84Qa*R\x81a)\x07V[\x80\x94PP` \x80\x86\x01Q\x93P`@\x86\x01Q\x92P``\x86\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a*\xCBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x84\x90R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[\x81\x88\x01\x91P\x88`\x1F\x83\x01\x12a+3W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x84\x90R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x81\xFD[\x81Q\x81\x81\x11\x15a+EWa+Ea)\xE7V[a+W`\x1F\x82\x01`\x1F\x19\x16\x85\x01a)\xFDV[\x91P\x80\x82R\x89\x84\x82\x85\x01\x01\x11\x15a+\xBDW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x85\x90R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R`\x84\x81\xFD[a+\xCC\x81\x85\x84\x01\x86\x86\x01a)SV[P\x95\x98\x94\x97P\x92\x95PPPPV[`\0` \x82\x84\x03\x12\x15a+\xEFWa+\xEFa(vV[PQ\x91\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a,\nW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a,*WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\0` \x80\x83R`\0\x84T\x81`\x01\x82`\x01\x1C\x91P`\x01\x83\x16\x80a,TW`\x7F\x83\x16\x92P[` \x83\x10\x81\x03a,rWcNH{q`\xE0\x1B\x85R`\"`\x04R`$\x85\xFD[` \x88\x01\x83\x90R`@\x88\x01\x81\x80\x15a,\x91W`\x01\x81\x14a,\xA7Wa,\xD2V[`\xFF\x19\x86\x16\x82R\x84\x15\x15`\x05\x1B\x82\x01\x96Pa,\xD2V[`\0\x8B\x81R` \x90 `\0[\x86\x81\x10\x15a,\xCCW\x81T\x84\x82\x01R\x90\x85\x01\x90\x89\x01a,\xB3V[\x83\x01\x97PP[P\x94\x99\x98PPPPPPPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a,\xF9Wa,\xF9a(vV[\x83Q\x92P` \x84\x01Q\x91P`@\x84\x01Q\x90P\x92P\x92P\x92V[`\0`\xA0\x82\x84\x03\x12\x15a-'Wa-'a(vV[`@Q`\xA0\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a-JWa-Ja)\xE7V[`@\x90\x81R\x83Q\x82R` \x80\x85\x01Q\x90\x83\x01R\x83\x81\x01Q\x90\x82\x01R``\x80\x84\x01Q\x90\x82\x01R`\x80\x83\x01Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a-\x89W`\0\x80\xFD[`\x80\x82\x01R\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15a-\xC7Wa-\xC7a-\x95V[\x81\x81\x05\x83\x14\x82\x15\x17a\x04YWa\x04Ya-\x95V[`\0`\x01`\xFF\x1B\x82\x01a-\xF0Wa-\xF0a-\x95V[P`\0\x03\x90V[\x82\x81R`@` \x82\x01R`\0a.\x10`@\x83\x01\x84a)wV[\x94\x93PPPPV[`@\x81R`\x19`@\x82\x01R\x7FDEX swap failed with data\0\0\0\0\0\0\0``\x82\x01R`\x80` \x82\x01R`\0a\x04v`\x80\x83\x01\x84a)wV[\x81\x81\x03\x81\x81\x11\x15a\x04YWa\x04Ya-\x95V[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a.\x92Wa.\x92a-\x95V[P\x92\x91PPV[\x80\x82\x01\x80\x82\x11\x15a\x04YWa\x04Ya-\x95V[`\0\x82a.\xC9WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\x01`\xFF\x1B\x82\x14`\0\x19\x84\x14\x16\x15a.\xE3Wa.\xE3a-\x95V[P\x05\x90V[\x80\x82\x01\x82\x81\x12`\0\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a/\x08Wa/\x08a-\x95V[PP\x92\x91PPV\xFETarget contract does not contain\xA2dipfsX\"\x12 k\x1F(\xA8\xD0\xAD7\xD8\x934\xD5<\xA4\xB0\x05\xE2\xD3\x99q*@#K\xBDVn3E\x92\xAE\xE2\xE5dsolcC\0\x08\x16\x003";
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
