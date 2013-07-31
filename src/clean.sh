#!/bin/sh -ex

find . -name '*.dylib' -o -name '*.dSYM' -o -name '*.bin' | xargs rm -rf

# vim: set ts=4 sw=4 et:
