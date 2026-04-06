use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::io::Read; // Needed for read_to_string()
use anyhow::Result;  

mod envelope;
mod errors;
mod line;
mod parser;
mod stream;

// Import the tidy() function from your library API
use logtidy::tidy;

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
    /// Clean and normalise logs
    Tidy {
        /// Input file (optional)
        #[arg(short, long)]
        file: Option<PathBuf>,
    },

    /// Show example input/output
    Sample,

    /// Show documentation
    Docs,

    /// Show credits
    About,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Tidy { file }) => {
            let input = read_input(file)?;
            let output = tidy(&input)?;
            print!("{output}");
        }

        Some(Commands::Sample) => {
            println!("Example input:");
            println!("  [raw log line]");
            println!("Example output:");
            println!("  {{ \"timestamp\": ..., \"message\": ... }}");
        }

        Some(Commands::Docs) => {
            println!("logtidy — documentation");
            println!("------------------------------------------------");
            println!("Purpose:");
            println!("  Convert raw, inconsistent logs into structured, predictable output.");
            println!();
            println!("Usage:");
            println!("  logtidy tidy --file <path>");
            println!("  cat app.log | logtidy tidy");
            println!();
            println!("Input:");
            println!("  - Raw log lines (text)");
            println!("  - Stdin or file");
            println!();
            println!("Output:");
            println!("  - Structured logs (JSON)");
            println!();
            println!("Full documentation:");
            println!("  https://github.com/musman-uk/logtidy");
        }

        Some(Commands::About) => {
            println!("logtidy — a small developer tool");
            println!("Author: Mohammed Usman");
            println!("Repository: https://github.com/musman-uk/logtidy");
        }

        None => {
            // Main menu
            println!("logtidy — normalise logs into structured form");
            println!("------------------------------------------------");
            println!("Commands:");
            println!("  tidy      Normalise logs");
            println!("  sample    Show example input/output");
            println!("  docs      Show documentation");
            println!("  about     Show credits");
            println!("  help      Show help");
        }
    }

    Ok(())
}

/// Reads from a file if provided, otherwise from stdin
fn read_input(file: Option<PathBuf>) -> anyhow::Result<String> {
    if let Some(path) = file {
        Ok(std::fs::read_to_string(path)?)
    } else {
        let mut buf = String::new();
        std::io::stdin().read_to_string(&mut buf)?;
        Ok(buf)
    }
}
