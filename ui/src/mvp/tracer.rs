//! Configure how to handle logs produced from the tracing crate.
use std::{
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
    Event, Subscriber,
};
use tracing_subscriber::{prelude::*, registry::LookupSpan, Layer};

// todo: handle generic channel values
// abstraction over our sender and receiver channels that we can pipe trace
// events over.
pub struct Tracer {
    pub sender: Arc<Mutex<Sender<String>>>,
    pub receiver: Arc<Mutex<Receiver<String>>>,
}

impl Tracer {
    pub fn new(sender: Sender<String>, receiver: Receiver<String>) -> Self {
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
        let (sender, receiver) = channel::<String>();
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

/// A layer with a tracer, which has a sender and receiver.
pub struct LayerWithChannel {
    sender: Arc<Mutex<Sender<String>>>,
}

impl LayerWithChannel {
    pub fn new(sender: Arc<Mutex<Sender<String>>>) -> Self {
        Self { sender }
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

impl<S> Layer<S> for LayerWithChannel
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    fn on_event(&self, event: &Event<'_>, _ctx: tracing_subscriber::layer::Context<'_, S>) {
        let mut visitor = TraceVisitor {
            message: String::new(),
        };
        event.record(&mut visitor);
        let _ = self.sender.lock().unwrap().send(visitor.message);
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

        let message = "Hello from Excalibur!";

        // log a message to see if its works
        tracing::info!(message);

        // get the last item from the receiver channel and log it
        let res = tracer.receiver.clone().lock().unwrap().try_recv();
        match res {
            Ok(msg) => {
                tracing::info!("Received message: {:?}", msg);
                assert!(msg.contains(message));
            }
            Err(e) => {
                tracing::error!("Failed to receive message: {:?}", e);
            }
        }
    }
}
