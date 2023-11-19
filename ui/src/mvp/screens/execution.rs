use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use arbiter_core::environment::builder::EnvironmentBuilder;
use ethers::{
    abi::{Token, Tokenizable},
    types::{Address, U256},
};
use iced::{Command, Element, Subscription};
use revm::primitives::{hash_map::HashMap as StorageMap, U256 as StorageValue};

use super::{
    app::Message,
    view::{self},
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

#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum TransactionSteps {
    #[default]
    Start,
    Simulated,
    Executed,
    Confirmed,
}

impl TransactionSteps {
    pub fn next(&self) -> Self {
        match self {
            Self::Start => Self::Simulated,
            Self::Simulated => Self::Executed,
            Self::Executed => Self::Confirmed,
            Self::Confirmed => Self::Start,
        }
    }

    pub fn get_cta(&self) -> String {
        match self {
            TransactionSteps::Start => "Review".to_string(),
            TransactionSteps::Simulated => "Execute".to_string(),
            TransactionSteps::Executed => "Confirming...".to_string(),
            TransactionSteps::Confirmed => "Exit".to_string(),
        }
    }

    pub fn get_instructions(&self) -> String {
        match self {
            TransactionSteps::Start => "Construct a transaction and then review it.".to_string(),
            TransactionSteps::Simulated => "Review simulated results then execute.".to_string(),
            TransactionSteps::Executed => "Wait for transaction to confirm.".to_string(),
            TransactionSteps::Confirmed => "Transaction confirmed. Exit to restart.".to_string(),
        }
    }
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
    // Highest level step that has been reached.
    checkpoint_step: TransactionSteps,
    pending_tx: bool,
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

pub type StorageDiffs = HashMap<StorageValue, (Option<StorageValue>, Option<StorageValue>)>;

#[derive(Default)]
pub struct Review {
    pub storage_before: StorageMap<StorageValue, StorageValue>,
    pub storage_after: StorageMap<StorageValue, StorageValue>,
    pub differences: Option<StorageDiffs>,
}

impl Review {
    /// Returns a diff of the before and after storage.
    /// The key is the storage slot, and the value is a tuple of the before and
    /// after values.
    pub fn get_diff(&self) -> StorageDiffs {
        let mut diff = HashMap::new();

        for (key, value) in self.storage_before.iter() {
            if let Some(after_value) = self.storage_after.get(key) {
                if after_value != value {
                    diff.insert(*key, (Some(*value), Some(*after_value)));
                }
            } else {
                diff.insert(*key, (Some(*value), None));
            }
        }

        for (key, value) in self.storage_after.iter() {
            if !self.storage_before.contains_key(key) {
                diff.insert(*key, (None, Some(*value)));
            }
        }

        diff
    }

