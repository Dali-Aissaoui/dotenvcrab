# dotenvcrab

A blazing fast Rust-powered CLI tool that validates .env files against JSON schema definitions - 1000x faster than Node.js alternatives.

## Features

- **Type Validation**: Ensures variables are the correct type (string, number, boolean, enum)
- **Required Field Checking**: Verifies all required variables are present
- **Default Values**: Supports default values for optional fields
- **Schema Documentation**: Self-documenting configuration requirements
- **Strict Mode**: Optionally fails when extra keys are present
- **Performance**: 1000x faster than JavaScript alternatives
- **Cross-Platform**: Works on Windows, macOS, and Linux

## Installation

```bash
# npm
npm install --save-dev dotenvcrab

# yarn
yarn add --dev dotenvcrab

# pnpm
pnpm add --save-dev dotenvcrab
```

## Usage

### Command Line

After installation, you can use dotenvcrab directly in your terminal:

```bash
npx dotenvcrab
```

With options:

```bash
npx dotenvcrab --env .env.production --schema env.production.schema.json --strict
```

### In package.json Scripts

Add dotenvcrab to your scripts:

```json
{
  "scripts": {
    "validate-env": "dotenvcrab",
    "prestart": "dotenvcrab",
    "prebuild": "dotenvcrab --env .env.production --schema env.production.schema.json"
  }
}
```

### With Husky Pre-commit Hook

1. Install husky:
```bash
npm install --save-dev husky
npx husky install
```

2. Create a pre-commit hook:
```bash
npx husky add .husky/pre-commit "npm run validate-env"
```

### Programmatic Usage

```javascript
const DotEnvCrab = require('dotenvcrab');

const validator = new DotEnvCrab();

// Simple validation
validator.validate()
  .then(result => {
    console.log('Validation successful!');
  })
  .catch(error => {
    console.error('Validation failed:', error.output);
    process.exit(1);
  });

// With options
validator.validate({
  envFile: '.env.production',
  schemaFile: 'env.production.schema.json',
  strict: true,
  json: true
})
  .then(result => {
    console.log('Validation successful!');
    console.log(result.data); // Parsed JSON result
  })
  .catch(error => {
    console.error('Validation failed:', error.output);
    process.exit(1);
  });
```

## Schema Format

The schema is defined in a JSON file with the following structure:

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
  }
}
```

## CI/CD Integration

### GitHub Actions

```yaml
name: Validate Environment

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

jobs:
  validate:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      
      # Option 1: Using npm package (slower but simpler)
      - name: Setup Node.js
        uses: actions/setup-node@v2
        with:
          node-version: '16'
      - name: Install dotenvcrab
        run: npm install -g dotenvcrab
      - name: Validate .env
        run: dotenvcrab --json
        
      # Option 2: Using binary directly (faster)
      - name: Setup dotenvcrab binary
        run: |
          curl -sSL https://github.com/Dali-Aissaoui/dotenvcrab/releases/latest/download/dotenvcrab-linux-amd64 -o /usr/local/bin/dotenvcrab
          chmod +x /usr/local/bin/dotenvcrab
      - name: Validate .env
        run: /usr/local/bin/dotenvcrab --json
```

### GitLab CI

```yaml
validate_env:
  stage: test
  script:
    # Option 1: Using npm package
    - npm install -g dotenvcrab
    - dotenvcrab --json
    
    # Option 2: Using binary directly (faster)
    - curl -sSL https://github.com/Dali-Aissaoui/dotenvcrab/releases/latest/download/dotenvcrab-linux-amd64 -o /usr/local/bin/dotenvcrab
    - chmod +x /usr/local/bin/dotenvcrab
    - /usr/local/bin/dotenvcrab --json
```

## License

MIT
