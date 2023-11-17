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
    const __BYTECODE: &[u8] = b"`\x80\x80`@R4a\0\x19W`@Qa\x04^\x90\x81a\0g\x829\xF3[bF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01Ra7\xB7`\xF1\x1B`d\x82\x01R`\x84\x90\xFD\xFE`\x80`@\x81\x81R`\x04\x91\x826\x10\x15a\0\x95W[`\x84\x83` \x84Q\x91\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01R\x7Fnor receive functions\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R\xFD[`\0\x90\x815`\xE0\x1Cc;=\x82\x1F\x14a\0\xADWPa\0\x12V[\x92\x90\x924a\x02fWP\x81\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x02\x0EW\x805\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x83\x16\x80\x93\x03a\x02bW`$5\x91\x82\x16\x80\x92\x03a\x02bW\x82;\x15a\x02$W\x83Q\x92\x83\x91\x7Fp\xA0\x821\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x82\x84R3\x82\x85\x01R\x83`$` \x95\x86\x93Z\xFA\x94\x85\x15a\x02XW\x87\x95a\x02)W[P\x83;\x15a\x02$W`$\x83\x92\x87Q\x95\x86\x93\x84\x92\x83R3\x90\x83\x01RZ\xFA\x91\x82\x15a\x02\x1AW\x85\x92a\x01\xC0W[P\x92``\x92\x91\x7FT\xA6\xD40z\x11\x95u\xDE\x15\xB0\xAD\x97z\xB1\x87\xCD\xA7\xE9\x7F00\x8A\xF0\xA7\x1D#^U_\xF4\xE6\x94\x82Q\x93\x84R\x83\x01RB\x90\x82\x01R\xA1\x80\xF3[\x92\x91P\x92\x83\x83\x81=\x83\x11a\x02\x13W[a\x01\xD9\x81\x83a\x03\xEEV[\x81\x01\x03\x12a\x02\x0EW\x91Q\x91\x92\x90\x91\x90\x7FT\xA6\xD40z\x11\x95u\xDE\x15\xB0\xAD\x97z\xB1\x87\xCD\xA7\xE9\x7F00\x8A\xF0\xA7\x1D#^U_\xF4\xE6a\x01\x87V[a\x02\xE6V[P=a\x01\xCFV[\x84Q=\x87\x82>=\x90\xFD[a\x03jV[\x90\x94P\x82\x81\x81=\x83\x11a\x02QW[a\x02A\x81\x83a\x03\xEEV[\x81\x01\x03\x12a\x02\x0EWQ\x938a\x01]V[P=a\x027V[\x86Q=\x89\x82>=\x90\xFD[\x84\x80\xFD[` `\x84\x92\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R\xFD[`\x84`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01R\x7Frt\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R\xFD[`\x84`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R\xFD[\x90`\x1F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x04/W`@RV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD";
    /// The bytecode of the contract.
    pub static PORTFOLIOTRACKER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@\x81\x81R`\x04\x91\x826\x10\x15a\0\x95W[`\x84\x83` \x84Q\x91\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01R\x7Fnor receive functions\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R\xFD[`\0\x90\x815`\xE0\x1Cc;=\x82\x1F\x14a\0\xADWPa\0\x12V[\x92\x90\x924a\x02fWP\x81\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x02\x0EW\x805\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x83\x16\x80\x93\x03a\x02bW`$5\x91\x82\x16\x80\x92\x03a\x02bW\x82;\x15a\x02$W\x83Q\x92\x83\x91\x7Fp\xA0\x821\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x82\x84R3\x82\x85\x01R\x83`$` \x95\x86\x93Z\xFA\x94\x85\x15a\x02XW\x87\x95a\x02)W[P\x83;\x15a\x02$W`$\x83\x92\x87Q\x95\x86\x93\x84\x92\x83R3\x90\x83\x01RZ\xFA\x91\x82\x15a\x02\x1AW\x85\x92a\x01\xC0W[P\x92``\x92\x91\x7FT\xA6\xD40z\x11\x95u\xDE\x15\xB0\xAD\x97z\xB1\x87\xCD\xA7\xE9\x7F00\x8A\xF0\xA7\x1D#^U_\xF4\xE6\x94\x82Q\x93\x84R\x83\x01RB\x90\x82\x01R\xA1\x80\xF3[\x92\x91P\x92\x83\x83\x81=\x83\x11a\x02\x13W[a\x01\xD9\x81\x83a\x03\xEEV[\x81\x01\x03\x12a\x02\x0EW\x91Q\x91\x92\x90\x91\x90\x7FT\xA6\xD40z\x11\x95u\xDE\x15\xB0\xAD\x97z\xB1\x87\xCD\xA7\xE9\x7F00\x8A\xF0\xA7\x1D#^U_\xF4\xE6a\x01\x87V[a\x02\xE6V[P=a\x01\xCFV[\x84Q=\x87\x82>=\x90\xFD[a\x03jV[\x90\x94P\x82\x81\x81=\x83\x11a\x02QW[a\x02A\x81\x83a\x03\xEEV[\x81\x01\x03\x12a\x02\x0EWQ\x938a\x01]V[P=a\x027V[\x86Q=\x89\x82>=\x90\xFD[\x84\x80\xFD[` `\x84\x92\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R\xFD[`\x84`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01R\x7Frt\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R\xFD[`\x84`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R\xFD[\x90`\x1F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x04/W`@RV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD";
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
