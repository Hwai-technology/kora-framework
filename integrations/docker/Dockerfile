# Multi-stage Dockerfile for Kora High-Performance Backend Application
# 1. Builder Stage using Official Rust Alpine Image
FROM rust:1.85-alpine AS builder

RUN apk add --no-cache musl-dev gcc libc-dev make

WORKDIR /usr/src/kora

# Copy workspace manifests
COPY Cargo.toml Cargo.lock ./
COPY crates ./crates

# Build release artifacts for native execution
RUN cargo build --release -p kora-cli

# 2. Production Minimal Runner Stage
FROM alpine:3.20

RUN apk add --no-cache ca-certificates tzdata

WORKDIR /app

# Copy compiled native binary
COPY --from=builder /usr/src/kora/target/release/kora /usr/local/bin/kora

# Expose default HTTP port
EXPOSE 7582

# Run native binary
CMD ["kora", "bench"]

