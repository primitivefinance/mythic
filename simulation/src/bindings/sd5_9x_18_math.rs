pub use sd5_9x_18_math::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types
)]
pub mod sd5_9x_18_math {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("div"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
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
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("z"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("SD59x18"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("mul"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
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
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("z"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("SD59x18"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("pow"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
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
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("z"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("SD59x18"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("PRBMath_MulDiv18_Overflow"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("PRBMath_MulDiv18_Overflow",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("x"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("y"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PRBMath_MulDiv_Overflow"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("PRBMath_MulDiv_Overflow",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("x"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("y"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("denominator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PRBMath_SD59x18_Div_InputTooSmall"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("PRBMath_SD59x18_Div_InputTooSmall",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PRBMath_SD59x18_Div_Overflow"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("PRBMath_SD59x18_Div_Overflow",),
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
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PRBMath_SD59x18_Exp2_InputTooBig"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("PRBMath_SD59x18_Exp2_InputTooBig",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("x"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("SD59x18"),
                            ),
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PRBMath_SD59x18_Log_InputTooSmall"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("PRBMath_SD59x18_Log_InputTooSmall",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("x"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("SD59x18"),
                            ),
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PRBMath_SD59x18_Mul_InputTooSmall"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("PRBMath_SD59x18_Mul_InputTooSmall",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PRBMath_SD59x18_Mul_Overflow"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("PRBMath_SD59x18_Mul_Overflow",),
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
                    },],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    /// The parsed JSON ABI of the contract.
    pub static SD59X18MATH_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[Pa\x0F[\x80a\0m`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\0\x8EW`\x005`\xE0\x1C\x80cCP\x918\x14a\0\xF3W\x80c\x92\xB0\xC5\xB2\x14a\x01\x18W\x80c\xBB\xE9=\x91\x14a\x01+W[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x01\x06a\x01\x016`\x04a\x0E\xA2V[a\x01>V[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\x01\x06a\x01&6`\x04a\x0E\xA2V[a\x01SV[a\x01\x06a\x0196`\x04a\x0E\xA2V[a\x01_V[`\0a\x01J\x83\x83a\x01kV[\x90P[\x92\x91PPV[`\0a\x01J\x83\x83a\x02GV[`\0a\x01J\x83\x83a\x02\xE9V[`\0\x82\x82`\x01`\xFF\x1B\x82\x14\x80a\x01\x84WP`\x01`\xFF\x1B\x81\x14[\x15a\x01\xA2W`@Qc\t\xFE+E`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80`\0\x84\x12a\x01\xB3W\x83a\x01\xB8V[\x83`\0\x03[\x91P`\0\x83\x12a\x01\xC8W\x82a\x01\xCDV[\x82`\0\x03[\x90P`\0a\x01\xE4\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x03\x8DV[\x90P`\x01`\x01`\xFF\x1B\x03\x81\x11\x15a\x02\x1DW`@Qc\xD4\x9C&\xB3`\xE0\x1B\x81R`\x04\x81\x01\x89\x90R`$\x81\x01\x88\x90R`D\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x19\x85\x85\x18\x13a\x02:\x81a\x025W\x82`\0\x03a\x027V[\x82[\x90V[\x99\x98PPPPPPPPPV[`\0\x82\x82\x81\x83\x03a\x02rW\x80\x15a\x02_W`\0a\x02iV[g\r\xE0\xB6\xB3\xA7d\0\0[\x92PPPa\x01MV[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x03a\x02\x93Wg\r\xE0\xB6\xB3\xA7d\0\0\x92PPPa\x01MV[\x80`\0\x03a\x02\xADWg\r\xE0\xB6\xB3\xA7d\0\0\x92PPPa\x01MV[g\r\xE0\xB6\xB3\xA7d\0\0\x81\x03a\x02\xC6W\x84\x92PPPa\x01MV[a\x02\xE0a\x02\xDBa\x02\xD5\x87a\x04gV[\x86a\x02\xE9V[a\x05\xDAV[\x95\x94PPPPPV[`\0\x82\x82`\x01`\xFF\x1B\x82\x14\x80a\x03\x02WP`\x01`\xFF\x1B\x81\x14[\x15a\x03 W`@Qc\xA6\x07\x0C%`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80`\0\x84\x12a\x031W\x83a\x036V[\x83`\0\x03[\x91P`\0\x83\x12a\x03FW\x82a\x03KV[\x82`\0\x03[\x90P`\0a\x03Y\x83\x83a\x06\x8CV[\x90P`\x01`\x01`\xFF\x1B\x03\x81\x11\x15a\x02\x1DW`@Qc\x12\x0B[C`\xE0\x1B\x81R`\x04\x81\x01\x89\x90R`$\x81\x01\x88\x90R`D\x01a\x02\x14V[`\0\x80\x80`\0\x19\x85\x87\t\x85\x87\x02\x92P\x82\x81\x10\x83\x82\x03\x03\x91PP\x80`\0\x03a\x03\xC7W\x83\x82\x81a\x03\xBDWa\x03\xBDa\x0F\x0FV[\x04\x92PPPa\x04`V[\x83\x81\x10a\x03\xF8W`@Qc\x0Ct\n\xEF`\xE3\x1B\x81R`\x04\x81\x01\x87\x90R`$\x81\x01\x86\x90R`D\x81\x01\x85\x90R`d\x01a\x02\x14V[`\0\x84\x86\x88\t\x85\x19`\x01\x90\x81\x01\x87\x16\x96\x87\x90\x04\x96\x82\x86\x03\x81\x90\x04\x95\x90\x92\x11\x90\x93\x03`\0\x82\x90\x03\x91\x90\x91\x04\x90\x92\x01\x91\x90\x91\x02\x91\x90\x91\x17`\x03\x84\x02`\x02\x90\x81\x18\x80\x86\x02\x82\x03\x02\x80\x86\x02\x82\x03\x02\x80\x86\x02\x82\x03\x02\x80\x86\x02\x82\x03\x02\x80\x86\x02\x82\x03\x02\x80\x86\x02\x90\x91\x03\x02\x02\x91PP[\x93\x92PPPV[`\0\x81\x81\x81\x13a\x04\x8DW`@Qc\x05\x9B\x10\x1B`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x01a\x02\x14V[`\0g\r\xE0\xB6\xB3\xA7d\0\0\x82\x12a\x04\xA6WP`\x01a\x04\xCCV[P`\0\x19\x81n\xC0\x97\xCE{\xC9\x07\x15\xB3K\x9F\x10\0\0\0\0\x81a\x04\xC8Wa\x04\xC8a\x0F\x0FV[\x05\x91P[`\0a\x05Xg\r\xE0\xB6\xB3\xA7d\0\0\x84\x05`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x91\x82\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11`\x06\x1B\x90\x81\x1Cc\xFF\xFF\xFF\xFF\x81\x11`\x05\x1B\x90\x81\x1Ca\xFF\xFF\x81\x11`\x04\x1B\x90\x81\x1C`\xFF\x81\x11`\x03\x90\x81\x1B\x91\x82\x1C`\x0F\x81\x11`\x02\x1B\x90\x81\x1C\x91\x82\x11\x87\x1B\x91\x82\x1C\x96\x90\x96\x11\x94\x90\x96\x17\x92\x90\x92\x17\x17\x17\x91\x90\x91\x17\x91\x90\x91\x17\x17\x90V[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0\x81\x02\x83\x82\x1Dg\r\xE0\xB6\xB3\xA7c\xFF\xFF\x19\x81\x01a\x05\x85WP\x91\x90\x91\x02\x94\x93PPPPV[g\x1B\xC1mgN\xC8\0\0g\x06\xF0[Y\xD3\xB2\0\0[`\0\x81\x13\x15a\x05\xCBWg\r\xE0\xB6\xB3\xA7d\0\0\x83\x80\x02\x05\x92P\x81\x83\x12a\x05\xC3W\x92\x83\x01\x92`\x01\x92\x90\x92\x1D\x91[`\x01\x1Da\x05\x98V[PPP\x91\x90\x91\x02\x94\x93PPPPV[`\0\x81\x81\x81\x12\x15a\x06;Wh\x03=\xD1x\t\x14\xB9q\x14\x19\x81\x12\x15a\x06\0WP`\0\x92\x91PPV[a\x064a\x06\x13a\x027a\x02\xDB\x84`\0\x03\x90V[n\xC0\x97\xCE{\xC9\x07\x15\xB3K\x9F\x10\0\0\0\0\x81a\x060Wa\x060a\x0F\x0FV[\x05\x90V[\x91Pa\x06\x86V[h\nh\x89\x06\xBD\x8A\xFF\xFF\xFF\x81\x13\x15a\x06gW`@Qbl\x1A\x05`\xE3\x1B\x81R`\x04\x81\x01\x84\x90R`$\x01a\x02\x14V[g\r\xE0\xB6\xB3\xA7d\0\0`@\x82\x90\x1B\x05a\x06\x82a\x027\x82a\x07BV[\x92PP[P\x91\x90PV[`\0\x80\x80`\0\x19\x84\x86\t\x84\x86\x02\x92P\x82\x81\x10\x83\x82\x03\x03\x91PP\x80`\0\x03a\x06\xC0WPg\r\xE0\xB6\xB3\xA7d\0\0\x90\x04\x90Pa\x01MV[g\r\xE0\xB6\xB3\xA7d\0\0\x81\x10a\x06\xF2W`@QcQsd\x8D`\xE0\x1B\x81R`\x04\x81\x01\x86\x90R`$\x81\x01\x85\x90R`D\x01a\x02\x14V[`\0g\r\xE0\xB6\xB3\xA7d\0\0\x85\x87\tb\x04\0\0\x81\x85\x03\x04\x93\x10\x90\x91\x03`\x01`\xEE\x1B\x02\x91\x90\x91\x17\x7F\xAC\xCB\x18\x16[\xD6\xFE1\xAE\x1C\xF3\x18\xDC[Q\xEE\xE0\xE1\xBAV\x9B\x88\xCDt\xC1w;\x91\xFA\xC1\x06i\x02\x91PP\x92\x91PPV[`\x01`\xBF\x1Bg\xFF\0\0\0\0\0\0\0\x82\x16\x15a\x08OWg\x80\0\0\0\0\0\0\0\x82\x16\x15a\x07vWh\x01j\t\xE6g\xF3\xBC\xC9\t\x02`@\x1C[g@\0\0\0\0\0\0\0\x82\x16\x15a\x07\x95Wh\x010o\xE0\xA3\x1BqR\xDF\x02`@\x1C[g \0\0\0\0\0\0\0\x82\x16\x15a\x07\xB4Wh\x01\x17+\x83\xC7\xD5\x17\xAD\xCE\x02`@\x1C[g\x10\0\0\0\0\0\0\0\x82\x16\x15a\x07\xD3Wh\x01\x0BU\x86\xCF\x98\x90\xF6*\x02`@\x1C[g\x08\0\0\0\0\0\0\0\x82\x16\x15a\x07\xF2Wh\x01\x05\x9B\r1XWC\xAE\x02`@\x1C[g\x04\0\0\0\0\0\0\0\x82\x16\x15a\x08\x11Wh\x01\x02\xC9\xA3\xE7x\x06\x0E\xE7\x02`@\x1C[g\x02\0\0\0\0\0\0\0\x82\x16\x15a\x080Wh\x01\x01c\xDA\x9F\xB33V\xD8\x02`@\x1C[g\x01\0\0\0\0\0\0\0\x82\x16\x15a\x08OWh\x01\0\xB1\xAF\xA5\xAB\xCB\xEDa\x02`@\x1C[f\xFF\0\0\0\0\0\0\x82\x16\x15a\tNWf\x80\0\0\0\0\0\0\x82\x16\x15a\x08|Wh\x01\0X\xC8m\xA1\xC0\x9E\xA2\x02`@\x1C[f@\0\0\0\0\0\0\x82\x16\x15a\x08\x9AWh\x01\0,`^.\x8C\xECP\x02`@\x1C[f \0\0\0\0\0\0\x82\x16\x15a\x08\xB8Wh\x01\0\x16/9\x04\x05\x1F\xA1\x02`@\x1C[f\x10\0\0\0\0\0\0\x82\x16\x15a\x08\xD6Wh\x01\0\x0B\x17^\xFF\xDCv\xBA\x02`@\x1C[f\x08\0\0\0\0\0\0\x82\x16\x15a\x08\xF4Wh\x01\0\x05\x8B\xA0\x1F\xB9\xF9m\x02`@\x1C[f\x04\0\0\0\0\0\0\x82\x16\x15a\t\x12Wh\x01\0\x02\xC5\xCC7\xDA\x94\x92\x02`@\x1C[f\x02\0\0\0\0\0\0\x82\x16\x15a\t0Wh\x01\0\x01b\xE5%\xEE\x05G\x02`@\x1C[f\x01\0\0\0\0\0\0\x82\x16\x15a\tNWh\x01\0\0\xB1rUw\\\x04\x02`@\x1C[e\xFF\0\0\0\0\0\x82\x16\x15a\nDWe\x80\0\0\0\0\0\x82\x16\x15a\tyWh\x01\0\0X\xB9\x1B[\xC9\xAE\x02`@\x1C[e@\0\0\0\0\0\x82\x16\x15a\t\x96Wh\x01\0\0,\\\x89\xD5\xECm\x02`@\x1C[e \0\0\0\0\0\x82\x16\x15a\t\xB3Wh\x01\0\0\x16.C\xF4\xF81\x02`@\x1C[e\x10\0\0\0\0\0\x82\x16\x15a\t\xD0Wh\x01\0\0\x0B\x17!\xBC\xFC\x9A\x02`@\x1C[e\x08\0\0\0\0\0\x82\x16\x15a\t\xEDWh\x01\0\0\x05\x8B\x90\xCF\x1En\x02`@\x1C[e\x04\0\0\0\0\0\x82\x16\x15a\n\nWh\x01\0\0\x02\xC5\xC8c\xB7?\x02`@\x1C[e\x02\0\0\0\0\0\x82\x16\x15a\n'Wh\x01\0\0\x01b\xE40\xE5\xA2\x02`@\x1C[e\x01\0\0\0\0\0\x82\x16\x15a\nDWh\x01\0\0\0\xB1r\x185Q\x02`@\x1C[d\xFF\0\0\0\0\x82\x16\x15a\x0B1Wd\x80\0\0\0\0\x82\x16\x15a\nmWh\x01\0\0\0X\xB9\x0C\x0BI\x02`@\x1C[d@\0\0\0\0\x82\x16\x15a\n\x89Wh\x01\0\0\0,\\\x86\x01\xCC\x02`@\x1C[d \0\0\0\0\x82\x16\x15a\n\xA5Wh\x01\0\0\0\x16.B\xFF\xF0\x02`@\x1C[d\x10\0\0\0\0\x82\x16\x15a\n\xC1Wh\x01\0\0\0\x0B\x17!\x7F\xBB\x02`@\x1C[d\x08\0\0\0\0\x82\x16\x15a\n\xDDWh\x01\0\0\0\x05\x8B\x90\xBF\xCE\x02`@\x1C[d\x04\0\0\0\0\x82\x16\x15a\n\xF9Wh\x01\0\0\0\x02\xC5\xC8_\xE3\x02`@\x1C[d\x02\0\0\0\0\x82\x16\x15a\x0B\x15Wh\x01\0\0\0\x01b\xE4/\xF1\x02`@\x1C[d\x01\0\0\0\0\x82\x16\x15a\x0B1Wh\x01\0\0\0\0\xB1r\x17\xF8\x02`@\x1C[c\xFF\0\0\0\x82\x16\x15a\x0C\x15Wc\x80\0\0\0\x82\x16\x15a\x0BXWh\x01\0\0\0\0X\xB9\x0B\xFC\x02`@\x1C[c@\0\0\0\x82\x16\x15a\x0BsWh\x01\0\0\0\0,\\\x85\xFE\x02`@\x1C[c \0\0\0\x82\x16\x15a\x0B\x8EWh\x01\0\0\0\0\x16.B\xFF\x02`@\x1C[c\x10\0\0\0\x82\x16\x15a\x0B\xA9Wh\x01\0\0\0\0\x0B\x17!\x7F\x02`@\x1C[c\x08\0\0\0\x82\x16\x15a\x0B\xC4Wh\x01\0\0\0\0\x05\x8B\x90\xC0\x02`@\x1C[c\x04\0\0\0\x82\x16\x15a\x0B\xDFWh\x01\0\0\0\0\x02\xC5\xC8`\x02`@\x1C[c\x02\0\0\0\x82\x16\x15a\x0B\xFAWh\x01\0\0\0\0\x01b\xE40\x02`@\x1C[c\x01\0\0\0\x82\x16\x15a\x0C\x15Wh\x01\0\0\0\0\0\xB1r\x18\x02`@\x1C[b\xFF\0\0\x82\x16\x15a\x0C\xF0Wb\x80\0\0\x82\x16\x15a\x0C:Wh\x01\0\0\0\0\0X\xB9\x0C\x02`@\x1C[b@\0\0\x82\x16\x15a\x0CTWh\x01\0\0\0\0\0,\\\x86\x02`@\x1C[b \0\0\x82\x16\x15a\x0CnWh\x01\0\0\0\0\0\x16.C\x02`@\x1C[b\x10\0\0\x82\x16\x15a\x0C\x88Wh\x01\0\0\0\0\0\x0B\x17!\x02`@\x1C[b\x08\0\0\x82\x16\x15a\x0C\xA2Wh\x01\0\0\0\0\0\x05\x8B\x91\x02`@\x1C[b\x04\0\0\x82\x16\x15a\x0C\xBCWh\x01\0\0\0\0\0\x02\xC5\xC8\x02`@\x1C[b\x02\0\0\x82\x16\x15a\x0C\xD6Wh\x01\0\0\0\0\0\x01b\xE4\x02`@\x1C[b\x01\0\0\x82\x16\x15a\x0C\xF0Wh\x01\0\0\0\0\0\0\xB1r\x02`@\x1C[a\xFF\0\x82\x16\x15a\r\xC2Wa\x80\0\x82\x16\x15a\r\x13Wh\x01\0\0\0\0\0\0X\xB9\x02`@\x1C[a@\0\x82\x16\x15a\r,Wh\x01\0\0\0\0\0\0,]\x02`@\x1C[a \0\x82\x16\x15a\rEWh\x01\0\0\0\0\0\0\x16.\x02`@\x1C[a\x10\0\x82\x16\x15a\r^Wh\x01\0\0\0\0\0\0\x0B\x17\x02`@\x1C[a\x08\0\x82\x16\x15a\rwWh\x01\0\0\0\0\0\0\x05\x8C\x02`@\x1C[a\x04\0\x82\x16\x15a\r\x90Wh\x01\0\0\0\0\0\0\x02\xC6\x02`@\x1C[a\x02\0\x82\x16\x15a\r\xA9Wh\x01\0\0\0\0\0\0\x01c\x02`@\x1C[a\x01\0\x82\x16\x15a\r\xC2Wh\x01\0\0\0\0\0\0\0\xB1\x02`@\x1C[`\xFF\x82\x16\x15a\x0E\x8BW`\x80\x82\x16\x15a\r\xE3Wh\x01\0\0\0\0\0\0\0Y\x02`@\x1C[`@\x82\x16\x15a\r\xFBWh\x01\0\0\0\0\0\0\0,\x02`@\x1C[` \x82\x16\x15a\x0E\x13Wh\x01\0\0\0\0\0\0\0\x16\x02`@\x1C[`\x10\x82\x16\x15a\x0E+Wh\x01\0\0\0\0\0\0\0\x0B\x02`@\x1C[`\x08\x82\x16\x15a\x0ECWh\x01\0\0\0\0\0\0\0\x06\x02`@\x1C[`\x04\x82\x16\x15a\x0E[Wh\x01\0\0\0\0\0\0\0\x03\x02`@\x1C[`\x02\x82\x16\x15a\x0EsWh\x01\0\0\0\0\0\0\0\x01\x02`@\x1C[`\x01\x82\x16\x15a\x0E\x8BWh\x01\0\0\0\0\0\0\0\x01\x02`@\x1C[g\r\xE0\xB6\xB3\xA7d\0\0\x02`@\x91\x90\x91\x1C`\xBF\x03\x1C\x90V[`\0\x80`@\x83\x85\x03\x12\x15a\x0F\0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[PP\x805\x92` \x90\x91\x015\x91PV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD\xFE\xA2dipfsX\"\x12 \xEE\xE0\xC6(!\xFF\xA1\xCA:\xE2\xA8Z\xFE\xBEpj\xDEyH\x96]\xB8v\x9E\xA1Km\xF1\xE7\x8A\x0E\x98dsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static SD59X18MATH_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\0\x8EW`\x005`\xE0\x1C\x80cCP\x918\x14a\0\xF3W\x80c\x92\xB0\xC5\xB2\x14a\x01\x18W\x80c\xBB\xE9=\x91\x14a\x01+W[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x01\x06a\x01\x016`\x04a\x0E\xA2V[a\x01>V[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\x01\x06a\x01&6`\x04a\x0E\xA2V[a\x01SV[a\x01\x06a\x0196`\x04a\x0E\xA2V[a\x01_V[`\0a\x01J\x83\x83a\x01kV[\x90P[\x92\x91PPV[`\0a\x01J\x83\x83a\x02GV[`\0a\x01J\x83\x83a\x02\xE9V[`\0\x82\x82`\x01`\xFF\x1B\x82\x14\x80a\x01\x84WP`\x01`\xFF\x1B\x81\x14[\x15a\x01\xA2W`@Qc\t\xFE+E`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80`\0\x84\x12a\x01\xB3W\x83a\x01\xB8V[\x83`\0\x03[\x91P`\0\x83\x12a\x01\xC8W\x82a\x01\xCDV[\x82`\0\x03[\x90P`\0a\x01\xE4\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x03\x8DV[\x90P`\x01`\x01`\xFF\x1B\x03\x81\x11\x15a\x02\x1DW`@Qc\xD4\x9C&\xB3`\xE0\x1B\x81R`\x04\x81\x01\x89\x90R`$\x81\x01\x88\x90R`D\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x19\x85\x85\x18\x13a\x02:\x81a\x025W\x82`\0\x03a\x027V[\x82[\x90V[\x99\x98PPPPPPPPPV[`\0\x82\x82\x81\x83\x03a\x02rW\x80\x15a\x02_W`\0a\x02iV[g\r\xE0\xB6\xB3\xA7d\0\0[\x92PPPa\x01MV[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x03a\x02\x93Wg\r\xE0\xB6\xB3\xA7d\0\0\x92PPPa\x01MV[\x80`\0\x03a\x02\xADWg\r\xE0\xB6\xB3\xA7d\0\0\x92PPPa\x01MV[g\r\xE0\xB6\xB3\xA7d\0\0\x81\x03a\x02\xC6W\x84\x92PPPa\x01MV[a\x02\xE0a\x02\xDBa\x02\xD5\x87a\x04gV[\x86a\x02\xE9V[a\x05\xDAV[\x95\x94PPPPPV[`\0\x82\x82`\x01`\xFF\x1B\x82\x14\x80a\x03\x02WP`\x01`\xFF\x1B\x81\x14[\x15a\x03 W`@Qc\xA6\x07\x0C%`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80`\0\x84\x12a\x031W\x83a\x036V[\x83`\0\x03[\x91P`\0\x83\x12a\x03FW\x82a\x03KV[\x82`\0\x03[\x90P`\0a\x03Y\x83\x83a\x06\x8CV[\x90P`\x01`\x01`\xFF\x1B\x03\x81\x11\x15a\x02\x1DW`@Qc\x12\x0B[C`\xE0\x1B\x81R`\x04\x81\x01\x89\x90R`$\x81\x01\x88\x90R`D\x01a\x02\x14V[`\0\x80\x80`\0\x19\x85\x87\t\x85\x87\x02\x92P\x82\x81\x10\x83\x82\x03\x03\x91PP\x80`\0\x03a\x03\xC7W\x83\x82\x81a\x03\xBDWa\x03\xBDa\x0F\x0FV[\x04\x92PPPa\x04`V[\x83\x81\x10a\x03\xF8W`@Qc\x0Ct\n\xEF`\xE3\x1B\x81R`\x04\x81\x01\x87\x90R`$\x81\x01\x86\x90R`D\x81\x01\x85\x90R`d\x01a\x02\x14V[`\0\x84\x86\x88\t\x85\x19`\x01\x90\x81\x01\x87\x16\x96\x87\x90\x04\x96\x82\x86\x03\x81\x90\x04\x95\x90\x92\x11\x90\x93\x03`\0\x82\x90\x03\x91\x90\x91\x04\x90\x92\x01\x91\x90\x91\x02\x91\x90\x91\x17`\x03\x84\x02`\x02\x90\x81\x18\x80\x86\x02\x82\x03\x02\x80\x86\x02\x82\x03\x02\x80\x86\x02\x82\x03\x02\x80\x86\x02\x82\x03\x02\x80\x86\x02\x82\x03\x02\x80\x86\x02\x90\x91\x03\x02\x02\x91PP[\x93\x92PPPV[`\0\x81\x81\x81\x13a\x04\x8DW`@Qc\x05\x9B\x10\x1B`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x01a\x02\x14V[`\0g\r\xE0\xB6\xB3\xA7d\0\0\x82\x12a\x04\xA6WP`\x01a\x04\xCCV[P`\0\x19\x81n\xC0\x97\xCE{\xC9\x07\x15\xB3K\x9F\x10\0\0\0\0\x81a\x04\xC8Wa\x04\xC8a\x0F\x0FV[\x05\x91P[`\0a\x05Xg\r\xE0\xB6\xB3\xA7d\0\0\x84\x05`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x91\x82\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11`\x06\x1B\x90\x81\x1Cc\xFF\xFF\xFF\xFF\x81\x11`\x05\x1B\x90\x81\x1Ca\xFF\xFF\x81\x11`\x04\x1B\x90\x81\x1C`\xFF\x81\x11`\x03\x90\x81\x1B\x91\x82\x1C`\x0F\x81\x11`\x02\x1B\x90\x81\x1C\x91\x82\x11\x87\x1B\x91\x82\x1C\x96\x90\x96\x11\x94\x90\x96\x17\x92\x90\x92\x17\x17\x17\x91\x90\x91\x17\x91\x90\x91\x17\x17\x90V[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0\x81\x02\x83\x82\x1Dg\r\xE0\xB6\xB3\xA7c\xFF\xFF\x19\x81\x01a\x05\x85WP\x91\x90\x91\x02\x94\x93PPPPV[g\x1B\xC1mgN\xC8\0\0g\x06\xF0[Y\xD3\xB2\0\0[`\0\x81\x13\x15a\x05\xCBWg\r\xE0\xB6\xB3\xA7d\0\0\x83\x80\x02\x05\x92P\x81\x83\x12a\x05\xC3W\x92\x83\x01\x92`\x01\x92\x90\x92\x1D\x91[`\x01\x1Da\x05\x98V[PPP\x91\x90\x91\x02\x94\x93PPPPV[`\0\x81\x81\x81\x12\x15a\x06;Wh\x03=\xD1x\t\x14\xB9q\x14\x19\x81\x12\x15a\x06\0WP`\0\x92\x91PPV[a\x064a\x06\x13a\x027a\x02\xDB\x84`\0\x03\x90V[n\xC0\x97\xCE{\xC9\x07\x15\xB3K\x9F\x10\0\0\0\0\x81a\x060Wa\x060a\x0F\x0FV[\x05\x90V[\x91Pa\x06\x86V[h\nh\x89\x06\xBD\x8A\xFF\xFF\xFF\x81\x13\x15a\x06gW`@Qbl\x1A\x05`\xE3\x1B\x81R`\x04\x81\x01\x84\x90R`$\x01a\x02\x14V[g\r\xE0\xB6\xB3\xA7d\0\0`@\x82\x90\x1B\x05a\x06\x82a\x027\x82a\x07BV[\x92PP[P\x91\x90PV[`\0\x80\x80`\0\x19\x84\x86\t\x84\x86\x02\x92P\x82\x81\x10\x83\x82\x03\x03\x91PP\x80`\0\x03a\x06\xC0WPg\r\xE0\xB6\xB3\xA7d\0\0\x90\x04\x90Pa\x01MV[g\r\xE0\xB6\xB3\xA7d\0\0\x81\x10a\x06\xF2W`@QcQsd\x8D`\xE0\x1B\x81R`\x04\x81\x01\x86\x90R`$\x81\x01\x85\x90R`D\x01a\x02\x14V[`\0g\r\xE0\xB6\xB3\xA7d\0\0\x85\x87\tb\x04\0\0\x81\x85\x03\x04\x93\x10\x90\x91\x03`\x01`\xEE\x1B\x02\x91\x90\x91\x17\x7F\xAC\xCB\x18\x16[\xD6\xFE1\xAE\x1C\xF3\x18\xDC[Q\xEE\xE0\xE1\xBAV\x9B\x88\xCDt\xC1w;\x91\xFA\xC1\x06i\x02\x91PP\x92\x91PPV[`\x01`\xBF\x1Bg\xFF\0\0\0\0\0\0\0\x82\x16\x15a\x08OWg\x80\0\0\0\0\0\0\0\x82\x16\x15a\x07vWh\x01j\t\xE6g\xF3\xBC\xC9\t\x02`@\x1C[g@\0\0\0\0\0\0\0\x82\x16\x15a\x07\x95Wh\x010o\xE0\xA3\x1BqR\xDF\x02`@\x1C[g \0\0\0\0\0\0\0\x82\x16\x15a\x07\xB4Wh\x01\x17+\x83\xC7\xD5\x17\xAD\xCE\x02`@\x1C[g\x10\0\0\0\0\0\0\0\x82\x16\x15a\x07\xD3Wh\x01\x0BU\x86\xCF\x98\x90\xF6*\x02`@\x1C[g\x08\0\0\0\0\0\0\0\x82\x16\x15a\x07\xF2Wh\x01\x05\x9B\r1XWC\xAE\x02`@\x1C[g\x04\0\0\0\0\0\0\0\x82\x16\x15a\x08\x11Wh\x01\x02\xC9\xA3\xE7x\x06\x0E\xE7\x02`@\x1C[g\x02\0\0\0\0\0\0\0\x82\x16\x15a\x080Wh\x01\x01c\xDA\x9F\xB33V\xD8\x02`@\x1C[g\x01\0\0\0\0\0\0\0\x82\x16\x15a\x08OWh\x01\0\xB1\xAF\xA5\xAB\xCB\xEDa\x02`@\x1C[f\xFF\0\0\0\0\0\0\x82\x16\x15a\tNWf\x80\0\0\0\0\0\0\x82\x16\x15a\x08|Wh\x01\0X\xC8m\xA1\xC0\x9E\xA2\x02`@\x1C[f@\0\0\0\0\0\0\x82\x16\x15a\x08\x9AWh\x01\0,`^.\x8C\xECP\x02`@\x1C[f \0\0\0\0\0\0\x82\x16\x15a\x08\xB8Wh\x01\0\x16/9\x04\x05\x1F\xA1\x02`@\x1C[f\x10\0\0\0\0\0\0\x82\x16\x15a\x08\xD6Wh\x01\0\x0B\x17^\xFF\xDCv\xBA\x02`@\x1C[f\x08\0\0\0\0\0\0\x82\x16\x15a\x08\xF4Wh\x01\0\x05\x8B\xA0\x1F\xB9\xF9m\x02`@\x1C[f\x04\0\0\0\0\0\0\x82\x16\x15a\t\x12Wh\x01\0\x02\xC5\xCC7\xDA\x94\x92\x02`@\x1C[f\x02\0\0\0\0\0\0\x82\x16\x15a\t0Wh\x01\0\x01b\xE5%\xEE\x05G\x02`@\x1C[f\x01\0\0\0\0\0\0\x82\x16\x15a\tNWh\x01\0\0\xB1rUw\\\x04\x02`@\x1C[e\xFF\0\0\0\0\0\x82\x16\x15a\nDWe\x80\0\0\0\0\0\x82\x16\x15a\tyWh\x01\0\0X\xB9\x1B[\xC9\xAE\x02`@\x1C[e@\0\0\0\0\0\x82\x16\x15a\t\x96Wh\x01\0\0,\\\x89\xD5\xECm\x02`@\x1C[e \0\0\0\0\0\x82\x16\x15a\t\xB3Wh\x01\0\0\x16.C\xF4\xF81\x02`@\x1C[e\x10\0\0\0\0\0\x82\x16\x15a\t\xD0Wh\x01\0\0\x0B\x17!\xBC\xFC\x9A\x02`@\x1C[e\x08\0\0\0\0\0\x82\x16\x15a\t\xEDWh\x01\0\0\x05\x8B\x90\xCF\x1En\x02`@\x1C[e\x04\0\0\0\0\0\x82\x16\x15a\n\nWh\x01\0\0\x02\xC5\xC8c\xB7?\x02`@\x1C[e\x02\0\0\0\0\0\x82\x16\x15a\n'Wh\x01\0\0\x01b\xE40\xE5\xA2\x02`@\x1C[e\x01\0\0\0\0\0\x82\x16\x15a\nDWh\x01\0\0\0\xB1r\x185Q\x02`@\x1C[d\xFF\0\0\0\0\x82\x16\x15a\x0B1Wd\x80\0\0\0\0\x82\x16\x15a\nmWh\x01\0\0\0X\xB9\x0C\x0BI\x02`@\x1C[d@\0\0\0\0\x82\x16\x15a\n\x89Wh\x01\0\0\0,\\\x86\x01\xCC\x02`@\x1C[d \0\0\0\0\x82\x16\x15a\n\xA5Wh\x01\0\0\0\x16.B\xFF\xF0\x02`@\x1C[d\x10\0\0\0\0\x82\x16\x15a\n\xC1Wh\x01\0\0\0\x0B\x17!\x7F\xBB\x02`@\x1C[d\x08\0\0\0\0\x82\x16\x15a\n\xDDWh\x01\0\0\0\x05\x8B\x90\xBF\xCE\x02`@\x1C[d\x04\0\0\0\0\x82\x16\x15a\n\xF9Wh\x01\0\0\0\x02\xC5\xC8_\xE3\x02`@\x1C[d\x02\0\0\0\0\x82\x16\x15a\x0B\x15Wh\x01\0\0\0\x01b\xE4/\xF1\x02`@\x1C[d\x01\0\0\0\0\x82\x16\x15a\x0B1Wh\x01\0\0\0\0\xB1r\x17\xF8\x02`@\x1C[c\xFF\0\0\0\x82\x16\x15a\x0C\x15Wc\x80\0\0\0\x82\x16\x15a\x0BXWh\x01\0\0\0\0X\xB9\x0B\xFC\x02`@\x1C[c@\0\0\0\x82\x16\x15a\x0BsWh\x01\0\0\0\0,\\\x85\xFE\x02`@\x1C[c \0\0\0\x82\x16\x15a\x0B\x8EWh\x01\0\0\0\0\x16.B\xFF\x02`@\x1C[c\x10\0\0\0\x82\x16\x15a\x0B\xA9Wh\x01\0\0\0\0\x0B\x17!\x7F\x02`@\x1C[c\x08\0\0\0\x82\x16\x15a\x0B\xC4Wh\x01\0\0\0\0\x05\x8B\x90\xC0\x02`@\x1C[c\x04\0\0\0\x82\x16\x15a\x0B\xDFWh\x01\0\0\0\0\x02\xC5\xC8`\x02`@\x1C[c\x02\0\0\0\x82\x16\x15a\x0B\xFAWh\x01\0\0\0\0\x01b\xE40\x02`@\x1C[c\x01\0\0\0\x82\x16\x15a\x0C\x15Wh\x01\0\0\0\0\0\xB1r\x18\x02`@\x1C[b\xFF\0\0\x82\x16\x15a\x0C\xF0Wb\x80\0\0\x82\x16\x15a\x0C:Wh\x01\0\0\0\0\0X\xB9\x0C\x02`@\x1C[b@\0\0\x82\x16\x15a\x0CTWh\x01\0\0\0\0\0,\\\x86\x02`@\x1C[b \0\0\x82\x16\x15a\x0CnWh\x01\0\0\0\0\0\x16.C\x02`@\x1C[b\x10\0\0\x82\x16\x15a\x0C\x88Wh\x01\0\0\0\0\0\x0B\x17!\x02`@\x1C[b\x08\0\0\x82\x16\x15a\x0C\xA2Wh\x01\0\0\0\0\0\x05\x8B\x91\x02`@\x1C[b\x04\0\0\x82\x16\x15a\x0C\xBCWh\x01\0\0\0\0\0\x02\xC5\xC8\x02`@\x1C[b\x02\0\0\x82\x16\x15a\x0C\xD6Wh\x01\0\0\0\0\0\x01b\xE4\x02`@\x1C[b\x01\0\0\x82\x16\x15a\x0C\xF0Wh\x01\0\0\0\0\0\0\xB1r\x02`@\x1C[a\xFF\0\x82\x16\x15a\r\xC2Wa\x80\0\x82\x16\x15a\r\x13Wh\x01\0\0\0\0\0\0X\xB9\x02`@\x1C[a@\0\x82\x16\x15a\r,Wh\x01\0\0\0\0\0\0,]\x02`@\x1C[a \0\x82\x16\x15a\rEWh\x01\0\0\0\0\0\0\x16.\x02`@\x1C[a\x10\0\x82\x16\x15a\r^Wh\x01\0\0\0\0\0\0\x0B\x17\x02`@\x1C[a\x08\0\x82\x16\x15a\rwWh\x01\0\0\0\0\0\0\x05\x8C\x02`@\x1C[a\x04\0\x82\x16\x15a\r\x90Wh\x01\0\0\0\0\0\0\x02\xC6\x02`@\x1C[a\x02\0\x82\x16\x15a\r\xA9Wh\x01\0\0\0\0\0\0\x01c\x02`@\x1C[a\x01\0\x82\x16\x15a\r\xC2Wh\x01\0\0\0\0\0\0\0\xB1\x02`@\x1C[`\xFF\x82\x16\x15a\x0E\x8BW`\x80\x82\x16\x15a\r\xE3Wh\x01\0\0\0\0\0\0\0Y\x02`@\x1C[`@\x82\x16\x15a\r\xFBWh\x01\0\0\0\0\0\0\0,\x02`@\x1C[` \x82\x16\x15a\x0E\x13Wh\x01\0\0\0\0\0\0\0\x16\x02`@\x1C[`\x10\x82\x16\x15a\x0E+Wh\x01\0\0\0\0\0\0\0\x0B\x02`@\x1C[`\x08\x82\x16\x15a\x0ECWh\x01\0\0\0\0\0\0\0\x06\x02`@\x1C[`\x04\x82\x16\x15a\x0E[Wh\x01\0\0\0\0\0\0\0\x03\x02`@\x1C[`\x02\x82\x16\x15a\x0EsWh\x01\0\0\0\0\0\0\0\x01\x02`@\x1C[`\x01\x82\x16\x15a\x0E\x8BWh\x01\0\0\0\0\0\0\0\x01\x02`@\x1C[g\r\xE0\xB6\xB3\xA7d\0\0\x02`@\x91\x90\x91\x1C`\xBF\x03\x1C\x90V[`\0\x80`@\x83\x85\x03\x12\x15a\x0F\0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[PP\x805\x92` \x90\x91\x015\x91PV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD\xFE\xA2dipfsX\"\x12 \xEE\xE0\xC6(!\xFF\xA1\xCA:\xE2\xA8Z\xFE\xBEpj\xDEyH\x96]\xB8v\x9E\xA1Km\xF1\xE7\x8A\x0E\x98dsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static SD59X18MATH_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
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
        /// Creates a new contract instance with the specified `ethers` client
        /// at `address`. The contract derefs to a `ethers::Contract`
        /// object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                SD59X18MATH_ABI.clone(),
                client,
            ))
        }
        /// Constructs the general purpose `Deployer` instance based on the
        /// provided constructor arguments and sends it. Returns a new
        /// instance of a deployer that returns an instance of this contract
        /// after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the
        ///   argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract
        /// instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the
        /// `greeter.json` artifact.
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
        /// Calls the contract's `div` (0x43509138) function
        pub fn div(
            &self,
            x: ::ethers::core::types::I256,
            y: ::ethers::core::types::I256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([67, 80, 145, 56], (x, y))
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `mul` (0xbbe93d91) function
        pub fn mul(
            &self,
            x: ::ethers::core::types::I256,
            y: ::ethers::core::types::I256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([187, 233, 61, 145], (x, y))
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `pow` (0x92b0c5b2) function
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
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for SD59x18Math<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    /// Custom Error type `PRBMath_MulDiv18_Overflow` with signature
    /// `PRBMath_MulDiv18_Overflow(uint256,uint256)` and selector `0x5173648d`
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
        Hash,
    )]
    #[etherror(
        name = "PRBMath_MulDiv18_Overflow",
        abi = "PRBMath_MulDiv18_Overflow(uint256,uint256)"
    )]
    pub struct PRBMath_MulDiv18_Overflow {
        pub x: ::ethers::core::types::U256,
        pub y: ::ethers::core::types::U256,
    }
    /// Custom Error type `PRBMath_MulDiv_Overflow` with signature
    /// `PRBMath_MulDiv_Overflow(uint256,uint256,uint256)` and selector
    /// `0x63a05778`
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
        Hash,
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
    /// Custom Error type `PRBMath_SD59x18_Div_InputTooSmall` with signature
    /// `PRBMath_SD59x18_Div_InputTooSmall()` and selector `0x9fe2b450`
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
        Hash,
    )]
    #[etherror(
        name = "PRBMath_SD59x18_Div_InputTooSmall",
        abi = "PRBMath_SD59x18_Div_InputTooSmall()"
    )]
    pub struct PRBMath_SD59x18_Div_InputTooSmall;
    /// Custom Error type `PRBMath_SD59x18_Div_Overflow` with signature
    /// `PRBMath_SD59x18_Div_Overflow(int256,int256)` and selector `0xd49c26b3`
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
        Hash,
    )]
    #[etherror(
        name = "PRBMath_SD59x18_Div_Overflow",
        abi = "PRBMath_SD59x18_Div_Overflow(int256,int256)"
    )]
    pub struct PRBMath_SD59x18_Div_Overflow {
        pub x: ::ethers::core::types::I256,
        pub y: ::ethers::core::types::I256,
    }
    /// Custom Error type `PRBMath_SD59x18_Exp2_InputTooBig` with signature
    /// `PRBMath_SD59x18_Exp2_InputTooBig(int256)` and selector `0x0360d028`
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
        Hash,
    )]
    #[etherror(
        name = "PRBMath_SD59x18_Exp2_InputTooBig",
        abi = "PRBMath_SD59x18_Exp2_InputTooBig(int256)"
    )]
    pub struct PRBMath_SD59x18_Exp2_InputTooBig {
        pub x: ::ethers::core::types::I256,
    }
    /// Custom Error type `PRBMath_SD59x18_Log_InputTooSmall` with signature
    /// `PRBMath_SD59x18_Log_InputTooSmall(int256)` and selector `0x059b101b`
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
        Hash,
    )]
    #[etherror(
        name = "PRBMath_SD59x18_Log_InputTooSmall",
        abi = "PRBMath_SD59x18_Log_InputTooSmall(int256)"
    )]
    pub struct PRBMath_SD59x18_Log_InputTooSmall {
        pub x: ::ethers::core::types::I256,
    }
    /// Custom Error type `PRBMath_SD59x18_Mul_InputTooSmall` with signature
    /// `PRBMath_SD59x18_Mul_InputTooSmall()` and selector `0xa6070c25`
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
        Hash,
    )]
    #[etherror(
        name = "PRBMath_SD59x18_Mul_InputTooSmall",
        abi = "PRBMath_SD59x18_Mul_InputTooSmall()"
    )]
    pub struct PRBMath_SD59x18_Mul_InputTooSmall;
    /// Custom Error type `PRBMath_SD59x18_Mul_Overflow` with signature
    /// `PRBMath_SD59x18_Mul_Overflow(int256,int256)` and selector `0x120b5b43`
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
        Hash,
    )]
    #[etherror(
        name = "PRBMath_SD59x18_Mul_Overflow",
        abi = "PRBMath_SD59x18_Mul_Overflow(int256,int256)"
    )]
    pub struct PRBMath_SD59x18_Mul_Overflow {
        pub x: ::ethers::core::types::I256,
        pub y: ::ethers::core::types::I256,
    }
    /// Container type for all of the contract's custom errors
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash,
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
            if let Ok(decoded) =
                <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) =
                <PRBMath_MulDiv18_Overflow as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PRBMath_MulDiv18_Overflow(decoded));
            }
            if let Ok(decoded) =
                <PRBMath_MulDiv_Overflow as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PRBMath_MulDiv_Overflow(decoded));
            }
            if let Ok(decoded) =
                <PRBMath_SD59x18_Div_InputTooSmall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PRBMath_SD59x18_Div_InputTooSmall(decoded));
            }
            if let Ok(decoded) =
                <PRBMath_SD59x18_Div_Overflow as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PRBMath_SD59x18_Div_Overflow(decoded));
            }
            if let Ok(decoded) =
                <PRBMath_SD59x18_Exp2_InputTooBig as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PRBMath_SD59x18_Exp2_InputTooBig(decoded));
            }
            if let Ok(decoded) =
                <PRBMath_SD59x18_Log_InputTooSmall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PRBMath_SD59x18_Log_InputTooSmall(decoded));
            }
            if let Ok(decoded) =
                <PRBMath_SD59x18_Mul_InputTooSmall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PRBMath_SD59x18_Mul_InputTooSmall(decoded));
            }
            if let Ok(decoded) =
                <PRBMath_SD59x18_Mul_Overflow as ::ethers::core::abi::AbiDecode>::decode(data)
            {
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
                Self::PRBMath_MulDiv18_Overflow(element) => ::core::fmt::Display::fmt(element, f),
                Self::PRBMath_MulDiv_Overflow(element) => ::core::fmt::Display::fmt(element, f),
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
    /// Container type for all input parameters for the `div` function with
    /// signature `div(int256,int256)` and selector `0x43509138`
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
        Hash,
    )]
    #[ethcall(name = "div", abi = "div(int256,int256)")]
    pub struct DivCall {
        pub x: ::ethers::core::types::I256,
        pub y: ::ethers::core::types::I256,
    }
    /// Container type for all input parameters for the `mul` function with
    /// signature `mul(int256,int256)` and selector `0xbbe93d91`
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
        Hash,
    )]
    #[ethcall(name = "mul", abi = "mul(int256,int256)")]
    pub struct MulCall {
        pub x: ::ethers::core::types::I256,
        pub y: ::ethers::core::types::I256,
    }
    /// Container type for all input parameters for the `pow` function with
    /// signature `pow(int256,int256)` and selector `0x92b0c5b2`
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
        Hash,
    )]
    #[ethcall(name = "pow", abi = "pow(int256,int256)")]
    pub struct PowCall {
        pub x: ::ethers::core::types::I256,
        pub y: ::ethers::core::types::I256,
    }
    /// Container type for all of the contract's call
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash,
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
            if let Ok(decoded) = <DivCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Div(decoded));
            }
            if let Ok(decoded) = <MulCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Mul(decoded));
            }
            if let Ok(decoded) = <PowCall as ::ethers::core::abi::AbiDecode>::decode(data) {
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
    /// Container type for all return fields from the `div` function with
    /// signature `div(int256,int256)` and selector `0x43509138`
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
        Hash,
    )]
    pub struct DivReturn {
        pub z: ::ethers::core::types::I256,
    }
    /// Container type for all return fields from the `mul` function with
    /// signature `mul(int256,int256)` and selector `0xbbe93d91`
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
        Hash,
    )]
    pub struct MulReturn {
        pub z: ::ethers::core::types::I256,
    }
    /// Container type for all return fields from the `pow` function with
    /// signature `pow(int256,int256)` and selector `0x92b0c5b2`
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
        Hash,
    )]
    pub struct PowReturn {
        pub z: ::ethers::core::types::I256,
    }
}
