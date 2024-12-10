use std::{
    env, fs,
    io::{Error, ErrorKind},
    path::{Path, PathBuf},
};
use unescaper::unescape;

pub fn read_configuration(component: &str, version: &u64, entry: &str) -> Result<String, Error> {
    let path = get_configuration_path(component, version, entry);

    if path.exists() {
        fs::read_to_string(path)
    } else {
        Err(Error::new(
            ErrorKind::NotFound,
            format!(
                "Configuration entry not found: {}/v{}/{}",
                component, version, entry
            ),
        ))
    }
}

pub fn write_configuration(
    component: &str,
    version: &u64,
    entry: &str,
    value: &str,
) -> Result<bool, Error> {
    let path = get_configuration_path(component, version, entry);
    let unescaped_value = unescape(value).map_err(|e| {
        Error::new(
            ErrorKind::InvalidInput,
            format!("Failed to unescape value: {}", e),
        )
    })?;

    if let Ok(current_value) = read_configuration(component, version, entry) {
        if current_value == unescaped_value {
            return Ok(false);
        }
    }

    fs::create_dir_all(path.parent().unwrap_or_else(|| Path::new(""))).map_err(|e| {
        Error::new(
            ErrorKind::Other,
            format!("Failed to create directory structure: {}", e),
        )
    })?;
    fs::write(&path, unescaped_value).map_err(|e| {
        Error::new(
            ErrorKind::Other,
            format!("Failed to write configuration to {}: {}", path.display(), e),
        )
    })?;

    Ok(true)
}

pub fn delete_configuration(component: &str, version: &u64, entry: &str) -> Result<(), Error> {
    let path = get_configuration_path(component, version, entry);
    if path.exists() {
        fs::remove_file(path)?;
        Ok(())
    } else {
        Err(Error::new(
            ErrorKind::NotFound,
            "Configuration entry does not exist",
        ))
    }
}

pub fn parse_configuration_path(path: &Path) -> Option<(String, u64, String)> {
    let parts: Vec<_> = path.iter().collect();

    if parts.len() < 4 {
        return None;
    }

    let entry_name = parts.last()?.to_str()?.to_string();
    let version_str = parts.get(parts.len() - 2)?.to_str()?;
    let version = version_str.strip_prefix('v')?.parse().ok()?;
    let component = parts.get(parts.len() - 3)?.to_str()?.to_string();

    Some((component, version, entry_name))
}

fn get_configuration_path(component: &str, version: &u64, entry: &str) -> PathBuf {
    let cosmic_folder = get_cosmic_configurations();

    Path::new(&cosmic_folder)
        .join(component)
        .join(format!("v{}", version))
        .join(entry)
}

pub fn get_cosmic_configurations() -> PathBuf {
    let config_home = env::var("XDG_CONFIG_HOME").unwrap_or_else(|_| {
        let home = env::var("HOME").unwrap();
        format!("{}/.config", home)
    });

    Path::new(&config_home).join("cosmic")
}
