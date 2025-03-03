# syntax=docker/dockerfile:experimental
FROM rust as builder

WORKDIR /app
# Copy the source code into the container.
COPY . .

# Build the project in release mode.
RUN cargo build --release

# Use a minimal base image to reduce final image size.
FROM debian:buster-slim

WORKDIR /app
# Copy the binary from the builder stage.
COPY --from=builder /app/target/release/lambert_w /app/lambert_w

# Set the startup command.
CMD ["/app/lambert_w"]
# Entrypoint to take argument
ENTRYPOINT ["/app/lambert_w"]
