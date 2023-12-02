//! View component for rendering different data that is actively monitored.

use super::*;

/// Renders a single piece of labeled data in a container with a panel
/// background and padding.
pub fn labeled_data_card<'a, Message>(
    label: String,
    data: String,
    _max_width: u16,
) -> Element<'a, Message>
where
    Message: 'a,
{
    let mut content = Column::new()
        .push(labeled_data(label, data))
        .width(Length::Fixed(100.0));
    content = content.spacing(Sizes::Sm as u16);
    Card::new(container(content))
        .padding(Sizes::Md as u16)
        .into()
}

/// Renders a group of labeled data cards in a row with a maximum amount of
/// elements per row. If the total amount of elements exceeds the maximum, it
/// will push a new row inside the column. There is a group label rendered above
/// the rows.
pub fn labeled_data_cards<'a, Message>(
    data: Vec<(String, String)>,
    max_elements: usize,
) -> Element<'a, Message>
where
    Message: 'a,
{
    let mut content = Column::new();

    let mut row = Row::new().spacing(Sizes::Sm as u16);
    let mut i = 0;
    for (label, data) in data {
        row = row.push(labeled_data_card(label, data, 200));
        i += 1;
        if i == max_elements {
            content = content.push(row);
            row = Row::new().spacing(Sizes::Sm as u16);
            i = 0;
        }
    }
    content = content.push(row);
    content.spacing(16).into()
}
