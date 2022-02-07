#!/bin/sh

# Update bundled .proto files

set -ex

cd "$(dirname $0)"

cargo build --manifest-path=../protoc-bin/Cargo.toml --bin protoc-bin-print-paths

eval "$(cargo run --manifest-path=../protoc-bin/Cargo.toml --bin protoc-bin-print-paths)"

test -n "$PROTOC"
test -n "$PROTOBUF_INCLUDE"


rm -rf google
cp -r "$PROTOBUF_INCLUDE/google" google

cp -r "$PROTOBUF_INCLUDE/google" ../protobuf-parse/src/proto/
cp -r "rustproto.proto" ../protobuf-parse/src/proto/

# vim: set ts=4 sw=4 et:
