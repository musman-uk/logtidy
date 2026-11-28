use thiserror::Error;
use serde_json::Error as SerdeError;

/// Domain-specific errors returned by the core processing modes (`batch` and `stream`).
/// Kept separate from `anyhow::Error` in `main.rs` so core modules expose typed failures.
#[derive(Debug, Error)]
pub enum LogTidierError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Failed to serialise JSON output: {0}")]
    JsonOutput(#[from] SerdeError),
}
