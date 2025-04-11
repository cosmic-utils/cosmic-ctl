use crate::{
    commands::Command,
    config::{delete_configuration, read_configuration, write_configuration},
    formats::FileFormat,
    schema::{ConfigFile, EntryContent, Operation},
};
use clap::Args;
use std::{
    fs,
    io::{Error, ErrorKind},
    path::PathBuf,
};

#[derive(Args)]
pub struct ApplyCommand {
    /// Path to the configuration file (supports JSON, YAML, TOML, RON).
    file: PathBuf,
    /// Print verbose output about skipped entries.
    #[arg(short, long)]
    verbose: bool,
}

impl Command for ApplyCommand {
    type Err = Error;

    fn execute(&self) -> Result<(), Self::Err> {
        // Detect file format from extension
        let format = FileFormat::from_path(&self.file)?;

        if self.verbose {
            println!("Using {} format for input file", format.name());
        }

        // Read and parse the configuration file
        let file_content = fs::read_to_string(&self.file)?;
        let config_file: ConfigFile = format.deserialize(&file_content)?;

        let mut delete_count = 0;
        let mut read_count = 0;
        let mut skipped = 0;
        let mut write_changes = 0;

        for entry in config_file.operations {
            match (entry.operation, entry.entries) {
                (Operation::Write, EntryContent::WriteEntries(entries)) => {
                    for (key, value) in entries {
                        match write_configuration(
                            &entry.component,
                            &entry.version,
                            &key,
                            &value,
                            &entry.xdg_directory,
                        ) {
                            Ok(false) => {
                                if self.verbose {
                                    println!(
                                        "Skipping {}/v{}/{} - value unchanged",
                                        entry.component, entry.version, key
                                    );
                                }
                                skipped += 1;
                            }
                            Ok(true) => write_changes += 1,
                            Err(e) => {
                                eprintln!(
                                    "Error writing {}/v{}/{}: {}",
                                    entry.component, entry.version, key, e
                                );
                                skipped += 1;
                            }
                        }
                    }
                }
                (Operation::Read, EntryContent::ReadDeleteEntries(keys)) => {
                    for key in keys {
                        match read_configuration(
                            &entry.component,
                            &entry.version,
                            &key,
                            &entry.xdg_directory,
                        ) {
                            Ok(content) => {
                                println!(
                                    "{}/v{}/{}: {}",
                                    entry.component, entry.version, key, content
                                );
                                read_count += 1;
                            }
                            Err(e) => {
                                if self.verbose {
                                    println!("{}", e);
                                }
                                skipped += 1;
                            }
                        }
                    }
                }
                (Operation::Delete, EntryContent::ReadDeleteEntries(keys)) => {
                    for key in keys {
                        match delete_configuration(
                            &entry.component,
                            &entry.version,
                            &key,
                            &entry.xdg_directory,
                        ) {
                            Ok(()) => {
                                if self.verbose {
                                    println!(
                                        "Deleted: {}/v{}/{}",
                                        entry.component, entry.version, key
                                    );
                                }
                                delete_count += 1;
                            }
                            Err(e) => {
                                if self.verbose {
                                    println!(
                                        "Failed to delete {}/v{}/{}: {}",
                                        entry.component, entry.version, key, e
                                    );
                                }
                                skipped += 1;
                            }
                        }
                    }
                }
                _ => {
                    return Err(Error::new(ErrorKind::InvalidData, "Invalid operation."));
                }
            }
        }

        println!(
            "Operations completed successfully. {} writes, {} reads, {} deletes, {} entries skipped.",
            write_changes, read_count, delete_count, skipped
        );
        Ok(())
    }
}
