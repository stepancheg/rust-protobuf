#!/bin/sh -ex

./clean.sh

rustc lib/protobuf.rs
rustc ./protobuf-bin-gen-rust.rs
rustc ./protoc-gen-rust.rs

# vim: set ts=4 sw=4 et:
