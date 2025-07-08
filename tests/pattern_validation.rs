use dotenvcrab::validation::{validate_env, ValidationError, ValidationResult};
use dotenvcrab::schema::Schema;
use std::collections::HashMap;
use serde_json;

fn parse_env_str(env_str: &str) -> Result<HashMap<String, String>, String> {
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

fn load_schema_from_str(schema_str: &str) -> Result<Schema, serde_json::Error> {
    serde_json::from_str(schema_str)
}

fn validate_env_with_schema(env: &str, schema: &str) -> ValidationResult {
    let env_map = parse_env_str(env).unwrap();
    let schema_map = load_schema_from_str(schema).unwrap();
    validate_env(&env_map, &schema_map, false)
}

#[test]
fn test_pattern_validation() {
    let schema = r#"{
        "EMAIL": {
            "type": "string",
            "pattern": "^[^@\\\\s]+@[^@\\\\s]+\\\\.[^@\\\\s]+$",
            "required": true
        }
    }"#;

    let valid_env = "EMAIL=admin@example.com";
    let result_valid = validate_env_with_schema(valid_env, schema);
    
    // Debug output
    println!("valid email test result: {:?}", result_valid);
    if !result_valid.is_valid {
        for error in &result_valid.errors {
            println!("Error: {:?}", error);
        }
    }
    
    assert!(result_valid.is_valid, "valid email should pass pattern check");
    let invalid_env = "EMAIL=not-an-email";
    let result_invalid = validate_env_with_schema(invalid_env, schema);
    assert!(!result_invalid.is_valid, "invalid email should fail pattern check");
    assert!(result_invalid.errors.iter().any(|e| match e {
        ValidationError::PatternMismatch(key, _) => key == "EMAIL",
        _ => false
    }), "should report pattern mismatch error for invalid email");
}
