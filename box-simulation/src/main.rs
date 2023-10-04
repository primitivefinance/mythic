use std::time::Instant;

use anyhow::Result;
use tokio::task::JoinHandle;
use tracing::event;
use tracing_subscriber;

mod settings;
mod setup;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "warn");
    }
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();

    let start = Instant::now();

    let config = settings::params::SimulationConfig::new()?;

    println!("Config: {:#?}", config);

    Ok(())
}
