//! Table data is stored separately from the table itself. This allows us to
//! update the values instead of regenerate the entire structure of the table.

use super::*;

/// Simple table data storage type.
type TableData2D = Vec<Vec<TableValue>>;

/// Table value primitive. This is the value that is stored in the table.
pub struct TableValue {
    pub inner: String,
}

/// Custom table allows the data to be adjusted while
/// avoiding the table from re-rendering completely.
/// This is a performance optimization.
///
/// To get this working, the table is built once and references the location
/// of the data instead of the data itself.
pub struct CustomTable<Message>
where
    Message: Default + 'static,
{
    data: TableData2D,
    builder: TableBuilder<Message>,
    last_table: Element<'static, Message>,
}

impl<Message> CustomTable<Message>
where
    Message: Default + 'static,
{
    /// Create a new custom table.
    pub fn new() -> Self {
        Self {
            data: vec![],
            builder: TableBuilder::new(),
            last_table: Container::new(Text::new("")).into(),
        }
    }

    /// Update the value of a cell.
    pub fn update_cell(&mut self, row: usize, column: usize, value: String) {
        self.data[row][column] = TableValue { inner: value };
    }

    /// Get the value of a cell.
    pub fn get_cell(&self, row: usize, column: usize) -> &str {
        &self.data[row][column].inner
    }

    /// Get the number of rows in the table.
    pub fn rows(&self) -> usize {
        self.data.len()
    }

    /// Get the number of columns in the table.
    pub fn columns(&self) -> usize {
        self.data[0].len()
    }

    pub fn refresh(&mut self) {
        self.last_table = self.build();
    }

    pub fn build(&self) -> Element<'static, Message>
    where
        Message: 'static + Default,
    {
        self.builder.build().into()
    }

    /// Render the table.
    pub fn view<'a>(&'a self) -> Container<'a, Message>
    where
        Message: 'static + Default,
    {
        Container::new(self.last_table)
    }
}
