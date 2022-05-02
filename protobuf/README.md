<!-- cargo-sync-readme start -->

# Library to read and write protocol buffers data

## Features

This crate has one feature, which is `with-bytes`.

`with-bytes` enables `protobuf` crate support for
[`bytes` crate](https://github.com/tokio-rs/bytes):
when parsing bytes or strings from `bytes::Bytes`,
`protobuf` will be able to reference the input instead of allocating subarrays.

Note, codegen also need to be instructed to generate `Bytes` or `Chars` for
`bytes` or `string` protobuf types instead of default `Vec<u8>` or `String`,
just enabling option on this crate is not enough.

See `Customize` struct in [`protobuf-codegen` crate](https://docs.rs/protobuf-codegen).

## Accompanying crates

* [`protobuf-json-mapping`](https://docs.rs/protobuf-json-mapping)

<!-- cargo-sync-readme end -->
