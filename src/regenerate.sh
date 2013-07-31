#!/bin/sh -ex

where_am_i=$(cd `dirname $0`; pwd)
PATH="$where_am_i:$PATH"

protoc --rust_out lib proto/shrug.proto

protoc --rust_out . -Iproto proto/google/protobuf/*.proto

mv descriptor.rs lib/

# vim: set ts=4 sw=4 et:
