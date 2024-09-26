use iced::widget::{button, pane_grid, scrollable, text, Column, Container, Row};
use iced::{Center, Element, Fill, Size};

use crate::components::system::ExcaliburContainer;

pub mod basic;
pub mod blocks;

#[derive(Debug, Clone, Copy, Default)]
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
}

#[derive(Debug, Clone, Copy)]
pub enum PaneType {
    Basic,
    Blocks,
}

#[derive(Clone, Copy)]
pub struct Pane {
    pub id: usize,
    pub is_pinned: bool,
    pub pane_type: PaneType,
}

impl Pane {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            is_pinned: false,
            pane_type: PaneType::Basic,
        }
    }

    pub fn new_with_type(id: usize, pane_type: PaneType) -> Self {
        Self {
            id,
            is_pinned: false,
            pane_type,
        }
    }
}
