use std::sync::Arc;

use arbiter_core::{
    environment::{builder::EnvironmentBuilder, Environment},
    middleware::RevmMiddleware,
};

use super::*;

mod components;
mod screen;

#[allow(clippy::large_enum_variant)]
/// Application state of an example app that runs arbiter's environment in the
/// background.
pub enum ExampleApp {
    Loading,
    Running {
        environment: Environment,
        client: Arc<RevmMiddleware>,
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
    /// Changes the current screen.
    ChangePage(Screen),
    /// Receiving a message from the Example screen.
    ExampleScreen(screen::ExampleScreenMessage),
}

impl Application for ExampleApp {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            {
                let env = EnvironmentBuilder::new().build();
                let client = RevmMiddleware::new(&env, Some("client")).unwrap();
                Self::Running {
                    environment: env,
                    client,
                    screen: Screen::Start,
                }
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Excalibur")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
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
