mod commands;
mod config;
mod formats;
mod interactive;
mod schema;
#[cfg(test)]
mod tests;
mod utils;

use crate::{commands::Commands, interactive::run_interactive_mode};
use clap::Parser;

/// CLI for COSMIC Desktop configuration management
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Some(cmd) => cmd.execute(),
        None => run_interactive_mode(),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
