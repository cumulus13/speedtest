@echo off
REM Build script for Windows testing
REM This script tests the build process on Windows

echo ========================================
echo Testing speedtest Build on Windows
echo ========================================
echo.

echo [1/5] Checking Rust installation...
cargo --version
if errorlevel 1 (
    echo ERROR: Cargo not found. Please install Rust from https://rustup.rs/
    exit /b 1
)
echo OK
echo.

echo [2/5] Cleaning previous builds...
cargo clean
echo OK
echo.

echo [3/5] Running cargo check...
cargo check
if errorlevel 1 (
    echo ERROR: Cargo check failed
    exit /b 1
)
echo OK
echo.

echo [4/5] Running tests...
cargo test --lib
if errorlevel 1 (
    echo ERROR: Tests failed
    exit /b 1
)
echo OK
echo.

echo [5/5] Building release binary...
cargo build --release
if errorlevel 1 (
    echo ERROR: Build failed
    exit /b 1
)
echo OK
echo.

echo ========================================
echo Build completed successfully!
echo Binary location: target\release\speedtest.exe
echo ========================================
echo.

echo Testing binary...
target\release\speedtest.exe --version
echo.

echo All tests passed!
