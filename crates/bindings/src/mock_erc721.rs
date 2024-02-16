pub use mock_erc721::*;
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
pub mod mock_erc721 {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("approve"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
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
                                name: ::std::borrow::ToOwned::to_owned("id"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("balanceOf"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("balanceOf"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("owner"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getApproved"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getApproved"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("initialize"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("initialize"),
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
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isApprovedForAll"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("isApprovedForAll"),
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
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bool"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("name"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("name"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::String,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("string"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ownerOf"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("ownerOf"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("id"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("owner"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("safeTransferFrom"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("safeTransferFrom"),
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
                                    name: ::std::borrow::ToOwned::to_owned("id"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability:
                                ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("safeTransferFrom"),
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
                                    name: ::std::borrow::ToOwned::to_owned("id"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability:
                                ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setApprovalForAll"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setApprovalForAll"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("approved"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bool"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("supportsInterface"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("supportsInterface"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("interfaceId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(4usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes4"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bool"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("symbol"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("symbol"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::String,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("string"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("tokenURI"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("tokenURI"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("id"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::String,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("string"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("transferFrom"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
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
                                name: ::std::borrow::ToOwned::to_owned("id"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("Approval"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
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
                                name: ::std::borrow::ToOwned::to_owned("id"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: true,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ApprovalForAll"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("ApprovalForAll"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("owner"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("approved"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Transfer"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
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
                                name: ::std::borrow::ToOwned::to_owned("id"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: true,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static MOCKERC721_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80\x80`@R4a\0\x19W`@Qa\x11I\x90\x81a\0g\x829\xF3[bF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01Ra7\xB7`\xF1\x1B`d\x82\x01R`\x84\x90\xFD\xFE`@`\x80\x81R`\x04\x90\x816\x10\x15a\0rW[\x90` `\x84\x92Q\x91bF\x1B\xCD`\xE5\x1B\x83R\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01Rtnor receive functions`X\x1B`d\x82\x01R\xFD[`\0\x805`\xE0\x1C\x80c\x01\xFF\xC9\xA7\x14a\x0B\x15W\x80c\x06\xFD\xDE\x03\x14a\neW\x80c\x08\x18\x12\xFC\x14a\n5W\x80c\t^\xA7\xB3\x14a\t\x80W\x80c#\xB8r\xDD\x14a\thW\x80cB\x84.\x0E\x14a\x08\x9EW\x80cL\xD8\x8Bv\x14a\x05hW\x80ccR!\x1E\x14a\x04\xFDW\x80cp\xA0\x821\x14a\x04\x8BW\x80c\x95\xD8\x9BA\x14a\x03\x9DW\x80c\xA2,\xB4e\x14a\x03\x18W\x80c\xB8\x8DO\xDE\x14a\x01\xCBW\x80c\xC8{V\xDD\x14a\x01oWc\xE9\x85\xE9\xC5\x14a\x01\x18WPa\0\x11V[\x904a\x01jW\x80`\x03\x196\x01\x12a\x01eW`\xFF\x81` \x93a\x017a\r)V[a\x01?a\rDV[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x83R`\x05\x87R\x83\x83 \x91\x16\x82R\x85R T\x91Q\x91\x16\x15\x15\x81R\xF3[a\x0B\xD7V[a\x0B\x87V[P\x904a\x01jW` \x80`\x03\x196\x01\x12a\x01eW\x91\x81Q\x92\x83\x91` \x83R``Q\x91\x82` \x85\x01R\x81[\x83\x81\x10a\x01\xB6WPP\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x81\x01\x03\x01\x90\xF3[`\x80\x81\x01Q\x87\x82\x01\x87\x01R\x86\x94P\x81\x01a\x01\x99V[P\x914a\x01jW`\x806`\x03\x19\x01\x12a\x01eWa\x01\xE6a\r)V[\x90a\x01\xEFa\rDV[\x91`D5\x90`d5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x03\x13W6`#\x82\x01\x12\x15a\x03\x0EWa\x02$\x906\x90`$\x81\x87\x015\x91\x01a\r\xE8V[\x91a\x020\x81\x86\x84a\x0E\xDFV[\x84;\x15\x94\x85\x15a\x02HW[\x87a\x02E\x87a\x10\xD4V[\x80\xF3[\x93\x94P\x91\x92\x90\x91\x90`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x90\x81;\x15a\x03\tW\x87` \x94a\x02\xA0\x89Q\x97\x88\x96\x87\x95\x86\x94c\n\x85\xBD\x01`\xE1\x1B\x9C\x8D\x87R3\x90\x87\x01R\x16`$\x85\x01R`D\x84\x01R`\x80`d\x84\x01R`\x84\x83\x01\x90a\x0C\xE9V[\x03\x92Z\xF1\x90\x81\x15a\x02\xFCWa\x02E\x93P\x84\x91a\x02\xCDW[P`\x01`\x01`\xE0\x1B\x03\x19\x16\x148\x80\x80\x80\x80a\x02;V[a\x02\xEF\x91P` =` \x11a\x02\xF5W[a\x02\xE7\x81\x83a\x0C\xB1V[\x81\x01\x90a\x10\xB4V[8a\x02\xB7V[P=a\x02\xDDV[PPPQ\x90=\x90\x82>=\x90\xFD[a\x10aV[a\r\x8FV[a\x0C'V[P\x904a\x01jW\x80`\x03\x196\x01\x12a\x01eWa\x032a\r)V[\x90`$5\x90\x81\x15\x15\x80\x92\x03a\x03\x99W3\x84R`\x05` R\x80\x84 \x92`\x01\x80`\xA0\x1B\x03\x16\x92\x83\x85R` R\x80\x84 `\xFF\x19\x81T\x16`\xFF\x84\x16\x17\x90UQ\x90\x81R\x7F\x170~\xAB9\xABa\x07\xE8\x89\x98E\xAD=Y\xBD\x96S\xF2\0\xF2 \x92\x04\x89\xCA+Y7il1` 3\x92\xA3\x80\xF3[\x83\x80\xFD[P4a\x01jW\x80`\x03\x196\x01\x12a\x01eW\x81Q\x91\x82\x82`\x01\x93`\x01T\x94a\x03\xC3\x86a\x0CwV[\x91\x82\x85R` \x96\x87`\x01\x82\x16\x91\x82`\0\x14a\x04dWPP`\x01\x14a\x04\x08W[PPPa\x04\x04\x92\x91a\x03\xF5\x91\x03\x85a\x0C\xB1V[Q\x92\x82\x84\x93\x84R\x83\x01\x90a\x0C\xE9V[\x03\x90\xF3[\x91\x90\x86\x93P`\x01\x83R\x7F\xB1\x0E-Rv\x12\x07;&\xEE\xCD\xFDq~j2\x0C\xF4KJ\xFA\xC2\xB0s-\x9F\xCB\xE2\xB7\xFA\x0C\xF6[\x82\x84\x10a\x04LWPPP\x82\x01\x01\x81a\x03\xF5a\x04\x04a\x03\xE2V[\x80T\x84\x8A\x01\x86\x01R\x88\x95P\x87\x94\x90\x93\x01\x92\x81\x01a\x043V[`\xFF\x19\x16\x87\x82\x01R\x93\x15\x15`\x05\x1B\x86\x01\x90\x93\x01\x93P\x84\x92Pa\x03\xF5\x91Pa\x04\x04\x90Pa\x03\xE2V[P\x824a\x01jW` 6`\x03\x19\x01\x12a\x01eW`\x01`\x01`\xA0\x1B\x03a\x04\xAEa\r)V[\x16\x90\x81\x15a\x04\xCBW` \x84\x80\x85\x85\x81R`\x03\x84R T\x90Q\x90\x81R\xF3[`d\x90` \x85Q\x91bF\x1B\xCD`\xE5\x1B\x83R\x82\x01R`\x0C`$\x82\x01RkZERO_ADDRESS`\xA0\x1B`D\x82\x01R\xFD[P\x82\x904a\x01jW` 6`\x03\x19\x01\x12a\x01eW\x815\x81R`\x02` R\x82\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x90\x81\x15a\x058WP` \x91Q\x90\x81R\xF3[`d\x90` \x84Q\x91bF\x1B\xCD`\xE5\x1B\x83R\x82\x01R`\n`$\x82\x01Ri\x13\x93\xD5\x17\xD3RS\x95\x11Q`\xB2\x1B`D\x82\x01R\xFD[P\x824a\x01jW\x82`\x03\x196\x01\x12a\x01eWg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x815\x81\x81\x11a\x03\x13Wa\x05\x99\x906\x90\x84\x01a\x0E\x84V[\x91`$5\x82\x81\x11a\x03\x13Wa\x05\xB1\x906\x90\x83\x01a\x0E\x84V[\x94`\xFF`\x06T\x16a\x08eWP\x82Q\x82\x81\x11a\x08RW\x80a\x05\xD1\x86Ta\x0CwV[\x94`\x1F\x95\x86\x81\x11a\x07\xE7W[P` \x90\x86\x83\x11`\x01\x14a\x07fW\x87\x92a\x07[W[PP\x81`\x01\x1B\x91`\0\x19\x90`\x03\x1B\x1C\x19\x16\x17\x84U[\x84Q\x91\x82\x11a\x07HWP`\x01\x91a\x06\x1E\x83Ta\x0CwV[\x81\x81\x11a\x06\xE6W[P` \x90\x82\x11`\x01\x14a\x06kW\x83\x94\x82\x93\x94\x92a\x06`W[PP`\0\x19`\x03\x83\x90\x1B\x1C\x19\x16\x90\x82\x1B\x17\x81U[`\xFF\x19`\x06T\x16\x17`\x06U\x80\xF3[\x01Q\x90P\x84\x80a\x06>V[\x82\x84R\x7F\xB1\x0E-Rv\x12\x07;&\xEE\xCD\xFDq~j2\x0C\xF4KJ\xFA\xC2\xB0s-\x9F\xCB\xE2\xB7\xFA\x0C\xF6\x90`\x1F\x19\x83\x16\x85[\x81\x81\x10a\x06\xD0WP\x95\x83\x85\x96\x97\x10a\x06\xB7W[PPP\x81\x1B\x01\x81Ua\x06RV[\x01Q`\0\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U\x84\x80\x80a\x06\xAAV[\x87\x83\x01Q\x84U\x92\x85\x01\x92` \x92\x83\x01\x92\x01a\x06\x97V[\x83\x85R\x7F\xB1\x0E-Rv\x12\x07;&\xEE\xCD\xFDq~j2\x0C\xF4KJ\xFA\xC2\xB0s-\x9F\xCB\xE2\xB7\xFA\x0C\xF6\x82\x80\x85\x01`\x05\x1C\x82\x01\x92` \x86\x10a\x07?W[\x01`\x05\x1C\x01\x90\x84\x90[\x82\x81\x10a\x074WPPa\x06&V[\x86\x81U\x01\x84\x90a\x07&V[\x92P\x81\x92a\x07\x1DV[cNH{q`\xE0\x1B\x84R`A\x90R`$\x83\xFD[\x01Q\x90P\x87\x80a\x05\xF2V[\x87\x80R\x7F)\r\xEC\xD9T\x8Bb\xA8\xD6\x03E\xA9\x888o\xC8K\xA6\xBC\x95H@\x08\xF66/\x93\x16\x0E\xF3\xE5c\x92P`\x1F\x19\x84\x16\x88[\x81\x81\x10a\x07\xCFWP\x90\x84`\x01\x95\x94\x93\x92\x10a\x07\xB6W[PPP\x81\x1B\x01\x84Ua\x06\x07V[\x01Q`\0\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U\x87\x80\x80a\x07\xA9V[\x92\x93` `\x01\x81\x92\x87\x86\x01Q\x81U\x01\x95\x01\x93\x01a\x07\x93V[\x90\x91P\x86\x80R\x7F)\r\xEC\xD9T\x8Bb\xA8\xD6\x03E\xA9\x888o\xC8K\xA6\xBC\x95H@\x08\xF66/\x93\x16\x0E\xF3\xE5c\x86\x80\x85\x01`\x05\x1C\x82\x01\x92` \x86\x10a\x08IW[\x90\x85\x94\x93\x92\x91\x01`\x05\x1C\x01\x90[\x81\x81\x10a\x08;WPa\x05\xDDV[\x88\x81U\x84\x93P`\x01\x01a\x08.V[\x92P\x81\x92a\x08!V[cNH{q`\xE0\x1B\x85R`A\x82R`$\x85\xFD[\x90` `d\x92Q\x91bF\x1B\xCD`\xE5\x1B\x83R\x82\x01R`\x13`$\x82\x01Rr\x10S\x14\x91PQ\x16W\xD2S\x92U\x12PS\x12V\x91Q`j\x1B`D\x82\x01R\xFD[P\x914a\x01jWa\x08\xAE6a\rZV[\x90\x92\x91a\x08\xBC\x82\x85\x83a\x0E\xDFV[\x83;\x15\x93\x84\x15a\x08\xD1W[\x86a\x02E\x86a\x10\xD4V[\x92\x93P\x90\x91`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x80;\x15a\x03\tW` \x92\x87`\xA4\x92\x88Q\x96\x87\x95\x86\x94c\n\x85\xBD\x01`\xE1\x1B\x9A\x8B\x87R3\x90\x87\x01R\x16`$\x85\x01R`D\x84\x01R`\x80`d\x84\x01R\x81`\x84\x84\x01RZ\xF1\x90\x81\x15a\x02\xFCWa\x02E\x93P\x84\x91a\tIW[P`\x01`\x01`\xE0\x1B\x03\x19\x16\x148\x80\x80\x80a\x08\xC7V[a\tb\x91P` =` \x11a\x02\xF5Wa\x02\xE7\x81\x83a\x0C\xB1V[8a\t4V[P4a\x01jWa\x02Ea\tz6a\rZV[\x91a\x0E\xDFV[P\x914a\x01jW\x81`\x03\x196\x01\x12a\x01eWa\t\x9Aa\r)V[`$5\x80\x85R`\x02` R\x83\x85 T\x90\x93\x91\x92`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x92\x903\x84\x14\x80\x15a\n\x16W[a\t\xCD\x90a\x0E\xA2V[\x85\x87R` R\x85 \x92\x16\x91\x82k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA0\x1B\x82T\x16\x17\x90U\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x84\x80\xA4\x80\xF3[P\x83\x87R`\x05` \x90\x81R\x82\x88 3\x89R\x90R\x81\x87 T`\xFF\x16a\t\xC4V[P4a\x01jW` 6`\x03\x19\x01\x12a\x01eW\x825\x81R` \x92\x83R\x81\x90 T\x90Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R\xF3[P4a\x01jW\x80`\x03\x196\x01\x12a\x01eW\x81Q\x91\x82\x82\x83Ta\n\x86\x81a\x0CwV[\x90\x81\x84R` \x95`\x01\x91\x87`\x01\x82\x16\x91\x82`\0\x14a\x04dWPP`\x01\x14a\n\xBAWPPPa\x04\x04\x92\x91a\x03\xF5\x91\x03\x85a\x0C\xB1V[\x91\x90\x86\x93P\x82\x80R\x7F)\r\xEC\xD9T\x8Bb\xA8\xD6\x03E\xA9\x888o\xC8K\xA6\xBC\x95H@\x08\xF66/\x93\x16\x0E\xF3\xE5c[\x82\x84\x10a\n\xFDWPPP\x82\x01\x01\x81a\x03\xF5a\x04\x04a\x03\xE2V[\x80T\x84\x8A\x01\x86\x01R\x88\x95P\x87\x94\x90\x93\x01\x92\x81\x01a\n\xE4V[P\x914a\x0B\x87W` 6`\x03\x19\x01\x12a\x01eW5\x90c\xFF\xFF\xFF\xFF`\xE0\x1B\x82\x16\x80\x92\x03a\x0B\x83W` \x92Pc\x01\xFF\xC9\xA7`\xE0\x1B\x82\x14\x91\x82\x15a\x0BrW[\x82\x15a\x0BaW[PQ\x90\x15\x15\x81R\xF3[c[^\x13\x9F`\xE0\x1B\x14\x91P8a\x0BXV[c\x80\xACX\xCD`\xE0\x1B\x81\x14\x92Pa\x0BQV[\x82\x80\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01Ra7\xB7`\xF1\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[\x90`\x01\x82\x81\x1C\x92\x16\x80\x15a\x0C\xA7W[` \x83\x10\x14a\x0C\x91WV[cNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[\x91`\x7F\x16\x91a\x0C\x86V[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x0C\xD3W`@RV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x91\x90\x82Q\x92\x83\x82R`\0[\x84\x81\x10a\r\x15WPP\x82`\0` \x80\x94\x95\x84\x01\x01R`\x1F\x80\x19\x91\x01\x16\x01\x01\x90V[` \x81\x83\x01\x81\x01Q\x84\x83\x01\x82\x01R\x01a\x0C\xF4V[`\x045\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\r?WV[`\0\x80\xFD[`$5\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\r?WV[``\x90`\x03\x19\x01\x12a\x01eW`\x01`\x01`\xA0\x1B\x03\x90`\x045\x82\x81\x16\x81\x03a\r?W\x91`$5\x90\x81\x16\x81\x03a\r?W\x90`D5\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x90\xFD[\x92\x91\x92g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x0C\xD3W`@Q\x91a\x0E\x12`\x1F\x82\x01`\x1F\x19\x16` \x01\x84a\x0C\xB1V[\x82\x94\x81\x84R\x81\x83\x01\x11a\x0E/W\x82\x81` \x93\x84`\0\x96\x017\x01\x01RV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R`\x84\x90\xFD[\x90\x80`\x1F\x83\x01\x12\x15a\x03\x0EW\x81` a\x0E\x9F\x935\x91\x01a\r\xE8V[\x90V[\x15a\x0E\xA9WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rm\x13\x93\xD5\x17\xD0UU\x12\x13\xD4\x92V\x91Q`\x92\x1B`D\x82\x01R`d\x90\xFD[`\0\x83\x81R`\x02` \x90\x81R`@\x80\x83 T`\x01`\x01`\xA0\x1B\x03\x95\x94\x86\x16\x94\x90\x86\x16\x85\x03a\x100W\x85\x16\x94\x85\x15a\x0F\xF8Wa\x0F,\x90\x853\x14\x90\x81\x15a\x0F\xDBW[\x81\x15a\x0F\xC5W[Pa\x0E\xA2V[\x83\x83R`\x03\x82R\x80\x83 \x80T\x90\x81\x15a\x0F\xB1W`\0\x19\x91\x82\x01\x90U\x85\x84R`\x03\x83R\x81\x84 \x80T\x90\x91\x81\x14a\x0F\xB1W`\x01\x01\x90U\x85\x83R`\x02\x82R\x80\x83 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16\x87\x17\x90\x91U`\x04\x90\x92R\x82 \x80T\x90\x91\x16\x90U\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x90\x80\xA4V[cNH{q`\xE0\x1B\x85R`\x11`\x04R`$\x85\xFD[\x90P\x87\x85R`\x04\x84R\x82\x85 T\x163\x148a\x0F&V[\x86\x86R`\x05\x85R\x83\x86 3\x87R\x85R\x83\x86 T`\xFF\x16\x91Pa\x0F\x1FV[\x81QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x84\x90R`\x11`$\x82\x01Rp\x12S\x95\x90S\x12Q\x17\xD4\x91P\xD2T\x12QS\x95`z\x1B`D\x82\x01R`d\x90\xFD[\x81QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x84\x90R`\n`$\x82\x01RiWRONG_FROM`\xB0\x1B`D\x82\x01R`d\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01Rd code`\xD8\x1B`d\x82\x01R`\x84\x90\xFD[\x90\x81` \x91\x03\x12a\x01eWQ`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x03a\r?W\x90V[\x15a\x10\xDBWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01Ro\x15S\x94\xD0Q\x91W\xD4\x91P\xD2T\x12QS\x95`\x82\x1B`D\x82\x01R`d\x90\xFD\xFE\xA2dipfsX\"\x12 M\xCB\xBAiY\x9F&Y2c#\xD4V\xC2H* h\x9C\xFB\xD7\xE6\xA3\xE1\xEC\xE5\xE8\"`\x7F(\xA2dsolcC\0\x08\x16\x003";
    /// The bytecode of the contract.
    pub static MOCKERC721_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`@`\x80\x81R`\x04\x90\x816\x10\x15a\0rW[\x90` `\x84\x92Q\x91bF\x1B\xCD`\xE5\x1B\x83R\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01Rtnor receive functions`X\x1B`d\x82\x01R\xFD[`\0\x805`\xE0\x1C\x80c\x01\xFF\xC9\xA7\x14a\x0B\x15W\x80c\x06\xFD\xDE\x03\x14a\neW\x80c\x08\x18\x12\xFC\x14a\n5W\x80c\t^\xA7\xB3\x14a\t\x80W\x80c#\xB8r\xDD\x14a\thW\x80cB\x84.\x0E\x14a\x08\x9EW\x80cL\xD8\x8Bv\x14a\x05hW\x80ccR!\x1E\x14a\x04\xFDW\x80cp\xA0\x821\x14a\x04\x8BW\x80c\x95\xD8\x9BA\x14a\x03\x9DW\x80c\xA2,\xB4e\x14a\x03\x18W\x80c\xB8\x8DO\xDE\x14a\x01\xCBW\x80c\xC8{V\xDD\x14a\x01oWc\xE9\x85\xE9\xC5\x14a\x01\x18WPa\0\x11V[\x904a\x01jW\x80`\x03\x196\x01\x12a\x01eW`\xFF\x81` \x93a\x017a\r)V[a\x01?a\rDV[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x83R`\x05\x87R\x83\x83 \x91\x16\x82R\x85R T\x91Q\x91\x16\x15\x15\x81R\xF3[a\x0B\xD7V[a\x0B\x87V[P\x904a\x01jW` \x80`\x03\x196\x01\x12a\x01eW\x91\x81Q\x92\x83\x91` \x83R``Q\x91\x82` \x85\x01R\x81[\x83\x81\x10a\x01\xB6WPP\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x81\x01\x03\x01\x90\xF3[`\x80\x81\x01Q\x87\x82\x01\x87\x01R\x86\x94P\x81\x01a\x01\x99V[P\x914a\x01jW`\x806`\x03\x19\x01\x12a\x01eWa\x01\xE6a\r)V[\x90a\x01\xEFa\rDV[\x91`D5\x90`d5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x03\x13W6`#\x82\x01\x12\x15a\x03\x0EWa\x02$\x906\x90`$\x81\x87\x015\x91\x01a\r\xE8V[\x91a\x020\x81\x86\x84a\x0E\xDFV[\x84;\x15\x94\x85\x15a\x02HW[\x87a\x02E\x87a\x10\xD4V[\x80\xF3[\x93\x94P\x91\x92\x90\x91\x90`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x90\x81;\x15a\x03\tW\x87` \x94a\x02\xA0\x89Q\x97\x88\x96\x87\x95\x86\x94c\n\x85\xBD\x01`\xE1\x1B\x9C\x8D\x87R3\x90\x87\x01R\x16`$\x85\x01R`D\x84\x01R`\x80`d\x84\x01R`\x84\x83\x01\x90a\x0C\xE9V[\x03\x92Z\xF1\x90\x81\x15a\x02\xFCWa\x02E\x93P\x84\x91a\x02\xCDW[P`\x01`\x01`\xE0\x1B\x03\x19\x16\x148\x80\x80\x80\x80a\x02;V[a\x02\xEF\x91P` =` \x11a\x02\xF5W[a\x02\xE7\x81\x83a\x0C\xB1V[\x81\x01\x90a\x10\xB4V[8a\x02\xB7V[P=a\x02\xDDV[PPPQ\x90=\x90\x82>=\x90\xFD[a\x10aV[a\r\x8FV[a\x0C'V[P\x904a\x01jW\x80`\x03\x196\x01\x12a\x01eWa\x032a\r)V[\x90`$5\x90\x81\x15\x15\x80\x92\x03a\x03\x99W3\x84R`\x05` R\x80\x84 \x92`\x01\x80`\xA0\x1B\x03\x16\x92\x83\x85R` R\x80\x84 `\xFF\x19\x81T\x16`\xFF\x84\x16\x17\x90UQ\x90\x81R\x7F\x170~\xAB9\xABa\x07\xE8\x89\x98E\xAD=Y\xBD\x96S\xF2\0\xF2 \x92\x04\x89\xCA+Y7il1` 3\x92\xA3\x80\xF3[\x83\x80\xFD[P4a\x01jW\x80`\x03\x196\x01\x12a\x01eW\x81Q\x91\x82\x82`\x01\x93`\x01T\x94a\x03\xC3\x86a\x0CwV[\x91\x82\x85R` \x96\x87`\x01\x82\x16\x91\x82`\0\x14a\x04dWPP`\x01\x14a\x04\x08W[PPPa\x04\x04\x92\x91a\x03\xF5\x91\x03\x85a\x0C\xB1V[Q\x92\x82\x84\x93\x84R\x83\x01\x90a\x0C\xE9V[\x03\x90\xF3[\x91\x90\x86\x93P`\x01\x83R\x7F\xB1\x0E-Rv\x12\x07;&\xEE\xCD\xFDq~j2\x0C\xF4KJ\xFA\xC2\xB0s-\x9F\xCB\xE2\xB7\xFA\x0C\xF6[\x82\x84\x10a\x04LWPPP\x82\x01\x01\x81a\x03\xF5a\x04\x04a\x03\xE2V[\x80T\x84\x8A\x01\x86\x01R\x88\x95P\x87\x94\x90\x93\x01\x92\x81\x01a\x043V[`\xFF\x19\x16\x87\x82\x01R\x93\x15\x15`\x05\x1B\x86\x01\x90\x93\x01\x93P\x84\x92Pa\x03\xF5\x91Pa\x04\x04\x90Pa\x03\xE2V[P\x824a\x01jW` 6`\x03\x19\x01\x12a\x01eW`\x01`\x01`\xA0\x1B\x03a\x04\xAEa\r)V[\x16\x90\x81\x15a\x04\xCBW` \x84\x80\x85\x85\x81R`\x03\x84R T\x90Q\x90\x81R\xF3[`d\x90` \x85Q\x91bF\x1B\xCD`\xE5\x1B\x83R\x82\x01R`\x0C`$\x82\x01RkZERO_ADDRESS`\xA0\x1B`D\x82\x01R\xFD[P\x82\x904a\x01jW` 6`\x03\x19\x01\x12a\x01eW\x815\x81R`\x02` R\x82\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x90\x81\x15a\x058WP` \x91Q\x90\x81R\xF3[`d\x90` \x84Q\x91bF\x1B\xCD`\xE5\x1B\x83R\x82\x01R`\n`$\x82\x01Ri\x13\x93\xD5\x17\xD3RS\x95\x11Q`\xB2\x1B`D\x82\x01R\xFD[P\x824a\x01jW\x82`\x03\x196\x01\x12a\x01eWg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x815\x81\x81\x11a\x03\x13Wa\x05\x99\x906\x90\x84\x01a\x0E\x84V[\x91`$5\x82\x81\x11a\x03\x13Wa\x05\xB1\x906\x90\x83\x01a\x0E\x84V[\x94`\xFF`\x06T\x16a\x08eWP\x82Q\x82\x81\x11a\x08RW\x80a\x05\xD1\x86Ta\x0CwV[\x94`\x1F\x95\x86\x81\x11a\x07\xE7W[P` \x90\x86\x83\x11`\x01\x14a\x07fW\x87\x92a\x07[W[PP\x81`\x01\x1B\x91`\0\x19\x90`\x03\x1B\x1C\x19\x16\x17\x84U[\x84Q\x91\x82\x11a\x07HWP`\x01\x91a\x06\x1E\x83Ta\x0CwV[\x81\x81\x11a\x06\xE6W[P` \x90\x82\x11`\x01\x14a\x06kW\x83\x94\x82\x93\x94\x92a\x06`W[PP`\0\x19`\x03\x83\x90\x1B\x1C\x19\x16\x90\x82\x1B\x17\x81U[`\xFF\x19`\x06T\x16\x17`\x06U\x80\xF3[\x01Q\x90P\x84\x80a\x06>V[\x82\x84R\x7F\xB1\x0E-Rv\x12\x07;&\xEE\xCD\xFDq~j2\x0C\xF4KJ\xFA\xC2\xB0s-\x9F\xCB\xE2\xB7\xFA\x0C\xF6\x90`\x1F\x19\x83\x16\x85[\x81\x81\x10a\x06\xD0WP\x95\x83\x85\x96\x97\x10a\x06\xB7W[PPP\x81\x1B\x01\x81Ua\x06RV[\x01Q`\0\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U\x84\x80\x80a\x06\xAAV[\x87\x83\x01Q\x84U\x92\x85\x01\x92` \x92\x83\x01\x92\x01a\x06\x97V[\x83\x85R\x7F\xB1\x0E-Rv\x12\x07;&\xEE\xCD\xFDq~j2\x0C\xF4KJ\xFA\xC2\xB0s-\x9F\xCB\xE2\xB7\xFA\x0C\xF6\x82\x80\x85\x01`\x05\x1C\x82\x01\x92` \x86\x10a\x07?W[\x01`\x05\x1C\x01\x90\x84\x90[\x82\x81\x10a\x074WPPa\x06&V[\x86\x81U\x01\x84\x90a\x07&V[\x92P\x81\x92a\x07\x1DV[cNH{q`\xE0\x1B\x84R`A\x90R`$\x83\xFD[\x01Q\x90P\x87\x80a\x05\xF2V[\x87\x80R\x7F)\r\xEC\xD9T\x8Bb\xA8\xD6\x03E\xA9\x888o\xC8K\xA6\xBC\x95H@\x08\xF66/\x93\x16\x0E\xF3\xE5c\x92P`\x1F\x19\x84\x16\x88[\x81\x81\x10a\x07\xCFWP\x90\x84`\x01\x95\x94\x93\x92\x10a\x07\xB6W[PPP\x81\x1B\x01\x84Ua\x06\x07V[\x01Q`\0\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U\x87\x80\x80a\x07\xA9V[\x92\x93` `\x01\x81\x92\x87\x86\x01Q\x81U\x01\x95\x01\x93\x01a\x07\x93V[\x90\x91P\x86\x80R\x7F)\r\xEC\xD9T\x8Bb\xA8\xD6\x03E\xA9\x888o\xC8K\xA6\xBC\x95H@\x08\xF66/\x93\x16\x0E\xF3\xE5c\x86\x80\x85\x01`\x05\x1C\x82\x01\x92` \x86\x10a\x08IW[\x90\x85\x94\x93\x92\x91\x01`\x05\x1C\x01\x90[\x81\x81\x10a\x08;WPa\x05\xDDV[\x88\x81U\x84\x93P`\x01\x01a\x08.V[\x92P\x81\x92a\x08!V[cNH{q`\xE0\x1B\x85R`A\x82R`$\x85\xFD[\x90` `d\x92Q\x91bF\x1B\xCD`\xE5\x1B\x83R\x82\x01R`\x13`$\x82\x01Rr\x10S\x14\x91PQ\x16W\xD2S\x92U\x12PS\x12V\x91Q`j\x1B`D\x82\x01R\xFD[P\x914a\x01jWa\x08\xAE6a\rZV[\x90\x92\x91a\x08\xBC\x82\x85\x83a\x0E\xDFV[\x83;\x15\x93\x84\x15a\x08\xD1W[\x86a\x02E\x86a\x10\xD4V[\x92\x93P\x90\x91`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x80;\x15a\x03\tW` \x92\x87`\xA4\x92\x88Q\x96\x87\x95\x86\x94c\n\x85\xBD\x01`\xE1\x1B\x9A\x8B\x87R3\x90\x87\x01R\x16`$\x85\x01R`D\x84\x01R`\x80`d\x84\x01R\x81`\x84\x84\x01RZ\xF1\x90\x81\x15a\x02\xFCWa\x02E\x93P\x84\x91a\tIW[P`\x01`\x01`\xE0\x1B\x03\x19\x16\x148\x80\x80\x80a\x08\xC7V[a\tb\x91P` =` \x11a\x02\xF5Wa\x02\xE7\x81\x83a\x0C\xB1V[8a\t4V[P4a\x01jWa\x02Ea\tz6a\rZV[\x91a\x0E\xDFV[P\x914a\x01jW\x81`\x03\x196\x01\x12a\x01eWa\t\x9Aa\r)V[`$5\x80\x85R`\x02` R\x83\x85 T\x90\x93\x91\x92`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x92\x903\x84\x14\x80\x15a\n\x16W[a\t\xCD\x90a\x0E\xA2V[\x85\x87R` R\x85 \x92\x16\x91\x82k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA0\x1B\x82T\x16\x17\x90U\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x84\x80\xA4\x80\xF3[P\x83\x87R`\x05` \x90\x81R\x82\x88 3\x89R\x90R\x81\x87 T`\xFF\x16a\t\xC4V[P4a\x01jW` 6`\x03\x19\x01\x12a\x01eW\x825\x81R` \x92\x83R\x81\x90 T\x90Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R\xF3[P4a\x01jW\x80`\x03\x196\x01\x12a\x01eW\x81Q\x91\x82\x82\x83Ta\n\x86\x81a\x0CwV[\x90\x81\x84R` \x95`\x01\x91\x87`\x01\x82\x16\x91\x82`\0\x14a\x04dWPP`\x01\x14a\n\xBAWPPPa\x04\x04\x92\x91a\x03\xF5\x91\x03\x85a\x0C\xB1V[\x91\x90\x86\x93P\x82\x80R\x7F)\r\xEC\xD9T\x8Bb\xA8\xD6\x03E\xA9\x888o\xC8K\xA6\xBC\x95H@\x08\xF66/\x93\x16\x0E\xF3\xE5c[\x82\x84\x10a\n\xFDWPPP\x82\x01\x01\x81a\x03\xF5a\x04\x04a\x03\xE2V[\x80T\x84\x8A\x01\x86\x01R\x88\x95P\x87\x94\x90\x93\x01\x92\x81\x01a\n\xE4V[P\x914a\x0B\x87W` 6`\x03\x19\x01\x12a\x01eW5\x90c\xFF\xFF\xFF\xFF`\xE0\x1B\x82\x16\x80\x92\x03a\x0B\x83W` \x92Pc\x01\xFF\xC9\xA7`\xE0\x1B\x82\x14\x91\x82\x15a\x0BrW[\x82\x15a\x0BaW[PQ\x90\x15\x15\x81R\xF3[c[^\x13\x9F`\xE0\x1B\x14\x91P8a\x0BXV[c\x80\xACX\xCD`\xE0\x1B\x81\x14\x92Pa\x0BQV[\x82\x80\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01Ra7\xB7`\xF1\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[\x90`\x01\x82\x81\x1C\x92\x16\x80\x15a\x0C\xA7W[` \x83\x10\x14a\x0C\x91WV[cNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[\x91`\x7F\x16\x91a\x0C\x86V[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x0C\xD3W`@RV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x91\x90\x82Q\x92\x83\x82R`\0[\x84\x81\x10a\r\x15WPP\x82`\0` \x80\x94\x95\x84\x01\x01R`\x1F\x80\x19\x91\x01\x16\x01\x01\x90V[` \x81\x83\x01\x81\x01Q\x84\x83\x01\x82\x01R\x01a\x0C\xF4V[`\x045\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\r?WV[`\0\x80\xFD[`$5\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\r?WV[``\x90`\x03\x19\x01\x12a\x01eW`\x01`\x01`\xA0\x1B\x03\x90`\x045\x82\x81\x16\x81\x03a\r?W\x91`$5\x90\x81\x16\x81\x03a\r?W\x90`D5\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x90\xFD[\x92\x91\x92g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x0C\xD3W`@Q\x91a\x0E\x12`\x1F\x82\x01`\x1F\x19\x16` \x01\x84a\x0C\xB1V[\x82\x94\x81\x84R\x81\x83\x01\x11a\x0E/W\x82\x81` \x93\x84`\0\x96\x017\x01\x01RV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R`\x84\x90\xFD[\x90\x80`\x1F\x83\x01\x12\x15a\x03\x0EW\x81` a\x0E\x9F\x935\x91\x01a\r\xE8V[\x90V[\x15a\x0E\xA9WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rm\x13\x93\xD5\x17\xD0UU\x12\x13\xD4\x92V\x91Q`\x92\x1B`D\x82\x01R`d\x90\xFD[`\0\x83\x81R`\x02` \x90\x81R`@\x80\x83 T`\x01`\x01`\xA0\x1B\x03\x95\x94\x86\x16\x94\x90\x86\x16\x85\x03a\x100W\x85\x16\x94\x85\x15a\x0F\xF8Wa\x0F,\x90\x853\x14\x90\x81\x15a\x0F\xDBW[\x81\x15a\x0F\xC5W[Pa\x0E\xA2V[\x83\x83R`\x03\x82R\x80\x83 \x80T\x90\x81\x15a\x0F\xB1W`\0\x19\x91\x82\x01\x90U\x85\x84R`\x03\x83R\x81\x84 \x80T\x90\x91\x81\x14a\x0F\xB1W`\x01\x01\x90U\x85\x83R`\x02\x82R\x80\x83 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16\x87\x17\x90\x91U`\x04\x90\x92R\x82 \x80T\x90\x91\x16\x90U\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x90\x80\xA4V[cNH{q`\xE0\x1B\x85R`\x11`\x04R`$\x85\xFD[\x90P\x87\x85R`\x04\x84R\x82\x85 T\x163\x148a\x0F&V[\x86\x86R`\x05\x85R\x83\x86 3\x87R\x85R\x83\x86 T`\xFF\x16\x91Pa\x0F\x1FV[\x81QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x84\x90R`\x11`$\x82\x01Rp\x12S\x95\x90S\x12Q\x17\xD4\x91P\xD2T\x12QS\x95`z\x1B`D\x82\x01R`d\x90\xFD[\x81QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x84\x90R`\n`$\x82\x01RiWRONG_FROM`\xB0\x1B`D\x82\x01R`d\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01Rd code`\xD8\x1B`d\x82\x01R`\x84\x90\xFD[\x90\x81` \x91\x03\x12a\x01eWQ`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x03a\r?W\x90V[\x15a\x10\xDBWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01Ro\x15S\x94\xD0Q\x91W\xD4\x91P\xD2T\x12QS\x95`\x82\x1B`D\x82\x01R`d\x90\xFD\xFE\xA2dipfsX\"\x12 M\xCB\xBAiY\x9F&Y2c#\xD4V\xC2H* h\x9C\xFB\xD7\xE6\xA3\xE1\xEC\xE5\xE8\"`\x7F(\xA2dsolcC\0\x08\x16\x003";
    /// The deployed bytecode of the contract.
    pub static MOCKERC721_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct MockERC721<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MockERC721<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MockERC721<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MockERC721<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MockERC721<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MockERC721))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MockERC721<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                MOCKERC721_ABI.clone(),
                client,
            ))
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
                MOCKERC721_ABI.clone(),
                MOCKERC721_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `approve` (0x095ea7b3) function
        pub fn approve(
            &self,
            spender: ::ethers::core::types::Address,
            id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([9, 94, 167, 179], (spender, id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `balanceOf` (0x70a08231) function
        pub fn balance_of(
            &self,
            owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getApproved` (0x081812fc) function
        pub fn get_approved(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([8, 24, 18, 252], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0x4cd88b76) function
        pub fn initialize(
            &self,
            name: ::std::string::String,
            symbol: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([76, 216, 139, 118], (name, symbol))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isApprovedForAll` (0xe985e9c5) function
        pub fn is_approved_for_all(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([233, 133, 233, 197], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `name` (0x06fdde03) function
        pub fn name(&self) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([6, 253, 222, 3], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ownerOf` (0x6352211e) function
        pub fn owner_of(
            &self,
            id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([99, 82, 33, 30], id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `safeTransferFrom` (0x42842e0e) function
        pub fn safe_transfer_from(
            &self,
            from: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([66, 132, 46, 14], (from, to, id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `safeTransferFrom` (0xb88d4fde) function
        pub fn safe_transfer_from_with_from_and_to_and_data(
            &self,
            from: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            id: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([184, 141, 79, 222], (from, to, id, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setApprovalForAll` (0xa22cb465) function
        pub fn set_approval_for_all(
            &self,
            operator: ::ethers::core::types::Address,
            approved: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([162, 44, 180, 101], (operator, approved))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `supportsInterface` (0x01ffc9a7) function
        pub fn supports_interface(
            &self,
            interface_id: [u8; 4],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([1, 255, 201, 167], interface_id)
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
        ///Calls the contract's `tokenURI` (0xc87b56dd) function
        pub fn token_uri(
            &self,
            id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([200, 123, 86, 221], id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferFrom` (0x23b872dd) function
        pub fn transfer_from(
            &self,
            from: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([35, 184, 114, 221], (from, to, id))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `Approval` event
        pub fn approval_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ApprovalFilter> {
            self.0.event()
        }
        ///Gets the contract's `ApprovalForAll` event
        pub fn approval_for_all_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ApprovalForAllFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `Transfer` event
        pub fn transfer_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, TransferFilter> {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, MockERC721Events> {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for MockERC721<M> {
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
        Hash,
    )]
    #[ethevent(name = "Approval", abi = "Approval(address,address,uint256)")]
    pub struct ApprovalFilter {
        #[ethevent(indexed)]
        pub owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub spender: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub id: ::ethers::core::types::U256,
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
        Hash,
    )]
    #[ethevent(name = "ApprovalForAll", abi = "ApprovalForAll(address,address,bool)")]
    pub struct ApprovalForAllFilter {
        #[ethevent(indexed)]
        pub owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub operator: ::ethers::core::types::Address,
        pub approved: bool,
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
        Hash,
    )]
    #[ethevent(name = "Transfer", abi = "Transfer(address,address,uint256)")]
    pub struct TransferFilter {
        #[ethevent(indexed)]
        pub from: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub id: ::ethers::core::types::U256,
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
        Hash,
    )]
    pub enum MockERC721Events {
        ApprovalFilter(ApprovalFilter),
        ApprovalForAllFilter(ApprovalForAllFilter),
        TransferFilter(TransferFilter),
    }
    impl ::ethers::contract::EthLogDecode for MockERC721Events {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(MockERC721Events::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = ApprovalForAllFilter::decode_log(log) {
                return Ok(MockERC721Events::ApprovalForAllFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(MockERC721Events::TransferFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for MockERC721Events {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ApprovalFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ApprovalForAllFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ApprovalFilter> for MockERC721Events {
        fn from(value: ApprovalFilter) -> Self {
            Self::ApprovalFilter(value)
        }
    }
    impl ::core::convert::From<ApprovalForAllFilter> for MockERC721Events {
        fn from(value: ApprovalForAllFilter) -> Self {
            Self::ApprovalForAllFilter(value)
        }
    }
    impl ::core::convert::From<TransferFilter> for MockERC721Events {
        fn from(value: TransferFilter) -> Self {
            Self::TransferFilter(value)
        }
    }
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
        Hash,
    )]
    #[ethcall(name = "approve", abi = "approve(address,uint256)")]
    pub struct ApproveCall {
        pub spender: ::ethers::core::types::Address,
        pub id: ::ethers::core::types::U256,
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
        Hash,
    )]
    #[ethcall(name = "balanceOf", abi = "balanceOf(address)")]
    pub struct BalanceOfCall {
        pub owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getApproved` function with signature `getApproved(uint256)` and selector `0x081812fc`
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
    #[ethcall(name = "getApproved", abi = "getApproved(uint256)")]
    pub struct GetApprovedCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `initialize` function with signature `initialize(string,string)` and selector `0x4cd88b76`
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
    #[ethcall(name = "initialize", abi = "initialize(string,string)")]
    pub struct InitializeCall {
        pub name: ::std::string::String,
        pub symbol: ::std::string::String,
    }
    ///Container type for all input parameters for the `isApprovedForAll` function with signature `isApprovedForAll(address,address)` and selector `0xe985e9c5`
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
    #[ethcall(name = "isApprovedForAll", abi = "isApprovedForAll(address,address)")]
    pub struct IsApprovedForAllCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
    );
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
        Hash,
    )]
    #[ethcall(name = "name", abi = "name()")]
    pub struct NameCall;
    ///Container type for all input parameters for the `ownerOf` function with signature `ownerOf(uint256)` and selector `0x6352211e`
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
    #[ethcall(name = "ownerOf", abi = "ownerOf(uint256)")]
    pub struct OwnerOfCall {
        pub id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `safeTransferFrom` function with signature `safeTransferFrom(address,address,uint256)` and selector `0x42842e0e`
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
    #[ethcall(
        name = "safeTransferFrom",
        abi = "safeTransferFrom(address,address,uint256)"
    )]
    pub struct SafeTransferFromCall {
        pub from: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `safeTransferFrom` function with signature `safeTransferFrom(address,address,uint256,bytes)` and selector `0xb88d4fde`
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
    #[ethcall(
        name = "safeTransferFrom",
        abi = "safeTransferFrom(address,address,uint256,bytes)"
    )]
    pub struct SafeTransferFromWithFromAndToAndDataCall {
        pub from: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub id: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `setApprovalForAll` function with signature `setApprovalForAll(address,bool)` and selector `0xa22cb465`
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
    #[ethcall(name = "setApprovalForAll", abi = "setApprovalForAll(address,bool)")]
    pub struct SetApprovalForAllCall {
        pub operator: ::ethers::core::types::Address,
        pub approved: bool,
    }
    ///Container type for all input parameters for the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `0x01ffc9a7`
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
    #[ethcall(name = "supportsInterface", abi = "supportsInterface(bytes4)")]
    pub struct SupportsInterfaceCall {
        pub interface_id: [u8; 4],
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
        Hash,
    )]
    #[ethcall(name = "symbol", abi = "symbol()")]
    pub struct SymbolCall;
    ///Container type for all input parameters for the `tokenURI` function with signature `tokenURI(uint256)` and selector `0xc87b56dd`
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
    #[ethcall(name = "tokenURI", abi = "tokenURI(uint256)")]
    pub struct TokenURICall {
        pub id: ::ethers::core::types::U256,
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
        Hash,
    )]
    #[ethcall(name = "transferFrom", abi = "transferFrom(address,address,uint256)")]
    pub struct TransferFromCall {
        pub from: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub id: ::ethers::core::types::U256,
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
        Hash,
    )]
    pub enum MockERC721Calls {
        Approve(ApproveCall),
        BalanceOf(BalanceOfCall),
        GetApproved(GetApprovedCall),
        Initialize(InitializeCall),
        IsApprovedForAll(IsApprovedForAllCall),
        Name(NameCall),
        OwnerOf(OwnerOfCall),
        SafeTransferFrom(SafeTransferFromCall),
        SafeTransferFromWithFromAndToAndData(SafeTransferFromWithFromAndToAndDataCall),
        SetApprovalForAll(SetApprovalForAllCall),
        SupportsInterface(SupportsInterfaceCall),
        Symbol(SymbolCall),
        TokenURI(TokenURICall),
        TransferFrom(TransferFromCall),
    }
    impl ::ethers::core::abi::AbiDecode for MockERC721Calls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <ApproveCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Approve(decoded));
            }
            if let Ok(decoded) = <BalanceOfCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BalanceOf(decoded));
            }
            if let Ok(decoded) = <GetApprovedCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetApproved(decoded));
            }
            if let Ok(decoded) = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded) =
                <IsApprovedForAllCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::IsApprovedForAll(decoded));
            }
            if let Ok(decoded) = <NameCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Name(decoded));
            }
            if let Ok(decoded) = <OwnerOfCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::OwnerOf(decoded));
            }
            if let Ok(decoded) =
                <SafeTransferFromCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SafeTransferFrom(decoded));
            }
            if let Ok(decoded) =
                <SafeTransferFromWithFromAndToAndDataCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::SafeTransferFromWithFromAndToAndData(decoded));
            }
            if let Ok(decoded) =
                <SetApprovalForAllCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetApprovalForAll(decoded));
            }
            if let Ok(decoded) =
                <SupportsInterfaceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SupportsInterface(decoded));
            }
            if let Ok(decoded) = <SymbolCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Symbol(decoded));
            }
            if let Ok(decoded) = <TokenURICall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TokenURI(decoded));
            }
            if let Ok(decoded) = <TransferFromCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TransferFrom(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MockERC721Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Approve(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BalanceOf(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetApproved(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Initialize(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IsApprovedForAll(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Name(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OwnerOf(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SafeTransferFrom(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SafeTransferFromWithFromAndToAndData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetApprovalForAll(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SupportsInterface(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Symbol(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TokenURI(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TransferFrom(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for MockERC721Calls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Approve(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetApproved(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsApprovedForAll(element) => ::core::fmt::Display::fmt(element, f),
                Self::Name(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnerOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::SafeTransferFrom(element) => ::core::fmt::Display::fmt(element, f),
                Self::SafeTransferFromWithFromAndToAndData(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetApprovalForAll(element) => ::core::fmt::Display::fmt(element, f),
                Self::SupportsInterface(element) => ::core::fmt::Display::fmt(element, f),
                Self::Symbol(element) => ::core::fmt::Display::fmt(element, f),
                Self::TokenURI(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFrom(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ApproveCall> for MockERC721Calls {
        fn from(value: ApproveCall) -> Self {
            Self::Approve(value)
        }
    }
    impl ::core::convert::From<BalanceOfCall> for MockERC721Calls {
        fn from(value: BalanceOfCall) -> Self {
            Self::BalanceOf(value)
        }
    }
    impl ::core::convert::From<GetApprovedCall> for MockERC721Calls {
        fn from(value: GetApprovedCall) -> Self {
            Self::GetApproved(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for MockERC721Calls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<IsApprovedForAllCall> for MockERC721Calls {
        fn from(value: IsApprovedForAllCall) -> Self {
            Self::IsApprovedForAll(value)
        }
    }
    impl ::core::convert::From<NameCall> for MockERC721Calls {
        fn from(value: NameCall) -> Self {
            Self::Name(value)
        }
    }
    impl ::core::convert::From<OwnerOfCall> for MockERC721Calls {
        fn from(value: OwnerOfCall) -> Self {
            Self::OwnerOf(value)
        }
    }
    impl ::core::convert::From<SafeTransferFromCall> for MockERC721Calls {
        fn from(value: SafeTransferFromCall) -> Self {
            Self::SafeTransferFrom(value)
        }
    }
    impl ::core::convert::From<SafeTransferFromWithFromAndToAndDataCall> for MockERC721Calls {
        fn from(value: SafeTransferFromWithFromAndToAndDataCall) -> Self {
            Self::SafeTransferFromWithFromAndToAndData(value)
        }
    }
    impl ::core::convert::From<SetApprovalForAllCall> for MockERC721Calls {
        fn from(value: SetApprovalForAllCall) -> Self {
            Self::SetApprovalForAll(value)
        }
    }
    impl ::core::convert::From<SupportsInterfaceCall> for MockERC721Calls {
        fn from(value: SupportsInterfaceCall) -> Self {
            Self::SupportsInterface(value)
        }
    }
    impl ::core::convert::From<SymbolCall> for MockERC721Calls {
        fn from(value: SymbolCall) -> Self {
            Self::Symbol(value)
        }
    }
    impl ::core::convert::From<TokenURICall> for MockERC721Calls {
        fn from(value: TokenURICall) -> Self {
            Self::TokenURI(value)
        }
    }
    impl ::core::convert::From<TransferFromCall> for MockERC721Calls {
        fn from(value: TransferFromCall) -> Self {
            Self::TransferFrom(value)
        }
    }
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
        Hash,
    )]
    pub struct BalanceOfReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getApproved` function with signature `getApproved(uint256)` and selector `0x081812fc`
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
    pub struct GetApprovedReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `isApprovedForAll` function with signature `isApprovedForAll(address,address)` and selector `0xe985e9c5`
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
    pub struct IsApprovedForAllReturn(pub bool);
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
        Hash,
    )]
    pub struct NameReturn(pub ::std::string::String);
    ///Container type for all return fields from the `ownerOf` function with signature `ownerOf(uint256)` and selector `0x6352211e`
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
    pub struct OwnerOfReturn {
        pub owner: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `0x01ffc9a7`
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
    pub struct SupportsInterfaceReturn(pub bool);
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
        Hash,
    )]
    pub struct SymbolReturn(pub ::std::string::String);
    ///Container type for all return fields from the `tokenURI` function with signature `tokenURI(uint256)` and selector `0xc87b56dd`
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
    pub struct TokenURIReturn(pub ::std::string::String);
}
