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
    const __BYTECODE: &[u8] = b"`\x80`@\x90\x80\x82R4a\x01#WP\x80Q`\x1Fa\x0EG8\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x91`\x01`\x01`@\x1B\x03\x83\x11\x84\x84\x10\x17a\x01\rW\x80\x84\x92`\x80\x94\x87R\x839\x81\x01\x03\x12a\0\xBEWa\0O\x81a\x01pV[\x90a\0\\` \x82\x01a\x01pV[a\0s``a\0l\x86\x85\x01a\x01pV[\x93\x01a\x01pV[\x90`\x01\x80`\xA0\x1B\x03\x92\x83\x80\x92\x81`\x01\x80`\xA0\x1B\x03\x19\x97\x16\x87`\0T\x16\x17`\0U\x16\x85`\x01T\x16\x17`\x01U\x16\x83`\x02T\x16\x17`\x02U\x16\x90`\x03T\x16\x17`\x03UQa\x0C\xBD\x90\x81a\x01\x8A\x829\xF3[\x81QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[bF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01Ra7\xB7`\xF1\x1B`d\x82\x01R`\x84\x90\xFD[Q\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x01\x84WV[`\0\x80\xFD\xFE`@`\x80\x81R`\x04\x90\x816\x10\x15a\0\x94W[\x90` `\x84\x92Q\x91\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01R\x7Fnor receive functions\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R\xFD[`\0\x805`\xE0\x1C\x80c5\xA9\x9A\xD0\x14a\x06\xC9W\x80c8\xD5.\x0F\x14a\x06xW\x80c\x8A/\xA5J\x14a\x01\xDEW\x80c\x99\x9B\x93\xAF\x14a\x01\x8DW\x80c\x9F'\xEFO\x14a\x01<Wc\xD2\xF7&Z\x14a\0\xE2WPa\0\x11V[\x904a\x017W\x81\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x012Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x92T\x16\x90Q\x90\x81R\xF3[a\x0B1V[a\n\xADV[P4a\x017W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x012W` \x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x01T\x16\x90Q\x90\x81R\xF3[P4a\x017W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x012W` \x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x03T\x16\x90Q\x90\x81R\xF3[P\x914a\x017W` \x91\x82\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x012W\x815\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x83`\x03T\x16\x80;\x15a\x05?W\x83Q\x7F#\xB8r\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R3\x84\x82\x01\x90\x81R0` \x82\x01R`@\x81\x01\x84\x90R\x88\x92\x83\x91\x83\x91\x90\x82\x90\x84\x90\x82\x90``\x01\x03\x92Z\xF1\x80\x15a\x06nWa\x06ZW[PP\x83`\x03T\x16\x90\x84\x87T\x16\x91\x80;\x15a\x05?W\x87\x90\x81\x86Q\x80\x92\x81\x83\x81a\x03\0\x89\x7F\t^\xA7\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x9B\x8C\x84R\x8D\x84\x01` \x90\x93\x92\x91\x93s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@\x82\x01\x95\x16\x81R\x01RV[\x03\x92Z\xF1\x80\x15a\x05\xC6Wa\x06FW[PT\x85\x16\x90\x81;\x15a\x05?W\x87\x91`D\x88\x92\x87Q\x94\x85\x93\x84\x92\x7F\x1F\xDA\xBC'\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R\x82\x8A\x85\x01R`$\x84\x01RZ\xF1\x80\x15a\x05\xA8W\x90\x86\x91a\x06\x1DW[PP\x83`\x02T\x16\x93\x84;\x15a\x05?W\x83Q\x91\x7Fp\xA0\x821\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x95\x86\x84R0\x85\x85\x01R\x87\x84`$\x81\x84Z\xFA\x93\x84\x15a\x06\x13W\x89\x94a\x05\xE4W[P\x82`\x01T\x16\x81;\x15a\x05?W\x86Q\x92\x83Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x85\x83\x01\x90\x81R` \x81\x01\x85\x90R\x89\x92\x91\x83\x91\x83\x91\x90\x82\x90\x84\x90\x82\x90`@\x01\x03\x92Z\xF1\x80\x15a\x05\xC6Wa\x05\xD0W[PP\x80`\x01T\x16\x81`\x02T\x16\x81;\x15a\x05?W\x88\x84\x81\x93a\x04~\x93\x83\x8AQ\x80\x96\x81\x95\x82\x94\x7F\xD0\x04\xF0\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R\x8D\x84\x01` \x90\x93\x92\x91\x93s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@\x82\x01\x95\x16\x81R\x01RV[\x03\x92Z\xF1\x80\x15a\x05\xC6Wa\x05\xB2W[PP`\x03T\x16\x93\x84;\x15a\x05?W\x83Q\x90\x81R0\x83\x82\x01R\x85\x81`$\x81\x88Z\xFA\x95\x86\x15a\x05\xA8W\x87\x96a\x05xW[PP\x84\x15a\x05DWP\x82;\x15a\x05?W\x81Q\x7F\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R3\x91\x81\x01\x91\x82R` \x82\x01\x94\x90\x94R\x84\x93\x92\x84\x91\x84\x91\x90\x82\x90\x84\x90\x82\x90`@\x01[\x03\x92Z\xF1\x90\x81\x15a\x056WPa\x05&WPP\x80\xF3[a\x05/\x90a\x0C9V[a\x012W\x80\xF3[Q=\x84\x82>=\x90\xFD[a\x0B\xB5V[\x84\x90`D\x93Q\x92\x7F\x84>0\xEC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R\x83\x01R`$\x82\x01R\xFD[\x90\x80\x92\x96P\x81=\x83\x11a\x05\xA1W[a\x05\x90\x81\x83a\x0C|V[\x81\x01\x03\x12a\x012WQ\x938\x80a\x04\xBBV[P=a\x05\x86V[\x84Q=\x89\x82>=\x90\xFD[a\x05\xBB\x90a\x0C9V[a\x012W\x868a\x04\x8DV[\x86Q=\x84\x82>=\x90\xFD[a\x05\xD9\x90a\x0C9V[a\x012W\x868a\x04\x0CV[\x90\x93P\x87\x81\x81=\x83\x11a\x06\x0CW[a\x05\xFC\x81\x83a\x0C|V[\x81\x01\x03\x12a\x012WQ\x928a\x03\xB7V[P=a\x05\xF2V[\x86Q=\x8B\x82>=\x90\xFD[\x81=\x83\x11a\x06?W[a\x060\x81\x83a\x0C|V[\x81\x01\x03\x12a\x012W\x848a\x03eV[P=a\x06&V[a\x06O\x90a\x0C9V[a\x012W\x868a\x03\x0FV[a\x06c\x90a\x0C9V[a\x012W\x858a\x02\x8EV[\x85Q=\x84\x82>=\x90\xFD[P4a\x017W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x012W` \x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x02T\x16\x90Q\x90\x81R\xF3[P\x914a\n\xADW` \x91\x82\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x012W\x815\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x83`\x03T\x16\x80;\x15a\x05?W\x83Q\x7F#\xB8r\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R3\x84\x82\x01\x90\x81R0` \x82\x01R`@\x81\x01\x84\x90R\x90\x91\x88\x91\x83\x91\x90\x82\x90\x84\x90\x82\x90``\x01\x03\x92Z\xF1\x80\x15a\x05\xA8Wa\n\x9AW[P\x83`\x03T\x16\x90\x84`\x01T\x16\x91\x80;\x15a\x05?W\x87\x90\x81\x86Q\x80\x92\x81\x83\x81a\x07\xEB\x89\x7F\t^\xA7\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x9B\x8C\x84R\x8D\x84\x01` \x90\x93\x92\x91\x93s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@\x82\x01\x95\x16\x81R\x01RV[\x03\x92Z\xF1\x80\x15a\x05\xC6Wa\n\x86W[PP\x84`\x01T\x16\x85`\x03T\x16\x81;\x15a\x05?W\x88\x92a\x08l\x92\x84\x92\x83\x89Q\x80\x96\x81\x95\x82\x94\x7F\xD0\x04\xF0\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R\x8C\x84\x01` \x90\x93\x92\x91\x93s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@\x82\x01\x95\x16\x81R\x01RV[\x03\x92Z\xF1\x80\x15a\x06nWa\nrW[PP\x83`\x02T\x16\x93\x84;\x15a\x05?W\x83Q\x91\x7Fp\xA0\x821\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x95\x86\x84R0\x85\x85\x01R\x87\x84`$\x81\x84Z\xFA\x93\x84\x15a\x06\x13W\x89\x94a\nCW[P\x82\x89T\x16\x81;\x15a\x05?W\x86Q\x92\x83Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x85\x83\x01\x90\x81R` \x81\x01\x85\x90R\x89\x92\x91\x83\x91\x83\x91\x90\x82\x90\x84\x90\x82\x90`@\x01\x03\x92Z\xF1\x80\x15a\x05\xC6Wa\n/W[PT\x81\x16\x80;\x15a\x05?W\x86\x88\x91`D\x87Q\x80\x94\x81\x93\x7F\x1F\xDA\xBC'\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x01\x8A\x84\x01R\x88`$\x84\x01RZ\xF1\x80\x15a\n%W\x90\x87\x91a\t\xFCWPP`\x03T\x16\x93\x84;\x15a\x05?W\x83Q\x90\x81R0\x83\x82\x01R\x85\x81`$\x81\x88Z\xFA\x95\x86\x15a\x05\xA8W\x87\x96a\x05xWPP\x84\x15a\x05DWP\x82;\x15a\x05?W\x81Q\x7F\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R3\x91\x81\x01\x91\x82R` \x82\x01\x94\x90\x94R\x84\x93\x92\x84\x91\x84\x91\x90\x82\x90\x84\x90\x82\x90`@\x01a\x05\x11V[\x81=\x83\x11a\n\x1EW[a\n\x0F\x81\x83a\x0C|V[\x81\x01\x03\x12a\x012W\x858a\x04\x8DV[P=a\n\x05V[\x85Q=\x8A\x82>=\x90\xFD[a\n8\x90a\x0C9V[a\x012W\x868a\t!V[\x90\x93P\x87\x81\x81=\x83\x11a\nkW[a\n[\x81\x83a\x0C|V[\x81\x01\x03\x12a\x012WQ\x928a\x08\xCDV[P=a\nQV[a\n{\x90a\x0C9V[a\x012W\x858a\x08{V[a\n\x8F\x90a\x0C9V[a\x012W\x868a\x07\xFAV[a\n\xA6\x90\x96\x91\x96a\x0C9V[\x948a\x07yV[`\x84`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R\xFD[`\x84`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01R\x7Frt\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R\xFD[`\x84`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x0CMW`@RV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[\x90`\x1F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x0CMW`@RV";
    /// The bytecode of the contract.
    pub static ATOMICARBITRAGE_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`@`\x80\x81R`\x04\x90\x816\x10\x15a\0\x94W[\x90` `\x84\x92Q\x91\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01R\x7Fnor receive functions\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R\xFD[`\0\x805`\xE0\x1C\x80c5\xA9\x9A\xD0\x14a\x06\xC9W\x80c8\xD5.\x0F\x14a\x06xW\x80c\x8A/\xA5J\x14a\x01\xDEW\x80c\x99\x9B\x93\xAF\x14a\x01\x8DW\x80c\x9F'\xEFO\x14a\x01<Wc\xD2\xF7&Z\x14a\0\xE2WPa\0\x11V[\x904a\x017W\x81\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x012Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x92T\x16\x90Q\x90\x81R\xF3[a\x0B1V[a\n\xADV[P4a\x017W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x012W` \x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x01T\x16\x90Q\x90\x81R\xF3[P4a\x017W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x012W` \x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x03T\x16\x90Q\x90\x81R\xF3[P\x914a\x017W` \x91\x82\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x012W\x815\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x83`\x03T\x16\x80;\x15a\x05?W\x83Q\x7F#\xB8r\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R3\x84\x82\x01\x90\x81R0` \x82\x01R`@\x81\x01\x84\x90R\x88\x92\x83\x91\x83\x91\x90\x82\x90\x84\x90\x82\x90``\x01\x03\x92Z\xF1\x80\x15a\x06nWa\x06ZW[PP\x83`\x03T\x16\x90\x84\x87T\x16\x91\x80;\x15a\x05?W\x87\x90\x81\x86Q\x80\x92\x81\x83\x81a\x03\0\x89\x7F\t^\xA7\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x9B\x8C\x84R\x8D\x84\x01` \x90\x93\x92\x91\x93s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@\x82\x01\x95\x16\x81R\x01RV[\x03\x92Z\xF1\x80\x15a\x05\xC6Wa\x06FW[PT\x85\x16\x90\x81;\x15a\x05?W\x87\x91`D\x88\x92\x87Q\x94\x85\x93\x84\x92\x7F\x1F\xDA\xBC'\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R\x82\x8A\x85\x01R`$\x84\x01RZ\xF1\x80\x15a\x05\xA8W\x90\x86\x91a\x06\x1DW[PP\x83`\x02T\x16\x93\x84;\x15a\x05?W\x83Q\x91\x7Fp\xA0\x821\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x95\x86\x84R0\x85\x85\x01R\x87\x84`$\x81\x84Z\xFA\x93\x84\x15a\x06\x13W\x89\x94a\x05\xE4W[P\x82`\x01T\x16\x81;\x15a\x05?W\x86Q\x92\x83Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x85\x83\x01\x90\x81R` \x81\x01\x85\x90R\x89\x92\x91\x83\x91\x83\x91\x90\x82\x90\x84\x90\x82\x90`@\x01\x03\x92Z\xF1\x80\x15a\x05\xC6Wa\x05\xD0W[PP\x80`\x01T\x16\x81`\x02T\x16\x81;\x15a\x05?W\x88\x84\x81\x93a\x04~\x93\x83\x8AQ\x80\x96\x81\x95\x82\x94\x7F\xD0\x04\xF0\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R\x8D\x84\x01` \x90\x93\x92\x91\x93s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@\x82\x01\x95\x16\x81R\x01RV[\x03\x92Z\xF1\x80\x15a\x05\xC6Wa\x05\xB2W[PP`\x03T\x16\x93\x84;\x15a\x05?W\x83Q\x90\x81R0\x83\x82\x01R\x85\x81`$\x81\x88Z\xFA\x95\x86\x15a\x05\xA8W\x87\x96a\x05xW[PP\x84\x15a\x05DWP\x82;\x15a\x05?W\x81Q\x7F\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R3\x91\x81\x01\x91\x82R` \x82\x01\x94\x90\x94R\x84\x93\x92\x84\x91\x84\x91\x90\x82\x90\x84\x90\x82\x90`@\x01[\x03\x92Z\xF1\x90\x81\x15a\x056WPa\x05&WPP\x80\xF3[a\x05/\x90a\x0C9V[a\x012W\x80\xF3[Q=\x84\x82>=\x90\xFD[a\x0B\xB5V[\x84\x90`D\x93Q\x92\x7F\x84>0\xEC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R\x83\x01R`$\x82\x01R\xFD[\x90\x80\x92\x96P\x81=\x83\x11a\x05\xA1W[a\x05\x90\x81\x83a\x0C|V[\x81\x01\x03\x12a\x012WQ\x938\x80a\x04\xBBV[P=a\x05\x86V[\x84Q=\x89\x82>=\x90\xFD[a\x05\xBB\x90a\x0C9V[a\x012W\x868a\x04\x8DV[\x86Q=\x84\x82>=\x90\xFD[a\x05\xD9\x90a\x0C9V[a\x012W\x868a\x04\x0CV[\x90\x93P\x87\x81\x81=\x83\x11a\x06\x0CW[a\x05\xFC\x81\x83a\x0C|V[\x81\x01\x03\x12a\x012WQ\x928a\x03\xB7V[P=a\x05\xF2V[\x86Q=\x8B\x82>=\x90\xFD[\x81=\x83\x11a\x06?W[a\x060\x81\x83a\x0C|V[\x81\x01\x03\x12a\x012W\x848a\x03eV[P=a\x06&V[a\x06O\x90a\x0C9V[a\x012W\x868a\x03\x0FV[a\x06c\x90a\x0C9V[a\x012W\x858a\x02\x8EV[\x85Q=\x84\x82>=\x90\xFD[P4a\x017W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x012W` \x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x02T\x16\x90Q\x90\x81R\xF3[P\x914a\n\xADW` \x91\x82\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x012W\x815\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x83`\x03T\x16\x80;\x15a\x05?W\x83Q\x7F#\xB8r\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R3\x84\x82\x01\x90\x81R0` \x82\x01R`@\x81\x01\x84\x90R\x90\x91\x88\x91\x83\x91\x90\x82\x90\x84\x90\x82\x90``\x01\x03\x92Z\xF1\x80\x15a\x05\xA8Wa\n\x9AW[P\x83`\x03T\x16\x90\x84`\x01T\x16\x91\x80;\x15a\x05?W\x87\x90\x81\x86Q\x80\x92\x81\x83\x81a\x07\xEB\x89\x7F\t^\xA7\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x9B\x8C\x84R\x8D\x84\x01` \x90\x93\x92\x91\x93s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@\x82\x01\x95\x16\x81R\x01RV[\x03\x92Z\xF1\x80\x15a\x05\xC6Wa\n\x86W[PP\x84`\x01T\x16\x85`\x03T\x16\x81;\x15a\x05?W\x88\x92a\x08l\x92\x84\x92\x83\x89Q\x80\x96\x81\x95\x82\x94\x7F\xD0\x04\xF0\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R\x8C\x84\x01` \x90\x93\x92\x91\x93s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@\x82\x01\x95\x16\x81R\x01RV[\x03\x92Z\xF1\x80\x15a\x06nWa\nrW[PP\x83`\x02T\x16\x93\x84;\x15a\x05?W\x83Q\x91\x7Fp\xA0\x821\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x95\x86\x84R0\x85\x85\x01R\x87\x84`$\x81\x84Z\xFA\x93\x84\x15a\x06\x13W\x89\x94a\nCW[P\x82\x89T\x16\x81;\x15a\x05?W\x86Q\x92\x83Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x85\x83\x01\x90\x81R` \x81\x01\x85\x90R\x89\x92\x91\x83\x91\x83\x91\x90\x82\x90\x84\x90\x82\x90`@\x01\x03\x92Z\xF1\x80\x15a\x05\xC6Wa\n/W[PT\x81\x16\x80;\x15a\x05?W\x86\x88\x91`D\x87Q\x80\x94\x81\x93\x7F\x1F\xDA\xBC'\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x01\x8A\x84\x01R\x88`$\x84\x01RZ\xF1\x80\x15a\n%W\x90\x87\x91a\t\xFCWPP`\x03T\x16\x93\x84;\x15a\x05?W\x83Q\x90\x81R0\x83\x82\x01R\x85\x81`$\x81\x88Z\xFA\x95\x86\x15a\x05\xA8W\x87\x96a\x05xWPP\x84\x15a\x05DWP\x82;\x15a\x05?W\x81Q\x7F\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R3\x91\x81\x01\x91\x82R` \x82\x01\x94\x90\x94R\x84\x93\x92\x84\x91\x84\x91\x90\x82\x90\x84\x90\x82\x90`@\x01a\x05\x11V[\x81=\x83\x11a\n\x1EW[a\n\x0F\x81\x83a\x0C|V[\x81\x01\x03\x12a\x012W\x858a\x04\x8DV[P=a\n\x05V[\x85Q=\x8A\x82>=\x90\xFD[a\n8\x90a\x0C9V[a\x012W\x868a\t!V[\x90\x93P\x87\x81\x81=\x83\x11a\nkW[a\n[\x81\x83a\x0C|V[\x81\x01\x03\x12a\x012WQ\x928a\x08\xCDV[P=a\nQV[a\n{\x90a\x0C9V[a\x012W\x858a\x08{V[a\n\x8F\x90a\x0C9V[a\x012W\x868a\x07\xFAV[a\n\xA6\x90\x96\x91\x96a\x0C9V[\x948a\x07yV[`\x84`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R\xFD[`\x84`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01R\x7Frt\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R\xFD[`\x84`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x0CMW`@RV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[\x90`\x1F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x0CMW`@RV";
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
