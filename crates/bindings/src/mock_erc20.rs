pub use mock_erc20::*;
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
pub mod mock_erc20 {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_name"),
                        kind: ::ethers::core::abi::ethabi::ParamType::String,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("string"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_symbol"),
                        kind: ::ethers::core::abi::ethabi::ParamType::String,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("string"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_decimals"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint8"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("DOMAIN_SEPARATOR"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("DOMAIN_SEPARATOR"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("allowance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("allowance"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("approve"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("approve"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("spender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
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
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("balanceOf"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("balanceOf"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("burn"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("burn"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
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
                    ::std::borrow::ToOwned::to_owned("decimals"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("decimals"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("mint"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("mint"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
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
                    ::std::borrow::ToOwned::to_owned("name"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("name"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("nonces"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("nonces"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("permit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("permit"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("spender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("deadline"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("v"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("r"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("s"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
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
                    ::std::borrow::ToOwned::to_owned("symbol"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("symbol"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("totalSupply"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("totalSupply"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("transfer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transfer"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
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
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("transferFrom"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transferFrom"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
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
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("Approval"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Approval"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("spender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Transfer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Transfer"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static MOCKERC20_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xE0`@\x90\x80\x82R4b\0\x04\x99WPb\0\x12\xCC\x808\x03\x80\x83Q\x92b\0\0%\x82\x85b\0\x04\xE6V[\x839\x81\x01``\x82\x82\x03\x12b\0\x04JW\x81Q`\x01`\x01`@\x1B\x03\x93\x90\x84\x81\x11b\0\x04DW\x82b\0\0V\x91\x85\x01b\0\x05ZV[\x92` \x92\x83\x82\x01Q\x86\x81\x11b\0\x04DW\x83\x91b\0\0u\x91\x84\x01b\0\x05ZV[\x91\x01Q`\xFF\x81\x16\x81\x03b\0\x04?W\x84Q\x94\x86\x86\x11b\0\x04)W`\0\x95\x80b\0\0\x9E\x88Tb\0\x06\x7FV[\x92`\x1F\x93\x84\x81\x11b\0\x03\xD8W[P\x87\x90\x84\x83\x11`\x01\x14b\0\x03pW\x89\x92b\0\x03dW[PP\x81`\x01\x1B\x91`\0\x19\x90`\x03\x1B\x1C\x19\x16\x17\x86U[\x82Q\x90\x87\x82\x11b\0\x03PW\x81\x90`\x01\x94b\0\0\xF2\x86Tb\0\x06\x7FV[\x82\x81\x11b\0\x02\xFBW[P\x87\x91\x83\x11`\x01\x14b\0\x02\x97W\x88\x92b\0\x02\x8BW[PP`\0\x19`\x03\x83\x90\x1B\x1C\x19\x16\x90\x83\x1B\x17\x82U[`\x80RF`\xA0R\x81Q\x84T\x91\x81\x86b\0\x01=\x85b\0\x06\x7FV[\x92\x83\x83R\x87\x83\x01\x95\x88\x82\x82\x16\x91\x82`\0\x14b\0\x02kWPP`\x01\x14b\0\x02+W[Pb\0\x01m\x92P\x03\x82b\0\x04\xE6V[Q\x90 \x92\x81Q\x92\x83\x01\x93\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x85R\x82\x84\x01R\x7F\xC8\x9E\xFD\xAAT\xC0\xF2\x0Cz\xDFa(\x82\xDF\tP\xF5\xA9Qc~\x03\x07\xCD\xCBLg/)\x8B\x8B\xC6``\x84\x01RF`\x80\x84\x01R0`\xA0\x84\x01R`\xA0\x83R`\xC0\x83\x01\x94\x83\x86\x10\x90\x86\x11\x17b\0\x02\x17WP\x83\x90RQ\x90 `\xC0Ra\x0C\x0F\x90\x81b\0\x06\xBD\x829`\x80Q\x81a\x06\x81\x01R`\xA0Q\x81a\nM\x01R`\xC0Q\x81a\nt\x01R\xF3[cNH{q`\xE0\x1B\x81R`A`\x04R`$\x90\xFD[\x87\x91P\x88\x80R\x81\x89 \x90\x89\x91[\x85\x83\x10b\0\x02RWPPb\0\x01m\x93P\x82\x01\x018b\0\x01^V[\x80T\x83\x88\x01\x85\x01R\x86\x94P\x89\x93\x90\x92\x01\x91\x81\x01b\0\x028V[`\xFF\x19\x16\x88Rb\0\x01m\x95\x15\x15`\x05\x1B\x85\x01\x01\x92P8\x91Pb\0\x01^\x90PV[\x01Q\x90P8\x80b\0\x01\x10V[\x85\x89R\x87\x89 \x86\x94P\x91\x90`\x1F\x19\x84\x16\x8A[\x8A\x82\x82\x10b\0\x02\xE4WPP\x84\x11b\0\x02\xCAW[PPP\x81\x1B\x01\x82Ub\0\x01$V[\x01Q`\0\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U8\x80\x80b\0\x02\xBCV[\x83\x85\x01Q\x86U\x89\x97\x90\x95\x01\x94\x93\x84\x01\x93\x01b\0\x02\xA9V[\x90\x91\x92P\x85\x89R\x87\x89 \x83\x80\x86\x01`\x05\x1C\x82\x01\x92\x8A\x87\x10b\0\x03FW[\x91\x86\x95\x89\x92\x95\x94\x93\x01`\x05\x1C\x01\x91[\x82\x81\x10b\0\x037WPPb\0\0\xFBV[\x8B\x81U\x86\x95P\x88\x91\x01b\0\x03'V[\x92P\x81\x92b\0\x03\x18V[cNH{q`\xE0\x1B\x87R`A`\x04R`$\x87\xFD[\x01Q\x90P8\x80b\0\0\xC1V[\x89\x80R\x88\x8A \x92P`\x1F\x19\x84\x16\x8A[\x8A\x82\x82\x10b\0\x03\xC1WPP\x90\x84`\x01\x95\x94\x93\x92\x10b\0\x03\xA7W[PPP\x81\x1B\x01\x86Ub\0\0\xD6V[\x01Q`\0\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U8\x80\x80b\0\x03\x99V[`\x01\x85\x96\x82\x93\x96\x86\x01Q\x81U\x01\x95\x01\x93\x01b\0\x03\x7FV[\x90\x91P\x88\x80R\x87\x89 \x84\x80\x85\x01`\x05\x1C\x82\x01\x92\x8A\x86\x10b\0\x04\x1FW[\x90\x85\x94\x93\x92\x91\x01`\x05\x1C\x01\x90[\x81\x81\x10b\0\x04\x10WPb\0\0\xABV[\x8A\x81U\x84\x93P`\x01\x01b\0\x04\x01V[\x92P\x81\x92b\0\x03\xF4V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x80\xFD[b\0\x05\nV[\x82QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[bF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01Ra7\xB7`\xF1\x1B`d\x82\x01R`\x84\x90\xFD[`\x1F\x90\x91\x01`\x1F\x19\x16\x81\x01\x90`\x01`\x01`@\x1B\x03\x82\x11\x90\x82\x10\x17b\0\x04)W`@RV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[\x91\x90\x80`\x1F\x84\x01\x12\x15b\0\x06&W\x82Q\x90`\x01`\x01`@\x1B\x03\x82\x11b\0\x04)W`@Q\x91` \x91b\0\x05\x96`\x1F\x83\x01`\x1F\x19\x16\x84\x01\x85b\0\x04\xE6V[\x81\x84R\x82\x82\x87\x01\x01\x11b\0\x05\xD1W`\0[\x81\x81\x10b\0\x05\xBDWP\x82`\0\x93\x94\x95P\x01\x01R\x90V[\x85\x81\x01\x83\x01Q\x84\x82\x01\x84\x01R\x82\x01b\0\x05\xA7V[`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x83\x90R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x90\xFD[\x90`\x01\x82\x81\x1C\x92\x16\x80\x15b\0\x06\xB1W[` \x83\x10\x14b\0\x06\x9BWV[cNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[\x91`\x7F\x16\x91b\0\x06\x8FV\xFE`@`\x80\x81R`\x04\x90\x816\x10\x15a\0rW[\x90` `\x84\x92Q\x91bF\x1B\xCD`\xE5\x1B\x83R\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01Rtnor receive functions`X\x1B`d\x82\x01R\xFD[`\0\x805`\xE0\x1C\x80c\x06\xFD\xDE\x03\x14a\x07\xF1W\x80c\t^\xA7\xB3\x14a\x07\x82W\x80c\x18\x16\r\xDD\x14a\x07eW\x80c#\xB8r\xDD\x14a\x06\xA5W\x80c1<\xE5g\x14a\x06iW\x80c6D\xE5\x15\x14a\x06GW\x80c@\xC1\x0F\x19\x14a\x05\xCBW\x80cp\xA0\x821\x14a\x05\x93W\x80c~\xCE\xBE\0\x14a\x05[W\x80c\x95\xD8\x9BA\x14a\x04|W\x80c\x9D\xC2\x9F\xAC\x14a\x04\x1BW\x80c\xA9\x05\x9C\xBB\x14a\x03\xA9W\x80c\xD5\x05\xAC\xCF\x14a\x01jWc\xDDb\xED>\x14a\x01\x18WPa\0\x11V[\x914a\x01eW\x81`\x03\x196\x01\x12a\x01`W` \x92\x82\x91a\x016a\t\xF4V[a\x01>a\n\x0FV[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x84R\x91\x86R\x83\x83 \x91\x16\x82R\x84R T\x90Q\x90\x81R\xF3[a\x08\xE9V[a\x08\x99V[P\x904a\x01eW`\xE06`\x03\x19\x01\x12a\x01`Wa\x01\x85a\t\xF4V[\x90a\x01\x8Ea\n\x0FV[\x91`D5`d5\x92`\x845\x92`\xFF\x84\x16\x80\x94\x03a\x03\xA5WB\x85\x10a\x03bWa\x01\xB4a\nHV[\x95`\x01\x80`\xA0\x1B\x03\x80\x92\x16\x95\x86\x89R` \x95`\x05\x87R\x84\x8A \x98\x89T\x99`\x01\x8B\x01\x90U\x85Q\x92\x85\x89\x85\x01\x95\x7Fnq\xED\xAE\x12\xB1\xB9\x7FM\x1F`7\x0F\xEF\x10\x10_\xA2\xFA\xAE\x01&\x11J\x16\x9Cd\x84]a&\xC9\x87R\x8B\x89\x87\x01R\x16\x9A\x8B``\x86\x01R\x88`\x80\x86\x01R`\xA0\x85\x01R`\xC0\x84\x01R`\xC0\x83R`\xE0\x83\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x94\x84\x82\x10\x86\x83\x11\x17a\x03NW\x81\x88R\x84Q\x90 a\x01\0\x85\x01\x92a\x19\x01`\xF0\x1B\x84Ra\x01\x02\x86\x01Ra\x01\"\x85\x01R`B\x81Ra\x01`\x84\x01\x94\x81\x86\x10\x90\x86\x11\x17a\x03;W\x84\x87RQ\x90 \x83Ra\x01\x80\x82\x01R`\xA45a\x01\xA0\x82\x01R`\xC45a\x01\xC0\x90\x91\x01R\x87\x80R\x84\x90\x88\x90`\x80\x90`\x01Z\xFA\x15a\x031W\x86Q\x16\x96\x87\x15\x15\x80a\x03(W[\x15a\x02\xF6W\x86\x97\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x95\x96\x97R\x83R\x80\x87 \x86\x88R\x83R\x81\x81\x88 UQ\x90\x81R\xA3\x80\xF3[\x83`d\x92Q\x91bF\x1B\xCD`\xE5\x1B\x83R\x82\x01R`\x0E`$\x82\x01Rm$\xA7+ \xA6$\xA2/\xA9\xA4\xA3\xA7\"\xA9`\x91\x1B`D\x82\x01R\xFD[P\x84\x88\x14a\x02\xB3V[\x81Q=\x88\x82>=\x90\xFD[cNH{q`\xE0\x1B\x8CR`A\x8DR`$\x8C\xFD[PcNH{q`\xE0\x1B\x8CR`A\x8DR`$\x8C\xFD[\x81QbF\x1B\xCD`\xE5\x1B\x81R` \x81\x8A\x01R`\x17`$\x82\x01R\x7FPERMIT_DEADLINE_EXPIRED\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x90\xFD[\x86\x80\xFD[P\x904a\x01eW\x80`\x03\x196\x01\x12a\x01`W` \x91a\x03\xC6a\t\xF4V[\x82`$5\x913\x84R`\x03\x86R\x81\x84 a\x03\xE0\x84\x82Ta\n%V[\x90U`\x01`\x01`\xA0\x1B\x03\x16\x80\x84R`\x03\x86R\x92 \x80T\x82\x01\x90U\x82Q\x90\x81R3\x90`\0\x80Q` a\x0B\xEF\x839\x81Q\x91R\x90\x85\x90\xA3Q`\x01\x81R\xF3[P\x904a\x01eW\x80`\x03\x196\x01\x12a\x01`W\x81\x90`\0\x80Q` a\x0B\xEF\x839\x81Q\x91R` a\x04Ha\t\xF4V[`$5\x90`\x01\x80`\xA0\x1B\x03\x16\x93\x84\x86R`\x03\x83R\x80\x86 a\x04j\x83\x82Ta\n%V[\x90U\x81`\x02T\x03`\x02UQ\x90\x81R\xA3\x80\xF3[P\x904a\x01eW\x81`\x03\x196\x01\x12a\x01`W\x80Q\x90\x82`\x01\x80T\x91a\x04\xA0\x83a\t9V[\x80\x86R\x92\x82\x81\x16\x90\x81\x15a\x053WP`\x01\x14a\x04\xD7W[PPPa\x04\xC9\x82a\x04\xD3\x94\x03\x83a\tsV[Q\x91\x82\x91\x82a\t\xABV[\x03\x90\xF3[\x94P\x80\x85R\x7F\xB1\x0E-Rv\x12\x07;&\xEE\xCD\xFDq~j2\x0C\xF4KJ\xFA\xC2\xB0s-\x9F\xCB\xE2\xB7\xFA\x0C\xF6[\x82\x86\x10a\x05\x1BWPPPa\x04\xC9\x82` a\x04\xD3\x95\x82\x01\x01\x94a\x04\xB7V[\x80T` \x87\x87\x01\x81\x01\x91\x90\x91R\x90\x95\x01\x94\x81\x01a\x04\xFEV[a\x04\xD3\x97P\x86\x93P` \x92Pa\x04\xC9\x94\x91P`\xFF\x19\x16\x82\x84\x01R\x15\x15`\x05\x1B\x82\x01\x01\x94a\x04\xB7V[P\x904a\x01eW` 6`\x03\x19\x01\x12a\x01`W` \x91\x81\x90`\x01`\x01`\xA0\x1B\x03a\x05\x83a\t\xF4V[\x16\x81R`\x05\x84R T\x90Q\x90\x81R\xF3[P\x904a\x01eW` 6`\x03\x19\x01\x12a\x01`W` \x91\x81\x90`\x01`\x01`\xA0\x1B\x03a\x05\xBBa\t\xF4V[\x16\x81R`\x03\x84R T\x90Q\x90\x81R\xF3[P\x914a\x01eW\x81`\x03\x196\x01\x12a\x01`Wa\x05\xE5a\t\xF4V[`$5\x91`\x02T\x90\x83\x82\x01\x80\x92\x11a\x064WP`\x02U`\x01`\x01`\xA0\x1B\x03\x16\x80\x84R`\x03` \x90\x81R\x83\x85 \x80T\x84\x01\x90U\x92Q\x91\x82R\x91\x83\x91`\0\x80Q` a\x0B\xEF\x839\x81Q\x91R\x91\x90\xA3\x80\xF3[cNH{q`\xE0\x1B\x86R`\x11\x90R`$\x85\xFD[P4a\x01eW6`\x03\x19\x01\x12a\x01`W` \x90a\x06ba\nHV[\x90Q\x90\x81R\xF3[P4a\x01eW6`\x03\x19\x01\x12a\x01`W` \x90Q`\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[P4a\x01eW``6`\x03\x19\x01\x12a\x01`Wa\x06\xBFa\t\xF4V[`\0\x80Q` a\x0B\xEF\x839\x81Q\x91Ra\x06\xD6a\n\x0FV[`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x80\x85R` \x87\x81R\x86\x86 3\x87R\x81R\x86\x86 T\x90\x97\x91\x94\x88\x93`D5\x93\x89\x93\x85`\x01\x82\x01a\x07BW[PPP\x86\x88R`\x03\x85R\x82\x88 a\x07#\x85\x82Ta\n%V[\x90U\x16\x95\x86\x81R`\x03\x84R \x81\x81T\x01\x90U\x85Q\x90\x81R\xA3Q`\x01\x81R\xF3[a\x07K\x91a\n%V[\x90\x88\x8AR\x86R\x83\x89 3\x8AR\x86R\x83\x89 U8\x80\x85a\x07\x0BV[P4a\x01eW6`\x03\x19\x01\x12a\x01`W` \x90`\x02T\x90Q\x90\x81R\xF3[P\x914a\x01eW\x81`\x03\x196\x01\x12a\x01`W` \x92a\x07\x9Fa\t\xF4V[\x91\x83`$5\x92\x83\x923\x82R\x87R\x81\x81 \x94`\x01\x80`\xA0\x1B\x03\x16\x94\x85\x82R\x87R U\x82Q\x90\x81R\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x843\x92\xA3Q`\x01\x81R\xF3[P\x904a\x08\x99W\x81`\x03\x196\x01\x12a\x01`W\x80Q\x90\x82\x80Ta\x08\x12\x81a\t9V[\x80\x85R\x91`\x01\x91\x80\x83\x16\x90\x81\x15a\x053WP`\x01\x14a\x08=WPPPa\x04\xC9\x82a\x04\xD3\x94\x03\x83a\tsV[\x80\x80\x96PR\x7F)\r\xEC\xD9T\x8Bb\xA8\xD6\x03E\xA9\x888o\xC8K\xA6\xBC\x95H@\x08\xF66/\x93\x16\x0E\xF3\xE5c[\x82\x86\x10a\x08\x81WPPPa\x04\xC9\x82` a\x04\xD3\x95\x82\x01\x01\x94a\x04\xB7V[\x80T` \x87\x87\x01\x81\x01\x91\x90\x91R\x90\x95\x01\x94\x81\x01a\x08dV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01Ra7\xB7`\xF1\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[\x90`\x01\x82\x81\x1C\x92\x16\x80\x15a\tiW[` \x83\x10\x14a\tSWV[cNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[\x91`\x7F\x16\x91a\tHV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\t\x95W`@RV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[` \x80\x82R\x82Q\x81\x83\x01\x81\x90R\x90\x93\x92`\0[\x82\x81\x10a\t\xE0WPP`@\x92\x93P`\0\x83\x82\x84\x01\x01R`\x1F\x80\x19\x91\x01\x16\x01\x01\x90V[\x81\x81\x01\x86\x01Q\x84\x82\x01`@\x01R\x85\x01a\t\xBEV[`\x045\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\n\nWV[`\0\x80\xFD[`$5\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\n\nWV[\x91\x90\x82\x03\x91\x82\x11a\n2WV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0F\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x03a\n\x96WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[`@Q\x81T\x82\x91a\n\xA6\x82a\t9V[\x80\x82R\x81` \x94\x85\x82\x01\x94`\x01\x90\x87\x82\x82\x16\x91\x82`\0\x14a\x0B\xD0WPP`\x01\x14a\x0BwW[Pa\n\xD8\x92P\x03\x82a\tsV[Q\x90 \x91`@Q\x91\x82\x01\x92\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x84R`@\x83\x01R\x7F\xC8\x9E\xFD\xAAT\xC0\xF2\x0Cz\xDFa(\x82\xDF\tP\xF5\xA9Qc~\x03\x07\xCD\xCBLg/)\x8B\x8B\xC6``\x83\x01RF`\x80\x83\x01R0`\xA0\x83\x01R`\xA0\x82R`\xC0\x82\x01\x90\x82\x82\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x17a\x0BcWP`@RQ\x90 \x90V[cNH{q`\xE0\x1B\x81R`A`\x04R`$\x90\xFD[\x87\x80R\x86\x91P\x87\x90\x7F)\r\xEC\xD9T\x8Bb\xA8\xD6\x03E\xA9\x888o\xC8K\xA6\xBC\x95H@\x08\xF66/\x93\x16\x0E\xF3\xE5c[\x85\x83\x10a\x0B\xB8WPPa\n\xD8\x93P\x82\x01\x018a\n\xCBV[\x80T\x83\x88\x01\x85\x01R\x86\x94P\x88\x93\x90\x92\x01\x91\x81\x01a\x0B\xA1V[`\xFF\x19\x16\x88Ra\n\xD8\x95\x15\x15`\x05\x1B\x85\x01\x01\x92P8\x91Pa\n\xCB\x90PV\xFE\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF";
    /// The bytecode of the contract.
    pub static MOCKERC20_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`@`\x80\x81R`\x04\x90\x816\x10\x15a\0rW[\x90` `\x84\x92Q\x91bF\x1B\xCD`\xE5\x1B\x83R\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01Rtnor receive functions`X\x1B`d\x82\x01R\xFD[`\0\x805`\xE0\x1C\x80c\x06\xFD\xDE\x03\x14a\x07\xF1W\x80c\t^\xA7\xB3\x14a\x07\x82W\x80c\x18\x16\r\xDD\x14a\x07eW\x80c#\xB8r\xDD\x14a\x06\xA5W\x80c1<\xE5g\x14a\x06iW\x80c6D\xE5\x15\x14a\x06GW\x80c@\xC1\x0F\x19\x14a\x05\xCBW\x80cp\xA0\x821\x14a\x05\x93W\x80c~\xCE\xBE\0\x14a\x05[W\x80c\x95\xD8\x9BA\x14a\x04|W\x80c\x9D\xC2\x9F\xAC\x14a\x04\x1BW\x80c\xA9\x05\x9C\xBB\x14a\x03\xA9W\x80c\xD5\x05\xAC\xCF\x14a\x01jWc\xDDb\xED>\x14a\x01\x18WPa\0\x11V[\x914a\x01eW\x81`\x03\x196\x01\x12a\x01`W` \x92\x82\x91a\x016a\t\xF4V[a\x01>a\n\x0FV[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x84R\x91\x86R\x83\x83 \x91\x16\x82R\x84R T\x90Q\x90\x81R\xF3[a\x08\xE9V[a\x08\x99V[P\x904a\x01eW`\xE06`\x03\x19\x01\x12a\x01`Wa\x01\x85a\t\xF4V[\x90a\x01\x8Ea\n\x0FV[\x91`D5`d5\x92`\x845\x92`\xFF\x84\x16\x80\x94\x03a\x03\xA5WB\x85\x10a\x03bWa\x01\xB4a\nHV[\x95`\x01\x80`\xA0\x1B\x03\x80\x92\x16\x95\x86\x89R` \x95`\x05\x87R\x84\x8A \x98\x89T\x99`\x01\x8B\x01\x90U\x85Q\x92\x85\x89\x85\x01\x95\x7Fnq\xED\xAE\x12\xB1\xB9\x7FM\x1F`7\x0F\xEF\x10\x10_\xA2\xFA\xAE\x01&\x11J\x16\x9Cd\x84]a&\xC9\x87R\x8B\x89\x87\x01R\x16\x9A\x8B``\x86\x01R\x88`\x80\x86\x01R`\xA0\x85\x01R`\xC0\x84\x01R`\xC0\x83R`\xE0\x83\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x94\x84\x82\x10\x86\x83\x11\x17a\x03NW\x81\x88R\x84Q\x90 a\x01\0\x85\x01\x92a\x19\x01`\xF0\x1B\x84Ra\x01\x02\x86\x01Ra\x01\"\x85\x01R`B\x81Ra\x01`\x84\x01\x94\x81\x86\x10\x90\x86\x11\x17a\x03;W\x84\x87RQ\x90 \x83Ra\x01\x80\x82\x01R`\xA45a\x01\xA0\x82\x01R`\xC45a\x01\xC0\x90\x91\x01R\x87\x80R\x84\x90\x88\x90`\x80\x90`\x01Z\xFA\x15a\x031W\x86Q\x16\x96\x87\x15\x15\x80a\x03(W[\x15a\x02\xF6W\x86\x97\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x95\x96\x97R\x83R\x80\x87 \x86\x88R\x83R\x81\x81\x88 UQ\x90\x81R\xA3\x80\xF3[\x83`d\x92Q\x91bF\x1B\xCD`\xE5\x1B\x83R\x82\x01R`\x0E`$\x82\x01Rm$\xA7+ \xA6$\xA2/\xA9\xA4\xA3\xA7\"\xA9`\x91\x1B`D\x82\x01R\xFD[P\x84\x88\x14a\x02\xB3V[\x81Q=\x88\x82>=\x90\xFD[cNH{q`\xE0\x1B\x8CR`A\x8DR`$\x8C\xFD[PcNH{q`\xE0\x1B\x8CR`A\x8DR`$\x8C\xFD[\x81QbF\x1B\xCD`\xE5\x1B\x81R` \x81\x8A\x01R`\x17`$\x82\x01R\x7FPERMIT_DEADLINE_EXPIRED\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x90\xFD[\x86\x80\xFD[P\x904a\x01eW\x80`\x03\x196\x01\x12a\x01`W` \x91a\x03\xC6a\t\xF4V[\x82`$5\x913\x84R`\x03\x86R\x81\x84 a\x03\xE0\x84\x82Ta\n%V[\x90U`\x01`\x01`\xA0\x1B\x03\x16\x80\x84R`\x03\x86R\x92 \x80T\x82\x01\x90U\x82Q\x90\x81R3\x90`\0\x80Q` a\x0B\xEF\x839\x81Q\x91R\x90\x85\x90\xA3Q`\x01\x81R\xF3[P\x904a\x01eW\x80`\x03\x196\x01\x12a\x01`W\x81\x90`\0\x80Q` a\x0B\xEF\x839\x81Q\x91R` a\x04Ha\t\xF4V[`$5\x90`\x01\x80`\xA0\x1B\x03\x16\x93\x84\x86R`\x03\x83R\x80\x86 a\x04j\x83\x82Ta\n%V[\x90U\x81`\x02T\x03`\x02UQ\x90\x81R\xA3\x80\xF3[P\x904a\x01eW\x81`\x03\x196\x01\x12a\x01`W\x80Q\x90\x82`\x01\x80T\x91a\x04\xA0\x83a\t9V[\x80\x86R\x92\x82\x81\x16\x90\x81\x15a\x053WP`\x01\x14a\x04\xD7W[PPPa\x04\xC9\x82a\x04\xD3\x94\x03\x83a\tsV[Q\x91\x82\x91\x82a\t\xABV[\x03\x90\xF3[\x94P\x80\x85R\x7F\xB1\x0E-Rv\x12\x07;&\xEE\xCD\xFDq~j2\x0C\xF4KJ\xFA\xC2\xB0s-\x9F\xCB\xE2\xB7\xFA\x0C\xF6[\x82\x86\x10a\x05\x1BWPPPa\x04\xC9\x82` a\x04\xD3\x95\x82\x01\x01\x94a\x04\xB7V[\x80T` \x87\x87\x01\x81\x01\x91\x90\x91R\x90\x95\x01\x94\x81\x01a\x04\xFEV[a\x04\xD3\x97P\x86\x93P` \x92Pa\x04\xC9\x94\x91P`\xFF\x19\x16\x82\x84\x01R\x15\x15`\x05\x1B\x82\x01\x01\x94a\x04\xB7V[P\x904a\x01eW` 6`\x03\x19\x01\x12a\x01`W` \x91\x81\x90`\x01`\x01`\xA0\x1B\x03a\x05\x83a\t\xF4V[\x16\x81R`\x05\x84R T\x90Q\x90\x81R\xF3[P\x904a\x01eW` 6`\x03\x19\x01\x12a\x01`W` \x91\x81\x90`\x01`\x01`\xA0\x1B\x03a\x05\xBBa\t\xF4V[\x16\x81R`\x03\x84R T\x90Q\x90\x81R\xF3[P\x914a\x01eW\x81`\x03\x196\x01\x12a\x01`Wa\x05\xE5a\t\xF4V[`$5\x91`\x02T\x90\x83\x82\x01\x80\x92\x11a\x064WP`\x02U`\x01`\x01`\xA0\x1B\x03\x16\x80\x84R`\x03` \x90\x81R\x83\x85 \x80T\x84\x01\x90U\x92Q\x91\x82R\x91\x83\x91`\0\x80Q` a\x0B\xEF\x839\x81Q\x91R\x91\x90\xA3\x80\xF3[cNH{q`\xE0\x1B\x86R`\x11\x90R`$\x85\xFD[P4a\x01eW6`\x03\x19\x01\x12a\x01`W` \x90a\x06ba\nHV[\x90Q\x90\x81R\xF3[P4a\x01eW6`\x03\x19\x01\x12a\x01`W` \x90Q`\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[P4a\x01eW``6`\x03\x19\x01\x12a\x01`Wa\x06\xBFa\t\xF4V[`\0\x80Q` a\x0B\xEF\x839\x81Q\x91Ra\x06\xD6a\n\x0FV[`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x80\x85R` \x87\x81R\x86\x86 3\x87R\x81R\x86\x86 T\x90\x97\x91\x94\x88\x93`D5\x93\x89\x93\x85`\x01\x82\x01a\x07BW[PPP\x86\x88R`\x03\x85R\x82\x88 a\x07#\x85\x82Ta\n%V[\x90U\x16\x95\x86\x81R`\x03\x84R \x81\x81T\x01\x90U\x85Q\x90\x81R\xA3Q`\x01\x81R\xF3[a\x07K\x91a\n%V[\x90\x88\x8AR\x86R\x83\x89 3\x8AR\x86R\x83\x89 U8\x80\x85a\x07\x0BV[P4a\x01eW6`\x03\x19\x01\x12a\x01`W` \x90`\x02T\x90Q\x90\x81R\xF3[P\x914a\x01eW\x81`\x03\x196\x01\x12a\x01`W` \x92a\x07\x9Fa\t\xF4V[\x91\x83`$5\x92\x83\x923\x82R\x87R\x81\x81 \x94`\x01\x80`\xA0\x1B\x03\x16\x94\x85\x82R\x87R U\x82Q\x90\x81R\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x843\x92\xA3Q`\x01\x81R\xF3[P\x904a\x08\x99W\x81`\x03\x196\x01\x12a\x01`W\x80Q\x90\x82\x80Ta\x08\x12\x81a\t9V[\x80\x85R\x91`\x01\x91\x80\x83\x16\x90\x81\x15a\x053WP`\x01\x14a\x08=WPPPa\x04\xC9\x82a\x04\xD3\x94\x03\x83a\tsV[\x80\x80\x96PR\x7F)\r\xEC\xD9T\x8Bb\xA8\xD6\x03E\xA9\x888o\xC8K\xA6\xBC\x95H@\x08\xF66/\x93\x16\x0E\xF3\xE5c[\x82\x86\x10a\x08\x81WPPPa\x04\xC9\x82` a\x04\xD3\x95\x82\x01\x01\x94a\x04\xB7V[\x80T` \x87\x87\x01\x81\x01\x91\x90\x91R\x90\x95\x01\x94\x81\x01a\x08dV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01Ra7\xB7`\xF1\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[\x90`\x01\x82\x81\x1C\x92\x16\x80\x15a\tiW[` \x83\x10\x14a\tSWV[cNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[\x91`\x7F\x16\x91a\tHV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\t\x95W`@RV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[` \x80\x82R\x82Q\x81\x83\x01\x81\x90R\x90\x93\x92`\0[\x82\x81\x10a\t\xE0WPP`@\x92\x93P`\0\x83\x82\x84\x01\x01R`\x1F\x80\x19\x91\x01\x16\x01\x01\x90V[\x81\x81\x01\x86\x01Q\x84\x82\x01`@\x01R\x85\x01a\t\xBEV[`\x045\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\n\nWV[`\0\x80\xFD[`$5\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\n\nWV[\x91\x90\x82\x03\x91\x82\x11a\n2WV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0F\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x03a\n\x96WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[`@Q\x81T\x82\x91a\n\xA6\x82a\t9V[\x80\x82R\x81` \x94\x85\x82\x01\x94`\x01\x90\x87\x82\x82\x16\x91\x82`\0\x14a\x0B\xD0WPP`\x01\x14a\x0BwW[Pa\n\xD8\x92P\x03\x82a\tsV[Q\x90 \x91`@Q\x91\x82\x01\x92\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x84R`@\x83\x01R\x7F\xC8\x9E\xFD\xAAT\xC0\xF2\x0Cz\xDFa(\x82\xDF\tP\xF5\xA9Qc~\x03\x07\xCD\xCBLg/)\x8B\x8B\xC6``\x83\x01RF`\x80\x83\x01R0`\xA0\x83\x01R`\xA0\x82R`\xC0\x82\x01\x90\x82\x82\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x17a\x0BcWP`@RQ\x90 \x90V[cNH{q`\xE0\x1B\x81R`A`\x04R`$\x90\xFD[\x87\x80R\x86\x91P\x87\x90\x7F)\r\xEC\xD9T\x8Bb\xA8\xD6\x03E\xA9\x888o\xC8K\xA6\xBC\x95H@\x08\xF66/\x93\x16\x0E\xF3\xE5c[\x85\x83\x10a\x0B\xB8WPPa\n\xD8\x93P\x82\x01\x018a\n\xCBV[\x80T\x83\x88\x01\x85\x01R\x86\x94P\x88\x93\x90\x92\x01\x91\x81\x01a\x0B\xA1V[`\xFF\x19\x16\x88Ra\n\xD8\x95\x15\x15`\x05\x1B\x85\x01\x01\x92P8\x91Pa\n\xCB\x90PV\xFE\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF";
    /// The deployed bytecode of the contract.
    pub static MOCKERC20_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct MockERC20<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MockERC20<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MockERC20<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MockERC20<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MockERC20<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MockERC20)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MockERC20<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    MOCKERC20_ABI.clone(),
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
                MOCKERC20_ABI.clone(),
                MOCKERC20_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `DOMAIN_SEPARATOR` (0x3644e515) function
        pub fn domain_separator(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([54, 68, 229, 21], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `allowance` (0xdd62ed3e) function
        pub fn allowance(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([221, 98, 237, 62], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `approve` (0x095ea7b3) function
        pub fn approve(
            &self,
            spender: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([9, 94, 167, 179], (spender, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `balanceOf` (0x70a08231) function
        pub fn balance_of(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `burn` (0x9dc29fac) function
        pub fn burn(
            &self,
            from: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([157, 194, 159, 172], (from, value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `decimals` (0x313ce567) function
        pub fn decimals(&self) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([49, 60, 229, 103], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mint` (0x40c10f19) function
        pub fn mint(
            &self,
            to: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([64, 193, 15, 25], (to, value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `name` (0x06fdde03) function
        pub fn name(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([6, 253, 222, 3], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `nonces` (0x7ecebe00) function
        pub fn nonces(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([126, 206, 190, 0], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `permit` (0xd505accf) function
        pub fn permit(
            &self,
            owner: ::ethers::core::types::Address,
            spender: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
            deadline: ::ethers::core::types::U256,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [213, 5, 172, 207],
                    (owner, spender, value, deadline, v, r, s),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `symbol` (0x95d89b41) function
        pub fn symbol(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([149, 216, 155, 65], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `totalSupply` (0x18160ddd) function
        pub fn total_supply(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([24, 22, 13, 221], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transfer` (0xa9059cbb) function
        pub fn transfer(
            &self,
            to: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([169, 5, 156, 187], (to, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferFrom` (0x23b872dd) function
        pub fn transfer_from(
            &self,
            from: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([35, 184, 114, 221], (from, to, amount))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `Approval` event
        pub fn approval_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ApprovalFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Transfer` event
        pub fn transfer_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            TransferFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            MockERC20Events,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for MockERC20<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "Approval", abi = "Approval(address,address,uint256)")]
    pub struct ApprovalFilter {
        #[ethevent(indexed)]
        pub owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub spender: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "Transfer", abi = "Transfer(address,address,uint256)")]
    pub struct TransferFilter {
        #[ethevent(indexed)]
        pub from: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's events
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
    pub enum MockERC20Events {
        ApprovalFilter(ApprovalFilter),
        TransferFilter(TransferFilter),
    }
    impl ::ethers::contract::EthLogDecode for MockERC20Events {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(MockERC20Events::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(MockERC20Events::TransferFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for MockERC20Events {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ApprovalFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ApprovalFilter> for MockERC20Events {
        fn from(value: ApprovalFilter) -> Self {
            Self::ApprovalFilter(value)
        }
    }
    impl ::core::convert::From<TransferFilter> for MockERC20Events {
        fn from(value: TransferFilter) -> Self {
            Self::TransferFilter(value)
        }
    }
    ///Container type for all input parameters for the `DOMAIN_SEPARATOR` function with signature `DOMAIN_SEPARATOR()` and selector `0x3644e515`
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
    #[ethcall(name = "DOMAIN_SEPARATOR", abi = "DOMAIN_SEPARATOR()")]
    pub struct DomainSeparatorCall;
    ///Container type for all input parameters for the `allowance` function with signature `allowance(address,address)` and selector `0xdd62ed3e`
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
    #[ethcall(name = "allowance", abi = "allowance(address,address)")]
    pub struct AllowanceCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
    );
    ///Container type for all input parameters for the `approve` function with signature `approve(address,uint256)` and selector `0x095ea7b3`
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
    #[ethcall(name = "approve", abi = "approve(address,uint256)")]
    pub struct ApproveCall {
        pub spender: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `balanceOf` function with signature `balanceOf(address)` and selector `0x70a08231`
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
    #[ethcall(name = "balanceOf", abi = "balanceOf(address)")]
    pub struct BalanceOfCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `burn` function with signature `burn(address,uint256)` and selector `0x9dc29fac`
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
    #[ethcall(name = "burn", abi = "burn(address,uint256)")]
    pub struct BurnCall {
        pub from: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `decimals` function with signature `decimals()` and selector `0x313ce567`
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
    #[ethcall(name = "decimals", abi = "decimals()")]
    pub struct DecimalsCall;
    ///Container type for all input parameters for the `mint` function with signature `mint(address,uint256)` and selector `0x40c10f19`
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
    #[ethcall(name = "mint", abi = "mint(address,uint256)")]
    pub struct MintCall {
        pub to: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `name` function with signature `name()` and selector `0x06fdde03`
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
    #[ethcall(name = "name", abi = "name()")]
    pub struct NameCall;
    ///Container type for all input parameters for the `nonces` function with signature `nonces(address)` and selector `0x7ecebe00`
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
    #[ethcall(name = "nonces", abi = "nonces(address)")]
    pub struct NoncesCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `permit` function with signature `permit(address,address,uint256,uint256,uint8,bytes32,bytes32)` and selector `0xd505accf`
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
        name = "permit",
        abi = "permit(address,address,uint256,uint256,uint8,bytes32,bytes32)"
    )]
    pub struct PermitCall {
        pub owner: ::ethers::core::types::Address,
        pub spender: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub deadline: ::ethers::core::types::U256,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    ///Container type for all input parameters for the `symbol` function with signature `symbol()` and selector `0x95d89b41`
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
    #[ethcall(name = "symbol", abi = "symbol()")]
    pub struct SymbolCall;
    ///Container type for all input parameters for the `totalSupply` function with signature `totalSupply()` and selector `0x18160ddd`
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
    #[ethcall(name = "totalSupply", abi = "totalSupply()")]
    pub struct TotalSupplyCall;
    ///Container type for all input parameters for the `transfer` function with signature `transfer(address,uint256)` and selector `0xa9059cbb`
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
    #[ethcall(name = "transfer", abi = "transfer(address,uint256)")]
    pub struct TransferCall {
        pub to: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `transferFrom` function with signature `transferFrom(address,address,uint256)` and selector `0x23b872dd`
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
    #[ethcall(name = "transferFrom", abi = "transferFrom(address,address,uint256)")]
    pub struct TransferFromCall {
        pub from: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
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
    pub enum MockERC20Calls {
        DomainSeparator(DomainSeparatorCall),
        Allowance(AllowanceCall),
        Approve(ApproveCall),
        BalanceOf(BalanceOfCall),
        Burn(BurnCall),
        Decimals(DecimalsCall),
        Mint(MintCall),
        Name(NameCall),
        Nonces(NoncesCall),
        Permit(PermitCall),
        Symbol(SymbolCall),
        TotalSupply(TotalSupplyCall),
        Transfer(TransferCall),
        TransferFrom(TransferFromCall),
    }
    impl ::ethers::core::abi::AbiDecode for MockERC20Calls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <DomainSeparatorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DomainSeparator(decoded));
            }
            if let Ok(decoded) = <AllowanceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Allowance(decoded));
            }
            if let Ok(decoded) = <ApproveCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Approve(decoded));
            }
            if let Ok(decoded) = <BalanceOfCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BalanceOf(decoded));
            }
            if let Ok(decoded) = <BurnCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Burn(decoded));
            }
            if let Ok(decoded) = <DecimalsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Decimals(decoded));
            }
            if let Ok(decoded) = <MintCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Mint(decoded));
            }
            if let Ok(decoded) = <NameCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Name(decoded));
            }
            if let Ok(decoded) = <NoncesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Nonces(decoded));
            }
            if let Ok(decoded) = <PermitCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Permit(decoded));
            }
            if let Ok(decoded) = <SymbolCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Symbol(decoded));
            }
            if let Ok(decoded) = <TotalSupplyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TotalSupply(decoded));
            }
            if let Ok(decoded) = <TransferCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Transfer(decoded));
            }
            if let Ok(decoded) = <TransferFromCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransferFrom(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MockERC20Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::DomainSeparator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Allowance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Approve(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BalanceOf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Burn(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Decimals(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Mint(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Name(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Nonces(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Permit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Symbol(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TotalSupply(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Transfer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferFrom(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for MockERC20Calls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DomainSeparator(element) => ::core::fmt::Display::fmt(element, f),
                Self::Allowance(element) => ::core::fmt::Display::fmt(element, f),
                Self::Approve(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::Burn(element) => ::core::fmt::Display::fmt(element, f),
                Self::Decimals(element) => ::core::fmt::Display::fmt(element, f),
                Self::Mint(element) => ::core::fmt::Display::fmt(element, f),
                Self::Name(element) => ::core::fmt::Display::fmt(element, f),
                Self::Nonces(element) => ::core::fmt::Display::fmt(element, f),
                Self::Permit(element) => ::core::fmt::Display::fmt(element, f),
                Self::Symbol(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalSupply(element) => ::core::fmt::Display::fmt(element, f),
                Self::Transfer(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFrom(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DomainSeparatorCall> for MockERC20Calls {
        fn from(value: DomainSeparatorCall) -> Self {
            Self::DomainSeparator(value)
        }
    }
    impl ::core::convert::From<AllowanceCall> for MockERC20Calls {
        fn from(value: AllowanceCall) -> Self {
            Self::Allowance(value)
        }
    }
    impl ::core::convert::From<ApproveCall> for MockERC20Calls {
        fn from(value: ApproveCall) -> Self {
            Self::Approve(value)
        }
    }
    impl ::core::convert::From<BalanceOfCall> for MockERC20Calls {
        fn from(value: BalanceOfCall) -> Self {
            Self::BalanceOf(value)
        }
    }
    impl ::core::convert::From<BurnCall> for MockERC20Calls {
        fn from(value: BurnCall) -> Self {
            Self::Burn(value)
        }
    }
    impl ::core::convert::From<DecimalsCall> for MockERC20Calls {
        fn from(value: DecimalsCall) -> Self {
            Self::Decimals(value)
        }
    }
    impl ::core::convert::From<MintCall> for MockERC20Calls {
        fn from(value: MintCall) -> Self {
            Self::Mint(value)
        }
    }
    impl ::core::convert::From<NameCall> for MockERC20Calls {
        fn from(value: NameCall) -> Self {
            Self::Name(value)
        }
    }
    impl ::core::convert::From<NoncesCall> for MockERC20Calls {
        fn from(value: NoncesCall) -> Self {
            Self::Nonces(value)
        }
    }
    impl ::core::convert::From<PermitCall> for MockERC20Calls {
        fn from(value: PermitCall) -> Self {
            Self::Permit(value)
        }
    }
    impl ::core::convert::From<SymbolCall> for MockERC20Calls {
        fn from(value: SymbolCall) -> Self {
            Self::Symbol(value)
        }
    }
    impl ::core::convert::From<TotalSupplyCall> for MockERC20Calls {
        fn from(value: TotalSupplyCall) -> Self {
            Self::TotalSupply(value)
        }
    }
    impl ::core::convert::From<TransferCall> for MockERC20Calls {
        fn from(value: TransferCall) -> Self {
            Self::Transfer(value)
        }
    }
    impl ::core::convert::From<TransferFromCall> for MockERC20Calls {
        fn from(value: TransferFromCall) -> Self {
            Self::TransferFrom(value)
        }
    }
    ///Container type for all return fields from the `DOMAIN_SEPARATOR` function with signature `DOMAIN_SEPARATOR()` and selector `0x3644e515`
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
    pub struct DomainSeparatorReturn(pub [u8; 32]);
    ///Container type for all return fields from the `allowance` function with signature `allowance(address,address)` and selector `0xdd62ed3e`
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
    pub struct AllowanceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `approve` function with signature `approve(address,uint256)` and selector `0x095ea7b3`
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
    pub struct ApproveReturn(pub bool);
    ///Container type for all return fields from the `balanceOf` function with signature `balanceOf(address)` and selector `0x70a08231`
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
    pub struct BalanceOfReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `decimals` function with signature `decimals()` and selector `0x313ce567`
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
    pub struct DecimalsReturn(pub u8);
    ///Container type for all return fields from the `name` function with signature `name()` and selector `0x06fdde03`
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
    pub struct NameReturn(pub ::std::string::String);
    ///Container type for all return fields from the `nonces` function with signature `nonces(address)` and selector `0x7ecebe00`
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
    pub struct NoncesReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `symbol` function with signature `symbol()` and selector `0x95d89b41`
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
    pub struct SymbolReturn(pub ::std::string::String);
    ///Container type for all return fields from the `totalSupply` function with signature `totalSupply()` and selector `0x18160ddd`
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
    pub struct TotalSupplyReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `transfer` function with signature `transfer(address,uint256)` and selector `0xa9059cbb`
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
    pub struct TransferReturn(pub bool);
    ///Container type for all return fields from the `transferFrom` function with signature `transferFrom(address,address,uint256)` and selector `0x23b872dd`
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
    pub struct TransferFromReturn(pub bool);
}
