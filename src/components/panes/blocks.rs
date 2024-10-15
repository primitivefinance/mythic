//! Pane for displaying subscribed blocks.

use iced::widget::{button, pane_grid, scrollable, text, Column};
use iced::{Center, Element, Fill, Size};

use super::Message;
use crate::components::system::ExcaliburContainer;

pub fn view_blocks<'a>(
    pane: pane_grid::Pane,
    total_panes: usize,
    is_pinned: bool,
    size: Size,
    block_number: u64,
) -> Element<'a, Message> {
    let button = |label, message| {
        button(text(label).width(Fill).align_x(Center).size(16))
            .width(Fill)
            .padding(8)
            .on_press(message)
    };

    let controls = Column::new()
        .push(button("Refresh", Message::Empty))
        .push_maybe(if total_panes > 1 && !is_pinned {
            Some(button("Close", Message::Close(pane)).style(button::danger))
        } else {
            None
        })
        .spacing(5)
        .max_width(160);

    let content = Column::new()
        .push(text(format!("{}", block_number)).size(24))
        .push(controls)
        .spacing(10)
        .align_x(Center);

    ExcaliburContainer::default()
        .build(scrollable(content))
        .center_y(Fill)
        .padding(5)
        .into()
}
