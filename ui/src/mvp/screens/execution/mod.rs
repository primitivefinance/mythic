pub mod form;
pub mod processing;
pub mod send;
pub mod utils;

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

use self::{
    form::{FormMessage, TransactionSteps},
    processing::StorageDiffs,
    utils::{empty_async, get_artifact_path},
};
use super::{
    app::Message,
    view::{self},
    State, *,
};
use crate::mvp::{
    api::{
        contacts::{self, ContactList, ContactValue, Contacts},
        forking::Forker,
        local::Local,
        scroll::{Scroll, UnsealedTransaction},
    },
    app::{Chains, Storage},
    units::address_to_string,
};

#[derive(Debug, Clone, Default)]
pub struct Cache {
    pub to_list: ContactList,
    pub from_list: ContactList,
    pub target_list: ContactList,
    pub simulated_results: Option<StorageDiffs>,
    pub executed_results: Option<StorageDiffs>,
}

pub struct Execution {
    chains: Chains,
    storage: Storage,
    form: form::Form,
    processing: processing::Processing,
    cache: Cache,
}

impl Execution {
    pub fn new(chains: Chains, storage: Storage) -> Self {
        let forker = Forker::new(
            EnvironmentBuilder::new().build(),
            chains.local.client.clone(),
            0,
            None,
        );

        // Cache these contact lists so we don't need to refetch them on view...
        // todo: add a way to refetch them.

        let contacts = storage.profile.contacts.clone();

        // todo: this looks expensive...
        let untrusted = contacts.get_list(contacts::Category::Untrusted);
        let trusted = contacts.get_list(contacts::Category::Trusted);
        let contracts = contacts.get_class_list(contacts::Class::Contract);

        tracing::info!("Untrusted: {:?}", untrusted);
        tracing::info!("Trusted: {:?}", trusted);
        tracing::info!("Contracts: {:?}", contracts);

        // Unwraps should be safe...
        let cache = Cache {
            to_list: untrusted.clone().unwrap().clone(),
            from_list: trusted.clone().unwrap().clone(),
            target_list: contracts.unwrap().clone(),
            ..Default::default()
        };

        Self {
            chains,
            storage,
            form: form::Form::new(),
            processing: processing::Processing::new(forker),
            cache,
        }
    }

    fn cache_slots(&mut self) {
        let simulated = self.processing.simulated.differences.clone();
        let executed = self.processing.executed.differences.clone();

        self.cache.simulated_results = simulated;
        self.cache.executed_results = executed;
    }

    /// Resets the form state back to the start.
    #[tracing::instrument(skip(self))]
    fn handle_restart(&mut self) -> Command<Message> {
        self.processing.reset();

        Command::perform(empty_async(), |_| {
            view::Execution::Form(FormMessage::Reset).into()
        })
    }

    /// Seals the unsealed transaction and begins the simulation process.
    #[tracing::instrument(skip(self))]
    fn handle_simulate(&mut self) -> Command<Message> {
        // todo: fix this
        let target = self.form.fields.target.clone().unwrap();
        let value = target.parse::<Address>().unwrap();
        self.processing.unsealed.target = value;

        let contact = self
            .storage
            .profile
            .contacts
            .get(&value, contacts::Category::Untrusted)
            .unwrap()
            .clone();
        let path = get_artifact_path(&contact.label);
        self.processing.unsealed.artifact = path;

        // todo: fix this too!
        let amount = self.form.fields.amount.clone();
        let amount: Token = amount.unwrap().into_token();
        self.processing.unsealed.arguments = vec![amount];

        let cmd = self.processing.simulate();

        // todo: is batch the right way to handle this?
        Command::batch(vec![
            Command::perform(empty_async(), |_| {
                view::Execution::Form(FormMessage::Simulate).into()
            }),
            cmd,
        ])
    }

    #[tracing::instrument(skip(self))]
    fn handle_execute(&mut self) -> Command<Message> {
        let cmd = self.processing.execute();

        Command::batch(vec![
            Command::perform(empty_async(), |_| {
                view::Execution::Form(FormMessage::Execute).into()
            }),
            cmd,
        ])
    }

    #[tracing::instrument(skip(self, scroll))]
    fn handle_completed_simulation(&mut self, scroll: Scroll) -> Command<Message> {
        tracing::info!("Simulated tx: {:?}", scroll);

        // todo: we do this complete step here, then do try load, maybe update?
        self.processing.complete_simulation(scroll.clone());

        let loaded = self.processing.try_load_simulated();
        if loaded.is_err() {
            tracing::error!("Error loading simulated tx: {:?}", loaded);
        }

        self.cache_slots();

        Command::perform(empty_async(), |_| {
            view::Execution::Form(FormMessage::CompleteSimulate).into()
        })
    }

    #[tracing::instrument(skip(self, scroll))]
    fn handle_completed_execution(&mut self, scroll: Scroll) -> Command<Message> {
        tracing::info!("Executed tx: {:?}", scroll);

        let _ = self.processing.complete_execution(scroll.clone());
        let loaded = self.processing.try_load_executed();
        if loaded.is_err() {
            tracing::error!("Error loading executed tx: {:?}", loaded);
        }

        self.cache_slots();

        Command::batch(vec![
            Command::perform(empty_async(), |_| {
                view::Execution::Form(FormMessage::CompleteExecute).into()
            }),
            Command::perform(empty_async(), |_| app::Execution::Confirmed.into()),
        ])
    }
}

impl State for Execution {
    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Empty => Command::none(),
            Message::View(msg) => {
                match msg {
                    view::Message::Execution(e) => match e {
                        view::Execution::Form(form_message) => {
                            return self.form.update(form_message)
                        }
                        view::Execution::Simulate => return self.handle_simulate(),
                        view::Execution::Execute => return self.handle_execute(),
                        view::Execution::Results => {}
                        view::Execution::Reset => return self.handle_restart(),
                    },
                    _ => {}
                }

                Command::none()
            }
            Message::WindowsMessage(app::WindowsMessage::Execution(msg)) => match msg {
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
                app::Execution::Confirmed => Command::perform(empty_async(), |_| {
                    app::Execution::Form(FormMessage::Confirm).into()
                }),
                _ => Command::none(),
            },
            _ => Command::none(),
        }
    }

    fn view<'a>(&'a self) -> Element<'a, view::Message> {
        // todo: add refs to cache?
        // todo: add performance benches
        view::app_layout(
            &view::Page::Execute,
            view::execute::execution_layout(
                &self.form,
                &self.cache.from_list,
                &self.cache.to_list,
                &self.cache.target_list,
                self.cache.simulated_results.clone(),
                self.cache.executed_results.clone(),
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
