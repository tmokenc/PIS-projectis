FROM rust:1.93-slim AS builder
WORKDIR /app

RUN apt-get update     && apt-get install -y --no-install-recommends protobuf-compiler pkg-config libssl-dev ca-certificates     && rm -rf /var/lib/apt/lists/*

COPY proto ./proto
COPY services/notification-service-rust ./services/notification-service-rust

WORKDIR /app/services/notification-service-rust
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update     && apt-get install -y --no-install-recommends ca-certificates libssl3     && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/services/notification-service-rust/target/release/notification-service-rust /usr/local/bin/app
WORKDIR /app
RUN mkdir -p /data
EXPOSE 50052
CMD ["/usr/local/bin/app"]
