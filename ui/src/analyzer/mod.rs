use analysis::reader::SimulationData;
use native_dialog::FileDialog;

use super::*;

pub struct AnalyzerApp {
    state: AnalyzerState,
    simulation_data: Option<SimulationData>,
    selected_events: Vec<(String, String)>,
}

#[derive(Debug, Clone)]
pub enum AnalyzerMessage {
    OpenFileExplorerClicked,
    ChooseEvents((String, String)),
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
                    SimulationData::new(path.to_str().unwrap())
                        .map(|data| {
                            self.simulation_data = Some(data);
                        })
                        .unwrap();
                }
            }
            AnalyzerMessage::ChooseEvents(selected) => {
                if self.selected_events.contains(&selected) {
                    self.selected_events.retain(|x| x.clone() != selected);
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
            AnalyzerState::DisplayEvents => self.view_events(),
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
        if let Some(sim_data) = &self.simulation_data {
            // Iterate over contracts
            for contract_name in sim_data.0.keys() {
                let mut contract_column = Column::new()
                    .align_items(Alignment::Start)
                    .spacing(5)
                    .push(Text::new(contract_name).size(20)); // Displaying the contract name

                // Iterate over events inside the current contract
                let events_map = sim_data.0.get(contract_name).unwrap();
                for event_name in events_map.keys() {
                    let checkbox = Checkbox::new(
                        event_name,
                        self.selected_events
                            .contains(&(contract_name.clone(), event_name.clone())),
                        move |_| {
                            AnalyzerMessage::ChooseEvents((
                                contract_name.clone(),
                                event_name.clone(),
                            ))
                        },
                    );
                    contract_column = contract_column.push(checkbox);
                }

                // Add the current contract's column to the events column
                events_column = events_column.push(contract_column);
            }

            events_column = events_column.push(
                Button::new("Analyze Events")
                    .on_press(AnalyzerMessage::EventsChosen)
                    .width(100),
            );
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
        let mut content = Row::new().spacing(20); // Spacing between columns

        for (contract_name, event_name) in &self.selected_events {
            let data = self
                .simulation_data
                .as_ref()
                .unwrap()
                .get_vectorized_events_from_str(contract_name, event_name);

            for (variable_name, values) in data {
                let mut column = Column::new().spacing(5); // Spacing between items in the column

                // Title for the column
                let title = Text::new(variable_name.clone()).size(20);
                column = column.push(title);

                // Calculate the breakpoints for displaying values
                let breakpoint = values.len().min(25);
                let start_of_last = values.len().saturating_sub(25);

                // Add the first 25 values
                for value in values.iter().take(breakpoint) {
                    let text = Text::new(value.to_string()).size(16);
                    column = column.push(text);
                }

                // Add the separator if there are more than 50 values
                if values.len() > 50 {
                    let separator = Text::new("...................").size(16);
                    column = column.push(separator);
                }

                // Add the last 25 values
                for value in values.iter().skip(start_of_last) {
                    let text = Text::new(value.to_string()).size(16);
                    column = column.push(text);
                }

                // Create a Scrollable for the Column
                let column = Scrollable::new(column)
                    .width(Length::FillPortion(1)) // Fill an equal portion of the row
                    .height(Length::Fill); // Fill the vertical space
                content = content.push(column);
            }
        }

        // Finalize
        container(content)
            .center_x()
            .center_y()
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

fn open_file_dialog() -> Option<PathBuf> {
    FileDialog::new()
        .set_location(".")
        .add_filter("JSON Files", &["json"])
        .show_open_single_file()
        .unwrap()
}
