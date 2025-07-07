use dotenvcrab::schema::SchemaField;
use dotenvcrab::validation::{validate_env, ValidationError, ValidationResult};
use std::collections::HashMap;

#[test]
fn test_validation_result_new() {
    let result = ValidationResult::new();
    assert!(result.is_valid);
    assert!(result.errors.is_empty());
}

#[test]
fn test_validation_result_add_error() {
    let mut result = ValidationResult::new();
    assert!(result.is_valid);
    
    result.add_error(ValidationError::MissingRequired("TEST".to_string()));
    
    assert!(!result.is_valid);
    assert_eq!(result.errors.len(), 1);
}

#[test]
fn test_validate_env_all_valid() {
    let mut schema = HashMap::new();
    schema.insert(
        "PORT".to_string(),
        SchemaField::Number {
            required: true,
            default: None,
            description: None,
        },
    );
    schema.insert(
        "DEBUG".to_string(),
        SchemaField::Boolean {
            required: true,
            default: None,
            description: None,
        },
    );
    
    let mut env_vars = HashMap::new();
    env_vars.insert("PORT".to_string(), "8080".to_string());
    env_vars.insert("DEBUG".to_string(), "true".to_string());
    
    let result = validate_env(&env_vars, &schema, false);
    
    assert!(result.is_valid);
    assert!(result.errors.is_empty());
}

#[test]
fn test_validate_env_missing_required() {
    let mut schema = HashMap::new();
    schema.insert(
        "PORT".to_string(),
        SchemaField::Number {
            required: true,
            default: None,
            description: None,
        },
    );
    schema.insert(
        "DEBUG".to_string(),
        SchemaField::Boolean {
            required: true,
            default: None,
            description: None,
        },
    );
    
    let mut env_vars = HashMap::new();
    env_vars.insert("PORT".to_string(), "8080".to_string());
    // DEBUG is missing
    
    let result = validate_env(&env_vars, &schema, false);
    
    assert!(!result.is_valid);
    assert_eq!(result.errors.len(), 1);
    
    match &result.errors[0] {
        ValidationError::MissingRequired(key) => {
            assert_eq!(key, "DEBUG");
        }
        _ => panic!("Expected MissingRequired error"),
    }
}

#[test]
fn test_validate_env_with_default_value() {
    let mut schema = HashMap::new();
    schema.insert(
        "PORT".to_string(),
        SchemaField::Number {
            required: true,
            default: None,
            description: None,
        },
    );
    schema.insert(
        "LOG_LEVEL".to_string(),
        SchemaField::Enum {
            required: true,
            values: vec!["debug".to_string(), "info".to_string()],
            default: Some("info".to_string()),
            description: None,
        },
    );
    
    let mut env_vars = HashMap::new();
    env_vars.insert("PORT".to_string(), "8080".to_string());
    // LOG_LEVEL is missing but has a default
    
    let result = validate_env(&env_vars, &schema, false);
    
    assert!(result.is_valid);
    assert!(result.errors.is_empty());
}

#[test]
fn test_validate_env_invalid_number() {
    let mut schema = HashMap::new();
    schema.insert(
        "PORT".to_string(),
        SchemaField::Number {
            required: true,
            default: None,
            description: None,
        },
    );
    
    let mut env_vars = HashMap::new();
    env_vars.insert("PORT".to_string(), "not-a-number".to_string());
    
    let result = validate_env(&env_vars, &schema, false);
    
    assert!(!result.is_valid);
    assert_eq!(result.errors.len(), 1);
    
    match &result.errors[0] {
        ValidationError::InvalidType(key, expected, got) => {
            assert_eq!(key, "PORT");
            assert_eq!(expected, "number");
            assert_eq!(got, "not-a-number");
        }
        _ => panic!("Expected InvalidType error"),
    }
}

#[test]
fn test_validate_env_invalid_boolean() {
    let mut schema = HashMap::new();
    schema.insert(
        "DEBUG".to_string(),
        SchemaField::Boolean {
            required: true,
            default: None,
            description: None,
        },
    );
    
    let mut env_vars = HashMap::new();
    env_vars.insert("DEBUG".to_string(), "not-a-boolean".to_string());
    
    let result = validate_env(&env_vars, &schema, false);
    
    assert!(!result.is_valid);
    assert_eq!(result.errors.len(), 1);
    
    match &result.errors[0] {
        ValidationError::InvalidType(key, expected, got) => {
            assert_eq!(key, "DEBUG");
            assert_eq!(expected, "boolean");
            assert_eq!(got, "not-a-boolean");
        }
        _ => panic!("Expected InvalidType error"),
    }
}

