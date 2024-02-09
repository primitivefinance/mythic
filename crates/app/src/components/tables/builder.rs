//! TableBuilder can be used to construct a table with different kinds of cells.

use super::*;

pub struct TableBuilder<Msg>
where
    Msg: 'static + Default,
{
    columns: Vec<ColumnBuilder<Msg>>,
    spacing_col: Option<Sizes>,
    spacing_row: Option<Sizes>,
    spacing_cell: Option<Sizes>,
    padding_col: Option<Sizes>,
    padding_row: Option<Sizes>,
    padding_cell: Option<Sizes>,
    padding_cell_internal: Option<Sizes>,
}

impl<Msg> Default for TableBuilder<Msg>
where
    Msg: 'static + Default,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<Msg> TableBuilder<Msg>
where
    Msg: 'static + Default,
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

    pub fn update_cell(
        &mut self,
        table_col: usize,
        table_row: usize,
        cell_col: usize,
        cell_value: Option<String>,
    ) {
        if let Some(column) = self.columns.get_mut(table_col) {
            column.update_cell(table_row, cell_col, cell_value);
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

    pub fn column(mut self, column: ColumnBuilder<Msg>) -> Self {
        self.columns.push(column);
        self
    }

    pub fn build<'a>(self) -> Row<'a, Msg> {
        let mut table = Row::new();

        for column in self.columns {
            // Specifies the spacing between rows in a column.
            // And the spacing between cells in a row.
            let column: Column<'static, Msg> = column
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
