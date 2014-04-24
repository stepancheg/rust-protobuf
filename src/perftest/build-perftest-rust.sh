#!/bin/sh -ex

cd $(dirname $0)

root=$(cd ..; pwd)
PATH="$root:$PATH"

protoc --rust_out . perftest_data.proto

rustc -O -L $root perftest.rs
