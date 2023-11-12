//! Configure how to handle logs produced from the tracing crate.
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

// todo: handle generic channel values
// abstraction over our sender and receiver channels that we can pipe trace
// events over.
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

    /// Returns a layer that can be used with the tracing subscriber.
    /// Use it by calling `tracer.layer_with_channel().boxed()`.
    pub fn layer_with_channel(&self) -> LayerWithChannel {
        LayerWithChannel::new(Arc::clone(&self.sender))
    }
}

/// Creates the channels for us!
impl Default for Tracer {
    fn default() -> Self {
        let (sender, receiver) = channel::<AppEventLog>();
        Self::new(sender, receiver)
    }
}

/// Where to send the logs to.
pub enum LogTarget {
    File(PathBuf),
    Stdout,
    Stderr,
}

/// Builder for the tracing config.
pub struct TraceConfigBuilder<S: Subscriber + for<'a> LookupSpan<'a>> {
    target: Option<LogTarget>,
    layers: Vec<Box<dyn Layer<S> + Send + Sync + 'static>>,
    _marker: std::marker::PhantomData<S>,
}

impl<S: Subscriber + for<'a> LookupSpan<'a>> TraceConfigBuilder<S> {
    pub fn new() -> Self {
        Self {
            target: None,
            layers: Vec::new(),
            _marker: std::marker::PhantomData,
        }
    }

    /// Adds a place to send the logs to.
    pub fn with_target(mut self, target: LogTarget) -> Self {
        self.target = Some(target);
        self
    }

    /// Adds a layer to the tracing config.
    pub fn with_layer(mut self, layer: Box<dyn Layer<S> + Send + Sync + 'static>) -> Self {
        self.layers.push(layer);
        self
    }

    // note: Filters on the layers only get applied to their own layer!
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

/// literally, just a layer that has a vector of logs.
pub struct LayerWithLogs {
    logs: Arc<Mutex<Vec<String>>>,
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

type LayerThreadStorageType = HashMap<AppEventLayer, HashMap<String, String>>;

/// A layer with a tracer, which has a sender and receiver.
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

/// Receives the event from the tracing layer.
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

/// Layers are a hierarchy of events that occur.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
pub enum AppEventLayer {
    User,
    System,
    World,
    Agent,
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
    pub data: Option<String>,
}

/// Structures the data for the application.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AppEventLog {
    pub data: HashMap<AppEventLayer, AppEventMetadata>,
    pub last_message: Option<AppEventMetadata>,
}

/// Stores the current span's fields in a hashmap.
struct FieldVisitor {
    fields: HashMap<AppEventLayer, HashMap<String, String>>,
    current_layer: Option<AppEventLayer>,
}

/// Stores the current span's fields in a hashmap in a thread local.
impl Visit for FieldVisitor {
    fn record_debug(&mut self, field: &Field, value: &dyn std::fmt::Debug) {
        let field_name = field.name();
        let field_value = format!("{:?}", value);

        // Check if the field is the layer
        if field_name == "layer" {
            // Convert the layer string to the corresponding AppEventLayer enum value
            self.current_layer = match field_value.as_str() {
                "user" => Some(AppEventLayer::User),
                "system" => Some(AppEventLayer::System),
                "world" => Some(AppEventLayer::World),
                "agent" => Some(AppEventLayer::Agent),
                _ => None,
            };
        } else {
            // Get the current layer's fields
            if let Some(layer) = &self.current_layer {
                let layer_fields = self
                    .fields
                    .entry(layer.clone())
                    .or_insert_with(HashMap::new);

                // Insert the attribute and its value
                layer_fields.insert(field_name.to_string(), field_value);
            }
        }

        // Store the fields in the CURRENT_SPAN_FIELDS thread-local variable
        CURRENT_SPAN_FIELDS.with(|fields| {
            *fields.borrow_mut() = self.fields.clone();
        });
    }
}

impl Visit for AppEventLog {
    fn record_debug(&mut self, field: &Field, value: &dyn std::fmt::Debug) {
        // Get the current layer from the current span fields
        let current_layer = CURRENT_SPAN_FIELDS.with(|fields| {
            let fields = fields.borrow();
            // it should be the last item in the hashmap
            fields.keys().last().cloned()
        });

        // Determine the next layer based on the current layer
        let next_layer = match current_layer.clone() {
            Some(AppEventLayer::User) => AppEventLayer::System,
            Some(AppEventLayer::System) => AppEventLayer::World,
            Some(AppEventLayer::World) => AppEventLayer::Agent,
            _ => return, // Ignore if there's no next layer
        };

        // Update self.data with the span fields for each layer
        CURRENT_SPAN_FIELDS.with(|fields| {
            let fields = fields.borrow();
            for layer in &[
                AppEventLayer::User,
                AppEventLayer::System,
                AppEventLayer::World,
                AppEventLayer::Agent,
            ] {
                if let Some(layer_fields) = fields.get(layer) {
                    if let Some(id) = layer_fields.get("id") {
                        if let Some(action) = layer_fields.get("action") {
                            let data = layer_fields.get("data").cloned();
                            let metadata = AppEventMetadata {
                                id: id.parse().unwrap_or(0),
                                action: AppEventAction::Custom(action.clone()),
                                data,
                            };

                            self.data.insert(layer.clone(), metadata);
                        }
                    }
                }
            }
        });

        // Set the message as the data for the next layer
        let metadata = AppEventMetadata {
            id: 0, // Set appropriate id
            action: AppEventAction::Custom(field.name().to_string()),
            data: Some(format!("{:?}", value)),
        };

        // Insert the new layer and metadata
        self.data.insert(next_layer, metadata);
    }
}

impl<S> Layer<S> for LayerWithChannel
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    fn on_event(&self, event: &Event<'_>, ctx: tracing_subscriber::layer::Context<'_, S>) {
        let mut visitor = AppEventLog {
            data: HashMap::new(),
            last_message: None,
        };

        event.record(&mut visitor);

        // Walk up the span hierarchy and collect the IDs
        let mut current_span = ctx.lookup_current();
        while let Some(span) = current_span {
            if let Some(fields) = span.extensions().get::<tracing::field::ValueSet>() {
                fields.record(&mut visitor);
            }
            current_span = span.parent();
        }

        let _ = self.sender.lock().unwrap().send(visitor);
    }

