#!/bin/sh -e

cd $(dirname $0)

set -x
git checkout lib/plugin.rs lib/descriptor.rs

# vim: set ts=4 sw=4 et:
