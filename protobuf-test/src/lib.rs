#![cfg(test)]

extern crate protobuf;

extern crate protobuf_test_common;

extern crate protobuf_serde;

#[cfg(feature = "with-bytes")]
extern crate bytes;

extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

mod v2;

// `cfg(proto3)` is emitted by `build.rs`

#[cfg(proto3)]
mod v3;

mod common;

#[cfg(proto3)]
mod google;

