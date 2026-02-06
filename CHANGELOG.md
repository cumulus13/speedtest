# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2024-XX-XX

### Added
- Initial release of speedtest
- Core speedtest functionality
  - Download speed testing
  - Upload speed testing
  - Server latency testing
  - Automatic best server selection
- Multiple output formats
  - JSON output
  - CSV output with custom delimiters
  - Simple text output
- Command-line interface with clap
- Progress bars using indicatif
- Support for HTTP and HTTPS
- Result sharing to speedtest.net
- Comprehensive error handling
- Server filtering (include/exclude by ID)
- Configurable timeouts
- Source address binding support
- Pre-allocation option for upload data
- Library API for integration into other projects

### Features
- Parallel downloads/uploads using rayon
- Automatic configuration retrieval from speedtest.net
- Distance-based server sorting using Haversine formula
- Type-safe implementation with strong error handling
- Cross-platform support (Windows, macOS, Linux)
- Zero external runtime dependencies (single binary)

### Documentation
- Comprehensive README with examples
- API documentation
- Usage examples for both CLI and library
- Architecture overview

[Unreleased]: https://github.com/cumulus13/speedtest/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/cumulus13/speedtest/releases/tag/v0.1.0
