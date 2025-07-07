mod cli;
mod output;
mod schema;
mod validation;

use clap::Parser;
use std::collections::HashMap;
use std::process;

fn main() {

    let args = cli::Cli::parse();
    
    let schema = match schema::load_schema(&args.schema) {
        Ok(schema) => schema,
        Err(err) => {
            eprintln!("Failed to load schema: {}", err);
            process::exit(1);
        }
    };
    
    let env_vars = match dotenvy::from_path_iter(&args.env) {
        Ok(vars) => {

            let mut env_map = HashMap::new();
            for var_result in vars {
                match var_result {
                    Ok((key, value)) => {
                        env_map.insert(key, value);
                    },
                    Err(err) => {
                        eprintln!("error parsing .env entry: {}", err);
                        process::exit(1);
                    }
                }
            }
            env_map
        }
        Err(err) => {
            eprintln!("failed to load .env file: {}", err);
            process::exit(1);
        }
    };
    
    let validation_result = validation::validate_env(&env_vars, &schema, args.strict);
    
    output::print_result(&validation_result, args.json);
    
    if !validation_result.is_valid {
        process::exit(1);
    }
}
