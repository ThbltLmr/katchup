# Build stage
FROM rust:1.76-slim as builder

# Set working directory
WORKDIR /usr/src/katchup

# Copy the manifest files
COPY api/Cargo.toml api/Cargo.lock ./

# Create a dummy main.rs to cache dependencies
RUN mkdir src && echo "fn main() {println!(\"dummy\");}" > src/main.rs

# Build dependencies
RUN cargo build --release

# Remove the dummy source file
RUN rm -rf src

# Copy your actual source code
COPY api/src ./src

# Force cargo to rebuild with actual source code
RUN touch src/main.rs

# Build the application
RUN cargo build --release

# Production stage
FROM debian:bullseye-slim

# Set working directory
WORKDIR /katchup

# Install SSL certificates for HTTPS requests
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the binary from the builder stage
COPY --from=builder /usr/src/katchup/target/release/api /katchup/api

# Expose port
EXPOSE 8000

# Run the binary
CMD ["/katchup/api"]
