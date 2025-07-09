use std::fs::File;
use std::io::Write;
use tempfile::tempdir;
mod test_helpers;
use test_helpers::run_dotenvcrab;

#[test]
fn test_valid_env() {
    let dir = tempdir().unwrap();
    
    let schema_path = dir.path().join("schema.json");
    let mut schema_file = File::create(&schema_path).unwrap();
    writeln!(
        schema_file,
        r#"{{
            "PORT": {{ "type": "number", "required": true }},
            "DEBUG": {{ "type": "boolean", "required": true }},
            "ENV": {{
                "type": "enum",
                "required": true,
                "values": ["dev", "staging", "production"]
            }}
        }}"#
    )
    .unwrap();
    
    let env_path = dir.path().join(".env");
    let mut env_file = File::create(&env_path).unwrap();
    writeln!(env_file, "PORT=8080").unwrap();
    writeln!(env_file, "DEBUG=true").unwrap();
    writeln!(env_file, "ENV=dev").unwrap();
    
    let output = run_dotenvcrab(&[
        "--env", env_path.to_str().unwrap(),
        "--schema", schema_path.to_str().unwrap(),
    ]);
    
    assert!(output.status.success());
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("All environment variables are valid"));
}

#[test]
fn test_invalid_env() {
    let dir = tempdir().unwrap();
    
    let schema_path = dir.path().join("schema.json");
    let mut schema_file = File::create(&schema_path).unwrap();
    writeln!(
        schema_file,
        r#"{{
            "PORT": {{ "type": "number", "required": true }},
            "DEBUG": {{ "type": "boolean", "required": true }},
            "ENV": {{
                "type": "enum",
                "required": true,
                "values": ["dev", "staging", "production"]
            }}
        }}"#
    )
    .unwrap();
    
    let env_path = dir.path().join(".env");
    let mut env_file = File::create(&env_path).unwrap();
    writeln!(env_file, "PORT=not-a-number").unwrap();
    writeln!(env_file, "DEBUG=maybe").unwrap();
    writeln!(env_file, "ENV=test").unwrap();
    
    let output = run_dotenvcrab(&[
        "--env", env_path.to_str().unwrap(),
        "--schema", schema_path.to_str().unwrap(),
    ]);
    
    assert!(!output.status.success());
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Invalid .env"));
    assert!(stdout.contains("PORT"));
    assert!(stdout.contains("expected number"));
    assert!(stdout.contains("DEBUG"));
    assert!(stdout.contains("expected boolean"));
    assert!(stdout.contains("ENV"));
    assert!(stdout.contains("expected one of"));
}

#[test]
fn test_strict_mode() {
    let dir = tempdir().unwrap();
    
    let schema_path = dir.path().join("schema.json");
    let mut schema_file = File::create(&schema_path).unwrap();
    writeln!(
        schema_file,
        r#"{{
            "PORT": {{ "type": "number", "required": true }}
        }}"#
    )
    .unwrap();
    
    let env_path = dir.path().join(".env");
    let mut env_file = File::create(&env_path).unwrap();
    writeln!(env_file, "PORT=8080").unwrap();
    writeln!(env_file, "EXTRA=value").unwrap();
    
    let output = run_dotenvcrab(&[
        "--env", env_path.to_str().unwrap(),
        "--schema", schema_path.to_str().unwrap(),
        "--strict",
    ]);
    
    assert!(!output.status.success());
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    eprintln!("STRICT MODE OUTPUT: '{}'\n", stdout);
    
    assert!(stdout.contains("Invalid .env"));
    assert!(stdout.contains("EXTRA"));
    assert!(stdout.contains("not in schema"));
    
    let output = run_dotenvcrab(&[
        "--env", env_path.to_str().unwrap(),
        "--schema", schema_path.to_str().unwrap(),
    ]);
    
    assert!(output.status.success());
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("All environment variables are valid"));
}

#[test]
fn test_json_output() {
    let dir = tempdir().unwrap();
    
    let schema_path = dir.path().join("schema.json");
    let mut schema_file = File::create(&schema_path).unwrap();
    writeln!(
        schema_file,
        r#"{{
            "PORT": {{ "type": "number", "required": true }}
        }}"#
    )
    .unwrap();
    
    let env_path = dir.path().join(".env");
    let mut env_file = File::create(&env_path).unwrap();
    writeln!(env_file, "PORT=8080").unwrap();
    
    let output = run_dotenvcrab(&[
        "--env", env_path.to_str().unwrap(),
        "--schema", schema_path.to_str().unwrap(),
        "--json",
    ]);
    
    assert!(output.status.success());
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    
    assert_eq!(json["valid"], true);
    assert!(json["errors"].as_array().unwrap().is_empty());
}

#[test]
fn test_missing_env_file() {
    let dir = tempdir().unwrap();
    
    let schema_path = dir.path().join("schema.json");
    let mut schema_file = File::create(&schema_path).unwrap();
    writeln!(
        schema_file,
        r#"{{
            "PORT": {{ "type": "number", "required": true }}
        }}"#
    )
    .unwrap();
    
    let output = run_dotenvcrab(&[
        "--env", dir.path().join("non-existent.env").to_str().unwrap(),
        "--schema", schema_path.to_str().unwrap(),
    ]);
    
    assert!(!output.status.success());
    
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("failed to load .env file"));
}

#[test]
fn test_missing_schema_file() {
    let dir = tempdir().unwrap();
    
    let env_path = dir.path().join(".env");
    let mut env_file = File::create(&env_path).unwrap();
    writeln!(env_file, "PORT=8080").unwrap();
    
    let output = run_dotenvcrab(&[
        "--env", env_path.to_str().unwrap(),
        "--schema", dir.path().join("non-existent.schema.json").to_str().unwrap(),
    ]);
    
    assert!(!output.status.success());
    
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Failed to load schema"));
}
