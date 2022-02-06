#![cfg(test)]

extern crate protobuf;

extern crate protobuf_test_common;

#[cfg(feature = "with-bytes")]
extern crate bytes;

mod v2;

// `cfg(proto3)` is emitted by `build.rs`

#[cfg(proto3)]
mod v3;

mod common;

#[cfg(proto3)]
mod google;

mod interop;

mod include_generated;
