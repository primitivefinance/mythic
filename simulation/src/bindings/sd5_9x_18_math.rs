pub use sd5_9x_18_math::*;
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
pub mod sd5_9x_18_math {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("div"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("div"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("x"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("SD59x18"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("y"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("SD59x18"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("z"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("SD59x18"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("mul"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("mul"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("x"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("SD59x18"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("y"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("SD59x18"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("z"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("SD59x18"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("pow"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("pow"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("x"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("SD59x18"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("y"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("SD59x18"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("z"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("SD59x18"),
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
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("PRBMath_MulDiv18_Overflow"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "PRBMath_MulDiv18_Overflow",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("x"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("y"),
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
                    ::std::borrow::ToOwned::to_owned("PRBMath_MulDiv_Overflow"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "PRBMath_MulDiv_Overflow",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("x"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("y"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("denominator"),
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
                    ::std::borrow::ToOwned::to_owned(
                        "PRBMath_SD59x18_Div_InputTooSmall",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "PRBMath_SD59x18_Div_InputTooSmall",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PRBMath_SD59x18_Div_Overflow"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "PRBMath_SD59x18_Div_Overflow",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("x"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("SD59x18"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("y"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("SD59x18"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PRBMath_SD59x18_Exp2_InputTooBig"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "PRBMath_SD59x18_Exp2_InputTooBig",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("x"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("SD59x18"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "PRBMath_SD59x18_Log_InputTooSmall",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "PRBMath_SD59x18_Log_InputTooSmall",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("x"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("SD59x18"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "PRBMath_SD59x18_Mul_InputTooSmall",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "PRBMath_SD59x18_Mul_InputTooSmall",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PRBMath_SD59x18_Mul_Overflow"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "PRBMath_SD59x18_Mul_Overflow",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("x"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("SD59x18"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("y"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("SD59x18"),
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
    pub static SD59X18MATH_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80\x80`@R4a\0\x19W`@Qa\x11{\x90\x81a\0g\x829\xF3[bF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01Ra7\xB7`\xF1\x1B`d\x82\x01R`\x84\x90\xFD\xFE`@`\x80\x81R`\x046\x10\x15a\0\x92W[`\x84\x90Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01R\x7Fnor receive functions\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R\xFD[`\0\x805`\xE0\x1C\x90\x81cCP\x918\x14a\0\xFCWP\x80c\x92\xB0\xC5\xB2\x14a\0\xE1Wc\xBB\xE9=\x91\x03a\0\x0FW4a\0\xDCW` \x90a\0\xD5a\0\xCF6a\x02\xBFV[\x90a\x0E\x9AV[\x90Q\x90\x81R\xF3[a\x02;V[P4a\0\xDCW` \x90a\0\xD5a\0\xF66a\x02\xBFV[\x90a\x03wV[\x824a\x02;Wa\x01\x0B6a\x02\xBFV[\x90\x92\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80\x85\x14\x90\x81\x15a\x021W[Pa\x02\x08W\x80\x84\x12\x15a\x01\xFFWa\x01^\x84\x82\x03[\x82\x84\x12\x15a\x01\xF8W\x83\x83\x03\x90a\x10\x93V[\x91\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x01\xC2W` \x94\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x18\x13\x15a\x01\xBBWP\x90[Q\x90\x81R\xF3[\x03\x90a\x01\xB5V[\x84`D\x91\x85Q\x91\x7F\xD4\x9C&\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04\x83\x01R`$\x82\x01R\xFD[\x83\x90a\x10\x93V[a\x01^\x84a\x01MV[`\x04\x83Q\x7F\x9F\xE2\xB4P\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xFD[\x90P\x82\x14\x85a\x019V[`\x84`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R\xFD[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC`@\x91\x01\x12a\x02\xF3W`\x045\x90`$5\x90V[`\x84`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01R\x7Frt\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R\xFD[\x80a\x03\x94WP`\0\x90a\x03\x91WPg\r\xE0\xB6\xB3\xA7d\0\0\x90V[\x90V[\x90g\r\xE0\xB6\xB3\xA7d\0\0\x80\x83\x14a\x03\xE6WP\x80a\x03\xB9WPPg\r\xE0\xB6\xB3\xA7d\0\0\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x81\x14a\x03\xE2Wa\x03\xDD\x90a\x03\xD8a\x03\x91\x93a\r\x05V[a\x0E\x9AV[a\x045V[P\x90V[\x91PP\x90V[\x80\x15a\x04\x06Wn\xC0\x97\xCE{\xC9\x07\x15\xB3K\x9F\x10\0\0\0\0\x05\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[`\0\x81\x12\x15a\x04\x80W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC\xC2.\x87\xF6\xEBF\x8E\xEB\x81\x12a\x04zWa\x04q\x90`\0\x03a\x045V[a\x03\x91\x90a\x03\xECV[P`\0\x90V[h\nh\x89\x06\xBD\x8A\xFF\xFF\xFF\x81\x13a\x0C\xD4Wg\r\xE0\xB6\xB3\xA7d\0\0\x80`@\x92\x83\x1B\x05\x90w\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\0\0\0\0\0\0\0\x83\x16a\x0B\xB7W[f\xFF\0\0\0\0\0\0\x83\x16a\n\xAFW[e\xFF\0\0\0\0\0\x83\x16a\t\xAFW[d\xFF\0\0\0\0\x83\x16a\x08\xB7W[c\xFF\0\0\0\x83\x16a\x07\xC7W[b\xFF\0\0\x83\x16a\x06\xDFW[a\xFF\0\x83\x16a\x05\xFFW[`\xFF\x83\x16a\x05(W[\x02\x91\x1C`\xBF\x03\x1C\x90V[`\x80\x83\x16a\x05\xEDW[\x83\x83\x16a\x05\xDBW[` \x83\x16a\x05\xC9W[`\x10\x83\x16a\x05\xB7W[`\x08\x83\x16a\x05\xA5W[`\x04\x83\x16a\x05\x93W[`\x02\x83\x16a\x05\x81W[`\x01\x83\x16\x15a\x05\x1EWh\x01\0\0\0\0\0\0\0\x01\x02\x83\x1Ca\x05\x1EV[h\x01\0\0\0\0\0\0\0\x01\x02\x83\x1Ca\x05fV[h\x01\0\0\0\0\0\0\0\x03\x02\x83\x1Ca\x05]V[h\x01\0\0\0\0\0\0\0\x06\x02\x83\x1Ca\x05TV[h\x01\0\0\0\0\0\0\0\x0B\x02\x83\x1Ca\x05KV[h\x01\0\0\0\0\0\0\0\x16\x02\x83\x1Ca\x05BV[h\x01\0\0\0\0\0\0\0,\x02\x83\x1Ca\x059V[h\x01\0\0\0\0\0\0\0Y\x02\x83\x1Ca\x051V[a\x80\0\x83\x16a\x06\xCDW[a@\0\x83\x16a\x06\xBBW[a \0\x83\x16a\x06\xA9W[a\x10\0\x83\x16a\x06\x97W[a\x08\0\x83\x16a\x06\x85W[a\x04\0\x83\x16a\x06sW[a\x02\0\x83\x16a\x06aW[a\x01\0\x83\x16\x15a\x05\x15Wh\x01\0\0\0\0\0\0\0\xB1\x02\x83\x1Ca\x05\x15V[h\x01\0\0\0\0\0\0\x01c\x02\x83\x1Ca\x06EV[h\x01\0\0\0\0\0\0\x02\xC6\x02\x83\x1Ca\x06;V[h\x01\0\0\0\0\0\0\x05\x8C\x02\x83\x1Ca\x061V[h\x01\0\0\0\0\0\0\x0B\x17\x02\x83\x1Ca\x06'V[h\x01\0\0\0\0\0\0\x16.\x02\x83\x1Ca\x06\x1DV[h\x01\0\0\0\0\0\0,]\x02\x83\x1Ca\x06\x13V[h\x01\0\0\0\0\0\0X\xB9\x02\x83\x1Ca\x06\tV[b\x80\0\0\x83\x16a\x07\xB5W[b@\0\0\x83\x16a\x07\xA3W[b \0\0\x83\x16a\x07\x91W[b\x10\0\0\x83\x16a\x07\x7FW[b\x08\0\0\x83\x16a\x07mW[b\x04\0\0\x83\x16a\x07[W[b\x02\0\0\x83\x16a\x07IW[b\x01\0\0\x83\x16\x15a\x05\x0BWh\x01\0\0\0\0\0\0\xB1r\x02\x83\x1Ca\x05\x0BV[h\x01\0\0\0\0\0\x01b\xE4\x02\x83\x1Ca\x07,V[h\x01\0\0\0\0\0\x02\xC5\xC8\x02\x83\x1Ca\x07!V[h\x01\0\0\0\0\0\x05\x8B\x91\x02\x83\x1Ca\x07\x16V[h\x01\0\0\0\0\0\x0B\x17!\x02\x83\x1Ca\x07\x0BV[h\x01\0\0\0\0\0\x16.C\x02\x83\x1Ca\x07\0V[h\x01\0\0\0\0\0,\\\x86\x02\x83\x1Ca\x06\xF5V[h\x01\0\0\0\0\0X\xB9\x0C\x02\x83\x1Ca\x06\xEAV[c\x80\0\0\0\x83\x16a\x08\xA5W[c@\0\0\0\x83\x16a\x08\x93W[c \0\0\0\x83\x16a\x08\x81W[c\x10\0\0\0\x83\x16a\x08oW[c\x08\0\0\0\x83\x16a\x08]W[c\x04\0\0\0\x83\x16a\x08KW[c\x02\0\0\0\x83\x16a\x089W[c\x01\0\0\0\x83\x16\x15a\x05\0Wh\x01\0\0\0\0\0\xB1r\x18\x02\x83\x1Ca\x05\0V[h\x01\0\0\0\0\x01b\xE40\x02\x83\x1Ca\x08\x1BV[h\x01\0\0\0\0\x02\xC5\xC8`\x02\x83\x1Ca\x08\x0FV[h\x01\0\0\0\0\x05\x8B\x90\xC0\x02\x83\x1Ca\x08\x03V[h\x01\0\0\0\0\x0B\x17!\x7F\x02\x83\x1Ca\x07\xF7V[h\x01\0\0\0\0\x16.B\xFF\x02\x83\x1Ca\x07\xEBV[h\x01\0\0\0\0,\\\x85\xFE\x02\x83\x1Ca\x07\xDFV[h\x01\0\0\0\0X\xB9\x0B\xFC\x02\x83\x1Ca\x07\xD3V[d\x80\0\0\0\0\x83\x16a\t\x9DW[d@\0\0\0\0\x83\x16a\t\x8BW[d \0\0\0\0\x83\x16a\tyW[d\x10\0\0\0\0\x83\x16a\tgW[d\x08\0\0\0\0\x83\x16a\tUW[d\x04\0\0\0\0\x83\x16a\tCW[d\x02\0\0\0\0\x83\x16a\t1W[d\x01\0\0\0\0\x83\x16\x15a\x04\xF4Wh\x01\0\0\0\0\xB1r\x17\xF8\x02\x83\x1Ca\x04\xF4V[h\x01\0\0\0\x01b\xE4/\xF1\x02\x83\x1Ca\t\x12V[h\x01\0\0\0\x02\xC5\xC8_\xE3\x02\x83\x1Ca\t\x05V[h\x01\0\0\0\x05\x8B\x90\xBF\xCE\x02\x83\x1Ca\x08\xF8V[h\x01\0\0\0\x0B\x17!\x7F\xBB\x02\x83\x1Ca\x08\xEBV[h\x01\0\0\0\x16.B\xFF\xF0\x02\x83\x1Ca\x08\xDEV[h\x01\0\0\0,\\\x86\x01\xCC\x02\x83\x1Ca\x08\xD1V[h\x01\0\0\0X\xB9\x0C\x0BI\x02\x83\x1Ca\x08\xC4V[e\x80\0\0\0\0\0\x83\x16a\n\x9DW[e@\0\0\0\0\0\x83\x16a\n\x8BW[e \0\0\0\0\0\x83\x16a\nyW[e\x10\0\0\0\0\0\x83\x16a\ngW[e\x08\0\0\0\0\0\x83\x16a\nUW[e\x04\0\0\0\0\0\x83\x16a\nCW[e\x02\0\0\0\0\0\x83\x16a\n1W[e\x01\0\0\0\0\0\x83\x16\x15a\x04\xE7Wh\x01\0\0\0\xB1r\x185Q\x02\x83\x1Ca\x04\xE7V[h\x01\0\0\x01b\xE40\xE5\xA2\x02\x83\x1Ca\n\x11V[h\x01\0\0\x02\xC5\xC8c\xB7?\x02\x83\x1Ca\n\x03V[h\x01\0\0\x05\x8B\x90\xCF\x1En\x02\x83\x1Ca\t\xF5V[h\x01\0\0\x0B\x17!\xBC\xFC\x9A\x02\x83\x1Ca\t\xE7V[h\x01\0\0\x16.C\xF4\xF81\x02\x83\x1Ca\t\xD9V[h\x01\0\0,\\\x89\xD5\xECm\x02\x83\x1Ca\t\xCBV[h\x01\0\0X\xB9\x1B[\xC9\xAE\x02\x83\x1Ca\t\xBDV[f\x80\0\0\0\0\0\0\x83\x16a\x0B\xA5W[f@\0\0\0\0\0\0\x83\x16a\x0B\x93W[f \0\0\0\0\0\0\x83\x16a\x0B\x81W[f\x10\0\0\0\0\0\0\x83\x16a\x0BoW[f\x08\0\0\0\0\0\0\x83\x16a\x0B]W[f\x04\0\0\0\0\0\0\x83\x16a\x0BKW[f\x02\0\0\0\0\0\0\x83\x16a\x0B9W[f\x01\0\0\0\0\0\0\x83\x16\x15a\x04\xD9Wh\x01\0\0\xB1rUw\\\x04\x02\x83\x1Ca\x04\xD9V[h\x01\0\x01b\xE5%\xEE\x05G\x02\x83\x1Ca\x0B\x18V[h\x01\0\x02\xC5\xCC7\xDA\x94\x92\x02\x83\x1Ca\x0B\tV[h\x01\0\x05\x8B\xA0\x1F\xB9\xF9m\x02\x83\x1Ca\n\xFAV[h\x01\0\x0B\x17^\xFF\xDCv\xBA\x02\x83\x1Ca\n\xEBV[h\x01\0\x16/9\x04\x05\x1F\xA1\x02\x83\x1Ca\n\xDCV[h\x01\0,`^.\x8C\xECP\x02\x83\x1Ca\n\xCDV[h\x01\0X\xC8m\xA1\xC0\x9E\xA2\x02\x83\x1Ca\n\xBEV[g\x80\0\0\0\0\0\0\0\x83\x16a\x0C\xB5W[g@\0\0\0\0\0\0\0\x83\x16a\x0C\xA3W[g \0\0\0\0\0\0\0\x83\x16a\x0C\x91W[g\x10\0\0\0\0\0\0\0\x83\x16a\x0C\x7FW[g\x08\0\0\0\0\0\0\0\x83\x16a\x0CmW[g\x04\0\0\0\0\0\0\0\x83\x16a\x0C[W[g\x02\0\0\0\0\0\0\0\x83\x16a\x0CIW[g\x01\0\0\0\0\0\0\0\x83\x16\x15a\x04\xCAWh\x01\0\xB1\xAF\xA5\xAB\xCB\xEDa\x02\x83\x1Ca\x04\xCAV[h\x01\x01c\xDA\x9F\xB33V\xD8\x02\x83\x1Ca\x0C'V[h\x01\x02\xC9\xA3\xE7x\x06\x0E\xE7\x02\x83\x1Ca\x0C\x17V[h\x01\x05\x9B\r1XWC\xAE\x02\x83\x1Ca\x0C\x07V[h\x01\x0BU\x86\xCF\x98\x90\xF6*\x02\x83\x1Ca\x0B\xF7V[h\x01\x17+\x83\xC7\xD5\x17\xAD\xCE\x02\x83\x1Ca\x0B\xE7V[h\x010o\xE0\xA3\x1BqR\xDF\x02\x83\x1Ca\x0B\xD7V[Pw\xB5\x04\xF33\xF9\xDEd\x84\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x0B\xC7V[`$\x90`@Q\x90\x7F\x03`\xD0(\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\xFD[\x80`\0\x80\x83\x13\x15a\x0EiWg\r\xE0\xB6\xB3\xA7d\0\0\x92\x83\x81\x12a\r\xFBWP`\x01\x92[\x80\x83\x05\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x91\x82\x1C\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x92\x83\x1Cc\xFF\xFF\xFF\xFF\x81\x11`\x05\x1B\x90\x81\x1Ca\xFF\xFF\x81\x11`\x04\x1B\x90\x81\x1C`\xFF\x81\x11`\x03\x1B\x90\x81\x1C\x91`\x0F\x83\x11`\x02\x1B\x92\x83\x1C\x93`\x01\x97\x88`\x03\x87\x11\x81\x1B\x96\x87\x1C\x11\x96\x17\x17\x17\x17\x17\x17\x17\x81\x81\x02\x94\x81\x1D\x90\x82\x82\x14a\r\xEFWPg\x06\xF0[Y\xD3\xB2\0\0\x90[\x84\x82\x13a\r\xC3WPPPPP\x02\x90V[\x80\x83\x91\x02\x05\x90g\x1B\xC1mgN\xC8\0\0\x82\x12\x15a\r\xE2W[\x83\x1D\x90a\r\xB3V[\x80\x91\x95\x01\x94\x83\x1D\x90a\r\xDAV[\x93PP\x93\x92PP\x02\x02\x90V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93\x92P\x80\x15a\x0E<Wn\xC0\x97\xCE{\xC9\x07\x15\xB3K\x9F\x10\0\0\0\0\x05\x91a\r&V[`$\x82\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x12`\x04R\xFD[`$\x83`@Q\x90\x7F\x05\x9B\x10\x1B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\xFD[\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80\x82\x14\x90\x81\x15a\x0F\xBCW[Pa\x0F\x92W`\0\x81\x12\x15a\x0F\x89Wa\x0E\xEF\x81`\0\x03[`\0\x84\x12\x15a\x0F\x82W\x83`\0\x03\x90a\x0F\xC6V[\x91\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x0FKW`\0\x91\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x18\x13\x15a\x0FGWP\x90V[\x03\x90V[`D\x92P`@Q\x91\x7F\x12\x0B[C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04\x83\x01R`$\x82\x01R\xFD[\x83\x90a\x0F\xC6V[a\x0E\xEF\x81a\x0E\xDCV[`\x04`@Q\x7F\xA6\x07\x0C%\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xFD[\x90P\x82\x148a\x0E\xC6V[\x91\x90\x91\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x82\t\x83\x82\x02\x91\x82\x80\x83\x10\x92\x03\x91\x80\x83\x03\x92\x14a\x10\x82Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x83\x10\x15a\x10KW\x94\x7F\xAC\xCB\x18\x16[\xD6\xFE1\xAE\x1C\xF3\x18\xDC[Q\xEE\xE0\xE1\xBAV\x9B\x88\xCDt\xC1w;\x91\xFA\xC1\x06i\x94\x95\t\x90\x82\x82\x11\x90\x03`\xEE\x1B\x91\x03`\x12\x1C\x17\x02\x90V[`D\x90\x86`@Q\x91\x7FQsd\x8D\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04\x83\x01R`$\x82\x01R\xFD[PPg\r\xE0\xB6\xB3\xA7d\0\0\x90\x04\x91PV[g\r\xE0\xB6\xB3\xA7d\0\0\x91\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x83\t\x92\x80\x83\x02\x92\x83\x80\x86\x10\x95\x03\x94\x80\x86\x03\x95\x14a\x11mW\x82\x85\x10\x15a\x111W\x90\x82\x91\t`\x01\x82\x19\x01\x82\x16\x80\x92\x04`\x02\x80\x82`\x03\x02\x18\x80\x83\x02\x82\x03\x02\x80\x83\x02\x82\x03\x02\x80\x83\x02\x82\x03\x02\x80\x83\x02\x82\x03\x02\x80\x83\x02\x82\x03\x02\x80\x92\x02\x90\x03\x02\x93`\x01\x83\x80`\0\x03\x04\x01\x90\x84\x83\x11\x90\x03\x02\x92\x03\x04\x17\x02\x90V[\x82`d\x92`@Q\x92\x7Fc\xA0Wx\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01R`$\x83\x01R`D\x82\x01R\xFD[PP\x80\x92P\x15a\x04\x06W\x04\x90V";
    /// The bytecode of the contract.
    pub static SD59X18MATH_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`@`\x80\x81R`\x046\x10\x15a\0\x92W[`\x84\x90Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01R\x7Fnor receive functions\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R\xFD[`\0\x805`\xE0\x1C\x90\x81cCP\x918\x14a\0\xFCWP\x80c\x92\xB0\xC5\xB2\x14a\0\xE1Wc\xBB\xE9=\x91\x03a\0\x0FW4a\0\xDCW` \x90a\0\xD5a\0\xCF6a\x02\xBFV[\x90a\x0E\x9AV[\x90Q\x90\x81R\xF3[a\x02;V[P4a\0\xDCW` \x90a\0\xD5a\0\xF66a\x02\xBFV[\x90a\x03wV[\x824a\x02;Wa\x01\x0B6a\x02\xBFV[\x90\x92\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80\x85\x14\x90\x81\x15a\x021W[Pa\x02\x08W\x80\x84\x12\x15a\x01\xFFWa\x01^\x84\x82\x03[\x82\x84\x12\x15a\x01\xF8W\x83\x83\x03\x90a\x10\x93V[\x91\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x01\xC2W` \x94\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x18\x13\x15a\x01\xBBWP\x90[Q\x90\x81R\xF3[\x03\x90a\x01\xB5V[\x84`D\x91\x85Q\x91\x7F\xD4\x9C&\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04\x83\x01R`$\x82\x01R\xFD[\x83\x90a\x10\x93V[a\x01^\x84a\x01MV[`\x04\x83Q\x7F\x9F\xE2\xB4P\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xFD[\x90P\x82\x14\x85a\x019V[`\x84`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R\xFD[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC`@\x91\x01\x12a\x02\xF3W`\x045\x90`$5\x90V[`\x84`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01R\x7Frt\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R\xFD[\x80a\x03\x94WP`\0\x90a\x03\x91WPg\r\xE0\xB6\xB3\xA7d\0\0\x90V[\x90V[\x90g\r\xE0\xB6\xB3\xA7d\0\0\x80\x83\x14a\x03\xE6WP\x80a\x03\xB9WPPg\r\xE0\xB6\xB3\xA7d\0\0\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x81\x14a\x03\xE2Wa\x03\xDD\x90a\x03\xD8a\x03\x91\x93a\r\x05V[a\x0E\x9AV[a\x045V[P\x90V[\x91PP\x90V[\x80\x15a\x04\x06Wn\xC0\x97\xCE{\xC9\x07\x15\xB3K\x9F\x10\0\0\0\0\x05\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[`\0\x81\x12\x15a\x04\x80W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC\xC2.\x87\xF6\xEBF\x8E\xEB\x81\x12a\x04zWa\x04q\x90`\0\x03a\x045V[a\x03\x91\x90a\x03\xECV[P`\0\x90V[h\nh\x89\x06\xBD\x8A\xFF\xFF\xFF\x81\x13a\x0C\xD4Wg\r\xE0\xB6\xB3\xA7d\0\0\x80`@\x92\x83\x1B\x05\x90w\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\0\0\0\0\0\0\0\x83\x16a\x0B\xB7W[f\xFF\0\0\0\0\0\0\x83\x16a\n\xAFW[e\xFF\0\0\0\0\0\x83\x16a\t\xAFW[d\xFF\0\0\0\0\x83\x16a\x08\xB7W[c\xFF\0\0\0\x83\x16a\x07\xC7W[b\xFF\0\0\x83\x16a\x06\xDFW[a\xFF\0\x83\x16a\x05\xFFW[`\xFF\x83\x16a\x05(W[\x02\x91\x1C`\xBF\x03\x1C\x90V[`\x80\x83\x16a\x05\xEDW[\x83\x83\x16a\x05\xDBW[` \x83\x16a\x05\xC9W[`\x10\x83\x16a\x05\xB7W[`\x08\x83\x16a\x05\xA5W[`\x04\x83\x16a\x05\x93W[`\x02\x83\x16a\x05\x81W[`\x01\x83\x16\x15a\x05\x1EWh\x01\0\0\0\0\0\0\0\x01\x02\x83\x1Ca\x05\x1EV[h\x01\0\0\0\0\0\0\0\x01\x02\x83\x1Ca\x05fV[h\x01\0\0\0\0\0\0\0\x03\x02\x83\x1Ca\x05]V[h\x01\0\0\0\0\0\0\0\x06\x02\x83\x1Ca\x05TV[h\x01\0\0\0\0\0\0\0\x0B\x02\x83\x1Ca\x05KV[h\x01\0\0\0\0\0\0\0\x16\x02\x83\x1Ca\x05BV[h\x01\0\0\0\0\0\0\0,\x02\x83\x1Ca\x059V[h\x01\0\0\0\0\0\0\0Y\x02\x83\x1Ca\x051V[a\x80\0\x83\x16a\x06\xCDW[a@\0\x83\x16a\x06\xBBW[a \0\x83\x16a\x06\xA9W[a\x10\0\x83\x16a\x06\x97W[a\x08\0\x83\x16a\x06\x85W[a\x04\0\x83\x16a\x06sW[a\x02\0\x83\x16a\x06aW[a\x01\0\x83\x16\x15a\x05\x15Wh\x01\0\0\0\0\0\0\0\xB1\x02\x83\x1Ca\x05\x15V[h\x01\0\0\0\0\0\0\x01c\x02\x83\x1Ca\x06EV[h\x01\0\0\0\0\0\0\x02\xC6\x02\x83\x1Ca\x06;V[h\x01\0\0\0\0\0\0\x05\x8C\x02\x83\x1Ca\x061V[h\x01\0\0\0\0\0\0\x0B\x17\x02\x83\x1Ca\x06'V[h\x01\0\0\0\0\0\0\x16.\x02\x83\x1Ca\x06\x1DV[h\x01\0\0\0\0\0\0,]\x02\x83\x1Ca\x06\x13V[h\x01\0\0\0\0\0\0X\xB9\x02\x83\x1Ca\x06\tV[b\x80\0\0\x83\x16a\x07\xB5W[b@\0\0\x83\x16a\x07\xA3W[b \0\0\x83\x16a\x07\x91W[b\x10\0\0\x83\x16a\x07\x7FW[b\x08\0\0\x83\x16a\x07mW[b\x04\0\0\x83\x16a\x07[W[b\x02\0\0\x83\x16a\x07IW[b\x01\0\0\x83\x16\x15a\x05\x0BWh\x01\0\0\0\0\0\0\xB1r\x02\x83\x1Ca\x05\x0BV[h\x01\0\0\0\0\0\x01b\xE4\x02\x83\x1Ca\x07,V[h\x01\0\0\0\0\0\x02\xC5\xC8\x02\x83\x1Ca\x07!V[h\x01\0\0\0\0\0\x05\x8B\x91\x02\x83\x1Ca\x07\x16V[h\x01\0\0\0\0\0\x0B\x17!\x02\x83\x1Ca\x07\x0BV[h\x01\0\0\0\0\0\x16.C\x02\x83\x1Ca\x07\0V[h\x01\0\0\0\0\0,\\\x86\x02\x83\x1Ca\x06\xF5V[h\x01\0\0\0\0\0X\xB9\x0C\x02\x83\x1Ca\x06\xEAV[c\x80\0\0\0\x83\x16a\x08\xA5W[c@\0\0\0\x83\x16a\x08\x93W[c \0\0\0\x83\x16a\x08\x81W[c\x10\0\0\0\x83\x16a\x08oW[c\x08\0\0\0\x83\x16a\x08]W[c\x04\0\0\0\x83\x16a\x08KW[c\x02\0\0\0\x83\x16a\x089W[c\x01\0\0\0\x83\x16\x15a\x05\0Wh\x01\0\0\0\0\0\xB1r\x18\x02\x83\x1Ca\x05\0V[h\x01\0\0\0\0\x01b\xE40\x02\x83\x1Ca\x08\x1BV[h\x01\0\0\0\0\x02\xC5\xC8`\x02\x83\x1Ca\x08\x0FV[h\x01\0\0\0\0\x05\x8B\x90\xC0\x02\x83\x1Ca\x08\x03V[h\x01\0\0\0\0\x0B\x17!\x7F\x02\x83\x1Ca\x07\xF7V[h\x01\0\0\0\0\x16.B\xFF\x02\x83\x1Ca\x07\xEBV[h\x01\0\0\0\0,\\\x85\xFE\x02\x83\x1Ca\x07\xDFV[h\x01\0\0\0\0X\xB9\x0B\xFC\x02\x83\x1Ca\x07\xD3V[d\x80\0\0\0\0\x83\x16a\t\x9DW[d@\0\0\0\0\x83\x16a\t\x8BW[d \0\0\0\0\x83\x16a\tyW[d\x10\0\0\0\0\x83\x16a\tgW[d\x08\0\0\0\0\x83\x16a\tUW[d\x04\0\0\0\0\x83\x16a\tCW[d\x02\0\0\0\0\x83\x16a\t1W[d\x01\0\0\0\0\x83\x16\x15a\x04\xF4Wh\x01\0\0\0\0\xB1r\x17\xF8\x02\x83\x1Ca\x04\xF4V[h\x01\0\0\0\x01b\xE4/\xF1\x02\x83\x1Ca\t\x12V[h\x01\0\0\0\x02\xC5\xC8_\xE3\x02\x83\x1Ca\t\x05V[h\x01\0\0\0\x05\x8B\x90\xBF\xCE\x02\x83\x1Ca\x08\xF8V[h\x01\0\0\0\x0B\x17!\x7F\xBB\x02\x83\x1Ca\x08\xEBV[h\x01\0\0\0\x16.B\xFF\xF0\x02\x83\x1Ca\x08\xDEV[h\x01\0\0\0,\\\x86\x01\xCC\x02\x83\x1Ca\x08\xD1V[h\x01\0\0\0X\xB9\x0C\x0BI\x02\x83\x1Ca\x08\xC4V[e\x80\0\0\0\0\0\x83\x16a\n\x9DW[e@\0\0\0\0\0\x83\x16a\n\x8BW[e \0\0\0\0\0\x83\x16a\nyW[e\x10\0\0\0\0\0\x83\x16a\ngW[e\x08\0\0\0\0\0\x83\x16a\nUW[e\x04\0\0\0\0\0\x83\x16a\nCW[e\x02\0\0\0\0\0\x83\x16a\n1W[e\x01\0\0\0\0\0\x83\x16\x15a\x04\xE7Wh\x01\0\0\0\xB1r\x185Q\x02\x83\x1Ca\x04\xE7V[h\x01\0\0\x01b\xE40\xE5\xA2\x02\x83\x1Ca\n\x11V[h\x01\0\0\x02\xC5\xC8c\xB7?\x02\x83\x1Ca\n\x03V[h\x01\0\0\x05\x8B\x90\xCF\x1En\x02\x83\x1Ca\t\xF5V[h\x01\0\0\x0B\x17!\xBC\xFC\x9A\x02\x83\x1Ca\t\xE7V[h\x01\0\0\x16.C\xF4\xF81\x02\x83\x1Ca\t\xD9V[h\x01\0\0,\\\x89\xD5\xECm\x02\x83\x1Ca\t\xCBV[h\x01\0\0X\xB9\x1B[\xC9\xAE\x02\x83\x1Ca\t\xBDV[f\x80\0\0\0\0\0\0\x83\x16a\x0B\xA5W[f@\0\0\0\0\0\0\x83\x16a\x0B\x93W[f \0\0\0\0\0\0\x83\x16a\x0B\x81W[f\x10\0\0\0\0\0\0\x83\x16a\x0BoW[f\x08\0\0\0\0\0\0\x83\x16a\x0B]W[f\x04\0\0\0\0\0\0\x83\x16a\x0BKW[f\x02\0\0\0\0\0\0\x83\x16a\x0B9W[f\x01\0\0\0\0\0\0\x83\x16\x15a\x04\xD9Wh\x01\0\0\xB1rUw\\\x04\x02\x83\x1Ca\x04\xD9V[h\x01\0\x01b\xE5%\xEE\x05G\x02\x83\x1Ca\x0B\x18V[h\x01\0\x02\xC5\xCC7\xDA\x94\x92\x02\x83\x1Ca\x0B\tV[h\x01\0\x05\x8B\xA0\x1F\xB9\xF9m\x02\x83\x1Ca\n\xFAV[h\x01\0\x0B\x17^\xFF\xDCv\xBA\x02\x83\x1Ca\n\xEBV[h\x01\0\x16/9\x04\x05\x1F\xA1\x02\x83\x1Ca\n\xDCV[h\x01\0,`^.\x8C\xECP\x02\x83\x1Ca\n\xCDV[h\x01\0X\xC8m\xA1\xC0\x9E\xA2\x02\x83\x1Ca\n\xBEV[g\x80\0\0\0\0\0\0\0\x83\x16a\x0C\xB5W[g@\0\0\0\0\0\0\0\x83\x16a\x0C\xA3W[g \0\0\0\0\0\0\0\x83\x16a\x0C\x91W[g\x10\0\0\0\0\0\0\0\x83\x16a\x0C\x7FW[g\x08\0\0\0\0\0\0\0\x83\x16a\x0CmW[g\x04\0\0\0\0\0\0\0\x83\x16a\x0C[W[g\x02\0\0\0\0\0\0\0\x83\x16a\x0CIW[g\x01\0\0\0\0\0\0\0\x83\x16\x15a\x04\xCAWh\x01\0\xB1\xAF\xA5\xAB\xCB\xEDa\x02\x83\x1Ca\x04\xCAV[h\x01\x01c\xDA\x9F\xB33V\xD8\x02\x83\x1Ca\x0C'V[h\x01\x02\xC9\xA3\xE7x\x06\x0E\xE7\x02\x83\x1Ca\x0C\x17V[h\x01\x05\x9B\r1XWC\xAE\x02\x83\x1Ca\x0C\x07V[h\x01\x0BU\x86\xCF\x98\x90\xF6*\x02\x83\x1Ca\x0B\xF7V[h\x01\x17+\x83\xC7\xD5\x17\xAD\xCE\x02\x83\x1Ca\x0B\xE7V[h\x010o\xE0\xA3\x1BqR\xDF\x02\x83\x1Ca\x0B\xD7V[Pw\xB5\x04\xF33\xF9\xDEd\x84\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x0B\xC7V[`$\x90`@Q\x90\x7F\x03`\xD0(\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\xFD[\x80`\0\x80\x83\x13\x15a\x0EiWg\r\xE0\xB6\xB3\xA7d\0\0\x92\x83\x81\x12a\r\xFBWP`\x01\x92[\x80\x83\x05\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x91\x82\x1C\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x92\x83\x1Cc\xFF\xFF\xFF\xFF\x81\x11`\x05\x1B\x90\x81\x1Ca\xFF\xFF\x81\x11`\x04\x1B\x90\x81\x1C`\xFF\x81\x11`\x03\x1B\x90\x81\x1C\x91`\x0F\x83\x11`\x02\x1B\x92\x83\x1C\x93`\x01\x97\x88`\x03\x87\x11\x81\x1B\x96\x87\x1C\x11\x96\x17\x17\x17\x17\x17\x17\x17\x81\x81\x02\x94\x81\x1D\x90\x82\x82\x14a\r\xEFWPg\x06\xF0[Y\xD3\xB2\0\0\x90[\x84\x82\x13a\r\xC3WPPPPP\x02\x90V[\x80\x83\x91\x02\x05\x90g\x1B\xC1mgN\xC8\0\0\x82\x12\x15a\r\xE2W[\x83\x1D\x90a\r\xB3V[\x80\x91\x95\x01\x94\x83\x1D\x90a\r\xDAV[\x93PP\x93\x92PP\x02\x02\x90V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93\x92P\x80\x15a\x0E<Wn\xC0\x97\xCE{\xC9\x07\x15\xB3K\x9F\x10\0\0\0\0\x05\x91a\r&V[`$\x82\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x12`\x04R\xFD[`$\x83`@Q\x90\x7F\x05\x9B\x10\x1B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\xFD[\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80\x82\x14\x90\x81\x15a\x0F\xBCW[Pa\x0F\x92W`\0\x81\x12\x15a\x0F\x89Wa\x0E\xEF\x81`\0\x03[`\0\x84\x12\x15a\x0F\x82W\x83`\0\x03\x90a\x0F\xC6V[\x91\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x0FKW`\0\x91\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x18\x13\x15a\x0FGWP\x90V[\x03\x90V[`D\x92P`@Q\x91\x7F\x12\x0B[C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04\x83\x01R`$\x82\x01R\xFD[\x83\x90a\x0F\xC6V[a\x0E\xEF\x81a\x0E\xDCV[`\x04`@Q\x7F\xA6\x07\x0C%\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xFD[\x90P\x82\x148a\x0E\xC6V[\x91\x90\x91\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x82\t\x83\x82\x02\x91\x82\x80\x83\x10\x92\x03\x91\x80\x83\x03\x92\x14a\x10\x82Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x83\x10\x15a\x10KW\x94\x7F\xAC\xCB\x18\x16[\xD6\xFE1\xAE\x1C\xF3\x18\xDC[Q\xEE\xE0\xE1\xBAV\x9B\x88\xCDt\xC1w;\x91\xFA\xC1\x06i\x94\x95\t\x90\x82\x82\x11\x90\x03`\xEE\x1B\x91\x03`\x12\x1C\x17\x02\x90V[`D\x90\x86`@Q\x91\x7FQsd\x8D\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04\x83\x01R`$\x82\x01R\xFD[PPg\r\xE0\xB6\xB3\xA7d\0\0\x90\x04\x91PV[g\r\xE0\xB6\xB3\xA7d\0\0\x91\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x83\t\x92\x80\x83\x02\x92\x83\x80\x86\x10\x95\x03\x94\x80\x86\x03\x95\x14a\x11mW\x82\x85\x10\x15a\x111W\x90\x82\x91\t`\x01\x82\x19\x01\x82\x16\x80\x92\x04`\x02\x80\x82`\x03\x02\x18\x80\x83\x02\x82\x03\x02\x80\x83\x02\x82\x03\x02\x80\x83\x02\x82\x03\x02\x80\x83\x02\x82\x03\x02\x80\x83\x02\x82\x03\x02\x80\x92\x02\x90\x03\x02\x93`\x01\x83\x80`\0\x03\x04\x01\x90\x84\x83\x11\x90\x03\x02\x92\x03\x04\x17\x02\x90V[\x82`d\x92`@Q\x92\x7Fc\xA0Wx\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01R`$\x83\x01R`D\x82\x01R\xFD[PP\x80\x92P\x15a\x04\x06W\x04\x90V";
    /// The deployed bytecode of the contract.
    pub static SD59X18MATH_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct SD59x18Math<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for SD59x18Math<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for SD59x18Math<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for SD59x18Math<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for SD59x18Math<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(SD59x18Math))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> SD59x18Math<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    SD59X18MATH_ABI.clone(),
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
                SD59X18MATH_ABI.clone(),
                SD59X18MATH_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `div` (0x43509138) function
        pub fn div(
            &self,
            x: ::ethers::core::types::I256,
            y: ::ethers::core::types::I256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([67, 80, 145, 56], (x, y))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mul` (0xbbe93d91) function
        pub fn mul(
            &self,
            x: ::ethers::core::types::I256,
            y: ::ethers::core::types::I256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([187, 233, 61, 145], (x, y))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pow` (0x92b0c5b2) function
        pub fn pow(
            &self,
            x: ::ethers::core::types::I256,
            y: ::ethers::core::types::I256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([146, 176, 197, 178], (x, y))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for SD59x18Math<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `PRBMath_MulDiv18_Overflow` with signature `PRBMath_MulDiv18_Overflow(uint256,uint256)` and selector `0x5173648d`
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
        name = "PRBMath_MulDiv18_Overflow",
        abi = "PRBMath_MulDiv18_Overflow(uint256,uint256)"
    )]
    pub struct PRBMath_MulDiv18_Overflow {
        pub x: ::ethers::core::types::U256,
        pub y: ::ethers::core::types::U256,
    }
    ///Custom Error type `PRBMath_MulDiv_Overflow` with signature `PRBMath_MulDiv_Overflow(uint256,uint256,uint256)` and selector `0x63a05778`
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
        name = "PRBMath_MulDiv_Overflow",
        abi = "PRBMath_MulDiv_Overflow(uint256,uint256,uint256)"
    )]
    pub struct PRBMath_MulDiv_Overflow {
        pub x: ::ethers::core::types::U256,
        pub y: ::ethers::core::types::U256,
        pub denominator: ::ethers::core::types::U256,
    }
    ///Custom Error type `PRBMath_SD59x18_Div_InputTooSmall` with signature `PRBMath_SD59x18_Div_InputTooSmall()` and selector `0x9fe2b450`
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
        name = "PRBMath_SD59x18_Div_InputTooSmall",
        abi = "PRBMath_SD59x18_Div_InputTooSmall()"
    )]
    pub struct PRBMath_SD59x18_Div_InputTooSmall;
    ///Custom Error type `PRBMath_SD59x18_Div_Overflow` with signature `PRBMath_SD59x18_Div_Overflow(int256,int256)` and selector `0xd49c26b3`
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
        name = "PRBMath_SD59x18_Div_Overflow",
        abi = "PRBMath_SD59x18_Div_Overflow(int256,int256)"
    )]
    pub struct PRBMath_SD59x18_Div_Overflow {
        pub x: ::ethers::core::types::I256,
        pub y: ::ethers::core::types::I256,
    }
    ///Custom Error type `PRBMath_SD59x18_Exp2_InputTooBig` with signature `PRBMath_SD59x18_Exp2_InputTooBig(int256)` and selector `0x0360d028`
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
        name = "PRBMath_SD59x18_Exp2_InputTooBig",
        abi = "PRBMath_SD59x18_Exp2_InputTooBig(int256)"
    )]
    pub struct PRBMath_SD59x18_Exp2_InputTooBig {
        pub x: ::ethers::core::types::I256,
    }
    ///Custom Error type `PRBMath_SD59x18_Log_InputTooSmall` with signature `PRBMath_SD59x18_Log_InputTooSmall(int256)` and selector `0x059b101b`
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
        name = "PRBMath_SD59x18_Log_InputTooSmall",
        abi = "PRBMath_SD59x18_Log_InputTooSmall(int256)"
    )]
    pub struct PRBMath_SD59x18_Log_InputTooSmall {
        pub x: ::ethers::core::types::I256,
    }
    ///Custom Error type `PRBMath_SD59x18_Mul_InputTooSmall` with signature `PRBMath_SD59x18_Mul_InputTooSmall()` and selector `0xa6070c25`
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
        name = "PRBMath_SD59x18_Mul_InputTooSmall",
        abi = "PRBMath_SD59x18_Mul_InputTooSmall()"
    )]
    pub struct PRBMath_SD59x18_Mul_InputTooSmall;
    ///Custom Error type `PRBMath_SD59x18_Mul_Overflow` with signature `PRBMath_SD59x18_Mul_Overflow(int256,int256)` and selector `0x120b5b43`
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
        name = "PRBMath_SD59x18_Mul_Overflow",
        abi = "PRBMath_SD59x18_Mul_Overflow(int256,int256)"
    )]
    pub struct PRBMath_SD59x18_Mul_Overflow {
        pub x: ::ethers::core::types::I256,
        pub y: ::ethers::core::types::I256,
    }
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
    pub enum SD59x18MathErrors {
        PRBMath_MulDiv18_Overflow(PRBMath_MulDiv18_Overflow),
        PRBMath_MulDiv_Overflow(PRBMath_MulDiv_Overflow),
        PRBMath_SD59x18_Div_InputTooSmall(PRBMath_SD59x18_Div_InputTooSmall),
        PRBMath_SD59x18_Div_Overflow(PRBMath_SD59x18_Div_Overflow),
        PRBMath_SD59x18_Exp2_InputTooBig(PRBMath_SD59x18_Exp2_InputTooBig),
        PRBMath_SD59x18_Log_InputTooSmall(PRBMath_SD59x18_Log_InputTooSmall),
        PRBMath_SD59x18_Mul_InputTooSmall(PRBMath_SD59x18_Mul_InputTooSmall),
        PRBMath_SD59x18_Mul_Overflow(PRBMath_SD59x18_Mul_Overflow),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for SD59x18MathErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <PRBMath_MulDiv18_Overflow as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PRBMath_MulDiv18_Overflow(decoded));
            }
            if let Ok(decoded) = <PRBMath_MulDiv_Overflow as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PRBMath_MulDiv_Overflow(decoded));
            }
            if let Ok(decoded) = <PRBMath_SD59x18_Div_InputTooSmall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PRBMath_SD59x18_Div_InputTooSmall(decoded));
            }
            if let Ok(decoded) = <PRBMath_SD59x18_Div_Overflow as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PRBMath_SD59x18_Div_Overflow(decoded));
            }
            if let Ok(decoded) = <PRBMath_SD59x18_Exp2_InputTooBig as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PRBMath_SD59x18_Exp2_InputTooBig(decoded));
            }
            if let Ok(decoded) = <PRBMath_SD59x18_Log_InputTooSmall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PRBMath_SD59x18_Log_InputTooSmall(decoded));
            }
            if let Ok(decoded) = <PRBMath_SD59x18_Mul_InputTooSmall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PRBMath_SD59x18_Mul_InputTooSmall(decoded));
            }
            if let Ok(decoded) = <PRBMath_SD59x18_Mul_Overflow as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PRBMath_SD59x18_Mul_Overflow(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for SD59x18MathErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::PRBMath_MulDiv18_Overflow(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PRBMath_MulDiv_Overflow(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PRBMath_SD59x18_Div_InputTooSmall(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PRBMath_SD59x18_Div_Overflow(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PRBMath_SD59x18_Exp2_InputTooBig(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PRBMath_SD59x18_Log_InputTooSmall(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PRBMath_SD59x18_Mul_InputTooSmall(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PRBMath_SD59x18_Mul_Overflow(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for SD59x18MathErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <PRBMath_MulDiv18_Overflow as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <PRBMath_MulDiv_Overflow as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <PRBMath_SD59x18_Div_InputTooSmall as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <PRBMath_SD59x18_Div_Overflow as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <PRBMath_SD59x18_Exp2_InputTooBig as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <PRBMath_SD59x18_Log_InputTooSmall as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <PRBMath_SD59x18_Mul_InputTooSmall as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <PRBMath_SD59x18_Mul_Overflow as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for SD59x18MathErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::PRBMath_MulDiv18_Overflow(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PRBMath_MulDiv_Overflow(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PRBMath_SD59x18_Div_InputTooSmall(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PRBMath_SD59x18_Div_Overflow(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PRBMath_SD59x18_Exp2_InputTooBig(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PRBMath_SD59x18_Log_InputTooSmall(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PRBMath_SD59x18_Mul_InputTooSmall(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PRBMath_SD59x18_Mul_Overflow(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for SD59x18MathErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<PRBMath_MulDiv18_Overflow> for SD59x18MathErrors {
        fn from(value: PRBMath_MulDiv18_Overflow) -> Self {
            Self::PRBMath_MulDiv18_Overflow(value)
        }
    }
    impl ::core::convert::From<PRBMath_MulDiv_Overflow> for SD59x18MathErrors {
        fn from(value: PRBMath_MulDiv_Overflow) -> Self {
            Self::PRBMath_MulDiv_Overflow(value)
        }
    }
    impl ::core::convert::From<PRBMath_SD59x18_Div_InputTooSmall> for SD59x18MathErrors {
        fn from(value: PRBMath_SD59x18_Div_InputTooSmall) -> Self {
            Self::PRBMath_SD59x18_Div_InputTooSmall(value)
        }
    }
    impl ::core::convert::From<PRBMath_SD59x18_Div_Overflow> for SD59x18MathErrors {
        fn from(value: PRBMath_SD59x18_Div_Overflow) -> Self {
            Self::PRBMath_SD59x18_Div_Overflow(value)
        }
    }
    impl ::core::convert::From<PRBMath_SD59x18_Exp2_InputTooBig> for SD59x18MathErrors {
        fn from(value: PRBMath_SD59x18_Exp2_InputTooBig) -> Self {
            Self::PRBMath_SD59x18_Exp2_InputTooBig(value)
        }
    }
    impl ::core::convert::From<PRBMath_SD59x18_Log_InputTooSmall> for SD59x18MathErrors {
        fn from(value: PRBMath_SD59x18_Log_InputTooSmall) -> Self {
            Self::PRBMath_SD59x18_Log_InputTooSmall(value)
        }
    }
    impl ::core::convert::From<PRBMath_SD59x18_Mul_InputTooSmall> for SD59x18MathErrors {
        fn from(value: PRBMath_SD59x18_Mul_InputTooSmall) -> Self {
            Self::PRBMath_SD59x18_Mul_InputTooSmall(value)
        }
    }
    impl ::core::convert::From<PRBMath_SD59x18_Mul_Overflow> for SD59x18MathErrors {
        fn from(value: PRBMath_SD59x18_Mul_Overflow) -> Self {
            Self::PRBMath_SD59x18_Mul_Overflow(value)
        }
    }
    ///Container type for all input parameters for the `div` function with signature `div(int256,int256)` and selector `0x43509138`
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
    #[ethcall(name = "div", abi = "div(int256,int256)")]
    pub struct DivCall {
        pub x: ::ethers::core::types::I256,
        pub y: ::ethers::core::types::I256,
    }
    ///Container type for all input parameters for the `mul` function with signature `mul(int256,int256)` and selector `0xbbe93d91`
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
    #[ethcall(name = "mul", abi = "mul(int256,int256)")]
    pub struct MulCall {
        pub x: ::ethers::core::types::I256,
        pub y: ::ethers::core::types::I256,
    }
    ///Container type for all input parameters for the `pow` function with signature `pow(int256,int256)` and selector `0x92b0c5b2`
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
    #[ethcall(name = "pow", abi = "pow(int256,int256)")]
    pub struct PowCall {
        pub x: ::ethers::core::types::I256,
        pub y: ::ethers::core::types::I256,
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
    pub enum SD59x18MathCalls {
        Div(DivCall),
        Mul(MulCall),
        Pow(PowCall),
    }
    impl ::ethers::core::abi::AbiDecode for SD59x18MathCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <DivCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Div(decoded));
            }
            if let Ok(decoded) = <MulCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Mul(decoded));
            }
            if let Ok(decoded) = <PowCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Pow(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for SD59x18MathCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Div(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Mul(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Pow(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for SD59x18MathCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Div(element) => ::core::fmt::Display::fmt(element, f),
                Self::Mul(element) => ::core::fmt::Display::fmt(element, f),
                Self::Pow(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DivCall> for SD59x18MathCalls {
        fn from(value: DivCall) -> Self {
            Self::Div(value)
        }
    }
    impl ::core::convert::From<MulCall> for SD59x18MathCalls {
        fn from(value: MulCall) -> Self {
            Self::Mul(value)
        }
    }
    impl ::core::convert::From<PowCall> for SD59x18MathCalls {
        fn from(value: PowCall) -> Self {
            Self::Pow(value)
        }
    }
    ///Container type for all return fields from the `div` function with signature `div(int256,int256)` and selector `0x43509138`
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
    pub struct DivReturn {
        pub z: ::ethers::core::types::I256,
    }
    ///Container type for all return fields from the `mul` function with signature `mul(int256,int256)` and selector `0xbbe93d91`
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
    pub struct MulReturn {
        pub z: ::ethers::core::types::I256,
    }
    ///Container type for all return fields from the `pow` function with signature `pow(int256,int256)` and selector `0x92b0c5b2`
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
    pub struct PowReturn {
        pub z: ::ethers::core::types::I256,
    }
}
