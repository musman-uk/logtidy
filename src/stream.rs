use crate::errors::LogTidyError;
use crate::line::Line;
use crate::parser::Parser;
use std::io::{self, BufRead};

/// Runs logtidy with a fixed JSON structure and consistent key order.
pub fn run() -> Result<(), LogTidyError> {
    let stdin = io::stdin();
    let parser = Parser::new();

    for (idx, line) in stdin.lock().lines().enumerate() {
        let text = line?;
        if text.trim().is_empty() {
            continue;
        }

        let line = Line::new(idx + 1, text);
        let json = parser.process(line);

        // Serde preserves insertion order from Envelope::into_json()
        println!("{}", serde_json::to_string(&json).unwrap());
    }

    Ok(())
}
