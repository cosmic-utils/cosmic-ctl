use crate::commands::{
    apply::ApplyCommand, backup::BackupCommand, delete::DeleteCommand, read::ReadCommand,
    reset::ResetCommand, write::WriteCommand, Command,
};
use inquire::{MultiSelect, Select, Text};
use std::{
    io::{Error, ErrorKind},
    path::PathBuf,
};

const XDG_DIRECTORIES: [&str; 5] = ["cache", "config", "data", "runtime", "state"];

pub fn run_interactive_mode() -> Result<(), Error> {
    let operation = Select::new(
        "What would you like to do?",
        vec!["Write", "Read", "Delete", "Apply", "Backup", "Reset"],
    )
    .prompt()
    .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?;

    match operation {
        "Write" => interactive_write()?,
        "Read" => interactive_read()?,
        "Delete" => interactive_delete()?,
        "Apply" => interactive_apply()?,
        "Backup" => interactive_backup()?,
        "Reset" => interactive_reset()?,
        _ => unreachable!(),
    }

    Ok(())
}

fn interactive_write() -> Result<(), Error> {
    let file_or_component = Select::new(
        "Would you like to write to a file or a component?",
        vec!["Component", "File"],
    )
    .prompt()
    .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?;

    if file_or_component == "File" {
        let file = Text::new("File path:")
            .prompt()
            .map_err(|e| Error::new(ErrorKind::Other, format!("Input error: {}", e)))?;
        let value = Text::new("Value:")
            .prompt()
            .map_err(|e| Error::new(ErrorKind::Other, format!("Input error: {}", e)))?;

        let cmd = WriteCommand {
            version: 1,
            component: None,
            entry: None,
            value,
            xdg_dir: "config".to_string(),
            file: Some(PathBuf::from(file)),
        };

        cmd.execute()
    } else {
        let component = Text::new("Component:")
            .prompt()
            .map_err(|e| Error::new(ErrorKind::Other, format!("Input error: {}", e)))?;
        let entry = Text::new("Entry:")
            .prompt()
            .map_err(|e| Error::new(ErrorKind::Other, format!("Input error: {}", e)))?;
        let version = Text::new("Version:")
            .with_default("1")
            .prompt()
            .map_err(|e| Error::new(ErrorKind::Other, format!("Input error: {}", e)))?
            .parse::<u64>()
            .map_err(|e| {
                Error::new(
                    ErrorKind::InvalidInput,
                    format!("Invalid version number: {}", e),
                )
            })?;
        let xdg_dir = Select::new("XDG Directory:", XDG_DIRECTORIES.to_vec())
            .prompt()
            .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?
            .to_string();
        let value = Text::new("Value:")
            .prompt()
            .map_err(|e| Error::new(ErrorKind::Other, format!("Input error: {}", e)))?;

        let cmd = WriteCommand {
            version,
            component: Some(component),
            entry: Some(entry),
            value,
            xdg_dir,
            file: None,
        };

        cmd.execute()
    }
}

fn interactive_read() -> Result<(), Error> {
    let file_or_component = Select::new(
        "Would you like to read from a file or a component?",
        vec!["Component", "File"],
    )
    .prompt()
    .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?;

    if file_or_component == "File" {
        let file = Text::new("File path:")
            .prompt()
            .map_err(|e| Error::new(ErrorKind::Other, format!("Input error: {}", e)))?;

        let cmd = ReadCommand {
            version: 1,
            component: None,
            entry: None,
            xdg_dir: "config".to_string(),
            file: Some(PathBuf::from(file)),
        };

        cmd.execute()
    } else {
        let component = Text::new("Component:")
            .prompt()
            .map_err(|e| Error::new(ErrorKind::Other, format!("Input error: {}", e)))?;
        let entry = Text::new("Entry:")
            .prompt()
            .map_err(|e| Error::new(ErrorKind::Other, format!("Input error: {}", e)))?;
        let version = Text::new("Version:")
            .with_default("1")
            .prompt()
            .map_err(|e| Error::new(ErrorKind::Other, format!("Input error: {}", e)))?
            .parse::<u64>()
            .map_err(|e| {
                Error::new(
                    ErrorKind::InvalidInput,
                    format!("Invalid version number: {}", e),
                )
            })?;
        let xdg_dir = Select::new("XDG Directory:", XDG_DIRECTORIES.to_vec())
            .prompt()
            .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?
            .to_string();

        let cmd = ReadCommand {
            version,
            component: Some(component),
            entry: Some(entry),
            xdg_dir,
            file: None,
        };

        cmd.execute()
    }
}

