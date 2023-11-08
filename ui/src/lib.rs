#![allow(clippy::all)]

pub mod analyzer;
pub mod example;
pub mod sdk;
pub mod styles;

use std::path::PathBuf;

use example::Flags;
use iced::{
    alignment, executor,
    widget::{button, container, text, Button, Checkbox, Column, Row, Scrollable, Text},
    Alignment, Application, Command, Element, Length, Settings, Theme,
};
use tracing_subscriber::prelude::*;

pub fn example() -> iced::Result {
    let (sender, receiver) = std::sync::mpsc::channel::<String>();
    let firehose_subscriber = example::firehose::FirehoseSubscriber::new(sender);

    let subscriber = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .finish();

    let subs = subscriber.with(firehose_subscriber);

    tracing::subscriber::set_global_default(subs)
        .expect("Failed to set global default subscriber in UI");
    example::ExampleApp::run(Settings::with_flags(Flags { receiver }))
}

pub fn analyzer() -> iced::Result {
    analyzer::AnalyzerApp::run(Settings::default())
}
