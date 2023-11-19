//! Views for executing transactions.

use iced::Color;
use iced_aw::{graphics::icons::icon_to_char, Icon, ICON_FONT};

use super::{text, Column, Element, Message, *};
use crate::mvp::{
    components::{button::route_button_style, tables::*},
    execution::TransactionSteps,
    screens::execution::StorageDiffs,
    units::address_to_string,
};

pub fn execution_layout<'a>(
    step: TransactionSteps,
    input_amount: String,
    target: Vec<(Address, String)>,
    selected: String,
    user_feedback: Option<String>,
    review_diffs: Option<StorageDiffs>,
    checkpoint_step: TransactionSteps,
    pending: bool,
) -> Element<'a, Message> {
    let address_book: Vec<String> = target
        .clone()
        .into_iter()
        .map(|(a, _): (Address, _)| address_to_string(&a))
        .collect();

    let steps = vec![
        (
            Icon::PencilSquare,
            "Build".to_string(),
            if checkpoint_step >= TransactionSteps::Start {
                Message::Execution(Execution::Route(TransactionSteps::Start))
            } else {
                Message::Empty
            },
            step == TransactionSteps::Start,
        ),
        (
            Icon::Sim,
            "Simulate".to_string(),
            if checkpoint_step >= TransactionSteps::Simulated {
                Message::Execution(Execution::Route(TransactionSteps::Simulated))
            } else {
                Message::Empty
            },
            step == TransactionSteps::Simulated,
        ),
        (
            Icon::CursorFill,
            "Execute".to_string(),
            if checkpoint_step >= TransactionSteps::Executed {
                Message::Execution(Execution::Route(TransactionSteps::Executed))
            } else {
                Message::Empty
            },
            step == TransactionSteps::Executed,
        ),
        (
            Icon::CheckCircleFill,
            "Confirm".to_string(),
            if checkpoint_step >= TransactionSteps::Confirmed {
                Message::Execution(Execution::Route(TransactionSteps::Confirmed))
            } else {
                Message::Empty
            },
            step == TransactionSteps::Confirmed,
        ),
    ];

    let action = match step {
        TransactionSteps::Start
            if !input_amount.is_empty() && selected.len() > 0 && step == checkpoint_step =>
        {
            Message::Execution(Execution::Next)
        }
        TransactionSteps::Simulated if !pending && step == checkpoint_step => {
            Message::Execution(Execution::Next)
        }
        TransactionSteps::Executed if !pending && step == checkpoint_step => {
            Message::Execution(Execution::Next)
        }
        TransactionSteps::Confirmed if !pending && step == checkpoint_step => {
            Message::Execution(Execution::Next)
        }
        _ => Message::Empty,
    };

    let content = match step {
        TransactionSteps::Start => {
            starting(input_amount.clone(), address_book.clone(), selected.clone())
        }
        TransactionSteps::Simulated => simulated(
            selected.clone(),
            selected.clone(),
            input_amount.clone(),
            review_diffs,
        ),
        TransactionSteps::Executed => executed(),
        TransactionSteps::Confirmed => confirmed(),
    };

    let steps_card = steps_group(steps);
    let submit_card = submit_group(action, step.clone(), user_feedback, checkpoint_step.clone());

    let column_1: Vec<Element<'a, Message>> = content;
    let column_2: Vec<Element<'a, Message>> = vec![steps_card.into(), submit_card.into()];

    // Fills the max window container space right now, which is pretty good.
    Column::new()
        .push(components::dual_column(column_1, column_2))
        .spacing(Sizes::Xl as u16)
        .padding(Sizes::Xl as u16)
        .height(Length::Fill)
        .width(Length::Fill)
        .into()
}

/// Panel for starting a new transaction.
pub fn starting<'a>(
    input_amount: String,
    address_book: Vec<String>,
    selected: String,
) -> Vec<Element<'a, Message>> {
    let selection = address_book.clone();
    let message_card = message_group(address_book.clone(), selected.clone());
    let data_card = data_group(
        address_book.clone(),
        selected.clone(),
        Some(input_amount.clone()),
        "0".to_string(),
    );

    let column_1: Vec<Element<'a, Message>> = vec![message_card.into(), data_card.into()];
    column_1
}

