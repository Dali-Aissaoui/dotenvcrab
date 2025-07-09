use dotenvcrab::validation::{validate_env, ValidationResult};
use dotenvcrab::schema::{Schema, SchemaField};
use std::collections::HashMap;
use std::io::Write;
use std::path::{Path, PathBuf};
use tempfile::NamedTempFile;
use serde_json;

/// parse environment variables from a string in the format KEY=VALUE
/// 
/// # arguments
/// * `env_str` - a string containing environment variables in KEY=VALUE format
/// 
/// # returns
/// * `Result<HashMap<String, String>, String>` - a map of environment variables or an error
pub fn parse_env_str(env_str: &str) -> Result<HashMap<String, String>, String> {
    let mut env_map = HashMap::new();
    
    for line in env_str.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        
        if let Some(pos) = line.find('=') {
            let key = line[..pos].trim().to_string();
            let value = line[pos+1..].trim().to_string();
            env_map.insert(key, value);
        } else {
            return Err(format!("invalid env line: {}", line));
        }
    }
    
    Ok(env_map)
}

/// load a schema from a JSON string
/// 
/// # arguments
/// * `schema_str` - a JSON string representing a schema
/// 
/// # returns
/// * `Result<Schema, serde_json::Error>` - a schema or a JSON parsing error
pub fn load_schema_from_str(schema_str: &str) -> Result<Schema, serde_json::Error> {
    serde_json::from_str(schema_str)
}

/// validate environment variables against a schema
/// 
/// # arguments
/// * `env` - a string containing environment variables in KEY=VALUE format
/// * `schema` - a JSON string representing a schema
/// * `strict` - whether to use strict mode validation
/// 
/// # Returns
/// * `ValidationResult` - The result of the validation
pub fn validate_env_with_schema(env: &str, schema: &str, strict: bool) -> ValidationResult {
    let env_map = parse_env_str(env).unwrap();
    let schema_map = load_schema_from_str(schema).unwrap();
    validate_env(&env_map, &schema_map, strict)
}

/// create a temporary file with the given content
/// 
/// # arguments
/// * `content` - the content to write to the file
/// 
/// # returns
/// * `NamedTempFile` - A temporary file with the content written to it
pub fn create_temp_file(content: &str) -> NamedTempFile {
    let mut file = NamedTempFile::new().unwrap();
    file.write_all(content.as_bytes()).unwrap();
    file
}

/// create a schema file with the given fields
/// 
/// # arguments
/// * `fields` - a map of field names to SchemaField definitions
/// 
/// # returns
/// * `(NamedTempFile, PathBuf)` - the temporary file and its path
pub fn create_schema_file(fields: &HashMap<String, SchemaField>) -> (NamedTempFile, PathBuf) {
    let schema_json = serde_json::to_string_pretty(fields).unwrap();
    let file = create_temp_file(&schema_json);
    let path = file.path().to_path_buf();
    (file, path)
}

/// create an environment file with the given variables
/// 
/// # arguments
/// * `vars` - a map of environment variable names to values
/// 
/// # returns
/// * `(NamedTempFile, PathBuf)` - The temporary file and its path
pub fn create_env_file(vars: &HashMap<String, String>) -> (NamedTempFile, PathBuf) {
    let mut content = String::new();
    for (key, value) in vars {
        content.push_str(&format!("{}={}\n", key, value));
    }
    let file = create_temp_file(&content);
    let path = file.path().to_path_buf();
    (file, path)
}

/// run the dotenvcrab CLI with the given arguments
/// 
/// # arguments
/// * `args` - the arguments to pass to the CLI
/// 
/// # returns
/// * `std::process::Output` - the output of the command
pub fn run_dotenvcrab(args: &[&str]) -> std::process::Output {
    std::process::Command::new("cargo")
        .arg("run")
        .arg("--")
        .args(args)
        .current_dir(Path::new(env!("CARGO_MANIFEST_DIR")))
        .output()
        .expect("Failed to execute command")
}
