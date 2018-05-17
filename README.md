rust-protobuf
=============

<!-- https://travis-ci.org/stepancheg/rust-protobuf.png -->
[![Build Status](https://img.shields.io/travis/stepancheg/rust-protobuf.svg)](https://travis-ci.org/stepancheg/rust-protobuf)
[![crates.io version](https://img.shields.io/crates/v/protobuf.svg)](https://crates.io/crates/protobuf)
[![License](https://img.shields.io/crates/l/protobuf.svg)](https://github.com/stepancheg/rust-protobuf/blob/master/LICENSE.txt)

[Protobuf](https://developers.google.com/protocol-buffers/docs/overview) implementation in [Rust](https://www.rust-lang.org/).

* Written in pure rust
* Generate rust code
* Has runtime library for generated code
  (Coded{Input|Output}Stream impl)

## How to generate rust code

There are several ways to generate rust code from `.proto` files

### Invoke protoc programmatically with protoc-rust crate (recommended)

Have a look at readme in [protoc-rust crate](https://github.com/stepancheg/rust-protobuf/tree/master/protoc-rust).

### Use pure rust protobuf parser and code generator (alpha)

Readme should be in
[protobuf-codegen-pure crate](https://github.com/stepancheg/rust-protobuf/tree/master/protobuf-codegen-pure).

### Use protoc-gen-rust plugin

Readme is [here](https://github.com/stepancheg/rust-protobuf/tree/master/protobuf-codegen).

## Generated code

Have a look at generated files, used internally in rust-protobuf:

* [descriptor.rs](https://github.com/stepancheg/rust-protobuf/blob/master/protobuf/src/descriptor.rs)
  for [descriptor.proto](https://github.com/stepancheg/rust-protobuf/blob/master/proto/google/protobuf/descriptor.proto)
  (that is part of Google protobuf)

## Rustdoc

docs.rs hosts [rustdoc for protobuf](https://docs.rs/protobuf/*/protobuf/).

## Copy-on-write

Rust-protobuf can be used with [bytes crate](https://github.com/carllerche/bytes).

To enable `Bytes` you need to:

1. Enable `with-bytes` feature in rust-protobuf:

```
[dependencies]
protobuf = { version = "~2.0", features = ["with-bytes"] }
```

2. Enable bytes option

with Customize when codegen is invoked programmatically:

```
protoc_rust::run(protoc_rust::Args {
    ...
    customize: Customize {
        carllerche_bytes_for_bytes: Some(true),
        carllerche_bytes_for_string: Some(true),
        ..Default::default()
    },
 });
 ```

or in `.proto` file:

```
import "rustproto.proto";

option (rustproto.carllerche_bytes_for_bytes_all) = true;
option (rustproto.carllerche_bytes_for_string_all) = true;
```

With these options enabled, fields of type `bytes` or `string` are
generated as `Bytes` or `Chars` respectively. When `CodedInputStream` is constructed
from `Bytes` object, fields of these types get subslices of original `Bytes` object,
instead of being allocated on heap.

## Related projects

* [quick-protobuf](https://github.com/tafia/quick-protobuf) — alternative protobuf implementation in Rust
* [prost](https://github.com/danburkert/prost) — another protobuf implementation in Rust
* [serde-protobuf](https://github.com/dflemstr/serde-protobuf)
* [grpc-rust](https://github.com/stepancheg/grpc-rust) — implementation of gRPC based on this library
* [grpc-rs](https://github.com/pingcap/grpc-rs/) — another gRPC implementation for Rust
