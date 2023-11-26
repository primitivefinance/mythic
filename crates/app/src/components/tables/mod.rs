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
pub mod data;
pub mod rows;

use std::rc::Rc;

use iced::BorderRadius;

use self::{builder::TableBuilder, columns::ColumnBuilder, rows::RowBuilder};
use super::{
    containers::{BorderedContainer, TableColumnContainer},
    *,
};

#[derive(Debug, Clone, Default)]
pub struct Asset {
    pub ticker: String,
    pub price: f64,
    pub selected: bool,
}

pub fn asset_selection_table<'a, Message>(
    assets: Vec<Asset>,
    on_select: impl Fn(Option<String>) -> Message + 'static,
) -> Container<'a, Message>
where
    Message: 'static + Default,
{
    let on_select = Rc::new(on_select);
    let rows: Vec<RowBuilder<Message>> = assets
        .into_iter()
        .map(|asset| {
            let ticker_cell = cells::CellBuilder::new().value(Some(asset.ticker.clone()));

            let price_cell = cells::CellBuilder::new().value(Some(asset.price.to_string()));

            let ticker = asset.ticker.clone();
            let on_select = Rc::clone(&on_select);
            let selected_cell = cells::CellBuilder::new()
                .checked(Some(asset.selected))
                .on_checkbox(move |_| on_select(Some(ticker.clone())));

            RowBuilder::new()
                .cell(ticker_cell)
                .cell(price_cell)
                .cell(selected_cell)
        })
        .collect();

    let col = ColumnBuilder::new().rows(rows);
    let table = TableBuilder::new()
        .column(col)
        .spacing_col(Sizes::Lg)
        .spacing_row(Sizes::Md);

    Container::new(table.build())
}

/// Renders a column in a summary table's row.
pub fn summary_column<'a>(value: String) -> Column<'a, Message> {
    Column::new()
        .push(
            Column::new()
                .push(h4(value.clone()))
                .padding(Sizes::Lg as u16),
        )
        .align_items(alignment::Alignment::Center)
        .width(Length::FillPortion(2))
}

pub fn copyable_summary_column<'a>(value: String) -> Column<'a, Message> {
    Column::new()
        .push(copyable_text(
            Column::new()
                .push(h4(value.clone()))
                .padding(Sizes::Lg as u16),
            value,
        ))
        .align_items(alignment::Alignment::Center)
        .width(Length::FillPortion(2))
}

/// Renders a row in a summary table.
/// If top row, render with top-left border radius.
/// If bottom row, render with bottom-left border radius.
/// Otherwise, render with no border radius.
/// todo: border radius is affected by being an inner border, doesn't look the
/// greatest unfortunately.
pub fn summary_row<'a>(
    columns: Vec<Column<'a, Message>>,
    row_position: usize,
    total_rows: usize,
) -> Container<'a, Message> {
    let col_radius: BorderRadius = match row_position {
        0 => [5.0, 0.0, 0.0, 0.0].into(),
        _ if row_position == total_rows - 1 => [0.0, 0.0, 0.0, 5.0].into(),
        _ => [0.0, 0.0, 0.0, 0.0].into(),
    };

    let col_radius = match total_rows {
        1 => [5.0, 0.0, 0.0, 5.0].into(),
        _ => col_radius,
    };

    // Edit the first column by wrapping it with a InfoContainer
    let mut columns = columns;
    let first_column = columns.remove(0);
    let first_column = Container::new(first_column)
        .style(TableColumnContainer::theme_with_border_radius(col_radius))
        .width(Length::Fill);
    columns.insert(0, Column::new().push(first_column).width(Length::Fill));

    // If top row, top left + right border radius.
    // If bottom row, bottom left + right border radius.
    // Otherwise, no border radius.
    let row_radius: BorderRadius = match row_position {
        0 => [5.0, 5.0, 0.0, 0.0].into(),
        _ if row_position == total_rows - 1 => [0.0, 0.0, 5.0, 5.0].into(),
        _ => [0.0, 0.0, 0.0, 0.0].into(),
    };

    // Override radius if only one row.
    let row_radius = match total_rows {
        1 => 5.0.into(),
        _ => row_radius,
    };

    Container::new(
        Row::with_children(
            columns
                .into_iter()
                .map(|c| c.into())
                .collect::<Vec<Element<'a, Message>>>(),
        )
        .align_items(alignment::Alignment::Center)
        .width(Length::Fill),
    )
    .style(BorderedContainer::theme_with_border_radius(row_radius))
    .into()
}

