use datatypes::portfolio::position::{Position, Positions};
use iced::{
    widget::{image, Image, Space},
    Padding,
};

use super::*;
use crate::{
    components::system::{ExcaliburButton, ExcaliburColor, ExcaliburContainer},
    model::portfolio::AlloyAddress,
};

pub struct Inventory;

impl Inventory {
    pub fn layout<'a, Message>(
        positions: Positions,
        logos: Vec<String>,
        on_allocate: Option<Message>,
        on_select_position: impl Fn(AlloyAddress) -> Message,
    ) -> Container<'a, Message>
    where
        Message: 'static + Clone + Default,
    {
        let positions_row = Row::with_children(
            positions
                .0
                .iter()
                .enumerate()
                .map(|(i, x)| {
                    ExcaliburButton::new()
                        .transparent()
                        .build(Self::position_layout::<Message>((
                            x.clone(),
                            image(logos[i].clone()),
                        )))
                        .on_press(on_select_position(x.asset.address.clone()).into())
                        .into()
                })
                .collect::<Vec<_>>(),
        )
        .spacing(Sizes::Lg);

        let allocated_positions = Row::with_children(
            positions
                .0
                .iter()
                .enumerate()
                .map(|(i, x)| {
                    ExcaliburButton::new()
                        .transparent()
                        .build(Self::position_layout::<Message>((
                            x.clone(),
                            image(logos[i].clone()),
                        )))
                        .on_press(on_select_position(x.asset.address.clone()).into())
                        .into()
                })
                .collect::<Vec<_>>(),
        )
        .spacing(Sizes::Lg);

        ExcaliburContainer::default()
            .middle_top()
            .round(Sizes::Sm)
            .build(
                Column::new()
                    .push(Self::header("Portfolio", "4000000.00"))
                    .push(
                        ExcaliburContainer::default()
                            .border_radius([0.0, 0.0, 8.0, 8.0].into())
                            .build(
                                Column::new()
                                    .spacing(Sizes::Md)
                                    .push(label("Unallocated ($0.00M)").secondary().build())
                                    .push(positions_row)
                                    .push(Self::separator())
                                    .push(label("Allocated ($0.00M)").secondary().build())
                                    .push(allocated_positions),
                            )
                            .padding(Sizes::Lg)
                            .width(Length::Fill)
                            .max_height(800.0),
                    )
                    .push(Self::footer::<Message>("Start Allocate", on_allocate)),
            )
    }

    pub fn separator<'a, Message>() -> Container<'a, Message>
    where
        Message: 'a,
    {
        ExcaliburContainer::default()
            .background(ExcaliburColor::Label(system::LabelColors::Quaternary))
            .build(Space::new(Length::Fill, 2.0))
            .width(Length::Fill)
    }

    pub fn header<'a, Message>(title: impl ToString, aum: impl ToString) -> Container<'a, Message>
    where
        Message: 'a,
    {
        ExcaliburContainer::default()
            .border_radius([8.0, 8.0, 0.0, 0.0].into())
            .build(
                Column::new()
                    .align_items(alignment::Alignment::Center)
                    .spacing(Sizes::Sm)
                    .push(label(title).secondary().build())
                    .push(label(aum).quantitative().ui_bold().title1().build())
                    .push(Self::separator()),
            )
            .padding(Sizes::Md)
            .center_x()
            .width(Length::Fill)
    }

    pub fn footer<'a, Message>(
        title: impl ToString,
        on_allocate: Option<Message>,
    ) -> Container<'a, Message>
    where
        Message: 'a + Clone,
    {
        ExcaliburContainer::default()
            .border_radius([0.0, 0.0, 8.0, 8.0].into())
            .build(
                Column::new()
                    .push(
                        Row::new()
                            .spacing(Sizes::Sm)
                            .push(
                                ExcaliburButton::new()
                                    .primary()
                                    .build::<Message>(label(title).body().build())
                                    .padding(Padding {
                                        top: 12.0,
                                        bottom: 12.0,
                                        left: 18.0,
                                        right: 18.0,
                                    })
                                    .on_press_maybe(on_allocate),
                            )
                            .align_items(alignment::Alignment::End),
                    )
                    .align_items(alignment::Alignment::End)
                    .width(Length::Fill),
            )
            .padding(Sizes::Lg)
            .center_x()
            .width(Length::Fill)
    }

    /// Individual inventory cell for each position.
    pub fn position_layout<'a, Message>(
        position_data: (Position, Image<image::Handle>),
    ) -> Container<'a, Message>
    where
        Message: 'a,
    {
        let (position, logo) = position_data;

        let position_data = Row::new()
            .spacing(Sizes::Sm)
            .align_items(alignment::Alignment::Center)
            .push(
                label(&format!("{}", position.balance.unwrap_or_default()))
                    .quantitative()
                    .build(),
            )
            .push(label("/").secondary().build())
            .push(
                label(&format!("{}", position.weight.unwrap_or_default()))
                    .percentage()
                    .billions()
                    .build(),
            );

        ExcaliburContainer::default()
            .transparent()
            .build(
                Column::new()
                    .spacing(Sizes::Sm)
                    .push(label(position.asset.name).secondary().build())
                    .push(logo.height(Length::Fixed(48.0)))
                    .push(position_data)
                    .align_items(alignment::Alignment::Center),
            )
            .padding(Sizes::Sm)
            .center_x()
    }
}
