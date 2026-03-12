mod parser;
mod envelope;
mod line;
mod stream;
mod errors;

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        match line {
            Ok(raw) => {
                let escaped = raw.replace('"', "\\\"");
                println!("{{\"raw\": \"{}\"}}", escaped);
            }
            Err(e) => {
                eprintln!("logtidy: failed to read input: {}", e);
                std::process::exit(1);
            }
        }
    }
}
