use crate::{
    commands::Command,
    config::{read_configuration, read_configuration_file},
};
use clap::Args;
use std::{io::Error, path::PathBuf};

#[derive(Args)]
pub struct ReadCommand {
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

impl Command for ReadCommand {
    type Err = Error;

    fn execute(&self) -> Result<(), Self::Err> {
        if let Some(file_path) = &self.file {
            match read_configuration_file(file_path) {
                Ok(value) => {
                    println!("{}", value);
                    Ok(())
                }
                Err(e) => {
                    eprintln!("Error reading configuration file: {}", e);
                    Err(e)
                }
            }
        } else {
            match read_configuration(
                self.component.as_ref().unwrap(),
                &self.version,
                self.entry.as_ref().unwrap(),
                &self.xdg_dir,
            ) {
                Ok(value) => {
                    println!("{}", value);
                    Ok(())
                }
                Err(e) => {
                    eprintln!("Error reading configuration entry: {}", e);
                    Err(e)
                }
            }
        }
    }
}
