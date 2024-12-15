mod commands;
mod config;
mod schema;
#[cfg(test)]
mod tests;
mod utils;

use crate::commands::Commands;
use clap::Parser;

/// CLI for COSMIC Desktop configuration management
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        cmd => {
            if let Err(e) = cmd.execute() {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
    }
}
