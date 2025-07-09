use dotenvcrab::validation::{validate_env, ValidationError};
use dotenvcrab::schema::SchemaField;
use std::collections::HashMap;
mod test_helpers;

#[test]
fn test_pattern_validation_valid() {
    let mut schema = HashMap::new();
    schema.insert(
        "EMAIL".to_string(),
        SchemaField::String {
            required: true,
            default: None,
            description: None,
            pattern: Some(r"^[^@\s]+@[^@\s]+\.[^@\s]+$".to_string()),
        },
    );
    
    let mut env_vars = HashMap::new();
    env_vars.insert("EMAIL".to_string(), "user@example.com".to_string());
    
    let result = validate_env(&env_vars, &schema, false);
    assert!(result.is_valid);
    assert!(result.errors.is_empty());
}

#[test]
fn test_pattern_validation_invalid() {
    let mut schema = HashMap::new();
    schema.insert(
        "EMAIL".to_string(),
        SchemaField::String {
            required: true,
            default: None,
            description: None,
            pattern: Some(r"^[^@\s]+@[^@\s]+\.[^@\s]+$".to_string()),
        },
    );
    
    let mut env_vars = HashMap::new();
    env_vars.insert("EMAIL".to_string(), "not-an-email".to_string());
    
    let result = validate_env(&env_vars, &schema, false);
    assert!(!result.is_valid);
    assert_eq!(result.errors.len(), 1);
    
    match &result.errors[0] {
        ValidationError::InvalidPattern(key, pattern) => {
            assert_eq!(key, "EMAIL");
            assert_eq!(pattern, r"^[^@\s]+@[^@\s]+\.[^@\s]+$");
        }
        _ => panic!("Expected InvalidPattern error"),
    }
}

#[test]
fn test_pattern_validation_multiple_fields() {
    let mut schema = HashMap::new();
    schema.insert(
        "EMAIL".to_string(),
        SchemaField::String {
            required: true,
            default: None,
            description: None,
            pattern: Some(r"^[^@\s]+@[^@\s]+\.[^@\s]+$".to_string()),
        },
    );
    schema.insert(
        "PHONE".to_string(),
        SchemaField::String {
            required: true,
            default: None,
            description: None,
            pattern: Some(r"^\d{3}-\d{3}-\d{4}$".to_string()),
        },
    );
    
    let mut env_vars = HashMap::new();
    env_vars.insert("EMAIL".to_string(), "user@example.com".to_string());
    env_vars.insert("PHONE".to_string(), "123-456".to_string());
    
    let result = validate_env(&env_vars, &schema, false);
    assert!(!result.is_valid);
    assert_eq!(result.errors.len(), 1);
    
    match &result.errors[0] {
        ValidationError::InvalidPattern(key, _) => {
            assert_eq!(key, "PHONE");
        }
        _ => panic!("Expected InvalidPattern error"),
    }
}

#[test]
fn test_invalid_regex_pattern() {
    let mut schema = HashMap::new();
    schema.insert(
        "EMAIL".to_string(),
        SchemaField::String {
            required: true,
            default: None,
            description: None,
            pattern: Some(r"[a-z++".to_string()),
        },
    );
    
    let mut env_vars = HashMap::new();
    env_vars.insert("EMAIL".to_string(), "test@example.com".to_string());
    
    let result = validate_env(&env_vars, &schema, false);
    assert!(!result.is_valid);
    assert_eq!(result.errors.len(), 1);
    
    match &result.errors[0] {
        ValidationError::InvalidRegexPattern(key, error) => {
            assert_eq!(key, "EMAIL");
            assert!(error.contains("syntax") || error.contains("parse"), 
                   "Error message should indicate a regex syntax problem: {}", error);
        }
        _ => panic!("Expected InvalidRegexPattern error, got {:?}", &result.errors[0]),
    }
}

#[test]
fn test_pattern_validation_with_other_errors() {
    let mut schema = HashMap::new();
    schema.insert(
        "EMAIL".to_string(),
        SchemaField::String {
            required: true,
            default: None,
            description: None,
            pattern: Some(r"^[^@\s]+@[^@\s]+\.[^@\s]+$".to_string()),
        },
    );
    schema.insert(
        "PORT".to_string(),
        SchemaField::Number {
            required: true,
            default: None,
            description: None,
        },
    );
    
    let mut env_vars = HashMap::new();
    env_vars.insert("EMAIL".to_string(), "not-an-email".to_string());
    env_vars.insert("PORT".to_string(), "not-a-number".to_string());
    
    let result = validate_env(&env_vars, &schema, false);
    assert!(!result.is_valid);
    assert_eq!(result.errors.len(), 2);
    
    let mut has_pattern_error = false;
    let mut has_type_error = false;
    
    for error in &result.errors {
        match error {
            ValidationError::InvalidPattern(key, _) => {
                assert_eq!(key, "EMAIL");
                has_pattern_error = true;
            }
            ValidationError::InvalidType(key, expected, _) => {
                assert_eq!(key, "PORT");
                assert_eq!(expected, "number");
                has_type_error = true;
            }
            _ => {}
        }
    }
    
    assert!(has_pattern_error);
    assert!(has_type_error);
}
