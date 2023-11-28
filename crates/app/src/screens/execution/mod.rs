pub mod form;
pub mod processing;
pub mod send;
pub mod utils;

use std::{self};

use api::contacts::{self, ContactList};
use arbiter_core::environment::builder::EnvironmentBuilder;
use clients::{forking::forking::Forker, scroll::Scroll};
use ethers::types::Address;
use iced::{Command, Element, Subscription};

use self::{
    form::FormMessage,
    processing::StorageDiffs,
    utils::{empty_async, get_artifact_path},
};
use super::{
    app::{Chains, Message, Storage},
    view::{self},
    State, *,
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
    #[allow(dead_code)]
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

        let forker_address = chains.clone().local.client.unwrap().clone().address();
        tracing::info!("Forker address: 0x{:x}", forker_address);

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
            to_list: untrusted.unwrap().clone(),
            from_list: trusted.unwrap().clone(),
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
        let from = self.form.fields.from.clone();
        let target = self.form.fields.target.clone().unwrap();
        let value = target.parse::<Address>().unwrap();

        // todo: why is this happening here?
        self.processing.unsealed.from = Some(from.unwrap().parse::<Address>().unwrap());
        self.processing.unsealed.target = value;

        let contact = self.storage.profile.contacts.find(&value);
        let contact = match contact {
            Some(contact) => contact,
            None => {
                tracing::error!("Could not find contact for address: {:?}", value);
                return Command::none();
            }
        };
        let path = get_artifact_path(&contact.label.to_lowercase());
        self.processing.unsealed.artifact = path;

        // todo: fix this too!
        let to = self.form.fields.to.clone().unwrap();
        let amount = self.form.fields.amount.clone();
        let amount = amount.unwrap();

        self.processing.unsealed.method = Some("transfer".to_string());
        // todo: this order matters right now...
        self.processing.unsealed.arguments = vec![to, amount];

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

        tracing::info!(
            "Simulated storage diffs: {:?}",
            self.cache.simulated_results
        );

        Command::perform(empty_async(), |_| {
            view::Execution::Form(FormMessage::CompleteSimulate).into()
        })
    }

    #[tracing::instrument(skip(self, scroll))]
    fn handle_completed_execution(&mut self, scroll: Scroll) -> Command<Message> {
        tracing::info!("Executed tx: {:?}", scroll);

        self.processing.complete_execution(scroll.clone());
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

    fn view(&self) -> Element<'_, view::Message> {
        // todo: add refs to cache?
        // todo: add performance benches
        view::execute::execution_layout(
            &self.form,
            &self.cache.from_list,
            &self.cache.to_list,
            &self.cache.target_list,
            self.cache.simulated_results.clone(),
            self.cache.executed_results.clone(),
        )
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::none()
    }

    fn load(&self) -> Command<Message> {
        Command::none()
    }
}
