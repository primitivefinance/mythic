use std::time::{Duration, Instant};

use datatypes::portfolio::coin::Coin;
use iced::Padding;
use iced_aw::{graphics::icons::icon_to_char, Icon, ICON_FONT};

use super::{dashboard::stages::review::Times, *};
use crate::{
    components::{
        select::excalibur_select,
        system::{
            ExcaliburButton, ExcaliburChart, ExcaliburColor, ExcaliburContainer,
            ExcaliburInputBuilder, ExcaliburText,
        },
    },
    controller::portfolio::dashboard::stages::review::EnumList,
};

#[derive(Debug, Clone, Default, Copy, PartialEq, Eq, Hash)]
pub enum SubmitState {
    #[default]
    Empty,
    Pending,
    Confirmed,
    Failed,
}

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
    pub state: SubmitState,
}

impl Form {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn pending(&mut self) {
        self.state = SubmitState::Pending;
    }

    pub fn confirmed(&mut self) {
        self.state = SubmitState::Confirmed;
    }

    pub fn failed(&mut self) {
        self.state = SubmitState::Failed;
    }

    pub fn view<Message>(
        &self,
        on_close: Option<Message>,
        submit: Option<Message>,
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
            FormView::form_content(
                FormView::strategy_form(
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
                FormView::deposit_form(
                    self.amount.clone(),
                    on_change_deposit,
                    FormView::review_summary(
                        "Review",
                        vec![
                            Row::new().spacing(Sizes::Sm).push("Example").push("-10.00"),
                            Row::new().spacing(Sizes::Sm).push("Example").push("-10.00"),
                            Row::new().spacing(Sizes::Sm).push("Example").push("-10.00"),
                        ],
                    ),
                    submit,
                    self.state,
                ),
                FormView::chart_layout(
                    &self.chart,
                    label("Strategy Preview").secondary(),
                    label("Synced").caption2().tertiary(),
                ),
            ),
            on_close,
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

#[derive(Debug, Clone, Default)]
pub struct LiquidityTemplateParameters {
    pub strike_price_wad: f64,
    pub sigma_percent_wad: f64,
    pub time_remaining_years_wad: f64,
}

impl LiquidityTypes {
    pub fn all() -> Vec<Self> {
        vec![Self::Low, Self::Med, Self::High]
    }

    // todo: work on this!
    pub fn to_parameters(&self, current_price: f64) -> LiquidityTemplateParameters {
        match self {
            LiquidityTypes::Low => LiquidityTemplateParameters {
                strike_price_wad: current_price * 2.0,
                sigma_percent_wad: 0.3,
                time_remaining_years_wad: 1.0,
            },
            LiquidityTypes::Med => LiquidityTemplateParameters {
                strike_price_wad: current_price,
                sigma_percent_wad: 1.0,
                time_remaining_years_wad: 1.0,
            },
            LiquidityTypes::High => LiquidityTemplateParameters {
                strike_price_wad: current_price * 1.1,
                sigma_percent_wad: 1.3,
                time_remaining_years_wad: 1.0,
            },
        }
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

pub fn space_between<'a, Message>(
    left: impl Into<Element<'a, Message>>,
    right: impl Into<Element<'a, Message>>,
) -> Row<'a, Message>
where
    Message: 'a,
{
    Row::new()
        .push(Column::new().width(Length::Fill).push(left))
        .push(
            Column::new()
                .width(Length::Fill)
                .align_items(alignment::Alignment::End)
                .push(right),
        )
        .align_items(alignment::Alignment::Center)
        .width(Length::Fill)
}

#[derive(Debug, Clone, Default)]
pub struct FormView;

impl FormView {
    pub fn layout<'a, Message>(
        form_content: impl Into<Element<'a, Message>>,
        on_close: Option<Message>,
    ) -> Container<'a, Message>
    where
        Message: 'a + Clone,
    {
        ExcaliburContainer::default().transparent().build(
            Column::new()
                .width(Length::Fill)
                .spacing(Sizes::Lg)
                .push(Self::header("Create new position", "ETH/USDC", on_close))
                .push(form_content),
        )
    }

    pub fn header<'a, Message>(
        subtitle: impl ToString,
        title: impl ToString,
        on_close: Option<Message>,
    ) -> Row<'a, Message>
    where
        Message: 'a + Clone,
    {
        space_between(
            Column::new()
                .spacing(Sizes::Sm)
                .push(label(subtitle).secondary().build())
                .push(label(title).title1().build()),
            ExcaliburButton::new()
                .transparent()
                .build(
                    label(icon_to_char(Icon::X))
                        .icon()
                        .headline()
                        .secondary()
                        .build(),
                )
                .on_press_maybe(on_close),
        )
    }

    /// Layout of the entire create position form, including preview chart.
    pub fn form_content<'a, Message>(
        strategy_form: impl Into<Element<'a, Message>>,
        deposit_form: impl Into<Element<'a, Message>>,
        chart: impl Into<Element<'a, Message>>,
    ) -> Column<'a, Message>
    where
        Message: 'a,
    {
        Column::new()
            .spacing(Sizes::Md)
            .width(Length::Fill)
            .push(
                Row::new()
                    .spacing(Sizes::Md)
                    .push(
                        Column::new()
                            .width(Length::FillPortion(2))
                            .push(strategy_form.into()),
                    )
                    .push(
                        Column::new()
                            .width(Length::FillPortion(3))
                            .spacing(Sizes::Md)
                            .push(chart.into()),
                    )
                    .width(Length::Fill),
            )
            .push(deposit_form.into())
    }

    /// Layout of the deposit input, review summary, and submit button.
    pub fn deposit_form<'a, Message>(
        deposit_amount: Option<String>,
        on_change_deposit: impl Fn(Option<String>) -> Message + 'static,
        review: impl Into<Element<'a, Message>>,
        submit: Option<Message>,
        state: SubmitState,
    ) -> Container<'a, Message>
    where
        Message: 'a + Clone + Default,
    {
        ExcaliburContainer::default().transparent().build(
            Row::new()
                .width(Length::Fill)
                .spacing(Sizes::Md)
                .push(
                    Self::deposit_input(deposit_amount, on_change_deposit)
                        .width(Length::FillPortion(2)),
                )
                .push(
                    Row::new()
                        .spacing(Sizes::Sm)
                        .width(Length::FillPortion(3))
                        .push(Container::new(review.into()).width(Length::FillPortion(2)))
                        .push(Self::submit(submit, state).width(Length::FillPortion(2))),
                ),
        )
    }

