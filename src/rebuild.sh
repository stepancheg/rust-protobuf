#!/bin/sh -ex

./clean.sh

rustc -O lib/protobuf.rs
rustc -O -L . ./protobuf-bin-gen-rust.rs
rustc -O -L . ./protoc-gen-rust.rs

# vim: set ts=4 sw=4 et:
