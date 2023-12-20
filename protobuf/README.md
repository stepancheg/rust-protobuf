<!-- cargo-sync-readme start -->

# Library to read and write protocol buffers data

## Features

This crate has following features

### `with-bytes`
`with-bytes` enables `protobuf` crate support for
[`bytes` crate](https://github.com/tokio-rs/bytes):
when parsing bytes or strings from `bytes::Bytes`,
`protobuf` will be able to reference the input instead of allocating subarrays.

Note, codegen also need to be instructed to generate `Bytes` or `Chars` for
`bytes` or `string` protobuf types instead of default `Vec<u8>` or `String`,
just enabling option on this crate is not enough.

See `Customize` struct in [`protobuf-codegen` crate](https://docs.rs/protobuf-codegen).

### `btreemaps`
Modifies the Rust representation of
[map fields](https://protobuf.dev/programming-guides/proto3/#maps) to
use `BTreeMap` by default instead of `HashMap`. This can be handy for
those who want deterministic serialization. Note that unknown fields
still use hashmaps.

## Accompanying crates

* [`protobuf-json-mapping`](https://docs.rs/protobuf-json-mapping)
  implements JSON parsing and serialization for protobuf messages.
* [`protobuf-codegen`](https://docs.rs/protobuf-codegen)
  can be used to generate rust code from `.proto` crates.
* [`protoc-bin-vendored`](https://docs.rs/protoc-bin-vendored)
  contains `protoc` command packed into the crate.
* [`protobuf-parse`](https://docs.rs/protobuf-parse) contains
  `.proto` file parser. Rarely need to be used directly,
  but can be used for mechanical processing of `.proto` files.

<!-- cargo-sync-readme end -->
