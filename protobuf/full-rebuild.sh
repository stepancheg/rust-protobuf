#!/bin/sh -ex

die() {
    echo "$@" >&2
    exit 1
}

cd $(dirname $0)

# Build protoc-gen-rust
cargo build --manifest-path ../protobuf-codegen/Cargo.toml

protoc_ver=$(protoc --version)
case "$protoc_ver" in
    "libprotoc 3"*)
        # Generate from descriptor.proto
        ./regenerate.sh
        # Build again with regenerated descriptor.proto
        cargo build --all
        ;;
    "libprotoc 2"*)
		echo "do not regenerate with proto 2"
        ;;
    *)
        die "unknown protoc version"
        ;;
esac

../protobuf-test/test.sh

# vim: set ts=4 sw=4 et:
