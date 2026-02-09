# Multi-stage build for minimal final image
FROM rust:1.75-slim as builder

WORKDIR /usr/src/speedtest

# Install dependencies
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Create dummy main to cache dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Copy actual source code
COPY src ./src

# Build for release
RUN touch src/main.rs && \
    cargo build --release

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/speedtest/target/release/speedtest /usr/local/bin/speedtest

# Create non-root user
RUN useradd -m -u 1000 speedtest
USER speedtest

ENTRYPOINT ["speedtest"]
CMD ["--help"]
