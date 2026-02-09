#!/bin/bash
# Build script for Linux/macOS testing
# This script tests the build process on Unix-like systems

set -e

echo "========================================"
echo "Testing speedtest Build on Unix"
echo "========================================"
echo ""

echo "[1/5] Checking Rust installation..."
if ! command -v cargo &> /dev/null; then
    echo "ERROR: Cargo not found. Please install Rust from https://rustup.rs/"
    exit 1
fi
cargo --version
echo "OK"
echo ""

echo "[2/5] Cleaning previous builds..."
cargo clean
echo "OK"
echo ""

echo "[3/5] Running cargo check..."
cargo check
echo "OK"
echo ""

echo "[4/5] Running tests..."
cargo test --lib
echo "OK"
echo ""

echo "[5/5] Building release binary..."
cargo build --release
echo "OK"
echo ""

echo "========================================"
echo "Build completed successfully!"
echo "Binary location: target/release/speedtest"
echo "========================================"
echo ""

echo "Testing binary..."
./target/release/speedtest --version
echo ""

echo "All tests passed!"
