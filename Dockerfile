# Use the official Rust image as the base
FROM rust:1.75-slim as builder

# Install system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Set the working directory
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock (if it exists)
COPY Cargo.toml ./

# Create a dummy main.rs to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies (this layer will be cached)
RUN cargo build --release

# Remove the dummy main.rs and copy the actual source
RUN rm src/main.rs
COPY src/ ./src/

# Build the actual application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create a non-root user
RUN useradd -m -u 1000 kaspa

# Copy the binary from the builder stage
COPY --from=builder /app/target/release/kaspa-tx-generator /usr/local/bin/kaspa-tx-generator

# Set the user
USER kaspa

# Set the working directory
WORKDIR /home/kaspa

# Default command
CMD ["kaspa-tx-generator"]
