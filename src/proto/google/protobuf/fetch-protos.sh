#!/bin/sh -ex

# fetch protos from protobuf respository
# comment out `reserved` directive for comatibility with protobuf 2.6.1
# https://github.com/google/protobuf/issues/1669#issuecomment-240598089
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
        | sed -e 's,^\( *\)\(reserved.*\),\1// \2,' \
        > $f
done

# vim: set ts=4 sw=4 et:
