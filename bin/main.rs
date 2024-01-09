use anyhow::Result;
use clap::{ArgAction, Parser, Subcommand};
use dotenv::dotenv;

#[derive(Parser)]
#[clap(name = "Excalibur")]
#[clap(version = env!("CARGO_PKG_VERSION"))]
#[clap(about = "Simulation driven development.", long_about = None)]
#[clap(author)]
/// The Args struct is used to parse and store the command-line arguments passed
/// to the `Excalibur` tool.
struct Args {
    /// Defines the subcommand to execute.
    #[command(subcommand)]
    /// The command field stores the subcommand to be executed.
    command: Option<Commands>,

    #[clap(short, long, global = true, required = false, action = ArgAction::Count, value_parser(
        clap::value_parser!(u8)))]
    /// The verbose field stores the verbosity level of the logging.
    verbose: Option<u8>,

    #[clap(long, global = true)]
    /// The simulation field indicates whether the simulation module should be
    /// ran.
    simulation: bool,

    #[clap(long, global = true)]
    /// The ui field indicates whether the UI module should be ran.
    ui: bool,

    #[clap(long, global = true)]
    /// The analysis field indicates whether the analysis module should ran.
    analysis: bool,

    #[clap(long, global = true)]
    arbiter_core: bool,

    #[clap(long, global = true)]
    /// The dev field indicates whether the application is running in
    /// development mode. The dev flag will show metrics on performance in
    /// the UI that can be helpful for debugging.
    dev: bool,
}

/// Defines available subcommands for the `Arbiter` tool.
#[derive(Subcommand)]
enum Commands {
    /// The `Simulate` subcommand is used to run a simulation.
    /// It takes a configuration file path as an argument.
    /// The default configuration file is "configs/v3/static.toml".
    Simulate {
        #[clap(index = 1, default_value = "configs/v3/static.toml")]
        config_path: String,
    },
    /// The `Analyze` subcommand is used to run an analysis.
    Analyze,
    /// The `Ui` subcommand is used to run the user interface.
    Ui,
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

    match &args.command {
        Some(Commands::Simulate { config_path }) => sim::run(config_path, args.verbose)?,
        Some(Commands::Analyze) => todo!(),
        Some(Commands::Ui) => app::run(args.dev)?,
        None => app::run(args.dev)?,
    }
    Ok(())
}
