#!/bin/sh -ex

cargo build

where_am_i=$(cd `dirname $0`/..; pwd)
PATH="$where_am_i/target/debug:$PATH"

rm -f test/pb_*

protoc --rust_out test proto/pb_shrug.proto
protoc --rust_out test proto/pb_test-sanitize-file-name.proto
protoc --rust_out test proto/pb_text_format_test_data.proto
protoc --rust_out test -I proto proto/pb_test_*.proto

cd test

rustc --test -L ../../target/debug lib.rs
./lib
