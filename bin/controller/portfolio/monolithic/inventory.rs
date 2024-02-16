use datatypes::portfolio::position::{Position, Positions};
use iced::{advanced::svg, widget::Space, Border, Padding};

use super::*;
use crate::{
    components::system::{ExcaliburButton, ExcaliburColor, ExcaliburContainer, ExcaliburTooltip},
    model::portfolio::AlloyAddress,
};
use iced::widget::Svg;
pub const INVENTORY_HEIGHT: f32 = 600.0;

pub struct Inventory;

impl Inventory {
    pub fn layout<'a, Message>(
        aum: impl ToString,
        unallocated_positions: Positions,
        allocated_positions: Positions,
        unallocated_logos: Vec<svg::Handle>,
        allocated_logos: Vec<svg::Handle>,
        on_allocate: Option<Message>,
        on_select_position: impl Fn(AlloyAddress) -> Message,
    ) -> Container<'a, Message>
    where
        Message: 'static + Clone + Default,
    {
        let _current_dir = std::env::current_dir().unwrap();

        let _allocated_weight_sum = allocated_positions
            .0
            .iter()
            .map(|x| x.weight.unwrap_or_default().value)
            .sum::<f64>();

        let _unallocated_weight_sum = unallocated_positions
            .0
            .iter()
            .map(|x| x.weight.unwrap_or_default().value)
            .sum::<f64>();

        // todo: there's a small discrepancy in aum. could be due to floating point
        // precision in the non-aum calculations via `market_value()`.
        // probably not that because discrepancy is more than 1%.
        let _total_value = aum.to_string().parse::<f64>().unwrap_or_default();

        let unallocated_value = unallocated_positions
            .0
            .iter()
            .map(|position| position.market_value())
            .sum::<f64>();
        let allocated_value = allocated_positions
            .0
            .iter()
            .map(|position| position.market_value())
            .sum::<f64>();

        let unallocated_positions = Row::with_children(
            unallocated_positions
                .0
                .iter()
                .filter(|x| x.balance.unwrap_or_default() > 0.0)
                .enumerate()
                .map(|(i, x)| {
                    ExcaliburButton::new()
                        .transparent()
                        .build(Self::position_layout::<Message>((
                            x.clone(),
                            iced::widget::svg(unallocated_logos[i].clone()),
                        )))
                        .on_press(on_select_position(x.asset.address))
                        .into()
                })
                .collect::<Vec<_>>(),
        )
        .spacing(Sizes::Lg);

        let allocated_positions = Row::with_children(
            allocated_positions
                .0
                .iter()
                .filter(|x| x.balance.unwrap_or_default() > 0.0)
                .enumerate()
                .map(|(i, x)| {
                    ExcaliburButton::new()
                        .transparent()
                        .build(Self::position_layout::<Message>((
                            x.clone(),
                            iced::widget::svg(allocated_logos[i].clone()),
                        )))
                        .on_press(on_select_position(x.asset.address))
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
                    .spacing(Sizes::Md)
                    .push(Self::header("Portfolio", aum))
                    .push(
                        ExcaliburContainer::default()
                            .border_radius(Border::with_radius(8.0))
                            .build(
                                Column::new()
                                    .spacing(Sizes::Sm)
                                    .push(
                                        Row::new()
                                            .align_items(alignment::Alignment::Center)
                                            .spacing(Sizes::Sm)
                                            .push(label("Unallocated").secondary().build())
                                            .push(
                                                label(format!("{}", unallocated_value))
                                                    .quantitative()
                                                    .ui_bold()
                                                    .secondary()
                                                    .usd()
                                                    .build(),
                                            ),
                                    )
                                    .push(unallocated_positions)
                                    .push(Self::separator())
                                    .push(
                                        Row::new()
                                            .align_items(alignment::Alignment::Center)
                                            .spacing(Sizes::Sm)
                                            .push(label("Allocated").secondary().build())
                                            .push(
                                                label(format!("{}", allocated_value))
                                                    .quantitative()
                                                    .ui_bold()
                                                    .secondary()
                                                    .usd()
                                                    .build(),
                                            ),
                                    )
                                    .push(allocated_positions),
                            )
                            .width(Length::Fill),
                    )
                    .push(Self::footer::<Message>("Create Position", on_allocate)),
            )
            .padding(Padding {
                top: 28.0,
                bottom: 28.0,
                left: 48.0,
                right: 48.0,
            })
            .max_height(INVENTORY_HEIGHT)
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
            .border_radius(Border::with_radius(8.0))
            .build(
                Column::new()
                    .align_items(alignment::Alignment::Center)
                    .spacing(Sizes::Lg)
                    .push(
                        Row::new()
                            .align_items(alignment::Alignment::Center)
                            .spacing(Sizes::Sm)
                            .push(Column::new().width(Length::FillPortion(1)))
                            .push(
                                Column::new()
                                    .align_items(alignment::Alignment::Center)
                                    .width(Length::FillPortion(1))
                                    .spacing(Sizes::Sm)
                                    .push(label(title).secondary().build())
                                    .push(label(aum).quantitative().ui_bold().title1().usd().build()),
                            )
                            .push(
                                Column::new()
                                    .align_items(alignment::Alignment::End)
                                    .width(Length::FillPortion(1))
                                    .push(
                                        ExcaliburTooltip::default().padding(Sizes::Sm).info().build(
                                            "Your portfolio of allocated and unallocated positions filtered by the default token list.",
                                        )
                                    ),
                            ),
                    )
                    .push(Self::separator()),
            )
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
            .border_radius(Border::with_radius(8.0))
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
                                        left: 32.0,
                                        right: 32.0,
                                    })
                                    .on_press_maybe(on_allocate),
                            )
                            .align_items(alignment::Alignment::End),
                    )
                    .align_items(alignment::Alignment::End)
                    .width(Length::Fill),
            )
            .center_x()
            .width(Length::Fill)
    }

    /// Individual inventory cell for each position.
    pub fn position_layout<'a, Message>(position_data: (Position, Svg)) -> Container<'a, Message>
    where
        Message: 'a,
    {
        let (position, logo) = position_data;
        let position_value =
            position.cost.unwrap_or_default() * position.balance.unwrap_or_default();

        let position_header = Row::new()
            .spacing(Sizes::Sm)
            .align_items(alignment::Alignment::Center)
            .push(label(position.asset.name).secondary().build())
            .push(label("@").secondary().build())
            .push(
                label(format!("{}", position.cost.unwrap_or_default()))
                    .quantitative()
                    .usd_symbol()
                    .build(),
            );

        let position_data = Row::new()
            .spacing(Sizes::Sm)
            .align_items(alignment::Alignment::Center)
            .push(
                label(format!("{}", position_value))
                    .quantitative()
                    .usd()
                    .build(),
            )
            .push(label("/").secondary().build())
            .push(
                label(format!("{}", position.weight.unwrap_or_default()))
                    .percentage()
                    .billions()
                    .build(),
            );

        ExcaliburContainer::default()
            .transparent()
            .build(
                Column::new()
                    .spacing(Sizes::Sm)
                    .push(position_header)
                    .push(Container::new(logo.width(Length::Fixed(48.0))).padding(1))
                    .push(position_data)
                    .align_items(alignment::Alignment::Center),
            )
            .padding(Sizes::Sm)
            .center_x()
    }
}
