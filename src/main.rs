//! Building the Excalibur application, laws, rules, and axioms.
//!
//! We can break rules, find loopholes in laws, but cannot avoid axioms.
//!
//!
//! # Axioms
//! - Models handle data. Views handle presentation. Controllers manage user
//!   input, models, and views.
//! - Models and views do not communicate directly.
//! - Data flows in one direction, starting at the model, and ending at the
//!   view.
//!
//! # Rules
//! - Switching to new controllers/screens/anything being rendered should
//!   offload as much logic as possible from `new` to a dedicated `load`.
//! - Add more rules!
use analysis::BatchData;
use ethers::prelude::*;
use iced::{
    alignment,
    event::Event,
    executor,
    theme::Palette,
    widget::{button, container, scrollable, text, Column, Row, Text},
    window, Application, Command, Element, Length, Settings, Subscription, Theme,
};

pub mod app;
mod components;
mod controller;
mod loader;
mod middleware;
mod model;
mod tracer;
mod view;

use std::sync::Arc;

use anyhow::Result;
use app::{App, AppMessage};
use clap::{ArgAction, Parser, Subcommand};
use components::{system::ExcaliburTheme, *};
use controller::*;
use dotenv::dotenv;
use loader::{Loader, LoaderMessage};
use model::Model;
use styles::*;

#[derive(Parser)]
#[clap(name = "Excalibur")]
#[clap(version = env!("CARGO_PKG_VERSION"))]
#[clap(about = "Simulation driven development.", long_about = None)]
#[clap(author)]
/// The Args struct is used to parse and store the command-line arguments passed
/// to the `Excalibur` tool.
struct Args {
    /// Defines the subcommand to execute.
    #[command(subcommand)]
    /// The command field stores the subcommand to be executed.
    command: Option<Commands>,

    #[clap(short, long, global = true, required = false, action = ArgAction::Count, value_parser(
        clap::value_parser!(u8)))]
    /// The verbose field stores the verbosity level of the logging.
    verbose: Option<u8>,

    #[clap(long, global = true)]
    /// The simulation field indicates whether the simulation module should be
    /// ran.
    simulation: bool,

    #[clap(long, global = true)]
    /// The ui field indicates whether the UI module should be ran.
    ui: bool,

    #[clap(long, global = true)]
    /// The analysis field indicates whether the analysis module should ran.
    analysis: bool,

    #[clap(long, global = true)]
    arbiter_core: bool,

    #[clap(long, global = true)]
    /// The dev field indicates whether the application is running in
    /// development mode. The dev flag will show metrics on performance in
    /// the UI that can be helpful for debugging.
    dev: bool,
}

/// Defines available subcommands for the `Arbiter` tool.
#[derive(Subcommand)]
enum Commands {
    /// The `Simulate` subcommand is used to run a simulation.
    /// It takes a configuration file path as an argument.
    /// The default configuration file is "configs/v3/static.toml".
    Simulate {
        #[clap(index = 1, default_value = "configs/v3/static.toml")]
        config_path: String,
    },
    /// The `Analyze` subcommand is used to run an analysis.
    Analyze {
        #[clap(index = 1)]
        data_dir: String,
        #[clap(subcommand)]
        analysis_type: AnalysisType,
    },
    /// The `Ui` subcommand is used to run the user interface.
    Ui,
}

#[derive(Subcommand)]
enum AnalysisType {
    Prices,
    Heatmap,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let args = Args::parse();

    let log_level = match args.verbose.unwrap_or(0) {
        0 => tracing::Level::ERROR,
        1 => tracing::Level::WARN,
        2 => tracing::Level::INFO,
        3 => tracing::Level::DEBUG,
        _ => tracing::Level::TRACE,
    };
    let mut filter = format!("excalibur={}", log_level);

    if args.simulation {
        filter.push_str(&format!(",simulation={}", log_level));
    }

    if args.ui {
        filter.push_str(&format!(",ui={}", log_level));
    }

    if args.analysis {
        filter.push_str(&format!(",analysis={}", log_level));
    }
    if args.arbiter_core {
        filter.push_str(&format!(",arbiter_core={}", log_level));
    }

    match &args.command {
        Some(Commands::Simulate { config_path }) => dfmm::run(config_path, args.verbose)?,
        Some(Commands::Analyze {
            data_dir,
            analysis_type,
        }) => {
            let batch_data = BatchData::new(data_dir).await;
            match analysis_type {
                AnalysisType::Prices => {
                    analysis::commands::plot_prices::plot_prices(batch_data).await
                }
                AnalysisType::Heatmap => {
                    analysis::commands::plot_heatmap::plot_heatmap(batch_data).await
                }
            }
        }
        Some(Commands::Ui) => run(args.dev)?,
        None => run(args.dev)?,
    }
    Ok(())
}

