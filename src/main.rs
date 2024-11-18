use clap::{Parser, Subcommand};
use std::{
    env, fs,
    path::{Path, PathBuf},
};

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
        component: String,
        /// The specific configuration entry to modify (e.g., 'autotile').
        #[arg(short, long)]
        entry: String,
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
            let path = get_config_path(component, version, entry);
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            fs::write(path, value).unwrap();
            println!("Configuration entry written successfully.");
        }
        Commands::Read {
            version,
            component,
            entry,
        } => {
            let path = get_config_path(component, version, entry);
            if path.exists() {
                let contents = fs::read_to_string(path).unwrap();
                println!("{}", contents);
            } else {
                eprintln!("Configuration entry does not exist.");
            }
        }
        Commands::Delete {
            version,
            component,
            entry,
        } => {
            let path = get_config_path(component, version, entry);
            if path.exists() {
                fs::remove_file(path).unwrap();
                println!("Configuration entry deleted successfully.");
            } else {
                println!("Configuration entry does not exist.");
            }
        }
    }
}

fn get_config_path(component: &str, version: &u32, entry: &str) -> PathBuf {
    let config_home = get_config_home();

    Path::new(&config_home)
        .join("cosmic")
        .join(component)
        .join(format!("v{}", version))
        .join(entry)
}

fn get_config_home() -> String {
    env::var("XDG_CONFIG_HOME").unwrap_or_else(|_| {
        let home = env::var("HOME").unwrap();
        format!("{}/.config", home)
    })
}
