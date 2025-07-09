use dotenvcrab::schema::{load_schema, SchemaField};
use std::path::Path;
mod test_helpers;
use test_helpers::{create_temp_file, load_schema_from_str};

#[test]
fn test_load_schema_success() {
    // Using test helper to create temp file
    let schema_json = r#"{
        "PORT": { "type": "number", "required": true },
        "DEBUG": { "type": "boolean", "required": true },
        "ENV": {
            "type": "enum",
            "required": true,
            "values": ["dev", "staging", "production"]
        }
    }"#;
    
    let schema = load_schema_from_str(schema_json).unwrap();
    
    assert_eq!(schema.len(), 3);
    
    match &schema["PORT"] {
        SchemaField::Number { required, .. } => {
            assert!(*required);
        }
        _ => panic!("Expected PORT to be a number field"),
    }
    
    match &schema["DEBUG"] {
        SchemaField::Boolean { required, .. } => {
            assert!(*required);
        }
        _ => panic!("Expected DEBUG to be a boolean field"),
    }
    
    match &schema["ENV"] {
        SchemaField::Enum {
            required,
            values,
            ..
        } => {
            assert!(*required);
            assert_eq!(values.len(), 3);
            assert!(values.contains(&"dev".to_string()));
            assert!(values.contains(&"staging".to_string()));
            assert!(values.contains(&"production".to_string()));
        }
        _ => panic!("Expected ENV to be an enum field"),
    }
}

#[test]
fn test_load_schema_file_not_found() {
    let result = load_schema(Path::new("/non/existent/path.json"));
    assert!(result.is_err());
}

#[test]
fn test_load_schema_invalid_json() {
    let temp_file = create_temp_file("this is not valid JSON");
    
    let result = load_schema(temp_file.path());
    assert!(result.is_err());
}

#[test]
fn test_schema_field_is_required() {
    let string_field = SchemaField::String {
        required: true,
        default: None,
        description: None,
    };
    assert!(string_field.is_required());
    
    let number_field = SchemaField::Number {
        required: false,
        default: None,
        description: None,
    };
    assert!(!number_field.is_required());
}

#[test]
fn test_schema_field_get_default() {
    let string_field = SchemaField::String {
        required: false,
        default: Some("default".to_string()),
        description: None,
    };
    assert_eq!(string_field.get_default(), Some("default".to_string()));
    
    let number_field = SchemaField::Number {
        required: false,
        default: Some(42.0),
        description: None,
    };
    assert_eq!(number_field.get_default(), Some("42".to_string()));
    
    let boolean_field = SchemaField::Boolean {
        required: false,
        default: Some(true),
        description: None,
    };
    assert_eq!(boolean_field.get_default(), Some("true".to_string()));
    
    let enum_field = SchemaField::Enum {
        required: false,
        values: vec!["a".to_string(), "b".to_string()],
        default: Some("a".to_string()),
        description: None,
    };
    assert_eq!(enum_field.get_default(), Some("a".to_string()));
    
    let no_default = SchemaField::String {
        required: true,
        default: None,
        description: None,
    };
    assert_eq!(no_default.get_default(), None);
}
