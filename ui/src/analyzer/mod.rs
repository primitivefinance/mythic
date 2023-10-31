use arbiter_core::bindings::liquid_exchange::{self, PriceChangeFilter};

use super::*;

pub struct AnalyzerApp {
    state: AnalyzerState,
    selected_file: Option<usize>,
    available_files: Vec<PathBuf>,
    selected_events: Vec<usize>,
    available_events: Vec<String>,
    file_content: Option<String>,
}

#[derive(Debug, Clone)]
pub enum AnalyzerMessage {
    ChooseFile(usize),
    FileChosen,
    ChooseEvent(usize),
    EventsChosen,
}

pub enum AnalyzerState {
    FileSelection,
    EventSelection,
    DisplayEvents,
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
            AnalyzerMessage::ChooseEvent(selected) => {
                if self.selected_events.contains(&selected) {
                    self.selected_events.retain(|&x| x != selected);
                } else {
                    self.selected_events.push(selected);
                }
            }
            AnalyzerMessage::EventsChosen => {
                self.state = AnalyzerState::FileSelection;
            }
            _ => {}
        }
        Command::none()
    }

    fn view(&self) -> Element<AnalyzerMessage> {
        match self.state {
            AnalyzerState::FileSelection => self.view_file_selection(),
            AnalyzerState::EventSelection => self.view_event_selection(),
            AnalyzerState::DisplayEvents => self.view_file_selection(), // TODO: Do something new later.
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
        // Create a column of checkboxes for event selection
        let mut events_column = Column::new().spacing(10);

        // List of possible event variants
        let possible_events = vec!["PriceChange".to_string(), "Swap".to_string()];

        for (index, event) in possible_events.iter().enumerate() {
            let checkbox = Checkbox::new(event, self.selected_events.contains(&index), move |_| {
                AnalyzerMessage::ChooseEvent(index)
            });
            events_column = events_column.push(checkbox);
        }

        // Embed the column inside the scrollable
        let events_content = Scrollable::new(events_column).width(Length::FillPortion(2));

        // Blurb of text and some other control button (e.g., a button to finalize selection)
        let blurb_content = Column::new()
            .align_items(Alignment::Center)
            .spacing(20)
            .push(Text::new("Choose which events you want to analyze."))
            .push(Button::new("Finalize").on_press(AnalyzerMessage::EventsChosen)) // You might need to add this message variant
            .width(Length::FillPortion(1));

        // Arrange blurb and event list side by side using a Row
        let content = Row::new()
            .padding(20)
            .spacing(20)
            .push(blurb_content)
            .push(events_content);

        // Finalize
        container(content)
            .center_x()
            .center_y()
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

fn liquid_exchange_event_to_string(event: &liquid_exchange::LiquidExchangeEvents) -> &str {
    match event {
        liquid_exchange::LiquidExchangeEvents::PriceChangeFilter(_) => "PriceChangeFilter",
        liquid_exchange::LiquidExchangeEvents::SwapFilter(_) => "SwapFilter",
    }
}
