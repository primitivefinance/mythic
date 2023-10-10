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
            constructor: ::core::option::Option::None,
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
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static ATOMICARBITRAGE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80\x80`@R4a\0\x19W`@Qa\t!\x90\x81a\0g\x829\xF3[bF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01Ra7\xB7`\xF1\x1B`d\x82\x01R`\x84\x90\xFD\xFE`@`\x80\x81R`\x04\x90\x816\x10\x15a\0rW[\x90` `\x84\x92Q\x91bF\x1B\xCD`\xE5\x1B\x83R\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01Rtnor receive functions`X\x1B`d\x82\x01R\xFD[`\0\x805`\xE0\x1C\x80c5\xA9\x9A\xD0\x14a\x05\tW\x80c8\xD5.\x0F\x14a\x04\xE2W\x80c\x8A/\xA5J\x14a\x01?W\x80c\x99\x9B\x93\xAF\x14a\x01\x18W\x80c\x9F'\xEFO\x14a\0\xF1Wc\xD2\xF7&Z\x14a\0\xC0WPa\0\x11V[\x904a\0\xECW\x81`\x03\x196\x01\x12a\0\xE7W\x90T\x90Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[a\x07\xBFV[a\x07oV[P4a\0\xECW6`\x03\x19\x01\x12a\0\xE7W`\x01T\x90Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[P4a\0\xECW6`\x03\x19\x01\x12a\0\xE7W`\x03T\x90Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[P\x914a\0\xECW` \x91\x82`\x03\x196\x01\x12a\0\xE7W`\x03T\x825\x92`\x01`\x01`\xA0\x1B\x03\x92\x90\x91\x83\x16\x80;\x15a\x03\xDEW\x81Qc#\xB8r\xDD`\xE0\x1B\x81R3\x84\x82\x01\x90\x81R0` \x82\x01R`@\x81\x01\x87\x90R\x88\x92\x83\x91\x83\x91\x90\x82\x90\x84\x90\x82\x90``\x01\x03\x92Z\xF1\x80\x15a\x04\xD8Wa\x04\xC4W[PP\x82`\x03T\x16\x92\x80\x87T\x16\x93\x80;\x15a\x03\xDEW\x82Qc\t^\xA7\xB3`\xE0\x1B\x80\x82R`\x01`\x01`\xA0\x1B\x03\x90\x96\x16\x85\x82\x01\x90\x81R` \x81\x01\x88\x90R\x89\x92\x83\x91\x83\x91\x90\x82\x90\x84\x90\x82\x90`@\x01\x03\x92Z\xF1\x80\x15a\x041Wa\x04\xB0W[PT`\x03T\x82\x16\x90\x82\x16\x80;\x15a\x03\xDEW\x83Qc\xD0\x04\xF0\xF7`\xE0\x1B\x80\x82R`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x81\x87\x01\x90\x81R` \x81\x01\x89\x90R\x8A\x92\x83\x91\x83\x91\x90\x82\x90\x84\x90\x82\x90`@\x01\x03\x92Z\xF1\x80\x15a\x04\xA6Wa\x04\x92W[PP\x81`\x02T\x16\x94\x85;\x15a\x03\xDEW\x83Q\x90cp\xA0\x821`\xE0\x1B\x96\x87\x83R0\x87\x84\x01R\x89\x83`$\x81\x84Z\xFA\x92\x83\x15a\x04\x88W\x8B\x93a\x04YW[P\x84`\x01T\x16\x91\x81;\x15a\x03\xDEW\x86Q\x90\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x82\x88\x01\x90\x81R` \x81\x01\x84\x90R\x8B\x92\x91\x83\x91\x83\x91\x90\x82\x90\x84\x90\x82\x90`@\x01\x03\x92Z\xF1\x80\x15a\x04OWa\x04;W[PP\x82`\x01T\x16\x91\x83`\x02T\x16\x91\x83;\x15a\x03\xDEW\x85Q\x91\x82R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x86\x82\x01\x90\x81R` \x81\x01\x92\x90\x92R\x89\x92\x90\x91\x83\x91\x83\x91\x82\x90\x84\x90\x82\x90`@\x01[\x03\x92Z\xF1\x80\x15a\x041Wa\x04\x1DW[PP`\x03T\x16\x92\x83;\x15a\x03\xDEW\x81Q\x90\x81R0\x83\x82\x01R\x85\x81`$\x81\x87Z\xFA\x95\x86\x15a\x04\x13W\x87\x96a\x03\xE3W[PP\x84a\x03xW[PPPa\x03u\x91\x11a\x08\xAEV[\x80\xF3[\x82;\x15a\x03\xDEW\x80Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3\x92\x81\x01\x92\x83R` \x83\x01\x86\x90R\x86\x93\x90\x92\x84\x91\x84\x91\x82\x90\x84\x90\x82\x90`@\x01\x03\x92Z\xF1\x90\x81\x15a\x03\xD5WPa\x03\xC1W[\x80a\x03hV[a\x03\xCA\x90a\x08bV[a\0\xE7W\x828a\x03\xBBV[Q=\x84\x82>=\x90\xFD[a\x08\x0FV[\x90\x80\x92\x96P\x81=\x83\x11a\x04\x0CW[a\x03\xFB\x81\x83a\x08\x8CV[\x81\x01\x03\x12a\0\xE7WQ\x938\x80a\x03`V[P=a\x03\xF1V[\x82Q=\x89\x82>=\x90\xFD[a\x04&\x90a\x08bV[a\0\xE7W\x868a\x032V[\x84Q=\x84\x82>=\x90\xFD[a\x04D\x90a\x08bV[a\0\xE7W\x888a\x02\xDEV[\x86Q=\x84\x82>=\x90\xFD[\x90\x92P\x89\x81\x81=\x83\x11a\x04\x81W[a\x04q\x81\x83a\x08\x8CV[\x81\x01\x03\x12a\0\xE7WQ\x918a\x02\x93V[P=a\x04gV[\x86Q=\x8D\x82>=\x90\xFD[a\x04\x9B\x90a\x08bV[a\0\xE7W\x878a\x02ZV[\x85Q=\x84\x82>=\x90\xFD[a\x04\xB9\x90a\x08bV[a\0\xE7W\x868a\x02\x05V[a\x04\xCD\x90a\x08bV[a\0\xE7W\x858a\x01\xADV[\x83Q=\x84\x82>=\x90\xFD[P4a\0\xECW6`\x03\x19\x01\x12a\0\xE7W`\x02T\x90Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[P\x914a\x07oW` \x91\x82`\x03\x196\x01\x12a\0\xE7W`\x03T\x825\x92`\x01`\x01`\xA0\x1B\x03\x92\x90\x91\x83\x16\x80;\x15a\x03\xDEW\x81Qc#\xB8r\xDD`\xE0\x1B\x81R3\x84\x82\x01\x90\x81R0` \x82\x01R`@\x81\x01\x87\x90R\x90\x91\x88\x91\x83\x91\x90\x82\x90\x84\x90\x82\x90``\x01\x03\x92Z\xF1\x80\x15a\x04\x13Wa\x07\\W[P\x82`\x03T\x16\x92\x80`\x01T\x16\x93\x80;\x15a\x03\xDEW\x82Qc\t^\xA7\xB3`\xE0\x1B\x80\x82R`\x01`\x01`\xA0\x1B\x03\x90\x96\x16\x85\x82\x01\x90\x81R` \x81\x01\x88\x90R\x89\x92\x83\x91\x83\x91\x90\x82\x90\x84\x90\x82\x90`@\x01\x03\x92Z\xF1\x80\x15a\x041Wa\x07HW[PP\x80`\x01T\x16\x81`\x03T\x16\x90\x80;\x15a\x03\xDEW\x83Qc\xD0\x04\xF0\xF7`\xE0\x1B\x80\x82R`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x81\x87\x01\x90\x81R` \x81\x01\x89\x90R\x8A\x92\x83\x91\x83\x91\x90\x82\x90\x84\x90\x82\x90`@\x01\x03\x92Z\xF1\x80\x15a\x04\xA6Wa\x074W[PP\x81`\x02T\x16\x94\x85;\x15a\x03\xDEW\x83Q\x90cp\xA0\x821`\xE0\x1B\x96\x87\x83R0\x87\x84\x01R\x89\x83`$\x81\x84Z\xFA\x92\x83\x15a\x04\x88W\x8B\x93a\x07\x05W[P\x84\x8BT\x16\x91\x81;\x15a\x03\xDEW\x86Q\x90\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x82\x88\x01\x90\x81R` \x81\x01\x84\x90R\x8B\x92\x91\x83\x91\x83\x91\x90\x82\x90\x84\x90\x82\x90`@\x01\x03\x92Z\xF1\x80\x15a\x04OWa\x06\xF1W[P\x83\x90T\x16\x91\x83`\x02T\x16\x91\x83;\x15a\x03\xDEW\x85Q\x91\x82R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x86\x82\x01\x90\x81R` \x81\x01\x92\x90\x92R\x89\x92\x90\x91\x83\x91\x83\x91\x82\x90\x84\x90\x82\x90`@\x01a\x03#V[a\x06\xFA\x90a\x08bV[a\0\xE7W\x888a\x06\xAAV[\x90\x92P\x89\x81\x81=\x83\x11a\x07-W[a\x07\x1D\x81\x83a\x08\x8CV[\x81\x01\x03\x12a\0\xE7WQ\x918a\x06`V[P=a\x07\x13V[a\x07=\x90a\x08bV[a\0\xE7W\x878a\x06'V[a\x07Q\x90a\x08bV[a\0\xE7W\x868a\x05\xCFV[a\x07h\x90\x96\x91\x96a\x08bV[\x948a\x05wV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01Ra7\xB7`\xF1\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01Rd code`\xD8\x1B`d\x82\x01R`\x84\x90\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x08vW`@RV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x08vW`@RV[\x15a\x08\xB5WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01RmNot profitable`\x90\x1B`D\x82\x01R`d\x90\xFD\xFE\xA2dipfsX\"\x12 \x02\xE4yCk\x9Cf\xC0]}\xF5TO\xCF\x81\x0F*\x15\x15\xBA\xE0?\xB0J\x0E%\x88\xADv\xEC\xED\xEDdsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static ATOMICARBITRAGE_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`@`\x80\x81R`\x04\x90\x816\x10\x15a\0rW[\x90` `\x84\x92Q\x91bF\x1B\xCD`\xE5\x1B\x83R\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01Rtnor receive functions`X\x1B`d\x82\x01R\xFD[`\0\x805`\xE0\x1C\x80c5\xA9\x9A\xD0\x14a\x05\tW\x80c8\xD5.\x0F\x14a\x04\xE2W\x80c\x8A/\xA5J\x14a\x01?W\x80c\x99\x9B\x93\xAF\x14a\x01\x18W\x80c\x9F'\xEFO\x14a\0\xF1Wc\xD2\xF7&Z\x14a\0\xC0WPa\0\x11V[\x904a\0\xECW\x81`\x03\x196\x01\x12a\0\xE7W\x90T\x90Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[a\x07\xBFV[a\x07oV[P4a\0\xECW6`\x03\x19\x01\x12a\0\xE7W`\x01T\x90Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[P4a\0\xECW6`\x03\x19\x01\x12a\0\xE7W`\x03T\x90Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[P\x914a\0\xECW` \x91\x82`\x03\x196\x01\x12a\0\xE7W`\x03T\x825\x92`\x01`\x01`\xA0\x1B\x03\x92\x90\x91\x83\x16\x80;\x15a\x03\xDEW\x81Qc#\xB8r\xDD`\xE0\x1B\x81R3\x84\x82\x01\x90\x81R0` \x82\x01R`@\x81\x01\x87\x90R\x88\x92\x83\x91\x83\x91\x90\x82\x90\x84\x90\x82\x90``\x01\x03\x92Z\xF1\x80\x15a\x04\xD8Wa\x04\xC4W[PP\x82`\x03T\x16\x92\x80\x87T\x16\x93\x80;\x15a\x03\xDEW\x82Qc\t^\xA7\xB3`\xE0\x1B\x80\x82R`\x01`\x01`\xA0\x1B\x03\x90\x96\x16\x85\x82\x01\x90\x81R` \x81\x01\x88\x90R\x89\x92\x83\x91\x83\x91\x90\x82\x90\x84\x90\x82\x90`@\x01\x03\x92Z\xF1\x80\x15a\x041Wa\x04\xB0W[PT`\x03T\x82\x16\x90\x82\x16\x80;\x15a\x03\xDEW\x83Qc\xD0\x04\xF0\xF7`\xE0\x1B\x80\x82R`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x81\x87\x01\x90\x81R` \x81\x01\x89\x90R\x8A\x92\x83\x91\x83\x91\x90\x82\x90\x84\x90\x82\x90`@\x01\x03\x92Z\xF1\x80\x15a\x04\xA6Wa\x04\x92W[PP\x81`\x02T\x16\x94\x85;\x15a\x03\xDEW\x83Q\x90cp\xA0\x821`\xE0\x1B\x96\x87\x83R0\x87\x84\x01R\x89\x83`$\x81\x84Z\xFA\x92\x83\x15a\x04\x88W\x8B\x93a\x04YW[P\x84`\x01T\x16\x91\x81;\x15a\x03\xDEW\x86Q\x90\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x82\x88\x01\x90\x81R` \x81\x01\x84\x90R\x8B\x92\x91\x83\x91\x83\x91\x90\x82\x90\x84\x90\x82\x90`@\x01\x03\x92Z\xF1\x80\x15a\x04OWa\x04;W[PP\x82`\x01T\x16\x91\x83`\x02T\x16\x91\x83;\x15a\x03\xDEW\x85Q\x91\x82R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x86\x82\x01\x90\x81R` \x81\x01\x92\x90\x92R\x89\x92\x90\x91\x83\x91\x83\x91\x82\x90\x84\x90\x82\x90`@\x01[\x03\x92Z\xF1\x80\x15a\x041Wa\x04\x1DW[PP`\x03T\x16\x92\x83;\x15a\x03\xDEW\x81Q\x90\x81R0\x83\x82\x01R\x85\x81`$\x81\x87Z\xFA\x95\x86\x15a\x04\x13W\x87\x96a\x03\xE3W[PP\x84a\x03xW[PPPa\x03u\x91\x11a\x08\xAEV[\x80\xF3[\x82;\x15a\x03\xDEW\x80Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3\x92\x81\x01\x92\x83R` \x83\x01\x86\x90R\x86\x93\x90\x92\x84\x91\x84\x91\x82\x90\x84\x90\x82\x90`@\x01\x03\x92Z\xF1\x90\x81\x15a\x03\xD5WPa\x03\xC1W[\x80a\x03hV[a\x03\xCA\x90a\x08bV[a\0\xE7W\x828a\x03\xBBV[Q=\x84\x82>=\x90\xFD[a\x08\x0FV[\x90\x80\x92\x96P\x81=\x83\x11a\x04\x0CW[a\x03\xFB\x81\x83a\x08\x8CV[\x81\x01\x03\x12a\0\xE7WQ\x938\x80a\x03`V[P=a\x03\xF1V[\x82Q=\x89\x82>=\x90\xFD[a\x04&\x90a\x08bV[a\0\xE7W\x868a\x032V[\x84Q=\x84\x82>=\x90\xFD[a\x04D\x90a\x08bV[a\0\xE7W\x888a\x02\xDEV[\x86Q=\x84\x82>=\x90\xFD[\x90\x92P\x89\x81\x81=\x83\x11a\x04\x81W[a\x04q\x81\x83a\x08\x8CV[\x81\x01\x03\x12a\0\xE7WQ\x918a\x02\x93V[P=a\x04gV[\x86Q=\x8D\x82>=\x90\xFD[a\x04\x9B\x90a\x08bV[a\0\xE7W\x878a\x02ZV[\x85Q=\x84\x82>=\x90\xFD[a\x04\xB9\x90a\x08bV[a\0\xE7W\x868a\x02\x05V[a\x04\xCD\x90a\x08bV[a\0\xE7W\x858a\x01\xADV[\x83Q=\x84\x82>=\x90\xFD[P4a\0\xECW6`\x03\x19\x01\x12a\0\xE7W`\x02T\x90Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[P\x914a\x07oW` \x91\x82`\x03\x196\x01\x12a\0\xE7W`\x03T\x825\x92`\x01`\x01`\xA0\x1B\x03\x92\x90\x91\x83\x16\x80;\x15a\x03\xDEW\x81Qc#\xB8r\xDD`\xE0\x1B\x81R3\x84\x82\x01\x90\x81R0` \x82\x01R`@\x81\x01\x87\x90R\x90\x91\x88\x91\x83\x91\x90\x82\x90\x84\x90\x82\x90``\x01\x03\x92Z\xF1\x80\x15a\x04\x13Wa\x07\\W[P\x82`\x03T\x16\x92\x80`\x01T\x16\x93\x80;\x15a\x03\xDEW\x82Qc\t^\xA7\xB3`\xE0\x1B\x80\x82R`\x01`\x01`\xA0\x1B\x03\x90\x96\x16\x85\x82\x01\x90\x81R` \x81\x01\x88\x90R\x89\x92\x83\x91\x83\x91\x90\x82\x90\x84\x90\x82\x90`@\x01\x03\x92Z\xF1\x80\x15a\x041Wa\x07HW[PP\x80`\x01T\x16\x81`\x03T\x16\x90\x80;\x15a\x03\xDEW\x83Qc\xD0\x04\xF0\xF7`\xE0\x1B\x80\x82R`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x81\x87\x01\x90\x81R` \x81\x01\x89\x90R\x8A\x92\x83\x91\x83\x91\x90\x82\x90\x84\x90\x82\x90`@\x01\x03\x92Z\xF1\x80\x15a\x04\xA6Wa\x074W[PP\x81`\x02T\x16\x94\x85;\x15a\x03\xDEW\x83Q\x90cp\xA0\x821`\xE0\x1B\x96\x87\x83R0\x87\x84\x01R\x89\x83`$\x81\x84Z\xFA\x92\x83\x15a\x04\x88W\x8B\x93a\x07\x05W[P\x84\x8BT\x16\x91\x81;\x15a\x03\xDEW\x86Q\x90\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x82\x88\x01\x90\x81R` \x81\x01\x84\x90R\x8B\x92\x91\x83\x91\x83\x91\x90\x82\x90\x84\x90\x82\x90`@\x01\x03\x92Z\xF1\x80\x15a\x04OWa\x06\xF1W[P\x83\x90T\x16\x91\x83`\x02T\x16\x91\x83;\x15a\x03\xDEW\x85Q\x91\x82R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x86\x82\x01\x90\x81R` \x81\x01\x92\x90\x92R\x89\x92\x90\x91\x83\x91\x83\x91\x82\x90\x84\x90\x82\x90`@\x01a\x03#V[a\x06\xFA\x90a\x08bV[a\0\xE7W\x888a\x06\xAAV[\x90\x92P\x89\x81\x81=\x83\x11a\x07-W[a\x07\x1D\x81\x83a\x08\x8CV[\x81\x01\x03\x12a\0\xE7WQ\x918a\x06`V[P=a\x07\x13V[a\x07=\x90a\x08bV[a\0\xE7W\x878a\x06'V[a\x07Q\x90a\x08bV[a\0\xE7W\x868a\x05\xCFV[a\x07h\x90\x96\x91\x96a\x08bV[\x948a\x05wV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01Ra7\xB7`\xF1\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01Rd code`\xD8\x1B`d\x82\x01R`\x84\x90\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x08vW`@RV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x08vW`@RV[\x15a\x08\xB5WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01RmNot profitable`\x90\x1B`D\x82\x01R`d\x90\xFD\xFE\xA2dipfsX\"\x12 \x02\xE4yCk\x9Cf\xC0]}\xF5TO\xCF\x81\x0F*\x15\x15\xBA\xE0?\xB0J\x0E%\x88\xADv\xEC\xED\xEDdsolcC\0\x08\x13\x003";
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
