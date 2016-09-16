#!/bin/sh -ex

cargo build

where_am_i=$(cd `dirname $0`/..; pwd)
PATH="$where_am_i/target/debug:$PATH"

run_test() {
    name=$1
    rm -f $name/pb_*

    protoc --rust_out $name -I $name-proto $name-proto/*.proto

    (
        cd $name

        rustc --test -L ../../target/debug lib.rs
        ./lib
    )
}

run_test test2

protoc_ver=$(protoc --version)
case "$protoc_ver" in
    "libprotoc 3"*) run_test test3 ;;
    *) echo "skipping tests for protobuf 3, because protoc version is ${protoc_ver}" ;;
esac

