use iced::widget::{button, column, scrollable, text, Scrollable, Text};
use iced::{time, Element};
use std::sync::Mutex;
use std::sync::{mpsc, Arc};
use std::time::Duration;
use tracing::field::{Field, Visit};
use tracing::metadata::Level;
use tracing::Id;
use tracing::{Event, Subscriber};
use tracing_subscriber::layer::Context;
use tracing_subscriber::Layer;

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
        metadata.level() == &Level::INFO
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
        let mut visitor = LogVisitor {
            message: String::new(),
        };
        event.record(&mut visitor);
        let _ = self.sender.send(visitor.message);
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
            _ => {}
        }
    }

    pub fn view<'a>(&self) -> Element<'a, FirehoseMessage> {
        let firehose = self
            .logs
            .iter()
            .fold(column![], |column, log| column.push(Text::new(log.clone())));

        let debug_trace_button = button(text("Send log")).on_press(FirehoseMessage::Empty);
        let process_button = button(text("Process log")).on_press(FirehoseMessage::ProcessLogs);
        let add_log_button = button(text("Add Log directly"))
            .on_press(FirehoseMessage::AddLog("New log".to_string()));

        let mut content = column![];
        content = content
            .push(debug_trace_button)
            .push(process_button)
            .push(add_log_button)
            .push(firehose)
            .spacing(4)
            .padding(8);

        scrollable(content).into()
    }

    pub fn subscription(&self) -> iced::Subscription<super::example::screen::ExampleScreenMessage> {
        time::every(Duration::from_millis(1000)).map(|_| {
            super::example::screen::ExampleScreenMessage::FirehoseComponent(
                FirehoseMessage::ProcessLogs,
            )
        })
    }
}