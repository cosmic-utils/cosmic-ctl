use crate::{
    commands::Command,
    config::{get_cosmic_configurations, parse_configuration_path, read_configuration},
    formats::FileFormat,
    schema::{ConfigFile, Entry, EntryContent, Operation},
};
use clap::Args;
use std::{
    collections::HashMap,
    fs,
    io::{Error, ErrorKind},
    path::PathBuf,
};
use walkdir::WalkDir;

#[derive(Args)]
pub struct BackupCommand {
    /// Path to the output configuration file (supports JSON, YAML, TOML, RON).
    file: PathBuf,
    /// Show which entries are being backed up.
    #[arg(short, long)]
    verbose: bool,
    /// The XDG directories to backup (comma-separated) (e.g., 'config,cache,data').
    #[arg(short, long, value_delimiter = ',', default_value = "config,state")]
    xdg_dirs: Vec<String>,
    /// Output format (auto-detected from file extension if not specified).
    #[arg(short, long)]
    format: Option<String>,
}

impl Command for BackupCommand {
    type Err = Error;

    fn execute(&self) -> Result<(), Self::Err> {
        let format = match &self.format {
            Some(fmt) => match fmt.to_lowercase().as_str() {
                "json" => FileFormat::Json,
                "yaml" | "yml" => FileFormat::Yaml,
                "toml" => FileFormat::Toml,
                "ron" => FileFormat::Ron,
                _ => {
                    return Err(Error::new(
                        ErrorKind::InvalidInput,
                        format!("Unsupported format: {}", fmt),
                    ))
                }
            },
            None => FileFormat::from_path(&self.file)?,
        };

        if self.verbose {
            println!("Using {} format for output file", format.name());
        }

        let mut total_entry_count = 0;
        let mut all_operations = Vec::new();

        for xdg_dir in &self.xdg_dirs {
            let cosmic_path = get_cosmic_configurations(xdg_dir)?;
            let mut operations: HashMap<(String, u64), HashMap<String, String>> = HashMap::new();
            let mut entry_count = 0;

            for entry in WalkDir::new(&cosmic_path)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.file_type().is_file())
            {
                if let Some((component, version, entry_name)) =
                    parse_configuration_path(&entry.path())
                {
                    match read_configuration(&component, &version, &entry_name, xdg_dir) {
                        Ok(content) => {
                            if self.verbose {
                                println!(
                                    "Backing up [{}]: {}/v{}/{}",
                                    xdg_dir, component, version, entry_name
                                );
                            }

                            operations
                                .entry((component, version))
                                .or_default()
                                .insert(entry_name, content);

                            entry_count += 1;
                        }
                        Err(e) => {
                            if self.verbose {
                                println!(
                                    "Failed to backup [{}] {}/v{}/{}: {}",
                                    xdg_dir, component, version, entry_name, e
                                );
                            }
                        }
                    }
                }
            }

            let xdg_entries: Vec<Entry> = operations
                .into_iter()
                .map(|((component, version), entries)| Entry {
                    component,
                    version,
                    operation: Operation::Write,
                    entries: EntryContent::WriteEntries(entries),
                    xdg_directory: xdg_dir.to_string(),
                })
                .collect();

            all_operations.extend(xdg_entries);
            total_entry_count += entry_count;

            if self.verbose {
                println!(
                    "Completed backup for {} directory: {} entries",
                    xdg_dir, entry_count
                );
            }
        }

        let backup_data = ConfigFile {
            // RON doesn't support JSON schemas
            schema: if format != FileFormat::Ron {
                Some("https://raw.githubusercontent.com/cosmic-utils/cosmic-ctl/refs/heads/main/schema.json".to_string())
            } else {
                None
            },
            operations: all_operations,
        };

        let formatted_data = format.serialize(&backup_data)?;
        fs::write(&self.file, formatted_data)?;

        println!(
            "Backup completed successfully. {} total entries backed up in {} format.",
            total_entry_count,
            format.name()
        );
        Ok(())
    }
}
