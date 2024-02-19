pub use log_normal::*;
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
pub mod log_normal {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                    name: ::std::borrow::ToOwned::to_owned("dfmm_"),
                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                    internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                        "address"
                    ),),
                },],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("computeSwapConstant"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("computeSwapConstant",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("poolId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
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
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("int256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("dfmm"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("dfmm"),
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
                    ::std::borrow::ToOwned::to_owned("getPoolParams"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getPoolParams"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("poolId"),
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
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("init"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
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
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
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
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("reserveY"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("totalLiquidity"),
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
                    ::std::borrow::ToOwned::to_owned("internalParams"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("internalParams"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("sigma"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct DynamicParam"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("tau"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct DynamicParam"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("strike"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct DynamicParam"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("swapFee"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
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
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("update"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
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
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
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
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("validateAllocateOrDeallocate"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("validateAllocateOrDeallocate",),
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
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
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
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("reserveY"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("totalLiquidity"),
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
                    ::std::borrow::ToOwned::to_owned("validateSwap"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
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
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
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
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("nextRy"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("nextL"),
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
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("Infinity"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("Infinity"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidSender"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InvalidSender"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidUpdateCode"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InvalidUpdateCode"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidUpdateEnd"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InvalidUpdateEnd"),
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
                    ::std::borrow::ToOwned::to_owned("NotDFMM"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("NotDFMM"),
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
    pub static LOGNORMAL_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80\x80`@R4a\0\xE6W`@Q`\x1Fa\x1FT8\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x91`\x01`\x01`@\x1B\x03\x83\x11\x84\x84\x10\x17a\0\xD0W\x80\x84\x92` \x94`@R\x839\x81\x01\x03\x12a\0\x80WQ`\x01`\x01`\xA0\x1B\x03\x81\x16\x90\x81\x90\x03a\0{W`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x91\x90\x91\x17\x90U`@Qa\x1E \x90\x81a\x014\x829\xF3[`\0\x80\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[bF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01Ra7\xB7`\xF1\x1B`d\x82\x01R`\x84\x90\xFD\xFE`\x80`@R`\x046\x10a\x0B\xF7W`\x005`\xE0\x1C\x80b.RK\x14a\0\x8BW\x80c\x1E\xDBq\xE5\x14a\0\x86W\x80ch\xBD>8\x14a\0\x81W\x80cs\xCB-\x03\x14a\0|W\x80c\x8A\x04\xBD\xD5\x14a\0wW\x80c\xAC\xAD)\x89\x14a\0rW\x80c\xAF\xBA\x13\xC4\x14a\0mWc\xDC\x17\x83U\x03a\x0B\xF7Wa\x0B\x8FV[a\x0BfV[a\t\xBFV[a\t\x85V[a\x08\x1FV[a\x04\xCCV[a\x03\xC7V[4a\x01\x07W`@6`\x03\x19\x01\x12a\x01\x02W`$5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\0\xFDWa\0\xF5a\0\xD2a\0\xC4` \x936\x90`\x04\x01a\x03aV[\x83\x80\x82Q\x83\x01\x01\x91\x01a\x0CZV[\x90a\0\xEFa\0\xE1`\x045a\x0EyV[\x86\x80\x82Q\x83\x01\x01\x91\x01a\x0C\xC6V[\x92a\x10tV[`@Q\x90\x81R\xF3[a\x01\xA7V[a\x01WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01Ra7\xB7`\xF1\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x90\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\xA0\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x02\x82W`@RV[a\x02PV[`\xC0\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x02\x82W`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x02\x82W`@RV[\x92\x91\x92g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x02\x82W`@Q\x91a\x02\xEF`\x1F\x82\x01`\x1F\x19\x16` \x01\x84a\x02\xA3V[\x82\x94\x81\x84R\x81\x83\x01\x11a\x03\x0CW\x82\x81` \x93\x84`\0\x96\x017\x01\x01RV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R`\x84\x90\xFD[\x90\x80`\x1F\x83\x01\x12\x15a\x03\x7FW\x81` a\x03|\x935\x91\x01a\x02\xC5V[\x90V[a\x01\xF7V[\x90`@Q`\x80\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x02\x82W`@R```\x03\x82\x94\x80T\x84R`\x01\x81\x01T` \x85\x01R`\x02\x81\x01T`@\x85\x01R\x01T\x91\x01RV[4a\x04\xB1W` 6`\x03\x19\x01\x12a\x01\x02W`\x045`\0R`\x01` Ra\x01\xC0`@`\0 a\x03\xF4\x81a\x03\x84V[\x90a\x04\x01`\x04\x82\x01a\x03\x84V[\x90a\x04\xA3a\x04\x11`\x08\x83\x01a\x03\x84V[a\x04y`\x0C\x84\x01T\x93`\r`\x01\x80`\xA0\x1B\x03\x91\x01T\x16\x94a\x04S`@Q\x80\x98``\x80\x91\x80Q\x84R` \x81\x01Q` \x85\x01R`@\x81\x01Q`@\x85\x01R\x01Q\x91\x01RV[\x80Q`\x80\x88\x01R` \x81\x01Q`\xA0\x88\x01R`@\x81\x01Q`\xC0\x88\x01R``\x01Q`\xE0\x87\x01RV[\x80Qa\x01\0\x86\x01R` \x81\x01Qa\x01 \x86\x01R`@\x81\x01Qa\x01@\x86\x01R``\x01Qa\x01`\x85\x01RV[a\x01\x80\x83\x01Ra\x01\xA0\x82\x01R\xF3[a\x01\x07V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\x04\xC7WV[`\0\x80\xFD[4a\x04\xB1W``\x80`\x03\x196\x01\x12a\x01\x02Wa\x04\xE9`\x045a\x04\xB6V[`$5\x90`D5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\0\xFDWa\x05\r\x906\x90`\x04\x01a\x03aV[\x90a\x05)a\x05\x1A\x84a\x0EyV[` \x80\x82Q\x83\x01\x01\x91\x01a\x0C\xC6V[`\0T\x90\x92\x90a\x05O\x90a\x05C\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x80;\x15a\x07\rW`@Qc3\x85N\xFD`\xE2\x1B\x81R`\x04\x81\x01\x95\x90\x95R\x82\x90\x85\x90`$\x90\x82\x90Z\xFA\x80\x15a\x07\x08W`\0\x91\x82\x95\x83\x92a\x06\xCDW[P\x80` \x80a\x05\x9C\x93Q\x83\x01\x01\x91\x01a\x0CZV[\x93\x91\x94\x90\x96\x82\x86\x11`\0\x14a\x06@WP\x91a\x05\xDD\x82a\x05\xD8\x83a\x05\xD3a\x05\xEE\x97a\x05\xC9a\x05\xE5\x98\x8Ca\r\x8AV[\x90\x8C\x01Q\x90a\x15\x9BV[a\x15\x9BV[a\x15\xC7V[P[\x83a\r\xE5V[\x93\x82\x86\x85a\x10tV[\x93\x84`\x13\x19\x12\x92\x83a\x065W[a\x061\x93\x94`@Q\x96\x87\x96\x87\x92`\xA0\x94\x91\x97\x96\x95\x92`\xC0\x85\x01\x98\x15\x15\x85R` \x85\x01R`@\x84\x01R``\x83\x01R`\x80\x82\x01R\x01RV[\x03\x90\xF3[`\x14\x86\x12\x93Pa\x05\xFBV[\x90\x92\x90\x91P\x81\x87\x11\x15a\x06oWa\x06i\x82a\x05\xD8\x83a\x05\xD3a\x05\xEE\x97a\x05\xC9a\x05\xE5\x98\x8Ea\r\x8AV[Pa\x05\xDFV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`0`$\x82\x01R\x7Finvalid swap: inputs x and y hav`D\x82\x01Roe the same sign!`\x80\x1B`d\x82\x01R`\x84\x90\xFD[\x90\x95Pa\x05\x9C\x92Pa\x06\xF5\x91P\x83=\x85\x11a\x07\x01W[a\x06\xED\x81\x83a\x02\xA3V[\x81\x01\x90a\x0CZV[\x95\x91\x92\x90\x95\x91\x90a\x05\x88V[P=a\x06\xE3V[a\rhV[a\r\x15V[\x90```\x03\x19\x83\x01\x12a\x01\x02W`\x045a\x07+\x81a\x04\xB6V[\x91`$5\x91`D5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x83\x82\x11a\0\xFDW\x80`#\x83\x01\x12\x15a\x03\x7FW\x81`\x04\x015\x93\x84\x11a\x07\xC6W`$\x84\x83\x01\x01\x11a\x07mW`$\x01\x91\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rjrray stride`\xA8\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x0ENL/$\r\x8C\xAD\xCC\xEE\x8D`\xAB\x1B`d\x82\x01R`\x84\x90\xFD[4a\x04\xB1Wa\x08-6a\x07\x12V[`\0T\x92\x93P\x91`\x01`\x01`\xA0\x1B\x03\x163\x03a\tsWa\t1a\t)a\x05\x1Aa\t$\x95a\x08k`\x80\x95a\x08\xEE\x97a\x08ba\x0ENV[P\x81\x01\x90a\x11sV[` \x81\x9B\x94\x9A\x93\x99\x92\x9B\x01Qa\x08\x8B\x86`\0R`\x01` R`@`\0 \x90V[U`@\x81\x01Q`\x04a\x08\xA7\x87`\0R`\x01` R`@`\0 \x90V[\x01U\x80Q`\x08a\x08\xC1\x87`\0R`\x01` R`@`\0 \x90V[\x01U``\x81\x01Q`\x0Ca\x08\xDE\x87`\0R`\x01` R`@`\0 \x90V[\x01U\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\ra\t\x04\x84`\0R`\x01` R`@`\0 \x90V[\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90UV[a\x0EyV[\x84\x83\x85a\x10tV[\x92\x83`\x13\x19\x12\x91\x82a\thW[`@\x80Q\x93\x15\x15\x84R` \x84\x01\x95\x90\x95R\x93\x82\x01\x92\x90\x92R``\x81\x01\x92\x90\x92R`\x80\x82\x01R`\xA0\x90\xF3[`\x14\x85\x12\x92Pa\t>V[`@QchS\xCB\xA7`\xE0\x1B\x81R`\x04\x90\xFD[4a\x04\xB1W``a\t\x956a\x07\x12V[\x81\x80\x94P\x94\x92\x94\x01\x03\x12a\x01\x02W\x805\x90a\t1a\t)a\x05\x1A`@` \x85\x015\x94\x015\x95a\x0EyV[4a\x04\xB1Wa\t\xCD6a\x07\x12V[`\0T\x90\x93\x90`\x01`\x01`\xA0\x1B\x03\x90\x81\x163\x03a\tsWa\n\x0Ea\x05C`\ra\n\0\x87`\0R`\x01` R`@`\0 \x90V[\x01T`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x91\x16\x03a\x0BTWa\n!\x83\x82\x01\x82a\x0E\x19V[a\n*\x81a\x0E.V[`\x01\x81\x03a\ncWPa\nMa\nHa\n^\x92`\x0C\x94\x956\x91a\x02\xC5V[a\x12\xAFV[\x92`\0R`\x01` R`@`\0 \x90V[\x01U[\0[a\nl\x81a\x0E.V[`\x03\x81\x03a\n\xA8WP\x90a\n\x90a\n\x8Ba\n\xA3\x93a\na\x956\x91a\x02\xC5V[a\x12\x05V[\x92\x90\x91`\0R`\x01` R`@`\0 \x90V[a\x12-V[a\n\xB1\x81a\x0E.V[`\x04\x81\x03a\n\xEBWP\x90`\x04a\n\xD2a\n\x8Ba\n\xE5\x94a\na\x966\x91a\x02\xC5V[\x93\x90\x92`\0R`\x01` R`@`\0 \x90V[\x01a\x12-V[a\n\xF4\x81a\x0E.V[`\x02\x81\x03a\x0B\x15WP\x90`\x08a\n\xD2a\n\x8Ba\n\xE5\x94a\na\x966\x91a\x02\xC5V[\x80a\x0B!`\x05\x92a\x0E.V[\x03a\x0BBWa\t\x04a\nMa\x0B=`\r\x93a\na\x966\x91a\x02\xC5V[a\x11\xDDV[`@Qc#]+=`\xE0\x1B\x81R`\x04\x90\xFD[`@Qcn\xDA\xEF/`\xE1\x1B\x81R`\x04\x90\xFD[4a\x04\xB1W`\x006`\x03\x19\x01\x12a\x01\x02W`\0T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x04\xB1W` \x80`\x03\x196\x01\x12a\x01\x02Wa\x0B\xAC`\x045a\x0EyV[\x90`@\x80Q\x91` \x83R\x83Q\x91\x82` \x85\x01R`\0[\x83\x81\x10a\x0B\xE4W\x84`@\x81\x86`\0\x83\x82\x84\x01\x01R`\x1F\x80\x19\x91\x01\x16\x81\x01\x03\x01\x90\xF3[\x85\x81\x01\x83\x01Q\x85\x82\x01\x83\x01R\x82\x01a\x0B\xC2V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01Rtnor receive functions`X\x1B`d\x82\x01R`\x84\x90\xFD[\x90\x81``\x91\x03\x12a\x01\x02W\x80Q\x91`@` \x83\x01Q\x92\x01Q\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FABI decoding: struct data too sh`D\x82\x01Rb\x1B\xDC\x9D`\xEA\x1B`d\x82\x01R`\x84\x90\xFD[\x90\x81`\xA0\x91\x03\x12a\x01\x02W`\x80`@Q\x91a\x0C\xE0\x83a\x02fV[\x80Q\x83R` \x81\x01Q` \x84\x01R`@\x81\x01Q`@\x84\x01R``\x81\x01Q``\x84\x01R\x01Qa\r\r\x81a\x04\xB6V[`\x80\x82\x01R\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01Rd code`\xD8\x1B`d\x82\x01R`\x84\x90\xFD[`@Q=`\0\x82>=\x90\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x91\x90\x82\x03\x91\x82\x11a\r\x97WV[a\rtV[\x91\x90\x82\x01\x80\x92\x11a\r\x97WV[\x90g\x1B\xC1mgN\xC8\0\0`\0\x83\x82\x03\x93\x12\x81\x84\x12\x81\x16\x91\x84\x13\x90\x15\x16\x17a\r\x97WV[\x90g\x11\x90\0\xAB\x10\x0F\xFB\xFF\x19\x82\x01\x91\x82\x13`\x01\x16a\r\x97WV[\x81\x81\x03\x92\x91`\0\x13\x80\x15\x82\x85\x13\x16\x91\x84\x12\x16\x17a\r\x97WV[`\x01`\xFF\x1B\x81\x14a\r\x97W`\0\x03\x90V[`\x06\x11\x15a\x04\xC7WV[\x90\x81` \x91\x03\x12a\x01\x02W5a\x03|\x81a\x0E\x0FV[`\x06\x11\x15a\x0E8WV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`@Q\x90a\x0E[\x82a\x02fV[`\0`\x80\x83\x82\x81R\x82` \x82\x01R\x82`@\x82\x01R\x82``\x82\x01R\x01RV[a\x0E\x81a\x0ENV[\x81`\0R`\x01` Ra\x0E\x9Fa\x0E\x9A`@`\0 a\x03\x84V[a\x12\xE4V[\x91` \x82\x01\x92\x83R\x80`\0R`\x01` Ra\x0E\xC3a\x0E\x9A`\x08`@`\0 \x01a\x03\x84V[\x82R`\x0Ca\x0F\x04a\x0E\xECa\x0E\x9A`\x04a\x0E\xE6\x86`\0R`\x01` R`@`\0 \x90V[\x01a\x03\x84V[\x92`@\x85\x01\x93\x84R`\0R`\x01` R`@`\0 \x90V[\x01T\x90``\x83\x01\x91\x82R`@Q\x93\x83Q` \x86\x01RQ`@\x85\x01RQ``\x84\x01RQ`\x80\x83\x01R`\x80`\x01\x80`\xA0\x1B\x03\x91\x01Q\x16`\xA0\x82\x01R`\xA0\x81Ra\x03|\x81a\x02\x87V[\x90\x81g \x05\xFEO&\x8E\xA0\0\x01\x91\x82\x12`\x01\x16a\r\x97WV[\x90\x81g\r\xC5R\x7Fd, \0\x01\x91\x82\x12`\x01\x16a\r\x97WV[\x90\x81g\r\xE0\xB6\xB3\xA7d\0\0\x01\x91\x82\x12`\x01\x16a\r\x97WV[\x90\x81g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19\x01\x91\x82\x12\x15`\x01\x16a\r\x97WV[\x90\x81g\x14\xA8EL\x19\xE1\xAC\0\x01\x91\x82\x12`\x01\x16a\r\x97WV[\x90\x81g\x0F\xC1\x0E\x01W\x82w\xFF\x19\x01\x91\x82\x12\x15`\x01\x16a\r\x97WV[\x90\x81g\x03\xDE\xBD\x08;\x8C|\0\x01\x91\x82\x12`\x01\x16a\r\x97WV[\x90\x81g\x02\x95\xD4\0\xEA2W\xFF\x19\x01\x91\x82\x12\x15`\x01\x16a\r\x97WV[\x90\x81g\x01W\xD8\xB2\xEC\xC7\x08\0\x01\x91\x82\x12`\x01\x16a\r\x97WV[\x90\x81g\x051\n\xA7\xD5!0\0\x01\x91\x82\x12`\x01\x16a\r\x97WV[\x90\x81g\r\xE0\xCC=\x15a\0\0\x01\x91\x82\x12`\x01\x16a\r\x97WV[\x91\x90\x91`\0\x83\x82\x01\x93\x84\x12\x91\x12\x90\x80\x15\x82\x16\x91\x15\x16\x17a\r\x97WV[\x90\x92\x82\x82\x10\x15a\x11.Wa\x03|\x93a\x10\xF1\x92\x84g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82a\x10\x9C\x83\x83a\x15\xF7V[\x10a\x11\x1BWP`\x01`\x01`\xFF\x1B\x03\x95\x90P[\x83Q\x91a\x10\xC4a\x10\xBE\x83\x85a\x16\x19V[\x85a\x15\xF7V[\x10a\x10\xF6WP`\x01`\x01`\xFF\x1B\x03\x92a\x10\xEB\x92P\x90P[`@` \x82\x01Q\x91\x01Q\x90a\x15rV[\x92a\x10XV[a\x10XV[a\x10\xEB\x92a\x11\na\x11\x10\x92a\x11\x15\x94a\x16\x19V[\x90a\x15\xF7V[a\x13\x9FV[\x91a\x10\xDBV[a\x11(\x91a\x11\x10\x91a\x15\xF7V[\x94a\x10\xAEV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FtradingFunction: invalid x\0\0\0\0\0\0`D\x82\x01R`d\x90\xFD[\x80\x91\x03a\x01\0\x81\x12a\x01\x02W\x815\x92` \x83\x015\x92`\xA0`@\x82\x015\x93`_\x19\x01\x12a\x11\xD8W`\xE0`@Q\x91a\x11\xA8\x83a\x02fV[``\x81\x015\x83R`\x80\x81\x015` \x84\x01R`\xA0\x81\x015`@\x84\x01R`\xC0\x81\x015``\x84\x01R\x015a\r\r\x81a\x04\xB6V[a\x0CuV[`@\x81\x80Q\x81\x01\x03\x12a\x01\x02W\x80a\x11\xFA` `@\x93\x01Qa\x0E\x0FV[\x01Qa\x05C\x81a\x04\xB6V[``\x81\x80Q\x81\x01\x03\x12a\x01\x02Wa\x12\x1F` \x82\x01Qa\x0E\x0FV[```@\x82\x01Q\x91\x01Q\x90\x91V[\x91\x90B\x82\x11\x15a\x12\x9DWa\x12Ca\x0E\x9A\x84a\x03\x84V[\x90\x81\x84UB`\x03\x85\x01UB\x83\x03\x91\x83\x83\x11a\r\x97Wa\x12a\x91a\r\xE5V[B\x83\x14a\x12\x87W`\x01`\xFF\x1B\x81\x14`\0\x19\x83\x14\x16a\r\x97W`\x02\x92`\x01\x85\x01U\x05\x91\x01UV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`@Qcf\xF1\x02\xED`\xE1\x1B\x81R`\x04\x90\xFD[`@\x81\x80Q\x81\x01\x03\x12a\x01\x02W\x80a\x12\xCC` `@\x93\x01Qa\x0E\x0FV[\x01Q\x90V[\x81\x81\x02\x92\x91\x81\x15\x91\x84\x04\x14\x17\x15a\r\x97WV[``\x81\x01Q` \x82\x01Q\x80\x82\x14a\x13_W\x80B\x11`\0\x14a\x13WW\x90[\x81\x03\x90\x81\x11a\r\x97W`@\x82\x01\x90\x81Q`\0\x81\x13`\0\x14a\x134WPa\x13.\x90a\x03|\x93Q\x92Q\x90a\x12\xD1V[\x90a\r\x9CV[\x92a\x13K\x92Pa\x13E\x90Q\x93a\r\xFEV[\x90a\x12\xD1V[\x81\x03\x90\x81\x11a\r\x97W\x90V[PB\x90a\x13\x01V[PPQ\x90V[\x90c;\x9A\xCA\0\x91\x82\x81\x02\x92\x81\x84\x05\x14\x90\x15\x17\x15a\r\x97WV[`\0\x81\x90\x03\x91\x90`\x01`\xFF\x1B\x81\x14`\x01\x16a\r\x97W`\0\x19\x83\x05\x03a\r\x97WV[g\x06\xF0[Y\xD3\xB2\0\0\x81\x14a\x15lWg\r\xE0\xB6\xB3\xA7d\0\0\x80\x82\x12\x15a\x15\x16W\x81\x15a\x157W`\x01\x82`\x01\x1B\x91`\x02\x93\x83\x05`\x02\x03a\r\x97W`\0\x83\x12\x80\x15a\x15[W[a\x15IW\x82\x15a\x15\x16Wg\x1B\xC1mgN\xC8\0\0\x83\x14a\x157W\x82\x12\x91\x82\x15a\x15(W\x92[a\x14\x10\x84a\x1B\xC7V[\x80\x15a\x15\x16Wa\x14\x82a\x14Aa\x14<a\x147a\x142a\x14\x87\x95\x99\x97\x96\x99a\x17[V[a\x1C\x88V[a\x16eV[a\x13eV[a\x14}a\x14Ua\x14P\x83a\x1B\xF2V[a\x0FJV[a\x14wa\x14ra\x14la\x14g\x86a\x1C\x1DV[a\x0FbV[\x85a\x1C\xFFV[a\x0FzV[\x90a\x1CfV[a\r\xE5V[a\x1C\xB0V[\x93`\0\x92[\x81\x84\x10a\x14\xBEWPPPPa\x03|\x91a\x14\xAB\x91`\0\x14a\x14\xB0Wa\x1B\xA0V[a\r\xFEV[a\x14\xB9\x90a\r\xFEV[a\x1B\xA0V[\x90\x91a\x15\x0C\x86a\x15\x06a\x14\xD6\x85a\x14}\x86\x99\x9Ba\x18\xFCV[a\x14wa\x14\xF6a\x14\xF1a\x14\xECa\x14\xAB\x87\x80a\x1C\xFFV[a\x1A\x1CV[a\x1C\xD8V[a\x15\0\x83\x86a\x1C\xFFV[\x90a\r\xE5V[\x90a\x10XV[\x95\x01\x92\x91\x90a\x14\x8CV[`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x90\xFD[a\x151\x90a\r\xA9V[\x92a\x14\x07V[`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x90\xFD[`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x90\xFD[Pg\x1B\xC1mgN\xC8\0\0\x83\x13a\x13\xE3V[P`\0\x90V[\x90a\x15|\x90a\x16eV[c;\x9A\xCA\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17\x15a\r\x97Wa\x03|\x91a\x16\x19V[\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x04\xC7W`\x01g\r\xE0\xB6\xB3\xA7d\0\0`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[\x90g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x81\x02\x92\x81\x84\x04\x14\x90\x15\x17\x81\x15\x15\x16\x15a\x04\xC7W`\x01\x90`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17\x82\x15\x15\x16\x15a\x04\xC7W\x04\x90V[\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x04\xC7Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x04\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x04\xC7Wg\x1B\xC1mgN\xC8\0\0\x90\x04\x90V[`\xB5\x81`\x01`\x88\x1B\x81\x10\x15a\x17\x0CW[\x80i\x01\0\0\0\0\0\0\0\0\0b\x01\0\0\x92\x10\x15a\x16\xFFW[e\x01\0\0\0\0\0\x81\x10\x15a\x16\xF2W[c\x01\0\0\0\x81\x10\x15a\x16\xE5W[\x01\x02`\x12\x1C`\x01\x90\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x80\x80\x92\x04\x10\x90\x03\x90V[`\x10\x1C\x91`\x08\x1B\x91a\x16\xA9V[` \x1C\x91`\x10\x1B\x91a\x16\x9CV[`@\x1C\x91` \x1B\x91a\x16\x8DV[Ph\xB5\0\0\0\0\0\0\0\0\x90P`\x80\x82\x90\x1Ca\x16uV[\x15a\x17*WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x90\xFD[}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x90a\x17\x87`\0\x82\x13a\x17#V[q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06a\x17\xA3\x82a\x1DKV[``\x92\x83\x82`\x9F\x03\x01\x1B`\x9F\x1C\x90`_\x19\x01}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x02\x92l\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x82m\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x81m\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x81m\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x81m\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x81m\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x81lFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x02\x8C\x1D\x01\x02\x8A\x1D\x01\x02\x88\x1D\x01\x02\x86\x1D\x01\x02\x84\x1D\x01\x02\x82\x1D\x01\x91x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x91l\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x90\x82m\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x81l\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x81m\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x81m\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x81l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x81\x01\x02\x89\x1D\x01\x02\x87\x1D\x01\x02\x85\x1D\x01\x02\x83\x1D\x01\x02\x90\x1D\x01\x02\x01\x05\x02\x01\x01`\xAE\x1D\x90V[\x80\x15a\x1A\x0FWgV\x98\xEE\xF0fp\0\0\x81\x12\x15a\x15lWgV\x98\xEE\xF0fo\xFF\xFF\x19\x81\x13\x15a\x1A\x02W`\0a\x19\xF2a\x191\x83a\x1D\xBDV[a\x19\xBAa\x14\xECa\x19Ka\x19Fa\x14r\x85a\x16:V[a\x1CGV[\x92a\x10\xF1a\x19\xEDa\x19\xE8a\x19\xE1a\x19\xDBa\x19\xD6a\x19\xD0a\x19\xCBa\x19\xC5a\x19\xC0\x8Da\x19\xBAa\x19\xB5a\x19\xAFa\x19\xAAa\x14la\x19\xA5a\x19\x9Fa\x19\x9Aa\x19\x94a\x19\x8F\x8Aa\x1D V[a\x0F\x92V[\x89a\x1C\xFFV[a\x0F\xACV[\x87a\x1C\xFFV[a\x0F\xC4V[a\x0F\xDEV[\x83a\x1C\xFFV[a\x0F\xF6V[\x90a\x1C\xFFV[a\x10\x10V[\x8Ca\x1C\xFFV[a\x10(V[\x8Aa\x1C\xFFV[a\x10@V[\x88a\x1C\xFFV[\x93\x80a\x1C\xFFV[a\x13~V[a\r\xCCV[\x91\x12\x15a\x03|Wa\x03|\x90a\r\xA9V[Pg\x1B\xC1mgN\xC8\0\0\x90V[Pg\r\xE0\xB6\xB3\xA7d\0\0\x90V[h\x02H\xCE6\xA7\x0C\xB2k>\x19\x81\x13\x15a\x15lWh\x07U\xBFy\x8BJ\x1B\xF1\xE5\x81\x12\x15a\x1BlWe\x03x-\xAC\xE9\xD9\x90`N\x1B\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x91``\x90`\x01`_\x1B\x84\x82\x84\x1B\x05\x01\x82\x1D\x93\x84\x02\x90\x03\x80l\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x81\x01\x02\x82\x1D\x90n\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dPn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x82n\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x81m\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x81m\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x81m\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x81l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x02\x8D\x1D\x01\x02\x8B\x1D\x01\x02\x89\x1D\x01\x02\x87\x1D\x01\x02\x85\x1D\x01\x93m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x93m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEA\x81\x01\x90\x84m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x91\x01\x01\x02\x90\x1D\x01\x02\x01\x05\x02\x90`\xC3\x03\x1C\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x90\xFD[g\x13\xA0K\xBD\xFD\xC9\xBE\x88\x90\x80\x82\x02\x91\x82\x05\x14`\x01\x16\x15a\x04\xC7Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x04\xC7Wg\x1B\xC1mgN\xC8\0\0\x90\x05\x90V[g\x03\xC1f\\z\xAB \0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x04\xC7Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[f\x9F2u$b\xA0\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x04\xC7Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[`\x01\x81\x15\x15\x16\x15a\x04\xC7Wn\xC0\x97\xCE{\xC9\x07\x15\xB3K\x9F\x10\0\0\0\0\x05\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17\x82\x15\x15\x16\x15a\x04\xC7W\x05\x90V[g\x1B\xC1mgN\xC7\xFF\xFF\x19\x81\x81\x02\x91`\x01\x91\x83\x05\x14\x16\x15a\x04\xC7Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\t\xD0(\xCCo _\xFF\x19\x81\x81\x02\x91`\x01\x91\x83\x05\x14\x16\x15a\x04\xC7Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x90\x80\x82\x02\x91\x82\x05\x14`\x01\x16\x15a\x04\xC7Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x04\xC7Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\x02_\x0F\xE1\x05\xA3\x14\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x04\xC7Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[a\x1DV\x81\x15\x15a\x17#V[\x80o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x07\x1B\x81\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x81\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x81\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x81\x81\x1C`\xFF\x10`\x03\x1B\x17\x81\x81\x1C`\x0F\x10`\x02\x1B\x17\x81\x81\x1C`\x03\x10`\x01\x1B\x17\x90\x81\x1C`\x01\x10\x17\x90V[`\x01`\xFF\x1B\x81\x14a\x1D\xD8W`\0\x81\x12\x15a\x03|W\x19`\x01\x01\x90V[`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x90\xFD\xFE\xA2dipfsX\"\x12 \xEC6\x0C\xDB\x88T\xB6\x16\x94\x9BZ\xE2\x06 i\xDF$I\xE5\xEF\xA1\xB1\xA1\xF0\x1F\xFAP\xF0\x94\x9B\x80\x85dsolcC\0\x08\x16\x003";
    /// The bytecode of the contract.
    pub static LOGNORMAL_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\x0B\xF7W`\x005`\xE0\x1C\x80b.RK\x14a\0\x8BW\x80c\x1E\xDBq\xE5\x14a\0\x86W\x80ch\xBD>8\x14a\0\x81W\x80cs\xCB-\x03\x14a\0|W\x80c\x8A\x04\xBD\xD5\x14a\0wW\x80c\xAC\xAD)\x89\x14a\0rW\x80c\xAF\xBA\x13\xC4\x14a\0mWc\xDC\x17\x83U\x03a\x0B\xF7Wa\x0B\x8FV[a\x0BfV[a\t\xBFV[a\t\x85V[a\x08\x1FV[a\x04\xCCV[a\x03\xC7V[4a\x01\x07W`@6`\x03\x19\x01\x12a\x01\x02W`$5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\0\xFDWa\0\xF5a\0\xD2a\0\xC4` \x936\x90`\x04\x01a\x03aV[\x83\x80\x82Q\x83\x01\x01\x91\x01a\x0CZV[\x90a\0\xEFa\0\xE1`\x045a\x0EyV[\x86\x80\x82Q\x83\x01\x01\x91\x01a\x0C\xC6V[\x92a\x10tV[`@Q\x90\x81R\xF3[a\x01\xA7V[a\x01WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01Ra7\xB7`\xF1\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x90\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\xA0\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x02\x82W`@RV[a\x02PV[`\xC0\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x02\x82W`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x02\x82W`@RV[\x92\x91\x92g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x02\x82W`@Q\x91a\x02\xEF`\x1F\x82\x01`\x1F\x19\x16` \x01\x84a\x02\xA3V[\x82\x94\x81\x84R\x81\x83\x01\x11a\x03\x0CW\x82\x81` \x93\x84`\0\x96\x017\x01\x01RV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R`\x84\x90\xFD[\x90\x80`\x1F\x83\x01\x12\x15a\x03\x7FW\x81` a\x03|\x935\x91\x01a\x02\xC5V[\x90V[a\x01\xF7V[\x90`@Q`\x80\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x02\x82W`@R```\x03\x82\x94\x80T\x84R`\x01\x81\x01T` \x85\x01R`\x02\x81\x01T`@\x85\x01R\x01T\x91\x01RV[4a\x04\xB1W` 6`\x03\x19\x01\x12a\x01\x02W`\x045`\0R`\x01` Ra\x01\xC0`@`\0 a\x03\xF4\x81a\x03\x84V[\x90a\x04\x01`\x04\x82\x01a\x03\x84V[\x90a\x04\xA3a\x04\x11`\x08\x83\x01a\x03\x84V[a\x04y`\x0C\x84\x01T\x93`\r`\x01\x80`\xA0\x1B\x03\x91\x01T\x16\x94a\x04S`@Q\x80\x98``\x80\x91\x80Q\x84R` \x81\x01Q` \x85\x01R`@\x81\x01Q`@\x85\x01R\x01Q\x91\x01RV[\x80Q`\x80\x88\x01R` \x81\x01Q`\xA0\x88\x01R`@\x81\x01Q`\xC0\x88\x01R``\x01Q`\xE0\x87\x01RV[\x80Qa\x01\0\x86\x01R` \x81\x01Qa\x01 \x86\x01R`@\x81\x01Qa\x01@\x86\x01R``\x01Qa\x01`\x85\x01RV[a\x01\x80\x83\x01Ra\x01\xA0\x82\x01R\xF3[a\x01\x07V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\x04\xC7WV[`\0\x80\xFD[4a\x04\xB1W``\x80`\x03\x196\x01\x12a\x01\x02Wa\x04\xE9`\x045a\x04\xB6V[`$5\x90`D5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\0\xFDWa\x05\r\x906\x90`\x04\x01a\x03aV[\x90a\x05)a\x05\x1A\x84a\x0EyV[` \x80\x82Q\x83\x01\x01\x91\x01a\x0C\xC6V[`\0T\x90\x92\x90a\x05O\x90a\x05C\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x80;\x15a\x07\rW`@Qc3\x85N\xFD`\xE2\x1B\x81R`\x04\x81\x01\x95\x90\x95R\x82\x90\x85\x90`$\x90\x82\x90Z\xFA\x80\x15a\x07\x08W`\0\x91\x82\x95\x83\x92a\x06\xCDW[P\x80` \x80a\x05\x9C\x93Q\x83\x01\x01\x91\x01a\x0CZV[\x93\x91\x94\x90\x96\x82\x86\x11`\0\x14a\x06@WP\x91a\x05\xDD\x82a\x05\xD8\x83a\x05\xD3a\x05\xEE\x97a\x05\xC9a\x05\xE5\x98\x8Ca\r\x8AV[\x90\x8C\x01Q\x90a\x15\x9BV[a\x15\x9BV[a\x15\xC7V[P[\x83a\r\xE5V[\x93\x82\x86\x85a\x10tV[\x93\x84`\x13\x19\x12\x92\x83a\x065W[a\x061\x93\x94`@Q\x96\x87\x96\x87\x92`\xA0\x94\x91\x97\x96\x95\x92`\xC0\x85\x01\x98\x15\x15\x85R` \x85\x01R`@\x84\x01R``\x83\x01R`\x80\x82\x01R\x01RV[\x03\x90\xF3[`\x14\x86\x12\x93Pa\x05\xFBV[\x90\x92\x90\x91P\x81\x87\x11\x15a\x06oWa\x06i\x82a\x05\xD8\x83a\x05\xD3a\x05\xEE\x97a\x05\xC9a\x05\xE5\x98\x8Ea\r\x8AV[Pa\x05\xDFV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`0`$\x82\x01R\x7Finvalid swap: inputs x and y hav`D\x82\x01Roe the same sign!`\x80\x1B`d\x82\x01R`\x84\x90\xFD[\x90\x95Pa\x05\x9C\x92Pa\x06\xF5\x91P\x83=\x85\x11a\x07\x01W[a\x06\xED\x81\x83a\x02\xA3V[\x81\x01\x90a\x0CZV[\x95\x91\x92\x90\x95\x91\x90a\x05\x88V[P=a\x06\xE3V[a\rhV[a\r\x15V[\x90```\x03\x19\x83\x01\x12a\x01\x02W`\x045a\x07+\x81a\x04\xB6V[\x91`$5\x91`D5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x83\x82\x11a\0\xFDW\x80`#\x83\x01\x12\x15a\x03\x7FW\x81`\x04\x015\x93\x84\x11a\x07\xC6W`$\x84\x83\x01\x01\x11a\x07mW`$\x01\x91\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rjrray stride`\xA8\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x0ENL/$\r\x8C\xAD\xCC\xEE\x8D`\xAB\x1B`d\x82\x01R`\x84\x90\xFD[4a\x04\xB1Wa\x08-6a\x07\x12V[`\0T\x92\x93P\x91`\x01`\x01`\xA0\x1B\x03\x163\x03a\tsWa\t1a\t)a\x05\x1Aa\t$\x95a\x08k`\x80\x95a\x08\xEE\x97a\x08ba\x0ENV[P\x81\x01\x90a\x11sV[` \x81\x9B\x94\x9A\x93\x99\x92\x9B\x01Qa\x08\x8B\x86`\0R`\x01` R`@`\0 \x90V[U`@\x81\x01Q`\x04a\x08\xA7\x87`\0R`\x01` R`@`\0 \x90V[\x01U\x80Q`\x08a\x08\xC1\x87`\0R`\x01` R`@`\0 \x90V[\x01U``\x81\x01Q`\x0Ca\x08\xDE\x87`\0R`\x01` R`@`\0 \x90V[\x01U\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\ra\t\x04\x84`\0R`\x01` R`@`\0 \x90V[\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90UV[a\x0EyV[\x84\x83\x85a\x10tV[\x92\x83`\x13\x19\x12\x91\x82a\thW[`@\x80Q\x93\x15\x15\x84R` \x84\x01\x95\x90\x95R\x93\x82\x01\x92\x90\x92R``\x81\x01\x92\x90\x92R`\x80\x82\x01R`\xA0\x90\xF3[`\x14\x85\x12\x92Pa\t>V[`@QchS\xCB\xA7`\xE0\x1B\x81R`\x04\x90\xFD[4a\x04\xB1W``a\t\x956a\x07\x12V[\x81\x80\x94P\x94\x92\x94\x01\x03\x12a\x01\x02W\x805\x90a\t1a\t)a\x05\x1A`@` \x85\x015\x94\x015\x95a\x0EyV[4a\x04\xB1Wa\t\xCD6a\x07\x12V[`\0T\x90\x93\x90`\x01`\x01`\xA0\x1B\x03\x90\x81\x163\x03a\tsWa\n\x0Ea\x05C`\ra\n\0\x87`\0R`\x01` R`@`\0 \x90V[\x01T`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x91\x16\x03a\x0BTWa\n!\x83\x82\x01\x82a\x0E\x19V[a\n*\x81a\x0E.V[`\x01\x81\x03a\ncWPa\nMa\nHa\n^\x92`\x0C\x94\x956\x91a\x02\xC5V[a\x12\xAFV[\x92`\0R`\x01` R`@`\0 \x90V[\x01U[\0[a\nl\x81a\x0E.V[`\x03\x81\x03a\n\xA8WP\x90a\n\x90a\n\x8Ba\n\xA3\x93a\na\x956\x91a\x02\xC5V[a\x12\x05V[\x92\x90\x91`\0R`\x01` R`@`\0 \x90V[a\x12-V[a\n\xB1\x81a\x0E.V[`\x04\x81\x03a\n\xEBWP\x90`\x04a\n\xD2a\n\x8Ba\n\xE5\x94a\na\x966\x91a\x02\xC5V[\x93\x90\x92`\0R`\x01` R`@`\0 \x90V[\x01a\x12-V[a\n\xF4\x81a\x0E.V[`\x02\x81\x03a\x0B\x15WP\x90`\x08a\n\xD2a\n\x8Ba\n\xE5\x94a\na\x966\x91a\x02\xC5V[\x80a\x0B!`\x05\x92a\x0E.V[\x03a\x0BBWa\t\x04a\nMa\x0B=`\r\x93a\na\x966\x91a\x02\xC5V[a\x11\xDDV[`@Qc#]+=`\xE0\x1B\x81R`\x04\x90\xFD[`@Qcn\xDA\xEF/`\xE1\x1B\x81R`\x04\x90\xFD[4a\x04\xB1W`\x006`\x03\x19\x01\x12a\x01\x02W`\0T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x04\xB1W` \x80`\x03\x196\x01\x12a\x01\x02Wa\x0B\xAC`\x045a\x0EyV[\x90`@\x80Q\x91` \x83R\x83Q\x91\x82` \x85\x01R`\0[\x83\x81\x10a\x0B\xE4W\x84`@\x81\x86`\0\x83\x82\x84\x01\x01R`\x1F\x80\x19\x91\x01\x16\x81\x01\x03\x01\x90\xF3[\x85\x81\x01\x83\x01Q\x85\x82\x01\x83\x01R\x82\x01a\x0B\xC2V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01Rtnor receive functions`X\x1B`d\x82\x01R`\x84\x90\xFD[\x90\x81``\x91\x03\x12a\x01\x02W\x80Q\x91`@` \x83\x01Q\x92\x01Q\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FABI decoding: struct data too sh`D\x82\x01Rb\x1B\xDC\x9D`\xEA\x1B`d\x82\x01R`\x84\x90\xFD[\x90\x81`\xA0\x91\x03\x12a\x01\x02W`\x80`@Q\x91a\x0C\xE0\x83a\x02fV[\x80Q\x83R` \x81\x01Q` \x84\x01R`@\x81\x01Q`@\x84\x01R``\x81\x01Q``\x84\x01R\x01Qa\r\r\x81a\x04\xB6V[`\x80\x82\x01R\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01Rd code`\xD8\x1B`d\x82\x01R`\x84\x90\xFD[`@Q=`\0\x82>=\x90\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x91\x90\x82\x03\x91\x82\x11a\r\x97WV[a\rtV[\x91\x90\x82\x01\x80\x92\x11a\r\x97WV[\x90g\x1B\xC1mgN\xC8\0\0`\0\x83\x82\x03\x93\x12\x81\x84\x12\x81\x16\x91\x84\x13\x90\x15\x16\x17a\r\x97WV[\x90g\x11\x90\0\xAB\x10\x0F\xFB\xFF\x19\x82\x01\x91\x82\x13`\x01\x16a\r\x97WV[\x81\x81\x03\x92\x91`\0\x13\x80\x15\x82\x85\x13\x16\x91\x84\x12\x16\x17a\r\x97WV[`\x01`\xFF\x1B\x81\x14a\r\x97W`\0\x03\x90V[`\x06\x11\x15a\x04\xC7WV[\x90\x81` \x91\x03\x12a\x01\x02W5a\x03|\x81a\x0E\x0FV[`\x06\x11\x15a\x0E8WV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`@Q\x90a\x0E[\x82a\x02fV[`\0`\x80\x83\x82\x81R\x82` \x82\x01R\x82`@\x82\x01R\x82``\x82\x01R\x01RV[a\x0E\x81a\x0ENV[\x81`\0R`\x01` Ra\x0E\x9Fa\x0E\x9A`@`\0 a\x03\x84V[a\x12\xE4V[\x91` \x82\x01\x92\x83R\x80`\0R`\x01` Ra\x0E\xC3a\x0E\x9A`\x08`@`\0 \x01a\x03\x84V[\x82R`\x0Ca\x0F\x04a\x0E\xECa\x0E\x9A`\x04a\x0E\xE6\x86`\0R`\x01` R`@`\0 \x90V[\x01a\x03\x84V[\x92`@\x85\x01\x93\x84R`\0R`\x01` R`@`\0 \x90V[\x01T\x90``\x83\x01\x91\x82R`@Q\x93\x83Q` \x86\x01RQ`@\x85\x01RQ``\x84\x01RQ`\x80\x83\x01R`\x80`\x01\x80`\xA0\x1B\x03\x91\x01Q\x16`\xA0\x82\x01R`\xA0\x81Ra\x03|\x81a\x02\x87V[\x90\x81g \x05\xFEO&\x8E\xA0\0\x01\x91\x82\x12`\x01\x16a\r\x97WV[\x90\x81g\r\xC5R\x7Fd, \0\x01\x91\x82\x12`\x01\x16a\r\x97WV[\x90\x81g\r\xE0\xB6\xB3\xA7d\0\0\x01\x91\x82\x12`\x01\x16a\r\x97WV[\x90\x81g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19\x01\x91\x82\x12\x15`\x01\x16a\r\x97WV[\x90\x81g\x14\xA8EL\x19\xE1\xAC\0\x01\x91\x82\x12`\x01\x16a\r\x97WV[\x90\x81g\x0F\xC1\x0E\x01W\x82w\xFF\x19\x01\x91\x82\x12\x15`\x01\x16a\r\x97WV[\x90\x81g\x03\xDE\xBD\x08;\x8C|\0\x01\x91\x82\x12`\x01\x16a\r\x97WV[\x90\x81g\x02\x95\xD4\0\xEA2W\xFF\x19\x01\x91\x82\x12\x15`\x01\x16a\r\x97WV[\x90\x81g\x01W\xD8\xB2\xEC\xC7\x08\0\x01\x91\x82\x12`\x01\x16a\r\x97WV[\x90\x81g\x051\n\xA7\xD5!0\0\x01\x91\x82\x12`\x01\x16a\r\x97WV[\x90\x81g\r\xE0\xCC=\x15a\0\0\x01\x91\x82\x12`\x01\x16a\r\x97WV[\x91\x90\x91`\0\x83\x82\x01\x93\x84\x12\x91\x12\x90\x80\x15\x82\x16\x91\x15\x16\x17a\r\x97WV[\x90\x92\x82\x82\x10\x15a\x11.Wa\x03|\x93a\x10\xF1\x92\x84g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82a\x10\x9C\x83\x83a\x15\xF7V[\x10a\x11\x1BWP`\x01`\x01`\xFF\x1B\x03\x95\x90P[\x83Q\x91a\x10\xC4a\x10\xBE\x83\x85a\x16\x19V[\x85a\x15\xF7V[\x10a\x10\xF6WP`\x01`\x01`\xFF\x1B\x03\x92a\x10\xEB\x92P\x90P[`@` \x82\x01Q\x91\x01Q\x90a\x15rV[\x92a\x10XV[a\x10XV[a\x10\xEB\x92a\x11\na\x11\x10\x92a\x11\x15\x94a\x16\x19V[\x90a\x15\xF7V[a\x13\x9FV[\x91a\x10\xDBV[a\x11(\x91a\x11\x10\x91a\x15\xF7V[\x94a\x10\xAEV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FtradingFunction: invalid x\0\0\0\0\0\0`D\x82\x01R`d\x90\xFD[\x80\x91\x03a\x01\0\x81\x12a\x01\x02W\x815\x92` \x83\x015\x92`\xA0`@\x82\x015\x93`_\x19\x01\x12a\x11\xD8W`\xE0`@Q\x91a\x11\xA8\x83a\x02fV[``\x81\x015\x83R`\x80\x81\x015` \x84\x01R`\xA0\x81\x015`@\x84\x01R`\xC0\x81\x015``\x84\x01R\x015a\r\r\x81a\x04\xB6V[a\x0CuV[`@\x81\x80Q\x81\x01\x03\x12a\x01\x02W\x80a\x11\xFA` `@\x93\x01Qa\x0E\x0FV[\x01Qa\x05C\x81a\x04\xB6V[``\x81\x80Q\x81\x01\x03\x12a\x01\x02Wa\x12\x1F` \x82\x01Qa\x0E\x0FV[```@\x82\x01Q\x91\x01Q\x90\x91V[\x91\x90B\x82\x11\x15a\x12\x9DWa\x12Ca\x0E\x9A\x84a\x03\x84V[\x90\x81\x84UB`\x03\x85\x01UB\x83\x03\x91\x83\x83\x11a\r\x97Wa\x12a\x91a\r\xE5V[B\x83\x14a\x12\x87W`\x01`\xFF\x1B\x81\x14`\0\x19\x83\x14\x16a\r\x97W`\x02\x92`\x01\x85\x01U\x05\x91\x01UV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`@Qcf\xF1\x02\xED`\xE1\x1B\x81R`\x04\x90\xFD[`@\x81\x80Q\x81\x01\x03\x12a\x01\x02W\x80a\x12\xCC` `@\x93\x01Qa\x0E\x0FV[\x01Q\x90V[\x81\x81\x02\x92\x91\x81\x15\x91\x84\x04\x14\x17\x15a\r\x97WV[``\x81\x01Q` \x82\x01Q\x80\x82\x14a\x13_W\x80B\x11`\0\x14a\x13WW\x90[\x81\x03\x90\x81\x11a\r\x97W`@\x82\x01\x90\x81Q`\0\x81\x13`\0\x14a\x134WPa\x13.\x90a\x03|\x93Q\x92Q\x90a\x12\xD1V[\x90a\r\x9CV[\x92a\x13K\x92Pa\x13E\x90Q\x93a\r\xFEV[\x90a\x12\xD1V[\x81\x03\x90\x81\x11a\r\x97W\x90V[PB\x90a\x13\x01V[PPQ\x90V[\x90c;\x9A\xCA\0\x91\x82\x81\x02\x92\x81\x84\x05\x14\x90\x15\x17\x15a\r\x97WV[`\0\x81\x90\x03\x91\x90`\x01`\xFF\x1B\x81\x14`\x01\x16a\r\x97W`\0\x19\x83\x05\x03a\r\x97WV[g\x06\xF0[Y\xD3\xB2\0\0\x81\x14a\x15lWg\r\xE0\xB6\xB3\xA7d\0\0\x80\x82\x12\x15a\x15\x16W\x81\x15a\x157W`\x01\x82`\x01\x1B\x91`\x02\x93\x83\x05`\x02\x03a\r\x97W`\0\x83\x12\x80\x15a\x15[W[a\x15IW\x82\x15a\x15\x16Wg\x1B\xC1mgN\xC8\0\0\x83\x14a\x157W\x82\x12\x91\x82\x15a\x15(W\x92[a\x14\x10\x84a\x1B\xC7V[\x80\x15a\x15\x16Wa\x14\x82a\x14Aa\x14<a\x147a\x142a\x14\x87\x95\x99\x97\x96\x99a\x17[V[a\x1C\x88V[a\x16eV[a\x13eV[a\x14}a\x14Ua\x14P\x83a\x1B\xF2V[a\x0FJV[a\x14wa\x14ra\x14la\x14g\x86a\x1C\x1DV[a\x0FbV[\x85a\x1C\xFFV[a\x0FzV[\x90a\x1CfV[a\r\xE5V[a\x1C\xB0V[\x93`\0\x92[\x81\x84\x10a\x14\xBEWPPPPa\x03|\x91a\x14\xAB\x91`\0\x14a\x14\xB0Wa\x1B\xA0V[a\r\xFEV[a\x14\xB9\x90a\r\xFEV[a\x1B\xA0V[\x90\x91a\x15\x0C\x86a\x15\x06a\x14\xD6\x85a\x14}\x86\x99\x9Ba\x18\xFCV[a\x14wa\x14\xF6a\x14\xF1a\x14\xECa\x14\xAB\x87\x80a\x1C\xFFV[a\x1A\x1CV[a\x1C\xD8V[a\x15\0\x83\x86a\x1C\xFFV[\x90a\r\xE5V[\x90a\x10XV[\x95\x01\x92\x91\x90a\x14\x8CV[`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x90\xFD[a\x151\x90a\r\xA9V[\x92a\x14\x07V[`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x90\xFD[`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x90\xFD[Pg\x1B\xC1mgN\xC8\0\0\x83\x13a\x13\xE3V[P`\0\x90V[\x90a\x15|\x90a\x16eV[c;\x9A\xCA\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17\x15a\r\x97Wa\x03|\x91a\x16\x19V[\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x04\xC7W`\x01g\r\xE0\xB6\xB3\xA7d\0\0`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[\x90g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x81\x02\x92\x81\x84\x04\x14\x90\x15\x17\x81\x15\x15\x16\x15a\x04\xC7W`\x01\x90`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17\x82\x15\x15\x16\x15a\x04\xC7W\x04\x90V[\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x04\xC7Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x04\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x04\xC7Wg\x1B\xC1mgN\xC8\0\0\x90\x04\x90V[`\xB5\x81`\x01`\x88\x1B\x81\x10\x15a\x17\x0CW[\x80i\x01\0\0\0\0\0\0\0\0\0b\x01\0\0\x92\x10\x15a\x16\xFFW[e\x01\0\0\0\0\0\x81\x10\x15a\x16\xF2W[c\x01\0\0\0\x81\x10\x15a\x16\xE5W[\x01\x02`\x12\x1C`\x01\x90\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x80\x80\x92\x04\x10\x90\x03\x90V[`\x10\x1C\x91`\x08\x1B\x91a\x16\xA9V[` \x1C\x91`\x10\x1B\x91a\x16\x9CV[`@\x1C\x91` \x1B\x91a\x16\x8DV[Ph\xB5\0\0\0\0\0\0\0\0\x90P`\x80\x82\x90\x1Ca\x16uV[\x15a\x17*WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x90\xFD[}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x90a\x17\x87`\0\x82\x13a\x17#V[q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06a\x17\xA3\x82a\x1DKV[``\x92\x83\x82`\x9F\x03\x01\x1B`\x9F\x1C\x90`_\x19\x01}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x02\x92l\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x82m\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x81m\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x81m\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x81m\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x81m\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x81lFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x02\x8C\x1D\x01\x02\x8A\x1D\x01\x02\x88\x1D\x01\x02\x86\x1D\x01\x02\x84\x1D\x01\x02\x82\x1D\x01\x91x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x91l\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x90\x82m\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x81l\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x81m\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x81m\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x81l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x81\x01\x02\x89\x1D\x01\x02\x87\x1D\x01\x02\x85\x1D\x01\x02\x83\x1D\x01\x02\x90\x1D\x01\x02\x01\x05\x02\x01\x01`\xAE\x1D\x90V[\x80\x15a\x1A\x0FWgV\x98\xEE\xF0fp\0\0\x81\x12\x15a\x15lWgV\x98\xEE\xF0fo\xFF\xFF\x19\x81\x13\x15a\x1A\x02W`\0a\x19\xF2a\x191\x83a\x1D\xBDV[a\x19\xBAa\x14\xECa\x19Ka\x19Fa\x14r\x85a\x16:V[a\x1CGV[\x92a\x10\xF1a\x19\xEDa\x19\xE8a\x19\xE1a\x19\xDBa\x19\xD6a\x19\xD0a\x19\xCBa\x19\xC5a\x19\xC0\x8Da\x19\xBAa\x19\xB5a\x19\xAFa\x19\xAAa\x14la\x19\xA5a\x19\x9Fa\x19\x9Aa\x19\x94a\x19\x8F\x8Aa\x1D V[a\x0F\x92V[\x89a\x1C\xFFV[a\x0F\xACV[\x87a\x1C\xFFV[a\x0F\xC4V[a\x0F\xDEV[\x83a\x1C\xFFV[a\x0F\xF6V[\x90a\x1C\xFFV[a\x10\x10V[\x8Ca\x1C\xFFV[a\x10(V[\x8Aa\x1C\xFFV[a\x10@V[\x88a\x1C\xFFV[\x93\x80a\x1C\xFFV[a\x13~V[a\r\xCCV[\x91\x12\x15a\x03|Wa\x03|\x90a\r\xA9V[Pg\x1B\xC1mgN\xC8\0\0\x90V[Pg\r\xE0\xB6\xB3\xA7d\0\0\x90V[h\x02H\xCE6\xA7\x0C\xB2k>\x19\x81\x13\x15a\x15lWh\x07U\xBFy\x8BJ\x1B\xF1\xE5\x81\x12\x15a\x1BlWe\x03x-\xAC\xE9\xD9\x90`N\x1B\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x91``\x90`\x01`_\x1B\x84\x82\x84\x1B\x05\x01\x82\x1D\x93\x84\x02\x90\x03\x80l\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x81\x01\x02\x82\x1D\x90n\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dPn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x82n\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x81m\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x81m\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x81m\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x81l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x02\x8D\x1D\x01\x02\x8B\x1D\x01\x02\x89\x1D\x01\x02\x87\x1D\x01\x02\x85\x1D\x01\x93m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x93m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEA\x81\x01\x90\x84m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x91\x01\x01\x02\x90\x1D\x01\x02\x01\x05\x02\x90`\xC3\x03\x1C\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x90\xFD[g\x13\xA0K\xBD\xFD\xC9\xBE\x88\x90\x80\x82\x02\x91\x82\x05\x14`\x01\x16\x15a\x04\xC7Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x04\xC7Wg\x1B\xC1mgN\xC8\0\0\x90\x05\x90V[g\x03\xC1f\\z\xAB \0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x04\xC7Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[f\x9F2u$b\xA0\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x04\xC7Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[`\x01\x81\x15\x15\x16\x15a\x04\xC7Wn\xC0\x97\xCE{\xC9\x07\x15\xB3K\x9F\x10\0\0\0\0\x05\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17\x82\x15\x15\x16\x15a\x04\xC7W\x05\x90V[g\x1B\xC1mgN\xC7\xFF\xFF\x19\x81\x81\x02\x91`\x01\x91\x83\x05\x14\x16\x15a\x04\xC7Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\t\xD0(\xCCo _\xFF\x19\x81\x81\x02\x91`\x01\x91\x83\x05\x14\x16\x15a\x04\xC7Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x90\x80\x82\x02\x91\x82\x05\x14`\x01\x16\x15a\x04\xC7Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x04\xC7Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\x02_\x0F\xE1\x05\xA3\x14\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x04\xC7Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[a\x1DV\x81\x15\x15a\x17#V[\x80o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x07\x1B\x81\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x81\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x81\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x81\x81\x1C`\xFF\x10`\x03\x1B\x17\x81\x81\x1C`\x0F\x10`\x02\x1B\x17\x81\x81\x1C`\x03\x10`\x01\x1B\x17\x90\x81\x1C`\x01\x10\x17\x90V[`\x01`\xFF\x1B\x81\x14a\x1D\xD8W`\0\x81\x12\x15a\x03|W\x19`\x01\x01\x90V[`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x90\xFD\xFE\xA2dipfsX\"\x12 \xEC6\x0C\xDB\x88T\xB6\x16\x94\x9BZ\xE2\x06 i\xDF$I\xE5\xEF\xA1\xB1\xA1\xF0\x1F\xFAP\xF0\x94\x9B\x80\x85dsolcC\0\x08\x16\x003";
    /// The deployed bytecode of the contract.
    pub static LOGNORMAL_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct LogNormal<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for LogNormal<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for LogNormal<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for LogNormal<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for LogNormal<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(LogNormal))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> LogNormal<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                LOGNORMAL_ABI.clone(),
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
                LOGNORMAL_ABI.clone(),
                LOGNORMAL_BYTECODE.clone().into(),
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
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([175, 186, 19, 196], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPoolParams` (0xdc178355) function
        pub fn get_pool_params(
            &self,
            pool_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Bytes> {
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
            (
                DynamicParam,
                DynamicParam,
                DynamicParam,
                ::ethers::core::types::U256,
                ::ethers::core::types::Address,
            ),
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
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for LogNormal<M> {
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
        Hash,
    )]
    #[etherror(name = "Infinity", abi = "Infinity()")]
    pub struct Infinity;
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
        Hash,
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
        Hash,
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
        Hash,
    )]
    #[etherror(name = "InvalidUpdateEnd", abi = "InvalidUpdateEnd()")]
    pub struct InvalidUpdateEnd;
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
        Hash,
    )]
    #[etherror(name = "NotDFMM", abi = "NotDFMM()")]
    pub struct NotDFMM;
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
    pub enum LogNormalErrors {
        Infinity(Infinity),
        InvalidSender(InvalidSender),
        InvalidUpdateCode(InvalidUpdateCode),
        InvalidUpdateEnd(InvalidUpdateEnd),
        Min(Min),
        NegativeInfinity(NegativeInfinity),
        NotDFMM(NotDFMM),
        OutOfBounds(OutOfBounds),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for LogNormalErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <Infinity as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Infinity(decoded));
            }
            if let Ok(decoded) = <InvalidSender as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidSender(decoded));
            }
            if let Ok(decoded) = <InvalidUpdateCode as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InvalidUpdateCode(decoded));
            }
            if let Ok(decoded) = <InvalidUpdateEnd as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InvalidUpdateEnd(decoded));
            }
            if let Ok(decoded) = <Min as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Min(decoded));
            }
            if let Ok(decoded) = <NegativeInfinity as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::NegativeInfinity(decoded));
            }
            if let Ok(decoded) = <NotDFMM as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NotDFMM(decoded));
            }
            if let Ok(decoded) = <OutOfBounds as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::OutOfBounds(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for LogNormalErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::Infinity(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InvalidSender(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InvalidUpdateCode(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InvalidUpdateEnd(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Min(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NegativeInfinity(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NotDFMM(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OutOfBounds(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for LogNormalErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector == <Infinity as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <InvalidSender as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidUpdateCode as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector == <InvalidUpdateEnd as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector == <Min as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <NegativeInfinity as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector == <NotDFMM as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <OutOfBounds as ::ethers::contract::EthError>::selector() => true,
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for LogNormalErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Infinity(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidSender(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidUpdateCode(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidUpdateEnd(element) => ::core::fmt::Display::fmt(element, f),
                Self::Min(element) => ::core::fmt::Display::fmt(element, f),
                Self::NegativeInfinity(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotDFMM(element) => ::core::fmt::Display::fmt(element, f),
                Self::OutOfBounds(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for LogNormalErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<Infinity> for LogNormalErrors {
        fn from(value: Infinity) -> Self {
            Self::Infinity(value)
        }
    }
    impl ::core::convert::From<InvalidSender> for LogNormalErrors {
        fn from(value: InvalidSender) -> Self {
            Self::InvalidSender(value)
        }
    }
    impl ::core::convert::From<InvalidUpdateCode> for LogNormalErrors {
        fn from(value: InvalidUpdateCode) -> Self {
            Self::InvalidUpdateCode(value)
        }
    }
    impl ::core::convert::From<InvalidUpdateEnd> for LogNormalErrors {
        fn from(value: InvalidUpdateEnd) -> Self {
            Self::InvalidUpdateEnd(value)
        }
    }
    impl ::core::convert::From<Min> for LogNormalErrors {
        fn from(value: Min) -> Self {
            Self::Min(value)
        }
    }
    impl ::core::convert::From<NegativeInfinity> for LogNormalErrors {
        fn from(value: NegativeInfinity) -> Self {
            Self::NegativeInfinity(value)
        }
    }
    impl ::core::convert::From<NotDFMM> for LogNormalErrors {
        fn from(value: NotDFMM) -> Self {
            Self::NotDFMM(value)
        }
    }
    impl ::core::convert::From<OutOfBounds> for LogNormalErrors {
        fn from(value: OutOfBounds) -> Self {
            Self::OutOfBounds(value)
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
        Hash,
    )]
    #[ethcall(
        name = "computeSwapConstant",
        abi = "computeSwapConstant(uint256,bytes)"
    )]
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
    )]
    pub enum LogNormalCalls {
        ComputeSwapConstant(ComputeSwapConstantCall),
        Dfmm(DfmmCall),
        GetPoolParams(GetPoolParamsCall),
        Init(InitCall),
        InternalParams(InternalParamsCall),
        Update(UpdateCall),
        ValidateAllocateOrDeallocate(ValidateAllocateOrDeallocateCall),
        ValidateSwap(ValidateSwapCall),
    }
    impl ::ethers::core::abi::AbiDecode for LogNormalCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <ComputeSwapConstantCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ComputeSwapConstant(decoded));
            }
            if let Ok(decoded) = <DfmmCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Dfmm(decoded));
            }
            if let Ok(decoded) = <GetPoolParamsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetPoolParams(decoded));
            }
            if let Ok(decoded) = <InitCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Init(decoded));
            }
            if let Ok(decoded) =
                <InternalParamsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InternalParams(decoded));
            }
            if let Ok(decoded) = <UpdateCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Update(decoded));
            }
            if let Ok(decoded) =
                <ValidateAllocateOrDeallocateCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ValidateAllocateOrDeallocate(decoded));
            }
            if let Ok(decoded) = <ValidateSwapCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ValidateSwap(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for LogNormalCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::ComputeSwapConstant(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Dfmm(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetPoolParams(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Init(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InternalParams(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Update(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ValidateAllocateOrDeallocate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ValidateSwap(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for LogNormalCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ComputeSwapConstant(element) => ::core::fmt::Display::fmt(element, f),
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
    impl ::core::convert::From<ComputeSwapConstantCall> for LogNormalCalls {
        fn from(value: ComputeSwapConstantCall) -> Self {
            Self::ComputeSwapConstant(value)
        }
    }
    impl ::core::convert::From<DfmmCall> for LogNormalCalls {
        fn from(value: DfmmCall) -> Self {
            Self::Dfmm(value)
        }
    }
    impl ::core::convert::From<GetPoolParamsCall> for LogNormalCalls {
        fn from(value: GetPoolParamsCall) -> Self {
            Self::GetPoolParams(value)
        }
    }
    impl ::core::convert::From<InitCall> for LogNormalCalls {
        fn from(value: InitCall) -> Self {
            Self::Init(value)
        }
    }
    impl ::core::convert::From<InternalParamsCall> for LogNormalCalls {
        fn from(value: InternalParamsCall) -> Self {
            Self::InternalParams(value)
        }
    }
    impl ::core::convert::From<UpdateCall> for LogNormalCalls {
        fn from(value: UpdateCall) -> Self {
            Self::Update(value)
        }
    }
    impl ::core::convert::From<ValidateAllocateOrDeallocateCall> for LogNormalCalls {
        fn from(value: ValidateAllocateOrDeallocateCall) -> Self {
            Self::ValidateAllocateOrDeallocate(value)
        }
    }
    impl ::core::convert::From<ValidateSwapCall> for LogNormalCalls {
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
    )]
    pub struct InternalParamsReturn {
        pub sigma: DynamicParam,
        pub tau: DynamicParam,
        pub strike: DynamicParam,
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
        Hash,
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
        Hash,
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
