#!/bin/sh -ex

./clean.sh

OPT=${RUST_PROTOBUF_OPT:--O}

rustc $OPT -g lib/protobuf.rs
rustc $OPT -g -L . ./protobuf-bin-gen-rust.rs
rustc $OPT -g -L . ./protoc-gen-rust.rs

# vim: set ts=4 sw=4 et:
