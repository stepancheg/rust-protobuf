#!/bin/sh -ex

./rebuild.sh
./regenerate.sh
./rebuild.sh
rustc --test lib/protobuf.rs

# vim: set ts=4 sw=4 et:
