#![feature(associated_type_defaults)]
#![feature(inherent_associated_types)]

use ethers::prelude::*;
use iced::{
    alignment, executor,
    theme::Palette,
    widget::{button, container, scrollable, text, Column, Container, Row, Text},
    window, Application, Command, Element, Length, Settings, Subscription, Theme,
};

mod app;
mod components;
mod loader;
mod logging;
mod middleware;
mod screens;
mod user;
mod view;

use std::sync::{Arc, Mutex};

use app::App;
use components::{system::ExcaliburTheme, *};
use loader::Loader;
use logging::tracer;
use screens::*;
use styles::*;
use user::*;

pub struct MVP {
    state: State,
    tracer: tracer::Tracer,
}

enum State {
    App(App),
    Loader(Loader),
}

#[derive(Debug)]
pub enum Message {
    Load(Box<loader::Message>),
    Update(Box<app::Message>),
    Quit,
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

        // Set the dev mode env variables based on the flag
        if flags.dev_mode {
            std::env::set_var("DEV_MODE", "true");
        }

        // 1. Initialize application with the Loader state and Message::Load.
        let (loader, command) = Loader::new(flags);
        let state = State::Loader(loader);

        (
            MVP { state, tracer },
            command.map(|msg| Message::Load(Box::new(msg))),
        )
    }

    fn title(&self) -> String {
        match &self.state {
            State::Loader(_) => String::from("Loading Excalibur"),
            State::App(_) => String::from("Excalibur"),
        }
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match (&mut self.state, message) {
            (_, Message::Quit) => {
                let state_cmd = match self.state {
                    State::App(ref mut app) => app.exit().map(|msg| Message::Update(Box::new(msg))),
                    _ => Command::none(),
                };

                Command::batch(vec![state_cmd, window::close()])
            }
            (State::Loader(l), Message::Load(msg)) => match *msg {
                // 3. Got the message from the loader we are ready to go!
                loader::Message::Ready(Ok((storage, chains, ledger, dev_client))) => {
                    // 4. Create our app and move to the app state.
                    let (app, command) = App::new(storage, chains, ledger, dev_client);
                    self.state = State::App(app);

                    // 5. Get to the next branch.
                    command.map(|msg| Message::Update(Box::new(msg)))
                }
                loader::Message::Ready(Err(error_message)) => {
                    tracing::error!("Failed to load app: {}", error_message);
                    Command::none()
                }
                // 2. Loader emits the Load message, update the loader state.
                _ => l.update(*msg).map(|msg| Message::Load(Box::new(msg))),
            },
            (State::App(app), Message::Update(msg)) => {
                // 6. Arrived at main application loop.
                // note: application loop is by mapping the result of update with Update
                // message.
                app.update(*msg).map(|msg| Message::Update(Box::new(msg)))
            }
            _ => Command::none(),
        }
    }

    // View gets called before `perform` gets called in `new`, by the way.
    fn view(&self) -> Element<Self::Message> {
        match &self.state {
            State::Loader(loader) => loader.view().map(|msg| Message::Load(Box::new(msg))),
            State::App(app) => app.view().map(|msg| Message::Update(Box::new(msg))),
        }
    }

    #[allow(unreachable_patterns)]
    fn subscription(&self) -> Subscription<Message> {
        match &self.state {
            State::Loader(loader) => loader
                .subscription()
                .map(|msg| Message::Load(Box::new(msg))),
            State::App(app) => app.subscription().map(|msg| Message::Update(Box::new(msg))),
            _ => Subscription::none(),
        }
    }

    fn theme(&self) -> Theme {
        ExcaliburTheme::theme()
    }
}

pub fn run(dev_mode: bool) -> iced::Result {
    let mut settings = Settings::with_flags(Flags { dev_mode });
    settings.window.icon = Some(logos::excalibur_logo_2());
    settings.antialiasing = true;
    MVP::run(settings)
}

pub fn custom_theme() -> iced::theme::Custom {
    iced::theme::Custom::new(Palette {
        background: iced::Color::BLACK.into(),
        primary: MINT_500.into(),
        text: PRIMARY_COLOR.into(),
        success: MINT_500.into(),
        danger: RED_400.into(),
    })
}
