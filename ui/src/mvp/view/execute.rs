//! Views for executing transactions.

use iced::widget::pick_list;

use super::{components::input::create_input_component, text, Column, Element, Message, *};
use crate::mvp::{
    execution::TransactionSteps, screens::execution::StorageDiffs, units::address_to_string,
};

pub fn execution_layout<'a>(
    step: TransactionSteps,
    input_amount: String,
    target: Vec<(Address, String)>,
    selected: String,
    user_feedback: Option<String>,
    review_diffs: Option<StorageDiffs>,
) -> Element<'a, Message> {
    let address_book: Vec<String> = target
        .clone()
        .into_iter()
        .map(|(a, _): (Address, _)| address_to_string(&a))
        .collect();

    let content = match step {
        TransactionSteps::Start => starting(input_amount, address_book, selected),
        TransactionSteps::Simulated => simulated(review_diffs),
        TransactionSteps::Executed => executed(),
        TransactionSteps::Confirmed => confirmed(),
    };

    let user_feedback = match user_feedback {
        Some(feedback) => text(feedback),
        None => text(""),
    };

    Column::new()
        .push(Row::new().push(content).height(Length::FillPortion(5)))
        .push(
            Row::new()
                .push(button(text("previous")).on_press(Message::Execution(Execution::Previous)))
                .push(button(text("next")).on_press(Message::Execution(Execution::Next)))
                .push(user_feedback)
                .align_items(alignment::Alignment::End)
                .height(Length::FillPortion(1))
                .spacing(8),
        )
        .spacing(16)
        .into()
}

/// Panel for starting a new transaction.
pub fn starting<'a>(
    input_amount: String,
    address_book: Vec<String>,
    selected: String,
) -> Element<'a, Message> {
    let title = data_item("execution".to_string()).size(36);
    let input = create_input_component(Some(input_amount.clone()), |value| {
        Message::Execution(view::Execution::AmountChanged(value))
    });

    let selection = address_book.clone();

    tracing::info!("Selection options: {:?}", selection);
    tracing::info!("Selected: {:?}", selected);

    let column_1: Vec<Element<'a, Message>> = vec![
        title.into(),
        label_item("to".to_string()).size(28).into(),
        pick_list(selection, Some(selected.clone()), |value| {
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

    let submit = submit_group();

    let column_2: Vec<Element<'a, Message>> = vec![submit.into()];

    Column::new()
        .push(components::dual_column(column_1, column_2))
        .spacing(16)
        .padding(32)
        .width(Length::Fill)
        .into()
}

/// Panel for executed the transaction's state diffs.
pub fn executed<'a>() -> Element<'a, Message> {
    let title = data_item("Execute Transaction".to_string()).size(36);

    let column_1: Vec<Element<'a, Message>> = vec![title.into()];
    let column_2: Vec<Element<'a, Message>> = vec![button(text("Execute")).into()];

    Column::new()
        .push(components::dual_column(column_1, column_2))
        .spacing(16)
        .padding(32)
        .width(Length::Fill)
        .into()
}

/// Extended panel for executed simulated diffs
pub fn simulated<'a>(review_diffs: Option<StorageDiffs>) -> Element<'a, Message> {
    let title = data_item("Review Transaction".to_string()).size(36);

    let mut column_1: Vec<Element<'a, Message>> = vec![title.into()];

    match review_diffs {
        Some(diffs) => {
            // let mut diff_list: Vec<Element<'a, Message>> = vec![
            // label_item("Slot".to_string()).into(),
            // label_item("Value".to_string()).into(),
            // ];
            // for (key, (before, after)) in diffs {
            // let slot = U256::from_little_endian(key.as_le_bytes().as_ref());
            //
            // let before_value = match before {
            // Some(value) => value.to_string(),
            // None => "".to_string(),
            // };
            //
            // let after_value = match after {
            // Some(value) => value.to_string(),
            // None => "".to_string(),
            // };
            //
            // diff_list.push(
            // Row::new()
            // .push(text(slot.to_string()))
            // .push(text(before_value))
            // .push(text(after_value))
            // .into(),
            // );
            // }
            //
            // column_1.push(Column::with_children(diff_list).into());
            column_1.push(storage_diffs_table(diffs).into());
        }
        None => {
            column_1.push(text("No diffs to display").into());
        }
    };

    Column::with_children(column_1)
        .spacing(16)
        .padding(32)
        .width(Length::Fill)
        .into()
}

/// Panel for transaction confirmation.
pub fn confirmed<'a>() -> Element<'a, Message> {
    Column::new()
        .push(text("Transaction Confirmed"))
        .push(text("Review the transaction's state diffs."))
        .into()
}

/// Storage diffs table
pub fn storage_diffs_table<'a>(review_diffs: StorageDiffs) -> Element<'a, Message> {
    let header = Row::new()
        .spacing(8)
        .padding(8)
        .push(label_item("Slot".to_string()))
        .push(label_item("Before".to_string()))
        .push(label_item("After".to_string()));

    let rows: Vec<Element<'a, Message>> = review_diffs
        .iter()
        .map(|(slot, (before, after))| {
            let before_value = match before {
                Some(value) => value.to_string(),
                None => "".to_string(),
            };

            let after_value = match after {
                Some(value) => value.to_string(),
                None => "".to_string(),
            };

            Row::new()
                .push(text(slot.to_string()))
                .push(text(before_value))
                .push(text(after_value))
                .spacing(8)
                .padding(8)
                .into()
        })
        .collect::<Vec<_>>();

    Column::new()
        .push(header)
        .push(Column::with_children(rows))
        .spacing(8)
        .padding(8)
        .into()
}

/// Submit group
pub fn submit_group<'a>() -> Element<'a, Message> {
    let title = h3("Submit".to_string());
    let info = text_label("Awaiting approval in review...".to_string());
    let button = action_button("Submit".to_string())
        .on_press(Message::Empty)
        .padding(Sizes::Md as u16)
        .width(Length::Fill);

    let inner_column = Column::new()
        .push(title)
        .push(info)
        .align_items(alignment::Alignment::Start)
        .spacing(Sizes::Sm as u16)
        .padding(Sizes::Sm as u16)
        .width(Length::Fill)
        .height(Length::Fill);

    let h: f32 = ByteScale::Xl4.between(&ByteScale::Xl5);

    Card::new(
        Column::new()
            .push(inner_column)
            .push(button)
            .align_items(alignment::Alignment::Center)
            .padding(Sizes::Md as u16)
            .spacing(Sizes::Md as u16),
    )
    .width(Length::Fixed(ByteScale::Xl5.into()))
    .max_height(h)
    .into()
}
