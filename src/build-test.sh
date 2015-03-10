#!/bin/sh -ex

cargo build

where_am_i=$(cd `dirname $0`/..; pwd)
PATH="$where_am_i/target/debug:$PATH"

protoc --rust_out test proto/shrug.proto
protoc --rust_out test proto/test-sanitize-file-name.proto
protoc --rust_out test proto/text_format_test_data.proto
protoc --rust_out test -I proto proto/test_*.proto

cd test

rustc --test -L ../../target/debug lib.rs
./lib
