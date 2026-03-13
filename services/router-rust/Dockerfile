FROM rust:1.93-slim AS builder
WORKDIR /app

RUN apt-get update     && apt-get install -y --no-install-recommends protobuf-compiler pkg-config libssl-dev ca-certificates     && rm -rf /var/lib/apt/lists/*

COPY proto ./proto
COPY services/router-rust ./services/router

WORKDIR /app/services/router
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update     && apt-get install -y --no-install-recommends ca-certificates libssl3     && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/services/router/target/release/router-rust /usr/local/bin/app
WORKDIR /app
EXPOSE 8080
CMD ["/usr/local/bin/app"]
