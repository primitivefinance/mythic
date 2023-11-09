use iced::time;

use super::{app::Message, view::terminal_view, *};

use std::{sync::mpsc::Receiver, time::Duration};

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

pub struct Terminal {
    logs: Vec<String>,
    receiver: Arc<Mutex<Receiver<String>>>,
}

impl Terminal {
    pub fn new(receiver: Arc<Mutex<Receiver<String>>>) -> Self {
        Self {
            logs: Vec::new(),
            receiver,
        }
    }
}

impl State for Terminal {
    fn view<'a>(&'a self) -> Element<'a, view::Message> {
        view::app_layout(terminal_view(self.logs.clone())).into()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ProcessTracer => {
                while let Ok(log) = self.receiver.lock().unwrap().try_recv() {
                    self.logs.push(log);
                }
                Command::none()
            }
            Message::View(msg) => match msg {
                view::Message::LogTrace => {
                    tracing::info!("LogTrace message received!");
                    Command::none()
                }
                _ => Command::none(),
            },
            _ => Command::none(),
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        time::every(Duration::from_millis(100)).map(|_| Message::ProcessTracer)
    }
}
