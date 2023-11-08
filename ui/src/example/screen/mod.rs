//! An example screen that deploys the Counter.sol smart contract.
//! Screens are individual views that can be rendered by the application.
//! The application also handles the execution of the screen's messages and
//! events.

use std::sync::{mpsc::Receiver, Arc, Mutex};

use arbiter_core::middleware::RevmMiddleware;
use iced::{
    widget::{button, column, container, row, text},
    Element, Length,
};
use iced_aw::{graphics::icons::icon_to_char, ICON_FONT};
use simulation::settings::{parameters::Multiple, *};
use tracing::info;

use super::{config_editor, deployer, firehose, run_sim_button, watcher};

mod banner;
pub mod start;

/// Screen for the deploy Counter.sol example.
#[derive(Debug, Clone)]
pub struct ExampleScreen {
    pub client: Arc<RevmMiddleware>,
    pub watcher: watcher::WatcherComponent,
    pub deployer: deployer::DeployerComponent,
    pub config: SimulationConfig<Multiple>,
    pub config_editor: config_editor::ConfigEditor,
    pub firehose: firehose::Firehose,
}

/// Messages for Application -> Screen communication.
/// Handles the messages for the components that are rendered in this screen.
#[derive(Clone, Debug)]
pub enum ExampleScreenMessage {
    Empty,
    FirehoseComponent(firehose::FirehoseMessage),
    WatcherComponent(watcher::AppToWatcherMessage),
    DeployerComponent(deployer::AppToDeployerMessage),
    EditorComponent(config_editor::EditorEvent),
    RunSimulation,
    SimulationComplete,
}

/// Messages for Screen -> Application communication.
#[derive(Clone)]
pub enum Event {
    Deploy,
    Toggle(bool),
    SimulationComplete,
}

/// Implements the following functions
/// - new - creates a new instance of the screen
/// - update - handles messages and returns events, called by the Application.
/// - view - renders the screen, called by the Application.
impl ExampleScreen {
    pub fn new(client: Arc<RevmMiddleware>, receiver: Arc<Mutex<Receiver<String>>>) -> Self {
        // todo: config management feature
        let config = simulation::simulations::import(
            &std::env::current_dir()
                .unwrap()
                .join("simulation")
                .join("src")
                .join("tests")
                .join("configs")
                .join("static.toml")
                .to_str()
                .unwrap(),
        )
        .unwrap();

        info!("Loaded config: {:?}", config);

        let firehose = firehose::Firehose::new(receiver);

        Self {
            client,
            watcher: watcher::WatcherComponent::new(),
            deployer: deployer::DeployerComponent::new(),
            config: config.clone(),
            config_editor: config_editor::ConfigEditor::new(config),
            firehose,
        }
    }

