use super::*;

use analysis::reader::SimulationData;
use iced::widget::pane_grid::DragEvent;
use native_dialog::FileDialog;
use std::collections::BTreeMap;

pub struct AnalyzerApp {
    state: AnalyzerState,
    simulation_data: Option<SimulationData>,
    selected_events: Vec<(usize, usize)>,
}

#[derive(Debug, Clone)]
pub enum AnalyzerMessage {
    OpenFileExplorerClicked,
    ChooseEvents((usize, usize)),
    EventsChosen,
}

pub enum AnalyzerState {
    Selection,
    DisplayEvents,
}

impl Application for AnalyzerApp {
    type Message = AnalyzerMessage;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<AnalyzerMessage>) {
        (
            AnalyzerApp {
                state: AnalyzerState::Selection,
                simulation_data: None,
                selected_events: Vec::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Excalibur: Analyzer")
    }

    fn update(&mut self, message: AnalyzerMessage) -> Command<AnalyzerMessage> {
        match message {
            AnalyzerMessage::OpenFileExplorerClicked => {
                if let Some(path) = open_file_dialog() {
                    SimulationData::new(path.to_str().unwrap()).map(|data| {
                        self.simulation_data = Some(data);
                    });
                }
            }
            AnalyzerMessage::ChooseEvents(selected) => {
                if self.selected_events.contains(&selected) {
                    self.selected_events.retain(|&x| x != selected);
                } else {
                    self.selected_events.push(selected);
                }
            }
            AnalyzerMessage::EventsChosen => {
                self.state = AnalyzerState::DisplayEvents;
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<AnalyzerMessage> {
        match self.state {
            AnalyzerState::Selection => self.view_selection(),
            AnalyzerState::DisplayEvents => todo!(), //self.view_events(),
        }
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

impl AnalyzerApp {
    fn view_selection(&self) -> Element<AnalyzerMessage> {
        // Handle file selection
        let file_chooser_column = Column::new()
            .align_items(Alignment::Start)
            .spacing(20)
            .push(Text::new("Choose which file you want to analyze."))
            .push(
                Button::new("Open File Explorer")
                    .on_press(AnalyzerMessage::OpenFileExplorerClicked),
            )
            .width(Length::FillPortion(1));

        // Handle which events to analyze
        let mut events_column = Column::new().align_items(Alignment::End).spacing(10);

        // If we have some simulation data
        if let Some(sim_data) = &self.simulation_data {
            // Iterate over contracts
            for (contract_index, contract_name) in sim_data.0.keys().enumerate() {
                let mut contract_column = Column::new()
                    .align_items(Alignment::Start)
                    .spacing(5)
                    .push(Text::new(contract_name).size(20)); // Displaying the contract name

                // Iterate over events inside the current contract
                let events_map = sim_data.0.get(contract_name).unwrap();
                for (event_index, event_name) in events_map.keys().enumerate() {
                    let checkbox = Checkbox::new(
                        event_name,
                        self.selected_events
                            .contains(&(contract_index, event_index)),
                        move |_| AnalyzerMessage::ChooseEvents((contract_index, event_index)),
                    );
                    contract_column = contract_column.push(checkbox);
                }

                // Add the current contract's column to the events column
                events_column = events_column.push(contract_column);
            }
        }

        let content = Row::new()
            .padding(20)
            .spacing(20)
            .push(file_chooser_column)
            .push(events_column);

        // Finalize
        container(content)
            .center_x()
            .center_y()
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn view_events(&self) -> Element<AnalyzerMessage> {
        todo!()
    }
}

fn open_file_dialog() -> Option<PathBuf> {
    FileDialog::new()
        .set_location(".")
        .add_filter("JSON Files", &["json"])
        .show_open_single_file()
        .unwrap()
}
