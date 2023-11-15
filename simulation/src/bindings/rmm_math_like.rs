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
                                    name: ::std::borrow::ToOwned::to_owned("deltaX"),
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
                                    name: ::std::borrow::ToOwned::to_owned("deltaX"),
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
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[Pa\x14\x87\x80a\0m`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\0\xEBW`\x005`\xE0\x1C\x80c\xB2\xC6\xDAw\x11a\0\xB3W\x80c\xB2\xC6\xDAw\x14a\x01\xC1W\x80c\xB3gmk\x14a\x01\xD4W\x80c\xCF%\xC3\xEF\x14a\x01\xE7W\x80c\xE3\x90\x8Fm\x14a\x01\xFAW\x80c\xFD\x11\x87\xF7\x14a\x02\rWa\0\xEBV[\x80c,\xCCT\xCC\x14a\x01PW\x80c\x8D\xFB\x85\xC6\x14a\x01uW\x80c\x8F\xE9\xC1v\x14a\x01\x88W\x80c\xAF\xE1\x08\x8E\x14a\x01\x9BW\x80c\xB0cCS\x14a\x01\xAEW[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x01ca\x01^6`\x04a\x12ZV[a\x02 V[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\x01ca\x01\x836`\x04a\x12\x98V[a\x029V[a\x01ca\x01\x966`\x04a\x12\x98V[a\x02PV[a\x01ca\x01\xA96`\x04a\x12\x98V[a\x02^V[a\x01ca\x01\xBC6`\x04a\x12\x98V[a\x02lV[a\x01ca\x01\xCF6`\x04a\x12\xCDV[a\x02zV[a\x01ca\x01\xE26`\x04a\x12\xCDV[a\x02\x8BV[a\x01ca\x01\xF56`\x04a\x12\xE9V[a\x02\x96V[a\x01ca\x02\x086`\x04a\x12\xE9V[a\x02\xB5V[a\x01ca\x02\x1B6`\x04a\x12\x98V[a\x02\xC7V[`\0a\x02/\x86\x86\x86\x86\x86a\x02\xD5V[\x96\x95PPPPPPV[`\0a\x02G\x85\x85\x85\x85a\x03\x9DV[\x95\x94PPPPPV[`\0a\x02G\x85\x85\x85\x85a\x03\xCDV[`\0a\x02G\x85\x85\x85\x85a\x04XV[`\0a\x02G\x85\x85\x85\x85a\x04\xB4V[`\0a\x02\x85\x82a\x05\x1AV[\x92\x91PPV[`\0a\x02\x85\x82a\x05.V[`\0a\x02\xA8\x89\x89\x89\x89\x89\x89\x89\x89a\x05BV[\x99\x98PPPPPPPPPV[`\0a\x02\xA8\x89\x89\x89\x89\x89\x89\x89\x89a\x05\x96V[`\0a\x02G\x85\x85\x85\x85a\x05\xEDV[`\0\x80a\x02\xF3a\x02\xED\x85g\x1B\xC1mgN\xC8\0\0a\x06yV[\x84a\x06\xACV[\x90P`\0a\x03\tg\x06\xF0[Y\xD3\xB2\0\0\x83a\x06\xACV[\x90P`\0a\x03\x16\x85a\x06\xC1V[a\x03$\x90c;\x9A\xCA\0a\x13WV[\x90P`\0a\x032\x87\x83a\x06\xACV[\x90P`\0a\x03@\x8B\x8Ba\x07eV[\x90Pa\x03\x8E\x89a\x03\x89\x86g\r\xE0\xB6\xB3\xA7d\0\0\x86a\x03fa\x03a\x88\x84a\x13nV[a\x07zV[a\x03p\x91\x90a\x13\x81V[a\x03z\x91\x90a\x13\xC7V[a\x03\x84\x91\x90a\x13\xF5V[a\x08\x17V[a\t\xC5V[\x9B\x9APPPPPPPPPPPV[`\0a\x03\xB6a\x03\xAC\x85\x84a\x06\xACV[a\x03a\x90\x85a\x14\x1CV[a\x03\xC3a\x03a\x86\x88a\x14\x1CV[a\x02G\x91\x90a\x140V[`\0\x80a\x03\xE2a\x03\xDD\x86\x86a\t\xDAV[a\t\xEFV[\x90P`\0a\x04\x04g\x06\xF0[Y\xD3\xB2\0\0a\x03\x89\x86g\x1B\xC1mgN\xC8\0\0a\x06yV[\x90P`\0a\x047\x85a\x04\x16\x84\x86a\x13\xF5V[a\x04(\x90g\r\xE0\xB6\xB3\xA7d\0\0a\x13\x81V[a\x042\x91\x90a\x13\xC7V[a\x0B\xCAV[\x90Pa\x04L\x88a\x04G\x88\x84a\t\xC5V[a\t\xDAV[\x98\x97PPPPPPPPV[`\0\x80a\x04ha\x03\xDD\x86\x86a\t\xDAV[\x90P`\0a\x04\x8Ag\x06\xF0[Y\xD3\xB2\0\0a\x03\x89\x86g\x1B\xC1mgN\xC8\0\0a\x06yV[\x90P`\0a\x04\x9C\x85a\x04\x16\x84\x86a\x140V[\x90Pa\x04L\x88a\x03\x89\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x13\xF5V[`\0\x80a\x04\xC4a\x03\xDD\x86\x86a\t\xDAV[\x90P`\0a\x04\xE6g\x06\xF0[Y\xD3\xB2\0\0a\x03\x89\x86g\x1B\xC1mgN\xC8\0\0a\x06yV[\x90P`\0a\x04\xF8\x85a\x04\x16\x84\x86a\x140V[\x90P`\0a\x05\x0E\x82g\r\xE0\xB6\xB3\xA7d\0\0a\x13\xF5V[\x90Pa\x02\xA8\x89\x82a\t\xDAV[`\0a\x02\x85g\r\xE0\xB6\xB3\xA7d\0\0\x83a\x13WV[`\0a\x02\x85g\r\xE0\xB6\xB3\xA7d\0\0\x83a\x14\x1CV[`\0\x80a\x05X\x84a\x05S\x87\x89a\x14XV[a\x06\xACV[\x90P`\0a\x05\x7Fa\x05ua\x03aa\x05o\x8B\x8Da\x14XV[\x85a\x07eV[a\x042\x90\x86a\x140V[\x90P\x8Aa\x05\x8C\x83\x83a\x06\xACV[a\x03\x8E\x91\x90a\x13\xF5V[`\0\x80a\x05\xA7\x84a\x05S\x87\x89a\x14XV[\x90P`\0a\x05\xE0a\x05\xCDa\x03aa\x05\xBE\x8D\x8Fa\x14XV[a\x05\xC8\x8A\x8Ca\x14XV[a\x07eV[a\x05\xD6\x86a\x14kV[a\x042\x91\x90a\x13\xF5V[\x90P\x88a\x05\x8C\x83\x83a\x06\xACV[`\0\x80a\x05\xFDa\x03\xDD\x86\x86a\t\xDAV[\x90P`\0a\x06\x1Fg\x06\xF0[Y\xD3\xB2\0\0a\x03\x89\x86g\x1B\xC1mgN\xC8\0\0a\x06yV[\x90P`\0a\x06-\x82\x84a\x13\xF5V[\x90P`\0\x85a\x06D\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x13\x81V[a\x06N\x91\x90a\x13\xC7V[\x90P`\0a\x06[\x82a\x0B\xCAV[\x90Pa\x06k\x88a\x03\x89\x8C\x84a\t\xC5V[\x9A\x99PPPPPPPPPPV[`\0a\x06\xA5g\r\xE0\xB6\xB3\xA7d\0\0\x83a\x06\x91\x86a\t\xEFV[a\x06\x9B\x91\x90a\x13\x81V[a\x03\x84\x91\x90a\x13\xC7V[\x93\x92PPPV[`\0a\x06\xA5\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x0C3V[`\xB5\x81`\x01`\x88\x1B\x81\x10a\x06\xDAW`@\x91\x90\x91\x1B\x90`\x80\x1C[i\x01\0\0\0\0\0\0\0\0\0\x81\x10a\x06\xF6W` \x91\x90\x91\x1B\x90`@\x1C[e\x01\0\0\0\0\0\x81\x10a\x07\x0EW`\x10\x91\x90\x91\x1B\x90` \x1C[c\x01\0\0\0\x81\x10a\x07$W`\x08\x91\x90\x91\x1B\x90`\x10\x1C[b\x01\0\0\x01\x02`\x12\x1C\x80\x82\x04\x01`\x01\x90\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x90\x81\x90\x04\x81\x11\x90\x03\x90V[`\0a\x06\xA5\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x0C3V[`\0g\x06\xF0[Y\xD3\xB2\0\0\x82\x03a\x07\x93WP`\0\x91\x90PV[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x12a\x07\xBBW`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\x07\xDCW`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x07\xE9\x83`\x02a\x13\x81V[\x90P`\0a\x07\xF6\x82a\x0CRV[\x90P`\0a\x08\x0Cg\x13\xA0K\xBD\xFD\xC9\xBE\x88\x83a\x0E\xD0V[\x90Pa\x02G\x81a\x14kV[`\0h\x02H\xCE6\xA7\x0C\xB2k>\x19\x82\x13a\x082WP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a\x08~W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05`\x01`_\x1B\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1Dm\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1Dn\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[`\0a\x06\xA5\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x0E\xE5V[`\0a\x06\xA5\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x0E\xE5V[`\0\x80\x82\x13a\n,W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x08uV[`\0``a\n9\x84a\x0F\x13V[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[`\0\x80g\x13\xA0K\xBD\xFD\xC9\xBE\x88a\x0B\xE8g\r\xE0\xB6\xB3\xA7d\0\0\x85a\x13\x81V[a\x0B\xF2\x91\x90a\x13\xC7V[\x90P`\0a\x0B\xFF\x82a\x14kV[\x90P`\0a\x0C\x0C\x82a\x0F\xBBV[\x90Pg\x1B\xC1mgN\xC8\0\0a\x0C)g\r\xE0\xB6\xB3\xA7d\0\0\x83a\x13\x81V[a\x02G\x91\x90a\x13\xC7V[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x0CKW`\0\x80\xFD[\x04\x92\x91PPV[`\0\x80\x82\x12\x80a\x0CiWPg\x1B\xC1mgN\xC8\0\0\x82\x13[\x15a\x0C\x87W`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\x0C\xA8W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81g\x1B\xC1mgN\xC8\0\0\x03a\x0C\xD0W`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x15a\x0C\xDBW\x91\x90PV[`\0g\r\xE0\xB6\xB3\xA7d\0\0\x83\x12a\r\x03Wa\x0C\xFE\x83g\x1B\xC1mgN\xC8\0\0a\x13\xF5V[a\r\x05V[\x82[\x90P`\0a\r\x1B\x82g\x1B\xC1mgN\xC8\0\0a\x11\x9FV[\x90P\x80`\0\x03a\r>W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\rI\x82a\t\xEFV[\x90P`\0c;\x9A\xCA\0a\rta\roa\rig\x1B\xC1mgN\xC8\0\0a\x14kV[\x85a\x0E\xD0V[a\x06\xC1V[a\r~\x91\x90a\x13\x81V[\x90P`\0\x80a\r\x95\x83g\x03\xC1f\\z\xAB \0a\x0E\xD0V[a\r\xA7\x90g \x05\xFEO&\x8E\xA0\0a\x140V[\x90P`\0a\r\xD7\x84a\r\xC0\x86f\x9F2u$b\xA0\0a\x0E\xD0V[a\r\xD2\x90g\r\xC5R\x7Fd, \0a\x140V[a\x0E\xD0V[a\r\xE9\x90g\r\xE0\xB6\xB3\xA7d\0\0a\x140V[\x90Pa\x0E\rg\t\xD0(\xCCo _\xFF\x19\x85a\x0E\x03\x85\x85a\x11\x9FV[a\r\xD2\x91\x90a\x13\xF5V[\x92PPP`\0[`\x02\x81\x10\x15a\x0E\xA8W`\0\x86a\x0E)\x84a\x0F\xBBV[a\x0E3\x91\x90a\x13\xF5V[\x90P`\0a\x0EA\x84\x85a\x0E\xD0V[a\x0EJ\x90a\x14kV[\x90P`\0a\x0EW\x82a\x08\x17V[\x90P`\0a\x0Ee\x86\x85a\x0E\xD0V[a\x0Ewg\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x84a\x0E\xD0V[a\x0E\x81\x91\x90a\x13\xF5V[\x90Pa\x0E\x8D\x84\x82a\x11\x9FV[a\x0E\x97\x90\x87a\x140V[\x95P\x84`\x01\x01\x94PPPPPa\x0E\x14V[g\r\xE0\xB6\xB3\xA7d\0\0\x88\x12a\x0E\xC5Wa\x0E\xC0\x82a\x14kV[a\x04LV[P\x96\x95PPPPPPV[`\0a\x06\xA5\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x11\xB0V[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x0E\xFDW`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[`\0\x80\x82\x11a\x0FPW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x08uV[P`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x82\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[`\0\x81`\0\x03a\x0F\xD4WPg\r\xE0\xB6\xB3\xA7d\0\0\x91\x90PV[gV\x98\xEE\xF0fp\0\0\x82\x12a\x0F\xEBWP`\0\x91\x90PV[a\x0F\xFCgV\x98\xEE\xF0fp\0\0a\x14kV[\x82\x13a\x10\x11WPg\x1B\xC1mgN\xC8\0\0\x91\x90PV[`\0a\x10\x1C\x83a\x11\xCFV[\x90P`\0a\x10Ug\r\xE0\xB6\xB3\xA7d\0\0a\x10>\x84g\x1B\xC1mgN\xC8\0\0a\x07eV[a\x10P\x90g\r\xE0\xB6\xB3\xA7d\0\0a\x140V[a\x11\x9FV[\x90P`\0\x80\x82a\x10\xB1\x81a\x10\x9E\x81a\x10\x8C\x81a\x10y\x81g\x02_\x0F\xE1\x05\xA3\x14\0a\x0E\xD0V[a\r\xD2\x90g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19a\x140V[a\r\xD2\x90g\x14\xA8EL\x19\xE1\xAC\0a\x140V[a\r\xD2\x90g\x0F\xC1\x0E\x01W\x82w\xFF\x19a\x140V[a\x10\xC3\x90g\x03\xDE\xBD\x08;\x8C|\0a\x140V[\x91P\x83\x90Pa\x11+\x81a\x11\x19\x81a\x11\x07\x81a\x10\xF5\x81a\x10\xE2\x81\x8Ba\x0E\xD0V[a\r\xD2\x90g\x02\x95\xD4\0\xEA2W\xFF\x19a\x140V[a\r\xD2\x90g\x01W\xD8\xB2\xEC\xC7\x08\0a\x140V[a\r\xD2\x90g\x051\n\xA7\xD5!0\0a\x140V[a\r\xD2\x90g\r\xE0\xCC=\x15a\0\0a\x140V[\x91P\x81g\x11\x90\0\xAB\x10\x0F\xFC\0a\x11A\x87\x88a\x0E\xD0V[a\x11M\x90`\0\x19a\x13\x81V[a\x11W\x91\x90a\x13\xF5V[a\x11a\x91\x90a\x140V[\x92PP`\0a\x11o\x83a\x08\x17V[\x90P`\0a\x11}\x85\x83a\x0E\xD0V[\x90P`\0\x88\x12a\x11\x8DW\x80a\x04LV[a\x04L\x81g\x1B\xC1mgN\xC8\0\0a\x13\xF5V[`\0a\x06\xA5\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x05\x85\x14\x17\x16a\x11\xC8W`\0\x80\xFD[\x05\x92\x91PPV[`\0`\x01`\xFF\x1B\x82\x03a\x11\xF5W`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x12\x15a\x12\x06WP\x19`\x01\x01\x90V[P\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\x12uWa\x12ua\x12\nV[PP\x835\x95` \x85\x015\x95P`@\x85\x015\x94``\x81\x015\x94P`\x80\x015\x92P\x90PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x12\xB1Wa\x12\xB1a\x12\nV[PP\x825\x94` \x84\x015\x94P`@\x84\x015\x93``\x015\x92P\x90PV[`\0` \x82\x84\x03\x12\x15a\x12\xE2Wa\x12\xE2a\x12\nV[P5\x91\x90PV[`\0\x80`\0\x80`\0\x80`\0\x80a\x01\0\x89\x8B\x03\x12\x15a\x13\tWa\x13\ta\x12\nV[PP\x865\x98` \x88\x015\x98P`@\x88\x015\x97``\x81\x015\x97P`\x80\x81\x015\x96P`\xA0\x81\x015\x95P`\xC0\x81\x015\x94P`\xE0\x015\x92P\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x02\x85Wa\x02\x85a\x13AV[\x81\x81\x03\x81\x81\x11\x15a\x02\x85Wa\x02\x85a\x13AV[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15a\x13\x9DWa\x13\x9Da\x13AV[\x81\x81\x05\x83\x14\x82\x15\x17a\x02\x85Wa\x02\x85a\x13AV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a\x13\xD6Wa\x13\xD6a\x13\xB1V[`\x01`\xFF\x1B\x82\x14`\0\x19\x84\x14\x16\x15a\x13\xF0Wa\x13\xF0a\x13AV[P\x05\x90V[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a\x14\x15Wa\x14\x15a\x13AV[P\x92\x91PPV[`\0\x82a\x14+Wa\x14+a\x13\xB1V[P\x04\x90V[\x80\x82\x01\x82\x81\x12`\0\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a\x14PWa\x14Pa\x13AV[PP\x92\x91PPV[\x80\x82\x01\x80\x82\x11\x15a\x02\x85Wa\x02\x85a\x13AV[`\0`\x01`\xFF\x1B\x82\x01a\x14\x80Wa\x14\x80a\x13AV[P`\0\x03\x90V";
    /// The bytecode of the contract.
    pub static RMMMATHLIKE_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\0\xEBW`\x005`\xE0\x1C\x80c\xB2\xC6\xDAw\x11a\0\xB3W\x80c\xB2\xC6\xDAw\x14a\x01\xC1W\x80c\xB3gmk\x14a\x01\xD4W\x80c\xCF%\xC3\xEF\x14a\x01\xE7W\x80c\xE3\x90\x8Fm\x14a\x01\xFAW\x80c\xFD\x11\x87\xF7\x14a\x02\rWa\0\xEBV[\x80c,\xCCT\xCC\x14a\x01PW\x80c\x8D\xFB\x85\xC6\x14a\x01uW\x80c\x8F\xE9\xC1v\x14a\x01\x88W\x80c\xAF\xE1\x08\x8E\x14a\x01\x9BW\x80c\xB0cCS\x14a\x01\xAEW[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x01ca\x01^6`\x04a\x12ZV[a\x02 V[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\x01ca\x01\x836`\x04a\x12\x98V[a\x029V[a\x01ca\x01\x966`\x04a\x12\x98V[a\x02PV[a\x01ca\x01\xA96`\x04a\x12\x98V[a\x02^V[a\x01ca\x01\xBC6`\x04a\x12\x98V[a\x02lV[a\x01ca\x01\xCF6`\x04a\x12\xCDV[a\x02zV[a\x01ca\x01\xE26`\x04a\x12\xCDV[a\x02\x8BV[a\x01ca\x01\xF56`\x04a\x12\xE9V[a\x02\x96V[a\x01ca\x02\x086`\x04a\x12\xE9V[a\x02\xB5V[a\x01ca\x02\x1B6`\x04a\x12\x98V[a\x02\xC7V[`\0a\x02/\x86\x86\x86\x86\x86a\x02\xD5V[\x96\x95PPPPPPV[`\0a\x02G\x85\x85\x85\x85a\x03\x9DV[\x95\x94PPPPPV[`\0a\x02G\x85\x85\x85\x85a\x03\xCDV[`\0a\x02G\x85\x85\x85\x85a\x04XV[`\0a\x02G\x85\x85\x85\x85a\x04\xB4V[`\0a\x02\x85\x82a\x05\x1AV[\x92\x91PPV[`\0a\x02\x85\x82a\x05.V[`\0a\x02\xA8\x89\x89\x89\x89\x89\x89\x89\x89a\x05BV[\x99\x98PPPPPPPPPV[`\0a\x02\xA8\x89\x89\x89\x89\x89\x89\x89\x89a\x05\x96V[`\0a\x02G\x85\x85\x85\x85a\x05\xEDV[`\0\x80a\x02\xF3a\x02\xED\x85g\x1B\xC1mgN\xC8\0\0a\x06yV[\x84a\x06\xACV[\x90P`\0a\x03\tg\x06\xF0[Y\xD3\xB2\0\0\x83a\x06\xACV[\x90P`\0a\x03\x16\x85a\x06\xC1V[a\x03$\x90c;\x9A\xCA\0a\x13WV[\x90P`\0a\x032\x87\x83a\x06\xACV[\x90P`\0a\x03@\x8B\x8Ba\x07eV[\x90Pa\x03\x8E\x89a\x03\x89\x86g\r\xE0\xB6\xB3\xA7d\0\0\x86a\x03fa\x03a\x88\x84a\x13nV[a\x07zV[a\x03p\x91\x90a\x13\x81V[a\x03z\x91\x90a\x13\xC7V[a\x03\x84\x91\x90a\x13\xF5V[a\x08\x17V[a\t\xC5V[\x9B\x9APPPPPPPPPPPV[`\0a\x03\xB6a\x03\xAC\x85\x84a\x06\xACV[a\x03a\x90\x85a\x14\x1CV[a\x03\xC3a\x03a\x86\x88a\x14\x1CV[a\x02G\x91\x90a\x140V[`\0\x80a\x03\xE2a\x03\xDD\x86\x86a\t\xDAV[a\t\xEFV[\x90P`\0a\x04\x04g\x06\xF0[Y\xD3\xB2\0\0a\x03\x89\x86g\x1B\xC1mgN\xC8\0\0a\x06yV[\x90P`\0a\x047\x85a\x04\x16\x84\x86a\x13\xF5V[a\x04(\x90g\r\xE0\xB6\xB3\xA7d\0\0a\x13\x81V[a\x042\x91\x90a\x13\xC7V[a\x0B\xCAV[\x90Pa\x04L\x88a\x04G\x88\x84a\t\xC5V[a\t\xDAV[\x98\x97PPPPPPPPV[`\0\x80a\x04ha\x03\xDD\x86\x86a\t\xDAV[\x90P`\0a\x04\x8Ag\x06\xF0[Y\xD3\xB2\0\0a\x03\x89\x86g\x1B\xC1mgN\xC8\0\0a\x06yV[\x90P`\0a\x04\x9C\x85a\x04\x16\x84\x86a\x140V[\x90Pa\x04L\x88a\x03\x89\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x13\xF5V[`\0\x80a\x04\xC4a\x03\xDD\x86\x86a\t\xDAV[\x90P`\0a\x04\xE6g\x06\xF0[Y\xD3\xB2\0\0a\x03\x89\x86g\x1B\xC1mgN\xC8\0\0a\x06yV[\x90P`\0a\x04\xF8\x85a\x04\x16\x84\x86a\x140V[\x90P`\0a\x05\x0E\x82g\r\xE0\xB6\xB3\xA7d\0\0a\x13\xF5V[\x90Pa\x02\xA8\x89\x82a\t\xDAV[`\0a\x02\x85g\r\xE0\xB6\xB3\xA7d\0\0\x83a\x13WV[`\0a\x02\x85g\r\xE0\xB6\xB3\xA7d\0\0\x83a\x14\x1CV[`\0\x80a\x05X\x84a\x05S\x87\x89a\x14XV[a\x06\xACV[\x90P`\0a\x05\x7Fa\x05ua\x03aa\x05o\x8B\x8Da\x14XV[\x85a\x07eV[a\x042\x90\x86a\x140V[\x90P\x8Aa\x05\x8C\x83\x83a\x06\xACV[a\x03\x8E\x91\x90a\x13\xF5V[`\0\x80a\x05\xA7\x84a\x05S\x87\x89a\x14XV[\x90P`\0a\x05\xE0a\x05\xCDa\x03aa\x05\xBE\x8D\x8Fa\x14XV[a\x05\xC8\x8A\x8Ca\x14XV[a\x07eV[a\x05\xD6\x86a\x14kV[a\x042\x91\x90a\x13\xF5V[\x90P\x88a\x05\x8C\x83\x83a\x06\xACV[`\0\x80a\x05\xFDa\x03\xDD\x86\x86a\t\xDAV[\x90P`\0a\x06\x1Fg\x06\xF0[Y\xD3\xB2\0\0a\x03\x89\x86g\x1B\xC1mgN\xC8\0\0a\x06yV[\x90P`\0a\x06-\x82\x84a\x13\xF5V[\x90P`\0\x85a\x06D\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x13\x81V[a\x06N\x91\x90a\x13\xC7V[\x90P`\0a\x06[\x82a\x0B\xCAV[\x90Pa\x06k\x88a\x03\x89\x8C\x84a\t\xC5V[\x9A\x99PPPPPPPPPPV[`\0a\x06\xA5g\r\xE0\xB6\xB3\xA7d\0\0\x83a\x06\x91\x86a\t\xEFV[a\x06\x9B\x91\x90a\x13\x81V[a\x03\x84\x91\x90a\x13\xC7V[\x93\x92PPPV[`\0a\x06\xA5\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x0C3V[`\xB5\x81`\x01`\x88\x1B\x81\x10a\x06\xDAW`@\x91\x90\x91\x1B\x90`\x80\x1C[i\x01\0\0\0\0\0\0\0\0\0\x81\x10a\x06\xF6W` \x91\x90\x91\x1B\x90`@\x1C[e\x01\0\0\0\0\0\x81\x10a\x07\x0EW`\x10\x91\x90\x91\x1B\x90` \x1C[c\x01\0\0\0\x81\x10a\x07$W`\x08\x91\x90\x91\x1B\x90`\x10\x1C[b\x01\0\0\x01\x02`\x12\x1C\x80\x82\x04\x01`\x01\x90\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x90\x81\x90\x04\x81\x11\x90\x03\x90V[`\0a\x06\xA5\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x0C3V[`\0g\x06\xF0[Y\xD3\xB2\0\0\x82\x03a\x07\x93WP`\0\x91\x90PV[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x12a\x07\xBBW`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\x07\xDCW`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x07\xE9\x83`\x02a\x13\x81V[\x90P`\0a\x07\xF6\x82a\x0CRV[\x90P`\0a\x08\x0Cg\x13\xA0K\xBD\xFD\xC9\xBE\x88\x83a\x0E\xD0V[\x90Pa\x02G\x81a\x14kV[`\0h\x02H\xCE6\xA7\x0C\xB2k>\x19\x82\x13a\x082WP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a\x08~W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05`\x01`_\x1B\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1Dm\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1Dn\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[`\0a\x06\xA5\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x0E\xE5V[`\0a\x06\xA5\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x0E\xE5V[`\0\x80\x82\x13a\n,W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x08uV[`\0``a\n9\x84a\x0F\x13V[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[`\0\x80g\x13\xA0K\xBD\xFD\xC9\xBE\x88a\x0B\xE8g\r\xE0\xB6\xB3\xA7d\0\0\x85a\x13\x81V[a\x0B\xF2\x91\x90a\x13\xC7V[\x90P`\0a\x0B\xFF\x82a\x14kV[\x90P`\0a\x0C\x0C\x82a\x0F\xBBV[\x90Pg\x1B\xC1mgN\xC8\0\0a\x0C)g\r\xE0\xB6\xB3\xA7d\0\0\x83a\x13\x81V[a\x02G\x91\x90a\x13\xC7V[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x0CKW`\0\x80\xFD[\x04\x92\x91PPV[`\0\x80\x82\x12\x80a\x0CiWPg\x1B\xC1mgN\xC8\0\0\x82\x13[\x15a\x0C\x87W`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\x0C\xA8W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81g\x1B\xC1mgN\xC8\0\0\x03a\x0C\xD0W`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x15a\x0C\xDBW\x91\x90PV[`\0g\r\xE0\xB6\xB3\xA7d\0\0\x83\x12a\r\x03Wa\x0C\xFE\x83g\x1B\xC1mgN\xC8\0\0a\x13\xF5V[a\r\x05V[\x82[\x90P`\0a\r\x1B\x82g\x1B\xC1mgN\xC8\0\0a\x11\x9FV[\x90P\x80`\0\x03a\r>W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\rI\x82a\t\xEFV[\x90P`\0c;\x9A\xCA\0a\rta\roa\rig\x1B\xC1mgN\xC8\0\0a\x14kV[\x85a\x0E\xD0V[a\x06\xC1V[a\r~\x91\x90a\x13\x81V[\x90P`\0\x80a\r\x95\x83g\x03\xC1f\\z\xAB \0a\x0E\xD0V[a\r\xA7\x90g \x05\xFEO&\x8E\xA0\0a\x140V[\x90P`\0a\r\xD7\x84a\r\xC0\x86f\x9F2u$b\xA0\0a\x0E\xD0V[a\r\xD2\x90g\r\xC5R\x7Fd, \0a\x140V[a\x0E\xD0V[a\r\xE9\x90g\r\xE0\xB6\xB3\xA7d\0\0a\x140V[\x90Pa\x0E\rg\t\xD0(\xCCo _\xFF\x19\x85a\x0E\x03\x85\x85a\x11\x9FV[a\r\xD2\x91\x90a\x13\xF5V[\x92PPP`\0[`\x02\x81\x10\x15a\x0E\xA8W`\0\x86a\x0E)\x84a\x0F\xBBV[a\x0E3\x91\x90a\x13\xF5V[\x90P`\0a\x0EA\x84\x85a\x0E\xD0V[a\x0EJ\x90a\x14kV[\x90P`\0a\x0EW\x82a\x08\x17V[\x90P`\0a\x0Ee\x86\x85a\x0E\xD0V[a\x0Ewg\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x84a\x0E\xD0V[a\x0E\x81\x91\x90a\x13\xF5V[\x90Pa\x0E\x8D\x84\x82a\x11\x9FV[a\x0E\x97\x90\x87a\x140V[\x95P\x84`\x01\x01\x94PPPPPa\x0E\x14V[g\r\xE0\xB6\xB3\xA7d\0\0\x88\x12a\x0E\xC5Wa\x0E\xC0\x82a\x14kV[a\x04LV[P\x96\x95PPPPPPV[`\0a\x06\xA5\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x11\xB0V[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x0E\xFDW`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[`\0\x80\x82\x11a\x0FPW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x08uV[P`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x82\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[`\0\x81`\0\x03a\x0F\xD4WPg\r\xE0\xB6\xB3\xA7d\0\0\x91\x90PV[gV\x98\xEE\xF0fp\0\0\x82\x12a\x0F\xEBWP`\0\x91\x90PV[a\x0F\xFCgV\x98\xEE\xF0fp\0\0a\x14kV[\x82\x13a\x10\x11WPg\x1B\xC1mgN\xC8\0\0\x91\x90PV[`\0a\x10\x1C\x83a\x11\xCFV[\x90P`\0a\x10Ug\r\xE0\xB6\xB3\xA7d\0\0a\x10>\x84g\x1B\xC1mgN\xC8\0\0a\x07eV[a\x10P\x90g\r\xE0\xB6\xB3\xA7d\0\0a\x140V[a\x11\x9FV[\x90P`\0\x80\x82a\x10\xB1\x81a\x10\x9E\x81a\x10\x8C\x81a\x10y\x81g\x02_\x0F\xE1\x05\xA3\x14\0a\x0E\xD0V[a\r\xD2\x90g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19a\x140V[a\r\xD2\x90g\x14\xA8EL\x19\xE1\xAC\0a\x140V[a\r\xD2\x90g\x0F\xC1\x0E\x01W\x82w\xFF\x19a\x140V[a\x10\xC3\x90g\x03\xDE\xBD\x08;\x8C|\0a\x140V[\x91P\x83\x90Pa\x11+\x81a\x11\x19\x81a\x11\x07\x81a\x10\xF5\x81a\x10\xE2\x81\x8Ba\x0E\xD0V[a\r\xD2\x90g\x02\x95\xD4\0\xEA2W\xFF\x19a\x140V[a\r\xD2\x90g\x01W\xD8\xB2\xEC\xC7\x08\0a\x140V[a\r\xD2\x90g\x051\n\xA7\xD5!0\0a\x140V[a\r\xD2\x90g\r\xE0\xCC=\x15a\0\0a\x140V[\x91P\x81g\x11\x90\0\xAB\x10\x0F\xFC\0a\x11A\x87\x88a\x0E\xD0V[a\x11M\x90`\0\x19a\x13\x81V[a\x11W\x91\x90a\x13\xF5V[a\x11a\x91\x90a\x140V[\x92PP`\0a\x11o\x83a\x08\x17V[\x90P`\0a\x11}\x85\x83a\x0E\xD0V[\x90P`\0\x88\x12a\x11\x8DW\x80a\x04LV[a\x04L\x81g\x1B\xC1mgN\xC8\0\0a\x13\xF5V[`\0a\x06\xA5\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x05\x85\x14\x17\x16a\x11\xC8W`\0\x80\xFD[\x05\x92\x91PPV[`\0`\x01`\xFF\x1B\x82\x03a\x11\xF5W`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x12\x15a\x12\x06WP\x19`\x01\x01\x90V[P\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\x12uWa\x12ua\x12\nV[PP\x835\x95` \x85\x015\x95P`@\x85\x015\x94``\x81\x015\x94P`\x80\x015\x92P\x90PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x12\xB1Wa\x12\xB1a\x12\nV[PP\x825\x94` \x84\x015\x94P`@\x84\x015\x93``\x015\x92P\x90PV[`\0` \x82\x84\x03\x12\x15a\x12\xE2Wa\x12\xE2a\x12\nV[P5\x91\x90PV[`\0\x80`\0\x80`\0\x80`\0\x80a\x01\0\x89\x8B\x03\x12\x15a\x13\tWa\x13\ta\x12\nV[PP\x865\x98` \x88\x015\x98P`@\x88\x015\x97``\x81\x015\x97P`\x80\x81\x015\x96P`\xA0\x81\x015\x95P`\xC0\x81\x015\x94P`\xE0\x015\x92P\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x02\x85Wa\x02\x85a\x13AV[\x81\x81\x03\x81\x81\x11\x15a\x02\x85Wa\x02\x85a\x13AV[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15a\x13\x9DWa\x13\x9Da\x13AV[\x81\x81\x05\x83\x14\x82\x15\x17a\x02\x85Wa\x02\x85a\x13AV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a\x13\xD6Wa\x13\xD6a\x13\xB1V[`\x01`\xFF\x1B\x82\x14`\0\x19\x84\x14\x16\x15a\x13\xF0Wa\x13\xF0a\x13AV[P\x05\x90V[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a\x14\x15Wa\x14\x15a\x13AV[P\x92\x91PPV[`\0\x82a\x14+Wa\x14+a\x13\xB1V[P\x04\x90V[\x80\x82\x01\x82\x81\x12`\0\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a\x14PWa\x14Pa\x13AV[PP\x92\x91PPV[\x80\x82\x01\x80\x82\x11\x15a\x02\x85Wa\x02\x85a\x13AV[`\0`\x01`\xFF\x1B\x82\x01a\x14\x80Wa\x14\x80a\x13AV[P`\0\x03\x90V";
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
        ///Calls the contract's `computeOutputXGivenY` (0xcf25c3ef) function
        pub fn compute_output_x_given_y(
            &self,
            x: ::ethers::core::types::U256,
            delta_x: ::ethers::core::types::U256,
            y: ::ethers::core::types::U256,
            delta_y: ::ethers::core::types::U256,
            l: ::ethers::core::types::U256,
            delta_l: ::ethers::core::types::U256,
            k: ::ethers::core::types::U256,
            sigma: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash(
                    [207, 37, 195, 239],
                    (x, delta_x, y, delta_y, l, delta_l, k, sigma),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `computeOutputYGivenX` (0xe3908f6d) function
        pub fn compute_output_y_given_x(
            &self,
            x: ::ethers::core::types::U256,
            delta_x: ::ethers::core::types::U256,
            y: ::ethers::core::types::U256,
            delta_y: ::ethers::core::types::U256,
            l: ::ethers::core::types::U256,
            delta_l: ::ethers::core::types::U256,
            k: ::ethers::core::types::U256,
            sigma: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash(
                    [227, 144, 143, 109],
                    (x, delta_x, y, delta_y, l, delta_l, k, sigma),
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
        pub delta_x: ::ethers::core::types::U256,
        pub y: ::ethers::core::types::U256,
        pub delta_y: ::ethers::core::types::U256,
        pub l: ::ethers::core::types::U256,
        pub delta_l: ::ethers::core::types::U256,
        pub k: ::ethers::core::types::U256,
        pub sigma: ::ethers::core::types::U256,
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
        pub delta_x: ::ethers::core::types::U256,
        pub y: ::ethers::core::types::U256,
        pub delta_y: ::ethers::core::types::U256,
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
