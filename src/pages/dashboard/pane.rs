use iced::widget::{button, pane_grid, scrollable, text, Column, Container, Row};
use iced::{Element, Size};

#[derive(Clone, Copy)]
pub struct Pane {
    id: usize,
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
        button(text(label).width(iced::Length::Fill).size(16))
            .width(iced::Length::Fill)
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
        .spacing(5)
        .max_width(10);

    let content = Column::new()
        .push(text(format!("{}x{}", size.width, size.height)).size(24))
        .push(controls)
        .spacing(10);

    Container::new(scrollable(content))
        .center_y()
        .padding(5)
        .into()
}

pub fn view_controls<'a>(
    pane: pane_grid::Pane,
    total_panes: usize,
    is_pinned: bool,
    is_maximized: bool,
) -> Element<'a, super::Message> {
    let row = Row::new().push(if total_panes > 1 {
        let (content, message) = if is_maximized {
            ("Restore", super::Message::Restore)
        } else {
            ("Maximize", super::Message::Maximize(pane))
        };

        button(text(content).size(14)).padding(3).on_press(message)
    } else {
        button(text("n/a").width(iced::Length::Fill))
    });

    let close = button(text("Close").size(14))
        .padding(3)
        .on_press(super::Message::Close(pane));

    row.push(close).into()
}
