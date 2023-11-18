use arbiter_core::environment::{builder::EnvironmentBuilder, Environment};
use iced::{
    font,
    widget::{column, container, text},
    Length,
};
use iced_aw::graphics::icons::ICON_FONT_BYTES;
use tracing::{Instrument, Span};

use super::*;

#[derive(Debug)]
pub enum Message {
    View,
    Loaded,
    LoadingFailed,
    Ready(anyhow::Result<(Environment, Local<Ws>)>),
}
pub struct Loader;

pub fn loader_span() -> Span {
    tracing::info_span!("Loader")
}

/// Starts arbiter in the background and connects to a local blockchain.
#[tracing::instrument(level = "info", name = "load_app")]
pub async fn load_app() -> anyhow::Result<(Environment, Local<Ws>), anyhow::Error> {
    let arbiter = EnvironmentBuilder::new().build();
    // todo: get this working without running anvil in background
    let local = Local::default()
        .with_anvil()
        .with_dev_wallet()
        .await
        .with_counter_contract()
        .await;
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
            Command::batch(vec![
                Command::perform(connect_to_server(), |res| {
                    if let Err(e) = res {
                        tracing::error!("Failed to connect to server: {:?}", e);
                        return Message::LoadingFailed;
                    }

                    Message::Loaded
                }),
                font::load(ICON_FONT_BYTES).map(|res| {
                    if let Err(e) = res {
                        tracing::error!("Failed to load icon font: {:?}", e);
                        return Message::LoadingFailed;
                    }

                    Message::Loaded
                }),
            ]),
        )
    }

    fn on_load(&mut self) -> Command<Message> {
        Command::perform(load_app().instrument(loader_span()), Message::Ready)
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
