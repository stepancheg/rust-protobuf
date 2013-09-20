#!/bin/sh -ex

./clean.sh

rustc lib/protobuf.rs
rustc -L lib ./protobuf-bin-gen-rust.rs
rustc -L lib ./protoc-gen-rust.rs

# vim: set ts=4 sw=4 et:
