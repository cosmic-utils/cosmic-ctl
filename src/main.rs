use clap::{Parser, Subcommand};
use serde::Deserialize;
use std::{
    collections::HashMap,
    env, fs,
    path::{Path, PathBuf},
};
use unescaper::unescape;

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
        /// The configuration version of the component.
        #[arg(short, long, default_value_t = 1)]
        version: u32,
        /// The component to configure (e.g., 'com.system76.CosmicComp').
        #[arg(short, long)]
        component: String,
        /// The specific configuration entry to modify (e.g., 'autotile').
        #[arg(short, long)]
        entry: String,
        /// The value to assign to the configuration entry. (e.g., 'true')
        value: String,
    },
    /// Read a configuration entry.
    #[command(disable_version_flag = true)]
    Read {
        /// The configuration version of the component.
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
        /// The configuration version of the component.
        #[arg(short, long, default_value_t = 1)]
        version: u32,
        /// The component to configure (e.g., 'com.system76.CosmicComp').
        #[arg(short, long)]
        component: String,
        /// The specific configuration entry to modify (e.g., 'autotile').
        #[arg(short, long)]
        entry: String,
    },
    /// Write configurations from a JSON file
    Apply {
        /// Path to the JSON file containing configuration entries.
        file: PathBuf,
    },
}

#[derive(Deserialize)]
struct Entry {
    component: String,
    version: u32,
    entries: HashMap<String, String>,
}

#[derive(Deserialize)]
struct ConfigFile {
    configurations: Vec<Entry>,
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
            apply_configuration(component, version, entry, value);
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
                eprintln!("Error: Configuration entry does not exist.");
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
                eprintln!("Error: Configuration entry does not exist.");
            }
        }
        Commands::Apply { file } => {
            if file.extension().and_then(|s| s.to_str()) != Some("json") {
                eprintln!("Error: The file is not in JSON format.");
                return;
            }

            let file_content = fs::read_to_string(file).expect("Unable to read file");
            let config_file: ConfigFile =
                serde_json::from_str(&file_content).expect("Invalid JSON format");

            for entry in config_file.configurations {
                for (key, value) in entry.entries {
                    apply_configuration(&entry.component, &entry.version, &key, &value);
                }
            }

            println!("Configurations applied successfully.");
        }
    }
}

fn apply_configuration(component: &str, version: &u32, entry: &str, value: &str) {
    let path = get_config_path(component, version, entry);
    let unescaped_value = unescape(value).unwrap();

    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, unescaped_value).unwrap();
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
