use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Operation {
    Write,
    Read,
    Delete,
}

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
pub enum EntryContent {
    WriteEntries(HashMap<String, String>),
    ReadDeleteEntries(Vec<String>),
}

#[derive(Deserialize, Serialize)]
pub struct Entry {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<u64>,
    pub operation: Operation,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entries: Option<EntryContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xdg_directory: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<PathBuf>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct ConfigFile {
    #[serde(rename = "$schema", skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    pub operations: Vec<Entry>,
}