    /// Applies the diff to the storage.
    pub fn apply_diff(&mut self) {
        self.differences = Some(self.get_diff());
    }
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
            checkpoint_step: TransactionSteps::default(),
            pending_tx: false,
        }
    }

    /// Resets the form state back to the start.
    #[tracing::instrument(skip(self))]
    fn handle_restart(&mut self) -> Command<Message> {
        self.unsealed = UnsealedTransaction::new();
        self.sealed = None;
        self.review = Review::default();
        self.user_feedback_message = None;
        self.pending_tx = false;
        self.checkpoint_step = TransactionSteps::Start;

        Command::perform(async { Ok::<(), ()>(()) }, |_| {
            app::Message::Execution(app::Execution::Arrived(TransactionSteps::Start))
        })
    }

    /// Seals the unsealed transaction and begins the simulation process.
    #[tracing::instrument(skip(self))]
    fn handle_simulate(&mut self) -> Command<Message> {
        // Return early if our checkpoint is past this step.
        if self.checkpoint_step > TransactionSteps::Simulated {
            return Command::none();
        }

        if self.unsealed.method.is_none() {
            self.unsealed = self.unsealed.clone().method("increment");
        }

        self.sealed = Some(self.unsealed.clone().seal());

        let scroll = self.sealed.clone().unwrap();
        let forker = self.forker.clone().unwrap();

        self.pending_tx = true;
        self.user_feedback_message = Some("Simulation in progress...".to_string());

        Command::perform(handle_simulate_scroll(scroll, forker), |res| {
            app::Message::Execution(app::Execution::Simulated(res))
        })
    }

    #[tracing::instrument(skip(self))]
    fn handle_execute(&mut self) -> Command<Message> {
        if self.checkpoint_step > TransactionSteps::Executed {
            return Command::none();
        }

        if self.sealed.is_none() {
            return Command::none();
        }

        let scroll = self.sealed.clone().unwrap();
        let forker = self.forker.clone().unwrap();

        self.pending_tx = true;
        self.user_feedback_message = Some("Sending transaction...".to_string());

        Command::perform(handle_execute_scroll(scroll, forker), |res| {
            app::Message::Execution(app::Execution::Executed(res))
        })
    }

    #[tracing::instrument(skip(self, scroll), fields(storage_before = ?self.review.storage_before.clone(), storage_after = ?self.review.storage_after.clone()))]
    fn handle_completed_simulation(&mut self, scroll: Scroll) -> Command<Message> {
        tracing::info!("Simulated tx: {:?}", scroll);
        self.pending_tx = false;
        self.checkpoint_step = TransactionSteps::Simulated;

        self.user_feedback_message = Some("Transaction simulated!".to_string());
        self.sealed = Some(scroll.clone());
        self.handle_load_storages()
    }

    #[tracing::instrument(skip(self, scroll),  fields(storage_before = ?self.review.storage_before.clone(), storage_after = ?self.review.storage_after.clone()))]
    fn handle_completed_execution(&mut self, scroll: Scroll) -> Command<Message> {
        tracing::info!("Executed tx: {:?}", scroll);
        self.pending_tx = false;
        self.checkpoint_step = TransactionSteps::Executed;

        self.user_feedback_message = Some("Transaction executed!".to_string());
        self.sealed = Some(scroll.clone());
        let _ = self.handle_load_storages();

        return Command::perform(async { Ok::<(), ()>(()) }, |_| {
            app::Message::Execution(app::Execution::Confirmed)
        });
    }

    #[tracing::instrument(skip(self))]
    fn handle_load_storages(&mut self) -> Command<Message> {
        let scroll = self.sealed.clone().unwrap();
        let account = scroll.payload.target.clone();

        let before = scroll.try_storage_before(account);
        match before {
            Ok(storage) => {
                tracing::debug!("Loading account storage into before: {:?}", storage.clone());

                self.review.storage_before = storage;
            }
            Err(e) => {
                tracing::warn!("No before storage found in scroll. {}", e);
            }
        };

        let after = scroll.try_storage_after(account);
        match after {
            Ok(storage) => {
                tracing::debug!("Loading account storage into after: {:?}", storage.clone());
                self.review.storage_after = storage;
            }
            Err(e) => {
                tracing::warn!("No before storage found in scroll. {}", e);
            }
        };

        self.review.apply_diff();

        Command::none()
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
                        view::Execution::Restart => {
                            return self.handle_restart();
                        }
                        // Handles routing to different steps during execution.
                        view::Execution::Route(route) => {
                            // Only route to the step if its been reached.
                            if route > self.checkpoint_step {
                                return Command::none();
                            }

                            return Command::perform(async { Ok::<(), ()>(()) }, |_| {
                                app::Message::Execution(app::Execution::Arrived(route))
                            });
                        }

                        // todo: probably remove in favor of route...
                        view::Execution::Next => {
                            // Get the next step
                            let next_step = self.step.next();

                            // If the next step is start, we need to reset our form state.
                            if next_step == TransactionSteps::Start {
                                return self.handle_restart();
                            }

                            // Checkpoint the step so we don't re-trigger the step actions.
                            self.checkpoint_step = next_step.clone();

                            // Route to the next step.
                            return Command::perform(async { Ok::<(), ()>(()) }, |_| {
                                app::Message::Execution(app::Execution::Arrived(next_step))
                            });
                        }
                        // todo: probably remove in favor of route...
                        view::Execution::Previous => {
                            self.step = match self.step {
                                TransactionSteps::Start => TransactionSteps::Confirmed,
                                TransactionSteps::Simulated => TransactionSteps::Start,
                                TransactionSteps::Executed => TransactionSteps::Simulated,
                                TransactionSteps::Confirmed => TransactionSteps::Executed,
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
                        view::Execution::FromAddressChanged(from) => {
                            tracing::debug!("From address changed: {}", from);
                            // todo: implement
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
                // todo: routing needs to be validated.
                app::Execution::Arrived(step) => {
                    self.step = step.clone();

                    match step {
                        TransactionSteps::Simulated => {
                            return self.handle_simulate();
                        }
                        TransactionSteps::Executed => {
                            return self.handle_execute();
                        }
                        TransactionSteps::Confirmed => {
                            // todo: implement
                        }
                        _ => {}
                    }

                    Command::none()
                }
                app::Execution::Simulated(msg) => {
                    match msg {
                        Ok(scroll) => {
                            return self.handle_completed_simulation(scroll);
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
                            return self.handle_completed_execution(scroll);
                        }
                        Err(e) => {
                            tracing::error!("Error executing tx: {:?}", e);
                        }
                    }

                    Command::none()
                }
                app::Execution::Confirmed => {
                    self.user_feedback_message =
                        Some("Transaction executed and confirmed.".to_string());

                    // Add the storage diffs to the [`Review`] struct so we can display them.
                    self.review.apply_diff();

                    // Set the checkpoint here.
                    self.checkpoint_step = TransactionSteps::Confirmed;

                    // Finally, route to the confirmed step.
                    return Command::perform(async { Ok::<(), ()>(()) }, |_| {
                        app::Message::Execution(app::Execution::Arrived(
                            TransactionSteps::Confirmed,
                        ))
                    });
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

        let review_diffs = self.review.differences.clone();

        view::app_layout(
            &view::Page::Execute,
            view::execute::execution_layout(
                self.step.clone(),
                input,
                sorted.clone(),
                selected.clone(),
                self.user_feedback_message.clone(),
                review_diffs,
                self.checkpoint_step.clone(),
                self.pending_tx.clone(),
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
