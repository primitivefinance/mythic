use datatypes::portfolio::coin::Coin;
use iced::Padding;
use iced_aw::{graphics::icons::icon_to_char, ICON_FONT};

use super::{dashboard::stages::review::Times, *};
use crate::{
    components::{
        input::create_input_component,
        select::excalibur_select,
        system::{
            ExcaliburButton, ExcaliburChart, ExcaliburContainer, ExcaliburInputBuilder,
            ExcaliburText,
        },
    },
    controller::portfolio::dashboard::stages::review::EnumList,
    select::custom_pick_list,
};

#[derive(Debug, Clone, Default)]
pub struct Form {
    pub chart: ExcaliburChart,
    pub amount: Option<String>,
    pub coins: Vec<Coin>,
    pub chosen_asset: Option<Coin>,
    pub chosen_quote: Option<Coin>,
    pub duration: Option<Times>,
    pub end_price: Option<String>,
    pub liquidity: Option<LiquidityTypes>,
}

impl Form {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn view<Message>(
        &self,
        submit: Message,
        on_change_deposit: impl Fn(Option<String>) -> Message + 'static,
        on_select_asset: impl Fn(Coin) -> Message + 'static,
        on_select_quote: impl Fn(Coin) -> Message + 'static,
        on_select_duration: impl Fn(Times) -> Message + 'static,
        on_change_end_price: impl Fn(Option<String>) -> Message + 'static,
        on_select_liquidity: impl Fn(LiquidityTypes) -> Message + 'static,
    ) -> Element<'_, Message>
    where
        Message: 'static + Default + Clone,
    {
        FormView::layout(
            FormView::prepare_form(
                self.amount.clone(),
                on_change_deposit,
                self.coins.clone(),
                self.chosen_asset.clone(),
                self.chosen_quote.clone(),
                on_select_asset,
                on_select_quote,
                Times::to_options(),
                self.duration,
                on_select_duration,
                self.end_price.clone(),
                on_change_end_price,
                LiquidityTypes::all(),
                self.liquidity,
                on_select_liquidity,
            ),
            FormView::chart_layout(
                &self.chart,
                label("Strategy Preview"),
                label("Synced").caption2().tertiary(),
            ),
            Column::new()
                .spacing(Sizes::Sm)
                .push(label("Instructions").secondary().build())
                .push(label("Review the details of the allocate.").build()),
            FormView::submit(Some(submit)),
        )
        .into()
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, Copy)]
pub enum LiquidityTypes {
    Low,
    #[default]
    Med,
    High,
}

impl LiquidityTypes {
    pub fn all() -> Vec<Self> {
        vec![Self::Low, Self::Med, Self::High]
    }
}

impl std::fmt::Display for LiquidityTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LiquidityTypes::Low => write!(f, "Low"),
            LiquidityTypes::Med => write!(f, "Medium"),
            LiquidityTypes::High => write!(f, "High"),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct FormView;

