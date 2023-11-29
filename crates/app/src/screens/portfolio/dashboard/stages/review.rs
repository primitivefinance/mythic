//! The review screen for making a portfolio adjustment.

use std::fmt::{self, Display};

use super::*;

#[derive(Debug, Clone, Default)]
pub enum FormMessage {
    #[default]
    Empty,
    StartTime(Option<Times>),
    Duration(Option<Times>),
    Fee(Option<Fees>),
    Strategy(Option<Strategies>),
    Submit,
}

impl MessageWrapperView for FormMessage {
    type ParentMessage = super::Message;
}

impl MessageWrapper for FormMessage {
    type ParentMessage = Message;
}

impl From<FormMessage> for <FormMessage as MessageWrapper>::ParentMessage {
    fn from(message: FormMessage) -> Self {
        Self::Form(message)
    }
}

impl From<FormMessage> for <FormMessage as MessageWrapperView>::ParentMessage {
    fn from(message: FormMessage) -> Self {
        Self::Review(Message::Form(message))
    }
}

#[derive(Debug, Clone, Default)]
pub struct Form {
    start_time: Option<Times>,
    duration: Option<Times>,
    fee: Option<Fees>,
    strategy: Option<Strategies>,
}

#[derive(Debug, Clone, Default)]
pub enum Message {
    #[default]
    Empty,
    Form(FormMessage),
}

impl MessageWrapperView for Message {
    type ParentMessage = super::Message;
}

impl MessageWrapper for Message {
    type ParentMessage = super::Message;
}

impl From<Message> for <Message as MessageWrapper>::ParentMessage {
    fn from(message: Message) -> Self {
        Self::Review(message)
    }
}

#[derive(Debug, Clone, Default)]
pub struct ReviewAdjustment {
    form: Form,
}

impl ReviewAdjustment {
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns an instructions element to guide the user.
    pub fn guide(&self, on_submit: Option<super::Message>) -> Container<'static, super::Message> {
        instructions(
            vec![instruction_text(
                "Select the adjustment parameters and strategy to use.".to_string(),
            )],
            Some("Simulate Adjustment".to_string()),
            None,
            on_submit,
        )
    }
}

impl State for ReviewAdjustment {
    type AppMessage = Message;
    type ViewMessage = FormMessage;

    fn update(&mut self, message: Self::AppMessage) -> Command<Self::AppMessage> {
        match message {
            Self::AppMessage::Form(form_message) => match form_message {
                FormMessage::Empty => {}
                FormMessage::StartTime(start_time) => {
                    self.form.start_time = start_time;
                }
                FormMessage::Duration(duration) => {
                    self.form.duration = duration;
                }
                FormMessage::Fee(fee) => {
                    self.form.fee = fee;
                }
                FormMessage::Strategy(strategy) => {
                    self.form.strategy = strategy;
                }
                // Problem: We are triggering submit in a child component of this review component.
                // However, we need to trigger some changes in the parent component of this review
                // component. One solution is to catch this event being sent to the
                // child in the propagation process. But that does not feel like the
                // best solution, what's a better one?
                // GPT: callbacks.
                // I think this is worse because
                // we already have all the messaging infrastructure in place to
                // react to it.
                FormMessage::Submit => {
                    let start_time_seconds = self
                        .form
                        .start_time
                        .as_ref()
                        .map(|x| x.to_seconds())
                        .unwrap_or_default();

                    let duration_seconds = self
                        .form
                        .duration
                        .as_ref()
                        .map(|x| x.to_seconds())
                        .unwrap_or_default();

                    let fee_percentage = self
                        .form
                        .fee
                        .as_ref()
                        .map(|x| x.to_value())
                        .unwrap_or_default();

                    let strategy = self
                        .form
                        .strategy
                        .as_ref()
                        .map(|x| x.to_value())
                        .unwrap_or_default();

                    tracing::info!(
                        "start_time_seconds: {}, duration_seconds: {}, fee_percentage: {}, strategy: {}",
                        start_time_seconds,
                        duration_seconds,
                        fee_percentage,
                        strategy
                    );
                }
            },
            Self::AppMessage::Empty => {}
        }

        Command::none()
    }

