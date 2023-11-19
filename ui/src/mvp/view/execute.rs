//! Views for executing transactions.

use iced::{widget::pick_list, BorderRadius, Color};
use iced_aw::{graphics::icons::icon_to_char, Icon, ICON_FONT};

use super::{components::input::create_input_component, text, Column, Element, Message, *};
use crate::mvp::{
    components::button::CustomButtonStyle, execution::TransactionSteps,
    screens::execution::StorageDiffs, units::address_to_string,
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
        TransactionSteps::Start if !input_amount.is_empty() && selected.len() > 0 => {
            Message::Execution(Execution::Next)
        }
        TransactionSteps::Simulated if !pending => Message::Execution(Execution::Next),
        TransactionSteps::Executed if !pending => Message::Execution(Execution::Next),
        TransactionSteps::Confirmed if !pending => Message::Execution(Execution::Next),
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
    let submit_card = submit_group(action, step.clone(), user_feedback);

    let column_1: Vec<Element<'a, Message>> = content;
    let column_2: Vec<Element<'a, Message>> = vec![steps_card.into(), submit_card.into()];

    Column::new()
        .push(
            Row::new()
                .push(
                    Column::new()
                        .push(components::dual_column(column_1, column_2))
                        .spacing(16)
                        .padding(32)
                        .width(Length::Fill),
                )
                .height(Length::FillPortion(5)),
        )
        .spacing(16)
        .into()
}

/// Panel for starting a new transaction.
pub fn starting<'a>(
    input_amount: String,
    address_book: Vec<String>,
    selected: String,
) -> Vec<Element<'a, Message>> {
    let title = data_item("Execution".to_string()).size(36);
    let selection = address_book.clone();
    let message_card = message_group(address_book.clone(), selected.clone());
    let data_card = data_group(
        address_book.clone(),
        selected.clone(),
        Some(input_amount.clone()),
        "0".to_string(),
    );

    let column_1: Vec<Element<'a, Message>> =
        vec![title.into(), message_card.into(), data_card.into()];

    column_1
}

/// Extended panel for executed simulated diffs
pub fn simulated<'a>(
    selected_to: String,
    selected_target: String,
    input_amount: String,
    review_diffs: Option<StorageDiffs>,
) -> Vec<Element<'a, Message>> {
    let title = data_item("Review Transaction".to_string()).size(36);
    let summary_card = summary_group(
        selected_to.clone(),
        selected_target.clone(),
        input_amount.clone(),
    );
    let simulated_card = review_group(review_diffs.clone());

    let column_1: Vec<Element<'a, Message>> =
        vec![title.into(), summary_card.into(), simulated_card.into()];
    column_1
}

/// Panel for executed the transaction's state diffs.
pub fn executed<'a>() -> Vec<Element<'a, Message>> {
    let title = data_item("Execute Transaction".to_string()).size(36);

    let column_1: Vec<Element<'a, Message>> = vec![title.into()];
    column_1
}

/// Panel for transaction confirmation.
pub fn confirmed<'a>() -> Vec<Element<'a, Message>> {
    vec![Column::new()
        .push(text("Transaction Confirmed"))
        .push(text("Review the transaction's state diffs."))
        .into()]
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
pub fn submit_group<'a>(
    action: Message,
    step: TransactionSteps,
    feedback: Option<String>,
) -> Element<'a, Message> {
    let title = match step {
        TransactionSteps::Start => "Build".to_string(),
        TransactionSteps::Simulated => "Simulate".to_string(),
        TransactionSteps::Executed => "Execute".to_string(),
        TransactionSteps::Confirmed => "Confirm".to_string(),
    };

    let button_cta = match step {
        TransactionSteps::Start => "Review".to_string(),
        TransactionSteps::Simulated => "Execute".to_string(),
        TransactionSteps::Executed => "Confirming...".to_string(),
        TransactionSteps::Confirmed => "Exit".to_string(),
    };

    let extra_info = match step {
        TransactionSteps::Start => ("Awaiting review...").to_string(),
        TransactionSteps::Simulated => ("Awaiting execution...").to_string(),
        TransactionSteps::Executed => ("Awaiting transaction...").to_string(),
        TransactionSteps::Confirmed => ("Transaction confirmed.").to_string(),
    };

    let feedback = match feedback {
        Some(msg) => h5(msg),
        None => text(""),
    };

    let title = h2(title);
    let info = text_label(extra_info);
    let mut button = action_button(button_cta)
        .padding(Sizes::Md as u16)
        .width(Length::Fill);

    // Disable the button unless its ready.
    match action {
        Message::Empty => {}
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
        .width(Length::Fill)
        .height(Length::Fill);

    let h: f32 = ByteScale::Xl4.between(&ByteScale::Xl6);

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

/// Column with a label and pick list field.
/// todo: add a message argument
pub fn select_group<'a>(
    title: String,
    options: Vec<String>,
    selected: String,
    on_selected: impl Fn(String) -> Message + 'a,
) -> Element<'a, Message> {
    let title = h3(title.to_string());
    let input = pick_list(options, Some(selected.clone()), on_selected).padding(Sizes::Md as u16);

    let input_container = Container::new(input).style(MenuContainerTheme::theme());

    Column::new()
        .push(title)
        .push(input_container)
        .spacing(Sizes::Md as u16)
        .into()
}

