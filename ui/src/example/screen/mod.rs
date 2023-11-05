//! An example screen that deploys the Counter.sol smart contract.
//! Screens are individual views that can be rendered by the application.
//! The application also handles the execution of the screen's messages and
//! events.

use std::sync::Arc;

use arbiter_core::middleware::RevmMiddleware;
use iced::{
    widget::{column, Component},
    Element,
};

use super::{config_editor, deployer, run_sim_button, watcher};

mod banner;
pub mod start;

/// Screen for the deploy Counter.sol example.
#[derive(Debug, Clone)]
pub struct ExampleScreen {
    pub client: Arc<RevmMiddleware>,
    pub watcher: watcher::WatcherComponent,
    pub deployer: deployer::DeployerComponent,
}

/// Messages for Application -> Screen communication.
/// Handles the messages for the components that are rendered in this screen.
#[derive(Clone, Debug)]
pub enum ExampleScreenMessage {
    WatcherComponent(watcher::AppToWatcherMessage),
    DeployerComponent(deployer::AppToDeployerMessage),
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
    pub fn new(client: Arc<RevmMiddleware>) -> Self {
        Self {
            client,
            watcher: watcher::WatcherComponent::new(),
            deployer: deployer::DeployerComponent::new(),
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
        }
    }

    pub fn view<'a>(&self) -> Element<'a, ExampleScreenMessage> {
        let mut content = column![];

        // Render the Deployer component, wraps the component's messages in a screen
        // message.
        content = content.push(
            self.deployer
                .view()
                .map(|message| ExampleScreenMessage::DeployerComponent(message)),
        );

        // Render the Watcher component, wraps the component's messages in a screen
        // message.
        content = content.push(
            self.watcher
                .view()
                .map(|message| ExampleScreenMessage::WatcherComponent(message)),
        );

        // Render the config editor
        let temp_config = simulation::settings::SimulationConfig::<
            simulation::settings::parameters::Meta,
        >::default();
        tracing::info!("temp_config: {:?}", temp_config);
        content = content.push(config_editor::ConfigEditor::new(temp_config));

        // Render the run sim button
        content = content.push(run_sim_button::RunSimButton::default());

        content.spacing(4).into()
    }
}
