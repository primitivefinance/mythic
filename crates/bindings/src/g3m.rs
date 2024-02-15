pub use g3m::*;
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
pub mod g3m {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("dfmm_"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("computeSwapConstant"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "computeSwapConstant",
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
                    ::std::borrow::ToOwned::to_owned("dfmm"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("dfmm"),
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
                    ::std::borrow::ToOwned::to_owned("getPoolParams"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getPoolParams"),
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
                            outputs: ::std::vec![
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
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
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
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
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
                                    name: ::std::borrow::ToOwned::to_owned("invariant"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
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
                                    name: ::std::borrow::ToOwned::to_owned("reserveY"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("totalLiquidity"),
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
                    ::std::borrow::ToOwned::to_owned("internalParams"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("internalParams"),
                            inputs: ::std::vec![
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("wX"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct DynamicParam"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("swapFee"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("controller"),
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
                    ::std::borrow::ToOwned::to_owned("update"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("update"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
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
                    ::std::borrow::ToOwned::to_owned("validateAllocateOrDeallocate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "validateAllocateOrDeallocate",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
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
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
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
                                    name: ::std::borrow::ToOwned::to_owned("invariant"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
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
                                    name: ::std::borrow::ToOwned::to_owned("reserveY"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("totalLiquidity"),
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
                    ::std::borrow::ToOwned::to_owned("validateSwap"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("validateSwap"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
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
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
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
                                    name: ::std::borrow::ToOwned::to_owned("invariant"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("liquidityDelta"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("nextRx"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("nextRy"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("nextL"),
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
                    ::std::borrow::ToOwned::to_owned("InvalidSender"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidSender"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidUpdateCode"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidUpdateCode"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidUpdateEnd"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidUpdateEnd"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidWeightX"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidWeightX"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NotDFMM"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("NotDFMM"),
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
    pub static G3M_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA0\x80`@R4a\0\xF1W`@Q`\x1Fa\x16\x158\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x91`\x01`\x01`@\x1B\x03\x83\x11\x84\x84\x10\x17a\0\xDBW\x80\x84\x92` \x94`@R\x839\x81\x01\x03\x12a\0\x8BWQ`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\0\x86W`\x80R`@Qa\x14\xD6\x90\x81a\x01?\x829`\x80Q\x81\x81\x81a\x04\x89\x01R\x81\x81a\x07\xCD\x01R\x81\x81a\t\x81\x01Ra\n\xDF\x01R\xF3[`\0\x80\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[bF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01Ra7\xB7`\xF1\x1B`d\x82\x01R`\x84\x90\xFD\xFE`\x80`@R`\x046\x10a\x0BvW`\x005`\xE0\x1C\x80b.RK\x14a\0\x8BW\x80c\x1E\xDBq\xE5\x14a\0\x86W\x80ch\xBD>8\x14a\0\x81W\x80cs\xCB-\x03\x14a\0|W\x80c\x8A\x04\xBD\xD5\x14a\0wW\x80c\xAC\xAD)\x89\x14a\0rW\x80c\xAF\xBA\x13\xC4\x14a\0mWc\xDC\x17\x83U\x03a\x0BvWa\x0B\x0EV[a\n\xC9V[a\thV[a\t.V[a\x07\xB1V[a\x04'V[a\x03\x9AV[4a\x01\x07W`@6`\x03\x19\x01\x12a\x01\x02W`$5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\0\xFDWa\0\xF5a\0\xD2a\0\xC4` \x936\x90`\x04\x01a\x03EV[\x83\x80\x82Q\x83\x01\x01\x91\x01a\x0B\xD9V[\x90a\0\xEFa\0\xE1`\x045a\r%V[\x86\x80\x82Q\x83\x01\x01\x91\x01a\x0B\xF4V[\x92a\x0E\x0EV[`@Q\x90\x81R\xF3[a\x01\xA7V[a\x01WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01Ra7\xB7`\xF1\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x90\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\x80\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x02\x82W`@RV[a\x02PV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x02\x82W`@RV[\x92\x91\x92g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x02\x82W`@Q\x91a\x02\xD3`\x1F\x82\x01`\x1F\x19\x16` \x01\x84a\x02\x87V[\x82\x94\x81\x84R\x81\x83\x01\x11a\x02\xF0W\x82\x81` \x93\x84`\0\x96\x017\x01\x01RV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R`\x84\x90\xFD[\x90\x80`\x1F\x83\x01\x12\x15a\x03cW\x81` a\x03`\x935\x91\x01a\x02\xA9V[\x90V[a\x01\xF7V[\x90`@Qa\x03u\x81a\x02fV[```\x03\x82\x94\x80T\x84R`\x01\x81\x01T` \x85\x01R`\x02\x81\x01T`@\x85\x01R\x01T\x91\x01RV[4a\x04\x0CW` 6`\x03\x19\x01\x12a\x01\x02W`\x045`\0R`\0` R`\xC0`@`\0 a\x03\xC6\x81a\x03hV[\x90`\x04\x81\x01T\x90`\x05`\x01\x80`\xA0\x1B\x03\x91\x01T\x16\x90```@Q\x93\x80Q\x85R` \x81\x01Q` \x86\x01R`@\x81\x01Q`@\x86\x01R\x01Q``\x84\x01R`\x80\x83\x01R`\xA0\x82\x01R\xF3[a\x01\x07V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\x04\"WV[`\0\x80\xFD[4a\x04\x0CW``6`\x03\x19\x01\x12a\x01\x02Wa\x04C`\x045a\x04\x11V[`$5`D5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\0\xFDWa\x04f\x906\x90`\x04\x01a\x03EV[\x90a\x04p\x81a\r%V[\x90a\x04\x86\x82Q\x92` \x80\x80\x95\x83\x01\x01\x91\x01a\x0B\xF4V[\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x06\x9FW`@Qc3\x85N\xFD`\xE2\x1B\x81R`\x04\x81\x01\x84\x90R\x90``\x90\x82\x90`$\x90\x82\x90Z\xFA\x94\x85\x15a\x06\x9AW`\0\x90\x81\x92\x82\x97a\x06aW[P\x80\x84\x80a\x04\xFF\x93Q\x83\x01\x01\x91\x01a\x0B\xD9V[\x94\x91\x95\x90\x97\x87\x87\x85\x81\x11`\0\x14a\x05\xC8W\x93a\x05T\x86\x94a\x05N\x86a\x05Ia\x05v\x9B\x97a\x05Da\x05a\x98`@a\x05;a\x05m\x9Fa\x05g\x9Fa\x0C\xAEV[\x91\x01Q\x90a\x11\xEEV[a\x11\xEEV[a\x12\x1AV[Pa\r%V[\x80Q\x81\x01\x82\x01\x91\x01a\x0B\xF4V[\x91a\x0E_V[\x83a\x0C\xCDV[\x93\x82\x86\x85a\x0E\x0EV[\x93\x84`\x13\x19\x12\x92\x83a\x05\xBDW[a\x05\xB9\x93\x94`@Q\x96\x87\x96\x87\x92`\xA0\x94\x91\x97\x96\x95\x92`\xC0\x85\x01\x98\x15\x15\x85R` \x85\x01R`@\x84\x01R``\x83\x01R`\x80\x82\x01R\x01RV[\x03\x90\xF3[`\x14\x86\x12\x93Pa\x05\x83V[PP\x91\x92\x90\x93\x80\x89\x11`\0\x14a\x06\x03Wa\x05aa\x05m\x94a\x05Ta\x05v\x97a\x05N\x85a\x05I\x8F\x99\x8Fa\x05D\x90`@a\x05;\x86a\x05g\x9Fa\x0C\xAEV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`0`$\x82\x01R\x7Finvalid swap: inputs x and y hav`D\x82\x01Roe the same sign!`\x80\x1B`d\x82\x01R`\x84\x90\xFD[\x90\x96Pa\x04\xFF\x92Pa\x06\x8B\x91P``=``\x11a\x06\x93W[a\x06\x83\x81\x83a\x02\x87V[\x81\x01\x90a\x0B\xD9V[\x96\x90\x92a\x04\xECV[P=a\x06yV[a\x0C\x8CV[a\x0C9V[\x90```\x03\x19\x83\x01\x12a\x01\x02W`\x045a\x06\xBD\x81a\x04\x11V[\x91`$5\x91`D5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x83\x82\x11a\0\xFDW\x80`#\x83\x01\x12\x15a\x03cW\x81`\x04\x015\x93\x84\x11a\x07XW`$\x84\x83\x01\x01\x11a\x06\xFFW`$\x01\x91\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rjrray stride`\xA8\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x0ENL/$\r\x8C\xAD\xCC\xEE\x8D`\xAB\x1B`d\x82\x01R`\x84\x90\xFD[4a\x04\x0CWa\x07\xBF6a\x06\xA4V[\x91\x92P`\x01`\x01`\xA0\x1B\x03\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x163\x03a\t\x1CW\x81`\xC0\x91\x81\x01\x03\x12a\x01\x02W\x805\x91` \x82\x015\x91`@\x81\x015\x94``\x82\x015\x90`\xA0\x83\x015\x92a\x08$\x84a\x04\x11V[g\r\xE0\xB6\xB3\xA7d\0\0\x83\x10\x15a\t\nWa\x08\xC8\x94a\x08\xC0\x94`\x80a\x08\xAC\x93a\x08\xB1\x96a\x08Z\x87`\0R`\0` R`@`\0 \x90V[U\x015`\x04a\x08s\x86`\0R`\0` R`@`\0 \x90V[\x01U\x16`\x05a\x08\x8C\x84`\0R`\0` R`@`\0 \x90V[\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90UV[a\r%V[` \x80\x82Q\x83\x01\x01\x91\x01a\x0B\xF4V[\x84\x83\x85a\x0E\x0EV[\x92\x83`\x13\x19\x12\x91\x82a\x08\xFFW[`@\x80Q\x93\x15\x15\x84R` \x84\x01\x95\x90\x95R\x93\x82\x01\x92\x90\x92R``\x81\x01\x92\x90\x92R`\x80\x82\x01R`\xA0\x90\xF3[`\x14\x85\x12\x92Pa\x08\xD5V[`@Qc\xE8\xA3\x8Aa`\xE0\x1B\x81R`\x04\x90\xFD[`@QchS\xCB\xA7`\xE0\x1B\x81R`\x04\x90\xFD[4a\x04\x0CW``a\t>6a\x06\xA4V[\x81\x80\x94P\x94\x92\x94\x01\x03\x12a\x01\x02W\x805\x90a\x08\xC8a\x08\xC0a\x08\xB1`@` \x85\x015\x94\x015\x95a\r%V[4a\x04\x0CWa\tv6a\x06\xA4V[\x92`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x163\x03a\t\x1CWa\t\xDEa\t\xD2`\x05a\t\xC4\x87`\0R`\0` R`@`\0 \x90V[\x01T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x91\x16\x03a\n\xB7Wa\t\xF1\x83\x82\x01\x82a\x0C\xF0V[a\t\xFA\x81a\r\x05V[`\x01\x81\x03a\n3WPa\n\x1Da\n\x18a\n.\x92`\x04\x94\x956\x91a\x02\xA9V[a\x0FPV[\x92`\0R`\0` R`@`\0 \x90V[\x01U[\0[a\n<\x81a\r\x05V[`\x02\x81\x03a\nxWP\x90a\n`a\n[a\ns\x93a\n1\x956\x91a\x02\xA9V[a\x0E\xA6V[\x92\x90\x91`\0R`\0` R`@`\0 \x90V[a\x0E\xCEV[\x80a\n\x84`\x03\x92a\r\x05V[\x03a\n\xA5Wa\x08\x8Ca\n\x1Da\n\xA0`\x05\x93a\n1\x966\x91a\x02\xA9V[a\x0E~V[`@Qc#]+=`\xE0\x1B\x81R`\x04\x90\xFD[`@Qcn\xDA\xEF/`\xE1\x1B\x81R`\x04\x90\xFD[4a\x04\x0CW`\x006`\x03\x19\x01\x12a\x01\x02W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x04\x0CW` \x80`\x03\x196\x01\x12a\x01\x02Wa\x0B+`\x045a\r%V[\x90`@\x80Q\x91` \x83R\x83Q\x91\x82` \x85\x01R`\0[\x83\x81\x10a\x0BcW\x84`@\x81\x86`\0\x83\x82\x84\x01\x01R`\x1F\x80\x19\x91\x01\x16\x81\x01\x03\x01\x90\xF3[\x85\x81\x01\x83\x01Q\x85\x82\x01\x83\x01R\x82\x01a\x0BAV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01Rtnor receive functions`X\x1B`d\x82\x01R`\x84\x90\xFD[\x90\x81``\x91\x03\x12a\x01\x02W\x80Q\x91`@` \x83\x01Q\x92\x01Q\x90V[\x90\x81`\x80\x91\x03\x12a\x01\x02W```@Q\x91a\x0C\x0E\x83a\x02fV[\x80Q\x83R` \x81\x01Q` \x84\x01R`@\x81\x01Q`@\x84\x01R\x01Qa\x0C1\x81a\x04\x11V[``\x82\x01R\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01Rd code`\xD8\x1B`d\x82\x01R`\x84\x90\xFD[`@Q=`\0\x82>=\x90\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x91\x90\x82\x03\x91\x82\x11a\x0C\xBBWV[a\x0C\x98V[\x91\x90\x82\x01\x80\x92\x11a\x0C\xBBWV[\x81\x81\x03\x92\x91`\0\x13\x80\x15\x82\x85\x13\x16\x91\x84\x12\x16\x17a\x0C\xBBWV[`\x04\x11\x15a\x04\"WV[\x90\x81` \x91\x03\x12a\x01\x02W5a\x03`\x81a\x0C\xE6V[`\x04\x11\x15a\r\x0FWV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`@\x80Qa\r2\x81a\x02fV[`\0\x91\x82\x82R` \x82\x01\x93\x83\x85R\x81\x83\x01\x84\x81R``\x84\x01\x90\x85\x82R\x82\x86R\x85` Ra\rha\rc\x85\x88 a\x03hV[a\x0F\x85V[\x80\x86Rg\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x03\x90\x81\x11a\x0C\xBBW\x84a\x03`\x97a\r\xC5\x95a\r\xB8\x94`\x05\x94a\x0E\0\x9CR\x81\x83R\x82` R`\x04\x84\x84 \x01T\x90R\x81R\x80` R \x01`\x01\x80`\xA0\x1B\x03\x90T\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x90RV[Q\x92\x83\x91` \x83\x01\x91\x90\x91```\x80\x82\x01\x93\x80Q\x83R` \x81\x01Q` \x84\x01R`@\x81\x01Q`@\x84\x01R\x81`\x01\x80`\xA0\x1B\x03\x91\x01Q\x16\x91\x01RV[\x03`\x1F\x19\x81\x01\x83R\x82a\x02\x87V[\x92` a\x0E7\x84a\x0E1a\x0E)a\x0E@\x96\x97a\x0EF\x99a\x12JV[\x85Q\x90a\x10.V[\x95a\x12JV[\x91\x01Q\x90a\x10.V[\x90a\x11\xEEV[g\r\xE0\xB6\xB3\xA7c\xFF\xFF\x19\x81\x01\x90\x81\x13`\x01\x16a\x0C\xBBW\x90V[a\x03`\x92\x91` a\x0Eua\x0E@\x93\x85Q\x90a\x10.V[\x93\x01Q\x90a\x10.V[`@\x81\x80Q\x81\x01\x03\x12a\x01\x02W\x80a\x0E\x9B` `@\x93\x01Qa\x0C\xE6V[\x01Qa\t\xD2\x81a\x04\x11V[``\x81\x80Q\x81\x01\x03\x12a\x01\x02Wa\x0E\xC0` \x82\x01Qa\x0C\xE6V[```@\x82\x01Q\x91\x01Q\x90\x91V[\x91\x90B\x82\x11\x15a\x0F>Wa\x0E\xE4a\rc\x84a\x03hV[\x90\x81\x84UB`\x03\x85\x01UB\x83\x03\x91\x83\x83\x11a\x0C\xBBWa\x0F\x02\x91a\x0C\xCDV[B\x83\x14a\x0F(W`\x01`\xFF\x1B\x81\x14`\0\x19\x83\x14\x16a\x0C\xBBW`\x02\x92`\x01\x85\x01U\x05\x91\x01UV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`@Qcf\xF1\x02\xED`\xE1\x1B\x81R`\x04\x90\xFD[`@\x81\x80Q\x81\x01\x03\x12a\x01\x02W\x80a\x0Fm` `@\x93\x01Qa\x0C\xE6V[\x01Q\x90V[\x81\x81\x02\x92\x91\x81\x15\x91\x84\x04\x14\x17\x15a\x0C\xBBWV[``\x81\x01Q\x90` \x81\x01Q\x80\x83\x14a\x10\x04W\x80B\x11`\0\x14a\x0F\xFCW\x91[\x82\x03\x91\x82\x11a\x0C\xBBW`@\x81\x01\x90\x81Q`\0\x81\x13`\0\x14a\x0F\xD6WPa\x03`\x92a\x0F\xD0\x91Q\x92Q\x90a\x0FrV[\x90a\x0C\xC0V[\x90Q\x91P`\x01`\xFF\x1B\x81\x14a\x0C\xBBWa\x03`\x92a\x0F\xF6\x91`\0\x03\x90a\x0FrV[\x90a\x0C\xAEV[PB\x91a\x0F\xA3V[P\x90PQ\x90V[\x81\x81\x02\x92\x91`\0\x82\x12`\x01`\xFF\x1B\x82\x14\x16a\x0C\xBBW\x81\x84\x05\x14\x90\x15\x17\x15a\x0C\xBBWV[a\x11\xDBa\x03`\x92}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84a\x11\xE9\x93a\x10d`\0\x82\x13a\x12lV[q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06a\x10\x80\x82a\x14.V[``\x92\x83\x82`\x9F\x03\x01\x1B`\x9F\x1C\x90`_\x19\x01}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x02\x92l\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x82m\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x81m\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x81m\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x81m\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x81m\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x81lFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x02\x8C\x1D\x01\x02\x8A\x1D\x01\x02\x88\x1D\x01\x02\x86\x1D\x01\x02\x84\x1D\x01\x02\x82\x1D\x01\x91x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x91l\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x90\x82m\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x81l\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x81m\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x81m\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x81l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x81\x01\x02\x89\x1D\x01\x02\x87\x1D\x01\x02\x85\x1D\x01\x02\x83\x1D\x01\x02\x90\x1D\x01\x02\x01\x05\x02\x01\x01`\xAE\x1Da\x10\x0BV[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[a\x12\xA4V[\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x04\"W`\x01g\r\xE0\xB6\xB3\xA7d\0\0`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[\x90g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x81\x02\x92\x81\x84\x04\x14\x90\x15\x17\x81\x15\x15\x16\x15a\x04\"W`\x01\x90`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17\x82\x15\x15\x16\x15a\x04\"W\x04\x90V[\x15a\x12sWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x90\xFD[h\x02H\xCE6\xA7\x0C\xB2k>\x19\x81\x13\x15a\x14(Wh\x07U\xBFy\x8BJ\x1B\xF1\xE5\x81\x12\x15a\x13\xF4We\x03x-\xAC\xE9\xD9\x90`N\x1B\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x91``\x90`\x01`_\x1B\x84\x82\x84\x1B\x05\x01\x82\x1D\x93\x84\x02\x90\x03\x80l\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x81\x01\x02\x82\x1D\x90n\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dPn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x82n\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x81m\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x81m\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x81m\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x81l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x02\x8D\x1D\x01\x02\x8B\x1D\x01\x02\x89\x1D\x01\x02\x87\x1D\x01\x02\x85\x1D\x01\x93m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x93m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEA\x81\x01\x90\x84m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x91\x01\x01\x02\x90\x1D\x01\x02\x01\x05\x02\x90`\xC3\x03\x1C\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x90\xFD[P`\0\x90V[a\x149\x81\x15\x15a\x12lV[\x80o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x07\x1B\x81\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x81\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x81\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x81\x81\x1C`\xFF\x10`\x03\x1B\x17\x81\x81\x1C`\x0F\x10`\x02\x1B\x17\x81\x81\x1C`\x03\x10`\x01\x1B\x17\x90\x81\x1C`\x01\x10\x17\x90V\xFE\xA2dipfsX\"\x12 \x1Cs#\xD0 \xEFE\x17\x1E\xCD\xD0|\xF3\x18\xD5(O\xE9\xB32*\xC7\xF91Z\xD4\x7F\xBA\x08\xDF\xB2\x80dsolcC\0\x08\x16\x003";
    /// The bytecode of the contract.
    pub static G3M_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\x0BvW`\x005`\xE0\x1C\x80b.RK\x14a\0\x8BW\x80c\x1E\xDBq\xE5\x14a\0\x86W\x80ch\xBD>8\x14a\0\x81W\x80cs\xCB-\x03\x14a\0|W\x80c\x8A\x04\xBD\xD5\x14a\0wW\x80c\xAC\xAD)\x89\x14a\0rW\x80c\xAF\xBA\x13\xC4\x14a\0mWc\xDC\x17\x83U\x03a\x0BvWa\x0B\x0EV[a\n\xC9V[a\thV[a\t.V[a\x07\xB1V[a\x04'V[a\x03\x9AV[4a\x01\x07W`@6`\x03\x19\x01\x12a\x01\x02W`$5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\0\xFDWa\0\xF5a\0\xD2a\0\xC4` \x936\x90`\x04\x01a\x03EV[\x83\x80\x82Q\x83\x01\x01\x91\x01a\x0B\xD9V[\x90a\0\xEFa\0\xE1`\x045a\r%V[\x86\x80\x82Q\x83\x01\x01\x91\x01a\x0B\xF4V[\x92a\x0E\x0EV[`@Q\x90\x81R\xF3[a\x01\xA7V[a\x01WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01Ra7\xB7`\xF1\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x90\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\x80\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x02\x82W`@RV[a\x02PV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x02\x82W`@RV[\x92\x91\x92g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x02\x82W`@Q\x91a\x02\xD3`\x1F\x82\x01`\x1F\x19\x16` \x01\x84a\x02\x87V[\x82\x94\x81\x84R\x81\x83\x01\x11a\x02\xF0W\x82\x81` \x93\x84`\0\x96\x017\x01\x01RV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R`\x84\x90\xFD[\x90\x80`\x1F\x83\x01\x12\x15a\x03cW\x81` a\x03`\x935\x91\x01a\x02\xA9V[\x90V[a\x01\xF7V[\x90`@Qa\x03u\x81a\x02fV[```\x03\x82\x94\x80T\x84R`\x01\x81\x01T` \x85\x01R`\x02\x81\x01T`@\x85\x01R\x01T\x91\x01RV[4a\x04\x0CW` 6`\x03\x19\x01\x12a\x01\x02W`\x045`\0R`\0` R`\xC0`@`\0 a\x03\xC6\x81a\x03hV[\x90`\x04\x81\x01T\x90`\x05`\x01\x80`\xA0\x1B\x03\x91\x01T\x16\x90```@Q\x93\x80Q\x85R` \x81\x01Q` \x86\x01R`@\x81\x01Q`@\x86\x01R\x01Q``\x84\x01R`\x80\x83\x01R`\xA0\x82\x01R\xF3[a\x01\x07V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\x04\"WV[`\0\x80\xFD[4a\x04\x0CW``6`\x03\x19\x01\x12a\x01\x02Wa\x04C`\x045a\x04\x11V[`$5`D5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\0\xFDWa\x04f\x906\x90`\x04\x01a\x03EV[\x90a\x04p\x81a\r%V[\x90a\x04\x86\x82Q\x92` \x80\x80\x95\x83\x01\x01\x91\x01a\x0B\xF4V[\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x06\x9FW`@Qc3\x85N\xFD`\xE2\x1B\x81R`\x04\x81\x01\x84\x90R\x90``\x90\x82\x90`$\x90\x82\x90Z\xFA\x94\x85\x15a\x06\x9AW`\0\x90\x81\x92\x82\x97a\x06aW[P\x80\x84\x80a\x04\xFF\x93Q\x83\x01\x01\x91\x01a\x0B\xD9V[\x94\x91\x95\x90\x97\x87\x87\x85\x81\x11`\0\x14a\x05\xC8W\x93a\x05T\x86\x94a\x05N\x86a\x05Ia\x05v\x9B\x97a\x05Da\x05a\x98`@a\x05;a\x05m\x9Fa\x05g\x9Fa\x0C\xAEV[\x91\x01Q\x90a\x11\xEEV[a\x11\xEEV[a\x12\x1AV[Pa\r%V[\x80Q\x81\x01\x82\x01\x91\x01a\x0B\xF4V[\x91a\x0E_V[\x83a\x0C\xCDV[\x93\x82\x86\x85a\x0E\x0EV[\x93\x84`\x13\x19\x12\x92\x83a\x05\xBDW[a\x05\xB9\x93\x94`@Q\x96\x87\x96\x87\x92`\xA0\x94\x91\x97\x96\x95\x92`\xC0\x85\x01\x98\x15\x15\x85R` \x85\x01R`@\x84\x01R``\x83\x01R`\x80\x82\x01R\x01RV[\x03\x90\xF3[`\x14\x86\x12\x93Pa\x05\x83V[PP\x91\x92\x90\x93\x80\x89\x11`\0\x14a\x06\x03Wa\x05aa\x05m\x94a\x05Ta\x05v\x97a\x05N\x85a\x05I\x8F\x99\x8Fa\x05D\x90`@a\x05;\x86a\x05g\x9Fa\x0C\xAEV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`0`$\x82\x01R\x7Finvalid swap: inputs x and y hav`D\x82\x01Roe the same sign!`\x80\x1B`d\x82\x01R`\x84\x90\xFD[\x90\x96Pa\x04\xFF\x92Pa\x06\x8B\x91P``=``\x11a\x06\x93W[a\x06\x83\x81\x83a\x02\x87V[\x81\x01\x90a\x0B\xD9V[\x96\x90\x92a\x04\xECV[P=a\x06yV[a\x0C\x8CV[a\x0C9V[\x90```\x03\x19\x83\x01\x12a\x01\x02W`\x045a\x06\xBD\x81a\x04\x11V[\x91`$5\x91`D5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x83\x82\x11a\0\xFDW\x80`#\x83\x01\x12\x15a\x03cW\x81`\x04\x015\x93\x84\x11a\x07XW`$\x84\x83\x01\x01\x11a\x06\xFFW`$\x01\x91\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rjrray stride`\xA8\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x0ENL/$\r\x8C\xAD\xCC\xEE\x8D`\xAB\x1B`d\x82\x01R`\x84\x90\xFD[4a\x04\x0CWa\x07\xBF6a\x06\xA4V[\x91\x92P`\x01`\x01`\xA0\x1B\x03\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x163\x03a\t\x1CW\x81`\xC0\x91\x81\x01\x03\x12a\x01\x02W\x805\x91` \x82\x015\x91`@\x81\x015\x94``\x82\x015\x90`\xA0\x83\x015\x92a\x08$\x84a\x04\x11V[g\r\xE0\xB6\xB3\xA7d\0\0\x83\x10\x15a\t\nWa\x08\xC8\x94a\x08\xC0\x94`\x80a\x08\xAC\x93a\x08\xB1\x96a\x08Z\x87`\0R`\0` R`@`\0 \x90V[U\x015`\x04a\x08s\x86`\0R`\0` R`@`\0 \x90V[\x01U\x16`\x05a\x08\x8C\x84`\0R`\0` R`@`\0 \x90V[\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90UV[a\r%V[` \x80\x82Q\x83\x01\x01\x91\x01a\x0B\xF4V[\x84\x83\x85a\x0E\x0EV[\x92\x83`\x13\x19\x12\x91\x82a\x08\xFFW[`@\x80Q\x93\x15\x15\x84R` \x84\x01\x95\x90\x95R\x93\x82\x01\x92\x90\x92R``\x81\x01\x92\x90\x92R`\x80\x82\x01R`\xA0\x90\xF3[`\x14\x85\x12\x92Pa\x08\xD5V[`@Qc\xE8\xA3\x8Aa`\xE0\x1B\x81R`\x04\x90\xFD[`@QchS\xCB\xA7`\xE0\x1B\x81R`\x04\x90\xFD[4a\x04\x0CW``a\t>6a\x06\xA4V[\x81\x80\x94P\x94\x92\x94\x01\x03\x12a\x01\x02W\x805\x90a\x08\xC8a\x08\xC0a\x08\xB1`@` \x85\x015\x94\x015\x95a\r%V[4a\x04\x0CWa\tv6a\x06\xA4V[\x92`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x163\x03a\t\x1CWa\t\xDEa\t\xD2`\x05a\t\xC4\x87`\0R`\0` R`@`\0 \x90V[\x01T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x91\x16\x03a\n\xB7Wa\t\xF1\x83\x82\x01\x82a\x0C\xF0V[a\t\xFA\x81a\r\x05V[`\x01\x81\x03a\n3WPa\n\x1Da\n\x18a\n.\x92`\x04\x94\x956\x91a\x02\xA9V[a\x0FPV[\x92`\0R`\0` R`@`\0 \x90V[\x01U[\0[a\n<\x81a\r\x05V[`\x02\x81\x03a\nxWP\x90a\n`a\n[a\ns\x93a\n1\x956\x91a\x02\xA9V[a\x0E\xA6V[\x92\x90\x91`\0R`\0` R`@`\0 \x90V[a\x0E\xCEV[\x80a\n\x84`\x03\x92a\r\x05V[\x03a\n\xA5Wa\x08\x8Ca\n\x1Da\n\xA0`\x05\x93a\n1\x966\x91a\x02\xA9V[a\x0E~V[`@Qc#]+=`\xE0\x1B\x81R`\x04\x90\xFD[`@Qcn\xDA\xEF/`\xE1\x1B\x81R`\x04\x90\xFD[4a\x04\x0CW`\x006`\x03\x19\x01\x12a\x01\x02W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x04\x0CW` \x80`\x03\x196\x01\x12a\x01\x02Wa\x0B+`\x045a\r%V[\x90`@\x80Q\x91` \x83R\x83Q\x91\x82` \x85\x01R`\0[\x83\x81\x10a\x0BcW\x84`@\x81\x86`\0\x83\x82\x84\x01\x01R`\x1F\x80\x19\x91\x01\x16\x81\x01\x03\x01\x90\xF3[\x85\x81\x01\x83\x01Q\x85\x82\x01\x83\x01R\x82\x01a\x0BAV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01Rtnor receive functions`X\x1B`d\x82\x01R`\x84\x90\xFD[\x90\x81``\x91\x03\x12a\x01\x02W\x80Q\x91`@` \x83\x01Q\x92\x01Q\x90V[\x90\x81`\x80\x91\x03\x12a\x01\x02W```@Q\x91a\x0C\x0E\x83a\x02fV[\x80Q\x83R` \x81\x01Q` \x84\x01R`@\x81\x01Q`@\x84\x01R\x01Qa\x0C1\x81a\x04\x11V[``\x82\x01R\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01Rd code`\xD8\x1B`d\x82\x01R`\x84\x90\xFD[`@Q=`\0\x82>=\x90\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x91\x90\x82\x03\x91\x82\x11a\x0C\xBBWV[a\x0C\x98V[\x91\x90\x82\x01\x80\x92\x11a\x0C\xBBWV[\x81\x81\x03\x92\x91`\0\x13\x80\x15\x82\x85\x13\x16\x91\x84\x12\x16\x17a\x0C\xBBWV[`\x04\x11\x15a\x04\"WV[\x90\x81` \x91\x03\x12a\x01\x02W5a\x03`\x81a\x0C\xE6V[`\x04\x11\x15a\r\x0FWV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`@\x80Qa\r2\x81a\x02fV[`\0\x91\x82\x82R` \x82\x01\x93\x83\x85R\x81\x83\x01\x84\x81R``\x84\x01\x90\x85\x82R\x82\x86R\x85` Ra\rha\rc\x85\x88 a\x03hV[a\x0F\x85V[\x80\x86Rg\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x03\x90\x81\x11a\x0C\xBBW\x84a\x03`\x97a\r\xC5\x95a\r\xB8\x94`\x05\x94a\x0E\0\x9CR\x81\x83R\x82` R`\x04\x84\x84 \x01T\x90R\x81R\x80` R \x01`\x01\x80`\xA0\x1B\x03\x90T\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x90RV[Q\x92\x83\x91` \x83\x01\x91\x90\x91```\x80\x82\x01\x93\x80Q\x83R` \x81\x01Q` \x84\x01R`@\x81\x01Q`@\x84\x01R\x81`\x01\x80`\xA0\x1B\x03\x91\x01Q\x16\x91\x01RV[\x03`\x1F\x19\x81\x01\x83R\x82a\x02\x87V[\x92` a\x0E7\x84a\x0E1a\x0E)a\x0E@\x96\x97a\x0EF\x99a\x12JV[\x85Q\x90a\x10.V[\x95a\x12JV[\x91\x01Q\x90a\x10.V[\x90a\x11\xEEV[g\r\xE0\xB6\xB3\xA7c\xFF\xFF\x19\x81\x01\x90\x81\x13`\x01\x16a\x0C\xBBW\x90V[a\x03`\x92\x91` a\x0Eua\x0E@\x93\x85Q\x90a\x10.V[\x93\x01Q\x90a\x10.V[`@\x81\x80Q\x81\x01\x03\x12a\x01\x02W\x80a\x0E\x9B` `@\x93\x01Qa\x0C\xE6V[\x01Qa\t\xD2\x81a\x04\x11V[``\x81\x80Q\x81\x01\x03\x12a\x01\x02Wa\x0E\xC0` \x82\x01Qa\x0C\xE6V[```@\x82\x01Q\x91\x01Q\x90\x91V[\x91\x90B\x82\x11\x15a\x0F>Wa\x0E\xE4a\rc\x84a\x03hV[\x90\x81\x84UB`\x03\x85\x01UB\x83\x03\x91\x83\x83\x11a\x0C\xBBWa\x0F\x02\x91a\x0C\xCDV[B\x83\x14a\x0F(W`\x01`\xFF\x1B\x81\x14`\0\x19\x83\x14\x16a\x0C\xBBW`\x02\x92`\x01\x85\x01U\x05\x91\x01UV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`@Qcf\xF1\x02\xED`\xE1\x1B\x81R`\x04\x90\xFD[`@\x81\x80Q\x81\x01\x03\x12a\x01\x02W\x80a\x0Fm` `@\x93\x01Qa\x0C\xE6V[\x01Q\x90V[\x81\x81\x02\x92\x91\x81\x15\x91\x84\x04\x14\x17\x15a\x0C\xBBWV[``\x81\x01Q\x90` \x81\x01Q\x80\x83\x14a\x10\x04W\x80B\x11`\0\x14a\x0F\xFCW\x91[\x82\x03\x91\x82\x11a\x0C\xBBW`@\x81\x01\x90\x81Q`\0\x81\x13`\0\x14a\x0F\xD6WPa\x03`\x92a\x0F\xD0\x91Q\x92Q\x90a\x0FrV[\x90a\x0C\xC0V[\x90Q\x91P`\x01`\xFF\x1B\x81\x14a\x0C\xBBWa\x03`\x92a\x0F\xF6\x91`\0\x03\x90a\x0FrV[\x90a\x0C\xAEV[PB\x91a\x0F\xA3V[P\x90PQ\x90V[\x81\x81\x02\x92\x91`\0\x82\x12`\x01`\xFF\x1B\x82\x14\x16a\x0C\xBBW\x81\x84\x05\x14\x90\x15\x17\x15a\x0C\xBBWV[a\x11\xDBa\x03`\x92}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84a\x11\xE9\x93a\x10d`\0\x82\x13a\x12lV[q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06a\x10\x80\x82a\x14.V[``\x92\x83\x82`\x9F\x03\x01\x1B`\x9F\x1C\x90`_\x19\x01}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x02\x92l\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x82m\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x81m\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x81m\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x81m\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x81m\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x81lFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x02\x8C\x1D\x01\x02\x8A\x1D\x01\x02\x88\x1D\x01\x02\x86\x1D\x01\x02\x84\x1D\x01\x02\x82\x1D\x01\x91x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x91l\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x90\x82m\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x81l\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x81m\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x81m\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x81l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x81\x01\x02\x89\x1D\x01\x02\x87\x1D\x01\x02\x85\x1D\x01\x02\x83\x1D\x01\x02\x90\x1D\x01\x02\x01\x05\x02\x01\x01`\xAE\x1Da\x10\x0BV[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[a\x12\xA4V[\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x04\"W`\x01g\r\xE0\xB6\xB3\xA7d\0\0`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[\x90g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x81\x02\x92\x81\x84\x04\x14\x90\x15\x17\x81\x15\x15\x16\x15a\x04\"W`\x01\x90`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17\x82\x15\x15\x16\x15a\x04\"W\x04\x90V[\x15a\x12sWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x90\xFD[h\x02H\xCE6\xA7\x0C\xB2k>\x19\x81\x13\x15a\x14(Wh\x07U\xBFy\x8BJ\x1B\xF1\xE5\x81\x12\x15a\x13\xF4We\x03x-\xAC\xE9\xD9\x90`N\x1B\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x91``\x90`\x01`_\x1B\x84\x82\x84\x1B\x05\x01\x82\x1D\x93\x84\x02\x90\x03\x80l\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x81\x01\x02\x82\x1D\x90n\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dPn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x82n\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x81m\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x81m\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x81m\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x81l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x02\x8D\x1D\x01\x02\x8B\x1D\x01\x02\x89\x1D\x01\x02\x87\x1D\x01\x02\x85\x1D\x01\x93m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x93m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEA\x81\x01\x90\x84m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x91\x01\x01\x02\x90\x1D\x01\x02\x01\x05\x02\x90`\xC3\x03\x1C\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x90\xFD[P`\0\x90V[a\x149\x81\x15\x15a\x12lV[\x80o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x07\x1B\x81\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x81\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x81\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x81\x81\x1C`\xFF\x10`\x03\x1B\x17\x81\x81\x1C`\x0F\x10`\x02\x1B\x17\x81\x81\x1C`\x03\x10`\x01\x1B\x17\x90\x81\x1C`\x01\x10\x17\x90V\xFE\xA2dipfsX\"\x12 \x1Cs#\xD0 \xEFE\x17\x1E\xCD\xD0|\xF3\x18\xD5(O\xE9\xB32*\xC7\xF91Z\xD4\x7F\xBA\x08\xDF\xB2\x80dsolcC\0\x08\x16\x003";
    /// The deployed bytecode of the contract.
    pub static G3M_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct G3M<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for G3M<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for G3M<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for G3M<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for G3M<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(G3M)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> G3M<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    G3M_ABI.clone(),
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
                G3M_ABI.clone(),
                G3M_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `computeSwapConstant` (0x002e524b) function
        pub fn compute_swap_constant(
            &self,
            pool_id: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([0, 46, 82, 75], (pool_id, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `dfmm` (0xafba13c4) function
        pub fn dfmm(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([175, 186, 19, 196], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPoolParams` (0xdc178355) function
        pub fn get_pool_params(
            &self,
            pool_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([220, 23, 131, 85], pool_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `init` (0x73cb2d03) function
        pub fn init(
            &self,
            p0: ::ethers::core::types::Address,
            pool_id: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                bool,
                ::ethers::core::types::I256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([115, 203, 45, 3], (p0, pool_id, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `internalParams` (0x1edb71e5) function
        pub fn internal_params(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (DynamicParam, ::ethers::core::types::U256, ::ethers::core::types::Address),
        > {
            self.0
                .method_hash([30, 219, 113, 229], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `update` (0xacad2989) function
        pub fn update(
            &self,
            sender: ::ethers::core::types::Address,
            pool_id: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([172, 173, 41, 137], (sender, pool_id, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `validateAllocateOrDeallocate` (0x8a04bdd5) function
        pub fn validate_allocate_or_deallocate(
            &self,
            p0: ::ethers::core::types::Address,
            pool_id: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                bool,
                ::ethers::core::types::I256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([138, 4, 189, 213], (p0, pool_id, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `validateSwap` (0x68bd3e38) function
        pub fn validate_swap(
            &self,
            p0: ::ethers::core::types::Address,
            pool_id: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                bool,
                ::ethers::core::types::I256,
                ::ethers::core::types::I256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([104, 189, 62, 56], (p0, pool_id, data))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for G3M<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `InvalidSender` with signature `InvalidSender()` and selector `0xddb5de5e`
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
    #[etherror(name = "InvalidSender", abi = "InvalidSender()")]
    pub struct InvalidSender;
    ///Custom Error type `InvalidUpdateCode` with signature `InvalidUpdateCode()` and selector `0x235d2b3d`
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
    #[etherror(name = "InvalidUpdateCode", abi = "InvalidUpdateCode()")]
    pub struct InvalidUpdateCode;
    ///Custom Error type `InvalidUpdateEnd` with signature `InvalidUpdateEnd()` and selector `0xcde205da`
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
    #[etherror(name = "InvalidUpdateEnd", abi = "InvalidUpdateEnd()")]
    pub struct InvalidUpdateEnd;
    ///Custom Error type `InvalidWeightX` with signature `InvalidWeightX()` and selector `0xe8a38a61`
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
    #[etherror(name = "InvalidWeightX", abi = "InvalidWeightX()")]
    pub struct InvalidWeightX;
    ///Custom Error type `NotDFMM` with signature `NotDFMM()` and selector `0x6853cba7`
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
    #[etherror(name = "NotDFMM", abi = "NotDFMM()")]
    pub struct NotDFMM;
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
    pub enum G3MErrors {
        InvalidSender(InvalidSender),
        InvalidUpdateCode(InvalidUpdateCode),
        InvalidUpdateEnd(InvalidUpdateEnd),
        InvalidWeightX(InvalidWeightX),
        NotDFMM(NotDFMM),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for G3MErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <InvalidSender as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidSender(decoded));
            }
            if let Ok(decoded) = <InvalidUpdateCode as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidUpdateCode(decoded));
            }
            if let Ok(decoded) = <InvalidUpdateEnd as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidUpdateEnd(decoded));
            }
            if let Ok(decoded) = <InvalidWeightX as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidWeightX(decoded));
            }
            if let Ok(decoded) = <NotDFMM as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NotDFMM(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for G3MErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::InvalidSender(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidUpdateCode(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidUpdateEnd(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidWeightX(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NotDFMM(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for G3MErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <InvalidSender as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidUpdateCode as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidUpdateEnd as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidWeightX as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NotDFMM as ::ethers::contract::EthError>::selector() => true,
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for G3MErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::InvalidSender(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidUpdateCode(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidUpdateEnd(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidWeightX(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotDFMM(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for G3MErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<InvalidSender> for G3MErrors {
        fn from(value: InvalidSender) -> Self {
            Self::InvalidSender(value)
        }
    }
    impl ::core::convert::From<InvalidUpdateCode> for G3MErrors {
        fn from(value: InvalidUpdateCode) -> Self {
            Self::InvalidUpdateCode(value)
        }
    }
    impl ::core::convert::From<InvalidUpdateEnd> for G3MErrors {
        fn from(value: InvalidUpdateEnd) -> Self {
            Self::InvalidUpdateEnd(value)
        }
    }
    impl ::core::convert::From<InvalidWeightX> for G3MErrors {
        fn from(value: InvalidWeightX) -> Self {
            Self::InvalidWeightX(value)
        }
    }
    impl ::core::convert::From<NotDFMM> for G3MErrors {
        fn from(value: NotDFMM) -> Self {
            Self::NotDFMM(value)
        }
    }
    ///Container type for all input parameters for the `computeSwapConstant` function with signature `computeSwapConstant(uint256,bytes)` and selector `0x002e524b`
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
    #[ethcall(name = "computeSwapConstant", abi = "computeSwapConstant(uint256,bytes)")]
    pub struct ComputeSwapConstantCall {
        pub pool_id: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `dfmm` function with signature `dfmm()` and selector `0xafba13c4`
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
    #[ethcall(name = "dfmm", abi = "dfmm()")]
    pub struct DfmmCall;
    ///Container type for all input parameters for the `getPoolParams` function with signature `getPoolParams(uint256)` and selector `0xdc178355`
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
    #[ethcall(name = "getPoolParams", abi = "getPoolParams(uint256)")]
    pub struct GetPoolParamsCall {
        pub pool_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `init` function with signature `init(address,uint256,bytes)` and selector `0x73cb2d03`
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
    #[ethcall(name = "init", abi = "init(address,uint256,bytes)")]
    pub struct InitCall {
        pub p0: ::ethers::core::types::Address,
        pub pool_id: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `internalParams` function with signature `internalParams(uint256)` and selector `0x1edb71e5`
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
    #[ethcall(name = "internalParams", abi = "internalParams(uint256)")]
    pub struct InternalParamsCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `update` function with signature `update(address,uint256,bytes)` and selector `0xacad2989`
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
    #[ethcall(name = "update", abi = "update(address,uint256,bytes)")]
    pub struct UpdateCall {
        pub sender: ::ethers::core::types::Address,
        pub pool_id: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `validateAllocateOrDeallocate` function with signature `validateAllocateOrDeallocate(address,uint256,bytes)` and selector `0x8a04bdd5`
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
        name = "validateAllocateOrDeallocate",
        abi = "validateAllocateOrDeallocate(address,uint256,bytes)"
    )]
    pub struct ValidateAllocateOrDeallocateCall {
        pub p0: ::ethers::core::types::Address,
        pub pool_id: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `validateSwap` function with signature `validateSwap(address,uint256,bytes)` and selector `0x68bd3e38`
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
    #[ethcall(name = "validateSwap", abi = "validateSwap(address,uint256,bytes)")]
    pub struct ValidateSwapCall {
        pub p0: ::ethers::core::types::Address,
        pub pool_id: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
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
    pub enum G3MCalls {
        ComputeSwapConstant(ComputeSwapConstantCall),
        Dfmm(DfmmCall),
        GetPoolParams(GetPoolParamsCall),
        Init(InitCall),
        InternalParams(InternalParamsCall),
        Update(UpdateCall),
        ValidateAllocateOrDeallocate(ValidateAllocateOrDeallocateCall),
        ValidateSwap(ValidateSwapCall),
    }
    impl ::ethers::core::abi::AbiDecode for G3MCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <ComputeSwapConstantCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ComputeSwapConstant(decoded));
            }
            if let Ok(decoded) = <DfmmCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Dfmm(decoded));
            }
            if let Ok(decoded) = <GetPoolParamsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetPoolParams(decoded));
            }
            if let Ok(decoded) = <InitCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Init(decoded));
            }
            if let Ok(decoded) = <InternalParamsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InternalParams(decoded));
            }
            if let Ok(decoded) = <UpdateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Update(decoded));
            }
            if let Ok(decoded) = <ValidateAllocateOrDeallocateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ValidateAllocateOrDeallocate(decoded));
            }
            if let Ok(decoded) = <ValidateSwapCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ValidateSwap(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for G3MCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::ComputeSwapConstant(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Dfmm(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetPoolParams(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Init(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InternalParams(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Update(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ValidateAllocateOrDeallocate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ValidateSwap(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for G3MCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ComputeSwapConstant(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Dfmm(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPoolParams(element) => ::core::fmt::Display::fmt(element, f),
                Self::Init(element) => ::core::fmt::Display::fmt(element, f),
                Self::InternalParams(element) => ::core::fmt::Display::fmt(element, f),
                Self::Update(element) => ::core::fmt::Display::fmt(element, f),
                Self::ValidateAllocateOrDeallocate(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ValidateSwap(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ComputeSwapConstantCall> for G3MCalls {
        fn from(value: ComputeSwapConstantCall) -> Self {
            Self::ComputeSwapConstant(value)
        }
    }
    impl ::core::convert::From<DfmmCall> for G3MCalls {
        fn from(value: DfmmCall) -> Self {
            Self::Dfmm(value)
        }
    }
    impl ::core::convert::From<GetPoolParamsCall> for G3MCalls {
        fn from(value: GetPoolParamsCall) -> Self {
            Self::GetPoolParams(value)
        }
    }
    impl ::core::convert::From<InitCall> for G3MCalls {
        fn from(value: InitCall) -> Self {
            Self::Init(value)
        }
    }
    impl ::core::convert::From<InternalParamsCall> for G3MCalls {
        fn from(value: InternalParamsCall) -> Self {
            Self::InternalParams(value)
        }
    }
    impl ::core::convert::From<UpdateCall> for G3MCalls {
        fn from(value: UpdateCall) -> Self {
            Self::Update(value)
        }
    }
    impl ::core::convert::From<ValidateAllocateOrDeallocateCall> for G3MCalls {
        fn from(value: ValidateAllocateOrDeallocateCall) -> Self {
            Self::ValidateAllocateOrDeallocate(value)
        }
    }
    impl ::core::convert::From<ValidateSwapCall> for G3MCalls {
        fn from(value: ValidateSwapCall) -> Self {
            Self::ValidateSwap(value)
        }
    }
    ///Container type for all return fields from the `computeSwapConstant` function with signature `computeSwapConstant(uint256,bytes)` and selector `0x002e524b`
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
    pub struct ComputeSwapConstantReturn(pub ::ethers::core::types::I256);
    ///Container type for all return fields from the `dfmm` function with signature `dfmm()` and selector `0xafba13c4`
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
    pub struct DfmmReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getPoolParams` function with signature `getPoolParams(uint256)` and selector `0xdc178355`
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
    pub struct GetPoolParamsReturn(pub ::ethers::core::types::Bytes);
    ///Container type for all return fields from the `init` function with signature `init(address,uint256,bytes)` and selector `0x73cb2d03`
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
    pub struct InitReturn {
        pub valid: bool,
        pub invariant: ::ethers::core::types::I256,
        pub reserve_x: ::ethers::core::types::U256,
        pub reserve_y: ::ethers::core::types::U256,
        pub total_liquidity: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `internalParams` function with signature `internalParams(uint256)` and selector `0x1edb71e5`
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
    pub struct InternalParamsReturn {
        pub w_x: DynamicParam,
        pub swap_fee: ::ethers::core::types::U256,
        pub controller: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `validateAllocateOrDeallocate` function with signature `validateAllocateOrDeallocate(address,uint256,bytes)` and selector `0x8a04bdd5`
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
    pub struct ValidateAllocateOrDeallocateReturn {
        pub valid: bool,
        pub invariant: ::ethers::core::types::I256,
        pub reserve_x: ::ethers::core::types::U256,
        pub reserve_y: ::ethers::core::types::U256,
        pub total_liquidity: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `validateSwap` function with signature `validateSwap(address,uint256,bytes)` and selector `0x68bd3e38`
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
    pub struct ValidateSwapReturn {
        pub valid: bool,
        pub invariant: ::ethers::core::types::I256,
        pub liquidity_delta: ::ethers::core::types::I256,
        pub next_rx: ::ethers::core::types::U256,
        pub next_ry: ::ethers::core::types::U256,
        pub next_l: ::ethers::core::types::U256,
    }
}