pub fn summary_table<'a>(values: Vec<(String, String)>) -> Container<'a, Message> {
    // If values has no values, just render centered text in the container "No
    // changes."
    if values.is_empty() {
        return Container::new(
            Row::new()
                .push(
                    Column::new()
                        .push(text("No items to show."))
                        .align_items(alignment::Alignment::Center)
                        .padding(Sizes::Md as u16),
                )
                .align_items(alignment::Alignment::Center),
        )
        .center_x()
        .center_y()
        .style(BorderedContainer::theme());
    }

    let mut rows: Vec<Element<'a, Message>> = vec![];

    let total_rows = values.len();
    for (i, (label, value)) in values.into_iter().enumerate() {
        let columns: Vec<Column<'a, Message>> =
            vec![summary_column(label), copyable_summary_column(value)];
        let row = summary_row(columns, i, total_rows);
        rows.push(row.into());
    }

    Container::new(Column::with_children(rows))
        .style(BorderedContainer::theme())
        .into()
}

/// Renders a very basic table with one row.
#[allow(dead_code)]
pub fn dev_table<'a>(
    value: Option<String>,
    select: Option<String>,
    checkbox: bool,
) -> Container<'a, Message> {
    let table_rows = RowBuilder::new()
        .cell(
            cells::CellBuilder::new()
                .value(Some("Input example".to_string()))
                .style(TableColumnContainer::theme),
        )
        .cell(
            cells::CellBuilder::new()
                .value(value.clone())
                .placeholder(Some("Placeholder 1".to_string()))
                .style(BorderedContainer::theme)
                .on_change(|value| Message::Developer(developer::Message::OnChange(value))),
        );

    let table_rows_2 = RowBuilder::new()
        .cell(
            cells::CellBuilder::new()
                .value(Some("Input & Select".to_string()))
                .style(TableColumnContainer::theme),
        )
        .cell(
            cells::CellBuilder::new()
                .value(value.clone())
                .placeholder(Some("Placeholder 2".to_string()))
                .style(BorderedContainer::theme)
                .on_change(|value| Message::Developer(developer::Message::OnChange(value))),
        )
        .cell(
            cells::CellBuilder::new()
                .on_select(|value| Message::Developer(developer::Message::OnSelect(value)))
                .options(Some(vec![
                    "Option 1".to_string(),
                    "Option 2".to_string(),
                    "Option 3".to_string(),
                ]))
                .selected(select.clone()),
        );

    let table_row_3 = RowBuilder::new().cell(
        cells::CellBuilder::new()
            .checked(Some(checkbox))
            .on_checkbox(|value| Message::Developer(developer::Message::OnCheckbox(value)))
            .style(TableColumnContainer::theme),
    );

    let col = ColumnBuilder::new()
        .row(table_rows)
        .row(table_rows_2)
        .row(table_row_3);

    let table = TableBuilder::new()
        .column(col)
        .spacing_col(Sizes::Lg)
        .spacing_row(Sizes::Md)
        .spacing_cell(Sizes::Sm)
        .padding_col(Sizes::Lg)
        .padding_row(Sizes::Md)
        .padding_cell(Sizes::Sm)
        .build();

    Container::new(table)
        .max_height(500.0)
        .style(BorderedContainer::theme())
        .into()
}

/// Renders a simple dual column table with a label and value.
pub fn key_value_table<'a, Message>(
    headers: Vec<String>,
    data: Vec<(String, String)>,
) -> TableBuilder<Message>
where
    Message: 'a + Default,
{
    TableBuilder::new().padding_cell(Sizes::Md).column(
        ColumnBuilder::new().headers(headers).rows(
            data.iter()
                .map(|(label, value)| {
                    RowBuilder::new()
                        .style(|| CustomContainer::theme(Some(iced::Background::Color(GRAY_500))))
                        .cells(vec![
                            CellBuilder::new().value(Some(label.clone())),
                            CellBuilder::new().value(Some(value.clone())).style(|| {
                                CustomContainer::theme(Some(iced::Background::Color(GRAY_400)))
                            }),
                        ])
                })
                .collect(),
        ),
    )
}
