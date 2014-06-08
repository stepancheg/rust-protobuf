#!/bin/sh -ex

cd $(dirname $0)

root=$(cd ..; pwd)
PATH="$root:$PATH"

protoc --rust_out . perftest_data.proto

rustc --opt-level=3 -L $root perftest.rs
