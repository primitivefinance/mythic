pub use log_normal_solver::*;
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
pub mod log_normal_solver {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                    name: ::std::borrow::ToOwned::to_owned("_strategy"),
                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                    internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                        "address"
                    ),),
                },],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("BISECTION_EPSILON"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("BISECTION_EPSILON"),
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
                    ::std::borrow::ToOwned::to_owned("MAX_BISECTION_ITERS"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("MAX_BISECTION_ITERS",),
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
                    ::std::borrow::ToOwned::to_owned("allocateGivenX"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("allocateGivenX"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("poolId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amountX"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
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
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("allocateGivenY"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("allocateGivenY"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("poolId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amountY"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
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
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("deallocateGivenX"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("deallocateGivenX"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("poolId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amountX"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
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
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("deallocateGivenY"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("deallocateGivenY"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("poolId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amountY"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
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
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("fetchPoolParams"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("fetchPoolParams"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("poolId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Address,
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct LogNormal.LogNormalParams",
                                ),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getInitialPoolData"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getInitialPoolData"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("rx"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("S"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("params"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned(
                                        "struct LogNormal.LogNormalParams",
                                    ),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getNextLiquidity"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getNextLiquidity"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("poolId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("rx"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("ry"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("L"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
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
                    ::std::borrow::ToOwned::to_owned("getNextReserveX"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getNextReserveX"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("poolId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("ry"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("L"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("S"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
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
                    ::std::borrow::ToOwned::to_owned("getNextReserveY"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getNextReserveY"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("poolId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("rx"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("L"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("S"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
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
                    ::std::borrow::ToOwned::to_owned("getPriceGivenXL"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getPriceGivenXL"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("poolId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("rx"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("L"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("price"),
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
                    ::std::borrow::ToOwned::to_owned("getPriceGivenYL"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getPriceGivenYL"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("poolId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("ry"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("L"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("price"),
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
                    ::std::borrow::ToOwned::to_owned("getReservesAndLiquidity"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getReservesAndLiquidity",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("poolId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
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
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("internalPrice"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("internalPrice"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("poolId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("price"),
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
                    ::std::borrow::ToOwned::to_owned("prepareControllerUpdate"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("prepareControllerUpdate",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("controller"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("prepareFeeUpdate"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("prepareFeeUpdate"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("swapFee"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("prepareSigmaUpdate"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("prepareSigmaUpdate"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("targetSigma"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("targetTimestamp"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("prepareStrikeUpdate"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("prepareStrikeUpdate",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("targetStrike"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("targetTimestamp"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("prepareTauUpdate"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("prepareTauUpdate"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("targetTau"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("targetTimestamp"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("simulateSwap"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("simulateSwap"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("poolId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
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
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bool"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
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
                    ::std::borrow::ToOwned::to_owned("strategy"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("strategy"),
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
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("BisectionLib_InvalidBounds"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("BisectionLib_InvalidBounds",),
                        inputs: ::std::vec![
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
                        ],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("BisectionLib_RootOutsideBounds"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("BisectionLib_RootOutsideBounds",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("lowerResult"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("upperResult"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int256"),
                                ),
                            },
                        ],
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
                    ::std::borrow::ToOwned::to_owned("OutOfBounds"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("OutOfBounds"),
                        inputs: ::std::vec![],
                    },],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static LOGNORMALSOLVER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80\x80`@R4a\0\xE8W`@Q`\x1Fb\0,\x038\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x91`\x01`\x01`@\x1B\x03\x83\x11\x84\x84\x10\x17a\0\xD2W\x80\x84\x92` \x94`@R\x839\x81\x01\x03\x12a\0\x82WQ`\x01`\x01`\xA0\x1B\x03\x81\x16\x90\x81\x90\x03a\0}W`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x91\x90\x91\x17\x90U`@Qa*\xCD\x90\x81b\0\x016\x829\xF3[`\0\x80\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[bF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01Ra7\xB7`\xF1\x1B`d\x82\x01R`\x84\x90\xFD\xFE`\x80`@R`\x046\x10a\x0B3W`\x005`\xE0\x1C\x80c\x04 X\n\x14a\x01lW\x80c\x12\x06I\xC5\x14a\x01gW\x80c\x13N\xAD\x12\x14a\x01bW\x80c\x1E\x97\x8C\xB0\x14a\x01]W\x80c9(\xFF\x97\x14a\x01XW\x80c;&\x8D]\x14a\x01SW\x80c;M\x100\x14a\x01NW\x80cN\x81\x7F\xD9\x14a\x01IW\x80c^\xB4\x08\xFC\x14a\x01DW\x80cb7V\x9F\x14a\x01?W\x80cme\"\x99\x14a\x01:W\x80c\x7F\x17@\x9C\x14a\x015W\x80c\x81\xB5\xFA\xC2\x14a\x010W\x80c\xA8\xC6.v\x14a\x01+W\x80c\xAFNC\x7F\x14a\x01&W\x80c\xB0\x9D\x04\xE5\x14a\x01!W\x80c\xCB\x1FU2\x14a\x01\x1CW\x80c\xCE\x15;\xF4\x14a\x01\x17W\x80c\xE9G\x16\xD5\x14a\x01\x12W\x80c\xEE>\x8C\xFB\x14a\x01\rW\x80c\xF3\r7\xF2\x14a\x01\x08Wc\xF9\xC2\x82\x11\x03a\x0B3Wa\x0B\x17V[a\n\xE7V[a\n\xB6V[a\n{V[a\n?V[a\t\xFAV[a\t\xC7V[a\t\xABV[a\t\x82V[a\tUV[a\x08\xB3V[a\x08\x97V[a\x08*V[a\x08\x0EV[a\x07\xF2V[a\x07\xC3V[a\x07\x88V[a\x04\xEBV[a\x04\x97V[a\x04\x04V[a\x02\xE8V[a\x02mV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01Ra7\xB7`\xF1\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[`\0[\x83\x81\x10a\x02$WPP`\0\x91\x01RV[\x81\x81\x01Q\x83\x82\x01R` \x01a\x02\x14V[\x90` \x91a\x02M\x81Q\x80\x92\x81\x85R\x85\x80\x86\x01\x91\x01a\x02\x11V[`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[\x90` a\x02j\x92\x81\x81R\x01\x90a\x024V[\x90V[4a\x02\xC5W`@6`\x03\x19\x01\x12a\x02\xC0Wa\x02\xBC`@Q`\x02` \x82\x01R`\x045`@\x82\x01R`$5``\x82\x01R``\x81Ra\x02\xA8\x81a\x03\x94V[`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x024V[\x03\x90\xF3[a\x01\xC1V[a\x01qV[`\x80\x90`\x03\x19\x01\x12a\x02\xC0W`\x045\x90`$5\x90`D5\x90`d5\x90V[4a\x02\xC5W` a\x03\x04a\x02\xFB6a\x02\xCAV[\x92\x91\x90\x91a\x0C\x1BV[`@Q\x90\x81R\xF3[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FABI decoding: struct data too sh`D\x82\x01Rb\x1B\xDC\x9D`\xEA\x1B`d\x82\x01R`\x84\x90\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\xA0\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x03\x8FW`@RV[a\x03]V[`\x80\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x03\x8FW`@RV[``\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x03\x8FW`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x03\x8FW`@RV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\x03\xFFWV[`\0\x80\xFD[4a\x02\xC5W`\xE06`\x03\x19\x01\x12a\x02\xC0W`\xA06`C\x19\x01\x12a\x04xWa\x02\xBCa\x04l`@Qa\x043\x81a\x03sV[`D5\x81R`d5` \x82\x01R`\x845`@\x82\x01R`\xA45``\x82\x01R`\xC45a\x04\\\x81a\x03\xEEV[`\x80\x82\x01R`$5`\x045a\x15\xB7V[`@Q\x91\x82\x91\x82a\x02YV[a\x03\x0CV[``\x90`\x03\x19\x01\x12a\x02\xC0W`\x045\x90`$5\x90`D5\x90V[4a\x02\xC5W` a\x03\x04a\x04\xB6a\x04\xAD6a\x04}V[\x91\x92\x90\x92a\x112V[\x91a\x17}V[\x80\x15\x15\x03a\x03\xFFWV[\x90\x92`\x80\x92a\x02j\x95\x94\x15\x15\x83R` \x83\x01R`@\x82\x01R\x81``\x82\x01R\x01\x90a\x024V[4a\x02\xC5W``6`\x03\x19\x01\x12a\x02\xC0W`$5`\x045a\x05\x0B\x82a\x04\xBCV[`D5\x90a\x05^a\x05\x1Aa\r\x12V[\x92a\x05#a\r\x12V[\x93a\x05-\x84a\x12\xB9V[` \x84\x99\x93\x95\x92\x99\x01\x94`@\x99\x8A\x86\x01\x92\x83R\x86R\x84Ra\x05M\x87a\x112V[\x95\x86\x91\x85Q\x90\x87Q\x90Q\x91\x8Aa\x11\xE0V[\x92\x15a\x07\0W\x92\x82a\x05\xA5a\x05\xAC\x93a\x05\x9Ea\x05\x99a\x05\x91a\x05\xCA\x98a\x05\x8C``a\x05\xF2\x9D\x9C\x01Q\x86a\x1F\xBAV[a\x1F\xBAV[\x86Q\x90a \x10V[a\rXV[\x93Qa\rkV[\x89Ra\rkV[a\x05\xBE\x88\x88\x01\x91\x80\x83R\x88Q\x88a\r\x05V[\x90\x87Q\x90Q\x90\x87a\x0C\x1BV[\x90a\x05\xE9a\x05\xDE` \x88\x01\x93\x80\x85Ra\rXV[\x80\x84R\x82Q\x11a\r\xDCV[Q\x90Q\x90a\r\xCFV[\x93[\x83Q\x91` \x85\x01Q\x92a\x066\x83\x87\x01\x91a\x06(\x83Q\x86Q\x97\x88\x93` \x85\x01`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[\x03`\x1F\x19\x81\x01\x86R\x85a\x03\xCCV[`\0Ta\x06Y\x90a\x06M\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x80;\x15a\x06\xFBW\x84`\xC0\x91a\x06\x84\x97\x86Q\x80\x99\x81\x94\x82\x93c\r\x17\xA7\xC7`\xE3\x1B\x84R0`\x04\x85\x01a\x0EjV[\x03\x91Z\xFA\x94\x85\x15a\x06\xF6W`\0\x95a\x06\xB6W[P\x90a\x06\xAB\x91a\x02\xBC\x95\x96Q\x90Q\x90a\x17}V[\x90Q\x94\x85\x94\x85a\x04\xC6V[a\x02\xBC\x95P\x90a\x06\xE1a\x06\xAB\x93\x92`\xC0=`\xC0\x11a\x06\xEFW[a\x06\xD9\x81\x83a\x03\xCCV[\x81\x01\x90a\x0E3V[PPPPP\x95P\x90\x91a\x06\x97V[P=a\x06\xCFV[a\x0C\x0FV[a\x0B\x96V[\x82a\x07Ia\x07\x82\x96a\x07<a\x07n\x95a\x075a\x05\x99a\x07-a\x07y\x9Aa\x05\x8C``a\x07f\x9B\x01Q\x86a\x1F\xBAV[\x85Q\x90a \x10V[\x92Qa\rkV[\x92` \x8C\x01\x93\x84Ra\rkV[a\x07[\x8B\x8B\x01\x91\x80\x83R\x83Q\x8Ba\x0E\x8EV[\x91Q\x90Q\x90\x89a\x0E\x9BV[\x80\x88Ra\rXV[\x80\x87R\x82Q\x11a\rxV[Q\x84Q\x90a\r\xCFV[\x93a\x05\xF4V[4a\x02\xC5W`@6`\x03\x19\x01\x12a\x02\xC0Wa\x02\xBC`@Q`\x04` \x82\x01R`\x045`@\x82\x01R`$5``\x82\x01R``\x81Ra\x02\xA8\x81a\x03\x94V[4a\x02\xC5W` 6`\x03\x19\x01\x12a\x02\xC0W` a\x03\x04`\x045a\x04\xB6a\x07\xE8\x82a\x12\xB9V[\x92\x91\x93\x90Pa\x112V[4a\x02\xC5W` a\x03\x04a\x08\x08a\x04\xAD6a\x04}V[\x91a\x19jV[4a\x02\xC5W` a\x03\x04a\x08!6a\x02\xCAV[\x92\x91\x90\x91a\x0E\x9BV[4a\x02\xC5W`@6`\x03\x19\x01\x12a\x02\xC0Wa\x08y`\x045a\x02\xBCa\x08[a\x08P\x83a\x12\xB9V[\x91\x90P`$5a\x1BgV[\x93\x90\x92\x84\x84a\x08sa\x08l\x84a\x112V[\x83\x83a\x17}V[\x92a\x0C\x1BV[\x92`@Q\x93\x84\x93\x84`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[4a\x02\xC5W`\x006`\x03\x19\x01\x12a\x02\xC0W` `@Q`\0\x81R\xF3[4a\x02\xC5W`@6`\x03\x19\x01\x12a\x02\xC0W`\x045a\t\x01a\x02\xBCa\x08\xE3a\x08\xD9\x84a\x12\xB9V[\x91P`$5a\x1B\x94V[\x92\x90\x93\x83\x85a\x08\xFBa\x08\xF4\x84a\x112V[\x83\x83a\x19jV[\x92a\x0E\x9BV[\x91`@Q\x93\x84\x93\x84`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[\x80Q\x82R` \x80\x82\x01Q\x90\x83\x01R`@\x80\x82\x01Q\x90\x83\x01R``\x80\x82\x01Q\x90\x83\x01R`\x80\x90\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x91\x01RV[4a\x02\xC5W` 6`\x03\x19\x01\x12a\x02\xC0W`\xA0a\ts`\x045a\x112V[a\t\x80`@Q\x80\x92a\t\x1FV[\xF3[4a\x02\xC5W`\x006`\x03\x19\x01\x12a\x02\xC0W`\0T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x02\xC5W` a\x03\x04a\t\xBE6a\x02\xCAV[\x92\x91\x90\x91a\x11\xE0V[4a\x02\xC5W` 6`\x03\x19\x01\x12a\x02\xC0Wa\x02\xBC`@Q`\x01` \x82\x01R`\x045`@\x82\x01R`@\x81Ra\x02\xA8\x81a\x03\xB0V[4a\x02\xC5W` 6`\x03\x19\x01\x12a\x02\xC0Wa\x02\xBC`\x045a\n\x1A\x81a\x03\xEEV[`@\x80Q`\x05` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x82\x82\x01R\x81Ra\x02\xA8\x81a\x03\xB0V[4a\x02\xC5W` 6`\x03\x19\x01\x12a\x02\xC0Wa\x02\xBCa\n^`\x045a\x12\xB9V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R\x90\x81\x90``\x82\x01\x90V[4a\x02\xC5W`@6`\x03\x19\x01\x12a\x02\xC0Wa\x02\xBC`@Q`\x03` \x82\x01R`\x045`@\x82\x01R`$5``\x82\x01R``\x81Ra\x02\xA8\x81a\x03\x94V[4a\x02\xC5W`@6`\x03\x19\x01\x12a\x02\xC0Wa\x08y`\x045a\x02\xBCa\x08[a\n\xDC\x83a\x12\xB9V[\x91\x90P`$5a\x1B\x94V[4a\x02\xC5W`@6`\x03\x19\x01\x12a\x02\xC0W`\x045a\t\x01a\x02\xBCa\x08\xE3a\x0B\r\x84a\x12\xB9V[\x91P`$5a\x1BgV[4a\x02\xC5W`\x006`\x03\x19\x01\x12a\x02\xC0W` `@Q`x\x81R\xF3[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01Rtnor receive functions`X\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01Rd code`\xD8\x1B`d\x82\x01R`\x84\x90\xFD[\x90\x81` \x91\x03\x12a\x02\xC0WQ\x90V[`@\x90a\x02j\x93\x92\x81R\x81` \x82\x01R\x01\x90a\x024V[`@Q=`\0\x82>=\x90\xFD[\x90\x91\x92a\x0C2\x90a\x0C+\x83a\x112V[\x90\x85a\x13\xB2V[\x90`@Q\x93a\x0Ci\x85a\x0C[\x83\x86\x88` \x85\x01`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[\x03`\x1F\x19\x81\x01\x87R\x86a\x03\xCCV[`\0Ta\x0C\x80\x90a\x06M\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x80;\x15a\x06\xFBWa\x0C\xA9\x95` \x91`@Q\x80\x80\x99\x81\x94b.RK`\xE0\x1B\x83R\x88`\x04\x84\x01a\x0B\xF8V[\x03\x91Z\xFA\x91\x82\x15a\x06\xF6Wa\x02j\x95`\0\x93a\x0C\xD0W[Pa\x0C\xCA\x90a\x112V[\x93a\x14\0V[a\x0C\xCA\x91\x93Pa\x0C\xF7\x90` =` \x11a\x0C\xFEW[a\x0C\xEF\x81\x83a\x03\xCCV[\x81\x01\x90a\x0B\xE9V[\x92\x90a\x0C\xC0V[P=a\x0C\xE5V[\x91a\x04\xB6a\x02j\x93a\x112V[`@Q\x90``\x82\x01\x82\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x03\x8FW`@R`\0`@\x83\x82\x81R\x82` \x82\x01R\x01RV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x90`\x01\x82\x01\x80\x92\x11a\rfWV[a\rBV[\x91\x90\x82\x01\x80\x92\x11a\rfWV[\x15a\r\x7FWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: x reserve increase`D\x82\x01Rad!`\xF0\x1B`d\x82\x01R`\x84\x90\xFD[\x91\x90\x82\x03\x91\x82\x11a\rfWV[\x15a\r\xE3WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: y reserve increase`D\x82\x01Rad!`\xF0\x1B`d\x82\x01R`\x84\x90\xFD[\x91\x90\x82`\xC0\x91\x03\x12a\x02\xC0W\x81Qa\x0EJ\x81a\x04\xBCV[\x91` \x81\x01Q\x91`@\x82\x01Q\x91``\x81\x01Q\x91`\xA0`\x80\x83\x01Q\x92\x01Q\x90V[a\x02j\x93\x92``\x92`\x01\x80`\xA0\x1B\x03\x16\x82R` \x82\x01R\x81`@\x82\x01R\x01\x90a\x024V[\x91a\x08\x08a\x02j\x93a\x112V[\x90\x91\x92a\x0E\xB2\x90a\x0E\xAB\x83a\x112V[\x90\x85a\x19\xE4V[\x90`@Q\x93a\x0E\xDB\x85a\x0C[\x83\x87\x87` \x85\x01`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[`\0Ta\x0E\xF2\x90a\x06M\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x80;\x15a\x06\xFBWa\x0F\x1B\x95` \x91`@Q\x80\x80\x99\x81\x94b.RK`\xE0\x1B\x83R\x88`\x04\x84\x01a\x0B\xF8V[\x03\x91Z\xFA\x91\x82\x15a\x06\xF6Wa\x02j\x95`\0\x93a\x0FBW[Pa\x0F<\x90a\x112V[\x93a\x1A\x11V[a\x0F<\x91\x93Pa\x0F`\x90` =` \x11a\x0C\xFEWa\x0C\xEF\x81\x83a\x03\xCCV[\x92\x90a\x0F2V[\x91\x90` \x92\x83\x81\x83\x03\x12a\x02\xC0W\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x82\x82\x11a\x10\x7FW\x01\x82`\x1F\x82\x01\x12\x15a\x10&W\x80Q\x91\x82\x11a\x03\x8FW`@Q\x92a\x0F\xB4`\x1F\x84\x01`\x1F\x19\x16\x87\x01\x85a\x03\xCCV[\x82\x84R\x85\x83\x83\x01\x01\x11a\x0F\xD1W\x84\x83\x94\x95a\x02j\x94\x01\x91\x01a\x02\x11V[`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x86\x90R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x86\x90R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x87\x90R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[\x91\x90\x82`\xA0\x91\x03\x12a\x04xW`@Qa\x10\xE7\x81a\x03sV[`\x80\x80\x82\x94\x80Q\x84R` \x81\x01Q` \x85\x01R`@\x81\x01Q`@\x85\x01R``\x81\x01Q``\x85\x01R\x01Q\x91a\x11\x1A\x83a\x03\xEEV[\x01RV[\x90`\xA0\x82\x82\x03\x12a\x02\xC0Wa\x02j\x91a\x10\xCFV[`\x80\x90`@Qa\x11A\x81a\x03sV[`\0\x92\x81\x84\x80\x93R\x82` \x82\x01R\x82`@\x82\x01R\x82``\x82\x01R\x01Ra\x11sa\x06Ma\x06M\x84T`\x01\x80`\xA0\x1B\x03\x16\x90V[\x80;\x15a\x06\xFBW`@Qc\xDC\x17\x83U`\xE0\x1B\x81R`\x04\x81\x01\x92\x90\x92R\x82\x90\x82\x90`$\x90\x82\x90Z\xFA\x90\x81\x15a\x06\xF6W\x82a\x02j\x93\x92a\x11\xBDW[PP` \x80\x82Q\x83\x01\x01\x91\x01a\x11\x1EV[a\x11\xD9\x92P=\x80\x91\x83>a\x11\xD1\x81\x83a\x03\xCCV[\x81\x01\x90a\x0FgV[8\x80a\x11\xACV[`@\x80Q` \x81\x01\x84\x90R\x90\x81\x01\x84\x90R``\x81\x01\x85\x90R\x93\x91\x92a\x12\x08\x85`\x80\x81\x01a\x0C[V[`\0T`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x06\xFBWa\x12=\x95` \x91`@Q\x80\x80\x99\x81\x94b.RK`\xE0\x1B\x83R\x88`\x04\x84\x01a\x0B\xF8V[\x03\x91Z\xFA\x91\x82\x15a\x06\xF6Wa\x02j\x95`\0\x93a\x12dW[Pa\x12^\x90a\x112V[\x93a\x1B\xBBV[a\x12^\x91\x93Pa\x12\x82\x90` =` \x11a\x0C\xFEWa\x0C\xEF\x81\x83a\x03\xCCV[\x92\x90a\x12TV[\x90\x81` \x91\x03\x12a\x02\xC0WQa\x02j\x81a\x03\xEEV[\x90\x81``\x91\x03\x12a\x02\xC0W\x80Q\x91`@` \x83\x01Q\x92\x01Q\x90V[`\0T\x90\x91\x90a\x12\xD3\x90a\x06M\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x80;\x15a\x06\xFBW` `\x04\x91`@Q\x92\x83\x80\x92c+\xEE\x84\xF1`\xE2\x1B\x82RZ\xFA\x90\x81\x15a\x06\xF6W`\0\x91a\x13\x83W[P`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x06\xFBW`@Qc3\x85N\xFD`\xE2\x1B\x81R`\x04\x81\x01\x93\x90\x93R``\x90\x83\x90`$\x90\x82\x90Z\xFA\x90\x81\x15a\x06\xF6W`\0\x80\x93`\0\x93a\x13LW[P\x92\x91\x90V[\x91\x93PPa\x13r\x91P``=``\x11a\x13|W[a\x13j\x81\x83a\x03\xCCV[\x81\x01\x90a\x12\x9EV[\x92\x90\x92\x918a\x13FV[P=a\x13`V[a\x13\xA5\x91P` =` \x11a\x13\xABW[a\x13\x9D\x81\x83a\x03\xCCV[\x81\x01\x90a\x12\x89V[8a\x13\x01V[P=a\x13\x93V[a\x05\x8C\x90a\x13\xD4a\x13\xCFa\x13\xCA\x86a\x02j\x97\x96a\x1D\x11V[a\x1D\x97V[a\x1D\xCAV[\x92Qa\x1F\xBAV[a\x13\xFE\x93``\x92\x96\x95\x93a\x01\0\x83\x01\x97\x83R` \x83\x01R`@\x82\x01R\x01\x90a\t\x1FV[V[\x92\x93`\0\x93\x85\x92\x91\x85\x85\x12\x15a\x15\x84W[\x85\x85\x12a\x15eW\x90a\x143a\x14A\x92[`@\x96`@Q\x95\x86\x94` \x86\x01a\x13\xDBV[\x03`\x1F\x19\x81\x01\x83R\x82a\x03\xCCV[\x81\x85\x92\x85\x96\x82\x81\x11a\x15BWa\x14W\x81\x85a*4V[\x92a\x14b\x81\x86a*4V[\x88a\x14m\x82\x87a\x17)V[\x13a\x15!WP\x90a\x14\x81\x91\x97\x96\x92\x97a\r\xCFV[`\x01\x95\x91\x82\x91\x87\x80[a\x14\x9CW[PPPPPPPPPP\x90V[\x15a\x14\xFDW[P\x86\x97\x98P\x81\x92a\x14\xBCa\x14\xB6\x8B\x89a\rkV[`\x01\x1C\x90V[\x99a\x14\xC7\x8B\x88a*4V[\x90\x84a\x14\xD3\x88\x84a\x17)V[\x13a\x14\xF1WPP\x89\x93[\x88a\x14\xE8\x89\x87a\r\xCFV[\x92\x01\x94\x99a\x14\x8AV[\x8B\x98P\x90\x95P\x93a\x14\xDDV[`\x14\x10\x80a\x15\x18W[\x15a\x15\x11W\x88a\x14\xA2V[\x80\x80a\x14\x8FV[P\x80\x83\x10a\x15\x06V[`@Qc\x06\xF1\xBE]`\xE2\x1B\x81R`\x04\x81\x01\x86\x90R`$\x81\x01\x91\x90\x91R`D\x90\xFD[`@Qc0\x82\xDF\xDB`\xE1\x1B\x81R`\x04\x81\x01\x91\x90\x91R`$\x81\x01\x92\x90\x92RP`D\x90\xFD[\x93P\x91a\x15q\x90a\x1F\xE6V[\x91a\x15~\x84\x83\x85\x84a\x1E\xC1V[\x93a\x14\x11V[\x85\x85\x13a\x15\x98W\x90a\x143a\x14A\x92a\x14!V[\x93P\x94a\x15\xA4\x90a\x1E\rV[\x94a\x15\xB1\x84\x83\x88\x84a\x1E\xC1V[\x93a\x15\x84V[\x91a\x15\xC8a\x13\xCFa\x13\xCA\x83\x85a#\xF4V[\x91g\r\xE0\xB6\xB3\xA7d\0\0\x92\x83\x03\x92\x83\x11a\rfWa\x16\x1F\x82a\x16\x0Ba\x16\0a\x13\xCFa\x13\xCA\x84a\x15\xFAa\x16=\x9A\x8Ca \x10V[\x97a\x1D\x11V[a\x05\x8C\x85\x84Qa\x1F\xBAV[\x92a\x16\x18\x82\x82\x86\x8Aa\x1E\xC1V[\x84\x88a\x1B\xBBV[\x90`@Q\x94` \x86\x01R`@\x85\x01R``\x84\x01R`\x80\x83\x01\x90a\t\x1FV[a\x01\0\x81Ra\x01 \x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x03\x8FW`@R\x90V[\x90g\x11\x90\0\xAB\x10\x0F\xFB\xFF\x19\x82\x01\x91\x82\x13`\x01\x16a\rfWV[\x90g\x1B\xC1mgN\xC8\0\0`\0\x83\x82\x03\x93\x12\x81\x84\x12\x81\x16\x91\x84\x13\x90\x15\x16\x17a\rfWV[\x81\x81\x03\x92\x91`\0\x13\x80\x15\x82\x85\x13\x16\x91\x84\x12\x16\x17a\rfWV[\x90g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x81\x02\x92\x81\x84\x05\x14\x90\x15\x17\x15a\rfWV[\x90g\x1B\xC1mgN\xC8\0\0\x91\x82\x81\x02\x92\x81\x84\x05\x14\x90\x15\x17\x15a\rfWV[`\0\x81\x90\x03\x91\x90`\x01`\xFF\x1B\x81\x14`\x01\x16a\rfW`\0\x19\x83\x05\x03a\rfWV[\x90c;\x9A\xCA\0\x91\x82\x81\x02\x92\x81\x84\x05\x14\x90\x15\x17\x15a\rfWV[\x81\x81\x02\x92\x91`\0\x82\x12`\x01`\xFF\x1B\x82\x14\x16a\rfW\x81\x84\x05\x14\x90\x15\x17\x15a\rfWV[\x81\x15a\x17gW`\x01`\xFF\x1B\x81\x14`\0\x19\x83\x14\x16a\rfW\x05\x90V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[\x91\x90\x91a\x17\xB8` \x83\x01\x93a\x17\xB2\x85Qa\x17\xAAa\x17\xA0`@\x88\x01\x92\x83Q\x90a eV[\x97Q\x82Q\x90a \x8EV[\x90Q\x90a\x1E,V[\x92a\x1EMV[\x92g\r\xE0\xB6\xB3\xA7d\0\0\x80\x85\x12\x15a\x186W`\0\x85\x13\x15a\x18+W`\0\x85\x82\x03\x95\x12\x81\x86\x12\x81\x16\x91\x86\x13\x90\x15\x16\x17a\rfWa\x18\x1Fa\x18$\x92a\x18\x1Aa\x18\x0Ca\x13\xCF\x94a\x18\x07a\x02j\x99a \xADV[a\x17)V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[a\x16\x9CV[a\"pV[\x90Qa\x1F\xBAV[PPPPP`\0\x19\x90V[PPPPP`\0\x90V[\x90\x81g\r\xE0\xB6\xB3\xA7d\0\0\x01\x91\x82\x12`\x01\x16a\rfWV[\x90\x81g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19\x01\x91\x82\x12\x15`\x01\x16a\rfWV[\x90\x81g\x14\xA8EL\x19\xE1\xAC\0\x01\x91\x82\x12`\x01\x16a\rfWV[\x90\x81g\x0F\xC1\x0E\x01W\x82w\xFF\x19\x01\x91\x82\x12\x15`\x01\x16a\rfWV[\x90\x81g\x03\xDE\xBD\x08;\x8C|\0\x01\x91\x82\x12`\x01\x16a\rfWV[\x90\x81g\x02\x95\xD4\0\xEA2W\xFF\x19\x01\x91\x82\x12\x15`\x01\x16a\rfWV[\x90\x81g\x01W\xD8\xB2\xEC\xC7\x08\0\x01\x91\x82\x12`\x01\x16a\rfWV[\x90\x81g\x051\n\xA7\xD5!0\0\x01\x91\x82\x12`\x01\x16a\rfWV[\x90\x81g\r\xE0\xCC=\x15a\0\0\x01\x91\x82\x12`\x01\x16a\rfWV[\x90\x81g \x05\xFEO&\x8E\xA0\0\x01\x91\x82\x12`\x01\x16a\rfWV[\x90\x81g\r\xC5R\x7Fd, \0\x01\x91\x82\x12`\x01\x16a\rfWV[\x91\x90\x91`\0\x83\x82\x01\x93\x84\x12\x91\x12\x90\x80\x15\x82\x16\x91\x15\x16\x17a\rfWV[\x91\x90\x91a\x19\xA8` \x83\x01\x91a\x19\xA2a\x19\x9A\x84Qa\x17\xAAa\x19\x90`@\x89\x01\x92\x83Q\x90a eV[\x96Q\x82Q\x90a \x8EV[\x95\x85Qa\x1E,V[\x90a\x1EMV[g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x82\x12\x15a\x186W`\0\x82\x13\x15a\x18+Wa\x02j\x94a\x18$\x93a\x19\xDEa\x18\x1F\x93a\x18\x07a\x13\xCF\x96a \xADV[\x05a\x19NV[\x91a\x13\xCAa\x13\xCF\x91a\x19\xF5\x93a#\xF4V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x03\x90\x81\x11a\rfWa\x02j\x91a\x1F\xBAV[\x92\x93`\0\x93\x85\x92\x91\x85\x85\x12\x15a\x1B4W[\x85\x85\x12a\x1B\x15W\x90a\x143a\x1AC\x92`@\x96`@Q\x95\x86\x94` \x86\x01a\x13\xDBV[\x81\x85\x92\x85\x96\x82\x81\x11a\x15BWa\x1AY\x81\x85a*UV[\x92a\x1Ad\x81\x86a*UV[\x88a\x1Ao\x82\x87a\x17)V[\x13a\x15!WP\x90a\x1A\x83\x91\x97\x96\x92\x97a\r\xCFV[`\x01\x95\x91\x82\x91\x87\x80[a\x1A\x9DWPPPPPPPPPP\x90V[\x15a\x1A\xF8W[P\x86\x97\x98P\x81\x92a\x1A\xB7a\x14\xB6\x8B\x89a\rkV[\x99a\x1A\xC2\x8B\x88a*UV[\x90\x84a\x1A\xCE\x88\x84a\x17)V[\x13a\x1A\xECWPP\x89\x93[\x88a\x1A\xE3\x89\x87a\r\xCFV[\x92\x01\x94\x99a\x1A\x8CV[\x8B\x98P\x90\x95P\x93a\x1A\xD8V[`\x14\x10\x80a\x1B\x0CW[\x15a\x15\x11W\x88a\x1A\xA3V[P\x80\x83\x10a\x1B\x01V[\x93P\x91a\x1B!\x90a\x1F\xE6V[\x91a\x1B.\x84\x83\x83\x86a\x1E\xC1V[\x93a\x1A\"V[\x85\x85\x13a\x1BHW\x90a\x143a\x1AC\x92a\x14!V[\x93P\x94a\x1BT\x90a\x1E\rV[\x94a\x1Ba\x84\x83\x83\x89a\x1E\xC1V[\x93a\x1B4V[\x92\x91\x90a\x1B}a\x1Bw\x82\x84a \x10V[\x85a\x1F\xBAV[\x93\x81\x03\x90\x81\x11a\rfW\x92\x81\x03\x90\x81\x11a\rfW\x90V[\x92\x91\x90a\x1B\xA4a\x1Bw\x82\x84a \x10V[\x93\x81\x01\x80\x91\x11a\rfW\x92\x81\x01\x80\x91\x11a\rfW\x90V[\x92\x93`\0\x93\x85\x92\x91\x85\x85\x12\x15a\x1C\xDEW[\x85\x85\x12a\x1C\xBFW\x90a\x143a\x1B\xED\x92`@\x96`@Q\x95\x86\x94` \x86\x01a\x13\xDBV[\x81\x85\x92\x85\x96\x82\x81\x11a\x15BWa\x1C\x03\x81\x85a*wV[\x92a\x1C\x0E\x81\x86a*wV[\x88a\x1C\x19\x82\x87a\x17)V[\x13a\x15!WP\x90a\x1C-\x91\x97\x96\x92\x97a\r\xCFV[`\x01\x95\x91\x82\x91\x87\x80[a\x1CGWPPPPPPPPPP\x90V[\x15a\x1C\xA2W[P\x86\x97\x98P\x81\x92a\x1Caa\x14\xB6\x8B\x89a\rkV[\x99a\x1Cl\x8B\x88a*wV[\x90\x84a\x1Cx\x88\x84a\x17)V[\x13a\x1C\x96WPP\x89\x93[\x88a\x1C\x8D\x89\x87a\r\xCFV[\x92\x01\x94\x99a\x1C6V[\x8B\x98P\x90\x95P\x93a\x1C\x82V[`\x14\x10\x80a\x1C\xB6W[\x15a\x15\x11W\x88a\x1CMV[P\x80\x83\x10a\x1C\xABV[\x93P\x94a\x1C\xCB\x90a\x1E\rV[\x94a\x1C\xD8\x84\x87\x84\x84a\x1E\xC1V[\x93a\x1B\xCCV[\x85\x85\x13a\x1C\xF2W\x90a\x143a\x1B\xED\x92a\x14!V[\x93P\x91a\x1C\xFE\x90a\x1F\xE6V[\x91a\x1D\x0B\x84\x84\x84\x84a\x1E\xC1V[\x93a\x1C\xDEV[a\x1D\x81a\x1D|a\x02j\x93a\x1Dva\x1Dq\x82Qa\x1Dlg\r\xE0\xB6\xB3\xA7d\0\0a\x1Dfa\x1Daa\x1D[a\x1DV`@` \x8B\x01Q\x9A\x01Q\x96a\x1DP\x88\x8Ca eV[\x9Da \x10V[a&IV[\x97a&IV[a\x16\xD2V[\x05a\"pV[a\x1E,V[a\x1EoV[\x90a\x16\x9CV[a\x16\xB5V[a\x17LV[`\x01`\xFF\x1B\x81\x14a\rfW`\0\x03\x90V[a\x1D\xC6a\x1D|a\x1D\xC1g\x13\xA0K\xBD\xFD\xC9\xBE\x88a\x1D\xBBg\x1B\xC1mgN\xC8\0\0\x95a\x16\xB5V[\x05a\x1D\x86V[a$3V[\x05\x90V[`\0\x81\x12a\x1D\xD5W\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01RotoUint: negative`\x80\x1B`D\x82\x01R`d\x90\xFD[a\x03\xE7\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x03\xFFWa\x03\xE8\x90\x04\x90V[\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x03\xFFWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x04\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17\x82\x15\x15\x16\x15a\x03\xFFW\x04\x90V[g\x06\xF0[Y\xD3\xB2\0\0\x90\x80\x82\x02\x91\x82\x04\x14`\x01\x16\x15a\x03\xFFWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x04\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x03\xFFWg\x1B\xC1mgN\xC8\0\0\x90\x04\x90V[\x90\x92\x82\x82\x10\x15a\x1FuWa\x02j\x93a\x1F>\x92\x84g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82a\x1E\xE9\x83\x83a\x1EMV[\x10a\x1FbWP`\x01`\x01`\xFF\x1B\x03\x95\x90P[\x83Q\x91a\x1F\x11a\x1F\x0B\x83\x85a\x1E,V[\x85a\x1EMV[\x10a\x1FCWP`\x01`\x01`\xFF\x1B\x03\x92a\x1F8\x92P\x90P[`@` \x82\x01Q\x91\x01Q\x90a eV[\x92a\x19NV[a\x19NV[a\x1F8\x92a\x19\xA2a\x1FW\x92a\x1F\\\x94a\x1E,V[a \xADV[\x91a\x1F(V[a\x1Fo\x91a\x1FW\x91a\x1EMV[\x94a\x1E\xFBV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FtradingFunction: invalid x\0\0\0\0\0\0`D\x82\x01R`d\x90\xFD[\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x03\xFFW`\x01g\r\xE0\xB6\xB3\xA7d\0\0`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[a\x03\xE9\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x03\xFFW`\x01a\x03\xE8`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[\x90g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x81\x02\x92\x81\x84\x04\x14\x90\x15\x17\x81\x15\x15\x16\x15a\x03\xFFW`\x01\x90`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[a\x01\0\x81\x83\x03\x12a\x02\xC0W\x80Q\x92a\x02j` \x83\x01Q\x93```@\x85\x01Q\x94\x01a\x10\xCFV[\x90a o\x90a%SV[c;\x9A\xCA\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17\x15a\rfWa\x02j\x91a\x1E,V[a\x02j\x91a\x1Dlg\r\xE0\xB6\xB3\xA7d\0\0a\x1Dfa\x1Daa\x1Dq\x95a&IV[g\x06\xF0[Y\xD3\xB2\0\0\x81\x14a\"jWg\r\xE0\xB6\xB3\xA7d\0\0\x80\x82\x12\x15a\"\x14W\x81\x15a\"5W`\x01\x82`\x01\x1B\x91`\x02\x93\x83\x05`\x02\x03a\rfW`\0\x83\x12\x80\x15a\"YW[a\"GW\x82\x15a\"\x14Wg\x1B\xC1mgN\xC8\0\0\x83\x14a\"5W\x82\x12\x91\x82\x15a\"&W\x92[a!\x1E\x84a)\x01V[\x80\x15a\"\x14Wa!\x8Ba!Oa!Ja!Ea!@a!\x90\x95\x99\x97\x96\x99a&IV[a(iV[a%SV[a\x17\x10V[a\x18\x1Aa!ca!^\x83a),V[a\x19\x1EV[a!\x85a!\x80a!za!u\x86a)WV[a\x196V[\x85a(\xE0V[a\x18@V[\x90a)\xA0V[a(\x91V[\x93`\0\x92[\x81\x84\x10a!\xC7WPPPPa\x02j\x91a!\xB4\x91`\0\x14a!\xB9Wa(\x17V[a\x1D\x86V[a!\xC2\x90a\x1D\x86V[a(\x17V[\x90\x91a\"\n\x86a\"\x04a!\xDF\x85a\x18\x1A\x86\x99\x9Ba$3V[a!\x85a!\xFAa!\xF5a\x18\x1Fa!\xB4\x87\x80a(\xE0V[a(\xB9V[a\x1Dv\x83\x86a(\xE0V[\x90a\x19NV[\x95\x01\x92\x91\x90a!\x95V[`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x90\xFD[a\"/\x90a\x16yV[\x92a!\x15V[`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x90\xFD[`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x90\xFD[Pg\x1B\xC1mgN\xC8\0\0\x83\x13a \xF1V[P`\0\x90V[h\x02H\xCE6\xA7\x0C\xB2k>\x19\x81\x13\x15a\"jWh\x07U\xBFy\x8BJ\x1B\xF1\xE5\x81\x12\x15a#\xC0We\x03x-\xAC\xE9\xD9\x90`N\x1B\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x91``\x90`\x01`_\x1B\x84\x82\x84\x1B\x05\x01\x82\x1D\x93\x84\x02\x90\x03\x80l\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x81\x01\x02\x82\x1D\x90n\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dPn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x82n\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x81m\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x81m\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x81m\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x81l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x02\x8D\x1D\x01\x02\x8B\x1D\x01\x02\x89\x1D\x01\x02\x87\x1D\x01\x02\x85\x1D\x01\x93m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x93m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEA\x81\x01\x90\x84m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x91\x01\x01\x02\x90\x1D\x01\x02\x01\x05\x02\x90`\xC3\x03\x1C\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x90\xFD[a\x1D\x81a\x1D|a\x02j\x93a\"\x04a\x1Dq\x82Qa\x1Dlg\r\xE0\xB6\xB3\xA7d\0\0a\x1Dfa\x1Daa\x1D[a\x1DV`@` \x8B\x01Q\x9A\x01Q\x96a\x1DP\x88\x8Ca eV[\x80\x15a%FWgV\x98\xEE\xF0fp\0\0\x81\x12\x15a\"jWgV\x98\xEE\xF0fo\xFF\xFF\x19\x81\x13\x15a%9W`\0a%)a$h\x83a'\xEAV[a$\xF1a\x18\x1Fa$\x82a$}a!\x80\x85a\x1E\x96V[a)\x81V[\x92a\x1F>a%$a%\x1Fa%\x18a%\x12a%\ra%\x07a%\x02a$\xFCa$\xF7\x8Da$\xF1a$\xECa$\xE6a$\xE1a!za$\xDCa$\xD6a$\xD1a$\xCBa$\xC6\x8Aa(>V[a\x18XV[\x89a(\xE0V[a\x18rV[\x87a(\xE0V[a\x18\x8AV[a\x18\xA4V[\x83a(\xE0V[a\x18\xBCV[\x90a(\xE0V[a\x18\xD6V[\x8Ca(\xE0V[a\x18\xEEV[\x8Aa(\xE0V[a\x19\x06V[\x88a(\xE0V[\x93\x80a(\xE0V[a\x16\xEFV[a\x16`V[\x91\x12\x15a\x02jWa\x02j\x90a\x16yV[Pg\x1B\xC1mgN\xC8\0\0\x90V[Pg\r\xE0\xB6\xB3\xA7d\0\0\x90V[`\xB5\x81`\x01`\x88\x1B\x81\x10\x15a%\xFAW[\x80i\x01\0\0\0\0\0\0\0\0\0b\x01\0\0\x92\x10\x15a%\xEDW[e\x01\0\0\0\0\0\x81\x10\x15a%\xE0W[c\x01\0\0\0\x81\x10\x15a%\xD3W[\x01\x02`\x12\x1C`\x01\x90\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x80\x80\x92\x04\x10\x90\x03\x90V[`\x10\x1C\x91`\x08\x1B\x91a%\x97V[` \x1C\x91`\x10\x1B\x91a%\x8AV[`@\x1C\x91` \x1B\x91a%{V[Ph\xB5\0\0\0\0\0\0\0\0\x90P`\x80\x82\x90\x1Ca%cV[\x15a&\x18WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x90\xFD[}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x90a&u`\0\x82\x13a&\x11V[q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06a&\x91\x82a)\xC2V[``\x92\x83\x82`\x9F\x03\x01\x1B`\x9F\x1C\x90`_\x19\x01}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x02\x92l\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x82m\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x81m\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x81m\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x81m\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x81m\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x81lFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x02\x8C\x1D\x01\x02\x8A\x1D\x01\x02\x88\x1D\x01\x02\x86\x1D\x01\x02\x84\x1D\x01\x02\x82\x1D\x01\x91x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x91l\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x90\x82m\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x81l\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x81m\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x81m\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x81l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x81\x01\x02\x89\x1D\x01\x02\x87\x1D\x01\x02\x85\x1D\x01\x02\x83\x1D\x01\x02\x90\x1D\x01\x02\x01\x05\x02\x01\x01`\xAE\x1D\x90V[`\x01`\xFF\x1B\x81\x14a(\x05W`\0\x81\x12\x15a\x02jW\x19`\x01\x01\x90V[`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x90\xFD[g\x13\xA0K\xBD\xFD\xC9\xBE\x88\x90\x80\x82\x02\x91\x82\x05\x14`\x01\x16\x15a\x03\xFFWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\x02_\x0F\xE1\x05\xA3\x14\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x03\xFFWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\x1B\xC1mgN\xC7\xFF\xFF\x19\x81\x81\x02\x91`\x01\x91\x83\x05\x14\x16\x15a\x03\xFFWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\t\xD0(\xCCo _\xFF\x19\x81\x81\x02\x91`\x01\x91\x83\x05\x14\x16\x15a\x03\xFFWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x90\x80\x82\x02\x91\x82\x05\x14`\x01\x16\x15a\x03\xFFWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x03\xFFWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x03\xFFWg\x1B\xC1mgN\xC8\0\0\x90\x05\x90V[g\x03\xC1f\\z\xAB \0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x03\xFFWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[f\x9F2u$b\xA0\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x03\xFFWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[`\x01\x81\x15\x15\x16\x15a\x03\xFFWn\xC0\x97\xCE{\xC9\x07\x15\xB3K\x9F\x10\0\0\0\0\x05\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17\x82\x15\x15\x16\x15a\x03\xFFW\x05\x90V[a)\xCD\x81\x15\x15a&\x11V[\x80o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x07\x1B\x81\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x81\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x81\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x81\x81\x1C`\xFF\x10`\x03\x1B\x17\x81\x81\x1C`\x0F\x10`\x02\x1B\x17\x81\x81\x1C`\x03\x10`\x01\x1B\x17\x90\x81\x1C`\x01\x10\x17\x90V[\x90a*Ka\x02j\x92` \x80\x82Q\x83\x01\x01\x91\x01a @V[\x93\x91\x92\x90Pa\x1E\xC1V[\x90a*la\x02j\x92` \x80\x82Q\x83\x01\x01\x91\x01a @V[\x93\x90P\x91\x90\x91a\x1E\xC1V[\x90a*\x8Ea\x02j\x92` \x80\x82Q\x83\x01\x01\x91\x01a @V[\x93\x92\x90Pa\x1E\xC1V\xFE\xA2dipfsX\"\x12 \\\xB4]\xDA\xE4\xE6\xE8!\xB5[\xD5\xEE,A\xAE6x\x1B\xB8u\xFE\xBCI\x82\x91\x99\xEE\xD4\xC3\xD17\x0BdsolcC\0\x08\x16\x003";
    /// The bytecode of the contract.
    pub static LOGNORMALSOLVER_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\x0B3W`\x005`\xE0\x1C\x80c\x04 X\n\x14a\x01lW\x80c\x12\x06I\xC5\x14a\x01gW\x80c\x13N\xAD\x12\x14a\x01bW\x80c\x1E\x97\x8C\xB0\x14a\x01]W\x80c9(\xFF\x97\x14a\x01XW\x80c;&\x8D]\x14a\x01SW\x80c;M\x100\x14a\x01NW\x80cN\x81\x7F\xD9\x14a\x01IW\x80c^\xB4\x08\xFC\x14a\x01DW\x80cb7V\x9F\x14a\x01?W\x80cme\"\x99\x14a\x01:W\x80c\x7F\x17@\x9C\x14a\x015W\x80c\x81\xB5\xFA\xC2\x14a\x010W\x80c\xA8\xC6.v\x14a\x01+W\x80c\xAFNC\x7F\x14a\x01&W\x80c\xB0\x9D\x04\xE5\x14a\x01!W\x80c\xCB\x1FU2\x14a\x01\x1CW\x80c\xCE\x15;\xF4\x14a\x01\x17W\x80c\xE9G\x16\xD5\x14a\x01\x12W\x80c\xEE>\x8C\xFB\x14a\x01\rW\x80c\xF3\r7\xF2\x14a\x01\x08Wc\xF9\xC2\x82\x11\x03a\x0B3Wa\x0B\x17V[a\n\xE7V[a\n\xB6V[a\n{V[a\n?V[a\t\xFAV[a\t\xC7V[a\t\xABV[a\t\x82V[a\tUV[a\x08\xB3V[a\x08\x97V[a\x08*V[a\x08\x0EV[a\x07\xF2V[a\x07\xC3V[a\x07\x88V[a\x04\xEBV[a\x04\x97V[a\x04\x04V[a\x02\xE8V[a\x02mV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01Ra7\xB7`\xF1\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[`\0[\x83\x81\x10a\x02$WPP`\0\x91\x01RV[\x81\x81\x01Q\x83\x82\x01R` \x01a\x02\x14V[\x90` \x91a\x02M\x81Q\x80\x92\x81\x85R\x85\x80\x86\x01\x91\x01a\x02\x11V[`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[\x90` a\x02j\x92\x81\x81R\x01\x90a\x024V[\x90V[4a\x02\xC5W`@6`\x03\x19\x01\x12a\x02\xC0Wa\x02\xBC`@Q`\x02` \x82\x01R`\x045`@\x82\x01R`$5``\x82\x01R``\x81Ra\x02\xA8\x81a\x03\x94V[`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x024V[\x03\x90\xF3[a\x01\xC1V[a\x01qV[`\x80\x90`\x03\x19\x01\x12a\x02\xC0W`\x045\x90`$5\x90`D5\x90`d5\x90V[4a\x02\xC5W` a\x03\x04a\x02\xFB6a\x02\xCAV[\x92\x91\x90\x91a\x0C\x1BV[`@Q\x90\x81R\xF3[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FABI decoding: struct data too sh`D\x82\x01Rb\x1B\xDC\x9D`\xEA\x1B`d\x82\x01R`\x84\x90\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\xA0\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x03\x8FW`@RV[a\x03]V[`\x80\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x03\x8FW`@RV[``\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x03\x8FW`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x03\x8FW`@RV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\x03\xFFWV[`\0\x80\xFD[4a\x02\xC5W`\xE06`\x03\x19\x01\x12a\x02\xC0W`\xA06`C\x19\x01\x12a\x04xWa\x02\xBCa\x04l`@Qa\x043\x81a\x03sV[`D5\x81R`d5` \x82\x01R`\x845`@\x82\x01R`\xA45``\x82\x01R`\xC45a\x04\\\x81a\x03\xEEV[`\x80\x82\x01R`$5`\x045a\x15\xB7V[`@Q\x91\x82\x91\x82a\x02YV[a\x03\x0CV[``\x90`\x03\x19\x01\x12a\x02\xC0W`\x045\x90`$5\x90`D5\x90V[4a\x02\xC5W` a\x03\x04a\x04\xB6a\x04\xAD6a\x04}V[\x91\x92\x90\x92a\x112V[\x91a\x17}V[\x80\x15\x15\x03a\x03\xFFWV[\x90\x92`\x80\x92a\x02j\x95\x94\x15\x15\x83R` \x83\x01R`@\x82\x01R\x81``\x82\x01R\x01\x90a\x024V[4a\x02\xC5W``6`\x03\x19\x01\x12a\x02\xC0W`$5`\x045a\x05\x0B\x82a\x04\xBCV[`D5\x90a\x05^a\x05\x1Aa\r\x12V[\x92a\x05#a\r\x12V[\x93a\x05-\x84a\x12\xB9V[` \x84\x99\x93\x95\x92\x99\x01\x94`@\x99\x8A\x86\x01\x92\x83R\x86R\x84Ra\x05M\x87a\x112V[\x95\x86\x91\x85Q\x90\x87Q\x90Q\x91\x8Aa\x11\xE0V[\x92\x15a\x07\0W\x92\x82a\x05\xA5a\x05\xAC\x93a\x05\x9Ea\x05\x99a\x05\x91a\x05\xCA\x98a\x05\x8C``a\x05\xF2\x9D\x9C\x01Q\x86a\x1F\xBAV[a\x1F\xBAV[\x86Q\x90a \x10V[a\rXV[\x93Qa\rkV[\x89Ra\rkV[a\x05\xBE\x88\x88\x01\x91\x80\x83R\x88Q\x88a\r\x05V[\x90\x87Q\x90Q\x90\x87a\x0C\x1BV[\x90a\x05\xE9a\x05\xDE` \x88\x01\x93\x80\x85Ra\rXV[\x80\x84R\x82Q\x11a\r\xDCV[Q\x90Q\x90a\r\xCFV[\x93[\x83Q\x91` \x85\x01Q\x92a\x066\x83\x87\x01\x91a\x06(\x83Q\x86Q\x97\x88\x93` \x85\x01`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[\x03`\x1F\x19\x81\x01\x86R\x85a\x03\xCCV[`\0Ta\x06Y\x90a\x06M\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x80;\x15a\x06\xFBW\x84`\xC0\x91a\x06\x84\x97\x86Q\x80\x99\x81\x94\x82\x93c\r\x17\xA7\xC7`\xE3\x1B\x84R0`\x04\x85\x01a\x0EjV[\x03\x91Z\xFA\x94\x85\x15a\x06\xF6W`\0\x95a\x06\xB6W[P\x90a\x06\xAB\x91a\x02\xBC\x95\x96Q\x90Q\x90a\x17}V[\x90Q\x94\x85\x94\x85a\x04\xC6V[a\x02\xBC\x95P\x90a\x06\xE1a\x06\xAB\x93\x92`\xC0=`\xC0\x11a\x06\xEFW[a\x06\xD9\x81\x83a\x03\xCCV[\x81\x01\x90a\x0E3V[PPPPP\x95P\x90\x91a\x06\x97V[P=a\x06\xCFV[a\x0C\x0FV[a\x0B\x96V[\x82a\x07Ia\x07\x82\x96a\x07<a\x07n\x95a\x075a\x05\x99a\x07-a\x07y\x9Aa\x05\x8C``a\x07f\x9B\x01Q\x86a\x1F\xBAV[\x85Q\x90a \x10V[\x92Qa\rkV[\x92` \x8C\x01\x93\x84Ra\rkV[a\x07[\x8B\x8B\x01\x91\x80\x83R\x83Q\x8Ba\x0E\x8EV[\x91Q\x90Q\x90\x89a\x0E\x9BV[\x80\x88Ra\rXV[\x80\x87R\x82Q\x11a\rxV[Q\x84Q\x90a\r\xCFV[\x93a\x05\xF4V[4a\x02\xC5W`@6`\x03\x19\x01\x12a\x02\xC0Wa\x02\xBC`@Q`\x04` \x82\x01R`\x045`@\x82\x01R`$5``\x82\x01R``\x81Ra\x02\xA8\x81a\x03\x94V[4a\x02\xC5W` 6`\x03\x19\x01\x12a\x02\xC0W` a\x03\x04`\x045a\x04\xB6a\x07\xE8\x82a\x12\xB9V[\x92\x91\x93\x90Pa\x112V[4a\x02\xC5W` a\x03\x04a\x08\x08a\x04\xAD6a\x04}V[\x91a\x19jV[4a\x02\xC5W` a\x03\x04a\x08!6a\x02\xCAV[\x92\x91\x90\x91a\x0E\x9BV[4a\x02\xC5W`@6`\x03\x19\x01\x12a\x02\xC0Wa\x08y`\x045a\x02\xBCa\x08[a\x08P\x83a\x12\xB9V[\x91\x90P`$5a\x1BgV[\x93\x90\x92\x84\x84a\x08sa\x08l\x84a\x112V[\x83\x83a\x17}V[\x92a\x0C\x1BV[\x92`@Q\x93\x84\x93\x84`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[4a\x02\xC5W`\x006`\x03\x19\x01\x12a\x02\xC0W` `@Q`\0\x81R\xF3[4a\x02\xC5W`@6`\x03\x19\x01\x12a\x02\xC0W`\x045a\t\x01a\x02\xBCa\x08\xE3a\x08\xD9\x84a\x12\xB9V[\x91P`$5a\x1B\x94V[\x92\x90\x93\x83\x85a\x08\xFBa\x08\xF4\x84a\x112V[\x83\x83a\x19jV[\x92a\x0E\x9BV[\x91`@Q\x93\x84\x93\x84`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[\x80Q\x82R` \x80\x82\x01Q\x90\x83\x01R`@\x80\x82\x01Q\x90\x83\x01R``\x80\x82\x01Q\x90\x83\x01R`\x80\x90\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x91\x01RV[4a\x02\xC5W` 6`\x03\x19\x01\x12a\x02\xC0W`\xA0a\ts`\x045a\x112V[a\t\x80`@Q\x80\x92a\t\x1FV[\xF3[4a\x02\xC5W`\x006`\x03\x19\x01\x12a\x02\xC0W`\0T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x02\xC5W` a\x03\x04a\t\xBE6a\x02\xCAV[\x92\x91\x90\x91a\x11\xE0V[4a\x02\xC5W` 6`\x03\x19\x01\x12a\x02\xC0Wa\x02\xBC`@Q`\x01` \x82\x01R`\x045`@\x82\x01R`@\x81Ra\x02\xA8\x81a\x03\xB0V[4a\x02\xC5W` 6`\x03\x19\x01\x12a\x02\xC0Wa\x02\xBC`\x045a\n\x1A\x81a\x03\xEEV[`@\x80Q`\x05` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x82\x82\x01R\x81Ra\x02\xA8\x81a\x03\xB0V[4a\x02\xC5W` 6`\x03\x19\x01\x12a\x02\xC0Wa\x02\xBCa\n^`\x045a\x12\xB9V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R\x90\x81\x90``\x82\x01\x90V[4a\x02\xC5W`@6`\x03\x19\x01\x12a\x02\xC0Wa\x02\xBC`@Q`\x03` \x82\x01R`\x045`@\x82\x01R`$5``\x82\x01R``\x81Ra\x02\xA8\x81a\x03\x94V[4a\x02\xC5W`@6`\x03\x19\x01\x12a\x02\xC0Wa\x08y`\x045a\x02\xBCa\x08[a\n\xDC\x83a\x12\xB9V[\x91\x90P`$5a\x1B\x94V[4a\x02\xC5W`@6`\x03\x19\x01\x12a\x02\xC0W`\x045a\t\x01a\x02\xBCa\x08\xE3a\x0B\r\x84a\x12\xB9V[\x91P`$5a\x1BgV[4a\x02\xC5W`\x006`\x03\x19\x01\x12a\x02\xC0W` `@Q`x\x81R\xF3[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01Rtnor receive functions`X\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01Rd code`\xD8\x1B`d\x82\x01R`\x84\x90\xFD[\x90\x81` \x91\x03\x12a\x02\xC0WQ\x90V[`@\x90a\x02j\x93\x92\x81R\x81` \x82\x01R\x01\x90a\x024V[`@Q=`\0\x82>=\x90\xFD[\x90\x91\x92a\x0C2\x90a\x0C+\x83a\x112V[\x90\x85a\x13\xB2V[\x90`@Q\x93a\x0Ci\x85a\x0C[\x83\x86\x88` \x85\x01`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[\x03`\x1F\x19\x81\x01\x87R\x86a\x03\xCCV[`\0Ta\x0C\x80\x90a\x06M\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x80;\x15a\x06\xFBWa\x0C\xA9\x95` \x91`@Q\x80\x80\x99\x81\x94b.RK`\xE0\x1B\x83R\x88`\x04\x84\x01a\x0B\xF8V[\x03\x91Z\xFA\x91\x82\x15a\x06\xF6Wa\x02j\x95`\0\x93a\x0C\xD0W[Pa\x0C\xCA\x90a\x112V[\x93a\x14\0V[a\x0C\xCA\x91\x93Pa\x0C\xF7\x90` =` \x11a\x0C\xFEW[a\x0C\xEF\x81\x83a\x03\xCCV[\x81\x01\x90a\x0B\xE9V[\x92\x90a\x0C\xC0V[P=a\x0C\xE5V[\x91a\x04\xB6a\x02j\x93a\x112V[`@Q\x90``\x82\x01\x82\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x03\x8FW`@R`\0`@\x83\x82\x81R\x82` \x82\x01R\x01RV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x90`\x01\x82\x01\x80\x92\x11a\rfWV[a\rBV[\x91\x90\x82\x01\x80\x92\x11a\rfWV[\x15a\r\x7FWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: x reserve increase`D\x82\x01Rad!`\xF0\x1B`d\x82\x01R`\x84\x90\xFD[\x91\x90\x82\x03\x91\x82\x11a\rfWV[\x15a\r\xE3WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: y reserve increase`D\x82\x01Rad!`\xF0\x1B`d\x82\x01R`\x84\x90\xFD[\x91\x90\x82`\xC0\x91\x03\x12a\x02\xC0W\x81Qa\x0EJ\x81a\x04\xBCV[\x91` \x81\x01Q\x91`@\x82\x01Q\x91``\x81\x01Q\x91`\xA0`\x80\x83\x01Q\x92\x01Q\x90V[a\x02j\x93\x92``\x92`\x01\x80`\xA0\x1B\x03\x16\x82R` \x82\x01R\x81`@\x82\x01R\x01\x90a\x024V[\x91a\x08\x08a\x02j\x93a\x112V[\x90\x91\x92a\x0E\xB2\x90a\x0E\xAB\x83a\x112V[\x90\x85a\x19\xE4V[\x90`@Q\x93a\x0E\xDB\x85a\x0C[\x83\x87\x87` \x85\x01`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[`\0Ta\x0E\xF2\x90a\x06M\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x80;\x15a\x06\xFBWa\x0F\x1B\x95` \x91`@Q\x80\x80\x99\x81\x94b.RK`\xE0\x1B\x83R\x88`\x04\x84\x01a\x0B\xF8V[\x03\x91Z\xFA\x91\x82\x15a\x06\xF6Wa\x02j\x95`\0\x93a\x0FBW[Pa\x0F<\x90a\x112V[\x93a\x1A\x11V[a\x0F<\x91\x93Pa\x0F`\x90` =` \x11a\x0C\xFEWa\x0C\xEF\x81\x83a\x03\xCCV[\x92\x90a\x0F2V[\x91\x90` \x92\x83\x81\x83\x03\x12a\x02\xC0W\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x82\x82\x11a\x10\x7FW\x01\x82`\x1F\x82\x01\x12\x15a\x10&W\x80Q\x91\x82\x11a\x03\x8FW`@Q\x92a\x0F\xB4`\x1F\x84\x01`\x1F\x19\x16\x87\x01\x85a\x03\xCCV[\x82\x84R\x85\x83\x83\x01\x01\x11a\x0F\xD1W\x84\x83\x94\x95a\x02j\x94\x01\x91\x01a\x02\x11V[`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x86\x90R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x86\x90R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x87\x90R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[\x91\x90\x82`\xA0\x91\x03\x12a\x04xW`@Qa\x10\xE7\x81a\x03sV[`\x80\x80\x82\x94\x80Q\x84R` \x81\x01Q` \x85\x01R`@\x81\x01Q`@\x85\x01R``\x81\x01Q``\x85\x01R\x01Q\x91a\x11\x1A\x83a\x03\xEEV[\x01RV[\x90`\xA0\x82\x82\x03\x12a\x02\xC0Wa\x02j\x91a\x10\xCFV[`\x80\x90`@Qa\x11A\x81a\x03sV[`\0\x92\x81\x84\x80\x93R\x82` \x82\x01R\x82`@\x82\x01R\x82``\x82\x01R\x01Ra\x11sa\x06Ma\x06M\x84T`\x01\x80`\xA0\x1B\x03\x16\x90V[\x80;\x15a\x06\xFBW`@Qc\xDC\x17\x83U`\xE0\x1B\x81R`\x04\x81\x01\x92\x90\x92R\x82\x90\x82\x90`$\x90\x82\x90Z\xFA\x90\x81\x15a\x06\xF6W\x82a\x02j\x93\x92a\x11\xBDW[PP` \x80\x82Q\x83\x01\x01\x91\x01a\x11\x1EV[a\x11\xD9\x92P=\x80\x91\x83>a\x11\xD1\x81\x83a\x03\xCCV[\x81\x01\x90a\x0FgV[8\x80a\x11\xACV[`@\x80Q` \x81\x01\x84\x90R\x90\x81\x01\x84\x90R``\x81\x01\x85\x90R\x93\x91\x92a\x12\x08\x85`\x80\x81\x01a\x0C[V[`\0T`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x06\xFBWa\x12=\x95` \x91`@Q\x80\x80\x99\x81\x94b.RK`\xE0\x1B\x83R\x88`\x04\x84\x01a\x0B\xF8V[\x03\x91Z\xFA\x91\x82\x15a\x06\xF6Wa\x02j\x95`\0\x93a\x12dW[Pa\x12^\x90a\x112V[\x93a\x1B\xBBV[a\x12^\x91\x93Pa\x12\x82\x90` =` \x11a\x0C\xFEWa\x0C\xEF\x81\x83a\x03\xCCV[\x92\x90a\x12TV[\x90\x81` \x91\x03\x12a\x02\xC0WQa\x02j\x81a\x03\xEEV[\x90\x81``\x91\x03\x12a\x02\xC0W\x80Q\x91`@` \x83\x01Q\x92\x01Q\x90V[`\0T\x90\x91\x90a\x12\xD3\x90a\x06M\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x80;\x15a\x06\xFBW` `\x04\x91`@Q\x92\x83\x80\x92c+\xEE\x84\xF1`\xE2\x1B\x82RZ\xFA\x90\x81\x15a\x06\xF6W`\0\x91a\x13\x83W[P`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x06\xFBW`@Qc3\x85N\xFD`\xE2\x1B\x81R`\x04\x81\x01\x93\x90\x93R``\x90\x83\x90`$\x90\x82\x90Z\xFA\x90\x81\x15a\x06\xF6W`\0\x80\x93`\0\x93a\x13LW[P\x92\x91\x90V[\x91\x93PPa\x13r\x91P``=``\x11a\x13|W[a\x13j\x81\x83a\x03\xCCV[\x81\x01\x90a\x12\x9EV[\x92\x90\x92\x918a\x13FV[P=a\x13`V[a\x13\xA5\x91P` =` \x11a\x13\xABW[a\x13\x9D\x81\x83a\x03\xCCV[\x81\x01\x90a\x12\x89V[8a\x13\x01V[P=a\x13\x93V[a\x05\x8C\x90a\x13\xD4a\x13\xCFa\x13\xCA\x86a\x02j\x97\x96a\x1D\x11V[a\x1D\x97V[a\x1D\xCAV[\x92Qa\x1F\xBAV[a\x13\xFE\x93``\x92\x96\x95\x93a\x01\0\x83\x01\x97\x83R` \x83\x01R`@\x82\x01R\x01\x90a\t\x1FV[V[\x92\x93`\0\x93\x85\x92\x91\x85\x85\x12\x15a\x15\x84W[\x85\x85\x12a\x15eW\x90a\x143a\x14A\x92[`@\x96`@Q\x95\x86\x94` \x86\x01a\x13\xDBV[\x03`\x1F\x19\x81\x01\x83R\x82a\x03\xCCV[\x81\x85\x92\x85\x96\x82\x81\x11a\x15BWa\x14W\x81\x85a*4V[\x92a\x14b\x81\x86a*4V[\x88a\x14m\x82\x87a\x17)V[\x13a\x15!WP\x90a\x14\x81\x91\x97\x96\x92\x97a\r\xCFV[`\x01\x95\x91\x82\x91\x87\x80[a\x14\x9CW[PPPPPPPPPP\x90V[\x15a\x14\xFDW[P\x86\x97\x98P\x81\x92a\x14\xBCa\x14\xB6\x8B\x89a\rkV[`\x01\x1C\x90V[\x99a\x14\xC7\x8B\x88a*4V[\x90\x84a\x14\xD3\x88\x84a\x17)V[\x13a\x14\xF1WPP\x89\x93[\x88a\x14\xE8\x89\x87a\r\xCFV[\x92\x01\x94\x99a\x14\x8AV[\x8B\x98P\x90\x95P\x93a\x14\xDDV[`\x14\x10\x80a\x15\x18W[\x15a\x15\x11W\x88a\x14\xA2V[\x80\x80a\x14\x8FV[P\x80\x83\x10a\x15\x06V[`@Qc\x06\xF1\xBE]`\xE2\x1B\x81R`\x04\x81\x01\x86\x90R`$\x81\x01\x91\x90\x91R`D\x90\xFD[`@Qc0\x82\xDF\xDB`\xE1\x1B\x81R`\x04\x81\x01\x91\x90\x91R`$\x81\x01\x92\x90\x92RP`D\x90\xFD[\x93P\x91a\x15q\x90a\x1F\xE6V[\x91a\x15~\x84\x83\x85\x84a\x1E\xC1V[\x93a\x14\x11V[\x85\x85\x13a\x15\x98W\x90a\x143a\x14A\x92a\x14!V[\x93P\x94a\x15\xA4\x90a\x1E\rV[\x94a\x15\xB1\x84\x83\x88\x84a\x1E\xC1V[\x93a\x15\x84V[\x91a\x15\xC8a\x13\xCFa\x13\xCA\x83\x85a#\xF4V[\x91g\r\xE0\xB6\xB3\xA7d\0\0\x92\x83\x03\x92\x83\x11a\rfWa\x16\x1F\x82a\x16\x0Ba\x16\0a\x13\xCFa\x13\xCA\x84a\x15\xFAa\x16=\x9A\x8Ca \x10V[\x97a\x1D\x11V[a\x05\x8C\x85\x84Qa\x1F\xBAV[\x92a\x16\x18\x82\x82\x86\x8Aa\x1E\xC1V[\x84\x88a\x1B\xBBV[\x90`@Q\x94` \x86\x01R`@\x85\x01R``\x84\x01R`\x80\x83\x01\x90a\t\x1FV[a\x01\0\x81Ra\x01 \x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x03\x8FW`@R\x90V[\x90g\x11\x90\0\xAB\x10\x0F\xFB\xFF\x19\x82\x01\x91\x82\x13`\x01\x16a\rfWV[\x90g\x1B\xC1mgN\xC8\0\0`\0\x83\x82\x03\x93\x12\x81\x84\x12\x81\x16\x91\x84\x13\x90\x15\x16\x17a\rfWV[\x81\x81\x03\x92\x91`\0\x13\x80\x15\x82\x85\x13\x16\x91\x84\x12\x16\x17a\rfWV[\x90g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x81\x02\x92\x81\x84\x05\x14\x90\x15\x17\x15a\rfWV[\x90g\x1B\xC1mgN\xC8\0\0\x91\x82\x81\x02\x92\x81\x84\x05\x14\x90\x15\x17\x15a\rfWV[`\0\x81\x90\x03\x91\x90`\x01`\xFF\x1B\x81\x14`\x01\x16a\rfW`\0\x19\x83\x05\x03a\rfWV[\x90c;\x9A\xCA\0\x91\x82\x81\x02\x92\x81\x84\x05\x14\x90\x15\x17\x15a\rfWV[\x81\x81\x02\x92\x91`\0\x82\x12`\x01`\xFF\x1B\x82\x14\x16a\rfW\x81\x84\x05\x14\x90\x15\x17\x15a\rfWV[\x81\x15a\x17gW`\x01`\xFF\x1B\x81\x14`\0\x19\x83\x14\x16a\rfW\x05\x90V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[\x91\x90\x91a\x17\xB8` \x83\x01\x93a\x17\xB2\x85Qa\x17\xAAa\x17\xA0`@\x88\x01\x92\x83Q\x90a eV[\x97Q\x82Q\x90a \x8EV[\x90Q\x90a\x1E,V[\x92a\x1EMV[\x92g\r\xE0\xB6\xB3\xA7d\0\0\x80\x85\x12\x15a\x186W`\0\x85\x13\x15a\x18+W`\0\x85\x82\x03\x95\x12\x81\x86\x12\x81\x16\x91\x86\x13\x90\x15\x16\x17a\rfWa\x18\x1Fa\x18$\x92a\x18\x1Aa\x18\x0Ca\x13\xCF\x94a\x18\x07a\x02j\x99a \xADV[a\x17)V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[a\x16\x9CV[a\"pV[\x90Qa\x1F\xBAV[PPPPP`\0\x19\x90V[PPPPP`\0\x90V[\x90\x81g\r\xE0\xB6\xB3\xA7d\0\0\x01\x91\x82\x12`\x01\x16a\rfWV[\x90\x81g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19\x01\x91\x82\x12\x15`\x01\x16a\rfWV[\x90\x81g\x14\xA8EL\x19\xE1\xAC\0\x01\x91\x82\x12`\x01\x16a\rfWV[\x90\x81g\x0F\xC1\x0E\x01W\x82w\xFF\x19\x01\x91\x82\x12\x15`\x01\x16a\rfWV[\x90\x81g\x03\xDE\xBD\x08;\x8C|\0\x01\x91\x82\x12`\x01\x16a\rfWV[\x90\x81g\x02\x95\xD4\0\xEA2W\xFF\x19\x01\x91\x82\x12\x15`\x01\x16a\rfWV[\x90\x81g\x01W\xD8\xB2\xEC\xC7\x08\0\x01\x91\x82\x12`\x01\x16a\rfWV[\x90\x81g\x051\n\xA7\xD5!0\0\x01\x91\x82\x12`\x01\x16a\rfWV[\x90\x81g\r\xE0\xCC=\x15a\0\0\x01\x91\x82\x12`\x01\x16a\rfWV[\x90\x81g \x05\xFEO&\x8E\xA0\0\x01\x91\x82\x12`\x01\x16a\rfWV[\x90\x81g\r\xC5R\x7Fd, \0\x01\x91\x82\x12`\x01\x16a\rfWV[\x91\x90\x91`\0\x83\x82\x01\x93\x84\x12\x91\x12\x90\x80\x15\x82\x16\x91\x15\x16\x17a\rfWV[\x91\x90\x91a\x19\xA8` \x83\x01\x91a\x19\xA2a\x19\x9A\x84Qa\x17\xAAa\x19\x90`@\x89\x01\x92\x83Q\x90a eV[\x96Q\x82Q\x90a \x8EV[\x95\x85Qa\x1E,V[\x90a\x1EMV[g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x82\x12\x15a\x186W`\0\x82\x13\x15a\x18+Wa\x02j\x94a\x18$\x93a\x19\xDEa\x18\x1F\x93a\x18\x07a\x13\xCF\x96a \xADV[\x05a\x19NV[\x91a\x13\xCAa\x13\xCF\x91a\x19\xF5\x93a#\xF4V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x03\x90\x81\x11a\rfWa\x02j\x91a\x1F\xBAV[\x92\x93`\0\x93\x85\x92\x91\x85\x85\x12\x15a\x1B4W[\x85\x85\x12a\x1B\x15W\x90a\x143a\x1AC\x92`@\x96`@Q\x95\x86\x94` \x86\x01a\x13\xDBV[\x81\x85\x92\x85\x96\x82\x81\x11a\x15BWa\x1AY\x81\x85a*UV[\x92a\x1Ad\x81\x86a*UV[\x88a\x1Ao\x82\x87a\x17)V[\x13a\x15!WP\x90a\x1A\x83\x91\x97\x96\x92\x97a\r\xCFV[`\x01\x95\x91\x82\x91\x87\x80[a\x1A\x9DWPPPPPPPPPP\x90V[\x15a\x1A\xF8W[P\x86\x97\x98P\x81\x92a\x1A\xB7a\x14\xB6\x8B\x89a\rkV[\x99a\x1A\xC2\x8B\x88a*UV[\x90\x84a\x1A\xCE\x88\x84a\x17)V[\x13a\x1A\xECWPP\x89\x93[\x88a\x1A\xE3\x89\x87a\r\xCFV[\x92\x01\x94\x99a\x1A\x8CV[\x8B\x98P\x90\x95P\x93a\x1A\xD8V[`\x14\x10\x80a\x1B\x0CW[\x15a\x15\x11W\x88a\x1A\xA3V[P\x80\x83\x10a\x1B\x01V[\x93P\x91a\x1B!\x90a\x1F\xE6V[\x91a\x1B.\x84\x83\x83\x86a\x1E\xC1V[\x93a\x1A\"V[\x85\x85\x13a\x1BHW\x90a\x143a\x1AC\x92a\x14!V[\x93P\x94a\x1BT\x90a\x1E\rV[\x94a\x1Ba\x84\x83\x83\x89a\x1E\xC1V[\x93a\x1B4V[\x92\x91\x90a\x1B}a\x1Bw\x82\x84a \x10V[\x85a\x1F\xBAV[\x93\x81\x03\x90\x81\x11a\rfW\x92\x81\x03\x90\x81\x11a\rfW\x90V[\x92\x91\x90a\x1B\xA4a\x1Bw\x82\x84a \x10V[\x93\x81\x01\x80\x91\x11a\rfW\x92\x81\x01\x80\x91\x11a\rfW\x90V[\x92\x93`\0\x93\x85\x92\x91\x85\x85\x12\x15a\x1C\xDEW[\x85\x85\x12a\x1C\xBFW\x90a\x143a\x1B\xED\x92`@\x96`@Q\x95\x86\x94` \x86\x01a\x13\xDBV[\x81\x85\x92\x85\x96\x82\x81\x11a\x15BWa\x1C\x03\x81\x85a*wV[\x92a\x1C\x0E\x81\x86a*wV[\x88a\x1C\x19\x82\x87a\x17)V[\x13a\x15!WP\x90a\x1C-\x91\x97\x96\x92\x97a\r\xCFV[`\x01\x95\x91\x82\x91\x87\x80[a\x1CGWPPPPPPPPPP\x90V[\x15a\x1C\xA2W[P\x86\x97\x98P\x81\x92a\x1Caa\x14\xB6\x8B\x89a\rkV[\x99a\x1Cl\x8B\x88a*wV[\x90\x84a\x1Cx\x88\x84a\x17)V[\x13a\x1C\x96WPP\x89\x93[\x88a\x1C\x8D\x89\x87a\r\xCFV[\x92\x01\x94\x99a\x1C6V[\x8B\x98P\x90\x95P\x93a\x1C\x82V[`\x14\x10\x80a\x1C\xB6W[\x15a\x15\x11W\x88a\x1CMV[P\x80\x83\x10a\x1C\xABV[\x93P\x94a\x1C\xCB\x90a\x1E\rV[\x94a\x1C\xD8\x84\x87\x84\x84a\x1E\xC1V[\x93a\x1B\xCCV[\x85\x85\x13a\x1C\xF2W\x90a\x143a\x1B\xED\x92a\x14!V[\x93P\x91a\x1C\xFE\x90a\x1F\xE6V[\x91a\x1D\x0B\x84\x84\x84\x84a\x1E\xC1V[\x93a\x1C\xDEV[a\x1D\x81a\x1D|a\x02j\x93a\x1Dva\x1Dq\x82Qa\x1Dlg\r\xE0\xB6\xB3\xA7d\0\0a\x1Dfa\x1Daa\x1D[a\x1DV`@` \x8B\x01Q\x9A\x01Q\x96a\x1DP\x88\x8Ca eV[\x9Da \x10V[a&IV[\x97a&IV[a\x16\xD2V[\x05a\"pV[a\x1E,V[a\x1EoV[\x90a\x16\x9CV[a\x16\xB5V[a\x17LV[`\x01`\xFF\x1B\x81\x14a\rfW`\0\x03\x90V[a\x1D\xC6a\x1D|a\x1D\xC1g\x13\xA0K\xBD\xFD\xC9\xBE\x88a\x1D\xBBg\x1B\xC1mgN\xC8\0\0\x95a\x16\xB5V[\x05a\x1D\x86V[a$3V[\x05\x90V[`\0\x81\x12a\x1D\xD5W\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01RotoUint: negative`\x80\x1B`D\x82\x01R`d\x90\xFD[a\x03\xE7\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x03\xFFWa\x03\xE8\x90\x04\x90V[\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x03\xFFWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x04\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17\x82\x15\x15\x16\x15a\x03\xFFW\x04\x90V[g\x06\xF0[Y\xD3\xB2\0\0\x90\x80\x82\x02\x91\x82\x04\x14`\x01\x16\x15a\x03\xFFWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x04\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x03\xFFWg\x1B\xC1mgN\xC8\0\0\x90\x04\x90V[\x90\x92\x82\x82\x10\x15a\x1FuWa\x02j\x93a\x1F>\x92\x84g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82a\x1E\xE9\x83\x83a\x1EMV[\x10a\x1FbWP`\x01`\x01`\xFF\x1B\x03\x95\x90P[\x83Q\x91a\x1F\x11a\x1F\x0B\x83\x85a\x1E,V[\x85a\x1EMV[\x10a\x1FCWP`\x01`\x01`\xFF\x1B\x03\x92a\x1F8\x92P\x90P[`@` \x82\x01Q\x91\x01Q\x90a eV[\x92a\x19NV[a\x19NV[a\x1F8\x92a\x19\xA2a\x1FW\x92a\x1F\\\x94a\x1E,V[a \xADV[\x91a\x1F(V[a\x1Fo\x91a\x1FW\x91a\x1EMV[\x94a\x1E\xFBV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FtradingFunction: invalid x\0\0\0\0\0\0`D\x82\x01R`d\x90\xFD[\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x03\xFFW`\x01g\r\xE0\xB6\xB3\xA7d\0\0`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[a\x03\xE9\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x03\xFFW`\x01a\x03\xE8`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[\x90g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x81\x02\x92\x81\x84\x04\x14\x90\x15\x17\x81\x15\x15\x16\x15a\x03\xFFW`\x01\x90`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[a\x01\0\x81\x83\x03\x12a\x02\xC0W\x80Q\x92a\x02j` \x83\x01Q\x93```@\x85\x01Q\x94\x01a\x10\xCFV[\x90a o\x90a%SV[c;\x9A\xCA\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17\x15a\rfWa\x02j\x91a\x1E,V[a\x02j\x91a\x1Dlg\r\xE0\xB6\xB3\xA7d\0\0a\x1Dfa\x1Daa\x1Dq\x95a&IV[g\x06\xF0[Y\xD3\xB2\0\0\x81\x14a\"jWg\r\xE0\xB6\xB3\xA7d\0\0\x80\x82\x12\x15a\"\x14W\x81\x15a\"5W`\x01\x82`\x01\x1B\x91`\x02\x93\x83\x05`\x02\x03a\rfW`\0\x83\x12\x80\x15a\"YW[a\"GW\x82\x15a\"\x14Wg\x1B\xC1mgN\xC8\0\0\x83\x14a\"5W\x82\x12\x91\x82\x15a\"&W\x92[a!\x1E\x84a)\x01V[\x80\x15a\"\x14Wa!\x8Ba!Oa!Ja!Ea!@a!\x90\x95\x99\x97\x96\x99a&IV[a(iV[a%SV[a\x17\x10V[a\x18\x1Aa!ca!^\x83a),V[a\x19\x1EV[a!\x85a!\x80a!za!u\x86a)WV[a\x196V[\x85a(\xE0V[a\x18@V[\x90a)\xA0V[a(\x91V[\x93`\0\x92[\x81\x84\x10a!\xC7WPPPPa\x02j\x91a!\xB4\x91`\0\x14a!\xB9Wa(\x17V[a\x1D\x86V[a!\xC2\x90a\x1D\x86V[a(\x17V[\x90\x91a\"\n\x86a\"\x04a!\xDF\x85a\x18\x1A\x86\x99\x9Ba$3V[a!\x85a!\xFAa!\xF5a\x18\x1Fa!\xB4\x87\x80a(\xE0V[a(\xB9V[a\x1Dv\x83\x86a(\xE0V[\x90a\x19NV[\x95\x01\x92\x91\x90a!\x95V[`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x90\xFD[a\"/\x90a\x16yV[\x92a!\x15V[`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x90\xFD[`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x90\xFD[Pg\x1B\xC1mgN\xC8\0\0\x83\x13a \xF1V[P`\0\x90V[h\x02H\xCE6\xA7\x0C\xB2k>\x19\x81\x13\x15a\"jWh\x07U\xBFy\x8BJ\x1B\xF1\xE5\x81\x12\x15a#\xC0We\x03x-\xAC\xE9\xD9\x90`N\x1B\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x91``\x90`\x01`_\x1B\x84\x82\x84\x1B\x05\x01\x82\x1D\x93\x84\x02\x90\x03\x80l\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x81\x01\x02\x82\x1D\x90n\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dPn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x82n\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x81m\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x81m\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x81m\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x81l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x02\x8D\x1D\x01\x02\x8B\x1D\x01\x02\x89\x1D\x01\x02\x87\x1D\x01\x02\x85\x1D\x01\x93m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x93m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEA\x81\x01\x90\x84m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x91\x01\x01\x02\x90\x1D\x01\x02\x01\x05\x02\x90`\xC3\x03\x1C\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x90\xFD[a\x1D\x81a\x1D|a\x02j\x93a\"\x04a\x1Dq\x82Qa\x1Dlg\r\xE0\xB6\xB3\xA7d\0\0a\x1Dfa\x1Daa\x1D[a\x1DV`@` \x8B\x01Q\x9A\x01Q\x96a\x1DP\x88\x8Ca eV[\x80\x15a%FWgV\x98\xEE\xF0fp\0\0\x81\x12\x15a\"jWgV\x98\xEE\xF0fo\xFF\xFF\x19\x81\x13\x15a%9W`\0a%)a$h\x83a'\xEAV[a$\xF1a\x18\x1Fa$\x82a$}a!\x80\x85a\x1E\x96V[a)\x81V[\x92a\x1F>a%$a%\x1Fa%\x18a%\x12a%\ra%\x07a%\x02a$\xFCa$\xF7\x8Da$\xF1a$\xECa$\xE6a$\xE1a!za$\xDCa$\xD6a$\xD1a$\xCBa$\xC6\x8Aa(>V[a\x18XV[\x89a(\xE0V[a\x18rV[\x87a(\xE0V[a\x18\x8AV[a\x18\xA4V[\x83a(\xE0V[a\x18\xBCV[\x90a(\xE0V[a\x18\xD6V[\x8Ca(\xE0V[a\x18\xEEV[\x8Aa(\xE0V[a\x19\x06V[\x88a(\xE0V[\x93\x80a(\xE0V[a\x16\xEFV[a\x16`V[\x91\x12\x15a\x02jWa\x02j\x90a\x16yV[Pg\x1B\xC1mgN\xC8\0\0\x90V[Pg\r\xE0\xB6\xB3\xA7d\0\0\x90V[`\xB5\x81`\x01`\x88\x1B\x81\x10\x15a%\xFAW[\x80i\x01\0\0\0\0\0\0\0\0\0b\x01\0\0\x92\x10\x15a%\xEDW[e\x01\0\0\0\0\0\x81\x10\x15a%\xE0W[c\x01\0\0\0\x81\x10\x15a%\xD3W[\x01\x02`\x12\x1C`\x01\x90\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x80\x80\x92\x04\x10\x90\x03\x90V[`\x10\x1C\x91`\x08\x1B\x91a%\x97V[` \x1C\x91`\x10\x1B\x91a%\x8AV[`@\x1C\x91` \x1B\x91a%{V[Ph\xB5\0\0\0\0\0\0\0\0\x90P`\x80\x82\x90\x1Ca%cV[\x15a&\x18WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x90\xFD[}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x90a&u`\0\x82\x13a&\x11V[q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06a&\x91\x82a)\xC2V[``\x92\x83\x82`\x9F\x03\x01\x1B`\x9F\x1C\x90`_\x19\x01}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x02\x92l\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x82m\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x81m\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x81m\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x81m\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x81m\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x81lFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x02\x8C\x1D\x01\x02\x8A\x1D\x01\x02\x88\x1D\x01\x02\x86\x1D\x01\x02\x84\x1D\x01\x02\x82\x1D\x01\x91x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x91l\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x90\x82m\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x81l\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x81m\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x81m\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x81l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x81\x01\x02\x89\x1D\x01\x02\x87\x1D\x01\x02\x85\x1D\x01\x02\x83\x1D\x01\x02\x90\x1D\x01\x02\x01\x05\x02\x01\x01`\xAE\x1D\x90V[`\x01`\xFF\x1B\x81\x14a(\x05W`\0\x81\x12\x15a\x02jW\x19`\x01\x01\x90V[`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x90\xFD[g\x13\xA0K\xBD\xFD\xC9\xBE\x88\x90\x80\x82\x02\x91\x82\x05\x14`\x01\x16\x15a\x03\xFFWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\x02_\x0F\xE1\x05\xA3\x14\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x03\xFFWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\x1B\xC1mgN\xC7\xFF\xFF\x19\x81\x81\x02\x91`\x01\x91\x83\x05\x14\x16\x15a\x03\xFFWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\t\xD0(\xCCo _\xFF\x19\x81\x81\x02\x91`\x01\x91\x83\x05\x14\x16\x15a\x03\xFFWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x90\x80\x82\x02\x91\x82\x05\x14`\x01\x16\x15a\x03\xFFWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x03\xFFWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x03\xFFWg\x1B\xC1mgN\xC8\0\0\x90\x05\x90V[g\x03\xC1f\\z\xAB \0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x03\xFFWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[f\x9F2u$b\xA0\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x03\xFFWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[`\x01\x81\x15\x15\x16\x15a\x03\xFFWn\xC0\x97\xCE{\xC9\x07\x15\xB3K\x9F\x10\0\0\0\0\x05\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17\x82\x15\x15\x16\x15a\x03\xFFW\x05\x90V[a)\xCD\x81\x15\x15a&\x11V[\x80o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x07\x1B\x81\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x81\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x81\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x81\x81\x1C`\xFF\x10`\x03\x1B\x17\x81\x81\x1C`\x0F\x10`\x02\x1B\x17\x81\x81\x1C`\x03\x10`\x01\x1B\x17\x90\x81\x1C`\x01\x10\x17\x90V[\x90a*Ka\x02j\x92` \x80\x82Q\x83\x01\x01\x91\x01a @V[\x93\x91\x92\x90Pa\x1E\xC1V[\x90a*la\x02j\x92` \x80\x82Q\x83\x01\x01\x91\x01a @V[\x93\x90P\x91\x90\x91a\x1E\xC1V[\x90a*\x8Ea\x02j\x92` \x80\x82Q\x83\x01\x01\x91\x01a @V[\x93\x92\x90Pa\x1E\xC1V\xFE\xA2dipfsX\"\x12 \\\xB4]\xDA\xE4\xE6\xE8!\xB5[\xD5\xEE,A\xAE6x\x1B\xB8u\xFE\xBCI\x82\x91\x99\xEE\xD4\xC3\xD17\x0BdsolcC\0\x08\x16\x003";
    /// The deployed bytecode of the contract.
    pub static LOGNORMALSOLVER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct LogNormalSolver<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for LogNormalSolver<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for LogNormalSolver<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for LogNormalSolver<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for LogNormalSolver<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(LogNormalSolver))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> LogNormalSolver<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                LOGNORMALSOLVER_ABI.clone(),
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
                LOGNORMALSOLVER_ABI.clone(),
                LOGNORMALSOLVER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `BISECTION_EPSILON` (0x6d652299) function
        pub fn bisection_epsilon(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([109, 101, 34, 153], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `MAX_BISECTION_ITERS` (0xf9c28211) function
        pub fn max_bisection_iters(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([249, 194, 130, 17], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `allocateGivenX` (0xee3e8cfb) function
        pub fn allocate_given_x(
            &self,
            pool_id: ::ethers::core::types::U256,
            amount_x: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([238, 62, 140, 251], (pool_id, amount_x))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `allocateGivenY` (0x7f17409c) function
        pub fn allocate_given_y(
            &self,
            pool_id: ::ethers::core::types::U256,
            amount_y: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([127, 23, 64, 156], (pool_id, amount_y))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deallocateGivenX` (0x6237569f) function
        pub fn deallocate_given_x(
            &self,
            pool_id: ::ethers::core::types::U256,
            amount_x: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([98, 55, 86, 159], (pool_id, amount_x))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deallocateGivenY` (0xf30d37f2) function
        pub fn deallocate_given_y(
            &self,
            pool_id: ::ethers::core::types::U256,
            amount_y: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([243, 13, 55, 242], (pool_id, amount_y))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `fetchPoolParams` (0x81b5fac2) function
        pub fn fetch_pool_params(
            &self,
            pool_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, LogNormalParams> {
            self.0
                .method_hash([129, 181, 250, 194], pool_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getInitialPoolData` (0x134ead12) function
        pub fn get_initial_pool_data(
            &self,
            rx: ::ethers::core::types::U256,
            s: ::ethers::core::types::U256,
            params: LogNormalParams,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Bytes> {
            self.0
                .method_hash([19, 78, 173, 18], (rx, s, params))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getNextLiquidity` (0xaf4e437f) function
        pub fn get_next_liquidity(
            &self,
            pool_id: ::ethers::core::types::U256,
            rx: ::ethers::core::types::U256,
            ry: ::ethers::core::types::U256,
            l: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([175, 78, 67, 127], (pool_id, rx, ry, l))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getNextReserveX` (0x5eb408fc) function
        pub fn get_next_reserve_x(
            &self,
            pool_id: ::ethers::core::types::U256,
            ry: ::ethers::core::types::U256,
            l: ::ethers::core::types::U256,
            s: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([94, 180, 8, 252], (pool_id, ry, l, s))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getNextReserveY` (0x120649c5) function
        pub fn get_next_reserve_y(
            &self,
            pool_id: ::ethers::core::types::U256,
            rx: ::ethers::core::types::U256,
            l: ::ethers::core::types::U256,
            s: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([18, 6, 73, 197], (pool_id, rx, l, s))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPriceGivenXL` (0x1e978cb0) function
        pub fn get_price_given_xl(
            &self,
            pool_id: ::ethers::core::types::U256,
            rx: ::ethers::core::types::U256,
            l: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([30, 151, 140, 176], (pool_id, rx, l))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPriceGivenYL` (0x4e817fd9) function
        pub fn get_price_given_yl(
            &self,
            pool_id: ::ethers::core::types::U256,
            ry: ::ethers::core::types::U256,
            l: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([78, 129, 127, 217], (pool_id, ry, l))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getReservesAndLiquidity` (0xce153bf4) function
        pub fn get_reserves_and_liquidity(
            &self,
            pool_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([206, 21, 59, 244], pool_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `internalPrice` (0x3b4d1030) function
        pub fn internal_price(
            &self,
            pool_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([59, 77, 16, 48], pool_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `prepareControllerUpdate` (0xcb1f5532) function
        pub fn prepare_controller_update(
            &self,
            controller: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Bytes> {
            self.0
                .method_hash([203, 31, 85, 50], controller)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `prepareFeeUpdate` (0xb09d04e5) function
        pub fn prepare_fee_update(
            &self,
            swap_fee: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Bytes> {
            self.0
                .method_hash([176, 157, 4, 229], swap_fee)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `prepareSigmaUpdate` (0xe94716d5) function
        pub fn prepare_sigma_update(
            &self,
            target_sigma: ::ethers::core::types::U256,
            target_timestamp: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Bytes> {
            self.0
                .method_hash([233, 71, 22, 213], (target_sigma, target_timestamp))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `prepareStrikeUpdate` (0x0420580a) function
        pub fn prepare_strike_update(
            &self,
            target_strike: ::ethers::core::types::U256,
            target_timestamp: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Bytes> {
            self.0
                .method_hash([4, 32, 88, 10], (target_strike, target_timestamp))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `prepareTauUpdate` (0x3b268d5d) function
        pub fn prepare_tau_update(
            &self,
            target_tau: ::ethers::core::types::U256,
            target_timestamp: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Bytes> {
            self.0
                .method_hash([59, 38, 141, 93], (target_tau, target_timestamp))
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
        ///Calls the contract's `strategy` (0xa8c62e76) function
        pub fn strategy(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([168, 198, 46, 118], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for LogNormalSolver<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `BisectionLib_InvalidBounds` with signature `BisectionLib_InvalidBounds(uint256,uint256)` and selector `0x6105bfb6`
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
        name = "BisectionLib_InvalidBounds",
        abi = "BisectionLib_InvalidBounds(uint256,uint256)"
    )]
    pub struct BisectionLib_InvalidBounds {
        pub lower: ::ethers::core::types::U256,
        pub upper: ::ethers::core::types::U256,
    }
    ///Custom Error type `BisectionLib_RootOutsideBounds` with signature `BisectionLib_RootOutsideBounds(int256,int256)` and selector `0x1bc6f974`
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
        name = "BisectionLib_RootOutsideBounds",
        abi = "BisectionLib_RootOutsideBounds(int256,int256)"
    )]
    pub struct BisectionLib_RootOutsideBounds {
        pub lower_result: ::ethers::core::types::I256,
        pub upper_result: ::ethers::core::types::I256,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
    )]
    pub enum LogNormalSolverErrors {
        BisectionLib_InvalidBounds(BisectionLib_InvalidBounds),
        BisectionLib_RootOutsideBounds(BisectionLib_RootOutsideBounds),
        Infinity(Infinity),
        Min(Min),
        NegativeInfinity(NegativeInfinity),
        OutOfBounds(OutOfBounds),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for LogNormalSolverErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) =
                <BisectionLib_InvalidBounds as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::BisectionLib_InvalidBounds(decoded));
            }
            if let Ok(decoded) =
                <BisectionLib_RootOutsideBounds as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::BisectionLib_RootOutsideBounds(decoded));
            }
            if let Ok(decoded) = <Infinity as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Infinity(decoded));
            }
            if let Ok(decoded) = <Min as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Min(decoded));
            }
            if let Ok(decoded) = <NegativeInfinity as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::NegativeInfinity(decoded));
            }
            if let Ok(decoded) = <OutOfBounds as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::OutOfBounds(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for LogNormalSolverErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::BisectionLib_InvalidBounds(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BisectionLib_RootOutsideBounds(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Infinity(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Min(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NegativeInfinity(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OutOfBounds(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for LogNormalSolverErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <BisectionLib_InvalidBounds as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <BisectionLib_RootOutsideBounds as ::ethers::contract::EthError>::selector() => {
                    true
                }
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
    impl ::core::fmt::Display for LogNormalSolverErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::BisectionLib_InvalidBounds(element) => ::core::fmt::Display::fmt(element, f),
                Self::BisectionLib_RootOutsideBounds(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Infinity(element) => ::core::fmt::Display::fmt(element, f),
                Self::Min(element) => ::core::fmt::Display::fmt(element, f),
                Self::NegativeInfinity(element) => ::core::fmt::Display::fmt(element, f),
                Self::OutOfBounds(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for LogNormalSolverErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<BisectionLib_InvalidBounds> for LogNormalSolverErrors {
        fn from(value: BisectionLib_InvalidBounds) -> Self {
            Self::BisectionLib_InvalidBounds(value)
        }
    }
    impl ::core::convert::From<BisectionLib_RootOutsideBounds> for LogNormalSolverErrors {
        fn from(value: BisectionLib_RootOutsideBounds) -> Self {
            Self::BisectionLib_RootOutsideBounds(value)
        }
    }
    impl ::core::convert::From<Infinity> for LogNormalSolverErrors {
        fn from(value: Infinity) -> Self {
            Self::Infinity(value)
        }
    }
    impl ::core::convert::From<Min> for LogNormalSolverErrors {
        fn from(value: Min) -> Self {
            Self::Min(value)
        }
    }
    impl ::core::convert::From<NegativeInfinity> for LogNormalSolverErrors {
        fn from(value: NegativeInfinity) -> Self {
            Self::NegativeInfinity(value)
        }
    }
    impl ::core::convert::From<OutOfBounds> for LogNormalSolverErrors {
        fn from(value: OutOfBounds) -> Self {
            Self::OutOfBounds(value)
        }
    }
    ///Container type for all input parameters for the `BISECTION_EPSILON` function with signature `BISECTION_EPSILON()` and selector `0x6d652299`
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
    #[ethcall(name = "BISECTION_EPSILON", abi = "BISECTION_EPSILON()")]
    pub struct BisectionEpsilonCall;
    ///Container type for all input parameters for the `MAX_BISECTION_ITERS` function with signature `MAX_BISECTION_ITERS()` and selector `0xf9c28211`
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
    #[ethcall(name = "MAX_BISECTION_ITERS", abi = "MAX_BISECTION_ITERS()")]
    pub struct MaxBisectionItersCall;
    ///Container type for all input parameters for the `allocateGivenX` function with signature `allocateGivenX(uint256,uint256)` and selector `0xee3e8cfb`
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
    #[ethcall(name = "allocateGivenX", abi = "allocateGivenX(uint256,uint256)")]
    pub struct AllocateGivenXCall {
        pub pool_id: ::ethers::core::types::U256,
        pub amount_x: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `allocateGivenY` function with signature `allocateGivenY(uint256,uint256)` and selector `0x7f17409c`
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
    #[ethcall(name = "allocateGivenY", abi = "allocateGivenY(uint256,uint256)")]
    pub struct AllocateGivenYCall {
        pub pool_id: ::ethers::core::types::U256,
        pub amount_y: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `deallocateGivenX` function with signature `deallocateGivenX(uint256,uint256)` and selector `0x6237569f`
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
    #[ethcall(name = "deallocateGivenX", abi = "deallocateGivenX(uint256,uint256)")]
    pub struct DeallocateGivenXCall {
        pub pool_id: ::ethers::core::types::U256,
        pub amount_x: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `deallocateGivenY` function with signature `deallocateGivenY(uint256,uint256)` and selector `0xf30d37f2`
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
    #[ethcall(name = "deallocateGivenY", abi = "deallocateGivenY(uint256,uint256)")]
    pub struct DeallocateGivenYCall {
        pub pool_id: ::ethers::core::types::U256,
        pub amount_y: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `fetchPoolParams` function with signature `fetchPoolParams(uint256)` and selector `0x81b5fac2`
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
    #[ethcall(name = "fetchPoolParams", abi = "fetchPoolParams(uint256)")]
    pub struct FetchPoolParamsCall {
        pub pool_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getInitialPoolData` function with signature `getInitialPoolData(uint256,uint256,(uint256,uint256,uint256,uint256,address))` and selector `0x134ead12`
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
        name = "getInitialPoolData",
        abi = "getInitialPoolData(uint256,uint256,(uint256,uint256,uint256,uint256,address))"
    )]
    pub struct GetInitialPoolDataCall {
        pub rx: ::ethers::core::types::U256,
        pub s: ::ethers::core::types::U256,
        pub params: LogNormalParams,
    }
    ///Container type for all input parameters for the `getNextLiquidity` function with signature `getNextLiquidity(uint256,uint256,uint256,uint256)` and selector `0xaf4e437f`
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
        name = "getNextLiquidity",
        abi = "getNextLiquidity(uint256,uint256,uint256,uint256)"
    )]
    pub struct GetNextLiquidityCall {
        pub pool_id: ::ethers::core::types::U256,
        pub rx: ::ethers::core::types::U256,
        pub ry: ::ethers::core::types::U256,
        pub l: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getNextReserveX` function with signature `getNextReserveX(uint256,uint256,uint256,uint256)` and selector `0x5eb408fc`
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
        name = "getNextReserveX",
        abi = "getNextReserveX(uint256,uint256,uint256,uint256)"
    )]
    pub struct GetNextReserveXCall {
        pub pool_id: ::ethers::core::types::U256,
        pub ry: ::ethers::core::types::U256,
        pub l: ::ethers::core::types::U256,
        pub s: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getNextReserveY` function with signature `getNextReserveY(uint256,uint256,uint256,uint256)` and selector `0x120649c5`
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
        name = "getNextReserveY",
        abi = "getNextReserveY(uint256,uint256,uint256,uint256)"
    )]
    pub struct GetNextReserveYCall {
        pub pool_id: ::ethers::core::types::U256,
        pub rx: ::ethers::core::types::U256,
        pub l: ::ethers::core::types::U256,
        pub s: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getPriceGivenXL` function with signature `getPriceGivenXL(uint256,uint256,uint256)` and selector `0x1e978cb0`
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
        name = "getPriceGivenXL",
        abi = "getPriceGivenXL(uint256,uint256,uint256)"
    )]
    pub struct GetPriceGivenXLCall {
        pub pool_id: ::ethers::core::types::U256,
        pub rx: ::ethers::core::types::U256,
        pub l: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getPriceGivenYL` function with signature `getPriceGivenYL(uint256,uint256,uint256)` and selector `0x4e817fd9`
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
        name = "getPriceGivenYL",
        abi = "getPriceGivenYL(uint256,uint256,uint256)"
    )]
    pub struct GetPriceGivenYLCall {
        pub pool_id: ::ethers::core::types::U256,
        pub ry: ::ethers::core::types::U256,
        pub l: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getReservesAndLiquidity` function with signature `getReservesAndLiquidity(uint256)` and selector `0xce153bf4`
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
        name = "getReservesAndLiquidity",
        abi = "getReservesAndLiquidity(uint256)"
    )]
    pub struct GetReservesAndLiquidityCall {
        pub pool_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `internalPrice` function with signature `internalPrice(uint256)` and selector `0x3b4d1030`
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
    #[ethcall(name = "internalPrice", abi = "internalPrice(uint256)")]
    pub struct InternalPriceCall {
        pub pool_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `prepareControllerUpdate` function with signature `prepareControllerUpdate(address)` and selector `0xcb1f5532`
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
        name = "prepareControllerUpdate",
        abi = "prepareControllerUpdate(address)"
    )]
    pub struct PrepareControllerUpdateCall {
        pub controller: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `prepareFeeUpdate` function with signature `prepareFeeUpdate(uint256)` and selector `0xb09d04e5`
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
    #[ethcall(name = "prepareFeeUpdate", abi = "prepareFeeUpdate(uint256)")]
    pub struct PrepareFeeUpdateCall {
        pub swap_fee: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `prepareSigmaUpdate` function with signature `prepareSigmaUpdate(uint256,uint256)` and selector `0xe94716d5`
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
        name = "prepareSigmaUpdate",
        abi = "prepareSigmaUpdate(uint256,uint256)"
    )]
    pub struct PrepareSigmaUpdateCall {
        pub target_sigma: ::ethers::core::types::U256,
        pub target_timestamp: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `prepareStrikeUpdate` function with signature `prepareStrikeUpdate(uint256,uint256)` and selector `0x0420580a`
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
        name = "prepareStrikeUpdate",
        abi = "prepareStrikeUpdate(uint256,uint256)"
    )]
    pub struct PrepareStrikeUpdateCall {
        pub target_strike: ::ethers::core::types::U256,
        pub target_timestamp: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `prepareTauUpdate` function with signature `prepareTauUpdate(uint256,uint256)` and selector `0x3b268d5d`
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
    #[ethcall(name = "prepareTauUpdate", abi = "prepareTauUpdate(uint256,uint256)")]
    pub struct PrepareTauUpdateCall {
        pub target_tau: ::ethers::core::types::U256,
        pub target_timestamp: ::ethers::core::types::U256,
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
        Hash,
    )]
    #[ethcall(name = "simulateSwap", abi = "simulateSwap(uint256,bool,uint256)")]
    pub struct SimulateSwapCall {
        pub pool_id: ::ethers::core::types::U256,
        pub swap_x_in: bool,
        pub amount_in: ::ethers::core::types::U256,
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
        Hash,
    )]
    #[ethcall(name = "strategy", abi = "strategy()")]
    pub struct StrategyCall;
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
    pub enum LogNormalSolverCalls {
        BisectionEpsilon(BisectionEpsilonCall),
        MaxBisectionIters(MaxBisectionItersCall),
        AllocateGivenX(AllocateGivenXCall),
        AllocateGivenY(AllocateGivenYCall),
        DeallocateGivenX(DeallocateGivenXCall),
        DeallocateGivenY(DeallocateGivenYCall),
        FetchPoolParams(FetchPoolParamsCall),
        GetInitialPoolData(GetInitialPoolDataCall),
        GetNextLiquidity(GetNextLiquidityCall),
        GetNextReserveX(GetNextReserveXCall),
        GetNextReserveY(GetNextReserveYCall),
        GetPriceGivenXL(GetPriceGivenXLCall),
        GetPriceGivenYL(GetPriceGivenYLCall),
        GetReservesAndLiquidity(GetReservesAndLiquidityCall),
        InternalPrice(InternalPriceCall),
        PrepareControllerUpdate(PrepareControllerUpdateCall),
        PrepareFeeUpdate(PrepareFeeUpdateCall),
        PrepareSigmaUpdate(PrepareSigmaUpdateCall),
        PrepareStrikeUpdate(PrepareStrikeUpdateCall),
        PrepareTauUpdate(PrepareTauUpdateCall),
        SimulateSwap(SimulateSwapCall),
        Strategy(StrategyCall),
    }
    impl ::ethers::core::abi::AbiDecode for LogNormalSolverCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <BisectionEpsilonCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::BisectionEpsilon(decoded));
            }
            if let Ok(decoded) =
                <MaxBisectionItersCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::MaxBisectionIters(decoded));
            }
            if let Ok(decoded) =
                <AllocateGivenXCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AllocateGivenX(decoded));
            }
            if let Ok(decoded) =
                <AllocateGivenYCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AllocateGivenY(decoded));
            }
            if let Ok(decoded) =
                <DeallocateGivenXCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DeallocateGivenX(decoded));
            }
            if let Ok(decoded) =
                <DeallocateGivenYCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DeallocateGivenY(decoded));
            }
            if let Ok(decoded) =
                <FetchPoolParamsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::FetchPoolParams(decoded));
            }
            if let Ok(decoded) =
                <GetInitialPoolDataCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetInitialPoolData(decoded));
            }
            if let Ok(decoded) =
                <GetNextLiquidityCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetNextLiquidity(decoded));
            }
            if let Ok(decoded) =
                <GetNextReserveXCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetNextReserveX(decoded));
            }
            if let Ok(decoded) =
                <GetNextReserveYCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetNextReserveY(decoded));
            }
            if let Ok(decoded) =
                <GetPriceGivenXLCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetPriceGivenXL(decoded));
            }
            if let Ok(decoded) =
                <GetPriceGivenYLCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetPriceGivenYL(decoded));
            }
            if let Ok(decoded) =
                <GetReservesAndLiquidityCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetReservesAndLiquidity(decoded));
            }
            if let Ok(decoded) = <InternalPriceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InternalPrice(decoded));
            }
            if let Ok(decoded) =
                <PrepareControllerUpdateCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PrepareControllerUpdate(decoded));
            }
            if let Ok(decoded) =
                <PrepareFeeUpdateCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PrepareFeeUpdate(decoded));
            }
            if let Ok(decoded) =
                <PrepareSigmaUpdateCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PrepareSigmaUpdate(decoded));
            }
            if let Ok(decoded) =
                <PrepareStrikeUpdateCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PrepareStrikeUpdate(decoded));
            }
            if let Ok(decoded) =
                <PrepareTauUpdateCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PrepareTauUpdate(decoded));
            }
            if let Ok(decoded) = <SimulateSwapCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SimulateSwap(decoded));
            }
            if let Ok(decoded) = <StrategyCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Strategy(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for LogNormalSolverCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::BisectionEpsilon(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MaxBisectionIters(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AllocateGivenX(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AllocateGivenY(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DeallocateGivenX(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DeallocateGivenY(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::FetchPoolParams(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetInitialPoolData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetNextLiquidity(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetNextReserveX(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetNextReserveY(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetPriceGivenXL(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetPriceGivenYL(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetReservesAndLiquidity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InternalPrice(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PrepareControllerUpdate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PrepareFeeUpdate(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PrepareSigmaUpdate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PrepareStrikeUpdate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PrepareTauUpdate(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SimulateSwap(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Strategy(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for LogNormalSolverCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::BisectionEpsilon(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaxBisectionIters(element) => ::core::fmt::Display::fmt(element, f),
                Self::AllocateGivenX(element) => ::core::fmt::Display::fmt(element, f),
                Self::AllocateGivenY(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeallocateGivenX(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeallocateGivenY(element) => ::core::fmt::Display::fmt(element, f),
                Self::FetchPoolParams(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetInitialPoolData(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetNextLiquidity(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetNextReserveX(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetNextReserveY(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPriceGivenXL(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPriceGivenYL(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetReservesAndLiquidity(element) => ::core::fmt::Display::fmt(element, f),
                Self::InternalPrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::PrepareControllerUpdate(element) => ::core::fmt::Display::fmt(element, f),
                Self::PrepareFeeUpdate(element) => ::core::fmt::Display::fmt(element, f),
                Self::PrepareSigmaUpdate(element) => ::core::fmt::Display::fmt(element, f),
                Self::PrepareStrikeUpdate(element) => ::core::fmt::Display::fmt(element, f),
                Self::PrepareTauUpdate(element) => ::core::fmt::Display::fmt(element, f),
                Self::SimulateSwap(element) => ::core::fmt::Display::fmt(element, f),
                Self::Strategy(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<BisectionEpsilonCall> for LogNormalSolverCalls {
        fn from(value: BisectionEpsilonCall) -> Self {
            Self::BisectionEpsilon(value)
        }
    }
    impl ::core::convert::From<MaxBisectionItersCall> for LogNormalSolverCalls {
        fn from(value: MaxBisectionItersCall) -> Self {
            Self::MaxBisectionIters(value)
        }
    }
    impl ::core::convert::From<AllocateGivenXCall> for LogNormalSolverCalls {
        fn from(value: AllocateGivenXCall) -> Self {
            Self::AllocateGivenX(value)
        }
    }
    impl ::core::convert::From<AllocateGivenYCall> for LogNormalSolverCalls {
        fn from(value: AllocateGivenYCall) -> Self {
            Self::AllocateGivenY(value)
        }
    }
    impl ::core::convert::From<DeallocateGivenXCall> for LogNormalSolverCalls {
        fn from(value: DeallocateGivenXCall) -> Self {
            Self::DeallocateGivenX(value)
        }
    }
    impl ::core::convert::From<DeallocateGivenYCall> for LogNormalSolverCalls {
        fn from(value: DeallocateGivenYCall) -> Self {
            Self::DeallocateGivenY(value)
        }
    }
    impl ::core::convert::From<FetchPoolParamsCall> for LogNormalSolverCalls {
        fn from(value: FetchPoolParamsCall) -> Self {
            Self::FetchPoolParams(value)
        }
    }
    impl ::core::convert::From<GetInitialPoolDataCall> for LogNormalSolverCalls {
        fn from(value: GetInitialPoolDataCall) -> Self {
            Self::GetInitialPoolData(value)
        }
    }
    impl ::core::convert::From<GetNextLiquidityCall> for LogNormalSolverCalls {
        fn from(value: GetNextLiquidityCall) -> Self {
            Self::GetNextLiquidity(value)
        }
    }
    impl ::core::convert::From<GetNextReserveXCall> for LogNormalSolverCalls {
        fn from(value: GetNextReserveXCall) -> Self {
            Self::GetNextReserveX(value)
        }
    }
    impl ::core::convert::From<GetNextReserveYCall> for LogNormalSolverCalls {
        fn from(value: GetNextReserveYCall) -> Self {
            Self::GetNextReserveY(value)
        }
    }
    impl ::core::convert::From<GetPriceGivenXLCall> for LogNormalSolverCalls {
        fn from(value: GetPriceGivenXLCall) -> Self {
            Self::GetPriceGivenXL(value)
        }
    }
    impl ::core::convert::From<GetPriceGivenYLCall> for LogNormalSolverCalls {
        fn from(value: GetPriceGivenYLCall) -> Self {
            Self::GetPriceGivenYL(value)
        }
    }
    impl ::core::convert::From<GetReservesAndLiquidityCall> for LogNormalSolverCalls {
        fn from(value: GetReservesAndLiquidityCall) -> Self {
            Self::GetReservesAndLiquidity(value)
        }
    }
    impl ::core::convert::From<InternalPriceCall> for LogNormalSolverCalls {
        fn from(value: InternalPriceCall) -> Self {
            Self::InternalPrice(value)
        }
    }
    impl ::core::convert::From<PrepareControllerUpdateCall> for LogNormalSolverCalls {
        fn from(value: PrepareControllerUpdateCall) -> Self {
            Self::PrepareControllerUpdate(value)
        }
    }
    impl ::core::convert::From<PrepareFeeUpdateCall> for LogNormalSolverCalls {
        fn from(value: PrepareFeeUpdateCall) -> Self {
            Self::PrepareFeeUpdate(value)
        }
    }
    impl ::core::convert::From<PrepareSigmaUpdateCall> for LogNormalSolverCalls {
        fn from(value: PrepareSigmaUpdateCall) -> Self {
            Self::PrepareSigmaUpdate(value)
        }
    }
    impl ::core::convert::From<PrepareStrikeUpdateCall> for LogNormalSolverCalls {
        fn from(value: PrepareStrikeUpdateCall) -> Self {
            Self::PrepareStrikeUpdate(value)
        }
    }
    impl ::core::convert::From<PrepareTauUpdateCall> for LogNormalSolverCalls {
        fn from(value: PrepareTauUpdateCall) -> Self {
            Self::PrepareTauUpdate(value)
        }
    }
    impl ::core::convert::From<SimulateSwapCall> for LogNormalSolverCalls {
        fn from(value: SimulateSwapCall) -> Self {
            Self::SimulateSwap(value)
        }
    }
    impl ::core::convert::From<StrategyCall> for LogNormalSolverCalls {
        fn from(value: StrategyCall) -> Self {
            Self::Strategy(value)
        }
    }
    ///Container type for all return fields from the `BISECTION_EPSILON` function with signature `BISECTION_EPSILON()` and selector `0x6d652299`
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
    pub struct BisectionEpsilonReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `MAX_BISECTION_ITERS` function with signature `MAX_BISECTION_ITERS()` and selector `0xf9c28211`
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
    pub struct MaxBisectionItersReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `allocateGivenX` function with signature `allocateGivenX(uint256,uint256)` and selector `0xee3e8cfb`
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
    pub struct AllocateGivenXReturn(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `allocateGivenY` function with signature `allocateGivenY(uint256,uint256)` and selector `0x7f17409c`
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
    pub struct AllocateGivenYReturn(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `deallocateGivenX` function with signature `deallocateGivenX(uint256,uint256)` and selector `0x6237569f`
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
    pub struct DeallocateGivenXReturn(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `deallocateGivenY` function with signature `deallocateGivenY(uint256,uint256)` and selector `0xf30d37f2`
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
    pub struct DeallocateGivenYReturn(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `fetchPoolParams` function with signature `fetchPoolParams(uint256)` and selector `0x81b5fac2`
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
    pub struct FetchPoolParamsReturn(pub LogNormalParams);
    ///Container type for all return fields from the `getInitialPoolData` function with signature `getInitialPoolData(uint256,uint256,(uint256,uint256,uint256,uint256,address))` and selector `0x134ead12`
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
    pub struct GetInitialPoolDataReturn(pub ::ethers::core::types::Bytes);
    ///Container type for all return fields from the `getNextLiquidity` function with signature `getNextLiquidity(uint256,uint256,uint256,uint256)` and selector `0xaf4e437f`
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
    pub struct GetNextLiquidityReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getNextReserveX` function with signature `getNextReserveX(uint256,uint256,uint256,uint256)` and selector `0x5eb408fc`
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
    pub struct GetNextReserveXReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getNextReserveY` function with signature `getNextReserveY(uint256,uint256,uint256,uint256)` and selector `0x120649c5`
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
    pub struct GetNextReserveYReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getPriceGivenXL` function with signature `getPriceGivenXL(uint256,uint256,uint256)` and selector `0x1e978cb0`
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
    pub struct GetPriceGivenXLReturn {
        pub price: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `getPriceGivenYL` function with signature `getPriceGivenYL(uint256,uint256,uint256)` and selector `0x4e817fd9`
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
    pub struct GetPriceGivenYLReturn {
        pub price: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `getReservesAndLiquidity` function with signature `getReservesAndLiquidity(uint256)` and selector `0xce153bf4`
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
    pub struct GetReservesAndLiquidityReturn(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `internalPrice` function with signature `internalPrice(uint256)` and selector `0x3b4d1030`
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
    pub struct InternalPriceReturn {
        pub price: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `prepareControllerUpdate` function with signature `prepareControllerUpdate(address)` and selector `0xcb1f5532`
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
    pub struct PrepareControllerUpdateReturn(pub ::ethers::core::types::Bytes);
    ///Container type for all return fields from the `prepareFeeUpdate` function with signature `prepareFeeUpdate(uint256)` and selector `0xb09d04e5`
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
    pub struct PrepareFeeUpdateReturn(pub ::ethers::core::types::Bytes);
    ///Container type for all return fields from the `prepareSigmaUpdate` function with signature `prepareSigmaUpdate(uint256,uint256)` and selector `0xe94716d5`
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
    pub struct PrepareSigmaUpdateReturn(pub ::ethers::core::types::Bytes);
    ///Container type for all return fields from the `prepareStrikeUpdate` function with signature `prepareStrikeUpdate(uint256,uint256)` and selector `0x0420580a`
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
    pub struct PrepareStrikeUpdateReturn(pub ::ethers::core::types::Bytes);
    ///Container type for all return fields from the `prepareTauUpdate` function with signature `prepareTauUpdate(uint256,uint256)` and selector `0x3b268d5d`
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
    pub struct PrepareTauUpdateReturn(pub ::ethers::core::types::Bytes);
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
        Hash,
    )]
    pub struct SimulateSwapReturn(
        pub bool,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::Bytes,
    );
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
        Hash,
    )]
    pub struct StrategyReturn(pub ::ethers::core::types::Address);
    ///`LogNormalParams(uint256,uint256,uint256,uint256,address)`
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
    pub struct LogNormalParams {
        pub strike: ::ethers::core::types::U256,
        pub sigma: ::ethers::core::types::U256,
        pub tau: ::ethers::core::types::U256,
        pub swap_fee: ::ethers::core::types::U256,
        pub controller: ::ethers::core::types::Address,
    }
}
