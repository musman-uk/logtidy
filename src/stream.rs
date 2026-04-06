use crate::errors::LogTidyError;
use crate::line::Line;
use crate::parser::Parser;
use std::io::{self, BufRead};

/// Runs logtidy by reading from stdin and printing directly to stdout.
/// This is the original behaviour and remains unchanged.
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

/// Runs logtidy on a provided string and returns the structured output.
/// This is used by the CLI so it can accept input from a file or stdin.
pub fn run_with_input(input: &str) -> Result<String, LogTidyError> {
    let parser = Parser::new();
    let mut output = String::new();

    for (idx, raw_line) in input.lines().enumerate() {
        let text = raw_line.trim();
        if text.is_empty() {
            continue;
        }

        let line = Line::new(idx + 1, text.to_string());
        let json = parser.process(line);

        output.push_str(&serde_json::to_string(&json).unwrap());
        output.push('\n');
    }

    Ok(output)
}
