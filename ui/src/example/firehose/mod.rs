use std::{
    sync::{mpsc, Arc, Mutex},
    time::Duration,
};

use iced::{
    time,
    widget::{button, column, container, row, scrollable, text},
    Element, Length, Renderer,
};
use tracing::{
    field::{Field, Visit},
    metadata::Level,
    Event, Id, Subscriber,
};
use tracing_subscriber::{layer::Context, Layer};

struct LogVisitor {
    message: String,
}

impl Visit for LogVisitor {
    fn record_debug(&mut self, field: &Field, value: &dyn std::fmt::Debug) {
        if field.name() == "message" {
            self.message = format!("{:?}", value);
        }
    }
}
pub struct FirehoseSubscriber {
    sender: mpsc::Sender<String>,
}

impl FirehoseSubscriber {
    pub fn new(sender: mpsc::Sender<String>) -> Self {
        Self { sender }
    }
}

impl Subscriber for FirehoseSubscriber {
    fn enabled(&self, metadata: &tracing::Metadata<'_>) -> bool {
        metadata.target().starts_with("simulation::") && metadata.level() <= &Level::DEBUG
    }

    fn new_span(&self, _: &tracing::span::Attributes<'_>) -> Id {
        Id::from_u64(0)
    }

    fn record(&self, _: &Id, _: &tracing::span::Record<'_>) {}

    fn record_follows_from(&self, _: &Id, _: &Id) {}

    fn event(&self, event: &Event<'_>) {
        let mut visitor = LogVisitor {
            message: String::new(),
        };
        event.record(&mut visitor);
        let _ = self.sender.send(visitor.message);
    }

    fn enter(&self, _: &Id) {}

    fn exit(&self, _: &Id) {}
}

impl<S> Layer<S> for FirehoseSubscriber
where
    S: Subscriber,
{
    fn on_event(&self, event: &Event<'_>, _ctx: Context<S>) {
        if event.metadata().target().starts_with("simulation::") {
            let mut visitor = LogVisitor {
                message: String::new(),
            };
            event.record(&mut visitor);
            let _ = self.sender.send(visitor.message);
        }
    }
}

// Firehose component
#[derive(Debug, Clone)]
pub struct Firehose {
    receiver: Arc<Mutex<mpsc::Receiver<String>>>,
    logs: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum FirehoseMessage {
    Empty,
    AddLog(String),
    ProcessLogs,
    PurgeLogs,
    SimulationComplete,
}

impl Firehose {
    pub fn new(receiver: Arc<Mutex<mpsc::Receiver<String>>>) -> Self {
        Self {
            receiver,
            logs: Vec::new(),
        }
    }

    pub fn update(&mut self, message: FirehoseMessage) {
        match message {
            FirehoseMessage::Empty => {
                tracing::info!("Empty message received");
            }
            FirehoseMessage::AddLog(log) => self.logs.push(log),
            FirehoseMessage::ProcessLogs => {
                while let Ok(log) = self.receiver.lock().unwrap().try_recv() {
                    self.logs.push(log);
                }
            }
            FirehoseMessage::PurgeLogs => {
                self.logs.clear();
            }
            FirehoseMessage::SimulationComplete => {}
        }
    }

    pub fn view<'a>(&self) -> Element<'a, FirehoseMessage> {
        let firehose_title = text("Firehose")
            .size(24)
            .vertical_alignment(iced::alignment::Vertical::Center)
            .horizontal_alignment(iced::alignment::Horizontal::Center);

        let firehose = self
            .logs
            .iter()
            .rev()
            .fold(column![].spacing(2), |column, log| {
                column.push(style_log_for_firehose(log.clone()))
            });
        let firehose_content = container(scrollable(firehose))
            .style(super::styles::background::Layer2Container::theme())
            .height(Length::Fixed(500.0))
            .width(Length::Fill)
            .padding(16);

        let mut firehose_actions = row![].width(Length::Fill).height(Length::Fill).spacing(4);
        let debug_trace_button = button(text("Send log")).on_press(FirehoseMessage::Empty);
        let process_button = button(text("Process log")).on_press(FirehoseMessage::ProcessLogs);
        let add_log_button = button(text("Add Log directly"))
            .on_press(FirehoseMessage::AddLog("New log".to_string()));
        let purge_log_button = button(text("Purge logs")).on_press(FirehoseMessage::PurgeLogs);

        firehose_actions = firehose_actions
            .push(debug_trace_button)
            .push(process_button)
            .push(add_log_button)
            .push(purge_log_button);

        let mut firehose_header = row![]
            .width(Length::Fill)
            .height(Length::Fixed(45.0))
            .padding(8);

        firehose_header = firehose_header.push(firehose_title);

        let mut content_container = column![].width(Length::Fill);
        content_container = content_container
            .push(firehose_header)
            .push(firehose_content)
            .push(firehose_actions)
            .spacing(16);

        container(content_container).padding(24).into()
    }

    pub fn subscription(&self) -> iced::Subscription<super::example::screen::ExampleScreenMessage> {
        time::every(Duration::from_millis(100)).map(|_| {
            super::example::screen::ExampleScreenMessage::FirehoseComponent(
                FirehoseMessage::ProcessLogs,
            )
        })
    }
}

pub fn style_log_for_firehose<'a>(log: String) -> Element<'a, FirehoseMessage, Renderer> {
    let firehose_element = text(log)
        .style(iced::theme::Text::Color(super::styles::GRAY))
        .size(12)
        .vertical_alignment(iced::alignment::Vertical::Center)
        .horizontal_alignment(iced::alignment::Horizontal::Left);

    container(firehose_element)
        .style(super::styles::background::FirehoseContainer::theme())
        .width(Length::Fill)
        .padding(4)
        .into()
}
