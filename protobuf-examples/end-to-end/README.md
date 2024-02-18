# End-to-End Example

A fork-and-go example of using protobufs.

## Motivation

Simple and obvious examples for using `rust-protobuf` module were
lacking, so this was created.

## Dependencies

protoc (Protobuf Compiler)

[Installation instructions](https://grpc.io/docs/protoc-installation)

### Installing Protoc on Ubuntu (and similar)

`sudo apt install protobuf-compiler`

## Look here

Key files to read are:

* src/protos/example.proto
* src/main.rs
* build.rs

## Using

Standard rust package:
```
$ cargo build
$ cargo run
```

## Contributions

Contributions are welcome. File an issue or PR.
