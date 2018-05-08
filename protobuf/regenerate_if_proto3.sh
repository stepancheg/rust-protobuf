#!/bin/sh -e

die() {
    echo "$@" >&2
    exit 1
}

case $(protoc --version) in
"libprotoc 3"*) $(dirname $0)/regenerate.sh ;;
"libprotoc 2"*) ;;
*) die "unknown protoc version"
esac

# vim: set ts=4 sw=4 et:
