ARG VARIANT="bullseye"
FROM rust:1-${VARIANT} as developer

RUN apt-get update && apt-get install -y \
    pkg-config \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /workspace

# Install common Rust tooling
RUN rustup component add rustfmt clippy

# -----------------------------------------------------------------------------
# Build stage
# -----------------------------------------------------------------------------

FROM developer as build

WORKDIR /app

COPY . .

RUN cargo build --release --bin tcp_listener

# -----------------------------------------------------------------------------
# Runtime stage
# -----------------------------------------------------------------------------
FROM build as runtime

WORKDIR /app

COPY --from=build /app/target/release/tcp_listener /usr/local/bin/tcp_listener

ENTRYPOINT ["tcp_listener"]