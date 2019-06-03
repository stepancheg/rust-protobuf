#![cfg(test)]

// Because we need compat with Rust 1.26
#![allow(bare_trait_objects)]

extern crate protobuf;

extern crate protobuf_test_common;

#[cfg(feature = "with-bytes")]
extern crate bytes;

#[cfg(feature = "with-serde")]
extern crate serde;
#[cfg(feature = "with-serde")]
#[macro_use]
extern crate serde_derive;
#[cfg(feature = "with-serde")]
extern crate serde_json;

mod v2;
mod v3;

mod common;
