ARG VARIANT="bullseye"
FROM rust:1-${VARIANT} as developer

RUN apt-get update && apt-get install -y \
    pkg-config \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /workspace

# Install common Rust tooling
RUN rustup component add rustfmt clippy