    fn view(&self) -> Element<'_, Self::ViewMessage> {
        let current_description = match self.form.strategy {
            Some(strategy) => strategy.description(),
            None => {
                "The strategy determines how the portfolio weight change is executed over time."
                    .to_string()
            }
        };

        let content = DualColumn::new()
            .column_2_alignment(alignment::Alignment::Start)
            .spacing(Sizes::Md)
            .column_1(vec![
                h2("Strategy Parameters".to_string()).into(),
                labeled_select(
                    "Adjust start time".to_string(),
                    Times::to_options(),
                    self.form.start_time.clone(),
                    |x| FormMessage::StartTime(Some(x)),
                )
                .into(),
                labeled_select(
                    "Adjust duration".to_string(),
                    Times::to_options(),
                    self.form.duration.clone(),
                    |x| FormMessage::Duration(Some(x)),
                )
                .into(),
                labeled_select(
                    "Choose pool fee".to_string(),
                    Fees::to_options(),
                    self.form.fee.clone(),
                    |x| FormMessage::Fee(Some(x)),
                )
                .into(),
            ])
            .column_2(vec![
                h2("Execution Strategy".to_string()).into(),
                labeled_select(
                    "Choose rebalance rate".to_string(),
                    Strategies::to_options(),
                    self.form.strategy.clone(),
                    |x| FormMessage::Strategy(Some(x)),
                )
                .into(),
                Card::template()
                    .background(Some(iced::Background::Color(GRAY_500)))
                    .build(
                        instructions_inner(vec![instruction_text(current_description)]),
                        9.0.into(),
                    )
                    .width(Length::Fill)
                    .padding(Sizes::Md)
                    .into(),
            ]);

        content.build().spacing(Sizes::Lg).into()
    }
}

pub trait EnumList<T> {
    fn to_options() -> Vec<Self>
    where
        Self: Sized;
    fn to_list() -> Vec<String>;
    fn to_value(&self) -> T;
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Times {
    OneHour,
    FourHour,
    EightHour,
    TwelveHour,
    OneDay,
    TwoDays,
    ThreeDays,
    OneWeek,
    TwoWeeks,
    OneMonth,
    ThreeMonths,
    SixMonths,
    OneYear,
}

impl Times {
    pub fn to_seconds(&self) -> f64 {
        self.to_value()
    }
}

impl Display for Times {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Times::OneHour => write!(f, "1 hour"),
            Times::FourHour => write!(f, "4 hours"),
            Times::EightHour => write!(f, "8 hours"),
            Times::TwelveHour => write!(f, "12 hours"),
            Times::OneDay => write!(f, "1 day"),
            Times::TwoDays => write!(f, "2 days"),
            Times::ThreeDays => write!(f, "3 days"),
            Times::OneWeek => write!(f, "1 week"),
            Times::TwoWeeks => write!(f, "2 weeks"),
            Times::OneMonth => write!(f, "1 month"),
            Times::ThreeMonths => write!(f, "3 months"),
            Times::SixMonths => write!(f, "6 months"),
            Times::OneYear => write!(f, "1 year"),
        }
    }
}

impl EnumList<f64> for Times {
    fn to_options() -> Vec<Times> {
        vec![
            Times::OneHour,
            Times::FourHour,
            Times::EightHour,
            Times::TwelveHour,
            Times::OneDay,
            Times::TwoDays,
            Times::ThreeDays,
            Times::OneWeek,
            Times::TwoWeeks,
            Times::OneMonth,
            Times::ThreeMonths,
            Times::SixMonths,
            Times::OneYear,
        ]
    }

    fn to_list() -> Vec<String> {
        vec![
            Times::OneHour.to_string(),
            Times::FourHour.to_string(),
            Times::EightHour.to_string(),
            Times::TwelveHour.to_string(),
            Times::OneDay.to_string(),
            Times::TwoDays.to_string(),
            Times::ThreeDays.to_string(),
            Times::OneWeek.to_string(),
            Times::TwoWeeks.to_string(),
            Times::OneMonth.to_string(),
            Times::ThreeMonths.to_string(),
            Times::SixMonths.to_string(),
            Times::OneYear.to_string(),
        ]
    }

