use atomicwrites::{AtomicFile, OverwriteBehavior};
use etcetera::{
    base_strategy::{BaseStrategy, Xdg},
    choose_base_strategy,
};
use std::{
    fs,
    io::{Error, ErrorKind, Write},
    path::{Path, PathBuf},
};
use unescaper::unescape;

fn get_base_strategy() -> Result<Xdg, Error> {
    choose_base_strategy().map_err(|e| {
        Error::new(
            ErrorKind::Other,
            format!("Failed to determine base strategy: {}", e),
        )
    })
}

pub fn read_configuration(
    component: &str,
    version: &u64,
    entry: &str,
    xdg_dir: &str,
) -> Result<String, Error> {
    let path = get_configuration_path(component, version, entry, xdg_dir)?;

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

pub fn read_configuration_file(file_path: &PathBuf) -> Result<String, Error> {
    if file_path.exists() {
        fs::read_to_string(file_path)
    } else {
        Err(Error::new(
            ErrorKind::NotFound,
            format!("Configuration file not found: {}", file_path.display()),
        ))
    }
}

pub fn write_configuration(
    component: &str,
    version: &u64,
    entry: &str,
    value: &str,
    xdg_dir: &str,
) -> Result<bool, Error> {
    let path = get_configuration_path(component, version, entry, xdg_dir)?;
    let unescaped_value = unescape(value).map_err(|e| {
        Error::new(
            ErrorKind::InvalidInput,
            format!("Failed to unescape value: {}", e),
        )
    })?;

    if let Ok(current_value) = read_configuration(component, version, entry, xdg_dir) {
        if current_value == unescaped_value {
            return Ok(false);
        }
    }

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| {
            Error::new(
                ErrorKind::Other,
                format!("Failed to create directory structure: {}", e),
            )
        })?;
    }

    let af = AtomicFile::new(&path, OverwriteBehavior::AllowOverwrite);
    af.write(|f| f.write_all(unescaped_value.as_bytes()))
        .map_err(|e| {
            Error::new(
                ErrorKind::Other,
                format!("Failed to write configuration to {}: {}", path.display(), e),
            )
        })?;

    Ok(true)
}

pub fn write_configuration_file(file_path: &PathBuf, value: &str) -> Result<bool, Error> {
    let unescaped_value = unescape(value).map_err(|e| {
        Error::new(
            ErrorKind::InvalidInput,
            format!("Failed to unescape value: {}", e),
        )
    })?;

    if let Ok(current_value) = fs::read_to_string(file_path) {
        if current_value == unescaped_value {
            return Ok(false);
        }
    }

    if let Some(parent) = file_path.parent() {
        fs::create_dir_all(parent).map_err(|e| {
            Error::new(
                ErrorKind::Other,
                format!("Failed to create directory structure: {}", e),
            )
        })?;
    }

    let af = AtomicFile::new(file_path, OverwriteBehavior::AllowOverwrite);
    af.write(|f| f.write_all(unescaped_value.as_bytes()))
        .map_err(|e| {
            Error::new(
                ErrorKind::Other,
                format!(
                    "Failed to write configuration to {}: {}",
                    file_path.display(),
                    e
                ),
            )
        })?;

    Ok(true)
}

pub fn delete_configuration(
    component: &str,
    version: &u64,
    entry: &str,
    xdg_dir: &str,
) -> Result<(), Error> {
    let path = get_configuration_path(component, version, entry, xdg_dir)?;
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

pub fn delete_configuration_file(file_path: &PathBuf) -> Result<(), Error> {
    if file_path.exists() {
        fs::remove_file(file_path)?;
        Ok(())
    } else {
        Err(Error::new(
            ErrorKind::NotFound,
            "Configuration file does not exist",
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

fn get_configuration_path(
    component: &str,
    version: &u64,
    entry: &str,
    xdg_dir: &str,
) -> Result<PathBuf, Error> {
    let cosmic_folder = get_cosmic_configurations(xdg_dir)?;

    Ok(cosmic_folder
        .join(component)
        .join(format!("v{}", version))
        .join(entry))
}

pub fn get_cosmic_configurations(xdg_dir: &str) -> Result<PathBuf, Error> {
    let config_dir = get_xdg_dir_path(xdg_dir)?.join("cosmic");
    Ok(config_dir)
}

pub fn get_xdg_dir_path(xdg_dir: &str) -> Result<PathBuf, Error> {
    match xdg_dir.to_lowercase().as_str() {
        "config" => Ok(get_base_strategy()?.config_dir()),
        "data" => Ok(get_base_strategy()?.data_dir()),
        "cache" => Ok(get_base_strategy()?.cache_dir()),
        "state" => get_base_strategy()?
            .state_dir()
            .ok_or_else(|| Error::new(ErrorKind::NotFound, "State directory is not available")),
        "runtime" => get_base_strategy()?
            .runtime_dir()
            .ok_or_else(|| Error::new(ErrorKind::NotFound, "Runtime directory is not available")),
        _ => Err(Error::new(
            ErrorKind::InvalidInput,
            format!("Invalid XDG directory: {}", xdg_dir),
        )),
    }
}
