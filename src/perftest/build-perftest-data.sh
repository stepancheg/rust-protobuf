#!/bin/sh -ex

cd $(dirname $0)
protoc ./perftest_data.proto --encode=PerftestData < perftest_data.pbtxt > perftest_data.pbbin

# vim: set ts=4 sw=4 et:
