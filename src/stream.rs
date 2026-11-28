use crate::errors::LogTidierError;
use crate::line::Line;
use crate::parser::Parser;
use std::io::{self, BufRead};

/// Streams stdin line-by-line so unbounded inputs (such as `tail -f`) run in constant memory
/// and emit records immediately without waiting for EOF.
pub fn stream() -> Result<(), LogTidierError> {
    let stdin = io::stdin();
    let parser = Parser::new();

    // Enumerate before skipping blank lines so 1-based line numbers match the input stream.
    for (idx, line) in stdin.lock().lines().enumerate() {
        let text = line?;
        if text.trim().is_empty() {
            continue;
        }

        let line = Line::new(idx + 1, text);
        let json = parser.process(line);

        // Safe to unwrap: `Envelope::into_json` produces a guaranteed-serialisable `Value`.
        println!("{}", serde_json::to_string(&json).unwrap());
    }

    Ok(())
}
