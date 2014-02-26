#!/bin/sh -ex

./clean.sh

rustc lib/protobuf.rs
rustc -L . ./protobuf-bin-gen-rust.rs
rustc -L . ./protoc-gen-rust.rs

# vim: set ts=4 sw=4 et:
