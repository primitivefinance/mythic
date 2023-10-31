#![allow(clippy::all)]

pub mod analyzer;
pub mod example;
pub mod sdk;

use iced::{
    alignment, executor,
    widget::{button, column, container, text},
    Application, Command, Element, Length, Settings, Theme,
};

pub fn example() -> iced::Result {
    example::ExampleApp::run(Settings::default())
}

pub fn analyzer() -> iced::Result {
    analyzer::AnalyzerApp::run(Settings::default())
}
