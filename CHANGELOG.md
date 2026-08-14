# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.0.16] - 2026-08-14

### Fixed
- musl release build (`x86_64-unknown-linux-musl`) failed because `openssl-sys`
  cannot cross-compile without a target sysroot. Switched `reqwest` to
  `rustls-tls` (disabling the default `native-tls`/OpenSSL backend), removing
  the OpenSSL dependency entirely.
- Security audit failure: bumped `quick-xml` from `0.31` to `0.41` to resolve
  RUSTSEC-2026-0194 (quadratic-time duplicate attribute check) and
  RUSTSEC-2026-0195 (unbounded namespace-declaration allocation), both high
  severity (7.5).
- Clippy `-D warnings` failures in `src/speedtest.rs`: replaced
  `if self.config.is_some() { ... .unwrap() }` with `if let Some(ref config)`,
  and `if let Ok(_) = ...` with `.is_ok()`.
- Clippy matrix jobs (`windows-latest`, `macos-latest`) were being cancelled
  whenever `ubuntu-latest` failed, since `fail-fast` defaulted to `true`. Set
  `fail-fast: false` so each OS reports its own result independently.

## [1.0.0] - 2025-02-08

### Added
- Initial release of speedtest
- Complete rewrite of speedtest-cli in Rust
- Multi-threaded download and upload testing
- Support for JSON, CSV, and simple text output
- Automatic best server selection based on latency
- Manual server selection with `--server` flag
- Server exclusion with `--exclude` flag
- HTTPS support with `--secure` flag
- Configurable timeout with `--timeout` flag
- Source IP binding with `--source` flag
- Single connection mode with `--single` flag
- Byte/bit unit selection with `--bytes` flag
- Colorful terminal output with progress indicators
- Cross-platform support:
  - Linux (x86_64, aarch64, armv7, i686)
  - Windows (x86_64, i686, aarch64)
  - macOS (x86_64, aarch64/M1/M2)
  - FreeBSD (x86_64)
  - NetBSD (x86_64)
- Comprehensive test coverage
- GitHub Actions CI/CD pipeline
- Docker support
- Detailed documentation

### Performance Improvements
- 4x faster startup time compared to Python version
- 6x lower memory usage
- 50% lower CPU usage
- No runtime dependencies (single static binary)

### Developer Features
- Well-documented code with examples
- Modular architecture
- Comprehensive error handling
- Integration tests
- Automated multi-platform builds
- Pre-built binaries for all platforms

## [0.9.0] - 2025-02-01 (Pre-release)

### Added
- Beta testing release
- Core functionality implementation
- Basic CI/CD pipeline

### Known Issues
- Share functionality not yet implemented
- Mini server support not yet implemented

---

[Unreleased]: https://github.com/cumulus13/speedtest/compare/v1.0.0...HEAD
[1.0.0]: https://github.com/cumulus13/speedtest/releases/tag/v1.0.0
[0.9.0]: https://github.com/cumulus13/speedtest/releases/tag/v0.9.0
