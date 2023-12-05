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
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[Pa\x148\x80a\0m`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\0\xEBW`\x005`\xE0\x1C\x80c\x8D\xFB\x85\xC6\x11a\0\xB3W\x80c\x8D\xFB\x85\xC6\x14a\x01\xC1W\x80c\xB2\xC6\xDAw\x14a\x01\xD4W\x80c\xB3gmk\x14a\x01\xE7W\x80c\xCF%\xC3\xEF\x14a\x01\xFAW\x80c\xE3\x90\x8Fm\x14a\x02\rWa\0\xEBV[\x80c!p\xEE6\x14a\x01PW\x80c,\xCCT\xCC\x14a\x01uW\x80cP\xE3\x04\xE5\x14a\x01\x88W\x80cX\0\xD4$\x14a\x01\x9BW\x80cg\x16\x13\xAA\x14a\x01\xAEW[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x01ca\x01^6`\x04a\x12\x0BV[a\x02 V[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\x01ca\x01\x836`\x04a\x12\x0BV[a\x029V[a\x01ca\x01\x966`\x04a\x12\x0BV[a\x02HV[a\x01ca\x01\xA96`\x04a\x12\x0BV[a\x02WV[a\x01ca\x01\xBC6`\x04a\x12\x0BV[a\x02fV[a\x01ca\x01\xCF6`\x04a\x12IV[a\x02uV[a\x01ca\x01\xE26`\x04a\x12~V[a\x02\x8CV[a\x01ca\x01\xF56`\x04a\x12~V[a\x02\x9DV[a\x01ca\x02\x086`\x04a\x12\x9AV[a\x02\xA8V[a\x01ca\x02\x1B6`\x04a\x12\x9AV[a\x02\xC7V[`\0a\x02/\x86\x86\x86\x86\x86a\x02\xD9V[\x96\x95PPPPPPV[`\0a\x02/\x86\x86\x86\x86\x86a\x03\x18V[`\0a\x02/\x86\x86\x86\x86\x86a\x03\x8AV[`\0a\x02/\x86\x86\x86\x86\x86a\x03\xBAV[`\0a\x02/\x86\x86\x86\x86\x86a\x03\xD0V[`\0a\x02\x83\x85\x85\x85\x85a\x03\xF2V[\x95\x94PPPPPV[`\0a\x02\x97\x82a\x04\"V[\x92\x91PPV[`\0a\x02\x97\x82a\x046V[`\0a\x02\xBA\x89\x89\x89\x89\x89\x89\x89\x89a\x04JV[\x99\x98PPPPPPPPPV[`\0a\x02\xBA\x89\x89\x89\x89\x89\x89\x89\x89a\x04\xCEV[`\0\x80a\x02\xF0a\x02\xEB\x87\x87\x87\x87a\x05\"V[a\x05\x80V[\x90Pa\x03\r\x87a\x03\x08\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x13\x08V[a\x05\xE9V[\x97\x96PPPPPPPV[`\0\x80a\x03%\x84\x84a\x06\x05V[\x90P`\0a\x033\x85\x85a\x063V[\x90P`\0a\x03A\x89\x89a\x06eV[\x90Pa\x02\xBA\x87a\x03\x08\x84g\r\xE0\xB6\xB3\xA7d\0\0\x87a\x03ga\x03b\x88\x84a\x13/V[a\x06zV[a\x03q\x91\x90a\x13BV[a\x03{\x91\x90a\x13\x88V[a\x03\x85\x91\x90a\x13\x08V[a\x07\x17V[`\0\x80a\x03\x9Ca\x02\xEB\x87\x87\x87\x87a\x05\"V[a\x03\xAE\x90g\r\xE0\xB6\xB3\xA7d\0\0a\x13\x08V[\x90Pa\x03\r\x87\x82a\x08\xC5V[`\0\x80a\x03\xAE\x85a\x03\x08a\x02\xEB\x89\x89\x89\x89a\x08\xDAV[`\0\x80a\x03\xE2a\x02\xEB\x87\x87\x87\x87a\x08\xDAV[\x90Pa\x03\r\x85a\x03\x08\x89\x84a\x05\xE9V[`\0a\x04\x0Ba\x04\x01\x85\x84a\t\x10V[a\x03b\x90\x85a\x13\xB6V[a\x04\x18a\x03b\x86\x88a\x13\xB6V[a\x02\x83\x91\x90a\x13\xCAV[`\0a\x02\x97g\r\xE0\xB6\xB3\xA7d\0\0\x83a\x13\xF2V[`\0a\x02\x97g\r\xE0\xB6\xB3\xA7d\0\0\x83a\x13\xB6V[`\0\x80a\x04W\x84\x84a\x06\x05V[\x90P`\0a\x04n\x86a\x04i\x89\x8Ba\x14\tV[a\t\x10V[\x90P`\0a\x04\x9Ea\x04\x8Ba\x03ba\x04\x85\x8D\x8Fa\x14\tV[\x85a\x06eV[a\x04\x94\x85a\x14\x1CV[a\x02\xEB\x91\x90a\x13\x08V[\x90P\x8Ba\x04\xB4a\x04\xAE\x8A\x8Ca\x14\tV[\x83a\t\x10V[a\x04\xBE\x91\x90a\x13\x08V[\x9C\x9BPPPPPPPPPPPPV[`\0\x80a\x04\xDB\x84\x84a\x06\x05V[\x90P`\0a\x04\xED\x86a\x04i\x89\x8Ba\x14\tV[\x90P`\0a\x05\x15a\x04\x8Ba\x03b\x8C\x8Fa\x05\x06\x91\x90a\x14\tV[a\x05\x10\x8C\x8Ea\x14\tV[a\x06eV[\x90P\x8Aa\x04\xB4\x83\x83a\t\x10V[`\0\x80a\x05/\x84\x84a\x06\x05V[\x90P`\0a\x05=\x87\x87a\t%V[\x90P`\0a\x05K\x86\x86a\x063V[\x90P\x82a\x05X\x82\x84a\x13\xCAV[a\x05j\x90g\r\xE0\xB6\xB3\xA7d\0\0a\x13BV[a\x05t\x91\x90a\x13\x88V[\x98\x97PPPPPPPPV[`\0\x80g\x13\xA0K\xBD\xFD\xC9\xBE\x88a\x05\x9Eg\r\xE0\xB6\xB3\xA7d\0\0\x85a\x13BV[a\x05\xA8\x91\x90a\x13\x88V[\x90P`\0a\x05\xB5\x82a\x14\x1CV[\x90P`\0a\x05\xC2\x82a\t9V[\x90Pg\x1B\xC1mgN\xC8\0\0a\x05\xDFg\r\xE0\xB6\xB3\xA7d\0\0\x83a\x13BV[a\x02\x83\x91\x90a\x13\x88V[`\0a\x05\xFE\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x0B\"V[\x93\x92PPPV[`\0\x80a\x06\x11\x83a\x0BPV[a\x06\x1F\x90c;\x9A\xCA\0a\x13\xF2V[\x90Pa\x06+\x84\x82a\t\x10V[\x94\x93PPPPV[`\0\x80a\x06Qa\x06K\x85g\x1B\xC1mgN\xC8\0\0a\x0B\xF4V[\x84a\t\x10V[\x90Pa\x06+g\x06\xF0[Y\xD3\xB2\0\0\x82a\t\x10V[`\0a\x05\xFE\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x0C V[`\0g\x06\xF0[Y\xD3\xB2\0\0\x82\x03a\x06\x93WP`\0\x91\x90PV[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x12a\x06\xBBW`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\x06\xDCW`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x06\xE9\x83`\x02a\x13BV[\x90P`\0a\x06\xF6\x82a\x0C?V[\x90P`\0a\x07\x0Cg\x13\xA0K\xBD\xFD\xC9\xBE\x88\x83a\x0E\xB8V[\x90Pa\x02\x83\x81a\x14\x1CV[`\0h\x02H\xCE6\xA7\x0C\xB2k>\x19\x82\x13a\x072WP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a\x07~W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05`\x01`_\x1B\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1Dm\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1Dn\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[`\0a\x05\xFE\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x0B\"V[`\0\x80a\x08\xE7\x84\x84a\x06\x05V[\x90P`\0a\x08\xF5\x87\x87a\t%V[\x90P`\0a\t\x03\x86\x86a\x063V[\x90P\x82a\x05X\x82\x84a\x13\x08V[`\0a\x05\xFE\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x0C V[`\0a\x05\xFEa\t4\x84\x84a\x08\xC5V[a\x0E\xCDV[`\0\x81`\0\x03a\tRWPg\r\xE0\xB6\xB3\xA7d\0\0\x91\x90PV[gV\x98\xEE\xF0fp\0\0\x82\x12a\tiWP`\0\x91\x90PV[a\tzgV\x98\xEE\xF0fp\0\0a\x14\x1CV[\x82\x13a\t\x8FWPg\x1B\xC1mgN\xC8\0\0\x91\x90PV[`\0a\t\x9A\x83a\x10\xA8V[\x90P`\0a\t\xD3g\r\xE0\xB6\xB3\xA7d\0\0a\t\xBC\x84g\x1B\xC1mgN\xC8\0\0a\x06eV[a\t\xCE\x90g\r\xE0\xB6\xB3\xA7d\0\0a\x13\xCAV[a\x10\xE3V[\x90P`\0\x80\x82a\n4\x81a\n!\x81a\n\x0F\x81a\t\xF7\x81g\x02_\x0F\xE1\x05\xA3\x14\0a\x0E\xB8V[a\n\n\x90g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19a\x13\xCAV[a\x0E\xB8V[a\n\n\x90g\x14\xA8EL\x19\xE1\xAC\0a\x13\xCAV[a\n\n\x90g\x0F\xC1\x0E\x01W\x82w\xFF\x19a\x13\xCAV[a\nF\x90g\x03\xDE\xBD\x08;\x8C|\0a\x13\xCAV[\x91P\x83\x90Pa\n\xAE\x81a\n\x9C\x81a\n\x8A\x81a\nx\x81a\ne\x81\x8Ba\x0E\xB8V[a\n\n\x90g\x02\x95\xD4\0\xEA2W\xFF\x19a\x13\xCAV[a\n\n\x90g\x01W\xD8\xB2\xEC\xC7\x08\0a\x13\xCAV[a\n\n\x90g\x051\n\xA7\xD5!0\0a\x13\xCAV[a\n\n\x90g\r\xE0\xCC=\x15a\0\0a\x13\xCAV[\x91P\x81g\x11\x90\0\xAB\x10\x0F\xFC\0a\n\xC4\x87\x88a\x0E\xB8V[a\n\xD0\x90`\0\x19a\x13BV[a\n\xDA\x91\x90a\x13\x08V[a\n\xE4\x91\x90a\x13\xCAV[\x92PP`\0a\n\xF2\x83a\x07\x17V[\x90P`\0a\x0B\0\x85\x83a\x0E\xB8V[\x90P`\0\x88\x12a\x0B\x10W\x80a\x05tV[a\x05t\x81g\x1B\xC1mgN\xC8\0\0a\x13\x08V[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x0B:W`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[`\xB5\x81`\x01`\x88\x1B\x81\x10a\x0BiW`@\x91\x90\x91\x1B\x90`\x80\x1C[i\x01\0\0\0\0\0\0\0\0\0\x81\x10a\x0B\x85W` \x91\x90\x91\x1B\x90`@\x1C[e\x01\0\0\0\0\0\x81\x10a\x0B\x9DW`\x10\x91\x90\x91\x1B\x90` \x1C[c\x01\0\0\0\x81\x10a\x0B\xB3W`\x08\x91\x90\x91\x1B\x90`\x10\x1C[b\x01\0\0\x01\x02`\x12\x1C\x80\x82\x04\x01`\x01\x90\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x90\x81\x90\x04\x81\x11\x90\x03\x90V[`\0a\x05\xFEg\r\xE0\xB6\xB3\xA7d\0\0\x83a\x0C\x0C\x86a\x0E\xCDV[a\x0C\x16\x91\x90a\x13BV[a\x03\x85\x91\x90a\x13\x88V[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x0C8W`\0\x80\xFD[\x04\x92\x91PPV[`\0\x80\x82\x12\x80a\x0CVWPg\x1B\xC1mgN\xC8\0\0\x82\x13[\x15a\x0CtW`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\x0C\x95W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81g\x1B\xC1mgN\xC8\0\0\x03a\x0C\xBDW`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x15a\x0C\xC8W\x91\x90PV[`\0g\r\xE0\xB6\xB3\xA7d\0\0\x83\x12a\x0C\xF0Wa\x0C\xEB\x83g\x1B\xC1mgN\xC8\0\0a\x13\x08V[a\x0C\xF2V[\x82[\x90P`\0a\r\x08\x82g\x1B\xC1mgN\xC8\0\0a\x10\xE3V[\x90P\x80`\0\x03a\r+W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\r6\x82a\x0E\xCDV[\x90P`\0c;\x9A\xCA\0a\raa\r\\a\rVg\x1B\xC1mgN\xC8\0\0a\x14\x1CV[\x85a\x0E\xB8V[a\x0BPV[a\rk\x91\x90a\x13BV[\x90P`\0\x80a\r\x82\x83g\x03\xC1f\\z\xAB \0a\x0E\xB8V[a\r\x94\x90g \x05\xFEO&\x8E\xA0\0a\x13\xCAV[\x90P`\0a\r\xBF\x84a\r\xAD\x86f\x9F2u$b\xA0\0a\x0E\xB8V[a\n\n\x90g\r\xC5R\x7Fd, \0a\x13\xCAV[a\r\xD1\x90g\r\xE0\xB6\xB3\xA7d\0\0a\x13\xCAV[\x90Pa\r\xF5g\t\xD0(\xCCo _\xFF\x19\x85a\r\xEB\x85\x85a\x10\xE3V[a\n\n\x91\x90a\x13\x08V[\x92PPP`\0[`\x02\x81\x10\x15a\x0E\x90W`\0\x86a\x0E\x11\x84a\t9V[a\x0E\x1B\x91\x90a\x13\x08V[\x90P`\0a\x0E)\x84\x85a\x0E\xB8V[a\x0E2\x90a\x14\x1CV[\x90P`\0a\x0E?\x82a\x07\x17V[\x90P`\0a\x0EM\x86\x85a\x0E\xB8V[a\x0E_g\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x84a\x0E\xB8V[a\x0Ei\x91\x90a\x13\x08V[\x90Pa\x0Eu\x84\x82a\x10\xE3V[a\x0E\x7F\x90\x87a\x13\xCAV[\x95P\x84`\x01\x01\x94PPPPPa\r\xFCV[g\r\xE0\xB6\xB3\xA7d\0\0\x88\x12a\x0E\xADWa\x0E\xA8\x82a\x14\x1CV[a\x05tV[P\x96\x95PPPPPPV[`\0a\x05\xFE\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x10\xF4V[`\0\x80\x82\x13a\x0F\nW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x07uV[`\0``a\x0F\x17\x84a\x11\x13V[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[`\0`\x01`\xFF\x1B\x82\x03a\x10\xCEW`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x12\x15a\x10\xDFWP\x19`\x01\x01\x90V[P\x90V[`\0a\x05\xFE\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x05\x85\x14\x17\x16a\x11\x0CW`\0\x80\xFD[\x05\x92\x91PPV[`\0\x80\x82\x11a\x11PW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x07uV[P`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x82\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\x12&Wa\x12&a\x11\xBBV[PP\x835\x95` \x85\x015\x95P`@\x85\x015\x94``\x81\x015\x94P`\x80\x015\x92P\x90PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x12bWa\x12ba\x11\xBBV[PP\x825\x94` \x84\x015\x94P`@\x84\x015\x93``\x015\x92P\x90PV[`\0` \x82\x84\x03\x12\x15a\x12\x93Wa\x12\x93a\x11\xBBV[P5\x91\x90PV[`\0\x80`\0\x80`\0\x80`\0\x80a\x01\0\x89\x8B\x03\x12\x15a\x12\xBAWa\x12\xBAa\x11\xBBV[PP\x865\x98` \x88\x015\x98P`@\x88\x015\x97``\x81\x015\x97P`\x80\x81\x015\x96P`\xA0\x81\x015\x95P`\xC0\x81\x015\x94P`\xE0\x015\x92P\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a\x13(Wa\x13(a\x12\xF2V[P\x92\x91PPV[\x81\x81\x03\x81\x81\x11\x15a\x02\x97Wa\x02\x97a\x12\xF2V[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15a\x13^Wa\x13^a\x12\xF2V[\x81\x81\x05\x83\x14\x82\x15\x17a\x02\x97Wa\x02\x97a\x12\xF2V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a\x13\x97Wa\x13\x97a\x13rV[`\x01`\xFF\x1B\x82\x14`\0\x19\x84\x14\x16\x15a\x13\xB1Wa\x13\xB1a\x12\xF2V[P\x05\x90V[`\0\x82a\x13\xC5Wa\x13\xC5a\x13rV[P\x04\x90V[\x80\x82\x01\x82\x81\x12`\0\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a\x13\xEAWa\x13\xEAa\x12\xF2V[PP\x92\x91PPV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x02\x97Wa\x02\x97a\x12\xF2V[\x80\x82\x01\x80\x82\x11\x15a\x02\x97Wa\x02\x97a\x12\xF2V[`\0`\x01`\xFF\x1B\x82\x01a\x141Wa\x141a\x12\xF2V[P`\0\x03\x90V";
    /// The bytecode of the contract.
    pub static RMMMATHLIKE_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\0\xEBW`\x005`\xE0\x1C\x80c\x8D\xFB\x85\xC6\x11a\0\xB3W\x80c\x8D\xFB\x85\xC6\x14a\x01\xC1W\x80c\xB2\xC6\xDAw\x14a\x01\xD4W\x80c\xB3gmk\x14a\x01\xE7W\x80c\xCF%\xC3\xEF\x14a\x01\xFAW\x80c\xE3\x90\x8Fm\x14a\x02\rWa\0\xEBV[\x80c!p\xEE6\x14a\x01PW\x80c,\xCCT\xCC\x14a\x01uW\x80cP\xE3\x04\xE5\x14a\x01\x88W\x80cX\0\xD4$\x14a\x01\x9BW\x80cg\x16\x13\xAA\x14a\x01\xAEW[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x01ca\x01^6`\x04a\x12\x0BV[a\x02 V[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\x01ca\x01\x836`\x04a\x12\x0BV[a\x029V[a\x01ca\x01\x966`\x04a\x12\x0BV[a\x02HV[a\x01ca\x01\xA96`\x04a\x12\x0BV[a\x02WV[a\x01ca\x01\xBC6`\x04a\x12\x0BV[a\x02fV[a\x01ca\x01\xCF6`\x04a\x12IV[a\x02uV[a\x01ca\x01\xE26`\x04a\x12~V[a\x02\x8CV[a\x01ca\x01\xF56`\x04a\x12~V[a\x02\x9DV[a\x01ca\x02\x086`\x04a\x12\x9AV[a\x02\xA8V[a\x01ca\x02\x1B6`\x04a\x12\x9AV[a\x02\xC7V[`\0a\x02/\x86\x86\x86\x86\x86a\x02\xD9V[\x96\x95PPPPPPV[`\0a\x02/\x86\x86\x86\x86\x86a\x03\x18V[`\0a\x02/\x86\x86\x86\x86\x86a\x03\x8AV[`\0a\x02/\x86\x86\x86\x86\x86a\x03\xBAV[`\0a\x02/\x86\x86\x86\x86\x86a\x03\xD0V[`\0a\x02\x83\x85\x85\x85\x85a\x03\xF2V[\x95\x94PPPPPV[`\0a\x02\x97\x82a\x04\"V[\x92\x91PPV[`\0a\x02\x97\x82a\x046V[`\0a\x02\xBA\x89\x89\x89\x89\x89\x89\x89\x89a\x04JV[\x99\x98PPPPPPPPPV[`\0a\x02\xBA\x89\x89\x89\x89\x89\x89\x89\x89a\x04\xCEV[`\0\x80a\x02\xF0a\x02\xEB\x87\x87\x87\x87a\x05\"V[a\x05\x80V[\x90Pa\x03\r\x87a\x03\x08\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x13\x08V[a\x05\xE9V[\x97\x96PPPPPPPV[`\0\x80a\x03%\x84\x84a\x06\x05V[\x90P`\0a\x033\x85\x85a\x063V[\x90P`\0a\x03A\x89\x89a\x06eV[\x90Pa\x02\xBA\x87a\x03\x08\x84g\r\xE0\xB6\xB3\xA7d\0\0\x87a\x03ga\x03b\x88\x84a\x13/V[a\x06zV[a\x03q\x91\x90a\x13BV[a\x03{\x91\x90a\x13\x88V[a\x03\x85\x91\x90a\x13\x08V[a\x07\x17V[`\0\x80a\x03\x9Ca\x02\xEB\x87\x87\x87\x87a\x05\"V[a\x03\xAE\x90g\r\xE0\xB6\xB3\xA7d\0\0a\x13\x08V[\x90Pa\x03\r\x87\x82a\x08\xC5V[`\0\x80a\x03\xAE\x85a\x03\x08a\x02\xEB\x89\x89\x89\x89a\x08\xDAV[`\0\x80a\x03\xE2a\x02\xEB\x87\x87\x87\x87a\x08\xDAV[\x90Pa\x03\r\x85a\x03\x08\x89\x84a\x05\xE9V[`\0a\x04\x0Ba\x04\x01\x85\x84a\t\x10V[a\x03b\x90\x85a\x13\xB6V[a\x04\x18a\x03b\x86\x88a\x13\xB6V[a\x02\x83\x91\x90a\x13\xCAV[`\0a\x02\x97g\r\xE0\xB6\xB3\xA7d\0\0\x83a\x13\xF2V[`\0a\x02\x97g\r\xE0\xB6\xB3\xA7d\0\0\x83a\x13\xB6V[`\0\x80a\x04W\x84\x84a\x06\x05V[\x90P`\0a\x04n\x86a\x04i\x89\x8Ba\x14\tV[a\t\x10V[\x90P`\0a\x04\x9Ea\x04\x8Ba\x03ba\x04\x85\x8D\x8Fa\x14\tV[\x85a\x06eV[a\x04\x94\x85a\x14\x1CV[a\x02\xEB\x91\x90a\x13\x08V[\x90P\x8Ba\x04\xB4a\x04\xAE\x8A\x8Ca\x14\tV[\x83a\t\x10V[a\x04\xBE\x91\x90a\x13\x08V[\x9C\x9BPPPPPPPPPPPPV[`\0\x80a\x04\xDB\x84\x84a\x06\x05V[\x90P`\0a\x04\xED\x86a\x04i\x89\x8Ba\x14\tV[\x90P`\0a\x05\x15a\x04\x8Ba\x03b\x8C\x8Fa\x05\x06\x91\x90a\x14\tV[a\x05\x10\x8C\x8Ea\x14\tV[a\x06eV[\x90P\x8Aa\x04\xB4\x83\x83a\t\x10V[`\0\x80a\x05/\x84\x84a\x06\x05V[\x90P`\0a\x05=\x87\x87a\t%V[\x90P`\0a\x05K\x86\x86a\x063V[\x90P\x82a\x05X\x82\x84a\x13\xCAV[a\x05j\x90g\r\xE0\xB6\xB3\xA7d\0\0a\x13BV[a\x05t\x91\x90a\x13\x88V[\x98\x97PPPPPPPPV[`\0\x80g\x13\xA0K\xBD\xFD\xC9\xBE\x88a\x05\x9Eg\r\xE0\xB6\xB3\xA7d\0\0\x85a\x13BV[a\x05\xA8\x91\x90a\x13\x88V[\x90P`\0a\x05\xB5\x82a\x14\x1CV[\x90P`\0a\x05\xC2\x82a\t9V[\x90Pg\x1B\xC1mgN\xC8\0\0a\x05\xDFg\r\xE0\xB6\xB3\xA7d\0\0\x83a\x13BV[a\x02\x83\x91\x90a\x13\x88V[`\0a\x05\xFE\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x0B\"V[\x93\x92PPPV[`\0\x80a\x06\x11\x83a\x0BPV[a\x06\x1F\x90c;\x9A\xCA\0a\x13\xF2V[\x90Pa\x06+\x84\x82a\t\x10V[\x94\x93PPPPV[`\0\x80a\x06Qa\x06K\x85g\x1B\xC1mgN\xC8\0\0a\x0B\xF4V[\x84a\t\x10V[\x90Pa\x06+g\x06\xF0[Y\xD3\xB2\0\0\x82a\t\x10V[`\0a\x05\xFE\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x0C V[`\0g\x06\xF0[Y\xD3\xB2\0\0\x82\x03a\x06\x93WP`\0\x91\x90PV[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x12a\x06\xBBW`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\x06\xDCW`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x06\xE9\x83`\x02a\x13BV[\x90P`\0a\x06\xF6\x82a\x0C?V[\x90P`\0a\x07\x0Cg\x13\xA0K\xBD\xFD\xC9\xBE\x88\x83a\x0E\xB8V[\x90Pa\x02\x83\x81a\x14\x1CV[`\0h\x02H\xCE6\xA7\x0C\xB2k>\x19\x82\x13a\x072WP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a\x07~W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05`\x01`_\x1B\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1Dm\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1Dn\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[`\0a\x05\xFE\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x0B\"V[`\0\x80a\x08\xE7\x84\x84a\x06\x05V[\x90P`\0a\x08\xF5\x87\x87a\t%V[\x90P`\0a\t\x03\x86\x86a\x063V[\x90P\x82a\x05X\x82\x84a\x13\x08V[`\0a\x05\xFE\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x0C V[`\0a\x05\xFEa\t4\x84\x84a\x08\xC5V[a\x0E\xCDV[`\0\x81`\0\x03a\tRWPg\r\xE0\xB6\xB3\xA7d\0\0\x91\x90PV[gV\x98\xEE\xF0fp\0\0\x82\x12a\tiWP`\0\x91\x90PV[a\tzgV\x98\xEE\xF0fp\0\0a\x14\x1CV[\x82\x13a\t\x8FWPg\x1B\xC1mgN\xC8\0\0\x91\x90PV[`\0a\t\x9A\x83a\x10\xA8V[\x90P`\0a\t\xD3g\r\xE0\xB6\xB3\xA7d\0\0a\t\xBC\x84g\x1B\xC1mgN\xC8\0\0a\x06eV[a\t\xCE\x90g\r\xE0\xB6\xB3\xA7d\0\0a\x13\xCAV[a\x10\xE3V[\x90P`\0\x80\x82a\n4\x81a\n!\x81a\n\x0F\x81a\t\xF7\x81g\x02_\x0F\xE1\x05\xA3\x14\0a\x0E\xB8V[a\n\n\x90g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19a\x13\xCAV[a\x0E\xB8V[a\n\n\x90g\x14\xA8EL\x19\xE1\xAC\0a\x13\xCAV[a\n\n\x90g\x0F\xC1\x0E\x01W\x82w\xFF\x19a\x13\xCAV[a\nF\x90g\x03\xDE\xBD\x08;\x8C|\0a\x13\xCAV[\x91P\x83\x90Pa\n\xAE\x81a\n\x9C\x81a\n\x8A\x81a\nx\x81a\ne\x81\x8Ba\x0E\xB8V[a\n\n\x90g\x02\x95\xD4\0\xEA2W\xFF\x19a\x13\xCAV[a\n\n\x90g\x01W\xD8\xB2\xEC\xC7\x08\0a\x13\xCAV[a\n\n\x90g\x051\n\xA7\xD5!0\0a\x13\xCAV[a\n\n\x90g\r\xE0\xCC=\x15a\0\0a\x13\xCAV[\x91P\x81g\x11\x90\0\xAB\x10\x0F\xFC\0a\n\xC4\x87\x88a\x0E\xB8V[a\n\xD0\x90`\0\x19a\x13BV[a\n\xDA\x91\x90a\x13\x08V[a\n\xE4\x91\x90a\x13\xCAV[\x92PP`\0a\n\xF2\x83a\x07\x17V[\x90P`\0a\x0B\0\x85\x83a\x0E\xB8V[\x90P`\0\x88\x12a\x0B\x10W\x80a\x05tV[a\x05t\x81g\x1B\xC1mgN\xC8\0\0a\x13\x08V[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x0B:W`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[`\xB5\x81`\x01`\x88\x1B\x81\x10a\x0BiW`@\x91\x90\x91\x1B\x90`\x80\x1C[i\x01\0\0\0\0\0\0\0\0\0\x81\x10a\x0B\x85W` \x91\x90\x91\x1B\x90`@\x1C[e\x01\0\0\0\0\0\x81\x10a\x0B\x9DW`\x10\x91\x90\x91\x1B\x90` \x1C[c\x01\0\0\0\x81\x10a\x0B\xB3W`\x08\x91\x90\x91\x1B\x90`\x10\x1C[b\x01\0\0\x01\x02`\x12\x1C\x80\x82\x04\x01`\x01\x90\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x90\x81\x90\x04\x81\x11\x90\x03\x90V[`\0a\x05\xFEg\r\xE0\xB6\xB3\xA7d\0\0\x83a\x0C\x0C\x86a\x0E\xCDV[a\x0C\x16\x91\x90a\x13BV[a\x03\x85\x91\x90a\x13\x88V[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x0C8W`\0\x80\xFD[\x04\x92\x91PPV[`\0\x80\x82\x12\x80a\x0CVWPg\x1B\xC1mgN\xC8\0\0\x82\x13[\x15a\x0CtW`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\x0C\x95W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81g\x1B\xC1mgN\xC8\0\0\x03a\x0C\xBDW`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x15a\x0C\xC8W\x91\x90PV[`\0g\r\xE0\xB6\xB3\xA7d\0\0\x83\x12a\x0C\xF0Wa\x0C\xEB\x83g\x1B\xC1mgN\xC8\0\0a\x13\x08V[a\x0C\xF2V[\x82[\x90P`\0a\r\x08\x82g\x1B\xC1mgN\xC8\0\0a\x10\xE3V[\x90P\x80`\0\x03a\r+W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\r6\x82a\x0E\xCDV[\x90P`\0c;\x9A\xCA\0a\raa\r\\a\rVg\x1B\xC1mgN\xC8\0\0a\x14\x1CV[\x85a\x0E\xB8V[a\x0BPV[a\rk\x91\x90a\x13BV[\x90P`\0\x80a\r\x82\x83g\x03\xC1f\\z\xAB \0a\x0E\xB8V[a\r\x94\x90g \x05\xFEO&\x8E\xA0\0a\x13\xCAV[\x90P`\0a\r\xBF\x84a\r\xAD\x86f\x9F2u$b\xA0\0a\x0E\xB8V[a\n\n\x90g\r\xC5R\x7Fd, \0a\x13\xCAV[a\r\xD1\x90g\r\xE0\xB6\xB3\xA7d\0\0a\x13\xCAV[\x90Pa\r\xF5g\t\xD0(\xCCo _\xFF\x19\x85a\r\xEB\x85\x85a\x10\xE3V[a\n\n\x91\x90a\x13\x08V[\x92PPP`\0[`\x02\x81\x10\x15a\x0E\x90W`\0\x86a\x0E\x11\x84a\t9V[a\x0E\x1B\x91\x90a\x13\x08V[\x90P`\0a\x0E)\x84\x85a\x0E\xB8V[a\x0E2\x90a\x14\x1CV[\x90P`\0a\x0E?\x82a\x07\x17V[\x90P`\0a\x0EM\x86\x85a\x0E\xB8V[a\x0E_g\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x84a\x0E\xB8V[a\x0Ei\x91\x90a\x13\x08V[\x90Pa\x0Eu\x84\x82a\x10\xE3V[a\x0E\x7F\x90\x87a\x13\xCAV[\x95P\x84`\x01\x01\x94PPPPPa\r\xFCV[g\r\xE0\xB6\xB3\xA7d\0\0\x88\x12a\x0E\xADWa\x0E\xA8\x82a\x14\x1CV[a\x05tV[P\x96\x95PPPPPPV[`\0a\x05\xFE\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x10\xF4V[`\0\x80\x82\x13a\x0F\nW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x07uV[`\0``a\x0F\x17\x84a\x11\x13V[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[`\0`\x01`\xFF\x1B\x82\x03a\x10\xCEW`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x12\x15a\x10\xDFWP\x19`\x01\x01\x90V[P\x90V[`\0a\x05\xFE\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x05\x85\x14\x17\x16a\x11\x0CW`\0\x80\xFD[\x05\x92\x91PPV[`\0\x80\x82\x11a\x11PW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x07uV[P`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x82\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\x12&Wa\x12&a\x11\xBBV[PP\x835\x95` \x85\x015\x95P`@\x85\x015\x94``\x81\x015\x94P`\x80\x015\x92P\x90PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x12bWa\x12ba\x11\xBBV[PP\x825\x94` \x84\x015\x94P`@\x84\x015\x93``\x015\x92P\x90PV[`\0` \x82\x84\x03\x12\x15a\x12\x93Wa\x12\x93a\x11\xBBV[P5\x91\x90PV[`\0\x80`\0\x80`\0\x80`\0\x80a\x01\0\x89\x8B\x03\x12\x15a\x12\xBAWa\x12\xBAa\x11\xBBV[PP\x865\x98` \x88\x015\x98P`@\x88\x015\x97``\x81\x015\x97P`\x80\x81\x015\x96P`\xA0\x81\x015\x95P`\xC0\x81\x015\x94P`\xE0\x015\x92P\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a\x13(Wa\x13(a\x12\xF2V[P\x92\x91PPV[\x81\x81\x03\x81\x81\x11\x15a\x02\x97Wa\x02\x97a\x12\xF2V[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15a\x13^Wa\x13^a\x12\xF2V[\x81\x81\x05\x83\x14\x82\x15\x17a\x02\x97Wa\x02\x97a\x12\xF2V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a\x13\x97Wa\x13\x97a\x13rV[`\x01`\xFF\x1B\x82\x14`\0\x19\x84\x14\x16\x15a\x13\xB1Wa\x13\xB1a\x12\xF2V[P\x05\x90V[`\0\x82a\x13\xC5Wa\x13\xC5a\x13rV[P\x04\x90V[\x80\x82\x01\x82\x81\x12`\0\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a\x13\xEAWa\x13\xEAa\x12\xF2V[PP\x92\x91PPV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x02\x97Wa\x02\x97a\x12\xF2V[\x80\x82\x01\x80\x82\x11\x15a\x02\x97Wa\x02\x97a\x12\xF2V[`\0`\x01`\xFF\x1B\x82\x01a\x141Wa\x141a\x12\xF2V[P`\0\x03\x90V";
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
        ///Calls the contract's `computeLGivenX` (0x50e304e5) function
        pub fn compute_l_given_x(
            &self,
            x: ::ethers::core::types::U256,
            s: ::ethers::core::types::U256,
            k: ::ethers::core::types::U256,
            sigma: ::ethers::core::types::U256,
            tau: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([80, 227, 4, 229], (x, s, k, sigma, tau))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `computeLGivenY` (0x5800d424) function
        pub fn compute_l_given_y(
            &self,
            y: ::ethers::core::types::U256,
            s: ::ethers::core::types::U256,
            k: ::ethers::core::types::U256,
            sigma: ::ethers::core::types::U256,
            tau: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([88, 0, 212, 36], (y, s, k, sigma, tau))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `computeOutputXGivenY` (0xcf25c3ef) function
        pub fn compute_output_x_given_y(
            &self,
            x: ::ethers::core::types::U256,
            y: ::ethers::core::types::U256,
            delta_y: ::ethers::core::types::U256,
            l: ::ethers::core::types::U256,
            delta_l: ::ethers::core::types::U256,
            k: ::ethers::core::types::U256,
            sigma: ::ethers::core::types::U256,
            tau: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash(
                    [207, 37, 195, 239],
                    (x, y, delta_y, l, delta_l, k, sigma, tau),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `computeOutputYGivenX` (0xe3908f6d) function
        pub fn compute_output_y_given_x(
            &self,
            x: ::ethers::core::types::U256,
            y: ::ethers::core::types::U256,
            delta_x: ::ethers::core::types::U256,
            l: ::ethers::core::types::U256,
            delta_l: ::ethers::core::types::U256,
            k: ::ethers::core::types::U256,
            sigma: ::ethers::core::types::U256,
            tau: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash(
                    [227, 144, 143, 109],
                    (x, y, delta_x, l, delta_l, k, sigma, tau),
                )
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
        ///Calls the contract's `computeXGivenL` (0x2170ee36) function
        pub fn compute_x_given_l(
            &self,
            l: ::ethers::core::types::U256,
            s: ::ethers::core::types::U256,
            k: ::ethers::core::types::U256,
            sigma: ::ethers::core::types::U256,
            tau: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([33, 112, 238, 54], (l, s, k, sigma, tau))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `computeYGivenL` (0x671613aa) function
        pub fn compute_y_given_l(
            &self,
            l: ::ethers::core::types::U256,
            s: ::ethers::core::types::U256,
            k: ::ethers::core::types::U256,
            sigma: ::ethers::core::types::U256,
            tau: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([103, 22, 19, 170], (l, s, k, sigma, tau))
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
    ///Container type for all input parameters for the `computeLGivenX` function with signature `computeLGivenX(uint256,uint256,uint256,uint256,uint256)` and selector `0x50e304e5`
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
        abi = "computeLGivenX(uint256,uint256,uint256,uint256,uint256)"
    )]
    pub struct ComputeLGivenXCall {
        pub x: ::ethers::core::types::U256,
        pub s: ::ethers::core::types::U256,
        pub k: ::ethers::core::types::U256,
        pub sigma: ::ethers::core::types::U256,
        pub tau: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `computeLGivenY` function with signature `computeLGivenY(uint256,uint256,uint256,uint256,uint256)` and selector `0x5800d424`
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
        abi = "computeLGivenY(uint256,uint256,uint256,uint256,uint256)"
    )]
    pub struct ComputeLGivenYCall {
        pub y: ::ethers::core::types::U256,
        pub s: ::ethers::core::types::U256,
        pub k: ::ethers::core::types::U256,
        pub sigma: ::ethers::core::types::U256,
        pub tau: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `computeOutputXGivenY` function with signature `computeOutputXGivenY(uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256)` and selector `0xcf25c3ef`
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
        abi = "computeOutputXGivenY(uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256)"
    )]
    pub struct ComputeOutputXGivenYCall {
        pub x: ::ethers::core::types::U256,
        pub y: ::ethers::core::types::U256,
        pub delta_y: ::ethers::core::types::U256,
        pub l: ::ethers::core::types::U256,
        pub delta_l: ::ethers::core::types::U256,
        pub k: ::ethers::core::types::U256,
        pub sigma: ::ethers::core::types::U256,
        pub tau: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `computeOutputYGivenX` function with signature `computeOutputYGivenX(uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256)` and selector `0xe3908f6d`
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
        abi = "computeOutputYGivenX(uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256)"
    )]
    pub struct ComputeOutputYGivenXCall {
        pub x: ::ethers::core::types::U256,
        pub y: ::ethers::core::types::U256,
        pub delta_x: ::ethers::core::types::U256,
        pub l: ::ethers::core::types::U256,
        pub delta_l: ::ethers::core::types::U256,
        pub k: ::ethers::core::types::U256,
        pub sigma: ::ethers::core::types::U256,
        pub tau: ::ethers::core::types::U256,
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
    ///Container type for all input parameters for the `computeXGivenL` function with signature `computeXGivenL(uint256,uint256,uint256,uint256,uint256)` and selector `0x2170ee36`
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
        abi = "computeXGivenL(uint256,uint256,uint256,uint256,uint256)"
    )]
    pub struct ComputeXGivenLCall {
        pub l: ::ethers::core::types::U256,
        pub s: ::ethers::core::types::U256,
        pub k: ::ethers::core::types::U256,
        pub sigma: ::ethers::core::types::U256,
        pub tau: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `computeYGivenL` function with signature `computeYGivenL(uint256,uint256,uint256,uint256,uint256)` and selector `0x671613aa`
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
        abi = "computeYGivenL(uint256,uint256,uint256,uint256,uint256)"
    )]
    pub struct ComputeYGivenLCall {
        pub l: ::ethers::core::types::U256,
        pub s: ::ethers::core::types::U256,
        pub k: ::ethers::core::types::U256,
        pub sigma: ::ethers::core::types::U256,
        pub tau: ::ethers::core::types::U256,
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
    ///Container type for all return fields from the `computeLGivenX` function with signature `computeLGivenX(uint256,uint256,uint256,uint256,uint256)` and selector `0x50e304e5`
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
    ///Container type for all return fields from the `computeLGivenY` function with signature `computeLGivenY(uint256,uint256,uint256,uint256,uint256)` and selector `0x5800d424`
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
    ///Container type for all return fields from the `computeOutputXGivenY` function with signature `computeOutputXGivenY(uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256)` and selector `0xcf25c3ef`
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
    ///Container type for all return fields from the `computeOutputYGivenX` function with signature `computeOutputYGivenX(uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256)` and selector `0xe3908f6d`
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
    ///Container type for all return fields from the `computeXGivenL` function with signature `computeXGivenL(uint256,uint256,uint256,uint256,uint256)` and selector `0x2170ee36`
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
    ///Container type for all return fields from the `computeYGivenL` function with signature `computeYGivenL(uint256,uint256,uint256,uint256,uint256)` and selector `0x671613aa`
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
