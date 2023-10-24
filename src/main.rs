use anyhow::Result;
use clap::{builder::TypedValueParser, ArgAction, CommandFactory, Parser, Subcommand};
use simulation::simulations;

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

/// Defines available subcommands for the `Arbiter` tool.
#[derive(Subcommand)]
enum Commands {
    /// Represents the `Bind` subcommand.
    Simulate {
        #[clap(index = 1, default_value = "simulation/configs/test/static.toml")]
        config_path: String,
    },
    Analyze {
        #[clap(index = 1, default_value = "test")]
        type_: String,
    },
}

fn main() -> Result<()> {
    let args = Args::parse();

    let log_level = match args.verbose.unwrap_or(0) {
        0 => tracing::Level::ERROR,
        1 => tracing::Level::WARN,
        2 => tracing::Level::INFO,
        3 => tracing::Level::DEBUG,
        4 | _ => tracing::Level::TRACE,
    };

    tracing_subscriber::fmt().with_max_level(log_level).init();

    match &args.command {
        Some(Commands::Simulate { config_path }) => {
            println!("config path: {}", config_path);
            simulations::batch(config_path)?;
        }
        Some(Commands::Analyze { type_ }) => println!(
            "Exit status: {:?}",
            std::process::Command::new("python")
                .current_dir("analysis")
                .arg("main.py")
                .arg("--type")
                .arg(type_)
                .status()?
        ),
        None => Args::command().print_long_help()?,
    }
    Ok(())
}
