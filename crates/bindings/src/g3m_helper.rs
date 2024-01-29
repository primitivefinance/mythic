pub use g3m_helper::*;
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
pub mod g3m_helper {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("g3m_"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("g3m"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("g3m"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract G3M"),
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
                                    name: ::std::borrow::ToOwned::to_owned("params"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct G3M.G3MParams"),
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
                                    name: ::std::borrow::ToOwned::to_owned("data"),
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
                    ::std::borrow::ToOwned::to_owned("prepareWeightXUpdate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "prepareWeightXUpdate",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("targetWeightX"),
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
    pub static G3MHELPER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA0\x80`@R4a\0\xE4W`@Q`\x1Fa\x06\xDA8\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x91`\x01`\x01`@\x1B\x03\x83\x11\x84\x84\x10\x17a\0\xCEW\x80\x84\x92` \x94`@R\x839\x81\x01\x03\x12a\0~WQ`\x01`\x01`\xA0\x1B\x03\x81\x16\x90\x81\x90\x03a\0yW`\x80R`@Qa\x05\xA8\x90\x81a\x012\x829`\x80Q\x81\x81\x81`\xD8\x01Ra\x03\xC5\x01R\xF3[`\0\x80\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[bF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01Ra7\xB7`\xF1\x1B`d\x82\x01R`\x84\x90\xFD\xFE`@`\x80\x81R`\x04\x90\x816\x10\x15a\0rW[\x90` `\x84\x92Q\x91bF\x1B\xCD`\xE5\x1B\x83R\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01Rtnor receive functions`X\x1B`d\x82\x01R\xFD[`\0\x805`\xE0\x1C\x80c%\th\xD9\x14a\x03\xF4W\x80c^q1\xA9\x14a\x03\xB2W\x80c\xB0\x9D\x04\xE5\x14a\x03rWc\xDC\x17\x83U\x14a\0\xAAWPa\0\x11V[\x904a\x03mW` \x92\x83`\x03\x196\x01\x12a\x01|W\x82\x82\x80Qa\0\xCB\x81a\x05TV[\x82\x81R\x86\x81\x01\x83\x90R\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x03\x1DW\x83`$\x91\x84Q\x92\x83\x80\x92c\xDC\x17\x83U`\xE0\x1B\x82R\x865\x87\x83\x01RZ\xFA\x93\x84\x15a\x03\x12W\x80\x94a\x01\x81W[PPP``\x82\x80Q\x81\x01\x03\x12a\x01|W\x80Q``\x93a\x01O\x82a\x05TV[\x80\x84\x01Q\x93\x84\x83R\x83\x86\x81\x83\x01Q\x92\x84\x86\x01\x93\x84R\x01Q\x93\x01\x92\x83R\x83Q\x94\x85RQ\x90\x84\x01RQ\x90\x82\x01R\xF3[a\x04\xB5V[\x90\x91\x92\x93P=\x80\x82\x84>a\x01\x95\x81\x84a\x05\x86V[\x82\x01\x92\x85\x83\x85\x03\x12a\x01|W\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93\x84\x82\x11a\x02\xC4W\x01\x91\x84`\x1F\x84\x01\x12\x15a\x02mW\x82Q\x93\x84\x11a\x02[WP\x84Q\x93a\x01\xE1`\x1F\x85\x01`\x1F\x19\x16\x88\x01\x86a\x05\x86V[\x83\x85R\x86\x84\x84\x01\x01\x11a\x02\x08WP\x90a\x01\xFF\x91\x85\x80\x85\x01\x91\x01a\x05\x05V[\x908\x80\x80a\x011V[\x84QbF\x1B\xCD`\xE5\x1B\x81R\x90\x81\x01\x86\x90R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R`\x84\x90\xFD[`A`$\x92cNH{q`\xE0\x1B\x83RR\xFD[\x85QbF\x1B\xCD`\xE5\x1B\x81R\x80\x83\x01\x88\x90R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x90\xFD[\x86QbF\x1B\xCD`\xE5\x1B\x81R\x80\x84\x01\x89\x90R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[\x83Q\x90=\x90\x82>=\x90\xFD[P\x83`\x84\x92Q\x91bF\x1B\xCD`\xE5\x1B\x83R\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01Rd code`\xD8\x1B`d\x82\x01R\xFD[a\x04eV[\x82\x844a\x03mW` 6`\x03\x19\x01\x12a\x01|W\x81a\x03\xAE\x92Q\x91`\x01` \x84\x01R5\x81\x83\x01R\x80\x82Ra\x03\xA4\x82a\x05TV[Q\x91\x82\x91\x82a\x05(V[\x03\x90\xF3[P4a\x03mW6`\x03\x19\x01\x12a\x01|WQ\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[P\x904a\x04eW\x80`\x03\x196\x01\x12a\x01|W\x80Q\x91`\x02` \x84\x01R\x835\x82\x84\x01R`$5``\x84\x01R``\x83R`\x80\x83\x01\x93\x83\x85\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x11\x17a\x04RWPP\x82\x90R`\x7F\x19\x90a\x04M\x81\x84a\x05(V[\x03\x01\x90\xF3[cNH{q`\xE0\x1B\x82R`A\x90R`$\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01Ra7\xB7`\xF1\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[`\0[\x83\x81\x10a\x05\x18WPP`\0\x91\x01RV[\x81\x81\x01Q\x83\x82\x01R` \x01a\x05\x08V[`@\x91` \x82Ra\x05H\x81Q\x80\x92\x81` \x86\x01R` \x86\x86\x01\x91\x01a\x05\x05V[`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[``\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x05pW`@RV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x05pW`@RV";
    /// The bytecode of the contract.
    pub static G3MHELPER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`@`\x80\x81R`\x04\x90\x816\x10\x15a\0rW[\x90` `\x84\x92Q\x91bF\x1B\xCD`\xE5\x1B\x83R\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01Rtnor receive functions`X\x1B`d\x82\x01R\xFD[`\0\x805`\xE0\x1C\x80c%\th\xD9\x14a\x03\xF4W\x80c^q1\xA9\x14a\x03\xB2W\x80c\xB0\x9D\x04\xE5\x14a\x03rWc\xDC\x17\x83U\x14a\0\xAAWPa\0\x11V[\x904a\x03mW` \x92\x83`\x03\x196\x01\x12a\x01|W\x82\x82\x80Qa\0\xCB\x81a\x05TV[\x82\x81R\x86\x81\x01\x83\x90R\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x03\x1DW\x83`$\x91\x84Q\x92\x83\x80\x92c\xDC\x17\x83U`\xE0\x1B\x82R\x865\x87\x83\x01RZ\xFA\x93\x84\x15a\x03\x12W\x80\x94a\x01\x81W[PPP``\x82\x80Q\x81\x01\x03\x12a\x01|W\x80Q``\x93a\x01O\x82a\x05TV[\x80\x84\x01Q\x93\x84\x83R\x83\x86\x81\x83\x01Q\x92\x84\x86\x01\x93\x84R\x01Q\x93\x01\x92\x83R\x83Q\x94\x85RQ\x90\x84\x01RQ\x90\x82\x01R\xF3[a\x04\xB5V[\x90\x91\x92\x93P=\x80\x82\x84>a\x01\x95\x81\x84a\x05\x86V[\x82\x01\x92\x85\x83\x85\x03\x12a\x01|W\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93\x84\x82\x11a\x02\xC4W\x01\x91\x84`\x1F\x84\x01\x12\x15a\x02mW\x82Q\x93\x84\x11a\x02[WP\x84Q\x93a\x01\xE1`\x1F\x85\x01`\x1F\x19\x16\x88\x01\x86a\x05\x86V[\x83\x85R\x86\x84\x84\x01\x01\x11a\x02\x08WP\x90a\x01\xFF\x91\x85\x80\x85\x01\x91\x01a\x05\x05V[\x908\x80\x80a\x011V[\x84QbF\x1B\xCD`\xE5\x1B\x81R\x90\x81\x01\x86\x90R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R`\x84\x90\xFD[`A`$\x92cNH{q`\xE0\x1B\x83RR\xFD[\x85QbF\x1B\xCD`\xE5\x1B\x81R\x80\x83\x01\x88\x90R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x90\xFD[\x86QbF\x1B\xCD`\xE5\x1B\x81R\x80\x84\x01\x89\x90R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[\x83Q\x90=\x90\x82>=\x90\xFD[P\x83`\x84\x92Q\x91bF\x1B\xCD`\xE5\x1B\x83R\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01Rd code`\xD8\x1B`d\x82\x01R\xFD[a\x04eV[\x82\x844a\x03mW` 6`\x03\x19\x01\x12a\x01|W\x81a\x03\xAE\x92Q\x91`\x01` \x84\x01R5\x81\x83\x01R\x80\x82Ra\x03\xA4\x82a\x05TV[Q\x91\x82\x91\x82a\x05(V[\x03\x90\xF3[P4a\x03mW6`\x03\x19\x01\x12a\x01|WQ\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[P\x904a\x04eW\x80`\x03\x196\x01\x12a\x01|W\x80Q\x91`\x02` \x84\x01R\x835\x82\x84\x01R`$5``\x84\x01R``\x83R`\x80\x83\x01\x93\x83\x85\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x11\x17a\x04RWPP\x82\x90R`\x7F\x19\x90a\x04M\x81\x84a\x05(V[\x03\x01\x90\xF3[cNH{q`\xE0\x1B\x82R`A\x90R`$\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01Ra7\xB7`\xF1\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[`\0[\x83\x81\x10a\x05\x18WPP`\0\x91\x01RV[\x81\x81\x01Q\x83\x82\x01R` \x01a\x05\x08V[`@\x91` \x82Ra\x05H\x81Q\x80\x92\x81` \x86\x01R` \x86\x86\x01\x91\x01a\x05\x05V[`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[``\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x05pW`@RV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x05pW`@RV";
    /// The deployed bytecode of the contract.
    pub static G3MHELPER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct G3MHelper<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for G3MHelper<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for G3MHelper<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for G3MHelper<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for G3MHelper<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(G3MHelper)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> G3MHelper<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    G3MHELPER_ABI.clone(),
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
                G3MHELPER_ABI.clone(),
                G3MHELPER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `g3m` (0x5e7131a9) function
        pub fn g_3m(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([94, 113, 49, 169], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPoolParams` (0xdc178355) function
        pub fn get_pool_params(
            &self,
            pool_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, G3Mparams> {
            self.0
                .method_hash([220, 23, 131, 85], pool_id)
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
        ///Calls the contract's `prepareWeightXUpdate` (0x250968d9) function
        pub fn prepare_weight_x_update(
            &self,
            target_weight_x: ::ethers::core::types::U256,
            target_timestamp: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([37, 9, 104, 217], (target_weight_x, target_timestamp))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for G3MHelper<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `g3m` function with signature `g3m()` and selector `0x5e7131a9`
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
    #[ethcall(name = "g3m", abi = "g3m()")]
    pub struct G3MCall;
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
    ///Container type for all input parameters for the `prepareWeightXUpdate` function with signature `prepareWeightXUpdate(uint256,uint256)` and selector `0x250968d9`
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
        name = "prepareWeightXUpdate",
        abi = "prepareWeightXUpdate(uint256,uint256)"
    )]
    pub struct PrepareWeightXUpdateCall {
        pub target_weight_x: ::ethers::core::types::U256,
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
    pub enum G3MHelperCalls {
        G3M(G3MCall),
        GetPoolParams(GetPoolParamsCall),
        PrepareFeeUpdate(PrepareFeeUpdateCall),
        PrepareWeightXUpdate(PrepareWeightXUpdateCall),
    }
    impl ::ethers::core::abi::AbiDecode for G3MHelperCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <G3MCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::G3M(decoded));
            }
            if let Ok(decoded) = <GetPoolParamsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetPoolParams(decoded));
            }
            if let Ok(decoded) = <PrepareFeeUpdateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PrepareFeeUpdate(decoded));
            }
            if let Ok(decoded) = <PrepareWeightXUpdateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PrepareWeightXUpdate(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for G3MHelperCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::G3M(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetPoolParams(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PrepareFeeUpdate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PrepareWeightXUpdate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for G3MHelperCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::G3M(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPoolParams(element) => ::core::fmt::Display::fmt(element, f),
                Self::PrepareFeeUpdate(element) => ::core::fmt::Display::fmt(element, f),
                Self::PrepareWeightXUpdate(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<G3MCall> for G3MHelperCalls {
        fn from(value: G3MCall) -> Self {
            Self::G3M(value)
        }
    }
    impl ::core::convert::From<GetPoolParamsCall> for G3MHelperCalls {
        fn from(value: GetPoolParamsCall) -> Self {
            Self::GetPoolParams(value)
        }
    }
    impl ::core::convert::From<PrepareFeeUpdateCall> for G3MHelperCalls {
        fn from(value: PrepareFeeUpdateCall) -> Self {
            Self::PrepareFeeUpdate(value)
        }
    }
    impl ::core::convert::From<PrepareWeightXUpdateCall> for G3MHelperCalls {
        fn from(value: PrepareWeightXUpdateCall) -> Self {
            Self::PrepareWeightXUpdate(value)
        }
    }
    ///Container type for all return fields from the `g3m` function with signature `g3m()` and selector `0x5e7131a9`
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
    pub struct G3MReturn(pub ::ethers::core::types::Address);
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
        pub params: G3Mparams,
    }
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
    pub struct PrepareFeeUpdateReturn {
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all return fields from the `prepareWeightXUpdate` function with signature `prepareWeightXUpdate(uint256,uint256)` and selector `0x250968d9`
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
    pub struct PrepareWeightXUpdateReturn(pub ::ethers::core::types::Bytes);
}
