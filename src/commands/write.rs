use crate::{commands::Command, config::write_configuration};
use clap::Args;
use std::io::Error;

#[derive(Args)]
pub struct WriteCommand {
    /// The configuration version of the component.
    #[arg(short, long, default_value_t = 1)]
    version: u64,
    /// The component to configure (e.g., 'com.system76.CosmicComp').
    #[arg(short, long)]
    component: String,
    /// The specific configuration entry to modify (e.g., 'autotile').
    #[arg(short, long)]
    entry: String,
    /// The value to assign to the configuration entry. (e.g., 'true').
    value: String,
}

impl Command for WriteCommand {
    type Err = Error;

    fn execute(&self) -> Result<(), Self::Err> {
        match write_configuration(&self.component, &self.version, &self.entry, &self.value) {
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
