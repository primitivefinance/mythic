use super::*;

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
}

pub struct Terminal {}

impl Terminal {
    pub fn new() -> Self {
        Self {}
    }
}

impl State for Terminal {
    fn view<'a>(&'a self) -> Element<'a, view::Message> {
        Column::new().push(Text::new("Terminal").size(50)).into()
    }

    fn update(&mut self, _message: Message) -> Command<Message> {
        Command::none()
    }
}
