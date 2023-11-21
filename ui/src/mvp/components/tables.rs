use iced::BorderRadius;

use super::{
    containers::{BorderedContainer, TableColumnContainer},
    *,
};

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
