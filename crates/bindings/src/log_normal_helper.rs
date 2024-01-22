pub use log_normal_helper::*;
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
pub mod log_normal_helper {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("logNormal_"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
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
                                    name: ::std::borrow::ToOwned::to_owned("params"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct LogNormal.LogNormalParams",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("logNormal"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("logNormal"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract LogNormal"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("prepareFeeUpdate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("prepareFeeUpdate"),
                            inputs: ::std::vec![
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
                    ::std::borrow::ToOwned::to_owned("prepareSigmaUpdate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("prepareSigmaUpdate"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("targetSigma"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("targetTimestamp"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("prepareStrikeUpdate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "prepareStrikeUpdate",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("targetStrike"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("targetTimestamp"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("prepareTauUpdate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("prepareTauUpdate"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("targetTau"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("targetTimestamp"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static LOGNORMALHELPER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA0`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`@Qa\x08\x948\x03\x80a\x08\x94\x839\x81\x01`@\x81\x90Ra\0|\x91a\0\x8DV[`\x01`\x01`\xA0\x1B\x03\x16`\x80Ra\x01\x08V[`\0` \x82\x84\x03\x12\x15a\0\xEAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01\x01W`\0\x80\xFD[\x93\x92PPPV[`\x80Qa\x07ja\x01*`\09`\0\x81\x81a\x01\xC1\x01Ra\x02o\x01Ra\x07j`\0\xF3\xFE`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\0\xAFW`\x005`\xE0\x1C\x80c\x04 X\n\x14a\x01\x14W\x80c;&\x8D]\x14a\x01=W\x80c\xB0\x9D\x04\xE5\x14a\x01PW\x80c\xDC\x17\x83U\x14a\x01cW\x80c\xE9G\x16\xD5\x14a\x01\xA9W\x80c\xFB\xF1{{\x14a\x01\xBCW[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x01'a\x01\"6`\x04a\x04BV[a\x01\xFBV[`@Qa\x014\x91\x90a\x04\x8BV[`@Q\x80\x91\x03\x90\xF3[a\x01'a\x01K6`\x04a\x04BV[a\x02\x0EV[a\x01'a\x01^6`\x04a\x04\xBEV[a\x02\x1AV[a\x01va\x01q6`\x04a\x04\xBEV[a\x02+V[`@Qa\x014\x91\x90\x81Q\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x80\x83\x01Q\x90\x82\x01R``\x91\x82\x01Q\x91\x81\x01\x91\x90\x91R`\x80\x01\x90V[a\x01'a\x01\xB76`\x04a\x04BV[a\x03[V[a\x01\xE3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x014V[``a\x02\x07\x83\x83a\x03gV[\x93\x92PPPV[``a\x02\x07\x83\x83a\x03\x96V[``a\x02%\x82a\x03\xAEV[\x92\x91PPV[a\x02V`@Q\x80`\x80\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`@Qc\xDC\x17\x83U`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R`\0\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xDC\x17\x83U\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x03\tW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x03\x1DW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x03E\x91\x90\x81\x01\x90a\x05!V[\x90P\x80\x80` \x01\x90Q\x81\x01\x90a\x02\x07\x91\x90a\x06\xA5V[``a\x02\x07\x83\x83a\x03\xDAV[```\x01\x83\x83`@Q` \x01a\x03\x7F\x93\x92\x91\x90a\x070V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x92\x91PPV[```\x03\x83\x83`@Q` \x01a\x03\x7F\x93\x92\x91\x90a\x070V[```\0\x82`@Q` \x01a\x03\xC4\x92\x91\x90a\x07OV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x91\x90PV[```\x02\x83\x83`@Q` \x01a\x03\x7F\x93\x92\x91\x90a\x070V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x04XWa\x04Xa\x03\xF2V[PP\x805\x92` \x90\x91\x015\x91PV[`\0[\x83\x81\x10\x15a\x04\x82W\x81\x81\x01Q\x83\x82\x01R` \x01a\x04jV[PP`\0\x91\x01RV[` \x81R`\0\x82Q\x80` \x84\x01Ra\x04\xAA\x81`@\x85\x01` \x87\x01a\x04gV[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x04\xD3Wa\x04\xD3a\x03\xF2V[P5\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x05\x19Wa\x05\x19a\x04\xDAV[`@R\x91\x90PV[`\0` \x80\x83\x85\x03\x12\x15a\x057Wa\x057a\x03\xF2V[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x05\x9AW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x84\x90R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x06\x02W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x84\x90R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x81\xFD[\x81Q\x81\x81\x11\x15a\x06\x14Wa\x06\x14a\x04\xDAV[a\x06&`\x1F\x82\x01`\x1F\x19\x16\x85\x01a\x04\xF0V[\x91P\x80\x82R\x86\x84\x82\x85\x01\x01\x11\x15a\x06\x8CW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x85\x90R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R`\x84\x81\xFD[a\x06\x9B\x81\x85\x84\x01\x86\x86\x01a\x04gV[P\x95\x94PPPPPV[`\0`\x80\x82\x84\x03\x12\x15a\x06\xBAWa\x06\xBAa\x03\xF2V[`@Q`\x80\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x06\xDDWa\x06\xDDa\x04\xDAV[\x80`@RP\x82Q\x81R` \x83\x01Q` \x82\x01R`@\x83\x01Q`@\x82\x01R``\x83\x01Q``\x82\x01R\x80\x91PP\x92\x91PPV[`\x04\x81\x10a\x07,WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x90RV[``\x81\x01a\x07>\x82\x86a\x07\x0EV[` \x82\x01\x93\x90\x93R`@\x01R\x91\x90PV[`@\x81\x01a\x07]\x82\x85a\x07\x0EV[\x82` \x83\x01R\x93\x92PPPV";
    /// The bytecode of the contract.
    pub static LOGNORMALHELPER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\0\xAFW`\x005`\xE0\x1C\x80c\x04 X\n\x14a\x01\x14W\x80c;&\x8D]\x14a\x01=W\x80c\xB0\x9D\x04\xE5\x14a\x01PW\x80c\xDC\x17\x83U\x14a\x01cW\x80c\xE9G\x16\xD5\x14a\x01\xA9W\x80c\xFB\xF1{{\x14a\x01\xBCW[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x01'a\x01\"6`\x04a\x04BV[a\x01\xFBV[`@Qa\x014\x91\x90a\x04\x8BV[`@Q\x80\x91\x03\x90\xF3[a\x01'a\x01K6`\x04a\x04BV[a\x02\x0EV[a\x01'a\x01^6`\x04a\x04\xBEV[a\x02\x1AV[a\x01va\x01q6`\x04a\x04\xBEV[a\x02+V[`@Qa\x014\x91\x90\x81Q\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x80\x83\x01Q\x90\x82\x01R``\x91\x82\x01Q\x91\x81\x01\x91\x90\x91R`\x80\x01\x90V[a\x01'a\x01\xB76`\x04a\x04BV[a\x03[V[a\x01\xE3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x014V[``a\x02\x07\x83\x83a\x03gV[\x93\x92PPPV[``a\x02\x07\x83\x83a\x03\x96V[``a\x02%\x82a\x03\xAEV[\x92\x91PPV[a\x02V`@Q\x80`\x80\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`@Qc\xDC\x17\x83U`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R`\0\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xDC\x17\x83U\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x03\tW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x03\x1DW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x03E\x91\x90\x81\x01\x90a\x05!V[\x90P\x80\x80` \x01\x90Q\x81\x01\x90a\x02\x07\x91\x90a\x06\xA5V[``a\x02\x07\x83\x83a\x03\xDAV[```\x01\x83\x83`@Q` \x01a\x03\x7F\x93\x92\x91\x90a\x070V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x92\x91PPV[```\x03\x83\x83`@Q` \x01a\x03\x7F\x93\x92\x91\x90a\x070V[```\0\x82`@Q` \x01a\x03\xC4\x92\x91\x90a\x07OV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x91\x90PV[```\x02\x83\x83`@Q` \x01a\x03\x7F\x93\x92\x91\x90a\x070V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x04XWa\x04Xa\x03\xF2V[PP\x805\x92` \x90\x91\x015\x91PV[`\0[\x83\x81\x10\x15a\x04\x82W\x81\x81\x01Q\x83\x82\x01R` \x01a\x04jV[PP`\0\x91\x01RV[` \x81R`\0\x82Q\x80` \x84\x01Ra\x04\xAA\x81`@\x85\x01` \x87\x01a\x04gV[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x04\xD3Wa\x04\xD3a\x03\xF2V[P5\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x05\x19Wa\x05\x19a\x04\xDAV[`@R\x91\x90PV[`\0` \x80\x83\x85\x03\x12\x15a\x057Wa\x057a\x03\xF2V[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x05\x9AW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x84\x90R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x06\x02W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x84\x90R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x81\xFD[\x81Q\x81\x81\x11\x15a\x06\x14Wa\x06\x14a\x04\xDAV[a\x06&`\x1F\x82\x01`\x1F\x19\x16\x85\x01a\x04\xF0V[\x91P\x80\x82R\x86\x84\x82\x85\x01\x01\x11\x15a\x06\x8CW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x85\x90R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R`\x84\x81\xFD[a\x06\x9B\x81\x85\x84\x01\x86\x86\x01a\x04gV[P\x95\x94PPPPPV[`\0`\x80\x82\x84\x03\x12\x15a\x06\xBAWa\x06\xBAa\x03\xF2V[`@Q`\x80\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x06\xDDWa\x06\xDDa\x04\xDAV[\x80`@RP\x82Q\x81R` \x83\x01Q` \x82\x01R`@\x83\x01Q`@\x82\x01R``\x83\x01Q``\x82\x01R\x80\x91PP\x92\x91PPV[`\x04\x81\x10a\x07,WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x90RV[``\x81\x01a\x07>\x82\x86a\x07\x0EV[` \x82\x01\x93\x90\x93R`@\x01R\x91\x90PV[`@\x81\x01a\x07]\x82\x85a\x07\x0EV[\x82` \x83\x01R\x93\x92PPPV";
    /// The deployed bytecode of the contract.
    pub static LOGNORMALHELPER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct LogNormalHelper<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for LogNormalHelper<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for LogNormalHelper<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for LogNormalHelper<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for LogNormalHelper<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(LogNormalHelper))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> LogNormalHelper<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    LOGNORMALHELPER_ABI.clone(),
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
                LOGNORMALHELPER_ABI.clone(),
                LOGNORMALHELPER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `getPoolParams` (0xdc178355) function
        pub fn get_pool_params(
            &self,
            pool_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, LogNormalParams> {
            self.0
                .method_hash([220, 23, 131, 85], pool_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `logNormal` (0xfbf17b7b) function
        pub fn log_normal(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([251, 241, 123, 123], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `prepareFeeUpdate` (0xb09d04e5) function
        pub fn prepare_fee_update(
            &self,
            swap_fee: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([176, 157, 4, 229], swap_fee)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `prepareSigmaUpdate` (0xe94716d5) function
        pub fn prepare_sigma_update(
            &self,
            target_sigma: ::ethers::core::types::U256,
            target_timestamp: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([233, 71, 22, 213], (target_sigma, target_timestamp))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `prepareStrikeUpdate` (0x0420580a) function
        pub fn prepare_strike_update(
            &self,
            target_strike: ::ethers::core::types::U256,
            target_timestamp: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([4, 32, 88, 10], (target_strike, target_timestamp))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `prepareTauUpdate` (0x3b268d5d) function
        pub fn prepare_tau_update(
            &self,
            target_tau: ::ethers::core::types::U256,
            target_timestamp: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([59, 38, 141, 93], (target_tau, target_timestamp))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for LogNormalHelper<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
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
    ///Container type for all input parameters for the `logNormal` function with signature `logNormal()` and selector `0xfbf17b7b`
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
    #[ethcall(name = "logNormal", abi = "logNormal()")]
    pub struct LogNormalCall;
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
        Hash
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
        Hash
    )]
    #[ethcall(name = "prepareSigmaUpdate", abi = "prepareSigmaUpdate(uint256,uint256)")]
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
        Hash
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
        Hash
    )]
    #[ethcall(name = "prepareTauUpdate", abi = "prepareTauUpdate(uint256,uint256)")]
    pub struct PrepareTauUpdateCall {
        pub target_tau: ::ethers::core::types::U256,
        pub target_timestamp: ::ethers::core::types::U256,
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
    pub enum LogNormalHelperCalls {
        GetPoolParams(GetPoolParamsCall),
        LogNormal(LogNormalCall),
        PrepareFeeUpdate(PrepareFeeUpdateCall),
        PrepareSigmaUpdate(PrepareSigmaUpdateCall),
        PrepareStrikeUpdate(PrepareStrikeUpdateCall),
        PrepareTauUpdate(PrepareTauUpdateCall),
    }
    impl ::ethers::core::abi::AbiDecode for LogNormalHelperCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <GetPoolParamsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetPoolParams(decoded));
            }
            if let Ok(decoded) = <LogNormalCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LogNormal(decoded));
            }
            if let Ok(decoded) = <PrepareFeeUpdateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PrepareFeeUpdate(decoded));
            }
            if let Ok(decoded) = <PrepareSigmaUpdateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PrepareSigmaUpdate(decoded));
            }
            if let Ok(decoded) = <PrepareStrikeUpdateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PrepareStrikeUpdate(decoded));
            }
            if let Ok(decoded) = <PrepareTauUpdateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PrepareTauUpdate(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for LogNormalHelperCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::GetPoolParams(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LogNormal(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PrepareFeeUpdate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PrepareSigmaUpdate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PrepareStrikeUpdate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PrepareTauUpdate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for LogNormalHelperCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::GetPoolParams(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogNormal(element) => ::core::fmt::Display::fmt(element, f),
                Self::PrepareFeeUpdate(element) => ::core::fmt::Display::fmt(element, f),
                Self::PrepareSigmaUpdate(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PrepareStrikeUpdate(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PrepareTauUpdate(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<GetPoolParamsCall> for LogNormalHelperCalls {
        fn from(value: GetPoolParamsCall) -> Self {
            Self::GetPoolParams(value)
        }
    }
    impl ::core::convert::From<LogNormalCall> for LogNormalHelperCalls {
        fn from(value: LogNormalCall) -> Self {
            Self::LogNormal(value)
        }
    }
    impl ::core::convert::From<PrepareFeeUpdateCall> for LogNormalHelperCalls {
        fn from(value: PrepareFeeUpdateCall) -> Self {
            Self::PrepareFeeUpdate(value)
        }
    }
    impl ::core::convert::From<PrepareSigmaUpdateCall> for LogNormalHelperCalls {
        fn from(value: PrepareSigmaUpdateCall) -> Self {
            Self::PrepareSigmaUpdate(value)
        }
    }
    impl ::core::convert::From<PrepareStrikeUpdateCall> for LogNormalHelperCalls {
        fn from(value: PrepareStrikeUpdateCall) -> Self {
            Self::PrepareStrikeUpdate(value)
        }
    }
    impl ::core::convert::From<PrepareTauUpdateCall> for LogNormalHelperCalls {
        fn from(value: PrepareTauUpdateCall) -> Self {
            Self::PrepareTauUpdate(value)
        }
    }
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
    pub struct GetPoolParamsReturn {
        pub params: LogNormalParams,
    }
    ///Container type for all return fields from the `logNormal` function with signature `logNormal()` and selector `0xfbf17b7b`
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
    pub struct LogNormalReturn(pub ::ethers::core::types::Address);
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
        Hash
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
        Hash
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
        Hash
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
        Hash
    )]
    pub struct PrepareTauUpdateReturn(pub ::ethers::core::types::Bytes);
}
