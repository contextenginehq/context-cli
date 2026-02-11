use std::io::Write;
use std::path::PathBuf;

use clap::Args;
use serde_json::json;

use context_core::cache::CacheManifest;

use crate::exit_codes::{self, CliError};

#[derive(Args)]
pub struct InspectArgs {
    /// Path to a built cache directory
    #[arg(long)]
    pub cache: PathBuf,
}

pub fn run(args: InspectArgs) -> Result<(), CliError> {
    // Load manifest
    let manifest_path = args.cache.join("manifest.json");
    let manifest_file = std::fs::File::open(&manifest_path)
        .map_err(|e| exit_codes::from_io_error(e, &args.cache))?;
    let manifest: CacheManifest = serde_json::from_reader(manifest_file)
        .map_err(exit_codes::from_manifest_parse)?;

    // Compute total_bytes from document files
    let mut total_bytes: u64 = 0;
    let mut all_files_exist = true;
    for entry in &manifest.documents {
        let doc_path = args.cache.join(&entry.file);
        match std::fs::metadata(&doc_path) {
            Ok(meta) => total_bytes += meta.len(),
            Err(_) => {
                all_files_exist = false;
            }
        }
    }

    let output = json!({
        "cache_version": manifest.cache_version,
        "document_count": manifest.document_count,
        "total_bytes": total_bytes,
        "valid": all_files_exist,
    });

    let stdout = std::io::stdout();
    let mut out = stdout.lock();
    serde_json::to_writer_pretty(&mut out, &output)
        .map_err(|e| CliError::internal(e))?;
    writeln!(out).map_err(|e| CliError::io_error(e))?;

    Ok(())
}
