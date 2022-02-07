#!/bin/sh -e

# Update bundled .proto files

cd "$(pwd)"

rm -rf google
mkdir -p google/protobuf/compiler
cp ../google-protobuf/src/google/protobuf/any.proto google/protobuf/
cp ../google-protobuf/src/google/protobuf/api.proto google/protobuf/
cp ../google-protobuf/src/google/protobuf/descriptor.proto google/protobuf/
cp ../google-protobuf/src/google/protobuf/duration.proto google/protobuf/
cp ../google-protobuf/src/google/protobuf/empty.proto google/protobuf/
cp ../google-protobuf/src/google/protobuf/field_mask.proto google/protobuf/
cp ../google-protobuf/src/google/protobuf/source_context.proto google/protobuf/
cp ../google-protobuf/src/google/protobuf/struct.proto google/protobuf/
cp ../google-protobuf/src/google/protobuf/timestamp.proto google/protobuf/
cp ../google-protobuf/src/google/protobuf/type.proto google/protobuf/
cp ../google-protobuf/src/google/protobuf/wrappers.proto google/protobuf/
cp ../google-protobuf/src/google/protobuf/compiler/plugin.proto google/protobuf/compiler/

# vim: set ts=4 sw=4 et:
