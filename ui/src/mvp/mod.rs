use iced::{
    executor, widget::column, window, Application, Command, Element, Settings, Subscription, Theme,
};

mod logos;
mod styles;
mod tracer;

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
        let trace = tracer::setup_with_channel();

        // log a message to see if its works
        tracing::info!("Hello from Excalibur!");

        // get the last item from the receiver channel and log it
        match trace {
            Ok(tracer) => {
                let res = tracer.receiver.clone().lock().unwrap().try_recv();
                match res {
                    Ok(msg) => {
                        tracing::info!("Received message: {:?}", msg);
                    }
                    Err(e) => {
                        tracing::error!("Failed to receive message: {:?}", e);
                    }
                }
            }
            Err(e) => {
                tracing::error!("Failed to setup tracer: {:?}", e);
            }
        }

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

    fn subscription(&self) -> Subscription<Message> {
        Subscription::none()
    }

    fn theme(&self) -> Theme {
        Theme::default()
    }
}

pub fn run() -> iced::Result {
    let mut settings = Settings::with_flags(Flags);
    settings.window.icon = Some(logos::excalibur_logo());
    MVP::run(settings)
}
