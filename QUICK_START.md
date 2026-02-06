# Quick Start Guide

Get up and running with speedtest in under 5 minutes!

## Installation

### Option 1: From Source (Fastest for first-time users)

```bash
# 1. Clone the repository
git clone https://github.com/cumulus13/speedtest
cd speedtest

# 2. Build the project
cargo build --release

# 3. Run it!
./target/release/speedtest
```

### Option 2: Install from crates.io

```bash
cargo install speedtest
speedtest
```

### Option 3: Download Pre-built Binary

Download from [Releases](https://github.com/cumulus13/speedtest/releases) page.

## Basic Usage

### Run a Complete Speed Test

```bash
speedtest
```

Output:
```
Retrieving speedtest.net configuration...
Testing from ISP Name (123.45.67.89)...
Retrieving speedtest.net server list...
Selecting best server based on ping...
Hosted by Server Sponsor (City, Country) [12.34 km]: 15.67 ms
Testing download speed...
[########################################] 100/100
Download: 95.43 Mbit/s
Testing upload speed...
[########################################] 80/80
Upload: 45.21 Mbit/s
```

### Common Options

```bash
# Skip download test
speedtest --no-download

# Skip upload test
speedtest --no-upload

# Simple output
speedtest --simple
# Output:
# Ping: 15.67 ms
# Download: 95.43 Mbit/s
# Upload: 45.21 Mbit/s

# JSON output
speedtest --json
```

## Using as a Library

### Add to Your Project

```toml
[dependencies]
speedtest = "0.1"
```

### Simple Example

```rust
use speedtest::{Speedtest, Result};

fn main() -> Result<()> {
    // Create instance
    let mut st = Speedtest::new(10, None, false)?;
    
    // Get config and find best server
    st.get_config()?;
    st.determine_best_server(None)?;
    
    // Run tests
    let download = st.test_download(None::<fn(usize, usize)>)?;
    let upload = st.test_upload(None::<fn(usize, usize)>, true)?;
    
    println!("Download: {:.2} Mbps", download / 1_000_000.0);
    println!("Upload: {:.2} Mbps", upload / 1_000_000.0);
    
    Ok(())
}
```

### With Progress Callback

```rust
use speedtest::Speedtest;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut st = Speedtest::new(10, None, false)?;
    st.get_config()?;
    st.determine_best_server(None)?;
    
    // Download with progress
    st.test_download(Some(|current, total| {
        println!("Progress: {}/{}", current, total);
    }))?;
    
    Ok(())
}
```

## CLI Examples

### List Available Servers

```bash
speedtest --list
```

### Test Against Specific Server

```bash
speedtest --server 12345
```

### Get JSON Output

```bash
speedtest --json
```

### Save Results to CSV

```bash
speedtest --csv >> results.csv
```

### Secure HTTPS Connection

```bash
speedtest --secure
```

## Next Steps

1. **Read the full README**: `cat README.md`
2. **Check examples**: `ls examples/`
3. **Run examples**: `cargo run --example simple`
4. **Read API docs**: `cargo doc --open`

## Troubleshooting

### Build Errors

```bash
# Clean and rebuild
cargo clean
cargo build --release
```

### Network Issues

```bash
# Try secure connection
speedtest --secure

# Increase timeout
speedtest --timeout 30
```

### Permission Errors (Linux/Mac)

```bash
# Make binary executable
chmod +x target/release/speedtest
```

## Getting Help

- Run `speedtest --help` for all options
- Check [README.md](README.md) for detailed documentation
- See [examples/](examples/) for code examples
- Open an issue on GitHub for bugs

## Quick Reference

| Command | Description |
|---------|-------------|
| `speedtest` | Run full test |
| `speedtest --simple` | Simple output |
| `speedtest --json` | JSON output |
| `speedtest --csv` | CSV output |
| `speedtest --list` | List servers |
| `speedtest --no-download` | Skip download |
| `speedtest --no-upload` | Skip upload |
| `speedtest --server ID` | Use specific server |
| `speedtest --secure` | Use HTTPS |
| `speedtest --help` | Show all options |

Happy testing! 🚀
