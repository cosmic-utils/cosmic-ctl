use crate::{
    commands::Command,
    config::{delete_configuration, delete_configuration_file},
};
use clap::Args;
use std::{io::Error, path::PathBuf};

#[derive(Args)]
pub struct DeleteCommand {
    /// The configuration version of the component.
    #[arg(short, long, default_value_t = 1)]
    version: u64,
    /// The component to configure (e.g., 'com.system76.CosmicComp').
    #[arg(short, long, required_unless_present = "file")]
    component: Option<String>,
    /// The specific configuration entry to modify (e.g., 'autotile').
    #[arg(short, long, required_unless_present = "file")]
    entry: Option<String>,
    /// The XDG directory to use (e.g., 'config', 'cache', 'data').
    #[arg(short, long, default_value = "config")]
    xdg_dir: String,
    /// Direct path to the configuration file.
    #[arg(short, long, required_unless_present_all = &["component", "entry"])]
    file: Option<PathBuf>,
}

impl Command for DeleteCommand {
    type Err = Error;

    fn execute(&self) -> Result<(), Self::Err> {
        if let Some(file_path) = &self.file {
            match delete_configuration_file(file_path) {
                Ok(()) => {
                    println!("Configuration file deleted successfully.");
                    Ok(())
                }
                Err(e) => {
                    eprintln!("Error deleting configuration file: {}", e);
                    Err(e)
                }
            }
        } else {
            match delete_configuration(
                self.component.as_ref().unwrap(),
                &self.version,
                self.entry.as_ref().unwrap(),
                &self.xdg_dir,
            ) {
                Ok(()) => {
                    println!("Configuration entry deleted successfully.");
                    Ok(())
                }
                Err(e) => {
                    eprintln!("Error deleting configuration entry: {}", e);
                    Err(e)
                }
            }
        }
    }
}
