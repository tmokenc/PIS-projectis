# Subject Service Template (Dart)

This template shows how a colleague can implement a service in Dart while sharing only the `.proto` contracts.

## Generate code
```bash
dart pub global activate protoc_plugin
export PATH="$PATH:$HOME/.pub-cache/bin"
protoc \
  --proto_path=../../contracts/proto \
  --dart_out=grpc:lib/generated \
  ../../contracts/proto/common/v1/common.proto \
  ../../contracts/proto/subject/v1/subject.proto
```
