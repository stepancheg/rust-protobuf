#!/bin/sh -ex

cargo build

where_am_i=$(cd `dirname $0`/..; pwd)
PATH="$where_am_i/target/debug:$PATH"

rm -f test2/pb_*

protoc --rust_out test2 test2-proto/pb_basic.proto
protoc --rust_out test2 test2-proto/pb_test-sanitize-file-name.proto
protoc --rust_out test2 test2-proto/pb_text_format_test_data.proto
protoc --rust_out test2 -I test2-proto test2-proto/pb_test_*.proto

cd test2

rustc --test -L ../../target/debug lib.rs
./lib
