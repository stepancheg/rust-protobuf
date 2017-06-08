#!/bin/sh -ex

cd $(dirname $0)

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

rm -rf tmp-generated
mkdir tmp-generated

protoc --rust_out tmp-generated -I../proto \
    ../proto/google/protobuf/*.proto \
    ../proto/google/protobuf/compiler/* \
    ../proto/rustproto.proto

mv tmp-generated/descriptor.rs tmp-generated/plugin.rs tmp-generated/rustproto.rs src/
mv tmp-generated/*.rs src/well_known_types/
(
    cd src/well_known_types
    exec > mod.rs
    echo "// This file is generated. Do not edit"

    mod_list() {
        ls | grep -v mod.rs | sed -e 's,\.rs$,,'
    }

    echo
    mod_list | sed -e 's,^,mod ,; s,$,;,'

    echo
    mod_list | while read mod; do
        echo "pub use self::$mod::*;"
    done
)

# vim: set ts=4 sw=4 et:
