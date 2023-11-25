//! Traits for implementing new application screens.

use super::{app::Message, *};

pub mod address_book;
pub mod developer;
pub mod empty;
pub mod execution;
pub mod exit;
pub mod experimental;
pub mod portfolio;
pub mod terminal;

/// Implement this trait to create new application states, from entire windows
/// to individual screens inside windows. This trait is used recursively to
/// build the entire application.

pub trait State
where
    Self: Sync + Send,
{
    /// Messages returned by the implemented view method.
    /// There exists a global "View" message as a variant in the
    /// main application message enum.
    /// todo: type defaults are unstable right now...
    type ViewMessage = view::Message;

    /// Messages returned to be executed by commands.
    /// Defaults to the global Application Message.
    /// All of these messages get dropped to the top of the stack (the main
    /// application), and piped back down to the screen that returned the
    /// message.
    /// todo: type defaults are unstable right now...
    type AppMessage = app::Message;

    /// Renders the screen which can produce [`ViewMessage`]s.
    fn view<'a>(&'a self) -> Element<'a, Self::ViewMessage>;

    /// Updates the screen with an [`AppMessage`] and returns a [`Command`]
    fn update(&mut self, _message: Self::AppMessage) -> Command<Self::AppMessage> {
        Command::none()
    }

    /// Handles any subscriptions for the screen.
    fn subscription(&self) -> Subscription<Self::AppMessage> {
        Subscription::none()
    }

    /// Triggered when creating the screen's state usually.
    fn load(&self) -> Command<Self::AppMessage> {
        Command::none()
    }

    /// Triggered on closing the screen, usually.
    fn exit(&mut self) -> Command<Self::AppMessage> {
        Command::none()
    }
}

/// Alias for the type of screen.
/// Windows are like tabs in a browser.
type WindowScreen = dyn State<ViewMessage = view::Message, AppMessage = app::Message>;

/// Wraps anything that implements WindowScreen type of State trait into an
/// easier to use struct.
pub struct Screen(pub Box<WindowScreen>);

impl Screen {
    pub fn new(state: Box<WindowScreen>) -> Self {
        Self(state)
    }

    pub fn view<'a>(&'a self) -> Element<'a, <WindowScreen as State>::ViewMessage> {
        self.0.view()
    }

    pub fn update(
        &mut self,
        message: <WindowScreen as State>::AppMessage,
    ) -> Command<<WindowScreen as State>::AppMessage> {
        self.0.update(message)
    }

    pub fn subscription(&self) -> Subscription<<WindowScreen as State>::AppMessage> {
        self.0.subscription()
    }

    pub fn load(&self) -> Command<<WindowScreen as State>::AppMessage> {
        self.0.load()
    }

    pub fn exit(&mut self) -> Command<<WindowScreen as State>::AppMessage> {
        self.0.exit()
    }
}
