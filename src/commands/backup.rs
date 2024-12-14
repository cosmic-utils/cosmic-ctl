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
}

impl Command for BackupCommand {
    type Err = Error;

    fn execute(&self) -> Result<(), Self::Err> {
        let cosmic_path = get_cosmic_configurations()?;
        let mut operations: HashMap<(String, u64), HashMap<String, String>> = HashMap::new();
        let mut entry_count = 0;

        for entry in WalkDir::new(cosmic_path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_file())
        {
            if let Some((component, version, entry_name)) = parse_configuration_path(entry.path()) {
                match read_configuration(&component, &version, &entry_name) {
                    Ok(content) => {
                        if self.verbose {
                            println!("Backing up: {}/v{}/{}", component, version, entry_name);
                        }

                        operations
                            .entry((component.clone(), version))
                            .or_default()
                            .insert(entry_name, content);

                        entry_count += 1;
                    }
                    Err(e) => {
                        if self.verbose {
                            println!(
                                "Failed to backup {}/v{}/{}: {}",
                                component, version, entry_name, e
                            );
                        }
                    }
                }
            }
        }

        let backup_data = ConfigFile {
                schema: "https://raw.githubusercontent.com/HeitorAugustoLN/cosmic-ctl/refs/heads/main/schema.json".to_string(),
                operations: operations
                    .into_iter()
                    .map(|((component, version), entries)| Entry {
                        component,
                        version,
                        operation: Operation::Write,
                        entries: EntryContent::WriteEntries(entries),
                })
                .collect(),
            };
        let json_data = serde_json::to_string_pretty(&backup_data)?;

        fs::write(&self.file, json_data)?;
        println!(
            "Backup completed successfully. {} entries backed up.",
            entry_count
        );
        Ok(())
    }
}
