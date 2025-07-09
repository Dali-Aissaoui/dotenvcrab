use dotenvcrab::validation::ValidationError;
mod test_helpers;
use test_helpers::validate_env_with_schema;

#[test]
fn test_pattern_validation_valid() {
    let schema_str = r#"{
        "EMAIL": { "type": "string", "required": true, "pattern": "^[^@\\s]+@[^@\\s]+\\.[^@\\s]+$" }
    }"#;
    

    let env_str = "EMAIL=user@example.com";
    
    let result = validate_env_with_schema(env_str, schema_str, false);
    assert!(result.is_valid);
    assert!(result.errors.is_empty());
}

#[test]
fn test_pattern_validation_invalid() {
    let schema_str = r#"{
        "EMAIL": { "type": "string", "required": true, "pattern": "^[^@\\s]+@[^@\\s]+\\.[^@\\s]+$" }
    }"#;
    
    let env_str = "EMAIL=not-an-email";
    
    let result = validate_env_with_schema(env_str, schema_str, false);
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
fn test_invalid_regex_pattern() {
    let schema_str = r#"{
        "EMAIL": { "type": "string", "required": true, "pattern": "[a-z++" }
    }"#;
    
    let env_str = "EMAIL=test@example.com";
    
    let result = validate_env_with_schema(env_str, schema_str, false);
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
fn test_pattern_validation_multiple_fields() {
    let schema_str = r#"{
        "EMAIL": { "type": "string", "required": true, "pattern": "^[^@\\s]+@[^@\\s]+\\.[^@\\s]+$" },
        "PHONE": { "type": "string", "required": true, "pattern": "^\\d{3}-\\d{3}-\\d{4}$" }
    }"#;

    let env_str = "EMAIL=user@example.com\nPHONE=123-456";
    
    let result = validate_env_with_schema(env_str, schema_str, false);
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
fn test_pattern_validation_with_other_errors() {
    let schema_str = r#"{
        "EMAIL": { "type": "string", "required": true, "pattern": "^[^@\\s]+@[^@\\s]+\\.[^@\\s]+$" },
        "PORT": { "type": "number", "required": true }
    }"#;
    
    let env_str = "EMAIL=not-an-email\nPORT=not-a-number";
    
    let result = validate_env_with_schema(env_str, schema_str, false);
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
