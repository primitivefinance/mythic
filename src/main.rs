use std::time::Instant;

use anyhow::Result;
use clap::{ArgAction, CommandFactory, Parser, Subcommand};
use dotenv::dotenv;
use simulation::simulations;
use ui as interface;
#[cfg(feature = "docker")]
pub mod container;

#[cfg(not(feature = "docker"))]
/// Represents command-line arguments passed to the `Arbiter` tool.
#[derive(Parser)]
#[clap(name = "Excalibur")]
#[clap(version = env!("CARGO_PKG_VERSION"))]
#[clap(about = "Simulation driven development.", long_about = None)]
#[clap(author)]
struct Args {
    /// Defines the subcommand to execute.
    #[command(subcommand)]
    command: Option<Commands>,

    #[clap(short, long, global = true, required = false, action = ArgAction::Count, value_parser(
        clap::value_parser!(u8)))]
    verbose: Option<u8>,
}

#[cfg(not(feature = "docker"))]
/// Defines available subcommands for the `Arbiter` tool.
#[derive(Subcommand)]
enum Commands {
    /// Represents the `Bind` subcommand.
    Simulate {
        #[clap(index = 1, default_value = "simulation/src/tests/configs/static.toml")]
        config_path: String,
    },
    Analyze,
    Ui {
        #[clap(index = 1, default_value = "example")]
        app: String,
    },
}

#[cfg(not(feature = "docker"))]
fn main() -> Result<()> {
    dotenv().ok();

    let args = Args::parse();

    let log_level = match args.verbose.unwrap_or(0) {
        0 => {
            println!("Running with tracing::Level::Error");
            tracing::Level::ERROR
        }
        1 => {
            println!("Running with tracing::Level::WARN");
            tracing::Level::WARN
        }
        2 => {
            println!("Running with tracing::Level::INFO");
            tracing::Level::INFO
        }
        3 => {
            println!("Running with tracing::Level::DEBUG");
            tracing::Level::DEBUG
        }
        _ => {
            println!("Running with tracing::Level::TRACE");
            tracing::Level::TRACE
        }
    };

    tracing_subscriber::fmt().with_max_level(log_level).init();

    match &args.command {
        Some(Commands::Simulate { config_path }) => {
            println!("Reading from config path: {}", config_path);
            let start = Instant::now();
            let config = simulations::import(config_path)?;
            simulations::batch(config)?;
            let duration = start.elapsed();
            println!("Total duration of simulations: {:?}", duration);
        }
        Some(Commands::Analyze) => todo!(),
        Some(Commands::Ui { app }) => match app.as_str() {
            "example" => interface::example()?,
            "analyzer" => interface::analyzer()?,
            _ => println!("Unknown app: {}, or no app provided", app),
        },
        None => Args::command().print_long_help()?,
    }
    Ok(())
}

#[cfg(feature = "docker")]
#[tokio::main]
async fn main() -> Result<()> {
    container::run().await;
    Ok(())
}
