#!/bin/sh -ex

cd $(dirname $0)

(
    cd ../protobuf
    # building protoc-gen-rust
    cargo build --features=$RUST_PROTOBUF_FEATURES --bin=protoc-gen-rust
)

cargo build --features=$RUST_PROTOBUF_FEATURES --release