impl FormView {
    pub fn layout<'a, Message>(
        content: impl Into<Element<'a, Message>>,
        chart: impl Into<Element<'a, Message>>,
        instructions: impl Into<Element<'a, Message>>,
        action: impl Into<Element<'a, Message>>,
    ) -> Container<'a, Message>
    where
        Message: 'a + Clone,
    {
        ExcaliburContainer::default().transparent().build(
            Column::new()
                .width(Length::Fill)
                .spacing(Sizes::Lg)
                .push(label("Allocate New Position").title1().build())
                .push(
                    Row::new()
                        .spacing(Sizes::Md)
                        .push(
                            Column::new()
                                .width(Length::FillPortion(2))
                                .push(content.into()),
                        )
                        .push(
                            Column::new()
                                .width(Length::FillPortion(2))
                                .spacing(Sizes::Md)
                                .push(Container::new(chart.into()).max_height(350.0)),
                        )
                        .width(Length::Fill),
                )
                .push(
                    Container::new(
                        Row::new()
                            .width(Length::Fill)
                            .spacing(Sizes::Md)
                            .push(Column::new().push(instructions.into()))
                            .push(
                                Column::new()
                                    .width(Length::Fill)
                                    .push(action.into())
                                    .align_items(alignment::Alignment::End),
                            ),
                    )
                    .width(Length::Fill)
                    .align_x(alignment::Horizontal::Right),
                ),
        )
    }

    pub fn prepare_form<'a, Message>(
        deposit_amount: Option<String>,
        on_change_deposit: impl Fn(Option<String>) -> Message + 'static,
        choice_assets: Vec<Coin>,
        chosen_asset: Option<Coin>,
        chosen_quote: Option<Coin>,
        on_select_asset: impl Fn(Coin) -> Message + 'static,
        on_select_quote: impl Fn(Coin) -> Message + 'static,
        choice_duration: Vec<Times>,
        chosen_duration: Option<Times>,
        on_select_duration: impl Fn(Times) -> Message + 'static,
        end_price: Option<String>,
        on_change_end_price: impl Fn(Option<String>) -> Message + 'static,
        choice_liquidity: Vec<LiquidityTypes>,
        chosen_liquidity: Option<LiquidityTypes>,
        on_select_liquidity: impl Fn(LiquidityTypes) -> Message + 'static,
    ) -> Container<'a, Message>
    where
        Message: 'static + Default + Clone,
    {
        ExcaliburContainer::default().transparent().build(
            Column::new()
                .spacing(Sizes::Md)
                .push(
                    Row::new()
                        .spacing(Sizes::Md)
                        .push(
                            Self::deposit_form(deposit_amount, on_change_deposit)
                                .width(Length::FillPortion(2)),
                        )
                        .push(
                            Self::liquidity_type_form(
                                choice_liquidity.clone(),
                                chosen_liquidity,
                                on_select_liquidity,
                            )
                            .width(Length::FillPortion(2)),
                        ),
                )
                .push(
                    Row::new()
                        .spacing(Sizes::Md)
                        .push(
                            Self::target_price_form(end_price, on_change_end_price)
                                .width(Length::FillPortion(2)),
                        )
                        .push(
                            Self::duration_form(
                                choice_duration.clone(),
                                chosen_duration,
                                on_select_duration,
                            )
                            .width(Length::FillPortion(2)),
                        ),
                ),
        )
    }

    pub fn submit<'a, Message>(on_submit: Option<Message>) -> Container<'a, Message>
    where
        Message: 'a + Clone,
    {
        ExcaliburContainer::default().transparent().build(
            Column::new()
                .spacing(Sizes::Sm)
                .push(label("Submit").secondary().build())
                .push(
                    ExcaliburButton::new()
                        .primary()
                        .build(label("Submit for Approval").build())
                        .padding(Padding {
                            top: Sizes::Md.into(),
                            bottom: Sizes::Md.into(),
                            left: Sizes::Lg.into(),
                            right: Sizes::Lg.into(),
                        }),
                )
                .width(Length::Fill),
        )
    }

    pub fn deposit_form<'a, Message>(
        deposit_amount: Option<String>,
        on_change_deposit: impl Fn(Option<String>) -> Message + 'static,
    ) -> Container<'a, Message>
    where
        Message: 'a + Default + Clone,
    {
        Self::form_item(
            "Deposit",
            Column::new()
                .push(
                    ExcaliburInputBuilder::new()
                        .light_border()
                        .border_radius([8.0, 8.0, 0.0, 0.0].into())
                        .placeholder("Enter deposit amount".to_string())
                        .width(Length::Fill)
                        .padding(Sizes::Md.into())
                        .icon(iced::widget::text_input::Icon::<iced::Font> {
                            font: ICON_FONT,
                            code_point: icon_to_char(iced_aw::Icon::Check),
                            size: Some(Sizes::Md.into()),
                            spacing: Sizes::Sm.into(),
                            side: iced::widget::text_input::Side::Left,
                        })
                        .build(deposit_amount, on_change_deposit),
                )
                .push(
                    ExcaliburContainer::default()
                        .light_border()
                        .border_radius([0.0, 0.0, 8.0, 8.0].into())
                        .build(
                            Row::new()
                                .padding(Sizes::Sm)
                                .push(label("Max").caption().secondary().build()),
                        )
                        .width(Length::Fill),
                ),
        )
    }

    pub fn duration_form<'a, Message>(
        choice_duration: Vec<Times>,
        chosen_duration: Option<Times>,
        on_select_duration: impl Fn(Times) -> Message + 'static,
    ) -> Container<'a, Message>
    where
        Message: 'a + Default,
    {
        Self::form_item(
            "Duration",
            Column::new().push(
                excalibur_select(
                    choice_duration,
                    chosen_duration,
                    on_select_duration,
                    "Select duration",
                    Some(8.0.into()),
                )
                .padding(Sizes::Md)
                .width(Length::Fill),
            ),
        )
    }

    pub fn liquidity_type_form<'a, Message>(
        choice_liquidity: Vec<LiquidityTypes>,
        chosen_liquidity: Option<LiquidityTypes>,
        on_select_liquidity: impl Fn(LiquidityTypes) -> Message + 'static,
    ) -> Container<'a, Message>
    where
        Message: 'a + Default,
    {
        Self::form_item(
            "Liquidity Type",
            Column::new()
                .push(
                    excalibur_select(
                        choice_liquidity,
                        chosen_liquidity,
                        on_select_liquidity,
                        "Select liquidity type",
                        Some([8.0, 8.0, 0.0, 0.0].into()),
                    )
                    .padding(Sizes::Md)
                    .width(Length::Fill),
                )
                .push(
                    ExcaliburContainer::default()
                        .light_border()
                        .border_radius([0.0, 0.0, 8.0, 8.0].into())
                        .build(
                            Row::new()
                                .padding(Sizes::Sm)
                                .push(
                                    label(icon_to_char(iced_aw::Icon::Info))
                                        .icon()
                                        .secondary()
                                        .caption()
                                        .build(),
                                )
                                .push(label("Range: ").caption().secondary().build()),
                        )
                        .width(Length::Fill),
                ),
        )
    }

    pub fn target_price_form<'a, Message>(
        target_price: Option<String>,
        on_change_end_price: impl Fn(Option<String>) -> Message + 'static,
    ) -> Container<'a, Message>
    where
        Message: 'a + Default + Clone,
    {
        Self::form_item(
            "Target Price",
            Column::new().push(
                ExcaliburInputBuilder::new()
                    .light_border()
                    .border_radius([8.0, 8.0, 0.0, 0.0].into())
                    .placeholder("Enter target price".to_string())
                    .width(Length::Fill)
                    .padding(Sizes::Md.into())
                    .icon(iced::widget::text_input::Icon::<iced::Font> {
                        font: ICON_FONT,
                        code_point: icon_to_char(iced_aw::Icon::Check),
                        size: Some(Sizes::Md.into()),
                        spacing: Sizes::Sm.into(),
                        side: iced::widget::text_input::Side::Left,
                    })
                    .build(target_price, on_change_end_price),
            ),
        )
    }

    pub fn form_item<'a, Message>(
        title: impl ToString,
        content: impl Into<Element<'a, Message>>,
    ) -> Container<'a, Message>
    where
        Message: 'a + Default,
    {
        ExcaliburContainer::default().transparent().build(
            Column::new()
                .spacing(Sizes::Md)
                .push(label(title).secondary().build())
                .push(
                    ExcaliburContainer::default()
                        .round(Sizes::Sm)
                        .middle_top()
                        .light_border()
                        .build(content),
                ),
        )
    }

    pub fn chart_layout<'a, Message>(
        chart: &'a ExcaliburChart,
        chart_title: ExcaliburText,
        sync_timestamp: ExcaliburText,
    ) -> Column<'a, Message>
    where
        Message: 'a + Default,
    {
        Column::new()
            .spacing(Sizes::Md)
            .push(chart_title.build())
            .push(chart.build().map(|_| Message::default()))
            .push(sync_timestamp.build())
    }
}
