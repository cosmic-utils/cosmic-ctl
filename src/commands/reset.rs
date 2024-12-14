use crate::{
    commands::Command,
    config::{delete_configuration, get_cosmic_configurations, parse_configuration_path},
    utils::split_string_respect_braces,
};
use bracoxide::explode;
use clap::Args;
use glob::Pattern;
use std::io::{stdin, stdout, Error, ErrorKind, Write};
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
}

impl Command for ResetCommand {
    type Err = Error;

    fn execute(&self) -> Result<(), Self::Err> {
        if !self.force {
            print!("Are you sure you want to delete all configuration entries? This action cannot be undone. [y/N] ");
            stdout().flush()?;

            let mut response = String::new();
            stdin().read_line(&mut response)?;

            if !response.trim().eq_ignore_ascii_case("y") {
                println!("Operation cancelled.");
                return Ok(());
            }
        }

        let cosmic_path = get_cosmic_configurations()?;
        let mut deleted_count = 0;
        let mut errors = Vec::new();

        if !cosmic_path.exists() {
            return Err(Error::new(
                ErrorKind::NotFound,
                "No configurations to delete.",
            ));
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
                        errors.push(format!("Invalid pattern '{}': {}", pattern, e));
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
            if let Some((component, version, entry_name)) = parse_configuration_path(entry.path()) {
                let relative_path = format!("{}/v{}/{}", component, version, entry_name);
                let should_exclude = exclude_patterns
                    .iter()
                    .any(|pattern| pattern.matches(&relative_path));

                if should_exclude {
                    if self.verbose {
                        println!("Skipping excluded path: {}", relative_path);
                    }
                    continue;
                }

                if self.verbose {
                    println!("Deleting: {}", entry.path().display());
                }

                match delete_configuration(&component, &version, &entry_name) {
                    Ok(()) => deleted_count += 1,
                    Err(e) => errors.push(format!("{}: {}", entry.path().display(), e)),
                }
            }
        }

        if errors.is_empty() {
            println!(
                "Successfully deleted {} configuration entries.",
                deleted_count
            );
        } else {
            println!(
                "Deleted {} configuration entries with {} errors:",
                deleted_count,
                errors.len()
            );
            for error in errors {
                eprintln!("Error: {}", error);
            }
        }

        Ok(())
    }
}
