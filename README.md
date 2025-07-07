# dotenvcrab Documentation

## Table of Contents

- [Introduction](#introduction)
  - [The Problem](#the-problem)
  - [The Solution](#the-solution)
- [Installation](#installation)
  - [Binary Installation](#binary-installation)
  - [From Source](#from-source)
  - [npm Package](#npm-package)
- [Usage](#usage)
  - [Basic Usage](#basic-usage)
  - [Command Line Options](#command-line-options)
  - [Exit Codes](#exit-codes)
- [Schema Format](#schema-format)
  - [Field Types](#field-types)
  - [Required Fields](#required-fields)
  - [Default Values](#default-values)
  - [Enum Values](#enum-values)
  - [Complete Example](#complete-example)
- [Integration](#integration)
  - [npm Scripts](#npm-scripts)
  - [Husky Pre-commit Hooks](#husky-pre-commit-hooks)
  - [CI/CD Integration](#cicd-integration)
- [Advanced Usage](#advanced-usage)
  - [Strict Mode](#strict-mode)
  - [JSON Output](#json-output)
  - [Multiple Environments](#multiple-environments)
- [Performance](#performance)
  - [Benchmarks](#benchmarks)
- [Troubleshooting](#troubleshooting)
- [Contributing](#contributing)
- [License](#license)

## Introduction

### The Problem

Environment variables are a critical part of modern application configuration, especially in cloud-native and containerized environments. However, they come with several challenges:

1. **Type Safety**: Environment variables are always strings, requiring manual parsing and validation
2. **Missing Variables**: Applications often fail at runtime due to missing required environment variables
3. **Invalid Values**: Typos or incorrect values can cause unexpected behavior
4. **Documentation**: It's difficult to know what environment variables an application needs
5. **Default Values**: Managing default values is often done inconsistently across codebases

These issues are particularly problematic in:

- Production deployments where failures are costly
- Microservice architectures with many configuration points
- Teams where configuration is managed by different people than those who write the code
- CI/CD pipelines where environment configuration errors should be caught early

### The Solution

dotenvcrab is a blazing fast, Rust-powered CLI tool that validates your `.env` files against a JSON schema definition. It provides:

- **Type Validation**: Ensures variables are the correct type (string, number, boolean, enum)
- **Required Field Checking**: Verifies all required variables are present
- **Default Values**: Supports default values for optional fields
- **Schema Documentation**: Self-documenting configuration requirements
- **Strict Mode**: Optionally fails when extra keys are present
- **Cross-Platform**: Works on Windows, macOS, and Linux

## Installation

### Binary Installation

#### macOS (Homebrew)

```bash
# Coming soon
brew install dotenvcrab
```

#### Linux

```bash
# Coming soon
curl -sSL https://github.com/Dali-Aissaoui/dotenvcrab/releases/latest/download/dotenvcrab-linux -o /usr/local/bin/dotenvcrab
chmod +x /usr/local/bin/dotenvcrab
```

#### Windows

```bash
# Coming soon
# Download from https://github.com/Dali-Aissaoui/dotenvcrab/releases
```

### From Source

Requirements:

- Rust 1.70.0 or later
- Cargo

```bash
# Clone the repository
git clone https://github.com/Dali-Aissaoui/dotenvcrab.git
cd dotenvcrab

# Build the project
cargo build --release

# The binary will be available at target/release/dotenvcrab
```

### npm Package

```bash
# Global installation
npm install -g dotenvcrab

# Project installation
npm install --save-dev dotenvcrab
```

## Usage

### Basic Usage

1. Create a schema file (`env.schema.json`):

```json
{
  "PORT": { "type": "number", "required": true },
  "DEBUG": { "type": "boolean", "required": true },
  "API_URL": { "type": "string", "required": true },
  "LOG_LEVEL": {
    "type": "enum",
    "values": ["debug", "info", "warn", "error"],
    "default": "info"
  }
}
```

2. Create your `.env` file:

```
PORT=3000
DEBUG=true
API_URL=https://api.example.com
LOG_LEVEL=debug
```

3. Run dotenvcrab:

```bash
dotenvcrab
```

If validation passes, you'll see:

```
✅ All environment variables are valid!
```

If validation fails, you'll see detailed errors:

```
- PORT: expected number, got string
- DEBUG: missing
- API_URL: missing
```

### Command Line Options

```
USAGE:
    dotenvcrab [OPTIONS]

OPTIONS:
    -e, --env <FILE>       Path to .env file [default: .env]
    -s, --schema <FILE>    Path to schema file [default: env.schema.json]
    -t, --strict           Fail on extra keys not in schema
    -j, --json             Output in JSON format
    -h, --help             Print help information
    -V, --version          Print version information
```

### Exit Codes

- `0`: Validation successful
- `1`: Validation failed (schema violations)
- `2`: File not found or permission error
- `3`: Schema parsing error
- `4`: Other errors

## Schema Format

The schema is defined in a JSON file with the following structure:

```json
{
  "VARIABLE_NAME": {
    "type": "string|number|boolean|enum",
    "required": true|false,
    "default": "default value",
    "values": ["value1", "value2"] // Only for enum type
  }
}
```

### Field Types

#### String

Accepts any string value.

```json
{
  "API_KEY": { "type": "string", "required": true }
}
```

#### Number

Accepts numeric values like `8080`, `3.14`, etc.

```json
{
  "PORT": { "type": "number", "required": true }
}
```

#### Boolean

Accepts various boolean representations:

- `true`: "true", "1", "yes", "y", "on"
- `false`: "false", "0", "no", "n", "off"

```json
{
  "DEBUG": { "type": "boolean", "required": true }
}
```

#### Enum

Accepts only values from a predefined list specified in the `values` array.

```json
{
  "NODE_ENV": {
    "type": "enum",
    "required": true,
    "values": ["development", "production", "test"]
  }
}
```

### Required Fields

Mark fields as required to ensure they are present in the `.env` file:

```json
{
  "API_KEY": { "type": "string", "required": true }
}
```

If a required field is missing, validation will fail.

### Default Values

Provide default values for optional fields:

```json
{
  "LOG_LEVEL": {
    "type": "enum",
    "values": ["debug", "info", "warn", "error"],
    "default": "info"
  }
}
```

If the field is missing in the `.env` file, the default value will be used.

### Enum Values

For enum fields, specify the allowed values:

```json
{
  "NODE_ENV": {
    "type": "enum",
    "required": true,
    "values": ["development", "production", "test"]
  }
}
```

### Complete Example

```json
{
  "PORT": {
    "type": "number",
    "required": true
  },
  "DEBUG": {
    "type": "boolean",
    "required": true
  },
  "API_URL": {
    "type": "string",
    "required": true
  },
  "NODE_ENV": {
    "type": "enum",
    "required": true,
    "values": ["development", "production", "test"]
  },
  "LOG_LEVEL": {
    "type": "enum",
    "values": ["debug", "info", "warn", "error"],
    "default": "info"
  },
  "CACHE_TTL": {
    "type": "number",
    "default": "3600"
  }
}
```

## Integration

### npm Scripts

Add dotenvcrab to your `package.json` scripts:

```json
{
  "scripts": {
    "validate-env": "dotenvcrab",
    "prestart": "dotenvcrab",
    "prebuild": "dotenvcrab --env .env.production --schema env.production.schema.json",
    "pretest": "dotenvcrab --env .env.test --schema env.test.schema.json"
  }
}
```

Run it manually:

```bash
npm run validate-env
```

Or let it run automatically before other scripts:

```bash
npm start  # Will run dotenvcrab first
```

### Husky Pre-commit Hooks

1. Install husky:

```bash
npm install --save-dev husky
npx husky install
```

2. Create a pre-commit hook:

```bash
npx husky add .husky/pre-commit "npm run validate-env"
```

This will validate your environment variables before each commit.

### CI/CD Integration

#### GitHub Actions

```yaml
name: Validate Environment

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  validate:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2

      - name: Setup dotenvcrab
        run: |
          curl -sSL https://github.com/Dali-Aissaoui/dotenvcrab/releases/latest/download/dotenvcrab-linux -o /usr/local/bin/dotenvcrab
          chmod +x /usr/local/bin/dotenvcrab

      - name: Validate .env
        run: dotenvcrab --json
```

#### GitLab CI

```yaml
validate_env:
  stage: test
  script:
    - curl -sSL https://github.com/Dali-Aissaoui/dotenvcrab/releases/latest/download/dotenvcrab-linux -o /usr/local/bin/dotenvcrab
    - chmod +x /usr/local/bin/dotenvcrab
    - dotenvcrab --json
```

## Advanced Usage

### Strict Mode

Strict mode fails validation if there are extra keys in the `.env` file that aren't defined in the schema:

```bash
dotenvcrab --strict
```

This is useful for catching typos and ensuring all environment variables are documented.

### JSON Output

For CI/CD pipelines or programmatic usage, you can get JSON output:

```bash
dotenvcrab --json
```

Example output:

```json
{
  "valid": false,
  "errors": [
    "missing required field PORT",
    "invalid type for DEBUG: expected boolean, got string"
  ]
}
```

### Multiple Environments

For projects with multiple environments, you can specify different `.env` and schema files:

```bash
# Development
dotenvcrab --env .env.development --schema env.development.schema.json

# Production
dotenvcrab --env .env.production --schema env.production.schema.json

# Test
dotenvcrab --env .env.test --schema env.test.schema.json
```

## Performance

dotenvcrab is written in Rust for maximum performance. It's designed to be fast enough to run in CI/CD pipelines and pre-commit hooks without adding noticeable delay.

## Troubleshooting

### Common Issues

#### "Failed to load .env file"

- Check that the `.env` file exists at the specified path
- Ensure you have read permissions for the file

#### "Failed to load schema"

- Check that the schema file exists at the specified path
- Ensure the schema is valid JSON

#### "Invalid type for [VARIABLE]"

- Check that the variable value matches the type specified in the schema
- For numbers, ensure there are no quotes or non-numeric characters
- For booleans, use one of the supported boolean representations

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](../CONTRIBUTING.md) for details.

## License

dotenvcrab is distributed under the MIT License. See [LICENSE](../LICENSE) for more information.
