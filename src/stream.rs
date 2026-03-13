use crate::errors::LogTidyError;
use crate::line::Line;
use crate::parser::Parser;
use std::io::{self, BufRead};

/// Runs logtidy using a custom JSON printer that preserves key order.
pub fn run_with_printer<F>(printer: F) -> Result<(), LogTidyError>
where
    F: Fn(&serde_json::Value) -> String,
{
    let stdin = io::stdin();
    let parser = Parser::new();

    for (idx, line) in stdin.lock().lines().enumerate() {
        let text = line?;
        if text.trim().is_empty() {
            continue;
        }

        let line = Line::new(idx + 1, text);
        let json = parser.process(line);

        println!("{}", printer(&json));
    }

    Ok(())
}
