//! Most important traits of the application.
//!
//! Traits for implementing new application screens.
//!
//! Q: Why do we have a `ViewMessage` and an `AppMessage`?
//! A: The `ViewMessage` implements `Clone` and the `AppMessage` does
//! not.

use super::*;

pub mod empty;
pub mod exit;
pub mod portfolio;
pub mod settings;
pub mod terminal;

/// All messages need to be wrapped in a message type that their parent
/// supports, this trait enforces that.
/// This enables us to call a child component which returns it's child method,
/// but then wrap it to the parent message, which the parent method returns.
///
/// For example:
/// ```rust
/// type Command<T> = iced::Command<T>;
///
/// trait MessageWrapper: Sized {
///     type ParentMessage: From<Self>;
/// }
///
/// trait State: Sized {
///     type AppMessage: MessageWrapper;
///
///     fn update(&mut self, message: Self::AppMessage) -> Command<Self::AppMessage>;
/// }
///
/// pub enum ChildMessage {
///     HelloWorld,
/// }
///
/// pub enum ParentMessage {
///     Child(ChildMessage),
/// }
///
/// impl MessageWrapper for ParentMessage {
///     type ParentMessage = Self;
/// }
///
/// impl From<ChildMessage> for ParentMessage {
///     fn from(message: ChildMessage) -> Self {
///         Self::Child(message)
///     }
/// }
///
/// impl MessageWrapper for ChildMessage {
///     type ParentMessage = ParentMessage;
/// }
///
/// pub struct Parent {
///     child: Child,
/// }
///
/// pub struct Child;
///
/// impl State for Child {
///     type AppMessage = ChildMessage;
///
///     fn update(&mut self, message: Self::AppMessage) -> Command<Self::AppMessage> {
///         println!("Hello world!");
///         Command::none()
///     }
/// }
///
/// impl State for Parent {
///     type AppMessage = ParentMessage;
///
///     fn update(&mut self, message: Self::AppMessage) -> Command<Self::AppMessage> {
///         match message {
///             ParentMessage::Child(message) => self.child.update(message).map(|x| x.into()),
///         }
///     }
/// }
/// ```
///
/// Note that `self.child.update()` returns a `Command<ChildMessage>`, but we
/// wrap it to `Command<ParentMessage>`, which is the parent's message type.
/// This is done by the `MessageWrapper` trait, which removes the work we would
/// have done in the component itself to wrap it to the proper message type for
/// the parent's update method.
pub trait MessageWrapper: Sized {
    type ParentMessage: From<Self>;
}

/// Same as MessageWrapper but for the View messages, which must implement
/// Clone.
pub trait MessageWrapperView: Sized {
    type ParentMessage: From<Self> + Clone;
}

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
    /// Implements the [`MessageWrapper`] trait, to allow children [`State`]
    /// components to be wrapped in the parent's message type.
    /// todo: type defaults are unstable right now...
    type ViewMessage: MessageWrapperView = view::Message;

    /// Messages returned to be executed by commands.
    /// Defaults to the global Application Message.
    /// All of these messages get metaphorically dropped to the top of the stack
    /// (the main application), and piped back down to the screen that
    /// returned the message.
    /// Implements the [`MessageWrapper`] trait, to allow children [`State`]
    /// components to be wrapped in the parent's message type.
    /// todo: type defaults are unstable right now...
    type AppMessage: MessageWrapper = app::Message;

    /// Renders the screen which can produce [`ViewMessage`]s.
    fn view(&self) -> Element<'_, Self::ViewMessage>;

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

    pub fn view(&self) -> Element<'_, <WindowScreen as State>::ViewMessage> {
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
