mod envelope;
mod errors;
mod line;
mod parser;
mod stream;

pub use errors::LogTidyError;

/// Public entry point for the CLI.
/// Takes raw input and returns structured output.
pub fn tidy(input: &str) -> Result<String, LogTidyError> {
    stream::run_with_input(input)
}