/// The MVP struct represents the Model-View-Presenter pattern used in this
/// application. It contains the state of the application and a tracer for
/// debugging. The state can be either the application itself or a loader.
/// The MVP struct also implements the Application trait from the iced library.
/// This is the outermost layer of the application.
pub struct MVP {
    state: State,
    #[allow(dead_code)]
    tracer: tracer::Tracer,
}
/// The `State` enum represents the current state of the application.
/// It can be either `App` when the application is running or `Loader` when the
/// application is loading. The state should only be in the loader state when
/// the application is starting up in the beginning.
enum State {
    App(App),
    Loader(Loader),
}

/// The `Message` enum represents the different types of messages that can be
/// sent within the application. It can be either `Load` when the application is
/// loading, `Update` when the application is updating, `Event` when an event
/// occurs, `Quit` when the application is quitting, or `ForceQuit` when the
/// application is forced to quit.
#[derive(Debug)]
pub enum Message {
    Load(LoaderMessage),
    Update(AppMessage),
    Event(Event),
    Quit,
    ForceQuit,
}

/// The `Flags` struct represents the flags that can be passed to the
/// application. It contains a single flag `dev_mode` which indicates whether
/// the application is running in development mode.
#[derive(Debug, Clone, Copy)]
pub struct Flags {
    pub dev_mode: bool,
}

/// The `Application` trait implementation for the `MVP` struct.
/// This trait provides the necessary types for the application to run.
/// `Message` is the type of messages that can be sent within the application.
/// `Theme` is the type of theme used in the application.
/// `Executor` is the type of executor used to run the application.
/// `Flags` is the type of flags that can be passed to the application.
impl Application for MVP {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = Flags;

    /// Creates a new instance of the MVP struct.
    ///
    /// This function takes a Flags struct as an argument and returns a tuple
    /// containing an MVP struct and a Command. The Flags struct contains a
    /// single flag `dev_mode` which indicates whether the application is
    /// running in development mode. If the `dev_mode` flag is set, the
    /// "DEV_MODE" environment variable is also set to "true".
    ///
    /// The function initializes the application with the Loader state and a
    /// Load message.
    ///
    /// # Arguments
    ///
    /// * `flags: Flags` - The flags that can be passed to the application.
    ///
    /// # Returns
    ///
    /// * `(MVP, Command<Message>)` - A tuple containing an MVP struct and a
    ///   Command.
    fn new(flags: Flags) -> (MVP, Command<Message>) {
        let tracer = tracer::setup_with_channel();
        if flags.dev_mode {
            std::env::set_var("DEV_MODE", "true");
        }

        let (loader, command) = Loader::new(flags);
        let state = State::Loader(loader);

        (MVP { state, tracer }, command.map(Message::Load))
    }

    /// Returns the title of the application.
    ///
    /// This function takes no arguments and returns a String.
    /// The title of the application depends on the current state of the
    /// application. If the application is in the Loader state, the title is
    /// "Loading Excalibur". If the application is in the App state, the
    /// title is "Excalibur".
    ///
    /// # Returns
    ///
    /// * `String` - The title of the application.
    fn title(&self) -> String {
        match &self.state {
            State::Loader(_) => String::from("Loading Excalibur"),
            State::App(_) => String::from("Excalibur"),
        }
    }

