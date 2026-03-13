mod envelope;
mod errors;
mod line;
mod parser;
mod stream;
mod print; 

use crate::print::to_json_preserve_order;

fn main() {
    if let Err(e) = stream::run_with_printer(to_json_preserve_order) {
        eprintln!("logtidy error: {}", e);
        std::process::exit(1);
    }
}
