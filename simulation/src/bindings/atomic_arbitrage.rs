pub use atomic_arbitrage::*;
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
pub mod atomic_arbitrage {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("exchangeAddress"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("liquidExchangeAddress"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("assetAddress"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("quoteAddress"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("asset"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("asset"),
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
                    ::std::borrow::ToOwned::to_owned("exchange"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("exchange"),
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
                    ::std::borrow::ToOwned::to_owned("liquidExchange"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("liquidExchange"),
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
                    ::std::borrow::ToOwned::to_owned("lower_exchange_price"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "lower_exchange_price",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("input"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
                    ::std::borrow::ToOwned::to_owned("quote"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("quote"),
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
                    ::std::borrow::ToOwned::to_owned("raise_exchange_price"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "raise_exchange_price",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("input"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("NotProfitable"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("NotProfitable"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("first_swap_output"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "second_swap_output",
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
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static ATOMICARBITRAGE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\x08\xAB8\x03\x80a\x08\xAB\x839\x81\x01`@\x81\x90Ra\0/\x91a\0\x9EV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x95\x86\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x82\x16\x17\x90\x91U`\x01\x80T\x94\x86\x16\x94\x82\x16\x94\x90\x94\x17\x90\x93U`\x02\x80T\x92\x85\x16\x92\x84\x16\x92\x90\x92\x17\x90\x91U`\x03\x80T\x91\x90\x93\x16\x91\x16\x17\x90Ua\0\xF2V[\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0\x99W`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\0\xB4W`\0\x80\xFD[a\0\xBD\x85a\0\x82V[\x93Pa\0\xCB` \x86\x01a\0\x82V[\x92Pa\0\xD9`@\x86\x01a\0\x82V[\x91Pa\0\xE7``\x86\x01a\0\x82V[\x90P\x92\x95\x91\x94P\x92PV[a\x07\xAA\x80a\x01\x01`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0bW`\x005`\xE0\x1C\x80c5\xA9\x9A\xD0\x14a\0gW\x80c8\xD5.\x0F\x14a\0|W\x80c\x8A/\xA5J\x14a\0\xABW\x80c\x99\x9B\x93\xAF\x14a\0\xBEW\x80c\x9F'\xEFO\x14a\0\xD1W\x80c\xD2\xF7&Z\x14a\0\xE4W[`\0\x80\xFD[a\0za\0u6`\x04a\x07BV[a\0\xF7V[\0[`\x02Ta\0\x8F\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\0za\0\xB96`\x04a\x07BV[a\x04\x87V[`\x03Ta\0\x8F\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x01Ta\0\x8F\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\0Ta\0\x8F\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x03T`@Qc#\xB8r\xDD`\xE0\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x83\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90`d\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x01IW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x01]W=`\0\x80>=`\0\xFD[PP`\x03T`\x01T`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x86\x90R\x91\x16\x92Pc\t^\xA7\xB3\x91P`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x01\xB1W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x01\xC5W=`\0\x80>=`\0\xFD[PP`\x01T`\x03T`@Qc\xD0\x04\xF0\xF7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x86\x90R\x91\x16\x92Pc\xD0\x04\xF0\xF7\x91P`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x02\x19W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x02-W=`\0\x80>=`\0\xFD[PP`\x02T`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x93P`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91Pcp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02|W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\xA0\x91\x90a\x07[V[`\x02T`\0T`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x84\x90R\x92\x93P\x16\x90c\t^\xA7\xB3\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x02\xF2W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x03\x06W=`\0\x80>=`\0\xFD[PP`\0T`@Qc\x1F\xDA\xBC'`\xE0\x1B\x81R`\x01`\x04\x82\x01R`$\x81\x01\x85\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x92Pc\x1F\xDA\xBC'\x91P`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x03\\W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\x80\x91\x90a\x07[V[P`\x03T`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\xCBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xEF\x91\x90a\x07[V[\x90P\x80\x15a\x04`W`\x03T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x83\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x04CW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x04WW=`\0\x80>=`\0\xFD[PPPPPPPV[`@Qc!\x0F\x8C;`\xE2\x1B\x81R`\x04\x81\x01\x83\x90R`$\x81\x01\x82\x90R`D\x01`@Q\x80\x91\x03\x90\xFD[`\x03T`@Qc#\xB8r\xDD`\xE0\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x83\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90`d\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x04\xD9W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x04\xEDW=`\0\x80>=`\0\xFD[PP`\x03T`\0T`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x86\x90R\x91\x16\x92Pc\t^\xA7\xB3\x91P`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x05AW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x05UW=`\0\x80>=`\0\xFD[PP`\0\x80T`@Qc\x1F\xDA\xBC'`\xE0\x1B\x81R`\x04\x81\x01\x92\x90\x92R`$\x82\x01\x85\x90R`\x01`\x01`\xA0\x1B\x03\x16\x92Pc\x1F\xDA\xBC'\x91P`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x05\xABW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xCF\x91\x90a\x07[V[P`\x02T`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\x19W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06=\x91\x90a\x07[V[`\x02T`\x01T`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x84\x90R\x92\x93P\x16\x90c\t^\xA7\xB3\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x06\x8FW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x06\xA3W=`\0\x80>=`\0\xFD[PP`\x01T`\x02T`@Qc\xD0\x04\xF0\xF7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x86\x90R\x91\x16\x92Pc\xD0\x04\xF0\xF7\x91P`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x06\xF7W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x07\x0BW=`\0\x80>=`\0\xFD[PP`\x03T`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x93P`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91Pcp\xA0\x821\x90`$\x01a\x03\xAEV[`\0` \x82\x84\x03\x12\x15a\x07TW`\0\x80\xFD[P5\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x07mW`\0\x80\xFD[PQ\x91\x90PV\xFE\xA2dipfsX\"\x12 j[\xA5\t\xA8\xB2B_\x8A\x07\x9A\x9820\xD3{\x8F\xF8\xDE\x85v\x18\x86-\xE5\\\x1E\xE6\xFB\xA4\x05\rdsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static ATOMICARBITRAGE_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0bW`\x005`\xE0\x1C\x80c5\xA9\x9A\xD0\x14a\0gW\x80c8\xD5.\x0F\x14a\0|W\x80c\x8A/\xA5J\x14a\0\xABW\x80c\x99\x9B\x93\xAF\x14a\0\xBEW\x80c\x9F'\xEFO\x14a\0\xD1W\x80c\xD2\xF7&Z\x14a\0\xE4W[`\0\x80\xFD[a\0za\0u6`\x04a\x07BV[a\0\xF7V[\0[`\x02Ta\0\x8F\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\0za\0\xB96`\x04a\x07BV[a\x04\x87V[`\x03Ta\0\x8F\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x01Ta\0\x8F\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\0Ta\0\x8F\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x03T`@Qc#\xB8r\xDD`\xE0\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x83\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90`d\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x01IW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x01]W=`\0\x80>=`\0\xFD[PP`\x03T`\x01T`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x86\x90R\x91\x16\x92Pc\t^\xA7\xB3\x91P`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x01\xB1W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x01\xC5W=`\0\x80>=`\0\xFD[PP`\x01T`\x03T`@Qc\xD0\x04\xF0\xF7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x86\x90R\x91\x16\x92Pc\xD0\x04\xF0\xF7\x91P`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x02\x19W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x02-W=`\0\x80>=`\0\xFD[PP`\x02T`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x93P`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91Pcp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02|W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\xA0\x91\x90a\x07[V[`\x02T`\0T`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x84\x90R\x92\x93P\x16\x90c\t^\xA7\xB3\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x02\xF2W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x03\x06W=`\0\x80>=`\0\xFD[PP`\0T`@Qc\x1F\xDA\xBC'`\xE0\x1B\x81R`\x01`\x04\x82\x01R`$\x81\x01\x85\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x92Pc\x1F\xDA\xBC'\x91P`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x03\\W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\x80\x91\x90a\x07[V[P`\x03T`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\xCBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xEF\x91\x90a\x07[V[\x90P\x80\x15a\x04`W`\x03T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x83\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x04CW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x04WW=`\0\x80>=`\0\xFD[PPPPPPPV[`@Qc!\x0F\x8C;`\xE2\x1B\x81R`\x04\x81\x01\x83\x90R`$\x81\x01\x82\x90R`D\x01`@Q\x80\x91\x03\x90\xFD[`\x03T`@Qc#\xB8r\xDD`\xE0\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x83\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90`d\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x04\xD9W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x04\xEDW=`\0\x80>=`\0\xFD[PP`\x03T`\0T`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x86\x90R\x91\x16\x92Pc\t^\xA7\xB3\x91P`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x05AW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x05UW=`\0\x80>=`\0\xFD[PP`\0\x80T`@Qc\x1F\xDA\xBC'`\xE0\x1B\x81R`\x04\x81\x01\x92\x90\x92R`$\x82\x01\x85\x90R`\x01`\x01`\xA0\x1B\x03\x16\x92Pc\x1F\xDA\xBC'\x91P`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x05\xABW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xCF\x91\x90a\x07[V[P`\x02T`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\x19W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06=\x91\x90a\x07[V[`\x02T`\x01T`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x84\x90R\x92\x93P\x16\x90c\t^\xA7\xB3\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x06\x8FW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x06\xA3W=`\0\x80>=`\0\xFD[PP`\x01T`\x02T`@Qc\xD0\x04\xF0\xF7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x86\x90R\x91\x16\x92Pc\xD0\x04\xF0\xF7\x91P`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x06\xF7W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x07\x0BW=`\0\x80>=`\0\xFD[PP`\x03T`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x93P`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91Pcp\xA0\x821\x90`$\x01a\x03\xAEV[`\0` \x82\x84\x03\x12\x15a\x07TW`\0\x80\xFD[P5\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x07mW`\0\x80\xFD[PQ\x91\x90PV\xFE\xA2dipfsX\"\x12 j[\xA5\t\xA8\xB2B_\x8A\x07\x9A\x9820\xD3{\x8F\xF8\xDE\x85v\x18\x86-\xE5\\\x1E\xE6\xFB\xA4\x05\rdsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static ATOMICARBITRAGE_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct AtomicArbitrage<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for AtomicArbitrage<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for AtomicArbitrage<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for AtomicArbitrage<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for AtomicArbitrage<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(AtomicArbitrage))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> AtomicArbitrage<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    ATOMICARBITRAGE_ABI.clone(),
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
                ATOMICARBITRAGE_ABI.clone(),
                ATOMICARBITRAGE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `asset` (0x38d52e0f) function
        pub fn asset(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([56, 213, 46, 15], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `exchange` (0xd2f7265a) function
        pub fn exchange(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([210, 247, 38, 90], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `liquidExchange` (0x9f27ef4f) function
        pub fn liquid_exchange(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([159, 39, 239, 79], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lower_exchange_price` (0x35a99ad0) function
        pub fn lower_exchange_price(
            &self,
            input: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([53, 169, 154, 208], input)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `quote` (0x999b93af) function
        pub fn quote(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([153, 155, 147, 175], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `raise_exchange_price` (0x8a2fa54a) function
        pub fn raise_exchange_price(
            &self,
            input: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([138, 47, 165, 74], input)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for AtomicArbitrage<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `NotProfitable` with signature `NotProfitable(uint256,uint256)` and selector `0x843e30ec`
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
    #[etherror(name = "NotProfitable", abi = "NotProfitable(uint256,uint256)")]
    pub struct NotProfitable {
        pub first_swap_output: ::ethers::core::types::U256,
        pub second_swap_output: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `asset` function with signature `asset()` and selector `0x38d52e0f`
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
    #[ethcall(name = "asset", abi = "asset()")]
    pub struct AssetCall;
    ///Container type for all input parameters for the `exchange` function with signature `exchange()` and selector `0xd2f7265a`
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
    #[ethcall(name = "exchange", abi = "exchange()")]
    pub struct ExchangeCall;
    ///Container type for all input parameters for the `liquidExchange` function with signature `liquidExchange()` and selector `0x9f27ef4f`
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
    #[ethcall(name = "liquidExchange", abi = "liquidExchange()")]
    pub struct LiquidExchangeCall;
    ///Container type for all input parameters for the `lower_exchange_price` function with signature `lower_exchange_price(uint256)` and selector `0x35a99ad0`
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
    #[ethcall(name = "lower_exchange_price", abi = "lower_exchange_price(uint256)")]
    pub struct LowerExchangePriceCall {
        pub input: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `quote` function with signature `quote()` and selector `0x999b93af`
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
    #[ethcall(name = "quote", abi = "quote()")]
    pub struct QuoteCall;
    ///Container type for all input parameters for the `raise_exchange_price` function with signature `raise_exchange_price(uint256)` and selector `0x8a2fa54a`
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
    #[ethcall(name = "raise_exchange_price", abi = "raise_exchange_price(uint256)")]
    pub struct RaiseExchangePriceCall {
        pub input: ::ethers::core::types::U256,
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
    pub enum AtomicArbitrageCalls {
        Asset(AssetCall),
        Exchange(ExchangeCall),
        LiquidExchange(LiquidExchangeCall),
        LowerExchangePrice(LowerExchangePriceCall),
        Quote(QuoteCall),
        RaiseExchangePrice(RaiseExchangePriceCall),
    }
    impl ::ethers::core::abi::AbiDecode for AtomicArbitrageCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AssetCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Asset(decoded));
            }
            if let Ok(decoded) = <ExchangeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Exchange(decoded));
            }
            if let Ok(decoded) = <LiquidExchangeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LiquidExchange(decoded));
            }
            if let Ok(decoded) = <LowerExchangePriceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LowerExchangePrice(decoded));
            }
            if let Ok(decoded) = <QuoteCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Quote(decoded));
            }
            if let Ok(decoded) = <RaiseExchangePriceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RaiseExchangePrice(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for AtomicArbitrageCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Asset(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Exchange(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LiquidExchange(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LowerExchangePrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Quote(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RaiseExchangePrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for AtomicArbitrageCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Asset(element) => ::core::fmt::Display::fmt(element, f),
                Self::Exchange(element) => ::core::fmt::Display::fmt(element, f),
                Self::LiquidExchange(element) => ::core::fmt::Display::fmt(element, f),
                Self::LowerExchangePrice(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Quote(element) => ::core::fmt::Display::fmt(element, f),
                Self::RaiseExchangePrice(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<AssetCall> for AtomicArbitrageCalls {
        fn from(value: AssetCall) -> Self {
            Self::Asset(value)
        }
    }
    impl ::core::convert::From<ExchangeCall> for AtomicArbitrageCalls {
        fn from(value: ExchangeCall) -> Self {
            Self::Exchange(value)
        }
    }
    impl ::core::convert::From<LiquidExchangeCall> for AtomicArbitrageCalls {
        fn from(value: LiquidExchangeCall) -> Self {
            Self::LiquidExchange(value)
        }
    }
    impl ::core::convert::From<LowerExchangePriceCall> for AtomicArbitrageCalls {
        fn from(value: LowerExchangePriceCall) -> Self {
            Self::LowerExchangePrice(value)
        }
    }
    impl ::core::convert::From<QuoteCall> for AtomicArbitrageCalls {
        fn from(value: QuoteCall) -> Self {
            Self::Quote(value)
        }
    }
    impl ::core::convert::From<RaiseExchangePriceCall> for AtomicArbitrageCalls {
        fn from(value: RaiseExchangePriceCall) -> Self {
            Self::RaiseExchangePrice(value)
        }
    }
    ///Container type for all return fields from the `asset` function with signature `asset()` and selector `0x38d52e0f`
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
    pub struct AssetReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `exchange` function with signature `exchange()` and selector `0xd2f7265a`
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
    pub struct ExchangeReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `liquidExchange` function with signature `liquidExchange()` and selector `0x9f27ef4f`
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
    pub struct LiquidExchangeReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `quote` function with signature `quote()` and selector `0x999b93af`
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
    pub struct QuoteReturn(pub ::ethers::core::types::Address);
}
