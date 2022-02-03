#!/bin/sh

set -ex

die() {
    echo "$@" >&2
    exit 1
}

test -n "$PROTOBUF_VERSION" || die "PROTOBUF_VERSION env var is undefined"

path_tr() {
    case $(uname) in
    MINGW64*)
        cygpath --mixed "$1"
        ;;
    *)
        echo "$1"
        ;;
    esac
}

test -n "$GITHUB_PATH"
test -n "$GITHUB_ENV"

path_tr "$HOME/pb/bin" >>"$GITHUB_PATH"
cat << EOF >> "$GITHUB_ENV"
LD_LIBRARY_PATH=$(path_tr $HOME/pb/lib)
PKG_CONFIG_PATH=$(path_tr $HOME/pb/lib/pkgconfig)
PROTOBUF_PREFIX=$(path_tr $HOME/pb)
EOF

if test -e "$HOME/pb/bin/protoc" -o -e "$HOME/pb/bin/protoc.exe"; then
    echo "Already exists"
    $HOME/pb/bin/protoc --version
    exit 0
fi

if test -e "$HOME/pb"; then
    echo "... but $HOME/pb exists, and in it:"
    (
        cd $HOME/pb
        ls
        echo "EOF"
    )
fi

cd
rm -rf pb
mkdir pb

case $(uname) in
Linux | Darwin)
    # Check we have ccache
    # ccache --version
    # export CC="ccache gcc"
    # export CXX="ccache g++"

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

    ./configure --prefix=$HOME/pb && make -j2 && make install

    ;;
MSYS_NT* | MINGW64*)
    (
        cd pb
        curl -sLO https://github.com/protocolbuffers/protobuf/releases/download/v$PROTOBUF_VERSION/protoc-$PROTOBUF_VERSION-win32.zip
        unzip protoc-$PROTOBUF_VERSION-win32.zip
    )
    ;;
*)
    die "unknown uname: $(uname)"
    ;;
esac

$HOME/pb/bin/protoc --version
