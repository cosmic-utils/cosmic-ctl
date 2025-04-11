use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    pub component: String,
    pub version: u64,
    pub operation: Operation,
    pub entries: EntryContent,
    pub xdg_directory: String,
}

#[derive(Deserialize, Serialize)]
pub struct ConfigFile {
    #[serde(rename = "$schema", skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    pub operations: Vec<Entry>,
}
