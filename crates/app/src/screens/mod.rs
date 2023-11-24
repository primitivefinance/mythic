//! Traits for implementing new application screens.
use super::{app::Message, *};

pub mod address_book;
pub mod developer;
pub mod empty;
pub mod execution;
pub mod exit;
pub mod experimental;
pub mod terminal;

/// Implement this trait to make a new screen for the app.
pub trait State
where
    Self: Sync + Send,
{
    fn view<'a>(&'a self) -> Element<'a, view::Message>;
    fn update(&mut self, _message: Message) -> Command<Message> {
        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::none()
    }

    fn load(&self) -> Command<Message> {
        Command::none()
    }

    fn exit(&mut self) -> Command<Message> {
        Command::none()
    }
}

/// Wraps anything that implements the State trait into an easier to use struct.
pub struct Screen(pub Box<dyn State>);

impl Screen {
    pub fn new(state: Box<dyn State>) -> Self {
        Self(state)
    }

    pub fn view<'a>(&'a self) -> Element<'a, view::Message> {
        self.0.view()
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        self.0.update(message)
    }

    pub fn subscription(&self) -> Subscription<Message> {
        self.0.subscription()
    }

    pub fn load(&self) -> Command<Message> {
        self.0.load()
    }

    pub fn exit(&mut self) -> Command<Message> {
        self.0.exit()
    }
}
