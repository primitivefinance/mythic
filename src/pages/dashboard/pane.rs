use iced::widget::{button, pane_grid, scrollable, text, Column, Container, Row};
use iced::{Center, Element, Fill, Size};

#[derive(Clone, Copy)]
pub struct Pane {
    pub id: usize,
    pub is_pinned: bool,
}

impl Pane {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            is_pinned: false,
        }
    }
}

pub fn view_content<'a>(
    pane: pane_grid::Pane,
    total_panes: usize,
    is_pinned: bool,
    size: Size,
) -> Element<'a, super::Message> {
    let button = |label, message| {
        button(text(label).width(Fill).align_x(Center).size(16))
            .width(Fill)
            .padding(8)
            .on_press(message)
    };

    let controls = Column::new()
        .push(button(
            "Spit horizontal",
            super::Message::Split(pane_grid::Axis::Horizontal, pane),
        ))
        .push(button(
            "Spit vertical",
            super::Message::Split(pane_grid::Axis::Vertical, pane),
        ))
        .push_maybe(if total_panes > 1 && !is_pinned {
            Some(button("Close", super::Message::Close(pane)).style(button::danger))
        } else {
            None
        })
        .spacing(5)
        .max_width(160);

    let content = Column::new()
        .push(text(format!("{}x{}", size.width, size.height)).size(24))
        .push(controls)
        .spacing(10)
        .align_x(Center);

    Container::new(scrollable(content))
        .center_y(Fill)
        .padding(5)
        .into()
}

pub fn view_controls<'a>(
    pane: pane_grid::Pane,
    total_panes: usize,
    is_pinned: bool,
    is_maximized: bool,
) -> Element<'a, super::Message> {
    let row = Row::new()
        .push_maybe(if total_panes > 1 {
            let (content, message) = if is_maximized {
                ("Restore", super::Message::Restore)
            } else {
                ("Maximize", super::Message::Maximize(pane))
            };

            Some(
                button(text(content).size(14))
                    .style(button::secondary)
                    .padding(3)
                    .on_press(message),
            )
        } else {
            None
        })
        .spacing(5);

    let close = button(text("Close").size(14))
        .style(button::danger)
        .padding(3)
        .on_press_maybe(if total_panes > 1 && !is_pinned {
            Some(super::Message::Close(pane))
        } else {
            None
        });

    row.push(close).into()
}
