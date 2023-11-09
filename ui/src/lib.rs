#![allow(clippy::all)]
pub mod analyzer;
pub mod example;

use example::Flags;
use iced::{
    alignment, executor,
    widget::{button, container, text, Button, Checkbox, Column, Row, Text},
    Alignment, Application, Command, Element, Length, Settings, Theme,
};
use tracing_subscriber::prelude::*;

pub fn example() -> iced::Result {
    let (sender, receiver) = std::sync::mpsc::channel::<String>();
    let firehose_subscriber = example::firehose::FirehoseSubscriber::new(sender);

    let subscriber = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .finish();

    let _subs = subscriber.with(firehose_subscriber);

    // tracing::subscriber::set_global_default(subs)
    //     .expect("Failed to set global default subscriber in UI");
    example::ExampleApp::run(Settings::with_flags(Flags { receiver }))
}
