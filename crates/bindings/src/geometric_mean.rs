pub use geometric_mean::*;
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
pub mod geometric_mean {
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
                    ::std::borrow::ToOwned::to_owned("name"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("name"),
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
    pub static GEOMETRICMEAN_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA0`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`@Qa\x1A\xB58\x03\x80a\x1A\xB5\x839\x81\x01`@\x81\x90Ra\0|\x91a\0\x8DV[`\x01`\x01`\xA0\x1B\x03\x16`\x80Ra\x01\x08V[`\0` \x82\x84\x03\x12\x15a\0\xEAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01\x01W`\0\x80\xFD[\x93\x92PPPV[`\x80Qa\x19}a\x018`\09`\0\x81\x81a\x02\xFA\x01R\x81\x81a\x03\xCA\x01R\x81\x81a\x06\x1A\x01Ra\x06\xD6\x01Ra\x19}`\0\xF3\xFE`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\0\xDFW`\x005`\xE0\x1C\x80cs\xCB-\x03\x11a\0\xB3W\x80cs\xCB-\x03\x14a\x02\x90W\x80c\x8A\x04\xBD\xD5\x14a\x02\xCDW\x80c\xAC\xAD)\x89\x14a\x02\xE0W\x80c\xAF\xBA\x13\xC4\x14a\x02\xF5W\x80c\xDC\x17\x83U\x14a\x034Wa\0\xDFV[\x80b.RK\x14a\x01DW\x80c\x06\xFD\xDE\x03\x14a\x01jW\x80c\x1E\xDBq\xE5\x14a\x01\xA3W\x80ch\xBD>8\x14a\x02NW[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x01Wa\x01R6`\x04a\x13\xBEV[a\x03GV[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\x96`@Q\x80`@\x01`@R\x80`\r\x81R` \x01l#\xB2\xB7\xB6\xB2\xBA94\xB1\xA6\xB2\xB0\xB7`\x99\x1B\x81RP\x81V[`@Qa\x01a\x91\x90a\x14QV[a\x02\x06a\x01\xB16`\x04a\x14dV[`\0` \x81\x81R\x91\x81R`@\x90\x81\x90 \x81Q`\x80\x81\x01\x83R\x81T\x81R`\x01\x82\x01T\x93\x81\x01\x93\x90\x93R`\x02\x81\x01T\x91\x83\x01\x91\x90\x91R`\x03\x81\x01T``\x83\x01R`\x04\x81\x01T`\x05\x90\x91\x01T`\x01`\x01`\xA0\x1B\x03\x16\x83V[`@\x80Q\x84Q\x81R` \x80\x86\x01Q\x90\x82\x01R\x84\x82\x01Q\x91\x81\x01\x91\x90\x91R``\x93\x84\x01Q\x93\x81\x01\x93\x90\x93R`\x80\x83\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x16`\xA0\x82\x01R`\xC0\x01a\x01aV[a\x02aa\x02\\6`\x04a\x14\x98V[a\x03\x9AV[`@\x80Q\x96\x15\x15\x87R` \x87\x01\x95\x90\x95R\x93\x85\x01\x92\x90\x92R``\x84\x01R`\x80\x83\x01R`\xA0\x82\x01R`\xC0\x01a\x01aV[a\x02\xA3a\x02\x9E6`\x04a\x14\xF7V[a\x06\tV[`@\x80Q\x95\x15\x15\x86R` \x86\x01\x94\x90\x94R\x92\x84\x01\x91\x90\x91R``\x83\x01R`\x80\x82\x01R`\xA0\x01a\x01aV[a\x02\xA3a\x02\xDB6`\x04a\x14\xF7V[a\x06|V[a\x02\xF3a\x02\xEE6`\x04a\x14\xF7V[a\x06\xCBV[\0[a\x03\x1C\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01aV[a\x01\x96a\x03B6`\x04a\x14dV[a\x08\xFCV[`\0\x80`\0\x80\x84\x80` \x01\x90Q\x81\x01\x90a\x03a\x91\x90a\x163V[\x92P\x92P\x92Pa\x03\x8E\x83\x83\x83a\x03v\x8Aa\x08\xFCV[\x80` \x01\x90Q\x81\x01\x90a\x03\x89\x91\x90a\x16dV[a\n\x1BV[\x93PPPP[\x92\x91PPV[`\0\x80`\0\x80`\0\x80`\0a\x03\xAE\x89a\x08\xFCV[\x80` \x01\x90Q\x81\x01\x90a\x03\xC1\x91\x90a\x16dV[\x90P`\0\x80`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xCE\x15;\xF4\x8D`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04\x16\x91\x81R` \x01\x90V[```@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x04~W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x04\x92W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xB6\x91\x90a\x163V[\x92P\x92P\x92P\x8A\x80` \x01\x90Q\x81\x01\x90a\x04\xD0\x91\x90a\x163V[\x91\x98P\x96P\x94P`\0\x80\x80\x85\x8A\x11\x15a\x05-Wa\x04\xED\x86\x8Ba\x16\xE9V[\x92Pa\x05\x06\x87`@\x01Q\x84a\n\x7F\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x91Pa\x05\x1C\x86a\x05\x16\x84\x87a\n\x7FV[\x90a\n\x9BV[a\x05&\x90\x82a\x16\xFCV[\x90Pa\x05\xCEV[\x84\x89\x11\x15a\x05hWa\x05?\x85\x8Aa\x16\xE9V[\x92Pa\x05X\x87`@\x01Q\x84a\n\x7F\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x91Pa\x05\x1C\x85a\x05\x16\x84\x87a\n\x7FV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`0`$\x82\x01R\x7Finvalid swap: inputs x and y hav`D\x82\x01Roe the same sign!`\x80\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[a\x05\xDA\x8A\x8A\x8A\x8Aa\n\x1BV[\x9BP\x8Ba\x05\xE7`\x14a\x17\x0FV[\x12\x80\x15a\x05\xF4WP`\x14\x8C\x12[\x9CPPPPPPPP\x93\x97P\x93\x97\x91\x95P\x93PV[`\0\x80\x80\x80\x803`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x06XW`@QchS\xCB\xA7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x06c\x88\x88\x88a\n\xB0V[P\x95\x9F\x94\x9EP\x92\x9CP\x90\x9AP\x98P\x90\x96PPPPPPPV[`\0\x80\x80\x80\x80a\x06\x8E\x86\x88\x01\x88a\x17+V[\x91\x94P\x92P\x90Pa\x06\xA4\x83\x83\x83a\x03v\x8Ca\x08\xFCV[\x93P\x83a\x06\xB1`\x14a\x17\x0FV[\x12\x80\x15a\x06\xBEWP`\x14\x84\x12[\x94P\x94P\x94P\x94P\x94P\x94V[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x07\x14W`@QchS\xCB\xA7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x83\x81R` \x81\x90R`@\x90 `\x05\x01T`\x01`\x01`\xA0\x1B\x03\x85\x81\x16\x91\x16\x14a\x07QW`@Qcn\xDA\xEF/`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x07_\x82\x84\x01\x84a\x17gV[\x90P`\x01\x81`\x03\x81\x11\x15a\x07uWa\x07ua\x17\x87V[\x03a\x07\xD0Wa\x07\xB9\x83\x83\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x0Bh\x92PPPV[`\0\x85\x81R` \x81\x90R`@\x90 `\x04\x01Ua\x08\xF5V[`\x02\x81`\x03\x81\x11\x15a\x07\xE4Wa\x07\xE4a\x17\x87V[\x03a\x08PW`\0\x80a\x08+\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x0B\x87\x92PPPV[`\0\x88\x81R` \x81\x90R`@\x90 \x91\x93P\x91Pa\x08I\x90\x83\x83a\x0B\xA9V[PPa\x08\xF5V[`\x03\x81`\x03\x81\x11\x15a\x08dWa\x08da\x17\x87V[\x03a\x08\xDCWa\x08\xA8\x83\x83\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x0C\x15\x92PPPV[`\0\x85\x81R` \x81\x90R`@\x90 `\x05\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90Ua\x08\xF5V[`@Qc#]+=`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPV[``a\t2`@Q\x80`\x80\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81RP\x90V[`\0\x83\x81R` \x81\x81R`@\x91\x82\x90 \x82Q`\x80\x81\x01\x84R\x81T\x81R`\x01\x82\x01T\x92\x81\x01\x92\x90\x92R`\x02\x81\x01T\x92\x82\x01\x92\x90\x92R`\x03\x90\x91\x01T``\x82\x01Ra\tz\x90a\x0C+V[\x80\x82Ra\t\x8F\x90g\r\xE0\xB6\xB3\xA7d\0\0a\x16\xE9V[` \x80\x83\x01\x91\x90\x91R`\0\x84\x81R\x80\x82R`@\x80\x82 `\x04\x81\x01T\x82\x86\x01R\x86\x83R\x91\x83R`\x05\x90\x91\x01T`\x01`\x01`\xA0\x1B\x03\x16``\x84\x01RQa\n\x04\x91\x83\x91\x01\x81Q\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x80\x83\x01Q\x90\x82\x01R``\x91\x82\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x91\x81\x01\x91\x90\x91R`\x80\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91PP\x91\x90PV[\x80Q`\0\x90\x81\x90a\n6\x90a\n0\x88\x87a\x0C\xBCV[\x90a\x0C\xD1V[\x90P`\0a\nU\x84` \x01Qa\n0\x87\x89a\x0C\xBC\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0a\nj\x83\x83a\n\x7FV[a\nt\x91\x90a\x17\x9DV[\x97\x96PPPPPPPV[`\0a\n\x94\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\r\x02V[\x93\x92PPPV[`\0a\n\x94\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\r\x02V[`\0\x80\x80\x80\x80\x80\x80\x80a\n\xC5\x89\x8B\x01\x8Ba\x17\xC4V[\x94\x9AP\x92\x98P\x90\x96P\x94P\x92P\x90Pg\r\xE0\xB6\xB3\xA7d\0\0\x83\x10a\n\xFCW`@Qc\xE8\xA3\x8Aa`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x8B\x81R` \x81\x90R`@\x90 \x83\x81U`\x04\x81\x01\x83\x90U`\x05\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x17\x90Ua\x0B?\x86\x86\x86a\x03v\x8Fa\x08\xFCV[\x96P\x86a\x0BL`\x14a\x17\x0FV[\x12\x80\x15a\x0BYWP`\x14\x87\x12[\x97P\x93\x97P\x93\x97P\x93\x97P\x93\x97V[`\0\x80\x82\x80` \x01\x90Q\x81\x01\x90a\x0B\x7F\x91\x90a\x18\x1CV[\x94\x93PPPPV[`\0\x80\x82\x80` \x01\x90Q\x81\x01\x90a\x0B\x9E\x91\x90a\x18MV[\x90\x95\x90\x94P\x92PPPV[B\x81\x11a\x0B\xC9W`@Qcf\xF1\x02\xED`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0B\xD2\x83a\r0V[`\0a\x0B\xDEB\x83a\x16\xE9V[\x84T\x90\x91P`\0\x90a\x0B\xF0\x90\x85a\x17\x9DV[\x90P`\0a\x0B\xFE\x83\x83a\x18\x87V[`\x01\x87\x01\x94\x90\x94UPPP`\x02\x90\x92\x01\x91\x90\x91UPV[`\0\x81\x80` \x01\x90Q\x81\x01\x90a\n\x94\x91\x90a\x18\xC3V[`\0\x81` \x01Q\x82``\x01Q\x03a\x0CAWPQ\x90V[`\0\x82` \x01QB\x11a\x0CTWBa\x0CZV[\x82` \x01Q[\x90P`\0\x83``\x01Q\x82a\x0Cn\x91\x90a\x16\xE9V[\x90P`\0\x84`@\x01Q\x13\x15a\x0C\x98W`@\x84\x01Qa\x0C\x8C\x90\x82a\x19\0V[\x84Qa\x0B\x7F\x91\x90a\x16\xFCV[\x83`@\x01Qa\x0C\xA6\x90a\x17\x0FV[a\x0C\xB0\x90\x82a\x19\0V[\x84Qa\x0B\x7F\x91\x90a\x16\xE9V[`\0a\n\x94\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\rrV[`\0a\n\x94g\r\xE0\xB6\xB3\xA7d\0\0\x83a\x0C\xE9\x86a\r\x91V[a\x0C\xF3\x91\x90a\x19\x17V[a\x0C\xFD\x91\x90a\x18\x87V[a\x0FlV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\r\x1AW`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[`@\x80Q`\x80\x81\x01\x82R\x82T\x81R`\x01\x83\x01T` \x82\x01R`\x02\x83\x01T\x91\x81\x01\x91\x90\x91R`\x03\x82\x01T``\x82\x01Ra\rg\x90a\x0C+V[\x81UB`\x03\x90\x91\x01UV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\r\x8AW`\0\x80\xFD[\x04\x92\x91PPV[`\0\x80\x82\x13a\r\xCEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x05\xC5V[`\0``a\r\xDB\x84a\x11\x15V[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[`\0h\x02H\xCE6\xA7\x0C\xB2k>\x19\x82\x13a\x0F\x87WP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a\x0F\xCEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x01a\x05\xC5V[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05`\x01`_\x1B\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1Dm\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1Dn\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[`\0\x80\x82\x11a\x11RW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x05\xC5V[P`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x82\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x81\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x12\xF5Wa\x12\xF5a\x12\xB6V[`@R\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a\x13\x11Wa\x13\x11a\x12]V[\x815` g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x13-Wa\x13-a\x12\xB6V[a\x13?`\x1F\x83\x01`\x1F\x19\x16\x82\x01a\x12\xCCV[\x82\x81R\x85\x82\x84\x87\x01\x01\x11\x15a\x13\xA3W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x83\x90R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R`\x84\x81\xFD[\x82\x82\x86\x01\x83\x83\x017`\0\x92\x81\x01\x90\x91\x01\x91\x90\x91R\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x13\xD4Wa\x13\xD4a\x11\xBDV[\x825\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x13\xF5Wa\x13\xF5a\x12\rV[a\x14\x01\x85\x82\x86\x01a\x12\xFDV[\x91PP\x92P\x92\x90PV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a\x141W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\x14\x15V[P`\0` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R`\0a\n\x94` \x83\x01\x84a\x14\x0BV[`\0` \x82\x84\x03\x12\x15a\x14yWa\x14ya\x11\xBDV[P5\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x14\x95W`\0\x80\xFD[PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x14\xB0Wa\x14\xB0a\x11\xBDV[\x835a\x14\xBB\x81a\x14\x80V[\x92P` \x84\x015\x91P`@\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x14\xE1Wa\x14\xE1a\x12\rV[a\x14\xED\x86\x82\x87\x01a\x12\xFDV[\x91PP\x92P\x92P\x92V[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a\x15\x10Wa\x15\x10a\x11\xBDV[\x845a\x15\x1B\x81a\x14\x80V[\x93P` \x85\x015\x92P`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x15BWa\x15Ba\x12\rV[\x81\x87\x01\x91P\x87`\x1F\x83\x01\x12a\x15YWa\x15Ya\x12]V[\x815\x81\x81\x11\x15a\x15\xBCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x0ENL/$\r\x8C\xAD\xCC\xEE\x8D`\xAB\x1B`d\x82\x01R`\x84\x81\xFD[\x88` \x82\x85\x01\x01\x11\x15a\x16$W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rjrray stride`\xA8\x1B`d\x82\x01R\x91P`\x84\x82\xFD[\x95\x98\x94\x97PP` \x01\x94PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x16KWa\x16Ka\x11\xBDV[\x83Q\x92P` \x84\x01Q\x91P`@\x84\x01Q\x90P\x92P\x92P\x92V[`\0`\x80\x82\x84\x03\x12\x15a\x16yWa\x16ya\x11\xBDV[`@Q`\x80\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x16\x9CWa\x16\x9Ca\x12\xB6V[\x80`@RP\x82Q\x81R` \x83\x01Q` \x82\x01R`@\x83\x01Q`@\x82\x01R``\x83\x01Qa\x16\xC7\x81a\x14\x80V[``\x82\x01R\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x03\x94Wa\x03\x94a\x16\xD3V[\x80\x82\x01\x80\x82\x11\x15a\x03\x94Wa\x03\x94a\x16\xD3V[`\0`\x01`\xFF\x1B\x82\x01a\x17$Wa\x17$a\x16\xD3V[P`\0\x03\x90V[`\0\x80`\0``\x84\x86\x03\x12\x15a\x17CWa\x17Ca\x11\xBDV[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[`\x04\x81\x10a\x14\x95W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x17|Wa\x17|a\x11\xBDV[\x815a\n\x94\x81a\x17ZV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a\x17\xBDWa\x17\xBDa\x16\xD3V[P\x92\x91PPV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a\x17\xE0Wa\x17\xE0a\x11\xBDV[\x865\x95P` \x87\x015\x94P`@\x87\x015\x93P``\x87\x015\x92P`\x80\x87\x015\x91P`\xA0\x87\x015a\x18\x0E\x81a\x14\x80V[\x80\x91PP\x92\x95P\x92\x95P\x92\x95V[`\0\x80`@\x83\x85\x03\x12\x15a\x182Wa\x182a\x11\xBDV[\x82Qa\x18=\x81a\x17ZV[` \x93\x90\x93\x01Q\x92\x94\x92\x93PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x18eWa\x18ea\x11\xBDV[\x83Qa\x18p\x81a\x17ZV[` \x85\x01Q`@\x90\x95\x01Q\x90\x96\x94\x95P\x93\x92PPPV[`\0\x82a\x18\xA4WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\x01`\xFF\x1B\x82\x14`\0\x19\x84\x14\x16\x15a\x18\xBEWa\x18\xBEa\x16\xD3V[P\x05\x90V[`\0\x80`@\x83\x85\x03\x12\x15a\x18\xD9Wa\x18\xD9a\x11\xBDV[\x82Qa\x18\xE4\x81a\x17ZV[` \x84\x01Q\x90\x92Pa\x18\xF5\x81a\x14\x80V[\x80\x91PP\x92P\x92\x90PV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x03\x94Wa\x03\x94a\x16\xD3V[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15a\x193Wa\x193a\x16\xD3V[\x81\x81\x05\x83\x14\x82\x15\x17a\x03\x94Wa\x03\x94a\x16\xD3V\xFE\xA2dipfsX\"\x12 mv\x02\x86\xE6\xCD\x11Y\x18)\xBF\x12\x94&\xDC\xB1\xCDh}\xB7\xA3\xAF\xFF\xE0`\xE9l\x9C\xAC~\xA1\xB1dsolcC\0\x08\x16\x003";
    /// The bytecode of the contract.
    pub static GEOMETRICMEAN_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\0\xDFW`\x005`\xE0\x1C\x80cs\xCB-\x03\x11a\0\xB3W\x80cs\xCB-\x03\x14a\x02\x90W\x80c\x8A\x04\xBD\xD5\x14a\x02\xCDW\x80c\xAC\xAD)\x89\x14a\x02\xE0W\x80c\xAF\xBA\x13\xC4\x14a\x02\xF5W\x80c\xDC\x17\x83U\x14a\x034Wa\0\xDFV[\x80b.RK\x14a\x01DW\x80c\x06\xFD\xDE\x03\x14a\x01jW\x80c\x1E\xDBq\xE5\x14a\x01\xA3W\x80ch\xBD>8\x14a\x02NW[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x01Wa\x01R6`\x04a\x13\xBEV[a\x03GV[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\x96`@Q\x80`@\x01`@R\x80`\r\x81R` \x01l#\xB2\xB7\xB6\xB2\xBA94\xB1\xA6\xB2\xB0\xB7`\x99\x1B\x81RP\x81V[`@Qa\x01a\x91\x90a\x14QV[a\x02\x06a\x01\xB16`\x04a\x14dV[`\0` \x81\x81R\x91\x81R`@\x90\x81\x90 \x81Q`\x80\x81\x01\x83R\x81T\x81R`\x01\x82\x01T\x93\x81\x01\x93\x90\x93R`\x02\x81\x01T\x91\x83\x01\x91\x90\x91R`\x03\x81\x01T``\x83\x01R`\x04\x81\x01T`\x05\x90\x91\x01T`\x01`\x01`\xA0\x1B\x03\x16\x83V[`@\x80Q\x84Q\x81R` \x80\x86\x01Q\x90\x82\x01R\x84\x82\x01Q\x91\x81\x01\x91\x90\x91R``\x93\x84\x01Q\x93\x81\x01\x93\x90\x93R`\x80\x83\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x16`\xA0\x82\x01R`\xC0\x01a\x01aV[a\x02aa\x02\\6`\x04a\x14\x98V[a\x03\x9AV[`@\x80Q\x96\x15\x15\x87R` \x87\x01\x95\x90\x95R\x93\x85\x01\x92\x90\x92R``\x84\x01R`\x80\x83\x01R`\xA0\x82\x01R`\xC0\x01a\x01aV[a\x02\xA3a\x02\x9E6`\x04a\x14\xF7V[a\x06\tV[`@\x80Q\x95\x15\x15\x86R` \x86\x01\x94\x90\x94R\x92\x84\x01\x91\x90\x91R``\x83\x01R`\x80\x82\x01R`\xA0\x01a\x01aV[a\x02\xA3a\x02\xDB6`\x04a\x14\xF7V[a\x06|V[a\x02\xF3a\x02\xEE6`\x04a\x14\xF7V[a\x06\xCBV[\0[a\x03\x1C\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01aV[a\x01\x96a\x03B6`\x04a\x14dV[a\x08\xFCV[`\0\x80`\0\x80\x84\x80` \x01\x90Q\x81\x01\x90a\x03a\x91\x90a\x163V[\x92P\x92P\x92Pa\x03\x8E\x83\x83\x83a\x03v\x8Aa\x08\xFCV[\x80` \x01\x90Q\x81\x01\x90a\x03\x89\x91\x90a\x16dV[a\n\x1BV[\x93PPPP[\x92\x91PPV[`\0\x80`\0\x80`\0\x80`\0a\x03\xAE\x89a\x08\xFCV[\x80` \x01\x90Q\x81\x01\x90a\x03\xC1\x91\x90a\x16dV[\x90P`\0\x80`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xCE\x15;\xF4\x8D`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04\x16\x91\x81R` \x01\x90V[```@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x04~W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x04\x92W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xB6\x91\x90a\x163V[\x92P\x92P\x92P\x8A\x80` \x01\x90Q\x81\x01\x90a\x04\xD0\x91\x90a\x163V[\x91\x98P\x96P\x94P`\0\x80\x80\x85\x8A\x11\x15a\x05-Wa\x04\xED\x86\x8Ba\x16\xE9V[\x92Pa\x05\x06\x87`@\x01Q\x84a\n\x7F\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x91Pa\x05\x1C\x86a\x05\x16\x84\x87a\n\x7FV[\x90a\n\x9BV[a\x05&\x90\x82a\x16\xFCV[\x90Pa\x05\xCEV[\x84\x89\x11\x15a\x05hWa\x05?\x85\x8Aa\x16\xE9V[\x92Pa\x05X\x87`@\x01Q\x84a\n\x7F\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x91Pa\x05\x1C\x85a\x05\x16\x84\x87a\n\x7FV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`0`$\x82\x01R\x7Finvalid swap: inputs x and y hav`D\x82\x01Roe the same sign!`\x80\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[a\x05\xDA\x8A\x8A\x8A\x8Aa\n\x1BV[\x9BP\x8Ba\x05\xE7`\x14a\x17\x0FV[\x12\x80\x15a\x05\xF4WP`\x14\x8C\x12[\x9CPPPPPPPP\x93\x97P\x93\x97\x91\x95P\x93PV[`\0\x80\x80\x80\x803`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x06XW`@QchS\xCB\xA7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x06c\x88\x88\x88a\n\xB0V[P\x95\x9F\x94\x9EP\x92\x9CP\x90\x9AP\x98P\x90\x96PPPPPPPV[`\0\x80\x80\x80\x80a\x06\x8E\x86\x88\x01\x88a\x17+V[\x91\x94P\x92P\x90Pa\x06\xA4\x83\x83\x83a\x03v\x8Ca\x08\xFCV[\x93P\x83a\x06\xB1`\x14a\x17\x0FV[\x12\x80\x15a\x06\xBEWP`\x14\x84\x12[\x94P\x94P\x94P\x94P\x94P\x94V[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x07\x14W`@QchS\xCB\xA7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x83\x81R` \x81\x90R`@\x90 `\x05\x01T`\x01`\x01`\xA0\x1B\x03\x85\x81\x16\x91\x16\x14a\x07QW`@Qcn\xDA\xEF/`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x07_\x82\x84\x01\x84a\x17gV[\x90P`\x01\x81`\x03\x81\x11\x15a\x07uWa\x07ua\x17\x87V[\x03a\x07\xD0Wa\x07\xB9\x83\x83\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x0Bh\x92PPPV[`\0\x85\x81R` \x81\x90R`@\x90 `\x04\x01Ua\x08\xF5V[`\x02\x81`\x03\x81\x11\x15a\x07\xE4Wa\x07\xE4a\x17\x87V[\x03a\x08PW`\0\x80a\x08+\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x0B\x87\x92PPPV[`\0\x88\x81R` \x81\x90R`@\x90 \x91\x93P\x91Pa\x08I\x90\x83\x83a\x0B\xA9V[PPa\x08\xF5V[`\x03\x81`\x03\x81\x11\x15a\x08dWa\x08da\x17\x87V[\x03a\x08\xDCWa\x08\xA8\x83\x83\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x0C\x15\x92PPPV[`\0\x85\x81R` \x81\x90R`@\x90 `\x05\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90Ua\x08\xF5V[`@Qc#]+=`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPV[``a\t2`@Q\x80`\x80\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81RP\x90V[`\0\x83\x81R` \x81\x81R`@\x91\x82\x90 \x82Q`\x80\x81\x01\x84R\x81T\x81R`\x01\x82\x01T\x92\x81\x01\x92\x90\x92R`\x02\x81\x01T\x92\x82\x01\x92\x90\x92R`\x03\x90\x91\x01T``\x82\x01Ra\tz\x90a\x0C+V[\x80\x82Ra\t\x8F\x90g\r\xE0\xB6\xB3\xA7d\0\0a\x16\xE9V[` \x80\x83\x01\x91\x90\x91R`\0\x84\x81R\x80\x82R`@\x80\x82 `\x04\x81\x01T\x82\x86\x01R\x86\x83R\x91\x83R`\x05\x90\x91\x01T`\x01`\x01`\xA0\x1B\x03\x16``\x84\x01RQa\n\x04\x91\x83\x91\x01\x81Q\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x80\x83\x01Q\x90\x82\x01R``\x91\x82\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x91\x81\x01\x91\x90\x91R`\x80\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91PP\x91\x90PV[\x80Q`\0\x90\x81\x90a\n6\x90a\n0\x88\x87a\x0C\xBCV[\x90a\x0C\xD1V[\x90P`\0a\nU\x84` \x01Qa\n0\x87\x89a\x0C\xBC\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0a\nj\x83\x83a\n\x7FV[a\nt\x91\x90a\x17\x9DV[\x97\x96PPPPPPPV[`\0a\n\x94\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\r\x02V[\x93\x92PPPV[`\0a\n\x94\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\r\x02V[`\0\x80\x80\x80\x80\x80\x80\x80a\n\xC5\x89\x8B\x01\x8Ba\x17\xC4V[\x94\x9AP\x92\x98P\x90\x96P\x94P\x92P\x90Pg\r\xE0\xB6\xB3\xA7d\0\0\x83\x10a\n\xFCW`@Qc\xE8\xA3\x8Aa`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x8B\x81R` \x81\x90R`@\x90 \x83\x81U`\x04\x81\x01\x83\x90U`\x05\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x17\x90Ua\x0B?\x86\x86\x86a\x03v\x8Fa\x08\xFCV[\x96P\x86a\x0BL`\x14a\x17\x0FV[\x12\x80\x15a\x0BYWP`\x14\x87\x12[\x97P\x93\x97P\x93\x97P\x93\x97P\x93\x97V[`\0\x80\x82\x80` \x01\x90Q\x81\x01\x90a\x0B\x7F\x91\x90a\x18\x1CV[\x94\x93PPPPV[`\0\x80\x82\x80` \x01\x90Q\x81\x01\x90a\x0B\x9E\x91\x90a\x18MV[\x90\x95\x90\x94P\x92PPPV[B\x81\x11a\x0B\xC9W`@Qcf\xF1\x02\xED`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0B\xD2\x83a\r0V[`\0a\x0B\xDEB\x83a\x16\xE9V[\x84T\x90\x91P`\0\x90a\x0B\xF0\x90\x85a\x17\x9DV[\x90P`\0a\x0B\xFE\x83\x83a\x18\x87V[`\x01\x87\x01\x94\x90\x94UPPP`\x02\x90\x92\x01\x91\x90\x91UPV[`\0\x81\x80` \x01\x90Q\x81\x01\x90a\n\x94\x91\x90a\x18\xC3V[`\0\x81` \x01Q\x82``\x01Q\x03a\x0CAWPQ\x90V[`\0\x82` \x01QB\x11a\x0CTWBa\x0CZV[\x82` \x01Q[\x90P`\0\x83``\x01Q\x82a\x0Cn\x91\x90a\x16\xE9V[\x90P`\0\x84`@\x01Q\x13\x15a\x0C\x98W`@\x84\x01Qa\x0C\x8C\x90\x82a\x19\0V[\x84Qa\x0B\x7F\x91\x90a\x16\xFCV[\x83`@\x01Qa\x0C\xA6\x90a\x17\x0FV[a\x0C\xB0\x90\x82a\x19\0V[\x84Qa\x0B\x7F\x91\x90a\x16\xE9V[`\0a\n\x94\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\rrV[`\0a\n\x94g\r\xE0\xB6\xB3\xA7d\0\0\x83a\x0C\xE9\x86a\r\x91V[a\x0C\xF3\x91\x90a\x19\x17V[a\x0C\xFD\x91\x90a\x18\x87V[a\x0FlV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\r\x1AW`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[`@\x80Q`\x80\x81\x01\x82R\x82T\x81R`\x01\x83\x01T` \x82\x01R`\x02\x83\x01T\x91\x81\x01\x91\x90\x91R`\x03\x82\x01T``\x82\x01Ra\rg\x90a\x0C+V[\x81UB`\x03\x90\x91\x01UV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\r\x8AW`\0\x80\xFD[\x04\x92\x91PPV[`\0\x80\x82\x13a\r\xCEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x05\xC5V[`\0``a\r\xDB\x84a\x11\x15V[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[`\0h\x02H\xCE6\xA7\x0C\xB2k>\x19\x82\x13a\x0F\x87WP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a\x0F\xCEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x01a\x05\xC5V[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05`\x01`_\x1B\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1Dm\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1Dn\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[`\0\x80\x82\x11a\x11RW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x05\xC5V[P`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x82\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x81\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x12\xF5Wa\x12\xF5a\x12\xB6V[`@R\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a\x13\x11Wa\x13\x11a\x12]V[\x815` g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x13-Wa\x13-a\x12\xB6V[a\x13?`\x1F\x83\x01`\x1F\x19\x16\x82\x01a\x12\xCCV[\x82\x81R\x85\x82\x84\x87\x01\x01\x11\x15a\x13\xA3W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x83\x90R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R`\x84\x81\xFD[\x82\x82\x86\x01\x83\x83\x017`\0\x92\x81\x01\x90\x91\x01\x91\x90\x91R\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x13\xD4Wa\x13\xD4a\x11\xBDV[\x825\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x13\xF5Wa\x13\xF5a\x12\rV[a\x14\x01\x85\x82\x86\x01a\x12\xFDV[\x91PP\x92P\x92\x90PV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a\x141W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\x14\x15V[P`\0` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R`\0a\n\x94` \x83\x01\x84a\x14\x0BV[`\0` \x82\x84\x03\x12\x15a\x14yWa\x14ya\x11\xBDV[P5\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x14\x95W`\0\x80\xFD[PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x14\xB0Wa\x14\xB0a\x11\xBDV[\x835a\x14\xBB\x81a\x14\x80V[\x92P` \x84\x015\x91P`@\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x14\xE1Wa\x14\xE1a\x12\rV[a\x14\xED\x86\x82\x87\x01a\x12\xFDV[\x91PP\x92P\x92P\x92V[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a\x15\x10Wa\x15\x10a\x11\xBDV[\x845a\x15\x1B\x81a\x14\x80V[\x93P` \x85\x015\x92P`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x15BWa\x15Ba\x12\rV[\x81\x87\x01\x91P\x87`\x1F\x83\x01\x12a\x15YWa\x15Ya\x12]V[\x815\x81\x81\x11\x15a\x15\xBCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x0ENL/$\r\x8C\xAD\xCC\xEE\x8D`\xAB\x1B`d\x82\x01R`\x84\x81\xFD[\x88` \x82\x85\x01\x01\x11\x15a\x16$W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rjrray stride`\xA8\x1B`d\x82\x01R\x91P`\x84\x82\xFD[\x95\x98\x94\x97PP` \x01\x94PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x16KWa\x16Ka\x11\xBDV[\x83Q\x92P` \x84\x01Q\x91P`@\x84\x01Q\x90P\x92P\x92P\x92V[`\0`\x80\x82\x84\x03\x12\x15a\x16yWa\x16ya\x11\xBDV[`@Q`\x80\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x16\x9CWa\x16\x9Ca\x12\xB6V[\x80`@RP\x82Q\x81R` \x83\x01Q` \x82\x01R`@\x83\x01Q`@\x82\x01R``\x83\x01Qa\x16\xC7\x81a\x14\x80V[``\x82\x01R\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x03\x94Wa\x03\x94a\x16\xD3V[\x80\x82\x01\x80\x82\x11\x15a\x03\x94Wa\x03\x94a\x16\xD3V[`\0`\x01`\xFF\x1B\x82\x01a\x17$Wa\x17$a\x16\xD3V[P`\0\x03\x90V[`\0\x80`\0``\x84\x86\x03\x12\x15a\x17CWa\x17Ca\x11\xBDV[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[`\x04\x81\x10a\x14\x95W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x17|Wa\x17|a\x11\xBDV[\x815a\n\x94\x81a\x17ZV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a\x17\xBDWa\x17\xBDa\x16\xD3V[P\x92\x91PPV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a\x17\xE0Wa\x17\xE0a\x11\xBDV[\x865\x95P` \x87\x015\x94P`@\x87\x015\x93P``\x87\x015\x92P`\x80\x87\x015\x91P`\xA0\x87\x015a\x18\x0E\x81a\x14\x80V[\x80\x91PP\x92\x95P\x92\x95P\x92\x95V[`\0\x80`@\x83\x85\x03\x12\x15a\x182Wa\x182a\x11\xBDV[\x82Qa\x18=\x81a\x17ZV[` \x93\x90\x93\x01Q\x92\x94\x92\x93PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x18eWa\x18ea\x11\xBDV[\x83Qa\x18p\x81a\x17ZV[` \x85\x01Q`@\x90\x95\x01Q\x90\x96\x94\x95P\x93\x92PPPV[`\0\x82a\x18\xA4WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\x01`\xFF\x1B\x82\x14`\0\x19\x84\x14\x16\x15a\x18\xBEWa\x18\xBEa\x16\xD3V[P\x05\x90V[`\0\x80`@\x83\x85\x03\x12\x15a\x18\xD9Wa\x18\xD9a\x11\xBDV[\x82Qa\x18\xE4\x81a\x17ZV[` \x84\x01Q\x90\x92Pa\x18\xF5\x81a\x14\x80V[\x80\x91PP\x92P\x92\x90PV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x03\x94Wa\x03\x94a\x16\xD3V[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15a\x193Wa\x193a\x16\xD3V[\x81\x81\x05\x83\x14\x82\x15\x17a\x03\x94Wa\x03\x94a\x16\xD3V\xFE\xA2dipfsX\"\x12 mv\x02\x86\xE6\xCD\x11Y\x18)\xBF\x12\x94&\xDC\xB1\xCDh}\xB7\xA3\xAF\xFF\xE0`\xE9l\x9C\xAC~\xA1\xB1dsolcC\0\x08\x16\x003";
    /// The deployed bytecode of the contract.
    pub static GEOMETRICMEAN_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct GeometricMean<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for GeometricMean<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for GeometricMean<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for GeometricMean<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for GeometricMean<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(GeometricMean))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> GeometricMean<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    GEOMETRICMEAN_ABI.clone(),
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
                GEOMETRICMEAN_ABI.clone(),
                GEOMETRICMEAN_BYTECODE.clone().into(),
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
        ///Calls the contract's `name` (0x06fdde03) function
        pub fn name(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([6, 253, 222, 3], ())
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
    for GeometricMean<M> {
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
    pub enum GeometricMeanErrors {
        InvalidSender(InvalidSender),
        InvalidUpdateCode(InvalidUpdateCode),
        InvalidUpdateEnd(InvalidUpdateEnd),
        InvalidWeightX(InvalidWeightX),
        NotDFMM(NotDFMM),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for GeometricMeanErrors {
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
    impl ::ethers::core::abi::AbiEncode for GeometricMeanErrors {
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
    impl ::ethers::contract::ContractRevert for GeometricMeanErrors {
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
    impl ::core::fmt::Display for GeometricMeanErrors {
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
    impl ::core::convert::From<::std::string::String> for GeometricMeanErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<InvalidSender> for GeometricMeanErrors {
        fn from(value: InvalidSender) -> Self {
            Self::InvalidSender(value)
        }
    }
    impl ::core::convert::From<InvalidUpdateCode> for GeometricMeanErrors {
        fn from(value: InvalidUpdateCode) -> Self {
            Self::InvalidUpdateCode(value)
        }
    }
    impl ::core::convert::From<InvalidUpdateEnd> for GeometricMeanErrors {
        fn from(value: InvalidUpdateEnd) -> Self {
            Self::InvalidUpdateEnd(value)
        }
    }
    impl ::core::convert::From<InvalidWeightX> for GeometricMeanErrors {
        fn from(value: InvalidWeightX) -> Self {
            Self::InvalidWeightX(value)
        }
    }
    impl ::core::convert::From<NotDFMM> for GeometricMeanErrors {
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
    ///Container type for all input parameters for the `name` function with signature `name()` and selector `0x06fdde03`
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
    #[ethcall(name = "name", abi = "name()")]
    pub struct NameCall;
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
    pub enum GeometricMeanCalls {
        ComputeSwapConstant(ComputeSwapConstantCall),
        Dfmm(DfmmCall),
        GetPoolParams(GetPoolParamsCall),
        Init(InitCall),
        InternalParams(InternalParamsCall),
        Name(NameCall),
        Update(UpdateCall),
        ValidateAllocateOrDeallocate(ValidateAllocateOrDeallocateCall),
        ValidateSwap(ValidateSwapCall),
    }
    impl ::ethers::core::abi::AbiDecode for GeometricMeanCalls {
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
            if let Ok(decoded) = <NameCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Name(decoded));
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
    impl ::ethers::core::abi::AbiEncode for GeometricMeanCalls {
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
                Self::Name(element) => ::ethers::core::abi::AbiEncode::encode(element),
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
    impl ::core::fmt::Display for GeometricMeanCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ComputeSwapConstant(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Dfmm(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPoolParams(element) => ::core::fmt::Display::fmt(element, f),
                Self::Init(element) => ::core::fmt::Display::fmt(element, f),
                Self::InternalParams(element) => ::core::fmt::Display::fmt(element, f),
                Self::Name(element) => ::core::fmt::Display::fmt(element, f),
                Self::Update(element) => ::core::fmt::Display::fmt(element, f),
                Self::ValidateAllocateOrDeallocate(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ValidateSwap(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ComputeSwapConstantCall> for GeometricMeanCalls {
        fn from(value: ComputeSwapConstantCall) -> Self {
            Self::ComputeSwapConstant(value)
        }
    }
    impl ::core::convert::From<DfmmCall> for GeometricMeanCalls {
        fn from(value: DfmmCall) -> Self {
            Self::Dfmm(value)
        }
    }
    impl ::core::convert::From<GetPoolParamsCall> for GeometricMeanCalls {
        fn from(value: GetPoolParamsCall) -> Self {
            Self::GetPoolParams(value)
        }
    }
    impl ::core::convert::From<InitCall> for GeometricMeanCalls {
        fn from(value: InitCall) -> Self {
            Self::Init(value)
        }
    }
    impl ::core::convert::From<InternalParamsCall> for GeometricMeanCalls {
        fn from(value: InternalParamsCall) -> Self {
            Self::InternalParams(value)
        }
    }
    impl ::core::convert::From<NameCall> for GeometricMeanCalls {
        fn from(value: NameCall) -> Self {
            Self::Name(value)
        }
    }
    impl ::core::convert::From<UpdateCall> for GeometricMeanCalls {
        fn from(value: UpdateCall) -> Self {
            Self::Update(value)
        }
    }
    impl ::core::convert::From<ValidateAllocateOrDeallocateCall> for GeometricMeanCalls {
        fn from(value: ValidateAllocateOrDeallocateCall) -> Self {
            Self::ValidateAllocateOrDeallocate(value)
        }
    }
    impl ::core::convert::From<ValidateSwapCall> for GeometricMeanCalls {
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
    ///Container type for all return fields from the `name` function with signature `name()` and selector `0x06fdde03`
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
    pub struct NameReturn(pub ::std::string::String);
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
