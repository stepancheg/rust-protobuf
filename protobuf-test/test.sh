#!/bin/sh -ex

cd $(dirname $0)
prj_root=$(cd ..; pwd)

(
    cd $prj_root/protobuf
    cargo build --features=$RUST_PROTOBUF_FEATURES --bin=protoc-gen-rust
)

PATH="$prj_root/target/debug:$PATH"

protoc_ver=$(protoc --version)
case "$protoc_ver" in
    "libprotoc 3"*)
        HAS_PROTO3=true
        ;;
    *)
        echo "skipping tests for protobuf 3, because protoc version is ${protoc_ver}"
        HAS_PROTO3=false
        ;;
esac

features=
if $HAS_PROTO3; then
    features=proto3
fi
if test -n "$RUST_PROTOBUF_FEATURES"; then
    if test -n "$features"; then
        features="$features "
    fi
    features="$features$RUST_PROTOBUF_FEATURES"
fi

cargo test --features="$features"

# vim: set ts=4 sw=4 et:
