//! Wrap/unwrap WETH

use iced::widget::{button, pane_grid, scrollable, text, text_input, Column, Row};
use iced::{Center, Element, Fill, Size};

use crate::components::system::ExcaliburContainer;

pub fn view_weth<'a>(
    pane: pane_grid::Pane,
    total_panes: usize,
    is_pinned: bool,
    size: Size,
    weth_address: &String,
    amount: &String,
) -> Element<'a, super::Message> {
    let button = |label, message| {
        button(text(label).width(Fill).align_x(Center).size(16))
            .width(Fill)
            .padding(8)
            .on_press(message)
    };

    let controls = Column::new()
        .push(
            text_input("Weth Address", weth_address)
                .on_input(move |x| super::Message::Update(pane, "weth_address".to_string(), x)),
        )
        .push(
            text_input("Amount", amount)
                .on_input(move |x| super::Message::Update(pane, "weth_amount".to_string(), x)),
        )
        .push(button("Submit", super::Message::SubmitForm(pane)))
        .push_maybe(if total_panes > 1 && !is_pinned {
            Some(button("Close", super::Message::Close(pane)).style(button::danger))
        } else {
            None
        })
        .spacing(5)
        .max_width(160);

    let content = Column::new()
        .push(text(format!("Weth")).size(24))
        .push(controls)
        .spacing(10)
        .align_x(Center);

    ExcaliburContainer::default()
        .build(scrollable(content))
        .center_y(Fill)
        .padding(5)
        .into()
}
