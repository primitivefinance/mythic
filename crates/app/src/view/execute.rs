//! Views for executing transactions.

use datatypes::units::address_to_string;
use iced::{widget::progress_bar, Color};
use iced_aw::{graphics::icons::icon_to_char, Icon, ICON_FONT};
use model::contacts::ContactList;

use super::{
    components::button::route_button_style,
    execution::form::TransactionSteps,
    controller::execution::{form::FormMessage, processing::StorageDiffs},
    text, Column, Element, Message, *,
};

/// `execution_layout` is a function that creates a layout for the execution of transactions.
///
/// # Arguments
///
/// * `form` - A reference to the form object that contains the transaction details.
/// * `from_list` - A reference to the list of contacts from which the transaction can be made.
/// * `to_list` - A reference to the list of contacts to which the transaction can be made.
/// * `target_list` - A reference to the list of contacts that can be targeted by the transaction.
/// * `sim_results` - An optional parameter that contains the results of the simulation of the transaction.
/// * `execute_results` - An optional parameter that contains the results of the execution of the transaction.
///
/// # Returns
///
/// This function returns an Element that represents the layout of the execution of transactions.
///
/// The layout is a column that contains the form view. The column fills the maximum window container space,
/// has a spacing and padding of `Sizes::Xl`, and has a height and width that fill the available space.
pub fn execution_layout<'a>(
    form: &execution::form::Form,
    from_list: &ContactList,
    to_list: &ContactList,
    target_list: &ContactList,
    sim_results: Option<StorageDiffs>,
    execute_results: Option<StorageDiffs>,
) -> Element<'a, Message> {
    Column::new()
        .push(form.view(
            from_list,
            to_list,
            target_list,
            sim_results,
            execute_results,
        ))
        .spacing(Sizes::Xl as u16)
        .padding(Sizes::Xl as u16)
        .height(Length::Fill)
        .width(Length::Fill)
        .into()
}

/// Panel for starting a new transaction.
pub fn starting<'a>(
    options_from: &ContactList,
    from: Option<String>,
    options_to: &ContactList,
    to: Option<String>,
    options_target: &ContactList,
    target: Option<String>,
    amount: Option<String>,
) -> Vec<Element<'a, Message>> {
    let message_card = message_group(options_from, from.clone(), options_to, to.clone());
    let data_card = data_group(
        options_target,
        target.clone(),
        amount.clone(),
        "0".to_string(),
    );

    let column_1: Vec<Element<'a, Message>> = vec![message_card, data_card];
    column_1
}

/// Extended panel for executed simulated diffs
pub fn simulated<'a>(
    selected_to: Option<String>,
    selected_target: Option<String>,
    input_amount: Option<String>,
    review_diffs: Option<StorageDiffs>,
) -> Vec<Element<'a, Message>> {
    let summary_card = summary_group(
        selected_to.clone(),
        selected_target.clone(),
        input_amount.clone(),
    );
    let simulated_card = review_group(review_diffs.clone());

    let column_1: Vec<Element<'a, Message>> = vec![summary_card, simulated_card];
    column_1
}

/// Panel for executed the transaction's state diffs.
pub fn executed<'a>(
    selected_to: Option<String>,
    selected_target: Option<String>,
    input_amount: Option<String>,
) -> Vec<Element<'a, Message>> {
    let summary_card = summary_group(
        selected_to.clone(),
        selected_target.clone(),
        input_amount.clone(),
    );

    let label = h3("Pending transaction...".to_string());
    let progress = progress_bar(0.0..=100.0, 20.0);

    let column_1: Vec<Element<'a, Message>> = vec![summary_card, label.into(), progress.into()];

    column_1
}

