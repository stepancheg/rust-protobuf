#!/bin/sh -e

ci/env-debug.sh
if [ -z "$ON_WINDOWS" ]; then ccache --zero-stats; fi
ci/install-protobuf.sh
if [ -z "$ON_WINDOWS" ]; then ccache --show-stats; fi
export PATH="$HOME/bin:$PATH"
export LD_LIBRARY_PATH="$HOME/lib"
which protoc
protoc --version
if [ -z "$ON_WINDOWS" ];
    then PKG_CONFIG_PATH="$HOME/lib/pkgconfig" interop/cxx/compile.sh
fi
export RUST_BACKTRACE=1

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
fi

# vim: set ts=4 sw=4 et:
