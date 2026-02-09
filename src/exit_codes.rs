use std::fmt;
use std::process;

// Frozen exit codes per cli_spec.md
pub const SUCCESS: i32 = 0;
pub const USAGE_ERROR: i32 = 1;
pub const INVALID_QUERY: i32 = 2;
pub const INVALID_BUDGET: i32 = 3;
pub const CACHE_MISSING: i32 = 4;
pub const CACHE_INVALID: i32 = 5;
pub const IO_ERROR: i32 = 6;
pub const INTERNAL_ERROR: i32 = 7;

pub struct CliError {
    pub code: i32,
    pub message: String,
}

impl CliError {
    pub fn new(code: i32, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }

    pub fn cache_missing(detail: impl fmt::Display) -> Self {
        Self::new(CACHE_MISSING, format!("Cache does not exist: {detail}"))
    }

    pub fn cache_invalid(detail: impl fmt::Display) -> Self {
        Self::new(CACHE_INVALID, format!("Cache is invalid: {detail}"))
    }

    pub fn io_error(detail: impl fmt::Display) -> Self {
        Self::new(IO_ERROR, format!("I/O error: {detail}"))
    }

    pub fn internal(detail: impl fmt::Display) -> Self {
        Self::new(INTERNAL_ERROR, format!("Internal error: {detail}"))
    }

    pub fn exit(self) -> ! {
        eprintln!("error: {}", self.message);
        process::exit(self.code);
    }
}

impl From<context_core::cache::CacheBuildError> for CliError {
    fn from(err: context_core::cache::CacheBuildError) -> Self {
        use context_core::cache::CacheBuildError;
        match &err {
            CacheBuildError::Io(_) => Self::io_error(&err),
            CacheBuildError::OutputExists(_) => Self::io_error(&err),
            CacheBuildError::Serialization(_) => Self::internal(&err),
            CacheBuildError::FilenameCollision(_) => Self::internal(&err),
            CacheBuildError::DuplicateDocumentId(_) => Self::cache_invalid(&err),
            CacheBuildError::InvalidVersionFormat(_) => Self::internal(&err),
        }
    }
}

impl From<context_core::types::SelectionError> for CliError {
    fn from(err: context_core::types::SelectionError) -> Self {
        use context_core::types::SelectionError;
        match &err {
            SelectionError::InvalidBudget(_) => Self::new(INVALID_BUDGET, err.to_string()),
            SelectionError::CacheError => Self::cache_invalid("cache integrity error"),
        }
    }
}

pub fn from_io_error(err: std::io::Error, cache_path: &std::path::Path) -> CliError {
    if err.kind() == std::io::ErrorKind::NotFound {
        CliError::cache_missing(cache_path.display())
    } else {
        CliError::io_error(&err)
    }
}

pub fn from_manifest_parse(err: serde_json::Error) -> CliError {
    CliError::cache_invalid(format!("invalid manifest: {err}"))
}
