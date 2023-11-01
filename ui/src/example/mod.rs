use std::sync::Arc;

use arbiter_core::{
    environment::{builder::EnvironmentBuilder, Environment},
    middleware::RevmMiddleware,
};

use super::*;

use crate::sdk::production::*;
use ethers::prelude::*;
use tracing::info;

mod components;
mod screen;
mod watcher;

#[allow(clippy::large_enum_variant)]
/// Application state of an example app that runs arbiter's environment in the
/// background.
pub enum ExampleApp {
    Loading,
    Running {
        environment: Environment,
        client: Arc<RevmMiddleware>,
        production: Option<Arc<Production<Ws>>>,
        screen: Screen,
    },
}

/// Screens implement their own `view` and `update` functions and forward
/// messages to this application's `update` function to process them with
/// [`Command`].
#[derive(Clone, Debug)]
pub enum Screen {
    /// The start screen is the first screen of the application.
    Start,
    /// Main screen of the application.
    Home,
    /// An example screen that deploys the Counter.sol smart contract.
    Example(screen::ExampleScreen),
}

#[derive(Debug, Clone)]
#[allow(clippy::large_enum_variant)]
/// Messages that can be sent to the application to process logic.
/// Each screen has a message variant that enables to application to mutate the
/// screen.
pub enum Message {
    SetProduction(Production<Ws>),
    /// Changes the current screen.
    ChangePage(Screen),
    /// Receiving a message from the Example screen.
    ExampleScreen(screen::ExampleScreenMessage),
    /// Error messages to pass up to the main application.
    Error(String),
    /// Debug messages to pass up to the main application.
    Debug(String),
}

impl Application for ExampleApp {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            {
                // Load sim environment
                let env = EnvironmentBuilder::new().build();
                let client = RevmMiddleware::new(&env, Some("client")).unwrap();

                Self::Running {
                    environment: env,
                    client,
                    screen: Screen::Start,
                    production: None,
                }
            },
            Command::perform(Production::new(), |res| match res {
                Ok(production) => Message::SetProduction(production),
                Err(err) => {
                    info!("Error setting production: {}", err);
                    Message::Error(err.to_string())
                }
            }),
        )
    }

    fn title(&self) -> String {
        String::from("Excalibur")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            // Handle debug messages
            Message::Debug(msg) => {
                info!("Debug: {}", msg);
            }
            // Handle error messages
            Message::Error(err) => {
                info!("Error: {}", err);
            }
            // Sets the production state of the application.
            Message::SetProduction(result) => {
                if let Self::Running { production, .. } = self {
                    let entity = Arc::new(result);
                    *production = Some(entity.clone());

                    // Start the sub
                    return Command::perform(watcher::Watcher::new(entity.clone().get()), |_| {
                        Message::Debug("Sub started".to_string())
                    });
                }
            }
            // Mutates this application's `screen` state to the new screen.
            Message::ChangePage(page) => {
                if let Self::Running { screen, .. } = self {
                    *screen = page;
                }
            }
            // Mutates the example screen's state or performs forwarded commands.
            Message::ExampleScreen(message) => {
                if let Self::Running { screen, .. } = self {
                    let Screen::Example(example) = screen else {
                        return Command::none();
                    };

                    if let Some(event) = example.update(message) {
                        match event {
                            screen::Event::Clicked => {
                                return Command::perform(
                                    crate::sdk::vault::Vault::deploy::<screen::ExampleScreenError>(
                                        example.client.clone(),
                                    ),
                                    |res| {
                                        Message::ExampleScreen(
                                            screen::ExampleScreenMessage::DeploySuccess(res),
                                        )
                                    },
                                );
                            }
                            screen::Event::Deployed(res) => match res {
                                Ok(vault) => {
                                    example.state = screen::ExampleScreenState::Deployed(vault);
                                }
                                Err(err) => {
                                    example.state =
                                        screen::ExampleScreenState::DeploymentFailed(err);
                                }
                            },
                        }
                    }
                }
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let content: Element<_> = match self {
            ExampleApp::Loading => text("Loading...").into(),
            ExampleApp::Running { client, screen, .. } => {
                // Base container for the Running state
                let mut content = self::column![];
                // Button for routing back to start screen.
                content = content
                    .push(button("Restart").on_press(Message::ChangePage(Screen::Start)))
                    .spacing(10)
                    .align_items(alignment::Alignment::Center);

                // Renders the current screen.
                match screen {
                    Screen::Start => {
                        let start_screen =
                            screen::start::StartScreen::new(|| Message::ChangePage(Screen::Home));
                        content = content.push(start_screen);
                    }
                    Screen::Home => {
                        // Button to go to the example screen.
                        let example_screen = screen::ExampleScreen::new(client.clone());
                        content =
                            content
                                .push(button("Go to Example").on_press(Message::ChangePage(
                                    Screen::Example(example_screen),
                                )));
                    }
                    Screen::Example(example) => {
                        content = content.push(example.view().map(Message::ExampleScreen));
                    }
                }

                content = content.push(watcher::watcher());

                content.into()
            }
        };

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
