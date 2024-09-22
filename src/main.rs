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
#[clap(name = "Mythic")]
#[clap(version = env!("CARGO_PKG_VERSION"))]
#[clap(about = "Mythic is a native desktop client for smart contracts.", long_about = None)]
#[clap(author)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,

    #[clap(short, long, global = true, required = false, action = ArgAction::Count, value_parser(
        clap::value_parser!(u8)))]
    verbose: Option<u8>,

    #[clap(long, global = true)]
    ui: bool,

    #[clap(long, global = true)]
    dev: bool,
}

#[derive(Subcommand)]
enum Commands {
    Ui,
}

fn main() -> Result<()> {
    dotenv().ok();

    let args = Args::parse();

    let log_level = match args.verbose.unwrap_or(0) {
        0 => tracing::Level::ERROR,
        1 => tracing::Level::WARN,
        2 => tracing::Level::INFO,
        3 => tracing::Level::DEBUG,
        _ => tracing::Level::TRACE,
    };
    let mut filter = format!("mythic={}", log_level);

    if args.ui {
        filter.push_str(&format!(",ui={}", log_level));
    }

    match &args.command {
        Some(Commands::Ui) => run(args.dev)?,
        None => run(args.dev)?,
    }
    Ok(())
}

pub struct MVP {
    state: State,
    #[allow(dead_code)]
    tracer: tracer::Tracer,
}
enum State {
    App(App),
    Loader(Loader),
}

#[derive(Debug)]
pub enum Message {
    Load(LoaderMessage),
    Update(AppMessage),
    Event(Event),
    Quit,
    ForceQuit,
}

#[derive(Debug, Clone, Copy)]
pub struct Flags {
    pub dev_mode: bool,
}

impl Application for MVP {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = Flags;

    fn new(flags: Flags) -> (MVP, Command<Message>) {
        let tracer = tracer::setup_with_channel();
        if flags.dev_mode {
            std::env::set_var("DEV_MODE", "true");
        }

        let (loader, command) = Loader::new(flags);
        let state = State::Loader(loader);

        (MVP { state, tracer }, command.map(Message::Load))
    }

    fn title(&self) -> String {
        match &self.state {
            State::Loader(_) => String::from("Loading Mythic"),
            State::App(_) => String::from("Mythic"),
        }
    }

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
                    _ => Command::perform(async {}, |()| Message::ForceQuit),
                };

                Command::batch(vec![state_cmd])
            }
            (State::Loader(l), Message::Load(msg)) => match msg {
                loader::LoaderMessage::Ready(Ok((model, client))) => {
                    let (app, command) = App::new(model, client);
                    self.state = State::App(app);

                    command.map(Message::Update)
                }
                loader::LoaderMessage::Ready(Err(error_message)) => {
                    tracing::error!("Failed to load app: {}", error_message);
                    Command::none()
                }
                loader::LoaderMessage::Quit => Command::perform(async {}, |()| Message::ForceQuit),
                _ => l.update(msg).map(Message::Load),
            },
            (State::App(app), Message::Update(msg)) => {
                if let app::AppMessage::QuitReady = msg {
                    return Command::perform(async {}, |()| Message::ForceQuit);
                }
                app.update(msg).map(Message::Update)
            }
            _ => Command::none(),
        }
    }

    fn view(&self) -> Element<Self::Message> {
        match &self.state {
            State::Loader(loader) => loader.view().map(Message::Load),
            State::App(app) => app.view().map(Message::Update),
        }
    }

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

pub fn run(dev_mode: bool) -> iced::Result {
    let mut settings = Settings::with_flags(Flags { dev_mode });
    settings.window.icon = Some(logos::excalibur_logo_2());
    settings.antialiasing = true;
    settings.exit_on_close_request = false;
    settings.id = Some("mythic-app".to_string());
    settings.window.size = (1280, 832);
    MVP::run(settings)
}

pub fn custom_theme() -> iced::theme::Custom {
    iced::theme::Custom::new(Palette {
        background: iced::Color::BLACK,
        primary: MINT_500,
        text: PRIMARY_COLOR,
        success: MINT_500,
        danger: RED_400,
    })
}
