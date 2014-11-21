#!/bin/sh -ex

cd $(dirname $0)

# Build protoc-gen-rust
./rebuild.sh
# Generate from descriptor.proto
./regenerate.sh
# Build again with regenerated descriptor.proto
./rebuild.sh

./build-test.sh

rm -f perftest/Cargo.lock
./perftest/build-perftest-rust.sh

# vim: set ts=4 sw=4 et:
