//! Screen for managing the address book.

use super::{app::Message, *};
use crate::mvp::{
    api::contacts::{self, *},
    app::AddressBookMessage,
};

pub struct AddressBookScreen {
    pub books: Contacts,
    pub new_address: Option<String>,
    pub new_label: Option<String>,
    // For rendering errors or other feedback in the form.
    pub feedback: Option<String>,
}

impl AddressBookScreen {
    pub fn new(books: Contacts) -> Self {
        Self {
            books,
            new_address: None,
            new_label: None,
            feedback: None,
        }
    }
}

impl State for AddressBookScreen {
    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::AddressBook(_) => Command::none(),
            Message::View(msg) => match msg {
                view::Message::AddressBook(msg) => match msg {
                    view::AddressBookViewMessage::AddressChanged(new_address) => {
                        self.new_address = new_address;
                        Command::none()
                    }
                    view::AddressBookViewMessage::LabelChanged(new_label) => {
                        self.new_label = new_label;
                        Command::none()
                    }
                    view::AddressBookViewMessage::Add => {
                        if let Some(address) = self.new_address.clone() {
                            let validated = address.parse::<Address>();

                            if self.new_label.is_none() {
                                self.feedback = Some("Label is required.".to_string());
                                return Command::none();
                            }

                            match validated {
                                Ok(validated) => {
                                    let label = self.new_label.clone().unwrap();
                                    let value = address;

                                    // Edit the address book.
                                    self.books.add(
                                        validated.clone(),
                                        label.clone(),
                                        contacts::Category::Untrusted,
                                    );

                                    // Provide some feedback.
                                    self.feedback =
                                        Some("Successfully added to address book.".to_string());

                                    tracing::info!(
                                        "Added address to address book: {} ({})",
                                        label,
                                        value.to_string()
                                    );
                                    // Clear the latest input and label
                                    self.new_address = None;
                                    self.new_label = None;

                                    return Command::perform(
                                        async move { Ok::<(), ()>(()) },
                                        move |_| {
                                            Message::AddressBook(AddressBookMessage::Add(
                                                label,
                                                validated,
                                                contacts::Category::Untrusted,
                                            ))
                                        },
                                    );
                                }
                                Err(e) => {
                                    self.feedback = Some(e.to_string());
                                    return Command::none();
                                }
                            }
                        }
                        Command::none()
                    }
                },
                _ => Command::none(),
            },
            _ => Command::none(),
        }
    }

    fn view<'a>(&'a self) -> Element<'a, view::Message> {
        view::app_layout(
            &view::Page::AddressBook,
            view::address_book::layout(
                self.books.clone(),
                self.new_address.clone().unwrap_or_default(),
                self.new_label.clone().unwrap_or_default(),
                self.feedback.clone(),
            ),
        )
        .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::none()
    }

    fn load(&self) -> Command<Message> {
        Command::none()
    }
}