/// Panel for transaction confirmation.
pub fn confirmed<'a>(
    selected_to: Option<String>,
    selected_target: Option<String>,
    input_amount: Option<String>,
    review_diffs: Option<StorageDiffs>,
    block: u64,
) -> Vec<Element<'a, Message>> {
    let title = h3("Execution Results".to_string());
    let table = state_deltas_table(review_diffs);
    let label_text = Row::new()
        .push(highlight_label("As of block".to_string()))
        .push(highlight_label(block.to_string()));

    let summary_card = summary_group(
        selected_to.clone(),
        selected_target.clone(),
        input_amount.clone(),
    );

    let results_card = Card::new(
        Column::new()
            .push(title)
            .push(table)
            .push(label_text)
            .spacing(Sizes::Lg as u16)
            .padding(Sizes::Lg as u16),
    )
    .max_width(ByteScale::Xl6 as u32 as f32)
    .max_height(ByteScale::Xl6 as u32 as f32);

    let label = h3("Transaction complete.".to_string());
    let progress = progress_bar(0.0..=100.0, 100.0);

    let column_1: Vec<Element<'a, Message>> = vec![
        summary_card,
        results_card.into(),
        label.into(),
        progress.into(),
    ];

    column_1
}

/// Submit group
pub fn submit_group<'a>(
    action: Message,
    feedback: Option<String>,
    checkpoint: TransactionSteps,
) -> Element<'a, Message> {
    let feedback = match feedback {
        Some(msg) => highlight_label(msg),
        None => text(""),
    };

    let title = h2("Instructions".to_string());
    let mut info = secondary_label(checkpoint.get_instructions());
    let mut button = action_button(checkpoint.get_cta())
        .padding(Sizes::Md as u16)
        .width(Length::Fill);

    // Disable the button unless its ready.
    match action {
        // todo: this assumes the action is empty if we are at the confirm step.
        Message::Empty => {
            // If we are hit the confirm step, then always render the button as "Exit"
            // stage.
            if checkpoint == TransactionSteps::Confirmed {
                info = secondary_label(checkpoint.get_instructions());
                button = action_button(checkpoint.get_cta())
                    .on_press(Execution::Reset.into())
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
        .align_items(alignment::Alignment::Start)
        .spacing(Sizes::Sm as u16)
        .padding(Sizes::Sm as u16)
        .width(Length::Fill);

    let _h: f32 = ByteScale::Xl4.between(&ByteScale::Xl5);

    Card::new(
        Column::new()
            .push(inner_column)
            .push(button)
            .push(
                Column::new()
                    .push(feedback)
                    .spacing(Sizes::Sm as u16)
                    .padding(Sizes::Sm as u16),
            )
            .padding(Sizes::Md as u16)
            .spacing(Sizes::Md as u16),
    )
    .into()
}

/// todo: Style the pick list more.
#[allow(suspicious_double_ref_op)]
pub fn message_group<'a>(
    options_from: &ContactList,
    selected_from: Option<String>,
    options_to: &ContactList,
    selected_to: Option<String>,
) -> Element<'a, Message> {
    // Build a reverse map, so we can easily look up the address from the label.
    let from_map: HashMap<String, Address> = options_from
        .get_all()
        .iter()
        .map(|(key, contact)| (contact.label.clone(), *key.clone()))
        .collect();

    // Get the labels, which are now the keys.
    let from_labels: Vec<String> = from_map.keys().cloned().collect();

    // Try to get the selected value as a label from the options.
    let from = options_from.try_get(&selected_from.unwrap_or_default());
    let from = match from {
        Ok(from) => Some(from.label.clone()),
        Err(_) => None,
    };

    let from_input = select_group("From".to_string(), from_labels, from, move |label| {
        let value = from_map.get(&label);
        let value = match value {
            Some(value) => *value,
            None => Address::default(),
        };
        FormMessage::ChangeFrom(address_to_string(&value)).into()
    });

    // Build a reverse map, so we can easily look up the address from the label.
    let to_map: HashMap<String, Address> = options_to
        .get_all()
        .iter()
        .map(|(key, contact)| (contact.label.clone(), *key.clone()))
        .collect();

    // Get the labels, which are now the keys.
    let to_labels: Vec<String> = to_map.keys().cloned().collect();

    // Try to get the selected value as a label from the options.
    let to = options_to.try_get(&selected_to.unwrap_or_default());
    let to = match to {
        Ok(to) => Some(to.label.clone()),
        Err(_) => None,
    };

    let to_input = select_group("To".to_string(), to_labels, to, move |label| {
        let value = to_map.get(&label);
        let value = match value {
            Some(value) => *value,
            None => Address::default(),
        };
        FormMessage::ChangeTo(address_to_string(&value)).into()
    });

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
#[allow(suspicious_double_ref_op)]
pub fn data_group<'a>(
    options: &ContactList,
    selected: Option<String>,
    input_value: Option<String>,
    input_placeholder: String,
) -> Element<'a, Message> {
    // Build a reverse map, so we can easily look up the address from the label.
    let contract_map: HashMap<String, Address> = options
        .get_all()
        .iter()
        .map(|(key, contact)| (contact.label.clone(), *key.clone()))
        .collect();

    // Get the labels, which are now the keys.
    let contract_labels: Vec<String> = contract_map.keys().cloned().collect();

    // Try to get the selected value as a label from the options.
    let contract = options.try_get(&selected.unwrap_or_default());
    let contract = match contract {
        Ok(contract) => Some(contract.label.clone()),
        Err(_) => None,
    };

    let contract_group = select_group(
        "Contract".to_string(),
        contract_labels,
        contract,
        move |label| {
            let value = contract_map.get(&label);
            let value = match value {
                Some(value) => *value,
                None => Address::default(),
            };
            FormMessage::ChangeTarget(address_to_string(&value)).into()
        },
    );

    let amount_group = input_group(
        "Amount".to_string(),
        input_value,
        input_placeholder,
        |value| FormMessage::ChangeAmount(value).into(),
    );

    let info_container = Container::new(
        Row::new()
            .spacing(Sizes::Md as u16)
            .align_items(alignment::Alignment::Center)
            .push(
                Column::new()
                    .push(highlight_label("Contract".to_string()))
                    .align_items(alignment::Alignment::Start)
                    .width(Length::Fill),
            )
            .push(
                Column::new()
                    .push(highlight_label("0x42f0...ffff".to_string()))
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
    selected_to: Option<String>,
    selected_target: Option<String>,
    input_value: Option<String>,
) -> Element<'a, Message> {
    let title = h3("Summary".to_string());

    let table = summary_table(vec![
        ("From".to_string(), selected_to.unwrap_or_default()),
        ("To".to_string(), selected_target.unwrap_or_default()),
        ("Amount".to_string(), input_value.unwrap_or_default()),
    ]);

    let label_text = Row::new()
        .push(highlight_label("As of block".to_string()))
        .push(highlight_label("1".to_string()));

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
    let table = state_deltas_table(review_diffs);
    let label_text = Row::new()
        .push(highlight_label("As of block".to_string()))
        .push(highlight_label("1".to_string()));

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

pub fn state_deltas_table<'a>(review_diffs: Option<StorageDiffs>) -> Container<'a, Message> {
    let mut values = vec![];

    // For each storage diff, compute the difference and render a label and diff.
    if let Some(review_diffs) = review_diffs {
        for (slot, (before, after)) in review_diffs.iter() {
            let diff = match (*before, *after) {
                (Some(before), Some(after)) => {
                    // If after is greater than before, subtract to get the diff.
                    if after > before {
                        let diff = after.checked_sub(before);
                        if diff.is_none() {
                            tracing::error!("Could not compute diff for storage slot: {:?}", slot);
                            continue;
                        }

                        Some(format!("+{}", diff.unwrap()))
                    } else {
                        let diff = before.checked_sub(after);
                        if diff.is_none() {
                            tracing::error!("Could not compute diff for storage slot: {:?}", slot);
                            continue;
                        }

                        Some(format!("-{}", diff.unwrap()))
                    }
                }
                _ => None,
            };

            if let Some(diff) = diff {
                values.push((slot.to_string(), diff));
            }
        }
    }

    let table = summary_table(values);
    table
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
            true => SELECTED_CONTAINER_COLOR,
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
