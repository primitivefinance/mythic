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
    const __BYTECODE: &[u8] = b"`\x80`@R`\x01\x80U4\x80\x15b\0\0bW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`@Qb\x002\xC58\x03\x80b\x002\xC5\x839\x81\x01`@\x81\x90Rb\0\0\x85\x91b\0\x01>V[`\x02\x80T`\x01`\x01`\xA0\x1B\x03\x80\x86\x16`\x01`\x01`\xA0\x1B\x03\x19\x92\x83\x16\x17\x90\x92U`\x03\x80T\x92\x85\x16\x92\x90\x91\x16\x91\x90\x91\x17\x90U`@Q\x81\x90b\0\0\xC5\x90b\0\x01\x13V[\x90\x81R` \x01`@Q\x80\x91\x03\x90`\0\xF0\x80\x15\x80\x15b\0\0\xE8W=`\0\x80>=`\0\xFD[P`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UPb\0\x01\xCA\x91PPV[a\x1E\xE1\x80b\0\x13\xE4\x839\x01\x90V[\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x019W`\0\x80\xFD[\x91\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15b\0\x01\x9FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[b\0\x01\xAA\x84b\0\x01!V[\x92Pb\0\x01\xBA` \x85\x01b\0\x01!V[\x91P`@\x84\x01Q\x90P\x92P\x92P\x92V[a\x12\n\x80b\0\x01\xDA`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x01\x01W`\x005`\xE0\x1C\x80cg\xE8(\xBF\x11a\0\xBEW\x80cg\xE8(\xBF\x14a\x02\x1DW\x80cp\xA0\x821\x14a\x020W\x80c\xB7\xD1\x9F\xC4\x14a\x02PW\x80c\xCF0\x90\x12\x14a\x02cW\x80c\xEB\xAD\xEF\x01\x14a\x02lW\x80c\xF0e\x95\x7F\x14a\x02zWa\x01\x01V[\x80c\x15w\x0F\x92\x14a\x01fW\x80c\x16\xDC\x16[\x14a\x01\x82W\x80cC\xC8\x85\xBA\x14a\x01\xADW\x80cM\xDFG\xD4\x14a\x01\xD1W\x80c^\"}X\x14a\x01\xFFW\x80cb}\xD5j\x14a\x02\x08W[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x01o`\x06T\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[`\x02Ta\x01\x95\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01yV[`\0Ta\x01\xC1\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x01yV[a\x01\xE4a\x01\xDF6`\x04a\x0E\xF5V[a\x02\x83V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x01yV[a\x01o`\x05T\x81V[a\x02\x1Ba\x02\x166`\x04a\x0E\xF5V[a\x05\xADV[\0[`\0Ta\x01\x95\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01oa\x02>6`\x04a\x10]V[`\x07` R`\0\x90\x81R`@\x90 T\x81V[`\x03Ta\x01\x95\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01o`\x01T\x81V[`\x04T`\x05T`\x06Ta\x01\xE4V[a\x01o`\x04T\x81V[`\0\x80`\0`\x01T`\x01\x14a\x02\xC8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x06`$\x82\x01Re\x1B\x1B\xD8\xDA\xD9Y`\xD2\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0`\x01\x81\x90U\x80T`@Qc\x13w\xD1\xF5`\xE2\x1B\x81R\x82\x91\x82\x91\x82\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cM\xDFG\xD4\x90a\x03\x08\x90\x8D\x90\x8D\x90`\x04\x01a\x10\x90V[`\xA0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x03`W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x11\xEA\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x03tW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\x98\x91\x90a\x10\xCFV[\x94P\x94P\x94P\x94P\x94P\x84a\x03\xD6W`\0\x84\x12a\x03\xB4\x85a\x07\xB2V[`@Qcw`m)`\xE1\x1B\x81R\x91\x15\x15`\x04\x83\x01R`$\x82\x01R`D\x01a\x02\xBFV[`\0\x80T`\xFF`\xA0\x1B\x19\x16`\x01`\xA0\x1B\x17\x81U`\x04\x84\x81U`\x05\x84\x90U`\x06\x83\x90U3\x80\x83R`\x07` R`@\x92\x83\x90 \x84\x90U`\x02T\x92Qc#\xB8r\xDD`\xE0\x1B\x81R\x91\x82\x01R0`$\x82\x01R`D\x81\x01\x85\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90`d\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x04\x97W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x11\xEA\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x04\xABW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xCF\x91\x90a\x11\x19V[P`\x03T`@Qc#\xB8r\xDD`\xE0\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90`d\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x05`W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x11\xEA\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x05tW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\x98\x91\x90a\x11\x19V[P`\x01\x80U\x91\x99\x90\x98P\x90\x96P\x94PPPPPV[`\x01T`\x01\x14a\x05\xE8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x06`$\x82\x01Re\x1B\x1B\xD8\xDA\xD9Y`\xD2\x1B`D\x82\x01R`d\x01a\x02\xBFV[`\0`\x01\x81\x90UT`\x01`\xA0\x1B\x90\x04`\xFF\x16a\x068W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rn\x1B\x9B\xDD\x08\x1A[\x9A]\x1AX[\x1A^\x99Y`\x8A\x1B`D\x82\x01R`d\x01a\x02\xBFV[`\0\x80T`@Qc\xC1nP\xEF`\xE0\x1B\x81R\x82\x91\x82\x91\x82\x91\x82\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xC1nP\xEF\x90a\x06s\x90\x8B\x90\x8B\x90`\x04\x01a\x10\x90V[`\xC0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x06\xC9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x11\xEA\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x06\xDDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x01\x91\x90a\x117V[\x95P\x95P\x95P\x95P\x95P\x95P\x85a\x07\x1FW`\0\x85\x12a\x03\xB4\x86a\x07\xB2V[`\x06\x81\x90U`\0\x80\x80\x80a\x073\x87\x87a\x07\xF1V[`\0T`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x80\x82R` \x82\x01\x86\x90R\x91\x81\x01\x84\x90R``\x81\x01\x8F\x90R\x95\x99P\x93\x97P\x91\x95P\x93P\x90\x91\x81\x86\x16\x91\x87\x16\x903\x90\x7F\xD4\xDB\xFA\xA7\xC0\xFEQ\xEE\xE2J\x1C\xDC\xAF\xD07\x8C\xD3e,\xDB\xA75\xDF\xC4\xB0a\xAAU\xC2\xA4X\xA5\x90`\x80\x01`@Q\x80\x91\x03\x90\xA4PP`\x01\x80UPPPPPPPPPPPV[`\0`\x01`\xFF\x1B\x82\x03a\x07\xD8W`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x12\x15a\x07\xE9WP\x19`\x01\x01\x90V[P\x80[\x91\x90PV[`\x04T`\x05T`\0\x91\x82\x91\x82\x91\x82\x91\x81\x88\x11\x15a\x08\x8AW`\x02T`\x03T`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x97P\x16\x94Pa\x08(\x82\x89a\x11\xA1V[\x93P\x86\x81\x11a\x08yW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7Finvalid swap: inputs x and y\0\0\0\0`D\x82\x01R`d\x01a\x02\xBFV[a\x08\x83\x87\x82a\x11\xA1V[\x92Pa\t\x08V[`\x03T`\x02T`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x97P\x16\x94Pa\x08\xAA\x81\x88a\x11\xA1V[\x93P\x87\x82\x11a\x08\xFBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7Finvalid swap: inputs x and y\0\0\0\0`D\x82\x01R`d\x01a\x02\xBFV[a\t\x05\x88\x83a\x11\xA1V[\x92P[`\x04\x88\x81U`\x05\x88\x90U`@Qcp\xA0\x821`\xE0\x1B\x81R0\x91\x81\x01\x91\x90\x91R`\0\x90`\x01`\x01`\xA0\x1B\x03\x88\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\t\x94W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x11\xEA\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\t\xA8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\xCC\x91\x90a\x11\xBAV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x88\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\nOW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x11\xEA\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\ncW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\x87\x91\x90a\x11\xBAV[`@Qc#\xB8r\xDD`\xE0\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x88\x90R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x89\x16\x90c#\xB8r\xDD\x90`d\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0B\x16W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x11\xEA\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x0B*W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0BN\x91\x90a\x11\x19V[P`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x86\x90R`\x01`\x01`\xA0\x1B\x03\x88\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0B\xD5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x11\xEA\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x0B\xE9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\r\x91\x90a\x11\x19V[Pa\x0C\x18\x86\x83a\x11\xD6V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x8A\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0C\x95W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x11\xEA\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0C\xA9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xCD\x91\x90a\x11\xBAV[\x10\x15a\r&W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: input token transf`D\x82\x01Ra2\xB9`\xF1\x1B`d\x82\x01R`\x84\x01a\x02\xBFV[a\r0\x85\x82a\x11\xA1V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x89\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\r\xADW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x11\xEA\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\r\xC1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\xE5\x91\x90a\x11\xBAV[\x10\x15a\x0E?W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7Finvalid swap: output token trans`D\x82\x01Rb32\xB9`\xE9\x1B`d\x82\x01R`\x84\x01a\x02\xBFV[PPPP\x92\x95\x91\x94P\x92PV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rjrray stride`\xA8\x1B`d\x82\x01R`\x84\x81\xFD[`\0\x80` \x80\x84\x86\x03\x12\x15a\x0F\x0CWa\x0F\x0Ca\x0ELV[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0FoW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x84\x90R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x0F\xD7W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x84\x90R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x81\xFD[\x815\x81\x81\x11\x15a\x10:W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x85\x90R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x0ENL/$\r\x8C\xAD\xCC\xEE\x8D`\xAB\x1B`d\x82\x01R`\x84\x81\xFD[\x87\x84\x82\x85\x01\x01\x11\x15a\x10NWa\x10Na\x0E\x9CV[\x91\x90\x92\x01\x96\x90\x95P\x93PPPPV[`\0` \x82\x84\x03\x12\x15a\x10rWa\x10ra\x0ELV[\x815`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x10\x89W`\0\x80\xFD[\x93\x92PPPV[` \x81R\x81` \x82\x01R\x81\x83`@\x83\x017`\0\x81\x83\x01`@\x90\x81\x01\x91\x90\x91R`\x1F\x90\x92\x01`\x1F\x19\x16\x01\x01\x91\x90PV[\x80Q\x80\x15\x15\x81\x14a\x07\xECW`\0\x80\xFD[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\x10\xEAWa\x10\xEAa\x0ELV[a\x10\xF3\x86a\x10\xBFV[` \x87\x01Q`@\x88\x01Q``\x89\x01Q`\x80\x90\x99\x01Q\x92\x9A\x91\x99P\x97\x96P\x90\x94P\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x11.Wa\x11.a\x0ELV[a\x10\x89\x82a\x10\xBFV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a\x11SWa\x11Sa\x0ELV[a\x11\\\x87a\x10\xBFV[\x95P` \x87\x01Q\x94P`@\x87\x01Q\x93P``\x87\x01Q\x92P`\x80\x87\x01Q\x91P`\xA0\x87\x01Q\x90P\x92\x95P\x92\x95P\x92\x95V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x11\xB4Wa\x11\xB4a\x11\x8BV[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x11\xCFWa\x11\xCFa\x0ELV[PQ\x91\x90PV[\x80\x82\x01\x80\x82\x11\x15a\x11\xB4Wa\x11\xB4a\x11\x8BV\xFETarget contract does not contain`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`@Qa\x1E\xE18\x03\x80a\x1E\xE1\x839\x81\x01`@\x81\x90Ra\0|\x91a\x01\x04V[g\r\xE0\xB6\xB3\xA7d\0\0\x81\x10a\0\xEAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7Fswap fee percentage must be less`D\x82\x01Ri than 100%`\xB0\x1B`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[`\x01U`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x163\x17\x90Ua\x01hV[`\0` \x82\x84\x03\x12\x15a\x01aW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[PQ\x91\x90PV[a\x1Dj\x80a\x01w`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x01cW`\x005`\xE0\x1C\x80c\xA1\x9C\xD3\xD1\x11a\0\xEFW\x80c\xC1\xE0\x04;\x11a\0\xBEW\x80c\xC1\xE0\x04;\x14a\x03*W\x80c\xC5)\x87\xCF\x14a\x032W\x80c\xCF\xC4\xAFU\x14a\x03:W\x80c\xEB\xAD\xEF\x01\x14a\x03BW\x80c\xF2\xF4\xEB&\x14a\x03JWa\x01cV[\x80c\xA1\x9C\xD3\xD1\x14a\x02\xC4W\x80c\xAF\xDF1\xCD\x14a\x02\xD7W\x80c\xC1e&\x12\x14a\x02\xDFW\x80c\xC1nP\xEF\x14a\x02\xE8Wa\x01cV[\x80c^aZk\x11a\x016W\x80c^aZk\x14a\x02TW\x80cf\x8F\x90U\x14a\x02wW\x80c\x7F\x0EL\x8C\x14a\x02\x89W\x80c\x8DR\xA1\xFC\x14a\x02\x9EW\x80c\x99\x87\xFEd\x14a\x02\xB1Wa\x01cV[\x80c.-yi\x14a\x01\xC8W\x80c>\x1E3\x92\x14a\x01\xF7W\x80c@\xB41i\x14a\x02\x0EW\x80cM\xDFG\xD4\x14a\x02\x17W[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x01\xD0a\x03uV[`@\x80Q\x82Q\x81R` \x80\x84\x01Q\x90\x82\x01R\x91\x81\x01Q\x90\x82\x01R``\x01[`@Q\x80\x91\x03\x90\xF3[a\x02\0`\x0BT\x81V[`@Q\x90\x81R` \x01a\x01\xEEV[a\x02\0`\x01T\x81V[a\x02*a\x02%6`\x04a\x18\x9CV[a\x03\xBEV[`@\x80Q\x95\x15\x15\x86R` \x86\x01\x94\x90\x94R\x92\x84\x01\x91\x90\x91R``\x83\x01R`\x80\x82\x01R`\xA0\x01a\x01\xEEV[a\x02\\a\x04,V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x01\xEEV[`\x02T`\x03T`\x04Ta\x02\\\x92\x91\x90\x83V[a\x02\x9Ca\x02\x976`\x04a\x19\xC1V[a\x04TV[\0[a\x02\x9Ca\x02\xAC6`\x04a\x19\xC1V[a\x05NV[a\x02\x9Ca\x02\xBF6`\x04a\x19\xC1V[a\x06\x12V[a\x02\0a\x02\xD26`\x04a\x1AVV[a\x06\xD6V[a\x02\0a\x07\rV[a\x02\0`\x06T\x81V[a\x02\xFBa\x02\xF66`\x04a\x1AVV[a\x07}V[`@\x80Q\x96\x15\x15\x87R` \x87\x01\x95\x90\x95R\x93\x85\x01\x92\x90\x92R``\x84\x01R`\x80\x83\x01R`\xA0\x82\x01R`\xC0\x01a\x01\xEEV[a\x01\xD0a\x08\xE9V[a\x02\0a\t4V[a\x02\0a\t\x9FV[a\x02\\a\n\nV[`\0Ta\x03]\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\xEEV[a\x03\x99`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[P`@\x80Q``\x81\x01\x82R`\x02T\x81R`\x03T` \x82\x01R`\x04T\x91\x81\x01\x91\x90\x91R\x90V[`\0\x80\x80\x80\x80a\x03\xD0\x86\x88\x01\x88a\x1BDV[\x80Q`\x02U` \x81\x01Q`\x03U`@\x01Q`\x04U\x91\x94P\x92P\x90Pa\x03\xF3a\n\xE4V[a\x04\x06\x83\x83\x83a\x04\x01a\x08\xE9V[a\x0B3V[\x93P\x83a\x04\x13`\na\x1C\x14V[\x12\x80\x15a\x04 WP`\n\x84\x12[\x94P\x92\x95P\x92\x95\x90\x93PV[`\0\x80`\0a\x049a\t4V[a\x04Aa\x07\rV[a\x04Ia\t\x9FV[\x92P\x92P\x92P\x90\x91\x92V[B\x81\x11a\x04|W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04s\x90a\x1C0V[`@Q\x80\x91\x03\x90\xFD[a\x04\x84a\x0CEV[`\0\x82`\x0FT\x11a\x04\xA1W`\x0FTa\x04\x9C\x90\x84a\x1C[V[a\x04\xAFV[\x82`\x0FTa\x04\xAF\x91\x90a\x1C[V[\x90Pa\x04\xBBB\x83a\x1C[V[a\x04\xC5\x90\x82a\x1CnV[`\x12U`\x10\x83\x90U`\x13\x82\x90U`\x0FT\x7F\x1EK Gg&%\xF4\xDF!\xD4|\xA7\xA0\x0C \x12\xCC\xF7x\xF2w\xBF2(\xDD\xF4Y=o\x84\xAA\x90\x84\x90\x84\x81\x83\x11a\x05\x16W`\x10T`\x0FTa\x05\x11\x91\x90a\x1C[V[a\x05&V[`\x0FT`\x10Ta\x05&\x91\x90a\x1C[V[`@\x80Q\x94\x85R` \x85\x01\x93\x90\x93R\x91\x83\x01R``\x82\x01R`\x80\x01`@Q\x80\x91\x03\x90\xA1PPPV[B\x81\x11a\x05mW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04s\x90a\x1C0V[a\x05ua\x0CVV[`\0\x82`\nT\x11a\x05\x92W`\nTa\x05\x8D\x90\x84a\x1C[V[a\x05\xA0V[\x82`\nTa\x05\xA0\x91\x90a\x1C[V[\x90Pa\x05\xACB\x83a\x1C[V[a\x05\xB6\x90\x82a\x1CnV[`\rU`\x0B\x83\x90U`\x0E\x82\x90U`\nT\x7F$\xBBEgU#\x9Cf@Y\xB5\x19\xE4c\xB4\xE5\xEA\xEF\x05\x08~\x87\xB0\xDF\xED\xF6\x06\x11iR\xAC\xEE\x90\x84\x90\x84\x81\x83\x11a\x06\x02W`\x0BT`\nTa\x05\x11\x91\x90a\x1C[V[`\nT`\x0BTa\x05&\x91\x90a\x1C[V[B\x81\x11a\x061W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04s\x90a\x1C0V[a\x069a\x0CgV[`\0\x82`\x05T\x11a\x06VW`\x05Ta\x06Q\x90\x84a\x1C[V[a\x06dV[\x82`\x05Ta\x06d\x91\x90a\x1C[V[\x90Pa\x06pB\x83a\x1C[V[a\x06z\x90\x82a\x1CnV[`\x08U`\x06\x83\x90U`\t\x82\x90U`\x05T\x7F\xB9hO\xE86\x87_\xAF\x05m\xB2]\xE9\x85S\x859`\xB3&\x08\x8A\x10!\xAF\xA7[\xA1`\x12d\xF5\x90\x84\x90\x84\x81\x83\x11a\x06\xC6W`\x06T`\x05Ta\x05\x11\x91\x90a\x1C[V[`\x05T`\x06Ta\x05&\x91\x90a\x1C[V[`\0\x80`\0\x80\x84\x80` \x01\x90Q\x81\x01\x90a\x06\xF0\x91\x90a\x1C\x90V[\x92P\x92P\x92Pa\x07\x04\x83\x83\x83a\x04\x01a\x08\xE9V[\x95\x94PPPPPV[`\0`\tTB\x10a\x07\x1FWP`\x06T\x90V[`\x06T`\x05T\x11a\x07VW`\x08T`\x07Ta\x07:\x90Ba\x1C[V[a\x07D\x91\x90a\x1C\xC1V[`\x05Ta\x07Q\x91\x90a\x1C\xD8V[\x90P\x90V[`\x08T`\x07Ta\x07f\x90Ba\x1C[V[a\x07p\x91\x90a\x1C\xC1V[`\x05Ta\x07Q\x91\x90a\x1C[V[`\0\x80`\0\x80`\0\x80`\0\x80`\0a\x07\x93a\n\nV[\x92P\x92P\x92P\x89\x80` \x01\x90Q\x81\x01\x90a\x07\xAD\x91\x90a\x1C\x90V[\x91\x97P\x95P\x93P`\0\x80\x80\x85\x89\x11\x15a\x08\x08Wa\x07\xCA\x86\x8Aa\x1C[V[\x91Pa\x07\xE1`\x01T\x83a\x0Cx\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa\x07\xF7\x86a\x07\xF1\x83\x87a\x0CxV[\x90a\x0C\x96V[a\x08\x01\x90\x84a\x1C\xD8V[\x92Pa\x08\xA2V[\x84\x88\x11\x15a\x08AWa\x08\x1A\x85\x89a\x1C[V[\x91Pa\x081`\x01T\x83a\x0Cx\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa\x07\xF7\x85a\x07\xF1\x83\x87a\x0CxV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`0`$\x82\x01R\x7Finvalid swap: inputs x and y hav`D\x82\x01Roe the same sign!`\x80\x1B`d\x82\x01R`\x84\x01a\x04sV[a\x08\xAC\x84\x88a\x1C\xEBV[\x99P`\0\x8Ba\x08\xBB`\na\x1C\x14V[\x12\x80\x15a\x08\xC8WP`\n\x8C\x12[\x90P\x80\x80\x15a\x08\xD7WP\x83\x8B\x12\x15[\x9CPPPPPPPP\x91\x93\x95P\x91\x93\x95V[a\t\r`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[a\t\x15a\t4V[a\t\x1Da\x07\rV[a\t%a\t\x9FV[`@\x84\x01R` \x83\x01R\x81R\x90V[`\0`\x0ETB\x10a\tFWP`\x0BT\x90V[`\x0BT`\nT\x11a\txW`\rT`\x0CTa\ta\x90Ba\x1C[V[a\tk\x91\x90a\x1C\xC1V[`\nTa\x07Q\x91\x90a\x1C\xD8V[`\rT`\x0CTa\t\x88\x90Ba\x1C[V[a\t\x92\x91\x90a\x1C\xC1V[`\nTa\x07Q\x91\x90a\x1C[V[`\0`\x13TB\x10a\t\xB1WP`\x10T\x90V[`\x10T`\x0FT\x11a\t\xE3W`\x12T`\x11Ta\t\xCC\x90Ba\x1C[V[a\t\xD6\x91\x90a\x1C\xC1V[`\x0FTa\x07Q\x91\x90a\x1C\xD8V[`\x12T`\x11Ta\t\xF3\x90Ba\x1C[V[a\t\xFD\x91\x90a\x1C\xC1V[`\x0FTa\x07Q\x91\x90a\x1C[V[`\0\x80`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEB\xAD\xEF\x01`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01```@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\n\xACW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\n\xC0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04I\x91\x90a\x1C\x90V[`\0a\n\xEEa\x03uV[` \x81\x01Q`\x06\x81\x90U`\x05UB`\t\x81\x90U`\x07\x81\x90U\x81Q`\x0B\x81\x90U`\nU`\x0E\x81\x90U`\x0C\x81\x90U`@\x90\x91\x01Q`\x10\x81\x90U`\x0FU`\x13\x81\x90U`\x11UPV[`\0\x82\x85\x10a\x0B\x84W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FtradingFunction: invalid x\0\0\0\0\0\0`D\x82\x01R`d\x01a\x04sV[`\0\x80g\r\xE0\xB6\xB3\xA7d\0\0a\x0B\x9A\x88\x87a\x0C\xABV[\x10a\x0B\xAEW`\x01`\x01`\xFF\x1B\x03\x91Pa\x0B\xC3V[a\x0B\xC0a\x0B\xBB\x88\x87a\x0C\xABV[a\x0C\xC0V[\x91P[g\r\xE0\xB6\xB3\xA7d\0\0a\x0B\xE3\x87a\x0B\xDE\x87`\0\x01Q\x89a\r]V[a\x0C\xABV[\x10a\x0B\xF6WP`\x01`\x01`\xFF\x1B\x03a\x0C\x0EV[a\x0C\x0Ba\x0B\xBB\x87a\x0B\xDE\x87`\0\x01Q\x89a\r]V[\x90P[`\0a\x0C\"\x85` \x01Q\x86`@\x01Qa\rrV[\x90P\x80a\x0C/\x83\x85a\x1D\x12V[a\x0C9\x91\x90a\x1D\x12V[\x98\x97PPPPPPPPV[a\x0CMa\t\x9FV[`\x0FUB`\x11UV[a\x0C^a\t4V[`\nUB`\x0CUV[a\x0Coa\x07\rV[`\x05UB`\x07UV[`\0a\x0C\x8D\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\r\xA0V[\x90P[\x92\x91PPV[`\0a\x0C\x8D\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\r\xA0V[`\0a\x0C\x8D\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\r\xCEV[`\0g\x06\xF0[Y\xD3\xB2\0\0\x82\x03a\x0C\xD9WP`\0\x91\x90PV[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x12a\r\x01W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\r\"W`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\r/\x83`\x02a\x1D:V[\x90P`\0a\r<\x82a\r\xEDV[\x90P`\0a\rRg\x13\xA0K\xBD\xFD\xC9\xBE\x88\x83a\x10kV[\x90Pa\x07\x04\x81a\x1C\x14V[`\0a\x0C\x8D\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\r\xCEV[`\0\x80a\r~\x83a\x10\x80V[a\r\x8C\x90c;\x9A\xCA\0a\x1C\xC1V[\x90Pa\r\x98\x84\x82a\r]V[\x94\x93PPPPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\r\xB8W`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\r\xE6W`\0\x80\xFD[\x04\x92\x91PPV[`\0\x80\x82\x12\x80a\x0E\x04WPg\x1B\xC1mgN\xC8\0\0\x82\x13[\x15a\x0E\"W`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\x0ECW`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81g\x1B\xC1mgN\xC8\0\0\x03a\x0EkW`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x15a\x0EvW\x91\x90PV[`\0g\r\xE0\xB6\xB3\xA7d\0\0\x83\x12a\x0E\x9EWa\x0E\x99\x83g\x1B\xC1mgN\xC8\0\0a\x1C\xEBV[a\x0E\xA0V[\x82[\x90P`\0a\x0E\xB6\x82g\x1B\xC1mgN\xC8\0\0a\x11$V[\x90P\x80`\0\x03a\x0E\xD9W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x0E\xE4\x82a\x119V[\x90P`\0c;\x9A\xCA\0a\x0F\x0Fa\x0F\na\x0F\x04g\x1B\xC1mgN\xC8\0\0a\x1C\x14V[\x85a\x10kV[a\x10\x80V[a\x0F\x19\x91\x90a\x1D:V[\x90P`\0\x80a\x0F0\x83g\x03\xC1f\\z\xAB \0a\x10kV[a\x0FB\x90g \x05\xFEO&\x8E\xA0\0a\x1D\x12V[\x90P`\0a\x0Fr\x84a\x0F[\x86f\x9F2u$b\xA0\0a\x10kV[a\x0Fm\x90g\r\xC5R\x7Fd, \0a\x1D\x12V[a\x10kV[a\x0F\x84\x90g\r\xE0\xB6\xB3\xA7d\0\0a\x1D\x12V[\x90Pa\x0F\xA8g\t\xD0(\xCCo _\xFF\x19\x85a\x0F\x9E\x85\x85a\x11$V[a\x0Fm\x91\x90a\x1C\xEBV[\x92PPP`\0[`\x02\x81\x10\x15a\x10CW`\0\x86a\x0F\xC4\x84a\x13\x14V[a\x0F\xCE\x91\x90a\x1C\xEBV[\x90P`\0a\x0F\xDC\x84\x85a\x10kV[a\x0F\xE5\x90a\x1C\x14V[\x90P`\0a\x0F\xF2\x82a\x14\xF8V[\x90P`\0a\x10\0\x86\x85a\x10kV[a\x10\x12g\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x84a\x10kV[a\x10\x1C\x91\x90a\x1C\xEBV[\x90Pa\x10(\x84\x82a\x11$V[a\x102\x90\x87a\x1D\x12V[\x95P\x84`\x01\x01\x94PPPPPa\x0F\xAFV[g\r\xE0\xB6\xB3\xA7d\0\0\x88\x12a\x10`Wa\x10[\x82a\x1C\x14V[a\x0C9V[P\x96\x95PPPPPPV[`\0a\x0C\x8D\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x16\xA1V[`\xB5\x81`\x01`\x88\x1B\x81\x10a\x10\x99W`@\x91\x90\x91\x1B\x90`\x80\x1C[i\x01\0\0\0\0\0\0\0\0\0\x81\x10a\x10\xB5W` \x91\x90\x91\x1B\x90`@\x1C[e\x01\0\0\0\0\0\x81\x10a\x10\xCDW`\x10\x91\x90\x91\x1B\x90` \x1C[c\x01\0\0\0\x81\x10a\x10\xE3W`\x08\x91\x90\x91\x1B\x90`\x10\x1C[b\x01\0\0\x01\x02`\x12\x1C\x80\x82\x04\x01`\x01\x90\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x90\x81\x90\x04\x81\x11\x90\x03\x90V[`\0a\x0C\x8D\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x16\xA1V[`\0\x80\x82\x13a\x11vW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x04sV[`\0``a\x11\x83\x84a\x16\xC0V[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[`\0\x81`\0\x03a\x13-WPg\r\xE0\xB6\xB3\xA7d\0\0\x91\x90PV[gV\x98\xEE\xF0fp\0\0\x82\x12a\x13DWP`\0\x91\x90PV[a\x13UgV\x98\xEE\xF0fp\0\0a\x1C\x14V[\x82\x13a\x13jWPg\x1B\xC1mgN\xC8\0\0\x91\x90PV[`\0a\x13u\x83a\x17hV[\x90P`\0a\x13\xAEg\r\xE0\xB6\xB3\xA7d\0\0a\x13\x97\x84g\x1B\xC1mgN\xC8\0\0a\x0C\xABV[a\x13\xA9\x90g\r\xE0\xB6\xB3\xA7d\0\0a\x1D\x12V[a\x11$V[\x90P`\0\x80\x82a\x14\n\x81a\x13\xF7\x81a\x13\xE5\x81a\x13\xD2\x81g\x02_\x0F\xE1\x05\xA3\x14\0a\x10kV[a\x0Fm\x90g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19a\x1D\x12V[a\x0Fm\x90g\x14\xA8EL\x19\xE1\xAC\0a\x1D\x12V[a\x0Fm\x90g\x0F\xC1\x0E\x01W\x82w\xFF\x19a\x1D\x12V[a\x14\x1C\x90g\x03\xDE\xBD\x08;\x8C|\0a\x1D\x12V[\x91P\x83\x90Pa\x14\x84\x81a\x14r\x81a\x14`\x81a\x14N\x81a\x14;\x81\x8Ba\x10kV[a\x0Fm\x90g\x02\x95\xD4\0\xEA2W\xFF\x19a\x1D\x12V[a\x0Fm\x90g\x01W\xD8\xB2\xEC\xC7\x08\0a\x1D\x12V[a\x0Fm\x90g\x051\n\xA7\xD5!0\0a\x1D\x12V[a\x0Fm\x90g\r\xE0\xCC=\x15a\0\0a\x1D\x12V[\x91P\x81g\x11\x90\0\xAB\x10\x0F\xFC\0a\x14\x9A\x87\x88a\x10kV[a\x14\xA6\x90`\0\x19a\x1D:V[a\x14\xB0\x91\x90a\x1C\xEBV[a\x14\xBA\x91\x90a\x1D\x12V[\x92PP`\0a\x14\xC8\x83a\x14\xF8V[\x90P`\0a\x14\xD6\x85\x83a\x10kV[\x90P`\0\x88\x12a\x14\xE6W\x80a\x0C9V[a\x0C9\x81g\x1B\xC1mgN\xC8\0\0a\x1C\xEBV[`\0h\x02H\xCE6\xA7\x0C\xB2k>\x19\x82\x13a\x15\x13WP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a\x15ZW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x01a\x04sV[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05`\x01`_\x1B\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1Dm\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1Dn\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x05\x85\x14\x17\x16a\x16\xB9W`\0\x80\xFD[\x05\x92\x91PPV[`\0\x80\x82\x11a\x16\xFDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x04sV[P`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x82\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[`\0`\x01`\xFF\x1B\x82\x03a\x17\x8EW`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x12\x15a\x17\x9FWP\x19`\x01\x01\x90V[P\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x81\xFD[`\0\x80` \x83\x85\x03\x12\x15a\x18\xB2Wa\x18\xB2a\x17\xA3V[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x18\xCDWa\x18\xCDa\x17\xF3V[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x18\xE4Wa\x18\xE4a\x18CV[\x815\x81\x81\x11\x15a\x19GW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x0ENL/$\r\x8C\xAD\xCC\xEE\x8D`\xAB\x1B`d\x82\x01R`\x84\x81\xFD[\x86` \x82\x85\x01\x01\x11\x15a\x19\xAFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rjrray stride`\xA8\x1B`d\x82\x01R\x91P`\x84\x82\xFD[` \x92\x90\x92\x01\x96\x91\x95P\x90\x93PPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x19\xD7Wa\x19\xD7a\x17\xA3V[PP\x805\x92` \x90\x91\x015\x91PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x1A\x1FWa\x1A\x1Fa\x19\xE6V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x1ANWa\x1ANa\x19\xE6V[`@R\x91\x90PV[`\0` \x80\x83\x85\x03\x12\x15a\x1AlWa\x1Ala\x17\xA3V[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x1A\x87Wa\x1A\x87a\x17\xF3V[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x1A\x9EWa\x1A\x9Ea\x18CV[\x815\x81\x81\x11\x15a\x1A\xB0Wa\x1A\xB0a\x19\xE6V[a\x1A\xC2`\x1F\x82\x01`\x1F\x19\x16\x85\x01a\x1A%V[\x91P\x80\x82R\x86\x84\x82\x85\x01\x01\x11\x15a\x1B(W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x85\x90R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R`\x84\x81\xFD[\x80\x84\x84\x01\x85\x84\x017`\0\x90\x82\x01\x90\x93\x01\x92\x90\x92RP\x93\x92PPPV[`\0\x80`\0\x80\x84\x86\x03`\xC0\x81\x12\x15a\x1B^Wa\x1B^a\x17\xA3V[\x855\x94P` \x86\x015\x93P`@\x86\x015\x92P```_\x19\x82\x01\x12\x15a\x1B\xCEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FABI decoding: struct data too sh`D\x82\x01Rb\x1B\xDC\x9D`\xEA\x1B`d\x82\x01R`\x84\x81\xFD[Pa\x1B\xD7a\x19\xFCV[``\x86\x015\x81R`\x80\x86\x015` \x82\x01R`\xA0\x90\x95\x015`@\x86\x01RP\x91\x94\x90\x93P\x90\x91\x90V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01`\xFF\x1B\x82\x01a\x1C)Wa\x1C)a\x1B\xFEV[P`\0\x03\x90V[` \x80\x82R`\x11\x90\x82\x01Rp\x15\\\x19\x18]\x19H\x19[\x99\x08\x1C\x18\\\xDC\xD9Y`z\x1B`@\x82\x01R``\x01\x90V[\x81\x81\x03\x81\x81\x11\x15a\x0C\x90Wa\x0C\x90a\x1B\xFEV[`\0\x82a\x1C\x8BWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[`\0\x80`\0``\x84\x86\x03\x12\x15a\x1C\xA8Wa\x1C\xA8a\x17\xA3V[\x83Q\x92P` \x84\x01Q\x91P`@\x84\x01Q\x90P\x92P\x92P\x92V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x0C\x90Wa\x0C\x90a\x1B\xFEV[\x80\x82\x01\x80\x82\x11\x15a\x0C\x90Wa\x0C\x90a\x1B\xFEV[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a\x1D\x0BWa\x1D\x0Ba\x1B\xFEV[P\x92\x91PPV[\x80\x82\x01\x82\x81\x12`\0\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a\x1D2Wa\x1D2a\x1B\xFEV[PP\x92\x91PPV[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15a\x1DVWa\x1DVa\x1B\xFEV[\x81\x81\x05\x83\x14\x82\x15\x17a\x0C\x90Wa\x0C\x90a\x1B\xFEV";
    /// The bytecode of the contract.
    pub static DFMM_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x01\x01W`\x005`\xE0\x1C\x80cg\xE8(\xBF\x11a\0\xBEW\x80cg\xE8(\xBF\x14a\x02\x1DW\x80cp\xA0\x821\x14a\x020W\x80c\xB7\xD1\x9F\xC4\x14a\x02PW\x80c\xCF0\x90\x12\x14a\x02cW\x80c\xEB\xAD\xEF\x01\x14a\x02lW\x80c\xF0e\x95\x7F\x14a\x02zWa\x01\x01V[\x80c\x15w\x0F\x92\x14a\x01fW\x80c\x16\xDC\x16[\x14a\x01\x82W\x80cC\xC8\x85\xBA\x14a\x01\xADW\x80cM\xDFG\xD4\x14a\x01\xD1W\x80c^\"}X\x14a\x01\xFFW\x80cb}\xD5j\x14a\x02\x08W[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x01o`\x06T\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[`\x02Ta\x01\x95\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01yV[`\0Ta\x01\xC1\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x01yV[a\x01\xE4a\x01\xDF6`\x04a\x0E\xF5V[a\x02\x83V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x01yV[a\x01o`\x05T\x81V[a\x02\x1Ba\x02\x166`\x04a\x0E\xF5V[a\x05\xADV[\0[`\0Ta\x01\x95\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01oa\x02>6`\x04a\x10]V[`\x07` R`\0\x90\x81R`@\x90 T\x81V[`\x03Ta\x01\x95\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01o`\x01T\x81V[`\x04T`\x05T`\x06Ta\x01\xE4V[a\x01o`\x04T\x81V[`\0\x80`\0`\x01T`\x01\x14a\x02\xC8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x06`$\x82\x01Re\x1B\x1B\xD8\xDA\xD9Y`\xD2\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0`\x01\x81\x90U\x80T`@Qc\x13w\xD1\xF5`\xE2\x1B\x81R\x82\x91\x82\x91\x82\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cM\xDFG\xD4\x90a\x03\x08\x90\x8D\x90\x8D\x90`\x04\x01a\x10\x90V[`\xA0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x03`W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x11\xEA\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x03tW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\x98\x91\x90a\x10\xCFV[\x94P\x94P\x94P\x94P\x94P\x84a\x03\xD6W`\0\x84\x12a\x03\xB4\x85a\x07\xB2V[`@Qcw`m)`\xE1\x1B\x81R\x91\x15\x15`\x04\x83\x01R`$\x82\x01R`D\x01a\x02\xBFV[`\0\x80T`\xFF`\xA0\x1B\x19\x16`\x01`\xA0\x1B\x17\x81U`\x04\x84\x81U`\x05\x84\x90U`\x06\x83\x90U3\x80\x83R`\x07` R`@\x92\x83\x90 \x84\x90U`\x02T\x92Qc#\xB8r\xDD`\xE0\x1B\x81R\x91\x82\x01R0`$\x82\x01R`D\x81\x01\x85\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90`d\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x04\x97W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x11\xEA\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x04\xABW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xCF\x91\x90a\x11\x19V[P`\x03T`@Qc#\xB8r\xDD`\xE0\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90`d\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x05`W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x11\xEA\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x05tW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\x98\x91\x90a\x11\x19V[P`\x01\x80U\x91\x99\x90\x98P\x90\x96P\x94PPPPPV[`\x01T`\x01\x14a\x05\xE8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x06`$\x82\x01Re\x1B\x1B\xD8\xDA\xD9Y`\xD2\x1B`D\x82\x01R`d\x01a\x02\xBFV[`\0`\x01\x81\x90UT`\x01`\xA0\x1B\x90\x04`\xFF\x16a\x068W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rn\x1B\x9B\xDD\x08\x1A[\x9A]\x1AX[\x1A^\x99Y`\x8A\x1B`D\x82\x01R`d\x01a\x02\xBFV[`\0\x80T`@Qc\xC1nP\xEF`\xE0\x1B\x81R\x82\x91\x82\x91\x82\x91\x82\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xC1nP\xEF\x90a\x06s\x90\x8B\x90\x8B\x90`\x04\x01a\x10\x90V[`\xC0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x06\xC9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x11\xEA\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x06\xDDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x01\x91\x90a\x117V[\x95P\x95P\x95P\x95P\x95P\x95P\x85a\x07\x1FW`\0\x85\x12a\x03\xB4\x86a\x07\xB2V[`\x06\x81\x90U`\0\x80\x80\x80a\x073\x87\x87a\x07\xF1V[`\0T`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x80\x82R` \x82\x01\x86\x90R\x91\x81\x01\x84\x90R``\x81\x01\x8F\x90R\x95\x99P\x93\x97P\x91\x95P\x93P\x90\x91\x81\x86\x16\x91\x87\x16\x903\x90\x7F\xD4\xDB\xFA\xA7\xC0\xFEQ\xEE\xE2J\x1C\xDC\xAF\xD07\x8C\xD3e,\xDB\xA75\xDF\xC4\xB0a\xAAU\xC2\xA4X\xA5\x90`\x80\x01`@Q\x80\x91\x03\x90\xA4PP`\x01\x80UPPPPPPPPPPPV[`\0`\x01`\xFF\x1B\x82\x03a\x07\xD8W`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x12\x15a\x07\xE9WP\x19`\x01\x01\x90V[P\x80[\x91\x90PV[`\x04T`\x05T`\0\x91\x82\x91\x82\x91\x82\x91\x81\x88\x11\x15a\x08\x8AW`\x02T`\x03T`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x97P\x16\x94Pa\x08(\x82\x89a\x11\xA1V[\x93P\x86\x81\x11a\x08yW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7Finvalid swap: inputs x and y\0\0\0\0`D\x82\x01R`d\x01a\x02\xBFV[a\x08\x83\x87\x82a\x11\xA1V[\x92Pa\t\x08V[`\x03T`\x02T`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x97P\x16\x94Pa\x08\xAA\x81\x88a\x11\xA1V[\x93P\x87\x82\x11a\x08\xFBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7Finvalid swap: inputs x and y\0\0\0\0`D\x82\x01R`d\x01a\x02\xBFV[a\t\x05\x88\x83a\x11\xA1V[\x92P[`\x04\x88\x81U`\x05\x88\x90U`@Qcp\xA0\x821`\xE0\x1B\x81R0\x91\x81\x01\x91\x90\x91R`\0\x90`\x01`\x01`\xA0\x1B\x03\x88\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\t\x94W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x11\xEA\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\t\xA8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\xCC\x91\x90a\x11\xBAV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x88\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\nOW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x11\xEA\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\ncW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\x87\x91\x90a\x11\xBAV[`@Qc#\xB8r\xDD`\xE0\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x88\x90R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x89\x16\x90c#\xB8r\xDD\x90`d\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0B\x16W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x11\xEA\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x0B*W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0BN\x91\x90a\x11\x19V[P`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x86\x90R`\x01`\x01`\xA0\x1B\x03\x88\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0B\xD5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x11\xEA\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x0B\xE9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\r\x91\x90a\x11\x19V[Pa\x0C\x18\x86\x83a\x11\xD6V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x8A\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0C\x95W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x11\xEA\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0C\xA9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xCD\x91\x90a\x11\xBAV[\x10\x15a\r&W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: input token transf`D\x82\x01Ra2\xB9`\xF1\x1B`d\x82\x01R`\x84\x01a\x02\xBFV[a\r0\x85\x82a\x11\xA1V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x89\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\r\xADW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x11\xEA\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\r\xC1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\xE5\x91\x90a\x11\xBAV[\x10\x15a\x0E?W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7Finvalid swap: output token trans`D\x82\x01Rb32\xB9`\xE9\x1B`d\x82\x01R`\x84\x01a\x02\xBFV[PPPP\x92\x95\x91\x94P\x92PV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rjrray stride`\xA8\x1B`d\x82\x01R`\x84\x81\xFD[`\0\x80` \x80\x84\x86\x03\x12\x15a\x0F\x0CWa\x0F\x0Ca\x0ELV[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0FoW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x84\x90R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x0F\xD7W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x84\x90R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x81\xFD[\x815\x81\x81\x11\x15a\x10:W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x85\x90R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x0ENL/$\r\x8C\xAD\xCC\xEE\x8D`\xAB\x1B`d\x82\x01R`\x84\x81\xFD[\x87\x84\x82\x85\x01\x01\x11\x15a\x10NWa\x10Na\x0E\x9CV[\x91\x90\x92\x01\x96\x90\x95P\x93PPPPV[`\0` \x82\x84\x03\x12\x15a\x10rWa\x10ra\x0ELV[\x815`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x10\x89W`\0\x80\xFD[\x93\x92PPPV[` \x81R\x81` \x82\x01R\x81\x83`@\x83\x017`\0\x81\x83\x01`@\x90\x81\x01\x91\x90\x91R`\x1F\x90\x92\x01`\x1F\x19\x16\x01\x01\x91\x90PV[\x80Q\x80\x15\x15\x81\x14a\x07\xECW`\0\x80\xFD[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\x10\xEAWa\x10\xEAa\x0ELV[a\x10\xF3\x86a\x10\xBFV[` \x87\x01Q`@\x88\x01Q``\x89\x01Q`\x80\x90\x99\x01Q\x92\x9A\x91\x99P\x97\x96P\x90\x94P\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x11.Wa\x11.a\x0ELV[a\x10\x89\x82a\x10\xBFV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a\x11SWa\x11Sa\x0ELV[a\x11\\\x87a\x10\xBFV[\x95P` \x87\x01Q\x94P`@\x87\x01Q\x93P``\x87\x01Q\x92P`\x80\x87\x01Q\x91P`\xA0\x87\x01Q\x90P\x92\x95P\x92\x95P\x92\x95V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x11\xB4Wa\x11\xB4a\x11\x8BV[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x11\xCFWa\x11\xCFa\x0ELV[PQ\x91\x90PV[\x80\x82\x01\x80\x82\x11\x15a\x11\xB4Wa\x11\xB4a\x11\x8BV\xFETarget contract does not contain";
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
        ///Gets the contract's `Swap` event
        pub fn swap_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, SwapFilter> {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, SwapFilter> {
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
        GetReservesAndLiquidity(GetReservesAndLiquidityCall),
        Init(InitCall),
        Inited(InitedCall),
        Locked(LockedCall),
        ReserveXWad(ReserveXWadCall),
        ReserveYWad(ReserveYWadCall),
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
            if let Ok(decoded) = <GetReservesAndLiquidityCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetReservesAndLiquidity(decoded));
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
                Self::GetReservesAndLiquidity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Init(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Inited(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Locked(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ReserveXWad(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ReserveYWad(element) => {
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
                Self::GetReservesAndLiquidity(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Init(element) => ::core::fmt::Display::fmt(element, f),
                Self::Inited(element) => ::core::fmt::Display::fmt(element, f),
                Self::Locked(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReserveXWad(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReserveYWad(element) => ::core::fmt::Display::fmt(element, f),
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
    impl ::core::convert::From<GetReservesAndLiquidityCall> for DFMMCalls {
        fn from(value: GetReservesAndLiquidityCall) -> Self {
            Self::GetReservesAndLiquidity(value)
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
