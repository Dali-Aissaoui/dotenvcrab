use dotenvcrab::validation::{ValidationError, ValidationResult};

#[test]
fn test_validation_result_valid() {
    let result = ValidationResult {
        is_valid: true,
        errors: Vec::new(),
    };
    
    assert!(result.is_valid);
    assert!(result.errors.is_empty());
}

#[test]
fn test_validation_result_invalid() {
    let mut result = ValidationResult {
        is_valid: false,
        errors: Vec::new(),
    };
    
    result.errors.push(ValidationError::MissingRequired("API_KEY".to_string()));
    result.errors.push(ValidationError::InvalidType(
        "PORT".to_string(),
        "number".to_string(),
        "abc".to_string(),
    ));
    result.errors.push(ValidationError::InvalidEnum(
        "ENV".to_string(),
        vec!["dev".to_string(), "prod".to_string()],
        "test".to_string(),
    ));
    result.errors.push(ValidationError::ExtraField("EXTRA".to_string()));
    
    assert!(!result.is_valid);
    assert_eq!(result.errors.len(), 4);
    
    let missing_required = &result.errors[0];
    if let ValidationError::MissingRequired(key) = missing_required {
        assert_eq!(key, "API_KEY");
    } else {
        panic!("Expected MissingRequired error");
    }
    
    let invalid_type = &result.errors[1];
    if let ValidationError::InvalidType(key, expected, got) = invalid_type {
        assert_eq!(key, "PORT");
        assert_eq!(expected, "number");
        assert_eq!(got, "abc");
    } else {
        panic!("Expected InvalidType error");
    }
    
    let invalid_enum = &result.errors[2];
    if let ValidationError::InvalidEnum(key, values, got) = invalid_enum {
        assert_eq!(key, "ENV");
        assert_eq!(values, &vec!["dev".to_string(), "prod".to_string()]);
        assert_eq!(got, "test");
    } else {
        panic!("Expected InvalidEnum error");
    }
    
    let extra_field = &result.errors[3];
    if let ValidationError::ExtraField(key) = extra_field {
        assert_eq!(key, "EXTRA");
    } else {
        panic!("Expected ExtraField error");
    }
}

#[test]
fn test_validation_error_to_string() {
    let error = ValidationError::MissingRequired("API_KEY".to_string());
    assert_eq!(error.to_string(), "missing required field: API_KEY");
    
    let error = ValidationError::InvalidType(
        "PORT".to_string(),
        "number".to_string(),
        "abc".to_string(),
    );
    assert_eq!(error.to_string(), "invalid type for PORT: expected number, got abc");
    
    let error = ValidationError::InvalidEnum(
        "ENV".to_string(),
        vec!["dev".to_string(), "prod".to_string()],
        "test".to_string(),
    );
    assert_eq!(error.to_string(), "invalid enum value for ENV: expected one of [\"dev\", \"prod\"], got test");
    
    let error = ValidationError::ExtraField("EXTRA".to_string());
    assert_eq!(error.to_string(), "extra field not in schema: EXTRA");
}
