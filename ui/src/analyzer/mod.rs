use arbiter_core::bindings::liquid_exchange;
use ethers::{abi::AbiType, contract::Event};
use iced::widget::{checkbox, Radio};
use std::path::PathBuf;

use super::*;

pub struct AnalyzerApp {
    selected_file: Option<usize>,
    available_files: Vec<PathBuf>,
    selected_events: Vec<liquid_exchange::LiquidExchangeEvents>,
    available_events: Vec<liquid_exchange::LiquidExchangeEvents>,
}

#[derive(Debug, Clone)]
pub enum AnalyzerMessage {
    ChooseFile(usize),
    ChooseEvent(liquid_exchange::LiquidExchangeEvents),
}

pub enum AnalyzerScreen {
    Start,
}

impl Application for AnalyzerApp {
    type Message = AnalyzerMessage;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<AnalyzerMessage>) {
        // Read files from a specific directory and populate available_files
        let files = std::fs::read_dir("analysis/static_volatilities/")
            .unwrap()
            .filter_map(|entry| {
                let entry = entry.unwrap();
                if entry.metadata().unwrap().is_file() {
                    Some(entry.path())
                } else {
                    None
                }
            })
            .collect();

        (
            AnalyzerApp {
                selected_file: None,
                available_files: files,
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
        match message {
            AnalyzerMessage::ChooseFile(selected) => {
                self.selected_file = Some(selected);
            }
            _ => {}
        }
        Command::none()
    }

    fn view(&self) -> Element<AnalyzerMessage> {
        // Base container for the Running state
        let mut content = self::column![];

        for (index, file) in self.available_files.iter().enumerate() {
            // Using a radio button for file selection
            let file_string = file.display().to_string();
            let radio = Radio::new(
                &file_string, // use string representation
                index,
                self.selected_file,
                AnalyzerMessage::ChooseFile,
            );

            content = content.push(radio);
        }

        // File selection

        // Finalize
        container(content)
            .center_x()
            .center_y()
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}
