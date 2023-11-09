use iced::{executor, widget::column, window, Application, Command, Element, Settings, Theme};

mod logos;

pub struct MVP;

#[derive(Debug)]
pub enum Message {
    Quit,
}

pub struct Flags;

impl Application for MVP {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = Flags;

    fn new(_flags: Flags) -> (MVP, Command<Message>) {
        (MVP, Command::none())
    }

    fn title(&self) -> String {
        String::from("Excalibur")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Quit => window::close(),
        }
    }

    fn view(&self) -> Element<Message> {
        column![].into()
    }
}

pub fn run() -> iced::Result {
    let mut settings = Settings::with_flags(Flags);
    settings.window.icon = Some(logos::excalibur_logo());
    MVP::run(settings)
}
