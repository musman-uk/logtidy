use thiserror::Error;
use serde_json::Error as SerdeError;

#[derive(Debug, Error)]
pub enum LogTidyError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Failed to serialise JSON output: {0}")]
    JsonOutput(#[from] SerdeError),

    #[error("Internal error: {0}")]
    Internal(String),
}
