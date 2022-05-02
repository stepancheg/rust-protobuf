<!-- cargo-sync-readme start -->

# Library to read and write protocol buffers data

## Version 3 is alpha

Currently developed branch of rust-protobuf is 3. It has the same spirit as version 2,
but contains numerous improvements like:
* runtime reflection for mutability, not just for access
* protobuf text format and JSON parsing (which rely on reflection)
* dynamic message support: work with protobuf data without generating code from schema
* lite runtime codegen option now produces correct code without reflection support

Stable version of rust-protobuf will be supported until version 3 released.

[Tracking issue for version 3](https://github.com/stepancheg/rust-protobuf/issues/518).

## Features

This crate has one feature, which is `with-bytes`.

`with-bytes` enables `protobuf` crate support for
[`bytes` crate](https://github.com/tokio-rs/bytes):
when parsing bytes or strings from `bytes::Bytes`,
`protobuf` will be able to reference the input instead of allocating subarrays.

Note, codegen also need to be instructed to generate `Bytes` or `Chars` for
`bytes` or `string` protobuf types instead of default `Vec<u8>` or `String`,
just enabling option on this crate is not enough.

See `Customize` struct in [`protobuf-codegen` crate](https://docs.rs/protobuf-codegen/%3E=3.0.0-alpha).

## Accompanying crates

* [`protobuf-json-mapping`](https://docs.rs/protobuf-json-mapping/%3E=3.0.0-alpha)

<!-- cargo-sync-readme end -->