/// Extended panel for executed simulated diffs
pub fn simulated<'a>(
    selected_to: String,
    selected_target: String,
    input_amount: String,
    review_diffs: Option<StorageDiffs>,
) -> Vec<Element<'a, Message>> {
    let summary_card = summary_group(
        selected_to.clone(),
        selected_target.clone(),
        input_amount.clone(),
    );
    let simulated_card = review_group(review_diffs.clone());

    let column_1: Vec<Element<'a, Message>> = vec![summary_card.into(), simulated_card.into()];
    column_1
}

/// Panel for executed the transaction's state diffs.
pub fn executed<'a>() -> Vec<Element<'a, Message>> {
    let column_1: Vec<Element<'a, Message>> = vec![text("executed".to_string()).into()];
    column_1
}

/// Panel for transaction confirmation.
pub fn confirmed<'a>() -> Vec<Element<'a, Message>> {
    vec![Column::new()
        .push(text("Transaction Confirmed"))
        .push(text("Review the transaction's state diffs."))
        .into()]
}

/// Submit group
pub fn submit_group<'a>(
    action: Message,
    step: TransactionSteps,
    feedback: Option<String>,
    checkpoint: TransactionSteps,
) -> Element<'a, Message> {
    let feedback = match feedback {
        Some(msg) => h5(msg).style(SECONDARY_COLOR),
        None => text(""),
    };

    let title = h2("Instructions".to_string());
    let mut info = text_label(step.get_instructions());
    let mut button = action_button(step.get_cta())
        .padding(Sizes::Md as u16)
        .width(Length::Fill);

    // Disable the button unless its ready.
    match action {
        // todo: this assumes the action is empty if we are at the confirm step.
        Message::Empty => {
            // If we are hit the confirm step, then always render the button as "Exit"
            // stage.
            if checkpoint == TransactionSteps::Confirmed {
                info = text_label(checkpoint.get_instructions());
                button = action_button(checkpoint.get_cta())
                    .on_press(Message::Execution(Execution::Restart))
                    .padding(Sizes::Md as u16)
                    .width(Length::Fill);
            }
        }
        _ => {
            button = button.on_press(action);
        }
    }

    let inner_column = Column::new()
        .push(title)
        .push(info)
        .push(feedback)
        .align_items(alignment::Alignment::Start)
        .spacing(Sizes::Sm as u16)
        .padding(Sizes::Sm as u16)
        .width(Length::Fill);

    let _h: f32 = ByteScale::Xl4.between(&ByteScale::Xl5);

    Card::new(
        Column::new()
            .push(inner_column)
            .push(button)
            .align_items(alignment::Alignment::Center)
            .padding(Sizes::Md as u16)
            .spacing(Sizes::Md as u16),
    )
    .width(Length::Fixed(ByteScale::Xl5.into()))
    .into()
}

/// todo: Style the pick list more.
pub fn message_group<'a>(options: Vec<String>, selected: String) -> Element<'a, Message> {
    let from_input = select_group(
        "From".to_string(),
        options.clone(),
        selected.clone(),
        |value| Message::Execution(view::Execution::FromAddressChanged(value)),
    );

    let to_input = select_group(
        "To".to_string(),
        options.clone(),
        selected.clone(),
        |value| Message::Execution(view::Execution::ToAddressChanged(value)),
    );

    Card::new(
        Column::new()
            .push(from_input)
            .push(to_input)
            .spacing(Sizes::Lg as u16)
            .padding(Sizes::Lg as u16),
    )
    .max_width(ByteScale::Xl6 as u32 as f32)
    .max_height(ByteScale::Xl6 as u32 as f32)
    .into()
}