    /// Simple column of rows of review item elements.
    pub fn review_summary<'a, Message>(
        title: impl ToString,
        rows: Vec<impl Into<Element<'a, Message>>>,
    ) -> Container<'a, Message>
    where
        Message: 'a + Default,
    {
        Self::form_item(
            title,
            Column::with_children(rows.into_iter().map(|x| x.into()).collect::<Vec<_>>())
                .spacing(Sizes::Sm)
                .padding(Sizes::Md),
        )
    }

    /// Layout of the strategy selections to choose from.
    /// todo: add a toggle to switch to advanced mode that lets you choose.
    pub fn strategy_form<'a, Message>(
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
            Column::new().spacing(Sizes::Md).push(
                Column::new()
                    .spacing(Sizes::Md)
                    .push(label("Choose strategy").secondary().build())
                    .push(
                        Column::new()
                            .spacing(Sizes::Md)
                            .push(
                                Self::strategy_template(
                                    Some(on_select_liquidity(LiquidityTypes::Low)),
                                    LiquidityTypes::Low,
                                    chosen_liquidity == Some(LiquidityTypes::Low),
                                )
                                .width(Length::Fill),
                            )
                            .push(
                                Self::strategy_template(
                                    Some(on_select_liquidity(LiquidityTypes::Med)),
                                    LiquidityTypes::Med,
                                    chosen_liquidity == Some(LiquidityTypes::Med),
                                )
                                .width(Length::Fill),
                            )
                            .push(
                                Self::strategy_template(
                                    Some(on_select_liquidity(LiquidityTypes::High)),
                                    LiquidityTypes::High,
                                    chosen_liquidity == Some(LiquidityTypes::High),
                                )
                                .width(Length::Fill),
                            ),
                    ),
            ),
        )
    }

    /// "Cast" for each strategy template option.
    pub fn strategy_template<'a, Message>(
        on_press: Option<Message>,
        value: impl ToString,
        active: bool,
    ) -> Column<'a, Message>
    where
        Message: 'a + Clone + Default,
    {
        let mut value = label(value).secondary();
        let mut background = ExcaliburColor::Background3;

        if active {
            value = value.highlight();
            background = ExcaliburColor::Background4;
        }

        Column::new().push(
            ExcaliburButton::new()
                .selectable()
                .border_radius(8.0.into())
                .active()
                .background(background)
                .build(
                    Column::new()
                        .push(
                            Row::new()
                                .padding(Sizes::Md)
                                .spacing(Sizes::Sm)
                                .width(Length::Fill)
                                .align_items(alignment::Alignment::Center)
                                .push(Column::new().push(label("Type").build()))
                                .push(
                                    Column::new()
                                        .width(Length::FillPortion(3))
                                        .align_items(alignment::Alignment::End)
                                        .push(value),
                                ),
                        )
                        .push(
                            ExcaliburContainer::default()
                                .light_border()
                                .border_radius([0.0, 0.0, 8.0, 8.0].into())
                                .build(
                                    Row::new()
                                        .padding(Padding {
                                            top: Sizes::Sm.into(),
                                            bottom: Sizes::Sm.into(),
                                            left: Sizes::Md.into(),
                                            right: Sizes::Md.into(),
                                        })
                                        .push(
                                            label(icon_to_char(Icon::Info))
                                                .caption()
                                                .secondary()
                                                .icon()
                                                .build(),
                                        ),
                                )
                                .width(Length::Fill),
                        ),
                )
                .padding(0)
                .width(Length::Fill)
                .on_press_maybe(on_press),
        )
    }

    /// Form submit button for creating the position.
    pub fn submit<'a, Message>(
        on_submit: Option<Message>,
        state: SubmitState,
    ) -> Container<'a, Message>
    where
        Message: 'a + Clone,
    {
        let mut on_submit = on_submit;
        let mut button_background = ExcaliburColor::Primary;
        let mut button_content = Row::new()
            .spacing(Sizes::Sm)
            .align_items(alignment::Alignment::Center);

        let loading_indicator = iced_loading_indicator::Widget::new(
            LOADING_INDICATOR_SIZE,
            Some(iced_loading_indicator::Style::CustomColor(
                iced::Color::from_rgb8(0xaa, 0xaa, 0xff),
            )),
            true,
        )
        .tick_duration_ms(LOADING_INDICATOR_SPEED_MS);

        match state {
            SubmitState::Empty => {
                button_content = button_content
                    .push(label(icon_to_char(Icon::ShieldShaded)).icon().build())
                    .push(label("Submit for Approval").build());
            }
            SubmitState::Pending => {
                button_content = button_content
                    .push(loading_indicator)
                    .push(label("Pending...").secondary().build());
                on_submit = None;
            }
            SubmitState::Confirmed => {
                button_content = button_content
                    .push(label(icon_to_char(Icon::CheckAll)).icon().title2().build())
                    .push(label("Continue").build());
            }
            SubmitState::Failed => {
                button_background = ExcaliburColor::Button(system::ButtonColors::Error);
                button_content = button_content
                    .push(label(icon_to_char(Icon::X)).icon().build())
                    .push(label("Failed").build());
            }
        }

        ExcaliburContainer::default().transparent().build(
            Column::new()
                .spacing(Sizes::Md)
                .push(label("Submit").secondary().build())
                .push(
                    ExcaliburButton::new()
                        .primary()
                        .active()
                        .background(button_background)
                        .hovered()
                        .background(button_background)
                        .disabled()
                        .background(button_background)
                        .pressed()
                        .background(button_background)
                        .build(button_content)
                        .padding(Padding {
                            top: Sizes::Md.into(),
                            bottom: Sizes::Md.into(),
                            left: Sizes::Lg.into(),
                            right: Sizes::Lg.into(),
                        })
                        .on_press_maybe(on_submit),
                )
                .width(Length::Fill),
        )
    }

    /// Form input for the deposit amount.
    pub fn deposit_input<'a, Message>(
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
                        .padding(Padding {
                            top: Sizes::Lg.into(),
                            bottom: Sizes::Lg.into(),
                            left: Sizes::Md.into(),
                            right: Sizes::Md.into(),
                        })
                        .size(system::Typography::Headline)
                        .icon(iced::widget::text_input::Icon::<iced::Font> {
                            font: ICON_FONT,
                            code_point: icon_to_char(iced_aw::Icon::ShieldShaded),
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
                                .padding(Padding {
                                    top: Sizes::Sm.into(),
                                    bottom: Sizes::Sm.into(),
                                    left: Sizes::Md.into(),
                                    right: Sizes::Md.into(),
                                })
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

    /// "Cast" of a single form element.
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

    /// Layout of the chart.
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
            .push(
                Row::new()
                    .align_items(alignment::Alignment::Center)
                    .spacing(Sizes::Md)
                    .push(chart_title.build())
                    .push(sync_timestamp.build()),
            )
            .push(
                ExcaliburContainer::default()
                    .build(chart.build().map(|_| Message::default()))
                    .width(Length::Fill)
                    .height(350.0),
            )
    }
}

const LOADING_INDICATOR_SIZE: f32 = 16.0;
const LOADING_INDICATOR_SPEED_MS: u64 = 85;
