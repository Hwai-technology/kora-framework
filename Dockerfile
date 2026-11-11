# ==============================================================================
# Kora Framework Official Container Image
# Multi-stage build for ultra-clean, minimal, secure production distribution
# ==============================================================================

# ------------------------------------------------------------------------------
# Stage 1: Build Kora Compiler & CLI Toolchain
# ------------------------------------------------------------------------------
FROM rust:slim-bookworm AS builder

WORKDIR /usr/src/kora

# Install required build tooling
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    build-essential \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy workspace manifest and source trees
COPY Cargo.toml Cargo.lock ./
COPY crates ./crates

# Compile the optimized release binary of the Kora CLI
RUN cargo build --release --bin kora

# Strip binary to reduce final container footprint
RUN strip target/release/kora

# ------------------------------------------------------------------------------
# Stage 2: Minimal Secure Runtime Environment
# ------------------------------------------------------------------------------
FROM debian:bookworm-slim AS runtime

LABEL org.opencontainers.image.title="Kora Framework"
LABEL org.opencontainers.image.description="High-performance, Clean Architecture backend programming language & framework compiling to native Rust."
LABEL org.opencontainers.image.url="https://github.com/Hwai-technology/kora-framework"
LABEL org.opencontainers.image.source="https://github.com/Hwai-technology/kora-framework"
LABEL org.opencontainers.image.licenses="AGPL-3.0-only"
LABEL org.opencontainers.image.vendor="Hwai Technology"

ENV DEBIAN_FRONTEND=noninteractive
ENV RUSTUP_HOME=/usr/local/rustup
ENV CARGO_HOME=/usr/local/cargo
ENV PATH="/usr/local/cargo/bin:${PATH}"
ENV KORA_PORT=7582
ENV PORT=7582

# Install runtime dependencies and build essentials required for native Rust compilation
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    curl \
    git \
    build-essential \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Install minimal Rust toolchain for on-demand Kora native codegen and compilation
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y \
    --default-toolchain stable \
    --profile minimal \
    --no-modify-path \
    && chmod -R a+w /usr/local/cargo

# Install the precompiled Kora binary
COPY --from=builder /usr/src/kora/target/release/kora /usr/local/bin/kora

# Create a non-privileged user and workspace directory
RUN useradd -m -u 1001 -s /bin/bash korauser && \
    mkdir -p /app && \
    chown -R korauser:korauser /app

WORKDIR /app

USER korauser

# Expose Kora's dedicated default server port
EXPOSE 7582

# Default health check probe against the auto-generated health endpoint
HEALTHCHECK --interval=30s --timeout=5s --start-period=5s --retries=3 \
    CMD curl -f http://127.0.0.1:7582/health || exit 1

ENTRYPOINT ["kora"]
CMD ["run"]

