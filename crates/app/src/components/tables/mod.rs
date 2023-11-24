//! TableBuilder can be used to construct a table with different kinds of cells.

pub mod cells;
pub mod columns;
pub mod rows;

use iced::BorderRadius;

use self::{columns::ColumnBuilder, rows::RowBuilder};
use super::{
    containers::{BorderedContainer, TableColumnContainer},
    *,
};

pub struct TableBuilder<Message>
where
    Message: 'static + Default,
{
    columns: Vec<ColumnBuilder<Message>>,
    spacing_col: Option<Sizes>,
    spacing_row: Option<Sizes>,
    spacing_cell: Option<Sizes>,
    padding_col: Option<Sizes>,
    padding_row: Option<Sizes>,
    padding_cell: Option<Sizes>,
    padding_cell_internal: Option<Sizes>,
}

impl<Message> TableBuilder<Message>
where
    Message: 'static + Default,
{
    pub fn new() -> Self {
        Self {
            columns: vec![],
            spacing_col: None,
            spacing_row: None,
            spacing_cell: None,
            padding_col: None,
            padding_row: None,
            padding_cell: None,
            padding_cell_internal: None,
        }
    }

    pub fn spacing_col(mut self, spacing_col: Sizes) -> Self {
        self.spacing_col = Some(spacing_col);
        self
    }

    pub fn spacing_row(mut self, spacing_row: Sizes) -> Self {
        self.spacing_row = Some(spacing_row);
        self
    }

    pub fn spacing_cell(mut self, spacing_cell: Sizes) -> Self {
        self.spacing_cell = Some(spacing_cell);
        self
    }

    pub fn padding_col(mut self, padding_col: Sizes) -> Self {
        self.padding_col = Some(padding_col);
        self
    }

    pub fn padding_row(mut self, padding_row: Sizes) -> Self {
        self.padding_row = Some(padding_row);
        self
    }

    pub fn padding_cell(mut self, padding_cell: Sizes) -> Self {
        self.padding_cell = Some(padding_cell);
        self
    }

    pub fn padding_cell_internal(mut self, padding_cell_internal: Sizes) -> Self {
        self.padding_cell_internal = Some(padding_cell_internal);
        self
    }

    pub fn column(mut self, column: ColumnBuilder<Message>) -> Self {
        self.columns.push(column);
        self
    }

    pub fn build(self) -> Row<'static, Message> {
        let mut table = Row::new();

        for column in self.columns {
            // Specifies the spacing between rows in a column.
            // And the spacing between cells in a row.
            let column: Column<'static, Message> = column
                .spacing(self.spacing_row.unwrap_or_default())
                .spacing_cell(self.spacing_cell.unwrap_or_default())
                .padding_row(self.padding_row.unwrap_or_default())
                .padding_cell(self.padding_cell.unwrap_or_default())
                .padding_cell_internal(self.padding_cell_internal.unwrap_or_default())
                .into();
            table = table.push(column);
        }

        // Specifies spacing of columns in a table.
        table
            .spacing(self.spacing_col.unwrap_or_default())
            .padding(self.padding_col.unwrap_or_default())
    }
}

impl<'a, Message> From<TableBuilder<Message>> for Row<'a, Message>
where
    Message: 'static + Default,
{
    fn from(table: TableBuilder<Message>) -> Self {
        table.build()
    }
}

impl<'a, Message> From<TableBuilder<Message>> for Container<'a, Message>
where
    Message: 'static + Default,
{
    fn from(table: TableBuilder<Message>) -> Self {
        table_container(table)
    }
}

pub fn table_container<'a, Message>(table: TableBuilder<Message>) -> Container<'a, Message>
where
    Message: 'static + Default,
{
    Container::new(table.build())
}

impl<'a, Message> From<TableBuilder<Message>> for Element<'a, Message>
where
    Message: 'static + Default,
{
    fn from(table: TableBuilder<Message>) -> Self {
        table.build().into()
    }
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
