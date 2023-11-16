pub use rmm_math_like::*;
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
pub mod rmm_math_like {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("computeInvariant"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("computeInvariant"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("reserveX"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("liquidity"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("reserveY"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("strikePrice"),
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
                                    name: ::std::borrow::ToOwned::to_owned("invariant"),
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
                    ::std::borrow::ToOwned::to_owned("computeLGivenX"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("computeLGivenX"),
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
                                    name: ::std::borrow::ToOwned::to_owned("S"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("K"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sigma"),
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
                                    name: ::std::borrow::ToOwned::to_owned("L"),
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
                    ::std::borrow::ToOwned::to_owned("computeLGivenY"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("computeLGivenY"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("y"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("S"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("K"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sigma"),
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
                                    name: ::std::borrow::ToOwned::to_owned("L"),
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
                    ::std::borrow::ToOwned::to_owned("computeOutputXGivenY"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "computeOutputXGivenY",
                            ),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("deltaY"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("L"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("deltaL"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("K"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sigma"),
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
                                    name: ::std::borrow::ToOwned::to_owned("outputX"),
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
                    ::std::borrow::ToOwned::to_owned("computeOutputYGivenX"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "computeOutputYGivenX",
                            ),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("deltaX"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("L"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("deltaL"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("K"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sigma"),
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
                                    name: ::std::borrow::ToOwned::to_owned("outputY"),
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
                    ::std::borrow::ToOwned::to_owned("computeSpotPrice"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("computeSpotPrice"),
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
                                    name: ::std::borrow::ToOwned::to_owned("L"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("K"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sigma"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tau"),
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
                                    name: ::std::borrow::ToOwned::to_owned("spotPrice"),
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
                    ::std::borrow::ToOwned::to_owned("computeXGivenL"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("computeXGivenL"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("L"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("S"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("K"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sigma"),
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
                                    name: ::std::borrow::ToOwned::to_owned("x"),
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
                    ::std::borrow::ToOwned::to_owned("computeYGivenL"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("computeYGivenL"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("L"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("S"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("K"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sigma"),
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
                                    name: ::std::borrow::ToOwned::to_owned("y"),
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
                    ::std::borrow::ToOwned::to_owned("fromWad"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("fromWad"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("a"),
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
                    ::std::borrow::ToOwned::to_owned("toWad"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("toWad"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("a"),
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
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::core::convert::From::from([
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
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static RMMMATHLIKE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[Pa\x14{\x80a\0m`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\0\xEBW`\x005`\xE0\x1C\x80c\xB0cCS\x11a\0\xB3W\x80c\xB0cCS\x14a\x01\xC1W\x80c\xB2\xC6\xDAw\x14a\x01\xD4W\x80c\xB3gmk\x14a\x01\xE7W\x80c\xE2\xD6\x9AI\x14a\x01\xFAW\x80c\xFD\x11\x87\xF7\x14a\x02\rWa\0\xEBV[\x80c\x12\x98\x8F$\x14a\x01PW\x80c,\xCCT\xCC\x14a\x01uW\x80c\x8D\xFB\x85\xC6\x14a\x01\x88W\x80c\x8F\xE9\xC1v\x14a\x01\x9BW\x80c\xAF\xE1\x08\x8E\x14a\x01\xAEW[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x01ca\x01^6`\x04a\x12WV[a\x02 V[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\x01ca\x01\x836`\x04a\x12\xA6V[a\x02=V[a\x01ca\x01\x966`\x04a\x12\xE4V[a\x02VV[a\x01ca\x01\xA96`\x04a\x12\xE4V[a\x02mV[a\x01ca\x01\xBC6`\x04a\x12\xE4V[a\x02{V[a\x01ca\x01\xCF6`\x04a\x12\xE4V[a\x02\x89V[a\x01ca\x01\xE26`\x04a\x13\x19V[a\x02\x97V[a\x01ca\x01\xF56`\x04a\x13\x19V[a\x02\xA8V[a\x01ca\x02\x086`\x04a\x12WV[a\x02\xB3V[a\x01ca\x02\x1B6`\x04a\x12\xE4V[a\x02\xC4V[`\0a\x021\x88\x88\x88\x88\x88\x88\x88a\x02\xD2V[\x98\x97PPPPPPPPV[`\0a\x02L\x86\x86\x86\x86\x86a\x03PV[\x96\x95PPPPPPV[`\0a\x02d\x85\x85\x85\x85a\x04\x13V[\x95\x94PPPPPV[`\0a\x02d\x85\x85\x85\x85a\x04CV[`\0a\x02d\x85\x85\x85\x85a\x04\xBDV[`\0a\x02d\x85\x85\x85\x85a\x05\x19V[`\0a\x02\xA2\x82a\x05\x8CV[\x92\x91PPV[`\0a\x02\xA2\x82a\x05\xA0V[`\0a\x021\x88\x88\x88\x88\x88\x88\x88a\x05\xB4V[`\0a\x02d\x85\x85\x85\x85a\x05\xF8V[`\0\x80a\x02\xE8\x84a\x02\xE3\x87\x89a\x13KV[a\x06vV[\x90P`\0a\x03+a\x03\x13a\x03\x0Ea\x02\xFF\x8B\x8Ea\x13KV[a\x03\t\x8A\x8Ca\x13KV[a\x06\x92V[a\x06\xA7V[a\x03\x1C\x86a\x13^V[a\x03&\x91\x90a\x13zV[a\x07DV[\x90P\x88a\x038\x83\x83a\x06vV[a\x03B\x91\x90a\x13zV[\x9A\x99PPPPPPPPPPV[`\0\x80a\x03na\x03h\x85g\x1B\xC1mgN\xC8\0\0a\x07\xADV[\x84a\x06vV[\x90P`\0a\x03\x84g\x06\xF0[Y\xD3\xB2\0\0\x83a\x06vV[\x90P`\0a\x03\x91\x85a\x07\xD9V[a\x03\x9F\x90c;\x9A\xCA\0a\x13\xA1V[\x90P`\0a\x03\xAD\x87\x83a\x06vV[\x90P`\0a\x03\xBB\x8B\x8Ba\x06\x92V[\x90Pa\x04\x04\x89a\x03\xFF\x86g\r\xE0\xB6\xB3\xA7d\0\0\x86a\x03\xDCa\x03\x0E\x88\x84a\x13\xB8V[a\x03\xE6\x91\x90a\x13\xCBV[a\x03\xF0\x91\x90a\x14\x11V[a\x03\xFA\x91\x90a\x13zV[a\x08}V[a\n+V[\x9B\x9APPPPPPPPPPPV[`\0a\x04,a\x04\"\x85\x84a\x06vV[a\x03\x0E\x90\x85a\x14?V[a\x049a\x03\x0E\x86\x88a\x14?V[a\x02d\x91\x90a\x14SV[`\0\x80a\x04Xa\x04S\x86\x86a\n@V[a\nUV[\x90P`\0a\x04zg\x06\xF0[Y\xD3\xB2\0\0a\x03\xFF\x86g\x1B\xC1mgN\xC8\0\0a\x07\xADV[\x90P`\0a\x04\xA8\x85a\x04\x8C\x84\x86a\x13zV[a\x04\x9E\x90g\r\xE0\xB6\xB3\xA7d\0\0a\x13\xCBV[a\x03&\x91\x90a\x14\x11V[\x90Pa\x021\x88a\x04\xB8\x88\x84a\n+V[a\n@V[`\0\x80a\x04\xCDa\x04S\x86\x86a\n@V[\x90P`\0a\x04\xEFg\x06\xF0[Y\xD3\xB2\0\0a\x03\xFF\x86g\x1B\xC1mgN\xC8\0\0a\x07\xADV[\x90P`\0a\x05\x01\x85a\x04\x8C\x84\x86a\x14SV[\x90Pa\x021\x88a\x03\xFF\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x13zV[`\0\x80a\x05)a\x04S\x86\x86a\n@V[\x90P`\0a\x05Kg\x06\xF0[Y\xD3\xB2\0\0a\x03\xFF\x86g\x1B\xC1mgN\xC8\0\0a\x07\xADV[\x90P`\0a\x05]\x85a\x04\x8C\x84\x86a\x14SV[\x90P`\0a\x05s\x82g\r\xE0\xB6\xB3\xA7d\0\0a\x13zV[\x90Pa\x05\x7F\x89\x82a\n@V[\x99\x98PPPPPPPPPV[`\0a\x02\xA2g\r\xE0\xB6\xB3\xA7d\0\0\x83a\x13\xA1V[`\0a\x02\xA2g\r\xE0\xB6\xB3\xA7d\0\0\x83a\x14?V[`\0\x80a\x05\xC5\x84a\x02\xE3\x87\x89a\x13KV[\x90P`\0a\x05\xE2a\x03\x13a\x03\x0Ea\x05\xDC\x8B\x8Da\x13KV[\x85a\x06\x92V[\x90P\x89a\x038a\x05\xF2\x88\x8Aa\x13KV[\x83a\x06vV[`\0\x80a\x06\x08a\x04S\x86\x86a\n@V[\x90P`\0a\x06*g\x06\xF0[Y\xD3\xB2\0\0a\x03\xFF\x86g\x1B\xC1mgN\xC8\0\0a\x07\xADV[\x90P`\0a\x068\x82\x84a\x13zV[\x90P`\0\x85a\x06O\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x13\xCBV[a\x06Y\x91\x90a\x14\x11V[\x90P`\0a\x06f\x82a\x07DV[\x90Pa\x03B\x88a\x03\xFF\x8C\x84a\n+V[`\0a\x06\x8B\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x0C0V[\x93\x92PPPV[`\0a\x06\x8B\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x0C0V[`\0g\x06\xF0[Y\xD3\xB2\0\0\x82\x03a\x06\xC0WP`\0\x91\x90PV[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x12a\x06\xE8W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\x07\tW`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x07\x16\x83`\x02a\x13\xCBV[\x90P`\0a\x07#\x82a\x0COV[\x90P`\0a\x079g\x13\xA0K\xBD\xFD\xC9\xBE\x88\x83a\x0E\xCDV[\x90Pa\x02d\x81a\x13^V[`\0\x80g\x13\xA0K\xBD\xFD\xC9\xBE\x88a\x07bg\r\xE0\xB6\xB3\xA7d\0\0\x85a\x13\xCBV[a\x07l\x91\x90a\x14\x11V[\x90P`\0a\x07y\x82a\x13^V[\x90P`\0a\x07\x86\x82a\x0E\xE2V[\x90Pg\x1B\xC1mgN\xC8\0\0a\x07\xA3g\r\xE0\xB6\xB3\xA7d\0\0\x83a\x13\xCBV[a\x02d\x91\x90a\x14\x11V[`\0a\x06\x8Bg\r\xE0\xB6\xB3\xA7d\0\0\x83a\x07\xC5\x86a\nUV[a\x07\xCF\x91\x90a\x13\xCBV[a\x03\xFA\x91\x90a\x14\x11V[`\xB5\x81`\x01`\x88\x1B\x81\x10a\x07\xF2W`@\x91\x90\x91\x1B\x90`\x80\x1C[i\x01\0\0\0\0\0\0\0\0\0\x81\x10a\x08\x0EW` \x91\x90\x91\x1B\x90`@\x1C[e\x01\0\0\0\0\0\x81\x10a\x08&W`\x10\x91\x90\x91\x1B\x90` \x1C[c\x01\0\0\0\x81\x10a\x08<W`\x08\x91\x90\x91\x1B\x90`\x10\x1C[b\x01\0\0\x01\x02`\x12\x1C\x80\x82\x04\x01`\x01\x90\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x90\x81\x90\x04\x81\x11\x90\x03\x90V[`\0h\x02H\xCE6\xA7\x0C\xB2k>\x19\x82\x13a\x08\x98WP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a\x08\xE4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05`\x01`_\x1B\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1Dm\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1Dn\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[`\0a\x06\x8B\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x10\xC6V[`\0a\x06\x8B\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x10\xC6V[`\0\x80\x82\x13a\n\x92W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x08\xDBV[`\0``a\n\x9F\x84a\x10\xF4V[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x0CHW`\0\x80\xFD[\x04\x92\x91PPV[`\0\x80\x82\x12\x80a\x0CfWPg\x1B\xC1mgN\xC8\0\0\x82\x13[\x15a\x0C\x84W`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\x0C\xA5W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81g\x1B\xC1mgN\xC8\0\0\x03a\x0C\xCDW`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x15a\x0C\xD8W\x91\x90PV[`\0g\r\xE0\xB6\xB3\xA7d\0\0\x83\x12a\r\0Wa\x0C\xFB\x83g\x1B\xC1mgN\xC8\0\0a\x13zV[a\r\x02V[\x82[\x90P`\0a\r\x18\x82g\x1B\xC1mgN\xC8\0\0a\x11\x9CV[\x90P\x80`\0\x03a\r;W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\rF\x82a\nUV[\x90P`\0c;\x9A\xCA\0a\rqa\rla\rfg\x1B\xC1mgN\xC8\0\0a\x13^V[\x85a\x0E\xCDV[a\x07\xD9V[a\r{\x91\x90a\x13\xCBV[\x90P`\0\x80a\r\x92\x83g\x03\xC1f\\z\xAB \0a\x0E\xCDV[a\r\xA4\x90g \x05\xFEO&\x8E\xA0\0a\x14SV[\x90P`\0a\r\xD4\x84a\r\xBD\x86f\x9F2u$b\xA0\0a\x0E\xCDV[a\r\xCF\x90g\r\xC5R\x7Fd, \0a\x14SV[a\x0E\xCDV[a\r\xE6\x90g\r\xE0\xB6\xB3\xA7d\0\0a\x14SV[\x90Pa\x0E\ng\t\xD0(\xCCo _\xFF\x19\x85a\x0E\0\x85\x85a\x11\x9CV[a\r\xCF\x91\x90a\x13zV[\x92PPP`\0[`\x02\x81\x10\x15a\x0E\xA5W`\0\x86a\x0E&\x84a\x0E\xE2V[a\x0E0\x91\x90a\x13zV[\x90P`\0a\x0E>\x84\x85a\x0E\xCDV[a\x0EG\x90a\x13^V[\x90P`\0a\x0ET\x82a\x08}V[\x90P`\0a\x0Eb\x86\x85a\x0E\xCDV[a\x0Etg\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x84a\x0E\xCDV[a\x0E~\x91\x90a\x13zV[\x90Pa\x0E\x8A\x84\x82a\x11\x9CV[a\x0E\x94\x90\x87a\x14SV[\x95P\x84`\x01\x01\x94PPPPPa\x0E\x11V[g\r\xE0\xB6\xB3\xA7d\0\0\x88\x12a\x0E\xC2Wa\x0E\xBD\x82a\x13^V[a\x021V[P\x96\x95PPPPPPV[`\0a\x06\x8B\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x11\xADV[`\0\x81`\0\x03a\x0E\xFBWPg\r\xE0\xB6\xB3\xA7d\0\0\x91\x90PV[gV\x98\xEE\xF0fp\0\0\x82\x12a\x0F\x12WP`\0\x91\x90PV[a\x0F#gV\x98\xEE\xF0fp\0\0a\x13^V[\x82\x13a\x0F8WPg\x1B\xC1mgN\xC8\0\0\x91\x90PV[`\0a\x0FC\x83a\x11\xCCV[\x90P`\0a\x0F|g\r\xE0\xB6\xB3\xA7d\0\0a\x0Fe\x84g\x1B\xC1mgN\xC8\0\0a\x06\x92V[a\x0Fw\x90g\r\xE0\xB6\xB3\xA7d\0\0a\x14SV[a\x11\x9CV[\x90P`\0\x80\x82a\x0F\xD8\x81a\x0F\xC5\x81a\x0F\xB3\x81a\x0F\xA0\x81g\x02_\x0F\xE1\x05\xA3\x14\0a\x0E\xCDV[a\r\xCF\x90g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19a\x14SV[a\r\xCF\x90g\x14\xA8EL\x19\xE1\xAC\0a\x14SV[a\r\xCF\x90g\x0F\xC1\x0E\x01W\x82w\xFF\x19a\x14SV[a\x0F\xEA\x90g\x03\xDE\xBD\x08;\x8C|\0a\x14SV[\x91P\x83\x90Pa\x10R\x81a\x10@\x81a\x10.\x81a\x10\x1C\x81a\x10\t\x81\x8Ba\x0E\xCDV[a\r\xCF\x90g\x02\x95\xD4\0\xEA2W\xFF\x19a\x14SV[a\r\xCF\x90g\x01W\xD8\xB2\xEC\xC7\x08\0a\x14SV[a\r\xCF\x90g\x051\n\xA7\xD5!0\0a\x14SV[a\r\xCF\x90g\r\xE0\xCC=\x15a\0\0a\x14SV[\x91P\x81g\x11\x90\0\xAB\x10\x0F\xFC\0a\x10h\x87\x88a\x0E\xCDV[a\x10t\x90`\0\x19a\x13\xCBV[a\x10~\x91\x90a\x13zV[a\x10\x88\x91\x90a\x14SV[\x92PP`\0a\x10\x96\x83a\x08}V[\x90P`\0a\x10\xA4\x85\x83a\x0E\xCDV[\x90P`\0\x88\x12a\x10\xB4W\x80a\x021V[a\x021\x81g\x1B\xC1mgN\xC8\0\0a\x13zV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x10\xDEW`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[`\0\x80\x82\x11a\x111W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x08\xDBV[P`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x82\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[`\0a\x06\x8B\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x05\x85\x14\x17\x16a\x11\xC5W`\0\x80\xFD[\x05\x92\x91PPV[`\0`\x01`\xFF\x1B\x82\x03a\x11\xF2W`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x12\x15a\x12\x03WP\x19`\x01\x01\x90V[P\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`\0\x80`\0\x80`\0\x80`\0`\xE0\x88\x8A\x03\x12\x15a\x12uWa\x12ua\x12\x07V[PP\x855\x97` \x87\x015\x97P`@\x87\x015\x96``\x81\x015\x96P`\x80\x81\x015\x95P`\xA0\x81\x015\x94P`\xC0\x015\x92P\x90PV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\x12\xC1Wa\x12\xC1a\x12\x07V[PP\x835\x95` \x85\x015\x95P`@\x85\x015\x94``\x81\x015\x94P`\x80\x015\x92P\x90PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x12\xFDWa\x12\xFDa\x12\x07V[PP\x825\x94` \x84\x015\x94P`@\x84\x015\x93``\x015\x92P\x90PV[`\0` \x82\x84\x03\x12\x15a\x13.Wa\x13.a\x12\x07V[P5\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x02\xA2Wa\x02\xA2a\x135V[`\0`\x01`\xFF\x1B\x82\x01a\x13sWa\x13sa\x135V[P`\0\x03\x90V[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a\x13\x9AWa\x13\x9Aa\x135V[P\x92\x91PPV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x02\xA2Wa\x02\xA2a\x135V[\x81\x81\x03\x81\x81\x11\x15a\x02\xA2Wa\x02\xA2a\x135V[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15a\x13\xE7Wa\x13\xE7a\x135V[\x81\x81\x05\x83\x14\x82\x15\x17a\x02\xA2Wa\x02\xA2a\x135V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a\x14 Wa\x14 a\x13\xFBV[`\x01`\xFF\x1B\x82\x14`\0\x19\x84\x14\x16\x15a\x14:Wa\x14:a\x135V[P\x05\x90V[`\0\x82a\x14NWa\x14Na\x13\xFBV[P\x04\x90V[\x80\x82\x01\x82\x81\x12`\0\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a\x14sWa\x14sa\x135V[PP\x92\x91PPV";
    /// The bytecode of the contract.
    pub static RMMMATHLIKE_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\0\xEBW`\x005`\xE0\x1C\x80c\xB0cCS\x11a\0\xB3W\x80c\xB0cCS\x14a\x01\xC1W\x80c\xB2\xC6\xDAw\x14a\x01\xD4W\x80c\xB3gmk\x14a\x01\xE7W\x80c\xE2\xD6\x9AI\x14a\x01\xFAW\x80c\xFD\x11\x87\xF7\x14a\x02\rWa\0\xEBV[\x80c\x12\x98\x8F$\x14a\x01PW\x80c,\xCCT\xCC\x14a\x01uW\x80c\x8D\xFB\x85\xC6\x14a\x01\x88W\x80c\x8F\xE9\xC1v\x14a\x01\x9BW\x80c\xAF\xE1\x08\x8E\x14a\x01\xAEW[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x01ca\x01^6`\x04a\x12WV[a\x02 V[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\x01ca\x01\x836`\x04a\x12\xA6V[a\x02=V[a\x01ca\x01\x966`\x04a\x12\xE4V[a\x02VV[a\x01ca\x01\xA96`\x04a\x12\xE4V[a\x02mV[a\x01ca\x01\xBC6`\x04a\x12\xE4V[a\x02{V[a\x01ca\x01\xCF6`\x04a\x12\xE4V[a\x02\x89V[a\x01ca\x01\xE26`\x04a\x13\x19V[a\x02\x97V[a\x01ca\x01\xF56`\x04a\x13\x19V[a\x02\xA8V[a\x01ca\x02\x086`\x04a\x12WV[a\x02\xB3V[a\x01ca\x02\x1B6`\x04a\x12\xE4V[a\x02\xC4V[`\0a\x021\x88\x88\x88\x88\x88\x88\x88a\x02\xD2V[\x98\x97PPPPPPPPV[`\0a\x02L\x86\x86\x86\x86\x86a\x03PV[\x96\x95PPPPPPV[`\0a\x02d\x85\x85\x85\x85a\x04\x13V[\x95\x94PPPPPV[`\0a\x02d\x85\x85\x85\x85a\x04CV[`\0a\x02d\x85\x85\x85\x85a\x04\xBDV[`\0a\x02d\x85\x85\x85\x85a\x05\x19V[`\0a\x02\xA2\x82a\x05\x8CV[\x92\x91PPV[`\0a\x02\xA2\x82a\x05\xA0V[`\0a\x021\x88\x88\x88\x88\x88\x88\x88a\x05\xB4V[`\0a\x02d\x85\x85\x85\x85a\x05\xF8V[`\0\x80a\x02\xE8\x84a\x02\xE3\x87\x89a\x13KV[a\x06vV[\x90P`\0a\x03+a\x03\x13a\x03\x0Ea\x02\xFF\x8B\x8Ea\x13KV[a\x03\t\x8A\x8Ca\x13KV[a\x06\x92V[a\x06\xA7V[a\x03\x1C\x86a\x13^V[a\x03&\x91\x90a\x13zV[a\x07DV[\x90P\x88a\x038\x83\x83a\x06vV[a\x03B\x91\x90a\x13zV[\x9A\x99PPPPPPPPPPV[`\0\x80a\x03na\x03h\x85g\x1B\xC1mgN\xC8\0\0a\x07\xADV[\x84a\x06vV[\x90P`\0a\x03\x84g\x06\xF0[Y\xD3\xB2\0\0\x83a\x06vV[\x90P`\0a\x03\x91\x85a\x07\xD9V[a\x03\x9F\x90c;\x9A\xCA\0a\x13\xA1V[\x90P`\0a\x03\xAD\x87\x83a\x06vV[\x90P`\0a\x03\xBB\x8B\x8Ba\x06\x92V[\x90Pa\x04\x04\x89a\x03\xFF\x86g\r\xE0\xB6\xB3\xA7d\0\0\x86a\x03\xDCa\x03\x0E\x88\x84a\x13\xB8V[a\x03\xE6\x91\x90a\x13\xCBV[a\x03\xF0\x91\x90a\x14\x11V[a\x03\xFA\x91\x90a\x13zV[a\x08}V[a\n+V[\x9B\x9APPPPPPPPPPPV[`\0a\x04,a\x04\"\x85\x84a\x06vV[a\x03\x0E\x90\x85a\x14?V[a\x049a\x03\x0E\x86\x88a\x14?V[a\x02d\x91\x90a\x14SV[`\0\x80a\x04Xa\x04S\x86\x86a\n@V[a\nUV[\x90P`\0a\x04zg\x06\xF0[Y\xD3\xB2\0\0a\x03\xFF\x86g\x1B\xC1mgN\xC8\0\0a\x07\xADV[\x90P`\0a\x04\xA8\x85a\x04\x8C\x84\x86a\x13zV[a\x04\x9E\x90g\r\xE0\xB6\xB3\xA7d\0\0a\x13\xCBV[a\x03&\x91\x90a\x14\x11V[\x90Pa\x021\x88a\x04\xB8\x88\x84a\n+V[a\n@V[`\0\x80a\x04\xCDa\x04S\x86\x86a\n@V[\x90P`\0a\x04\xEFg\x06\xF0[Y\xD3\xB2\0\0a\x03\xFF\x86g\x1B\xC1mgN\xC8\0\0a\x07\xADV[\x90P`\0a\x05\x01\x85a\x04\x8C\x84\x86a\x14SV[\x90Pa\x021\x88a\x03\xFF\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x13zV[`\0\x80a\x05)a\x04S\x86\x86a\n@V[\x90P`\0a\x05Kg\x06\xF0[Y\xD3\xB2\0\0a\x03\xFF\x86g\x1B\xC1mgN\xC8\0\0a\x07\xADV[\x90P`\0a\x05]\x85a\x04\x8C\x84\x86a\x14SV[\x90P`\0a\x05s\x82g\r\xE0\xB6\xB3\xA7d\0\0a\x13zV[\x90Pa\x05\x7F\x89\x82a\n@V[\x99\x98PPPPPPPPPV[`\0a\x02\xA2g\r\xE0\xB6\xB3\xA7d\0\0\x83a\x13\xA1V[`\0a\x02\xA2g\r\xE0\xB6\xB3\xA7d\0\0\x83a\x14?V[`\0\x80a\x05\xC5\x84a\x02\xE3\x87\x89a\x13KV[\x90P`\0a\x05\xE2a\x03\x13a\x03\x0Ea\x05\xDC\x8B\x8Da\x13KV[\x85a\x06\x92V[\x90P\x89a\x038a\x05\xF2\x88\x8Aa\x13KV[\x83a\x06vV[`\0\x80a\x06\x08a\x04S\x86\x86a\n@V[\x90P`\0a\x06*g\x06\xF0[Y\xD3\xB2\0\0a\x03\xFF\x86g\x1B\xC1mgN\xC8\0\0a\x07\xADV[\x90P`\0a\x068\x82\x84a\x13zV[\x90P`\0\x85a\x06O\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x13\xCBV[a\x06Y\x91\x90a\x14\x11V[\x90P`\0a\x06f\x82a\x07DV[\x90Pa\x03B\x88a\x03\xFF\x8C\x84a\n+V[`\0a\x06\x8B\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x0C0V[\x93\x92PPPV[`\0a\x06\x8B\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x0C0V[`\0g\x06\xF0[Y\xD3\xB2\0\0\x82\x03a\x06\xC0WP`\0\x91\x90PV[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x12a\x06\xE8W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\x07\tW`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x07\x16\x83`\x02a\x13\xCBV[\x90P`\0a\x07#\x82a\x0COV[\x90P`\0a\x079g\x13\xA0K\xBD\xFD\xC9\xBE\x88\x83a\x0E\xCDV[\x90Pa\x02d\x81a\x13^V[`\0\x80g\x13\xA0K\xBD\xFD\xC9\xBE\x88a\x07bg\r\xE0\xB6\xB3\xA7d\0\0\x85a\x13\xCBV[a\x07l\x91\x90a\x14\x11V[\x90P`\0a\x07y\x82a\x13^V[\x90P`\0a\x07\x86\x82a\x0E\xE2V[\x90Pg\x1B\xC1mgN\xC8\0\0a\x07\xA3g\r\xE0\xB6\xB3\xA7d\0\0\x83a\x13\xCBV[a\x02d\x91\x90a\x14\x11V[`\0a\x06\x8Bg\r\xE0\xB6\xB3\xA7d\0\0\x83a\x07\xC5\x86a\nUV[a\x07\xCF\x91\x90a\x13\xCBV[a\x03\xFA\x91\x90a\x14\x11V[`\xB5\x81`\x01`\x88\x1B\x81\x10a\x07\xF2W`@\x91\x90\x91\x1B\x90`\x80\x1C[i\x01\0\0\0\0\0\0\0\0\0\x81\x10a\x08\x0EW` \x91\x90\x91\x1B\x90`@\x1C[e\x01\0\0\0\0\0\x81\x10a\x08&W`\x10\x91\x90\x91\x1B\x90` \x1C[c\x01\0\0\0\x81\x10a\x08<W`\x08\x91\x90\x91\x1B\x90`\x10\x1C[b\x01\0\0\x01\x02`\x12\x1C\x80\x82\x04\x01`\x01\x90\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x90\x81\x90\x04\x81\x11\x90\x03\x90V[`\0h\x02H\xCE6\xA7\x0C\xB2k>\x19\x82\x13a\x08\x98WP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a\x08\xE4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05`\x01`_\x1B\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1Dm\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1Dn\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[`\0a\x06\x8B\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x10\xC6V[`\0a\x06\x8B\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x10\xC6V[`\0\x80\x82\x13a\n\x92W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x08\xDBV[`\0``a\n\x9F\x84a\x10\xF4V[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x0CHW`\0\x80\xFD[\x04\x92\x91PPV[`\0\x80\x82\x12\x80a\x0CfWPg\x1B\xC1mgN\xC8\0\0\x82\x13[\x15a\x0C\x84W`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\x0C\xA5W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81g\x1B\xC1mgN\xC8\0\0\x03a\x0C\xCDW`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x15a\x0C\xD8W\x91\x90PV[`\0g\r\xE0\xB6\xB3\xA7d\0\0\x83\x12a\r\0Wa\x0C\xFB\x83g\x1B\xC1mgN\xC8\0\0a\x13zV[a\r\x02V[\x82[\x90P`\0a\r\x18\x82g\x1B\xC1mgN\xC8\0\0a\x11\x9CV[\x90P\x80`\0\x03a\r;W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\rF\x82a\nUV[\x90P`\0c;\x9A\xCA\0a\rqa\rla\rfg\x1B\xC1mgN\xC8\0\0a\x13^V[\x85a\x0E\xCDV[a\x07\xD9V[a\r{\x91\x90a\x13\xCBV[\x90P`\0\x80a\r\x92\x83g\x03\xC1f\\z\xAB \0a\x0E\xCDV[a\r\xA4\x90g \x05\xFEO&\x8E\xA0\0a\x14SV[\x90P`\0a\r\xD4\x84a\r\xBD\x86f\x9F2u$b\xA0\0a\x0E\xCDV[a\r\xCF\x90g\r\xC5R\x7Fd, \0a\x14SV[a\x0E\xCDV[a\r\xE6\x90g\r\xE0\xB6\xB3\xA7d\0\0a\x14SV[\x90Pa\x0E\ng\t\xD0(\xCCo _\xFF\x19\x85a\x0E\0\x85\x85a\x11\x9CV[a\r\xCF\x91\x90a\x13zV[\x92PPP`\0[`\x02\x81\x10\x15a\x0E\xA5W`\0\x86a\x0E&\x84a\x0E\xE2V[a\x0E0\x91\x90a\x13zV[\x90P`\0a\x0E>\x84\x85a\x0E\xCDV[a\x0EG\x90a\x13^V[\x90P`\0a\x0ET\x82a\x08}V[\x90P`\0a\x0Eb\x86\x85a\x0E\xCDV[a\x0Etg\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x84a\x0E\xCDV[a\x0E~\x91\x90a\x13zV[\x90Pa\x0E\x8A\x84\x82a\x11\x9CV[a\x0E\x94\x90\x87a\x14SV[\x95P\x84`\x01\x01\x94PPPPPa\x0E\x11V[g\r\xE0\xB6\xB3\xA7d\0\0\x88\x12a\x0E\xC2Wa\x0E\xBD\x82a\x13^V[a\x021V[P\x96\x95PPPPPPV[`\0a\x06\x8B\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x11\xADV[`\0\x81`\0\x03a\x0E\xFBWPg\r\xE0\xB6\xB3\xA7d\0\0\x91\x90PV[gV\x98\xEE\xF0fp\0\0\x82\x12a\x0F\x12WP`\0\x91\x90PV[a\x0F#gV\x98\xEE\xF0fp\0\0a\x13^V[\x82\x13a\x0F8WPg\x1B\xC1mgN\xC8\0\0\x91\x90PV[`\0a\x0FC\x83a\x11\xCCV[\x90P`\0a\x0F|g\r\xE0\xB6\xB3\xA7d\0\0a\x0Fe\x84g\x1B\xC1mgN\xC8\0\0a\x06\x92V[a\x0Fw\x90g\r\xE0\xB6\xB3\xA7d\0\0a\x14SV[a\x11\x9CV[\x90P`\0\x80\x82a\x0F\xD8\x81a\x0F\xC5\x81a\x0F\xB3\x81a\x0F\xA0\x81g\x02_\x0F\xE1\x05\xA3\x14\0a\x0E\xCDV[a\r\xCF\x90g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19a\x14SV[a\r\xCF\x90g\x14\xA8EL\x19\xE1\xAC\0a\x14SV[a\r\xCF\x90g\x0F\xC1\x0E\x01W\x82w\xFF\x19a\x14SV[a\x0F\xEA\x90g\x03\xDE\xBD\x08;\x8C|\0a\x14SV[\x91P\x83\x90Pa\x10R\x81a\x10@\x81a\x10.\x81a\x10\x1C\x81a\x10\t\x81\x8Ba\x0E\xCDV[a\r\xCF\x90g\x02\x95\xD4\0\xEA2W\xFF\x19a\x14SV[a\r\xCF\x90g\x01W\xD8\xB2\xEC\xC7\x08\0a\x14SV[a\r\xCF\x90g\x051\n\xA7\xD5!0\0a\x14SV[a\r\xCF\x90g\r\xE0\xCC=\x15a\0\0a\x14SV[\x91P\x81g\x11\x90\0\xAB\x10\x0F\xFC\0a\x10h\x87\x88a\x0E\xCDV[a\x10t\x90`\0\x19a\x13\xCBV[a\x10~\x91\x90a\x13zV[a\x10\x88\x91\x90a\x14SV[\x92PP`\0a\x10\x96\x83a\x08}V[\x90P`\0a\x10\xA4\x85\x83a\x0E\xCDV[\x90P`\0\x88\x12a\x10\xB4W\x80a\x021V[a\x021\x81g\x1B\xC1mgN\xC8\0\0a\x13zV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x10\xDEW`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[`\0\x80\x82\x11a\x111W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x08\xDBV[P`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x82\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[`\0a\x06\x8B\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x05\x85\x14\x17\x16a\x11\xC5W`\0\x80\xFD[\x05\x92\x91PPV[`\0`\x01`\xFF\x1B\x82\x03a\x11\xF2W`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x12\x15a\x12\x03WP\x19`\x01\x01\x90V[P\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`\0\x80`\0\x80`\0\x80`\0`\xE0\x88\x8A\x03\x12\x15a\x12uWa\x12ua\x12\x07V[PP\x855\x97` \x87\x015\x97P`@\x87\x015\x96``\x81\x015\x96P`\x80\x81\x015\x95P`\xA0\x81\x015\x94P`\xC0\x015\x92P\x90PV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\x12\xC1Wa\x12\xC1a\x12\x07V[PP\x835\x95` \x85\x015\x95P`@\x85\x015\x94``\x81\x015\x94P`\x80\x015\x92P\x90PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x12\xFDWa\x12\xFDa\x12\x07V[PP\x825\x94` \x84\x015\x94P`@\x84\x015\x93``\x015\x92P\x90PV[`\0` \x82\x84\x03\x12\x15a\x13.Wa\x13.a\x12\x07V[P5\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x02\xA2Wa\x02\xA2a\x135V[`\0`\x01`\xFF\x1B\x82\x01a\x13sWa\x13sa\x135V[P`\0\x03\x90V[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a\x13\x9AWa\x13\x9Aa\x135V[P\x92\x91PPV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x02\xA2Wa\x02\xA2a\x135V[\x81\x81\x03\x81\x81\x11\x15a\x02\xA2Wa\x02\xA2a\x135V[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15a\x13\xE7Wa\x13\xE7a\x135V[\x81\x81\x05\x83\x14\x82\x15\x17a\x02\xA2Wa\x02\xA2a\x135V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a\x14 Wa\x14 a\x13\xFBV[`\x01`\xFF\x1B\x82\x14`\0\x19\x84\x14\x16\x15a\x14:Wa\x14:a\x135V[P\x05\x90V[`\0\x82a\x14NWa\x14Na\x13\xFBV[P\x04\x90V[\x80\x82\x01\x82\x81\x12`\0\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a\x14sWa\x14sa\x135V[PP\x92\x91PPV";
    /// The deployed bytecode of the contract.
    pub static RMMMATHLIKE_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct RMMMathLike<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for RMMMathLike<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for RMMMathLike<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for RMMMathLike<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for RMMMathLike<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(RMMMathLike))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> RMMMathLike<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    RMMMATHLIKE_ABI.clone(),
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
                RMMMATHLIKE_ABI.clone(),
                RMMMATHLIKE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `computeInvariant` (0x8dfb85c6) function
        pub fn compute_invariant(
            &self,
            reserve_x: ::ethers::core::types::U256,
            liquidity: ::ethers::core::types::U256,
            reserve_y: ::ethers::core::types::U256,
            strike_price: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash(
                    [141, 251, 133, 198],
                    (reserve_x, liquidity, reserve_y, strike_price),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `computeLGivenX` (0xb0634353) function
        pub fn compute_l_given_x(
            &self,
            x: ::ethers::core::types::U256,
            s: ::ethers::core::types::U256,
            k: ::ethers::core::types::U256,
            sigma: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([176, 99, 67, 83], (x, s, k, sigma))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `computeLGivenY` (0x8fe9c176) function
        pub fn compute_l_given_y(
            &self,
            y: ::ethers::core::types::U256,
            s: ::ethers::core::types::U256,
            k: ::ethers::core::types::U256,
            sigma: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([143, 233, 193, 118], (y, s, k, sigma))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `computeOutputXGivenY` (0xe2d69a49) function
        pub fn compute_output_x_given_y(
            &self,
            x: ::ethers::core::types::U256,
            y: ::ethers::core::types::U256,
            delta_y: ::ethers::core::types::U256,
            l: ::ethers::core::types::U256,
            delta_l: ::ethers::core::types::U256,
            k: ::ethers::core::types::U256,
            sigma: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([226, 214, 154, 73], (x, y, delta_y, l, delta_l, k, sigma))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `computeOutputYGivenX` (0x12988f24) function
        pub fn compute_output_y_given_x(
            &self,
            x: ::ethers::core::types::U256,
            y: ::ethers::core::types::U256,
            delta_x: ::ethers::core::types::U256,
            l: ::ethers::core::types::U256,
            delta_l: ::ethers::core::types::U256,
            k: ::ethers::core::types::U256,
            sigma: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([18, 152, 143, 36], (x, y, delta_x, l, delta_l, k, sigma))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `computeSpotPrice` (0x2ccc54cc) function
        pub fn compute_spot_price(
            &self,
            x: ::ethers::core::types::U256,
            l: ::ethers::core::types::U256,
            k: ::ethers::core::types::U256,
            sigma: ::ethers::core::types::U256,
            tau: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([44, 204, 84, 204], (x, l, k, sigma, tau))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `computeXGivenL` (0xafe1088e) function
        pub fn compute_x_given_l(
            &self,
            l: ::ethers::core::types::U256,
            s: ::ethers::core::types::U256,
            k: ::ethers::core::types::U256,
            sigma: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([175, 225, 8, 142], (l, s, k, sigma))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `computeYGivenL` (0xfd1187f7) function
        pub fn compute_y_given_l(
            &self,
            l: ::ethers::core::types::U256,
            s: ::ethers::core::types::U256,
            k: ::ethers::core::types::U256,
            sigma: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([253, 17, 135, 247], (l, s, k, sigma))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `fromWad` (0xb3676d6b) function
        pub fn from_wad(
            &self,
            a: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([179, 103, 109, 107], a)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `toWad` (0xb2c6da77) function
        pub fn to_wad(
            &self,
            a: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([178, 198, 218, 119], a)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for RMMMathLike<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
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
    pub enum RMMMathLikeErrors {
        Infinity(Infinity),
        Min(Min),
        NegativeInfinity(NegativeInfinity),
        OutOfBounds(OutOfBounds),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for RMMMathLikeErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <Infinity as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Infinity(decoded));
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
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for RMMMathLikeErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::Infinity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Min(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NegativeInfinity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OutOfBounds(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for RMMMathLikeErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <Infinity as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <Min as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NegativeInfinity as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OutOfBounds as ::ethers::contract::EthError>::selector() => true,
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for RMMMathLikeErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Infinity(element) => ::core::fmt::Display::fmt(element, f),
                Self::Min(element) => ::core::fmt::Display::fmt(element, f),
                Self::NegativeInfinity(element) => ::core::fmt::Display::fmt(element, f),
                Self::OutOfBounds(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for RMMMathLikeErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<Infinity> for RMMMathLikeErrors {
        fn from(value: Infinity) -> Self {
            Self::Infinity(value)
        }
    }
    impl ::core::convert::From<Min> for RMMMathLikeErrors {
        fn from(value: Min) -> Self {
            Self::Min(value)
        }
    }
    impl ::core::convert::From<NegativeInfinity> for RMMMathLikeErrors {
        fn from(value: NegativeInfinity) -> Self {
            Self::NegativeInfinity(value)
        }
    }
    impl ::core::convert::From<OutOfBounds> for RMMMathLikeErrors {
        fn from(value: OutOfBounds) -> Self {
            Self::OutOfBounds(value)
        }
    }
    ///Container type for all input parameters for the `computeInvariant` function with signature `computeInvariant(uint256,uint256,uint256,uint256)` and selector `0x8dfb85c6`
    #[derive(
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
        name = "computeInvariant",
        abi = "computeInvariant(uint256,uint256,uint256,uint256)"
    )]
    pub struct ComputeInvariantCall {
        pub reserve_x: ::ethers::core::types::U256,
        pub liquidity: ::ethers::core::types::U256,
        pub reserve_y: ::ethers::core::types::U256,
        pub strike_price: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `computeLGivenX` function with signature `computeLGivenX(uint256,uint256,uint256,uint256)` and selector `0xb0634353`
    #[derive(
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
        name = "computeLGivenX",
        abi = "computeLGivenX(uint256,uint256,uint256,uint256)"
    )]
    pub struct ComputeLGivenXCall {
        pub x: ::ethers::core::types::U256,
        pub s: ::ethers::core::types::U256,
        pub k: ::ethers::core::types::U256,
        pub sigma: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `computeLGivenY` function with signature `computeLGivenY(uint256,uint256,uint256,uint256)` and selector `0x8fe9c176`
    #[derive(
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
        name = "computeLGivenY",
        abi = "computeLGivenY(uint256,uint256,uint256,uint256)"
    )]
    pub struct ComputeLGivenYCall {
        pub y: ::ethers::core::types::U256,
        pub s: ::ethers::core::types::U256,
        pub k: ::ethers::core::types::U256,
        pub sigma: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `computeOutputXGivenY` function with signature `computeOutputXGivenY(uint256,uint256,uint256,uint256,uint256,uint256,uint256)` and selector `0xe2d69a49`
    #[derive(
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
        name = "computeOutputXGivenY",
        abi = "computeOutputXGivenY(uint256,uint256,uint256,uint256,uint256,uint256,uint256)"
    )]
    pub struct ComputeOutputXGivenYCall {
        pub x: ::ethers::core::types::U256,
        pub y: ::ethers::core::types::U256,
        pub delta_y: ::ethers::core::types::U256,
        pub l: ::ethers::core::types::U256,
        pub delta_l: ::ethers::core::types::U256,
        pub k: ::ethers::core::types::U256,
        pub sigma: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `computeOutputYGivenX` function with signature `computeOutputYGivenX(uint256,uint256,uint256,uint256,uint256,uint256,uint256)` and selector `0x12988f24`
    #[derive(
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
        name = "computeOutputYGivenX",
        abi = "computeOutputYGivenX(uint256,uint256,uint256,uint256,uint256,uint256,uint256)"
    )]
    pub struct ComputeOutputYGivenXCall {
        pub x: ::ethers::core::types::U256,
        pub y: ::ethers::core::types::U256,
        pub delta_x: ::ethers::core::types::U256,
        pub l: ::ethers::core::types::U256,
        pub delta_l: ::ethers::core::types::U256,
        pub k: ::ethers::core::types::U256,
        pub sigma: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `computeSpotPrice` function with signature `computeSpotPrice(uint256,uint256,uint256,uint256,uint256)` and selector `0x2ccc54cc`
    #[derive(
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
        name = "computeSpotPrice",
        abi = "computeSpotPrice(uint256,uint256,uint256,uint256,uint256)"
    )]
    pub struct ComputeSpotPriceCall {
        pub x: ::ethers::core::types::U256,
        pub l: ::ethers::core::types::U256,
        pub k: ::ethers::core::types::U256,
        pub sigma: ::ethers::core::types::U256,
        pub tau: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `computeXGivenL` function with signature `computeXGivenL(uint256,uint256,uint256,uint256)` and selector `0xafe1088e`
    #[derive(
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
        name = "computeXGivenL",
        abi = "computeXGivenL(uint256,uint256,uint256,uint256)"
    )]
    pub struct ComputeXGivenLCall {
        pub l: ::ethers::core::types::U256,
        pub s: ::ethers::core::types::U256,
        pub k: ::ethers::core::types::U256,
        pub sigma: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `computeYGivenL` function with signature `computeYGivenL(uint256,uint256,uint256,uint256)` and selector `0xfd1187f7`
    #[derive(
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
        name = "computeYGivenL",
        abi = "computeYGivenL(uint256,uint256,uint256,uint256)"
    )]
    pub struct ComputeYGivenLCall {
        pub l: ::ethers::core::types::U256,
        pub s: ::ethers::core::types::U256,
        pub k: ::ethers::core::types::U256,
        pub sigma: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `fromWad` function with signature `fromWad(uint256)` and selector `0xb3676d6b`
    #[derive(
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
    #[ethcall(name = "fromWad", abi = "fromWad(uint256)")]
    pub struct FromWadCall {
        pub a: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `toWad` function with signature `toWad(uint256)` and selector `0xb2c6da77`
    #[derive(
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
    #[ethcall(name = "toWad", abi = "toWad(uint256)")]
    pub struct ToWadCall {
        pub a: ::ethers::core::types::U256,
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
    pub enum RMMMathLikeCalls {
        ComputeInvariant(ComputeInvariantCall),
        ComputeLGivenX(ComputeLGivenXCall),
        ComputeLGivenY(ComputeLGivenYCall),
        ComputeOutputXGivenY(ComputeOutputXGivenYCall),
        ComputeOutputYGivenX(ComputeOutputYGivenXCall),
        ComputeSpotPrice(ComputeSpotPriceCall),
        ComputeXGivenL(ComputeXGivenLCall),
        ComputeYGivenL(ComputeYGivenLCall),
        FromWad(FromWadCall),
        ToWad(ToWadCall),
    }
    impl ::ethers::core::abi::AbiDecode for RMMMathLikeCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <ComputeInvariantCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ComputeInvariant(decoded));
            }
            if let Ok(decoded) = <ComputeLGivenXCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ComputeLGivenX(decoded));
            }
            if let Ok(decoded) = <ComputeLGivenYCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ComputeLGivenY(decoded));
            }
            if let Ok(decoded) = <ComputeOutputXGivenYCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ComputeOutputXGivenY(decoded));
            }
            if let Ok(decoded) = <ComputeOutputYGivenXCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ComputeOutputYGivenX(decoded));
            }
            if let Ok(decoded) = <ComputeSpotPriceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ComputeSpotPrice(decoded));
            }
            if let Ok(decoded) = <ComputeXGivenLCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ComputeXGivenL(decoded));
            }
            if let Ok(decoded) = <ComputeYGivenLCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ComputeYGivenL(decoded));
            }
            if let Ok(decoded) = <FromWadCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FromWad(decoded));
            }
            if let Ok(decoded) = <ToWadCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ToWad(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for RMMMathLikeCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::ComputeInvariant(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ComputeLGivenX(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ComputeLGivenY(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ComputeOutputXGivenY(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ComputeOutputYGivenX(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ComputeSpotPrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ComputeXGivenL(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ComputeYGivenL(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FromWad(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ToWad(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for RMMMathLikeCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ComputeInvariant(element) => ::core::fmt::Display::fmt(element, f),
                Self::ComputeLGivenX(element) => ::core::fmt::Display::fmt(element, f),
                Self::ComputeLGivenY(element) => ::core::fmt::Display::fmt(element, f),
                Self::ComputeOutputXGivenY(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ComputeOutputYGivenX(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ComputeSpotPrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::ComputeXGivenL(element) => ::core::fmt::Display::fmt(element, f),
                Self::ComputeYGivenL(element) => ::core::fmt::Display::fmt(element, f),
                Self::FromWad(element) => ::core::fmt::Display::fmt(element, f),
                Self::ToWad(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ComputeInvariantCall> for RMMMathLikeCalls {
        fn from(value: ComputeInvariantCall) -> Self {
            Self::ComputeInvariant(value)
        }
    }
    impl ::core::convert::From<ComputeLGivenXCall> for RMMMathLikeCalls {
        fn from(value: ComputeLGivenXCall) -> Self {
            Self::ComputeLGivenX(value)
        }
    }
    impl ::core::convert::From<ComputeLGivenYCall> for RMMMathLikeCalls {
        fn from(value: ComputeLGivenYCall) -> Self {
            Self::ComputeLGivenY(value)
        }
    }
    impl ::core::convert::From<ComputeOutputXGivenYCall> for RMMMathLikeCalls {
        fn from(value: ComputeOutputXGivenYCall) -> Self {
            Self::ComputeOutputXGivenY(value)
        }
    }
    impl ::core::convert::From<ComputeOutputYGivenXCall> for RMMMathLikeCalls {
        fn from(value: ComputeOutputYGivenXCall) -> Self {
            Self::ComputeOutputYGivenX(value)
        }
    }
    impl ::core::convert::From<ComputeSpotPriceCall> for RMMMathLikeCalls {
        fn from(value: ComputeSpotPriceCall) -> Self {
            Self::ComputeSpotPrice(value)
        }
    }
    impl ::core::convert::From<ComputeXGivenLCall> for RMMMathLikeCalls {
        fn from(value: ComputeXGivenLCall) -> Self {
            Self::ComputeXGivenL(value)
        }
    }
    impl ::core::convert::From<ComputeYGivenLCall> for RMMMathLikeCalls {
        fn from(value: ComputeYGivenLCall) -> Self {
            Self::ComputeYGivenL(value)
        }
    }
    impl ::core::convert::From<FromWadCall> for RMMMathLikeCalls {
        fn from(value: FromWadCall) -> Self {
            Self::FromWad(value)
        }
    }
    impl ::core::convert::From<ToWadCall> for RMMMathLikeCalls {
        fn from(value: ToWadCall) -> Self {
            Self::ToWad(value)
        }
    }
    ///Container type for all return fields from the `computeInvariant` function with signature `computeInvariant(uint256,uint256,uint256,uint256)` and selector `0x8dfb85c6`
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
    pub struct ComputeInvariantReturn {
        pub invariant: ::ethers::core::types::I256,
    }
    ///Container type for all return fields from the `computeLGivenX` function with signature `computeLGivenX(uint256,uint256,uint256,uint256)` and selector `0xb0634353`
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
    pub struct ComputeLGivenXReturn {
        pub l: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `computeLGivenY` function with signature `computeLGivenY(uint256,uint256,uint256,uint256)` and selector `0x8fe9c176`
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
    pub struct ComputeLGivenYReturn {
        pub l: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `computeOutputXGivenY` function with signature `computeOutputXGivenY(uint256,uint256,uint256,uint256,uint256,uint256,uint256)` and selector `0xe2d69a49`
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
    pub struct ComputeOutputXGivenYReturn {
        pub output_x: ::ethers::core::types::I256,
    }
    ///Container type for all return fields from the `computeOutputYGivenX` function with signature `computeOutputYGivenX(uint256,uint256,uint256,uint256,uint256,uint256,uint256)` and selector `0x12988f24`
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
    pub struct ComputeOutputYGivenXReturn {
        pub output_y: ::ethers::core::types::I256,
    }
    ///Container type for all return fields from the `computeSpotPrice` function with signature `computeSpotPrice(uint256,uint256,uint256,uint256,uint256)` and selector `0x2ccc54cc`
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
    pub struct ComputeSpotPriceReturn {
        pub spot_price: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `computeXGivenL` function with signature `computeXGivenL(uint256,uint256,uint256,uint256)` and selector `0xafe1088e`
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
    pub struct ComputeXGivenLReturn {
        pub x: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `computeYGivenL` function with signature `computeYGivenL(uint256,uint256,uint256,uint256)` and selector `0xfd1187f7`
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
    pub struct ComputeYGivenLReturn {
        pub y: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `fromWad` function with signature `fromWad(uint256)` and selector `0xb3676d6b`
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
    pub struct FromWadReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `toWad` function with signature `toWad(uint256)` and selector `0xb2c6da77`
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
    pub struct ToWadReturn(pub ::ethers::core::types::U256);
}
