#!/usr/bin/env sh

set -ex

version=2.6.1
basename=protobuf-$version

curl -sL https://github.com/google/protobuf/releases/download/v$version/$basename.tar.bz2 | tar jx

cd $basename && ./configure --prefix=/usr && make && make install
