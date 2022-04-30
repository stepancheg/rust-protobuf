#![cfg(test)]

extern crate protobuf;

extern crate protobuf_test_common;

#[cfg(feature = "with-bytes")]
extern crate bytes;

mod v2;
mod v3;

mod common;

mod google;

mod interop;

mod include_generated;
