# Contributing to speedtest

Thank you for your interest in contributing to speedtest! This document provides guidelines and instructions for contributing.

## Code of Conduct

This project adheres to a code of conduct. By participating, you are expected to uphold this code. Please be respectful and considerate in your interactions.

## How to Contribute

### Reporting Bugs

Before creating bug reports, please check existing issues to avoid duplicates. When creating a bug report, include:

- A clear, descriptive title
- Steps to reproduce the issue
- Expected behavior
- Actual behavior
- Your environment (OS, Rust version, etc.)
- Any relevant logs or error messages

### Suggesting Enhancements

Enhancement suggestions are tracked as GitHub issues. When creating an enhancement suggestion, include:

- A clear, descriptive title
- Detailed description of the proposed functionality
- Explanation of why this enhancement would be useful
- Possible implementation approach (if applicable)

### Pull Requests

1. Fork the repository
2. Create a new branch from `main`:
   ```bash
   git checkout -b feature/your-feature-name
   ```

3. Make your changes:
   - Follow the coding style (use `cargo fmt`)
   - Add tests for new functionality
   - Update documentation as needed
   - Ensure all tests pass: `cargo test`

4. Commit your changes:
   ```bash
   git commit -m "Add feature: description"
   ```
   - Use clear, descriptive commit messages
   - Reference issues if applicable

5. Push to your fork:
   ```bash
   git push origin feature/your-feature-name
   ```

6. Open a Pull Request:
   - Provide a clear description of the changes
   - Reference any related issues
   - Ensure CI passes

## Development Setup

### Prerequisites

- Rust 1.70 or later
- Cargo (comes with Rust)

### Building

```bash
# Clone the repository
git clone https://github.com/cumulus13/speedtest
cd speedtest

# Build the project
cargo build

# Run tests
cargo test

# Build release version
cargo build --release
```

### Running Tests

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_name
```

### Code Style

This project uses `rustfmt` for code formatting and `clippy` for linting.

```bash
# Format code
cargo fmt

# Run clippy
cargo clippy

# Run clippy with all features
cargo clippy --all-features -- -D warnings
```

## Project Structure

```
speedtest/
├── src/
│   ├── lib.rs              # Library entry point
│   ├── error.rs            # Error types
│   ├── types.rs            # Data structures
│   ├── utils.rs            # Utility functions
│   ├── http.rs             # HTTP client
│   ├── speedtest.rs        # Core logic
│   ├── latency.rs          # Latency testing
│   ├── download.rs         # Download testing
│   ├── upload.rs           # Upload testing
│   ├── share.rs            # Result sharing
│   └── bin/
│       └── speedtest.rs    # CLI application
├── examples/               # Example code
├── tests/                  # Integration tests
└── benches/                # Benchmarks

```

## Coding Guidelines

### Rust Best Practices

- Use descriptive variable and function names
- Prefer explicit error handling over `unwrap()` or `expect()`
- Add documentation comments for public APIs
- Use `Result<T, E>` for functions that can fail
- Prefer iterators over loops when appropriate

### Documentation

- All public items should have documentation comments
- Include examples in documentation where helpful
- Update README.md if adding new features
- Keep CHANGELOG.md up to date

### Testing

- Write unit tests for new functions
- Add integration tests for major features
- Ensure tests are deterministic
- Mock external dependencies when possible

Example test:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance_calculation() {
        let origin = (0.0, 0.0);
        let dest = (1.0, 1.0);
        let dist = distance(origin, dest);
        assert!(dist > 0.0);
    }
}
```

## Performance Considerations

- Profile code before optimizing
- Use benchmarks to measure improvements
- Avoid unnecessary allocations
- Consider using `&str` over `String` when possible
- Use `Vec::with_capacity` when size is known

## Documentation

### Adding Documentation

```rust
/// Calculate the distance between two coordinates.
///
/// Uses the Haversine formula to calculate the great-circle
/// distance between two points on a sphere.
///
/// # Arguments
///
/// * `origin` - The starting coordinates (lat, lon)
/// * `destination` - The ending coordinates (lat, lon)
///
/// # Returns
///
/// Distance in kilometers
///
/// # Example
///
/// ```
/// use speedtest::utils::distance;
///
/// let dist = distance((0.0, 0.0), (1.0, 1.0));
/// assert!(dist > 0.0);
/// ```
pub fn distance(origin: (f64, f64), destination: (f64, f64)) -> f64 {
    // Implementation
}
```

## Release Process

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md`
3. Create a git tag: `git tag -a v0.1.0 -m "Release v0.1.0"`
4. Push tag: `git push origin v0.1.0`
5. Publish to crates.io: `cargo publish`

## Questions?

If you have questions:

- Check existing issues and discussions
- Open a new issue with the "question" label
- Reach out to maintainers

## License

By contributing, you agree that your contributions will be licensed under the Apache License 2.0.
