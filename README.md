# rust-protobuf

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

Documentation is [hosted on docs.rs](https://docs.rs/protobuf).

## Versions and branches

### Version 3

Version 3 is current stable version. Compared to version 2 it implements:
* runtime reflection
* JSON and text format parsing and printing
* dynamic messages (messages which can be created from `.proto` file on the fly
  without code generation)

### Version 2

Version is previous stable version. Only most critical bugfixes will be applied
to 2.x version, otherwise it won't be maintained.

### Help

The crate **needs help**:
* testing
* documentation
* examples to be used as documentation
* feedback on API design
* feedback on implementation
* pull requests
* a new maintainer

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for a list of changes and compatility issues between versions.

## Related projects

* [prost](https://github.com/danburkert/prost) — another protobuf implementation in Rust, also has gRPC implementation
* [quick-protobuf](https://github.com/tafia/quick-protobuf) — alternative protobuf implementation in Rust
* [grpc-rs](https://github.com/pingcap/grpc-rs/) — another gRPC implementation for Rust
* [grpc-rust](https://github.com/stepancheg/grpc-rust) — incomplete implementation of gRPC based on this library
