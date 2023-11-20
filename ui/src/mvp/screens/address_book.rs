//! Screen for managing the address book.

use super::{app::Message, *};
use crate::mvp::{
    api::contacts::{self, *},
    app::AddressBookMessage,
};

#[derive(Debug, Clone, Default)]
pub struct Form {
    pub address: Option<String>,
    pub label: Option<String>,
    pub category: Option<contacts::Category>,
    pub class: Option<contacts::Class>,
    pub feedback: Option<String>,
}

impl Form {
    pub fn new() -> Self {
        Self {
            address: None,
            label: None,
            category: None,
            class: None,
            feedback: None,
        }
    }

    pub fn clear(&mut self) {
        self.address = None;
        self.label = None;
        self.category = None;
        self.class = None;
        self.feedback = None;
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AddressBookDisplay {
    List,
    Add,
}

pub struct AddressBookScreen {
    pub books: Contacts,
    pub form: Form,
    pub current: contacts::Category,
    pub display: AddressBookDisplay,
}

impl AddressBookScreen {
    pub fn new(books: Contacts) -> Self {
        Self {
            books,
            form: Form::new(),
            current: contacts::Category::Untrusted,
            display: AddressBookDisplay::List,
        }
    }
}

impl State for AddressBookScreen {
    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::View(msg) => match msg {
                view::Message::AddressBook(msg) => match msg {
                    view::AddressBookViewMessage::ChangeDisplay(display) => {
                        self.display = display;
                        Command::none()
                    }
                    view::AddressBookViewMessage::RouteTo(category) => {
                        self.display = AddressBookDisplay::List;
                        self.current = category;
                        Command::none()
                    }
                    view::AddressBookViewMessage::ResetForm => {
                        self.form.clear();
                        Command::none()
                    }
                    view::AddressBookViewMessage::Remove((category, key)) => {
                        self.books.remove(&key, category);
                        Command::none()
                    }
                    view::AddressBookViewMessage::CategoryChanged(category) => {
                        self.form.category = Some(category);
                        Command::none()
                    }
                    view::AddressBookViewMessage::ClassChanged(class) => {
                        self.form.class = Some(class);
                        Command::none()
                    }
                    view::AddressBookViewMessage::AddressChanged(new_address) => {
                        self.form.address = new_address;
                        Command::none()
                    }
                    view::AddressBookViewMessage::LabelChanged(new_label) => {
                        self.form.label = new_label;
                        Command::none()
                    }
                    view::AddressBookViewMessage::Add => {
                        if let Some(address) = self.form.address.clone() {
                            let validated = address.parse::<Address>();

                            if self.form.label.is_none() {
                                self.form.feedback = Some("Label is required.".to_string());
                                return Command::none();
                            }

                            if self.form.category.is_none() {
                                self.form.feedback = Some("Category is required.".to_string());
                                return Command::none();
                            }

                            if self.form.class.is_none() {
                                self.form.feedback = Some("Class is required.".to_string());
                                return Command::none();
                            }

                            match validated {
                                Ok(validated) => {
                                    let label = self.form.label.clone().unwrap();
                                    let contact = ContactValue {
                                        label: label.clone(),
                                        ..Default::default()
                                    };
                                    let category = self.form.category.clone().unwrap();

                                    let value = address;

                                    // Edit the address book.
                                    self.books.add(
                                        validated.clone(),
                                        contact.clone(),
                                        category.clone(),
                                    );

                                    // Provide some feedback.
                                    self.form.feedback = Some(format!(
                                        "Successfully added to {} contacts.",
                                        category.clone()
                                    ));

                                    tracing::info!(
                                        "Added address to address book: {} ({})",
                                        label,
                                        value.to_string()
                                    );
                                    // Clear the latest input and label
                                    self.form.address = None;
                                    self.form.label = None;

                                    return Command::perform(
                                        async move { Ok::<(), ()>(()) },
                                        move |_| {
                                            Message::StorageMessage(
                                                app::StorageMessage::AddressBook(
                                                    AddressBookMessage::Add(
                                                        label, validated, category,
                                                    ),
                                                ),
                                            )
                                        },
                                    );
                                }
                                Err(e) => {
                                    self.form.feedback = Some(e.to_string());
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
                self.form.clone(),
                self.books.clone(),
                self.current.clone(),
                self.display.clone(),
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
