#!/bin/sh -ex

cd $(dirname $0)
prj_root=$(cd ..; pwd)

(
    cd $prj_root/protobuf
    cargo build --features=$RUST_PROTOBUF_FEATURES --bin=protoc-gen-rust
)

cargo test --features="$RUST_PROTOBUF_FEATURES"

# vim: set ts=4 sw=4 et:
