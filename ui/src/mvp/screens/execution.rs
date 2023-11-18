use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use arbiter_core::{environment::builder::EnvironmentBuilder, middleware::cast::recast_address};
use ethers::{
    abi::{Token, Tokenizable},
    types::{Address, U256},
};
use iced::{Command, Element, Subscription};
use revm::primitives::{hash_map::HashMap as StorageMap, U256 as StorageValue};

use super::{
    app::Message,
    view::{self, execute::Addresses},
    State, *,
};
use crate::mvp::{
    api::{
        address_book::{AddressBookCategory, AddressBookManager},
        forking::Forker,
        local::Local,
        scroll::{Scroll, UnsealedTransaction},
    },
    units::address_to_string,
};

#[derive(Default, Clone, PartialEq, Eq)]
pub enum TransactionSteps {
    #[default]
    Start,
    Review,
    Simulated,
    Confirmed,
}

pub struct Execution {
    #[allow(dead_code)]
    unsealed: UnsealedTransaction,
    #[allow(dead_code)]
    sealed: Option<Scroll>,
    step: TransactionSteps,
    #[allow(dead_code)]
    review: Review,
    #[allow(dead_code)]
    local: Local<Ws>,
    #[allow(dead_code)]
    address_books: AddressBookManager,
    forker: Option<Arc<tokio::sync::Mutex<Forker>>>,
    user_feedback_message: Option<String>,
}

pub fn get_artifact_path(name: &str) -> PathBuf {
    // counter -> Counter
    let contract_name = name.chars().next().unwrap().to_uppercase().to_string() + &name[1..];
    // ui/
    let cwd = std::env::current_dir().unwrap();
    // root/contracts/out/Counter.sol/Counter.json
    // todo: clean this up...
    Path::new(&cwd)
        .join("contracts")
        .join("out")
        .join(format!("{}.sol", contract_name))
        .join(format!("{}.json", contract_name))
}

#[derive(Default)]
pub struct Review {
    pub storage_before: StorageMap<StorageValue, StorageValue>,
    pub storage_after: StorageMap<StorageValue, StorageValue>,
}

impl Execution {
    pub fn new(local: Local<Ws>, address_books: AddressBookManager) -> Self {
        // add a default address to untrusted address book
        let mut label = "default";
        let default_address = match local.counter_contract {
            Some(address) => {
                label = "counter";
                address
            }
            // Address from deploying counter contract in dev mode.
            None => "0x5fbdb2315678afecb367f032d93f642f64180aa3"
                .parse::<Address>()
                .unwrap(),
        };
        let mut books = address_books.clone();
        books.add(
            default_address,
            label.to_string(),
            AddressBookCategory::Untrusted,
        );

        let forker = Arc::new(tokio::sync::Mutex::new(Forker::new(
            EnvironmentBuilder::new().build(),
            local.client.clone(),
            0,
            None,
        )));

        Self {
            unsealed: UnsealedTransaction::new(),
            sealed: None,
            step: TransactionSteps::default(),
            review: Review::default(),
            local,
            address_books: books,
            forker: Some(forker),
            user_feedback_message: None,
        }
    }

    /// Seals the unsealed transaction and begins the simulation process.
    #[tracing::instrument(skip(self))]
    fn handle_review(&mut self) -> Command<Message> {
        if self.unsealed.method.is_none() {
            self.unsealed = self.unsealed.clone().method("increment");
        }

        self.sealed = Some(self.unsealed.clone().seal());

        let scroll = self.sealed.clone().unwrap();
        let forker = self.forker.clone().unwrap();

        Command::perform(handle_simulate_scroll(scroll, forker), |res| {
            app::Message::Execution(app::Execution::Simulated(res))
        })
    }

    #[tracing::instrument(skip(self))]
    fn handle_execute(&mut self) -> Command<Message> {
        if self.sealed.is_none() {
            return Command::none();
        }

        let scroll = self.sealed.clone().unwrap();
        let forker = self.forker.clone().unwrap();

        Command::perform(handle_execute_scroll(scroll, forker), |res| {
            app::Message::Execution(app::Execution::Executed(res))
        })
    }
}

#[tracing::instrument(skip(scroll, forker))]
async fn handle_simulate_scroll(
    scroll: Scroll,
    forker: Arc<tokio::sync::Mutex<Forker>>,
) -> anyhow::Result<Scroll, anyhow::Error> {
    let mut scroll = scroll.clone();

    let locked = forker.lock().await;

    // Get the block number and load it into the forker.
    let block = locked.load_block_number().await?;
    tracing::debug!("Loaded block number: {}", block);

    // Simulate the tx.
    let _ = scroll.simulate(&locked, Some(block)).await?;

    tracing::debug!(
        "Simulated tx, before storage: {:?}",
        scroll.stages.before.clone()
    );

    Ok(scroll)
}

#[tracing::instrument(skip(scroll, forker))]
async fn handle_execute_scroll(
    scroll: Scroll,
    forker: Arc<tokio::sync::Mutex<Forker>>,
) -> anyhow::Result<Scroll, anyhow::Error> {
    let mut scroll = scroll.clone();

    let locked = forker.lock().await;
    let block = locked.load_block_number().await?;

    let res = scroll.execute(&locked, Some(block)).await?;

    tracing::debug!("Executed tx: {:?}", res);

    // Loading into db.
    let block_number = locked.load_block_number().await?;

    scroll.load_after(&locked, Some(block_number))?;

    Ok(scroll)
}