/// Renders a target contract selection field and an input field for the amount.
pub fn data_group<'a>(
    options: Vec<String>,
    selected: String,
    input_value: Option<String>,
    input_placeholder: String,
) -> Element<'a, Message> {
    let contract_group = select_group("Contract".to_string(), options, selected, |value| {
        Message::Execution(view::Execution::ToAddressChanged(value))
    });

    let amount_group = input_group(
        "Amount".to_string(),
        input_value,
        input_placeholder,
        |value| Message::Execution(view::Execution::AmountChanged(value)),
    );

    let info_container = Container::new(
        Row::new()
            .spacing(Sizes::Md as u16)
            .align_items(alignment::Alignment::Center)
            .push(
                Column::new()
                    .push(text_label("Contract".to_string()))
                    .align_items(alignment::Alignment::Start)
                    .width(Length::Fill),
            )
            .push(
                Column::new()
                    .push(text_label("0x42f0...ffff".to_string()))
                    .align_items(alignment::Alignment::End)
                    .width(Length::Fill),
            ),
    )
    .padding(Sizes::Md as u16)
    .style(InfoContainer::theme());

    Card::new(
        Column::new()
            .push(contract_group)
            .push(amount_group)
            .push(info_container)
            .spacing(Sizes::Lg as u16)
            .padding(Sizes::Lg as u16),
    )
    .max_width(ByteScale::Xl6 as u32 as f32)
    .max_height(ByteScale::Xl7 as u32 as f32)
    .into()
}

pub fn summary_group<'a>(
    selected_to: String,
    selected_target: String,
    input_value: String,
) -> Element<'a, Message> {
    let title = h3("Summary".to_string());

    let table = summary_table(vec![
        ("From".to_string(), selected_to),
        ("To".to_string(), selected_target),
        ("Amount".to_string(), input_value),
    ]);

    let label_text = Row::new()
        .push(text_label("As of block".to_string()))
        .push(text_label("1".to_string()));

    Card::new(
        Column::new()
            .push(title)
            .push(table)
            .push(label_text)
            .spacing(Sizes::Lg as u16)
            .padding(Sizes::Lg as u16),
    )
    .max_width(ByteScale::Xl6 as u32 as f32)
    .max_height(ByteScale::Xl6 as u32 as f32)
    .into()
}

/// Review group
pub fn review_group<'a>(review_diffs: Option<StorageDiffs>) -> Element<'a, Message> {
    let title = h3("Simulation Results".to_string());

    let mut values = vec![];

    // For each storage diff, compute the difference and render a label and diff.
    if let Some(review_diffs) = review_diffs {
        for (slot, (before, after)) in review_diffs.iter() {
            let diff = match (before.clone(), after.clone()) {
                (Some(before), Some(after)) => after.checked_sub(before),
                _ => None,
            };

            match diff {
                Some(diff) => {
                    values.push((slot.to_string(), diff.to_string()));
                }
                None => {}
            }
        }
    }

    let table = summary_table(values);

    let label_text = Row::new()
        .push(text_label("As of block".to_string()))
        .push(text_label("1".to_string()));

    Card::new(
        Column::new()
            .push(title)
            .push(table)
            .push(label_text)
            .spacing(Sizes::Lg as u16)
            .padding(Sizes::Lg as u16),
    )
    .max_width(ByteScale::Xl6 as u32 as f32)
    .max_height(ByteScale::Xl6 as u32 as f32)
    .into()
}

/// Renders the steps in the execution process where the tuple is (Step Icon,
/// Step Name, On press).
pub fn steps_group<'a>(steps: Vec<(Icon, String, Message, bool)>) -> Column<'a, Message> {
    let title = h3("Steps".to_string());
    let mut rows: Vec<Element<'a, Message>> = vec![Column::new().push(title).into()];

    for (icon, item, on_press, current) in steps.into_iter() {
        let mut row = Row::new()
            .spacing(Sizes::Sm as u16)
            .align_items(alignment::Alignment::Center);
        if current {
            row = row.push(
                container(Column::new())
                    .width(Length::Fixed(Sizes::Xs as u32 as f32))
                    .height(Length::Fixed(Sizes::Xl as u32 as f32))
                    .style(Indicator::theme()),
            );
        }

        row = row
            .push(text(icon_to_char(icon)).font(ICON_FONT))
            .push(h3(item));

        let bg_color = match current {
            true => TABLE_COLUMN_BG_COLOR,
            false => Color::TRANSPARENT,
        };

        let mut row = button(row)
            .padding(Sizes::Sm as u16)
            .style(route_button_style(bg_color).as_custom())
            .width(Length::Fill);

        // Disable the button if it has an empty message.
        match on_press {
            Message::Empty => {}
            _ => {
                row = row.on_press(on_press);
            }
        }

        rows.push(row.into());
    }

    Column::with_children(rows).spacing(Sizes::Sm as u16)
}
