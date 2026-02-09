# Contributing to speedtest

Thank you for your interest in contributing to speedtest! This document provides guidelines and instructions for contributing.

## Code of Conduct

By participating in this project, you agree to maintain a respectful and collaborative environment.

## How to Contribute

### Reporting Bugs

1. Check if the bug has already been reported in [Issues](https://github.com/cumulus13/speedtest/issues)
2. If not, create a new issue with:
   - Clear title and description
   - Steps to reproduce
   - Expected vs actual behavior
   - Your environment (OS, Rust version, etc.)
   - Any relevant logs or screenshots

### Suggesting Features

1. Check if the feature has been suggested in [Issues](https://github.com/cumulus13/speedtest/issues)
2. If not, create a new issue with:
   - Clear description of the feature
   - Use cases and benefits
   - Possible implementation approach

### Pull Requests

1. Fork the repository
2. Create a new branch (`git checkout -b feature/your-feature-name`)
3. Make your changes
4. Run tests (`cargo test`)
5. Run clippy (`cargo clippy`)
6. Format code (`cargo fmt`)
7. Commit your changes (`git commit -m 'Add some feature'`)
8. Push to your branch (`git push origin feature/your-feature-name`)
9. Open a Pull Request

## Development Setup

### Prerequisites

- Rust 1.70 or later
- Git

### Building

```bash
git clone https://github.com/cumulus13/speedtest.git
cd speedtest
cargo build
```

### Running Tests

```bash
cargo test
```

### Running Clippy

```bash
cargo clippy --all-targets --all-features
```

### Formatting

```bash
cargo fmt
```

## Code Style

- Follow Rust standard style guidelines
- Use `cargo fmt` before committing
- Ensure `cargo clippy` passes without warnings
- Write tests for new features
- Document public APIs

## Testing

- Write unit tests for new functionality
- Ensure all tests pass before submitting PR
- Add integration tests where appropriate

## Documentation

- Update README.md if adding new features
- Document all public functions and types
- Include examples in documentation where helpful

## Commit Messages

- Use clear, descriptive commit messages
- Start with a verb in present tense (Add, Fix, Update, etc.)
- Reference issue numbers where applicable

Example:
```
Add support for IPv6 connections (#123)

- Implement IPv6 socket binding
- Update tests for IPv6
- Add documentation for IPv6 usage
```

## Release Process

Releases are managed by the project maintainers. Version numbers follow [Semantic Versioning](https://semver.org/).

## Questions?

Feel free to open an issue for any questions about contributing.

Thank you for contributing to speedtest! 🦀
