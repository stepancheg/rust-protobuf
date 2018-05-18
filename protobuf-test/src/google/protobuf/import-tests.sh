#!/bin/sh -e

cd $(dirname $0)

rm -rf *.proto protobuf-git

git clone --branch v3.5.2 --depth 1 https://github.com/google/protobuf/ protobuf-git

cp protobuf-git/src/google/protobuf/unittest*.proto ./

rm -rf protobuf-git

rm *_proto3.proto

# vim: set ts=4 sw=4 et:
