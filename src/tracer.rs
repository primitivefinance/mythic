#![allow(dead_code)]

use std::{
    cell::RefCell,
    collections::HashMap,
    fs::File,
    io,
    path::PathBuf,
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc, Mutex,
    },
};

use tracing::{
    field::{Field, Visit},
    span::{Attributes, Id},
    Event, Subscriber,
};
use tracing_subscriber::{layer::Context, prelude::*, registry::LookupSpan, Layer};

thread_local! {
    static CURRENT_SPAN_FIELDS: RefCell<LayerThreadStorageType> = RefCell::new(HashMap::new());
}

pub struct Tracer {
    pub sender: Arc<Mutex<Sender<AppEventLog>>>,
    pub receiver: Arc<Mutex<Receiver<AppEventLog>>>,
}

impl Tracer {
    pub fn new(sender: Sender<AppEventLog>, receiver: Receiver<AppEventLog>) -> Self {
        let sender = Arc::new(Mutex::new(sender));
        let receiver = Arc::new(Mutex::new(receiver));
        Self { sender, receiver }
    }

    pub fn layer_with_channel(&self) -> LayerWithChannel {
        LayerWithChannel::new(Arc::clone(&self.sender))
    }
}

impl Default for Tracer {
    fn default() -> Self {
        let (sender, receiver) = channel::<AppEventLog>();
        Self::new(sender, receiver)
    }
}

pub enum LogTarget {
    File(PathBuf),
    Stdout,
    Stderr,
}

pub struct TraceConfigBuilder<S: Subscriber + for<'a> LookupSpan<'a>> {
    target: Option<LogTarget>,
    layers: Vec<Box<dyn Layer<S> + Send + Sync + 'static>>,
    _marker: std::marker::PhantomData<S>,
}

impl<S: Subscriber + for<'a> LookupSpan<'a>> Default for TraceConfigBuilder<S> {
    fn default() -> Self {
        Self::new()
    }
}

impl<S: Subscriber + for<'a> LookupSpan<'a>> TraceConfigBuilder<S> {
    pub fn new() -> Self {
        Self {
            target: None,
            layers: Vec::new(),
            _marker: std::marker::PhantomData,
        }
    }

    pub fn with_target(mut self, target: LogTarget) -> Self {
        self.target = Some(target);
        self
    }

    pub fn with_layer(mut self, layer: Box<dyn Layer<S> + Send + Sync + 'static>) -> Self {
        self.layers.push(layer);
        self
    }

    pub fn build(self) -> Box<dyn Layer<S> + Send + Sync + 'static> {
        let fmt_layer: Box<dyn Layer<S> + Send + Sync + 'static> = match self.target {
            Some(LogTarget::File(path)) => {
                let file = File::create(path).expect("failed to create log file");
                Box::new(tracing_subscriber::fmt::layer().with_writer(file))
            }
            Some(LogTarget::Stdout) => {
                Box::new(tracing_subscriber::fmt::layer().with_writer(io::stdout))
            }
            Some(LogTarget::Stderr) => {
                Box::new(tracing_subscriber::fmt::layer().with_writer(io::stderr))
            }
            None => panic!("Log target must be set"),
        };

        let mut layer: Box<dyn Layer<S> + Send + Sync + 'static> = fmt_layer;

        for l in self.layers {
            layer = Box::new(layer.and_then(l));
        }

        layer
            .with_filter(tracing_subscriber::filter::filter_fn(|metadata| {
                !metadata.target().starts_with("gfx_backend_vulkan")
                    && !metadata.target().starts_with("wgpu_hal")
                    && !metadata.target().starts_with("winit")
                    && !metadata.target().starts_with("iced_winit")
                    && !metadata.target().starts_with("wgpu_core")
                    && !metadata.target().starts_with("naga")
                    && !metadata.target().starts_with("iced_wgpu")
                    && !metadata.target().starts_with("cosmic_text")
                    && !metadata.target().starts_with("tokio_tungstenite")
                    && !metadata.target().starts_with("tungstenite")
                    && metadata.level() <= &tracing::Level::TRACE
            }))
            .boxed()
    }
}

pub struct LayerWithLogs {
    logs: Arc<Mutex<Vec<String>>>,
}
impl Default for LayerWithLogs {
    fn default() -> Self {
        Self::new()
    }
}

