use clap::{Parser, Subcommand};

/// CLI for COSMIC Desktop configuration management
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Write a configuration entry.
    #[command(disable_version_flag = true)]
    Write {
        /// Configuration version to use.
        #[arg(short, long, default_value_t = 1)]
        version: u32,
        /// The component to configure (e.g., 'com.system76.CosmicComp').
        #[arg(short, long)]
        component: String,
        /// The specific configuration entry to modify (e.g., 'autotile').
        #[arg(short, long)]
        entry: String,
        /// The value to assign to the configuration entry.
        value: String,
    },
    /// Read a configuration entry.
    #[command(disable_version_flag = true)]
    Read {
        /// Configuration version to use.
        #[arg(short, long, default_value_t = 1)]
        version: u32,
        /// The component to configure (e.g., 'com.system76.CosmicComp').
        #[arg(short, long)]
        component: Option<String>,
        /// The specific configuration entry to modify (e.g., 'autotile').
        #[arg(short, long)]
        entry: Option<String>,
    },
    /// Delete a configuration entry.
    #[command(disable_version_flag = true)]
    Delete {
        /// Configuration version to use.
        #[arg(short, long, default_value_t = 1)]
        version: u32,
        /// The component to configure (e.g., 'com.system76.CosmicComp').
        #[arg(short, long)]
        component: String,
        /// The specific configuration entry to modify (e.g., 'autotile').
        #[arg(short, long)]
        entry: String,
    },
}

fn main() {
    let cli: Cli = Cli::parse();

    match &cli.command {
        Commands::Write {
            component,
            version,
            entry,
            value,
        } => {
            println!(
                "Component: {}, Version: {}, Entry: {}, Value: {}",
                component, version, entry, value
            )
        }
        Commands::Read {
            version,
            component,
            entry,
        } => {
            println!(
                "Component: {:?}, Version: {}, Entry: {:?}",
                component, version, entry
            )
        }
        Commands::Delete {
            version,
            component,
            entry,
        } => {
            println!(
                "Component: {}, Version: {}, Entry: {}",
                component, version, entry
            )
        }
    }
}
