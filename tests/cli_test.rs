use dotenvcrab::cli::Cli;
use clap::Parser;
use std::path::PathBuf;

#[test]
fn test_cli_default_values() {
    let cli = Cli::parse_from(["dotenvcrab"]);
    
    assert_eq!(cli.env, PathBuf::from(".env"));
    assert_eq!(cli.schema, PathBuf::from("env.schema.json"));
    assert!(!cli.strict);
    assert!(!cli.json);
    assert!(!cli.generate);
}

#[test]
fn test_cli_custom_values() {
    let cli = Cli::parse_from([
        "dotenvcrab",
        "--env", "custom.env",
        "--schema", "custom.schema.json",
        "--strict",
        "--json",
        "--generate",
    ]);
    
    assert_eq!(cli.env, PathBuf::from("custom.env"));
    assert_eq!(cli.schema, PathBuf::from("custom.schema.json"));
    assert!(cli.strict);
    assert!(cli.json);
    assert!(cli.generate);
}

#[test]
fn test_cli_short_options() {
    let cli = Cli::parse_from([
        "dotenvcrab",
        "-e", "custom.env",
        "-s", "custom.schema.json",
        "-x",
        "-j",
        "-g",
    ]);
    
    assert_eq!(cli.env, PathBuf::from("custom.env"));
    assert_eq!(cli.schema, PathBuf::from("custom.schema.json"));
    assert!(cli.strict);
    assert!(cli.json);
    assert!(cli.generate);
}

#[test]
fn test_cli_mixed_options() {
    let cli = Cli::parse_from([
        "dotenvcrab",
        "-e", "custom.env",
        "--schema", "custom.schema.json",
        "-x",
        "--json",
    ]);
    
    assert_eq!(cli.env, PathBuf::from("custom.env"));
    assert_eq!(cli.schema, PathBuf::from("custom.schema.json"));
    assert!(cli.strict);
    assert!(cli.json);
    assert!(!cli.generate);
}
