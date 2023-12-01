use std::borrow::Cow;

use iced::Color;
use iced_aw::{graphics::icons::icon_to_char, ICON_FONT};
use user::contacts::{self, Contacts};

use super::{
    components::{button::route_button_style, tables::summary_table},
    screens::address_book::{AddressBookDisplay, Form},
    Message, *,
};

pub fn layout<'a>(
    form: screens::address_book::Form,
    books: Contacts,
    current: contacts::Category,
    display: AddressBookDisplay,
) -> Element<'a, Message> {
    let on_list_display = matches!(display, AddressBookDisplay::List);

    let routes = vec![
        (
            Icon::PersonDash,
            "Untrusted".to_string(),
            Message::AddressBook(AddressBookViewMessage::RouteTo(
                contacts::Category::Untrusted,
            )),
            current == contacts::Category::Untrusted && on_list_display,
        ),
        (
            Icon::PersonPlus,
            "Trusted".to_string(),
            Message::AddressBook(AddressBookViewMessage::RouteTo(contacts::Category::Trusted)),
            current == contacts::Category::Trusted && on_list_display,
        ),
        (
            Icon::PersonX,
            "Blocked".to_string(),
            Message::AddressBook(AddressBookViewMessage::RouteTo(contacts::Category::Blocked)),
            current == contacts::Category::Blocked && on_list_display,
        ),
        (
            Icon::Clock,
            "Recent".to_string(),
            Message::AddressBook(AddressBookViewMessage::RouteTo(contacts::Category::Recent)),
            current == contacts::Category::Recent && on_list_display,
        ),
        (
            Icon::Plus,
            "Add Contact".to_string(),
            Message::AddressBook(AddressBookViewMessage::ChangeDisplay(
                AddressBookDisplay::Add,
            )),
            display == AddressBookDisplay::Add,
        ),
    ];

    let contact_group = Column::new()
        .push(contact_directory(routes))
        .width(Length::FillPortion(1))
        .padding(Sizes::Md as u16);

    let mut action_group = Column::new()
        .width(Length::FillPortion(2))
        .padding(Sizes::Md as u16)
        .spacing(Sizes::Md as u16)
        .align_items(alignment::Alignment::End);

    match display {
        AddressBookDisplay::List => {
            let card = list_contact_card(books.clone(), current.clone());
            action_group = action_group.push(card);
        }
        AddressBookDisplay::Add => {
            let card = add_contact_card(form.clone());
            action_group = action_group.push(card);
        }
    }

    Row::new()
        .push(contact_group)
        .push(action_group)
        .padding(Sizes::Sm as u16)
        .spacing(Sizes::Sm as u16)
        .into()
}

pub fn list_contact_card<'a>(
    books: Contacts,
    category: contacts::Category,
) -> Element<'a, Message> {
    let mut values: Vec<(String, String)> = vec![];

    let list = books.get_list(category.clone());
    if let Some(list) = list {
        for (address, contact) in list.get_all() {
            values.push((contact.label.clone(), format!("0x{:x}", address)));
        }
    }

    let table = summary_table(values)
        .width(Length::Fill)
        .height(Length::Fill);

    let content = Column::new()
        .push(h2(format!("{} Contacts", category)))
        .push(table)
        .padding(Sizes::Md as u16)
        .spacing(Sizes::Md as u16);

    Card::new(content)
        .max_width(ByteScale::Xl6 as u32 as f32)
        .max_height(ByteScale::Xl6 as u32 as f32)
        .into()
}

pub fn contact_directory<'a>(routes: Vec<(Icon, String, Message, bool)>) -> Column<'a, Message> {
    let mut rows: Vec<Element<'a, Message>> = vec![Row::new()
        .push(
            Column::new()
                .push(highlight_label("Contact Lists".to_string()))
                .align_items(alignment::Alignment::Center),
        )
        .padding(Sizes::Sm as u16)
        .spacing(Sizes::Sm as u16)
        .align_items(alignment::Alignment::Center)
        .width(Length::Fill)
        .into()];

    for (icon, item, on_press, current) in routes.into_iter() {
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

pub fn add_contact_card<'a>(form: Form) -> Element<'a, Message> {
    let title = h2("Add Contact".to_string());

    let label_input = input_group(
        "Label".to_string(),
        form.label,
        "Enter a label for the contact".to_string(),
        |value| Message::AddressBook(AddressBookViewMessage::LabelChanged(value)),
    );

    let address_input = input_group(
        "Address".to_string(),
        form.address,
        "Enter the address for the contact".to_string(),
        |value| Message::AddressBook(AddressBookViewMessage::AddressChanged(value)),
    );

    let category_select = select_group(
        "Category".to_string(),
        contacts::Category::all(),
        form.category,
        |value| Message::AddressBook(AddressBookViewMessage::CategoryChanged(value)),
    );

    let class_select = select_group(
        "Class".to_string(),
        contacts::Class::all(),
        form.class,
        |value| Message::AddressBook(AddressBookViewMessage::ClassChanged(value)),
    );

    let instructions = highlight_label(
        "Enter the label and address for the new contact, and select the category.".to_string(),
    );
    let mut info_column = Column::new()
        .push(h3("Instructions".to_string()))
        .push(instructions)
        .spacing(Sizes::Sm as u16);

    if form.feedback.is_some() {
        info_column = info_column.push(text(form.feedback.unwrap()).size(12));
    }

    let submit = button(
        text("Add Contact")
            .vertical_alignment(alignment::Vertical::Center)
            .horizontal_alignment(alignment::Horizontal::Center),
    )
    .on_press(Message::AddressBook(AddressBookViewMessage::Add))
    .padding(Sizes::Md as u16);

    let content = Column::new()
        .push(title)
        .push(
            Row::new()
                .push(label_input.width(Length::FillPortion(2)))
                .push(address_input.width(Length::FillPortion(2)))
                .spacing(Sizes::Sm as u16),
        )
        .push(
            Row::new()
                .push(category_select.width(Length::FillPortion(2)))
                .push(class_select.width(Length::FillPortion(2)))
                .spacing(Sizes::Sm as u16),
        )
        .push(
            Row::new()
                .push(info_column.width(Length::FillPortion(2)))
                .push(submit.width(Length::FillPortion(2)))
                .spacing(Sizes::Sm as u16)
                .align_items(alignment::Alignment::Center),
        )
        .spacing(Sizes::Md as u16)
        .padding(Sizes::Lg as u16);

    Card::new(content)
        .max_width(ByteScale::Xl6 as u32 as f32)
        .max_height(ByteScale::Xl6 as u32 as f32)
        .into()
}

pub fn select_group<'a, T>(
    title: String,
    options: impl Into<Cow<'a, [T]>>,
    selected: Option<T>,
    on_selected: impl Fn(T) -> Message + 'a,
) -> Column<'a, Message>
where
    [T]: ToOwned<Owned = Vec<T>>,
    T: Clone + ToString + Eq + 'static,
{
    Column::new()
        .push(h3(title))
        .push(
            components::select::custom_pick_list(
                options,
                selected,
                on_selected,
                Some("Select option".to_string()),
            )
            .padding(Sizes::Md as u16)
            .width(Length::Fill),
        )
        .width(Length::Shrink)
        .spacing(Sizes::Md as u16)
}
