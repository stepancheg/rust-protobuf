#!/bin/sh

# Print environment in travis job
# The script is cheap but may save hours of debugging

set -e

echo "pwd: `pwd`"
echo "uname: `uname`"
echo "PATH: $PATH"
rustc --version
cargo --version
echo
echo "Environment variables:"
env

# vim: set ts=4 sw=4 et:
