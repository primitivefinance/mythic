use analysis::{
    reader::SimulationData,
    visualize::{plots::line::LinePlot, Figure},
    wad_to_float,
};
use ethers::types::U256;
use iced::widget::Image;
use image;
use native_dialog::FileDialog;

use super::*;

pub struct AnalyzerApp {
    state: AnalyzerState,
    simulation_data: Option<SimulationData>,
    selected_variables: Vec<(String, String, String)>,
}

#[derive(Debug, Clone)]
pub enum AnalyzerMessage {
    OpenFileExplorerClicked,
    ChooseVariables((String, String, String)),
    VariablesChosen,
}

pub enum AnalyzerState {
    Selection,
    DisplayFigure,
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
                selected_variables: Vec::new(),
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
            AnalyzerMessage::ChooseVariables(selected) => {
                if self.selected_variables.contains(&selected) {
                    self.selected_variables.retain(|x| x.clone() != selected);
                } else {
                    self.selected_variables.push(selected);
                }
            }
            AnalyzerMessage::VariablesChosen => {
                self.state = AnalyzerState::DisplayFigure;
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<AnalyzerMessage> {
        match self.state {
            AnalyzerState::Selection => self.view_selection(),
            AnalyzerState::DisplayFigure => self.view_events(),
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
            for contract_name in sim_data.contract_events.keys() {
                let mut contract_column = Column::new()
                    .align_items(Alignment::Start)
                    .spacing(5)
                    .push(Text::new(contract_name).size(20)); // Displaying the contract name

                // Iterate over events inside the current contract
                let events_map = sim_data.contract_events.get(contract_name).unwrap();
                for event_name in events_map.keys() {
                    let data = self
                        .simulation_data
                        .as_ref()
                        .unwrap()
                        .get_vectorized_events_from_str(contract_name, event_name);

                    for (variable_name, _values) in data {
                        let checkbox = Checkbox::new(
                            variable_name.clone(),
                            self.selected_variables.contains(&(
                                contract_name.clone(),
                                event_name.clone(),
                                variable_name.clone(),
                            )),
                            move |_| {
                                AnalyzerMessage::ChooseVariables((
                                    contract_name.clone(),
                                    event_name.clone(),
                                    variable_name.clone(),
                                ))
                            },
                        );
                        contract_column = contract_column.push(checkbox);
                    }
                }

                // Add the current contract's column to the events column
                events_column = events_column.push(contract_column);
            }

            events_column = events_column.push(
                Button::new("Analyze Events")
                    .on_press(AnalyzerMessage::VariablesChosen)
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
        let mut figure = Figure::new("data", None);
        for (contract_name, event_name, variable_name) in &self.selected_variables {
            let data = self
                .simulation_data
                .as_ref()
                .unwrap()
                .get_vectorized_events_from_str(contract_name, event_name);

            let variable = data.get(variable_name).unwrap();
            let (indices, values): (Vec<_>, Vec<_>) = variable
                .into_iter()
                .cloned()
                .enumerate()
                .map(|(idx, val)| (idx as f64, val.clone()))
                .unzip();
            println!("values as value: {:?}", values);
            let values = values
                .into_iter()
                .map(|x| x.as_str().unwrap().to_owned())
                .collect::<Vec<String>>();
            println!("values as string: {:?}", values);
            let values = values
                .into_iter()
                .map(|x| U256::from_str_radix(&x, 16).unwrap())
                .collect::<Vec<U256>>();
            println!("values as u256: {:?}", values);
            let values = values
                .into_iter()
                .map(|x| wad_to_float(x))
                .collect::<Vec<f64>>();

            let lineplot = LinePlot {
                x_data: indices,
                y_data: values,
            };
            figure.add_plot(lineplot);
        }

        figure.create();

        // Load the image data
        let img = image::open("Data.png").unwrap();
        let img = img.to_rgba8();
        let (width, height) = img.dimensions();
        let img_data = img.into_raw();

        // Create an iced::Image
        let handle = iced::widget::image::Handle::from_pixels(width, height, img_data);
        let img_widget = Image::new(handle);

        // Add the image to the UI
        content = content.push(img_widget);

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
