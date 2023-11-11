use arbiter_core::environment::{builder::EnvironmentBuilder, Environment};
use iced::{
    alignment,
    widget::{button, column, container, text},
    Length,
};

use super::*;

#[derive(Debug)]
pub enum Message {
    View,
    Loaded,
    Ready(anyhow::Result<(Environment, Local<Ws>)>),
}
pub struct Loader;

/// Starts arbiter in the background and connects to a local blockchain.
pub async fn load_app() -> anyhow::Result<(Environment, Local<Ws>), anyhow::Error> {
    let arbiter = EnvironmentBuilder::new().build();
    // todo: get this working without running anvil in background
    let local = Local::default();
    Ok((arbiter, local))
}

pub async fn connect_to_server() -> anyhow::Result<()> {
    Ok(())
}

impl Loader {
    pub fn new() -> (Self, Command<Message>) {
        // Triggers the next step in the main application loop by emitting the Loaded
        // message.
        (
            Self {},
            Command::perform(connect_to_server(), |_| Message::Loaded),
        )
    }

    fn on_load(&mut self) -> Command<Message> {
        Command::perform(load_app(), Message::Ready)
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::View => Command::none(),
            Message::Loaded => self.on_load(),
            _ => Command::none(),
        }
    }

    pub fn view(&self) -> Element<Message> {
        container(column![text("Running Loader!")])
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
