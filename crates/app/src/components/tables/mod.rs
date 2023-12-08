//! todo: The performance of this table I am assuming is not very good,
//! because it will be re-regenerated every time we call it.
//! We should make a wrapper component that builds the table once and only
//! updates the values.
//! This will give us the desired structure of the table, without forcing
//! renders to happen repeatedly. Maybe we can make a wrapper component that
//! caches the table and only updates the values.
//! Or the builder can natively expose a way to update all the values...
//! To improve the performance of your table builder, you could consider using a
//! design pattern such as the Flyweight pattern. This pattern is used to
//! minimize memory usage or computational expenses by sharing as much as
//! possible with similar objects.
//!
//! In your case, you could create a Table struct
//! that holds the structure of the table (columns, rows, cells) and a separate
//! TableData struct that holds the actual data for the table. The Table struct
//! would only be built once, and the TableData could be updated as needed.

pub mod builder;
pub mod cells;
pub mod columns;
pub mod rows;

use self::{builder::TableBuilder, columns::ColumnBuilder, rows::RowBuilder};
use super::*;

/// Renders a simple dual column table with a label and value.
pub fn key_value_table<'a, Message>(
    headers: Vec<String>,
    data: Vec<(
        impl Into<Element<'static, Message>>,
        impl Into<Element<'static, Message>>,
    )>,
) -> TableBuilder<Message>
where
    Message: 'a + Default,
{
    TableBuilder::new().padding_cell(Sizes::Md).column(
        ColumnBuilder::new().headers(headers).rows(
            data.into_iter()
                .enumerate()
                .map(|(index, (label, value))| {
                    let bg = match index % 2 == 0 {
                        true => TABLE_ROW_1,
                        false => TABLE_ROW_2,
                    };
                    RowBuilder::new()
                        .style(move || CustomContainer::theme(Some(iced::Background::Color(bg))))
                        .cells(vec![
                            CellBuilder::new().child(label),
                            CellBuilder::new().child(value),
                        ])
                })
                .collect(),
        ),
    )
}
