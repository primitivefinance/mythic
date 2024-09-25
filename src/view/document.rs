//! Layout for the application.
use iced::alignment;
use iced::widget::{Column, Container, Row};
use iced::{Element, Fill, Length};

use super::header;
use crate::components::system::{ExcaliburColor, ExcaliburContainer};
use crate::pages::Lifecycle;
use crate::view::ViewMessage;

pub fn body<'a, T: Into<Element<'a, ViewMessage>>>(
    header: &'a header::Header,
    main: T,
) -> Element<'a, ViewMessage> {
    Container::new(
        Row::new()
            .push(
                Column::new()
                    .push(header.view())
                    .width(Length::FillPortion(1)),
            )
            .push(
                Column::new()
                    .push(page_layout(main))
                    .width(Length::FillPortion(5)),
            ),
    )
    .style(move |_theme| {
        ExcaliburContainer::default()
            .background(ExcaliburColor::Background1)
            .theme()
    })
    .width(Fill)
    .height(Fill)
    .center_x(Fill)
    .center_y(Fill)
    .into()
}

pub fn page_layout<'a, T: Into<Element<'a, ViewMessage>>>(content: T) -> Element<'a, ViewMessage> {
    Container::new(content)
        .center_x(Fill)
        .center_y(Fill)
        .align_x(alignment::Horizontal::Center)
        .align_y(alignment::Vertical::Center)
        .width(Fill)
        .height(Fill)
        .style(move |_theme| ExcaliburContainer::default().middle_bottom().theme())
        .into()
}
