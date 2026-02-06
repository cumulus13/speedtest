# Build Instructions

This document provides detailed instructions for building speedtest from source.

## Prerequisites

### Required

- **Rust** 1.70 or later
  - Install from [rustup.rs](https://rustup.rs/)
  - Verify: `rustc --version`

- **Cargo** (comes with Rust)
  - Verify: `cargo --version`

### Optional

- **Git** - for cloning the repository
- **Docker** - for containerized builds

## Quick Start

```bash
# Clone the repository
git clone https://github.com/cumulus13/speedtest
cd speedtest

# Build in debug mode
cargo build

# Build in release mode (optimized)
cargo build --release

# The binary will be at:
# Debug: target/debug/speedtest
# Release: target/release/speedtest
```

## Build Modes

### Debug Build

```bash
cargo build
```

- Faster compilation
- Includes debug symbols
- No optimizations
- Larger binary size
- Good for development

### Release Build

```bash
cargo build --release
```

- Slower compilation
- Optimized for performance
- Smaller binary size
- Recommended for production use

### Release with LTO (Link Time Optimization)

This is already configured in `Cargo.toml`:

```toml
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
strip = true
```

```bash
cargo build --release
```

This produces the smallest and fastest binary.

## Platform-Specific Instructions

### Linux

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build
cargo build --release

# Install system-wide (optional)
sudo cp target/release/speedtest /usr/local/bin/
```

### macOS

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build
cargo build --release

# Install system-wide (optional)
sudo cp target/release/speedtest /usr/local/bin/
```

### Windows

```powershell
# Install Rust from https://rustup.rs/

# Build
cargo build --release

# The binary will be at: target\release\speedtest.exe
```

## Cross-Compilation

### For Linux from macOS/Windows

```bash
# Install cross-compilation toolchain
rustup target add x86_64-unknown-linux-gnu

# Build
cargo build --release --target x86_64-unknown-linux-gnu
```

### For Windows from Linux/macOS

```bash
# Install cross-compilation toolchain
rustup target add x86_64-pc-windows-gnu

# Install mingw (Linux)
sudo apt-get install mingw-w64

# Build
cargo build --release --target x86_64-pc-windows-gnu
```

### For macOS from Linux

```bash
# Install OSXCross (Linux only)
# Follow instructions at: https://github.com/tpoechtrager/osxcross

# Add target
rustup target add x86_64-apple-darwin

# Build
cargo build --release --target x86_64-apple-darwin
```

## Using Docker

### Build with Docker

```dockerfile
# Dockerfile
FROM rust:1.70 as builder

WORKDIR /app
COPY . .

RUN cargo build --release

FROM debian:bullseye-slim
COPY --from=builder /app/target/release/speedtest /usr/local/bin/

CMD ["speedtest"]
```

```bash
# Build Docker image
docker build -t speedtest .

# Run
docker run --rm speedtest speedtest --help
```

### Multi-stage Build for Smaller Images

```dockerfile
FROM rust:1.70-alpine as builder

RUN apk add --no-cache musl-dev

WORKDIR /app
COPY . .

RUN cargo build --release --target x86_64-unknown-linux-musl

FROM alpine:latest
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/speedtest /usr/local/bin/

CMD ["speedtest"]
```

## Static Linking (Linux)

For a fully static binary that doesn't depend on system libraries:

```bash
# Install musl target
rustup target add x86_64-unknown-linux-musl

# Install musl tools (Ubuntu/Debian)
sudo apt-get install musl-tools

# Build
cargo build --release --target x86_64-unknown-linux-musl
```

The resulting binary will have no external dependencies.

## Optimizing Binary Size

### Strip Debug Symbols

This is already configured in `Cargo.toml` with `strip = true`, but you can also do it manually:

```bash
# Linux/macOS
strip target/release/speedtest

# Windows (using MinGW)
strip target/release/speedtest.exe
```

### Use UPX (Optional)

```bash
# Install UPX
# Ubuntu/Debian: sudo apt-get install upx
# macOS: brew install upx

# Compress binary
upx --best --lzma target/release/speedtest
```

## Building Only the Library

```bash
# Build library only
cargo build --lib --release

# The library will be at:
# target/release/libspeedtest.rlib (static)
# target/release/libspeedtest.so (Linux)
# target/release/libspeedtest.dylib (macOS)
# target/release/speedtest.dll (Windows)
```

## Building with Features

```bash
# Build with all features
cargo build --release --all-features

# Build with no default features
cargo build --release --no-default-features
```

## Running Tests During Build

```bash
# Build and test
cargo build --release && cargo test --release

# Build, test, and check formatting
cargo build --release && cargo test && cargo fmt -- --check && cargo clippy
```

## Installation

### From Source

```bash
# Build and install to ~/.cargo/bin
cargo install --path .

# This puts the binary in your PATH if ~/.cargo/bin is in your PATH
```

### From crates.io

```bash
cargo install speedtest
```

## Troubleshooting

### OpenSSL Errors (Linux)

```bash
# Ubuntu/Debian
sudo apt-get install pkg-config libssl-dev

# Fedora
sudo dnf install openssl-devel

# Arch
sudo pacman -S openssl
```

### Linking Errors

If you get linking errors, try:

```bash
# Clean and rebuild
cargo clean
cargo build --release
```

### Out of Memory During Build

```bash
# Reduce parallelism
cargo build --release -j 1
```

Or increase swap space:

```bash
# Linux
sudo fallocate -l 4G /swapfile
sudo chmod 600 /swapfile
sudo mkswap /swapfile
sudo swapon /swapfile
```

## Build Performance Tips

### Use Cargo Cache

```bash
# The following directories can be cached for faster builds:
~/.cargo/registry
~/.cargo/git
target/
```

### Parallel Builds

Cargo automatically uses all CPU cores. To limit:

```bash
cargo build --release -j 4
```

### Use sccache

```bash
# Install sccache
cargo install sccache

# Set environment variable
export RUSTC_WRAPPER=sccache

# Build
cargo build --release
```

## Verification

After building, verify the binary works:

```bash
# Check version
./target/release/speedtest --version

# Run help
./target/release/speedtest --help

# Run a quick test (skip actual speed test)
./target/release/speedtest --list
```

## Creating Distributable Packages

### Linux (DEB)

```bash
# Install cargo-deb
cargo install cargo-deb

# Create .deb package
cargo deb

# Package will be at: target/debian/speedtest_*.deb
```

### macOS (Homebrew)

Create a Homebrew formula:

```ruby
class Speedtest < Formula
  desc "Speedtest CLI in Rust"
  homepage "https://github.com/cumulus13/speedtest"
  url "https://github.com/cumulus13/speedtest/archive/v0.1.0.tar.gz"
  sha256 "..."

  depends_on "rust" => :build

  def install
    system "cargo", "install", "--root", prefix, "--path", "."
  end

  test do
    system "#{bin}/speedtest", "--version"
  end
end
```

### Windows (MSI)

Use WiX Toolset or create a simple installer with Inno Setup.

## Continuous Integration

See `.github/workflows/ci.yml` for automated build configuration.

## Questions?

If you encounter build issues:

1. Check this document
2. Search existing issues on GitHub
3. Open a new issue with:
   - Your OS and version
   - Rust version (`rustc --version`)
   - Full error message
   - Steps to reproduce
