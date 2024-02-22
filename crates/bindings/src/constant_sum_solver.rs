pub use constant_sum_solver::*;
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
pub mod constant_sum_solver {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("strategy_"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("getInitialPoolData"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getInitialPoolData"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("rx"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("ry"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("params"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct ConstantSum.ConstantSumParams",
                                        ),
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
                (
                    ::std::borrow::ToOwned::to_owned("simulateAllocateOrDeallocate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "simulateAllocateOrDeallocate",
                            ),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("IsAllocate"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountX"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountY"),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                    ::std::borrow::ToOwned::to_owned("simulateSwap"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("simulateSwap"),
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
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
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
                    ::std::borrow::ToOwned::to_owned("strategy"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("strategy"),
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
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("NotEnoughLiquidity"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("NotEnoughLiquidity"),
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
    pub static CONSTANTSUMSOLVER_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80\x80`@R4a\0\xE6W`@Q`\x1Fa\x0F58\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x91`\x01`\x01`@\x1B\x03\x83\x11\x84\x84\x10\x17a\0\xD0W\x80\x84\x92` \x94`@R\x839\x81\x01\x03\x12a\0\x80WQ`\x01`\x01`\xA0\x1B\x03\x81\x16\x90\x81\x90\x03a\0{W`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x91\x90\x91\x17\x90U`@Qa\x0E\x01\x90\x81a\x014\x829\xF3[`\0\x80\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[bF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01Ra7\xB7`\xF1\x1B`d\x82\x01R`\x84\x90\xFD\xFE`@`\x80\x81R`\x04\x806\x10a\t5W`\0\x91\x825`\xE0\x1C\x92\x83c9(\xFF\x97\x14a\0[WPPP\x80c\x89\xEA\x85Y\x14a\0VW\x80c\x8A\x1A \xDE\x14a\0QWc\xA8\xC6.v\x14a\0LW8\x80a\t5V[a\t\x0CV[a\x06&V[a\x05SV[4a\x03\xC0W``6`\x03\x19\x01\x12a\x03\xBBW`$5\x835a\0z\x82a\x04`V[`D5\x90a\0\x86a\t\x98V[\x95a\0\x8Fa\t\x98V[\x85T\x90\x94\x90a\0\xB4\x90a\0\xA8\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x93\x84;\x15a\x02\xB2W\x87Qc+\xEE\x84\xF1`\xE2\x1B\x81R` \x92\x90\x83\x81\x86\x81\x8AZ\xFA\x90\x81\x15a\x02\xADW\x89\x91a\x03\x8EW[P`\x01`\x01`\xA0\x1B\x03\x16\x91\x82;\x15a\x02\xB2W``\x8AQ\x80\x94c3\x85N\xFD`\xE2\x1B\x82R\x81\x80a\x01\x16\x8B\x8B\x83\x01\x91\x90` \x83\x01\x92RV[\x03\x91Z\xFA\x90\x81\x15a\x02\xADW\x8B\x93\x8A\x90\x8B\x90\x8C\x94a\x03XW[P\x8C\x87\x87\x01\x96\x01\x93\x84R\x85R\x8CR\x87;\x15a\x02\xB2W\x8AQc\xDC\x17\x83U`\xE0\x1B\x81R\x86\x81\x01\x88\x81R\x8B\x90\x82\x90\x81\x90` \x01\x03\x81\x8CZ\xFA\x80\x15a\x02\xADWa\x01\x83\x91\x8C\x91a\x036W[P\x86\x80\x82Q\x83\x01\x01\x91\x01a\x0B\xAEV[\x92\x15a\x02\xC7W\x90a\x01\xC4\x83\x92a\x01\xBD\x87a\x01\xCC\x96\x01Qa\x01\xB7a\x01\xB1a\x01\xA9\x83\x86a\r\\V[\x97Q\x85a\r\x88V[\x91a\x0B\xFEV[\x90a\r\x88V[\x9DQa\x0C&V[\x89RQa\x0C&V[\x88\x87\x01R\x88\x81Q\x10a\x02\xB7W\x90a\x01\xE7\x89a\x02$\x93Qa\x0C\x19V[\x81\x87\x01R[a\x02\x16\x88Q\x96\x87\x92\x83\x01\x91\x90\x91`@\x80``\x83\x01\x94\x80Q\x84R` \x81\x01Q` \x85\x01R\x01Q\x91\x01RV[\x03`\x1F\x19\x81\x01\x86R\x85a\x05\x0FV[\x82;\x15a\x02\xB2W\x85Qc\r\x17\xA7\xC7`\xE3\x1B\x81R\x92`\xC0\x92\x84\x92\x83\x91\x82\x91a\x02P\x91\x89\x910\x90\x85\x01a\x0CjV[\x03\x91Z\xFA\x92\x83\x15a\x02\xADW\x92a\x02uW[Pa\x02q\x91\x92Q\x93\x84\x93\x84a\x04\xB7V[\x03\x90\xF3[a\x02q\x92Pa\x02\x9B\x90`\xC0=`\xC0\x11a\x02\xA6W[a\x02\x93\x81\x83a\x05\x0FV[\x81\x01\x90a\x0C3V[PPPPP\x91a\x02aV[P=a\x02\x89V[a\n\x1FV[a\t\xB7V[\x87QcC#\xA5U`\xE0\x1B\x81R\x83\x90\xFD[\x90a\x03\n\x83\x92a\x01\xBD\x87\x96\x9E\x96a\x03\x14\x96\x01Q\x94a\x03\x02\x83a\x02\xFDa\x02\xF7a\x02\xEF\x8A\x84a\r\\V[\x85Q\x90a\r,V[\x98a\x0B\xFEV[a\r\x88V[\x90Q\x90a\r\xA9V[\x85\x8A\x01RQa\x0C&V[\x88\x87\x01R\x88\x81Q\x10a\x02\xB7W\x90a\x03/\x89a\x02$\x93Qa\x0C\x19V[\x86Ra\x01\xECV[a\x03R\x91P=\x80\x8E\x83>a\x03J\x81\x83a\x05\x0FV[\x81\x01\x90a\nFV[\x8Ea\x01tV[\x91PPa\x03~\x91\x92P``=``\x11a\x03\x87W[a\x03v\x81\x83a\x05\x0FV[\x81\x01\x90a\n+V[\x92\x91\x90\x8Ea\x01.V[P=a\x03lV[a\x03\xAE\x91P\x84=\x86\x11a\x03\xB4W[a\x03\xA6\x81\x83a\x05\x0FV[\x81\x01\x90a\n\nV[\x8Ba\0\xE1V[P=a\x03\x9CV[a\x04\x10V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01Ra7\xB7`\xF1\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[\x80\x15\x15\x03a\x04jWV[`\0\x80\xFD[`\0[\x83\x81\x10a\x04\x82WPP`\0\x91\x01RV[\x81\x81\x01Q\x83\x82\x01R` \x01a\x04rV[\x90` \x91a\x04\xAB\x81Q\x80\x92\x81\x85R\x85\x80\x86\x01\x91\x01a\x04oV[`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[a\x04\xD5\x93\x92``\x92\x15\x15\x82R` \x82\x01R\x81`@\x82\x01R\x01\x90a\x04\x92V[\x90V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[``\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x05\nW`@RV[a\x04\xD8V[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x05\nW`@RV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\x04jWV[\x90` a\x04\xD5\x92\x81\x81R\x01\x90a\x04\x92V[4a\x06\x08W`\xA06`\x03\x19\x01\x12a\x03\xBBW``6`C\x19\x01\x12a\x05\xB7Wa\x02qa\x05\xAB`@Qa\x05\x82\x81a\x04\xEEV[`D5\x81R`d5` \x82\x01R`\x845a\x05\x9B\x81a\x051V[`@\x82\x01R`$5`\x045a\x0C\x8EV[`@Q\x91\x82\x91\x82a\x05BV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FABI decoding: struct data too sh`D\x82\x01Rb\x1B\xDC\x9D`\xEA\x1B`d\x82\x01R`\x84\x90\xFD[a\x03\xC0V[`@\x90a\x04\xD5\x93\x92\x15\x15\x81R\x81` \x82\x01R\x01\x90a\x04\x92V[4a\x06\x08W`\x806`\x03\x19\x01\x12a\x03\xBBW`\x04`$5\x815a\x06G\x82a\x04`V[`D5\x90`d5\x90a\x06Wa\t\x98V[\x94a\x06`a\t\x98V[\x95`\0\x95a\x06za\0\xA8a\0\xA8\x89T`\x01\x80`\xA0\x1B\x03\x16\x90V[\x94\x85;\x15a\x02\xB2W`@\x96\x87Q\x93c+\xEE\x84\xF1`\xE2\x1B\x85R` \x94\x85\x81\x88\x81\x8CZ\xFA\x90\x81\x15a\x02\xADW\x8B\x91a\x08\xEFW[P`\x01`\x01`\xA0\x1B\x03\x16\x91\x82;\x15a\x02\xB2W``\x8AQ\x80\x94c3\x85N\xFD`\xE2\x1B\x82R\x81\x80a\x06\xDF\x8D\x8D\x83\x01\x91\x90` \x83\x01\x92RV[\x03\x91Z\xFA\x92\x83\x15a\x02\xADW\x8B\x90\x8C\x90\x8D\x95a\x08\xC9W[P\x87\x84\x01\x94\x8C\x85\x01R\x84R\x82R\x88;\x15a\x02\xB2W\x89Q\x91c\xDC\x17\x83U`\xE0\x1B\x83R\x8B\x83\x80a\x07*\x8C\x8C\x83\x01\x91\x90` \x83\x01\x92RV[\x03\x81\x8DZ\xFA\x92\x83\x15a\x02\xADW\x8D\x93a\x07T\x91\x8E\x80\x92a\x08\xAEW[PP\x88\x80\x82Q\x83\x01\x01\x91\x01a\x0B\xAEV[\x95\x15a\x08UW\x93a\x07\xD0\x96\x95\x93a\x07\x8D\x93a\x07va\x07~\x94a\x07\x93\x98Qa\x0C&V[\x90RQa\x0C&V[\x80\x85\x8D\x01R\x8BQ\x92Q\x90a\r,V[\x90a\x0C&V[\x86\x89\x01R[a\x07\xC2\x86Q\x98\x89\x92\x83\x01\x91\x90\x91`@\x80``\x83\x01\x94\x80Q\x84R` \x81\x01Q` \x85\x01R\x01Q\x91\x01RV[\x03`\x1F\x19\x81\x01\x88R\x87a\x05\x0FV[\x82;\x15a\x02\xB2W\x83Qc\x8A\x04\xBD\xD5`\xE0\x1B\x81R\x92`\xA0\x92\x84\x92\x83\x91\x82\x91a\x07\xFC\x91\x8B\x910\x90\x85\x01a\x0CjV[\x03\x91Z\xFA\x92\x83\x15a\x02\xADW\x92a\x08\x1CW[Pa\x02q\x90Q\x92\x83\x92\x83a\x06\rV[a\x02q\x91\x92Pa\x08C\x90`\xA0=`\xA0\x11a\x08NW[a\x08;\x81\x83a\x05\x0FV[\x81\x01\x90a\x0C\xFCV[PPPP\x91\x90a\x08\rV[P=a\x081V[\x90\x80\x92P\x81Q\x10\x80\x15a\x08\xA4W[a\x08\x94Wa\x08\x8B\x93a\x07\xD0\x96\x95\x93\x8Da\x08\x83a\x07\x8D\x95a\x07~\x95Qa\x0C\x19V[\x90RQa\x0C\x19V[\x86\x89\x01Ra\x07\x98V[\x89QcC#\xA5U`\xE0\x1B\x81R\x87\x90\xFD[P\x83\x83Q\x10a\x08cV[a\x08\xC2\x92P=\x80\x91\x83>a\x03J\x81\x83a\x05\x0FV[8\x8Ea\x07DV[\x91PPa\x08\xE6\x91\x93P``=``\x11a\x03\x87Wa\x03v\x81\x83a\x05\x0FV[\x93\x91\x908a\x06\xF5V[a\t\x06\x91P\x86=\x88\x11a\x03\xB4Wa\x03\xA6\x81\x83a\x05\x0FV[8a\x06\xAAV[4a\x06\x08W`\x006`\x03\x19\x01\x12a\x03\xBBW`\0T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01Rtnor receive functions`X\x1B`d\x82\x01R`\x84\x90\xFD[`@Q\x90a\t\xA5\x82a\x04\xEEV[`\0`@\x83\x82\x81R\x82` \x82\x01R\x01RV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01Rd code`\xD8\x1B`d\x82\x01R`\x84\x90\xFD[\x90\x81` \x91\x03\x12a\x03\xBBWQa\x04\xD5\x81a\x051V[`@Q=`\0\x82>=\x90\xFD[\x90\x81``\x91\x03\x12a\x03\xBBW\x80Q\x91`@` \x83\x01Q\x92\x01Q\x90V[\x91\x90` \x92\x83\x81\x83\x03\x12a\x03\xBBW\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x82\x82\x11a\x0B^W\x01\x82`\x1F\x82\x01\x12\x15a\x0B\x05W\x80Q\x91\x82\x11a\x05\nW`@Q\x92a\n\x93`\x1F\x84\x01`\x1F\x19\x16\x87\x01\x85a\x05\x0FV[\x82\x84R\x85\x83\x83\x01\x01\x11a\n\xB0W\x84\x83\x94\x95a\x04\xD5\x94\x01\x91\x01a\x04oV[`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x86\x90R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x86\x90R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x87\x90R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[\x90\x81``\x91\x03\x12a\x03\xBBW`@\x80Q\x91a\x0B\xC7\x83a\x04\xEEV[\x80Q\x83R` \x81\x01Q` \x84\x01R\x01Qa\x0B\xE0\x81a\x051V[`@\x82\x01R\x90V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x90g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x03\x91\x82\x11a\x0C\x14WV[a\x0B\xE8V[\x91\x90\x82\x03\x91\x82\x11a\x0C\x14WV[\x91\x90\x82\x01\x80\x92\x11a\x0C\x14WV[\x91\x90\x82`\xC0\x91\x03\x12a\x03\xBBW\x81Qa\x0CJ\x81a\x04`V[\x91` \x81\x01Q\x91`@\x82\x01Q\x91``\x81\x01Q\x91`\xA0`\x80\x83\x01Q\x92\x01Q\x90V[a\x04\xD5\x93\x92``\x92`\x01\x80`\xA0\x1B\x03\x16\x82R` \x82\x01R\x81`@\x82\x01R\x01\x90a\x04\x92V[\x91\x90a\x0C\x9B\x82Q\x82a\r,V[\x83\x01\x90\x81\x84\x11a\x0C\x14W`@Q\x93` \x85\x01R`@\x84\x01R``\x83\x01R\x80Q`\x80\x83\x01R` \x81\x01Q`\xA0\x83\x01R`@`\x01\x80`\xA0\x1B\x03\x91\x01Q\x16`\xC0\x82\x01R`\xC0\x81R`\xE0\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x05\nW`@R\x90V[\x90\x81`\xA0\x91\x03\x12a\x03\xBBW\x80Qa\r\x12\x81a\x04`V[\x91` \x82\x01Q\x91`@\x81\x01Q\x91`\x80``\x83\x01Q\x92\x01Q\x90V[\x90g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x81\x02\x92\x81\x84\x04\x14\x90\x15\x17\x81\x15\x15\x16\x15a\x04jW`\x01\x90`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x04jW`\x01g\r\xE0\xB6\xB3\xA7d\0\0`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x04jWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x04\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17\x82\x15\x15\x16\x15a\x04jW\x04\x90V\xFE\xA2dipfsX\"\x12 \xB5\x01\xD2\xB1\x92\xD5\x1F\x89\xE1\xB9\x80\xD4\xA1 J\xC3\x19\xB6\x94\xED\x8F\xE6\xA8\x8B\xD6'\x1CpQ\x11\x10#dsolcC\0\x08\x16\x003";
    /// The bytecode of the contract.
    pub static CONSTANTSUMSOLVER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`@`\x80\x81R`\x04\x806\x10a\t5W`\0\x91\x825`\xE0\x1C\x92\x83c9(\xFF\x97\x14a\0[WPPP\x80c\x89\xEA\x85Y\x14a\0VW\x80c\x8A\x1A \xDE\x14a\0QWc\xA8\xC6.v\x14a\0LW8\x80a\t5V[a\t\x0CV[a\x06&V[a\x05SV[4a\x03\xC0W``6`\x03\x19\x01\x12a\x03\xBBW`$5\x835a\0z\x82a\x04`V[`D5\x90a\0\x86a\t\x98V[\x95a\0\x8Fa\t\x98V[\x85T\x90\x94\x90a\0\xB4\x90a\0\xA8\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x93\x84;\x15a\x02\xB2W\x87Qc+\xEE\x84\xF1`\xE2\x1B\x81R` \x92\x90\x83\x81\x86\x81\x8AZ\xFA\x90\x81\x15a\x02\xADW\x89\x91a\x03\x8EW[P`\x01`\x01`\xA0\x1B\x03\x16\x91\x82;\x15a\x02\xB2W``\x8AQ\x80\x94c3\x85N\xFD`\xE2\x1B\x82R\x81\x80a\x01\x16\x8B\x8B\x83\x01\x91\x90` \x83\x01\x92RV[\x03\x91Z\xFA\x90\x81\x15a\x02\xADW\x8B\x93\x8A\x90\x8B\x90\x8C\x94a\x03XW[P\x8C\x87\x87\x01\x96\x01\x93\x84R\x85R\x8CR\x87;\x15a\x02\xB2W\x8AQc\xDC\x17\x83U`\xE0\x1B\x81R\x86\x81\x01\x88\x81R\x8B\x90\x82\x90\x81\x90` \x01\x03\x81\x8CZ\xFA\x80\x15a\x02\xADWa\x01\x83\x91\x8C\x91a\x036W[P\x86\x80\x82Q\x83\x01\x01\x91\x01a\x0B\xAEV[\x92\x15a\x02\xC7W\x90a\x01\xC4\x83\x92a\x01\xBD\x87a\x01\xCC\x96\x01Qa\x01\xB7a\x01\xB1a\x01\xA9\x83\x86a\r\\V[\x97Q\x85a\r\x88V[\x91a\x0B\xFEV[\x90a\r\x88V[\x9DQa\x0C&V[\x89RQa\x0C&V[\x88\x87\x01R\x88\x81Q\x10a\x02\xB7W\x90a\x01\xE7\x89a\x02$\x93Qa\x0C\x19V[\x81\x87\x01R[a\x02\x16\x88Q\x96\x87\x92\x83\x01\x91\x90\x91`@\x80``\x83\x01\x94\x80Q\x84R` \x81\x01Q` \x85\x01R\x01Q\x91\x01RV[\x03`\x1F\x19\x81\x01\x86R\x85a\x05\x0FV[\x82;\x15a\x02\xB2W\x85Qc\r\x17\xA7\xC7`\xE3\x1B\x81R\x92`\xC0\x92\x84\x92\x83\x91\x82\x91a\x02P\x91\x89\x910\x90\x85\x01a\x0CjV[\x03\x91Z\xFA\x92\x83\x15a\x02\xADW\x92a\x02uW[Pa\x02q\x91\x92Q\x93\x84\x93\x84a\x04\xB7V[\x03\x90\xF3[a\x02q\x92Pa\x02\x9B\x90`\xC0=`\xC0\x11a\x02\xA6W[a\x02\x93\x81\x83a\x05\x0FV[\x81\x01\x90a\x0C3V[PPPPP\x91a\x02aV[P=a\x02\x89V[a\n\x1FV[a\t\xB7V[\x87QcC#\xA5U`\xE0\x1B\x81R\x83\x90\xFD[\x90a\x03\n\x83\x92a\x01\xBD\x87\x96\x9E\x96a\x03\x14\x96\x01Q\x94a\x03\x02\x83a\x02\xFDa\x02\xF7a\x02\xEF\x8A\x84a\r\\V[\x85Q\x90a\r,V[\x98a\x0B\xFEV[a\r\x88V[\x90Q\x90a\r\xA9V[\x85\x8A\x01RQa\x0C&V[\x88\x87\x01R\x88\x81Q\x10a\x02\xB7W\x90a\x03/\x89a\x02$\x93Qa\x0C\x19V[\x86Ra\x01\xECV[a\x03R\x91P=\x80\x8E\x83>a\x03J\x81\x83a\x05\x0FV[\x81\x01\x90a\nFV[\x8Ea\x01tV[\x91PPa\x03~\x91\x92P``=``\x11a\x03\x87W[a\x03v\x81\x83a\x05\x0FV[\x81\x01\x90a\n+V[\x92\x91\x90\x8Ea\x01.V[P=a\x03lV[a\x03\xAE\x91P\x84=\x86\x11a\x03\xB4W[a\x03\xA6\x81\x83a\x05\x0FV[\x81\x01\x90a\n\nV[\x8Ba\0\xE1V[P=a\x03\x9CV[a\x04\x10V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01Ra7\xB7`\xF1\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[\x80\x15\x15\x03a\x04jWV[`\0\x80\xFD[`\0[\x83\x81\x10a\x04\x82WPP`\0\x91\x01RV[\x81\x81\x01Q\x83\x82\x01R` \x01a\x04rV[\x90` \x91a\x04\xAB\x81Q\x80\x92\x81\x85R\x85\x80\x86\x01\x91\x01a\x04oV[`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[a\x04\xD5\x93\x92``\x92\x15\x15\x82R` \x82\x01R\x81`@\x82\x01R\x01\x90a\x04\x92V[\x90V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[``\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x05\nW`@RV[a\x04\xD8V[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x05\nW`@RV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\x04jWV[\x90` a\x04\xD5\x92\x81\x81R\x01\x90a\x04\x92V[4a\x06\x08W`\xA06`\x03\x19\x01\x12a\x03\xBBW``6`C\x19\x01\x12a\x05\xB7Wa\x02qa\x05\xAB`@Qa\x05\x82\x81a\x04\xEEV[`D5\x81R`d5` \x82\x01R`\x845a\x05\x9B\x81a\x051V[`@\x82\x01R`$5`\x045a\x0C\x8EV[`@Q\x91\x82\x91\x82a\x05BV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FABI decoding: struct data too sh`D\x82\x01Rb\x1B\xDC\x9D`\xEA\x1B`d\x82\x01R`\x84\x90\xFD[a\x03\xC0V[`@\x90a\x04\xD5\x93\x92\x15\x15\x81R\x81` \x82\x01R\x01\x90a\x04\x92V[4a\x06\x08W`\x806`\x03\x19\x01\x12a\x03\xBBW`\x04`$5\x815a\x06G\x82a\x04`V[`D5\x90`d5\x90a\x06Wa\t\x98V[\x94a\x06`a\t\x98V[\x95`\0\x95a\x06za\0\xA8a\0\xA8\x89T`\x01\x80`\xA0\x1B\x03\x16\x90V[\x94\x85;\x15a\x02\xB2W`@\x96\x87Q\x93c+\xEE\x84\xF1`\xE2\x1B\x85R` \x94\x85\x81\x88\x81\x8CZ\xFA\x90\x81\x15a\x02\xADW\x8B\x91a\x08\xEFW[P`\x01`\x01`\xA0\x1B\x03\x16\x91\x82;\x15a\x02\xB2W``\x8AQ\x80\x94c3\x85N\xFD`\xE2\x1B\x82R\x81\x80a\x06\xDF\x8D\x8D\x83\x01\x91\x90` \x83\x01\x92RV[\x03\x91Z\xFA\x92\x83\x15a\x02\xADW\x8B\x90\x8C\x90\x8D\x95a\x08\xC9W[P\x87\x84\x01\x94\x8C\x85\x01R\x84R\x82R\x88;\x15a\x02\xB2W\x89Q\x91c\xDC\x17\x83U`\xE0\x1B\x83R\x8B\x83\x80a\x07*\x8C\x8C\x83\x01\x91\x90` \x83\x01\x92RV[\x03\x81\x8DZ\xFA\x92\x83\x15a\x02\xADW\x8D\x93a\x07T\x91\x8E\x80\x92a\x08\xAEW[PP\x88\x80\x82Q\x83\x01\x01\x91\x01a\x0B\xAEV[\x95\x15a\x08UW\x93a\x07\xD0\x96\x95\x93a\x07\x8D\x93a\x07va\x07~\x94a\x07\x93\x98Qa\x0C&V[\x90RQa\x0C&V[\x80\x85\x8D\x01R\x8BQ\x92Q\x90a\r,V[\x90a\x0C&V[\x86\x89\x01R[a\x07\xC2\x86Q\x98\x89\x92\x83\x01\x91\x90\x91`@\x80``\x83\x01\x94\x80Q\x84R` \x81\x01Q` \x85\x01R\x01Q\x91\x01RV[\x03`\x1F\x19\x81\x01\x88R\x87a\x05\x0FV[\x82;\x15a\x02\xB2W\x83Qc\x8A\x04\xBD\xD5`\xE0\x1B\x81R\x92`\xA0\x92\x84\x92\x83\x91\x82\x91a\x07\xFC\x91\x8B\x910\x90\x85\x01a\x0CjV[\x03\x91Z\xFA\x92\x83\x15a\x02\xADW\x92a\x08\x1CW[Pa\x02q\x90Q\x92\x83\x92\x83a\x06\rV[a\x02q\x91\x92Pa\x08C\x90`\xA0=`\xA0\x11a\x08NW[a\x08;\x81\x83a\x05\x0FV[\x81\x01\x90a\x0C\xFCV[PPPP\x91\x90a\x08\rV[P=a\x081V[\x90\x80\x92P\x81Q\x10\x80\x15a\x08\xA4W[a\x08\x94Wa\x08\x8B\x93a\x07\xD0\x96\x95\x93\x8Da\x08\x83a\x07\x8D\x95a\x07~\x95Qa\x0C\x19V[\x90RQa\x0C\x19V[\x86\x89\x01Ra\x07\x98V[\x89QcC#\xA5U`\xE0\x1B\x81R\x87\x90\xFD[P\x83\x83Q\x10a\x08cV[a\x08\xC2\x92P=\x80\x91\x83>a\x03J\x81\x83a\x05\x0FV[8\x8Ea\x07DV[\x91PPa\x08\xE6\x91\x93P``=``\x11a\x03\x87Wa\x03v\x81\x83a\x05\x0FV[\x93\x91\x908a\x06\xF5V[a\t\x06\x91P\x86=\x88\x11a\x03\xB4Wa\x03\xA6\x81\x83a\x05\x0FV[8a\x06\xAAV[4a\x06\x08W`\x006`\x03\x19\x01\x12a\x03\xBBW`\0T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01Rtnor receive functions`X\x1B`d\x82\x01R`\x84\x90\xFD[`@Q\x90a\t\xA5\x82a\x04\xEEV[`\0`@\x83\x82\x81R\x82` \x82\x01R\x01RV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01Rd code`\xD8\x1B`d\x82\x01R`\x84\x90\xFD[\x90\x81` \x91\x03\x12a\x03\xBBWQa\x04\xD5\x81a\x051V[`@Q=`\0\x82>=\x90\xFD[\x90\x81``\x91\x03\x12a\x03\xBBW\x80Q\x91`@` \x83\x01Q\x92\x01Q\x90V[\x91\x90` \x92\x83\x81\x83\x03\x12a\x03\xBBW\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x82\x82\x11a\x0B^W\x01\x82`\x1F\x82\x01\x12\x15a\x0B\x05W\x80Q\x91\x82\x11a\x05\nW`@Q\x92a\n\x93`\x1F\x84\x01`\x1F\x19\x16\x87\x01\x85a\x05\x0FV[\x82\x84R\x85\x83\x83\x01\x01\x11a\n\xB0W\x84\x83\x94\x95a\x04\xD5\x94\x01\x91\x01a\x04oV[`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x86\x90R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x86\x90R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x87\x90R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[\x90\x81``\x91\x03\x12a\x03\xBBW`@\x80Q\x91a\x0B\xC7\x83a\x04\xEEV[\x80Q\x83R` \x81\x01Q` \x84\x01R\x01Qa\x0B\xE0\x81a\x051V[`@\x82\x01R\x90V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x90g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x03\x91\x82\x11a\x0C\x14WV[a\x0B\xE8V[\x91\x90\x82\x03\x91\x82\x11a\x0C\x14WV[\x91\x90\x82\x01\x80\x92\x11a\x0C\x14WV[\x91\x90\x82`\xC0\x91\x03\x12a\x03\xBBW\x81Qa\x0CJ\x81a\x04`V[\x91` \x81\x01Q\x91`@\x82\x01Q\x91``\x81\x01Q\x91`\xA0`\x80\x83\x01Q\x92\x01Q\x90V[a\x04\xD5\x93\x92``\x92`\x01\x80`\xA0\x1B\x03\x16\x82R` \x82\x01R\x81`@\x82\x01R\x01\x90a\x04\x92V[\x91\x90a\x0C\x9B\x82Q\x82a\r,V[\x83\x01\x90\x81\x84\x11a\x0C\x14W`@Q\x93` \x85\x01R`@\x84\x01R``\x83\x01R\x80Q`\x80\x83\x01R` \x81\x01Q`\xA0\x83\x01R`@`\x01\x80`\xA0\x1B\x03\x91\x01Q\x16`\xC0\x82\x01R`\xC0\x81R`\xE0\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x05\nW`@R\x90V[\x90\x81`\xA0\x91\x03\x12a\x03\xBBW\x80Qa\r\x12\x81a\x04`V[\x91` \x82\x01Q\x91`@\x81\x01Q\x91`\x80``\x83\x01Q\x92\x01Q\x90V[\x90g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x81\x02\x92\x81\x84\x04\x14\x90\x15\x17\x81\x15\x15\x16\x15a\x04jW`\x01\x90`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x04jW`\x01g\r\xE0\xB6\xB3\xA7d\0\0`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x04jWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x04\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17\x82\x15\x15\x16\x15a\x04jW\x04\x90V\xFE\xA2dipfsX\"\x12 \xB5\x01\xD2\xB1\x92\xD5\x1F\x89\xE1\xB9\x80\xD4\xA1 J\xC3\x19\xB6\x94\xED\x8F\xE6\xA8\x8B\xD6'\x1CpQ\x11\x10#dsolcC\0\x08\x16\x003";
    /// The deployed bytecode of the contract.
    pub static CONSTANTSUMSOLVER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct ConstantSumSolver<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ConstantSumSolver<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ConstantSumSolver<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ConstantSumSolver<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ConstantSumSolver<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(ConstantSumSolver))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ConstantSumSolver<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    CONSTANTSUMSOLVER_ABI.clone(),
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
                CONSTANTSUMSOLVER_ABI.clone(),
                CONSTANTSUMSOLVER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `getInitialPoolData` (0x89ea8559) function
        pub fn get_initial_pool_data(
            &self,
            rx: ::ethers::core::types::U256,
            ry: ::ethers::core::types::U256,
            params: ConstantSumParams,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([137, 234, 133, 89], (rx, ry, params))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `simulateAllocateOrDeallocate` (0x8a1a20de) function
        pub fn simulate_allocate_or_deallocate(
            &self,
            pool_id: ::ethers::core::types::U256,
            is_allocate: bool,
            amount_x: ::ethers::core::types::U256,
            amount_y: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (bool, ::ethers::core::types::Bytes),
        > {
            self.0
                .method_hash(
                    [138, 26, 32, 222],
                    (pool_id, is_allocate, amount_x, amount_y),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `simulateSwap` (0x3928ff97) function
        pub fn simulate_swap(
            &self,
            pool_id: ::ethers::core::types::U256,
            swap_x_in: bool,
            amount_in: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (bool, ::ethers::core::types::U256, ::ethers::core::types::Bytes),
        > {
            self.0
                .method_hash([57, 40, 255, 151], (pool_id, swap_x_in, amount_in))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `strategy` (0xa8c62e76) function
        pub fn strategy(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([168, 198, 46, 118], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for ConstantSumSolver<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `NotEnoughLiquidity` with signature `NotEnoughLiquidity()` and selector `0x4323a555`
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
    #[etherror(name = "NotEnoughLiquidity", abi = "NotEnoughLiquidity()")]
    pub struct NotEnoughLiquidity;
    ///Container type for all input parameters for the `getInitialPoolData` function with signature `getInitialPoolData(uint256,uint256,(uint256,uint256,address))` and selector `0x89ea8559`
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
        name = "getInitialPoolData",
        abi = "getInitialPoolData(uint256,uint256,(uint256,uint256,address))"
    )]
    pub struct GetInitialPoolDataCall {
        pub rx: ::ethers::core::types::U256,
        pub ry: ::ethers::core::types::U256,
        pub params: ConstantSumParams,
    }
    ///Container type for all input parameters for the `simulateAllocateOrDeallocate` function with signature `simulateAllocateOrDeallocate(uint256,bool,uint256,uint256)` and selector `0x8a1a20de`
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
        name = "simulateAllocateOrDeallocate",
        abi = "simulateAllocateOrDeallocate(uint256,bool,uint256,uint256)"
    )]
    pub struct SimulateAllocateOrDeallocateCall {
        pub pool_id: ::ethers::core::types::U256,
        pub is_allocate: bool,
        pub amount_x: ::ethers::core::types::U256,
        pub amount_y: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `simulateSwap` function with signature `simulateSwap(uint256,bool,uint256)` and selector `0x3928ff97`
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
    #[ethcall(name = "simulateSwap", abi = "simulateSwap(uint256,bool,uint256)")]
    pub struct SimulateSwapCall {
        pub pool_id: ::ethers::core::types::U256,
        pub swap_x_in: bool,
        pub amount_in: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `strategy` function with signature `strategy()` and selector `0xa8c62e76`
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
    #[ethcall(name = "strategy", abi = "strategy()")]
    pub struct StrategyCall;
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
    pub enum ConstantSumSolverCalls {
        GetInitialPoolData(GetInitialPoolDataCall),
        SimulateAllocateOrDeallocate(SimulateAllocateOrDeallocateCall),
        SimulateSwap(SimulateSwapCall),
        Strategy(StrategyCall),
    }
    impl ::ethers::core::abi::AbiDecode for ConstantSumSolverCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <GetInitialPoolDataCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetInitialPoolData(decoded));
            }
            if let Ok(decoded) = <SimulateAllocateOrDeallocateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SimulateAllocateOrDeallocate(decoded));
            }
            if let Ok(decoded) = <SimulateSwapCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SimulateSwap(decoded));
            }
            if let Ok(decoded) = <StrategyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Strategy(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ConstantSumSolverCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::GetInitialPoolData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SimulateAllocateOrDeallocate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SimulateSwap(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Strategy(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for ConstantSumSolverCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::GetInitialPoolData(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SimulateAllocateOrDeallocate(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SimulateSwap(element) => ::core::fmt::Display::fmt(element, f),
                Self::Strategy(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<GetInitialPoolDataCall> for ConstantSumSolverCalls {
        fn from(value: GetInitialPoolDataCall) -> Self {
            Self::GetInitialPoolData(value)
        }
    }
    impl ::core::convert::From<SimulateAllocateOrDeallocateCall>
    for ConstantSumSolverCalls {
        fn from(value: SimulateAllocateOrDeallocateCall) -> Self {
            Self::SimulateAllocateOrDeallocate(value)
        }
    }
    impl ::core::convert::From<SimulateSwapCall> for ConstantSumSolverCalls {
        fn from(value: SimulateSwapCall) -> Self {
            Self::SimulateSwap(value)
        }
    }
    impl ::core::convert::From<StrategyCall> for ConstantSumSolverCalls {
        fn from(value: StrategyCall) -> Self {
            Self::Strategy(value)
        }
    }
    ///Container type for all return fields from the `getInitialPoolData` function with signature `getInitialPoolData(uint256,uint256,(uint256,uint256,address))` and selector `0x89ea8559`
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
    pub struct GetInitialPoolDataReturn(pub ::ethers::core::types::Bytes);
    ///Container type for all return fields from the `simulateAllocateOrDeallocate` function with signature `simulateAllocateOrDeallocate(uint256,bool,uint256,uint256)` and selector `0x8a1a20de`
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
    pub struct SimulateAllocateOrDeallocateReturn(
        pub bool,
        pub ::ethers::core::types::Bytes,
    );
    ///Container type for all return fields from the `simulateSwap` function with signature `simulateSwap(uint256,bool,uint256)` and selector `0x3928ff97`
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
    pub struct SimulateSwapReturn(
        pub bool,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::Bytes,
    );
    ///Container type for all return fields from the `strategy` function with signature `strategy()` and selector `0xa8c62e76`
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
    pub struct StrategyReturn(pub ::ethers::core::types::Address);
    ///`ConstantSumParams(uint256,uint256,address)`
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
    pub struct ConstantSumParams {
        pub price: ::ethers::core::types::U256,
        pub swap_fee: ::ethers::core::types::U256,
        pub controller: ::ethers::core::types::Address,
    }
}
