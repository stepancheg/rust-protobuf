#!/bin/sh -ex

./clean.sh

( cd lib && rustc *.rc ) || false
rustc -L lib ./protobuf-bin-gen-rust.rc
rustc -L lib ./protoc-gen-rust.rc

# vim: set ts=4 sw=4 et:
