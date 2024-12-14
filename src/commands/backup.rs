use crate::{
    commands::Command,
    config::{get_cosmic_configurations, parse_configuration_path, read_configuration},
    schema::{ConfigFile, Entry, EntryContent, Operation},
};
use clap::Args;
use std::{collections::HashMap, fs, io::Error, path::PathBuf};
use walkdir::WalkDir;

#[derive(Args)]
pub struct BackupCommand {
    /// Path to the output JSON file.
    file: PathBuf,
    /// Show which entries are being backed up.
    #[arg(short, long)]
    verbose: bool,
    /// The XDG directories to backup (comma-separated) (e.g., 'config,cache,data').
    #[arg(short, long, value_delimiter = ',', default_value = "config,data")]
    xdg_dirs: Vec<String>,
}

impl Command for BackupCommand {
    type Err = Error;

    fn execute(&self) -> Result<(), Self::Err> {
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
            schema: "https://raw.githubusercontent.com/HeitorAugustoLN/cosmic-ctl/refs/heads/main/schema.json".to_string(),
            operations: all_operations,
        };

        let json_data = serde_json::to_string_pretty(&backup_data)?;
        fs::write(&self.file, json_data)?;

        println!(
            "Backup completed successfully. {} total entries backed up.",
            total_entry_count
        );
        Ok(())
    }
}
