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
                    ::std::borrow::ToOwned::to_owned("calculateProfit"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("calculateProfit"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("xForY"),
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
                    ::std::borrow::ToOwned::to_owned("cdf"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("cdf"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("input"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("int256"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("output"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("int256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
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
                    ::std::borrow::ToOwned::to_owned("derivativeOfProfit"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("derivativeOfProfit"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("xForY"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bool"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("guess"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("int256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("divWadDown"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("divWadDown"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("x"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("y"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("z"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("divWadUp"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("divWadUp"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("x"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("y"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("z"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
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
                    ::std::borrow::ToOwned::to_owned("log"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("log"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("x"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("int256"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("z"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("int256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("lower_exchange_price"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("lower_exchange_price",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("input"),
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
                    ::std::borrow::ToOwned::to_owned("mulWadDown"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("mulWadDown"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("x"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("y"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("z"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("mulWadUp"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("mulWadUp"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("x"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("y"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("z"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("pdf"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("pdf"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("input"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("int256"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("output"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("int256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ppf"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("ppf"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("input"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("int256"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("output"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("int256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
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
                            name: ::std::borrow::ToOwned::to_owned("input"),
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
                    ::std::borrow::ToOwned::to_owned("searchMaxArbProfit"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("searchMaxArbProfit"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("best_guess"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
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
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
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
                    ::std::borrow::ToOwned::to_owned("simulateSwap"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("simulateSwap"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("swapXIn"),
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
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("solver"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("solver"),
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
                    ::std::borrow::ToOwned::to_owned("sqrt"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("sqrt"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("x"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("z"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
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
                    ::std::borrow::ToOwned::to_owned("try_lower_exchange_price"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("try_lower_exchange_price",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("input"),
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
                            name: ::std::borrow::ToOwned::to_owned("input"),
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
                    ::std::borrow::ToOwned::to_owned("Price"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("Price"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("price"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("timestamp"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
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
                    ::std::borrow::ToOwned::to_owned("Infinity"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("Infinity"),
                        inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("Min"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("Min"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NegativeInfinity"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("NegativeInfinity"),
                        inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("OutOfBounds"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("OutOfBounds"),
                        inputs: ::std::vec![],
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
    /// The parsed JSON ABI of the contract.
    pub static ATOMICV2_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R`\n\x80Ta\xFF\xFF\x19\x16`\x01\x17\x90U4\x80\x15b\0\0lW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`@Qb\x003L8\x03\x80b\x003L\x839\x81\x01`@\x81\x90Rb\0\0\x8F\x91b\0\x01hV[`\x02\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16`\x01`\x01`\xA0\x1B\x03\x88\x81\x16\x91\x90\x91\x17\x90\x92U`\x01\x80T\x82\x16\x87\x84\x16\x17\x90U`\0\x80T\x82\x16\x86\x84\x16\x17\x90U`\x03\x80T\x82\x16\x85\x84\x16\x17\x90U`\x04\x80T\x90\x91\x16\x91\x83\x16\x91\x90\x91\x17\x90U`@Qb\0\0\xF3\x90b\0\x01=V[`@Q\x80\x91\x03\x90`\0\xF0\x80\x15\x80\x15b\0\x01\x10W=`\0\x80>=`\0\xFD[P`\x05\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UPb\0\x02#\x93PPPPV[a\x01\x8B\x80b\x001\xC1\x839\x01\x90V[\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x01cW`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15b\0\x01\xCCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[b\0\x01\xD7\x86b\0\x01KV[\x94Pb\0\x01\xE7` \x87\x01b\0\x01KV[\x93Pb\0\x01\xF7`@\x87\x01b\0\x01KV[\x92Pb\0\x02\x07``\x87\x01b\0\x01KV[\x91Pb\0\x02\x17`\x80\x87\x01b\0\x01KV[\x90P\x92\x95P\x92\x95\x90\x93PV[a/\x8E\x80b\0\x023`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x02'W`\x005`\xE0\x1C\x80c\x98\xD8\x83M\x11a\x01QW\x80c\xD0\xB7\x1B\x1E\x11a\0\xEFW\x80c\xE5$\xF8I\x11a\0\xBEW\x80c\xE5$\xF8I\x14a\x05\x15W\x80c\xEF\xA3Lx\x14a\x05(W\x80c\xF3\xC9s\xCF\x14a\x05;W\x80c\xFA.Y\x94\x14a\x05HWa\x02'V[\x80c\xD0\xB7\x1B\x1E\x14a\x04\xC9W\x80c\xD2L\xE6\xE5\x14a\x04\xDCW\x80c\xD2\xF7&Z\x14a\x04\xEFW\x80c\xD7\x82=\xF7\x14a\x05\x02Wa\x02'V[\x80c\xA1S\x87\x89\x11a\x01+W\x80c\xA1S\x87\x89\x14a\x04/W\x80c\xAE\x97h\xA8\x14a\x04{W\x80c\xB7\xDA+\xAF\x14a\x04\x8EW\x80c\xBD%-\x06\x14a\x04\xB6Wa\x02'V[\x80c\x98\xD8\x83M\x14a\x03\xF6W\x80c\x99\x9B\x93\xAF\x14a\x04\tW\x80c\x9F'\xEFO\x14a\x04\x1CWa\x02'V[\x80cI\xA7\xA2m\x11a\x01\xC9W\x80cw\xE5\xEA\x07\x11a\x01\x98W\x80cw\xE5\xEA\x07\x14a\x03\xBEW\x80c\x85\xB3\x19\xFF\x14a\x03\xD1W\x80c\x8A/\xA5J\x14a\x03\xDAW\x80c\x93e \xC3\x14a\x03\xEDWa\x02'V[\x80cI\xA7\xA2m\x14a\x03mW\x80cdI\xFCW\x14a\x03\x80W\x80cgsB\xCE\x14a\x03\xA2W\x80cr\xB9\x82F\x14a\x03\xB5Wa\x02'V[\x80c5\xA9\x9A\xD0\x11a\x02\x05W\x80c5\xA9\x9A\xD0\x14a\x03\x07W\x80c6yr:\x14a\x03\x1CW\x80c7\xC6\xA4J\x14a\x03/W\x80c8\xD5.\x0F\x14a\x03BWa\x02'V[\x80c\n\xC304\x14a\x02\x8CW\x80c\x18\\\x1A^\x14a\x02\xB8W\x80c-[l\xB9\x14a\x02\xE6W[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x02\x9Fa\x02\x9A6`\x04a*\x8EV[a\x05QV[`@Qa\x02\xAF\x94\x93\x92\x91\x90a+\rV[`@Q\x80\x91\x03\x90\xF3[a\x02\xCBa\x02\xC66`\x04a+>V[a\x061V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x02\xAFV[a\x02\xF9a\x02\xF46`\x04a+qV[a\x07rV[`@Q\x90\x81R` \x01a\x02\xAFV[a\x03\x1Aa\x03\x156`\x04a+qV[a\x07\x83V[\0[a\x02\xF9a\x03*6`\x04a+qV[a\tkV[a\x02\xF9a\x03=6`\x04a+\x8DV[a\tvV[`\x03Ta\x03U\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\xAFV[`\x02Ta\x03U\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\nTa\x03\x92\x90a\x01\0\x90\x04`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x02\xAFV[a\x02\xF9a\x03\xB06`\x04a+qV[a\t\x89V[a\x02\xF9`\x08T\x81V[a\x02\xF9a\x03\xCC6`\x04a*\x8EV[a\t\x94V[a\x02\xF9`\tT\x81V[a\x03\x1Aa\x03\xE86`\x04a+qV[a\n\xCDV[a\x02\xF9`\x06T\x81V[`\x05Ta\x03U\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x04Ta\x03U\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\0Ta\x03U\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x0BT`\x0CT`\rT`\x0ET`\x0FTa\x04L\x94\x93\x92`\xFF\x16\x91\x90\x85V[`@Qa\x02\xAF\x95\x94\x93\x92\x91\x90\x94\x85R` \x85\x01\x93\x90\x93R\x90\x15\x15`@\x84\x01R``\x83\x01R`\x80\x82\x01R`\xA0\x01\x90V[a\x02\xF9a\x04\x896`\x04a+\x8DV[a\x0C\xA9V[a\x04\xA1a\x04\x9C6`\x04a*\x8EV[a\x0C\xB5V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x02\xAFV[a\x02\xF9a\x04\xC46`\x04a+\x8DV[a\x0E\xF5V[a\x02\xF9a\x04\xD76`\x04a+qV[a\x0F\x01V[a\x02\xF9a\x04\xEA6`\x04a+qV[a\x0F\x0CV[`\x01Ta\x03U\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x03\x1Aa\x05\x106`\x04a+qV[a\x0F\x17V[a\x02\xF9a\x05#6`\x04a+\x8DV[a\x10UV[a\x03\x1Aa\x0566`\x04a+qV[a\x10aV[`\nTa\x03\x92\x90`\xFF\x16\x81V[a\x02\xF9`\x07T\x81V[`\x02T`@Qc\x02\xB0\xCC\r`\xE2\x1B\x81R\x83\x15\x15`\x04\x82\x01R`$\x81\x01\x83\x90R`\0\x91\x82\x91\x82\x91``\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\n\xC304\x90`D\x01`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x05\xE5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/n\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x05\xF9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x06!\x91\x90\x81\x01\x90a+\xC8V[\x92\x99\x91\x98P\x96P\x90\x94P\x92PPPV[`\0\x80\x80\x80\x80\x80\x80a\x06G\x89`\x1Ea\x03\xE8a\x10\xB8V[\x90P`\0a\x06Y\x8Aa\x03\xE8`da\x10\xD7V[\x90Pa\x06\x86`@Q\x80`@\x01`@R\x80`\t\x81R` \x01h4\xBA2\xB90\xBA4\xB7\xB7`\xB9\x1B\x81RP\x84a\x11\x05V[\x80\x82\x10\x80\x15a\x06\x95WP`@\x83\x10[\x15a\x07?W`\x02a\x06\xA6\x83\x83a-\xA3V[a\x06\xB0\x91\x90a-\xCCV[a\x06\xBA\x90\x83a-\xE0V[\x94Pa\x06\xE1`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01b\x1BZY`\xEA\x1B\x81RP\x86a\x11\x05V[a\x06\xEB\x89\x86a\t\x94V[\x93Pa\x07\x19`@Q\x80`@\x01`@R\x80`\n\x81R` \x01iderivative`\xB0\x1B\x81RP\x85a\x11JV[`\0\x84\x13\x15a\x07*W\x84\x91Pa\x07-V[P\x83[\x82a\x077\x81a-\xF3V[\x93PPa\x06\x86V[`\x02a\x07K\x82\x84a-\xE0V[a\x07U\x91\x90a-\xCCV[\x97Pa\x07a\x89\x89a\x0C\xB5V[\x98\x9B\x90\x9AP\x97\x98PPPPPPPPV[`\0a\x07}\x82a\x11\x8FV[\x92\x91PPV[`\x02T`@\x80QcB\xC9\x88[`\xE1\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x85\x93\x10\xB6\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\x08\x06W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/n\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x08\x1AW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08>\x91\x90a.\x0CV[`@\x80Q\x82\x81RB` \x82\x01R\x91\x92P\x7F\xD15<h\xE7\x9E\xF7\r\xE8N\xE9\r/\xAC\xF8E\xEC$\x89Q\x16\xD4\xA05\x05\xAAAxZ\xF7\x1FZ\x91\x01`@Q\x80\x91\x03\x90\xA1a\x08\x82\x82a\x13jV[`\nTa\x08\x97\x90a\x01\0\x90\x04`\xFF\x16\x83a\x16\xB3V[`\nT`\x03T`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01Ra\t_\x92`\xFF\x16\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01[` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\t\"W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/n\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\t6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\tZ\x91\x90a.\x0CV[a\x1A\x1BV[a\tga\x1E#V[PPV[`\0a\x07}\x82a mV[`\0a\t\x82\x83\x83a!\x13V[\x93\x92PPPV[`\0a\x07}\x82a!(V[`\0\x80a\t\xA3a'\x10\x84a-\xA3V[\x90P`\0a\t\xB3\x84a'\x10a-\xE0V[\x90Pa\t\xDC`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01dguess`\xD8\x1B\x81RP\x85a\x11\x05V[`\0a\t\xE8\x86\x83a\x0C\xB5V[P\x90P`\0a\t\xF7\x87\x85a\x0C\xB5V[P\x90Pa\n$`@Q\x80`@\x01`@R\x80`\x08\x81R` \x01g\x07\x07&\xF6f\x97EW`\xC4\x1B\x81RP\x83a\x11JV[a\nP`@Q\x80`@\x01`@R\x80`\n\x81R` \x01i897\xB34\xBA\"7\xBB\xB7`\xB1\x1B\x81RP\x82a\x11JV[`@\x80Q\x80\x82\x01\x90\x91R`\n\x81Ridifference`\xB0\x1B` \x82\x01Ra\n\x84\x90a\n\x7F\x83\x85a.(V[a\x11JV[`\0aN a\n\x9Bg\r\xE0\xB6\xB3\xA7d\0\0\x84a.OV[a\n\xADg\r\xE0\xB6\xB3\xA7d\0\0\x86a.OV[a\n\xB7\x91\x90a.(V[a\n\xC1\x91\x90a.\x7FV[\x98\x97PPPPPPPPV[`\x02T`@\x80QcB\xC9\x88[`\xE1\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x85\x93\x10\xB6\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\x0BPW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/n\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0BdW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\x88\x91\x90a.\x0CV[`@\x80Q\x82\x81RB` \x82\x01R\x91\x92P\x7F\xD15<h\xE7\x9E\xF7\r\xE8N\xE9\r/\xAC\xF8E\xEC$\x89Q\x16\xD4\xA05\x05\xAAAxZ\xF7\x1FZ\x91\x01`@Q\x80\x91\x03\x90\xA1a\x0B\xCC\x82a\x13jV[`\nTa\x0B\xE1\x90a\x01\0\x90\x04`\xFF\x16\x83a\x1A\x1BV[`\nT`\x03T`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01Ra\t_\x92`\xFF\x16\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01[` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0ClW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/n\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0C\x80W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xA4\x91\x90a.\x0CV[a\x16\xB3V[`\0a\t\x82\x83\x83a!\xCCV[`\0\x80`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xA05\xB1\xFE`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\rGW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/n\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\r[W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\x7F\x91\x90a.\x0CV[`\x02T`@Qc\x02\xB0\xCC\r`\xE2\x1B\x81R\x87\x15\x15`\x04\x82\x01R`$\x81\x01\x87\x90R\x91\x92P`\0\x91\x82\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\n\xC304\x90`D\x01`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0E\x13W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/n\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0E'W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0EO\x91\x90\x81\x01\x90a+\xC8V[P\x92P\x92P\x92P\x82a\x0E\xA8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FInvalid swap simulation\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x88a\x0E\xB5W\x87a\x0E\xBFV[a\x0E\xBF\x88\x86a!\xCCV[\x90P`\0\x89a\x0E\xD7Wa\x0E\xD2\x84\x87a!\xCCV[a\x0E\xD9V[\x83[\x90Pa\x0E\xE5\x82\x82a.(V[\x9A\x92\x99P\x91\x97PPPPPPPPV[`\0a\t\x82\x83\x83a!\xE1V[`\0a\x07}\x82a!\xF6V[`\0a\x07}\x82a\"_V[a\x0F \x81a\x13jV[`\nTa\x0F5\x90a\x01\0\x90\x04`\xFF\x16\x82a\x1A\x1BV[`\nT`\x03T`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01Ra\x0Fn\x92`\xFF\x16\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01a\x0C\x16V[`\x04\x80T`@Qcp\xA0\x821`\xE0\x1B\x81R0\x92\x81\x01\x92\x90\x92R`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0F\xF0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/n\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x10\x04W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10(\x91\x90a.\x0CV[`\x08\x81\x90U`\x07Ta\x109\x91a.(V[`@QcB\xD5\xD4o`\xE1\x1B\x81R`\x04\x01a\x0E\x9F\x91\x81R` \x01\x90V[`\0a\t\x82\x83\x83a\"\xBBV[a\x10j\x81a\x13jV[`\nTa\x10\x7F\x90a\x01\0\x90\x04`\xFF\x16\x82a\x16\xB3V[`\nT`\x03T`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01Ra\x0Fn\x92`\xFF\x16\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01a\x08\xCCV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x10\xD0W`\0\x80\xFD[\x04\x92\x91PPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x10\xEFW`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[a\tg\x82\x82`@Q`$\x01a\x11\x1B\x92\x91\x90a.\xADV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c-\x83\x9C\xB3`\xE2\x1B\x17\x90Ra\"\xD0V[a\tg\x82\x82`@Q`$\x01a\x11`\x92\x91\x90a.\xADV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\x1ES\x13G`\xE1\x1B\x17\x90Ra\"\xD0V[`\0\x80\x82\x13a\x11\xCCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x0E\x9FV[`\0``a\x11\xD9\x84a\"\xDCV[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[`\x04\x80T`@Qcp\xA0\x821`\xE0\x1B\x81R3\x92\x81\x01\x92\x90\x92R`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x13\xF1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/n\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x14\x05W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14)\x91\x90a.\x0CV[\x90P\x81\x81\x10\x15a\x14VW`@Qc\n\xBEZ\x89`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x81\x01\x83\x90R`D\x01a\x0E\x9FV[`\x04\x80T`@Qcn\xB1v\x9F`\xE1\x1B\x81R3\x92\x81\x01\x92\x90\x92R0`$\x83\x01R`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xDDb\xED>\x90`D\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x14\xE3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/n\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x14\xF7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\x1B\x91\x90a.\x0CV[\x90P\x82\x81\x10\x15a\x15HW`@Qc\xDAV\xD3\xC5`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x81\x01\x84\x90R`D\x01a\x0E\x9FV[`\x04\x80T`@Qc#\xB8r\xDD`\xE0\x1B\x81R3\x92\x81\x01\x92\x90\x92R0`$\x83\x01R`D\x82\x01\x85\x90R`\x01`\x01`\xA0\x1B\x03\x16\x90c#\xB8r\xDD\x90`d\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x15\xD9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/n\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x15\xEDW=`\0\x80>=`\0\xFD[PP`\x04\x80T`@Qcp\xA0\x821`\xE0\x1B\x81R0\x92\x81\x01\x92\x90\x92R`\x01`\x01`\xA0\x1B\x03\x16\x92Pcp\xA0\x821\x91P`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x16sW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/n\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x16\x87W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\xAB\x91\x90a.\x0CV[`\x07UPPPV[\x81\x15a\x18\tW`\x03T`\0T`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x84\x90R\x91\x16\x90c\t^\xA7\xB3\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x17GW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/n\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x17[W=`\0\x80>=`\0\xFD[PP`\0T`\x03T`@Qc\xD0\x04\xF0\xF7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x86\x90R\x91\x16\x92Pc\xD0\x04\xF0\xF7\x91P`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x17\xEDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/n\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x18\x01W=`\0\x80>=`\0\xFD[PPPPPPV[`\x04\x80T`\0T`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x93\x81\x01\x93\x90\x93R`$\x83\x01\x84\x90R\x16\x90c\t^\xA7\xB3\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x18\x99W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/n\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x18\xADW=`\0\x80>=`\0\xFD[PP`\0T`\x04\x80T`@Qc\xD0\x04\xF0\xF7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x92\x81\x01\x92\x90\x92R`$\x82\x01\x86\x90R\x90\x91\x16\x92Pc\xD0\x04\xF0\xF7\x91P`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x19CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/n\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x19WW=`\0\x80>=`\0\xFD[PP`\x03T`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x92Pcp\xA0\x821\x91P`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x19\xDCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/n\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x19\xF0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A\x14\x91\x90a.\x0CV[`\x06UPPV[\x81\x15a\x1A\xCCW`\x03T`\x01T`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x84\x90R\x91\x16\x90c\t^\xA7\xB3\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1A\xAFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/n\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x1A\xC3W=`\0\x80>=`\0\xFD[PPPPa\x1BuV[`\x04\x80T`\x01T`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x93\x81\x01\x93\x90\x93R`$\x83\x01\x84\x90R\x16\x90c\t^\xA7\xB3\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1B\\W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/n\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x1BpW=`\0\x80>=`\0\xFD[PPPP[`\x02T`@Qc\x02\xB0\xCC\r`\xE2\x1B\x81R\x83\x15\x15`\x04\x82\x01R`$\x81\x01\x83\x90R`\0\x91\x82\x91\x82\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\n\xC304\x90`D\x01`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x1C\x08W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/n\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x1C\x1CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1CD\x91\x90\x81\x01\x90a+\xC8V[\x93P\x93P\x93P\x93P\x83a\x1CrW\x83\x83\x83\x83`@Qc\x03\x14\xE6#`\xE3\x1B\x81R`\x04\x01a\x0E\x9F\x94\x93\x92\x91\x90a+\rV[`\x01T`@Qc1>\xEA\xB5`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cb}\xD5j\x90a\x1C\xA2\x90\x84\x90`\x04\x01a.\xCFV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1C\xFAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/n\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x92PPP\x80\x15a\x1D\x0BWP`\x01[a\x1DZW=\x80\x80\x15a\x1D9W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x1D>V[``\x91P[P\x80`@Qcg\xA1k\x8D`\xE1\x1B\x81R`\x04\x01a\x0E\x9F\x91\x90a.\xE2V[\x85a\x18\x01W`\x03T`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x1D\xE0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/n\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x1D\xF4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E\x18\x91\x90a.\x0CV[`\x06UPPPPPPV[`\x04\x80T`@Qcp\xA0\x821`\xE0\x1B\x81R0\x92\x81\x01\x92\x90\x92R`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x1E\xA5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/n\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x1E\xB9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E\xDD\x91\x90a.\x0CV[`\x08\x81\x90U`\x07T\x11\x15a\x1FhW\x7F\xB6[.\x08]}\x04\x0C1?}N\x1A\xC9\x0FY7\x02o\xEEI~\x0E$\xA7\xEF\xF1jU\xE1\xC5\xEA`\x08T`\x07Ta\x1F\x1C\x91\x90a-\xA3V[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xA1`\x07T`\x08Ta\x1F=\x81\x83a.(V[`@Qc\xB1n7\x83`\xE0\x1B\x81R`\x04\x81\x01\x93\x90\x93R`$\x83\x01\x91\x90\x91R\x19`D\x82\x01R`d\x01a\x0E\x9FV[`\0`\x07T`\x08Ta\x1Fz\x91\x90a-\xA3V[\x90P\x80`\t`\0\x82\x82Ta\x1F\x8E\x91\x90a-\xE0V[\x90\x91UPP`@Q\x81\x81R\x7F5}\x90_\x181 \x97\x97\xDFMU\xD7\x9C\\[\xF1\xD9\xF71\x1C\x97j\xFD\x05\xE1=\x88\x1E\xAB\x9B\xC8\x90` \x01`@Q\x80\x91\x03\x90\xA1`\x04\x80T`\x08T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3\x93\x81\x01\x93\x90\x93R`$\x83\x01R`\x01`\x01`\xA0\x1B\x03\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a RW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/n\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a fW=`\0\x80>=`\0\xFD[PPPPPV[`\0g\x06\xF0[Y\xD3\xB2\0\0\x82\x03a \x86WP`\0\x91\x90PV[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x12a \xAEW`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a \xCFW`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a \xDC\x83`\x02a.OV[\x90P`\0a \xE9\x82a#\x84V[\x90P`\0a \xFFg\x13\xA0K\xBD\xFD\xC9\xBE\x88\x83a&\x02V[\x90Pa!\n\x81a/)V[\x95\x94PPPPPV[`\0a\t\x82\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x10\xB8V[`\xB5\x81`\x01`\x88\x1B\x81\x10a!AW`@\x91\x90\x91\x1B\x90`\x80\x1C[i\x01\0\0\0\0\0\0\0\0\0\x81\x10a!]W` \x91\x90\x91\x1B\x90`@\x1C[e\x01\0\0\0\0\0\x81\x10a!uW`\x10\x91\x90\x91\x1B\x90` \x1C[c\x01\0\0\0\x81\x10a!\x8BW`\x08\x91\x90\x91\x1B\x90`\x10\x1C[b\x01\0\0\x01\x02`\x12\x1C\x80\x82\x04\x01`\x01\x90\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x90\x81\x90\x04\x81\x11\x90\x03\x90V[`\0a\t\x82\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x10\xD7V[`\0a\t\x82\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x10\xD7V[`\0\x80g\x13\xA0K\xBD\xFD\xC9\xBE\x88a\"\x14g\r\xE0\xB6\xB3\xA7d\0\0\x85a.OV[a\"\x1E\x91\x90a.\x7FV[\x90P`\0a\"+\x82a/)V[\x90P`\0a\"8\x82a&\x17V[\x90Pg\x1B\xC1mgN\xC8\0\0a\"Ug\r\xE0\xB6\xB3\xA7d\0\0\x83a.OV[a!\n\x91\x90a.\x7FV[`\0\x80g\x1B\xC1mgN\xC8\0\0\x83a\"u\x81a/)V[a\"\x7F\x91\x90a.OV[a\"\x89\x91\x90a.\x7FV[\x90Pa\"\x94\x81a'\xFBV[\x90Pg\"\xC9U\"\x95T\xC1\xB6a\"\xB1g\r\xE0\xB6\xB3\xA7d\0\0\x83a.OV[a\t\x82\x91\x90a.\x7FV[`\0a\t\x82\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x10\xB8V[a\"\xD9\x81a)\xA4V[PV[`\0\x80\x82\x11a#\x19W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x0E\x9FV[P`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x82\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[`\0\x80\x82\x12\x80a#\x9BWPg\x1B\xC1mgN\xC8\0\0\x82\x13[\x15a#\xB9W`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a#\xDAW`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81g\x1B\xC1mgN\xC8\0\0\x03a$\x02W`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x15a$\rW\x91\x90PV[`\0g\r\xE0\xB6\xB3\xA7d\0\0\x83\x12a$5Wa$0\x83g\x1B\xC1mgN\xC8\0\0a.(V[a$7V[\x82[\x90P`\0a$M\x82g\x1B\xC1mgN\xC8\0\0a)\xC5V[\x90P\x80`\0\x03a$pW`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a${\x82a\x11\x8FV[\x90P`\0c;\x9A\xCA\0a$\xA6a$\xA1a$\x9Bg\x1B\xC1mgN\xC8\0\0a/)V[\x85a&\x02V[a!(V[a$\xB0\x91\x90a.OV[\x90P`\0\x80a$\xC7\x83g\x03\xC1f\\z\xAB \0a&\x02V[a$\xD9\x90g \x05\xFEO&\x8E\xA0\0a/EV[\x90P`\0a%\t\x84a$\xF2\x86f\x9F2u$b\xA0\0a&\x02V[a%\x04\x90g\r\xC5R\x7Fd, \0a/EV[a&\x02V[a%\x1B\x90g\r\xE0\xB6\xB3\xA7d\0\0a/EV[\x90Pa%?g\t\xD0(\xCCo _\xFF\x19\x85a%5\x85\x85a)\xC5V[a%\x04\x91\x90a.(V[\x92PPP`\0[`\x02\x81\x10\x15a%\xDAW`\0\x86a%[\x84a&\x17V[a%e\x91\x90a.(V[\x90P`\0a%s\x84\x85a&\x02V[a%|\x90a/)V[\x90P`\0a%\x89\x82a'\xFBV[\x90P`\0a%\x97\x86\x85a&\x02V[a%\xA9g\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x84a&\x02V[a%\xB3\x91\x90a.(V[\x90Pa%\xBF\x84\x82a)\xC5V[a%\xC9\x90\x87a/EV[\x95P\x84`\x01\x01\x94PPPPPa%FV[g\r\xE0\xB6\xB3\xA7d\0\0\x88\x12a%\xF7Wa%\xF2\x82a/)V[a\n\xC1V[P\x96\x95PPPPPPV[`\0a\t\x82\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a)\xD6V[`\0\x81`\0\x03a&0WPg\r\xE0\xB6\xB3\xA7d\0\0\x91\x90PV[gV\x98\xEE\xF0fp\0\0\x82\x12a&GWP`\0\x91\x90PV[a&XgV\x98\xEE\xF0fp\0\0a/)V[\x82\x13a&mWPg\x1B\xC1mgN\xC8\0\0\x91\x90PV[`\0a&x\x83a)\xF5V[\x90P`\0a&\xB1g\r\xE0\xB6\xB3\xA7d\0\0a&\x9A\x84g\x1B\xC1mgN\xC8\0\0a!\x13V[a&\xAC\x90g\r\xE0\xB6\xB3\xA7d\0\0a/EV[a)\xC5V[\x90P`\0\x80\x82a'\r\x81a&\xFA\x81a&\xE8\x81a&\xD5\x81g\x02_\x0F\xE1\x05\xA3\x14\0a&\x02V[a%\x04\x90g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19a/EV[a%\x04\x90g\x14\xA8EL\x19\xE1\xAC\0a/EV[a%\x04\x90g\x0F\xC1\x0E\x01W\x82w\xFF\x19a/EV[a'\x1F\x90g\x03\xDE\xBD\x08;\x8C|\0a/EV[\x91P\x83\x90Pa'\x87\x81a'u\x81a'c\x81a'Q\x81a'>\x81\x8Ba&\x02V[a%\x04\x90g\x02\x95\xD4\0\xEA2W\xFF\x19a/EV[a%\x04\x90g\x01W\xD8\xB2\xEC\xC7\x08\0a/EV[a%\x04\x90g\x051\n\xA7\xD5!0\0a/EV[a%\x04\x90g\r\xE0\xCC=\x15a\0\0a/EV[\x91P\x81g\x11\x90\0\xAB\x10\x0F\xFC\0a'\x9D\x87\x88a&\x02V[a'\xA9\x90`\0\x19a.OV[a'\xB3\x91\x90a.(V[a'\xBD\x91\x90a/EV[\x92PP`\0a'\xCB\x83a'\xFBV[\x90P`\0a'\xD9\x85\x83a&\x02V[\x90P`\0\x88\x12a'\xE9W\x80a\n\xC1V[a\n\xC1\x81g\x1B\xC1mgN\xC8\0\0a.(V[`\0h\x02H\xCE6\xA7\x0C\xB2k>\x19\x82\x13a(\x16WP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a(]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x01a\x0E\x9FV[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05`\x01`_\x1B\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1Dm\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1Dn\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[\x80Qjconsole.log` \x83\x01`\0\x80\x84\x83\x85Z\xFAPPPPPV[`\0a\t\x82\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x05\x85\x14\x17\x16a)\xEEW`\0\x80\xFD[\x05\x92\x91PPV[`\0`\x01`\xFF\x1B\x82\x03a*\x1BW`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x12\x15a*,WP\x19`\x01\x01\x90V[P\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[\x80\x15\x15\x81\x14a\"\xD9W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a*\xA4Wa*\xA4a*0V[\x825a*\xAF\x81a*\x80V[\x94` \x93\x90\x93\x015\x93PPPV[`\0[\x83\x81\x10\x15a*\xD8W\x81\x81\x01Q\x83\x82\x01R` \x01a*\xC0V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra*\xF9\x81` \x86\x01` \x86\x01a*\xBDV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[\x84\x15\x15\x81R\x83` \x82\x01R\x82`@\x82\x01R`\x80``\x82\x01R`\0a+4`\x80\x83\x01\x84a*\xE1V[\x96\x95PPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a+TWa+Ta*0V[\x825\x91P` \x83\x015a+f\x81a*\x80V[\x80\x91PP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a+\x86Wa+\x86a*0V[P5\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a+\xA3Wa+\xA3a*0V[PP\x805\x92` \x90\x91\x015\x91PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a+\xE1Wa+\xE1a*0V[\x84Qa+\xEC\x81a*\x80V[\x80\x94PP` \x80\x86\x01Q\x93P`@\x80\x87\x01Q\x93P``\x87\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a,eW\x82QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x85\x90R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[\x81\x89\x01\x91P\x89`\x1F\x83\x01\x12a,\xCCW\x82QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x85\x90R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x81\xFD[\x81Q\x81\x81\x11\x15a,\xDEWa,\xDEa+\xB2V[\x83Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a-\x05Wa-\x05a+\xB2V[\x81\x86R\x82\x81R\x8C\x87\x84\x87\x01\x01\x11\x15a-mW\x85QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x88\x90R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R\x93P`\x84\x84\xFD[a-|\x83\x88\x83\x01\x89\x88\x01a*\xBDV[\x99\x9C\x98\x9BP\x96\x99PPPPPPPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x07}Wa\x07}a-\x8DV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a-\xDBWa-\xDBa-\xB6V[P\x04\x90V[\x80\x82\x01\x80\x82\x11\x15a\x07}Wa\x07}a-\x8DV[`\0`\x01\x82\x01a.\x05Wa.\x05a-\x8DV[P`\x01\x01\x90V[`\0` \x82\x84\x03\x12\x15a.!Wa.!a*0V[PQ\x91\x90PV[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a.HWa.Ha-\x8DV[P\x92\x91PPV[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15a.kWa.ka-\x8DV[\x81\x81\x05\x83\x14\x82\x15\x17a\x07}Wa\x07}a-\x8DV[`\0\x82a.\x8EWa.\x8Ea-\xB6V[`\x01`\xFF\x1B\x82\x14`\0\x19\x84\x14\x16\x15a.\xA8Wa.\xA8a-\x8DV[P\x05\x90V[`@\x81R`\0a.\xC0`@\x83\x01\x85a*\xE1V[\x90P\x82` \x83\x01R\x93\x92PPPV[` \x81R`\0a\t\x82` \x83\x01\x84a*\xE1V[`@\x81R`\x19`@\x82\x01R\x7FDEX swap failed with data\0\0\0\0\0\0\0``\x82\x01R`\x80` \x82\x01R`\0a\t\x82`\x80\x83\x01\x84a*\xE1V[`\0`\x01`\xFF\x1B\x82\x01a/>Wa/>a-\x8DV[P`\0\x03\x90V[\x80\x82\x01\x82\x81\x12`\0\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a/eWa/ea-\x8DV[PP\x92\x91PPV\xFETarget contract does not contain`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x163\x17\x90Ua\x01\x0C\x80a\0\x7F`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\0xW`\x005`\xE0\x1C\x80c\x91\xE6\xECB\x14a\0\xDDW[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[`\0Ta\0\xF0\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xF3";
    /// The bytecode of the contract.
    pub static ATOMICV2_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x02'W`\x005`\xE0\x1C\x80c\x98\xD8\x83M\x11a\x01QW\x80c\xD0\xB7\x1B\x1E\x11a\0\xEFW\x80c\xE5$\xF8I\x11a\0\xBEW\x80c\xE5$\xF8I\x14a\x05\x15W\x80c\xEF\xA3Lx\x14a\x05(W\x80c\xF3\xC9s\xCF\x14a\x05;W\x80c\xFA.Y\x94\x14a\x05HWa\x02'V[\x80c\xD0\xB7\x1B\x1E\x14a\x04\xC9W\x80c\xD2L\xE6\xE5\x14a\x04\xDCW\x80c\xD2\xF7&Z\x14a\x04\xEFW\x80c\xD7\x82=\xF7\x14a\x05\x02Wa\x02'V[\x80c\xA1S\x87\x89\x11a\x01+W\x80c\xA1S\x87\x89\x14a\x04/W\x80c\xAE\x97h\xA8\x14a\x04{W\x80c\xB7\xDA+\xAF\x14a\x04\x8EW\x80c\xBD%-\x06\x14a\x04\xB6Wa\x02'V[\x80c\x98\xD8\x83M\x14a\x03\xF6W\x80c\x99\x9B\x93\xAF\x14a\x04\tW\x80c\x9F'\xEFO\x14a\x04\x1CWa\x02'V[\x80cI\xA7\xA2m\x11a\x01\xC9W\x80cw\xE5\xEA\x07\x11a\x01\x98W\x80cw\xE5\xEA\x07\x14a\x03\xBEW\x80c\x85\xB3\x19\xFF\x14a\x03\xD1W\x80c\x8A/\xA5J\x14a\x03\xDAW\x80c\x93e \xC3\x14a\x03\xEDWa\x02'V[\x80cI\xA7\xA2m\x14a\x03mW\x80cdI\xFCW\x14a\x03\x80W\x80cgsB\xCE\x14a\x03\xA2W\x80cr\xB9\x82F\x14a\x03\xB5Wa\x02'V[\x80c5\xA9\x9A\xD0\x11a\x02\x05W\x80c5\xA9\x9A\xD0\x14a\x03\x07W\x80c6yr:\x14a\x03\x1CW\x80c7\xC6\xA4J\x14a\x03/W\x80c8\xD5.\x0F\x14a\x03BWa\x02'V[\x80c\n\xC304\x14a\x02\x8CW\x80c\x18\\\x1A^\x14a\x02\xB8W\x80c-[l\xB9\x14a\x02\xE6W[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x02\x9Fa\x02\x9A6`\x04a*\x8EV[a\x05QV[`@Qa\x02\xAF\x94\x93\x92\x91\x90a+\rV[`@Q\x80\x91\x03\x90\xF3[a\x02\xCBa\x02\xC66`\x04a+>V[a\x061V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x02\xAFV[a\x02\xF9a\x02\xF46`\x04a+qV[a\x07rV[`@Q\x90\x81R` \x01a\x02\xAFV[a\x03\x1Aa\x03\x156`\x04a+qV[a\x07\x83V[\0[a\x02\xF9a\x03*6`\x04a+qV[a\tkV[a\x02\xF9a\x03=6`\x04a+\x8DV[a\tvV[`\x03Ta\x03U\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\xAFV[`\x02Ta\x03U\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\nTa\x03\x92\x90a\x01\0\x90\x04`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x02\xAFV[a\x02\xF9a\x03\xB06`\x04a+qV[a\t\x89V[a\x02\xF9`\x08T\x81V[a\x02\xF9a\x03\xCC6`\x04a*\x8EV[a\t\x94V[a\x02\xF9`\tT\x81V[a\x03\x1Aa\x03\xE86`\x04a+qV[a\n\xCDV[a\x02\xF9`\x06T\x81V[`\x05Ta\x03U\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x04Ta\x03U\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\0Ta\x03U\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x0BT`\x0CT`\rT`\x0ET`\x0FTa\x04L\x94\x93\x92`\xFF\x16\x91\x90\x85V[`@Qa\x02\xAF\x95\x94\x93\x92\x91\x90\x94\x85R` \x85\x01\x93\x90\x93R\x90\x15\x15`@\x84\x01R``\x83\x01R`\x80\x82\x01R`\xA0\x01\x90V[a\x02\xF9a\x04\x896`\x04a+\x8DV[a\x0C\xA9V[a\x04\xA1a\x04\x9C6`\x04a*\x8EV[a\x0C\xB5V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x02\xAFV[a\x02\xF9a\x04\xC46`\x04a+\x8DV[a\x0E\xF5V[a\x02\xF9a\x04\xD76`\x04a+qV[a\x0F\x01V[a\x02\xF9a\x04\xEA6`\x04a+qV[a\x0F\x0CV[`\x01Ta\x03U\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x03\x1Aa\x05\x106`\x04a+qV[a\x0F\x17V[a\x02\xF9a\x05#6`\x04a+\x8DV[a\x10UV[a\x03\x1Aa\x0566`\x04a+qV[a\x10aV[`\nTa\x03\x92\x90`\xFF\x16\x81V[a\x02\xF9`\x07T\x81V[`\x02T`@Qc\x02\xB0\xCC\r`\xE2\x1B\x81R\x83\x15\x15`\x04\x82\x01R`$\x81\x01\x83\x90R`\0\x91\x82\x91\x82\x91``\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\n\xC304\x90`D\x01`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x05\xE5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/n\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x05\xF9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x06!\x91\x90\x81\x01\x90a+\xC8V[\x92\x99\x91\x98P\x96P\x90\x94P\x92PPPV[`\0\x80\x80\x80\x80\x80\x80a\x06G\x89`\x1Ea\x03\xE8a\x10\xB8V[\x90P`\0a\x06Y\x8Aa\x03\xE8`da\x10\xD7V[\x90Pa\x06\x86`@Q\x80`@\x01`@R\x80`\t\x81R` \x01h4\xBA2\xB90\xBA4\xB7\xB7`\xB9\x1B\x81RP\x84a\x11\x05V[\x80\x82\x10\x80\x15a\x06\x95WP`@\x83\x10[\x15a\x07?W`\x02a\x06\xA6\x83\x83a-\xA3V[a\x06\xB0\x91\x90a-\xCCV[a\x06\xBA\x90\x83a-\xE0V[\x94Pa\x06\xE1`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01b\x1BZY`\xEA\x1B\x81RP\x86a\x11\x05V[a\x06\xEB\x89\x86a\t\x94V[\x93Pa\x07\x19`@Q\x80`@\x01`@R\x80`\n\x81R` \x01iderivative`\xB0\x1B\x81RP\x85a\x11JV[`\0\x84\x13\x15a\x07*W\x84\x91Pa\x07-V[P\x83[\x82a\x077\x81a-\xF3V[\x93PPa\x06\x86V[`\x02a\x07K\x82\x84a-\xE0V[a\x07U\x91\x90a-\xCCV[\x97Pa\x07a\x89\x89a\x0C\xB5V[\x98\x9B\x90\x9AP\x97\x98PPPPPPPPV[`\0a\x07}\x82a\x11\x8FV[\x92\x91PPV[`\x02T`@\x80QcB\xC9\x88[`\xE1\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x85\x93\x10\xB6\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\x08\x06W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/n\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x08\x1AW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08>\x91\x90a.\x0CV[`@\x80Q\x82\x81RB` \x82\x01R\x91\x92P\x7F\xD15<h\xE7\x9E\xF7\r\xE8N\xE9\r/\xAC\xF8E\xEC$\x89Q\x16\xD4\xA05\x05\xAAAxZ\xF7\x1FZ\x91\x01`@Q\x80\x91\x03\x90\xA1a\x08\x82\x82a\x13jV[`\nTa\x08\x97\x90a\x01\0\x90\x04`\xFF\x16\x83a\x16\xB3V[`\nT`\x03T`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01Ra\t_\x92`\xFF\x16\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01[` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\t\"W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/n\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\t6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\tZ\x91\x90a.\x0CV[a\x1A\x1BV[a\tga\x1E#V[PPV[`\0a\x07}\x82a mV[`\0a\t\x82\x83\x83a!\x13V[\x93\x92PPPV[`\0a\x07}\x82a!(V[`\0\x80a\t\xA3a'\x10\x84a-\xA3V[\x90P`\0a\t\xB3\x84a'\x10a-\xE0V[\x90Pa\t\xDC`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01dguess`\xD8\x1B\x81RP\x85a\x11\x05V[`\0a\t\xE8\x86\x83a\x0C\xB5V[P\x90P`\0a\t\xF7\x87\x85a\x0C\xB5V[P\x90Pa\n$`@Q\x80`@\x01`@R\x80`\x08\x81R` \x01g\x07\x07&\xF6f\x97EW`\xC4\x1B\x81RP\x83a\x11JV[a\nP`@Q\x80`@\x01`@R\x80`\n\x81R` \x01i897\xB34\xBA\"7\xBB\xB7`\xB1\x1B\x81RP\x82a\x11JV[`@\x80Q\x80\x82\x01\x90\x91R`\n\x81Ridifference`\xB0\x1B` \x82\x01Ra\n\x84\x90a\n\x7F\x83\x85a.(V[a\x11JV[`\0aN a\n\x9Bg\r\xE0\xB6\xB3\xA7d\0\0\x84a.OV[a\n\xADg\r\xE0\xB6\xB3\xA7d\0\0\x86a.OV[a\n\xB7\x91\x90a.(V[a\n\xC1\x91\x90a.\x7FV[\x98\x97PPPPPPPPV[`\x02T`@\x80QcB\xC9\x88[`\xE1\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x85\x93\x10\xB6\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\x0BPW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/n\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0BdW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\x88\x91\x90a.\x0CV[`@\x80Q\x82\x81RB` \x82\x01R\x91\x92P\x7F\xD15<h\xE7\x9E\xF7\r\xE8N\xE9\r/\xAC\xF8E\xEC$\x89Q\x16\xD4\xA05\x05\xAAAxZ\xF7\x1FZ\x91\x01`@Q\x80\x91\x03\x90\xA1a\x0B\xCC\x82a\x13jV[`\nTa\x0B\xE1\x90a\x01\0\x90\x04`\xFF\x16\x83a\x1A\x1BV[`\nT`\x03T`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01Ra\t_\x92`\xFF\x16\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01[` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0ClW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/n\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0C\x80W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xA4\x91\x90a.\x0CV[a\x16\xB3V[`\0a\t\x82\x83\x83a!\xCCV[`\0\x80`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xA05\xB1\xFE`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\rGW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/n\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\r[W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\x7F\x91\x90a.\x0CV[`\x02T`@Qc\x02\xB0\xCC\r`\xE2\x1B\x81R\x87\x15\x15`\x04\x82\x01R`$\x81\x01\x87\x90R\x91\x92P`\0\x91\x82\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\n\xC304\x90`D\x01`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0E\x13W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/n\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0E'W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0EO\x91\x90\x81\x01\x90a+\xC8V[P\x92P\x92P\x92P\x82a\x0E\xA8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FInvalid swap simulation\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x88a\x0E\xB5W\x87a\x0E\xBFV[a\x0E\xBF\x88\x86a!\xCCV[\x90P`\0\x89a\x0E\xD7Wa\x0E\xD2\x84\x87a!\xCCV[a\x0E\xD9V[\x83[\x90Pa\x0E\xE5\x82\x82a.(V[\x9A\x92\x99P\x91\x97PPPPPPPPV[`\0a\t\x82\x83\x83a!\xE1V[`\0a\x07}\x82a!\xF6V[`\0a\x07}\x82a\"_V[a\x0F \x81a\x13jV[`\nTa\x0F5\x90a\x01\0\x90\x04`\xFF\x16\x82a\x1A\x1BV[`\nT`\x03T`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01Ra\x0Fn\x92`\xFF\x16\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01a\x0C\x16V[`\x04\x80T`@Qcp\xA0\x821`\xE0\x1B\x81R0\x92\x81\x01\x92\x90\x92R`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0F\xF0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/n\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x10\x04W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10(\x91\x90a.\x0CV[`\x08\x81\x90U`\x07Ta\x109\x91a.(V[`@QcB\xD5\xD4o`\xE1\x1B\x81R`\x04\x01a\x0E\x9F\x91\x81R` \x01\x90V[`\0a\t\x82\x83\x83a\"\xBBV[a\x10j\x81a\x13jV[`\nTa\x10\x7F\x90a\x01\0\x90\x04`\xFF\x16\x82a\x16\xB3V[`\nT`\x03T`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01Ra\x0Fn\x92`\xFF\x16\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01a\x08\xCCV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x10\xD0W`\0\x80\xFD[\x04\x92\x91PPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x10\xEFW`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[a\tg\x82\x82`@Q`$\x01a\x11\x1B\x92\x91\x90a.\xADV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c-\x83\x9C\xB3`\xE2\x1B\x17\x90Ra\"\xD0V[a\tg\x82\x82`@Q`$\x01a\x11`\x92\x91\x90a.\xADV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\x1ES\x13G`\xE1\x1B\x17\x90Ra\"\xD0V[`\0\x80\x82\x13a\x11\xCCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x0E\x9FV[`\0``a\x11\xD9\x84a\"\xDCV[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[`\x04\x80T`@Qcp\xA0\x821`\xE0\x1B\x81R3\x92\x81\x01\x92\x90\x92R`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x13\xF1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/n\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x14\x05W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14)\x91\x90a.\x0CV[\x90P\x81\x81\x10\x15a\x14VW`@Qc\n\xBEZ\x89`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x81\x01\x83\x90R`D\x01a\x0E\x9FV[`\x04\x80T`@Qcn\xB1v\x9F`\xE1\x1B\x81R3\x92\x81\x01\x92\x90\x92R0`$\x83\x01R`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xDDb\xED>\x90`D\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x14\xE3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/n\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x14\xF7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\x1B\x91\x90a.\x0CV[\x90P\x82\x81\x10\x15a\x15HW`@Qc\xDAV\xD3\xC5`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x81\x01\x84\x90R`D\x01a\x0E\x9FV[`\x04\x80T`@Qc#\xB8r\xDD`\xE0\x1B\x81R3\x92\x81\x01\x92\x90\x92R0`$\x83\x01R`D\x82\x01\x85\x90R`\x01`\x01`\xA0\x1B\x03\x16\x90c#\xB8r\xDD\x90`d\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x15\xD9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/n\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x15\xEDW=`\0\x80>=`\0\xFD[PP`\x04\x80T`@Qcp\xA0\x821`\xE0\x1B\x81R0\x92\x81\x01\x92\x90\x92R`\x01`\x01`\xA0\x1B\x03\x16\x92Pcp\xA0\x821\x91P`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x16sW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/n\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x16\x87W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\xAB\x91\x90a.\x0CV[`\x07UPPPV[\x81\x15a\x18\tW`\x03T`\0T`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x84\x90R\x91\x16\x90c\t^\xA7\xB3\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x17GW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/n\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x17[W=`\0\x80>=`\0\xFD[PP`\0T`\x03T`@Qc\xD0\x04\xF0\xF7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x86\x90R\x91\x16\x92Pc\xD0\x04\xF0\xF7\x91P`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x17\xEDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/n\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x18\x01W=`\0\x80>=`\0\xFD[PPPPPPV[`\x04\x80T`\0T`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x93\x81\x01\x93\x90\x93R`$\x83\x01\x84\x90R\x16\x90c\t^\xA7\xB3\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x18\x99W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/n\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x18\xADW=`\0\x80>=`\0\xFD[PP`\0T`\x04\x80T`@Qc\xD0\x04\xF0\xF7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x92\x81\x01\x92\x90\x92R`$\x82\x01\x86\x90R\x90\x91\x16\x92Pc\xD0\x04\xF0\xF7\x91P`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x19CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/n\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x19WW=`\0\x80>=`\0\xFD[PP`\x03T`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x92Pcp\xA0\x821\x91P`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x19\xDCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/n\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x19\xF0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A\x14\x91\x90a.\x0CV[`\x06UPPV[\x81\x15a\x1A\xCCW`\x03T`\x01T`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x84\x90R\x91\x16\x90c\t^\xA7\xB3\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1A\xAFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/n\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x1A\xC3W=`\0\x80>=`\0\xFD[PPPPa\x1BuV[`\x04\x80T`\x01T`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x93\x81\x01\x93\x90\x93R`$\x83\x01\x84\x90R\x16\x90c\t^\xA7\xB3\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1B\\W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/n\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x1BpW=`\0\x80>=`\0\xFD[PPPP[`\x02T`@Qc\x02\xB0\xCC\r`\xE2\x1B\x81R\x83\x15\x15`\x04\x82\x01R`$\x81\x01\x83\x90R`\0\x91\x82\x91\x82\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\n\xC304\x90`D\x01`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x1C\x08W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/n\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x1C\x1CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1CD\x91\x90\x81\x01\x90a+\xC8V[\x93P\x93P\x93P\x93P\x83a\x1CrW\x83\x83\x83\x83`@Qc\x03\x14\xE6#`\xE3\x1B\x81R`\x04\x01a\x0E\x9F\x94\x93\x92\x91\x90a+\rV[`\x01T`@Qc1>\xEA\xB5`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cb}\xD5j\x90a\x1C\xA2\x90\x84\x90`\x04\x01a.\xCFV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1C\xFAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/n\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x92PPP\x80\x15a\x1D\x0BWP`\x01[a\x1DZW=\x80\x80\x15a\x1D9W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x1D>V[``\x91P[P\x80`@Qcg\xA1k\x8D`\xE1\x1B\x81R`\x04\x01a\x0E\x9F\x91\x90a.\xE2V[\x85a\x18\x01W`\x03T`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x1D\xE0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/n\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x1D\xF4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E\x18\x91\x90a.\x0CV[`\x06UPPPPPPV[`\x04\x80T`@Qcp\xA0\x821`\xE0\x1B\x81R0\x92\x81\x01\x92\x90\x92R`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x1E\xA5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/n\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x1E\xB9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E\xDD\x91\x90a.\x0CV[`\x08\x81\x90U`\x07T\x11\x15a\x1FhW\x7F\xB6[.\x08]}\x04\x0C1?}N\x1A\xC9\x0FY7\x02o\xEEI~\x0E$\xA7\xEF\xF1jU\xE1\xC5\xEA`\x08T`\x07Ta\x1F\x1C\x91\x90a-\xA3V[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xA1`\x07T`\x08Ta\x1F=\x81\x83a.(V[`@Qc\xB1n7\x83`\xE0\x1B\x81R`\x04\x81\x01\x93\x90\x93R`$\x83\x01\x91\x90\x91R\x19`D\x82\x01R`d\x01a\x0E\x9FV[`\0`\x07T`\x08Ta\x1Fz\x91\x90a-\xA3V[\x90P\x80`\t`\0\x82\x82Ta\x1F\x8E\x91\x90a-\xE0V[\x90\x91UPP`@Q\x81\x81R\x7F5}\x90_\x181 \x97\x97\xDFMU\xD7\x9C\\[\xF1\xD9\xF71\x1C\x97j\xFD\x05\xE1=\x88\x1E\xAB\x9B\xC8\x90` \x01`@Q\x80\x91\x03\x90\xA1`\x04\x80T`\x08T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3\x93\x81\x01\x93\x90\x93R`$\x83\x01R`\x01`\x01`\xA0\x1B\x03\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a RW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a/n\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a fW=`\0\x80>=`\0\xFD[PPPPPV[`\0g\x06\xF0[Y\xD3\xB2\0\0\x82\x03a \x86WP`\0\x91\x90PV[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x12a \xAEW`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a \xCFW`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a \xDC\x83`\x02a.OV[\x90P`\0a \xE9\x82a#\x84V[\x90P`\0a \xFFg\x13\xA0K\xBD\xFD\xC9\xBE\x88\x83a&\x02V[\x90Pa!\n\x81a/)V[\x95\x94PPPPPV[`\0a\t\x82\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x10\xB8V[`\xB5\x81`\x01`\x88\x1B\x81\x10a!AW`@\x91\x90\x91\x1B\x90`\x80\x1C[i\x01\0\0\0\0\0\0\0\0\0\x81\x10a!]W` \x91\x90\x91\x1B\x90`@\x1C[e\x01\0\0\0\0\0\x81\x10a!uW`\x10\x91\x90\x91\x1B\x90` \x1C[c\x01\0\0\0\x81\x10a!\x8BW`\x08\x91\x90\x91\x1B\x90`\x10\x1C[b\x01\0\0\x01\x02`\x12\x1C\x80\x82\x04\x01`\x01\x90\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x90\x81\x90\x04\x81\x11\x90\x03\x90V[`\0a\t\x82\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x10\xD7V[`\0a\t\x82\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x10\xD7V[`\0\x80g\x13\xA0K\xBD\xFD\xC9\xBE\x88a\"\x14g\r\xE0\xB6\xB3\xA7d\0\0\x85a.OV[a\"\x1E\x91\x90a.\x7FV[\x90P`\0a\"+\x82a/)V[\x90P`\0a\"8\x82a&\x17V[\x90Pg\x1B\xC1mgN\xC8\0\0a\"Ug\r\xE0\xB6\xB3\xA7d\0\0\x83a.OV[a!\n\x91\x90a.\x7FV[`\0\x80g\x1B\xC1mgN\xC8\0\0\x83a\"u\x81a/)V[a\"\x7F\x91\x90a.OV[a\"\x89\x91\x90a.\x7FV[\x90Pa\"\x94\x81a'\xFBV[\x90Pg\"\xC9U\"\x95T\xC1\xB6a\"\xB1g\r\xE0\xB6\xB3\xA7d\0\0\x83a.OV[a\t\x82\x91\x90a.\x7FV[`\0a\t\x82\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x10\xB8V[a\"\xD9\x81a)\xA4V[PV[`\0\x80\x82\x11a#\x19W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x0E\x9FV[P`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x82\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[`\0\x80\x82\x12\x80a#\x9BWPg\x1B\xC1mgN\xC8\0\0\x82\x13[\x15a#\xB9W`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a#\xDAW`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81g\x1B\xC1mgN\xC8\0\0\x03a$\x02W`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x15a$\rW\x91\x90PV[`\0g\r\xE0\xB6\xB3\xA7d\0\0\x83\x12a$5Wa$0\x83g\x1B\xC1mgN\xC8\0\0a.(V[a$7V[\x82[\x90P`\0a$M\x82g\x1B\xC1mgN\xC8\0\0a)\xC5V[\x90P\x80`\0\x03a$pW`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a${\x82a\x11\x8FV[\x90P`\0c;\x9A\xCA\0a$\xA6a$\xA1a$\x9Bg\x1B\xC1mgN\xC8\0\0a/)V[\x85a&\x02V[a!(V[a$\xB0\x91\x90a.OV[\x90P`\0\x80a$\xC7\x83g\x03\xC1f\\z\xAB \0a&\x02V[a$\xD9\x90g \x05\xFEO&\x8E\xA0\0a/EV[\x90P`\0a%\t\x84a$\xF2\x86f\x9F2u$b\xA0\0a&\x02V[a%\x04\x90g\r\xC5R\x7Fd, \0a/EV[a&\x02V[a%\x1B\x90g\r\xE0\xB6\xB3\xA7d\0\0a/EV[\x90Pa%?g\t\xD0(\xCCo _\xFF\x19\x85a%5\x85\x85a)\xC5V[a%\x04\x91\x90a.(V[\x92PPP`\0[`\x02\x81\x10\x15a%\xDAW`\0\x86a%[\x84a&\x17V[a%e\x91\x90a.(V[\x90P`\0a%s\x84\x85a&\x02V[a%|\x90a/)V[\x90P`\0a%\x89\x82a'\xFBV[\x90P`\0a%\x97\x86\x85a&\x02V[a%\xA9g\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x84a&\x02V[a%\xB3\x91\x90a.(V[\x90Pa%\xBF\x84\x82a)\xC5V[a%\xC9\x90\x87a/EV[\x95P\x84`\x01\x01\x94PPPPPa%FV[g\r\xE0\xB6\xB3\xA7d\0\0\x88\x12a%\xF7Wa%\xF2\x82a/)V[a\n\xC1V[P\x96\x95PPPPPPV[`\0a\t\x82\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a)\xD6V[`\0\x81`\0\x03a&0WPg\r\xE0\xB6\xB3\xA7d\0\0\x91\x90PV[gV\x98\xEE\xF0fp\0\0\x82\x12a&GWP`\0\x91\x90PV[a&XgV\x98\xEE\xF0fp\0\0a/)V[\x82\x13a&mWPg\x1B\xC1mgN\xC8\0\0\x91\x90PV[`\0a&x\x83a)\xF5V[\x90P`\0a&\xB1g\r\xE0\xB6\xB3\xA7d\0\0a&\x9A\x84g\x1B\xC1mgN\xC8\0\0a!\x13V[a&\xAC\x90g\r\xE0\xB6\xB3\xA7d\0\0a/EV[a)\xC5V[\x90P`\0\x80\x82a'\r\x81a&\xFA\x81a&\xE8\x81a&\xD5\x81g\x02_\x0F\xE1\x05\xA3\x14\0a&\x02V[a%\x04\x90g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19a/EV[a%\x04\x90g\x14\xA8EL\x19\xE1\xAC\0a/EV[a%\x04\x90g\x0F\xC1\x0E\x01W\x82w\xFF\x19a/EV[a'\x1F\x90g\x03\xDE\xBD\x08;\x8C|\0a/EV[\x91P\x83\x90Pa'\x87\x81a'u\x81a'c\x81a'Q\x81a'>\x81\x8Ba&\x02V[a%\x04\x90g\x02\x95\xD4\0\xEA2W\xFF\x19a/EV[a%\x04\x90g\x01W\xD8\xB2\xEC\xC7\x08\0a/EV[a%\x04\x90g\x051\n\xA7\xD5!0\0a/EV[a%\x04\x90g\r\xE0\xCC=\x15a\0\0a/EV[\x91P\x81g\x11\x90\0\xAB\x10\x0F\xFC\0a'\x9D\x87\x88a&\x02V[a'\xA9\x90`\0\x19a.OV[a'\xB3\x91\x90a.(V[a'\xBD\x91\x90a/EV[\x92PP`\0a'\xCB\x83a'\xFBV[\x90P`\0a'\xD9\x85\x83a&\x02V[\x90P`\0\x88\x12a'\xE9W\x80a\n\xC1V[a\n\xC1\x81g\x1B\xC1mgN\xC8\0\0a.(V[`\0h\x02H\xCE6\xA7\x0C\xB2k>\x19\x82\x13a(\x16WP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a(]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x01a\x0E\x9FV[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05`\x01`_\x1B\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1Dm\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1Dn\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[\x80Qjconsole.log` \x83\x01`\0\x80\x84\x83\x85Z\xFAPPPPPV[`\0a\t\x82\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x05\x85\x14\x17\x16a)\xEEW`\0\x80\xFD[\x05\x92\x91PPV[`\0`\x01`\xFF\x1B\x82\x03a*\x1BW`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x12\x15a*,WP\x19`\x01\x01\x90V[P\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[\x80\x15\x15\x81\x14a\"\xD9W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a*\xA4Wa*\xA4a*0V[\x825a*\xAF\x81a*\x80V[\x94` \x93\x90\x93\x015\x93PPPV[`\0[\x83\x81\x10\x15a*\xD8W\x81\x81\x01Q\x83\x82\x01R` \x01a*\xC0V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra*\xF9\x81` \x86\x01` \x86\x01a*\xBDV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[\x84\x15\x15\x81R\x83` \x82\x01R\x82`@\x82\x01R`\x80``\x82\x01R`\0a+4`\x80\x83\x01\x84a*\xE1V[\x96\x95PPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a+TWa+Ta*0V[\x825\x91P` \x83\x015a+f\x81a*\x80V[\x80\x91PP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a+\x86Wa+\x86a*0V[P5\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a+\xA3Wa+\xA3a*0V[PP\x805\x92` \x90\x91\x015\x91PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a+\xE1Wa+\xE1a*0V[\x84Qa+\xEC\x81a*\x80V[\x80\x94PP` \x80\x86\x01Q\x93P`@\x80\x87\x01Q\x93P``\x87\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a,eW\x82QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x85\x90R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[\x81\x89\x01\x91P\x89`\x1F\x83\x01\x12a,\xCCW\x82QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x85\x90R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x81\xFD[\x81Q\x81\x81\x11\x15a,\xDEWa,\xDEa+\xB2V[\x83Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a-\x05Wa-\x05a+\xB2V[\x81\x86R\x82\x81R\x8C\x87\x84\x87\x01\x01\x11\x15a-mW\x85QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x88\x90R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R\x93P`\x84\x84\xFD[a-|\x83\x88\x83\x01\x89\x88\x01a*\xBDV[\x99\x9C\x98\x9BP\x96\x99PPPPPPPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x07}Wa\x07}a-\x8DV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a-\xDBWa-\xDBa-\xB6V[P\x04\x90V[\x80\x82\x01\x80\x82\x11\x15a\x07}Wa\x07}a-\x8DV[`\0`\x01\x82\x01a.\x05Wa.\x05a-\x8DV[P`\x01\x01\x90V[`\0` \x82\x84\x03\x12\x15a.!Wa.!a*0V[PQ\x91\x90PV[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a.HWa.Ha-\x8DV[P\x92\x91PPV[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15a.kWa.ka-\x8DV[\x81\x81\x05\x83\x14\x82\x15\x17a\x07}Wa\x07}a-\x8DV[`\0\x82a.\x8EWa.\x8Ea-\xB6V[`\x01`\xFF\x1B\x82\x14`\0\x19\x84\x14\x16\x15a.\xA8Wa.\xA8a-\x8DV[P\x05\x90V[`@\x81R`\0a.\xC0`@\x83\x01\x85a*\xE1V[\x90P\x82` \x83\x01R\x93\x92PPPV[` \x81R`\0a\t\x82` \x83\x01\x84a*\xE1V[`@\x81R`\x19`@\x82\x01R\x7FDEX swap failed with data\0\0\0\0\0\0\0``\x82\x01R`\x80` \x82\x01R`\0a\t\x82`\x80\x83\x01\x84a*\xE1V[`\0`\x01`\xFF\x1B\x82\x01a/>Wa/>a-\x8DV[P`\0\x03\x90V[\x80\x82\x01\x82\x81\x12`\0\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a/eWa/ea-\x8DV[PP\x92\x91PPV\xFETarget contract does not contain";
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
        /// Creates a new contract instance with the specified `ethers` client
        /// at `address`. The contract derefs to a `ethers::Contract`
        /// object.
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
        /// Constructs the general purpose `Deployer` instance based on the
        /// provided constructor arguments and sends it. Returns a new
        /// instance of a deployer that returns an instance of this contract
        /// after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the
        ///   argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract
        /// instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the
        /// `greeter.json` artifact.
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
        /// Calls the contract's `XTOY` (0xf3c973cf) function
        pub fn xtoy(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([243, 201, 115, 207], ())
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `YTOX` (0x6449fc57) function
        pub fn ytox(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([100, 73, 252, 87], ())
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `asset` (0x38d52e0f) function
        pub fn asset(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([56, 213, 46, 15], ())
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `calculateProfit` (0xb7da2baf) function
        pub fn calculate_profit(
            &self,
            x_for_y: bool,
            amount_in: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::I256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([183, 218, 43, 175], (x_for_y, amount_in))
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `cdf` (0xd0b71b1e) function
        pub fn cdf(
            &self,
            input: ::ethers::core::types::I256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([208, 183, 27, 30], input)
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `cumulativeProfit` (0x85b319ff) function
        pub fn cumulative_profit(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([133, 179, 25, 255], ())
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `derivativeOfProfit` (0x77e5ea07) function
        pub fn derivative_of_profit(
            &self,
            x_for_y: bool,
            guess: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([119, 229, 234, 7], (x_for_y, guess))
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `divWadDown` (0x37c6a44a) function
        pub fn div_wad_down(
            &self,
            x: ::ethers::core::types::U256,
            y: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([55, 198, 164, 74], (x, y))
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `divWadUp` (0xbd252d06) function
        pub fn div_wad_up(
            &self,
            x: ::ethers::core::types::U256,
            y: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([189, 37, 45, 6], (x, y))
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `exchange` (0xd2f7265a) function
        pub fn exchange(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([210, 247, 38, 90], ())
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `intermediateTokenXBalance` (0x936520c3)
        /// function
        pub fn intermediate_token_x_balance(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([147, 101, 32, 195], ())
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `intermediateTokenYEndBalance` (0x72b98246)
        /// function
        pub fn intermediate_token_y_end_balance(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([114, 185, 130, 70], ())
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `intermediateTokenYStartBalance` (0xfa2e5994)
        /// function
        pub fn intermediate_token_y_start_balance(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([250, 46, 89, 148], ())
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `liquidExchange` (0x9f27ef4f) function
        pub fn liquid_exchange(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([159, 39, 239, 79], ())
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `log` (0x2d5b6cb9) function
        pub fn log(
            &self,
            x: ::ethers::core::types::I256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([45, 91, 108, 185], x)
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `lower_exchange_price` (0x35a99ad0) function
        pub fn lower_exchange_price(
            &self,
            input_y_amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([53, 169, 154, 208], input_y_amount)
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `mulWadDown` (0xe524f849) function
        pub fn mul_wad_down(
            &self,
            x: ::ethers::core::types::U256,
            y: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([229, 36, 248, 73], (x, y))
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `mulWadUp` (0xae9768a8) function
        pub fn mul_wad_up(
            &self,
            x: ::ethers::core::types::U256,
            y: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([174, 151, 104, 168], (x, y))
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `pdf` (0xd24ce6e5) function
        pub fn pdf(
            &self,
            input: ::ethers::core::types::I256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([210, 76, 230, 229], input)
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `ppf` (0x3679723a) function
        pub fn ppf(
            &self,
            input: ::ethers::core::types::I256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([54, 121, 114, 58], input)
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `profitFinder` (0x98d8834d) function
        pub fn profit_finder(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([152, 216, 131, 77], ())
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `quote` (0x999b93af) function
        pub fn quote(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([153, 155, 147, 175], ())
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `raise_exchange_price` (0x8a2fa54a) function
        pub fn raise_exchange_price(
            &self,
            input_y_amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([138, 47, 165, 74], input_y_amount)
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `searchMaxArbProfit` (0x185c1a5e) function
        pub fn search_max_arb_profit(
            &self,
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
                .method_hash([24, 92, 26, 94], (best_guess, x_for_y))
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `simulateSwap` (0x0ac33034) function
        pub fn simulate_swap(
            &self,
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
                .method_hash([10, 195, 48, 52], (swap_x_in, amount_in))
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `solver` (0x49a7a26d) function
        pub fn solver(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([73, 167, 162, 109], ())
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `sqrt` (0x677342ce) function
        pub fn sqrt(
            &self,
            x: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([103, 115, 66, 206], x)
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `tradePacket` (0xa1538789) function
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
        /// Calls the contract's `try_lower_exchange_price` (0xefa34c78)
        /// function
        pub fn try_lower_exchange_price(
            &self,
            input_y_amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([239, 163, 76, 120], input_y_amount)
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `try_raise_exchange_price` (0xd7823df7)
        /// function
        pub fn try_raise_exchange_price(
            &self,
            input_y_amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([215, 130, 61, 247], input_y_amount)
                .expect("method not found (this should never happen)")
        }
        /// Gets the contract's `Loss` event
        pub fn loss_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LossFilter> {
            self.0.event()
        }
        /// Gets the contract's `Price` event
        pub fn price_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, PriceFilter> {
            self.0.event()
        }
        /// Gets the contract's `Profit` event
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
    /// Custom Error type `AttemptedProfit` with signature
    /// `AttemptedProfit(int256)` and selector `0x85aba8de`
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
    /// Custom Error type `CatastrophicSwapFailure` with signature
    /// `CatastrophicSwapFailure()` and selector `0x3203791f`
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
    /// Custom Error type `DexSwapFailure` with signature
    /// `DexSwapFailure(string,bytes)` and selector `0xcf42d71a`
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
    /// Custom Error type `FindingTradeError` with signature
    /// `FindingTradeError(bytes)` and selector `0x1a439ed1`
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
    /// Custom Error type `Infinity` with signature `Infinity()` and selector
    /// `0x07a02127`
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
    #[etherror(name = "Infinity", abi = "Infinity()")]
    pub struct Infinity;
    /// Custom Error type `InsufficientApprovalY` with signature
    /// `InsufficientApprovalY(uint256,uint256)` and selector `0xda56d3c5`
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
    /// Custom Error type `InsufficientBalanceX` with signature
    /// `InsufficientBalanceX(uint256,uint256)` and selector `0x0295b09c`
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
    /// Custom Error type `InsufficientBalanceY` with signature
    /// `InsufficientBalanceY(uint256,uint256)` and selector `0x0abe5a89`
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
    /// Custom Error type `MaximizingProfitTrade` with signature
    /// `MaximizingProfitTrade(uint256,uint256)` and selector `0x2a369f23`
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
    /// Custom Error type `Min` with signature `Min()` and selector `0x4d2d75b1`
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
    #[etherror(name = "Min", abi = "Min()")]
    pub struct Min;
    /// Custom Error type `NegativeInfinity` with signature `NegativeInfinity()`
    /// and selector `0x8bb56614`
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
    #[etherror(name = "NegativeInfinity", abi = "NegativeInfinity()")]
    pub struct NegativeInfinity;
    /// Custom Error type `NotProfitable` with signature
    /// `NotProfitable(uint256,uint256)` and selector `0x843e30ec`
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
    /// Custom Error type `OutOfBounds` with signature `OutOfBounds()` and
    /// selector `0xb4120f14`
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
    #[etherror(name = "OutOfBounds", abi = "OutOfBounds()")]
    pub struct OutOfBounds;
    /// Custom Error type `SimulatedSwapFailure` with signature
    /// `SimulatedSwapFailure(bool,uint256,uint256,bytes)` and selector
    /// `0x18a73118`
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
    /// Custom Error type `UnprofitableArbitrage` with signature
    /// `UnprofitableArbitrage(uint256,uint256,uint256)` and selector
    /// `0xb16e3783`
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
    /// Container type for all of the contract's custom errors
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
            if let Ok(decoded) = <Infinity as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Infinity(decoded));
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
            if let Ok(decoded) = <Min as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Min(decoded));
            }
            if let Ok(decoded) = <NegativeInfinity as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::NegativeInfinity(decoded));
            }
            if let Ok(decoded) = <NotProfitable as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NotProfitable(decoded));
            }
            if let Ok(decoded) = <OutOfBounds as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::OutOfBounds(decoded));
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
                Self::Infinity(element) => ::ethers::core::abi::AbiEncode::encode(element),
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
                Self::NegativeInfinity(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NotProfitable(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OutOfBounds(element) => ::ethers::core::abi::AbiEncode::encode(element),
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
                _ if selector == <Infinity as ::ethers::contract::EthError>::selector() => true,
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
                _ if selector == <Min as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <NegativeInfinity as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector == <NotProfitable as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector == <OutOfBounds as ::ethers::contract::EthError>::selector() => true,
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
                Self::Infinity(element) => ::core::fmt::Display::fmt(element, f),
                Self::InsufficientApprovalY(element) => ::core::fmt::Display::fmt(element, f),
                Self::InsufficientBalanceX(element) => ::core::fmt::Display::fmt(element, f),
                Self::InsufficientBalanceY(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaximizingProfitTrade(element) => ::core::fmt::Display::fmt(element, f),
                Self::Min(element) => ::core::fmt::Display::fmt(element, f),
                Self::NegativeInfinity(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotProfitable(element) => ::core::fmt::Display::fmt(element, f),
                Self::OutOfBounds(element) => ::core::fmt::Display::fmt(element, f),
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
        Hash,
    )]
    #[ethevent(name = "Profit", abi = "Profit(uint256)")]
    pub struct ProfitFilter {
        pub profit: ::ethers::core::types::U256,
    }
    /// Container type for all of the contract's events
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
    /// Container type for all input parameters for the `XTOY` function with
    /// signature `XTOY()` and selector `0xf3c973cf`
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
    /// Container type for all input parameters for the `YTOX` function with
    /// signature `YTOX()` and selector `0x6449fc57`
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
    /// Container type for all input parameters for the `asset` function with
    /// signature `asset()` and selector `0x38d52e0f`
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
    /// Container type for all input parameters for the `calculateProfit`
    /// function with signature `calculateProfit(bool,uint256)` and selector
    /// `0xb7da2baf`
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
    #[ethcall(name = "calculateProfit", abi = "calculateProfit(bool,uint256)")]
    pub struct CalculateProfitCall {
        pub x_for_y: bool,
        pub amount_in: ::ethers::core::types::U256,
    }
    /// Container type for all input parameters for the `cdf` function with
    /// signature `cdf(int256)` and selector `0xd0b71b1e`
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
    #[ethcall(name = "cdf", abi = "cdf(int256)")]
    pub struct CdfCall {
        pub input: ::ethers::core::types::I256,
    }
    /// Container type for all input parameters for the `cumulativeProfit`
    /// function with signature `cumulativeProfit()` and selector `0x85b319ff`
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
    /// Container type for all input parameters for the `derivativeOfProfit`
    /// function with signature `derivativeOfProfit(bool,uint256)` and selector
    /// `0x77e5ea07`
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
    #[ethcall(name = "derivativeOfProfit", abi = "derivativeOfProfit(bool,uint256)")]
    pub struct DerivativeOfProfitCall {
        pub x_for_y: bool,
        pub guess: ::ethers::core::types::U256,
    }
    /// Container type for all input parameters for the `divWadDown` function
    /// with signature `divWadDown(uint256,uint256)` and selector `0x37c6a44a`
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
    #[ethcall(name = "divWadDown", abi = "divWadDown(uint256,uint256)")]
    pub struct DivWadDownCall {
        pub x: ::ethers::core::types::U256,
        pub y: ::ethers::core::types::U256,
    }
    /// Container type for all input parameters for the `divWadUp` function with
    /// signature `divWadUp(uint256,uint256)` and selector `0xbd252d06`
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
    #[ethcall(name = "divWadUp", abi = "divWadUp(uint256,uint256)")]
    pub struct DivWadUpCall {
        pub x: ::ethers::core::types::U256,
        pub y: ::ethers::core::types::U256,
    }
    /// Container type for all input parameters for the `exchange` function with
    /// signature `exchange()` and selector `0xd2f7265a`
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
    /// Container type for all input parameters for the
    /// `intermediateTokenXBalance` function with signature
    /// `intermediateTokenXBalance()` and selector `0x936520c3`
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
    /// Container type for all input parameters for the
    /// `intermediateTokenYEndBalance` function with signature
    /// `intermediateTokenYEndBalance()` and selector `0x72b98246`
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
    /// Container type for all input parameters for the
    /// `intermediateTokenYStartBalance` function with signature
    /// `intermediateTokenYStartBalance()` and selector `0xfa2e5994`
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
    /// Container type for all input parameters for the `liquidExchange`
    /// function with signature `liquidExchange()` and selector `0x9f27ef4f`
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
    /// Container type for all input parameters for the `log` function with
    /// signature `log(int256)` and selector `0x2d5b6cb9`
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
    #[ethcall(name = "log", abi = "log(int256)")]
    pub struct LogCall {
        pub x: ::ethers::core::types::I256,
    }
    /// Container type for all input parameters for the `lower_exchange_price`
    /// function with signature `lower_exchange_price(uint256)` and selector
    /// `0x35a99ad0`
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
    /// Container type for all input parameters for the `mulWadDown` function
    /// with signature `mulWadDown(uint256,uint256)` and selector `0xe524f849`
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
    #[ethcall(name = "mulWadDown", abi = "mulWadDown(uint256,uint256)")]
    pub struct MulWadDownCall {
        pub x: ::ethers::core::types::U256,
        pub y: ::ethers::core::types::U256,
    }
    /// Container type for all input parameters for the `mulWadUp` function with
    /// signature `mulWadUp(uint256,uint256)` and selector `0xae9768a8`
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
    #[ethcall(name = "mulWadUp", abi = "mulWadUp(uint256,uint256)")]
    pub struct MulWadUpCall {
        pub x: ::ethers::core::types::U256,
        pub y: ::ethers::core::types::U256,
    }
    /// Container type for all input parameters for the `pdf` function with
    /// signature `pdf(int256)` and selector `0xd24ce6e5`
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
    #[ethcall(name = "pdf", abi = "pdf(int256)")]
    pub struct PdfCall {
        pub input: ::ethers::core::types::I256,
    }
    /// Container type for all input parameters for the `ppf` function with
    /// signature `ppf(int256)` and selector `0x3679723a`
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
    #[ethcall(name = "ppf", abi = "ppf(int256)")]
    pub struct PpfCall {
        pub input: ::ethers::core::types::I256,
    }
    /// Container type for all input parameters for the `profitFinder` function
    /// with signature `profitFinder()` and selector `0x98d8834d`
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
    /// Container type for all input parameters for the `quote` function with
    /// signature `quote()` and selector `0x999b93af`
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
    /// Container type for all input parameters for the `raise_exchange_price`
    /// function with signature `raise_exchange_price(uint256)` and selector
    /// `0x8a2fa54a`
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
    /// Container type for all input parameters for the `searchMaxArbProfit`
    /// function with signature `searchMaxArbProfit(uint256,bool)` and selector
    /// `0x185c1a5e`
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
    #[ethcall(name = "searchMaxArbProfit", abi = "searchMaxArbProfit(uint256,bool)")]
    pub struct SearchMaxArbProfitCall {
        pub best_guess: ::ethers::core::types::U256,
        pub x_for_y: bool,
    }
    /// Container type for all input parameters for the `simulateSwap` function
    /// with signature `simulateSwap(bool,uint256)` and selector `0x0ac33034`
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
    #[ethcall(name = "simulateSwap", abi = "simulateSwap(bool,uint256)")]
    pub struct SimulateSwapCall {
        pub swap_x_in: bool,
        pub amount_in: ::ethers::core::types::U256,
    }
    /// Container type for all input parameters for the `solver` function with
    /// signature `solver()` and selector `0x49a7a26d`
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
    #[ethcall(name = "solver", abi = "solver()")]
    pub struct SolverCall;
    /// Container type for all input parameters for the `sqrt` function with
    /// signature `sqrt(uint256)` and selector `0x677342ce`
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
    #[ethcall(name = "sqrt", abi = "sqrt(uint256)")]
    pub struct SqrtCall {
        pub x: ::ethers::core::types::U256,
    }
    /// Container type for all input parameters for the `tradePacket` function
    /// with signature `tradePacket()` and selector `0xa1538789`
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
    /// Container type for all input parameters for the
    /// `try_lower_exchange_price` function with signature
    /// `try_lower_exchange_price(uint256)` and selector `0xefa34c78`
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
    /// Container type for all input parameters for the
    /// `try_raise_exchange_price` function with signature
    /// `try_raise_exchange_price(uint256)` and selector `0xd7823df7`
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
    /// Container type for all of the contract's call
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
                <CalculateProfitCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CalculateProfit(decoded));
            }
            if let Ok(decoded) = <CdfCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Cdf(decoded));
            }
            if let Ok(decoded) =
                <CumulativeProfitCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CumulativeProfit(decoded));
            }
            if let Ok(decoded) =
                <DerivativeOfProfitCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DerivativeOfProfit(decoded));
            }
            if let Ok(decoded) = <DivWadDownCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DivWadDown(decoded));
            }
            if let Ok(decoded) = <DivWadUpCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DivWadUp(decoded));
            }
            if let Ok(decoded) = <ExchangeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Exchange(decoded));
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
            if let Ok(decoded) = <LogCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log(decoded));
            }
            if let Ok(decoded) =
                <LowerExchangePriceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::LowerExchangePrice(decoded));
            }
            if let Ok(decoded) = <MulWadDownCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MulWadDown(decoded));
            }
            if let Ok(decoded) = <MulWadUpCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MulWadUp(decoded));
            }
            if let Ok(decoded) = <PdfCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Pdf(decoded));
            }
            if let Ok(decoded) = <PpfCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Ppf(decoded));
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
                <SearchMaxArbProfitCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SearchMaxArbProfit(decoded));
            }
            if let Ok(decoded) = <SimulateSwapCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SimulateSwap(decoded));
            }
            if let Ok(decoded) = <SolverCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Solver(decoded));
            }
            if let Ok(decoded) = <SqrtCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Sqrt(decoded));
            }
            if let Ok(decoded) = <TradePacketCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TradePacket(decoded));
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
                Self::CalculateProfit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Cdf(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CumulativeProfit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DerivativeOfProfit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DivWadDown(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DivWadUp(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Exchange(element) => ::ethers::core::abi::AbiEncode::encode(element),
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
                Self::Log(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LowerExchangePrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MulWadDown(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MulWadUp(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Pdf(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Ppf(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ProfitFinder(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Quote(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RaiseExchangePrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SearchMaxArbProfit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SimulateSwap(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Solver(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Sqrt(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TradePacket(element) => ::ethers::core::abi::AbiEncode::encode(element),
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
                Self::DerivativeOfProfit(element) => ::core::fmt::Display::fmt(element, f),
                Self::DivWadDown(element) => ::core::fmt::Display::fmt(element, f),
                Self::DivWadUp(element) => ::core::fmt::Display::fmt(element, f),
                Self::Exchange(element) => ::core::fmt::Display::fmt(element, f),
                Self::IntermediateTokenXBalance(element) => ::core::fmt::Display::fmt(element, f),
                Self::IntermediateTokenYEndBalance(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IntermediateTokenYStartBalance(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LiquidExchange(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log(element) => ::core::fmt::Display::fmt(element, f),
                Self::LowerExchangePrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::MulWadDown(element) => ::core::fmt::Display::fmt(element, f),
                Self::MulWadUp(element) => ::core::fmt::Display::fmt(element, f),
                Self::Pdf(element) => ::core::fmt::Display::fmt(element, f),
                Self::Ppf(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProfitFinder(element) => ::core::fmt::Display::fmt(element, f),
                Self::Quote(element) => ::core::fmt::Display::fmt(element, f),
                Self::RaiseExchangePrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::SearchMaxArbProfit(element) => ::core::fmt::Display::fmt(element, f),
                Self::SimulateSwap(element) => ::core::fmt::Display::fmt(element, f),
                Self::Solver(element) => ::core::fmt::Display::fmt(element, f),
                Self::Sqrt(element) => ::core::fmt::Display::fmt(element, f),
                Self::TradePacket(element) => ::core::fmt::Display::fmt(element, f),
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
    /// Container type for all return fields from the `XTOY` function with
    /// signature `XTOY()` and selector `0xf3c973cf`
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
    /// Container type for all return fields from the `YTOX` function with
    /// signature `YTOX()` and selector `0x6449fc57`
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
    /// Container type for all return fields from the `asset` function with
    /// signature `asset()` and selector `0x38d52e0f`
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
    /// Container type for all return fields from the `calculateProfit` function
    /// with signature `calculateProfit(bool,uint256)` and selector `0xb7da2baf`
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
    pub struct CalculateProfitReturn(
        pub ::ethers::core::types::I256,
        pub ::ethers::core::types::U256,
    );
    /// Container type for all return fields from the `cdf` function with
    /// signature `cdf(int256)` and selector `0xd0b71b1e`
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
    pub struct CdfReturn {
        pub output: ::ethers::core::types::I256,
    }
    /// Container type for all return fields from the `cumulativeProfit`
    /// function with signature `cumulativeProfit()` and selector `0x85b319ff`
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
    /// Container type for all return fields from the `derivativeOfProfit`
    /// function with signature `derivativeOfProfit(bool,uint256)` and selector
    /// `0x77e5ea07`
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
    pub struct DerivativeOfProfitReturn(pub ::ethers::core::types::I256);
    /// Container type for all return fields from the `divWadDown` function with
    /// signature `divWadDown(uint256,uint256)` and selector `0x37c6a44a`
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
    pub struct DivWadDownReturn {
        pub z: ::ethers::core::types::U256,
    }
    /// Container type for all return fields from the `divWadUp` function with
    /// signature `divWadUp(uint256,uint256)` and selector `0xbd252d06`
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
    pub struct DivWadUpReturn {
        pub z: ::ethers::core::types::U256,
    }
    /// Container type for all return fields from the `exchange` function with
    /// signature `exchange()` and selector `0xd2f7265a`
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
    /// Container type for all return fields from the
    /// `intermediateTokenXBalance` function with signature
    /// `intermediateTokenXBalance()` and selector `0x936520c3`
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
    /// Container type for all return fields from the
    /// `intermediateTokenYEndBalance` function with signature
    /// `intermediateTokenYEndBalance()` and selector `0x72b98246`
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
    /// Container type for all return fields from the
    /// `intermediateTokenYStartBalance` function with signature
    /// `intermediateTokenYStartBalance()` and selector `0xfa2e5994`
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
    /// Container type for all return fields from the `liquidExchange` function
    /// with signature `liquidExchange()` and selector `0x9f27ef4f`
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
    /// Container type for all return fields from the `log` function with
    /// signature `log(int256)` and selector `0x2d5b6cb9`
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
    pub struct LogReturn {
        pub z: ::ethers::core::types::I256,
    }
    /// Container type for all return fields from the `mulWadDown` function with
    /// signature `mulWadDown(uint256,uint256)` and selector `0xe524f849`
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
    pub struct MulWadDownReturn {
        pub z: ::ethers::core::types::U256,
    }
    /// Container type for all return fields from the `mulWadUp` function with
    /// signature `mulWadUp(uint256,uint256)` and selector `0xae9768a8`
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
    pub struct MulWadUpReturn {
        pub z: ::ethers::core::types::U256,
    }
    /// Container type for all return fields from the `pdf` function with
    /// signature `pdf(int256)` and selector `0xd24ce6e5`
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
    pub struct PdfReturn {
        pub output: ::ethers::core::types::I256,
    }
    /// Container type for all return fields from the `ppf` function with
    /// signature `ppf(int256)` and selector `0x3679723a`
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
    pub struct PpfReturn {
        pub output: ::ethers::core::types::I256,
    }
    /// Container type for all return fields from the `profitFinder` function
    /// with signature `profitFinder()` and selector `0x98d8834d`
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
    /// Container type for all return fields from the `quote` function with
    /// signature `quote()` and selector `0x999b93af`
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
    /// Container type for all return fields from the `searchMaxArbProfit`
    /// function with signature `searchMaxArbProfit(uint256,bool)` and selector
    /// `0x185c1a5e`
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
    pub struct SearchMaxArbProfitReturn {
        pub best_amount_in: ::ethers::core::types::U256,
        pub best_profit: ::ethers::core::types::I256,
        pub expected_price: ::ethers::core::types::U256,
    }
    /// Container type for all return fields from the `simulateSwap` function
    /// with signature `simulateSwap(bool,uint256)` and selector `0x0ac33034`
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
    pub struct SimulateSwapReturn {
        pub valid: bool,
        pub estimated_out: ::ethers::core::types::U256,
        pub estimated_price: ::ethers::core::types::U256,
        pub payload: ::ethers::core::types::Bytes,
    }
    /// Container type for all return fields from the `solver` function with
    /// signature `solver()` and selector `0x49a7a26d`
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
    pub struct SolverReturn(pub ::ethers::core::types::Address);
    /// Container type for all return fields from the `sqrt` function with
    /// signature `sqrt(uint256)` and selector `0x677342ce`
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
    pub struct SqrtReturn {
        pub z: ::ethers::core::types::U256,
    }
    /// Container type for all return fields from the `tradePacket` function
    /// with signature `tradePacket()` and selector `0xa1538789`
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
}
