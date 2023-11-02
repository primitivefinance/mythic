//! An example screen that deploys the Counter.sol smart contract.
//! Screens are individual views that can be rendered by the application.
//! The application also handles the execution of the screen's messages and
//! events.

use std::sync::Arc;

use arbiter_core::middleware::RevmMiddleware;
use iced::{
    alignment::{self},
    widget::{button, column, text},
    Element, Length,
};
use thiserror::Error;

use crate::sdk::vault::*;
use tracing::info;

use tokio_util::sync::CancellationToken;

mod banner;
pub mod start;

/// Screen for the deploy Counter.sol example.
#[derive(Debug, Clone)]
pub struct ExampleScreen {
    pub client: Arc<RevmMiddleware>,
    pub state: ExampleScreenState,
    pub watcher: WatcherManager,
}

#[derive(Clone, Debug)]
pub struct WatcherManager {
    pub state: WatcherState,
    pub handler: Option<CancellationToken>,
}

/// States of the example screen.
/// This example is only concerned with the deployed state of Counter.sol
#[derive(Debug, Clone)]
pub enum ExampleScreenState {
    NotDeployed,
    Deployed(Vault),
    DeploymentFailed(ExampleScreenError),
}

#[derive(Debug, Clone)]
pub enum WatcherState {
    On,
    Off,
}

/// Messages for Application -> Screen communication.
#[derive(Clone, Debug)]
pub enum ExampleScreenMessage {
    Deploy,
    DeploySuccess(Result<Vault, ExampleScreenError>),
    ToggleWatcher(WatcherState),
    SetWatcher(Option<CancellationToken>),
    AbortWatcher,
    Empty,
}

/// Errors that can occur during the deployment of Counter.sol.
#[derive(Debug, Error, Clone)]
pub enum ExampleScreenError {
    #[error("API Error")]
    ProviderError(#[from] &'static ethers::providers::ProviderError),
}

/// Messages for Screen -> Application communication.
#[derive(Clone)]
pub enum Event {
    Clicked,
    Deployed(Result<Vault, ExampleScreenError>),
    Toggle(WatcherState),
}

/// Implements the following functions
/// - new - creates a new instance of the screen
/// - update - handles messages and returns events, called by the Application.
/// - view - renders the screen, called by the Application.
impl ExampleScreen {
    pub fn new(client: Arc<RevmMiddleware>) -> Self {
        Self {
            client,
            state: ExampleScreenState::NotDeployed,
            watcher: WatcherManager {
                state: WatcherState::Off,
                handler: None,
            },
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
            ExampleScreenMessage::ToggleWatcher(state) => {
                info!("Got watcher message: {:?}", state);
                self.watcher.state = state.clone();

                // Tell the application to handle the resulting toggle state.
                Some(Event::Toggle(state.clone()))
            }
            ExampleScreenMessage::SetWatcher(handle) => {
                info!("Got watcher update handle");

                // Set the handler
                self.watcher.handler = handle;

                None
            }
            ExampleScreenMessage::AbortWatcher => {
                info!("Got watcher abort");
                match self.watcher.handler {
                    Some(ref handle) => {
                        info!("Cancelling watcher using token");
                        handle.cancel();
                    }
                    None => {
                        info!("No watcher online to abort.");
                    }
                }

                None
            }
            ExampleScreenMessage::Empty => None,
            ExampleScreenMessage::Deploy => Some(Event::Clicked),
            ExampleScreenMessage::DeploySuccess(res) => Some(Event::Deployed(res)),
        }
    }

    pub fn view<'a>(&self) -> Element<'a, ExampleScreenMessage> {
        let mut content = column![];

        let button = |label, on_press| {
            button(
                text(label)
                    .width(Length::Fill)
                    .height(40)
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .vertical_alignment(alignment::Vertical::Center),
            )
            .on_press(on_press)
        };
        content = content.push(button("Deploy Counter.sol", ExampleScreenMessage::Deploy));

        // Render the watcher buttons
        content = content.push(button(
            "Turn watcher ON",
            ExampleScreenMessage::ToggleWatcher(WatcherState::On),
        ));
        content = content.push(button(
            "Turn watcher OFF",
            ExampleScreenMessage::ToggleWatcher(WatcherState::Off),
        ));

        content = match &self.state {
            ExampleScreenState::NotDeployed => content,
            ExampleScreenState::Deployed(_entity) => content.push(text("Successfully Deployed")),
            ExampleScreenState::DeploymentFailed(error) => {
                content.push(text(format!("Deployment failed: {:?}", error)))
            }
        };

        content.spacing(4).into()
    }
}
