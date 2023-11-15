//! Views for executing transactions.

use super::{text, Column, Element, Message};

/// Panel for reviewing the transaction's state diffs.
pub fn review<'a>() -> Element<'a, Message> {
    Column::new()
        .push(text("Review"))
        .push(text("Review the transaction's state diffs."))
        .into()
}
