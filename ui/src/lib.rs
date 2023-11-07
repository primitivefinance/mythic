#![allow(clippy::all)]

pub mod analyzer;
pub mod example;
pub mod sdk;

use std::path::PathBuf;

use iced::{
    alignment, executor,
    widget::{button, container, text, Button, Checkbox, Column, Row, Text},
    Alignment, Application, Command, Element, Length, Settings, Theme,
};

pub fn example() -> iced::Result {
    example::ExampleApp::run(Settings::default())
}

pub fn analyzer() -> iced::Result {
    analyzer::AnalyzerApp::run(Settings::default())
}
