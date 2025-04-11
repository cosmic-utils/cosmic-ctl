mod commands;
mod config;
mod formats;
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

    if let Err(e) = cli.command.execute() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
