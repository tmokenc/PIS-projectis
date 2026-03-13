FROM dart:stable AS builder

RUN apt-get update \
    && apt-get install -y --no-install-recommends protobuf-compiler ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY services/subject-service-dart-template/pubspec.yaml ./pubspec.yaml
RUN dart pub get
RUN dart pub global activate protoc_plugin

COPY services/subject-service-dart-template ./
COPY proto /proto
ENV PATH="/root/.pub-cache/bin:${PATH}"
RUN mkdir -p lib/generated \
    && protoc --proto_path=/proto --dart_out=grpc:lib/generated /proto/common.proto /proto/subject.proto \
    && dart pub get \
    && dart compile exe bin/server.dart -o /app/server

FROM debian:bookworm-slim AS runtime

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/server /app/server

EXPOSE 50053
CMD ["/app/server"]
