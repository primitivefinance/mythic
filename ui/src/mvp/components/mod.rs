pub mod button;

use button::*;
use iced::Color;

// These components should return View messages.
use super::{view::Message, *};

/// Renders a gray text label in lowercase.
pub fn label_item<'a>(t: String) -> Element<'a, Message> {
    let content = t.to_lowercase();

    text(content)
        .size(16)
        .style(Color::from_rgb(0.5, 0.5, 0.5))
        .into()
}

/// Renders a column with a label and an element.
pub fn labeled<'a, T: Into<Element<'a, Message>>>(
    label: String,
    element: T,
) -> Element<'a, Message> {
    let mut content = Column::new().push(label_item(label)).push(element.into());
    content = content.spacing(8);
    content.into()
}

/// Renders a row of labeled controls, where each control has a label.
pub fn labeled_controls<'a, T: Into<Element<'a, Message>>>(
    controls: Vec<(String, T)>,
) -> Element<'a, Message> {
    let mut content = Row::new();
    for (label, control) in controls {
        content = content.push(labeled(label, control));
    }
    content.spacing(4).into()
}

/// Renders a column with a label and a piece of data with the DAGGERSQUARE
/// font.
pub fn labeled_data<'a>(label: String, data: String) -> Element<'a, Message> {
    let mut content = Column::new()
        .push(label_item(label))
        .push(text(data).font(FONT_DAGGERSQUARE).size(20));
    content = content.spacing(8);
    content.into()
}

/// Renders a nice blue button.
pub fn action_button<'a>(label: String) -> iced::widget::Button<'a, Message> {
    let content = text(label)
        .size(16)
        .horizontal_alignment(iced::alignment::Horizontal::Center)
        .vertical_alignment(iced::alignment::Vertical::Center)
        .style(Color::WHITE);
    let action_button_style = CustomButtonStyle::new()
        .border_radius(5.0.into())
        .background_color(Color::from_rgb8(35, 88, 226))
        .hovered()
        .border_radius(5.0.into())
        .background_color(Color::from_rgb8(88, 135, 255))
        .pressed()
        .border_radius(5.0.into())
        .background_color(Color::from_rgb8(11, 63, 197));
    button(content).style(action_button_style.as_custom())
}

/// Container that groups actions or settings with a label and a row of
/// controls.
pub fn controls_container<'a, T: Into<Element<'a, Message>>>(
    label: String,
    actions: Vec<T>,
) -> Element<'a, Message> {
    let mut content = Column::new().push(label_item(label));
    let mut row = Row::new().spacing(4);
    for action in actions {
        row = row.push(action.into());
    }
    content = content.push(row);
    content.spacing(8).into()
}

/// Containers that groups multiple labeled data pieces under a label
pub fn labeled_data_container<'a>(
    label: String,
    data: Vec<(String, String)>,
    max_elements: usize,
) -> Element<'a, Message> {
    let mut content = Column::new().push(label_item(label));
    content = content.push(labeled_data_row(data, max_elements));
    content.into()
}

/// Renders a row of labeled data elements using labeled_data. Specify the
/// maximum amount of elements in the row, if the total amount of elements
/// exceeds the value, it will push a new row to the column.
pub fn labeled_data_row<'a>(
    label_data: Vec<(String, String)>,
    max_elements: usize,
) -> Element<'a, Message> {
    let mut content = Column::new();
    let mut row = Row::new().spacing(4);
    let mut i = 0;
    for (label, data) in label_data {
        row = row.push(labeled_data(label, data));
        i += 1;
        if i == max_elements {
            content = content.push(row);
            row = Row::new().spacing(4);
            i = 0;
        }
    }
    content = content.push(row);
    content.spacing(8).into()
}
