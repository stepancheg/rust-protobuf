#!/bin/sh

set -ex

cd "$(dirname "$0")"

rm -rf protobuf protobuf-git

mkdir protobuf

git clone --branch v3.5.2 --depth 1 https://github.com/google/protobuf/ protobuf-git

rsync -r --include='*.proto' --include='*/' --exclude='*' --prune-empty-dirs protobuf-git protobuf

rm -rf protobuf-git

# vim: set ts=4 sw=4 et:
