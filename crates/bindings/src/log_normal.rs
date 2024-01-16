pub use log_normal::*;
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
pub mod log_normal {
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
                                    name: ::std::borrow::ToOwned::to_owned("sigma"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct DynamicParam"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tau"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct DynamicParam"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("strike"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(256usize),
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
    pub static LOGNORMAL_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`@Qa\x1E\x898\x03\x80a\x1E\x89\x839\x81\x01`@\x81\x90Ra\0|\x91a\0\xA1V[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90Ua\x01\x1CV[`\0` \x82\x84\x03\x12\x15a\0\xFEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01\x15W`\0\x80\xFD[\x93\x92PPPV[a\x1D^\x80a\x01+`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\0\xD4W`\x005`\xE0\x1C\x80c\x8E-\xD4\0\x11a\0\xA8W\x80c\x8E-\xD4\0\x14a\x02\xE3W\x80c\x9F\x83\x13{\x14a\x03 W\x80c\xAF\xBA\x13\xC4\x14a\x033W\x80c\xDC\x17\x83U\x14a\x03^Wa\0\xD4V[\x80b.RK\x14a\x019W\x80c\x02\x16\xB88\x14a\x01_W\x80c\x1E\xDBq\xE5\x14a\x01tW\x80c2\x14\x89\x0F\x14a\x02\xA1W[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x01La\x01G6`\x04a\x17\xC0V[a\x03~V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01ra\x01m6`\x04a\x18\xB8V[a\x03\xD1V[\0[a\x02%a\x01\x826`\x04a\x19\xE7V[`\x01` \x81\x81R`\0\x92\x83R`@\x92\x83\x90 \x83Q`\x80\x80\x82\x01\x86R\x82T\x82R\x93\x82\x01T\x81\x84\x01R`\x02\x82\x01T\x81\x86\x01R`\x03\x82\x01T``\x80\x83\x01\x91\x90\x91R\x85Q\x80\x86\x01\x87R`\x04\x84\x01T\x81R`\x05\x84\x01T\x81\x86\x01R`\x06\x84\x01T\x81\x88\x01R`\x07\x84\x01T\x81\x83\x01R\x86Q\x95\x86\x01\x87R`\x08\x84\x01T\x86R`\t\x84\x01T\x94\x86\x01\x94\x90\x94R`\n\x83\x01T\x95\x85\x01\x95\x90\x95R`\x0B\x82\x01T\x94\x84\x01\x94\x90\x94R`\x0C\x01T\x90\x91\x90\x84V[`@\x80Q\x85Q\x81R` \x80\x87\x01Q\x81\x83\x01R\x86\x83\x01Q\x82\x84\x01R``\x96\x87\x01Q\x87\x83\x01R\x85Q`\x80\x83\x01R\x80\x86\x01Q`\xA0\x83\x01R\x85\x83\x01Q`\xC0\x83\x01R\x94\x86\x01Q`\xE0\x82\x01R\x83Qa\x01\0\x82\x01R\x93\x83\x01Qa\x01 \x85\x01R\x82\x01Qa\x01@\x84\x01R\x92\x01Qa\x01`\x82\x01Ra\x01\x80\x81\x01\x91\x90\x91Ra\x01\xA0\x01a\x01VV[a\x02\xB4a\x02\xAF6`\x04a\x17\xC0V[a\x04pV[`@\x80Q\x96\x15\x15\x87R` \x87\x01\x95\x90\x95R\x93\x85\x01\x92\x90\x92R``\x84\x01R`\x80\x83\x01R`\xA0\x82\x01R`\xC0\x01a\x01VV[a\x02\xF6a\x02\xF16`\x04a\x18\xB8V[a\x06\xDCV[`@\x80Q\x95\x15\x15\x86R` \x86\x01\x94\x90\x94R\x92\x84\x01\x91\x90\x91R``\x83\x01R`\x80\x82\x01R`\xA0\x01a\x01VV[a\x02\xF6a\x03.6`\x04a\x18\xB8V[a\x07+V[`\0Ta\x03F\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01VV[a\x03qa\x03l6`\x04a\x19\xE7V[a\x07SV[`@Qa\x01V\x91\x90a\x1A\x03V[`\0\x80`\0\x80\x84\x80` \x01\x90Q\x81\x01\x90a\x03\x98\x91\x90a\x1AQV[\x92P\x92P\x92Pa\x03\xC5\x83\x83\x83a\x03\xAD\x8Aa\x07SV[\x80` \x01\x90Q\x81\x01\x90a\x03\xC0\x91\x90a\x1A\xD3V[a\x08\xCEV[\x93PPPP[\x92\x91PPV[`\0a\x03\xDF\x82\x84\x01\x84a\x1BcV[`\0\x94\x85R`\x01` \x81\x81R`@\x96\x87\x90 \x83Q\x80Q\x82U\x80\x83\x01Q\x93\x82\x01\x93\x90\x93U\x82\x88\x01Q`\x02\x82\x01U``\x92\x83\x01Q`\x03\x82\x01U\x81\x84\x01Q\x80Q`\x04\x83\x01U\x80\x83\x01Q`\x05\x83\x01U\x80\x89\x01Q`\x06\x83\x01U\x83\x01Q`\x07\x82\x01U\x87\x84\x01Q\x80Q`\x08\x83\x01U\x91\x82\x01Q`\t\x82\x01U\x96\x81\x01Q`\n\x88\x01U\x81\x01Q`\x0B\x87\x01U\x01Q`\x0C\x90\x94\x01\x93\x90\x93UPPPV[`\0\x80`\0\x80`\0\x80`\0a\x04\x84\x89a\x07SV[\x80` \x01\x90Q\x81\x01\x90a\x04\x97\x91\x90a\x1A\xD3V[`\0\x80T`@Qc3\x85N\xFD`\xE2\x1B\x81R`\x04\x81\x01\x8D\x90R\x92\x93P\x90\x91\x82\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xCE\x15;\xF4\x90`$\x01```@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x055W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x05IW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05m\x91\x90a\x1AQV[\x92P\x92P\x92P\x8A\x80` \x01\x90Q\x81\x01\x90a\x05\x87\x91\x90a\x1AQV[\x91\x98P\x96P\x94P`\0\x80\x80\x85\x8A\x11\x15a\x05\xE4Wa\x05\xA4\x86\x8Ba\x1B\xDBV[\x91Pa\x05\xBD\x87``\x01Q\x83a\t\xE0\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa\x05\xD3\x86a\x05\xCD\x83\x87a\t\xE0V[\x90a\t\xFCV[a\x05\xDD\x90\x84a\x1B\xEEV[\x92Pa\x06\x85V[\x84\x89\x11\x15a\x06\x1FWa\x05\xF6\x85\x8Aa\x1B\xDBV[\x91Pa\x06\x0F\x87``\x01Q\x83a\t\xE0\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa\x05\xD3\x85a\x05\xCD\x83\x87a\t\xE0V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`0`$\x82\x01R\x7Finvalid swap: inputs x and y hav`D\x82\x01Roe the same sign!`\x80\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[a\x06\x8F\x84\x89a\x1C\x01V[\x9APa\x06\x9D\x8A\x8A\x8A\x8Aa\x08\xCEV[\x9BP`\0\x8Ca\x06\xAC`\x1Ea\x1C(V[\x12\x80\x15a\x06\xB9WP`\x1E\x8D\x12[\x90P\x80\x80\x15a\x06\xC8WP\x83\x8C\x12\x15[\x9DPPPPPPPPP\x92\x95P\x92\x95P\x92\x95V[`\0\x80\x80\x80\x80a\x06\xEE\x86\x88\x01\x88a\x1CDV[\x91\x94P\x92P\x90Pa\x07\x04\x83\x83\x83a\x03\xAD\x8Ca\x07SV[\x93P\x83a\x07\x11`\x1Ea\x1C(V[\x12\x80\x15a\x07\x1EWP`\x1E\x84\x12[\x94P\x93\x97\x92\x96P\x93P\x93PV[`\0\x80`\0\x80`\0a\x07>\x88\x88\x88a\n\x11V[P\x93\x9C\x92\x9BP\x90\x99P\x97P\x90\x95P\x93PPPPV[``a\x07\x80`@Q\x80`\x80\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[a\x07\xD1`\x01`\0\x85\x81R` \x01\x90\x81R` \x01`\0 `\0\x01`@Q\x80`\x80\x01`@R\x90\x81`\0\x82\x01T\x81R` \x01`\x01\x82\x01T\x81R` \x01`\x02\x82\x01T\x81R` \x01`\x03\x82\x01T\x81RPPa\n\xC5V[` \x80\x83\x01\x91\x90\x91R`\0\x84\x81R`\x01\x82R`@\x90\x81\x90 \x81Q`\x80\x81\x01\x83R`\x08\x82\x01T\x81R`\t\x82\x01T\x93\x81\x01\x93\x90\x93R`\n\x81\x01T\x91\x83\x01\x91\x90\x91R`\x0B\x01T``\x82\x01Ra\x08\"\x90a\n\xC5V[\x81R`\0\x83\x81R`\x01` \x90\x81R`@\x91\x82\x90 \x82Q`\x80\x81\x01\x84R`\x04\x82\x01T\x81R`\x05\x82\x01T\x92\x81\x01\x92\x90\x92R`\x06\x81\x01T\x92\x82\x01\x92\x90\x92R`\x07\x90\x91\x01T``\x82\x01Ra\x08q\x90a\n\xC5V[`@\x82\x81\x01\x91\x82R`\0\x94\x85R`\x01` \x90\x81R\x94\x81\x90 `\x0C\x01T``\x80\x85\x01\x91\x82R\x82Q\x85Q\x81\x89\x01R\x96\x90\x94\x01Q\x86\x83\x01R\x91Q\x92\x85\x01\x92\x90\x92RQ`\x80\x80\x85\x01\x91\x90\x91R\x81Q\x80\x85\x03\x90\x91\x01\x81R`\xA0\x90\x93\x01\x90RP\x90V[`\0\x82\x85\x10a\t\x1FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FtradingFunction: invalid x\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06|V[`\0\x80g\r\xE0\xB6\xB3\xA7d\0\0a\t5\x88\x87a\x0B^V[\x10a\tIW`\x01`\x01`\xFF\x1B\x03\x91Pa\t^V[a\t[a\tV\x88\x87a\x0B^V[a\x0BsV[\x91P[g\r\xE0\xB6\xB3\xA7d\0\0a\t~\x87a\ty\x87`\0\x01Q\x89a\x0C\x19V[a\x0B^V[\x10a\t\x91WP`\x01`\x01`\xFF\x1B\x03a\t\xA9V[a\t\xA6a\tV\x87a\ty\x87`\0\x01Q\x89a\x0C\x19V[\x90P[`\0a\t\xBD\x85` \x01Q\x86`@\x01Qa\x0C.V[\x90P\x80a\t\xCA\x83\x85a\x1CsV[a\t\xD4\x91\x90a\x1CsV[\x98\x97PPPPPPPPV[`\0a\t\xF5\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x0CTV[\x93\x92PPPV[`\0a\t\xF5\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x0CTV[`\0\x80`\0\x80`\0a\nD`@Q\x80`\x80\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[a\nP\x87\x89\x01\x89a\x1C\x9BV[` \x80\x82\x01Q`\0\x8F\x81R`\x01\x90\x92R`@\x91\x82\x90 \x90\x81U\x90\x82\x01Q`\x04\x82\x01U\x81Q`\x08\x82\x01U``\x82\x01Q`\x0C\x90\x91\x01U\x92\x96P\x90\x94P\x92P\x90Pa\n\x9D\x84\x84\x84a\x03\xAD\x8Da\x07SV[\x94P\x84a\n\xAA`\x1Ea\x1C(V[\x12\x80\x15a\n\xB7WP`\x1E\x85\x12[\x95P\x93\x97P\x93\x97\x91\x95P\x93PV[`\0\x81` \x01Q\x82`@\x01Q\x03a\n\xDBWPQ\x90V[`\0\x82` \x01QB\x11a\n\xEEWBa\n\xF4V[\x82` \x01Q[\x90P`\0\x83`@\x01Q\x82a\x0B\x08\x91\x90a\x1B\xDBV[\x90P`\0\x84``\x01Q\x13\x15a\x0B:W``\x84\x01Qa\x0B&\x90\x82a\x1D\x17V[\x84Qa\x0B2\x91\x90a\x1B\xEEV[\x94\x93PPPPV[\x83``\x01Qa\x0BH\x90a\x1C(V[a\x0BR\x90\x82a\x1D\x17V[\x84Qa\x0B2\x91\x90a\x1B\xDBV[`\0a\t\xF5\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x0C\x82V[`\0g\x06\xF0[Y\xD3\xB2\0\0\x82\x03a\x0B\x8CWP`\0\x91\x90PV[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x12a\x0B\xB4W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\x0B\xD5W`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x0B\xE2\x83`\x02a\x1D.V[\x90P`\0a\x0B\xEF\x82a\x0C\xA1V[\x90P`\0a\x0C\x05g\x13\xA0K\xBD\xFD\xC9\xBE\x88\x83a\x0F\x1FV[\x90Pa\x0C\x10\x81a\x1C(V[\x95\x94PPPPPV[`\0a\t\xF5\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x0C\x82V[`\0\x80a\x0C:\x83a\x0F4V[a\x0CH\x90c;\x9A\xCA\0a\x1D\x17V[\x90Pa\x0B2\x84\x82a\x0C\x19V[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x0ClW`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x0C\x9AW`\0\x80\xFD[\x04\x92\x91PPV[`\0\x80\x82\x12\x80a\x0C\xB8WPg\x1B\xC1mgN\xC8\0\0\x82\x13[\x15a\x0C\xD6W`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\x0C\xF7W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81g\x1B\xC1mgN\xC8\0\0\x03a\r\x1FW`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x15a\r*W\x91\x90PV[`\0g\r\xE0\xB6\xB3\xA7d\0\0\x83\x12a\rRWa\rM\x83g\x1B\xC1mgN\xC8\0\0a\x1C\x01V[a\rTV[\x82[\x90P`\0a\rj\x82g\x1B\xC1mgN\xC8\0\0a\x0F\xD8V[\x90P\x80`\0\x03a\r\x8DW`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\r\x98\x82a\x0F\xEDV[\x90P`\0c;\x9A\xCA\0a\r\xC3a\r\xBEa\r\xB8g\x1B\xC1mgN\xC8\0\0a\x1C(V[\x85a\x0F\x1FV[a\x0F4V[a\r\xCD\x91\x90a\x1D.V[\x90P`\0\x80a\r\xE4\x83g\x03\xC1f\\z\xAB \0a\x0F\x1FV[a\r\xF6\x90g \x05\xFEO&\x8E\xA0\0a\x1CsV[\x90P`\0a\x0E&\x84a\x0E\x0F\x86f\x9F2u$b\xA0\0a\x0F\x1FV[a\x0E!\x90g\r\xC5R\x7Fd, \0a\x1CsV[a\x0F\x1FV[a\x0E8\x90g\r\xE0\xB6\xB3\xA7d\0\0a\x1CsV[\x90Pa\x0E\\g\t\xD0(\xCCo _\xFF\x19\x85a\x0ER\x85\x85a\x0F\xD8V[a\x0E!\x91\x90a\x1C\x01V[\x92PPP`\0[`\x02\x81\x10\x15a\x0E\xF7W`\0\x86a\x0Ex\x84a\x11\xC8V[a\x0E\x82\x91\x90a\x1C\x01V[\x90P`\0a\x0E\x90\x84\x85a\x0F\x1FV[a\x0E\x99\x90a\x1C(V[\x90P`\0a\x0E\xA6\x82a\x13\xACV[\x90P`\0a\x0E\xB4\x86\x85a\x0F\x1FV[a\x0E\xC6g\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x84a\x0F\x1FV[a\x0E\xD0\x91\x90a\x1C\x01V[\x90Pa\x0E\xDC\x84\x82a\x0F\xD8V[a\x0E\xE6\x90\x87a\x1CsV[\x95P\x84`\x01\x01\x94PPPPPa\x0EcV[g\r\xE0\xB6\xB3\xA7d\0\0\x88\x12a\x0F\x14Wa\x0F\x0F\x82a\x1C(V[a\t\xD4V[P\x96\x95PPPPPPV[`\0a\t\xF5\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x15UV[`\xB5\x81`\x01`\x88\x1B\x81\x10a\x0FMW`@\x91\x90\x91\x1B\x90`\x80\x1C[i\x01\0\0\0\0\0\0\0\0\0\x81\x10a\x0FiW` \x91\x90\x91\x1B\x90`@\x1C[e\x01\0\0\0\0\0\x81\x10a\x0F\x81W`\x10\x91\x90\x91\x1B\x90` \x1C[c\x01\0\0\0\x81\x10a\x0F\x97W`\x08\x91\x90\x91\x1B\x90`\x10\x1C[b\x01\0\0\x01\x02`\x12\x1C\x80\x82\x04\x01`\x01\x90\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x90\x81\x90\x04\x81\x11\x90\x03\x90V[`\0a\t\xF5\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x15UV[`\0\x80\x82\x13a\x10*W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x06|V[`\0``a\x107\x84a\x15tV[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[`\0\x81`\0\x03a\x11\xE1WPg\r\xE0\xB6\xB3\xA7d\0\0\x91\x90PV[gV\x98\xEE\xF0fp\0\0\x82\x12a\x11\xF8WP`\0\x91\x90PV[a\x12\tgV\x98\xEE\xF0fp\0\0a\x1C(V[\x82\x13a\x12\x1EWPg\x1B\xC1mgN\xC8\0\0\x91\x90PV[`\0a\x12)\x83a\x16\x1CV[\x90P`\0a\x12bg\r\xE0\xB6\xB3\xA7d\0\0a\x12K\x84g\x1B\xC1mgN\xC8\0\0a\x0B^V[a\x12]\x90g\r\xE0\xB6\xB3\xA7d\0\0a\x1CsV[a\x0F\xD8V[\x90P`\0\x80\x82a\x12\xBE\x81a\x12\xAB\x81a\x12\x99\x81a\x12\x86\x81g\x02_\x0F\xE1\x05\xA3\x14\0a\x0F\x1FV[a\x0E!\x90g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19a\x1CsV[a\x0E!\x90g\x14\xA8EL\x19\xE1\xAC\0a\x1CsV[a\x0E!\x90g\x0F\xC1\x0E\x01W\x82w\xFF\x19a\x1CsV[a\x12\xD0\x90g\x03\xDE\xBD\x08;\x8C|\0a\x1CsV[\x91P\x83\x90Pa\x138\x81a\x13&\x81a\x13\x14\x81a\x13\x02\x81a\x12\xEF\x81\x8Ba\x0F\x1FV[a\x0E!\x90g\x02\x95\xD4\0\xEA2W\xFF\x19a\x1CsV[a\x0E!\x90g\x01W\xD8\xB2\xEC\xC7\x08\0a\x1CsV[a\x0E!\x90g\x051\n\xA7\xD5!0\0a\x1CsV[a\x0E!\x90g\r\xE0\xCC=\x15a\0\0a\x1CsV[\x91P\x81g\x11\x90\0\xAB\x10\x0F\xFC\0a\x13N\x87\x88a\x0F\x1FV[a\x13Z\x90`\0\x19a\x1D.V[a\x13d\x91\x90a\x1C\x01V[a\x13n\x91\x90a\x1CsV[\x92PP`\0a\x13|\x83a\x13\xACV[\x90P`\0a\x13\x8A\x85\x83a\x0F\x1FV[\x90P`\0\x88\x12a\x13\x9AW\x80a\t\xD4V[a\t\xD4\x81g\x1B\xC1mgN\xC8\0\0a\x1C\x01V[`\0h\x02H\xCE6\xA7\x0C\xB2k>\x19\x82\x13a\x13\xC7WP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a\x14\x0EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x01a\x06|V[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05`\x01`_\x1B\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1Dm\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1Dn\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x05\x85\x14\x17\x16a\x15mW`\0\x80\xFD[\x05\x92\x91PPV[`\0\x80\x82\x11a\x15\xB1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x06|V[P`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x82\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[`\0`\x01`\xFF\x1B\x82\x03a\x16BW`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x12\x15a\x16SWP\x19`\x01\x01\x90V[P\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x81\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x80\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x17\x89Wa\x17\x89a\x17PV[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x17\xB8Wa\x17\xB8a\x17PV[`@R\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x17\xD6Wa\x17\xD6a\x16WV[\x825\x91P` \x80\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x17\xF9Wa\x17\xF9a\x16\xA7V[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x18\x10Wa\x18\x10a\x16\xF7V[\x815\x81\x81\x11\x15a\x18\"Wa\x18\"a\x17PV[a\x184`\x1F\x82\x01`\x1F\x19\x16\x85\x01a\x17\x8FV[\x91P\x80\x82R\x87\x84\x82\x85\x01\x01\x11\x15a\x18\x9AW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x85\x90R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R`\x84\x81\xFD[\x80\x84\x84\x01\x85\x84\x017`\0\x84\x82\x84\x01\x01RP\x80\x93PPPP\x92P\x92\x90PV[`\0\x80`\0`@\x84\x86\x03\x12\x15a\x18\xD0Wa\x18\xD0a\x16WV[\x835\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x18\xF2Wa\x18\xF2a\x16\xA7V[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x19\tWa\x19\ta\x16\xF7V[\x815\x81\x81\x11\x15a\x19lW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x0ENL/$\r\x8C\xAD\xCC\xEE\x8D`\xAB\x1B`d\x82\x01R`\x84\x81\xFD[\x87` \x82\x85\x01\x01\x11\x15a\x19\xD4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rjrray stride`\xA8\x1B`d\x82\x01R\x91P`\x84\x82\xFD[` \x83\x01\x94P\x80\x93PPPP\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a\x19\xFCWa\x19\xFCa\x16WV[P5\x91\x90PV[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a\x1A0W\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a\x1A\x14V[P`\0`@\x82\x86\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x92PPP\x92\x91PPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x1AiWa\x1Aia\x16WV[\x83Q\x92P` \x84\x01Q\x91P`@\x84\x01Q\x90P\x92P\x92P\x92V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FABI decoding: struct data too sh`D\x82\x01Rb\x1B\xDC\x9D`\xEA\x1B`d\x82\x01R`\x84\x81\xFD[`\0`\x80\x82\x84\x03\x12\x15a\x1A\xE8Wa\x1A\xE8a\x16WV[a\x1A\xF0a\x17fV[\x82Q\x81R` \x83\x01Q` \x82\x01R`@\x83\x01Q`@\x82\x01R``\x83\x01Q``\x82\x01R\x80\x91PP\x92\x91PPV[`\0`\x80\x82\x84\x03\x12\x15a\x1B1Wa\x1B1a\x1A\x82V[a\x1B9a\x17fV[\x90P\x815\x81R` \x82\x015` \x82\x01R`@\x82\x015`@\x82\x01R``\x82\x015``\x82\x01R\x92\x91PPV[`\0a\x01\xA0\x82\x84\x03\x12\x15a\x1ByWa\x1Bya\x16WV[a\x1B\x81a\x17fV[a\x1B\x8B\x84\x84a\x1B\x1CV[\x81Ra\x1B\x9A\x84`\x80\x85\x01a\x1B\x1CV[` \x82\x01Ra\x1B\xAD\x84a\x01\0\x85\x01a\x1B\x1CV[`@\x82\x01Ra\x01\x80\x92\x90\x92\x015``\x83\x01RP\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x03\xCBWa\x03\xCBa\x1B\xC5V[\x80\x82\x01\x80\x82\x11\x15a\x03\xCBWa\x03\xCBa\x1B\xC5V[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a\x1C!Wa\x1C!a\x1B\xC5V[P\x92\x91PPV[`\0`\x01`\xFF\x1B\x82\x01a\x1C=Wa\x1C=a\x1B\xC5V[P`\0\x03\x90V[`\0\x80`\0``\x84\x86\x03\x12\x15a\x1C\\Wa\x1C\\a\x16WV[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[\x80\x82\x01\x82\x81\x12`\0\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a\x1C\x93Wa\x1C\x93a\x1B\xC5V[PP\x92\x91PPV[`\0\x80`\0\x80\x84\x86\x03`\xE0\x81\x12\x15a\x1C\xB5Wa\x1C\xB5a\x16WV[\x855\x94P` \x86\x015\x93P`@\x86\x015\x92P`\x80`_\x19\x82\x01\x12\x15a\x1C\xDCWa\x1C\xDCa\x1A\x82V[Pa\x1C\xE5a\x17fV[``\x86\x015\x81R`\x80\x86\x015` \x82\x01R`\xA0\x86\x015`@\x82\x01R`\xC0\x86\x015``\x82\x01R\x80\x91PP\x92\x95\x91\x94P\x92PV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x03\xCBWa\x03\xCBa\x1B\xC5V[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15a\x1DJWa\x1DJa\x1B\xC5V[\x81\x81\x05\x83\x14\x82\x15\x17a\x03\xCBWa\x03\xCBa\x1B\xC5V";
    /// The bytecode of the contract.
    pub static LOGNORMAL_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\0\xD4W`\x005`\xE0\x1C\x80c\x8E-\xD4\0\x11a\0\xA8W\x80c\x8E-\xD4\0\x14a\x02\xE3W\x80c\x9F\x83\x13{\x14a\x03 W\x80c\xAF\xBA\x13\xC4\x14a\x033W\x80c\xDC\x17\x83U\x14a\x03^Wa\0\xD4V[\x80b.RK\x14a\x019W\x80c\x02\x16\xB88\x14a\x01_W\x80c\x1E\xDBq\xE5\x14a\x01tW\x80c2\x14\x89\x0F\x14a\x02\xA1W[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x01La\x01G6`\x04a\x17\xC0V[a\x03~V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01ra\x01m6`\x04a\x18\xB8V[a\x03\xD1V[\0[a\x02%a\x01\x826`\x04a\x19\xE7V[`\x01` \x81\x81R`\0\x92\x83R`@\x92\x83\x90 \x83Q`\x80\x80\x82\x01\x86R\x82T\x82R\x93\x82\x01T\x81\x84\x01R`\x02\x82\x01T\x81\x86\x01R`\x03\x82\x01T``\x80\x83\x01\x91\x90\x91R\x85Q\x80\x86\x01\x87R`\x04\x84\x01T\x81R`\x05\x84\x01T\x81\x86\x01R`\x06\x84\x01T\x81\x88\x01R`\x07\x84\x01T\x81\x83\x01R\x86Q\x95\x86\x01\x87R`\x08\x84\x01T\x86R`\t\x84\x01T\x94\x86\x01\x94\x90\x94R`\n\x83\x01T\x95\x85\x01\x95\x90\x95R`\x0B\x82\x01T\x94\x84\x01\x94\x90\x94R`\x0C\x01T\x90\x91\x90\x84V[`@\x80Q\x85Q\x81R` \x80\x87\x01Q\x81\x83\x01R\x86\x83\x01Q\x82\x84\x01R``\x96\x87\x01Q\x87\x83\x01R\x85Q`\x80\x83\x01R\x80\x86\x01Q`\xA0\x83\x01R\x85\x83\x01Q`\xC0\x83\x01R\x94\x86\x01Q`\xE0\x82\x01R\x83Qa\x01\0\x82\x01R\x93\x83\x01Qa\x01 \x85\x01R\x82\x01Qa\x01@\x84\x01R\x92\x01Qa\x01`\x82\x01Ra\x01\x80\x81\x01\x91\x90\x91Ra\x01\xA0\x01a\x01VV[a\x02\xB4a\x02\xAF6`\x04a\x17\xC0V[a\x04pV[`@\x80Q\x96\x15\x15\x87R` \x87\x01\x95\x90\x95R\x93\x85\x01\x92\x90\x92R``\x84\x01R`\x80\x83\x01R`\xA0\x82\x01R`\xC0\x01a\x01VV[a\x02\xF6a\x02\xF16`\x04a\x18\xB8V[a\x06\xDCV[`@\x80Q\x95\x15\x15\x86R` \x86\x01\x94\x90\x94R\x92\x84\x01\x91\x90\x91R``\x83\x01R`\x80\x82\x01R`\xA0\x01a\x01VV[a\x02\xF6a\x03.6`\x04a\x18\xB8V[a\x07+V[`\0Ta\x03F\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01VV[a\x03qa\x03l6`\x04a\x19\xE7V[a\x07SV[`@Qa\x01V\x91\x90a\x1A\x03V[`\0\x80`\0\x80\x84\x80` \x01\x90Q\x81\x01\x90a\x03\x98\x91\x90a\x1AQV[\x92P\x92P\x92Pa\x03\xC5\x83\x83\x83a\x03\xAD\x8Aa\x07SV[\x80` \x01\x90Q\x81\x01\x90a\x03\xC0\x91\x90a\x1A\xD3V[a\x08\xCEV[\x93PPPP[\x92\x91PPV[`\0a\x03\xDF\x82\x84\x01\x84a\x1BcV[`\0\x94\x85R`\x01` \x81\x81R`@\x96\x87\x90 \x83Q\x80Q\x82U\x80\x83\x01Q\x93\x82\x01\x93\x90\x93U\x82\x88\x01Q`\x02\x82\x01U``\x92\x83\x01Q`\x03\x82\x01U\x81\x84\x01Q\x80Q`\x04\x83\x01U\x80\x83\x01Q`\x05\x83\x01U\x80\x89\x01Q`\x06\x83\x01U\x83\x01Q`\x07\x82\x01U\x87\x84\x01Q\x80Q`\x08\x83\x01U\x91\x82\x01Q`\t\x82\x01U\x96\x81\x01Q`\n\x88\x01U\x81\x01Q`\x0B\x87\x01U\x01Q`\x0C\x90\x94\x01\x93\x90\x93UPPPV[`\0\x80`\0\x80`\0\x80`\0a\x04\x84\x89a\x07SV[\x80` \x01\x90Q\x81\x01\x90a\x04\x97\x91\x90a\x1A\xD3V[`\0\x80T`@Qc3\x85N\xFD`\xE2\x1B\x81R`\x04\x81\x01\x8D\x90R\x92\x93P\x90\x91\x82\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xCE\x15;\xF4\x90`$\x01```@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x055W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x05IW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05m\x91\x90a\x1AQV[\x92P\x92P\x92P\x8A\x80` \x01\x90Q\x81\x01\x90a\x05\x87\x91\x90a\x1AQV[\x91\x98P\x96P\x94P`\0\x80\x80\x85\x8A\x11\x15a\x05\xE4Wa\x05\xA4\x86\x8Ba\x1B\xDBV[\x91Pa\x05\xBD\x87``\x01Q\x83a\t\xE0\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa\x05\xD3\x86a\x05\xCD\x83\x87a\t\xE0V[\x90a\t\xFCV[a\x05\xDD\x90\x84a\x1B\xEEV[\x92Pa\x06\x85V[\x84\x89\x11\x15a\x06\x1FWa\x05\xF6\x85\x8Aa\x1B\xDBV[\x91Pa\x06\x0F\x87``\x01Q\x83a\t\xE0\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa\x05\xD3\x85a\x05\xCD\x83\x87a\t\xE0V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`0`$\x82\x01R\x7Finvalid swap: inputs x and y hav`D\x82\x01Roe the same sign!`\x80\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[a\x06\x8F\x84\x89a\x1C\x01V[\x9APa\x06\x9D\x8A\x8A\x8A\x8Aa\x08\xCEV[\x9BP`\0\x8Ca\x06\xAC`\x1Ea\x1C(V[\x12\x80\x15a\x06\xB9WP`\x1E\x8D\x12[\x90P\x80\x80\x15a\x06\xC8WP\x83\x8C\x12\x15[\x9DPPPPPPPPP\x92\x95P\x92\x95P\x92\x95V[`\0\x80\x80\x80\x80a\x06\xEE\x86\x88\x01\x88a\x1CDV[\x91\x94P\x92P\x90Pa\x07\x04\x83\x83\x83a\x03\xAD\x8Ca\x07SV[\x93P\x83a\x07\x11`\x1Ea\x1C(V[\x12\x80\x15a\x07\x1EWP`\x1E\x84\x12[\x94P\x93\x97\x92\x96P\x93P\x93PV[`\0\x80`\0\x80`\0a\x07>\x88\x88\x88a\n\x11V[P\x93\x9C\x92\x9BP\x90\x99P\x97P\x90\x95P\x93PPPPV[``a\x07\x80`@Q\x80`\x80\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[a\x07\xD1`\x01`\0\x85\x81R` \x01\x90\x81R` \x01`\0 `\0\x01`@Q\x80`\x80\x01`@R\x90\x81`\0\x82\x01T\x81R` \x01`\x01\x82\x01T\x81R` \x01`\x02\x82\x01T\x81R` \x01`\x03\x82\x01T\x81RPPa\n\xC5V[` \x80\x83\x01\x91\x90\x91R`\0\x84\x81R`\x01\x82R`@\x90\x81\x90 \x81Q`\x80\x81\x01\x83R`\x08\x82\x01T\x81R`\t\x82\x01T\x93\x81\x01\x93\x90\x93R`\n\x81\x01T\x91\x83\x01\x91\x90\x91R`\x0B\x01T``\x82\x01Ra\x08\"\x90a\n\xC5V[\x81R`\0\x83\x81R`\x01` \x90\x81R`@\x91\x82\x90 \x82Q`\x80\x81\x01\x84R`\x04\x82\x01T\x81R`\x05\x82\x01T\x92\x81\x01\x92\x90\x92R`\x06\x81\x01T\x92\x82\x01\x92\x90\x92R`\x07\x90\x91\x01T``\x82\x01Ra\x08q\x90a\n\xC5V[`@\x82\x81\x01\x91\x82R`\0\x94\x85R`\x01` \x90\x81R\x94\x81\x90 `\x0C\x01T``\x80\x85\x01\x91\x82R\x82Q\x85Q\x81\x89\x01R\x96\x90\x94\x01Q\x86\x83\x01R\x91Q\x92\x85\x01\x92\x90\x92RQ`\x80\x80\x85\x01\x91\x90\x91R\x81Q\x80\x85\x03\x90\x91\x01\x81R`\xA0\x90\x93\x01\x90RP\x90V[`\0\x82\x85\x10a\t\x1FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FtradingFunction: invalid x\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06|V[`\0\x80g\r\xE0\xB6\xB3\xA7d\0\0a\t5\x88\x87a\x0B^V[\x10a\tIW`\x01`\x01`\xFF\x1B\x03\x91Pa\t^V[a\t[a\tV\x88\x87a\x0B^V[a\x0BsV[\x91P[g\r\xE0\xB6\xB3\xA7d\0\0a\t~\x87a\ty\x87`\0\x01Q\x89a\x0C\x19V[a\x0B^V[\x10a\t\x91WP`\x01`\x01`\xFF\x1B\x03a\t\xA9V[a\t\xA6a\tV\x87a\ty\x87`\0\x01Q\x89a\x0C\x19V[\x90P[`\0a\t\xBD\x85` \x01Q\x86`@\x01Qa\x0C.V[\x90P\x80a\t\xCA\x83\x85a\x1CsV[a\t\xD4\x91\x90a\x1CsV[\x98\x97PPPPPPPPV[`\0a\t\xF5\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x0CTV[\x93\x92PPPV[`\0a\t\xF5\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x0CTV[`\0\x80`\0\x80`\0a\nD`@Q\x80`\x80\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[a\nP\x87\x89\x01\x89a\x1C\x9BV[` \x80\x82\x01Q`\0\x8F\x81R`\x01\x90\x92R`@\x91\x82\x90 \x90\x81U\x90\x82\x01Q`\x04\x82\x01U\x81Q`\x08\x82\x01U``\x82\x01Q`\x0C\x90\x91\x01U\x92\x96P\x90\x94P\x92P\x90Pa\n\x9D\x84\x84\x84a\x03\xAD\x8Da\x07SV[\x94P\x84a\n\xAA`\x1Ea\x1C(V[\x12\x80\x15a\n\xB7WP`\x1E\x85\x12[\x95P\x93\x97P\x93\x97\x91\x95P\x93PV[`\0\x81` \x01Q\x82`@\x01Q\x03a\n\xDBWPQ\x90V[`\0\x82` \x01QB\x11a\n\xEEWBa\n\xF4V[\x82` \x01Q[\x90P`\0\x83`@\x01Q\x82a\x0B\x08\x91\x90a\x1B\xDBV[\x90P`\0\x84``\x01Q\x13\x15a\x0B:W``\x84\x01Qa\x0B&\x90\x82a\x1D\x17V[\x84Qa\x0B2\x91\x90a\x1B\xEEV[\x94\x93PPPPV[\x83``\x01Qa\x0BH\x90a\x1C(V[a\x0BR\x90\x82a\x1D\x17V[\x84Qa\x0B2\x91\x90a\x1B\xDBV[`\0a\t\xF5\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x0C\x82V[`\0g\x06\xF0[Y\xD3\xB2\0\0\x82\x03a\x0B\x8CWP`\0\x91\x90PV[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x12a\x0B\xB4W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\x0B\xD5W`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x0B\xE2\x83`\x02a\x1D.V[\x90P`\0a\x0B\xEF\x82a\x0C\xA1V[\x90P`\0a\x0C\x05g\x13\xA0K\xBD\xFD\xC9\xBE\x88\x83a\x0F\x1FV[\x90Pa\x0C\x10\x81a\x1C(V[\x95\x94PPPPPV[`\0a\t\xF5\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x0C\x82V[`\0\x80a\x0C:\x83a\x0F4V[a\x0CH\x90c;\x9A\xCA\0a\x1D\x17V[\x90Pa\x0B2\x84\x82a\x0C\x19V[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x0ClW`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x0C\x9AW`\0\x80\xFD[\x04\x92\x91PPV[`\0\x80\x82\x12\x80a\x0C\xB8WPg\x1B\xC1mgN\xC8\0\0\x82\x13[\x15a\x0C\xD6W`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\x0C\xF7W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81g\x1B\xC1mgN\xC8\0\0\x03a\r\x1FW`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x15a\r*W\x91\x90PV[`\0g\r\xE0\xB6\xB3\xA7d\0\0\x83\x12a\rRWa\rM\x83g\x1B\xC1mgN\xC8\0\0a\x1C\x01V[a\rTV[\x82[\x90P`\0a\rj\x82g\x1B\xC1mgN\xC8\0\0a\x0F\xD8V[\x90P\x80`\0\x03a\r\x8DW`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\r\x98\x82a\x0F\xEDV[\x90P`\0c;\x9A\xCA\0a\r\xC3a\r\xBEa\r\xB8g\x1B\xC1mgN\xC8\0\0a\x1C(V[\x85a\x0F\x1FV[a\x0F4V[a\r\xCD\x91\x90a\x1D.V[\x90P`\0\x80a\r\xE4\x83g\x03\xC1f\\z\xAB \0a\x0F\x1FV[a\r\xF6\x90g \x05\xFEO&\x8E\xA0\0a\x1CsV[\x90P`\0a\x0E&\x84a\x0E\x0F\x86f\x9F2u$b\xA0\0a\x0F\x1FV[a\x0E!\x90g\r\xC5R\x7Fd, \0a\x1CsV[a\x0F\x1FV[a\x0E8\x90g\r\xE0\xB6\xB3\xA7d\0\0a\x1CsV[\x90Pa\x0E\\g\t\xD0(\xCCo _\xFF\x19\x85a\x0ER\x85\x85a\x0F\xD8V[a\x0E!\x91\x90a\x1C\x01V[\x92PPP`\0[`\x02\x81\x10\x15a\x0E\xF7W`\0\x86a\x0Ex\x84a\x11\xC8V[a\x0E\x82\x91\x90a\x1C\x01V[\x90P`\0a\x0E\x90\x84\x85a\x0F\x1FV[a\x0E\x99\x90a\x1C(V[\x90P`\0a\x0E\xA6\x82a\x13\xACV[\x90P`\0a\x0E\xB4\x86\x85a\x0F\x1FV[a\x0E\xC6g\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x84a\x0F\x1FV[a\x0E\xD0\x91\x90a\x1C\x01V[\x90Pa\x0E\xDC\x84\x82a\x0F\xD8V[a\x0E\xE6\x90\x87a\x1CsV[\x95P\x84`\x01\x01\x94PPPPPa\x0EcV[g\r\xE0\xB6\xB3\xA7d\0\0\x88\x12a\x0F\x14Wa\x0F\x0F\x82a\x1C(V[a\t\xD4V[P\x96\x95PPPPPPV[`\0a\t\xF5\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x15UV[`\xB5\x81`\x01`\x88\x1B\x81\x10a\x0FMW`@\x91\x90\x91\x1B\x90`\x80\x1C[i\x01\0\0\0\0\0\0\0\0\0\x81\x10a\x0FiW` \x91\x90\x91\x1B\x90`@\x1C[e\x01\0\0\0\0\0\x81\x10a\x0F\x81W`\x10\x91\x90\x91\x1B\x90` \x1C[c\x01\0\0\0\x81\x10a\x0F\x97W`\x08\x91\x90\x91\x1B\x90`\x10\x1C[b\x01\0\0\x01\x02`\x12\x1C\x80\x82\x04\x01`\x01\x90\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x90\x81\x90\x04\x81\x11\x90\x03\x90V[`\0a\t\xF5\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x15UV[`\0\x80\x82\x13a\x10*W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x06|V[`\0``a\x107\x84a\x15tV[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[`\0\x81`\0\x03a\x11\xE1WPg\r\xE0\xB6\xB3\xA7d\0\0\x91\x90PV[gV\x98\xEE\xF0fp\0\0\x82\x12a\x11\xF8WP`\0\x91\x90PV[a\x12\tgV\x98\xEE\xF0fp\0\0a\x1C(V[\x82\x13a\x12\x1EWPg\x1B\xC1mgN\xC8\0\0\x91\x90PV[`\0a\x12)\x83a\x16\x1CV[\x90P`\0a\x12bg\r\xE0\xB6\xB3\xA7d\0\0a\x12K\x84g\x1B\xC1mgN\xC8\0\0a\x0B^V[a\x12]\x90g\r\xE0\xB6\xB3\xA7d\0\0a\x1CsV[a\x0F\xD8V[\x90P`\0\x80\x82a\x12\xBE\x81a\x12\xAB\x81a\x12\x99\x81a\x12\x86\x81g\x02_\x0F\xE1\x05\xA3\x14\0a\x0F\x1FV[a\x0E!\x90g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19a\x1CsV[a\x0E!\x90g\x14\xA8EL\x19\xE1\xAC\0a\x1CsV[a\x0E!\x90g\x0F\xC1\x0E\x01W\x82w\xFF\x19a\x1CsV[a\x12\xD0\x90g\x03\xDE\xBD\x08;\x8C|\0a\x1CsV[\x91P\x83\x90Pa\x138\x81a\x13&\x81a\x13\x14\x81a\x13\x02\x81a\x12\xEF\x81\x8Ba\x0F\x1FV[a\x0E!\x90g\x02\x95\xD4\0\xEA2W\xFF\x19a\x1CsV[a\x0E!\x90g\x01W\xD8\xB2\xEC\xC7\x08\0a\x1CsV[a\x0E!\x90g\x051\n\xA7\xD5!0\0a\x1CsV[a\x0E!\x90g\r\xE0\xCC=\x15a\0\0a\x1CsV[\x91P\x81g\x11\x90\0\xAB\x10\x0F\xFC\0a\x13N\x87\x88a\x0F\x1FV[a\x13Z\x90`\0\x19a\x1D.V[a\x13d\x91\x90a\x1C\x01V[a\x13n\x91\x90a\x1CsV[\x92PP`\0a\x13|\x83a\x13\xACV[\x90P`\0a\x13\x8A\x85\x83a\x0F\x1FV[\x90P`\0\x88\x12a\x13\x9AW\x80a\t\xD4V[a\t\xD4\x81g\x1B\xC1mgN\xC8\0\0a\x1C\x01V[`\0h\x02H\xCE6\xA7\x0C\xB2k>\x19\x82\x13a\x13\xC7WP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a\x14\x0EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x01a\x06|V[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05`\x01`_\x1B\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1Dm\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1Dn\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x05\x85\x14\x17\x16a\x15mW`\0\x80\xFD[\x05\x92\x91PPV[`\0\x80\x82\x11a\x15\xB1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x06|V[P`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x82\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[`\0`\x01`\xFF\x1B\x82\x03a\x16BW`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x12\x15a\x16SWP\x19`\x01\x01\x90V[P\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x81\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x80\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x17\x89Wa\x17\x89a\x17PV[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x17\xB8Wa\x17\xB8a\x17PV[`@R\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x17\xD6Wa\x17\xD6a\x16WV[\x825\x91P` \x80\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x17\xF9Wa\x17\xF9a\x16\xA7V[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x18\x10Wa\x18\x10a\x16\xF7V[\x815\x81\x81\x11\x15a\x18\"Wa\x18\"a\x17PV[a\x184`\x1F\x82\x01`\x1F\x19\x16\x85\x01a\x17\x8FV[\x91P\x80\x82R\x87\x84\x82\x85\x01\x01\x11\x15a\x18\x9AW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x85\x90R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R`\x84\x81\xFD[\x80\x84\x84\x01\x85\x84\x017`\0\x84\x82\x84\x01\x01RP\x80\x93PPPP\x92P\x92\x90PV[`\0\x80`\0`@\x84\x86\x03\x12\x15a\x18\xD0Wa\x18\xD0a\x16WV[\x835\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x18\xF2Wa\x18\xF2a\x16\xA7V[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x19\tWa\x19\ta\x16\xF7V[\x815\x81\x81\x11\x15a\x19lW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x0ENL/$\r\x8C\xAD\xCC\xEE\x8D`\xAB\x1B`d\x82\x01R`\x84\x81\xFD[\x87` \x82\x85\x01\x01\x11\x15a\x19\xD4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rjrray stride`\xA8\x1B`d\x82\x01R\x91P`\x84\x82\xFD[` \x83\x01\x94P\x80\x93PPPP\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a\x19\xFCWa\x19\xFCa\x16WV[P5\x91\x90PV[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a\x1A0W\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a\x1A\x14V[P`\0`@\x82\x86\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x92PPP\x92\x91PPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x1AiWa\x1Aia\x16WV[\x83Q\x92P` \x84\x01Q\x91P`@\x84\x01Q\x90P\x92P\x92P\x92V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FABI decoding: struct data too sh`D\x82\x01Rb\x1B\xDC\x9D`\xEA\x1B`d\x82\x01R`\x84\x81\xFD[`\0`\x80\x82\x84\x03\x12\x15a\x1A\xE8Wa\x1A\xE8a\x16WV[a\x1A\xF0a\x17fV[\x82Q\x81R` \x83\x01Q` \x82\x01R`@\x83\x01Q`@\x82\x01R``\x83\x01Q``\x82\x01R\x80\x91PP\x92\x91PPV[`\0`\x80\x82\x84\x03\x12\x15a\x1B1Wa\x1B1a\x1A\x82V[a\x1B9a\x17fV[\x90P\x815\x81R` \x82\x015` \x82\x01R`@\x82\x015`@\x82\x01R``\x82\x015``\x82\x01R\x92\x91PPV[`\0a\x01\xA0\x82\x84\x03\x12\x15a\x1ByWa\x1Bya\x16WV[a\x1B\x81a\x17fV[a\x1B\x8B\x84\x84a\x1B\x1CV[\x81Ra\x1B\x9A\x84`\x80\x85\x01a\x1B\x1CV[` \x82\x01Ra\x1B\xAD\x84a\x01\0\x85\x01a\x1B\x1CV[`@\x82\x01Ra\x01\x80\x92\x90\x92\x015``\x83\x01RP\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x03\xCBWa\x03\xCBa\x1B\xC5V[\x80\x82\x01\x80\x82\x11\x15a\x03\xCBWa\x03\xCBa\x1B\xC5V[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a\x1C!Wa\x1C!a\x1B\xC5V[P\x92\x91PPV[`\0`\x01`\xFF\x1B\x82\x01a\x1C=Wa\x1C=a\x1B\xC5V[P`\0\x03\x90V[`\0\x80`\0``\x84\x86\x03\x12\x15a\x1C\\Wa\x1C\\a\x16WV[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[\x80\x82\x01\x82\x81\x12`\0\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a\x1C\x93Wa\x1C\x93a\x1B\xC5V[PP\x92\x91PPV[`\0\x80`\0\x80\x84\x86\x03`\xE0\x81\x12\x15a\x1C\xB5Wa\x1C\xB5a\x16WV[\x855\x94P` \x86\x015\x93P`@\x86\x015\x92P`\x80`_\x19\x82\x01\x12\x15a\x1C\xDCWa\x1C\xDCa\x1A\x82V[Pa\x1C\xE5a\x17fV[``\x86\x015\x81R`\x80\x86\x015` \x82\x01R`\xA0\x86\x015`@\x82\x01R`\xC0\x86\x015``\x82\x01R\x80\x91PP\x92\x95\x91\x94P\x92PV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x03\xCBWa\x03\xCBa\x1B\xC5V[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15a\x1DJWa\x1DJa\x1B\xC5V[\x81\x81\x05\x83\x14\x82\x15\x17a\x03\xCBWa\x03\xCBa\x1B\xC5V";
    /// The deployed bytecode of the contract.
    pub static LOGNORMAL_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
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
            f.debug_tuple(::core::stringify!(LogNormal)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> LogNormal<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    LOGNORMAL_ABI.clone(),
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
        ///Calls the contract's `init` (0x9f83137b) function
        pub fn init(
            &self,
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
                .method_hash([159, 131, 19, 123], (pool_id, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `internalParams` (0x1edb71e5) function
        pub fn internal_params(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (DynamicParam, DynamicParam, DynamicParam, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([30, 219, 113, 229], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `update` (0x0216b838) function
        pub fn update(
            &self,
            pool_id: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([2, 22, 184, 56], (pool_id, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `validateAllocateOrDeallocate` (0x8e2dd400) function
        pub fn validate_allocate_or_deallocate(
            &self,
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
                .method_hash([142, 45, 212, 0], (pool_id, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `validateSwap` (0x3214890f) function
        pub fn validate_swap(
            &self,
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
                .method_hash([50, 20, 137, 15], (pool_id, data))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for LogNormal<M> {
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
    pub enum LogNormalErrors {
        Infinity(Infinity),
        Min(Min),
        NegativeInfinity(NegativeInfinity),
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
    impl ::ethers::core::abi::AbiEncode for LogNormalErrors {
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
    impl ::ethers::contract::ContractRevert for LogNormalErrors {
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
    impl ::core::fmt::Display for LogNormalErrors {
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
    ///Container type for all input parameters for the `init` function with signature `init(uint256,bytes)` and selector `0x9f83137b`
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
    #[ethcall(name = "init", abi = "init(uint256,bytes)")]
    pub struct InitCall {
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
    ///Container type for all input parameters for the `update` function with signature `update(uint256,bytes)` and selector `0x0216b838`
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
    #[ethcall(name = "update", abi = "update(uint256,bytes)")]
    pub struct UpdateCall {
        pub pool_id: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `validateAllocateOrDeallocate` function with signature `validateAllocateOrDeallocate(uint256,bytes)` and selector `0x8e2dd400`
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
        abi = "validateAllocateOrDeallocate(uint256,bytes)"
    )]
    pub struct ValidateAllocateOrDeallocateCall {
        pub pool_id: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `validateSwap` function with signature `validateSwap(uint256,bytes)` and selector `0x3214890f`
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
    #[ethcall(name = "validateSwap", abi = "validateSwap(uint256,bytes)")]
    pub struct ValidateSwapCall {
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
    impl ::ethers::core::abi::AbiEncode for LogNormalCalls {
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
    impl ::core::fmt::Display for LogNormalCalls {
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
    ///Container type for all return fields from the `init` function with signature `init(uint256,bytes)` and selector `0x9f83137b`
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
        pub sigma: DynamicParam,
        pub tau: DynamicParam,
        pub strike: DynamicParam,
        pub swap_fee: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `validateAllocateOrDeallocate` function with signature `validateAllocateOrDeallocate(uint256,bytes)` and selector `0x8e2dd400`
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
    ///Container type for all return fields from the `validateSwap` function with signature `validateSwap(uint256,bytes)` and selector `0x3214890f`
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
