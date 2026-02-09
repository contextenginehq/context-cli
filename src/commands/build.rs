use std::path::PathBuf;

use clap::Args;
use walkdir::WalkDir;

use context_core::cache::{CacheBuildConfig, CacheBuilder};
use context_core::document::{Document, DocumentId, Metadata};

use crate::exit_codes::CliError;

#[derive(Args)]
pub struct BuildArgs {
    /// Directory containing .md source files
    #[arg(long)]
    pub sources: PathBuf,

    /// Output cache directory
    #[arg(long)]
    pub cache: PathBuf,

    /// Remove existing cache before building
    #[arg(long)]
    pub force: bool,
}

pub fn run(args: BuildArgs) -> Result<(), CliError> {
    // Validate sources directory exists
    if !args.sources.is_dir() {
        return Err(CliError::io_error(format!(
            "sources directory does not exist: {}",
            args.sources.display()
        )));
    }

    // Handle --force
    if args.force && args.cache.exists() {
        std::fs::remove_dir_all(&args.cache).map_err(|e| CliError::io_error(&e))?;
    }

    // Walk sources for .md files
    let mut documents = Vec::new();
    for entry in WalkDir::new(&args.sources)
        .sort_by_file_name()
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let ext = path.extension().and_then(|e| e.to_str());
        if ext != Some("md") {
            continue;
        }

        let id = DocumentId::from_path(&args.sources, path)
            .map_err(|e| CliError::internal(format!("ID error for {}: {e}", path.display())))?;

        let source = path
            .strip_prefix(&args.sources)
            .unwrap_or(path)
            .to_string_lossy()
            .to_string();

        let raw_content = std::fs::read(path).map_err(|e| {
            CliError::io_error(format!("reading {}: {e}", path.display()))
        })?;

        let doc = Document::ingest(id, source, raw_content, Metadata::default()).map_err(
            |e| CliError::internal(format!("ingesting {}: {e}", path.display())),
        )?;

        documents.push(doc);
    }

    let doc_count = documents.len();

    // Build cache
    let builder = CacheBuilder::new(CacheBuildConfig::v0());
    let cache = builder.build(documents, &args.cache)?;

    eprintln!(
        "Built cache: {} documents, version {}",
        doc_count, cache.manifest.cache_version
    );

    Ok(())
}
