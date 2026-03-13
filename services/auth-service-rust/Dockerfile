FROM rust:1.93-slim AS builder
WORKDIR /app

RUN apt-get update     && apt-get install -y --no-install-recommends protobuf-compiler pkg-config libssl-dev ca-certificates     && rm -rf /var/lib/apt/lists/*

COPY proto ./proto
COPY services/auth-service-rust ./services/auth-service-rust

WORKDIR /app/services/auth-service-rust
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update     && apt-get install -y --no-install-recommends ca-certificates libssl3     && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/services/auth-service-rust/target/release/auth-service-rust /usr/local/bin/app
WORKDIR /app
RUN mkdir -p /data
EXPOSE 50051
CMD ["/usr/local/bin/app"]
