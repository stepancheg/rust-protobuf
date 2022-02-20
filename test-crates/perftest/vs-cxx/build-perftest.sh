#!/bin/sh -ex

cd $(dirname $0)

./build-perftest-data.sh
cargo build --release
./build-perftest-cxx.sh

# vim: set ts=4 sw=4 et:
