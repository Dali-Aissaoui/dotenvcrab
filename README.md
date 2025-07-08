# dotenvcrab

A blazing fast, portable CLI tool to validate your `.env` files against a JSON schema. Written in Rust for maximum performance and reliability.

---

## Features
- **Type validation**: string, number, boolean, enum
- **Required field checking**
- **Default values** for optional fields
- **Strict mode** to catch extra/typo keys
- **Colorized, human-friendly output**
- **JSON output** for CI/CD
- **Cross-platform**: macOS, Linux, Windows
- **No dependencies**: just a single binary

---

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

#### Linux (x86_64)
```sh
curl -L -o dotenvcrab "https://github.com/Dali-Aissaoui/dotenvcrab/releases/latest/download/dotenvcrab-linux-amd64"
chmod +x dotenvcrab
./dotenvcrab --help
```

#### macOS (Apple Silicon)
```sh
curl -L -o dotenvcrab "https://github.com/Dali-Aissaoui/dotenvcrab/releases/latest/download/dotenvcrab-macos-arm64"
chmod +x dotenvcrab
./dotenvcrab --help
```

#### macOS (Intel)
```sh
curl -L -o dotenvcrab "https://github.com/Dali-Aissaoui/dotenvcrab/releases/latest/download/dotenvcrab-macos-amd64"
chmod +x dotenvcrab
./dotenvcrab --help
```

#### Windows (PowerShell)
```powershell
Invoke-WebRequest -Uri "https://github.com/Dali-Aissaoui/dotenvcrab/releases/latest/download/dotenvcrab-windows-amd64.exe" -OutFile "dotenvcrab.exe"
./dotenvcrab.exe --help
```

## Features

- **Type validation**: string, number, boolean, enum
- **Required field checking**
- **Default values** for optional fields
- **Strict mode** to catch extra/typo keys
- **Colorized, human-friendly output**
- **JSON output** for CI/CD
- **Cross-platform**: macOS, Linux, Windows
- **No dependencies**: just a single binary

## Usage

### Basic Usage

1. Create your `env.schema.json` file (see Schema Format below).
2. Create your `.env` file.
3. Run:
   ```sh
   ./dotenvcrab
   ```

**Example output (success):**
```
✅ All environment variables are valid!
```

**Example output (failure):**
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

## CI/CD Integration

### GitHub Actions
```yaml
jobs:
  validate_env:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - name: Download dotenvcrab
        run: |
          curl -L -o dotenvcrab "https://github.com/Dali-Aissaoui/dotenvcrab/releases/latest/download/dotenvcrab-linux-amd64"
          chmod +x dotenvcrab
      - name: Validate .env
        run: ./dotenvcrab --json
```

### GitLab CI
```yaml
validate_env:
  stage: test
  script:
    - curl -L -o dotenvcrab "https://github.com/Dali-Aissaoui/dotenvcrab/releases/latest/download/dotenvcrab-linux-amd64"
    - chmod +x dotenvcrab
    - ./dotenvcrab --json
```

## Advanced Usage

### Strict Mode
Detect typos and undocumented variables:
```sh
./dotenvcrab --strict
```

### JSON Output
For CI/CD or programmatic parsing:
```sh
./dotenvcrab --json
```
Example:
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
```sh
./dotenvcrab --env .env.production --schema env.production.schema.json
./dotenvcrab --env .env.test --schema env.test.schema.json
```

## Updating
To update, just re-run the install command. The `/latest/download/` URL always fetches the newest release.

## Troubleshooting

- **"Failed to load .env file"**: Ensure the file exists and is readable.
- **"Failed to load schema"**: Ensure the schema file exists and is valid JSON.
- **"Invalid type for [VARIABLE]"**: Check your value and schema type.
- **File not executable**: Run `chmod +x dotenvcrab` after download.

## Contributing
Contributions are welcome! See [CONTRIBUTING.md](../CONTRIBUTING.md).

Contributions are welcome! Please see [CONTRIBUTING.md](../CONTRIBUTING.md) for details.

## License

dotenvcrab is distributed under the MIT License. See [LICENSE](../LICENSE) for more information.
