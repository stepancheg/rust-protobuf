#!/bin/sh

set -ex

cd $(dirname $0)

protoc --cpp_out=. interop_pb.proto
c++ --version
c++ -std=c++11 -Wall -O1 \
    interop.cc interop_pb.pb.cc \
    `pkg-config --cflags --libs protobuf` \
    -o interop
./interop self-test

# vim: set ts=4 sw=4 et:
