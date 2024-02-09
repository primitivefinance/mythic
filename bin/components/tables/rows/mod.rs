use super::{cells::CellBuilder, *};
use crate::controller::portfolio::monolithic::tx_history::TxHistory;

/// Very basic table row with two cells, a label and text input.
#[allow(dead_code)]
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
    Message: Default + 'static,
{
    cells: Vec<CellBuilder<Message>>,
    spacing: Option<Sizes>,
    padding: Option<Sizes>,
    padding_cell: Option<Sizes>,
    padding_cell_internal: Option<Sizes>,
    containerize: Box<dyn Fn(Row<'static, Message>) -> Row<'static, Message>>,
    border_bottom: Option<iced::theme::Container>,
    is_last: bool,
}

impl<Message> Clone for RowBuilder<Message>
where
    Message: Default + 'static,
{
    fn clone(&self) -> Self {
        Self {
            cells: vec![],
            spacing: self.spacing,
            padding: self.padding,
            padding_cell: self.padding_cell,
            padding_cell_internal: self.padding_cell_internal,
            containerize: Box::new(|e| e),
            border_bottom: None,
            is_last: false,
        }
    }
}

impl<Message> Default for RowBuilder<Message>
where
    Message: Default,
{
    fn default() -> Self {
        Self::new()
    }
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
            // Returns self.
            containerize: Box::new(|e| e),
            border_bottom: None,
            is_last: false,
        }
    }

    pub fn last_row(mut self, is_last: bool) -> Self {
        self.is_last = is_last;
        self
    }

    pub fn border_bottom(mut self, border_bottom: iced::theme::Container) -> Self {
        self.border_bottom = Some(border_bottom);
        self
    }

    pub fn style(mut self, style: impl Fn() -> iced::theme::Container + 'static) -> Self {
        self.containerize = Box::new(move |e| Row::new().push(Container::new(e).style(style())));
        self
    }

    pub fn update_cell(&mut self, index: usize, value: Option<String>) {
        if let Some(cell) = self.cells.get_mut(index) {
            cell.update_value(value);
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

    pub fn cells(mut self, cells: Vec<CellBuilder<Message>>) -> Self {
        self.cells = cells;
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

        row = row
            .align_items(alignment::Alignment::Center)
            .spacing(self.spacing.unwrap_or_default())
            .padding(self.padding.unwrap_or_default());

        if let (Some(_border_bottom), false) = (self.border_bottom, self.is_last) {
            row = Row::new()
                .push(Column::new().push(row).push(TxHistory::separator()))
                .align_items(alignment::Alignment::Center)
                .spacing(self.spacing.unwrap_or_default())
                .padding(self.padding.unwrap_or_default());
        }

        (self.containerize)(row)
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
