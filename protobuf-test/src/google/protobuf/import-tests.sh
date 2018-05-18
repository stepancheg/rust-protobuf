#!/bin/sh -e

cd $(dirname $0)

rm -rf *.proto protobuf-git

git clone --branch v3.5.2 --depth 1 https://github.com/google/protobuf/ protobuf-git

cp protobuf-git/src/google/protobuf/unittest*.proto ./

rm -rf protobuf-git

# This file causes memory overflow in `rustc` when executed on travis-ci
rm unittest_enormous_descriptor.proto

# These files duplicate similar files for proto2 and cannot be compiled together
rm *_proto3.proto

# vim: set ts=4 sw=4 et:
