#!/usr/bin/env bash

set -e

cd $(dirname $0)

tag_name=`curl -s https://api.github.com/repos/protocolbuffers/protobuf/releases/latest | grep tag_name | cut -d '"' -f 4`
echo "updating protoc binaries to version $tag_name" >&2

for arch in linux-aarch_64 linux-ppcle_64 linux-x86_32 linux-x86_64 osx-x86_64 win32; do
    TMPFILE=`mktemp`
    url="https://github.com/protocolbuffers/protobuf/releases/download/${tag_name}/protoc-${tag_name#v}-${arch}.zip"
    echo "downloading $url..." >&2
    curl -sL "$url" --output ${TMPFILE}.zip
    if [[ $arch == "win32" ]]; then
        unzip -p ${TMPFILE}.zip bin/protoc.exe > bin/protoc-${arch}.exe
    else
        unzip -p ${TMPFILE}.zip bin/protoc > bin/protoc-${arch}
    fi
    rm ${TMPFILE}.zip
done
