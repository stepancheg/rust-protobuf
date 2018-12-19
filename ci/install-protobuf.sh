#!/bin/sh

set -ex

die() {
    echo "$@" >&2
    exit 1
}

test -n "$PROTOBUF_VERSION" || die "PROTOBUF_VERSION env var is undefined"

case `uname` in
    Linux)
        # Check we have ccache
        ccache --version
        export CC="ccache gcc"
        export CXX="ccache g++"

        case "$PROTOBUF_VERSION" in
        2*)
            basename=protobuf-$PROTOBUF_VERSION
            ;;
        3*)
            basename=protobuf-cpp-$PROTOBUF_VERSION
            ;;
        *)
            die "unknown protobuf version: $PROTOBUF_VERSION"
            ;;
        esac

        curl -sL https://github.com/protocolbuffers/protobuf/releases/download/v$PROTOBUF_VERSION/$basename.tar.gz | tar zx

        cd protobuf-$PROTOBUF_VERSION

        ./configure --prefix=$HOME && make -j2 && make install

    ;;
    MSYS_NT*)
        (
            cd $HOME
            curl -sLO https://github.com/protocolbuffers/protobuf/releases/download/v$PROTOBUF_VERSION/protoc-$PROTOBUF_VERSION-win32.zip
            unzip protoc-$PROTOBUF_VERSION-win32.zip
        )
    ;;
    *)
        die "unknown uname: `uname`"
    ;;
esac

$HOME/bin/protoc --version
