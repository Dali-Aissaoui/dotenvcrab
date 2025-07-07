use clap::{Parser, ValueHint};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "dotenvcrab",
    author = "dotenvcrab",
    version,
    about = "A Rust-powered CLI tool that validates .env files against JSON schema definitions",
    long_about = "dotenvcrab is a blazing fast, portable, and safe CLI tool that validates .env files against JSON schema definitions."
)]
pub struct Cli {
    #[arg(short, long, default_value = ".env", value_hint = ValueHint::FilePath)]
    pub env: PathBuf,

    #[arg(short, long, default_value = "env.schema.json", value_hint = ValueHint::FilePath)]
    pub schema: PathBuf,

    #[arg(short = 'x', long)]
    pub strict: bool,

    #[arg(short, long)]
    pub json: bool,

    #[arg(short, long)]
    pub generate: bool,
}
