use arbiter_core::bindings::liquid_exchange;
use ethers::{abi::AbiType, contract::Event};
use iced::{
    widget::{checkbox, Button, Column, Radio, Row, Scrollable, Text},
    Alignment,
};
use std::path::PathBuf;

use super::*;

pub struct AnalyzerApp {
    state: AnalyzerState,
    selected_file: Option<usize>,
    available_files: Vec<PathBuf>,
    selected_events: Vec<liquid_exchange::LiquidExchangeEvents>,
    available_events: Vec<liquid_exchange::LiquidExchangeEvents>,
    file_content: Option<String>,
    next_button_state: button::State,
}

#[derive(Debug, Clone)]
pub enum AnalyzerMessage {
    ChooseFile(usize),
    FileChosen,
    ChooseEvent(liquid_exchange::LiquidExchangeEvents),
}

pub enum AnalyzerState {
    FileSelection,
    EventSelection,
}

impl Application for AnalyzerApp {
    type Message = AnalyzerMessage;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<AnalyzerMessage>) {
        // Read files from a specific directory and populate available_files
        let files = std::fs::read_dir("analysis/static_volatilities/gbm_drift=0_vol=1")
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
                state: AnalyzerState::FileSelection,
                selected_file: None,
                available_files: files,
                selected_events: vec![],
                available_events: vec![],
                file_content: None,
                next_button_state: button::State::new(),
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
            AnalyzerMessage::FileChosen => {
                if let Some(index) = self.selected_file {
                    if let Some(path) = self.available_files.get(index) {
                        let content = std::fs::read_to_string(path)
                            .unwrap_or_else(|_| "Failed to read the file.".to_string());
                        self.file_content = Some(content);
                    }
                }
                self.state = AnalyzerState::EventSelection;
            }
            _ => {}
        }
        Command::none()
    }

    fn view(&self) -> Element<AnalyzerMessage> {
        match self.state {
            AnalyzerState::FileSelection => self.view_file_selection(),
            AnalyzerState::EventSelection => self.view_event_selection(),
        }
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

impl AnalyzerApp {
    fn view_file_selection(&self) -> Element<AnalyzerMessage> {
        // Create a column of radio buttons for file selection
        let mut files_column = Column::new().spacing(10);
        for (index, file) in self.available_files.iter().enumerate() {
            let file_string = file.display().to_string();
            let radio = Radio::new(
                &file_string, // use string representation
                index,
                self.selected_file,
                AnalyzerMessage::ChooseFile,
            );
            files_column = files_column.push(radio);
        }

        // Embed the column inside the scrollable
        let files_content = Scrollable::new(files_column).width(Length::FillPortion(2));

        // Blurb of text and Next button
        let blurb_content = Column::new()
            .align_items(Alignment::Center)
            .spacing(20)
            .push(Text::new("Choose which file you want to analyze."))
            .push(Button::new("Next").on_press(AnalyzerMessage::FileChosen))
            .width(Length::FillPortion(1));

        // Arrange blurb and file list side by side using a Row
        let content = Row::new()
            .padding(20)
            .spacing(20)
            .push(blurb_content)
            .push(files_content);

        // Finalize
        container(content)
            .center_x()
            .center_y()
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn view_event_selection(&self) -> Element<AnalyzerMessage> {
        // Base container for the EventSelection state
        let mut content = self::column![];

        // Add a Text element with the message "Choose which events you want."
        content = content.push(Text::new("Choose which events you want."));

        // Here, you could further add elements for event selection. For simplicity, I'm not adding any in this example.

        // Finalize
        container(content)
            .center_x()
            .center_y()
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
