use anyhow::Result;
use clap::{command, CommandFactory, Parser, Subcommand};
use simulation::simulations::{self, dynamic_weights, SimulationType};

/// Represents command-line arguments passed to the `Arbiter` tool.
#[derive(Parser)]
#[command(name = "Portfolio-in-a-Box")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "Simulation driven development.", long_about = None)]
#[command(author)]
struct Args {
    /// Defines the subcommand to execute.
    #[command(subcommand)]
    command: Option<Commands>,
}

/// Defines available subcommands for the `Arbiter` tool.
#[derive(Subcommand)]
enum Commands {
    /// Represents the `Bind` subcommand.
    Simulate {
        #[clap(index = 1, default_value = "simulation/configs/test.toml")]
        config_path: String,
    },
    Analyze {
        #[clap(index = 1, default_value = "test")]
        type_: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "warn");
    }
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let args = Args::parse();

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