impl LayerWithLogs {
    pub fn new() -> Self {
        LayerWithLogs {
            logs: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn logs(&self) -> Arc<Mutex<Vec<String>>> {
        Arc::clone(&self.logs)
    }
}

impl<S> Layer<S> for LayerWithLogs
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    fn on_event(&self, event: &Event<'_>, _ctx: tracing_subscriber::layer::Context<'_, S>) {
        let mut logs = self.logs.lock().unwrap();
        let message = format!("{:?}", event);
        logs.push(message);
    }
}

type LayerThreadStorageType = HashMap<AppEventLayer, AppEventMetadata>;

pub struct LayerWithChannel {
    sender: Arc<Mutex<Sender<AppEventLog>>>,
    current_span_fields: std::thread::LocalKey<RefCell<LayerThreadStorageType>>,
}

impl LayerWithChannel {
    pub fn new(sender: Arc<Mutex<Sender<AppEventLog>>>) -> Self {
        Self {
            sender,
            current_span_fields: CURRENT_SPAN_FIELDS,
        }
    }
}

struct TraceVisitor {
    message: String,
}

impl Visit for TraceVisitor {
    fn record_debug(&mut self, field: &Field, value: &dyn std::fmt::Debug) {
        if field.name() == "message" {
            self.message = format!("{:?}", value);
        }
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
pub enum AppEventLayer {
    User,
    System,
    World,
    Agent,
    Default,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum AppEventAction {
    Add,
    Remove,
    Update,
    Custom(String),
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AppEventMetadata {
    pub id: u32,
    pub action: AppEventAction,
    pub data: Vec<String>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AppEventLog {
    pub data: HashMap<AppEventLayer, AppEventMetadata>,
    pub last_message: Option<AppEventMetadata>,
}

impl Default for AppEventLog {
    fn default() -> Self {
        Self::new()
    }
}

impl AppEventLog {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
            last_message: None,
        }
    }

    pub fn with_data(data: HashMap<AppEventLayer, AppEventMetadata>) -> Self {
        Self {
            data,
            last_message: None,
        }
    }
}

struct FieldVisitor {
    fields: HashMap<AppEventLayer, AppEventMetadata>,
    current_layer: Option<AppEventLayer>,
}

impl Visit for FieldVisitor {
    fn record_debug(&mut self, field: &Field, value: &dyn std::fmt::Debug) {
        let field_name = field.name();
        let field_value = format!("{:?}", value);

        if field_name == "layer" {
            self.current_layer = match field_value.as_str() {
                "user" => Some(AppEventLayer::User),
                "system" => Some(AppEventLayer::System),
                "world" => Some(AppEventLayer::World),
                "agent" => Some(AppEventLayer::Agent),
                _ => Some(AppEventLayer::Default),
            };
        }

        if let Some(layer) = &self.current_layer {
            let metadata = self
                .fields
                .entry(layer.clone())
                .or_insert_with(|| AppEventMetadata {
                    action: AppEventAction::Custom(field.name().to_string()),
                    data: Vec::new(),
                    id: 0,
                });

            if field_name.contains("data") {
                metadata.data.push(field_value);
            }
        }

        CURRENT_SPAN_FIELDS.with(|fields| {
            *fields.borrow_mut() = self.fields.clone();
        });
    }
}

impl Visit for AppEventLog {
    fn record_debug(&mut self, field: &Field, value: &dyn std::fmt::Debug) {
        let current_layer: Option<AppEventLayer> = CURRENT_SPAN_FIELDS.with(|fields| {
            let fields = fields.borrow();
            for layer in &[
                AppEventLayer::User,
                AppEventLayer::System,
                AppEventLayer::World,
                AppEventLayer::Agent,
            ] {
                if let Some(layer_fields) = fields.get(layer) {
                    self.data.insert(layer.clone(), layer_fields.clone());
                }
            }

            fields
                .iter()
                .last()
                .map(|(layer, _)| Some(layer.clone()))
                .unwrap_or(None)
        });

        let message = format!("{:?}", value);

        if let Some(layer) = current_layer.clone() {
            if let Some(layer_metadata) = self.data.get_mut(&layer) {
                layer_metadata.data.push(message.clone());
            } else {
                let metadata = AppEventMetadata {
                    action: AppEventAction::Custom(field.name().to_string()),
                    data: vec![message.clone()],
                    id: 0,
                };
                self.data.insert(layer, metadata);
            }
        } else {
            let metadata = AppEventMetadata {
                action: AppEventAction::Custom(field.name().to_string()),
                data: vec![message.clone()],
                id: 0,
            };
            self.data.insert(AppEventLayer::Default, metadata);
        }

        CURRENT_SPAN_FIELDS.with(|fields| {
            let mut fields = fields.borrow_mut();
            if let Some(layer) = current_layer.clone() {
                if let Some(layer_fields) = fields.get_mut(&layer) {
                    layer_fields.data = self.data.get(&layer).unwrap().data.clone();
                }
            }
        });
    }
}

impl<S> Layer<S> for LayerWithChannel
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    fn on_event(&self, event: &Event<'_>, _ctx: tracing_subscriber::layer::Context<'_, S>) {
        let mut visitor = AppEventLog {
            data: HashMap::new(),
            last_message: None,
        };

        event.record(&mut visitor);

        let _ = self.sender.lock().unwrap().send(visitor);
    }

    fn on_new_span(&self, attrs: &Attributes<'_>, _id: &Id, _ctx: Context<'_, S>) {
        let current_fields = CURRENT_SPAN_FIELDS.with(|fields| fields.borrow().clone());

        let mut visitor = FieldVisitor {
            fields: current_fields,
            current_layer: None,
        };

        attrs.record(&mut visitor);

        CURRENT_SPAN_FIELDS.with(|fields| {
            *fields.borrow_mut() = visitor.fields.clone();
        });
    }
}

pub fn setup() {
    let layer = TraceConfigBuilder::new()
        .with_target(LogTarget::Stdout)
        .build();

    tracing_subscriber::registry().with(layer).init();
}

pub fn setup_with_channel() -> Tracer {
    let tracer = Tracer::default();

    let layer = TraceConfigBuilder::new()
        .with_target(LogTarget::Stdout)
        .with_layer(tracer.layer_with_channel().boxed())
        .build();

    tracing_subscriber::registry().with(layer).init();

    tracer
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use tracing::warn;
    use tracing_subscriber::filter;

    use super::*;

    pub fn setup<W: Write>(writer: &mut W) {
        let vec_layer = LayerWithLogs::new();
        let logs = vec_layer.logs();

        let filtered_vec_layer = vec_layer.with_filter(filter::LevelFilter::WARN).boxed();

        let layer = TraceConfigBuilder::new()
            .with_target(LogTarget::Stdout)
            .with_layer(filtered_vec_layer)
            .build();

        tracing_subscriber::registry().with(layer).init();

        warn!("Tracing initialized");

        let logs = logs.lock().unwrap();
        for log in logs.iter() {
            writeln!(writer, "{}", log).unwrap();
        }
    }

    #[ignore]
    #[test]
    fn test_tracer_builder() {
        let mut buffer = Vec::new();
        let output = String::from_utf8(buffer).unwrap();

        assert!(output.contains("Tracing initialized"));
    }

    #[ignore]
    #[test]
    fn test_tracer_with_channel() {
        let tracer = setup_with_channel();

        let message = "1id Hello from Mythic!";

        tracing::info!(message);

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

    #[tracing::instrument(fields(layer = %"user", id = %"1", action = %"click"))]
    fn user_trace() {
        tracing::info!("I'm a user doing stuff!");
        system_trace();
    }

    #[tracing::instrument(fields(layer = %"system", id = %"2", action = %"step"))]
    fn system_trace() {
        tracing::info!("I'm a system doing stuff!");
        world_trace();
    }

    #[tracing::instrument(fields(layer = %"world", id = %"3", action = %"manage"))]
    fn world_trace() {
        tracing::info!("I'm a world doing stuff!");
        agent_trace();
    }

    #[tracing::instrument(fields(layer = %"agent", id = %"4", action = %"act"))]
    fn agent_trace() {
        tracing::info!("I'm an agent doing stuff!");
        default_tracing();
    }

    #[tracing::instrument]
    fn default_tracing() {
        tracing::info!("I'm a default trace!");
    }

    #[ignore]
    #[test]
    fn test_tracer_app_data() {
        let tracer = setup_with_channel();

        user_trace();

        while let Ok(msg) = tracer.receiver.clone().lock().unwrap().try_recv() {
            println!("Received message: {:?}", msg.data);
        }
    }

    #[tracing::instrument(fields(layer = %"user"))]
    fn single_trace() {
        tracing::info!("I'm a single trace!");
    }

    #[ignore]
    #[test]
    fn test_single_layer_field() {
        let tracer = setup_with_channel();

        single_trace();

        while let Ok(msg) = tracer.receiver.clone().lock().unwrap().try_recv() {
            println!("Received message: {:?}", msg.data);
        }
    }
}
