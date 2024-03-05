pub use arb_math::*;
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
pub mod arb_math {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("cdf"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("cdf"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("input"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("output"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("divWadDown"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("divWadDown"),
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("z"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("divWadUp"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("divWadUp"),
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("z"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("log"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("x"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("z"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("mulWadDown"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("mulWadDown"),
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("z"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("mulWadUp"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("mulWadUp"),
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("z"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("pdf"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("pdf"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("input"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("output"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
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
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("y"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("z"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ppf"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("ppf"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("input"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("output"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("sqrt"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("sqrt"),
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
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("z"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
                    ::std::borrow::ToOwned::to_owned("Infinity"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("Infinity"),
                            inputs: ::std::vec![],
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
                (
                    ::std::borrow::ToOwned::to_owned("NegativeInfinity"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("NegativeInfinity"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OutOfBounds"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("OutOfBounds"),
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
    pub static ARBMATH_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[Pa\x10K\x80a\0m`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\0\xEBW`\x005`\xE0\x1C\x80c\xAE\x97h\xA8\x11a\0\xB3W\x80c\xAE\x97h\xA8\x14a\x01\xC1W\x80c\xBD%-\x06\x14a\x01\xD4W\x80c\xD0\xB7\x1B\x1E\x14a\x01\xE7W\x80c\xD2L\xE6\xE5\x14a\x01\xFAW\x80c\xE5$\xF8I\x14a\x02\rWa\0\xEBV[\x80c-[l\xB9\x14a\x01PW\x80c6yr:\x14a\x01uW\x80c7\xC6\xA4J\x14a\x01\x88W\x80cgsB\xCE\x14a\x01\x9BW\x80c\x92\xB0\xC5\xB2\x14a\x01\xAEW[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x01ca\x01^6`\x04a\x0E\xE7V[a\x02 V[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\x01ca\x01\x836`\x04a\x0E\xE7V[a\x021V[a\x01ca\x01\x966`\x04a\x0F\x03V[a\x02<V[a\x01ca\x01\xA96`\x04a\x0E\xE7V[a\x02OV[a\x01ca\x01\xBC6`\x04a\x0F\x03V[a\x02ZV[a\x01ca\x01\xCF6`\x04a\x0F\x03V[a\x02fV[a\x01ca\x01\xE26`\x04a\x0F\x03V[a\x02rV[a\x01ca\x01\xF56`\x04a\x0E\xE7V[a\x02~V[a\x01ca\x02\x086`\x04a\x0E\xE7V[a\x02\x89V[a\x01ca\x02\x1B6`\x04a\x0F\x03V[a\x02\x94V[`\0a\x02+\x82a\x02\xA0V[\x92\x91PPV[`\0a\x02+\x82a\x04\x80V[`\0a\x02H\x83\x83a\x05&V[\x93\x92PPPV[`\0a\x02+\x82a\x05;V[`\0a\x02H\x83\x83a\x05\xDFV[`\0a\x02H\x83\x83a\x06\x10V[`\0a\x02H\x83\x83a\x06%V[`\0a\x02+\x82a\x06:V[`\0a\x02+\x82a\x06\xA3V[`\0a\x02H\x83\x83a\x06\xFFV[`\0\x80\x82\x13a\x02\xE2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0``a\x02\xEF\x84a\x07\x14V[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[`\0g\x06\xF0[Y\xD3\xB2\0\0\x82\x03a\x04\x99WP`\0\x91\x90PV[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x12a\x04\xC1W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\x04\xE2W`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x04\xEF\x83`\x02a\x0F>V[\x90P`\0a\x04\xFC\x82a\x07\xBCV[\x90P`\0a\x05\x12g\x13\xA0K\xBD\xFD\xC9\xBE\x88\x83a\n=V[\x90Pa\x05\x1D\x81a\x0FnV[\x95\x94PPPPPV[`\0a\x02H\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\nRV[`\xB5\x81`\x01`\x88\x1B\x81\x10a\x05TW`@\x91\x90\x91\x1B\x90`\x80\x1C[i\x01\0\0\0\0\0\0\0\0\0\x81\x10a\x05pW` \x91\x90\x91\x1B\x90`@\x1C[e\x01\0\0\0\0\0\x81\x10a\x05\x88W`\x10\x91\x90\x91\x1B\x90` \x1C[c\x01\0\0\0\x81\x10a\x05\x9EW`\x08\x91\x90\x91\x1B\x90`\x10\x1C[b\x01\0\0\x01\x02`\x12\x1C\x80\x82\x04\x01`\x01\x90\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x90\x81\x90\x04\x81\x11\x90\x03\x90V[`\0a\x02Hg\r\xE0\xB6\xB3\xA7d\0\0\x83a\x05\xF7\x86a\x02\xA0V[a\x06\x01\x91\x90a\x0F>V[a\x06\x0B\x91\x90a\x0F\x8AV[a\nqV[`\0a\x02H\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x0C\x1AV[`\0a\x02H\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x0C\x1AV[`\0\x80g\x13\xA0K\xBD\xFD\xC9\xBE\x88a\x06Xg\r\xE0\xB6\xB3\xA7d\0\0\x85a\x0F>V[a\x06b\x91\x90a\x0F\x8AV[\x90P`\0a\x06o\x82a\x0FnV[\x90P`\0a\x06|\x82a\x0CHV[\x90Pg\x1B\xC1mgN\xC8\0\0a\x06\x99g\r\xE0\xB6\xB3\xA7d\0\0\x83a\x0F>V[a\x05\x1D\x91\x90a\x0F\x8AV[`\0\x80g\x1B\xC1mgN\xC8\0\0\x83a\x06\xB9\x81a\x0FnV[a\x06\xC3\x91\x90a\x0F>V[a\x06\xCD\x91\x90a\x0F\x8AV[\x90Pa\x06\xD8\x81a\nqV[\x90Pg\"\xC9U\"\x95T\xC1\xB6a\x06\xF5g\r\xE0\xB6\xB3\xA7d\0\0\x83a\x0F>V[a\x02H\x91\x90a\x0F\x8AV[`\0a\x02H\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\nRV[`\0\x80\x82\x11a\x07QW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x02\xD9V[P`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x82\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[`\0\x80\x82\x12\x80a\x07\xD3WPg\x1B\xC1mgN\xC8\0\0\x82\x13[\x15a\x07\xF1W`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\x08\x12W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81g\x1B\xC1mgN\xC8\0\0\x03a\x08:W`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x15a\x08EW\x91\x90PV[`\0g\r\xE0\xB6\xB3\xA7d\0\0\x83\x12a\x08mWa\x08h\x83g\x1B\xC1mgN\xC8\0\0a\x0F\xC6V[a\x08oV[\x82[\x90P`\0a\x08\x85\x82g\x1B\xC1mgN\xC8\0\0a\x0E,V[\x90P\x80`\0\x03a\x08\xA8W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x08\xB3\x82a\x02\xA0V[\x90P`\0c;\x9A\xCA\0a\x08\xDEa\x08\xD9a\x08\xD3g\x1B\xC1mgN\xC8\0\0a\x0FnV[\x85a\n=V[a\x05;V[a\x08\xE8\x91\x90a\x0F>V[\x90P`\0\x80a\x08\xFF\x83g\x03\xC1f\\z\xAB \0a\n=V[a\t\x11\x90g \x05\xFEO&\x8E\xA0\0a\x0F\xEDV[\x90P`\0a\tA\x84a\t*\x86f\x9F2u$b\xA0\0a\n=V[a\t<\x90g\r\xC5R\x7Fd, \0a\x0F\xEDV[a\n=V[a\tS\x90g\r\xE0\xB6\xB3\xA7d\0\0a\x0F\xEDV[\x90Pa\twg\t\xD0(\xCCo _\xFF\x19\x85a\tm\x85\x85a\x0E,V[a\t<\x91\x90a\x0F\xC6V[\x92PPP`\0[`\x02\x81\x10\x15a\n\x12W`\0\x86a\t\x93\x84a\x0CHV[a\t\x9D\x91\x90a\x0F\xC6V[\x90P`\0a\t\xAB\x84\x85a\n=V[a\t\xB4\x90a\x0FnV[\x90P`\0a\t\xC1\x82a\nqV[\x90P`\0a\t\xCF\x86\x85a\n=V[a\t\xE1g\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x84a\n=V[a\t\xEB\x91\x90a\x0F\xC6V[\x90Pa\t\xF7\x84\x82a\x0E,V[a\n\x01\x90\x87a\x0F\xEDV[\x95P\x84`\x01\x01\x94PPPPPa\t~V[g\r\xE0\xB6\xB3\xA7d\0\0\x88\x12a\n/Wa\n*\x82a\x0FnV[a\n1V[\x81[\x98\x97PPPPPPPPV[`\0a\x02H\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x0E=V[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\njW`\0\x80\xFD[\x04\x92\x91PPV[`\0h\x02H\xCE6\xA7\x0C\xB2k>\x19\x82\x13a\n\x8CWP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a\n\xD3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x01a\x02\xD9V[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05`\x01`_\x1B\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1Dm\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1Dn\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x0C2W`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[`\0\x81`\0\x03a\x0CaWPg\r\xE0\xB6\xB3\xA7d\0\0\x91\x90PV[gV\x98\xEE\xF0fp\0\0\x82\x12a\x0CxWP`\0\x91\x90PV[a\x0C\x89gV\x98\xEE\xF0fp\0\0a\x0FnV[\x82\x13a\x0C\x9EWPg\x1B\xC1mgN\xC8\0\0\x91\x90PV[`\0a\x0C\xA9\x83a\x0E\\V[\x90P`\0a\x0C\xE2g\r\xE0\xB6\xB3\xA7d\0\0a\x0C\xCB\x84g\x1B\xC1mgN\xC8\0\0a\x05&V[a\x0C\xDD\x90g\r\xE0\xB6\xB3\xA7d\0\0a\x0F\xEDV[a\x0E,V[\x90P`\0\x80\x82a\r>\x81a\r+\x81a\r\x19\x81a\r\x06\x81g\x02_\x0F\xE1\x05\xA3\x14\0a\n=V[a\t<\x90g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19a\x0F\xEDV[a\t<\x90g\x14\xA8EL\x19\xE1\xAC\0a\x0F\xEDV[a\t<\x90g\x0F\xC1\x0E\x01W\x82w\xFF\x19a\x0F\xEDV[a\rP\x90g\x03\xDE\xBD\x08;\x8C|\0a\x0F\xEDV[\x91P\x83\x90Pa\r\xB8\x81a\r\xA6\x81a\r\x94\x81a\r\x82\x81a\ro\x81\x8Ba\n=V[a\t<\x90g\x02\x95\xD4\0\xEA2W\xFF\x19a\x0F\xEDV[a\t<\x90g\x01W\xD8\xB2\xEC\xC7\x08\0a\x0F\xEDV[a\t<\x90g\x051\n\xA7\xD5!0\0a\x0F\xEDV[a\t<\x90g\r\xE0\xCC=\x15a\0\0a\x0F\xEDV[\x91P\x81g\x11\x90\0\xAB\x10\x0F\xFC\0a\r\xCE\x87\x88a\n=V[a\r\xDA\x90`\0\x19a\x0F>V[a\r\xE4\x91\x90a\x0F\xC6V[a\r\xEE\x91\x90a\x0F\xEDV[\x92PP`\0a\r\xFC\x83a\nqV[\x90P`\0a\x0E\n\x85\x83a\n=V[\x90P`\0\x88\x12a\x0E\x1AW\x80a\n1V[a\n1\x81g\x1B\xC1mgN\xC8\0\0a\x0F\xC6V[`\0a\x02H\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x05\x85\x14\x17\x16a\x0EUW`\0\x80\xFD[\x05\x92\x91PPV[`\0`\x01`\xFF\x1B\x82\x03a\x0E\x82W`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x12\x15a\x0E\x93WP\x19`\x01\x01\x90V[P\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`\0` \x82\x84\x03\x12\x15a\x0E\xFCWa\x0E\xFCa\x0E\x97V[P5\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x0F\x19Wa\x0F\x19a\x0E\x97V[PP\x805\x92` \x90\x91\x015\x91PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15a\x0FZWa\x0FZa\x0F(V[\x81\x81\x05\x83\x14\x82\x15\x17a\x02+Wa\x02+a\x0F(V[`\0`\x01`\xFF\x1B\x82\x01a\x0F\x83Wa\x0F\x83a\x0F(V[P`\0\x03\x90V[`\0\x82a\x0F\xA7WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\x01`\xFF\x1B\x82\x14`\0\x19\x84\x14\x16\x15a\x0F\xC1Wa\x0F\xC1a\x0F(V[P\x05\x90V[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a\x0F\xE6Wa\x0F\xE6a\x0F(V[P\x92\x91PPV[\x80\x82\x01\x82\x81\x12`\0\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a\x10\rWa\x10\ra\x0F(V[PP\x92\x91PPV\xFE\xA2dipfsX\"\x12 \x93\xDC\xF6\x91a.Oq\x1C*\xFF~\xB1\t&\xAF\xC7\x98\xB1S\x01;g\xC8\xCC\xA7\x0B\xE2?W<=dsolcC\0\x08\x16\x003";
    /// The bytecode of the contract.
    pub static ARBMATH_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\0\xEBW`\x005`\xE0\x1C\x80c\xAE\x97h\xA8\x11a\0\xB3W\x80c\xAE\x97h\xA8\x14a\x01\xC1W\x80c\xBD%-\x06\x14a\x01\xD4W\x80c\xD0\xB7\x1B\x1E\x14a\x01\xE7W\x80c\xD2L\xE6\xE5\x14a\x01\xFAW\x80c\xE5$\xF8I\x14a\x02\rWa\0\xEBV[\x80c-[l\xB9\x14a\x01PW\x80c6yr:\x14a\x01uW\x80c7\xC6\xA4J\x14a\x01\x88W\x80cgsB\xCE\x14a\x01\x9BW\x80c\x92\xB0\xC5\xB2\x14a\x01\xAEW[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x01ca\x01^6`\x04a\x0E\xE7V[a\x02 V[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\x01ca\x01\x836`\x04a\x0E\xE7V[a\x021V[a\x01ca\x01\x966`\x04a\x0F\x03V[a\x02<V[a\x01ca\x01\xA96`\x04a\x0E\xE7V[a\x02OV[a\x01ca\x01\xBC6`\x04a\x0F\x03V[a\x02ZV[a\x01ca\x01\xCF6`\x04a\x0F\x03V[a\x02fV[a\x01ca\x01\xE26`\x04a\x0F\x03V[a\x02rV[a\x01ca\x01\xF56`\x04a\x0E\xE7V[a\x02~V[a\x01ca\x02\x086`\x04a\x0E\xE7V[a\x02\x89V[a\x01ca\x02\x1B6`\x04a\x0F\x03V[a\x02\x94V[`\0a\x02+\x82a\x02\xA0V[\x92\x91PPV[`\0a\x02+\x82a\x04\x80V[`\0a\x02H\x83\x83a\x05&V[\x93\x92PPPV[`\0a\x02+\x82a\x05;V[`\0a\x02H\x83\x83a\x05\xDFV[`\0a\x02H\x83\x83a\x06\x10V[`\0a\x02H\x83\x83a\x06%V[`\0a\x02+\x82a\x06:V[`\0a\x02+\x82a\x06\xA3V[`\0a\x02H\x83\x83a\x06\xFFV[`\0\x80\x82\x13a\x02\xE2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0``a\x02\xEF\x84a\x07\x14V[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[`\0g\x06\xF0[Y\xD3\xB2\0\0\x82\x03a\x04\x99WP`\0\x91\x90PV[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x12a\x04\xC1W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\x04\xE2W`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x04\xEF\x83`\x02a\x0F>V[\x90P`\0a\x04\xFC\x82a\x07\xBCV[\x90P`\0a\x05\x12g\x13\xA0K\xBD\xFD\xC9\xBE\x88\x83a\n=V[\x90Pa\x05\x1D\x81a\x0FnV[\x95\x94PPPPPV[`\0a\x02H\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\nRV[`\xB5\x81`\x01`\x88\x1B\x81\x10a\x05TW`@\x91\x90\x91\x1B\x90`\x80\x1C[i\x01\0\0\0\0\0\0\0\0\0\x81\x10a\x05pW` \x91\x90\x91\x1B\x90`@\x1C[e\x01\0\0\0\0\0\x81\x10a\x05\x88W`\x10\x91\x90\x91\x1B\x90` \x1C[c\x01\0\0\0\x81\x10a\x05\x9EW`\x08\x91\x90\x91\x1B\x90`\x10\x1C[b\x01\0\0\x01\x02`\x12\x1C\x80\x82\x04\x01`\x01\x90\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x90\x81\x90\x04\x81\x11\x90\x03\x90V[`\0a\x02Hg\r\xE0\xB6\xB3\xA7d\0\0\x83a\x05\xF7\x86a\x02\xA0V[a\x06\x01\x91\x90a\x0F>V[a\x06\x0B\x91\x90a\x0F\x8AV[a\nqV[`\0a\x02H\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x0C\x1AV[`\0a\x02H\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x0C\x1AV[`\0\x80g\x13\xA0K\xBD\xFD\xC9\xBE\x88a\x06Xg\r\xE0\xB6\xB3\xA7d\0\0\x85a\x0F>V[a\x06b\x91\x90a\x0F\x8AV[\x90P`\0a\x06o\x82a\x0FnV[\x90P`\0a\x06|\x82a\x0CHV[\x90Pg\x1B\xC1mgN\xC8\0\0a\x06\x99g\r\xE0\xB6\xB3\xA7d\0\0\x83a\x0F>V[a\x05\x1D\x91\x90a\x0F\x8AV[`\0\x80g\x1B\xC1mgN\xC8\0\0\x83a\x06\xB9\x81a\x0FnV[a\x06\xC3\x91\x90a\x0F>V[a\x06\xCD\x91\x90a\x0F\x8AV[\x90Pa\x06\xD8\x81a\nqV[\x90Pg\"\xC9U\"\x95T\xC1\xB6a\x06\xF5g\r\xE0\xB6\xB3\xA7d\0\0\x83a\x0F>V[a\x02H\x91\x90a\x0F\x8AV[`\0a\x02H\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\nRV[`\0\x80\x82\x11a\x07QW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x02\xD9V[P`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x82\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[`\0\x80\x82\x12\x80a\x07\xD3WPg\x1B\xC1mgN\xC8\0\0\x82\x13[\x15a\x07\xF1W`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\x08\x12W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81g\x1B\xC1mgN\xC8\0\0\x03a\x08:W`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x15a\x08EW\x91\x90PV[`\0g\r\xE0\xB6\xB3\xA7d\0\0\x83\x12a\x08mWa\x08h\x83g\x1B\xC1mgN\xC8\0\0a\x0F\xC6V[a\x08oV[\x82[\x90P`\0a\x08\x85\x82g\x1B\xC1mgN\xC8\0\0a\x0E,V[\x90P\x80`\0\x03a\x08\xA8W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x08\xB3\x82a\x02\xA0V[\x90P`\0c;\x9A\xCA\0a\x08\xDEa\x08\xD9a\x08\xD3g\x1B\xC1mgN\xC8\0\0a\x0FnV[\x85a\n=V[a\x05;V[a\x08\xE8\x91\x90a\x0F>V[\x90P`\0\x80a\x08\xFF\x83g\x03\xC1f\\z\xAB \0a\n=V[a\t\x11\x90g \x05\xFEO&\x8E\xA0\0a\x0F\xEDV[\x90P`\0a\tA\x84a\t*\x86f\x9F2u$b\xA0\0a\n=V[a\t<\x90g\r\xC5R\x7Fd, \0a\x0F\xEDV[a\n=V[a\tS\x90g\r\xE0\xB6\xB3\xA7d\0\0a\x0F\xEDV[\x90Pa\twg\t\xD0(\xCCo _\xFF\x19\x85a\tm\x85\x85a\x0E,V[a\t<\x91\x90a\x0F\xC6V[\x92PPP`\0[`\x02\x81\x10\x15a\n\x12W`\0\x86a\t\x93\x84a\x0CHV[a\t\x9D\x91\x90a\x0F\xC6V[\x90P`\0a\t\xAB\x84\x85a\n=V[a\t\xB4\x90a\x0FnV[\x90P`\0a\t\xC1\x82a\nqV[\x90P`\0a\t\xCF\x86\x85a\n=V[a\t\xE1g\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x84a\n=V[a\t\xEB\x91\x90a\x0F\xC6V[\x90Pa\t\xF7\x84\x82a\x0E,V[a\n\x01\x90\x87a\x0F\xEDV[\x95P\x84`\x01\x01\x94PPPPPa\t~V[g\r\xE0\xB6\xB3\xA7d\0\0\x88\x12a\n/Wa\n*\x82a\x0FnV[a\n1V[\x81[\x98\x97PPPPPPPPV[`\0a\x02H\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x0E=V[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\njW`\0\x80\xFD[\x04\x92\x91PPV[`\0h\x02H\xCE6\xA7\x0C\xB2k>\x19\x82\x13a\n\x8CWP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a\n\xD3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x01a\x02\xD9V[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05`\x01`_\x1B\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1Dm\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1Dn\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x0C2W`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[`\0\x81`\0\x03a\x0CaWPg\r\xE0\xB6\xB3\xA7d\0\0\x91\x90PV[gV\x98\xEE\xF0fp\0\0\x82\x12a\x0CxWP`\0\x91\x90PV[a\x0C\x89gV\x98\xEE\xF0fp\0\0a\x0FnV[\x82\x13a\x0C\x9EWPg\x1B\xC1mgN\xC8\0\0\x91\x90PV[`\0a\x0C\xA9\x83a\x0E\\V[\x90P`\0a\x0C\xE2g\r\xE0\xB6\xB3\xA7d\0\0a\x0C\xCB\x84g\x1B\xC1mgN\xC8\0\0a\x05&V[a\x0C\xDD\x90g\r\xE0\xB6\xB3\xA7d\0\0a\x0F\xEDV[a\x0E,V[\x90P`\0\x80\x82a\r>\x81a\r+\x81a\r\x19\x81a\r\x06\x81g\x02_\x0F\xE1\x05\xA3\x14\0a\n=V[a\t<\x90g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19a\x0F\xEDV[a\t<\x90g\x14\xA8EL\x19\xE1\xAC\0a\x0F\xEDV[a\t<\x90g\x0F\xC1\x0E\x01W\x82w\xFF\x19a\x0F\xEDV[a\rP\x90g\x03\xDE\xBD\x08;\x8C|\0a\x0F\xEDV[\x91P\x83\x90Pa\r\xB8\x81a\r\xA6\x81a\r\x94\x81a\r\x82\x81a\ro\x81\x8Ba\n=V[a\t<\x90g\x02\x95\xD4\0\xEA2W\xFF\x19a\x0F\xEDV[a\t<\x90g\x01W\xD8\xB2\xEC\xC7\x08\0a\x0F\xEDV[a\t<\x90g\x051\n\xA7\xD5!0\0a\x0F\xEDV[a\t<\x90g\r\xE0\xCC=\x15a\0\0a\x0F\xEDV[\x91P\x81g\x11\x90\0\xAB\x10\x0F\xFC\0a\r\xCE\x87\x88a\n=V[a\r\xDA\x90`\0\x19a\x0F>V[a\r\xE4\x91\x90a\x0F\xC6V[a\r\xEE\x91\x90a\x0F\xEDV[\x92PP`\0a\r\xFC\x83a\nqV[\x90P`\0a\x0E\n\x85\x83a\n=V[\x90P`\0\x88\x12a\x0E\x1AW\x80a\n1V[a\n1\x81g\x1B\xC1mgN\xC8\0\0a\x0F\xC6V[`\0a\x02H\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x05\x85\x14\x17\x16a\x0EUW`\0\x80\xFD[\x05\x92\x91PPV[`\0`\x01`\xFF\x1B\x82\x03a\x0E\x82W`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x12\x15a\x0E\x93WP\x19`\x01\x01\x90V[P\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`\0` \x82\x84\x03\x12\x15a\x0E\xFCWa\x0E\xFCa\x0E\x97V[P5\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x0F\x19Wa\x0F\x19a\x0E\x97V[PP\x805\x92` \x90\x91\x015\x91PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15a\x0FZWa\x0FZa\x0F(V[\x81\x81\x05\x83\x14\x82\x15\x17a\x02+Wa\x02+a\x0F(V[`\0`\x01`\xFF\x1B\x82\x01a\x0F\x83Wa\x0F\x83a\x0F(V[P`\0\x03\x90V[`\0\x82a\x0F\xA7WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\x01`\xFF\x1B\x82\x14`\0\x19\x84\x14\x16\x15a\x0F\xC1Wa\x0F\xC1a\x0F(V[P\x05\x90V[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a\x0F\xE6Wa\x0F\xE6a\x0F(V[P\x92\x91PPV[\x80\x82\x01\x82\x81\x12`\0\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a\x10\rWa\x10\ra\x0F(V[PP\x92\x91PPV\xFE\xA2dipfsX\"\x12 \x93\xDC\xF6\x91a.Oq\x1C*\xFF~\xB1\t&\xAF\xC7\x98\xB1S\x01;g\xC8\xCC\xA7\x0B\xE2?W<=dsolcC\0\x08\x16\x003";
    /// The deployed bytecode of the contract.
    pub static ARBMATH_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct ArbMath<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ArbMath<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ArbMath<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ArbMath<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ArbMath<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(ArbMath)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ArbMath<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    ARBMATH_ABI.clone(),
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
                ARBMATH_ABI.clone(),
                ARBMATH_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `cdf` (0xd0b71b1e) function
        pub fn cdf(
            &self,
            input: ::ethers::core::types::I256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([208, 183, 27, 30], input)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `divWadDown` (0x37c6a44a) function
        pub fn div_wad_down(
            &self,
            x: ::ethers::core::types::U256,
            y: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([55, 198, 164, 74], (x, y))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `divWadUp` (0xbd252d06) function
        pub fn div_wad_up(
            &self,
            x: ::ethers::core::types::U256,
            y: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([189, 37, 45, 6], (x, y))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x2d5b6cb9) function
        pub fn log(
            &self,
            x: ::ethers::core::types::I256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([45, 91, 108, 185], x)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mulWadDown` (0xe524f849) function
        pub fn mul_wad_down(
            &self,
            x: ::ethers::core::types::U256,
            y: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([229, 36, 248, 73], (x, y))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mulWadUp` (0xae9768a8) function
        pub fn mul_wad_up(
            &self,
            x: ::ethers::core::types::U256,
            y: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([174, 151, 104, 168], (x, y))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pdf` (0xd24ce6e5) function
        pub fn pdf(
            &self,
            input: ::ethers::core::types::I256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([210, 76, 230, 229], input)
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
        ///Calls the contract's `ppf` (0x3679723a) function
        pub fn ppf(
            &self,
            input: ::ethers::core::types::I256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([54, 121, 114, 58], input)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sqrt` (0x677342ce) function
        pub fn sqrt(
            &self,
            x: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([103, 115, 66, 206], x)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for ArbMath<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `Infinity` with signature `Infinity()` and selector `0x07a02127`
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
    #[etherror(name = "Infinity", abi = "Infinity()")]
    pub struct Infinity;
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
    ///Custom Error type `NegativeInfinity` with signature `NegativeInfinity()` and selector `0x8bb56614`
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
    #[etherror(name = "NegativeInfinity", abi = "NegativeInfinity()")]
    pub struct NegativeInfinity;
    ///Custom Error type `OutOfBounds` with signature `OutOfBounds()` and selector `0xb4120f14`
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
    #[etherror(name = "OutOfBounds", abi = "OutOfBounds()")]
    pub struct OutOfBounds;
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
    pub enum ArbMathErrors {
        Infinity(Infinity),
        Min(Min),
        NegativeInfinity(NegativeInfinity),
        OutOfBounds(OutOfBounds),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for ArbMathErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <Infinity as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Infinity(decoded));
            }
            if let Ok(decoded) = <Min as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Min(decoded));
            }
            if let Ok(decoded) = <NegativeInfinity as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NegativeInfinity(decoded));
            }
            if let Ok(decoded) = <OutOfBounds as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OutOfBounds(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ArbMathErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::Infinity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Min(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NegativeInfinity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OutOfBounds(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for ArbMathErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <Infinity as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <Min as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NegativeInfinity as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OutOfBounds as ::ethers::contract::EthError>::selector() => true,
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for ArbMathErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Infinity(element) => ::core::fmt::Display::fmt(element, f),
                Self::Min(element) => ::core::fmt::Display::fmt(element, f),
                Self::NegativeInfinity(element) => ::core::fmt::Display::fmt(element, f),
                Self::OutOfBounds(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for ArbMathErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<Infinity> for ArbMathErrors {
        fn from(value: Infinity) -> Self {
            Self::Infinity(value)
        }
    }
    impl ::core::convert::From<Min> for ArbMathErrors {
        fn from(value: Min) -> Self {
            Self::Min(value)
        }
    }
    impl ::core::convert::From<NegativeInfinity> for ArbMathErrors {
        fn from(value: NegativeInfinity) -> Self {
            Self::NegativeInfinity(value)
        }
    }
    impl ::core::convert::From<OutOfBounds> for ArbMathErrors {
        fn from(value: OutOfBounds) -> Self {
            Self::OutOfBounds(value)
        }
    }
    ///Container type for all input parameters for the `cdf` function with signature `cdf(int256)` and selector `0xd0b71b1e`
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
    #[ethcall(name = "cdf", abi = "cdf(int256)")]
    pub struct CdfCall {
        pub input: ::ethers::core::types::I256,
    }
    ///Container type for all input parameters for the `divWadDown` function with signature `divWadDown(uint256,uint256)` and selector `0x37c6a44a`
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
    #[ethcall(name = "divWadDown", abi = "divWadDown(uint256,uint256)")]
    pub struct DivWadDownCall {
        pub x: ::ethers::core::types::U256,
        pub y: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `divWadUp` function with signature `divWadUp(uint256,uint256)` and selector `0xbd252d06`
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
    #[ethcall(name = "divWadUp", abi = "divWadUp(uint256,uint256)")]
    pub struct DivWadUpCall {
        pub x: ::ethers::core::types::U256,
        pub y: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(int256)` and selector `0x2d5b6cb9`
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
    #[ethcall(name = "log", abi = "log(int256)")]
    pub struct LogCall {
        pub x: ::ethers::core::types::I256,
    }
    ///Container type for all input parameters for the `mulWadDown` function with signature `mulWadDown(uint256,uint256)` and selector `0xe524f849`
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
    #[ethcall(name = "mulWadDown", abi = "mulWadDown(uint256,uint256)")]
    pub struct MulWadDownCall {
        pub x: ::ethers::core::types::U256,
        pub y: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `mulWadUp` function with signature `mulWadUp(uint256,uint256)` and selector `0xae9768a8`
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
    #[ethcall(name = "mulWadUp", abi = "mulWadUp(uint256,uint256)")]
    pub struct MulWadUpCall {
        pub x: ::ethers::core::types::U256,
        pub y: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `pdf` function with signature `pdf(int256)` and selector `0xd24ce6e5`
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
    #[ethcall(name = "pdf", abi = "pdf(int256)")]
    pub struct PdfCall {
        pub input: ::ethers::core::types::I256,
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
    ///Container type for all input parameters for the `ppf` function with signature `ppf(int256)` and selector `0x3679723a`
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
    #[ethcall(name = "ppf", abi = "ppf(int256)")]
    pub struct PpfCall {
        pub input: ::ethers::core::types::I256,
    }
    ///Container type for all input parameters for the `sqrt` function with signature `sqrt(uint256)` and selector `0x677342ce`
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
    #[ethcall(name = "sqrt", abi = "sqrt(uint256)")]
    pub struct SqrtCall {
        pub x: ::ethers::core::types::U256,
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
    pub enum ArbMathCalls {
        Cdf(CdfCall),
        DivWadDown(DivWadDownCall),
        DivWadUp(DivWadUpCall),
        Log(LogCall),
        MulWadDown(MulWadDownCall),
        MulWadUp(MulWadUpCall),
        Pdf(PdfCall),
        Pow(PowCall),
        Ppf(PpfCall),
        Sqrt(SqrtCall),
    }
    impl ::ethers::core::abi::AbiDecode for ArbMathCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <CdfCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Cdf(decoded));
            }
            if let Ok(decoded) = <DivWadDownCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DivWadDown(decoded));
            }
            if let Ok(decoded) = <DivWadUpCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DivWadUp(decoded));
            }
            if let Ok(decoded) = <LogCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log(decoded));
            }
            if let Ok(decoded) = <MulWadDownCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MulWadDown(decoded));
            }
            if let Ok(decoded) = <MulWadUpCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MulWadUp(decoded));
            }
            if let Ok(decoded) = <PdfCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Pdf(decoded));
            }
            if let Ok(decoded) = <PowCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Pow(decoded));
            }
            if let Ok(decoded) = <PpfCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Ppf(decoded));
            }
            if let Ok(decoded) = <SqrtCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Sqrt(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ArbMathCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Cdf(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DivWadDown(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DivWadUp(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Log(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MulWadDown(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MulWadUp(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Pdf(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Pow(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Ppf(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Sqrt(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for ArbMathCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Cdf(element) => ::core::fmt::Display::fmt(element, f),
                Self::DivWadDown(element) => ::core::fmt::Display::fmt(element, f),
                Self::DivWadUp(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log(element) => ::core::fmt::Display::fmt(element, f),
                Self::MulWadDown(element) => ::core::fmt::Display::fmt(element, f),
                Self::MulWadUp(element) => ::core::fmt::Display::fmt(element, f),
                Self::Pdf(element) => ::core::fmt::Display::fmt(element, f),
                Self::Pow(element) => ::core::fmt::Display::fmt(element, f),
                Self::Ppf(element) => ::core::fmt::Display::fmt(element, f),
                Self::Sqrt(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<CdfCall> for ArbMathCalls {
        fn from(value: CdfCall) -> Self {
            Self::Cdf(value)
        }
    }
    impl ::core::convert::From<DivWadDownCall> for ArbMathCalls {
        fn from(value: DivWadDownCall) -> Self {
            Self::DivWadDown(value)
        }
    }
    impl ::core::convert::From<DivWadUpCall> for ArbMathCalls {
        fn from(value: DivWadUpCall) -> Self {
            Self::DivWadUp(value)
        }
    }
    impl ::core::convert::From<LogCall> for ArbMathCalls {
        fn from(value: LogCall) -> Self {
            Self::Log(value)
        }
    }
    impl ::core::convert::From<MulWadDownCall> for ArbMathCalls {
        fn from(value: MulWadDownCall) -> Self {
            Self::MulWadDown(value)
        }
    }
    impl ::core::convert::From<MulWadUpCall> for ArbMathCalls {
        fn from(value: MulWadUpCall) -> Self {
            Self::MulWadUp(value)
        }
    }
    impl ::core::convert::From<PdfCall> for ArbMathCalls {
        fn from(value: PdfCall) -> Self {
            Self::Pdf(value)
        }
    }
    impl ::core::convert::From<PowCall> for ArbMathCalls {
        fn from(value: PowCall) -> Self {
            Self::Pow(value)
        }
    }
    impl ::core::convert::From<PpfCall> for ArbMathCalls {
        fn from(value: PpfCall) -> Self {
            Self::Ppf(value)
        }
    }
    impl ::core::convert::From<SqrtCall> for ArbMathCalls {
        fn from(value: SqrtCall) -> Self {
            Self::Sqrt(value)
        }
    }
    ///Container type for all return fields from the `cdf` function with signature `cdf(int256)` and selector `0xd0b71b1e`
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
    pub struct CdfReturn {
        pub output: ::ethers::core::types::I256,
    }
    ///Container type for all return fields from the `divWadDown` function with signature `divWadDown(uint256,uint256)` and selector `0x37c6a44a`
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
    pub struct DivWadDownReturn {
        pub z: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `divWadUp` function with signature `divWadUp(uint256,uint256)` and selector `0xbd252d06`
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
    pub struct DivWadUpReturn {
        pub z: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `log` function with signature `log(int256)` and selector `0x2d5b6cb9`
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
    pub struct LogReturn {
        pub z: ::ethers::core::types::I256,
    }
    ///Container type for all return fields from the `mulWadDown` function with signature `mulWadDown(uint256,uint256)` and selector `0xe524f849`
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
    pub struct MulWadDownReturn {
        pub z: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `mulWadUp` function with signature `mulWadUp(uint256,uint256)` and selector `0xae9768a8`
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
    pub struct MulWadUpReturn {
        pub z: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `pdf` function with signature `pdf(int256)` and selector `0xd24ce6e5`
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
    pub struct PdfReturn {
        pub output: ::ethers::core::types::I256,
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
    ///Container type for all return fields from the `ppf` function with signature `ppf(int256)` and selector `0x3679723a`
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
    pub struct PpfReturn {
        pub output: ::ethers::core::types::I256,
    }
    ///Container type for all return fields from the `sqrt` function with signature `sqrt(uint256)` and selector `0x677342ce`
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
    pub struct SqrtReturn {
        pub z: ::ethers::core::types::U256,
    }
}
