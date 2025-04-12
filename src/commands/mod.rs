pub mod apply;
pub mod backup;
pub mod delete;
pub mod read;
pub mod reset;
pub mod write;

use crate::commands::{
    apply::ApplyCommand, backup::BackupCommand, delete::DeleteCommand, read::ReadCommand,
    reset::ResetCommand, write::WriteCommand,
};
use clap::Subcommand;
use std::io::Error;

#[derive(Subcommand)]
pub enum Commands {
    /// Write configurations from a JSON file.
    Apply(ApplyCommand),
    /// Backup all configuration entries to a JSON file.
    Backup(BackupCommand),
    /// Delete a configuration entry.
    #[command(disable_version_flag = true)]
    Delete(DeleteCommand),
    /// Read a configuration entry.
    #[command(disable_version_flag = true)]
    Read(ReadCommand),
    /// Delete all configuration entries.
    Reset(ResetCommand),
    /// Write a configuration entry.
    #[command(disable_version_flag = true)]
    Write(WriteCommand),
}

impl Commands {
    pub(crate) fn execute(&self) -> Result<(), Error> {
        match self {
            Commands::Apply(cmd) => cmd.execute(),
            Commands::Backup(cmd) => cmd.execute(),
            Commands::Delete(cmd) => cmd.execute(),
            Commands::Read(cmd) => cmd.execute(),
            Commands::Reset(cmd) => cmd.execute(),
            Commands::Write(cmd) => cmd.execute(),
        }
    }
}

pub trait Command {
    type Err;

    fn execute(&self) -> Result<(), Self::Err>;
}
