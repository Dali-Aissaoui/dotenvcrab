# Contributing to dotenvcrab

Thank you for considering contributing to dotenvcrab! This document outlines the process for contributing to the project and how to get started.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
  - [Development Environment](#development-environment)
  - [Project Structure](#project-structure)
- [Making Contributions](#making-contributions)
  - [Reporting Bugs](#reporting-bugs)
  - [Suggesting Features](#suggesting-features)
  - [Pull Requests](#pull-requests)
- [Development Workflow](#development-workflow)
  - [Branching Strategy](#branching-strategy)
  - [Commit Messages](#commit-messages)
  - [Testing](#testing)
- [Style Guidelines](#style-guidelines)
  - [Rust Code Style](#rust-code-style)
  - [Documentation](#documentation)
- [Release Process](#release-process)

## Code of Conduct

By participating in this project, you agree to abide by our [Code of Conduct](CODE_OF_CONDUCT.md). Please read it before contributing.

## Getting Started

### Development Environment

To set up your development environment:

1. **Install Rust and Cargo**
   - Follow the instructions at [rust-lang.org](https://www.rust-lang.org/tools/install)
   - dotenvcrab requires Rust 1.70.0 or later

2. **Clone the Repository**
   ```bash
   git clone https://github.com/Dali-Aissaoui/dotenvcrab.git
   cd dotenvcrab
   ```

3. **Install Development Dependencies**
   ```bash
   cargo build
   ```

4. **Run Tests**
   ```bash
   cargo test
   ```

### Project Structure

The project is organized as follows:

```
dotenvcrab/
├── src/                # Source code
│   ├── main.rs         # Entry point
│   ├── lib.rs          # Library exports
│   ├── cli.rs          # CLI argument handling
│   ├── schema.rs       # Schema parsing and validation
│   ├── validation.rs   # Validation logic
│   └── output.rs       # Output formatting
├── tests/              # Integration tests
├── examples/           # Example usage
└── docs/               # Documentation
```

## Making Contributions

### Reporting Bugs

If you find a bug, please report it by creating an issue on our [GitHub Issues](https://github.com/Dali-Aissaoui/dotenvcrab/issues) page. When reporting a bug, please include:

- A clear, descriptive title
- Steps to reproduce the bug
- Expected behavior
- Actual behavior
- Environment information (OS, Rust version, etc.)
- Any relevant logs or error messages

### Suggesting Features

We welcome feature suggestions! To suggest a feature:

1. Check existing issues to see if the feature has already been suggested
2. Create a new issue with the label "enhancement"
3. Clearly describe the feature and its use case
4. Explain how it would benefit the project

### Pull Requests

To submit a pull request:

1. Fork the repository
2. Create a new branch for your changes
3. Make your changes
4. Run tests to ensure they pass
5. Submit a pull request

All pull requests should:
- Address a specific issue or have a clear purpose
- Include tests for new functionality
- Update documentation as needed
- Follow the style guidelines

## Development Workflow

### Branching Strategy

- `main` - The main branch containing the latest stable release
- `develop` - Development branch for integrating features
- `feature/name` - Feature branches for new features
- `fix/name` - Fix branches for bug fixes

### Commit Messages

Follow the [Conventional Commits](https://www.conventionalcommits.org/) specification:

```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

Types include:
- `feat`: A new feature
- `fix`: A bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Adding or modifying tests
- `chore`: Maintenance tasks

### Testing

All code changes should be accompanied by tests. We use Rust's built-in testing framework:

- Unit tests should be included in the same file as the code they test
- Integration tests should be placed in the `tests/` directory
- Run tests with `cargo test`

## Style Guidelines

### Rust Code Style

- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `rustfmt` to format your code
- Use `clippy` to catch common mistakes and improve your code
- Run before committing:
  ```bash
  cargo fmt
  cargo clippy -- -D warnings
  ```

### Documentation

- Document all public functions, structs, and traits
- Use Rust doc comments (`///`) for API documentation
- Keep the README and other documentation up to date with changes

## Release Process

1. Update version in `Cargo.toml`
2. Update the CHANGELOG.md
3. Create a new GitHub release with release notes
4. Publish to crates.io with `cargo publish`

---

Thank you for contributing to dotenvcrab! Your efforts help make this project better for everyone.
