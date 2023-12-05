use std::time::Instant;

use anyhow::Result;
use clap::{ArgAction, CommandFactory, Parser, Subcommand};
use dotenv::dotenv;
use tracing_subscriber::filter::EnvFilter;

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

    #[clap(long, global = true)]
    simulation: bool,

    #[clap(long, global = true)]
    ui: bool,

    #[clap(long, global = true)]
    analysis: bool,

    #[clap(long, global = true)]
    arbiter_core: bool,
}

/// Defines available subcommands for the `Arbiter` tool.
#[derive(Subcommand)]
enum Commands {
    Simulate {
        #[clap(index = 1, default_value = "configs/volatility_targeting/static.toml")]
        config_path: String,
    },
    Analyze,
    Ui {
        #[clap(index = 1, default_value = "")]
        app: String,
    },
    V3,
}

fn main() -> Result<()> {
    dotenv().ok();

    let args = Args::parse();

    let log_level = match args.verbose.unwrap_or(0) {
        0 => tracing::Level::ERROR,
        1 => tracing::Level::WARN,
        2 => tracing::Level::INFO,
        3 => tracing::Level::DEBUG,
        _ => tracing::Level::TRACE,
    };
    let mut filter = format!("excalibur={}", log_level);

    if args.simulation {
        filter.push_str(&format!(",simulation={}", log_level));
    }

    if args.ui {
        filter.push_str(&format!(",ui={}", log_level));
    }

    if args.analysis {
        filter.push_str(&format!(",analysis={}", log_level));
    }
    if args.arbiter_core {
        filter.push_str(&format!(",arbiter_core={}", log_level));
    }

    let env_filter = EnvFilter::new(filter);

    match &args.command {
        Some(Commands::Simulate { config_path }) => sim::run(args.verbose)?,
        Some(Commands::Analyze) => todo!(),
        Some(Commands::Ui { app: _ }) => app::run()?,
        Some(Commands::V3) => sim::run(args.verbose)?,
        None => Args::command().print_long_help()?,
    }
    Ok(())
}
