#!/bin/sh -e

if [ -z "$ON_WINDOWS" ]; then ccache --zero-stats; fi
ci/install-protobuf.sh
if [ -z "$ON_WINDOWS" ]; then ccache --show-stats; fi
export PATH="$HOME/bin:$PATH"
which protoc
protoc --version
export RUST_BACKTRACE=1

rustc --version
PATH=/home/travis/bin:$PATH cargo build --manifest-path=protobuf-codegen/Cargo.toml
protobuf/regenerate.sh
cargo test --all
# `cargo test --all --features=FFF` doesn't work if there are crates without feature `FFF`
# hence the explicit list of tests
protobuf-test/test.sh
protobuf-codegen-pure-test/test.sh
protoc/test.sh
cargo build --all --all-targets

if [ -z "$ON_WINDOWS" ]; then
    cargo doc -p protobuf
    cargo doc -p protoc
fi

# vim: set ts=4 sw=4 et:
