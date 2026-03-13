#!/usr/bin/env bash
set -euo pipefail

protoc   --proto_path=../../proto   --dart_out=grpc:lib/generated   ../../proto/common.proto   ../../proto/subject.proto