impl State for Execution {
    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Empty => Command::none(),
            Message::View(msg) => {
                match msg {
                    view::Message::Execution(e) => match e {
                        view::Execution::Next => {
                            self.step = match self.step {
                                TransactionSteps::Start => TransactionSteps::Review,
                                TransactionSteps::Review => TransactionSteps::Simulated,
                                TransactionSteps::Simulated => TransactionSteps::Confirmed,
                                TransactionSteps::Confirmed => TransactionSteps::Start,
                            };

                            if self.step == TransactionSteps::Review {
                                return self.handle_review();
                            }

                            if self.step == TransactionSteps::Confirmed {
                                return self.handle_execute();
                            }
                        }
                        view::Execution::Previous => {
                            self.step = match self.step {
                                TransactionSteps::Start => TransactionSteps::Confirmed,
                                TransactionSteps::Review => TransactionSteps::Start,
                                TransactionSteps::Simulated => TransactionSteps::Review,
                                TransactionSteps::Confirmed => TransactionSteps::Simulated,
                            };
                        }
                        view::Execution::ToAddressChanged(value) => {
                            let value = value.parse::<Address>().unwrap();
                            let label = self
                                .address_books
                                .get(&value, AddressBookCategory::Untrusted)
                                .unwrap()
                                .clone();

                            let path = get_artifact_path(&label);
                            self.unsealed = self.unsealed.clone().artifact(path);

                            self.unsealed = self.unsealed.clone().target(value);
                        }
                        view::Execution::AmountChanged(amount) => match amount {
                            Some(amount) => {
                                let amount: Token = amount.into_token();
                                // todo: fix this too!
                                self.unsealed.arguments = vec![amount];
                            }
                            None => {}
                        },
                    },
                    _ => {}
                }

                Command::none()
            }
            Message::Simulation(_) => Command::none(),
            Message::Data(_) => Command::none(),
            Message::AddressBook(_) => Command::none(),
            Message::Execution(msg) => match msg {
                app::Execution::Simulated(msg) => {
                    match msg {
                        Ok(scroll) => {
                            tracing::info!("Simulated tx: {:?}", scroll);
                            self.sealed = Some(scroll.clone());

                            let scroll_storage = scroll.stages.before.clone();

                            let storage = match scroll_storage {
                                Some(storage) => {
                                    let address: revm::primitives::Address =
                                        scroll.payload.target.clone().to_fixed_bytes().into();
                                    let account = storage.accounts.get(&address).unwrap();

                                    tracing::debug!(
                                        "Loading account storage into before: {:?}",
                                        account.storage
                                    );
                                    account.storage.clone()
                                }
                                None => {
                                    tracing::error!("No storage found in scroll");
                                    return Command::none();
                                }
                            };

                            // todo: update this to not hold the revm storage type?
                            self.review.storage_before = storage;
                            self.user_feedback_message = Some("Transaction simulated!".to_string());
                        }
                        Err(e) => {
                            tracing::error!("Error simulating tx: {:?}", e);
                        }
                    }

                    Command::none()
                }
                app::Execution::Executed(msg) => {
                    match msg {
                        Ok(scroll) => {
                            tracing::info!("Executed tx: {:?}", scroll);
                            self.sealed = Some(scroll.clone());

                            let scroll_storage = scroll.stages.after.clone();

                            let storage = match scroll_storage {
                                Some(storage) => {
                                    let address: revm::primitives::Address =
                                        scroll.payload.target.clone().to_fixed_bytes().into();
                                    let account = storage.accounts.get(&address).unwrap();

                                    tracing::debug!(
                                        "Loading account storage into after: {:?}",
                                        account.storage
                                    );
                                    account.storage.clone()
                                }
                                None => {
                                    tracing::error!("No storage found in scroll");
                                    return Command::none();
                                }
                            };

                            // todo: update this to not hold the revm storage type?
                            self.review.storage_after = storage;
                            self.user_feedback_message = Some("Transaction executed!".to_string());
                        }
                        Err(e) => {
                            tracing::error!("Error executing tx: {:?}", e);
                        }
                    }

                    Command::none()
                }
                _ => Command::none(),
            },
            _ => Command::none(),
        }
    }

    fn view<'a>(&'a self) -> Element<'a, view::Message> {
        let sorted = self
            .address_books
            .list_sorted(AddressBookCategory::Untrusted);

        let selected = address_to_string(&self.unsealed.target);

        let input: Vec<String> = self
            .unsealed
            .arguments
            .iter()
            .map(|t| t.to_string())
            .collect();

        // todo: fix this
        let input = match input.len() {
            1 => input[0].clone(),
            _ => "".to_string(),
        };

        view::app_layout(
            &view::Page::Execute,
            view::execute::execution_layout(
                self.step.clone(),
                input,
                sorted.clone(),
                selected.clone(),
                self.user_feedback_message.clone(),
            ),
        )
        .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::none()
    }

    fn load(&self) -> Command<Message> {
        Command::none()
    }
}
