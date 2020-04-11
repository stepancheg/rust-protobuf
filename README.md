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
* Supports both Protobuf versions 2 and 3

## List of crates

`rust-protobuf` — repository provides multiple crates:

* `protobuf` — protobuf runtime
* `protobuf-codegen` — protobuf codegen engine and `protoc-gen-rust` plugin for `protoc` command
* `protoc` — programmatically work with `protoc` command
* `protoc-rust` — codegen which can be invoked programmatically using `protoc` binary (e. g. from `build.rs`)
* `protobuf-codegen-pure` — pure rust codegen
* `protoc-bin-vendored` — `protoc` binary packaged as crate, can be used with `protoc` or `protoc-rust` crates

## About versions and branches

- `2.*.*` is the latest stable version. `2.*.*` versions follow semver conventions
- versions below `2` are no longer supported

See [CHANGELOG.md](CHANGELOG.md) for a list of changes and compatility issues between versions.

## How to generate rust code

There are several ways to generate rust code from `.proto` files:

* [Invoke protoc programmatically with protoc-rust crate (`protoc-rust` crate, recommended)](https://github.com/stepancheg/rust-protobuf/tree/master/protoc-rust)
* [Use pure rust protobuf parser and code generator (`protobuf-codegen-pure` crate)](https://github.com/stepancheg/rust-protobuf/tree/master/protobuf-codegen-pure)
* [Use `protoc-gen-rust` plugin for Google's `protoc` command](https://github.com/stepancheg/rust-protobuf/tree/master/protobuf-codegen)

## Generated code

Have a look at generated files, used internally in rust-protobuf:

* [descriptor.rs](https://github.com/stepancheg/rust-protobuf/blob/master/protobuf/src/descriptor.rs)
  for [descriptor.proto](https://github.com/stepancheg/rust-protobuf/blob/master/proto/google/protobuf/descriptor.proto)
  (that is part of Google protobuf)

## Rustdoc

docs.rs hosts [rustdoc for protobuf](https://docs.rs/protobuf/*/protobuf/).

## Getting help

Feel free to [open an issue](https://github.com/stepancheg/rust-protobuf/issues/new)
if you need help with rust-protobuf.

## Copy-on-write

Rust-protobuf can be used with [bytes crate](https://github.com/carllerche/bytes).

To enable `Bytes` you need to:

1. Enable `with-bytes` feature in rust-protobuf:

```toml
[dependencies]
protobuf = { version = "2", features = ["with-bytes"] }
```

2. Enable bytes option

with `Customize` when codegen is invoked programmatically:

With stable rust-protobuf:

```rust
protoc_rust::run(protoc_rust::Args {
    ...
    customize: Customize {
        carllerche_bytes_for_bytes: Some(true),
        carllerche_bytes_for_string: Some(true),
        ..Default::default()
    },
 });
```

With rust-protobuf from master:

```rust
protoc_rust::Args::new()
    ...
    .customize(Customize {
        carllerche_bytes_for_bytes: Some(true),
        carllerche_bytes_for_string: Some(true),
        ..Default::default()
    })
    .run()?;
```

or in `.proto` file:

```proto
import "rustproto.proto";

option (rustproto.carllerche_bytes_for_bytes_all) = true;
option (rustproto.carllerche_bytes_for_string_all) = true;
```

With these options enabled, fields of type `bytes` or `string` are
generated as `Bytes` or `Chars` respectively. When `CodedInputStream` is constructed
from `Bytes` object, fields of these types get subslices of original `Bytes` object,
instead of being allocated on heap.

## serde_derive support

(Only in master, not released yet)

Rust-protobuf can be used with [serde](https://github.com/serde-rs/serde).

To enable `serde` you need to:

1. Enable serde option

with `Customize` when codegen is invoked programmatically:

with stable rust-protobuf:

```rust
protoc_rust::run(protoc_rust::Args {
    ...
    customize: Customize {
        serde_derive: Some(true),
        ..Default::default()
    },
});
```
 
with rust-protobuf from master:

```rust
protoc_rust::Args::new()
    ...
    .customize(Customize {
        serde_derive: Some(true),
        ..Default::default()
    })
    .run()?;
```

or in `.proto` file:

```proto
import "rustproto.proto";

option (rustproto.serde_derive_all) = true;
```

You may now `Serialize` and `Deserialize` messages:

```rust
let my_message = MyMessage::new();
serde_json::to_string(&my_message).unwrap();
```

## Related projects

* [quick-protobuf](https://github.com/tafia/quick-protobuf) — alternative protobuf implementation in Rust
* [prost](https://github.com/danburkert/prost) — another protobuf implementation in Rust
* [serde-protobuf](https://github.com/dflemstr/serde-protobuf)
* [grpc-rust](https://github.com/stepancheg/grpc-rust) — implementation of gRPC based on this library
* [grpc-rs](https://github.com/pingcap/grpc-rs/) — another gRPC implementation for Rust
