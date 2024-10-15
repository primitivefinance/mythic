//! Layout for the application.
use iced::alignment;
use iced::widget::Column;
use iced::{Element, Fill};

use super::connect;
use super::header;
use crate::components::system::ExcaliburContainer;
use crate::pages::Lifecycle;
use crate::view::ViewMessage;

pub fn body<'a, T: Into<Element<'a, ViewMessage>>>(
    connect: &'a connect::Connect,
    header: &'a header::Header,
    main: T,
) -> Element<'a, ViewMessage> {
    ExcaliburContainer::default()
        .bottom()
        .build(
            Column::new()
                .push(header.view())
                .push(Column::new().push(page_layout(main)).width(Fill))
                .push(connect.view()),
        )
        .width(Fill)
        .height(Fill)
        .center_x(Fill)
        .center_y(Fill)
        .into()
}

pub fn page_layout<'a, T: Into<Element<'a, ViewMessage>>>(content: T) -> Element<'a, ViewMessage> {
    ExcaliburContainer::default()
        .middle_bottom()
        .build(content)
        .center_x(Fill)
        .center_y(Fill)
        .align_x(alignment::Horizontal::Center)
        .align_y(alignment::Vertical::Center)
        .width(Fill)
        .height(Fill)
        .into()
}
