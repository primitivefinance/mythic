pub use dfmm::*;
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
pub mod dfmm {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("tokenX_"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("tokenY_"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("swapFeePercentageWad"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("balanceOf"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("balanceOf"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("balance"),
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
                    ::std::borrow::ToOwned::to_owned("getReservesAndLiquidity"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getReservesAndLiquidity",
                            ),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
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
                    ::std::borrow::ToOwned::to_owned("getSwapConstant"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getSwapConstant"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("init"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("init"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("inited"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("inited"),
                            inputs: ::std::vec![],
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("locked"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("locked"),
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
                    ::std::borrow::ToOwned::to_owned("reserveXWad"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("reserveXWad"),
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
                    ::std::borrow::ToOwned::to_owned("reserveYWad"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("reserveYWad"),
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
                    ::std::borrow::ToOwned::to_owned("simulateSwap"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("simulateSwap"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("swapXIn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountIn"),
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
                                    name: ::std::borrow::ToOwned::to_owned("valid"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("estimatedOut"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("swapData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("source"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("source"),
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
                    ::std::borrow::ToOwned::to_owned("swap"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("swap"),
                            inputs: ::std::vec![
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("tokenX"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("tokenX"),
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
                    ::std::borrow::ToOwned::to_owned("tokenY"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("tokenY"),
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
                    ::std::borrow::ToOwned::to_owned("totalLiquidity"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("totalLiquidity"),
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
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("Swap"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Swap"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("swapper"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("source"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("tokenIn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("tokenOut"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amountIn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amountOut"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("liquidityDelta"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("Invalid"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("Invalid"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("negative"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "swapConstantGrowth",
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
                (
                    ::std::borrow::ToOwned::to_owned("Min"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("Min"),
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
    pub static DFMM_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R`\x01\x80U4\x80\x15b\0\0bW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`@Qb\0B\"8\x03\x80b\0B\"\x839\x81\x01`@\x81\x90Rb\0\0\x85\x91b\0\x01>V[`\x02\x80T`\x01`\x01`\xA0\x1B\x03\x80\x86\x16`\x01`\x01`\xA0\x1B\x03\x19\x92\x83\x16\x17\x90\x92U`\x03\x80T\x92\x85\x16\x92\x90\x91\x16\x91\x90\x91\x17\x90U`@Q\x81\x90b\0\0\xC5\x90b\0\x01\x13V[\x90\x81R` \x01`@Q\x80\x91\x03\x90`\0\xF0\x80\x15\x80\x15b\0\0\xE8W=`\0\x80>=`\0\xFD[P`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UPb\0\x01\xCA\x91PPV[a)\xF8\x80b\0\x18*\x839\x01\x90V[\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x019W`\0\x80\xFD[\x91\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15b\0\x01\x9FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[b\0\x01\xAA\x84b\0\x01!V[\x92Pb\0\x01\xBA` \x85\x01b\0\x01!V[\x91P`@\x84\x01Q\x90P\x92P\x92P\x92V[a\x16P\x80b\0\x01\xDA`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x017W`\x005`\xE0\x1C\x80cb}\xD5j\x11a\0\xD9W\x80c\xB7\xD1\x9F\xC4\x11a\0\xB3W\x80c\xB7\xD1\x9F\xC4\x14a\x02\xB4W\x80c\xCF0\x90\x12\x14a\x02\xC7W\x80c\xEB\xAD\xEF\x01\x14a\x02\xD0W\x80c\xF0e\x95\x7F\x14a\x02\xDEWa\x017V[\x80cb}\xD5j\x14a\x02lW\x80cg\xE8(\xBF\x14a\x02\x81W\x80cp\xA0\x821\x14a\x02\x94Wa\x017V[\x80c\x19c\x824\x11a\x01\x15W\x80c\x19c\x824\x14a\x02\tW\x80cC\xC8\x85\xBA\x14a\x02\x11W\x80cM\xDFG\xD4\x14a\x025W\x80c^\"}X\x14a\x02cWa\x017V[\x80c\n\xC304\x14a\x01\x9CW\x80c\x15w\x0F\x92\x14a\x01\xC7W\x80c\x16\xDC\x16[\x14a\x01\xDEW[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x01\xAFa\x01\xAA6`\x04a\x116V[a\x02\xE7V[`@Qa\x01\xBE\x93\x92\x91\x90a\x11\xB5V[`@Q\x80\x91\x03\x90\xF3[a\x01\xD0`\x06T\x81V[`@Q\x90\x81R` \x01a\x01\xBEV[`\x02Ta\x01\xF1\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\xBEV[a\x01\xD0a\x03\xC0V[`\0Ta\x02%\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x01\xBEV[a\x02Ha\x02C6`\x04a\x128V[a\x04\xBBV[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x01\xBEV[a\x01\xD0`\x05T\x81V[a\x02\x7Fa\x02z6`\x04a\x128V[a\x07\xE5V[\0[`\0Ta\x01\xF1\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01\xD0a\x02\xA26`\x04a\x13]V[`\x07` R`\0\x90\x81R`@\x90 T\x81V[`\x03Ta\x01\xF1\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01\xD0`\x01T\x81V[`\x04T`\x05T`\x06Ta\x02HV[a\x01\xD0`\x04T\x81V[`\0\x80T`@Qc\x02\xB0\xCC\r`\xE2\x1B\x81R\x84\x15\x15`\x04\x82\x01R`$\x81\x01\x84\x90R\x82\x91``\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\n\xC304\x90`D\x01`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x03wW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x160\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x03\x8BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x03\xB3\x91\x90\x81\x01\x90a\x13\xA6V[\x92P\x92P\x92P\x92P\x92P\x92V[`\x04T`\x05T`\x06T`@\x80Q` \x81\x01\x94\x90\x94R\x83\x01\x91\x90\x91R``\x82\x01R`\0\x90\x81\x90`\x80\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R`\0Tc\xA1\x9C\xD3\xD1`\xE0\x1B\x83R\x90\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\xA1\x9C\xD3\xD1\x90a\x04'\x90\x84\x90`\x04\x01a\x14\xCDV[` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x04}W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x160\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x04\x91W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xB5\x91\x90a\x14\xE0V[\x91PP\x90V[`\0\x80`\0`\x01T`\x01\x14a\x05\0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x06`$\x82\x01Re\x1B\x1B\xD8\xDA\xD9Y`\xD2\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0`\x01\x81\x90U\x80T`@Qc\x13w\xD1\xF5`\xE2\x1B\x81R\x82\x91\x82\x91\x82\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cM\xDFG\xD4\x90a\x05@\x90\x8D\x90\x8D\x90`\x04\x01a\x14\xFCV[`\xA0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x05\x98W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x160\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x05\xACW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xD0\x91\x90a\x15+V[\x94P\x94P\x94P\x94P\x94P\x84a\x06\x0EW`\0\x84\x12a\x05\xEC\x85a\t\xEFV[`@Qcw`m)`\xE1\x1B\x81R\x91\x15\x15`\x04\x83\x01R`$\x82\x01R`D\x01a\x04\xF7V[`\0\x80T`\xFF`\xA0\x1B\x19\x16`\x01`\xA0\x1B\x17\x81U`\x04\x84\x81U`\x05\x84\x90U`\x06\x83\x90U3\x80\x83R`\x07` R`@\x92\x83\x90 \x84\x90U`\x02T\x92Qc#\xB8r\xDD`\xE0\x1B\x81R\x91\x82\x01R0`$\x82\x01R`D\x81\x01\x85\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90`d\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x06\xCFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x160\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x06\xE3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x07\x91\x90a\x15wV[P`\x03T`@Qc#\xB8r\xDD`\xE0\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90`d\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x07\x98W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x160\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x07\xACW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\xD0\x91\x90a\x15wV[P`\x01\x80U\x91\x99\x90\x98P\x90\x96P\x94PPPPPV[`\x01T`\x01\x14a\x08 W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x06`$\x82\x01Re\x1B\x1B\xD8\xDA\xD9Y`\xD2\x1B`D\x82\x01R`d\x01a\x04\xF7V[`\0`\x01\x81\x90UT`\x01`\xA0\x1B\x90\x04`\xFF\x16a\x08pW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rn\x1B\x9B\xDD\x08\x1A[\x9A]\x1AX[\x1A^\x99Y`\x8A\x1B`D\x82\x01R`d\x01a\x04\xF7V[`\0\x80T`@Qc\xC1nP\xEF`\xE0\x1B\x81R\x82\x91\x82\x91\x82\x91\x82\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xC1nP\xEF\x90a\x08\xAB\x90\x8B\x90\x8B\x90`\x04\x01a\x14\xFCV[`\xC0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\t\x01W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x160\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\t\x15W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t9\x91\x90a\x15\x97V[\x95P\x95P\x95P\x95P\x95P\x95P\x85a\tWW`\0\x85\x12a\x05\xEC\x86a\t\xEFV[`\x06\x81\x90U`\0\x80\x80\x80a\tk\x87\x87a\n*V[`\0T`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x80\x82R` \x82\x01\x86\x90R\x91\x81\x01\x84\x90R``\x81\x01\x8F\x90R\x95\x99P\x93\x97P\x91\x95P\x93P3\x92\x8B\x91\x80\x88\x16\x91\x90\x89\x16\x90\x85\x90\x7F\xD4\xDB\xFA\xA7\xC0\xFEQ\xEE\xE2J\x1C\xDC\xAF\xD07\x8C\xD3e,\xDB\xA75\xDF\xC4\xB0a\xAAU\xC2\xA4X\xA5\x90`\x80\x01`@Q\x80\x91\x03\x90\xA4PP`\x01\x80UPPPPPPPPPPPPPV[`\0`\x01`\xFF\x1B\x82\x03a\n\x15W`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x12\x15a\n&WP\x19`\x01\x01\x90V[P\x90V[`\x04T`\x05T`\0\x91\x82\x91\x82\x91\x82\x91\x81\x88\x11\x15a\n\xC3W`\x02T`\x03T`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x97P\x16\x94Pa\na\x82\x89a\x16\x03V[\x93P\x86\x81\x11a\n\xB2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7Finvalid swap: inputs x and y\0\0\0\0`D\x82\x01R`d\x01a\x04\xF7V[a\n\xBC\x87\x82a\x16\x03V[\x92Pa\x0BAV[`\x03T`\x02T`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x97P\x16\x94Pa\n\xE3\x81\x88a\x16\x03V[\x93P\x87\x82\x11a\x0B4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7Finvalid swap: inputs x and y\0\0\0\0`D\x82\x01R`d\x01a\x04\xF7V[a\x0B>\x88\x83a\x16\x03V[\x92P[`\x04\x88\x81U`\x05\x88\x90U`@Qcp\xA0\x821`\xE0\x1B\x81R0\x91\x81\x01\x91\x90\x91R`\0\x90`\x01`\x01`\xA0\x1B\x03\x88\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0B\xCDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x160\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0B\xE1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\x05\x91\x90a\x14\xE0V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x88\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0C\x88W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x160\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0C\x9CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xC0\x91\x90a\x14\xE0V[`@Qc#\xB8r\xDD`\xE0\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x88\x90R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x89\x16\x90c#\xB8r\xDD\x90`d\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\rOW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x160\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\rcW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\x87\x91\x90a\x15wV[P`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x86\x90R`\x01`\x01`\xA0\x1B\x03\x88\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0E\x0EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x160\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x0E\"W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0EF\x91\x90a\x15wV[Pa\x0EQ\x86\x83a\x16\x1CV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x8A\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0E\xCEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x160\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0E\xE2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\x06\x91\x90a\x14\xE0V[\x10\x15a\x0F_W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: input token transf`D\x82\x01Ra2\xB9`\xF1\x1B`d\x82\x01R`\x84\x01a\x04\xF7V[a\x0Fi\x85\x82a\x16\x03V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x89\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0F\xE6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x160\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0F\xFAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\x1E\x91\x90a\x14\xE0V[\x10\x15a\x10xW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7Finvalid swap: output token trans`D\x82\x01Rb32\xB9`\xE9\x1B`d\x82\x01R`\x84\x01a\x04\xF7V[PPPP\x92\x95\x91\x94P\x92PV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[\x80\x15\x15\x81\x14a\x113W`\0\x80\xFD[PV[`\0\x80`@\x83\x85\x03\x12\x15a\x11LWa\x11La\x10\x85V[\x825a\x11W\x81a\x11%V[\x94` \x93\x90\x93\x015\x93PPPV[`\0[\x83\x81\x10\x15a\x11\x80W\x81\x81\x01Q\x83\x82\x01R` \x01a\x11hV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x11\xA1\x81` \x86\x01` \x86\x01a\x11eV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[\x83\x15\x15\x81R\x82` \x82\x01R```@\x82\x01R`\0a\x11\xD6``\x83\x01\x84a\x11\x89V[\x95\x94PPPPPV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x81\xFD[`\0\x80` \x83\x85\x03\x12\x15a\x12NWa\x12Na\x10\x85V[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x12iWa\x12ia\x10\xD5V[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x12\x80Wa\x12\x80a\x11\xDFV[\x815\x81\x81\x11\x15a\x12\xE3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x0ENL/$\r\x8C\xAD\xCC\xEE\x8D`\xAB\x1B`d\x82\x01R`\x84\x81\xFD[\x86` \x82\x85\x01\x01\x11\x15a\x13KW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rjrray stride`\xA8\x1B`d\x82\x01R\x91P`\x84\x82\xFD[` \x92\x90\x92\x01\x96\x91\x95P\x90\x93PPPPV[`\0` \x82\x84\x03\x12\x15a\x13rWa\x13ra\x10\x85V[\x815`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x13\x89W`\0\x80\xFD[\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15a\x13\xBEWa\x13\xBEa\x10\x85V[\x83Qa\x13\xC9\x81a\x11%V[` \x85\x01Q`@\x86\x01Q\x91\x94P\x92Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x13\xF1Wa\x13\xF1a\x10\xD5V[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x14\x08Wa\x14\x08a\x11\xDFV[\x81Q\x81\x81\x11\x15a\x14\x1AWa\x14\x1Aa\x13\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a\x14BWa\x14Ba\x13\x90V[\x81`@R\x82\x81R\x89` \x84\x87\x01\x01\x11\x15a\x14\xADW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R\x93P`\x84\x84\xFD[a\x14\xBE\x83` \x83\x01` \x88\x01a\x11eV[\x80\x95PPPPPP\x92P\x92P\x92V[` \x81R`\0a\x13\x89` \x83\x01\x84a\x11\x89V[`\0` \x82\x84\x03\x12\x15a\x14\xF5Wa\x14\xF5a\x10\x85V[PQ\x91\x90PV[` \x81R\x81` \x82\x01R\x81\x83`@\x83\x017`\0\x81\x83\x01`@\x90\x81\x01\x91\x90\x91R`\x1F\x90\x92\x01`\x1F\x19\x16\x01\x01\x91\x90PV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\x15FWa\x15Fa\x10\x85V[\x85Qa\x15Q\x81a\x11%V[` \x87\x01Q`@\x88\x01Q``\x89\x01Q`\x80\x90\x99\x01Q\x92\x9A\x91\x99P\x97\x96P\x90\x94P\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x15\x8CWa\x15\x8Ca\x10\x85V[\x81Qa\x13\x89\x81a\x11%V[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a\x15\xB3Wa\x15\xB3a\x10\x85V[\x86Qa\x15\xBE\x81a\x11%V[` \x88\x01Q`@\x89\x01Q``\x8A\x01Q`\x80\x8B\x01Q`\xA0\x90\x9B\x01Q\x93\x9C\x92\x9BP\x90\x99\x90\x98P\x96P\x90\x94P\x92PPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x16\x16Wa\x16\x16a\x15\xEDV[\x92\x91PPV[\x80\x82\x01\x80\x82\x11\x15a\x16\x16Wa\x16\x16a\x15\xEDV\xFETarget contract does not contain`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`@Qb\0)\xF88\x03\x80b\0)\xF8\x839\x81\x01`@\x81\x90Ra\0~\x91a\0\x86V[`\0Ua\0\xEAV[`\0` \x82\x84\x03\x12\x15a\0\xE3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[PQ\x91\x90PV[a(\xFE\x80b\0\0\xFA`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x01\xF6W`\x005`\xE0\x1C\x80c\x85\xAAEN\x11a\x01FW\x80c\xC1nP\xEF\x11a\0\xE4W\x80c\xDB\xAF\x11B\x11a\0\xBEW\x80c\xDB\xAF\x11B\x14a\x04\xB5W\x80c\xE4\x93t(\x14a\x04\xC8W\x80c\xF9\xC2\x82\x11\x14a\x04\xDBW\x80c\xFF\xB3bf\x14a\x04\xE4Wa\x01\xF6V[\x80c\xC1nP\xEF\x14a\x04MW\x80c\xCDd\xAE\xA2\x14a\x04\x8FW\x80c\xD7\xF9{\xEF\x14a\x04\xA2Wa\x01\xF6V[\x80c\x9D\x81\x9E\x83\x11a\x01 W\x80c\x9D\x81\x9E\x83\x14a\x03\xD6W\x80c\xA1\x9C\xD3\xD1\x14a\x03\xE9W\x80c\xA6\xD34\x98\x14a\x03\xFCW\x80c\xBDB-(\x14a\x04\x0FWa\x01\xF6V[\x80c\x85\xAAEN\x14a\x03\xA1W\x80c\x88;m\xC5\x14a\x03\xB0W\x80c\x97\x16\xAE\xBB\x14a\x03\xC3Wa\x01\xF6V[\x80c@\xB41i\x11a\x01\xB3W\x80cX\xFAc\xCA\x11a\x01\x8DW\x80cX\xFAc\xCA\x14a\x03oW\x80cd\x17\xD4\xB5\x14a\x03wW\x80cj\x14`$\x14a\x03\x8AW\x80cme\"\x99\x14a\x03\x99Wa\x01\xF6V[\x80c@\xB41i\x14a\x03\x16W\x80cM\xDFG\xD4\x14a\x03\x1FW\x80cV\x08\xBE\xA1\x14a\x03\\Wa\x01\xF6V[\x80c\n\xC304\x14a\x02[W\x80c\x0F\n\xA3\x95\x14a\x02\x86W\x80c\x18M0\xBA\x14a\x02\xA6W\x80c\x1A\x88\xBCf\x14a\x02\xC7W\x80c\x1D\x9C\xF7\"\x14a\x02\xF4W\x80c']g\xC8\x14a\x03\x07W[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x02na\x02i6`\x04a!\x11V[a\x04\xF7V[`@Qa\x02}\x93\x92\x91\x90a!\x8BV[`@Q\x80\x91\x03\x90\xF3[a\x02\x99a\x02\x946`\x04a\"\xAAV[a\x07\xFCV[`@Qa\x02}\x91\x90a\"\xEDV[a\x02\xB9a\x02\xB46`\x04a#\0V[a\x08.V[`@Q\x90\x81R` \x01a\x02}V[`\x01T`\x02T`\x03Ta\x02\xD9\x92\x91\x90\x83V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x02}V[a\x02\xB9a\x03\x026`\x04a#9V[a\x08\x80V[a\x02\xB9g\x1B\xC1mgN\xC8\0\0\x81V[a\x02\xB9`\0T\x81V[a\x032a\x03-6`\x04a#\xAEV[a\x08\xCEV[`@\x80Q\x95\x15\x15\x86R` \x86\x01\x94\x90\x94R\x92\x84\x01\x91\x90\x91R``\x83\x01R`\x80\x82\x01R`\xA0\x01a\x02}V[a\x02\xB9a\x03j6`\x04a#\0V[a\tjV[a\x02\xB9`\0\x81V[a\x02\xB9a\x03\x856`\x04a\"\xAAV[a\t\xABV[a\x02\xB9g\r\xE0\xB6\xB3\xA7d\0\0\x81V[a\x02\xB9`\x01\x81V[a\x02\xB9g\x06\xF0[Y\xD3\xB2\0\0\x81V[a\x02\xB9a\x03\xBE6`\x04a\"\xAAV[a\n\x06V[a\x02\xB9a\x03\xD16`\x04a$\xD3V[a\n_V[a\x02\xB9a\x03\xE46`\x04a#\0V[a\n\xFCV[a\x02\xB9a\x03\xF76`\x04a%\x03V[a\x0B\x91V[a\x02\xB9a\x04\n6`\x04a#9V[a\x0B\xECV[a\x02\x99a\x04\x1D6`\x04a%\xF1V[`@\x80Q` \x81\x01\x94\x90\x94R\x83\x81\x01\x92\x90\x92R``\x80\x84\x01\x91\x90\x91R\x81Q\x80\x84\x03\x90\x91\x01\x81R`\x80\x90\x92\x01\x90R\x90V[a\x04`a\x04[6`\x04a%\x03V[a\x0C\x15V[`@\x80Q\x96\x15\x15\x87R` \x87\x01\x95\x90\x95R\x93\x85\x01\x92\x90\x92R``\x84\x01R`\x80\x83\x01R`\xA0\x82\x01R`\xC0\x01a\x02}V[a\x02\xB9a\x04\x9D6`\x04a\"\xAAV[a\x0F\xA4V[a\x02\xB9a\x04\xB06`\x04a$\xD3V[a\x0F\xF5V[a\x02\xB9a\x04\xC36`\x04a#\0V[a\x10\x1DV[a\x02\xB9a\x04\xD66`\x04a%\xF1V[a\x10_V[a\x02\xB9a\x01\0\x81V[a\x02\xB9a\x04\xF26`\x04a%\xF1V[a\x10\xDDV[`\0\x80``a\x05\x1D`@Q\x80`\x80\x01`@R\x80`G\x81R` \x01a(\x92`G\x919a\x11dV[`\0\x80`\x003`\x01`\x01`\xA0\x1B\x03\x16c\xEB\xAD\xEF\x01`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01```@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x05\xABW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x05\xBFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xE3\x91\x90a& V[`@\x80Q` \x81\x01\x85\x90R\x90\x81\x01\x83\x90R``\x81\x01\x82\x90R\x92\x95P\x90\x93P\x91P`\0\x90a\x06!\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@Ra\x0B\x91V[\x90P`\0\x89\x15a\x07\x0EW`\0\x80Ta\x06:\x90\x8B\x90a\x11\xAAV[\x90P`\0a\x06R\x87a\x06L\x84\x88a\x11\xAAV[\x90a\x11\xC5V[\x90Pa\x06_`\x01\x82a&gV[\x90Pa\x06k\x8B\x88a&gV[\x96Pa\x06w\x81\x86a&gV[`@\x80Q``\x81\x01\x82R`\x01T\x81R`\x02T` \x82\x01R`\x03T\x91\x81\x01\x91\x90\x91R\x90\x95P\x86\x90a\x06\xAC\x90\x89\x90\x88\x90\x88\x90a\t\xABV[\x96Pa\x06\xB9`\x01\x88a&gV[\x96Pa\x06\xC5\x87\x82a&zV[\x93Pa\x07\x06`@Q\x80`@\x01`@R\x80`\x1C\x81R` \x01\x7FEsimated Y reserve to submit\0\0\0\0\x81RP\x88a\x11\xDAV[PPPa\x07\xA8V[`\0\x80Ta\x07\x1D\x90\x8B\x90a\x11\xAAV[\x90P`\0a\x07/\x86a\x06L\x84\x88a\x11\xAAV[\x90Pa\x07<`\x01\x82a&gV[\x90Pa\x07H\x8B\x87a&gV[\x95Pa\x07T\x81\x86a&gV[`@\x80Q``\x81\x01\x82R`\x01T\x81R`\x02T` \x82\x01R`\x03T\x91\x81\x01\x91\x90\x91R\x90\x95P\x87\x90a\x07\x89\x90\x88\x90\x88\x90\x88\x90a\x0F\xA4V[\x97Pa\x07\x96`\x01\x89a&gV[\x97Pa\x07\xA2\x88\x82a&zV[\x93PPPP[`@\x80Q` \x81\x01\x87\x90R\x90\x81\x01\x85\x90R``\x81\x01\x84\x90R`\0\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`\0a\x07\xE2\x82a\x0C\x15V[P\x93\x9DP\x95\x9BP\x93\x99PPPPPPPPPP\x92P\x92P\x92V[``\x84\x84\x84\x84`@Q` \x01a\x08\x15\x94\x93\x92\x91\x90a&\x8DV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x94\x93PPPPV[`\0\x80a\x08;\x84\x84a\n_V[\x90P`\0a\x08H\x82a\x12#V[\x90P`\0a\x08U\x82a\x08\x80V[\x90Pa\x08sa\x08l\x82g\r\xE0\xB6\xB3\xA7d\0\0a&zV[\x88\x90a\x11\xAAV[\x93PPPP[\x93\x92PPPV[`\0\x80\x82\x12\x15a\x08\xCAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01RotoUint: negative`\x80\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[P\x90V[`\0\x80`\0\x80`\0a\x08\xF7`@Q\x80`\x80\x01`@R\x80`G\x81R` \x01a(\x92`G\x919a\x11dV[a\t\x03\x86\x88\x01\x88a\"\xAAV[\x80Q`\x01\x81\x90U` \x80\x83\x01Q`\x02\x81\x90U`@\x93\x84\x01Q`\x03\x81\x90U\x84Q``\x81\x01\x86R\x93\x84R\x91\x83\x01R\x91\x81\x01\x91\x90\x91R\x92\x95P\x90\x93P\x91Pa\tM\x90\x84\x90\x84\x90\x84\x90a\x12\x8CV[\x93Pa\t[`\0`\x03a&\xC1V[\x84\x12\x15\x94P\x92\x95P\x92\x95\x90\x93PV[`\0\x80a\tw\x84\x84a\x0F\xF5V[\x90P`\0a\t\x84\x82a\x12#V[\x90P`\0a\t\x91\x82a\x08\x80V[\x85Q\x90\x91Pa\x08s\x90a\t\xA4\x90\x83a\x13<V[\x88\x90a\x13QV[\x80Q`\0\x90`d\x90\x82\x90a\t\xC1\x90`\x01\x90a&zV[\x90Pa\t\xFB\x87\x87\x87\x87`@Q` \x01a\t\xDD\x94\x93\x92\x91\x90a&\x8DV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83\x83`\x01a\x01\0a\x13fa\x13\xA1V[\x97\x96PPPPPPPV[`\0\x80a\n\x14\x86`\x01a&gV[\x90P`\0k\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x90Pa\t\xFB\x87\x87\x87\x87`@Q` \x01a\nA\x94\x93\x92\x91\x90a&\x8DV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83\x83`\x01a\x01\0a\x14\xB2a\x13\xA1V[`\0\x80a\nt\x83` \x01Q\x84`@\x01Qa\x14\xE3V[\x90P`\0a\n\x85\x84` \x01Qa\x0B\xECV[\x90P`\0a\n\xA8a\n\xA3\x86`\0\x01Q\x88a\x11\xC5\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x15\rV[\x90P\x80a\n\xB4\x81a&\xE9V[\x91PP`\0a\n\xD0\x86`@\x01Q\x84a\x13<\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\n\xDA\x90\x83a&\xC1V[\x90P\x83a\n\xF0\x82g\r\xE0\xB6\xB3\xA7d\0\0\x83a\x10\xDDV[\x98\x97PPPPPPPPV[`\0\x80a\x0B\t\x84\x84a\n_V[\x90P`\0a\x0B\x16\x82a\x12#V[\x90P`\0a\x0B#\x82a\x08\x80V[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0\x81\x10a\x0B|W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7Flx: denominator is zero\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x08\xC1V[a\x08sa\t\xA4\x82g\r\xE0\xB6\xB3\xA7d\0\0a&zV[`\0\x80`\0\x80\x84\x80` \x01\x90Q\x81\x01\x90a\x0B\xAB\x91\x90a& V[`@\x80Q``\x81\x01\x82R`\x01T\x81R`\x02T` \x82\x01R`\x03T\x91\x81\x01\x91\x90\x91R\x92\x95P\x90\x93P\x91Pa\x0B\xE3\x90\x84\x90\x84\x90\x84\x90a\x12\x8CV[\x95\x94PPPPPV[`\0\x80a\x0C\x01\x83g\x1B\xC1mgN\xC8\0\0a\x16\xE8V[\x90Pa\x08yg\x06\xF0[Y\xD3\xB2\0\0\x82a\x13<V[`\0\x80`\0\x80`\0\x80a\x0C?`@Q\x80`\x80\x01`@R\x80`G\x81R` \x01a(\x92`G\x919a\x11dV[`\0\x80`\x003`\x01`\x01`\xA0\x1B\x03\x16c\xEB\xAD\xEF\x01`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01```@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0C\xCDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0C\xE1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\x05\x91\x90a& V[\x92P\x92P\x92P\x89\x80` \x01\x90Q\x81\x01\x90a\r\x1F\x91\x90a& V[\x91\x97P\x95P\x93P`\x01\x83\x87\x11\x15a\rxW`\0a\r<\x85\x89a&zV[\x90P`\0a\rU`\0T\x83a\x11\xAA\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa\re\x86a\x06L\x83\x87a\x11\xAAV[a\ro\x90\x84a&gV[\x92PPPa\x0E\x16V[\x82\x86\x11\x15a\r\xB5W`\0a\r\x8C\x84\x88a&zV[\x90P`\0a\r\xA5`\0T\x83a\x11\xAA\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa\re\x85a\x06L\x83\x87a\x11\xAAV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`0`$\x82\x01R\x7Finvalid swap: inputs x and y hav`D\x82\x01Roe the same sign!`\x80\x1B`d\x82\x01R`\x84\x01a\x08\xC1V[a\x0E \x82\x86a'\x08V[`@\x80Q``\x81\x01\x82R`\x01T\x81R`\x02T` \x82\x01R`\x03T\x91\x81\x01\x91\x90\x91R\x90\x98Pa\x0ES\x90\x85\x90\x85\x90\x85\x90a\x12\x8CV[`@\x80Q``\x81\x01\x82R`\x01T\x81R`\x02T` \x82\x01R`\x03T\x91\x81\x01\x91\x90\x91Ra\x0E\x83\x90\x89\x90\x89\x90\x89\x90a\x12\x8CV[a\x0E\x8D\x91\x90a'\x08V[\x98Pa\x0E\xC4`@Q\x80`@\x01`@R\x80`\x14\x81R` \x01s\nn\xEC.\x04\x0Cm\xED\xCEn\x8C-\xCE\x84\x0C\xEEM\xEE\xEE\x8D`c\x1B\x81RPa\x11dV[a\x0E\xCD\x89a\x17\x19V[a\x0F\x0B`@Q\x80`@\x01`@R\x80`\x19\x81R` \x01\x7FSubmitted Liquidity delta\0\0\0\0\0\0\0\x81RPa\x11dV[a\x0F\x14\x88a\x17\x19V[a\x0FH`@Q\x80`@\x01`@R\x80`\x13\x81R` \x01rMin liquidity delta`h\x1B\x81RPa\x11dV[a\x0FQ\x81a\x17\x19V[a\x0Fr`@Q\x80``\x01`@R\x80`%\x81R` \x01a(\xD9`%\x919a\x11dV[a\x0F\x84a\x0F\x7F\x82\x8Aa'\x08V[a\x17\x19V[`\0\x89\x12\x15\x80\x15a\x0F\x95WP\x80\x88\x12\x15[\x99PPPPP\x91\x93\x95P\x91\x93\x95V[`\0`\x02\x81a\x0F\xBB\x82g\r\xE0\xB6\xB3\xA7d\0\0a&zV[\x90Pa\t\xFB\x87\x87\x87\x87`@Q` \x01a\x0F\xD7\x94\x93\x92\x91\x90a&\x8DV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83\x83`\x01a\x01\0a\x17^a\x13\xA1V[`\0a\x10\t\x82` \x01Q\x83`@\x01Qa\x14\xE3V[a\x10\x13\x84\x84a\n_V[a\x08y\x91\x90a'\x08V[`\0\x80a\x10*\x84\x84a\x0F\xF5V[\x90P`\0a\x107\x82a\x12#V[\x90P`\0a\x10D\x82a\x08\x80V[\x85Q\x90\x91Pa\x08s\x90\x82\x90a\x10Y\x90\x8Aa\x11\xAAV[\x90a\x11\xAAV[\x82\x82\x02\x81\x15\x80\x15\x90a\x10\x87WP\x83\x15\x80a\x10\x87WP\x82\x84\x82\x81a\x10\x84Wa\x10\x84a'/V[\x05\x14[a\x10\xC5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rn\x1B][\x1AY\x1A]\x88\x1A[\x9D\x98[\x1AY`\x8A\x1B`D\x82\x01R`d\x01a\x08\xC1V[\x81\x81\x81a\x10\xD4Wa\x10\xD4a'/V[\x05\x94\x93PPPPV[`\0a\x10\xEA\x84\x84\x84a\x10_V[\x90P\x81a\x10\xF7\x84\x86a'EV[a\x11\x01\x91\x90a'uV[\x15a\x08yW`\x01`\x01`\xFF\x1B\x03\x81\x12a\x11QW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01RqmulidivUp overflow`p\x1B`D\x82\x01R`d\x01a\x08\xC1V[a\x11\\`\x01\x82a&\xC1V[\x94\x93PPPPV[a\x11\xA7\x81`@Q`$\x01a\x11x\x91\x90a\"\xEDV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\x10L\x13\xEB`\xE2\x1B\x17\x90Ra\x17\x8FV[PV[`\0a\x08y\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x17\xB0V[\x92\x91PPV[`\0a\x08y\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x17\xB0V[a\x12\x1F\x82\x82`@Q`$\x01a\x11\xF0\x92\x91\x90a'\x89V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c-\x83\x9C\xB3`\xE2\x1B\x17\x90Ra\x17\x8FV[PPV[`\0\x80g\x13\xA0K\xBD\xFD\xC9\xBE\x88a\x12Ag\r\xE0\xB6\xB3\xA7d\0\0\x85a'EV[a\x12K\x91\x90a'\xABV[\x90P`\0a\x12X\x82a'\xD9V[\x90P`\0a\x12e\x82a\x17\xDEV[\x90Pg\x1B\xC1mgN\xC8\0\0a\x12\x82g\r\xE0\xB6\xB3\xA7d\0\0\x83a'EV[a\x0B\xE3\x91\x90a'\xABV[`\0\x82\x85\x10a\x12\xDDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FtradingFunction: invalid x\0\0\0\0\0\0`D\x82\x01R`d\x01a\x08\xC1V[`\0a\x12\xF1a\x12\xEC\x87\x86a\x13QV[a\x19\xC7V[\x90P`\0a\x13\x0Fa\x12\xEC\x87a\x13\n\x87`\0\x01Q\x89a\x13<V[a\x13QV[\x90P`\0a\x13%\x85` \x01Q\x86`@\x01Qa\x14\xE3V[\x90P\x80a\x132\x83\x85a&\xC1V[a\n\xF0\x91\x90a&\xC1V[`\0a\x08y\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x1AdV[`\0a\x08y\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x1AdV[`\0\x80`\0\x80`\0\x86\x80` \x01\x90Q\x81\x01\x90a\x13\x82\x91\x90a'\xF5V[\x93P\x93P\x93P\x93P\x81a\x13\x97\x85\x88\x86\x85a\x12\x8CV[a\t\xFB\x91\x90a'\x08V[`\0\x84\x86\x11\x15a\x13\xCEW`@Qc0\x82\xDF\xDB`\xE1\x1B\x81R`\x04\x81\x01\x87\x90R`$\x81\x01\x86\x90R`D\x01a\x08\xC1V[`\0a\x13\xDE\x88\x88\x85c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x13\xF0\x89\x88\x86c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x13\xFE\x82\x84a'EV[\x13\x15a\x14'W`@Qc\x06\xF1\xBE]`\xE2\x1B\x81R`\x04\x81\x01\x83\x90R`$\x81\x01\x82\x90R`D\x01a\x08\xC1V[`\0a\x143\x89\x89a&zV[\x90P`\0[`\x02a\x14D\x8A\x8Ca&gV[a\x14N\x91\x90a(fV[\x94P`\0a\x14`\x8C\x87\x89c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x14n\x86\x83a'EV[\x13a\x14{W\x85\x99Pa\x14\x82V[\x85\x9AP\x80\x94P[a\x14\x8C\x8B\x8Ba&zV[\x92PP`\x01\x01\x87\x82\x11\x80\x15a\x14\xA0WP\x86\x81\x10[a\x148WPPPP\x96\x95PPPPPPV[`\0\x80`\0\x80`\0\x86\x80` \x01\x90Q\x81\x01\x90a\x14\xCE\x91\x90a'\xF5V[\x93P\x93P\x93P\x93P\x81a\x13\x97\x85\x85\x89\x85a\x12\x8CV[`\0\x80a\x14\xEF\x83a\x1A\x83V[\x90P`\0a\x15\x01c;\x9A\xCA\0\x83a(zV[\x90Pa\x0B\xE3\x85\x82a\x13<V[`\0\x80\x82\x13a\x15JW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x08\xC1V[`\0``a\x15W\x84a\x1B'V[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[`\0a\x08yg\r\xE0\xB6\xB3\xA7d\0\0\x83a\x17\0\x86a\x15\rV[a\x17\n\x91\x90a'EV[a\x17\x14\x91\x90a'\xABV[a\x1B\xCFV[a\x11\xA7\x81`@Q`$\x01a\x17/\x91\x81R` \x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c-[l\xB9`\xE0\x1B\x17\x90Ra\x17\x8FV[`\0\x80`\0\x80`\0\x86\x80` \x01\x90Q\x81\x01\x90a\x17z\x91\x90a'\xF5V[\x93P\x93P\x93P\x93P\x81a\x13\x97\x87\x86\x86\x85a\x12\x8CV[\x80Qjconsole.log` \x83\x01`\0\x80\x84\x83\x85Z\xFAPPPPPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x17\xC8W`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[`\0\x81`\0\x03a\x17\xF7WPg\r\xE0\xB6\xB3\xA7d\0\0\x91\x90PV[gV\x98\xEE\xF0fp\0\0\x82\x12a\x18\x0EWP`\0\x91\x90PV[a\x18\x1FgV\x98\xEE\xF0fp\0\0a'\xD9V[\x82\x13a\x184WPg\x1B\xC1mgN\xC8\0\0\x91\x90PV[`\0a\x18?\x83a\x1DxV[\x90P`\0a\x18xg\r\xE0\xB6\xB3\xA7d\0\0a\x18a\x84g\x1B\xC1mgN\xC8\0\0a\x13QV[a\x18s\x90g\r\xE0\xB6\xB3\xA7d\0\0a&\xC1V[a\x1D\xAFV[\x90P`\0\x80\x82a\x18\xD9\x81a\x18\xC6\x81a\x18\xB4\x81a\x18\x9C\x81g\x02_\x0F\xE1\x05\xA3\x14\0a\x1D\xC4V[a\x18\xAF\x90g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19a&\xC1V[a\x1D\xC4V[a\x18\xAF\x90g\x14\xA8EL\x19\xE1\xAC\0a&\xC1V[a\x18\xAF\x90g\x0F\xC1\x0E\x01W\x82w\xFF\x19a&\xC1V[a\x18\xEB\x90g\x03\xDE\xBD\x08;\x8C|\0a&\xC1V[\x91P\x83\x90Pa\x19S\x81a\x19A\x81a\x19/\x81a\x19\x1D\x81a\x19\n\x81\x8Ba\x1D\xC4V[a\x18\xAF\x90g\x02\x95\xD4\0\xEA2W\xFF\x19a&\xC1V[a\x18\xAF\x90g\x01W\xD8\xB2\xEC\xC7\x08\0a&\xC1V[a\x18\xAF\x90g\x051\n\xA7\xD5!0\0a&\xC1V[a\x18\xAF\x90g\r\xE0\xCC=\x15a\0\0a&\xC1V[\x91P\x81g\x11\x90\0\xAB\x10\x0F\xFC\0a\x19i\x87\x88a\x1D\xC4V[a\x19u\x90`\0\x19a'EV[a\x19\x7F\x91\x90a'\x08V[a\x19\x89\x91\x90a&\xC1V[\x92PP`\0a\x19\x97\x83a\x1B\xCFV[\x90P`\0a\x19\xA5\x85\x83a\x1D\xC4V[\x90P`\0\x88\x12a\x19\xB5W\x80a\n\xF0V[a\n\xF0\x81g\x1B\xC1mgN\xC8\0\0a'\x08V[`\0g\x06\xF0[Y\xD3\xB2\0\0\x82\x03a\x19\xE0WP`\0\x91\x90PV[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x12a\x1A\x08W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\x1A)W`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x1A6\x83`\x02a'EV[\x90P`\0a\x1AC\x82a\x1D\xD9V[\x90P`\0a\x1AYg\x13\xA0K\xBD\xFD\xC9\xBE\x88\x83a\x1D\xC4V[\x90Pa\x0B\xE3\x81a'\xD9V[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x1A|W`\0\x80\xFD[\x04\x92\x91PPV[`\xB5\x81`\x01`\x88\x1B\x81\x10a\x1A\x9CW`@\x91\x90\x91\x1B\x90`\x80\x1C[i\x01\0\0\0\0\0\0\0\0\0\x81\x10a\x1A\xB8W` \x91\x90\x91\x1B\x90`@\x1C[e\x01\0\0\0\0\0\x81\x10a\x1A\xD0W`\x10\x91\x90\x91\x1B\x90` \x1C[c\x01\0\0\0\x81\x10a\x1A\xE6W`\x08\x91\x90\x91\x1B\x90`\x10\x1C[b\x01\0\0\x01\x02`\x12\x1C\x80\x82\x04\x01`\x01\x90\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x90\x81\x90\x04\x81\x11\x90\x03\x90V[`\0\x80\x82\x11a\x1BdW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x08\xC1V[P`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x82\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[`\0h\x02H\xCE6\xA7\x0C\xB2k>\x19\x82\x13a\x1B\xEAWP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a\x1C1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x01a\x08\xC1V[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05`\x01`_\x1B\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1Dm\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1Dn\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[`\0`\x01`\xFF\x1B\x82\x03a\x1D\x9EW`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x12\x15a\x08\xCAWP\x19`\x01\x01\x90V[`\0a\x08y\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a RV[`\0a\x08y\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a RV[`\0\x80\x82\x12\x80a\x1D\xF0WPg\x1B\xC1mgN\xC8\0\0\x82\x13[\x15a\x1E\x0EW`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\x1E/W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81g\x1B\xC1mgN\xC8\0\0\x03a\x1EWW`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x15a\x1EbW\x91\x90PV[`\0g\r\xE0\xB6\xB3\xA7d\0\0\x83\x12a\x1E\x8AWa\x1E\x85\x83g\x1B\xC1mgN\xC8\0\0a'\x08V[a\x1E\x8CV[\x82[\x90P`\0a\x1E\xA2\x82g\x1B\xC1mgN\xC8\0\0a\x1D\xAFV[\x90P\x80`\0\x03a\x1E\xC5W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x1E\xD0\x82a\x15\rV[\x90P`\0c;\x9A\xCA\0a\x1E\xFBa\x1E\xF6a\x1E\xF0g\x1B\xC1mgN\xC8\0\0a'\xD9V[\x85a\x1D\xC4V[a\x1A\x83V[a\x1F\x05\x91\x90a'EV[\x90P`\0\x80a\x1F\x1C\x83g\x03\xC1f\\z\xAB \0a\x1D\xC4V[a\x1F.\x90g \x05\xFEO&\x8E\xA0\0a&\xC1V[\x90P`\0a\x1FY\x84a\x1FG\x86f\x9F2u$b\xA0\0a\x1D\xC4V[a\x18\xAF\x90g\r\xC5R\x7Fd, \0a&\xC1V[a\x1Fk\x90g\r\xE0\xB6\xB3\xA7d\0\0a&\xC1V[\x90Pa\x1F\x8Fg\t\xD0(\xCCo _\xFF\x19\x85a\x1F\x85\x85\x85a\x1D\xAFV[a\x18\xAF\x91\x90a'\x08V[\x92PPP`\0[`\x02\x81\x10\x15a *W`\0\x86a\x1F\xAB\x84a\x17\xDEV[a\x1F\xB5\x91\x90a'\x08V[\x90P`\0a\x1F\xC3\x84\x85a\x1D\xC4V[a\x1F\xCC\x90a'\xD9V[\x90P`\0a\x1F\xD9\x82a\x1B\xCFV[\x90P`\0a\x1F\xE7\x86\x85a\x1D\xC4V[a\x1F\xF9g\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x84a\x1D\xC4V[a \x03\x91\x90a'\x08V[\x90Pa \x0F\x84\x82a\x1D\xAFV[a \x19\x90\x87a&\xC1V[\x95P\x84`\x01\x01\x94PPPPPa\x1F\x96V[g\r\xE0\xB6\xB3\xA7d\0\0\x88\x12a GWa B\x82a'\xD9V[a\n\xF0V[P\x96\x95PPPPPPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x05\x85\x14\x17\x16a jW`\0\x80\xFD[\x05\x92\x91PPV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`\0\x80`@\x83\x85\x03\x12\x15a!'Wa!'a qV[\x825\x80\x15\x15\x81\x14a!7W`\0\x80\xFD[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a!kW` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a!OV[P`\0` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[\x83\x15\x15\x81R\x82` \x82\x01R```@\x82\x01R`\0a\x0B\xE3``\x83\x01\x84a!EV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FABI decoding: struct data too sh`D\x82\x01Rb\x1B\xDC\x9D`\xEA\x1B`d\x82\x01R`\x84\x81\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\"6Wa\"6a!\xFDV[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\"eWa\"ea!\xFDV[`@R\x91\x90PV[`\0``\x82\x84\x03\x12\x15a\"\x82Wa\"\x82a!\xACV[a\"\x8Aa\"\x13V[\x90P\x815\x81R` \x82\x015` \x82\x01R`@\x82\x015`@\x82\x01R\x92\x91PPV[`\0\x80`\0\x80`\xC0\x85\x87\x03\x12\x15a\"\xC3Wa\"\xC3a qV[\x845\x93P` \x85\x015\x92P`@\x85\x015\x91Pa\"\xE2\x86``\x87\x01a\"mV[\x90P\x92\x95\x91\x94P\x92PV[` \x81R`\0a\x08y` \x83\x01\x84a!EV[`\0\x80`\0`\xA0\x84\x86\x03\x12\x15a#\x18Wa#\x18a qV[\x835\x92P` \x84\x015\x91Pa#0\x85`@\x86\x01a\"mV[\x90P\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a#NWa#Na qV[P5\x91\x90PV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x81\xFD[`\0\x80` \x83\x85\x03\x12\x15a#\xC4Wa#\xC4a qV[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a#\xDFWa#\xDFa \xC1V[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a#\xF6Wa#\xF6a#UV[\x815\x81\x81\x11\x15a$YW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x0ENL/$\r\x8C\xAD\xCC\xEE\x8D`\xAB\x1B`d\x82\x01R`\x84\x81\xFD[\x86` \x82\x85\x01\x01\x11\x15a$\xC1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rjrray stride`\xA8\x1B`d\x82\x01R\x91P`\x84\x82\xFD[` \x92\x90\x92\x01\x96\x91\x95P\x90\x93PPPPV[`\0\x80`\x80\x83\x85\x03\x12\x15a$\xE9Wa$\xE9a qV[\x825\x91Pa$\xFA\x84` \x85\x01a\"mV[\x90P\x92P\x92\x90PV[`\0` \x80\x83\x85\x03\x12\x15a%\x19Wa%\x19a qV[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a%4Wa%4a \xC1V[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a%KWa%Ka#UV[\x815\x81\x81\x11\x15a%]Wa%]a!\xFDV[a%o`\x1F\x82\x01`\x1F\x19\x16\x85\x01a\"<V[\x91P\x80\x82R\x86\x84\x82\x85\x01\x01\x11\x15a%\xD5W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x85\x90R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R`\x84\x81\xFD[\x80\x84\x84\x01\x85\x84\x017`\0\x90\x82\x01\x90\x93\x01\x92\x90\x92RP\x93\x92PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a&\tWa&\ta qV[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a&8Wa&8a qV[\x83Q\x92P` \x84\x01Q\x91P`@\x84\x01Q\x90P\x92P\x92P\x92V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x11\xBFWa\x11\xBFa&QV[\x81\x81\x03\x81\x81\x11\x15a\x11\xBFWa\x11\xBFa&QV[\x84\x81R` \x80\x82\x01\x85\x90R`@\x80\x83\x01\x85\x90R\x83Q``\x84\x01R\x90\x83\x01Q`\x80\x83\x01R\x82\x01Q`\xA0\x82\x01R`\xC0\x81\x01a\x0B\xE3V[\x80\x82\x01\x82\x81\x12`\0\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a&\xE1Wa&\xE1a&QV[PP\x92\x91PPV[`\0`\x01`\x01`\xFF\x1B\x01\x82\x01a'\x01Wa'\x01a&QV[P`\x01\x01\x90V[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a'(Wa'(a&QV[P\x92\x91PPV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15a'aWa'aa&QV[\x81\x81\x05\x83\x14\x82\x15\x17a\x11\xBFWa\x11\xBFa&QV[`\0\x82a'\x84Wa'\x84a'/V[P\x07\x90V[`@\x81R`\0a'\x9C`@\x83\x01\x85a!EV[\x90P\x82` \x83\x01R\x93\x92PPPV[`\0\x82a'\xBAWa'\xBAa'/V[`\x01`\xFF\x1B\x82\x14`\0\x19\x84\x14\x16\x15a'\xD4Wa'\xD4a&QV[P\x05\x90V[`\0`\x01`\xFF\x1B\x82\x01a'\xEEWa'\xEEa&QV[P`\0\x03\x90V[`\0\x80`\0\x80\x84\x86\x03`\xC0\x81\x12\x15a(\x0FWa(\x0Fa qV[\x85Q\x94P` \x86\x01Q\x93P`@\x86\x01Q\x92P```_\x19\x82\x01\x12\x15a(6Wa(6a!\xACV[Pa(?a\"\x13V[``\x86\x01Q\x81R`\x80\x86\x01Q` \x82\x01R`\xA0\x90\x95\x01Q`@\x86\x01RP\x91\x94\x90\x93P\x90\x91\x90V[`\0\x82a(uWa(ua'/V[P\x04\x90V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x11\xBFWa\x11\xBFa&QV\xFEMake sure the calling contract of this console.log is the Core contractliquidity delta - min liquidity delta";
    /// The bytecode of the contract.
    pub static DFMM_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x017W`\x005`\xE0\x1C\x80cb}\xD5j\x11a\0\xD9W\x80c\xB7\xD1\x9F\xC4\x11a\0\xB3W\x80c\xB7\xD1\x9F\xC4\x14a\x02\xB4W\x80c\xCF0\x90\x12\x14a\x02\xC7W\x80c\xEB\xAD\xEF\x01\x14a\x02\xD0W\x80c\xF0e\x95\x7F\x14a\x02\xDEWa\x017V[\x80cb}\xD5j\x14a\x02lW\x80cg\xE8(\xBF\x14a\x02\x81W\x80cp\xA0\x821\x14a\x02\x94Wa\x017V[\x80c\x19c\x824\x11a\x01\x15W\x80c\x19c\x824\x14a\x02\tW\x80cC\xC8\x85\xBA\x14a\x02\x11W\x80cM\xDFG\xD4\x14a\x025W\x80c^\"}X\x14a\x02cWa\x017V[\x80c\n\xC304\x14a\x01\x9CW\x80c\x15w\x0F\x92\x14a\x01\xC7W\x80c\x16\xDC\x16[\x14a\x01\xDEW[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x01\xAFa\x01\xAA6`\x04a\x116V[a\x02\xE7V[`@Qa\x01\xBE\x93\x92\x91\x90a\x11\xB5V[`@Q\x80\x91\x03\x90\xF3[a\x01\xD0`\x06T\x81V[`@Q\x90\x81R` \x01a\x01\xBEV[`\x02Ta\x01\xF1\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\xBEV[a\x01\xD0a\x03\xC0V[`\0Ta\x02%\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x01\xBEV[a\x02Ha\x02C6`\x04a\x128V[a\x04\xBBV[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x01\xBEV[a\x01\xD0`\x05T\x81V[a\x02\x7Fa\x02z6`\x04a\x128V[a\x07\xE5V[\0[`\0Ta\x01\xF1\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01\xD0a\x02\xA26`\x04a\x13]V[`\x07` R`\0\x90\x81R`@\x90 T\x81V[`\x03Ta\x01\xF1\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01\xD0`\x01T\x81V[`\x04T`\x05T`\x06Ta\x02HV[a\x01\xD0`\x04T\x81V[`\0\x80T`@Qc\x02\xB0\xCC\r`\xE2\x1B\x81R\x84\x15\x15`\x04\x82\x01R`$\x81\x01\x84\x90R\x82\x91``\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\n\xC304\x90`D\x01`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x03wW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x160\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x03\x8BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x03\xB3\x91\x90\x81\x01\x90a\x13\xA6V[\x92P\x92P\x92P\x92P\x92P\x92V[`\x04T`\x05T`\x06T`@\x80Q` \x81\x01\x94\x90\x94R\x83\x01\x91\x90\x91R``\x82\x01R`\0\x90\x81\x90`\x80\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R`\0Tc\xA1\x9C\xD3\xD1`\xE0\x1B\x83R\x90\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\xA1\x9C\xD3\xD1\x90a\x04'\x90\x84\x90`\x04\x01a\x14\xCDV[` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x04}W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x160\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x04\x91W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xB5\x91\x90a\x14\xE0V[\x91PP\x90V[`\0\x80`\0`\x01T`\x01\x14a\x05\0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x06`$\x82\x01Re\x1B\x1B\xD8\xDA\xD9Y`\xD2\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0`\x01\x81\x90U\x80T`@Qc\x13w\xD1\xF5`\xE2\x1B\x81R\x82\x91\x82\x91\x82\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cM\xDFG\xD4\x90a\x05@\x90\x8D\x90\x8D\x90`\x04\x01a\x14\xFCV[`\xA0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x05\x98W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x160\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x05\xACW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xD0\x91\x90a\x15+V[\x94P\x94P\x94P\x94P\x94P\x84a\x06\x0EW`\0\x84\x12a\x05\xEC\x85a\t\xEFV[`@Qcw`m)`\xE1\x1B\x81R\x91\x15\x15`\x04\x83\x01R`$\x82\x01R`D\x01a\x04\xF7V[`\0\x80T`\xFF`\xA0\x1B\x19\x16`\x01`\xA0\x1B\x17\x81U`\x04\x84\x81U`\x05\x84\x90U`\x06\x83\x90U3\x80\x83R`\x07` R`@\x92\x83\x90 \x84\x90U`\x02T\x92Qc#\xB8r\xDD`\xE0\x1B\x81R\x91\x82\x01R0`$\x82\x01R`D\x81\x01\x85\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90`d\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x06\xCFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x160\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x06\xE3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x07\x91\x90a\x15wV[P`\x03T`@Qc#\xB8r\xDD`\xE0\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90`d\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x07\x98W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x160\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x07\xACW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\xD0\x91\x90a\x15wV[P`\x01\x80U\x91\x99\x90\x98P\x90\x96P\x94PPPPPV[`\x01T`\x01\x14a\x08 W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x06`$\x82\x01Re\x1B\x1B\xD8\xDA\xD9Y`\xD2\x1B`D\x82\x01R`d\x01a\x04\xF7V[`\0`\x01\x81\x90UT`\x01`\xA0\x1B\x90\x04`\xFF\x16a\x08pW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rn\x1B\x9B\xDD\x08\x1A[\x9A]\x1AX[\x1A^\x99Y`\x8A\x1B`D\x82\x01R`d\x01a\x04\xF7V[`\0\x80T`@Qc\xC1nP\xEF`\xE0\x1B\x81R\x82\x91\x82\x91\x82\x91\x82\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xC1nP\xEF\x90a\x08\xAB\x90\x8B\x90\x8B\x90`\x04\x01a\x14\xFCV[`\xC0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\t\x01W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x160\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\t\x15W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t9\x91\x90a\x15\x97V[\x95P\x95P\x95P\x95P\x95P\x95P\x85a\tWW`\0\x85\x12a\x05\xEC\x86a\t\xEFV[`\x06\x81\x90U`\0\x80\x80\x80a\tk\x87\x87a\n*V[`\0T`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x80\x82R` \x82\x01\x86\x90R\x91\x81\x01\x84\x90R``\x81\x01\x8F\x90R\x95\x99P\x93\x97P\x91\x95P\x93P3\x92\x8B\x91\x80\x88\x16\x91\x90\x89\x16\x90\x85\x90\x7F\xD4\xDB\xFA\xA7\xC0\xFEQ\xEE\xE2J\x1C\xDC\xAF\xD07\x8C\xD3e,\xDB\xA75\xDF\xC4\xB0a\xAAU\xC2\xA4X\xA5\x90`\x80\x01`@Q\x80\x91\x03\x90\xA4PP`\x01\x80UPPPPPPPPPPPPPV[`\0`\x01`\xFF\x1B\x82\x03a\n\x15W`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x12\x15a\n&WP\x19`\x01\x01\x90V[P\x90V[`\x04T`\x05T`\0\x91\x82\x91\x82\x91\x82\x91\x81\x88\x11\x15a\n\xC3W`\x02T`\x03T`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x97P\x16\x94Pa\na\x82\x89a\x16\x03V[\x93P\x86\x81\x11a\n\xB2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7Finvalid swap: inputs x and y\0\0\0\0`D\x82\x01R`d\x01a\x04\xF7V[a\n\xBC\x87\x82a\x16\x03V[\x92Pa\x0BAV[`\x03T`\x02T`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x97P\x16\x94Pa\n\xE3\x81\x88a\x16\x03V[\x93P\x87\x82\x11a\x0B4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7Finvalid swap: inputs x and y\0\0\0\0`D\x82\x01R`d\x01a\x04\xF7V[a\x0B>\x88\x83a\x16\x03V[\x92P[`\x04\x88\x81U`\x05\x88\x90U`@Qcp\xA0\x821`\xE0\x1B\x81R0\x91\x81\x01\x91\x90\x91R`\0\x90`\x01`\x01`\xA0\x1B\x03\x88\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0B\xCDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x160\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0B\xE1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\x05\x91\x90a\x14\xE0V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x88\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0C\x88W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x160\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0C\x9CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xC0\x91\x90a\x14\xE0V[`@Qc#\xB8r\xDD`\xE0\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x88\x90R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x89\x16\x90c#\xB8r\xDD\x90`d\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\rOW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x160\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\rcW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\x87\x91\x90a\x15wV[P`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x86\x90R`\x01`\x01`\xA0\x1B\x03\x88\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0E\x0EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x160\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x0E\"W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0EF\x91\x90a\x15wV[Pa\x0EQ\x86\x83a\x16\x1CV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x8A\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0E\xCEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x160\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0E\xE2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\x06\x91\x90a\x14\xE0V[\x10\x15a\x0F_W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: input token transf`D\x82\x01Ra2\xB9`\xF1\x1B`d\x82\x01R`\x84\x01a\x04\xF7V[a\x0Fi\x85\x82a\x16\x03V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x89\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0F\xE6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x160\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0F\xFAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\x1E\x91\x90a\x14\xE0V[\x10\x15a\x10xW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7Finvalid swap: output token trans`D\x82\x01Rb32\xB9`\xE9\x1B`d\x82\x01R`\x84\x01a\x04\xF7V[PPPP\x92\x95\x91\x94P\x92PV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[\x80\x15\x15\x81\x14a\x113W`\0\x80\xFD[PV[`\0\x80`@\x83\x85\x03\x12\x15a\x11LWa\x11La\x10\x85V[\x825a\x11W\x81a\x11%V[\x94` \x93\x90\x93\x015\x93PPPV[`\0[\x83\x81\x10\x15a\x11\x80W\x81\x81\x01Q\x83\x82\x01R` \x01a\x11hV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x11\xA1\x81` \x86\x01` \x86\x01a\x11eV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[\x83\x15\x15\x81R\x82` \x82\x01R```@\x82\x01R`\0a\x11\xD6``\x83\x01\x84a\x11\x89V[\x95\x94PPPPPV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x81\xFD[`\0\x80` \x83\x85\x03\x12\x15a\x12NWa\x12Na\x10\x85V[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x12iWa\x12ia\x10\xD5V[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x12\x80Wa\x12\x80a\x11\xDFV[\x815\x81\x81\x11\x15a\x12\xE3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x0ENL/$\r\x8C\xAD\xCC\xEE\x8D`\xAB\x1B`d\x82\x01R`\x84\x81\xFD[\x86` \x82\x85\x01\x01\x11\x15a\x13KW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rjrray stride`\xA8\x1B`d\x82\x01R\x91P`\x84\x82\xFD[` \x92\x90\x92\x01\x96\x91\x95P\x90\x93PPPPV[`\0` \x82\x84\x03\x12\x15a\x13rWa\x13ra\x10\x85V[\x815`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x13\x89W`\0\x80\xFD[\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15a\x13\xBEWa\x13\xBEa\x10\x85V[\x83Qa\x13\xC9\x81a\x11%V[` \x85\x01Q`@\x86\x01Q\x91\x94P\x92Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x13\xF1Wa\x13\xF1a\x10\xD5V[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x14\x08Wa\x14\x08a\x11\xDFV[\x81Q\x81\x81\x11\x15a\x14\x1AWa\x14\x1Aa\x13\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a\x14BWa\x14Ba\x13\x90V[\x81`@R\x82\x81R\x89` \x84\x87\x01\x01\x11\x15a\x14\xADW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R\x93P`\x84\x84\xFD[a\x14\xBE\x83` \x83\x01` \x88\x01a\x11eV[\x80\x95PPPPPP\x92P\x92P\x92V[` \x81R`\0a\x13\x89` \x83\x01\x84a\x11\x89V[`\0` \x82\x84\x03\x12\x15a\x14\xF5Wa\x14\xF5a\x10\x85V[PQ\x91\x90PV[` \x81R\x81` \x82\x01R\x81\x83`@\x83\x017`\0\x81\x83\x01`@\x90\x81\x01\x91\x90\x91R`\x1F\x90\x92\x01`\x1F\x19\x16\x01\x01\x91\x90PV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\x15FWa\x15Fa\x10\x85V[\x85Qa\x15Q\x81a\x11%V[` \x87\x01Q`@\x88\x01Q``\x89\x01Q`\x80\x90\x99\x01Q\x92\x9A\x91\x99P\x97\x96P\x90\x94P\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x15\x8CWa\x15\x8Ca\x10\x85V[\x81Qa\x13\x89\x81a\x11%V[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a\x15\xB3Wa\x15\xB3a\x10\x85V[\x86Qa\x15\xBE\x81a\x11%V[` \x88\x01Q`@\x89\x01Q``\x8A\x01Q`\x80\x8B\x01Q`\xA0\x90\x9B\x01Q\x93\x9C\x92\x9BP\x90\x99\x90\x98P\x96P\x90\x94P\x92PPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x16\x16Wa\x16\x16a\x15\xEDV[\x92\x91PPV[\x80\x82\x01\x80\x82\x11\x15a\x16\x16Wa\x16\x16a\x15\xEDV\xFETarget contract does not contain";
    /// The deployed bytecode of the contract.
    pub static DFMM_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct DFMM<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for DFMM<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for DFMM<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for DFMM<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for DFMM<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(DFMM)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> DFMM<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    DFMM_ABI.clone(),
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
                DFMM_ABI.clone(),
                DFMM_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `balanceOf` (0x70a08231) function
        pub fn balance_of(
            &self,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], account)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getReservesAndLiquidity` (0xebadef01) function
        pub fn get_reserves_and_liquidity(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([235, 173, 239, 1], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getSwapConstant` (0x19638234) function
        pub fn get_swap_constant(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([25, 99, 130, 52], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `init` (0x4ddf47d4) function
        pub fn init(
            &self,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([77, 223, 71, 212], data)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `inited` (0x43c885ba) function
        pub fn inited(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([67, 200, 133, 186], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `locked` (0xcf309012) function
        pub fn locked(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([207, 48, 144, 18], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `reserveXWad` (0xf065957f) function
        pub fn reserve_x_wad(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([240, 101, 149, 127], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `reserveYWad` (0x5e227d58) function
        pub fn reserve_y_wad(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([94, 34, 125, 88], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `simulateSwap` (0x0ac33034) function
        pub fn simulate_swap(
            &self,
            swap_x_in: bool,
            amount_in: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (bool, ::ethers::core::types::U256, ::ethers::core::types::Bytes),
        > {
            self.0
                .method_hash([10, 195, 48, 52], (swap_x_in, amount_in))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `source` (0x67e828bf) function
        pub fn source(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([103, 232, 40, 191], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `swap` (0x627dd56a) function
        pub fn swap(
            &self,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([98, 125, 213, 106], data)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `tokenX` (0x16dc165b) function
        pub fn token_x(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([22, 220, 22, 91], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `tokenY` (0xb7d19fc4) function
        pub fn token_y(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([183, 209, 159, 196], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `totalLiquidity` (0x15770f92) function
        pub fn total_liquidity(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([21, 119, 15, 146], ())
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `Swap` event
        pub fn swap_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, SwapFilter> {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, SwapFilter> {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for DFMM<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `Invalid` with signature `Invalid(bool,uint256)` and selector `0xeec0da52`
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
    #[etherror(name = "Invalid", abi = "Invalid(bool,uint256)")]
    pub struct Invalid {
        pub negative: bool,
        pub swap_constant_growth: ::ethers::core::types::U256,
    }
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
    pub enum DFMMErrors {
        Invalid(Invalid),
        Min(Min),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for DFMMErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <Invalid as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Invalid(decoded));
            }
            if let Ok(decoded) = <Min as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Min(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for DFMMErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::Invalid(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Min(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for DFMMErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <Invalid as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <Min as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for DFMMErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Invalid(element) => ::core::fmt::Display::fmt(element, f),
                Self::Min(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for DFMMErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<Invalid> for DFMMErrors {
        fn from(value: Invalid) -> Self {
            Self::Invalid(value)
        }
    }
    impl ::core::convert::From<Min> for DFMMErrors {
        fn from(value: Min) -> Self {
            Self::Min(value)
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
    #[ethevent(
        name = "Swap",
        abi = "Swap(address,address,address,address,uint256,uint256,int256)"
    )]
    pub struct SwapFilter {
        #[ethevent(indexed)]
        pub swapper: ::ethers::core::types::Address,
        pub source: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub token_in: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub token_out: ::ethers::core::types::Address,
        pub amount_in: ::ethers::core::types::U256,
        pub amount_out: ::ethers::core::types::U256,
        pub liquidity_delta: ::ethers::core::types::I256,
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
    pub struct BalanceOfCall {
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getReservesAndLiquidity` function with signature `getReservesAndLiquidity()` and selector `0xebadef01`
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
    #[ethcall(name = "getReservesAndLiquidity", abi = "getReservesAndLiquidity()")]
    pub struct GetReservesAndLiquidityCall;
    ///Container type for all input parameters for the `getSwapConstant` function with signature `getSwapConstant()` and selector `0x19638234`
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
    #[ethcall(name = "getSwapConstant", abi = "getSwapConstant()")]
    pub struct GetSwapConstantCall;
    ///Container type for all input parameters for the `init` function with signature `init(bytes)` and selector `0x4ddf47d4`
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
    #[ethcall(name = "init", abi = "init(bytes)")]
    pub struct InitCall {
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `inited` function with signature `inited()` and selector `0x43c885ba`
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
    #[ethcall(name = "inited", abi = "inited()")]
    pub struct InitedCall;
    ///Container type for all input parameters for the `locked` function with signature `locked()` and selector `0xcf309012`
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
    #[ethcall(name = "locked", abi = "locked()")]
    pub struct LockedCall;
    ///Container type for all input parameters for the `reserveXWad` function with signature `reserveXWad()` and selector `0xf065957f`
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
    #[ethcall(name = "reserveXWad", abi = "reserveXWad()")]
    pub struct ReserveXWadCall;
    ///Container type for all input parameters for the `reserveYWad` function with signature `reserveYWad()` and selector `0x5e227d58`
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
    #[ethcall(name = "reserveYWad", abi = "reserveYWad()")]
    pub struct ReserveYWadCall;
    ///Container type for all input parameters for the `simulateSwap` function with signature `simulateSwap(bool,uint256)` and selector `0x0ac33034`
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
    #[ethcall(name = "simulateSwap", abi = "simulateSwap(bool,uint256)")]
    pub struct SimulateSwapCall {
        pub swap_x_in: bool,
        pub amount_in: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `source` function with signature `source()` and selector `0x67e828bf`
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
    #[ethcall(name = "source", abi = "source()")]
    pub struct SourceCall;
    ///Container type for all input parameters for the `swap` function with signature `swap(bytes)` and selector `0x627dd56a`
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
    #[ethcall(name = "swap", abi = "swap(bytes)")]
    pub struct SwapCall {
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `tokenX` function with signature `tokenX()` and selector `0x16dc165b`
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
    #[ethcall(name = "tokenX", abi = "tokenX()")]
    pub struct TokenXCall;
    ///Container type for all input parameters for the `tokenY` function with signature `tokenY()` and selector `0xb7d19fc4`
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
    #[ethcall(name = "tokenY", abi = "tokenY()")]
    pub struct TokenYCall;
    ///Container type for all input parameters for the `totalLiquidity` function with signature `totalLiquidity()` and selector `0x15770f92`
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
    #[ethcall(name = "totalLiquidity", abi = "totalLiquidity()")]
    pub struct TotalLiquidityCall;
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
    pub enum DFMMCalls {
        BalanceOf(BalanceOfCall),
        GetReservesAndLiquidity(GetReservesAndLiquidityCall),
        GetSwapConstant(GetSwapConstantCall),
        Init(InitCall),
        Inited(InitedCall),
        Locked(LockedCall),
        ReserveXWad(ReserveXWadCall),
        ReserveYWad(ReserveYWadCall),
        SimulateSwap(SimulateSwapCall),
        Source(SourceCall),
        Swap(SwapCall),
        TokenX(TokenXCall),
        TokenY(TokenYCall),
        TotalLiquidity(TotalLiquidityCall),
    }
    impl ::ethers::core::abi::AbiDecode for DFMMCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <BalanceOfCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BalanceOf(decoded));
            }
            if let Ok(decoded) = <GetReservesAndLiquidityCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetReservesAndLiquidity(decoded));
            }
            if let Ok(decoded) = <GetSwapConstantCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetSwapConstant(decoded));
            }
            if let Ok(decoded) = <InitCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Init(decoded));
            }
            if let Ok(decoded) = <InitedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Inited(decoded));
            }
            if let Ok(decoded) = <LockedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Locked(decoded));
            }
            if let Ok(decoded) = <ReserveXWadCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ReserveXWad(decoded));
            }
            if let Ok(decoded) = <ReserveYWadCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ReserveYWad(decoded));
            }
            if let Ok(decoded) = <SimulateSwapCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SimulateSwap(decoded));
            }
            if let Ok(decoded) = <SourceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Source(decoded));
            }
            if let Ok(decoded) = <SwapCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Swap(decoded));
            }
            if let Ok(decoded) = <TokenXCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TokenX(decoded));
            }
            if let Ok(decoded) = <TokenYCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TokenY(decoded));
            }
            if let Ok(decoded) = <TotalLiquidityCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TotalLiquidity(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for DFMMCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::BalanceOf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetReservesAndLiquidity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetSwapConstant(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Init(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Inited(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Locked(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ReserveXWad(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ReserveYWad(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SimulateSwap(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Source(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Swap(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TokenX(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TokenY(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TotalLiquidity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for DFMMCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::BalanceOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetReservesAndLiquidity(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetSwapConstant(element) => ::core::fmt::Display::fmt(element, f),
                Self::Init(element) => ::core::fmt::Display::fmt(element, f),
                Self::Inited(element) => ::core::fmt::Display::fmt(element, f),
                Self::Locked(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReserveXWad(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReserveYWad(element) => ::core::fmt::Display::fmt(element, f),
                Self::SimulateSwap(element) => ::core::fmt::Display::fmt(element, f),
                Self::Source(element) => ::core::fmt::Display::fmt(element, f),
                Self::Swap(element) => ::core::fmt::Display::fmt(element, f),
                Self::TokenX(element) => ::core::fmt::Display::fmt(element, f),
                Self::TokenY(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalLiquidity(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<BalanceOfCall> for DFMMCalls {
        fn from(value: BalanceOfCall) -> Self {
            Self::BalanceOf(value)
        }
    }
    impl ::core::convert::From<GetReservesAndLiquidityCall> for DFMMCalls {
        fn from(value: GetReservesAndLiquidityCall) -> Self {
            Self::GetReservesAndLiquidity(value)
        }
    }
    impl ::core::convert::From<GetSwapConstantCall> for DFMMCalls {
        fn from(value: GetSwapConstantCall) -> Self {
            Self::GetSwapConstant(value)
        }
    }
    impl ::core::convert::From<InitCall> for DFMMCalls {
        fn from(value: InitCall) -> Self {
            Self::Init(value)
        }
    }
    impl ::core::convert::From<InitedCall> for DFMMCalls {
        fn from(value: InitedCall) -> Self {
            Self::Inited(value)
        }
    }
    impl ::core::convert::From<LockedCall> for DFMMCalls {
        fn from(value: LockedCall) -> Self {
            Self::Locked(value)
        }
    }
    impl ::core::convert::From<ReserveXWadCall> for DFMMCalls {
        fn from(value: ReserveXWadCall) -> Self {
            Self::ReserveXWad(value)
        }
    }
    impl ::core::convert::From<ReserveYWadCall> for DFMMCalls {
        fn from(value: ReserveYWadCall) -> Self {
            Self::ReserveYWad(value)
        }
    }
    impl ::core::convert::From<SimulateSwapCall> for DFMMCalls {
        fn from(value: SimulateSwapCall) -> Self {
            Self::SimulateSwap(value)
        }
    }
    impl ::core::convert::From<SourceCall> for DFMMCalls {
        fn from(value: SourceCall) -> Self {
            Self::Source(value)
        }
    }
    impl ::core::convert::From<SwapCall> for DFMMCalls {
        fn from(value: SwapCall) -> Self {
            Self::Swap(value)
        }
    }
    impl ::core::convert::From<TokenXCall> for DFMMCalls {
        fn from(value: TokenXCall) -> Self {
            Self::TokenX(value)
        }
    }
    impl ::core::convert::From<TokenYCall> for DFMMCalls {
        fn from(value: TokenYCall) -> Self {
            Self::TokenY(value)
        }
    }
    impl ::core::convert::From<TotalLiquidityCall> for DFMMCalls {
        fn from(value: TotalLiquidityCall) -> Self {
            Self::TotalLiquidity(value)
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
        Hash
    )]
    pub struct BalanceOfReturn {
        pub balance: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `getReservesAndLiquidity` function with signature `getReservesAndLiquidity()` and selector `0xebadef01`
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
    pub struct GetReservesAndLiquidityReturn(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `getSwapConstant` function with signature `getSwapConstant()` and selector `0x19638234`
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
    pub struct GetSwapConstantReturn(pub ::ethers::core::types::I256);
    ///Container type for all return fields from the `init` function with signature `init(bytes)` and selector `0x4ddf47d4`
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
    pub struct InitReturn(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `inited` function with signature `inited()` and selector `0x43c885ba`
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
    pub struct InitedReturn(pub bool);
    ///Container type for all return fields from the `locked` function with signature `locked()` and selector `0xcf309012`
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
    pub struct LockedReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `reserveXWad` function with signature `reserveXWad()` and selector `0xf065957f`
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
    pub struct ReserveXWadReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `reserveYWad` function with signature `reserveYWad()` and selector `0x5e227d58`
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
    pub struct ReserveYWadReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `simulateSwap` function with signature `simulateSwap(bool,uint256)` and selector `0x0ac33034`
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
    pub struct SimulateSwapReturn {
        pub valid: bool,
        pub estimated_out: ::ethers::core::types::U256,
        pub swap_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all return fields from the `source` function with signature `source()` and selector `0x67e828bf`
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
    pub struct SourceReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `tokenX` function with signature `tokenX()` and selector `0x16dc165b`
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
    pub struct TokenXReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `tokenY` function with signature `tokenY()` and selector `0xb7d19fc4`
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
    pub struct TokenYReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `totalLiquidity` function with signature `totalLiquidity()` and selector `0x15770f92`
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
    pub struct TotalLiquidityReturn(pub ::ethers::core::types::U256);
}
