use ethers::prelude::*;
use iced::{
    alignment, executor,
    widget::{button, column, container, row, scrollable, text, Column, Row, Space, Text},
    window, Application, Command, Element, Length, Settings, Subscription, Theme,
};

mod app;
mod loader;
mod local;
mod logos;
mod state;
mod styles;
mod tracer;
mod view;
mod world;

use std::sync::{Arc, Mutex};

use app::App;
use loader::Loader;
use local::Local;
use styles::*;

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

pub struct Flags;

impl Application for MVP {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = Flags;

    fn new(_flags: Flags) -> (MVP, Command<Message>) {
        let tracer = tracer::setup_with_channel();

        // 1. Initialize application with the Loader state and Message::Load.
        let (loader, command) = Loader::new();
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
            (_, Message::Quit) => window::close(),
            (State::Loader(l), Message::Load(msg)) => match *msg {
                // 3. Got the message from the loader we are ready to go!
                loader::Message::Ready(Ok((arbiter, local))) => {
                    // 4. Create our app and move to the app state.
                    let (app, command) = App::new(arbiter, local, self.tracer.receiver.clone());
                    self.state = State::App(app);

                    // 5. Get to the next branch.
                    command.map(|msg| Message::Update(Box::new(msg)))
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

    fn subscription(&self) -> Subscription<Message> {
        match &self.state {
            State::App(app) => app.subscription().map(|msg| Message::Update(Box::new(msg))),
            _ => Subscription::none(),
        }
    }

    fn theme(&self) -> Theme {
        Theme::default()
    }
}

pub fn run() -> iced::Result {
    let mut settings = Settings::with_flags(Flags);
    settings.window.icon = Some(logos::excalibur_logo_2());
    MVP::run(settings)
}
