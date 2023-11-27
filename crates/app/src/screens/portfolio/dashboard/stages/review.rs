//! The review screen for making a portfolio adjustment.

use super::*;

#[derive(Debug, Clone, Default)]
pub enum FormMessage {
    #[default]
    Empty,
    StartTime(Option<String>),
    EndTime(Option<String>),
    Rebate(Option<String>),
    Strategy(Option<String>),
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
    start_time: Option<String>,
    end_time: Option<String>,
    rebate: Option<String>,
    strategy: Option<String>,
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
                FormMessage::EndTime(end_time) => {
                    self.form.end_time = end_time;
                }
                FormMessage::Rebate(rebate) => {
                    self.form.rebate = rebate;
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
                FormMessage::Submit => {}
            },
            Self::AppMessage::Empty => {}
        }

        Command::none()
    }

    fn view(&self) -> Element<'_, Self::ViewMessage> {
        let content = DualColumn::new().column_2_alignment(alignment::Alignment::Start)
            .spacing(Sizes::Md)
            .column_1(vec![
                h2("Adjust Portfolio".to_string()).into(),
                labeled_input(
                    "Adjust start time".to_string(),
                    self.form.start_time.clone(),
                    "11/25".to_string(),
                     FormMessage::StartTime,
                )
                .into(),
                labeled_input(
                    "Adjust end time".to_string(),
                    self.form.end_time.clone(),
                    "12/25".to_string(),
                    FormMessage::EndTime,
                )
                .into(),
                labeled_input(
                    "Rebate".to_string(),
                    self.form.rebate.clone(),
                    "0.1".to_string(),
                    FormMessage::Rebate,
                )
                .into(),
            ])
            .column_2(vec![
                h2("Select Strategy".to_string()).into(),
                labeled_input(
                    "Strategy".to_string(),
                    self.form.strategy.clone(),
                    "0.1".to_string(),
                    FormMessage::Strategy,
                )
                .into(),
                Card::template()
                    .background(Some(iced::Background::Color(GRAY_500)))
                    .build(
                        instructions_inner(vec![instruction_text(
                            "This is a basic strategy that will continuously adjust the position until the target has been reached".to_string(),
                        )]),
                        9.0.into(),
                    )
                    .into(),
            ]);

        Container::new(
            Row::new()
                .spacing(Sizes::Lg)
                .push(
                    content
                        .build()
                        .spacing(Sizes::Lg)
                        .width(Length::FillPortion(2)),
                )
                .push(
                    instructions(
                        vec![instruction_text(
                            "Simulate the adjustment before executing it.".to_string(),
                        )],
                        Some("Simulate Adjustment".to_string()),
                        None,
                        Some(FormMessage::Submit),
                    )
                    .width(Length::FillPortion(1)),
                ),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()
    }
}
