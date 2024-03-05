pub use constant_sum_solver::*;
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
pub mod constant_sum_solver {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("strategy_"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("getInitialPoolData"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getInitialPoolData"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("rx"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("ry"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("params"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct ConstantSum.ConstantSumParams",
                                        ),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("simulateAllocateOrDeallocate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "simulateAllocateOrDeallocate",
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
                                    name: ::std::borrow::ToOwned::to_owned("IsAllocate"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountX"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountY"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
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
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
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
                    ::std::borrow::ToOwned::to_owned("strategy"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("strategy"),
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
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("NotEnoughLiquidity"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("NotEnoughLiquidity"),
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
    pub static CONSTANTSUMSOLVER_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`@Qa\x13\xE28\x03\x80a\x13\xE2\x839\x81\x01`@\x81\x90Ra\0|\x91a\0\xA1V[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90Ua\x01\x1CV[`\0` \x82\x84\x03\x12\x15a\0\xFEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01\x15W`\0\x80\xFD[\x93\x92PPPV[a\x12\xB7\x80a\x01+`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\0\x99W`\x005`\xE0\x1C\x80c9(\xFF\x97\x14a\0\xFEW\x80c\x89\xEA\x85Y\x14a\x01)W\x80c\x8A\x1A \xDE\x14a\x01IW\x80c\xA8\xC6.v\x14a\x01jW[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x01\x11a\x01\x0C6`\x04a\x0C\xD0V[a\x01\x95V[`@Qa\x01 \x93\x92\x91\x90a\r[V[`@Q\x80\x91\x03\x90\xF3[a\x01<a\x0176`\x04a\x0E\nV[a\x06\xCDV[`@Qa\x01 \x91\x90a\x0E\xC5V[a\x01\\a\x01W6`\x04a\x0E\xD8V[a\x07CV[`@Qa\x01 \x92\x91\x90a\x0F\x18V[`\0Ta\x01}\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01 V[`\0\x80``a\x01\xBE`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[a\x01\xE2`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`\0\x80T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xAF\xBA\x13\xC4`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x02lW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x12b\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x02\x80W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\xA4\x91\x90a\x0F;V[`\x01`\x01`\xA0\x1B\x03\x16c\xCE\x15;\xF4\x89`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x02\xD1\x91\x81R` \x01\x90V[```@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x03'W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x12b\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x03;W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03_\x91\x90a\x0FbV[`@\x85\x81\x01\x91\x90\x91R` \x85\x01\x91\x90\x91R\x90\x83R`\0\x80T\x91Qc\xDC\x17\x83U`\xE0\x1B\x81R`\x04\x81\x01\x8B\x90R\x90\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xDC\x17\x83U\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x03\xF5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x12b\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x04\tW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x041\x91\x90\x81\x01\x90a\x0F\x93V[\x80` \x01\x90Q\x81\x01\x90a\x04D\x91\x90a\x11\x17V[\x90P`\0\x88\x15a\x04\xFEW`\0a\x04g\x83` \x01Q\x8Aa\x0B\xC5\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa\x04\x97\x83` \x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x04\x84\x91\x90a\x11rV[\x84Qa\x04\x91\x90\x8C\x90a\x0B\xE3V[\x90a\x0B\xE3V[\x85Q\x90\x92Pa\x04\xA7\x90\x8A\x90a\x11\x85V[\x84R`@\x85\x01Qa\x04\xB9\x90\x82\x90a\x11\x85V[`@\x85\x01R` \x85\x01Q\x82\x11\x15a\x04\xE3W`@QcC#\xA5U`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x85` \x01Qa\x04\xF3\x91\x90a\x11rV[` \x85\x01RPa\x05\xA8V[\x81Q` \x83\x01Q`\0\x91a\x05\x1D\x91a\x05\x17\x90\x8C\x90a\x0B\xC5V[\x90a\x0B\xF8V[\x90Pa\x05I\x83`\0\x01Qa\x05C\x8B\x86` \x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x04\x91\x91\x90a\x11rV[\x90a\x0C\rV[\x91P\x88\x85` \x01Qa\x05[\x91\x90a\x11\x85V[` \x85\x01R`@\x85\x01Qa\x05p\x90\x82\x90a\x11\x85V[`@\x85\x01R\x84Q\x82\x11\x15a\x05\x97W`@QcC#\xA5U`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x84Qa\x05\xA4\x90\x83\x90a\x11rV[\x84RP[`@\x80Q\x84Q` \x80\x83\x01\x91\x90\x91R\x85\x01Q\x81\x83\x01R\x90\x84\x01Q``\x82\x01R`\0\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16ch\xBD>80\x8E\x85`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x06$\x93\x92\x91\x90a\x11\x98V[`\xC0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x06zW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x12b\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x06\x8EW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xB2\x91\x90a\x11\xBFV[P\x93\x9CP\x95\x9AP\x93\x98PPPPPPPPP\x93P\x93P\x93\x90PV[\x80Q``\x90`\0\x90a\x06\xE0\x90\x85\x90a\x0B\xF8V[a\x06\xEA\x90\x86a\x11\x85V[`@\x80Q` \x80\x82\x01\x89\x90R\x81\x83\x01\x88\x90R``\x82\x01\x84\x90R\x86Q`\x80\x83\x01R\x86\x01Q`\xA0\x82\x01R\x90\x85\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\xC0\x82\x01R\x90\x91P`\xE0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91PP\x93\x92PPPV[`\0``a\x07k`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[a\x07\x8F`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`\0\x80T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xAF\xBA\x13\xC4`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x08\x19W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x12b\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x08-W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08Q\x91\x90a\x0F;V[`\x01`\x01`\xA0\x1B\x03\x16c\xCE\x15;\xF4\x89`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x08~\x91\x81R` \x01\x90V[```@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x08\xD4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x12b\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x08\xE8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\x0C\x91\x90a\x0FbV[`@\x85\x81\x01\x91\x90\x91R` \x85\x01\x91\x90\x91R\x90\x83R`\0\x80T\x91Qc\xDC\x17\x83U`\xE0\x1B\x81R`\x04\x81\x01\x8B\x90R\x90\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xDC\x17\x83U\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\t\xA2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x12b\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\t\xB6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\t\xDE\x91\x90\x81\x01\x90a\x0F\x93V[\x80` \x01\x90Q\x81\x01\x90a\t\xF1\x91\x90a\x11\x17V[\x90P\x87\x15a\nAW\x82Qa\n\x06\x90\x88\x90a\x11\x85V[\x82R` \x83\x01Qa\n\x18\x90\x87\x90a\x11\x85V[` \x83\x01\x81\x90R\x81Qa\n+\x91\x90a\x0B\xF8V[\x82Qa\n7\x91\x90a\x11\x85V[`@\x83\x01Ra\n\xB5V[\x82Q\x87\x11\x80a\nSWP\x85\x83` \x01Q\x10[\x15a\nqW`@QcC#\xA5U`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82Qa\n~\x90\x88\x90a\x11rV[\x82R` \x83\x01Qa\n\x90\x90\x87\x90a\x11rV[` \x83\x01\x81\x90R\x81Qa\n\xA3\x91\x90a\x0B\xF8V[\x82Qa\n\xAF\x91\x90a\x11\x85V[`@\x83\x01R[`@\x80Q\x83Q` \x80\x83\x01\x91\x90\x91R\x84\x01Q\x81\x83\x01R\x90\x83\x01Q``\x82\x01R`\0\x90`\x80\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R`\0\x80Tc\x8A\x04\xBD\xD5`\xE0\x1B\x84R\x91\x93P\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x8A\x04\xBD\xD5\x90a\x0B!\x900\x90\x8F\x90\x87\x90`\x04\x01a\x11\x98V[`\xA0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0BwW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x12b\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0B\x8BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\xAF\x91\x90a\x12\x15V[P\x92\x9E\x94\x9DP\x93\x9BPPPPPPPPPPPPV[`\0a\x0B\xDA\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x0C\"V[\x90P[\x92\x91PPV[`\0a\x0B\xDA\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x0CPV[`\0a\x0B\xDA\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x0C\"V[`\0a\x0B\xDA\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x0CPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x0C:W`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x0ChW`\0\x80\xFD[\x04\x92\x91PPV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[\x80\x15\x15\x81\x14a\x0C\xCDW`\0\x80\xFD[PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x0C\xE8Wa\x0C\xE8a\x0CoV[\x835\x92P` \x84\x015a\x0C\xFA\x81a\x0C\xBFV[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0[\x83\x81\x10\x15a\r&W\x81\x81\x01Q\x83\x82\x01R` \x01a\r\x0EV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\rG\x81` \x86\x01` \x86\x01a\r\x0BV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[\x83\x15\x15\x81R\x82` \x82\x01R```@\x82\x01R`\0a\r|``\x83\x01\x84a\r/V[\x95\x94PPPPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\r\xBEWa\r\xBEa\r\x85V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\r\xEDWa\r\xEDa\r\x85V[`@R\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0C\xCDW`\0\x80\xFD[`\0\x80`\0\x83\x85\x03`\xA0\x81\x12\x15a\x0E#Wa\x0E#a\x0CoV[\x845\x93P` \x85\x015\x92P```?\x19\x82\x01\x12\x15a\x0E\x8CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FABI decoding: struct data too sh`D\x82\x01Rb\x1B\xDC\x9D`\xEA\x1B`d\x82\x01R`\x84\x81\xFD[Pa\x0E\x95a\r\x9BV[`@\x85\x015\x81R``\x85\x015` \x82\x01R`\x80\x85\x015a\x0E\xB4\x81a\r\xF5V[`@\x82\x01R\x92\x95\x91\x94P\x91\x92P\x90PV[` \x81R`\0a\x0B\xDA` \x83\x01\x84a\r/V[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x0E\xF1Wa\x0E\xF1a\x0CoV[\x845\x93P` \x85\x015a\x0F\x03\x81a\x0C\xBFV[\x93\x96\x93\x95PPPP`@\x82\x015\x91``\x015\x90V[\x82\x15\x15\x81R`@` \x82\x01R`\0a\x0F3`@\x83\x01\x84a\r/V[\x94\x93PPPPV[`\0` \x82\x84\x03\x12\x15a\x0FPWa\x0FPa\x0CoV[\x81Qa\x0F[\x81a\r\xF5V[\x93\x92PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x0FzWa\x0Fza\x0CoV[\x83Q\x92P` \x84\x01Q\x91P`@\x84\x01Q\x90P\x92P\x92P\x92V[`\0` \x80\x83\x85\x03\x12\x15a\x0F\xA9Wa\x0F\xA9a\x0CoV[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x10\x0CW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x84\x90R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x10tW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x84\x90R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x81\xFD[\x81Q\x81\x81\x11\x15a\x10\x86Wa\x10\x86a\r\x85V[a\x10\x98`\x1F\x82\x01`\x1F\x19\x16\x85\x01a\r\xC4V[\x91P\x80\x82R\x86\x84\x82\x85\x01\x01\x11\x15a\x10\xFEW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x85\x90R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R`\x84\x81\xFD[a\x11\r\x81\x85\x84\x01\x86\x86\x01a\r\x0BV[P\x95\x94PPPPPV[`\0``\x82\x84\x03\x12\x15a\x11,Wa\x11,a\x0CoV[a\x114a\r\x9BV[\x82Q\x81R` \x83\x01Q` \x82\x01R`@\x83\x01Qa\x11P\x81a\r\xF5V[`@\x82\x01R\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x0B\xDDWa\x0B\xDDa\x11\\V[\x80\x82\x01\x80\x82\x11\x15a\x0B\xDDWa\x0B\xDDa\x11\\V[`\x01\x80`\xA0\x1B\x03\x84\x16\x81R\x82` \x82\x01R```@\x82\x01R`\0a\r|``\x83\x01\x84a\r/V[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a\x11\xDBWa\x11\xDBa\x0CoV[\x86Qa\x11\xE6\x81a\x0C\xBFV[` \x88\x01Q`@\x89\x01Q``\x8A\x01Q`\x80\x8B\x01Q`\xA0\x90\x9B\x01Q\x93\x9C\x92\x9BP\x90\x99\x90\x98P\x96P\x90\x94P\x92PPPV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\x120Wa\x120a\x0CoV[\x85Qa\x12;\x81a\x0C\xBFV[` \x87\x01Q`@\x88\x01Q``\x89\x01Q`\x80\x90\x99\x01Q\x92\x9A\x91\x99P\x97\x96P\x90\x94P\x92PPPV\xFETarget contract does not contain\xA2dipfsX\"\x12 6\xB7\xDD\\\x0B\x8F7\xA9\xE3\xBB\xF2hM\x1F\x08\xC4\xBC8Cq\xF8\x99O\x04\x85@x\x96\x18\\\x98\x10dsolcC\0\x08\x16\x003";
    /// The bytecode of the contract.
    pub static CONSTANTSUMSOLVER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\0\x99W`\x005`\xE0\x1C\x80c9(\xFF\x97\x14a\0\xFEW\x80c\x89\xEA\x85Y\x14a\x01)W\x80c\x8A\x1A \xDE\x14a\x01IW\x80c\xA8\xC6.v\x14a\x01jW[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x01\x11a\x01\x0C6`\x04a\x0C\xD0V[a\x01\x95V[`@Qa\x01 \x93\x92\x91\x90a\r[V[`@Q\x80\x91\x03\x90\xF3[a\x01<a\x0176`\x04a\x0E\nV[a\x06\xCDV[`@Qa\x01 \x91\x90a\x0E\xC5V[a\x01\\a\x01W6`\x04a\x0E\xD8V[a\x07CV[`@Qa\x01 \x92\x91\x90a\x0F\x18V[`\0Ta\x01}\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01 V[`\0\x80``a\x01\xBE`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[a\x01\xE2`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`\0\x80T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xAF\xBA\x13\xC4`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x02lW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x12b\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x02\x80W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\xA4\x91\x90a\x0F;V[`\x01`\x01`\xA0\x1B\x03\x16c\xCE\x15;\xF4\x89`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x02\xD1\x91\x81R` \x01\x90V[```@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x03'W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x12b\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x03;W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03_\x91\x90a\x0FbV[`@\x85\x81\x01\x91\x90\x91R` \x85\x01\x91\x90\x91R\x90\x83R`\0\x80T\x91Qc\xDC\x17\x83U`\xE0\x1B\x81R`\x04\x81\x01\x8B\x90R\x90\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xDC\x17\x83U\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x03\xF5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x12b\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x04\tW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x041\x91\x90\x81\x01\x90a\x0F\x93V[\x80` \x01\x90Q\x81\x01\x90a\x04D\x91\x90a\x11\x17V[\x90P`\0\x88\x15a\x04\xFEW`\0a\x04g\x83` \x01Q\x8Aa\x0B\xC5\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa\x04\x97\x83` \x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x04\x84\x91\x90a\x11rV[\x84Qa\x04\x91\x90\x8C\x90a\x0B\xE3V[\x90a\x0B\xE3V[\x85Q\x90\x92Pa\x04\xA7\x90\x8A\x90a\x11\x85V[\x84R`@\x85\x01Qa\x04\xB9\x90\x82\x90a\x11\x85V[`@\x85\x01R` \x85\x01Q\x82\x11\x15a\x04\xE3W`@QcC#\xA5U`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x85` \x01Qa\x04\xF3\x91\x90a\x11rV[` \x85\x01RPa\x05\xA8V[\x81Q` \x83\x01Q`\0\x91a\x05\x1D\x91a\x05\x17\x90\x8C\x90a\x0B\xC5V[\x90a\x0B\xF8V[\x90Pa\x05I\x83`\0\x01Qa\x05C\x8B\x86` \x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x04\x91\x91\x90a\x11rV[\x90a\x0C\rV[\x91P\x88\x85` \x01Qa\x05[\x91\x90a\x11\x85V[` \x85\x01R`@\x85\x01Qa\x05p\x90\x82\x90a\x11\x85V[`@\x85\x01R\x84Q\x82\x11\x15a\x05\x97W`@QcC#\xA5U`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x84Qa\x05\xA4\x90\x83\x90a\x11rV[\x84RP[`@\x80Q\x84Q` \x80\x83\x01\x91\x90\x91R\x85\x01Q\x81\x83\x01R\x90\x84\x01Q``\x82\x01R`\0\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16ch\xBD>80\x8E\x85`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x06$\x93\x92\x91\x90a\x11\x98V[`\xC0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x06zW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x12b\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x06\x8EW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xB2\x91\x90a\x11\xBFV[P\x93\x9CP\x95\x9AP\x93\x98PPPPPPPPP\x93P\x93P\x93\x90PV[\x80Q``\x90`\0\x90a\x06\xE0\x90\x85\x90a\x0B\xF8V[a\x06\xEA\x90\x86a\x11\x85V[`@\x80Q` \x80\x82\x01\x89\x90R\x81\x83\x01\x88\x90R``\x82\x01\x84\x90R\x86Q`\x80\x83\x01R\x86\x01Q`\xA0\x82\x01R\x90\x85\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\xC0\x82\x01R\x90\x91P`\xE0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91PP\x93\x92PPPV[`\0``a\x07k`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[a\x07\x8F`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`\0\x80T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xAF\xBA\x13\xC4`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x08\x19W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x12b\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x08-W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08Q\x91\x90a\x0F;V[`\x01`\x01`\xA0\x1B\x03\x16c\xCE\x15;\xF4\x89`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x08~\x91\x81R` \x01\x90V[```@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x08\xD4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x12b\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x08\xE8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\x0C\x91\x90a\x0FbV[`@\x85\x81\x01\x91\x90\x91R` \x85\x01\x91\x90\x91R\x90\x83R`\0\x80T\x91Qc\xDC\x17\x83U`\xE0\x1B\x81R`\x04\x81\x01\x8B\x90R\x90\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xDC\x17\x83U\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\t\xA2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x12b\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\t\xB6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\t\xDE\x91\x90\x81\x01\x90a\x0F\x93V[\x80` \x01\x90Q\x81\x01\x90a\t\xF1\x91\x90a\x11\x17V[\x90P\x87\x15a\nAW\x82Qa\n\x06\x90\x88\x90a\x11\x85V[\x82R` \x83\x01Qa\n\x18\x90\x87\x90a\x11\x85V[` \x83\x01\x81\x90R\x81Qa\n+\x91\x90a\x0B\xF8V[\x82Qa\n7\x91\x90a\x11\x85V[`@\x83\x01Ra\n\xB5V[\x82Q\x87\x11\x80a\nSWP\x85\x83` \x01Q\x10[\x15a\nqW`@QcC#\xA5U`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82Qa\n~\x90\x88\x90a\x11rV[\x82R` \x83\x01Qa\n\x90\x90\x87\x90a\x11rV[` \x83\x01\x81\x90R\x81Qa\n\xA3\x91\x90a\x0B\xF8V[\x82Qa\n\xAF\x91\x90a\x11\x85V[`@\x83\x01R[`@\x80Q\x83Q` \x80\x83\x01\x91\x90\x91R\x84\x01Q\x81\x83\x01R\x90\x83\x01Q``\x82\x01R`\0\x90`\x80\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R`\0\x80Tc\x8A\x04\xBD\xD5`\xE0\x1B\x84R\x91\x93P\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x8A\x04\xBD\xD5\x90a\x0B!\x900\x90\x8F\x90\x87\x90`\x04\x01a\x11\x98V[`\xA0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0BwW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x12b\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0B\x8BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\xAF\x91\x90a\x12\x15V[P\x92\x9E\x94\x9DP\x93\x9BPPPPPPPPPPPPV[`\0a\x0B\xDA\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x0C\"V[\x90P[\x92\x91PPV[`\0a\x0B\xDA\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x0CPV[`\0a\x0B\xDA\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x0C\"V[`\0a\x0B\xDA\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x0CPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x0C:W`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x0ChW`\0\x80\xFD[\x04\x92\x91PPV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[\x80\x15\x15\x81\x14a\x0C\xCDW`\0\x80\xFD[PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x0C\xE8Wa\x0C\xE8a\x0CoV[\x835\x92P` \x84\x015a\x0C\xFA\x81a\x0C\xBFV[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0[\x83\x81\x10\x15a\r&W\x81\x81\x01Q\x83\x82\x01R` \x01a\r\x0EV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\rG\x81` \x86\x01` \x86\x01a\r\x0BV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[\x83\x15\x15\x81R\x82` \x82\x01R```@\x82\x01R`\0a\r|``\x83\x01\x84a\r/V[\x95\x94PPPPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\r\xBEWa\r\xBEa\r\x85V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\r\xEDWa\r\xEDa\r\x85V[`@R\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0C\xCDW`\0\x80\xFD[`\0\x80`\0\x83\x85\x03`\xA0\x81\x12\x15a\x0E#Wa\x0E#a\x0CoV[\x845\x93P` \x85\x015\x92P```?\x19\x82\x01\x12\x15a\x0E\x8CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FABI decoding: struct data too sh`D\x82\x01Rb\x1B\xDC\x9D`\xEA\x1B`d\x82\x01R`\x84\x81\xFD[Pa\x0E\x95a\r\x9BV[`@\x85\x015\x81R``\x85\x015` \x82\x01R`\x80\x85\x015a\x0E\xB4\x81a\r\xF5V[`@\x82\x01R\x92\x95\x91\x94P\x91\x92P\x90PV[` \x81R`\0a\x0B\xDA` \x83\x01\x84a\r/V[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x0E\xF1Wa\x0E\xF1a\x0CoV[\x845\x93P` \x85\x015a\x0F\x03\x81a\x0C\xBFV[\x93\x96\x93\x95PPPP`@\x82\x015\x91``\x015\x90V[\x82\x15\x15\x81R`@` \x82\x01R`\0a\x0F3`@\x83\x01\x84a\r/V[\x94\x93PPPPV[`\0` \x82\x84\x03\x12\x15a\x0FPWa\x0FPa\x0CoV[\x81Qa\x0F[\x81a\r\xF5V[\x93\x92PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x0FzWa\x0Fza\x0CoV[\x83Q\x92P` \x84\x01Q\x91P`@\x84\x01Q\x90P\x92P\x92P\x92V[`\0` \x80\x83\x85\x03\x12\x15a\x0F\xA9Wa\x0F\xA9a\x0CoV[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x10\x0CW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x84\x90R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x10tW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x84\x90R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x81\xFD[\x81Q\x81\x81\x11\x15a\x10\x86Wa\x10\x86a\r\x85V[a\x10\x98`\x1F\x82\x01`\x1F\x19\x16\x85\x01a\r\xC4V[\x91P\x80\x82R\x86\x84\x82\x85\x01\x01\x11\x15a\x10\xFEW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x85\x90R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R`\x84\x81\xFD[a\x11\r\x81\x85\x84\x01\x86\x86\x01a\r\x0BV[P\x95\x94PPPPPV[`\0``\x82\x84\x03\x12\x15a\x11,Wa\x11,a\x0CoV[a\x114a\r\x9BV[\x82Q\x81R` \x83\x01Q` \x82\x01R`@\x83\x01Qa\x11P\x81a\r\xF5V[`@\x82\x01R\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x0B\xDDWa\x0B\xDDa\x11\\V[\x80\x82\x01\x80\x82\x11\x15a\x0B\xDDWa\x0B\xDDa\x11\\V[`\x01\x80`\xA0\x1B\x03\x84\x16\x81R\x82` \x82\x01R```@\x82\x01R`\0a\r|``\x83\x01\x84a\r/V[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a\x11\xDBWa\x11\xDBa\x0CoV[\x86Qa\x11\xE6\x81a\x0C\xBFV[` \x88\x01Q`@\x89\x01Q``\x8A\x01Q`\x80\x8B\x01Q`\xA0\x90\x9B\x01Q\x93\x9C\x92\x9BP\x90\x99\x90\x98P\x96P\x90\x94P\x92PPPV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\x120Wa\x120a\x0CoV[\x85Qa\x12;\x81a\x0C\xBFV[` \x87\x01Q`@\x88\x01Q``\x89\x01Q`\x80\x90\x99\x01Q\x92\x9A\x91\x99P\x97\x96P\x90\x94P\x92PPPV\xFETarget contract does not contain\xA2dipfsX\"\x12 6\xB7\xDD\\\x0B\x8F7\xA9\xE3\xBB\xF2hM\x1F\x08\xC4\xBC8Cq\xF8\x99O\x04\x85@x\x96\x18\\\x98\x10dsolcC\0\x08\x16\x003";
    /// The deployed bytecode of the contract.
    pub static CONSTANTSUMSOLVER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct ConstantSumSolver<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ConstantSumSolver<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ConstantSumSolver<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ConstantSumSolver<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ConstantSumSolver<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(ConstantSumSolver))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ConstantSumSolver<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    CONSTANTSUMSOLVER_ABI.clone(),
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
                CONSTANTSUMSOLVER_ABI.clone(),
                CONSTANTSUMSOLVER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `getInitialPoolData` (0x89ea8559) function
        pub fn get_initial_pool_data(
            &self,
            rx: ::ethers::core::types::U256,
            ry: ::ethers::core::types::U256,
            params: ConstantSumParams,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([137, 234, 133, 89], (rx, ry, params))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `simulateAllocateOrDeallocate` (0x8a1a20de) function
        pub fn simulate_allocate_or_deallocate(
            &self,
            pool_id: ::ethers::core::types::U256,
            is_allocate: bool,
            amount_x: ::ethers::core::types::U256,
            amount_y: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (bool, ::ethers::core::types::Bytes),
        > {
            self.0
                .method_hash(
                    [138, 26, 32, 222],
                    (pool_id, is_allocate, amount_x, amount_y),
                )
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
            (bool, ::ethers::core::types::U256, ::ethers::core::types::Bytes),
        > {
            self.0
                .method_hash([57, 40, 255, 151], (pool_id, swap_x_in, amount_in))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `strategy` (0xa8c62e76) function
        pub fn strategy(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([168, 198, 46, 118], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for ConstantSumSolver<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `NotEnoughLiquidity` with signature `NotEnoughLiquidity()` and selector `0x4323a555`
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
    #[etherror(name = "NotEnoughLiquidity", abi = "NotEnoughLiquidity()")]
    pub struct NotEnoughLiquidity;
    ///Container type for all input parameters for the `getInitialPoolData` function with signature `getInitialPoolData(uint256,uint256,(uint256,uint256,address))` and selector `0x89ea8559`
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
        name = "getInitialPoolData",
        abi = "getInitialPoolData(uint256,uint256,(uint256,uint256,address))"
    )]
    pub struct GetInitialPoolDataCall {
        pub rx: ::ethers::core::types::U256,
        pub ry: ::ethers::core::types::U256,
        pub params: ConstantSumParams,
    }
    ///Container type for all input parameters for the `simulateAllocateOrDeallocate` function with signature `simulateAllocateOrDeallocate(uint256,bool,uint256,uint256)` and selector `0x8a1a20de`
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
        name = "simulateAllocateOrDeallocate",
        abi = "simulateAllocateOrDeallocate(uint256,bool,uint256,uint256)"
    )]
    pub struct SimulateAllocateOrDeallocateCall {
        pub pool_id: ::ethers::core::types::U256,
        pub is_allocate: bool,
        pub amount_x: ::ethers::core::types::U256,
        pub amount_y: ::ethers::core::types::U256,
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
        Hash
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
        Hash
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
        Hash
    )]
    pub enum ConstantSumSolverCalls {
        GetInitialPoolData(GetInitialPoolDataCall),
        SimulateAllocateOrDeallocate(SimulateAllocateOrDeallocateCall),
        SimulateSwap(SimulateSwapCall),
        Strategy(StrategyCall),
    }
    impl ::ethers::core::abi::AbiDecode for ConstantSumSolverCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <GetInitialPoolDataCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetInitialPoolData(decoded));
            }
            if let Ok(decoded) = <SimulateAllocateOrDeallocateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SimulateAllocateOrDeallocate(decoded));
            }
            if let Ok(decoded) = <SimulateSwapCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SimulateSwap(decoded));
            }
            if let Ok(decoded) = <StrategyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Strategy(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ConstantSumSolverCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::GetInitialPoolData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SimulateAllocateOrDeallocate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SimulateSwap(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Strategy(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for ConstantSumSolverCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::GetInitialPoolData(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SimulateAllocateOrDeallocate(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SimulateSwap(element) => ::core::fmt::Display::fmt(element, f),
                Self::Strategy(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<GetInitialPoolDataCall> for ConstantSumSolverCalls {
        fn from(value: GetInitialPoolDataCall) -> Self {
            Self::GetInitialPoolData(value)
        }
    }
    impl ::core::convert::From<SimulateAllocateOrDeallocateCall>
    for ConstantSumSolverCalls {
        fn from(value: SimulateAllocateOrDeallocateCall) -> Self {
            Self::SimulateAllocateOrDeallocate(value)
        }
    }
    impl ::core::convert::From<SimulateSwapCall> for ConstantSumSolverCalls {
        fn from(value: SimulateSwapCall) -> Self {
            Self::SimulateSwap(value)
        }
    }
    impl ::core::convert::From<StrategyCall> for ConstantSumSolverCalls {
        fn from(value: StrategyCall) -> Self {
            Self::Strategy(value)
        }
    }
    ///Container type for all return fields from the `getInitialPoolData` function with signature `getInitialPoolData(uint256,uint256,(uint256,uint256,address))` and selector `0x89ea8559`
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
    pub struct GetInitialPoolDataReturn(pub ::ethers::core::types::Bytes);
    ///Container type for all return fields from the `simulateAllocateOrDeallocate` function with signature `simulateAllocateOrDeallocate(uint256,bool,uint256,uint256)` and selector `0x8a1a20de`
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
    pub struct SimulateAllocateOrDeallocateReturn(
        pub bool,
        pub ::ethers::core::types::Bytes,
    );
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
        Hash
    )]
    pub struct SimulateSwapReturn(
        pub bool,
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
        Hash
    )]
    pub struct StrategyReturn(pub ::ethers::core::types::Address);
    ///`ConstantSumParams(uint256,uint256,address)`
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
    pub struct ConstantSumParams {
        pub price: ::ethers::core::types::U256,
        pub swap_fee: ::ethers::core::types::U256,
        pub controller: ::ethers::core::types::Address,
    }
}
