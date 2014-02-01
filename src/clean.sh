#!/bin/sh -ex

find . -name '*.dylib' -o -name '*.dSYM' -o -name '*.bin' -o -name '*.so' | xargs rm -rf
rm -f lib/protobuf

# vim: set ts=4 sw=4 et:
