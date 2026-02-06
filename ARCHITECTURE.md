# Architecture

This document describes the architecture and design decisions of speedtest.

## Overview

speedtest is designed as both a library and a CLI application, following Rust best practices for modularity, error handling, and performance.

## Project Structure

```
speedtest/
├── Cargo.toml                 # Package manifest
├── src/
│   ├── lib.rs                 # Library entry point & public API
│   ├── error.rs               # Error types using thiserror
│   ├── types.rs               # Data structures and DTOs
│   ├── utils.rs               # Utility functions
│   ├── http.rs                # HTTP client wrapper
│   ├── speedtest.rs           # Core Speedtest struct
│   ├── latency.rs             # Latency measurement
│   ├── download.rs            # Download speed testing
│   ├── upload.rs              # Upload speed testing
│   ├── share.rs               # Result sharing to speedtest.net
│   └── bin/
│       └── speedtest.rs       # CLI application
├── examples/                  # Usage examples
├── tests/                     # Integration tests
└── benches/                   # Benchmarks
```

## Core Components

### 1. Error Handling (`error.rs`)

Uses `thiserror` for ergonomic error definitions:

```rust
#[derive(Error, Debug)]
pub enum SpeedtestError {
    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),
    // ... more variants
}

pub type Result<T> = std::result::Result<T, SpeedtestError>;
```

**Design Decisions:**
- Single `Result` type alias for consistency
- Automatic conversion from common error types
- Descriptive error messages
- No panic-based error handling

### 2. Type System (`types.rs`)

Strongly typed data structures with serde support:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Server {
    pub id: u32,
    pub sponsor: String,
    // ...
}
```

**Design Decisions:**
- Separate types for input (XML) vs output (JSON)
- `#[derive]` for common traits
- Optional fields use `Option<T>`
- Validation in constructors

### 3. HTTP Client (`http.rs`)

Wrapper around `reqwest` for consistent behavior:

```rust
pub struct HttpClient {
    client: Client,
    secure: bool,
}
```

**Design Decisions:**
- Blocking client for simplicity
- Configurable timeout
- Automatic User-Agent setting
- Cache-busting for all requests
- GZIP compression support

### 4. Core Logic (`speedtest.rs`)

Main struct containing all speedtest functionality:

```rust
pub struct Speedtest {
    config: Option<Config>,
    http_client: HttpClient,
    servers: HashMap<String, Vec<Server>>,
    // ...
}
```

**Design Decisions:**
- Builder-like API
- Lazy initialization
- Stateful design (stores results)
- Fluent interface where appropriate

### 5. Parallel Operations

Both download and upload use `rayon` for parallelism:

```rust
pool.install(|| {
    urls.par_iter().enumerate().for_each(|(i, url)| {
        // Parallel work
    });
});
```

**Design Decisions:**
- Thread pool per operation
- Configurable parallelism from server config
- Atomic counters for thread-safe aggregation
- Timeout checking in each thread

## Data Flow

### Typical Usage Flow

```
1. Create Speedtest instance
   ↓
2. get_config() - Fetch speedtest.net configuration
   ↓
3. get_servers() - Retrieve available servers
   ↓
4. determine_best_server() - Ping test to find lowest latency
   ↓
5. test_download() - Parallel download test
   ↓
6. test_upload() - Parallel upload test
   ↓
7. get_results() - Retrieve SpeedtestResults
   ↓
8. share_results() (optional) - Share to speedtest.net
```

### Configuration Flow

```
speedtest.net API
        ↓ (XML)
   Parse with quick-xml
        ↓
   SpeedtestConfig struct
        ↓
   Convert to Config
        ↓
   Store in Speedtest
```

### Server Selection Flow

```
speedtest.net API
        ↓ (XML)
   Parse servers
        ↓
   Calculate distances
        ↓
   Sort by distance
        ↓
   Ping top N servers
        ↓
   Select best latency
```

## Performance Considerations

### Memory Management

- Pre-allocation for upload data (configurable)
- Streaming for large downloads
- Minimal cloning (use references where possible)
- Arc for shared data in parallel contexts

### Concurrency

- Thread pool size from server configuration
- Rayon for work-stealing parallelism
- Atomic operations for counters
- No locks in hot paths

### Network Optimization

- HTTP/2 when available (via reqwest)
- Connection pooling
- Parallel requests
- GZIP compression
- Keep-alive connections

## Error Recovery

### Graceful Degradation

- If one server fails, try others
- Timeout-based cancellation
- Partial results are valid
- Continue on individual request failures

### Error Propagation

```
Operation Error
      ↓
SpeedtestError
      ↓
Result<T>
      ↓
User/CLI
```

## Design Patterns

### 1. Builder Pattern

```rust
let speedtest = Speedtest::new(timeout, source, secure)?;
speedtest.get_config()?;
speedtest.determine_best_server(None)?;
```

### 2. Strategy Pattern

Different implementations for download/upload in separate modules.

### 3. Template Method

```rust
impl Speedtest {
    // Template for speed tests
    fn test_speed<F>(...) {
        // 1. Prepare
        // 2. Execute parallel requests
        // 3. Aggregate results
        // 4. Calculate speed
    }
}
```

### 4. Adapter Pattern

HTTP client wraps reqwest to provide consistent interface.

## Testing Strategy

### Unit Tests

- Pure functions (utils, calculations)
- Data structure conversions
- Error handling

### Integration Tests

- Full workflow tests
- Mock server responses
- Error scenarios

### Performance Tests

- Benchmark critical paths
- Memory profiling
- Concurrency testing

## Security Considerations

### Network Security

- HTTPS support
- Certificate validation
- No credential storage
- Secure random for cache-busting

### Input Validation

- Parse XML safely
- Validate server responses
- Bounds checking
- Type safety

## Extensibility Points

### 1. Custom HTTP Client

Replace `HttpClient` for special requirements.

### 2. Progress Callbacks

```rust
test_download(Some(|current, total| {
    // Custom progress handling
}))
```

### 3. Output Formats

Easy to add new serialization formats via serde.

### 4. Server Selection

Override server selection with custom logic.

## Dependencies

### Core Dependencies

- **reqwest** - HTTP client
- **tokio** - Async runtime (for reqwest)
- **rayon** - Data parallelism
- **serde** - Serialization
- **quick-xml** - XML parsing
- **thiserror** - Error handling

### CLI Dependencies

- **clap** - Argument parsing
- **indicatif** - Progress bars

### Rationale

All dependencies are:
- Well-maintained
- Widely used
- Production-ready
- Licensed appropriately

## Future Enhancements

### Planned Features

1. Async/await API
2. WebSocket support
3. Custom server support
4. Result history
5. Configuration files

### Potential Optimizations

1. Zero-copy deserialization
2. Custom allocator for upload data
3. Connection pre-warming
4. Adaptive parallelism

## Compatibility

### Rust Version

- Minimum: 1.70
- Tested on: stable, beta
- Edition: 2021

### Platforms

- Linux (x86_64, ARM)
- macOS (x86_64, ARM64)
- Windows (x86_64)
- BSD variants (untested but should work)

## Code Style

- Format with `rustfmt`
- Lint with `clippy`
- Follow Rust API guidelines
- Document public APIs
- Write idiomatic Rust

## Contributing

See CONTRIBUTING.md for:
- Code organization
- Testing requirements
- Documentation standards
- Review process
