use crate::validation::{ValidationError, ValidationResult};
use colored::*;
use serde::Serialize;

#[derive(Serialize)]
struct JsonOutput {
    valid: bool,
    errors: Vec<String>,
}

pub fn print_result(result: &ValidationResult, json_output: bool) {
    if json_output {
        print_json_result(result);
    } else {
        print_colored_result(result);
    }
}

pub fn print_colored_result(result: &ValidationResult) {
    if result.is_valid {
        println!("{}", "✅ All environment variables are valid!".green().bold());
    } else {
        println!("{}", "❌ Invalid .env:".red().bold());
        
        for error in &result.errors {
            match error {
                ValidationError::MissingRequired(key) => {
                    println!("- {}: {}", key.yellow(), "missing".red());
                }
                ValidationError::InvalidType(key, expected, got) => {
                    println!(
                        "- {}: expected {}, got {}",
                        key.yellow(),
                        expected.green(),
                        got.red()
                    );
                }
                ValidationError::InvalidEnum(key, values, got) => {
                    println!(
                        "- {}: expected one of {}, got {}",
                        key.yellow(),
                        format!("[{}]", values.join(", ")).green(),
                        got.red()
                    );
                }
                ValidationError::ExtraField(key) => {
                    println!("- {}: {}", key.yellow(), "not in schema".red());
                }
            }
        }
    }
}

pub fn print_json_result(result: &ValidationResult) {
    let json_output = JsonOutput {
        valid: result.is_valid,
        errors: result
            .errors
            .iter()
            .map(|e| e.to_string())
            .collect(),
    };
    
    println!("{}", serde_json::to_string_pretty(&json_output).unwrap());
}
