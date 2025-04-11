use crate::{
    commands::Command,
    config::{
        delete_configuration, delete_configuration_file, read_configuration,
        read_configuration_file, write_configuration, write_configuration_file,
    },
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
        let file_format = FileFormat::from_path(&self.file)?;

        if self.verbose {
            println!("Using {} format for input file", file_format.name());
        }

        let file_content = fs::read_to_string(&self.file)?;
        let config_file: ConfigFile = file_format.deserialize(&file_content)?;

        let mut delete_count = 0;
        let mut read_count = 0;
        let mut skipped = 0;
        let mut write_changes = 0;

        for entry in config_file.operations {
            if let Some(file_path) = &entry.file {
                match entry.operation {
                    Operation::Write => {
                        let value = entry.value.ok_or_else(|| {
                            Error::new(
                                ErrorKind::InvalidData,
                                "Value is required for file write operations",
                            )
                        })?;

                        match write_configuration_file(file_path, &value) {
                            Ok(false) => {
                                if self.verbose {
                                    println!("Skipping {} - value unchanged", file_path.display());
                                }
                                skipped += 1;
                            }
                            Ok(true) => {
                                if self.verbose {
                                    println!("Wrote to {}", file_path.display());
                                }
                                write_changes += 1;
                            }
                            Err(e) => {
                                eprintln!("Error writing {}: {}", file_path.display(), e);
                                skipped += 1;
                            }
                        }
                    }
                    Operation::Read => match read_configuration_file(file_path) {
                        Ok(content) => {
                            println!("{}: {}", file_path.display(), content);
                            read_count += 1;
                        }
                        Err(e) => {
                            if self.verbose {
                                println!("Error reading {}: {}", file_path.display(), e);
                            }
                            skipped += 1;
                        }
                    },
                    Operation::Delete => match delete_configuration_file(file_path) {
                        Ok(()) => {
                            if self.verbose {
                                println!("Deleted: {}", file_path.display());
                            }
                            delete_count += 1;
                        }
                        Err(e) => {
                            if self.verbose {
                                println!("Failed to delete {}: {}", file_path.display(), e);
                            }
                            skipped += 1;
                        }
                    },
                }
                continue;
            }

            let component = entry.component.as_deref().ok_or_else(|| {
                Error::new(
                    ErrorKind::InvalidData,
                    "Component is required when file is not specified",
                )
            })?;
            let version = entry.version.as_ref().ok_or_else(|| {
                Error::new(
                    ErrorKind::InvalidData,
                    "Version is required when file is not specified",
                )
            })?;
            let xdg_dir = entry.xdg_directory.as_deref().unwrap_or("config");

            let entries = entry.entries.ok_or_else(|| {
                Error::new(
                    ErrorKind::InvalidData,
                    "Entries are required when file is not specified",
                )
            })?;

            match (entry.operation, entries) {
                (Operation::Write, EntryContent::WriteEntries(entries)) => {
                    for (key, value) in entries {
                        match write_configuration(component, version, &key, &value, xdg_dir) {
                            Ok(false) => {
                                if self.verbose {
                                    println!(
                                        "Skipping {}/v{}/{} - value unchanged",
                                        component, version, key
                                    );
                                }
                                skipped += 1;
                            }
                            Ok(true) => write_changes += 1,
                            Err(e) => {
                                eprintln!(
                                    "Error writing {}/v{}/{}: {}",
                                    component, version, key, e
                                );
                                skipped += 1;
                            }
                        }
                    }
                }
                (Operation::Read, EntryContent::ReadDeleteEntries(keys)) => {
                    for key in keys {
                        match read_configuration(component, version, &key, xdg_dir) {
                            Ok(content) => {
                                println!("{}/v{}/{}: {}", component, version, key, content);
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
                        match delete_configuration(component, version, &key, xdg_dir) {
                            Ok(()) => {
                                if self.verbose {
                                    println!("Deleted: {}/v{}/{}", component, version, key);
                                }
                                delete_count += 1;
                            }
                            Err(e) => {
                                if self.verbose {
                                    println!(
                                        "Failed to delete {}/v{}/{}: {}",
                                        component, version, key, e
                                    );
                                }
                                skipped += 1;
                            }
                        }
                    }
                }
                _ => {
                    return Err(Error::new(
                        ErrorKind::InvalidData,
                        "Invalid operation configuration.",
                    ));
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