    /// Records current span fields to the span field storage in this thread.
    fn on_new_span(&self, attrs: &Attributes<'_>, _id: &Id, _ctx: Context<'_, S>) {
        // Clone the current fields from the thread-local storage
        let current_fields = CURRENT_SPAN_FIELDS.with(|fields| fields.borrow().clone());

        // Create the FieldVisitor with the current fields
        let mut visitor = FieldVisitor {
            fields: current_fields,
            current_layer: None,
        };

        // Record the new span into the visitor
        attrs.record(&mut visitor);

        // Store the new span fields
        CURRENT_SPAN_FIELDS.with(|fields| {
            *fields.borrow_mut() = visitor.fields.clone();
            println!("CURRENT_SPAN_FIELDS: {:?}", fields.borrow());
        });
    }
}

/// Very basic tracing setup.
pub fn setup() {
    // for some reason, if you don't add the filter_layer here it gets some type
    // errors, doesn't like generic. We build our "onion" layer with a target
    // here.
    let layer = TraceConfigBuilder::new()
        .with_target(LogTarget::Stdout)
        .build();

    // Actually initialize the layer to be used by the subscriber.
    tracing_subscriber::registry().with(layer).init();
}

/// Creates a tracer with a channel that can be used to send and receive traces.
pub fn setup_with_channel() -> Tracer {
    let tracer = Tracer::default();

    // for some reason, if you don't add the filter_layer here it gets some type
    // errors, doesn't like generic. We build our "onion" layer with a target
    // here.
    let layer = TraceConfigBuilder::new()
        .with_target(LogTarget::Stdout)
        .with_layer(tracer.layer_with_channel().boxed())
        .build();

    // Actually initialize the layer to be used by the subscriber.
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
        // This layer is very basic, it just pretty prints the event.
        let vec_layer = LayerWithLogs::new();
        let logs = vec_layer.logs();

        // We can add a filter to the layer so it will only capture events that are WARN
        // or higher. note: filter in build() is also applied.
        let filtered_vec_layer = vec_layer.with_filter(filter::LevelFilter::WARN).boxed();

        // for some reason, if you don't add the filter_layer here it gets some type
        // errors, doesn't like generic. We build our "onion" layer with a
        // target here.
        let layer = TraceConfigBuilder::new()
            .with_target(LogTarget::Stdout)
            .with_layer(filtered_vec_layer)
            .build();

        // Actually initialize the layer to be used by the subscriber.
        tracing_subscriber::registry().with(layer).init();

        // We trigger a warn trace so that the filtered vec layer can pick it up.
        warn!("Tracing initialized");

        // After initializing the subscriber, you can print the logs to see the warn
        // trace.
        let logs = logs.lock().unwrap();
        for log in logs.iter() {
            writeln!(writer, "{}", log).unwrap();
        }
    }

    #[test]
    fn test_tracer_builder() {
        let mut buffer = Vec::new();
        setup(&mut buffer); // triggers a warn tracer with a special layer with a warn depth filter.
        let output = String::from_utf8(buffer).unwrap();

        // assert the output contains the warn trace we triggered.
        assert!(output.contains("Tracing initialized"));
    }

    // todo: fix this, it breaks when all tests run because global dispatch is set
    // in the other one...
    #[test]
    fn test_tracer_with_channel() {
        let tracer = setup_with_channel();

        let message = "1id Hello from Excalibur!";

        // log a message to see if its works
        tracing::info!(message);

        // get the last item from the receiver channel and log it
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

    #[tracing::instrument(fields(id = %"1"))]
    fn identified_trace() {
        tracing::info!("Hello, Excalibur!");
        entity_trace();
    }

    #[tracing::instrument(fields(id = %"4", entity = %"agent", action = %"add", data = %"agent_data"))]
    fn entity_trace() {
        tracing::info!("I'm an agent doing stuff!");
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

    #[tracing::instrument(fields(layer = %"agent", id = %"4", action = %"act", data = %"agent_data"))]
    fn agent_trace() {
        tracing::info!("I'm an agent doing stuff!");
    }

    #[test]
    fn test_tracer_app_data() {
        let tracer = setup_with_channel();

        // log a message with a span that sets an id field
        user_trace();

        // get the last item from the receiver channel and log it
        while let Some(msg) = tracer.receiver.clone().lock().unwrap().try_recv().ok() {
            // println the app event log if the data's hashmap has more than 2 items
            if msg.data.len() > 3 {
                println!("Received message: {:?}", msg.data);
            }
        }
    }
}
