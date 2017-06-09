#!/bin/sh -ex

protoc --cpp_out=. ./perftest_data.proto
clang++ -std=c++11 -O3 -g -Wall -o perftest-cxx perftest-cxx.cxx perftest_data.pb.cc -lprotobuf

# vim: set ts=4 sw=4 et:
