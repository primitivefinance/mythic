//! Signers are any entity that can sign and execute transactions.
//! These signers can be used within the app.

use super::*;
use crate::components::system::label;

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
        Self::Signers(message)
    }
}

pub struct SignerManagement;

impl SignerManagement {
    pub fn new() -> Self {
        Self
    }
}

impl State for SignerManagement {
    type AppMessage = Message;
    type ViewMessage = Message;

    fn load(&self) -> Command<Self::AppMessage> {
        Command::none()
    }

    fn update(&mut self, _message: Self::AppMessage) -> Command<Self::AppMessage> {
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::ViewMessage> {
        Container::new(Column::new().push(label(&"Select an app to get started.").title2().build()))
            .center_x()
            .center_y()
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
