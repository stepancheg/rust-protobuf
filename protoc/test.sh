#!/bin/sh -e

cd $(dirname $0)

(
    echo "building protoc-gen-rust"
    cd ../protobuf-codegen
    cargo build --bin=protoc-gen-rust
)

echo "cargo check in test-protoc"
cd test-protoc
exec cargo check --features=$RUST_PROTOBUF_FEATURES

# vim: set ts=4 sw=4 et:
