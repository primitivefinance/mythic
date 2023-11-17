use std::collections::HashMap;

use ethers::types::{Address, U256};
use iced::{Command, Element, Subscription};

use super::{
    app::Message,
    view::{self, execute::Addresses},
    State, *,
};
use crate::mvp::api::{
    local::Local,
    scroll::{Scroll, UnsealedTransaction},
};

#[derive(Default)]
pub struct CraftingTransaction {
    pub to: Addresses,
    pub amount: String,
}

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
}

#[derive(Default)]
pub struct Review {
    pub storage_before: HashMap<U256, U256>,
    pub storage_after: HashMap<U256, U256>,
}

impl Execution {
    pub fn new(local: Local<Ws>) -> Self {
        Self {
            unsealed: UnsealedTransaction::new(),
            sealed: None,
            step: TransactionSteps::default(),
            review: Review::default(),
            local,
        }
    }

    /// Uses the `apply_cheatcode` method on a client to fetch the storage slots
    /// of the target address.
    fn handle_review(&self) -> Command<Message> {
        let _target_address = Address::zero();

        Command::none()
    }
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
                        }
                        view::Execution::Previous => {
                            self.step = match self.step {
                                TransactionSteps::Start => TransactionSteps::Confirmed,
                                TransactionSteps::Review => TransactionSteps::Start,
                                TransactionSteps::Simulated => TransactionSteps::Review,
                                TransactionSteps::Confirmed => TransactionSteps::Simulated,
                            };
                        }
                        view::Execution::ToAddressChanged(_to) => {
                            // self.unsealed.target = to;
                        }
                        view::Execution::AmountChanged(amount) => match amount {
                            Some(_amount) => {
                                // let amount: Token = amount.into_token();
                                // self.unsealed.arg(amount);
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
        }
    }

    fn view<'a>(&'a self) -> Element<'a, view::Message> {
        view::app_layout(
            &view::Page::Execute,
            view::execute::execution_layout(
                self.step.clone(),
                "temp".to_string(),
                Addresses::Trusted,
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
