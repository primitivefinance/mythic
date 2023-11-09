use std::sync::{mpsc::Receiver, Arc, Mutex};

use arbiter_core::{
    environment::{builder::EnvironmentBuilder, Environment},
    middleware::RevmMiddleware,
};
use ethers::prelude::*;
use iced::{widget::row, Color};
use tracing::info;

use self::footer::Footer;
use super::*;

pub mod components;
mod config_editor;
mod deployer;
pub mod firehose;
mod footer;
mod header;
mod run_sim_button;
mod screen;
mod sdk;
mod sidebar;
mod styles;
mod watcher;

use sdk::production::*;

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
        trace_receiver: Arc<Mutex<Receiver<String>>>,
        footer: Footer,
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

pub struct Flags {
    pub receiver: std::sync::mpsc::Receiver<String>,
}

impl Application for ExampleApp {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = Flags;

    fn new(flags: Flags) -> (Self, Command<Message>) {
        (
            {
                // Load sim environment
                let env = EnvironmentBuilder::new().build();
                let client = RevmMiddleware::new(&env, Some("client")).unwrap();
                let client_cloned = client.clone();
                let trace_receiver = Arc::new(Mutex::new(flags.receiver));
                Self::Running {
                    environment: env,
                    client,
                    screen: Screen::Example(screen::ExampleScreen::new(
                        client_cloned,
                        trace_receiver.clone(),
                    )),
                    production: None,
                    trace_receiver,
                    footer: footer::FooterBuilder::new()
                        .add_crate_info()
                        .add_git_commit()
                        .add_system_info()
                        .build(),
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

                return Command::batch(vec![iced::font::load(
                    iced_aw::graphics::icons::ICON_FONT_BYTES,
                )
                .map(|_| Message::Empty)]);
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
                            screen::Event::SimulationComplete => {
                                // Pass ExampleScreenMessage::SimulationComplete
                                return Command::perform(async { Ok::<(), ()>(()) }, |_| {
                                    Message::ExampleScreen(
                                        screen::ExampleScreenMessage::SimulationComplete,
                                    )
                                });
                            }
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
                                    crate::example::sdk::vault::Vault::deploy::<
                                        deployer::DeployerError,
                                    >(example.client.clone()),
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
        let title = self.title();

        let content: Element<_> = match self {
            ExampleApp::Loading => text("Loading...").into(),
            ExampleApp::Running {
                client,
                screen,
                trace_receiver,
                footer,
                ..
            } => {
                // Base container for the Running state

                // Header with title and restart button
                let header = header::Header::new(title)
                    .view()
                    .map(|_| Message::Debug("header press".into()));

                // Renders the current screen.
                let screen_content = match screen {
                    Screen::Start => {
                        let start_screen =
                            screen::start::StartScreen::new(|| Message::ChangePage(Screen::Home));
                        start_screen.into()
                    }
                    Screen::Home => {
                        // Button to go to the example screen.
                        let example_screen =
                            screen::ExampleScreen::new(client.clone(), trace_receiver.clone());
                        button("Go to Example")
                            .on_press(Message::ChangePage(Screen::Example(example_screen)))
                            .into()
                    }
                    Screen::Example(example) => example.view().map(Message::ExampleScreen),
                };

                let screen_content = container(screen_content)
                    .width(Length::FillPortion(8))
                    .height(Length::Fill)
                    .style(styles::background::WhiteContainer::theme());

                let mut screen_container = row![].height(Length::Fill);
                screen_container = screen_container.push(sidebar::Sidebar::new(footer.clone()));
                screen_container = screen_container.push(screen_content);

                // Combine all elements into a column
                let content = Column::new()
                    .push(header)
                    .push(screen_container)
                    .align_items(alignment::Alignment::Center);

                content.into()
            }
        };

        container(content)
            .center_x()
            .center_y()
            .width(Length::Fill)
            .height(Length::Fill)
            .style(styles::background::BackgroundContainer::theme())
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::custom(iced::theme::Palette {
            background: WHITE,
            text: BLACK,
            primary: BLACK,
            success: Color::from_rgb(0.0, 1.0, 0.0),
            danger: Color::from_rgb(1.0, 0.0, 0.0),
        })
    }

    fn subscription(&self) -> iced::Subscription<Message> {
        if let Self::Running { screen, .. } = self {
            match screen {
                Screen::Example(example) => example.subscription().map(Message::ExampleScreen),
                _ => iced::Subscription::none(),
            }
        } else {
            iced::Subscription::none()
        }
    }
}

pub const WHITE: Color = Color::from_rgb(
    0xfc as f32 / 255.0,
    0xfc as f32 / 255.0,
    0xfc as f32 / 255.0,
);

pub const BLACK: Color = Color::from_rgb(
    0x00 as f32 / 255.0,
    0x00 as f32 / 255.0,
    0x00 as f32 / 255.0,
);

pub const SECONDARY: Color = Color::from_rgb(
    0xf8 as f32 / 255.0,
    0xf9 as f32 / 255.0,
    0xf9 as f32 / 255.0,
);

pub struct ContainerTheme;

impl iced::widget::container::StyleSheet for ContainerTheme {
    type Style = iced::Theme;

    fn appearance(&self, _: &<Self as container::StyleSheet>::Style) -> container::Appearance {
        container::Appearance {
            background: Some(iced::Background::Color(SECONDARY)),
            ..Default::default()
        }
    }
}

impl ContainerTheme {
    pub fn theme() -> iced::theme::Container {
        iced::theme::Container::Custom(Box::from(ContainerTheme))
    }
}
