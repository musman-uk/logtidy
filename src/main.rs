mod envelope;
mod errors;
mod line;
mod parser;
mod stream;

fn main() {
    if let Err(e) = stream::run() {
        eprintln!("logtidy error: {}", e);
        std::process::exit(1);
    }
}
