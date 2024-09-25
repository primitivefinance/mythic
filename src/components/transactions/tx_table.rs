use iced::widget::{Column, Container, Space};
use iced::{Element, Length};

use crate::components::styles::{Sizes, GRAY_600};
use crate::components::system::{label, ExcaliburColor, ExcaliburContainer};

#[derive(Debug, Clone, Default)]
pub struct TxHistory;

impl TxHistory {
    pub fn layout<'a, Message>(
        title: impl ToString,
        subtitle: impl ToString,
        table: impl Into<Element<'a, Message>>,
    ) -> Container<'a, Message>
    where
        Message: 'a + Clone,
    {
        ExcaliburContainer::default().build(
            Column::new()
                .spacing(Sizes::Md)
                .push(
                    Column::new()
                        .spacing(Sizes::Sm)
                        .push(label(subtitle).secondary().build())
                        .push(label(title).title1().build()),
                )
                .push(
                    ExcaliburContainer::default()
                        .light_border()
                        .build(Column::new().push(iced::widget::text("todo".to_string()))),
                ),
        )
    }

    pub fn separator<'a, Message>() -> Container<'a, Message>
    where
        Message: 'a,
    {
        ExcaliburContainer::default()
            .background(ExcaliburColor::Custom(GRAY_600))
            .build(Space::new(Length::Fill, 1.0))
            .width(Length::Fill)
    }
}
