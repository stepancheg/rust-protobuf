#!/bin/sh -ex

cd $(dirname $0)

(
    cd ../protobuf
    # building protoc-gen-rust
    cargo build --features=$RUST_PROTOBUF_FEATURES --bin=protoc-gen-rust
)

root=$(cd ..; pwd)
PATH="$root/target/debug:$PATH"

protoc --rust_out . perftest_data.proto

cargo build --features=$RUST_PROTOBUF_FEATURES --release
