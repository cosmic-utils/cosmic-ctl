use crate::{
    commands::Command,
    config::{write_configuration, write_configuration_file},
};
use clap::Args;
use std::{io::Error, path::PathBuf};

#[derive(Args)]
pub struct WriteCommand {
    /// The configuration version of the component.
    #[arg(short, long, default_value_t = 1)]
    version: u64,
    /// The component to configure (e.g., 'com.system76.CosmicComp').
    #[arg(short, long, required_unless_present = "file")]
    component: Option<String>,
    /// The specific configuration entry to modify (e.g., 'autotile').
    #[arg(short, long, required_unless_present = "file")]
    entry: Option<String>,
    /// The value to assign to the configuration entry. (e.g., 'true').
    value: String,
    /// The XDG directory to use (e.g., 'config', 'cache', 'data').
    #[arg(short, long, default_value = "config")]
    xdg_dir: String,
    /// Direct path to the configuration file.
    #[arg(long, required_unless_present_all = &["component", "entry"])]
    file: Option<PathBuf>,
}

impl Command for WriteCommand {
    type Err = Error;

    fn execute(&self) -> Result<(), Self::Err> {
        if let Some(file_path) = &self.file {
            match write_configuration_file(file_path, &self.value) {
                Ok(true) => {
                    println!("Configuration file written successfully.");
                    Ok(())
                }
                Ok(false) => {
                    println!("Doing nothing. Configuration file already has the same value.");
                    Ok(())
                }
                Err(e) => {
                    eprintln!("Error writing configuration file: {}", e);
                    Err(e)
                }
            }
        } else {
            match write_configuration(
                self.component.as_ref().unwrap(),
                &self.version,
                self.entry.as_ref().unwrap(),
                &self.value,
                &self.xdg_dir,
            ) {
                Ok(true) => {
                    println!("Configuration entry written successfully.");
                    Ok(())
                }
                Ok(false) => {
                    println!("Doing nothing. Configuration entry already has the same value.");
                    Ok(())
                }
                Err(e) => {
                    eprintln!("Error writing configuration entry: {}", e);
                    Err(e)
                }
            }
        }
    }
}
