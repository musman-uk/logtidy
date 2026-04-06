mod envelope;
mod errors;
mod line;
mod parser;
mod stream;

pub use errors::LogtidyError;

/// Public entry point for the CLI.
/// Takes raw input and returns structured output.
pub fn tidy(input: &str) -> Result<String, LogtidyError> {
    stream::run_with_input(input)
}
