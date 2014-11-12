#!/bin/sh -ex

cd $(dirname $0)

cargo build --release

root=$(cd ../..; pwd)
PATH="$root/target/release:$PATH"

protoc --rust_out . perftest_data.proto
