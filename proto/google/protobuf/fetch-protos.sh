#!/bin/sh -ex

# fetch protos from protobuf respository
for f in descriptor.proto compiler/plugin.proto \
    any.proto \
    api.proto \
    duration.proto \
    empty.proto \
    field_mask.proto \
    source_context.proto \
    struct.proto \
    timestamp.proto \
    type.proto \
    wrappers.proto \
; do
    curl -s https://raw.githubusercontent.com/google/protobuf/v3.1.0/src/google/protobuf/$f \
        > $f
done

# vim: set ts=4 sw=4 et:
