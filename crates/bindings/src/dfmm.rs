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
                    ::std::borrow::ToOwned::to_owned("getNextLiquidity"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getNextLiquidity"),
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
                    ::std::borrow::ToOwned::to_owned("getParams"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getParams"),
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
                    ::std::borrow::ToOwned::to_owned("getSwapConstant"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getSwapConstant"),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("internalPrice"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("internalPrice"),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("simulateSwap"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
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
                                    name: ::std::borrow::ToOwned::to_owned("swapData"),
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
                    ::std::borrow::ToOwned::to_owned("source"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("source"),
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
                    ::std::borrow::ToOwned::to_owned("LogParams"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("LogParams"),
                            inputs: ::std::vec![
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
                    ::std::borrow::ToOwned::to_owned("Min"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("Min"),
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
    const __BYTECODE: &[u8] = b"`\x80`@R`\x01\x80U4\x80\x15b\0\0bW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`@Qb\0M\x018\x03\x80b\0M\x01\x839\x81\x01`@\x81\x90Rb\0\0\x85\x91b\0\x01>V[`\x02\x80T`\x01`\x01`\xA0\x1B\x03\x80\x86\x16`\x01`\x01`\xA0\x1B\x03\x19\x92\x83\x16\x17\x90\x92U`\x03\x80T\x92\x85\x16\x92\x90\x91\x16\x91\x90\x91\x17\x90U`@Q\x81\x90b\0\0\xC5\x90b\0\x01\x13V[\x90\x81R` \x01`@Q\x80\x91\x03\x90`\0\xF0\x80\x15\x80\x15b\0\0\xE8W=`\0\x80>=`\0\xFD[P`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UPb\0\x01\xCA\x91PPV[a1Q\x80b\0\x1B\xB0\x839\x01\x90V[\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x019W`\0\x80\xFD[\x91\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15b\0\x01\x9FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[b\0\x01\xAA\x84b\0\x01!V[\x92Pb\0\x01\xBA` \x85\x01b\0\x01!V[\x91P`@\x84\x01Q\x90P\x92P\x92P\x92V[a\x19\xD6\x80b\0\x01\xDA`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x01XW`\x005`\xE0\x1C\x80cb}\xD5j\x11a\0\xEFW\x80c\xB7\xD1\x9F\xC4\x11a\0\xBEW\x80c\xB7\xD1\x9F\xC4\x14a\x02\xE6W\x80c\xCF0\x90\x12\x14a\x02\xF9W\x80c\xEB\xAD\xEF\x01\x14a\x03\x02W\x80c\xF0e\x95\x7F\x14a\x03\x10W\x80c\xF3\xA8\xEF\xE3\x14a\x03\x19Wa\x01XV[\x80cb}\xD5j\x14a\x02\x96W\x80cg\xE8(\xBF\x14a\x02\xABW\x80cp\xA0\x821\x14a\x02\xBEW\x80c\x85\x93\x10\xB6\x14a\x02\xDEWa\x01XV[\x80cC\xC8\x85\xBA\x11a\x01+W\x80cC\xC8\x85\xBA\x14a\x023W\x80cM\xDFG\xD4\x14a\x02WW\x80c^\"}X\x14a\x02\x85W\x80c^aZk\x14a\x02\x8EWa\x01XV[\x80c\n\xC304\x14a\x01\xBDW\x80c\x15w\x0F\x92\x14a\x01\xE9W\x80c\x16\xDC\x16[\x14a\x02\0W\x80c\x19c\x824\x14a\x02+W[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x01\xD0a\x01\xCB6`\x04a\x14{V[a\x03!V[`@Qa\x01\xE0\x94\x93\x92\x91\x90a\x14\xFAV[`@Q\x80\x91\x03\x90\xF3[a\x01\xF2`\x06T\x81V[`@Q\x90\x81R` \x01a\x01\xE0V[`\x02Ta\x02\x13\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\xE0V[a\x01\xF2a\x03\xFDV[`\0Ta\x02G\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x01\xE0V[a\x02ja\x02e6`\x04a\x15\x84V[a\x04\xF8V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x01\xE0V[a\x01\xF2`\x05T\x81V[a\x02ja\x08\"V[a\x02\xA9a\x02\xA46`\x04a\x15\x84V[a\x08\xF5V[\0[`\0Ta\x02\x13\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01\xF2a\x02\xCC6`\x04a\x16\xA9V[`\x07` R`\0\x90\x81R`@\x90 T\x81V[a\x01\xF2a\x0C!V[`\x03Ta\x02\x13\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01\xF2`\x01T\x81V[`\x04T`\x05T`\x06Ta\x02jV[a\x01\xF2`\x04T\x81V[a\x01\xF2a\x0C\xEEV[`\0\x80T`@Qc\x02\xB0\xCC\r`\xE2\x1B\x81R\x84\x15\x15`\x04\x82\x01R`$\x81\x01\x84\x90R\x82\x91\x82\x91``\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\n\xC304\x90`D\x01`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x03\xB1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x19\xB6\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x03\xC5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x03\xED\x91\x90\x81\x01\x90a\x16\xF2V[\x92\x99\x91\x98P\x96P\x90\x94P\x92PPPV[`\x04T`\x05T`\x06T`@\x80Q` \x81\x01\x94\x90\x94R\x83\x01\x91\x90\x91R``\x82\x01R`\0\x90\x81\x90`\x80\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R`\0Tc\xA1\x9C\xD3\xD1`\xE0\x1B\x83R\x90\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\xA1\x9C\xD3\xD1\x90a\x04d\x90\x84\x90`\x04\x01a\x18\"V[` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x04\xBAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x19\xB6\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x04\xCEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xF2\x91\x90a\x185V[\x91PP\x90V[`\0\x80`\0`\x01T`\x01\x14a\x05=W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x06`$\x82\x01Re\x1B\x1B\xD8\xDA\xD9Y`\xD2\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0`\x01\x81\x90U\x80T`@Qc\x13w\xD1\xF5`\xE2\x1B\x81R\x82\x91\x82\x91\x82\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cM\xDFG\xD4\x90a\x05}\x90\x8D\x90\x8D\x90`\x04\x01a\x18QV[`\xA0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x05\xD5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x19\xB6\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x05\xE9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\r\x91\x90a\x18\x80V[\x94P\x94P\x94P\x94P\x94P\x84a\x06KW`\0\x84\x12a\x06)\x85a\r4V[`@Qcw`m)`\xE1\x1B\x81R\x91\x15\x15`\x04\x83\x01R`$\x82\x01R`D\x01a\x054V[`\0\x80T`\xFF`\xA0\x1B\x19\x16`\x01`\xA0\x1B\x17\x81U`\x04\x84\x81U`\x05\x84\x90U`\x06\x83\x90U3\x80\x83R`\x07` R`@\x92\x83\x90 \x84\x90U`\x02T\x92Qc#\xB8r\xDD`\xE0\x1B\x81R\x91\x82\x01R0`$\x82\x01R`D\x81\x01\x85\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90`d\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x07\x0CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x19\xB6\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x07 W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07D\x91\x90a\x18\xCCV[P`\x03T`@Qc#\xB8r\xDD`\xE0\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90`d\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x07\xD5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x19\xB6\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x07\xE9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\r\x91\x90a\x18\xCCV[P`\x01\x80U\x91\x99\x90\x98P\x90\x96P\x94PPPPPV[`\0\x80`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c^aZk`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01```@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x08\xB2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x19\xB6\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x08\xC6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xEA\x91\x90a\x18\xECV[\x92P\x92P\x92P\x90\x91\x92V[`\x01T`\x01\x14a\t0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x06`$\x82\x01Re\x1B\x1B\xD8\xDA\xD9Y`\xD2\x1B`D\x82\x01R`d\x01a\x054V[`\0`\x01\x81\x90UT`\x01`\xA0\x1B\x90\x04`\xFF\x16a\t\x80W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rn\x1B\x9B\xDD\x08\x1A[\x9A]\x1AX[\x1A^\x99Y`\x8A\x1B`D\x82\x01R`d\x01a\x054V[`\0\x80T`@Qc\xC1nP\xEF`\xE0\x1B\x81R\x82\x91\x82\x91\x82\x91\x82\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xC1nP\xEF\x90a\t\xBB\x90\x8B\x90\x8B\x90`\x04\x01a\x18QV[`\xC0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\n\x11W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x19\xB6\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\n%W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\nI\x91\x90a\x19\x1DV[\x95P\x95P\x95P\x95P\x95P\x95P\x85a\ngW`\0\x85\x12a\x06)\x86a\r4V[`\x06\x81\x90U`\0\x80\x80\x80a\n{\x87\x87a\roV[`\0\x80T`@\x80Qc^aZk`\xE0\x1B\x81R\x90Q\x96\x9AP\x94\x98P\x92\x96P\x90\x94P3\x93`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x92\x8C\x92\x8C\x92\x8C\x92\x90\x91\x82\x91\x82\x91\x88\x91c^aZk\x91`\x04\x80\x83\x01\x92``\x92\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\x0B\x1AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x19\xB6\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0B.W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0BR\x91\x90a\x18\xECV[`@\x80Q\x84\x81R` \x81\x01\x84\x90R\x90\x81\x01\x82\x90R``\x81\x01\x89\x90R`\x80\x81\x01\x88\x90R\x92\x95P\x90\x93P\x91P\x7F\xC52.\x1Bh\xAE\x0C\xE0J\xA8\xF2\x95\xAE\xC0\xC4\xF0\xF1\xDE\xDE\xB5\xC3\xB3\xF6\x0CE\xF86\xD2(d/\xEC\x90`\xA0\x01`@Q\x80\x91\x03\x90\xA1`@\x80Q`\x01`\x01`\xA0\x1B\x03\x89\x81\x16\x82R` \x82\x01\x8D\x90R\x91\x81\x01\x8B\x90R``\x81\x01\x88\x90R\x81\x8D\x16\x91\x8E\x81\x16\x91\x90\x8B\x16\x90\x7F\xD4\xDB\xFA\xA7\xC0\xFEQ\xEE\xE2J\x1C\xDC\xAF\xD07\x8C\xD3e,\xDB\xA75\xDF\xC4\xB0a\xAAU\xC2\xA4X\xA5\x90`\x80\x01`@Q\x80\x91\x03\x90\xA4PP`\x01\x80UPPPPPPPPPPPPPPPPPPV[`\0\x80T`\x04\x80T`\x06T`@Qc\\x\x05\xB9`\xE1\x1B\x81R\x92\x83\x01\x91\x90\x91R`$\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xB8\xF0\x0Br\x90`D\x01[` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0C\xB1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x19\xB6\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0C\xC5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xE9\x91\x90a\x185V[\x90P\x90V[`\0\x80T`\x04\x80T`\x05T`\x06T`@Qcv\x14\xECs`\xE1\x1B\x81R\x93\x84\x01\x92\x90\x92R`$\x83\x01R`D\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xEC)\xD8\xE6\x90`d\x01a\x0C[V[`\0`\x01`\xFF\x1B\x82\x03a\rZW`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x12\x15a\rkWP\x19`\x01\x01\x90V[P\x90V[`\x04T`\x05T`\0\x91\x82\x91\x82\x91\x82\x91\x81\x88\x11\x15a\x0E\x08W`\x02T`\x03T`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x97P\x16\x94Pa\r\xA6\x82\x89a\x19\x89V[\x93P\x86\x81\x11a\r\xF7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7Finvalid swap: inputs x and y\0\0\0\0`D\x82\x01R`d\x01a\x054V[a\x0E\x01\x87\x82a\x19\x89V[\x92Pa\x0E\x86V[`\x03T`\x02T`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x97P\x16\x94Pa\x0E(\x81\x88a\x19\x89V[\x93P\x87\x82\x11a\x0EyW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7Finvalid swap: inputs x and y\0\0\0\0`D\x82\x01R`d\x01a\x054V[a\x0E\x83\x88\x83a\x19\x89V[\x92P[`\x04\x88\x81U`\x05\x88\x90U`@Qcp\xA0\x821`\xE0\x1B\x81R0\x91\x81\x01\x91\x90\x91R`\0\x90`\x01`\x01`\xA0\x1B\x03\x88\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0F\x12W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x19\xB6\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0F&W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0FJ\x91\x90a\x185V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x88\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0F\xCDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x19\xB6\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0F\xE1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\x05\x91\x90a\x185V[`@Qc#\xB8r\xDD`\xE0\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x88\x90R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x89\x16\x90c#\xB8r\xDD\x90`d\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x10\x94W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x19\xB6\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x10\xA8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\xCC\x91\x90a\x18\xCCV[P`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x86\x90R`\x01`\x01`\xA0\x1B\x03\x88\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x11SW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x19\xB6\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x11gW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\x8B\x91\x90a\x18\xCCV[Pa\x11\x96\x86\x83a\x19\xA2V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x8A\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x12\x13W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x19\xB6\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x12'W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12K\x91\x90a\x185V[\x10\x15a\x12\xA4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: input token transf`D\x82\x01Ra2\xB9`\xF1\x1B`d\x82\x01R`\x84\x01a\x054V[a\x12\xAE\x85\x82a\x19\x89V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x89\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x13+W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x19\xB6\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x13?W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13c\x91\x90a\x185V[\x10\x15a\x13\xBDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7Finvalid swap: output token trans`D\x82\x01Rb32\xB9`\xE9\x1B`d\x82\x01R`\x84\x01a\x054V[PPPP\x92\x95\x91\x94P\x92PV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[\x80\x15\x15\x81\x14a\x14xW`\0\x80\xFD[PV[`\0\x80`@\x83\x85\x03\x12\x15a\x14\x91Wa\x14\x91a\x13\xCAV[\x825a\x14\x9C\x81a\x14jV[\x94` \x93\x90\x93\x015\x93PPPV[`\0[\x83\x81\x10\x15a\x14\xC5W\x81\x81\x01Q\x83\x82\x01R` \x01a\x14\xADV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x14\xE6\x81` \x86\x01` \x86\x01a\x14\xAAV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[\x84\x15\x15\x81R\x83` \x82\x01R\x82`@\x82\x01R`\x80``\x82\x01R`\0a\x15!`\x80\x83\x01\x84a\x14\xCEV[\x96\x95PPPPPPV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x81\xFD[`\0\x80` \x83\x85\x03\x12\x15a\x15\x9AWa\x15\x9Aa\x13\xCAV[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x15\xB5Wa\x15\xB5a\x14\x1AV[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x15\xCCWa\x15\xCCa\x15+V[\x815\x81\x81\x11\x15a\x16/W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x0ENL/$\r\x8C\xAD\xCC\xEE\x8D`\xAB\x1B`d\x82\x01R`\x84\x81\xFD[\x86` \x82\x85\x01\x01\x11\x15a\x16\x97W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rjrray stride`\xA8\x1B`d\x82\x01R\x91P`\x84\x82\xFD[` \x92\x90\x92\x01\x96\x91\x95P\x90\x93PPPPV[`\0` \x82\x84\x03\x12\x15a\x16\xBEWa\x16\xBEa\x13\xCAV[\x815`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x16\xD5W`\0\x80\xFD[\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x17\x0BWa\x17\x0Ba\x13\xCAV[\x84Qa\x17\x16\x81a\x14jV[\x80\x94PP` \x85\x01Q\x92P`@\x85\x01Q\x91P``\x85\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x17FWa\x17Fa\x14\x1AV[\x81\x87\x01\x91P\x87`\x1F\x83\x01\x12a\x17]Wa\x17]a\x15+V[\x81Q\x81\x81\x11\x15a\x17oWa\x17oa\x16\xDCV[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a\x17\x97Wa\x17\x97a\x16\xDCV[\x81`@R\x82\x81R\x8A` \x84\x87\x01\x01\x11\x15a\x18\x02W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R\x93P`\x84\x84\xFD[a\x18\x13\x83` \x83\x01` \x88\x01a\x14\xAAV[\x97\x9A\x96\x99P\x94\x97PPPPPPV[` \x81R`\0a\x16\xD5` \x83\x01\x84a\x14\xCEV[`\0` \x82\x84\x03\x12\x15a\x18JWa\x18Ja\x13\xCAV[PQ\x91\x90PV[` \x81R\x81` \x82\x01R\x81\x83`@\x83\x017`\0\x81\x83\x01`@\x90\x81\x01\x91\x90\x91R`\x1F\x90\x92\x01`\x1F\x19\x16\x01\x01\x91\x90PV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\x18\x9BWa\x18\x9Ba\x13\xCAV[\x85Qa\x18\xA6\x81a\x14jV[` \x87\x01Q`@\x88\x01Q``\x89\x01Q`\x80\x90\x99\x01Q\x92\x9A\x91\x99P\x97\x96P\x90\x94P\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x18\xE1Wa\x18\xE1a\x13\xCAV[\x81Qa\x16\xD5\x81a\x14jV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x19\x04Wa\x19\x04a\x13\xCAV[\x83Q\x92P` \x84\x01Q\x91P`@\x84\x01Q\x90P\x92P\x92P\x92V[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a\x199Wa\x199a\x13\xCAV[\x86Qa\x19D\x81a\x14jV[` \x88\x01Q`@\x89\x01Q``\x8A\x01Q`\x80\x8B\x01Q`\xA0\x90\x9B\x01Q\x93\x9C\x92\x9BP\x90\x99\x90\x98P\x96P\x90\x94P\x92PPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x19\x9CWa\x19\x9Ca\x19sV[\x92\x91PPV[\x80\x82\x01\x80\x82\x11\x15a\x19\x9CWa\x19\x9Ca\x19sV\xFETarget contract does not contain`\x80`@R4\x80\x15b\0\0^W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`@Qb\x001Q8\x03\x80b\x001Q\x839\x81\x01`@\x81\x90Rb\0\0\x81\x91b\0\0\xF9V[g\r\xE0\xB6\xB3\xA7d\0\0\x81\x10b\0\0\xF0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7Fswap fee percentage must be less`D\x82\x01Ri than 100%`\xB0\x1B`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[`\0Ub\0\x01^V[`\0` \x82\x84\x03\x12\x15b\0\x01WW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[PQ\x91\x90PV[a/\xE3\x80b\0\x01n`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x02\xC1W`\x005`\xE0\x1C\x80c\x8FG\x04\x0B\x11a\x01\x9EW\x80c\xC1\xE0\x04;\x11a\x01\x10W\x80c\xDB\xAF\x11B\x11a\0\xD4W\x80c\xDB\xAF\x11B\x14a\x06HW\x80c\xE3\xD0\xDC\xA5\x14a\x06[W\x80c\xE4\x93t(\x14a\x06dW\x80c\xEC)\xD8\xE6\x14a\x06wW\x80c\xF9\xC2\x82\x11\x14a\x06\x8AW\x80c\xFF\xB3bf\x14a\x06\x92Wa\x02\xC1V[\x80c\xC1\xE0\x04;\x14a\x06\nW\x80c\xC5)\x87\xCF\x14a\x06\x12W\x80c\xCDd\xAE\xA2\x14a\x06\x1AW\x80c\xCF\xC4\xAFU\x14a\x06-W\x80c\xD7\xF9{\xEF\x14a\x065Wa\x02\xC1V[\x80c\xA6\xD34\x98\x11a\x01bW\x80c\xA6\xD34\x98\x14a\x05SW\x80c\xAF\xDF1\xCD\x14a\x05fW\x80c\xB8\xF0\x0Br\x14a\x05nW\x80c\xBDB-(\x14a\x05\x81W\x80c\xC1e&\x12\x14a\x05\xBFW\x80c\xC1nP\xEF\x14a\x05\xC8Wa\x02\xC1V[\x80c\x8FG\x04\x0B\x14a\x04\xF4W\x80c\x97\x16\xAE\xBB\x14a\x05\x07W\x80c\x99\x87\xFEd\x14a\x05\x1AW\x80c\x9D\x81\x9E\x83\x14a\x05-W\x80c\xA1\x9C\xD3\xD1\x14a\x05@Wa\x02\xC1V[\x80cX\xFAc\xCA\x11a\x027W\x80cme\"\x99\x11a\x01\xFBW\x80cme\"\x99\x14a\x04\x9AW\x80c\x7F\x0EL\x8C\x14a\x04\xA2W\x80c\x84\xC8*&\x14a\x04\xB7W\x80c\x85\xAAEN\x14a\x04\xBFW\x80c\x88;m\xC5\x14a\x04\xCEW\x80c\x8DR\xA1\xFC\x14a\x04\xE1Wa\x02\xC1V[\x80cX\xFAc\xCA\x14a\x04;W\x80c^aZk\x14a\x04CW\x80cd\x17\xD4\xB5\x14a\x04fW\x80cf\x8F\x90U\x14a\x04yW\x80cj\x14`$\x14a\x04\x8BWa\x02\xC1V[\x80c']g\xC8\x11a\x02\x89W\x80c']g\xC8\x14a\x03\xB5W\x80c.-yi\x14a\x03\xC4W\x80c>\x1E3\x92\x14a\x03\xD9W\x80c@\xB41i\x14a\x03\xE2W\x80cM\xDFG\xD4\x14a\x03\xEBW\x80cV\x08\xBE\xA1\x14a\x04(Wa\x02\xC1V[\x80c\x06%\xA6#\x14a\x03&W\x80c\n\xC304\x14a\x03LW\x80c\x0F\n\xA3\x95\x14a\x03oW\x80c\x18M0\xBA\x14a\x03\x8FW\x80c\x1D\x9C\xF7\"\x14a\x03\xA2W[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x039a\x0346`\x04a'\xA4V[a\x06\xA5V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x03_a\x03Z6`\x04a'\xE2V[a\x07)V[`@Qa\x03C\x94\x93\x92\x91\x90a(\\V[a\x03\x82a\x03}6`\x04a)\x8BV[a\n\xBBV[`@Qa\x03C\x91\x90a)\xCEV[a\x039a\x03\x9D6`\x04a)\xE1V[a\n\xEDV[a\x039a\x03\xB06`\x04a*\x1AV[a\x0B?V[a\x039g\x1B\xC1mgN\xC8\0\0\x81V[a\x03\xCCa\x0B\x88V[`@Qa\x03C\x91\x90a*6V[a\x039`\nT\x81V[a\x039`\0T\x81V[a\x03\xFEa\x03\xF96`\x04a*\xB0V[a\x0B\xD1V[`@\x80Q\x95\x15\x15\x86R` \x86\x01\x94\x90\x94R\x92\x84\x01\x91\x90\x91R``\x83\x01R`\x80\x82\x01R`\xA0\x01a\x03CV[a\x039a\x0466`\x04a)\xE1V[a\x0C?V[a\x039`\0\x81V[a\x04Ka\x0C\x80V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x03CV[a\x039a\x04t6`\x04a)\x8BV[a\x0C\xA8V[`\x01T`\x02T`\x03Ta\x04K\x92\x91\x90\x83V[a\x039g\r\xE0\xB6\xB3\xA7d\0\0\x81V[a\x039`\x01\x81V[a\x04\xB5a\x04\xB06`\x04a+\xD5V[a\r\rV[\0[a\x039`\n\x81V[a\x039g\x06\xF0[Y\xD3\xB2\0\0\x81V[a\x039a\x04\xDC6`\x04a)\x8BV[a\r\xFEV[a\x04\xB5a\x04\xEF6`\x04a+\xD5V[a\x0E\x8CV[a\x039a\x05\x026`\x04a+\xD5V[a\x0FPV[a\x039a\x05\x156`\x04a+\xFAV[a\x0F\x8AV[a\x04\xB5a\x05(6`\x04a+\xD5V[a\x10\x1BV[a\x039a\x05;6`\x04a)\xE1V[a\x10\xDFV[a\x039a\x05N6`\x04a,*V[a\x11tV[a\x039a\x05a6`\x04a*\x1AV[a\x11\xABV[a\x039a\x11\xD4V[a\x039a\x05|6`\x04a+\xD5V[a\x12DV[a\x03\x82a\x05\x8F6`\x04a-\x18V[`@\x80Q` \x81\x01\x94\x90\x94R\x83\x81\x01\x92\x90\x92R``\x80\x84\x01\x91\x90\x91R\x81Q\x80\x84\x03\x90\x91\x01\x81R`\x80\x90\x92\x01\x90R\x90V[a\x039`\x05T\x81V[a\x05\xDBa\x05\xD66`\x04a,*V[a\x12jV[`@\x80Q\x96\x15\x15\x87R` \x87\x01\x95\x90\x95R\x93\x85\x01\x92\x90\x92R``\x84\x01R`\x80\x83\x01R`\xA0\x82\x01R`\xC0\x01a\x03CV[a\x03\xCCa\x14\xABV[a\x039a\x14\xF6V[a\x039a\x06(6`\x04a)\x8BV[a\x15aV[a\x039a\x15\xA9V[a\x039a\x06C6`\x04a+\xFAV[a\x16\x14V[a\x039a\x06V6`\x04a)\xE1V[a\x16<V[a\x039`\0\x19\x81V[a\x039a\x06r6`\x04a-\x18V[a\x16~V[a\x039a\x06\x856`\x04a-\x18V[a\x16\xFCV[a\x039`Z\x81V[a\x039a\x06\xA06`\x04a-\x18V[a\x175V[`\0\x80a\x06\xB2\x84\x84a\x17\xB4V[\x90P`\0a\x06\xC0\x85\x85a\x0FPV[\x90P`\0a\x06\xCE\x89\x89a\x17\xE9V[\x90Pa\x07\x1C\x87a\x07\x17\x84g\r\xE0\xB6\xB3\xA7d\0\0\x87a\x06\xF4a\x06\xEF\x88\x84a-]V[a\x17\xFEV[a\x06\xFE\x91\x90a-pV[a\x07\x08\x91\x90a-\xB6V[a\x07\x12\x91\x90a-\xE4V[a\x18\x9BV[a\x1ADV[\x99\x98PPPPPPPPPV[`\0\x80`\0```\0\x80`\x003`\x01`\x01`\xA0\x1B\x03\x16c\xEB\xAD\xEF\x01`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01```@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x07\xBEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x07\xD2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\xF6\x91\x90a.\x0BV[\x92P\x92P\x92Pa\x08\x07\x83\x83\x83a\x16\xFCV[`@\x80Q` \x81\x01\x86\x90R\x90\x81\x01\x84\x90R``\x81\x01\x82\x90R\x90\x91P`\0\x90a\x08A\x90`\x80\x01[`@Q` \x81\x83\x03\x03\x81R\x90`@Ra\x11tV[\x90P`\0\x8A\x15a\tiW`\0\x80Ta\x08Z\x90\x8C\x90a\x1ADV[\x90P`\0a\x08r\x87a\x08l\x84\x88a\x1ADV[\x90a\x1AYV[\x90Pa\x08\x7F`\x01\x82a.<V[\x90Pa\x08\x8B\x8C\x88a.<V[\x96Pa\x08\x97\x81\x86a.<V[\x94P\x85a\x08\xA8\x88\x87\x87a\x04ta\x14\xABV[\x96Pa\x08\xB5`\x01\x88a.<V[\x96P\x80\x87\x10a\t\x16W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: y reserve increase`D\x82\x01Rad!`\xF0\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[a\t \x87\x82a-]V[\x93Pa\ta`@Q\x80`@\x01`@R\x80`\x1C\x81R` \x01\x7FEsimated Y reserve to submit\0\0\0\0\x81RP\x88a\x1AnV[PPPa\n9V[`\0\x80Ta\tx\x90\x8C\x90a\x1ADV[\x90P`\0a\t\x8A\x86a\x08l\x84\x88a\x1ADV[\x90Pa\t\x97`\x01\x82a.<V[\x90Pa\t\xA3\x8C\x87a.<V[\x95Pa\t\xAF\x81\x86a.<V[\x94P\x86a\t\xC0\x87\x87\x87a\x06(a\x14\xABV[\x97Pa\t\xCD`\x01\x89a.<V[\x97P\x80\x88\x10a\n)W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: x reserve increase`D\x82\x01Rad!`\xF0\x1B`d\x82\x01R`\x84\x01a\t\rV[a\n3\x88\x82a-]V[\x93PPPP[`@\x80Q` \x81\x01\x87\x90R\x90\x81\x01\x85\x90R``\x81\x01\x84\x90R`\0\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`\0a\ns\x82a\x12jV[PPPPP\x90P`\0a\n\x84a\x14\xABV[\x90P\x81\x84a\n\xA1\x8A\x89\x85`\0\x01Q\x86` \x01Q\x87`@\x01Qa\x06\xA5V[\x85\x9BP\x9BP\x9BP\x9BPPPPPPPPP\x92\x95\x91\x94P\x92PV[``\x84\x84\x84\x84`@Q` \x01a\n\xD4\x94\x93\x92\x91\x90a.OV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x94\x93PPPPV[`\0\x80a\n\xFA\x84\x84a\x0F\x8AV[\x90P`\0a\x0B\x07\x82a\x1A\xB7V[\x90P`\0a\x0B\x14\x82a\x0B?V[\x90Pa\x0B2a\x0B+\x82g\r\xE0\xB6\xB3\xA7d\0\0a-]V[\x88\x90a\x1ADV[\x93PPPP[\x93\x92PPPV[`\0\x80\x82\x12\x15a\x0B\x84W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01RotoUint: negative`\x80\x1B`D\x82\x01R`d\x01a\t\rV[P\x90V[a\x0B\xAC`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[P`@\x80Q``\x81\x01\x82R`\x01T\x81R`\x02T` \x82\x01R`\x03T\x91\x81\x01\x91\x90\x91R\x90V[`\0\x80\x80\x80\x80a\x0B\xE3\x86\x88\x01\x88a)\x8BV[\x80Q`\x01U` \x81\x01Q`\x02U`@\x01Q`\x03U\x91\x94P\x92P\x90Pa\x0C\x06a\x1B V[a\x0C\x19\x83\x83\x83a\x0C\x14a\x14\xABV[a\x1BoV[\x93P\x83a\x0C&`\na.\x83V[\x12\x80\x15a\x0C3WP`\n\x84\x12[\x94P\x92\x95P\x92\x95\x90\x93PV[`\0\x80a\x0CL\x84\x84a\x16\x14V[\x90P`\0a\x0CY\x82a\x1A\xB7V[\x90P`\0a\x0Cf\x82a\x0B?V[\x85Q\x90\x91Pa\x0B2\x90a\x0Cy\x90\x83a\x1CdV[\x88\x90a\x17\xE9V[`\0\x80`\0a\x0C\x8Da\x14\xF6V[a\x0C\x95a\x11\xD4V[a\x0C\x9Da\x15\xA9V[\x92P\x92P\x92P\x90\x91\x92V[\x80Q`\0\x90`\n\x90\x82\x90\x82\x90a\x0C\xBF\x90\x88\x90a\x1ADV[a\x0C\xC9\x91\x90a-]V[\x90Pa\r\x02\x87\x87\x87\x87`@Q` \x01a\x0C\xE5\x94\x93\x92\x91\x90a.OV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83\x83`\x01`Za\x1Cya\x1C\xB4V[\x97\x96PPPPPPPV[B\x81\x11a\r,W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\t\r\x90a.\x9FV[a\r4a\x1D\xC5V[`\0\x82`\x0ET\x11a\rQW`\x0ETa\rL\x90\x84a-]V[a\r_V[\x82`\x0ETa\r_\x91\x90a-]V[\x90Pa\rkB\x83a-]V[a\ru\x90\x82a.\xCAV[`\x11U`\x0F\x83\x90U`\x12\x82\x90U`\x0ET\x7F\x1EK Gg&%\xF4\xDF!\xD4|\xA7\xA0\x0C \x12\xCC\xF7x\xF2w\xBF2(\xDD\xF4Y=o\x84\xAA\x90\x84\x90\x84\x81\x83\x11a\r\xC6W`\x0FT`\x0ETa\r\xC1\x91\x90a-]V[a\r\xD6V[`\x0ET`\x0FTa\r\xD6\x91\x90a-]V[`@\x80Q\x94\x85R` \x85\x01\x93\x90\x93R\x91\x83\x01R``\x82\x01R`\x80\x01`@Q\x80\x91\x03\x90\xA1PPPV[\x80Q`\0\x90\x81\x90a\x0E\x10\x90\x86\x90a\x17\xE9V[\x90P`\0\x81\x87\x11a\x0E+Wa\x0E&\x82`\x01a.<V[a\x0E6V[a\x0E6\x87`\x01a.<V[\x90P`\0k\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x90Pa\x0E\x80\x88\x88\x88\x88`@Q` \x01a\x0Ec\x94\x93\x92\x91\x90a.OV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83\x83`\x01`Za\x1D\xD6a\x1C\xB4V[\x98\x97PPPPPPPPV[B\x81\x11a\x0E\xABW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\t\r\x90a.\x9FV[a\x0E\xB3a\x1E\x06V[`\0\x82`\tT\x11a\x0E\xD0W`\tTa\x0E\xCB\x90\x84a-]V[a\x0E\xDEV[\x82`\tTa\x0E\xDE\x91\x90a-]V[\x90Pa\x0E\xEAB\x83a-]V[a\x0E\xF4\x90\x82a.\xCAV[`\x0CU`\n\x83\x90U`\r\x82\x90U`\tT\x7F$\xBBEgU#\x9Cf@Y\xB5\x19\xE4c\xB4\xE5\xEA\xEF\x05\x08~\x87\xB0\xDF\xED\xF6\x06\x11iR\xAC\xEE\x90\x84\x90\x84\x81\x83\x11a\x0F@W`\nT`\tTa\r\xC1\x91\x90a-]V[`\tT`\nTa\r\xD6\x91\x90a-]V[`\0\x80a\x0Fna\x0Fh\x85g\x1B\xC1mgN\xC8\0\0a\x1E\x17V[\x84a\x1CdV[\x90Pa\x0F\x82g\x06\xF0[Y\xD3\xB2\0\0\x82a\x1CdV[\x94\x93PPPPV[`\0\x80a\x0F\x9F\x83` \x01Q\x84`@\x01Qa\x17\xB4V[\x90P`\0a\x0F\xB0\x84` \x01Qa\x11\xABV[\x90P`\0a\x0F\xD3a\x0F\xCE\x86`\0\x01Q\x88a\x1AY\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x1ECV[\x90P\x80a\x0F\xDF\x81a.\xDEV[\x91PP`\0a\x0F\xFB\x86`@\x01Q\x84a\x1Cd\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x10\x05\x90\x83a.\xFDV[\x90P\x83a\x0E\x80\x82g\r\xE0\xB6\xB3\xA7d\0\0\x83a\x175V[B\x81\x11a\x10:W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\t\r\x90a.\x9FV[a\x10Ba \x1EV[`\0\x82`\x04T\x11a\x10_W`\x04Ta\x10Z\x90\x84a-]V[a\x10mV[\x82`\x04Ta\x10m\x91\x90a-]V[\x90Pa\x10yB\x83a-]V[a\x10\x83\x90\x82a.\xCAV[`\x07U`\x05\x83\x90U`\x08\x82\x90U`\x04T\x7F\xB9hO\xE86\x87_\xAF\x05m\xB2]\xE9\x85S\x859`\xB3&\x08\x8A\x10!\xAF\xA7[\xA1`\x12d\xF5\x90\x84\x90\x84\x81\x83\x11a\x10\xCFW`\x05T`\x04Ta\r\xC1\x91\x90a-]V[`\x04T`\x05Ta\r\xD6\x91\x90a-]V[`\0\x80a\x10\xEC\x84\x84a\x0F\x8AV[\x90P`\0a\x10\xF9\x82a\x1A\xB7V[\x90P`\0a\x11\x06\x82a\x0B?V[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0\x81\x10a\x11_W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7Flx: denominator is zero\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\t\rV[a\x0B2a\x0Cy\x82g\r\xE0\xB6\xB3\xA7d\0\0a-]V[`\0\x80`\0\x80\x84\x80` \x01\x90Q\x81\x01\x90a\x11\x8E\x91\x90a.\x0BV[\x92P\x92P\x92Pa\x11\xA2\x83\x83\x83a\x0C\x14a\x14\xABV[\x95\x94PPPPPV[`\0\x80a\x11\xC0\x83g\x1B\xC1mgN\xC8\0\0a\x1E\x17V[\x90Pa\x0B8g\x06\xF0[Y\xD3\xB2\0\0\x82a\x1CdV[`\0`\x08TB\x10a\x11\xE6WP`\x05T\x90V[`\x05T`\x04T\x11a\x12\x1DW`\x07T`\x06Ta\x12\x01\x90Ba-]V[a\x12\x0B\x91\x90a/%V[`\x04Ta\x12\x18\x91\x90a.<V[\x90P\x90V[`\x07T`\x06Ta\x12-\x90Ba-]V[a\x127\x91\x90a/%V[`\x04Ta\x12\x18\x91\x90a-]V[`\0\x80a\x12Oa\x14\xABV[\x90Pa\x0F\x82\x84\x84\x83`\0\x01Q\x84` \x01Q\x85`@\x01Qa\x06\xA5V[`\0\x80`\0\x80`\0\x80`\0\x80`\x003`\x01`\x01`\xA0\x1B\x03\x16c\xEB\xAD\xEF\x01`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01```@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x13\x01W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x13\x15W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x139\x91\x90a.\x0BV[\x92P\x92P\x92Pa\x13J\x83\x83\x83a\x16\xFCV[\x90P\x89\x80` \x01\x90Q\x81\x01\x90a\x13`\x91\x90a.\x0BV[\x91\x97P\x95P\x93P`\x01\x83\x87\x11\x15a\x13\xB9W`\0a\x13}\x85\x89a-]V[\x90P`\0a\x13\x96`\0T\x83a\x1AD\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa\x13\xA6\x86a\x08l\x83\x87a\x1ADV[a\x13\xB0\x90\x84a.<V[\x92PPPa\x14WV[\x82\x86\x11\x15a\x13\xF6W`\0a\x13\xCD\x84\x88a-]V[\x90P`\0a\x13\xE6`\0T\x83a\x1AD\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa\x13\xA6\x85a\x08l\x83\x87a\x1ADV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`0`$\x82\x01R\x7Finvalid swap: inputs x and y hav`D\x82\x01Roe the same sign!`\x80\x1B`d\x82\x01R`\x84\x01a\t\rV[a\x14a\x82\x86a-\xE4V[\x97Pa\x14q\x84\x84\x84a\x0C\x14a\x14\xABV[a\x14\x7F\x88\x88\x88a\x0C\x14a\x14\xABV[a\x14\x89\x91\x90a-\xE4V[\x98P`\0\x89\x12\x15\x80\x15a\x14\x9CWP\x80\x88\x12\x15[\x99PPPPP\x91\x93\x95P\x91\x93\x95V[a\x14\xCF`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[a\x14\xD7a\x14\xF6V[a\x14\xDFa\x11\xD4V[a\x14\xE7a\x15\xA9V[`@\x84\x01R` \x83\x01R\x81R\x90V[`\0`\rTB\x10a\x15\x08WP`\nT\x90V[`\nT`\tT\x11a\x15:W`\x0CT`\x0BTa\x15#\x90Ba-]V[a\x15-\x91\x90a/%V[`\tTa\x12\x18\x91\x90a.<V[`\x0CT`\x0BTa\x15J\x90Ba-]V[a\x15T\x91\x90a/%V[`\tTa\x12\x18\x91\x90a-]V[`\0`\n\x81a\x15p\x82\x87a-]V[\x90Pa\r\x02\x87\x87\x87\x87`@Q` \x01a\x15\x8C\x94\x93\x92\x91\x90a.OV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83\x83`\x01`Za /a\x1C\xB4V[`\0`\x12TB\x10a\x15\xBBWP`\x0FT\x90V[`\x0FT`\x0ET\x11a\x15\xEDW`\x11T`\x10Ta\x15\xD6\x90Ba-]V[a\x15\xE0\x91\x90a/%V[`\x0ETa\x12\x18\x91\x90a.<V[`\x11T`\x10Ta\x15\xFD\x90Ba-]V[a\x16\x07\x91\x90a/%V[`\x0ETa\x12\x18\x91\x90a-]V[`\0a\x16(\x82` \x01Q\x83`@\x01Qa\x17\xB4V[a\x162\x84\x84a\x0F\x8AV[a\x0B8\x91\x90a-\xE4V[`\0\x80a\x16I\x84\x84a\x16\x14V[\x90P`\0a\x16V\x82a\x1A\xB7V[\x90P`\0a\x16c\x82a\x0B?V[\x85Q\x90\x91Pa\x0B2\x90\x82\x90a\x16x\x90\x8Aa\x1ADV[\x90a\x1ADV[\x82\x82\x02\x81\x15\x80\x15\x90a\x16\xA6WP\x83\x15\x80a\x16\xA6WP\x82\x84\x82\x81a\x16\xA3Wa\x16\xA3a-\xA0V[\x05\x14[a\x16\xE4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rn\x1B][\x1AY\x1A]\x88\x1A[\x9D\x98[\x1AY`\x8A\x1B`D\x82\x01R`d\x01a\t\rV[\x81\x81\x81a\x16\xF3Wa\x16\xF3a-\xA0V[\x05\x94\x93PPPPV[`@\x80Q` \x81\x01\x85\x90R\x90\x81\x01\x83\x90R``\x81\x01\x82\x90R`\0\x90\x81\x90a\x17%\x90`\x80\x01a\x08-V[\x90Pa\x11\xA2\x85\x85\x83a\x04\xDCa\x14\xABV[`\0a\x17B\x84\x84\x84a\x16~V[\x90P\x81a\x17O\x84\x86a-pV[a\x17Y\x91\x90a/<V[\x15a\x0B8W`\x01`\x01`\xFF\x1B\x03\x81\x12a\x17\xA9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01RqmulidivUp overflow`p\x1B`D\x82\x01R`d\x01a\t\rV[a\x0F\x82`\x01\x82a.\xFDV[`\0\x80a\x17\xC0\x83a `V[\x90P`\0a\x17\xD2c;\x9A\xCA\0\x83a/%V[\x90Pa\x17\xDE\x85\x82a\x1CdV[\x92PPP[\x92\x91PPV[`\0a\x0B8\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a!\x04V[`\0g\x06\xF0[Y\xD3\xB2\0\0\x82\x03a\x18\x17WP`\0\x91\x90PV[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x12a\x18?W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\x18`W`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x18m\x83`\x02a-pV[\x90P`\0a\x18z\x82a!#V[\x90P`\0a\x18\x90g\x13\xA0K\xBD\xFD\xC9\xBE\x88\x83a#\xA1V[\x90Pa\x11\xA2\x81a.\x83V[`\0h\x02H\xCE6\xA7\x0C\xB2k>\x19\x82\x13a\x18\xB6WP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a\x18\xFDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x01a\t\rV[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05`\x01`_\x1B\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1Dm\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1Dn\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[`\0a\x0B8\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a#\xB6V[`\0a\x0B8\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a#\xB6V[a\x1A\xB3\x82\x82`@Q`$\x01a\x1A\x84\x92\x91\x90a/PV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c-\x83\x9C\xB3`\xE2\x1B\x17\x90Ra#\xE4V[PPV[`\0\x80g\x13\xA0K\xBD\xFD\xC9\xBE\x88a\x1A\xD5g\r\xE0\xB6\xB3\xA7d\0\0\x85a-pV[a\x1A\xDF\x91\x90a-\xB6V[\x90P`\0a\x1A\xEC\x82a.\x83V[\x90P`\0a\x1A\xF9\x82a#\xF0V[\x90Pg\x1B\xC1mgN\xC8\0\0a\x1B\x16g\r\xE0\xB6\xB3\xA7d\0\0\x83a-pV[a\x11\xA2\x91\x90a-\xB6V[`\0a\x1B*a\x0B\x88V[` \x81\x01Q`\x05\x81\x90U`\x04UB`\x08\x81\x90U`\x06\x81\x90U\x81Q`\n\x81\x90U`\tU`\r\x81\x90U`\x0B\x81\x90U`@\x90\x91\x01Q`\x0F\x81\x90U`\x0EU`\x12\x81\x90U`\x10UPV[`\0\x82\x85\x10a\x1B\xC0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FtradingFunction: invalid x\0\0\0\0\0\0`D\x82\x01R`d\x01a\t\rV[`\0\x80g\r\xE0\xB6\xB3\xA7d\0\0a\x1B\xD6\x88\x87a\x17\xE9V[\x10a\x1B\xE4W`\xFC\x91Pa\x1B\xF4V[a\x1B\xF1a\x06\xEF\x88\x87a\x17\xE9V[\x91P[g\r\xE0\xB6\xB3\xA7d\0\0a\x1C\x14\x87a\x1C\x0F\x87`\0\x01Q\x89a\x1CdV[a\x17\xE9V[\x10a\x1C!WP`\xFCa\x1C9V[a\x1C6a\x06\xEF\x87a\x1C\x0F\x87`\0\x01Q\x89a\x1CdV[\x90P[`\0a\x1CM\x85` \x01Q\x86`@\x01Qa\x17\xB4V[\x90P\x80a\x1CZ\x83\x85a.\xFDV[a\x0E\x80\x91\x90a.\xFDV[`\0a\x0B8\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a!\x04V[`\0\x80`\0\x80`\0\x86\x80` \x01\x90Q\x81\x01\x90a\x1C\x95\x91\x90a/rV[\x93P\x93P\x93P\x93P\x81a\x1C\xAA\x85\x88\x86\x85a\x1BoV[a\r\x02\x91\x90a-\xE4V[`\0\x84\x86\x11\x15a\x1C\xE1W`@Qc0\x82\xDF\xDB`\xE1\x1B\x81R`\x04\x81\x01\x87\x90R`$\x81\x01\x86\x90R`D\x01a\t\rV[`\0a\x1C\xF1\x88\x88\x85c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x1D\x03\x89\x88\x86c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x1D\x11\x82\x84a-pV[\x13\x15a\x1D:W`@Qc\x06\xF1\xBE]`\xE2\x1B\x81R`\x04\x81\x01\x83\x90R`$\x81\x01\x82\x90R`D\x01a\t\rV[`\0a\x1DF\x89\x89a-]V[\x90P`\0[`\x02a\x1DW\x8A\x8Ca.<V[a\x1Da\x91\x90a.\xCAV[\x94P`\0a\x1Ds\x8C\x87\x89c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x1D\x81\x86\x83a-pV[\x13a\x1D\x8EW\x85\x99Pa\x1D\x95V[\x85\x9AP\x80\x94P[a\x1D\x9F\x8B\x8Ba-]V[\x92PP`\x01\x01\x87\x82\x11\x80\x15a\x1D\xB3WP\x86\x81\x10[a\x1DKWPPPP\x96\x95PPPPPPV[a\x1D\xCDa\x15\xA9V[`\x0EUB`\x10UV[`\0\x80`\0\x80`\0\x86\x80` \x01\x90Q\x81\x01\x90a\x1D\xF2\x91\x90a/rV[\x93P\x93P\x93P\x93Pa\r\x02\x84\x84\x88\x84a\x1BoV[a\x1E\x0Ea\x14\xF6V[`\tUB`\x0BUV[`\0a\x0B8g\r\xE0\xB6\xB3\xA7d\0\0\x83a\x1E/\x86a\x1ECV[a\x1E9\x91\x90a-pV[a\x07\x12\x91\x90a-\xB6V[`\0\x80\x82\x13a\x1E\x80W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\t\rV[`\0``a\x1E\x8D\x84a%\xD4V[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[a &a\x11\xD4V[`\x04UB`\x06UV[`\0\x80`\0\x80`\0\x86\x80` \x01\x90Q\x81\x01\x90a K\x91\x90a/rV[\x93P\x93P\x93P\x93P\x81a\x1C\xAA\x87\x86\x86\x85a\x1BoV[`\xB5\x81`\x01`\x88\x1B\x81\x10a yW`@\x91\x90\x91\x1B\x90`\x80\x1C[i\x01\0\0\0\0\0\0\0\0\0\x81\x10a \x95W` \x91\x90\x91\x1B\x90`@\x1C[e\x01\0\0\0\0\0\x81\x10a \xADW`\x10\x91\x90\x91\x1B\x90` \x1C[c\x01\0\0\0\x81\x10a \xC3W`\x08\x91\x90\x91\x1B\x90`\x10\x1C[b\x01\0\0\x01\x02`\x12\x1C\x80\x82\x04\x01`\x01\x90\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x90\x81\x90\x04\x81\x11\x90\x03\x90V[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a!\x1CW`\0\x80\xFD[\x04\x92\x91PPV[`\0\x80\x82\x12\x80a!:WPg\x1B\xC1mgN\xC8\0\0\x82\x13[\x15a!XW`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a!yW`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81g\x1B\xC1mgN\xC8\0\0\x03a!\xA1W`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x15a!\xACW\x91\x90PV[`\0g\r\xE0\xB6\xB3\xA7d\0\0\x83\x12a!\xD4Wa!\xCF\x83g\x1B\xC1mgN\xC8\0\0a-\xE4V[a!\xD6V[\x82[\x90P`\0a!\xEC\x82g\x1B\xC1mgN\xC8\0\0a&|V[\x90P\x80`\0\x03a\"\x0FW`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\"\x1A\x82a\x1ECV[\x90P`\0c;\x9A\xCA\0a\"Ea\"@a\":g\x1B\xC1mgN\xC8\0\0a.\x83V[\x85a#\xA1V[a `V[a\"O\x91\x90a-pV[\x90P`\0\x80a\"f\x83g\x03\xC1f\\z\xAB \0a#\xA1V[a\"x\x90g \x05\xFEO&\x8E\xA0\0a.\xFDV[\x90P`\0a\"\xA8\x84a\"\x91\x86f\x9F2u$b\xA0\0a#\xA1V[a\"\xA3\x90g\r\xC5R\x7Fd, \0a.\xFDV[a#\xA1V[a\"\xBA\x90g\r\xE0\xB6\xB3\xA7d\0\0a.\xFDV[\x90Pa\"\xDEg\t\xD0(\xCCo _\xFF\x19\x85a\"\xD4\x85\x85a&|V[a\"\xA3\x91\x90a-\xE4V[\x92PPP`\0[`\x02\x81\x10\x15a#yW`\0\x86a\"\xFA\x84a#\xF0V[a#\x04\x91\x90a-\xE4V[\x90P`\0a#\x12\x84\x85a#\xA1V[a#\x1B\x90a.\x83V[\x90P`\0a#(\x82a\x18\x9BV[\x90P`\0a#6\x86\x85a#\xA1V[a#Hg\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x84a#\xA1V[a#R\x91\x90a-\xE4V[\x90Pa#^\x84\x82a&|V[a#h\x90\x87a.\xFDV[\x95P\x84`\x01\x01\x94PPPPPa\"\xE5V[g\r\xE0\xB6\xB3\xA7d\0\0\x88\x12a#\x96Wa#\x91\x82a.\x83V[a\x0E\x80V[P\x96\x95PPPPPPV[`\0a\x0B8\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a&\x8DV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a#\xCEW`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[a#\xED\x81a&\xACV[PV[`\0\x81`\0\x03a$\tWPg\r\xE0\xB6\xB3\xA7d\0\0\x91\x90PV[gV\x98\xEE\xF0fp\0\0\x82\x12a$ WP`\0\x91\x90PV[a$1gV\x98\xEE\xF0fp\0\0a.\x83V[\x82\x13a$FWPg\x1B\xC1mgN\xC8\0\0\x91\x90PV[`\0a$Q\x83a&\xCDV[\x90P`\0a$\x8Ag\r\xE0\xB6\xB3\xA7d\0\0a$s\x84g\x1B\xC1mgN\xC8\0\0a\x17\xE9V[a$\x85\x90g\r\xE0\xB6\xB3\xA7d\0\0a.\xFDV[a&|V[\x90P`\0\x80\x82a$\xE6\x81a$\xD3\x81a$\xC1\x81a$\xAE\x81g\x02_\x0F\xE1\x05\xA3\x14\0a#\xA1V[a\"\xA3\x90g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19a.\xFDV[a\"\xA3\x90g\x14\xA8EL\x19\xE1\xAC\0a.\xFDV[a\"\xA3\x90g\x0F\xC1\x0E\x01W\x82w\xFF\x19a.\xFDV[a$\xF8\x90g\x03\xDE\xBD\x08;\x8C|\0a.\xFDV[\x91P\x83\x90Pa%`\x81a%N\x81a%<\x81a%*\x81a%\x17\x81\x8Ba#\xA1V[a\"\xA3\x90g\x02\x95\xD4\0\xEA2W\xFF\x19a.\xFDV[a\"\xA3\x90g\x01W\xD8\xB2\xEC\xC7\x08\0a.\xFDV[a\"\xA3\x90g\x051\n\xA7\xD5!0\0a.\xFDV[a\"\xA3\x90g\r\xE0\xCC=\x15a\0\0a.\xFDV[\x91P\x81g\x11\x90\0\xAB\x10\x0F\xFC\0a%v\x87\x88a#\xA1V[a%\x82\x90`\0\x19a-pV[a%\x8C\x91\x90a-\xE4V[a%\x96\x91\x90a.\xFDV[\x92PP`\0a%\xA4\x83a\x18\x9BV[\x90P`\0a%\xB2\x85\x83a#\xA1V[\x90P`\0\x88\x12a%\xC2W\x80a\x0E\x80V[a\x0E\x80\x81g\x1B\xC1mgN\xC8\0\0a-\xE4V[`\0\x80\x82\x11a&\x11W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\t\rV[P`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x82\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[`\0a\x0B8\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x05\x85\x14\x17\x16a&\xA5W`\0\x80\xFD[\x05\x92\x91PPV[\x80Qjconsole.log` \x83\x01`\0\x80\x84\x83\x85Z\xFAPPPPPV[`\0`\x01`\xFF\x1B\x82\x03a&\xF3W`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x12\x15a\x0B\x84WP\x19`\x01\x01\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a'\xBFWa'\xBFa'\x04V[PP\x835\x95` \x85\x015\x95P`@\x85\x015\x94``\x81\x015\x94P`\x80\x015\x92P\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a'\xF8Wa'\xF8a'\x04V[\x825\x80\x15\x15\x81\x14a(\x08W`\0\x80\xFD[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a(<W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a( V[P`\0` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[\x84\x15\x15\x81R\x83` \x82\x01R\x82`@\x82\x01R`\x80``\x82\x01R`\0a(\x83`\x80\x83\x01\x84a(\x16V[\x96\x95PPPPPPV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FABI decoding: struct data too sh`D\x82\x01Rb\x1B\xDC\x9D`\xEA\x1B`d\x82\x01R`\x84\x81\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a)\x17Wa)\x17a(\xDEV[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a)FWa)Fa(\xDEV[`@R\x91\x90PV[`\0``\x82\x84\x03\x12\x15a)cWa)ca(\x8DV[a)ka(\xF4V[\x90P\x815\x81R` \x82\x015` \x82\x01R`@\x82\x015`@\x82\x01R\x92\x91PPV[`\0\x80`\0\x80`\xC0\x85\x87\x03\x12\x15a)\xA4Wa)\xA4a'\x04V[\x845\x93P` \x85\x015\x92P`@\x85\x015\x91Pa)\xC3\x86``\x87\x01a)NV[\x90P\x92\x95\x91\x94P\x92PV[` \x81R`\0a\x0B8` \x83\x01\x84a(\x16V[`\0\x80`\0`\xA0\x84\x86\x03\x12\x15a)\xF9Wa)\xF9a'\x04V[\x835\x92P` \x84\x015\x91Pa*\x11\x85`@\x86\x01a)NV[\x90P\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a*/Wa*/a'\x04V[P5\x91\x90PV[\x81Q\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x80\x83\x01Q\x90\x82\x01R``\x81\x01a\x17\xE3V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x81\xFD[`\0\x80` \x83\x85\x03\x12\x15a*\xC6Wa*\xC6a'\x04V[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a*\xE1Wa*\xE1a'TV[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a*\xF8Wa*\xF8a*WV[\x815\x81\x81\x11\x15a+[W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x0ENL/$\r\x8C\xAD\xCC\xEE\x8D`\xAB\x1B`d\x82\x01R`\x84\x81\xFD[\x86` \x82\x85\x01\x01\x11\x15a+\xC3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rjrray stride`\xA8\x1B`d\x82\x01R\x91P`\x84\x82\xFD[` \x92\x90\x92\x01\x96\x91\x95P\x90\x93PPPPV[`\0\x80`@\x83\x85\x03\x12\x15a+\xEBWa+\xEBa'\x04V[PP\x805\x92` \x90\x91\x015\x91PV[`\0\x80`\x80\x83\x85\x03\x12\x15a,\x10Wa,\x10a'\x04V[\x825\x91Pa,!\x84` \x85\x01a)NV[\x90P\x92P\x92\x90PV[`\0` \x80\x83\x85\x03\x12\x15a,@Wa,@a'\x04V[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a,[Wa,[a'TV[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a,rWa,ra*WV[\x815\x81\x81\x11\x15a,\x84Wa,\x84a(\xDEV[a,\x96`\x1F\x82\x01`\x1F\x19\x16\x85\x01a)\x1DV[\x91P\x80\x82R\x86\x84\x82\x85\x01\x01\x11\x15a,\xFCW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x85\x90R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R`\x84\x81\xFD[\x80\x84\x84\x01\x85\x84\x017`\0\x90\x82\x01\x90\x93\x01\x92\x90\x92RP\x93\x92PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a-0Wa-0a'\x04V[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x17\xE3Wa\x17\xE3a-GV[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15a-\x8CWa-\x8Ca-GV[\x81\x81\x05\x83\x14\x82\x15\x17a\x17\xE3Wa\x17\xE3a-GV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a-\xC5Wa-\xC5a-\xA0V[`\x01`\xFF\x1B\x82\x14`\0\x19\x84\x14\x16\x15a-\xDFWa-\xDFa-GV[P\x05\x90V[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a.\x04Wa.\x04a-GV[P\x92\x91PPV[`\0\x80`\0``\x84\x86\x03\x12\x15a.#Wa.#a'\x04V[\x83Q\x92P` \x84\x01Q\x91P`@\x84\x01Q\x90P\x92P\x92P\x92V[\x80\x82\x01\x80\x82\x11\x15a\x17\xE3Wa\x17\xE3a-GV[\x84\x81R` \x80\x82\x01\x85\x90R`@\x80\x83\x01\x85\x90R\x83Q``\x84\x01R\x90\x83\x01Q`\x80\x83\x01R\x82\x01Q`\xA0\x82\x01R`\xC0\x81\x01a\x11\xA2V[`\0`\x01`\xFF\x1B\x82\x01a.\x98Wa.\x98a-GV[P`\0\x03\x90V[` \x80\x82R`\x11\x90\x82\x01Rp\x15\\\x19\x18]\x19H\x19[\x99\x08\x1C\x18\\\xDC\xD9Y`z\x1B`@\x82\x01R``\x01\x90V[`\0\x82a.\xD9Wa.\xD9a-\xA0V[P\x04\x90V[`\0`\x01`\x01`\xFF\x1B\x01\x82\x01a.\xF6Wa.\xF6a-GV[P`\x01\x01\x90V[\x80\x82\x01\x82\x81\x12`\0\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a/\x1DWa/\x1Da-GV[PP\x92\x91PPV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x17\xE3Wa\x17\xE3a-GV[`\0\x82a/KWa/Ka-\xA0V[P\x07\x90V[`@\x81R`\0a/c`@\x83\x01\x85a(\x16V[\x90P\x82` \x83\x01R\x93\x92PPPV[`\0\x80`\0\x80\x84\x86\x03`\xC0\x81\x12\x15a/\x8CWa/\x8Ca'\x04V[\x85Q\x94P` \x86\x01Q\x93P`@\x86\x01Q\x92P```_\x19\x82\x01\x12\x15a/\xB3Wa/\xB3a(\x8DV[Pa/\xBCa(\xF4V[``\x86\x01Q\x81R`\x80\x86\x01Q` \x82\x01R`\xA0\x90\x95\x01Q`@\x86\x01RP\x91\x94\x90\x93P\x90\x91\x90V";
    /// The bytecode of the contract.
    pub static DFMM_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x01XW`\x005`\xE0\x1C\x80cb}\xD5j\x11a\0\xEFW\x80c\xB7\xD1\x9F\xC4\x11a\0\xBEW\x80c\xB7\xD1\x9F\xC4\x14a\x02\xE6W\x80c\xCF0\x90\x12\x14a\x02\xF9W\x80c\xEB\xAD\xEF\x01\x14a\x03\x02W\x80c\xF0e\x95\x7F\x14a\x03\x10W\x80c\xF3\xA8\xEF\xE3\x14a\x03\x19Wa\x01XV[\x80cb}\xD5j\x14a\x02\x96W\x80cg\xE8(\xBF\x14a\x02\xABW\x80cp\xA0\x821\x14a\x02\xBEW\x80c\x85\x93\x10\xB6\x14a\x02\xDEWa\x01XV[\x80cC\xC8\x85\xBA\x11a\x01+W\x80cC\xC8\x85\xBA\x14a\x023W\x80cM\xDFG\xD4\x14a\x02WW\x80c^\"}X\x14a\x02\x85W\x80c^aZk\x14a\x02\x8EWa\x01XV[\x80c\n\xC304\x14a\x01\xBDW\x80c\x15w\x0F\x92\x14a\x01\xE9W\x80c\x16\xDC\x16[\x14a\x02\0W\x80c\x19c\x824\x14a\x02+W[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x01\xD0a\x01\xCB6`\x04a\x14{V[a\x03!V[`@Qa\x01\xE0\x94\x93\x92\x91\x90a\x14\xFAV[`@Q\x80\x91\x03\x90\xF3[a\x01\xF2`\x06T\x81V[`@Q\x90\x81R` \x01a\x01\xE0V[`\x02Ta\x02\x13\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\xE0V[a\x01\xF2a\x03\xFDV[`\0Ta\x02G\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x01\xE0V[a\x02ja\x02e6`\x04a\x15\x84V[a\x04\xF8V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x01\xE0V[a\x01\xF2`\x05T\x81V[a\x02ja\x08\"V[a\x02\xA9a\x02\xA46`\x04a\x15\x84V[a\x08\xF5V[\0[`\0Ta\x02\x13\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01\xF2a\x02\xCC6`\x04a\x16\xA9V[`\x07` R`\0\x90\x81R`@\x90 T\x81V[a\x01\xF2a\x0C!V[`\x03Ta\x02\x13\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01\xF2`\x01T\x81V[`\x04T`\x05T`\x06Ta\x02jV[a\x01\xF2`\x04T\x81V[a\x01\xF2a\x0C\xEEV[`\0\x80T`@Qc\x02\xB0\xCC\r`\xE2\x1B\x81R\x84\x15\x15`\x04\x82\x01R`$\x81\x01\x84\x90R\x82\x91\x82\x91``\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\n\xC304\x90`D\x01`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x03\xB1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x19\xB6\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x03\xC5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x03\xED\x91\x90\x81\x01\x90a\x16\xF2V[\x92\x99\x91\x98P\x96P\x90\x94P\x92PPPV[`\x04T`\x05T`\x06T`@\x80Q` \x81\x01\x94\x90\x94R\x83\x01\x91\x90\x91R``\x82\x01R`\0\x90\x81\x90`\x80\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R`\0Tc\xA1\x9C\xD3\xD1`\xE0\x1B\x83R\x90\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\xA1\x9C\xD3\xD1\x90a\x04d\x90\x84\x90`\x04\x01a\x18\"V[` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x04\xBAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x19\xB6\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x04\xCEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xF2\x91\x90a\x185V[\x91PP\x90V[`\0\x80`\0`\x01T`\x01\x14a\x05=W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x06`$\x82\x01Re\x1B\x1B\xD8\xDA\xD9Y`\xD2\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0`\x01\x81\x90U\x80T`@Qc\x13w\xD1\xF5`\xE2\x1B\x81R\x82\x91\x82\x91\x82\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cM\xDFG\xD4\x90a\x05}\x90\x8D\x90\x8D\x90`\x04\x01a\x18QV[`\xA0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x05\xD5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x19\xB6\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x05\xE9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\r\x91\x90a\x18\x80V[\x94P\x94P\x94P\x94P\x94P\x84a\x06KW`\0\x84\x12a\x06)\x85a\r4V[`@Qcw`m)`\xE1\x1B\x81R\x91\x15\x15`\x04\x83\x01R`$\x82\x01R`D\x01a\x054V[`\0\x80T`\xFF`\xA0\x1B\x19\x16`\x01`\xA0\x1B\x17\x81U`\x04\x84\x81U`\x05\x84\x90U`\x06\x83\x90U3\x80\x83R`\x07` R`@\x92\x83\x90 \x84\x90U`\x02T\x92Qc#\xB8r\xDD`\xE0\x1B\x81R\x91\x82\x01R0`$\x82\x01R`D\x81\x01\x85\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90`d\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x07\x0CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x19\xB6\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x07 W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07D\x91\x90a\x18\xCCV[P`\x03T`@Qc#\xB8r\xDD`\xE0\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90`d\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x07\xD5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x19\xB6\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x07\xE9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\r\x91\x90a\x18\xCCV[P`\x01\x80U\x91\x99\x90\x98P\x90\x96P\x94PPPPPV[`\0\x80`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c^aZk`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01```@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x08\xB2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x19\xB6\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x08\xC6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xEA\x91\x90a\x18\xECV[\x92P\x92P\x92P\x90\x91\x92V[`\x01T`\x01\x14a\t0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x06`$\x82\x01Re\x1B\x1B\xD8\xDA\xD9Y`\xD2\x1B`D\x82\x01R`d\x01a\x054V[`\0`\x01\x81\x90UT`\x01`\xA0\x1B\x90\x04`\xFF\x16a\t\x80W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rn\x1B\x9B\xDD\x08\x1A[\x9A]\x1AX[\x1A^\x99Y`\x8A\x1B`D\x82\x01R`d\x01a\x054V[`\0\x80T`@Qc\xC1nP\xEF`\xE0\x1B\x81R\x82\x91\x82\x91\x82\x91\x82\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xC1nP\xEF\x90a\t\xBB\x90\x8B\x90\x8B\x90`\x04\x01a\x18QV[`\xC0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\n\x11W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x19\xB6\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\n%W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\nI\x91\x90a\x19\x1DV[\x95P\x95P\x95P\x95P\x95P\x95P\x85a\ngW`\0\x85\x12a\x06)\x86a\r4V[`\x06\x81\x90U`\0\x80\x80\x80a\n{\x87\x87a\roV[`\0\x80T`@\x80Qc^aZk`\xE0\x1B\x81R\x90Q\x96\x9AP\x94\x98P\x92\x96P\x90\x94P3\x93`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x92\x8C\x92\x8C\x92\x8C\x92\x90\x91\x82\x91\x82\x91\x88\x91c^aZk\x91`\x04\x80\x83\x01\x92``\x92\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\x0B\x1AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x19\xB6\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0B.W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0BR\x91\x90a\x18\xECV[`@\x80Q\x84\x81R` \x81\x01\x84\x90R\x90\x81\x01\x82\x90R``\x81\x01\x89\x90R`\x80\x81\x01\x88\x90R\x92\x95P\x90\x93P\x91P\x7F\xC52.\x1Bh\xAE\x0C\xE0J\xA8\xF2\x95\xAE\xC0\xC4\xF0\xF1\xDE\xDE\xB5\xC3\xB3\xF6\x0CE\xF86\xD2(d/\xEC\x90`\xA0\x01`@Q\x80\x91\x03\x90\xA1`@\x80Q`\x01`\x01`\xA0\x1B\x03\x89\x81\x16\x82R` \x82\x01\x8D\x90R\x91\x81\x01\x8B\x90R``\x81\x01\x88\x90R\x81\x8D\x16\x91\x8E\x81\x16\x91\x90\x8B\x16\x90\x7F\xD4\xDB\xFA\xA7\xC0\xFEQ\xEE\xE2J\x1C\xDC\xAF\xD07\x8C\xD3e,\xDB\xA75\xDF\xC4\xB0a\xAAU\xC2\xA4X\xA5\x90`\x80\x01`@Q\x80\x91\x03\x90\xA4PP`\x01\x80UPPPPPPPPPPPPPPPPPPV[`\0\x80T`\x04\x80T`\x06T`@Qc\\x\x05\xB9`\xE1\x1B\x81R\x92\x83\x01\x91\x90\x91R`$\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xB8\xF0\x0Br\x90`D\x01[` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0C\xB1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x19\xB6\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0C\xC5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xE9\x91\x90a\x185V[\x90P\x90V[`\0\x80T`\x04\x80T`\x05T`\x06T`@Qcv\x14\xECs`\xE1\x1B\x81R\x93\x84\x01\x92\x90\x92R`$\x83\x01R`D\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xEC)\xD8\xE6\x90`d\x01a\x0C[V[`\0`\x01`\xFF\x1B\x82\x03a\rZW`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x12\x15a\rkWP\x19`\x01\x01\x90V[P\x90V[`\x04T`\x05T`\0\x91\x82\x91\x82\x91\x82\x91\x81\x88\x11\x15a\x0E\x08W`\x02T`\x03T`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x97P\x16\x94Pa\r\xA6\x82\x89a\x19\x89V[\x93P\x86\x81\x11a\r\xF7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7Finvalid swap: inputs x and y\0\0\0\0`D\x82\x01R`d\x01a\x054V[a\x0E\x01\x87\x82a\x19\x89V[\x92Pa\x0E\x86V[`\x03T`\x02T`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x97P\x16\x94Pa\x0E(\x81\x88a\x19\x89V[\x93P\x87\x82\x11a\x0EyW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7Finvalid swap: inputs x and y\0\0\0\0`D\x82\x01R`d\x01a\x054V[a\x0E\x83\x88\x83a\x19\x89V[\x92P[`\x04\x88\x81U`\x05\x88\x90U`@Qcp\xA0\x821`\xE0\x1B\x81R0\x91\x81\x01\x91\x90\x91R`\0\x90`\x01`\x01`\xA0\x1B\x03\x88\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0F\x12W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x19\xB6\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0F&W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0FJ\x91\x90a\x185V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x88\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0F\xCDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x19\xB6\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0F\xE1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\x05\x91\x90a\x185V[`@Qc#\xB8r\xDD`\xE0\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x88\x90R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x89\x16\x90c#\xB8r\xDD\x90`d\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x10\x94W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x19\xB6\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x10\xA8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\xCC\x91\x90a\x18\xCCV[P`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x86\x90R`\x01`\x01`\xA0\x1B\x03\x88\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x11SW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x19\xB6\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x11gW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\x8B\x91\x90a\x18\xCCV[Pa\x11\x96\x86\x83a\x19\xA2V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x8A\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x12\x13W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x19\xB6\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x12'W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12K\x91\x90a\x185V[\x10\x15a\x12\xA4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: input token transf`D\x82\x01Ra2\xB9`\xF1\x1B`d\x82\x01R`\x84\x01a\x054V[a\x12\xAE\x85\x82a\x19\x89V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x89\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x13+W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x19\xB6\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x13?W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13c\x91\x90a\x185V[\x10\x15a\x13\xBDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7Finvalid swap: output token trans`D\x82\x01Rb32\xB9`\xE9\x1B`d\x82\x01R`\x84\x01a\x054V[PPPP\x92\x95\x91\x94P\x92PV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[\x80\x15\x15\x81\x14a\x14xW`\0\x80\xFD[PV[`\0\x80`@\x83\x85\x03\x12\x15a\x14\x91Wa\x14\x91a\x13\xCAV[\x825a\x14\x9C\x81a\x14jV[\x94` \x93\x90\x93\x015\x93PPPV[`\0[\x83\x81\x10\x15a\x14\xC5W\x81\x81\x01Q\x83\x82\x01R` \x01a\x14\xADV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x14\xE6\x81` \x86\x01` \x86\x01a\x14\xAAV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[\x84\x15\x15\x81R\x83` \x82\x01R\x82`@\x82\x01R`\x80``\x82\x01R`\0a\x15!`\x80\x83\x01\x84a\x14\xCEV[\x96\x95PPPPPPV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x81\xFD[`\0\x80` \x83\x85\x03\x12\x15a\x15\x9AWa\x15\x9Aa\x13\xCAV[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x15\xB5Wa\x15\xB5a\x14\x1AV[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x15\xCCWa\x15\xCCa\x15+V[\x815\x81\x81\x11\x15a\x16/W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x0ENL/$\r\x8C\xAD\xCC\xEE\x8D`\xAB\x1B`d\x82\x01R`\x84\x81\xFD[\x86` \x82\x85\x01\x01\x11\x15a\x16\x97W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rjrray stride`\xA8\x1B`d\x82\x01R\x91P`\x84\x82\xFD[` \x92\x90\x92\x01\x96\x91\x95P\x90\x93PPPPV[`\0` \x82\x84\x03\x12\x15a\x16\xBEWa\x16\xBEa\x13\xCAV[\x815`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x16\xD5W`\0\x80\xFD[\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x17\x0BWa\x17\x0Ba\x13\xCAV[\x84Qa\x17\x16\x81a\x14jV[\x80\x94PP` \x85\x01Q\x92P`@\x85\x01Q\x91P``\x85\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x17FWa\x17Fa\x14\x1AV[\x81\x87\x01\x91P\x87`\x1F\x83\x01\x12a\x17]Wa\x17]a\x15+V[\x81Q\x81\x81\x11\x15a\x17oWa\x17oa\x16\xDCV[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a\x17\x97Wa\x17\x97a\x16\xDCV[\x81`@R\x82\x81R\x8A` \x84\x87\x01\x01\x11\x15a\x18\x02W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R\x93P`\x84\x84\xFD[a\x18\x13\x83` \x83\x01` \x88\x01a\x14\xAAV[\x97\x9A\x96\x99P\x94\x97PPPPPPV[` \x81R`\0a\x16\xD5` \x83\x01\x84a\x14\xCEV[`\0` \x82\x84\x03\x12\x15a\x18JWa\x18Ja\x13\xCAV[PQ\x91\x90PV[` \x81R\x81` \x82\x01R\x81\x83`@\x83\x017`\0\x81\x83\x01`@\x90\x81\x01\x91\x90\x91R`\x1F\x90\x92\x01`\x1F\x19\x16\x01\x01\x91\x90PV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\x18\x9BWa\x18\x9Ba\x13\xCAV[\x85Qa\x18\xA6\x81a\x14jV[` \x87\x01Q`@\x88\x01Q``\x89\x01Q`\x80\x90\x99\x01Q\x92\x9A\x91\x99P\x97\x96P\x90\x94P\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x18\xE1Wa\x18\xE1a\x13\xCAV[\x81Qa\x16\xD5\x81a\x14jV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x19\x04Wa\x19\x04a\x13\xCAV[\x83Q\x92P` \x84\x01Q\x91P`@\x84\x01Q\x90P\x92P\x92P\x92V[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a\x199Wa\x199a\x13\xCAV[\x86Qa\x19D\x81a\x14jV[` \x88\x01Q`@\x89\x01Q``\x8A\x01Q`\x80\x8B\x01Q`\xA0\x90\x9B\x01Q\x93\x9C\x92\x9BP\x90\x99\x90\x98P\x96P\x90\x94P\x92PPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x19\x9CWa\x19\x9Ca\x19sV[\x92\x91PPV[\x80\x82\x01\x80\x82\x11\x15a\x19\x9CWa\x19\x9Ca\x19sV\xFETarget contract does not contain";
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
        ///Calls the contract's `balanceOf` (0x70a08231) function
        pub fn balance_of(
            &self,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], account)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getNextLiquidity` (0xf3a8efe3) function
        pub fn get_next_liquidity(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([243, 168, 239, 227], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getParams` (0x5e615a6b) function
        pub fn get_params(
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
                .method_hash([94, 97, 90, 107], ())
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
        ///Calls the contract's `getSwapConstant` (0x19638234) function
        pub fn get_swap_constant(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([25, 99, 130, 52], ())
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
        ///Calls the contract's `internalPrice` (0x859310b6) function
        pub fn internal_price(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([133, 147, 16, 182], ())
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
        ///Calls the contract's `simulateSwap` (0x0ac33034) function
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
        ///Calls the contract's `source` (0x67e828bf) function
        pub fn source(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([103, 232, 40, 191], ())
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
        ///Gets the contract's `LogParams` event
        pub fn log_params_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogParamsFilter,
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
        Min(Min),
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
            if let Ok(decoded) = <Min as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Min(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for DFMMErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::Invalid(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Min(element) => ::ethers::core::abi::AbiEncode::encode(element),
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
                _ if selector == <Min as ::ethers::contract::EthError>::selector() => {
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
                Self::Min(element) => ::core::fmt::Display::fmt(element, f),
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
    impl ::core::convert::From<Min> for DFMMErrors {
        fn from(value: Min) -> Self {
            Self::Min(value)
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
        name = "LogParams",
        abi = "LogParams(uint256,uint256,uint256,uint256,uint256)"
    )]
    pub struct LogParamsFilter {
        pub strike: ::ethers::core::types::U256,
        pub sigma: ::ethers::core::types::U256,
        pub tau: ::ethers::core::types::U256,
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
        LogParamsFilter(LogParamsFilter),
        SwapFilter(SwapFilter),
    }
    impl ::ethers::contract::EthLogDecode for DFMMEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = LogParamsFilter::decode_log(log) {
                return Ok(DFMMEvents::LogParamsFilter(decoded));
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
                Self::LogParamsFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SwapFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<LogParamsFilter> for DFMMEvents {
        fn from(value: LogParamsFilter) -> Self {
            Self::LogParamsFilter(value)
        }
    }
    impl ::core::convert::From<SwapFilter> for DFMMEvents {
        fn from(value: SwapFilter) -> Self {
            Self::SwapFilter(value)
        }
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
    ///Container type for all input parameters for the `getNextLiquidity` function with signature `getNextLiquidity()` and selector `0xf3a8efe3`
    #[derive(
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
    #[ethcall(name = "getNextLiquidity", abi = "getNextLiquidity()")]
    pub struct GetNextLiquidityCall;
    ///Container type for all input parameters for the `getParams` function with signature `getParams()` and selector `0x5e615a6b`
    #[derive(
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
    #[ethcall(name = "getParams", abi = "getParams()")]
    pub struct GetParamsCall;
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
    ///Container type for all input parameters for the `getSwapConstant` function with signature `getSwapConstant()` and selector `0x19638234`
    #[derive(
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
    #[ethcall(name = "getSwapConstant", abi = "getSwapConstant()")]
    pub struct GetSwapConstantCall;
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
    ///Container type for all input parameters for the `internalPrice` function with signature `internalPrice()` and selector `0x859310b6`
    #[derive(
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
    #[ethcall(name = "internalPrice", abi = "internalPrice()")]
    pub struct InternalPriceCall;
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
    ///Container type for all input parameters for the `simulateSwap` function with signature `simulateSwap(bool,uint256)` and selector `0x0ac33034`
    #[derive(
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
    #[ethcall(name = "simulateSwap", abi = "simulateSwap(bool,uint256)")]
    pub struct SimulateSwapCall {
        pub swap_x_in: bool,
        pub amount_in: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `source` function with signature `source()` and selector `0x67e828bf`
    #[derive(
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
    #[ethcall(name = "source", abi = "source()")]
    pub struct SourceCall;
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
        BalanceOf(BalanceOfCall),
        GetNextLiquidity(GetNextLiquidityCall),
        GetParams(GetParamsCall),
        GetReservesAndLiquidity(GetReservesAndLiquidityCall),
        GetSwapConstant(GetSwapConstantCall),
        Init(InitCall),
        Inited(InitedCall),
        InternalPrice(InternalPriceCall),
        Locked(LockedCall),
        ReserveXWad(ReserveXWadCall),
        ReserveYWad(ReserveYWadCall),
        SimulateSwap(SimulateSwapCall),
        Source(SourceCall),
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
            if let Ok(decoded) = <BalanceOfCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BalanceOf(decoded));
            }
            if let Ok(decoded) = <GetNextLiquidityCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetNextLiquidity(decoded));
            }
            if let Ok(decoded) = <GetParamsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetParams(decoded));
            }
            if let Ok(decoded) = <GetReservesAndLiquidityCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetReservesAndLiquidity(decoded));
            }
            if let Ok(decoded) = <GetSwapConstantCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetSwapConstant(decoded));
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
            if let Ok(decoded) = <InternalPriceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InternalPrice(decoded));
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
            if let Ok(decoded) = <SimulateSwapCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SimulateSwap(decoded));
            }
            if let Ok(decoded) = <SourceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Source(decoded));
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
                Self::BalanceOf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetNextLiquidity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetParams(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetReservesAndLiquidity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetSwapConstant(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Init(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Inited(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InternalPrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Locked(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ReserveXWad(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ReserveYWad(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SimulateSwap(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Source(element) => ::ethers::core::abi::AbiEncode::encode(element),
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
                Self::BalanceOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetNextLiquidity(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetParams(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetReservesAndLiquidity(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetSwapConstant(element) => ::core::fmt::Display::fmt(element, f),
                Self::Init(element) => ::core::fmt::Display::fmt(element, f),
                Self::Inited(element) => ::core::fmt::Display::fmt(element, f),
                Self::InternalPrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::Locked(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReserveXWad(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReserveYWad(element) => ::core::fmt::Display::fmt(element, f),
                Self::SimulateSwap(element) => ::core::fmt::Display::fmt(element, f),
                Self::Source(element) => ::core::fmt::Display::fmt(element, f),
                Self::Swap(element) => ::core::fmt::Display::fmt(element, f),
                Self::TokenX(element) => ::core::fmt::Display::fmt(element, f),
                Self::TokenY(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalLiquidity(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<BalanceOfCall> for DFMMCalls {
        fn from(value: BalanceOfCall) -> Self {
            Self::BalanceOf(value)
        }
    }
    impl ::core::convert::From<GetNextLiquidityCall> for DFMMCalls {
        fn from(value: GetNextLiquidityCall) -> Self {
            Self::GetNextLiquidity(value)
        }
    }
    impl ::core::convert::From<GetParamsCall> for DFMMCalls {
        fn from(value: GetParamsCall) -> Self {
            Self::GetParams(value)
        }
    }
    impl ::core::convert::From<GetReservesAndLiquidityCall> for DFMMCalls {
        fn from(value: GetReservesAndLiquidityCall) -> Self {
            Self::GetReservesAndLiquidity(value)
        }
    }
    impl ::core::convert::From<GetSwapConstantCall> for DFMMCalls {
        fn from(value: GetSwapConstantCall) -> Self {
            Self::GetSwapConstant(value)
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
    impl ::core::convert::From<InternalPriceCall> for DFMMCalls {
        fn from(value: InternalPriceCall) -> Self {
            Self::InternalPrice(value)
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
    impl ::core::convert::From<SimulateSwapCall> for DFMMCalls {
        fn from(value: SimulateSwapCall) -> Self {
            Self::SimulateSwap(value)
        }
    }
    impl ::core::convert::From<SourceCall> for DFMMCalls {
        fn from(value: SourceCall) -> Self {
            Self::Source(value)
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
    ///Container type for all return fields from the `getNextLiquidity` function with signature `getNextLiquidity()` and selector `0xf3a8efe3`
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
    pub struct GetNextLiquidityReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getParams` function with signature `getParams()` and selector `0x5e615a6b`
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
    pub struct GetParamsReturn(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
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
    ///Container type for all return fields from the `getSwapConstant` function with signature `getSwapConstant()` and selector `0x19638234`
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
    pub struct GetSwapConstantReturn(pub ::ethers::core::types::I256);
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
    ///Container type for all return fields from the `internalPrice` function with signature `internalPrice()` and selector `0x859310b6`
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
    pub struct InternalPriceReturn {
        pub price: ::ethers::core::types::U256,
    }
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
    ///Container type for all return fields from the `simulateSwap` function with signature `simulateSwap(bool,uint256)` and selector `0x0ac33034`
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
        pub swap_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all return fields from the `source` function with signature `source()` and selector `0x67e828bf`
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
    pub struct SourceReturn(pub ::ethers::core::types::Address);
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