    fn to_value(&self) -> f64 {
        match self {
            Times::OneHour => chrono::Duration::hours(1).num_seconds() as f64,
            Times::FourHour => chrono::Duration::hours(4).num_seconds() as f64,
            Times::EightHour => chrono::Duration::hours(8).num_seconds() as f64,
            Times::TwelveHour => chrono::Duration::hours(12).num_seconds() as f64,
            Times::OneDay => chrono::Duration::days(1).num_seconds() as f64,
            Times::TwoDays => chrono::Duration::days(2).num_seconds() as f64,
            Times::ThreeDays => chrono::Duration::days(3).num_seconds() as f64,
            Times::OneWeek => chrono::Duration::weeks(1).num_seconds() as f64,
            Times::TwoWeeks => chrono::Duration::weeks(2).num_seconds() as f64,
            Times::OneMonth => chrono::Duration::weeks(4).num_seconds() as f64, // Approximation
            Times::ThreeMonths => chrono::Duration::weeks(12).num_seconds() as f64, // Approximation
            Times::SixMonths => chrono::Duration::weeks(24).num_seconds() as f64, // Approximation
            Times::OneYear => chrono::Duration::weeks(52).num_seconds() as f64, // Approximation
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Default)]
pub enum Fees {
    #[default]
    OneBps,
    ThreeBps,
    TenBps,
    ThirtyBps,
    FiftyBps,
    OneHundredBps,
}

impl Display for Fees {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Fees::OneBps => write!(f, "0.01%"),
            Fees::ThreeBps => write!(f, "0.03%"),
            Fees::TenBps => write!(f, "0.10%"),
            Fees::ThirtyBps => write!(f, "0.30%"),
            Fees::FiftyBps => write!(f, "0.50%"),
            Fees::OneHundredBps => write!(f, "1.00%"),
        }
    }
}

impl EnumList<f64> for Fees {
    fn to_options() -> Vec<Fees> {
        vec![
            Fees::OneBps,
            Fees::ThreeBps,
            Fees::TenBps,
            Fees::ThirtyBps,
            Fees::FiftyBps,
            Fees::OneHundredBps,
        ]
    }

    fn to_list() -> Vec<String> {
        vec![
            Fees::OneBps.to_string(),
            Fees::ThreeBps.to_string(),
            Fees::TenBps.to_string(),
            Fees::ThirtyBps.to_string(),
            Fees::FiftyBps.to_string(),
            Fees::OneHundredBps.to_string(),
        ]
    }

    fn to_value(&self) -> f64 {
        match self {
            Fees::OneBps => 0.0001,
            Fees::ThreeBps => 0.0003,
            Fees::TenBps => 0.001,
            Fees::ThirtyBps => 0.003,
            Fees::FiftyBps => 0.005,
            Fees::OneHundredBps => 0.01,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Default)]
pub enum Strategies {
    #[default]
    Linear,
    SCurve,
    Exponential,
}

impl Strategies {
    pub fn description(&self) -> String {
        match self {
            Strategies::Linear => {
                "Changes the portfolio weights by the same amounts over time.".to_string()
            }
            Strategies::SCurve => {
                "Changes the portfolio weights slowly at first, then quickly, then slowly again."
                    .to_string()
            }
            Strategies::Exponential => {
                "Accelerates the portfolio weight changes until completion.".to_string()
            }
        }
    }
}

impl Display for Strategies {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Strategies::Linear => write!(f, "Linear"),
            Strategies::SCurve => write!(f, "S-Curve"),
            Strategies::Exponential => write!(f, "Exponential"),
        }
    }
}

impl EnumList<Strategies> for Strategies {
    fn to_options() -> Vec<Strategies> {
        vec![
            Strategies::Linear,
            Strategies::SCurve,
            Strategies::Exponential,
        ]
    }

    fn to_list() -> Vec<String> {
        vec![
            Strategies::Linear.to_string(),
            Strategies::SCurve.to_string(),
            Strategies::Exponential.to_string(),
        ]
    }

    fn to_value(&self) -> Strategies {
        *self
    }
}
