use super::{cells::CellBuilder, rows::RowBuilder, *};

pub struct ColumnBuilder<Message>
where
    Message: 'static + Default,
{
    rows: Vec<RowBuilder<Message>>,
    headers: Vec<String>,
    spacing: Option<Sizes>,
    spacing_cell: Option<Sizes>,
    padding: Option<Sizes>,
    padding_row: Option<Sizes>,
    padding_cell: Option<Sizes>,
    padding_cell_internal: Option<Sizes>,
}

impl<Message> Default for ColumnBuilder<Message>
where
    Message: 'static + Default,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<Message> ColumnBuilder<Message>
where
    Message: 'static + Default,
{
    pub fn new() -> Self {
        Self {
            rows: vec![],
            headers: vec![],
            spacing: None,
            spacing_cell: None,
            padding: None,
            padding_row: None,
            padding_cell: None,
            padding_cell_internal: None,
        }
    }

    pub fn headers(mut self, headers: Vec<String>) -> Self {
        self.headers = headers;
        self
    }

    pub fn update_cell(&mut self, row: usize, column: usize, value: Option<String>) {
        if let Some(row) = self.rows.get_mut(row) {
            row.update_cell(column, value);
        }
    }

    pub fn spacing(mut self, spacing: Sizes) -> Self {
        self.spacing = Some(spacing);
        self
    }

    pub fn spacing_cell(mut self, spacing_cell: Sizes) -> Self {
        self.spacing_cell = Some(spacing_cell);
        self
    }

    pub fn padding(mut self, padding: Sizes) -> Self {
        self.padding = Some(padding);
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

    pub fn row(mut self, row: RowBuilder<Message>) -> Self {
        self.rows.push(row);
        self
    }

    pub fn rows(mut self, rows: Vec<RowBuilder<Message>>) -> Self {
        self.rows = rows;
        self
    }

    /// Handles spacing of all child elements.
    pub fn build(self) -> Column<'static, Message> {
        let mut column = Column::new();

        // Add the headers first.
        if !self.headers.is_empty() {
            let row: Row<'static, Message> = RowBuilder::new()
                .cells(
                    self.headers
                        .into_iter()
                        .map(|header| {
                            CellBuilder::new()
                                .child(label_item(header))
                                .internal_padding(self.padding_cell_internal)
                                .external_padding(self.padding_cell)
                        })
                        .collect(),
                )
                .spacing(self.spacing.unwrap_or_default())
                .padding(self.padding_row.unwrap_or_default())
                .into();
            column = column.push(row);
        }

        // Specifies the spacing between cells in a row.
        for row in self.rows {
            let row: Row<'static, Message> = row
                .spacing(self.spacing_cell.unwrap_or_default())
                .padding(self.padding_row.unwrap_or_default())
                .padding_cell(self.padding_cell.unwrap_or_default())
                .padding_cell_internal(self.padding_cell_internal.unwrap_or_default())
                .into();
            column = column.push(row);
        }

        // Specifies spacing of rows.
        column
            .align_items(alignment::Alignment::Center)
            .spacing(self.spacing.unwrap_or_default())
            .padding(self.padding.unwrap_or_default())
    }
}

impl<'a, Message> From<ColumnBuilder<Message>> for Column<'a, Message>
where
    Message: 'static + Default,
{
    fn from(table: ColumnBuilder<Message>) -> Self {
        table.build()
    }
}

impl<'a, Message> From<ColumnBuilder<Message>> for Element<'a, Message>
where
    Message: 'static + Default,
{
    fn from(table: ColumnBuilder<Message>) -> Self {
        table.into()
    }
}
