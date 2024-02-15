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
    const __BYTECODE: &[u8] = b"`\x80`@\x90\x80\x82R4b\0\x01YWP\x80Q`\x1Fb\0#/8\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x91`\x01`\x01`@\x1B\x03\x83\x11\x84\x84\x10\x17b\0\x01CW\x80\x84\x92`\xA0\x94\x87R\x839\x81\x01\x03\x12b\0\0\xF4Wb\0\0U\x81b\0\x01\xA6V[\x90b\0\0d` \x82\x01b\0\x01\xA6V[\x90b\0\0r\x84\x82\x01b\0\x01\xA6V[\x91b\0\0\x8F`\x80b\0\0\x87``\x85\x01b\0\x01\xA6V[\x93\x01b\0\x01\xA6V[\x91`\x01a\xFF\xFF\x19`\tT\x16\x17`\tU`\x01\x80`\xA0\x1B\x03\x80\x94\x81\x80\x94\x81`\x01\x80`\xA0\x1B\x03\x19\x99\x16\x89`\x02T\x16\x17`\x02U\x16\x87`\x01T\x16\x17`\x01U\x16\x85`\0T\x16\x17`\0U\x16\x83`\x03T\x16\x17`\x03U\x16\x90`\x04T\x16\x17`\x04UQa!n\x90\x81b\0\x01\xC1\x829\xF3[\x81QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[bF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01Ra7\xB7`\xF1\x1B`d\x82\x01R`\x84\x90\xFD[Q\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03b\0\x01\xBBWV[`\0\x80\xFD\xFE`\x80`@R`\x046\x10a\t\xF2W`\x005`\xE0\x1C\x80c-[l\xB9\x14a\x01|W\x80c6yr:\x14a\x01wW\x80c7\xC6\xA4J\x14a\x01rW\x80c8\xD5.\x0F\x14a\x01mW\x80c9(\xFF\x97\x14a\x01hW\x80cI\xA7\xA2m\x14a\x01cW\x80cdI\xFCW\x14a\x01^W\x80cgsB\xCE\x14a\x01YW\x80cr\xB9\x82F\x14a\x01TW\x80c\x85\xB3\x19\xFF\x14a\x01OW\x80c\x93e \xC3\x14a\x01JW\x80c\x96\xFB\xEE\x1D\x14a\x01EW\x80c\x99\x9B\x93\xAF\x14a\x01@W\x80c\x9F'\xEFO\x14a\x01;W\x80c\xAE\x97h\xA8\x14a\x016W\x80c\xBD%-\x06\x14a\x011W\x80c\xD0\xB7\x1B\x1E\x14a\x01,W\x80c\xD2L\xE6\xE5\x14a\x01'W\x80c\xD2\xF7&Z\x14a\x01\"W\x80c\xE5$\xF8I\x14a\x01\x1DW\x80c\xF3\xC9s\xCF\x14a\x01\x18W\x80c\xF9\0^\xB5\x14a\x01\x13Wc\xFA.Y\x94\x03a\t\xF2Wa\t\xD4V[a\x08\x93V[a\x08pV[a\x080V[a\x08\x07V[a\x07\xAAV[a\x07cV[a\x07\x16V[a\x06\xCAV[a\x06\xA1V[a\x06xV[a\x04\xF5V[a\x04\xD7V[a\x04\xB9V[a\x04\x9BV[a\x04}V[a\x04WV[a\x04.V[a\x03jV[a\x02\xC7V[a\x02uV[a\x02]V[a\x028V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01Ra7\xB7`\xF1\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[` \x90`\x03\x19\x01\x12a\x023W`\x045\x90V[a\x01\xD1V[4a\x02XW` a\x02Pa\x02K6a\x02!V[a\x0C\xE0V[`@Q\x90\x81R\xF3[a\x01\x81V[4a\x02XW` a\x02Pa\x02p6a\x02!V[a\x0E\xFFV[4a\x02XW`@6`\x03\x19\x01\x12a\x023W`\x045`$5\x90g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17\x82\x15\x15\x16\x15a\x02\xB7W` \x91`@Q\x91\x04\x81R\xF3[`\0\x80\xFD[`\0\x91\x03\x12a\x023WV[4a\x02XW`\x006`\x03\x19\x01\x12a\x023W`\x03T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[\x80\x15\x15\x03a\x02\xB7WV[`\0[\x83\x81\x10a\x03\rWPP`\0\x91\x01RV[\x81\x81\x01Q\x83\x82\x01R` \x01a\x02\xFDV[\x90` \x91a\x036\x81Q\x80\x92\x81\x85R\x85\x80\x86\x01\x91\x01a\x02\xFAV[`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[\x90\x92`\x80\x92a\x03g\x95\x94\x15\x15\x83R` \x83\x01R`@\x82\x01R\x81``\x82\x01R\x01\x90a\x03\x1DV[\x90V[4a\x02XW``6`\x03\x19\x01\x12a\x023W`$5a\x03\x87\x81a\x02\xF0V[`\x02T`\x01`\x01`\xA0\x1B\x03\x16\x90\x81;\x15a\x04)W`@Qc9(\xFF\x97`\xE0\x1B\x81R`\x04\x805\x90\x82\x01R\x90\x15\x15`$\x82\x01R`D\x805\x90\x82\x01R\x90`\0\x90\x81\x90\x83\x90`d\x90\x82\x90Z\xFA\x80\x15a\x04$W\x81\x80\x93\x81\x80\x93a\x03\xF7W[PP\x90a\x03\xF3\x91`@Q\x94\x85\x94\x85a\x03BV[\x03\x90\xF3[\x91P\x91Pa\x03\xF3\x93Pa\x04\x1C\x92P=\x80\x91\x83>a\x04\x14\x81\x83a\n\xD7V[\x81\x01\x90a\x0B\x15V[\x91\x938a\x03\xE0V[a\x0C\x8DV[a\nUV[4a\x02XW`\x006`\x03\x19\x01\x12a\x023W`\x02T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x02XW`\x006`\x03\x19\x01\x12a\x023W` `\xFF`\tT`\x08\x1C\x16`@Q\x90\x15\x15\x81R\xF3[4a\x02XW` 6`\x03\x19\x01\x12a\x023W` a\x02P`\x045a\x10\xD2V[4a\x02XW`\x006`\x03\x19\x01\x12a\x023W` `\x07T`@Q\x90\x81R\xF3[4a\x02XW`\x006`\x03\x19\x01\x12a\x023W` `\x08T`@Q\x90\x81R\xF3[4a\x02XW`\x006`\x03\x19\x01\x12a\x023W` `\x05T`@Q\x90\x81R\xF3[4a\x02XW`@6`\x03\x19\x01\x12a\x023W`\x02T`$5\x90`\x045\x90a\x051\x90a\x05%\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x80;\x15a\x04)W`@Qc\x03\xB4\xD1\x03`\xE4\x1B\x81R`\x04\x81\x01\x83\x90R` \x93\x90\x91\x84\x90\x83\x90`$\x90\x82\x90Z\xFA\x80\x15a\x04$Wa\x05\xAA\x7F\xD15<h\xE7\x9E\xF7\r\xE8N\xE9\r/\xAC\xF8E\xEC$\x89Q\x16\xD4\xA05\x05\xAAAxZ\xF7\x1FZ\x91a\x05\xC4\x94`\0\x91a\x06[W[P`@\x80Q\x91\x82RB` \x83\x01R\x90\x91\x82\x91\x82\x01\x90V[\x03\x90\xA1a\x05\xB6\x81a\x11\x90V[`\tT`\x08\x1C`\xFF\x16a\x13\x91V[`\tT`\xFF\x16`\x03Ta\x05\xE1\x90a\x05%\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x91\x82;\x15a\x04)W`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x92\x84\x90\x84\x90`$\x90\x82\x90Z\xFA\x92\x83\x15a\x04$Wa\x06\"\x94`\0\x94a\x06,W[PPa\x16\x8AV[a\x06*a\x19VV[\0[a\x06L\x92\x94P\x80=\x10a\x06TW[a\x06D\x81\x83a\n\xD7V[\x81\x01\x90a\x0C\x99V[\x918\x80a\x06\x1BV[P=a\x06:V[a\x06r\x91P\x87=\x89\x11a\x06TWa\x06D\x81\x83a\n\xD7V[8a\x05\x93V[4a\x02XW`\x006`\x03\x19\x01\x12a\x023W`\x04T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x02XW`\x006`\x03\x19\x01\x12a\x023W`\0T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x02XW`@6`\x03\x19\x01\x12a\x023W`\x045`$5\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x02\xB7W` \x90`@Q\x90`\x01g\r\xE0\xB6\xB3\xA7d\0\0`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x81R\xF3[4a\x02XW`@6`\x03\x19\x01\x12a\x023W`\x045`$5g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x81\x02\x92\x81\x84\x04\x14\x90\x15\x17\x81\x15\x15\x16\x15a\x02\xB7W` \x91`\x01`@Q\x92`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x81R\xF3[4a\x02XW` g\x1B\xC1mgN\xC8\0\0a\x07\xA1a\x07\x91a\x07\x9Cg\x13\xA0K\xBD\xFD\xC9\xBE\x88a\x07\x96a\x07\x916a\x02!V[a\x0E\x97V[\x05a\x0E\xEEV[a\x1C\xB7V[\x05`@Q\x90\x81R\xF3[4a\x02XWa\x07\xB86a\x02!V[a\x07\xC1\x81a\x0E\xEEV[\x80\x82\x02\x91`\0\x82\x12`\x01`\xFF\x1B\x82\x14\x16a\x08\x02W\x81\x83\x05\x14\x90\x15\x17\x15a\x08\x02Wg\"\xC9U\"\x95T\xC1\xB6a\x07\xA1a\x07\x91g\x1B\xC1mgN\xC8\0\0` \x94\x05a\x1D\xDCV[a\x0E\x81V[4a\x02XW`\x006`\x03\x19\x01\x12a\x023W`\x01T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x02XW`@6`\x03\x19\x01\x12a\x023W`\x045`$5\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x02\xB7W` \x90g\r\xE0\xB6\xB3\xA7d\0\0`@Q\x91\x04\x81R\xF3[4a\x02XW`\x006`\x03\x19\x01\x12a\x023W` `\xFF`\tT\x16`@Q\x90\x15\x15\x81R\xF3[4a\x02XW`@6`\x03\x19\x01\x12a\x023W`\x02T`\x045\x90`$5\x90a\x08\xC3\x90a\x05%\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x90\x81;\x15a\x04)W`@Qc\x03\xB4\xD1\x03`\xE4\x1B\x81R`\x04\x81\x01\x84\x90R` \x93\x90\x92\x84\x90\x84\x90`$\x90\x82\x90Z\xFA\x80\x15a\x04$Wa\t<\x7F\xD15<h\xE7\x9E\xF7\r\xE8N\xE9\r/\xAC\xF8E\xEC$\x89Q\x16\xD4\xA05\x05\xAAAxZ\xF7\x1FZ\x91a\tW\x95`\0\x91a\x06[WP`@\x80Q\x91\x82RB` \x83\x01R\x90\x91\x82\x91\x82\x01\x90V[\x03\x90\xA1a\tH\x82a\x11\x90V[`\tT`\x08\x1C`\xFF\x16\x90a\x16\x8AV[`\tT`\xFF\x16`\x03Ta\tt\x90a\x05%\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x90\x81;\x15a\x04)W`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x91\x83\x90\x83\x90`$\x90\x82\x90Z\xFA\x91\x82\x15a\x04$Wa\x06\"\x93`\0\x93a\t\xB5W[PPa\x13\x91V[a\t\xCC\x92\x93P\x80=\x10a\x06TWa\x06D\x81\x83a\n\xD7V[\x908\x80a\t\xAEV[4a\x02XW`\x006`\x03\x19\x01\x12a\x023W` `\x06T`@Q\x90\x81R\xF3[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01Rtnor receive functions`X\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01Rd code`\xD8\x1B`d\x82\x01R`\x84\x90\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\n\xD2W`@RV[a\n\xA8V[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\n\xD2W`@RV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\n\xD2W`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\x80\x81\x83\x03\x12a\x023W\x80Qa\x0B*\x81a\x02\xF0V[\x92` \x92\x83\x83\x01Q\x93`@\x84\x01Q\x93``\x81\x01Q\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x0C=W\x01\x82`\x1F\x82\x01\x12\x15a\x0B\xE4W\x80Q\x91a\x0Bf\x83a\n\xF9V[\x93a\x0Bt`@Q\x95\x86a\n\xD7V[\x83\x85R\x81\x84\x84\x01\x01\x11a\x0B\x90Wa\x03g\x92\x91\x84\x82\x01\x91\x01a\x02\xFAV[`\x84\x90`@Q\x90bF\x1B\xCD`\xE5\x1B\x82R`\x04\x82\x01R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x83\x90R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x84\x90R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[`@Q=`\0\x82>=\x90\xFD[\x90\x81` \x91\x03\x12a\x023WQ\x90V[\x15a\x0C\xAFWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x90\xFD[}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x90a\r\x0C`\0\x82\x13a\x0C\xA8V[q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06a\r(\x82a\x1A\xF0V[``\x92\x83\x82`\x9F\x03\x01\x1B`\x9F\x1C\x90`_\x19\x01}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x02\x92l\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x82m\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x81m\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x81m\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x81m\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x81m\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x81lFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x02\x8C\x1D\x01\x02\x8A\x1D\x01\x02\x88\x1D\x01\x02\x86\x1D\x01\x02\x84\x1D\x01\x02\x82\x1D\x01\x91x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x91l\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x90\x82m\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x81l\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x81m\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x81m\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x81l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x81\x01\x02\x89\x1D\x01\x02\x87\x1D\x01\x02\x85\x1D\x01\x02\x83\x1D\x01\x02\x90\x1D\x01\x02\x01\x05\x02\x01\x01`\xAE\x1D\x90V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x90g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x81\x02\x92\x81\x84\x05\x14\x90\x15\x17\x15a\x08\x02WV[\x90c;\x9A\xCA\0\x91\x82\x81\x02\x92\x81\x84\x05\x14\x90\x15\x17\x15a\x08\x02WV[`\0\x81\x90\x03\x91\x90`\x01`\xFF\x1B\x81\x14`\x01\x16a\x08\x02W`\0\x19\x83\x05\x03a\x08\x02WV[`\x01`\xFF\x1B\x81\x14a\x08\x02W`\0\x03\x90V[g\x06\xF0[Y\xD3\xB2\0\0\x81\x14a\x10\xCCWg\r\xE0\xB6\xB3\xA7d\0\0\x80\x82\x12\x15a\x10vW\x81\x15a\x10\x97W`\x01\x82`\x01\x1B\x91`\x02\x93\x83\x05`\x02\x03a\x08\x02W`\0\x83\x12\x80\x15a\x10\xBBW[a\x10\xA9W\x82\x15a\x10vWg\x1B\xC1mgN\xC8\0\0\x83\x14a\x10\x97W\x82\x12\x91\x82\x15a\x10\x88W\x92[a\x0Fp\x84a\x1F\x87V[\x80\x15a\x10vWa\x0F\xE2a\x0F\xA1a\x0F\x9Ca\x0F\x97a\x0F\x92a\x0F\xE7\x95\x99\x97\x96\x99a\x0C\xE0V[a HV[a\x10\xD2V[a\x0E\xB4V[a\x0F\xDDa\x0F\xB5a\x0F\xB0\x83a\x1F\xB2V[a\x1BbV[a\x0F\xD7a\x0F\xD2a\x0F\xCCa\x0F\xC7\x86a\x1F\xDDV[a\x1BzV[\x85a \xBFV[a\x1B\x92V[\x90a &V[a\x190V[a pV[\x93`\0\x92[\x81\x84\x10a\x10\x1EWPPPPa\x03g\x91a\x10\x0B\x91`\0\x14a\x10\x10Wa\x1F`V[a\x0E\xEEV[a\x10\x19\x90a\x0E\xEEV[a\x1F`V[\x90\x91a\x10l\x86a\x10fa\x106\x85a\x0F\xDD\x86\x99\x9Ba\x1C\xB7V[a\x0F\xD7a\x10Va\x10Qa\x10La\x10\x0B\x87\x80a \xBFV[a\x1D\xDCV[a \x98V[a\x10`\x83\x86a \xBFV[\x90a\x190V[\x90a\x1CpV[\x95\x01\x92\x91\x90a\x0F\xECV[`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x90\xFD[a\x10\x91\x90a\x18\xF4V[\x92a\x0FgV[`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x90\xFD[`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x90\xFD[Pg\x1B\xC1mgN\xC8\0\0\x83\x13a\x0FCV[P`\0\x90V[`\xB5\x81`\x01`\x88\x1B\x81\x10\x15a\x11yW[\x80i\x01\0\0\0\0\0\0\0\0\0b\x01\0\0\x92\x10\x15a\x11lW[e\x01\0\0\0\0\0\x81\x10\x15a\x11_W[c\x01\0\0\0\x81\x10\x15a\x11RW[\x01\x02`\x12\x1C`\x01\x90\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x80\x80\x92\x04\x10\x90\x03\x90V[`\x10\x1C\x91`\x08\x1B\x91a\x11\x16V[` \x1C\x91`\x10\x1B\x91a\x11\tV[`@\x1C\x91` \x1B\x91a\x10\xFAV[Ph\xB5\0\0\0\0\0\0\0\0\x90P`\x80\x82\x90\x1Ca\x10\xE2V[`\x04\x80Ta\x11\xA8\x90a\x05%\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x90\x81;\x15a\x04)W`@\x80Qcp\xA0\x821`\xE0\x1B\x80\x82R3\x82\x85\x01\x90\x81R` \x96\x91\x95\x93\x94\x93\x91\x92\x90\x87\x90\x82\x90\x81\x90\x83\x01\x03\x81\x85Z\xFA\x90\x81\x15a\x04$W`\0\x91a\x13tW[P\x82\x81\x10a\x13QWP\x80;\x15a\x04)W\x83Qcn\xB1v\x9F`\xE1\x1B\x81R3\x84\x82\x01\x90\x81R0` \x82\x01R\x87\x90\x82\x90\x81\x90`@\x01\x03\x81\x85Z\xFA\x90\x81\x15a\x04$W`\0\x91a\x134W[P\x82\x81\x10a\x13\rWP\x80;\x15a\x04)W\x83Qc#\xB8r\xDD`\xE0\x1B\x81R3\x84\x82\x01\x90\x81R0` \x82\x01R`@\x81\x01\x93\x90\x93R\x91`\0\x91\x83\x91\x82\x90\x84\x90\x82\x90``\x01\x03\x92Z\xF1\x80\x15a\x04$Wa\x12\xF4W[P\x80Ta\x12\x99\x90a\x05%\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x80;\x15a\x04)W\x91Q\x92\x83R0\x90\x83\x01\x90\x81R\x83\x91\x83\x91\x82\x90\x81\x90` \x01\x03\x91Z\xFA\x90\x81\x15a\x04$Wa\x12\xD5\x92`\0\x92a\x12\xD7W[PP`\x06UV[V[a\x12\xED\x92P\x80=\x10a\x06TWa\x06D\x81\x83a\n\xD7V[8\x80a\x12\xCEV[\x80a\x13\x01a\x13\x07\x92a\n\xBEV[\x80a\x02\xBCV[8a\x12\x82V[\x84Qc\xDAV\xD3\xC5`\xE0\x1B\x81R\x80\x85\x01\x91\x82R` \x82\x01\x84\x90R\x90\x81\x90`@\x01\x03\x90\xFD[\x03\x90\xFD[a\x13K\x91P\x87=\x89\x11a\x06TWa\x06D\x81\x83a\n\xD7V[8a\x123V[\x84Qc\n\xBEZ\x89`\xE0\x1B\x81R\x80\x85\x01\x91\x82R` \x82\x01\x84\x90R\x90\x81\x90`@\x01\x03\x90\xFD[a\x13\x8B\x91P\x87=\x89\x11a\x06TWa\x06D\x81\x83a\n\xD7V[8a\x11\xEDV[\x15a\x14\x8CW`\x03Ta\x13\xAD\x90a\x05%\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\0\x80T\x90\x91\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x80;\x15a\x04)W`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16`\x04\x83\x01R`$\x82\x01\x84\x90R\x82\x90\x82\x90`D\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x04$Wa\x14yW[P\x80Ta\x14\x19\x90a\x05%\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x03T`\x01`\x01`\xA0\x1B\x03\x16\x92\x90\x80;\x15a\x04)W`@Qc\xD0\x04\xF0\xF7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x94\x90\x94\x16`\x04\x85\x01R`$\x84\x01\x91\x90\x91R\x82\x90`D\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x04$Wa\x14lWPV[\x80a\x13\x01a\x12\xD5\x92a\n\xBEV[\x80a\x13\x01a\x14\x86\x92a\n\xBEV[8a\x14\x02V[`\x04Ta\x14\xA3\x90a\x05%\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\0\x80T\x90\x92\x91\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x80;\x15a\x04)W`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16`\x04\x83\x01R`$\x82\x01\x83\x90R\x83\x90\x82\x90`D\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x04$Wa\x15\xEBW[P\x81Ta\x15\x10\x90a\x05%\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x04T\x90\x91\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x82;\x15a\x04)W`@Qc\xD0\x04\xF0\xF7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16`\x04\x83\x01R`$\x82\x01R\x90\x82\x90\x82\x90`D\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x04$Wa\x15\xD8W[P`\x03Ta\x15{\x90a\x05%\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x80;\x15a\x04)W`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90` \x90\x82\x90`$\x90\x82\x90Z\xFA\x90\x81\x15a\x04$Wa\x12\xD5\x92\x91a\x15\xB9W[P`\x05UV[a\x15\xD2\x91P` =` \x11a\x06TWa\x06D\x81\x83a\n\xD7V[8a\x15\xB3V[\x80a\x13\x01a\x15\xE5\x92a\n\xBEV[8a\x15cV[\x80a\x13\x01a\x15\xF8\x92a\n\xBEV[8a\x14\xF9V[`@\x90a\x03g\x93\x92\x81R\x81` \x82\x01R\x01\x90a\x03\x1DV[=\x15a\x16@W=\x90a\x16&\x82a\n\xF9V[\x91a\x164`@Q\x93\x84a\n\xD7V[\x82R=`\0` \x84\x01>V[``\x90V[\x90`\x80a\x03g\x92`@\x81R`\x19`@\x82\x01R\x7FDEX swap failed with data\0\0\0\0\0\0\0``\x82\x01R\x81` \x82\x01R\x01\x90a\x03\x1DV[\x81\x15a\x18eW`\x03Ta\x16\xA7\x90a\x05%\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x01T`\x01`\x01`\xA0\x1B\x03\x16\x90\x80;\x15a\x04)W`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16`\x04\x83\x01R`$\x82\x01\x85\x90R`\0\x90\x82\x90`D\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x04$Wa\x18RW[P[`\x02Ta\x17\x12\x90a\x05%\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x90\x81;\x15a\x04)W`@Qc9(\xFF\x97`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R\x83\x15\x15`$\x82\x01R`D\x81\x01\x94\x90\x94R`\0\x93\x91\x84\x90\x83\x90`d\x90\x82\x90Z\xFA\x91\x82\x15a\x04$W\x84\x85\x91\x86\x90\x87\x95a\x180W[P\x81\x15a\x18\x12WPP`\x01Ta\x17\x81\x91Pa\x05%\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x91\x82;\x15a\x04)Wa\x17\xAC\x92\x85\x92\x83`@Q\x80\x96\x81\x95\x82\x94c\xBD\x06%\xAB`\xE0\x1B\x84R`\x04\x84\x01a\x15\xFEV[\x03\x92Z\xF1\x90\x81a\x17\xFFW[Pa\x17\xE0Wa\x130a\x17\xC7a\x16\x15V[`@Qcg\xA1k\x8D`\xE1\x1B\x81R\x91\x82\x91`\x04\x83\x01a\x16EV[\x15a\x17\xE8WPV[`\x03Ta\x15{\x90a\x05%\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x80a\x13\x01a\x18\x0C\x92a\n\xBEV[8a\x17\xB7V[\x84a\x130\x91`@Q\x94\x85\x94c\x03\x14\xE6#`\xE3\x1B\x86R`\x04\x86\x01a\x03BV[\x92PPPa\x18I\x91\x92P=\x80\x86\x83>a\x04\x14\x81\x83a\n\xD7V[\x93\x92\x908a\x17`V[\x80a\x13\x01a\x18_\x92a\n\xBEV[8a\x16\xF9V[`\x04Ta\x18|\x90a\x05%\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x01T`\x01`\x01`\xA0\x1B\x03\x16\x90\x80;\x15a\x04)W`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16`\x04\x83\x01R`$\x82\x01\x85\x90R`\0\x90\x82\x90`D\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x04$Wa\x18\xD4W[Pa\x16\xFBV[\x80a\x13\x01a\x18\xE1\x92a\n\xBEV[8a\x18\xCEV[\x91\x90\x82\x03\x91\x82\x11a\x08\x02WV[\x90g\x1B\xC1mgN\xC8\0\0`\0\x83\x82\x03\x93\x12\x81\x84\x12\x81\x16\x91\x84\x13\x90\x15\x16\x17a\x08\x02WV[\x90g\x11\x90\0\xAB\x10\x0F\xFB\xFF\x19\x82\x01\x91\x82\x13`\x01\x16a\x08\x02WV[\x81\x81\x03\x92\x91`\0\x13\x80\x15\x82\x85\x13\x16\x91\x84\x12\x16\x17a\x08\x02WV[\x91\x90\x82\x01\x80\x92\x11a\x08\x02WV[`\x04Ta\x19m\x90a\x05%\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x80;\x15a\x04)W`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90` \x90\x82\x90`$\x90\x82\x90Z\xFA\x80\x15a\x04$Wa\x19\xAC\x91`\0\x91a\x1A\xD1W[P`\x07UV[`\x07T`\x06T\x90\x81\x81\x10a\x1AgWa\x19\xE8\x7F5}\x90_\x181 \x97\x97\xDFMU\xD7\x9C\\[\xF1\xD9\xF71\x1C\x97j\xFD\x05\xE1=\x88\x1E\xAB\x9B\xC8\x92a\x1A\x0C\x92a\x18\xE7V[a\x19\xFCa\x19\xF7\x82`\x08Ta\x19IV[`\x08UV[`@Q\x90\x81R\x90\x81\x90` \x82\x01\x90V[\x03\x90\xA1`\x04Ta\x1A&\x90a\x05%\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x07T\x90\x80;\x15a\x04)W`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x92\x90\x92R`\0\x90\x82\x90`D\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x04$Wa\x14lWPV[\x90\x7F\xB6[.\x08]}\x04\x0C1?}N\x1A\xC9\x0FY7\x02o\xEEI~\x0E$\xA7\xEF\xF1jU\xE1\xC5\xEAa\x1A\x96a\x19\xFC\x84\x84a\x18\xE7V[\x03\x90\xA1a\x130a\x1A\xA6\x83\x83a\x190V[\x19`@Q\x93\x84\x93c\xB1n7\x83`\xE0\x1B\x85R`\x04\x85\x01`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[a\x1A\xEA\x91P` =` \x11a\x06TWa\x06D\x81\x83a\n\xD7V[8a\x19\xA6V[a\x1A\xFB\x81\x15\x15a\x0C\xA8V[\x80o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x07\x1B\x81\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x81\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x81\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x81\x81\x1C`\xFF\x10`\x03\x1B\x17\x81\x81\x1C`\x0F\x10`\x02\x1B\x17\x81\x81\x1C`\x03\x10`\x01\x1B\x17\x90\x81\x1C`\x01\x10\x17\x90V[\x90\x81g \x05\xFEO&\x8E\xA0\0\x01\x91\x82\x12`\x01\x16a\x08\x02WV[\x90\x81g\r\xC5R\x7Fd, \0\x01\x91\x82\x12`\x01\x16a\x08\x02WV[\x90\x81g\r\xE0\xB6\xB3\xA7d\0\0\x01\x91\x82\x12`\x01\x16a\x08\x02WV[\x90\x81g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19\x01\x91\x82\x12\x15`\x01\x16a\x08\x02WV[\x90\x81g\x14\xA8EL\x19\xE1\xAC\0\x01\x91\x82\x12`\x01\x16a\x08\x02WV[\x90\x81g\x0F\xC1\x0E\x01W\x82w\xFF\x19\x01\x91\x82\x12\x15`\x01\x16a\x08\x02WV[\x90\x81g\x03\xDE\xBD\x08;\x8C|\0\x01\x91\x82\x12`\x01\x16a\x08\x02WV[\x90\x81g\x02\x95\xD4\0\xEA2W\xFF\x19\x01\x91\x82\x12\x15`\x01\x16a\x08\x02WV[\x90\x81g\x01W\xD8\xB2\xEC\xC7\x08\0\x01\x91\x82\x12`\x01\x16a\x08\x02WV[\x90\x81g\x051\n\xA7\xD5!0\0\x01\x91\x82\x12`\x01\x16a\x08\x02WV[\x90\x81g\r\xE0\xCC=\x15a\0\0\x01\x91\x82\x12`\x01\x16a\x08\x02WV[\x91\x90\x91`\0\x83\x82\x01\x93\x84\x12\x91\x12\x90\x80\x15\x82\x16\x91\x15\x16\x17a\x08\x02WV[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x02\xB7Wg\x1B\xC1mgN\xC8\0\0\x90\x04\x90V[\x80\x15a\x1D\xCFWgV\x98\xEE\xF0fp\0\0\x81\x12\x15a\x10\xCCWgV\x98\xEE\xF0fo\xFF\xFF\x19\x81\x13\x15a\x1D\xC2W`\0a\x1D\xB2a\x1C\xEC\x83a!\x0BV[a\x1Dua\x10La\x1D\x06a\x1D\x01a\x0F\xD2\x85a\x1C\x8CV[a \x07V[\x92a\x1D\xADa\x1D\xA8a\x1D\xA3a\x1D\x9Ca\x1D\x96a\x1D\x91a\x1D\x8Ba\x1D\x86a\x1D\x80a\x1D{\x8Da\x1Dua\x1Dpa\x1Dja\x1Dea\x0F\xCCa\x1D`a\x1DZa\x1DUa\x1DOa\x1DJ\x8Aa \xE0V[a\x1B\xAAV[\x89a \xBFV[a\x1B\xC4V[\x87a \xBFV[a\x1B\xDCV[a\x1B\xF6V[\x83a \xBFV[a\x1C\x0EV[\x90a \xBFV[a\x1C(V[\x8Ca \xBFV[a\x1C@V[\x8Aa \xBFV[a\x1CXV[\x88a \xBFV[\x93\x80a \xBFV[a\x0E\xCDV[a\x19\x17V[a\x1CpV[\x91\x12\x15a\x03gWa\x03g\x90a\x18\xF4V[Pg\x1B\xC1mgN\xC8\0\0\x90V[Pg\r\xE0\xB6\xB3\xA7d\0\0\x90V[h\x02H\xCE6\xA7\x0C\xB2k>\x19\x81\x13\x15a\x10\xCCWh\x07U\xBFy\x8BJ\x1B\xF1\xE5\x81\x12\x15a\x1F,We\x03x-\xAC\xE9\xD9\x90`N\x1B\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x91``\x90`\x01`_\x1B\x84\x82\x84\x1B\x05\x01\x82\x1D\x93\x84\x02\x90\x03\x80l\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x81\x01\x02\x82\x1D\x90n\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dPn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x82n\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x81m\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x81m\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x81m\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x81l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x02\x8D\x1D\x01\x02\x8B\x1D\x01\x02\x89\x1D\x01\x02\x87\x1D\x01\x02\x85\x1D\x01\x93m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x93m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEA\x81\x01\x90\x84m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x91\x01\x01\x02\x90\x1D\x01\x02\x01\x05\x02\x90`\xC3\x03\x1C\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x90\xFD[g\x13\xA0K\xBD\xFD\xC9\xBE\x88\x90\x80\x82\x02\x91\x82\x05\x14`\x01\x16\x15a\x02\xB7Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x02\xB7Wg\x1B\xC1mgN\xC8\0\0\x90\x05\x90V[g\x03\xC1f\\z\xAB \0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x02\xB7Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[f\x9F2u$b\xA0\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x02\xB7Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[`\x01\x81\x15\x15\x16\x15a\x02\xB7Wn\xC0\x97\xCE{\xC9\x07\x15\xB3K\x9F\x10\0\0\0\0\x05\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17\x82\x15\x15\x16\x15a\x02\xB7W\x05\x90V[g\x1B\xC1mgN\xC7\xFF\xFF\x19\x81\x81\x02\x91`\x01\x91\x83\x05\x14\x16\x15a\x02\xB7Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\t\xD0(\xCCo _\xFF\x19\x81\x81\x02\x91`\x01\x91\x83\x05\x14\x16\x15a\x02\xB7Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x90\x80\x82\x02\x91\x82\x05\x14`\x01\x16\x15a\x02\xB7Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x02\xB7Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\x02_\x0F\xE1\x05\xA3\x14\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x02\xB7Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[`\x01`\xFF\x1B\x81\x14a!&W`\0\x81\x12\x15a\x03gW\x19`\x01\x01\x90V[`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x90\xFD\xFE\xA2dipfsX\"\x12 \xC4\x04\x92l\xBA\xD6>[A\x05\x163\x1C\xCE.\x1AQ\xD8%C\x0C\x1A\x02\xB9?\xCD\x89\x86\x9D7\xFCbdsolcC\0\x08\x16\x003";
    /// The bytecode of the contract.
    pub static ATOMICV2_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\t\xF2W`\x005`\xE0\x1C\x80c-[l\xB9\x14a\x01|W\x80c6yr:\x14a\x01wW\x80c7\xC6\xA4J\x14a\x01rW\x80c8\xD5.\x0F\x14a\x01mW\x80c9(\xFF\x97\x14a\x01hW\x80cI\xA7\xA2m\x14a\x01cW\x80cdI\xFCW\x14a\x01^W\x80cgsB\xCE\x14a\x01YW\x80cr\xB9\x82F\x14a\x01TW\x80c\x85\xB3\x19\xFF\x14a\x01OW\x80c\x93e \xC3\x14a\x01JW\x80c\x96\xFB\xEE\x1D\x14a\x01EW\x80c\x99\x9B\x93\xAF\x14a\x01@W\x80c\x9F'\xEFO\x14a\x01;W\x80c\xAE\x97h\xA8\x14a\x016W\x80c\xBD%-\x06\x14a\x011W\x80c\xD0\xB7\x1B\x1E\x14a\x01,W\x80c\xD2L\xE6\xE5\x14a\x01'W\x80c\xD2\xF7&Z\x14a\x01\"W\x80c\xE5$\xF8I\x14a\x01\x1DW\x80c\xF3\xC9s\xCF\x14a\x01\x18W\x80c\xF9\0^\xB5\x14a\x01\x13Wc\xFA.Y\x94\x03a\t\xF2Wa\t\xD4V[a\x08\x93V[a\x08pV[a\x080V[a\x08\x07V[a\x07\xAAV[a\x07cV[a\x07\x16V[a\x06\xCAV[a\x06\xA1V[a\x06xV[a\x04\xF5V[a\x04\xD7V[a\x04\xB9V[a\x04\x9BV[a\x04}V[a\x04WV[a\x04.V[a\x03jV[a\x02\xC7V[a\x02uV[a\x02]V[a\x028V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01Ra7\xB7`\xF1\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[` \x90`\x03\x19\x01\x12a\x023W`\x045\x90V[a\x01\xD1V[4a\x02XW` a\x02Pa\x02K6a\x02!V[a\x0C\xE0V[`@Q\x90\x81R\xF3[a\x01\x81V[4a\x02XW` a\x02Pa\x02p6a\x02!V[a\x0E\xFFV[4a\x02XW`@6`\x03\x19\x01\x12a\x023W`\x045`$5\x90g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17\x82\x15\x15\x16\x15a\x02\xB7W` \x91`@Q\x91\x04\x81R\xF3[`\0\x80\xFD[`\0\x91\x03\x12a\x023WV[4a\x02XW`\x006`\x03\x19\x01\x12a\x023W`\x03T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[\x80\x15\x15\x03a\x02\xB7WV[`\0[\x83\x81\x10a\x03\rWPP`\0\x91\x01RV[\x81\x81\x01Q\x83\x82\x01R` \x01a\x02\xFDV[\x90` \x91a\x036\x81Q\x80\x92\x81\x85R\x85\x80\x86\x01\x91\x01a\x02\xFAV[`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[\x90\x92`\x80\x92a\x03g\x95\x94\x15\x15\x83R` \x83\x01R`@\x82\x01R\x81``\x82\x01R\x01\x90a\x03\x1DV[\x90V[4a\x02XW``6`\x03\x19\x01\x12a\x023W`$5a\x03\x87\x81a\x02\xF0V[`\x02T`\x01`\x01`\xA0\x1B\x03\x16\x90\x81;\x15a\x04)W`@Qc9(\xFF\x97`\xE0\x1B\x81R`\x04\x805\x90\x82\x01R\x90\x15\x15`$\x82\x01R`D\x805\x90\x82\x01R\x90`\0\x90\x81\x90\x83\x90`d\x90\x82\x90Z\xFA\x80\x15a\x04$W\x81\x80\x93\x81\x80\x93a\x03\xF7W[PP\x90a\x03\xF3\x91`@Q\x94\x85\x94\x85a\x03BV[\x03\x90\xF3[\x91P\x91Pa\x03\xF3\x93Pa\x04\x1C\x92P=\x80\x91\x83>a\x04\x14\x81\x83a\n\xD7V[\x81\x01\x90a\x0B\x15V[\x91\x938a\x03\xE0V[a\x0C\x8DV[a\nUV[4a\x02XW`\x006`\x03\x19\x01\x12a\x023W`\x02T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x02XW`\x006`\x03\x19\x01\x12a\x023W` `\xFF`\tT`\x08\x1C\x16`@Q\x90\x15\x15\x81R\xF3[4a\x02XW` 6`\x03\x19\x01\x12a\x023W` a\x02P`\x045a\x10\xD2V[4a\x02XW`\x006`\x03\x19\x01\x12a\x023W` `\x07T`@Q\x90\x81R\xF3[4a\x02XW`\x006`\x03\x19\x01\x12a\x023W` `\x08T`@Q\x90\x81R\xF3[4a\x02XW`\x006`\x03\x19\x01\x12a\x023W` `\x05T`@Q\x90\x81R\xF3[4a\x02XW`@6`\x03\x19\x01\x12a\x023W`\x02T`$5\x90`\x045\x90a\x051\x90a\x05%\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x80;\x15a\x04)W`@Qc\x03\xB4\xD1\x03`\xE4\x1B\x81R`\x04\x81\x01\x83\x90R` \x93\x90\x91\x84\x90\x83\x90`$\x90\x82\x90Z\xFA\x80\x15a\x04$Wa\x05\xAA\x7F\xD15<h\xE7\x9E\xF7\r\xE8N\xE9\r/\xAC\xF8E\xEC$\x89Q\x16\xD4\xA05\x05\xAAAxZ\xF7\x1FZ\x91a\x05\xC4\x94`\0\x91a\x06[W[P`@\x80Q\x91\x82RB` \x83\x01R\x90\x91\x82\x91\x82\x01\x90V[\x03\x90\xA1a\x05\xB6\x81a\x11\x90V[`\tT`\x08\x1C`\xFF\x16a\x13\x91V[`\tT`\xFF\x16`\x03Ta\x05\xE1\x90a\x05%\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x91\x82;\x15a\x04)W`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x92\x84\x90\x84\x90`$\x90\x82\x90Z\xFA\x92\x83\x15a\x04$Wa\x06\"\x94`\0\x94a\x06,W[PPa\x16\x8AV[a\x06*a\x19VV[\0[a\x06L\x92\x94P\x80=\x10a\x06TW[a\x06D\x81\x83a\n\xD7V[\x81\x01\x90a\x0C\x99V[\x918\x80a\x06\x1BV[P=a\x06:V[a\x06r\x91P\x87=\x89\x11a\x06TWa\x06D\x81\x83a\n\xD7V[8a\x05\x93V[4a\x02XW`\x006`\x03\x19\x01\x12a\x023W`\x04T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x02XW`\x006`\x03\x19\x01\x12a\x023W`\0T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x02XW`@6`\x03\x19\x01\x12a\x023W`\x045`$5\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x02\xB7W` \x90`@Q\x90`\x01g\r\xE0\xB6\xB3\xA7d\0\0`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x81R\xF3[4a\x02XW`@6`\x03\x19\x01\x12a\x023W`\x045`$5g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x81\x02\x92\x81\x84\x04\x14\x90\x15\x17\x81\x15\x15\x16\x15a\x02\xB7W` \x91`\x01`@Q\x92`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x81R\xF3[4a\x02XW` g\x1B\xC1mgN\xC8\0\0a\x07\xA1a\x07\x91a\x07\x9Cg\x13\xA0K\xBD\xFD\xC9\xBE\x88a\x07\x96a\x07\x916a\x02!V[a\x0E\x97V[\x05a\x0E\xEEV[a\x1C\xB7V[\x05`@Q\x90\x81R\xF3[4a\x02XWa\x07\xB86a\x02!V[a\x07\xC1\x81a\x0E\xEEV[\x80\x82\x02\x91`\0\x82\x12`\x01`\xFF\x1B\x82\x14\x16a\x08\x02W\x81\x83\x05\x14\x90\x15\x17\x15a\x08\x02Wg\"\xC9U\"\x95T\xC1\xB6a\x07\xA1a\x07\x91g\x1B\xC1mgN\xC8\0\0` \x94\x05a\x1D\xDCV[a\x0E\x81V[4a\x02XW`\x006`\x03\x19\x01\x12a\x023W`\x01T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x02XW`@6`\x03\x19\x01\x12a\x023W`\x045`$5\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x02\xB7W` \x90g\r\xE0\xB6\xB3\xA7d\0\0`@Q\x91\x04\x81R\xF3[4a\x02XW`\x006`\x03\x19\x01\x12a\x023W` `\xFF`\tT\x16`@Q\x90\x15\x15\x81R\xF3[4a\x02XW`@6`\x03\x19\x01\x12a\x023W`\x02T`\x045\x90`$5\x90a\x08\xC3\x90a\x05%\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x90\x81;\x15a\x04)W`@Qc\x03\xB4\xD1\x03`\xE4\x1B\x81R`\x04\x81\x01\x84\x90R` \x93\x90\x92\x84\x90\x84\x90`$\x90\x82\x90Z\xFA\x80\x15a\x04$Wa\t<\x7F\xD15<h\xE7\x9E\xF7\r\xE8N\xE9\r/\xAC\xF8E\xEC$\x89Q\x16\xD4\xA05\x05\xAAAxZ\xF7\x1FZ\x91a\tW\x95`\0\x91a\x06[WP`@\x80Q\x91\x82RB` \x83\x01R\x90\x91\x82\x91\x82\x01\x90V[\x03\x90\xA1a\tH\x82a\x11\x90V[`\tT`\x08\x1C`\xFF\x16\x90a\x16\x8AV[`\tT`\xFF\x16`\x03Ta\tt\x90a\x05%\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x90\x81;\x15a\x04)W`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x91\x83\x90\x83\x90`$\x90\x82\x90Z\xFA\x91\x82\x15a\x04$Wa\x06\"\x93`\0\x93a\t\xB5W[PPa\x13\x91V[a\t\xCC\x92\x93P\x80=\x10a\x06TWa\x06D\x81\x83a\n\xD7V[\x908\x80a\t\xAEV[4a\x02XW`\x006`\x03\x19\x01\x12a\x023W` `\x06T`@Q\x90\x81R\xF3[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01Rtnor receive functions`X\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01Rd code`\xD8\x1B`d\x82\x01R`\x84\x90\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\n\xD2W`@RV[a\n\xA8V[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\n\xD2W`@RV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\n\xD2W`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\x80\x81\x83\x03\x12a\x023W\x80Qa\x0B*\x81a\x02\xF0V[\x92` \x92\x83\x83\x01Q\x93`@\x84\x01Q\x93``\x81\x01Q\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x0C=W\x01\x82`\x1F\x82\x01\x12\x15a\x0B\xE4W\x80Q\x91a\x0Bf\x83a\n\xF9V[\x93a\x0Bt`@Q\x95\x86a\n\xD7V[\x83\x85R\x81\x84\x84\x01\x01\x11a\x0B\x90Wa\x03g\x92\x91\x84\x82\x01\x91\x01a\x02\xFAV[`\x84\x90`@Q\x90bF\x1B\xCD`\xE5\x1B\x82R`\x04\x82\x01R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x83\x90R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x84\x90R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[`@Q=`\0\x82>=\x90\xFD[\x90\x81` \x91\x03\x12a\x023WQ\x90V[\x15a\x0C\xAFWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x90\xFD[}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x90a\r\x0C`\0\x82\x13a\x0C\xA8V[q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06a\r(\x82a\x1A\xF0V[``\x92\x83\x82`\x9F\x03\x01\x1B`\x9F\x1C\x90`_\x19\x01}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x02\x92l\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x82m\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x81m\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x81m\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x81m\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x81m\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x81lFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x02\x8C\x1D\x01\x02\x8A\x1D\x01\x02\x88\x1D\x01\x02\x86\x1D\x01\x02\x84\x1D\x01\x02\x82\x1D\x01\x91x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x91l\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x90\x82m\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x81l\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x81m\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x81m\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x81l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x81\x01\x02\x89\x1D\x01\x02\x87\x1D\x01\x02\x85\x1D\x01\x02\x83\x1D\x01\x02\x90\x1D\x01\x02\x01\x05\x02\x01\x01`\xAE\x1D\x90V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x90g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x81\x02\x92\x81\x84\x05\x14\x90\x15\x17\x15a\x08\x02WV[\x90c;\x9A\xCA\0\x91\x82\x81\x02\x92\x81\x84\x05\x14\x90\x15\x17\x15a\x08\x02WV[`\0\x81\x90\x03\x91\x90`\x01`\xFF\x1B\x81\x14`\x01\x16a\x08\x02W`\0\x19\x83\x05\x03a\x08\x02WV[`\x01`\xFF\x1B\x81\x14a\x08\x02W`\0\x03\x90V[g\x06\xF0[Y\xD3\xB2\0\0\x81\x14a\x10\xCCWg\r\xE0\xB6\xB3\xA7d\0\0\x80\x82\x12\x15a\x10vW\x81\x15a\x10\x97W`\x01\x82`\x01\x1B\x91`\x02\x93\x83\x05`\x02\x03a\x08\x02W`\0\x83\x12\x80\x15a\x10\xBBW[a\x10\xA9W\x82\x15a\x10vWg\x1B\xC1mgN\xC8\0\0\x83\x14a\x10\x97W\x82\x12\x91\x82\x15a\x10\x88W\x92[a\x0Fp\x84a\x1F\x87V[\x80\x15a\x10vWa\x0F\xE2a\x0F\xA1a\x0F\x9Ca\x0F\x97a\x0F\x92a\x0F\xE7\x95\x99\x97\x96\x99a\x0C\xE0V[a HV[a\x10\xD2V[a\x0E\xB4V[a\x0F\xDDa\x0F\xB5a\x0F\xB0\x83a\x1F\xB2V[a\x1BbV[a\x0F\xD7a\x0F\xD2a\x0F\xCCa\x0F\xC7\x86a\x1F\xDDV[a\x1BzV[\x85a \xBFV[a\x1B\x92V[\x90a &V[a\x190V[a pV[\x93`\0\x92[\x81\x84\x10a\x10\x1EWPPPPa\x03g\x91a\x10\x0B\x91`\0\x14a\x10\x10Wa\x1F`V[a\x0E\xEEV[a\x10\x19\x90a\x0E\xEEV[a\x1F`V[\x90\x91a\x10l\x86a\x10fa\x106\x85a\x0F\xDD\x86\x99\x9Ba\x1C\xB7V[a\x0F\xD7a\x10Va\x10Qa\x10La\x10\x0B\x87\x80a \xBFV[a\x1D\xDCV[a \x98V[a\x10`\x83\x86a \xBFV[\x90a\x190V[\x90a\x1CpV[\x95\x01\x92\x91\x90a\x0F\xECV[`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x90\xFD[a\x10\x91\x90a\x18\xF4V[\x92a\x0FgV[`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x90\xFD[`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x90\xFD[Pg\x1B\xC1mgN\xC8\0\0\x83\x13a\x0FCV[P`\0\x90V[`\xB5\x81`\x01`\x88\x1B\x81\x10\x15a\x11yW[\x80i\x01\0\0\0\0\0\0\0\0\0b\x01\0\0\x92\x10\x15a\x11lW[e\x01\0\0\0\0\0\x81\x10\x15a\x11_W[c\x01\0\0\0\x81\x10\x15a\x11RW[\x01\x02`\x12\x1C`\x01\x90\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x80\x80\x92\x04\x10\x90\x03\x90V[`\x10\x1C\x91`\x08\x1B\x91a\x11\x16V[` \x1C\x91`\x10\x1B\x91a\x11\tV[`@\x1C\x91` \x1B\x91a\x10\xFAV[Ph\xB5\0\0\0\0\0\0\0\0\x90P`\x80\x82\x90\x1Ca\x10\xE2V[`\x04\x80Ta\x11\xA8\x90a\x05%\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x90\x81;\x15a\x04)W`@\x80Qcp\xA0\x821`\xE0\x1B\x80\x82R3\x82\x85\x01\x90\x81R` \x96\x91\x95\x93\x94\x93\x91\x92\x90\x87\x90\x82\x90\x81\x90\x83\x01\x03\x81\x85Z\xFA\x90\x81\x15a\x04$W`\0\x91a\x13tW[P\x82\x81\x10a\x13QWP\x80;\x15a\x04)W\x83Qcn\xB1v\x9F`\xE1\x1B\x81R3\x84\x82\x01\x90\x81R0` \x82\x01R\x87\x90\x82\x90\x81\x90`@\x01\x03\x81\x85Z\xFA\x90\x81\x15a\x04$W`\0\x91a\x134W[P\x82\x81\x10a\x13\rWP\x80;\x15a\x04)W\x83Qc#\xB8r\xDD`\xE0\x1B\x81R3\x84\x82\x01\x90\x81R0` \x82\x01R`@\x81\x01\x93\x90\x93R\x91`\0\x91\x83\x91\x82\x90\x84\x90\x82\x90``\x01\x03\x92Z\xF1\x80\x15a\x04$Wa\x12\xF4W[P\x80Ta\x12\x99\x90a\x05%\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x80;\x15a\x04)W\x91Q\x92\x83R0\x90\x83\x01\x90\x81R\x83\x91\x83\x91\x82\x90\x81\x90` \x01\x03\x91Z\xFA\x90\x81\x15a\x04$Wa\x12\xD5\x92`\0\x92a\x12\xD7W[PP`\x06UV[V[a\x12\xED\x92P\x80=\x10a\x06TWa\x06D\x81\x83a\n\xD7V[8\x80a\x12\xCEV[\x80a\x13\x01a\x13\x07\x92a\n\xBEV[\x80a\x02\xBCV[8a\x12\x82V[\x84Qc\xDAV\xD3\xC5`\xE0\x1B\x81R\x80\x85\x01\x91\x82R` \x82\x01\x84\x90R\x90\x81\x90`@\x01\x03\x90\xFD[\x03\x90\xFD[a\x13K\x91P\x87=\x89\x11a\x06TWa\x06D\x81\x83a\n\xD7V[8a\x123V[\x84Qc\n\xBEZ\x89`\xE0\x1B\x81R\x80\x85\x01\x91\x82R` \x82\x01\x84\x90R\x90\x81\x90`@\x01\x03\x90\xFD[a\x13\x8B\x91P\x87=\x89\x11a\x06TWa\x06D\x81\x83a\n\xD7V[8a\x11\xEDV[\x15a\x14\x8CW`\x03Ta\x13\xAD\x90a\x05%\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\0\x80T\x90\x91\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x80;\x15a\x04)W`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16`\x04\x83\x01R`$\x82\x01\x84\x90R\x82\x90\x82\x90`D\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x04$Wa\x14yW[P\x80Ta\x14\x19\x90a\x05%\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x03T`\x01`\x01`\xA0\x1B\x03\x16\x92\x90\x80;\x15a\x04)W`@Qc\xD0\x04\xF0\xF7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x94\x90\x94\x16`\x04\x85\x01R`$\x84\x01\x91\x90\x91R\x82\x90`D\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x04$Wa\x14lWPV[\x80a\x13\x01a\x12\xD5\x92a\n\xBEV[\x80a\x13\x01a\x14\x86\x92a\n\xBEV[8a\x14\x02V[`\x04Ta\x14\xA3\x90a\x05%\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\0\x80T\x90\x92\x91\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x80;\x15a\x04)W`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16`\x04\x83\x01R`$\x82\x01\x83\x90R\x83\x90\x82\x90`D\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x04$Wa\x15\xEBW[P\x81Ta\x15\x10\x90a\x05%\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x04T\x90\x91\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x82;\x15a\x04)W`@Qc\xD0\x04\xF0\xF7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16`\x04\x83\x01R`$\x82\x01R\x90\x82\x90\x82\x90`D\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x04$Wa\x15\xD8W[P`\x03Ta\x15{\x90a\x05%\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x80;\x15a\x04)W`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90` \x90\x82\x90`$\x90\x82\x90Z\xFA\x90\x81\x15a\x04$Wa\x12\xD5\x92\x91a\x15\xB9W[P`\x05UV[a\x15\xD2\x91P` =` \x11a\x06TWa\x06D\x81\x83a\n\xD7V[8a\x15\xB3V[\x80a\x13\x01a\x15\xE5\x92a\n\xBEV[8a\x15cV[\x80a\x13\x01a\x15\xF8\x92a\n\xBEV[8a\x14\xF9V[`@\x90a\x03g\x93\x92\x81R\x81` \x82\x01R\x01\x90a\x03\x1DV[=\x15a\x16@W=\x90a\x16&\x82a\n\xF9V[\x91a\x164`@Q\x93\x84a\n\xD7V[\x82R=`\0` \x84\x01>V[``\x90V[\x90`\x80a\x03g\x92`@\x81R`\x19`@\x82\x01R\x7FDEX swap failed with data\0\0\0\0\0\0\0``\x82\x01R\x81` \x82\x01R\x01\x90a\x03\x1DV[\x81\x15a\x18eW`\x03Ta\x16\xA7\x90a\x05%\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x01T`\x01`\x01`\xA0\x1B\x03\x16\x90\x80;\x15a\x04)W`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16`\x04\x83\x01R`$\x82\x01\x85\x90R`\0\x90\x82\x90`D\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x04$Wa\x18RW[P[`\x02Ta\x17\x12\x90a\x05%\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x90\x81;\x15a\x04)W`@Qc9(\xFF\x97`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R\x83\x15\x15`$\x82\x01R`D\x81\x01\x94\x90\x94R`\0\x93\x91\x84\x90\x83\x90`d\x90\x82\x90Z\xFA\x91\x82\x15a\x04$W\x84\x85\x91\x86\x90\x87\x95a\x180W[P\x81\x15a\x18\x12WPP`\x01Ta\x17\x81\x91Pa\x05%\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x91\x82;\x15a\x04)Wa\x17\xAC\x92\x85\x92\x83`@Q\x80\x96\x81\x95\x82\x94c\xBD\x06%\xAB`\xE0\x1B\x84R`\x04\x84\x01a\x15\xFEV[\x03\x92Z\xF1\x90\x81a\x17\xFFW[Pa\x17\xE0Wa\x130a\x17\xC7a\x16\x15V[`@Qcg\xA1k\x8D`\xE1\x1B\x81R\x91\x82\x91`\x04\x83\x01a\x16EV[\x15a\x17\xE8WPV[`\x03Ta\x15{\x90a\x05%\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x80a\x13\x01a\x18\x0C\x92a\n\xBEV[8a\x17\xB7V[\x84a\x130\x91`@Q\x94\x85\x94c\x03\x14\xE6#`\xE3\x1B\x86R`\x04\x86\x01a\x03BV[\x92PPPa\x18I\x91\x92P=\x80\x86\x83>a\x04\x14\x81\x83a\n\xD7V[\x93\x92\x908a\x17`V[\x80a\x13\x01a\x18_\x92a\n\xBEV[8a\x16\xF9V[`\x04Ta\x18|\x90a\x05%\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x01T`\x01`\x01`\xA0\x1B\x03\x16\x90\x80;\x15a\x04)W`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16`\x04\x83\x01R`$\x82\x01\x85\x90R`\0\x90\x82\x90`D\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x04$Wa\x18\xD4W[Pa\x16\xFBV[\x80a\x13\x01a\x18\xE1\x92a\n\xBEV[8a\x18\xCEV[\x91\x90\x82\x03\x91\x82\x11a\x08\x02WV[\x90g\x1B\xC1mgN\xC8\0\0`\0\x83\x82\x03\x93\x12\x81\x84\x12\x81\x16\x91\x84\x13\x90\x15\x16\x17a\x08\x02WV[\x90g\x11\x90\0\xAB\x10\x0F\xFB\xFF\x19\x82\x01\x91\x82\x13`\x01\x16a\x08\x02WV[\x81\x81\x03\x92\x91`\0\x13\x80\x15\x82\x85\x13\x16\x91\x84\x12\x16\x17a\x08\x02WV[\x91\x90\x82\x01\x80\x92\x11a\x08\x02WV[`\x04Ta\x19m\x90a\x05%\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x80;\x15a\x04)W`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90` \x90\x82\x90`$\x90\x82\x90Z\xFA\x80\x15a\x04$Wa\x19\xAC\x91`\0\x91a\x1A\xD1W[P`\x07UV[`\x07T`\x06T\x90\x81\x81\x10a\x1AgWa\x19\xE8\x7F5}\x90_\x181 \x97\x97\xDFMU\xD7\x9C\\[\xF1\xD9\xF71\x1C\x97j\xFD\x05\xE1=\x88\x1E\xAB\x9B\xC8\x92a\x1A\x0C\x92a\x18\xE7V[a\x19\xFCa\x19\xF7\x82`\x08Ta\x19IV[`\x08UV[`@Q\x90\x81R\x90\x81\x90` \x82\x01\x90V[\x03\x90\xA1`\x04Ta\x1A&\x90a\x05%\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x07T\x90\x80;\x15a\x04)W`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x92\x90\x92R`\0\x90\x82\x90`D\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x04$Wa\x14lWPV[\x90\x7F\xB6[.\x08]}\x04\x0C1?}N\x1A\xC9\x0FY7\x02o\xEEI~\x0E$\xA7\xEF\xF1jU\xE1\xC5\xEAa\x1A\x96a\x19\xFC\x84\x84a\x18\xE7V[\x03\x90\xA1a\x130a\x1A\xA6\x83\x83a\x190V[\x19`@Q\x93\x84\x93c\xB1n7\x83`\xE0\x1B\x85R`\x04\x85\x01`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[a\x1A\xEA\x91P` =` \x11a\x06TWa\x06D\x81\x83a\n\xD7V[8a\x19\xA6V[a\x1A\xFB\x81\x15\x15a\x0C\xA8V[\x80o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x07\x1B\x81\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x81\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x81\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x81\x81\x1C`\xFF\x10`\x03\x1B\x17\x81\x81\x1C`\x0F\x10`\x02\x1B\x17\x81\x81\x1C`\x03\x10`\x01\x1B\x17\x90\x81\x1C`\x01\x10\x17\x90V[\x90\x81g \x05\xFEO&\x8E\xA0\0\x01\x91\x82\x12`\x01\x16a\x08\x02WV[\x90\x81g\r\xC5R\x7Fd, \0\x01\x91\x82\x12`\x01\x16a\x08\x02WV[\x90\x81g\r\xE0\xB6\xB3\xA7d\0\0\x01\x91\x82\x12`\x01\x16a\x08\x02WV[\x90\x81g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19\x01\x91\x82\x12\x15`\x01\x16a\x08\x02WV[\x90\x81g\x14\xA8EL\x19\xE1\xAC\0\x01\x91\x82\x12`\x01\x16a\x08\x02WV[\x90\x81g\x0F\xC1\x0E\x01W\x82w\xFF\x19\x01\x91\x82\x12\x15`\x01\x16a\x08\x02WV[\x90\x81g\x03\xDE\xBD\x08;\x8C|\0\x01\x91\x82\x12`\x01\x16a\x08\x02WV[\x90\x81g\x02\x95\xD4\0\xEA2W\xFF\x19\x01\x91\x82\x12\x15`\x01\x16a\x08\x02WV[\x90\x81g\x01W\xD8\xB2\xEC\xC7\x08\0\x01\x91\x82\x12`\x01\x16a\x08\x02WV[\x90\x81g\x051\n\xA7\xD5!0\0\x01\x91\x82\x12`\x01\x16a\x08\x02WV[\x90\x81g\r\xE0\xCC=\x15a\0\0\x01\x91\x82\x12`\x01\x16a\x08\x02WV[\x91\x90\x91`\0\x83\x82\x01\x93\x84\x12\x91\x12\x90\x80\x15\x82\x16\x91\x15\x16\x17a\x08\x02WV[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x02\xB7Wg\x1B\xC1mgN\xC8\0\0\x90\x04\x90V[\x80\x15a\x1D\xCFWgV\x98\xEE\xF0fp\0\0\x81\x12\x15a\x10\xCCWgV\x98\xEE\xF0fo\xFF\xFF\x19\x81\x13\x15a\x1D\xC2W`\0a\x1D\xB2a\x1C\xEC\x83a!\x0BV[a\x1Dua\x10La\x1D\x06a\x1D\x01a\x0F\xD2\x85a\x1C\x8CV[a \x07V[\x92a\x1D\xADa\x1D\xA8a\x1D\xA3a\x1D\x9Ca\x1D\x96a\x1D\x91a\x1D\x8Ba\x1D\x86a\x1D\x80a\x1D{\x8Da\x1Dua\x1Dpa\x1Dja\x1Dea\x0F\xCCa\x1D`a\x1DZa\x1DUa\x1DOa\x1DJ\x8Aa \xE0V[a\x1B\xAAV[\x89a \xBFV[a\x1B\xC4V[\x87a \xBFV[a\x1B\xDCV[a\x1B\xF6V[\x83a \xBFV[a\x1C\x0EV[\x90a \xBFV[a\x1C(V[\x8Ca \xBFV[a\x1C@V[\x8Aa \xBFV[a\x1CXV[\x88a \xBFV[\x93\x80a \xBFV[a\x0E\xCDV[a\x19\x17V[a\x1CpV[\x91\x12\x15a\x03gWa\x03g\x90a\x18\xF4V[Pg\x1B\xC1mgN\xC8\0\0\x90V[Pg\r\xE0\xB6\xB3\xA7d\0\0\x90V[h\x02H\xCE6\xA7\x0C\xB2k>\x19\x81\x13\x15a\x10\xCCWh\x07U\xBFy\x8BJ\x1B\xF1\xE5\x81\x12\x15a\x1F,We\x03x-\xAC\xE9\xD9\x90`N\x1B\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x91``\x90`\x01`_\x1B\x84\x82\x84\x1B\x05\x01\x82\x1D\x93\x84\x02\x90\x03\x80l\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x81\x01\x02\x82\x1D\x90n\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dPn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x82n\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x81m\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x81m\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x81m\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x81l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x02\x8D\x1D\x01\x02\x8B\x1D\x01\x02\x89\x1D\x01\x02\x87\x1D\x01\x02\x85\x1D\x01\x93m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x93m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEA\x81\x01\x90\x84m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x91\x01\x01\x02\x90\x1D\x01\x02\x01\x05\x02\x90`\xC3\x03\x1C\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x90\xFD[g\x13\xA0K\xBD\xFD\xC9\xBE\x88\x90\x80\x82\x02\x91\x82\x05\x14`\x01\x16\x15a\x02\xB7Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x02\xB7Wg\x1B\xC1mgN\xC8\0\0\x90\x05\x90V[g\x03\xC1f\\z\xAB \0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x02\xB7Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[f\x9F2u$b\xA0\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x02\xB7Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[`\x01\x81\x15\x15\x16\x15a\x02\xB7Wn\xC0\x97\xCE{\xC9\x07\x15\xB3K\x9F\x10\0\0\0\0\x05\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17\x82\x15\x15\x16\x15a\x02\xB7W\x05\x90V[g\x1B\xC1mgN\xC7\xFF\xFF\x19\x81\x81\x02\x91`\x01\x91\x83\x05\x14\x16\x15a\x02\xB7Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\t\xD0(\xCCo _\xFF\x19\x81\x81\x02\x91`\x01\x91\x83\x05\x14\x16\x15a\x02\xB7Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x90\x80\x82\x02\x91\x82\x05\x14`\x01\x16\x15a\x02\xB7Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x02\xB7Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\x02_\x0F\xE1\x05\xA3\x14\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x02\xB7Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[`\x01`\xFF\x1B\x81\x14a!&W`\0\x81\x12\x15a\x03gW\x19`\x01\x01\x90V[`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x90\xFD\xFE\xA2dipfsX\"\x12 \xC4\x04\x92l\xBA\xD6>[A\x05\x163\x1C\xCE.\x1AQ\xD8%C\x0C\x1A\x02\xB9?\xCD\x89\x86\x9D7\xFCbdsolcC\0\x08\x16\x003";
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
        LossFilter(LossFilter),
        PriceFilter(PriceFilter),
        ProfitFilter(ProfitFilter),
    }
    impl ::ethers::contract::EthLogDecode for AtomicV2Events {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
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
                Self::LossFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::PriceFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProfitFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
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
}
