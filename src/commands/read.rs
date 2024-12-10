use crate::{commands::Command, config::read_configuration};
use clap::Args;
use std::io::Error;

#[derive(Args)]
pub struct ReadCommand {
    /// The configuration version of the component.
    #[arg(short, long, default_value_t = 1)]
    version: u64,
    /// The component to configure (e.g., 'com.system76.CosmicComp').
    #[arg(short, long)]
    component: String,
    /// The specific configuration entry to modify (e.g., 'autotile').
    #[arg(short, long)]
    entry: String,
}

impl Command for ReadCommand {
    type Err = Error;

    fn execute(&self) -> Result<(), Self::Err> {
        match read_configuration(&self.component, &self.version, &self.entry) {
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
