#!/bin/sh -e

cd $(dirname $0)

set -x
git checkout ./plugin.rs ./lib/descriptor.rs ./lib/shrug.rs

# vim: set ts=4 sw=4 et:
