use crate::{
    commands::Command,
    config::{delete_configuration, get_cosmic_configurations, parse_configuration_path},
    utils::split_string_respect_braces,
};
use bracoxide::explode;
use clap::Args;
use glob::Pattern;
use std::io::{stdin, stdout, Error, Write};
use walkdir::WalkDir;

#[derive(Args)]
pub struct ResetCommand {
    /// Skip confirmation prompt.
    #[arg(short, long)]
    force: bool,
    /// Show which entries are being deleted.
    #[arg(short, long)]
    verbose: bool,
    /// Patterns to exclude from reset (comma-separated).
    #[arg(long)]
    exclude: Option<String>,
    /// The XDG directories to backup (comma-separated) (e.g., 'config,cache,data').
    #[arg(short, long, value_delimiter = ',', default_value = "config,data")]
    xdg_dirs: Vec<String>,
}

impl Command for ResetCommand {
    type Err = Error;

    fn execute(&self) -> Result<(), Self::Err> {
        if !self.force {
            print!("Are you sure you want to delete all configuration entries from XDG directories {}? This action cannot be undone. [y/N] ",
                self.xdg_dirs.join(", "));
            stdout().flush()?;

            let mut response = String::new();
            stdin().read_line(&mut response)?;

            if !response.trim().eq_ignore_ascii_case("y") {
                println!("Operation cancelled.");
                return Ok(());
            }
        }

        let mut total_deleted_count = 0;
        let mut all_errors = Vec::new();

        for xdg_dir in &self.xdg_dirs {
            let cosmic_path = get_cosmic_configurations(xdg_dir)?;
            let mut deleted_count = 0;
            let mut errors = Vec::new();

            if !cosmic_path.exists() {
                if self.verbose {
                    println!("No configuration entries found in {}.", xdg_dir);
                }
                errors.push(format!("No configuration entries found in {}.", xdg_dir));
                continue;
            }

            let exclude_patterns = split_string_respect_braces(self.exclude.clone())
                .into_iter()
                .flat_map(|pattern| explode(&pattern).unwrap_or_else(|_| vec![pattern.clone()]))
                .filter_map(|pattern| {
                    let pattern = if !pattern.contains('/') {
                        format!("{}/**", pattern)
                    } else if pattern.matches('/').count() == 1 {
                        format!("{}/*", pattern)
                    } else {
                        pattern
                    };

                    match Pattern::new(&pattern) {
                        Ok(p) => Some(p),
                        Err(e) => {
                            errors.push(format!("Invalid exclude pattern '{}': {}", pattern, e));
                            None
                        }
                    }
                })
                .collect::<Vec<_>>();

            for entry in WalkDir::new(&cosmic_path)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.path().is_file())
            {
                if let Some((component, version, entry_name)) =
                    parse_configuration_path(entry.path())
                {
                    let relative_path = format!("{}/v{}/{}", component, version, entry_name);
                    let should_exclude = exclude_patterns
                        .iter()
                        .any(|pattern| pattern.matches(&relative_path));

                    if should_exclude {
                        if self.verbose {
                            println!("Skipping excluded path [{}]: {}", xdg_dir, relative_path);
                        }
                        continue;
                    }

                    if self.verbose {
                        println!("Deleting [{}]: {}", xdg_dir, entry.path().display());
                    }

                    match delete_configuration(&component, &version, &entry_name, xdg_dir) {
                        Ok(()) => deleted_count += 1,
                        Err(e) => {
                            errors.push(format!("[{}] {}: {}", xdg_dir, entry.path().display(), e))
                        }
                    }
                }
            }

            if self.verbose {
                println!(
                    "Completed reset for {} directory: {} entries deleted",
                    xdg_dir, deleted_count
                );
            }

            total_deleted_count += deleted_count;
            all_errors.extend(errors);
        }

        if all_errors.is_empty() {
            println!(
                "Successfully deleted {} configuration entries.",
                total_deleted_count
            );
        } else {
            println!(
                "Deleted {} configuration entries with {} errors:",
                total_deleted_count,
                all_errors.len()
            );
            for error in all_errors {
                eprintln!("Error: {}", error);
            }
        }

        Ok(())
    }
}
