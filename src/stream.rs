use crate::errors::LogTidyError;
use crate::line::Line;
use crate::parser::Parser;
use std::io::{self, BufRead};

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
        println!("{}", json);
    }

    Ok(())
}
