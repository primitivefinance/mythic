#![allow(clippy::all)]

use dotenv::dotenv;
use iced::{Application, Settings};
pub mod counter_component;
pub mod example;
pub mod vault;

#[tokio::main]
pub async fn run() -> iced::Result {
    dotenv().ok();
    example::Example::run(Settings::default())
}
