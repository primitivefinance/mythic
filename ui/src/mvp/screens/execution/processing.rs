//! Handles all transaction related logic for any network including simulations.

use std::sync::Arc;

use tokio::sync::Mutex;

use super::{api::scroll::*, send::handle_simulate_scroll, *};

pub type StorageDiffs = HashMap<StorageValue, (Option<StorageValue>, Option<StorageValue>)>;

// type alias for operations in this module.
type Slots = StorageMap<StorageValue, StorageValue>;

#[derive(Default)]
pub struct StorageSlots {
    pub storage_before: StorageMap<StorageValue, StorageValue>,
    pub storage_after: StorageMap<StorageValue, StorageValue>,
    pub differences: Option<StorageDiffs>,
}

impl StorageSlots {
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

#[derive(Default)]
pub struct Processing {
    pub forker: Arc<Mutex<Forker>>,
    pub unsealed: UnsealedTransaction,
    pub sealed: Option<Scroll>,
    pub simulated: StorageSlots,
    pub executed: StorageSlots,
}

impl Processing {
    pub fn new(forker: Forker) -> Self {
        Self {
            forker: Arc::new(Mutex::new(forker)),
            ..Default::default()
        }
    }

    pub fn simulate(&mut self) -> Command<app::Message> {
        if self.unsealed.method.is_none() {
            self.unsealed = self.unsealed.clone().method("increment");
        }

        self.sealed = Some(self.unsealed.clone().seal());

        let scroll = self.sealed.clone().unwrap();
        let forker = self.forker.clone();

        Command::perform(handle_simulate_scroll(scroll, forker), |res| {
            app::Execution::Simulated(res).into()
        })
    }

    pub fn complete_simulation(&mut self, scroll: Scroll) {
        self.sealed = Some(scroll.clone());
    }

    pub fn execute(&mut self) -> Command<app::Message> {
        if self.sealed.is_none() {
            return Command::none();
        }

        let scroll = self.sealed.clone().unwrap();
        let forker = self.forker.clone();

        Command::perform(send::handle_execute_scroll(scroll, forker), |res| {
            app::Execution::Executed(res).into()
        })
    }

    pub fn complete_execution(&mut self, scroll: Scroll) {
        self.sealed = Some(scroll.clone());
    }

    pub fn reset(&mut self) {
        self.unsealed = UnsealedTransaction::default();
        self.sealed = None;
        self.simulated = StorageSlots::default();
        self.executed = StorageSlots::default();

        // todo: add forker reset fn?
    }

    /// Gets the storage slots before and after for the target account in the
    /// sealed scroll.
    pub fn try_load(&self) -> anyhow::Result<(Slots, Slots), anyhow::Error> {
        let scroll: Scroll = self.sealed.clone().unwrap();
        let account = scroll.payload.target.clone();

        let before: Slots = scroll.try_storage_before(account)?;
        let after: Slots = scroll.try_storage_after(account)?;

        Ok((before, after))
    }

    /// Edits the simulated before and after storage slots for the target of the
    /// sealed scroll.
    pub fn try_load_simulated(&mut self) -> anyhow::Result<(), anyhow::Error> {
        let (before, after) = self.try_load()?;
        self.simulated.storage_before = before;
        self.simulated.storage_after = after;
        self.simulated.apply_diff();

        Ok(())
    }

    /// Edits the executed before and after storage slots for the target of the
    /// sealed scroll.
    pub fn try_load_executed(&mut self) -> anyhow::Result<(), anyhow::Error> {
        let (before, after) = self.try_load()?;
        self.executed.storage_before = before;
        self.executed.storage_after = after;
        self.executed.apply_diff();

        Ok(())
    }
}
