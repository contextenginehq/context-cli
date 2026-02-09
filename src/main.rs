mod commands;
mod exit_codes;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "context", version, about = "Context platform CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Build context cache from source documents
    Build(commands::build::BuildArgs),
    /// Resolve context for a query
    Resolve(commands::resolve::ResolveArgs),
    /// Inspect cache state and metadata
    Inspect(commands::inspect::InspectArgs),
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Build(args) => commands::build::run(args),
        Commands::Resolve(args) => commands::resolve::run(args),
        Commands::Inspect(args) => commands::inspect::run(args),
    };

    if let Err(e) = result {
        e.exit();
    }
}