/// Column with a label and text input field.
pub fn input_group<'a>(
    title: String,
    value: Option<String>,
    placeholder: String,
    on_change: impl Fn(Option<String>) -> Message + 'static,
) -> Element<'a, Message> {
    let title = h3(title.to_string());
    // todo: change this so padding is modifiable.
    let input = create_input_component(value, on_change);

    Column::new()
        .push(title)
        .push(input)
        .spacing(Sizes::Md as u16)
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
            )
            .width(Length::Fill),
    )
    .width(Length::Fill)
    .height(Length::Fill)
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

/// Renders a column in a summary table's row.
pub fn summary_column<'a>(value: String) -> Column<'a, Message> {
    Column::new()
        .push(h4(value))
        .align_items(alignment::Alignment::Center)
        .padding(Sizes::Md as u16)
        .width(Length::FillPortion(2))
}

/// Renders a row in a summary table.
/// If top row, render with top-left border radius.
/// If bottom row, render with bottom-left border radius.
/// Otherwise, render with no border radius.
/// todo: border radius is affected by being an inner border, doesn't look the
/// greatest unfortunately.
pub fn summary_row<'a>(
    columns: Vec<Column<'a, Message>>,
    row_position: usize,
    total_rows: usize,
) -> Container<'a, Message> {
    let col_radius: BorderRadius = match row_position {
        0 => [5.0, 0.0, 0.0, 0.0].into(),
        _ if row_position == total_rows - 1 => [0.0, 0.0, 0.0, 5.0].into(),
        _ => [0.0, 0.0, 0.0, 0.0].into(),
    };

    let col_radius = match total_rows {
        1 => [5.0, 0.0, 0.0, 5.0].into(),
        _ => col_radius,
    };

    // Edit the first column by wrapping it with a InfoContainer
    let mut columns = columns;
    let first_column = columns.remove(0);
    let first_column = Container::new(first_column)
        .style(TableColumnContainer::theme_with_border_radius(col_radius))
        .width(Length::Fill);
    columns.insert(0, Column::new().push(first_column).width(Length::Fill));

    // If top row, top left + right border radius.
    // If bottom row, bottom left + right border radius.
    // Otherwise, no border radius.
    let row_radius: BorderRadius = match row_position {
        0 => [5.0, 5.0, 0.0, 0.0].into(),
        _ if row_position == total_rows - 1 => [0.0, 0.0, 5.0, 5.0].into(),
        _ => [0.0, 0.0, 0.0, 0.0].into(),
    };

    // Override radius if only one row.
    let row_radius = match total_rows {
        1 => 5.0.into(),
        _ => row_radius,
    };

    Container::new(
        Row::with_children(
            columns
                .into_iter()
                .map(|c| c.into())
                .collect::<Vec<Element<'a, Message>>>(),
        )
        .align_items(alignment::Alignment::Center)
        .width(Length::Fill),
    )
    .style(BorderedContainer::theme_with_border_radius(row_radius))
    .into()
}

pub fn summary_table<'a>(values: Vec<(String, String)>) -> Container<'a, Message> {
    // If values has no values, just render centered text in the container "No
    // changes."
    if values.is_empty() {
        return Container::new(
            Row::new()
                .push(
                    Column::new()
                        .push(text("No changes."))
                        .align_items(alignment::Alignment::Center),
                )
                .align_items(alignment::Alignment::Center),
        )
        .center_x()
        .center_y()
        .style(BorderedContainer::theme());
    }

    let mut rows: Vec<Element<'a, Message>> = vec![];

    let total_rows = values.len();
    for (i, (label, value)) in values.into_iter().enumerate() {
        let columns: Vec<Column<'a, Message>> = vec![summary_column(label), summary_column(value)];
        let row = summary_row(columns, i, total_rows);
        rows.push(row.into());
    }

    Container::new(Column::with_children(rows))
        .style(BorderedContainer::theme())
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

        let style = CustomButtonStyle::new()
            .text_color(Color::WHITE)
            .border_radius(3.0.into())
            .background_color(bg_color)
            .pressed()
            .text_color(Color::WHITE)
            .border_radius(3.0.into())
            .background_color(TABLE_COLUMN_BG_COLOR)
            .hovered()
            .text_color(Color::WHITE)
            .border_radius(3.0.into())
            .background_color(CARD_BG_COLOR)
            .disabled()
            .text_color(DISABLED_COLOR)
            .border_radius(3.0.into());
        let mut row = button(row)
            .padding(Sizes::Sm as u16)
            .style(style.as_custom())
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

    Column::with_children(rows)
        .spacing(Sizes::Sm as u16)
        .width(Length::Fill)
}
