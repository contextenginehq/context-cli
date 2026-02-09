use std::io::Write;
use std::path::PathBuf;

use clap::{Args, ValueEnum};

use context_core::cache::{CacheManifest, ContextCache};
use context_core::selection::ContextSelector;
use context_core::types::Query;

use crate::exit_codes::{self, CliError};

#[derive(ValueEnum, Clone)]
pub enum Format {
    Json,
    Pretty,
}

#[derive(Args)]
pub struct ResolveArgs {
    /// Path to a built cache directory
    #[arg(long)]
    pub cache: PathBuf,

    /// Search query (empty string is allowed)
    #[arg(long)]
    pub query: String,

    /// Maximum token budget (minimum: 0)
    #[arg(long)]
    pub budget: usize,

    /// Output format
    #[arg(long, default_value = "json")]
    pub format: Format,
}

pub fn run(args: ResolveArgs) -> Result<(), CliError> {
    // Load manifest
    let manifest_path = args.cache.join("manifest.json");
    let manifest_file = std::fs::File::open(&manifest_path)
        .map_err(|e| exit_codes::from_io_error(e, &args.cache))?;
    let manifest: CacheManifest = serde_json::from_reader(manifest_file)
        .map_err(exit_codes::from_manifest_parse)?;

    let cache = ContextCache {
        root: args.cache,
        manifest,
    };

    // Run selection
    let selector = ContextSelector::default();
    let query = Query::new(&args.query);
    let result = selector.select(&cache, query, args.budget)?;

    // Serialize to stdout
    let stdout = std::io::stdout();
    let mut out = stdout.lock();
    match args.format {
        Format::Json => serde_json::to_writer(&mut out, &result)
            .map_err(|e| CliError::internal(e))?,
        Format::Pretty => serde_json::to_writer_pretty(&mut out, &result)
            .map_err(|e| CliError::internal(e))?,
    }
    writeln!(out).map_err(|e| CliError::io_error(e))?;

    Ok(())
}
