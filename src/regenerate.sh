#!/bin/sh -ex

die() {
    echo "$@" >&2
    exit 1
}

protoc_ver=$(protoc --version)
case "$protoc_ver" in
    "libprotoc 3"*) ;;
    *)
        die "you need to use protobuf 3 to regenerate .rs from .proto"
    ;;
esac

where_am_i=$(cd `dirname $0`/..; pwd)
PATH="$where_am_i/target/debug:$PATH"

protoc --rust_out . -Iproto \
    proto/google/protobuf/*.proto \
    proto/google/protobuf/compiler/* \
    proto/rustproto.proto

mv descriptor.rs plugin.rs rustproto.rs lib/

# vim: set ts=4 sw=4 et:
