#!/bin/sh -ex

cd $(dirname $0)

(
    cd ../..
    # building protoc-gen-rust
    cargo build
)

root=$(cd ../..; pwd)
PATH="$root/target/debug:$PATH"

protoc --rust_out . perftest_data.proto

cargo build --release
