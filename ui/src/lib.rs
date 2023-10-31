#![allow(clippy::all)]

use dotenv::dotenv;

pub mod sdk;

pub mod analyzer;
pub mod example;

use iced::{
    alignment, executor,
    widget::{button, column, container, text},
    Application, Command, Element, Length, Settings, Theme,
};

#[tokio::main]
pub async fn run() -> iced::Result {
    dotenv().ok();
    example::ExampleApp::run(Settings::default())
}

pub fn example() -> iced::Result {
    example::ExampleApp::run(Settings::default())
}