    /// This logic can be a little confusing at first, here's a summary of
    /// what's going on:
    /// - The application renders this screen in `ExampleApp::view()`.
    /// - This screen has a button that triggers a message, i.e.
    ///   `ExampleScreenMessage::Deploy`.
    /// - The `ExampleApp::view()` function captures the triggered message in
    ///   the screen's `view()`.
    /// - Captured messages get passed to `ExampleApp::update()`.
    /// - `ExampleApp::update()` calls `ExampleScreen::update()` with the
    ///   captured message.
    /// - `ExampleScreen::update()` returns an event.
    /// - `ExampleApp` matches the event with a command or message to perform.
    /// - `ExampleApp` executes a `perform` or updates the screen's state.
    ///
    /// This design will probably be updated, but right now we are using it
    /// because only the `Application` can execute commands that are async.
    pub fn update(&mut self, message: ExampleScreenMessage) -> Option<Event> {
        match message {
            ExampleScreenMessage::Empty => None,
            ExampleScreenMessage::WatcherComponent(message) => {
                // Call the update method to get the component's updates.
                let watcher_message = self.watcher.update(message);

                // Handle screen-level logic, and return an App-level message.
                match watcher_message {
                    Some(watcher_message) => {
                        // If the watcher has a message, return it.
                        match watcher_message {
                            watcher::WatcherToAppMessage::Toggle(state) => {
                                Some(Event::Toggle(state))
                            }
                        }
                    }
                    None => None,
                }
            }
            ExampleScreenMessage::DeployerComponent(message) => {
                // Call the update method to get the component's updates.
                let deployer_message = self.deployer.update(message);

                // Handle screen-level logic, and return an App-level message.
                match deployer_message {
                    Some(deployer_message) => match deployer_message {
                        deployer::DeployerToAppMessage::TriggerDeploy => Some(Event::Deploy),
                        deployer::DeployerToAppMessage::Deployed(_result) => None,
                    },
                    None => None,
                }
            }
            ExampleScreenMessage::EditorComponent(message) => {
                let editor_message = self.config_editor.update(message);

                match editor_message {
                    Some(config_editor::EditorToAppMessage::SaveConfig(store)) => {
                        info!("Saving config: {:?}", store);
                        let config = config_editor::Config::from_store(store);
                        self.config = config;
                        info!("Config saved: {:?}", self.config);

                        None
                    }
                    None => None,
                }
            }
            ExampleScreenMessage::FirehoseComponent(message) => {
                self.firehose.update(message);
                None
            }
            ExampleScreenMessage::RunSimulation => {
                let config = self.config.clone();
                run_sim_button::RunSimButton::new(config).run().unwrap();

                Some(Event::SimulationComplete)
            }
            ExampleScreenMessage::SimulationComplete => {
                info!("Simulation complete");
                self.firehose
                    .update(firehose::FirehoseMessage::SimulationComplete);

                None
            }
        }
    }

    pub fn view<'a>(&self) -> Element<'a, ExampleScreenMessage> {
        // Container to fill the entire screen.
        let mut screen_container = column![].width(Length::Fill).height(Length::Fill);

        let mut screen_header = row![]
            .width(Length::Fill)
            .height(Length::Fixed(90.0))
            .padding(16)
            .spacing(16)
            .align_items(iced::alignment::Alignment::Center);

        let screen_title = text("Simulation")
            .size(24)
            .horizontal_alignment(iced::alignment::Horizontal::Left)
            .width(Length::FillPortion(6))
            .height(Length::Fill);

        let run_sim_text = row![
            text(icon_to_char(iced_aw::graphics::icons::Icon::PlayFill).to_string())
                .font(ICON_FONT)
                .horizontal_alignment(iced::alignment::Horizontal::Center)
                .vertical_alignment(iced::alignment::Vertical::Center)
                .height(Length::Fill),
            text("Launch sim")
                .horizontal_alignment(iced::alignment::Horizontal::Center)
                .vertical_alignment(iced::alignment::Vertical::Center)
                .height(Length::Fill)
        ]
        .align_items(iced::alignment::Alignment::Center)
        .spacing(4)
        .height(Length::Fill)
        .padding(8);
        let run_sim = button(run_sim_text).on_press(ExampleScreenMessage::RunSimulation);
        screen_header = screen_header.push(screen_title).push(run_sim);

        // Create a column for the config editor and run sim button
        let config_editor_content = container(
            self.config_editor
                .view()
                .map(|message| ExampleScreenMessage::EditorComponent(message)),
        );

        // Combine the row and column into a single column
        let editor_container = container(config_editor_content).width(Length::FillPortion(2));

        let mut firehose_container = column![].width(Length::FillPortion(2));
        let firehose = self
            .firehose
            .view()
            .map(|message| ExampleScreenMessage::FirehoseComponent(message));
        firehose_container = firehose_container.push(firehose);

        let mut screen_content = row![].width(Length::Fill).height(Length::Fill);
        screen_content = screen_content
            .push(editor_container)
            .push(firehose_container);
        screen_container = screen_container.push(screen_header).push(
            container(screen_content).style(super::styles::background::Layer1Container::theme()),
        );

        container(screen_container).into()
    }

    pub fn subscription(&self) -> iced::Subscription<ExampleScreenMessage> {
        self.firehose.subscription()
    }
}
