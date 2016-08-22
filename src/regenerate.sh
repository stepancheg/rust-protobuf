#!/bin/sh -ex

where_am_i=$(cd `dirname $0`/..; pwd)
PATH="$where_am_i/target/debug:$PATH"

protoc --rust_out . -Iproto proto/google/protobuf/*.proto proto/google/protobuf/compiler/*

mv descriptor.rs plugin.rs lib/

# vim: set ts=4 sw=4 et:
