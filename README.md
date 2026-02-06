# Speedtest-rs

[![Crates.io](https://img.shields.io/crates/v/speedtest.svg)](https://crates.io/crates/speedtest)
[![Documentation](https://docs.rs/speedtest/badge.svg)](https://docs.rs/speedtest)
[![License](https://img.shields.io/crates/l/speedtest.svg)](https://github.com/cumulus13/speedtest/blob/main/LICENSE)

Speedtest-cli for testing internet bandwidth

## Features

- 🚀 **Fast & Efficient**: Written in Rust for maximum performance
- 🔒 **Type-Safe**: Leverages Rust's type system for reliability
- 📊 **Progress Bars**: Beautiful terminal progress indicators using indicatif
- 🌐 **HTTP/HTTPS Support**: Choose between secure and non-secure connections
- 📝 **Multiple Output Formats**: JSON, CSV, and simple text output
- 🎯 **Server Selection**: Automatic best server selection or manual server choice
- ⚡ **Parallel Downloads/Uploads**: Utilizes rayon for concurrent operations
- 🛡️ **Production-Ready**: Comprehensive error handling and testing

## Installation

### From crates.io

```bash
cargo install speedtest
```

### From source

```bash
git clone https://github.com/cumulus13/speedtest
cd speedtest
cargo build --release
```

The binary will be available at `target/release/speedtest`.

## Usage

### Basic Usage

```bash
# Run a complete speedtest
speedtest

# Skip download test
speedtest --no-download

# Skip upload test
speedtest --no-upload

# Use secure HTTPS connections
speedtest --secure
```

### Output Formats

```bash
# Simple output
speedtest --simple

# JSON output
speedtest --json

# CSV output
speedtest --csv

# CSV with custom delimiter
speedtest --csv --csv-delimiter ';'

# Print CSV headers
speedtest --csv-header
```

### Server Selection

```bash
# List available servers
speedtest --list

# Use specific server
speedtest --server 12345

# Exclude specific servers
speedtest --exclude 12345 --exclude 67890
```

### Advanced Options

```bash
# Display values in bytes instead of bits
speedtest --bytes

# Share results and get image URL
speedtest --share

# Set custom timeout (in seconds)
speedtest --timeout 30

# Bind to specific source address
speedtest --source 192.168.1.100

# Disable upload data pre-allocation (for low-memory systems)
speedtest --no-pre-allocate
```

## Library Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
speedtest = "0.1"
```

Example code:

```rust
use speedtest::{Speedtest, Result};

fn main() -> Result<()> {
    // Create speedtest instance
    // Parameters: timeout (seconds), source_address, use_secure
    let mut speedtest = Speedtest::new(10, None, false)?;
    
    // Get configuration from speedtest.net
    speedtest.get_config()?;
    
    // Find the best server based on ping
    speedtest.determine_best_server(None)?;
    
    let best = speedtest.get_best_server().unwrap();
    println!("Testing against: {} ({}) - {:.2} ms",
             best.sponsor, best.name, best.latency);
    
    // Run download test with progress callback
    let download_speed = speedtest.test_download(Some(|current, total| {
        println!("Download progress: {}/{}", current, total);
    }))?;
    
    // Run upload test
    let upload_speed = speedtest.test_upload(
        Some(|current, total| {
            println!("Upload progress: {}/{}", current, total);
        }),
        true  // pre-allocate data
    )?;
    
    println!("Download: {:.2} Mbps", download_speed / 1_000_000.0);
    println!("Upload: {:.2} Mbps", upload_speed / 1_000_000.0);
    
    // Get detailed results
    let results = speedtest.get_results();
    println!("Ping: {:.2} ms", results.ping);
    println!("Bytes sent: {}", results.bytes_sent);
    println!("Bytes received: {}", results.bytes_received);
    
    // Share results (optional)
    if let Ok(share_url) = speedtest.share_results() {
        println!("Share: {}", share_url);
    }
    
    Ok(())
}
```

## API Documentation

### Main Types

- **`Speedtest`**: Main struct for performing speed tests
- **`SpeedtestResults`**: Contains test results (download, upload, ping, etc.)
- **`Server`**: Information about a speedtest server
- **`Config`**: Speedtest configuration from speedtest.net

### Key Methods

- `Speedtest::new(timeout, source_address, secure)`: Create new instance
- `get_config()`: Download speedtest.net configuration
- `get_servers()`: Retrieve list of available servers
- `get_closest_servers(limit)`: Get nearest servers
- `determine_best_server()`: Find server with lowest latency
- `test_download(callback)`: Perform download speed test
- `test_upload(callback, pre_allocate)`: Perform upload speed test
- `share_results()`: Share results to speedtest.net

## Architecture

The project is structured into several modules for maintainability:

```
src/
├── lib.rs              # Library entry point
├── error.rs            # Error types and Result alias
├── types.rs            # Data structures (Server, Config, Results, etc.)
├── utils.rs            # Helper functions (distance calculation, etc.)
├── http.rs             # HTTP client wrapper
├── speedtest.rs        # Core Speedtest struct and server operations
├── latency.rs          # Server latency testing
├── download.rs         # Download speed testing
├── upload.rs           # Upload speed testing
├── share.rs            # Result sharing functionality
└── bin/
    └── speedtest.rs    # CLI application
```

## Performance Considerations

- Uses `rayon` for parallel downloads/uploads with configurable thread pools
- Pre-allocation of upload data to improve performance (can be disabled)
- Efficient binary data handling with zero-copy operations where possible
- Connection pooling via `reqwest`

## Error Handling

All operations return `Result<T, SpeedtestError>` for comprehensive error handling:

```rust
use speedtest::{Speedtest, SpeedtestError};

match Speedtest::new(10, None, false) {
    Ok(mut st) => {
        if let Err(e) = st.get_config() {
            match e {
                SpeedtestError::ConfigRetrievalError(msg) => {
                    eprintln!("Config error: {}", msg);
                },
                SpeedtestError::HttpError(e) => {
                    eprintln!("HTTP error: {}", e);
                },
                _ => eprintln!("Other error: {}", e),
            }
        }
    },
    Err(e) => eprintln!("Failed to create speedtest: {}", e),
}
```

## Testing

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_distance
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Original [speedtest-cli](https://github.com/sivel/speedtest-cli) by Matt Martz
- [speedtest.net](https://www.speedtest.net/) by Ookla

## Changelog

### Version 0.1.0

- Initial release
- Basic speedtest functionality
- Download and upload testing
- Server selection and latency testing
- JSON, CSV, and simple output formats
- Progress bars with indicatif
- Result sharing to speedtest.net

## Roadmap

- [ ] Async/await support with tokio
- [ ] Custom server support (Speedtest Mini)
- [ ] More detailed progress reporting
- [ ] Retry logic for failed requests
- [ ] Configuration file support
- [ ] Result history tracking
- [ ] WebSocket support for real-time updates

## FAQ

**Q: Why Rust instead of Python?**  
A: Rust provides better performance, memory safety, and enables distributing a single binary without runtime dependencies.

**Q: Is this compatible with the original speedtest-cli?**  
A: Yes, it implements the same core functionality and can produce compatible output formats.

**Q: Can I use this in my own project?**  
A: Absolutely! This is both a CLI tool and a library. Add it as a dependency and use the API.

**Q: Does this work on Windows/Mac/Linux?**  
A: Yes, it's cross-platform and works on all major operating systems.

## Support

If you encounter any issues or have questions:

- Open an issue on [GitHub](https://github.com/cumulus13/speedtest/issues)
- Check existing issues for solutions
- Read the [API documentation](https://docs.rs/speedtest)
