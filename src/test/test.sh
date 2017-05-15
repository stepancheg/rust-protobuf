#!/bin/sh -ex

cd $(dirname $0)
prj_root=$(cd ../..; pwd)

(
    cd $prj_root
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

rm -f src/*/pb_*.rs
rm -f src/*/*_pb.rs

(
    cd src/common
    for f in *.rs; do
        for v in v2 v3; do
            (
                echo '// generated'
                echo 'include!("../common/'$f'");'
            ) > ../$v/$f
        done
    done
)

protoc -I../proto --rust_out src/v2 -I src/v2 src/v2/*.proto
if $HAS_PROTO3; then
    protoc -I../proto --rust_out src/v3 -I src/v3 src/v3/*.proto
    protoc -I../proto --rust_out src/google/protobuf -I src src/google/protobuf/*.proto
else
    # Because `#[cfg(nonexistent)]` still requires module files to exist
    # https://github.com/rust-lang/rust/pull/36482
    for f in src/v3/*.proto src/google/protobuf/*.proto; do
        f=${f%.proto}
        (
            echo '// generated'
            echo '// empty file because protobuf 3 is not available'
        ) > $f.rs
    done
fi

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
