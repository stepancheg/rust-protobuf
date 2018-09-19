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
mod v3;

mod common;

mod interop;
