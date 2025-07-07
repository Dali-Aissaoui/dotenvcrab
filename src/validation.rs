use crate::schema::{Schema, SchemaField};
use std::collections::{HashMap, HashSet};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("missing required field: {0}")]
    MissingRequired(String),
    
    #[error("invalid type for {0}: expected {1}, got {2}")]
    InvalidType(String, String, String),
    
    #[error("invalid enum value for {0}: expected one of {1:?}, got {2}")]
    InvalidEnum(String, Vec<String>, String),
    
    #[error("extra field not in schema: {0}")]
    ExtraField(String),
}

#[derive(Debug)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub errors: Vec<ValidationError>,
}

impl ValidationResult {
    pub fn new() -> Self {
        Self {
            is_valid: true,
            errors: Vec::new(),
        }
    }
    
    pub fn add_error(&mut self, error: ValidationError) {
        self.is_valid = false;
        self.errors.push(error);
    }
}

pub fn validate_env(
    env_vars: &HashMap<String, String>,
    schema: &Schema,
    strict: bool,
) -> ValidationResult {
    let mut result = ValidationResult::new();
    let mut processed_keys = HashSet::new();
    
    for (key, field) in schema {
        processed_keys.insert(key.clone());
        
        if let Some(value) = env_vars.get(key) {
            match field {
                SchemaField::String { .. } => {
                }
                
                SchemaField::Number { .. } => {
                    if value.parse::<f64>().is_err() {
                        result.add_error(ValidationError::InvalidType(
                            key.clone(),
                            "number".to_string(),
                            value.clone(),
                        ));
                    }
                }
                
                SchemaField::Boolean { .. } => {
                    let lowercase = value.to_lowercase();
                    if !["true", "false", "1", "0", "yes", "no"].contains(&lowercase.as_str()) {
                        result.add_error(ValidationError::InvalidType(
                            key.clone(),
                            "boolean".to_string(),
                            value.clone(),
                        ));
                    }
                }
                
                SchemaField::Enum { values, .. } => {
                    if !values.contains(value) {
                        result.add_error(ValidationError::InvalidEnum(
                            key.clone(),
                            values.clone(),
                            value.clone(),
                        ));
                    }
                }
            }
        } else {
            if field.is_required() {
                if field.get_default().is_none() {
                    result.add_error(ValidationError::MissingRequired(key.clone()));
                }
            }
        }
    }
    
    if strict {
        for key in env_vars.keys() {
            if !processed_keys.contains(key) {
                result.add_error(ValidationError::ExtraField(key.clone()));
            }
        }
    }
    
    result
}


