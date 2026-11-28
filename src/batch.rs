use crate::errors::LogTidierError;
use crate::parser::Parser;
use crate::line::Line;

pub fn batch(input: &str) -> Result<String, LogTidierError> {
    let parser = Parser::new();
    let mut output = String::new();

    // Enumerate before skipping blank lines so 1-based line numbers stay aligned
    // with the original input positions.
    for (idx, raw_line) in input.lines().enumerate() {
        let text = raw_line.trim();
        if text.is_empty() {
            continue;
        }

        let line = Line::new(idx + 1, text.to_string());
        let json = parser.process(line);

        // Safe to unwrap: `Envelope::into_json` builds a self-contained `Value` tree
        // with string keys and valid JSON primitives, so serialisation cannot fail.
        output.push_str(&serde_json::to_string(&json).unwrap());
        output.push('\n');
    }

    Ok(output)
}
