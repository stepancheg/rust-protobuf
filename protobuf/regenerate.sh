#!/bin/sh -ex

cd "$(dirname "$0")"

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

cargo build --manifest-path=../protobuf-codegen/Cargo.toml
cargo build --manifest-path=../protoc-bin/Cargo.toml --bin protoc-bin-which

PROTOC=$(cargo run --manifest-path=../protoc-bin/Cargo.toml --bin protoc-bin-which)

where_am_i=$(
    cd ..
    pwd
)

rm -rf tmp-generated
mkdir tmp-generated

case $(uname) in
Linux)
    exe_suffix=""
    ;;
MSYS_NT*)
    exe_suffix=".exe"
    ;;
esac

mkdir -p tmp-generated/google-protobuf/google/protobuf/compiler
cp ../google-protobuf/src/google/protobuf/*.proto tmp-generated/google-protobuf/google/protobuf/
cp ../google-protobuf/src/google/protobuf/compiler/*.proto tmp-generated/google-protobuf/google/protobuf/compiler/

"$PROTOC" \
    --plugin=protoc-gen-rust="$where_am_i/target/debug/protoc-gen-rust$exe_suffix" \
    --rust_out tmp-generated \
    --rust_opt 'serde_derive=true inside_protobuf=true' \
    -I../proto \
    -I tmp-generated/google-protobuf \
    tmp-generated/google-protobuf/google/protobuf/any.proto \
    tmp-generated/google-protobuf/google/protobuf/api.proto \
    tmp-generated/google-protobuf/google/protobuf/descriptor.proto \
    tmp-generated/google-protobuf/google/protobuf/duration.proto \
    tmp-generated/google-protobuf/google/protobuf/empty.proto \
    tmp-generated/google-protobuf/google/protobuf/field_mask.proto \
    tmp-generated/google-protobuf/google/protobuf/source_context.proto \
    tmp-generated/google-protobuf/google/protobuf/struct.proto \
    tmp-generated/google-protobuf/google/protobuf/timestamp.proto \
    tmp-generated/google-protobuf/google/protobuf/type.proto \
    tmp-generated/google-protobuf/google/protobuf/wrappers.proto \
    tmp-generated/google-protobuf/google/protobuf/compiler/plugin.proto \
    ../proto/rustproto.proto

mv tmp-generated/descriptor.rs tmp-generated/plugin.rs tmp-generated/rustproto.rs src/
mv tmp-generated/*.rs src/well_known_types/
(
    cd src/well_known_types
    exec >mod.rs
    echo "// This file is generated. Do not edit"
    echo '//! Generated code for "well known types"'
    echo "//!"
    echo "//! [This document](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf) describes these types."

    mod_list() {
        # shellcheck disable=SC2010
        ls | grep -v mod.rs | sed -e 's,\.rs$,,'
    }

    echo
    mod_list | sed -e 's,^,mod ,; s,$,;,'

    echo
    mod_list | while read -r mod; do
        echo "pub use self::$mod::*;"
    done
)

# vim: set ts=4 sw=4 et:
