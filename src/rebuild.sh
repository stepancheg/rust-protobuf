#!/bin/sh -ex

./clean.sh

( cd lib && rustc *.rc ) || false
rustc -Llib ./protobuf-bin-gen-rust.rc
rustc -Llib ./protoc-gen-rust.rc

# vim: set ts=4 sw=4 et:
