rust-protobuf
=============

<!-- https://travis-ci.org/stepancheg/rust-protobuf.png -->
[![GitHub Workflow Status](https://img.shields.io/github/workflow/status/stepancheg/rust-protobuf/CI)](https://github.com/stepancheg/rust-protobuf/actions?query=workflow%3ACI)
[![crates.io version](https://img.shields.io/crates/v/protobuf.svg)](https://crates.io/crates/protobuf)
[![License](https://img.shields.io/crates/l/protobuf.svg)](https://github.com/stepancheg/rust-protobuf/blob/master/LICENSE.txt)

[Protobuf](https://developers.google.com/protocol-buffers/docs/overview) implementation in [Rust](https://www.rust-lang.org/).

* Written in pure rust
* Generates rust code
* Has runtime library support for generated code
  (Coded{Input|Output}Stream impl)
* Supports both Protobuf versions 2 and 3
* and more

## Where is documentation

Documentation is moved to the crates.

* [protobuf=2](https://docs.rs/protobuf/=2)
* [protobuf=3](https://docs.rs/protobuf/>=3.0.0-alpha)

(Note both versions 2 and 3 or rust-protobuf support both `proto2` and `proto3`
syntax of `.proto` files.)

## About versions and branches

### Version 2

`2.*.*` is the latest stable version. `2.*.*` versions follow semver conventions,
including generated code: code generated with `2.*.*` is compatible with newer `2.*.*`.

### Version 3

Compared to version 2, it has:
* runtime reflection support
* JSON and text format parsing and printing (based on reflection)
* dynamic messages (messages which can be created using schema but without generated code)

Version 3 of rust-protobuf is mostly feature-complete, but to release it:
* more testing needed
* API need to be polished since breaking API is not semver-friendly

[Tracking issue for rust-protobuf=3](https://github.com/stepancheg/rust-protobuf/issues/518).

The crate **needs help**:
* testing
* documentation
* examples to be used as documentation
* feedback on API design
* feedback on implementation
* pull requests
* maybe even a new maintainer

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for a list of changes and compatility issues between versions.

## Related projects

* [prost](https://github.com/danburkert/prost) — another protobuf implementation in Rust, also has gRPC implementation
* [quick-protobuf](https://github.com/tafia/quick-protobuf) — alternative protobuf implementation in Rust
* [grpc-rs](https://github.com/pingcap/grpc-rs/) — another gRPC implementation for Rust
* [grpc-rust](https://github.com/stepancheg/grpc-rust) — incomplete implementation of gRPC based on this library