#[test]
fn test_validate_env_valid_boolean_variations() {
    let mut schema = HashMap::new();
    schema.insert(
        "DEBUG1".to_string(),
        SchemaField::Boolean {
            required: true,
            default: None,
            description: None,
        },
    );
    schema.insert(
        "DEBUG2".to_string(),
        SchemaField::Boolean {
            required: true,
            default: None,
            description: None,
        },
    );
    schema.insert(
        "DEBUG3".to_string(),
        SchemaField::Boolean {
            required: true,
            default: None,
            description: None,
        },
    );
    schema.insert(
        "DEBUG4".to_string(),
        SchemaField::Boolean {
            required: true,
            default: None,
            description: None,
        },
    );
    schema.insert(
        "DEBUG5".to_string(),
        SchemaField::Boolean {
            required: true,
            default: None,
            description: None,
        },
    );
    schema.insert(
        "DEBUG6".to_string(),
        SchemaField::Boolean {
            required: true,
            default: None,
            description: None,
        },
    );
    
    let mut env_vars = HashMap::new();
    env_vars.insert("DEBUG1".to_string(), "true".to_string());
    env_vars.insert("DEBUG2".to_string(), "false".to_string());
    env_vars.insert("DEBUG3".to_string(), "1".to_string());
    env_vars.insert("DEBUG4".to_string(), "0".to_string());
    env_vars.insert("DEBUG5".to_string(), "yes".to_string());
    env_vars.insert("DEBUG6".to_string(), "no".to_string());
    
    let result = validate_env(&env_vars, &schema, false);
    
    assert!(result.is_valid);
    assert!(result.errors.is_empty());
}

#[test]
fn test_validate_env_invalid_enum() {
    let mut schema = HashMap::new();
    schema.insert(
        "ENV".to_string(),
        SchemaField::Enum {
            required: true,
            values: vec!["dev".to_string(), "prod".to_string()],
            default: None,
            description: None,
        },
    );
    
    let mut env_vars = HashMap::new();
    env_vars.insert("ENV".to_string(), "staging".to_string());
    
    let result = validate_env(&env_vars, &schema, false);
    
    assert!(!result.is_valid);
    assert_eq!(result.errors.len(), 1);
    
    match &result.errors[0] {
        ValidationError::InvalidEnum(key, values, got) => {
            assert_eq!(key, "ENV");
            assert_eq!(values, &vec!["dev".to_string(), "prod".to_string()]);
            assert_eq!(got, "staging");
        }
        _ => panic!("Expected InvalidEnum error"),
    }
}

#[test]
fn test_validate_env_strict_mode() {
    let mut schema = HashMap::new();
    schema.insert(
        "PORT".to_string(),
        SchemaField::Number {
            required: true,
            default: None,
            description: None,
        },
    );
    
    let mut env_vars = HashMap::new();
    env_vars.insert("PORT".to_string(), "8080".to_string());
    env_vars.insert("EXTRA_VAR".to_string(), "value".to_string());
    
    // without strict mode
    let result = validate_env(&env_vars, &schema, false);
    assert!(result.is_valid);
    assert!(result.errors.is_empty());
    
    // with strict mode
    let result = validate_env(&env_vars, &schema, true);
    assert!(!result.is_valid);
    assert_eq!(result.errors.len(), 1);
    
    match &result.errors[0] {
        ValidationError::ExtraField(key) => {
            assert_eq!(key, "EXTRA_VAR");
        }
        _ => panic!("Expected ExtraField error"),
    }
}

#[test]
fn test_validate_env_multiple_errors() {
    let mut schema = HashMap::new();
    schema.insert(
        "PORT".to_string(),
        SchemaField::Number {
            required: true,
            default: None,
            description: None,
        },
    );
    schema.insert(
        "ENV".to_string(),
        SchemaField::Enum {
            required: true,
            values: vec!["dev".to_string(), "prod".to_string()],
            default: None,
            description: None,
        },
    );
    schema.insert(
        "DEBUG".to_string(),
        SchemaField::Boolean {
            required: true,
            default: None,
            description: None,
        },
    );
    
    let mut env_vars = HashMap::new();
    env_vars.insert("PORT".to_string(), "not-a-number".to_string());
    env_vars.insert("ENV".to_string(), "staging".to_string());
    // DEBUG is missing
    env_vars.insert("EXTRA_VAR".to_string(), "value".to_string());
    
    let result = validate_env(&env_vars, &schema, true);
    
    assert!(!result.is_valid);
    assert_eq!(result.errors.len(), 4);
    
    // check that we have all expected error types
    let mut has_invalid_type = false;
    let mut has_invalid_enum = false;
    let mut has_missing_required = false;
    let mut has_extra_field = false;
    
    for error in &result.errors {
        match error {
            ValidationError::InvalidType(key, ..) if key == "PORT" => {
                has_invalid_type = true;
            }
            ValidationError::InvalidEnum(key, ..) if key == "ENV" => {
                has_invalid_enum = true;
            }
            ValidationError::MissingRequired(key) if key == "DEBUG" => {
                has_missing_required = true;
            }
            ValidationError::ExtraField(key) if key == "EXTRA_VAR" => {
                has_extra_field = true;
            }
            _ => {}
        }
    }
    
    assert!(has_invalid_type);
    assert!(has_invalid_enum);
    assert!(has_missing_required);
    assert!(has_extra_field);
}