    /// Updates the state of the application based on the incoming message.
    ///
    /// This function takes a mutable reference to the application and a
    /// message. It updates the state of the application based on the
    /// message and returns a command.
    ///
    /// The function handles different types of messages including ForceQuit,
    /// Quit, Load, and Update. It also handles the transition from the
    /// Loader state to the App state.
    ///
    /// # Arguments
    ///
    /// * `&mut self` - A mutable reference to the application.
    /// * `message: Self::Message` - The incoming message.
    ///
    /// # Returns
    ///
    /// * `Command<Self::Message>` - The command to be executed after the state
    ///   update.
    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match (&mut self.state, message) {
            (_, Message::ForceQuit) => window::close(),
            (_, Message::Quit)
            | (
                _,
                Message::Event(iced::event::Event::Window(iced::window::Event::CloseRequested)),
            ) => {
                let state_cmd = match self.state {
                    State::App(ref mut app) => app.exit().map(Message::Update),
                    // todo: handle specific closure logic for the loading screen.
                    _ => Command::perform(async {}, |()| Message::ForceQuit),
                };

                Command::batch(vec![state_cmd])
            }
            (State::Loader(l), Message::Load(msg)) => match msg {
                // 3. Got the message from the loader we are ready to go!
                loader::LoaderMessage::Ready(Ok((model, client))) => {
                    // 4. Create our app and move to the app state.
                    let (app, command) = App::new(model, client);
                    self.state = State::App(app);

                    // 5. Get to the next branch.
                    command.map(Message::Update)
                }
                loader::LoaderMessage::Ready(Err(error_message)) => {
                    tracing::error!("Failed to load app: {}", error_message);
                    Command::none()
                }
                loader::LoaderMessage::Quit => Command::perform(async {}, |()| Message::ForceQuit),
                // 2. Loader emits the Load message, update the loader state.
                _ => l.update(msg).map(Message::Load),
            },
            (State::App(app), Message::Update(msg)) => {
                if let app::AppMessage::QuitReady = msg {
                    return Command::perform(async {}, |()| Message::ForceQuit);
                }
                // 6. Arrived at main application loop.
                // note: application loop is by mapping the result of update with Update
                // message.
                app.update(msg).map(Message::Update)
            }
            _ => Command::none(),
        }
    }

    /// The `view` function is responsible for rendering the current state of
    /// the application. It matches on the current state and calls the
    /// corresponding `view` function.
    ///
    /// If the current state is `Loader`, it calls the `view` function of the
    /// `Loader` struct. The `Loader`'s `view` function returns an `Element`
    /// that is then mapped to a `Load` message.
    ///
    /// If the current state is `App`, it calls the `view` function of the `App`
    /// struct. The `App`'s `view` function returns an `Element` that is
    /// then mapped to an `Update` message.
    ///
    /// # Returns
    ///
    /// * `Element<Self::Message>` - The `Element` to be rendered based on the
    ///   current state.
    fn view(&self) -> Element<Self::Message> {
        match &self.state {
            State::Loader(loader) => loader.view().map(Message::Load),
            State::App(app) => app.view().map(Message::Update),
        }
    }

    /// The `subscription` function is responsible for managing the
    /// subscriptions of the application. It matches on the current state
    /// and calls the corresponding `subscription` function.
    ///
    /// If the current state is `Loader`, it calls the `subscription` function
    /// of the `Loader` struct. The `Loader`'s `subscription` function
    /// returns a `Subscription` that is then mapped to a `Load` message.
    ///
    /// If the current state is `App`, it calls the `subscription` function of
    /// the `App` struct. The `App`'s `subscription` function returns a
    /// `Subscription` that is then mapped to an `Update` message.
    ///
    /// Additionally, it listens for window close events and maps them to
    /// `Event` messages.
    ///
    /// # Returns
    ///
    /// * `Subscription<Self::Message>` - The `Subscription` to be used based on
    ///   the current state.
    fn subscription(&self) -> Subscription<Message> {
        Subscription::batch(vec![
            match &self.state {
                State::Loader(loader) => loader.subscription().map(Message::Load),
                State::App(app) => app.subscription().map(Message::Update),
            },
            iced::subscription::events_with(|event, _status| {
                if matches!(
                    event,
                    iced::event::Event::Window(iced::window::Event::CloseRequested)
                ) {
                    Some(Self::Message::Event(event))
                } else {
                    None
                }
            }),
        ])
    }

    fn theme(&self) -> Theme {
        ExcaliburTheme::theme()
    }
}

/// Runs the Excalibur application
///
/// The function first creates a `Settings` object with the specified `dev_mode`
/// using the `Settings::with_flags` function. It then sets various properties
/// of the `Settings` object:
/// - The window icon is set to the Excalibur logo.
/// - Antialiasing is enabled for smoother graphics.
/// - The application is set to not exit when a close request is received. This
///   allows the application to handle the close request in its own way.
/// - The id of the application is set to "excalibur-app". This is used by the
///   operating system to identify the application.
/// - The window size is set to 1280x832 pixels.
///
/// The function runs the application with the specified settings using the
/// `MVP::run` function.
///
/// # Arguments
///
/// * `dev_mode` - A boolean indicating whether the application should run in
///   development mode.
///
/// # Returns
///
/// * `iced::Result` - The result of running the application. If the application
///   runs successfully, it returns `Ok(())`. If an error occurs, it returns
///   `Err(e)` where `e` is the error.
pub fn run(dev_mode: bool) -> iced::Result {
    let mut settings = Settings::with_flags(Flags { dev_mode });
    settings.window.icon = Some(logos::excalibur_logo_2());
    settings.antialiasing = true;
    settings.exit_on_close_request = false;
    settings.id = Some("excalibur-app".to_string());
    settings.window.size = (1280, 832);
    // im kinda confused about this, what logic actually runs, i can't really follow
    // form this point on
    MVP::run(settings)
}

/// Returns a custom theme for the application.
pub fn custom_theme() -> iced::theme::Custom {
    iced::theme::Custom::new(Palette {
        background: iced::Color::BLACK,
        primary: MINT_500,
        text: PRIMARY_COLOR,
        success: MINT_500,
        danger: RED_400,
    })
}
