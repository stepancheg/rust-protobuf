#!/bin/sh -ex

where_am_i=$(cd "$(dirname "$0")/.."; pwd)
PATH="$where_am_i/target/debug:$PATH"

cd "$where_am_i/examples"

mkdir -p basicproto
protoc --rust_out basicproto basic.proto
mv basicproto/basic.rs basicproto/mod.rs

cargo build --example basic
