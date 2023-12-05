//! Contacts are all the addresses that can be interacted with in the app.

use super::*;

#[derive(Debug, Clone, Default)]
pub enum Message {
    #[default]
    Empty,
}

impl MessageWrapper for Message {
    type ParentMessage = super::Message;
}

impl MessageWrapperView for Message {
    type ParentMessage = super::Message;
}

impl From<Message> for <Message as MessageWrapper>::ParentMessage {
    fn from(message: Message) -> Self {
        Self::Contacts(message)
    }
}

pub struct ContactsManagement;

impl ContactsManagement {
    pub fn new() -> Self {
        Self
    }
}

impl State for ContactsManagement {
    type AppMessage = Message;
    type ViewMessage = Message;

    fn load(&self) -> Command<Self::AppMessage> {
        Command::none()
    }

    fn update(&mut self, _message: Self::AppMessage) -> Command<Self::AppMessage> {
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::ViewMessage> {
        Container::new(Column::new().push(h2("Select an app to get started.".to_string())))
            .center_x()
            .center_y()
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
