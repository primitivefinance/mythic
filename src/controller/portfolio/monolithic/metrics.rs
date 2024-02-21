use super::*;
use crate::components::system::{ExcaliburContainer, ExcaliburText};

pub struct Metrics;

impl Metrics {
    pub fn layout<'a, Message>(
        position_title: ExcaliburText,
        external_price: ExcaliburText,
        external_price_subtext: ExcaliburText,
        aum: ExcaliburText,
        health: ExcaliburText,
    ) -> Container<'a, Message>
    where
        Message: 'static + Clone + Default,
    {
        ExcaliburContainer::default().build(
            Column::new()
                .spacing(Sizes::Md)
                .push(
                    Column::new()
                        .spacing(Sizes::Sm)
                        .push(label("Active position").secondary().build())
                        .push(position_title),
                )
                .push(
                    Row::new()
                        .spacing(Sizes::Md)
                        .align_items(alignment::Alignment::Center)
                        .push(Self::item(
                            label("Price").secondary(),
                            external_price,
                            external_price_subtext.tertiary(),
                        ))
                        .push(Self::item(
                            label("AUM").secondary(),
                            aum,
                            label("USD").tertiary(),
                        ))
                        .push(Self::item(
                            label("Health").secondary(),
                            health,
                            label("rAUM / tAUM").tertiary(),
                        )),
                ),
        )
    }

    pub fn item<'a, Message>(
        title: ExcaliburText,
        value: ExcaliburText,
        caption: ExcaliburText,
    ) -> Container<'a, Message>
    where
        Message: 'a + Clone + Default,
    {
        ExcaliburContainer::default().build(
            Column::new()
                .push(title)
                .push(
                    ExcaliburContainer::default()
                        .light_border()
                        .middle_top()
                        .round(Sizes::Sm)
                        .build(
                            Row::new()
                                .width(Length::Fill)
                                .align_items(alignment::Alignment::Center)
                                .push(Column::new().push(value))
                                .push(
                                    Column::new()
                                        .align_items(alignment::Alignment::End)
                                        .push(caption),
                                )
                                .spacing(Sizes::Sm),
                        )
                        .padding(Sizes::Lg),
                )
                .spacing(Sizes::Sm),
        )
    }
}
