use crate::errors::LogTidyError;
use crate::parser::Parser;
use crate::line::Line;

/// Batch mode: process an entire input string and return structured output.
pub fn batch(input: &str) -> Result<String, LogTidyError> {
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
