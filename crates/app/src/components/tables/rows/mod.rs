use super::{cells::CellBuilder, *};

/// Very basic table row with two cells, a label and text input.
pub fn dev_row<'a, Message>(
    value: Option<String>,
    on_change: impl Fn(Option<String>) -> Message + 'static,
) -> Row<'a, Message>
where
    Message: 'static + Default,
{
    let label_cell = CellBuilder::new().value(Some("Label".to_string()));
    let input_cell = CellBuilder::new().value(value).on_change(on_change);
    RowBuilder::new().cell(label_cell).cell(input_cell).build()
}

/// todo: support Padding customization
pub struct RowBuilder<Message>
where
    Message: Default,
{
    cells: Vec<CellBuilder<Message>>,
    spacing: Option<Sizes>,
    padding: Option<Sizes>,
    padding_cell: Option<Sizes>,
    padding_cell_internal: Option<Sizes>,
}

impl<Message> RowBuilder<Message>
where
    Message: Default,
{
    pub fn new() -> Self {
        Self {
            cells: vec![],
            spacing: None,
            padding: None,
            padding_cell: None,
            padding_cell_internal: None,
        }
    }

    pub fn spacing(mut self, spacing: Sizes) -> Self {
        self.spacing = Some(spacing);
        self
    }

    pub fn cell(mut self, cell: CellBuilder<Message>) -> Self {
        self.cells.push(cell);
        self
    }

    pub fn padding(mut self, padding: Sizes) -> Self {
        self.padding = Some(padding);
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

    pub fn build(self) -> Row<'static, Message>
    where
        Message: 'static + Default,
    {
        let mut row = Row::new();

        for cell in self.cells {
            let cell: Container<'static, Message> = cell
                .internal_padding(self.padding_cell_internal)
                .external_padding(self.padding_cell)
                .into();
            row = row.push(cell);
        }

        row.align_items(alignment::Alignment::Center)
            .spacing(self.spacing.unwrap_or_default())
            .padding(self.padding.unwrap_or_default())
    }
}

impl<'a, Message> From<RowBuilder<Message>> for Row<'a, Message>
where
    Message: 'static + Default,
{
    fn from(row: RowBuilder<Message>) -> Self {
        row.build()
    }
}

impl<'a, Message> From<RowBuilder<Message>> for Element<'a, Message>
where
    Message: 'static + Default,
{
    fn from(row: RowBuilder<Message>) -> Self {
        row.into()
    }
}
