pub use portfolio_tracker::*;
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
pub mod portfolio_tracker {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("logPortfolio"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("logPortfolio"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenX"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenY"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("GhostEvent"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("GhostEvent"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("ghosted"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("LogPortfolio"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("LogPortfolio"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("tokenXBalance"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("tokenYBalance"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("blockTimestamp"),
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
    pub static PORTFOLIOTRACKER_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[Pa\x03\xC6\x80a\0m`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\0xW`\x005`\xE0\x1C\x80c;=\x82\x1F\x14a\0\xDDW[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\0\xF0a\0\xEB6`\x04a\x03>V[a\0\xF2V[\0[`@Qcp\xA0\x821`\xE0\x1B\x81R3`\x04\x82\x01R`\0\x90`\x01`\x01`\xA0\x1B\x03\x84\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x01\x84W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x01\x98W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xBC\x91\x90a\x03tV[`@Qcp\xA0\x821`\xE0\x1B\x81R3`\x04\x82\x01R\x90\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x84\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x02QW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x02eW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\x89\x91\x90a\x03tV[`@\x80Q\x84\x81R` \x81\x01\x83\x90RB\x81\x83\x01R\x90Q\x91\x92P\x7FT\xA6\xD40z\x11\x95u\xDE\x15\xB0\xAD\x97z\xB1\x87\xCD\xA7\xE9\x7F00\x8A\xF0\xA7\x1D#^U_\xF4\xE6\x91\x90\x81\x90\x03``\x01\x90\xA1PPPPV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x039W`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x03TWa\x03Ta\x02\xD2V[a\x03]\x83a\x03\"V[\x91Pa\x03k` \x84\x01a\x03\"V[\x90P\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\x03\x89Wa\x03\x89a\x02\xD2V[PQ\x91\x90PV\xFE\xA2dipfsX\"\x12 \x9BS\xE0p\x9A&\xBD\x08\xC2\xB5BW5\x93d?I\x1Erp\xAA\x90\x8BA\xA6\xED\xFE\x81\xE4\xCA\xD8\x06dsolcC\0\x08\x16\x003";
    /// The bytecode of the contract.
    pub static PORTFOLIOTRACKER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\0xW`\x005`\xE0\x1C\x80c;=\x82\x1F\x14a\0\xDDW[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\0\xF0a\0\xEB6`\x04a\x03>V[a\0\xF2V[\0[`@Qcp\xA0\x821`\xE0\x1B\x81R3`\x04\x82\x01R`\0\x90`\x01`\x01`\xA0\x1B\x03\x84\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x01\x84W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x01\x98W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xBC\x91\x90a\x03tV[`@Qcp\xA0\x821`\xE0\x1B\x81R3`\x04\x82\x01R\x90\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x84\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x02QW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x02eW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\x89\x91\x90a\x03tV[`@\x80Q\x84\x81R` \x81\x01\x83\x90RB\x81\x83\x01R\x90Q\x91\x92P\x7FT\xA6\xD40z\x11\x95u\xDE\x15\xB0\xAD\x97z\xB1\x87\xCD\xA7\xE9\x7F00\x8A\xF0\xA7\x1D#^U_\xF4\xE6\x91\x90\x81\x90\x03``\x01\x90\xA1PPPPV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x039W`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x03TWa\x03Ta\x02\xD2V[a\x03]\x83a\x03\"V[\x91Pa\x03k` \x84\x01a\x03\"V[\x90P\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\x03\x89Wa\x03\x89a\x02\xD2V[PQ\x91\x90PV\xFE\xA2dipfsX\"\x12 \x9BS\xE0p\x9A&\xBD\x08\xC2\xB5BW5\x93d?I\x1Erp\xAA\x90\x8BA\xA6\xED\xFE\x81\xE4\xCA\xD8\x06dsolcC\0\x08\x16\x003";
    /// The deployed bytecode of the contract.
    pub static PORTFOLIOTRACKER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct PortfolioTracker<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for PortfolioTracker<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for PortfolioTracker<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for PortfolioTracker<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for PortfolioTracker<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(PortfolioTracker))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> PortfolioTracker<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    PORTFOLIOTRACKER_ABI.clone(),
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
                PORTFOLIOTRACKER_ABI.clone(),
                PORTFOLIOTRACKER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `logPortfolio` (0x3b3d821f) function
        pub fn log_portfolio(
            &self,
            token_x: ::ethers::core::types::Address,
            token_y: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([59, 61, 130, 31], (token_x, token_y))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `GhostEvent` event
        pub fn ghost_event_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            GhostEventFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `LogPortfolio` event
        pub fn log_portfolio_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogPortfolioFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PortfolioTrackerEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for PortfolioTracker<M> {
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
    #[ethevent(name = "GhostEvent", abi = "GhostEvent(bool)")]
    pub struct GhostEventFilter {
        pub ghosted: bool,
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
    #[ethevent(name = "LogPortfolio", abi = "LogPortfolio(uint256,uint256,uint256)")]
    pub struct LogPortfolioFilter {
        pub token_x_balance: ::ethers::core::types::U256,
        pub token_y_balance: ::ethers::core::types::U256,
        pub block_timestamp: ::ethers::core::types::U256,
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
    pub enum PortfolioTrackerEvents {
        GhostEventFilter(GhostEventFilter),
        LogPortfolioFilter(LogPortfolioFilter),
    }
    impl ::ethers::contract::EthLogDecode for PortfolioTrackerEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = GhostEventFilter::decode_log(log) {
                return Ok(PortfolioTrackerEvents::GhostEventFilter(decoded));
            }
            if let Ok(decoded) = LogPortfolioFilter::decode_log(log) {
                return Ok(PortfolioTrackerEvents::LogPortfolioFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for PortfolioTrackerEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::GhostEventFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogPortfolioFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<GhostEventFilter> for PortfolioTrackerEvents {
        fn from(value: GhostEventFilter) -> Self {
            Self::GhostEventFilter(value)
        }
    }
    impl ::core::convert::From<LogPortfolioFilter> for PortfolioTrackerEvents {
        fn from(value: LogPortfolioFilter) -> Self {
            Self::LogPortfolioFilter(value)
        }
    }
    ///Container type for all input parameters for the `logPortfolio` function with signature `logPortfolio(address,address)` and selector `0x3b3d821f`
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
    #[ethcall(name = "logPortfolio", abi = "logPortfolio(address,address)")]
    pub struct LogPortfolioCall {
        pub token_x: ::ethers::core::types::Address,
        pub token_y: ::ethers::core::types::Address,
    }
}
