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
                    ::std::borrow::ToOwned::to_owned("internalPrice"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("internalPrice"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("price"),
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
                                    name: ::std::borrow::ToOwned::to_owned("estimatedPrice"),
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
    const __BYTECODE: &[u8] = b"`\x80`@R`\x01\x80U4\x80\x15b\0\0bW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`@Qb\0F 8\x03\x80b\0F \x839\x81\x01`@\x81\x90Rb\0\0\x85\x91b\0\x01>V[`\x02\x80T`\x01`\x01`\xA0\x1B\x03\x80\x86\x16`\x01`\x01`\xA0\x1B\x03\x19\x92\x83\x16\x17\x90\x92U`\x03\x80T\x92\x85\x16\x92\x90\x91\x16\x91\x90\x91\x17\x90U`@Q\x81\x90b\0\0\xC5\x90b\0\x01\x13V[\x90\x81R` \x01`@Q\x80\x91\x03\x90`\0\xF0\x80\x15\x80\x15b\0\0\xE8W=`\0\x80>=`\0\xFD[P`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UPb\0\x01\xCA\x91PPV[a-\x03\x80b\0\x19\x1D\x839\x01\x90V[\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x019W`\0\x80\xFD[\x91\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15b\0\x01\x9FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[b\0\x01\xAA\x84b\0\x01!V[\x92Pb\0\x01\xBA` \x85\x01b\0\x01!V[\x91P`@\x84\x01Q\x90P\x92P\x92P\x92V[a\x17C\x80b\0\x01\xDA`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x01BW`\x005`\xE0\x1C\x80cb}\xD5j\x11a\0\xE4W\x80c\xB7\xD1\x9F\xC4\x11a\0\xB3W\x80c\xB7\xD1\x9F\xC4\x14a\x02\xC8W\x80c\xCF0\x90\x12\x14a\x02\xDBW\x80c\xEB\xAD\xEF\x01\x14a\x02\xE4W\x80c\xF0e\x95\x7F\x14a\x02\xF2Wa\x01BV[\x80cb}\xD5j\x14a\x02xW\x80cg\xE8(\xBF\x14a\x02\x8DW\x80cp\xA0\x821\x14a\x02\xA0W\x80c\x85\x93\x10\xB6\x14a\x02\xC0Wa\x01BV[\x80c\x19c\x824\x11a\x01 W\x80c\x19c\x824\x14a\x02\x15W\x80cC\xC8\x85\xBA\x14a\x02\x1DW\x80cM\xDFG\xD4\x14a\x02AW\x80c^\"}X\x14a\x02oWa\x01BV[\x80c\n\xC304\x14a\x01\xA7W\x80c\x15w\x0F\x92\x14a\x01\xD3W\x80c\x16\xDC\x16[\x14a\x01\xEAW[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x01\xBAa\x01\xB56`\x04a\x12\x19V[a\x02\xFBV[`@Qa\x01\xCA\x94\x93\x92\x91\x90a\x12\x98V[`@Q\x80\x91\x03\x90\xF3[a\x01\xDC`\x06T\x81V[`@Q\x90\x81R` \x01a\x01\xCAV[`\x02Ta\x01\xFD\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\xCAV[a\x01\xDCa\x03\xD7V[`\0Ta\x021\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x01\xCAV[a\x02Ta\x02O6`\x04a\x13\"V[a\x04\xD2V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x01\xCAV[a\x01\xDC`\x05T\x81V[a\x02\x8Ba\x02\x866`\x04a\x13\"V[a\x07\xFCV[\0[`\0Ta\x01\xFD\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01\xDCa\x02\xAE6`\x04a\x14GV[`\x07` R`\0\x90\x81R`@\x90 T\x81V[a\x01\xDCa\n\x06V[`\x03Ta\x01\xFD\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01\xDC`\x01T\x81V[`\x04T`\x05T`\x06Ta\x02TV[a\x01\xDC`\x04T\x81V[`\0\x80T`@Qc\x02\xB0\xCC\r`\xE2\x1B\x81R\x84\x15\x15`\x04\x82\x01R`$\x81\x01\x84\x90R\x82\x91\x82\x91``\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\n\xC304\x90`D\x01`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x03\x8BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x17#\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x03\x9FW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x03\xC7\x91\x90\x81\x01\x90a\x14\x90V[\x92\x99\x91\x98P\x96P\x90\x94P\x92PPPV[`\x04T`\x05T`\x06T`@\x80Q` \x81\x01\x94\x90\x94R\x83\x01\x91\x90\x91R``\x82\x01R`\0\x90\x81\x90`\x80\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R`\0Tc\xA1\x9C\xD3\xD1`\xE0\x1B\x83R\x90\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\xA1\x9C\xD3\xD1\x90a\x04>\x90\x84\x90`\x04\x01a\x15\xC0V[` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x04\x94W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x17#\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x04\xA8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xCC\x91\x90a\x15\xD3V[\x91PP\x90V[`\0\x80`\0`\x01T`\x01\x14a\x05\x17W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x06`$\x82\x01Re\x1B\x1B\xD8\xDA\xD9Y`\xD2\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0`\x01\x81\x90U\x80T`@Qc\x13w\xD1\xF5`\xE2\x1B\x81R\x82\x91\x82\x91\x82\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cM\xDFG\xD4\x90a\x05W\x90\x8D\x90\x8D\x90`\x04\x01a\x15\xEFV[`\xA0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x05\xAFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x17#\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x05\xC3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xE7\x91\x90a\x16\x1EV[\x94P\x94P\x94P\x94P\x94P\x84a\x06%W`\0\x84\x12a\x06\x03\x85a\n\xD2V[`@Qcw`m)`\xE1\x1B\x81R\x91\x15\x15`\x04\x83\x01R`$\x82\x01R`D\x01a\x05\x0EV[`\0\x80T`\xFF`\xA0\x1B\x19\x16`\x01`\xA0\x1B\x17\x81U`\x04\x84\x81U`\x05\x84\x90U`\x06\x83\x90U3\x80\x83R`\x07` R`@\x92\x83\x90 \x84\x90U`\x02T\x92Qc#\xB8r\xDD`\xE0\x1B\x81R\x91\x82\x01R0`$\x82\x01R`D\x81\x01\x85\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90`d\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x06\xE6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x17#\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x06\xFAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x1E\x91\x90a\x16jV[P`\x03T`@Qc#\xB8r\xDD`\xE0\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90`d\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x07\xAFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x17#\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x07\xC3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\xE7\x91\x90a\x16jV[P`\x01\x80U\x91\x99\x90\x98P\x90\x96P\x94PPPPPV[`\x01T`\x01\x14a\x087W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x06`$\x82\x01Re\x1B\x1B\xD8\xDA\xD9Y`\xD2\x1B`D\x82\x01R`d\x01a\x05\x0EV[`\0`\x01\x81\x90UT`\x01`\xA0\x1B\x90\x04`\xFF\x16a\x08\x87W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rn\x1B\x9B\xDD\x08\x1A[\x9A]\x1AX[\x1A^\x99Y`\x8A\x1B`D\x82\x01R`d\x01a\x05\x0EV[`\0\x80T`@Qc\xC1nP\xEF`\xE0\x1B\x81R\x82\x91\x82\x91\x82\x91\x82\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xC1nP\xEF\x90a\x08\xC2\x90\x8B\x90\x8B\x90`\x04\x01a\x15\xEFV[`\xC0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\t\x18W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x17#\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\t,W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\tP\x91\x90a\x16\x8AV[\x95P\x95P\x95P\x95P\x95P\x95P\x85a\tnW`\0\x85\x12a\x06\x03\x86a\n\xD2V[`\x06\x81\x90U`\0\x80\x80\x80a\t\x82\x87\x87a\x0B\rV[`\0T`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x80\x82R` \x82\x01\x86\x90R\x91\x81\x01\x84\x90R``\x81\x01\x8F\x90R\x95\x99P\x93\x97P\x91\x95P\x93P3\x92\x8B\x91\x80\x88\x16\x91\x90\x89\x16\x90\x85\x90\x7F\xD4\xDB\xFA\xA7\xC0\xFEQ\xEE\xE2J\x1C\xDC\xAF\xD07\x8C\xD3e,\xDB\xA75\xDF\xC4\xB0a\xAAU\xC2\xA4X\xA5\x90`\x80\x01`@Q\x80\x91\x03\x90\xA4PP`\x01\x80UPPPPPPPPPPPPPV[`\0\x80T`\x04\x80T`\x06T`@Qc\\x\x05\xB9`\xE1\x1B\x81R\x92\x83\x01\x91\x90\x91R`$\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xB8\xF0\x0Br\x90`D\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\n\x95W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x17#\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\n\xA9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\xCD\x91\x90a\x15\xD3V[\x90P\x90V[`\0`\x01`\xFF\x1B\x82\x03a\n\xF8W`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x12\x15a\x0B\tWP\x19`\x01\x01\x90V[P\x90V[`\x04T`\x05T`\0\x91\x82\x91\x82\x91\x82\x91\x81\x88\x11\x15a\x0B\xA6W`\x02T`\x03T`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x97P\x16\x94Pa\x0BD\x82\x89a\x16\xF6V[\x93P\x86\x81\x11a\x0B\x95W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7Finvalid swap: inputs x and y\0\0\0\0`D\x82\x01R`d\x01a\x05\x0EV[a\x0B\x9F\x87\x82a\x16\xF6V[\x92Pa\x0C$V[`\x03T`\x02T`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x97P\x16\x94Pa\x0B\xC6\x81\x88a\x16\xF6V[\x93P\x87\x82\x11a\x0C\x17W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7Finvalid swap: inputs x and y\0\0\0\0`D\x82\x01R`d\x01a\x05\x0EV[a\x0C!\x88\x83a\x16\xF6V[\x92P[`\x04\x88\x81U`\x05\x88\x90U`@Qcp\xA0\x821`\xE0\x1B\x81R0\x91\x81\x01\x91\x90\x91R`\0\x90`\x01`\x01`\xA0\x1B\x03\x88\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0C\xB0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x17#\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0C\xC4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xE8\x91\x90a\x15\xD3V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x88\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\rkW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x17#\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\r\x7FW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\xA3\x91\x90a\x15\xD3V[`@Qc#\xB8r\xDD`\xE0\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x88\x90R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x89\x16\x90c#\xB8r\xDD\x90`d\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0E2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x17#\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x0EFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Ej\x91\x90a\x16jV[P`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x86\x90R`\x01`\x01`\xA0\x1B\x03\x88\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0E\xF1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x17#\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x0F\x05W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F)\x91\x90a\x16jV[Pa\x0F4\x86\x83a\x17\x0FV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x8A\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0F\xB1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x17#\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0F\xC5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xE9\x91\x90a\x15\xD3V[\x10\x15a\x10BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: input token transf`D\x82\x01Ra2\xB9`\xF1\x1B`d\x82\x01R`\x84\x01a\x05\x0EV[a\x10L\x85\x82a\x16\xF6V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x89\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x10\xC9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x17#\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x10\xDDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\x01\x91\x90a\x15\xD3V[\x10\x15a\x11[W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7Finvalid swap: output token trans`D\x82\x01Rb32\xB9`\xE9\x1B`d\x82\x01R`\x84\x01a\x05\x0EV[PPPP\x92\x95\x91\x94P\x92PV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[\x80\x15\x15\x81\x14a\x12\x16W`\0\x80\xFD[PV[`\0\x80`@\x83\x85\x03\x12\x15a\x12/Wa\x12/a\x11hV[\x825a\x12:\x81a\x12\x08V[\x94` \x93\x90\x93\x015\x93PPPV[`\0[\x83\x81\x10\x15a\x12cW\x81\x81\x01Q\x83\x82\x01R` \x01a\x12KV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x12\x84\x81` \x86\x01` \x86\x01a\x12HV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[\x84\x15\x15\x81R\x83` \x82\x01R\x82`@\x82\x01R`\x80``\x82\x01R`\0a\x12\xBF`\x80\x83\x01\x84a\x12lV[\x96\x95PPPPPPV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x81\xFD[`\0\x80` \x83\x85\x03\x12\x15a\x138Wa\x138a\x11hV[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x13SWa\x13Sa\x11\xB8V[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x13jWa\x13ja\x12\xC9V[\x815\x81\x81\x11\x15a\x13\xCDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x0ENL/$\r\x8C\xAD\xCC\xEE\x8D`\xAB\x1B`d\x82\x01R`\x84\x81\xFD[\x86` \x82\x85\x01\x01\x11\x15a\x145W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rjrray stride`\xA8\x1B`d\x82\x01R\x91P`\x84\x82\xFD[` \x92\x90\x92\x01\x96\x91\x95P\x90\x93PPPPV[`\0` \x82\x84\x03\x12\x15a\x14\\Wa\x14\\a\x11hV[\x815`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x14sW`\0\x80\xFD[\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x14\xA9Wa\x14\xA9a\x11hV[\x84Qa\x14\xB4\x81a\x12\x08V[\x80\x94PP` \x85\x01Q\x92P`@\x85\x01Q\x91P``\x85\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x14\xE4Wa\x14\xE4a\x11\xB8V[\x81\x87\x01\x91P\x87`\x1F\x83\x01\x12a\x14\xFBWa\x14\xFBa\x12\xC9V[\x81Q\x81\x81\x11\x15a\x15\rWa\x15\ra\x14zV[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a\x155Wa\x155a\x14zV[\x81`@R\x82\x81R\x8A` \x84\x87\x01\x01\x11\x15a\x15\xA0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R\x93P`\x84\x84\xFD[a\x15\xB1\x83` \x83\x01` \x88\x01a\x12HV[\x97\x9A\x96\x99P\x94\x97PPPPPPV[` \x81R`\0a\x14s` \x83\x01\x84a\x12lV[`\0` \x82\x84\x03\x12\x15a\x15\xE8Wa\x15\xE8a\x11hV[PQ\x91\x90PV[` \x81R\x81` \x82\x01R\x81\x83`@\x83\x017`\0\x81\x83\x01`@\x90\x81\x01\x91\x90\x91R`\x1F\x90\x92\x01`\x1F\x19\x16\x01\x01\x91\x90PV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\x169Wa\x169a\x11hV[\x85Qa\x16D\x81a\x12\x08V[` \x87\x01Q`@\x88\x01Q``\x89\x01Q`\x80\x90\x99\x01Q\x92\x9A\x91\x99P\x97\x96P\x90\x94P\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x16\x7FWa\x16\x7Fa\x11hV[\x81Qa\x14s\x81a\x12\x08V[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a\x16\xA6Wa\x16\xA6a\x11hV[\x86Qa\x16\xB1\x81a\x12\x08V[` \x88\x01Q`@\x89\x01Q``\x8A\x01Q`\x80\x8B\x01Q`\xA0\x90\x9B\x01Q\x93\x9C\x92\x9BP\x90\x99\x90\x98P\x96P\x90\x94P\x92PPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x17\tWa\x17\ta\x16\xE0V[\x92\x91PPV[\x80\x82\x01\x80\x82\x11\x15a\x17\tWa\x17\ta\x16\xE0V\xFETarget contract does not contain`\x80`@R4\x80\x15b\0\0^W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`@Qb\0-\x038\x03\x80b\0-\x03\x839\x81\x01`@\x81\x90Rb\0\0\x81\x91b\0\0\x8AV[`\0Ub\0\0\xEFV[`\0` \x82\x84\x03\x12\x15b\0\0\xE8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[PQ\x91\x90PV[a,\x04\x80b\0\0\xFF`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x022W`\x005`\xE0\x1C\x80c\x85\xAAEN\x11a\x01\\W\x80c\xC1nP\xEF\x11a\0\xEFW\x80c\xE3\xD0\xDC\xA5\x11a\0\xBEW\x80c\xE3\xD0\xDC\xA5\x14a\x05/W\x80c\xE4\x93t(\x14a\x058W\x80c\xF9\xC2\x82\x11\x14a\x05KW\x80c\xFF\xB3bf\x14a\x05TWa\x022V[\x80c\xC1nP\xEF\x14a\x04\xB4W\x80c\xCDd\xAE\xA2\x14a\x04\xF6W\x80c\xD7\xF9{\xEF\x14a\x05\tW\x80c\xDB\xAF\x11B\x14a\x05\x1CWa\x022V[\x80c\xA1\x9C\xD3\xD1\x11a\x01+W\x80c\xA1\x9C\xD3\xD1\x14a\x04=W\x80c\xA6\xD34\x98\x14a\x04PW\x80c\xB8\xF0\x0Br\x14a\x04cW\x80c\xBDB-(\x14a\x04vWa\x022V[\x80c\x85\xAAEN\x14a\x03\xF5W\x80c\x88;m\xC5\x14a\x04\x04W\x80c\x97\x16\xAE\xBB\x14a\x04\x17W\x80c\x9D\x81\x9E\x83\x14a\x04*Wa\x022V[\x80c@\xB41i\x11a\x01\xD4W\x80cd\x17\xD4\xB5\x11a\x01\xA3W\x80cd\x17\xD4\xB5\x14a\x03\xC3W\x80cj\x14`$\x14a\x03\xD6W\x80cme\"\x99\x14a\x03\xE5W\x80c\x84\xC8*&\x14a\x03\xEDWa\x022V[\x80c@\xB41i\x14a\x03bW\x80cM\xDFG\xD4\x14a\x03kW\x80cV\x08\xBE\xA1\x14a\x03\xA8W\x80cX\xFAc\xCA\x14a\x03\xBBWa\x022V[\x80c\x18M0\xBA\x11a\x02\x10W\x80c\x18M0\xBA\x14a\x03\0W\x80c\x1A\x88\xBCf\x14a\x03\x13W\x80c\x1D\x9C\xF7\"\x14a\x03@W\x80c']g\xC8\x14a\x03SWa\x022V[\x80c\x06%\xA6#\x14a\x02\x97W\x80c\n\xC304\x14a\x02\xBDW\x80c\x0F\n\xA3\x95\x14a\x02\xE0W[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x02\xAAa\x02\xA56`\x04a#\xA4V[a\x05gV[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x02\xD0a\x02\xCB6`\x04a#\xE2V[a\x06SV[`@Qa\x02\xB4\x94\x93\x92\x91\x90a$\\V[a\x02\xF3a\x02\xEE6`\x04a%\x8BV[a\n\x8DV[`@Qa\x02\xB4\x91\x90a%\xCEV[a\x02\xAAa\x03\x0E6`\x04a%\xE1V[a\n\xBFV[`\x01T`\x02T`\x03Ta\x03%\x92\x91\x90\x83V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x02\xB4V[a\x02\xAAa\x03N6`\x04a&\x1AV[a\x0B\x11V[a\x02\xAAg\x1B\xC1mgN\xC8\0\0\x81V[a\x02\xAA`\0T\x81V[a\x03~a\x03y6`\x04a&\x8FV[a\x0BZV[`@\x80Q\x95\x15\x15\x86R` \x86\x01\x94\x90\x94R\x92\x84\x01\x91\x90\x91R``\x83\x01R`\x80\x82\x01R`\xA0\x01a\x02\xB4V[a\x02\xAAa\x03\xB66`\x04a%\xE1V[a\x0B\xF6V[a\x02\xAA`\0\x81V[a\x02\xAAa\x03\xD16`\x04a%\x8BV[a\x0C7V[a\x02\xAAg\r\xE0\xB6\xB3\xA7d\0\0\x81V[a\x02\xAA`\x01\x81V[a\x02\xAA`\n\x81V[a\x02\xAAg\x06\xF0[Y\xD3\xB2\0\0\x81V[a\x02\xAAa\x04\x126`\x04a%\x8BV[a\x0C\x91V[a\x02\xAAa\x04%6`\x04a'\xB4V[a\x0C\xEAV[a\x02\xAAa\x0486`\x04a%\xE1V[a\r\x87V[a\x02\xAAa\x04K6`\x04a'\xE4V[a\x0E\x1CV[a\x02\xAAa\x04^6`\x04a&\x1AV[a\x0EnV[a\x02\xAAa\x04q6`\x04a(\xD2V[a\x0E\x97V[a\x02\xF3a\x04\x846`\x04a(\xF7V[`@\x80Q` \x81\x01\x94\x90\x94R\x83\x81\x01\x92\x90\x92R``\x80\x84\x01\x91\x90\x91R\x81Q\x80\x84\x03\x90\x91\x01\x81R`\x80\x90\x92\x01\x90R\x90V[a\x04\xC7a\x04\xC26`\x04a'\xE4V[a\x0E\xB4V[`@\x80Q\x96\x15\x15\x87R` \x87\x01\x95\x90\x95R\x93\x85\x01\x92\x90\x92R``\x84\x01R`\x80\x83\x01R`\xA0\x82\x01R`\xC0\x01a\x02\xB4V[a\x02\xAAa\x05\x046`\x04a%\x8BV[a\x12CV[a\x02\xAAa\x05\x176`\x04a'\xB4V[a\x12\x8CV[a\x02\xAAa\x05*6`\x04a%\xE1V[a\x12\xB4V[a\x02\xAA`\0\x19\x81V[a\x02\xAAa\x05F6`\x04a(\xF7V[a\x12\xF6V[a\x02\xAAa\x01\0\x81V[a\x02\xAAa\x05b6`\x04a(\xF7V[a\x13tV[`\0\x80a\x05t\x84\x84a\x13\xFBV[\x90P`\0a\x05\x81\x85a\x0EnV[\x90P`\0a\x05\x8F\x82\x86a\x140V[\x90P`\0a\x05\x9D\x8A\x8Aa\x14EV[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0\x81\x12a\x05\xBBW`\0\x94PPPPPa\x06JV[`\0\x81\x13a\x05\xD1W`\0\x19\x94PPPPPa\x06JV[`\0a\x05\xEDa\x05\xE8\x83g\r\xE0\xB6\xB3\xA7d\0\0a)<V[a\x14ZV[\x90P`\0\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x06\x05\x88\x85a)cV[a\x06\x0F\x91\x90a)\xA9V[a\x06\x19\x91\x90a)<V[\x90P`\0a\x06&\x82a\x14\xF7V[\x90P`\0a\x063\x82a\x0B\x11V[\x90Pa\x06?\x8C\x82a\x16\xA0V[\x98PPPPPPPPP[\x95\x94PPPPPV[`\0\x80`\0``a\x06{`@Q\x80`\x80\x01`@R\x80`G\x81R` \x01a+\x98`G\x919a\x16\xB5V[`\0\x80`\x003`\x01`\x01`\xA0\x1B\x03\x16c\xEB\xAD\xEF\x01`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01```@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x07\tW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x07\x1DW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07A\x91\x90a)\xD7V[`@\x80Q` \x81\x01\x85\x90R\x90\x81\x01\x83\x90R``\x81\x01\x82\x90R\x92\x95P\x90\x93P\x91P`\0\x90a\x07\x7F\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@Ra\x0E\x1CV[\x90P`\0\x8A\x15a\x08\xCBW`\0\x80Ta\x07\x98\x90\x8C\x90a\x16\xA0V[\x90P`\0a\x07\xB0\x87a\x07\xAA\x84\x88a\x16\xA0V[\x90a\x16\xFBV[\x90Pa\x07\xBD`\x01\x82a*\x08V[\x90Pa\x07\xC9\x8C\x88a*\x08V[\x96Pa\x07\xD5\x81\x86a*\x08V[`@\x80Q``\x81\x01\x82R`\x01T\x81R`\x02T` \x82\x01R`\x03T\x91\x81\x01\x91\x90\x91R\x90\x95P\x86\x90a\x08\n\x90\x89\x90\x88\x90\x88\x90a\x0C7V[\x96Pa\x08\x17`\x01\x88a*\x08V[\x96P\x80\x87\x10a\x08xW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: y reserve increase`D\x82\x01Rad!`\xF0\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[a\x08\x82\x87\x82a*\x1BV[\x93Pa\x08\xC3`@Q\x80`@\x01`@R\x80`\x1C\x81R` \x01\x7FEsimated Y reserve to submit\0\0\0\0\x81RP\x88a\x17\x10V[PPPa\n\x17V[`\0\x80Ta\x08\xDA\x90\x8C\x90a\x16\xA0V[\x90P`\0a\x08\xEC\x86a\x07\xAA\x84\x88a\x16\xA0V[\x90Pa\x08\xF9`\x01\x82a*\x08V[\x90Pa\t\x05\x8C\x87a*\x08V[\x95Pa\t\x11\x81\x86a*\x08V[\x94P`\0\x87\x90Pa\tC`@Q\x80`@\x01`@R\x80`\t\x81R` \x01hadjustedY`\xB8\x1B\x81RP\x88a\x17\x10V[a\tn`@Q\x80`@\x01`@R\x80`\t\x81R` \x01h\x18Y\x1A\x9D\\\xDD\x19Y\x13`\xBA\x1B\x81RP\x87a\x17\x10V[`@\x80Q``\x81\x01\x82R`\x01T\x81R`\x02T` \x82\x01R`\x03T\x91\x81\x01\x91\x90\x91Ra\t\x9E\x90\x88\x90\x88\x90\x88\x90a\x12CV[\x97Pa\t\xAB`\x01\x89a*\x08V[\x97P\x80\x88\x10a\n\x07W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: x reserve increase`D\x82\x01Rad!`\xF0\x1B`d\x82\x01R`\x84\x01a\x08oV[a\n\x11\x88\x82a*\x1BV[\x93PPPP[`@\x80Q` \x81\x01\x87\x90R\x90\x81\x01\x85\x90R``\x81\x01\x84\x90R`\0\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`\0a\nQ\x82a\x0E\xB4V[PPPPP\x90P\x80\x83a\nu\x89\x88`\x01`\0\x01T`\x01\x80\x01T`\x01`\x02\x01Ta\x05gV[\x91\x9F\x90\x9EP\x90\x9CP\x91\x9AP\x90\x98PPPPPPPPPV[``\x84\x84\x84\x84`@Q` \x01a\n\xA6\x94\x93\x92\x91\x90a*.V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x94\x93PPPPV[`\0\x80a\n\xCC\x84\x84a\x0C\xEAV[\x90P`\0a\n\xD9\x82a\x17YV[\x90P`\0a\n\xE6\x82a\x0B\x11V[\x90Pa\x0B\x04a\n\xFD\x82g\r\xE0\xB6\xB3\xA7d\0\0a*\x1BV[\x88\x90a\x16\xA0V[\x93PPPP[\x93\x92PPPV[`\0\x80\x82\x12\x15a\x0BVW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01RotoUint: negative`\x80\x1B`D\x82\x01R`d\x01a\x08oV[P\x90V[`\0\x80`\0\x80`\0a\x0B\x83`@Q\x80`\x80\x01`@R\x80`G\x81R` \x01a+\x98`G\x919a\x16\xB5V[a\x0B\x8F\x86\x88\x01\x88a%\x8BV[\x80Q`\x01\x81\x90U` \x80\x83\x01Q`\x02\x81\x90U`@\x93\x84\x01Q`\x03\x81\x90U\x84Q``\x81\x01\x86R\x93\x84R\x91\x83\x01R\x91\x81\x01\x91\x90\x91R\x92\x95P\x90\x93P\x91Pa\x0B\xD9\x90\x84\x90\x84\x90\x84\x90a\x17\xC2V[\x93Pa\x0B\xE7`\0`\x03a*bV[\x84\x12\x15\x94P\x92\x95P\x92\x95\x90\x93PV[`\0\x80a\x0C\x03\x84\x84a\x12\x8CV[\x90P`\0a\x0C\x10\x82a\x17YV[\x90P`\0a\x0C\x1D\x82a\x0B\x11V[\x85Q\x90\x91Pa\x0B\x04\x90a\x0C0\x90\x83a\x140V[\x88\x90a\x14EV[\x80Q`\0\x90`\n\x90\x82\x90a\x0CL\x90\x83\x90a*\x1BV[\x90Pa\x0C\x86\x87\x87\x87\x87`@Q` \x01a\x0Ch\x94\x93\x92\x91\x90a*.V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83\x83`\x01a\x01\0a\x18ma\x18\xA8V[\x97\x96PPPPPPPV[`\0\x80a\x0C\x9F\x86`\x01a*\x08V[\x90P`\0k\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x90Pa\x0C\x86\x87\x87\x87\x87`@Q` \x01a\x0C\xCC\x94\x93\x92\x91\x90a*.V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83\x83`\x01a\x01\0a\x19\xB9a\x18\xA8V[`\0\x80a\x0C\xFF\x83` \x01Q\x84`@\x01Qa\x13\xFBV[\x90P`\0a\r\x10\x84` \x01Qa\x0EnV[\x90P`\0a\r3a\r.\x86`\0\x01Q\x88a\x16\xFB\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x19\xEAV[\x90P\x80a\r?\x81a*\x8AV[\x91PP`\0a\r[\x86`@\x01Q\x84a\x140\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\re\x90\x83a*bV[\x90P\x83a\r{\x82g\r\xE0\xB6\xB3\xA7d\0\0\x83a\x13tV[\x98\x97PPPPPPPPV[`\0\x80a\r\x94\x84\x84a\x0C\xEAV[\x90P`\0a\r\xA1\x82a\x17YV[\x90P`\0a\r\xAE\x82a\x0B\x11V[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0\x81\x10a\x0E\x07W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7Flx: denominator is zero\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x08oV[a\x0B\x04a\x0C0\x82g\r\xE0\xB6\xB3\xA7d\0\0a*\x1BV[`\0\x80`\0\x80\x84\x80` \x01\x90Q\x81\x01\x90a\x0E6\x91\x90a)\xD7V[`@\x80Q``\x81\x01\x82R`\x01T\x81R`\x02T` \x82\x01R`\x03T\x91\x81\x01\x91\x90\x91R\x92\x95P\x90\x93P\x91Pa\x06J\x90\x84\x90\x84\x90\x84\x90a\x17\xC2V[`\0\x80a\x0E\x83\x83g\x1B\xC1mgN\xC8\0\0a\x1B\xC5V[\x90Pa\x0B\ng\x06\xF0[Y\xD3\xB2\0\0\x82a\x140V[`\0a\x0B\n\x83\x83`\x01`\0\x01T`\x01\x80\x01T`\x01`\x02\x01Ta\x05gV[`\0\x80`\0\x80`\0\x80a\x0E\xDE`@Q\x80`\x80\x01`@R\x80`G\x81R` \x01a+\x98`G\x919a\x16\xB5V[`\0\x80`\x003`\x01`\x01`\xA0\x1B\x03\x16c\xEB\xAD\xEF\x01`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01```@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0FlW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0F\x80W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xA4\x91\x90a)\xD7V[\x92P\x92P\x92P\x89\x80` \x01\x90Q\x81\x01\x90a\x0F\xBE\x91\x90a)\xD7V[\x91\x97P\x95P\x93P`\x01\x83\x87\x11\x15a\x10\x17W`\0a\x0F\xDB\x85\x89a*\x1BV[\x90P`\0a\x0F\xF4`\0T\x83a\x16\xA0\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa\x10\x04\x86a\x07\xAA\x83\x87a\x16\xA0V[a\x10\x0E\x90\x84a*\x08V[\x92PPPa\x10\xB5V[\x82\x86\x11\x15a\x10TW`\0a\x10+\x84\x88a*\x1BV[\x90P`\0a\x10D`\0T\x83a\x16\xA0\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa\x10\x04\x85a\x07\xAA\x83\x87a\x16\xA0V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`0`$\x82\x01R\x7Finvalid swap: inputs x and y hav`D\x82\x01Roe the same sign!`\x80\x1B`d\x82\x01R`\x84\x01a\x08oV[a\x10\xBF\x82\x86a)<V[`@\x80Q``\x81\x01\x82R`\x01T\x81R`\x02T` \x82\x01R`\x03T\x91\x81\x01\x91\x90\x91R\x90\x98Pa\x10\xF2\x90\x85\x90\x85\x90\x85\x90a\x17\xC2V[`@\x80Q``\x81\x01\x82R`\x01T\x81R`\x02T` \x82\x01R`\x03T\x91\x81\x01\x91\x90\x91Ra\x11\"\x90\x89\x90\x89\x90\x89\x90a\x17\xC2V[a\x11,\x91\x90a)<V[\x98Pa\x11c`@Q\x80`@\x01`@R\x80`\x14\x81R` \x01s\nn\xEC.\x04\x0Cm\xED\xCEn\x8C-\xCE\x84\x0C\xEEM\xEE\xEE\x8D`c\x1B\x81RPa\x16\xB5V[a\x11l\x89a\x1B\xF6V[a\x11\xAA`@Q\x80`@\x01`@R\x80`\x19\x81R` \x01\x7FSubmitted Liquidity delta\0\0\0\0\0\0\0\x81RPa\x16\xB5V[a\x11\xB3\x88a\x1B\xF6V[a\x11\xE7`@Q\x80`@\x01`@R\x80`\x13\x81R` \x01rMin liquidity delta`h\x1B\x81RPa\x16\xB5V[a\x11\xF0\x81a\x1B\xF6V[a\x12\x11`@Q\x80``\x01`@R\x80`%\x81R` \x01a+\xDF`%\x919a\x16\xB5V[a\x12#a\x12\x1E\x82\x8Aa)<V[a\x1B\xF6V[`\0\x89\x12\x15\x80\x15a\x124WP\x80\x88\x12\x15[\x99PPPPP\x91\x93\x95P\x91\x93\x95V[`\0`\n\x81a\x12R\x82\x87a*\x1BV[\x90Pa\x0C\x86\x87\x87\x87\x87`@Q` \x01a\x12n\x94\x93\x92\x91\x90a*.V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83\x83`\x01a\x01\0a\x1C;a\x18\xA8V[`\0a\x12\xA0\x82` \x01Q\x83`@\x01Qa\x13\xFBV[a\x12\xAA\x84\x84a\x0C\xEAV[a\x0B\n\x91\x90a)<V[`\0\x80a\x12\xC1\x84\x84a\x12\x8CV[\x90P`\0a\x12\xCE\x82a\x17YV[\x90P`\0a\x12\xDB\x82a\x0B\x11V[\x85Q\x90\x91Pa\x0B\x04\x90\x82\x90a\x12\xF0\x90\x8Aa\x16\xA0V[\x90a\x16\xA0V[\x82\x82\x02\x81\x15\x80\x15\x90a\x13\x1EWP\x83\x15\x80a\x13\x1EWP\x82\x84\x82\x81a\x13\x1BWa\x13\x1Ba)\x93V[\x05\x14[a\x13\\W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rn\x1B][\x1AY\x1A]\x88\x1A[\x9D\x98[\x1AY`\x8A\x1B`D\x82\x01R`d\x01a\x08oV[\x81\x81\x81a\x13kWa\x13ka)\x93V[\x05\x94\x93PPPPV[`\0a\x13\x81\x84\x84\x84a\x12\xF6V[\x90P\x81a\x13\x8E\x84\x86a)cV[a\x13\x98\x91\x90a*\xA9V[\x15a\x0B\nW`\x01`\x01`\xFF\x1B\x03\x81\x12a\x13\xE8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01RqmulidivUp overflow`p\x1B`D\x82\x01R`d\x01a\x08oV[a\x13\xF3`\x01\x82a*bV[\x94\x93PPPPV[`\0\x80a\x14\x07\x83a\x1ClV[\x90P`\0a\x14\x19c;\x9A\xCA\0\x83a*\xBDV[\x90Pa\x14%\x85\x82a\x140V[\x92PPP[\x92\x91PPV[`\0a\x0B\n\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x1D\x10V[`\0a\x0B\n\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x1D\x10V[`\0g\x06\xF0[Y\xD3\xB2\0\0\x82\x03a\x14sWP`\0\x91\x90PV[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x12a\x14\x9BW`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\x14\xBCW`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x14\xC9\x83`\x02a)cV[\x90P`\0a\x14\xD6\x82a\x1D/V[\x90P`\0a\x14\xECg\x13\xA0K\xBD\xFD\xC9\xBE\x88\x83a\x1F\xADV[\x90Pa\x06J\x81a*\xD4V[`\0h\x02H\xCE6\xA7\x0C\xB2k>\x19\x82\x13a\x15\x12WP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a\x15YW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x01a\x08oV[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05`\x01`_\x1B\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1Dm\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1Dn\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[`\0a\x0B\n\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x1F\xC2V[a\x16\xF8\x81`@Q`$\x01a\x16\xC9\x91\x90a%\xCEV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\x10L\x13\xEB`\xE2\x1B\x17\x90Ra\x1F\xF0V[PV[`\0a\x0B\n\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x1F\xC2V[a\x17U\x82\x82`@Q`$\x01a\x17&\x92\x91\x90a*\xF0V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c-\x83\x9C\xB3`\xE2\x1B\x17\x90Ra\x1F\xF0V[PPV[`\0\x80g\x13\xA0K\xBD\xFD\xC9\xBE\x88a\x17wg\r\xE0\xB6\xB3\xA7d\0\0\x85a)cV[a\x17\x81\x91\x90a)\xA9V[\x90P`\0a\x17\x8E\x82a*\xD4V[\x90P`\0a\x17\x9B\x82a \x11V[\x90Pg\x1B\xC1mgN\xC8\0\0a\x17\xB8g\r\xE0\xB6\xB3\xA7d\0\0\x83a)cV[a\x06J\x91\x90a)\xA9V[`\0\x82\x85\x10a\x18\x13W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FtradingFunction: invalid x\0\0\0\0\0\0`D\x82\x01R`d\x01a\x08oV[`\0a\x18\"a\x05\xE8\x87\x86a\x14EV[\x90P`\0a\x18@a\x05\xE8\x87a\x18;\x87`\0\x01Q\x89a\x140V[a\x14EV[\x90P`\0a\x18V\x85` \x01Q\x86`@\x01Qa\x13\xFBV[\x90P\x80a\x18c\x83\x85a*bV[a\r{\x91\x90a*bV[`\0\x80`\0\x80`\0\x86\x80` \x01\x90Q\x81\x01\x90a\x18\x89\x91\x90a+\x12V[\x93P\x93P\x93P\x93P\x81a\x18\x9E\x85\x88\x86\x85a\x17\xC2V[a\x0C\x86\x91\x90a)<V[`\0\x84\x86\x11\x15a\x18\xD5W`@Qc0\x82\xDF\xDB`\xE1\x1B\x81R`\x04\x81\x01\x87\x90R`$\x81\x01\x86\x90R`D\x01a\x08oV[`\0a\x18\xE5\x88\x88\x85c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x18\xF7\x89\x88\x86c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x19\x05\x82\x84a)cV[\x13\x15a\x19.W`@Qc\x06\xF1\xBE]`\xE2\x1B\x81R`\x04\x81\x01\x83\x90R`$\x81\x01\x82\x90R`D\x01a\x08oV[`\0a\x19:\x89\x89a*\x1BV[\x90P`\0[`\x02a\x19K\x8A\x8Ca*\x08V[a\x19U\x91\x90a+\x83V[\x94P`\0a\x19g\x8C\x87\x89c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x19u\x86\x83a)cV[\x13a\x19\x82W\x85\x99Pa\x19\x89V[\x85\x9AP\x80\x94P[a\x19\x93\x8B\x8Ba*\x1BV[\x92PP`\x01\x01\x87\x82\x11\x80\x15a\x19\xA7WP\x86\x81\x10[a\x19?WPPPP\x96\x95PPPPPPV[`\0\x80`\0\x80`\0\x86\x80` \x01\x90Q\x81\x01\x90a\x19\xD5\x91\x90a+\x12V[\x93P\x93P\x93P\x93P\x81a\x18\x9E\x85\x85\x89\x85a\x17\xC2V[`\0\x80\x82\x13a\x1A'W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x08oV[`\0``a\x1A4\x84a!\xF5V[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[`\0a\x0B\ng\r\xE0\xB6\xB3\xA7d\0\0\x83a\x1B\xDD\x86a\x19\xEAV[a\x1B\xE7\x91\x90a)cV[a\x1B\xF1\x91\x90a)\xA9V[a\x14\xF7V[a\x16\xF8\x81`@Q`$\x01a\x1C\x0C\x91\x81R` \x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c-[l\xB9`\xE0\x1B\x17\x90Ra\x1F\xF0V[`\0\x80`\0\x80`\0\x86\x80` \x01\x90Q\x81\x01\x90a\x1CW\x91\x90a+\x12V[\x93P\x93P\x93P\x93P\x81a\x18\x9E\x87\x86\x86\x85a\x17\xC2V[`\xB5\x81`\x01`\x88\x1B\x81\x10a\x1C\x85W`@\x91\x90\x91\x1B\x90`\x80\x1C[i\x01\0\0\0\0\0\0\0\0\0\x81\x10a\x1C\xA1W` \x91\x90\x91\x1B\x90`@\x1C[e\x01\0\0\0\0\0\x81\x10a\x1C\xB9W`\x10\x91\x90\x91\x1B\x90` \x1C[c\x01\0\0\0\x81\x10a\x1C\xCFW`\x08\x91\x90\x91\x1B\x90`\x10\x1C[b\x01\0\0\x01\x02`\x12\x1C\x80\x82\x04\x01`\x01\x90\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x90\x81\x90\x04\x81\x11\x90\x03\x90V[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x1D(W`\0\x80\xFD[\x04\x92\x91PPV[`\0\x80\x82\x12\x80a\x1DFWPg\x1B\xC1mgN\xC8\0\0\x82\x13[\x15a\x1DdW`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\x1D\x85W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81g\x1B\xC1mgN\xC8\0\0\x03a\x1D\xADW`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x15a\x1D\xB8W\x91\x90PV[`\0g\r\xE0\xB6\xB3\xA7d\0\0\x83\x12a\x1D\xE0Wa\x1D\xDB\x83g\x1B\xC1mgN\xC8\0\0a)<V[a\x1D\xE2V[\x82[\x90P`\0a\x1D\xF8\x82g\x1B\xC1mgN\xC8\0\0a\"\x9DV[\x90P\x80`\0\x03a\x1E\x1BW`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x1E&\x82a\x19\xEAV[\x90P`\0c;\x9A\xCA\0a\x1EQa\x1ELa\x1EFg\x1B\xC1mgN\xC8\0\0a*\xD4V[\x85a\x1F\xADV[a\x1ClV[a\x1E[\x91\x90a)cV[\x90P`\0\x80a\x1Er\x83g\x03\xC1f\\z\xAB \0a\x1F\xADV[a\x1E\x84\x90g \x05\xFEO&\x8E\xA0\0a*bV[\x90P`\0a\x1E\xB4\x84a\x1E\x9D\x86f\x9F2u$b\xA0\0a\x1F\xADV[a\x1E\xAF\x90g\r\xC5R\x7Fd, \0a*bV[a\x1F\xADV[a\x1E\xC6\x90g\r\xE0\xB6\xB3\xA7d\0\0a*bV[\x90Pa\x1E\xEAg\t\xD0(\xCCo _\xFF\x19\x85a\x1E\xE0\x85\x85a\"\x9DV[a\x1E\xAF\x91\x90a)<V[\x92PPP`\0[`\x02\x81\x10\x15a\x1F\x85W`\0\x86a\x1F\x06\x84a \x11V[a\x1F\x10\x91\x90a)<V[\x90P`\0a\x1F\x1E\x84\x85a\x1F\xADV[a\x1F'\x90a*\xD4V[\x90P`\0a\x1F4\x82a\x14\xF7V[\x90P`\0a\x1FB\x86\x85a\x1F\xADV[a\x1FTg\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x84a\x1F\xADV[a\x1F^\x91\x90a)<V[\x90Pa\x1Fj\x84\x82a\"\x9DV[a\x1Ft\x90\x87a*bV[\x95P\x84`\x01\x01\x94PPPPPa\x1E\xF1V[g\r\xE0\xB6\xB3\xA7d\0\0\x88\x12a\x1F\xA2Wa\x1F\x9D\x82a*\xD4V[a\r{V[P\x96\x95PPPPPPV[`\0a\x0B\n\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\"\xAEV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x1F\xDAW`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[\x80Qjconsole.log` \x83\x01`\0\x80\x84\x83\x85Z\xFAPPPPPV[`\0\x81`\0\x03a *WPg\r\xE0\xB6\xB3\xA7d\0\0\x91\x90PV[gV\x98\xEE\xF0fp\0\0\x82\x12a AWP`\0\x91\x90PV[a RgV\x98\xEE\xF0fp\0\0a*\xD4V[\x82\x13a gWPg\x1B\xC1mgN\xC8\0\0\x91\x90PV[`\0a r\x83a\"\xCDV[\x90P`\0a \xABg\r\xE0\xB6\xB3\xA7d\0\0a \x94\x84g\x1B\xC1mgN\xC8\0\0a\x14EV[a \xA6\x90g\r\xE0\xB6\xB3\xA7d\0\0a*bV[a\"\x9DV[\x90P`\0\x80\x82a!\x07\x81a \xF4\x81a \xE2\x81a \xCF\x81g\x02_\x0F\xE1\x05\xA3\x14\0a\x1F\xADV[a\x1E\xAF\x90g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19a*bV[a\x1E\xAF\x90g\x14\xA8EL\x19\xE1\xAC\0a*bV[a\x1E\xAF\x90g\x0F\xC1\x0E\x01W\x82w\xFF\x19a*bV[a!\x19\x90g\x03\xDE\xBD\x08;\x8C|\0a*bV[\x91P\x83\x90Pa!\x81\x81a!o\x81a!]\x81a!K\x81a!8\x81\x8Ba\x1F\xADV[a\x1E\xAF\x90g\x02\x95\xD4\0\xEA2W\xFF\x19a*bV[a\x1E\xAF\x90g\x01W\xD8\xB2\xEC\xC7\x08\0a*bV[a\x1E\xAF\x90g\x051\n\xA7\xD5!0\0a*bV[a\x1E\xAF\x90g\r\xE0\xCC=\x15a\0\0a*bV[\x91P\x81g\x11\x90\0\xAB\x10\x0F\xFC\0a!\x97\x87\x88a\x1F\xADV[a!\xA3\x90`\0\x19a)cV[a!\xAD\x91\x90a)<V[a!\xB7\x91\x90a*bV[\x92PP`\0a!\xC5\x83a\x14\xF7V[\x90P`\0a!\xD3\x85\x83a\x1F\xADV[\x90P`\0\x88\x12a!\xE3W\x80a\r{V[a\r{\x81g\x1B\xC1mgN\xC8\0\0a)<V[`\0\x80\x82\x11a\"2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x08oV[P`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x82\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[`\0a\x0B\n\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x05\x85\x14\x17\x16a\"\xC6W`\0\x80\xFD[\x05\x92\x91PPV[`\0`\x01`\xFF\x1B\x82\x03a\"\xF3W`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x12\x15a\x0BVWP\x19`\x01\x01\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a#\xBFWa#\xBFa#\x04V[PP\x835\x95` \x85\x015\x95P`@\x85\x015\x94``\x81\x015\x94P`\x80\x015\x92P\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a#\xF8Wa#\xF8a#\x04V[\x825\x80\x15\x15\x81\x14a$\x08W`\0\x80\xFD[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a$<W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a$ V[P`\0` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[\x84\x15\x15\x81R\x83` \x82\x01R\x82`@\x82\x01R`\x80``\x82\x01R`\0a$\x83`\x80\x83\x01\x84a$\x16V[\x96\x95PPPPPPV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FABI decoding: struct data too sh`D\x82\x01Rb\x1B\xDC\x9D`\xEA\x1B`d\x82\x01R`\x84\x81\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a%\x17Wa%\x17a$\xDEV[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a%FWa%Fa$\xDEV[`@R\x91\x90PV[`\0``\x82\x84\x03\x12\x15a%cWa%ca$\x8DV[a%ka$\xF4V[\x90P\x815\x81R` \x82\x015` \x82\x01R`@\x82\x015`@\x82\x01R\x92\x91PPV[`\0\x80`\0\x80`\xC0\x85\x87\x03\x12\x15a%\xA4Wa%\xA4a#\x04V[\x845\x93P` \x85\x015\x92P`@\x85\x015\x91Pa%\xC3\x86``\x87\x01a%NV[\x90P\x92\x95\x91\x94P\x92PV[` \x81R`\0a\x0B\n` \x83\x01\x84a$\x16V[`\0\x80`\0`\xA0\x84\x86\x03\x12\x15a%\xF9Wa%\xF9a#\x04V[\x835\x92P` \x84\x015\x91Pa&\x11\x85`@\x86\x01a%NV[\x90P\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a&/Wa&/a#\x04V[P5\x91\x90PV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x81\xFD[`\0\x80` \x83\x85\x03\x12\x15a&\xA5Wa&\xA5a#\x04V[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a&\xC0Wa&\xC0a#TV[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a&\xD7Wa&\xD7a&6V[\x815\x81\x81\x11\x15a':W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x0ENL/$\r\x8C\xAD\xCC\xEE\x8D`\xAB\x1B`d\x82\x01R`\x84\x81\xFD[\x86` \x82\x85\x01\x01\x11\x15a'\xA2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rjrray stride`\xA8\x1B`d\x82\x01R\x91P`\x84\x82\xFD[` \x92\x90\x92\x01\x96\x91\x95P\x90\x93PPPPV[`\0\x80`\x80\x83\x85\x03\x12\x15a'\xCAWa'\xCAa#\x04V[\x825\x91Pa'\xDB\x84` \x85\x01a%NV[\x90P\x92P\x92\x90PV[`\0` \x80\x83\x85\x03\x12\x15a'\xFAWa'\xFAa#\x04V[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a(\x15Wa(\x15a#TV[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a(,Wa(,a&6V[\x815\x81\x81\x11\x15a(>Wa(>a$\xDEV[a(P`\x1F\x82\x01`\x1F\x19\x16\x85\x01a%\x1DV[\x91P\x80\x82R\x86\x84\x82\x85\x01\x01\x11\x15a(\xB6W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x85\x90R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R`\x84\x81\xFD[\x80\x84\x84\x01\x85\x84\x017`\0\x90\x82\x01\x90\x93\x01\x92\x90\x92RP\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a(\xE8Wa(\xE8a#\x04V[PP\x805\x92` \x90\x91\x015\x91PV[`\0\x80`\0``\x84\x86\x03\x12\x15a)\x0FWa)\x0Fa#\x04V[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a)\\Wa)\\a)&V[P\x92\x91PPV[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15a)\x7FWa)\x7Fa)&V[\x81\x81\x05\x83\x14\x82\x15\x17a\x14*Wa\x14*a)&V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a)\xB8Wa)\xB8a)\x93V[`\x01`\xFF\x1B\x82\x14`\0\x19\x84\x14\x16\x15a)\xD2Wa)\xD2a)&V[P\x05\x90V[`\0\x80`\0``\x84\x86\x03\x12\x15a)\xEFWa)\xEFa#\x04V[\x83Q\x92P` \x84\x01Q\x91P`@\x84\x01Q\x90P\x92P\x92P\x92V[\x80\x82\x01\x80\x82\x11\x15a\x14*Wa\x14*a)&V[\x81\x81\x03\x81\x81\x11\x15a\x14*Wa\x14*a)&V[\x84\x81R` \x80\x82\x01\x85\x90R`@\x80\x83\x01\x85\x90R\x83Q``\x84\x01R\x90\x83\x01Q`\x80\x83\x01R\x82\x01Q`\xA0\x82\x01R`\xC0\x81\x01a\x06JV[\x80\x82\x01\x82\x81\x12`\0\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a*\x82Wa*\x82a)&V[PP\x92\x91PPV[`\0`\x01`\x01`\xFF\x1B\x01\x82\x01a*\xA2Wa*\xA2a)&V[P`\x01\x01\x90V[`\0\x82a*\xB8Wa*\xB8a)\x93V[P\x07\x90V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x14*Wa\x14*a)&V[`\0`\x01`\xFF\x1B\x82\x01a*\xE9Wa*\xE9a)&V[P`\0\x03\x90V[`@\x81R`\0a+\x03`@\x83\x01\x85a$\x16V[\x90P\x82` \x83\x01R\x93\x92PPPV[`\0\x80`\0\x80\x84\x86\x03`\xC0\x81\x12\x15a+,Wa+,a#\x04V[\x85Q\x94P` \x86\x01Q\x93P`@\x86\x01Q\x92P```_\x19\x82\x01\x12\x15a+SWa+Sa$\x8DV[Pa+\\a$\xF4V[``\x86\x01Q\x81R`\x80\x86\x01Q` \x82\x01R`\xA0\x90\x95\x01Q`@\x86\x01RP\x91\x94\x90\x93P\x90\x91\x90V[`\0\x82a+\x92Wa+\x92a)\x93V[P\x04\x90V\xFEMake sure the calling contract of this console.log is the Core contractliquidity delta - min liquidity delta";
    /// The bytecode of the contract.
    pub static DFMM_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x01BW`\x005`\xE0\x1C\x80cb}\xD5j\x11a\0\xE4W\x80c\xB7\xD1\x9F\xC4\x11a\0\xB3W\x80c\xB7\xD1\x9F\xC4\x14a\x02\xC8W\x80c\xCF0\x90\x12\x14a\x02\xDBW\x80c\xEB\xAD\xEF\x01\x14a\x02\xE4W\x80c\xF0e\x95\x7F\x14a\x02\xF2Wa\x01BV[\x80cb}\xD5j\x14a\x02xW\x80cg\xE8(\xBF\x14a\x02\x8DW\x80cp\xA0\x821\x14a\x02\xA0W\x80c\x85\x93\x10\xB6\x14a\x02\xC0Wa\x01BV[\x80c\x19c\x824\x11a\x01 W\x80c\x19c\x824\x14a\x02\x15W\x80cC\xC8\x85\xBA\x14a\x02\x1DW\x80cM\xDFG\xD4\x14a\x02AW\x80c^\"}X\x14a\x02oWa\x01BV[\x80c\n\xC304\x14a\x01\xA7W\x80c\x15w\x0F\x92\x14a\x01\xD3W\x80c\x16\xDC\x16[\x14a\x01\xEAW[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x01\xBAa\x01\xB56`\x04a\x12\x19V[a\x02\xFBV[`@Qa\x01\xCA\x94\x93\x92\x91\x90a\x12\x98V[`@Q\x80\x91\x03\x90\xF3[a\x01\xDC`\x06T\x81V[`@Q\x90\x81R` \x01a\x01\xCAV[`\x02Ta\x01\xFD\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\xCAV[a\x01\xDCa\x03\xD7V[`\0Ta\x021\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x01\xCAV[a\x02Ta\x02O6`\x04a\x13\"V[a\x04\xD2V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x01\xCAV[a\x01\xDC`\x05T\x81V[a\x02\x8Ba\x02\x866`\x04a\x13\"V[a\x07\xFCV[\0[`\0Ta\x01\xFD\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01\xDCa\x02\xAE6`\x04a\x14GV[`\x07` R`\0\x90\x81R`@\x90 T\x81V[a\x01\xDCa\n\x06V[`\x03Ta\x01\xFD\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01\xDC`\x01T\x81V[`\x04T`\x05T`\x06Ta\x02TV[a\x01\xDC`\x04T\x81V[`\0\x80T`@Qc\x02\xB0\xCC\r`\xE2\x1B\x81R\x84\x15\x15`\x04\x82\x01R`$\x81\x01\x84\x90R\x82\x91\x82\x91``\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\n\xC304\x90`D\x01`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x03\x8BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x17#\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x03\x9FW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x03\xC7\x91\x90\x81\x01\x90a\x14\x90V[\x92\x99\x91\x98P\x96P\x90\x94P\x92PPPV[`\x04T`\x05T`\x06T`@\x80Q` \x81\x01\x94\x90\x94R\x83\x01\x91\x90\x91R``\x82\x01R`\0\x90\x81\x90`\x80\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R`\0Tc\xA1\x9C\xD3\xD1`\xE0\x1B\x83R\x90\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\xA1\x9C\xD3\xD1\x90a\x04>\x90\x84\x90`\x04\x01a\x15\xC0V[` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x04\x94W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x17#\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x04\xA8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xCC\x91\x90a\x15\xD3V[\x91PP\x90V[`\0\x80`\0`\x01T`\x01\x14a\x05\x17W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x06`$\x82\x01Re\x1B\x1B\xD8\xDA\xD9Y`\xD2\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0`\x01\x81\x90U\x80T`@Qc\x13w\xD1\xF5`\xE2\x1B\x81R\x82\x91\x82\x91\x82\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cM\xDFG\xD4\x90a\x05W\x90\x8D\x90\x8D\x90`\x04\x01a\x15\xEFV[`\xA0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x05\xAFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x17#\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x05\xC3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xE7\x91\x90a\x16\x1EV[\x94P\x94P\x94P\x94P\x94P\x84a\x06%W`\0\x84\x12a\x06\x03\x85a\n\xD2V[`@Qcw`m)`\xE1\x1B\x81R\x91\x15\x15`\x04\x83\x01R`$\x82\x01R`D\x01a\x05\x0EV[`\0\x80T`\xFF`\xA0\x1B\x19\x16`\x01`\xA0\x1B\x17\x81U`\x04\x84\x81U`\x05\x84\x90U`\x06\x83\x90U3\x80\x83R`\x07` R`@\x92\x83\x90 \x84\x90U`\x02T\x92Qc#\xB8r\xDD`\xE0\x1B\x81R\x91\x82\x01R0`$\x82\x01R`D\x81\x01\x85\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90`d\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x06\xE6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x17#\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x06\xFAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x1E\x91\x90a\x16jV[P`\x03T`@Qc#\xB8r\xDD`\xE0\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90`d\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x07\xAFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x17#\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x07\xC3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\xE7\x91\x90a\x16jV[P`\x01\x80U\x91\x99\x90\x98P\x90\x96P\x94PPPPPV[`\x01T`\x01\x14a\x087W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x06`$\x82\x01Re\x1B\x1B\xD8\xDA\xD9Y`\xD2\x1B`D\x82\x01R`d\x01a\x05\x0EV[`\0`\x01\x81\x90UT`\x01`\xA0\x1B\x90\x04`\xFF\x16a\x08\x87W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rn\x1B\x9B\xDD\x08\x1A[\x9A]\x1AX[\x1A^\x99Y`\x8A\x1B`D\x82\x01R`d\x01a\x05\x0EV[`\0\x80T`@Qc\xC1nP\xEF`\xE0\x1B\x81R\x82\x91\x82\x91\x82\x91\x82\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xC1nP\xEF\x90a\x08\xC2\x90\x8B\x90\x8B\x90`\x04\x01a\x15\xEFV[`\xC0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\t\x18W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x17#\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\t,W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\tP\x91\x90a\x16\x8AV[\x95P\x95P\x95P\x95P\x95P\x95P\x85a\tnW`\0\x85\x12a\x06\x03\x86a\n\xD2V[`\x06\x81\x90U`\0\x80\x80\x80a\t\x82\x87\x87a\x0B\rV[`\0T`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x80\x82R` \x82\x01\x86\x90R\x91\x81\x01\x84\x90R``\x81\x01\x8F\x90R\x95\x99P\x93\x97P\x91\x95P\x93P3\x92\x8B\x91\x80\x88\x16\x91\x90\x89\x16\x90\x85\x90\x7F\xD4\xDB\xFA\xA7\xC0\xFEQ\xEE\xE2J\x1C\xDC\xAF\xD07\x8C\xD3e,\xDB\xA75\xDF\xC4\xB0a\xAAU\xC2\xA4X\xA5\x90`\x80\x01`@Q\x80\x91\x03\x90\xA4PP`\x01\x80UPPPPPPPPPPPPPV[`\0\x80T`\x04\x80T`\x06T`@Qc\\x\x05\xB9`\xE1\x1B\x81R\x92\x83\x01\x91\x90\x91R`$\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xB8\xF0\x0Br\x90`D\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\n\x95W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x17#\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\n\xA9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\xCD\x91\x90a\x15\xD3V[\x90P\x90V[`\0`\x01`\xFF\x1B\x82\x03a\n\xF8W`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x12\x15a\x0B\tWP\x19`\x01\x01\x90V[P\x90V[`\x04T`\x05T`\0\x91\x82\x91\x82\x91\x82\x91\x81\x88\x11\x15a\x0B\xA6W`\x02T`\x03T`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x97P\x16\x94Pa\x0BD\x82\x89a\x16\xF6V[\x93P\x86\x81\x11a\x0B\x95W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7Finvalid swap: inputs x and y\0\0\0\0`D\x82\x01R`d\x01a\x05\x0EV[a\x0B\x9F\x87\x82a\x16\xF6V[\x92Pa\x0C$V[`\x03T`\x02T`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x97P\x16\x94Pa\x0B\xC6\x81\x88a\x16\xF6V[\x93P\x87\x82\x11a\x0C\x17W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7Finvalid swap: inputs x and y\0\0\0\0`D\x82\x01R`d\x01a\x05\x0EV[a\x0C!\x88\x83a\x16\xF6V[\x92P[`\x04\x88\x81U`\x05\x88\x90U`@Qcp\xA0\x821`\xE0\x1B\x81R0\x91\x81\x01\x91\x90\x91R`\0\x90`\x01`\x01`\xA0\x1B\x03\x88\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0C\xB0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x17#\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0C\xC4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xE8\x91\x90a\x15\xD3V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x88\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\rkW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x17#\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\r\x7FW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\xA3\x91\x90a\x15\xD3V[`@Qc#\xB8r\xDD`\xE0\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x88\x90R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x89\x16\x90c#\xB8r\xDD\x90`d\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0E2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x17#\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x0EFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Ej\x91\x90a\x16jV[P`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x86\x90R`\x01`\x01`\xA0\x1B\x03\x88\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0E\xF1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x17#\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x0F\x05W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F)\x91\x90a\x16jV[Pa\x0F4\x86\x83a\x17\x0FV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x8A\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0F\xB1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x17#\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0F\xC5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xE9\x91\x90a\x15\xD3V[\x10\x15a\x10BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: input token transf`D\x82\x01Ra2\xB9`\xF1\x1B`d\x82\x01R`\x84\x01a\x05\x0EV[a\x10L\x85\x82a\x16\xF6V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x89\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x10\xC9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a\x17#\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x10\xDDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\x01\x91\x90a\x15\xD3V[\x10\x15a\x11[W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7Finvalid swap: output token trans`D\x82\x01Rb32\xB9`\xE9\x1B`d\x82\x01R`\x84\x01a\x05\x0EV[PPPP\x92\x95\x91\x94P\x92PV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[\x80\x15\x15\x81\x14a\x12\x16W`\0\x80\xFD[PV[`\0\x80`@\x83\x85\x03\x12\x15a\x12/Wa\x12/a\x11hV[\x825a\x12:\x81a\x12\x08V[\x94` \x93\x90\x93\x015\x93PPPV[`\0[\x83\x81\x10\x15a\x12cW\x81\x81\x01Q\x83\x82\x01R` \x01a\x12KV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x12\x84\x81` \x86\x01` \x86\x01a\x12HV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[\x84\x15\x15\x81R\x83` \x82\x01R\x82`@\x82\x01R`\x80``\x82\x01R`\0a\x12\xBF`\x80\x83\x01\x84a\x12lV[\x96\x95PPPPPPV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x81\xFD[`\0\x80` \x83\x85\x03\x12\x15a\x138Wa\x138a\x11hV[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x13SWa\x13Sa\x11\xB8V[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x13jWa\x13ja\x12\xC9V[\x815\x81\x81\x11\x15a\x13\xCDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x0ENL/$\r\x8C\xAD\xCC\xEE\x8D`\xAB\x1B`d\x82\x01R`\x84\x81\xFD[\x86` \x82\x85\x01\x01\x11\x15a\x145W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rjrray stride`\xA8\x1B`d\x82\x01R\x91P`\x84\x82\xFD[` \x92\x90\x92\x01\x96\x91\x95P\x90\x93PPPPV[`\0` \x82\x84\x03\x12\x15a\x14\\Wa\x14\\a\x11hV[\x815`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x14sW`\0\x80\xFD[\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x14\xA9Wa\x14\xA9a\x11hV[\x84Qa\x14\xB4\x81a\x12\x08V[\x80\x94PP` \x85\x01Q\x92P`@\x85\x01Q\x91P``\x85\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x14\xE4Wa\x14\xE4a\x11\xB8V[\x81\x87\x01\x91P\x87`\x1F\x83\x01\x12a\x14\xFBWa\x14\xFBa\x12\xC9V[\x81Q\x81\x81\x11\x15a\x15\rWa\x15\ra\x14zV[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a\x155Wa\x155a\x14zV[\x81`@R\x82\x81R\x8A` \x84\x87\x01\x01\x11\x15a\x15\xA0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R\x93P`\x84\x84\xFD[a\x15\xB1\x83` \x83\x01` \x88\x01a\x12HV[\x97\x9A\x96\x99P\x94\x97PPPPPPV[` \x81R`\0a\x14s` \x83\x01\x84a\x12lV[`\0` \x82\x84\x03\x12\x15a\x15\xE8Wa\x15\xE8a\x11hV[PQ\x91\x90PV[` \x81R\x81` \x82\x01R\x81\x83`@\x83\x017`\0\x81\x83\x01`@\x90\x81\x01\x91\x90\x91R`\x1F\x90\x92\x01`\x1F\x19\x16\x01\x01\x91\x90PV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\x169Wa\x169a\x11hV[\x85Qa\x16D\x81a\x12\x08V[` \x87\x01Q`@\x88\x01Q``\x89\x01Q`\x80\x90\x99\x01Q\x92\x9A\x91\x99P\x97\x96P\x90\x94P\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x16\x7FWa\x16\x7Fa\x11hV[\x81Qa\x14s\x81a\x12\x08V[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a\x16\xA6Wa\x16\xA6a\x11hV[\x86Qa\x16\xB1\x81a\x12\x08V[` \x88\x01Q`@\x89\x01Q``\x8A\x01Q`\x80\x8B\x01Q`\xA0\x90\x9B\x01Q\x93\x9C\x92\x9BP\x90\x99\x90\x98P\x96P\x90\x94P\x92PPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x17\tWa\x17\ta\x16\xE0V[\x92\x91PPV[\x80\x82\x01\x80\x82\x11\x15a\x17\tWa\x17\ta\x16\xE0V\xFETarget contract does not contain";
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
        ///Calls the contract's `internalPrice` (0x859310b6) function
        pub fn internal_price(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([133, 147, 16, 182], ())
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
            (
                bool,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::Bytes,
            ),
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
    ///Container type for all input parameters for the `internalPrice` function with signature `internalPrice()` and selector `0x859310b6`
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
    #[ethcall(name = "internalPrice", abi = "internalPrice()")]
    pub struct InternalPriceCall;
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
        InternalPrice(InternalPriceCall),
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
            if let Ok(decoded) = <InternalPriceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InternalPrice(decoded));
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
                Self::InternalPrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
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
                Self::InternalPrice(element) => ::core::fmt::Display::fmt(element, f),
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
    impl ::core::convert::From<InternalPriceCall> for DFMMCalls {
        fn from(value: InternalPriceCall) -> Self {
            Self::InternalPrice(value)
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
    ///Container type for all return fields from the `internalPrice` function with signature `internalPrice()` and selector `0x859310b6`
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
    pub struct InternalPriceReturn {
        pub price: ::ethers::core::types::U256,
    }
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
        pub estimated_price: ::ethers::core::types::U256,
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
