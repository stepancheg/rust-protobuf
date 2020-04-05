#!/usr/bin/env bash

cd $(dirname $0)

for arch in linux-aarch_64 linux-ppcle_64 linux-x86_32 linux-x86_64 osx-x86_64 win32; do
    tag_name=`curl -s https://api.github.com/repos/protocolbuffers/protobuf/releases/latest | grep tag_name | cut -d '"' -f 4`
    TMPFILE=`mktemp`
    curl -sL https://github.com/protocolbuffers/protobuf/releases/download/${tag_name}/protoc-${tag_name#v}-${arch}.zip --output ${TMPFILE}.zip
    if [[ $arch == "win32" ]]; then
        unzip -p ${TMPFILE}.zip bin/protoc.exe > bin/protoc-${arch}.exe
    else
        unzip -p ${TMPFILE}.zip bin/protoc > bin/protoc-${arch}
    fi
    rm ${TMPFILE}.zip
done