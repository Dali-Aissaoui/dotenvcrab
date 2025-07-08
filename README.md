# dotenvcrab

> **The fastest, type-safe, zero-dependency CLI for validating .env files—any stack, any CI, anywhere.**

---

## 🚀 Quickstart (TL;DR)

```sh
# Download and run for your platform (Linux x86_64 example)
curl -L -o dotenvcrab "https://github.com/Dali-Aissaoui/dotenvcrab/releases/latest/download/dotenvcrab-linux-amd64"
chmod +x dotenvcrab
./dotenvcrab --help
```

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

## Why dotenvcrab?

**dotenvcrab** is designed for modern teams who want:

- **Maximum speed** (Rust-native, 10-100x faster than Node.js alternatives)
- **Zero dependencies** (no Node.js, npm, or Python required—just a single binary)
- **True type safety** (validate numbers, booleans, enums, not just presence)
- **Strict validation** (fail on extra keys, catch typos and misconfigurations early)
- **First-class CI/CD support** (machine-readable JSON output, no runtime install needed)
- **Universal compatibility** (works in any stack: Node.js, Python, Go, Java, Docker, etc.)
- **Effortless updates** (just re-run the install command for the latest version)

### Comparison: dotenvcrab vs. dotenv-safe

| Feature                         | **dotenvcrab** (this project)          | [dotenv-safe](https://www.npmjs.com/package/dotenv-safe) |
| ------------------------------- | -------------------------------------- | -------------------------------------------------------- |
| **Language**                    | Rust (native binary)                   | JavaScript (npm package)                                 |
| **Performance**                 | Extremely fast (native code)           | Slower (Node.js runtime)                                 |
| **Cross-platform**              | Yes (Linux, macOS, Windows)            | Yes (Node.js required)                                   |
| **Type validation**             | Yes (string, number, boolean, enum)    | No (string presence only)                                |
| **Schema format**               | JSON schema                            | `.env.example` file                                      |
| **Strict mode (no extra keys)** | Yes                                    | No                                                       |
| **Default values**              | Yes                                    | No                                                       |
| **Colorized output**            | Yes                                    | No                                                       |
| **JSON output**                 | Yes (for CI/CD integration)            | No                                                       |
| **Zero dependencies**           | Yes (single binary, no Node.js needed) | Requires Node.js, npm                                    |
| **CI/CD integration**           | Native, simple binary download         | npm install + Node.js                                    |
| **Usage in any language stack** | Yes (binary, not tied to Node.js)      | Node.js/JavaScript only                                  |
| **Install/Update**              | Download/curl, always latest           | npm install/update                                       |
| **Custom error messages**       | Detailed, colorized, machine-readable  | Basic, plain text                                        |

**Choose dotenvcrab if you want: blazing speed, type safety, strict validation, and a tool that works everywhere with zero setup.**

---

## Performance

**Why is dotenvcrab so fast?**

- **Rust-native binaries** are compiled directly to machine code and run without the overhead of a virtual machine or garbage collector.
- **No runtime dependency loading**: dotenvcrab is a single executable, while Node.js tools must spin up the Node runtime and load many npm modules.
- **Instant startup**: For short-lived CLI tools, startup time dominates. Rust binaries typically start and finish in a few milliseconds, while Node.js CLIs can take 50–200ms just to start up.

### Benchmark

On a typical developer laptop (M1 MacBook, SSD):

| Tool        | Typical Run Time (validating .env) |
| ----------- | ---------------------------------- |
| dotenvcrab  | 2–5 ms                             |
| dotenv-safe | 70–150 ms                          |

Tested with: `time ./dotenvcrab` vs. `time npx dotenv-safe` on a sample project.

> Rust-native binaries start and finish in a fraction of the time required to spin up Node.js, load dependencies, and parse files.

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

## Usage

### Minimal Example

**1. `env.schema.json`**

```json
{
  "PORT": { "type": "number", "required": true },
  "DEBUG": { "type": "boolean", "default": false },
  "API_URL": { "type": "string", "required": true }
}
```

**2. `.env` (valid)**

```
PORT=8080
DEBUG=true
API_URL=https://api.example.com
```

**Run:**

```sh
./dotenvcrab
```

**Output:**

```
✅ All environment variables are valid!
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

### GitHub Actions: Single Environment

```yaml
jobs:
  validate_env:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Download dotenvcrab
        run: |
          curl -L -o dotenvcrab "https://github.com/Dali-Aissaoui/dotenvcrab/releases/latest/download/dotenvcrab-linux-amd64"
          chmod +x dotenvcrab
      - name: Validate .env
        run: ./dotenvcrab --json
```

### GitHub Actions: Matrix (Multiple Environments)

```yaml
jobs:
  validate_env:
    runs-on: ubuntu-latest
    strategy:
      matrix:
        envfile: [".env", ".env.production", ".env.staging"]
    steps:
      - uses: actions/checkout@v4
      - name: Download dotenvcrab
        run: |
          curl -L -o dotenvcrab "https://github.com/Dali-Aissaoui/dotenvcrab/releases/latest/download/dotenvcrab-linux-amd64"
          chmod +x dotenvcrab
      - name: Validate ${{ matrix.envfile }}
        run: ./dotenvcrab --env ${{ matrix.envfile }} --json
```

### GitHub Actions: Cache the Binary for Faster CI

```yaml
jobs:
  validate_env:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Cache dotenvcrab binary
        id: cache-dotenvcrab
        uses: actions/cache@v4
        with:
          path: dotenvcrab
          key: ${{ runner.os }}-dotenvcrab-latest
      - name: Download dotenvcrab if not cached
        if: steps.cache-dotenvcrab.outputs.cache-hit != 'true'
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

## Using dotenvcrab in CI/CD Pipelines

**Why validate your env files in CI/CD?**

- **Catch misconfigurations early:** Fail fast if critical env vars are missing, mistyped, or invalid before deploy.
- **Prevent production outages:** Ensure only valid, documented config reaches production.
- **Automate config checks:** Make env validation a required step for every PR, branch, or release.

**Typical CI/CD Pipeline Flow:**

1. Checkout code
2. Download the correct dotenvcrab binary for your runner
3. Run validation (on all relevant `.env` files)
4. Fail the build if validation fails (output is machine- and human-readable)
5. (Optional) Use JSON output for integration with other tools

### Example: GitHub Actions (with matrix, JSON output, and fail-fast)

```yaml
jobs:
  validate_env:
    runs-on: ubuntu-latest
    strategy:
      matrix:
        envfile: [".env", ".env.production", ".env.staging"]
    steps:
      - uses: actions/checkout@v4
      - name: Download dotenvcrab
        run: |
          curl -L -o dotenvcrab "https://github.com/Dali-Aissaoui/dotenvcrab/releases/latest/download/dotenvcrab-linux-amd64"
          chmod +x dotenvcrab
      - name: Validate ${{ matrix.envfile }}
        run: ./dotenvcrab --env ${{ matrix.envfile }} --json
```

**Best Practices:**

- Run dotenvcrab as early as possible in your pipeline.
- Use the `--json` flag for machine-readable output (for bots, dashboards, or custom reporting).
- Make env validation a required check for merging or deploying.

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

Contributions are welcome! Please see [CONTRIBUTING.md](./CONTRIBUTING.md) for details.

## Roadmap & Planned Features

We’re committed to making dotenvcrab the most robust and developer-friendly env validation tool available. Planned and proposed features include:

- **Pattern (regex) validation** for strings
  ```json
  {
    "EMAIL": { "type": "string", "pattern": "^[^@\\s]+@[^@\\s]+\\.[^@\\s]+$" }
  }
  // EMAIL must be a valid email address
  ```
- **Min/max length and value constraints** for strings and numbers
  ```json
  {
    "USERNAME": { "type": "string", "minLength": 3, "maxLength": 16 },
    "PORT": { "type": "number", "min": 1024, "max": 65535 }
  }
  // USERNAME must be 3-16 chars; PORT must be in valid range
  ```
- **Format validation** (email, URL, date, etc.)
  ```json
  {
    "URL": { "type": "string", "format": "uri" },
    "START_DATE": { "type": "string", "format": "date" }
  }
  // URL must be a valid URI; START_DATE must be a date
  ```
- **Conditional required fields** (e.g., `requiredIf`)
  ```json
  {
    "SSL_CERT": { "type": "string", "requiredIf": "SSL_ENABLED" }
  }
  // SSL_CERT required if SSL_ENABLED is set
  ```
- **Secret/masked field support** (for sensitive values)
  ```json
  {
    "API_KEY": { "type": "string", "secret": true }
  }
  // API_KEY will be masked in logs/output
  ```
- **Deprecated field warnings**
  ```json
  {
    "OLD_VAR": { "type": "string", "deprecated": true }
  }
  // Warn if OLD_VAR is present
  ```
- **Schema reuse and references** (import shared schema fragments)
  ```json
  {
    "$ref": "./shared.schema.json#/DATABASE"
  }
  // Reuse DATABASE definition from shared schema
  ```
- **Conditional logic** (`if/then/else` validation)
  ```json
  {
    "if": { "ENV": "production" },
    "then": { "SENTRY_DSN": { "type": "string", "required": true } },
    "else": { "SENTRY_DSN": { "required": false } }
  }
  // SENTRY_DSN required only in production
  ```
- **Custom error messages** per field
  ```json
  {
    "PORT": {
      "type": "number",
      "min": 1024,
      "max": 65535,
      "errorMessage": "PORT must be a number between 1024 and 65535."
    }
  }
  ```
- **Better error reporting** (grouping, suggestions, multi-format)
  > Example: Output errors grouped by type, with suggestions and optional YAML/JSON output.
- **Auto-generate .env templates** from schema
  > Example: `dotenvcrab schema-to-env` generates a `.env.example` file based on schema.
- **Schema linting** (warn about unused/misspelled properties)
  > Example: Warn if schema contains properties not used in `.env` or vice versa.
- **IDE/editor integration** (e.g., VSCode extension)
  > Example: Real-time validation and autocomplete for `.env` files in your editor.

**Have an idea or want to contribute?** Open an issue or PR—your feedback shapes the future of dotenvcrab!

## License

dotenvcrab is distributed under the MIT License. See [LICENSE](../LICENSE) for more information.
