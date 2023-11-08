//! An example screen that deploys the Counter.sol smart contract.
//! Screens are individual views that can be rendered by the application.
//! The application also handles the execution of the screen's messages and
//! events.

use std::sync::{mpsc::Receiver, Arc, Mutex};

use arbiter_core::middleware::RevmMiddleware;
use iced::{
    widget::{button, column, row, text, Column, Row},
    Element, Length,
};
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
}

/// Messages for Screen -> Application communication.
#[derive(Clone)]
pub enum Event {
    Deploy,
    Toggle(bool),
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

        /*  let (sender, receiver) = std::sync::mpsc::channel();
        let sender = Arc::new(Mutex::new(sender));
        let receiver = Arc::new(Mutex::new(receiver)); */
        let firehose = firehose::Firehose::new(receiver);

        /* let subscriber = firehose::FirehoseSubscriber::new(sender.lock().unwrap().clone());
        tracing::subscriber::with_default(subscriber, || {
            // Now you can use tracing::info! to log messages
            tracing::info!("This is a log message");
        }); */

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
                //let sender_clone = self.sim_log_sender.lock().unwrap().clone();
                let config = self.config.clone();
                run_sim_button::RunSimButton::run_on_thread(config).unwrap();
                /* std::thread::spawn(move || {
                    let subscriber = firehose::FirehoseSubscriber::new(sender_clone);
                    tracing::subscriber::with_default(subscriber, || {
                        // Now you can use tracing::info! to log messages
                        tracing::info!("This is a log message");

                        // todo: fix unwrap on running sim
                        run_sim_button::RunSimButton::run_on_thread(config).unwrap();
                    });
                }); */

                None
            }
        }
    }

    pub fn view<'a>(&self) -> Element<'a, ExampleScreenMessage> {
        // Create a row for the Deployer and Watcher components
        let deployer_watcher_row = Row::new()
            .push(
                self.deployer
                    .view()
                    .map(|message| ExampleScreenMessage::DeployerComponent(message)),
            )
            .push(
                self.watcher
                    .view()
                    .map(|message| ExampleScreenMessage::WatcherComponent(message)),
            )
            .spacing(10);

        // Create a column for the config editor and run sim button
        let editor_run_sim_column = Column::new()
            .push(
                self.config_editor
                    .view("Simulation Config")
                    .map(|message| ExampleScreenMessage::EditorComponent(message)),
            )
            .push(run_sim_button::RunSimButton::new(self.config.clone()))
            .spacing(10);

        // Combine the row and column into a single column
        let content = Column::new()
            .push(deployer_watcher_row)
            .push(editor_run_sim_column)
            .spacing(10)
            .padding(10)
            .width(Length::FillPortion(2));

        let firehose = self
            .firehose
            .view()
            .map(|message| ExampleScreenMessage::FirehoseComponent(message));

        row![
            content,
            column![
                button(text(format!("Run sim"))).on_press(ExampleScreenMessage::RunSimulation),
                firehose,
            ]
            .width(Length::FillPortion(2))
        ]
        .into()
    }

    pub fn subscription(&self) -> iced::Subscription<ExampleScreenMessage> {
        self.firehose.subscription()
    }
}