fn interactive_delete() -> Result<(), Error> {
    let file_or_component = Select::new(
        "Would you like to delete a file or a component?",
        vec!["Component", "File"],
    )
    .prompt()
    .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?;

    if file_or_component == "File" {
        let file = Text::new("File path:")
            .prompt()
            .map_err(|e| Error::new(ErrorKind::Other, format!("Input error: {}", e)))?;

        let cmd = DeleteCommand {
            version: 1,
            component: None,
            entry: None,
            xdg_dir: "config".to_string(),
            file: Some(PathBuf::from(file)),
        };

        cmd.execute()
    } else {
        let component = Text::new("Component:")
            .prompt()
            .map_err(|e| Error::new(ErrorKind::Other, format!("Input error: {}", e)))?;
        let entry = Text::new("Entry:")
            .prompt()
            .map_err(|e| Error::new(ErrorKind::Other, format!("Input error: {}", e)))?;
        let version = Text::new("Version:")
            .with_default("1")
            .prompt()
            .map_err(|e| Error::new(ErrorKind::Other, format!("Input error: {}", e)))?
            .parse::<u64>()
            .map_err(|e| {
                Error::new(
                    ErrorKind::InvalidInput,
                    format!("Invalid version number: {}", e),
                )
            })?;
        let xdg_dir = Select::new("XDG Directory:", XDG_DIRECTORIES.to_vec())
            .prompt()
            .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?
            .to_string();

        let cmd = DeleteCommand {
            version,
            component: Some(component),
            entry: Some(entry),
            xdg_dir,
            file: None,
        };

        cmd.execute()
    }
}

fn interactive_apply() -> Result<(), Error> {
    let file = Text::new("Configuration file path:")
        .prompt()
        .map_err(|e| Error::new(ErrorKind::Other, format!("Input error: {}", e)))?;
    let verbose = Select::new("Verbose output?", vec!["Yes", "No"])
        .prompt()
        .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?
        == "Yes";

    let cmd = ApplyCommand {
        file: PathBuf::from(file),
        verbose,
    };

    cmd.execute()
}

fn interactive_backup() -> Result<(), Error> {
    let file = Text::new("Output file path:")
        .prompt()
        .map_err(|e| Error::new(ErrorKind::Other, format!("Input error: {}", e)))?;

    let verbose = Select::new("Verbose output?", vec!["Yes", "No"])
        .prompt()
        .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?
        == "Yes";

    let selected_dirs = MultiSelect::new(
        "Select XDG directories to backup:",
        XDG_DIRECTORIES.to_vec(),
    )
    .prompt()
    .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?;

    if selected_dirs.is_empty() {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "At least one XDG directory must be selected",
        ));
    }

    let xdg_dirs: Vec<String> = selected_dirs.into_iter().map(String::from).collect();

    let cmd = BackupCommand {
        file: PathBuf::from(file),
        verbose,
        xdg_dirs,
        format: None, // Will be auto-detected from file extension
    };

    cmd.execute()
}

fn interactive_reset() -> Result<(), Error> {
    let exclude = Text::new("Patterns to exclude (comma-separated, leave empty for none):")
        .prompt()
        .map_err(|e| Error::new(ErrorKind::Other, format!("Input error: {}", e)))?;

    let verbose = Select::new("Show verbose output?", vec!["Yes", "No"])
        .prompt()
        .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?
        == "Yes";

    let exclude_option = if exclude.trim().is_empty() {
        None
    } else {
        Some(exclude)
    };

    let selected_dirs =
        MultiSelect::new("Select XDG directories to reset:", XDG_DIRECTORIES.to_vec())
            .prompt()
            .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?;

    if selected_dirs.is_empty() {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "At least one XDG directory must be selected",
        ));
    }

    let xdg_dirs: Vec<String> = selected_dirs.into_iter().map(String::from).collect();

    // In interactive mode, we still want confirmation
    // but we'll handle it through the ResetCommand's own prompt
    let cmd = ResetCommand {
        force: false, // Let the reset command handle confirmation
        verbose,
        exclude: exclude_option,
        xdg_dirs,
    };

    cmd.execute()
}
