use serde::{de::DeserializeOwned, Serialize};
use std::{
    io::{Error, ErrorKind},
    path::Path,
};

/// Supported file formats for configuration files
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileFormat {
    Json,
    Yaml,
    Toml,
    Ron,
}

impl FileFormat {
    /// Detect file format from extension
    pub fn from_path(path: &Path) -> Result<Self, Error> {
        match path.extension().and_then(|s| s.to_str()) {
            Some(ext) => match ext.to_lowercase().as_str() {
                "json" => Ok(FileFormat::Json),
                "yaml" | "yml" => Ok(FileFormat::Yaml),
                "toml" => Ok(FileFormat::Toml),
                "ron" => Ok(FileFormat::Ron),
                _ => Err(Error::new(
                    ErrorKind::InvalidInput,
                    format!("Unsupported file format: {}", ext),
                )),
            },
            None => Err(Error::new(ErrorKind::InvalidInput, "File has no extension")),
        }
    }

    /// File extension for this format
    pub fn extension(&self) -> &'static str {
        match self {
            FileFormat::Json => "json",
            FileFormat::Yaml => "yaml",
            FileFormat::Toml => "toml",
            FileFormat::Ron => "ron",
        }
    }

    /// Human-readable name of the format
    pub fn name(&self) -> &'static str {
        match self {
            FileFormat::Json => "JSON",
            FileFormat::Yaml => "YAML",
            FileFormat::Toml => "TOML",
            FileFormat::Ron => "RON",
        }
    }

    /// Deserialize from string data in this format
    pub fn deserialize<T: DeserializeOwned>(&self, data: &str) -> Result<T, Error> {
        match self {
            FileFormat::Json => serde_json::from_str(data).map_err(|e| {
                Error::new(ErrorKind::InvalidData, format!("JSON parsing error: {}", e))
            }),
            FileFormat::Yaml => serde_yaml::from_str(data).map_err(|e| {
                Error::new(ErrorKind::InvalidData, format!("YAML parsing error: {}", e))
            }),
            FileFormat::Toml => toml::from_str(data).map_err(|e| {
                Error::new(ErrorKind::InvalidData, format!("TOML parsing error: {}", e))
            }),
            FileFormat::Ron => ron::from_str(data).map_err(|e| {
                Error::new(ErrorKind::InvalidData, format!("RON parsing error: {}", e))
            }),
        }
    }

    /// Serialize to a string in this format
    pub fn serialize<T: Serialize>(&self, value: &T) -> Result<String, Error> {
        match self {
            FileFormat::Json => serde_json::to_string_pretty(value).map_err(|e| {
                Error::new(
                    ErrorKind::InvalidData,
                    format!("JSON serialization error: {}", e),
                )
            }),
            FileFormat::Yaml => serde_yaml::to_string(value).map_err(|e| {
                Error::new(
                    ErrorKind::InvalidData,
                    format!("YAML serialization error: {}", e),
                )
            }),
            FileFormat::Toml => toml::to_string_pretty(value).map_err(|e| {
                Error::new(
                    ErrorKind::InvalidData,
                    format!("TOML serialization error: {}", e),
                )
            }),
            FileFormat::Ron => {
                let config = ron::ser::PrettyConfig::new().separate_tuple_members(true);
                ron::ser::to_string_pretty(value, config).map_err(|e| {
                    Error::new(
                        ErrorKind::InvalidData,
                        format!("RON serialization error: {}", e),
                    )
                })
            }
        }
    }
}
