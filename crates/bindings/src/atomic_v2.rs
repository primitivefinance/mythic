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
                    ::std::borrow::ToOwned::to_owned("calculateProfit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("calculateProfit"),
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
                                    name: ::std::borrow::ToOwned::to_owned("xForY"),
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
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
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
                    ::std::borrow::ToOwned::to_owned("derivativeOfProfit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("derivativeOfProfit"),
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
                                    name: ::std::borrow::ToOwned::to_owned("xForY"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("guess"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
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
                    ::std::borrow::ToOwned::to_owned("profitFinder"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("profitFinder"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract ProfitFinder"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
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
                    ::std::borrow::ToOwned::to_owned("searchMaxArbProfit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("searchMaxArbProfit"),
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
                                    name: ::std::borrow::ToOwned::to_owned("best_guess"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("xForY"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("best_amount_in"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("best_profit"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("expectedPrice"),
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
                    ::std::borrow::ToOwned::to_owned("tradePacket"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("tradePacket"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("block_number"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("block_timestamp"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("profit"),
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
                    ::std::borrow::ToOwned::to_owned("try_lower_exchange_price"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "try_lower_exchange_price",
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
                                    name: ::std::borrow::ToOwned::to_owned("input_y_amount"),
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
                    ::std::borrow::ToOwned::to_owned("try_raise_exchange_price"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "try_raise_exchange_price",
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
                                    name: ::std::borrow::ToOwned::to_owned("input_y_amount"),
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
                    ::std::borrow::ToOwned::to_owned("MaximizingProfitTrade"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "MaximizingProfitTrade",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("trade"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("profit"),
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
    const __BYTECODE: &[u8] = b"`\x80`@\x90\x80\x82R4b\0\x01\xB1WP\x80Q`\x01`\x01`@\x1B\x03\x90`\x1Fb\0-p8\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x91\x84\x83\x11\x84\x84\x10\x17b\0\x01\x9BW\x80\x84\x92`\xA0\x94\x88R\x839\x81\x01\x03\x12b\0\x01LWb\0\0W\x81b\0\x01\xFEV[\x91b\0\0f` \x83\x01b\0\x01\xFEV[\x90b\0\0t\x85\x84\x01b\0\x01\xFEV[\x91b\0\0\x91`\x80b\0\0\x89``\x87\x01b\0\x01\xFEV[\x95\x01b\0\x01\xFEV[`\x01a\xFF\xFF\x19`\nT\x16\x17`\nU`\x01\x80`\xA0\x1B\x03\x80\x95\x81\x80\x94\x81`\x01\x80`\xA0\x1B\x03\x19\x9A\x16\x8A`\x02T\x16\x17`\x02U\x16\x88`\x01T\x16\x17`\x01U\x81`\0\x96\x16\x88\x87T\x16\x17\x86U\x16\x86`\x03T\x16\x17`\x03U\x16\x84`\x04T\x16\x17`\x04U\x84Q\x90a\x01\xC1\x80\x83\x01\x91\x83\x83\x10\x90\x83\x11\x17b\0\x018W\x90\x82\x91b\0+\xAF\x839\x03\x90\x82\xF0\x90\x81\x15b\0\x01-WP\x16\x90`\x05T\x16\x17`\x05UQa)\x96\x90\x81b\0\x02\x19\x829\xF3[\x84Q\x90=\x90\x82>=\x90\xFD[cNH{q`\xE0\x1B\x84R`A`\x04R`$\x84\xFD[\x82QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[bF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01Ra7\xB7`\xF1\x1B`d\x82\x01R`\x84\x90\xFD[Q\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03b\0\x02\x13WV[`\0\x80\xFD\xFE`\x80`@R`\x046\x10a\x0E\x14W`\x005`\xE0\x1C\x80c\x17\xE4r\x82\x14a\x01\xECW\x80c-[l\xB9\x14a\x01\xE7W\x80c6yr:\x14a\x01\xE2W\x80c7\xC6\xA4J\x14a\x01\xDDW\x80c8\x0B\xC6\x80\x14a\x01\xD8W\x80c8\xD5.\x0F\x14a\x01\xD3W\x80c9(\xFF\x97\x14a\x01\xCEW\x80cI\xA7\xA2m\x14a\x01\xC9W\x80cdI\xFCW\x14a\x01\xC4W\x80cgsB\xCE\x14a\x01\xBFW\x80cq\xDA\x13s\x14a\x01\xBAW\x80cr\xB9\x82F\x14a\x01\xB5W\x80c\x85\xB3\x19\xFF\x14a\x01\xB0W\x80c\x91/\x82\x83\x14a\x01\xABW\x80c\x93e \xC3\x14a\x01\xA6W\x80c\x96\xFB\xEE\x1D\x14a\x01\xA1W\x80c\x98\xD8\x83M\x14a\x01\x9CW\x80c\x99\x9B\x93\xAF\x14a\x01\x97W\x80c\x9F'\xEFO\x14a\x01\x92W\x80c\xA1S\x87\x89\x14a\x01\x8DW\x80c\xAE\x97h\xA8\x14a\x01\x88W\x80c\xBD%-\x06\x14a\x01\x83W\x80c\xD0\xB7\x1B\x1E\x14a\x01~W\x80c\xD2L\xE6\xE5\x14a\x01yW\x80c\xD2\xF7&Z\x14a\x01tW\x80c\xE5$\xF8I\x14a\x01oW\x80c\xF3\xC9s\xCF\x14a\x01jW\x80c\xF9\0^\xB5\x14a\x01eW\x80c\xFA.Y\x94\x14a\x01`Wc\xFA=\xFA\xBB\x03a\x0E\x14Wa\r\xFBV[a\r\xDDV[a\x0C\x9CV[a\x0CyV[a\x0C9V[a\x0C\x10V[a\x0B\xB3V[a\x0BlV[a\x0B\x1FV[a\n\xFEV[a\n\xB9V[a\n\x90V[a\ngV[a\n>V[a\x08\xE5V[a\x08\xC7V[a\x08\xA2V[a\x08\x84V[a\x08fV[a\x07AV[a\x07#V[a\x06\xFDV[a\x06\xD4V[a\x06(V[a\x05bV[a\x04\xA6V[a\x04_V[a\x04GV[a\x04'V[a\x02\x91V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01Ra7\xB7`\xF1\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[4a\x04\x10W`@6`\x03\x19\x01\x12a\x04\x0BWa\x02\xC2`$5a\x02\xB1\x81a\x16\"V[`\nT`\x08\x1C`\xFF\x16`\x045a\x1B\x15V[`\nT`\xFF\x16`\x03Ta\x02\xEB\x90a\x02\xDF\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x80;\x15a\x03\xE7W`@Qcp\xA0\x821`\xE0\x1B\x80\x82R0`\x04\x83\x01R` \x93\x90\x92\x84\x90\x83\x90`$\x90\x82\x90Z\xFA\x90\x81\x15a\x03\xE2Wa\x03/\x92`\0\x92a\x03\xECW[Pa\x18\x1DV[`\x04Ta\x03F\x90a\x02\xDF\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x90\x81;\x15a\x03\xE7W`@Q\x90\x81R0`\x04\x82\x01R\x90\x82\x90\x82\x90`$\x90\x82\x90Z\xFA\x90\x81\x15a\x03\xE2Wa\x03\x80\x92`\0\x92a\x03\xB5W[PP`\x08UV[a\x03\xB1a\x03\x92`\x08T`\x07T\x90a\x115V[`@QcB\xD5\xD4o`\xE1\x1B\x81R`\x04\x81\x01\x91\x90\x91R\x90\x81\x90`$\x82\x01\x90V[\x03\x90\xFD[a\x03\xD4\x92P\x80=\x10a\x03\xDBW[a\x03\xCC\x81\x83a\x0E\xC2V[\x81\x01\x90a\x13\x04V[8\x80a\x03yV[P=a\x03\xC2V[a\x13\x13V[a\x12\xB1V[a\x04\x04\x91\x92P\x85=\x87\x11a\x03\xDBWa\x03\xCC\x81\x83a\x0E\xC2V[\x908a\x03)V[a\x02AV[a\x01\xF1V[` \x90`\x03\x19\x01\x12a\x04\x0BW`\x045\x90V[4a\x04\x10W` a\x04?a\x04:6a\x04\x15V[a&\xC5V[`@Q\x90\x81R\xF3[4a\x04\x10W` a\x04?a\x04Z6a\x04\x15V[a$\xDDV[4a\x04\x10W`@6`\x03\x19\x01\x12a\x04\x0BW`\x045`$5\x90g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17\x82\x15\x15\x16\x15a\x04\xA1W` \x91`@Q\x91\x04\x81R\xF3[`\0\x80\xFD[4a\x04\x10W`@6`\x03\x19\x01\x12a\x04\x0BWa\x04\xD4`$5a\x04\xC6\x81a\x16\"V[`\nT`\x08\x1C`\xFF\x16a\x18\x1DV[`\nT`\xFF\x16`\x03Ta\x04\xF1\x90a\x02\xDF\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x80;\x15a\x03\xE7W`@Qcp\xA0\x821`\xE0\x1B\x80\x82R0`\x04\x83\x01R` \x93\x90\x92\x84\x90\x83\x90`$\x90\x82\x90Z\xFA\x90\x81\x15a\x03\xE2Wa\x03/\x92`\0\x92a\x058W[P`\x045a\x1B\x15V[a\x05P\x91\x92P\x85=\x87\x11a\x03\xDBWa\x03\xCC\x81\x83a\x0E\xC2V[\x908a\x05/V[`\0\x91\x03\x12a\x04\x0BWV[4a\x04\x10W`\x006`\x03\x19\x01\x12a\x04\x0BW`\x03T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[\x80\x15\x15\x03a\x04\xA1WV[``\x90`\x03\x19\x01\x12a\x04\x0BW`\x045\x90`$5a\x05\xB1\x81a\x05\x8BV[\x90`D5\x90V[`\0[\x83\x81\x10a\x05\xCBWPP`\0\x91\x01RV[\x81\x81\x01Q\x83\x82\x01R` \x01a\x05\xBBV[\x90` \x91a\x05\xF4\x81Q\x80\x92\x81\x85R\x85\x80\x86\x01\x91\x01a\x05\xB8V[`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[\x90\x92`\x80\x92a\x06%\x95\x94\x15\x15\x83R` \x83\x01R`@\x82\x01R\x81``\x82\x01R\x01\x90a\x05\xDBV[\x90V[4a\x04\x10Wa\x0666a\x05\x95V[`\x02T`\x01`\x01`\xA0\x1B\x03\x16\x92\x90\x83;\x15a\x03\xE7W`@Qc9(\xFF\x97`\xE0\x1B\x81R`\x04\x81\x01\x92\x90\x92R\x91\x15\x15`$\x82\x01R`D\x81\x01\x91\x90\x91R\x90`\0\x90\x81\x90\x83\x90`d\x90\x82\x90Z\xFA\x80\x15a\x03\xE2W\x81\x80\x93\x81\x80\x93a\x06\xA7W[PP\x90a\x06\xA3\x91`@Q\x94\x85\x94\x85a\x06\0V[\x03\x90\xF3[\x91P\x91Pa\x06\xA3\x93Pa\x06\xCC\x92P=\x80\x91\x83>a\x06\xC4\x81\x83a\x0E\xC2V[\x81\x01\x90a\x13\x1FV[\x91\x938a\x06\x90V[4a\x04\x10W`\x006`\x03\x19\x01\x12a\x04\x0BW`\x02T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x04\x10W`\x006`\x03\x19\x01\x12a\x04\x0BW` `\xFF`\nT`\x08\x1C\x16`@Q\x90\x15\x15\x81R\xF3[4a\x04\x10W` 6`\x03\x19\x01\x12a\x04\x0BW` a\x04?`\x045a(fV[4a\x04\x10W``6`\x03\x19\x01\x12a\x04\x0BW`\x045`$5`D5a\x07d\x81a\x05\x8BV[`\0\x80\x93a\x07za\x07t\x85a\x10[V[\x94a\x10\xA4V[`@\x94a\x07\xA6\x86Qa\x07\x8B\x81a\x0E\xA6V[`\t\x81Rh4\xBA2\xB90\xBA4\xB7\xB7`\xB9\x1B` \x82\x01Ra\x0F\x84V[\x81\x81\x10\x80a\x08]W[\x15a\x08#Wa\x07\xD0a\x07\xCAa\x07\xC4\x83\x85a\x0F\x16V[`\x01\x1C\x90V[\x82a\x0F#V[\x90a\x07\xE2\x82a\x07\xDDa\x0F0V[a\x0F\xCEV[\x84a\x07\xEE\x83\x88\x87a\x11\xCBV[a\x07\xFF\x81a\x07\xFAa\x0FOV[a\x10,V[\x13\x15a\x08\x16WPa\x08\x10\x90\x96a\x0FuV[\x95a\x07\xA6V[\x96\x91Pa\x08\x10\x90\x91a\x0FuV[a\x06\xA3\x93Pa\x07\xC4a\x08A\x93\x92a\x089\x92a\x0F#V[\x93\x84\x91a\x14\xE3V[\x93Q\x92\x83R` \x83\x01R`@\x82\x01\x92\x90\x92R\x90\x81\x90``\x82\x01\x90V[P\x85\x87\x10a\x07\xAFV[4a\x04\x10W`\x006`\x03\x19\x01\x12a\x04\x0BW` `\x08T`@Q\x90\x81R\xF3[4a\x04\x10W`\x006`\x03\x19\x01\x12a\x04\x0BW` `\tT`@Q\x90\x81R\xF3[4a\x04\x10W`@a\x08\xBBa\x08\xB56a\x05\x95V[\x91a\x14\xE3V[\x82Q\x91\x82R` \x82\x01R\xF3[4a\x04\x10W`\x006`\x03\x19\x01\x12a\x04\x0BW` `\x06T`@Q\x90\x81R\xF3[4a\x04\x10W`@6`\x03\x19\x01\x12a\x04\x0BW`\x02T`$5\x90`\x045\x90a\t\x15\x90a\x02\xDF\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x80;\x15a\x03\xE7W`@Qc\x03\xB4\xD1\x03`\xE4\x1B\x81R`\x04\x81\x01\x83\x90R` \x93\x90\x91\x84\x90\x83\x90`$\x90\x82\x90Z\xFA\x80\x15a\x03\xE2Wa\t\x8E\x7F\xD15<h\xE7\x9E\xF7\r\xE8N\xE9\r/\xAC\xF8E\xEC$\x89Q\x16\xD4\xA05\x05\xAAAxZ\xF7\x1FZ\x91a\t\x9A\x94`\0\x91a\n!W[P`@\x80Q\x91\x82RB` \x83\x01R\x90\x91\x82\x91\x82\x01\x90V[\x03\x90\xA1a\x04\xC6\x81a\x16\"V[`\nT`\xFF\x16`\x03Ta\t\xB7\x90a\x02\xDF\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x91\x82;\x15a\x03\xE7W`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x92\x84\x90\x84\x90`$\x90\x82\x90Z\xFA\x92\x83\x15a\x03\xE2Wa\t\xF8\x94`\0\x94a\n\x02W[PPa\x1B\x15V[a\n\0a\x1DrV[\0[a\n\x19\x92\x94P\x80=\x10a\x03\xDBWa\x03\xCC\x81\x83a\x0E\xC2V[\x918\x80a\t\xF1V[a\n8\x91P\x87=\x89\x11a\x03\xDBWa\x03\xCC\x81\x83a\x0E\xC2V[8a\twV[4a\x04\x10W`\x006`\x03\x19\x01\x12a\x04\x0BW`\x05T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x04\x10W`\x006`\x03\x19\x01\x12a\x04\x0BW`\x04T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x04\x10W`\x006`\x03\x19\x01\x12a\x04\x0BW`\0T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x04\x10W`\x006`\x03\x19\x01\x12a\x04\x0BW`\xA0`\x0BT`\x0CT`\xFF`\rT\x16`\x0ET\x90`\x0FT\x92`@Q\x94\x85R` \x85\x01R\x15\x15`@\x84\x01R``\x83\x01R`\x80\x82\x01R\xF3[4a\x04\x10W`@6`\x03\x19\x01\x12a\x04\x0BW` a\x04?`$5`\x045a\x10\xCDV[4a\x04\x10W`@6`\x03\x19\x01\x12a\x04\x0BW`\x045`$5g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x81\x02\x92\x81\x84\x04\x14\x90\x15\x17\x81\x15\x15\x16\x15a\x04\xA1W` \x91`\x01`@Q\x92`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x81R\xF3[4a\x04\x10W` g\x1B\xC1mgN\xC8\0\0a\x0B\xAAa\x0B\x9Aa\x0B\xA5g\x13\xA0K\xBD\xFD\xC9\xBE\x88a\x0B\x9Fa\x0B\x9A6a\x04\x15V[a\x11tV[\x05a\x1F\x0BV[a FV[\x05`@Q\x90\x81R\xF3[4a\x04\x10Wa\x0B\xC16a\x04\x15V[a\x0B\xCA\x81a\x1F\x0BV[\x80\x82\x02\x91`\0\x82\x12`\x01`\xFF\x1B\x82\x14\x16a\x0C\x0BW\x81\x83\x05\x14\x90\x15\x17\x15a\x0C\x0BWg\"\xC9U\"\x95T\xC1\xB6a\x0B\xAAa\x0B\x9Ag\x1B\xC1mgN\xC8\0\0` \x94\x05a!\x81V[a\x0F\0V[4a\x04\x10W`\x006`\x03\x19\x01\x12a\x04\x0BW`\x01T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x04\x10W`@6`\x03\x19\x01\x12a\x04\x0BW`\x045`$5\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x04\xA1W` \x90g\r\xE0\xB6\xB3\xA7d\0\0`@Q\x91\x04\x81R\xF3[4a\x04\x10W`\x006`\x03\x19\x01\x12a\x04\x0BW` `\xFF`\nT\x16`@Q\x90\x15\x15\x81R\xF3[4a\x04\x10W`@6`\x03\x19\x01\x12a\x04\x0BW`\x02T`\x045\x90`$5\x90a\x0C\xCC\x90a\x02\xDF\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x90\x81;\x15a\x03\xE7W`@Qc\x03\xB4\xD1\x03`\xE4\x1B\x81R`\x04\x81\x01\x84\x90R` \x93\x90\x92\x84\x90\x84\x90`$\x90\x82\x90Z\xFA\x80\x15a\x03\xE2Wa\rE\x7F\xD15<h\xE7\x9E\xF7\r\xE8N\xE9\r/\xAC\xF8E\xEC$\x89Q\x16\xD4\xA05\x05\xAAAxZ\xF7\x1FZ\x91a\r`\x95`\0\x91a\n!WP`@\x80Q\x91\x82RB` \x83\x01R\x90\x91\x82\x91\x82\x01\x90V[\x03\x90\xA1a\rQ\x82a\x16\"V[`\nT`\x08\x1C`\xFF\x16\x90a\x1B\x15V[`\nT`\xFF\x16`\x03Ta\r}\x90a\x02\xDF\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x90\x81;\x15a\x03\xE7W`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x91\x83\x90\x83\x90`$\x90\x82\x90Z\xFA\x91\x82\x15a\x03\xE2Wa\t\xF8\x93`\0\x93a\r\xBEW[PPa\x18\x1DV[a\r\xD5\x92\x93P\x80=\x10a\x03\xDBWa\x03\xCC\x81\x83a\x0E\xC2V[\x908\x80a\r\xB7V[4a\x04\x10W`\x006`\x03\x19\x01\x12a\x04\x0BW` `\x07T`@Q\x90\x81R\xF3[4a\x04\x10W` a\x04?a\x0E\x0E6a\x05\x95V[\x91a\x11\xCBV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01Rtnor receive functions`X\x1B`d\x82\x01R`\x84\x90\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x0E\xA1W`@RV[a\x0EwV[`@\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x0E\xA1W`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x0E\xA1W`@RV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x0E\xA1W`\x1F\x01`\x1F\x19\x16` \x01\x90V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x91\x90\x82\x03\x91\x82\x11a\x0C\x0BWV[\x91\x90\x82\x01\x80\x92\x11a\x0C\x0BWV[`@Q\x90a\x0F=\x82a\x0E\xA6V[`\x03\x82Rb\x1BZY`\xEA\x1B` \x83\x01RV[`@Q\x90a\x0F\\\x82a\x0E\xA6V[`\n\x82Riderivative`\xB0\x1B` \x83\x01RV[`\0\x19\x81\x14a\x0C\x0BW`\x01\x01\x90V[a\x0F\xB2a\x0F\xC7a\x0F\xCC\x92`@Q\x92\x83\x91c-\x83\x9C\xB3`\xE2\x1B` \x84\x01R`@`$\x84\x01R`d\x83\x01\x90a\x05\xDBV[`\0`D\x83\x01R\x03`\x1F\x19\x81\x01\x83R\x82a\x0E\xC2V[a\x10\x11V[V[a\x0F\xC7a\x0F\xFD\x91a\x0F\xCC\x93`@Q\x93\x84\x92c-\x83\x9C\xB3`\xE2\x1B` \x85\x01R`@`$\x85\x01R`d\x84\x01\x90a\x05\xDBV[\x90`D\x83\x01R\x03`\x1F\x19\x81\x01\x83R\x82a\x0E\xC2V[`\0\x80\x91` \x81Q\x91\x01jconsole.logZ\xFAPV[a\x0F\xC7a\x0F\xFD\x91a\x0F\xCC\x93`@Q\x93\x84\x92c\x1ES\x13G`\xE1\x1B` \x85\x01R`@`$\x85\x01R`d\x84\x01\x90a\x05\xDBV[`\x1E\x81\x02\x90`\x1E\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x04\xA1Wa\x03\xE8\x90\x04\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x04\xA1Wg\x1B\xC1mgN\xC8\0\0\x90\x04\x90V[a\x03\xE8\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x04\xA1W`\x01`d`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x04\xA1W`\x01g\r\xE0\xB6\xB3\xA7d\0\0`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[\x90g\x11\x90\0\xAB\x10\x0F\xFB\xFF\x19\x82\x01\x91\x82\x13`\x01\x16a\x0C\x0BWV[\x90g\x1B\xC1mgN\xC8\0\0`\0\x83\x82\x03\x93\x12\x81\x84\x12\x81\x16\x91\x84\x13\x90\x15\x16\x17a\x0C\x0BWV[\x81\x81\x03\x92\x91`\0\x13\x80\x15\x82\x85\x13\x16\x91\x84\x12\x16\x17a\x0C\x0BWV[`@Q\x90a\x11[\x82a\x0E\xA6V[`\n\x82Ridifference`\xB0\x1B` \x83\x01RV[\x90g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x81\x02\x92\x81\x84\x05\x14\x90\x15\x17\x15a\x0C\x0BWV[`\0\x81\x90\x03\x91\x90`\x01`\xFF\x1B\x81\x14`\x01\x16a\x0C\x0BW`\0\x19\x83\x05\x03a\x0C\x0BWV[\x90c;\x9A\xCA\0\x91\x82\x81\x02\x92\x81\x84\x05\x14\x90\x15\x17\x15a\x0C\x0BWV[a'\x0F\x19\x83\x01\x91\x83\x83\x11a\x0C\x0BWa'\x10\x84\x01\x93\x84\x81\x11a\x0C\x0BWa\x120a\x12\xA9\x94a\x12\xA3\x93a\x12)a\x06%\x98a\x12\"a\x12\x9D\x96`@Qa\x12\x0B\x81a\x0E\xA6V[`\x05\x81Rdguess`\xD8\x1B` \x82\x01Ra\x0F\xCEV[\x82\x88a\x14\xE3V[P\x95a\x14\xE3V[P\x92a\x12\\\x81`@Qa\x12B\x81a\x0E\xA6V[`\x08\x81Rg\x07\x07&\xF6f\x97EW`\xC4\x1B` \x82\x01Ra\x10,V[a\x12\x88\x84`@Qa\x12l\x81a\x0E\xA6V[`\n\x81Ri897\xB34\xBA\"7\xBB\xB7`\xB1\x1B` \x82\x01Ra\x10,V[a\x0B\x9Aa\x12\x95\x85\x83a\x115V[a\x07\xFAa\x11NV[\x91a\x11tV[\x90a\x115V[aN \x90\x05\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01Rd code`\xD8\x1B`d\x82\x01R`\x84\x90\xFD[\x90\x81` \x91\x03\x12a\x04\x0BWQ\x90V[`@Q=`\0\x82>=\x90\xFD[`\x80\x81\x83\x03\x12a\x04\x0BW\x80Qa\x134\x81a\x05\x8BV[\x92` \x92\x83\x83\x01Q\x93`@\x84\x01Q\x93``\x81\x01Q\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x14GW\x01\x82`\x1F\x82\x01\x12\x15a\x13\xEEW\x80Q\x91a\x13p\x83a\x0E\xE4V[\x93a\x13~`@Q\x95\x86a\x0E\xC2V[\x83\x85R\x81\x84\x84\x01\x01\x11a\x13\x9AWa\x06%\x92\x91\x84\x82\x01\x91\x01a\x05\xB8V[`\x84\x90`@Q\x90bF\x1B\xCD`\xE5\x1B\x82R`\x04\x82\x01R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x83\x90R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x84\x90R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[\x15a\x14\x9EWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FInvalid swap simulation\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x90\xFD[`\0\x80T\x93\x94\x92\x93\x90\x91\x90a\x15\x02\x90a\x02\xDF\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x80;\x15a\x03\xE7W` \x83\x91`\x04`@Q\x80\x94\x81\x93cP\x1A\xD8\xFF`\xE1\x1B\x83RZ\xF1\x90\x81\x15a\x03\xE2W\x83\x91a\x16\x04W[P`\x02Ta\x15H\x90a\x02\xDF\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x80;\x15a\x03\xE7W`@Qc9(\xFF\x97`\xE0\x1B\x81R`\x04\x81\x01\x93\x90\x93R\x85\x15\x15`$\x84\x01R`D\x83\x01\x87\x90R\x83\x90\x83\x90`d\x90\x82\x90Z\xFA\x94\x85\x15a\x03\xE2W\x83\x96\x84\x93\x85\x97a\x15\xD6W[Pa\x15\x9Fa\x15\xBD\x96\x97\x98a\x14\x97V[\x82\x82\x15a\x15\xCFWa\x15\xAF\x91a\x10\xCDV[\x93[P\x15a\x15\xC1WPa\x115V[\x91\x90V[a\x15\xCA\x91a\x10\xCDV[a\x115V[P\x93a\x15\xB1V[a\x15\x9F\x98Pa\x15\xBD\x96\x97Pa\x15\xF6\x91\x94P=\x80\x87\x83>a\x06\xC4\x81\x83a\x0E\xC2V[P\x94\x91\x98\x90\x94\x97\x96Pa\x15\x90V[a\x16\x1C\x91P` =\x81\x11a\x03\xDBWa\x03\xCC\x81\x83a\x0E\xC2V[8a\x150V[`\x04\x80Ta\x16:\x90a\x02\xDF\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x90\x81;\x15a\x03\xE7W`@\x80Qcp\xA0\x821`\xE0\x1B\x80\x82R3\x82\x85\x01\x90\x81R` \x96\x91\x95\x93\x94\x93\x91\x92\x90\x87\x90\x82\x90\x81\x90\x83\x01\x03\x81\x85Z\xFA\x90\x81\x15a\x03\xE2W`\0\x91a\x18\0W[P\x82\x81\x10a\x17\xDDWP\x80;\x15a\x03\xE7W\x83Qcn\xB1v\x9F`\xE1\x1B\x81R3\x84\x82\x01\x90\x81R0` \x82\x01R\x87\x90\x82\x90\x81\x90`@\x01\x03\x81\x85Z\xFA\x90\x81\x15a\x03\xE2W`\0\x91a\x17\xC0W[P\x82\x81\x10a\x17\x9DWP\x80;\x15a\x03\xE7W\x83Qc#\xB8r\xDD`\xE0\x1B\x81R3\x84\x82\x01\x90\x81R0` \x82\x01R`@\x81\x01\x93\x90\x93R\x91`\0\x91\x83\x91\x82\x90\x84\x90\x82\x90``\x01\x03\x92Z\xF1\x80\x15a\x03\xE2Wa\x17\x84W[P\x80Ta\x17+\x90a\x02\xDF\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x80;\x15a\x03\xE7W\x91Q\x92\x83R0\x90\x83\x01\x90\x81R\x83\x91\x83\x91\x82\x90\x81\x90` \x01\x03\x91Z\xFA\x90\x81\x15a\x03\xE2Wa\x0F\xCC\x92`\0\x92a\x17gW[PP`\x07UV[a\x17}\x92P\x80=\x10a\x03\xDBWa\x03\xCC\x81\x83a\x0E\xC2V[8\x80a\x17`V[\x80a\x17\x91a\x17\x97\x92a\x0E\x8DV[\x80a\x05WV[8a\x17\x14V[\x84Qc\xDAV\xD3\xC5`\xE0\x1B\x81R\x80\x85\x01\x91\x82R` \x82\x01\x84\x90R\x90\x81\x90`@\x01\x03\x90\xFD[a\x17\xD7\x91P\x87=\x89\x11a\x03\xDBWa\x03\xCC\x81\x83a\x0E\xC2V[8a\x16\xC5V[\x84Qc\n\xBEZ\x89`\xE0\x1B\x81R\x80\x85\x01\x91\x82R` \x82\x01\x84\x90R\x90\x81\x90`@\x01\x03\x90\xFD[a\x18\x17\x91P\x87=\x89\x11a\x03\xDBWa\x03\xCC\x81\x83a\x0E\xC2V[8a\x16\x7FV[\x15a\x19\x18W`\x03Ta\x189\x90a\x02\xDF\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\0\x80T\x90\x91\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x80;\x15a\x03\xE7W`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16`\x04\x83\x01R`$\x82\x01\x84\x90R\x82\x90\x82\x90`D\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x03\xE2Wa\x19\x05W[P\x80Ta\x18\xA5\x90a\x02\xDF\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x03T`\x01`\x01`\xA0\x1B\x03\x16\x92\x90\x80;\x15a\x03\xE7W`@Qc\xD0\x04\xF0\xF7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x94\x90\x94\x16`\x04\x85\x01R`$\x84\x01\x91\x90\x91R\x82\x90`D\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x03\xE2Wa\x18\xF8WPV[\x80a\x17\x91a\x0F\xCC\x92a\x0E\x8DV[\x80a\x17\x91a\x19\x12\x92a\x0E\x8DV[8a\x18\x8EV[`\x04Ta\x19/\x90a\x02\xDF\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\0\x80T\x90\x92\x91\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x80;\x15a\x03\xE7W`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16`\x04\x83\x01R`$\x82\x01\x83\x90R\x83\x90\x82\x90`D\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x03\xE2Wa\x1AvW[P\x81Ta\x19\x9C\x90a\x02\xDF\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x04T\x90\x91\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x82;\x15a\x03\xE7W`@Qc\xD0\x04\xF0\xF7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16`\x04\x83\x01R`$\x82\x01R\x90\x82\x90\x82\x90`D\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x03\xE2Wa\x1AcW[P`\x03Ta\x1A\x07\x90a\x02\xDF\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x80;\x15a\x03\xE7W`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90` \x90\x82\x90`$\x90\x82\x90Z\xFA\x90\x81\x15a\x03\xE2Wa\x0F\xCC\x92\x91a\x1AEW[P`\x06UV[a\x1A]\x91P` =\x81\x11a\x03\xDBWa\x03\xCC\x81\x83a\x0E\xC2V[8a\x1A?V[\x80a\x17\x91a\x1Ap\x92a\x0E\x8DV[8a\x19\xEFV[\x80a\x17\x91a\x1A\x83\x92a\x0E\x8DV[8a\x19\x85V[`@\x90a\x06%\x93\x92\x81R\x81` \x82\x01R\x01\x90a\x05\xDBV[=\x15a\x1A\xCBW=\x90a\x1A\xB1\x82a\x0E\xE4V[\x91a\x1A\xBF`@Q\x93\x84a\x0E\xC2V[\x82R=`\0` \x84\x01>V[``\x90V[\x90`\x80a\x06%\x92`@\x81R`\x19`@\x82\x01R\x7FDEX swap failed with data\0\0\0\0\0\0\0``\x82\x01R\x81` \x82\x01R\x01\x90a\x05\xDBV[\x81\x15a\x1C\xF0W`\x03Ta\x1B2\x90a\x02\xDF\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x01T`\x01`\x01`\xA0\x1B\x03\x16\x90\x80;\x15a\x03\xE7W`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16`\x04\x83\x01R`$\x82\x01\x85\x90R`\0\x90\x82\x90`D\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x03\xE2Wa\x1C\xDDW[P[`\x02Ta\x1B\x9D\x90a\x02\xDF\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x90\x81;\x15a\x03\xE7W`@Qc9(\xFF\x97`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R\x83\x15\x15`$\x82\x01R`D\x81\x01\x94\x90\x94R`\0\x93\x91\x84\x90\x83\x90`d\x90\x82\x90Z\xFA\x91\x82\x15a\x03\xE2W\x84\x85\x91\x86\x90\x87\x95a\x1C\xBBW[P\x81\x15a\x1C\x9DWPP`\x01Ta\x1C\x0C\x91Pa\x02\xDF\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x91\x82;\x15a\x03\xE7Wa\x1C7\x92\x85\x92\x83`@Q\x80\x96\x81\x95\x82\x94c\xBD\x06%\xAB`\xE0\x1B\x84R`\x04\x84\x01a\x1A\x89V[\x03\x92Z\xF1\x90\x81a\x1C\x8AW[Pa\x1CkWa\x03\xB1a\x1CRa\x1A\xA0V[`@Qcg\xA1k\x8D`\xE1\x1B\x81R\x91\x82\x91`\x04\x83\x01a\x1A\xD0V[\x15a\x1CsWPV[`\x03Ta\x1A\x07\x90a\x02\xDF\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x80a\x17\x91a\x1C\x97\x92a\x0E\x8DV[8a\x1CBV[\x84a\x03\xB1\x91`@Q\x94\x85\x94c\x03\x14\xE6#`\xE3\x1B\x86R`\x04\x86\x01a\x06\0V[\x92PPPa\x1C\xD4\x91\x92P=\x80\x86\x83>a\x06\xC4\x81\x83a\x0E\xC2V[\x93\x92\x908a\x1B\xEBV[\x80a\x17\x91a\x1C\xEA\x92a\x0E\x8DV[8a\x1B\x84V[`\x04Ta\x1D\x07\x90a\x02\xDF\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x01T`\x01`\x01`\xA0\x1B\x03\x16\x90\x80;\x15a\x03\xE7W`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16`\x04\x83\x01R`$\x82\x01\x85\x90R`\0\x90\x82\x90`D\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x03\xE2Wa\x1D_W[Pa\x1B\x86V[\x80a\x17\x91a\x1Dl\x92a\x0E\x8DV[8a\x1DYV[`\x04Ta\x1D\x89\x90a\x02\xDF\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x80;\x15a\x03\xE7W`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90` \x90\x82\x90`$\x90\x82\x90Z\xFA\x80\x15a\x03\xE2Wa\x1D\xC8\x91`\0\x91a\x1E\xEDW[P`\x08UV[`\x08T`\x07T\x90\x81\x81\x10a\x1E\x83Wa\x1E\x04\x7F5}\x90_\x181 \x97\x97\xDFMU\xD7\x9C\\[\xF1\xD9\xF71\x1C\x97j\xFD\x05\xE1=\x88\x1E\xAB\x9B\xC8\x92a\x1E(\x92a\x0F\x16V[a\x1E\x18a\x1E\x13\x82`\tTa\x0F#V[`\tUV[`@Q\x90\x81R\x90\x81\x90` \x82\x01\x90V[\x03\x90\xA1`\x04Ta\x1EB\x90a\x02\xDF\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x08T\x90\x80;\x15a\x03\xE7W`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x92\x90\x92R`\0\x90\x82\x90`D\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x03\xE2Wa\x18\xF8WPV[\x90\x7F\xB6[.\x08]}\x04\x0C1?}N\x1A\xC9\x0FY7\x02o\xEEI~\x0E$\xA7\xEF\xF1jU\xE1\xC5\xEAa\x1E\xB2a\x1E\x18\x84\x84a\x0F\x16V[\x03\x90\xA1a\x03\xB1a\x1E\xC2\x83\x83a\x115V[\x19`@Q\x93\x84\x93c\xB1n7\x83`\xE0\x1B\x85R`\x04\x85\x01`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[a\x1F\x05\x91P` =\x81\x11a\x03\xDBWa\x03\xCC\x81\x83a\x0E\xC2V[8a\x1D\xC2V[`\x01`\xFF\x1B\x81\x14a\x0C\x0BW`\0\x03\x90V[\x90\x81g\r\xE0\xB6\xB3\xA7d\0\0\x01\x91\x82\x12`\x01\x16a\x0C\x0BWV[\x90\x81g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19\x01\x91\x82\x12\x15`\x01\x16a\x0C\x0BWV[\x90\x81g\x14\xA8EL\x19\xE1\xAC\0\x01\x91\x82\x12`\x01\x16a\x0C\x0BWV[\x90\x81g\x0F\xC1\x0E\x01W\x82w\xFF\x19\x01\x91\x82\x12\x15`\x01\x16a\x0C\x0BWV[\x90\x81g\x03\xDE\xBD\x08;\x8C|\0\x01\x91\x82\x12`\x01\x16a\x0C\x0BWV[\x90\x81g\x02\x95\xD4\0\xEA2W\xFF\x19\x01\x91\x82\x12\x15`\x01\x16a\x0C\x0BWV[\x90\x81g\x01W\xD8\xB2\xEC\xC7\x08\0\x01\x91\x82\x12`\x01\x16a\x0C\x0BWV[\x90\x81g\x051\n\xA7\xD5!0\0\x01\x91\x82\x12`\x01\x16a\x0C\x0BWV[\x90\x81g\r\xE0\xCC=\x15a\0\0\x01\x91\x82\x12`\x01\x16a\x0C\x0BWV[\x90\x81g \x05\xFEO&\x8E\xA0\0\x01\x91\x82\x12`\x01\x16a\x0C\x0BWV[\x90\x81g\r\xC5R\x7Fd, \0\x01\x91\x82\x12`\x01\x16a\x0C\x0BWV[\x91\x90\x91`\0\x83\x82\x01\x93\x84\x12\x91\x12\x90\x80\x15\x82\x16\x91\x15\x16\x17a\x0C\x0BWV[\x80\x15a!tWgV\x98\xEE\xF0fp\0\0\x81\x12\x15a!nWgV\x98\xEE\xF0fo\xFF\xFF\x19\x81\x13\x15a!aWa!Oa y\x82a#\x05V[a!\ra!Ja \x98a \x93a \x8E\x85a\x10yV[a\x1F\x1CV[a#\xF5V[\x92a!Ea!@a!;a!4a!.a!)a!#a!\x1Ea!\x18a!\x13\x8Da!\ra!\x08a!\x02a \xFDa \xF7a \xF2a \xECa \xE7a \xE1a \xDC\x8Aa#2V[a\x1F4V[\x89a#\xD4V[a\x1FNV[\x87a#\xD4V[a\x1FfV[\x85a#\xD4V[a\x1F\x80V[\x83a#\xD4V[a\x1F\x98V[\x90a#\xD4V[a\x1F\xB2V[\x8Ca#\xD4V[a\x1F\xCAV[\x8Aa#\xD4V[a\x1F\xE2V[\x88a#\xD4V[\x93\x80a#\xD4V[a\x11\x91V[a\x10\xF9V[a *V[a!\x81V[\x90`\0\x13\x15a\x06%Wa\x06%\x90a\x11\x12V[Pg\x1B\xC1mgN\xC8\0\0\x90V[P`\0\x90V[Pg\r\xE0\xB6\xB3\xA7d\0\0\x90V[h\x02H\xCE6\xA7\x0C\xB2k>\x19\x81\x13\x15a!nWh\x07U\xBFy\x8BJ\x1B\xF1\xE5\x81\x12\x15a\"\xD1We\x03x-\xAC\xE9\xD9\x90`N\x1B\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x91``\x90`\x01`_\x1B\x84\x82\x84\x1B\x05\x01\x82\x1D\x93\x84\x02\x90\x03\x80l\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x81\x01\x02\x82\x1D\x90n\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dPn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x82n\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x81m\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x81m\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x81m\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x81l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x02\x8D\x1D\x01\x02\x8B\x1D\x01\x02\x89\x1D\x01\x02\x87\x1D\x01\x02\x85\x1D\x01\x93m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x93m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEA\x81\x01\x90\x84m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x91\x01\x01\x02\x90\x1D\x01\x02\x01\x05\x02\x90`\xC3\x03\x1C\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x90\xFD[`\x01`\xFF\x1B\x81\x14a# W`\0\x81\x12\x15a\x06%W\x19`\x01\x01\x90V[`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x90\xFD[g\x02_\x0F\xE1\x05\xA3\x14\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x04\xA1Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\t\xD0(\xCCo _\xFF\x19\x81\x81\x02\x91`\x01\x91\x83\x05\x14\x16\x15a\x04\xA1Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x90\x80\x82\x02\x91\x82\x05\x14`\x01\x16\x15a\x04\xA1Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\x1B\xC1mgN\xC7\xFF\xFF\x19\x81\x81\x02\x91`\x01\x91\x83\x05\x14\x16\x15a\x04\xA1Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x04\xA1Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[`\x01\x81\x15\x15\x16\x15a\x04\xA1Wn\xC0\x97\xCE{\xC9\x07\x15\xB3K\x9F\x10\0\0\0\0\x05\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17\x82\x15\x15\x16\x15a\x04\xA1W\x05\x90V[g\x13\xA0K\xBD\xFD\xC9\xBE\x88\x90\x80\x82\x02\x91\x82\x05\x14`\x01\x16\x15a\x04\xA1Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x04\xA1Wg\x1B\xC1mgN\xC8\0\0\x90\x05\x90V[g\x03\xC1f\\z\xAB \0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x04\xA1Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[f\x9F2u$b\xA0\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x04\xA1Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\x06\xF0[Y\xD3\xB2\0\0\x81\x14a!nWg\r\xE0\xB6\xB3\xA7d\0\0\x80\x82\x12\x15a&7W\x81\x15a&XW`\x01\x82\x81\x1B\x91`\x02\x93\x83\x05\x84\x03a\x0C\x0BW`\0\x83\x12\x80\x15a&|W[a&jW\x82\x15a&7Wg\x1B\xC1mgN\xC8\0\0\x83\x14a&XW\x82\x12\x91\x82\x15a&IW\x92[a%L\x84a$]V[\x80\x15a&7Wa%\xAEa%}a%xa%sa%na%\xB3\x95\x99\x97\x96\x99a&\xC5V[a#\xACV[a(fV[a\x11\xB2V[a\x15\xCAa%\x91a%\x8C\x83a$\x88V[a\x1F\xFAV[a%\xA8a \x8Ea \xF7a%\xA3\x86a$\xB3V[a \x12V[\x90a$\x14V[a#]V[\x93`\0\x92[\x81\x84\x10a%\xEAWPPPPa\x06%\x91a%\xD7\x91`\0\x14a%\xDCWa$6V[a\x1F\x0BV[a%\xE5\x90a\x1F\x0BV[a$6V[\x90\x91a&-\x86a&'a&\x02\x85a\x15\xCA\x86\x99\x9Ba FV[a%\xA8a&\x1Da&\x18a!Ja%\xD7\x87\x80a#\xD4V[a#\x85V[a\x12\xA3\x83\x86a#\xD4V[\x90a *V[\x95\x01\x92\x91\x90a%\xB8V[`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x90\xFD[a&R\x90a\x11\x12V[\x92a%CV[`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x90\xFD[`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x90\xFD[Pg\x1B\xC1mgN\xC8\0\0\x83\x13a%\x1FV[\x15a&\x94WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x90\xFD[}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x90a&\xF1`\0\x82\x13a&\x8DV[q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06a'\r\x82a)$V[``\x92\x83\x82`\x9F\x03\x01\x1B`\x9F\x1C\x90`_\x19\x01}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x02\x92l\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x82m\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x81m\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x81m\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x81m\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x81m\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x81lFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x02\x8C\x1D\x01\x02\x8A\x1D\x01\x02\x88\x1D\x01\x02\x86\x1D\x01\x02\x84\x1D\x01\x02\x82\x1D\x01\x91x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x91l\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x90\x82m\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x81l\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x81m\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x81m\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x81l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x81\x01\x02\x89\x1D\x01\x02\x87\x1D\x01\x02\x85\x1D\x01\x02\x83\x1D\x01\x02\x90\x1D\x01\x02\x01\x05\x02\x01\x01`\xAE\x1D\x90V[`\xB5\x81`\x01`\x88\x1B\x81\x10\x15a)\rW[\x80i\x01\0\0\0\0\0\0\0\0\0b\x01\0\0\x92\x10\x15a)\0W[e\x01\0\0\0\0\0\x81\x10\x15a(\xF3W[c\x01\0\0\0\x81\x10\x15a(\xE6W[\x01\x02`\x12\x1C`\x01\x90\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x80\x80\x92\x04\x10\x90\x03\x90V[`\x10\x1C\x91`\x08\x1B\x91a(\xAAV[` \x1C\x91`\x10\x1B\x91a(\x9DV[`@\x1C\x91` \x1B\x91a(\x8EV[Ph\xB5\0\0\0\0\0\0\0\0\x90P`\x80\x82\x90\x1Ca(vV[a)/\x81\x15\x15a&\x8DV[\x80o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x07\x1B\x81\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x81\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x81\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x81\x81\x1C`\xFF\x10`\x03\x1B\x17\x81\x81\x1C`\x0F\x10`\x02\x1B\x17\x81\x81\x1C`\x03\x10`\x01\x1B\x17\x90\x81\x1C`\x01\x10\x17\x90V`\x80\x80`@R4a\0+W`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x163\x17\x90U`@Qa\x01H\x90\x81a\0y\x829\xF3[bF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01Ra7\xB7`\xF1\x1B`d\x82\x01R`\x84\x90\xFD\xFE`\x80\x80`@R`\x046\x10\x15a\0rW[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01Rtnor receive functions`X\x1B`d\x82\x01R`\x84\x90\xFD[`\x005`\xE0\x1Cc\x91\xE6\xECB\x03a\0\x0FW4a\0\xFBW`\x006`\x03\x19\x01\x12a\0\xABW`\0T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[bF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01Ra7\xB7`\xF1\x1B`d\x82\x01R`\x84\x90\xFD";
    /// The bytecode of the contract.
    pub static ATOMICV2_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\x0E\x14W`\x005`\xE0\x1C\x80c\x17\xE4r\x82\x14a\x01\xECW\x80c-[l\xB9\x14a\x01\xE7W\x80c6yr:\x14a\x01\xE2W\x80c7\xC6\xA4J\x14a\x01\xDDW\x80c8\x0B\xC6\x80\x14a\x01\xD8W\x80c8\xD5.\x0F\x14a\x01\xD3W\x80c9(\xFF\x97\x14a\x01\xCEW\x80cI\xA7\xA2m\x14a\x01\xC9W\x80cdI\xFCW\x14a\x01\xC4W\x80cgsB\xCE\x14a\x01\xBFW\x80cq\xDA\x13s\x14a\x01\xBAW\x80cr\xB9\x82F\x14a\x01\xB5W\x80c\x85\xB3\x19\xFF\x14a\x01\xB0W\x80c\x91/\x82\x83\x14a\x01\xABW\x80c\x93e \xC3\x14a\x01\xA6W\x80c\x96\xFB\xEE\x1D\x14a\x01\xA1W\x80c\x98\xD8\x83M\x14a\x01\x9CW\x80c\x99\x9B\x93\xAF\x14a\x01\x97W\x80c\x9F'\xEFO\x14a\x01\x92W\x80c\xA1S\x87\x89\x14a\x01\x8DW\x80c\xAE\x97h\xA8\x14a\x01\x88W\x80c\xBD%-\x06\x14a\x01\x83W\x80c\xD0\xB7\x1B\x1E\x14a\x01~W\x80c\xD2L\xE6\xE5\x14a\x01yW\x80c\xD2\xF7&Z\x14a\x01tW\x80c\xE5$\xF8I\x14a\x01oW\x80c\xF3\xC9s\xCF\x14a\x01jW\x80c\xF9\0^\xB5\x14a\x01eW\x80c\xFA.Y\x94\x14a\x01`Wc\xFA=\xFA\xBB\x03a\x0E\x14Wa\r\xFBV[a\r\xDDV[a\x0C\x9CV[a\x0CyV[a\x0C9V[a\x0C\x10V[a\x0B\xB3V[a\x0BlV[a\x0B\x1FV[a\n\xFEV[a\n\xB9V[a\n\x90V[a\ngV[a\n>V[a\x08\xE5V[a\x08\xC7V[a\x08\xA2V[a\x08\x84V[a\x08fV[a\x07AV[a\x07#V[a\x06\xFDV[a\x06\xD4V[a\x06(V[a\x05bV[a\x04\xA6V[a\x04_V[a\x04GV[a\x04'V[a\x02\x91V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01Ra7\xB7`\xF1\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[4a\x04\x10W`@6`\x03\x19\x01\x12a\x04\x0BWa\x02\xC2`$5a\x02\xB1\x81a\x16\"V[`\nT`\x08\x1C`\xFF\x16`\x045a\x1B\x15V[`\nT`\xFF\x16`\x03Ta\x02\xEB\x90a\x02\xDF\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x80;\x15a\x03\xE7W`@Qcp\xA0\x821`\xE0\x1B\x80\x82R0`\x04\x83\x01R` \x93\x90\x92\x84\x90\x83\x90`$\x90\x82\x90Z\xFA\x90\x81\x15a\x03\xE2Wa\x03/\x92`\0\x92a\x03\xECW[Pa\x18\x1DV[`\x04Ta\x03F\x90a\x02\xDF\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x90\x81;\x15a\x03\xE7W`@Q\x90\x81R0`\x04\x82\x01R\x90\x82\x90\x82\x90`$\x90\x82\x90Z\xFA\x90\x81\x15a\x03\xE2Wa\x03\x80\x92`\0\x92a\x03\xB5W[PP`\x08UV[a\x03\xB1a\x03\x92`\x08T`\x07T\x90a\x115V[`@QcB\xD5\xD4o`\xE1\x1B\x81R`\x04\x81\x01\x91\x90\x91R\x90\x81\x90`$\x82\x01\x90V[\x03\x90\xFD[a\x03\xD4\x92P\x80=\x10a\x03\xDBW[a\x03\xCC\x81\x83a\x0E\xC2V[\x81\x01\x90a\x13\x04V[8\x80a\x03yV[P=a\x03\xC2V[a\x13\x13V[a\x12\xB1V[a\x04\x04\x91\x92P\x85=\x87\x11a\x03\xDBWa\x03\xCC\x81\x83a\x0E\xC2V[\x908a\x03)V[a\x02AV[a\x01\xF1V[` \x90`\x03\x19\x01\x12a\x04\x0BW`\x045\x90V[4a\x04\x10W` a\x04?a\x04:6a\x04\x15V[a&\xC5V[`@Q\x90\x81R\xF3[4a\x04\x10W` a\x04?a\x04Z6a\x04\x15V[a$\xDDV[4a\x04\x10W`@6`\x03\x19\x01\x12a\x04\x0BW`\x045`$5\x90g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17\x82\x15\x15\x16\x15a\x04\xA1W` \x91`@Q\x91\x04\x81R\xF3[`\0\x80\xFD[4a\x04\x10W`@6`\x03\x19\x01\x12a\x04\x0BWa\x04\xD4`$5a\x04\xC6\x81a\x16\"V[`\nT`\x08\x1C`\xFF\x16a\x18\x1DV[`\nT`\xFF\x16`\x03Ta\x04\xF1\x90a\x02\xDF\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x80;\x15a\x03\xE7W`@Qcp\xA0\x821`\xE0\x1B\x80\x82R0`\x04\x83\x01R` \x93\x90\x92\x84\x90\x83\x90`$\x90\x82\x90Z\xFA\x90\x81\x15a\x03\xE2Wa\x03/\x92`\0\x92a\x058W[P`\x045a\x1B\x15V[a\x05P\x91\x92P\x85=\x87\x11a\x03\xDBWa\x03\xCC\x81\x83a\x0E\xC2V[\x908a\x05/V[`\0\x91\x03\x12a\x04\x0BWV[4a\x04\x10W`\x006`\x03\x19\x01\x12a\x04\x0BW`\x03T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[\x80\x15\x15\x03a\x04\xA1WV[``\x90`\x03\x19\x01\x12a\x04\x0BW`\x045\x90`$5a\x05\xB1\x81a\x05\x8BV[\x90`D5\x90V[`\0[\x83\x81\x10a\x05\xCBWPP`\0\x91\x01RV[\x81\x81\x01Q\x83\x82\x01R` \x01a\x05\xBBV[\x90` \x91a\x05\xF4\x81Q\x80\x92\x81\x85R\x85\x80\x86\x01\x91\x01a\x05\xB8V[`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[\x90\x92`\x80\x92a\x06%\x95\x94\x15\x15\x83R` \x83\x01R`@\x82\x01R\x81``\x82\x01R\x01\x90a\x05\xDBV[\x90V[4a\x04\x10Wa\x0666a\x05\x95V[`\x02T`\x01`\x01`\xA0\x1B\x03\x16\x92\x90\x83;\x15a\x03\xE7W`@Qc9(\xFF\x97`\xE0\x1B\x81R`\x04\x81\x01\x92\x90\x92R\x91\x15\x15`$\x82\x01R`D\x81\x01\x91\x90\x91R\x90`\0\x90\x81\x90\x83\x90`d\x90\x82\x90Z\xFA\x80\x15a\x03\xE2W\x81\x80\x93\x81\x80\x93a\x06\xA7W[PP\x90a\x06\xA3\x91`@Q\x94\x85\x94\x85a\x06\0V[\x03\x90\xF3[\x91P\x91Pa\x06\xA3\x93Pa\x06\xCC\x92P=\x80\x91\x83>a\x06\xC4\x81\x83a\x0E\xC2V[\x81\x01\x90a\x13\x1FV[\x91\x938a\x06\x90V[4a\x04\x10W`\x006`\x03\x19\x01\x12a\x04\x0BW`\x02T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x04\x10W`\x006`\x03\x19\x01\x12a\x04\x0BW` `\xFF`\nT`\x08\x1C\x16`@Q\x90\x15\x15\x81R\xF3[4a\x04\x10W` 6`\x03\x19\x01\x12a\x04\x0BW` a\x04?`\x045a(fV[4a\x04\x10W``6`\x03\x19\x01\x12a\x04\x0BW`\x045`$5`D5a\x07d\x81a\x05\x8BV[`\0\x80\x93a\x07za\x07t\x85a\x10[V[\x94a\x10\xA4V[`@\x94a\x07\xA6\x86Qa\x07\x8B\x81a\x0E\xA6V[`\t\x81Rh4\xBA2\xB90\xBA4\xB7\xB7`\xB9\x1B` \x82\x01Ra\x0F\x84V[\x81\x81\x10\x80a\x08]W[\x15a\x08#Wa\x07\xD0a\x07\xCAa\x07\xC4\x83\x85a\x0F\x16V[`\x01\x1C\x90V[\x82a\x0F#V[\x90a\x07\xE2\x82a\x07\xDDa\x0F0V[a\x0F\xCEV[\x84a\x07\xEE\x83\x88\x87a\x11\xCBV[a\x07\xFF\x81a\x07\xFAa\x0FOV[a\x10,V[\x13\x15a\x08\x16WPa\x08\x10\x90\x96a\x0FuV[\x95a\x07\xA6V[\x96\x91Pa\x08\x10\x90\x91a\x0FuV[a\x06\xA3\x93Pa\x07\xC4a\x08A\x93\x92a\x089\x92a\x0F#V[\x93\x84\x91a\x14\xE3V[\x93Q\x92\x83R` \x83\x01R`@\x82\x01\x92\x90\x92R\x90\x81\x90``\x82\x01\x90V[P\x85\x87\x10a\x07\xAFV[4a\x04\x10W`\x006`\x03\x19\x01\x12a\x04\x0BW` `\x08T`@Q\x90\x81R\xF3[4a\x04\x10W`\x006`\x03\x19\x01\x12a\x04\x0BW` `\tT`@Q\x90\x81R\xF3[4a\x04\x10W`@a\x08\xBBa\x08\xB56a\x05\x95V[\x91a\x14\xE3V[\x82Q\x91\x82R` \x82\x01R\xF3[4a\x04\x10W`\x006`\x03\x19\x01\x12a\x04\x0BW` `\x06T`@Q\x90\x81R\xF3[4a\x04\x10W`@6`\x03\x19\x01\x12a\x04\x0BW`\x02T`$5\x90`\x045\x90a\t\x15\x90a\x02\xDF\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x80;\x15a\x03\xE7W`@Qc\x03\xB4\xD1\x03`\xE4\x1B\x81R`\x04\x81\x01\x83\x90R` \x93\x90\x91\x84\x90\x83\x90`$\x90\x82\x90Z\xFA\x80\x15a\x03\xE2Wa\t\x8E\x7F\xD15<h\xE7\x9E\xF7\r\xE8N\xE9\r/\xAC\xF8E\xEC$\x89Q\x16\xD4\xA05\x05\xAAAxZ\xF7\x1FZ\x91a\t\x9A\x94`\0\x91a\n!W[P`@\x80Q\x91\x82RB` \x83\x01R\x90\x91\x82\x91\x82\x01\x90V[\x03\x90\xA1a\x04\xC6\x81a\x16\"V[`\nT`\xFF\x16`\x03Ta\t\xB7\x90a\x02\xDF\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x91\x82;\x15a\x03\xE7W`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x92\x84\x90\x84\x90`$\x90\x82\x90Z\xFA\x92\x83\x15a\x03\xE2Wa\t\xF8\x94`\0\x94a\n\x02W[PPa\x1B\x15V[a\n\0a\x1DrV[\0[a\n\x19\x92\x94P\x80=\x10a\x03\xDBWa\x03\xCC\x81\x83a\x0E\xC2V[\x918\x80a\t\xF1V[a\n8\x91P\x87=\x89\x11a\x03\xDBWa\x03\xCC\x81\x83a\x0E\xC2V[8a\twV[4a\x04\x10W`\x006`\x03\x19\x01\x12a\x04\x0BW`\x05T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x04\x10W`\x006`\x03\x19\x01\x12a\x04\x0BW`\x04T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x04\x10W`\x006`\x03\x19\x01\x12a\x04\x0BW`\0T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x04\x10W`\x006`\x03\x19\x01\x12a\x04\x0BW`\xA0`\x0BT`\x0CT`\xFF`\rT\x16`\x0ET\x90`\x0FT\x92`@Q\x94\x85R` \x85\x01R\x15\x15`@\x84\x01R``\x83\x01R`\x80\x82\x01R\xF3[4a\x04\x10W`@6`\x03\x19\x01\x12a\x04\x0BW` a\x04?`$5`\x045a\x10\xCDV[4a\x04\x10W`@6`\x03\x19\x01\x12a\x04\x0BW`\x045`$5g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x81\x02\x92\x81\x84\x04\x14\x90\x15\x17\x81\x15\x15\x16\x15a\x04\xA1W` \x91`\x01`@Q\x92`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x81R\xF3[4a\x04\x10W` g\x1B\xC1mgN\xC8\0\0a\x0B\xAAa\x0B\x9Aa\x0B\xA5g\x13\xA0K\xBD\xFD\xC9\xBE\x88a\x0B\x9Fa\x0B\x9A6a\x04\x15V[a\x11tV[\x05a\x1F\x0BV[a FV[\x05`@Q\x90\x81R\xF3[4a\x04\x10Wa\x0B\xC16a\x04\x15V[a\x0B\xCA\x81a\x1F\x0BV[\x80\x82\x02\x91`\0\x82\x12`\x01`\xFF\x1B\x82\x14\x16a\x0C\x0BW\x81\x83\x05\x14\x90\x15\x17\x15a\x0C\x0BWg\"\xC9U\"\x95T\xC1\xB6a\x0B\xAAa\x0B\x9Ag\x1B\xC1mgN\xC8\0\0` \x94\x05a!\x81V[a\x0F\0V[4a\x04\x10W`\x006`\x03\x19\x01\x12a\x04\x0BW`\x01T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x04\x10W`@6`\x03\x19\x01\x12a\x04\x0BW`\x045`$5\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x04\xA1W` \x90g\r\xE0\xB6\xB3\xA7d\0\0`@Q\x91\x04\x81R\xF3[4a\x04\x10W`\x006`\x03\x19\x01\x12a\x04\x0BW` `\xFF`\nT\x16`@Q\x90\x15\x15\x81R\xF3[4a\x04\x10W`@6`\x03\x19\x01\x12a\x04\x0BW`\x02T`\x045\x90`$5\x90a\x0C\xCC\x90a\x02\xDF\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x90\x81;\x15a\x03\xE7W`@Qc\x03\xB4\xD1\x03`\xE4\x1B\x81R`\x04\x81\x01\x84\x90R` \x93\x90\x92\x84\x90\x84\x90`$\x90\x82\x90Z\xFA\x80\x15a\x03\xE2Wa\rE\x7F\xD15<h\xE7\x9E\xF7\r\xE8N\xE9\r/\xAC\xF8E\xEC$\x89Q\x16\xD4\xA05\x05\xAAAxZ\xF7\x1FZ\x91a\r`\x95`\0\x91a\n!WP`@\x80Q\x91\x82RB` \x83\x01R\x90\x91\x82\x91\x82\x01\x90V[\x03\x90\xA1a\rQ\x82a\x16\"V[`\nT`\x08\x1C`\xFF\x16\x90a\x1B\x15V[`\nT`\xFF\x16`\x03Ta\r}\x90a\x02\xDF\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x90\x81;\x15a\x03\xE7W`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x91\x83\x90\x83\x90`$\x90\x82\x90Z\xFA\x91\x82\x15a\x03\xE2Wa\t\xF8\x93`\0\x93a\r\xBEW[PPa\x18\x1DV[a\r\xD5\x92\x93P\x80=\x10a\x03\xDBWa\x03\xCC\x81\x83a\x0E\xC2V[\x908\x80a\r\xB7V[4a\x04\x10W`\x006`\x03\x19\x01\x12a\x04\x0BW` `\x07T`@Q\x90\x81R\xF3[4a\x04\x10W` a\x04?a\x0E\x0E6a\x05\x95V[\x91a\x11\xCBV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01Rtnor receive functions`X\x1B`d\x82\x01R`\x84\x90\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x0E\xA1W`@RV[a\x0EwV[`@\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x0E\xA1W`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x0E\xA1W`@RV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x0E\xA1W`\x1F\x01`\x1F\x19\x16` \x01\x90V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x91\x90\x82\x03\x91\x82\x11a\x0C\x0BWV[\x91\x90\x82\x01\x80\x92\x11a\x0C\x0BWV[`@Q\x90a\x0F=\x82a\x0E\xA6V[`\x03\x82Rb\x1BZY`\xEA\x1B` \x83\x01RV[`@Q\x90a\x0F\\\x82a\x0E\xA6V[`\n\x82Riderivative`\xB0\x1B` \x83\x01RV[`\0\x19\x81\x14a\x0C\x0BW`\x01\x01\x90V[a\x0F\xB2a\x0F\xC7a\x0F\xCC\x92`@Q\x92\x83\x91c-\x83\x9C\xB3`\xE2\x1B` \x84\x01R`@`$\x84\x01R`d\x83\x01\x90a\x05\xDBV[`\0`D\x83\x01R\x03`\x1F\x19\x81\x01\x83R\x82a\x0E\xC2V[a\x10\x11V[V[a\x0F\xC7a\x0F\xFD\x91a\x0F\xCC\x93`@Q\x93\x84\x92c-\x83\x9C\xB3`\xE2\x1B` \x85\x01R`@`$\x85\x01R`d\x84\x01\x90a\x05\xDBV[\x90`D\x83\x01R\x03`\x1F\x19\x81\x01\x83R\x82a\x0E\xC2V[`\0\x80\x91` \x81Q\x91\x01jconsole.logZ\xFAPV[a\x0F\xC7a\x0F\xFD\x91a\x0F\xCC\x93`@Q\x93\x84\x92c\x1ES\x13G`\xE1\x1B` \x85\x01R`@`$\x85\x01R`d\x84\x01\x90a\x05\xDBV[`\x1E\x81\x02\x90`\x1E\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x04\xA1Wa\x03\xE8\x90\x04\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x04\xA1Wg\x1B\xC1mgN\xC8\0\0\x90\x04\x90V[a\x03\xE8\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x04\xA1W`\x01`d`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x04\xA1W`\x01g\r\xE0\xB6\xB3\xA7d\0\0`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[\x90g\x11\x90\0\xAB\x10\x0F\xFB\xFF\x19\x82\x01\x91\x82\x13`\x01\x16a\x0C\x0BWV[\x90g\x1B\xC1mgN\xC8\0\0`\0\x83\x82\x03\x93\x12\x81\x84\x12\x81\x16\x91\x84\x13\x90\x15\x16\x17a\x0C\x0BWV[\x81\x81\x03\x92\x91`\0\x13\x80\x15\x82\x85\x13\x16\x91\x84\x12\x16\x17a\x0C\x0BWV[`@Q\x90a\x11[\x82a\x0E\xA6V[`\n\x82Ridifference`\xB0\x1B` \x83\x01RV[\x90g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x81\x02\x92\x81\x84\x05\x14\x90\x15\x17\x15a\x0C\x0BWV[`\0\x81\x90\x03\x91\x90`\x01`\xFF\x1B\x81\x14`\x01\x16a\x0C\x0BW`\0\x19\x83\x05\x03a\x0C\x0BWV[\x90c;\x9A\xCA\0\x91\x82\x81\x02\x92\x81\x84\x05\x14\x90\x15\x17\x15a\x0C\x0BWV[a'\x0F\x19\x83\x01\x91\x83\x83\x11a\x0C\x0BWa'\x10\x84\x01\x93\x84\x81\x11a\x0C\x0BWa\x120a\x12\xA9\x94a\x12\xA3\x93a\x12)a\x06%\x98a\x12\"a\x12\x9D\x96`@Qa\x12\x0B\x81a\x0E\xA6V[`\x05\x81Rdguess`\xD8\x1B` \x82\x01Ra\x0F\xCEV[\x82\x88a\x14\xE3V[P\x95a\x14\xE3V[P\x92a\x12\\\x81`@Qa\x12B\x81a\x0E\xA6V[`\x08\x81Rg\x07\x07&\xF6f\x97EW`\xC4\x1B` \x82\x01Ra\x10,V[a\x12\x88\x84`@Qa\x12l\x81a\x0E\xA6V[`\n\x81Ri897\xB34\xBA\"7\xBB\xB7`\xB1\x1B` \x82\x01Ra\x10,V[a\x0B\x9Aa\x12\x95\x85\x83a\x115V[a\x07\xFAa\x11NV[\x91a\x11tV[\x90a\x115V[aN \x90\x05\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01Rd code`\xD8\x1B`d\x82\x01R`\x84\x90\xFD[\x90\x81` \x91\x03\x12a\x04\x0BWQ\x90V[`@Q=`\0\x82>=\x90\xFD[`\x80\x81\x83\x03\x12a\x04\x0BW\x80Qa\x134\x81a\x05\x8BV[\x92` \x92\x83\x83\x01Q\x93`@\x84\x01Q\x93``\x81\x01Q\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x14GW\x01\x82`\x1F\x82\x01\x12\x15a\x13\xEEW\x80Q\x91a\x13p\x83a\x0E\xE4V[\x93a\x13~`@Q\x95\x86a\x0E\xC2V[\x83\x85R\x81\x84\x84\x01\x01\x11a\x13\x9AWa\x06%\x92\x91\x84\x82\x01\x91\x01a\x05\xB8V[`\x84\x90`@Q\x90bF\x1B\xCD`\xE5\x1B\x82R`\x04\x82\x01R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x83\x90R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x84\x90R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[\x15a\x14\x9EWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FInvalid swap simulation\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x90\xFD[`\0\x80T\x93\x94\x92\x93\x90\x91\x90a\x15\x02\x90a\x02\xDF\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x80;\x15a\x03\xE7W` \x83\x91`\x04`@Q\x80\x94\x81\x93cP\x1A\xD8\xFF`\xE1\x1B\x83RZ\xF1\x90\x81\x15a\x03\xE2W\x83\x91a\x16\x04W[P`\x02Ta\x15H\x90a\x02\xDF\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x80;\x15a\x03\xE7W`@Qc9(\xFF\x97`\xE0\x1B\x81R`\x04\x81\x01\x93\x90\x93R\x85\x15\x15`$\x84\x01R`D\x83\x01\x87\x90R\x83\x90\x83\x90`d\x90\x82\x90Z\xFA\x94\x85\x15a\x03\xE2W\x83\x96\x84\x93\x85\x97a\x15\xD6W[Pa\x15\x9Fa\x15\xBD\x96\x97\x98a\x14\x97V[\x82\x82\x15a\x15\xCFWa\x15\xAF\x91a\x10\xCDV[\x93[P\x15a\x15\xC1WPa\x115V[\x91\x90V[a\x15\xCA\x91a\x10\xCDV[a\x115V[P\x93a\x15\xB1V[a\x15\x9F\x98Pa\x15\xBD\x96\x97Pa\x15\xF6\x91\x94P=\x80\x87\x83>a\x06\xC4\x81\x83a\x0E\xC2V[P\x94\x91\x98\x90\x94\x97\x96Pa\x15\x90V[a\x16\x1C\x91P` =\x81\x11a\x03\xDBWa\x03\xCC\x81\x83a\x0E\xC2V[8a\x150V[`\x04\x80Ta\x16:\x90a\x02\xDF\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x90\x81;\x15a\x03\xE7W`@\x80Qcp\xA0\x821`\xE0\x1B\x80\x82R3\x82\x85\x01\x90\x81R` \x96\x91\x95\x93\x94\x93\x91\x92\x90\x87\x90\x82\x90\x81\x90\x83\x01\x03\x81\x85Z\xFA\x90\x81\x15a\x03\xE2W`\0\x91a\x18\0W[P\x82\x81\x10a\x17\xDDWP\x80;\x15a\x03\xE7W\x83Qcn\xB1v\x9F`\xE1\x1B\x81R3\x84\x82\x01\x90\x81R0` \x82\x01R\x87\x90\x82\x90\x81\x90`@\x01\x03\x81\x85Z\xFA\x90\x81\x15a\x03\xE2W`\0\x91a\x17\xC0W[P\x82\x81\x10a\x17\x9DWP\x80;\x15a\x03\xE7W\x83Qc#\xB8r\xDD`\xE0\x1B\x81R3\x84\x82\x01\x90\x81R0` \x82\x01R`@\x81\x01\x93\x90\x93R\x91`\0\x91\x83\x91\x82\x90\x84\x90\x82\x90``\x01\x03\x92Z\xF1\x80\x15a\x03\xE2Wa\x17\x84W[P\x80Ta\x17+\x90a\x02\xDF\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x80;\x15a\x03\xE7W\x91Q\x92\x83R0\x90\x83\x01\x90\x81R\x83\x91\x83\x91\x82\x90\x81\x90` \x01\x03\x91Z\xFA\x90\x81\x15a\x03\xE2Wa\x0F\xCC\x92`\0\x92a\x17gW[PP`\x07UV[a\x17}\x92P\x80=\x10a\x03\xDBWa\x03\xCC\x81\x83a\x0E\xC2V[8\x80a\x17`V[\x80a\x17\x91a\x17\x97\x92a\x0E\x8DV[\x80a\x05WV[8a\x17\x14V[\x84Qc\xDAV\xD3\xC5`\xE0\x1B\x81R\x80\x85\x01\x91\x82R` \x82\x01\x84\x90R\x90\x81\x90`@\x01\x03\x90\xFD[a\x17\xD7\x91P\x87=\x89\x11a\x03\xDBWa\x03\xCC\x81\x83a\x0E\xC2V[8a\x16\xC5V[\x84Qc\n\xBEZ\x89`\xE0\x1B\x81R\x80\x85\x01\x91\x82R` \x82\x01\x84\x90R\x90\x81\x90`@\x01\x03\x90\xFD[a\x18\x17\x91P\x87=\x89\x11a\x03\xDBWa\x03\xCC\x81\x83a\x0E\xC2V[8a\x16\x7FV[\x15a\x19\x18W`\x03Ta\x189\x90a\x02\xDF\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\0\x80T\x90\x91\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x80;\x15a\x03\xE7W`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16`\x04\x83\x01R`$\x82\x01\x84\x90R\x82\x90\x82\x90`D\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x03\xE2Wa\x19\x05W[P\x80Ta\x18\xA5\x90a\x02\xDF\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x03T`\x01`\x01`\xA0\x1B\x03\x16\x92\x90\x80;\x15a\x03\xE7W`@Qc\xD0\x04\xF0\xF7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x94\x90\x94\x16`\x04\x85\x01R`$\x84\x01\x91\x90\x91R\x82\x90`D\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x03\xE2Wa\x18\xF8WPV[\x80a\x17\x91a\x0F\xCC\x92a\x0E\x8DV[\x80a\x17\x91a\x19\x12\x92a\x0E\x8DV[8a\x18\x8EV[`\x04Ta\x19/\x90a\x02\xDF\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\0\x80T\x90\x92\x91\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x80;\x15a\x03\xE7W`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16`\x04\x83\x01R`$\x82\x01\x83\x90R\x83\x90\x82\x90`D\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x03\xE2Wa\x1AvW[P\x81Ta\x19\x9C\x90a\x02\xDF\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x04T\x90\x91\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x82;\x15a\x03\xE7W`@Qc\xD0\x04\xF0\xF7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16`\x04\x83\x01R`$\x82\x01R\x90\x82\x90\x82\x90`D\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x03\xE2Wa\x1AcW[P`\x03Ta\x1A\x07\x90a\x02\xDF\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x80;\x15a\x03\xE7W`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90` \x90\x82\x90`$\x90\x82\x90Z\xFA\x90\x81\x15a\x03\xE2Wa\x0F\xCC\x92\x91a\x1AEW[P`\x06UV[a\x1A]\x91P` =\x81\x11a\x03\xDBWa\x03\xCC\x81\x83a\x0E\xC2V[8a\x1A?V[\x80a\x17\x91a\x1Ap\x92a\x0E\x8DV[8a\x19\xEFV[\x80a\x17\x91a\x1A\x83\x92a\x0E\x8DV[8a\x19\x85V[`@\x90a\x06%\x93\x92\x81R\x81` \x82\x01R\x01\x90a\x05\xDBV[=\x15a\x1A\xCBW=\x90a\x1A\xB1\x82a\x0E\xE4V[\x91a\x1A\xBF`@Q\x93\x84a\x0E\xC2V[\x82R=`\0` \x84\x01>V[``\x90V[\x90`\x80a\x06%\x92`@\x81R`\x19`@\x82\x01R\x7FDEX swap failed with data\0\0\0\0\0\0\0``\x82\x01R\x81` \x82\x01R\x01\x90a\x05\xDBV[\x81\x15a\x1C\xF0W`\x03Ta\x1B2\x90a\x02\xDF\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x01T`\x01`\x01`\xA0\x1B\x03\x16\x90\x80;\x15a\x03\xE7W`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16`\x04\x83\x01R`$\x82\x01\x85\x90R`\0\x90\x82\x90`D\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x03\xE2Wa\x1C\xDDW[P[`\x02Ta\x1B\x9D\x90a\x02\xDF\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x90\x81;\x15a\x03\xE7W`@Qc9(\xFF\x97`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R\x83\x15\x15`$\x82\x01R`D\x81\x01\x94\x90\x94R`\0\x93\x91\x84\x90\x83\x90`d\x90\x82\x90Z\xFA\x91\x82\x15a\x03\xE2W\x84\x85\x91\x86\x90\x87\x95a\x1C\xBBW[P\x81\x15a\x1C\x9DWPP`\x01Ta\x1C\x0C\x91Pa\x02\xDF\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x91\x82;\x15a\x03\xE7Wa\x1C7\x92\x85\x92\x83`@Q\x80\x96\x81\x95\x82\x94c\xBD\x06%\xAB`\xE0\x1B\x84R`\x04\x84\x01a\x1A\x89V[\x03\x92Z\xF1\x90\x81a\x1C\x8AW[Pa\x1CkWa\x03\xB1a\x1CRa\x1A\xA0V[`@Qcg\xA1k\x8D`\xE1\x1B\x81R\x91\x82\x91`\x04\x83\x01a\x1A\xD0V[\x15a\x1CsWPV[`\x03Ta\x1A\x07\x90a\x02\xDF\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x80a\x17\x91a\x1C\x97\x92a\x0E\x8DV[8a\x1CBV[\x84a\x03\xB1\x91`@Q\x94\x85\x94c\x03\x14\xE6#`\xE3\x1B\x86R`\x04\x86\x01a\x06\0V[\x92PPPa\x1C\xD4\x91\x92P=\x80\x86\x83>a\x06\xC4\x81\x83a\x0E\xC2V[\x93\x92\x908a\x1B\xEBV[\x80a\x17\x91a\x1C\xEA\x92a\x0E\x8DV[8a\x1B\x84V[`\x04Ta\x1D\x07\x90a\x02\xDF\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x01T`\x01`\x01`\xA0\x1B\x03\x16\x90\x80;\x15a\x03\xE7W`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16`\x04\x83\x01R`$\x82\x01\x85\x90R`\0\x90\x82\x90`D\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x03\xE2Wa\x1D_W[Pa\x1B\x86V[\x80a\x17\x91a\x1Dl\x92a\x0E\x8DV[8a\x1DYV[`\x04Ta\x1D\x89\x90a\x02\xDF\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x80;\x15a\x03\xE7W`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90` \x90\x82\x90`$\x90\x82\x90Z\xFA\x80\x15a\x03\xE2Wa\x1D\xC8\x91`\0\x91a\x1E\xEDW[P`\x08UV[`\x08T`\x07T\x90\x81\x81\x10a\x1E\x83Wa\x1E\x04\x7F5}\x90_\x181 \x97\x97\xDFMU\xD7\x9C\\[\xF1\xD9\xF71\x1C\x97j\xFD\x05\xE1=\x88\x1E\xAB\x9B\xC8\x92a\x1E(\x92a\x0F\x16V[a\x1E\x18a\x1E\x13\x82`\tTa\x0F#V[`\tUV[`@Q\x90\x81R\x90\x81\x90` \x82\x01\x90V[\x03\x90\xA1`\x04Ta\x1EB\x90a\x02\xDF\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x08T\x90\x80;\x15a\x03\xE7W`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x92\x90\x92R`\0\x90\x82\x90`D\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x03\xE2Wa\x18\xF8WPV[\x90\x7F\xB6[.\x08]}\x04\x0C1?}N\x1A\xC9\x0FY7\x02o\xEEI~\x0E$\xA7\xEF\xF1jU\xE1\xC5\xEAa\x1E\xB2a\x1E\x18\x84\x84a\x0F\x16V[\x03\x90\xA1a\x03\xB1a\x1E\xC2\x83\x83a\x115V[\x19`@Q\x93\x84\x93c\xB1n7\x83`\xE0\x1B\x85R`\x04\x85\x01`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[a\x1F\x05\x91P` =\x81\x11a\x03\xDBWa\x03\xCC\x81\x83a\x0E\xC2V[8a\x1D\xC2V[`\x01`\xFF\x1B\x81\x14a\x0C\x0BW`\0\x03\x90V[\x90\x81g\r\xE0\xB6\xB3\xA7d\0\0\x01\x91\x82\x12`\x01\x16a\x0C\x0BWV[\x90\x81g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19\x01\x91\x82\x12\x15`\x01\x16a\x0C\x0BWV[\x90\x81g\x14\xA8EL\x19\xE1\xAC\0\x01\x91\x82\x12`\x01\x16a\x0C\x0BWV[\x90\x81g\x0F\xC1\x0E\x01W\x82w\xFF\x19\x01\x91\x82\x12\x15`\x01\x16a\x0C\x0BWV[\x90\x81g\x03\xDE\xBD\x08;\x8C|\0\x01\x91\x82\x12`\x01\x16a\x0C\x0BWV[\x90\x81g\x02\x95\xD4\0\xEA2W\xFF\x19\x01\x91\x82\x12\x15`\x01\x16a\x0C\x0BWV[\x90\x81g\x01W\xD8\xB2\xEC\xC7\x08\0\x01\x91\x82\x12`\x01\x16a\x0C\x0BWV[\x90\x81g\x051\n\xA7\xD5!0\0\x01\x91\x82\x12`\x01\x16a\x0C\x0BWV[\x90\x81g\r\xE0\xCC=\x15a\0\0\x01\x91\x82\x12`\x01\x16a\x0C\x0BWV[\x90\x81g \x05\xFEO&\x8E\xA0\0\x01\x91\x82\x12`\x01\x16a\x0C\x0BWV[\x90\x81g\r\xC5R\x7Fd, \0\x01\x91\x82\x12`\x01\x16a\x0C\x0BWV[\x91\x90\x91`\0\x83\x82\x01\x93\x84\x12\x91\x12\x90\x80\x15\x82\x16\x91\x15\x16\x17a\x0C\x0BWV[\x80\x15a!tWgV\x98\xEE\xF0fp\0\0\x81\x12\x15a!nWgV\x98\xEE\xF0fo\xFF\xFF\x19\x81\x13\x15a!aWa!Oa y\x82a#\x05V[a!\ra!Ja \x98a \x93a \x8E\x85a\x10yV[a\x1F\x1CV[a#\xF5V[\x92a!Ea!@a!;a!4a!.a!)a!#a!\x1Ea!\x18a!\x13\x8Da!\ra!\x08a!\x02a \xFDa \xF7a \xF2a \xECa \xE7a \xE1a \xDC\x8Aa#2V[a\x1F4V[\x89a#\xD4V[a\x1FNV[\x87a#\xD4V[a\x1FfV[\x85a#\xD4V[a\x1F\x80V[\x83a#\xD4V[a\x1F\x98V[\x90a#\xD4V[a\x1F\xB2V[\x8Ca#\xD4V[a\x1F\xCAV[\x8Aa#\xD4V[a\x1F\xE2V[\x88a#\xD4V[\x93\x80a#\xD4V[a\x11\x91V[a\x10\xF9V[a *V[a!\x81V[\x90`\0\x13\x15a\x06%Wa\x06%\x90a\x11\x12V[Pg\x1B\xC1mgN\xC8\0\0\x90V[P`\0\x90V[Pg\r\xE0\xB6\xB3\xA7d\0\0\x90V[h\x02H\xCE6\xA7\x0C\xB2k>\x19\x81\x13\x15a!nWh\x07U\xBFy\x8BJ\x1B\xF1\xE5\x81\x12\x15a\"\xD1We\x03x-\xAC\xE9\xD9\x90`N\x1B\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x91``\x90`\x01`_\x1B\x84\x82\x84\x1B\x05\x01\x82\x1D\x93\x84\x02\x90\x03\x80l\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x81\x01\x02\x82\x1D\x90n\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dPn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x82n\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x81m\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x81m\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x81m\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x81l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x02\x8D\x1D\x01\x02\x8B\x1D\x01\x02\x89\x1D\x01\x02\x87\x1D\x01\x02\x85\x1D\x01\x93m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x93m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEA\x81\x01\x90\x84m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x91\x01\x01\x02\x90\x1D\x01\x02\x01\x05\x02\x90`\xC3\x03\x1C\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x90\xFD[`\x01`\xFF\x1B\x81\x14a# W`\0\x81\x12\x15a\x06%W\x19`\x01\x01\x90V[`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x90\xFD[g\x02_\x0F\xE1\x05\xA3\x14\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x04\xA1Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\t\xD0(\xCCo _\xFF\x19\x81\x81\x02\x91`\x01\x91\x83\x05\x14\x16\x15a\x04\xA1Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x90\x80\x82\x02\x91\x82\x05\x14`\x01\x16\x15a\x04\xA1Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\x1B\xC1mgN\xC7\xFF\xFF\x19\x81\x81\x02\x91`\x01\x91\x83\x05\x14\x16\x15a\x04\xA1Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x04\xA1Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[`\x01\x81\x15\x15\x16\x15a\x04\xA1Wn\xC0\x97\xCE{\xC9\x07\x15\xB3K\x9F\x10\0\0\0\0\x05\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17\x82\x15\x15\x16\x15a\x04\xA1W\x05\x90V[g\x13\xA0K\xBD\xFD\xC9\xBE\x88\x90\x80\x82\x02\x91\x82\x05\x14`\x01\x16\x15a\x04\xA1Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x04\xA1Wg\x1B\xC1mgN\xC8\0\0\x90\x05\x90V[g\x03\xC1f\\z\xAB \0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x04\xA1Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[f\x9F2u$b\xA0\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x04\xA1Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\x06\xF0[Y\xD3\xB2\0\0\x81\x14a!nWg\r\xE0\xB6\xB3\xA7d\0\0\x80\x82\x12\x15a&7W\x81\x15a&XW`\x01\x82\x81\x1B\x91`\x02\x93\x83\x05\x84\x03a\x0C\x0BW`\0\x83\x12\x80\x15a&|W[a&jW\x82\x15a&7Wg\x1B\xC1mgN\xC8\0\0\x83\x14a&XW\x82\x12\x91\x82\x15a&IW\x92[a%L\x84a$]V[\x80\x15a&7Wa%\xAEa%}a%xa%sa%na%\xB3\x95\x99\x97\x96\x99a&\xC5V[a#\xACV[a(fV[a\x11\xB2V[a\x15\xCAa%\x91a%\x8C\x83a$\x88V[a\x1F\xFAV[a%\xA8a \x8Ea \xF7a%\xA3\x86a$\xB3V[a \x12V[\x90a$\x14V[a#]V[\x93`\0\x92[\x81\x84\x10a%\xEAWPPPPa\x06%\x91a%\xD7\x91`\0\x14a%\xDCWa$6V[a\x1F\x0BV[a%\xE5\x90a\x1F\x0BV[a$6V[\x90\x91a&-\x86a&'a&\x02\x85a\x15\xCA\x86\x99\x9Ba FV[a%\xA8a&\x1Da&\x18a!Ja%\xD7\x87\x80a#\xD4V[a#\x85V[a\x12\xA3\x83\x86a#\xD4V[\x90a *V[\x95\x01\x92\x91\x90a%\xB8V[`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x90\xFD[a&R\x90a\x11\x12V[\x92a%CV[`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x90\xFD[`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x90\xFD[Pg\x1B\xC1mgN\xC8\0\0\x83\x13a%\x1FV[\x15a&\x94WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x90\xFD[}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x90a&\xF1`\0\x82\x13a&\x8DV[q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06a'\r\x82a)$V[``\x92\x83\x82`\x9F\x03\x01\x1B`\x9F\x1C\x90`_\x19\x01}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x02\x92l\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x82m\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x81m\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x81m\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x81m\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x81m\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x81lFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x02\x8C\x1D\x01\x02\x8A\x1D\x01\x02\x88\x1D\x01\x02\x86\x1D\x01\x02\x84\x1D\x01\x02\x82\x1D\x01\x91x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x91l\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x90\x82m\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x81l\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x81m\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x81m\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x81l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x81\x01\x02\x89\x1D\x01\x02\x87\x1D\x01\x02\x85\x1D\x01\x02\x83\x1D\x01\x02\x90\x1D\x01\x02\x01\x05\x02\x01\x01`\xAE\x1D\x90V[`\xB5\x81`\x01`\x88\x1B\x81\x10\x15a)\rW[\x80i\x01\0\0\0\0\0\0\0\0\0b\x01\0\0\x92\x10\x15a)\0W[e\x01\0\0\0\0\0\x81\x10\x15a(\xF3W[c\x01\0\0\0\x81\x10\x15a(\xE6W[\x01\x02`\x12\x1C`\x01\x90\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x80\x80\x92\x04\x10\x90\x03\x90V[`\x10\x1C\x91`\x08\x1B\x91a(\xAAV[` \x1C\x91`\x10\x1B\x91a(\x9DV[`@\x1C\x91` \x1B\x91a(\x8EV[Ph\xB5\0\0\0\0\0\0\0\0\x90P`\x80\x82\x90\x1Ca(vV[a)/\x81\x15\x15a&\x8DV[\x80o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x07\x1B\x81\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x81\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x81\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x81\x81\x1C`\xFF\x10`\x03\x1B\x17\x81\x81\x1C`\x0F\x10`\x02\x1B\x17\x81\x81\x1C`\x03\x10`\x01\x1B\x17\x90\x81\x1C`\x01\x10\x17\x90V";
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
        ///Calls the contract's `calculateProfit` (0x912f8283) function
        pub fn calculate_profit(
            &self,
            pool_id: ::ethers::core::types::U256,
            x_for_y: bool,
            amount_in: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::I256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([145, 47, 130, 131], (pool_id, x_for_y, amount_in))
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
        ///Calls the contract's `derivativeOfProfit` (0xfa3dfabb) function
        pub fn derivative_of_profit(
            &self,
            pool_id: ::ethers::core::types::U256,
            x_for_y: bool,
            guess: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([250, 61, 250, 187], (pool_id, x_for_y, guess))
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
        ///Calls the contract's `profitFinder` (0x98d8834d) function
        pub fn profit_finder(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([152, 216, 131, 77], ())
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
        ///Calls the contract's `searchMaxArbProfit` (0x71da1373) function
        pub fn search_max_arb_profit(
            &self,
            pool_id: ::ethers::core::types::U256,
            best_guess: ::ethers::core::types::U256,
            x_for_y: bool,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::I256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([113, 218, 19, 115], (pool_id, best_guess, x_for_y))
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
        ///Calls the contract's `try_lower_exchange_price` (0x380bc680) function
        pub fn try_lower_exchange_price(
            &self,
            pool_id: ::ethers::core::types::U256,
            input_y_amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([56, 11, 198, 128], (pool_id, input_y_amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `try_raise_exchange_price` (0x17e47282) function
        pub fn try_raise_exchange_price(
            &self,
            pool_id: ::ethers::core::types::U256,
            input_y_amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([23, 228, 114, 130], (pool_id, input_y_amount))
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
        Hash
    )]
    #[etherror(
        name = "MaximizingProfitTrade",
        abi = "MaximizingProfitTrade(uint256,uint256)"
    )]
    pub struct MaximizingProfitTrade {
        pub trade: ::ethers::core::types::U256,
        pub profit: ::ethers::core::types::U256,
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
        CatastrophicSwapFailure(CatastrophicSwapFailure),
        DexSwapFailure(DexSwapFailure),
        FindingTradeError(FindingTradeError),
        Infinity(Infinity),
        InsufficientApprovalY(InsufficientApprovalY),
        InsufficientBalanceX(InsufficientBalanceX),
        InsufficientBalanceY(InsufficientBalanceY),
        MaximizingProfitTrade(MaximizingProfitTrade),
        Min(Min),
        NegativeInfinity(NegativeInfinity),
        NotProfitable(NotProfitable),
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
            if let Ok(decoded) = <MaximizingProfitTrade as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MaximizingProfitTrade(decoded));
            }
            if let Ok(decoded) = <Min as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Min(decoded));
            }
            if let Ok(decoded) = <NegativeInfinity as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NegativeInfinity(decoded));
            }
            if let Ok(decoded) = <NotProfitable as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NotProfitable(decoded));
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
                Self::CatastrophicSwapFailure(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DexSwapFailure(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FindingTradeError(element) => {
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
                Self::MaximizingProfitTrade(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Min(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NegativeInfinity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NotProfitable(element) => {
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
                _ if selector
                    == <MaximizingProfitTrade as ::ethers::contract::EthError>::selector() => {
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
                    == <NotProfitable as ::ethers::contract::EthError>::selector() => {
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
                Self::CatastrophicSwapFailure(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DexSwapFailure(element) => ::core::fmt::Display::fmt(element, f),
                Self::FindingTradeError(element) => ::core::fmt::Display::fmt(element, f),
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
                Self::MaximizingProfitTrade(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Min(element) => ::core::fmt::Display::fmt(element, f),
                Self::NegativeInfinity(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotProfitable(element) => ::core::fmt::Display::fmt(element, f),
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
    impl ::core::convert::From<MaximizingProfitTrade> for AtomicV2Errors {
        fn from(value: MaximizingProfitTrade) -> Self {
            Self::MaximizingProfitTrade(value)
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
    impl ::core::convert::From<NotProfitable> for AtomicV2Errors {
        fn from(value: NotProfitable) -> Self {
            Self::NotProfitable(value)
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
    ///Container type for all input parameters for the `calculateProfit` function with signature `calculateProfit(uint256,bool,uint256)` and selector `0x912f8283`
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
    #[ethcall(name = "calculateProfit", abi = "calculateProfit(uint256,bool,uint256)")]
    pub struct CalculateProfitCall {
        pub pool_id: ::ethers::core::types::U256,
        pub x_for_y: bool,
        pub amount_in: ::ethers::core::types::U256,
    }
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
    ///Container type for all input parameters for the `derivativeOfProfit` function with signature `derivativeOfProfit(uint256,bool,uint256)` and selector `0xfa3dfabb`
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
        name = "derivativeOfProfit",
        abi = "derivativeOfProfit(uint256,bool,uint256)"
    )]
    pub struct DerivativeOfProfitCall {
        pub pool_id: ::ethers::core::types::U256,
        pub x_for_y: bool,
        pub guess: ::ethers::core::types::U256,
    }
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
        Hash
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
    ///Container type for all input parameters for the `searchMaxArbProfit` function with signature `searchMaxArbProfit(uint256,uint256,bool)` and selector `0x71da1373`
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
        name = "searchMaxArbProfit",
        abi = "searchMaxArbProfit(uint256,uint256,bool)"
    )]
    pub struct SearchMaxArbProfitCall {
        pub pool_id: ::ethers::core::types::U256,
        pub best_guess: ::ethers::core::types::U256,
        pub x_for_y: bool,
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
        Hash
    )]
    #[ethcall(name = "tradePacket", abi = "tradePacket()")]
    pub struct TradePacketCall;
    ///Container type for all input parameters for the `try_lower_exchange_price` function with signature `try_lower_exchange_price(uint256,uint256)` and selector `0x380bc680`
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
        name = "try_lower_exchange_price",
        abi = "try_lower_exchange_price(uint256,uint256)"
    )]
    pub struct TryLowerExchangePriceCall {
        pub pool_id: ::ethers::core::types::U256,
        pub input_y_amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `try_raise_exchange_price` function with signature `try_raise_exchange_price(uint256,uint256)` and selector `0x17e47282`
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
        name = "try_raise_exchange_price",
        abi = "try_raise_exchange_price(uint256,uint256)"
    )]
    pub struct TryRaiseExchangePriceCall {
        pub pool_id: ::ethers::core::types::U256,
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
        Hash
    )]
    pub enum AtomicV2Calls {
        Xtoy(XtoyCall),
        Ytox(YtoxCall),
        Asset(AssetCall),
        CalculateProfit(CalculateProfitCall),
        Cdf(CdfCall),
        CumulativeProfit(CumulativeProfitCall),
        DerivativeOfProfit(DerivativeOfProfitCall),
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
        ProfitFinder(ProfitFinderCall),
        Quote(QuoteCall),
        RaiseExchangePrice(RaiseExchangePriceCall),
        SearchMaxArbProfit(SearchMaxArbProfitCall),
        SimulateSwap(SimulateSwapCall),
        Solver(SolverCall),
        Sqrt(SqrtCall),
        TradePacket(TradePacketCall),
        TryLowerExchangePrice(TryLowerExchangePriceCall),
        TryRaiseExchangePrice(TryRaiseExchangePriceCall),
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
            if let Ok(decoded) = <CalculateProfitCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CalculateProfit(decoded));
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
            if let Ok(decoded) = <DerivativeOfProfitCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DerivativeOfProfit(decoded));
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
            if let Ok(decoded) = <ProfitFinderCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ProfitFinder(decoded));
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
            if let Ok(decoded) = <SearchMaxArbProfitCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SearchMaxArbProfit(decoded));
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
            if let Ok(decoded) = <TradePacketCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TradePacket(decoded));
            }
            if let Ok(decoded) = <TryLowerExchangePriceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TryLowerExchangePrice(decoded));
            }
            if let Ok(decoded) = <TryRaiseExchangePriceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
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
                Self::CalculateProfit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Cdf(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CumulativeProfit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DerivativeOfProfit(element) => {
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
                Self::ProfitFinder(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Quote(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RaiseExchangePrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SearchMaxArbProfit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SimulateSwap(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Solver(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Sqrt(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TradePacket(element) => {
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
                Self::CalculateProfit(element) => ::core::fmt::Display::fmt(element, f),
                Self::Cdf(element) => ::core::fmt::Display::fmt(element, f),
                Self::CumulativeProfit(element) => ::core::fmt::Display::fmt(element, f),
                Self::DerivativeOfProfit(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
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
                Self::ProfitFinder(element) => ::core::fmt::Display::fmt(element, f),
                Self::Quote(element) => ::core::fmt::Display::fmt(element, f),
                Self::RaiseExchangePrice(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SearchMaxArbProfit(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SimulateSwap(element) => ::core::fmt::Display::fmt(element, f),
                Self::Solver(element) => ::core::fmt::Display::fmt(element, f),
                Self::Sqrt(element) => ::core::fmt::Display::fmt(element, f),
                Self::TradePacket(element) => ::core::fmt::Display::fmt(element, f),
                Self::TryLowerExchangePrice(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TryRaiseExchangePrice(element) => {
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
    impl ::core::convert::From<CalculateProfitCall> for AtomicV2Calls {
        fn from(value: CalculateProfitCall) -> Self {
            Self::CalculateProfit(value)
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
    impl ::core::convert::From<DerivativeOfProfitCall> for AtomicV2Calls {
        fn from(value: DerivativeOfProfitCall) -> Self {
            Self::DerivativeOfProfit(value)
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
    impl ::core::convert::From<SearchMaxArbProfitCall> for AtomicV2Calls {
        fn from(value: SearchMaxArbProfitCall) -> Self {
            Self::SearchMaxArbProfit(value)
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
    impl ::core::convert::From<TradePacketCall> for AtomicV2Calls {
        fn from(value: TradePacketCall) -> Self {
            Self::TradePacket(value)
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
    ///Container type for all return fields from the `calculateProfit` function with signature `calculateProfit(uint256,bool,uint256)` and selector `0x912f8283`
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
    pub struct CalculateProfitReturn(
        pub ::ethers::core::types::I256,
        pub ::ethers::core::types::U256,
    );
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
    ///Container type for all return fields from the `derivativeOfProfit` function with signature `derivativeOfProfit(uint256,bool,uint256)` and selector `0xfa3dfabb`
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
    pub struct DerivativeOfProfitReturn(pub ::ethers::core::types::I256);
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
        Hash
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
        Hash
    )]
    pub struct QuoteReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `searchMaxArbProfit` function with signature `searchMaxArbProfit(uint256,uint256,bool)` and selector `0x71da1373`
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
    pub struct SearchMaxArbProfitReturn {
        pub best_amount_in: ::ethers::core::types::U256,
        pub best_profit: ::ethers::core::types::I256,
        pub expected_price: ::ethers::core::types::U256,
    }
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
        Hash
    )]
    pub struct TradePacketReturn {
        pub block_number: ::ethers::core::types::U256,
        pub block_timestamp: ::ethers::core::types::U256,
        pub raise_price: bool,
        pub amount_in: ::ethers::core::types::U256,
        pub profit: ::ethers::core::types::U256,
    }
}
