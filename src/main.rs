use ethers::prelude::*;
use iced::{
    alignment,
    event::Event,
    theme::Palette,
    widget::{button, container, text, Column, Row, Text},
    window, Element, Length, Settings, Subscription, Task, Theme,
};

mod app;
mod components;
mod data;
mod loader;
mod middleware;
mod pages;
mod tracer;
mod view;

use std::sync::Arc;

use anyhow::Result;
use app::{App, AppMessage};
use clap::{ArgAction, Parser, Subcommand};
use components::{system::ExcaliburTheme, *};
use data::Model;
use dotenv::dotenv;
use loader::{Loader, LoaderMessage};
use pages::*;
use styles::*;

#[derive(Parser)]
#[clap(name = "Mythic")]
#[clap(version = env!("CARGO_PKG_VERSION"))]
#[clap(about = "Mythic is a native blockchain navigator.", long_about = None)]
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

impl MVP {
    fn new(flags: Flags) -> (MVP, Task<Message>) {
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

    fn update(&mut self, message: Message) -> Task<Message> {
        match (&mut self.state, message) {
            (_, Message::ForceQuit) => window::get_latest().then(|id| window::close(id.unwrap())),
            (_, Message::Quit)
            | (
                _,
                Message::Event(iced::event::Event::Window(iced::window::Event::CloseRequested)),
            ) => {
                let state_cmd = match self.state {
                    State::App(ref mut app) => app.exit().map(Message::Update),
                    _ => Task::perform(async {}, |()| Message::ForceQuit),
                };

                Task::batch(vec![state_cmd])
            }
            (State::Loader(l), Message::Load(msg)) => match msg {
                loader::LoaderMessage::Ready(Ok((model, client))) => {
                    let (app, command) = App::new(model, client);
                    self.state = State::App(app);

                    command.map(Message::Update)
                }
                loader::LoaderMessage::Ready(Err(error_message)) => {
                    tracing::error!("Failed to load app: {}", error_message);
                    Task::none()
                }
                loader::LoaderMessage::Quit => Task::perform(async {}, |()| Message::ForceQuit),
                _ => l.update(msg).map(Message::Load),
            },
            (State::App(app), Message::Update(msg)) => {
                if let app::AppMessage::QuitReady = msg {
                    return Task::perform(async {}, |()| Message::ForceQuit);
                }
                app.update(msg).map(Message::Update)
            }
            _ => Task::none(),
        }
    }

    fn view(&self) -> Element<Message> {
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
            window::close_requests().map(|_| {
                Message::Event(iced::event::Event::Window(
                    iced::window::Event::CloseRequested,
                ))
            }),
        ])
    }

    fn theme(&self) -> Theme {
        ExcaliburTheme::theme()
    }
}

pub fn run(dev_mode: bool) -> iced::Result {
    iced::application(MVP::title, MVP::update, MVP::view)
        .subscription(MVP::subscription)
        .theme(MVP::theme)
        .window(window::Settings {
            size: iced::Size::new(1280.0, 832.0),
            icon: Some(logos::excalibur_logo_2()),
            ..Default::default()
        })
        .exit_on_close_request(false)
        .antialiasing(true)
        .font(include_bytes!("../assets/fonts/DAGGERSQUARE.otf").as_slice())
        .font(iced_aw::BOOTSTRAP_FONT_BYTES)
        .run_with(move || MVP::new(Flags { dev_mode }))
}

pub fn custom_theme() -> iced::theme::Custom {
    iced::theme::Custom::new(
        "main".to_string(),
        Palette {
            background: iced::Color::BLACK,
            primary: MINT_500,
            text: PRIMARY_COLOR,
            success: MINT_500,
            danger: RED_400,
        },
    )
}
