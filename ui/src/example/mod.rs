use std::sync::Arc;

use arbiter_core::{
    environment::{builder::EnvironmentBuilder, Environment},
    middleware::RevmMiddleware,
};
use ethers::prelude::*;
use tracing::info;

use super::*;
use crate::sdk::production::*;

mod components;
mod deployer;
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
    /// Hacky way to do Command::perform without returning a message.
    Empty,
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
            Message::Empty => {}
            // Handle general debug messages
            Message::Debug(msg) => {
                info!("Debug: {}", msg);
            }
            // Handle general error messages
            Message::Error(err) => {
                info!("Error: {}", err);
            }
            // Sets the production client of the application.
            Message::SetProduction(result) => {
                if let Self::Running { production, .. } = self {
                    let entity = Arc::new(result);
                    *production = Some(entity.clone());
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
                if let Self::Running {
                    screen, production, ..
                } = self
                {
                    let Screen::Example(example) = screen else {
                        return Command::none();
                    };

                    if let Some(event) = example.update(message) {
                        match event {
                            screen::Event::Toggle(state) => {
                                if let Some(production) = production {
                                    match state {
                                        true => {
                                            info!("Starting watcher");
                                            // Turn on the watcher, which returns an instance of
                                            // `Watcher`.
                                            // `Watcher` has a `handle` which is a cancel token to
                                            // cancel the event listener stream.
                                            // This cancel token is cloned and sent to the
                                            // component, so it can cancel the stream when it
                                            // receives
                                            // an abort message from this application.
                                            return Command::perform(
                                                watcher::Watcher::new(production.clone().get()),
                                                |res| {
                                                    match res {
                                                    Ok(watcher) => Message::ExampleScreen(
                                                        screen::ExampleScreenMessage::WatcherComponent(watcher::AppToWatcherMessage::SetWatcher(
                                                            Some(watcher.handle),
                                                        )),
                                                    ),
                                                    Err(err) => {
                                                        info!("Error starting watcher: {}", err);

                                                        // todo: probably handle empty messages in a better way?
                                                        Message::Empty
                                                    }
                                                }
                                                },
                                            );
                                        }
                                        false => {
                                            info!("Stopping watcher");
                                            // Turn off the watcher by sending the abort message to
                                            // the component.
                                            return Command::perform(
                                                async { Ok::<(), ()>(()) },
                                                |_| {
                                                    Message::ExampleScreen(
                                                        screen::ExampleScreenMessage::WatcherComponent(watcher::AppToWatcherMessage::AbortWatcher),
                                                    )
                                                },
                                            );
                                        }
                                    }
                                }
                            }
                            screen::Event::Deploy => {
                                info!("Deploying vault");
                                return Command::perform(
                                    crate::sdk::vault::Vault::deploy::<deployer::DeployerError>(
                                        example.client.clone(),
                                    ),
                                    |res| {
                                        Message::ExampleScreen(
                                            screen::ExampleScreenMessage::DeployerComponent(
                                                deployer::AppToDeployerMessage::DeploySuccess(res),
                                            ),
                                        )
                                    },
                                );
                            }
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
                // note: Each screen is instantiated when its routed to. This has the effect
                // of completely wiping any state that was on the screen before.
                // This could be a benefit, as its a clean way to reset the state of a screen.
                // But there may be scenarios where we want to preserve state of a screen.
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

                // Push text on whether the watcher is ON or OFF
                content = content.push(
                    text(match screen {
                        Screen::Example(example) => match example.watcher.status {
                            true => "Watcher is ON",
                            false => "Watcher is OFF",
                        },
                        _ => "",
                    })
                    .size(30)
                    .width(Length::Fill)
                    .horizontal_alignment(alignment::Horizontal::Center),
                );

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
