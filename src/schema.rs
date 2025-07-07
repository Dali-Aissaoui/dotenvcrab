use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SchemaError {
    #[error("failed to read schema file: {0}")]
    FileReadError(#[from] std::io::Error),
    
    #[error("failed to parse schema JSON: {0}")]
    ParseError(#[from] serde_json::Error),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum SchemaField {
    #[serde(rename = "string")]
    String {
        #[serde(default)]
        required: bool,
        #[serde(default)]
        default: Option<String>,
        #[serde(default)]
        description: Option<String>,
    },
    
    #[serde(rename = "number")]
    Number {
        #[serde(default)]
        required: bool,
        #[serde(default)]
        default: Option<f64>,
        #[serde(default)]
        description: Option<String>,
    },
    
    #[serde(rename = "boolean")]
    Boolean {
        #[serde(default)]
        required: bool,
        #[serde(default)]
        default: Option<bool>,
        #[serde(default)]
        description: Option<String>,
    },
    
    #[serde(rename = "enum")]
    Enum {
        #[serde(default)]
        required: bool,
        values: Vec<String>,
        #[serde(default)]
        default: Option<String>,
        #[serde(default)]
        description: Option<String>,
    },
}

impl SchemaField {
    pub fn is_required(&self) -> bool {
        match self {
            SchemaField::String { required, .. } => *required,
            SchemaField::Number { required, .. } => *required,
            SchemaField::Boolean { required, .. } => *required,
            SchemaField::Enum { required, .. } => *required,
        }
    }
    
    pub fn get_default(&self) -> Option<String> {
        match self {
            SchemaField::String { default, .. } => default.clone(),
            SchemaField::Number { default, .. } => default.map(|n| n.to_string()),
            SchemaField::Boolean { default, .. } => default.map(|b| b.to_string()),
            SchemaField::Enum { default, .. } => default.clone(),
        }
    }
}

pub type Schema = HashMap<String, SchemaField>;

pub fn load_schema<P: AsRef<Path>>(path: P) -> Result<Schema, SchemaError> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let schema = serde_json::from_reader(reader)?;
    Ok(schema)
}
