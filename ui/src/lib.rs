#![allow(clippy::all)]

use dotenv::dotenv;
use iced::{Application, Settings};
pub mod app;
pub mod components;
pub mod screen;
pub mod sdk;

#[tokio::main]
pub async fn run() -> iced::Result {
    dotenv().ok();
    app::example::ExampleApp::run(Settings::default())
}

pub fn example() -> iced::Result {
    app::example::ExampleApp::run(Settings::default())
}
