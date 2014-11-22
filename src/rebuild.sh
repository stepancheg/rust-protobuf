#!/bin/sh -ex

# Test should be executed before build.
# See https://github.com/rust-lang/cargo/issues/961
cargo test
cargo build

# vim: set ts=4 sw=4 et:
