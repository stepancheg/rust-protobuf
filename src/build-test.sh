#!/bin/sh -ex

cargo build

where_am_i=$(cd `dirname $0`/..; pwd)
PATH="$where_am_i/target:$PATH"

protoc --rust_out test proto/shrug.proto
protoc --rust_out test proto/test_root.proto
protoc --rust_out test proto/test_nonunique_enum.proto
protoc --rust_out test proto/test_ident.proto
protoc --rust_out test proto/test_lite_runtime.proto
protoc --rust_out test proto/text_format_test_data.proto

cd test

rustc --test -L ../../target lib.rs
./lib
