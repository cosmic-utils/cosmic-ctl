use crate::{commands::Command, config::delete_configuration};
use clap::Args;
use std::io::Error;

#[derive(Args)]
pub struct DeleteCommand {
    /// The configuration version of the component.
    #[arg(short, long, default_value_t = 1)]
    version: u64,
    /// The component to configure (e.g., 'com.system76.CosmicComp').
    #[arg(short, long)]
    component: String,
    /// The specific configuration entry to modify (e.g., 'autotile').
    #[arg(short, long)]
    entry: String,
    /// The XDG directory to use (e.g., 'config', 'cache', 'data').
    #[arg(short, long, default_value = "config")]
    xdg_dir: String,
}

impl Command for DeleteCommand {
    type Err = Error;

    fn execute(&self) -> Result<(), Self::Err> {
        match delete_configuration(&self.component, &self.version, &self.entry, &self.xdg_dir) {
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
