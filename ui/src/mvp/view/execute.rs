//! Views for executing transactions.

use iced::widget::pick_list;

use super::{components::input::create_input_component, text, Column, Element, Message, *};
use crate::mvp::execution::TransactionSteps;

pub fn execution_layout<'a>(
    step: TransactionSteps,
    input_amount: String,
    to_address: Addresses,
) -> Element<'a, Message> {
    let content = match step {
        TransactionSteps::Start => starting(input_amount, to_address),
        TransactionSteps::Review => reviewing(),
        TransactionSteps::Simulated => simulated(),
        TransactionSteps::Confirmed => confirmed(),
    };

    Column::new()
        .push(Row::new().push(content).height(Length::FillPortion(5)))
        .push(
            Row::new()
                .push(button(text("previous")).on_press(Message::Execution(Execution::Previous)))
                .push(button(text("next")).on_press(Message::Execution(Execution::Next)))
                .align_items(alignment::Alignment::End)
                .height(Length::FillPortion(1))
                .spacing(8),
        )
        .spacing(16)
        .into()
}

/// Panel for starting a new transaction.
pub fn starting<'a>(input_amount: String, selected: Addresses) -> Element<'a, Message> {
    let title = data_item("execution".to_string()).size(36);
    let input = create_input_component(Some(input_amount.clone()), |value| {
        Message::Execution(view::Execution::AmountChanged(value))
    });

    let summary = Column::new()
        .spacing(4)
        .push(label_item("summary".to_string()).size(16))
        .push(text("Transaction will succeed"))
        .push("Transaction has warnings");

    let column_1: Vec<Element<'a, Message>> = vec![
        title.into(),
        label_item("to".to_string()).size(28).into(),
        pick_list(&Addresses::ALL[..], Some(selected.clone()), |value| {
            Message::Execution(view::Execution::ToAddressChanged(value))
        })
        .into(),
        label_item("action".to_string()).size(28).into(),
        input.into(),
        Row::new()
            .push(label_item("transaction cost".to_string()))
            .push(text("$20.00"))
            .into(),
    ];

    let column_2: Vec<Element<'a, Message>> = vec![summary.into()];

    Column::new()
        .push(components::dual_column(column_1, column_2))
        .spacing(16)
        .padding(32)
        .width(Length::Fill)
        .into()
}

/// Panel for reviewing the transaction's state diffs.
pub fn reviewing<'a>() -> Element<'a, Message> {
    let title = data_item("review".to_string()).size(36);

    let column_1: Vec<Element<'a, Message>> = vec![title.into()];
    let column_2: Vec<Element<'a, Message>> = vec![button(text("Confirm")).into()];

    Column::new()
        .push(components::dual_column(column_1, column_2))
        .spacing(16)
        .padding(32)
        .width(Length::Fill)
        .into()
}

/// Extended panel for reviewing simulated diffs
pub fn simulated<'a>() -> Element<'a, Message> {
    Column::new()
        .push(text("Simulated"))
        .push(text("Review the transaction's state diffs."))
        .into()
}

/// Panel for transaction confirmation.
pub fn confirmed<'a>() -> Element<'a, Message> {
    Column::new()
        .push(text("Confirmed"))
        .push(text("Review the transaction's state diffs."))
        .into()
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum Addresses {
    #[default]
    Zero,
    Trusted,
    Untrusted,
}

impl Addresses {
    const ALL: [Addresses; 3] = [Addresses::Zero, Addresses::Trusted, Addresses::Untrusted];
}

impl std::fmt::Display for Addresses {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Addresses::Zero => "zero".to_string(),
                Addresses::Trusted => "trusted".to_string(),
                Addresses::Untrusted => "untrusted".to_string(),
            }
        )
    }
}
