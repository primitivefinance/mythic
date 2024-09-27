use std::collections::HashMap;
use std::sync::Arc;

use alloy::rpc::types::TransactionReceipt;
use iced::widget::pane_grid;
use iced::Task;

use crate::blockchain::AlloyClient;
use crate::components::panes::handler::FormTransactionHandler;
use crate::contracts::weth::WethWrapHandler;

pub mod basic;
pub mod blocks;
pub mod handler;
pub mod weth;

#[derive(Debug, Clone, Default)]
pub enum Message {
    #[default]
    Empty,

    Split(pane_grid::Axis, pane_grid::Pane),
    SplitFocused(pane_grid::Axis),
    FocusAdjacent(pane_grid::Direction),
    Clicked(pane_grid::Pane),
    Dragged(pane_grid::DragEvent),
    Resized(pane_grid::ResizeEvent),
    TogglePin(pane_grid::Pane),
    Maximize(pane_grid::Pane),
    Restore,
    Close(pane_grid::Pane),
    CloseFocused,

    Open(PaneType),
    Update(pane_grid::Pane, String, String),
    SubmitForm(pane_grid::Pane),
    TransactionResult(Result<TransactionReceipt, PaneError>),
}

#[derive(Debug, Clone)]
pub enum PaneError {
    Empty,
    Custom(String),
    Tx(String),
}

#[derive(Debug, Clone)]
pub struct Form {
    pub fields: HashMap<String, String>,
}

#[derive(Debug, Clone, Copy)]
pub enum PaneType {
    Basic,
    Blocks,
    Weth,
}

#[derive(Debug, Clone)]
pub struct Pane {
    pub id: usize,
    pub is_pinned: bool,
    pub pane_type: PaneType,

    pub data: HashMap<String, Form>,
}

impl Pane {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            is_pinned: false,
            pane_type: PaneType::Basic,
            data: HashMap::new(),
        }
    }

    pub fn new_with_type(id: usize, pane_type: PaneType) -> Self {
        Self {
            id,
            is_pinned: false,
            pane_type,
            data: HashMap::new(),
        }
    }

    pub fn set(&mut self, form_id: &str, field: String, value: String) {
        self.data
            .entry(form_id.to_string())
            .or_insert_with(|| Form {
                fields: HashMap::new(),
            })
            .fields
            .insert(field, value);
    }

    pub fn get(&self, form_id: &str, field: &str) -> Option<&String> {
        self.data
            .get(form_id)
            .and_then(|form| form.fields.get(field))
    }

    pub fn submit_form(&self, form_id: &str, client: Arc<AlloyClient>) -> Task<Message> {
        let form = match self.data.get(form_id) {
            Some(form) => form,
            None => {
                tracing::error!("Failed to submit form for pane, form not found");
                return Task::none();
            }
        };

        match self.pane_type {
            PaneType::Weth => {
                tracing::debug!("Submitting WETH form");
                WethWrapHandler::handle_transaction(client, &form.fields)
            }
            _ => Task::none(),
        }
    }
}
