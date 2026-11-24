use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::io::Read;
use anyhow::{Result, Context};

mod envelope;
mod errors;
mod line;
mod parser;
mod stream;
mod batch;

/// CLI definition
#[derive(Parser)]
#[command(
    name = "logtidy",
    version,
    about = "Normalise logs into structured form",
    long_about = None
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

/// Subcommands
#[derive(Subcommand)]
enum Commands {
    /// Batch mode: process entire input at once
    Batch {
        /// Input file (optional)
        #[arg(short, long)]
        file: Option<PathBuf>,
    },

    /// Stream mode: read from stdin line-by-line
    Stream,

    /// Show example input/output
    Sample,

    /// Show documentation
    Docs,

    /// Show tool information
    About,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Batch { file }) => {
            let input = read_input(file)?;
            let output = batch::batch(&input)?;
            print!("{output}");
        }

        Some(Commands::Stream) => {
            stream::stream()?;
        }

        Some(Commands::Sample) => {
            println!("sample");
            println!("------------------------------------------------");
            println!("Example input:");
            println!("  2024-03-10T12:00:01Z INFO Starting service");
            println!("Example output:");
            println!("  {{\"line\":1,\"message\":\"Starting service\",\"timestamp\":\"2024-03-10T12:00:01Z\",\"level\":\"INFO\",\"fields\":null}}");
        }

        Some(Commands::Docs) => {
            println!("docs");
            println!("------------------------------------------------");
            println!("Purpose:");
            println!("  Convert raw logs into structured, predictable JSONL output.");
            println!();
            println!("Usage:");
            println!("  logtidy batch --file <path>");
            println!("  cat app.log | logtidy stream");
            println!();
            println!("Input:");
            println!("  - Raw, semi-structured, or JSON log lines");
            println!("  - Stdin or file");
            println!();
            println!("Output:");
            println!("  - Structured logs (JSONL)");
            println!();
            println!("Examples:");
            println!("  For example input/output, run:");
            println!("    logtidy sample");
            println!();
            println!("Full Documentation:");
            println!("  https://github.com/musman-uk/logtidy");
        }

        Some(Commands::About) => {
            println!("about");
            println!("------------------------------------------------");
            println!("Name: LogTidy");
            println!("Version: {}", env!("CARGO_PKG_VERSION"));
            println!("License: {}", env!("CARGO_PKG_LICENSE"));
            println!("Source: https://github.com/musman-uk/logtidy");
            println!();
            println!("© Mohammed Usman");
        }

        None => {
            println!("logtidy");
            println!("------------------------------------------------");
            println!("Normalise logs into structured form");
            println!();
            println!("Commands:");
            println!("  batch     Process entire input at once");
            println!("  stream    Read from stdin line-by-line");
            println!("  sample    Show example input/output");
            println!("  docs      Show condensed documentation");
            println!("  about     Show tool information");
            println!("  help      Show help");
        }
    }

    Ok(())
}

/// Reads from a file if provided, otherwise from stdin
fn read_input(file: Option<PathBuf>) -> Result<String> {
    if let Some(path) = file {
        Ok(std::fs::read_to_string(&path)
            .with_context(|| format!("Failed to read file: {}", path.display()))?)
    } else {
        let mut buf = String::new();
        std::io::stdin()
            .read_to_string(&mut buf)
            .context("Failed to read from stdin")?;
        Ok(buf)
    }
}