# syntax=docker/dockerfile:experimental
FROM rust:latest as builder

WORKDIR /app
# Copy the source code into the container.
COPY . .

# Build the project in release mode.
RUN cargo build --release

# Use a minimal base image to reduce final image size.
FROM debian:latest

WORKDIR /app
# Copy the binary from the builder stage.
COPY --from=builder /app/target/release/rust_lambert_w /app/rust_lambert_w

# Set the startup command.
CMD ["/app/rust_lambert_w"]
# Entrypoint to take argument
ENTRYPOINT ["/app/rust_lambert_w"]
