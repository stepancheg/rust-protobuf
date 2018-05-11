#!/bin/sh -ex

for p in protobuf protobuf-codegen protobuf-codegen-pure protoc protoc-rust; do
    (
        cd $p
        cargo publish
    )
done

# vim: set ts=4 sw=4 et:
