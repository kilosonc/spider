# Use a Rust base image with Cargo installed
FROM rust:1.86.0 AS builder

# Set the working directory inside the container
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files
COPY cargo-client.toml ./Cargo.toml
COPY client.rs ./src/main.rs

RUN cargo build --release

# Start a new stage to create a smaller image without unnecessary build dependencies
FROM debian:latest

# Set the working directory
WORKDIR /app

# Copy the built binary from the previous stage
COPY --from=builder /app/target/release/client ./

# Command to run the application
CMD ["./client"]