<!-- cargo-sync-readme start -->

# Library to read and write protocol buffers data

# Version 3 is alpha

Currently developed branch of rust-protobuf is 3. It has the same spirit as version 2,
but contains numerous improvements like:
* runtime reflection for mutability, not just for access
* protobuf text format and JSON parsing (which rely on reflection)
* dynamic message support: work with protobuf data without generating code from schema

Stable version of rust-protobuf will be supported until version 3 released.

[Tracking issue for version 3](https://github.com/stepancheg/rust-protobuf/issues/518).

# Accompanying crates

* [`protobuf-codegen`](https://docs.rs/protobuf-codegen/%3E=3.0.0-alpha)
  can be used to rust code from `.proto` crates.
* [`protoc`](https://docs.rs/protoc/%3E=3.0.0-alpha) crate can be used to invoke `protoc` programmatically.
* [`protoc-bin-vendored`](https://docs.rs/protoc-bin-vendored/%3E=3.0.0-alpha)
  contains `protoc` command packed into the crate.
* [`protobuf-parse`](https://docs.rs/protobuf-parse/%3E=3.0.0-alpha) contains
  `.proto` file parser. Rarely need to be used directly,
  but can be used for mechanical processing of `.proto` files.

<!-- cargo-sync-readme end -->
