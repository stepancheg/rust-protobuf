#!/bin/sh -ex

./rebuild.sh
./regenerate.sh
./rebuild.sh
rust test lib/protobuf.rc

# vim: set ts=4 sw=4 et:
