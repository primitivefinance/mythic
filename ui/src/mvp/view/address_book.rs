use super::{components::input::create_input_component, Message, *};
use crate::mvp::api::contacts::{self, ContactList, Contacts};

pub fn layout<'a>(
    books: Contacts,
    last_address: String,
    last_label: String,
    feedback: Option<String>,
) -> Element<'a, Message> {
    let mut content = Column::new()
        .push(data_item("Address Book".to_string()).size(28))
        .push(add_address(last_label, last_address));

    if feedback.is_some() {
        content = content.push(text(feedback.unwrap()).size(12));
    }

    if let Some(list) = books.get_list(contacts::Category::Untrusted) {
        content = content.push(list_untrusted_book(list.clone()));
    }

    content.padding(32).spacing(16).into()
}

/// Renders a text input and button for adding a new address.
/// todo: only supports adding to untrusted categories, upgrade in future.
pub fn add_address<'a>(label: String, address: String) -> Element<'a, Message> {
    let label_input = create_input_component(Some(label), |value| {
        Message::AddressBook(AddressBookViewMessage::LabelChanged(value))
    });

    let address_input = create_input_component(Some(address), |value| {
        Message::AddressBook(AddressBookViewMessage::AddressChanged(value))
    });

    let submit =
        button(text("add address")).on_press(Message::AddressBook(AddressBookViewMessage::Add));

    Row::new()
        .push(label_input)
        .push(address_input)
        .push(submit)
        .spacing(8)
        .into()
}

pub fn list_untrusted_book<'a>(book: ContactList) -> Element<'a, Message> {
    let mut content = Column::new()
        .push(text("Untrusted Addresses".to_string()).size(18))
        .spacing(16);

    let sorted_addresses = book.get_all();
    for (address, contact) in sorted_addresses {
        let row = Row::new()
            .push(label_item(contact.label.clone()))
            .push(text(address.to_string()))
            .spacing(8);
        content = content.push(row);
    }

    content.into()
}
