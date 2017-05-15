#!/bin/sh -ex

cd $(dirname $0)

(
    cd ../..
    # building protoc-gen-rust
    cargo build --features=$RUST_PROTOBUF_FEATURES
)

root=$(cd ../..; pwd)
PATH="$root/target/debug:$PATH"

protoc --rust_out . perftest_data.proto

cargo build --features=$RUST_PROTOBUF_FEATURES --release
