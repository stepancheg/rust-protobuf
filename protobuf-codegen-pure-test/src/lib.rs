#![cfg(test)]

extern crate protobuf;

extern crate protobuf_test_common;

#[cfg(feature = "with-bytes")]
extern crate bytes;

extern crate serde;
#[macro_use]
extern crate serde_derive;

mod v2;
mod v3;

mod common;
