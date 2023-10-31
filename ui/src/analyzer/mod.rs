use arbiter_core::bindings::liquid_exchange;
use ethers::{abi::AbiType, contract::Event};
use std::path::PathBuf;

use super::*;

pub struct AnalyzerApp {
    selected_file: Option<PathBuf>,
    available_files: Vec<PathBuf>,
    selected_events: Vec<liquid_exchange::LiquidExchangeEvents>,
    available_events: Vec<liquid_exchange::LiquidExchangeEvents>,
}

#[derive(Debug, Clone)]
pub struct AnalyzerMessage {}

impl Application for AnalyzerApp {
    type Message = AnalyzerMessage;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<AnalyzerMessage>) {
        (
            AnalyzerApp {
                selected_file: None,
                available_files: vec![],
                selected_events: vec![],
                available_events: vec![],
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Excalibur: Analyzer")
    }

    fn update(&mut self, message: AnalyzerMessage) -> Command<AnalyzerMessage> {
        Command::none()
    }

    fn view(&self) -> Element<AnalyzerMessage> {
        todo!()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}
