#!/bin/sh -ex

# Build protoc-gen-rust
./rebuild.sh
# Generate from descriptor.proto
./regenerate.sh
# Build again with regenerated descriptor.proto
./rebuild.sh

./build-test.sh
./test

# vim: set ts=4 sw=4 et:
