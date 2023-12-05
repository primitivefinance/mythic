pub use profit_finder::*;
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
pub mod profit_finder {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("atomic"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("atomic"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract AtomicV2"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("searchLowerPrice"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("searchLowerPrice"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("max_iters"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("epsilon"),
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
                                    name: ::std::borrow::ToOwned::to_owned("maxProfit"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("maxTrade"),
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
                    ::std::borrow::ToOwned::to_owned("searchRaisePrice"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("searchRaisePrice"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("max_iters"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("epsilon"),
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
                                    name: ::std::borrow::ToOwned::to_owned("maxProfit"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("maxTrade"),
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
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("MaximizingProfitTrade"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "MaximizingProfitTrade",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("trade"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("profit"),
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
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static PROFITFINDER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x163\x17\x90Ua\x12J\x80a\0\x7F`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\0\x8EW`\x005`\xE0\x1C\x80c\x0F\xF5s\x1D\x14a\0\xF3W\x80c5(\x93\0\x14a\x01 W\x80c\x91\xE6\xECB\x14a\x013W[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x01\x06a\x01\x016`\x04a\x10\x95V[a\x01^V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01[`@Q\x80\x91\x03\x90\xF3[a\x01\x06a\x01.6`\x04a\x10\x95V[a\x08`V[`\0Ta\x01F\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\x17V[`\0\x80a\x03\xE8\x83\x10a\x01\xB7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7Fepsilon must be less than 1000\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[P`\x01`\xFF\x1B\x90P`\x01\x80g\r\xE0\xB6\xB3\xA7d\0\0`\0\x80a\x01\xD8\x84\x84a\x10\xD0V[\x90P`\0\x80[`\x03a\x01\xEA\x87\x87a\x10\xD0V[a\x01\xF4\x91\x90a\x10\xE9V[a\x01\xFE\x90\x87a\x11\x0BV[\x91P`\x03a\x02\x0C\x87\x87a\x10\xD0V[a\x02\x16\x91\x90a\x10\xE9V[a\x02 \x90\x86a\x10\xD0V[\x90P`\0\x80T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x99\x9B\x93\xAF`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x02\xACW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x12\x05\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x02\xC0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\xE4\x91\x90a\x11\x1EV[`\x01`\x01`\xA0\x1B\x03\x16c@\xC1\x0F\x190a\x02\xFD\x84\x86a\x11\x0BV[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x04\x83\x01R`$\x82\x01R`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x03\x81W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x12\x05\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x03\x95W=`\0\x80>=`\0\xFD[PPPP`\0\x80T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x99\x9B\x93\xAF`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x04#W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x12\x05\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x047W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04[\x91\x90a\x11\x1EV[`\0T`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x91c\t^\xA7\xB3\x91\x16a\x04{\x84\x86a\x11\x0BV[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x04\x83\x01R`$\x82\x01R`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x04\xFFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x12\x05\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x05\x13W=`\0\x80>=`\0\xFD[PP`\0\x80T`@Qc\xD7\x82=\xF7`\xE0\x1B\x81R`\x04\x81\x01\x87\x90R\x91\x93P\x83\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\xD7\x82=\xF7\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x05\x9EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x12\x05\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x92PPP\x80\x15a\x05\xAFWP`\x01[a\x06\x05W=\x80\x80\x15a\x05\xDDW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x05\xE2V[``\x91P[P`\x04\x81\x01\x90P\x80\x80` \x01\x90Q\x81\x01\x90a\x05\xFD\x91\x90a\x11QV[\x92PPa\x06\rV[`\x01`\xFF\x1B\x91P[`\0T`@Qc\xD7\x82=\xF7`\xE0\x1B\x81R`\x04\x81\x01\x85\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xD7\x82=\xF7\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x06\x91W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x12\x05\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x92PPP\x80\x15a\x06\xA2WP`\x01[a\x06\xF8W=\x80\x80\x15a\x06\xD0W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x06\xD5V[``\x91P[P`\x04\x81\x01\x90P\x80\x80` \x01\x90Q\x81\x01\x90a\x06\xF0\x91\x90a\x11QV[\x91PPa\x06\xFFV[P`\x01`\xFF\x1B[a\x07'`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01eTrade1`\xD0\x1B\x81RP\x85a\x0F;V[a\x07P`@Q\x80`@\x01`@R\x80`\x07\x81R` \x01fProfit1`\xC8\x1B\x81RP\x83a\x0F\x84V[a\x07x`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01e*90\xB22\x99`\xD1\x1B\x81RP\x84a\x0F;V[a\x07\xA1`@Q\x80`@\x01`@R\x80`\x07\x81R` \x01f(97\xB34\xBA\x19`\xC9\x1B\x81RP\x82a\x0F\x84V[a\x07\xCC`@Q\x80`@\x01`@R\x80`\t\x81R` \x01h\x13X^\x14\x1C\x9B\xD9\x9A]`\xBA\x1B\x81RP\x8Ba\x0F\x84V[\x80\x82\x12\x15a\x07\xE2W\x83\x97P\x80\x99P\x82\x98Pa\x07\xECV[\x82\x96P\x81\x99P\x83\x98P[a\x07\xF6\x88\x88a\x10\xD0V[\x94Pa\x08\x03`\x01\x87a\x11\x0BV[\x95PPP\x88\x83\x11\x80\x15a\x08\x15WP\x89\x84\x10[a\x01\xDEWa\x08=`@Q\x80``\x01`@R\x80`%\x81R` \x01a\x12%`%\x919\x85\x89\x8Ba\x0F\xC9V[`@Qc*6\x9F#`\xE0\x1B\x81R`\x04\x81\x01\x88\x90R`$\x81\x01\x89\x90R`D\x01a\x01\xAEV[`\0\x80a\x03\xE8\x83\x10a\x08\xB4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7Fepsilon must be less than 1000\0\0`D\x82\x01R`d\x01a\x01\xAEV[P`\x01`\xFF\x1B\x90P`\x01\x80h\x05k\xC7^-c\x10\0\0`\0\x80a\x08\xD6\x84\x84a\x10\xD0V[\x90P`\0\x80[`\x03a\x08\xE8\x87\x87a\x10\xD0V[a\x08\xF2\x91\x90a\x10\xE9V[a\x08\xFC\x90\x87a\x11\x0BV[\x91P`\x03a\t\n\x87\x87a\x10\xD0V[a\t\x14\x91\x90a\x10\xE9V[a\t\x1E\x90\x86a\x10\xD0V[\x90P`\0\x80T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x99\x9B\x93\xAF`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\t\xAAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x12\x05\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\t\xBEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\xE2\x91\x90a\x11\x1EV[`\x01`\x01`\xA0\x1B\x03\x16c@\xC1\x0F\x190a\t\xFB\x84\x86a\x11\x0BV[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x04\x83\x01R`$\x82\x01R`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\n\x7FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x12\x05\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\n\x93W=`\0\x80>=`\0\xFD[PPPP`\0\x80T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x99\x9B\x93\xAF`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0B!W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x12\x05\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0B5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0BY\x91\x90a\x11\x1EV[`\0T`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x91c\t^\xA7\xB3\x91\x16a\x0By\x84\x86a\x11\x0BV[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x04\x83\x01R`$\x82\x01R`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0B\xFDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x12\x05\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x0C\x11W=`\0\x80>=`\0\xFD[PP`\0\x80T`@Qc\x1D\xF4i\x8F`\xE3\x1B\x81R`\x04\x81\x01\x87\x90R\x91\x93P\x83\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\xEF\xA3Lx\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0C\x9CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x12\x05\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x92PPP\x80\x15a\x0C\xADWP`\x01[a\r\x03W=\x80\x80\x15a\x0C\xDBW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x0C\xE0V[``\x91P[P`\x04\x81\x01\x90P\x80\x80` \x01\x90Q\x81\x01\x90a\x0C\xFB\x91\x90a\x11QV[\x92PPa\r\x0BV[`\x01`\xFF\x1B\x91P[`\0T`@Qc\x1D\xF4i\x8F`\xE3\x1B\x81R`\x04\x81\x01\x85\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xEF\xA3Lx\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\r\x8FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x12\x05\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x92PPP\x80\x15a\r\xA0WP`\x01[a\r\xF6W=\x80\x80\x15a\r\xCEW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\r\xD3V[``\x91P[P`\x04\x81\x01\x90P\x80\x80` \x01\x90Q\x81\x01\x90a\r\xEE\x91\x90a\x11QV[\x91PPa\r\xFDV[P`\x01`\xFF\x1B[a\x0E%`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01eTrade1`\xD0\x1B\x81RP\x85a\x0F;V[a\x0EN`@Q\x80`@\x01`@R\x80`\x07\x81R` \x01fProfit1`\xC8\x1B\x81RP\x83a\x0F\x84V[a\x0Ev`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01e*90\xB22\x99`\xD1\x1B\x81RP\x84a\x0F;V[a\x0E\x9F`@Q\x80`@\x01`@R\x80`\x07\x81R` \x01f(97\xB34\xBA\x19`\xC9\x1B\x81RP\x82a\x0F\x84V[a\x0E\xCA`@Q\x80`@\x01`@R\x80`\t\x81R` \x01h\x13X^\x14\x1C\x9B\xD9\x9A]`\xBA\x1B\x81RP\x8Ba\x0F\x84V[\x80\x82\x12\x15a\x0E\xE0W\x83\x97P\x80\x99P\x82\x98Pa\x0E\xEAV[\x82\x96P\x81\x99P\x83\x98P[a\x0E\xF4\x88\x88a\x10\xD0V[\x94Pa\x0F\x01`\x01\x87a\x11\x0BV[\x95PPP\x88\x83\x11\x80\x15a\x0F\x13WP\x89\x84\x10[a\x08\xDCWa\x08=`@Q\x80``\x01`@R\x80`%\x81R` \x01a\x12%`%\x919\x85\x89\x8Ba\x0F\xC9V[a\x0F\x80\x82\x82`@Q`$\x01a\x0FQ\x92\x91\x90a\x11\xB3V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c-\x83\x9C\xB3`\xE2\x1B\x17\x90Ra\x10\x18V[PPV[a\x0F\x80\x82\x82`@Q`$\x01a\x0F\x9A\x92\x91\x90a\x11\xB3V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\x1ES\x13G`\xE1\x1B\x17\x90Ra\x10\x18V[a\x10\x12\x84\x84\x84\x84`@Q`$\x01a\x0F\xE3\x94\x93\x92\x91\x90a\x11\xD5V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\xA7\xA8xS`\xE0\x1B\x17\x90Ra\x10\x18V[PPPPV[a\x10!\x81a\x10$V[PV[\x80Qjconsole.log` \x83\x01`\0\x80\x84\x83\x85Z\xFAPPPPPV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x10\xABWa\x10\xABa\x10EV[PP\x805\x92` \x90\x91\x015\x91PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x10\xE3Wa\x10\xE3a\x10\xBAV[\x92\x91PPV[`\0\x82a\x11\x06WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[\x80\x82\x01\x80\x82\x11\x15a\x10\xE3Wa\x10\xE3a\x10\xBAV[`\0` \x82\x84\x03\x12\x15a\x113Wa\x113a\x10EV[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x11JW`\0\x80\xFD[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x11fWa\x11fa\x10EV[PQ\x91\x90PV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a\x11\x93W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\x11wV[P`\0` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[`@\x81R`\0a\x11\xC6`@\x83\x01\x85a\x11mV[\x90P\x82` \x83\x01R\x93\x92PPPV[`\x80\x81R`\0a\x11\xE8`\x80\x83\x01\x87a\x11mV[` \x83\x01\x95\x90\x95RP`@\x81\x01\x92\x90\x92R``\x90\x91\x01R\x91\x90PV\xFETarget contract does not containProfitFinder[FOUND] (i,trade,profit):";
    /// The bytecode of the contract.
    pub static PROFITFINDER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\0\x8EW`\x005`\xE0\x1C\x80c\x0F\xF5s\x1D\x14a\0\xF3W\x80c5(\x93\0\x14a\x01 W\x80c\x91\xE6\xECB\x14a\x013W[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x01\x06a\x01\x016`\x04a\x10\x95V[a\x01^V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01[`@Q\x80\x91\x03\x90\xF3[a\x01\x06a\x01.6`\x04a\x10\x95V[a\x08`V[`\0Ta\x01F\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\x17V[`\0\x80a\x03\xE8\x83\x10a\x01\xB7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7Fepsilon must be less than 1000\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[P`\x01`\xFF\x1B\x90P`\x01\x80g\r\xE0\xB6\xB3\xA7d\0\0`\0\x80a\x01\xD8\x84\x84a\x10\xD0V[\x90P`\0\x80[`\x03a\x01\xEA\x87\x87a\x10\xD0V[a\x01\xF4\x91\x90a\x10\xE9V[a\x01\xFE\x90\x87a\x11\x0BV[\x91P`\x03a\x02\x0C\x87\x87a\x10\xD0V[a\x02\x16\x91\x90a\x10\xE9V[a\x02 \x90\x86a\x10\xD0V[\x90P`\0\x80T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x99\x9B\x93\xAF`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x02\xACW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x12\x05\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x02\xC0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\xE4\x91\x90a\x11\x1EV[`\x01`\x01`\xA0\x1B\x03\x16c@\xC1\x0F\x190a\x02\xFD\x84\x86a\x11\x0BV[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x04\x83\x01R`$\x82\x01R`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x03\x81W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x12\x05\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x03\x95W=`\0\x80>=`\0\xFD[PPPP`\0\x80T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x99\x9B\x93\xAF`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x04#W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x12\x05\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x047W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04[\x91\x90a\x11\x1EV[`\0T`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x91c\t^\xA7\xB3\x91\x16a\x04{\x84\x86a\x11\x0BV[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x04\x83\x01R`$\x82\x01R`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x04\xFFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x12\x05\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x05\x13W=`\0\x80>=`\0\xFD[PP`\0\x80T`@Qc\xD7\x82=\xF7`\xE0\x1B\x81R`\x04\x81\x01\x87\x90R\x91\x93P\x83\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\xD7\x82=\xF7\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x05\x9EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x12\x05\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x92PPP\x80\x15a\x05\xAFWP`\x01[a\x06\x05W=\x80\x80\x15a\x05\xDDW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x05\xE2V[``\x91P[P`\x04\x81\x01\x90P\x80\x80` \x01\x90Q\x81\x01\x90a\x05\xFD\x91\x90a\x11QV[\x92PPa\x06\rV[`\x01`\xFF\x1B\x91P[`\0T`@Qc\xD7\x82=\xF7`\xE0\x1B\x81R`\x04\x81\x01\x85\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xD7\x82=\xF7\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x06\x91W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x12\x05\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x92PPP\x80\x15a\x06\xA2WP`\x01[a\x06\xF8W=\x80\x80\x15a\x06\xD0W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x06\xD5V[``\x91P[P`\x04\x81\x01\x90P\x80\x80` \x01\x90Q\x81\x01\x90a\x06\xF0\x91\x90a\x11QV[\x91PPa\x06\xFFV[P`\x01`\xFF\x1B[a\x07'`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01eTrade1`\xD0\x1B\x81RP\x85a\x0F;V[a\x07P`@Q\x80`@\x01`@R\x80`\x07\x81R` \x01fProfit1`\xC8\x1B\x81RP\x83a\x0F\x84V[a\x07x`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01e*90\xB22\x99`\xD1\x1B\x81RP\x84a\x0F;V[a\x07\xA1`@Q\x80`@\x01`@R\x80`\x07\x81R` \x01f(97\xB34\xBA\x19`\xC9\x1B\x81RP\x82a\x0F\x84V[a\x07\xCC`@Q\x80`@\x01`@R\x80`\t\x81R` \x01h\x13X^\x14\x1C\x9B\xD9\x9A]`\xBA\x1B\x81RP\x8Ba\x0F\x84V[\x80\x82\x12\x15a\x07\xE2W\x83\x97P\x80\x99P\x82\x98Pa\x07\xECV[\x82\x96P\x81\x99P\x83\x98P[a\x07\xF6\x88\x88a\x10\xD0V[\x94Pa\x08\x03`\x01\x87a\x11\x0BV[\x95PPP\x88\x83\x11\x80\x15a\x08\x15WP\x89\x84\x10[a\x01\xDEWa\x08=`@Q\x80``\x01`@R\x80`%\x81R` \x01a\x12%`%\x919\x85\x89\x8Ba\x0F\xC9V[`@Qc*6\x9F#`\xE0\x1B\x81R`\x04\x81\x01\x88\x90R`$\x81\x01\x89\x90R`D\x01a\x01\xAEV[`\0\x80a\x03\xE8\x83\x10a\x08\xB4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7Fepsilon must be less than 1000\0\0`D\x82\x01R`d\x01a\x01\xAEV[P`\x01`\xFF\x1B\x90P`\x01\x80h\x05k\xC7^-c\x10\0\0`\0\x80a\x08\xD6\x84\x84a\x10\xD0V[\x90P`\0\x80[`\x03a\x08\xE8\x87\x87a\x10\xD0V[a\x08\xF2\x91\x90a\x10\xE9V[a\x08\xFC\x90\x87a\x11\x0BV[\x91P`\x03a\t\n\x87\x87a\x10\xD0V[a\t\x14\x91\x90a\x10\xE9V[a\t\x1E\x90\x86a\x10\xD0V[\x90P`\0\x80T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x99\x9B\x93\xAF`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\t\xAAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x12\x05\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\t\xBEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\xE2\x91\x90a\x11\x1EV[`\x01`\x01`\xA0\x1B\x03\x16c@\xC1\x0F\x190a\t\xFB\x84\x86a\x11\x0BV[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x04\x83\x01R`$\x82\x01R`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\n\x7FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x12\x05\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\n\x93W=`\0\x80>=`\0\xFD[PPPP`\0\x80T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x99\x9B\x93\xAF`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0B!W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x12\x05\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0B5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0BY\x91\x90a\x11\x1EV[`\0T`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x91c\t^\xA7\xB3\x91\x16a\x0By\x84\x86a\x11\x0BV[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x04\x83\x01R`$\x82\x01R`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0B\xFDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x12\x05\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x0C\x11W=`\0\x80>=`\0\xFD[PP`\0\x80T`@Qc\x1D\xF4i\x8F`\xE3\x1B\x81R`\x04\x81\x01\x87\x90R\x91\x93P\x83\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\xEF\xA3Lx\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0C\x9CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x12\x05\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x92PPP\x80\x15a\x0C\xADWP`\x01[a\r\x03W=\x80\x80\x15a\x0C\xDBW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x0C\xE0V[``\x91P[P`\x04\x81\x01\x90P\x80\x80` \x01\x90Q\x81\x01\x90a\x0C\xFB\x91\x90a\x11QV[\x92PPa\r\x0BV[`\x01`\xFF\x1B\x91P[`\0T`@Qc\x1D\xF4i\x8F`\xE3\x1B\x81R`\x04\x81\x01\x85\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xEF\xA3Lx\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\r\x8FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x12\x05\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x92PPP\x80\x15a\r\xA0WP`\x01[a\r\xF6W=\x80\x80\x15a\r\xCEW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\r\xD3V[``\x91P[P`\x04\x81\x01\x90P\x80\x80` \x01\x90Q\x81\x01\x90a\r\xEE\x91\x90a\x11QV[\x91PPa\r\xFDV[P`\x01`\xFF\x1B[a\x0E%`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01eTrade1`\xD0\x1B\x81RP\x85a\x0F;V[a\x0EN`@Q\x80`@\x01`@R\x80`\x07\x81R` \x01fProfit1`\xC8\x1B\x81RP\x83a\x0F\x84V[a\x0Ev`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01e*90\xB22\x99`\xD1\x1B\x81RP\x84a\x0F;V[a\x0E\x9F`@Q\x80`@\x01`@R\x80`\x07\x81R` \x01f(97\xB34\xBA\x19`\xC9\x1B\x81RP\x82a\x0F\x84V[a\x0E\xCA`@Q\x80`@\x01`@R\x80`\t\x81R` \x01h\x13X^\x14\x1C\x9B\xD9\x9A]`\xBA\x1B\x81RP\x8Ba\x0F\x84V[\x80\x82\x12\x15a\x0E\xE0W\x83\x97P\x80\x99P\x82\x98Pa\x0E\xEAV[\x82\x96P\x81\x99P\x83\x98P[a\x0E\xF4\x88\x88a\x10\xD0V[\x94Pa\x0F\x01`\x01\x87a\x11\x0BV[\x95PPP\x88\x83\x11\x80\x15a\x0F\x13WP\x89\x84\x10[a\x08\xDCWa\x08=`@Q\x80``\x01`@R\x80`%\x81R` \x01a\x12%`%\x919\x85\x89\x8Ba\x0F\xC9V[a\x0F\x80\x82\x82`@Q`$\x01a\x0FQ\x92\x91\x90a\x11\xB3V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c-\x83\x9C\xB3`\xE2\x1B\x17\x90Ra\x10\x18V[PPV[a\x0F\x80\x82\x82`@Q`$\x01a\x0F\x9A\x92\x91\x90a\x11\xB3V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\x1ES\x13G`\xE1\x1B\x17\x90Ra\x10\x18V[a\x10\x12\x84\x84\x84\x84`@Q`$\x01a\x0F\xE3\x94\x93\x92\x91\x90a\x11\xD5V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\xA7\xA8xS`\xE0\x1B\x17\x90Ra\x10\x18V[PPPPV[a\x10!\x81a\x10$V[PV[\x80Qjconsole.log` \x83\x01`\0\x80\x84\x83\x85Z\xFAPPPPPV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x10\xABWa\x10\xABa\x10EV[PP\x805\x92` \x90\x91\x015\x91PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x10\xE3Wa\x10\xE3a\x10\xBAV[\x92\x91PPV[`\0\x82a\x11\x06WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[\x80\x82\x01\x80\x82\x11\x15a\x10\xE3Wa\x10\xE3a\x10\xBAV[`\0` \x82\x84\x03\x12\x15a\x113Wa\x113a\x10EV[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x11JW`\0\x80\xFD[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x11fWa\x11fa\x10EV[PQ\x91\x90PV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a\x11\x93W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\x11wV[P`\0` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[`@\x81R`\0a\x11\xC6`@\x83\x01\x85a\x11mV[\x90P\x82` \x83\x01R\x93\x92PPPV[`\x80\x81R`\0a\x11\xE8`\x80\x83\x01\x87a\x11mV[` \x83\x01\x95\x90\x95RP`@\x81\x01\x92\x90\x92R``\x90\x91\x01R\x91\x90PV\xFETarget contract does not containProfitFinder[FOUND] (i,trade,profit):";
    /// The deployed bytecode of the contract.
    pub static PROFITFINDER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct ProfitFinder<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ProfitFinder<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ProfitFinder<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ProfitFinder<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ProfitFinder<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(ProfitFinder))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ProfitFinder<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    PROFITFINDER_ABI.clone(),
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
                PROFITFINDER_ABI.clone(),
                PROFITFINDER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `atomic` (0x91e6ec42) function
        pub fn atomic(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([145, 230, 236, 66], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `searchLowerPrice` (0x35289300) function
        pub fn search_lower_price(
            &self,
            max_iters: ::ethers::core::types::U256,
            epsilon: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::I256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([53, 40, 147, 0], (max_iters, epsilon))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `searchRaisePrice` (0x0ff5731d) function
        pub fn search_raise_price(
            &self,
            max_iters: ::ethers::core::types::U256,
            epsilon: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::I256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([15, 245, 115, 29], (max_iters, epsilon))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for ProfitFinder<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `MaximizingProfitTrade` with signature `MaximizingProfitTrade(uint256,uint256)` and selector `0x2a369f23`
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
    #[etherror(
        name = "MaximizingProfitTrade",
        abi = "MaximizingProfitTrade(uint256,uint256)"
    )]
    pub struct MaximizingProfitTrade {
        pub trade: ::ethers::core::types::U256,
        pub profit: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `atomic` function with signature `atomic()` and selector `0x91e6ec42`
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
    #[ethcall(name = "atomic", abi = "atomic()")]
    pub struct AtomicCall;
    ///Container type for all input parameters for the `searchLowerPrice` function with signature `searchLowerPrice(uint256,uint256)` and selector `0x35289300`
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
    #[ethcall(name = "searchLowerPrice", abi = "searchLowerPrice(uint256,uint256)")]
    pub struct SearchLowerPriceCall {
        pub max_iters: ::ethers::core::types::U256,
        pub epsilon: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `searchRaisePrice` function with signature `searchRaisePrice(uint256,uint256)` and selector `0x0ff5731d`
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
    #[ethcall(name = "searchRaisePrice", abi = "searchRaisePrice(uint256,uint256)")]
    pub struct SearchRaisePriceCall {
        pub max_iters: ::ethers::core::types::U256,
        pub epsilon: ::ethers::core::types::U256,
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
    pub enum ProfitFinderCalls {
        Atomic(AtomicCall),
        SearchLowerPrice(SearchLowerPriceCall),
        SearchRaisePrice(SearchRaisePriceCall),
    }
    impl ::ethers::core::abi::AbiDecode for ProfitFinderCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AtomicCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Atomic(decoded));
            }
            if let Ok(decoded) = <SearchLowerPriceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SearchLowerPrice(decoded));
            }
            if let Ok(decoded) = <SearchRaisePriceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SearchRaisePrice(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ProfitFinderCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Atomic(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SearchLowerPrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SearchRaisePrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for ProfitFinderCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Atomic(element) => ::core::fmt::Display::fmt(element, f),
                Self::SearchLowerPrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::SearchRaisePrice(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AtomicCall> for ProfitFinderCalls {
        fn from(value: AtomicCall) -> Self {
            Self::Atomic(value)
        }
    }
    impl ::core::convert::From<SearchLowerPriceCall> for ProfitFinderCalls {
        fn from(value: SearchLowerPriceCall) -> Self {
            Self::SearchLowerPrice(value)
        }
    }
    impl ::core::convert::From<SearchRaisePriceCall> for ProfitFinderCalls {
        fn from(value: SearchRaisePriceCall) -> Self {
            Self::SearchRaisePrice(value)
        }
    }
    ///Container type for all return fields from the `atomic` function with signature `atomic()` and selector `0x91e6ec42`
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
    pub struct AtomicReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `searchLowerPrice` function with signature `searchLowerPrice(uint256,uint256)` and selector `0x35289300`
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
    pub struct SearchLowerPriceReturn {
        pub max_profit: ::ethers::core::types::I256,
        pub max_trade: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `searchRaisePrice` function with signature `searchRaisePrice(uint256,uint256)` and selector `0x0ff5731d`
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
    pub struct SearchRaisePriceReturn {
        pub max_profit: ::ethers::core::types::I256,
        pub max_trade: ::ethers::core::types::U256,
    }
}
