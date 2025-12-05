FROM rust:1.75 as builder

WORKDIR /usr/src/deriax

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source
COPY src ./src
COPY signatures ./signatures

# Build release
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    libssl3 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/deriax/target/release/deriax /usr/local/bin/deriax
COPY --from=builder /usr/src/deriax/signatures /usr/local/share/deriax/signatures

# Create directories
RUN mkdir -p /data/input /data/output /data/cache /data/logs

WORKDIR /data

EXPOSE 8080

ENTRYPOINT ["deriax"]
CMD ["--help"]
