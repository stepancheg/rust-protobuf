#!/bin/sh -ex

cd $(dirname $0)

cargo test --features="$RUST_PROTOBUF_FEATURES"

# vim: set ts=4 sw=4 et:
